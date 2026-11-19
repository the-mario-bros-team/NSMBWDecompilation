# Coverage - Wave 108 (`game::worldmap`, `0x80120000` bucket remainder)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 10 | 4,140 | 1 | 9 |

All ten rows are `translated`; none is a `stub` or `partial`.

## What this pass covers

The `0x80120000` bucket had 14 untranslated entries when wave 107 finished.
Ten are written here; the four deliberate skips are listed below. The pass
covers five clusters:

- **`0x80120310` - the fall actor's sine/contact state.** Steps the `+0x104`
  halfword ladder toward zero, probes the ground into `+0x5F4`, classifies the
  height delta into the `+0x5F8` contact state and drives `+0xB0` either
  through the `0x8015F140` ease (`r2 - 0x5A90` rate, `1.0` limit, `0.05`
  deadband, `0.3` clamp) or through the `+0x5F0` halfword sine (`+0x280` per
  frame, `0x80237BD0`, `r2 - 0x5AF8` = `1/256` angle scale, `r2 - 0x5A88`
  gain). The tail runs the `+0x1EC` collider query and posts `0x803756F8` or
  detaches via `0x8011EBB0`.
- **`0x801252D0` - the `+0x3C4` pose snapshot.** Seeds the two static
  vector triples at `0x80375C08`/`0x80375C14`, resolves the player through
  `0x80125F10`, steps the `+0x3C4` heading with `0x8015F200`, copies the
  `+0x480..+0x4A0` record and runs virtual slot `+0x0C` of `+0x47C`.
- **`0x801260E0`/`0x801260F0` - the tile query.** The 4-byte thunk forwards
  its three arguments. The query calls `0x80077650`/`0x80077700` with the
  low 16 bits of the point's integer parts, classifies cells `0x38`, `0x39`
  and `0x3B`, maps the point to the 16-unit grid through the
  `0x43300000` fixed-point-to-double idiom, and walks upward in 16-unit
  steps until it finds a `0x3B` cell.
- **`0x80126310` - the `+0x3BC..+0x3CC` damping ladder.** Tests the
  `0x801C14C0` distance against the `r2 - 0x599C` epsilon, damps `+0x3BC`
  with the `0x800CD360`/`0x800CD370` factors under the `+0x3C8`/`+0x3CC`
  counters, and, on the speed branch, writes the two SDA pose words plus the
  `r2 - 0x598C` word into the three output records.
- **`0x801275B0`, `0x80127600`, `0x80127990` - the `+0x1564` pose ladders.**
  The first stores the `0x80127470` speed with the pose-2 gain. The second
  resets `+0x14A4` and runs the `0x10`/`0x2B`/`0xD`/`0xC`/`0xF` flag ladder
  with ids `5`/`8`/`0xB` and the fixed fallbacks `0x79`/`0x47`/`0x43`/
  `0x76`/`0x75`. The third gates on `+0xEC < 0` and flags `0xC`/`0x10`/  `0xD`/`0xF`, maps `+0x1564` through `0x802F4E08` into the `0x80370DCC`
  vector table and ends in `0x80127B30`.
- **`0x8012C640`/`0x8012DFB0` - the ride pose and point builders.** The big
  one runs the `0x8012C5E0` gate, the `+0x484` state machine and the
  `0x80057650` speed classes, and posts the controller `+0x5C`/`+0x8C` poses
  plus sound `0xE6`. The second averages two animation points read through
  the `0x80056E30` slot `+0x50`, transforms them with `0x801C0BD0`,
  `0x801C0540` and `0x8016F130`, wraps X with `0x80101AA0`, and lifts Y when
  controller flag `4` is set.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80120310` | `fn_80120310` | 452 | medium |
| `0x801252D0` | `fn_801252d0` | 376 | medium |
| `0x801260E0` | `fn_801260e0` | 4 | high |
| `0x801260F0` | `fn_801260f0` | 544 | medium |
| `0x80126310` | `fn_80126310` | 380 | medium |
| `0x801275B0` | `fn_801275b0` | 68 | medium |
| `0x80127600` | `fn_80127600` | 724 | medium |
| `0x80127990` | `fn_80127990` | 412 | medium |
| `0x8012C640` | `fn_8012c640` | 824 | medium |
| `0x8012DFB0` | `fn_8012dfb0` | 356 | medium |

The ten sizes sum to 4,140 bytes.

## Deliberate skips

**`0x80121280` (264 bytes).** The body eases `+0xEC` with `0x8015F140`, but the
export prints only four operands (target, rate, limit, value pointer); the
fourth `f64` (the deadband) is missing from the call and no neighbouring value
proves it. The rest of the body is literal, but reproducing the call would mean
inventing that constant.

**`0x80125D20` (112 bytes) and `0x80125FA0` (76 bytes).** Both are declared
`void`/`undefined4` with **no parameters**, yet each calls `0x80125F10`, whose
only argument is the actor whose `+0x392` slot byte selects the player record.
There is no value in either export that can supply that actor, so the call
cannot be reconstructed without inventing a receiver.

**`0x8012DE80` (296 bytes).** The export places the `0xB` animation buffer at
`local_14` and the `0xE` buffer at `local_20`; because the second 20-byte write
covers `local_14`/`local_10`, the two records overlap in the Ghidra frame and
`local_10` would be read from the wrong record. The `0x80101AA0` call also
prints no operand. Reproducing the overlap would mean guessing which stack
slot each component comes from, so the body is left out rather than invented.

## Notes for the coordinator

- `0x80127600` forwards its otherwise-unused second parameter to
  `0x80145E90`; that call is printed without arguments in the export. The
  receiver is the only object in scope, so the body calls
  `wm_set_sound_by_mode` with it. This is recorded as `medium`, not `high`.
- `0x801260F0` recovers its `bounds` parameter from the saved `r4` that the
  `0x802DCD70` prologue helper exposes to Ghidra as `extraout_r4`; the helper
  itself is a register save and has no semantics, so it is omitted from the
  Rust body.
- `0x80126310` recovers its actor and pose pointers the same way from
  `0x802DCD74` (`r3:r4`). The two `0x800CD360`/`0x800CD370` factors return
  `f64`, so they are called through `w102_call_f64_0`, not the integer
  `call0`.
- The four skipped addresses remain untranslated entries of the
  `0x80120000` bucket and are not claimed by this CSV.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs` (982 public bodies in the file after this pass).
- No address was already in `ledger/functions.csv`; all ten were untranslated
  entries of the `0x80120000` bucket at the start of the pass.
- `cargo check --message-format=short` reports no error for
  `src/game/worldmap.rs`.
