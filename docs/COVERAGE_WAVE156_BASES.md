# Coverage - Wave 156 (`game::bases`)

## How to read this file

This page documents the Wave 156 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL (REL module 2). Every address is a **static
analysis address** under the canonical synthetic base **`0x80600000`**; the Wii
REL loader chooses the real load address, so these numbers identify positions
inside the REL image, not runtime addresses. Every `# Original` block in the
source carries the line
`Synthetic base: 0x80600000; this is not a runtime address.`

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee contract, a field role or a table meaning had
to be named rather than resolved.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 17 | 5,660 | 11 | 6 |

All 17 rows are `translated`; none is `partial`. The module header now reports
3,799 bodies (3,782 before this pass + 17). Eight rows are the "ready" rows of
`docs/SPEC_WAVE155_BASES.md` (4,648 bytes); the other nine are `0x8064`-bucket
bodies (1,012 bytes), which takes that planner bucket from 68 to 59 untranslated
addresses.

## What this pass covers

- **The eight "ready" rows of `docs/SPEC_WAVE155_BASES.md`**, the
  584-to-576-byte band:

  | cluster | body |
  |---|---|
  | `daIntermittent_c` Splush | `0x80695E00`: clip commit, three effect emits, SE `0x308`, climb integration, `0.95` crossing rules, two-state phase word |
  | `CS_W5_Ibara` loader | `0x8078F880`: allocator, model, five-way `AB/AC/BC/CA/CB` clip select, clip plus texture-SRT bind, two rate pairs |
  | world `RouteInfo` reader | `0x807BAC40`: `W%X/pointW%X.csv` then the `a`/`b`/`c` series, token classification and the two per-world flag tables |
  | world-select state | `0x807BBE20`: ten unlock bytes, pane-row spacing, per-pane state words, `StateID_OpenAnimeEndWait` dispatch |
  | `hanachan` loader | `0x807649B0`: four body segments at rates `4/8/12/16`, head at `0.0`, then the blend wrappers |
  | world-map ribbon create | `0x807BE750`: parameter nibbles, state plus layout, `0x803504E0` transform seed, ortho cameras 5 and 6 |
  | `cobBoard` loader | `0x8075C6F0`: `ScnMdl` cast against `0x802F8FF0`, model option `0x30001`, `group1..group8` walk |
  | world-map dispatcher | `0x807C22C0`: eleven request bytes in priority order, eight state records, four-player resync |

- **Nine `0x8064`-bucket bodies** (1,012 bytes):

  | cluster | body |
  |---|---|
  | treasure-box effects | `0x8064CF50` (`Wm_ob_treasureboxwait`/`treasureboxtail`, `8.0` lift or `4.0` in mode 4) and its per-frame caller `0x8064D0A0` |
  | cursor grid extents | `0x8064DE60`: two `|high - low| / step` truncations from the `0x807C8678` `{0.5, 1.0}` table |
  | release wrappers | `0x80649FD0`, `0x8064B040`, `0x8064C220`, `0x8064D460`, `0x8064E470` (destructor `0x8015F800`) and `0x8064B270` (destructor `0x80168D80`), each followed by the conditional `0x802B9280` release |

## Where the binary differed from the spec prose

1. **`0x80695E00`'s `0.95` crossing is an `if`/`else`, not two sibling `if`s.**
   The export and the raw words at `0x80695FC8..0x8069600C` are
   `if (0.95 <= old || new < 0.95) { if (0.95 < old && new <= 0.95) limit = 0; }
   else { limit = 0; }`. For finite heights this is exactly the spec's two
   crossing arms, but a NaN height takes the **else** and writes the limit; the
   body keeps that shape.
2. **`0x807BE750`'s state call takes `screen`.** The export prints
   `FUN_807beaf0()` with no arguments, but `r3` still holds the screen at
   `0x807BE7A8`; the body passes it.
3. **`0x8075C6F0`'s dynamic cast result is used unchecked.** When `+0x1B0` is
   null or the cast fails, the binary still calls slot `+0x20` through the null
   pointer at `0x8075C82C`; the translation keeps the unconditional
   `vmethod2(scn, 0x20, ...)` and documents the quirk instead of adding a guard.

Confirmations rather than corrections: `0x807BAC40`'s `strlen == 14` guard fires
exactly for the no-letter name (`W1/pointW1.csv` is 14 characters,
`W1/pointW1a.csv` is 15); `0x807BBE20`'s `2 + (unlock != 0)` is the
`neg`/`or`/`rlwinm` idiom that gives `2` locked and `3` unlocked; `0x807649B0`
re-runs the model factory on every part iteration and the rate is
`(i + 1) * 4.0f`, not a table read; `0x8078F880` keeps the discarded second
`0x8023A270` call at `0x8078FA14`; `0x807BE750`'s `+0x228`/`+0x338` are `stfs`
of `20000.0f`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80649FD0` | `fn_80649fd0` | 88 | high |
| `0x8064B040` | `fn_8064b040` | 88 | high |
| `0x8064B270` | `fn_8064b270` | 88 | high |
| `0x8064C220` | `fn_8064c220` | 88 | high |
| `0x8064CF50` | `fn_8064cf50` | 216 | high |
| `0x8064D0A0` | `fn_8064d0a0` | 136 | high |
| `0x8064D460` | `fn_8064d460` | 88 | high |
| `0x8064DE60` | `fn_8064de60` | 132 | high |
| `0x8064E470` | `fn_8064e470` | 88 | high |
| `0x80695E00` | `fn_80695e00` | 584 | high |
| `0x8075C6F0` | `fn_8075c6f0` | 576 | medium |
| `0x807649B0` | `fn_807649b0` | 580 | medium |
| `0x8078F880` | `fn_8078f880` | 584 | medium |
| `0x807BAC40` | `fn_807bac40` | 584 | medium |
| `0x807BBE20` | `fn_807bbe20` | 584 | high |
| `0x807BE750` | `fn_807be750` | 580 | medium |
| `0x807C22C0` | `fn_807c22c0` | 576 | medium |

**Totals:** 17 bodies, 5,660 bytes. The address window runs from `0x80649FD0` to
`0x807C22C0`; the bucket subset is `0x80649FD0..0x8064E470`.

## Notes on inference

- `0x8078F880` and `0x8075C6F0` are `medium`: every string, float and RTTI
  record is read, but the `0x80069xxx`/`0x800F2Axx` helpers are thin unledgered
  tail calls, `0x8016A3A0`'s kind word is named by its shape, and
  `0x8000D660`'s selector consumer stays opaque.
- `0x807BAC40` is `medium`: four callees are address-only (`0x800F9970`,
  `0x801026D0`, `0x80102730`, `0x807BAE90`); all four are decoded in
  `docs/SPEC_WAVE155_BASES.md` section 3.1.
- `0x807649B0` is `medium`: `0x80169D90`, `0x800F2A30` and the 272-byte
  `0x80765980` are address-only; the last one is quoted in the spec and is
  called with `abi_call1!` rather than inlined.
- `0x807BE750` is `medium`: `0x807BEAF0` (ten-way state dispatch) and
  `0x807C1220`/`0x807C1380`/`0x80258BD0` stay address-only.
- `0x807C22C0` is `medium`: the eleven request flags are set outside this body
  set and `0x80354E60`/`0x80354E70` are `main.dol` BSS; `0x807C3AF0` and
  `0x807C3A70` are address-only setters.
- `0x80695E00`, `0x807BBE20`, `0x8064CF50`, `0x8064D0A0`, `0x8064DE60` and the
  six release wrappers are `high`: every guard, store and literal is visible in
  the export and the raw words. `0x8015F800`, `0x80168D80` and `0x802B9280` keep
  `// unknown:` comments for their destructor/release roles.

## Deliberate skips

- `0x80648060` (504), `0x80648AA0` (456), `0x8064DCC0` (408) and the rest of
  the `0x8064` bucket's large `dScCrsin_c`-style camera/ground bodies were not
  started: they read `_DAT_80429DD0`/`_DAT_80429E00` sub-fields through many
  unledgered helpers (`0x80070BF0`, `0x800EAA00`, `0x800EBBE0`) whose contracts
  are not pinned, so the field roles could not be stated without guessing.
- `0x80643260` (148) is skipped: its export ends in `halt_baddata()` at the
  final branch, so no complete body can be written from the given evidence.
- `0x80640AC0` (116) is skipped for the same reason (the whole export is
  `halt_baddata()`).
- Nothing was written that the raw words did not support; the three deviations
  above are implemented as the binary behaves.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (17 rows, 5,660 bytes),
  re-read from the TSV for this pass rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 17 were checked against
  the ledger's address column before being written, and the lane validator
  repeats that check.
- `node tools/check-lane-csv.mjs wave156` reports `OK` (17 rows for
  `wave156-bases.csv`, 17 unique addresses in the lane).
- `cargo check --message-format=short` exits 0 and reports no diagnostic naming
  `src/game/bases.rs`.
- All 17 bodies were decoded from the raw words with
  `file_offset = (address - 0x80600000) + 0x1000`; every literal was read out of
  `work/rels/elf/d_basesNP.elf` or `work/dol/main.elf`.
