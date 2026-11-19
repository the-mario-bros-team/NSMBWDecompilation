# Coverage - Wave 90 (enemies: the `killer_mag` block of the `0x80AA0000` bucket)

## How to read this file

This page records the Wave 90 `game::enemies` lane: 30 functions of the
`0x80AA0000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs`. Every row in the tables below is one real
`pub unsafe fn fn_<addr>` body in that file, and the `evidence` column of the
machine-readable mirror (`ledger/agents/wave90-enemies.csv`) names the Ghidra C
export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Constants
are cited by address and read through `platform::read_u32` / `read_f32` /
`read_u16` / `read_u8`, never transcribed as invented values. The values quoted
in comments were verified against `work/rels/elf/d_enemiesNP.elf` and are
listed in the section comment that opens the new code block.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 8,460 | 20 | 10 |

The bucket holds 188 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 77 were already in
`ledger/functions.csv` before this pass, 30 are claimed here and 81 remain.

## What this pass covers

One contiguous run, `0x80AA62A0..0x80AA9350`, with the single 592-byte gap
listed under "Deliberate skips". The whole run belongs to one `0xA20`-byte
actor class.

**Identity.** The four constructors install class vtable `0x80B53A80`, and
virtual slot `+0x08` of that vtable is `0x80AA67A0` itself. The binder at
`0x80AA86C0` loads `killer_mag` / `g3d/killer_mag.brres` (variant 0) and
`killer` / `g3d/killer.brres` (variant 1) from the tables at `0x80B53998`,
`0x80B539D0` and `0x80B539D8`, and binds the `search` clip. The spawn
initialiser selects the records `0x80B74C10`, `0x80B74C50`, `0x80B74C90` and
`0x80B74CD0`, which `register_killer_states` names
`daEnKiller_c::StateID_MagMove`, `Move`, `SearchMove` and `SearchMagM`. The
class therefore shares that state table but is not the `0xE48`-byte
`alloc_killer_actor` class (different size, different class vtable).

**Cluster 1, `0x80AA62A0..0x80AA6FC0`, construction and transform.**

- `0x80AA62A0`, `0x80AA63E0`, `0x80AA6520` and `0x80AA6660` are four
  byte-identical constructors (verified by file diff): allocate `0xA20`,
  base-construct, install the class vtable, the model-holder context at
  `+0x524`, the model record at `+0x544`, three animation holders at
  `+0x584`/`+0x5C0`/`+0x5EC`, two `0x802D7AA0` helpers at `+0x618`/`+0x740`,
  the animator at `+0x868` and the render block at `+0x8C4`.
- `0x80AA67A0` is the spawn initialiser: it decodes the spawn kind word at
  `+0x8`, arms the `+0x484` collision block, decodes the spawn word `+0x4`
  into the variant nibble and the size/loop fields, selects one of the two
  model variants, runs [`fn_80aa8950`] and switches to one of the four state
  records.
- `0x80AA6D90` is the frame step (three countdowns, the `+0x394` virtual step,
  the mode word and the `+0x9CC`/`+0x9D0` state shuffle).
- `0x80AA6FC0` commits the transform: `0x80064AF0` in place, `0x801C0BD0`
  matrix build, the three rotation writes, the `+0x544` model commit and the
  `+0x994..+0x99C` translation cache.

**Cluster 2, `0x80AA7570..0x80AA82F0`, contact and attack records.**

- `0x80AA7570` is the contact report selected by the `+0x45D` flag: detach
  through `0x80096700`, or a six-argument damage report through `0x800E20A0`
  carrying the player's character type.
- `0x80AA7680` writes the x/y meeting point of two actors and the two `+0x9D8`
  mode words.
- `0x80AA7B90` and `0x80AA7C50` arm the `+0x414` attack record from the back
  and front side respectively (the `0x80B2F730`/`0x80B2F750` five-word presets,
  lateral/forward speeds, side byte and kind byte).
- `0x80AA8170` and `0x80AA82F0` rebuild the same record from a live player
  through virtual slots `+0x164`/`+0x158` and `+0x20C`, plus the
  `0x8009F690`/`0x8009F710` action lookup.

**Cluster 3, `0x80AA86C0..0x80AA8D30`, model binder and the spin/height
machine.**

- `0x80AA86C0` is the model and clip binder described above.
- `0x80AA8950` initialises the `+0x868` wall-foot splash record from the
  per-side row at `0x80B2F7F8`.
- `0x80AA89F0`, `0x80AA8A20` and `0x80AA8AA0` are the roll-offset ramp, its
  limit step and its application to the roll angle.
- `0x80AA8B00`, `0x80AA8C10` and `0x80AA8C30` are the spin state machine: the
  three modes, their `0xFFFF`/`0x10000`/`0x20000` limits and the two arm
  entries.
- `0x80AA8C60` and `0x80AA8D30` are the two frame steps that gate the spin,
  count `+0xA00` down and write the `2564.0`/`1564.0` low-height values.

**Cluster 4, `0x80AA8DF0..0x80AA9350`, effects and helpers.**

- `0x80AA8DF0` resolves the model node and raises
  `Wm_en_magkillersmoke` / `Wm_en_searchkiller`, scaling the point by `0.1`
  (or `0.06` after eight hits) for the `killer` variant.
- `0x80AA8FE0` probes both wall-foot sides (`+-21.0` or `+-8.0`) and raises
  one of the four `Wm_en_wfsplash_*` effects plus sound `0x285` when the
  contact side changes.
- `0x80AA9250`, `0x80AA92B0` and `0x80AA92E0` are the ground probe, the
  lateral-speed select and the pending-turn step.
- `0x80AA9350` computes the aim angle with `0x801611C0`, biases it by
  `+-0x4000` and stores it through `0x8015F4F0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AA62A0` | `fn_80aa62a0` | 320 | high |
| `0x80AA63E0` | `fn_80aa63e0` | 320 | high |
| `0x80AA6520` | `fn_80aa6520` | 320 | high |
| `0x80AA6660` | `fn_80aa6660` | 320 | high |
| `0x80AA67A0` | `fn_80aa67a0` | 1520 | medium |
| `0x80AA6D90` | `fn_80aa6d90` | 156 | high |
| `0x80AA6FC0` | `fn_80aa6fc0` | 260 | high |
| `0x80AA7570` | `fn_80aa7570` | 136 | medium |
| `0x80AA7680` | `fn_80aa7680` | 260 | medium |
| `0x80AA7B90` | `fn_80aa7b90` | 180 | high |
| `0x80AA7C50` | `fn_80aa7c50` | 224 | high |
| `0x80AA7D30` | `fn_80aa7d30` | 404 | medium |
| `0x80AA8170` | `fn_80aa8170` | 380 | medium |
| `0x80AA82F0` | `fn_80aa82f0` | 380 | medium |
| `0x80AA86C0` | `fn_80aa86c0` | 644 | high |
| `0x80AA8950` | `fn_80aa8950` | 156 | high |
| `0x80AA89F0` | `fn_80aa89f0` | 48 | high |
| `0x80AA8A20` | `fn_80aa8a20` | 120 | high |
| `0x80AA8AA0` | `fn_80aa8aa0` | 84 | high |
| `0x80AA8B00` | `fn_80aa8b00` | 268 | high |
| `0x80AA8C10` | `fn_80aa8c10` | 32 | high |
| `0x80AA8C30` | `fn_80aa8c30` | 44 | high |
| `0x80AA8C60` | `fn_80aa8c60` | 196 | high |
| `0x80AA8D30` | `fn_80aa8d30` | 180 | high |
| `0x80AA8DF0` | `fn_80aa8df0` | 484 | medium |
| `0x80AA8FE0` | `fn_80aa8fe0` | 620 | medium |
| `0x80AA9250` | `fn_80aa9250` | 88 | high |
| `0x80AA92B0` | `fn_80aa92b0` | 48 | high |
| `0x80AA92E0` | `fn_80aa92e0` | 112 | medium |
| `0x80AA9350` | `fn_80aa9350` | 156 | medium |

## Deliberate skips

- **`0x80AA8470` (592 bytes) is not translated.** It is the per-player contact
  sweep that sits between `0x80AA82F0` and `0x80AA86C0`. Its export opens with
  an `unaff_GQR0`/`ldexpf` paired-single block (three warnings about removed
  unreachable blocks) and the block feeds the two `double` arguments of the
  class-virtual call at slot `+0x3FC`; the real operands are not recoverable
  from the C. Its three contact callees ([`fn_80aa8170`],
  [`fn_80aa82f0`], [`fn_80aa7c50`]) are translated, so only the dispatch loop
  is missing. The gap `0x80AA846C..0x80AA8470` before it and
  `0x80AA86C0` after it are accounted for by this omission.
- **`0x80AA9630` is not translated.** The spawn initialiser calls it to decide
  whether the `killer` variant may start; it lies outside the selected run and
  is therefore called by address through [`W90_FN_80AA9630`]. The constant
  carries a `// not translated` note.
- **`0x80AA6E30` is not translated.** It is the frame slot `+0x2C` of the class
  vtable (`0x80B53A80 + 0x2C`) and starts immediately after `0x80AA6D90`, but
  it is not in the candidate list of this pass. `fn_80aa67a0` invokes it
  through the vtable, so the behaviour is still reachable at runtime.
- **Small gaps inside the run have no `functions.tsv` entries.**
  `0x80AA6E2C..0x80AA6FC0`, `0x80AA70C4..0x80AA7570`,
  `0x80AA75FC..0x80AA7680`, `0x80AA7784..0x80AA7B90`,
  `0x80AA7EC4..0x80AA8170` and `0x80AA93EC..0x80AA93F0` were never identified
  by Ghidra in this bucket, so no body could be written for them.
- **Ten bodies are `medium`, not `high`.** `fn_80aa67a0` (several field roles
  inferred from their consumers), `fn_80aa7570` (the export prints no operands
  for `0x80096700`), `fn_80aa7680` (the role of `+0x9D8`), `fn_80aa7d30` (the
  `+0x90`/`+0x94` vector and the `0x8042C1B0` table), `fn_80aa8170` /
  `fn_80aa82f0` (the `+0x20C` argument vector and the record tails),
  `fn_80aa8df0` (the scale selected for the `killer` variant),
  `fn_80aa8fe0` (the GQR prologue is dead and the returned side flag is
  inferred), `fn_80aa92e0` (the semantics of `0x80097B30`) and `fn_80aa9350`
  (the `context` argument). In every case the instruction sequence, the calls
  and the stores are literal.
- **The remaining 81 functions of the bucket stay untranslated.** After this
  pass the largest untouched runs are `0x80AA0080..0x80AA0530` (the head of
  the bucket, cut by the already-ledgered wave-34 salvage),
  `0x80AA0C20..0x80AA14E0` and `0x80AA93F0..0x80AAFE00`.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 30 sizes sum to
  8,460 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 30 was
  already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave90` reports 30 rows for this CSV and
  `OK`.
- `cargo check --message-format=short` finishes with no diagnostic naming
  `src/game/enemies.rs`.
