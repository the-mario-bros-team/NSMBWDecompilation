# Coverage - Wave 67 (enemies: `0x80B03780..0x80B03E20`, `0x80B04A30..0x80B04E10`, `0x80B054D0..0x80B06120`)

## How to read this file

This page records the Wave 67 `game::enemies` lane: three runs in the
`0x80B00000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs`. Every row corresponds to one real `pub unsafe fn` body in
that file, and the `evidence` column names the Ghidra C export that body was
reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 26 | 3,604 | 21 | 3 | 2 |

## What this pass covers

The bucket `0x80B00000..0x80B0FFFF` holds 207 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 58 of them were already in
`ledger/functions.csv` when this lane started and 149 were untranslated. No
address is repeated. The three runs below all belong to **one actor**: an enemy
that follows a byte rail with a head segment, a second segment and up to `0x14`
trailing segments.

The actor's shape, as established by the exports in this pass:

| offset | contents |
|---|---|
| `+0x060` | class vtable `0x80B66498` |
| `+0x394` | controller object |
| `+0x524` | actor sub-object / animation context |
| `+0x540` | fixed segment array, `2` elements of `0x1BC` (head + second) |
| `+0x5E4` | head position `x`/`y`/`z` |
| `+0x6E4`, `+0x8A0` | per-segment "holder needs settling" bytes |
| `+0x7A4` | second segment `y` (the height tested against the floor) |
| `+0x8B8` | trailing segment array, `0x14` elements of `0x1B8` |
| `+0x2B18` | live trailing count |
| `+0x2B1C` | rail direction-byte buffer |
| `+0x2B20` | accumulated rail length |
| `+0x2B24` | `u16` scratch counter |
| `+0x2B28` | per-variant state word from table `0x80B66480` |
| `+0x2B30` | growth timer used by the integration step |

Every segment owns `f32` position `+0xA4..+0xAC`, velocity `+0xB0..+0xB8`,
committed rail point `+0xBC/+0xC0`, model holder at `+0xC8`, flag word `+0x198`
(`1` = placed, `2` = follow controller released), rail index `+0x1AC` and
follow target `+0x1B0`.

- **Class skeleton (`0x80B03780..0x80B03A20`).** Two registration/release
  callbacks with the same body ([`fn_80b03780`], [`fn_80b037e0`]), two constant
  producers (`0x80357BF4`, `0x80357BB4`), the unrolled five-link `+0x30` chain
  walker [`fn_80b03940`] and the `:`-suffix name predicate
  [`fn_80b03a20`]. The walker is the same shape as the wave-66
  [`fn_80b1c4f0`], which is what fixes the `+0xC` slot role.
- **Container (`0x80B03AE0..0x80B03E20`).** Two 48-byte argument-swapping
  forwarders to `0x802DCBFC`, the `0x2B40`-byte constructor
  [`fn_80b03b40`] (class vtable `0x80B66498`, two `0x802DC9A0` fixed-capacity
  arrays of `2 x 0x1BC` and `0x14 x 0x1B8`), the head element constructor
  [`fn_80b03bf0`] (vtable `0x80B66718`, holder `+0x48` written
  `0x803295D8` then `0x80329518`), the trailing element constructor
  [`fn_80b03cc0`] (base vtable kept), and both element destructors
  [`fn_80b03d80`] / [`fn_80b03e20`].
- **Segment-wide frame operations (`0x80B04A30..0x80B04E10`).** The render-list
  rebuild [`fn_80b04a30`] (opens `0x8016A960` on `+0x524`, re-binds every
  segment, commits `0x8016A9B0`), the three per-segment loops that release the
  clips, bind the rail model and reset the holders, the spawn-word variant
  select [`fn_80b04df0`] and the follow-controller release [`fn_80b04e10`]
  (which sets bit 0 of the second segment's `+0x198` word, `0x894 = 0x6FC +
  0x198`).
- **Position, timing and integration (`0x80B054D0..0x80B06120`).** The
  chain-trail shift [`fn_80b054d0`] (each trailing segment takes the position
  of the one in front, `z` forced to `0x80B31A9C`), the second-segment position
  copy [`fn_80b055d0`], the below-floor test [`fn_80b05620`], the state/timer
  test [`fn_80b05670`], the ground step [`fn_80b05df0`], and the two per-frame
  integrators [`fn_80b05f00`] (with the `+0x2B30` growth window) and
  [`fn_80b06120`] (without it).

### Identity evidence

`0x80B66498`, `0x80B66718`, `0x80B66724`, `0x803295D8`, `0x80329518`,
`0x80329578` and the table `0x80B66480` are all literal in the exports. The
class name is **not** printed anywhere in this run, so the actor is described by
allocation size and offsets only - no Nintendo symbol is invented for it.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80B03780` | `fn_80b03780` | 88 | high |
| `0x80B037E0` | `fn_80b037e0` | 92 | high |
| `0x80B03840` | `fn_80b03840` | 12 | high |
| `0x80B03850` | `fn_80b03850` | 12 | high |
| `0x80B03940` | `fn_80b03940` | 224 | high |
| `0x80B03A20` | `fn_80b03a20` | 136 | medium |
| `0x80B03AE0` | `fn_80b03ae0` | 48 | low |
| `0x80B03B10` | `fn_80b03b10` | 48 | low |
| `0x80B03B40` | `fn_80b03b40` | 164 | high |
| `0x80B03BF0` | `fn_80b03bf0` | 200 | high |
| `0x80B03CC0` | `fn_80b03cc0` | 188 | high |
| `0x80B03D80` | `fn_80b03d80` | 148 | high |
| `0x80B03E20` | `fn_80b03e20` | 176 | high |
| `0x80B04A30` | `fn_80b04a30` | 96 | medium |
| `0x80B04C90` | `fn_80b04c90` | 112 | high |
| `0x80B04D00` | `fn_80b04d00` | 124 | high |
| `0x80B04D80` | `fn_80b04d80` | 112 | high |
| `0x80B04DF0` | `fn_80b04df0` | 28 | high |
| `0x80B04E10` | `fn_80b04e10` | 124 | high |
| `0x80B054D0` | `fn_80b054d0` | 248 | high |
| `0x80B055D0` | `fn_80b055d0` | 68 | high |
| `0x80B05620` | `fn_80b05620` | 80 | high |
| `0x80B05670` | `fn_80b05670` | 160 | medium |
| `0x80B05DF0` | `fn_80b05df0` | 152 | high |
| `0x80B05F00` | `fn_80b05f00` | 420 | high |
| `0x80B06120` | `fn_80b06120` | 344 | high |

## Deliberate skips

- **`0x80B04A90` (512 bytes).** The export is dominated by GQR/paired-single
  artefacts: the body opens with four copies of the `psq_l` dequantisation
  idiom (`ldexpf`, `CONCAT13`/`CONCAT22` into a local) whose inputs are
  uninitialised `f28..f31`/`ps28..ps31` pseudo-registers. The real payload
  (segment placement through `0x80B04270`) is readable, but the surrounding
  arithmetic cannot be reproduced without inventing the missing inputs.
- **`0x80B04E90` (1,592 bytes).** The rail-byte buffer builder. It walks the
  rail points, converts each delta into a direction code and then fills a
  freshly allocated `length + 2` buffer with run-length codes; Ghidra prints
  the fills as four unrolled `memset` loops with generated sanity branches. The
  intent is clear but the literal body would be an interpretation of those
  loops rather than a translation, so it is left for a pass that can read the
  instruction stream. [`fn_80b04a30`] calls it by address (`0x80B04E90`) and
  says so in its `# Confidence` block.
- **`0x80B05740` (328 bytes), `0x80B05A80` (180 bytes), `0x80B05B80`
  (172 bytes).** All three call `0x80B04120` with a **clip-name argument**
  (`s_create_80b66490` / `s_ridden_80b66478`). The wave-10 translation of
  `fn_80b04120` has a one-argument signature, because its own export printed
  only the receiver; forwarding a body through it would silently drop the clip
  name. Rather than change a previously validated body in this pass, these
  three stay untranslated until `fn_80b04120` is revised.
- **`0x80B058A0` (476 bytes).** Same GQR/paired-single opening as
  `0x80B04A90`; the incoming `f31`/`ps31` pair is dequantised into two locals
  that are never read, and the `-((byte)(gqr >> 0x18) & 0x3f)` epilogue has no
  recoverable operand.
- **The rest of the bucket.** `0x80B03F00..0x80B04880` was already translated
  by wave 10 (`block_snake_ice`), and `0x80B06200..0x80B0FF50` plus the
  `0x80B0F...` tail is the next natural continuation. None of those addresses
  is claimed here.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 26 sizes sum to
  3,604 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice.
- No address was already in `ledger/functions.csv` (58 of the 207 bucket
  addresses were, and all were excluded before the runs were chosen).
- `node tools/check-lane-csv.mjs wave67` reports `OK` with
  `wave67-enemies.csv: 26 rows`.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/game/enemies.rs` (the only crate error in the run names
  `src/game/bases.rs`, another lane's file).
