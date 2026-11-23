# Multiplayer performance report (2026-11-22)

Build: `ui`/`main` at 83a694d (p9 merged). PC: RTX 5070 Ti, measured paced
at 60 fps, muted, windowed, the default settings (60 fps, scale 2, V-Sync off).

## Summary

Local multiplayer drops to **1.5–2 frames shown a second (6–9 game fps)** as
soon as two or more players are in a course. One function causes it:
`gx::World::apply`, the code that moves the picture into each player's
window, takes **87% of all the CPU time the game uses**. It reads vertices back
from GPU memory that the CPU can only write quickly. Everything else
(emulation, GX, the driver) is small next to it. Fixing it is a small,
local change.

Single player and two-player online on one PC both run at full speed in
these tests. The online code does have real design problems, listed below,
that would cause lag over the internet and break games with 3 or more
players.

## Measurements (paced, 7400 fields of 1-1)

| Run | Game speed | Frames shown | Main window render |
|---|---|---|---|
| Solo | median 100%, 10th percentile 98% | ~60 fps | ~1 ms |
| 4 players, on the World map | 100% | 60 fps | ~1 ms |
| 4 players, in the course (camera box, 3 player windows) | **6–19%** | **1.5–2.8 fps** | **35–80 ms** |
| Online host + joiner on 127.0.0.1 | 100% on both (38% for about 1 s while joining) | 60 fps | ~1 ms |

Emulation stayed at 5–8 ms a field throughout. The window thread is what
falls behind.

## 1. The cause of the 1 fps: reading GPU memory on the CPU

**Evidence.**
- An xperf CPU sample of 20 s in the 4-player course: `World::apply` uses
  20.4 s of 23.6 s of CPU, a full core.
- The samples sit on two instructions of its inner vertex loop, the ones
  right after the load of each vertex's position (`vmovsd (%rbx,%r9)`,
  144-byte stride).
- GX's own transform of the same vertices costs about 1.5 ms a field, so the
  number of vertices is normal. Each read is what's slow.

**Why.** `Renderer::render_with` hands `fill` the frame's vertex buffer
*mapped from GPU memory* (`MemoryLocation::CpuToGpu`: host-visible,
write-combined). Its doc comment says "`fill` should only write"
(`crates/vk-render/src/lib.rs:1238`). The player-window code copies the
vertices in and then runs `World::apply` on that same buffer. `apply` reads
every world vertex back, and each read from that memory is uncached.
- `crates/frontend/src/play.rs:703-704`: each player window, every field.
- `crates/frontend/src/play.rs:741-742`: the main window, every field while
  the box is on.

That is about 30–40 ms per window per field. With 3 player windows plus the
main window, one field costs about 125 ms on the single window thread, so
the game crawls at 8 fps. Only one of each batch of fields is shown, which
leaves ~2 fps on screen.

**Fix.** Do the move while copying: read from `out.vertices` (normal
memory), write the moved vertex into `dst` once, and never read `dst`. For
example, `World::apply_into(src, dst, view, backdrop)`. Expected cost: the
same as today's plain copy, about 1 ms per window. Check: the same paced
4-player run at 100% and 60 fps shown, and screenshots byte-identical to
today's.

## 2. Other local multiplayer problems (smaller, but they add up)

1. **All windows render on one thread, one after another.** Every player
   window renders and presents every field on the window thread, before the
   main window. Even after fix 1, 4 windows cost about 4 copies and 4
   submissions a field. When the thread falls behind, player windows still
   render every queued field (only the main window skips presenting), so it
   catches up slowly.
2. **Each player window is a whole new Vulkan instance and device.** That
   means 4 devices on one GPU, every texture uploaded 4 times, and every new
   window first re-uploads every texture in use. Sharing one device, with
   one swapchain per window, would be lighter.
3. **The front end keeps a CPU copy of every texture** (`self.textures`, a
   `clone()` of each new texture every field) only so a later window can
   upload them.
4. **In-between frames are off while the box is on.** At an fps setting
   above 60, the main window drops to 60 updates a second in multiplayer,
   still one field behind real time. `prepare_field` also clones the whole
   vertex buffer for each prepared pair.
5. **The box is built around the game's stale camera rectangle** (PORT_PLAN
   12.4, open issue 1). It keeps the course start inside the box, so the
   game draws a much wider area than needed in multiplayer.
6. **Backdrop work per layer piece.** `Backdrop::y_change` clones and sorts
   a small vector for every 16-unit height step of every background piece,
   every window, every field. It's minor, but it allocates in a hot path.

## 3. Online problems (not visible on one PC, real over the internet)

1. **Fixed 2-field input delay with plain lockstep** (`net.rs`, `DELAY = 2`).
   A machine can only run a field once every other machine's input for it
   has arrived. With a round trip above about 33 ms × 2 = 66 ms (common on
   the internet), every field waits, and the whole game slows to the round
   trip. On a LAN, 2 fields is also one more than needed. The delay should
   follow the measured round trip.
2. **The host relays inputs only at the start of its own field.** One
   joiner's input reaches another joiner only when the host's game thread
   next calls `pump()`. That adds up to 1 field plus the host's emulation
   time to every relayed input, which matters with 3 or more players.
3. **No time sync.** Lockstep settles with one machine receiving inputs
   exactly on time and the other with all the slack, so small network
   jitter causes repeated hitches on the first. The catch-up logic in
   `emu.rs` pulls it back to that edge each time.
4. **Waiting polls.** The game thread polls each peer's channel with
   500 µs sleeps instead of blocking on one channel, so it wakes late and
   burns CPU. Each message is also written as two `write_all` calls
   (header, body): two packets.
5. **Bug: 3 or more players online desync.** When a second joiner comes in,
   only the host and the new joiner learn its `active_from` (from the
   welcome). Earlier joiners are never told, so they feed that remote an
   empty input and never bring its player into the course. Their game
   diverges from the host's, and the hash check stops the session. M4 was
   only ever checked with 2 machines.
6. **Joining stalls the host for about 1 s.** The host saves and deflates
   22 MB on its game thread while the game waits. This is acceptable, but it
   is the dip to 38% in the measurements.

## 4. Single player

On this PC single player runs at full speed: median 100%, 1 ms render,
slow fields only at boot and scene loads (20–50 ms). The 1 fps problem only
starts with 2 or more players in a course. A solo player never gets the
box or player windows. If single player feels slow on another PC or other
settings (fps above 60, a higher scale, V-Sync, a laptop GPU), a
`--log-fps` line from that setup would show where.

## Suggested order

1. Fix 1 (move vertices while copying). This alone should restore
   60 fps in local multiplayer.
2. Online items 5 (3+ player desync) and 2 (relay in the reader thread).
3. Online items 1 and 3 (delay from measured round trip, time sync), plus
   item 4. Test with an added-latency option to stand in for the internet.
4. Local items 1–3 (one Vulkan device, windows not blocking each other),
   then the box rectangle (item 5).

Each step is checked with the paced runs above, the lockstep hash runs (1-1,
4p) on both engines, and screenshots against the current build.
