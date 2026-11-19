# Wave 218, lane F - `src/engine/dsys.rs`, addresses `0x800FF6A0..0x800FFE20`

Eight entries of `work/ghidra/main-gb/export/functions.tsv` (rows 3462-3469),
all in the `0x800FF000` run of the `engine::dsys` ledger module. All eight are
translated; none deferred.

| address | tsv size | real extent | body |
|---|---|---|---|
| `0x800FF6A0` | 8 | 8 (tail branch, no `blr`) | callback thunk `store(slot - 0x11)` |
| `0x800FF6B0` | 8 | 8 (tail branch, no `blr`) | callback thunk `record(slot - 0x11)` |
| `0x800FF6C0` | 108 | 108 (`blr` at `0x800FF728`, 4 B padding) | copy twelve floats into record `n` |
| `0x800FF730` | 16 | 16 (`blr` at `0x800FF73C`) | address of record `n` |
| `0x800FF740` | 780 | 780 (`blr` at `0x800FFA48`, 4 B padding) | arc length, cubic travel offset, per-segment pitch |
| `0x800FFA50` | 496 | 496 (`blr` at `0x800FFC3C`, no padding) | snap the records inside the window |
| `0x800FFC40` | 356 | 356 (`blr` at `0x800FFDA0`, 4 B padding) | nearest-record probe and actor snap |
| `0x800FFDB0` | 112 | 112 (`blr` at `0x800FFE1C`) | feed one `x` sample into the tracker |

No `functions.tsv` size in this lane is a prologue artefact. The two 8-byte
entries are pure tail branches (`addi r4,r4,-17` plus `b`), and every other
entry ends in a `blr` at `start + size - 4`; the four bodies that are followed by
a padding word are the 108/780/356-byte ones listed above.

Bytes translated: 1884.

## What the run is

The geometry side of the actor whose build pass `0x800FE760` (wave 218 lane E)
runs last. The actor owns six 0x30-byte (twelve-float) transforms at `+0x2A4`;
the callbacks address them as virtual slots `0x11..0x16`, hence the `-0x11`
bias in the two thunks.

Fields used by the lane:

| offset | type | use |
|---|---|---|
| `+0x20` | pointer | actor that receives the snapped position |
| `+0x294` | `u32` | mode; the rebuild runs only when it is `1` |
| `+0x298` | `f32` | accepted `x` window, shrunk to the best hit |
| `+0x29C` | `f32` | last sample |
| `+0x2A0` | `u32` | "strip created" latch |
| `+0x2A4` | `f32[6][12]` | one 3x4 transform per segment, stride `0x30` |

`0x800FF740` rebuilds the strip: it clamps the travel of the sample against
`+0x29C` to `[-30, 30]`, accumulates the six segment lengths through
`0x801C14C0`, builds a translation transform whose `y` is
`travel * t^3 / 1000` (`t` = arc length / `10`), multiplies it by the transform
`0x801C0C60` builds from three zero constants, and keeps the product in a stack
scratch array. The second pass copies each record, **adds** the matching scratch
transform (`0x80238130` is a twelve-lane addition, translated in
`src/resource/bres.rs`), stores the negated vertical angle from the previous
segment (`0x80160EA0`) and writes it through `0x8016EEF0`.

`0x800FFA50`, `0x800FFC40` and `0x800FFDB0` are the window side: they sample the
actor position through the `0x800FF660` wrapper (`0x800D5790(object, out,
0x11)`), compare `x`, and either rebuild the transform of every record inside the
window (`0x800FFA50`), probe the nearest point on a segment with `0x80075FD0`
and snap the actor through `0x80152880` (`0x800FFC40`), or shrink the window to a
new sample (`0x800FFDB0`).

## Callees and constants

New consts carry the lane letter `_F`: `FN_800D5790_F`, `FN_800FF660_F`,
`FN_80160EA0_F`, `FN_80238130_F`, `FN_80075FD0_F`, `FN_80152880_F`, and the
seven SDA floats `F_6024_F` (`0.0`), `F_5FFC_F` (`30.0`), `F_5FF8_F` (`-30.0`),
`F_5FF4_F` (`10.0`), `F_5FF0_F` (`1000.0`), `F_5FEC_F` (`4.0`), `F_6010_F`
(`0.1`). The SDA floats are read at run time because the exports print them as
unresolved `unaff_r2` operands. `FN_8016F130`, `FN_8016EEF0`, `FN_801C14C0`,
`FN_801C0BD0`, `FN_801C0C60` and `FN_801C0540` already existed in the module and
are reused.

`0x800FFA50`'s export is incomplete: it drops the `0.1` test at `0x800FFB34`,
the ratio division at `0x800FFB3C` and the nine `* dVar4` stores at
`0x800FFB84`. The translated body follows the instruction stream and keeps the
export's call sequence.

## One evidence conflict

At `0x800FF9B8`/`0x800FF9C8`/`0x800FF9D0` (inside `0x800FF740`) the instruction
stream of `work/dol/main.elf` is `neg r0,r3` / `extsh r0,r30` /
`sth r30,0x8(r1)`, so the halfword store reads `r30` (the finished loop counter)
while the two computations write `r0`. The export prints
`local_268[0] = -sVar5`, which is the only reading under which the two
instructions are live. The translated body follows the export and the
`# Confidence` block of `fn_800ff740` records the discrepancy.

## Verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to a scratch directory, the
lane block was appended to the copy of `src/engine/dsys.rs`, and
`cargo check --lib` there exits `0` with no diagnostic pointing inside the
appended block.
