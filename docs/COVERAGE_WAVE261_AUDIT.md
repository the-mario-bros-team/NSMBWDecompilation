# Audit - Wave 261 (ranked backlog, first four runs)

## 1. The four ranked runs are all vtables, not dispatch tables

| Run | Slots | Shape | Result |
|---|---:|---|---|
| `0x80306400` | 193 | vtable: 193 distinct targets, no interior labels, `togezo` scene builder at slot 0 | 10 bodies; **53** uncovered |
| `0x80303104` | 187 | method array of `daEnDpakkunBase_c`; generations at `0x80303A9C`/`0x80303F48` share words | 10 bodies; prologue **31 -> 21** |
| `0x8030241c` | 199 | C++ vtable; **`d_enemiesNP.rel` has 10 relocations naming the header** | 10 bodies; **50** uncovered |
| `0x80303aa4` | 183 | vtable, twin of `0x80303F50` (182 words, 166 shared) | 10 bodies; prologue **25 -> 6** |

This corrects the worst case for planning: these are **not** jump tables - the run
slot count is genuinely the method count. Only `0x803218ec` and its relatives were
dispatch tables.

`0x80306400` has **no installer in any image** (200-instruction dataflow window
plus raw-word scan), so it is an uninstalled vtable - logged as a reference gap.

## 2. Case arms are bodies

The 20 arms `0x802DDF44..0x802DE10C` are twenty contiguous 24-byte frames, each
building a different ten-power text pointer (`lis`, `li`, two `addi`,
`bl 0x802DDE04`, `b`). They are interior labels only in Ghidra's sense, so all 20
were translated and the band closed at **82/82**.

## 3. Body-level bug fixed

The shipped decimal-formatter default arm uses the truncating signed divide idiom
(`rlwinm`/`add`/`srawi`), so `fn_802de124` must use Rust `/ 2`, not `>> 1`:
`-63` yields `-31`, where an arithmetic shift gives `-32`. Fixed and verified.

## 4. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | `0x80306400` first 10 | 10 | 2,072 |
| B | `0x80303104` first 10 | 10 | 1,684 |
| C | `0x8030241c` first 10 | 10 | 2,296 |
| D | `0x80303aa4` next 10 | 10 | 2,188 |
| E | 20 decimal-formatter case arms | 20 | 480 |

Lane D's note on `0x80169F80` is worth keeping: its translated sibling documents a
`(u64, object, descriptor)` ABI, but this call site passes `f1 = 0.0` plus
`r3`/`r4 = +0x544`/`+0x584`, so the address is called with the shipped shape.

## 5. Evidence audit (lane F)

28 table words/slots, 31 extents, 28 padding sides, untabled status, ownership,
23 dispatcher counts, the `dEnBoss_c` final census (265/265, 265 unique, by-file
133/77/24/24/7) and the new band all reproduce. **Seven doc fixes**, plus the
`fn_802de124` body bug above. Notably two claims were `main.dol`-only:
`d_en_bossNP.elf` branches to slots 237/240-243/252, and REL boss vtables repeat
ten of the thirteen words.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations.
- `check-dupe-bodies` exit 0 (no duplicates from the lane B/D overlap: lane D
  referenced the nine bodies lane B had already landed).
- `scripts/check.ps1` exit 0.
