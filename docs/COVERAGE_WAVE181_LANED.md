# Coverage - Wave 181 lane D (`game::bases`)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane D block) | `game::bases` | 8 | 764 | 8 | 0 |

Full lane from the sub-300-byte sweep of the band. Source of the bodies:
`work/wave181/laneD_block.rs`. Every address is the REL `d_basesNP`; all
addresses in the docs are `0x8060_0000`-based analysis addresses, not runtime
addresses.

## What this pass covers

- **Layout reset + animation (1 body).** `0x806139D0` queues sound `0x84`
  through `0x8019_5380` with the manager word stored at `0x8042_A488`, runs the
  `0x800C_9500` layout pass on `+0x70` and starts animation `9`
  (`0x800C_92F0(this + 0x70, 9, 0)`). All three arguments sets are literal
  (`li r4,132`, `li r4,9`, `li r5,0`).
- **Deleting destructors (5 bodies).** `0x80614470`, `0x80615600`,
  `0x8061B030` (96 bytes) and `0x806144D0`, `0x8061B090` (100 bytes) all
  compile to the same body already translated as [`fn_806357e0`]:
  `if this { if this+0x18 != 0 { 0x8015_F9F0(this+0x18, 0) }
  if free > 0 { 0x802B_9280(this) } } return this`. The REL keeps five entries
  because each class registers its own destructor; the Rust keeps five bodies so
  the original call targets still resolve.
- **Pane flag + animation restart (1 body).** `0x80615D90` is the show/hide tail
  of the `dFileSelect_c` family, already called by [`fn_806178d0`],
  [`fn_806187b0`] and [`fn_80618a60`] with mode `0`/`1`. It rewrites the low bit
  of `*(*(screen + 0x27C) + 0xBB)` and tail-branches to
  `0x800C_92F0(screen + 0x74, animation, 0)`.
- **Animation-finished transition (1 body).** `0x8061B720` polls
  `0x800C_9610(screen + 0x70, 0xFFFFFFFF)` and, on `0`, dispatches vtable slot
  `+0x18` of the object at `+0x330` with descriptor `0x8082_4BC0`. Identical in
  shape to the translated [`fn_80617860`], which uses `+0x20C`/`0x8082_45F0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806139D0` | `fn_806139d0` | 84 | high |
| `0x80614470` | `fn_80614470` | 96 | high |
| `0x806144D0` | `fn_806144d0` | 100 | high |
| `0x80615600` | `fn_80615600` | 100 | high |
| `0x80615D90` | `fn_80615d90` | 100 | high |
| `0x8061B030` | `fn_8061b030` | 96 | high |
| `0x8061B090` | `fn_8061b090` | 100 | high |
| `0x8061B720` | `fn_8061b720` | 88 | high |

## Deferred

None.

## Corrections and notes

- `0x80615D90` is the one body Ghidra could not decompile: its export stops at
  `halt_baddata()` right after the two `stb` stores. The body was decoded from
  the raw words, which are a clean `cmpi`/`beq` + two straight-line halves:
  - `rlwinm r0,r6,0,24,30` keeps MSB bits 24..30, i.e. LSB bits 1..7 - a
    `& 0xFE` mask; `ori r0,r0,1` then sets bit 0. The `mode == 0` half
    (`beq` target `0x80615DC8`) only stores the masked byte, the fall-through
    half stores `(byte & 0xFE) | 1`.
  - The two halves pick different animation tables at `0x807C_6C28` and
    `0x807C_6C34`, both read from `work/rels/elf/d_basesNP.elf`:
    `0x807C6C28 = [13, 14, 15]`, `0x807C6C34 = [10, 11, 12]`. The Ghidra
    export showed only the flag store, which is why its `else` branch (set bit)
    and its `if` branch (clear bit) read inverted relative to the animation ids.
  - The index scale is `rlwinm r0,r4,2,0,29` - `index * 4` with the low two
    bits of the shifted value cleared, modelled with
    `index.wrapping_shl(2) & 0xFFFF_FFFC`. Callers only pass `0..3`
    ([`fn_806187b0`], [`fn_80618a60`]), so the mask never bites.
  - The final `b 0x800C92F0` is a tail branch, so the callee's `r3` is
    discarded and `fn_80615d90` is `void`.
- The five destructors split by size: the 100-byte forms carry one extra,
  provably redundant null test (`41820014 bc 12,2,...` on `r3` after the
  `cmpi cr0,3,r3,0` guard) that the compiler emitted in front of the
  `addic. r3,r3,24`. Nothing observable changes, so all five share one body.
  `addic. r3,r3,24` is a `r3 + 0x18` null test; with `r3 != 0` already
  established it can never be zero, and the existing module convention
  ([`fn_806357e0`]) keeps that test as written.
- `0x8015F9F0` (here) and `0x8015F800` (other destructors in this module) are
  distinct release helpers; these five bodies use `0x8015F9F0` only, verified
  from `bl 0x8015f9f0` in every one of the five raw listings.
- `0x802B9280` is called with `this` in `r3` (`mr r3,r30` before the `bl`), and
  the epilogue restores `r3 = r30`, hence the `-> usize` return.
- `0x8061B720` loads the vtable straight from the object's first word
  (`lwz r12,0x330(r31)`) - the interface pointer lives *at* `+0x330`, so the
  existing `vmethod1(obj, slot, arg)` helper applies rather than the
  `abi_callN!` form used for objects whose vtable sits at `+0x78`/`+0xB4`.

## Verification

- Every one of the eight bodies was checked against the raw words printed by
  `work/.../ppc159.js` (`$env:LEN='34'` and `'64'`), not just the Ghidra
  export; the five destructor listings agree instruction-for-instruction apart
  from the redundant guard and the return-address displacements.
- Constants read from `work/rels/elf/d_basesNP.elf`: the two animation tables
  at `0x807C6C28`/`0x807C6C34`. Sound id `0x84`, animation `9`, descriptor
  `0x8082_4BC0` (`addi r4,r4,19392`, `r4 = 0x80820000`) and the flag masks are
  all immediate fields in the instruction words.
- `src/game/bases.rs` was not modified; the coordinator's `merge.cjs` appends
  the block.
