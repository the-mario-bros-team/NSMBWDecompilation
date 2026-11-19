# Coverage - wave 189 lane A (`game::bases`, `0x80700320..0x80717CF0`)

Lane A of wave 189 translated ten bodies from `rel/d_basesNP.elf` (bucket
`0x80600000`, synthetic analysis base; not a runtime address). All ten are
`translated` at `high` confidence. Nothing was deferred.

Address span: `0x80700320..0x80717D88`. Block:
`work/wave189/laneA_block.rs`. Ledger rows:
`ledger/agents/wave189-lanea.csv`.

## Bodies

| Address | Bytes | Name | Description | Confidence |
| --- | --- | --- | --- | --- |
| `0x80700320` | 160 | `fn_80700320` | Snaps `+0xAC` to `+0x8F0` and dispatches actor slot `+0xD4` when the signed `0x8042C1A0` byte times the speed delta is `> 0.0`. | high |
| `0x80705890` | 172 | `fn_80705890` | Emits `count` screen lines through `0x80077860` (id `0x92`), stepping `-y` by `16.0`. | high |
| `0x80706360` | 164 | `fn_80706360` | Membership test of a word in the twelve `0xC`-stride entries at `+0x438`; returns `0`/`1`. | high |
| `0x80706440` | 144 | `fn_80706440` | Re-seats every occupied `+0x438` record via `0x80706850`, then writes `0x600`/`0` or `0x30` into `+0x3FC`/`+0x400`. | high |
| `0x80706980` | 140 | `fn_80706980` | Tears down the `+0x438` table and `+0x3F0`/`+0x3D0` members; frees when `count > 0`. | high |
| `0x80709EC0` | 184 | `fn_80709ec0` | Actor constructor: base `0x80094E80`, five member ctors, vtables `0x80805210`/`0x807E3E98`/`0x80805490`/`0x807E2740`. | high |
| `0x80714370` | 136 | `fn_80714370` | Compares the `:`-suffix of virtual slot `+0x20` against the given name via `strchr`/`strcmp`. | high |
| `0x80717AE0` | 148 | `fn_80717ae0` | Allocates `0x4A10` bytes and builds the nine-element `0x2290` arrays at `+0x4DC`. | high |
| `0x80717B80` | 136 | `fn_80717b80` | Constructs the three `0x84`-element arrays at `+0x0` (9), `+0x4A4` (`0x36`) and `+0x207C` (4). | high |
| `0x80717CF0` | 152 | `fn_80717cf0` | Destroys those three arrays in descending order; frees via `0x802B9280` when `count > 0`. | high |

## Evidence and decoding notes

- **Ghidra's `80706360` return value is wrong.** The export returns the `uVar2`
  accumulator (`5` on the second pass), but every match arm is `li r3,0` before
  `blr`, and the fall-through is `li r3,1`. All early exits therefore return
  `0`; `uVar2` only feeds the dead `addi r4,r4,5`. The body returns `0`/`1`.
  The two `bdnz` passes walk six words each at `0x48` stride, which is the same
  twelve entries as a single `0xC`-stride walk.
- **`80705890` is a `do`/`while (--count != 0)`.** The loop head is the `fneg`
  at `0x807058DC`, so a zero count still runs once and wraps to `2^32 - 1`
  iterations. The body keeps that shape with `wrapping_sub`. `x` is the single
  `fctiwz` of `+0xAC` taken before the loop; `y` is recomputed from `-f30` each
  iteration. Reuses the existing `draw_screen_line` helper (`0x80077860`).
- **`80700320` raises on the sign table, not the raw delta.** `fcmpo cr0,f1,f0`
  plus `bc 4,1` branches unless `f1 > f0`, i.e. the body runs when
  `byte * (+0xAC - +0x8F0) > 0.0`; Ghidra's `DAT_807cc4dc < X` says the same.
  The `0x43300000`/`0x43300000_80000000` pair is the compiler's signed
  byte-to-float conversion, written as `as i8 as f32` like `fn_806b5a80`.
- **`80706980` / `80717CF0` null guards and `count > 0`.** Both use
  `cmpwi`/`bc 4,1`; CR bit 1 is GT, so the branch means "skip unless greater".
  `li r4,-1` is passed as `0xFFFF_FFFF`.
- **`80709EC0` store order.** `0x802D7AA0(this+0x70C)` installs its own vtable,
  which is then overwritten by `0x807E2740`; both stores are kept in order.
  `0x80094E80` keeps `r3 == this`, so it is called explicitly through
  `abi_call1!` (same convention as `fn_807262f0`).
- **`80717B80`/`80717CF0` callbacks.** The array builder `0x802DC9A0` takes
  `(base, ctor 0x80717C10, dtor 0x80717C80, 0x84, count)`; the destructor
  `0x802DCA98` takes `(base, dtor 0x80717C80, 0x84, count)`.

## Deferred

None.
