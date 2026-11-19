# Audit - Wave 264 (eleventh vtable, standing page-offset gate)

## 1. `0x80304e18` complete: 186/186

The last 16 words were fifteen `bclr` stubs plus the leaf `0x80034720` (which
flips the `+0x348` side byte and zeroes `+0xE8`). Pre-scan confirmed none of the
16 already had a body, table row or ledger row.

**Eleven vtables complete**: 275, 278, 311, 45, 306, 235, 265, 183, 182, 187,
186.

## 2. `0x80304940` decoded: `daEnJimenPakkunBase_c`

- span `0x80304940..0x80304C28`, **186 slots**, all distinct, none null;
- header `0x80304938` (`{0,0}`), the shared **base-class** table;
- installed by three `d_enemiesNP` constructors via
  `lis r4,0x8030` / `addi r4,r4,0x4938` / `stw r4,0x60(...)`
  (`0x80AB32B4`, `0x80ABD154`, `0x80ADEB1C`), each of which then overwrites
  `+0x60` with its own derived REL vtable;
- identity from the adjacent `daEnJimenPakkunBase_c::StateID_DieOther/
  DieIceBreak/Attack/SakasaAttack` strings.

Verified one install site here: `0x80AB32B4..0x80AB32C0` is
`lis r4,0x8030` / `addi r4,r4,18744` / `stw r4,0x60(r30)`.

Coverage after the lane: 153/186 (33 uncovered, 10 prologue left).

## 3. New standing gate tool: `tools/check-page-offsets.mjs`

The `addi`-as-`ori` sign-extension error has now appeared six times
(`0x8043B840`, `0x8030D198`, `0x8030D1B0`, `0x8030D240`, `0x803535EC`,
`0x8043C1B0`). It is invisible to the compiler and to the evidence audits, so it
is now a tool rather than a per-wave script.

The tool re-derives each documented `lis page` + `addi reg,reg,imm` (or
`lwzu`) pair and compares the result with the declared constant. It skips pairs
where the doc already names the resolved base, so documented field offsets
(`0x80359470 + 0xC4 -> 0x80359534`) are not flagged. Wired into
`scripts/check.ps1` after the duplicate-bodies check.

First run: 26 documented pairs, 11 with a negative immediate, **1 real
mis-decode**:

| Constant | Was | Correct |
|---|---|---|
| `E252_FLOAT_TABLE_8043_C1B0` | `0x8043C1B0` | **`0x8042C1B0`** |

`lis r4,0x8043` + `addi r4,r4,-15952` is `0x80430000 - 0x3E50 = 0x8042C1B0`;
the constant's own doc already claimed the sibling read it there. The image
holds `1.5f` at `+0x00` and `-1.5f` at `+0x04`, so the doc's "image holds
zeros" line was wrong too. Renamed, corrected, four readers updated.

## 4. Tables in progress

| Table | State |
|---|---|
| `0x80306400` (`daEnTogezo_c`, uninstalled) | 177/193, **16 leaf slots, 0 prologue** |
| `0x8030241c` | 185/199, **14 leaves, 0 prologue** |
| `0x80304940` (`daEnJimenPakkunBase_c`) | 153/186, 10 prologue left |
| `0x80305958` (`daEnObjCoinBlock_c`) | 178/190, **12 leaves, 0 prologue** |

`0x80305958` decoded as a 190-slot vtable headed `0x80305950`, installed at
class `+0x60` by constructor `0x80037EF0` (`lis r4,0x8030` / `addi r4,r4,0x5950`
/ `stw r4,0x60(r29)`) and re-installed by `0x80038080`. Identity:
`daEnObjCoinBlock_c`, from the seven `StateID_*` strings and `"revival_shell"`.

Its wave-260 survey numbers were **stale**: 48 uncovered was not reproducible;
at lane start only 22 remained (waves 261-263 had already translated shared
`0x8003xxxx` base-class methods this table also names), and only 3 were
prologue-shaped. Lane E translated those three plus seven more real entries
(812 B). One body, `0x8003A5D0`, has an unreachable float block: its `0x1001D`
guard tests `r0`, which the callee restores to the function's own address.

## 5. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | `0x80306400` next 12 | 12 | ~800 |
| B | `0x8030241c` next 12 | 12 | 428 |
| C | `0x80304e18` last 16 (closes) | 16 | 84 |
| D | `0x80304940` first 10 | 10 | ~1,630 |
| E | `0x80305958` first 10 real entries | 10 | 812 |
| F | padding-gap leaves + constant consolidation | 3 | 36 |

## 6. Findings

- Three functions found in "padding gaps" (`0x800206E0`, `0x800206F0`,
  `0x80025AA0`) are real code; the surrounding zeros are alignment and no table
  word names them.
- `0x80034700` (between slots 178 and 161 of `0x80304e18`) is another such
  untabled 20-byte `+0x7C` node forwarder, left for a leaf pass.
- `0x803535EC` now has one constant owner (`DAT_803535EC_E`) with a
  `// canonical:` note at the retired site.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-dupe-bodies` exit 0 (both passes).
- `check-page-offsets` 0 mismatches.
- `scripts/check.ps1` exit 0.
