# Coverage - Wave 219 (`resource::archive`, main.dol part 1)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/archive.rs` | `resource::archive` | 60 | 17,580 | 50 | 10 |

First 60 `resource::archive` bodies, `0x802C0010`-`0x802C4D80`, byte-balanced
across lanes (2,436-3,824 B each). The module is now **609 / 747**.

## What landed

- **Sprite and panel bodies (lane A).** `0x802C0450`, `0x802C0B60`,
  `0x802C11E0`, the 4-byte `0x802C1270`, `0x802C2300`, `0x802C2F70`, the
  432-byte `0x802C32B0`, `0x802C34B0`, `0x802C35F0` and the 2,424-byte
  `0x802C3D80`.
- **GX state and texture bodies (lane B).** `0x802C0110`, `0x802C02C0`,
  `0x802C0BA0`, `0x802C0C70`, `0x802C20E0`, `0x802C2250`, `0x802C30C0`,
  `0x802C35B0`, `0x802C38E0` and the 1,664-byte `0x802C4700`.
- **Render-state reset and quads (lane C).** `0x802C0010`, `0x802C0240`,
  the 1,104-byte `0x802C05E0`, `0x802C1710`, `0x802C17D0`, `0x802C18C0`,
  `0x802C2F00`, `0x802C31A0`, `0x802C3570`, the 548-byte `0x802C4D80`.
- **Tile decoders and draw loops (lane D).** the 936-byte `0x802C0E30`,
  `0x802C1280`, `0x802C1340`, `0x802C1370`, `0x802C17C0`, `0x802C20F0`,
  `0x802C22C0`, the 2,048-byte `0x802C2700`, `0x802C3050`, `0x802C3210`.
- **Display objects and GX channels (lane E).** `0x802C0A80`, `0x802C0AC0`,
  `0x802C0B30`, `0x802C0C20`, `0x802C1D10`, `0x802C1F80`, `0x802C2060`,
  the 920-byte `0x802C2360`, `0x802C3020`, `0x802C3630`.
- **Quads, ribbons and destructors (lane F).** `0x802C0040`, `0x802C0090`,
  `0x802C0A30`, `0x802C0B40`, the 800-byte `0x802C13F0`, `0x802C17E0`,
  the 728-byte `0x802C1A30`, `0x802C3000`, `0x802C3120`, `0x802C3790`.

## Corrections and recoveries

- **`0x802C2700` is 2,048 bytes, not 576.** The tsv value stops at the
  18-entry jump table at `0x80350198`; the arms run to `0x802C2DC8`, the shared
  capture tail to `0x802C2DCC`, and the `blr` sits at `0x802C2EFC` — exactly the
  next entry. `# Original` keeps 576 for the checker; the prose records 2,048.
- **`0x802C3D80` signature corrected.** Its `bl 0x802DCD54` is the middle of
  `_savegpr_19`, so Ghidra's `void(void)` is wrong: the body is
  `(r3 owner, r4 mode)`, proven by `mr r21,r3` / `mr r19,r4` then `cmpi r19,0`
  and `cmpi r19,2`.
- **Untabled code found again.** `0x802C3460`, `0x802C3490` and the thunks
  `0x802C3510`/`0x802C3520` explain the gaps after `0x802C32B0`/`0x802C34B0` but
  have no tsv row.
- `0x802C18C0`: the export's two stack slots for the `r13-0x4744` arm are
  swapped — the `==1` case stores into the **high** byte (`value << 24`), the
  other into the low byte. Followed the disassembly.
- `0x802C3630`: ten `sdata2` words are `lfs` floats, not integers —
  `30.0, 20.0, 1.6, 0.0, 0.4, 0.2, -0.0075, 10.0, 15.0, 1.0` (bit-exact).
- `0x802C0E30`'s `r8`/`r9` are live across the `0x802C05E0` call, which reads
  them as its sixth/seventh argument.
- `0x802C13F0` (medium): the `0x802C1650..0x802C1694` block is four
  `ps_merge00` (`+ psq_l`/`psq_st`) whose only surviving stores zero the two
  neighbour `y` components before the `0x801C1530` cross product.
- `0x802C3000`'s third field is a `u8` (`lbzx`), not a word.
- `0x802C17E0`'s `(clz >> 3) & 0x1FFFFFFC` index is 4 iff the scale is zero,
  else 0.
- Three bodies drop a callee argument that the disassembly restores
  (`0x802B9280`, `0x802C2700`, `0x802C0B30`).
- `0x802C05E0`'s `bl 0x802DCD64` is `__savegprlr_23`, so the export's `u64`
  return is bogus: `r3` is the mode, `r4` the destination.

## Merge note

Four lane rows had unquoted commas inside their evidence prose (8/10-field rows).
They were repaired to seven fields with `; ` separators, and the four malformed
ledger rows were dropped and re-merged from the corrected CSVs.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-lane-csv.mjs wave219`: 60 rows, 60 unique, 50 high + 10 medium.
- Ledger: 19,978 rows (98.65%); `resource::archive` 609/747.
- `scripts/check.ps1` exit 0.

## Next

138 `resource::archive` bodies, then `src/game/stage.rs` (132), the two
`ui::hud` stragglers and the untabled bodies found in dsys/archive.
