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
| 0x00 | 0x04 | `u32` | mUniqueID | ctor store (4B) |
| 0x04 | 0x04 | `u32` | mParam | ctor store (4B) |
| 0x08 | 0x02 | `u16` | mProfName | ctor store (2B) |
| 0x0A | 0x01 | `u8` | mLifecycleState | zero-init (no ctor store) |
| 0x0B | 0x01 | `bool` | mDeleteRequested | zero-init (no ctor store) |
| 0x0C | 0x01 | `bool` | mDeferExecute | zero-init (no ctor store) |
| 0x0D | 0x01 | `bool` | mDeferRetryCreate | zero-init (no ctor store) |
| 0x0E | 0x01 | `u8` | mGroupType | ctor store (1B) |
| 0x0F | 0x01 | `u8` | mProcControl | ctor store (1B) |
| 0x10 | 0x4C | `fManager_c` | mMng | ctor call 0x80161530 |
| 0x5C | 0x04 | `EGG::FrmHeap *` | mHeap | zero-init (no ctor store) |
| 0x60 | 0x04 | `vptr` | vtable | ctor store (4B) |

## dBase_c

- Constructor: `0x8006C420`, this = r31
- Object size: 0x70 (112)
- vtable pointer: 0x60

> cOwnerSetMg_c at 100 is a second base subobject; only its 4-byte head is observed here (the store of 0).

| Offset | Size | Type | Name | Evidence |
|---|---|---|---|---|
| 0x60 | 0x04 | `vptr` | vtable | ctor store (4B) |
| 0x64 | 0x04 | `cOwnerSetMg_c` | mOwnerSet | ctor store (4B) |
| 0x68 | 0x04 | `const char *` | mpKindString | ctor store (4B) |
| 0x6C | 0x04 | `const char *` | mpNameString | ctor store (4B) |

## dBaseActor_c

- Constructor: `0x8006C6D0`, this = r31
- Object size: 0x128 (296)
- vtable pointer: 0x60

| Offset | Size | Type | Name | Evidence |
|---|---|---|---|---|
| 0x60 | 0x04 | `vptr` | vtable | ctor store (4B) |
| 0x70 | 0x0C | `fLiNdBaLink_c` | mLinkActor | ctor store (4B) |
| 0x7C | 0x30 | `mMtx_c` | mMatrix | zero-init (no ctor store) |
| 0xAC | 0x0C | `mVec3_c` | mPos | ctor stores (12B) |
| 0xB8 | 0x0C | `mVec3_c` | mLastPos | ctor stores (12B) |
| 0xC4 | 0x0C | `mVec3_c` | mPosDelta | zero-init (no ctor store) |
| 0xD0 | 0x0C | `mVec3_c` | mCenterOffs | ctor stores (12B) |
| 0xDC | 0x0C | `mVec3_c` | mScale | ctor stores (12B) |
| 0xE8 | 0x0C | `mVec3_c` | mSpeed | zero-init (no ctor store) |
| 0xF4 | 0x0C | `mVec3_c` | mSpeedMax | zero-init (no ctor store) |
| 0x100 | 0x06 | `mAng3_c` | mAngle | ctor stores (6B, 2B units) |
| 0x106 | 0x06 | `mAng3_c` | mAngle3D | ctor stores (6B, 2B units) |
| 0x10C | 0x04 | `f32` | mSpeedF | zero-init (no ctor store) |
| 0x110 | 0x04 | `f32` | mMaxSpeedF | zero-init (no ctor store) |
| 0x114 | 0x04 | `f32` | mAccelY | zero-init (no ctor store) |
| 0x118 | 0x04 | `f32` | mMaxFallSpeed | zero-init (no ctor store) |
| 0x11C | 0x04 | `f32` | mAccelF | zero-init (no ctor store) |
| 0x120 | 0x04 | `u32` | mActorProperties | ctor store (4B) |
| 0x124 | 0x01 | `bool` | mVisible | ctor store (1B) |
| 0x125 | 0x01 | `u8` | m_125 | zero-init (no ctor store) |
| 0x126 | 0x02 | `pad` | padding | zero-init (no ctor store) |

## dActor_c

- Constructor: `0x80064110`, this = r29
- Object size: 0x394 (916)
- vtable pointer: 0x60

| Offset | Size | Type | Name | Evidence |
|---|---|---|---|---|
| 0x128 | 0x04 | `int` | mCarryingPlayerID | ctor store (4B) |
| 0x12C | 0x04 | `u32` | mCarryingFlags | zero-init (no ctor store) |
| 0x130 | 0x01 | `u8` | mCarryingDirection | zero-init (no ctor store) |
| 0x131 | 0x03 | `pad` | padding | zero-init (no ctor store) |
| 0x134 | 0x04 | `u32` | mComboCount | zero-init (no ctor store) |
| 0x138 | 0x04 | `u32` | m_138 | zero-init (no ctor store) |
| 0x13C | 0x04 | `u32` | m_13C | ctor store (4B) |
| 0x140 | 0x04 | `f32` | m_140 | ctor store (4B) |
| 0x144 | 0x04 | `u32` | m_144 | zero-init (no ctor store) |
| 0x148 | 0xA4 | `dCc_c` | mCollisionCheck | ctor call 0x8008C200 |
| 0x1EC | 0xF0 | `dBc_c` | mBgCheck | ctor call 0x8006CF40 |
| 0x2DC | 0x34 | `dRc_c` | mRideCheck | ctor call 0x800D9860 |
| 0x310 | 0x08 | `mVec2_c` | m_310 | zero-init (no ctor store) |
| 0x318 | 0x08 | `mVec2_c` | mVisibleAreaSize | ctor stores (8B) |
| 0x320 | 0x08 | `mVec2_c` | mVisibleAreaOffset | ctor stores (8B) |
| 0x328 | 0x08 | `mVec2_c` | m_328 | ctor stores (8B) |
| 0x330 | 0x08 | `mVec2_c` | mMaxBoundDistance | ctor stores (8B) |
| 0x338 | 0x04 | `f32` | mDestroyBoundDistanceTop | ctor store (4B) |
| 0x33C | 0x04 | `f32` | mDestroyBoundDistanceBottom | ctor store (4B) |
| 0x340 | 0x04 | `f32` | mDestroyBoundDistanceLeft | ctor store (4B) |
| 0x344 | 0x04 | `f32` | mDestroyBoundDistanceRight | ctor store (4B) |
| 0x348 | 0x01 | `u8` | mDirection | zero-init (no ctor store) |
| 0x349 | 0x01 | `u8` | mZoneID | ctor store (1B) |
| 0x34A | 0x01 | `u8` | m_34A | zero-init (no ctor store) |
| 0x34B | 0x01 | `pad` | padding | zero-init (no ctor store) |
| 0x34C | 0x04 | `u8 *` | mpByteStorage | ctor store (4B) |
| 0x350 | 0x04 | `u16 *` | mpShortStorage | ctor store (4B) |
| 0x354 | 0x02 | `u8[2]` | mEventNums | zero-init (no ctor store) |
| 0x356 | 0x02 | `pad` | padding | zero-init (no ctor store) |
| 0x358 | 0x08 | `u64` | mEventMask | zero-init (no ctor store) |
| 0x360 | 0x04 | `u32` | m_360 | zero-init (no ctor store) |
| 0x364 | 0x02 | `u16` | mSpawnInfoFlags | zero-init (no ctor store) |
| 0x366 | 0x01 | `bool` | mKillIfDisabled | zero-init (no ctor store) |
| 0x367 | 0x01 | `pad` | padding | zero-init (no ctor store) |
| 0x368 | 0x04 | `u32` | mEaterActorID | ctor store (4B) |
| 0x36C | 0x01 | `u8` | mEatState | zero-init (no ctor store) |
| 0x36D | 0x01 | `u8` | mEatSpitType | ctor store (1B) |
| 0x36E | 0x02 | `pad` | padding | zero-init (no ctor store) |
| 0x370 | 0x0C | `mVec3_c` | mScaleBeforeBeingEaten | zero-init (no ctor store) |
| 0x37C | 0x04 | `u32` | mScoreType | zero-init (no ctor store) |
| 0x380 | 0x04 | `u32` | mLookAtMode | ctor store (4B) |
| 0x384 | 0x04 | `u32` | m_384 | zero-init (no ctor store) |
| 0x388 | 0x04 | `dPropelParts_c *` | mpCarryPropelActor | zero-init (no ctor store) |
| 0x38C | 0x01 | `u8` | mKind | zero-init (no ctor store) |
| 0x38D | 0x01 | `s8` | mPlayerNo | ctor store (1B) |
| 0x38E | 0x01 | `u8` | mDisableMask | ctor store (1B) |
| 0x38F | 0x01 | `u8` | mLayerID | zero-init (no ctor store) |
| 0x390 | 0x01 | `bool` | mDeleteForever | ctor store (1B) |
| 0x391 | 0x01 | `bool` | mAppearsOnBackfence | zero-init (no ctor store) |
| 0x392 | 0x02 | `u8[2]` | m_392 | zero-init (no ctor store) |

## dActorState_c

- Constructor: `0x80066FC0`, this = r31
- Object size: 0x3D4 (980)
- vtable pointer: 0x60

> Own field starts at 916 (0x394). The constructor writes vtable words at 916,920,924,928,932,936 and constructs a sub-object at 940 (0x3AC). The state manager is 64 bytes: in dActorMultiState_c the second manager's vtable block starts exactly 64 bytes after the first (980 vs 916), proving the stride. Class size 0x3D4 follows.

| Offset | Size | Type | Name | Evidence |
|---|---|---|---|---|
| 0x60 | 0x04 | `vptr` | vtable | ctor store (4B) |
| 0x394 | 0x40 | `sFStateMgr_c<dActorState_c, sStateMethodUsr_FI_c>` | mStateMgr | ctor call 0x8015FC90 at +940 |

## dActorMultiState_c

- Constructor: `none (inline in dEn_c)`
- Object size: 0x414 (1044)
- vtable pointer: 0x60

> No out-of-line constructor is present; its state manager is constructed inline by dEn_c. Its own part is 128 bytes (two 64-byte state managers, vtable blocks at 916 and 980). Full object size is 0x414: dEn_c's first own field starts at 1044 (0x414).

| Offset | Size | Type | Name | Evidence |
|---|---|---|---|---|
| 0x394 | 0x80 | `sFStateStateMgr_c<dActorMultiState_c, sStateMethodUsr_FI_c, sStateMethodUsr_FI_c>` | mStateMgr | ctor call 0x8015FC90 at +940 |

## dEn_c

- Constructor: `0x80094E80`, this = r31
- Object size: not proven in this slice
- vtable pointer: 0x60

> Fields from 1148 (0x47C) on follow the retail d_enemy.hpp order but were not reached by this slice's constructor dump; they are marked documented, not verified.

| Offset | Size | Type | Name | Evidence |
|---|---|---|---|---|
| 0x60 | 0x04 | `vptr` | vtable | ctor store (4B) |
| 0x414 | 0x20 | `dDeathInfo_c` | mDeathInfo | zero-init (no ctor store) |
| 0x434 | 0x04 | `u32` | mCcValue | zero-init (no ctor store) |
| 0x438 | 0x02 | `u16` | m_24 | ctor store (2B) |
| 0x43A | 0x06 | `u8[6]` | mPad1 | zero-init (no ctor store) |
| 0x440 | 0x0C | `mVec3_c` | mFootPush | ctor stores (12B) |
| 0x44C | 0x0C | `mVec3_c` | mFootPush2 | zero-init (no ctor store) |
| 0x458 | 0x01 | `u8` | mDeathFallDirection | zero-init (no ctor store) |
| 0x459 | 0x01 | `u8` | mIceDeathDirection | zero-init (no ctor store) |
| 0x45A | 0x01 | `bool` | mKilled | ctor store (1B) |
| 0x45B | 0x01 | `u8` | mPad2 | zero-init (no ctor store) |
| 0x45C | 0x01 | `bool` | mFootAttr3 | ctor store (1B) |
| 0x45D | 0x01 | `bool` | mInLiquid | zero-init (no ctor store) |
| 0x45E | 0x01 | `bool` | mFootAttr1 | ctor store (1B) |
| 0x45F | 0x05 | `u8[5]` | mPad3 | zero-init (no ctor store) |
| 0x464 | 0x18 | `dEnBoyoMng_c` | mBoyoMng | ctor stores (12B) |
| 0x47C | ? | `dIceMng_c` | mIceMng | retail header only (unverified here) |
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
