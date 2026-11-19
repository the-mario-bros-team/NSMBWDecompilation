# Coverage - Wave 135 (`game::bases`)

## How to read this file

This page documents the Wave 135 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`** produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
`# Original` block in the source carries that warning line. Cross-module cells
(`0x8037xxxx`, `0x8042xxxx`, `0x8080xxxx`) keep their absolute `main.dol`
values; this REL's own `.data` cells keep their `0x807Cxxxx` / `0x807Dxxxx` /
`0x8081xxxx` / `0x8082xxxx` synthetic values.

Every cited file is a real Ghidra C export under
`work/ghidra/rel-d_basesNP/export/`, and every byte count was re-read from
`work/ghidra/rel-d_basesNP/export/functions.tsv` for this pass. Every body's last
word was also re-read from `work/rels/elf/d_basesNP.elf` at
`(address - 0x80600000) + 0x1000` and is `0x4E800020` (`blr`) at
`entry + size - 4`.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee contract, a field role or a `main.dol`
cell's meaning had to be inferred. Comments in the bodies keep
`// observed:` / `// inferred:` / `// unknown:` markers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 25 | 3,876 | 10 | 15 |

All 25 rows are `translated`; none is `partial`. The module header now reports
3437 bodies (3412 + 25).

## What this pass covers

- **The thirteen class-B bodies of `docs/SPEC_WAVE134_BASES.md`
  (2,892 bytes).** All ten "ready" addresses plus both "conditional" ones and
  the tied-13th hand-off:

  | cluster | bodies |
  |---|---|
  | render copy | `0x80608DF0` twelve-record float/orientation copy |
  | pane submit / reset | `0x80642D20` four-pane `+26` submit, `0x80643470` channel reset, `0x8068E440` pane regroup |
  | proximity | `0x8069A6E0` 128-unit 2-D player test |
  | basis rebuild | `0x8069E5F0`, `0x8069F560`, `0x8069F9D0` 3x3 rebuild-and-normalise family |
  | emitters | `0x806B4F90` `Wm_en_landsmoke_s`, `0x80747B30` effect 282 |
  | transform helpers | `0x807268C0` `plp` yaw/scale, `0x80780A90` chain-distance selector, `0x80781B60` mode-selected position copy |

- **Twelve `0x8079` planner-bucket bodies (984 bytes)**, expanding that bucket
  from 95/184 to 107/184 translated addresses:

  | cluster | bodies |
  |---|---|
  | id selectors | `0x807972E0`, `0x8079AF60` state/flag to id |
  | predicates | `0x8079B0C0`, `0x8079B240`, `0x8079B700` |
  | activity probes | `0x8079B120`, `0x8079B160` |
  | table scan | `0x8079BAC0` five-id lookup with world filter |
  | copy / apply | `0x80798CC0` halfword copy, `0x8079B060` state-gated float apply |
  | wrappers | `0x80793E50` guarded bulk call |
  | sound | `0x8079ED20` variant-sound table picker |

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80608DF0` | `fn_80608df0` | 248 | high |
| `0x80642D20` | `fn_80642d20` | 204 | medium |
| `0x80643470` | `fn_80643470` | 228 | high |
| `0x8068E440` | `fn_8068e440` | 228 | medium |
| `0x8069A6E0` | `fn_8069a6e0` | 236 | medium |
| `0x8069E5F0` | `fn_8069e5f0` | 204 | high |
| `0x8069F560` | `fn_8069f560` | 208 | high |
| `0x8069F9D0` | `fn_8069f9d0` | 208 | high |
| `0x806B4F90` | `fn_806b4f90` | 208 | medium |
| `0x807268C0` | `fn_807268c0` | 248 | medium |
| `0x80747B30` | `fn_80747b30` | 224 | medium |
| `0x80780A90` | `fn_80780a90` | 240 | medium |
| `0x80781B60` | `fn_80781b60` | 208 | medium |
| `0x80793E50` | `fn_80793e50` | 64 | medium |
| `0x807972E0` | `fn_807972e0` | 76 | high |
| `0x80798CC0` | `fn_80798cc0` | 68 | medium |
| `0x8079AF60` | `fn_8079af60` | 72 | high |
| `0x8079B060` | `fn_8079b060` | 92 | medium |
| `0x8079B0C0` | `fn_8079b0c0` | 80 | high |
| `0x8079B120` | `fn_8079b120` | 64 | medium |
| `0x8079B160` | `fn_8079b160` | 84 | medium |
| `0x8079B240` | `fn_8079b240` | 28 | high |
| `0x8079B700` | `fn_8079b700` | 28 | high |
| `0x8079BAC0` | `fn_8079bac0` | 160 | medium |
| `0x8079ED20` | `fn_8079ed20` | 168 | medium |

**Totals:** 25 bodies, 3,876 bytes (10 high, 15 medium). The address window runs
from `0x80608DF0` to `0x8079ED20`.

## Where the instruction stream corrects the specification

Two places where the wave-134 prose disagreed with the shipped bytes; the bodies
follow the bytes and say so at the site:

1. **`0x80643470` - the channel loop runs `1..=7`, not `1..=6`.**
   `docs/SPEC_WAVE134_BASES.md` §5 step 2 says "`for (i = 1; i <= 6; i++)`", but
   the shipped words are `li r30,1` at `0x80643498`, `cmpwi r30,7` at
   `0x806434B0` and `ble` (`0x4081FFE8`) back to `0x8064349C`, i.e. seven
   iterations. The Ghidra export's `while (iVar1 < 8)` agrees with the bytes.
   The body emits channels `1..=7`.
2. **`0x8068E440` - the float skip is `!=`, not "EQ or unordered".** The spec's
   step 4 parenthetical claims the `beq` also skips on unordered, but
   `0x8068E4C8` is `fcmpu cr0,f31,f0` and `0x8068E4CC` is `beq` on `cr0.EQ`;
   an unordered compare has `EQ = 0`, so the call is taken. The Rust body writes
   `if value != 0.0`, which reproduces the exact predicate (NaN included).

The bucket bodies were read straight from the export and the raw words; no
specification claim was involved.

## Constant and cross-artifact checks

- `0x807CA0E8` = `16384.0` (`128^2`) and `0x807CA08C` = `0.0` for
  [`fn_8069a6e0`]; `0x807C8350` = `26.0` for [`fn_80642d20`]; `0x807CA1FC` =
  `0.0` for [`fn_8069f560`]/[`fn_8069f9d0`]; `0x807C6730` = `0.0` for
  [`fn_80608df0`]; `0x807CE488` = `128.0` and `0x807CE48C` = `0.5` for
  [`fn_80747b30`]; `0x807CD278` = `1.0` for [`fn_807268c0`].
- `0x8042C1A0` is shipped in `main.elf` PH11 and reads `01 FF 00 00`, i.e. the
  signed-byte table `{+1, -1}` used by [`fn_80747b30`].
- `0x807D00B8` is shipped in this REL and holds the five halfword ids
  `{655, 658, 656, 657, 659}` scanned by [`fn_8079bac0`].
- `0x807D0218` and `0x807D0228` hold `0x2EB` at their first entries, which is
  the sentinel that suppresses the spawn in [`fn_8079ed20`].
- `0x8042A18C`, `0x80429E00`, `0x80429F7C`, `0x8042A1A0`, `0x8042A24D` and
  `0x8042A4A8` are `main.dol` runtime cells: every body reads them at run time
  and cites them as addresses, never as literals.

## Deliberate skips

Read and rejected, with the reason:

- **`halt_baddata` exports.** `0x80794BF0`, `0x8079B110`, `0x8079B1C0` and
  `0x8079F4B0` truncate on bad instruction data (jump tables or embedded data),
  so no honest body can be reconstructed from the export alone.
- **Pure thunks and stubs.** `0x80792A10` and `0x8079EF00` are four-byte `blr`
  bodies and `0x8079EF10` is `li r3,1` / `blr`; they carry no reconstructible
  behaviour beyond `return`/`return 1`, so they were left for an
  address-for-address pass instead of padding this lane.
- **`0x8079B700`'s sibling pair `0x8079B240`** was landed, but the larger
  `0x8079C330` (4,232 bytes) and `0x8079D3C0` (3,176 bytes) tables were not
  attempted: each needs its own call graph and several depend on the
  `0x802DCD70` save-helper `undefined8` return model.
- **The rest of the `0x8079` bucket** (77 remaining addresses, from
  `0x80790280` upward) is untouched. Several are class-C `qr3` bodies; the
  others were simply not decoded within this lane's budget.
- **`0x8069F9D0` is not a duplicate skip.** It is byte-identical to
  `0x8069F560` modulo four relocated `bl` words, so it was landed as a one-line
  delegation with the word-level comparison recorded in its doc block.

## Notes on inference

- [`fn_80793e50`], [`fn_80798cc0`], [`fn_8079b120`], [`fn_8079b160`],
  [`fn_8079bac0`], [`fn_8079b060`] and [`fn_8079ed20`] are `medium`: every call
  target, offset and immediate is literal, but a callee contract or a table's
  meaning is not described by this REL.
- [`fn_80642d20`], [`fn_806b4f90`], [`fn_8069e5f0`] and [`fn_8068e440`] are
  `medium` for one virtual slot each (`0xB0`, `0x2AC` and `0x10`), whose
  contract is not in the artifact; the call shape and every argument register
  are literal.
- [`fn_80747b30`] is `medium`: the spawn variant and the deliberate `pos[0]`
  height override are read from the instruction order, not from a symbol.
- [`fn_8069a6e0`] is `medium`: the three PS words
  (`0x10000032`/`0x103E07BA`/`0x10210014`) are folded to `dx*dx + dy*dy` exactly
  as Ghidra prints them; the strict Gekko lane convention is not re-derived.
- The `unaff_GQR0` blocks at the head and tail of several exports are the `qr0`
  `f31`/`f30` prologue spill and epilogue reload pairs described in
  `docs/REL_ELF_MAPPING.md`; no body in this pass uses `qr3` and no body
  contains an `mtspr` to a `GQRn`.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (25 rows, 3,876 bytes),
  re-read from the TSV rather than from prose; every body's last word was
  re-read from `work/rels/elf/d_basesNP.elf` at
  `(address - 0x80600000) + 0x1000` and is `0x4E800020`.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 25 were checked against
  the ledger's address column before being written, and none is duplicated.
- `node tools/check-lane-csv.mjs wave135` reports `OK`;
  `wave135-bases.csv` has 25 rows and 25 unique addresses.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/game/bases.rs`.
