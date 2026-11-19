# Wave 145 - implementation specification for 12 untranslated `d_enemiesNP` bodies

## What this page is

`docs/ANALYSIS_WAVE127_ENEMIES.md` reclassified 384 formerly-skipped `d_enemiesNP`
addresses into 173 class **B**, 160 class **C** and 51 class **D**. Waves 128-144
(`docs/SPEC_WAVE128_ENEMIES.md` .. `docs/SPEC_WAVE144_ENEMIES.md`) specified that
whole block: wave 143 left one class-D `qr3` row, which wave 144 took together
with the eleven largest remaining bodies. That leaves the plain size-ranked
tranche: the next twelve bodies by descending `functions.tsv` extent, whether or
not they contain a `qr3` site.

A live re-walk of `work/ghidra/rel-d_enemiesNP/export/functions.tsv` (3,555 rows)
against the `address` column of `ledger/functions.csv` (16,848 addresses), against
every `0x80Axxxxx`/`0x80Bxxxxx` address that appears in a `^#` heading of any
`docs/*.md` (1,403 distinct addresses) and against every `fn_<addr>`/`FN_<addr>`
token in `src/**/*.rs` (68 files) leaves **137** untranslated bodies. The largest
twelve that still have a Ghidra export are the rows below.

This page is **read-only**: it writes no Rust, no ledger row and no other doc.

## Selection

Every row below is absent from the ledger, from every `^#` heading and from
`src/`. For all twelve rows the body reachable from the entry fills the TSV
extent exactly (`reachable insns == size / 4`), so no row borrows a neighbour's
instructions and none contains a `bctr` table that has to be read out of the
image.

| # | address | TSV size | reachable insns | `qr3` sites | role | verdict |
|---:|---|---:|---:|---:|---|---|
| 1 | `0x80A96DC0` | 848 | 212 | 0 | `jump_step` spring platform: per-slot bounce phase + 32-entry compression-to-speed table | **ready** |
| 2 | `0x80B179D0` | 736 | 184 | 0 | `daEnUnizoo_c` state handler: 4-way yaw select, reverse-on-slope, scale decay to 1.0 | **ready** |
| 3 | `0x80A87030` | 732 | 183 | 1 | item actor swim step: water level minus `12.0` plus `4.0 * sin(angle/256)` | **ready** |
| 4 | `0x80A03FF0` | 708 | 177 | 0 | `daEnBuku_c` handler: 4-slot latch, 2-unit pull toward the cached anchor | **ready** |
| 5 | `0x80B162C0` | 704 | 176 | 0 | Unizo move handler: three state-name gates, velocity scaled by `3.0` in mode 1 | **ready** |
| 6 | `0x80A22820` | 700 | 175 | 0 | `staffRollBlock` model binder: 14 block sets, three clip tables, second `block_broken` model | **conditional** |
| 7 | `0x80AAD9B0` | 656 | 164 | 0 | `daEnKild_c` handler: 12-byte segment fan + tile-command writes | **ready** |
| 8 | `0x80ACC8F0` | 652 | 163 | 0 | `daEnMouse_c` handler: side toggle, SE 487/488, `264.0` floor, `0x400` angle step | **ready** |
| 9 | `0x80AADC50` | 628 | 157 | 0 | `daEnKild_c` follower: sway integration, `600`-frame retire | **ready** |
| 10 | `0x80AF2CF0` | 624 | 156 | 0 | four-player colour/position marker publisher on two effect objects | **ready** |
| 11 | `0x80A05000` | 616 | 154 | 0 | `daEnBuku_c` release-marker step: SE 473/474, `0.5..2.0` ramp | **ready** |
| 12 | `0x80A78350` | 616 | 154 | 0 | `daEnHnswich_c` handler: switch/course command, three-way state select | **ready** |

**Split: 11 ready, 1 conditional.** Total 8,220 bytes of TSV extent and 2,055
reachable instructions.

The single conditional row has exactly one external fact missing, named in its
**Verdict** line: the scale of the float seed that `0x80167B20` receives. Its
writer (`+0x7FC = (param >> 4) & 0xFFFFFF` at `0x80A22258`) is now pinned, so
only the `2^52` conversion is open. Everything else in that body (strings,
records, clip index, call order) is literal.

Four rows a size ranking would place next are deliberately passed over:

* `0x80A969C0` (844 B) is no longer untranslated: a concurrent lane already holds
  `FN_80A969C0` in `src/game/enemies.rs`.
* `0x80A84F10` (744 B), `0x80A56620` (736 B) and `0x80A56B10` (672 B) are already
  named `fn_<addr>` bodies in `src/game/enemies.rs`.
* `0x80B16060` (608 B) is now the largest remaining row and starts wave 146.

## The `qr3` pool at this wave

`docs/REL_ELF_MAPPING.md` pins **`GQR3 = 0x00050005`** (unsigned 16-bit, scale
0), installed by `main.dol` at `0x8025EEEC`/`0x8025EEF0`. The RAW stream of this
REL contains no `mtspr GQRn`, so every `psq_*` here depends on a `main.dol` word.
With that value every data `psq` in this module has the shape

```
psq_l fD,0(rA),1,qr3      ; W = 1 (single element), I = 3
```

which is one unsigned 16-bit load scaled by `2^0`:

```
fD = (f32)*(u16 *)rA
```

There is **one** `qr3` site among the twelve: `0x80A870F8` in rank 3, the angle
read of the swim bob. It is the only reason the wave-127 table had that row in
class C, and it is written below as `(f32)(u16)`.

Ten `qr3` bodies remain untranslated after this wave, all smaller than rank 12,
so they do not displace any row above. They are the natural continuation of the
`qr3` tranche:

| address | TSV size | `qr3` sites | relation recorded in the docs |
|---|---:|---:|---|
| `0x80B16F40` | 560 | 2 | `0x80B1....` block (`COVERAGE_WAVE143_ENEMIES.md:103`) |
| `0x80A1EBC0` | 520 | 6 | aim/placement family (`COVERAGE_WAVE128_ENEMIES.md:136`) |
| `0x80A06560` | 500 | 2 | `0x80A03C90` family |
| `0x80A56920` | 476 | 4 | firebar emitter (`COVERAGE_WAVE133_ENEMIES.md`) |
| `0x80A88BC0` | 368 | 2 | `0x80A87030` twin |
| `0x80A58D70` | 308 | 1 | jump arc with the `0x8000` half-turn |
| `0x80A80F40` | 264 | 1 | `0x80A80E50` family |
| `0x80A584E0` | 232 | 2 | angle writer |
| `0x80A80E50` | 220 | 1 | `0x80A80F40` family |
| `0x80B22750` | 128 | 1 | GQR quantiser row |

## Reading conventions

* Synthetic base is `0x80A00000`. The REL has five file-backed `PT_LOAD`
  segments, so the `+0x1000` formula of `docs/REL_ELF_MAPPING.md` applies to the
  first one only (`p_offset = 0x10F0`, `p_vaddr = 0x80A000F0`); later segments
  use their own `p_offset`/`p_vaddr`. The later segments start at
  `0x80B2A518`, `0x80B2A7A4`, `0x80B2A7B0` and `0x80B32DC0`, and the
  file-backed image ends at `0x80B6E438`; the last
  `PT_LOAD` is the zero-fill segment `0x80B6E500` (`memsz = 0xB4E0`), so
  anything at or above `0x80B6E500` is BSS and is marked **(bss)**.
* `this` is the incoming `r3` unless stated otherwise; `+0xNNN` are byte offsets
  from `this`. `+0xAC`/`+0xB0`/`+0xB4` are x/y/z (the module's actor convention);
  `+0xDC`/`+0xE0`/`+0xE4` are the three "colour/scale" floats.
* `addi rX,r0,imm` is `li`, `addis rX,r0,imm` is `lis`, `or rX,rY,rY` is `mr`.
  `rlwinm rA,rS,sh,mb,me` rotates `rS` left by `sh` and masks `mb..me`
  (wrapping); `slwi`/`srwi` are the usual aliases, and `rlwinm rA,rS,0,28,31`
  is "low nibble".
* On a `bc`, `bo = 12` branches on a true CR bit and `bo = 4` on a false one;
  `bi & 3` is `LT/GT/EQ/SO`. `cror 2,1,2` (`0x4C411382`, XO 449) after an
  `fcmpo` rewrites `cr0.eq` as `gt|eq`, so the following `bne` reads
  `if !(a >= b)`; `cror 2,0,2` (`0x4C401382`) gives `lt|eq`.
* The paired-single opcodes in this build are `psq_l = 56` and `psq_st = 60`;
  quantiser is `(w >> 12) & 7`, `W` is `(w >> 15) & 1`, displacement is the
  signed 12-bit field. Every `qr0` site in the bodies below is an `f29..f31`
  stack spill (`docs/REL_ELF_MAPPING.md:53-55`).
* A-form float ops: `fadds/fsubs` take the second source from bits 16-20,
  `fmuls/fdivs` from bits 21-25, and `fmadds/fmsubs` print `fD,fA,fC,fB`.
* `fctiwz` + `stfd` + `lwz` is the module's f32-to-int conversion; the
  `0x43300000`/`2^52` (`4503599627370496`) idiom is the unsigned counterpart
  used for float-from-u32.
* `sth`/`lha` pairs store angle words; an angle is in 16-bit turn units and the
  pre-scale to the `0x80237BD0`/`0x80237C40` helpers is `1/256`.
* `bl 0x802DCD48..0x802DCD74` are the `_savegpr` helpers and
  `bl 0x802DCD94..0x802DCDC0` the matching `_restgpr` helpers. They do not take
  arguments and do not clobber `r3..r15`; a Ghidra "64-bit return" from them is
  the saved register pair. Six of the twelve rows below were class C in
  `docs/ANALYSIS_WAVE127_ENEMIES.md` only because of that shape.

## Symbolic helpers used below

Roles are the ledger's (`ledger/functions.csv`) or the role established by the
cited earlier wave.

| address | name / shape | role |
|---|---|---|
| `0x80237BD0` / `0x80237C40` | `sin_angle16(f1)` / `cos_angle16(f1)` | ledgered `resource::bres`; angle in 16-bit turn units, pre-scaled by `1/256` |
| `0x802DCD48`..`0x802DCD74` | `_savegpr_N` | ledgered `runtime::cpp`; no arguments, `r3..r15` survive |
| `0x802DCD94`..`0x802DCDC0` | `_restgpr_N` | ledgered `runtime::cpp` |
| `0x8005F900` | `player_by_slot_A(slot) -> player` | ledgered `game::player`; table-A resolver |
| `0x8005FA60` | `player_by_slot_B(slot) -> player` | ledgered `game::player`; table-B scan |
| `0x80056CF0` | `player_flag(player, bit) -> u32` | ledgered `game::player`; `(-u\|u)>>31` of `+0x1014` |
| `0x80056C70` | `player_flag_set(player, bit)` | ledgered `game::player`; ORs `1<<(bit&31)` |
| `0x8005E6F0` | `player_thunk_flag(rec+0xEA4) -> u32` | ledgered `game::player`; sets `+0x19` |
| `0x8006A4B0` | `vec3_copy(dst, src)` | unledgered `main.dol`; 3-float copy, the translated bodies keep it by address |
| `0x80198C30` | `sound_play_3d(mgr, id, pos, arg)` | ledgered `audio::ax` |
| `0x80197F00` | `sound_play(mgr, id, pos, arg)` | ledgered `audio::ax` |
| `0x80198870` | `sound_play_5(mgr, id, arg, pos, 0, 0)` | unledgered `main.dol`; the 6-argument 3D twin of `0x80198C30` |
| `0x80168080` | `anim_get_f32(element) -> f32` | ledgered `resource::registry`; vtable `+0x20` of the element |
| `0x801680E0` | `anim_set_f32(element, f1)` | ledgered `resource::registry`; vtable `+0x28` |
| `0x80166EE0` | `anim_get_f32_b(holder, 0) -> f32` | unledgered `resource::registry` |
| `0x80166F10` | `anim_commit(holder, f1, 0)` | ledgered `resource::registry` |
| `0x80169D90` | `child_group_bind(group, pair, block, kind, a, b)` | ledgered `resource::registry` |
| `0x80167420` / `0x80166830` | `group3_bind(...)` / `group2_bind(...)` | ledgered `resource::registry` |
| `0x80167800` / `0x80166BD0` | `group3_commit(...)` / `group2_commit(...)` | ledgered `resource::registry` |
| `0x80169F80` | `group_attach(group, child, f1)` | ledgered `resource::registry` |
| `0x80167B20` | `group_set_frame(group, index, f1)` | ledgered `resource::registry` |
| `0x80064BF0` | `child_commit_word(rec)` | unledgered `main.dol` |
| `0x80064D10` | `actor_retire(this, flag)` | unledgered `main.dol` |
| `0x80064F50` | `actor_frame_commit(this, 0)` | unledgered `main.dol` |
| `0x80065110` | `actor_contact_test(this) -> u32` | unledgered `main.dol` |
| `0x8006CD90` / `0x8006CDE0` | `actor_matrix_step(this)` / `actor_physics_step(this)` | unledgered `main.dol` |
| `0x800955F0` | `actor_orbit_follow(this)` | ledgered `game::stage`; sin/cos orbit into `+0xAC/+0xB0/+0xB4` |
| `0x80096E60` | `actor_status_flags(this) -> u32` | ledgered `game::stage`; bit 0 = ground contact, bit 1 = the `0x8006EF40` bit-2 contribution, bit 2 = the `0x80097050` bit-4 contribution |
| `0x80096F60` | `actor_ground_block(this)` | ledgered `game::stage` |
| `0x80097050` | `actor_contact_block(this)` | ledgered `game::stage` |
| `0x8006EF40` | `actor_ground_apply(holder, mask)` | unledgered `main.dol` |
| `0x80070CB0` | `ground_query(holder) -> u32` | ledgered `game::stage`; `+0x88 & 0x1FE000` |
| `0x80070760` / `0x800707E0` | `height_band(rec)` / `height_positive(rec)` | ledgered `game::stage` |
| `0x80077860` | `tile_command(manager, x, y, 0, 0)` | ledgered `game::stage`; tile-command kind 2..4 |
| `0x8007F950` | `splice_pair(rec)` | ledgered `game::stage` |
| `0x800B2EE0` | `rand_below(n) -> u32` in `[0, n)` | ledgered `game::stage`; duplicate of `0x800B2ED0` |
| `0x8015F4F0` | `step_short_clamped_wrap(v*, target, step)` | ledgered `game::task` |
| `0x8013E630` / `0x80170A00` | `player_notice(player)` / `vec3_normalise(v*)` | unledgered `main.dol`; `r3` only, and a 3-float in/out |
| `0x800B1100` | `stage_layer_effect(a, layer, 1, f1, f2)` | unledgered `main.dol` |
| `0x800D8800` | `course_slot_command(mgr, index, value)` | unledgered `main.dol` |
| `0x800E41C0` | `course_command(mgr, a, b, c, d, e, f)` | unledgered `main.dol`; 7 arguments |
| `0x8019BC50` | `sound_cue(mgr, cue)` | ledgered `audio::ax` |
| `0x800690C0` / `0x800690E0` | `holder_block_init(dst, -1, name, 0, size)` / finisher | unledgered `main.dol` |
| `0x800DF180` | `bres_load(manager+4, name, path)` | ledgered `resource::bres` |
| `0x80239E30` / `0x8023A200` / `0x8023A270` | `model_node` / `AnmTexPat` / `AnmTexSrt` lookup | ledgered `resource::bres` |
| `0x80429DD0` | stage/height manager | `main.dol`; `+0x8FEC4` water level |
| `0x8042A100` | water manager | `main.dol`; `+0x8C0` horizontal drift float |
| `0x80429D5C` / `0x80429D60` | sound manager (3D) / sound manager | `main.dol` |
| `0x8042A038` | resource manager | `main.dol` |
| `0x8042A000` | course/player manager | `main.dol` |
| `0x8042A078` | course command object | `main.dol`; `+0x6C8` layer byte |
| `0x80429F30` / `0x80429F80` | effect manager / flag word | `main.dol`; `+0x8` of the flag word suppresses the cue |
| `0x8042A1C8` | stage layer table | `main.dol`; `+0x120E` current layer byte |
| `0x80377C48` | resource-name object | `main.dol` |

---

## 1. `0x80A96DC0` - spring-platform bounce phase and launch speed

* **Address:** `0x80A96DC0`
* **TSV size:** 848 bytes (212 words)
* **Reachable body:** 212 instructions, the whole extent with no gaps; 13 direct
  `bl` (two of them `_savegpr_27`/`_restgpr_27`), two `bctrl` (both the player
  vtable slot `+0x3FC`), 18 conditional branches, six direct `b` joins, one
  `blr` at `0x80A9710C`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a96dc0_FUN_80a96dc0.c`
  - the export loses the first ~10 instructions behind the `f31` spill and
    types the body `void FUN_80a96dc0(undefined4, undefined4, char)`; the RAW
    stream is authoritative.
* **Proposed signature:**
  `pub unsafe fn fn_80a96dc0(this: usize, mode: u32, slot: i32)`
  - `this` = the platform actor; `mode` (`r4`) selects the player table;
    `slot` (`r5`, signed byte) is the player slot `0..3`.
* **Callers in this REL:** one `bl` at `0x80A971EC`, inside `0x80A97180`
  (476 B, untranslated), which loops the four slots and calls
  `fn_80a96dc0(this, *(u16 *)(this + 0x61C + 2*i), i)` for every non-zero
  per-slot hit flag. So the `mode` argument is the raw hit-flag word, not a
  constant, and the callee's `mode == 1` test selects between the two player
  tables. The address is not in any `.data` table.
* **Family evidence:** `src/game/enemies.rs:42595` documents the run
  `0x80A959A0..0x80A977C0` as the `jump_step` spring platform
  (`g3d/jump_step.brres` at `0x80B51254`), with `+0x5B0 + i*4` = per-slot speed,
  `+0x61C + i*2` = hit flag and `+0x624 + i*2` = bounce phase. `0x80A96990`
  (ledgered) is the per-slot reset this body calls.
* **Constants:**
  - base `r31 = 0x80B2EC90`
  - `+0x44` -> `0x80B2ECD4` = `0.0`
  - `+0x140` -> `0x80B2EDD0` = `6.0`
  - `+0x144` -> `0x80B2EDD4` = `4.43`
  - `+0xC0` -> `0x80B2ED50`: 32 `f32` entries, indexed by the player's height
    difference:

    ```
    4.43 4.43 4.43 4.43 4.40 4.38 4.37 4.37 4.36 4.36 4.35 4.35
    4.34 4.33 4.31 4.30 4.29 4.28 4.26 4.24 4.23 4.22 4.21 4.20
    4.18 4.16 4.15 4.13 4.11 4.09 4.08 4.06
    ```

    The body copies the table to the stack in 16 double-word iterations before
    indexing; that copy is a compiler artefact of `lfsx` through a saved base,
    not observable behaviour.
* **Behaviour:**

  ```
  player = if mode == 1 { 0x8005F900(slot) } else { 0x8005FA60(slot) }
  if (player->+0x274 & 0x3C000000) != 0:                return
  if 0x80056CF0(player, 4) != 0: 0x80A96990(this, slot); return
  if 0x80056CF0(player, 6) != 0: 0x80A96990(this, slot); return
  player->+0x10C = 0.0
  armed = 0
  if 0x8005E6F0(player + 0xEA4) != 0:
      if this->+0x624[slot] != 3:
          if this->+0x632 <= 6:
              d = (i32)(player->+0xB0 - this->+0x5AC)
              this->+0x5B0[slot] = if d < 32 { table32[d] } else { 6.0 }
          else:
              this->+0x5B0[slot] = 4.43
          this->+0x624[slot] = 1
          armed = 1
  else if this->+0x632 == 6 and this->+0x624[slot] == 0:
      this->+0x5B0[slot] = 6.0
      this->+0x624[slot] = 2
      armed = 1
  if this->+0x632 <= 6 and armed:
      player->+0x10C = 0.0
      phase = this->+0x624[slot]
      if phase == 2:
          0x8006A4B0(pos, this + 0xAC)
          0x80198C30(*(0x80429D5C), 663, pos, 0)
          player->vtable[+0x3FC](player, 1, this->+0x5B0[slot], this->+0x5EC, 2, 0)
      else if phase == 1:
          0x8006A4B0(pos, this + 0xAC)
          0x80198C30(*(0x80429D5C), 664, pos, 0)
          player->vtable[+0x3FC](player, 1, this->+0x5B0[slot], this->+0x5EC, 1, 0)
          this->+0x624[slot] = 3
  if this->+0x624[slot] == 0:
      player->+0xAC = this->+0x5C0[slot] + (this->+0xAC - this->+0x5DC[slot])
  if player->+0xB0 < this->+0x5A4:
      player->+0xB0 = this->+0x5A4
  0x80056C70(player, 0x86)
  if this->+0x632 == 1:
      this->+0x624[slot] = 0
      this->+0x61C[slot] = 0
      this->+0x608[slot] = 0
      this->+0x5B0[slot] = 0.0
  ```

  The `vtable[+0x3FC]` call is on the **player** object (its vtable is loaded
  from `player+0x60`); the two float arguments are the per-slot speed and
  `this->+0x5EC`, and `r4 = 1`, `r5 = 1|2`, `r6 = 0`.
* **Verdict: ready.** Every constant lives in the image; the only external
  object is the player vtable, which is the module's normal player contract.

## 2. `0x80B179D0` - `daEnUnizoo_c` four-way yaw select and scale decay

* **Address:** `0x80B179D0`
* **TSV size:** 736 bytes (184 words)
* **Reachable body:** 184 instructions, the whole extent; 10 direct `bl`
  (`_savegpr_26`/`_restgpr_26` included), one `bctrl` (the `+0x5C4` anim slot
  `+0x14`), 22 conditional branches, eleven `b` joins, one `blr`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b179d0_FUN_80b179d0.c`
  - `/* WARNING: Removing unreachable block (ram,0x80b17c90) */`; the export
    also drops the `f31` prologue.
* **Proposed signature:** `pub unsafe fn fn_80b179d0(this: usize)`
* **Callers in this REL:** none by `bl`; the entry is a state handler in the
  `daEnUnizoo_c` table at `0x80B6A450` (entry 9 of 10, between `0x80B179C0` and
  `0x80B178B0`).
* **Family evidence:** the class strings `daEnUnizoo_c::StateID_Wait` are at
  `0x80B6A508`, 0x58 bytes after the table; `0x80B178B0` (translated) is the
  state initialiser of the same object, `0x80B17670` the two-axis facing step,
  `0x80B174F0` the `1.5..2.0` vertical ramp. All four share the float block
  `0x80B32230`.
* **Constants:** base `r29 = 0x80B32230`
  - `+0x84` -> `0x80B322B4` = `1.0` (scale floor)
  - `+0xAC` -> `0x80B322DC` = `0.0` (speed reset)
  - `+0xE0` -> `0x80B32310` = `0.125` (scale step)
  - `+0x104` -> `0x80B32334` = `70.0`, `+0x108` -> `0x80B32338` = `66.0`
    (anim window)
  - `+0x10C` -> `0x80B3233C` = `0.1`
  - `+0x110`..`+0x11C` -> `0x80B32340/44/48/4C` = `0.39`, `0.78`, `1.17`,
    `1.56` (the four yaw speeds)
  - `+0x120` -> `0x80B32350` = `0.02`
  - the four yaw steps are the immediates `288`, `320`, `352`, `384`
    (`0x120`, `0x140`, `0x160`, `0x180`)
* **Behaviour:**

  ```
  if this->+0x644 != 0: this->+0x644 -= 1
  if this->+0x644 != 0: 0x80B174F0(this); return
  r = 0x80B174F0(this)
  if r == 0:                                   // scale decays to 1.0
      if this->+0xDC != 1.0:
          s = this->+0xDC - 0.125
          this->+0xDC = s
          if s <= 1.0: s = 1.0; this->+0xDC = s
          this->+0xE0 = s; this->+0xE4 = s
  f = 0x80166EE0(this + 0x5C4, 0)
  if f < 66.0 or f > 70.0: vtable(+0x5C4)->+0x14(+0x5C4)   // seek the clip
  0x8006CD90(this); 0x8006CDE0(this); 0x800955F0(this)
  if (0x80096E60(this) & 1) != 0:
      band  = 0x80070760(this + 0x1EC)
      up    = 0x800707E0(this + 0x1EC)
      this->+0x11C = 0.1
      switch band:
        1: f31 = 0.39; step = 288
        2: f31 = 0.78; step = 320
        3: f31 = 1.17; step = 352
        4: f31 = 1.56; step = 384
      if up != 0: f31 = -f31
      if this->+0x640 == 0:
          this->+0x648 = 1
          this->+0xF4 = f31
          this->+0x61C = step
          this->+0x65C = up
      else:
          if |this->+0xF4| < |f31|:
              this->+0xF4 = f31
              this->+0x61C = step
              this->+0x65C = up
          if up != 0: this->+0x102 += this->+0x61C
          else:       this->+0x102 -= this->+0x61C
      this->+0x640 = 1
  else:
      if this->+0x648 != 0:
          if this->+0x65C != 0: this->+0x102 += this->+0x61C
          else:                 this->+0x102 -= this->+0x61C
          this->+0x648 = 0
      this->+0x11C = 0.02
      this->+0xF4  = 0.0
  if this->+0x648 == 0:
      if this->+0x640 != 0:
          0x80B17670(this)                      // two-axis facing step
      else:
          this->+0x100 += this->+0x664
          if this->+0x66B != 0: this->+0x102 += this->+0x666
          else:                 this->+0x102 -= this->+0x666
  ```

  Field roles agree with `fn_80b17670`'s Rust body: `+0x100`/`+0x102` are the
  two live facing angles (`i16`), `+0x61E`/`+0x620` their saved copies,
  `+0x61C` the angle step, `+0xF4` the signed yaw speed.
* **Verdict: ready.** The band table `0x80070760` returns `0..4`; every branch
  and clamp is literal.

## 3. `0x80A87030` - item-actor swim bob with the one `qr3` site

* **Address:** `0x80A87030`
* **TSV size:** 732 bytes (183 words)
* **Reachable body:** 183 instructions, the whole extent; 20 direct `bl` (two
  `_savegpr`/`_restgpr`), one `bctrl` (`+0xD4`), 21 conditional branches, three
  `b` joins, one `blr`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a87030_FUN_80a87030.c`
  - two `/* WARNING: Removing unreachable block ... */` notes at
    `0x80A872E8`/`0x80A870F8`; the `qr3` load is rendered as a GQR-parameter
    preamble.
* **Proposed signature:** `pub unsafe fn fn_80a87030(this: usize)`
* **Callers in this REL:** none by `bl`; a state handler of the `0x80A86Fxx`
  item actor (table at `0x80B4E918`, between `0x80A87020` and `0x80A86FF0`).
  That table sits in the `Wm_ob_item*` effect block
  (`Wm_ob_itemsealandsmk` `0x80B4E41C`, `Wm_ob_itempropeller` `0x80B4E450`,
  `Wm_ob_itemfall` `0x80B4E47C`).
* **Constants:** base `r31 = 0x80B2E420`
  - `+0x16C` -> `0x80B2E58C` = `8.0`
  - `+0x94` -> `0x80B2E4B4` = `0.0`
  - `+0x1A0` -> `0x80B2E5C0` = `4.0`
  - `+0x284` -> `0x80B2E6A4` = `0.00390625` (`1/256`)
  - `+0x1C8` -> `0x80B2E5E8` = `-1.0`
  - water level: `*(u32 *)0x80429DD0 + 0x8FEC4` (`r3` is `addis 9` then
    `lfs -316`), the documented `+0x8FEC4` float
  - drift: `*(u32 *)0x8042A100 + 0x8C0`, the documented horizontal drift float
* **The `qr3` site:** `0x80A870F8  psq_l f1,0(r4),w1,qr3` with `r4 = r1 + 8`
  and `r1+8` holding the `i16` copy of `this->+0xDF0` (`sth r0,8(r1)` at
  `0x80A870F4`). Under `GQR3 = 0x00050005` (`W = 1`, `I = 3`) this is one
  unsigned 16-bit load, so the source is `(f32)(u16)this->+0xDF0`, which is
  then multiplied by `1/256` at `0x80A87100` before `0x80237BD0`.
* **Behaviour:**

  ```
  0x8006CDE0(this); 0x800955F0(this); 0x80A85C30(this); 0x80A85F10(this)
  if this->+0xDD6 != 0: return                      // hold latch
  if this->+0xDA0 != 0:                             // on the water surface
      level = *(f32 *)(*(u32 *)0x80429DD0 + 0x8FEC4)
      if level - 8.0 >= this->+0xB0:
          this->+0xEC = 0.0; this->+0xE8 = 0.0
          this->+0xAC += *(f32 *)(*(u32 *)0x8042A100 + 0x8C0)
          a = this->+0xDF0
          this->+0xB0 = ((level - 8.0) - 4.0)
                      + 4.0 * sin16((f32)(u16)a * (1/256))
          this->+0xDF0 = a + 1024
      goto tail
  r = 0x80096E60(this)
  0x80A86160(this)
  if (r & 1) != 0:                                  // ground contact
      this->+0xDF3 = 1; this->+0xEC = 0.0; this->+0xE8 = 0.0
      if this->+0xE04 == 0 and this->+0xDF4 == 0:
          0x80A86070(this); 0x80A865E0(this)        // smoke + burst stage
  this->+0xE04 = 0
  if (r & 2) != 0: this->+0xEC = -1.0
  if this->+0xDCA == 14 and armed:
      if this->+0xD6C != 0: this->+0xD6C -= 1
      if this->+0xD6C == 0: this->vtable[+0xD4](this, 0x80B737B8)   // (bss)
  0x80A86470(this)
  if this->+0xDF3 == 0 and this->+0xDA4 == 2: 0x80A85CD0(this)
  tail:
  if 0x80065110(this) != 0: 0x80A85A20(this); return
  if this->+0xDA0 == 0: this->+0xB4 = 0x80A85D20(this)
  if this->+0xD94 == 3:
      if this->+0xD9C == 0:
          if this->+0xEC <= 0.0: 0x80A85EF0(this); this->+0xD94 = 1
      else:
          if this->+0xDCC != 0: this->+0xDCC -= 1
          if this->+0xDCC == 1: 0x80A85EF0(this); this->+0xD94 = 1
  0x80A85590(this); 0x80A85520(this)
  if 0x80A85650(this) == 0: 0x80A85AD0(this)
  ```

  `armed` is the `r30` breadcrumb set when the ground branch fires; it is
  tested only in the `+0xDCA == 14` block. `0x80A85650` is rank 10 of
  `docs/SPEC_WAVE144_ENEMIES.md`.
* **Verdict: ready.** The single `qr3` site is a plain `(f32)(u16)` conversion
  and the two manager offsets are the ones `docs/SPEC_WAVE143_ENEMIES.md:212`
  and `docs/SPEC_WAVE144_ENEMIES.md:190` already pin.

## 4. `0x80A03FF0` - `daEnBuku_c` four-slot latch and 2-unit anchor pull

* **Address:** `0x80A03FF0`
* **TSV size:** 708 bytes (177 words)
* **Reachable body:** 177 instructions, the whole extent; 8 direct `bl` (two
  `_savegpr_24`/`_restgpr_24`), five `bctrl` (player slots `+0x3D4`, `+0x3F4`
  and the actor slot `+0xD4`), 15 conditional branches, two `b` joins, one
  `blr`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a03ff0_FUN_80a03ff0.c`
  - three "unreachable block" warnings; the body is typed `void FUN(...)`, its
    incoming `f1` is printed as `extraout_f1`.
* **Proposed signature:** `pub unsafe fn fn_80a03ff0(this: usize, frame: f32)`
  - `frame` is the incoming `f1`, the `+0x584` clip frame of the same actor
    (rank 11 passes it).
* **Callers in this REL:** one `bl` at `0x80A05204` (rank 11).
* **Family evidence:** `daEnBuku_c::StateID_Swim` / `SrcSwim` / `AttackSwim` /
  `AttackWait` are at `0x80B33A0C..0x80B33A7C`; `g3d/bakubaku.brres` at
  `0x80B33674`; the state table holding `0x80A05000` is at `0x80B339B4`.
  `0x80A03ED0` (translated) walks the same slots (`+0x6D8`, `0x8013E7C0`), and
  `0x80A03F40` (translated) is the `+0x6B4 = 0x3D` timer writer.
* **Constants:** base `r28 = 0x80B2A910`
  - `+0x84` -> `0x80B2A994` = `0.0`
  - `+0xA4` -> `0x80B2A9B4` = `42.0`, `+0xA8` -> `0x80B2A9B8` = `44.0`
  - `+0xAC` -> `0x80B2A9BC` = `1500.0`
  - `+0xB0` -> `0x80B2A9C0` = `2.0` (pull distance)
  - `+0xB4` -> `0x80B2A9C4` = `64.0`
  - timer immediate `61` (`0x3D`) written to `+0x6B4[slot]`
  - descriptors `0x80B7E7B8` and `0x80B7E878` **(bss)**
* **Slot arrays:** `+0x6C8[4]` (`i8`, table-A slot), `+0x6CC[4]` (`i8`,
  table-A slot), `+0x6D0[4]` (`u8`, gate read only by rank 11), `+0x6D4[4]`
  (`u8`, latch), `+0x6D8[4]` (`u8`), `+0x6BC[4]` (`u16`), `+0x6B4[4]` (`u16`),
  `+0x6C4[4]` (`u8`).
* **Behaviour:**

  ```
  if this->+0x6DE != 0:
      for i in 0..3:
          s = (i8)this->+0x6C8[i]
          if s == -1: continue
          if this->+0x6D4[i] != 0: continue
          p = 0x8005F900(s)
          if p->vtable[+0x3D4](p) != 0: continue
          if p->vtable[+0x3F4](p, this, 12) != 0: this->+0x6D4[i] = 1
  if 42.0 <= frame <= 44.0:
      this->+0xB4 = 1500.0
      if 0x80A03F40(this) != 0:
          this->+0x6E1 = this->+0x348
          this->vtable[+0xD4](this, 0x80B7E7B8)
      return
  for i in 0..3:
      s = (i8)this->+0x6CC[i]
      if s != -1:                                  // else skip the zero stores
          p = 0x8005F900(s)
          if this->+0x6BC[i] == 0 and this->+0x6D4[i] != 0:
              dx = this->+0x67C - p->+0xAC
              dy = this->+0x680 - p->+0xB0
              pz = p->+0xB4
              0x8013E630(p)
              this->+0x6BC[i] = 1
              this->+0x6B4[i] = 61
              this->+0x6C4[i] = this->+0x6CC[i]
              this->+0x6D8[i] = 1
              n = 0x80170A00(&(dx, dy, 0.0))      // unit vector
              p->+0xAC = p->+0xAC + 2.0 * n.x
              p->+0xB0 = p->+0xB0 + 2.0 * n.y
              p->+0xB4 = pz
          p->+0x10C = 0.0                       // reached from both guards
          p->+0xE8 = 0.0; p->+0xEC = 0.0; p->+0xF0 = 0.0
  if frame >= 64.0:
      if 0x80A03F40(this) != 0:
          this->+0x6E1 = this->+0x348
          this->vtable[+0xD4](this, 0x80B7E7B8)
      else:
          this->vtable[+0xD4](this, 0x80B7E878)
  ```

  Note the four zero stores after the pull: the compiler placed them after the
  `+0x6BC`/`+0x6D4` merge point, so they run for every slot whose `+0x6CC` is
  not `-1`, whether or not the pull itself fired. That is why they are drawn
  outside the inner `if`.
* **Verdict: ready.** All twelve constants are in the image, the two latches are
  literal, and the only external contract is the player vtable.

## 5. `0x80B162C0` - Unizo move handler with three state-name gates

* **Address:** `0x80B162C0`
* **TSV size:** 704 bytes (176 words)
* **Reachable body:** 176 instructions, the whole extent; 7 direct `bl`
  (including `0x801680E0` twice, `0x80166F10`, `0x80B16F40`, `0x80B16BA0`,
  `0x80B171B0` and `0x80064F50`), 13 `bctrl` (nine of them through a two-level
  vtable chain), 13 conditional branches, two `b` joins, one `blr`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b162c0_FUN_80b162c0.c`
  - the export recovers the whole body (`undefined4 FUN_80b162c0(int)`), which
    makes this the cleanest cross-check in the wave.
* **Proposed signature:** `pub unsafe fn fn_80b162c0(this: usize) -> u32`
  - returns `1` unconditionally.
* **Callers in this REL:** none by `bl`; referenced from two duplicate state
  tables, `0x80B69F08` and `0x80B6A188`, in the `unizo` / `unizoBig` data block.
* **Constants:** base none; the only float used is `0x80B322C8` = `3.0`
  (addressed as `lis 0x80B3; lfs 8904`), plus the two `1.0` words at
  `0x80B69E44` and `0x80B69E48` (identical values, two load sites) and `0.1`
  at `0x80B3233C`.
* **Vtable chain:** `this->+0x394` is a state machine; slot `+0x28` returns the
  current *state object*, whose slot `+0x14(desc)` answers "is this state".
  Three descriptors located at `0x80B78EB8`, `0x80B78EF8` and `0x80B78F38`
  (0x40 bytes apart) **(bss)**.
* **Behaviour:**

  ```
  if this->+0x634 != 0: this->+0x634 -= 1
  if this->+0x628 == 0:
      0x801680E0(this + 0x58C, 1.0)
      0x80166F10(this + 0x5C4, 1.0, 0)
  else:
      0x801680E0(this + 0x58C, 1.0)
  this->vtable_chained(+0x394, +0x10)()             // state-machine update
  in_first   = state_is(this, desc1) || state_is(this, desc2)
  in_all     = in_first || state_is(this, desc3)
  if in_all: 0x80B16F40(this)
  if this->+0x628 == 1:
      this->+0x170 = 3.0 * this->+0x610 * this->+0xE0
      this->+0x16C = 3.0 * this->+0x610 * this->+0xDC
  else:
      this->+0x170 = this->+0x610 * this->+0xE0
      this->+0x16C = this->+0x610 * this->+0xDC
  0x80B16BA0(this)
  in_all = state_is(this, desc1) || state_is(this, desc2) || state_is(this, desc3)
  if in_all: 0x80B171B0(this)
  0x80064F50(this, 0)
  return 1
  ```

  The predicate idiom is `neg r0,r3; or r0,r0,r3; rlwinm r0,r0,1,31,31` followed
  by `bne`, i.e. "result was non-zero".
* **Verdict: ready.** `0x80B16F40`, `0x80B16BA0` and `0x80B171B0` are the
  Unizo step helpers; `0x80B171B0` is translated, the other two are the next
  tranche's Unizo rows.

## 6. `0x80A22820` - `staffRollBlock` model binder (conditional)

* **Address:** `0x80A22820`
* **TSV size:** 700 bytes (175 words)
* **Reachable body:** 175 instructions, the whole extent; 23 direct `bl`
  (including `0x800DF180` twice, `0x80239E30` twice and three pointer-table
  lookups), one `bctrl` (`+0x18` of the first child group), **no conditional
  branch and no `b` join** — the binder is straight-line code — one `blr`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a22820_FUN_80a22820.c`
  - the export models the `f31` spill as a `GQR0` parameter block and prints
    the whole prologue as quantiser arithmetic; the clip index and every string
    are still readable.
* **Proposed signature:** `pub unsafe fn fn_80a22820(this: usize)`
* **Callers in this REL:** one `bl` at `0x80A222DC`, inside the unlisted body
  that starts at `0x80A22200` (126 instructions, ~`0x1F0` bytes; there is no
  `functions.tsv` row between `0x80A22110` and `0x80A223F0`). That body is the
  only writer of `+0x7FC`: at `0x80A22258` it stores
  `+0x7FC = (*(u32 *)(this + 4) >> 4) & 0xFFFFFF`, after splitting the same
  constructor word into a mode nibble (`+0x7F8`/`+0x800`) and `+0x7EC`.
* **Strings (in image):**
  - `0x80B39960` `"g3d/staffRollBlock.brres"`, `0x80B3997C` `"staffRollBlock"`
  - `0x80B3998C` `"g3d/block_broken.brres"`, `0x80B399A4` `"block_broken"`
  - `0x80B3980C..0x80B398A8`: fourteen 12-byte records
    `block_00 .. block_013`, each `{ "block_0N", "k_0N", "N" }`
  - three identical 14-pointer arrays at `0x80B398B8`, `0x80B398F0`,
    `0x80B39928` that index those records
* **Constants:** `0x80B2B7CC` = `0.0` (`lfs`), `0x80B2B7B0` = `1.0`, and
  `0x80B2B7D0` = the `2^52` double `4503599627370496.0`.
* **Behaviour:**

  ```
  0x800690C0(this + 0x698, 0xFFFFFFFF, *(0x80377C48), 0, 0x20)
  model = 0x800DF180(*(0x8042A038) + 4, "staffRollBlock",
                     "g3d/staffRollBlock.brres")
  this->+0x6B4 = model
  q = (i32)this->+0x7FC / 16                     // truncating, 0..13
  this->+0x6B4 = 0x80239E30(this + 0x6B4, table1[q])
  0x8000F1B0(&local3, 3, &suffix3)               // suffix3 = 5F 69 69 00
  0x8000F0F0(&local2, 2, &suffix2)               // suffix2 = 1E 2D 28 00
  0x80169D90(this + 0x6BC, &pair, this + 0x698, 0x327, 1, 0)
  0x80064BF0(this + 0x6BC)
  srt = 0x8023A270(this + 0x6B4, table2[q])      // AnmTexSrt
  0x80167420(this + 0x740, ...)
  0x80167800(this + 0x740, this + 0x6BC, ...)
  0x80169F80(this + 0x6BC, this + 0x740, 0.0)
  0x80167B20(this + 0x740, 0, <seed from +0x7FC>)
  pat = 0x8023A200(this + 0x6B4, table3[q])      // AnmTexPat
  0x80166830(this + 0x76C, ...)
  0x80166BD0(this + 0x76C, this + 0x6BC, ...)
  vtable(+0x6BC)->+0x18(+0x6BC, +0x76C)
  0x80166F10(this + 0x76C, 1.0, 0)
  this->+0x6B8 = 0x800DF180(*(0x8042A038) + 4, "block_broken",
                            "g3d/block_broken.brres")
  0x80169D90(this + 0x6FC,
              &0x80239E30(this + 0x6B8, "block_broken"), this + 0x698, 0x20, 1, 0)
  0x80064BF0(this + 0x6FC)
  0x800690E0(this + 0x698)
  ```

  The quotient is computed with the compiler's signed-division sequence
  (`rlwinm`/`subf`/`srawi`/`addze`), which truncates toward zero, so for the
  non-negative `+0x7FC` written by `0x80A22200` the index is exactly
  `+0x7FC >> 4`, i.e. `(param >> 8) & 0xFFFFF` where `param = *(u32 *)(this + 4)`.
  The same block then converts a correction word built from `(+0x7FC & 0xF)` and
  bit 30 of `+0x7FC` through the `0x43300000`/`2^52` idiom and passes the result
  as the `0x80167B20` frame argument.
* **Verdict: conditional.** Blocker: the scale of the `0x80167B20` seed. The
  writer is now known (`+0x7FC = (param >> 4) & 0xFFFFFF` at `0x80A22258`), so
  the index `q` and the source nibble `(param >> 4) & 0xF` are pinned; what is
  still open is the `2^52` conversion itself, whose operand is the
  `xoris`-flipped correction word rather than the quotient, so the resulting
  `f32` cannot be stated without also fixing the writer's `param` encoding
  (mode nibble in bits 0-3, sub-index in bits 4-7, block index in bits 8+).
  Everything else (strings, fourteen records, three tables, holder order) is
  literal and ready.

## 7. `0x80AAD9B0` - `daEnKild_c` segment fan and tile commands

* **Address:** `0x80AAD9B0`
* **TSV size:** 656 bytes (164 words)
* **Reachable body:** 164 instructions, the whole extent; 12 direct `bl`
  (`_savegpr_24`/`_restgpr_24` included, `0x80077860` three times and
  `0x800B2EE0` six times), two conditional branches (the loop test and the
  store guard), one `b` join, one `blr`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aad9b0_FUN_80aad9b0.c`
  - the export prints the body as `void FUN_80aad9b0(void)` because of the save
    helper in front of it; it does resolve all three random tables.
* **Proposed signature:** `pub unsafe fn fn_80aad9b0(this: usize)`
* **Callers in this REL:** none by `bl`; state handler 3 of the `daEnKild_c`
  table at `0x80B54400`/`0x80B5440C`/`0x80B54418` (the three entries are
  `0x80AADC40`, `0x80AADC50`, `0x80AAD9B0`). Class and state strings:
  `daEnKild_c::StateID_Wait/Slide/Fall` at `0x80B5441C..0x80B54464`,
  `g3d/killer_king.brres` at `0x80B544CC`, nodes `eye_left`/`eye_right` at
  `0x80B544B4`, effects `Wm_en_killervanish`/`Wm_en_kingkiller` at
  `0x80B544F0`/`0x80B54504`.
* **Constants:** base `r31 = 0x80B2F880`
  - `+0x24` -> `0x80B2F8A4` = `16.0` (segment spacing)
  - `+0x30` -> `0x80B2F8B0` = `8.0` (x bias)
  - `+0x88` -> `0x80B2F908`: eight `f32` sway pairs
    `-7.0, 7.0, -6.0, 6.0, -6.5, 6.5, -7.5, 7.5`
  - `+0xA8` -> `0x80B2F928`: four `f32` lateral speeds `4.0, 3.5, 3.0, 4.5`
  - `+0xB8` -> `0x80B2F938`: two `f32` seeds `0.0625, -0.0625`
  - `+0xE36` timer = `600` (`li 0x258`)
* **Random draws:** `0x800B2EE0(4)` is a bounded draw in `[0, 4)`; the code
  only ever uses `rand & 0x7F` multiplied by 2 (giving `0, 2, 4, 6`) or the raw
  value as a word index, which is exactly what makes the 8- and 4-entry tables
  above sufficient. `+0xE3E` is the 0/1 side offset added to the table index.
* **Behaviour:**

  ```
  x  = this->+0xAC ; y = this->+0xB0 ; z = this->+0xB4
  top = y + 16.0 * this->+0xE0
  gx  = (i32)(x - 8.0) & 0xFFF0
  this->+0xBAC = x ; this->+0xBA0 = x
  for i in 0 .. this->+0xE20 - 1:
      r = this->+0xE3E + 2 * 0x800B2EE0(4)
      rec = this + 12*i
      rec->+0xBB8 = x
      rec->+0xBBC = top
      rec->+0xC90 = sway8[r]
      rec->+0xC94 = speed4[0x800B2EE0(4)]
      0x80077860(*(0x80429DD0), gx, (-((i32)(16.0 + top) & 0xFFF0)) & 0xFFFF, 0, 0)
      top = top + 16.0 * this->+0xE0
  this->+0xBB0 = top
  0x80077860(*(0x80429DD0), gx, (-((i32)(16.0 + top) & 0xFFF0)) & 0xFFFF, 0, 0)
  top = top + 16.0 * this->+0xE0
  0x80077860(*(0x80429DD0), gx, (-((i32)(16.0 + top) & 0xFFF0)) & 0xFFFF, 0, 0)
  this->+0xBA4 = top
  this->+0xC78 = sway8[this->+0xE3E + 2 * 0x800B2EE0(4)]
  this->+0xC84 = sway8[this->+0xE3E + 2 * 0x800B2EE0(4)]
  this->+0xC7C = speed4[0x800B2EE0(4)]
  this->+0xC88 = speed4[0x800B2EE0(4)]
  this->+0xDF8 = seed2[this->+0xE3E]
  this->+0xE3A = 1
  this->+0xE36 = 600
  0x8007F950(this + 0xA60)
  this->+0x390 = 1
  ```

  The three trailing tile commands walk the same `gx` column upward, which is
  the "cap" of the fan.
* **Verdict: ready.** The `0x80077860` tile write, the three table bases and
  the `[0,4)` draw contract are all pinned; the record stride `12` matches
  rank 9.

## 8. `0x80ACC8F0` - `daEnMouse_c` side toggle and `264.0` floor

* **Address:** `0x80ACC8F0`
* **TSV size:** 652 bytes (163 words)
* **Reachable body:** 163 instructions, the whole extent; 17 direct `bl`
  (`_savegpr_27`/`_restgpr_27` included, `0x8006A4B0` twice, `0x80198870`
  twice), six `bctrl` (the state slot `+0xD4`, the parent state machine and the
  `+0x548` animation slot), 10 conditional branches, four `b` joins, one `blr`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80acc8f0_FUN_80acc8f0.c`
  - the export prints the body as `void FUN_80acc8f0(void)`; the `f31` save is
    a `psq_st ...,qr0` spill, not quantisation.
* **Proposed signature:** `pub unsafe fn fn_80acc8f0(this: usize)`
* **Callers in this REL:** none by `bl`; state handler 8 of the table at
  `0x80B5A608` (entries `0x80ACC510`, `0x80ACC520`, `0x80ACC4B0`,
  `0x80ACC730`, `0x80ACC740`, `0x80ACC710`, `0x80ACC8E0`, `0x80ACC8F0`).
  `daEnMouse_c::StateID_Walk/DieFumi/DieFall` are at `0x80B5A6B4..`, and
  `g3d/mouse.brres` at `0x80B5A308`; `0x80ACADD0` (translated) is the clip
  binder whose name table is `0x80B30628` = `{"walk", "panic"}`.
* **Constants:** base `r29 = 0x80B30628`
  - `+0x1C` -> `0x80B30644` = `1.0`, `+0x20` -> `0x80B30648` = `0.0`
  - `+0x88` -> `0x80B306B0` = `0.0`
  - `+0x100` -> `0x80B30728` = `264.0` (x floor)
  - descriptor base `r28 = 0x80B75FE0`: `+0x98` -> `0x80B76078`,
    `+0x158` -> `0x80B76138`, `+0x198` -> `0x80B76178` **(bss)** —
    three state descriptors, 0xC0 apart
  - sound ids `487` (`0x1E7`) and `488` (`0x1E8`) — the same two words also
    appear at `0x80B30734`/`0x80B30738`
* **Behaviour:**

  ```
  ok = 0x80ACB280(this, &which)
  if ok != 0:
      if this->+0x348 != which: this->+0x348 ^= 1
      this->vtable[+0xD4](this, 0x80B76178)
      return
  parent = 0x80ACB000(this->+0x5C0)
  if parent == 0: return
  if state_is(parent + 0x394, 0x80B76078):
      0x8006A4B0(pos, this + 0xAC)
      0x80198870(*(0x80429D5C), 487, *(this), pos, 0, 0)
  else if state_is(parent + 0x394, 0x80B76138):
      0x8006A4B0(pos, this + 0xAC)
      0x80198870(*(0x80429D5C), 488, *(this), pos, 0, 0)
  if this->+0x5D7 != parent->+0x5D7:
      0x80ACADD0(this, clip = parent->+0x5D7, 0, f1 = 0.0, f2 = 1.0)  // clip binder
  this->vtable(+0x548)->+0x1C(this + 0x548)      // animation step
  this->+0x348 = parent->+0x348
  0x8006CDE0(this); 0x800955F0(this)
  x = this->+0xAC
  if x <= 264.0: this->+0xAC = 264.0; clamped = 1 else clamped = 0
  0x80096F60(this)
  g = 0x80070CB0(this + 0x1EC)
  0x80ACAE70(this, g)
  if g != 0: this->+0xEC = 0.0
  0x8006EF40(this + 0x1EC, this->+0x274)
  0x80097050(this)
  this->+0x5D6 = (this->+0x274 & (1 << (21 + this->+0x348))) != 0
  if clamped != 0: this->+0xAC = x
  step = this->+0x5D0[this->+0x348]              // i16, two entries
  0x8015F4F0(this + 0x102, step, 0x400)
  ```

  `*(this)` is the actor's first word, passed as the 3rd argument of the SE
  call (the "owner" id the sound manager expects).
* **Verdict: ready.** `0x80ACB280`, `0x80ACB000`, `0x80ACADD0` and `0x80ACAE70`
  are all translated, so the two call contracts are already in the repo.

## 9. `0x80AADC50` - `daEnKild_c` follower integration

* **Address:** `0x80AADC50`
* **TSV size:** 628 bytes (157 words)
* **Reachable body:** 157 instructions, the whole extent; 4 direct `bl`
  (`0x80AAD680` three times, `0x80064D10` once), no `bctrl`, 10 conditional
  branches, one `b` join, one `blr`. The first 40 instructions contain no call
  at all.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aadc50_FUN_80aadc50.c`
  - the export keeps the whole body but prints the three `f29..f31` saves as
    quantiser traffic.
* **Proposed signature:** `pub unsafe fn fn_80aadc50(this: usize)`
* **Callers in this REL:** none by `bl`; state handler 2 of the `daEnKild_c`
  table (see rank 7).
* **Constants:** base `r30 = 0x80B2F880`
  - `+0x34` -> `0x80B2F8B4` = `0.0`
  - `+0xC0` -> `0x80B2F940` = `0.21875` (per-frame decay)
  - `+0xC4` -> `0x80B2F944` = `-4.0` (decay floor)
  - angle step `4096` (`0x1000`) and phase `1024` (`0x400`) are immediates
* **Behaviour:**

  ```
  this->+0xD54 += 0x1000
  this->+0xD5A += 0x1000
  this->+0xC78 += this->+0xDF8
  this->+0xC84 += this->+0xDF8
  if 0x80AAD680(this)      != 0: this->+0xC78 = 0.0
  if 0x80AAD680(this)      != 0: this->+0xC84 = 0.0
  this->+0xC7C -= 0.21875 ; if this->+0xC7C  <= -4.0: this->+0xC7C  = -4.0
  this->+0xC88 -= 0.21875 ; if this->+0xC88  <= -4.0: this->+0xC88  = -4.0
  this->+0xBA0 += this->+0xC78
  this->+0xBA4 += this->+0xC7C
  this->+0xBA8 += this->+0xC80
  this->+0xBAC += this->+0xC84
  this->+0xBB0 += this->+0xC88
  this->+0xBB4 += this->+0xC8C
  this->+0xAC = this->+0xBA0
  this->+0xB0 = this->+0xBA4
  for i in 0 .. this->+0xE20 - 1:
      this->+0xD60[6*i] += 0x1000
      rec = this + 12*i
      rec->+0xC90 += this->+0xDF8
      if 0x80AAD680(this) != 0: rec->+0xC90 = 0.0
      rec->+0xC94 -= 0.21875 ; if rec->+0xC94 <= -4.0: rec->+0xC94 = -4.0
      rec->+0xBB8 += rec->+0xC90
      rec->+0xBBC += rec->+0xC94
      rec->+0xBC0 += rec->+0xC98
  if this->+0xE36 != 0: this->+0xE36 -= 1
  if this->+0xE36 == 0: 0x80064D10(this, this->+0x390)
  ```

  `0x80AAD680` (translated) is the zero-threshold predicate selected in sense
  by `+0xE3E`; it is called with only `r3 = this` and its answer is used as
  "the sway crossed zero, restart it".
* **Verdict: ready.** Pure integration; no external fact.

## 10. `0x80AF2CF0` - four-player colour/position marker publisher

* **Address:** `0x80AF2CF0`
* **TSV size:** 624 bytes (156 words)
* **Reachable body:** 156 instructions, the whole extent; 4 direct `bl`
  (`0x8005F900`, `0x80056CF0`, two `_savegpr`/`_restgpr`), six `bctrl` (three
  per effect object: `+0xA0`, `+0x5C`, `+0x60`), 8 conditional branches, four
  `b` joins, one `blr`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80af2cf0_FUN_80af2cf0.c`
  - `/* WARNING: Removing unreachable block (ram,0x80af2f44) */`; the body is
    typed `void FUN_80af2cf0(void)` behind `_savegpr_16`.
* **Proposed signature:** `pub unsafe fn fn_80af2cf0(this: usize)`
* **Callers in this REL:** one `bl` at `0x80AF29F8`, inside `0x80AF2810`
  (412 B, untranslated), which pulses two `0x8007FDA0` effect slots first and
  calls the next step `0x80AF2F60` after this one. No `.data` reference.
* **Constants:** `0x80B313C8` = `5500.0` (the marker's z). The four colour
  schemes are immediates (`255` = `0xFF`, `204` = `0xCC`, `221` = `0xDD`,
  `206` = `0xCE`, `41` = `0x29`, `112` = `0x70`, `51` = `0x33`, `108` = `0x6C`,
  `172` = `0xAC`, `0` = `0x00`).
* **Mode table:** `this->+0x14D4` selects one of four byte-quadruples:

  | mode | first RGBA | second RGBA |
  |---:|---|---|
  | 0 | `FF CC DD FF` | `FF 29 70 FF` |
  | 1 | `CC FF CE FF` | `29 FF 33 FF` |
  | 2 | `00 FF FF FF` | `00 6C FF FF` |
  | 3 | `FF FF AC FF` | `FF FF 00 FF` |

* **Behaviour:**

  ```
  pos = (0, 0, 5500.0)
  for slot in 0..3:
      p = 0x8005F900(slot)
      if p == 0: continue
      if 0x80056CF0(p, 125) != 0: continue
      if this->+0xB4C[slot] == 0: continue
      pos.x = p->+0xAC ; pos.y = p->+0xB0 ; pos.z = 5500.0
      (a, b) = scheme[this->+0x14D4]
      for obj in { this + 0x8FC, this + 0xA24 }:
          obj->vtable[+0xA0](obj, desc, 0, pos, 0, 0, 0)
          obj->vtable[+0x5C](obj, &a, &b, 0, 3)
          obj->vtable[+0x60](obj, a.a, b.a, 0, 3)
  ```

  The descriptor arguments are `0x80B62BDC` for `+0x8FC` and `0x80B62BF4` for
  `+0xA24`; both are 24 bytes apart inside the same name block. The `+0x5C`
  call takes the two 4-byte colour words, the `+0x60` call the two alpha bytes
  (`a[3]`, `b[3]`).
* **Verdict: ready.** Only the two effect vtable contracts are external, and
  they are the same `+0xA0`/`+0x5C`/`+0x60` trio used by other effect objects in
  this module.

## 11. `0x80A05000` - `daEnBuku_c` release-marker frame step

* **Address:** `0x80A05000`
* **TSV size:** 616 bytes (154 words)
* **Reachable body:** 154 instructions, the whole extent; 11 direct `bl`
  (`0x80168080`, `0x8006A4B0` twice, `0x80197F00` twice, `0x80A04A90`,
  `0x80A048D0`, `0x80A03FF0`, `0x80A03ED0`, `0x80A03C30`, `0x80A04BF0`),
  one `bctrl` (`+0x1C` of `+0x544`), 17 conditional branches, three `b` joins,
  one `blr`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a05000_FUN_80a05000.c`
  - `/* WARNING: Removing unreachable block (ram,0x80a05248) */`.
* **Proposed signature:** `pub unsafe fn fn_80a05000(this: usize)`
* **Callers in this REL:** none by `bl`; state handler of the `daEnBuku_c`
  table at `0x80B339B4`.
* **Constants:** base `r31 = 0x80B2A910`
  - `+0x88` -> `0x80B2A998` = `16.0`
  - `+0x94` -> `0x80B2A9A4` = `1.0`
  - `+0xA4` -> `0x80B2A9B4` = `42.0`, `+0xA8` -> `0x80B2A9B8` = `44.0`
  - `+0xB0` -> `0x80B2A9C0` = `2.0`
  - `+0xEC` -> `0x80B2A9FC` = `0.5`
  - `+0x128` -> `0x80B2AA38` = `33.0`, `+0x12C` -> `0x80B2AA3C` = `37.0`
  - `+0x130` -> `0x80B2AA40` = `20.0`, `+0x134` -> `0x80B2AA44` = `18.0`
  - `+0x138` -> `0x80B2AA48` = `41.0`
  - sound ids `473` (`0x1D9`) and `474` (`0x1DA`); timer immediate `16`
* **Behaviour:**

  ```
  f = 0x80168080(this + 0x584)                   // current clip frame
  if f == 16.0:
      0x8006A4B0(pos, this + 0xAC); 0x80197F00(*(0x80429D5C), 473, pos, 0)
  if f == 42.0:
      0x8006A4B0(pos, this + 0xAC); 0x80197F00(*(0x80429D5C), 474, pos, 0)
  if f == 33.0 and this->+0x6DC == 0:
      this->+0x6B2 = 16
      this->+0x6DC = 1
  if 33.0 <= f <= 37.0:
      this->+0x6A4 += 0.5 ; if this->+0x6A4 >= 2.0: this->+0x6A4 = 2.0
  else:
      this->+0x6A4 -= 1.0 ; if this->+0x6A4 <= 0.5: this->+0x6A4 = 0.5
  if f < 20.0: 0x80A04A90(this, this->+0x6A4)
  else:        0x80A04BF0(this, this->+0x6A4)
  0x80A048D0(this)
  if 18.0 <= f <= 44.0:
      this->+0x6DD = 1
      for i in 0..3:
          if this->+0x6D0[i] != 0:
              this->+0x6CC[i] = i
              this->+0x6DE = 1
  else:
      this->+0x6DD = 0
      this->+0x6DE = 0
  0x80A03FF0(this, f)                            // rank 4
  if f == 41.0: 0x80A03ED0(this)
  if this->+0x6B2 == 0: this->vtable(+0x544)->+0x1C(this + 0x544)
  0x80A03C30(this)
  ```

  This body is the proof that rank 4's float parameter is the `+0x584` clip
  frame: it is the only caller, and it passes `f` unchanged.
* **Verdict: ready.** All eight frame thresholds and both ramps are literal.

## 12. `0x80A78350` - `daEnHnswich_c` switch/course command

* **Address:** `0x80A78350`
* **TSV size:** 616 bytes (154 words)
* **Reachable body:** 154 instructions, the whole extent; 11 direct `bl`
  (`0x80168080`, `0x800B1100`, `0x800E41C0` twice, `0x800D8800` four times),
  three `bctrl` (the `+0xD4` state slot on its three exits), 14 conditional
  branches, six `b` joins, one `blr`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a78350_FUN_80a78350.c`
  - the export keeps the whole body; the three state dispatches share the
    `+0xD4` slot.
* **Proposed signature:** `pub unsafe fn fn_80a78350(this: usize)`
* **Callers in this REL:** none by `bl`; state handler of the table at
  `0x80B4C43C` (`0x80A78340`, `0x80A78350`, `0x80A78220`), which sits before
  `daEnHnswich_c::StateID_Wait/PushReserve/PushOn/DelWait/PushWait/MvOff/`
  `TimerDel/Del` at `0x80B4C500..`; effects `Wm_ko_switch`, `Wm_ob_switch`,
  `Wm_en_burst_s` at `0x80B4C05C..0x80B4C07C`.
* **Constants:** `0x80B2DD0C` is the frame-threshold table — its first five
  words are `1.0`, indexed by `this->+0x88C` (`0..4`). The rest of the block
  from `0x80B2DD20` is a vtable, not data.
* **Behaviour:**

  ```
  0x80A779E0(this); 0x80A77E60(this)
  f = 0x80168080(this + 0x584)
  if f >= threshold5[this->+0x88C]:
      if this->+0x878 == 0:
          0x800B1100(*(0x80429F30), *(0x8042A1C8) + 0x120E, 1,
                     this->+0x844, this->+0x848)
      if this->+0x884 != 0:
          n = this->+0x87C * 10
          switch this->+0x884:
            1: 0x800D8800(*(0x8042A000), 0, n)
            2: 0x800D8800(*(0x8042A000), 1, n)
            3: 0x800D8800(*(0x8042A000), 0, n)
               0x800D8800(*(0x8042A000), 1, n)
      if this->+0x355 != 0:
          *(0x8042A078) + 0x6C8 = this->+0x8A6
          0x800E41C0(*(0x8042A078), this->+0x89E & 0xFF,
                     this->+0x87C & 0xFF,
                     (this->+0x880 == 0) ? 1 : 0, 0,
                     this->+0x85C, this->+0x86C)
      cue = 1
      if this->+0x86C != 0: cue = 0
      if (*(0x80429F80) & 8) != 0: cue = 0
      if cue != 0: 0x8019BC50(*(0x8042A4A8), 8)
      if this->+0x87C == 0:     this->vtable[+0xD4](this, 0x80B72EE8)
      else if this->+0x878 == 0: this->vtable[+0xD4](this, 0x80B72EE8)
      else:                      this->vtable[+0xD4](this, 0x80B72F28)
  ```

  The two states `0x80B72EE8` and `0x80B72F28` are 0x40 bytes apart in the
  zero-fill segment **(bss)**.
* **Verdict: ready.** `0x800D8800`/`0x800E41C0` are the course-manager commands
  rank 10 also uses, and the layer byte offset `+0x120E` is already recorded in
  `docs/SPEC_WAVE144_ENEMIES.md:198`.

---

## Cross-checks between the twelve

1. **The `+0x584` frame is the shared currency of the `daEnBuku_c` block.**
   Rank 11 reads it with `0x80168080(this + 0x584)` and hands it to rank 4
   unchanged; rank 4 then tests it against `42.0`/`44.0`/`64.0`. Rank 4
   therefore takes `(this, frame)`, not `(this)`. Ghidra's `extraout_f1` is the
   incoming `f1` of the save helper, not a second argument.
2. **Six rows move from class C to ready.** `docs/ANALYSIS_WAVE127_ENEMIES.md`
   lists `0x80A22820` at `:139`, `0x80A78350` at `:251`, `0x80A96DC0` at `:287`,
   `0x80AAD9B0`/`0x80AADC50` at `:315-316` and `0x80ACC8F0` at `:363`, all as
   class **C** only because the export prints "a callee with no arguments" and
   because of `f29..f31` spills.
   The callee is `_savegpr_16/24/26/27`, which takes no arguments and preserves
   `r3..r15`, and every `psq_*` in those bodies is `qr0` with `W = 0`. There is
   no quantisation anywhere in this wave except the one rank-3 site.
3. **Three ranks share one constant block.** `0x80B2A910` serves ranks 4 and 11
   (`42.0`, `44.0`, `1500.0`, `2.0`, `64.0`, `33.0`, `37.0`, `20.0`, `18.0`,
   `41.0`), and `0x80B2F880` serves ranks 7 and 9. The overlapping offsets agree,
   which pins the tables without a second source.
4. **The two `daEnKild_c` rows agree on the record layout.** Rank 7 writes
   `12`-byte records at `this + 12*i` with fields `+0xBB8..+0xBC0` and
   `+0xC90..+0xC98`; rank 9 reads exactly those, plus a `6`-byte angle at
   `+0xD60 + 6*i`. The stride and the two clusters match, so neither body can
   have borrowed the other's extent.
5. **`0x80A87030` is called the twin of `0x80A88BC0`, but the `qr3` counts
   differ.** `docs/SPEC_WAVE144_ENEMIES.md:99` records the pair as twins; the
   remaining twin carries two `qr3` sites against this row's one, so the two are
   not byte copies and the twin must be decoded on its own terms in wave 146.

## Verification

* Every address and TSV size above matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`, and the twelve sizes sum
  to 8,220 bytes.
* Every entry was decoded from `work/rels/elf/d_enemiesNP.elf` through the
  `PT_LOAD` headers (`docs/REL_ELF_MAPPING.md`), not with the naive
  `address - 0x80A00000` formula.
* For every row `reachable insns * 4 == TSV size`: 212, 184, 183, 177, 176, 175,
  164, 163, 157, 156, 154, 154. No row needed a jump table, and none contains
  an unmapped word, an `extraout_` or a `halt_baddata`.
* Absence was re-checked against: the 16,848-address `ledger/functions.csv`, the
  1,403 addresses that appear in a `^#` heading of `docs/*.md`, every
  `fn_<addr>`/`FN_<addr>` token in the 68 `src/**/*.rs` files, and a `bl`
  scan of the whole image for callers.
* `bl` callers found: `0x80A96DC0` <- `0x80A971EC`; `0x80A03FF0` <- `0x80A05204`;
  `0x80A22820` <- `0x80A222DC`; `0x80AF2CF0` <- `0x80AF29F8`. The other eight
  are reached only from `.data` state-handler tables, which are listed per row.
* Constants were read out of the image with the ELF reader, not from the
  exports. Where an export and the RAW stream disagree (the `f31` prologues,
  the `qr3` preamble, the `_savegpr` "64-bit return"), the RAW stream wins.
* No file outside `docs/SPEC_WAVE145_ENEMIES.md` was created or modified by this
  pass. A concurrent implementation lane is editing `src/game/enemies.rs` for
  the wave-144 tranche (`// 127. Wave 145` at `src/game/enemies.rs:183699`
  consumes `docs/SPEC_WAVE144_ENEMIES.md`); that lane's rows overlap this page
  only in the shared helpers `0x8005F900`, `0x8006A4B0` and `0x80198C30`, which
  are already constant there.
