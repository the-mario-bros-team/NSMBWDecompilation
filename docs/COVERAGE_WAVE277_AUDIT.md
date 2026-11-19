# Audit - Wave 277 (two tables closed, savegpr sweep part 2)

## 1. Tables closed

| Table | Slots | Identity |
|---|---:|---|
| `0x8034E4B8` | **16/16** | `nw4r::g3d::AnmObjTexSrtOverride` |
| - | - | (`0x8034E860` opened at 10/14) |

**Forty-two vtables complete.** All three `0x120`-stride NW4R override tables are
now closed.

`0x8034E860` was identified as **`nw4r::g3d::ScnRoot`**, not the adjacent
`"NodeTree"` literal: slot 3 returns `0x802F8FC8`, and slot 0 tests
`ScnRoot`/`ScnGroup`/`ScnObj`/`G3dObj`. The literal belongs to the lookup name
`fn_80250e70` and also precedes the `ScnMdlSimple` table.

## 2. `0x80301C00` reclassified

Wave 227 called it a state table. It is a **166-slot `daBullet_c` class vtable**:
dispatch is `lwz r12,0x60(rX)` then `lwz r12,8+4*slot(r12)`, and four
`d_enemiesNP` constructors install it and immediately overwrite `+0x60` with
their own concrete REL vtable (`0x80B368F8`, `0x80B36BA8`, `0x80B36E58`,
`0x80B37108`), which share 140-141 of 166 words. Coverage 155/166, prologue 0.

## 3. `Wm_en_magmawave` decoded

`0x80323FE0` (55 slots) is installed by `FUN_8011B500`, which allocates 0xE70
bytes; the descriptor carries nine method pointers, the `ice_A1..ice_C2` texture
names and `g3d/ice.brres`. Coverage 52/55.

## 4. `_savegpr`/`_restgpr` sweep, part 2

Rule: smallest main runtime address first (REL bodies with synthetic bases are
not in `main.elf`).

| Metric | Before | After |
|---|---:|---:|
| Modelled helper calls | 616 | **567** |
| Bodies containing them | 324 | **299** |
| Bodies changed this wave | - | 25 |
| Calls removed this wave | - | **50** |
| Constants deleted | - | 10 |
| Genuine calls found | - | **0** |

Every site disassembled to a `bl` into `0x802DCD40..0x802DCD88`
(`_savegpr_14..31`) or `0x802DCD8C..0x802DCDD4` (`_restgpr_14..31`) at a
prologue/epilogue position. Variables holding a fake "return" were rebound to
the incoming `r3`/`r4`, with the two in-repo call sites updated.

The 50th call was written through an alias constant (`W100_EPILOGUE_C0`), which
is why a literal scan counts 49.

## 5. Prose-only target resolved

`0x802449B0` is a 12-byte leaf returning the `AnmObjMatClrOverride` RTTI record
`0x802F8588` (slot 3 of `0x8034E278`), named by exactly one table word and no
branch. Translated; the prose mention is now a doc link.

## 6. Gap corpus

90 of 672 candidates covered, **577 eligible**. Batch 7 was mostly 64-byte
deleting destructors. Two scanner improvements: opcode-63 A-form decoding by XO
(`fmr`/`fctiwz`/`fres`/`fctiw`/`frsp`/`fsqrt`/`frsqrte`/`fneg`/`fnabs`/`mffs`/
`mtfsf`), and candidate filtering by documented extent or `fn_<addr>`.

## 7. Evidence audit

44 wave-276 bodies re-derived: **0 mechanical findings**. The savegpr sweep was
independently verified (14 bodies carry genuine helpers at prologue/epilogue
positions, none still names a `0x802DCD*` helper). Helper names corrected:
`0x802DCD70/BC` are savegpr **26**, `0x802DCD6C/B8` are savegpr **25**.

## 8. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | close TexSrt + open ScnRoot | 4 + 8 | ~1,500 |
| B | `0x80323fe0` first 8 | 8 | ~1,760 |
| C | savegpr sweep part 2 | 25 fixed | - |
| D | `0x80301c00` first 8 | 8 | 916 |
| E | gap batch 7 | 12 | 768 |
| F | prose target + audit | 1 | 12 |

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations.
- `check-dupe-bodies` exit 0; `check-page-offsets` 34 pairs / 13,706 tokens,
  0 mismatches.
- `scripts/check.ps1` exit 0.
