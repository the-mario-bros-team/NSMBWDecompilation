# Coverage - Wave 195 lane A (`game::bases`, 256-300 byte sweep)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Deferred |
|---|---|---:|---:|---:|---:|---:|
| `src/game/bases.rs` (lane A block) | `game::bases` | 10 | 2792 | 9 | 1 | 0 |

Ten bodies, all from the 256-300 byte band. Source of the bodies:
`work/wave195/laneA_block.rs`; per-function ledger:
`ledger/agents/wave195-lanea.csv`.

## What this pass covers

- **`dModelPlayDate_c` layout loader (1 body).** `0x80626B80` (268) is the
  sibling of the `dFileSelect` loader already translated as `fn_80615800`:
  gate on `+0x130`, layout object at `+0xAC`, vtable at `+0xB4` with the
  explicit slot `+0x14`, then `0x800C8FB0`/`0x800C90F0`/`0x80007220`/
  `0x800C8F20`/`0x800C8EB0`/`0x800C8E40`/`0x800C8D60`/`0x800C8DD0` against
  `modelPlayDate_09.brlyt`, `onModel.brlan`, `A00_model`, `T_needCoinX_01`,
  `T_modelTitle_00`, `P_model1_00`, `N_Coin_00` and `W_model1_00`.
- **Pane transform broadcast (1 body).** `0x80628610` (296) reads one NW4R
  transform with `0x800C9680(source[+0xAC], source[+0x278])` into the existing
  private 17-byte `LayoutTransform` type, then writes its four words and tail
  byte into ten panes: five pointers at `+0xB4..+0xC4` in each of two passes
  (`+0x14` owner stride).
- **Save-board selection (1 body).** `0x8062BCF0` (296) steps a selection
  index over eleven slots: backward search from `+0x500` down through the
  `+0x516` occupancy bytes with `+0x43C = 2`, forward search with
  `subfic ...,11` and `+0x43C = 1`, and wrap-to-`10`/`0` when the slot count
  at `+0x504` is at least `2`.
- **Coin-pane hide + idle dispatch (1 body).** `0x8062CFF0` (288) sets
  `+0x291 = 1` on twenty panes at `+0x2DC..+0x328`, optionally runs
  `0x8062C0B0` when bit `6` of the global word `0x80429F80` is set, calls the
  already-translated `fn_8062c5f0`, clears `+0x50F..+0x511` and dispatches
  `0x80825FC8` through `[+0x32C]->+0x18`.
- **Result-pane vector refresh (1 body).** `0x80636F30` (288) runs only for
  `0x800B4610()` values `3..=5`; for each of four source/node pairs it asks
  `[+0x74C+i*4]->+0x24` for words `0` and `2`, forces the node translate
  `+0x30` to `11.0` (constant `0x807C7E80`) and calls
  `[+0x75C+i*4]->+0x28` with the two three-word vectors.
- **Result-record promotion (1 body).** `0x80637450` (264) compares each of
  the three records' `+0x7C0` with `+0x7FC`, copies `+0x7F8..0x830` into
  `+0x7BC..0x7F4` (including four `frsp` round-trips) and recurses on `this`.
  It is the callee already invoked by the translated `fn_80637ed0`.
- **Start-panel bit application (1 body).** `0x80638440` (292) uses the mask
  `(1 << [+0x8C4]) & 0xFF` against each record's `+0x7BC` byte, adds `10` to
  the matching record's `+0x7C0`, rebinds its `+0x7E0` pane (`+0x1D` and the
  `0x807C7EC0 + pane*12 + count*4` channel), calls `fn_80637050` and
  dispatches `0x80826C60`; when no record matches it bumps `+0x8C4` and
  dispatches `0x80826CA0` once the count exceeds `3`.
- **Result-pane text init (1 body).** `0x80638C80` (268) is the flag-less
  sibling of the translated `fn_80637610`: token `0x800CDC60`, four-record
  loop binding `+0x74C`/`+0x75C` with message id `0x65`, page word `+0x8D0`,
  then channel `record[+0x7E0] + 12` through `0x800C92F0`, `+0x8D4 = 0`,
  `+0x8D8 = 0x1E` and the `fn_806376e0`/`fn_80637910` tail.
- **28-byte record cascade (1 body).** `0x8063A740` (260) walks
  `(3 - column)` records of `0x1C` bytes from `this + column*0x1C`; when
  `+0x2E5 == 0` and `+0x301 != 0` it shifts `+0x2CC..+0x2FC` down/up with two
  packed-byte extracts (`+0x2CC >> 24` into `+0x2E8`, `+0x2E4 >> 24/16` into
  `+0x300`/`+0x301`), otherwise it only sets `+0x300` when `+0x2D0` equals
  `+0x2EC`. This is the callee left as `abi_call2!` by the translated
  `fn_8063a850`.
- **Lift state reset (1 body).** `0x8064AAC0` (272) copies `+0x630` to
  `+0xB0`, writes `1.0` (`0x807C8570`) to `+0xDC/+0xE0/+0xE4`, clears
  `0x80166EF0(this+0x704, 0)` plus six words, bumps the global byte at
  `[[0x80429DD0] + 0x900C4]` by `+3`, and in mode `1` remaps `+0x348`
  through `0x807C858C`/`0x807C8590` with `+0x75A` from `0x807C8568`; tail is
  `[+0x60]->+0xD4(this, 0x80827FD0)`.

## Reuse and helper notes

- No new helpers were introduced. The block uses `platform::read_u8` /
  `read_u16` / `read_u32` / `read_i32` / `read_f32` / `write_u8` / `write_u16`
  / `write_u32` / `write_i32` / `write_f32` / `call0`, `ptr_at`,
  `vmethod1`, `abi_call1`..`abi_call5`, `call0_ret` and the existing private
  `LayoutTransform` stack type.
- Direct Rust calls to already-translated bodies: `fn_8062c5f0`,
  `fn_80637050`, `fn_806376e0`, `fn_80637910`, and the self-recursion in
  `fn_80637450`.
- `0x80637450`'s prologue/epilogue `0x802DCD70`/`0x802DCDBC` are the
  `_savegpr`/`_restgpr` thunks; they are dropped and `r3` (the receiver)
  survives into the body, which is why the recursive `bl` passes `this`.
- `0x80638C80` also starts with `_savegpr`; same treatment.

## Verification

- `work/wave195/laneA_block.rs` was appended to a scratch copy of the crate;
  `cargo check --offline` passes, and `node tools/doc-lint.mjs <scratch>/src`
  reports `{"ok": true, "files": 68, "violations": 0}`.
- Every raw word was re-read from `work/rels/elf/d_basesNP.elf`; each body's
  byte size matches `functions.tsv` (268, 296, 296, 288, 288, 264, 292, 268,
  260, 272 = 2792).
- Constants read directly: `0x807C7E80 = 11.0`, `0x807C8570 = 1.0`,
  `0x807C858C = {3,2,0,1}`, `0x807C8590 = {2,3,1,0}`,
  `0x807C8568 = {49152,16384,32768,0,...}` (u16). String addresses in
  `0x80626B80` were derived from the `lis/addi` pair base `0x807DAFC0`.
- All ten evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`; the CSV targets `src/game/bases.rs`.
- The one `medium` row is `0x80636F30`: every offset, slot and constant is
  literal, but the source slot `+0x24` is modelled as a one-word out-parameter
  call (`out`, receiver, index) because the two four-byte stack results in the
  frame show it writes one word, not an NW4R type with a known name.

## Deferred

None. All ten bodies were proven from the raw instruction stream.
