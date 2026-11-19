# Coverage - Wave 119 (`game::bases`)

## How to read this file

This page documents the Wave 119 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`** produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
cited file is a real Ghidra C export under
`work/ghidra/rel-d_basesNP/export/`, and every byte count was re-read from
`work/ghidra/rel-d_basesNP/export/functions.tsv`.

The `0x8069` bucket holds 185 functions. Before this pass 79 were already
translated, leaving 106. This pass takes 26 of them, from `0x8069A140` to
`0x8069FEB0`: the remaining helper bodies of the `0xEB0`-byte `koopa_clown`
actor family whose constructor is [`fn_8069dbc0`], plus its release wrapper and
the two `+0x30` chain twins.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee identity, a field role or an interface slot
had to be inferred. Comments in the bodies keep `// observed:`, `// inferred:`,
`// partial:` and `// unknown:` markers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 26 | 4,012 | 4 | 22 |

All 26 rows are `translated`; none is `partial`.

## What this pass covers

- **State helpers (`0x8069A140..0x8069A940`), 11 bodies.** The drop/height test
  [`fn_8069a140`]; the `0x80699F80` window gate that re-enters state
  `0x8082B5F0` [`fn_8069a230`]; the two player-query selectors
  [`fn_8069a290`] / [`fn_8069a300`]; the one-shot `+0xDB8` latch and state
  `0x8082B6B0` [`fn_8069a370`]; the pane/count writer [`fn_8069a430`]; the
  player-mask/tag query [`fn_8069a680`]; the count blend over `0x807F13D0` and
  `0x807F13D4` [`fn_8069a7d0`]; the `+0xDBC` zero test [`fn_8069a8f0`]; the
  `+0xDDC` countdown [`fn_8069a910`]; and the character-scale/state writer
  [`fn_8069a940`].
- **Per-frame transitions and spawn gates (`0x8069B200..0x8069EAE0`), 12
  bodies.** The two `+0x378` "alive" switches [`fn_8069b200`] /
  [`fn_8069b650`]; the mode/timer refresh [`fn_8069bce0`]; the kind-`0x130`
  counter [`fn_8069e370`] and its spawn gate [`fn_8069e3d0`]; the `2`/`3` mode
  selector [`fn_8069e6c0`]; the `0x80064740` neighbour placement with its
  character-table offset [`fn_8069e880`]; the `0x80168230` transform spawn
  [`fn_8069e9b0`]; the `0x8082B8B4` interface-name state switch
  [`fn_8069ea30`]; and the two-target `+0x778`/`+0x77C` slide [`fn_8069eae0`].
- **Spawn arming (`0x8069F080..0x8069FEB0`), 2 bodies.** The `+0xD7C` cap and
  `+0x502` timer reset [`fn_8069f080`] and the ground-probe start
  [`fn_8069feb0`].
- **Teardown and chain twins (`0x8069D990..0x8069DC70`), 3 bodies.** The
  `+0x30` child walk [`fn_8069d990`] and colon-trimmed name compare
  [`fn_8069da70`] (byte-for-byte twins of [`fn_806be930`] / [`fn_806bea10`]),
  and the `koopa_clown` release wrapper [`fn_8069dc70`].

Shared fields: class vtable `+0x60`, resource container `+0x540`, model block
`+0x544`, state word `+0x360`, timer `+0x500`, latch `+0xDB8`, mode `+0xDCC`,
countdown `+0xDDC`, and the player-like object returned by class slots `+0x2DC`
/ `+0x3B4` whose interface at `+0x5F8` answers slots `+0xC`, `+0x10` and
`+0x14`.

Start with [`fn_8069a940`] (the character scaling), [`fn_8069e880`] (the
neighbour placement) and [`fn_8069e9b0`] (the transform spawn).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8069A140` | `fn_8069a140` | 36 | high |
| `0x8069A230` | `fn_8069a230` | 92 | medium |
| `0x8069A290` | `fn_8069a290` | 100 | medium |
| `0x8069A300` | `fn_8069a300` | 100 | medium |
| `0x8069A370` | `fn_8069a370` | 192 | medium |
| `0x8069A430` | `fn_8069a430` | 232 | medium |
| `0x8069A680` | `fn_8069a680` | 92 | medium |
| `0x8069A7D0` | `fn_8069a7d0` | 276 | medium |
| `0x8069A8F0` | `fn_8069a8f0` | 20 | high |
| `0x8069A910` | `fn_8069a910` | 48 | high |
| `0x8069A940` | `fn_8069a940` | 180 | medium |
| `0x8069B200` | `fn_8069b200` | 268 | medium |
| `0x8069B650` | `fn_8069b650` | 240 | medium |
| `0x8069BCE0` | `fn_8069bce0` | 296 | medium |
| `0x8069D990` | `fn_8069d990` | 224 | medium |
| `0x8069DA70` | `fn_8069da70` | 136 | medium |
| `0x8069DC70` | `fn_8069dc70` | 112 | high |
| `0x8069E370` | `fn_8069e370` | 84 | medium |
| `0x8069E3D0` | `fn_8069e3d0` | 148 | medium |
| `0x8069E6C0` | `fn_8069e6c0` | 124 | medium |
| `0x8069E880` | `fn_8069e880` | 304 | medium |
| `0x8069E9B0` | `fn_8069e9b0` | 124 | medium |
| `0x8069EA30` | `fn_8069ea30` | 172 | medium |
| `0x8069EAE0` | `fn_8069eae0` | 108 | medium |
| `0x8069F080` | `fn_8069f080` | 232 | medium |
| `0x8069FEB0` | `fn_8069feb0` | 72 | medium |

**Totals:** 26 bodies, 4,012 bytes (4 high, 22 medium).

## Deliberate skips

Read and rejected, with the reason:

- **`0x802DCBFC` thunks.** `0x8069DB30` and `0x8069DB60` only forward two
  unnamed arguments to the unresolved indirect-jump stub `0x802DCBFC`. The
  module header already records that stub as unresolved, so a body here would
  be a signature guess.
- **Receiver-less `0x802DCD70` bodies.** `0x8069E5F0`, `0x8069F560` and
  `0x8069F9D0` recover their object through the `0x802DCD70` save stub and end
  with `0x802DCDBC`; the exports print no owner argument, so the receiver would
  have to be invented. `0x8069FF00` calls `0x8069E5F0` first and was left with
  it rather than shipped as a call to a body that does not exist.
- **Paired-single / GQR reconstruction failures.** `0x8069A170`,
  `0x8069A6E0`, `0x8069AB00` and `0x8069B750` carry Ghidra's
  `unaff_GQR0`/`ldexpf` reconstruction and, in `0x8069A6E0`/`0x8069AB00`,
  payload registers whose paired-single slots never reach a named parameter.
  The constants and call sites are visible, but the short-to-float conversions
  cannot be rebuilt without guessing which slot holds which value.
- **Unrecovered jump table.** `0x8069ACA0` contains
  "Could not recover jumptable at 0x8069ACC4. Too many branches" and Ghidra
  replaced the dispatch with a bare indirect call, so the transition target is
  lost.
- **Long bodies left for a later sweep.** `0x8069F640` (892 bytes) and
  `0x8069FAB0` (996 bytes) are reconstructable in principle — both repeat the
  `CONCAT44` widening already handled here — but they sit outside this pass's
  26-body window and were not written.
- **The rest of the bucket.** The 80 other untranslated addresses in `0x8069`
  were not translated in this pass; they remain for a later sweep.

## Notes on inference

- No class name survives in these exports. The header calls the family
  `koopa_clown` because the constructor [`fn_8069dbc0`] and the loader
  [`fn_8069dce0`] build the same `+0x60`/`+0x540`/`+0x544` layout; the prose
  never presents the vtable as a named class.
- Class slots `+0x2DC` and `+0x3B4` return the same player-like object; its
  interface at `+0x5F8` answers slots `+0xC`, `+0x10` and `+0x14`, but the
  object and the three answers stay `// unknown:`.
- The `CONCAT44(0x43300000, x ^ 0x80000000)` blocks in `0x8069A430`,
  `0x8069A7D0`, `0x8069A940`, `0x8069E880` and `0x8069FEB0` are Ghidra's signed
  word-to-float widening (`2^52 + 2^31` bias at `0x807CA0B0` / `0x807CA208`).
  The bodies use plain `as i32 as f32` / `as f32` casts and keep an
  `// observed:` note at each site.
- [`fn_8069a680`] never reads its first argument; the parameter is kept because
  the call sites pass the owner in `r3`.
- [`fn_8069e370`] takes no argument, although its callers pass the owner in
  `r3`; the export's own prototype has no parameters.
- [`fn_8069a940`] calls `0x80C30B10` with no printed arguments. Only the address
  survives, so the body calls it as a no-argument routine and says so.
- [`fn_8069dc70`] prints the guard `object + 0xD98 != 0`, which is always true
  for a non-null object; the body keeps the test as the transform-block guard
  the compiler emitted.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (26 rows, 4,012 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 79 translated
  addresses were re-read first and none is duplicated here. The file now holds
  2,994 bodies.
- `node tools/check-lane-csv.mjs wave119` reports `OK` and names no problem in
  `wave119-bases.csv`.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/game/bases.rs`; diagnostics raised by other lanes' files are ignored.
