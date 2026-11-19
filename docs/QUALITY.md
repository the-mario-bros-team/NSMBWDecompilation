# Decompilation quality report — `main.dol` (SMNE01, r37675)

## 1. Plain-English summary

We scanned all 10,276 decompiled functions for decompiler failure markers and
structural artefacts. The good news is that **Ghidra never fell back to a hard
"cannot decode" marker** (`halt_baddata` appears zero times), so every function was
at least decoded as PowerPC instructions. 1,207 files (11.7%) carry a `WARNING:`
comment, but most of those comments are cosmetic: they describe how Ghidra cleaned
up switch branches or how it could not track an ABI register.

Two things are real limitations rather than cosmetics:

1. **Small `switch` tables are not fully recovered** in 122 functions (1.2%). These
   are tiny dispatchers (median 28 bytes), 82 of them in game code. The decompiled C
   still compiles, but one or more `case` arms may be missing.
2. **Paired-single / GQR floating-point code is mis-rendered** in 728 functions
   (7.1%). Ghidra prints bogus `ldexpf(...)` expressions and `unaff_GQR0` reads where
   the original used `psq_l`/`psq_st` (GX quantized loads). This is concentrated in
   the 0x8028–0x802A render band.

A third, benign finding explains most of the "unaffected register" noise: the
`0x802DCDxx` block is a set of CodeWarrior `__savegprlr_N` / `__restgprlr_N`
register-save helpers, not game logic. When game functions call them, Ghidra reports
`unaff_r25…r31` for registers that the stub reads. Those warnings are **not**
decompilation failures.

**Bottom line:** the export is usable for scalar/integer work and for boot/runtime
translation. Treat 0x8028–0x802A (render) and the 122 switch dispatchers as
"needs raw-instruction review" before trusting the C.

---

## 2. Terms used in this report

| Term | Meaning |
|---|---|
| **Function / file** | One row in `functions.tsv` and one `.c` file named `<address>_<name>.c`. Counts are per file unless stated. |
| **Ghidra** | The decompiler that produced the C (`Ghidra 12.1.4`, `PowerPC:BE:32:Gekko_Broadway`, export `work/ghidra/main-gb/export`). |
| **`halt_baddata`** | Pseudo-instruction Ghidra emits when it **cannot decode bytes as an instruction**. A hard decode failure. Should be zero. |
| **`WARNING:`** | A comment Ghidra adds to the C to explain an assumption or a cleanup. Not necessarily an error. |
| **`unaff_rN`** | "Unaffected register": Ghidra believes the function **reads** register `N` but never sees it written inside the function. Common for ABI registers set before `entry`. |
| **r13** | PowerPC EABI **small-data-area (SDA) base**. Points at `.sdata`/`.sbss`; `0x800041C0` sets it to `0x8042F680`. |
| **r2** | PowerPC EABI **TOC / SDA2 base**; `0x800041C0` sets it to `0x80433080`. |
| **GQR** | Graphics Quantizer Register. Configures paired-single (`psq_l`/`psq_st`) GX vertex loads. |
| **Paired-single** | PowerPC SIMD float instructions (`ps_*`) used by GX/G3D code. Ghidra's Gekko module does not model them well. |
| **Jump table** | A table of code addresses used to implement `switch`. If Ghidra cannot recover it, the `switch` is incomplete. |
| **Thunk** | A 4-byte `b <target>` trampoline; exported as `thunk_FUN_xxxxxxxx`. |
| **SDA-relative global** | A global reached as `r13 + constant` instead of a named `DAT_` address. |
| **Confidence (this report)** | `high` = direct evidence (string xref, vtable, distinctive instruction sequence); `medium` = structural inference from several call sites; `low` = address-band guess only. |

---

## 3. Headline counts

Percentages are of 10,276 functions.

| Artefact | Files | % | Lines | Verdict |
|---|---:|---:|---:|---|
| `halt_baddata` (hard decode failure) | **0** | 0.0% | 0 | **none — good** |
| Any `WARNING:` comment | 1,207 | 11.7% | 4,348 | mostly annotation noise |
| `unaff_*` (any ABI register) | 4,040 | 39.3% | 23,404 | annotation; see §5.2–5.3 |
| `unaff_r13` (SDA base) | 2,333 | 22.7% | — | annotation; unresolved SDA globals |
| `unaff_r2` (TOC base) | 2,128 | 20.7% | — | annotation |
| `unaff_GQR0` (GX quantizer) | 730 | 7.1% | — | paired-single path |
| Bogus `ldexpf(...)` expression | 728 | 7.1% | — | **real, paired-single mis-render** |
| `Could not recover jumptable` | 122 | 1.2% | 128 | **real, small switch** |
| `Treating indirect jump as call` | 122 | 1.2% | 128 | same 122 files as above |
| `Removing unreachable block` | 773 | 7.5% | 3,707 | pruning noise; worst 144 in one function |
| `Globals starting with '_' overlap` | 317 | 3.1% | 317 | symbol-size noise |
| `Do nothing block with infinite loop` | 29 | 0.3% | 50 | mostly intentional `b .` spin loops |
| `Type propagation algorithm not settling` | 13 | 0.1% | 13 | mild, local types |
| `may have set the stack pointer` | 3 | <0.1% | 3 | OS context save/restore stubs |
| `Heritage AFTER dead removal` | 1 | <0.1% | 1 | isolated |
| `Restarted to delay deadcode elimination` | 1 | <0.1% | 1 | isolated |
| `CONCAT` / `extraout_` / `switchD_` | 677 / 161 / 4 | 6.6 / 1.6 / <0.1% | — | normal Ghidra type reconstruction |

Function inventory: 2,224,372 bytes total, mean 216 bytes, median 120 bytes, largest
7,620 bytes. Size distribution:

| Size (bytes) | Functions | Share |
|---|---:|---:|
| < 32 | 1,719 | 16.7% |
| 32–127 | 3,637 | 35.4% |
| 128–511 | 4,023 | 39.1% |
| 512–2,047 | 829 | 8.1% |
| ≥ 2,048 | 68 | 0.7% |

Non-game entries that inflate the count: 85 `thunk_*` trampolines, 95 functions whose
body is only `return;`, 162 functions of 4 bytes, 576 of ≤ 8 bytes.

---

## 4. Warning classes in detail

`WARNING:` text was normalised (addresses replaced by `ADDR`) and grouped. The nine
distinct classes are listed below with their real meaning.

| Warning class | Lines | Files | What it means | Decode failure? | Action |
|---|---:|---:|---|---|---|
| `Removing unreachable block (ram,ADDR)` | 3,707 | 773 | Ghidra deleted a block it believed was unreachable after branch/switch recovery. In paired-single code this often removes real fall-through. | No (CFG cleanup), but can hide code | Trust only when the function has no GQR/`ldexpf` pattern |
| `Globals starting with '_' overlap smaller symbols` | 317 | 317 | Data-symbol size overlap (e.g. `_DAT_80000044` inside a larger object). | No | Cosmetic |
| `Treating indirect jump as call` | 128 | 122 | A computed jump (`bctr`) was rendered as a call. | Yes, local CFG | Review the 122 switch functions |
| `Could not recover jumptable at ADDR. Too many branches` | 128 | 122 | Switch table not recovered. | Yes, local CFG | Recover from raw table if gameplay-relevant |
| `Do nothing block with infinite loop` | 50 | 29 | A block with no exit, usually an intentional `b .` idle/spin loop. | Usually no | Accept; verify context |
| `Type propagation algorithm not settling` | 13 | 13 | Local variable types oscillated. | No | Cosmetic; check pointer casts |
| `This function may have set the stack pointer` | 3 | 3 | Context save/restore (`802e9300`, `802e9398`, …). | No | Expected for OS context code |
| `Heritage AFTER dead removal` | 1 | 1 | One-off decompiler bookkeeping. | No | Ignore |
| `Restarted to delay deadcode elimination` | 1 | 1 | One-off decompiler bookkeeping. | No | Ignore |

All 122 "indirect jump" files are the **same** 122 "jumptable" files — one defect per
function, not two. Their size distribution: median 28 bytes, mean 39 bytes, maximum
576 bytes. By band: **82 game code** (< 0x801A0000), **13 SDK** (0x801A–0x8022),
**27 middleware** (≥ 0x8023). These are small state/ID dispatchers, not the huge
render functions.

---

## 5. Findings that change how we translate

### 5.1 There are no hard decode failures

`halt_baddata` = 0 across all 10,276 files. Every function decoded to valid
instructions. Any remaining problem is a **modelling** problem (types, CFG, ABI), not
a "bytes are not code" problem. This is the single most important quality result.

### 5.2 `unaff_r13` / `unaff_r2` are the SDA and TOC bases, not errors

`r13` points at the small-data area and `r2` at the TOC; both are established in the
CRT startup before `entry`. Ghidra does not model the values, so functions that touch
`.sdata`/`.sbss` render as:

```c
*(undefined4 *)(unaff_r13 + -0x4f48) = 1;   /* a global, address unknown */
```

This affects 2,333 functions (22.7%) and is **not** an instruction-decoding failure.
The practical cost is that global names and cross-references in `.sdata`/`.sbss`
cannot be recovered from the C alone. When translating such a function, keep the
offset and mark the access `// unknown:` unless the same offset can be tied to a
named `DAT_` elsewhere.

Same story for `unaff_r2` (TOC base): 2,128 functions.

### 5.3 Paired-single / GQR code is genuinely mis-rendered (728 files)

Ghidra renders GX paired-single operations as repeated bogus float code. A clear
example is the tail of `802a71d0`:

```c
if ((unaff_GQR0 & 0x3f000000) != 0) {
  ldexpf(-(bVar3 & 0x3f));
}
... (repeated six times) ...
FUN_802dcd8c();   /* register restore, see 5.5 */
```

`ldexpf` is not called by the game here; the pattern is Ghidra's fallback for the
quantized load/store. 728 files contain at least one `ldexpf(` and 730 contain
`unaff_GQR0` — effectively the same set (7.1% of the binary), concentrated in
0x8028–0x802A. **Do not translate float/GQR code from this C**; restrict translation
to scalar/integer paths or re-decompile with GQR values set on a future pass.

### 5.4 The 122 switch dispatchers are a small, bounded defect

Median size 28 bytes, 82 in game code. They are the `switch (id)` functions that
dispatch actor/scene states. The C is likely missing one or more cases. Since they
are small, they can be re-derived from the raw jump table in `main.elf` without
re-running Ghidra. Priority depends on whether the owning class is on the translation
path (e.g. `dScBoot`, `daPlBase`).

Largest of the 122 (best first targets if a switch must be exact):

| Address | Size | Band |
|---|---:|---|
| `802c2700` | 576 | middleware |
| `80130a70` | 172 | game |
| `80257740` | 128 | middleware |
| `800d69b0` | 116 | game |
| `80205f74` | 116 | SDK |
| `800d5680` | 112 | game |
| `800b53a0` | 92 | game |
| `800e4680` | 84 | game |
| `800636e0` | 80 | game |
| `80133e10` | 76 | game |

### 5.5 Most `unaff_r25…r31` comes from CodeWarrior register-save stubs

Raw ELF inspection at `0x802DCD40` shows a contiguous run of
`stw r14,-0x48(r11)` … `stw r31,-4(r11)` followed by `blr`. `0x802DCD8C` is the
matching `lwz … blr` restore run. These are the Metrowerks `__savegprlr_N` /
`__restgprlr_N` helpers, entered at different `N` labels that all share the tail:

| Range | Symbols | Evidence |
|---|---|---|
| `0x802DCD40`–`0x802DCD74` | `__savegprlr_14` … `__savegprlr_27` | `stw rN,-disp(r11)` chain, `blr` |
| `0x802DCD8C`–`0x802DCDC0` | `__restgprlr_14` … `__restgprlr_27` | `lwz rN,-disp(r11)` chain, `blr` |
| `0x802DCD00`–`0x802DCD3C` | `__restfpr_17` … `__restfpr_31` | `lfd fN,-disp(r11)` chain, `blr` |
| `0x802DCDD8`+ | 64-bit arithmetic helpers (inferred) | 32×32→64 and divide loops |

Ghidra split several of these into 4-byte "functions" because each `stw`/`lwz` is a
valid alternate entry point. That inflates the function count and produces most of the
`unaff_r25…r31` reports in callers. Exclude the range from translation scheduling.

### 5.6 Function-boundary noise inflates the corpus

95 functions are empty (`return;` only); 85 are `thunk_*`; 162 are exactly 4 bytes.
These are real linker/runtime artefacts, not game logic. They should be filtered out
of coverage metrics and out of the "next to translate" queue.

### 5.7 Unreachable-block pruning is concentrated in one band

773 files contain at least one "Removing unreachable block" comment (3,707 total).
The worst offenders cluster in the debug/exception band and in 0x8028–0x802A:

| Address | Size | Unreachable blocks |
|---|---:|---:|
| `802a71d0` | 2,748 | 144 |
| `802389f0` | 1,752 | 126 |
| `80292c90` | 5,464 | 106 |
| `80295c20` | 1,232 | 66 |
| `8028fea0` | 2,024 | 63 |
| `8028f860` | 1,600 | 53 |

For the 0x8028–0x802A entries the warnings co-occur with the `ldexpf`/GQR mis-render,
so the pruning is a symptom of the same modelling gap. `802389f0` (0x8023
debug/exception code) is a separate case; elsewhere (actor code) the comments are
ordinary switch cleanup.

---

## 6. Address-band map (evidence-based)

This map is used to label functions that have no strings of their own. "Evidence"
lists the strongest anchor found in that band.

| Range | Contents | Evidence |
|---|---|---|
| `0x80004000`–`0x80006780` | CRT / Metrowerks runtime | `entry`, memcpy/memset/cache routines, ROM-copy table |
| `0x80006780`–`0x80190000` | Game code (actors, stages, UI, scenes) | `*_c::StateID_*`, `g3d/*.brres`, `course/*.bin` |
| `0x80190000`–`0x801A0000` | Sound / ARC layer | `PLAYER_SE_*`, `wii_mj2d_sound.brsar`, `ARCInitHandle` |
| `0x801A0000`–`0x801C0000` | OS kernel | `OSLink`, `OSReset`, `OSExec`, `OSStateTM` |
| `0x801C0000`–`0x801D0000` | GX / CPUFifo + DVD/DI | `CPUFifo %08X`, `DVDLow*` |
| `0x801D0000`–`0x801E0000` | DVD / ISFS / NAND / DSP + wireless glue | `DVDLow*`, `ISFS error`, `NANDFileInfo`, `_shared2_` |
| `0x801E0000`–`0x80230000` | WPAD/HID + Bluetooth stack | `Nintendo RVL-CNT`, `L2CAP`, `RFCOMM`, `SDP` |
| `0x80230000`–`0x80240000` | Debug console / exception printer | `db_console.cpp`, `EXCEPTION OCCURRED`, `CONTEXT` |
| `0x80240000`–`0x80280000` | NW4R math / sound band (inferred) | `NodeTree` at `0x80250e70`; GQR math |
| `0x80280000`–`0x802B0000` | **NW4R G3D / GX render** | no strings; GQR + calls into `0x801C11xx` GX FIFO |
| `0x802B0000`–`0x802E0000` | EGG middleware | `EGGRootMEM1/2`, `EGG::CoreControllerMgr` |
| `0x802E0000`–`0x802E96E0` | C runtime (`str*`, printf core) | `strcmp`/`strcpy` idioms, printf loops |
| `0x802E96E0`–`0x802ED9E0` | MetroTRK debug monitor | `MetroTRK for Revolution`, `TRK_*` |

The 0x8028–0x802A label is an **inference** (position between NodeTree and EGG plus
GX/GQR call pattern), not a string hit. It is the least certain band in the export and
also the one with the most decompiler noise.

---

## 7. Top 50 largest functions

`Basis` says why the subsystem was assigned. "class strings" = the function itself
registers `<Class>::StateID_*` names; "band+callers" = address band plus distinctive
callees; "GQR/GX" = paired-single math and calls into the GX FIFO; "`0x… band`" = no
direct evidence, assignment is by address band only.

| # | Address | Size | Ghidra name | Probable subsystem | Basis |
|---:|---|---:|---|---|---|
| 1 | `800592c0` | 7,620 | `FUN_800592c0` | Player actor — `daPlBase_c` state table | class strings |
| 2 | `801769b0` | 7,420 | `FUN_801769b0` | UI/menu (file-select/option layout) | `P1_Def.brlyt`, `optnBtn` |
| 3 | `80299840` | 6,316 | `FUN_80299840` | NW4R G3D / GX render | GQR/GX; no strings |
| 4 | `800129a0` | 6,204 | `FUN_800129a0` | Course-select UI — `dCourseSelectGuide_c` | class strings |
| 5 | `802e624c` | 5,816 | `FUN_802e624c` | C runtime printf core | CRT band, char-output helper |
| 6 | `801e1e20` | 5,628 | `FUN_801e1e20` | HID / WPAD input | 0x801E band, 0x5f8 report buffer |
| 7 | `80292c90` | 5,464 | `FUN_80292c90` | NW4R G3D / GX render | 12 calls into `0x801C11xx` GX |
| 8 | `80213240` | 4,916 | `FUN_80213240` | Bluetooth L2CAP | `L2CAP_*` strings |
| 9 | `80289410` | 4,792 | `FUN_80289410` | NW4R G3D / GX render | GQR/GX |
| 10 | `800c75b0` | 4,772 | `FUN_800c75b0` | Line manager — `dLineMng_c` | class strings |
| 11 | `8015ddd0` | 4,716 | `FUN_8015ddd0` | Boot scene — `dScBoot_c` | class strings + REL load |
| 12 | `8017ba40` | 4,692 | `FUN_8017ba40` | UI/menu | `P1_Def.brlyt`, `optnBtn` |
| 13 | `80180780` | 4,484 | `FUN_80180780` | UI / layout render | 0x8018 band, GX calls |
| 14 | `80084690` | 4,328 | `FUN_80084690` | Stage / background collision | 0x8008 band, `course/*.bin` |
| 15 | `8028b6c0` | 4,280 | `FUN_8028b6c0` | NW4R G3D / GX render | GQR/GX |
| 16 | `801e39c0` | 4,224 | `FUN_801e39c0` | HID / WPAD input | 0x801E band, report buffer |
| 17 | `8028a6f0` | 4,012 | `FUN_8028a6f0` | NW4R G3D / GX render | GQR/GX |
| 18 | `80153400` | 3,832 | `FUN_80153400` | Yoshi actor — `daYoshi_c` | class strings |
| 19 | `8017a120` | 3,660 | `FUN_8017a120` | UI/menu | 0x8017 band, menu state |
| 20 | `8007ac40` | 3,632 | `FUN_8007ac40` | Stage actor | 0x8007 band |
| 21 | `8028cee0` | 3,468 | `FUN_8028cee0` | NW4R G3D / GX render | GQR/GX |
| 22 | `801befc0` | 3,448 | `FUN_801befc0` | OS runtime | 0x801B band, OS calls |
| 23 | `8029f900` | 3,392 | `FUN_8029f900` | NW4R G3D / GX render | GQR/GX |
| 24 | `802821e0` | 2,988 | `FUN_802821e0` | NW4R G3D / GX render | GQR/GX |
| 25 | `8025ca20` | 2,952 | `FUN_8025ca20` | NW4R sound / NodeTree band | 0x8025 band |
| 26 | `80191d20` | 2,808 | `FUN_80191d20` | Sound | 0x8019 band |
| 27 | `801e55c0` | 2,792 | `FUN_801e55c0` | WPAD calibration | `wpadCalibrationX`, `1G/0G` |
| 28 | `801f0b10` | 2,788 | `FUN_801f0b10` | WPAD/BTE link-key storage | `write_stored_link_key` |
| 29 | `8017af70` | 2,764 | `FUN_8017af70` | UI/menu | 0x8017 band |
| 30 | `802a71d0` | 2,748 | `FUN_802a71d0` | NW4R G3D / GX render | GQR/GX; worst warning file |
| 31 | `80215cec` | 2,672 | `FUN_80215cec` | Bluetooth L2CAP | `L2CAP_*` strings |
| 32 | `802405b0` | 2,660 | `FUN_802405b0` | NW4R math (GQR) | GQR0/GQR3 float math |
| 33 | `8021aaf4` | 2,652 | `FUN_8021aaf4` | Bluetooth RFCOMM | `rfc_mx_*` strings |
| 34 | `800f3e80` | 2,544 | `FUN_800f3e80` | Replay / demo control | `crossroad`, `demostop` |
| 35 | `800544f0` | 2,540 | `FUN_800544f0` | Player (0x8005 band) | band+callers |
| 36 | `801ce2e0` | 2,528 | `FUN_801ce2e0` | DVD / DI driver | 0x801CE, DVD calls |
| 37 | `801deb70` | 2,524 | `FUN_801deb70` | WPAD / wireless link | `WiFi uses channel`, `Reconnect` |
| 38 | `801a0e90` | 2,492 | `FUN_801a0e90` | OS kernel | 0x801A band |
| 39 | `8021b738` | 2,476 | `FUN_8021b738` | Bluetooth RFCOMM port | `Port_state_*` strings |
| 40 | `802e3998` | 2,444 | `FUN_802e3998` | C runtime formatted output | CRT band, `%`-format loop |
| 41 | `802c3d80` | 2,424 | `FUN_802c3d80` | EGG middleware | 0x802C band, EGG callees |
| 42 | `80178c50` | 2,388 | `FUN_80178c50` | UI/menu | 0x8017 band, layout strings |
| 43 | `80188df0` | 2,384 | `FUN_80188df0` | Layout / GX render | 0x8018 band, GX calls |
| 44 | `8023a860` | 2,380 | `FUN_8023a860` | Debug console / exception | 0x8023 band, `db_console` |
| 45 | `80191260` | 2,376 | `FUN_80191260` | Sound | 0x8019 band, sound handle |
| 46 | `8022c9c0` | 2,352 | `FUN_8022c9c0` | Bluetooth SDP | 0x8022 band |
| 47 | `8009dd60` | 2,316 | `FUN_8009dd60` | Boss — `dEnBossKoopaJrBase_c` | class strings |
| 48 | `801de010` | 2,296 | `FUN_801de010` | WPAD input math | 0x801DE, HID struct +0x8b8 |
| 49 | `801dfe00` | 2,292 | `FUN_801dfe00` | WPAD firmware check | `2_1_or_later` |
| 50 | `8007bef0` | 2,272 | `FUN_8007bef0` | Stage actor | 0x8007 band |

Rows 3, 9, 15, 17, 21, 23, 24, 30, 32 are all in the same float/GQR family; they are
**not** good translation candidates until the paired-single issue is addressed.

---

## 8. Recommended next 20 functions to translate

Ordering follows the plan's boot → runtime → resource priority: each item unblocks the
next stage, and none depends on the mis-rendered render band. "Depends on" lists what
must exist in Rust first.

| # | Address | Proposed name | Why now | Depends on | Confidence |
|---:|---|---|---|---|---|
| 1 | `80004050` | `entry` (already named) | The only entry point; shows CRT order and the hand-off to the game thread. | — | high |
| 2 | `80004250` | `init_data_sections` | Defines the ROM→RAM copy table; needed to model static data. | #3, #4 | high |
| 3 | `80004364` | `memcpy` | Used by 192 functions; foundational for every loader. | — | high |
| 4 | `800046b4` | `memset` | Used by 182 functions; pairs with #3. | — | high |
| 5 | `80004330` | `flush_cache` | Required after any code/data copy (REL loader, patches). | — | high |
| 6 | `801aa800` | `OSInit` | First SDK call from `entry`; establishes OS state used by everything else. | #3, #4 | high |
| 7 | `801aa550` | `os_print_boot_banner` | Confirms console revision/TV format; useful for runtime gating. | #6 | high |
| 8 | `801ac990` | `OSInitCache` | Cache setup must precede REL loading. | #6 | high |
| 9 | `802dd3c4` | `init_default_heap` | Creates the default heap before any allocation. | #3, #4 | high |
| 10 | `800c9f90` | `start_game_thread` | Shows the exact thread hand-off and stack size (0xF000). | #6 | high |
| 11 | `800c9f60` | `game_thread_main` | The game's real main; calls init then the forever loop. | #10 | high |
| 12 | `800c9f10` | `game_init` | First game-side init; calls `dSys` setup. | #11 | high |
| 13 | `800e48d0` | `dSys_init` | Creates `RootHeapMEM1/MEM2` and all engine heaps — the runtime's memory map. | #9, #12 | high |
| 14 | `800c9f40` | `game_main_loop` | The per-frame driver; defines update order for everything after. | #13 | high |
| 15 | `8015f7c0` | `dStateBase_ctor` | State-machine base: vtable + name + ID; needed for all actor state tables. | #3, #4 | high |
| 16 | `802dc780` | `dStateMng_addState` | Registration primitive used by every `*_initStates`. | #15 | medium |
| 17 | `8015ddd0` | `dScBoot_initStates` | Boot scene; also loads `d_enemies`/`d_en_boss` — the resource/REL entry point. | #15, #16, #18 | high |
| 18 | `8008f870` | `dDyl_ctor` | REL/dynamic-module object constructor; prerequisite for REL loading. | #3, #4 | medium |
| 19 | `800592c0` | `daPlBase_initStates` | Largest state table (37); validates the state-machine model on real gameplay. | #15, #16 | high |
| 20 | `802e1ab4` | `strcmp` | Used across the loader/UI; cheap, self-contained, high confidence. | — | high |

Why not the big 0x8028–0x802A functions yet: they are render code with the paired-single
mis-render, so translating their C would bake in wrong float semantics. Revisit after a
GQR-aware pass or raw-instruction review.

Secondary queue (once the above exists): `800e4e40` (`dSys_frame_update`), `8015f600`
(`dStateMng_initList`), `80016a80` / `80018330` (warning UI state tables),
`801b1960` (`OSLink`), `802e1938` (`strcpy`), `8010e830` (`dWarningManager`).

---

## 9. Method and reproducibility

- Scope: the Ghidra export in `work/ghidra/main-gb/export` (processor `Gekko_Broadway`,
  10,276 function files). This measures **how trustworthy the exported C is**, not
  whether the Rust port is complete. No code, tool, config, or export was modified.
- Input: `work/ghidra/main-gb/export/functions.tsv` + 10,276 `*.c`.
- Counts by exact substring/regex over the exported C; each class was verified by
  listing the matching files, not just counting lines.
- Raw instruction checks used `work/dol/main.elf` (ELF32 BE, program headers) to
  confirm the `__savegprlr`/`__restgprlr` family at `0x802DCD40`/`0x802DCD8C`.
- String addresses: `vaddr = file_offset + 0x80003F00` (verified against Ghidra's
  `s_<name>_<addr>` labels).
- Reproduce: grep the export for the literal patterns in §3; the warning texts are
  stable Ghidra strings.

### Limitations

- Counts are per exported file; a function split by Ghidra counts twice.
- `strings.txt` is a permissive printable-run scan, so a few "strings" are binary
  noise; only identifier-shaped hits are used in `SYMBOLS.md`.
- The 0x8028–0x802A subsystem label is an inference, not a string hit.
- No REL module was decompiled in this export; quality findings cover `main.dol` only.
