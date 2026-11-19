# Coverage - Wave 91 (`game::worldmap`, `0x80120000` bucket)

## How to read this file

Each row is one `pub unsafe fn` body appended to `src/game/worldmap.rs`, named
`fn_` plus its original `main.dol` address. Byte sizes come from
`work/ghidra/main-gb/export/functions.tsv`; every evidence path is the Ghidra C
export the body was transcribed from. Field offsets are literal offsets from the
object pointer. `r2` is `0x80433080` (`SDA2_BASE`) and `r13` is `0x8042F680`
(`SDA_BASE`); every `r2`-relative float below was read from `work/dol/main.elf`
at the absolute address named in its constant comment. Values at `0x8037xxxx`
are runtime data addresses and are referenced, never embedded.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 28 | 6724 | 7 | 20 |

One further body is `low` confidence (`fn_80125a90`).

## What this pass covers

- **`0x801202A0`..`0x80120780` - fall-state entry pair.** `0x801202A0` is the
  ground probe that stores `+0x5F4` and clears `+0xE8`/`+0x5F0`/`+0x5F8`;
  `0x80120720` seeds `+0x5C8 = 0.4` and `+0x360 = 1`; `0x80120780` runs the
  two-state machine over `+0x360`, the `0.03` timer clamp on `+0x5CC`, the
  `0x802F4C90` speed table and the `0x80375838` / `0x8011EBB0` exit.
- **`0x80120950` - second fall variant.** The same ladder with the `0.05`
  clamp, the `0.005` increment and the `0x802F4CA0`/`0x802F4C80` tables, ending
  in `0x8011EBB0` only.
- **`0x80120AC0`..`0x80121390` - recovery and slide setup.** `0x80120AC0` arms
  the `0x78` timer and closes layout channel `+0x148`; `0x80120FD0` /
  `0x801211E0` are the two recovery entries (`0.8` / `0.25` impulse scale,
  `-2500` depth, `+0x530 = 0x408` / `8`); `0x80120C70` copies the ground height
  to `+0x5FC` and picks `+0x114` from the `0x80375878` name test; `0x80121390`
  builds the slide offsets `1.0 + 0.5 *` the `+0x5B4`/`+0x5B8` extents.
- **`0x801214A0`/`0x801217C0`/`0x80121B00` - contact and landing ladder.** The
  three-way mode byte at `+0x1E7`/`+0x2D1`/`+0x53D`, the `+0x5C4` player
  resolve, the `0x8011EB00` / `0x8011EE80` / `0x8011EFE0` contact tests, the
  `+0x184` callback store, the `0x8026F`/`0x8026D` sound pair and the
  `0x80082240` scene-rail clamp with the `64.0` margin.
- **`0x801250D0`..`0x80125D90` - the `0x80125xxx` target/record helpers.**
  Pad mode comes from `r13 - 0x56E0 -> +0x20`; `0x801250D0` and `0x80125140`
  dispatch the `+0x460`/`+0x47C`/`+0x4B4`/`+0x4EC`/`+0x4D0` children;
  `0x80125450` caches the resolved player into the `+0x394..+0x3A8` triple and
  runs the `0x80125670`/`0x80125830` builders with the `+0x3AC`/`+0x3B4`/`+0x3B8`
  epsilon tests; `0x80125A30`/`0x80125A90` are the small copy dispatchers;
  `0x80125B80`+`0x80125D90` are the three-probe target scan.
- **`0x80126F40`..`0x801285D0` - player action entries.** The eight-input
  landing chain, the `+0x2530` countdown, the `0x8012E760`/`0x801266B0`
  result pair, the `0x80375C2C`/`0x803761F0`/`0x803761B0` event posts, the
  state-1 model call `(2.0, 0.0, 0.0, model, 10)` and the ten-part
  `0x801284C0` eligibility gate used by `0x801285D0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801202A0` | `fn_801202a0` | 92 | high |
| `0x80120720` | `fn_80120720` | 80 | high |
| `0x80120780` | `fn_80120780` | 400 | medium |
| `0x80120950` | `fn_80120950` | 364 | medium |
| `0x80120AC0` | `fn_80120ac0` | 84 | high |
| `0x80120C70` | `fn_80120c70` | 124 | medium |
| `0x80120FD0` | `fn_80120fd0` | 136 | high |
| `0x801211E0` | `fn_801211e0` | 144 | high |
| `0x80121390` | `fn_80121390` | 168 | high |
| `0x801214A0` | `fn_801214a0` | 604 | medium |
| `0x80121760` | `fn_80121760` | 84 | high |
| `0x801217C0` | `fn_801217c0` | 456 | medium |
| `0x80121B00` | `fn_80121b00` | 584 | medium |
| `0x80121D50` | `fn_80121d50` | 52 | medium |
| `0x801250D0` | `fn_801250d0` | 108 | medium |
| `0x80125140` | `fn_80125140` | 208 | medium |
| `0x80125450` | `fn_80125450` | 536 | medium |
| `0x80125A30` | `fn_80125a30` | 92 | medium |
| `0x80125A90` | `fn_80125a90` | 232 | low |
| `0x80125B80` | `fn_80125b80` | 308 | medium |
| `0x80125D90` | `fn_80125d90` | 380 | medium |
| `0x80126F40` | `fn_80126f40` | 264 | medium |
| `0x80127340` | `fn_80127340` | 196 | medium |
| `0x801278E0` | `fn_801278e0` | 176 | medium |
| `0x80128190` | `fn_80128190` | 144 | medium |
| `0x80128220` | `fn_80128220` | 188 | medium |
| `0x801284C0` | `fn_801284c0` | 268 | medium |
| `0x801285D0` | `fn_801285d0` | 252 | medium |

## Deliberate skips

- **GQR-reconstruction records `0x801200C0`, `0x80120310`, `0x80120520`,
  `0x80120B40`, `0x80120D30` and `0x80121090`.** Each export opens with the
  `unaff_GQR0`/`unaff_GQR3`/`ldexpf` prologue (and usually a removed block), so
  the paired-single scale factors are not recoverable from the C. The same
  policy was used by waves 36, 60, 64, 73 and 90.
- **`0x80121280`.** Its `0x8015F140` call shows only three doubles plus the
  `+0xEC` pointer, while the callee has four doubles and a pointer. The fourth
  double is dropped by the export, so the call would be invented.
- **`0x80121990`.** The export renders the side comparison as
  `(fVar5 < fVar6) && (fVar6 < fVar5)`, which is false for every input, and the
  condition sits next to a removed unreachable block. The real branch cannot be
  recovered from the C.
- **`0x801252D0`.** `extraout_var` captures the `r3` halfword of an unprototyped
  call and stores it at `+0x49A`; the value is not expressed anywhere in the
  export.
- **`0x80125F70` and `0x80125FA0`.** Already documented by wave 77: the first
  renders a degenerate `countLeadingZeros` expression that is `1` for all 256
  inputs, and the second calls a receiver slot with the `this` pointer dropped.
  Both stay raw addresses; `0x80125F70` is called by
  [`fn_80125450`]/[`fn_80125D90`].
- **`0x801260F0`.** Its `extraout_r4` is the live dispenser pointer carried
  across `0x802DCD70`; the export never assigns it, so the six comparisons
  cannot be reconstructed. `0x80125B80` calls the four-byte thunk
  `0x801260E0` by raw address.
- **`0x80125D20`.** The export prints `0x80125F10()` without arguments although
  the helper needs the actor; the `this` pointer would be invented.
- **`0x80126970`.** The model call arguments are `(double)DAT_80371150` and
  `(double)DAT_80371154`; both are `0x8037xxxx` runtime-data addresses, not
  static float values in `main.elf`, so the pose constants would be invented.
- **`0x80125210`, `0x80125240`, `0x80125270`, `0x801252A0`.** The four
  record-copy helpers still have unrecovered jump tables. They are not claimed
  as bodies but are called at their raw addresses by [`fn_80125a90`].
- **The rest of the bucket.** `0x80126A80`, `0x801275B0`, `0x80127600`,
  `0x80127990`, `0x80127B30`, `0x80128EA0` and the remaining multi-branch state
  machines were not attempted in this pass; they stay as raw addresses in the
  bodies above.

## Verification

- Every address and byte size matches `functions.tsv`; the 28 sizes sum to
  6,724 bytes.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv`; all 28 were untranslated
  entries of the `0x80120000` bucket at the start of the pass.
- `cargo check --message-format=short` reports no error or warning that names
  `src/game/worldmap.rs` (the build still stops on other lanes' errors).
- `node tools/check-lane-csv.mjs wave91` reports `OK`; `wave91-worldmap.csv`
  has 28 unique rows and no duplicate address.
