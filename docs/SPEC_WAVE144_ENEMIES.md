# Wave 144 - implementation specification for 12 untranslated `d_enemiesNP` bodies

## What this page is

`docs/ANALYSIS_WAVE127_ENEMIES.md` reclassified 384 formerly-skipped `d_enemiesNP`
addresses into 173 class **B**, 160 class **C** and 51 class **D**. Waves 128-143
(`docs/SPEC_WAVE128_ENEMIES.md` .. `docs/SPEC_WAVE143_ENEMIES.md`) specified that
whole block; wave 143 left exactly **one** class-D `qr3` row, `0x80AE2250`
(108 B), and the 4-byte alias `0x80A337B0`.

A live re-walk of `work/ghidra/rel-d_enemiesNP/export/functions.tsv` against the
`address` column of `ledger/functions.csv` (16,815 rows, read-only), against
every `0x80Axxxxx`/`0x80Bxxxxx` address that appears in a `^#` heading of any
`docs/*.md` (279 distinct addresses), and against every `fn_<addr>` token in
`src/**/*.rs` (3,270 distinct addresses) leaves **202** untranslated bodies.
Twenty of them still contain a `psq_l ...,qr3` load; the class-D tranche,
however, is drained to its last row. This wave therefore takes

1. `0x80AE2250` - the last class-D `qr3` row, and
2. the eleven largest other untranslated bodies that have a Ghidra export,
   by descending `functions.tsv` size.

This page is **read-only**: it writes no Rust, no ledger row and no other doc.

## Selection

Every row below is absent from the ledger, from every doc heading and from
`src/`. `functions.tsv` size is a bucket extent; for all twelve rows the body
reachable from the entry fills the extent exactly (word count = `size / 4`), so
no row borrows a neighbour's instructions. The only exception in the walk is
`0x80A3DEA0`, whose 16-case jump table is walked once read out of the image
(see rank 1); with those 16 seeds the union is 735/735 words, no gaps.

| # | address | TSV size | reachable insns | role | verdict |
|---:|---|---:|---:|---|---|
| 1 | `0x80A3DEA0` | 2940 | 735 | actor constructor: param-nibble unpack, ground bind, 16-state dispatch | **ready** |
| 2 | `0x80B04E90` | 1592 | 398 | stage-section polyline analyser: run-length direction buffer into `+0x2B1C` | **ready** |
| 3 | `0x80A7BEC0` | 1464 | 366 | `walk_*` clip-sequence player with contact-driven turn and landing smoke | **ready** |
| 4 | `0x80A9B3C0` | 1196 | 299 | per-slot (4-player) shot/effect record builder with neighbour fallback | **conditional** |
| 5 | `0x80B147D0` | 1184 | 296 | `daEnTogeTekkyu_c` 4-variant initialiser: collision descriptor + 11-word animation block | **ready** |
| 6 | `0x80A92F40` | 1172 | 293 | player-anchored chaser step: scale ramp, gravity clamp, facing quartet | **conditional** |
| 7 | `0x80AAE520` | 1156 | 289 | 4-variant segmented actor constructor: 15 collision segments + colours | **ready** |
| 8 | `0x80B0C900` | 1140 | 285 | `daEnTeresa_c` model binder: `teresa.brres`, A/B/fog animation sets | **ready** |
| 9 | `0x80A673D0` | 1080 | 270 | goal actor model binder: `goal_set.brres`, base/toride/flag/fog clip sets | **ready** |
| 10 | `0x80A85650` | 976 | 244 | one-shot shot resolver: per-player state table, SE 0x116 | **ready** |
| 11 | `0x80AA0C20` | 872 | 218 | actor step: approach latch, 470/467 SE fan-out, clip selection | **conditional** |
| 12 | `0x80AE2250` | 108 | 27 | `qr3` wing step: `y = base + 1.5*sin_angle16(angle/256)` | **ready** |

**Split: 9 ready, 3 conditional.** Total 14,880 bytes of TSV extent and 3,720
reachable instructions.

The three conditional rows are the only ones that need an external fact; each
has its blocker named in its **Verdict** line. Nothing else is missing: all
direct calls have their argument registers staged in the stream, all constant
blocks live in the REL image except where marked **(ext)**, and no row takes an
incoming float argument.

Three rows that a size ranking would place next are deliberately passed over:

* `0x80A96DC0` (848 B) and `0x80A969C0` (844 B) follow rank 11 by size; they are
  left for the next tranche.
* `0x80A337B0` is still the 4-byte `b 0x80A337C0` alias with no body of its own
  (`docs/SPEC_WAVE143_ENEMIES.md:11-16`).

## The `qr3` pool at this wave

`docs/REL_ELF_MAPPING.md` pins **`GQR3 = 0x00050005`** (unsigned 16-bit, scale
0), installed by `main.dol` at `0x8025EEEC`/`0x8025EEF0`. The RAW stream in this
REL holds no `mtspr GQRn`, so every `psq_*` in this module depends on a `main.dol`
word. With that value, every data `psq` in this module has the shape

```
psq_l fD,0(rA),1,qr3      ; W = 1 (single element), I = 3
```

which is one unsigned 16-bit load scaled by `2^0`:

```
fD = (f32)*(u16 *)rA
```

`0x80AE2250` (rank 12) is the last class-D row with such a site. The scan of
the 202-address remainder found **19 more** `qr3` bodies that were never part of
the wave-127 class-D block (they were recorded as GQR-damaged skips by waves
128-143). They are not claimed here because they are smaller than the eleven
non-`qr3` rows above; they are the natural `qr3` continuation of wave 145:

| address | TSV size | `qr3` sites | relation recorded in the docs |
|---|---:|---:|---|
| `0x80A87030` | 732 | 1 | calls `fn_80a85650` (rank 10 of this wave) |
| `0x80A1E990` | 560 | 5 | aim/placement family (`COVERAGE_WAVE128_ENEMIES.md:136`) |
| `0x80B16F40` | 560 | 2 | `0x80B1....` block (`COVERAGE_WAVE141_ENEMIES.md:193`) |
| `0x80A1EBC0` | 520 | 6 | same family as `0x80A1E990` |
| `0x80A06560` | 500 | 2 | `0x80A03C90` family |
| `0x80A56920` | 476 | 4 | firebar emitter (`COVERAGE_WAVE133_ENEMIES.md`) |
| `0x80B27ED0` | 432 | 2 | `+0x6E8` slot relay |
| `0x80A94D10` | 404 | 1 | none recorded |
| `0x80AC0840` | 396 | 1 | `GQR3` consumer (`COVERAGE_WAVE134_ENEMIES.md:142`) |
| `0x80A88BC0` | 368 | 2 | `0x80A87030` twin |
| `0x80A58D70` | 308 | 1 | jump arc with the `0x8000` half-turn |
| `0x80AF3400` | 308 | 2 | `0x80AF2CF0` family |
| `0x80A80F40` | 264 | 1 | `0x80A80E50` family |
| `0x80A584E0` | 232 | 2 | angle writer |
| `0x80A80E50` | 220 | 1 | `0x80A80F40` family |
| `0x80A03C90` | 212 | 3 | `0x80A06560` family |
| `0x80A5DBC0` | 212 | 1 | firebar block |
| `0x80AF3570` | 196 | 2 | `0x80AF3400` twin |
| `0x80B22750` | 128 | 1 | GQR quantiser row |

All nineteen decode end-to-end from the entry (no `bctr`, no bad word, no
`extraout_`), so once claimed they are ready bodies under the same GQR3 rule.

## Reading conventions

* Synthetic base is `0x80A00000`. ELF file offset is
  `(address - 0x80A00000) + 0x1000` for the first `PT_LOAD`
  (`p_offset = 0x10F0`, `p_vaddr = 0x80A000F0`); the other segments use their
  own `p_offset`/`p_vaddr` (`docs/REL_ELF_MAPPING.md`).
* The REL file-backed image ends at `0x80B6E438`; anything above it is the
  zero-filled `PT_LOAD` (`0x80B6E500`, `memsz = 0xB4E0`) or `main.dol`, and is
  marked **(ext)**. Name strings that live inside the image are quoted.
* `this` is the incoming `r3` unless stated otherwise; `+0xNNN` are byte offsets
  from `this`. `+0xAC`/`+0xB0`/`+0xB4` are x/y/z (the module's actor
  convention); `+0xDC`/`+0xE0`/`+0xE4` are the three "colour/scale" floats.
* `addi rX,r0,imm` is `li`, `addis rX,r0,imm` is `lis`, `or rX,rY,rY` is `mr`.
  `rlwinm rA,rS,sh,mb,me` rotates `rS` left by `sh` and masks `mb..me`
  (wrapping); `slwi`/`srwi` are the usual aliases, and `rlwinm rA,rS,0,28,31`
  is "low nibble".
* On a `bc`, `bo = 12` branches on a true CR bit and `bo = 4` on a false one;
  `bi & 3` is `LT/GT/EQ/SO`. `cror 2,1,2` (`0x4C411382`, XO 449) after an
  `fcmpo` rewrites `cr0.eq` as `gt|eq`, so the following `bne` reads
  `if !(a >= b)`; `cror 2,0,2` gives `lt|eq`.
* The paired-single opcodes in this build are `psq_l = 56` and `psq_st = 60`;
  quantiser is `(w >> 12) & 7`, `W` is `(w >> 15) & 1`, displacement is the
  signed 12-bit field. `qr0` sites in the bodies below are `f28..f31` stack
  spills (`docs/REL_ELF_MAPPING.md:53-55`).
* A-form float ops: `fadds/fsubs` take the second source from bits 16-20,
  `fmuls/fdivs` from bits 21-25, and `fmadds/fmsubs` print `fD,fA,fC,fB`.
* `fctiwz` + `stfd` + `lwz` is the module's f32-to-int conversion; the
  `0x43300000`/`2^52` (`4503599627370496`) idiom is the unsigned counterpart
  used for float-from-u32.
* `sth`/`lha` pairs store angle words; an angle is in 16-bit turn units and the
  pre-scale to the `0x80237BD0`/`0x80237C40` helpers is `1/256`.

## Symbolic helpers used below

Roles are the ledger's (`ledger/functions.csv`) or the role established by the
cited earlier wave.

| address | name / shape | role |
|---|---|---|
| `0x80237BD0` | `sin_angle16(f1)` | ledgered `resource::bres`; angle in 16-bit turn units, pre-scaled by `1/256` |
| `0x80237C40` | `cos_angle16(f1)` | ledgered `resource::bres`; same convention |
| `0x802DCD54` | `_savegpr_22` | ledgered `runtime::cpp`; saves/restores `r22..r31`, does not touch `r3..r10` |
| `0x802DCD5C` | `_savegpr_23` | ledgered `runtime::cpp` |
| `0x802DCD64` | `_savegpr_24` | ledgered `runtime::cpp` |
| `0x802DCD6C` | `_savegpr_25` | ledgered `runtime::cpp` |
| `0x802DCD70` | `_savegpr_26` | ledgered `runtime::cpp` |
| `0x802DCD74` | `_savegpr_27` | ledgered `runtime::cpp` |
| `0x802DCDA0` | `_restgpr_22` | ledgered `runtime::cpp` |
| `0x802DCDA8` | `_restgpr_23` | ledgered `runtime::cpp` |
| `0x802DCDB0` | `_restgpr_24` | ledgered `runtime::cpp` |
| `0x802DCDB8` | `_restgpr_25` | ledgered `runtime::cpp` |
| `0x802DCDBC` | `_restgpr_26` | ledgered `runtime::cpp` |
| `0x802DCDC0` | `_restgpr_27` | ledgered `runtime::cpp` |
| `0x80021690` | `actor_physics_step(this)` | unledgered `main.dol`; the translated bodies call it by address |
| `0x800272F0` | `actor_setup_2F0(this)` | unledgered `main.dol` |
| `0x80027310` | `actor_set_scale_mode(this, mode)` | unledgered `main.dol` |
| `0x800277A0` | `actor_mode_set(this, flag)` | unledgered `main.dol` |
| `0x80064680` | `actor_query_680(kind, this, arg, pos, flag, dir)` | unledgered `main.dol` |
| `0x80077810` | `stage_ground_probe(manager, x, y, layer)` | unledgered `main.dol`; returns the ground hit count |
| `0x80078520` | `stage_handle(...)` | ledgered `game::stage` |
| `0x80097700` | `fn_80097700(this, pos) -> u32` | ledgered `game::stage` |
| `0x80161800`/`0x80161750` | `ground_link_remove`/`ground_link_add` | ledgered `resource::registry` |
| `0x801625B0`/`0x801625D0` | `link_first(this)`/`link_next(rec)` | ledgered (`0x801625D0` unledgered) |
| `0x8064F990` | `actor_world_pos(dst, rec)` | unledgered `main.dol`; writes a 3-float vector |
| `0x80096240` | `fn_80096240(this, pos) -> u32` | ledgered `game::stage`; facing side (0..3) |
| `0x8008C330`/`0x8008C380` | `collision_area_register`/`collision_area_unregister` | ledgered `game::stage` |
| `0x8008C3E0` | `collision_area_set_descriptor` | ledgered `game::stage` |
| `0x8006D090` | `collision_slot_bind(dst, owner, a, b, c)` | unledgered `main.dol` |
| `0x800955F0` | `fn_800955f0(this)` | ledgered `game::stage`; orbit/position follow |
| `0x80A9FD50` | `clip_play(this, index)` | ledgered `game::enemies` |
| `0x80A9FF20` | `contact_check(this)` | ledgered `game::enemies` |
| `0x80AA0340` | `target_resolve(this)` | ledgered `game::enemies` |
| `0x80A863D0` | `fn_80a863d0(this)` | ledgered `game::enemies` |
| `0x80A86400` | `fn_80a86400(this)` | ledgered `game::enemies` |
| `0x80A855A0` | `fn_80a855a0(this)` | ledgered `game::enemies` |
| `0x80197F00` | `sound_play(mgr, id, pos, arg)` | ledgered `audio::ax` |
| `0x80198C30` | `sound_play_3d(mgr, id, pos, arg)` | ledgered `audio::ax` |
| `0x80429DD0` | stage/height manager | `main.dol`; `+0x8FEC4` water level, `+0x90038` floor height |
| `0x80429E00` | camera/wipe manager | `main.dol`; `+0x38`/`+0x3C`/`+0x40`/`+0x44` bounds |
| `0x80429D40` | player manager | `main.dol`; `+0x24` player count, `+0x44` bound |
| `0x8042A050` | DSys work object | `main.dol` |
| `0x80429F10` | sound/flag manager | `main.dol` |
| `0x80429D60` | sound manager | `main.dol` |
| `0x80429D5C` | sound manager (3D) | `main.dol` |
| `0x8042A038` | resource manager | `main.dol` |
| `0x8042A1C8` | stage layer table | `main.dol`; `+0x120E` is the current layer byte |
| `0x80429E98` | stage section base | `main.dol`; `+0x3C` is the section data pointer |
| `0x80377C48` | resource-name object | `main.dol` |

---

## 1. `0x80A3DEA0` - param-nibble constructor with a 16-state dispatch

* **Address:** `0x80A3DEA0`
* **TSV size:** 2940 bytes (735 words)
* **Reachable body:** 735 instructions, the whole extent with no gaps once the
  16-case jump table is read; 16 direct `bl`, one `bctrl` (vtable `+0xD4`),
  27 conditional branches, six direct `b` joins, one `blr` at `0x80A3EA18`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a3dea0_FUN_80a3dea0.c`
  - the export omits the first ~30 prologue instructions behind the `f31`
  spill and renders `_savegpr_27`/`_restgpr_27` as a 64-bit "return"; the RAW
  stream is authoritative.
* **Proposed signature:** `pub unsafe fn fn_80a3dea0(this: usize)`
  - `this` is the actor; `u32[this+4]` is the constructor parameter block.
* **Callers in this REL:** none (`bl` scan of the file-backed range finds no
  32-bit reference to the entry); the body is reached from the `.data` actor
  factory table, not from code in this REL.
* **Constants:** one block at `0x80B2C4E8` (`lis r31,0x80B3; addi -15128`) and
  a BSS name-slot block at `0x80B706D8` (`lis r30,0x80B7; addi 1752`).

| address | type | value / role |
|---|---|---|
| `0x80B2C53C` | `f32` | `0.0` - initial `+0xA74`, `+0x36D` guard, colour write |
| `0x80B2C540` | `f32` | `8.0` - spawn offset applied to x/y |
| `0x80B2C544` | `f32` | `-3000.0` - z used when `+0x38F` is set |
| `0x80B2C548` | `f32` | `500.0` - z otherwise |
| `0x80B2C54C` | `f32` | `1.0` - default colour/scale and `+0xE8` seed |
| `0x80B2C550` | `f32` | `0.89` - state-15 impact colour |
| `0x80B2C554` | `f32` | `17.0` - state-5/state-11 y lift |
| `0x80B2C558` | `f32` | `320.0` - state-8 collision half-extent x |
| `0x80B2C55C` | `f32` | `64.0` - state-8 half-extent y |
| `0x80B2C560` | `f32` | `256.0` - default half-extent x |
| `0x80B2C564` | `f32` | `512.0` - default half-extent y |
| `0x80B3EE94` | `u32[16]` | jump table: `0x80A3E20C, 0x80A3E734, 0x80A3E768, 0x80A3E988, 0x80A3E7A8, 0x80A3E978, 0x80A3E944, 0x80A3E7EC, 0x80A3E67C, 0x80A3E844, 0x80A3E8A8, 0x80A3E8CC, 0x80A3E910, 0x80A3E20C, 0x80A3E65C, 0x80A3E4A4` |
| `0x80B2C4F0/4FC/508` | `f32[4]` | state-4 triple indexed by `(param >> 2) & 0x3C` |
| `0x80B2C518/528` | `f32[4]` | state-7 pair indexed the same way |
| `0x8042BB18` | `u8[2]` **(ext)** | `{1,1}` seed for `+0x690` when `+0xA80 == 1` |
| `0x80353930`, `0x803537F0`, `0x80353870`, `0x803538B0`, `0x803538F0`, `0x80353830` | name objects **(ext, main.dol)** | state-0/13 animation choices |
| `0x80B706E8`, `0x80B70828`, `0x80B707A8`, `0x80B70768`, `0x80B709A8`, `0x80B70868`, `0x80B708E8`, `0x80B70928`, `0x80B70968`, `0x80B709E8`, `0x80B70A28` | name objects **(ext, REL BSS)** | the other states' animation names |

* **Fields:** `+0x04` parameter block; `+0x60` vtable; `+0xAC/B0/B4` position;
  `+0xDC/E0/E4` colour/scale; `+0xE8`, `+0xEC` state seeds; `+0x124`, `+0x1E6`,
  `+0x318`, `+0x31C`, `+0x320`, `+0x324`, `+0x348`, `+0x36D`, `+0x38F`,
  `+0x45C`, `+0x45D`, `+0x574/578/57C`, `+0x690`, `+0x698..0x6A0`,
  `+0x6D0/6D4/6D8`, `+0x6E8..0x6F0` (spawn copy), `+0x6F4..0x6FC`,
  `+0x710`, `+0x718`, `+0x71C`, `+0x724`, `+0xA4C`, `+0xA64`, `+0xA68`,
  `+0xA74`, `+0xA78`, `+0xA7C` (state), `+0xA80`, `+0xA90`, `+0xA94`,
  `+0xA95`, `+0xA99`, `+0xAA4`, `+0xAB4`.
* **Verdict:** **ready** (confidence high) - the jump table and every float
  block are inside the image, there is no incoming float and no `extraout_`.

### Frame

`stwu r1,-112(r1)`; LR at `+0x74`; `addi r11,r1,96`; `stfd f31,96` +
`psq_st f31,104,0,qr0` (plain `f31` spill); `bl 0x802DCD74` saves `r27..r31`.
Epilogue at `0x80A3E9FC..0x80A3EA18` restores `f31`, calls `0x802DCDC0`, reloads
LR and `blr`s. Live registers: `r27 = this`, `r28`, `r29 = param`, `r30 = name
block`, `r31 = float block`.

### Parameter unpack

```
p = u32[this+0x04]
state      =  p        & 0xF   -> +0xA7C
mode       = (p >> 20) & 0xF   -> +0x718
sub        = (p >>  8) & 0xF   -> +0xA80 and +0xA95
layer      = (p >> 16) & 0xF   -> +0x348
variant    =  p >> 24          -> +0x71C
state9kind = (p >> 12) & 0xF   -> +0xA99
```

### Behaviour

```c
this = r3;
param = u32[this+4];
// spawn copy
f32[this+0x6E8..0x6F0] = f32[this+0xAC..0xB4];
f32[this+0xA74] = 0.0;
+0xA7C = param & 0xF; +0x718 = (param>>20)&0xF; +0x348 = (param>>16)&0xF;
+0x71C = param>>24;  +0xA80 = (param>>8)&0xF;  +0xA95 = (param>>8)&0xF;
+0xA99 = (param>>12)&0xF;
+0x6E8 = x; +0x6EC = y; +0x6F0 = z;

if (+0x718 == 2) { +0xAA4 = 1; +0x718 = 0; }
if (+0x718 == 4 || +0x718 == 7) +0xAB4 = 1;

if (+0xAB4 != 0) {                       // states 4/7: ride a linked object
    act_query(0x178, this, (+0x718==4 ? 0 : 1), &pos, 0, 0);
    for (rec = link_first(this); rec; rec = link_next(rec)) {
        actor_world_pos(v, rec);
        f32[this+0xAC] = v[0] - 8.0f;
        actor_world_pos(v, rec);
        f32[this+0xB0] = 8.0f + v[1];
    }
}
f32[this+0x710] = 0.0;
f32[this+0xB0] -= 8.0f;
f32[this+0xAC] += 8.0f;
actor_physics_step(this);
f32[this+0xB4] = u8[this+0x38F] ? -3000.0f : 500.0f;
f32[this+0x71C] = param >> 24;
u8 [this+0x348] = (param >> 16) & 0xF;
+0xA80 = 0;
if (+0xA7C == 0 && (param>>8)&0xF != 0) {
    +0xA80 = (param>>8)&0xF;
    f32[this+0xB4] = -3000.0f;
    f32[this+0x6F0] = -3000.0f;
}
if (+0xA7C == 0 || +0xA7C == 13) {        // ground-bound states
    if (f32[this+0xB4] != -3000.0f && +0xA7C != 13) f32[this+0xB4] = 500.0f;
    if (*(u32*)0x80429DD0 != +0xA4C) {
        if (+0xA4C) ground_link_remove(+0xA4C+100, &+0xA4C, +0xA4C==-100?0:+0xA4C);
        if (new)     ground_link_add   (new +100,  &+0xA4C, new==-100?0:new);
    }
    h = stage_handle(+0xA4C, 0);
    fn_80014820(h, this+0x698);
    u16[this+0x6A0] = 30;
    fn_800146B0(x, -y, z, this+0x698);    // cylinder descriptor
} else {
    actor_mode_set(this, (+0xA7C - 1) < 2);
}
+0xA95 = (param >> 8) & 0xF;
actor_setup_2F0(this);
+0x36D = 0;
f32[this+0xDC..0xE4] = 1.0f;
+0x690 = (+0xA80 == 1) ? u8[0x8042BB18 + 1] : 1;
actor_set_scale_mode(this, +0x690);
+0x6D0 = 0; +0x6D4 = 0x80A3F360; +0x6D8 = 0;
+0x574 = 0; +0x578 = 0x80A3F360; +0x57C = 0;
if (+0xA7C != 0 && +0xA7C != 13 && stage_probe(this, &pos)) +0x45D = 1;
state_dispatch(+0xA7C);                   // 16-case table, below
if (+0xA7C == 8) { f32[this+0x318]=320.0f; f32[this+0x31C]=64.0f; }
else             { f32[this+0x318]=256.0f; f32[this+0x31C]=512.0f; }
f32[this+0x320] = 0.0f;
f32[this+0x324] = 8.0f;
```

### State dispatch

| state | target | behaviour |
|---:|---|---|
| 0, 13 | `0x80A3E20C` | if `+0xAA4 == 0` and `stage_ground_probe(0x80429DD0, x, -y, +0x38F)` returns non-zero: return `2`, set `*(u32*)0x80429D14 |= 1`. Otherwise pick the clip from `+0x718` (0: `+0x71C==0` -> `0x80353930` (state 13 -> `0x80B706E8`), else copy spawn to `+0x6F4..` and use `0x803537F0`; 8: `+0x724=+0x71C`, `+0x71C=0`, physics, `0x80353930`; 3: `0x80353870`; 5: `0x803538B0`; 6: `0x803538F0`; 4/7: `0x80353930`; other: `0x80353830`), return `1` |
| 1 | `0x80A3E734` | `fn_80a3de90(this)`; `+0x348 = facing(this,&pos)`; clip `0x80B70828` |
| 2 | `0x80A3E768` | same plus `+0xA95 = 2`, `+0xA94 = 2` |
| 3 | `0x80A3E988` | default tail: `+0xA78=0xFFF`, `+0xA68=1.0`, `+0xA64=0.0`, clip `0x80B70768` |
| 4 | `0x80A3E7A8` | `i=(param>>2)&0x3C`; `+0xA78/+0xA68/+0xA64 = f32[0x80B2C4F0/4FC/508 + i]`; clip `0x80B707A8` |
| 5 | `0x80A3E978` | `f32[+0xB0] += 17.0`; fall through to the state-3 tail |
| 6 | `0x80A3E944` | restore position from `+0x6E8..0x6F0`; clip `0x80B709A8` |
| 7 | `0x80A3E7EC` | `fn_80a3de90`; `i=(param>>2)&0x3C`; `+0xA90=0`, `+0xE8=f32[0x80B2C518+i]`, `+0xEC=f32[0x80B2C528+i]`, `+0x124=0`, `+0x45D=0`, `+0x45C=1`; clip `0x80B70868` |
| 8 | `0x80A3E67C` | clip by `+0x718`/`+0x71C` as in state 0; `fn_80a3de90`; `+0xA90=0` |
| 9 | `0x80A3E844` | `+0xA99=(param>>12)&0xF`; clip `0x80B708E8`; `sound_play_3d(0x80429D60, 0x237, &pos, fn_80069530(+0xA99))`; `fn_80a3f280(this)` |
| 10 | `0x80A3E8A8` | clip `0x80B70928`; `fn_80a3f280(this)` |
| 11 | `0x80A3E8CC` | `f32[+0xB0] += 17.0`; `+0xA78=0xFFF`, `+0xA68=1.0`, `+0xA64=0.0`; clip `0x80B70768` |
| 12 | `0x80A3E910` | restore position from spawn; clip `0x80B70968` |
| 14 | `0x80A3E65C` | clip `0x80B709E8`; return `1` |
| 15 | `0x80A3E4A4` | two ground probes: first hit -> return `2` + global flag; second -> rebind ground, `+0xA7C=0`, clip `0x80B706E8`, return `1`; otherwise `actor_query(0x235, this, 1, &pos(x, y-8, z), 0, +0x38F)`, colour 0.89, clip `0x80B70A28`, return `1` |
| >15 | `0x80A3E988` | default tail |

### Notes

* `0x80A3DE90` (16 bytes before this entry) is a separate unclaimed body; the
  four calls above (`states 1, 2, 7, 8`) are `bl 0x80A3DE90`.
* `+0x6D4`/`+0x578` are set to the fixed address `0x80A3F360`, a ledgered
  `game::enemies` callback; they are the "on-ground" pair for the two
  collision descriptors at `+0x6CC` and `+0x570`.
* The two word stores at `0x80A3E1AC`/`0x80A3E1B8` are the same constant
  (`0x80A3F360`) into both descriptors, not a table read.

---

## 2. `0x80B04E90` - stage-section polyline analyser

* **Address:** `0x80B04E90`
* **TSV size:** 1592 bytes
* **Reachable body:** 398 instructions = the whole extent; 5 direct `bl`, 44
  conditional branches, six `b` joins, one `blr` at `0x80B054C4`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b04e90_FUN_80b04e90.c`
  - the export is a long `do/while` unroll of two byte-fill loops; the block
     structure below is exact and shorter than the C.
* **Proposed signature:** `pub unsafe fn fn_80b04e90(this: usize)`
  - `this` is a stage-object record; `u32[this+4]` packs
    `kind = (p >> 4) & 0xF`, `start = (p >> 12) & 0xFF`,
    `base = (p >> 8) & 0xFF0`.
* **Callers in this REL:** one `bl` from `0x80B04A70` (the object's constructor
  at `0x80B04A70..0x80B04E8F`, not in `functions.tsv`).
* **Constants:** none in the image; all inputs are the object, the current
  stage layer and the section record.

### Frame

`stwu r1,-64(r1)`; LR at `+0x44`; `bl 0x802DCD5C` saves `r23..r31`; epilogue
`bl 0x802DCDA8` at `0x80B054B4`. Scratch: `r30 = this`.

### Behaviour

```c
this = r3;
p = u32[this+4];
entry = stage_object_section_entry((p >> 4) & 0xF);      // 0x800D90C0
sec   = *(u32*)0x80429E98 + (u32)u8[*(u32*)0x8042A1C8 + 0x120E] * 0x3B0;
if (u32[sec+0x2C] == 0) sec = 0;
count = u16[entry+4];                                     // point count
pts   = (s16*)((p & 0xFF0) + u32[sec+0x3C] + u16[entry+2] * 0x10);
start = (p >> 12) & 0xFF;

// pass 1: total run length = sum of |dx| + |dy| over the polyline
total = 0;
for (i = start; i < count - 1; i++) {
    dx = (s16)((pts[8] >> 4) - (pts[0] >> 4));
    dy = (s16)((pts[9] >> 4) - (pts[1] >> 4));
    total += abs16(dx) + abs16(dy);                       // 0x802DE514
    pts += 8;                                             // 16-byte points
}
u32[this+0x2B20] = total;
buf = mem_alloc(total + 2);                               // 0x802B9250
u32[this+0x2B1C] = buf;

// pass 2: emit a direction byte per unit step
out = 1;
pts = initial;
for (i = start; i < count - 1; i++) {
    dx = (s16)((pts[8] >> 4) - (pts[0] >> 4));
    dy = (s16)((pts[9] >> 4) - (pts[1] >> 4));
    ax = abs16(dx); ay = abs16(dy);
    hdir = dx < 0 ? 3 : dx > 0 ? 4 : 0;                   // left / right / none
    vdir = dy < 0 ? 1 : dy > 0 ? 2 : 0;                   // down / up / none
    if (ay < ax) { emit min(ax,ay) pairs (hdir,vdir), then (ax-ay) hdir }
    else         { emit min(ax,ay) pairs (hdir,vdir), then (ay-ax) vdir }
    out += ax + ay;
    pts += 8;
}
buf[0] = 0;
buf[total + 1] = 0;
```

The two emission arms are the two halves of the export's `if (iVar9 < iVar17)`
branch; the 16-byte-at-a-time and 8-byte-at-a-time unrolled fills in the export
are the compiler's `memset`-style expansion of the same run.

* **Verdict:** **ready** (confidence high) - every operand is either the object,
  a stage global with a documented role, or the section record; there is no
  float, no table and no indirect call.

---

## 3. `0x80A7BEC0` - `walk_*` clip-sequence player with contact turn

* **Address:** `0x80A7BEC0`
* **TSV size:** 1464 bytes
* **Reachable body:** 366 instructions = the whole extent; 23 direct `bl`, six
  vtable `bctrl`, 18 conditional branches, six `b` joins, one `blr` at
  `0x80A7C474`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a7bec0_FUN_80a7bec0.c`
  - the first ~130 lines are `ldexpf`/`CONCAT` restores of the `f28..f31`
  prologue spills; the body underneath is ordinary scalar code. Ghidra removes
  four unreachable stores at the tail (`0x80A7C444..0x80A7C45C`).
* **Proposed signature:** `pub unsafe fn fn_80a7bec0(this: usize)`
* **Callers in this REL:** none; the body is reached from the actor's state
  table.
* **Constants:**

| address | type | value / role |
|---|---|---|
| `0x80B2DE20` | `f32` | `0.0` - zero writes and `+0xE8` seed |
| `0x80B2DE28` | `f32` | `1.0` - weight for `0x80169F80`/`0x801680E0` |
| `0x80B2DE40` | `f32` | `45.0` - the "long clip finished" length |
| `0x80B2DE44` | `f32` | `40.0` - default clip length |
| `0x80B2DE48` | `f32` | `0.25` - end-of-sequence sentinel / step |
| `0x80B2E154` | `f32` | `3.0` - landing smoke scale |
| `0x80B2E158` | `f32` | `1.2` - smoker scale written into BSS |
| `0x80B2DE58` | `u32[]` | `PTR_s_walk_st_AR_m` - clip-name pointer array |
| `0x80B2DE5C` | `u32[]` | `PTR_s_walk_ed_BR_m` - clip-name pointer array |
| `0x80B73158..0x80B73164` | BSS **(ext)** | smoke-emitter "initialised" flag + three `1.2` scales |
| `0x80B73028` | name object **(ext, BSS)** | fallback clip |
| `0x80B4C63C`/`0x80B4C64C` | strings (image) | the `walk_st_AR_*` / `walk_ed_BR_*` clip names |

* **Fields:** `+0x60` vtable; `+0x348` u8 facing; `+0x544`/`+0x584` animation
  handles; `+0x5B8` u8; `+0x614` clip-name cursor; `+0x618` float-length
  cursor; `+0x688..0x690` smoke origin; `+0xD1C + i*0xA4` collision slots;
  `+0xE80` f32 cached ground height; `+0xE92` s16 turn step; `+0xE9A`, `+0xE9B`,
  `+0xE9C`, `+0xE9E`, `+0xEA0`, `+0xEA1`, `+0xEA2`, `+0xEA3`, `+0xEA5`,
  `+0xEA6`, `+0xEA8` u8 state bytes.
* **Verdict:** **ready** (confidence high) - the `psq_st/psq_l` pairs are the
  `f28..f31` spills only; every callee has explicit arguments in the stream.

### Frame

`stwu r1,-160(r1)`; LR at `+0xA4`; `stfd/psq_st` pairs for `f31` (`96/152`),
`f30` (`128/136`), `f29` (`112/120`) and `f28` (`96/104`); `bl 0x802DCD54`
saves `r22..r31`. Epilogue at `0x80A7C454..0x80A7C474` restores the four FPRs
and calls `0x802DCDA0`.

### Behaviour

```c
this = r3;
fn_80a79df0(this);                        // state ladder (ledgered)
n = u8[this+0xEA6];                       // iteration count, saved in r24
if (u8[this+0xE9A] != 0 && u8[this+0xEA0] == 0) {
    u8[this+0xEA0] = 0x10;
    step = u8[this+0xEA1] ? -0x800 : 0x800;
    if (*(u32*)(this+0x618) == 0x80B2DE48) step = -step;
    s16[this+0xE92] = step;
    if (*(u32*)(this+0x618) == 0x80B2DE48) {
        if (u8[this+0xE9B]) { fn_80168fe0(f32[this+0xE80], this+0x584); u8[this+0xE9B]=0; }
        u8[this+0x5B8] = 3;
        u8[this+0xEA1] ^= 1;
        u8[this+0x348] ^= 1;                       // mirror facing
        swap(u8[this+0xEA2], u8[this+0xEA3]);
        u32[this+0x614] = &walk_ed_BR_m[dir*0x24 + flip*0x12 + ea3*6 + ea2*2];
        *(u32*)(this+0x618) = 0x80B2DE44;
        fn_80a7a2c0(40.0f, this);
        u8[this+0xE9C] = 1;
        bit = u8[0x80B2DE38 + flip + dir*2];        // (u8*)(-0x7F4D21C8)
        u8[this+0xEA8] &= ~(1 << bit);
        collision_area_unregister(this + 0xD1C + bit*0xA4);
    } else {
        n = 2; u8[this+0xEA6] = 2;
    }
    fn_80a79cc0(this, 0, 1);
    fn_80a7ae80(this);
}
if (u8[this+0xE9E] == 0) {
    do {
        // takeoff event, only while the cursor is at the end sentinel
        if (!u8[this+0xE9B] && fn_80a7a3c0(this) == 0 && *(u32*)(this+0x618) == 0x80B2DE48) {
            f32[this+0xE80] = fn_80168080(this+0x584);
            f32[this+0xE8] = 0.0f;
            u8[this+0xEA8] = 3;
            vslot(this+0x60, 0x9C)(this);
            u8[this+0xE9B] = 1;
            sound_play(0x80429D5C, 0x1E2, &pos, 0);
            p = this + 0x688 + idx*0xC;            // idx from +0xEA1/+0x348
            if (u8[0x80B73158] == 0) {
                f32[0x80B7315C..0x80B73164] = 1.2f; u8[0x80B73158] = 1;
            }
            fn_8016C890("Wm_en_landsmoke_s", 0, &p, 0, &0x80B7315C);
        }
        fn_8006cd40(this);
        if (fn_80169020(this+0x584) == 0) {
            vslot(this+0x544, 0x1C)(this+0x544);
            fn_80a7a030(this); fn_80a7a170(this);
        } else {
            cur = *(float**)(this+0x618);
            if (cur >= &0x80B2DE48) {              // sequence exhausted
                if (fn_80a7ab90(this) == 0) clip(this, 0x80B73028);
                else {
                    name = walk_st_AR[dir*0x24 + flip*0x12 + ea2*6 + ea3*2];
                    u32[this+0x614] = &walk_st_AR[that index] + 1;
                    anim = brres_get_anm_chr(this+0x540, name);
                    fn_801651f0(this+0x584, this+0x544, &anim, 1);
                    fn_80169f80(1.0f, this+0x544, this+0x584);
                    fn_801680e0(1.0f, this+0x584);
                    vslot(this+0x544, 0x1C)(this+0x544);
                    fn_80168fe0(0.0f, this+0x584);
                    u8[this+0xE9B] = 0; u8[this+0xE9C] = 0;
                    u8[this+0xEA6] = u8[this+0xEA5];
                    *(u32*)(this+0x618) = 0x80B2DE44;
                    fn_80a7a2c0(45.0f, this); fn_80a7a360(45.0f, this);
                    u8[this+0xEA8] &= ~(1 << bit);
                    collision_area_unregister(this + 0xD1C + bit*0xA4);
                    fn_80a7a030(this); fn_80a7a170(this);
                }
                break;
            }
            anim = brres_get_anm_chr(this+0x540, *cur);
            fn_801651f0(this+0x584, this+0x544, &anim, 1);
            fn_80169f80(1.0f, this+0x544, this+0x584);
            fn_801680e0(1.0f, this+0x584);
            vslot(this+0x544, 0x1C)(this+0x544);
            if (!u8[this+0xE9C]) fn_80168fe0(0.0f, this+0x584);
            len = *cur; *(float**)(this+0x618) = cur + 1;
            fn_80a7a2c0(len, this); fn_80a7a360(len, this);
            if (u8[this+0xE9C]) fn_80a7a3c0(this);
            u8[this+0xE9C] = 0;
            fn_80a7a030(this); fn_80a7a170(this);
        }
    } while (--n != 0);
} else {
    u8[this+0xE9E]--;
    fn_80a7a030(this); fn_80a7a170(this);
}
```

* **Notes:** the clip-name stride is `0x24` per facing (2), `0x12` per flip (2),
  `6` per `+0xEA2`, `2` per `+0xEA3` - a 2x2x3x3 table. The `bit` index is
  `u8[0x80B2DE38 + +0xEA1 + +0x348*2]`; `0x80B2DE38` is inside the same image
  block as the constants above.
* The `+0xE92` turn step (`+-0x800`, i.e. `+-1/32` turn) is written every time
  the "turn armed" byte `+0xEA0` was zero; it is the contact-driven yaw step the
  parent state machine consumes.

---

## 4. `0x80A9B3C0` - per-slot shot/effect record builder

* **Address:** `0x80A9B3C0`
* **TSV size:** 1196 bytes
* **Reachable body:** 299 instructions = the whole extent; 11 direct `bl`, 27
  conditional branches, nine `b`/loop joins, one `blr` at `0x80A9B868`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a9b3c0_FUN_80a9b3c0.c`
  - declared `void FUN_80a9b3c0(void)`; the `uVar12 = func_0x802dcd70()` split
    into `(u32)uVar12`/`(u32)(uVar12>>32)` is Ghidra modelling the
    `_savegpr_26` 64-bit "return" pair. The RAW stream proves **two** incoming
    arguments: `r3 = this` (kept in `r27` at `0x80A9B3E4`) and `r4 = slot`
    (kept in `r28` at `0x80A9B3E8`).
* **Proposed signature:**
  `pub unsafe fn fn_80a9b3c0(this: usize, slot: usize)`
  - `slot` is used both as an index (`1 << slot`) and as a byte offset
    (`r31 = this + slot`), so the four callers pass `0, 1, 2, 3` (the
    `+0xC38 + slot*2` halfword array and the `+0xC50 + slot` byte array are
    four-player slot tables).
* **Callers in this REL:** none by `bl`; reached indirectly (state/vtable
  table), which is also why the slot argument is not visible inside this body.
* **Constants:**

| address | type | value / role |
|---|---|---|
| `0x80B51CC8`/`0x80B51CCC`/`0x80B51CD0` | `{0, -1, fn}` | "previous slot" neighbour record for `0x802DCBC0` |
| `0x80B51CD4`/`0x80B51CD8`/`0x80B51CDC` | `{0, -1, fn}` | alternate neighbour record |
| `0x80B51CE0`/`0x80B51CE4`/`0x80B51CE8` | `{0, -1, fn}` | "next slot" neighbour record |
| `0x80B51CEC`/`0x80B51CF0`/`0x80B51CF4` | `{0, -1, fn}` | alternate next-slot record |
| `0x80B2EFDC` | `u16[]` | per-facing angle pair selected by `+0x348 & 1` |
| `0x80B2EFE4` | `f32` | `0.0` |
| `0x80B2EFF8` | `f32` | `16.0` |
| `0x80B2EFFC` | `f32` | `8.0` |
| `0x80B2F008` | `f64` | `2^52` bias for `(double)CONCAT44(0x43300000, ...) - 2^52` |
| `0x80B2F010` | `f32` | `0.5` |
| `0x80B2F014` | `f32` | `3000.0` |
| `0x80430038`/`0x8043003C`/`0x80430040`/`0x80430044` | `f32` **(ext)** | player-region bounds from `0x80429E00` |
| `0x8042BB19` | `u8` **(ext)** | alias flag read by the parallel body at `0x80AA0C20` |

* **Fields:** `+0x04` param word (bit 28 selects the `0x800B2EE0` timer
  variant); `+0x348` u8 facing mask; `+0xB04 + slot*12` neighbour record;
  `+0xC38 + slot*2` u16 angle; `+0xC48 + slot` u8 timer; `+0xC50 + slot` u8
  second timer; `+0xC58 + slot` u8 "shot fired"; `+0xC80` u8 active mask;
  `+0xC81` u8 slot count; `+0xC88 + slot` u8 record state; `+0x744 + slot*0x38`
  collider block; `+0xA04 + slot*0x20 + i*8 + side*4` shot record words;
  `+0xAC/B0/B4` position.
* **Verdict:** **conditional** (confidence medium) - the body is complete and
  every constant is in-image, but the second argument (`r4`) is a slot ordinal
  that only the indirect caller stages, and four callees
  (`0x800B2EE0`, `0x802DCBC0`, `0x80A9B1B0`, `0x80A9AF80`) are effect-band
  helpers whose argument meanings are established here only by use. Once the
  vtable entry that reaches this body is identified, the row is ready.

### Frame

`stwu r1,-128(r1)`; LR at `+0x84`; `bl 0x802DCD70` saves `r26..r31`; epilogue
`bl 0x802DCDBC` at `0x80A9B858`. `r27 = this`, `r28 = slot`,
`r29 = 0x80B51C50`, `r30 = 0x80B2EFD8`, `r31 = this + slot`.

### Behaviour (shape)

```c
this = r3; slot = r4;
p = this + slot;                       // per-slot record base
if (s8[p+0xC88] == 0) {                // arm
    u16[p+0xC38] = u16[0x80B2EFDC + ((u8[this+0x348] >> slot) & 1)*2];
    if (((u8[this+0xC80] >> slot) & 1) == 0) {
        if ((u32[this+4] & 0x10) == 0) u8[p+0xC48] = 0x20;
        else                           u8[p+0xC48] = fn_800B2EE0(6)*0x20 + 0x10;
    }
    u8[this+0xC50+slot] = 0x40;
    s8[p+0xC88] += 1;
    return;
}
if (s8[p+0xC88] == -1) return;
if (((u8[this+0xC80] >> slot) & 1) == 0) {
    if (u8[p+0xC48] == 0) {
        if (u8[this+0xC50+slot] != 0 && --u8[this+0xC50+slot] == 0)
            fn_80a9b1b0(this, &kf_rec, slot);
    } else if (--u8[p+0xC48] == 0) {
        // shot spawn: threshold 16.0/8.0 against the player region
        ...
        hit = fn_80a9af80(this, slot, facing_bit);
        if (hit != -1) {
            rec = fn_80aaa360(&point, (facing_bit<<4)|2|(hit<<8), &out, 0x20);
            sound_play_3d(0x80429D60, 0x1BD, &pos, 0);
            u8[this+0xC58+slot] = 1;
            fn_80168fe0(0.0, this + slot*0x38 + 0x744);
        }
    }
} else {
    // scan the other slots (up then down) for a free neighbour record;
    // the first free one runs the same keyframe callback and rewinds.
    ...
}
```

The exact arm/scan/shot structure is the export's three nested `if`/`do`
groups (`80a9b3c0_FUN_80a9b3c0.c:25-160`); the two `0x802DCBC0` calls are a
three-word keyframe record compare (`{0, -1, fn}`) against
`this + slot*12 + 0xB04`.

---

## 5. `0x80B147D0` - `daEnTogeTekkyu_c` 4-variant initialiser

* **Address:** `0x80B147D0`
* **TSV size:** 1184 bytes
* **Reachable body:** 296 instructions = the whole extent; 5 direct `bl`, four
  vtable `bctrl`, four conditional branches (`+0x...F0/F8/00/08`), one `blr` at
  `0x80B14C6C`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b147d0_FUN_80b147d0.c`
  - the export is the four `if/else` arms verbatim; every arm writes the same
  two blocks and then calls the same `0x8006D090`.
* **Proposed signature:** `pub unsafe fn fn_80b147d0(this: usize)`
  - variant = `u32[this+4] & 3`.
* **Callers in this REL:** none by `bl`; instantiated from the actor factory.
* **Constants:** four copies from `main.dol` `0x802EE7A0..0x802EE830` **(ext)**:

| address | role |
|---|---|
| `0x802EE7A0..0x802EE7CC` | 11-word animation/curve block copied to `+0x614..0x63C` for variants 0/2/3 (`0x802EE7E0` for variant 1) |
| `0x802EE810`, `0x802EE814`, `0x802EE818`, `0x802EE81C` | collision descriptor floats 0..3 |
| `0x802EE820`, `0x802EE821` | two `u8` collision bytes |
| `0x802EE824`, `0x802EE828` | two `u32` collision words |
| `0x802EE82C` | `u16` collision field |
| `0x802EE830` | `u32` collision field |
| `0x8042B6B0` | `u32[4]` direction -> `+0xE8` seed table **(ext)** |
| `0x80353CA8`, `0x803539E4` | name objects **(ext, main.dol)** |
| `0x80B78E78` | name object **(ext, REL BSS)** |

* **Fields:** `+0x04` param; `+0x60` vtable; `+0xE8` f32 seed; `+0xEC` f32;
  `+0x100` s16 half-turn; `+0x148` collision slot; `+0x1EC` child slot;
  `+0x238` position pointer (set to `this+0xAC`); `+0x348` u8 facing;
  `+0x5F0..0x610` copied collision descriptor; `+0x614..0x63C` 11-word block;
  `+0x620` block used as a collision descriptor source; `+0x630`;
  `+0x77C` u32 mode; `+0x7C4` u8 flag.
* **Verdict:** **ready** (confidence high) - four straight-line variants with
  fixed constants; nothing depends on an unresolved register.

### Behaviour

```c
this = r3;
switch (u32[this+4] & 3) {
case 0:
    copy descriptor 0x802EE810..0x802EE830 -> this+0x5F0..0x610;
    collision_area_set_descriptor(this+0x148, this, this+0x5F0);
    collision_area_register(this+0x148);
    copy 0x802EE7A0..0x802EE7CC -> this+0x614..0x63C;
    +0x348 = facing(this, &pos);
    +0x7C4 = 0; +0x77C = 1;
    +0xE8 = u32[0x8042B6B0 + +0x348*4];
    clip(this, 0x80353CA8);
    break;
case 1:
    copy 0x802EE7E4..0x802EE80C (11 words, base 0x802EE7E0) -> this+0x614..0x63C;
    +0x7C4 = 0; +0x77C = 2;
    clip(this, 0x80B78E78);
    break;
case 2:
    same descriptor + 11-word block as case 0;
    +0x348 = facing(this, &pos);
    +0xEC = 0.0; +0x7C4 = 0; +0x77C = 0;
    clip(this, 0x803539E4);
    break;
case 3:
    same descriptor + block;
    +0x348 = facing(this, &pos);
    +0x100 = 0x8000;                                       // half turn
    +0xEC = 0.0; +0x7C4 = 1; +0x77C = 0;
    clip(this, 0x803539E4);
    break;
}
collision_slot_bind(this+0x1EC, this, this+0x620, this+0x614, this+0x630);
f32[this+0x238] = this + 0xAC;                             // position pointer
```

* **Notes:** variant 1 reuses the `uVar1/uVar3..uVar11` values already loaded
  from `0x802EE7E4..0x802EE80C` in the prologue; the base word for that copy is
  `0x802EE7E0`, one word before the shared block, so the 11 destination words
  are `0x802EE7E4..0x802EE80C`. All four arms end at `0x80B14C38`.

---

## 6. `0x80A92F40` - player-anchored chaser step

* **Address:** `0x80A92F40`
* **TSV size:** 1172 bytes
* **Reachable body:** 293 instructions = the whole extent; 19 direct `bl`,
  four vtable `bctrl`, 29 conditional branches, 12 `b` joins, one `blr` at
  `0x80A933D0`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a92f40_FUN_80a92f40.c`
  - correct shape, but several float fields are printed as `int` (`piVar4[0x2b]`
    is `f32[+0xAC]`, `piVar4[0x37]` is `f32[+0xDC]`); the listing below uses the
    true types. The `0x43300000`/`2^52` blocks are the unsigned-int conversion
    of `u16`/`u8` values.
* **Proposed signature:** `pub unsafe fn fn_80a92f40(this: usize)`
* **Callers in this REL:** none by `bl`; reached from the actor state table.
* **Constants:**

| address | type | value / role |
|---|---|---|
| `0x80B2EAE0` | `f32` | `1.0` - scale ramp ceiling |
| `0x80B2EB1C` | `f32` | `0.015625` (1/64) - scale ramp per frame |
| `0x80B2EA7C` | `f32` | `1.25` - scale clamp |
| `0x80B2EAD0` | `f32` | `0.0` |
| `0x80B2EAE8` | `f32` | `16.0` |
| `0x80B2EAEC` | `f32` | `-2.0` |
| `0x80B2EAF0` | `f32` | `2.0` |
| `0x80B2EB20` | `f32` | `0.0625` (1/16) |
| `0x80B2EB34` | `f32` | `-0.125` |
| `0x80B2EB38` | `f32` | `1.625` |
| `0x80B2EB3C` | `f32` | `-2.375` |
| `0x80B2EB40` | `f32` | `-0.0625` |
| `0x80B2EB44` | `f32` | `-0.375` |
| `0x80B2EB48` | `f32` | `0.125` |
| `0x80B2EB50` | `f64` | `2^52` bias |
| `0x80B2EB2C`/`0x80B2EB30` | `s16[2]` | turn steps indexed by `+0x348` |
| `0x80429E00` | manager **(ext)** | `+0x38`/`+0x3C`/`+0x40`/`+0x44` region |
| `0x80429D40` | player manager **(ext)** | `+0x44` bound |
| `0x80429DD0` | height manager **(ext)** | `+0x90038` floor |

* **Fields:** `+0x60` vtable; `+0xAC/B0/B4` position; `+0xDC/E0/E4` scale;
  `+0xE8`, `+0xEC` vertical speeds; `+0xF4` turn step; `+0x102` s16 angle;
  `+0x114/118/11C` facing quartet; `+0x348` u8 facing; `+0x548` trail object;
  `+0x70C` "attached" word; `+0x726` u8 player index; `+0xEA4` player-action
  base; `+0xEA8` s16 sub-state.
* **Verdict:** **conditional** (confidence medium) - the body is fully decoded,
  but it consumes three unledgered `main.dol` helpers whose contracts are not in
  the module: `0x8004A7B0` (writes a player position vector),
  `0x80056CF0` (a player-flag test that selects turn step `0x800` and bypasses
  the facing quartet) and `0x80064D10` (the "drop" path). Once those are
  ledgered, the row is ready.

### Behaviour (condensed)

```c
this = r3;
// 1. resolve the player linked to this actor
p = (+0x70C == 0) ? fn_8005F900(s8[this+0x726]) : fn_8005FA60(s8[this+0x726]);
if (p == 0) { clip(this, 0x80B74038); return; }
t = fn_80a922c0(this);
if (t != 0) { fn_80064D10(this, 0); return; }
if (u32[p+0x1008] != u32[this+0]) { fn_80a91d10(this); clip(this, 0x80B74038); return; }

// 2. scale ramp 0.0 -> 1.0 in 1/64 steps, clamp 1.25
if (f32[this+0xDC] < 1.0) {
    s = min(f32[this+0xDC] + 0.015625, 1.25);
    f32[this+0xDC] = f32[this+0xE0] = f32[this+0xE4] = s;
}
// 3. gravity/ceiling probe on the player's child slot
if (fn_80070CB0(p+0x1EC) != 0 && f32[this+0xEC] < 0.0) {
    f32[this+0xEC] = 0.0;
    if (fn_80070720(p+0x1EC) == 3) f32[this+0xB0] = (f32)((u32)f32[p+0xB0] & ~0xF);
    else { fn_8004A7B0(&v, p); f32[this+0xB0] = v[1]; }
}
// 4. player contact bits -> x/z snaps
if (u32[p+0x10D4] & 0x60) { f32[this+0xE8] = 0.0; fn_8004A7B0(&v,p); f32[this+0xAC] = v[0]; }
if (u32[p+0x10D4] & 0x02) { f32[this+0xEC] = 0.0; fn_8004A7B0(&v,p); f32[this+0xB0] = v[1]; }
// 5. facing quartet from the sub-state
if (s16[p+0xEA8] == 0) {
    f32[this+0x11C] = 0.0625; f32[this+0x114] = -0.0625;
    f32[this+0xF4]  = 0.0;    f32[this+0x118] = -0.375;
} else {
    a = fn_8005E4F0(p+0xEA4);
    if (a) { f32[this+0x114] = -0.125; f32[this+0x118] = 1.625; }
    else if (fn_8005E500(p+0xEA4)) { f32[this+0x114] = -0.125; f32[this+0x118] = -2.375; }
    else { f32[this+0x114] = -0.0625; f32[this+0x118] = -0.375; }
    b = fn_8005E510(p+0xEA4);
    if (b) { u8[this+0x348] = 1; f32[this+0x11C] = 0.125; f32[this+0xF4] = -2.0; }
    else if (fn_8005E520(p+0xEA4)) { u8[this+0x348] = 0; f32[this+0x11C] = 0.125; f32[this+0xF4] = 2.0; }
    else { u8[this+0x348] = 0; f32[this+0x11C] = 0.0625; f32[this+0xF4] = 0.0; }
}
// 6. turn step: 0x200 or 0x400 when attached, 0x800 when the flag is set
if (+0x70C == 0) step_short_clamped(&+0x102, s16[0x80B2EB2C + u8[+0x348]*2], 0x200);
else {
    idx = u8[+0x348];
    lim = 0x400;
    if (fn_80056CF0(p, 0xB0)) { idx = u8[p+0x1540]; lim = 0x800; }
    step_short_clamped(&+0x102, s16[0x80B2EB30 + idx*2], lim);
}
// 7. generic follow steps, then camera/player region clamps
fn_8006CD90(this); fn_8006CE70(this); fn_8006CD40(this);
// x: clamp into [cam[0x38], cam[0x38] + cam[0x40]]; zero +0xE8 if it was > 0
// if (player flags & (0x80000|0x100000)): +0xE8 = 0; x = player position.x
// x: if (0 < playerManager[0x44] < x) x = playerManager[0x44]  (and zero +0xE8)
// y: if (y + 16.0 > -cam[0x3C]) { y = -height[0x90038] - 16.0; if (+0xEC > 0) +0xEC = 0; }
// 8. child-object tail call
vslot(this+0x548, 0x1C)(this+0x548);
```

---

## 7. `0x80AAE520` - 4-variant segmented actor constructor

* **Address:** `0x80AAE520`
* **TSV size:** 1156 bytes
* **Reachable body:** 289 instructions = the whole extent; 7 direct `bl`, one
  vtable `bctrl`, seven conditional branches, three `b` joins, one `blr` at
  `0x80AAE9A0`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aae520_FUN_80aae520.c`
  - the `ldexpf`/`unaff_GQR0` lines are the `f29..f31` prologue spills; the
    three `do/while` loops of 15 iterations are the segment builders.
* **Proposed signature:** `pub unsafe fn fn_80aae520(this: usize)`
  - variant = `u32[this+4] & 3`; count = `(u32[this+4] >> 8) & 0xF`.
* **Callers in this REL:** none by `bl`; factory-instantiated.
* **Constants:**

| address | type | value / role |
|---|---|---|
| `0x80B2F964` | collision descriptor | 8-word template for `0x8008C3E0` (`+0x148`) |
| `0x80B2F988` | `u8[4]` | variant -> `+0x348` value (`{0,1,2,3}`) |
| `0x80B2F98C` | `f32` | `0.5` - half of the segment count |
| `0x80B2F990` | `f32` | `1.0` - colour |
| `0x80B2F994` | `f32` | `0.0` |
| `0x80B2F998` | `f32` | `112.0` - ring radius / y start |
| `0x80B2F99C` | `f32` | `64.0` - alternate radius |
| `0x80B2F9A0` | `f32` | `16.0` - per-segment y step |
| `0x80B2F9A4` | `f32` | `48.0` - variant-2 x offset |
| `0x80B2F9A8` | `f32` | `32.0` - variant-2/3 extra record offset |
| `0x80B2F9AC` | `f32` | `512.0` - `+0x318`/`+0x31C` |
| `0x80B2F9B0` | `f64` | `2^52` bias |
| `0x80B74F50` | name object **(ext, BSS)** | constructor clip |

* **Fields:** `+0x04` param; `+0x60` vtable; `+0x148` collision slot;
  `+0x164..0x170` and `+0x8CC..0x8E8` two 4-float blocks; `+0x1E6` u8;
  `+0x318`/`+0x31C` f32; `+0x348` u8; `+0x36D` u8; `+0x6E0` second collision
  slot; `+0x6FC..0x708` copy; `+0x90C + i*0xF0` 15 child slots;
  `+0x171C/+0x1720/+0x1724` packed records; `+0x17D0/+0x17D4/+0x17D8` extra
  record; `+0x1808` f32.
* **Verdict:** **ready** (confidence high) - the variant table, the collision
  templates and every float are in the image.

### Behaviour

```c
this = r3; p = u32[this+4]; v = p & 3;
+0x348 = u8[0x80B2F988 + v];
+0x1808 = 0.5f * (f32)(((p >> 8) & 0xF) + 1);
fn_80aaf050(this);
f32[+0xDC..0xE4] = 1.0f;
+0x36D = 0;
collision_area_set_descriptor(this+0x148, this, 0x80B2F964);
+0x1E6 = 1;
f32[+0x8CC]=0; f32[+0x8D0]=0; f32[+0x8D4]=112; f32[+0x8D8]=112;
f32[+0x164]=0; f32[+0x168]=0; f32[+0x16C]=112; f32[+0x170]=112;
collision_area_set_descriptor(this+0x6E0, this, 0x80B2F964);
f32[+0x8DC]=0; f32[+0x8E0]=0;
if (+0x348 < 2) { f32[+0x8E4]=64;  f32[+0x8E8]=112; }
else            { f32[+0x8E4]=112; f32[+0x8E8]=64;  }
f32[+0x6FC..0x708] = f32[+0x8DC..0x8E8];
fn_80aaeb30(this);

if (+0x348 < 2) {                       // variants 0/1: 15-point descending ring
    y = 112.0f;
    for (i = 0; i < 15; i++) {
        x = fn_80aaf2b0(112.0f, y, this);
        pack(+0x171C + i*0xC, 0x91804100);
        i32[+0x1720 + i*0xC] = ((f32[+0x8CC] + x) * 4096);
        i32[+0x1724 + i*0xC] = ((f32[+0x8D0] + y) * 4096);
        collision_slot_bind(+0x90C + i*0xF0, this, 0, 0, +0x171C + i*0xC);
        y -= 16.0f;
    }
} else if (+0x348 == 2) {               // variant 2: 48-unit x offset, extra tail
    y = 112.0f;
    for (i = 0; i < 15; i++) {
        x = fn_80aaf2b0(112.0f, y, this);
        i32[+0x1720 + i*0xC] = ((f32[+0x8CC] + y) - 48.0f) * 4096;
        i32[+0x1724 + i*0xC] = (f32[+0x8D0] - x) * 4096;
        pack(+0x171C + i*0xC, 0x91804100);
        collision_slot_bind(+0x90C + i*0xF0, this, +0x171C + i*0xC, 0, 0);
        y -= 16.0f;
    }
    pack(+0x17D0, 0x91804100);
    i32[+0x17D8] = f32[+0x8D0] * 4096;
    i32[+0x17D4] = (112.0f + f32[+0x8CC] - 32.0f) * 4096;
    collision_slot_bind(+0x1EC, this, +0x17D0, 0, 0);
} else {                                // variant 3: +32/-32, mirrored x
    y = 112.0f;
    for (i = 0; i < 15; i++) {
        x = fn_80aaf2b0(112.0f, y, this);
        i32[+0x1720 + i*0xC] = (32.0f + (f32[+0x8CC] + y)) * 4096;
        i32[+0x1724 + i*0xC] = (f32[+0x8D0] + x) * 4096;
        pack(+0x171C + i*0xC, 0x91804100);
        collision_slot_bind(+0x90C + i*0xF0, this, 0, +0x171C + i*0xC, 0);
        y -= 16.0f;
    }
    pack(+0x17D0, 0x91804100);
    i32[+0x17D8] = f32[+0x8D0] * 4096;
    i32[+0x17D4] = (32.0f + 16.0f + 112.0f + f32[+0x8CC]) * 4096;
    collision_slot_bind(+0x1EC, this, 0, +0x17D0, 0);
}
f32[+0x318] = 512.0f; f32[+0x31C] = 512.0f;
clip(this, 0x80B74F50);
```

* **Notes:** the `0x91804100` word is the packed "child record" tag; the three
  `0x8006D090` argument orders are exactly `(dst, owner, rec, 0, 0)`,
  `(dst, owner, 0, 0, rec)` and `(dst, owner, 0, rec, 0)` in the three arms.

---

## 8. `0x80B0C900` - `daEnTeresa_c` model binder

* **Address:** `0x80B0C900`
* **TSV size:** 1140 bytes
* **Reachable body:** 285 instructions = the whole extent; 24 direct `bl`, three
  vtable `bctrl`, **no conditional branch**, one `blr` at `0x80B0CD70`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b0c900_FUN_80b0c900.c`
  - exact, and the clearest of the twelve: one straight-line model setup.
* **Proposed signature:** `pub unsafe fn fn_80b0c900(this: usize)`
* **Callers in this REL:** one `bl` from `0x80B0BC44` (inside the Teresa
  constructor block, not in `functions.tsv`).
* **Constants (strings are in the image):**

| address | value |
|---|---|
| `0x80B67F5C` | `"g3d/teresa.brres"` |
| `0x80B67F70` | `"teresa"` |
| `0x80B67F78` | `"teresaA"` |
| `0x80B67F88` | `"shay_teresaA"` |
| `0x80B67F98` | `"teresaB"` |
| `0x80B67FA0` | `"shay_teresaB"` |
| `0x80B67FB0` | `"fog"` |
| `0x80B67F30` | 16-byte "small" animation block base (BSS image data) |
| `0x80B31E5C` | `f32 1.0` - weight for `0x80169F80`/`0x801680E0` |
| `0x80377C48` | resource-name object **(ext)** |
| `0x8042A038` | resource manager **(ext)** |

* **Fields:** `+0x528` model/alloc object; `+0x544` brres; `+0x548` fog chr;
  `+0x588`/`+0x5C8` chr A/B; `+0x608`/`+0x640`/`+0x678` tex-pat slots;
  `+0x6B4`/`+0x6E0`/`+0x710` tex-srt slots; `+0x73C`/`+0x774` vis slots.
* **Verdict:** **ready** (confidence high) - linear binder with all names in the
  image.

### Behaviour

```c
this = r3;
fn_800690C0(this+0x528, -1, u32[0x80377C48], 0, 0x20);
brres = fn_800DF180(u32[0x8042A038]+4, "teresa", "g3d/teresa.brres");
u32[this+0x544] = brres;

// --- A set ---
chrA = brres_get_anm_chr(brres, "teresaA");
u32[this+0x54C..] bind: fn_80169D90(this+0x588, &chrA, this+0x528, 99, 1, 0);
fn_80064BD0(this+0x588);
patA = brres_get_anm_tex_pat(brres, "teresaA");
fn_801650D0(this+0x640, &patA_pair, &patA, this+0x528, 0);
fn_801651F0(this+0x640, this+0x588, &patA, 0);
fn_80169F80(1.0f, this+0x588, this+0x640);
fn_801680E0(1.0f, this+0x640);
srtA = brres_get_anm_tex_srt(brres, "shay_teresaA");
fn_80166830(this+0x6B4, &srtA_pair, &srtA, this+0x528, 0, 1);
fn_80166BD0(this+0x6B4, this+0x588, &srtA, 0, 1);
vslot(this+0x588, 0x18)(this+0x588, this+0x6B4);
fn_80166F10(1.0f, this+0x6B4, 0);
fn_80166D10(this+0x6B4, 0);
visA = brres_get_anm_vis(brres, "shay_teresaA");
fn_80167C40(this+0x73C, &visA_pair, &visA, this+0x528, 0);
fn_80167D60(this+0x73C, this+0x588, &visA, 1);
fn_80169F80(1.0f, this+0x588, this+0x73C);
fn_801680E0(1.0f, this+0x73C);

// --- B set (same shape) ---
chrB = brres_get_anm_chr(brres, "teresaB")  -> +0x5C8
patB = brres_get_anm_tex_pat(brres,"teresaB")-> +0x678
srtB = brres_get_anm_tex_srt(brres,"shay_teresaB") -> +0x6E0
visB = brres_get_anm_vis(brres,"shay_teresaB")     -> +0x774

// --- fog set ---
chrF = brres_get_anm_chr(brres, "fog")   -> +0x548 (id 0x224)
patF = brres_get_anm_tex_pat(brres,"fog")-> +0x608
srtF = brres_get_anm_tex_srt(brres,"fog")-> +0x710
fn_800690E0(this+0x528);
```

---

## 9. `0x80A673D0` - goal-actor model binder

* **Address:** `0x80A673D0`
* **TSV size:** 1080 bytes
* **Reachable body:** 270 instructions = the whole extent; 26 direct `bl`, three
  vtable `bctrl`, **no conditional branch**, one `blr` at `0x80A67804`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a673d0_FUN_80a673d0.c`
  - exact; the first block is the `f31` spill, the rest is the binder.
* **Proposed signature:** `pub unsafe fn fn_80a673d0(this: usize)`
* **Callers in this REL:** one `bl` from `0x80A66D50`.
* **Constants:**

| address | value |
|---|---|
| `0x80B493C0` | `"g3d/goal_set.brres"` |
| `0x80B493D4` | `"goal_set"` |
| `0x80B493E0` | `"baseA"` |
| `0x80B493E8` | `"toride_std"` |
| `0x80B493F4` | `"goal_flag"` |
| `0x80B49400` | `"wait"` |
| `0x80B2D5CC` | `f32 0.0` |
| `0x80B2D5D4` | `f32 1.0` |
| `0x80B2D5E8` | `f64 2^52 + 2^31` conversion bias |
| `0x80377C48` | resource-name object **(ext)** |
| `0x8042A038` | resource manager **(ext)** |

* **Fields:** `+0x52C` model/alloc object; `+0x548` brres; `+0x54C` `baseA` chr;
  `+0x58C` `goal_flag` chr; `+0x5CC` `toride_std` chr; `+0x60C` `wait` tex-pat;
  `+0x6A4` `goal_flag` tex-pat pointer; `+0x6A8` `goal_flag` tex-srt;
  `+0x6D4`/`+0x700` tex-srt; `+0x72C` fog tex-srt; `+0x758` vis;
  `+0xAA0`/`+0xAA4` two `u32` frame counters read for the animation frame.
* **Verdict:** **ready** (confidence high).

### Behaviour

```c
this = r3;
frame = (f32)(u32[this+0xAA0]*2 + u32[this+0xAA4]);   // 2^52 conversion idiom
fn_800690C0(f64_bias, this+0x52C, -1, u32[0x80377C48], 0, 0x20);
brres = fn_800DF180(u32[0x8042A038]+4, "goal_set", "g3d/goal_set.brres");
u32[this+0x548] = brres;

chr = brres_get_anm_chr(brres, "baseA");
fn_80169D90(this+0x54C, &chr, this+0x52C, 0x20, 1, 0);
fn_80064BF0(this+0x54C);
pat = brres_get_anm_tex_pat(brres, "baseA");
fn_80166830(this+0x700, &pair, &pat, this+0x52C, 0, 1);
fn_80166BD0(this+0x700, this+0x54C, &pat, 0, 1);
vslot(this+0x54C, 0x18)(this+0x54C, this+0x700);
fn_80166EF0(frame, this+0x700, 0);
fn_80166F10(0.0f, this+0x700, 0);

chr = brres_get_anm_chr(brres, "toride_std");
fn_80169D90(this+0x5CC, &chr, this+0x52C, 99, 1, 0);
fn_80064BF0(this+0x5CC);
pat = brres_get_anm_tex_pat(brres, "toride_std");
fn_80166830(this+0x6D4, &pair, &pat, this+0x52C, 0, 1);
fn_80166BD0(this+0x6D4, this+0x5CC, &pat, 0, 1);
vslot(this+0x5CC, 0x18)(this+0x5CC, this+0x6D4);
fn_80166EF0(frame, this+0x6D4, 0);
fn_80166F10(0.0f, this+0x6D4, 0);
vis = brres_get_anm_vis(brres, "toride_std");
fn_80167C40(this+0x758, &pair, &vis, this+0x52C, 0);
fn_80167D60(this+0x758, this+0x5CC, &vis, 1);
fn_80169F80(1.0f, this+0x5CC, this+0x758);
fn_801680E0(0.0f, this+0x758);
fn_80168FE0(frame, this+0x758);

chr = brres_get_anm_chr(brres, "goal_flag");
fn_80169D90(this+0x58C, &chr, this+0x52C, 0x227, 1, 0);
fn_80064BF0(this+0x58C);
fn_8016A000(this+0x58C, this+0x524);
pat = brres_get_anm_tex_pat(brres, "wait");
fn_801650D0(this+0x60C, &pair, &pat, this+0x52C, 0);
fn_801651F0(this+0x60C, this+0x58C, &pat, 0);
fn_80169F80(1.0f, this+0x58C, this+0x60C);
u32[this+0x6A4] = brres_get_anm_tex_pat(brres, "goal_flag");
fn_80166830(this+0x6A8, &pair, &u32[this+0x6A4], this+0x52C, 0, 1);
fn_80166BD0(this+0x6A8, this+0x58C, &u32[this+0x6A4], 0, 1);
vslot(this+0x58C, 0x18)(this+0x58C, this+0x6A8);
fn_80166EF0(frame, this+0x6A8, 0);
fn_80166F10(0.0f, this+0x6A8, 0);
texsrt = brres_get_anm_tex_srt(brres, "wait");
fn_80167420(this+0x72C, &pair, &texsrt, this+0x52C, 0, 1);
fn_80167800(this+0x72C, this+0x58C, &texsrt, 0, 0);
fn_80169F80(1.0f, this+0x58C, this+0x72C);
fn_800690E0(this+0x52C);
```

---

## 10. `0x80A85650` - one-shot shot resolver

* **Address:** `0x80A85650`
* **TSV size:** 976 bytes
* **Reachable body:** 244 instructions = the whole extent; 12 direct `bl`, three
  vtable `bctrl`, 32 conditional branches, 17 `b` joins, one `blr` at
  `0x80A85A1C`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a85650_FUN_80a85650.c`
  - usable; the `func_0x8005f900()`/`func_0x8004dd00()` "no argument" prints are
  artefacts of the guard block (the stream stages the slot in `r3` at
  `0x80A85680`).
* **Proposed signature:** `pub unsafe fn fn_80a85650(this: usize) -> u32`
  - returns `1` when the shot was consumed, `0` otherwise.
* **Callers in this REL:** 16 `bl` sites: `0x80A86AC8`, `0x80A86D90`,
  `0x80A86DEC`, `0x80A872CC`, `0x80A87560`, `0x80A8779C`, `0x80A8793C`,
  `0x80A87AEC`, `0x80A87B78`, `0x80A87D70`, `0x80A88188`, `0x80A88378`,
  `0x80A88868`, `0x80A88D9C`, `0x80A8904C`, `0x80A89064` - all inside the
  `0x80A86xxx` state bodies, which set `+0xBEC` and `+0xDFF` first.
* **Constants:** no image constants; the body uses global managers and the SE /
  message ids `0x116`, `0x294`, `4`, `8`.

* **Fields:** `+0xBEC` u32 request word (non-zero = armed); `+0xDFF` s8 player
  slot (-1 = none); `+0xDCA` u16 state (0..14); `+0x14E0` u8 player kind;
  `+0x348` u8 side; `+0x60` vtable; `+0xAC/B0/B4` position;
  `+0xAE8` (player) and `+0x1540` (player) read through helpers.
* **Verdict:** **ready** (confidence high) - the body is a finite state table
  with all branches explicit; the only external contracts are the documented
  `0x8005F900`/`0x8004DD00`/`0x8005FB90` player helpers.

### Behaviour

```c
if (+0xBEC == 0 || s8[+0xDFF] == -1) return 0;
player = fn_8005F900(s8[+0xDFF]);
dead   = fn_8004DD00(player) != 0;
if (dead) { +0xBEC = 0; +0xDFF = -1; return 0; }
if (+0xDCA == 6) {
    fn_80a863d0(this);
    vslot(player+0x60, 0x418)(player);
    fn_800E24B0(u32[0x8042A050], 8, s8[+0xDFF], 1);
    fn_80a855a0(this); fn_80162510(this);
    return 1;
}
if (vslot(player+0x60, 0xE0)(player) != 0) { +0xBEC = 0; +0xDFF = -1; return 0; }
if (+0xDCA == 2) {
    fn_80a863d0(this);
    target = fn_8005FB90(s8[+0xDFF]);
    if (target) vslot(target+0x60, 0x3D8)(target, 0, 0x294);
    fn_800E24B0(u32[0x8042A050], 4, s8[+0xDFF], 1);
    fn_800A7860(u32[0x80429F10], s8[+0xDFF]);
    fn_80a855a0(this); fn_80162510(this);
    return 1;
}
// state table: (state -> required kind, clip index)
kind = s8[u32[player+0x14E0]];
idx = 6; hit = 0;
switch (+0xDCA) {
case 0x0: if (kind==1 || kind==2 || (kind-4)<3) { idx=0; hit=2; } else hit = fn_80145AC0(player,1)!=0; break;
case 0x1: if (kind==2) { idx=1; hit=2; } else hit = fn_80145AC0(player,2)!=0; break;
case 0x4: if (kind==6) { idx=5; hit=2; } else hit = fn_80145AC0(player,6)!=0; break;
case 0xB: if (kind==4) { idx=2; hit=2; } else hit = fn_80145AC0(player,4)!=0; break;
case 0xD: if (kind==3) { idx=4; hit=2; } else hit = fn_80145AC0(player,3)!=0; break;
case 0xE: if (kind==5) { idx=3; hit=2; } else hit = fn_80145AC0(player,5)!=0; break;
default: break;                                  // hit stays 0
}
if (hit == 0) return 0;
if (hit == 2 && idx != 6) {
    fn_80a863d0(this);
    fn_8006A4B0(&pos, this+0xAC);
    sound_play_3d(u32[0x80429D60], 0x116, &pos, 0);
}
if (hit == 1) { fn_80a86400(this); fn_800A7860(u32[0x80429F10], s8[+0xDFF]); }
fn_800E24B0(u32[0x8042A050], 4, s8[+0xDFF], 1);
fn_80a855a0(this); fn_80162510(this);
return 1;
```

* **Notes:** the `+0xDCA` values are the parent state machine's state ids; the
  `(state -> kind, idx)` table above is the complete set of arms in the stream
  (`state 3/5/6..10/12` fall through to `hit = 0`). The `0x294` passed to the
  player's vtable slot `+0x3D8` is the damage/animation selector.

---

## 11. `0x80AA0C20` - actor step with clip-selection ladder

* **Address:** `0x80AA0C20`
* **TSV size:** 872 bytes
* **Reachable body:** 218 instructions = the whole extent; 13 direct `bl`, 13
  vtable `bctrl`, 21 conditional branches, five `b` joins, one `blr` at
  `0x80AA0F84`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aa0c20_FUN_80aa0c20.c`
  - exact; the tail four `if` blocks are the clip-selection ladder.
* **Proposed signature:** `pub unsafe fn fn_80aa0c20(this: usize)`
* **Callers in this REL:** none by `bl`; reached from the actor state table.
* **Constants:**

| address | type | value / role |
|---|---|---|
| `0x80B2F2F4` | `f32` | `0.0` - `f31` seed |
| `0x80B2F354` | `f32` | `16.0` - "too far" threshold |
| `0x80B2F380` | `f32` | `55.0` - registry-value comparison |
| `0x80B2F384` | `f32` | `112.0` - "drop" threshold |
| `0x80B74638` | BSS name block **(ext)** | `+0x10`, `+0x50`, `+0x90`, `+0x150` clip objects |
| `0x80B74988` | name object **(ext, BSS)** | query argument for the `+0x394` vtable |
| `0x80429D5C` | sound manager **(ext)** | |
| `0x80429D5C`-side SE ids | `470`, `467` | fired at `0x80197F00` |

* **Fields:** `+0x60` vtable; `+0xAC/B0/B4` position; `+0x124` u8 (cleared on
  the drop branch); `+0x17C` u32 (bit 2 cleared); `+0x348` u8 facing;
  `+0x394` sub-object with vtable slots `+0x28` and `+0x14`; `+0x54C` sub-object
  with slot `+0x1C`; `+0x70C` registry handle; `+0x844` animation object with
  vtable slot `+0x10`; `+0x848`/`+0x854` state words; `+0x870`/`+0x872` u16
  gates.
* **Verdict:** **conditional** (confidence medium) - the body is exact, but the
  four clip objects at `0x80B74648`, `0x80B74688`, `0x80B746C8`, `0x80B74788`
  are BSS slots whose contents are filled by the actor's constructor (outside
  this body), and the `+0x844` object's `+0x10` slot is an unledgered
  `resource::registry` query. The behaviour below is the exact call sequence;
  only the *names* those four slots hold are external.

### Behaviour

```c
this = r3;
fn_8006cde0(this);
fn_800955f0(this);
fn_80a9ff20(this);                 // contact check
target = fn_80aa0340(this);        // resolved target record, 0 if none
dy = 0.0f;
t27 = 0;
if (target != 0) {
    dy = f32[this+0xB0] - f32[target+0xB0];
    v = vslot(target+0x394, 0x28)(target+0x394);
    t27 = vslot(*v, 0x14)(v, 0x80B74988) != 0;
    if (t27 && +0x854 == 0) {
        if (+0x872 == 0) {
            collision_area_register(this+0x148);
            +0x848 = 1;
            +0x17C &= ~4;
        }
        +0x854 = 1;
    }
}
arm = (dy >= 16.0f) ||
      (t27 == 0 && fn_80168080(this+0x70C) == 55.0f);
if (!arm && +0x870 == 0) {                  // "not close enough": start the SE
    +0x870 = 1;
    fn_8006A4B0(&pos, this+0xAC);
    sound_play(0x80429D5C, +0x872 == 1 ? 470 : 467, &pos, 0);
    fn_80a9fd50(this, 2);
}
if (fn_80168080(this+0x70C) >= 112.0f && target != 0) {   // cror eq,gt,eq
    u8[target+0x124] = 0;
    +0x848 = 0;
}
if (+0x870 != 0 && fn_80169020(this+0x70C) != 0) {
    obj = +0x844;
    if (vslot(obj, 0x10)(obj, 0x80B74688) != 0) {
        vslot(this+0x60, 0xD4)(this, u32[this+0x844]);
    } else if (vslot(obj, 0x10)(obj, 0x80B746C8) != 0) {
        side = fn_80096240(this, this+0xAC);
        vslot(this+0x60, 0xD4)(this, side == u8[this+0x348] ? 0x80B74648 : 0x80B74688);
    } else {
        side = fn_80096240(this, this+0xAC);
        if (side == u8[this+0x348]) {
            if (vslot(obj, 0x10)(obj, 0x80B74788) != 0)
                vslot(this+0x60, 0xD4)(this, 0x80B74648);
            else
                vslot(this+0x60, 0xD4)(this, u32[this+0x844]);
        } else {
            vslot(this+0x60, 0xD4)(this, 0x80B74688);
        }
    }
}
if (!arm && +0x870 != 0) vslot(this+0x54C, 0x1C)(this+0x54C);
```

---

## 12. `0x80AE2250` - `qr3` wing step

* **Address:** `0x80AE2250`
* **TSV size:** 108 bytes
* **Reachable body:** 27 instructions = the whole extent; one direct `bl`
  (`0x80237BD0`), no branch, one `blr` at `0x80AE22B8`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ae2250_FUN_80ae2250.c`
  - the export prints `unaff_GQR3`/`ldexpf`; that is the single `qr3` load below
    and is resolved by `GQR3 = 0x00050005`.
* **Proposed signature:** `pub unsafe fn fn_80ae2250(this: usize)`
* **Callers in this REL:** one `bl` from `0x80AE2950` (the `0x80AE29xx` step
  body, not in `functions.tsv`).
* **Constants:**

| address | type | value / role |
|---|---|---|
| `0x80B30DA4` | `f32` | `0.00390625` (1/256) - angle pre-scale |
| `0x80B30DA8` | `f32` | `1.5` - vertical amplitude |

* **Fields:** `+0xAA8` s16 angle (turn units); `+0x8B0` f32 base height;
  `+0xB0` f32 y.
* **Verdict:** **ready** (confidence high) - the only non-scalar instruction is
  the `qr3` halfword load, which is exactly `(f32)(u16)field`.

### Frame

`stwu r1,-32(r1)`; LR at `+0x24`; `r31` saved at `+0x1C` and holds `this`.
No FPR is saved: the export's `unaff_GQR3` block is the single `qr3` load
below, not a spill.

### Behaviour

```c
this = r3;
angle = s16[this+0xAA8];
half  = (u16)angle;                      // sth + psq_l ...,qr3: zero-extends
ph    = (f32)half * (1.0f/256.0f);
s     = sin_angle16(ph);                 // 0x80237BD0
s16[this+0xAA8] = (s16)(angle + 0xC00);  // +3/16 turn per frame
f32[this+0xB0]  = f32[this+0x8B0] + 1.5f * s;
```

The `lha`/`sth` round-trip means the angle is truncated to 16 bits each frame;
`sin_angle16` is periodic in `256`, so the wrap is exact.

---

## Verification

* 202 untranslated addresses remain after removing all 16,815 ledger rows, all
  279 `0x80A`/`0x80B` addresses that appear in `docs/*.md` headings and all
  3,270 `fn_<addr>` tokens in `src/**/*.rs`; the twelve rows above are the
  first twelve of the stated order.
* Every row has a Ghidra export at
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
* Reachable-instruction count equals `size / 4` for all twelve rows
  (`0x80A3DEA0` after seeding its 16-case table at `0x80B3EE94`); no row
  borrows a neighbour's body and no reachable path lands on data.
* No body reads a `GQRn` other than `qr0` (prologue spills) or `qr3`
  (`0x80AE2250`), and no body takes an incoming float argument.
* The three conditional verdicts name their one external fact: the slot
  convention of `0x80A9B3C0`, the three unledgered `main.dol` helpers of
  `0x80A92F40`, and the four BSS clip objects of `0x80AA0C20`.

