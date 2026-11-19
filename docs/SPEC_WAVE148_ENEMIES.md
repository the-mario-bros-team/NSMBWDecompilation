# Wave 148 - implementation specification for 12 untranslated `d_enemiesNP` bodies

## What this page is

`docs/ANALYSIS_WAVE127_ENEMIES.md` reclassified 384 formerly-skipped `d_enemiesNP`
addresses into 173 class **B**, 160 class **C** and 51 class **D**. Waves 128-147
worked through that block; this page is the next plain size-ranked tranche: the
twelve largest remaining bodies that are untranslated, have a Ghidra export, and
are fully walkable from their entry.

A live re-walk of `work/ghidra/rel-d_enemiesNP/export/functions.tsv` (3,555 rows)
against the `address` column of `ledger/functions.csv` (16,941 addresses), against
every `0x80Axxxxx`/`0x80Bxxxxx` address in a `^#` heading of any `docs/*.md`
(1,468 distinct addresses at selection time, i.e. before this page existed) and
against every `fn_<addr>`/`FN_<addr>` token in `src/**/*.rs` (16,313 distinct
addresses in 68 files) leaves **84** untranslated bodies, all 84 with an export.
Two of the rows wave 147 listed as "next in size" are no longer eligible:
`0x80A2EC00` (472) and `0x80B27ED0` (432) are translated in
`src/game/enemies.rs` without a ledger row, so the size sort skips them.

A re-run of the same three scans after this page was written reports **72**
candidates, because the twelve sections below are the twelve missing rows.

This page is **read-only**: it writes no Rust, no ledger row and no other doc.

## Selection

The ranking metric is the **reachable body**, not the raw `functions.tsv` extent,
because the TSV size is a bucket extent that can include a neighbour's padding or
an unreachable arm (see `docs/REL_ELF_MAPPING.md`). Each candidate was walked from
its entry through every conditional branch, every internal `b` and every
**linked** indirect branch (`bctrl`, `blrl`), until all paths ended in `bclr`, a
tail `b` or a tail `bcctr`.

| # | address | TSV size | reachable insns | `qr3` sites | role | verdict |
|---:|---|---:|---:|---:|---|---|
| 1 | `0x80B058A0` | 476 | 119 | 0 | world-map rail actor: trailing-segment rebuild and state hand-off | **ready** |
| 2 | `0x80A81150` | 456 | 114 | 0 | descent step that lands the actor into the `walk` clip and state `0x80B734B8` | **ready** |
| 3 | `0x80A5EB00` | 448 | 112 | 0 | clip-gated step with two land-smoke emits, two sound cues and three state exits | **ready** |
| 4 | `0x80AE69E0` | 444 | 111 | 0 | pair-contact record builder, template B (fixed `0x80B31038` block) | **ready** |
| 5 | `0x80A5C660` | 436 | 109 | 0 | multi-slot attack dispatch from a position pair | **ready** |
| 6 | `0x80A94100` | 408 | 102 | 0 | pair-contact record builder, counter variant (`0x8009F710` slot) | **ready** |
| 7 | `0x80AB26A0` | 408 | 102 | 0 | two-clause arrival gate that latches the dash state `0x80B74FD0` | **ready** |
| 8 | `0x80A84A00` | 400 | 100 | 0 | 10-link chain emitter over the `+0x584` holder array | **ready** |
| 9 | `0x80AA0530` | 388 | 97 | 0 | `daEnKanon_c` dash/overlap state step | **ready** |
| 10 | `0x80AB29C0` | 376 | 94 | 0 | 5-link chain integrator (angles, velocities, per-axis targets) | **ready** |
| 11 | `0x80A53850` | 368 | 92 | 0 | rail-record snap plus surface-crossing effect | **conditional** |
| 12 | `0x80A88BC0` | 368 | 92 | 2 | 10-segment spiral position solver (writes the `0x80A84A00` input) | **ready** |

**Split: 11 ready, 1 conditional.** Total 4,976 bytes of TSV extent and 1,244
reachable instructions. Every one of the twelve walks to a terminal instruction
at exactly `entry + size - 4`, so `reachable == extent` for all twelve and the
"extent is padding" caveat does not apply here.

Ranks 8 and 12 are the two halves of one actor: `0x80A88BC0` solves the ten
`+0xC24` positions from the ten `+0xDDC` halfword angles, and `0x80A84A00`
consumes exactly those positions. Ranks 4, 5 and 6 are close relatives of three
already-translated bodies (`fn_80ac1ed0`, `fn_80ac2a50`, and the `Wm` effect
update `fn_80098900` described in `docs/COVERAGE_WAVE113_STAGE.md`), so their
record and call contracts are pinned by existing code rather than inferred here.

### Why wave 147's three held-back rows are claimed here

`docs/SPEC_WAVE147_ENEMIES.md` ("Deliberate skips") left `0x80A5C660`,
`0x80A5EB00` and `0x80A53850` unclaimed. Each objection is answered by the raw
stream, not by the Ghidra export:

* **`0x80A5C660` - "`extraout_f1`; the float state threading is not recoverable."**
  The stream carries the float explicitly at every call site:
  `0x80A5C6FC lfs f1,0x64(r31)` before `0x80267420`, and
  `0x80A5C7C8 lfs f1,0x70(r31)` before `0x802674C0`. `extraout_f1` is the
  decompiler losing `f1` across the preceding `bctrl`, exactly the artefact
  `docs/COVERAGE_WAVE113_STAGE.md:83` already resolved for the same three
  callees ("take their value in `f1` and the object in `r3`").
* **`0x80A5EB00` - "five unledgered `main.dol` helpers and a four-way clip test at
  `0x801690E0`; the clip-name roles are not established."**
  `0x801690E0(anim, frame)` is the frame-window test, `0x80169020(anim)` the
  "clip finished" test and `0x801680E0(anim, rate)` the binder - all documented
  in `src/game/enemies.rs:20921` and `src/engine/display.rs:9082` and used by
  waves 145-147. The four constants are frame numbers (55, 104, 102, 99, 18) and
  each gate is followed by a fully literal action, so no clip *name* is needed.
* **`0x80A53850` - "`0x800EB930` returns a float in `f1` in addition to its `r3`
  verdict."** `0x800EB930` is `fn_800eb930`, a **`void`** scan of the `0x50`-entry
  record table (`src/engine/dsys.rs`, "nearest matching record"). Its `r3` result
  is the matched entry's kind byte (the `_restgpr_24(uVar4)` tail), and the caller
  reads exactly that: `extsb r3` then `cmpwi 1`. There is no `f1` result. The row
  is still marked *conditional* here, but for a different and narrower reason -
  see its own section.

### Rows below the cut

| address | TSV size | reachable | why it is not claimed |
|---|---:|---:|---|
| `0x80A04D10` | 360 | 90 | next in size order; clean, left for wave 149 |
| `0x80B28F90` | 356 | 89 | clean; the 10-link emitter twin of rank 8 (adds `0x801C0540`) |
| `0x80A334D0` | 352 | 88 | clean, 5 tail calls that are all in-module |
| `0x80B05740` | 328 | 82 | clean; the same rail-segment family as rank 1 |
| `0x80B25280` | 328 | 82 | clean, 2 `qr0` spill words only |
| `0x80B253D0` | 328 | 82 | clean; one `0x8012F810` tail call differs from its twin above |
| `0x80B2A360` | 324 | 81 | clean, four `psq` spills, no calls outside `0x8005Fxxx` |
| `0x80AD5CD0` | 316 | 79 | clean, four `bctrl` sites |

The size cut is clean: 368 bytes is claimed twice (ranks 11 and 12) and the next
row is 360, so no tie-break rule is needed.

## The `qr3` pool at this wave

`docs/REL_ELF_MAPPING.md` pins **`GQR3 = 0x00050005`** (unsigned 16-bit, scale
0), installed by `main.dol` at `0x8025EEEC`/`0x8025EEF0`, and records that the RAW
stream of this REL contains no `mtspr GQRn`. A re-scan of the extents of all 84
remaining bodies finds four `qr3` users and five sites; rank 12 owns two of them:

| address | size | `qr3` sites | where |
|---|---:|---:|---|
| `0x80A88BC0` | 368 | 2 | `0x80A88C50`, `0x80A88C70` (the same per-link halfword, read twice) |
| `0x80A80F40` | 264 | 1 | per-link angle (not claimed) |
| `0x80A80E50` | 220 | 1 | per-link angle (not claimed) |
| `0x80B22750` | 128 | 1 | single angle (not claimed) |

With `W = 1` and `I = 3` every data `psq` here is one unsigned 16-bit load scaled
by `2^0`:

```
psq_l fD,0(rA),1,qr3      ; fD = (f32)*(u16 *)rA
```

All other `psq` words in the twelve bodies are `qr0` `f31`/`f30`/`f29` stack
spills (`stfd`/`lfd` beside them) and carry no quantisation.

## Reading conventions

* Synthetic base is `0x80A00000`. The REL has five file-backed `PT_LOAD`
  segments (`0x80A000F0`, `0x80B2A518`, `0x80B2A7A4`, `0x80B2A7B0`,
  `0x80B32DC0`); `docs/REL_ELF_MAPPING.md`'s `+0x1000` formula applies to the
  first. The file-backed image ends at `0x80B6E438`, so anything at or above the
  zero-fill segment `0x80B6E500` is **BSS** and is marked **(bss)** below.
* `this` is the incoming `r3` unless stated otherwise; `+0xNNN` are byte offsets
  from `this`. `+0xAC`/`+0xB0`/`+0xB4` are x/y/z; `+0x100`/`+0x102`/`+0x104`
  are the three `s16` angles; `+0x348` is the side/facing byte; `+0x38F` is the
  player id byte; `+0x60` is the class vtable; `+0xE8`/`+0xEC`/`+0xF0` are the
  three speed components.
* `addi rD,r0,imm` (A field `0`) is `li` and `addis rD,r0,imm` is `lis`; the
  base is the literal zero, not `r0`. So `addi r5,r0,0x600` is `li r5,0x600`,
  while `addi r0,r5,0x400` is `r0 = r5 + 0x400`.
* `rlwinm rA,rS,sh,mb,me` is `ROTL32(rS,sh) & mask(mb..me)` with mask bits
  numbered **from the MSB** (bit 0 = `0x80000000`, bit 31 = `1`). In particular
  `rlwinm r0,r0,26,28,29` is `(r0 >> 6) & 0xC`, and `rlwinm. r0,r3,0,31,31`
  tests bit 0 of the word.
* On a `bc`, `bo = 12` branches on a **true** CR bit and `bo = 4` on a **false**
  one; `bi & 3` is LT/GT/EQ/SO. `cror cr0.eq,cr0.lt,cr0.eq` (`0x4C401382`,
  XO 449) rewrites `cr0.eq` as `lt|eq`, so the following `bne` reads
  `if !(a <= b)`; `cror cr0.eq,cr0.gt,cr0.eq` (`0x4C411382`) gives `gt|eq`.
* `0x4E800421` is `bctrl` (`LK = 1`): an indirect call that **returns**. Only
  `0x4E800420` (LK = 0) is a tail `bcctr`.
* `0x43300000` in the high word with the value `^ 0x80000000` in the low word,
  minus the double at `0x80B2FB40` (`2^52 + 2^31`), is the `u32 -> f64`
  conversion; the shorter `2^52`-only form (`0x80B31AA0`, `0x80B2E??` class)
  appears where the value is provably below `2^31`.
* `fctiw` truncates **nothing**: it rounds per FPSCR (round-to-nearest-even by
  default). Where a body uses `fctiw` instead of `fctiwz` this page says
  `round_to_nearest`.

## Symbolic helpers used below

| address | symbol | contract used here |
|---|---|---|
| `0x8006CDE0` | `fn_8006cde0(this)` | per-frame actor step (main.dol, unledgered) |
| `0x8006CD40` | `fn_8006cd40(this)` | second per-frame actor step (main.dol, unledgered) |
| `0x8006A4B0` | `fn_8006a4b0(&out, &pos)` | copy a three-float position out of an actor |
| `0x800955F0` | `fn_800955f0(this)` | velocity/orbit integrate; writes `+0xEC`, `+0xF0`, `+0xAC..+0xB4` |
| `0x80096E60` | `fn_80096e60(this) -> u8` | ground/animation bits, rewrites `+0xEC` |
| `0x80072BC0` | `fn_80072bc0(block)` | ground/contact probe on the `+0x1EC` physics block |
| `0x80070CB0` | `fn_80070cb0(block) -> u32` | ground probe used to leave a descent state |
| `0x800954D0` | `fn_800954d0(actor, &pos)` | copy a three-float position into the actor |
| `0x80064A30` | `fn_80064a30(xa, xb) -> u32` | signed-x direction/side code from two promoted `f32` |
| `0x80064AF0` | `fn_80064af0(&pos, &angles, 1)` | build/convert the three-angle frame (unledgered) |
| `0x80065DC0` | `fn_80065dc0(actor, f1, &pos)` | sound event `0x284` at `pos` with a scale triple (unledgered) |
| `0x8009F630` / `0x8009F710` | `fn_8009f630(actor)` / `fn_8009f710(block, index)` | contact-record word / per-target slot word |
| `0x80065B80` | `fn_80065b80(actor, s8, 0)` | apply an animation/side code |
| `0x800EB930` | `fn_800eb930(table, &point, &pair, &height, &angle, layer) -> s8` | scan the `0x50`-entry record table; leaves the matched kind byte (or `-1`) in `r3` |
| `0x80162510` | `fn_80162510(actor)` | dispose the actor |
| `0x8015F4F0` | `fn_8015f4f0(&s16, target, step) -> u32` | step an `s16` angle, `1` on arrival |
| `0x801662F0` | `fn_801662f0(holder, index)` with value in `f1` | set the `0x38`-stride element's value |
| `0x80166320` | `fn_80166320(holder, index) -> u32` | range test on one element |
| `0x80166340` | `fn_80166340(holder, 1, 0)` | commit one animation element |
| `0x801680E0` | `fn_801680e0(anim, rate)` | bind/advance a clip with `rate` in `f1` |
| `0x80168520` | `fn_80168520(holder, handle)` | rebuild the keyed entry from a handle |
| `0x801690E0` | `fn_801690e0(anim, frame) -> u32` | "frame lies inside the animation window" |
| `0x80169020` | `fn_80169020(anim) -> u32` | "clip finished" |
| `0x8016A170` / `0x8016A150` / `0x8016A1A0` | scene-node setters | matrix / point / finalise, one holder each |
| `0x8016EDD0` | `fn_8016edd0(node, &s16)` | Y-rotation concat in place |
| `0x801C0BD0` | `fn_801c0bd0(x, y, z, out)` | write a 3x4 translation matrix |
| `0x80198C30` | `fn_80198c30(mgr, id, &pos, 0)` | play sound event `id` at `pos` |
| `0x80197F00` | `fn_80197f00(mgr, id, &pos, 0)` | play sound event `id` at `pos` |
| `0x8019ECE0` | `fn_8019ece0(handle, &from, &to, 0)` | level/position record submit |
| `0x80196660` | `fn_80196660(mgr, id) -> u32` | collision/attack id mask word |
| `0x80267390` / `0x80267420` / `0x802674C0` | value-in-`f1` actions | damage store / ground lerp / speed store |
| `0x80275560` | `fn_80275560(actor, slot) -> u32` | 16-byte slot getter, `>= 1` for a live slot |
| `0x80237BD0` / `0x80237C40` | `sin` / `cos` | `f64` in and out, radian argument |
| `0x8023A0B0` / `0x801651F0` / `0x80169F80` | clip lookup / pair bind / rate set | `80AB1780`'s four calls |
| `0x80B04680` / `0x80B04740` | `fn_80b04680(owner, rail) -> u32` / `fn_80b04740(owner, rail) -> u32` | rail probe / rail advance (`1` = rail finished) |
| `0x80B04120` / `0x80B054D0` | `fn_80b04120(seg, name)` / `fn_80b054d0(actor)` | bind one trailing segment / rewrite all trailing positions |
| `0x80A80470` / `0x80A80860` / `0x80A5D9D0` | world-map helpers | effect submit / clip bind / arrival flag |
| `0x80A5E2C0` / `0x80A5DD20` | `fn_80a5e2c0(scale, actor, &point)` / `fn_80a5dd20(actor)` | land-smoke emit / target state write |
| `0x80A9FF20` / `0x80AA00A0` | `fn_80a9ff20(actor) -> u8` / `fn_80aa00a0(actor)` | flag byte (bit0 contact, bit1 animation, bit2 overlap) / Kanon frame tail |
| `0x80AB1780` / `0x80AB1830` / `0x80AB1900` | world-map chain helpers | clip binder `(f1, this, index)` / effect timer / angle->speed |
| `0x80AB1D60` / `0x80AB1DF0` | `fn_80ab1d60(this, &dir, rate) -> u32` / `fn_80ab1df0(this)` | aim `+0x100` at `dir` / run the `+0x544` body slot `+0x1C` |
| `0x80AB1F70` / `0x80AB1F10` | `fn_80ab1f70(this, axis)` / `fn_80ab1f10(this, axis)` | velocity integrate / per-axis target step |
| `0x802DCD58/60/64/68/6C/70/74` and `0x802DCDA4/AC/B0/B4/B8/BC/C0` | `_savegpr_N` / `_restgpr_N` | incoming `r3..r10` survive; the export's "no arguments" is the restore pair |

---

## 1. `0x80B058A0` - world-map rail actor trailing-segment rebuild

* **Extent** 476 bytes (119 words), 119 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80b058a0_FUN_80b058a0.c`.
* **Signature** `unsafe fn fn_80b058a0(this: usize)` - `r3` only, returns `void`.
* **Role** re-probe the two rails owned by the actor (`+0x540` and `+0x6FC`,
  both against the payload pointer at `+0x2B1C`), advance them, emit sound
  `0x27C`, rewrite every trailing segment's position, raise the segment count up
  to the cap at `+0x2B38`, then (re)bind each of the `+0x8B8 + i*0x1B8`
  segments with the `"create"` clip and a staggered time offset. Finally the
  class parameter object at `+0x2B28` decides which of the two state objects is
  assigned. It is the emitter that consumes `fn_80b04680`/`fn_80b04740`/
  `fn_80b04120`/`fn_80b054d0`.

### Behaviour

1. Spill `f31` (`stfd` + `psq_st ...,qr0`), `_savegpr_23`.
2. `rail = *(u32 *)(this+0x2B1C)`;
   `a = fn_80b04680(this+0x540, rail)`; `b = fn_80b04680(this+0x6FC, rail)`.
3. If `a != 0 || b != 0`:
   1. `r1 = fn_80b04740(this+0x540, *(u32 *)(this+0x2B1C))`,
      `r2 = fn_80b04740(this+0x6FC, *(u32 *)(this+0x2B1C))`.
   2. `fn_8006a4b0(&pos, this+0xAC)`; `fn_80198c30(*(u32 *)0x80429D60, 0x27C, &pos, 0)`.
   3. `fn_80b054d0(this)`.
   4. `n = *(s32 *)(this+0x2B34) + 1; *(s32 *)(this+0x2B34) = n;`
      `if (n > *(s32 *)(this+0x2B38)) *(s32 *)(this+0x2B34) = *(s32 *)(this+0x2B38);`
   5. `step = round_to_nearest(16.0 / *(f32 *)(0x80B31A84 + ((*(u32 *)(this+4) >> 6) & 0xC)))`
      (`fctiw`).
   6. `t = 0;` for `i = 0; i < *(s32 *)(this+0x2B34); i++`:
      `seg = this + 0x8B8 + i*0x1B8`;
      `fn_80b04120(seg, 0x80B66490 /* "create" */)`;
      `fn_801662f0(seg + 0x48, 0)` with `f1 = (f32)t`;
      `t += step`.
4. If `r1 != 0 || r2 != 0`:
   `obj = *(u32 *)(this+0x2B28)`;
   `v = (*(u32 *)obj[0] + 0x10)(obj, 0x80B78618)`;
   `name = v == 0 ? 0x80B78518 : 0x80B78618`;
   `(*(u32 *)(this+0x60) + 0xD4)(this, name)`.
5. `_restgpr_23`; return.

### Constants

| value | meaning |
|---|---|
| `0x80B31A74` = `16.0` | base interval numerator |
| `0x80B31A84` = `{0.5, 1.0, 1.4, 1.0}` | interval divisor, index `(object_id >> 6) & 0xC` |
| `0x80B31AA0` = `2^52` (double) | `u32 -> f64` fold for the staggered time |
| `0x80B66490` = `"create"` | clip bound to every trailing segment |
| `0x80B78618`, `0x80B78518` | state objects **(bss)** |
| `0x80429D60` | sound manager pointer (main.dol) |
| `0x27C` | sound event id |
| `0x1B8` / `+0x8B8` / `+0x48` | segment stride / segment array / per-segment value field |
| `+0x2B1C`, `+0x2B28`, `+0x2B34`, `+0x2B38` | rail payload, class parameter object, live count, count cap |

### Port notes and verdict

`fctiw` rounds to nearest, so `16.0 / 1.4` is `11`, not `11.43`; a port that
truncates produces the same values for this table (32, 16, 11, 16) but should
keep `round()` for fidelity. `fn_801662f0` takes the value in `f1` and the index
in `r4`; the `r4 = 0` in the stream is a literal zero, not a register. The two
rail sub-objects and the class parameter object are ordinary in-module state.
**Verdict: ready.**

## 2. `0x80A81150` - descent step that lands into `walk`

* **Extent** 456 bytes (114 words), 114 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80a81150_FUN_80a81150.c`.
* **Signature** `unsafe fn fn_80a81150(this: usize)` - `r3` only, returns `void`.
* **Role** the airborne/descent half of a world-map walker: it finalises four
  sub-objects, runs the four standard frame helpers, squares the vertical speed
  when the class flag word allows it, re-aims the model while the speed is
  negative, and on ground contact resets the speed, plays `"walk"` at rate `2.0`
  and switches to state `0x80B734B8`.

### Behaviour

1. `f31 = *(f32 *)(this+0xEC)` (saved before the helpers overwrite it);
   `_savegpr_25`, `psq_st`/`stfd` `f31` spill.
2. `(*(u32 *)(this+0x5F4) + 0x14)(this+0x5F4)`,
   `(*(u32 *)(this+0x624) + 0x14)(this+0x624)`,
   `(*(u32 *)(this+0x5BC) + 0x14)(this+0x5BC)`,
   `(*(u32 *)(this+0x544) + 0x1C)(this+0x544)`.
3. `fn_8006cde0(this)`, `fn_800955f0(this)`, `fn_80096e60(this)`,
   `fn_80a80470(this)`.
4. `f = *(f32 *)(this+0xEC)`; if `f > 0.0 && (*(u32 *)(this+0x274) & 0x3C000000) != 0`
   then `*(f32 *)(this+0x114) = -0.1875` and `*(f32 *)(this+0xEC) = -0.5*f*f`.
5. If `f31 > 0.0 && *(f32 *)(this+0xEC) <= 0.0` then `*(f32 *)(this+0x114) = -0.1875`.
6. If `*(f32 *)(this+0xEC) < 0.0`:
   `fn_8015f4f0(this+0x102, *(s16 *)(0x8042C1A4 + *(u8 *)(this+0x348)*2), 0x600)`
   then `fn_8015f4f0(this+0x100, 0, 0x200)`.
7. If `fn_80070cb0(this+0x1EC) != 0`:
   1. `*(f32 *)(this+0xEC) = 0.0`.
   2. `fn_801680e0(this+0x584, 2.0)`.
   3. `fn_80a80860(this, 0x80B4D760 /* "walk" */, 0)`.
   4. `h = (*(u32 *)(this+0x5F4) + 0xC)(this+0x5F4); fn_80168520(this+0x544, h)`.
   5. `(*(u32 *)(this+0x60) + 0xD4)(this, 0x80B734B8)`.
8. `_restgpr_25`; return.

### Constants

| value | meaning |
|---|---|
| `0x80B2E2F8` = `0.0` | speed sign threshold |
| `0x80B2E330` = `-0.1875` | `+0x114` latch value |
| `0x80B2E334` = `-0.5` | squared-speed coefficient |
| `0x80B2E304` = `2.0` | `"walk"` clip rate |
| `0x8042C1A4` = `{8192, -8192, 1024, -1024}` (`s16`) | per-side turn target, index `+0x348` |
| `0x80B4D760` = `"walk"` | clip bound on landing |
| `0x80B734B8` | walk state object **(bss)** |
| `0x600` / `0x200` | yaw step / pitch step |
| `0x3C000000` | mask over bits 2..5 of `+0x274` |

### Port notes and verdict

Step 4 is `fmuls` twice and therefore `-0.5 * f * f`, not `(-0.5 * f) * (-0.5 * f)`.
The three `psq_st`/`stfd` pairs are spills. `0x8015F4F0` is void here; the return
is ignored on both paths. **Verdict: ready.**

## 3. `0x80A5EB00` - clip-gated step with land-smoke and sound cues

* **Extent** 448 bytes (112 words), 112 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80a5eb00_FUN_80a5eb00.c`.
* **Signature** `unsafe fn fn_80a5eb00(this: usize)` - `r3` only, returns `void`.
* **Role** the per-frame step of a small ground actor: two sub-object finalisers,
  the three standard helpers, then a ground-probe-gated pair of land-smoke
  emits keyed on clip frames `55.0` and `104.0`, and a five-way exit chain
  (`0x80169020` finished -> state `0x80B71B60`; frames `102.0` / `99.0` / `18.0`
  -> target state write / sound `534` / sound `533`).

### Behaviour

1. `(*(u32 *)(this+0x5C0) + 0x14)(this+0x5C0)`,
   `(*(u32 *)(this+0x544) + 0x1C)(this+0x544)`.
2. `fn_8006cde0(this)`, `fn_800955f0(this)`, `fn_80a5d9d0(this)`.
3. If `fn_80072bc0(this+0x1EC) != 0`:
   1. `*(f32 *)(this+0xEC) = 0.0`.
   2. If `fn_801690e0(this+0x584, 55.0) != 0`:
      `p = { *(f32 *)(this+0x63C), *(f32 *)(this+0xB0), *(f32 *)(this+0x644) };`
      `fn_80a5e2c0(1.0, this, &p)`.
   3. Else if `fn_801690e0(this+0x584, 104.0) != 0`:
      `fn_80a5e2c0(0.7, this, this+0x648)`.
4. If `fn_80169020(this+0x584) != 0`:
   `(*(u32 *)(this+0x60) + 0xD4)(this, 0x80B71B60)`; goto 8.
5. If `fn_801690e0(this+0x584, 102.0) != 0`: `fn_80a5dd20(this)`; goto 8.
6. If `fn_801690e0(this+0x584, 99.0) != 0`:
   `fn_8006a4b0(&p, this+0xAC); fn_80197f00(*(u32 *)0x80429D5C, 534, &p, 0)`;
   goto 8.
7. If `fn_801690e0(this+0x584, 18.0) != 0`:
   `fn_8006a4b0(&p, this+0xAC); fn_80197f00(*(u32 *)0x80429D5C, 533, &p, 0)`.
8. Return.

### Constants

| value | meaning |
|---|---|
| `0x80B2D19C` = `0.0` | speed on ground contact |
| `0x80B2D204` = `55.0`, `0x80B2D208` = `104.0` | land-smoke clip frames |
| `0x80B2D20C` = `102.0`, `0x80B2D210` = `99.0`, `0x80B2D214` = `18.0` | exit-clip frames |
| `0x80B2D198` = `1.0`, `0x80B2D1E8` = `0.7` | land-smoke scales |
| `0x80B71B60` | exit state object **(bss)** |
| `534`, `533` | sound event ids |
| `0x80429D5C` | sound manager pointer (main.dol) |
| `+0x584`, `+0x63C`, `+0x644`, `+0x648` | clip holder, x, z, second point pair |

### Port notes and verdict

`fn_80a5e2c0` reads only the **first two** words of `point` and substitutes
`5500.0` (`0x80B2D1E4`) for the third, so the `{x, y, z}` triple built at
`0x80A5EBA0..0x80A5EBAC` loses its `z` word in the emit; keep that shape. The
second call passes `this+0x648` directly (two floats, no stack copy). All five
clip tests are the same `0x801690E0(anim, frame)` call with a different `f1`.
**Verdict: ready.**

## 4. `0x80AE69E0` - pair-contact record builder, template B

* **Extent** 444 bytes (111 words), 111 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80ae69e0_FUN_80ae69e0.c`.
* **Signature** `unsafe fn fn_80ae69e0(this: usize, point_src: usize, pair: usize)`
  - `r3` owns the record, `r4` is the two-float source at `+0x90`/`+0x94`, `r5`
  supplies the partner actor at `+0x4`; returns `void`.
* **Role** the third member of the contact-record family: `fn_80ac1ed0` (variant
  table + `0x80B302C8` block) and `fn_80ac2a50` (partner through `+0x510`,
  `0x8009F710` slot) are the other two. This one copies the position from the
  second argument, derives the side byte from the two actors' `X + 0xD0` terms,
  and fills `+0x414..+0x432` from the `0x80B31038` block plus the geometry
  constants at `0x8042C1B8..C0`.

### Behaviour

1. `_savegpr_24`; `partner = *(u32 *)(r5+4)`; `this = r3`; `src = r4`.
2. `side = (u8)fn_80064a30((f32)(*(f32 *)(this+0xAC) + *(f32 *)(this+0xD0)),
   (f32)(*(f32 *)(partner+0xAC) + *(f32 *)(partner+0xD0)))`.
3. `(*(u32 *)(partner+0x60) + 0x6C)(partner)` (accessor, result used twice below).
4. `fn_800954d0(this, { *(f32 *)(src+0x90), *(f32 *)(src+0x94), 5500.0 })`.
5. `d = (*(u32 *)(partner+0x60) + 0x3E8)(partner)`; `fn_80065b80(partner, (s8)d, 0)`.
6. `slot = fn_8009f630(partner)`.
7. Write the record:
   `+0x432 = 1`;
   `+0x414 = *(f32 *)(0x80B30F18 + side*4)`;
   `+0x418 = 1.5`; `+0x41C = -1.5`; `+0x420 = -0.0625`;
   `+0x424 = 0x80357BF4`; `+0x428 = slot`; `+0x42C = 0xFFFFFFFF`;
   `+0x430 = side`; `+0x431 = *(u8 *)((*(u32 *)(partner+0x60) + 0x6C)(partner))`.
8. `_restgpr_24`; return.

### Constants

| value | meaning |
|---|---|
| `0x80B30F18` = `{0.5, -0.5, 0.25, -0.25}` | `+0x414` template word, index `side` |
| `0x8042C1BC` = `1.5`, `0x8042C1C0` = `-1.5`, `0x8042C1B8` = `-0.0625` | `+0x41C` / `+0x420` words |
| `0x80B31048` = `0x80357BF4`, `0x80B31050` = `0xFFFFFFFF` | `+0x424` / `+0x42C` template words |
| `0x80B31058` = `5500.0` | position record `z` sentinel |
| `+0x414..+0x432` | the seven-word contact record |

### Port notes and verdict

The side byte is not bounds-checked: `fn_80064a30` returns `0..255` and
`side*4` indexes `0x80B30F18`, exactly as the twin `fn_80ac1ef0` family does.
The record's `+0x418`/`+0x41C`/`+0x420` words come from `0x8042C1B8..C0`
(`-0.0625`, `1.5`, `-1.5`), i.e. the same three constants the sibling
`0x80AC2A50` uses in a different order. `+0x424`/`+0x42C` are pointers into
`main.dol` data, not floats. **Verdict: ready.**

## 5. `0x80A5C660` - multi-slot attack dispatch from a position pair

* **Extent** 436 bytes (109 words), 109 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80a5c660_FUN_80a5c660.c`.
* **Signature** `unsafe fn fn_80a5c660(this: usize, pos: *const [f32; 2])` -
  `r3` the attacker, `r4` a two-float position pair; returns `void`.
* **Role** the enemy-side twin of the `Wm` effect update `fn_80098900`
  (`docs/COVERAGE_WAVE113_STAGE.md:71`): it stores the pair into
  `+0xA4`/`+0xA8`, finds the first live slot with the `0x80275560` scan, submits
  a level record, and then applies the complemented `0x80196660` id mask to
  every live entry of the `+0x74` array (stride 8, count `+0x54`).

### Behaviour

1. `_savegpr_26`; `*(f32 *)(this+0xA4) = pos[0]; *(f32 *)(this+0xA8) = pos[1];`
2. `i = 0;` while `fn_80275560(this, i) <= 0` do `{ i++; if (i >= 4) goto 9; }`.
3. `fn_8019ece0(*(u32 *)(this+0x5C), this+0x64, this+0xA4, 0)`.
4. For `k = 0; k < *(u32 *)(this+0x54); k++`:
   `ref = *(u32 *)(this+0x74 + k*8)`; skip when `ref == 0`.
   1. `m = ~fn_80196660(*(u32 *)0x8042A488, *(u32 *)(ref+0x9C))`.
   2. `hit = (m & 1) != 0`; if `!hit` then
      `g = *(u32 *)0x8042A4A8`;
      `hit = (*(u32 *)(g+0x14) == 3) || (*(u32 *)(g+0x14) == 2 && *(u32 *)(g+0x10) == 3)`.
   3. If `hit`: `fn_80267420(ref, f1 = *(f32 *)(this+0x64))`.
   4. If `(m & 8) != 0`:
      `id = ref ? *(u32 *)(ref+0x9C) : 0xFFFFFFFF`;
      `fn_80275780(*(u32 *)(*(u32 *)0x8042A488 + 0x100), id, &out)`;
      `v = *(s32 *)(out+0x0C)`;
      `d = *(s32 *)(this+0x68)`;
      `fn_80267390(ref, d < 0 ? v + d : v)`.
   5. If `(m & 2) != 0`: `fn_802674C0(ref, f1 = *(f32 *)(this+0x70))`.
5. `_restgpr_26`; return.

### Constants

| value | meaning |
|---|---|
| `0x8042A488` | global pointer whose `+0x100` is the id-mask manager (main.dol) |
| `0x8042A4A8` | game-mode global: `+0x14` mode, `+0x10` sub-mode (main.dol) |
| `1`, `8`, `2` | action bits of the complemented id mask |
| `+0x54`, `+0x74` (stride 8) | live-entry count and array |
| `+0x5C`, `+0x64`, `+0x68`, `+0x70`, `+0xA4`, `+0xA8` | handle, radius, damage delta, speed, position pair |

### Port notes and verdict

`out` is the stack record handed to `0x80275780` at `r1+8`; the value read back
is `r1+0x14`, i.e. `out+0x0C`. The three `f1` operands (`+0x64` for the lerp,
`+0x70` for the speed store) are explicit `lfs` in the stream - the export's
`extraout_f1` is not a missing operand. The mode gate reads two `main.dol`
globals; both are already modelled by the port (`fn_80098900` in
`src/game/stage.rs` uses the same pair). **Verdict: ready.**

## 6. `0x80A94100` - pair-contact record builder, counter variant

* **Extent** 408 bytes (102 words), 102 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80a94100_FUN_80a94100.c`.
* **Signature** `unsafe fn fn_80a94100(this: usize, point_src: usize, partner_owner: usize)`
  - same argument shape as rank 4; returns `void`.
* **Role** the counter-driven sibling of rank 4: instead of
  `0x8009F630(partner)` it advances the partner's `+0x134` slot counter (clamped
  at 8) and resolves the slot with `0x8009F710(this+0x510, counter)`, and the
  slot branch is gated on the partner's `+0x6C` accessor byte being `< 4`. The
  template block is `0x80B2EBB8..0x80B2EBD4` and the geometry constants are the
  three `0x8042C2D0..D8` words.

### Behaviour

1. `_savegpr_25`; `partner = *(u32 *)(r5+4)`.
2. `side = (u8)fn_80064a30((f32)(*(f32 *)(this+0xAC) + *(f32 *)(this+0xD0)),
   (f32)(*(f32 *)(partner+0xAC) + *(f32 *)(partner+0xD0)))`.
3. `cls = *(u8 *)((*(u32 *)(partner+0x60) + 0x6C)(partner))`.
4. `fn_800954d0(this, { *(f32 *)(src+0x90), *(f32 *)(src+0x94), 5500.0 })`.
5. `slot = 0xFFFFFFFF;` if `cls < 4`:
   `fn_80065b80(partner, *(u32 *)(partner+0x134), 0)`;
   `n = *(u32 *)(partner+0x134) + 1; *(u32 *)(partner+0x134) = n;`
   `if (n > 8) *(u32 *)(partner+0x134) = 8;`
   `slot = fn_8009f710(this+0x510, *(u32 *)(partner+0x134))`.
6. Write the record:
   `+0x432 = 1`;
   `+0x414 = *(f32 *)(0x8042C1B0 + side*4)`;
   `+0x418 = 3.9`; `+0x41C = -4.0`; `+0x420 = -0.24375`;
   `+0x424 = 0x80357BF4`; `+0x428 = slot`; `+0x42C = 0xFFFFFFFF`;
   `+0x430 = side`; `+0x431 = cls`.
7. `_restgpr_25`; return.

### Constants

| value | meaning |
|---|---|
| `0x8042C1B0` = `{1.5, -1.5, -0.0625, 1.5}` | `+0x414` template word, index `side` |
| `0x8042C2D4` = `3.9`, `0x8042C2D8` = `-4.0`, `0x8042C2D0` = `-0.24375` | `+0x418` / `+0x41C` / `+0x420` |
| `0x80B2EBC8` = `0x80357BF4`, `0x80B2EBD0` = `0xFFFFFFFF` | `+0x424` / `+0x42C` |
| `0x80B2EBD8` = `5500.0` | position record `z` |
| `8` | `+0x134` counter cap |
| `+0x510` | this actor's slot block |

### Port notes and verdict

The `bVar1 < 4` test replaces `fn_80ac2a50`'s `0x80026C00` target lookup, so the
"target index" here is the partner's class/accessor byte, and it is stored twice
(`+0x431` and, byte-for-byte, in the dead stack quadword). The initial
`lwzu r26,-0x1448(r9)` block loads eight words from `0x80B2EBB8`, six of which
are zero; only `+0x424` and `+0x42C` are non-zero. **Verdict: ready.**

## 7. `0x80AB26A0` - two-clause arrival gate into the dash state

* **Extent** 408 bytes (102 words), 102 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80ab26a0_FUN_80ab26a0.c`.
* **Signature** `unsafe fn fn_80ab26a0(this: usize)` - `r3` only, returns `void`.
* **Role** the "both sub-objects have arrived" gate of a world-map enemy: it
  finalises `+0x73C` (and `+0x794` when `+0x988 == 1`), runs `fn_80ab1df0`,
  tests the `+0x73C` element range and the `fn_80ab1d60` aim, and when both tests
  pass latches `+0xE8` to `+-0.3`, snapshots the position into `+0x890..+0x898`,
  commits both holders, rebinds animation index `0` and enters state
  `0x80B74FD0`.

### Behaviour

1. If `*(u32 *)(this+0x988) == 1`: `(*(u32 *)(this+0x794) + 0x14)(this+0x794)`.
2. `(*(u32 *)(this+0x73C) + 0x14)(this+0x73C)`.
3. `n = fn_80166320(this+0x73C, 0) != 0 ? 1 : 0`.
4. `fn_80ab1df0(this)`.
5. `dir = { *(f32 *)(0x80B2FB50 + *(u8 *)(this+0x348)*4), 0.0, 0.0 }`;
   if `fn_80ab1d60(this, &dir, 128) != 0` then `n += 1`.
6. `fn_80ab1900(this)`.
7. If `n == 2`:
   1. `*(f32 *)(this+0xE8) = *(u8 *)(this+0x348) == 0 ? 0.3 : -0.3`.
   2. `*(f32 *)(this+0xF4) = *(f32 *)(this+0xE8)`; `*(f32 *)(this+0xEC) = 0.0`.
   3. `*(f32 *)(this+0x890) = *(f32 *)(this+0xAC)`,
      `+0x894 = +0xB0`, `+0x898 = +0xB4`.
   4. `fn_80166340(this+0x73C, 1, 0)`;
      if `*(u32 *)(this+0x988) == 1` then `fn_80166340(this+0x794, 1, 0)`.
   5. `fn_80ab1780(8.0, this, 0)`.
   6. `(*(u32 *)(this+0x60) + 0xD4)(this, 0x80B74FD0)`.
8. `fn_80ab1830(this)`; return.

### Constants

| value | meaning |
|---|---|
| `0x80B2FAE4` = `0.0` | `+0xEC` and the record's second/third words |
| `0x80B2FAE8` = `8.0` | `fn_80ab1780` clip rate |
| `0x80B2FB50` = `{1.0, -1.0, 0.0, -0.25}` | direction table, index `+0x348` |
| `0x80B54BA0` = `0.3` | `+0xE8`/`+0xF4` dash speed, negated for `+0x348 != 0` |
| `0x80B54BBA` = `128` (`s16`) | aim rate for `fn_80ab1d60` |
| `0x80B74FD0` | dash state object **(bss)** |

### Port notes and verdict

`fn_80ab1d60` returns `1` on arrival (its `+0x100` reached the target through
`0x8015F4F0`), so `n == 2` means "the `+0x73C` element is in range **and** the
heading arrived". The direction vector's `y`/`z` words are literal zeros
(`li`), not leftovers. `fn_80ab1780` is called with `f1 = 8.0` and index `0`.
**Verdict: ready.**

## 8. `0x80A84A00` - 10-link chain emitter

* **Extent** 400 bytes (100 words), 100 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80a84a00_FUN_80a84a00.c`.
* **Signature** `unsafe fn fn_80a84a00(this: usize) -> u32` - always returns `1`.
* **Role** render half of the spiral actor whose physics half is rank 12: for each
  of ten links it takes the solved position at `+0xC24 + i*0xC`, the three `s16`
  angles at `+0xD14 + i*6`, builds the matrix, and pushes it into the scene node
  at `+0x584 + i*0x40`. The two guard bytes `+0xE00` and `+0xDF7` select
  "already done" and "release only" paths.

### Behaviour

1. `_savegpr_25`.
2. If `*(u8 *)(this+0xE00) != 0` return `1`.
3. If `*(u8 *)(this+0xDF7) == 0`:
   `(*(u32 *)(this+0x544) + 0x14)(this+0x544)`;
   if `*(u16 *)(this+0xDCA) == 2` then `(*(u32 *)(this+0xE48) + 0x10)(this+0xE48)`;
   return `1`.
4. For `i = 0; i < 10; i++`:
   1. `pos = { *(f32 *)(this+0xC24 + i*0xC), *(f32 *)(this+0xC28 + i*0xC),
      *(f32 *)(this+0xC2C + i*0xC) }`.
   2. `ang = { *(s16 *)(this+0xD14 + i*6), *(s16 *)(this+0xD16 + i*6),
      *(s16 *)(this+0xD18 + i*6) }`.
   3. `fn_80064af0(&pos, &ang, 1)`; `fn_801c0bd0(pos[0], pos[1], pos[2], &mtx)`.
   4. `fn_8016edd0(this+0x7C, &ang[1])` (Y-rotation on the node itself).
   5. `node = this + 0x584 + i*0x40`;
      `fn_8016a170(node, &mtx)`;
      `fn_8016a150(node, this+0xC9C + i*0xC)`;
      `fn_8016a1a0(node, 0)`;
      `(*(u32 *)node + 0x14)(node)`.
5. Return `1`.

### Constants

| value | meaning |
|---|---|
| `10` | link count |
| `+0xC24` (stride `0xC`) | solved position array (written by rank 12) |
| `+0xD14` (stride 6) | per-link `{pitch, yaw, roll}` `s16` triple |
| `+0xC9C` (stride `0xC`) | per-link scene point |
| `+0x584` (stride `0x40`) | per-link scene node with a vtable at `+0` |
| `+0x7C` | the actor's own node for the Y-rotation |
| `+0xDCA`, `+0xDF7`, `+0xE00`, `+0xE48` | mode halfword, release flag, done flag, second holder |

### Port notes and verdict

The two `stfs`/`sth` groups at `0x80A84A84..0x80A84AB0` (the actor's own
`+0xAC..+0xB4` and `+0x100..+0x104`) are **overwritten** by the per-link values
at `0x80A84AB4..0x80A84AD8` before either is read; mark them `// dead:`. The
`s16` handed to `0x8016EDD0` is `ang[1]` (`+0xD16`), not `ang[0]`. The node
pointer is re-loaded from `+0x584 + i*0x40` for the virtual call, so the scene
node owns its own vtable. **Verdict: ready.**

## 9. `0x80AA0530` - `daEnKanon_c` dash/overlap state step

* **Extent** 388 bytes (97 words), 97 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80aa0530_FUN_80aa0530.c`.
* **Signature** `unsafe fn fn_80aa0530(this: usize)` - `r3` only, returns `void`.
* **Role** the per-frame step of the `daEnKanon_c` actor
  (see `fn_80aa00a0`, `src/game/enemies.rs:46460`): it integrates the two
  standard helpers, reads the `fn_80a9ff20` flag byte, starts or ends the dash
  (`+0x86E`), keeps the dash speed in `+0xE8` (from the `+0x348` table) or
  restores the saved `+0x838`, resets the x position to `+0xB8` on a wall hit,
  and exits to state `0x80B74688` when `|dx| >= 0.1` with the overlap bit set.

### Behaviour

1. `fn_8006cde0(this)`, `fn_800955f0(this)`.
2. `dx = *(f32 *)(this+0xAC) - *(f32 *)(this+0x238)` (read after the two helpers,
   and only consumed by the step-6 wall test).
3. `flags = fn_80a9ff20(this)`.
4. If `(flags & 1) != 0`:
   1. `*(u16 *)(this+0x86E) = 1`; `*(u32 *)(this+0x850) = 0`; `*(u16 *)(this+0x86C) = 0`.
   2. `*(f32 *)(this+0xE8) = tbl[*(u8 *)(this+0x348)]`.
   3. If `*(f32 *)(this+0x440) != 0.0`:
      `*(u16 *)(this+0x86C) = 1`; `*(f32 *)(this+0x838) = *(f32 *)(this+0x440)`.
5. Else if `*(u16 *)(this+0x86E) != 0`:
   1. `*(u16 *)(this+0x86E) = 0`.
   2. If `*(u16 *)(this+0x86C) == 0`:
      `*(f32 *)(this+0xAC) = *(f32 *)(this+0xB8)`; `fn_80072bc0(this+0x1EC)`;
      if `*(u32 *)(this+0x850) == 0` then `(*(u32 *)(this+0x60) + 0xD4)(this, 0x80B74688)`.
   3. Else `*(f32 *)(this+0xE8) = *(f32 *)(this+0x838)`.
   4. `*(u16 *)(this+0x86C) = 0`.
6. If `(flags & 4) != 0 && |dx| >= 0.1`:
   `(*(u32 *)(this+0x60) + 0xD4)(this, 0x80B74688)`.
7. `(*(u32 *)(this+0x54C) + 0x1C)(this+0x54C)`; `fn_80aa00a0(this)`.

### Constants

| value | meaning |
|---|---|
| `0x80B2F240` = `{0.5, -0.5, 0.0, 8.0}` | dash speed table, index `+0x348` |
| `0x80B2F2F4` = `0.0` | `+0x440` "no saved speed" test |
| `0x80B2F368` = `0.1` | wall-hit distance threshold |
| `0x80B74688` | exit state object **(bss)** |
| `+0x86C`, `+0x86E` | dash-active latch, dash-restore latch |
| `+0x838`, `+0x850`, `+0x440` | saved speed, second latch, speed source |

### Port notes and verdict

`|dx|` is computed with `fabs` + `frsp` then compared with `cror`-rewritten
`cr0` (`gt|eq`), so the test is `>= 0.1` including equality. The `+0xE8` write in
step 4.2 happens **before** the `+0x440` test, so a zero `+0x440` still leaves the
speed overwritten. `fn_80a9ff20`'s bit roles (bit 0 contact, bit 2 overlap) are
documented at `src/game/enemies.rs:143087`. **Verdict: ready.**

## 10. `0x80AB29C0` - 5-link chain integrator

* **Extent** 376 bytes (94 words), 94 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80ab29c0_FUN_80ab29c0.c`.
* **Signature** `unsafe fn fn_80ab29c0(this: usize)` - `r3` only, returns `void`.
* **Role** the integrator for the world-map 5-link chain: it advances the yaw and
  roll halfwords of links 1..4, lets `fn_80ab1f70`/`fn_80ab1f10` step each axis,
  integrates `+0x8CC..` from the `+0x908..` velocities, clamps a negative
  vertical velocity's target to `0.0`, then finalises `+0x768`, `+0x794` and
  counts the `+0x9A0` timer down by `5.0` per frame.

### Behaviour

1. Spill `f31`; `_savegpr_27`.
2. `fn_8006cde0(this)`, `fn_8006cd40(this)`.
3. For `i = 1; i <= 4; i++`:
   1. `*(s16 *)(this+0x93E + (i-1)*6) += 0x400`;
      `*(s16 *)(this+0x940 + (i-1)*6) += 0x200`.
   2. `fn_80ab1f70(this, i)`; `fn_80ab1f10(this, i)`.
   3. If `*(f32 *)(this+0x90C + (i-1)*0xC) < 0.0` then
      `*(f32 *)(this+0x968 + (i-1)*4) = 0.0`.
   4. `*(f32 *)(this+0x8CC + (i-1)*0xC) += *(f32 *)(this+0x908 + (i-1)*0xC)`;
      `*(f32 *)(this+0x8D0 + (i-1)*0xC) += *(f32 *)(this+0x90C + (i-1)*0xC)`;
      `*(f32 *)(this+0x8D4 + (i-1)*0xC) += *(f32 *)(this+0x910 + (i-1)*0xC)`.
4. `(*(u32 *)(this+0x768) + 0x14)(this+0x768)`.
5. If `*(u32 *)(this+0x988) == 1`:
   `(*(u32 *)(this+0x794) + 0x14)(this+0x794)`;
   if `*(u32 *)(this+0x9A0) != 0` then
   `c = (s32)((f32)(u32)*(this+0x9A0) - 5.0)` (the `0x80B2FB40` `2^52+2^31`
   fold, then `fctiw`), `*(u32 *)(this+0x9A0) = c`, and
   `if (c < 1) *(u32 *)(this+0x9A0) = 0`.
6. `_restgpr_27`; return.

### Constants

| value | meaning |
|---|---|
| `0x80B2FAE4` = `0.0` | vertical-velocity target clamp |
| `0x80B2FB40` = `2^52 + 2^31` (double) | `u32 -> f64` fold for the timer |
| `0x80B54BB4` = `5.0` | per-frame timer decrement |
| `0x400` / `0x200` | yaw roll-up per frame / roll roll-up per frame |
| `+0x8C0` (stride `0xC`) | link positions (`+0x8CC` with the `r29 = this+0xC` bias) |
| `+0x900` (stride `0xC`) | link velocities |
| `+0x938`/`+0x93C` (stride 6) | link angle triples; links 1..4 only |
| `+0x964` (stride 4, biased to `+0x968`) | per-axis targets |
| `+0x768`, `+0x794`, `+0x988`, `+0x9A0` | two holders, variant flag, timer |

### Port notes and verdict

The pointer bases in the original are biased by `+0xC` (`r29 = this+0xC`), `+6`
(`r30`) and `+4` (`r28`); the offsets above are already de-biased, so a direct
transcription reads `+0x8CC`, `+0x93E` and `+0x968` respectively. Only the
**second and third** halfwords of each link's angle triple are advanced. The
timer is a `u32` decremented by 5 and clamped to 0, and `fctiw` rounds to
nearest, so values are exact multiples of 5. **Verdict: ready.**

## 11. `0x80A53850` - rail-record snap plus surface-crossing effect

* **Extent** 368 bytes (92 words), 92 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80a53850_FUN_80a53850.c`.
* **Signature** `unsafe fn fn_80a53850(this: usize)` - `r3` only, returns `void`.
* **Role** it asks the `0x50`-entry record table at `*(u32 *)0x8042A100` for the
  nearest record to `(this+0xAC, this+0xB0)` with the layer byte `+0x38F`,
  combines the record's height with the stage surface height at
  `*(u32 *)0x80429DD0 + 0x8FEF0` (only when that object's `+0x9002C` mode byte is
  `3`), and emits the crossing effect `0x80065DC0` when the result lies strictly
  between `+0xBC` and `+0xB0`.

### Behaviour

1. `_savegpr_25`; `p = { *(f32 *)(this+0xAC), *(f32 *)(this+0xB0) }` on the
   stack; `pair`, `height` and `angle` are written by the callee.
2. `kind = fn_800eb930(*(u32 *)0x8042A100, &p, &pair, &height, &angle, *(u8 *)(this+0x38F))`
   (`r3` is the `s8` kind byte; the table base is the global's value).
3. `flags = (kind == 1) ? 1 : 0`.
4. `g = *(u32 *)0x80429DD0`; if `*(u8 *)(g + 0x9002C) == 3` then
   `flags |= 2; h2 = *(f32 *)(g + 0x8FEF0)`.
5. If `flags == 2` then `height = h2`; else if `flags == 3 && height <= h2` then
   `height = h2`.
6. If `flags != 0 && ((*(f32 *)(this+0xBC) <= height && height < *(f32 *)(this+0xB0))
   || (*(f32 *)(this+0xB0) <= height && height < *(f32 *)(this+0xBC)))`:
   1. `scale = 0.7`; `dx = 8.0`;
      if `*(u32 *)(this+0x5D0) != 0` then `scale = 1.0; dx = 16.0`.
   2. If `*(f32 *)(this+0xE8) < 0.0` then `dx = -dx`.
   3. `fn_80065dc0(this, f1 = scale, { *(f32 *)(this+0xAC) + dx, height, 6500.0 })`.

### Constants

| value | meaning |
|---|---|
| `0x8042A100` | pointer to the `0x50`-entry record table (main.dol) |
| `0x80429DD0` | stage/scene object pointer; `+0x9002C` mode byte, `+0x8FEF0` surface height (main.dol) |
| `0x80B2CD68` = `0.0` | direction-sign threshold |
| `0x80B2CD80` = `1.0` | alternative scale |
| `0x80B2CDD8` = `0.7` | default scale |
| `0x80B2CDDC` = `8.0`, `0x80B2CDE0` = `16.0` | x offsets |
| `0x80B2CDE4` = `6500.0` | record `z` sentinel |
| `+0x5D0`, `+0xE8`, `+0xBC`, `+0x38F` | variant flag, x speed, second height, layer byte |

### Port notes and verdict

The two `fcmpo`+`cror` pairs implement `0 <= 1`-style rewrites, so step 6 is a
strict interval test on `[min, max)` with the two heights in either order. The one
unresolved item is the pair of `main.dol` stage fields at `0x80429DD0 + 0x9002C`
and `+0x8FEF0`: nothing in this REL reads or writes them, and their roles
("stage mode" and "surface height") are inferred from the `== 3` gate and the
`max`/`replace` arithmetic alone. Every store, branch, constant and call argument
is otherwise exact, so the body can be typed the moment a caller-side or
`main.dol`-side note for those two fields lands. **Verdict: conditional** - one
bounded step: name `0x80429DD0 + 0x9002C` / `+0x8FEF0`.

## 12. `0x80A88BC0` - 10-segment spiral position solver (`qr3`)

* **Extent** 368 bytes (92 words), 92 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80a88bc0_FUN_80a88bc0.c`.
* **Signature** `unsafe fn fn_80a88bc0(this: usize)` - `r3` only, returns `void`.
* **Role** physics half of rank 8: it decays the scroll speed `+0xD60` by `0.25`
  to a floor of `0.0`, advances the phase `+0xD5C`, and solves the ten
  `+0xC24 + i*0xC` positions from the ten `u16` angles at `+0xDDC + i*2` as
  `this + phase*(cos, sin)` with a `+16.0` y offset and a `+20.0` z offset. When
  the scroll has stopped it counts `+0xDDA` down and disposes the actor at zero.

### Behaviour

1. Spill `f31`/`f30`/`f29` (`stfd` + `psq_st ...,qr0` pairs); `_savegpr_25`.
2. `*(f32 *)(this+0xD60) -= 0.25`; `if (*(f32 *)(this+0xD60) < 0.0) *(f32 *)(this+0xD60) = 0.0`.
3. `*(f32 *)(this+0xD5C) += *(f32 *)(this+0xD60)`.
4. For `i = 0; i < 10; i++`:
   1. `a = (f32)(u16)*(s16 *)(this+0xDDC + i*2)`   (`psq_l ...,1,qr3`).
   2. `c = cos(*(f32)(1/256) * a)`; `s = sin(*(f32)(1/256) * a)` (`f64` results).
   3. `ph = *(f32 *)(this+0xD5C)`.
   4. `*(f32 *)(this+0xC24 + i*0xC) = *(f32 *)(this+0xAC) + (f32)(ph * c)`;
      `*(f32 *)(this+0xC28 + i*0xC) = (f32)(*(f32 *)(this+0xB0) + (f32)(ph * s)) + 16.0`;
      `*(f32 *)(this+0xC2C + i*0xC) = *(f32 *)(this+0xB4) + 20.0`.
5. If `*(f32 *)(this+0xD60) == 0.0`:
   `if (*(u16 *)(this+0xDDA) != 0) *(u16 *)(this+0xDDA) -= 1;`
   `if (*(u16 *)(this+0xDDA) == 0) fn_80162510(this)`.
6. `_restgpr_25`; return.

### Constants

| value | meaning |
|---|---|
| `0x80B2E5DC` = `0.25` | scroll decay per frame |
| `0x80B2E4B4` = `0.0` | scroll floor |
| `0x80B2E6A4` = `0.00390625` (`1/256`) | angle -> radian scale |
| `0x80B2E5AC` = `16.0` | y offset |
| `0x80B2E6F0` = `20.0` | z offset |
| `GQR3 = 0x00050005` | `qr3` = one unsigned `u16`, scale 0 |
| `+0xD5C`, `+0xD60`, `+0xDDA`, `+0xDDC` | phase, scroll, countdown, ten `u16` angles |

### Port notes and verdict

Both `qr3` sites read the **same** halfword, re-staged on the stack at `r1+0xA`
and `r1+0x8`; with `GQR3 = 0x00050005` the load is exactly `(f32)(u16)`. The
`1/256` scale is applied to the `f32` before the `sin`/`cos` call, and the
results are `f64` (A-form `fmuls` with `f29` then the `f64` call). The `y`
expression is `(f32)(y + (f32)(ph*s)) + 16.0`, i.e. the `16.0` is added after the
single-precision narrowing, and `z` uses `+0xB4 + 20.0` with no phase term. The
`fcmpo`+`beq` pair at `0x80A88CD0` is an exact `== 0.0` test. **Verdict: ready.**

---

## Verification

* Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 12 sizes sum to 4,976
  bytes and the twelve walks reach 1,244 instructions, i.e. `reachable == extent`
  for every row.
* Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
* All twelve addresses are absent from `ledger/functions.csv`, from every `^#`
  heading of every **other** `docs/*.md`, and from every `fn_<addr>`/`FN_<addr>`
  token in `src/**/*.rs` (checked mechanically, read-only, before this page was
  written; this page is now the only place the twelve appear as headings).
* No Rust file, no ledger row and no other document was written by this pass; the
  only file created is `docs/SPEC_WAVE148_ENEMIES.md`.
* No body was walked through a `bctrl` as a terminator: every `0x4E800421` is
  followed through to `pc+4`, and every `0x4E800420` ends the walk.
* Total `qr3` sites claimed: 2 (rank 12). All other `psq` words in the twelve
  bodies are `qr0` `f29`/`f30`/`f31` spills.
