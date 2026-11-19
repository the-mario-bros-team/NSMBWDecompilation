# Coverage - Wave 85 (`game::bases`, `0x8076` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** under the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader chooses the real load address, so the numbers only
identify positions inside the REL image. Cross-module pointers (`0x8037xxxx`,
`0x8042xxxx`, `0x807Cxxxx`) are `main.dol`/REL data addresses imported by the
module and keep their absolute values.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`. "Role" below is the
plain-English behaviour. Confidence is `high` when guards, stores, constants
and call arguments are literal, and `medium` when the role of a field, a
global, a callee or a dropped argument had to be inferred (those bodies carry
`// observed:`/`// inferred:`/`// unknown:` notes). No body in this pass is
`low`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 29 | 6,592 | 18 | 11 |

## What this pass covers

The `0x8076` bucket holds 203 functions; 61 were already in
`ledger/functions.csv` when this pass began, leaving 142. This pass takes 29 of
the remaining bodies in four groups.

- **The `WS` ship actor, `0x80765A90..0x80766F20` (16 bodies).** Constructor
  [`fn_80765d90`] (vtable `0x80815BB8`, manager `+0x128`, animation manager
  `+0x144`, animation object `+0x184`, three `0x34`-byte animation slots at
  `+0x1BC`, `+0x1E8` and `+0x214`), the five-pane release [`fn_80765ba0`], the
  three `WS_W_X` slot loaders [`fn_80766250`]/[`fn_80766340`]/[`fn_80766430`],
  the `WS_Select` base loader with its nine-way variant switch [`fn_807665a0`],
  the pane selector [`fn_807667c0`] (tables `0x80815BA4` and `0x807CEF60`), the
  material refresh [`fn_807668b0`] (`0x802F8FF0` type token), the frame
  save/restore pair [`fn_80766be0`]/[`fn_80766d20`], the paused-frame driver
  [`fn_80766df0`] (`0x80600510` gate plus the `+0x270` yaw states), and the
  ladder transform [`fn_80766f20`] with its trigonometry
  [`fn_807669b0`]/[`fn_80766a70`] and blend helpers
  [`fn_80766b30`]/[`fn_80766b70`]. `+0x27C` selects one of nine variants; `+0xDC`
  scales the model, `+0x244` is the pivot and `+0x2A8..+0x2C0` are vertical
  trims.
- **The `I_kinoko` item actor, `0x80767210..0x807678C0` (8 bodies).** The
  `0x210`-byte allocator [`fn_80767210`], the constructor [`fn_80767240`]
  (vtable `0x80815DB8`, spawn-parameter nibble `actor[+4] & 0xF` into
  `+0x20C`), the destructor [`fn_80767300`], the start [`fn_80767390`], the
  nine-variant model loader [`fn_807675a0`] (model names `0x80815C98`, clips
  `0x80815CC8`, `SI_kinoko_*` overlay `0x80815D38`, `stok_wait` `0x80815D78`,
  switch pane `0x80815D88`), the `stok_wait*` selector [`fn_80767840`] (table
  `0x80815DB0`) and the colour/matrix writer [`fn_807678c0`]. Variants `0` and
  `5` are the two switch forms (`+0x1C4` pane, `0x23` pane id) and variant `5`
  also tints and re-times the model.
- **The randomised layout of the large course object,
  `0x807656B0..0x80765960` (5 bodies).** Four generator calls rebuild the
  transforms `+0xAC`, `+0x454`, `+0x460` and `+0x46C` ([`fn_807656b0`]), 200
  three-float offsets fill `+0x484` ([`fn_80765770`]) through the two biased
  samplers [`fn_807657f0`]/[`fn_80765860`], and [`fn_80765960`] copies the
  cached `+0xDF8` position back.
- **One neighbouring constructor.** [`fn_80767a60`] builds a `0x250`-byte
  object (vtable `0x80815EE0`, manager `+0x18C`, animation object `+0x1A8`,
  slot `+0x1B4` with vtable `0x807E3E98`); no string is reachable from it, so
  the class name stays `// unknown:`.

The new local ABI helpers are `f32_of_f32` (one `f32` in and out, for
`0x80237BD0`/`0x80237C40`), `call_u32x2_f32` (for the `0x80167B30` family frame
getters) and `call_f32_u32x2_i16` (for the degree-halfword offset builders).

## Functions

| address | function | bytes | confidence | role |
|---|---|---:|---|---|
| `0x807656B0` | `fn_807656b0` | 184 | high | four randomised transform rebuilds |
| `0x80765770` | `fn_80765770` | 116 | high | 200 × `0x0C` offset array at `+0x484` |
| `0x807657F0` | `fn_807657f0` | 104 | medium | random vector + `0x8081597C/80/84` |
| `0x80765860` | `fn_80765860` | 104 | medium | random vector + `0x80815970/74/78` |
| `0x80765960` | `fn_80765960` | 28 | high | `+0xDF8` copy into `+0xAC` |
| `0x80765BA0` | `fn_80765ba0` | 112 | high | five-pane stop + release |
| `0x80765D90` | `fn_80765d90` | 252 | high | `WS` constructor |
| `0x80766250` | `fn_80766250` | 236 | high | `WS_W_X` slot `+0x1BC` loader |
| `0x80766340` | `fn_80766340` | 236 | high | `WS_W_X` slot `+0x1E8` loader |
| `0x80766430` | `fn_80766430` | 360 | medium | `WS_W_X` slot `+0x214` loader + rate |
| `0x807665A0` | `fn_807665a0` | 532 | high | `WS_Select` loader + variant switch |
| `0x807667C0` | `fn_807667c0` | 240 | medium | pane/frame selector |
| `0x807668B0` | `fn_807668b0` | 256 | medium | material refresh |
| `0x807669B0` | `fn_807669b0` | 180 | medium | rotated horizontal offset |
| `0x80766A70` | `fn_80766a70` | 188 | medium | rotated vertical offset |
| `0x80766B30` | `fn_80766b30` | 52 | high | corner lerp |
| `0x80766B70` | `fn_80766b70` | 108 | medium | `(h - scale*sin)/h` ratio |
| `0x80766BE0` | `fn_80766be0` | 312 | medium | park three animation frames |
| `0x80766D20` | `fn_80766d20` | 196 | high | restore the three frames |
| `0x80766DF0` | `fn_80766df0` | 300 | high | pause gate + `+0x270` yaw states |
| `0x80766F20` | `fn_80766f20` | 752 | medium | ladder transform writer |
| `0x80767210` | `fn_80767210` | 48 | high | allocate `0x210` + constructor |
| `0x80767240` | `fn_80767240` | 192 | high | `I_kinoko` constructor |
| `0x80767300` | `fn_80767300` | 144 | high | `I_kinoko` destructor |
| `0x80767390` | `fn_80767390` | 164 | high | `I_kinoko` start |
| `0x807675A0` | `fn_807675a0` | 664 | medium | nine-variant model loader |
| `0x80767840` | `fn_80767840` | 128 | high | `stok_wait*` selector |
| `0x807678C0` | `fn_807678c0` | 280 | high | colour + matrix writer |
| `0x80767A60` | `fn_80767a60` | 124 | high | unnamed constructor |

## Address range

`0x807656B0..0x80767A60`, four groups:

- Group 1 is `0x807656B0..0x80765960`; the two `0x80765180`/`0x80765200`/
  `0x80765290` twelve-byte pads and the `0x807658D0` sampler break the run.
- Group 2 is `0x80765BA0..0x80766F20`, 16 bodies. The already-translated
  `0x80765C10` sits between [`fn_80765ba0`] and [`fn_80765d90`] and is not
  repeated here.
- Group 3 is `0x80767210..0x807678C0`, 8 bodies, contiguous apart from the
  `0x80767A60` entry below.
- Group 4 is the single constructor `0x80767A60`.
- After this pass the bucket still has 113 untranslated functions.

## Deliberate skips

Two bodies in this window were rejected rather than guessed, and the rest of
the bucket was simply not reached:

- `0x807658D0` (144 bytes) is the shared random-vector sampler. Its export is a
  paired-single (`GQR0`) reconstruction with six "removing unreachable block"
  warnings and reads `DAT_80815964..0x8081596C` through byte/halfword/word
  quantisation branches that the decompiler could not fold. Its *callers* are
  translated and call it by address with an `// unknown:` note; the body itself
  cannot be reconstructed honestly from this export.
- `0x80765980` (272 bytes) loads `character_SV` and five panes through a saved
  `0x802DCD6C`/`0x802DCDB8` prolog, but the export drops the pointer argument of
  `0x8016A010` and never shows which register holds the pane table; the five
  `0x8016A3A0` calls therefore have no provable owner. Deferred.
- `0x80765A90` (268 bytes) is the matching five-pane transform writer and has
  the same paired-single problem as `0x807658D0` (four unreachable-block
  warnings, `unaff_GQR0` scaling of `f30`/`f31` arguments). Deferred with the
  body above.
- `0x807656B0`/`0x80765770` depend on `0x807658D0` at runtime only through an
  address-only call, so they are still complete transcriptions of their own
  bodies.
- Not reached in this pass: the `0x80760070..0x80764550` block (coincidence /
  effect actors with clear exports), `0x80768470` (276 bytes, the killer
  launcher's remaining helper), `0x8076B560..0x8076BA60`, the two large bodies
  `0x8076C900` (2,100 bytes) and `0x8076DA30` (2,656 bytes), and the
  `0x8076D140..0x8076FF80` tail.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (29 rows, 6,592 bytes),
  re-read from the TSV rather than from prose; `0x807656B0` was corrected from
  a stale 292-byte count to its real 184 bytes during the check.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the 61 pre-existing rows of
  this bucket were listed before writing and none is in this pass.
- `node tools/check-lane-csv.mjs wave85` reports `wave85-bases.csv: 29 rows`
  and `OK`.
- `cargo check --message-format=short` produces no diagnostic for
  `src/game/bases.rs`.
