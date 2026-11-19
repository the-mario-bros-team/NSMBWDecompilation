# Wave 213 lane D coverage (`game::player`)

Slice: the ten `game::player` bodies at `0x8006A4B0`, `0x8006A5E0`,
`0x8006A620`, `0x8006A6A0`, `0x8006A6F0`, `0x8006A730`, `0x8006A760`,
`0x8006A7D0`, `0x8006A7F0` and `0x8006A830`. All ten are translated; nothing
was deferred.

Bodies live in `work/wave213/laneD_block.rs` and are appended to
`src/game/player.rs` by the merge step; ledger rows are in
`ledger/agents/wave213-laneD.csv`.

## Translated

| Address | Bytes | Rust body | Summary |
| --- | --- | --- | --- |
| `0x8006A4B0` | 180 | `fn_8006a4b0` | World-to-screen projection. Reads the scene at `0x80429E00`, queries `0x80082240(x, scene)`, offsets by `*(0x8042BBFC) * scene->0x40`, computes the vertical centre with `0x800B30E0()` (`scene->0x3C - scene->0x44 * *(0x8042C578)`), then writes `k * (x - xoff) / sx` and `k * (y - yctr) / sy` with `k = *(0x8042BC00)`. Callers keep a two-float stack record, which is why the crate refers to this as a "two-word record builder". |
| `0x8006A5E0` | 64 | `fn_8006a5e0` | Builds `(*(0x8042BC10), *(0x8042BBF8))` and its swap on the stack and calls `0x800F9EA0(a, b, &(p,q), &(q,p))`. |
| `0x8006A620` | 128 | `fn_8006a620` | State publisher: ignores the sentinel `0x2EE`; otherwise asks `0x80069610` for an id, forwards a non-zero id to `fn_8006a6a0`, re-triggers `fn_8006a730` on the `5 -> 5` edge using `0x8006A250` / `0x8006A270` (second argument reduced to `b != 0`), and stores the low halfword at `0x80427934`. |
| `0x8006A6A0` | 56 | `fn_8006a6a0` | Stores `id` at `0x80427938` and calls `0x8019B660(*(0x8042A4A8), id)`. |
| `0x8006A6F0` | 52 | `fn_8006a6f0` | Opens the eight-byte descriptor at `0x8006A6E0` against `0x80427938` through `0x8016B950` (a `b 0x8016B8D0` thunk), stores the handle at `0x80429D80`, and returns `1`. |
| `0x8006A730` | 20 | `fn_8006a730` | Tail-forwards `(a, b)` to `0x8019B880(*(0x8042A4A8), a, b)`. |
| `0x8006A760` | 112 | `fn_8006a760` | When the handle at `0x80429D80` is non-null with a set `+0x08` byte, releases it through `0x8016B7E0` and zeroes the slot; returns `1` when the slot is empty and `0x8019B9F0(sound)` is non-zero. |
| `0x8006A7D0` | 28 | `fn_8006a7d0` | `mask |= 1 << index` on the byte at `0x80429D84`. |
| `0x8006A7F0` | 32 | `fn_8006a7f0` | `mask &= !(1 << index)` on the byte at `0x80429D84`. |
| `0x8006A830` | 344 | `fn_8006a830` | Per-frame request machine: shifts the two-frame enable history (`0x80429D86` / `0x80429D85` / `0x80429D84`), calls `0x8019C670` / `0x8019C730` on the enable edge, forces bit `0x8` when the mode object's `+0x1D0` is `2`, sends the `0xB` payload to `0x8019C7F0` when idle, enters the pause with `0x800695F0` + `0x80195380(obj, 0x7F, 1)` on bit `0x2`, and drops the request via `0x8019C880` or leaves the pause via `0x80069600` once `0x8006A9A0` reports completion. |

## Sizes and extents

All ten `# Original` blocks keep the canonical `main-gb` `functions.tsv` value,
and each one was re-checked against the prologue and the `blr`:

- `0x8006A4B0`: `blr` at `0x8006A560`, next entry `0x8006A570`; 180 bytes.
- `0x8006A5E0`: `blr` at `0x8006A61C`, next entry `0x8006A620`; 64 bytes.
- `0x8006A620`: `blr` at `0x8006A698`, next entry `0x8006A6A0`; 128 bytes.
- `0x8006A6A0`: `blr` at `0x8006A6D4`; 56 bytes.
- `0x8006A6F0`: `blr` at `0x8006A720`; 52 bytes.
- `0x8006A730`: the last instruction is `b 0x8019B880` at `0x8006A740`, so the
  body is 20 bytes with no local `blr`.
- `0x8006A760`: `blr` at `0x8006A7CC`; 112 bytes.
- `0x8006A7D0`: `blr` at `0x8006A7E8`; 28 bytes (the four-byte tail is the
  alignment pad to `0x8006A7F0`).
- `0x8006A7F0`: `blr` at `0x8006A80C`; 32 bytes (same four-byte pad).
- `0x8006A830`: `blr` at `0x8006A984`, next entry `0x8006A9A0`; 344 bytes.

No table value in this lane is a 16/20-byte prologue artefact.

## Notes on the raw words

- SDA bases: `r13 = 0x8042F680`, `r2 = 0x80433080`. The cluster's globals are
  `0x80427934` (last state, `r13 - 0x7D4C`), `0x80427938` (sound id,
  `r13 - 0x7D48`), `0x80429D80` (handle, `r13 - 0x5900`), `0x80429D84`
  (`r13 - 0x58FC`, mask), `0x80429D85` / `0x80429D86`
  (`r13 - 0x58FB` / `- 0x58FA`, history), `0x80429D74` / `0x80429D78`
  (`r13 - 0x590C` / `- 0x5908`, request and accumulator), and `0x80429D71`
  (`r13 - 0x590F`, paused byte).
- `0x8006A6F0`'s first argument is the word at `0x8006A6E0`, not a code
  address: `lis 0x8007` / `addi -0x5920` assembles to `0x8006A6E0`.
- `0x8006A620`'s call to `0x8006A6A0` carries `r3` from `0x80069610`; the
  Ghidra C prints no argument there, but the `bl` follows the `cmpi` without
  reloading `r3`, so the id is forwarded.
- `0x8006A760`'s `0x8016B7E0` likewise receives `r3 = handle` from the
  preceding `lwz`.
- `0x8019B9F0` is literally `li r3,1; blr` (8 bytes), so the second half of
  `fn_8006a760` always reports `1` today; the call is still reproduced.
- `0x8006A830`'s final `(param_1 & 2) == 0` test is `rlwinm r31,r30,0,30,30`,
  i.e. bit 1 after the `& 0xB` reduction; `flags` is `u32` and the mode-force
  `|= 8` cannot affect that bit.
- The `1 << index` shifts in `0x8006A7D0` / `0x8006A7F0` use the PowerPC `slw`
  semantics (low six bits of the count; counts `>= 32` produce zero). The Rust
  bodies reproduce that rather than Rust's five-bit wrapping shift.

## Reused names

The lane reuses `W22_SCENE_80429E00` (`r13 - 0x5880`), `W74_SOUND_8042A4A8`
(`r13 - 0x51D8`) and `W92_EFFECT_TABLE_8042A488` (`r13 - 0x51F8`) instead of
defining new constants. It does not redefine `0x8006A4B0`; the body itself is
the definition of that address.
