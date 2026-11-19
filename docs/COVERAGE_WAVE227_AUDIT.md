# Audit - Wave 227 (size triage, pointer tables, decoder, NaN)

## 1. All 46 size mismatches adjudicated: the docs were wrong, never the tsv

Waves 3, 7, 31, 48, 55, 62, 65, 66, 68, 74, 75, 83, 88, 90, 92, 95, 98, 104,
114, 116, 123, 125, 128, 160, 166, 168, 192 and 203 carried 46 rows whose
`# Original` byte count disagreed with `functions.tsv`. In **every** case the
measured `prologue -> closing transfer` extent equalled the tsv size, so the
doc was corrected and each block gained an `Extent:` line with the real address
range and closing instruction.

No Rust body text changed, so this wave carries no behaviour risk.

Why the errors clustered: several bodies make **indirect calls**
(`mtctr` + `bcctr` with `LK=1`), whose continuation instructions still belong
to the body. A naive "stop at the first `bcctr`" scan truncates them -
`0x80A19A70` was documented as 92 bytes of its real 352. Two other rows were
over-counts that included padding into the next function.

Notable corrections: `0x80A39DE0` 108 -> 308, `0x8021FD44` 64 -> 224,
`0x80761040` 220 -> 544, `0x80A19A70` 92 -> 352, `0x80219BE8` 196 -> 380.

## 2. Pointer-table backlog inventoried

`docs/UNTABLED_POINTER_TABLES.md` accounts for all 2,457 entries found in wave
226:

| Bucket | Count |
|---|---:|
| Owned by a pointer run (826 runs; 494 tables >= 3 words) | 1,950 |
| Call-site only (72 gaps) | 92 |
| Unreferenced | 415 |

487,260 bytes of owned bodies, split into 12 high-reachability / 18 medium /
651 low-reachability groups. Top groups by reachability then size:

| Table | Entries | Bytes | Role |
|---|---:|---:|---|
| `0x802ED9E0` | 76 | 58,612 | DOL `.ctors` constructor table (181 pointers + 3 zero words) |
| `0x802F8EF0` | 20 | 5,272 | nw4r `scn` class tables (`ScnObj`/`ScnLeaf`/`ScnGroup`/`ScnRoot`) |
| `0x80343E90` | 9 | 4,840 | WPAD/KPAD extension callbacks |
| `0x80314534` | 22 | 3,584 | `game::stage` object vtable (68 slots) |
| `0x80310DB0` | 3 | 1,744 | collision-area vtable tail |
| `0x803280E0` | 15 | 1,112 | SDA state tables (`Pausewindow_c::StateID_*`) |
| `0x802F8510` | 6 | 804 | nw4r animation curve/kernel table |
| `0x802EE830` | 1 | 696 | enemy-66 record slot -> `0x80095950` |
| `0x802EF328` | 1 | 400 | player-cluster table -> `0x80064880` |
| `0x802F6D30` | 3 | 204 | HID-host method table |

Largest *unreferenced* payloads (state tables whose loader is not yet
reconstructed): `0x80325428` (83 entries, 21,596 B, world-map `Wm`),
`0x80314068` (59, 12,788 B, `dEn_c` base state), `0x803084D0` (44, 11,352 B,
player state), `0x8032679C` (64, 11,088 B, `Wm`/Yoshi).

Documented discrepancy (repaired in wave 237): `src/runtime/os.rs` used to say the
ctor table has "185 words, 184 code pointers"; the ELF has 181 pointers ending
`0x802EDCB0`, plus three zero words at `0x802EDCB4..0x802EDCC0`.

## 3. Decoder completed for condition-register and indexed float ops

`ppc159.js` now decodes the XL-form CR group (`cror`, `crxor`, `creqv`,
`crandc`, `crnand`, `crand`, `crorc`, `crnor`) and `lfsx`/`lfdx`/`stfsx`/
`stfdx`. That renames 2,639 words in the image (997 `cror`, 774 `crxor`,
12 `creqv`, 488 `lfsx`, 349 `stfsx`, 30 `lfdx`, 1 `stfdx`). Cross-checks: the
`0x4CC63182` `crxor 6,6,6` varargs idiom, `x4` scaling for `lfsx`/`stfsx`
versus `x8` for `lfdx`/`stfdx`, and 29 pre-existing doc lines naming `lfsx`.
No committed semantics change; two stale prose mentions of `xl449` remain.

## 4. NaN/unordered audit: 9 real fidelity fixes

The hardware sequence materialises **1 iff `LT|UN`** (`cror 2,1,2` merges
`GT|EQ`, then `cntlzw`), i.e. true for NaN, where Rust `<` gives 0. Of 3,984
compare sites, 21 materialise the flag with `cntlzw`. Nine sites in eight
bodies were rewritten to `!(a >= b)`, which is identical for ordered inputs:

`fn_80033f10`, `fn_8002fa50`, `fn_800a0550`, `fn_8011f700`, `fn_801214a0`
(twice) and `fn_8011e020`/`fn_8011e1a0`/`fn_8011e420`.

Already exact: `fn_8003acd0`, `fn_80031b70`, `fn_80033a30`, and
`fn_8011f700`'s `+0xEC <= 0` gate.

Found and documented only (not rewritten, because the change would not be
order-preserving):

- `fn_8002d480` (`src/ui/hud.rs:19843`): hardware is `LT|UN` but the Rust is
  `v > 0.0` - inverted for ordered inputs too.
- `fn_8011ea00` (`src/game/worldmap.rs:37252`): operand roles look swapped; the
  NaN semantics cannot be settled without re-deriving the mapping.
- 333 further branch-form bodies have their raw taken-sets recorded in
  `work/wave227/laneF_branch_class.tsv` for a later pass.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-lane-csv` reports **0** byte-size mismatches for all 28 touched waves.
- Ledger unchanged at 20,250 rows (99.99%), 0 duplicates.
- `scripts/check.ps1` exit 0.
