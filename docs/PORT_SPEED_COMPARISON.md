# Recomp (NSMB-Wii-Recompilation) vs decomp port (the-mario-bros-team/NSMBWDecompilation)

Written 2026-11-25, from this repo at `246bc22` (v0.3.0) and its own measurements in `docs/PORT_PLAN.md` sections 10-18. Nothing from it was built or run here yet. Recomp numbers are from NSMB-Wii-Recompilation v0.5.0 plus PR #44, measured on the owner's PC on 2026-11-22.

## What each one is

Both run the game's own PowerPC code through ahead-of-time translation. The 52 MB of hand-written Rust in the decomp's `src/` is not what runs: its own plan says "nothing runs the game" there. The playable port is in `crates/`.

| | Recomp | Decomp port |
|---|---|---|
| CPU | WiiCompiled: C# translator, C++ per guest function, **every function in MAP.txt** | `ppc-recomp`: Rust per guest function, **only functions a recorded run executed** (`profile-all.txt`), interpreter for the rest |
| Generated code | Guest registers in C locals; spilled only where a callee needs them (per-function ABI masks); loads/stores are flat pointer + byte swap; direct calls | **Every instruction calls the interpreter's handler** on `&mut Cpu` with a constant word; checked loads/stores; slice-budget check after every branch; `match pc` entry; about 120 bytes of x86 per guest instruction |
| OS and threads | HLE OS; guest threads are fibers that switch at OS calls | The game's own OS runs; interrupts and thread switches at any instruction, from a cycle budget (needed for save states and online lockstep) |
| Graphics | **The GX library itself is replaced** with native code that feeds Aurora (vertex decode and TEV on the GPU) | The game writes GX commands to an emulated FIFO; a scanner on the game thread, then a decode thread and a transform thread parse and transform them on the CPU. GPU vertex processing arrived 2026-11-24 (Vulkan only) |
| Hot-function natives | About 170: GX, `nw4r::lyt` quads, light textures, strcmp/breff searches, AX mixing and reverb, the gameplay hooks | A handful: light textures, strcmp, sqrt, reverb |
| GX-side caches | Display-list cache, texture hashing driven by write notifications, draw merging, pipeline cache | Indexed geometry, recycled buffers, chunked texture hashing, light textures, merging of back-to-back draws with the same state; no display-list cache yet |

## The numbers we have (not yet from the same PC)

| | Recomp, the owner's PC (Ryzen 5 5600G) | Decomp, its author's PC (unknown CPU, RTX 5070 Ti) |
|---|---|---|
| 1-1, game thread per frame | 2.8-3.0 ms | 5.0 ms (headless), plus GX threads |
| Water course 4-1 / 4-2 | 4.0-4.5 ms | 5.4-6.1 ms |
| 8-1 | - | 6.5 ms (8.2 before every course was profiled) |
| 4 players in 1-1 | - | 8.2 ms average, 12.9 ms worst second |

The decomp's PC is very likely the faster one, so the real gap is at least this big. Its numbers also leave out the GX decode and transform threads, which its own profile shows as the limit in heavy scenes.

## Why lots of enemies hurts it more

Every enemy adds guest code (state machine, collision, model matrices: float and paired-single heavy) and draws (vertices through the FIFO). Each of those is dearer in the decomp:

1. **Per guest instruction.** Interpreter handlers on a struct, checked memory, and a branch budget cost roughly 2x our translated C++ for the same instructions.
2. **Code the profile never reached runs interpreted.** Its profile comes from one scripted run per course (hold right and hop for 1,110 fields). Enemy states, bosses and objects that run didn't trigger fall back to the interpreter, which its own numbers put at about 2x slower again. In 8-1, before the all-course profile, 22% of the game thread was interpreter and 5% dispatcher.
3. **Every vertex goes through the FIFO.** The game writes it into the emulated gather pipe, then the scanner, the decode thread and the CPU transform handle it (60 ns per vertex). Ours never encodes or decodes a FIFO.
4. **No display-list cache.** Every model's display list is copied into the stream and decoded again every frame (back-to-back draws with the same state are merged, as in the recomp).
5. **Stores pay for code tracking.** Each one checks whether it hit a code page (about 20 host instructions per store).

## Getting it to the recomp's speed, biggest win first

Each step can be checked with the decomp's own gates: the 1-1 hash script (98 RAM hashes, 13,194,609,027 instructions) and `--hash-gx`.

1. **Generate every function in MAP.txt, not just profiled ones.** No interpreter for unseen enemy and boss code, and no lag spikes in courses the profile skipped. Costs build time and binary size only. Low risk.
2. **Registers in locals in generated code.** Load the guest registers a function uses into locals once, spill only at calls and exits, as WiiCompiled does with its ABI masks. The biggest single CPU win.
3. **Cheaper memory:** flat loads and stores with no error path everywhere, and code-change tracking through page protection or REL-only ranges, instead of a check on every store.
4. **Fewer budget checks:** at function entry and loop back-edges only, its own plan's original design, with a counter in a local. Keeps determinism.
5. **Natives for the hot library code:** PSMTX/PSVEC math, `nw4r::lyt` quad drawing, the GX library's own functions, and breff/breft searches. Each one needs the hash gate.
6. **GX at the API level or on the GPU:** finish the GPU vertex path (Direct3D too), and add a display-list cache keyed on write generations. Or hook the GX library's functions directly, as the recomp does, so vertices skip the FIFO. That's the largest piece, and it changes how save states see GX.
7. **Translator polish:** leaf inlining and GQR hoisting.

Steps 2-4 trade nothing it needs: state stays in `Cpu` at every call and slice end, so save states and lockstep still work. Step 6's API-level route is the one that interacts with save states and needs a design decision.

## Functionality, briefly

The decomp port already has things the recomp doesn't: save states, online play with drop-in and a camera per player (separate player windows), a Practice tab (game speed, Fast Forward, Restart Course, level timer, death counter, input display), up to 400 fps shown with in-between frames, Vulkan, D3D12 and D3D11 renderers, Android, Mods and texture packs, and an updater. The recomp has Newer SMBW, Bluetooth passthrough, the HD pack downloader, the LAN lobby, and more natives and device fixes (Adreno, Intel D3D11, old Android). Both read the same disc build (SMNE01 rev 1); the decomp uses the recomp's MAP.txt for function starts.

## To make this a real comparison

Run both on one PC in the same course: one with lots of enemies, plus 1-1 and a 4-player course. Use the recomp's `NSMBW_LOG_FPS` game-thread time and the decomp's `--perf` CSV. Its v0.3.0 release ships a Windows zip (14.3 MB); downloading it needs the user's OK.
