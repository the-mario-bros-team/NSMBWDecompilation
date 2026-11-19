# Audit - Wave 251 (inventory invariant, three vtables complete)

## 1. Inventory invariant armed

`tools/verify-ledger.mjs` now treats the Ghidra exports as an explicit
inventory and fails hard on:

1. a missing `functions.tsv` (previously silently skipped, shrinking the known
   set);
2. a ledger address absent from every tsv - the `0x8004AAC0` class, which used
   to be printed but not fail;
3. `rows > inventoryAddresses` (`inventory.headroom` negative).

Gate-clean tree: 20,250 rows / 20,250 unique / 0 duplicates / 0 outside
inventory, inventory 20,252, headroom 2. Negative tests: appending an
`8004aac0` row exits 1 with `rowsOutsideInventory: 1`; deleting an export exits 1
with headroom -769. `scripts/check.ps1` already passes `--enforce-bodies`, so the
rule is live.

The headroom of 2 is exactly the two documented deferrals (`0x80697090` 248 B,
`0x806E2BF0` 96 B) - 344 bytes of code the ledger never accounted for, now
visible in every gate run.

## 2. Three vtables complete

| Table | Slots | Result |
|---|---:|---|
| `WM_PLAYER_VTABLE` `0x8032679C` | 311 | **0 uncovered** (302 pointers, 293 unique targets, 256 untabled bodies all present) |
| player-class `0x803084C8` | 275 | complete (wave 247) |
| WorldMap actor `0x80325420` | 278 | complete (wave 247) |

`WM_PLAYER_VTABLE` owners: 204 `worldmap`, 59 `player`, 21 `enemies`,
6 `display`, 2 `yoshi`, 1 `interaction`.

Scanner trap documented: slots 0/16 are untabled but translated as
`wm_player_enter`/`wm_player_destroy`, so an `fn_<hex>`-only scan reports them
missing.

## 3. Dispatcher helper family verified

All six exercised helpers **pass** an independent register-window check against
the original sequences:

| helper | sites | verdict |
|---|---:|---|
| `interface_call2` | 21 | pass |
| `interface_call3` | 44 | pass |
| `interface_call_desc2` | 3 | pass |
| `interface_call_desc3` | 1 | pass |
| `interface_call_desc5` | 2 | pass |
| `interface_call_f32_1` | 1 | pass |

Findings:

- `interface_call4`, `interface_call5`, `interface_call_desc4` have **no call
  sites**. `interface_call5`'s doc names the wrong helper (the two predicates it
  cites call `interface_call_desc5`).
- `interface_call4` has one real shipped shape it should own: `0x80746810` in
  `fn_80746640` (`d_basesNP`), `r3..r6 = (actor, &probe, name, 50)`, descriptor
  `0x807CE410 + 12*index`, currently hand-resolved.
- Family risk: `interface_callN(interface, object, …)` and
  `interface_call_descN(descriptor, object, …)` take their first two arguments
  in **opposite roles**, and mixing them still compiles.

Sweep: 1,574 `bl 0x802DCBFC` sites across all five images; only six set
`r6`/`r7`.

## 4. Bodies and page-offset fix

| Lane | Work | Result |
|---|---|---|
| C | `dEn_c` 226-236 | 10 bodies, 1,600 B |
| D | `0x803502F8` band 0x80350678+ | 10 bodies, ~316 B |
| E | `dAcPy_c` record bank | 2 stubs; **111/111 handler words complete** |

Lane C caught another `addi` sign-extension error:
`SIDE_TABLE_8043_C1A0_C244` was `0x8043C1A0`, but
`lis r4,0x8043` + `addi -15968` gives **`0x8042C1A0`**. Renamed and corrected;
it matches `stage::W69_SIDE_TABLE` and wave 235's constant, and the image holds
`{1,-1,0,0,32,...}` there.

Lane E corrected the record base of the `dAcPy_c` bank to `0x80325948` (150
records of `{0, key, value}`, 50 states x 3).

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- Ledger 20,250 rows: **20,226 translated / 22 partial / 2 stub**.
- `scripts/check.ps1` exit 0.
