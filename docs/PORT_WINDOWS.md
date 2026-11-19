# Windows port plan (post-decompilation)

## What this is not

Decompilation reconstructs the original PowerPC/Wii logic. It is **not** a
Windows port: the code targets big-endian Broadway, fixed MMIO addresses,
GX/ AX hardware, and a 24 MB MEM1 arena. A port replaces all of that while
keeping the reconstructed game logic.

## Path

```
Rust reconstruction (this repo)
        │
        ├── nsmbw-core      game logic, no hardware access
        ├── nsmbw-wii       original ABI: MMIO, GX/AX command streams, OS calls
        ├── nsmbw-assets    U8/BRRES/BCSV/BLYT/BRSTM/BRSAR decoders
        ├── nsmbw-render    GX → wgpu (or D3D12) translation, TEV shaders
        └── nsmbw-win       window, input, audio, filesystem, timing, save
```

The key refactor is **hardware abstraction before translation**: game code
calls `gx::*`, `ax::*`, `os::*`, `dvd::*`, never raw MMIO. `nsmbw-wii` provides
the original implementation; `nsmbw-win` provides the PC implementation.

## Milestones

| # | Milestone | Exit criteria |
|---|---|---|
| P0 | Boot vertical slice | `entry` → CRT/OS init → main loop runs on Windows, blank window, no assets |
| P1 | Asset pipeline | All 820 U8 arcs + BRRES/TPL/BLYT/BCSV decode; title screen renders |
| P2 | 1-1 slice | Load Stage 01-01, render World 1-1, player physics + input, no audio |
| P3 | Audio/save/menus | BRSTM/BRSAR playback, save data, pause/options |
| P4 | Full game | All worlds, bosses, modes, multiplayer |

## Hard parts (ranked)

1. **GX → GPU**: TEV pipeline, indirect texturing, EFB copies, XFB, material
   state. This is the largest single subsystem.
2. **Audio**: DSP ADPCM (BRSTM/BRSAR), JAS/JAI sequencing, AX voice mixing.
3. **Paired-single FPU**: Broadway `psq_l/ps_st` semantics; translate to scalar
   or 2-lane f32 in the port.
4. **Timing/scheduling**: VI retrace, OS alarms, DVD latency; deterministic loop.
5. **Input**: Wiimote/nunchuk/classic → keyboard/XInput mapping.
6. **Assets**: every format must be decoded at runtime from the user's copy.

## Rules

- Never bundle game assets or extracted content with the port.
- Port reads the user's own dumped disc tree (`files/`, `sys/`).
- Keep the Wii implementation compiling as `nsmbw-wii`; it is the reference.
- A Windows feature is "done" only when the same reconstructed function runs
  through the PC backend without changing game logic.

## Immediate prerequisite

Do not start P0 until: (a) boot/CRT + engine startup lanes are translated,
(b) hardware abstraction traits exist, (c) asset decoders for the first scene
are available. Premature porting would fork the decompilation and lose the
evidence trail.
