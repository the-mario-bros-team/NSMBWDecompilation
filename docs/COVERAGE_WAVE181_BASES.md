# Coverage - Wave 181 (`game::bases` sub-300 sweep, part 1)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 40 | 3,944 | 37 | 3 |

First wave of the sub-300-byte sweep, 8 bodies per lane. The 300-512 band is
already complete, so this wave begins on the 64-128 byte helpers and
destructors.

## Composition

Of the 40 bodies, **16 are deleting destructors** of the established A-type
family (`0x8015F9F0(this + 0x18, 0)` then `0x802B9280(this)` when `free > 0`,
returning `this`). They match the existing `fn_806357e0` / `fn_80632180`
shapes exactly. The 96-byte vs 100-byte split is purely a duplicated
`cmpi cr0,3,r3,0`; there is no semantic difference.

The rest are small state-machine steps:

- **Layout gates (5).** `0x8061B820`, `0x8060C840`, `0x8060C960`,
  `0x8061B720` and `0x8060DB40` poll `0x800C9610(this + 0x70, -1) == 0` and
  dispatch a BSS descriptor through the `+0x330` wrapper slot `+0x18`.
- **Animation switchers (7).** `0x80609860`, `0x806098E0`, `0x8060DBB0`,
  `0x8060DC10`, `0x806135F0`, `0x806138F0`, `0x80613960`,
  `0x80624790`, `0x80624930` start clips and dispatch `NowDisp` / `Exit` /
  `keyWait` / `StopEndWait` / `ExitAnimeEndWait`.
- **Pane and text helpers (4).** `0x80615D90`, `0x806099A0`, `0x80613180`,
  `0x8060F670`.
- **Frame updates (3).** `0x80609370`, `0x8060C5F0`, `0x8061B820`.

## Corrections and cross-checks

- `0x80615D90`'s export `halt_baddata`s after its two stores; raw-word decoding
  recovered the animation tables `0x807C6C28 = [13,14,15]` and
  `0x807C6C34 = [10,11,12]` and corrected Ghidra's inverted `if`/`else`. Three
  already-translated callers (`fn_806178d0`, `fn_806187b0`, `fn_80618a60`) pin
  the `(screen, index, mode)` argument order.
- The BSS descriptors (`0x808229D8`, `0x80822A18`, `0x80823200/240/280`,
  `0x80823600/680`, `0x80823810`, `0x808236D0`, `0x80823DF0/3E70/3EB0`,
  `0x80824B80/4BC0`) sit past the file-backed end (`0x808228B8`), so they are
  passed as literals and never read. Their roles were recovered from the
  `.rodata` name words (`dCollectionCoinDate_c::StateID_NowDisp`,
  `dCourseClear_c::StateID_Disp`, ...).
- `+0x78` layout vtables use `read_u32(this + 0x78)` + `abi_call1!`, not
  `vmethod1(this + 0x70, ...)`.
- `0x8060DBB0`, `0x806138F0` and `0x80613960` are `medium` only because the
  `0x801D_FAC0/FAD0/FB30` callees are four-byte `main.dol` thunks with
  unresolved roles.
- Branch polarity was re-derived from encodings (`bc 4,2` = `bne`,
  `bc 4,1` = `ble`) and confirms Ghidra on every guard in this wave.

## Verification

- `cargo check` succeeds with 4,369 public bodies and no duplicate names.
- `node tools/check-lane-csv.mjs wave181`: 40 rows, 40 unique addresses,
  37 high + 3 medium.
- All 40 evidence exports exist and every size matches `functions.tsv`
  (3,944 bytes total).
