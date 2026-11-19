# Coverage - Wave 239 (closing verification)

## Gates

| Gate | Result |
|---|---|
| `cargo check` | 0 errors, 26 pre-existing warnings |
| `node tools/doc-lint.mjs src` | ok, 68 files, 0 violations |
| `node tools/merge-ledger.mjs` | 20,250 records from 1,067 lane CSVs |
| `node tools/verify-ledger.mjs` | 20,250 rows / 20,250 unique / 0 duplicates / 0 missing |
| `scripts/check.ps1` | full chain, OK |

## Final inventory

| Source | Translated | Total |
|---|---:|---:|
| main.dol | 10,276 | 10,276 |
| d_profileNP | 36 | 36 |
| d_basesNP | 5,613 | 5,615 |
| d_enemiesNP | 3,555 | 3,555 |
| d_en_bossNP | 770 | 770 |
| **Ledger** | **20,250** | **20,252** |

The two absences are the documented deferrals `0x806E2BF0` (frame-only writes)
and `0x80697090` (reads stack slots it never writes).

Semantic-module reconciliation: `game::bases` holds 5,607 rows, not 5,613,
because six `d_basesNP` bodies are player functions filed in `player.rs`.

Ledger status column: 20,020 translated / 217 partial / 13 stub (labels on
present bodies, not missing code). Confidence: 12,772 high / 7,209 medium /
269 low. Staging: 0.

## Untabled corpus

| Quantity | Count |
|---|---:|
| Original corpus entries | 2,457 |
| With a translated body | 259 |
| Remaining | 2,198 |
| Aliases of other slots | 371 |

`src` also holds 288 non-corpus `main.dol` starts without a standard prologue
and one REL address, for 583 untabled body blocks over 548 addresses.

## Wave-239 repairs

- **`0x806E7F50` had no body** despite a translated ledger row: only a dangling
  doc stub. Reconstructed the 160-byte body from its export, cross-checked
  against `fn_80654880`. Every bases ledger row now resolves.
- `fn_8060180` (7-digit typo for `fn_80600180`) renamed with its three doc links.
- `fn_8011a380`'s `fmuls` read the reserved FRB field instead of FRC, making
  `1.0f`/`0.5f` look dead; the body now writes the real
  `480.0 / 256.0 / 240.0 / 128.0` width and half-extent block.
- `fn_802c6000`'s cited table word corrected to `0x803502C0`.
- Module headers corrected: `dsys` 149 -> 402, `archive` 321 -> 771,
  `bres` 260 -> 261, `boot_scene` 12 -> 29, `player` 537 -> 857.

## Known open items (for any future pass)

1. **2,198 untabled bodies remain**, concentrated in the three large vtables
   (`0x80325420` WorldMap, `0x80326794` WM player, `0x80314068` `dEn_c`) plus
   the `0x803084C8` player class. `docs/UNTABLED_POINTER_TABLES.md` is current.
2. **230 legacy status labels** (217 partial / 13 stub) could be re-adjudicated;
   all have bodies.
3. 269 low-confidence rows could be re-verified; the `fmuls`/`extsb`/`fcmpu`
   operand traps found in waves 227-239 suggest a wider operand audit would pay.
4. The `verify-ledger` check does not assert that a ledgered row has a body;
   the `0x806E7F50` gap slipped through until wave 239. Worth adding.
