# Promotion map: staging → semantic modules

All 15 lanes are complete. 54 functions already live in semantic modules
(`boot/`, `runtime/`, `engine/`); the other **210 are staging** in
`src/subsystems/r*.rs`. Promotion moves them; it does not copy.

## Rules

1. Move the function, do not duplicate it.
2. Update the ledger row's `module` and `rust_file`.
3. Update the destination module's `//!` header and "Start here" links.
4. The destination file must pass `node tools/doc-lint.mjs src`.
5. Keep `# Original` address/evidence; never drop the chain.
6. If the role is still unknown, it stays staged.

## Destination map

| Staging range | Clusters | Destination |
|---|---|---|
| `r80005_80014` | vertex grids, GX quad emission, vector math | `render/gx.rs`, `render/geometry.rs` |
| | path/rail nodes | `game/path.rs` |
| | node arrays, object construction | `game/actor.rs` |
| `r80014_80040` | `pointGet`, `batteryInfo`, `errorInfo`, lyt panes | `ui/lyt.rs`, `ui/hud.rs` |
| `r80040_80080` | destructors, vtables, child chains | `runtime/cpp.rs` |
| | `obj_dokan` pipe actor | `game/actors/dokan.rs` |
| | player pose/state/effects | `game/player.rs` |
| `r80080_800c0` | stage-resource bootstrap, course parser, bgdat | `game/stage.rs`, `resource/course.rs` |
| `r80100_80140` | four-slot SDA state, bit tables | `engine/globals.rs` |
| | WorldMap effect slots | `game/worldmap.rs` |
| `r80140_80180` | WorldMap actors/effects | `game/worldmap.rs` |
| | interaction object | `game/interaction.rs` |
| | Yoshi / tongue / states | `game/player/yoshi.rs` |
| | display, pause, boot scene, save banner | `engine/display.rs`, `game/boot_scene.rs` |
| | lyt nodes/animation, home menu, dialogs | `ui/lyt.rs`, `ui/menu.rs` |
| `r80180_801c0` | NW4R lists, BRLAN transforms | `ui/lyt.rs` |
| | audio DMA MMIO | `audio/ax.rs` |
| | 16-slot id table | `runtime/ids.rs` |
| `r801c0_80200` | DVD/FST open/read/close, callbacks | `runtime/dvd.rs` |
| | WPAD/KPAD init/callbacks | `runtime/input.rs` |
| `r80200_80260` | Broadcom BTE: HCI/BTM/L2CAP/RFCOMM | `runtime/bt.rs` (new) |
| `r80260_802ed` | archive reader, packed values | `resource/archive.rs` |
| | four-slot player table, alarms, completion | `game/player.rs` |
| | RSEQ header/lookup | `audio/seq.rs` |

## Promotion order (port-driven)

1. **`runtime/dvd.rs` + `runtime/input.rs`** — needed for any runnable slice.
2. **`resource/archive.rs` + `game/stage.rs`** — the disc→level path.
3. **`ui/lyt.rs`** — title/HUD path.
4. **`render/gx.rs` + `audio/ax.rs`** — presentation.
5. **`game/player.rs` + actors/worldmap** — gameplay.
6. **`runtime/bt.rs`** — lowest priority; Wiimote transport on PC will be
   replaced by XInput/keyboard anyway.

## New semantic modules this creates

`src/resource/`, `src/render/geometry.rs`, `src/game/path.rs`,
`src/game/actors/`, `src/game/player/`, `src/engine/display.rs`,
`src/ui/menu.rs`, `src/runtime/{cpp,dvd,input,ids,bt}.rs`,
`src/audio/seq.rs`.

Create each module with its `//!` header before moving the first function.

## Definition of done for the promotion pass

- `src/subsystems/` contains only genuinely unclassified functions.
- Every moved function is listed in `docs/FUNCTION_INDEX.md` under its
  semantic module.
- `cargo check` = 0, `doc-lint` = 0.
- `docs/COVERAGE.md` reflects the new module names.
