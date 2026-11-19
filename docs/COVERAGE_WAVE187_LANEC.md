# Coverage - Wave 187, lane C (`game::bases`, sub-300-byte sweep)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 10 | 1600 | 9 | 1 |

All ten lane-C addresses are 136-192 bytes in
`work/ghidra/rel-d_basesNP/export/functions.tsv`. Lane C added no helpers and did
not touch `src/game/bases.rs`; the block is in `work/wave187/laneC_block.rs`, the
ledger is `ledger/agents/wave187-lanec.csv`, and the bodies are appended by the
coordinator.

## Translated

- `0x806376E0` (184 bytes) - results-screen start cue stepper: the `+0x8D4`
  counter selects cue `0x9B..0x9E` on the sound manager `0x8042_A488`, then the
  counter is incremented unconditionally.
- `0x806385A0` (192 bytes) - four-player start countdown: polls the twelve pane
  handles in table `0x807C_7EF0` (`0x21..0x2C`) and returns on the first busy
  one; otherwise increments `+0x8C4`, reloads `0x1E` into `+0x8C8` and
  dispatches `0x8082_6C20` while the count is below 4, else `0x8082_6CA0`.
- `0x80638680` (144 bytes) - start/abort descriptor select: while `+0x8AF` is
  set, `0x8063_7560(this)` zero runs [`fn_80637310`] and dispatches
  `0x8082_6D20`, otherwise `0x8063_7610(this, 0)` then `0x8082_6D60`.
- `0x80638AD0` (152 bytes) - binds the four `0x3C`-stride controller-pane
  animations at `+0x7E0`, choosing `pane + 4` for records whose `+0x7C0` word
  differs from the reference record's.
- `0x8063A680` (180 bytes) - rebinds the twelve pane words at `+0x25C..+0x29C`
  onto the four animation objects at `+0x2D0`, using each record's `+0x2E0`
  player id and blend flags `1,0,0,1`.
- `0x8063A8C0` (140 bytes) - copies record `+0x2CC` flag bits 0/1/2 into bit 0 of
  `+0xBB` on the three pane objects selected by the `0x807C_7FC0`/`7FD0`/`7FE0`
  grouping tables ({0,3,6,9}, {1,4,7,10}, {2,5,8,11}).
- `0x8063ABE0` (172 bytes) - sets bit 0 of `[[this+0x244]+0xBB]`, adds the four
  handles `+0x24C..+0x258` to the manager at `+0x248` via `0x802A_B810`, then
  binds one handle per record with `0x802A_B7C0`.
- `0x8063D4E0` (164 bytes) - results-screen idle reset: gated by
  [`fn_80600510`] and `[[0x8042_A2D8]+0xB00] < 1`; re-enables system input,
  plays sound `(0xB, 0, 0)` and dispatches `0x8082_7460` on `+0x208`.
- `0x8063F060` (136 bytes) - polls the `+0xAC` animation with `[this + 0x274]`
  and dispatches `0x8082_75A0` (bit `0x10` of `0x8042_9F80` clear) or
  `0x8082_75E0` (set) on `+0x70`.
- `0x8063F450` (136 bytes) - plays sound `(0x79, 1)`, resets `0x8042_A2C8`, and
  binds `[this+0x26C]+9` or `+0xC` on the `+0xAC` animation depending on
  `0x8042_9F80 & 0x10`.

## Deferred

None.

## Notes and limitations

- `0x8063A680`'s base is the incoming `r3`: the `0x802DCD68` `_savegpr_24`
  prologue does not clobber `r3`, so Ghidra prints `void FUN_8063a680(void)` and
  models the helper's return value as the object base. The Rust takes `this` and
  emits neither `0x802DCD68` nor `0x802DCDB4`.
- `0x80638680`'s `0x8063_7560` and `0x8063_7610` exports also print no usable
  argument (`_savegpr` prologue); the raw words keep `r3 = this` up to both
  `bl` sites and set `r4 = 0` only for `0x8063_7610`, so the calls are emitted
  with `(this)` and `(this, 0)`. `0x8063_7560` has no Rust body yet.
- `0x8063A8C0` is `medium`: the three `rlwimi` inserts are encoded with `SH`
  values `0,31,30`, which Ghidra folds to `>> 0/1/2 & 1` and which the Rust
  spells that way. The grouping tables make the member order unambiguous (id
  selects a group of three consecutive objects, bit `n` selects member `n`), but
  the shift-count encoding itself was not force-decoded bit-for-bit.
- `0x8063D4E0` tests the same two `main.dol` mode bytes as [`fn_80600510`]
  (`0x8042_A2DD` and `0x8042_A2DC`); the Rust reuses that helper instead of
  duplicating the two byte reads.
- `0x8010_7F10` is called with the neighbour's argument form
  (`abi_call1!(0x8010_7F10, 0)`, as in [`fn_8063d160`]); the raw `bl` passes
  whatever `r3` held, and the result feeds `0x8010_8160` exactly as Ghidra
  shows.
- Callee roles for `0x800C_9610`/`0x800C_92F0`/`0x8019_5380`/`0x8010_C780`/
  `0x800E_1E60`/`0x800B_3BE0`/`0x802A_B810`/`0x802A_B7C0` follow the existing
  `d_basesNP` translations (`fn_8063f300`, `fn_8063d160`, `fn_80635b10`); every
  argument value is read straight from the raw words.
