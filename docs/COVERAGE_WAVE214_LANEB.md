# Coverage - Wave 214, lane B (`game::player`, the `0x8006CC00..0x8006CE70` motion kernel)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 9 | 652 | 7 | 2 | 0 |

Status split: 9 `translated`, 0 `partial`, 0 deferred.

Body block: `work/wave214/laneB_block.rs`. Ledger rows:
`ledger/agents/wave214-laneB.csv`.

## How to read this file

* **Address** - virtual entry point in the DOL image, not a file offset.
* **Name** - the Ghidra name of the body, which the Rust function keeps as
  `fn_<address>`; every new constant in the block is suffixed `_B`.
* **Size** - the `functions.tsv` value. All nine were re-derived from the
  prologue and the closing `blr`; four of them (`0x8006CD90`, `0x8006CDE0`,
  `0x8006CE10`, `0x8006CE70`) include four bytes of alignment padding after the
  `blr`, so their instruction extent is four bytes shorter than the inventory
  value. None is a 16/20-byte prologue artefact; the smallest real body here is
  the 8-byte tail call at `0x8006CD40`.
* **Confidence** - strength of the evidence: `high` means every address, field
  offset, immediate, comparison and branch is literal in the disassembly;
  `medium` marks the two bodies whose *callee roles* are inferred rather than
  observed.
* `r2 = 0x80433080` throughout, so `r2 - 0x7448` = `0x8042BC38`.

## What this pass covers

This is the shared per-frame motion kernel of the `0x8012xxxx` actor family.
`0x80121B00` calls `0x8006CD90`, `0x8006CDE0` and `0x8006CD40` in that order;
`0x8011F700`, `0x8011FED0`, `0x80120780`, `0x801217C0`, `0x80123xxx` and the
`0x801133B0` group call subsets of the same nine. Field map, consistent across
the whole cluster:

| offset | type | role |
|---|---|---|
| `+0x7C` | 12 `f32` | pose matrix (built by `0x801C0BD0`) |
| `+0xAC..+0xB4` | `f32` x3 | world position |
| `+0xE8..+0xF0` | `f32` x3 | velocity, integrated by `0x8006CD40` |
| `+0xF4` | `f32` | horizontal-velocity reference of `0x8006CD90` |
| `+0xF8` | `f32` | vertical floor of `0x8006CDE0` |
| `+0x102` | `u16` | second angle of the `+0x100..+0x104` triple |
| `+0x108` | `s16` | heading (`0x10000` per turn) |
| `+0x10C` | `f32` | speed |
| `+0x110` | `f32` | speed target |
| `+0x114` | `f32` | vertical accel/step |
| `+0x118` | `f32` | vertical reference / floor |
| `+0x11C` | `f32` | ramp step |

### Velocity rebuild - `0x8006CC00` (176 B, medium)

The signed heading at `+0x108` is loaded twice (Ghidra shows the two
`psq_l`-quantised conversions), scaled by the `1/256` word at `0x8042BC38` and
evaluated by `0x80237BD0` (sine) then `0x80237C40` (cosine). The vertical
component is ramped first - `+0xEC = max(+0xEC + +0x114, +0x118)` - and the two
horizontal components are then the speed times the two results:
`+0xF0 = +0x10C * cos`, `+0xE8 = +0x10C * sin`. Note the sine lands in `+0xE8`
(x, the component `0x8006CD40` adds to `+0xAC`) and the cosine in `+0xF0`.

### Pose refresh - `0x8006CCB0` (80 B, medium)

`0x801C0BD0(this + 0x7C, x, y, z)` rebuilds the matrix translation from
`+0xAC..+0xB4`; the `u16` at `+0x102` is copied to the stack and handed to
`0x8016EDD0(this + 0x7C, &angle)`. That callee is already translated as
`resource::registry::fn_8016edd0`, which returns immediately for a zero id.

### Position integration - `0x8006CD00` / `0x8006CD40` (52 B / 8 B, high)

`0x8006CD00` adds a caller-supplied `f32[3]` to `+0xAC..+0xB4`.
`0x8006CD40` is the tail call `0x8006CD00(this, this + 0xE8)`, i.e. the
velocity is integrated into the position. Its `functions.tsv` size is 8 bytes
and the shipped slot holds exactly the `addi`/`b` pair.

### Ramps - `0x8006CD90`, `0x8006CDE0`, `0x8006CE10`, `0x8006CE70` (72/36/84/84 B, high)

* `0x8006CD90` steps `+0xE8` toward `+0xF4` by `+0x11C` and clamps at `+0xF4`;
  it stores unconditionally.
* `0x8006CDE0` is the vertical ramp alone: `+0xEC = max(+0xEC + +0x114, +0xF8)`.
* `0x8006CE10` steps `+0x10C` toward `+0x110` by `+0x11C`, clamps at the target,
  and only writes when the value changed. The `bclr 4,1` at `0x8006CE40` is a
  redundant guard the compiler left after the first compare and is reproduced
  as an early return for the not-taken half.
* `0x8006CE70` steps `+0xEC` by `+0x114` in the direction *away* from `+0x118`,
  clamping on the reference, again writing only on change. Its two halves are
  swapped relative to `0x8006CE10`: below the reference it subtracts, above it
  adds. With `+0x114` stored as the negative vertical acceleration implied by
  `0x8006CDE0`, the arithmetic degenerates to the same ramp toward the
  reference; the literal sign is what the block reproduces.

### Glue - `0x8006CD50` (60 B, high)

`0x8006CE10(this)` (r3 is untouched, so the implicit argument is `this`), then
`+0xE8 = +0x10C`, then `0x8006CE70(this)`.

## Re-held judgement calls

* `0x80237BD0` / `0x80237C40` are read as sine / cosine of a 256-units-per-turn
  angle. This is the wave-103 identification, reused here through the existing
  `W103_SIN_80237BD0`, `W103_COS_80237C40` and `w103_sin_cos` items rather than
  redeclared.
* Ghidra's `0x8006CC00` export is dominated by `GQR`/`ldexpf` noise. The real
  body was re-disassembled from `main.elf`: the `sth` + `psq_l ...,1,3` pair
  around `0x8006CC20` is the `(s16) -> f32` conversion of `+0x108`, and the two
  results are `+0x10C * sin` / `+0x10C * cos` in `+0xE8` / `+0xF0`.
* The Ghidra C variable names for `0x8006CD90`/`0x8006CE70` are swapped with
  respect to their registers; the branch directions, the `fadds`/`fsubs`
  halves and both clamps were therefore checked instruction by instruction
  (`fcmpu` operand order resolved from the encoding, not the disassembler's
  `F63_32` fallback printout).

## Verification

The block was appended to a scratch copy of `src/` + `Cargo.toml` +
`Cargo.lock` (`%TEMP%\opencode\w214b`) and `cargo check --lib` exited 0 with the
crate's 23 pre-existing warnings; none refers to `src/game/player.rs` or to
this block. Each `# Original` byte size was cross-checked against
`work/ghidra/main-gb/export/functions.tsv` by the same regular expression
`tools/check-lane-csv.mjs` uses, and all nine match.

## Not covered here

The neighbouring bodies (`0x8006CED0`, `0x8006C0E0`, `0x8006C420`,
`0x8006A4B0`, …) belong to other lanes or earlier waves. This lane only adds
the nine addresses listed in `ledger/agents/wave214-laneB.csv`.
