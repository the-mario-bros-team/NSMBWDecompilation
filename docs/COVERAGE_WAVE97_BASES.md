# Coverage - Wave 97 (`game::bases`, `0x8068` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** under the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader chooses the real load address, so the numbers only
identify positions inside the REL image. In this image a virtual address maps
to the REL file at `file_offset = address - 0x80600000`; that is how the float
and string constants cited below were checked. Cross-module pointers
(`0x8035xxxx`, `0x8042xxxx`, `0x807Exxxx`) keep their absolute values.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c` (checked with a glob,
never assumed). "Role" below is the plain-English behaviour. Confidence is
`high` when guards, stores, constants and call arguments are literal, and
`medium` when the role of a field, a global, a callee or a virtual slot had to
be inferred (those bodies carry `// observed:` / `// inferred:` /
`// unknown:` notes). No body in this pass is `low`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 29 | 4,328 | 12 | 17 |

## What this pass covers

The `0x8068` bucket holds 176 functions; 48 were already in
`ledger/functions.csv` when this pass began, leaving 128. This pass takes 29
from three adjacent actor families whose constructors and model loaders were
already translated:

- **`lift_freehole` helper tail, `0x80689B60..0x80689F40` (6 bodies).** The
  `0x470`-byte actor built by [`fn_80689a40`] and given `g3d/lift_freehole.brres`
  by [`fn_80689ab0`] keeps a target handle at `+0x3FC`, a target-relative offset
  at `+0x460..+0x468`, an interface block at `+0x394` and the model at `+0x3F0`.
  [`fn_80689b60`] walks the global list for type `0x1E9` and matches the
  `(word+4 >> 4) & 0xF` variant nibble; [`fn_80689bc0`] snapshots the claimed
  target; [`fn_80689c10`] adds the offset back and spawns effect `0x2F0`
  through `0x801995A0`; [`fn_80689ce0`] re-resolves the handle and re-places;
  [`fn_80689d40`] is the claim path that also dispatches state `0x8082AB70`;
  [`fn_80689f40`] is the model matrix writer.
- **`block_ice` helper tail, `0x8068A290..0x8068AB60` (5 bodies).** The
  `0xDA8`-byte actor built by [`fn_8068a410`] (vtable `0x807EEC78`) and given
  `g3d/block_ice.brres` by [`fn_8068a860`], with its eight `0xE4`-size helper
  blocks at `+0x55C..+0xD74`. [`fn_8068a290`] is the release wrapper (note the
  C's comma operator: `0x8015F800` runs even when the free is skipped);
  [`fn_8068a790`] tears down the interface `+0x394`, the `+0x460` block and the
  first `+0x55C` block; [`fn_8068a980`] rebuilds the model matrix from position
  and the three Euler halfwords; [`fn_8068aa60`] is the event gate for contact
  kinds `0x69..0x6B` and `0x13`; [`fn_8068ab60`] starts the 60-frame launch
  (sound `0x270`, timer `0x3C`, velocity divided by 60).
- **`gabon_rock` state bodies, `0x8068C030..0x8068D510` (18 bodies).** The
  `0x720`-byte actor built by [`fn_8068b3e0`] (vtable `0x807EF0B8`) and
  modelled by [`fn_8068b5f0`] from the `PTR_s_throw_rock` table at
  `0x807EF008`; the class string `gabon_rock` sits at `0x807EF010`. The pass
  covers the two descriptor gates [`fn_8068c030`]/[`fn_8068d330`], the
  scene-bounds test [`fn_8068c080`], the landing transition [`fn_8068c140`]
  with its volume helper [`fn_8068c400`] and per-player aim
  [`fn_8068c450`], the `Wm_en_iron` and landing-smoke emitters
  [`fn_8068c6b0`]/[`fn_8068c720`], the `throw_rock` clip rebind
  [`fn_8068c820`], the spawn restore [`fn_8068c8b0`], the thrown-rock setup
  [`fn_8068ca80`], the duplicated death effect
  [`fn_8068cc90`]/[`fn_8068ce50`], the two fall updates
  [`fn_8068cd70`]/[`fn_8068cf30`], the player-slot snapshot [`fn_8068cf90`]
  and the two angle steppers [`fn_8068d450`]/[`fn_8068d510`].

Start with [`fn_80689b60`] for the list walk, [`fn_8068c080`] for the scene
test and [`fn_8068c450`] for the folded integer-to-double idiom, which is the
only place in this pass where a Ghidra constant had to be unwound against the
REL bytes.

## Address range

`0x80689B60..0x80689F40` (6 bodies), `0x8068A290..0x8068AB60` (5) and
`0x8068C030..0x8068D510` (18). After this pass the `0x8068` bucket still has 99
untranslated functions.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80689B60` | `fn_80689b60` | 92 | medium |
| `0x80689BC0` | `fn_80689bc0` | 80 | medium |
| `0x80689C10` | `fn_80689c10` | 196 | medium |
| `0x80689CE0` | `fn_80689ce0` | 84 | medium |
| `0x80689D40` | `fn_80689d40` | 124 | high |
| `0x80689F40` | `fn_80689f40` | 108 | high |
| `0x8068A290` | `fn_8068a290` | 88 | high |
| `0x8068A790` | `fn_8068a790` | 88 | high |
| `0x8068A980` | `fn_8068a980` | 220 | medium |
| `0x8068AA60` | `fn_8068aa60` | 96 | high |
| `0x8068AB60` | `fn_8068ab60` | 196 | medium |
| `0x8068C030` | `fn_8068c030` | 72 | medium |
| `0x8068C080` | `fn_8068c080` | 188 | medium |
| `0x8068C140` | `fn_8068c140` | 196 | medium |
| `0x8068C400` | `fn_8068c400` | 80 | high |
| `0x8068C450` | `fn_8068c450` | 324 | medium |
| `0x8068C6B0` | `fn_8068c6b0` | 108 | medium |
| `0x8068C720` | `fn_8068c720` | 256 | medium |
| `0x8068C820` | `fn_8068c820` | 136 | high |
| `0x8068C8B0` | `fn_8068c8b0` | 136 | high |
| `0x8068CA80` | `fn_8068ca80` | 164 | medium |
| `0x8068CC90` | `fn_8068cc90` | 204 | medium |
| `0x8068CD70` | `fn_8068cd70` | 224 | medium |
| `0x8068CE50` | `fn_8068ce50` | 204 | medium |
| `0x8068CF30` | `fn_8068cf30` | 96 | high |
| `0x8068CF90` | `fn_8068cf90` | 184 | medium |
| `0x8068D330` | `fn_8068d330` | 168 | high |
| `0x8068D450` | `fn_8068d450` | 132 | high |
| `0x8068D510` | `fn_8068d510` | 84 | high |

## Deliberate skips

The bodies below were read and rejected rather than guessed; the rest of the
bucket was simply not reached.

- `0x8068AB10` is the state helper called by [`fn_8068ab60`]. Its export is
  truncated with `WARNING: Control flow encountered bad instruction data` and
  `halt_baddata()`, so it exists only as an address-only `platform::call0`.
  [`fn_8068ab60`] is therefore marked `medium`, not `high`.
- `0x8068C5A0` is the floor-height probe shared by [`fn_8068ca80`],
  [`fn_8068cd70`] and the untranslated frame bodies. It is a paired-single
  reconstruction (`unaff_GQR0`, `ldexpf` lane extraction, `extraout_f1`), so no
  body was emitted; the three callers model it as `call_f32_u32` and describe
  the `f1` return explicitly. This is the same failure documented for
  `0x806A23B0` in wave 95 and `0x80662E10` in wave 96.
- `0x8068C210` starts with `func_0x802dcd74()` returning a 64-bit pair whose
  halves are then used as a player index and a pointer; the register class is
  not recoverable, and the body also blends the `+0xE8`/`+0xF4` speeds through
  the same paired-single save bracket. Not entered.
- `0x8068C940` calls `0x80168FE0` and `0x801690E0` with the float argument
  dropped by the export (the same address is called as `(f32, ptr)` elsewhere
  in this file). Without the missing constant the frame step cannot be
  reconstructed, so it was left rather than guessed.
- `0x8068CB40` calls `0x80075270` with three arguments (`x`, surface byte, `0`)
  while the four-argument form used by [`fn_8068d330`] and the rest of the file
  takes a second coordinate. The lost float blocks an honest body.
- `0x8068D2C0` contains a `+0x60`-vtable virtual whose argument list the export
  prints empty; the receiver is not provable, so it was skipped with `0x8068`
  having plenty of fully readable neighbours.
- The `0x802DCBFC` 48-byte forwarders already documented at `0x806512F0` were
  not re-emitted; none of them fall inside the three ranges above.
- Not reached: the large bodies `0x80680720` (660 bytes), `0x80680FD0` (676),
  `0x806822B0` (1,108), `0x80684680` (936), `0x80686550` (1,884) and
  `0x8068E0B0` (912), plus the remaining `0x8068` helpers outside the three
  clusters.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (29 rows, 4,328 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 29 were on the bucket's
  untranslated list before writing.
- The constants cited in [`fn_8068c450`] and [`fn_8068ab60`] were checked
  against the REL bytes at `file_offset = address - 0x80600000`
  (`0x807C9A60` = `0x4330000080000000`, `0x807C9B38` = the same bias pair,
  `0x807C9B68` = `0x4330000000000000`).
- `node tools/check-lane-csv.mjs wave97` reports `wave97-bases.csv: 29 rows`
  and `OK`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/bases.rs`; the only errors in the crate at the end of the pass
  belong to the other wave-97 lanes.
