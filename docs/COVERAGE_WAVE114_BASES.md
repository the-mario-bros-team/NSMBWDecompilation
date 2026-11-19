# Coverage - Wave 114 (`game::bases`, `0x8076` bucket)

## How to read this file

This page documents the Wave 114 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`** produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
cited file is a real Ghidra C export under
`work/ghidra/rel-d_basesNP/export/`, and every byte count was re-read from
`work/ghidra/rel-d_basesNP/export/functions.tsv`.

"Confidence" is `high` when the guards, stores, constants and call arguments are
literal, and `medium` when a field role, a callee identity or a lost receiver had
to be inferred. No body in this pass is `low`. Comments in the bodies keep
`// observed:`, `// inferred:` and `// unknown:` markers.

The `0x8076` bucket holds 203 functions; 90 were already translated before this
pass (waves 61 and 47), leaving 113. This pass takes 29 of the *early* ones, the
contiguous `0x80760070..0x80762B30` window, which is the World-7 course prop
block that immediately precedes the already-translated `cobKinopio` carrier
window (`0x8076A270..0x8076B4A0`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 29 | 5,516 | 17 | 12 |

## What this pass covers

- **The `CS_W7` cloud lift (`0x80760070..0x807603D0`).** Model and archive
  loader [`fn_80760070`] (`CS_W7` archive, `CS_W7_Cloud` model, holder at
  `+0x18C`, part `+0x1AC`, clip player `+0x1B8`, byte `+0x1EC`), the matrix
  writer [`fn_807601a0`], the origin reset pair
  [`fn_80760250`]/[`fn_807602d0`], and the twenty-entry `group01..` clip scan
  [`fn_80760340`] with its sphere-highlight pass [`fn_807603d0`].
- **The `cobCourse` / `cobCourseClear` door (`0x807605C0..0x80761930`).** The
  `0x248`-byte-ish door object with class vtable `0x808150A0` at `+0x60`: the
  constructor [`fn_807605c0`], the loader [`fn_80760b90`] that mounts
  `cobCourse` and builds three `0x2C`-byte animation slots at `+0x1B4` from the
  `cobCourseClear` name table `0x80815070`, the per-frame driver
  [`fn_80760a80`], the part release [`fn_80760b50`], the transform writer
  [`fn_80760f40`], the three-state arm counter [`fn_80760ff0`], the cue handler
  [`fn_80761040`] (events `3`/`0x5E`/`0x93`, delays `10`/`0x1F`, effect `0x21`
  and cue `0x1F`), the slot switcher [`fn_80761260`], the four-state open
  sequence [`fn_80761310`], the type-`0x27E` actor scan [`fn_80761480`], the
  low-variant reset [`fn_80761680`] and the World-2 gate [`fn_80761930`].
- **The `pakkun` course actor (`0x80761A60..0x80762B30`).** The constructor
  [`fn_80761a60`] (vtable `0x80815248`, two part adapters `+0x1AC`/`+0x1EC`,
  `0x38`-byte animation array `+0x22C`, transform nodes `+0x270`/`+0x298`), the
  destructor [`fn_80761b40`], the linked-object release [`fn_80761e60`], the
  `pakkun` / `g3d_pakkun_brres` loader [`fn_80761eb0`] with the `cs_wait` clip,
  the part tick [`fn_80762000`], the variant offset/tint body [`fn_807621b0`]
  with its counter clear [`fn_80762240`], the two neighbouring constructors
  [`fn_807623d0`]/[`fn_807626a0`], the per-frame body [`fn_80762890`] and the
  transform writer [`fn_80762b30`].

Start with [`fn_80760b90`] for the door layout, [`fn_80761040`] for its cue
state machine and [`fn_80761eb0`] for the actor model binding.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80760070` | `fn_80760070` | 292 | medium |
| `0x807601A0` | `fn_807601a0` | 176 | high |
| `0x80760250` | `fn_80760250` | 128 | high |
| `0x807602D0` | `fn_807602d0` | 12 | high |
| `0x80760340` | `fn_80760340` | 136 | medium |
| `0x807603D0` | `fn_807603d0` | 260 | medium |
| `0x807605C0` | `fn_807605c0` | 136 | high |
| `0x80760A80` | `fn_80760a80` | 208 | medium |
| `0x80760B50` | `fn_80760b50` | 48 | high |
| `0x80760B90` | `fn_80760b90` | 932 | medium |
| `0x80760F40` | `fn_80760f40` | 168 | high |
| `0x80760FF0` | `fn_80760ff0` | 80 | high |
| `0x80761040` | `fn_80761040` | 220 | high |
| `0x80761260` | `fn_80761260` | 132 | medium |
| `0x80761310` | `fn_80761310` | 356 | medium |
| `0x80761480` | `fn_80761480` | 144 | medium |
| `0x80761680` | `fn_80761680` | 92 | medium |
| `0x80761930` | `fn_80761930` | 44 | high |
| `0x80761A60` | `fn_80761a60` | 224 | high |
| `0x80761B40` | `fn_80761b40` | 216 | high |
| `0x80761E60` | `fn_80761e60` | 68 | high |
| `0x80761EB0` | `fn_80761eb0` | 324 | medium |
| `0x80762000` | `fn_80762000` | 16 | medium |
| `0x807621B0` | `fn_807621b0` | 140 | high |
| `0x80762240` | `fn_80762240` | 12 | high |
| `0x807623D0` | `fn_807623d0` | 84 | high |
| `0x807626A0` | `fn_807626a0` | 156 | high |
| `0x80762890` | `fn_80762890` | 156 | medium |
| `0x80762B30` | `fn_80762b30` | 188 | high |

**Totals:** 29 translated, 0 partial, 0 stub (17 high, 12 medium, 5,516 bytes).

## Deliberate skips

Read and rejected, with the reason:

- **Paired-single bodies.** `0x80761510` (340 bytes), `0x807616E0` (408) and
  `0x80761880` (176) open with `unaff_GQR0`/`ldexpf` brackets and carry the
  folded `2^52` integer-to-float subtraction, so the source values cannot be
  proved. The two translated call sites ([`fn_80760a80`], [`fn_80760b90`],
  [`fn_80761040`], [`fn_80761310`]) keep them as address calls with explicit
  notes.
- **`0x80761670` (4 bytes)** is a `halt_baddata` stub with no recoverable body.
- **`0x80762010` (404 bytes)** is the "stem growth" transform writer; it mixes
  the GQR bracket, a `double`/`float` ladder around `0x80168080`/`0x801680C0`
  and a `longlong` truncation, so its rate arithmetic cannot be reconstructed
  honestly in this pass.
- **`0x80762CE0` (440) and `0x80762B20` (16).** `0x80762CE0` reads a
  `0x18`-stride variant table with two-negated `_DAT_80377DEC/DF4` branches and
  is left as an address call from [`fn_80762890`]; `0x80762B20` is a 4-byte
  jump-table stub with no recoverable target list.
- **Later `0x8076` bodies not in this window.** `0x80762BF0..0x80764270`,
  `0x80764350..0x807667C0` and `0x8076B090..0x8076FF80` were not opened; several
  are manager-side bodies reached from the new code, which call them by address
  with `// unknown:` notes.

## Notes on inference

- Neither class name is present in any export. The archive and pane strings
  (`CS_W7`, `CS_W7_Cloud`, `cobCourse`, `cobCourseClear`, `pakkun`) and the
  shared model layouts place them in the World-7 course props; the module
  header and the bodies call them the "cloud lift" and the "`cobCourse` door"
  and mark the class names `// inferred:`.
- `0x802DCD6C`/`0x802DCD70`/`0x802DCD74` are the compiler save/restore helper
  family documented in `docs/COVERAGE_r80260_802ed.md`. They hide the incoming
  `this` in `0x80760070`, `0x80760340`, `0x807603D0` and `0x80761EB0`; the
  bodies restore it as a parameter and say so in their doc blocks, following
  the existing `fn_806f9d50`/`fn_806f9e20` convention in this file.
- `0x80761260` receives both rates typed as `undefined8` in the export, while
  every call site passes `(double)<f32 pool word>` and the two callees
  (`0x80166310`/`0x801662F0`) are the f32 setters used elsewhere in this file;
  the body therefore takes `f32` and records the reconstruction.
- The `(float)(4503601774854204.0 - DAT_807CEDD8)` expression in `0x80761310` is
  the decompiler's folded `2^52` integer-to-float sequence; the body reproduces
  the literal expression with an `// unknown:` note instead of inventing a
  symbolic value.
- `0x80815070`, `0x80814E60`, `0x80814EC8` and `0x80815228` are pointer tables
  inside the REL image and are read word by word, exactly as the export
  dereferences the `PTR_s_*` symbols.
- `0x803295D8`, `0x80329578`, `0x807FEF40` and the pool words under `0x807CExxx`
  keep their raw addresses; only the archive/pane strings and the two class
  vtables are named in prose.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (29 rows, 5,516 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 90 translated
  addresses were re-read first and none is duplicated here. The file now holds
  2,856 bodies.
- `node tools/check-lane-csv.mjs wave114` reports `OK` and names no problem in
  `wave114-bases.csv`.
- `cargo check --message-format=short` reports no error or warning naming
  `src/game/bases.rs`; diagnostics raised by other lanes' files are ignored.
