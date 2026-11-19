# Audit - Wave 238 (batch 11, overlap merge, final accounting)

## Bodies translated (54)

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| B | `WM_PLAYER_VTABLE` 121-135 | 12 | 2,124 |
| C | WorldMap vtable 136-145 | 10 | 252 |
| D | `dEn_c` 90-102 | 10 | 244 |
| E | `0x80350230` family 11-18 + derived | 10 | 364 |

## Real bug fixed: `stage::fn_80098ad0`

The wave-237 asymmetry between `dEn_c` slot 82 and `stage::fn_80098ad0` was
settled from the image: both descriptors are genuinely different
(`0x803585C4` vs `0x803576B4`), but **both** pass `this + 0x394` itself, because
`lwzu r12,r3,0x394` leaves `r3 = this+0x394` for the `bcctr`. The `dEn_c` body
was right; `stage::fn_80098ad0` passed the stored word and is now fixed.

## Vtable overlap encoded

`WM_PLAYER_VTABLE` and the WorldMap run share targets: they diverge at slots
121-123 and agree from 124 onward. Wave 238 lane B's batch therefore also
covered WorldMap 127-135, and lane C correctly started at 136 rather than
duplicating. The inventory now records one merged row (`527` unique words,
`147` entries, `32,684` B) with `Overlap:` ranges on both run sections, so the
next wave does not have to rediscover it.

## `0x80350230` structure corrected again

Wave 237 read four sub-vtables with six zero words. Wave 238 showed the zeros
are **two more two-word headers**: `0x803502A8` is a derived class's primary
vtable (constructor `0x802C56D0` stores it at `+0x04`, with `0x803502B4` at
`+0x10` and `0x803502C4` at `+0x14`). The band is **7 sub-vtables / 36 slots in
50 words**, and the base fourth ends at `0x803502A8`, not `0x803502F8`.

## Untabled accounting (recomputed)

| Quantity | Count |
|---|---:|
| Original corpus entries | 2,457 |
| With a translated body | **259** |
| ... exact `fn_<address>` | 251 |
| ... suffixed/semantic | 8 |
| Still without a body | **2,198** |
| Aliases of other slots | 371 (311 among bodyless entries) |
| Named by any `src` literal (upper bound) | 628 |

The count closes: `259 + 2,198 = 2,457`. Aliases are slot reuse, not a third
bucket - one body serves every slot that names it. Separately, `src` holds 583
untabled body blocks over 548 distinct addresses: 259 corpus entries, 288 other
`main.dol` starts without a standard prologue, and one REL address.

## Other findings

- `dEn_c` slots 99/102 are a byte-identical tail-call pair; slot 101 is the
  table's first `"Wm_mr_yoshistep"` spawn, same request shape as
  `player::fn_80065660`.
- `dEn_c` reachability is weaker from slot 90 on: only 3 of 10 have
  `lwz r12,r12+off` dispatch sites; the rest are documented as table-only
  reconstructions.
- `0x8004FFA0..0x80050290` is 756 B (wave 237 said 752).
- Nine WorldMap tail calls share callee `0x8004E290` with distinct interface
  records and modes.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `scripts/check.ps1` exit 0; ledger 20,250 rows, 0 staging.
