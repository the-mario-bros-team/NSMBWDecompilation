# Wave 215, lane B - `0x800F0B60..0x800F103C` (`engine::dsys`)

Ten bodies of the sprite/keyed-track and model-block helper run above
`dSys`. All ten are `translated`; the two `medium` entries depend on
evidence the Ghidra export cannot carry (the `0x802DCD74` register-save stub
and the two forwarded words of a tail thunk).

Block file: `work/wave215/laneB_block.rs`
Ledger: `ledger/agents/wave215-laneB.csv`

## Bodies

| address | size (tsv) | real extent | name | confidence | role |
|---|---|---|---|---|---|
| `0x800F0B60` | 164 | `0x800F0B60..0x800F0C04` (164) | [`fn_800f0b60`] | high | named `+0x688` slot carries `0.0` on the `+0x9C` track |
| `0x800F0C10` | 164 | `0x800F0C10..0x800F0CB4` (164) | [`fn_800f0c10`] | high | named `+0x548` slot carries `0.0` on the `+0x70` track |
| `0x800F0CC0` | 144 | `0x800F0CC0..0x800F0D4C` (140) | [`fn_800f0cc0`] | medium | named live timed slot + pending-name predicate |
| `0x800F0D50` | 208 | `0x800F0D50..0x800F0E20` (208) | [`fn_800f0d50`] | high | keyed event-table test (`+0x7DC`, 0xE-byte records) |
| `0x800F0E20` | 144 | `0x800F0E20..0x800F0EB0` (144) | [`fn_800f0e20`] | high | registered-child name search (`+0xB64`/`+0xB68`) |
| `0x800F0EB0` | 8 | `0x800F0EB0..0x800F0EB8` (8) | [`fn_800f0eb0`] | medium | tail thunk `0x80168230(object + 8, key, out)` |
| `0x800F0EC0` | 104 | `0x800F0EC0..0x800F0F28` (104) | [`fn_800f0ec0`] | high | resolve one name, then bind it |
| `0x800F0F30` | 64 | `0x800F0F30..0x800F0F70` (64) | [`fn_800f0f30`] | high | resolve one name only |
| `0x800F0F70` | 72 | `0x800F0F70..0x800F0FB8` (72) | [`fn_800f0f70`] | high | `(lookup >> 31) ^ 1`: name is present |
| `0x800F0FC0` | 124 | `0x800F0FC0..0x800F103C` (124) | [`fn_800f0fc0`] | high | index of the first present start clip, else `5` |

Nine sizes were re-checked against the prologue (`stwu r1,-N`) and the closing
`bclr` and agree with `work/ghidra/main-gb/export/functions.tsv` byte for byte.
One exception:

* `0x800F0CC0` - `functions.tsv` says 144, but the `bclr` is at `0x800F0D4C`,
  so the body itself is 140 bytes and the four trailing bytes are the
  alignment pad up to `0x800F0D50`. The doc block keeps the `functions.tsv`
  value in `# Original` and states the real extent.

`0x800F0EB0` is the second kind of artefact: it is a two-instruction tail thunk
(`addi r3,r3,8` / `b 0x80168230`) with no `bclr` of its own. Its 8-byte
`functions.tsv` size is the real extent.

## Shared layout

| offset | meaning |
|---|---|
| `+0x70` | first animation track handle of a sprite |
| `+0x9C` | second animation track handle of a sprite |
| `+0x548` | 16-slot name array (stride `0x14`) of the second pool |
| `+0x688` | 16-slot name array (stride `0x14`) of the first pool |
| `+0x7DC`/`+0x7E0` | keyed event table (0xE-byte records) and its count |
| `+0xB64`/`+0xB68` | registered-child pointer array and its count |
| `+0x08` | model block handed to `0x80168400` and `0x80168230` |
| `+0xC8`/`+0xDC` | timed-slot name and live flag (stride `0x48`) |

The event records at `+0x7DC` are fetched with `0x800FD720`, which returns
`table + 8 + index * 0xE` (or null past the count). A record carries a tag byte
at `+0xB` (`'a'` = `0x61`) and a 9-byte name at `+2`; the float read through
`0x800EFE60` is tested against the two sentinels `r2 - 0x619C` (`-0.8`) and
`r2 - 0x61B0` (`1.0`).

## Callees

| address | shape | role |
|---|---|---|
| `0x802E1AB4` | `(a, b) -> i32` | byte-wise `strcmp`; `0` = equal |
| `0x802E1BD0` | `(a, b, n) -> i32` | `strncmp`; `0` = equal |
| `0x800FD720` | `(table, index) -> ptr` | 0xE-byte record fetch |
| `0x800EFE60` | `(object, record) -> f64` | one `f32` of a sprite record |
| `0x800F1B90` | `(child, name) -> u32` | `cntlzw(strncmp(...)) >> 5` |
| `0x80168400` | `(object + 8) -> u32` | builds the one-word lookup key |
| `0x80164F00` | `(key, name) -> i32` | archive lookup, `-1` when absent |
| `0x80168230` | `(object, key, out) -> bool` | binds/fetches one node |

## Data read to resolve meaning

* `r2 - 0x61AC` = `0x8042CED4` = `0.0` (the "no offset" word already named
  `F_CONST_61AC` in the module).
* `r2 - 0x619C` = `0x8042CEE4` = `-0.8` and `r2 - 0x61B0` = `0x8042CED0` =
  `1.0`; both are compared exactly (`fcmpu`) in [`fn_800f0d50`].
* `0x8031BA90` holds the five start-clip name pointers scanned by
  [`fn_800f0fc0`]: `"start"`, `"startPan"`, `"startTilt"`, `"startFix"`,
  `"startW8"`.

## Evidence notes (Ghidra export artefacts)

* `0x802DCD74` is the outlined register-save stub
  (`stw r27..r31,-0x4..(r11)` + `bclr`) and `0x802DCDC0` its restore twin.
  Ghidra prints the save stub as returning `u64`; in [`fn_800f0cc0`] the two
  words are the live `r3`/`r4` (object base and key), exactly as in the other
  bodies of this module that start with the stub. The final `r3` leaves through
  the restore twin, which is why the exported prototype is `void`.
* Every export of these bodies shows `ldexpf` / `unaff_GQR0` / `unaff_r2`
  noise. That is the dead `f31` save and the CodeWarrior float-to-double
  conversion scaffolding, not executed code.
* `FUN_800f0eb0` is printed with one parameter because `r4`/`r5` are only
  forwarded to the tail callee; the three-argument call site in
  [`fn_800f0ec0`] and `0x80168230`'s own signature fix the real arity.
