# Analysis - Wave 158 (`game::bases`) - read-only reclassification

## Purpose

This is a **read-only** lane. No Rust file, ledger row or existing document was
modified; the only output is this report. It answers one question for the
1,786 `d_basesNP` functions that are still absent from
`ledger/functions.csv` and from `src/game/bases.rs`: **which of them can a later
wave actually write, and which are pinned by a missing artifact?**

## Method

1. **Enumeration.** `work/ghidra/rel-d_basesNP/export/functions.tsv` holds 5,615
   rows. `ledger/functions.csv` holds 3,816 `game::bases` rows and
   `src/game/bases.rs` holds 3,829 `pub unsafe fn` bodies at the time this
   report was finalised. The ledger is a subset of the bodies; one body is
   spelled `fn_8060180` with seven hex digits, so address matching - not name
   matching - was used. A concurrent wave added 13 bodies to `bases.rs`
   during this lane (the ledger was not yet merged): `0x80612D30` `0x80612EC0`
   `0x806131D0` `0x80613380` `0x80613A30` `0x806D8B90` `0x807390D0` `0x80741A70`
   `0x80763C20` `0x80776440` `0x8077C440` `0x807A0090` `0x807B2550`. The snapshot below is
   the later tree and those 13 are excluded from the result set.
2. **Result set.** A TSV row is *remaining* when its address is in neither
   set: **1,786 rows / 289,068 bytes** (the tree moved under this lane
   from 1,799 rows / 294,500 bytes when the pass started). Every one maps
   inside the REL text segment, so none is a phantom row. Six of them are
   already translated under `game::player` (the player module reuses the same
   REL text range for six `0x8060` bodies), so the genuinely unclaimed count
   is **1,780**. They stay in the table as class S rather than being
   silently dropped.
3. **Evidence.** Sizes and names come from the TSV; each export under
   `work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c` was read for
   `halt_baddata`, `unaff_*`, jumptable warnings, indirect-call shape, string
   literals and the dominant `this + 0xNN` field.
4. **Raw stream.** A throwaway PowerPC decoder (kept in
   `%TEMP%\opencode\ppc158.js`, never in the repo) read the ELF32-BE program
   headers and decoded `psq_l`/`psq_st` (GQR index in bits 12-14), `bl`/`b`,
   `mtctr`/`bctr`, `addi`/`lis`, the integer and float load/store set, and the
   `mfspr`/`mtspr` pair. `file_offset = address - 0x80600000 + 0x1000` matches
   PT_LOAD 0 exactly (`0x10F0` maps `0x806000F0`).
5. **Prior reasons.** Every `## Deliberate skips` section in
   `COVERAGE_WAVE100_BASES.md` .. `COVERAGE_WAVE157_BASES.md` was read. 644 of
   the 1,786 rows carry a documented prior reason; **1,142 were never named**
   in any skip list (they were simply never reached).

## Classes

| Class | Meaning | Rows | Bytes |
|---|---|---:|---:|
| **R** | reconstructible now from the shipped artifact | 1,176 | 251,100 |
| **P** | needs one specific pinned fact (GQR3 word, extent fix) | 34 | 8,352 |
| **S** | already covered - 6 under `game::player`, 97 byte-identical twins | 103 | 6,916 |
| **U** | genuinely unreconstructable from this REL | 473 | 22,700 |
| | **total absent from the bases slice** | **1,786** | **289,068** |
| | *already translated under `game::player`* | *6* | *820* |
| | **addresses that still need a body** | **1,780** | **288,248** |

**Honest bottom line.** 1,273 of the 1,780 unclaimed addresses (257,196 of 288,248 bytes) are writable today -
**71.5 % of the rows and 89.2 % of the bytes** - 1,176 by
fresh translation (R) and 97 by cloning an existing body (S).
Another 34 (8,352 bytes) unblock with one pinned constant or a four-byte extent
correction. The 473 rows that stay unreconstructable (22,700 bytes, 7.9 % of
the unclaimed bytes) are one repeated artifact - `0x802DCBFC` trampolines - and cannot
be given semantics from anything shipped in this REL.

The work is not dominated by monsters: 196 R rows are 64 bytes or less and
858 are 256 bytes or less; the 27 bodies above 512 bytes are only 17 % of the
class's bytes. A wave can therefore make visible progress with small,
individually verifiable bodies instead of opening the five 3-4 KiB
paired-single machines first.

## What the artifact settles (and what prior lanes got wrong)

1. **`halt_baddata` is not a truncation in 74 of 75 rows.** Of the 75 remaining
   rows whose export contains `halt_baddata`, 72 end their raw stream in a
   plain tail `b` and 2 in `bctr`; only `0x80643260` really stops mid-body
   (its last word is `rlwinm`, and the true body continues with `b` + `blr`
   at `0x806432F4`/`0x806432F8`, eight bytes past the TSV extent). Ghidra emits
   "bad instruction data" when the tail branch lands outside the analysed text
   (these targets are `main.dol` addresses), not when the bytes are missing.
   The 74 are class **R**.
2. **The 13 "could not recover jumptable" rows are virtual tail calls.**
   12 decode to `lwz r12,off(r3|rX)` / `lwz r12,off(r12)` /
   `mtctr r12` / `bctr`, or to a short guard followed by exactly that; the
   thirteenth (`0x80762B20`) is a byte-identical twin of `fn_80762000` and is
   class S. There is no table to recover, so the other twelve are class **R**.
3. **`unaff_GQR0` is a decompiler artifact, not an unknown.** 161 remaining
   exports print it. The raw decoder shows the GQR index in every `psq`
   instruction, and `qr0` is the 32-bit float pair used for `f31` saves and
   plain float movement, so those bodies are class **R**. Only `unaff_GQR3`
   (33 rows, 54 `psq` ops) is a real gap, and it is one word wide (class P).
4. **The `0x802DCBFC` forwarders are genuinely opaque.** 473 remaining rows
   are that trampoline family: 472 of exactly 48 bytes and one of 44. Each
   body is `stwu`/`mflr`, `mr r5,r3`, `mr r3,r4`, `addi r12,r5,off`,
   `bl 0x802DCBFC`, epilogue. The hidden `r12` adjustment is readable - 235
   use `+0x18`, 183 use `+0x24`, 54 use `+0xC`, one (`0x806E3BE0`) uses
   `this+0x5E4` - but the callee list behind the trampoline lives in
   `main.dol`, so no target and therefore no meaning can be stated. These
   stay **U**; they are 27 % of the unclaimed rows and 7.9 % of the
   unclaimed bytes.
5. **103 rows are already covered.** 97 are byte-identical to a body that already exists:
   distinct vtable/getter slots that return the same constant or perform the
   same empty body as an already-translated address; the largest groups are
   39 copies of `fn_80656f50` and 21 copies of `fn_806535e0`. The other
   6 (`0x80601CD0`..`0x80602FF0`) are translated in `src/game/player.rs` under
   their own `fn_` names and are only missing from the `game::bases` ledger
   slice. Class **S** - no new semantic content.

## Full reclassification table

`bytes` is the `functions.tsv` size. `prev` is the coverage-doc wave that
previously skipped the row, when one exists. Notes are generated from the
export and the raw words; for bodies of 64 bytes or less the note is the
decoded instruction stream itself.

### Class R - reconstructible now (1,176 rows, 251,100 bytes)

#### `0x8060xxxx` (44)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x80600B10` | 96 | R | prev W116; 2 direct calls; no psq; field `+0x18` |
| `0x80600B70` | 100 | R | prev W116; 2 direct calls; no psq; field `+0x18` |
| `0x80600D40` | 180 | R | ends in virtual tail call (1 indirect); no psq; field `+0x70`; string `s_charaChangeSelectArrow_02_brlyt_807d0ee8` |
| `0x80601010` | 220 | R | 1 direct call; no psq; field `+0x70` |
| `0x806010F0` | 156 | R | ends in virtual tail call (1 indirect); no psq; field `+0x70` |
| `0x80601210` | 88 | R | 1 direct call; no psq; field `+0x70` |
| `0x80601270` | 128 | R | ends in virtual tail call (2 indirect); no psq; field `+0x208` |
| `0x80601300` | 128 | R | 2 direct calls; no psq; field `+0x70` |
| `0x80601F50` | 100 | R | prev W116; 2 direct calls; no psq; field `+0x18` |
| `0x80605540` | 96 | R | prev W116; 2 direct calls; no psq; field `+0x18` |
| `0x806055A0` | 100 | R | prev W116; 2 direct calls; no psq; field `+0x18` |
| `0x80605BA0` | 1164 | R | prev W116; 3 direct calls; no psq |
| `0x80606A10` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x80606A50` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x80606A90` | 96 | R | 2 direct calls; no psq; field `+0x18` |
| `0x80606AF0` | 100 | R | 2 direct calls; no psq; field `+0x18` |
| `0x80608FD0` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x80609010` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x80609050` | 96 | R | 2 direct calls; no psq; field `+0x18` |
| `0x806090B0` | 100 | R | 2 direct calls; no psq; field `+0x18` |
| `0x80609260` | 200 | R | ends in virtual tail call (1 indirect); no psq; field `+0x70`; string `s_worldCollectionCoinDate_12_brlyt_807d2a6c` |
| `0x80609370` | 104 | R | ends in virtual tail call (2 indirect); no psq; field `+0x288` |
| `0x80609860` | 72 | R | ends in virtual tail call (1 indirect); no psq; field `+0x208` |
| `0x806098E0` | 104 | R | ends in virtual tail call (1 indirect); no psq; field `+0x28b` |
| `0x80609E70` | 88 | R | 2 direct calls; no psq |
| `0x8060A080` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x8060A0C0` | 96 | R | 2 direct calls; no psq; field `+0x18` |
| `0x8060A120` | 100 | R | 2 direct calls; no psq; field `+0x18` |
| `0x8060A5D0` | 324 | R | prev W116; 3 direct calls; prologue save-helper `0x802dcd6c`; 1 qr0 psq (float pair / f31 save) |
| `0x8060A950` | 372 | R | prev W116; 4 direct calls; no psq; string `s_Wm_2d_continue_807d30dc` |
| `0x8060C1B0` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x8060C1F0` | 96 | R | 2 direct calls; no psq; field `+0x18` |
| `0x8060C250` | 100 | R | 2 direct calls; no psq; field `+0x18` |
| `0x8060C400` | 496 | R | prev W155; ends in virtual tail call (1 indirect); no psq; field `+0x70`; string `s_corseClear_corseClear_arc_807d386c` |
| `0x8060C5F0` | 120 | R | ends in virtual tail call (2 indirect); no psq; field `+0x70` |
| `0x8060C6D0` | 240 | R | ends in virtual tail call (1 indirect); no psq; field `+0x208`; string `s_Wm_2d_courseclear_807d38d8` |
| `0x8060C840` | 88 | R | ends in virtual tail call (1 indirect); no psq; field `+0x330` |
| `0x8060C960` | 100 | R | ends in virtual tail call (1 indirect); no psq; field `+0x330` |
| `0x8060D6F0` | 64 | R | `lbz r0,+0x274(r3); cmpwi r0,0; bne 0x8060D704; li r0,+0x0; stb r0,+0x274(r3); … (8 insns)` — export truncated, raw body complete |
| `0x8060DB40` | 96 | R | ends in virtual tail call (1 indirect); no psq; field `+0x208` |
| `0x8060DBB0` | 84 | R | 3 direct calls; no psq; field `+0x70` |
| `0x8060DC10` | 112 | R | ends in virtual tail call (1 indirect); no psq; field `+0x244` |
| `0x8060F670` | 96 | R | 2 direct calls; no psq; field `+0x18` |
| `0x8060FB50` | 396 | R | prev W155; ends in virtual tail call (2 indirect); prologue save-helper `0x802dcd68`; 1 qr0 psq (float pair / f31 save); string `s_Wm_2d_titlestar01_807d49ac` |

#### `0x8061xxxx` (42)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x80610510` | 460 | R | 11 direct calls; prologue save-helper `0x802dcd6c`; no psq |
| `0x80612C90` | 64 | R | prev W131; `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x80612CD0` | 96 | R | 2 direct calls; no psq; field `+0x18` |
| `0x80613180` | 72 | R | 2 direct calls; no psq; field `+0x70` |
| `0x80613290` | 216 | R | 3 direct calls; 1 qr0 psq (float pair / f31 save); field `+0x70` |
| `0x80613530` | 156 | R | ends in virtual tail call (1 indirect); no psq; field `+0x208` |
| `0x806135F0` | 96 | R | ends in virtual tail call (1 indirect); no psq; field `+0x208` |
| `0x806138F0` | 80 | R | ends in virtual tail call (1 indirect); no psq; field `+0x208` |
| `0x80613960` | 88 | R | ends in virtual tail call (1 indirect); no psq; field `+0x208` |
| `0x806139D0` | 84 | R | 3 direct calls; no psq; field `+0x70` |
| `0x80614470` | 96 | R | 2 direct calls; no psq; field `+0x18` |
| `0x806144D0` | 100 | R | 2 direct calls; no psq; field `+0x18` |
| `0x806148F0` | 144 | R | prev W145; Ghidra truncated at the tail `b`; raw body complete; leaf, no calls; no psq; field `+0x276` |
| `0x80615560` | 64 | R | prev W131; `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x806155A0` | 96 | R | 2 direct calls; no psq; field `+0x18` |
| `0x80615600` | 100 | R | 2 direct calls; no psq; field `+0x18` |
| `0x80615A80` | 204 | R | prev W102; 3 direct calls; prologue save-helper `0x802dcd74`; 1 qr0 psq (float pair / f31 save) |
| `0x80615CC0` | 36 | R | prev W102; `lwz r0,+0x2b4(r3); lis r4,0x807c; addi r4,r4,+0x6c0c; li r5,+0x0; rlwinm r0,r0,2,0,29; … (8 insns)` — export truncated, raw body complete |
| `0x80615CF0` | 36 | R | prev W102; `lwz r0,+0x2b8(r3); lis r4,0x807c; addi r4,r4,+0x6c18; li r5,+0x0; rlwinm r0,r0,2,0,29; … (8 insns)` — export truncated, raw body complete |
| `0x80615D90` | 100 | R | prev W102; Ghidra truncated at the tail `b`; raw body complete; leaf, no calls; no psq; field `+0x27c` |
| `0x80615E00` | 60 | R | prev W102; `stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); stw r31,+0xc(r1); or r31,r3,r3; … (8 insns)` |
| `0x80615E40` | 60 | R | prev W102; `stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); stw r31,+0xc(r1); or r31,r3,r3; … (8 insns)` |
| `0x8061AFF0` | 64 | R | prev W131; `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x8061B030` | 96 | R | 2 direct calls; no psq; field `+0x18` |
| `0x8061B090` | 100 | R | 2 direct calls; no psq; field `+0x18` |
| `0x8061B260` | 496 | R | ends in virtual tail call (1 indirect); no psq; field `+0x70`; string `s_gameOver_gameOver_arc_807d7a5c` |
| `0x8061B530` | 240 | R | ends in virtual tail call (1 indirect); no psq; field `+0x208`; string `s_Wm_2d_gameover_807d7ac4` |
| `0x8061B720` | 88 | R | ends in virtual tail call (1 indirect); no psq; field `+0x330` |
| `0x8061B820` | 104 | R | ends in virtual tail call (1 indirect); no psq; field `+0x330` |
| `0x8061D790` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x8061D7D0` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x8061D810` | 96 | R | 2 direct calls; no psq; field `+0x18` |
| `0x8061D870` | 100 | R | 2 direct calls; no psq; field `+0x18` |
| `0x8061DA20` | 212 | R | ends in virtual tail call (1 indirect); no psq; field `+0xac`; string `s_messageWindow_messageWindow_arc_807d8478` |
| `0x8061DC70` | 460 | R | prev W102; 5 direct calls; no psq; field `+0x25c` |
| `0x8061E710` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x8061E750` | 96 | R | 2 direct calls; no psq; field `+0x18` |
| `0x8061E7B0` | 100 | R | 2 direct calls; no psq; field `+0x18` |
| `0x8061E980` | 312 | R | ends in virtual tail call (1 indirect); no psq; field `+0x70`; string `s_miniGameCannon_miniGameCannon_ar_807d8bac` |
| `0x8061ED70` | 212 | R | 5 direct calls; prologue save-helper `0x802dcd70`; no psq |
| `0x8061F0F0` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; li r4,+0xd; li r5,+0x0; stw r0,+0x14(r1); … (8 insns)` |
| `0x8061F2F0` | 516 | R | 4 direct calls; no psq; field `+0x274` |

#### `0x8062xxxx` (48)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x80620570` | 100 | R | prev W140; 2 direct calls; no psq; field `+0x18` |
| `0x80620A00` | 340 | R | 3 direct calls; prologue save-helper `0x802dcd6c`; 1 qr0 psq (float pair / f31 save) |
| `0x80624190` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x80624230` | 100 | R | prev W140; 2 direct calls; no psq; field `+0x18` |
| `0x80624400` | 212 | R | ends in virtual tail call (1 indirect); no psq; field `+0xac`; string `s_modelPlayArrow_modelPlayArrow_ar_807da5f0` |
| `0x806245F0` | 136 | R | 1 direct call; 1 qr0 psq (float pair / f31 save); field `+0xac` |
| `0x80624790` | 128 | R | 1 direct call; no psq; field `+0xac` |
| `0x80624880` | 172 | R | prev W140; 4 direct calls; no psq; field `+0xac` |
| `0x80624930` | 120 | R | 2 direct calls; no psq; field `+0xac` |
| `0x806249B0` | 112 | R | ends in virtual tail call (1 indirect); no psq; field `+0xac` |
| `0x80624A20` | 164 | R | prev W140; 4 direct calls; no psq; field `+0x25c` |
| `0x80624AD0` | 144 | R | 2 direct calls; no psq; field `+0xac` |
| `0x80624B60` | 112 | R | ends in virtual tail call (1 indirect); no psq; field `+0xac` |
| `0x80624BE0` | 152 | R | 2 direct calls; no psq; field `+0xac` |
| `0x80624C80` | 112 | R | ends in virtual tail call (1 indirect); no psq; field `+0xac` |
| `0x80625970` | 24 | R | prev W130; `lwz r5,+0x248(r3); lis r4,0x807e; addi r4,r4,-0x53d0; li r6,+0x1; addi r3,r3,+0x288; b 0x800B3B60` — export truncated, raw body complete |
| `0x80626900` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x806269A0` | 100 | R | prev W140; 2 direct calls; no psq; field `+0x18` |
| `0x80626B80` | 268 | R | ends in virtual tail call (1 indirect); no psq; field `+0xac`; string `s_modelPlayDate_09_brlyt_807db218` |
| `0x80627160` | 224 | R | 2 direct calls; no psq; field `+0x27c`; string `s_Wm_2d_moviecoinkira_807db248` |
| `0x80627240` | 136 | R | prev W140; 5 direct calls; no psq; field `+0xac` |
| `0x80627350` | 136 | R | prev W140; 2 direct calls; no psq; field `+0x27c` |
| `0x806273E0` | 156 | R | ends in virtual tail call (1 indirect); no psq; field `+0x280` |
| `0x80627A80` | 180 | R | ends in virtual tail call (2 indirect); no psq; field `+0x70`; string `s_modelPlayGuide_modelPlayGuide_ar_807db498` |
| `0x80627E10` | 100 | R | prev W140; 2 direct calls; no psq; field `+0x18` |
| `0x80628450` | 444 | R | 2 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x80628610` | 296 | R | 1 direct call; no psq; field `+0xb0` |
| `0x80628810` | 160 | R | prev W130; 3 direct calls; no psq; field `+0xa84` |
| `0x806288B0` | 136 | R | prev W130; 2 direct calls; no psq; field `+0xa83` |
| `0x80629240` | 416 | R | ends in virtual tail call (4 indirect); no psq; field `+0x74` |
| `0x80629530` | 324 | R | ends in virtual tail call (3 indirect); no psq; field `+0xe0` |
| `0x806296A0` | 184 | R | ends in virtual tail call (3 indirect); no psq; field `+0x74` |
| `0x80629A30` | 252 | R | ends in virtual tail call (1 indirect); prologue save-helper `0x802dcd74`; no psq |
| `0x8062B4C0` | 100 | R | prev W140; 2 direct calls; no psq; field `+0x18` |
| `0x8062BA50` | 328 | R | ends in virtual tail call (1 indirect); no psq; field `+0x70`; string `s_MultiCourseSelect_MultiCourseSel_807dc508` |
| `0x8062BCF0` | 296 | R | leaf, no calls; no psq; field `+0x500` |
| `0x8062C0B0` | 448 | R | prev W140; 6 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x8062C270` | 236 | R | leaf, no calls; no psq; field `+0x2dc` |
| `0x8062C360` | 328 | R | ends in virtual tail call (2 indirect); no psq; field `+0x40c` |
| `0x8062C4B0` | 312 | R | 3 direct calls; no psq; field `+0x404` |
| `0x8062C770` | 132 | R | prev W140; 3 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x8062C800` | 216 | R | prev W104; 4 direct calls; prologue save-helper `0x802dcd70`; no psq |
| `0x8062CFF0` | 288 | R | ends in virtual tail call (1 indirect); no psq; field `+0x32c` |
| `0x8062D120` | 156 | R | prev W104; 4 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x8062D660` | 384 | R | prev W104; 4 direct calls; 1 qr0 psq (float pair / f31 save); field `+0x70` |
| `0x8062EA90` | 196 | R | prev W104; 1 direct call; no psq; field `+0x500` |
| `0x8062ED90` | 240 | R | prev W104; 4 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x8062FFF0` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |

#### `0x8063xxxx` (45)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x80630BF0` | 484 | R | leaf, no calls; no psq; field `+0x298` |
| `0x80636F30` | 288 | R | prev W111; ends in virtual tail call (4 indirect); 1 qr0 psq (float pair / f31 save); field `+0x75c` |
| `0x80637120` | 492 | R | 2 direct calls; prologue save-helper `0x802dcd6c`; no psq |
| `0x80637310` | 100 | R | 1 direct call; no psq |
| `0x80637380` | 64 | R | `li r0,+0x4; or r5,r3,r3; li r6,+0x0; mtctr r0; lwz r0,+0x7dc(r5); … (8 insns)` |
| `0x806373C0` | 140 | R | leaf, no calls; no psq |
| `0x80637450` | 264 | R | 2 direct calls; prologue save-helper `0x802dcd70`; no psq |
| `0x80637560` | 168 | R | leaf, no calls; no psq; field `+0x7fc` |
| `0x80637610` | 204 | R | 4 direct calls; prologue save-helper `0x802dcd68`; no psq |
| `0x806376E0` | 184 | R | 1 direct call; no psq; field `+0x8d4` |
| `0x806377A0` | 368 | R | ends in virtual tail call (1 indirect); prologue save-helper `0x802dcd6c`; 1 qr0 psq (float pair / f31 save); string `s_Wm_2d_result_807df44c` |
| `0x80637910` | 300 | R | 2 direct calls; no psq; field `+0x8d0`; string `s_Wm_2d_resultno1_807df460` |
| `0x80637CB0` | 112 | R | 1 direct call; no psq; field `+0x70` |
| `0x80637D20` | 400 | R | ends in virtual tail call (1 indirect); prologue save-helper `0x802dcd64`; 1 qr0 psq (float pair / f31 save) |
| `0x80637ED0` | 240 | R | 7 direct calls; prologue save-helper `0x802dcd70`; no psq |
| `0x80638440` | 292 | R | ends in virtual tail call (2 indirect); no psq; field `+0x8c4` |
| `0x806385A0` | 192 | R | ends in virtual tail call (2 indirect); no psq; field `+0x6a8` |
| `0x80638680` | 144 | R | ends in virtual tail call (2 indirect); no psq; field `+0x6a8` |
| `0x80638720` | 124 | R | 1 direct call; no psq; field `+0x70` |
| `0x806387A0` | 112 | R | ends in virtual tail call (1 indirect); no psq; field `+0x6a8` |
| `0x80638820` | 312 | R | 1 direct call; 2 qr0 psq (float pair / f31 save); field `+0x8dc` |
| `0x80638960` | 352 | R | ends in virtual tail call (1 indirect); no psq; field `+0x6a8` |
| `0x80638AD0` | 152 | R | 1 direct call; no psq; field `+0x70` |
| `0x80638B70` | 252 | R | ends in virtual tail call (2 indirect); no psq; field `+0x6a8` |
| `0x80638C80` | 268 | R | 7 direct calls; prologue save-helper `0x802dcd70`; no psq |
| `0x80638D90` | 252 | R | ends in virtual tail call (1 indirect); no psq; field `+0x70` |
| `0x80638F00` | 248 | R | 2 direct calls; no psq; field `+0x8b4` |
| `0x80639000` | 480 | R | prev W111; ends in virtual tail call (1 indirect); prologue save-helper `0x802dcd70`; no psq |
| `0x8063A470` | 240 | R | ends in virtual tail call (1 indirect); no psq; field `+0x70`; string `s_pointResultDateFileFree_pointRes_807dff08` |
| `0x8063A560` | 84 | R | 3 direct calls; no psq |
| `0x8063A680` | 180 | R | 3 direct calls; prologue save-helper `0x802dcd68`; no psq |
| `0x8063A740` | 260 | R | leaf, no calls; no psq; field `+0x301` |
| `0x8063A850` | 100 | R | 1 direct call; no psq |
| `0x8063A8C0` | 140 | R | leaf, no calls; no psq |
| `0x8063ABE0` | 172 | R | 2 direct calls; no psq; field `+0x248` |
| `0x8063B900` | 324 | R | 3 direct calls; prologue save-helper `0x802dcd70`; 1 qr0 psq (float pair / f31 save) |
| `0x8063BA50` | 324 | R | 3 direct calls; prologue save-helper `0x802dcd70`; 1 qr0 psq (float pair / f31 save) |
| `0x8063C090` | 208 | R | prev W138; 4 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x8063D260` | 72 | R | prev W138; ends in virtual tail call (1 indirect); no psq; field `+0x208` |
| `0x8063D4E0` | 164 | R | ends in virtual tail call (1 indirect); no psq; field `+0x208` |
| `0x8063EAF0` | 312 | R | ends in virtual tail call (1 indirect); no psq; field `+0xac`; string `s_fileSelectPlayer_fileSelectPlaye_807e1154` |
| `0x8063EF90` | 204 | R | 3 direct calls; no psq; field `+0xac` |
| `0x8063F060` | 136 | R | ends in virtual tail call (2 indirect); no psq; field `+0x70` |
| `0x8063F120` | 428 | R | ends in virtual tail call (4 indirect); no psq; field `+0x70` |
| `0x8063F450` | 136 | R | 3 direct calls; no psq; field `+0xac` |

#### `0x8064xxxx` (38)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x80640AC0` | 116 | R | prev W112; Ghidra truncated at the tail `b`; raw body complete; leaf, no calls; no psq |
| `0x80642CE0` | 64 | R | prev W112; `lis r7,0x807d; lfs f1,+0x4(r5); lfs f0,-0x7cb0(r7); mulli r0,r6,276; lis r6,0x807e; … (8 insns)` — export truncated, raw body complete |
| `0x80644FE0` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x80645D70` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x80646C90` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x80648060` | 504 | R | prev W153; 3 direct calls; no psq |
| `0x80648470` | 224 | R | 1 direct call; no psq |
| `0x806488E0` | 404 | R | prev W153; ends in virtual tail call (1 indirect); no psq |
| `0x80648AA0` | 456 | R | prev W153; ends in virtual tail call (2 indirect); no psq |
| `0x80649090` | 216 | R | ends in virtual tail call (2 indirect); no psq |
| `0x80649190` | 180 | R | ends in virtual tail call (1 indirect); no psq |
| `0x80649280` | 208 | R | ends in virtual tail call (2 indirect); no psq; field `+0x394` |
| `0x80649380` | 188 | R | ends in virtual tail call (2 indirect); no psq |
| `0x80649460` | 196 | R | ends in virtual tail call (1 indirect); no psq |
| `0x80649560` | 132 | R | ends in virtual tail call (1 indirect); no psq; field `+0x394` |
| `0x8064A150` | 192 | R | 6 direct calls; no psq |
| `0x8064A210` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x8064A250` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x8064A4F0` | 40 | R | `stwu r1,-0x10(r1); mfspr r0,lr; addi r3,r3,+0x534; stw r0,+0x14(r1); bl 0x8007F950; … (8 insns)` |
| `0x8064A820` | 180 | R | 1 direct call; no psq |
| `0x8064A9E0` | 224 | R | leaf, no calls; no psq; field `+0x758` |
| `0x8064AAC0` | 272 | R | ends in virtual tail call (1 indirect); no psq; field `+0x348` |
| `0x8064B660` | 84 | R | prev W146; ends in virtual tail call (1 indirect); no psq; field `+0x3b4` |
| `0x8064D150` | 72 | R | ends in virtual tail call (1 indirect); no psq; field `+0x3f0` |
| `0x8064D650` | 292 | R | ends in virtual tail call (1 indirect); no psq; field `+0x148` |
| `0x8064DB70` | 324 | R | 4 direct calls; prologue save-helper `0x802dcd74`; 2 qr0 psq (float pair / f31 save) |
| `0x8064DCC0` | 408 | R | prev W153; 3 direct calls; prologue save-helper `0x802dcd74`; 3 qr0 psq (float pair / f31 save) |
| `0x8064DEF0` | 368 | R | prev W153; 3 direct calls; no psq; field `+0x404` |
| `0x8064E060` | 284 | R | leaf, no calls; no psq; field `+0x3fc` |
| `0x8064E1D0` | 264 | R | 2 direct calls; no psq; field `+0x3fc` |
| `0x8064EB50` | 220 | R | 8 direct calls; no psq; field `+0x7c` |
| `0x8064EC70` | 256 | R | 2 direct calls; no psq; field `+0x584` |
| `0x8064ED70` | 300 | R | 5 direct calls; no psq; field `+0x584` |
| `0x8064EEB0` | 332 | R | ends in virtual tail call (2 indirect); 1 qr0 psq (float pair / f31 save); field `+0x600` |
| `0x8064F390` | 348 | R | prev W153; ends in virtual tail call (1 indirect); 1 qr0 psq (float pair / f31 save); field `+0x5bc` |
| `0x8064F510` | 360 | R | prev W153; ends in virtual tail call (2 indirect); no psq; field `+0x5bc` |
| `0x8064F6A0` | 348 | R | prev W153; ends in virtual tail call (1 indirect); 1 qr0 psq (float pair / f31 save); field `+0x584` |
| `0x8064F820` | 360 | R | prev W112; ends in virtual tail call (2 indirect); no psq; field `+0x584` |

#### `0x8065xxxx` (32)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x806505B0` | 80 | R | 1 direct call; no psq |
| `0x80651590` | 24 | R | prev W120; `lwz r3,+0x3d8(r3); cmpwi r3,0; bne 0x806515A4; li r3,+0x0; blr; b 0x80162D00` — export truncated, raw body complete |
| `0x80651640` | 384 | R | prev W157; 4 direct calls; prologue save-helper `0x802dcd70`; 5 qr0 psq (float pair / f31 save) |
| `0x80651A60` | 452 | R | prev W157; 3 direct calls; no psq; field `+0x3d8` |
| `0x80652310` | 44 | R | prev W120; `lis r5,0x8043; lis r4,0x807d; lwz r5,-0x6200(r5); lfs f1,-0x7888(r4); lfs f2,+0x3c(r5); … (8 insns)` — export truncated, raw body complete |
| `0x80653B60` | 88 | R | 3 direct calls; no psq |
| `0x80653BC0` | 88 | R | 3 direct calls; no psq |
| `0x80653CE0` | 124 | R | leaf, no calls; no psq; field `+0x74` |
| `0x80656080` | 300 | R | 2 direct calls; no psq; field `+0x3d4` |
| `0x80656210` | 72 | R | leaf, no calls; no psq |
| `0x806564A0` | 92 | R | prev W157; 4 direct calls; no psq |
| `0x80656500` | 80 | R | 1 direct call; no psq; field `+0x3d0` |
| `0x80656570` | 124 | R | ends in virtual tail call (2 indirect); no psq; field `+0x394` |
| `0x80656610` | 164 | R | 2 direct calls; no psq; field `+0x3e0` |
| `0x80656B70` | 88 | R | 2 direct calls; no psq |
| `0x80656CF0` | 48 | R | `stwu r1,-0x10(r1); mfspr r0,lr; li r3,+0x7b0; stw r0,+0x14(r1); bl 0x801628C0; … (8 insns)` |
| `0x80656EB0` | 148 | R | ends in virtual tail call (2 indirect); no psq |
| `0x80657C10` | 260 | R | ends in virtual tail call (1 indirect); no psq; field `+0x5fa` |
| `0x80658080` | 172 | R | 4 direct calls; no psq; field `+0x584`; string `s_launch_bomb_807e60e0` |
| `0x80658140` | 280 | R | ends in virtual tail call (3 indirect); 2 qr0 psq (float pair / f31 save); field `+0x544` |
| `0x806583E0` | 180 | R | 1 direct call; no psq; field `+0xe8` |
| `0x80658610` | 104 | R | 2 direct calls; no psq; field `+0xac` |
| `0x80658690` | 276 | R | ends in virtual tail call (3 indirect); no psq; field `+0x544` |
| `0x8065C740` | 308 | R | 5 direct calls; prologue save-helper `0x802dcd68`; no psq |
| `0x8065C9A0` | 376 | R | prev W157; 5 direct calls; 3 qr0 psq (float pair / f31 save) |
| `0x8065CB20` | 3788 | R | prev W141; 11 direct calls; prologue save-helper `0x802dcd40`; 9 qr0 psq (float pair / f31 save) |
| `0x8065DCA0` | 8 | R | `li r3,+0x7; b 0x800FFF50` — export truncated, raw body complete |
| `0x8065DD50` | 1772 | R | prev W120; ends in virtual tail call (2 indirect); prologue save-helper `0x802dcd58`; 4 qr0 psq (float pair / f31 save) |
| `0x8065E440` | 20 | R | `lwz r5,+0x0(r3); lwz r0,+0x0(r4); stw r0,+0x0(r3); stw r5,+0x0(r4); blr` |
| `0x8065E460` | 128 | R | leaf, no calls; no psq |
| `0x8065E4E0` | 180 | R | leaf, no calls; no psq |
| `0x8065F090` | 1772 | R | prev W120; ends in virtual tail call (25 indirect); prologue save-helper `0x802dcd74`; no psq |

#### `0x8066xxxx` (29)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x806601B0` | 260 | R | prev W126; leaf, no calls; no psq |
| `0x80660E00` | 148 | R | leaf, no calls; no psq |
| `0x80661070` | 3432 | R | prev W141; ends in virtual tail call (17 indirect); prologue save-helper `0x802dcd5c`; 3 qr0 psq (float pair / f31 save) |
| `0x806626A0` | 92 | R | 2 direct calls; no psq |
| `0x80662700` | 12 | R | prev W126; `lis r3,0x8035; addi r3,r3,+0x2670; blr` |
| `0x806628D0` | 136 | R | ends in virtual tail call (1 indirect); no psq; field `+0x20` |
| `0x80662E10` | 296 | R | 3 direct calls; prologue save-helper `0x802dcd6c`; 2 qr0 psq (float pair / f31 save) |
| `0x80662F40` | 184 | R | 3 direct calls; prologue save-helper `0x802dcd74`; 1 qr0 psq (float pair / f31 save) |
| `0x80663050` | 296 | R | 3 direct calls; prologue save-helper `0x802dcd70`; 4 qr0 psq (float pair / f31 save) |
| `0x80663DF0` | 176 | R | prev W126; 6 direct calls; no psq |
| `0x80665150` | 12 | R | prev W126; `lis r3,0x8035; addi r3,r3,+0x2670; blr` |
| `0x80665320` | 136 | R | ends in virtual tail call (1 indirect); no psq; field `+0x20` |
| `0x80666940` | 136 | R | ends in virtual tail call (1 indirect); no psq; field `+0x20` |
| `0x806676C0` | 136 | R | ends in virtual tail call (1 indirect); no psq; field `+0x20` |
| `0x806677E0` | 76 | R | 2 direct calls; no psq |
| `0x80667CF0` | 8 | R | prev W126; `addi r3,r3,+0xac; b 0x80094560` — export truncated, raw body complete |
| `0x80667DD0` | 4 | R | prev W126; `b 0x80667DE0` |
| `0x80668CA0` | 64 | R | prev W126; `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x80668F60` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x80668FA0` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x80668FE0` | 96 | R | 2 direct calls; no psq; field `+0x18` |
| `0x80669040` | 100 | R | 2 direct calls; no psq; field `+0x18` |
| `0x806691D0` | 88 | R | prev W126; 2 direct calls; no psq |
| `0x8066B440` | 40 | R | `stwu r1,-0x10(r1); mfspr r0,lr; addi r3,r3,+0x594; stw r0,+0x14(r1); bl 0x8007F950; … (8 insns)` |
| `0x8066B470` | 12 | R | prev W126; `stfs f1,+0x588(r3); stfs f1,+0xb0(r3); blr` |
| `0x8066BE70` | 48 | R | prev W126; `stwu r1,-0x10(r1); mfspr r0,lr; li r3,+0x970; stw r0,+0x14(r1); bl 0x801628C0; … (8 insns)` |
| `0x8066C090` | 28 | R | prev W126; `stw r4,+0x5ac(r3); lis r4,0x8083; addi r4,r4,-0x67f8; lwz r12,+0x60(r3); lwz r12,+0xd4(r12); … (7 insns)` |
| `0x8066E0C0` | 472 | R | prev W126; ends in virtual tail call (2 indirect); prologue save-helper `0x802dcd70`; 4 qr0 psq (float pair / f31 save) |
| `0x8066F5E0` | 136 | R | ends in virtual tail call (1 indirect); no psq; field `+0x20` |

#### `0x8067xxxx` (34)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x80671510` | 248 | R | prev W151; 8 direct calls; 2 qr0 psq (float pair / f31 save); field `+0x584`; string `s_wait_ball_big_807e9ed0` |
| `0x80671A80` | 256 | R | prev W151; ends in virtual tail call (5 indirect); 1 qr0 psq (float pair / f31 save); field `+0x60` |
| `0x80673600` | 76 | R | leaf, no calls; no psq; field `+0xe8` |
| `0x80673C80` | 136 | R | ends in virtual tail call (1 indirect); no psq; field `+0x20` |
| `0x80674840` | 88 | R | 2 direct calls; no psq |
| `0x80675040` | 68 | R | Ghidra truncated at the tail `b`; raw body complete; leaf, no calls; no psq; field `+0x4d4` |
| `0x80675090` | 348 | R | prev W151; 7 direct calls; 1 qr0 psq (float pair / f31 save); field `+0x38d` |
| `0x80675960` | 252 | R | prev W151; ends in virtual tail call (4 indirect); 1 qr0 psq (float pair / f31 save); field `+0x430` |
| `0x80678300` | 248 | R | prev W108; 1 direct call; 1 qr0 psq (float pair / f31 save); field `+0x4c8` |
| `0x80678400` | 100 | R | prev W108; 2 direct calls; no psq; field `+0xdc` |
| `0x806787C0` | 392 | R | prev W108; ends in virtual tail call (1 indirect); prologue save-helper `0x802dcd70`; 5 qr0 psq (float pair / f31 save) |
| `0x8067A790` | 44 | R | prev W151; `lwz r0,+0x478(r3); lis r5,0x807f; addi r5,r5,-0x40c8; or r6,r3,r3; rlwinm r0,r0,2,0,29; … (8 insns)` — export truncated, raw body complete |
| `0x8067A900` | 100 | R | 4 direct calls; no psq; field `+0xac` |
| `0x8067ACD0` | 88 | R | 2 direct calls; no psq |
| `0x8067B160` | 92 | R | ends in virtual tail call (1 indirect); no psq; field `+0x3f0` |
| `0x8067B1C0` | 64 | R | `lis r4,0x8043; lfs f0,+0xac(r3); lwz r3,-0x6200(r4); lfs f1,+0x38(r3); lfs f2,+0x40(r3); … (8 insns)` |
| `0x8067B380` | 76 | R | 1 direct call; no psq; field `+0xac`; string `s_Wm_mr_fireball_hit_807ec140` |
| `0x8067B420` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); stw r31,+0xc(r1); or r31,r3,r3; … (8 insns)` |
| `0x8067BC30` | 116 | R | 4 direct calls; no psq; field `+0x1ec` |
| `0x8067BD00` | 132 | R | leaf, no calls; no psq; field `+0x348` |
| `0x8067C340` | 92 | R | 2 direct calls; no psq |
| `0x8067C570` | 136 | R | ends in virtual tail call (1 indirect); no psq; field `+0x20` |
| `0x8067C690` | 216 | R | 4 direct calls; no psq |
| `0x8067C9C0` | 80 | R | 1 direct call; no psq; field `+0x1ec` |
| `0x8067CAA0` | 100 | R | ends in virtual tail call (1 indirect); no psq; field `+0x42c`; string `s__Wm_en_pakkunfire_807ec5bb` |
| `0x8067CD10` | 76 | R | 3 direct calls; no psq; field `+0x1ec` |
| `0x8067CD60` | 132 | R | 2 direct calls; no psq; field `+0x274` |
| `0x8067D140` | 88 | R | 2 direct calls; no psq |
| `0x8067D1A0` | 92 | R | 2 direct calls; no psq |
| `0x8067D3D0` | 136 | R | ends in virtual tail call (1 indirect); no psq; field `+0x20` |
| `0x8067D4F0` | 76 | R | 2 direct calls; no psq |
| `0x8067E160` | 412 | R | prev W141; 6 direct calls; prologue save-helper `0x802dcd70`; no psq |
| `0x8067ECC0` | 188 | R | 3 direct calls; no psq; field `+0x349` |
| `0x8067F350` | 88 | R | 2 direct calls; no psq |

#### `0x8068xxxx` (26)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x80680AB0` | 244 | R | ends in virtual tail call (1 indirect); no psq |
| `0x80683200` | 420 | R | ends in virtual tail call (2 indirect); 1 qr0 psq (float pair / f31 save); field `+0x104` |
| `0x80683720` | 48 | R | `stwu r1,-0x10(r1); mfspr r0,lr; li r3,+0x8a8; stw r0,+0x14(r1); bl 0x801628C0; … (8 insns)` |
| `0x806842D0` | 76 | R | 2 direct calls; no psq |
| `0x806843B0` | 336 | R | 1 direct call; 2 qr0 psq (float pair / f31 save); field `+0x348` |
| `0x80684500` | 384 | R | 6 direct calls; prologue save-helper `0x802dcd6c`; 2 qr0 psq (float pair / f31 save) |
| `0x80684B10` | 128 | R | ends in virtual tail call (4 indirect); no psq; field `+0x60` |
| `0x80685150` | 260 | R | Ghidra truncated at the tail `b`; raw body complete; leaf, no calls; no psq; field `+0x670` |
| `0x806856B0` | 48 | R | `stwu r1,-0x10(r1); mfspr r0,lr; li r3,+0x500; stw r0,+0x14(r1); bl 0x801628C0; … (8 insns)` |
| `0x80685BA0` | 88 | R | 2 direct calls; no psq |
| `0x806860E0` | 180 | R | 1 direct call; no psq; field `+0x470` |
| `0x806861E0` | 88 | R | leaf, no calls; no psq; field `+0x348` |
| `0x80686240` | 272 | R | 3 direct calls; prologue save-helper `0x802dcd70`; 1 qr0 psq (float pair / f31 save) |
| `0x80686350` | 132 | R | ends in virtual tail call (1 indirect); no psq |
| `0x80687380` | 120 | R | ends in virtual tail call (1 indirect); no psq; field `+0x14` |
| `0x806876D0` | 88 | R | 2 direct calls; no psq |
| `0x8068AB10` | 20 | R | `lwz r0,+0x55c(r3); li r4,+0x4; rlwinm r0,r0,0,31,29; stwu r0,+0x55c(r3); b 0x800B8250` — export truncated, raw body complete |
| `0x8068B260` | 88 | R | 2 direct calls; no psq |
| `0x8068C210` | 492 | R | ends in virtual tail call (6 indirect); prologue save-helper `0x802dcd74`; no psq |
| `0x8068C5A0` | 264 | R | 3 direct calls; prologue save-helper `0x802dcd74`; 2 qr0 psq (float pair / f31 save) |
| `0x8068C940` | 312 | R | ends in virtual tail call (3 indirect); no psq; field `+0x430` |
| `0x8068CB40` | 332 | R | ends in virtual tail call (1 indirect); no psq; field `+0x4a8`; string `s_Wm_en_landsmoke_s_807ef0a4` |
| `0x8068D2C0` | 88 | R | ends in virtual tail call (1 indirect); no psq; field `+0xe8` |
| `0x8068DCA0` | 88 | R | 2 direct calls; no psq |
| `0x8068EF50` | 88 | R | 2 direct calls; no psq |
| `0x8068F0D0` | 76 | R | 2 direct calls; no psq |

#### `0x8069xxxx` (43)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x806907D0` | 120 | R | 2 direct calls; no psq |
| `0x80693060` | 20 | R | `lwz r0,+0x554(r3); li r4,+0x4; rlwinm r0,r0,0,31,29; stwu r0,+0x554(r3); b 0x800B8250` — export truncated, raw body complete |
| `0x80693A50` | 256 | R | 8 direct calls; no psq; field `+0x7c` |
| `0x80693BE0` | 360 | R | 9 direct calls; no psq; field `+0x594`; string `s_Wm_en_explosion_807f0730` |
| `0x80693D50` | 116 | R | 3 direct calls; no psq |
| `0x80693DD0` | 268 | R | 2 direct calls; 1 qr0 psq (float pair / f31 save); field `+0x4a8` |
| `0x80693F00` | 416 | R | 3 direct calls; no psq |
| `0x806940A0` | 112 | R | 2 direct calls; no psq; field `+0x594` |
| `0x806944C0` | 88 | R | 2 direct calls; no psq |
| `0x806947C0` | 148 | R | ends in virtual tail call (1 indirect); no psq; field `+0x4e` |
| `0x80694F30` | 432 | R | ends in virtual tail call (2 indirect); 1 qr0 psq (float pair / f31 save); string `s_Wm_en_sphitsmoke_807f0790` |
| `0x806952D0` | 488 | R | 7 direct calls; prologue save-helper `0x802dcd70`; 6 qr0 psq (float pair / f31 save) |
| `0x806954C0` | 192 | R | 2 direct calls; 2 qr0 psq (float pair / f31 save); field `+0x548` |
| `0x80695580` | 64 | R | `lbz r3,+0x38c(r4); addi r0,r3,-0x1; cmplwi r0,1; ble 0x8069559C; cmpwi r3,3; … (8 insns)` — export truncated, raw body complete |
| `0x806956B0` | 64 | R | `lbz r3,+0x38c(r4); addi r0,r3,-0x1; cmplwi r0,1; ble 0x806956CC; cmpwi r3,3; … (8 insns)` — export truncated, raw body complete |
| `0x80696E00` | 124 | R | 2 direct calls; no psq; field `+0x443` |
| `0x80696EA0` | 176 | R | ends in virtual tail call (1 indirect); no psq; field `+0x394` |
| `0x80696F70` | 132 | R | ends in virtual tail call (1 indirect); no psq; field `+0x102` |
| `0x80697090` | 248 | R | 2 direct calls; no psq; field `+0xac` |
| `0x80697840` | 60 | R | `stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); stw r31,+0xc(r1); or r31,r3,r3; … (8 insns)` |
| `0x80697C30` | 204 | R | ends in virtual tail call (1 indirect); no psq; field `+0x768` |
| `0x80697D00` | 184 | R | ends in virtual tail call (1 indirect); no psq |
| `0x80697DC0` | 328 | R | leaf, no calls; no psq; field `+0xb0` |
| `0x80697F10` | 324 | R | ends in virtual tail call (10 indirect); no psq; field `+0x60` |
| `0x80698090` | 244 | R | ends in virtual tail call (6 indirect); no psq; field `+0x60` |
| `0x80698A20` | 84 | R | ends in virtual tail call (1 indirect); no psq; field `+0x60` |
| `0x80699190` | 136 | R | ends in virtual tail call (1 indirect); no psq; field `+0x20` |
| `0x80699320` | 88 | R | 2 direct calls; no psq |
| `0x806998D0` | 36 | R | `stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); bl 0x800A3D00; lwz r0,+0x14(r1); … (8 insns)` |
| `0x80699940` | 236 | R | 3 direct calls; 1 qr0 psq (float pair / f31 save); field `+0xac` |
| `0x80699A30` | 292 | R | 3 direct calls; 2 qr0 psq (float pair / f31 save); field `+0xac` |
| `0x80699B60` | 160 | R | 3 direct calls; no psq |
| `0x80699C00` | 184 | R | ends in virtual tail call (1 indirect); no psq |
| `0x80699CC0` | 364 | R | leaf, no calls; no psq; field `+0xb0` |
| `0x80699E30` | 332 | R | ends in virtual tail call (10 indirect); no psq; field `+0x60` |
| `0x80699F80` | 44 | R | `lfs f1,+0xac(r3); lfs f0,+0xd98(r3); fcmpo cr0,f1,f0; blt 0x80699F9C; lfs f0,+0xd94(r3); … (8 insns)` |
| `0x80699FB0` | 44 | R | `lfs f1,+0xac(r3); lfs f0,+0xda0(r3); fcmpo cr0,f1,f0; blt 0x80699FCC; lfs f0,+0xd9c(r3); … (8 insns)` |
| `0x80699FE0` | 344 | R | ends in virtual tail call (2 indirect); no psq; field `+0x60` |
| `0x8069ACD0` | 16 | R | `li r0,+0xb4; stw r4,+0xdd0(r3); stw r0,+0xdd4(r3); blr` |
| `0x8069ACE0` | 52 | R | `stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); stw r31,+0xc(r1); or r31,r3,r3; … (8 insns)` |
| `0x8069C170` | 320 | R | ends in virtual tail call (5 indirect); no psq; field `+0x60` |
| `0x8069C620` | 336 | R | ends in virtual tail call (5 indirect); no psq; field `+0x60` |
| `0x8069FF00` | 396 | R | prev W119; ends in virtual tail call (7 indirect); no psq; field `+0x60` |

#### `0x806Axxxx` (26)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x806A24F0` | 1624 | R | prev W142; ends in virtual tail call (19 indirect); prologue save-helper `0x802dcd74`; 2 qr0 psq (float pair / f31 save) |
| `0x806A5D10` | 88 | R | 2 direct calls; no psq |
| `0x806A5D70` | 92 | R | 2 direct calls; no psq |
| `0x806A5FC0` | 136 | R | ends in virtual tail call (1 indirect); no psq; field `+0x20` |
| `0x806A6A70` | 12 | R | prev W124; `li r3,+0x136; li r4,+0x0; b 0x80162D50` — export truncated, raw body complete |
| `0x806A6A80` | 32 | R | prev W124; `lwz r4,+0x6fc(r3); lfs f2,+0xac(r3); lfs f3,+0x0(r4); lfs f1,+0x4(r4); lfs f0,+0xb0(r3); … (8 insns)` — export truncated, raw body complete |
| `0x806A6AA0` | 112 | R | prev W124; ends in virtual tail call (1 indirect); no psq; field `+0x718` |
| `0x806A81B0` | 88 | R | 2 direct calls; no psq |
| `0x806A8210` | 92 | R | 2 direct calls; no psq |
| `0x806A93D0` | 88 | R | 2 direct calls; no psq |
| `0x806A9430` | 88 | R | 2 direct calls; no psq |
| `0x806A95B0` | 136 | R | ends in virtual tail call (1 indirect); no psq; field `+0x20` |
| `0x806A9F50` | 228 | R | 4 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x806AA040` | 252 | R | 4 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x806AAB90` | 384 | R | 6 direct calls; no psq; field `+0x584`; string `s_fly_broom_807f39bc` |
| `0x806AB340` | 92 | R | 2 direct calls; no psq |
| `0x806AB560` | 136 | R | ends in virtual tail call (1 indirect); no psq; field `+0x20` |
| `0x806AB8E0` | 88 | R | 2 direct calls; no psq |
| `0x806ABBC0` | 68 | R | ends in virtual tail call (2 indirect); no psq; field `+0x394` |
| `0x806ABCD0` | 136 | R | ends in virtual tail call (4 indirect); no psq; field `+0x60` |
| `0x806AE010` | 136 | R | ends in virtual tail call (1 indirect); no psq; field `+0x20` |
| `0x806AEDF0` | 88 | R | 2 direct calls; no psq |
| `0x806AEE50` | 92 | R | 2 direct calls; no psq |
| `0x806AEEC0` | 12 | R | `lis r3,0x8083; addi r3,r3,-0x34fc; blr` |
| `0x806AF090` | 136 | R | ends in virtual tail call (1 indirect); no psq; field `+0x20` |
| `0x806AF740` | 88 | R | 2 direct calls; no psq |

#### `0x806Bxxxx` (29)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x806B1380` | 88 | R | 2 direct calls; no psq |
| `0x806B1AE0` | 40 | R | prev W118; `lis r4,0x8043; lfs f0,+0xb0(r3); lwz r4,-0x6200(r4); lfs f1,+0x44(r4); lfs f2,+0x3c(r4); … (8 insns)` — export truncated, raw body complete |
| `0x806B46F0` | 88 | R | 2 direct calls; no psq |
| `0x806B4A10` | 268 | R | ends in virtual tail call (5 indirect); no psq; field `+0x60` |
| `0x806B4B20` | 124 | R | 2 direct calls; no psq; field `+0xac` |
| `0x806B4BA0` | 456 | R | ends in virtual tail call (6 indirect); 1 qr0 psq (float pair / f31 save); field `+0x60` |
| `0x806B4ED0` | 96 | R | 1 direct call; no psq; field `+0x6dc` |
| `0x806B8CF0` | 88 | R | 2 direct calls; no psq |
| `0x806B9720` | 32 | R | prev W118; `or r5,r3,r3; lis r3,0x807f; addi r3,r3,+0x61d4; li r4,+0x0; addi r5,r5,+0xac; … (8 insns)` — export truncated, raw body complete |
| `0x806B9800` | 132 | R | 3 direct calls; 1 qr0 psq (float pair / f31 save); field `+0x484` |
| `0x806B9CB0` | 72 | R | prev W118; ends in virtual tail call (1 indirect); no psq; field `+0x60` |
| `0x806BA630` | 8 | R | `addi r3,r3,+0xac; b 0x800944F0` — export truncated, raw body complete |
| `0x806BA660` | 76 | R | 4 direct calls; no psq |
| `0x806BA820` | 88 | R | 2 direct calls; no psq |
| `0x806BAEB0` | 28 | R | `sth r4,+0x6dc(r3); lis r4,0x8083; addi r4,r4,-0x3844; lwzu r12,+0x394(r3); lwz r12,+0x18(r12); … (7 insns)` |
| `0x806BC2B0` | 92 | R | 2 direct calls; no psq |
| `0x806BC320` | 12 | R | prev W118; `lis r3,0x8083; addi r3,r3,-0x34fc; blr` |
| `0x806BC610` | 48 | R | `stwu r1,-0x10(r1); mfspr r0,lr; li r3,+0xc50; stw r0,+0x14(r1); bl 0x801628C0; … (8 insns)` |
| `0x806BC640` | 60 | R | `stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); stw r31,+0xc(r1); or r31,r3,r3; … (8 insns)` |
| `0x806BDAC0` | 148 | R | 4 direct calls; no psq; field `+0x644`; string `s_jump_st_807f69c8` |
| `0x806BE780` | 88 | R | 2 direct calls; no psq |
| `0x806BE7E0` | 92 | R | 2 direct calls; no psq |
| `0x806BE840` | 12 | R | prev W118; `lis r3,0x8035; addi r3,r3,+0x7874; blr` |
| `0x806BEB30` | 48 | R | `stwu r1,-0x10(r1); mfspr r0,lr; li r3,+0x630; stw r0,+0x14(r1); bl 0x801628C0; … (8 insns)` |
| `0x806BF080` | 220 | R | 3 direct calls; no psq; field `+0x480` |
| `0x806BF1C0` | 252 | R | prev W118; 4 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x806BF4B0` | 88 | R | 2 direct calls; no psq |
| `0x806BF630` | 48 | R | `stwu r1,-0x10(r1); mfspr r0,lr; li r3,+0x948; stw r0,+0x14(r1); bl 0x801628C0; … (8 insns)` |
| `0x806BFE60` | 104 | R | 4 direct calls; no psq; field `+0x544`; string `s_Position_Jr_807f749c` |

#### `0x806Cxxxx` (27)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x806C13B0` | 232 | R | prev W134; 5 direct calls; no psq; field `+0xdc` |
| `0x806C1B00` | 124 | R | prev W134; ends in virtual tail call (2 indirect); no psq; field `+0x394` |
| `0x806C3790` | 388 | R | prev W134; ends in virtual tail call (4 indirect); 1 qr0 psq (float pair / f31 save); field `+0x60` |
| `0x806C6E40` | 88 | R | prev W105; 2 direct calls; no psq |
| `0x806C6F00` | 92 | R | prev W134; 2 direct calls; no psq |
| `0x806C7120` | 136 | R | prev W134; ends in virtual tail call (1 indirect); no psq; field `+0x20` |
| `0x806C7240` | 136 | R | ends in virtual tail call (1 indirect); no psq; field `+0x20` |
| `0x806C7890` | 392 | R | prev W134; 5 direct calls; prologue save-helper `0x802dcd64`; 2 qr0 psq (float pair / f31 save) |
| `0x806C8200` | 88 | R | prev W134; 2 direct calls; no psq |
| `0x806C8880` | 308 | R | prev W134; 7 direct calls; no psq; field `+0x480` |
| `0x806C8F00` | 88 | R | prev W134; 2 direct calls; no psq |
| `0x806C9600` | 136 | R | 3 direct calls; no psq |
| `0x806C97D0` | 360 | R | prev W134; ends in virtual tail call (1 indirect); prologue save-helper `0x802dcd74`; no psq |
| `0x806CA820` | 88 | R | 2 direct calls; no psq |
| `0x806CAA40` | 48 | R | `stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); stw r31,+0xc(r1); or r31,r3,r3; … (8 insns)` |
| `0x806CAA70` | 88 | R | 2 direct calls; no psq |
| `0x806CB050` | 232 | R | prev W134; 1 direct call; 1 qr0 psq (float pair / f31 save); field `+0x53c` |
| `0x806CB210` | 84 | R | prev W149; 2 direct calls; no psq; field `+0x40c` |
| `0x806CBAF0` | 88 | R | 2 direct calls; no psq |
| `0x806CC530` | 228 | R | prev W134; 3 direct calls; no psq; field `+0x568` |
| `0x806CC6F0` | 384 | R | prev W134; 2 direct calls; no psq; field `+0xb0` |
| `0x806CCED0` | 88 | R | 2 direct calls; no psq |
| `0x806CEAC0` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x806CEB00` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x806CF400` | 452 | R | prev W134; 6 direct calls; prologue save-helper `0x802dcd70`; 1 qr0 psq (float pair / f31 save); string `s_tire_R_807fa5e4` |
| `0x806CFB30` | 456 | R | prev W134; 4 direct calls; prologue save-helper `0x802dcd74`; 2 qr0 psq (float pair / f31 save) |
| `0x806CFFD0` | 264 | R | prev W134; 6 direct calls; prologue save-helper `0x802dcd74`; 1 qr0 psq (float pair / f31 save) |

#### `0x806Dxxxx` (34)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x806D00E0` | 68 | R | leaf, no calls; no psq |
| `0x806D0130` | 244 | R | 5 direct calls; prologue save-helper `0x802dcd70`; 2 qr0 psq (float pair / f31 save) |
| `0x806D02A0` | 176 | R | 1 direct call; 2 qr0 psq (float pair / f31 save) |
| `0x806D0350` | 188 | R | 2 direct calls; no psq; field `+0x630` |
| `0x806D0410` | 256 | R | 4 direct calls; no psq; field `+0x640` |
| `0x806D0590` | 36 | R | `lis r4,0x8043; lbz r0,+0x430(r3); lwz r3,-0x5ff8(r4); li r5,+0x5; li r6,+0x2; … (8 insns)` — export truncated, raw body complete |
| `0x806D0780` | 120 | R | Ghidra truncated at the tail `b`; raw body complete; leaf, no calls; no psq; field `+0x430` |
| `0x806D08D0` | 184 | R | ends in virtual tail call (1 indirect); no psq; field `+0x430` |
| `0x806D16B0` | 332 | R | ends in virtual tail call (1 indirect); 1 qr0 psq (float pair / f31 save); field `+0x394` |
| `0x806D1ED0` | 28 | R | prev W122; `lis r4,0x807d; lbz r3,+0x430(r3); addi r4,r4,-0x4b14; lfs f1,+0x4(r4); extsb r3,r3; … (7 insns)` — export truncated, raw body complete |
| `0x806D21A0` | 36 | R | prev W122; `lis r4,0x8043; lbz r0,+0x430(r3); lwz r3,-0x5ff8(r4); li r5,+0x7; li r6,+0x2; … (8 insns)` — export truncated, raw body complete |
| `0x806D22B0` | 120 | R | Ghidra truncated at the tail `b`; raw body complete; leaf, no calls; no psq; field `+0x430` |
| `0x806D2390` | 364 | R | 9 direct calls; no psq; field `+0x430` |
| `0x806D26C0` | 88 | R | 2 direct calls; no psq |
| `0x806D2840` | 48 | R | `stwu r1,-0x10(r1); mfspr r0,lr; li r3,+0x778; stw r0,+0x14(r1); bl 0x801628C0; … (8 insns)` |
| `0x806D3370` | 312 | R | 2 direct calls; no psq; field `+0xb0` |
| `0x806D38D0` | 36 | R | prev W122; `lis r4,0x8043; lbz r0,+0x430(r3); lwz r3,-0x5ff8(r4); li r5,+0x5; li r6,+0x2; … (8 insns)` — export truncated, raw body complete |
| `0x806D3AC0` | 120 | R | prev W122; Ghidra truncated at the tail `b`; raw body complete; leaf, no calls; no psq; field `+0x430` |
| `0x806D4A20` | 204 | R | 1 direct call; 1 qr0 psq (float pair / f31 save); field `+0xb4`; string `s_Wm_en_burst_ss_807fae84` |
| `0x806D4D30` | 164 | R | ends in virtual tail call (1 indirect); no psq; field `+0x58c` |
| `0x806D5FB0` | 516 | R | 5 direct calls; prologue save-helper `0x802dcd74`; 3 qr0 psq (float pair / f31 save); string `s_ridden_807cb6a8` |
| `0x806D62C0` | 528 | R | ends in virtual tail call (3 indirect); prologue save-helper `0x802dcd60`; 1 qr0 psq (float pair / f31 save) |
| `0x806D6680` | 516 | R | ends in virtual tail call (1 indirect); prologue save-helper `0x802dcd74`; no psq; string `s_warning_807fb0e0` |
| `0x806D6950` | 536 | R | ends in virtual tail call (1 indirect); prologue save-helper `0x802dcd6c`; no psq |
| `0x806D6E30` | 36 | R | `stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); bl 0x806D6270; lwz r0,+0x14(r1); … (8 insns)` |
| `0x806D7A30` | 204 | R | 1 direct call; 1 qr0 psq (float pair / f31 save); field `+0xb4`; string `s_Wm_en_burst_ss_807fb31c` |
| `0x806D7D60` | 164 | R | ends in virtual tail call (1 indirect); no psq; field `+0xb0` |
| `0x806D86E0` | 384 | R | ends in virtual tail call (3 indirect); prologue save-helper `0x802dcd74`; no psq |
| `0x806DB300` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x806DD1A0` | 332 | R | ends in virtual tail call (4 indirect); no psq; field `+0x394` |
| `0x806DD550` | 248 | R | ends in virtual tail call (1 indirect); no psq; field `+0x3f8` |
| `0x806DEFD0` | 292 | R | ends in virtual tail call (5 indirect); no psq; field `+0x394` |
| `0x806DFA60` | 472 | R | ends in virtual tail call (2 indirect); prologue save-helper `0x802dcd70`; no psq |
| `0x806DFDB0` | 408 | R | ends in virtual tail call (3 indirect); no psq; field `+0xe5` |

#### `0x806Exxxx` (30)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x806E0110` | 532 | R | ends in virtual tail call (3 indirect); no psq; field `+0x6a9` |
| `0x806E0E20` | 12 | R | `lwz r0,+0x0(r4); stw r0,+0x700(r3); blr` |
| `0x806E0EA0` | 92 | R | prev W109; ends in virtual tail call (1 indirect); no psq; field `+0x394` |
| `0x806E12B0` | 76 | R | 2 direct calls; no psq; field `+0x3d0` |
| `0x806E1780` | 76 | R | 2 direct calls; no psq |
| `0x806E2950` | 440 | R | prev W109; 5 direct calls; prologue save-helper `0x802dcd5c`; no psq |
| `0x806E2BF0` | 96 | R | prev W137; leaf, no calls; no psq; field `+0x430` |
| `0x806E2E40` | 272 | R | 2 direct calls; no psq; field `+0x8c6` |
| `0x806E3370` | 4 | R | prev W109; `b 0x806E3480` |
| `0x806E33A0` | 224 | R | ends in virtual tail call (1 indirect); no psq; field `+0x20` |
| `0x806E3FC0` | 316 | R | prev W109; 8 direct calls; prologue save-helper `0x802dcd5c`; 3 qr0 psq (float pair / f31 save); string `s_test_lift_807fcdec` |
| `0x806E5370` | 420 | R | 9 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x806E5580` | 120 | R | prev W137; ends in virtual tail call (1 indirect); no psq; field `+0x3f0` |
| `0x806E64D0` | 72 | R | prev W109; 2 direct calls; no psq; field `+0x588` |
| `0x806E7350` | 120 | R | prev W137; ends in virtual tail call (1 indirect); no psq; field `+0x3f0` |
| `0x806E7450` | 56 | R | `stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); stw r31,+0xc(r1); or r31,r3,r3; … (8 insns)` |
| `0x806E7AA0` | 148 | R | 5 direct calls; no psq; field `+0x418` |
| `0x806E7F50` | 160 | R | 7 direct calls; no psq; field `+0x38d` |
| `0x806E7FF0` | 324 | R | ends in virtual tail call (3 indirect); no psq; field `+0x7c` |
| `0x806E8340` | 288 | R | ends in virtual tail call (1 indirect); no psq; field `+0x694` |
| `0x806E8990` | 16 | R | prev W109; `lis r5,0x807d; addi r4,r3,+0xac; lfs f1,-0x4204(r5); b 0x80097170` — export truncated, raw body complete |
| `0x806E9AC0` | 92 | R | 2 direct calls; no psq |
| `0x806E9B20` | 12 | R | `lis r3,0x8036; addi r3,r3,-0x7efc; blr` |
| `0x806EC2B0` | 200 | R | prev W109; ends in virtual tail call (1 indirect); 1 qr0 psq (float pair / f31 save) |
| `0x806EC380` | 148 | R | prev W109; 3 direct calls; 1 qr0 psq (float pair / f31 save) |
| `0x806ED160` | 252 | R | prev W109; ends in virtual tail call (2 indirect); prologue save-helper `0x802dcd6c`; no psq |
| `0x806ED260` | 176 | R | prev W109; 3 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x806ED340` | 288 | R | prev W109; 2 direct calls; 2 qr0 psq (float pair / f31 save); field `+0x1024` |
| `0x806EECA0` | 384 | R | prev W109; 4 direct calls; prologue save-helper `0x802dcd68`; 2 qr0 psq (float pair / f31 save) |
| `0x806EF9E0` | 148 | R | 1 direct call; no psq; field `+0x448` |

#### `0x806Fxxxx` (46)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x806F0060` | 60 | R | `stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); stw r31,+0xc(r1); or r31,r3,r3; … (8 insns)` |
| `0x806F0DB0` | 356 | R | ends in virtual tail call (1 indirect); prologue save-helper `0x802dcd5c`; no psq |
| `0x806F0F20` | 496 | R | prev W154; 3 direct calls; 1 qr0 psq (float pair / f31 save); field `+0x3f0` |
| `0x806F1210` | 276 | R | 1 direct call; no psq; field `+0x348` |
| `0x806F1D30` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x806F2100` | 520 | R | prev W154; 3 direct calls; prologue save-helper `0x802dcd64`; 4 qr0 psq (float pair / f31 save) |
| `0x806F2420` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x806F2560` | 76 | R | 3 direct calls; no psq |
| `0x806F25B0` | 340 | R | 3 direct calls; prologue save-helper `0x802dcd64`; 2 qr0 psq (float pair / f31 save) |
| `0x806F2710` | 520 | R | prev W154; 3 direct calls; prologue save-helper `0x802dcd64`; 4 qr0 psq (float pair / f31 save) |
| `0x806F2E50` | 436 | R | 9 direct calls; no psq |
| `0x806F3010` | 100 | R | 1 direct call; no psq |
| `0x806F3080` | 100 | R | 1 direct call; no psq |
| `0x806F42F0` | 332 | R | ends in virtual tail call (1 indirect); prologue save-helper `0x802dcd70`; no psq |
| `0x806F4BE0` | 140 | R | 3 direct calls; no psq; field `+0x7c` |
| `0x806F4C80` | 184 | R | 4 direct calls; no psq; field `+0x40` |
| `0x806F5640` | 292 | R | ends in virtual tail call (2 indirect); no psq; field `+0x524`; string `s_W7_shiroboss_bg_M_80801554` |
| `0x806F6120` | 136 | R | ends in virtual tail call (1 indirect); no psq; field `+0x20` |
| `0x806F6240` | 212 | R | 8 direct calls; no psq |
| `0x806F64B0` | 492 | R | prev W154; ends in virtual tail call (1 indirect); no psq; field `+0x580`; string `s_MG_balloon_80801bc4` |
| `0x806F6700` | 220 | R | 8 direct calls; no psq; field `+0x7c` |
| `0x806F7430` | 140 | R | 5 direct calls; no psq; field `+0x5f0` |
| `0x806F7500` | 396 | R | ends in virtual tail call (1 indirect); no psq; field `+0x574` |
| `0x806F7830` | 220 | R | 8 direct calls; no psq; field `+0x7c` |
| `0x806F8050` | 116 | R | 2 direct calls; no psq; field `+0x14b` |
| `0x806F8160` | 252 | R | 1 direct call; no psq; field `+0x104` |
| `0x806F86C0` | 88 | R | 2 direct calls; no psq |
| `0x806F8720` | 92 | R | 2 direct calls; no psq |
| `0x806F8950` | 136 | R | ends in virtual tail call (1 indirect); no psq; field `+0x20` |
| `0x806FB740` | 120 | R | prev W113; 1 direct call; no psq |
| `0x806FB7C0` | 188 | R | prev W113; 4 direct calls; prologue save-helper `0x802dcd70`; 1 qr0 psq (float pair / f31 save) |
| `0x806FC840` | 88 | R | 2 direct calls; no psq |
| `0x806FCA20` | 88 | R | 2 direct calls; no psq; field `+0x60` |
| `0x806FD7A0` | 124 | R | ends in virtual tail call (1 indirect); no psq; field `+0x464`; string `s_Wm_mg_panelperfect_808032f8` |
| `0x806FDBB0` | 152 | R | 4 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x806FDF80` | 152 | R | 4 direct calls; no psq; field `+0x1d4` |
| `0x806FE020` | 308 | R | prev W113; ends in virtual tail call (1 indirect); no psq; field `+0x194` |
| `0x806FE170` | 156 | R | 1 direct call; no psq; field `+0x208` |
| `0x806FE210` | 116 | R | 4 direct calls; no psq; field `+0x208` |
| `0x806FE290` | 116 | R | ends in virtual tail call (1 indirect); no psq |
| `0x806FE310` | 212 | R | ends in virtual tail call (2 indirect); no psq; field `+0x208` |
| `0x806FE3F0` | 140 | R | 3 direct calls; no psq; field `+0x214` |
| `0x806FE810` | 68 | R | 2 direct calls; no psq; field `+0x1d4` |
| `0x806FE860` | 436 | R | 11 direct calls; no psq; field `+0x21c` |
| `0x806FF390` | 96 | R | leaf, no calls; no psq; field `+0x39c` |
| `0x806FFC50` | 212 | R | prev W113; 6 direct calls; no psq |

#### `0x8070xxxx` (39)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x80700320` | 160 | R | ends in virtual tail call (1 indirect); no psq; field `+0xac` |
| `0x807003C0` | 492 | R | ends in virtual tail call (1 indirect); prologue save-helper `0x802dcd6c`; 4 qr0 psq (float pair / f31 save); string `s_Wm_jr_electricstart_80803a70` |
| `0x80701390` | 416 | R | 8 direct calls; prologue save-helper `0x802dcd60`; 1 qr0 psq (float pair / f31 save) |
| `0x80702FD0` | 516 | R | 5 direct calls; prologue save-helper `0x802dcd74`; 3 qr0 psq (float pair / f31 save) |
| `0x80703940` | 248 | R | ends in virtual tail call (1 indirect); prologue save-helper `0x802dcd68`; no psq |
| `0x80703A40` | 252 | R | 6 direct calls; no psq; field `+0xe0f` |
| `0x80704790` | 208 | R | 5 direct calls; no psq; field `+0xac` |
| `0x80705600` | 260 | R | 4 direct calls; no psq |
| `0x807057C0` | 204 | R | ends in virtual tail call (1 indirect); no psq; field `+0x3b8` |
| `0x80705890` | 172 | R | 1 direct call; 2 qr0 psq (float pair / f31 save); field `+0xac` |
| `0x80705B00` | 116 | R | 4 direct calls; no psq; field `+0x3b4` |
| `0x80705C20` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x80706110` | 220 | R | 8 direct calls; no psq; field `+0x7c` |
| `0x807061F0` | 56 | R | `lwz r0,+0x0(r3); stw r0,+0x43c(r3); stw r0,+0x448(r3); stw r0,+0x454(r3); stw r0,+0x460(r3); … (8 insns)` |
| `0x80706230` | 80 | R | leaf, no calls; no psq; field `+0x3fc` |
| `0x80706280` | 224 | R | 2 direct calls; no psq |
| `0x80706360` | 164 | R | leaf, no calls; no psq; field `+0x438` |
| `0x80706410` | 44 | R | `li r0,+0xc; addi r3,r3,+0x438; mtctr r0; ori r0,r0,0x0; lwz r0,+0x0(r3); … (8 insns)` |
| `0x80706440` | 144 | R | 1 direct call; no psq; field `+0x400` |
| `0x80706510` | 340 | R | ends in virtual tail call (1 indirect); no psq; field `+0xff` |
| `0x80706690` | 340 | R | ends in virtual tail call (2 indirect); no psq; field `+0xff` |
| `0x807067F0` | 84 | R | 2 direct calls; no psq |
| `0x80706980` | 140 | R | 6 direct calls; no psq; field `+0x438` |
| `0x80706BD0` | 88 | R | 2 direct calls; no psq |
| `0x80706D50` | 76 | R | 2 direct calls; no psq |
| `0x807073D0` | 48 | R | `lis r4,0x8043; lbz r5,+0x3e1(r3); addi r4,r4,-0x3e60; lwz r0,+0x4(r3); x31/87; … (8 insns)` |
| `0x807074B0` | 236 | R | ends in virtual tail call (1 indirect); no psq; field `+0x104` |
| `0x80707620` | 404 | R | ends in virtual tail call (2 indirect); no psq; field `+0x104` |
| `0x80707800` | 224 | R | 1 direct call; no psq; field `+0x3d0` |
| `0x80707F40` | 272 | R | 1 direct call; no psq; field `+0x3d4` |
| `0x80708070` | 240 | R | 3 direct calls; no psq; field `+0x3e6` |
| `0x80708160` | 88 | R | 2 direct calls; no psq |
| `0x807082D0` | 88 | R | 2 direct calls; no psq |
| `0x80709E90` | 48 | R | `stwu r1,-0x10(r1); mfspr r0,lr; li r3,+0x828; stw r0,+0x14(r1); bl 0x801628C0; … (8 insns)` |
| `0x80709EC0` | 184 | R | 7 direct calls; no psq; field `+0x584` |
| `0x8070F0F0` | 88 | R | 2 direct calls; no psq |
| `0x8070F7C0` | 500 | R | 3 direct calls; no psq; field `+0x694` |
| `0x8070F9C0` | 232 | R | prev W110; leaf, no calls; no psq |
| `0x8070FE40` | 460 | R | 10 direct calls; no psq; field `+0x544` |

#### `0x8071xxxx` (45)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x807103D0` | 296 | R | prev W110; 1 direct call; no psq; field `+0x7b0` |
| `0x80711F60` | 204 | R | 4 direct calls; no psq; string `s_obj_kinoko_80806b90` |
| `0x80714370` | 136 | R | ends in virtual tail call (1 indirect); no psq; field `+0x20` |
| `0x807169E0` | 92 | R | ends in virtual tail call (1 indirect); no psq; field `+0x670` |
| `0x80716A70` | 92 | R | ends in virtual tail call (1 indirect); no psq; field `+0x690` |
| `0x80716B60` | 92 | R | ends in virtual tail call (1 indirect); no psq; field `+0xb0` |
| `0x80717960` | 88 | R | 2 direct calls; no psq |
| `0x80717AE0` | 148 | R | 7 direct calls; no psq |
| `0x80717B80` | 136 | R | 3 direct calls; no psq; field `+0x4a4` |
| `0x80717CF0` | 152 | R | 3 direct calls; no psq |
| `0x80717EF0` | 116 | R | ends in virtual tail call (1 indirect); no psq; field `+0x394` |
| `0x80718960` | 100 | R | ends in virtual tail call (1 indirect); no psq; field `+0x5bc` |
| `0x807189D0` | 136 | R | 2 direct calls; no psq; field `+0x5cd` |
| `0x80718CF0` | 388 | R | ends in virtual tail call (1 indirect); 1 qr0 psq (float pair / f31 save); field `+0x694` |
| `0x80719250` | 336 | R | 10 direct calls; no psq; field `+0x544` |
| `0x807193E0` | 40 | R | `stwu r1,-0x10(r1); mfspr r0,lr; addi r3,r3,+0x5b0; stw r0,+0x14(r1); bl 0x8007F950; … (8 insns)` |
| `0x80719460` | 316 | R | 5 direct calls; 1 qr0 psq (float pair / f31 save); field `+0x694` |
| `0x807195A0` | 512 | R | prev W139; ends in virtual tail call (6 indirect); 1 qr0 psq (float pair / f31 save); field `+0x698`; string `s_Wm_ro_pipeappear_808083c0` |
| `0x80719B80` | 88 | R | 2 direct calls; no psq |
| `0x80719D00` | 132 | R | 5 direct calls; no psq |
| `0x8071A230` | 132 | R | 5 direct calls; no psq; field `+0x584` |
| `0x8071AD90` | 80 | R | ends in virtual tail call (1 indirect); no psq; field `+0x544` |
| `0x8071B8E0` | 168 | R | 4 direct calls; 1 qr0 psq (float pair / f31 save); string `s_lift_turi_ashiba_80809244` |
| `0x8071B990` | 44 | R | `lis r4,0x807d; addi r4,r4,-0x30b0; lfs f0,+0x14(r4); lfs f1,+0x18(r4); lfs f2,+0x1c(r4); … (8 insns)` |
| `0x8071BA20` | 236 | R | 3 direct calls; no psq; field `+0xb4` |
| `0x8071BBB0` | 148 | R | 7 direct calls; no psq |
| `0x8071BC50` | 132 | R | leaf, no calls; no psq; field `+0x324` |
| `0x8071BD20` | 216 | R | 5 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x8071C180` | 304 | R | ends in virtual tail call (1 indirect); prologue save-helper `0x802dcd60`; no psq |
| `0x8071D120` | 128 | R | leaf, no calls; no psq |
| `0x8071D1A0` | 416 | R | 8 direct calls; 1 qr0 psq (float pair / f31 save) |
| `0x8071D340` | 156 | R | ends in virtual tail call (1 indirect); no psq |
| `0x8071DBF0` | 204 | R | 2 direct calls; 2 qr0 psq (float pair / f31 save); field `+0xb0` |
| `0x8071E060` | 204 | R | 4 direct calls; 1 qr0 psq (float pair / f31 save) |
| `0x8071EAA0` | 88 | R | 2 direct calls; no psq |
| `0x8071ED20` | 132 | R | 3 direct calls; no psq |
| `0x8071EEE0` | 204 | R | ends in virtual tail call (1 indirect); no psq; field `+0x3b8` |
| `0x8071EFB0` | 152 | R | 4 direct calls; no psq; string `s_obj_yashi_wait_807cd064` |
| `0x8071F0B0` | 104 | R | 3 direct calls; no psq; field `+0x3b4`; string `s_Wm_mr_palm_s_807cd070` |
| `0x8071F390` | 412 | R | ends in virtual tail call (3 indirect); prologue save-helper `0x802dcd5c`; no psq |
| `0x8071F780` | 48 | R | `stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); lwzu r12,+0x3b4(r3); lwz r12,+0x14(r12); … (8 insns)` |
| `0x8071F7B0` | 36 | R | prev W139; `stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); bl 0x8071F340; lwz r0,+0x14(r1); … (8 insns)` |
| `0x8071FB10` | 48 | R | `stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); lwzu r12,+0x6b8(r3); lwz r12,+0x14(r12); … (8 insns)` |
| `0x8071FB40` | 40 | R | `stwu r1,-0x10(r1); mfspr r0,lr; addi r3,r3,+0x534; stw r0,+0x14(r1); bl 0x8007F950; … (8 insns)` |
| `0x8071FB70` | 204 | R | 6 direct calls; no psq; field `+0x7c` |

#### `0x8072xxxx` (23)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x807204B0` | 136 | R | ends in virtual tail call (1 indirect); no psq; field `+0x20` |
| `0x807208D0` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x80720920` | 48 | R | `stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); lwzu r12,+0x70(r3); lwz r12,+0x10(r12); … (8 insns)` |
| `0x80720980` | 36 | R | prev W125; `cmpwi r4,0; beq 0x80720998; lis r3,0x8043; li r4,+0x0; lwz r3,-0x636c(r3); … (8 insns)` — export truncated, raw body complete |
| `0x807209B0` | 88 | R | 1 direct call; no psq; field `+0xb4` |
| `0x807211D0` | 24 | R | prev W125; `li r0,+0x1; stb r0,+0x5b8(r3); li r4,+0x1; li r5,+0x0; addi r3,r3,+0x5c0; b 0x80166F40` — export truncated, raw body complete |
| `0x80721270` | 28 | R | `stw r4,+0x63c(r3); lis r4,0x8083; addi r4,r4,+0x290; lwz r12,+0x60(r3); lwz r12,+0xd4(r12); … (7 insns)` |
| `0x80721290` | 8 | R | prev W125; `stw r4,+0x638(r3); blr` |
| `0x80721800` | 484 | R | prev W125; ends in virtual tail call (2 indirect); no psq; field `+0x628` |
| `0x80722560` | 88 | R | prev W125; 2 direct calls; no psq |
| `0x807225C0` | 92 | R | 2 direct calls; no psq |
| `0x807227E0` | 136 | R | ends in virtual tail call (1 indirect); no psq; field `+0x20` |
| `0x807238F0` | 92 | R | 2 direct calls; no psq |
| `0x80723B10` | 136 | R | ends in virtual tail call (1 indirect); no psq; field `+0x20` |
| `0x807242C0` | 88 | R | prev W125; Ghidra truncated at the tail `b`; raw body complete; leaf, no calls; no psq; field `+0x47c` |
| `0x80724320` | 88 | R | prev W125; Ghidra truncated at the tail `b`; raw body complete; leaf, no calls; no psq; field `+0x47c` |
| `0x807262C0` | 48 | R | `stwu r1,-0x10(r1); mfspr r0,lr; li r3,+0x890; stw r0,+0x14(r1); bl 0x801628C0; … (8 insns)` |
| `0x80727180` | 16 | R | prev W125; `lis r5,0x807d; addi r4,r3,+0xac; lfs f1,-0x2d88(r5); b 0x80097170` — export truncated, raw body complete |
| `0x80728CC0` | 424 | R | ends in virtual tail call (1 indirect); prologue save-helper `0x802dcd74`; 1 qr0 psq (float pair / f31 save); string `s_remo_lift_soroban_8080b598` |
| `0x8072C330` | 160 | R | ends in virtual tail call (1 indirect); no psq; field `+0x4a0` |
| `0x8072C500` | 52 | R | `stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); stw r31,+0xc(r1); or r31,r3,r3; … (8 insns)` |
| `0x8072C830` | 88 | R | 2 direct calls; no psq |
| `0x8072FBA0` | 188 | R | 3 direct calls; no psq; field `+0x4e8` |

#### `0x8073xxxx` (35)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x80730BA0` | 64 | R | prev W128; `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x80731330` | 80 | R | prev W128; 1 direct call; no psq; field `+0x4d8` |
| `0x80731380` | 464 | R | 7 direct calls; prologue save-helper `0x802dcd74`; 1 qr0 psq (float pair / f31 save); string `s_houdai_rot_large_gun_8080d044` |
| `0x807316F0` | 144 | R | ends in virtual tail call (1 indirect); no psq; field `+0x9ec` |
| `0x80731D10` | 540 | R | 1 direct call; no psq; field `+0x9f8` |
| `0x80731FB0` | 64 | R | prev W128; `stwu r1,-0x10(r1); mfspr r0,lr; lis r4,0x8043; stw r0,+0x14(r1); lwz r4,-0x6040(r4); … (8 insns)` |
| `0x807337D0` | 304 | R | 10 direct calls; no psq; field `+0x7c` |
| `0x80733900` | 312 | R | 5 direct calls; 1 qr0 psq (float pair / f31 save); string `s_houdai_slope_gun_8080d5c4` |
| `0x80733B00` | 292 | R | 5 direct calls; no psq; field `+0x634`; string `s_Wm_en_burst_ss_8080d618` |
| `0x80733C30` | 340 | R | 5 direct calls; no psq; field `+0x634`; string `s_Wm_en_burst_ss_8080d618` |
| `0x80733D90` | 136 | R | leaf, no calls; no psq; field `+0xac` |
| `0x80733EC0` | 68 | R | leaf, no calls; no psq; field `+0x64c` |
| `0x80733F10` | 88 | R | leaf, no calls; no psq |
| `0x80733F70` | 428 | R | prev W128; 1 direct call; no psq; field `+0x65c` |
| `0x80734170` | 148 | R | ends in virtual tail call (1 indirect); no psq; field `+0x63c` |
| `0x80734240` | 224 | R | ends in virtual tail call (1 indirect); no psq; field `+0x644` |
| `0x80734340` | 192 | R | ends in virtual tail call (1 indirect); no psq; field `+0x644` |
| `0x80734B00` | 220 | R | ends in virtual tail call (1 indirect); 1 qr0 psq (float pair / f31 save); field `+0x104` |
| `0x80734D00` | 100 | R | ends in virtual tail call (1 indirect); no psq |
| `0x80736890` | 284 | R | prev W128; 3 direct calls; prologue save-helper `0x802dcd70`; 4 qr0 psq (float pair / f31 save) |
| `0x80737270` | 284 | R | 3 direct calls; prologue save-helper `0x802dcd70`; 4 qr0 psq (float pair / f31 save) |
| `0x807382F0` | 256 | R | prev W101; 8 direct calls; 2 qr0 psq (float pair / f31 save); field `+0x468` |
| `0x80738460` | 276 | R | prev W101; 7 direct calls; 1 qr0 psq (float pair / f31 save) |
| `0x80738DB0` | 248 | R | prev W101; 3 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x8073AF20` | 256 | R | prev W128; ends in virtual tail call (1 indirect); no psq; field `+0x7c` |
| `0x8073B1B0` | 104 | R | prev W128; ends in virtual tail call (1 indirect); no psq; field `+0xb0` |
| `0x8073B240` | 80 | R | prev W128; ends in virtual tail call (2 indirect); no psq; field `+0x430` |
| `0x8073B290` | 152 | R | 6 direct calls; no psq; field `+0x488` |
| `0x8073B440` | 88 | R | prev W128; 2 direct calls; no psq |
| `0x8073C120` | 88 | R | prev W128; 2 direct calls; no psq |
| `0x8073C730` | 220 | R | prev W128; 8 direct calls; no psq; field `+0x7c` |
| `0x8073DA10` | 60 | R | prev W128; `lwz r0,+0x8e0(r3); cmpwi r0,1; blr; lwz r4,+0x8f4(r3); cmpwi r4,0; … (8 insns)` — export truncated, raw body complete |
| `0x8073F440` | 416 | R | prev W128; ends in virtual tail call (1 indirect); no psq; field `+0x3b4` |
| `0x8073F810` | 204 | R | prev W128; 6 direct calls; prologue save-helper `0x802dcd68`; no psq; string `s_node01_807cdf28` |
| `0x8073FF60` | 508 | R | prev W128; 3 direct calls; prologue save-helper `0x802dcd68`; 2 qr0 psq (float pair / f31 save) |

#### `0x8074xxxx` (33)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x80740A60` | 204 | R | 6 direct calls; prologue save-helper `0x802dcd68`; no psq; string `s_node01_807cdff0` |
| `0x80740C90` | 388 | R | ends in virtual tail call (1 indirect); prologue save-helper `0x802dcd70`; 1 qr0 psq (float pair / f31 save) |
| `0x80741CA0` | 4 | R | prev W117; `b 0x80741A70` |
| `0x807422D0` | 20 | R | prev W150; `lwz r3,+0x3e8(r3); cmpwi r3,0; blr; li r3,+0x4; b 0x800B2EE0` — export truncated, raw body complete |
| `0x80742470` | 364 | R | 6 direct calls; prologue save-helper `0x802dcd58`; no psq |
| `0x80743480` | 136 | R | ends in virtual tail call (1 indirect); no psq; field `+0x20` |
| `0x807435A0` | 288 | R | 6 direct calls; no psq |
| `0x807436C0` | 204 | R | ends in virtual tail call (1 indirect); no psq; field `+0x394` |
| `0x80743790` | 392 | R | ends in virtual tail call (1 indirect); no psq; field `+0x3f0`; string `s_center_fire_808102f4` |
| `0x80743920` | 20 | R | prev W150; `lwzu r12,+0x5d0(r3); li r5,+0x2; lwz r12,+0x8(r12); mtctr r12; bctr` |
| `0x80743AF0` | 136 | R | 5 direct calls; no psq; field `+0x3f0` |
| `0x80745C40` | 204 | R | ends in virtual tail call (1 indirect); no psq; field `+0x3b8` |
| `0x80745D10` | 176 | R | 1 direct call; 2 qr0 psq (float pair / f31 save); field `+0xb0` |
| `0x80746400` | 268 | R | 2 direct calls; no psq; field `+0x3d0` |
| `0x80746910` | 344 | R | 4 direct calls; prologue save-helper `0x802dcd70`; 3 qr0 psq (float pair / f31 save) |
| `0x80746C20` | 424 | R | 6 direct calls; prologue save-helper `0x802dcd74`; 1 qr0 psq (float pair / f31 save) |
| `0x80746F90` | 400 | R | 2 direct calls; 1 qr0 psq (float pair / f31 save); field `+0x3d0` |
| `0x80747120` | 192 | R | 4 direct calls; 1 qr0 psq (float pair / f31 save); field `+0x38f` |
| `0x807492B0` | 84 | R | prev W117; 2 direct calls; no psq |
| `0x80749310` | 56 | R | prev W117; `stwu r1,-0x10(r1); mfspr r0,lr; lis r6,0x807d; stw r0,+0x14(r1); addi r6,r6,-0x1af8; … (8 insns)` |
| `0x80749C50` | 164 | R | prev W117; 1 direct call; no psq |
| `0x8074A4F0` | 256 | R | 1 direct call; 2 qr0 psq (float pair / f31 save) |
| `0x8074B8D0` | 320 | R | 4 direct calls; prologue save-helper `0x802dcd74`; 5 qr0 psq (float pair / f31 save) |
| `0x8074BB00` | 248 | R | ends in virtual tail call (1 indirect); 1 qr0 psq (float pair / f31 save); field `+0x3d8` |
| `0x8074C3D0` | 360 | R | 4 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x8074C990` | 164 | R | ends in virtual tail call (1 indirect); no psq |
| `0x8074CA70` | 228 | R | 6 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x8074DAD0` | 56 | R | `stwu r1,-0x10(r1); mfspr r0,lr; li r4,+0x184; stw r0,+0x14(r1); bl 0x8006CF10; … (8 insns)` |
| `0x8074DB30` | 112 | R | 1 direct call; 1 qr0 psq (float pair / f31 save); field `+0x3d0` |
| `0x8074DBA0` | 360 | R | 5 direct calls; 1 qr0 psq (float pair / f31 save) |
| `0x8074DD30` | 248 | R | 4 direct calls; no psq; field `+0x3d4` |
| `0x8074EFF0` | 220 | R | 5 direct calls; no psq |
| `0x8074F600` | 396 | R | ends in virtual tail call (1 indirect); prologue save-helper `0x802dcd74`; no psq; string `s_Wm_mr_landsmoke_80812390` |

#### `0x8075xxxx` (39)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x807529B0` | 260 | R | prev W144; ends in virtual tail call (1 indirect); prologue save-helper `0x802dcd74`; no psq |
| `0x80752AC0` | 200 | R | leaf, no calls; no psq |
| `0x807537D0` | 236 | R | prev W144; leaf, no calls; no psq |
| `0x807538E0` | 196 | R | ends in virtual tail call (1 indirect); no psq |
| `0x807539E0` | 284 | R | prev W144; ends in virtual tail call (1 indirect); no psq |
| `0x80753B80` | 352 | R | prev W144; 11 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x80754430` | 308 | R | prev W106; 5 direct calls; 1 qr0 psq (float pair / f31 save); field `+0xb0` |
| `0x80754570` | 36 | R | prev W106; `lis r4,0x8043; li r0,+0x1; lwz r4,-0x6230(r4); addis r4,r4,0x9; stb r0,+0x2c(r4); … (8 insns)` |
| `0x80756FD0` | 136 | R | 4 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x807571C0` | 28 | R | prev W144; `lbz r3,+0x63d(r3); lis r5,0x807d; lis r4,0x807d; lfs f1,-0x1628(r5); lfs f2,-0x1658(r4); … (7 insns)` — export truncated, raw body complete |
| `0x807571E0` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); bl 0x807571C0; extsh r0,r3; … (8 insns)` |
| `0x80757280` | 120 | R | Ghidra truncated at the tail `b`; raw body complete; leaf, no calls; no psq; field `+0x63d` |
| `0x80757A80` | 48 | R | `stwu r1,-0x10(r1); mfspr r0,lr; li r3,+0x384; stw r0,+0x14(r1); bl 0x801628C0; … (8 insns)` |
| `0x80757D30` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x80757D70` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x80757DB0` | 96 | R | 2 direct calls; no psq; field `+0x18` |
| `0x80757E10` | 100 | R | 2 direct calls; no psq; field `+0x18` |
| `0x80758010` | 316 | R | prev W144; 2 direct calls; no psq; field `+0xdc` |
| `0x80758310` | 184 | R | ends in virtual tail call (1 indirect); prologue save-helper `0x802dcd74`; no psq |
| `0x807583D0` | 420 | R | prev W132; 5 direct calls; prologue save-helper `0x802dcd70`; 1 qr0 psq (float pair / f31 save) |
| `0x807586D0` | 400 | R | prev W144; ends in virtual tail call (5 indirect); no psq; field `+0x244` |
| `0x80759A20` | 36 | R | prev W132; `lwz r0,+0x380(r3); cmpwi r0,0; blr; li r0,+0x0; stw r0,+0x380(r3); … (8 insns)` |
| `0x8075ADA0` | 184 | R | 6 direct calls; no psq; field `+0x60` |
| `0x8075AEF0` | 104 | R | ends in virtual tail call (1 indirect); no psq; field `+0x128` |
| `0x8075B040` | 440 | R | prev W144; ends in virtual tail call (3 indirect); no psq; field `+0x704`; string `s_cobAntlion_80814400` |
| `0x8075B2B0` | 336 | R | prev W144; ends in virtual tail call (3 indirect); no psq; field `+0x744` |
| `0x8075B8E0` | 60 | R | `stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); stw r31,+0xc(r1); or r31,r3,r3; … (8 insns)` |
| `0x8075BB60` | 448 | R | prev W144; 5 direct calls; prologue save-helper `0x802dcd60`; no psq |
| `0x8075BD20` | 368 | R | prev W144; 9 direct calls; prologue save-helper `0x802dcd54`; no psq |
| `0x8075BE90` | 212 | R | 5 direct calls; prologue save-helper `0x802dcd64`; no psq |
| `0x8075BF70` | 160 | R | 4 direct calls; prologue save-helper `0x802dcd70`; no psq |
| `0x8075C010` | 96 | R | 1 direct call; no psq |
| `0x8075C070` | 336 | R | prev W144; 2 direct calls; no psq; field `+0x1ac` |
| `0x8075C1C0` | 104 | R | 2 direct calls; no psq |
| `0x8075C320` | 144 | R | 4 direct calls; no psq; field `+0x1b8` |
| `0x8075C3B0` | 216 | R | ends in virtual tail call (1 indirect); no psq; field `+0x1ec` |
| `0x8075C550` | 340 | R | prev W144; ends in virtual tail call (4 indirect); no psq; field `+0x1b8`; string `s_cobKoopaShip_808145e8` |
| `0x8075F9E0` | 84 | R | prev W106; 2 direct calls; no psq |
| `0x8075FA40` | 240 | R | prev W106; ends in virtual tail call (1 indirect); 2 qr0 psq (float pair / f31 save); string `s_cobCastle_80814c90` |

#### `0x8076xxxx` (59)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x80761670` | 12 | R | prev W114; `li r4,+0x0; addi r3,r3,+0x20c; b 0x801662E0` — export truncated, raw body complete |
| `0x80762010` | 404 | R | prev W114; 7 direct calls; 1 qr0 psq (float pair / f31 save); field `+0x22c` |
| `0x80762970` | 420 | R | ends in virtual tail call (2 indirect); prologue save-helper `0x802dcd6c`; no psq; string `s_cobDokanRoute_80815550` |
| `0x80762BF0` | 80 | R | prev W114; 3 direct calls; no psq; field `+0xdc` |
| `0x80762EA0` | 312 | R | 8 direct calls; 1 qr0 psq (float pair / f31 save); field `+0x1ec` |
| `0x80762FE0` | 212 | R | 5 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x807630C0` | 456 | R | ends in virtual tail call (2 indirect); no psq; field `+0x25c` |
| `0x80763740` | 188 | R | 6 direct calls; no psq; field `+0x184` |
| `0x80763A30` | 308 | R | 12 direct calls; prologue save-helper `0x802dcd68`; no psq; string `s_cobGhost_80815760` |
| `0x80763B70` | 176 | R | 5 direct calls; no psq; field `+0x7c` |
| `0x80764350` | 76 | R | prev W114; 3 direct calls; no psq; field `+0x60` |
| `0x80764550` | 292 | R | 9 direct calls; no psq; field `+0x3cc` |
| `0x80764690` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x807646D0` | 284 | R | 11 direct calls; no psq; field `+0x3cc` |
| `0x80764910` | 132 | R | ends in virtual tail call (2 indirect); no psq; field `+0x38c` |
| `0x80764C00` | 120 | R | ends in virtual tail call (2 indirect); no psq; field `+0x38c` |
| `0x80764C80` | 380 | R | 7 direct calls; prologue save-helper `0x802dcd70`; 1 qr0 psq (float pair / f31 save) |
| `0x80764E00` | 252 | R | 3 direct calls; no psq; field `+0x102` |
| `0x80764F00` | 152 | R | 3 direct calls; no psq; field `+0x47c` |
| `0x80765180` | 12 | R | prev W147; `li r0,+0x1; stw r0,+0x47c(r3); b 0x800F6CB0` — export truncated, raw body complete |
| `0x807658D0` | 144 | R | 1 direct call; 4 qr0 psq (float pair / f31 save) |
| `0x80765980` | 272 | R | ends in virtual tail call (1 indirect); prologue save-helper `0x802dcd6c`; no psq; string `s_character_SV_80815a38` |
| `0x80765A90` | 268 | R | 7 direct calls; prologue save-helper `0x802dcd70`; 2 qr0 psq (float pair / f31 save) |
| `0x80768470` | 276 | R | 3 direct calls; 1 qr0 psq (float pair / f31 save); field `+0x204` |
| `0x8076B560` | 100 | R | 3 direct calls; no psq; field `+0x128` |
| `0x8076B9B0` | 176 | R | 5 direct calls; no psq; field `+0x7c` |
| `0x8076BEC0` | 60 | R | `stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); stw r31,+0xc(r1); or r31,r3,r3; … (8 insns)` |
| `0x8076C090` | 68 | R | 1 direct call; no psq; field `+0x60` |
| `0x8076C270` | 60 | R | `stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); stw r31,+0xc(r1); or r31,r3,r3; … (8 insns)` |
| `0x8076C360` | 120 | R | 3 direct calls; no psq; field `+0x128` |
| `0x8076C4E0` | 160 | R | 6 direct calls; no psq; field `+0x188`; string `s_character_SV_808168cc` |
| `0x8076C580` | 160 | R | 3 direct calls; no psq; field `+0x188` |
| `0x8076C620` | 144 | R | ends in virtual tail call (1 indirect); no psq; field `+0xac` |
| `0x8076D140` | 176 | R | ends in virtual tail call (1 indirect); 1 qr0 psq (float pair / f31 save); field `+0x1ac` |
| `0x8076D1F0` | 132 | R | 2 direct calls; no psq; field `+0x1b4` |
| `0x8076D280` | 76 | R | 2 direct calls; no psq |
| `0x8076D3B0` | 116 | R | 6 direct calls; no psq; field `+0x60` |
| `0x8076D620` | 76 | R | ends in virtual tail call (1 indirect); no psq; field `+0x1a8` |
| `0x8076D680` | 368 | R | 14 direct calls; prologue save-helper `0x802dcd68`; no psq; string `s_koopaJr_80816a80` |
| `0x8076D7F0` | 176 | R | 5 direct calls; no psq; field `+0x7c` |
| `0x8076D8E0` | 52 | R | `stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); stw r31,+0xc(r1); or r31,r3,r3; … (8 insns)` |
| `0x8076E490` | 228 | R | 5 direct calls; no psq |
| `0x8076E660` | 84 | R | 1 direct call; no psq |
| `0x8076E6C0` | 160 | R | 8 direct calls; no psq; field `+0x448` |
| `0x8076E760` | 252 | R | ends in virtual tail call (1 indirect); no psq; field `+0x448` |
| `0x8076EAA0` | 352 | R | ends in virtual tail call (4 indirect); no psq; field `+0x39c` |
| `0x8076EC00` | 72 | R | ends in virtual tail call (1 indirect); no psq; field `+0x1a0` |
| `0x8076ECF0` | 368 | R | 16 direct calls; prologue save-helper `0x802dcd68`; no psq; string `s_cobKoopaShip_80816d68` |
| `0x8076EE60` | 288 | R | 7 direct calls; no psq; field `+0x4b4` |
| `0x8076EF80` | 2544 | R | prev W141; ends in virtual tail call (1 indirect); prologue save-helper `0x802dcd70`; no psq |
| `0x8076F970` | 388 | R | ends in virtual tail call (2 indirect); no psq; field `+0x498` |
| `0x8076FB00` | 152 | R | 2 direct calls; no psq; field `+0x102` |
| `0x8076FBA0` | 188 | R | 2 direct calls; no psq; field `+0xac` |
| `0x8076FC60` | 312 | R | 6 direct calls; no psq; field `+0x470`; string `s_shipRoute_80816d44` |
| `0x8076FDA0` | 156 | R | 6 direct calls; prologue save-helper `0x802dcd74`; no psq; string `s_shipRoute_80816d44` |
| `0x8076FE70` | 116 | R | 1 direct call; no psq; field `+0x474` |
| `0x8076FEF0` | 24 | R | prev W147; `lis r4,0x8081; lis r6,0x8043; or r5,r3,r3; lwz r3,-0x5e74(r6); lwz r4,+0x6d54(r4); b 0x801006E0` — export truncated, raw body complete |
| `0x8076FF10` | 104 | R | 2 direct calls; no psq; string `s_shipRoute_80816d44` |
| `0x8076FF80` | 364 | R | prev W114; 9 direct calls; prologue save-helper `0x802dcd70`; no psq; string `s_cobKoopaShip_80816ba0` |

#### `0x8077xxxx` (51)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x80770BE0` | 112 | R | Ghidra truncated at the tail `b`; raw body complete; leaf, no calls; no psq; field `+0xac` |
| `0x80772BD0` | 348 | R | prev W129; ends in virtual tail call (2 indirect); prologue save-helper `0x802dcd68`; no psq |
| `0x80773200` | 4 | R | prev W142; `b 0x80773210` |
| `0x80773210` | 2096 | R | prev W129; ends in virtual tail call (10 indirect); 1 qr0 psq (float pair / f31 save) |
| `0x80773D70` | 376 | R | prev W129; 11 direct calls; prologue save-helper `0x802dcd60`; no psq |
| `0x80773EF0` | 408 | R | 10 direct calls; prologue save-helper `0x802dcd64`; 2 qr0 psq (float pair / f31 save) |
| `0x807740D0` | 1656 | R | prev W129; 25 direct calls; prologue save-helper `0x802dcd58`; no psq |
| `0x80774880` | 468 | R | 8 direct calls; prologue save-helper `0x802dcd70`; no psq; string `s_cobKinokoRed_807cf3c0` |
| `0x80774A60` | 356 | R | prev W129; 7 direct calls; prologue save-helper `0x802dcd70`; no psq; string `s_cobKiller_807cf418` |
| `0x80774BD0` | 296 | R | prev W129; 6 direct calls; prologue save-helper `0x802dcd70`; no psq |
| `0x80774D00` | 236 | R | 5 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x80774DF0` | 420 | R | 10 direct calls; prologue save-helper `0x802dcd6c`; no psq |
| `0x80774FA0` | 132 | R | 2 direct calls; no psq |
| `0x80775080` | 128 | R | 1 direct call; no psq |
| `0x807754D0` | 108 | R | 2 direct calls; no psq |
| `0x80775730` | 196 | R | prev W129; 5 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x80775800` | 92 | R | 1 direct call; no psq |
| `0x80775860` | 100 | R | 2 direct calls; no psq |
| `0x80775BB0` | 336 | R | prev W129; 7 direct calls; prologue save-helper `0x802dcd68`; no psq |
| `0x807762B0` | 172 | R | 6 direct calls; no psq; field `+0x184`; string `s_g3d_model_brres_808173b0` |
| `0x80776390` | 176 | R | 5 direct calls; no psq; field `+0x7c` |
| `0x80776750` | 116 | R | 6 direct calls; no psq; field `+0x60` |
| `0x80776C00` | 4 | R | prev W142; `b 0x80776C10` |
| `0x80776C80` | 136 | R | prev W115; 4 direct calls; no psq |
| `0x80777990` | 96 | R | 2 direct calls; no psq; field `+0x18` |
| `0x807779F0` | 100 | R | 2 direct calls; no psq; field `+0x18` |
| `0x80777BA0` | 140 | R | 2 direct calls; no psq; field `+0xac` |
| `0x80777D20` | 176 | R | 5 direct calls; no psq; field `+0x7c` |
| `0x80778100` | 164 | R | 4 direct calls; no psq; field `+0x1ec`; string `s_cobSandpillar_80817830` |
| `0x80778200` | 156 | R | 4 direct calls; no psq; field `+0x1ec`; string `s_cobSandpillar_80817830` |
| `0x80778640` | 84 | R | 1 direct call; no psq; field `+0x1ec` |
| `0x807786E0` | 100 | R | 1 direct call; no psq; field `+0x1ec` |
| `0x80778770` | 104 | R | 1 direct call; no psq; field `+0x1ec` |
| `0x807787E0` | 144 | R | ends in virtual tail call (1 indirect); 1 qr0 psq (float pair / f31 save); field `+0xe0` |
| `0x80778880` | 148 | R | 4 direct calls; no psq; field `+0x1ec`; string `s_cobSandpillar_80817830` |
| `0x807789A0` | 176 | R | ends in virtual tail call (2 indirect); no psq; field `+0x1ac`; string `s_ef_cobSandpillar_80817848` |
| `0x807794A0` | 84 | R | 3 direct calls; no psq; field `+0x184` |
| `0x80779770` | 176 | R | 5 direct calls; no psq; field `+0x7c` |
| `0x80779900` | 124 | R | 6 direct calls; no psq; field `+0x184` |
| `0x80779CA0` | 404 | R | ends in virtual tail call (1 indirect); prologue save-helper `0x802dcd6c`; no psq; string `s_CS_W_d_80817ef0` |
| `0x80779E40` | 176 | R | 5 direct calls; no psq; field `+0x7c` |
| `0x80779EF0` | 160 | R | 6 direct calls; no psq; field `+0x1b8` |
| `0x8077A000` | 40 | R | prev W142; `lwz r0,+0x4(r3); or r5,r3,r3; lis r4,0x8081; lis r3,0x8043; addi r4,r4,+0x7f38; … (8 insns)` — export truncated, raw body complete |
| `0x8077A110` | 148 | R | 4 direct calls; no psq; field `+0x1b8` |
| `0x8077AB80` | 176 | R | 5 direct calls; no psq; field `+0x7c` |
| `0x8077B110` | 76 | R | 3 direct calls; no psq; field `+0x60` |
| `0x8077B2B0` | 172 | R | 6 direct calls; no psq; field `+0x138`; string `s_cobStop_80818150` |
| `0x8077B360` | 176 | R | 5 direct calls; no psq; field `+0x7c` |
| `0x8077B450` | 4076 | R | prev W129; ends in virtual tail call (29 indirect); prologue save-helper `0x802dcd68`; no psq |
| `0x8077CED0` | 144 | R | prev W100; 5 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x8077E410` | 344 | R | prev W100; ends in virtual tail call (2 indirect); 3 qr0 psq (float pair / f31 save) |

#### `0x8078xxxx` (58)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x807812D0` | 4 | R | prev W115; `b 0x80100940` — export truncated, raw body complete |
| `0x80781C80` | 16 | R | prev W115; `lis r4,0x807d; li r5,+0x6; addi r4,r4,-0x9f8; b 0x800F6D60` — export truncated, raw body complete |
| `0x80781C90` | 32 | R | prev W115; `lwz r5,+0x1cc(r3); lis r3,0x8043; lwz r3,-0x5d84(r3); li r4,+0x9; lwz r6,+0x4(r5); … (8 insns)` — export truncated, raw body complete |
| `0x807826B0` | 516 | R | ends in virtual tail call (3 indirect); no psq; field `+0x1a4`; string `s_cobHatenaSwitch_80818650` |
| `0x80782970` | 204 | R | ends in virtual tail call (1 indirect); no psq; field `+0x214` |
| `0x80782B20` | 220 | R | 7 direct calls; no psq; field `+0x60` |
| `0x80782C40` | 284 | R | ends in virtual tail call (1 indirect); no psq; field `+0x434` |
| `0x80782ED0` | 120 | R | ends in virtual tail call (3 indirect); no psq; field `+0x428` |
| `0x80783300` | 252 | R | 7 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x80783590` | 32 | R | `lis r6,0x807d; li r0,+0x2; stw r0,+0x424(r3); li r4,+0x0; lwz r3,+0x434(r3); … (8 insns)` — export truncated, raw body complete |
| `0x807837F0` | 160 | R | 3 direct calls; no psq; field `+0x10c` |
| `0x80783930` | 88 | R | leaf, no calls; no psq |
| `0x80786180` | 264 | R | 1 direct call; no psq; field `+0x1c8` |
| `0x807882E0` | 28 | R | prev W115; `lis r4,0x8082; lis r5,0x8082; addi r4,r4,-0x6ddc; li r7,+0x1; or r6,r4,r4; … (7 insns)` — export truncated, raw body complete |
| `0x80788AE0` | 108 | R | 4 direct calls; no psq; string `s_Koopa_80818c78` |
| `0x80788B50` | 348 | R | 4 direct calls; 2 qr0 psq (float pair / f31 save) |
| `0x80789790` | 508 | R | 9 direct calls; no psq; field `+0x35c` |
| `0x80789990` | 208 | R | leaf, no calls; no psq; field `+0xd8` |
| `0x80789A80` | 136 | R | 8 direct calls; no psq |
| `0x80789D90` | 200 | R | 1 direct call; no psq |
| `0x8078A270` | 16 | R | prev W115; `lis r4,0x807d; li r5,+0x3; addi r4,r4,-0x510; b 0x800F6D60` — export truncated, raw body complete |
| `0x8078A3A0` | 48 | R | prev W115; `cmpwi r3,0; or r5,r4,r4; bne 0x8078A3BC; li r3,+0x289; li r5,+0x0; … (8 insns)` — export truncated, raw body complete |
| `0x8078A740` | 48 | R | prev W115; `cmpwi r3,0; or r5,r4,r4; bne 0x8078A75C; li r3,+0x28d; li r5,+0x0; … (8 insns)` — export truncated, raw body complete |
| `0x8078AF50` | 464 | R | ends in virtual tail call (6 indirect); no psq; field `+0x444` |
| `0x8078B5B0` | 48 | R | prev W115; `cmpwi r3,0; or r5,r4,r4; bne 0x8078B5CC; li r3,+0x286; li r5,+0x0; … (8 insns)` — export truncated, raw body complete |
| `0x8078B7D0` | 48 | R | prev W115; `cmpwi r3,0; or r5,r4,r4; bne 0x8078B7EC; li r3,+0x28a; li r5,+0x0; … (8 insns)` — export truncated, raw body complete |
| `0x8078B9F0` | 48 | R | prev W115; `cmpwi r3,0; or r5,r4,r4; bne 0x8078BA0C; li r3,+0x28c; li r5,+0x0; … (8 insns)` — export truncated, raw body complete |
| `0x8078BC10` | 48 | R | prev W115; `cmpwi r3,0; or r5,r4,r4; bne 0x8078BC2C; li r3,+0x28b; li r5,+0x0; … (8 insns)` — export truncated, raw body complete |
| `0x8078BE30` | 48 | R | prev W115; `cmpwi r3,0; or r5,r4,r4; bne 0x8078BE4C; li r3,+0x287; li r5,+0x0; … (8 insns)` — export truncated, raw body complete |
| `0x8078C060` | 48 | R | prev W115; `cmpwi r3,0; or r5,r4,r4; bne 0x8078C07C; li r3,+0x288; li r5,+0x0; … (8 insns)` — export truncated, raw body complete |
| `0x8078C500` | 424 | R | ends in virtual tail call (2 indirect); prologue save-helper `0x802dcd6c`; no psq; string `s_g3d_bros_brres_8081a264` |
| `0x8078CD10` | 128 | R | 1 direct call; no psq; field `+0x6bc` |
| `0x8078CFA0` | 116 | R | leaf, no calls; no psq |
| `0x8078D020` | 128 | R | 3 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x8078D0A0` | 60 | R | `stwu r1,-0x10(r1); mfspr r0,lr; lis r4,0x8043; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x8078D0E0` | 168 | R | 4 direct calls; no psq |
| `0x8078D290` | 124 | R | 4 direct calls; no psq; field `+0x1b8` |
| `0x8078D560` | 428 | R | ends in virtual tail call (1 indirect); no psq; field `+0x18c`; string `s_CS_W_d_8081a3f0` |
| `0x8078D710` | 176 | R | 5 direct calls; no psq; field `+0x7c` |
| `0x8078D7C0` | 76 | R | 2 direct calls; no psq; field `+0x1e0` |
| `0x8078D8F0` | 76 | R | 2 direct calls; no psq; field `+0x60` |
| `0x8078DAC0` | 164 | R | ends in virtual tail call (3 indirect); no psq; field `+0x1c0` |
| `0x8078DB80` | 240 | R | ends in virtual tail call (1 indirect); no psq; field `+0x188`; string `s_daWmDirector__m_allocator_8081a564` |
| `0x8078DC70` | 2576 | R | prev W115; ends in virtual tail call (19 indirect); prologue save-helper `0x802dcd70`; no psq |
| `0x8078E900` | 12 | R | `lis r3,0x8043; lwz r3,-0x5b58(r3); b 0x8019BC30` — export truncated, raw body complete |
| `0x8078E9A0` | 404 | R | prev W115; ends in virtual tail call (1 indirect); no psq; field `+0x1c8` |
| `0x8078EB40` | 376 | R | prev W115; 4 direct calls; no psq |
| `0x8078ED10` | 224 | R | 3 direct calls; no psq; field `+0x1b4` |
| `0x8078EDF0` | 240 | R | 4 direct calls; no psq |
| `0x8078EEE0` | 372 | R | 7 direct calls; no psq; string `s_cobKinoko1up_8081a824` |
| `0x8078F060` | 124 | R | 1 direct call; no psq; field `+0x1d0` |
| `0x8078F0E0` | 184 | R | 2 direct calls; no psq; field `+0x1b4` |
| `0x8078F2D0` | 116 | R | ends in virtual tail call (1 indirect); no psq; field `+0x1b4` |
| `0x8078F350` | 196 | R | ends in virtual tail call (2 indirect); no psq; field `+0x1b4` |
| `0x8078F500` | 172 | R | 5 direct calls; no psq; field `+0x1b0` |
| `0x8078FAD0` | 176 | R | 5 direct calls; no psq; field `+0x7c` |
| `0x8078FB80` | 256 | R | 7 direct calls; no psq; field `+0x1b0` |
| `0x8078FF30` | 384 | R | prev W115; 5 direct calls; no psq; field `+0x214` |

#### `0x8079xxxx` (54)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x80790280` | 268 | R | prev W107; 6 direct calls; prologue save-helper `0x802dcd74`; 1 qr0 psq (float pair / f31 save) |
| `0x80792EE0` | 4 | R | `b 0x80162590` — export truncated, raw body complete |
| `0x807946D0` | 452 | R | ends in virtual tail call (1 indirect); 2 qr0 psq (float pair / f31 save); field `+0x1ec` |
| `0x80795AA0` | 412 | R | ends in virtual tail call (1 indirect); 1 qr0 psq (float pair / f31 save); field `+0x1d0` |
| `0x807962B0` | 1428 | R | prev W143; 27 direct calls; prologue save-helper `0x802dcd6c`; 1 qr0 psq (float pair / f31 save) |
| `0x80796850` | 448 | R | 10 direct calls; no psq; field `+0xb0` |
| `0x80796CF0` | 196 | R | leaf, no calls; no psq; field `+0x288` |
| `0x807970A0` | 128 | R | prev W148; leaf, no calls; no psq; field `+0x18c` |
| `0x80797190` | 264 | R | 3 direct calls; no psq; field `+0x240` |
| `0x80797810` | 328 | R | 4 direct calls; no psq; field `+0x2a4` |
| `0x807980F0` | 420 | R | 8 direct calls; prologue save-helper `0x802dcd5c`; 2 qr0 psq (float pair / f31 save) |
| `0x807985C0` | 232 | R | 7 direct calls; no psq; field `+0x2a0` |
| `0x80798E20` | 236 | R | 1 direct call; no psq; field `+0x10c` |
| `0x80799200` | 312 | R | 6 direct calls; no psq; field `+0x230` |
| `0x80799960` | 108 | R | prev W148; ends in virtual tail call (1 indirect); no psq; field `+0x250` |
| `0x80799B00` | 288 | R | prev W107; 7 direct calls; prologue save-helper `0x802dcd70`; no psq |
| `0x80799D70` | 304 | R | prev W107; 2 direct calls; 1 qr0 psq (float pair / f31 save); field `+0x230` |
| `0x8079AC60` | 408 | R | 9 direct calls; prologue save-helper `0x802dcd5c`; no psq |
| `0x8079AE00` | 352 | R | 7 direct calls; prologue save-helper `0x802dcd64`; no psq |
| `0x8079AFB0` | 172 | R | 3 direct calls; no psq; field `+0x22c` |
| `0x8079B110` | 16 | R | prev W107; `lis r4,0x807d; li r5,+0x6; addi r4,r4,+0xa0; b 0x800F6D60` — export truncated, raw body complete |
| `0x8079B1C0` | 12 | R | prev W107; `lwz r3,+0x1d0(r3); addi r3,r3,+0x28; b 0x80169020` — export truncated, raw body complete |
| `0x8079B1D0` | 112 | R | 1 direct call; no psq; field `+0x27c` |
| `0x8079B260` | 152 | R | 4 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x8079B300` | 240 | R | 5 direct calls; no psq |
| `0x8079B3F0` | 440 | R | ends in virtual tail call (1 indirect); no psq; field `+0x23c` |
| `0x8079B5B0` | 172 | R | 4 direct calls; no psq; field `+0x22c` |
| `0x8079B660` | 160 | R | 1 direct call; no psq; field `+0x22c` |
| `0x8079B720` | 160 | R | 2 direct calls; no psq; field `+0x22c` |
| `0x8079BA00` | 192 | R | ends in virtual tail call (1 indirect); 1 qr0 psq (float pair / f31 save); field `+0x230` |
| `0x8079BB60` | 168 | R | 2 direct calls; no psq |
| `0x8079BC10` | 196 | R | 4 direct calls; no psq |
| `0x8079BD40` | 12 | R | prev W148; `lis r3,0x8043; lwz r3,-0x5e60(r3); b 0x8079B1C0` |
| `0x8079BD50` | 148 | R | leaf, no calls; no psq |
| `0x8079BE80` | 120 | R | 2 direct calls; no psq; field `+0x29c` |
| `0x8079C150` | 224 | R | 3 direct calls; no psq; field `+0x1d0` |
| `0x8079C230` | 144 | R | 1 direct call; no psq; field `+0x2e0` |
| `0x8079C330` | 4232 | R | prev W135; ends in virtual tail call (13 indirect); prologue save-helper `0x802dcd6c`; no psq |
| `0x8079D3C0` | 3176 | R | prev W135; ends in virtual tail call (35 indirect); no psq; field `+0x60` |
| `0x8079E030` | 432 | R | 7 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x8079E1E0` | 296 | R | 6 direct calls; no psq; field `+0x1d0` |
| `0x8079E310` | 452 | R | 7 direct calls; 4 qr0 psq (float pair / f31 save); field `+0xac` |
| `0x8079E550` | 292 | R | 6 direct calls; no psq; field `+0x1fc` |
| `0x8079E680` | 300 | R | ends in virtual tail call (2 indirect); no psq; field `+0x2c0` |
| `0x8079EE70` | 140 | R | prev W100; 3 direct calls; no psq |
| `0x8079EF20` | 288 | R | 8 direct calls; no psq; field `+0x1d0` |
| `0x8079F040` | 512 | R | 10 direct calls; no psq; field `+0x2f4` |
| `0x8079F390` | 284 | R | 5 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x8079F4B0` | 68 | R | prev W135; Ghidra truncated at the tail `b`; raw body complete; leaf, no calls; no psq |
| `0x8079F500` | 232 | R | 6 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x8079F5F0` | 416 | R | 9 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x8079F790` | 360 | R | 8 direct calls; prologue save-helper `0x802dcd54`; no psq |
| `0x8079F900` | 260 | R | 9 direct calls; prologue save-helper `0x802dcd74`; no psq; field `+0x22c` |
| `0x8079FA10` | 212 | R | 4 direct calls; prologue save-helper `0x802dcd74`; no psq |

#### `0x807Axxxx` (50)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x807A02C0` | 428 | R | prev W127; 4 direct calls; no psq; field `+0xac` |
| `0x807A28B0` | 440 | R | ends in virtual tail call (3 indirect); 2 qr0 psq (float pair / f31 save); field `+0xe0`; string `s_Wm_ob_firespillar01_8081c630` |
| `0x807A2CA0` | 500 | R | prev W127; ends in virtual tail call (4 indirect); 2 qr0 psq (float pair / f31 save); field `+0xfc`; string `s_Wm_ob_firespillar01_8081c630` |
| `0x807A3D00` | 444 | R | prev W127; ends in virtual tail call (1 indirect); prologue save-helper `0x802dcd74`; 1 qr0 psq (float pair / f31 save); string `s_I_yoshi_egg_8081c6ac` |
| `0x807A4450` | 216 | R | prev W127; 2 direct calls; 1 qr0 psq (float pair / f31 save); field `+0x348` |
| `0x807A57F0` | 56 | R | `lwz r0,+0x408(r3); cmpwi r0,0; blr; lis r3,0x8043; lwz r3,-0x60b0(r3); … (8 insns)` — export truncated, raw body complete |
| `0x807A58E0` | 288 | R | prev W127; 1 direct call; 1 qr0 psq (float pair / f31 save); field `+0x3f0` |
| `0x807A5CC0` | 128 | R | prev W127; ends in virtual tail call (1 indirect); no psq; field `+0x3fc` |
| `0x807A7C40` | 164 | R | 2 direct calls; no psq; field `+0x1c0`; string `s_cameraA_8081d358` |
| `0x807AB370` | 44 | R | `stwu r1,-0x10(r1); mfspr r0,lr; li r3,+0x0; stw r0,+0x14(r1); bl 0x800B0DB0; … (8 insns)` |
| `0x807AB3A0` | 152 | R | ends in virtual tail call (3 indirect); no psq; field `+0x74` |
| `0x807AB590` | 252 | R | prev W127; ends in virtual tail call (2 indirect); prologue save-helper `0x802dcd70`; no psq |
| `0x807AB900` | 112 | R | 3 direct calls; no psq |
| `0x807AB970` | 84 | R | 4 direct calls; no psq; field `+0xe8` |
| `0x807AB9D0` | 132 | R | ends in virtual tail call (1 indirect); no psq; field `+0x74` |
| `0x807ABA60` | 56 | R | `stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); bl 0x80109340; lwz r0,+0x1d8(r3); … (8 insns)` |
| `0x807ABAB0` | 92 | R | ends in virtual tail call (1 indirect); no psq; field `+0x74` |
| `0x807ABB10` | 116 | R | 4 direct calls; no psq |
| `0x807ABD60` | 188 | R | 8 direct calls; prologue save-helper `0x802dcd6c`; no psq; string `s_Env_world_8081db10` |
| `0x807ABE20` | 188 | R | 8 direct calls; prologue save-helper `0x802dcd74`; no psq; string `s_Env_world_8081db10` |
| `0x807ABEE0` | 84 | R | 2 direct calls; no psq |
| `0x807ABF40` | 8 | R | prev W127; `or r3,r4,r4; b 0x801648B0` — export truncated, raw body complete |
| `0x807ABF50` | 460 | R | prev W127; ends in virtual tail call (2 indirect); 2 qr0 psq (float pair / f31 save); field `+0x10c` |
| `0x807AC260` | 16 | R | prev W127; `lwz r12,+0x0(r3); lwz r12,+0x8(r12); mtctr r12; bctr` |
| `0x807AC270` | 16 | R | prev W127; `lwz r12,+0x0(r3); lwz r12,+0x8(r12); mtctr r12; bctr` |
| `0x807AC280` | 16 | R | prev W127; `lwz r12,+0x0(r3); lwz r12,+0x8(r12); mtctr r12; bctr` |
| `0x807AD210` | 100 | R | prev W127; 2 direct calls; no psq; field `+0x18` |
| `0x807AD590` | 300 | R | prev W127; ends in virtual tail call (1 indirect); prologue save-helper `0x802dcd68`; no psq |
| `0x807ADA10` | 188 | R | 8 direct calls; prologue save-helper `0x802dcd6c`; no psq; string `s_Env_world_8081e0a4` |
| `0x807ADAD0` | 188 | R | 8 direct calls; prologue save-helper `0x802dcd74`; no psq; string `s_Env_world_8081e0a4` |
| `0x807ADB90` | 84 | R | 2 direct calls; no psq |
| `0x807ADBF0` | 8 | R | `or r3,r4,r4; b 0x801648B0` — export truncated, raw body complete |
| `0x807ADC00` | 460 | R | ends in virtual tail call (2 indirect); 2 qr0 psq (float pair / f31 save); field `+0xd8` |
| `0x807AE400` | 164 | R | prev W127; 17 direct calls; no psq |
| `0x807AE6A0` | 40 | R | `stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); bl 0x80162A20; cntlzw r0,r3; … (8 insns)` |
| `0x807AE8A0` | 64 | R | prev W127; `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x807AE8E0` | 96 | R | prev W127; 2 direct calls; no psq; field `+0x18` |
| `0x807AE940` | 100 | R | prev W127; 2 direct calls; no psq; field `+0x18` |
| `0x807AECB0` | 188 | R | 8 direct calls; prologue save-helper `0x802dcd6c`; no psq; string `s_Env_world_8081e400` |
| `0x807AED70` | 188 | R | 8 direct calls; prologue save-helper `0x802dcd74`; no psq; string `s_Env_world_8081e400` |
| `0x807AEE30` | 84 | R | 2 direct calls; no psq |
| `0x807AEE90` | 8 | R | `or r3,r4,r4; b 0x801648B0` — export truncated, raw body complete |
| `0x807AEEA0` | 460 | R | prev W127; ends in virtual tail call (2 indirect); 2 qr0 psq (float pair / f31 save); field `+0xd4` |
| `0x807AF520` | 204 | R | prev W127; 17 direct calls; no psq |
| `0x807AF940` | 40 | R | `stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); bl 0x80162A20; cntlzw r0,r3; … (8 insns)` |
| `0x807AFB30` | 64 | R | prev W127; `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x807AFB70` | 96 | R | 2 direct calls; no psq; field `+0x18` |
| `0x807AFBD0` | 100 | R | 2 direct calls; no psq; field `+0x18` |
| `0x807AFEF0` | 108 | R | ends in virtual tail call (2 indirect); no psq; field `+0x74` |
| `0x807AFF90` | 152 | R | prev W127; ends in virtual tail call (2 indirect); no psq; field `+0x74` |

#### `0x807Bxxxx` (55)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x807B00C0` | 320 | R | prev W133; ends in virtual tail call (3 indirect); no psq; field `+0xc0` |
| `0x807B06C0` | 188 | R | prev W133; 8 direct calls; prologue save-helper `0x802dcd6c`; no psq; string `s_Env_world_8081e6ec` |
| `0x807B0780` | 188 | R | prev W133; 8 direct calls; prologue save-helper `0x802dcd74`; no psq; string `s_Env_world_8081e6ec` |
| `0x807B08A0` | 8 | R | prev W103; `or r3,r4,r4; b 0x801648B0` — export truncated, raw body complete |
| `0x807B5C50` | 8 | R | prev W103; `or r3,r4,r4; b 0x801648B0` — export truncated, raw body complete |
| `0x807B5C60` | 188 | R | 9 direct calls; prologue save-helper `0x802dcd74`; no psq; string `s_Env_movie_8081fbe0` |
| `0x807B5D20` | 268 | R | 9 direct calls; prologue save-helper `0x802dcd74`; no psq; string `s_fog_demomovie_bfog_8081fbf8` |
| `0x807B73D0` | 284 | R | 8 direct calls; no psq; field `+0x78` |
| `0x807B7540` | 500 | R | prev W143; 3 direct calls; no psq; field `+0x120e` |
| `0x807B7B40` | 524 | R | prev W143; 10 direct calls; prologue save-helper `0x802dcd6c`; no psq |
| `0x807B7D60` | 456 | R | prev W143; 2 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x807B7F30` | 396 | R | 10 direct calls; no psq |
| `0x807B8A90` | 244 | R | prev W152; 1 direct call; no psq; field `+0x120c` |
| `0x807B8B90` | 192 | R | 4 direct calls; no psq |
| `0x807B8CB0` | 516 | R | prev W143; 13 direct calls; prologue save-helper `0x802dcd6c`; no psq; string `s_light_Toride2Scene_blight_807d0880` |
| `0x807B8EC0` | 8 | R | prev W103; `or r3,r4,r4; b 0x801648B0` — export truncated, raw body complete |
| `0x807B90F0` | 452 | R | prev W143; 23 direct calls; no psq |
| `0x807B9F40` | 392 | R | prev W103; 15 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x807BA440` | 204 | R | prev W152; 5 direct calls; no psq; string `s_light_W_X_blight_80820af0` |
| `0x807BA510` | 188 | R | prev W103; 8 direct calls; prologue save-helper `0x802dcd6c`; no psq; string `s_Env_world_80820b18` |
| `0x807BA660` | 216 | R | 9 direct calls; prologue save-helper `0x802dcd74`; no psq; string `s_Env_world_80820b18` |
| `0x807BA740` | 8 | R | prev W103; `or r3,r4,r4; b 0x801648B0` — export truncated, raw body complete |
| `0x807BA750` | 192 | R | 4 direct calls; no psq |
| `0x807BA860` | 184 | R | 1 direct call; no psq |
| `0x807BA920` | 260 | R | 5 direct calls; no psq |
| `0x807BAA30` | 232 | R | 6 direct calls; no psq; string `s_Stage_80820b88` |
| `0x807BAB20` | 280 | R | prev W152; 5 direct calls; no psq |
| `0x807BAE90` | 120 | R | leaf, no calls; no psq |
| `0x807BAF10` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; lis r3,0x8043; li r4,-0x1; stw r0,+0x14(r1); … (8 insns)` |
| `0x807BAF50` | 356 | R | 9 direct calls; prologue save-helper `0x802dcd5c`; no psq |
| `0x807BB710` | 304 | R | prev W103; leaf, no calls; no psq; field `+0x26c` |
| `0x807BB870` | 84 | R | prev W103; leaf, no calls; no psq; field `+0x248` |
| `0x807BB8D0` | 276 | R | prev W103; 2 direct calls; no psq; field `+0x284` |
| `0x807BBA40` | 484 | R | prev W143; 3 direct calls; prologue save-helper `0x802dcd74`; 7 qr0 psq (float pair / f31 save) |
| `0x807BC220` | 156 | R | prev W103; ends in virtual tail call (1 indirect); 1 qr0 psq (float pair / f31 save); field `+0x27c` |
| `0x807BC7E0` | 484 | R | prev W103; ends in virtual tail call (1 indirect); no psq; field `+0x288` |
| `0x807BCC00` | 220 | R | prev W103; leaf, no calls; no psq; field `+0x27c` |
| `0x807BDF20` | 260 | R | ends in virtual tail call (1 indirect); no psq; field `+0x70`; string `s_worldSelectMapGuide_worldSelectM_80821440` |
| `0x807BE070` | 108 | R | ends in virtual tail call (1 indirect); no psq; field `+0x70` |
| `0x807BE130` | 36 | R | `lwz r4,+0x230(r3); lwz r0,+0x22c(r3); x31/0; blr; stw r4,+0x22c(r3); … (8 insns)` — export truncated, raw body complete |
| `0x807BE230` | 412 | R | prev W143; 1 direct call; 5 qr0 psq (float pair / f31 save) |
| `0x807BE3D0` | 184 | R | 1 direct call; 2 qr0 psq (float pair / f31 save) |
| `0x807BE4C0` | 428 | R | prev W143; 6 direct calls; no psq; field `+0x60` |
| `0x807BE6B0` | 160 | R | 6 direct calls; no psq; field `+0x6c8` |
| `0x807BEA40` | 176 | R | 6 direct calls; no psq; field `+0x4a8` |
| `0x807BEAF0` | 280 | R | 12 direct calls; no psq; field `+0x6c0` |
| `0x807BF5A0` | 260 | R | prev W103; 2 direct calls; 2 qr0 psq (float pair / f31 save); field `+0x6fc` |
| `0x807BF6B0` | 164 | R | prev W103; 4 direct calls; no psq; field `+0x608` |
| `0x807BF760` | 244 | R | prev W103; 4 direct calls; no psq; field `+0x608` |
| `0x807BF860` | 200 | R | 3 direct calls; no psq; field `+0x608` |
| `0x807BF930` | 488 | R | prev W143; 5 direct calls; no psq; field `+0x60c` |
| `0x807BFB20` | 184 | R | 1 direct call; no psq; field `+0x5f8` |
| `0x807BFC20` | 220 | R | prev W103; 4 direct calls; no psq; field `+0x608` |
| `0x807BFD00` | 276 | R | prev W103; 2 direct calls; no psq; field `+0x608` |
| `0x807BFE20` | 372 | R | prev W133; 6 direct calls; no psq; field `+0x60c` |

#### `0x807Cxxxx` (62)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x807C0330` | 200 | R | 2 direct calls; no psq; field `+0x658` |
| `0x807C0400` | 500 | R | 9 direct calls; no psq; field `+0x6a0` |
| `0x807C0600` | 416 | R | 3 direct calls; no psq; field `+0x6ac` |
| `0x807C07A0` | 260 | R | 1 direct call; no psq; field `+0x6a4` |
| `0x807C08B0` | 252 | R | leaf, no calls; no psq; field `+0x6a4` |
| `0x807C09B0` | 196 | R | 6 direct calls; prologue save-helper `0x802dcd70`; no psq |
| `0x807C0A80` | 152 | R | 1 direct call; no psq |
| `0x807C0B20` | 232 | R | 6 direct calls; prologue save-helper `0x802dcd74`; no psq |
| `0x807C0C10` | 276 | R | 1 direct call; 3 qr0 psq (float pair / f31 save); field `+0x4b8` |
| `0x807C0D30` | 384 | R | 7 direct calls; 2 qr0 psq (float pair / f31 save); field `+0x710` |
| `0x807C0EB0` | 288 | R | 5 direct calls; no psq; field `+0x6f8` |
| `0x807C0FD0` | 204 | R | 3 direct calls; no psq; field `+0x6f8` |
| `0x807C10A0` | 132 | R | 1 direct call; no psq; field `+0x6f8` |
| `0x807C1130` | 96 | R | 1 direct call; no psq; field `+0x6ac` |
| `0x807C1190` | 100 | R | leaf, no calls; no psq; field `+0x660` |
| `0x807C1200` | 28 | R | `lis r4,0x807d; li r0,+0x0; lfs f0,+0xac8(r4); stw r0,+0x720(r3); stfs f0,+0x724(r3); … (7 insns)` |
| `0x807C1220` | 260 | R | 1 direct call; no psq; field `+0x724` |
| `0x807C1330` | 76 | R | leaf, no calls; no psq; field `+0x72c` |
| `0x807C1380` | 28 | R | `lis r4,0x807d; lfs f0,+0x724(r3); lfs f1,+0xa6c(r4); fcmpu cr0,f1,f0; x31/19; … (7 insns)` |
| `0x807C13A0` | 112 | R | 1 direct call; no psq |
| `0x807C1410` | 96 | R | leaf, no calls; no psq |
| `0x807C1470` | 20 | R | `lwz r3,+0x6a0(r3); addi r0,r3,-0x3; cntlzw r0,r0; rlwinm r3,r0,27,5,31; blr` |
| `0x807C1AE0` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns)` |
| `0x807C1B20` | 96 | R | 2 direct calls; no psq; field `+0x18` |
| `0x807C1B80` | 100 | R | 2 direct calls; no psq; field `+0x18` |
| `0x807C1C80` | 196 | R | 1 direct call; no psq; field `+0x53c` |
| `0x807C1FE0` | 100 | R | ends in virtual tail call (1 indirect); no psq; field `+0x70` |
| `0x807C2070` | 92 | R | ends in virtual tail call (1 indirect); no psq; field `+0x70` |
| `0x807C21B0` | 220 | R | ends in virtual tail call (1 indirect); prologue save-helper `0x802dcd70`; no psq |
| `0x807C25B0` | 328 | R | ends in virtual tail call (5 indirect); no psq; field `+0x70` |
| `0x807C2710` | 88 | R | 3 direct calls; no psq; field `+0xb4` |
| `0x807C2810` | 60 | R | `stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); li r0,+0x0; stw r31,+0xc(r1); … (8 insns)` |
| `0x807C28A0` | 192 | R | ends in virtual tail call (1 indirect); no psq; field `+0xb0` |
| `0x807C2970` | 64 | R | `stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); stw r31,+0xc(r1); or r31,r3,r3; … (8 insns)` |
| `0x807C2A00` | 328 | R | ends in virtual tail call (3 indirect); no psq; field `+0x530` |
| `0x807C2B70` | 108 | R | ends in virtual tail call (1 indirect); no psq; field `+0x530` |
| `0x807C2C00` | 140 | R | ends in virtual tail call (2 indirect); no psq; field `+0x70` |
| `0x807C2CC0` | 148 | R | ends in virtual tail call (2 indirect); no psq; field `+0x70` |
| `0x807C2DB0` | 68 | R | 1 direct call; no psq; field `+0x530` |
| `0x807C2E50` | 328 | R | ends in virtual tail call (3 indirect); no psq; field `+0x530` |
| `0x807C3020` | 140 | R | ends in virtual tail call (2 indirect); no psq; field `+0x70` |
| `0x807C30E0` | 148 | R | ends in virtual tail call (2 indirect); no psq; field `+0x70` |
| `0x807C31F0` | 112 | R | 4 direct calls; no psq; field `+0xc0` |
| `0x807C3260` | 104 | R | ends in virtual tail call (1 indirect); no psq; field `+0x70` |
| `0x807C3540` | 292 | R | ends in virtual tail call (3 indirect); no psq; field `+0x530` |
| `0x807C38A0` | 104 | R | ends in virtual tail call (2 indirect); no psq; field `+0x70` |
| `0x807C39D0` | 52 | R | `lbz r0,+0x502(r3); cmpwi r0,0; blr; lis r4,0x8043; li r5,+0x1; … (8 insns)` |
| `0x807C3A10` | 44 | R | `lbz r0,+0x502(r3); cmpwi r0,0; blr; li r0,+0x0; li r4,+0x1; … (8 insns)` |
| `0x807C3A40` | 44 | R | `lbz r0,+0x502(r3); cmpwi r0,0; blr; li r4,+0x1; li r0,+0x0; … (8 insns)` |
| `0x807C3A70` | 24 | R | `li r0,+0x0; li r4,+0x1; stb r4,+0x503(r3); stb r0,+0x505(r3); stb r0,+0x502(r3); blr` |
| `0x807C3A90` | 44 | R | `lbz r0,+0x4fe(r3); cmpwi r0,0; blr; li r0,+0x0; li r4,+0x1; … (8 insns)` |
| `0x807C3AC0` | 44 | R | `lbz r0,+0x4fe(r3); cmpwi r0,0; blr; li r4,+0x1; li r0,+0x0; … (8 insns)` |
| `0x807C3AF0` | 12 | R | `li r0,+0x1; stb r0,+0x4ff(r3); blr` |
| `0x807C3B00` | 8 | R | `lbz r3,+0x500(r3); blr` |
| `0x807C3B10` | 32 | R | `li r4,+0x1; li r0,+0x0; stb r4,+0x506(r3); stb r4,+0x508(r3); stb r0,+0x507(r3); … (8 insns)` |
| `0x807C3B30` | 16 | R | `li r0,+0x1; stb r0,+0x507(r3); stb r0,+0x50f(r3); blr` |
| `0x807C3B40` | 20 | R | `li r4,+0x1; li r0,+0x0; stb r4,+0x53d(r3); stb r0,+0x540(r3); blr` |
| `0x807C3B60` | 52 | R | `lis r5,0x8035; rlwinm r0,r4,2,0,29; addi r5,r5,+0x4e60; lwzx r0,r5,r0; rlwinm r0,r0,2,0,29; … (8 insns)` |
| `0x807C5B10` | 248 | R | 2 direct calls; no psq; field `+0x7c` |
| `0x807C5E00` | 96 | R | 4 direct calls; no psq |
| `0x807C5E60` | 56 | R | `stwu r1,-0x10(r1); mfspr r0,lr; lis r4,0x807d; stw r0,+0x14(r1); lfs f0,+0xbd8(r4); … (8 insns)` |
| `0x807C5EA0` | 136 | R | ends in virtual tail call (1 indirect); no psq; field `+0x158` |

### Class P - needs one pinned fact (34 rows, 8,352 bytes)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x80643260` | 148 | P | prev W112; TSV extent 148 B stops at a `rlwinm`; raw continues with `b` and `blr` at 0x806432F4/0x806432F8; extent 148 -> 156 |
| `0x80648710` | 84 | P | 1 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x8064DA10` | 180 | P | 2 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x8067A520` | 232 | P | prev W151; 2 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x80680CD0` | 424 | P | 1 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x80682F40` | 400 | P | 1 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x806830D0` | 264 | P | 2 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x806914D0` | 108 | P | 1 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x80691860` | 284 | P | 3 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x80691D20` | 248 | P | 3 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x806BBB90` | 320 | P | 2 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x806C0F00` | 276 | P | prev W105; 1 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x806CAD10` | 304 | P | prev W134; 2 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x806CB140` | 208 | P | prev W134; 2 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x806E26B0` | 228 | P | prev W109; 2 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x806E3EB0` | 188 | P | prev W109; 2 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x806EC5F0` | 220 | P | prev W109; 2 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x806EC6D0` | 172 | P | prev W109; 2 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x806EC780` | 172 | P | prev W109; 2 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x806EDE70` | 308 | P | prev W109; 1 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x80706850` | 304 | P | 1 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x80713250` | 244 | P | prev W110; 2 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x80713B00` | 376 | P | prev W110; 1 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x8071F120` | 356 | P | 1 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x8072CE80` | 256 | P | 2 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x80730360` | 212 | P | prev W128; 2 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x80731780` | 368 | P | 2 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x80734E30` | 332 | P | prev W128; 1 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x8073F000` | 120 | P | prev W128; 1 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x80743A20` | 196 | P | 1 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x80751C90` | 120 | P | prev W106; 1 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x80752B90` | 156 | P | 2 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x80754620` | 80 | P | prev W106; 1 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |
| `0x80758880` | 464 | P | prev W144; 2 data `psq` in qr3; slot/base/DQ explicit, only the GQR3 type/scale word is external |

The 33 `qr3` rows need the value written into GQR3 by `main.dol`'s
`OSInitFastCast` (no `mtspr GQRn` exists anywhere in `d_basesNP`). Pin it once
and all 33 become mechanical: every slot, base, `W` bit and `DQ` is explicit in
the instruction word. `0x80643260` needs its extent corrected from 148 to
156 bytes (the raw stream continues with `b` and `blr`).

### Class S - already covered under an existing body (103 rows, 6,916 bytes)

| address | bytes | class | concrete note |
|---|---:|---|---|
| `0x806001E0` | 4 | S | byte-identical to covered `fn_806122f0`; separate slot, clone the existing body |
| `0x806001F0` | 4 | S | byte-identical to covered `fn_806122f0`; separate slot, clone the existing body |
| `0x80601CD0` | 64 | S | already translated in module `game::player` as `fn_80601cd0`; the address is missing only from the bases ledger slice |
| `0x80601EF0` | 96 | S | already translated in module `game::player` as `fn_80601ef0`; the address is missing only from the bases ledger slice |
| `0x80602C20` | 328 | S | already translated in module `game::player` as `fn_80602c20`; the address is missing only from the bases ledger slice |
| `0x80602EE0` | 180 | S | already translated in module `game::player` as `fn_80602ee0`; the address is missing only from the bases ledger slice |
| `0x80602FB0` | 64 | S | already translated in module `game::player` as `fn_80602fb0`; the address is missing only from the bases ledger slice |
| `0x80602FF0` | 88 | S | already translated in module `game::player` as `fn_80602ff0`; the address is missing only from the bases ledger slice |
| `0x806099A0` | 96 | S | byte-identical to covered `fn_80611790`; separate slot, clone the existing body |
| `0x8065BFB0` | 12 | S | prev W120; byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x8065BFC0` | 12 | S | prev W120; byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x8065BFD0` | 12 | S | prev W120; byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x8065BFE0` | 12 | S | prev W120; byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x8065BFF0` | 12 | S | prev W120; byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x8065C000` | 12 | S | prev W120; byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x806627F0` | 224 | S | byte-identical to covered `fn_806535e0`; separate slot, clone the existing body |
| `0x80665120` | 12 | S | prev W126; byte-identical to covered `fn_8065b220`; separate slot, clone the existing body |
| `0x80665130` | 12 | S | prev W126; byte-identical to covered `fn_8066f400`; separate slot, clone the existing body |
| `0x80665140` | 12 | S | prev W126; byte-identical to covered `fn_8065b230`; separate slot, clone the existing body |
| `0x80665240` | 224 | S | prev W126; byte-identical to covered `fn_806535e0`; separate slot, clone the existing body |
| `0x806654F0` | 12 | S | prev W126; byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x80665500` | 12 | S | prev W126; byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x80665510` | 12 | S | prev W126; byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x80665520` | 12 | S | prev W126; byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x80666860` | 224 | S | byte-identical to covered `fn_806535e0`; separate slot, clone the existing body |
| `0x80666AD0` | 12 | S | prev W126; byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x80666AE0` | 12 | S | prev W126; byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x806675E0` | 224 | S | byte-identical to covered `fn_806535e0`; separate slot, clone the existing body |
| `0x8066F500` | 224 | S | byte-identical to covered `fn_806535e0`; separate slot, clone the existing body |
| `0x80673BA0` | 224 | S | byte-identical to covered `fn_806535e0`; separate slot, clone the existing body |
| `0x806744C0` | 48 | S | byte-identical to covered `fn_8064cc00`; separate slot, clone the existing body |
| `0x8067C490` | 224 | S | prev W151; byte-identical to covered `fn_806535e0`; separate slot, clone the existing body |
| `0x8067D2F0` | 224 | S | prev W151; byte-identical to covered `fn_806535e0`; separate slot, clone the existing body |
| `0x80683D40` | 48 | S | byte-identical to covered `fn_8066c670`; separate slot, clone the existing body |
| `0x806990B0` | 224 | S | byte-identical to covered `fn_806535e0`; separate slot, clone the existing body |
| `0x806A5DD0` | 12 | S | byte-identical to covered `fn_8069d870`; separate slot, clone the existing body |
| `0x806A5DE0` | 12 | S | byte-identical to covered `fn_80698fa0`; separate slot, clone the existing body |
| `0x806A5DF0` | 12 | S | byte-identical to covered `fn_80698fc0`; separate slot, clone the existing body |
| `0x806A5EE0` | 224 | S | byte-identical to covered `fn_806535e0`; separate slot, clone the existing body |
| `0x806A8270` | 12 | S | byte-identical to covered `fn_806ef3b0`; separate slot, clone the existing body |
| `0x806A9700` | 12 | S | byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x806A9710` | 12 | S | byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x806A9720` | 12 | S | byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x806A9730` | 12 | S | byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x806AB480` | 224 | S | byte-identical to covered `fn_806535e0`; separate slot, clone the existing body |
| `0x806AB6B0` | 12 | S | byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x806AB6C0` | 12 | S | byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x806AB6D0` | 12 | S | byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x806ADF30` | 224 | S | byte-identical to covered `fn_806535e0`; separate slot, clone the existing body |
| `0x806AEEB0` | 12 | S | byte-identical to covered `fn_806ef3b0`; separate slot, clone the existing body |
| `0x806AEFB0` | 224 | S | byte-identical to covered `fn_806535e0`; separate slot, clone the existing body |
| `0x806B4DB0` | 12 | S | prev W118; byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x806B4EC0` | 8 | S | prev W118; byte-identical to covered `fn_80738890`; separate slot, clone the existing body |
| `0x806BAAB0` | 12 | S | byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x806BAAC0` | 12 | S | byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x806BB220` | 4 | S | byte-identical to covered `fn_806122f0`; separate slot, clone the existing body |
| `0x806BC310` | 12 | S | prev W118; byte-identical to covered `fn_806ef3b0`; separate slot, clone the existing body |
| `0x806BF660` | 12 | S | prev W118; byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x806BF670` | 12 | S | prev W118; byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x806C3930` | 12 | S | byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x806C3940` | 12 | S | byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x806C3950` | 12 | S | byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x806C3960` | 12 | S | byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x806C3970` | 12 | S | byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x806C3980` | 12 | S | byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x806C62A0` | 12 | S | byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x806C62B0` | 12 | S | byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x806C62C0` | 12 | S | byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x806C7040` | 224 | S | prev W134; byte-identical to covered `fn_806535e0`; separate slot, clone the existing body |
| `0x806E7EF0` | 48 | S | byte-identical to covered `fn_8066c670`; separate slot, clone the existing body |
| `0x806E9C10` | 224 | S | byte-identical to covered `fn_806535e0`; separate slot, clone the existing body |
| `0x806F5A30` | 48 | S | byte-identical to covered `fn_8066c670`; separate slot, clone the existing body |
| `0x806F6040` | 224 | S | byte-identical to covered `fn_806535e0`; separate slot, clone the existing body |
| `0x806F8780` | 12 | S | byte-identical to covered `fn_80659170`; separate slot, clone the existing body |
| `0x806F8870` | 224 | S | byte-identical to covered `fn_806535e0`; separate slot, clone the existing body |
| `0x806FEBB0` | 96 | S | byte-identical to covered `fn_80611790`; separate slot, clone the existing body |
| `0x8071B700` | 48 | S | byte-identical to covered `fn_8066c670`; separate slot, clone the existing body |
| `0x8071F940` | 12 | S | prev W139; byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x807203D0` | 224 | S | prev W125; byte-identical to covered `fn_806535e0`; separate slot, clone the existing body |
| `0x80721020` | 12 | S | prev W125; byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x80722700` | 224 | S | byte-identical to covered `fn_806535e0`; separate slot, clone the existing body |
| `0x80722930` | 12 | S | prev W125; byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x80722940` | 12 | S | byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x80722950` | 12 | S | byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x80722960` | 12 | S | prev W125; byte-identical to covered `fn_80656f50`; separate slot, clone the existing body |
| `0x80723A30` | 224 | S | byte-identical to covered `fn_806535e0`; separate slot, clone the existing body |
| `0x8072C1F0` | 4 | S | prev W125; byte-identical to covered `fn_806122f0`; separate slot, clone the existing body |
| `0x807433A0` | 224 | S | byte-identical to covered `fn_806535e0`; separate slot, clone the existing body |
| `0x80750DA0` | 4 | S | prev W106; byte-identical to covered `fn_806122f0`; separate slot, clone the existing body |
| `0x807622A0` | 4 | S | prev W147; byte-identical to covered `fn_806122f0`; separate slot, clone the existing body |
| `0x80762B20` | 16 | S | prev W114; byte-identical to covered `fn_80762000`; separate slot, clone the existing body |
| `0x807639F0` | 48 | S | byte-identical to covered `fn_80760b50`; separate slot, clone the existing body |
| `0x8076B090` | 4 | S | prev W114; byte-identical to covered `fn_806122f0`; separate slot, clone the existing body |
| `0x8076BA60` | 4 | S | prev W147; byte-identical to covered `fn_806122f0`; separate slot, clone the existing body |
| `0x807769B0` | 48 | S | byte-identical to covered `fn_8075ead0`; separate slot, clone the existing body |
| `0x80778A50` | 96 | S | byte-identical to covered `fn_80611790`; separate slot, clone the existing body |
| `0x8077A9F0` | 48 | S | byte-identical to covered `fn_8075f040`; separate slot, clone the existing body |
| `0x8077D660` | 4 | S | prev W100; byte-identical to covered `fn_806122f0`; separate slot, clone the existing body |
| `0x8077D670` | 8 | S | prev W100; byte-identical to covered `fn_8079ef10`; separate slot, clone the existing body |
| `0x80792A10` | 4 | S | prev W135; byte-identical to covered `fn_806122f0`; separate slot, clone the existing body |
| `0x8079EF00` | 4 | S | prev W135; byte-identical to covered `fn_806122f0`; separate slot, clone the existing body |
| `0x807A2200` | 48 | S | prev W127; byte-identical to covered `fn_806521f0`; separate slot, clone the existing body |
| `0x807C5B00` | 8 | S | byte-identical to covered `fn_8079ef10`; separate slot, clone the existing body |

### Class U - genuinely unreconstructable (473 rows, 22,700 bytes)

All 473 are `0x802DCBFC` forwarders. They are listed as address runs so the
table stays readable; `bytes` is the run total.

| addresses | n | bytes | class | concrete note |
|---|---:|---:|---|---|
| `0x80601780` `0x806017B0` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x80604360` `0x80604390` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x806064E0` `0x80606510` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x80608350` `0x80608380` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x80609B10` `0x80609B40` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x8060B620` `0x8060B650` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x8060CB80` `0x8060CBB0` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x8060DE00` `0x8060DE30` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x8060EFB0` `0x8060EFE0` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x80610850` `0x80610880` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x80611900` `0x80611930` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x806126E0` `0x80612710` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x80613C40` `0x80613C70` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x80614EE0` `0x80614F10` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x80618CA0` `0x80618CD0` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x8061BC80` `0x8061BCB0` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x8061D0B0` `0x8061D0E0` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x8061E150` `0x8061E180` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x8061FAB0` `0x8061FAE0` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x80621FB0` `0x80621FE0` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x80623A50` `0x80623A80` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x80624E80` `0x80624EB0` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x806261E0` `0x80626210` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x80627600` `0x80627630` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x8062A260` `0x8062A290` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x8062EFF0` `0x8062F020` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x806318F0` `0x80631920` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x80634C90` `0x80634CC0` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x806360D0` `0x80636100` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x80639390` `0x806393C0` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x8063AEB0` `0x8063AEE0` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x8063D740` `0x8063D770` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x8063F7F0` `0x8063F820` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x80641570` `0x806415A0` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x80644180` `0x806441B0` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x806457C0` `0x806457F0` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x80646590` `0x806465C0` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x8064A0F0` `0x8064A120` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8064B160` `0x8064B190` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8064C340` `0x8064C370` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8064D580` `0x8064D5B0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8064E590` `0x8064E5C0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80650140` `0x80650170` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80650810` `0x80650840` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x80652080` `0x806520B0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80652700` `0x80652730` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80653780` `0x806537B0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80655EC0` `0x80655EF0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80656C90` `0x80656CC0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80659400` `0x80659430` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8065A810` `0x8065A840` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8065B4C0` `0x8065B4F0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8065BE20` `0x8065BE50` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80662990` `0x806629C0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806653E0` `0x80665410` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80666A00` `0x80666A30` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80667780` `0x806677B0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80668420` `0x80668450` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8066A7C0` `0x8066A7F0` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x8066BE10` `0x8066BE40` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8066D530` `0x8066D560` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8066F6A0` `0x8066F6D0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806705E0` `0x80670610` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80672FC0` `0x80672FF0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80673D40` `0x80673D70` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80674960` `0x80674990` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806761C0` `0x806761F0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806776D0` `0x80677700` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80679A10` `0x80679A40` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80679FA0` `0x80679FD0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8067ADF0` `0x8067AE20` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8067BA10` `0x8067BA40` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8067C630` `0x8067C660` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8067D490` `0x8067D4C0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8067F470` `0x8067F4A0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806815A0` `0x806815D0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806820C0` `0x806820F0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806836C0` `0x806836F0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80684270` `0x806842A0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80685650` `0x80685680` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806877F0` `0x80687820` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806899E0` `0x80689A10` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8068A3B0` `0x8068A3E0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8068B380` `0x8068B3B0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8068DDC0` `0x8068DDF0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8068F070` `0x8068F0A0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80690150` `0x80690180` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806913F0` `0x80691420` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806928C0` `0x806928F0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80693650` `0x80693680` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806945E0` `0x80694610` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80696950` `0x80696980` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80697670` `0x806976A0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80699250` `0x80699280` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8069DB30` `0x8069DB60` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806A1560` `0x806A1590` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806A2230` `0x806A2260` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806A6080` `0x806A60B0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806A8500` `0x806A8530` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806A9550` `0x806A9580` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806A9670` `0x806A96A0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806AB620` `0x806AB650` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806AE0D0` `0x806AE100` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806AF150` `0x806AF180` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806AF860` `0x806AF890` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806B0210` `0x806B0240` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806B14A0` `0x806B14D0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806B26D0` `0x806B2700` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806B3360` `0x806B3390` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806B4810` `0x806B4840` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806B6D80` `0x806B6DB0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806B8E10` `0x806B8E40` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806BA2A0` `0x806BA2D0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806BA940` `0x806BA970` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806BC5B0` `0x806BC5E0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806BEAD0` `0x806BEB00` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806BF5D0` `0x806BF600` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806C0B10` `0x806C0B40` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806C2FB0` `0x806C2FE0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806C5D20` `0x806C5D50` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806C71E0` `0x806C7210` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806C7300` `0x806C7330` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806C8320` `0x806C8350` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806C9020` `0x806C9050` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806CA940` `0x806CA970` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806CBC10` `0x806CBC40` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806CCFF0` `0x806CD020` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806CDE00` `0x806CDE30` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806CE940` `0x806CE970` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806D1410` `0x806D1440` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806D27E0` `0x806D2810` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806D4450` `0x806D4480` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806D5230` `0x806D5260` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806D7510` `0x806D7540` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806D8260` `0x806D8290` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806D9200` `0x806D9230` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806DB050` `0x806DB080` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806DBE70` `0x806DBEA0` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x806DCDE0` `0x806DCE10` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806DEC20` `0x806DEC50` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806E0840` `0x806E0870` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806E11E0` `0x806E1210` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806E1720` `0x806E1750` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806E3910` `0x806E3940` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806E3BE0` | 1 | 44 | U | hidden r12 offset 0x5E4 |
| `0x806E4450` `0x806E4480` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806E5A60` `0x806E5A90` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806E6BB0` `0x806E6BE0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806E7810` `0x806E7840` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806E9DB0` `0x806E9DE0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806EAAC0` `0x806EAAF0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806EBD60` `0x806EBD90` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806EE5D0` `0x806EE600` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806EF640` `0x806EF670` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806EFE90` `0x806EFEC0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806F1C50` `0x806F1C80` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806F2DF0` `0x806F2E20` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806F61E0` `0x806F6210` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806F73A0` `0x806F73D0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806FAAD0` `0x806FAB00` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806FC960` `0x806FC990` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x806FED20` `0x806FED50` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x80700F90` `0x80700FC0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x807017C0` `0x807017F0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80704E00` `0x80704E30` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x807054E0` `0x80705510` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80706CF0` `0x80706D20` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80707CE0` `0x80707D10` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x807083F0` `0x80708420` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8070B040` `0x8070B070` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8070F210` `0x8070F240` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80711E30` `0x80711E60` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x807157B0` `0x807157E0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80717A80` `0x80717AB0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80719CA0` `0x80719CD0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8071EBC0` `0x8071EBF0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80720570` `0x807205A0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80720D90` `0x80720DC0` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x807228A0` `0x807228D0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80723BD0` `0x80723C00` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80726260` `0x80726290` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80728200` `0x80728230` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80728B90` `0x80728BC0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80729F50` `0x80729F80` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8072A880` `0x8072A8B0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8072C950` `0x8072C980` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8072DBA0` `0x8072DBD0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8072F1F0` `0x8072F220` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80730A90` `0x80730AC0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80732770` `0x807327A0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80733360` `0x80733390` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x807348F0` `0x80734920` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80735260` `0x80735290` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8073A290` `0x8073A2C0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8073AB30` `0x8073AB60` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8073B560` `0x8073B590` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8073C240` `0x8073C270` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8073CBE0` `0x8073CC10` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8073ECB0` `0x8073ECE0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80742920` `0x80742950` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80743540` `0x80743570` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80743F20` | 1 | 48 | U | hidden r12 offset 0x24 |
| `0x807477D0` `0x80747800` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80747F40` `0x80747F70` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80749120` `0x80749150` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8074A2D0` `0x8074A300` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8074B530` `0x8074B560` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8074C050` `0x8074C080` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8074D160` `0x8074D190` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8074D640` `0x8074D670` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8074E0C0` `0x8074E0F0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x8074ED50` `0x8074ED80` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80750B90` `0x80750BC0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80751400` `0x80751430` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80751860` `0x80751890` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x807520A0` `0x807520D0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80753170` `0x807531A0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80754080` `0x807540B0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80754DD0` `0x80754E00` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x807554C0` `0x807554F0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x807566A0` `0x807566D0` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80757A20` `0x80757A50` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x80759C00` `0x80759C30` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x80778BC0` `0x80778BF0` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x807875D0` `0x80787600` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x807A3820` `0x807A3850` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x807A4F00` `0x807A4F30` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x807A6300` `0x807A6330` | 2 | 96 | U | hidden r12 offset 0x18/0x24 |
| `0x807A9B60` `0x807A9B90` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x807AC290` `0x807AC2C0` `0x807AC2F0` | 3 | 144 | U | hidden r12 offset 0x24/0x18/0xC |
| `0x807ADF40` `0x807ADF70` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x807AF1E0` `0x807AF210` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x807B0BF0` `0x807B0C20` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x807B30E0` `0x807B3110` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x807B43E0` `0x807B4410` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x807BD0B0` `0x807BD0E0` | 2 | 96 | U | hidden r12 offset 0x18/0xC |
| `0x807C3D10` `0x807C3D40` | 2 | 96 | U | hidden r12 offset 0x18/0xC |

## Distribution by 64 KiB block

| block | remaining | R | P | S | U |
|---|---:|---:|---:|---:|---:|
| `0x80600000` | 71 | 44 | 0 | 9 | 18 |
| `0x80610000` | 62 | 42 | 0 | 0 | 20 |
| `0x80620000` | 62 | 48 | 0 | 0 | 14 |
| `0x80630000` | 59 | 45 | 0 | 0 | 14 |
| `0x80640000` | 59 | 38 | 3 | 0 | 18 |
| `0x80650000` | 60 | 32 | 0 | 6 | 22 |
| `0x80660000` | 61 | 29 | 0 | 14 | 18 |
| `0x80670000` | 65 | 34 | 1 | 4 | 26 |
| `0x80680000` | 52 | 26 | 3 | 1 | 22 |
| `0x80690000` | 65 | 43 | 3 | 1 | 18 |
| `0x806A0000` | 62 | 26 | 0 | 16 | 20 |
| `0x806B0000` | 62 | 29 | 1 | 8 | 24 |
| `0x806C0000` | 64 | 27 | 3 | 10 | 24 |
| `0x806D0000` | 56 | 34 | 0 | 0 | 22 |
| `0x806E0000` | 67 | 30 | 6 | 2 | 29 |
| `0x806F0000` | 65 | 46 | 0 | 5 | 14 |
| `0x80700000` | 58 | 39 | 1 | 0 | 18 |
| `0x80710000` | 60 | 45 | 3 | 2 | 10 |
| `0x80720000` | 57 | 23 | 1 | 9 | 24 |
| `0x80730000` | 61 | 35 | 4 | 0 | 22 |
| `0x80740000` | 60 | 33 | 1 | 1 | 25 |
| `0x80750000` | 66 | 39 | 4 | 1 | 22 |
| `0x80760000` | 64 | 59 | 0 | 5 | 0 |
| `0x80770000` | 58 | 51 | 0 | 5 | 2 |
| `0x80780000` | 60 | 58 | 0 | 0 | 2 |
| `0x80790000` | 56 | 54 | 0 | 2 | 0 |
| `0x807A0000` | 66 | 50 | 0 | 1 | 15 |
| `0x807B0000` | 63 | 55 | 0 | 0 | 8 |
| `0x807C0000` | 65 | 62 | 0 | 1 | 2 |

Every block still has between 52 and 71 unclaimed rows; the remaining work is
uniformly spread, not concentrated in one subsystem.

## Ranked next-claim list

Ranking favours **dense groups of small complete bodies**: they are fast to
verify, they cluster in one class family, and one reviewer pass covers a whole
sibling set. Large single bodies are valuable but should not be mixed into the
same wave as a 12-body accessor cluster.

Rank 1 is the best single claim in the module: `0x807C39D0..0x807C3B60` is a
state-flag machine over `+0x4F0..+0x540` (set/clear/test triples, one
pure getter, one indexed lookup), 12 bodies and 392 bytes total, with no
paired-single and no indirect dispatch. Ranks 4-8 are five copies of the same
"two 64-byte accessors plus one 96-byte applier" family, so one naming pass
serves all fifteen bodies.

| rank | range | bodies | bytes | first work item |
|---:|---|---:|---:|---|
| 1 | `0x807C39D0`..`0x807C3B60` | 12 | 392 | lbz r0,+0x502(r3); cmpwi r0,0; blr; lis r4,0x8043; li r5,+0x1; … (8 insns) |
| 2 | `0x807AC260`..`0x807AC280` | 3 | 48 | lwz r12,+0x0(r3); lwz r12,+0x8(r12); mtctr r12; bctr |
| 3 | `0x806A6A70`..`0x806A6AA0` | 3 | 156 | li r3,+0x136; li r4,+0x0; b 0x80162D50 — export truncated, raw body complete |
| 4 | `0x80606A10`..`0x80606A90` | 3 | 224 | stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns) |
| 5 | `0x80608FD0`..`0x80609050` | 3 | 224 | stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns) |
| 6 | `0x8061D790`..`0x8061D810` | 3 | 224 | stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns) |
| 7 | `0x80668F60`..`0x80668FE0` | 3 | 224 | stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns) |
| 8 | `0x80757D30`..`0x80757DB0` | 3 | 224 | stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insns) |
| 9 | `0x8071FB10`..`0x8071FB70` | 3 | 292 | stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); lwzu r12,+0x6b8(r3); lwz r12,+0x14(r12); … (8 insns) |
| 10 | `0x80699F80`..`0x80699FE0` | 3 | 432 | lfs f1,+0xac(r3); lfs f0,+0xd98(r3); fcmpo cr0,f1,f0; blt 0x80699F9C; lfs f0,+0xd94(r3); … (8 insns) |
| 11 | `0x80721270`..`0x80721290` | 2 | 36 | stw r4,+0x63c(r3); lis r4,0x8083; addi r4,r4,+0x290; lwz r12,+0x60(r3); lwz r12,+0xd4(r12); … (7 insns) |
| 12 | `0x80781C80`..`0x80781C90` | 2 | 48 | lis r4,0x807d; li r5,+0x6; addi r4,r4,-0x9f8; b 0x800F6D60 — export truncated, raw body complete |
| 13 | `0x8066B440`..`0x8066B470` | 2 | 52 | stwu r1,-0x10(r1); mfspr r0,lr; addi r3,r3,+0x594; stw r0,+0x14(r1); bl 0x8007F950; … (8 insns) |
| 14 | `0x8069ACD0`..`0x8069ACE0` | 2 | 68 | li r0,+0xb4; stw r4,+0xdd0(r3); stw r0,+0xdd4(r3); blr |
| 15 | `0x80615CC0`..`0x80615CF0` | 2 | 72 | lwz r0,+0x2b4(r3); lis r4,0x807c; addi r4,r4,+0x6c0c; li r5,+0x0; rlwinm r0,r0,2,0,29; … (8 insns) — export truncated,… |
| 16 | `0x806BA630`..`0x806BA660` | 2 | 84 | addi r3,r3,+0xac; b 0x800944F0 — export truncated, raw body complete |
| 17 | `0x8071F780`..`0x8071F7B0` | 2 | 84 | stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); lwzu r12,+0x3b4(r3); lwz r12,+0x14(r12); … (8 insns) |
| 18 | `0x807571C0`..`0x807571E0` | 2 | 92 | lbz r3,+0x63d(r3); lis r5,0x807d; lis r4,0x807d; lfs f1,-0x1628(r5); lfs f2,-0x1658(r4); … (7 insns) — export truncated… |
| 19 | `0x806BC610`..`0x806BC640` | 2 | 108 | stwu r1,-0x10(r1); mfspr r0,lr; li r3,+0xc50; stw r0,+0x14(r1); bl 0x801628C0; … (8 insns) |
| 20 | `0x80615E00`..`0x80615E40` | 2 | 120 | stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); stw r31,+0xc(r1); or r31,r3,r3; … (8 insns) |

### Top 10 individual addresses

The head of each of the ten highest-ranked groups, i.e. the ten bodies a
first batch should claim first.

| # | address | bytes | why |
|---:|---|---:|---|
| 1 | `0x807C39D0` | 52 | head of a 12-body group, 392 B total; lbz r0,+0x502(r3); cmpwi r0,0; blr; lis r4,0x8043; li r5,+0x1; … (8 insns) |
| 2 | `0x807AC260` | 16 | head of a 3-body group, 48 B total; lwz r12,+0x0(r3); lwz r12,+0x8(r12); mtctr r12; bctr |
| 3 | `0x806A6A70` | 12 | head of a 3-body group, 156 B total; li r3,+0x136; li r4,+0x0; b 0x80162D50 — export truncated, raw body complete |
| 4 | `0x80606A10` | 64 | head of a 3-body group, 224 B total; stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insn… |
| 5 | `0x80608FD0` | 64 | head of a 3-body group, 224 B total; stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insn… |
| 6 | `0x8061D790` | 64 | head of a 3-body group, 224 B total; stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insn… |
| 7 | `0x80668F60` | 64 | head of a 3-body group, 224 B total; stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insn… |
| 8 | `0x80757D30` | 64 | head of a 3-body group, 224 B total; stwu r1,-0x10(r1); mfspr r0,lr; cmpwi r3,0; stw r0,+0x14(r1); stw r31,+0xc(r1); … (8 insn… |
| 9 | `0x8071FB10` | 48 | head of a 3-body group, 292 B total; stwu r1,-0x10(r1); mfspr r0,lr; stw r0,+0x14(r1); lwzu r12,+0x6b8(r3); lwz r12,+0x14(r12)… |
| 10 | `0x80699F80` | 44 | head of a 3-body group, 432 B total; lfs f1,+0xac(r3); lfs f0,+0xd98(r3); fcmpo cr0,f1,f0; blt 0x80699F9C; lfs f0,+0xd94(r3);… |

### Quick singles (isolated R bodies of 48 bytes or less)

| address | bytes | note |
|---|---:|---|
| `0x80667DD0` | 4 | b 0x80667DE0 |
| `0x80741CA0` | 4 | b 0x80741A70 |
| `0x80776C00` | 4 | b 0x80776C10 |
| `0x807812D0` | 4 | b 0x80100940 — export truncated, raw body complete |
| `0x80792EE0` | 4 | b 0x80162590 — export truncated, raw body complete |
| `0x8065DCA0` | 8 | li r3,+0x7; b 0x800FFF50 — export truncated, raw body complete |
| `0x80667CF0` | 8 | addi r3,r3,+0xac; b 0x80094560 — export truncated, raw body complete |
| `0x807B08A0` | 8 | or r3,r4,r4; b 0x801648B0 — export truncated, raw body complete |
| `0x807B8EC0` | 8 | or r3,r4,r4; b 0x801648B0 — export truncated, raw body complete |
| `0x80662700` | 12 | lis r3,0x8035; addi r3,r3,+0x2670; blr |
| `0x80665150` | 12 | lis r3,0x8035; addi r3,r3,+0x2670; blr |
| `0x806AEEC0` | 12 | lis r3,0x8083; addi r3,r3,-0x34fc; blr |
| `0x806BC320` | 12 | lis r3,0x8083; addi r3,r3,-0x34fc; blr |
| `0x806BE840` | 12 | lis r3,0x8035; addi r3,r3,+0x7874; blr |
| `0x806E0E20` | 12 | lwz r0,+0x0(r4); stw r0,+0x700(r3); blr |
| `0x806E9B20` | 12 | lis r3,0x8036; addi r3,r3,-0x7efc; blr |
| `0x80761670` | 12 | li r4,+0x0; addi r3,r3,+0x20c; b 0x801662E0 — export truncated, raw body complete |
| `0x80765180` | 12 | li r0,+0x1; stw r0,+0x47c(r3); b 0x800F6CB0 — export truncated, raw body complete |
| `0x8078E900` | 12 | lis r3,0x8043; lwz r3,-0x5b58(r3); b 0x8019BC30 — export truncated, raw body complete |
| `0x806E8990` | 16 | lis r5,0x807d; addi r4,r3,+0xac; lfs f1,-0x4204(r5); b 0x80097170 — export truncated, raw body complete |
| `0x80727180` | 16 | lis r5,0x807d; addi r4,r3,+0xac; lfs f1,-0x2d88(r5); b 0x80097170 — export truncated, raw body complete |
| `0x8078A270` | 16 | lis r4,0x807d; li r5,+0x3; addi r4,r4,-0x510; b 0x800F6D60 — export truncated, raw body complete |
| `0x8079B110` | 16 | lis r4,0x807d; li r5,+0x6; addi r4,r4,+0xa0; b 0x800F6D60 — export truncated, raw body complete |
| `0x8068AB10` | 20 | lwz r0,+0x55c(r3); li r4,+0x4; rlwinm r0,r0,0,31,29; stwu r0,+0x55c(r3); b 0x800B8250 — export truncated, raw… |
| `0x80693060` | 20 | lwz r0,+0x554(r3); li r4,+0x4; rlwinm r0,r0,0,31,29; stwu r0,+0x554(r3); b 0x800B8250 — export truncated, raw… |

### Largest remaining bodies (all class R, claim last)

| address | bytes | note |
|---|---:|---|
| `0x8079C330` | 4232 | ends in virtual tail call (13 indirect); prologue save-helper `0x802dcd6c`; no psq |
| `0x8077B450` | 4076 | ends in virtual tail call (29 indirect); prologue save-helper `0x802dcd68`; no psq |
| `0x8065CB20` | 3788 | 11 direct calls; prologue save-helper `0x802dcd40`; 9 qr0 psq (float pair / f31 save) |
| `0x80661070` | 3432 | ends in virtual tail call (17 indirect); prologue save-helper `0x802dcd5c`; 3 qr0 psq (float pair / f31 save) |
| `0x8079D3C0` | 3176 | ends in virtual tail call (35 indirect); no psq; field `+0x60` |
| `0x8078DC70` | 2576 | ends in virtual tail call (19 indirect); prologue save-helper `0x802dcd70`; no psq |
| `0x8076EF80` | 2544 | ends in virtual tail call (1 indirect); prologue save-helper `0x802dcd70`; no psq |
| `0x80773210` | 2096 | ends in virtual tail call (10 indirect); 1 qr0 psq (float pair / f31 save) |
| `0x8065DD50` | 1772 | ends in virtual tail call (2 indirect); prologue save-helper `0x802dcd58`; 4 qr0 psq (float pair / f31 save) |
| `0x8065F090` | 1772 | ends in virtual tail call (25 indirect); prologue save-helper `0x802dcd74`; no psq |
| `0x807740D0` | 1656 | 25 direct calls; prologue save-helper `0x802dcd58`; no psq |
| `0x806A24F0` | 1624 | ends in virtual tail call (19 indirect); prologue save-helper `0x802dcd74`; 2 qr0 psq (float pair / f31 save) |

## Verification and limits

- Enumeration was re-run from the TSV and the ledger for this report; the
  totals are 5,615 TSV rows, 3,816 ledger addresses, 3,829 bodies, 1,786 remaining.
- Sizes and addresses in every table come from the TSV, not from prose.
- Every cited export exists under `work/ghidra/rel-d_basesNP/export/`.
- No `merge-ledger`, `check-lane-csv` or `scripts/check.ps1` run was made:
  this lane writes nothing that those gates validate.
- **What "R" does not promise.** R means the bytes carry enough information
  to write a faithful body today; it does not promise a semantic class name.
  Bodies built from a `0x802DCD40..0x802DCD74` save-helper keep their
  receiver in entry `r3` and need the register walk documented by
  `ANALYSIS_WAVE127_BASES.md`; bodies reached only through a virtual tail call
  keep the receiver in `r3` by the ABI.
- **What stays out of reach.** The `0x802DCBFC` trampoline target list is not
  in this REL; only a `main.dol` analysis (or a runtime trace of the
  trampoline) can name `0x802DCBFC`'s callees. Until then 473 rows have no
  describable effect, and writing them as opaque stubs would inflate the
  ledger without adding knowledge.
