# Coverage - Wave 106 (`game::bases`, `0x8075` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the
`d_basesNP` REL. Every address on this page is a **static analysis address**
under the canonical synthetic base **`0x80600000`** (produced by
`tools/rel2elf.mjs`), never a runtime address: the Wii REL loader chooses the
real load address, so the numbers only identify positions inside the REL image.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c` (globbed by address
prefix, never assumed). "Confidence" is `high` when guards, stores, constants
and call arguments are literal, and `medium` when a callee identity, a field
role or a dropped argument had to be inferred; those bodies carry
`// observed:` / `// inferred:` notes. No body in this pass is `low`.

The bucket holds 207 functions; 87 were already in `ledger/functions.csv` when
this pass began, leaving 120. This pass takes 28 of them, in three clusters
inside `0x80750000..0x8075FD00`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 28 | 2,456 | 23 | 5 |

## What this pass covers

- **The warp-cannon tail (`0x80750000..0x80751F80`, 12 bodies).** These bodies
  continue the warp-cannon actor that wave 31 translated up to `0x8074FF90`:
  the quarter-turn angle handler [`fn_80750000`] (the `+0x2000` bias is what
  separates it from [`fn_8074fc90`]), the camera/world double blend
  [`fn_80750090`], the model/name bind [`fn_80750180`], the full destructor
  [`fn_80750220`], three byte-identical release wrappers
  ([`fn_80750a70`]/[`fn_80751740`]/[`fn_80751f80`]), the `+0x520` handle pair
  [`fn_80750e10`]/[`fn_80750ea0`], and the `0x570`-byte water-rail
  constructor [`fn_807518c0`] with its interface step [`fn_80751b20`] and
  water-surface clamp [`fn_80751bd0`].
- **The water-rail `0x1A4xx` family (`0x80754370..0x80754870`, 11 bodies).**
  The shutdown/restart/motion trio [`fn_80754370`]/[`fn_807543c0`]/
  [`fn_80754400`], the two-word height publisher [`fn_807545a0`] and its
  per-frame writer [`fn_807545c0`], the type-`0x1DC` owner walk
  [`fn_80754670`] with its store [`fn_807546b0`] and resolve
  [`fn_807546c0`], the state entry [`fn_80754740`], and the two path-advance
  bodies [`fn_807547d0`]/[`fn_80754870`]. Both advance bodies add the height
  delta produced by `0x80754620` and step the 7-bit phase byte at `+0x1A4B8`.
- **The `cobCastle` door tail (`0x8075F570..0x8075FD00`, 5 bodies).** The
  neighbouring wave-78 castle object ends at `0x8075F1C0`; this pass resumes
  in the following actor: the timer clear [`fn_8075f570`], the one-shot
  `0x29D` open effect [`fn_8075f980`], the `Koopa` bone record
  [`fn_8075fb30`], the position offsetter [`fn_8075fb90`] and the
  `0x80814EF8` in-place constructor [`fn_8075fd00`].

Start with [`fn_807518c0`] for the water-rail layout and [`fn_807545c0`] for
its height arithmetic; [`fn_8075fd00`] is the clearest in-place constructor in
this pass.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80750000` | `fn_80750000` | 104 | high |
| `0x80750090` | `fn_80750090` | 228 | medium |
| `0x80750180` | `fn_80750180` | 124 | medium |
| `0x80750220` | `fn_80750220` | 168 | high |
| `0x80750A70` | `fn_80750a70` | 88 | high |
| `0x80750E10` | `fn_80750e10` | 12 | high |
| `0x80750EA0` | `fn_80750ea0` | 92 | high |
| `0x80751740` | `fn_80751740` | 88 | high |
| `0x807518C0` | `fn_807518c0` | 156 | medium |
| `0x80751B20` | `fn_80751b20` | 40 | high |
| `0x80751BD0` | `fn_80751bd0` | 116 | high |
| `0x80751F80` | `fn_80751f80` | 88 | high |
| `0x80754370` | `fn_80754370` | 80 | high |
| `0x807543C0` | `fn_807543c0` | 64 | high |
| `0x80754400` | `fn_80754400` | 40 | high |
| `0x807545A0` | `fn_807545a0` | 32 | high |
| `0x807545C0` | `fn_807545c0` | 88 | high |
| `0x80754670` | `fn_80754670` | 56 | high |
| `0x807546B0` | `fn_807546b0` | 16 | high |
| `0x807546C0` | `fn_807546c0` | 84 | high |
| `0x80754740` | `fn_80754740` | 100 | high |
| `0x807547D0` | `fn_807547d0` | 128 | medium |
| `0x80754870` | `fn_80754870` | 84 | medium |
| `0x8075F570` | `fn_8075f570` | 12 | high |
| `0x8075F980` | `fn_8075f980` | 92 | high |
| `0x8075FB30` | `fn_8075fb30` | 92 | high |
| `0x8075FB90` | `fn_8075fb90` | 60 | high |
| `0x8075FD00` | `fn_8075fd00` | 124 | high |

**Totals:** 28 translated, 0 partial, 0 stub (23 high, 5 medium, 2,456 bytes).

## Deliberate skips

Read and rejected, with the reason:

- `0x80750B90`, `0x80750BC0`, `0x80751400`, `0x80751430`, `0x80751860`,
  `0x80751890`, `0x807520A0` and `0x807520D0` (48 bytes each) are two-word
  stubs that tail-call the unresolved indirect-jump helper `0x802DCBFC`; the
  same trap is documented in the module header and in
  `docs/COVERAGE_r80260_802ed.md`.
- `0x80750DA0` (4 bytes) is an empty `blr` body. It has no instruction to
  reconstruct and its only caller, [`fn_80750d20`], already invokes it by
  address.
- `0x80754430` (308 bytes), `0x80754620` (80 bytes) and `0x80751C90`
  (120 bytes) open with `unaff_GQR0`/`unaff_GQR3` paired-single brackets and
  `ldexpf` calls. The incoming GQR state cannot be reconstructed from the
  export, so only the two callers of `0x80754620` are translated, calling it by
  address; its own body is left untranslated.
- `0x80754570` (36 bytes) ends with an indirect jump that Ghidra could only
  recover as "too many branches, treating indirect jump as call"
  (`0x80754590`), so the callee's argument list is lost.
- `0x8075F9E0` (84 bytes) is declared with two `undefined8` parameters and
  calls `0x8075FA40` with two arguments even though that body reads a third
  (the actor) argument; the missing pointer cannot be recovered honestly.
- `0x8075FA40` (240 bytes) is the paired-single GQR body that
  `0x8075F9E0` targets.

The remaining untranslated functions of the bucket were not opened in this
pass (budget, not doubt); this pass deliberately stops at `0x8075FD00`, the
end of the bucket.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (28 rows, 2,456 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 87 translated
  addresses were re-read first and none is duplicated here. The file now holds
  2,624 bodies.
- `node tools/check-lane-csv.mjs wave106` reports
  `wave106-bases.csv: 28 rows` and `OK`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/bases.rs`.
