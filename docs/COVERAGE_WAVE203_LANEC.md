# Wave 203 lane C coverage (`ui::hud`)

Ten bodies, all translated, no deferrals. Range `0x80020ED0..0x800217A0`
(`main.dol` runtime addresses).

| Address | Size | Body | Confidence |
| --- | --- | --- | --- |
| `0x80020ED0` | 112 | `fn_80020ed0` - force button state `3`, take the child's `+0x6C` digit, store it in group `+0x670`, mirror `+0x674`, write mode `+0x68D`. | high |
| `0x80020F40` | 176 | `fn_80020f40` - state-`3`/flag-bit-`0` gate, owner chain `+0x394` slot `+0x28` then slot `+0x14` against `0x80353A64`; success calls `fn_80020ed0` and returns `1`. | medium |
| `0x80021100` | 56 | `fn_80021100` - null guard; input bit `1` sets `+0x67D`/`+0x67E`, bits `0x28` set `+0x67F`. | high |
| `0x800212C0` | 424 | `fn_800212c0` - select one of the three digit groups (`+0x66C`, `+0x668`, `+0x670`) with the `0xFF` sentinel and return mode `0`/`1`/`2`/`3`. | high |
| `0x80021470` | 128 | `fn_80021470` - look up a player record through `0x8005FB90`, call its `+0x3FC` position slot with two floats and four integers, set `this + player + 0x675`. | medium |
| `0x800214F0` | 128 | `fn_800214f0` - per-frame strict float gate and four-slot scan over `+0x668`/`+0x675`, forwarding the slot byte to `fn_80021470`. | high |
| `0x80021580` | 128 | `fn_80021580` - byte-identical twin of `fn_80021470`. | high |
| `0x80021600` | 136 | `fn_80021600` - twin of `fn_800214f0` calling `fn_80021580`. | high |
| `0x80021740` | 60 | `fn_80021740` - write `{0x80020001, 0xFFFF9000, 0x7000, 0x10000}` at `+0x524` and tail-call `0x8006D090(this+0x1EC, this, 0, this+0x524, 0)`. | high |
| `0x80021780` | 32 | `fn_80021780` - post-step validator; `+0x1F8 != 0` forwards `this+0x1EC` to `0x8006EF40` with mask `0`. | high |

## Notes carried into the bodies

- **Manager fields.** These bodies continue the manager run begun by wave 203
  lane B at `0x800208C0`. The three four-byte digit groups are `+0x668`,
  `+0x66C` and `+0x670`; `fn_80020aa0` (lane B) writes `+0x66C`, while
  `fn_80020ed0` writes `+0x670`. `+0x694` is the selected digit, `+0x68D` is
  the output mode, `+0x680` is the button state,
  `+0x675..+0x678` are the four per-player done flags and `+0x689` is the
  start gate. `+0x284` receives a copy of the selected digit on the `+0x66C`
  and `+0x670` paths only, exactly as the original stores it.
- **Sentinel.** Ghidra prints the digit comparisons as `c != -1`; the bytes
  are unsigned `lbz`/`stb`, so `0xFF` is the correct Rust test.
- **Loop argument.** `FUN_800214f0`/`FUN_80021600` do not pass the loop index
  to the position helper. The raw words load `this + i + 0x668`, sign-extend
  it (`extsb`) into `r4`, and `bl` the helper; the Ghidra export drops that
  operand. The reconstruction forwards the slot byte, which is the value the
  ABI actually carries.
- **Strict float gate.** `fcmpo` followed by `cror cr0[eq], cr0[lt], cr0[eq]`
  and `bc 12,2` is the `(SDA2[0x7DEC] < this+0xEC)` test; equality skips the
  scan.
- **Position call helper.** `0x80021470`/`0x80021580` call the virtual
  `+0x3FC` slot with `f1`, `f2`, `r3..r6`. The existing host-side helpers only
  cover two-float/two-integer calls, so the six-operand ABI is expressed with
  an inline `extern "C"` transmute in each body rather than adding a new
  module helper.
- **Constants.** `SDA2_BASE - 0x7DEC` is the frame/time float, `SDA2_BASE -
  0x7DE4` plus `SDA_BASE - 0x5370` is the X position operand. Their symbolic
  roles are not identified further.
- **Descriptor.** `0x80021740` stores `0x80020001` as data, not a callable
  target with the low bit stripped; the raw store is preserved verbatim.
