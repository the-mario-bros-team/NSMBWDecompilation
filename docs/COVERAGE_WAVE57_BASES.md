# Coverage - Wave 57 (`game::bases`, `0x8067` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL (synthetic base `0x80600000`; every address is a static analysis address,
not a runtime address). Byte counts come from
`work/ghidra/rel-d_basesNP/export/functions.tsv`; the quoted evidence files are
the Ghidra C exports under `work/ghidra/rel-d_basesNP/export/`. Confidence is
`high` when guards, stores, vtables and call arguments are literal, and `medium`
when a callee role, a dropped argument or a float ABI had to be inferred.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 6172 | 16 | 14 |

## What this pass covers

- The Lemmy-ball actor body (`0x80670030..0x80672390`), which sits directly
  below the already translated `daCastleLemmyBall_c` state table at
  `0x80672500`: the four-player gate [`fn_80670030`], the `0xC20`-byte
  multi-vtable constructor [`fn_80670640`], its base destructor
  [`fn_806708e0`] and full destructor [`fn_80672390`], the `+0x30` child-chain
  walk [`fn_80670440`], the trimmed-name comparison [`fn_80670520`], and the
  animation states [`fn_80671620`] / [`fn_80671740`] / [`fn_80671810`] /
  [`fn_806719f0`] / [`fn_80671bd0`] / [`fn_80671cd0`] that bind
  `wait_ball_big` (`0x807E9ED0`) and `throw_ball_big_ed_wait` (`0x807E9E74`)
  and spawn `Wm_ob_ballgrowbig` (`0x807E9EEC`) through the `+0x8B8` effect
  interface. [`fn_80671280`] counts the `+0x8A0`/`+0x8A4` animation span.
- The `0x6F8`-byte sibling actor constructor [`fn_80673020`] (vtable
  `0x807EA610`) and its three player-event handlers [`fn_806730d0`] /
  [`fn_80673170`] / [`fn_80673210`], which dispatch player state `1` to the
  wave-23 helpers `0x806B4A10` / `0x806B4B20` / `0x806B4BA0` and state `3` to
  the actor virtual `+0xD4` with the `0x80358584` tag.
- The masked-input watcher [`fn_806741e0`] that mirrors the `+0x3A8` mode into
  `0x80673F50` and the `+0x3B8` latch, and the slot query [`fn_80674470`].
- The `choropoo` actor: the `g3d/choropoo.brres` loader [`fn_806744f0`] with
  its `choropoo` model (`0x807EAB18`) and `choropoo_hole` pane
  (`0x807EAB24`), the `0x4E0`-byte neighbour constructor [`fn_806749c0`], the
  model advance [`fn_80674e00`], the transform rebuild [`fn_80674e50`], the
  frame-4 animation binding [`fn_80674f40`], the player-speed scale
  [`fn_80674f90`] / [`fn_80674ff0`], and the animation selector
  [`fn_806751f0`].
- The `Wm_ob_flagget` flag-pole handlers [`fn_806752a0`] and [`fn_806753b0`]:
  the capture timer, the `0x800B1100` visual, the `0x8008EF30` capture flags,
  the `Wm_ob_flagget` effect (`0x807EB0D4`), sound `0xAC`, and the global
  captured flag at `0x80429DD0 + 0x90124`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80670030` | `fn_80670030` | 200 | high |
| `0x80670300` | `fn_80670300` | 92 | high |
| `0x80670440` | `fn_80670440` | 224 | medium |
| `0x80670520` | `fn_80670520` | 136 | medium |
| `0x80670640` | `fn_80670640` | 436 | high |
| `0x806708E0` | `fn_806708e0` | 236 | high |
| `0x80671280` | `fn_80671280` | 52 | high |
| `0x80671620` | `fn_80671620` | 280 | medium |
| `0x80671740` | `fn_80671740` | 188 | high |
| `0x80671810` | `fn_80671810` | 476 | medium |
| `0x806719F0` | `fn_806719f0` | 116 | high |
| `0x80671BD0` | `fn_80671bd0` | 248 | medium |
| `0x80671CD0` | `fn_80671cd0` | 136 | high |
| `0x80672390` | `fn_80672390` | 368 | high |
| `0x80673020` | `fn_80673020` | 176 | high |
| `0x806730D0` | `fn_806730d0` | 148 | medium |
| `0x80673170` | `fn_80673170` | 148 | medium |
| `0x80673210` | `fn_80673210` | 148 | medium |
| `0x806741E0` | `fn_806741e0` | 312 | medium |
| `0x80674470` | `fn_80674470` | 80 | high |
| `0x806744F0` | `fn_806744f0` | 172 | high |
| `0x806749C0` | `fn_806749c0` | 236 | high |
| `0x80674E00` | `fn_80674e00` | 48 | high |
| `0x80674E50` | `fn_80674e50` | 228 | medium |
| `0x80674F40` | `fn_80674f40` | 68 | high |
| `0x80674F90` | `fn_80674f90` | 96 | high |
| `0x80674FF0` | `fn_80674ff0` | 72 | medium |
| `0x806751F0` | `fn_806751f0` | 164 | medium |
| `0x806752A0` | `fn_806752a0` | 272 | medium |
| `0x806753B0` | `fn_806753b0` | 616 | medium |

## Address range

`0x80670030..0x806753B0` (three contiguous runs: the Lemmy-ball actor at
`0x80670030..0x80672390`, the `0x6F8` actor and its handlers at
`0x80673020..0x80673210`, and the `choropoo` / flag-pole block at
`0x806741E0..0x806753B0`).

## Deliberate skips

- `0x80670fa0` (184), `0x80671060` (172), `0x80671510` (248), `0x80671a80`
  (256) and `0x80671d70` (536) are the actor's transform/effect handlers. Ghidra
  reconstructs their paired-single loads as `unaff_GQR0` plus `ldexpf` calls and
  removes unreachable blocks, so the float sequence cannot be recovered from the
  export honestly.
- `0x80673f50` (656) is the capture spawn called by [`fn_806741e0`]; it is still
  rejected for the same paired-single reason and is called by address.
- `0x80675040` (68) ends in `halt_baddata()` after a truncated control-flow
  transfer, and `0x80675090` (348) embeds the same GQR/`ldexpf` pattern. Both
  were left for a later wave.
- `0x80670bb0` (108) copies a two-byte stack word through `0x8000D5B0`,
  overwrites only its low byte and passes it back to `0x80168880`; the partial
  buffer makes the intended value ambiguous.
- `0x80673600` (76) subtracts `DAT_807c91c8` from the constant
  `4503601774856448.0` (a `2^52` conversion bias) inside a single expression;
  the export does not establish whether the subtraction is part of the
  fixed-point conversion or a real offset, so it was skipped.
- `0x80672e20`, `0x80672f00`, `0x80672fc0` and `0x80672ff0` are byte-identical
  copies of the already translated `0x80670440`, `0x80670520`, `0x806705e0`
  and `0x80670610` helpers (same child-chain walk, name compare and
  `0x802DCBFC` forwarders). They add no new evidence and were not duplicated.
- `0x806705e0` (48) and `0x80670610` (48) are the two `0x802DCBFC` forwarders
  already represented by `fn_806512f0`/`fn_80651320`; the same hidden-`r12`
  convention gives no additional information here.
- These addresses were already in `ledger/functions.csv` and were not
  duplicated: `0x80672500`, `0x80672B20`, `0x80672B90`, `0x80672BE0`,
  `0x80672C60`, `0x80672CC0`, `0x80672D20`, `0x80672D30`, `0x80673830`,
  `0x80673A50`, `0x80673AB0`, `0x80673DA0`, `0x80674C00`, `0x80676E10`,
  `0x80677730`, `0x80677870`, `0x80677DD0`, `0x80679200`, `0x806798F0`,
  `0x80679A70`, `0x8067AE50`, `0x8067B5B0`, `0x8067B8F0`, `0x8067BA70`,
  `0x8067F4D0`.
- The rest of the `0x8067` bucket (144 untranslated at lane end, including the
  `0x806755xx..0x80677Exx` flag/`kokkan` block and the `0x8067DC50..0x8067FC80`
  tail) is outside this window.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv`.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000` line.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error for
  `src/game/bases.rs`.
- `node tools/check-lane-csv.mjs wave57` reports `OK` for `wave57-bases.csv`.
