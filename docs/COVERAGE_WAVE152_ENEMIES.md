# Coverage - Wave 152 (`game::enemies`)

## How to read this file

This page records the Wave 152 `game::enemies` lane: 18 functions of REL module
3, `d_enemiesNP`, appended to `src/game/enemies.rs` as section 134 and mirrored
one row per body in `ledger/agents/wave152-enemies.csv`.

Every `address` is a **synthetic analysis address** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none of
these is a runtime address; every function doc repeats that note. Raw words were
decoded from `work/rels/elf/d_enemiesNP.elf` through the `PT_LOAD` headers as
described in `docs/REL_ELF_MAPPING.md`
(`file_offset = address - 0x80A00000 + 0x1000` for the first segment), and every
body was followed from its entry to its terminal instruction. Byte counts are the
`size` column of `work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 18 | 5416 | 9 | 9 |

Status split: 18 `translated` (9 high, 9 medium), no `partial` and no `stub`.

Covered address range: `0x80A707E0..0x80AF2400` (the last body's `blr` sits at
`0x80AF251C`). 4,828 bytes are `docs/SPEC_WAVE151_ENEMIES.md` rows and 588 bytes
are `0x80AA0000` planner-bucket rows.

## What this pass covers

- **The eight "ready" rows of `docs/SPEC_WAVE151_ENEMIES.md`:**
  - `0x80A8BD20` - `kurage` constructor: `g3d/kurage.brres`, the three light
    model holders (`0x324`/`0x224`/`0x224`), two srt blend blocks and one colour
    block, then `+0x190 = 0xFF`, `+0x194 = 1`.
  - `0x80AAD490` - shot/slide launch: `0x80AACD50` band test, free slot from
    `0x80AACE90`, the height row `0x80B2F8E8`, the packed `variant | slot << 8`
    word, sound `0x1BD` and the `+0xE34` latch.
  - `0x80AC0840` - `lift_maruta` per-slot collision response, including the
    single `qr3` halfword widen and the `32.0 * 0x80237C40` height ramp.
  - `0x80A7A8C0` - two-row upward ground probe returning the hit-row count.
  - `0x80ADE060` - impact effect record at `(x, y - 8, z + 16)` with the
    `0x80B30AF0` id table and the single/multi-player split.
  - `0x80A7A460` - the `2 x {4, 2, 2}` segment-position grid over `0xA4`-byte
    records.
  - `0x80A7AA00` - downward probe fallback returning `0`/`-1`/`-2`.
  - `0x80AEAED0` - signed offset fan: one trail node per table entry.
- **Three of the four "conditional" rows** (the fourth, `0x80A10880`, is
  deliberately skipped; see below):
  - `0x80A707E0` - the `0x278`-byte chain-segment builder. The specification's
    open item ("which `u16` does `r5` carry") turned out not to exist; see the
    corrections below.
  - `0x80AF2400` - `Wm_en_burst_m` cannon placement; the only open item was the
    four-byte-early TSV extent, and the trailing `blr` at `0x80AF251C` belongs to
    the body.
  - `0x80A969C0` - carrier contact test; both arms are literal apart from two
    `+0x6C` vtable dispatches whose class is unnamed.
- **All seven remaining bodies of the `0x80AA0000` planner bucket** (the eighth,
  `0x80AAD490`, is the specification row above):
  - `0x80AA1030` - two base per-frame updates plus `0x80AA0080` at scale `1.0`.
  - `0x80AA3A90` - slot-position writer over the `+0x914` handle array.
  - `0x80AAE360`, `0x80AAE390`, `0x80AAFCB0`, `0x80AAFCE0` - four 48-byte
    `0x802DCBFC` deleting-destructor forwarders.
  - `0x80AAF2B0` - the `(a^2 - b^2) * sqrt(a^2 - b^2)` helper.

### `qr3` sites

| address | site | port |
|---|---|---|
| `0x80AC0840` | `0x80AC0940` `psq_l f1,0(r4),3,1` | the stored `u16` phase is widened as `(f32)(u16)` and scaled by `0x80B30198 = 0.00390625` before `0x80237C40` |

`GQR3` is `0x00050005` per `docs/REL_ELF_MAPPING.md` (type 5 = unsigned 16-bit,
scale 0, `W = 1`), so the load is exactly `(f32)(u16)phase`. Every other
`psq_l`/`psq_st` in the 18 rows is a `qr0` `f31`/`f30`/`f29` stack spill.

### Specification corrections made while porting

- **`0x80A707E0` has no `u16` argument.** The page's first two statements
  (`*(u16*)(0x80B2DAE4 + side*2) = value;` read back and halved) are not in the
  stream. The raw body at `0x80A7082C` is an `lhax r5,r4,r0` read of the
  existing global, and there is no store anywhere in the 652 bytes. The
  signature is therefore `fn_80a707e0(actor: usize)`; the incoming `r5` is dead.
- **`0x80A707E0` stores the slot index at `+0x270`.** The page says
  `obj->0x270 = 0`; the raw body at `0x80A70A14` is `stw r28,624(r30)` where
  `r28` is the `0..+0x540` loop index. The port writes the index.
- **`0x80A969C0`'s adopt arm has no `+/- 2.0` widen.** The export's second arm
  builds `sa - v[2]` / `sa + v[2]`, while the first arm (the player's own
  segment) does subtract and add `0x80B2ED44`. The port follows the stream.
- **`0x80A969C0`'s counter arm order.** A hit with a live rider clears `+0x634`;
  otherwise the counter increments up to `0x3D` before the leave notification,
  exactly as the export shows (`if (ride == 0 || hit == 0)`).
- **`0x80AEAED0` emits `2 * (+0x814 + 1)` nodes, and `0.5` is live.** The page's
  closing formula (`2 * n + 1`) is one short: with `+0x814 == 0` the stream emits
  ids `0` and `-1`. The `0.5` load is also used by the `+0x820 == 0` y leg, so
  it is not a dead load.
- **`0x80AA3A90`'s bound is sign-extended.** `+0xA53` is read with `lbz` but
  immediately passed through `extsb` before `(count + 1) * 4`; the port keeps
  the signed form, so a byte above `0x7F` makes the loop empty.
- **`0x80ADE060` call shape.** The export prints three arguments to
  `0x80022810`/`0x80022B10`; the specification's five-argument shape (with the
  id in `r6` and the flags in `r7`) is used and marked `medium`, because the id
  is also written into the 24-byte record at `+0x10`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A707E0` | `fn_80a707e0` | 652 | high |
| `0x80A7A460` | `fn_80a7a460` | 312 | high |
| `0x80A7A8C0` | `fn_80a7a8c0` | 320 | high |
| `0x80A7AA00` | `fn_80a7aa00` | 292 | high |
| `0x80A8BD20` | `fn_80a8bd20` | 640 | high |
| `0x80A969C0` | `fn_80a969c0` | 844 | medium |
| `0x80AA1030` | `fn_80aa1030` | 72 | high |
| `0x80AA3A90` | `fn_80aa3a90` | 216 | medium |
| `0x80AAD490` | `fn_80aad490` | 484 | high |
| `0x80AAE360` | `fn_80aae360` | 48 | medium |
| `0x80AAE390` | `fn_80aae390` | 48 | medium |
| `0x80AAF2B0` | `fn_80aaf2b0` | 108 | medium |
| `0x80AAFCB0` | `fn_80aafcb0` | 48 | medium |
| `0x80AAFCE0` | `fn_80aafce0` | 48 | medium |
| `0x80AC0840` | `fn_80ac0840` | 396 | medium |
| `0x80ADE060` | `fn_80ade060` | 320 | medium |
| `0x80AEAED0` | `fn_80aeaed0` | 284 | high |
| `0x80AF2400` | `fn_80af2400` | 284 | high |

### Why nine rows are `medium`

- **`0x80AC0840`** - the guards, the `qr3` widen, the `1/256` scale and the
  `32.0` height are literal; the `0x80237C40` unit and the object roles are
  inferred from the specification.
- **`0x80A969C0`** - both arms, the `0x16` flag table, the `0x23C` block and the
  `0x3D` limit are literal; the two `+0x6C` vtable dispatches return an object
  whose first byte is the state id, and that class is not named.
- **`0x80ADE060`** - the record layout, the id table and the `8`/`16` offsets are
  literal; the two `main.dol` callees are outside this REL.
- **`0x80AA3A90`** - the stride, the `16.0` step and the `& 0x1F8` mask are
  literal; the eight-byte table role is inferred from the record stride.
- **the four forwarders** - the visible `0x802DCBFC(mem, mem, size)` call is
  exact, but the hidden `r12` this-adjust operand cannot be modelled in Rust.
- **`0x80AAF2B0`** - all loads, compares and the multiply are literal; the
  identity of `0x80237BA0` as the square-root helper is inferred.

## Deliberate skips

- **`0x80A10880` (940 bytes).** The last untranslated `0x80A10000` bucket body
  and specification rank 1. Wave 151 already skipped it because its `2^52`
  `u32 -> f64` fold divides by `29.0` and offsets by a `2^52` word, and its
  outer loop advances through a link the export does not type. Re-reading the
  advice in `docs/SPEC_WAVE151_ENEMIES.md` did not resolve those three operands,
  so writing a body would be a guess rather than a translation. It remains the
  top candidate for a focused pass.
- **`0x80A965D0` (280 bytes).** The next spec row below the cut
  ("clean per-slot child activation; `pk`-model holder rebuild"). It was not
  claimed here because the lane's quota was met by the eight ready rows, the
  three conditional rows and the seven bucket bodies.
- **`0x80AC1800` (276 bytes)** still carries GQR damage in its export and was not
  attempted; the surrounding comment in section 69 now records that only
  `0x80AC0840` of that pair was resolved.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 18 sizes sum to 5,416
  bytes (4,828 specification rows, 588 bucket rows).
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; all 18 addresses were absent from
  `ledger/functions.csv` and from `src/**/*.rs` before this pass.
- Every `# Original` block carries the `Synthetic base: 0x80A00000` line.
- The single data `qr3` site is ported as `(f32)(u16)`; every other `psq` in
  these rows is a `qr0` spill.
- The module header was updated: cluster item 134, the cluster count (133 to
  134), the function totals (3468 to 3486 public translations, 3397 to 3415
  `fn_<addr>` bodies), the wave list, the "start here" list and the records list.
- `cargo check --message-format=short` reports no error or warning naming
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave152` reports 18 rows for this lane, 18
  unique addresses and `OK`.
