# Wave 194 lane C coverage (`game::bases`)

Ten functions, `0x8074A4F0`..`0x8075C3B0`, all translated into
`work/wave194/laneC_block.rs` for the coordinator to append to
`src/game/bases.rs`. Nothing under `src/` was edited by this lane; the block was
type-checked against a scratch crate that stubs only the helpers it calls.

## Translated

| Address | Bytes | Rust name | Summary |
| --- | --- | --- | --- |
| `0x8074_A4F0` | 256 | `fn_8074a4f0` | Rectangle test `left <= x`, `x < left + width + shift`, `y < top`, `top - height <= y` for kind `(+0x4 & 0xF) == 1`; `shift` is `-160.0` when `0x8005FEF0` reports more than one player, else `0.0`. |
| `0x8074_BB00` | 248 | `fn_8074bb00` | Plays cue `0x86` at `{0x8074B8D0 slot, +0xB0, 1500.0}`, then walks the three-slot cooldown: `+0x3D8` hold, `+0x3D4` count, `{60,90,120,150}` re-arm table at `+0x3E0`, positions at `+0x3E4`. |
| `0x8074_CA70` | 228 | `fn_8074ca70` | Four-slot overlap scan that latches the changed slot at `+0x41C`, decrements the `+0x42A` countdown (`0x1F` reload) and calls `fn_8074c3d0` while `fn_8074c350 < 2`. |
| `0x8074_DD30` | 248 | `fn_8074dd30` | `+0x3D4` timer expiry: counts overlapping players, checks `fn_8074dad0` and the `0x80429D48 + 0xBCA` byte, then advances `+0x3D8` and re-arms `+0x3D4` to `0x50`/`0xB4`. |
| `0x8074_EFF0` | 220 | `fn_8074eff0` | Mode-1/mode-2 matrix rewrite of the `+0x4` part: `0x801C0500` copy, `0x8016EDD0`/`0x8016EEF0` rotation from `+0x6A0`/`+0x6A2`, `0x801C0BD0` + `0x801C0540` translation pass for mode 1. |
| `0x8075_2AC0` | 200 | `fn_80752ac0` | `1/16`-step bounce integrator over `+0x47C`/`+0x480`/`+0x484`, `+0.125` bias, `0.5` clamp, direction byte `+0x348`. |
| `0x8075_37D0` | 236 | `fn_807537d0` | Same integrator over `+0x1A4A8`/`+0x1A4AC`/`+0x1A4B0`, additionally latching `+0x1A490`/`+0x1A494` into `+0x1A498`. |
| `0x8075_38E0` | 196 | `fn_807538e0` | Bails to virtual `+0x394` slot `+0x18` (`0x80831FD0`) when `+0x358 | +0x35C` is set, else steps `+0xB0` via `fn_807537d0` unless `+0x1A4A4`/`+0x348` already agree. |
| `0x8075_BE90` | 212 | `fn_8075be90` | Two-by-two `0x800BBC10` slot scan; per occupied slot resolves the `0x16518`-stride part and calls `0x800EF570` with the `part[3] - '0'` character offset. |
| `0x8075_C3B0` | 216 | `fn_8075c3b0` | Destructor: `0x80814668` at `+0x60`, releases the `+0x1EC` member, tears down `+0x1B8`/`+0x1AC`/`+0x18C`/`+0x158`/`+0x13C`, `0x800F2770(this, 0)`, optional `0x80162920`. |

## Deferred

None. All ten bodies were recovered from the raw words.

## Notes on the two requested callees

* `fn_8075be90(_this: usize, first: u32, second: u32)` - the exact signature has
  three words in `r3`/`r4`/`r5`, but the body never reads `r3`. `fn_8075c070`
  calls it as `fn_8075be90(param_1, 0, 1)` from states `3` and `4`; `first` is
  forwarded to `0x800EF570` as `r4` and `second` as `r6`. The Ghidra export's
  `extraout_r4` is just `r24 = first`.
* `fn_8074bb00` needs the return register of `0x80064610`, which the existing
  `fn_8074b880` wrapper discards (its Rust signature is `-> ()`). The body
  therefore emits the identical `(0x86, 0, &point, 0, 0)` call inline and tests
  `r3`, matching the raw `cmpwi`/`beq` at `0x8074BB7C..0x8074BB84`.

## Two places where the raw words beat the export

* `0x807538E0`: the export folds two masked tests against the `0x8042A078`
  word pair. The object code is `and r5,r0,r0` / `and r6,r4,r4` /
  `or. r0,r4,r0` / `beq`, so the mask loads are dead and the guard is
  `(+0x358 | +0x35C) == 0`. The block implements the OR test and documents it.
* `0x8074A4F0`: four dead `lfs`/`stfs` spills of `0x80429E00+0x38..0x44` sit in
  the prologue. They are omitted; the live loads are only `+0x3D4..+0x3E4`.

## Evidence

* Exports: `work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`
* Raw words: `ppc159.js` over `work/rels/elf/d_basesNP.elf`
* Constants read from the ELF: `0x807CE598 = 0.0`, `0x807CE5E4 = 160.0`,
  `0x807CE618 = 1500.0`, `0x807CE6C8 = 0.0`, `0x807CE844 = 0.0625`,
  `0x807CE848 = 0.125`, `0x807CE838 = 0.5`, `0x807CE8A0 = 0.0625`,
  `0x807CE8A4 = 0.125`.
* Ledger: `ledger/agents/wave194-lanec.csv` (10 rows).
