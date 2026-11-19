# Wave 157 — implementation spec for the last three `d_enemiesNP` bodies

`0x80A3A4A0` (2,620 B / 655 insns), `0x80A3DEA0` (2,940 B / 735 insns),
`0x80A7BEC0` (1,464 B / 366 insns).

Read-only lane: no Rust, ledger CSV, tool or existing document was touched; this
file is the only write. `tools/merge-ledger.mjs`, `tools/check-lane-csv.mjs` and
`scripts/check.ps1` were not run.

## 0. Method and evidence

| item | value |
| --- | --- |
| raw stream | `work/rels/elf/d_enemiesNP.elf` (sha256 `d1c48633…8de564`, 1,504,312 B / `0x16F438`) |
| decoder | throwaway PowerPC32/Gekko disassembler (`%TEMP%\opencode\ppc.js`), 0 undecoded words in all 1,756 instructions |
| Ghidra cross-check | `work/ghidra/rel-d_enemiesNP/export/{80a3a4a0,80a3dea0,80a7bec0}_FUN_*.c` |
| ledger state at start | none of the three has a `ledger/functions.csv` row and none has a `fn_<addr>` body in `src/` |

### 0.1 Address → file offset

All three bodies live in program header **0**, `p_offset = 0x10F0`,
`p_vaddr = 0x80A000F0`, so

```
file_offset = address - 0x80A00000 + 0x1000
```

holds for them (`docs/REL_ELF_MAPPING.md`). One addition this wave: **the whole
file-backed image shares that bias**. Program headers 1–4 re-base at
`0x80B2A518 / 0x80B2A7A4 / 0x80B2A7B0 / 0x80B32DC0`, and each satisfies
`p_offset - p_vaddr + 0x80A00000 = 0x1000`, so the same formula also reads the
float blocks at `0x80B2C2A8` / `0x80B2C4E8` / `0x80B2DDF0` and the clip tables at
`0x80B2DE58` / `0x80B3EE94`. The last program header (`0x80B6E500`,
`memsz = 0xB4E0`, `filesz = 0`) is the zero-filled BSS: `0x80B706D8`,
`0x80B73018` and `0x80B73158` are **not** in the file and are marked *(ext)* below.

### 0.2 Corrections to earlier pages (evidence-backed)

| earlier claim | raw verdict |
| --- | --- |
| `AUDIT_WAVE156_ENEMIES.md:196-200` — the four `psq` body loads at `0x80A3A954/A9B0/A9CC/A9EC` are "`psq_l f0,d(r3),0,0`" and GQR4/qr3 is not involved | **Wrong word decode.** All four are `e003b000` = `psq_l f0,0(r3),1,qr3` (`W=1`, `I=3`), i.e. one unsigned 16-bit load scaled by `2^0` per `GQR3 = 0x00050005`. The quantiser **is** used; the audit's conclusion (no dependency on an externally programmed GQR) still holds because `GQR3` is installed by `main.dol` (`docs/REL_ELF_MAPPING.md:56-58`). |
| `AUDIT_WAVE156_ENEMIES.md:196` — the 36 prologue `psq_st` words are `W=1` | **They are `W=0`** (`(w>>15)&1 == 0`), i.e. an 8-byte paired store that fills the second half of each 16-byte `f14..f31` save slot. `stfd fN,off` + `psq_st fN,off+8,0,qr0` covers exactly 16 bytes per register; the frame is exactly 18×16 = 288 bytes at `r1+256..r1+543`. |
| `docs/SPEC_WAVE144_ENEMIES.md` §3 — the clip stride is `0x24/0x12/6/2` *and* `+0x614 = &walk_ed_BR_m[...]` | The strides are **byte** offsets `0x90/0x48/0x18/8` into a `u32` name-pointer array (equals 36/18/6/2 **pointers**), the base is `0x80B2DE58` (= `walk_st_AR_m`, *not* `walk_ed_BR_m`), and `+0x614` is set to `base + offset + 4`, i.e. the **second** pointer (`walk_ed_*`) of the cell. |
| `docs/SPEC_WAVE144_ENEMIES.md` §3 — turn step is `±0x800` | `+0xE92` receives `ea1 + 0x800` when `+0xEA1 == 0` and `ea1 - 0x800` otherwise, i.e. `0x800` / `0xFFFFF801` (−2047) for the only two reachable `ea1` values. |
| `docs/SPEC_WAVE144_ENEMIES.md` §1 — `+0xA80 = (param>>8)&0xF` unconditionally, and `+0xB4`/`+0x6F0` rewritten whenever the nibble is non-zero | Raw first stores `state` into `+0xA80` (`0x80A3E03C`) and only on the `state == 0` path replaces it with `(p>>8)&0xF`; the gate is a `rlwinm.` (`0x57A0C73F`, `Rc = 1`), so the following `beq` genuinely tests the nibble, and `-3000.0` is written to `+0xB4`/`+0x6F0` only inside that path. |
| Ghidra export of `0x80A3A4A0` — `*(r27+0x5F4)`-driven state write renders as `= 3` | Raw is `lwz r0,0x5F4(r27); …; addi r0,r0,3; stw r0,0x2CC0(r26)`, i.e. **`state = r27[0x5F4] + 3`**. |
| Ghidra export of `0x80A3A4A0` — the `{0xFA,0xFA}` colour pair | Raw computes `(0|1)+250` and `(0|1)+500 & 0xFF`, i.e. `0xFA/0xF4` or `0xFB/0xF5`. |

### 0.3 Callee signatures used by all three bodies

Registers are the ones the stream actually stages. "ledger" is
`ledger/functions.csv`.

| address | signature as used here | source of the contract |
| --- | --- | --- |
| `0x802DCD40` | `_savegpr_14` (saves `r14..r31` at `r11-72 .. r11-4`) | ledger `runtime::cpp`; `r11 = r1+256` |
| `0x802DCD54` | `_savegpr_22` (saves `r22..r31`) | ledger; `r11 = r1+96` |
| `0x802DCD74` | `_savegpr_27` (saves `r27..r31`) | ledger; `r11 = r1+96` |
| `0x802DCD8C` / `D A0` / `DC0` | `_restgpr_14` / `_restgpr_22` / `_restgpr_27` | ledger |
| `0x8015F140` | `(target: f64, rate: f64, limit: f64, deadband: f64, value: *mut f32) -> f64` | `src/engine/display.rs:17943` (`fn_8015f140`), literal |
| `0x80237BD0` / `0x80237C40` | `sin_angle16(angle/256) -> f32` / `cos_angle16(angle/256) -> f32` | `src/engine/*` (`fn_80237bd0`), ledger `resource::bres` |
| `0x802BE470` | `(sq_distance: f64) -> f64` square root | `work/ghidra/main/export/802be470_FUN_802be470.c` (wraps `0x802E8620`); used immediately after `0x801C1650` in `src/game/bases.rs:197391` |
| `0x801C1650` | `(vec3_a, vec3_b) -> f32` squared distance (also writes a result vector through the second pointer) | `src/game/bases.rs:188851` (`call_u32x2_f64`), `bases.rs:171298` |
| `0x8005F900` | `player_object(slot: s8) -> player*` (`0` when absent) | `src/engine/display.rs:2252/2484` |
| `0x80056CF0` | `player_state_is(player*, state_id: u32) -> u32` | `src/engine/display.rs:16604/17460` |
| `0x800B2F10` | `(phase: f32) -> f32` oscillator (called with `0.6`) | `src/engine/dsys.rs:4310` (`call_f32_arg_ret`) |
| `0x800B2EE0` | `(u32 id) -> u32` frame-count producer | `src/engine/display.rs:9032/9284` |
| `0x80162D00` | `(owner_word) -> owner/scene*` | `src/engine/display.rs:17557` |
| `0x80064680` | `(kind, this, arg, vec3* pos, flag, dir)` actor query | `src/game/enemies.rs:54724` (`EN59_FN_80064680`), `docs/SPEC_WAVE144_ENEMIES.md:170` |
| `0x80077810` | `(stage_manager, x: s16, y: s16, layer) -> u32` ground probe | `docs/SPEC_WAVE144_ENEMIES.md:171`, `EN59_FN_80077810` |
| `0x80078520` | `(ground_handle, 0) -> stage handle` | ledger `game::stage` |
| `0x80014820` | `(handle, dst12) -> void` cylinder fill | `EN59_FN_80014820` |
| `0x800146B0` | `(dst12, x: f32, y: f32, z: f32)` cylinder descriptor (`r3 = &rec`, `f1..f3`) | `EN59_FN_800146B0` |
| `0x801625B0` / `0x801625D0` | `link_first(this) -> rec` / `link_next(rec) -> rec` | ledger; `docs/SPEC_WAVE144_ENEMIES.md:175` |
| `0x8064F990` | `actor_world_pos(dst12, rec)` | `docs/SPEC_WAVE144_ENEMIES.md:176` |
| `0x80161800` / `0x80161750` | `ground_link_remove(handle, *slot, arg)` / `ground_link_add(...)` | ledger `resource::registry` |
| `0x80021690` | `actor_physics_step(this)` | `docs/SPEC_WAVE144_ENEMIES.md:166` |
| `0x800272F0` | `actor_setup_2F0(this)` | `docs/SPEC_WAVE144_ENEMIES.md:167` |
| `0x80027310` | `actor_set_scale_mode(this, mode)` | `EN59_FN_80027310` |
| `0x800277A0` | `actor_mode_set(this, flag)` | `docs/SPEC_WAVE144_ENEMIES.md:169` |
| `0x80097700` | `(this, vec3* pos) -> u32` stage probe | ledger `game::stage` |
| `0x80096240` | `(this, vec3* pos) -> u32` facing side 0..3 | `FN_80096240`, ledger |
| `0x80069530` | `(kind: s8) -> u32` SE variant selector | used by `fn_80a3f280` family |
| `0x80198C30` | `sound_play_3d(mgr, id, vec3* pos, arg)` | ledger `audio::ax` |
| `0x8006A4B0` | `(dst12, vec3* src)` copy three floats | used by `fn_80a7ae80` |
| `0x80168080` | `(this+0x584) -> f32` current frame | `W93_FN_80168080` |
| `0x80169020` | `(this+0x584) -> u32` "finished" predicate | `FN_80169020` |
| `0x8023A0B0` | `(obj, char* name) -> anim handle` | `W93_FN_8023A0B0` |
| `0x801651F0` | `(mixer, chr, anim*, 1)` bind | `W114_FN_801651F0` |
| `0x80169F80` | `(weight, chr, mixer)` blend | `W114_FN_80169F80` |
| `0x801680E0` | `(weight, mixer)` | `W76_FN_801680E0` |
| `0x80168FE0` | `(frame, obj)` set frame | `W89_FN_80168FE0` |
| `0x8008C380` | `collision_area_unregister(slot)` | ledger `game::stage` |
| `0x80197F00` | `sound_play(mgr, id, vec3* pos, arg)` | ledger `audio::ax` |
| `0x8016C890` | `effect_spawn(name, 0, vec3* pos, 0, f32* scale4)` | used in `fn_80a7ab90` family |
| `0x8006CD40` | `actor_follow_step(this)` | `FN_8006CD40` |
| `0x801C0BD0` | `(x: f64, y: f64, z: f64, out30)` 3×4 translation build | `W93_FN_801C0BD0` |
| `0x8016A170` | `(out30, src30)` commit transform | `W93_FN_8016A170` |
| `0x80064D10` | `(this, flag)` killer/drop request | `src/game/enemies.rs:21135` |

In-module callees (all `translated` in the ledger):
`0x80A3DE90`, `0x80A3F280`, `0x80A3F360`, `0x80A3A0A0`, `0x80A79DF0`,
`0x80A79CC0`, `0x80A7A030`, `0x80A7A170`, `0x80A7A2C0`, `0x80A7A360`,
`0x80A7A3C0`, `0x80A7AB90`, `0x80A7AE80`.

---

## 1. `0x80A3A4A0` — Fire Snake 150-segment body manager

* **Address:** `0x80A3A4A0`, **extent** 2,620 B = 655 words,
  `0x80A3A4A0..0x80A3AED8`, single `blr` at `0x80A3AED8`. Linear walk from the
  entry fills the extent with no gap and no tail call.
* **Proposed signature:** `pub unsafe fn fn_80a3a4a0(this: usize) -> u32`
  (`r3 = 1` in the epilogue at `0x80A3AE34`). Zero-argument member function:
  nothing is written to `r3` before `bl 0x802DCD40`.
* **Reached from:** one tail `b` at `0x80A3C9E8`, inside the unnamed state
  dispatcher at `0x80A3C990..0x80A3C9F8` (not in `functions.tsv`). That
  dispatcher selects this body when `u8[this+0x7B09] == 1`
  (`0x80A3C9C0/C9C8`), and the sibling bodies for `0`, `5`, `8` are
  `0x80A3AEE0`, `0x80A3BC90`, `0x80A3C4A0`. Before dispatching it returns `1`
  early when `u8[*(0x80429DD0)+0x90118] == 0 && (u8[0x80429CFA] & 1) != 0`.
* **Frame:** `stwu r1,-544`; LR at `548(r1)`; `r11 = r1+256`;
  `stfd fN,off` + `psq_st fN,off+8,0,qr0` for `f31..f14` at `528,512,…,256`;
  `bl 0x802DCD40` (GPR save at `r1+184..255`). Epilogue `0x80A3AE30..0x80A3AED8`
  restores the 18 FPRs, `bl 0x802DCD8C`, LR, `addi r1,r1,544`, `blr`.
* **Live registers:** `r15 = this`, `r27 = owner`, `r28 = 0x80B2C2A8`
  (float block), `r26 = this + 4i`, `r25 = this + 12i`, `r24 = this + 2i`,
  `r23 = this + 8i`, `r22 = this+0x4430+12i`, `r21 = this+0x24F0+4i`,
  `r20 = this+0x5BA0+12i`, `r19 = this+0x7AF0+4j`,
  `r14 = this+0x82DC+12i`, `r17 = i`, `r18 = player slot`, `r16 = scratch`.

### 1.1 Field map

| offset | type | meaning | writer / reader |
| --- | --- | --- | --- |
| `+0x00` | u32 | object/vtable word (not touched here) | — |
| `+0x7C` | `f32[12]` | 3×4 transform handed to the model | `A E00` (`0x801C0BD0`), `AE14` (`0x8016A170`) |
| `+0xAC/B0/B4` | `f32[3]` | actor position (x, y, z) | `A7C4..A7D8`, `ABCC` |
| `+0x544` | u32 | animation/model holder | `AE0C` (`0x8016A170`) |
| `+0x7AF0` | `f32[4]` | per-player eased "attention" value | `A5FC/A640/A65C` (`0x8015F140`), `AB A0` |
| `+0x7B00` | f32 | segment brightness/amplitude for this frame | `AB94`, `AD10`, `AE1C` (steps to 1.0) |
| `+0x7B04` | `s32` | build order counter (`< 47`, `< 500`) | `A670`, `A6A0`, `A788`, `AB38` |
| `+0x7B09` | u8 | body-variant selector (`1` here; `{2,3}` = colour variants) | `A D0C`; dispatcher `C9C0` |
| `+0x9A4C` | u32 | owner/scene word | `A56C` |
| `+0x9A54` | f32 | z of the transform built at `+0x7C` | `ADFC` |
| `+0x1550` | u32 | shared countdown (init 150) | `A864`, `AA74`, `AB44` |
| `+0x2CC0` | `u32[150]` | per-segment state | `A740` |
| `+0x3490` | `f32[150]` | per-segment spin rate (`0.5*f16`) | `A7B8` |
| `+0x3C60` | `f32[150]` | per-segment radius (eased to 0.2) | `A7C0`, `A964`, `A9F8`, `AA4C` |
| `+0x4430` | `f32[150][3]` | per-segment position | `A7D0`, `A8DC`, `AA5C` |
| `+0x5B0` | `f32[150][2]` | per-segment scale pair (eased to 60.0) | `A86C`, `AA9C` |
| `+0x5BA0` | `f32[150][3]` | per-segment launch origin | `AC74` |
| `+0x7310` | `u16[150]` | per-segment angle word (16-bit turn units) | `A808`, `A998`, `A920` |
| `+0x7B0A` | `s16[150]` | per-segment pitch angle | `A874`, `A8F4` |
| `+0x7EF2` | `s16[150]` | per-segment yaw angle | `A878`, `A904` |
| `+0x82DC` | `f32[150][3]` | per-segment launch target | `A87C`, `AC08` |
| `+0x1D20` | `u8[150][4]` | per-segment colour A | `A810`, `AD84` |
| `+0x24F0` | `u8[150][4]` | per-segment colour B | `A848`, `AB14`, `ADCC` |

### 1.2 Per-segment record layout (proved)

`i` runs `0..149`; each array is dense and disjoint (`150` elements). All
end addresses are one byte past the last element:

| base | elem size | index register | `i` stride | end | disjoint? |
| --- | ---: | --- | ---: | --- | --- |
| `+0x1D20` | 4 | `r21` | 4 | `+0x1F78` | yes |
| `+0x24F0` | 4 | `r21` | 4 | `+0x2748` | yes |
| `+0x2CC0` | 4 | `r26` | 4 | `+0x2F18` | yes |
| `+0x3490` | 4 | `r26` | 4 | `+0x36E8` | yes |
| `+0x3C60` | 4 | `r26` | 4 | `+0x3EB8` | yes |
| `+0x4430` | 12 | `r22`/`r25` | 12 | `+0x4B38` | yes |
| `+0x05B0` | 8 | `r23` | 8 | `+0x0A60` | yes |
| `+0x5BA0` | 12 | `r20` | 12 | `+0x62A8` | yes |
| `+0x7310` | 2 | `r24` | 2 | `+0x743C` | yes |
| `+0x7B0A` | 2 | `r24` | 2 | `+0x7C36` | yes |
| `+0x7EF2` | 2 | `r24` | 2 | `+0x801E` | yes |
| `+0x82DC` | 12 | `r14` | 12 | `+0x89E4` | yes |

Scalars `+0x7AF0` (16 B) / `+0x7B00` / `+0x7B04` / `+0x7B09` sit between
`+0x7AF0` and `+0x7B0A` and do not intersect the `+0x7B0A` array.

A segment therefore owns
`{state @+0x2CC0, spin @+0x3490, radius @+0x3C60, pos @+0x4430, scale2 @+0x5B0,
launch_origin @+0x5BA0, angle @+0x7310, pitch @+0x7B0A, yaw @+0x7EF2,
launch_target @+0x82DC, colourA @+0x1D20, colourB @+0x24F0}`, each at its own
`i` stride.

### 1.3 Constants — block `0x80B2C2A8` (`r28`), all inside the image

| address | value | used as |
| --- | --- | --- |
| `0x80B2C2A8` | `0.1` | `+0x7AF0` step dead-band partner (arm A/B `f3`) |
| `0x80B2C2AC` | `0.1` | arm C step limit (`f3`) |
| `0x80B2C2B0` | `80.0` | `f29`, the nearest-player radius multiplier |
| `0x80B2C2BC` | `4.5` | `f21`, radius bias in state 1 |
| `0x80B2C2C0` | `1.5` | `f20`, radius gain in state 1 |
| `0x80B2C2C4` | `25.0` | `f17`, angle-word gain |
| `0x80B2C2F0` | `60.0` | `f23`, angle divisor and state-2 scale target |
| `0x80B2C2F4` | `0.025` | `+0x3C60` step limit |
| `0x80B2C318` | `1.2` | (block neighbour, not read here) |
| `0x80B2C328` | `f64 2^52` | `f27`, unsigned-int → float bias |
| `0x80B2C330` | `1.0` | arm A target |
| `0x80B2C338` | `0.25` | arm A/B rate |
| `0x80B2C33C` | `1e-5` | arm A/B dead-band |
| `0x80B2C340` | `0.6` | state-1 oscillator argument |
| `0x80B2C344` | `0.1` | `+0x3C60` rate, arm C rate |
| `0x80B2C348` | `1e-6` | arm C dead-band |
| `0x80B2C34C` | `0.4` | `f18`, state-1 radius base |
| `0x80B2C350` | `0.5` | `f19`, `+0x3490` gain |
| `0x80B2C354` | `1.5` | (neighbour) |
| `0x80B2C358` | `0.3` | `f22`, `+0x5B0` base |
| `0x80B2C35C` | `8192.0` | `r27[0x102]` → x offset divisor |
| `0x80B2C360` | `26.0` | x offset gain |
| `0x80B2C364` | `65536.0` | radius → `u16` angle scale |
| `0x80B2C368` | `50.0` | (neighbour) |
| `0x80B2C36C` | `1/256` | `f26`, angle pre-scale for `0x80237BD0/C40` |
| `0x80B2C370` | `0.02` | radius-scaled y bias |
| `0x80B2C374` | `0.2` | `+0x3C60` target |
| `0x80B2C378` | `1e-4` | `+0x3C60` dead-band |
| `0x80B2C37C` | `0.025` | `+0x5B0` step rate |
| `0x80B2C380` | `0.01` | `+0x5B0` dead-band |
| `0x80B2C384` | `255.0` | `f28`, colour normaliser |
| `0x80B2C388` | `10.0` | `f30`, y lift for the target probe |
| `0x80B2C38C` | `3.0` | `f31`, launch-direction back-off |
| `0x80B2C394` | `163.0` | colour A alpha scale |
| `0x80B2C398` | `0.001` | `+0x7B00` dead-band |
| `0x80B2C3A0` | `f64 2^52 + 2^31` | `f25`, signed-int → float bias |

Other image/external operands:

| address | type | role |
| --- | --- | --- |
| `0x8042A200` | u32 **(ext, `main.dol`)** | frame counter; `& 3` gates the order counter |
| `0x80429DD0` | u32 **(ext)** | stage/height manager pointer chain |
| `0x80429D60` | u32 **(ext)** | sound manager (not used here) |

### 1.4 Pseudo-code (numbered, with address anchors)

```
 1  [A4A0-A540]  prologue: frame 544, LR, f14..f31 save, _savegpr_14
 2  [A544-A554]  r28 = 0x80B2C2A8; r15 = this
 3  [A55C-A570]  r27 = fn_80162D00(u32[this+0x9A4C])
 4  [A574-A594]  f16 = 1.0; f15 = 1.0; f0 = 0.0 -> 100/104/108(r1)
 5  [A580]       r16 = this+0x7AF0
 6  [A598-A66C]  for (j = 0; j < 4; j++) {                  # player pass
 7      [A598]      p = fn_8005F900(j)
 8      [A5A0-A5A8] if (p != 0) r17 = p
 9      [A5AC-A5BC] if (fn_80056CF0(r17, 0x53) == 0) {
10      [A5C4-A5E4]     if (fn_80056CF0(r17,0x2B) || fn_80056CF0(r17,0x0C))
11      [A5E8-A600]         fn_8015F140(1.0, 0.25, 0.1, 1e-5, this+0x7AF0+4j)
12      [A604-A628]     else if (fn_80056CF0(r17,0x2E) || fn_80056CF0(r17,0x29))
13      [A62C-A644]         fn_8015F140(0.6, 0.25, 0.1, 1e-5, this+0x7AF0+4j)
14      [A648-A65C]     else fn_8015F140(0.0, 0.1, 0.1, 1e-6, this+0x7AF0+4j)
15      [A660-A66C]   j++, r16 += 4 }
16  [A670-A6A0]  c = s32[this+0x7B04]
17              if (c < 0x2F && (u32[0x8042A200] & 3) == 0 && r27 != 0 && c < 500)
18                  s32[this+0x7B04] = c + 1
19  [A6A4-A73C]  load f15..f31 / f14..f28 constant set (see 1.3);
               spill 163.0 to 176(r1), 8192.0/26.0/65536.0/0.02 to
               144/152/160/168(r1), 0x43300000 to 112 and 120(r1)
20  [A740-A750]  for (i = 0; i < 150; i++, all cursors += stride) {
21      s = u32[this+0x2CC0+4i]
22      if ((u32)(s - 2) <= 1) goto 51                       # states 2, 3
23      if (s == 0) goto 24; if (s == 1) goto 30; if (s == 10) goto 51
24      goto 77                                             # unknown state
25  # --- state 0 -----------------------------------------------------------
26  [A770-A788]  skip if r27 == 0 || u32[r27+0x5F4] != 0 || i > s32[this+0x7B04]
27  [A78C-A798]  else u32[this+0x2CC0+4i] = 1                     # -> state 1
28      (f15 = 0.0 is loaded at A774 before the guards)
29      goto 77
30  # --- state 1 -----------------------------------------------------------
31  [A7A0-A7A8]  f16 = 0.4 + fn_800B2F10(0.6)
32  [A7B0-A7C0]  f32[this+0x3C60+4i] = 4.5 + 1.5*f16
33  [A7B4-A7B8]  f32[this+0x3490+4i] = 0.5*f16
34  [A7C4-A7D8]  f32[this+0x4430+4i ..+8] = this+0xAC/B0/B4
35  [A7DC]       u32 v = fn_800B2EE0(0x10000)
36  [A7E4-A7F4]  f15 = (0.3 * 60.0) / 60.0            # = 0.3
37  [A7E0-A820]  colour A <- {0xFF,0xFF,0xFF,0xFF}; u16[this+0x7310+2i] = (u16)v
38  [A820-A860]  colour B <- {0x00,0x00,0x00,0xFF} at this+0x24F0+4i
39  [A864-A868]  u32[this+0x1550] = 150
40  [A86C-A870]  f32[this+0x5B0+8i] = f32[this+0x5B4+8i] = 18.0
41  [A874-A884]  s16[this+0x7B0A+2i] = 0; s16[this+0x7EF2+2i] = 0
42              f32[this+0x82DC+12i ..+8] = 0.0
43  [A888-A8D8]  if (r27 != 0) {
44                  x = (f32)(s16)r27[0x102]
45                  f = r27[0x5F8] - 26.0 * (-(x / 8192.0))
46                  f32[88(r1)] = f ; f32[this+0x4430+12i] = f
47                  f32[92(r1)] = r27[0x5FC] ; f32[this+0x4434] = r27[0x5FC]
48                  f32[96(r1)] = r27[0x600] ; f32[this+0x4438] = f16
49                  s16[this+0x7B0A+2i] = s16[r27+0x100]
50                  s16[this+0x7EF2+2i] = s16[r27+0x102] }
51  # --- states 2, 3, 10: body build, entry 0x80A3A930 ----------------------
52  [A908-A92C]  (state-1 fallthrough only) a = f32[this+0x3C60+4i]
                u16[this+0x7310+2i] = (u16)(65536.0 * a)
                u32[this+0x2CC0+4i] = s + 1                  # -> state 2
53  [A930-A950]  pos = f32[this+0x4430+12i ..+8] into 88/92/96(r1)
54  [A94C-A958]  a1 = (f32)(u16)s16[this+0x7B0A+2i]; f1 = sin_angle16(a1/256)
55  [A960-A984]  f2 = 1200.0 ; f1 = 1200.0 * sin
56  [A964-A998]  rad = 25.0 * f32[this+0x3C60+4i]
                u16[this+0x7310+2i] = (u16)(s16)rad          # angle word
57  [A99C-A9C8]  a2 = s16[this+0x7EF2+2i]; c1 = cos_angle16(a1/256)
58  [A9CC-A9E0]  f16 = c1 * sin_angle16(a2/256)
59  [A9E4-A9F4]  s1 = sin_angle16(a1/256)
60  [A9F8-AA1C]  r = f32[this+0x3C60+4i]; out[0] = r
61  [AA20-AA40]  local.x = pos.x + f16*r
                local.y = pos.y + s1*r + 0.02*r
                local.z = pos.z
62  [AA44]       fn_8015F140(0.2, 0.1, 0.025, 1e-4, 40(r1))   # ease radius
63  [AA48-AA64]  f32[this+0x3C60+4i] = 40(r1)
                f32[this+0x4430+12i ..+8] = local
64  [AA68-AA70]  s = u32[this+0x2CC0+4i]
65  [AA70-AA8C]  if (s == 2) {
                    c = u32[this+0x1550]
                    if (c <= 0) u32[this+0x2CC0+4i] = 3      # test, then store c-1
                    u32[this+0x1550] = c - 1
66  [AA90-AABC]      fn_8015F140(60.0, 0.025, 0.5, 0.01, this+0x5B0+8i)
                    f32[this+0x5B4+8i] = f32[this+0x5B0+8i] }
67  [AAC4-AAC8]  else if (s == 3) {
68  [AACC-AB34]      c3 = u8[this+0x24F0+4i+3]; pack 4 bytes
                    if (c3 >= 8) c3 -= 8 else c3 = 0
                    colour B bytes 0..3 <- {b0,b1,b2,c3}
69  [AB38-AB60]     if (c3 == 0 && i == s32[this+0x7B04]) {
                       c = u32[this+0x1550]
                       if (c <= 0) fn_80064D10(this, 1)     # test, then store c-1
                       u32[this+0x1550] = c - 1 }
70  [AB64-AB74]     f15 = (f32)c3 / 255.0 }
71  [AB78-AB90]  if (r27 != 0 && u32[r27+0x5F4] != 0)
                    u32[this+0x2CC0+4i] = u32[r27+0x5F4] + 3
72  [AB94-AC2C]  f16 = f32[this+0x7B00]
                for (j = 0; j < 4; j++) {
                    v = f32[this+0x7AF0+4j]
                    p = fn_8005F900(j)
                    if (p == 0 || fn_80056CF0(p,0x53) != 0) continue
                    q = { p[0xAC], p[0xB0] + 10.0 + local.z, local.z }
                    d = sqrt(fn_801C1650(&q, &local))
                    if (d < 80.0 * v) {
                        f32[this+0x82DC+12i ..+8] = q
                        u32[this+0x2CC0+4i] = 10 } }
73  [AC30-AC38]  if (u32[this+0x2CC0+4i] != 10) goto 77
74  [AC3C-AC48]  fn_80a3a0a0(&local, this+0x82DC+12i, dir@76(r1))
75  [AC4C-AC7C]  f32[this+0x5BA0+12i] = o.x - 3.0*dir.x
                f32[this+0x5BA4+12i] = o.y - 3.0*dir.y
                f32[this+0x5BA8+12i] = o.z
76  [AC80-AD08]  c3 = colour B byte 3; if (c3 > 17) c3 -= 17 else c3 = 0
                colour B <- {c3+199, c3+150, 255, c3}; f15 = c3/255
                if (c3 == 0) u32[this+0x2CC0+4i] = 3
77  # --- per-iteration tail ------------------------------------------------
78  [AD0C-AD10]  f16 *= f15
79  [AD14-AD40]  colA = {255,255,255}; v = u8[this+0x7B09]
                if ((u8)(v-2) <= 1) { colA[0] = 250 + (v-2); colA[1] = 244 + (v-2) }
80  [AD44-AD80]  colA[3] = (u8)(163.0 * f16)
                u8[this+0x1D20+4i .. +3] = colA
81  [ADA4-ADC0]  c3 = u8[this+0x24F0+4i+3]
                u8[this+0x24F0+4i .. +3] = { c3+199, c3+150, 255, c3 }
82  [ADEC-ADF0]  cursors += stride; if (++i < 150) goto 20 }
83  [ADF4-AE14]  fn_801C0BD0(0.0, 0.0, f32[this+0x9A54], this+0x7C)
                fn_8016A170(this+0x544, this+0x7C)
84  [AE18-AE2C]  fn_8015F140(1.0, 0.25, 0.1, 0.001, this+0x7B00)
85  [AE34]       return 1
```

Notes on individual instructions that the export renders wrongly:

* `0x80A3A684` is `rlwinm. r0,r0,0,30,31` (Rc = 1), so the following `bne`
  really tests `(u32[0x8042A200] & 3) != 0`.
* `0x80A3A8A8..A8D4` is the signed-16 → float idiom:
  `xoris r0,r0,0x8000; stw r0,116(r1); lfd f0,112(r1);
   fsubs f1,f0,f25` with `f25 = 2^52+2^31` yields `(f32)(s16)value`.
* `0x80A3A8C8` is `fneg f1,f0` (`0xFC200050`, XO 40, FRB = `f0`), not a table read.
* `0x80A3A954/A9B0/A9CC/A9EC` are `psq_l f0,0(r3),1,qr3` — one unsigned 16-bit
  load each (`GQR3 = 0x00050005`), i.e. `f0 = (f32)(u16)r3[0]`.
* `0x80A3A994/A9A8/A9E4` `add r16,r0,r0` are dead (r16 is the loop index later).
* `0x80A3A97C..A9A4` computes `1200.0*sin(a1/256)` and converts it, but the
  result is only stored to `128(r1)`/loaded into `r16` and never used — dead.
* `0x80A3AD30/AD38` are two consecutive `addi r0,r0,250`, so the two bytes are
  `250 + (v-2)` and `(500 + (v-2)) & 0xFF = 244 + (v-2)`.
* `0x80A3AE34` sets `r3 = 1`; `r27` (owner) is reused as a scratch at `AD0C`.

### 1.5 Residual ambiguity — **ready**

| # | item | risk |
| --- | --- | --- |
| A1 | `fn_800B2EE0(0x10000)` — the argument is materialised by `addis r3,r0,1` (`0x80A3A7AC`); Ghidra also reads `0x10000`. `r0` at that point is the `state-2` residue, so the *literal* is only guaranteed if `r0 == 0`; the stored value is `(u16)v` into `+0x7310+2i`. | low — the write is a 16-bit angle word either way; a port can use `0x10000` |
| A2 | `0x800B2F10(0.6)` contract (oscillator in `[0,1]`); only used as an amplitude gain. | low — literal call, no decode risk |
| A3 | The `{2,3}` colour tweak (`250/244` + variant index) has no established art meaning. | cosmetic |
| A4 | `0x801C1650` writes something through its second pointer (the segment position local); this body never reads it back. | none — dead write here |
| A5 | Field *names* (`+0x7B00` "brightness", `+0x5B0` "scale pair") are inferred; offsets, types and strides are proved. | naming only |

No table, constant, callee or branch target is missing; 655/655 words decode and
every path ends in the single `blr`.

---

## 2. `0x80A3DEA0` — parameter-nibble constructor with a 16-way state dispatch

* **Address:** `0x80A3DEA0`, **extent** 2,940 B = 735 words,
  `0x80A3DEA0..0x80A3EA18`, single `blr` at `0x80A3EA18`.
* **Proposed signature:** `pub unsafe fn fn_80a3dea0(this: usize) -> u32`
  returns `2` on the two "spawn blocked by ground" paths
  (`0x80A3E260/0x80A3E4EC`) and `1` on the single common tail
  (`0x80A3E9EC`).
* **Reached from:** no `bl`/`b` in the file-backed range; instantiated from the
  actor factory table.
* **Frame:** `stwu r1,-112`; LR at `116(r1)`; `r11 = r1+96`;
  `stfd f31,96` + `psq_st f31,104,0,qr0`; `bl 0x802DCD74` (GPR save at
  `r1+76..92`). Epilogue `0x80A3E9FC..0x80A3EA18`.
* **Live registers:** `r28 = this`, `r29 = param` (`u32[this+4]`),
  `r31 = 0x80B2C4E8` (float block), `r30 = 0x80B706D8` (animation-name slot
  block), `r27 = temporary`.

### 2.1 Field map

| offset | type | meaning | anchor |
| --- | --- | --- | --- |
| `+0x04` | u32 | constructor parameter word | `DEBC` |
| `+0x60` | u32 | vtable; slot `+0xD4` is called with `(this, name)` | `E2B4`, `E644`, `E7D8`, … |
| `+0xAC/B0/B4` | `f32[3]` | position x/y/z | `DEC8`, `DFA4` |
| `+0xDC/E0/E4` | `f32[3]` | colour/scale triple (all `1.0`), `0.89` in state 15 | `E15C-E164`, `E62C` |
| `+0xE8`, `+0xEC` | f32 | state-7 vertical-seed pair | `E81C/E820` |
| `+0x124` | u8 | state-7 flag (0) | `E824` |
| `+0x318`, `+0x31C` | f32 | bounds pair (`320/64` for state 8, else `256/512`) | `E9CC/E9D0`, `E9E0/E9E4` |
| `+0x320`, `+0x324` | f32 | `0.0`, `8.0` | `E9F4/E9F8` |
| `+0x348` | u8 | layer nibble `(p>>16)&0xF`, later the facing from `0x80096240` | `E038`, `E748` |
| `+0x36D` | u8 | 0 | `E158` |
| `+0x38F` | u8 | "spawn in air" flag (selects `-3000.0` z) | `E000`, `E230`, `E4BC` |
| `+0x45C`, `+0x45D` | u8 | state-7 flags (`1`, `0`); `+0x45D` set by the stage probe | `E828/E82C`, `E1E4` |
| `+0x574/578/57C` | u32[3] | `{0, 0x80A3F360, 0}` callback record | `E1B4-E1BC` |
| `+0x5F4` | u32 | (owner) state selector read in `0x80A3A4A0`, not here | — |
| `+0x690` | u32 | scale mode (`1`, or `u8[0x8042BB18+1]`) | `E178/E184` |
| `+0x698` | 12 B | cylinder record filled by `0x80014820` / `0x800146B0` | `E0F0`, `E104` |
| `+0x6A0` | u16 | cylinder lifetime `30` | `E100` |
| `+0x6D0/6D4/6D8` | u32[3] | `{0, 0x80A3F360, 0}` callback record | `E1A0-E1B0` |
| `+0x6E8/6EC/6F0` | f32[3] | spawn copy of the position | `DEF8-DF00` |
| `+0x6F4/6F8/6FC` | f32[3] | second copy (states 0/13, 3, 4, 5, 6, 7, 8) | `E2A8-E2B0`, `E380` |
| `+0x710` | u32 | 0 | `DFEC` |
| `+0x718` | u32 | mode `(p>>20)&0xF` (`2` is folded to `0`) | `DEF0` |
| `+0x71C` | u32 | variant `(p>>24)&0xFF` | `E034` |
| `+0x724` | u32 | state-8 saved variant | `E32C` |
| `+0xA4C` | u32 | ground/height handle | `E094`, `E0E4` |
| `+0xA64` | f32 | state 3/5/11/15 colour-0 | `E8F8`, `E9A4` |
| `+0xA68` | f32 | state 3/5/11/15 colour-1 (`1.0`) | `E8F4` |
| `+0xA74` | f32 | 0.0 | `DF04` |
| `+0xA78` | u32 | state 3/4/11/15 mask (`0xFFF` or table word) | `E8EC` |
| `+0xA7C` | u32 | **state** `p & 0xF` | `DEE8` |
| `+0xA80` | u32 | sub-nibble `(p>>8)&0xF` (0 unless state 0) | `E03C`, `E048` |
| `+0xA90` | u32 | 0 (states 7, 8) | `E72C`, `E810` |
| `+0xA94`, `+0xA95` | u8 | 2 / 2 in state 2; `+0xA95` also `(p>>8)&0xF` | `E78C/E790`, `E13C` |
| `+0xA99` | u8 | `(p>>12)&0xF`, state-9 SE index | `E848` |
| `+0xAA4` | u32 | "no ground probe" flag (mode 2) | `DF14` |
| `+0xAB4` | u32 | "ride linked object" flag (modes 4, 7) | `DF30` |

### 2.2 Parameter unpack (proved)

```
p = u32[this+4]                                   # DEBC
+0xA7C = p & 0xF                                   # rlwinm 0,28,31
+0x718 = (p >> 20) & 0xF                           # rlwinm 12,28,31
+0x71C = (p >> 24) & 0xFF                          # rlwinm 8,24,31  (E024)
+0x348 = (p >> 16) & 0xF                           # rlwinm 16,28,31
+0xA80 = state ; if (state == 0) +0xA80 = (p>>8)&0xF   # rlwinm. 24,28,31 (Rc=1)
+0xA99 = (p >> 12) & 0xF                           # rlwinm 20,28,31
index for state 4 / 7 = (p >> 2) & 0xF             # rlwinm 30,26,29 -> *4
```

`+0xA95` is written three times: `(p>>8)&0xF` (`E13C`), then again the same
value, and `2` in state 2 (`E78C`).

### 2.3 Constants

Block `0x80B2C4E8` (`r31`) — every word is inside the image:

| address | type | value | role |
| --- | --- | --- | --- |
| `0x80B2C4F0` | `u32[3]` | `0x1FFF, 0x3FFF, 0x7FFF` | state-4 `+0xA78` (indexed `+4i`) |
| `0x80B2C4FC` | `f32[3]` | `1.0, 3.0, 4.0` | state-4 `+0xA68` |
| `0x80B2C508` | `f32[3]` | `0.0, 1.0, 4.0` | state-4 `+0xA64` |
| `0x80B2C518` | `f32[4]` | `-0.33325, -0.1875, 0.1875, 0.33325` | state-7 `+0xE8` |
| `0x80B2C528` | `f32[4]` | `0.33325, 0.41650, 0.5, 0.41650` | state-7 `+0xEC` |
| `0x80B2C53C` | f32 | `0.0` | `+0xA74`, state 3/11 `+0xA64`, `+0x320` |
| `0x80B2C540` | f32 | `8.0` | linked-object spawn offset, `+0x324` |
| `0x80B2C544` | f32 | `-3000.0` | z when `+0x38F` |
| `0x80B2C548` | f32 | `500.0` | z otherwise |
| `0x80B2C54C` | f32 | `1.0` | `+0xDC/E0/E4`, `+0xA68` |
| `0x80B2C550` | f32 | `0.89` | state-15 colour |
| `0x80B2C554` | f32 | `17.0` | state 5 / 11 y lift |
| `0x80B2C558` | f32 | `320.0` | state-8 `+0x318` |
| `0x80B2C55C` | f32 | `64.0` | state-8 `+0x31C` |
| `0x80B2C560` | f32 | `256.0` | default `+0x318` |
| `0x80B2C564` | f32 | `512.0` | default `+0x31C` |

Jump table `0x80B3EE94`, 16 words (read from the image, all targets inside the
body):

| idx | target | idx | target |
| ---: | --- | ---: | --- |
| 0 | `0x80A3E20C` | 8 | `0x80A3E67C` |
| 1 | `0x80A3E734` | 9 | `0x80A3E844` |
| 2 | `0x80A3E768` | 10 | `0x80A3E8A8` |
| 3 | `0x80A3E988` | 11 | `0x80A3E8CC` |
| 4 | `0x80A3E7A8` | 12 | `0x80A3E910` |
| 5 | `0x80A3E978` | 13 | `0x80A3E20C` |
| 6 | `0x80A3E944` | 14 | `0x80A3E65C` |
| 7 | `0x80A3E7EC` | 15 | `0x80A3E4A4` |

State `> 15` falls through to `0x80A3E988` (the shared tail).

Animation/state name objects handed to vtable `+0xD4` (`(this, name)`):

| address | used by state | provenance |
| --- | --- | --- |
| `0x80B706E8` | 0/13 (`== 13`), 15 (ground rebind) | *ext, REL BSS* `r30+0x10` |
| `0x80B70768` | 3, 5, 11 | *ext, REL BSS* `r30+0x90` |
| `0x80B707A8` | 4 | *ext, REL BSS* `r30+0xD0` |
| `0x80B70828` | 1, 2 | *ext, REL BSS* `r30+0x150` |
| `0x80B70868` | 7 | *ext, REL BSS* `r30+0x190` |
| `0x80B708E8` | 9 | *ext, REL BSS* `r30+0x210` |
| `0x80B70928` | 10 | *ext, REL BSS* `r30+0x250` |
| `0x80B70968` | 12 | *ext, REL BSS* `r30+0x290` |
| `0x80B709A8` | 6 | *ext, REL BSS* `r30+0x2D0` |
| `0x80B709E8` | 14 | *ext, REL BSS* `r30+0x310` |
| `0x80B70A28` | 15 (fallback) | *ext, REL BSS* `r30+0x350` |
| `0x803537F0` | 0/13 (`+0x71C != 0`) | *ext, `main.dol`* |
| `0x80353830` | 0/13 "other", 8 (`+0x718 != 0`) | *ext, `main.dol`* |
| `0x80353870` | 3 (mode 3) | *ext, `main.dol`* |
| `0x803538B0` | 5 (mode 5) | *ext, `main.dol`* |
| `0x803538F0` | 6 (mode 6) | *ext, `main.dol`* |
| `0x80353930` | 0/13 default, 8 default, 4/7 | *ext, `main.dol`* |

Other operands:

| address | role |
| --- | --- |
| `0x8042BB18` | `u8[2]` seed table; `+0x690 = u8[0x8042BB18 + 1]` when `+0xA80 == 1` **(ext)** |
| `0x80429DD0` | stage/height manager word (ground probe, ground handle) **(ext)** |
| `0x80429D14` | `u8*` global "spawn rejected" flag; `|= 1` then return 2 **(ext)** |
| `0x80429D60` | sound manager word for the state-9 3-D SE **(ext)** |
| `0x80B706D8` | base of the 11 animation-name slots above **(ext, REL BSS)** |

### 2.4 Pseudo-code (numbered)

```
  1  [DEA0-DEB8]  prologue: frame 112, LR at 116, f31 at 96/104, _savegpr_27
  2  [DEBC-DEE4]  p = u32[this+4]; r31 = 0x80B2C4E8; r30 = 0x80B706D8
  3  [DEC8-DEF0]  mode = (p>>20)&0xF
  4  [DED8-DF00]  f32[+0x6E8..+0x6F0] = f32[+0xAC..+0xB4]
                  f32[+0xA74] = 0.0
                  u32[+0xA7C] = p & 0xF
                  u32[+0x718] = mode
  5  [DF04-DF18]  if (mode == 2) { u32[+0xAA4] = 1; u32[+0x718] = 0 }
  6  [DF1C-DF30]  if (mode == 4 || mode == 7) u32[+0xAB4] = 1
  7  [DF34-DFD4]  if (u32[+0xAB4] != 0) {
  8                  fn_80064680(0x178, this, mode==7 ? 1 : 0, this+0xAC, 0, 0)
  9                  for (rec = fn_801625B0(this); rec; rec = fn_801625D0(rec)) {
                       v = fn_8064F990(&v, rec)
                       f32[+0xAC] = v[0] - 8.0
                       w = fn_8064F990(&w, rec)
                       f32[+0xB0] = 8.0 + w[1] } }
 10  [DFD4-E000]  f32[+0x710] = 0 ; f32[+0xB0] -= 8.0 ; f32[+0xAC] += 8.0
                  fn_80021690(this)
 11  [E000-E01C]  f32[+0xB4] = u8[+0x38F] ? -3000.0 : 500.0
 12  [E020-E048]  u32[+0x71C] = (p>>24)&0xFF
                  u8 [+0x348] = (p>>16)&0xF
                  u32[+0xA80] = state
 13  [E040-E058]  if (state == 0) {
                      u32[+0xA80] = (p>>8)&0xF
                      if ((p>>8)&0xF != 0) {
                          f32[+0xB4] = -3000.0 ; f32[+0x6F0] = -3000.0 } }
 14  [E05C-E06C]  if (state == 0 || state == 13) {
 15  [E070-E08C]      if (f32[+0xB4] != -3000.0 && state != 13) f32[+0xB4] = 500.0
 16  [E090-E0E4]      new = u32[0x80429DD0]
                      if (new != u32[+0xA4C]) {
                          if (u32[+0xA4C])
                              fn_80161800(u32[+0xA4C]+100, &u32[+0xA4C],
                                          u32[+0xA4C]==0xFFFFFF9C ? 0 : u32[+0xA4C])
                          if (new)
                              fn_80161750(new+100, &u32[+0xA4C],
                                          new==0xFFFFFF9C ? 0 : new) }
 17  [E0E4-E118]      h = fn_80078520(u32[+0xA4C], 0)
                      fn_80014820(h, this+0x698)
                      u16[+0x6A0] = 30
                      fn_800146B0(this+0x698, f32[+0x6E8], -f32[+0x6EC],
                                  f32[+0xB4])
 18  [E11C-E138]  } else fn_800277A0(this, (u32)(state-1) <= 1)
 19  [E138-E144]  u8[+0xA95] = (p>>8)&0xF ; fn_800272F0(this)
 20  [E148-E190]  u8[+0x36D] = 0 ; f32[+0xDC..+0xE4] = 1.0
                  u32[+0x690] = (u32[+0xA80] == 1) ? u8[0x8042BB19] : 1
                  fn_80027310(this, u32[+0x690])
 21  [E194-E1BC]  u32[+0x6D0]=0 ; u32[+0x6D4]=0x80A3F360 ; u32[+0x6D8]=0
                  u32[+0x574]=0 ; u32[+0x578]=0x80A3F360 ; u32[+0x57C]=0
 22  [E1C0-E1E4]  if (state != 0 && state != 13 && fn_80097700(this, this+0xAC))
                      u8[+0x45D] = 1
 23  [E1E8-E208]  if (state <= 15) ctr = u32[0x80B3EE94 + state*4] ; goto ctr
                  else goto 0x80A3E988
 24  # ---- state 0 / 13 : 0x80A3E20C -------------------------------------
 25  [E20C-E278]  if (u32[+0xAA4] == 0) {
                      if (fn_80077810(u32[0x80429DD0], (s16)f32[+0x6E8],
                                      (s16)-f32[+0x6EC], u8[+0x38F]) != 0) {
                          u8[0x80429D14] |= 1 ; return 2 } }
 26  [E278-E2C4]  if (u32[+0x718] == 0) {
                      if (u32[+0x71C] != 0) {
                          f32[+0x6F4..+0x6FC] = f32[+0xAC..+0xB4]
                          clip(0x803537F0) }
                      else {
                          fn_80021690(this)
                          clip(state == 13 ? 0x80B706E8 : 0x80353930) } }
 27  [E31C-E35C]  else if (u32[+0x718] == 8) {
                      u32[+0x724] = u32[+0x71C] ; u32[+0x71C] = 0
                      fn_80021690(this) ; clip(0x80353930) }
 28  [E360-E498]  else { f32[+0x6F4..+0x6FC] = f32[+0xAC..+0xB4]
                      clip(mode 3 -> 0x80353870, 5 -> 0x803538B0,
                           6 -> 0x803538F0, 4|7 -> 0x80353930,
                           else -> 0x80353830) }
 29  [E49C-E4A0]  goto 63 (common tail, r3 = 1)
 30  # ---- state 15 : 0x80A3E4A4 ------------------------------------------
 31  [E4A4-E500]  if (fn_80077810(stage, (s16)f32[+0x6E8], (s16)-f32[+0x6EC],
                                  u8[+0x38F]) != 0) {
                      u8[0x80429D14] |= 1 ; return 2 }
 32  [E504-E540]  if (fn_80077810(stage, (s16)f32[+0x6E8], (s16)-f32[+0x6EC], 2)
                      != 0) {
 33  [E544-E5C8]      ground-link swap (identical to lines 16-17)
                      f32[+0x6F4..+0x6FC] = f32[+0xAC..+0xB4]
                      u32[+0xA7C] = 0
                      clip(0x80B706E8) ; goto 63 (common tail, r3 = 1) }
 34  [E5F0-E658]  fn_80064680(0x235, this, 1, {f32[+0x6E8], f32[+0x6EC]-8.0,
                                              f32[+0x6F0]}, 0, u8[+0x38F])
                  f32[+0xDC..+0xE4] = 0.89
                  clip(0x80B70A28) ; goto 63 (common tail, r3 = 1)
 35  # ---- state 14 : 0x80A3E65C ------------------------------------------
 36  [E65C-E678]  clip(0x80B709E8) ; goto 63 (common tail, r3 = 1)
 37  # ---- state 8 : 0x80A3E67C -------------------------------------------
 38  [E67C-E6E8]  if (u32[+0x718] == 0) {
                      if (u32[+0x71C] != 0) {
                          f32[+0x6F4..+0x6FC] = f32[+0xAC..+0xB4]
                          clip(0x803537F0) } else clip(0x80353930) }
                  else { f32[+0x6F4..+0x6FC] = f32[+0xAC..+0xB4]
                         clip(0x80353830) }
 39  [E720-E730]  fn_80a3de90(this) ; u32[+0xA90] = 0 ; goto 63
 40  # ---- state 1 : 0x80A3E734 -------------------------------------------
 41  [E734-E764]  fn_80a3de90(this)
                  u8[+0x348] = (u8)fn_80096240(this, this+0xAC)
                  clip(0x80B70828) ; goto 63
 42  # ---- state 2 : 0x80A3E768 -------------------------------------------
 43  [E768-E7A4]  fn_80a3de90(this)
                  u8[+0x348] = (u8)fn_80096240(this, this+0xAC)
                  u8[+0xA95] = 2 ; u8[+0xA94] = 2
                  clip(0x80B70828) ; goto 63
 44  # ---- state 4 : 0x80A3E7A8 -------------------------------------------
 45  [E7A8-E7E8]  i = (p>>2)&0xF
                  u32[+0xA78] = u32[0x80B2C4F0 + 4i]
                  f32[+0xA68] = f32[0x80B2C4FC + 4i]
                  f32[+0xA64] = f32[0x80B2C508 + 4i]
                  clip(0x80B707A8) ; goto 63
 46  # ---- state 7 : 0x80A3E7EC -------------------------------------------
 47  [E7EC-E840]  fn_80a3de90(this)
                  i = (p>>2)&0xF
                  f32[+0xE8] = f32[0x80B2C518 + 4i]
                  f32[+0xEC] = f32[0x80B2C528 + 4i]
                  u32[+0xA90] = 0 ; u8[+0x124] = 0
                  u8[+0x45D] = 0 ; u8[+0x45C] = 1
                  clip(0x80B70868) ; goto 63
 48  # ---- state 9 : 0x80A3E844 -------------------------------------------
 49  [E844-E8A4]  u8[+0xA99] = (p>>12)&0xF
                  clip(0x80B708E8)
                  v = fn_80069530((s8)u8[+0xA99])
                  fn_8006A4B0(&pos, this+0xAC)
                  fn_80198C30(u32[0x80429D60], 0x237, &pos, v)
                  fn_80a3f280(this) ; goto 63
 50  # ---- state 10 : 0x80A3E8A8 ------------------------------------------
 51  [E8A8-E8C8]  clip(0x80B70928) ; fn_80a3f280(this) ; goto 63
 52  # ---- state 11 : 0x80A3E8CC ------------------------------------------
 53  [E8CC-E90C]  f32[+0xB0] += 17.0
                  u32[+0xA78] = 0xFFF ; f32[+0xA68] = 1.0 ; f32[+0xA64] = 0.0
                  clip(0x80B70768) ; goto 63
 54  # ---- state 12 : 0x80A3E910 ------------------------------------------
 55  [E910-E940]  f32[+0xAC..+0xB4] = f32[+0x6E8..+0x6F0]
                  clip(0x80B70968) ; goto 63
 56  # ---- state 6 : 0x80A3E944 -------------------------------------------
 57  [E944-E974]  f32[+0xAC..+0xB4] = f32[+0x6E8..+0x6F0]
                  clip(0x80B709A8) ; goto 63
 58  # ---- state 5 : 0x80A3E978 (falls into the state-3 tail) -------------
 59  [E978-E984]  f32[+0xB0] += 17.0
 60  # ---- state 3 (and state > 15) : 0x80A3E988 --------------------------
 61  [E988-E9B4]  u32[+0xA78] = 0xFFF ; f32[+0xA68] = 1.0 ; f32[+0xA64] = 0.0
                  clip(0x80B70768)
 62  # ---- common tail : 0x80A3E9B8 ---------------------------------------
 63  [E9B8-E9E4]  if (u32[+0xA7C] == 8) { f32[+0x318] = 320.0 ; f32[+0x31C] = 64.0 }
                  else                   { f32[+0x318] = 256.0 ; f32[+0x31C] = 512.0 }
 64  [E9E8-E9F8]  f32[+0x320] = 0.0 ; f32[+0x324] = 8.0
 65  [E9EC]       r3 = 1
 66  [E9FC-EA18]  epilogue: f31 restore, _restgpr_27, LR, frame, blr
```

Where `clip(x)` is `(*(u32*)(u32[this+0x60] + 0xD4))(this, x)` — `r12` from
`+0x60`, slot `+0xD4`, `r3 = this`, `r4 = x`.

### 2.5 State-transition table (proved)

| state | handler | pre-work | clip | returns |
| ---: | --- | --- | --- | --- |
| 0 | `E20C` | ground probe unless `+0xAA4` | `0x803537F0` / `0x80353930` / `0x80B706E8` | 2 or 1 |
| 1 | `E734` | `fn_80a3de90`, facing | `0x80B70828` | 1 |
| 2 | `E768` | `fn_80a3de90`, facing, `+0xA95=+0xA94=2` | `0x80B70828` | 1 |
| 3 | `E988` | `+0xB0 += 17` only if entered from state 5 | `0x80B70768` | 1 |
| 4 | `E7A8` | table row `+0xA78/+0xA68/+0xA64` | `0x80B707A8` | 1 |
| 5 | `E978` | `+0xB0 += 17` then falls into 3 | `0x80B70768` | 1 |
| 6 | `E944` | restore spawn position | `0x80B709A8` | 1 |
| 7 | `E7EC` | `fn_80a3de90`, table pair, flags | `0x80B70868` | 1 |
| 8 | `E67C` | mode/variant clip choice, `fn_80a3de90`, `+0xA90=0` | `0x8035{37F0,3930,3830}` | 1 |
| 9 | `E844` | `fn_80069530` + 3-D SE `0x237` | `0x80B708E8` | 1 |
| 10 | `E8A8` | `fn_80a3f280` | `0x80B70928` | 1 |
| 11 | `E8CC` | `+0xB0 += 17`, `0xFFF/1.0/0.0` | `0x80B70768` | 1 |
| 12 | `E910` | restore spawn position | `0x80B70968` | 1 |
| 13 | `E20C` | same as 0, no `500.0` rewrite, `0x80B706E8` | — | 2 or 1 |
| 14 | `E65C` | — | `0x80B709E8` | 1 |
| 15 | `E4A4` | two ground probes, rebind or `0x80064680(0x235)` | `0x80B706E8` / `0x80B70A28` | 2 or 1 |
| >15 | `E988` | — | `0x80B70768` | 1 |

### 2.6 Residual ambiguity — **ready**

| # | item | risk |
| --- | --- | --- |
| B1 | `+0x724` (state-8 saved variant) is write-only inside this body. | none |
| B2 | The `+0xAA4` / `+0xAB4` flag *names* are inferred from behaviour; the stores and tests are literal. | naming only |
| B3 | `fn_80069530((s8)+0xA99)` return is passed as the 4th argument of `0x80198C30`; its units (volume / pan) are not established in-module. | low — integer passthrough |
| B4 | `0x80014820` / `0x800146B0` write the 12-byte record at `+0x698` plus the `u16` at `+0x6A0`; the internal layout of that record belongs to the collision cylinder type, not to this body. | low — opaque record, no field read here |
| B5 | The ground-handle convention `handle + 100` and the `-100` (`0xFFFFFF9C`) special case are literal but their meaning is external. | low |
| B6 | Whether the `+0xD4` slot is "set clip/state" (consistent with `0x80A7BEC0` and `fn_80a79cc0`) is a cross-body inference. | low — argument is always a name object |

All 735 words decode, the 16-way table is complete and in-image, and every
handler joins the single tail at `0x80A3E9B8`.

---

## 3. `0x80A7BEC0` — `walk_*` clip-sequence player

* **Address:** `0x80A7BEC0`, **extent** 1,464 B = 366 words,
  `0x80A7BEC0..0x80A7C474`, single `blr` at `0x80A7C474`.
* **Proposed signature:** `pub unsafe fn fn_80a7bec0(this: usize)`
  (void; `r3` is never written before the epilogue).
* **Reached from:** no direct reference in the file-backed range; reached from
  the actor's state table.
* **Frame:** `stwu r1,-160`; LR at `164(r1)`; `r11 = r1+96`;
  `stfd/psq_st` pairs for `f31` (`144/152`), `f30` (`128/136`), `f29`
  (`112/120`), `f28` (`96/104`); `bl 0x802DCD54` (GPR save `r1+56..92`).
  Epilogue `0x80A7C440..0x80A7C474`.
* **Live registers:** `r23 = this`, `r24 = n` (iteration budget),
  `r25 = 0x80B73018`, `r26 = 0x80B2DDF0`, `r31 = 0x80B2DE40`,
  `r30 = 0x80B2DE38`, `r20 = 0x80B7315C`, `r21 = 0x80B50000`,
  `r19 = clip-length cursor`, `r22 = 0`, `r27 = 3`, `r28 = 1`,
  `r29 = 0x80430000`.

### 3.1 Field map

| offset | type | meaning | anchor |
| --- | --- | --- | --- |
| `+0x60` | u32 | vtable; slot `+0x9C` called with `(this)`, slot `+0xD4` with `(this, name)` | `C128/C12C`, `C3F8/C404` |
| `+0x348` | u8 | facing 0/1 (toggled on the turn) | `BF9C`, `C024`, `C16C` |
| `+0x540` | u32 | animation source object (`0x8023A0B0` receiver) | `C1F8`, `C2D8`, `C35C` |
| `+0x544` | u32 | animation holder; its vtable `+0x1C` is called with `(this+0x544)` | `C214`, `C240` |
| `+0x584` | u32 | mixer/frame object | `BF84`, `C210` |
| `+0x5B8` | u8 | `3` on a turn | `BFA4` |
| `+0x614` | u32 | clip-name cursor (`u32*`, +4 per name) | `C014`, `C1F4`, `C30C` |
| `+0x618` | u32 | clip-length cursor (`f32*`, +4 per length); `0x80B2DE48` = exhausted | `C01C`, `C1E4` |
| `+0x688` | `f32[3]` | smoke origin per collision slot, stride `0xC` from `+0x688` | `C184/C188/C18C` |
| `+0x7C` | `f32[12]` | transform (used by `fn_80a7a030`, not here) | — |
| `+0xD1C` | `u8[0xA4]` | collision slot per `bit` | `C048`, `C060`, `C3C0` |
| `+0xE80` | f32 | cached animation frame at take-off | `BF80`, `C118` |
| `+0xE8` | f32 | vertical speed, zeroed at take-off | `C120` |
| `+0xE92` | s16 | contact turn step (`0x800` / `0xFFFFF801`) | `BF68` |
| `+0xE9A` | u8 | "turn requested" | `BF0C` |
| `+0xE9B` | u8 | "take-off fired" | `BF74`, `C138`, `C368` |
| `+0xE9C` | u8 | "first clip of the sequence" | `C040`, `C254`, `C2A4` |
| `+0xE9E` | u8 | "sequence paused" counter | `C090` |
| `+0xEA0` | u8 | turn cooldown (`ea1 + 0x10`) | `BF30` |
| `+0xEA1` | u8 | flip 0/1 (toggled) | `BF28`, `BFA8`, `C034` |
| `+0xEA2` | u8 | clip group `m/u/d` (0..2) | `BFE4`, `C2E0` |
| `+0xEA3` | u8 | clip pair 0/1 | `BFE0`, `C2E4` |
| `+0xEA5` | u8 | next iteration budget | `C368` |
| `+0xEA6` | u8 | iteration budget (set to 2 on a turn) | `BF10`, `C074`, `C380` |
| `+0xEA8` | u8 | collision-active bitmask | `C038`, `C05C`, `C3D4` |

### 3.2 Clip-table record layout (proved from the image)

`0x80B2DE58`, **72 `u32` name pointers = 2 facings × 36**, ending at
`0x80B2DF78`; each 144-byte facing block is a 2×3×3 grid of 2-pointer cells:

```
index(facing, flip, group, pair) =
    0x80B2DE58 + facing*0x90 + flip*0x48 + group*0x18 + pair*0x8     (bytes)
                (36 ptr)       (18 ptr)     (6 ptr)      (2 ptr)
cell[0] = "walk_st_<AA/BL..>_<m/u/d>"      cell[1] = "walk_ed_<..>_<m/u/d>"
```

Only `0x80B2DF78` follows: the 10-entry `wait_*` name table that
`fn_80a79cc0` indexes (ledger note "table 0x80B2DFA0"), so 72 entries is the
whole clip array.

Decoded sample (strings are image data at `0x80B4C63C..`):

| index | cell[0] (`st`) | cell[1] (`ed`) |
| ---: | --- | --- |
| 0 | `walk_st_AR_m` | `walk_ed_BR_m` |
| 2 | `walk_st_AR_m` | `walk_ed_BR_u` |
| 4 | `walk_st_AR_m` | `walk_ed_BR_d` |
| 6 | `walk_st_AR_u` | `walk_ed_BR_m` |
| 18 | `walk_st_BR_m` | `walk_ed_AR_m` |
| 36 | (mirrored block, `st_BR_*` ↔ `ed_AR_*`) | |
| 54 | `walk_st_BL_*` | `walk_ed_AL_*` |

Length cursors (`f32` array at `0x80B2DE40`, used as a pointer, `+4` per clip):

| address | value | role |
| --- | --- | --- |
| `0x80B2DE40` | `45.0` | length used on a sequence restart (`C384`) |
| `0x80B2DE44` | `40.0` | length used on a turn; the cursor is **initialised to this address** (`BF44/BF4C`) |
| `0x80B2DE48` | `0.25` | **end sentinel address** — `*(u32*)(this+0x618) == 0x80B2DE48` means "sequence exhausted" |

Collision-bit table `0x80B2DE38` = bytes `{0x00, 0x01, 0x01, 0x00}`,
indexed `facing*2 + flip`:

```
bit = u8[0x80B2DE38 + u8[this+0x348]*2 + u8[this+0xEA1]]
    = {0:0, 1:1, 2:1, 3:0}[that index]
slot = this + 0xD1C + bit*0xA4
```

### 3.3 Constants and data

| address | type | value / role |
| --- | --- | --- |
| `0x80B2DE20` | f32 | `0.0` — `+0xE8`, `fn_80168FE0` frame |
| `0x80B2DE28` | f32 | `1.0` — weight for `0x80169F80` / `0x801680E0` |
| `0x80B2DE38` | u8[4] | `00 01 01 00` collision-bit table |
| `0x80B2DE40` | f32 | `45.0` restart clip length |
| `0x80B2DE44` | f32 | `40.0` turn clip length |
| `0x80B2DE48` | f32 | `0.25` end-sentinel address |
| `0x80B2DE58` | `u32[72]` | `walk_st_/walk_ed_` name table |
| `0x80B2E154` | f32 | `3.0` — smoke y offset |
| `0x80B2E158` | f32 | `1.2` — smoke scale written into BSS |
| `0x80B4C8A4` | char* | `"Wm_en_landsmoke_s"` (image string) |
| `0x80B73028` | u32 | fallback clip name object **(ext, REL BSS)** |
| `0x80B73158` | u8 | smoke "initialised" flag **(ext, REL BSS)** |
| `0x80B7315C..0x80B73164` | f32[3] | three `1.2` scales **(ext, REL BSS)** |
| `0x80429D5C` | u32 | sound manager (`r29 = 0x80430000`, `lwz r19,-0x62A4(r29)`) **(ext)** |

### 3.4 Pseudo-code (numbered)

```
  1  [BEC0-BEF0]  prologue: frame 160, LR at 164, f28..f31 at 96..159,
                  _savegpr_22
  2  [BEF4-BF08]  r25 = 0x80B73018 ; r26 = 0x80B2DDF0 ; r23 = this
                  fn_80a79df0(this)
  3  [BF0C-BF18]  n = u8[+0xEA6]
                  if (u8[+0xE9A] == 0 || u8[+0xEA0] != 0) goto 17
  4  [BF28-BF40]  flip = u8[+0xEA1]
                  u8[+0xEA0] = 0x10 + flip
                  step = flip + 0x800            # 0x800 or 0x801
                  if (flip != 0) step = flip - 0x800   # 0xFFFFF801
  5  [BF44-BF58]  if (*(u32*)(this+0x618) == 0x80B2DE48) step = -step
  6  [BF5C-BF68]  s16[+0xE92] = step
  7  [BF6C-BF70]  if (*(u32*)(this+0x618) != 0x80B2DE48) goto 15
  8  [BF74-BF90]  if (u8[+0xE9B]) { fn_80168FE0(f32[+0xE80], this+0x584);
                                    u8[+0xE9B] = 0 }
  9  [BF94-BFA8]  u8[+0x5B8] = 3
                  u8[+0xEA1] ^= 1
                  u8[+0x348] = (u8[+0x348] == 0) ? 1 : 0
 10  [BFEC/BFF8]  tmp = u8[+0xEA2]; u8[+0xEA2] = u8[+0xEA3]; u8[+0xEA3] = tmp
 11  [BFCC-C014]  u32[+0x614] = 0x80B2DE58
                                 + u8[+0x348]*0x90 + u8[+0xEA1]*0x48
                                 + u8[+0xEA2]*0x18 + u8[+0xEA3]*0x08
                                 + 4                  # the "ed" pointer
 12  [C018-C020]  u32[+0x618] = 0x80B2DE44 ; fn_80a7a2c0(40.0, this)
 13  [C024-C068]  u8[+0xE9C] = 1
                  bit = u8[0x80B2DE38 + u8[+0x348]*2 + u8[+0xEA1]]
                  u8[+0xEA8] &= ~(1 << bit)
                  fn_8008C380(this + 0xD1C + bit*0xA4)
 14  [C06C]       goto 17
 15  [C070-C074] (arrived when the cursor was not the sentinel)
                  n = 2 ; u8[+0xEA6] = 2
 16  [C078-C08C]  fn_80a79cc0(this, 0, 1)
                  fn_80a7ae80(this)
 17  [C090-C0B4]  if (u8[+0xE9E] != 0) {
                      u8[+0xE9E] -= 1
                      fn_80a7a030(this) ; fn_80a7a170(this)
                      return }
 18  [C0B8-C0E4]  f29 = 0.0 ; f31 = 1.2 ; f30 = 3.0 ; r30 = 0x80B2DE38
                  r31 = 0x80B2DE40 ; r20 = 0x80B7315C ; r22 = 0
                  r27 = 3 ; r28 = 1 ; r29 = 0x80430000 ; r21 = 0x80B50000
 19  [C0E4]       do {
 20  [C0E4-C10C]      if (u8[+0xE9B] == 0 && fn_80a7a3c0(this) == 0
                      && *(u32*)(this+0x618) == 0x80B2DE48) {
 21  [C110-C124]         f32[+0xE80] = fn_80168080(this+0x584)
                        f32[+0xE8] = 0.0
                        u8[+0xEA8] = 3
                        (*(u32[this+0x60] + 0x9C))(this)
 22  [C138-C15C]         u8[+0xE9B] = 1
                        fn_8006A4B0(&p, this+0xAC)
                        fn_80197F00(u32[0x80429D5C], 0x1E2, &p, 0)
 23  [C160-C1B0]         bit = u8[0x80B2DE38 + u8[+0x348]*2 + u8[+0xEA1]]
                        s = this + 0x688 + bit*12
                        v = { s[0], s[4] - 3.0, s[8] }
                        if ((s8)u8[0x80B73158] == 0) {
                            f32[0x80B7315C .. 0x80B73164] = 1.2
                            u8[0x80B73158] = 1 }
 24  [C1B4-C1C8]         fn_8016C890("Wm_en_landsmoke_s" /*0x80B4C8A4*/,
                                      0, &v, 0, 0x80B7315C) }
 25  [C1CC-C1D0]      fn_8006CD40(this)
 26  [C1D4-C1E0]      if (fn_80169020(this+0x584) == 0) goto 40
 27  [C1E4-C1F0]      cur = *(u32*)(this+0x618)
 28  [C1F0]          if (cur >= 0x80B2DE48) goto 34
 29  # --- advance to the next clip in the sequence ------------------------
 30  [C1F4-C208]      name  = *(u32*)u32[+0x614]
                      u32[+0x614] += 4
                      anim = fn_8023A0B0(this+0x540, name)
 31  [C210-C250]      fn_801651F0(this+0x584, this+0x544, &anim, 1)
                      fn_80169F80(1.0, this+0x544, this+0x584)
                      fn_801680E0(1.0, this+0x584)
                      (*(u32[this+0x544] + 0x1C))(this+0x544)
 32  [C254-C268]      if (u8[+0xE9C] == 0) fn_80168FE0(0.0, this+0x584)
 33  [C26C-C2B8]      len = *(f32*)cur ; u32[+0x618] = cur + 4
                      fn_80a7a2c0(len, this) ; fn_80a7a360(len, this)
                      if (u8[+0xE9C] != 0) fn_80a7a3c0(this)
                      u8[+0xE9C] = 0
                      fn_80a7a030(this) ; fn_80a7a170(this)
                      goto 42
 34  # --- sequence exhausted ----------------------------------------------
 35  [C2BC-C2C8]      if (fn_80a7ab90(this) == 0) {
 36  [C3F8-C410]          (*(u32[this+0x60] + 0xD4))(this, u32[0x80B73028])
                          return }
 37  [C2CC-C30C]      off = u8[+0x348]*0x90 + u8[+0xEA1]*0x48
                        + u8[+0xEA2]*0x18 + u8[+0xEA3]*0x08
                      anim = fn_8023A0B0(this+0x540, u32[0x80B2DE58 + off])
                      u32[+0x614] = 0x80B2DE58 + off + 4
 38  [C310-C364]      fn_801651F0(this+0x584, this+0x544, &anim, 1)
                      fn_80169F80(1.0, this+0x544, this+0x584)
                      fn_801680E0(1.0, this+0x584)
                      (*(u32[this+0x544] + 0x1C))(this+0x544)
                      fn_80168FE0(0.0, this+0x584)
 39  [C368-C3F4]      u8[+0xE9B] = 0 ; u8[+0xE9C] = 0
                      u8[+0xEA6] = u8[+0xEA5]
                      u32[+0x618] = 0x80B2DE44
                      fn_80a7a2c0(45.0, this) ; fn_80a7a360(45.0, this)
                      bit = u8[0x80B2DE38 + u8[+0x348]*2 + u8[+0xEA1]]
                      u8[+0xEA8] &= ~(1 << bit)
                      fn_8008C380(this + 0xD1C + bit*0xA4)
                      fn_80a7a030(this) ; fn_80a7a170(this)
                      return
 40  # --- animation still running -----------------------------------------
 41  [C414-C434]      (*(u32[this+0x544] + 0x1C))(this+0x544)
                      fn_80a7a030(this) ; fn_80a7a170(this)
 42  [C438-C43C]  } while (--n != 0)
 43  [C440-C474]  epilogue: f28..f31 restore, _restgpr_22, LR, frame, blr
```

Structural notes:

* Lines 34–39 (`return`) exit the **whole function**, not just the `do` loop;
  lines 40–42 fall through to the loop counter.
* `n` is `u8[+0xEA6]`; on a turn it is forced to `2` (line 15), otherwise it is
  copied from `+0xEA5` when a sequence restarts (line 39).
* `+0x614` always points at the **second** (`walk_ed_*`) name of a cell after a
  turn/restart, so the pair `{ed, next-cell st}` is what plays before the
  sentinel is reached.
* The two `bcctr` sites use vtable `+0x9C` (this object, no extra argument) and
  `+0xD4` (this object, name object in `r4`); the `+0x544` object's vtable
  `+0x1C` takes `this+0x544` as its receiver.

### 3.5 Residual ambiguity — **ready**

| # | item | risk |
| --- | --- | --- |
| C1 | `u8[+0xEA0] = ea1 + 0x10` and the turn step `ea1 ± 0x800` reuse the flip byte as part of the constant; only `ea1 ∈ {0,1}` is reachable, giving `0x10/0x11` and `0x800 / 0xFFFFF801`. | low — record the literal, do not "clean up" to `±0x800` |
| C2 | The `group`/`pair` meaning of `+0xEA2`/`+0xEA3` (which of `m/u/d` and `0/1`) is data-driven; the stride arithmetic is proved. | naming only |
| C3 | `+0x688 + 12*bit` records are only read (three `f32` at `+0, +4, +8`); their producer is outside this body. | low |
| C4 | `fn_80a7ab90` returns the "should restart" verdict; only its boolean use is needed here. | none |
| C5 | `0x8016C890`'s 5th argument is the address `0x80B7315C` (a live 3-float BSS scale block, lazily initialised to `1.2`), not a value. | low |

All 366 words decode; every constant is in the image except the three explicitly
marked BSS cells; the ambiguity is naming, not structure.

---

## 4. Readiness summary

| address | bytes | insns | tables | verdict | blocking evidence |
| --- | ---: | ---: | --- | --- | --- |
| `0x80A3A4A0` | 2,620 | 655 | `0x80B2C2A8` float block, `0x8042A200` | **ready** | none (residuals A1–A5 are naming/one-constant) |
| `0x80A3DEA0` | 2,940 | 735 | `0x80B2C4E8` block + 3 indexed arrays, `0x80B3EE94` 16-word table, 17 name objects | **ready** | none (residuals B1–B6 are naming/external helpers) |
| `0x80A7BEC0` | 1,464 | 366 | `0x80B2DE58` 72-entry table, `0x80B2DE38..0x80B2DE48` block, `0x80B73028`/`0x80B73158` BSS | **ready** | none (residuals C1–C5 are naming/literal constants) |

Every callee is either a `main.dol` helper this module already calls by address
or a translated sibling; the only GQR dependency is `qr3 = 0x00050005` on the
four `psq_l` sites of `0x80A3A4A0`.

## 5. What this lane did not do

- No Rust edit, no ledger row, no merge, no `check-lane-csv`, no
  `scripts/check.ps1`, no change to any existing document.
- The decoder lives in `%TEMP%\opencode\ppc.js` and is not part of the repo.
- The three bodies remain unledgered and unimplemented at the time of writing;
  re-check `ledger/functions.csv` and `src/` before starting the port.
