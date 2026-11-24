# NSMBW → Rust reconstruction

Source-level Rust reconstruction of **New SUPER MARIO BROS. Wii**
(SMNE01, build `r37675`), derived from the shipped PowerPC binary.

This is not a playable game yet, and it is not Nintendo source. Every function
is reconstructed from the binary, with its Ghidra evidence cited.

## New here?

Read in this order:

1. **[docs/READING_GUIDE.md](docs/READING_GUIDE.md)** — how the repo is layered
2. **[docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)** — machine, memory map, boot flow
3. **[docs/BOOT_CHAIN.md](docs/BOOT_CHAIN.md)** — entry point to game startup
4. **[docs/COVERAGE_FINAL.md](docs/COVERAGE_FINAL.md)** — authoritative
   translation and untabled-corpus accounting
5. **[docs/COVERAGE.md](docs/COVERAGE.md)** — what is translated vs total
6. **[docs/PROGRESS.md](docs/PROGRESS.md)** — measured percentage per module
7. **[docs/FUNCTION_INDEX.md](docs/FUNCTION_INDEX.md)** — every translated function
8. **[docs/GLOSSARY.md](docs/GLOSSARY.md)** — DOL, REL, U8, BRRES, GX, AX, r13
9. **[docs/ORGANIZATION.md](docs/ORGANIZATION.md)** — why range modules exist and how they become semantic modules
10. **[docs/PROMOTION.md](docs/PROMOTION.md)** — exact staging → semantic destination map
11. **[docs/PIPELINE.md](docs/PIPELINE.md)** — evidence → translation → gates workflow
12. **[docs/RUST_STYLE.md](docs/RUST_STYLE.md)** — the documentation standard

## What is where

| Path | Contents |
|---|---|
| `src/boot/` | entry, CRT, `memcpy`/`memset`, cache flush, init tables |
| `src/runtime/` | OS init, allocator, scheduler, DVD, input, C++ runtime |
| `src/engine/` | game startup and main loop |
| `src/subsystems/` | unclassified address-range extraction (parallel lanes) |
| `src/platform.rs` | big-endian volatile access + indirect calls |
| `work/ghidra/main-gb/export/` | evidence: 10,276 decompiled C functions |
| `work/arcs/index.json` | 820 U8 archives / 2,933 members |
| `work/rels/` | decompressed RELs + structural metadata |
| `docs/` | architecture, formats, coverage, port plan |
| `ledger/` | per-function status/confidence/evidence |

## Status

| Stage | State |
|---|---|
| 0 — inventory, U8 index, LZ11 RELs, DOL→ELF | done |
| 1 — Ghidra 12.1.4 + Gekko_Broadway (20,252 funcs exported) | done |
| 2 — boot/OS/engine core + semantic promotion | done |
| 3 - REL decompilation + Waves 2-239 | inventory complete: 20,250 / 20,252 (99.99%) |
| 4 — resources/engine/gameplay expansion | in progress |
| 5–7 — verification, matching, port | queued |
| Windows port | queued, see `docs/PORT_WINDOWS.md` |

Coverage: `docs/COVERAGE_FINAL.md` is the authoritative summary;
`docs/COVERAGE.md` and `docs/PROGRESS.md` are generated views. Gates:
`cargo check` = 0, `doc-lint` = 0. Next targets: `docs/NEXT_STEPS.md`.

The 20,252 count is the Ghidra `functions.tsv` inventory, and it is now fully
accounted for: 20,250 translated plus two documented `game::bases` deferrals
(`0x806E2BF0`, `0x80697090`). It is **not** the whole binary. Wave 226 found
2,457 prologue-verified function starts outside `functions.tsv`; waves 228-238
translated 259 of them (251 exact `fn_<address>` names, 8 suffixed or semantic
names), so **2,198 remain**, and 371 corpus entries are aliases of other slots
(one body serves every slot that names it). Untabled bodies are tracked in
`work/wave228/` through `work/wave238/` and in `docs/COVERAGE_FINAL.md`, never
in the ledger.

## Reading a function

```rust
/// Copies `len` bytes, then flushes data cache + invalidates icache.
/// # Original
/// - Ghidra: `FUN_80004364` at `0x80004364` (668 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80004364_FUN_80004364.c`
/// # Confidence
/// high — matches the canonical CodeWarrior `memcpy` shape.
```

Addresses are virtual (`0x80004000` base), not file offsets. Confidence means
evidence strength, not code quality.

## Build

```powershell
.\scripts\check.ps1
```

If local script execution is disabled on this machine, run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\check.ps1
```

Runs `cargo check`, the doc linter, and regenerates the ledger, function index,
coverage and progress. `cargo check` needs no linker; a Windows binary later
requires Visual Studio Build Tools. The port plan is separate
(`docs/PORT_WINDOWS.md`).

## The port's HD textures and mods

The port (`crates/frontend`) reads these from folders beside its `port.toml`,
and the Settings window's **Mods** tab turns them on and off:

- `texture_packs\<name>\`: a Dolphin texture pack (`tex1_*.png` or `.dds`,
  anywhere under that folder, so Dolphin's `Load\Textures\SMN` folder works
  as it is). It only changes what is drawn, so each player in a net game
  chooses for themselves. The pack made for this game is the **New Super
  Mario Bros. Wii HD Texture Pack** v2.81 (PNG edition) by **Bighead, razius,
  jaco8, CDAMJC and Amon1995**. Get it from its own Dolphin forum thread and
  put its `SMN` folder in `texture_packs\NSMBW_HD_v2.81\`. No pack is
  included in this repository.
- `texture_dumps = true` in `settings.toml`: every texture no pack replaces
  is written to `texture_dumps\` under its pack name, to make a pack from.
- `mods\<name>\`: files laid out like the disc's `files\` folder are used
  instead of the disc's own from the next start. Mods load in name order,
  and a later one's file wins. They are off in net games.

## The port's controllers

Controllers are read through SDL 3, which `crates/input` builds from source
(building the port needs CMake and a C compiler). If CMake's Visual Studio
generator can't build anything on your PC ("No CMAKE_C_COMPILER could be
found"), build with `CMAKE_GENERATOR=Ninja` and the `ninja` that comes with
Visual Studio (`Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja`) on `PATH`.

- **Motion sensors** (Settings > Gameplay > Motion Sensors, on by default): on
  a controller that has them (DualShock 4, DualSense, Switch Pro, Joy-Cons and
  the others SDL knows), turning it like a steering wheel tilts the Wii Remote
  (full tilt at 30°, nothing under 4°) and a sharp jolt shakes it. Gravity is
  followed as you play, so there is nothing to calibrate. A stick or trigger
  bound to Tilt comes first while it is pushed. `[controller] motion_sensors`
  in `settings.toml`.
- **Bluetooth passthrough** (experimental, off by default, Windows): the game's
  own Bluetooth stack drives real Wii Remotes through a spare USB Bluetooth
  adapter, as on a Wii. Not yet tried with a real adapter or remote.
  1. Use a spare USB Bluetooth adapter, not the one your keyboard, mouse or
     headphones use. As with Dolphin's passthrough, a Broadcom BCM20702 or
     CSR8510 A10 chip works best.
  2. Give it the WinUSB driver with [Zadig](https://zadig.akeo.ie): *Options >
     List All Devices*, pick the adapter (check its USB ID so it is not your
     PC's own Bluetooth), choose **WinUSB** and **Replace Driver**. To give it
     back to Windows: Device Manager > the adapter > *Uninstall device* with
     its driver, then unplug and replug it.
  3. Turn on **Settings > Gameplay > Bluetooth Passthrough** and restart the
     game. Adapters on any other driver are left alone; the console says why
     none could be used, and the keyboard and controllers then play as usual.
  4. Pair each remote once: press **F8**, then the red SYNC button under the
     remote's battery cover. After that, any button connects it. Holding F8
     for 10 seconds forgets every pairing.

  While it is on, the keyboard and controllers don't drive the game. It is
  off in net games and scripted runs. Pairings are kept in
  `saves\bt-passthrough\`, apart from the emulated remotes'. The console's
  `[bluetooth]` lines say what happened.

To check without the game: `cargo run -p input --example controllers` lists the
controllers SDL finds with their accelerometer and the tilt it makes, and says
which adapter passthrough would use. `NSMBW_INPUT_LOG=1` prints each change of
a remote's buttons, shake or tilt as it goes to the game.

## Legal

Use your own dump. No game assets or extracted content are redistributed here.

## Credits

- Player 1's character pick (the Settings window's Gameplay tab) is mkwcat's
  Gecko code "Player 1 Can Change Character"
  ([mkwcat/gecko-codes](https://github.com/mkwcat/gecko-codes), `nsmbw/`),
  Copyright (c) 2024 mkwcat, used under the MIT License; the notice is in
  `crates/hle-os/src/p1_character.rs`.
- The Android app's GPU Driver option loads an imported driver with
  [libadrenotools](https://github.com/bylaws/libadrenotools) and its
  liblinkernsbypass, Copyright (c) 2021 Billy Laws, used under the BSD
  2-Clause License; the source and both licence texts are in
  `android/third_party/libadrenotools` (see its `NOTICE.md` there).
- The other versions of the game (Europe, ...) find what the port names by
  its USA address through NSMBW-Updated's address map
  ([NSMBW-Community/NSMBW-Updated](https://github.com/NSMBW-Community/NSMBW-Updated),
  `code/address-map.txt`), by Ninji, CLF78 and RoadrunnerWMC, Copyright (c)
  2021 RoadrunnerWMC, used under the MIT License; the file and its notice are
  in `crates/game-version`. Each version's Player 1 character code is
  mkwcat's own for that version.
- Controllers: [SDL 3](https://libsdl.org) (zlib License), through the
  `sdl3-sys` bindings (zlib License).
- Bluetooth passthrough's USB access: [nusb](https://github.com/kevinmehall/nusb)
  (MIT or Apache-2.0). Its design follows Dolphin's real-adapter mode and
  WitherBlitz's NSMB-Wii-Recompilation.

## License

See [LICENSE](LICENSE). In short: the NSMBW Decomp project may not use any of this repository under any circumstances. Anyone else may fork it and do what they want with the fork; mods may use all of the code; a new repository that uses this code must credit NSMBWDecompilation explicitly. No game files are included: bring your own copy of New Super Mario Bros. Wii (USA, SMNE01, or Europe, SMNP01; revision 1).
