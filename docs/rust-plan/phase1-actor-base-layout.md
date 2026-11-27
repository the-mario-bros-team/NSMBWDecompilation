# Phase 1.3: shared actor base-chain layout

Revision: SMNE01 rev 1. Source of truth: constructors decoded from `E:/Localsend/mariobroswii/sys/main.dol`, not a header. Every offset marked "ctor store"/"ctor call" is proven by the class constructor; see the gate output.

## Chain and a correction

```
fBase_c -> dBase_c -> dBaseActor_c -> dActor_c
dActor_c -> dActorState_c          (single-state branch)
dActor_c -> dActorMultiState_c -> dEn_c   (multi-state branch)
```

**Correction to the plan:** `dEn_c` does **not** derive from `dActorState_c`. It derives from `dActorMultiState_c`, which is a sibling of `dActorState_c` under `dActor_c`. The constructors prove this: `dEn_c` builds the multi-state manager (two 0x40 vtable blocks at 0x394 and 0x3D4) and its first own field is at 0x414, not after `dActorState_c`.

## fBase_c

- Constructor: `0x80161AD0`, this = r31
- Object size: 0x64 (100)
- vtable pointer: 0x60

> The 76-byte mManager_c interior (16..91) is one embedded object; its own fields are not split out in this slice.

| Offset | Size | Type | Name | Evidence |
|---|---|---|---|---|
| 0x00 | 0x04 | `u32` | id | ctor store (4B) |
| 0x04 | 0x04 | `u32` | params | ctor store (4B) |
| 0x08 | 0x02 | `u16` | profile_name | ctor store (2B) |
| 0x0A | 0x01 | `u8` | lifecycle_state | zero-init (no ctor store) |
| 0x0B | 0x01 | `bool` | delete_request | zero-init (no ctor store) |
| 0x0C | 0x01 | `bool` | update_request | zero-init (no ctor store) |
| 0x0D | 0x01 | `bool` | create_request | zero-init (no ctor store) |
| 0x0E | 0x01 | `u8` | group_type | ctor store (1B) |
| 0x0F | 0x01 | `u8` | proc_control | ctor store (1B) |
| 0x10 | 0x4C | `fManager_c` | manager | ctor call 0x80161530 |
| 0x5C | 0x04 | `EGG::FrmHeap *` | heap | zero-init (no ctor store) |
| 0x60 | 0x04 | `vptr` | vtable | ctor store (4B) |

## dBase_c

- Constructor: `0x8006C420`, this = r31
- Object size: 0x70 (112)
- vtable pointer: 0x60

> cOwnerSetMg_c at 100 is a second base subobject; only its 4-byte head is observed here (the store of 0).

| Offset | Size | Type | Name | Evidence |
|---|---|---|---|---|
| 0x60 | 0x04 | `vptr` | vtable | ctor store (4B) |
| 0x64 | 0x04 | `cOwnerSetMg_c` | owner_set | ctor store (4B) |
| 0x68 | 0x04 | `const char *` | kind_string | ctor store (4B) |
| 0x6C | 0x04 | `const char *` | profile_name_string | ctor store (4B) |

## dBaseActor_c

- Constructor: `0x8006C6D0`, this = r31
- Object size: 0x128 (296)
- vtable pointer: 0x60

| Offset | Size | Type | Name | Evidence |
|---|---|---|---|---|
| 0x60 | 0x04 | `vptr` | vtable | ctor store (4B) |
| 0x70 | 0x0C | `fLiNdBaLink_c` | actor_link | ctor store (4B) |
| 0x7C | 0x30 | `mMtx_c` | matrix | zero-init (no ctor store) |
| 0xAC | 0x0C | `mVec3_c` | pos | ctor stores (12B) |
| 0xB8 | 0x0C | `mVec3_c` | prev_pos | ctor stores (12B) |
| 0xC4 | 0x0C | `mVec3_c` | pos_change | zero-init (no ctor store) |
| 0xD0 | 0x0C | `mVec3_c` | center_offset | ctor stores (12B) |
| 0xDC | 0x0C | `mVec3_c` | unk_dc | ctor stores (12B) |
| 0xE8 | 0x0C | `mVec3_c` | speed | zero-init (no ctor store) |
| 0xF4 | 0x0C | `mVec3_c` | speed_target | zero-init (no ctor store) |
| 0x100 | 0x06 | `mAng3_c` | angle | ctor stores (6B, 2B units) |
| 0x106 | 0x06 | `mAng3_c` | move_angle | ctor stores (6B, 2B units) |
| 0x10C | 0x04 | `f32` | forward_speed | zero-init (no ctor store) |
| 0x110 | 0x04 | `f32` | forward_speed_target | zero-init (no ctor store) |
| 0x114 | 0x04 | `f32` | gravity | zero-init (no ctor store) |
| 0x118 | 0x04 | `f32` | fall_speed_limit | zero-init (no ctor store) |
| 0x11C | 0x04 | `f32` | accel | zero-init (no ctor store) |
| 0x120 | 0x04 | `u32` | flags_120 | ctor store (4B) |
| 0x124 | 0x01 | `bool` | draw_enabled | ctor store (1B) |
| 0x125 | 0x01 | `u8` | unk_125 | zero-init (no ctor store) |
| 0x126 | 0x02 | `pad` | padding | zero-init (no ctor store) |

## dActor_c

- Constructor: `0x80064110`, this = r29
- Object size: 0x394 (916)
- vtable pointer: 0x60

| Offset | Size | Type | Name | Evidence |
|---|---|---|---|---|
| 0x128 | 0x04 | `int` | unk_128 | ctor store (4B) |
| 0x12C | 0x04 | `u32` | unk_12c | zero-init (no ctor store) |
| 0x130 | 0x01 | `u8` | unk_130 | zero-init (no ctor store) |
| 0x131 | 0x03 | `pad` | padding | zero-init (no ctor store) |
| 0x134 | 0x04 | `u32` | unk_134 | zero-init (no ctor store) |
| 0x138 | 0x04 | `u32` | unk_138 | zero-init (no ctor store) |
| 0x13C | 0x04 | `u32` | unk_13c | ctor store (4B) |
| 0x140 | 0x04 | `f32` | unk_140 | ctor store (4B) |
| 0x144 | 0x04 | `u32` | unk_144 | zero-init (no ctor store) |
| 0x148 | 0xA4 | `dCc_c` | cc | ctor call 0x8008C200 |
| 0x1EC | 0xF0 | `dBc_c` | bc | ctor call 0x8006CF40 |
| 0x2DC | 0x34 | `dRc_c` | rc | ctor call 0x800D9860 |
| 0x310 | 0x08 | `mVec2_c` | unk_310 | zero-init (no ctor store) |
| 0x318 | 0x08 | `mVec2_c` | unk_318 | ctor stores (8B) |
| 0x320 | 0x08 | `mVec2_c` | unk_320 | ctor stores (8B) |
| 0x328 | 0x08 | `mVec2_c` | unk_328 | ctor stores (8B) |
| 0x330 | 0x08 | `mVec2_c` | unk_330 | ctor stores (8B) |
| 0x338 | 0x04 | `f32` | unk_338 | ctor store (4B) |
| 0x33C | 0x04 | `f32` | unk_33c | ctor store (4B) |
| 0x340 | 0x04 | `f32` | unk_340 | ctor store (4B) |
| 0x344 | 0x04 | `f32` | unk_344 | ctor store (4B) |
| 0x348 | 0x01 | `u8` | unk_348 | zero-init (no ctor store) |
| 0x349 | 0x01 | `u8` | unk_349 | ctor store (1B) |
| 0x34A | 0x01 | `u8` | unk_34a | zero-init (no ctor store) |
| 0x34B | 0x01 | `pad` | padding | zero-init (no ctor store) |
| 0x34C | 0x04 | `u8 *` | unk_34c | ctor store (4B) |
| 0x350 | 0x04 | `u16 *` | unk_350 | ctor store (4B) |
| 0x354 | 0x02 | `u8[2]` | unk_354 | zero-init (no ctor store) |
| 0x356 | 0x02 | `pad` | padding | zero-init (no ctor store) |
| 0x358 | 0x08 | `u64` | unk_358 | zero-init (no ctor store) |
| 0x360 | 0x04 | `u32` | unk_360 | zero-init (no ctor store) |
| 0x364 | 0x02 | `u16` | unk_364 | zero-init (no ctor store) |
| 0x366 | 0x01 | `bool` | block_hit_pending | zero-init (no ctor store) |
| 0x367 | 0x01 | `pad` | padding | zero-init (no ctor store) |
| 0x368 | 0x04 | `u32` | unk_368 | ctor store (4B) |
| 0x36C | 0x01 | `u8` | unk_36c | zero-init (no ctor store) |
| 0x36D | 0x01 | `u8` | unk_36d | ctor store (1B) |
| 0x36E | 0x02 | `pad` | padding | zero-init (no ctor store) |
| 0x370 | 0x0C | `mVec3_c` | unk_370 | zero-init (no ctor store) |
| 0x37C | 0x04 | `u32` | unk_37c | zero-init (no ctor store) |
| 0x380 | 0x04 | `u32` | unk_380 | ctor store (4B) |
| 0x384 | 0x04 | `u32` | unk_384 | zero-init (no ctor store) |
| 0x388 | 0x04 | `dPropelParts_c *` | unk_388 | zero-init (no ctor store) |
| 0x38C | 0x01 | `u8` | kind | zero-init (no ctor store) |
| 0x38D | 0x01 | `s8` | unk_38d | ctor store (1B) |
| 0x38E | 0x01 | `u8` | exec_stop_mask | ctor store (1B) |
| 0x38F | 0x01 | `u8` | unk_38f | zero-init (no ctor store) |
| 0x390 | 0x01 | `bool` | unk_390 | ctor store (1B) |
| 0x391 | 0x01 | `bool` | unk_391 | zero-init (no ctor store) |
| 0x392 | 0x02 | `u8[2]` | unk_392 | zero-init (no ctor store) |

## dActorState_c

- Constructor: `0x80066FC0`, this = r31
- Object size: 0x3D4 (980)
- vtable pointer: 0x60

> Own field starts at 916 (0x394). The constructor writes vtable words at 916,920,924,928,932,936 and constructs a sub-object at 940 (0x3AC). The state manager is 64 bytes: in dActorMultiState_c the second manager's vtable block starts exactly 64 bytes after the first (980 vs 916), proving the stride. Class size 0x3D4 follows.

| Offset | Size | Type | Name | Evidence |
|---|---|---|---|---|
| 0x60 | 0x04 | `vptr` | vtable | ctor store (4B) |
| 0x394 | 0x40 | `sFStateMgr_c<dActorState_c, sStateMethodUsr_FI_c>` | state_mgr | ctor call 0x8015FC90 at +940 |

## dActorMultiState_c

- Constructor: `none (inline in dEn_c)`
- Object size: 0x414 (1044)
- vtable pointer: 0x60

> No out-of-line constructor is present; its state manager is constructed inline by dEn_c. Its own part is 128 bytes (two 64-byte state managers, vtable blocks at 916 and 980). Full object size is 0x414: dEn_c's first own field starts at 1044 (0x414).

| Offset | Size | Type | Name | Evidence |
|---|---|---|---|---|
| 0x394 | 0x80 | `sFStateStateMgr_c<dActorMultiState_c, sStateMethodUsr_FI_c, sStateMethodUsr_FI_c>` | state_mgr | ctor call 0x8015FC90 at +940 |

## dEn_c

- Constructor: `0x80094E80`, this = r31
- Object size: not proven in this slice
- vtable pointer: 0x60

> Fields from 1148 (0x47C) on follow the retail d_enemy.hpp order but were not reached by this slice's constructor dump; they are marked documented, not verified.

| Offset | Size | Type | Name | Evidence |
|---|---|---|---|---|
| 0x60 | 0x04 | `vptr` | vtable | ctor store (4B) |
| 0x414 | 0x20 | `dDeathInfo_c` | death_info | zero-init (no ctor store) |
| 0x434 | 0x04 | `u32` | unk_434 | zero-init (no ctor store) |
| 0x438 | 0x02 | `u16` | unk_438 | ctor store (2B) |
| 0x43A | 0x06 | `u8[6]` | unk_43a | zero-init (no ctor store) |
| 0x440 | 0x0C | `mVec3_c` | unk_440 | ctor stores (12B) |
| 0x44C | 0x0C | `mVec3_c` | unk_44c | zero-init (no ctor store) |
| 0x458 | 0x01 | `u8` | unk_458 | zero-init (no ctor store) |
| 0x459 | 0x01 | `u8` | unk_459 | zero-init (no ctor store) |
| 0x45A | 0x01 | `bool` | unk_45a | ctor store (1B) |
| 0x45B | 0x01 | `u8` | unk_45b | zero-init (no ctor store) |
| 0x45C | 0x01 | `bool` | unk_45c | ctor store (1B) |
| 0x45D | 0x01 | `bool` | unk_45d | zero-init (no ctor store) |
| 0x45E | 0x01 | `bool` | unk_45e | ctor store (1B) |
| 0x45F | 0x05 | `u8[5]` | unk_45f | zero-init (no ctor store) |
| 0x464 | 0x18 | `dEnBoyoMng_c` | boyo_mng | ctor stores (12B) |
| 0x47C | ? | `dIceMng_c` | ice_mng | retail header only (unverified here) |
| ? | 0x04 | `f32` | mAirAccelY | retail header only (unverified here) |
| ? | 0x04 | `f32` | mAirSpeedMaxY | retail header only (unverified here) |
| ? | 0x04 | `f32` | mAirMaxFallSpeed | retail header only (unverified here) |
| ? | 0x04 | `u32` | mFlags | retail header only (unverified here) |
| ? | 0x02 | `u16` | mTimer1 | retail header only (unverified here) |
| ? | 0x02 | `u16` | mTimer2 | retail header only (unverified here) |
| ? | ? | `dPlayerDownTimer_c` | mNoHitPlayer | retail header only (unverified here) |
| ? | 0x01 | `u8` | mTimer3 | retail header only (unverified here) |
| ? | ? | `dEnCombo_c` | mCombo | retail header only (unverified here) |
| ? | ? | `dEnFumiProc_c` | mFumiProc | retail header only (unverified here) |

## How this was checked

`work/phase1/layout-gate.cjs` decodes each constructor from `main.dol`, collects every store whose base register is `this`, and every constructor call whose `r3 = this + offset`, then checks:

1. each claimed field has a store of the claimed width at the claimed offset (vectors: every element);
2. each sub-object is constructed at the claimed offset;
3. the vtable pointer is written at the claimed offset;
4. each class size equals the first own field of the next class in the chain;
5. no constructor store is left unexplained before the unresolved tail.

Result: **0 fail, 0 warn**. The `dEn_c` tail from 0x47C on follows the retail `d_enemy.hpp` order but is not claimed as proven here.
