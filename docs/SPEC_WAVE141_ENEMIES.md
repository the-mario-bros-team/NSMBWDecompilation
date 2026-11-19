# Wave 141 - implementation specification for 12 class-D `d_enemiesNP` bodies

## What this page is

Wave 127 (`docs/ANALYSIS_WAVE127_ENEMIES.md`) reclassified 384 formerly-skipped
`d_enemiesNP` addresses into 173 class **B**, 160 class C and 51 class D. Waves
128-140 (`docs/SPEC_WAVE128_ENEMIES.md` .. `docs/SPEC_WAVE140_ENEMIES.md`)
specified 144 of them, and the implementation lanes drained those plus a run of
rows that later waves pulled out of the D bucket once `GQR3` was pinned.

Wave 140 (`docs/SPEC_WAVE140_ENEMIES.md:15-18`) opened the qr3 tranche: the
class-D rule "a `psq_l/psq_st` whose quantiser is `GQR1..GQR7`" stopped applying
to `qr3` once `docs/REL_ELF_MAPPING.md` pinned **`GQR3 = 0x00050005`** (unsigned
16-bit, scale 0). This lane is the continuation of that tranche: the next twelve
largest `d_enemiesNP` addresses that are class D **only** because of a
`psq_l ...,qr3` u16 load.

This page is **read-only**: it writes no Rust, no ledger row and no other doc.

Each body was decoded from `work/rels/elf/d_enemiesNP.elf` with the `+0x1000`
segment shift of `docs/REL_ELF_MAPPING.md` (first `PT_LOAD` is
`p_offset = 0x10F0`, `p_vaddr = 0x80A000F0`), instruction by instruction from the
entry, following every conditional branch and every linked call until each path
ended in `blr`, a tail `b` or a tail call. Each body was cross-checked against
the Ghidra export at the cited path and against the `size` column of
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`. Constants that live inside
the REL image were read out of the ELF itself, so the integer and float values
below are the shipped values. Addresses outside the file-backed image are marked
**(ext)**; helper addresses below `0x80A00000` are `main.dol` functions.

## Selection

Ordering is by `functions.tsv` size, largest first; the reachable-instruction
count of every row agrees with that order (the two metrics pick the same
twelve). Every row below is absent from the `address` column of
`ledger/functions.csv` (checked against all rows, read-only), absent from every
heading of every `docs/*.md`, and has no `fn_<addr>` body in `src/`.

| # | address | TSV size | reachable insns | role | verdict |
|---:|---|---:|---:|---|---|
| 1 | `0x80A0E580` | 968 | 242 | `daEnBeansKuribo_c` orbit/hop step around the player | **ready** |
| 2 | `0x80B0C540` | 848 | 212 | two-node matrix commit with a 24-unit drop and a spin probe | **ready** |
| 3 | `0x80AEB450` | 700 | 175 | `daEnPukuPukuParent_c::StateID_Move` chain-of-segments solver | **ready** |
| 4 | `0x80AE4A50` | 660 | 165 | `daEnPolter_c::StateID_Wait` hover/steer step | **ready** |
| 5 | `0x80A4B740` | 604 | 151 | `daEnCrow_c` circle-to-target steer with a lateral give-up test | **ready** |
| 6 | `0x80A4B420` | 548 | 137 | `daEnCrow_c` dive: pitch clamp, landing probe, message set | **ready** |
| 7 | `0x80A635D0` | 488 | 122 | `daEnGesso_c` two-phase bob with a pad-query message fan-out | **ready** |
| 8 | `0x80A63FF0` | 392 | 98 | `daEnGesso_c` drifting jet with a decelerating coast | **ready** |
| 9 | `0x80A641F0` | 380 | 95 | `daEnGesso_c` coast stop with a deceleration floor | **ready** |
| 10 | `0x80ABA080` | 308 | 77 | hit-response splash: directional impulse, sound, vtable call | **ready** |
| 11 | `0x80A4BBD0` | 304 | 76 | `daEnCrow_c` climb and level-off | **ready** |
| 12 | `0x80A46200` | 300 | 75 | vertical ping-pong platform step (group-3 actor) | **ready** |

**Split: 12 ready, 0 conditional.** Total 6,500 bytes of TSV extent and 1,625
reachable instructions.

No row needed a caveat: every one of the twelve walks to `blr` with every
branch target inside its own extent, and the extent is filled exactly (the
reachable word count equals `size / 4` for all twelve). The only data that is
not in this image is used as an opaque argument - a message pointer, a vtable
slot or a `main.dol` float - and never selects a path or enters the arithmetic.
The `qr3` loads are the single blocker the wave-127 table recorded, and it is
now gone.

## GQR3 and the qr3 pool (state at this wave)

`docs/REL_ELF_MAPPING.md` pins **`GQR3 = 0x00050005`** (unsigned 16-bit, scale
0), installed by `main.dol` at `0x8025EEEC`/`0x8025EEF0`
(`li r3,5` / `oris r3,r3,5` / `mtspr 915,r3`). The RAW stream in this REL holds
no `mtspr GQRn`, so every `psq_*` in this module depends on a `main.dol` word.

Every data `psq` in the twelve bodies below is the same instruction shape:

```
psq_l fD,0(rA),1,qr3     ; word = 0xE0<D><A>B000: DQ = 0, W = 1, qr = 3
```

which with `GQR3 = 0x00050005` is **one unsigned 16-bit load scaled by `2^0`**:

```
fD = (f32)*(u16 *)(rA)
```

The idiom is always "stage the halfword on the stack, then load it":

```
sth   r0,10(r1)
addi  r3,r1,10
psq_l f1,0(r3),1,qr3      ; f1 = (f32)(u16)r0
lfs   f0,..(r31)          ; 1/256
fmuls f1,f0,f1            ; -> 16-bit turn units scaled to the sin/cos argument
bl    0x80237BD0          ; sin_angle16 / 0x80237C40 cos_angle16
```

A Rust port reads the `u16` field directly and widens it; no paired-single
traffic is needed. The `psq_st fN,..,qr0` prologue saves (where present) are
plain `stfd` spills and stay as they are (`docs/SPEC_WAVE140_ENEMIES.md:85-86`).

Counts, read off the twelve extents (`psq_l ...,qr3` sites / total `psq`):

| address | `qr3` sites | `qr0` spill pairs |
|---|---:|---:|
| `0x80A0E580` | 2 | 0 |
| `0x80B0C540` | 1 | 0 |
| `0x80AEB450` | 3 | 6 |
| `0x80AE4A50` | 3 | 0 |
| `0x80A4B740` | 2 | 0 |
| `0x80A4B420` | 2 | 0 |
| `0x80A635D0` | 1 | 0 |
| `0x80A63FF0` | 2 | 1 |
| `0x80A641F0` | 2 | 1 |
| `0x80ABA080` | 2 | 2 |
| `0x80A4BBD0` | 2 | 0 |
| `0x80A46200` | 1 | 1 |

## Reading conventions

* Synthetic base is `0x80A00000`. ELF file offset is
  `(address - 0x80A00000) + 0x1000`.
* The REL's file-backed image ends at `0x80B6E4B8`; anything above that is a
  `main.dol` address and is marked **(ext)**.
* `this` is the incoming `r3` unless stated otherwise; `+0xNNN` are byte
  offsets from `this`. `+0xAC`/`+0xB0`/`+0xB4` are x/y/z in every body below
  (the module's actor convention).
* `addi rX,r0,imm` is `li rX,imm`, and `addis rX,r0,imm` is `lis rX,imm`
  (`r0` is not read when `RA == 0`). `or rX,rY,rY` is `mr rX,rY`.
* `andi.` is opcode 28, `cmpi` is **11**, `cmpli` is **10**, `rlwinm` is **21**,
  `rlwimi` is **20**, `rlwnm` is **23**. `fcmpu` is 63/XO 0, `fcmpo` 63/XO 32.
* On a `bc`/`bclr`, `bo = 12` branches on a true CR bit and `bo = 4` on a false
  one; `bclr bo=20 bi=0` is `blr`; `bcctr bo=20 bi=0` with `LK` is `bctrl`.
* `cror` after an `fcmpo` rewrites `cr0.eq`: `cror 2,0,2` gives `le`
  (`lt|eq`) and `cror 2,1,2` gives `ge` (`gt|eq`). A following `bne` therefore
  reads `if !(a <= b)` / `if !(a >= b)`. Both idioms appear below and are
  written in the C form that results.
* The paired-single opcodes in this build are `psq_l = 56` and `psq_st = 60`
  (`docs/REL_ELF_MAPPING.md`); the quantiser is bits 12-14, `W` is bit 15.
* A-form float ops take their third operand from `fC` (bits 6-10):
  `fmuls fD,fA,fC`, `fmadds fD,fA,fC,fB`. That is why the `1/256 * angle`
  pre-scale reads `fmuls f1,f0,f1` in the stream: `f0 = 1/256`, `f1 = angle`.
* `0x80237C40` is `cos_angle16(f1)` and `0x80237BD0` is `sin_angle16(f1)`
  (`docs/SPEC_WAVE140_ENEMIES.md:116-117`); the angle is in 16-bit turn units
  and the pre-scale is `*(1/256)`, i.e. the helpers take a `0..255` "degree".
* TSV `size` is a bucket extent. All twelve bodies below fill their extent
  exactly.
* In the per-section constants tables, `offset` is relative to the table base
  named on that section's **Constants** line (an `lis`/`addi` pair in the
  stream), never to `this`.

## Symbolic helpers used below

Roles are the ledger's (`ledger/functions.csv`) or, where the helper is
unledgered, the role established by `docs/SPEC_WAVE129/130/140_ENEMIES.md`.

| address | name / shape | role |
|---|---|---|
| `0x8005F900` | `player_by_slot(slot: u32) -> usize` | ledgered `game::player`; table-A resolver, `0` for an empty slot |
| `0x80056CF0` | `player_has(player, kind: u32) -> u32` | ledgered `game::player`; reads `+0x1014 + 4*(kind>>5)` |
| `0x80057CC0` | `player_scratch(player) -> u32` | ledgered `game::player`; returns the `+0x1088` word |
| `0x80057C40` | `player_scratch_clear(player)` | `main.dol`; twin of `0x80057CC0` (result unused here) |
| `0x8005E780` | `player_flag_17(player) -> u32` | ledgered `game::player`; single byte load at `+0x17` |
| `0x8013AAA0` | `worldmap_query(player) -> u32` | ledgered `game::worldmap`; the second player-state predicate of rank 1 |
| `0x8006A4B0` | `pos_to_vect(r3 out: *mut [f32;3], r4 pos: *const f32[3])` | `main.dol`; copies a 3-float position (wave 130 helper table) |
| `0x80064AF0` | `pos_euler_to_vect(r3 out, r4 ang, r5 mode)` | `main.dol`; position + Euler to vector (wave 129/130) |
| `0x8006CD40` | `base_actor_update(this)` | `main.dol`; first half of the base actor update (wave 130:93) |
| `0x8006CED0` | `actor_world_pos(r3 out: *mut [f32;3], r4 actor)` | `main.dol`; fills `out` with the actor position |
| `0x801C0BD0` | `set_translation(r3 matrix, f1 x, f2 y, f3 z)` | ledgered `runtime::dvd`; 3x4 translation builder |
| `0x8016A170` | `holder_copy(holder, src)` | ledgered `resource::registry`; copy into a matrix holder |
| `0x8016A150` | `holder_offset(holder, src)` | ledgered `resource::registry`; offset a holder |
| `0x8016A1A0` | `holder_clear(holder, flag)` | ledgered `resource::registry`; clear a holder flag |
| `0x80168400` | `holder_flush(holder)` | ledgered `resource::registry`; matrix flush |
| `0x80168180` | `holder_row(holder, row: u32, out)` | ledgered `resource::registry`; matrix row reader |
| `0x8016EDD0` / `0x8016ECB0` / `0x8016EEF0` | `holder_yaw` / `holder_pitch` / `holder_roll(holder, *u16)` | ledgered `resource::registry`; set a matrix axis from a halfword |
| `0x8015F470` | `step_float_clamped(f1 target: f64, f2 step: f64, r3 value: *mut f32) -> u32` | ledgered `engine::display` thunk; `1` on arrival |
| `0x8015F4F0` | `step_short_clamped(value: *mut i16, target: i32, step: i32) -> u32` | ledgered `game::task`; `1` on arrival |
| `0x8015F200` | `step_angle_toward(value: *mut i16, target, step, limit, limit2) -> u32` | ledgered `game::task`; `1` on arrival |
| `0x8015F2E0` | `step_angle_4(value: *mut i16, target, step, limit)` | `main.dol`; the 4-argument form of the same family |
| `0x801611C0` | `atan2_16(f1 y: f64, f2 x: f64) -> i16` | ledgered `resource::registry`; eight-octant `atan2` in turn units |
| `0x80237BD0` | `sin_angle16(f1) -> f32` | ledgered `resource::bres`; sine through `0x802F74A4` |
| `0x80237C40` | `cos_angle16(f1) -> f32` | ledgered `resource::bres`; cosine through `0x802F74A0` |
| `0x801680E0` | `set_anim_rate(holder, f1 rate)` | ledgered `resource::registry`; vtable slot `+0x28` |
| `0x80169020` | `test_anim_holder(holder) -> u32` | ledgered `resource::registry`; slots `0x20`/`0x2C` vs `+0x2C`/`+0x28` |
| `0x80197F00` | `se_play_pos(r3 system, r4 cue: u32, r5 pos, r6 flag)` | ledgered `audio::ax`; position-argument cue |
| `0x80198660` | `se_play_actor(r3 system, r4 cue, r5 owner, r6 pos, r7 0)` | ledgered `audio::ax`; owner + position cue |
| `0x80198C30` | `se_play_pos_2(r3 system, r4 cue: u32, r5 pos, r6 flag)` | ledgered `audio::ax`; the `0x52B0/0x52AC` level pair |
| `0x80076DC0` | `stage_kind_claim(r3 manager, r4 owner, r5 word, r6 u16, r7 s8) -> u32` | ledgered `game::stage`; 100-entry kind scan, ordinal cached at `+0x1F8` |
| `0x80076FB0` | `stage_kind_release(r3 manager, r4 u16 id)` | `main.dol`; the release twin of `0x80076DC0` |
| `0x800E20A0` | `pose_notify(r3 manager, r4 actor, r5 anim, f1, f2, r6 slot)` | ledgered `engine::dsys`; notification pose push |
| `0x8009F630` | `stage_slot_word() -> u32` | ledgered `game::stage`; `+0x60`/`+0x3EC` slot read |
| `0x8009F710` | `anim_bind(r3 holder, r4 index)` | ledgered `game::stage`; kind 1/2 tables, index clamped |
| `0x802DCD70` / `0x802DCD74` / `0x802DCDBC` / `0x802DCDC0` | `_savegpr_26` / `_savegprlr_27` / `_restgpr_26` / `_restgprlr_27` | ledgered `runtime::cpp`; CodeWarrior register save/restore, **not** argument-destroying stubs |
| `0x80A0DCC0` | `side_byte_update(this, player)` | ledgered `game::enemies` (translated); side byte table `0x8042C1B0` |
| `0x80A0DEF0` | `frame_commit(this)` | ledgered `game::enemies` (translated); vtable `+0x1C` commit |
| `0x80A0DF00` | `button_predicates(this, player) -> u32` | ledgered `game::enemies` (translated); six predicates |
| `0x80AE4410` | `facing_mirror_step(this, r4 delta: *const f32[3], r5 bits: u32) -> i32` | ledgered `game::enemies`; mirrors x/z by `+0x348`, yaw via `0x801611C0` |
| `0x80AE4380` | `ramp_flip(this)` | ledgered `game::enemies`; calls `0x80AE4300(actor,+0x6C4,0x100,0x10)`, flips `+0x6C4` |
| `0x80A61B20` | `gesso_phase_step(this)` | ledgered `game::enemies`; `+0x737` countdown, else `+0x102 += 0x800` |
| `0x80A61B60` | `gesso_phase_timer(this)` | ledgered `game::enemies`; reloads `+0x737 = 0x20` when idle |
| `0x80A46180` | `ramp_apply(this)` | ledgered `game::enemies`; applies ramp to `+0x104` by direction `+0xAD2` |

---

## 1. `0x80A0E580` - `daEnBeansKuribo_c` orbit/hop step

* **Address:** `0x80A0E580`
* **TSV size:** 968 bytes
* **Reachable body:** 242 instructions, 242 words = the whole extent; 21 direct
  `bl`, 2 `bctrl` (both on the player's vtable), one `blr` at `0x80A0E944`.
  Every branch target is inside the extent, so the extent is exact.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a0e580_FUN_80a0e580.c`
  - the export is a `GQR0`/`ldexpf` mess; the RAW stream has **no** `psq_st`
    at all, only the two `qr3` loads at `0x80A0E6B0` and `0x80A0E6E0`.
* **Proposed signature:** `pub unsafe fn fn_80a0e580(this: usize)`
* **Callers in this REL:** no `bl` and no branch; one 32-bit word at
  `0x80B350FC`, the last record of the `daEnBeansKuribo_c` state-handler array
  (neighbours `0x80A0E570`, `0x80A0E370`; the class name block is
  `0x80B35130`..`0x80B35194`, Walk/Turn/Around/DieFall).
* **Constants:** from `0x80B2ACB8` (`lis r29,0x80b3` +
  `addi r29,r29,-21320`):

| address | type | value / role |
|---|---|---|
| `0x80B2AD10` | f32 | `4.0` - subtracted from the probe point's y (`probe.y - 4.0`) |
| `0x80B2AD18` | f32 | `0.0` - the hop floor |
| `0x80B2AD64` | f32 | `0.00390625` = `1/256`, turn-units pre-scale |
| `0x80B2AD68` | f32 | `8.0` - orbit radius |

* **Fields:** `+0x000` owner word; `+0x060` vtable; `+0x0AC/0x0B0/0x0B4` x/y/z;
  `+0x100/0x102/0x104` `s16` angles; `+0x348` `u8` side byte; `+0x510` animation
  holder; `+0x5E8` f32 hop height; `+0x5EC` f32 hop velocity; `+0x5F4` u32
  counter; `+0x610` u16 pad id (`0xFFFF` = none); `+0x612` u16 orbit angle;
  `+0x614` `s8` player slot; `+0x615` `u8` hop-direction flip.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-96(r1)`; `vrsave` saved at `+0x64`; `_savegpr_26` (`bl 0x802DCD70`,
r26-r31 at `r11-0x18..-4` with `r11 = r1+0x60`); no FPR save (the body only
uses `f0`, `f1`, `f2`). Epilogue: `_restgpr_26` (`bl 0x802DCDBC`), `vrsave`,
`addi r1,r1,96`, `blr`.

### Behaviour

```
owner  = u32[this+0x000]
player = player_by_slot((s8)u8[this+0x614])

# 1. hop integrator + target height
hop = f32[this+0x5E8]
vel = f32[this+0x5EC]
f32[this+0x5E8] = hop + vel                      # 0x80A0E5C0/C5CC
probe = player_vtable[0x3C0](player)             # returns a 3-float point
px, py, pz = f32[player+0xAC], f32[player+0xB0], f32[player+0xB4]
qx, qy, qz = f32[probe+0], f32[probe+4] - 4.0, f32[probe+8]
dy = fabsf(qy - py)                              # 0x80A0E604..E618

if (f32[this+0x5E8] >= dy) {                     # ceiling reached
    f32[this+0x5E8] = dy
    u8[this+0x615] ^= 1
    f32[this+0x5EC] = -f32[this+0x5EC]
} else if (f32[this+0x5E8] < 0.0) {              # floor reached
    f32[this+0x5E8] = 0.0
    u8[this+0x615] ^= 1
    f32[this+0x5EC] = -f32[this+0x5EC]
}

# 2. orbit around the player
u16[this+0x612] += 0x400                          # 5.625 deg/frame
i16[this+0x102] += 0x400
a  = (f32)(u16)u16[this+0x612] * (1.0/256.0)
f32[this+0x0AC] = px + 8.0 * cos_angle16(a)
f32[this+0x0B0] = py + f32[this+0x5E8]
f32[this+0x0B4] = pz + 8.0 * sin_angle16(a)

# 3. contact / pad bookkeeping
c    = player_scratch(player)                     # 0x80057CC0
mode = (c >= 4 && c <= 8) ? 2 : (c > 8 ? 3 : 1)   # 0x80A0E718..E738
player_scratch_clear(player)                      # 0x80057C40
pos  = {f32[this+0x0AC], f32[this+0x0B0], f32[this+0x0B4]}
se_play_actor(*0x80439D5C, 508, owner, &pos, 0)   # 0x80198660, cue 0x1FC

flags = 0
if player_has(player, 4)             flags = 3
if worldmap_query(player)            { if player_has(player, 43) flags = 1 }
if player_has(player, 29)            flags = 1
if (player_flag_17(player+0xEA4) == 0 && fn_80a0df00(this, player) == 0)
    goto tail                                    # 0x80A0E7D8/E7EC

if (stage_kind_claim(*0x80439DC0, owner, c,
                     u16[this+0x610], (s8)u8[this+0x614]) != 0) {
    flags = 1
} else {
    u32[this+0x5F4] += 1
    if ((s32)u32[this+0x5F4] >= mode) flags = 1
}

tail:
if (player_vtable[0x3D4](player) != 0) flags = 2
if (flags == 0) { fn_80a0def0(this); return }

u8[this+0x348] = (f32[this+0x0AC] >= px) ? 0 : 1   # 0x80A0E86C..E884
fn_80a0dcc0(this, player)
if (flags == 2) {
    anim = anim_bind(this+0x510, stage_slot_word())      # 0x8009F710
    pose_notify(*0x8043A050, this, anim, f1 = *0x8043CC88,
                f2 = *0x8043CC8C, (s8)u8[this+0x614])    # 0x800E20A0
}
pos_to_vect(&local16, this+0xAC)
se_play_pos(*0x80439D5C, 506, &local16, 0)               # 0x80197F00, cue 0x1FA
if (u16[this+0x610] != 0xFFFF)
    stage_kind_release(*0x80439DC0, u16[this+0x610])
u16[this+0x610] = 0xFFFF
fn_80a0def0(this)
```

### Notes

* `0x80439D5C`, `0x80439DC0`, `0x8043A050`, `0x8043CC88`, `0x8043CC8C` are
  `main.dol` globals **(ext)**; they are passed through, never tested.
* `player_vtable` is `u32[player+0x60]`; the slots used are `0x3C0` (returns a
  3-float probe point) and `0x3D4` (returns a state word). Both are called with
  `r3 = player`.
* The hop fields `+0x5E8`/`+0x5EC` are shared with the already-translated
  sibling `fn_80a0e450` (`src/game/enemies.rs:168271`), which seeds
  `+0x5E8 = |dy|` and `+0x5EC = 0.15` (`0x80B2AD60`). This body is the per-frame
  ping-pong over those two fields.
* The `0x8005F900` index is the sign-extended `s8` at `+0x614`, so an empty
  slot yields the null player and the two vtable calls dereference `+0x60` of
  that null - the retail code assumes the slot is live (same assumption as
  `fn_80a0e450`).

---

## 2. `0x80B0C540` - two-node matrix commit with a 24-unit drop

* **Address:** `0x80B0C540`
* **TSV size:** 848 bytes
* **Reachable body:** 212 instructions = the whole extent; 11 direct `bl`,
  2 `bctrl` on the `+0x394` node, one `blr` at `0x80B0C88C`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b0c540_FUN_80b0c540.c`
  - the only `psq` in the stream is the single `qr3` load at `0x80B0C61C`; the
    export's paired-single block is cosmetic.
* **Proposed signature:** `pub unsafe fn fn_80b0c540(this: usize)`
* **Callers in this REL:** `bl` from `0x80B0BF0C`, inside the class frame
  update whose vtable block is `0x80B68050` (the update itself is
  `0x80B0BE80`, referenced at `0x80B68068`). The caller then reads `+0x9D0`
  and, when it is zero, writes `f32[this+0xDC] = 1.0` (`0x80B31DD4`).
* **Constants:** from `0x80B31DB8` (`lis r31,0x80b3` +
  `addi r31,r31,7608`):

| address | type | value / role |
|---|---|---|
| `0x80B31E50` | f32 | `0.00390625` = `1/256`, turn-units pre-scale |
| `0x80B31E54` | f32 | `3.0` - y displacement amplitude |
| `0x80B31E58` | f32 | `24.0` - vertical drop of the second node |
| `0x80B88988`, `0x80B889C8` | ptr | **(ext)** node-probe arguments |

* **Fields:** `+0x060` vtable; `+0x07C` primary matrix; `+0x0AC/0x0B0/0x0B4`
  x/y/z; `+0x100/0x102/0x104` `s16` angles; `+0x0DC` holder offset; `+0x0E4`
  f32 scale; `+0x394` sub-node (its own vtable at `+0x000`, slot `0x28`);
  `+0x548`, `+0x588`, `+0x5C8` matrix holders; `+0x988/0x98C/0x990` and
  `+0x9A0/0x9A4/0x9A8` f32 triples; `+0x9C8` u32 flag (render second node);
  `+0x9D0` u32 flag (scale the drop); `+0x9DC` `s16` spin angle.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-128(r1)`; `vrsave` at `+0x84`; `stw r31,124` and `stw r30,120`
(saved by hand, no `_savegpr` helper). No FPR save - `f0..f4` are scratch.
Epilogue restores both words, `vrsave`, `addi r1,r1,128`, `blr`.

### Behaviour

```
node   = this + 0x394
angles = { (u16)i16[this+0x100], (u16)i16[this+0x102], (u16)i16[this+0x104] }
pos    = { f32[this+0x0AC], f32[this+0x0B0], f32[this+0x0B4] }

# 1. two "does this sub-animation exist" probes
if (!node_vtable[0x28](node, 0x80B88988) &&
    !node_vtable[0x28](node, 0x80B889C8))
    i16[this+0x9DC] += 0x400                     # 5.625 deg/frame

# 2. lift the y by 3*cos(angle) and build the effective position
a = (f32)(u16)i16[this+0x9DC] * (1.0/256.0)
pos.y += 3.0 * cos_angle16(a)                     # 0x80B0C61C..C648
vect = pos_euler_to_vect(&pos, &angles, 1)        # 0x80064AF0

# 3. primary node
set_translation(this+0x7C, vect.x, vect.y, vect.z)          # 0x801C0BD0
holder_yaw  (this+0x7C, &(u16)i16[this+0x100])              # 0x8016EDD0
holder_pitch(this+0x7C, &(u16)i16[this+0x102])              # 0x8016ECB0
holder_roll (this+0x7C, &(u16)i16[this+0x104])              # 0x8016EEF0
holder_copy  (this+0x588, this+0x7C)                        # 0x8016A170
holder_offset(this+0x588, this+0xDC)                        # 0x8016A150
holder_clear (this+0x588, 0)                                # 0x8016A1A0
holder_flush (this+0x588)                                   # 0x80168400
holder_row   (this+0x588, 3, &local72)                      # 0x80168180
holder_row   (this+0x588, 0, &local72)
f32[this+0x988] = local84                         # row 3 -> local 0x54/0x64/0x74
f32[this+0x98C] = local100
f32[this+0x990] = local116

if (u32[this+0x9C8] != 0) {                       # second, render-only node
    holder_copy  (this+0x5C8, this+0x7C)
    holder_offset(this+0x5C8, this+0xDC)
    holder_clear (this+0x5C8, 0)
}

# 4. drop the copied triad by 24 (or 24*scale) and commit the second node
pos = { f32[this+0x988], f32[this+0x98C], f32[this+0x990] }
pos.y -= (u32[this+0x9D0] == 1) ? 24.0 * f32[this+0xE4] : 24.0
angles = { 0, 0, 0 }
vect = pos_euler_to_vect(&pos, &angles, 1)
set_translation(this+0x7C, vect.x, vect.y, vect.z)
holder_yaw  (this+0x7C, &0)
holder_pitch(this+0x7C, &0)
holder_roll (this+0x7C, &0)
holder_copy  (this+0x548, this+0x7C)
holder_offset(this+0x548, this+0xDC)
holder_clear (this+0x548, 0)
```

### Notes

* The two `+0x394` probes are `lwzu r12,916(r3)` followed by `bctrl`: the
  callee is `u32[u32[this+0x394] + 0x28]` with `r3 = this+0x394` and the
  argument in `r4`. The return value is booleanised by
  `neg r0,r3 / or r0,r0,r3 / rlwinm. r0,r0,1,31,31` (the MWCC `!= 0` idiom),
  and **zero means "no animation"**, which advances the spin.
* `0x80B88988` and `0x80B889C8` are above the image end (`0x80B6E4B8`), so they
  are opaque `main.dol` arguments; the reconstruction does not need their
  contents.
* The local matrix row buffer is the 60-word area at `r1+0x48`; the three
  values read back are the row-3 translation (`r1+0x54`, `+0x64`, `+0x74`).
  This is the same read-back shape `docs/SPEC_WAVE130_ENEMIES.md:833` records
  for `0x80AE6520`.

---

## 3. `0x80AEB450` - `daEnPukuPukuParent_c::StateID_Move` chain solver

* **Address:** `0x80AEB450`
* **TSV size:** 700 bytes
* **Reachable body:** 175 instructions = the whole extent; 3 direct `bl`
  (`0x80237BD0` sin, `0x802DCD74` `_savegprlr_27`, `0x802DCDC0` restore);
  no indirect call; one `blr` at `0x80AEB708`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aeb450_FUN_80aeb450.c`
  - 6 `psq_st`/`psq_l` pairs for `f26..f31` plus the three `qr3` data loads at
    `0x80AEB52C`, `0x80AEB5DC`, `0x80AEB680`; everything else is scalar and the
    export is authoritative once the GQR noise is stripped.
* **Proposed signature:** `pub unsafe fn fn_80aeb450(this: usize)`
* **Callers in this REL:** one 32-bit word at `0x80B611D4`; the record sits
  directly before the string `daEnPukuPukuParent_c::StateID_Move` at
  `0x80B611E4` (the three records `0x80B611C4/C8`, `0x80B611D0/D4`,
  `0x80B611DC/E0` are `0x80AEB440`, `0x80AEB450`, `0x80AEB430`).
* **Constants:** from `0x80B31150` (`lis r5,0x80b3` +
  `addi r5,r5,4432`):

| offset | type | value / role |
|---|---|---|
| `+0x54` | f32 | `32.0` - per-segment pitch in mode 3 |
| `+0x58` | f64 | `0.0` - the "subtract nothing" bias of the int->double conversion |
| `+0x60` | f32 | `0.0` - sign test for the mode-1 wave |
| `+0x64` | f32 | `0.25` - the wave share that stays with the base |
| `+0x68` | f32 | `0.00390625` = `1/256` |
| `+0x6C` | f32 | `-32.0` - per-segment pitch in mode 2/4 |
| `+0x0C..+0x48` | f32[16] | alternating amplitudes `32, -32, 64, -64, 96, -96, 128, -128, 160, -160, 192, -192, 224, -224, 256, -256` |

* **Fields:** `+0x7D0` f32[] segment y array (index 0 = the body y, 1..count =
  the chain), `+0x810` u32 state, `+0x818` u32 segment count, `+0x820` u32 gate
  (non-zero = run), `+0x830` `s16` wave phase.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-144(r1)`; `vrsave` at `+0x94`; `_savegprlr_27` (`bl 0x802DCD74`,
r27-r31 + LR, `r11 = r1+0x30`); `f26..f31` saved as `stfd` + `psq_st fN,..,qr0`
pairs at `+0x30..+0x88`, restored in the mirror order at `0x80AEB6C8..0x80AEB6F4`.
Epilogue: `bl 0x802DCDC0`, `vrsave`, `addi r1,r1,144`, `blr`.

### Behaviour

```
if (u32[this+0x820] == 0) return
i16[this+0x830] += 0x100                       # 1.40625 deg/frame
f32[this+0x7D0] = f32[this+0x0B0]              # segment 0 = body y
state = u32[this+0x810]
n     = (s32)u32[this+0x818]
if (n <= 0) return
theta = (f32)(u16)i16[this+0x830] * (1.0/256.0)
y     = f32[this+0x0B0]

switch (state) {
case 0, 1:                                     # 0x80AEB4E8 - travelling wave
    for (i = 1; i <= n; i++) {
        amp = f32[0x80B31150 + 0x0C + 4*(i-1)]
        h   = amp * 0.25
        s   = fabsf(h) * cos_angle16(theta)
        if (h < 0.0) s = -s
        f32[this+0x7D0 + 4*i] = (y + amp) - h + s
    }
    break

case 2, 4:                                     # 0x80AEB584 - hang down
    for (i = 1; i <= n; i++) {
        f  = (f32)i
        p  = -32.0 * f                          # = -32i
        q  = p * 0.25                           # = -8i
        s  = q * sin_angle16(theta)             # = -8i * sin
        f32[this+0x7D0 + 4*i] = (y + p - q) - s # = (y - 24i) - 8i*sin
    }
    break

case 3:                                        # 0x80AEB628 - stand up
    for (i = 1; i <= n; i++) {
        f  = (f32)i
        p  = 32.0 * f                           # = 32i
        q  = p * 0.25                           # = 8i
        s  = q * sin_angle16(theta)             # = 8i * sin
        f32[this+0x7D0 + 4*i] = (y + p - q) + s # = (y + 24i) + 8i*sin
    }
    break
}
```

### Notes

* The int->double conversion of the loop index is the MWCC `xoris`/`0x4330`
  idiom with the zero double at `+0x58`; the subtraction of `f31` is a no-op and
  is kept only because the compiler emitted it.
* `fmuls f1,f2,f1` in the hanging and standing cases means `q * sin(...)`, i.e.
  the sine *is* consumed (`f1` is both a source and the destination). Do not
  read it as `f2 * f2`.
* The sinusoid is added to the per-segment `-q` term and the body y, so the
  chain is a static ramp plus a travelling wave in the mode-0/1 case and a
  hanging/standing ramp with the same wave in the other two.
* The mode's amplitude table has 16 entries (`+0x0C..+0x48`, the last one
  `0x80B31198 = -256.0`); index 16 would read `0x80B3119C = 0.5`, the first word
  of the following block. The loop does not clamp, so `+0x818` must be treated
  as valid for `0..=15`.

---

## 4. `0x80AE4A50` - `daEnPolter_c::StateID_Wait` hover/steer step

* **Address:** `0x80AE4A50`
* **TSV size:** 660 bytes
* **Reachable body:** 165 instructions = the whole extent; 5 direct `bl`,
  3 `bctrl` (one on `+0x62C`, one on `+0x658`, one on the `+0x60` message slot),
  one `blr` at `0x80AE4CE0`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ae4a50_FUN_80ae4a50.c`
  - the export is the usual `unaff_GQR3` pseudo-code; the RAW stream has no FPR
  spills and exactly three `qr3` loads (`0x80AE4A9C`, `0x80AE4B7C`, `0x80AE4BA0`).
* **Proposed signature:** `pub unsafe fn fn_80ae4a50(this: usize)`
* **Callers in this REL:** one 32-bit word at `0x80B607A4`; the group of three
  records `0x80B60794/C8`, `0x80B607A0/D4`, `0x80B607AC/B0` is followed by the
  `daEnPolter_c` name block at `0x80B60820` (Wait, WaitTime, UpMove,
  StandAngleReset). `0x80AE4A50` is the middle record of the first group.
* **Constants:** from `0x80B30DC8` (`lis r29,0x80b3` +
  `addi r29,r29,3528`):

| offset | type | value / role |
|---|---|---|
| `+0x40` | f32 | `40.0` - x amplitude when `+0x6B0 != 0` |
| `+0x44` | f32 | `1.0` - x steering step when `+0x6B0 != 0` |
| `+0xAC` | f32 | `8.0` - x dead zone |
| `+0xBC` | f32 | `0.0` - the z component of the follow delta |
| `+0xC0` | f32 | `0.00390625` = `1/256` |
| `+0xD0` | f32 | `20.0` - x amplitude when `+0x6B0 == 0` |
| `+0xD4` | f32 | `0.3` - x steering step when `+0x6B0 == 0` |

* **Fields:** `+0x0AC/0x0B0/0x0B4` x/y/z; `+0x0D0/0x0D4/0x0D8` f32 follow
  offsets; `+0x500` u16 gate; `+0x60` vtable (message slot `0xD4`);
  `+0x62C` and `+0x658` sub-nodes (slot `0x14`); `+0x688` f32 ramp;
  `+0x69C` f32 steering x; `+0x6A0` f32 ramp rate; `+0x6B0` u32 mode;
  `+0x6BA` `s16` bob phase; `+0x6BC` `s16` steer phase.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-80(r1)`; `vrsave` at `+0x54`; `stw r31,76`, `stw r30,72`,
`stw r29,68` by hand; no FPR save. Epilogue mirrors it and `blr`s.

### Behaviour

```
sub_node(this+0x62C)[0x14](this+0x62C)           # per-frame sub-node tick

# 1. vertical bob
i16[this+0x6BA] += 0x240                          # 3.164 deg/frame
theta = (f32)(u16)i16[this+0x6BA] * (1.0/256.0)
f32[this+0x6A0] += 0.5 * sin_angle16(theta)      # 0x80AE4AB4..C8
f32[this+0x688]  = f32[this+0x6A0]
f32[this+0x0B0]  = f32[*0x80429E00 + 0x3C] - f32[this+0x688]   # (ext) base y

# 2. acquire target
target = fn_80064740(this, &local16)             # main.dol; 0 = no target
if (target == 0) goto tail

step = (u32[this+0x6B0] == 0) ? 0.3 : 1.0
if (fabsf(f32[this+0x69C] - f32[target+0xAC]) > 8.0) {
    if (f32[target+0xAC] >= f32[this+0x69C]) f32[this+0x69C] += step
    else                                     f32[this+0x69C] -= step
}

# 3. lateral sweep
i16[this+0x6BC] += 0x300                          # 4.21875 deg/frame
theta2 = (f32)(u16)i16[this+0x6BC] * (1.0/256.0)
amp    = (u32[this+0x6B0] == 0) ? 20.0 : 40.0
f32[this+0x0AC] = f32[this+0x69C] + amp * cos_angle16(theta2)

# 4. pitch or full follow
if (u32[this+0x6B0] == 0) {
    i16[this+0x102] += 0x400
    i16[this+0x104] += 0x80
} else {
    p   = { f32[target+0xAC] + f32[this+0x0D0],
            f32[target+0x0B0] + f32[this+0x0D4],
            f32[target+0x0B4] + f32[this+0x0D8] }
    d   = { p.x - f32[this+0x0AC], p.y - f32[this+0x0B0], 0.0 }
    local24 = p ; local48 = p
    fn_80ae4410(this, &d, 0x200)                 # facing mirror step
    fn_80ae4380(this)                            # ramp flip
}

tail:
if (u16[this+0x500] == 0) {
    msg = (u32[this+0x6B0] == 0) ? 0x80B77110 : 0x80B77150     # (ext)
    this_vtable[0xD4](this, msg)
}
sub_node(this+0x658)[0x14](this+0x658)
```

### Notes

* `0x80B77110` and `0x80B77150` are above the image end and are opaque message
  pointers **(ext)**.
* `fn_80064740(this, out)` is a `main.dol` helper that both returns the target
  actor and writes a position into `out`; only the return value is used here.
* `fn_80ae4410` / `fn_80ae4380` are ledgered siblings in this module
  (`game::enemies`); their contracts are recorded in the helper table above.
* The x steering is a one-pole step with an 8-unit dead zone, and the sweep is
  added on top of it, so `+0x69C` is the steering state, not the position.

---

## 5. `0x80A4B740` - `daEnCrow_c` circle-to-target steer

* **Address:** `0x80A4B740`
* **TSV size:** 604 bytes
* **Reachable body:** 151 instructions = the whole extent; 6 direct `bl`,
  2 `bctrl` (one on `+0x544`, one on the `+0x60` message slot), one `blr` at
  `0x80A4B998`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a4b740_FUN_80a4b740.c`
  - two `qr3` loads (`0x80A4B834`, `0x80A4B860`), no FPR spills.
* **Proposed signature:** `pub unsafe fn fn_80a4b740(this: usize)`
* **Callers in this REL:** one 32-bit word at `0x80B43164` (the `daEnCrow_c`
  state-handler array; the name block is `0x80B43228`..`0x80B432EC`,
  Appear/Loop/Swoop/Escape/Escape2/Escape3/DieFumi/DieFall).
* **Constants:** from `0x80B2C9B0` (`lis r31,0x80b3` +
  `addi r31,r31,-13904`):

| offset | type | value / role |
|---|---|---|
| `+0x60` | f64 | `0x4330000080000000` - the MWCC int->double bias, not a game value |
| `+0x94` | f32 | `0.00390625` = `1/256` |
| `+0xA4` | `s16[2]` | `{-4551, +4551}` = `+-25 deg` in turn units |
| `+0xA8` | `s16[2]` | `{-10013, +10013}` = `+-55 deg` |
| `+0xB0` | f32[8] | `{1.5, 1.7141, 1.9282, 2.1423, 2.3564, 2.5706, 2.7847, 2.9988}` |
| `+0xD0` | `s16[8]` | `{192, 256, 128, 160, 80, 96, 64, 72}` - per-index `+0x5BC` rate |
| `+0xE0` | `s16[8]` | `{544, 640, 456, 496, 416, 432, 386, 400}` - per-index `+0x5C0` rate |
| `+0xF0` | f32 | `16.0` - the lateral give-up window |

* **Fields:** `+0x100/0x102/0x104` `s16` angles; `+0x0AC/0x0B0/0x0B4` x/y/z;
  `+0x0E8/0x0EC` f32 velocity; `+0x10C` f32 speed; `+0x348` `u8` side byte;
  `+0x500` u16 gate; `+0x544` matrix node (slot `0x1C`); `+0x5BC` u32 phase;
  `+0x5C0` u32 orbit phase; `+0x5C8` u16 index; `+0x5D0` `s8` player slot.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-32(r1)`; `vrsave` at `+0x24`; `stw r31,28`, `stw r30,24` by hand; no
FPR save (uses `f0..f5`).

### Behaviour

```
sub_node(this+0x544)[0x1C](this+0x544)
idx = u16[this+0x5C8]

# 1. phase accumulators
if (u32[this+0x5BC] < 0x1000)
    u32[this+0x5BC] += (u32)((s32)i16[0x80B2C9B0 + 0xD0 + 2*idx] << 12)
u32[this+0x5C0] += (u32)((s32)i16[0x80B2C9B0 + 0xE0 + 2*idx] << 12)
if (u32[this+0x5C0] > 0x800) u32[this+0x5C0] = 0x800

# 2. facing
if (u16[this+0x500] != 0) {
    i16[this+0x104] -= 0x800                      # -11.25 deg
} else {
    step_short_clamped(&i16[this+0x104],
        0x80B2C9B0 + 0xA4 + 2*u8[this+0x348], 512)
    step_short_clamped(&i16[this+0x102],
        0x80B2C9B0 + 0xA8 + 2*u8[this+0x348], 1024)
}

# 3. velocity from the two phases
theta = (f32)(u16)(u32[this+0x5BC] >> 12) * (1.0/256.0)
f32[this+0x0E8] =  f32[this+0x10C] * cos_angle16(theta)
f32[this+0x0EC] = -f32[0x80B2C9B0 + 0xB0 + 4*idx] * sin_angle16(theta)

# 4. drift with the world height
base_actor_update(this)                           # 0x8006CD40
gy  = f32[*0x80429E00 + 0x38]
ref = f32[*(*0x80439DD0 + 0x90000) - 0x158]       # (ext)
f32[this+0x0AC] += (gy - ref)

# 5. face the motion direction
kv  = (f32)(s32)(s8)u8[0x8043C1A0 + u8[this+0x348]]   # (ext) side constant
ang = atan2_16(-f32[this+0x0EC], f32[this+0x0E8] * kv)
step_short_clamped(&i16[this+0x100], ang, 512)

# 6. lateral give-up test
player = player_by_slot((s8)u8[this+0x5D0])
side   = u8[this+0x348]
if (side == 0) {
    if (f32[this+0x0AC] > f32[player+0xAC] + 16.0)
        this_vtable[0xD4](this, 0x80B710F0)       # (ext)
} else if (side == 1) {
    if (f32[this+0x0AC] < f32[player+0xAC] - 16.0)
        this_vtable[0xD4](this, 0x80B710F0)
}
```

### Notes

* The `>> 12` on `+0x5BC` is the fixed-point to angle conversion: the
  accumulator is `u32` and its top 16 bits are the turn units. `+0x5C0` is
  accumulated and clamped to `0x800` but not read in this body; it is the
  state's distance/time counter that the sibling bodies consume.
* `atan2_16(f1 = -vz', f2 = vx' * kv)` is called with the negated `+0x0EC` and
  the `+0x0E8` scaled by `kv`, exactly as `fneg f1,f3` / `fmuls f2,f0,f2`
  encode it. `kv` is the sign-extended byte `0x8043C1A0[u8[+0x348]]` widened
  through the `0x4330000080000000` bias; the table itself is `main.dol`
  **(ext)**, but the conversion makes it a plain `f32`.
* The 40.0 at `+0x9C` belongs to the sibling rank 6 body's landing test; this
  body never reads `+0x98` or `+0x9C`, only `+0x94`, `+0xA4`, `+0xA8`, `+0xB0`,
  `+0xD0`, `+0xE0`, `+0xF0` and the `+0x60` double.
* `0x80B710F0` is above the image end: an opaque message pointer **(ext)**.
* The `this.x += (gy - ref)` step is literal, not a transposed y store: ranks 6
  and 11 carry the identical `lfs`/`fadds`/`stfs` on `+0xAC`, so the whole
  `daEnCrow_c` family rides a stage that translates x with the global height
  register `*0x80429E00 + 0x38` minus the `main.dol` reference at
  `*(*0x80439DD0 + 0x90000) - 0x158`.

---

## 6. `0x80A4B420` - `daEnCrow_c` dive with a landing probe

* **Address:** `0x80A4B420`
* **TSV size:** 548 bytes
* **Reachable body:** 137 instructions = the whole extent; 7 direct `bl`,
  2 `bctrl` (one on `+0x544`, one on the `+0x60` message slot), one `blr` at
  `0x80A4B640`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a4b420_FUN_80a4b420.c`
  - two `qr3` loads (`0x80A4B4F0`, `0x80A4B514`), no FPR spills.
* **Proposed signature:** `pub unsafe fn fn_80a4b420(this: usize)`
* **Callers in this REL:** one 32-bit word at `0x80B43140` (same `daEnCrow_c`
  array as rank 5).
* **Constants:** `0x80B2C9B0` again:

| offset | type | value / role |
|---|---|---|
| `+0x30` | f32[2] | `{3.0, -3.0}` - speed target by side |
| `+0x40` | `s16[2]` | `{-12288, +12288}` = `+-67.5 deg` yaw offset by side |
| `+0x90` | f32 | `0.0078125` = `1/128` - speed step |
| `+0x94` | f32 | `0.00390625` = `1/256` |
| `+0x98` | f32 | `0.375` |
| `+0x9C` | f32 | `40.0` - landing height window |

* **Fields:** `+0x100/0x102/0x104` `s16` angles; `+0x0AC/0x0B0/0x0B4` x/y/z;
  `+0x0E8/0x0EC` f32 velocity; `+0x10C` f32 speed; `+0x348` `u8` side byte;
  `+0x500` u16 gate; `+0x544` matrix node (slot `0x1C`); `+0x5BC` u32 phase;
  `+0x5C8` u16 index; `+0x5D0` `s8` player slot; `+0x5D4/0x5D8/0x5DC` f32 last
  landing position.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-64(r1)`; `vrsave` at `+0x44`; `stw r31,60 .. stw r28,48` by hand; no
FPR save.

### Behaviour

```
sub_node(this+0x544)[0x1C](this+0x544)
side = u8[this+0x348]

# 1. speed toward the side target
step_float_clamped(f1 = (f64)f32[0x80B2C9B0 + 0x30 + 4*side],
                   f2 = (f64)f32[0x80B2C9B0 + 0x90],     # 1/128
                   &f32[this+0x10C])

# 2. phase and yaw
u32[this+0x5BC] += 0xA0000
i16[this+0x102] = (s16)(u32[this+0x5BC] >> 12)
                  + i16[0x80B2C9B0 + 0x40 + 2*side]
if (i16[this+0x104] != -0x4000)
    if (step_angle_toward(&i16[this+0x104], -0x4000, 64, 512, 64) == 0)
        i16[this+0x104] = -0x4000

# 3. velocity
theta = (f32)(u16)(u32[this+0x5BC] >> 12) * (1.0/256.0)
f32[this+0x0E8] = f32[this+0x10C] * cos_angle16(theta)
f32[this+0x0EC] = 0.375 * sin_angle16(theta)

base_actor_update(this)                           # 0x8006CD40
gy  = f32[*0x80429E00 + 0x38]
ref = f32[*(*0x80439DD0 + 0x90000) - 0x158]       # (ext)
f32[this+0x0AC] += (gy - ref)

# 4. landing probe once the dive is past 0xC00
if (u32[this+0x5BC] >= 0xC00) {
    player = player_by_slot((s8)u8[this+0x5D0])
    hit = 0
    if (player != 0) {
        actor_world_pos(&local28, player)         # 0x8006CED0
        if (f32[this+0x0B0] >= f32[local28+4] + 40.0) hit = 1
    }
    if (hit) {
        actor_world_pos(&local16, player)
        f32[this+0x5D4] = f32[local16+0]
        f32[this+0x5D8] = f32[local16+4]
        f32[this+0x5DC] = f32[local16+8]
        this_vtable[0xD4](this, 0x80B710B0)       # (ext)
    } else {
        this_vtable[0xD4](this, 0x80B71130)       # (ext)
    }
}
```

### Notes

* The pitch clamp is a plain `-90 deg` (`-0x4000`) approach at 64 units/frame
  with a 512 limit and a 64 second limit; when the helper reports "not
  arrived" (returns 0) the field is written directly.
* The landing test is `this.y >= player.y + 40.0` (the `cror 2,1,2` form is
  `ge`; the following `bne` skips when the condition fails).
* `0x80B710B0`/`0x80B71130` are above the image end **(ext)**.

---

## 7. `0x80A635D0` - `daEnGesso_c` two-phase bob

* **Address:** `0x80A635D0`
* **TSV size:** 488 bytes
* **Reachable body:** 122 instructions = the whole extent; 3 direct `bl`
  (`0x80A61B20` sibling step, `0x80237BD0` sin, `0x80097800` pad query),
  2 `bctrl` (one on `+0x544`, one on the `+0x60` message slot), one `blr` at
  `0x80A637B4`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a635d0_FUN_80a635d0.c`
  - one `qr3` load (`0x80A63640`), no FPR spill.
* **Proposed signature:** `pub unsafe fn fn_80a635d0(this: usize)`
* **Callers in this REL:** one 32-bit word at `0x80B4874C` (a `daEnGesso_c`
  handler record; the class name block is `0x80B4890C`..`0x80B48988`,
  SwimReady/SwimUp/SwimWait/SwimDown/SwimDown_Fin).
* **Constants:** from `0x80B2D2B0` (`lis r31,0x80b3` +
  `addi r31,r31,-11600`):

| offset | type | value / role |
|---|---|---|
| `+0x34` | f32 | `96.0` - added to `+0x1CC` for the pad query range (144.0 total) |
| `+0x1A8` | f64 | `0x4330000080000000` - int->double bias |
| `+0x1C4` | f32 | `0.00390625` = `1/256` |
| `+0x1C8` | f32 | `1024.0` - bob amplitude |
| `+0x1CC` | f32 | `48.0` - pad query base range |

* **Fields:** `+0x104` `s16` pitch; `+0x360` u32 state; `+0x544` matrix node
  (slot `0x1C`); `+0x584` animation holder; `+0x728` `s16` base pitch;
  `+0x72A` `s16` bob phase; `+0x72E` `u8` flag; `+0x731` `u8` mode;
  `+0x733` `u8` timer A; `+0x734` `u8` timer B.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-48(r1)`; `vrsave` at `+0x34`; `stw r31,44`, `stw r30,40`,
`stw r29,36` by hand; no FPR save.

### Behaviour

```
fn_80a61b20(this)                                 # sibling phase step
sub_node(this+0x544)[0x1C](this+0x544)

state = u32[this+0x360]
if (state == 1) {
    i16[this+0x72A] += 0x1000                     # 22.5 deg/frame
    theta = (f32)(u16)i16[this+0x72A] * (1.0/256.0)
    i16[this+0x104] = (s16)(s32)( (f32)(s32)i16[this+0x728]
                                  + 1024.0 * sin_angle16(theta) )
    if (i16[this+0x72A] == 0) {                   # wrapped: one full turn
        if (--u8[this+0x734] == 0) {
            u8[this+0x733] = 30
            i16[this+0x104] = i16[this+0x728]
            u32[this+0x360] = 2
        }
    }
} else if (state == 2) {
    if (--u8[this+0x733] == 0) {
        u8[this+0x734] = 3
        u32[this+0x360] = 1
    }
}

# pad query: 48 + 96 = 144 units
if (fn_80097800(this, f32[0x80B2D2B0+0x1CC] + f32[0x80B2D2B0+0x34]) == 0)
    goto done

if (u8[this+0x72E] != 0) {
    this_vtable[0xD4](this, 0x80B72110)           # (ext)
} else switch (u8[this+0x731]) {
case 1: this_vtable[0xD4](this, 0x80B72110); break   # (ext)
case 2: this_vtable[0xD4](this, 0x80B72150); break   # (ext)
case 3: this_vtable[0xD4](this, 0x80B72190); break   # (ext)
}

done: ;
```

### Notes

* The bob is a `1024` amplitude sine around `+0x728`, converted through
  `fctiwz` (so the result is truncated, not rounded) and stored back as `s16`.
* `+0x734` counts 3 frames of state 1 and `+0x733` counts 30 frames of state 2;
  the two timers are reloaded at the swap, which is why both exist.
* `0x80097800(this, 144.0)` is a `main.dol` range/pad query; only its
  zero/non-zero result is used.
* The three message pointers are above the image end **(ext)**; the fan-out is
  driven only by the two bytes at `+0x72E` and `+0x731`.

---

## 8. `0x80A63FF0` - `daEnGesso_c` drifting jet

* **Address:** `0x80A63FF0`
* **TSV size:** 392 bytes
* **Reachable body:** 98 instructions = the whole extent; 6 direct `bl`,
  2 `bctrl` (one on `+0x544`, one on the `+0x60` message slot), one `blr` at
  `0x80A64174`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a63ff0_FUN_80a63ff0.c`
  - two `qr3` loads (`0x80A64048`, `0x80A64070`) plus one `f31` spill pair.
* **Proposed signature:** `pub unsafe fn fn_80a63ff0(this: usize)`
* **Callers in this REL:** one 32-bit word at `0x80B48890` (same `daEnGesso_c`
  array as rank 7).
* **Constants:** `0x80B2D2B0` again:

| offset | type | value / role |
|---|---|---|
| `+0x128` | f32 | `0.0` - the z velocity term |
| `+0x130` | f32 | `1.0` - animation rate written on the state-2 switch |
| `+0x1C4` | f32 | `0.00390625` = `1/256` |

* **Fields:** `+0x0AC/0x0B0/0x0B4` x/y/z; `+0x0E8/0x0EC/0x0F0` f32 velocity;
  `+0x104` `s16` yaw; `+0x10C` f32 speed; `+0x360` u32 state; `+0x544` matrix
  node; `+0x584` animation holder; `+0x70C` f32 deceleration; `+0x724` f32
  z clamp; `+0x733` `u8` timer.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-48(r1)`; `vrsave` at `+0x34`; `stfd f31,32` + `psq_st f31,40,0,qr0`;
`stw r31,28`, `stw r30,24`. Epilogue restores `f31` with the `qr0` pair and
`blr`s.

### Behaviour

```
fn_80a61b60(this)
fn_80a61b20(this)
sub_node(this+0x544)[0x1C](this+0x544)

theta = (f32)(u16)(i16[this+0x104] + 0x4000) * (1.0/256.0)
vx = f32[this+0x10C] * cos_angle16(theta)
vz = f32[this+0x10C] * sin_angle16(theta)

f32[this+0x0AC] = f32[this+0x0AC] + f32[this+0x0E8] + vx
f32[this+0x0B0] = f32[this+0x0B0] + f32[this+0x0EC] + vz
f32[this+0x0B4] = f32[this+0x0B4] + f32[this+0x0F0] + 0.0

state = u32[this+0x360]
if (state == 1) {
    if (--u8[this+0x733] == 0) {
        set_anim_rate(this+0x584, 1.0)            # 0x801680E0
        f32[this+0x0B4] = f32[this+0x724]
        u32[this+0x360] = 2
    }
} else if (state == 2) {
    f32[this+0x10C] = f32[this+0x10C] - f32[this+0x70C]
    if (test_anim_holder(this+0x584) != 0)        # 0x80169020
        this_vtable[0xD4](this, 0x80B72350)       # (ext)
}
```

### Notes

* The `+0x4000` quarter-turn offset turns the yaw into a "forward" direction:
  x uses the cosine of `yaw + 90 deg` and z the sine, which is the standard
  `(sin, cos)` velocity pair rotated by a quarter turn.
* The three locals staged at `r1+0x0C/0x10/0x14` (`vx`, `vz`, `0.0`) are
  written but never read in this body; they are the compiler's dead spill of the
  velocity triple, kept in the Rust port for structural fidelity only.
* `0x80B72350` is above the image end **(ext)**.

---

## 9. `0x80A641F0` - `daEnGesso_c` coast stop

* **Address:** `0x80A641F0`
* **TSV size:** 380 bytes
* **Reachable body:** 95 instructions = the whole extent; 6 direct `bl`,
  2 `bctrl` (one on `+0x544`, one on the `+0x60` message slot), one `blr` at
  `0x80A64368`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a641f0_FUN_80a641f0.c`
  - two `qr3` loads (`0x80A64240`, `0x80A6426C`) plus one `f31` spill pair.
* **Proposed signature:** `pub unsafe fn fn_80a641f0(this: usize)`
* **Callers in this REL:** one 32-bit word at `0x80B488B4` (same `daEnGesso_c`
  array as ranks 7-8).
* **Constants:**

| address | type | value / role |
|---|---|---|
| `0x80B2D474` | f32 | `0.00390625` = `1/256` (read as `lfs f0,-11148(r31)`) |
| `0x80B2D3D8` | f32 | `0.0` - the deceleration floor and the z term |

* **Fields:** `+0x0AC/0x0B0/0x0B4` x/y/z; `+0x0E8/0x0EC/0x0F0` f32 velocity;
  `+0x104` `s16` yaw; `+0x10C` f32 speed; `+0x544` matrix node; `+0x584`
  animation holder; `+0x70C` f32 deceleration.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-48(r1)`; `vrsave` at `+0x34`; `stfd f31` + `psq_st f31` pair;
`stw r31,28`, `stw r30,24`.

### Behaviour

```
fn_80a61b60(this)
fn_80a61b20(this)
sub_node(this+0x544)[0x1C](this+0x544)

theta = (f32)(u16)(i16[this+0x104] + 0x4000) * (1.0/256.0)
vx = f32[this+0x10C] * cos_angle16(theta)
vy = f32[this+0x10C] * sin_angle16(theta)

f32[this+0x0AC] += f32[this+0x0E8] + vx
f32[this+0x0B0] += f32[this+0x0EC] + vy
f32[this+0x0B4] += f32[this+0x0F0] + 0.0

speed = f32[this+0x10C] - f32[this+0x70C]
f32[this+0x10C] = speed
if (speed <= 0.0) f32[this+0x10C] = 0.0           # floor, not a bounce

if (step_short_clamped(&i16[this+0x104], 0, 128) != 0     # 0x8015F4F0
    && f32[this+0x10C] <= 0.0
    && test_anim_holder(this+0x584) != 0)
    this_vtable[0xD4](this, 0x80B71F90)           # (ext)
```

### Notes

* This is the sibling of rank 8 with the sine moved to the y axis and the
  speed decaying instead of the state flag switching. Both `cror 2,0,2` sites
  are `le` tests: the speed is floored at 0, and the message only fires when the
  speed has reached 0.
* `0x80B71F90` is above the image end **(ext)**.

---

## 10. `0x80ABA080` - hit-response splash

* **Address:** `0x80ABA080`
* **TSV size:** 308 bytes
* **Reachable body:** 77 instructions = the whole extent; 5 direct `bl`,
  1 `bctrl` on the *incoming* actor's vtable slot `0x3FC`, one `blr` at
  `0x80ABA1B0`.
* **Proposed signature:**
  `pub unsafe fn fn_80aba080(this: usize, other: usize)`
  - `r4` is the other actor and is **not** nullable-guarded by the caller; the
    body itself returns immediately when it is 0.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aba080_FUN_80aba080.c`
  - two `qr3` loads (`0x80ABA0F4`, `0x80ABA110`) plus `f30`/`f31` spill pairs.
* **Callers in this REL:** `bl` from `0x80AB9ED8`, inside a four-slot loop
  (`r28 < 4`) that resolves each slot with `0x8005F900` and passes the player as
  `r4`; the call is made only when bit 0 of the loop mask is set.
* **Constants:** from `0x80B2FF20` (`lis r30,0x80b3` +
  `addi r30,r30,-224`):

| offset | type | value / role |
|---|---|---|
| `+0x28` | f32 | `5.2` - impulse magnitude |
| `+0x30` | f32 | `0.0` - base of the atan2 x argument |
| `+0x34` | f32 | `0.5` - blend factor between the impulse and its mirror |

* **Fields:** `+0x0AC/0x0B0/0x0B4` x/y/z; `+0x53C` f32 source scale;
  `other+0xE8` f32 target scale; `other+0x60` vtable.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-80(r1)`; `vrsave` at `+0x54`; `f31`/`f30` saved as `stfd` +
`psq_st ..,qr0` pairs; `stw r31,44 .. stw r28,32` by hand.

### Behaviour

```
if (other == 0) return

mag = 5.2
x   = f32[this+0x53C] * f32[other+0xE8]           # range * target scale
ang = atan2_16(f1 = (f64)mag, f2 = (f64)(0.0 + x))     # 0x801611C0

c = mag * cos_angle16((f32)(u16)ang * (1.0/256.0))     # f30
s = mag * sin_angle16((f32)(u16)ang * (1.0/256.0))     # f2

pos_to_vect(&local16, this+0xAC)                        # 0x8006A4B0
se_play_pos_2(*0x80439D60, 0x706, &local16, 0)          # 0x80198C30, cue 1798

other_vtable[0x3FC](other,
                    f1 = s + 0.5 * (mag - s),           # = 2.6 * (1 + sin)
                    f2 = c,                             # = 5.2 * cos
                    1, 1, 0)
```

### Notes

* The `f1` passed to the vtable is a *blend*, not the raw sine term: the code
  forms `mag - s` (`fsubs f1,f31,f2`), multiplies it by the `0.5` at `+0x34`
  (`fmuls f0,f0,f1`) and adds `s` back (`fadds f31,f2,f0`), i.e.
  `f1 = 2.6 * (1 + sin(ang))`. It is the half-way point between the impulse and
  its mirror, which keeps the effect non-negative.
* The atan2 x argument is `range * other_scale`, so a small range collapses the
  direction towards 0 and the impulse becomes purely vertical.
* `0x80439D60` is a `main.dol` global **(ext)**; the cue id (`0x706` = 1798) is
  an immediate, so it is exact.
* No branch in the body depends on the return value of the sound call.

---

## 11. `0x80A4BBD0` - `daEnCrow_c` climb and level-off

* **Address:** `0x80A4BBD0`
* **TSV size:** 304 bytes
* **Reachable body:** 76 instructions = the whole extent; 4 direct `bl`,
  1 `bctrl` on `+0x544`, one `blr` at `0x80A4BCFC`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a4bbd0_FUN_80a4bbd0.c`
  - two `qr3` loads (`0x80A4BC38`, `0x80A4BC5C`), no FPR spills.
* **Proposed signature:** `pub unsafe fn fn_80a4bbd0(this: usize)`
* **Callers in this REL:** one 32-bit word at `0x80B431AC` (same `daEnCrow_c`
  array as ranks 5-6).
* **Constants:** `0x80B2C9B0` again:

| offset | type | value / role |
|---|---|---|
| `+0x40` | `s16[2]` | `{-12288, +12288}` = `+-67.5 deg` yaw offset by side |
| `+0x94` | f32 | `0.00390625` = `1/256` |
| `+0x98` | f32 | `0.375` - vertical velocity scale |

* **Fields:** `+0x100/0x102/0x104` `s16` angles; `+0x0AC/0x0B0/0x0B4` x/y/z;
  `+0x0E8/0x0EC` f32 velocity; `+0x10C` f32 speed; `+0x348` `u8` side byte;
  `+0x544` matrix node; `+0x5BC` u32 phase; `+0x5C8` u16 index (unused here).
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-32(r1)`; `vrsave` at `+0x24`; `stw r31,28`, `stw r30,24`,
`stw r29,20` by hand; no FPR save.

### Behaviour

```
sub_node(this+0x544)[0x1C](this+0x544)

u32[this+0x5BC] += 0x100000
if (u32[this+0x5BC] > 0x1000) u32[this+0x5BC] = 0x1000
phase = (s16)(u32[this+0x5BC] >> 12)
theta = (f32)(u16)(u32[this+0x5BC] >> 12) * (1.0/256.0)

f32[this+0x0E8] = f32[this+0x10C] * cos_angle16(theta)
f32[this+0x0EC] = 0.375 * sin_angle16(theta)

base_actor_update(this)                            # 0x8006CD40
gy  = f32[*0x80429E00 + 0x38]
ref = f32[*(*0x80439DD0 + 0x90000) - 0x158]        # (ext)
f32[this+0x0AC] += (gy - ref)

i16[this+0x102] = phase + i16[0x80B2C9B0 + 0x40 + 2*u8[this+0x348]]

if (i16[this+0x104] != 0)
    step_angle_4(&i16[this+0x104], 0, 16, 1024)     # 0x8015F2E0
```

### Notes

* The phase is clamped at `0x1000`, i.e. the argument never exceeds `16.0`
  before the `1/256` scale, so the climb curve stops at a quarter of its sweep.
* The `+0x5C8` index is loaded by the sibling bodies; this one does not read the
  per-index tables, which is why the `0x80B2C9B0 + 0xD0/+0xE0` tables are
  absent from its constant list.
* `+0x104` is levelled toward 0 at 16 units/frame with a 1024 limit; the helper
  writes the value back, so no extra store is needed.

---

## 12. `0x80A46200` - vertical ping-pong platform step

* **Address:** `0x80A46200`
* **TSV size:** 300 bytes
* **Reachable body:** 75 instructions = the whole extent; 2 direct `bl`
  (`0x80237BD0` sin, `0x80A46180` ramp apply), no indirect call, one `blr` at
  `0x80A46328`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a46200_FUN_80a46200.c`
  - the wave-126 skip note is the `qr3`+`ldexpf` pattern; the RAW stream has one
    `qr3` load (`0x80A462CC`) and one `f31` spill pair.
* **Proposed signature:** `pub unsafe fn fn_80a46200(this: usize)`
* **Callers in this REL:** `bl` from `0x80A45FE4`, inside the `0x80A45FB0`
  frame update (vtable block `0x80B41AC8`; the group-3 actor of
  `docs/COVERAGE_WAVE59_ENEMIES.md:43`), immediately before
  `0x8007FDA0(this+0x534)` and `0x80A46090`.
* **Constants:** from `0x80B2C7A0` (`lis r30,0x80b3` +
  `addi r30,r30,-14432`):

| offset | type | value / role |
|---|---|---|
| `+0x18` | f32 | `0.0` - the idle displacement |
| `+0x1C` | f32 | `0.00390625` = `1/256` |
| `+0x20` | f32 | `4.0` - travel amplitude |

* **Fields:** `+0x0B0` y; `+0x61C` f32 base height; `+0x0ACC` u16 travel
  counter; `+0x0ACE` `s16` travel offset; `+0x0AD2` `s16` ramp direction
  (owned by the sibling `fn_80a46180`); `+0x0AD4` u32 source latch;
  `+0x0AD8` u32 travel target (signed: sign = direction).
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-48(r1)`; `vrsave` at `+0x34`; `stfd f31` + `psq_st f31` pair;
`stw r31,28`, `stw r30,24`.

### Behaviour

```
g    = u32[0x80439D40]                              # (ext) manager
meas = f32[*(*0x80429DD0 + 0x90000) - 0x13C]        # (ext) reference height

if (u32[g+0x2C] == 0 && u32[this+0xAD4] != 0) {
    u32[this+0xAD8] = u32[this+0xAD4]
    i16[this+0xACE] = 0
    u16[this+0xACC] = 0
}

disp = 0.0
if (u32[this+0xAD8] != 0) {
    if ((s32)u32[this+0xAD8] < 0) i16[this+0xACE] -= 0x400
    else                          i16[this+0xACE] += 0x400
    u16[this+0xACC] += 0x400
    if (u16[this+0xACC] >= 0x8000) {
        u16[this+0xACC] = 0
        u32[this+0xAD8] = 0
        i16[this+0xACE] = 0
    }
    disp = 4.0 * sin_angle16((f32)(u16)i16[this+0xACE] * (1.0/256.0))
}

f32[this+0x0B0] = meas + f32[this+0x61C] + disp

fn_80a46180(this)                                   # ramp apply (+0x104)
u32[this+0xAD4] = u32[g+0x2C]
```

### Notes

* The travel is a half-period ping-pong: `+0xACC` runs `0 .. 0x8000` at
  `0x400`/frame, and at the wrap both the target and the offset are zeroed, so
  the platform returns exactly to `+0x61C`.
* The sign of the target selects the direction; a negative target drives the
  offset down, a positive one drives it up.
* `+0xAD2` is not touched here - the sibling `fn_80a46180` (translated) owns the
  `+0x104` ramp and flips `+0xAD2` when it reaches zero.
* `0x80439D40`, `0x80429DD0` and the `-0x13C` reference are `main.dol`
  **(ext)** data.

---

## Verification

* Twelve addresses, all absent from the `address` column of
  `ledger/functions.csv`, all absent from every `#`-heading of every
  `docs/*.md`, and none has a `fn_<addr>` body in `src/`.
* Each body was walked from its entry over every conditional branch and every
  linked call; each walk terminated at exactly one `blr`, and the reachable
  word count equals `size / 4` for all twelve (968/242, 848/212, 700/175,
  660/165, 604/151, 548/137, 488/122, 392/98, 380/95, 308/77, 304/76, 300/75).
* Every direct branch target lands inside the same extent; no path lands on
  data, and no body reads a register the decoder never defined.
* The `qr3` count per body matches the table in "GQR3 and the qr3 pool"; no body
  contains a `psq_st` on a data base and none reads `qr1`, `qr2`, `qr4`..`qr7`.
* Constants were read from the ELF at the addresses cited; the float and
  halfword values in the tables are the shipped bytes, not estimates.
* Arms-length cross-checks used while decoding (not new claims):
  `fn_80a0e450`'s Rust port for the `+0x5E8`/`+0x5EC` hop fields
  (`src/game/enemies.rs:168271`), `docs/SPEC_WAVE130_ENEMIES.md:92-114` for the
  helper contracts, `docs/COVERAGE_WAVE59_ENEMIES.md:43` for the group-3
  vtable, and `work/ghidra/rel-d_enemiesNP/export/functions.tsv` for the
  extents.

## Left for the next wave

After this tranche the wave-127 qr3 pool still has fifteen unclaimed rows (all
absent from the ledger and from every heading). Sizes are TSV extents:

| address | size | | address | size |
|---|---:|---|---|---:|
| `0x80AFDD50` | 300 | | `0x80A7C600` | 184 |
| `0x80AB1900` | 256 | | `0x80B23480` | 180 |
| `0x80B07D70` | 252 | | `0x80AB21D0` | 176 |
| `0x80A79630` | 236 | | `0x80AFD170` | 156 |
| `0x80ABAB50` | 236 | | `0x80ADDAC0` | 124 |
| `0x80A40290` | 232 | | `0x80ADDA40` | 120 |
| `0x80A20810` | 228 | | | |
| `0x80A7ADA0` | 220 | | | |
| `0x80AA3F50` | 200 | | | |

The four remaining non-qr3 class-D rows are `0x80A79810` (360),
`0x80AEC4E0` (292) and `0x80AEEFA0` (292), which read an `extraout_` register
the decompiler never defines, and `0x80A337B0` (4), a `b 0x80A337C0` alias of
another unclaimed entry. None of them is unblocked by the GQR3 pin.
