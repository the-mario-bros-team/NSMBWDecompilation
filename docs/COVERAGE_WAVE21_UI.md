# Coverage - wave 21 HUD/menu helpers (`0x80018330..0x80175330`)

## How to read this file

- **Scope** - the eighth UI translation pass. It adds 25 functions to the two
  existing semantic modules `src/ui/hud.rs` and `src/ui/menu.rs`; no staging
  module or new `.rs` file is created.
- **Addresses** are virtual `sys/main.dol` addresses (image base
  `0x80004000`), not file offsets.
- **Evidence** is the Ghidra C export under
  `work/ghidra/main-gb/export/`; each row names its file.
- **Status** follows `docs/RUST_STYLE.md`:
  `translated` = full control flow and every call accounted for;
  `partial` = flow and major effects present, one operand not literal;
  `stub` = address/role only.
- **Confidence** measures the evidence, not code quality. `medium` marks a
  function whose control flow and stores are exact but whose operand split or
  global-role naming was inferred from a twin body.
- Start with the **summary**, then the **function tables**. The **cluster
  notes** explain how the new bodies connect to the earlier HUD warning
  scenes, the home-button widget table and the animation-record pools.

## Summary

| metric | value |
|---|---:|
| functions added to `src/ui/hud.rs` | 7 |
| functions added to `src/ui/menu.rs` | 18 |
| functions added in total | 25 |
| bytes translated | 8,212 |
| `translated` / `partial` / `stub` | 25 / 0 / 0 |
| high confidence | 23 |
| medium confidence | 2 |
| new staging modules | 0 |
| files written | `src/ui/hud.rs`, `src/ui/menu.rs`, this document, `ledger/agents/wave21-ui.csv`, `ledger/functions.csv` |

Address span: `0x80018330..0x80175330`. The HUD half completes the two warning
state tables that wave 19 listed as next targets (the 2,168-byte
`dWarningErrorInfo_c` builder and the 824-byte `dWarningNunchuk_c` builder) and
adds the nunchuk release/teardown/animation bodies. The menu half adds the
four-player widget table, the shared 0x2E2C-byte animation-record pools, the
pause-menu boot/shutdown path and the menu-object constructor.

## Function table

### `ui::hud` - error/nunchuk state tables and nunchuk teardown

| address | Rust function | role | status | conf. | evidence |
|---|---|---|---|---|---|
| `0x80018330` | `fn_80018330` | build the eleven `dWarningErrorInfo_c` state descriptors | translated | high | `80018330_FUN_80018330.c` |
| `0x80019560` | `fn_80019560` | build the four `dWarningNunchuk_c` state descriptors | translated | high | `80019560_FUN_80019560.c` |
| `0x800198A0` | `fn_800198a0` | release one nunchuk state record | translated | high | `800198a0_FUN_800198a0.c` |
| `0x80019AC0` | `fn_80019ac0` | release a record with its `+0x18` sub-object | translated | high | `80019ac0_FUN_80019ac0.c` |
| `0x80019B90` | `fn_80019b90` | release `+0x1B4`, destroy the layout, free | translated | high | `80019b90_FUN_80019b90.c` |
| `0x80019E30` | `fn_80019e30` | start the nunchuk animation and notify | translated | high | `80019e30_FUN_80019e30.c` |
| `0x80019EF0` | `fn_80019ef0` | notify when the nunchuk animation ends | translated | high | `80019ef0_FUN_80019ef0.c` |

### `ui::menu` - player widgets, animation records and menu boot

| address | Rust function | role | status | conf. | evidence |
|---|---|---|---|---|---|
| `0x80172EA0` | `fn_80172ea0` | per-player input hook / teardown | translated | high | `80172ea0_FUN_80172ea0.c` |
| `0x80172FB0` | `fn_80172fb0` | per-player event hook and retrigger | translated | high | `80172fb0_FUN_80172fb0.c` |
| `0x801730C0` | `fn_801730c0` | per-player hook timer callback | translated | high | `801730c0_FUN_801730c0.c` |
| `0x80173130` | `fn_80173130` | construct and register one player widget | translated | high | `80173130_FUN_80173130.c` |
| `0x80173290` | `fn_80173290` | uninstall one player widget's hooks | translated | high | `80173290_FUN_80173290.c` |
| `0x80173310` | `fn_80173310` | copy an event colour record into a widget | translated | high | `80173310_FUN_80173310.c` |
| `0x80173510` | `fn_80173510` | per-frame widget update and hold timer | translated | high | `80173510_FUN_80173510.c` |
| `0x80173FB0` | `fn_80173fb0` | state callback: enter the four-argument handler | translated | high | `80173fb0_FUN_80173fb0.c` |
| `0x801740C0` | `fn_801740c0` | state callback: four-argument handler | translated | high | `801740c0_FUN_801740c0.c` |
| `0x801741B0` | `fn_801741b0` | reset one 0x68-byte record and install a callback | translated | high | `801741b0_FUN_801741b0.c` |
| `0x80174240` | `fn_80174240` | lock and write one animation-record slot | translated | medium | `80174240_FUN_80174240.c` |
| `0x80174350` | `fn_80174350` | take an animation record from the free pool | translated | high | `80174350_FUN_80174350.c` |
| `0x80174560` | `fn_80174560` | pump and retire finished animation records | translated | high | `80174560_FUN_80174560.c` |
| `0x801747A0` | `fn_801747a0` | play the home-button MIDI sound and link its record | translated | high | `801747a0_FUN_801747a0.c` |
| `0x801748E0` | `fn_801748e0` | initialise the pause/home-button menu | translated | high | `801748e0_FUN_801748e0.c` |
| `0x80174AF0` | `fn_80174af0` | release every animation record on shutdown | translated | high | `80174af0_FUN_80174af0.c` |
| `0x80174E10` | `fn_80174e10` | choose the menu viewport and boot the menu | translated | medium | `80174e10_FUN_80174e10.c` |
| `0x80175330` | `fn_80175330` | construct the home-button menu object | translated | high | `80175330_FUN_80175330.c` |

All evidence paths are relative to `work/ghidra/main-gb/export/`.

## Cluster notes

### Error-window state table (`0x80018330`)

This is the priority-1 next target of `docs/COVERAGE_WAVE19_UI.md`. It first
initialises the one-time guard word at `SDA_BASE - 0x5A58` (`u16 0x1E`, then
the byte at `+2`). It then builds eleven descriptors at
`0x80352038`, `0x80352078`, `0x803520B8`, `0x803520F8`, `0x80352138`,
`0x80352178`, `0x803521B8`, `0x803521F8`, `0x80352238`, `0x80352278` and
`0x803522B8`. Every block calls `0x8015F7C0` with its
`dWarningErrorInfo_c::StateID_*` name, installs vtable `0x803005A4`, copies
nine words from the adjacent `.data` block (`0x80300218`, `0x8030023C`, ...,
`0x80300380`) and registers with `0x802DC780` and the shared release callback
`0x80018BB0`.

The copy order is preserved and is not the source order: data `+0x18/+0x1C/
+0x20` go to descriptor `+0x0C/+0x10/+0x14`, then data `+0x0C..+0x14`,
`+0x00..+0x04` and `+0x08` go to `+0x18..+0x2C`. Because all eleven blocks are
identical, the file uses one host-side helper `build_error_state`; the helper
is not a ledger row and has no address of its own.

### Nunchuk state table (`0x80019560`)

The smaller twin: four `dWarningNunchuk_c::StateID_*` descriptors at
`0x803522F8`, `0x80352338`, `0x80352378` and `0x803523B8`, vtable `0x803009B4`,
release callback `0x800198A0`, and the same nine-word copy order through
`build_nunchuk_state`.

### Nunchuk teardown and animation (`0x800198A0..0x80019EF0`)

`0x800198a0` is the state release callback: it destroys the base object with
`0x8015F800(this, 0)` and optionally frees it. `0x80019ac0` and `0x80019b90`
are the record destructors; the second also releases `+0x1B4` and destroys the
layout at `+4` through `0x800C89A0(this + 4, -1)`. Both preserve the original's
redundant null comparisons (`this + 0x18 != 0`, `this != -0x19C`).

`0x80019e30` fetches the current frame from `0x800CDC60`, starts animation
`0x68` through `0x800C9A60`, and notifies the member at `+0x19C` with
`0x80352438`. `0x80019ef0` queries `0x800C9610(this + 4, -1)` and sends the
`0x80352478` notification only when the animation has finished.

### Per-player widget table (`0x80172EA0..0x801735B0`)

The table `0x80377F88[4]` holds one widget per local player. The records are
part of the menu manager:

| offset | width | meaning |
|---|---|---|
| `+0x00` | word | player index |
| `+0x04` | word | unscaled animation speed |
| `+0x1C` | byte | cleared by the constructor |
| `+0x20` | word | owning argument / four-record base |
| `+0x24` | word | chained input callback |
| `+0x28` | word | saved callback |
| `+0x30` | 64-bit | press timestamp |
| `+0x40` | byte | connected |
| `+0x41` | byte | timer armed |
| `+0x42` | byte | hold active |
| `+0x43` | byte | hold started |
| `+0x44` | byte | enabled |

`0x80173130` constructs one widget and initialises the two timer objects at
`0x80377E08 + player*0x30` and `0x80377EC8 + player*0x30`. `0x80172ea0` and
`0x80172fb0` are the two channel callbacks; `0x801730c0` is the timer callback
that clears the armed byte after a `3` notification. `0x80173290` removes the
two bindings. `0x80173310` folds an event record's three colour words and
twelve flag bits into the widget record. `0x80173510` calls the record setter
only while the timer is idle, and `0x801735b0` combines the two record
predicates.

### Animation-record pools (`0x80173FB0..0x80174AF0`)

The manager owns two pools of 0x2E2C-byte records: four at `+0` and three at
`+0xB8B0`. Each record has `+0x2E1C` active, `+0x2E20` forward link, `+0x2E24`
back link and `+0x2E28` id. Menus `4`, `0x17` and `0x19` use the alternate
pool and the alternate list at `+0x1433C`; every other menu uses the normal
list at `+0x14334`.

`0x80174350` scans the matching pool for a free record and, when none is free,
unlinks the oldest record of the list, stops it and reuses it. `0x80174560` is
the pump registered by `0x801748e0`: while `0x801B24C0` reports state `1` it
advances three menu hooks and retires every active record that has stopped and
whose `+0x418` is clear, until the pump reports `8`. `0x801747a0` opens a
`midi_00_home_button_mid` sound and appends the acquired record to the tail.
`0x80174af0` is the shutdown walk over both pools.

`0x80173fb0`/`0x801740c0` are the two 0x68-byte record callbacks: they mark
`+0x5C`/`+0x62` and re-arm the record's embedded timer with `0x801AB540` for
50 ms. `0x801741b0` resets one record and installs `0x80173fb0`.
`0x80174240` writes the sound id, duration and mode into a record under the
`0x802DCD70` lock; its first two arguments are not used by the body (Ghidra
prints them, the original only reads the third and fourth).

### Pause-menu boot and menu object (`0x801748E0..0x80175330`)

`0x801748e0` runs only for a resource block larger than `0x186FF`. It resets
the sound-system idle flag, clears all seven records, opens
`wt_HomeButtonSe_wt` and `wt_HomeButtonSe_pcm`, binds the timer group at
`+0x14680`, registers `0x80174560` through `0x801B5130` and publishes the
manager at `0x80377FA0`.

`0x80174e10` chooses one of three viewport modes from the boot record's
`+0x10` and `+0x3C`, fills `0x80377FA8` in mode `1`, updates the two viewport
globals and then boots the menu. `0x80175330` constructs the 0x7A0-byte menu:
it clears the handle/state fields, constructs the draw-info object at `+0x258`
through the already-translated `0x8017F390`, initialises eight timer objects
(`+0x468..+0x4F8`, `+0x528..+0x5B8`, stride `0x30`) plus `+0x5E8`, and
initialises the global widget at `0x80377FC0`.

## Unknowns and limits

1. **Indirect jump stub.** `0x802DCBFC`, used by the two untranslated
   forwarders `0x800182D0`/`0x80018300`, still has no recovered jump table.
   Both 48-byte entry points remain next targets.
2. **Lock result split.** `0x802DCD70` returns the manager and record index in
   `r3:r4`. The Rust body splits the 64-bit result as high = manager and
   low = index, matching Ghidra's `(int)(u >> 32)` / `(int)u`; the like-shaped
   actor body uses the same convention. This is the only reason
   `0x80174240` is `medium`.
3. **Viewport globals.** `0x8032A304` and `0x80378008` are observed pointer
   globals whose exact owner is not named by the export; `0x80174e10` is
   `medium` for that naming, not for its control flow.
4. **Opaque stack records.** The sound handles passed to `0x8019F700` /
   `0x8019F9B0` are untouched stack buffers (`16`/`12`/`40` bytes). Their
   internal layout is callee-owned and not interpreted.
5. **Field roles.** Widget bytes `+0x40..+0x44`, record links
   `+0x2E20/+0x2E24` and record id `+0x2E28` are inferred from the call sites;
   the offsets and all stores are observed.
6. **Twin entry points.** `0x800182D0` and `0x80018300` are byte-for-byte
   twins; both are left untranslated and would add 96 bytes with no new
   behaviour until `0x802DCBFC` is decoded.
7. **Callee bodies.** `0x801E1060`, `0x801E10F0`, `0x801E0F40`, `0x801E5450`,
   `0x801B24C0`, `0x801B5130`, `0x801B5860`, `0x801A1FD0`, `0x8019F700`,
   `0x8019FE50/80/90`, `0x8019F9B0`, `0x8019F1E0/1F0`, `0x801A0050`,
   `0x801944A0/70`, `0x80192A70`, `0x80191D20`, `0x80194760`, `0x801946C0`,
   `0x80194630`, `0x801948B0`, `0x801AB2E0/40/70/540`, `0x801B2390`,
   `0x801B2E20`, `0x801D4380`, `0x801D5560`, `0x801C9A60` and the
   `0x802DCD70/BC/C0` lock helpers are called by address; their bodies are
   outside this pass. `0x8017F390` and the record predicates
   `0x801742E0`/`0x80174300` are already translated and are called directly.

## Verification

```text
cargo check          # exits 0 (no diagnostic references hud.rs or menu.rs)
node tools/doc-lint.mjs src
# {"ok": true, "files": 68, "violations": 0, "byRule": {}}
```

Only `src/ui/hud.rs`, `src/ui/menu.rs`, this document,
`ledger/agents/wave21-ui.csv` and the 25 matching rows appended to
`ledger/functions.csv` were written. `src/lib.rs`, every `mod.rs`,
`Cargo.toml`, `src/platform.rs`, the generated coverage files and the other
lanes were left untouched.

## Next targets

| priority | target | size | why | first evidence |
|---|---|---:|---|---|
| 1 | `0x80175510` | 2,144 B | creates the full menu viewport and its panes; the constructor added here only reaches it | `80175510_FUN_80175510.c` |
| 2 | `0x8017DCC0` | 728 B | dialog animation task; consumes the wave-12 `0x8017D520` evaluator | `8017dcc0_FUN_8017dcc0.c` |
| 3 | `0x80175F70` | 1,184 B | home-button battery/controller pane builder next to the widget table | `80175f70_FUN_80175f70.c` |
| 4 | `0x801769B0` | 7,420 B | the large dialog layout body opened by `0x80174F60` | `801769b0_FUN_801769b0.c` |
| 5 | `0x80172BE0` | 256 B | remaining per-player widget helper between `0x80172A...` and the widget table | `80172be0_FUN_80172be0.c` |
| 6 | `0x8017EA10` | 1,376 B | dialog close/confirm body after the vertex-buffer helpers | `8017ea10_FUN_8017ea10.c` |
| 7 | `0x80171680` | 452 B | home-menu GX setup; deferred because the export contains unreachable-block and `unaff_GQR0` warnings | `80171680_FUN_80171680.c` |
| 8 | `0x800182D0`, `0x80018300` | 48 B each | twin forwarders to `0x802DCBFC`; add only if the unresolved stub is decoded | `800182d0_FUN_800182d0.c`, `80018300_FUN_80018300.c` |
