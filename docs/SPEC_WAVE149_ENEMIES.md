# Wave 149 - implementation specification for 12 untranslated `d_enemiesNP` bodies

## What this page is

`docs/ANALYSIS_WAVE127_ENEMIES.md` reclassified 384 formerly-skipped `d_enemiesNP`
addresses into 173 class **B**, 160 class **C** and 51 class **D**. Waves 128-148
worked through that block; this page continues the plain size-ranked tranche: the
twelve largest remaining bodies that are untranslated, have a Ghidra export, and
are fully walkable from their entry.

A live re-walk at selection time of
`work/ghidra/rel-d_enemiesNP/export/functions.tsv` (3,555 rows) against the
`address` column of `ledger/functions.csv` (16,973 distinct addresses), against
every `0x80Axxxxx`/`0x80Bxxxxx` address in a `^#` heading of any `docs/*.md`
(339 distinct addresses) and against every `fn_<addr>`/`FN_<addr>` token in
`src/**/*.rs` (16,313 distinct addresses) leaves **72** untranslated bodies, all
72 with an export. The twelve sections below are the twelve largest, so a re-run
of the same three scans after this page is written reports **60** candidates.

This page is **read-only**: it writes no Rust, no ledger row and no other doc.

## Selection

The ranking metric is the **reachable body**, not the raw `functions.tsv` extent,
because the TSV size is a bucket extent that can include a neighbour's padding or
an unreachable arm (see `docs/REL_ELF_MAPPING.md`). Each candidate was walked from
its entry through every conditional branch, every internal `b` and every
**linked** indirect branch (`bctrl`/`bclr` with `LK = 1`), until all paths ended
in `bclr`, a tail `b` or a tail `bcctr`. Conditional `bclr`/`bcctr` forms (for
example `bnelr`) were followed on both the taken and the fall-through edge, so a
body ending in `bnelr` still reports its fall-through reachability.

| # | address | TSV size | reachable insns | `qr3` sites | role | verdict |
|---:|---|---:|---:|---:|---|---|
| 1 | `0x80A04D10` | 360 | 90 | 0 | `daEnBuku_c` per-frame state step that latches `StateID_SrcSwim` on a 120-unit L2 contact | **ready** |
| 2 | `0x80B28F90` | 356 | 89 | 0 | three-axis pivot matrix build with a plus/minus length offset around `+0x7C` | **ready** |
| 3 | `0x80A334D0` | 352 | 88 | 0 | `ankou` per-frame step: 185-frame timer, `-5`/`+12` lane offsets, jitter triple, direction probe | **ready** |
| 4 | `0x80B05740` | 328 | 82 | 0 | world-map rail-actor rebind plus live-count and per-link step recompute | **ready** |
| 5 | `0x80B25280` | 328 | 82 | 0 | x-distance effect row and lateral tilt (`ob_stream` family) | **ready** |
| 6 | `0x80B253D0` | 328 | 82 | 0 | y-distance twin of rank 5 with the pitch tilt call | **ready** |
| 7 | `0x80B2A360` | 324 | 81 | 0 | four-player x-band occupancy flag on the scene manager | **conditional** |
| 8 | `0x80AD5CD0` | 316 | 79 | 0 | contact-mask gate plus two-state emitter burst on the `+0x564` node | **ready** |
| 9 | `0x80A62A30` | 312 | 78 | 0 | clip-frame crossing at frame `18.0` latches four speed words | **ready** |
| 10 | `0x80AF16A0` | 312 | 78 | 0 | four-player proximity effect spawn (`id 60`, flags `0x2080`) | **ready** |
| 11 | `0x80A2FA30` | 308 | 77 | 0 | `0x6950`-byte actor constructor: vtable plus five count-500 arrays | **ready** |
| 12 | `0x80AC8850` | 304 | 76 | 0 | side-based `+0x414..+0x432` hit-record builder (template `0x80B30550`) | **ready** |

**Split: 11 ready, 1 conditional.** Total 3,928 bytes of TSV extent and 982
reachable instructions. Every one of the twelve walks to a terminal instruction at
exactly `entry + size - 4`, so `reachable == extent` for all twelve and the
"extent is padding" caveat does not apply here. The size cut is clean: 312 bytes
is claimed twice (ranks 9 and 10) and the next row is 308, so no tie-break rule
is needed.

Five of the twelve are close relatives of already-translated bodies, so their
record and call contracts are pinned by existing code rather than inferred here:
rank 3 is the `ankou` class built by `fn_80a32f10`, rank 9 sits in the translated
`gesso` block of `src/game/enemies.rs`, rank 12 writes the same `+0x414..+0x432`
hit record as `fn_80ab5e50`, and ranks 5/6 are siblings of the `Wm_ob_stream`
descriptor pair `fn_80b25020`/`fn_80b25130`.

### Rows below the cut

| address | TSV size | reachable | why it is not claimed |
|---|---:|---:|---|
| `0x80A390F0` | 300 | 75 | next in size order; clean, left for wave 150 |
| `0x80AEE850` | 296 | 74 | clean; single `0x8007FBF0` four-float call |
| `0x80A638F0` | 284 | 71 | clean; `gesso` tail with a `+0x704` angle write |
| `0x80A63A30` | 284 | 71 | clean; the twin of the row above |
| `0x80AEAED0` | 284 | 71 | clean; two `bctrl` sites, all callees in-module |
| `0x80A965D0` | 280 | 70 | clean; `pk`-model holder rebuild |
| `0x80B16A80` | 276 | 69 | clean; `0x800955F0` orbit integrate |
| `0x80A7AB90` | 272 | 68 | clean; `0x80198C30` sound with an `s16` block |

## The `qr3` pool at this wave

`docs/REL_ELF_MAPPING.md` pins **`GQR3 = 0x00050005`** (unsigned 16-bit, scale
0), installed by `main.dol` at `0x8025EEEC`/`0x8025EEF0`, and records that the
RAW stream of this REL contains no `mtspr GQRn`. A re-scan of the extents of all
72 remaining bodies finds that every `qr3` user is *below* this wave's cut:

| address | size | `qr3` sites | where |
|---|---:|---:|---|
| `0x80A80F40` | 264 | 1 | per-link angle (`0x80A80FD8`) |
| `0x80A80E50` | 220 | 1 | per-link angle (`0x80A80ECC`) |
| `0x80B22750` | 128 | 1 | single angle (`0x80B22784`) |

None of the twelve bodies claimed here reads `qr3`. All `psq` words in the twelve
are `qr0` `f31`/`f30` stack spills (`stfd`/`lfd` beside them) and carry no
quantisation, so nothing on this page depends on the `GQR3` word. With `W = 1`
and `I = 3` a data `psq` in this module would be one unsigned `u16` load scaled by
`2^0`; that form does not occur here.

## Reading conventions

* Synthetic base is `0x80A00000`. The REL has five file-backed `PT_LOAD`
  segments (`0x80A000F0`, `0x80B2A518`, `0x80B2A7A4`, `0x80B2A7B0`,
  `0x80B32DC0`); `docs/REL_ELF_MAPPING.md`'s `+0x1000` formula applies to the
  first. The file-backed image ends at `0x80B6E438`, so anything at or above the
  zero-fill segment `0x80B6E500` is **BSS** and is marked **(bss)** below.
* `this` is the incoming `r3` unless stated otherwise; `+0xNNN` are byte offsets
  from `this`. `+0xAC`/`+0xB0`/`+0xB4` are x/y/z; `+0x100`/`+0x102`/`+0x104`
  are the three `s16` angles; `+0x348` is the side/facing byte; `+0x60` is the
  class vtable; `+0xD0`/`+0xD4`/`+0xD8` are the three velocity components;
  `+0xE8`/`+0xEC`/`+0xF0` are the three speed components.
* `addi rD,r0,imm` (A field `0`) is `li`; `addis rD,r0,imm` is `lis`. The base
  is the literal zero, not `r0`. `addi r0,rS,imm` is a **real** add into `r0`.
* `rlwinm rA,rS,sh,mb,me` is `ROTL32(rS,sh) & mask(mb..me)` with mask bits
  numbered **from the MSB** (bit 0 = `0x80000000`, bit 31 = `1`). In particular
  `rlwinm r0,r3,20,28,31` is `(r3 >> 12) & 0xF` and `rlwinm r0,r3,5,31,31`
  is bit 27 of `r3`.
* On a `bc`, `bo = 12` branches on a **true** CR bit and `bo = 4` on a **false**
  one; `bi & 3` is LT/GT/EQ/SO. `cror cr0.eq,cr0.lt,cr0.eq` (`0x4C401382`,
  XO 449) rewrites `cr0.eq` as `lt|eq`, so the following `bne` reads
  `if !(a <= b)`; `cror cr0.eq,cr0.gt,cr0.eq` (`0x4C411382`) gives `gt|eq`.
* A-form floating point does **not** use a uniform third operand:
  `fadds`/`fsubs`/`fdivs` take `fB` (bits 16-20), while `fmuls` takes `fC`
  (bits 21-25) and `fmadds`/`fmsubs`/`fnmadds`/`fnmsubs`/`fsel` take
  `fD,fA,fC,fB`. Reading `fmuls` as `fB` turns the `x*x` squaring at
  `0x80A04DCC` into a multiply by the constant `14400.0`; both operands are
  `f4` there.
* `0x4E800421` is `bctrl` (`LK = 1`): an indirect call that **returns**. Only
  `0x4E800420` (LK = 0) is a tail `bcctr`. Likewise `bnelr` (`0x4D820020`)
  returns on the taken edge and falls through on the other.
* `0x43300000` in the high word with the value `^ 0x80000000` in the low word,
  minus the double at `0x80B2D458`/`0x80B30978`/`0x80B31AA0` (`2^52`), is the
  `u32 -> f64` conversion. `fctiw`/`fctiwz` truncate toward zero; the bodies
  below use `fctiwz` everywhere.

## Symbolic helpers used below

| address | symbol | contract used here |
|---|---|---|
| `0x8006A200` | `fn_8006a200(mask) -> u32` | `(*(u8 *)(r13 - 0x5916) & mask) != 0` (main.dol SDA flag byte) |
| `0x8005FFB0` | `fn_8005ffb0() -> u32` | game-mode code (main.dol, "GAME_MODE_CODE") |
| `0x8005F900` | `fn_8005f900(i) -> usize` | player object for slot `i`, `0` when absent |
| `0x80056CF0` | `fn_80056cf0(actor, id) -> u32` | character/state predicate (`id = 83` here) |
| `0x80064610` | `fn_80064610(id, param, &rec, flags, extra)` | effect spawn: `0x80064600(extra)` then `0x8006ca80(id, param, &rec, flags)` |
| `0x80064740` | `fn_80064740(actor, &out) -> usize` | predicted-position neighbour query; returns the matched actor or `0` (internally `0x8006CED0` + `0x80064790`, a jump-table dispatch through `*(r13-0x5984)`) |
| `0x8006CED0` | `fn_8006ced0(&out, actor)` | `out[0..3] = actor.pos + actor.vel` (`+0xAC..B4` + `+0xD0..D8`) |
| `0x80064A30` | `fn_80064a30(xa, xb) -> u32` | signed-x side code from two promoted `f32` |
| `0x80064AF0` | `fn_80064af0(&pos, &angles, 1)` | build/convert the three-angle frame (unledgered) |
| `0x800D8BB0` | `fn_800d8bb0(object, player, row, flags, extra)` | effect-display spawn: row record `0x80318C78 + row*0x14`, ORs `flags` into `object+0x30`, stores `player` at `+0x39` |
| `0x800D7980` / `0x800D79B0` | `fn_800d7980(node)` / `fn_800d79b0(node)` | start / stop one display node |
| `0x8012F810` / `0x8012F8E0` | `fn_8012f810(amount in f1, target)` / `fn_8012f8e0(...)` | short rotation setters: pitch / lateral (`amount` in `f1`, actor in `r3`) |
| `0x80162B00` | `fn_80162b00(id, layer, param, mode)` | effect-record spawn behind `0x8006CA80` |
| `0x801628C0` | `fn_801628c0(size) -> usize` | operator new |
| `0x8016A010` | `fn_8016a010(obj)` | store vtable `0x80329670` at `+0x00`, zero `+0x04` |
| `0x80166310` / `0x801662F0` | `fn_80166310(holder, idx)` / `fn_801662f0(holder, idx)` | with value in `f1`: `0x801680E0(value, elem)` / `0x80168FE0(value, elem)` on `elem = *(holder+0x28) + idx*0x38` |
| `0x80166360` | `fn_80166360(holder, idx) -> f32` | `elem = *(holder+0x28) + idx*0x38`; returns `*(f32 *)(elem + 0x28)` |
| `0x80168080` | `fn_80168080(holder) -> f32` | current clip frame via `(*(*(holder+4))[0x20])()` (unledgered) |
| `0x801680E0` | `fn_801680e0(holder, rate in f1)` | bind/advance a clip |
| `0x80169020` | `fn_80169020(holder) -> u32` | "clip finished" test |
| `0x8016ECB0` / `0x8016EDD0` / `0x8016EEF0` | `fn_8016ecb0(node, &s16)` / `...edd0` / `...eef0` | X / Y / Z rotation concat in place |
| `0x8016A170` / `0x8016A150` / `0x8016A1A0` / `0x8016A0F0` | scene-node setters | matrix / point / finalise / mode (`(node,2,1)`) |
| `0x801C0BD0` | `fn_801c0bd0(x, y, z, out)` | write a 3x4 translation matrix |
| `0x801C0540` | `fn_801c0540(a, b, out)` | 3x4 matrix concat `out = a * b` |
| `0x802DC9A0` | `fn_802dc9a0(base, ctor, dtor, stride, count)` | `__construct_array`: `count` elements of `stride` bytes |
| `0x80B04120` | `fn_80b04120(seg, name)` | bind one trailing segment to a clip name |
| `0x80A049D0` | `fn_80a049d0(actor)` | `daEnBuku_c` pitch wobble + base steps |
| `0x80A048D0` | `fn_80a048d0(actor) -> u32` | `daEnBuku_c` ground/landing probe (`+0x6E0` gate, `0x80097B30`) |
| `0x80A03C30` | `fn_80a03c30(actor)` | 24-frame periodic effect trigger (tail `0x80078470`) |
| `0x80A33630` / `0x80A337E0` / `0x80A343B0` | `fn_80a33630(this)` / `fn_80a337e0(this)` / `fn_80a343b0(this, &pos)` | `ankou` matrix rebuild / timer reset / direction write |
| `0x80A337B0` | `fn_80a337b0(this)` | 4-byte `b 0x80A337C0` thunk; target tail-calls `0x80078470(scene, 2, +0x6CC, +0x6D0, 4)` |
| `0x80AD54A0` | `fn_80ad54a0(this)` | sway/orbit integrate for the `0x4D40`-class container |
| `0x802DCD6C/74/B8/C0` | `_savegpr_N` / `_restgpr_N` | incoming `r3..r10` survive; an export's "no arguments" is the restore pair |

---

## 1. `0x80A04D10` - `daEnBuku_c` step that latches `StateID_SrcSwim`

* **Extent** 360 bytes (90 words), 90 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80a04d10_FUN_80a04d10.c`.
* **Signature** `unsafe fn fn_80a04d10(this: usize)` - `r3` only, returns `void`.
* **Role** one of the four `daEnBuku_c` per-state steps registered at
  `0x80A05500` (the four states are `0x80B6E7B8` `Swim`, `0x80B6E7F8`
  `SrcSwim`, `0x80B6E838` `AttackSwim`, `0x80B6E878` `AttackWait`). It runs the
  pitch/base step, rebinds the swim clip when the landing probe fires, and - while
  the actor is **not** mid-turn (`+0x6E0 == 0`) and within a 120-unit L2 distance
  of the actor returned by the neighbour query - switches to `SrcSwim`. It is the
  emitter that consumes `fn_80a049d0`/`fn_80a048d0`/`fn_80a03c30`.

### Behaviour

1. Spill `f31` (`stfd` + `psq_st ...,qr0`), `_savegpr_31`; `r31 = this`.
2. `fn_80a049d0(this)`.
3. `if fn_80a048d0(this) != 0 { fn_801680e0(this + 0x584, f1 = 0.5) }`.
4. `if *(u8 *)(this + 0x6E0) == 0`:
   1. `other = fn_80064740(this, &scratch)` (`&scratch` is `r1 + 8`); if
      `other == 0` skip to step 5.
   2. `sx = (*(f32 *)(other+0xAC) - *(f32 *)(this+0xAC)) + *(f32 *)(this+0xD0)`;
      `sy = (*(f32 *)(other+0xB0) - *(f32 *)(this+0xB0)) + *(f32 *)(this+0xD4)`;
      `sz = (*(f32 *)(other+0xB4) - *(f32 *)(this+0xB4)) + *(f32 *)(this+0xD8)`.
   3. `if sx*sx + sy*sy <= 14400.0 && (*(u32 *)(*(u32 *)(other+0x60) + 0x3D4))(other) == 0`
      then `(*(u32 *)(*(u32 *)(this+0x60) + 0xD4))(this, 0x80B6E7F8)`.
5. `(*(u32 *)(*(u32 *)(this+0x544) + 0x1C))(this + 0x544)`.
6. `fn_80a03c30(this)`.
7. `_restgpr_31`; return.

The twelve `stfs` at `0x80A04DAC..0x80A04E00` (`r1+0x10..0x3F`) build a 12-float
temporary that no call reads: the only indirect call in that range is the
argument-less `+0x3D4` slot on `other`. They are non-observable stack scratch and
a port may drop them.

### Constants

| value | meaning |
|---|---|
| `0x80B2A9FC` = `0.5` | clip rate re-applied after a landing |
| `0x80B2AA28` = `14400.0` | squared contact radius (`120^2`) |
| `0x80B6E7F8` | `daEnBuku_c::StateID_SrcSwim` state object **(bss)** |
| `+0x6E0` | turn-in-progress flag (set by `fn_80a04790`) |
| `+0x584` | animation holder |
| `+0x544` | embedded owner/list (vtable slot `+0x1C` = per-frame step) |
| `+0x60` | class vtable; `+0xD4` = `setState(this, state)` |
| `+0x3D4` (on `other`) | "not eligible" predicate, `0` means eligible |

### Port notes and verdict

`sx*sx + sy*sy` is **not** `(sx+sy)*14400`: the two `fmuls` at `0x80A04DCC`
and `0x80A04DD4` multiply `f4` by `f4` and `f1` by `f1` (the `fC` operand is
the destination register), and the compare at `0x80A04DE8` is
`fcmpo cr0, f1, f0` plus `cror eq,lt,eq`, i.e. `<=`. The `+0xD4` slot call is
the same state setter that `fn_80a05290` uses for the same state object. The
neighbour query selects the actor through a main.dol jump table, but this body
only consumes the returned pointer's position and its `+0x3D4` verdict, so the
selection rule does not need to be re-implemented. **Verdict: ready.**

## 2. `0x80B28F90` - three-axis pivot matrix build with a plus/minus length offset

* **Extent** 356 bytes (89 words), 89 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80b28f90_FUN_80b28f90.c`.
* **Signature** `unsafe fn fn_80b28f90(this: usize)` - `r3` only, returns `void`.
* **Role** rebuilds the `+0x7C` 3x4 matrix from the actor position and the three
  `s16` angles, rotating Y, translating `+d` on Y, rotating X, translating `-d`
  on Y, then rotating Z, where `d = 16.0 * *(f32 *)(this + 0x6FC)`. It then hands
  the matrix and the `+0xDC` point to the `+0x6B8` node wrapper and finalises it
  in mode `(2, 1)`.

### Behaviour

1. `pos = { *(f32 *)(this+0xAC), *(f32 *)(this+0xB0), *(f32 *)(this+0xB4) }`
   on the stack at `r1+0x1C..0x24`; `ang = { (s16)+0x100, (s16)+0x102,
   (s16)+0x104 }` at `r1+0x14..0x18`.
2. `d = f1 * *(f32 *)(this+0x6FC)` with `f1 = 16.0` (`0x80B32CF4`).
3. `fn_80064af0(&pos, &ang, 1)` - build the angle frame in place.
4. `fn_801c0bd0(pos.x, pos.y + d, pos.z, this + 0x7C)`.
5. `fn_8016edd0(this + 0x7C, &ang[1])` (Y rotation).
6. `fn_801c0bd0(0.0, d, 0.0, &m1)` then
   `fn_801c0540(this + 0x7C, &m1, this + 0x7C)`.
7. `fn_8016ecb0(this + 0x7C, &ang[0])` (X rotation).
8. `fn_801c0bd0(0.0, -d, 0.0, &m2)` then
   `fn_801c0540(this + 0x7C, &m2, this + 0x7C)`.
9. `fn_8016eef0(this + 0x7C, &ang[2])` (Z rotation).
10. `fn_8016a170(this + 0x6B8, this + 0x7C)`;
    `fn_8016a150(this + 0x6B8, this + 0xDC)`;
    `fn_8016a1a0(this + 0x6B8, 0)`;
    `fn_8016a0f0(this + 0x6B8, 2, 1)`.

### Constants

| value | meaning |
|---|---|
| `0x80B32CF4` = `16.0` | length multiplier applied to `+0x6FC` |
| `0x80B32CE0` = `0.0` | x and z of both translation matrices |
| `+0x6FC` | `f32` length (rail/pivot half-length) |
| `+0x7C` | 3x4 matrix under construction |
| `+0xDC` | point handed to the node wrapper |
| `+0x6B8` | node wrapper (matrix / point / finalise) |

### Port notes and verdict

The two offset matrices are `(0, +d, 0)` and `(0, -d, 0)`, not `(d,0,0)`:
`fn_801c0bd0` receives `f1 = 0.0`, `f2 = ±d`, `f3 = 0.0`. The rotation order is
Y, translate, X, translate, Z - identical to `fn_80a33630`'s documented order in
`src/game/enemies.rs`. `+0x100`/`+0x102`/`+0x104` are `s16` (`lha`), not floats.
**Verdict: ready.**

## 3. `0x80A334D0` - `ankou` per-frame step

* **Extent** 352 bytes (88 words), 88 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80a334d0_FUN_80a334d0.c`.
* **Signature** `unsafe fn fn_80a334d0(this: usize)` - `r3` only, returns `void`.
* **Role** per-frame step of the `ankou` actor constructed by `fn_80a32f10`
  (class vtable `0x80B3D558`, `0x6E0` bytes). It refreshes the model matrix,
  arms the spawn timer while the actor is not mid-turn, advances a 185-frame
  clamp, writes the two lane offsets `-5`/`+12`, runs the `+0x6A0` sub-object,
  copies the jitter triple and pushes the direction target.

### Behaviour

1. Spill `f31`, `_savegpr_30`; `r30 = this`; `r31 = 0x80B2BF48` (data block
   base, used as `r31+0x10` and `r31+0x88`).
2. `fn_80a33630(this)` - rebuild `+0x7C` (matrix) from `+0xAC..B4`,
   `+0x100..104` and the `+0x6A0` node.
3. `if *(u8 *)(this + 0x390) == 0 && fn_8006a200(1) != 0`:
   `fn_80a337b0(this)` (thunk to `0x80A337C0`, which tail-calls
   `0x80078470(*(0x80429DD0), 2, f1 = +0x6CC, f2 = +0x6D0, 4)`), then
   `fn_80a337e0(this)`.
4. `t = *(f32 *)(this+0x6C8)`; `if t < 185.0 { v = t + 1.0; *(f32 *)(this+0x6C8) = v;
   if v > 185.0 { *(f32 *)(this+0x6C8) = 185.0 } }`.
5. `*(f32 *)(this+0x6A4) = *(f32 *)(this+0x6BC) - 5.0`;
   `*(f32 *)(this+0x6A8) = *(f32 *)(this+0x6C0) + 12.0`;
   `*(f32 *)(this+0x6AC) = *(f32 *)(this+0x6C4)`;
   `*(f32 *)(this+0x6B0) = 185.0`.
6. `(*(u32 *)(*(u32 *)(this+0x6A0) + 0xC))(this + 0x6A0)`.
7. `*(f32 *)(this+0x65C) = *(f32 *)0x80B701C0 + 0.0`;
   `*(f32 *)(this+0x660) = *(f32 *)0x80B701C4 + 0.0`;
   `*(f32 *)(this+0x664) = *(f32 *)0x80B701C8 + 0.0`.
8. `fn_80a343b0(this + 0x5BC, &p)` with
   `p = { *(f32 *)(this+0x6BC), *(f32 *)(this+0x6C0),
   *(f32 *)(this+0x6C4) + 96.0 }`.
9. `_restgpr_30`; return.

### Constants

| value | meaning |
|---|---|
| `0x80B2BF50` = `0.0` | additive term for the jitter triple (`f31`) |
| `0x80B2BF60` = `185.0` | timer cap and `+0x6B0` value |
| `0x80B2BF64` = `1.0` | timer step |
| `0x80B2BF68` = `-5.0` | `+0x6A4` lane offset |
| `0x80B2BF6C` = `12.0` | `+0x6A8` lane offset |
| `0x80B2BFD0` = `96.0` | forward probe distance added to `+0x6C4` |
| `0x80B701C0`/`C4`/`C8` | jitter triple **(bss)**; copied by `fn_80a32f10` into `+0x65C..664` and re-added here |
| `0x8006A200(1)` | SDA flag-bit test that gates the spawn arming |
| `+0x6A0` | sub-object whose slot `+0x0C` is stepped each frame |
| `+0x6BC`/`+0x6C0`/`+0x6C4` | authoritative position triple |
| `+0x6A4`/`+0x6A8`/`+0x6AC`/`+0x6B0` | derived lane/target record |
| `+0x6C8` | `f32` timer, clamped to `185.0` |
| `+0x65C`/`+0x660`/`+0x664` | jitter offset triple |

### Port notes and verdict

Only `+0x694`-style `fcmpo`+`bc` pairs are used, and both clamp edges are exact
(`< 185.0` to enter, `> 185.0` to clamp). The three `+0x65C..664` stores are an
identity copy: the only writer of the `0x80B701C0..C8` block in this REL is the
constructor `fn_80a32f10`, which copies the same words, and the additive term is
`0.0`; a port may keep the stores or fold them, behaviour is unchanged. The three
`stfs` at `0x80A33594..0x80A335A0` before the `+0x6A0` virtual are dead stack
scratch (the virtual takes only `r3`). **Verdict: ready.**

## 4. `0x80B05740` - world-map rail-actor rebind and count recompute

* **Extent** 328 bytes (82 words), 82 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80b05740_FUN_80b05740.c`.
* **Signature** `unsafe fn fn_80b05740(this: usize)` - `r3` only, returns `void`.
* **Role** the rebind half of the world-map rail actor whose step is
  `0x80B058A0` (wave 148, rank 1). It binds `"create"`/`"ridden"` to the
  `+0x540`/`+0x6FC`/`+0x8B8` segments, seeds the two animation values, copies the
  three saved vectors, then recomputes the per-link step and the live-count cap
  from the animation element.

### Behaviour

1. `_savegpr_27`; `r27 = this`; `r29 = 0x80B31A50`; `r30 = 0x80B60000`
  (the `"create"`/`"ridden"` literals are formed as `r30/r31 + 0x6490/0x6478`).
2. `fn_80b04120(this + 0x540, 0x80B66490 /* "create" */)`.
3. `fn_80166310(this + 0x588, 0)` with `f1 = 0.0` (`0x80B31A58`).
4. `fn_801662f0(this + 0x588, 0)` with `f1 = 25.0` (`0x80B31AAC`).
5. `fn_80b04120(this + 0x6FC, 0x80B66478 /* "ridden" */)`.
6. `fn_80b04120(this + 0x8B8, 0x80B66490)`.
7. `for (i = 1; i < *(s32 *)(this + 0x2B18); i++)`
   `fn_80b04120(this + 0xA70 + (i-1)*0x1B8, 0x80B66478)`.
8. `div = *(f32 *)(0x80B31A84 + ((*(u32 *)(this+4) >> 6) & 0xC))`;
   `step = (s32)fctiwz(16.0 / div)`.
9. `*(u32 *)(this+0x5FC) = *(u32 *)(this+0x5E4)`;
   `*(u32 *)(this+0x600) = *(u32 *)(this+0x5E8)`;
   `*(u32 *)(this+0x604) = *(u32 *)(this+0x5EC)`;
   `*(u32 *)(this+0x7B8) = *(u32 *)(this+0x7A0)`;
   `*(u32 *)(this+0x7BC) = *(u32 *)(this+0x7A4)`;
   `*(u32 *)(this+0x7C0) = *(u32 *)(this+0x7A8)`.
10. `*(s32 *)(this+0x2B34) = 0`.
11. `frame = fn_80166360(this + 0x588, 0)`;
    `*(s32 *)(this+0x2B38) = (s32)fctiwz(frame / (f32)step)`.

### Constants

| value | meaning |
|---|---|
| `0x80B31A58` = `0.0` | first animation value |
| `0x80B31AAC` = `25.0` | second animation value |
| `0x80B31A74` = `16.0` | step numerator |
| `0x80B31A84` = `{0.5, 1.0, 1.4, 1.0}` | step divisor, index `(object_id >> 6) & 0xC` |
| `0x80B31AA0` = `2^52` (double) | `u32 -> f64` fold for the step |
| `0x80B66490` = `"create"`, `0x80B66478` = `"ridden"` | clip names |
| `+0x540`, `+0x6FC`, `+0x8B8` | rail, ridden segment, first trailing segment |
| `+0x1B8` / `+0xA70` | trailing-segment stride / second-segment base |
| `+0x2B18` | segment count |
| `+0x2B34` / `+0x2B38` | live count / count cap |
| `+0x588` | 0x38-stride animation holder (element 0) |

### Port notes and verdict

`fn_80166310` binds the element through `0x801680E0` (rate form) while
`fn_801662f0` writes it through `0x80168FE0` (value form), so the two calls are
not duplicates. The loop starts at `i = 1` because `+0x8B8` is the first segment
and `+0xA70 = +0x8B8 + 0x1B8`. `fctiwz` truncates; `16.0/1.4` is `11`, not
`11.43`. The final division uses `(f32)step`, so `frame / step` is exact for the
table's integer steps. **Verdict: ready.**

## 5. `0x80B25280` - x-distance effect row and lateral tilt

* **Extent** 328 bytes (82 words), 82 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80b25280_FUN_80b25280.c`.
* **Signature** `unsafe fn fn_80b25280(this: usize, other: usize)` - `r3` is the
  emitter, `r4` the tracked object; returns `void`. The export prints
  `void FUN_80b25280(void)` because `_savegpr_27` is modelled as a 64-bit return;
  `r30 = r3` and `r31 = r4` in the stream.
* **Role** one of the two distance-banded effect steps of the world-map
  `Wm_ob_stream` descriptor family (`fn_80b25020`/`fn_80b25130`). It measures the
  predicted **x** separation to `other`, picks a `1.3 * table[bucket]` amount, and
  arms a per-player effect row and a lateral tilt.

### Behaviour

1. `_savegpr_27`; `r30 = this`; `r31 = other`.
2. `fn_8006ced0(&p, other)` - `p = other.pos + other.vel`.
3. `i = (s32)fctiwz(|p.x - *(f32 *)(this+0xAC)| * 0.0625)`.
4. `amount = 1.3 * *(f32 *)(0x80B32970 + i*4)`.
5. `player = (s8)*(u8 *)( (*(u32 *)(*(u32 *)(other+0x60) + 0x6C))(other) )`.
6. `slot = this + player*4 + 0x664`; `n = *(s32 *)slot - 1; *(s32 *)slot = n`.
7. `if n < 1`:
   * `row = *(s32 *)(0x80B329B0 + i*4)` when `*(s32 *)(other+0x1090) == 5`,
     else `row = *(s32 *)(0x80B32990 + i*4)`;
   * `fn_800d8bb0(*(u32 *)0x8042A008, player, row, 0, 0)`;
   * `*(s32 *)slot = 8`.
8. `if *(s32 *)(this+0x524) == 3` then `fn_8012f8e0(-amount, other)` else
   `fn_8012f8e0(amount, other)`.
9. `_restgpr_27`; return.

### Constants

| value | meaning |
|---|---|
| `0x80B32970` | `f32[8]` = `{0.1015625, 0.1015625, 0.0703125, 0.0703125, 0.046875, 0.0390625, 0.0390625, 0.0390625}` |
| `0x80B329DC` = `1.3` | amount multiplier |
| `0x80B32A80` = `0.0625` | distance -> bucket scale (`1/16`) |
| `0x80B32990` | `s32[8]` = `{13,13,13,14,14,14,14,14}` (normal rows) |
| `0x80B329B0` | `s32[8]` = `{15,15,15,16,16,16,16,16}` (`other+0x1090 == 5`) |
| `0x8042A008` | global effect-display object (main.dol) |
| `+0x664` | per-player counter array (index `player`, stride 4) |
| `+0x524` | variant/state word (`3` flips the tilt sign) |
| `+0x60` / `+0x6C` (on `other`) | vtable / "player byte" accessor |
| `+0x1090` (on `other`) | mode word selecting the row table |

### Port notes and verdict

The distance is `|p.x - this.x|`; there is **no clamp** on `i`, so an
out-of-band separation reads past the eight-entry tables - this is faithful
behaviour of the original and should not be "fixed" in a port. The counter is
re-armed to `8` on every emission and decremented once per call, so the effect
fires at most every eighth call per player. `fn_8012f8e0` is the lateral setter
with the amount in `f1` and `other` in `r3` (see `fn_80b21ac0`). **Verdict: ready.**

## 6. `0x80B253D0` - y-distance twin with the pitch tilt call

* **Extent** 328 bytes (82 words), 82 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80b253d0_FUN_80b253d0.c`.
* **Signature** `unsafe fn fn_80b253d0(this: usize, other: usize)` - same
  register roles as rank 5; returns `void`.
* **Role** the vertical twin of rank 5: identical tables, counters and rows, but
  it measures the **y** separation, flips the sign on `+0x524 == 1` and uses the
  pitch setter `0x8012F810`.

### Behaviour

Identical to rank 5 except for three words:

1. step 3 reads `p.y` and `*(f32 *)(this+0xB0)`:
   `i = (s32)fctiwz(|p.y - *(f32 *)(this+0xB0)| * 0.0625)`.
2. step 8 tests `*(s32 *)(this+0x524) == 1`.
3. step 8 calls `fn_8012f810(±amount, other)`.

Everything else - `fn_8006ced0`, the `1.3` multiplier, both row tables, the
`+0x664` counter and the `fn_800d8bb0` re-arm to `8` - is instruction-for-word
identical and uses the same constants.

### Constants

Same table block as rank 5 (`0x80B32970`, `0x80B32990`, `0x80B329B0`,
`0x80B329DC`, `0x80B32A80`, `0x8042A008`), plus `+0xB0` and `+0x524 == 1`.

### Port notes and verdict

The two bodies are not interchangeable: rank 5 feeds the lateral setter and rank
6 the pitch setter, and the `+0x524` constant differs (`3` vs `1`). Everything
else is shared, so a port should factor the common tail and pass the axis and the
sign-state constant. **Verdict: ready.**

## 7. `0x80B2A360` - four-player x-band occupancy flag

* **Extent** 324 bytes (81 words), 81 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80b2a360_FUN_80b2a360.c`.
* **Signature** `unsafe fn fn_80b2a360(this: usize) -> u32` - `r3` only, always
  returns `1`.
* **Role** clears a scene-manager flag, then - when the game mode is not `1` and
  the mode-manager triple at `+0x120C..0x120E` reads `(1, 4, 0)` - scans the four
  player objects and raises the flag when any player's `+0x152C` coordinate lies
  inside the actor's x band `[x, x + 16.0 * *(f32 *)(this+0x524)]`.

### Behaviour

1. Spill `f31`/`f30`; `f1 = 16.0`; `f0 = *(f32 *)(this+0x524)`;
   `f31 = *(f32 *)(this+0xAC)`; `f30 = f31 + 16.0 * f0`.
2. `stage = *(u32 *)0x80429DD0`; `*(u8 *)(stage + 0x909CB) = 0`.
3. `if fn_8005ffb0() != 1` **and** `mgr = *(u32 *)0x8042A1C8` with
   `*(u8 *)(mgr+0x120C) == 1 && *(u8 *)(mgr+0x120D) == 4 && *(u8 *)(mgr+0x120E) == 0`:
   1. `n = 0`; `for (i = 0; i < 4; i++)`:
      `p = fn_8005f900(i)`; if `p != 0` and
      `*(f32 *)(p+0x152C) >= f31` and `*(f32 *)(p+0x152C) <= f30` then `n++`.
   2. `if n != 0 { *(u8 *)(stage + 0x909CB) = 1 }`.
4. `return 1`.

### Constants

| value | meaning |
|---|---|
| `0x80B32DB0` = `16.0` | band width multiplier |
| `0x80429DD0` | scene manager pointer; `+0x909CB` is the occupancy byte (main.dol) |
| `0x8042A1C8` | mode-manager pointer; `+0x120C`/`+0x120D`/`+0x120E` read `1`/`4`/`0` (main.dol) |
| `0x8005FFB0` | game-mode code |
| `+0x524` | `f32` band scale for this class (note: an `s32` state word in the `0x80B25xxx` family) |
| `+0xAC` | x origin |
| `+0x152C` (on a player) | compared coordinate |

### Port notes and verdict

The mode test is `fn_8005ffb0() != 1`. The stream computes
`r0 = mode - 1`, `clz(r0)`, then `rlwinm. r0,r0,27,5,31`, which is non-zero
exactly when `clz == 32`, i.e. when `mode == 1`; CR0.EQ is then clear and the
following `bc 4,2` **takes** the branch to the epilogue, so the body is skipped
for mode `1` and runs for every other mode. The final `addi r3,r0,1` sets the
return value.

The one unresolved item is semantic, not structural: `0x8042A1C8 + 0x120C..0x120E`
and `0x80429DD0 + 0x909CB` live in `main.dol` and neither this REL nor any
existing doc records what they select. The comparison constants, the band
arithmetic, the four-slot loop and the single flag store are exact, so the body
can be typed as soon as those four fields are named. **Verdict: conditional** -
one bounded step: name the mode triple and the scene flag.

## 8. `0x80AD5CD0` - contact-mask gate and two-state emitter burst

* **Extent** 316 bytes (79 words), 79 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80ad5cd0_FUN_80ad5cd0.c`.
* **Signature** `unsafe fn fn_80ad5cd0(this: usize)` - `r3` only, returns `void`.
* **Role** the mask-gated handler of the `0x4D40`-class container (the class whose
  `+0x4C2C`/`+0x4C2D` side bytes and `+0x564` node are described by `fn_80ad6ed0`
  and `fn_80ad6f50`). It returns immediately unless one of the two contact masks
  intersects `+0x358`/`+0x35C`, then sways, applies the two side offsets through
  the class virtual `+0x280`, and selects one of two emitter states.

### Behaviour

1. `p = *(u32 *)0x8042A078`; `if ((*(u32 *)(this+0x358) & p[0]) |
   (*(u32 *)(this+0x35C) & p[1])) == 0` return.
2. `fn_80ad54a0(this)` - sway/orbit integrate.
3. `a = 16.0 * (f32)(s8)*(u8 *)(this+0x4C2C)`;
   `b = 16.0 * (f32)(s8)*(u8 *)(this+0x4C2D)`.
4. `(*(u32 *)(*(u32 *)(this+0x60) + 0x280))(f1 = a, f2 = b, r3 = this)`.
5. `if ((*(u32 *)(this+4) >> 12) & 0xF) == 0`:
   `(*(u32 *)(*(u32 *)(this+0x60) + 0xD4))(this, 0x80B766B0)`; return.
6. `else if ((*(u32 *)(this+4) >> 27) & 1) == 0`:
   `fn_800d79b0(this + 0x564)`; `(*(...+0xD4))(this, 0x80B76730)`; return.
7. `else`: `fn_800d7980(this + 0x564)`; `(*(...+0xD4))(this, 0x80B76730)`; return.

### Constants

| value | meaning |
|---|---|
| `0x8042A078` | main.dol pointer to the two contact masks (`p[0]`, `p[1]`) |
| `0x80B30964` = `16.0` | side offset scale |
| `0x80B30978` = `2^52` (double) | `s8 -> f64` fold |
| `0x80B766B0` | first state object **(bss)** |
| `0x80B76730` | second state object **(bss)** |
| `+0x358` / `+0x35C` | two contact words tested against the masks |
| `+0x4C2C` / `+0x4C2D` | two `s8` side bytes (`0x10` units each) |
| `+0x564` | display node started/stopped by `0x800D7980`/`0x800D79B0` |
| `+0x60` | vtable; `+0x280` = two-float apply, `+0xD4` = set state |
| `+4` bits 12-15 / bit 27 | variant nibble / second-variant bit |

### Port notes and verdict

The two side values are `(f32)(s8)(u8)` - `lbz` then `extsb` - so an index byte
of `0xFF` is `-1`, not `255`; the `2^52` fold is the same one used by
`fn_80ad6ed0`. The mask is read through a pointer (`lwz r4,-0x5F88(r4)` then two
`lwz`), not from two adjacent globals. `fn_800d7980`/`fn_800d79b0` are the
start/stop pair documented in `docs/COVERAGE_WAVE95_ENEMIES.md`.
**Verdict: ready.**

## 9. `0x80A62A30` - clip-frame crossing at frame `18.0`

* **Extent** 312 bytes (78 words), 78 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80a62a30_FUN_80a62a30.c`.
* **Signature** `unsafe fn fn_80a62a30(this: usize)` - `r3` only, returns `void`.
* **Role** per-frame step in the translated `gesso` block (`fn_80a61b60`,
  `fn_80a61b20`, `fn_80a61e60`, `fn_80a61d90`). It reads the `+0x584` clip frame
  before and after the `+0x544` sub-object step; when the frame crosses `18.0`
  upward it writes the four speed words `+0x114`, `+0x11C`, `+0xF4`, `+0xF8`,
  then runs the base steps and leaves to a BSS state when the clip finishes.

### Behaviour

1. `_savegpr_31`; `r31 = this`.
2. `fn_80a61b60(this)`; `fn_80a61b20(this)`.
3. `before = f1 = fn_80168080(this + 0x584)`.
4. `(*(u32 *)(*(u32 *)(this+0x544) + 0x1C))(this + 0x544)`.
5. `after = f1 = fn_80168080(this + 0x584)`.
6. `if after >= 18.0 && before < 18.0`:
   1. `side = (s8)*(u8 *)(0x8042C1A0 + *(u8 *)(this+0x348))` via
      `lbzx`/`extsb`; `sv = (f32)side`.
   2. `*(f32 *)(this+0x11C) = 0.06`;
      `*(f32 *)(this+0x114) = 0.125`;
      `*(f32 *)(this+0xF4) = sv * 0.5`;
      `*(f32 *)(this+0xF8) = -0.55`.
7. `fn_8006cd90(this)`; `fn_80a61e60(this)`; `fn_8006cd40(this)`;
   `fn_80a61d90(this)`.
8. `if fn_80169020(this + 0x584) != 0`
   then `(*(u32 *)(*(u32 *)(this+0x60) + 0xD4))(this, 0x80B71F50)`.

### Constants

| value | meaning |
|---|---|
| `0x80B2D468` = `18.0` | clip-frame threshold |
| `0x80B2D458` = `2^52` (double) | `s32 -> f64` fold for the side table value |
| `0x80B2D2C0` = `0.06` | `+0x11C` |
| `0x80B2D2C4` = `0.125` | `+0x114` |
| `0x80B2D2C8` = `0.5` | `+0xF4` multiplier |
| `0x80B2D2CC` = `0.55` | negated into `+0xF8` |
| `0x8042C1A0` | main.dol `s8` side table; entries `{+1,-1,0,0,+32,0,-32,0,+4,0,-4,0,0,0,0,0}` |
| `0x80B71F50` | state object **(bss)** |
| `+0x348` | side byte indexing the table |
| `+0x584` | clip holder |
| `+0x544` | embedded owner (slot `+0x1C` steps it) |

### Port notes and verdict

The crossing test is strict on the low side and inclusive on the high side:
`after >= 18.0` (`fcmpo` + `cror eq,gt,eq`) and `before < 18.0` (`bc 4,0` on
LT). The side value is `s8`, so the `+32`/`-32` entries are `32`/`-32`, and only
the `+1`/`-1` entries are reachable for the actor's `0`/`1` side convention. The
`0.55` is negated by `fneg` before the store, so `+0xF8` is `-0.55`.
**Verdict: ready.**

## 10. `0x80AF16A0` - four-player proximity effect spawn

* **Extent** 312 bytes (78 words), 78 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80af16a0_FUN_80af16a0.c`.
* **Signature** `unsafe fn fn_80af16a0(this: usize)` - `r3` only, returns `void`.
* **Role** walks the four-slot word array at `+0x67C` and, for each live player
  that passes four gates, spawns effect `60` at a two-point record derived from the
  player position, with the per-player flags `(i << 16) | 0x2080 | *(u32 *)(this +
  0x68C)`.

### Behaviour

1. `_savegpr_25` (`r25..r31`); `r28 = this`.
2. `f30 = 2.0` (`0x80B31338`); `f31 = 0.0` (`0x80B312F8`).
3. `for (i = 0; i < 4; i++, r28 += 4)`:
   1. `if *(s32 *)(r28 + 0x67C) == 0` continue.
   2. `p = fn_8005f900(i)`; if `p == 0` continue.
   3. `if (*(u8 *)0x80429CA8 & (1 << i)) == 0` continue.
   4. `if fn_80056cf0(p, 83) != 0` continue.
   5. `if (*(u32 *)(*(u32 *)(p+0x60) + 0xE0))(p) != 0` continue.
   6. `rec[0] = *(f32 *)(p+0xAC)`;
      `rec[1] = *(f32 *)(*(u32 *)0x80429E00 + 0x3C) + 2.0`;
      `rec[2] = 0.0`;
      `rec[3] = *(f32 *)(p+0xAC)`;
      `rec[4] = *(f32 *)(p+0xB0)`;
      `rec[5] = *(f32 *)(p+0xB4)`.
   7. `fn_80064610(60, *(u32 *)(this+0x68C) | (i << 16) | 0x2080, &rec, 0, 0)`.
4. `_restgpr_25`; return.

### Constants

| value | meaning |
|---|---|
| `0x80B31338` = `2.0` | y lift added to the scene height |
| `0x80B312F8` = `0.0` | z of the first record point |
| `0x80429CA8` | main.dol per-player byte; bit `i` must be set |
| `0x80429E00` | scene/camera object pointer; `+0x3C` is a height (main.dol) |
| `60` | effect id passed to `0x80064610` |
| `83` | character/state id passed to `0x80056CF0` |
| `0x2080` | fixed flag half of the effect parameter |
| `+0x67C` | four `s32` per-player enable words (`+0x67C..0x688`) |
| `+0x68C` | shared effect parameter word |
| `+0x60` / `+0xE0` | player vtable / "eligible" predicate |

### Port notes and verdict

`r28` advances by 4 while `r26` counts to 4, so `+0x67C` is a **four-word
array**, not a scalar: the loop reads `+0x67C`, `+0x680`, `+0x684`, `+0x688`.
`r27` accumulates `0x10000` per iteration (`addis r27,r27,0x1`), so the parameter
is `(i << 16) | 0x2080 | *(u32 *)(this+0x68C)`. The record is six floats at
`r1+8..0x20`; the first point uses the scene height `+2.0` and the second uses
the player's own y. **Verdict: ready.**

## 11. `0x80A2FA30` - `0x6950`-byte actor constructor

* **Extent** 308 bytes (77 words), 77 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80a2fa30_FUN_80a2fa30.c`.
* **Signature** `unsafe fn fn_80a2fa30() -> usize` - no arguments, returns the new
  actor or `0`.
* **Role** allocates `0x6950` bytes, runs the base actor constructor, installs class
  vtable `0x80B3C838` at `+0x60`, constructs the `+0x524` sub-object and the
  `+0x544` owner (vtable `0x80B3CAB8`), then builds five count-500 arrays and one
  four-element array of `0x0C`-byte records. It is the allocating constructor the
  class's step functions (including the `+0x414` record builder in rank 12) run
  behind.

### Behaviour

1. Frame `0x20`; `base = fn_801628c0(0x6950)`; if `base == 0` return `0`.
2. `fn_80094e80(base)` (base actor constructor).
3. `*(u32 *)(base+0x60) = 0x80B3C838`.
4. `fn_80069020(base + 0x524)`; `*(u32 *)(base+0x540) = 0`.
5. `fn_8016a010(base + 0x544)`; `*(u32 *)(base+0x544) = 0x80B3CAB8`.
6. `fn_802dc9a0(base + 0x56C, 0x80A2FC30, 0x80A00240, 8, 500)`;
   `fn_802dc9a0(base + 0x150C, 0x80A2FC30, 0x80A00240, 8, 500)`.
7. `fn_802dc9a0(base + 0x2C7C, 0x80A2FC40, 0x80A22AE0, 4, 500)`;
   `fn_802dc9a0(base + 0x344C, 0x80A2FC40, 0x80A22AE0, 4, 500)`.
8. `fn_802dc9a0(base + 0x4BBC, 0x80A2FC50, 0x80A00200, 12, 500)`.
9. `fn_802dc9a0(base + 0x6910, 0x80A2FC50, 0x80A00200, 12, 4)`.
10. Return `base`.

### Constants

| value | meaning |
|---|---|
| `0x6950` | allocation size |
| `0x80B3C838` | class vtable stored at `+0x60` |
| `0x80B3CAB8` | owner vtable stored at `+0x544` |
| `0x80A2FC30` | empty element constructor (`blr`) |
| `0x80A2FC40` | element constructor: `*(u32 *)(elem+0) = 0xFFFFFFFF` |
| `0x80A2FC50` | empty `0x0C`-stride element constructor (`blr`) |
| `0x80A00240`, `0x80A22AE0`, `0x80A00200` | element destructors (all ledgered) |
| `+0x524` | `0x80069020` sub-object |
| `+0x544` | owner/list with vtable `0x80B3CAB8` |

### Port notes and verdict

`fn_802dc9a0(base, ctor, dtor, stride, count)` is the `__construct_array` shape
confirmed by `src/runtime/cpp.rs`, so the five descriptors are
`stride 8 x 500`, `stride 8 x 500`, `stride 4 x 500`, `stride 4 x 500`,
`stride 12 x 500`, plus `stride 12 x 4`. The arrays tile the allocation exactly:
`0x56C + 0xFA0 = 0x150C`, `0x150C + 0xFA0 = 0x24AC` (gap to `0x2C7C`),
`0x2C7C + 0x7D0 = 0x344C`, `0x344C + 0x7D0 = 0x3C1C` (gap to `0x4BBC`),
`0x4BBC + 0x1770 = 0x632C` (gap to `0x6910`), `0x6910 + 0x30 = 0x6940 <= 0x6950`.
The two gaps hold no constructed array and are not touched here. `0x80094E80`
is called with `r3 = base` (`fn_80a32f10` prints it as argument-less). The class
identity beyond the two vtable words is not recorded in this repo, but every
store and descriptor is literal, so the constructor is implementable without it.
**Verdict: ready.**

## 12. `0x80AC8850` - side-based `+0x414..+0x432` hit-record builder

* **Extent** 304 bytes (76 words), 76 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80ac8850_FUN_80ac8850.c`.
* **Signature** `unsafe fn fn_80ac8850(this: usize, other: usize, param: usize,
  scale: f32)` - `r3 = this`, `r4 = other`, `r5 = param`, incoming `f1 = scale`;
  returns `void`. The export prints `void FUN_80ac8850(a, b, c)` and reads the
  scale from `extraout_f1`; the stream carries it in `f31` (`fmr f31,f1`).
* **Role** builds the same seven-word hit/attack record that `fn_80ab5e50` copies
  from `0x80B2FD30`, but derives it: entry `+0x414` is
  `(±1 + 0.4 * this.x-speed) * scale`, and the remaining words are the
  `0x80B30550` template plus the side byte, the player byte and the activation
  flag.

### Behaviour

1. `_savegpr_27`; `r27 = this`; `r28 = other`; `r29 = param`; `f31 = f1`.
2. `side = (u8)fn_80064a30( *(f32 *)(this+0xAC) + *(f32 *)(this+0xD0),
   *(f32 *)(other+0xAC) + *(f32 *)(other+0xD0) )` (`clrlwi r30,r3,24`).
3. `pbyte = *(u8 *)( (*(u32 *)(*(u32 *)(other+0x60) + 0x6C))(other) )`.
4. `v = (*(f32 *)(0x80B30558 + (side & 3)*4) + 0.4 * *(f32 *)(this+0xE8)) * f31`.
5. `*(f32 *)(this+0x414) = v`;
   `*(u32 *)(this+0x418) = 0x40000000` (`2.0`);
   `*(u32 *)(this+0x41C) = 0xC0C00000` (`-6.0`);
   `*(u32 *)(this+0x420) = 0xBE400000` (`-0.1875`);
   `*(u32 *)(this+0x424) = 0x80B75F30`;
   `*(u32 *)(this+0x428) = r29` (param);
   `*(u32 *)(this+0x42C) = 0xFFFFFFFF`;
   `*(u8 *)(this+0x430) = side`;
   `*(u8 *)(this+0x431) = pbyte`;
   `*(u8 *)(this+0x432) = 1`.
6. `_restgpr_27`; return.

### Constants

| value | meaning |
|---|---|
| `0x80B30558` | `f32[2]` = `{+1.0, -1.0}`, indexed by `side` |
| `0x80B30580` = `0.4` | x-speed multiplier |
| `0x80B30564` = `2.0` | `+0x418` |
| `0x80B30568` = `-6.0` | `+0x41C` |
| `0x80B3056C` = `-0.1875` | `+0x420` |
| `0x80B30570` = `0x80B75F30` | `+0x424` (BSS object pointer) |
| `0x80B30578` = `0xFFFFFFFF` | `+0x42C` |
| `+0xE8` | this x-speed |
| `+0xAC` / `+0xD0` | x / x-velocity of each actor |
| `+0x414..+0x432` | the hit/attack record (same block as `fn_80ab5e50`) |

### Port notes and verdict

The multiplier order is `(table + 0.4*vx) * scale`; `fmuls f3,f3,f31` at
`0x80AC88F8` uses `fC = f31`, the incoming `f1`, which the export loses as
`extraout_f1`. `0x80064A30` returns the side code in `r3`, masked to a byte;
`rlwinm r0,r30,2,22,29` then indexes `0x80B30558 + side*4`, and only the `0`/`1`
entries (`+1.0`/`-1.0`) are meaningful. The nine stack stores at
`0x80AC8948..0x80AC8968` mirror the record but are not read by any call and can be
dropped. `+0x424` is a **pointer** (`stw`), not a float. **Verdict: ready.**

---

## Verification

* Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 12 sizes sum to 3,928
  bytes and the twelve walks reach 982 instructions, i.e. `reachable == extent`
  for every row (`entry + size - 4` is the last word of each walk).
* Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
* All twelve addresses are absent from `ledger/functions.csv`, from every `^#`
  heading of every **other** `docs/*.md`, and from every `fn_<addr>`/`FN_<addr>`
  token in `src/**/*.rs` (checked mechanically, read-only, before this page was
  written; this page is now the only place the twelve appear as headings).
* No Rust file, no ledger row and no other document was written by this pass; the
  only file created is `docs/SPEC_WAVE149_ENEMIES.md`.
* No body was walked through a `bctrl` as a terminator: every `0x4E800421` is
  followed through to `pc+4`, and every `0x4E800420` ends the walk. Conditional
  `bclr` forms were followed on both edges.
* Total `qr3` sites claimed: 0. All `psq` words in the twelve bodies are `qr0`
  `f29`/`f30`/`f31` spills; the three remaining `qr3` bodies in the module are
  listed above and are all below this wave's size cut.
