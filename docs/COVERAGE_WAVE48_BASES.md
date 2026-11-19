# Coverage - Wave 48 (bases)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 3680 | 11 | 16 |

The remaining three bodies are `low` confidence: they are the
`0x802DCBFC` tail-call wrappers `fn_80743ef0`, `fn_80744ab0` and
`fn_80744ae0`.

## What this pass covers

- The `0x80743BA0..0x80744010` torch-light tail: the `daTorchiLight_c`
  destructor [`fn_80743bf0`] (which retypes and releases the `+0x4A8`
  sub-object, closes five handles and runs the base destructor
  `0x800671B0`), the two-part release [`fn_80743ba0`], the seven-entry
  variant selector [`fn_80744010`], and the `0x802DCBFC` forwarding stub
  [`fn_80743ef0`].
- The `obj_chikuwa_block` actor, `0x80744750..0x80745610`: two registered
  state destructors [`fn_80744750`] / [`fn_807447b0`], the two constant
  class getters [`fn_80744810`] / [`fn_80744820`], the `+0x30` parent-chain
  probe [`fn_80744910`] and the `name`-suffix comparison [`fn_807449f0`].
- The `0x18C`-byte actor itself: constructor [`fn_80744b10`] (vtable
  `0x80810758`, arena `+0x128`, model `+0x144`, animation `+0x15C`), the
  `g3d/obj_chikuwa_block.brres` loader with its `obj_chikuwa_block` pane
  [`fn_80744dc0`], the animation-end check [`fn_80744c50`], the two
  transform writers [`fn_80744cf0`] (saved position `+0x17C`) and
  [`fn_807452a0`] (per-variant offset pair at `0x80810818`), and the
  destructor [`fn_80745610`].
- The `g3d/rail.brres` pass: the loader [`fn_80745550`] (`rail_00` name
  table `0x808109D8`), the per-variant parameter pair [`fn_80745380`], the
  `rail_mat` material binding and flag clear [`fn_80745410`], and the model
  node loop [`fn_80745490`]. The two state wrappers [`fn_80744cc0`] and
  [`fn_80745270`] re-run these passes and return 1.
- The neighbouring rail-placement actor, `0x80746100..0x80747310`:
  the `0xE4`-stride sub-object reset [`fn_80746100`], the timer plus
  camera-window test [`fn_80746510`], the horizontal overlap scan
  [`fn_80746560`], the chained up-to-100 record spawner [`fn_80746890`], and
  the two state handlers [`fn_80747260`] / [`fn_80747310`] that signal the
  interface event `0x808316A0` / `0x80831660`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80743BA0` | `fn_80743ba0` | 80 | medium |
| `0x80743BF0` | `fn_80743bf0` | 208 | high |
| `0x80743EF0` | `fn_80743ef0` | 48 | low |
| `0x80744010` | `fn_80744010` | 68 | high |
| `0x80744750` | `fn_80744750` | 88 | high |
| `0x807447B0` | `fn_807447b0` | 92 | high |
| `0x80744810` | `fn_80744810` | 12 | high |
| `0x80744820` | `fn_80744820` | 12 | high |
| `0x80744910` | `fn_80744910` | 224 | medium |
| `0x807449F0` | `fn_807449f0` | 136 | medium |
| `0x80744AB0` | `fn_80744ab0` | 48 | low |
| `0x80744AE0` | `fn_80744ae0` | 48 | low |
| `0x80744B10` | `fn_80744b10` | 148 | high |
| `0x80744C50` | `fn_80744c50` | 108 | medium |
| `0x80744CC0` | `fn_80744cc0` | 36 | medium |
| `0x80744CF0` | `fn_80744cf0` | 200 | medium |
| `0x80744DC0` | `fn_80744dc0` | 356 | high |
| `0x80745270` | `fn_80745270` | 36 | medium |
| `0x807452A0` | `fn_807452a0` | 212 | medium |
| `0x80745380` | `fn_80745380` | 132 | medium |
| `0x80745410` | `fn_80745410` | 116 | medium |
| `0x80745490` | `fn_80745490` | 180 | medium |
| `0x80745550` | `fn_80745550` | 192 | high |
| `0x80745610` | `fn_80745610` | 116 | high |
| `0x80746100` | `fn_80746100` | 76 | medium |
| `0x80746510` | `fn_80746510` | 48 | medium |
| `0x80746560` | `fn_80746560` | 220 | medium |
| `0x80746890` | `fn_80746890` | 116 | high |
| `0x80747260` | `fn_80747260` | 136 | medium |
| `0x80747310` | `fn_80747310` | 188 | medium |

## Deliberate skips

- `0x80740370` (552 bytes), `0x80740a60` (204 bytes), `0x80740c90`
  (388 bytes) and `0x80746910` (344 bytes): the exports contain the
  paired-single/GQR artefacts (`unaff_GQR0`, `ldexpf`, `CONCAT12/13/22`)
  already rejected in wave 47, so the float paths cannot be reconstructed
  honestly.
- `0x807449F0`'s string helpers (`0x802E1C40`, `0x802E1AB4`) are identified
  as `strchr`/`strcmp` by argument shape only; the bodies keep the raw call
  addresses and the confidence is `medium`.
- The rest of the `0x80740000` bucket (`0x80747310` upward; 197 entries were
  untranslated before this pass, 167 remain after it) is outside this lane.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv`.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000` line.
- No address was already in `ledger/functions.csv`; the only earlier entries
  in the bucket are the `0x80743CC0..0x80743F50` torch-light trio, the
  `0x80745A80`/`0x80745B30` `untei_rope` pair and the
  `0x8074E120..0x8074FF90` wave-31/32 range.
- `cargo check --message-format=short` reports no error for
  `src/game/bases.rs` (remaining warnings are in other modules).
- `node tools/check-lane-csv.mjs wave48` reports no problem naming
  `wave48-bases.csv`.
