# Coverage - wave 184 lane E (`game::bases`, `d_basesNP`)

## How to read this file

This is the lane-E slice of the wave-184 sub-300 sweep: ten small bodies from
the PowerPC REL `d_basesNP`, all between `0x80716B60` and `0x807209B0`. Every
new body lives in `work/wave184/laneE_block.rs` and is appended to
`src/game/bases.rs` by the coordinator; lane E did not edit `src/game/bases.rs`,
`lib.rs`, any `mod.rs`, `Cargo.toml`, `src/platform.rs`, another lane's files,
`ledger/functions.csv` or any generated document.

- **address** - REL virtual address; the synthetic analysis base is
  `0x80600000`, so these are not runtime addresses.
- **name** - the Rust function name that will exist in `src/game/bases.rs`.
- **role** - one plain sentence.
- **confidence** - `high` when every load/store/branch/constant was read from
  the raw words; `medium` when a field meaning or call identity is inferred.

All ten entries were translated; nothing was deferred.

## Summary

| Metric | Count |
|---|---:|
| Bodies translated | 10 |
| Bytes of original code | 972 |
| `translated` | 10 |
| Deferred | 0 |

## Bodies

| Address | Size | Name | Role | Confidence |
|---|---:|---|---|---|
| `0x80716B60` | 92 | `fn_80716b60` | Runs `0x8009_55F0`, clamps height `+0xB0` up to `+0x670` and fires event `0x8082_FAE0` through vtable slot `0xD4`. | high |
| `0x80717960` | 88 | `fn_80717960` | Registered-object destructor: `0x8015_F800(obj,0)` then `0x802B_9280(obj)` when `free > 0`. | high |
| `0x80717EF0` | 116 | `fn_80717ef0` | If `0x8006_4F50(this,0)` is zero: interface slot `+0x10` at `this+0x394`, snapshot `+0xAC..+0xB4` into `+0x49FC..+0x4A04`, then `0x8004_5450(this)`; else `1`. | high |
| `0x80718960` | 100 | `fn_80718960` | Copies the first byte of `other`'s slot-`0x6C` result into `this+0x5CD` when `other+0x38C` is 1/2 and `this+0x5BC & 0x10`. | high |
| `0x80719B80` | 88 | `fn_80719b80` | Same destructor template as `0x80717960` (identical raw words). | high |
| `0x8071AD90` | 80 | `fn_8071ad90` | Compares the `0x8016_A5D0` frame token with `+0x538`; on mismatch dispatches interface slot `+0x10` at `this+0x544`; always returns `1`. | high |
| `0x8071D120` | 128 | `fn_8071d120` | Clamped byte lerp: `t<0 -> from`, `t>1 -> to`, else `(from + trunc(t*(to-from))) & 0xFF`. | high |
| `0x8071EAA0` | 88 | `fn_8071eaa0` | Same destructor template as `0x80717960`. | high |
| `0x8071F0B0` | 104 | `fn_8071f0b0` | Builds a 52-byte transform from `+0x3B4`, extracts 16 bytes and plays table entry `0x807C_D070[index]` (`"Wm_mr_palm_s"`, `"Wm_mr_palm"`) via `0x8016_C890`. | high |
| `0x807209B0` | 88 | `fn_807209b0` | On global course state `0x8042_8730 == 0x2EE`: set `+0xB4 = 1`, call `fn_80720980(this,0)`, set `+0x58`/`+0x94` of the `0x8042_9C94` effect pool. | high |

## Deferred

None.

## Notes and corrections

- `0x8071D120`: Ghidra types the blend factor as `double`. The shipped body
  loads its two limits with `lfs` (0.0 at `0x807C_D014`, 1.0 at `0x807C_D020`)
  and uses `fcmpo`, `fmuls`, `fsubs` and `fctiwz`, so the source parameter is
  `f32`; the translated signature follows the instructions, not Ghidra's
  prototype. The `0x4330000080000000` constant at `0x807C_D028` plus the
  `xoris 0x8000` stack pair is the compiler's unsigned-int-to-float idiom.
  `fctiwz` truncates toward zero, so the cast is `as i32`, not a round.
- `0x8071F0B0`: Ghidra prints `(&PTR_s_Wm_mr_palm_s_807cd070)[param_2]`; the
  raw word is a `lwzx` from `0x807C_D070`, which is a two-entry pointer table
  followed by float data, hence the explicit `read_u32(0x807C_D070 + index*4)`.
- `0x807209B0`: Ghidra's `_DAT_80428730` overlapping symbol is a 16-bit field
  in this function (`lhz`), so the check uses `read_u16`.
- `0x80718960`: `functions.tsv` records 92 bytes, but the raw words show the
  `addi r1,r1,16` / `mtspr 256,r0` / `bclr` epilogue at `0x807189B0..0x807189C3`,
  so the real size is 100 (the value used by the wave brief); the Ghidra export
  stops two words early.
- `0x80716B60`, `0x80717EF0`, `0x80718960`, `0x8071AD90`: the virtual calls
  read the vtable pointer out of the object (`+0x60`, `+0x394`, `+0x544`)
  rather than from offset 0, and are emitted as raw slot loads plus
  `abi_callN!`; `0x80718960` uses the existing `actor_method0` helper because
  its vtable is the standard `+0x60` one.
- `0x80717960`, `0x80719B80`, `0x8071EAA0`: three independent destructor
  callbacks with identical code; the raw words differ only in the relocated
  `bl` displacements.

## Verification

The block was type-checked with `rustc --edition 2021 --crate-type lib` against
local stubs for `platform` and the `abi_callN!` macros; it compiles with no
errors or warnings. The stubs were deleted afterwards.
