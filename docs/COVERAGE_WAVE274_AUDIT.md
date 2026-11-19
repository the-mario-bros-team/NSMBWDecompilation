# Audit - Wave 274 (three tables closed, one identity resolved)

## 1. Tables closed

| Table | Slots | Identity |
|---|---:|---|
| `0x80329D78` | **23/23** | `homebutton::gui::PaneManager` |
| `0x8034E5E8` | **20/20** | `nw4r::g3d::AnmObjChrBlend` |
| `0x8032E040` | **14/14** | JAS/Wii-MJ2D cue-group (world-map effect pool) |

**Thirty-three vtables complete.**

## 2. Identity resolutions and corrections

- `0x8034E5E8` was wave-273's `// unknown:` "G3D child container". Its slot 0
  RTTI records resolve the chain `AnmObjChrBlend -> AnmObjChrNode -> AnmObjChr ->
  AnmObj -> G3dObj`, so the class is **`nw4r::g3d::AnmObjChrBlend`**.
- `0x8032E040` is **not** a gameplay world-map actor vtable. Its installer is
  `wm_effect_pool_init` (`fn_80104140`), slot 0 is cue-object code
  (`0x800984B0`), and the shared forwarders `0x802755D0`/`0x80275610`/
  `0x80275590`/`0x802755B0`/`0x80197570`/`0x80046E00` plus `0x80059030` recur in
  the player/stage/worldmap/archive cue tables whose method arrays start at
  `0x803089C8`, `0x80308A08`, `0x80308A48`, `0x80312320` and `0x80321B78`
  (slots 1-6 and 9 match this table). The nearby `0x8034F0C0` table shares only
  slots 1-4. It is the JAS/Wii-MJ2D cue-group table.
- The three `0x120`-stride siblings `0x8034E278` / `0x8034E398` / `0x8034E4B8`
  are `AnmObjMatClrOverride` / `AnmObjTexPatOverride` / `AnmObjTexSrtOverride`
  with **all 48 slot words distinct** - three real tables, not aliases.

## 3. Two decode-level findings

- **`0x80046E00`** reads `r29` without ever writing it; bits 0/1/3 are masks and
  nothing branches into the body. Modelled as an explicit hidden `flags`
  argument, with the dead `nor r3,r29,r3` not reproduced.
- **`fn_801974a0`** (used by slot 5) was modelled with a double dereference for
  its counter increment; the image is `lwz r4,8(p)` plus a byte store. Fixed -
  the old model would have written through garbage.

## 4. Homebutton GUI cluster

`PaneManager` closed; `PaneComponent` (`0x80329CD0`, 24 slots) opened at 15/24.
RTTI: `0x80329D68 = {0x80329D30, 0x80329D50}`, base-info lists
`homebutton::gui::Interface` and `homebutton::gui::Component`. Its measured
prologue set is **three**, not the eight the prompt assumed; all three are closed
and no prologue remains.

## 5. Evidence audit

63 wave-273 bodies re-derived: **0 mechanical findings**. All six `d_basesNP`
installers of `0x8031BF70` build the relocated header and store it at `+0x60`;
all 21 `-0xF8` thunks resolve to real method starts (11 new target `0x80194xxx`,
10 pre-existing target the `0x802Dxxxx` archive block).

The mojibake cleanup was independently verified as comment-only: replaying both
scripts on pre-cleanup backups produced **0 code-token changes** with line counts
preserved, and no non-ASCII code or string tokens remain in the touched files.

Two doc corrections: the wave-273 audit's `ScnGroup` inference updated to
`AnmObjChrBlend`, and two byte totals corrected to measured values (1,536 / 792).

## 6. Gap corpus

49 of 260 band candidates translated, **211 free**. Batch 4 included
`fn_800f5fd0`, the shared `d_basesNP` actor base constructor that installs
`0x8031BE58` - the grandparent all six `character_SV` constructors call.

## 7. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | close PaneManager + open PaneComponent | 9 + 9 | 1,488 |
| B | `0x8031bf70` next 12 | 12 | 100 |
| C | close AnmObjChrBlend + open MatClrOverride | 8 + 10 | 2,460 |
| D | close `0x8032E040` | 12 | 2,760 |
| E | gap batch 4 | 12 | 876 |
| F | audit wave 273 | 0 (doc fixes) | - |

## Verification

- `cargo check` exit 0; 26 warnings; `doc-lint` 0 violations.
- `check-dupe-bodies` exit 0; `check-page-offsets` 34 pairs / 13,693 tokens,
  0 mismatches.
- `scripts/check.ps1` exit 0.
