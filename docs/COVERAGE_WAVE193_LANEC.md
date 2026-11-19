# Wave 193 lane C coverage (`game::bases`)

Ten functions, `0x806DD550`..`0x806F6700`, all translated into
`work/wave193/laneC_block.rs` for the coordinator to append to
`src/game/bases.rs`. Nothing under `src/` was edited by this lane.

## Translated

| Address | Bytes | Rust name | Summary |
| --- | --- | --- | --- |
| `0x806D_D550` | 248 | `fn_806dd550` | Seeds the `+0x3F8` target model through the `0x802F8FF0` type word, enables it, and applies a `(1.0, +0x6EC, 1.0)` scale matrix. |
| `0x806E_26B0` | 228 | `fn_806e26b0` | Rebuilds the four `0xE0`-stride plank nodes at `+0x430` from `+0x8C4`: `48.0 * trig` pair, anchor add, angle at `+0xDC`, `0x4000` step. |
| `0x806E_33A0` | 224 | `fn_806e33a0` | Resets one plank node from its `+0x60` origin, zero angles and unit scale, then calls node slot `+0x14`. |
| `0x806E_9C10` | 224 | `fn_806e9c10` | Five-deep `+0x30` chain walk returning the node whose successor passes `virtual +0x0C`; recurses on the fifth successor. |
| `0x806E_C2B0` | 200 | `fn_806ec2b0` | Stores an attach position/angle, loads the bolt BRRES, calls `virtual +0x08`, then seeds and commits the `+0x80` animation pair. |
| `0x806E_C5F0` | 220 | `fn_806ec5f0` | Computes `target[0] = +0x16C - sin*S`, `target[1] = +0x170 + cos*S` from the `+0x178` heading and the runtime global `0x8042B7A8`. |
| `0x806E_D160` | 252 | `fn_806ed160` | Walks one cart record's rider list, ORs the `1 << slot` mask for the cart and its attached rider, and starts the `+0x138E` hold counter. |
| `0x806F_6040` | 224 | `fn_806f6040` | Byte-identical twin of `fn_806e9c10` with a different recursion target. |
| `0x806F_6240` | 212 | `fn_806f6240` | `0x858`-byte lift actor constructor: `0x80801C58` vtable, `+0x53C`/`+0x560`/`+0x580`/`+0x5C0`/`+0x5F8`/`+0x724` sub-objects. |
| `0x806F_6700` | 220 | `fn_806f6700` | Rebuilds `+0x7C` from position/`+0x100` angles via `0x80064AF0`, then copies it and `+0xDC` into the `+0x580` node. |

## Deferred

None. All ten bodies were recovered from the raw words; no jump table, no
recoverable-only branch, and no missing constant blocked a translation.

## Notes on the two requested callees

* `fn_806ec5f0(this: usize, target: usize)` - writes a two-float target and
  reads the runtime global `0x8042B7A8`. Callers `fn_806ec6d0` (adds
  `+20.0 * cos`) and `fn_806ec780` (adds `-20.0 * cos`) keep working: both add
  onto the seeds this body writes.
* `fn_806ed160(this: usize, record: usize) -> u32` - the export prints no
  arguments because the body opens with the `0x802DCD6C` register save; the
  three call sites in `fn_806ed260` (`0x806ED2B4`, `0x806ED2CC`, `0x806ED2EC`)
  pass `r3 = manager`, `r4 = record` and consume the returned mask in `r3`.
  `fn_806ed260` may now call this body directly instead of `abi_call2!`.

## Evidence

* Exports: `work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`
* Raw words: `ppc159.js` over `work/rels/elf/d_basesNP.elf`, plus a branch scan
  used to recover the `0x806EC2B0`/`0x806EC5F0`/`0x806ED160` call sites.
* Constants read from the ELF: `0x807CBBE4 = 48.0`, `0x807CBBE0 = 0.00390625`,
  `0x807CBBF0 = 1.0`, `0x807CB9B0 = 1.0`, `0x807CBF44 = 0.00390625`.
