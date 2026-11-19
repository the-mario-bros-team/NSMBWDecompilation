# Coverage - Wave 54 (`d_enemiesNP`, enemies lane)

## How to read this file

This page records one lane of Wave 54: the `0x80A10000` bucket of REL module 3,
`d_enemiesNP`, promoted to `src/game/enemies.rs`. Every row below corresponds to
a real `pub unsafe fn` body in that file; the `evidence` column of the lane CSV
names the Ghidra C export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these are runtime addresses.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 4,728 | 12 | 18 |

## What this pass covers

The pass is one contiguous run, `0x80A135A0..0x80A15490` (30 functions), which
breaks into three blocks:

- **shared five-entry skeleton, `0x80A135A0..0x80A13CF0`.** The
  registration/normaliser pair (`0x80A135A0`, `0x80A13600`), the constant
  producer returning `0x80357BF4`, the `+0x30` chain walker, the `:`
  -suffix name predicate and the two 48-byte deleting-destructor forwarders,
  plus a third normaliser copy at `0x80A14580` and a second forwarder pair at
  `0x80A146A0`/`0x80A146D0`. This is the same body shape as the wave-46 and
  wave-53 blocks, and the doc blocks cross-reference it instead of repeating
  the reasoning.
- **unnamed `g3d/met.brres` actor, `0x80A13950..0x80A14140`.** A `0x798`-byte
  constructor (vtable `0x80B35C80`, holders at `+0x524`, `+0x544`, `+0x584`,
  `+0x5BC`, `+0x618`, `+0x674`, `+0x72C`), the `+0x7C` matrix rebuild from
  position `+0xAC` and the three 16-bit angles `+0x100..+0x104`, the
  `g3d/met.brres` binder, and two frame tails: the flag-bit-2 state switch to
  `0x80B6F218` and the `0x80097B30` wall test that reflects `+0x790`/`+0xE8`
  and switches to `0x80B6F1D8`.
- **`Wm_en_iron` world-map actor, `0x80A14700..0x80A15490`.** A second
  `0x798`-byte constructor (vtable `0x80B36040`, effect helper `+0x65C`), the
  spawn gate `0x80A14A60`, the spawn initialiser `0x80A147C0`, the
  `g3d/iron_ball.brres` binder (`iron_ball`/`iron_ballBig`), the facing
  selector, the `0x80064A30` contact response, the `+0x544` step, the volume
  helper, the 572-byte main per-frame handler, the four-byte empty hook, the
  `Wm_en_iron` effect spawn, the landing-smoke selector, the turn and wobble
  helpers and the phase/clip selector.

The class labels are `// inferred:` because the export names no class in this
bucket: the surviving string evidence is the model archives `g3d/met.brres` /
`g3d/iron_ball.brres` and the effect names `Wm_en_iron`,
`Wm_en_sndlandsmk_s` and `Wm_en_landsmoke_s`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A135A0` | `fn_80a135a0` | 88 | high |
| `0x80A13600` | `fn_80a13600` | 92 | high |
| `0x80A13660` | `fn_80a13660` | 12 | high |
| `0x80A13750` | `fn_80a13750` | 224 | medium |
| `0x80A13830` | `fn_80a13830` | 136 | medium |
| `0x80A138F0` | `fn_80a138f0` | 48 | medium |
| `0x80A13920` | `fn_80a13920` | 48 | medium |
| `0x80A13950` | `fn_80a13950` | 172 | high |
| `0x80A13CF0` | `fn_80a13cf0` | 268 | medium |
| `0x80A13E60` | `fn_80a13e60` | 296 | medium |
| `0x80A14090` | `fn_80a14090` | 116 | medium |
| `0x80A14140` | `fn_80a14140` | 208 | medium |
| `0x80A14580` | `fn_80a14580` | 88 | high |
| `0x80A146A0` | `fn_80a146a0` | 48 | medium |
| `0x80A146D0` | `fn_80a146d0` | 48 | medium |
| `0x80A14700` | `fn_80a14700` | 192 | high |
| `0x80A147C0` | `fn_80a147c0` | 380 | medium |
| `0x80A14940` | `fn_80a14940` | 172 | high |
| `0x80A149F0` | `fn_80a149f0` | 100 | high |
| `0x80A14A60` | `fn_80a14a60` | 208 | medium |
| `0x80A14B30` | `fn_80a14b30` | 260 | medium |
| `0x80A14C90` | `fn_80a14c90` | 48 | medium |
| `0x80A14DE0` | `fn_80a14de0` | 80 | high |
| `0x80A14E80` | `fn_80a14e80` | 572 | medium |
| `0x80A151E0` | `fn_80a151e0` | 4 | high |
| `0x80A151F0` | `fn_80a151f0` | 108 | high |
| `0x80A15260` | `fn_80a15260` | 164 | medium |
| `0x80A15310` | `fn_80a15310` | 208 | high |
| `0x80A153E0` | `fn_80a153e0` | 164 | medium |
| `0x80A15490` | `fn_80a15490` | 176 | medium |

## Deliberate skips

- **No function was skipped inside the run.** Every address from `0x80A135A0`
  through `0x80A15490` has a body in `src/game/enemies.rs`.
- **The rest of the `0x80A10000` bucket.** 148 functions remain outside this
  run. The next contiguous candidates are `0x80A15870` onward and the
  `0x80A16D90..0x80A17230` block; they were left untouched to keep this lane
  contiguous and reviewable.
- **Hidden `r12` this-adjust operands.** The four deleting-destructor
  forwarders are `translated` but their `0x802DCBFC(param_2, param_2, param_1)`
  call cannot model the hidden register; this is stated in each doc block and
  is the same limitation recorded in waves 47 and 53.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 30 sizes sum to
  4,728 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error or warning for
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave54` prints `OK` and reports 30 rows for
  this lane.
