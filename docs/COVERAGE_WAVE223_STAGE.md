# Coverage - Wave 223 (`game::stage`, main.dol part 1)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 60 | 10,868 | 47 | 13 |

First 60 `game::stage` bodies, `0x800A02D0`-`0x800A7420`, byte-balanced across
lanes (1,708-2,116 B each). The module is now **1154 / 1226**.

## What landed

- **Variant records and Jr model (lane A).** `0x800A1190`, `0x800A1830`,
  `0x800A1980`, `0x800A2BE0`, the 812-byte `0x800A3100`, `0x800A42B0`,
  `0x800A47B0`, `0x800A7250`, `0x800A73F0`, `0x800A7420`.
- **Destructor and ramps (lane B).** `0x800A0EA0`, `0x800A2670`, `0x800A2840`,
  `0x800A2BD0`, `0x800A2F40`, `0x800A4740`, `0x800A4AC0`, `0x800A4C10`, the
  532-byte `0x800A5210`, `0x800A62F0`.
- **Hit records and spin (lane C).** `0x800A0700`, `0x800A0DC0`, `0x800A1C80`,
  `0x800A2530`, `0x800A2810`, `0x800A2BC0`, `0x800A3060`, `0x800A4990`,
  `0x800A4DF0`, `0x800A65E0`.
- **Proximity and height modes (lane D).** `0x800A02D0`, `0x800A0550`,
  `0x800A0A30`, `0x800A0C70`, `0x800A12A0`, `0x800A2BB0`, `0x800A4270`,
  `0x800A43E0`, `0x800A5440`, `0x800A7110`.
- **State probes (lane E).** `0x800A0FB0`, `0x800A2870`, `0x800A2B20`,
  `0x800A2B70`, `0x800A2BA0`, `0x800A2EA0`, `0x800A3AE0`, `0x800A4B20`,
  `0x800A4C40`, `0x800A7330`.
- **Actor init and yaw (lane F).** `0x800A1370`, `0x800A16A0`, `0x800A2750`,
  `0x800A29C0`, `0x800A2A20`, `0x800A2B80`, `0x800A2B90`, `0x800A3D30`,
  `0x800A4150`, `0x800A4480`.

## Corrections and recoveries

- **`bc 12,1` is branch-on-`CR0[GT]`** (BI 1, not LT), so `0x800A5210`'s state 0
  advances on `timer-1 <= 0`.
- **Height polarity** (`0x800A0550`): `fcmpu` + `cror cr0[EQ],cr0[GT],cr0[EQ]`
  gives `height >= X`, and both the `bc 4,2` and the `cntlzw`+`rlwinm 27,24,31`
  fold invert it, so the stored flag is `height < X`.
- **`0x800A2870`'s probe chain is inverted relative to the export**: all three
  probes non-zero returns `0` immediately. Its `bl 0x802DCD70` is `_savegpr_26`,
  so the real signature is `(context, actor, mode) -> u32`.
- `0x800A2A20`'s `FUN_802dcd74()` is `_savegpr` (mirror `_restgpr` =
  `0x802DCDC0`); the export's `uVar6 >> 0x20` is bogus.
- `0x800A4150`'s divisions are `fdivs`, not the widened `double` divide Ghidra
  prints.
- `0x800A7250` has the corrected `fn_8001D850` shape: four pops, a fifth test,
  then recursion **on the same child**.
- `0x800A4270`'s export drops the `r4 = param_1` argument that `0x800A42B0`
  reads at `[r4+0x60]+0x2F8`.
- `0x800A43E0`'s `xoris 0x8000` + `lfd 0x43300000…`/`fsubs` chain is the
  signed-short-to-double idiom, reproduced as `read_u16(..) as i16 as f32`.
- `0x800A0FB0`'s compare is an import thunk (`r13-0x5980` → `0x80429D00`, empty
  in the image); the in-image stub `0x80064A40` is `fcmpo`/`mfcr`/`rlwinm`,
  i.e. `f1 < f2`.
- `0x800A1190` is byte-identical to `fn_80097f70`; `0x800A7330` to
  `0x8009E910`; `0x800A2B70`/`0x800A2BA0` to `0x80098350`/`0x80098360`.
- Data read from the ELF: `0x802F0760` = `0.0, 3.8, -4.0, -0.243902, …`,
  `0x8042C320` = `0x0100, 0xFF00, …`, `0x8042C31C` = `1.0f`,
  `0x8042C1C4` = `0.5f`, and the `"getin"`/`"face"`/`"damage"`/`"hit"` strings.
- Thirteen bodies are medium only for the unmodellable `r12` descriptor
  (`0x802DCBFC` forwards) or for inferred record roles; all offsets and calls
  are literal.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-lane-csv.mjs wave223`: 60 rows, 60 unique, 47 high + 13 medium.
- Ledger: 20,178 rows (99.63%); `game::stage` 1154/1226.
- `scripts/check.ps1` exit 0.

## Next

72 `game::stage` bodies (rounds 224-225), then the untabled bodies and the
accumulated audit items.
