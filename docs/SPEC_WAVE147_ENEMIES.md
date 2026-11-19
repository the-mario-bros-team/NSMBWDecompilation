# Wave 147 - implementation specification for 12 untranslated `d_enemiesNP` bodies

## What this page is

`docs/ANALYSIS_WAVE127_ENEMIES.md` reclassified 384 formerly-skipped `d_enemiesNP`
addresses into 173 class **B**, 160 class **C** and 51 class **D**. Waves 128-146
worked through that block; this page is the next plain size-ranked tranche: the
twelve largest remaining bodies that are untranslated, have a Ghidra export, and
are fully walkable from their entry.

A live re-walk of `work/ghidra/rel-d_enemiesNP/export/functions.tsv` (3,555 rows)
against the `address` column of `ledger/functions.csv` (16,910 rows), against
every `0x80Axxxxx`/`0x80Bxxxxx` address in a `^#` heading of any `docs/*.md`
(1,448 distinct addresses) and against every `fn_<addr>`/`FN_<addr>` token in
`src/**/*.rs` (16,027 distinct addresses) leaves **133** untranslated bodies, all
133 with an export.

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
| 1 | `0x80A56B10` | 672 | 168 | 0 | firebar model emitter over the `+0x2484` segment table | **ready** |
| 2 | `0x80A6FBC0` | 624 | 156 | 0 | animation-gated sway ramp with a class state machine | **ready** |
| 3 | `0x80ADDE10` | 580 | 145 | 0 | bubble/impact state entry with three state lookups | **ready** |
| 4 | `0x80AD4000` | 532 | 133 | 0 | `+0x664/+0x662` gate: kill, detach and respawn record | **ready** |
| 5 | `0x80A1EBC0` | 520 | 130 | 6 | circle/offset position solve for the `0x80A1E990` actor | **ready** |
| 6 | `0x80A20900` | 520 | 130 | 0 | four-player proximity damage emitter (`0x43B`) | **ready** |
| 7 | `0x80B0D4D0` | 520 | 130 | 0 | turn-order verdict over the four players | **ready** |
| 8 | `0x80B04A90` | 512 | 128 | 0 | chain-segment initialiser from the stage section table | **ready** |
| 9 | `0x80B16CE0` | 504 | 126 | 0 | `unizo` / `unizoBig` brres + clip binder | **ready** |
| 10 | `0x80A06560` | 500 | 125 | 2 | four-slot attachment placement (circle and record modes) | **ready** |
| 11 | `0x80B115A0` | 480 | 120 | 0 | yaw-pitch-yaw spawn matrix + render-item build | **ready** |
| 12 | `0x80A56920` | 476 | 119 | 4 | firebar phase advance and segment spiral | **ready** |

**Split: 12 ready, 0 conditional.** Total 6,440 bytes of TSV extent and 1,610
reachable instructions. Every one of the twelve walks to a terminal instruction
at exactly `entry + size - 4`, so `reachable == extent` for all twelve and the
"extent is padding" caveat of the earlier waves does not apply here.

Each row's residual external operand is named in its **Port notes** bullet; none
of them is a value that has to be recovered before the body can be typed:
they are `main.dol` addresses, translated helpers, or runtime state that the
port already has to touch.

### Why wave 146's five held-back rows are claimed here

`docs/SPEC_WAVE146_ENEMIES.md:52` lists `0x80A56B10`, `0x80A6FBC0`,
`0x80ADDE10`, `0x80AD4000` and `0x80A20900` as "rows a size sort would place
higher", with reasons of the form "body ends in a vtable `bcctr`" or "main path
enters `lwzu r12,0x20D0(r3)` + vtable `+0xA0`; the walk cannot follow it".

That diagnosis does not survive a look at the encoding. Every one of those
"`bcctr`" words is **`0x4E800421`**, which is `bctrl`: opcode 19, XL-form,
`LK = 1` (bit 31), `BO = 20`. It is a **linked indirect call that returns**, not
a tail branch. A walk that treats it as a terminator stops at the first virtual
call and reports a 16- or 32-instruction body; a walk that follows it (fall
through to `pc+4`) reaches the real end:

| address | wave 146 "reachable" | real reachable | real terminal |
|---|---:|---:|---|
| `0x80A56B10` | 32 | 168 | `0x80A56DAC bclr` |
| `0x80A6FBC0` | 16 | 156 | `0x80A6FE2C bclr` |
| `0x80ADDE10` | 13 | 145 | `0x80ADE050 bclr` |
| `0x80AD4000` | 27 | 133 | `0x80AD4210 bclr` |
| `0x80A20900` | 40 | 130 | `0x80A20B04 bclr` |

All five have `reachable == extent`, no zero word, no unknown opcode, and a
`bclr` exactly at the last word of the extent. They are ordinary bodies with
9, 9, 8, 1 and 5 virtual calls respectively, and they are ranks 1-4 and 6 here.

`0x80A6FBC0` is additionally mis-classified by `docs/ANALYSIS_WAVE127_ENEMIES.md:245`
as class C for an "incoming `f1` (float argument) plus FPR spill". The
`stfd f31,0x20(r1)` / `psq_st f31,0x28(r1),qr0,0` pair at `0x80A6FBCC..0x80A6FBD0`
is the f31 **spill** (restored at `0x80A6FE10..0x80A6FE14`); the body has no
float argument. This is the same artefact `docs/ANALYSIS_WAVE127_ENEMIES.md:80`
describes as finding 1.

### Rows below the cut

| address | TSV size | reachable | why it is not claimed |
|---|---:|---:|---|
| `0x80B058A0` | 476 | 92 | stops at `0x80B05980` on `0x7C64042E`: opcode 31 with the 10-bit extension word `0x217` (the `lwzx` placement with extension bit 21 set), which this lane's table does not classify |
| `0x80A2EC00` | 472 | 118 | next in size order after the tranche; clean, left for wave 148 |
| `0x80A81150` | 456 | 114 | clean |
| `0x80A5EB00` | 448 | 112 | clean |
| `0x80AE69E0` | 444 | 61 | stops on the same `0x217` extension word |
| `0x80A5C660` | 436 | 109 | clean |
| `0x80B27ED0` | 432 | 108 | clean, 2 `qr3` sites |
| `0x80A00D00` | 424 | 106 | clean |

`0x80B058A0` ties `0x80A56920` at 476 bytes. The tie is broken on the walk: 119
instructions reachable versus 92, and `0x80B058A0`'s walk cannot be completed.

Twelve of the 133 remaining rows stop on that same `0x217` word (for example
`0x80B252F8` `0x7C04DC2E`, `0x80AC88D0` `0x7C64042E`, `0x80AB2730`,
`0x80AE6AD0`, `0x80A94214`, `0x80AA05A8`). Fifteen rows in total stop short of
their extent: those twelve, plus `0x80A03C30`, `0x80A91D30` and `0x80AF2400`,
which are short bodies followed by padding. None of the fifteen is claimed.

## The `qr3` pool at this wave

`docs/REL_ELF_MAPPING.md` pins **`GQR3 = 0x00050005`** (unsigned 16-bit, scale
0), installed by `main.dol` at `0x8025EEEC`/`0x8025EEF0`, and records that the RAW
stream of this REL contains no `mtspr GQRn`. A re-scan of the extents of all 133
remaining bodies and of the twelve claimed here finds zero such sites. With
`W = 1` and `I = 3` every data `psq` here is one unsigned 16-bit load scaled by
`2^0`:

```
psq_l fD,0(rA),1,qr3      ; fD = (f32)*(u16 *)rA
```

Three of the twelve rows use it, twelve sites in total:

| address | sites | what the halfword is |
|---|---|---|
| `0x80A1EBC0` | `0x80A1EC0C`, `0x80A1EC38`, `0x80A1ECB4`, `0x80A1ECE0`, `0x80A1ED1C`, `0x80A1ED38` | a `s16` angle staged on the stack (`sth` then `psq_l` from `r1+off`) |
| `0x80A06560` | `0x80A06660`, `0x80A06680` | `(s16)(+0x115E + table[t+0x5C0])` - one angle, read twice |
| `0x80A56920` | `0x80A569BC`, `0x80A569D8`, `0x80A56A30`, `0x80A56A58` | the `+0x2480` phase, and `phase - 0x8000` for the mirrored half |

In all three bodies the halfword is a stack temporary and is pre-scaled by
`1/256` (`0x80B2B6F0`, `0x80B2AA9C`, `0x80B2CEFC`) before the `sin`/`cos` call, so
the port reads `(f32)(u16)halfword` and multiplies by `1/256`.

Twelve `qr3` bodies were untranslated before this wave; nine remain after it, all
at or below 432 bytes: `0x80B27ED0` (432, 2 sites), `0x80A88BC0` (368, 2),
`0x80A58D70` (308, 1), `0x80AF3400` (308, 2), `0x80A80F40` (264, 1),
`0x80A584E0` (232, 2), `0x80A80E50` (220, 1), `0x80AF3570` (196, 2),
`0x80B22750` (128, 1). They are the continuation of the `qr3` tranche.

## Reading conventions

* Synthetic base is `0x80A00000`. The REL has five file-backed `PT_LOAD`
  segments; the `+0x1000` formula of `docs/REL_ELF_MAPPING.md` applies to the
  first one (`p_offset = 0x10F0`, `p_vaddr = 0x80A000F0`). The file-backed image
  ends at `0x80B6E438`; the last `PT_LOAD` is the zero-fill segment
  `0x80B6E500` (`memsz = 0xB4E0`), so anything at or above `0x80B6E500` is BSS
  and is marked **(bss)**.
* `this` is the incoming `r3` unless stated otherwise; `+0xNNN` are byte offsets
  from `this`. `+0xAC`/`+0xB0`/`+0xB4` are x/y/z; `+0x100`/`+0x102`/`+0x104`
  are the three s16 angles (x/yaw/roll); `+0xDC`/`+0xE0`/`+0xE4` are the
  colour/scale triple; `+0x348` is the side/facing byte; `+0x38F` is the player
  id byte; `+0x60` is the actor's class vtable; `+0x394` is the state sub-object.
* `addi rX,r0,imm` is `li`, `addis rX,r0,imm` is `lis`, `or rX,rY,rY` is `mr`.
  **`addi r0,r0,imm` (D = 0, A = 0) is `li r0,imm`**, not a negation of `r0`;
  this matters for the `-0x1D00` clamp in rank 2.
* `rlwinm rA,rS,sh,mb,me` is `ROTL32(rS,sh) & mask(mb..me)` with the mask bits
  numbered **from the MSB** (bit 0 = `0x80000000`, bit 31 = `1`). So
  `rlwinm r0,r3,0,28,31` is `r3 & 0xF`, `rlwinm r3,r0,28,28,31` is
  `(r0 >> 4) & 0xF`, `rlwinm r0,r3,0,16,31` is `r3 & 0xFFFF`, and
  `rlwinm r4,r4,1,0,30` is `r4 * 2`.
* On a `bc`, `bo = 12` branches on a true CR bit and `bo = 4` on a false one;
  `bi & 3` is `LT/GT/EQ/SO`. `cror 2,1,2` (`0x4C411382`, XO 449) after an
  `fcmpo` rewrites `cr0.eq` as `gt|eq`, so the following `bne` reads
  `if !(a >= b)`; `cror 2,0,2` (`0x4C401382`) gives `lt|eq`.
* `bo = 4, bi = 0` is "branch if not LT" (`bge`), `bo = 4, bi = 1` is
  "branch if not GT" (`ble`).
* `psq_st`/`psq_l` with `qr0` into `0x30..0x58(r1)` next to a matching
  `stfd`/`lfd` is an **f31..f27 spill**, not quantisation. The `unaff_GQR0`
  noise and the `ldexpf` calls in the Ghidra exports of this module are that
  spill plus the `2^52` integer-to-double idiom (`0x43300000` high word, value
  `^ 0x80000000` in the low word, subtract `2^52 + 2^31`).
* `0x4E800421` is `bctrl` (`LK = 1`): an indirect call that **returns**. Only
  `0x4E800420` (LK = 0) is a tail `bcctr`.

## Symbolic helpers used below

| address | symbol | contract used here |
|---|---|---|
| `0x8005F900` | `fn_8005f900(index) -> *mut actor` | player actor by index `0..3`, `0` when absent |
| `0x80056CF0` | `fn_80056cf0(player, flag) -> u32` | player flag bit: reads the word at `+0x1014 + 4*(flag>>5)`, returns bit `flag & 31` |
| `0x80162D00` | `fn_80162d00(id) -> *mut actor` | registry lookup by object id (`id & 7` selects a `0x20`-byte bucket) |
| `0x800777B0` | `fn_800777b0(obj, u16, u16, u8)` | cell decode (`0x80077730`) then the `0x80082140` layer store |
| `0x80161800` | `fn_80161800(key, node, value)` | unlink `node` from its list when `*node == key`, clear both links |
| `0x80161750` | `fn_80161750(key, node, value)` | append `node` at the list tail and store `key` at `+0x00` |
| `0x80078520` | `fn_80078520(index, 0) -> usize` | `*(u32 *)(0x90A40 + index*4) + 0x38` |
| `0x80014820` | `fn_80014820(node, list)` | intrusive list append (count/head/tail at `+0/+4/+8`) |
| `0x80022600` | `fn_80022600(this, &rec)` (unledgered) | submit the 20-byte actor record built on the stack |
| `0x800D0470` | `fn_800d0470(mgr, node, u8, kind, extra)` | effect-record submit (pos/kind/extra/code at `+0x0C..+0x20`) |
| `0x8005FB90` | `fn_8005fb90() -> *mut player` | resolve the current player (table A, event-flag `0x4B`) |
| `0x80057EE0` | `fn_80057ee0(player, id, flag, 0)` | player interface dispatch, `+0xDEC` slot `0x30` |
| `0x8008C380` | `collision_area_unregister(area)` | unlink a collision area (`+0xA2` guard, `+0x10/+0x14` neighbours) |
| `0x8007F900` | `fn_8007f900(list)` | list reset at `r13-0x58A0`, `+0xDC` latch |
| `0x800690C0` / `0x800690E0` | `fn_800690c0` / `fn_800690e0` | fixed-size holder ctor `(this, -1, name, 0, count)` / dtor |
| `0x800DF180` | `fn_800df180(arc, name, path) -> res` | brres resource load |
| `0x80239E30` / `0x8023A0B0` / `0x8023A200` | clip / anm-chr / anm-tex-pat lookup | `(res, name) -> clip` |
| `0x80169D90` | `fn_80169d90` | bind one clip into a model: `(model, &clip, base, kind, 1, 0)` |
| `0x801650D0` / `0x801651F0` / `0x80166830` / `0x80166BD0` | anim-holder pair binders | as used in waves 145-146 |
| `0x80169F80` / `0x80166EF0` / `0x80166F10` | anim rate setters | take `f1` first |
| `0x80064BD0` / `0x80064BF0` / `0x80064C10` | `fn_80064bd0` / `bf0` / `c10` | model finish / visibility on / off |
| `0x80237BD0` / `0x80237C40` | `fn_80237bd0` / `fn_80237c40` | `sin` / `cos` of the `1/256`-scaled angle, `f64` in and out |
| `0x80168080` | `fn_80168080(anim) -> f32` | current frame of an animation element (vtable slot `+0x20`) |
| `0x801690E0` | `fn_801690e0(anim) -> u32` | range containment with rate sign and mode bit 2 |
| `0x80169020` | `fn_80169020(anim) -> u32` | "reached the end" test over slots `+0x20`/`+0x2C` |
| `0x80A207E0` | `fn_80a207e0(this, &rec) -> s16` | two floats to a `s16` angle through `0x801611C0` |
| `0x80A20810` | `fn_80a20810(this, radius, &rec, s16) -> u32` | `|radius| > |vec2|` through two `sqrt` calls |
| `0x802BE470` | `fn_802be470(x) -> f32` | square-root wrapper with `f32` narrowing |
| `0x801C0BD0` | `fn_801c0bd0(x, y, z, out)` | write a 3x4 translation matrix to `out` |
| `0x801C0540` | `fn_801c0540(a, b, out)` | 3x4 concat |
| `0x8016F130` | `fn_8016f130(mtx, out)` | copy the translation column (`+0xC`, `+0x1C`, `+0x2C`) |
| `0x8016ECB0` / `0x8016EDD0` / `0x8016EEF0` | rotation concat X / Y / Z | `(mtx, &s16 angle)`, in place |
| `0x800B8700` | `fn_800b8700(obj, &desc, 1) -> u32` | build `count` render items, returns the 1/0 flag |
| `0x802DCA98` | `fn_802dca98(base, dtor, stride, count)` | array destroy: `dtor(element, -1)` from the end |
| `0x80A0B5A0` | `fn_80a0b5a0` | element destructor of the 60-byte temporary in rank 11 |
| `0x80B04270` | `fn_80b04270(seg, owner, &pos, mode)` | place one chain segment (pos to `+0xA4..0xAC`, latch `+0x1B0`, `+0x198 = 4`) |
| `0x80A06130` | `fn_80a06130(out, record, index)` | record index `((u8[+0xA05]-u8[+0xA00+index])&0x7F)*0x14`; copies 3 words + 3 halfwords |
| `0x80A65CE0` | `fn_80a65ce0(target, &pos)` | set x/y from `pos`, `z = pos.z - 10.0*(+0x5C0+1)` |
| `0x8015F140` | `fn_8015f140(target, rate, limit, deadband, *mut f32) -> f64` | rate/limit/dead-band step on a live `f32` |
| `0x800D90C0` | `stage_object_section_entry(id)` | stage `+0x38` list, count `+0xA8`, 8-byte stride, first byte match |
| `0x80069530` | `fn_80069530(index)` (unledgered) | per-player handle passed to the effect call |
| `0x801995A0` | `fn_801995a0(mgr, id, owner, &pos, s16, handle)` (unledgered) | spawn an effect id on an actor |
| `0x80196EB0` | `fn_80196eb0(mgr, s16)` (unledgered) | single-`s16` effect call |
| `0x802DCD58/60/64/6C/74` and `0x802DCDA4/AC/B0/B8/C0` | `_savegpr_N` / `_restgpr_N` | incoming `r3..` survive; the export's "no arguments" is the restore pair |

---

## 1. `0x80A56B10` - firebar model emitter

* **Extent** 672 bytes (168 words), 168 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80a56b10_FUN_80a56b10.c`.
* **Signature** `unsafe fn fn_80a56b10(this: usize)` - `r3` only, returns `void`.
* **Role** bind the three firebar render holders (`this+0x20D0`, `this+0x2208`,
  `this+0x2340`, 0x138 bytes apart) to this frame's segment table: one object at
  the actor itself plus one per `+0x1EF0/+0x1EF4` offset entry, and a mirrored
  second half when `+0x2486 == 1`. It is the emitter that pairs with the
  classifier at `0x80A56620` (wave 146 rank 2) and with the phase advance at
  `0x80A56920` (rank 12 here).

### Behaviour

1. Save `f31` (spill only), `r25..r31`; `r25 = this`;
   `r28 = 0x80B2CEB8`, `r29 = 0x80B40000`, `r30`/`r31` likewise (literal bases).
2. `pos = { *(f32 *)(this+0xAC), *(f32 *)(this+0xB0) + 8.0, *(f32 *)(this+0xB4) }`
   into `r1+0x14..0x1C`; `scale = {1.0, 1.0, 1.0}` into `r1+8..0x10`.
3. Three emits with that `pos`/`scale`, in this order:
   * `emit(this + 0x20D0, "Wm_en_firebar")`
   * `emit(this + 0x2208, "Wm_en_firebar_fire")`
   * `emit(this + 0x2340, "Wm_en_firebar_ind")`

   where `emit(obj, name)` is
   `(*(u32 *)obj->vtable[0xA0])(obj, name, 0, &pos, 0, &scale)`: `r3 = obj`,
   `r4 = name`, `r5 = 0`, `r6 = &pos`, `r7 = 0`, `r8 = &scale`. The first word of
   each holder is its vtable pointer, loaded with `lwzu r12,0x20D0(r3)` /
   `lwz r12,0x2208(r25)` / `lwz r12,0x2340(r25)`.
4. `scale` is rewritten to `{1.0, 1.0, 1.0}` from `0x80B2CEEC`.
5. `n = *(u16 *)(this+0x2484)`; if `n <= 0` skip to the epilogue.
   For `i = 0 .. n-1`, with `rec = this + i*0xC`:
   1. `pos.x = *(f32 *)(this+0xAC) + *(f32 *)(rec+0x1EF0)`,
      `pos.y = *(f32 *)(this+0xB0) + (8.0 + *(f32 *)(rec+0x1EF4))`,
      `pos.z = *(f32 *)(this+0xB4)`.
   2. The same three emits as step 3.
   3. If `*(s16 *)(this+0x2486) == 1`, repeat with
      `idx = i + n`:
      `pos.x = this.x + *(f32 *)(this + 0x1EF0 + idx*0xC)`,
      `pos.y = this.y + (8.0 + *(f32 *)(this + 0x1EF4 + idx*0xC))`,
      `pos.z = this.z`, then the three emits.
6. Restore `r25..r31`, `f31`; return.

### Constants

| address | value | use |
|---|---|---|
| `0x80B2CEE0` | `1.0f` | scale triple |
| `0x80B2CEEC` | `1.0f` | scale triple, second load |
| `0x80B2CEF0` | `8.0f` | base y offset |
| `0x80B467C4` | `"Wm_en_firebar"` | holder 0 name |
| `0x80B467B0` | `"Wm_en_firebar_fire"` | holder 1 name |
| `0x80B4679C` | `"Wm_en_firebar_ind"` | holder 2 name |
| `0x80B467D8` | `0x80A57220` | class-descriptor word directly after the three names |

### Port notes and verdict

* The three holders are a contiguous row: `0x2208 - 0x20D0 = 0x138`,
  `0x2340 - 0x2208 = 0x138`. Each is an embedded object whose first word is its
  vtable pointer; slot `+0xA0` is the "bind/create from a name, position and
  scale" method. Nothing in the body reads the vtable from a global.
* The `+0x1EF0/+0x1EF4` table is written by rank 12 (`0x80A56920`), so the two
  bodies must be ported together: rank 12 writes `12*(i+1)*cos/sin` and this body
  adds the `8.0` y bias.
* `stfd f31` + `psq_st f31,...,qr0,0` at `0x80A56B20..0x80A56B24` is an f31
  spill; the export's `unaff_GQR0`/`ldexpf` prologue is that spill. No `qr3`
  site, no float argument.
* `0x80A5637C` in `FUN_80a55de0` is the only caller (one `bl`).
* **Verdict: ready.** All strings, offsets, strides and the mirror flag are
  literal; the only virtual call is a method on a holder this body was passed.

---

## 2. `0x80A6FBC0` - animation-gated sway ramp with a class state machine

* **Extent** 624 bytes (156 words), 156 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80a6fbc0_FUN_80a6fbc0.c`.
* **Signature** `unsafe fn fn_80a6fbc0(this: usize)` - `r3` only.
* **Role** per-frame step of an actor whose model holder is `this+0x544` and
  whose animation holder is `this+0x584`: it ramps one of two sway angles
  (`+0x704` or `+0x706`) while the current clip has *not* reached a branch
  length, zeroes the tilt triple when the actor leaves the ground, and runs a
  two-state machine on `+0x360` that ends in a class state change.

### Behaviour

1. `(*(u32 *)(this+0x544) -> vtable[0x1C])(this+0x544)` - one call on the model
   holder's own vtable.
2. `0x8006CDE0(this)`; `0x800955F0(this)` - two argument-free per-frame steps.
3. `len = (*(u32 *)(this+0x60) -> vtable[0x2CC])(this)`;
   `cur = 0x80168080(this+0x584)`.
   * If `cur >= len`: read `f = 0x80168080(this+0x584)` again; if `f >= 28.0f`
     (`0x80B2DAD0`) then `*(s16 *)(this+0x706) -= 0x109`, and if the result is
     `< -0x1D00` store `-0x1D00`.
   * Else: `*(s16 *)(this+0x704) -= *(s16 *)(0x80B2DACC + *(u32 *)(this+0x85C)*2)`
     (both entries of that table are `494`), and if the result is `< -0x1D00`
     store `-0x1D00`.
4. `clear = ((*(u32 *)(this+0x60) -> vtable[0x300])(this) == 0)`.
5. If `clear`:
   * if `0x80072BC0(this+0x1EC) != 0` then
     `*(f32 *)(this+0xE8) = *(f32 *)(this+0xEC) = *(f32 *)(this+0xF0) = 0.0f`
     (`0x80B2DA80`);
   * if `0x8006FA40(this+0x1EC, 0) != 0` then `*(f32 *)(this+0xE8) = 0.0f`.
6. State machine, `s = *(u32 *)(this+0x360)`:
   * `s == 1`: if `0x80168080(this+0x584) >= (vtable[0x2C8])(this)` then
     `(vtable[0x2EC])(this)` and `*(u32 *)(this+0x360) = 2`.
   * `s == 2`:
     * if `0x801690E0(this+0x584) != 0` then `(vtable[0x2F0])(this)`;
     * else if `0x80168080(this+0x584) >= (vtable[0x2CC])(this)`:
       if `0x80169020(this+0x584) != 0` and `clear` then
       `(vtable[0xD4])(this, 0x80352CB4)` (class state change);
     * else `0x80024190(this)` and `(vtable[0x2E8])(this)`.
   * any other `s`: nothing.
7. Epilogue.

### Constants

| address | value | use |
|---|---|---|
| `0x80B2DA80` | `0.0f` | tilt triple reset |
| `0x80B2DACC` | `s16[2] {494, 494}` | per-variant ramp step for `+0x704` |
| `0x80B2DAD0` | `28.0f` | clip-time gate for `+0x706` |
| `-0x1D00` | literal `0xE300` | clamp floor for both angles |
| `0x109` | literal `265` | `+0x706` step |
| `0x80352CB4` | `main.dol` state token | `+0xD4` argument |

### Port notes and verdict

* The two clamp stores are `li r0,-0x1D00` (`0x3800E300`, `D = 0`, `A = 0`), not
  `r0 - 7424`. Keep `v = max(v, -0x1D00)` on the sign-extended `s16`.
* `0x80168080` returns the *current* frame of the animation element (it dispatches
  the element's `+0x20` slot); `vtable[0x2C8]` / `vtable[0x2CC]` return the two
  branch lengths. The three comparisons are all "current >= length".
* `0x80B2DACC` is file-backed and readable - both entries are `0x01EE` - so the
  `+0x85C` index can only be 0 or 1 before it would read the `28.0f` literal
  behind it. Document the index as `0..1`.
* `0x8006CDE0` and `0x80024190` are unledgered `main.dol` bodies called with
  `r3 = this` only; both argument lists are literal in the staging.
* **Verdict: ready.** Every offset, table entry, slot number and constant is in
  this REL; the `+0xD4` call is the class state change with a literal token.

---

## 3. `0x80ADDE10` - bubble/impact state entry

* **Extent** 580 bytes (145 words), 145 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80adde10_FUN_80adde10.c`.
* **Signature** `unsafe fn fn_80adde10(this: usize, variant: u32)` - `r3` actor,
  `r4` variant (the caller `0x80ADDB40` passes `playerByte + 1`).
* **Role** the shared "enter the hurt/knockback state" body of the bubble actor
  family (`0x80AD3D50` in `docs/SPEC_WAVE128_ENEMIES.md` is the step, `0x80AD4000`
  here is the kill gate): it refuses to run while any of three states is
  registered, latches `+0x390`, emits the `0xB` cell effect, re-links the
  `+0x784` record list to the current cell, writes the `+0x760` position record
  and switches to state `0x80352A34` or `0x80352A74`.

### Behaviour

1. `st = (*(u32 *)(this+0x394) -> vtable[0x28])(this+0x394)`; if
   `(st -> vtable[0x14])(st, 0x80352A34) != 0` return. Repeat the same two calls
   for `0x80352A74`, then for `0x80B76DA8` (**bss**). The test is
   `neg; or; rlwinm. 1,31,31`, i.e. "result != 0".
2. `*(u8 *)(this+0x390) = 1`.
3. If `*(u32 *)(this+0x878) == 0xB`:
   `0x800777B0(*(u32 *)0x80429DD0, (u16)(s32)*(f32 *)(this+0x87C),
   (u16)(s32)(-*(f32 *)(this+0x880)), *(u8 *)(this+0x38F))`.
4. `0x80ADE060(this, variant)` - the paired body, same module.
5. `cur = *(u32 *)0x80429DD0`; `old = *(u32 *)(this+0x784)`; if `cur != old`:
   * if `old != 0`: `0x80161800(old + 100, this+0x784, old == -100 ? 0 : old)`;
   * if `cur != 0`: `0x80161750(cur + 100, this+0x784, cur == -100 ? 0 : cur)`.
6. `0x80014820(0x80078520(*(u32 *)(this+0x784), 0), this+0x760)`.
7. `*(u16 *)(this+0x768) = 50`;
   `*(f32 *)(this+0x76C) = *(f32 *)(this+0xAC) - 8.0f`;
   `*(f32 *)(this+0x770) = -(*(f32 *)(this+0xB0) + 8.0f)`;
   `*(f32 *)(this+0x774) = *(f32 *)(this+0xB4)`;
   `*(f32 *)(this+0xE8) = *(f32 *)(this+0xEC) = *(f32 *)(this+0xF0) = 0.0f`.
8. `(*(u32 *)(this+0x60) -> vtable[0xD4])(this, variant == 0 ? 0x80352A34 : 0x80352A74)`.

### Constants

| address | value | use |
|---|---|---|
| `0x80352A34` | `main.dol` state token | first guard and `variant == 0` target |
| `0x80352A74` | `main.dol` state token | second guard and `variant != 0` target |
| `0x80B76DA8` | **(bss)** state token | third guard |
| `0x80B30B44` | `8.0f` | x/y record bias |
| `0x80B30B38` | `0.0f` | tilt triple reset |
| `0x80429DD0` | `u32` global (cell/ground object) | effect call argument and list key |
| `50` | literal `0x32` | `+0x768` |

### Port notes and verdict

* `(-u | u)`'s sign bit is "`u != 0`"; the three guards are ordinary
  "is this state registered?" tests, and the body runs only when all three are
  absent. Keep them as three sequential early returns.
* `+0x784` is an intrusive list node keyed by the cell id from `0x80429DD0`; the
  `+100` is part of the key, and the `== -100 ? 0 : v` fold is the compiled
  "null key" special case. `0x80161800` unlinks, `0x80161750` appends.
* The `(u16)(s32)` casts are the two `fctiwz` results truncated with
  `rlwinm ...,0,16,31`; `-*(f32 *)(this+0x880)` is an `fneg` before the `fctiwz`.
* `0x80B76DA8` is above the file-backed end and below the zero-fill segment, so
  it is BSS; it is used only as an address token, and the port compares the same
  constant.
* `0x80ADE060` is in the TSV (444 bytes, still untranslated) and is the sibling
  "apply the impact" body; it is called with the same `(this, variant)` pair.
* **Verdict: ready.** Three literal state tokens, one literal cell-layer call,
  all offsets literal; no quantiser, no incoming float.

---

## 4. `0x80AD4000` - `+0x664/+0x662` kill gate

* **Extent** 532 bytes (133 words), 133 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80ad4000_FUN_80ad4000.c`.
* **Signature** `unsafe fn fn_80ad4000(this: usize)` - `r3` only.
* **Role** the one-shot transition that turns the bubble actor into its "popped"
  state: it requires the `+0x664` timer to be zero, the `+0x662` counter to be
  non-zero and `+0x68C` to be clear, then unregisters the collision area,
  detaches and re-anchors the `+0x814` record, writes the `+0x698` spawn record,
  pokes the current player and submits both the actor record and the effect
  record.

### Behaviour

1. If `*(u16 *)(this+0x664) != 0` return; if `*(u16 *)(this+0x662) == 0`
   return; if `*(u8 *)(this+0x68C) != 0` return.
2. `(*(u32 *)(this+0x60) -> vtable[0xD4])(this, 0x80352AF4)`.
3. `*(u8 *)(this+0x68A) = 2`; `*(u8 *)(this+0x68D) = 2`;
   `*(u8 *)(this+0x689) = 0`; `*(u16 *)(this+0x664) = 0x10`;
   `*(u32 *)(this+0x84C) = 3`.
4. `0x8008C380(this+0x148)` (collision area unregister);
   `0x8007F900(this+0x534)`.
5. `cur = *(u32 *)0x80429DD0`; `old = *(u32 *)(this+0x814)`; when they differ,
   `0x80161800(old+100, this+0x814, old == -100 ? 0 : old)` then
   `0x80161750(cur+100, this+0x814, cur == -100 ? 0 : cur)` - the same
   unlink/append pair as rank 3, on a different node.
6. `0x80014820(0x80078520(*(u32 *)(this+0x814), 0), this+0x698)`.
7. `*(u16 *)(this+0x6A0) = 50`;
   `*(f32 *)(this+0x6A4) = *(f32 *)(this+0xAC) - 8.0f`;
   `*(f32 *)(this+0x6A8) = -(* (f32 *)(this+0xB0) + 16.0f)`.
8. If `*(s8 *)(this+0x694) != -1`:
   `p = 0x8005FB90()`; if `p != 0` then
   `0x80057EE0(p, 0x273, *(u32 *)(p+0x1090) == 3, 0)`.
9. Build the 20-byte record at `r1+8`:
   `{f32 x, f32 y, f32 z}` from `+0xAC/+0xB0/+0xB4`,
   `u8[0x0C] = *(u8 *)(this+0x694)`, `u32[0x10] = *(u32 *)(this+0x854)`,
   `u8[0x14] = 0`, `u8[0x15] = (*(u32 *)(this+0x858) != 0)`,
   `u8[0x16] = *(u32 *)(this+0x71C) != 0 ? (u8)(*(u32 *)(this+0x718) + 1) : 0`;
   `0x80022600(this, &rec)`.
10. `0x800D0470(*(u32 *)0x80429FC8, this+0x81C,
    *(u8 *)(*(u32 *)0x8042A1C8 + 0x120E), 0, *(u32 *)(this+0x850))`.

### Constants

| address | value | use |
|---|---|---|
| `0x80352AF4` | `main.dol` state token | `+0xD4` argument |
| `0x80B30920` | `8.0f` | `+0x6A4` x bias |
| `0x80B3093C` | `16.0f` | `+0x6A8` y bias |
| `0x273` | literal `627` | `0x80057EE0` id |
| `0x10` / `3` / `2` / `50` | literals | `+0x664`, `+0x84C`, `+0x68A/+0x68D`, `+0x6A0` |
| `0x80429DD0`, `0x80429FC8`, `0x8042A1C8+0x120E` | `main.dol` globals | cell key, display manager, scene byte |

### Port notes and verdict

* The record layout at step 9 is byte-identical to the sibling body in
  `docs/SPEC_WAVE128_ENEMIES.md:781`, which is the cross-check for the offsets
  `+0x0C/+0x10/+0x14..+0x16`.
* `0x8005FB90()` takes no argument here; `r3` at the call site is the return of
  `0x80014820` and is dead. The player pointer comes from the global table.
* The `+0x1090 == 3` test is an `li r5,0` / `li r5,1` fold, i.e. the third
  argument is `0`/`1`.
* The y record is **negative**: `fneg` after the `16.0` add. This differs from
  rank 3 (`8.0`, also negated) and is easy to get wrong.
* `0x80022600` is unledgered; its `(r3 = this, r4 = &rec)` staging is literal and
  matches wave 128's use of the same helper.
* **Verdict: ready.** The gate, the five field writes, the two record writes and
  the effect call are all literal.

---

## 5. `0x80A1EBC0` - circle/offset position solve (`qr3`)

* **Extent** 520 bytes (130 words), 130 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80a1ebc0_FUN_80a1ebc0.c`.
* **Signature** `unsafe fn fn_80a1ebc0(this: usize, out: *mut [f32; 3])` -
  `r3` actor, `r4` the three-float destination.
* **Role** the position solver of the same class as wave 146 rank 9
  (`0x80A1E990`): given a target actor at `+0x71C`, produce a point on a circle
  of radius `+0x720` around it (mode A), on a two-term basis `+0x6F8/+0x6FC`
  (mode B), or an eight-unit offset from the actor's own yaw when `+0x716 != 0`.
  It shares the field map `+0x714` mode / `+0x71C` target / `+0x720` radius with
  that body (this body uses the `+0x726` offset; wave 146 rank 9 also reads
  `+0x724`).

### Behaviour

1. If `*(s16 *)(this+0x716) != 0` go to step 3.
2. `t = 0x80162D00(*(u32 *)(this+0x71C))`; if `t == 0` return.
   `tx = *(f32 *)(t+0xAC)`, `ty = *(f32 *)(t+0xB0)`, `tyaw = *(s16 *)(t+0x104)`.
   * **mode A** (`*(u8 *)(this+0x714) == 1`):
     `a = (s16)(tyaw + *(s16 *)(this+0x726))`;
     `c = cos((f32)(u16)a * 1/256)`, `s = sin((f32)(u16)a * 1/256)`;
     `out[0] = tx + *(f32 *)(this+0x720) * c`;
     `out[1] = ty + *(f32 *)(this+0x720) * s`;
     `out[2] = *(f32 *)(this+0xB4)`.
   * **mode B** (otherwise):
     `a = (s16)(tyaw + 0x4000)`;
     `c = cos(...)`, `s = sin(...)`;
     `k1 = *(f32 *)(this+0x6F8)`, `k2 = 8.0f + *(f32 *)(this+0x6FC)`;
     `out[0] = tx + k1*c - k2*s`;
     `out[1] = ty + k1*s + k2*c`;
     `out[2] = *(f32 *)(this+0xB4)`.
   * then `out[1] -= 16.0f`.
3. **self path** (`+0x716 != 0`): `a = *(s16 *)(this+0x104)`;
   `c = cos((f32)(u16)a * 1/256)`, `s = sin(...)`;
   `out[0] = *(f32 *)(this+0xAC) + 8.0f*c`;
   `out[1] = *(f32 *)(this+0xB0) + 8.0f*s`;
   `out[2] = *(f32 *)(this+0xB4)`.
4. Return.

### Constants

| address | value | use |
|---|---|---|
| `0x80B2B6DC` | `8.0f` | mode-B `k2` bias and self-path radius |
| `0x80B2B6E4` | `16.0f` | `out[1]` bias, both target modes |
| `0x80B2B6F0` | `0.00390625f` (1/256) | angle to radians |

`qr3` sites (all `psq_l f1,0(rX),qr3,1` on a stack copy of the `s16` angle):
`0x80A1EC0C`, `0x80A1EC38`, `0x80A1ECB4`, `0x80A1ECE0`, `0x80A1ED1C`,
`0x80A1ED38`.

### Port notes and verdict

* **`0x80237C40` is the cosine and `0x80237BD0` the sine** (`src/engine/display.rs`
  pins both). The body calls cosine first, then sine, so `out[0]` carries the
  cosine term and `out[1]` the sine term - do not swap them.
* `GQR3 = 0x00050005` with `W = 1` makes every `psq_l` here `(f32)(u16)` of the
  halfword; the following `fmuls` by `1/256` (at `0x80B2B6F0`) is the angle unit.
  Do not model the halfword as a paired float.
* The exported `CONCAT31/CONCAT21` trees and the `ldexpf` calls are the f31 spill
  plus the `2^52` integer-to-double idiom for the `(f32)(u16)` conversion.
* `+0x716` is an `s16` gate, not a byte; both reads are `lha`.
* **Verdict: ready.** `qr3` is pinned, the two `sin`/`cos` identities are
  established, and every offset and constant is literal.

---

## 6. `0x80A20900` - four-player proximity damage emitter

* **Extent** 520 bytes (130 words), 130 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80a20900_FUN_80a20900.c`.
* **Signature** `unsafe fn fn_80a20900(this: usize)` - `r3` only.
* **Role** per-frame loop over the four player slots: for every active player
  whose class test at vtable `+0xE0` returns zero and whose planar distance to
  this actor passes the
  `0x80A20810` radius test, emit effect `0x43B` at the actor position and then a
  second, single-parameter effect whose value is `400.0` while the actor sits in
  one of two states and the measured distance otherwise.

### Behaviour

1. `i = 0`, `bit = 1`; loop while `i < 4`:
   1. If `(bit & *(u8 *)0x80429CA8) == 0` skip to the next slot.
   2. `p = 0x8005F900(i)`; if `p == 0` skip.
   3. If `(*(u32 *)(p+0x60) -> vtable[0xE0])(p) != 0` skip.
   4. `rec = { f32 |p.x - this.x|, f32 |p.y - this.y|, f32 p.x, f32 p.y,
      f32 p.z }` at `r1+0x10..0x20` (`fabs` + `frsp` on the two deltas).
   5. `ang = 0x80A207E0(this, &rec)` (`s16`).
   6. If `0x80A20810(this, 400.0f, &rec, ang) == 0` skip.
   7. `d = |0x802BE470(rec[0]*rec[0] + rec[1]*rec[1])|` (`fabs` + `frsp`).
   8. `h = 0x80069530(i)`; `0x8006A4B0(&tmp, this+0xAC)`;
      `0x801995A0(*(u32 *)0x80429D60, 0x43B, *(u32 *)this, &tmp,
      (s16)(s32)d, h)`.
   9. `param = 400.0f` if `has_state(0x80352AF4) || has_state(0x80352B34)`,
      otherwise `param = d`;
      `0x80196EB0(*(u32 *)0x8042A488, (s16)(s32)param)`.
   10. `i++`, `bit <<= 1`.
2. Epilogue.

`has_state(tok)` is `st = (*(u32 *)(this+0x394) -> vtable[0x28])(this+0x394)`
followed by `(st -> vtable[0x14])(st, tok) != 0` - the same three-instruction
pattern as rank 3.

### Constants

| address | value | use |
|---|---|---|
| `0x80B2B768` | `400.0f` | `0x80A20810` radius and the state fallback |
| `0x43B` | literal `1083` | effect id |
| `0x80352AF4` / `0x80352B34` | `main.dol` state tokens | `400.0` selection |
| `0x80429CA8` | `u8` active-player mask | loop gate |
| `0x80429D60` | `u32` effect manager | `0x801995A0` argument 1 |
| `0x8042A488` | `u32` effect owner | `0x80196EB0` argument 1 |

### Port notes and verdict

* The mask test is `and.` on a byte loaded once per iteration and the shifted
  `1 << i`; the shift is `slw r0,r30,r0` with `r30` doubling each pass.
* `0x80A207E0` returns an `s16` angle and `0x80A20810` takes it as a **fourth**
  argument (`r5 = extsh(r3)`); both are already translated
  (`game::enemies::fn_80a207e0` / `fn_80a20810`).
* `0x801995A0` is the six-argument "spawn effect id on an actor" form
  (`src/game/enemies.rs` uses it with ids `0x2E5`/`0x2E0`); the `s16` fifth
  argument here is the distance in whole units.
* `0x80196EB0` is unledgered and undocumented; the staging is `r3 = *(u32 *)0x8042A488`,
  `r4 = (s16)(s32)param`. Kept as an address call, no contract asserted.
* The state branch is `if (in state A) 400.0 else if (in state B) 400.0 else d`;
  the two `neg; or; rlwinm.` tests are "token found".
* **Verdict: ready.** Effect ids, radius, mask and state tokens are literal; the
  five `bctrl` sites are all on `this+0x394` or on a player vtable.

---

## 7. `0x80B0D4D0` - turn-order verdict

* **Extent** 520 bytes (130 words), 130 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80b0d4d0_FUN_80b0d4d0.c`.
* **Signature** `unsafe fn fn_80b0d4d0(this: usize) -> u32` - returns `0` when
  the actor must yield, `1` when it may act.
* **Role** the "is it my turn?" test that wave 146 rank 3 calls as
  `if (fn_80b0d4d0(this) == 0) return 1;`. It scans the four player slots and
  compares each player's side against the actor's, using the player's
  `+0xD0` x-bias and two player flags. Two variants, selected by
  `*(u32 *)(this+0x9D0)`.

### Behaviour

1. `zero = 0.0f` (`0x80B31E34`).
2. **Variant A** (`*(u32 *)(this+0x9D0) == 0`): for `i = 0 .. 3`:
   1. `p = 0x8005F900(i)`; skip when `p == 0` or `(*(u8 *)0x80429CA8 & (1<<i)) == 0`.
   2. `pSide = *(u8 *)(p+0x348)`;
      `dx = *(f32 *)(this+0xAC) - (*(f32 *)(p+0xAC) + *(f32 *)(p+0xD0))`.
   3. `front = (*(u8 *)(this+0x348) == 0) ? (dx > 0.0f) : (dx < 0.0f)`.
   4. If `0x80056CF0(p, 0x35) != 0` then `pSide ^= 1`.
   5. If `front && 0x80056CF0(p, 0x33) == 0 && *(u8 *)(this+0x348) != pSide`
      then return `0`.
   6. Otherwise continue; after the loop return `1`.
3. **Variant B** (`*(u32 *)(this+0x9D0) != 0`): `n = 0`, `allDiff = 1`;
   for `i = 0 .. 3`:
   1. same player resolve, `dx`, `front` and effective side as A.2-A.4;
   2. if `front`: `n++`; if `*(u8 *)(this+0x348) == pSide` then `allDiff = 0`.
   3. after the loop: if `n != 0 && allDiff` return `0`, else return `1`.
4. Return the `0`/`1` in `r3` through `_restgpr_25`.

### Constants

| address | value | use |
|---|---|---|
| `0x80B31E34` | `0.0f` | front/back threshold |
| `0x80429CA8` | `u8` active-player mask | slot gate |
| `0x35` / `0x33` | literals 53 / 51 | player flag bits for `0x80056CF0` |
| `0x9D0` | `u32` | variant selector |

### Port notes and verdict

* The `front` test is **not** the export's `dVar10 < dVar11`. The raw branches
  are `bo = 4, bi = 1` (`bgt`-clear, i.e. `dx > 0` sets the flag) when the
  actor's side is `0`, and `bo = 4, bi = 0` (`blt`-clear, i.e. `dx < 0` sets it)
  otherwise. Take the raw direction, not the export's.
* `0x80056CF0(player, flag)` reads the word at `+0x1014 + 4*(flag>>5)` and
  returns bit `flag & 31`; it is already translated, so `0x35`/`0x33` stay as
  the literal flag numbers rather than as named player states.
* The return values are only `0` and `1` - both arms end in `li r3,0` / `li r3,1`.
  Wave 146 described this callee as returning "the 0/1/2 turn-order verdict"; the
  stream has two outcomes only.
* `r30` holds `0x80429CA8`'s address and `r29`/`r28` rotate between the two
  loops; the mask byte is re-loaded every iteration in variant A.
* **Verdict: ready.** No float argument, no table, no indirect call; the only
  helpers are two translated player functions.

---

## 8. `0x80B04A90` - chain-segment initialiser

* **Extent** 512 bytes (128 words), 128 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80b04a90_FUN_80b04a90.c`.
* **Signature** `unsafe fn fn_80b04a90(this: usize)` - `r3` only.
* **Role** lay out the `n+2` segments of the chain actor: the head at
  `this+0x540`, `n` segments at `this+0x8B8 + i*0x1B8` and the tail at
  `this+0x6FC`, all along -x with a 16-unit pitch, seeded from a 16-byte entry of
  the stage-object section table selected by the `+4` descriptor word.

### Behaviour

1. Save `f28..f31` (spills), `r23..r31`; `w = *(u32 *)(this+4)`.
2. `section = 0x800D90C0((w >> 4) & 0xF)`.
3. `area = *(u32 *)0x80429E98 + *(u8 *)(*(u32 *)0x8042A1C8 + 0x120E) * 0x3B0`;
   if `*(u32 *)(area+0x2C) == 0` set `area = 0`.
4. `n = *(u16 *)(section+2)`;
   `tbl = *(u32 *)(area+0x3C) + n*16 + ((w >> 8) & 0xFF0)`.
5. `a = *(s16 *)(tbl)`, `b = *(s16 *)(tbl+2)` (both sign-extended).
6. `count = *(u32 *)(this+0x2B18)`; `mode = (w >> 20) & 1`.
7. `x = 8.0f + (f32)a + 16.0f*(f32)count`; `y = -(f32)b - 8.0f`.
8. `0x80B04270(this+0x540, this, &{x, y, 1516.0f}, mode)`.
9. For `i = 0 .. count-1`:
   `0x80B04270(this + 0x8B8 + i*0x1B8, this, &{x, y, 1500.0f}, mode)`, then
   `x -= 16.0f`.
10. `0x80B04270(this+0x6FC, this, &{x, y, 1516.0f}, mode)` - with the `x` left by
    the loop, i.e. `x0 - 16*count`.
11. `*(u32 *)(this+0x6EC) = count + 1`;
    `*(u32 *)(this+0x8A8) = 1`;
    `k = (w >> 8) & 3`; `*(u32 *)(this+0x6F8) = k`; `*(u32 *)(this+0x8B4) = k`.

### Constants

| address | value | use |
|---|---|---|
| `0x80B31A60` | `8.0f` | x origin and y bias |
| `0x80B31A74` | `16.0f` | segment pitch |
| `0x80B31A98` | `1516.0f` | head/tail z |
| `0x80B31A9C` | `1500.0f` | body z |
| `0x80B31AA0` | `2^52 + 2^31` | `(f32)(s32)` conversion bias |
| `0x1B8` | stride 440 | segment objects |
| `0x3B0` | stride 944 | area blocks from `0x80429E98` |

### Port notes and verdict

* The two `s16` reads are `lhaux`/`lha`: the first updates `r9` by the byte
  offset, so the second reads `tbl+off+2`. Model it as
  `let e = tbl.add(off); (e[0] as i16, e[1] as i16)`.
* `((w >> 8) & 0xFF0)` comes from `rlwinm r3,r8,24,20,27`; the shift-and-rotate
  pair reduces to `(w >> 8) & 0xFF0` because the mask cannot see the rotated-in
  low byte.
* The `2^52 + 2^31` bias at `0x80B31AA0` is the **signed** 32-bit-to-double
  idiom: it converts the sign-extended `s16` (and the `+0x2B18` count) correctly
  for negative values. Do not replace it with a plain `as f32` on the raw word.
* `*(u32 *)(area+0x2C) == 0` makes `area` null and the following
  `lwz r6,60(r6)` would read address `0x3C`. That is what the stream says (the
  guard is dead in practice because `+0x2C` is always non-zero); a port should
  keep the guard and test for the null before the table read rather than
  reproduce the fault.
* `0x80B04270` is translated: it copies the position into the segment's
  `+0xA4..0xAC`, latches the target at `+0x1B0`, calls `0x8007FA40` with four
  floats and eight integers, sets `+0x198 = 4` and calls `0x8007F900`.
* `0x800D90C0` is `stage_object_section_entry` (`stage+0x38` list, count
  `+0xA8`, 8-byte stride, first-byte match); the section record's `+2` count and
  `+0x3C` table pointer are the layout this body asserts.
* **Verdict: ready.** Every offset, stride, float and shift is literal; the only
  runtime inputs are the stage-object section record and the area block, both
  reached through translated helpers/globals.

---

## 9. `0x80B16CE0` - `unizo` / `unizoBig` binder

* **Extent** 504 bytes (126 words), 126 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80b16ce0_FUN_80b16ce0.c`.
* **Signature** `unsafe fn fn_80b16ce0(this: usize)` - `r3` only.
* **Role** the shared model/animation binder of the `unizo` actor, in the same
  shape as wave 146 rank 1 (`I_kinoko`): it loads `g3d/unizo.brres` or
  `g3d/unizoBig.brres` from the `+0x628` variant index, binds the base clip with
  kind `0x23`/`0x20`, builds the animation holder, and for variant 0 also builds
  a texture-pattern holder driven by the `wait` animation. It is the callee of
  `0x80B16060` (wave 146 rank 8) at `0x80B160DC`.

### Behaviour

1. `0x800690C0(this+0x52C, -1, *(u32 *)0x80377C48, 0, 0x20)` - 32-entry holder.
2. `v = *(u32 *)(this+0x628)` (0 or 1).
3. `res = 0x800DF180(*(u32 *)0x8042A038 + 4, names[v], paths[v])`;
   `*(u32 *)(this+0x548) = res`.
4. `clip = 0x80239E30(this+0x548, clips[v])`.
5. `0x80169D90(this+0x54C, &clip, this+0x52C, kinds[v], 1, 0)`.
6. `0x80064BD0(this+0x54C)`.
7. If `v == 1`: `0x8016A000(this+0x54C, this+0x524)`.
8. `anm = 0x8023A0B0(this+0x548, anims[v])`.
9. `0x801650D0(this+0x58C, &clip, &anm, this+0x52C, 0)`.
10. `0x801651F0(this+0x58C, this+0x54C, &anm, 0)`.
11. `0x80169F80(1.0f, this+0x54C, this+0x58C)`.
12. If `v == 0`:
    * `pat = 0x8023A200(this+0x548, texpat[0])`;
    * `0x80166830(this+0x5C4, &clip, &pat, this+0x52C, 0, 1)`;
    * `0x80166BD0(this+0x5C4, this+0x54C, &pat, 0, 0)`;
    * `(*(u32 *)(this+0x54C) -> vtable[0x18])(this+0x54C, this+0x5C4)`;
    * `0x80166EF0(1.0f, this+0x5C4, 0)`; `0x80166F10(1.0f, this+0x5C4, 0)`.
13. `0x800690E0(this+0x52C)`; return.

### Constants

| address | value | use |
|---|---|---|
| `0x80B69E90` | `{"unizo", "unizoBig"}` | resource-name table |
| `0x80B69EC0` | `{"g3d/unizo.brres", "g3d/unizoBig.brres"}` | model-path table |
| `0x80B69EC8` | `{"unizo", "unizoBig"}` | base-clip table |
| `0x80B69ED8` | `{"wait", "wait"}` | animation table |
| `0x80B69EE0` | `{"wait"}` | texture-pattern table (variant 0 only) |
| `0x80B322E8` | `u32[2] {0x23, 0x20}` | clip bind kinds |
| `0x80B322B4` | `1.0f` | animation rate |
| `0x80B69E44` | `1.0f` | second rate |
| `0x80377C48` / `0x8042A038` | `main.dol` | holder name and archive global |

### Port notes and verdict

* The five tables are four-byte pointer arrays indexed by the **word**
  `*(u32 *)(this+0x628)` (`rlwinm r0,r0,2,0,29`), so the variant is a word, not a
  byte, and only `0`/`1` are in range.
* The bind kind differs per variant: `0x23` for `unizo`, `0x20` for `unizoBig`
  (table `0x80B322E8`), which is the same pair wave 146 rank 1 used.
* `0x80064BD0` is the model "finish" call, not the visibility setter
  (`0x80064BF0`/`0x80064C10`): `docs/COVERAGE_WAVE30_BASES.md:130` records the
  same substitution.
* The `bctrl` at `0x80B16E94` is the model holder's own `+0x18` slot with
  `r4 = this+0x5C4`; both operands are objects this body just filled.
* **Verdict: ready.** Strings, tables, kinds, rates and the two variant branches
  are all literal in this REL.

---

## 10. `0x80A06560` - four-slot attachment placement (`qr3`)

* **Extent** 500 bytes (125 words), 125 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80a06560_FUN_80a06560.c`.
* **Signature** `unsafe fn fn_80a06560(this: usize)` - `r3` only.
* **Role** place up to four attached actors (`this + 0x738 + i*4` holds their
  registry ids) relative to this actor: either on a radius-16 circle at
  `+0x115E + table[target+0x5C0]` with a rate/limit smoothing, or from the
  `+0x74C` record table entry `target+0x5C0 + 1` (position and three angles).
  It then copies this actor's angles onto the attachment, with `+0x102`
  overridden in the record mode.

### Behaviour

1. For `i = 0 .. 3`:
   1. `id = *(u32 *)(this + 0x738 + i*4)`; skip when `id == 0`.
   2. `t = 0x80162D00(id)`; skip when `t == 0` or `*(u8 *)(t+0x5BC) == 0`.
   3. If `*(u8 *)(this+0x1162) == 0` (**circle mode**):
      1. `a = (s16)(*(s16 *)(this+0x115E) +
         *(s16 *)(0x80B2AA90 + *(u8 *)(t+0x5C0)*2))`.
      2. `rx = *(f32 *)(this+0xAC) + 16.0f*cos((f32)(u16)a * 1/256)`;
         `ry = *(f32 *)(this+0xB0) + 16.0f*sin((f32)(u16)a * 1/256)`.
      3. `vx = *(f32 *)(t+0xAC)`, `vy = *(f32 *)(t+0xB0)`;
         `fn_8015F140(rx, 0.125, *(f32 *)(this+0x1154), 0.125, &vx)`;
         `fn_8015F140(ry, 0.125, *(f32 *)(this+0x1158), 0.125, &vy)`.
      4. `0x80A65CE0(t, &{vx, vy, *(f32 *)(this+0xB4)})`.
      5. `*(s16 *)(t+0x100) = *(s16 *)(this+0x100)`;
         `*(s16 *)(t+0x102) = *(s16 *)(this+0x102)`;
         `*(s16 *)(t+0x104) = *(s16 *)(this+0x104)`.
   4. Else (**record mode**):
      1. `0x80A06130(&out, this+0x74C, (u8)(*(u8 *)(t+0x5C0) + 1))` - `out` is
         three floats at `r1+0x2C..0x34` and three `s16` at `r1+0x38..0x3C`.
      2. `0x80A65CE0(t, &{out.f0, out.f1, out.f2})`.
      3. `*(s16 *)(t+0x100) = out.s0`; `*(s16 *)(t+0x102) = out.s1`;
         `*(s16 *)(t+0x104) = out.s2`;
         then `*(s16 *)(t+0x102) = *(s16 *)(this+0x102)`.
2. Return.

### Constants

| address | value | use |
|---|---|---|
| `0x80B2AA90` | `s16[4] {0x2000, -0x2000, -0x6000, 0x6000}` | per-slot yaw offset |
| `0x80B2AA98` | `16.0f` | circle radius |
| `0x80B2AA9C` | `0.00390625f` (1/256) | angle to radians |
| `0x80B2AAA0` | `0.125f` | `fn_8015F140` rate and dead-band |

`qr3` sites: `0x80A06660` and `0x80A06680` - both read the same stack copy of
the angle, once for the cosine and once for the sine.

### Port notes and verdict

* The export's two branches are **inverted**: the raw `bc 12,2` at `0x80A065E4`
  takes the `0x80A06644` block when `*(u8 *)(this+0x1162) == 0`, so the
  "circle + smoothing" block is the `== 0` case and the record block is the
  non-zero case. Keep the raw direction.
* `fn_8015F140(target, rate, limit, deadband, *mut f32)` **steps the value it is
  given**: the stack slot is first seeded with the target's own coordinate and
  then stepped toward the circle point, which makes the attachment lag behind by
  the rate. Seeding it with the circle point would make the whole step a no-op.
* The `+0x1154`/`+0x1158` limits and `0.125` rate/dead-band are runtime floats in
  the actor, not REL constants; `+0x1162` is the mode byte.
* The angle table index is `*(u8 *)(t+0x5C0)`, i.e. 0..3 for the four-entry table;
  in the record mode the same byte `+1` indexes `0x80A06130`'s `0x14`-byte
  records.
* `0x80A65CE0` overwrites the z it is handed: it stores two words and computes
  `z = pos.z - 10.0*(+0x5C0+1)` on the target.
* **Verdict: ready.** `qr3` pinned, `sin`/`cos` identities established, both
  helpers translated; the only non-literal inputs are runtime actor floats.

---

## 11. `0x80B115A0` - yaw-pitch-yaw spawn matrix

* **Extent** 480 bytes (120 words), 120 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80b115a0_FUN_80b115a0.c`.
* **Signature** `unsafe fn fn_80b115a0(this: usize) -> u32` - returns the flag
  from `0x800B8700`.
* **Role** build this actor's spawn matrix (translate to the actor, apply the
  side-relative yaw / pitch / roll, lift by ten units, drop the result by eleven)
  and hand the resulting point plus a 1.15 scale triple to the render-item
  builder at `this+0x484`.

### Behaviour

1. `mtx = translate(*(f32 *)(this+0xAC), *(f32 *)(this+0xB0), *(f32 *)(this+0xB4))`
   at `r1+0xA8`.
2. `rotY(*(s16 *)(this+0x102))`.
3. `rotY((s16)(table[*(u8 *)(this+0x348)] - *(s16 *)(this+0x102)))` with
   `table = 0x80B31FF0`.
4. `rotX(*(s16 *)(this+0x100))`.
5. `rotY((s16)(*(s16 *)(this+0x102) - table[*(u8 *)(this+0x348)]))`.
6. `rotZ(*(s16 *)(this+0x104))`.
7. `concat(translate(0.0f, 10.0f, 0.0f))`.
8. `0x8016F130(mtx, &v)`; `v.y -= 11.0f`.
9. Build the 60-byte descriptor at `r1+0x68`:
   `{u32 0; f32 v.x; f32 v.y; f32 v.z; f32 1.15; f32 1.15; f32 1.15;
   f32 0.0 x 8}`.
10. `ret = 0x800B8700(this+0x484, &desc, 1)`.
11. `0x802DCA98(&desc, 0x80A0B5A0, 0x3C, 1)` - compiler array-destroy of the
    single 60-byte temporary.
12. Return `ret`.

### Constants

| address | value | use |
|---|---|---|
| `0x80B31FF0` | `s16[2] {0x4000, -0x4000}` | per-side yaw offset |
| `0x80B32008` | `0.0f` | translate x/z and the eight zeros |
| `0x80B32048` | `10.0f` | upward lift |
| `0x80B3204C` | `11.0f` | final y drop |
| `0x80B32050` | `1.149999976158142f` | scale triple |
| `0x3C` | `60` | temporary stride/size |

### Port notes and verdict

* The three rotations are **in-place concats**, so the order matters. The net
  orientation is `Ry(table[side]) * Rx(pitch) * Ry(yaw - table[side])`: the two
  extra yaw terms cancel against each other and put the pitch between them.
  Write the six calls in the same order rather than folding them.
* `rotX`/`rotY`/`rotZ` are `0x8016ECB0` / `0x8016EDD0` / `0x8016EEF0`; each
  builds a 0x3C-byte rotation packet from an `s16` and concatenates it into the
  matrix. The ledger's "format and submit one message id" prose for these
  addresses is a mislabel of the packet builder - the operands are `(mtx, &s16)`.
* `0x8016F130` copies the translation column (`mtx+0xC/+0x1C/+0x2C`), so the
  `-11.0` applies to the lifted (`+10.0`) y - the net y bias is `-1.0`.
* `0x802DCA98(base, dtor, stride, count)` destroys from the end; with
  `count = 1` it destroys exactly `&desc` through `0x80A0B5A0` (ledgered as
  `fn_80a0b5a0`, "free helper without the null guard"). Keep the call: it is the
  C++ temporary's array-destructor, and the object it destroys owns nothing here.
* The descriptor's first word is `0` and its layout is exactly 15 words, so the
  `0x3C` stride and the 60-byte temporary agree.
* **Verdict: ready.** All six rotation operands, the three vectors and the scale
  triple are literal; the two callees are translated.

---

## 12. `0x80A56920` - firebar phase advance and segment spiral (`qr3`)

* **Extent** 476 bytes (119 words), 119 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80a56920_FUN_80a56920.c`.
* **Signature** `unsafe fn fn_80a56920(this: usize)` - `r3` only.
* **Role** advance the firebar phase and rewrite the `+0x1EF0/+0x1EF4` offset
  table that rank 1 consumes: segment `i` sits at radius `12*(i+1)` on the
  current phase, and the mirrored half (when `+0x2486 == 1`) is the same sweep
  half a turn later. It also clears the per-slot `+0x5E8` word and copies each
  offset pair into the `+0x56C/+0x570` slot records with the `8.0` y bias.

### Behaviour

1. `*(s16 *)(this+0x2480) = (s16)(*(s16 *)(this+0x2480) + *(s16 *)(this+0x2482))`.
2. `n = *(u16 *)(this+0x2484)`; if `n <= 0` skip to the epilogue.
   `r = 12.0f`; for `i = 0 .. n-1`, with `off = this + i*0xC` and
   `slot = this + i*0xA4`:
   1. `*(u16 *)(slot+0x5E8) = 0`.
   2. `a = *(s16 *)(this+0x2480)`;
      `*(f32 *)(off+0x1EF0) = r * cos((f32)(u16)a * 1/256)`;
      `*(f32 *)(off+0x1EF4) = r * sin((f32)(u16)a * 1/256)`.
   3. `*(f32 *)(slot+0x56C) = *(f32 *)(off+0x1EF0)`;
      `*(f32 *)(slot+0x570) = 8.0f + *(f32 *)(off+0x1EF4)`.
   4. If `*(u16 *)(this+0x2486) == 1`, with `idx = i + n`,
      `off2 = this + idx*0xC`, `slot2 = this + idx*0xA4`:
      1. `*(u16 *)(slot2+0x5E8) = 0`.
      2. `a2 = (s16)(*(s16 *)(this+0x2480) - 0x8000)`;
         `*(f32 *)(off2+0x1EF0) = r * cos((f32)(u16)a2 * 1/256)`;
         `*(f32 *)(off2+0x1EF4) = r * sin((f32)(u16)a2 * 1/256)`.
      3. `*(f32 *)(slot2+0x56C) = *(f32 *)(off2+0x1EF0)`;
         `*(f32 *)(slot2+0x570) = 8.0f + *(f32 *)(off2+0x1EF4)`.
   5. `r += 12.0f`.
3. Epilogue.

### Constants

| address | value | use |
|---|---|---|
| `0x80B2CEF8` | `12.0f` | radius increment and `0x1EF0/0x1EF4` scale |
| `0x80B2CEFC` | `0.00390625f` (1/256) | angle to radians |
| `0x80B2CEF0` | `8.0f` | `+0x570` y bias |
| `0x8000` | literal | mirror half-turn |

`qr3` sites: `0x80A569BC` (cosine), `0x80A569D8` (sine),
`0x80A56A30` (cosine, mirror), `0x80A56A58` (sine, mirror).

### Port notes and verdict

* The **radius grows per index but the angle does not**: every segment uses the
  same `+0x2480` phase and gets `12*(i+1)`. This is the table rank 1 reads, so the
  two bodies must agree on the `12.0` and the `8.0` bias.
* The mirror angle is `(s16)(phase - 0x8000)`; the raw does
  `addis r3,r3,1` / `addi r0,r3,-32768`, i.e. an 18-bit intermediate folded back
  to `s16`. Model it as a wrapping `i16` subtraction of `0x8000`.
* Only `+0x5E8` (record word 0) is cleared here; the side byte at `+0x5EF` that
  wave 146 rank 2 writes is `+0x5E8 + 7` in the same `0xA4`-stride record.
* The four `psq_l` sites read stack copies of the `s16`; with
  `GQR3 = 0x00050005` each is `(f32)(u16)`, then `* 1/256`. The cosine call is
  `0x80237C40` and the sine `0x80237BD0`.
* The export's `in_f28..in_f31` / `unaff_GQR0` prologue is four f31..f28 spills
  restored at `0x80A56AC8..0x80A56AE4`.
* **Verdict: ready.** `qr3` pinned, all offsets and strides literal, no calls
  other than `sin`/`cos` and the register-save pair.

---

## Cross-checks between the twelve

* **Spill vs quantiser.** Eleven of the twelve begin with `stfd fN` +
  `psq_st fN,...,qr0,0` pairs; all are `f31..f28` saves restored in the epilogue.
  Only ranks 5, 10 and 12 contain a data `qr3` site (twelve in total, quoted
  above). No body contains `mtspr GQRn`.
* **Two firebar bodies, one table.** Ranks 1 and 12 are the writer/reader pair of
  `+0x1EF0/+0x1EF4` (stride `0xC`) and `+0x5E8`/`+0x56C` (stride `0xA4`); rank 12
  writes the offsets and the `+0x56C` slot copies, rank 1 reads them and adds the
  `8.0` y bias. They share `+0x2484` (count) and `+0x2486` (mirror).
* **Two `+0x714/+0x71C` bodies.** Ranks 5 and 10 belong to the target-following
  class of wave 146 rank 9 (`0x80A1E990`): `+0x714` mode, `+0x71C` target id,
  `+0x720` radius, `+0x724`/`+0x726` offsets. Rank 5 writes a position into a
  caller buffer; rank 10 writes it onto the attachment actor.
* **Two state-token access patterns.** Ranks 3, 6 and 7 all reach the state
  manager through `(*(u32 *)(this+0x394) -> vtable[0x28])(this+0x394)` and then
  `->vtable[0x14](state, token)`; the `neg; or; rlwinm. 1,31,31` test is
  "token != 0" and must not be read as a signed comparison.
* **Three `+0x60` vtable users.** Ranks 2, 4 and 6 all dispatch the actor class
  vtable at `+0x60`; rank 2 uses
  `+0x2C8/+0x2CC/+0x2E8/+0x2EC/+0x2F0/+0x300/+0xD4`, rank 4 uses only `+0xD4`,
  and rank 6 uses `+0xE0` (the per-player "special" test) plus `+0xD4`.
* **Two `unizo` binders.** Rank 9 (this wave) is the shared binder that
  `0x80B16060` (wave 146 rank 8) calls to build the `unizoBig` variant; the
  `+0x628` word is the variant index for both.
* **Two record writers, two shapes.** Ranks 3 and 4 build the same 24-byte actor
  record (`{f32 x, y, z; u8 +0x694; u32 +0x854; u8 0; u8 flag; u8 link}`) for
  `0x80022600` and then submit an effect record through `0x800D0470`; rank 6
  builds a different 20-byte `{f32 |dx|, |dy|, f32 px, py, pz}` for
  `0x80A207E0`/`0x80A20810`. The `+0x694` id byte and the `+0x854` word appear in
  both ranks 3 and 4, and the `+0x71C`/`+0x718` pair in all three.

## Verification

* `functions.tsv` row count 3,555; ledger rows 16,910; distinct heading addresses
  1,448; distinct `fn_`/`FN_` tokens 16,027; remaining bodies 133, all with an
  export.
* Every one of the twelve decodes end-to-end from its entry with the
  `file_offset = address - 0x80A00000 + 0x1000` mapping; the walk produces no
  out-of-extent target, no zero word and no unknown opcode.
* Reachable/extent ratio is `1.00` for all twelve, and the terminal instruction
  is at `entry + size - 4` for all twelve (`bclr` in every case). The extent is
  therefore a true body length here, unlike the padded rows of wave 146.
* Every `psq_l`/`psq_st` in the twelve was decoded with opcode 56/60 and its
  quantiser field read: `qr0` everywhere except the twelve `qr3` data sites
  listed in the `qr3` section.
* `bctrl` count per rank: 9, 9, 8, 1, 0, 5, 0, 0, 1, 0, 0, 0. Each was checked to
  be `0x4E800421` (`LK = 1`) and each falls through to a live instruction; none is
  a tail `bcctr`.
* Every conditional branch in the twelve was resolved to a target inside its
  extent. The six loop headers (`+0x2484` in ranks 1 and 12, the four-slot scans
  in ranks 6 and 7, the segment loop in rank 8, the id table in rank 10) were
  walked from both the entry and the skip edge; the `n <= 0` / `i < 4` guards are
  the `bc 4,1` / `bc 12,0` pairs quoted in the text.
* All float and halfword constants quoted above were read from
  `work/rels/elf/d_enemiesNP.elf`; the BSS tokens (`0x80B76DA8`) and the
  `main.dol` tokens (`0x80352A34`, `0x80352A74`, `0x80352AF4`, `0x80352B34`,
  `0x80352CB4`) return no file bytes, which is why they are listed as tokens
  rather than values.
* No file in the repository was modified by this lane.
