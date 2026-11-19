# Coverage - Wave 79 (`game::bases`, `0x8071` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the
`d_basesNP` REL. Every address is a **static analysis address** under the
canonical synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a
runtime address: the Wii REL loader chooses the real load address, so the
numbers only identify positions inside the REL image. Cross-module pointers
(`0x8037xxxx`, `0x8042xxxx`) are `main.dol` addresses imported by the REL and
keep their absolute values.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`. In the table below,
"role" is the plain-English behaviour, and confidence is `high` when guards,
stores, constants, strings and call arguments are literal, `medium` when the
role of a field or callee had to be inferred (those bodies carry `// observed:`,
`// inferred:` or `// unknown:` notes), and `low` when only the instruction
sequence is certain.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 4272 | 20 | 10 |

No body in this pass is `low` confidence.

## What this pass covers

The `0x8071` bucket held 159 functions at lane start; 12 were already in
`ledger/functions.csv` (the banners `0x80711E90`, `0x80712B90`, `0x807146F0`,
`0x80715930`, `0x80718E80`, `0x8071A400`, `0x8071AFF0`, `0x8071C0D0`,
`0x8071CC00`, `0x8071EC20`, `0x8071EDB0` and `0x8071FC40`), leaving 147. This
pass takes two contiguous runs around two already-recorded model loaders.

- **Ludwig lift member tail, `0x80714150..0x807151F0`** — the twenty-two bodies
  of the `0x890`-byte actor whose `g3d/lift_boss_ludwig.brres` /
  `magic_lift_boss_ludwig` loader is the already translated [`fn_807146f0`]:
  - the two generic release helpers [`fn_80714150`] and [`fn_80715690`]
    (`0x8015F800` + optional `0x802B9280`), the `+0x30` child descent
    [`fn_80714290`] and the two `0x802DCBFC` forwarders
    [`fn_80714430`] / [`fn_80714460`];
  - the allocating constructor [`fn_80714490`] (0x890 bytes) and the
    constructor [`fn_807144c0`] (vtable `0x808071E0`, model `+0x544`, animation
    blocks `+0x560`/`+0x584`, four `0x802DC9A0` controller arrays at `+0x600`
    and `+0x6D4`, inline object chains at `+0x5E4` and `+0x6B8`, tail object at
    `+0x760`), plus the constant-product getter [`fn_80714640`];
  - the curve group: the ping-pong target [`fn_80714c90`] (`+0x6A4`/`+0x6A8`
    bounds over the `+0x74C` curve), the driver [`fn_80714d40`] that shrinks and
    restores the `+0x6AC`/`+0x740` rates from `0x807CCC74`, the latch pair
    [`fn_80714ce0`] / [`fn_80714cf0`] and the field reset [`fn_80714d00`];
  - the presentation bodies: the timer/virtual step [`fn_80714a60`], the matrix
    writer [`fn_80714b20`], the animation-frame to angle conversion
    [`fn_80714eb0`], the track mixer [`fn_80714fd0`], the `Wm_lu_liftspark`
    spawn [`fn_80714f40`], the three per-frame steps
    [`fn_80715070`] / [`fn_807150f0`] / [`fn_80715170`] and the destructor
    [`fn_807151f0`].

- **Morton pillar start, `0x80715810..0x807160B0`** — eight bodies of the
  `0x7C8`-byte actor whose `boss_morton_ashiba` / `magic_boss_morton_ashiba`
  loader is the already translated [`fn_80715930`]: the allocating constructor
  [`fn_80715810`], the constructor [`fn_80715840`] (vtable `0x80807650`,
  particle block `+0x550`, magic holder `+0x634`/`+0x638`, animation `+0x644`,
  inline object `+0x69C`, pillar slots `+0x668`/`+0x66C`, timer `+0x7C4`), the
  landing handler [`fn_80715a60`], the per-frame state [`fn_80715c90`], the
  matrix writer [`fn_80715da0`], the `Wm_mo_pillarhit` effect
  [`fn_80715ed0`], the two-stage pillar arming [`fn_80715f20`] and its
  sub-effect variant [`fn_807160b0`].

Recurring layout facts used by this pass: `+0x60` is the class vtable,
`+0x7C` the NW4R transform matrix, `+0xAC`/`+0xB0`/`+0xB4` the position,
`+0xDC` the scale, `+0x100`/`+0x102`/`+0x104` the rotation angles, `+0x394` an
inline interface object, `+0x524` the resource accessor and `+0x544` the model
holder. Transform writers always run `0x801C0BD0` (build matrix),
`0x8016EEF0`/`0x8016EF50` (apply angles), `0x801C0540` (multiply) and
`0x8016A170`/`0x8016A150` (bind matrix, bind scale).

## Functions

| address | function | bytes | confidence | role |
|---|---|---:|---|---|
| `0x80714150` | `fn_80714150` | 92 | high | generic state release with optional free |
| `0x80714290` | `fn_80714290` | 224 | high | five-step `+0x30` child descent |
| `0x80714430` | `fn_80714430` | 48 | medium | forwarder through `0x802DCBFC` |
| `0x80714460` | `fn_80714460` | 48 | medium | second `0x802DCBFC` forwarder |
| `0x80714490` | `fn_80714490` | 48 | high | 0x890-byte allocating constructor |
| `0x807144C0` | `fn_807144c0` | 384 | high | Ludwig lift constructor |
| `0x80714640` | `fn_80714640` | 28 | high | constant float product |
| `0x80714A60` | `fn_80714a60` | 100 | high | timer and virtual step |
| `0x80714B20` | `fn_80714b20` | 156 | high | matrix writer |
| `0x80714C90` | `fn_80714c90` | 76 | medium | ping-pong curve target |
| `0x80714CE0` | `fn_80714ce0` | 12 | high | set latch `+0x754` |
| `0x80714CF0` | `fn_80714cf0` | 12 | high | clear latch `+0x754` |
| `0x80714D00` | `fn_80714d00` | 52 | high | curve driver reset |
| `0x80714D40` | `fn_80714d40` | 356 | medium | curve driver with rate shrink/restore |
| `0x80714EB0` | `fn_80714eb0` | 144 | medium | animation frame to `+0x104` angle |
| `0x80714F40` | `fn_80714f40` | 96 | high | `Wm_lu_liftspark` spawn |
| `0x80714FD0` | `fn_80714fd0` | 144 | medium | two-track mixer |
| `0x80715070` | `fn_80715070` | 92 | high | per-frame model and angle step |
| `0x807150F0` | `fn_807150f0` | 84 | high | per-frame step plus spark |
| `0x80715170` | `fn_80715170` | 128 | high | per-frame step with rate decay |
| `0x807151F0` | `fn_807151f0` | 348 | high | Ludwig lift destructor |
| `0x80715690` | `fn_80715690` | 88 | high | generic state release with optional free |
| `0x80715810` | `fn_80715810` | 48 | high | 0x7C8-byte allocating constructor |
| `0x80715840` | `fn_80715840` | 228 | high | Morton pillar constructor |
| `0x80715A60` | `fn_80715a60` | 196 | medium | landing / kind `0xFE` arming branch |
| `0x80715C90` | `fn_80715c90` | 184 | medium | per-frame height compare and flag |
| `0x80715DA0` | `fn_80715da0` | 128 | high | matrix writer with fixed rotation |
| `0x80715ED0` | `fn_80715ed0` | 80 | high | `Wm_mo_pillarhit` effect |
| `0x80715F20` | `fn_80715f20` | 400 | medium | two-stage pillar arming |
| `0x807160B0` | `fn_807160b0` | 248 | medium | pillar sub-effect variant |

## Address range

`0x80714150..0x807160B0`, two runs with deliberate holes:

- Run 1 covers `0x80714150..0x807151F0`. Two addresses inside it are not part of
  this pass: `0x807146F0` (312 bytes) was already translated by an earlier pass,
  and `0x80714370` (136 bytes) is left for a later pass, so the run is not
  byte-contiguous even though it is address-contiguous.
- Run 2 covers `0x80715810..0x807160B0`; `0x80715930` inside it was already
  translated.
- Everything below `0x80714150` (`0x80710000..0x80714150`) and everything above
  `0x807160B0` (`0x80716240..0x8071FC40`) is untouched.

## Deliberate skips

- `0x80714370` (136 bytes) — the `:`-suffix name compare. It calls
  `0x802E1C40`/`0x802E1AB4` with a virtual `+0x20` name getter; the body is
  reconstructable but was left out to keep this lane at 30 bodies and to avoid
  adding unverified string-helper prototypes.
- `0x80714EB0` is included, but two of its inputs are only known as the global
  doubles `0x8042DCDC` and `0x807CCC78`; the signed-int-to-double bit idiom is
  reproduced as plain casts and the body is marked `medium`. The paired-single
  `GQR0` prologue/epilogue that Ghidra prints in [`fn_80714d40`] likewise has no
  Rust equivalent and is documented in that body rather than modelled.
- `0x80715E20` (124 bytes) — newly rejected. Its
  `CONCAT31(local_14._0_3_, *(byte *)(param_1 + 0x664))` cannot be resolved:
  the export does not say which byte of the four-byte pattern is replaced, and
  the surrounding `0x8000D5B0` / `0x80168880` calls are only known by address.
  No body was written rather than guess a byte lane.
- `0x80716240` and `0x807162E0` (140 and 80 bytes, the pillar model-bind and
  animation steps) are **not** rejected; they were left out to keep the lane at
  30 bodies.
- The rest of the bucket is untouched. Nothing else in this range was judged
  unreconstructable.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 4,272 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the 12 pre-existing rows of
  this bucket were listed before writing and none is in this pass.
- `node tools/check-lane-csv.mjs wave79` reports
  `wave79-bases.csv: 30 rows` and `OK`.
- `cargo check --message-format=short` produces no diagnostic for
  `src/game/bases.rs`.
