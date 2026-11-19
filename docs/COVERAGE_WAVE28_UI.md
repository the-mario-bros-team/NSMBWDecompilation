# Coverage - wave 28 HUD/menu continuations (`0x800182D0..0x8017F0B0`)

## How to read this file

- **Scope** - the tenth UI translation pass. It adds 25 functions to the two
  existing semantic modules `src/ui/hud.rs` and `src/ui/menu.rs`; no staging
  module and no new `.rs` file is created.
- **Addresses** are virtual `sys/main.dol` addresses (image base `0x80004000`),
  not file offsets. The CSV uses the plain eight-digit hex form.
- **Evidence** is the Ghidra C export under
  `work/ghidra/main-gb/export/`; every row names its file.
- **Status** follows `docs/RUST_STYLE.md`:
  `translated` = full control flow and every call accounted for;
  `partial` = flow and major effects present, one operand not literal;
  `stub` = address/role only.
- **Confidence** measures the evidence, not code quality. `medium` marks a body
  whose flow and stores are exact but whose virtual-call operand, argument
  split or data role was inferred from the surrounding bodies.
- Start with the **summary**, then the **function tables**. The **cluster
  notes** explain how the warning-scene thunks connect to the existing state
  tables, and how the 0x68-byte record group connects to the battery/text
  widgets already in the two modules.

## Summary

| metric | value |
|---|---:|
| functions added to `src/ui/hud.rs` | 8 |
| functions added to `src/ui/menu.rs` | 17 |
| functions added in total | 25 |
| bytes translated | 3,904 |
| `translated` / `partial` / `stub` | 25 / 0 / 0 |
| high confidence | 9 |
| medium confidence | 16 |
| new staging modules | 0 |
| files written | `src/ui/hud.rs`, `src/ui/menu.rs`, this document, `ledger/agents/wave28-ui.csv`, `ledger/functions.csv` |

Address span: `0x800182D0..0x8017F0B0`. The HUD half closes the two warning
scenes: four forwarding thunks that sit beside the `errorInfo` and nunchuk
state-table builders, the close-notify body, the two open/start marker
writers, and the virtual attach helper. The menu half adds the grid record
destroy twin, the enabled-chain walk, the press/hold widget timer, the
scrolling-text group (constructor, timer arm, per-frame update), three live
menu wrappers, two GX state resets, one viewport allocator, one dialog-record
destructor and the dialog text-quad draw.

## Function table

### `ui::hud` - warning-scene thunks and markers

| address | Rust function | role | status | conf. | evidence |
|---|---|---|---|---|---|
| `0x800182D0` | `fn_800182d0` | forward two operands to the unresolved `0x802DCBFC` stub (`b, b, a`) | translated | medium | `800182d0_FUN_800182d0.c` |
| `0x80018300` | `fn_80018300` | twin immediately before the `errorInfo` state table | translated | medium | `80018300_FUN_80018300.c` |
| `0x80019500` | `fn_80019500` | twin in the nunchuk cluster | translated | medium | `80019500_FUN_80019500.c` |
| `0x80019530` | `fn_80019530` | twin immediately before the nunchuk state table | translated | medium | `80019530_FUN_80019530.c` |
| `0x8001A010` | `fn_8001a010` | notify the `+0x19C` member while the close animation runs | translated | medium | `8001a010_FUN_8001a010.c` |
| `0x8001A080` | `fn_8001a080` | store `+0x1E8` and set the open/start markers | translated | high | `8001a080_FUN_8001a080.c` |
| `0x8001A0A0` | `fn_8001a0a0` | set the two start markers `+0x1E6`/`+0x1E7` | translated | high | `8001a0a0_FUN_8001a0a0.c` |
| `0x8001A0B0` | `fn_8001a0b0` | attach an object if virtual slot `+0xC` accepts it | translated | medium | `8001a0b0_FUN_8001a0b0.c` |

### `ui::menu` - grid record, timer, wrapper and GX bodies

| address | Rust function | role | status | conf. | evidence |
|---|---|---|---|---|---|
| `0x801705E0` | `fn_801705e0` | unlink a grid entry and release its resource | translated | high | `801705e0_FUN_801705e0.c` |
| `0x80171C30` | `fn_80171c30` | clear nine state bytes of a small record | translated | high | `80171c30_FUN_80171c30.c` |
| `0x80172330` | `fn_80172330` | conditional delete of a UI record | translated | high | `80172330_FUN_80172330.c` |
| `0x80172BE0` | `fn_80172be0` | walk a chain of up to five enabled records | translated | medium | `80172be0_FUN_80172be0.c` |
| `0x80173620` | `fn_80173620` | advance one widget's press/hold timer | translated | medium | `80173620_FUN_80173620.c` |
| `0x80173A30` | `fn_80173a30` | update the four scrolling-text widgets | translated | medium | `80173a30_FUN_80173a30.c` |
| `0x80173CD0` | `fn_80173cd0` | construct the four-record widget group | translated | medium | `80173cd0_FUN_80173cd0.c` |
| `0x80173E40` | `fn_80173e40` | arm the scrolling-text timer | translated | medium | `80173e40_FUN_80173e40.c` |
| `0x80174DF0` | `fn_80174df0` | allocate through the global viewport allocator | translated | high | `80174df0_FUN_80174df0.c` |
| `0x80174FA0` | `fn_80174fa0` | refresh the live menu's battery gauge | translated | medium | `80174fa0_FUN_80174fa0.c` |
| `0x80175010` | `fn_80175010` | start the live menu's dialog dismiss | translated | medium | `80175010_FUN_80175010.c` |
| `0x80175140` | `fn_80175140` | reset the live menu's player animations | translated | medium | `80175140_FUN_80175140.c` |
| `0x80175170` | `fn_80175170` | reset the GX state for the text/quad pass | translated | high | `80175170_FUN_80175170.c` |
| `0x80178A20` | `fn_80178a20` | feed one widget's text or arm its timer | translated | medium | `80178a20_FUN_80178a20.c` |
| `0x80178B40` | `fn_80178b40` | poll the release query and rearm the retry timer | translated | medium | `80178b40_FUN_80178b40.c` |
| `0x8017E490` | `fn_8017e490` | dialog-record element destructor | translated | high | `8017e490_FUN_8017e490.c` |
| `0x8017F0B0` | `fn_8017f0b0` | draw the dialog text quad | translated | high | `8017f0b0_FUN_8017f0b0.c` |

All evidence paths are relative to `work/ghidra/main-gb/export/`.

## Cluster notes

### Warning-scene thunks (`0x800182D0..0x80019530`)

The four 48-byte bodies are byte-identical: a save-register prologue, one
`0x802DCBFC(b, b, a)` call and a restore. They sit immediately before the
`dWarningErrorInfo_c` state-table builder `0x80018330` and the
`dWarningNunchuk_c` builder `0x80019560`, so they are the factory trampolines
for those classes (`// inferred:`). `0x802DCBFC` still has an unrecovered jump
table; the Rust passes the three operands exactly as the export prints them
and labels the rows `medium`, matching the wave-19 treatment of
`0x80016A20`/`0x80016A50`.

### Nunchuk open, start and close (`0x8001A010..0x8001A0B0`)

`0x8001A080`/`0x8001A0A0` write the same field block the wave-26 bodies use
(`+0x1E5` open, `+0x1E6` start, `+0x1E7` second start). The caller
`0x8010DA20` passes `scene + 0x74`, which is why the fields are addressed from
the scene base in the export. `0x8001A010` is the close-side counterpart of
`0x80018060`: it queries `0x800C9610(layout, -1)` and, when the query returns
`0`, calls slot `+0x18` of the member at `+0x19C` with the BSS object
`0x803523F8`. `0x8001A0B0` is the attach helper: slot `+0xC` of the candidate
receives the candidate itself, and a zero result installs it at `this + 0xC`.

### Grid record and enabled chain (`0x801705E0..0x80172BE0`)

`0x801705E0` is the destroy twin of the wave-7 `0x8016FE40`; both unlink the
node at `+8` through `0x80228F80` and release the word at `+0` through
`0x801D4710`. `0x80171C30` clears the nine state bytes used by one record.
`0x80172330`/`0x8017E490` are the same conditional-delete body as the
nunchuk destructors. `0x80172BE0` walks the `+0xC` chain: four hops are
unrolled (each tests `byte[+0xCF] & 1`) and the fifth hop uses the getters
`0x80172CE0`/`0x80172CF0` before recursing.

### Widget press/hold timer and scrolling text (`0x80173620..0x80173E40`)

`0x80173CD0` builds the four 0x68-byte records at the manager base
(`+0x50 = 0`, `+0x58 = -1`, `+0x5C = 1`, `+0x62 = 1`), opens the archive into
`+0x1D0` and stores the `+0x1EC` ready byte. `0x80173E40` arms the `+0x1A0`
timer with the 64-bit OS time and the callback `0x80173A30`. The callback
scales up to 40 16-bit samples by `byte[+0x5D] / 10`, zero-fills the tail,
submits the block through `0x80227EF0`/`0x801E3680` and advances the source by
0x50 bytes. `0x80173620` is the independent press/hold state machine on the
same widget: it stores the 64-bit stamp at `+0x38`, latches `+0x42`/`+0x43`,
and after 480,000 ms sends notification `2`, arms the `0x80377EC8` timer and
sets `+0x41`.

### Live-menu wrappers and GX resets (`0x80174DF0..0x8017F0B0`)

`0x80174DF0` passes the viewport pointer from `0x8032A304` and the requested
size to `0x801D5540`. The three wrappers `0x80174FA0`, `0x80175010` and
`0x80175140` fetch the live menu with `0x80175500` and forward it to the
already-translated `0x801788C0`, `0x8017D1D0` and `0x8017D3E0`; the export
prints the second call without an operand because the menu is still in `r3`.
`0x80175170` and `0x8017F0B0` are straight GX state sequences; the second one
ends with the `0xCC008000` FIFO write of `param_1 + 4`.

## Ledger reconciliation

`0x801735B0` already had a translated body in `src/ui/menu.rs` (and a matching
`# Original` block) but no row in `ledger/functions.csv`. It was therefore not
counted in this wave and not re-translated. Its row should be added by the lane
that owns that earlier pass.

## Unknowns and limits

1. **`0x802DCBFC`** - the four HUD thunks forward three operands to a stub
   whose jump table is not recovered. The Rust reproduces the call only and
   marks the rows `medium`.
2. **Register-save prologues** - `0x80173CD0`, `0x80173E40` and `0x80173620`
   recover their `this` from the `0x802DCD6C`/`0x802DCD74` save helpers, which
   Ghidra models as returning the incoming `r3`/`r4`. The argument split of
   `0x80173CD0` is confirmed by its only caller
   (`FUN_80173cd0(manager, archive)` in `0x80175510`); `0x80173E40` is
   confirmed by `FUN_80173e40(manager + 0x2BC)` in `0x80175F70`.
3. **Forwarded menu operand** - `0x80174FA0`, `0x80175010` and `0x80175140`
   fetch the menu and then call a one-argument body with no printed operand.
   The Rust forwards the fetched menu because that value is still in `r3`,
   matching the printed wave-26 wrappers.
4. **`0x80173A30` scaling** - the original performs two sign corrections around
   the `/10`; they cancel for every input, so the Rust keeps the single signed
   divide and records the observation. The SDA2 polynomial inside
   `0x80227EF0` is outside this pass.
5. **Release-query thunks** - `0x801DFAC0`/`0x801DFAD0` are 4-byte thunks to
   `0x801F3840`/`0x801F38F0`; their `r3` value is consumed as a flag even
   though the export declares the bodies `void`.
6. **Unused argument** - `0x8017F0B0` never reads `param_2`; the parameter is
   kept so the signature matches the export.
7. **GX setters** - the `0x801Cxxxx` calls in `0x80175170`/`0x8017F0B0` are
   addressed by number; the export gives no names, so only the operand order
   is documented.

## Verification

```text
cargo check          # exits 0; no diagnostic references hud.rs or menu.rs
node tools/doc-lint.mjs src
# {"ok": true, "files": 68, "violations": 0, "byRule": {}}
```

The crate check was run after the edits settled; it exits 0 and no warning or
error names `src/ui/hud.rs` or `src/ui/menu.rs`. `doc-lint` passes with 0
violations.

Only `src/ui/hud.rs`, `src/ui/menu.rs`, this document,
`ledger/agents/wave28-ui.csv` and the 25 matching rows appended to
`ledger/functions.csv` were written. `src/lib.rs`, every `mod.rs`,
`Cargo.toml`, `src/platform.rs`, the generated coverage files and the other
lanes were left untouched.

## Next targets

| priority | target | size | why | first evidence |
|---|---|---:|---|---|
| 1 | `0x80175510` | 2,144 B | constructs the full menu viewport; calls `0x80173CD0` | `80175510_FUN_80175510.c` |
| 2 | `0x80175F70` | 1,184 B | battery/controller pane builder; calls `0x80173E40` | `80175f70_FUN_80175f70.c` |
| 3 | `0x801769B0` | 7,420 B | the large dialog layout body | `801769b0_FUN_801769b0.c` |
| 4 | `0x8017DCC0` | 728 B | dialog animation task; contains paired-single quantisation | `8017dcc0_FUN_8017dcc0.c` |
| 5 | `0x8017EA10` | 1,376 B | dialog close/confirm vertex draw | `8017ea10_FUN_8017ea10.c` |
| 6 | `0x801701A0` | 612 B | scrolling-text draw body next to this wave's record group | `801701a0_FUN_801701a0.c` |
| 7 | `0x8016FD30`, `0x801704D0` | 228 B each | grid constructors; need the two dropped ABI operands (`extraout_r4`, `extraout_f1`) | `8016fd30_FUN_8016fd30.c` |
