# Symbol mining report — `main.dol` (SMNE01, r37675)

## 1. Plain-English summary

We mined **14,761 text runs** from the executable and the four REL modules. Those runs
contain **509 distinct class-like names** (`…_c`), **2,544 state names** of the form
`Class::StateID_…`, **19 NW4R middleware type names** (`nw4hbm::…`), and a large
inventory of asset paths (`g3d/*.brres`, `*.brlyt`, `*.brlan`, course `.bin`, …).

The single most useful discovery is that the retail binary **keeps its C++ class names
in plain text** as state-registration labels. For example, the boot scene registers
`dScBoot_c::StateID_ResetWait`, `dScBoot_c::StateID_FadeOutWait`, and 22 more. That
gives a near-certain class name plus the exact function that owns the state table.
We used this to name 25 state-table functions, 21 runtime/CRT functions, 6 game-boot
functions, 3 state-machine primitives, 1 REL-linker constructor, and 3 compiler-runtime
stubs.

The main limitation: **only 74 of the 509 classes have strings in `main.dol`**, and
only 25 of those have a state-registration function in this export. The other 435
class names live in the REL modules (`d_basesNP` alone has 249), which have **not been
decompiled to Ghidra yet**. Those are excellent future names but cannot be attached to
addresses today.

Everything proposed in the CSV is backed by one of: a string xref, a vtable write, a
distinctive instruction sequence, or a repeated call pattern. Everything else is left
as `FUN_xxxxxxxx`.

---

## 2. How to read the evidence and confidence

### 2.1 Evidence types

| Evidence type | What it is | Strength | Example |
|---|---|---|---|
| **State-name xref** | The function writes a `Class::StateID_Name` string into a state object and registers it. | Very strong for the **class** and role. | `dScBoot_c::StateID_*` in `8015ddd0` |
| **RTTI string** | A C++ type name emitted by the compiler for `dynamic_cast`/typeinfo. | Very strong for the **type name**, not for a function. | `nw4hbm::lyt::Layout` |
| **Assert-variable string** | A `v(member::name)` literal used by an assert macro. | Strong for **member/class spelling**. | `v(mHeap::archiveHeap)` |
| **Vtable write** | The function stores a known vtable pointer at offset 0. | Strong for **constructor** role. | `8015f7c0` writes `&DAT_80328a18` |
| **Distinctive instruction pattern** | A loop/instruction sequence that only one library routine has. | Strong for **runtime helpers**. | `dcbst/sync/icbi/isync` = cache flush |
| **Repeated call pattern** | The same callee sequence appears at many call sites with the same argument shape. | Medium; role is inferred, name is descriptive. | `802dc780` list insert |
| **Address band + neighbours** | Nothing in the function itself; label comes from the surrounding module. | Low; used only in `QUALITY.md`, never in the CSV. | 0x8028–0x802A render band |

### 2.2 Confidence labels

| Label | Rule used here |
|---|---|
| `high` | The **class and role** are directly evidenced (state strings, vtable, unique instruction idiom). The spelling of a descriptive suffix (e.g. `_initStates`) is ours, but the identity is not in doubt. |
| `medium` | The role is inferred from structure/multiple call sites; a different name could be defensible. |
| `low` | Address-band guess only. **No CSV row uses `low`.** |

### 2.3 Naming conventions

- Keep the original `FUN_xxxxxxxx` when no evidence exists (per `docs/RUST_STYLE.md`).
- `Class_initStates` = the function that builds and registers that class's state table.
- `Class_ctor` = constructor-shaped function (vtable write + field init).
- Runtime helpers keep their conventional C/SDK name (`memcpy`, `strcmp`, `OSLink`).
- Address is always preserved in the CSV and in the Rust doc comment.

---

## 3. Where the names come from

| Source | Volume | What it yields | Caveat |
|---|---:|---|---|
| `work/dol/strings.txt` | 7,401 runs | 74 classes, NW4R RTTI, heap/assert names, asset paths | permissive printable-run scan; a few runs are binary noise |
| `d_basesNP.strings.txt` | 4,538 runs | 249 classes (actors, scenes, UI, items) | no Ghidra export yet |
| `d_enemiesNP.strings.txt` | 1,765 runs | 163 classes (enemies) | no Ghidra export yet |
| `d_en_bossNP.strings.txt` | 1,004 runs | 22 classes (bosses) | no Ghidra export yet |
| `d_profileNP.strings.txt` | 53 runs | 1 class (`dYesNoWindow_c`) + UI layout names | no Ghidra export yet |
| `work/arcs/index.json` | 2,933 members | asset extensions and directory families | names only; no code |
| Ghidra C export | 10,276 functions | function bodies, call sites, data refs | `main.dol` only |

String virtual addresses satisfy `vaddr = file_offset + 0x80003F00`; this was verified
against Ghidra's own `s_<name>_<address>` labels (e.g. file offset `0x003051EB` →
`0x803090EB`).

---

## 4. Namespaces and middleware

| Namespace / library | Names found | Meaning |
|---|---|---|
| `nw4hbm::lyt` | `AnimTransformBasic`, `AnimTransform`, `ArcResourceAccessor`, `ResourceAccessor`, `Bounding`, `Pane`, `detail::PaneBase`, `DrawInfo`, `Group`, `Layout`, `Material`, `Picture`, `TextBox`, `Window` (14) | NW4R layout/UI library (BRlyt runtime) |
| `nw4hbm::ut` | `ResFont`, `detail::ResFontBase`, `Font`, `TagProcessorBase<char>`, `TagProcessorBase<wchar_t>` (5) | NW4R font/text utilities (BRfnt runtime) |
| `d2d` | `ResAccMultLoader_c` | 2D resource-access loader |
| `m2d` | `m2d::create` | 2D middleware entry |
| `mHeap` | `gameHeaps[1]`, `gameHeaps[2]`, `archiveHeap`, `commandHeap`, `dylinkHeap`, `assertHeap` | engine heap singletons |
| `dDyl` | `cCc_frmHeap` | dynamic-module (REL) linker |
| `dBase_c` | `(dBase_c` (truncated assert fragment) | engine base class for scenes/actors |
| `fBase_c` | `mHeap` | base framework class |
| `EGG` | `EGGRootMEM1`, `EGGRootMEM2`, `EGG::CoreControllerMgr` | EGG middleware (memory + controllers) |
| `MetroTRK` | `MetroTRK for Revolution v0.4`, `TRK_*` | Metrowerks debug monitor |

`nw4hbm` is the internal namespace of the NW4R libraries used by NSMBW; seeing these
strings is direct proof that the layout (`lyt`) and font (`ut`) libraries are linked
into `main.dol`, even though their code has no assert strings of its own.

---

## 5. Class-name inventory

### 5.1 Distribution by module

| Module | Unique `*_c` names | State labels | Notes |
|---|---:|---:|---|
| `main.dol` | 74 | 487 | engine, boot, player base, UI warnings |
| `d_basesNP` | 249 | 1,077 | the bulk of gameplay: actors, scenes, modes |
| `d_enemiesNP` | 163 | 689 | enemy actors |
| `d_en_bossNP` | 22 | 283 | boss actors |
| `d_profileNP` | 1 | 8 | `dYesNoWindow_c` (profile/UI) |
| **Total** | **509 unique** | **2,544** | state labels counted across all modules |

The important consequence: **most game logic is in the RELs, not in `main.dol`.**
`main.dol` holds the engine, the CRT, the SDK, and a subset of boot/UI/player code.

### 5.2 Largest classes by number of state labels

All counts are `Class::StateID_*` strings. "Main code" = a registration function for
that class exists in the current `main.dol` export.

| # | Class | States | Strings in | Main code? |
|---:|---|---:|---|---|
| 1 | `dAcPy_c` | 49 | main strings | no — registrant is in a REL/unexported |
| 2 | `dFileSelect_c` | 44 | d_basesNP | no |
| 3 | `daPlBase_c` | 37 | main | **yes** |
| 4 | `dCourseSelectManager_c` | 35 | d_basesNP | no |
| 5 | `dCourseSelectGuide_c` | 32 | main | **yes** |
| 6 | `dEnTorideKokoopa_c` | 28 | main strings | no |
| 7 | `dLineMng_c` | 25 | main | **yes** |
| 8 | `dScBoot_c` | 24 | main | **yes** |
| 9 | `daEnBossKameck_c` | 24 | d_en_bossNP | no |
| 10 | `daEnBossLemmy_c` | 22 | d_en_bossNP | no |
| 11 | `daEnBossLudwig_c` | 22 | d_en_bossNP | no |
| 12 | `daEnGesso_c` | 21 | d_enemiesNP | no |
| 13 | `daEnItem_c` | 21 | d_enemiesNP | no |
| 14 | `dCharacterChangeSelectBase_c` | 20 | d_basesNP | no |
| 15 | `dPointResultMulti_c` | 20 | d_basesNP | no |
| 16 | `daEnBossCastleRoy_c` | 20 | d_en_bossNP | no |
| 17 | `daIce_c` | 19 | main strings | no |
| 18 | `daYoshi_c` | 19 | main | **yes** |
| 19 | `dModelPlayManager_c` | 19 | d_basesNP | no |
| 20 | `dMultiCourseSelect_c` | 18 | d_basesNP | no |

### 5.3 Classes with a state-table function in `main.dol`

These 25 are the highest-value names in the export. All are `high` confidence.

| Address | Class | States | Notes |
|---|---|---:|---|
| `8015ddd0` | `dScBoot_c` | 24 | also loads `d_enemies` / `d_en_boss` RELs |
| `800592c0` | `daPlBase_c` | 37 | player base |
| `800129a0` | `dCourseSelectGuide_c` | 32 | course-select UI |
| `80153400` | `daYoshi_c` | 19 | Yoshi |
| `800c75b0` | `dLineMng_c` | 25 | line/rail manager |
| `8009dd60` | `dEnBossKoopaJrBase_c` | 11 | boss base |
| `8010e830` | `dWarningManager_c` | 8 | warning UI manager |
| `8010b890` | `dGoalManager_c` | 8 | goal manager |
| `8009a320` | `dEnBoss_c` | 7 | boss base |
| `80037750` | `daEnObjCoinBlock_c` | 7 | coin block object |
| `8003c050` | `daEnShell_c` | 7 | shell enemy |
| `800ba5e0` | `dIggyWanKusari_c` | 6 | Iggy chain |
| `800ea060` | `dWanKusari_c` | 6 | chain |
| `8015b320` | `Pausewindow_c` | 6 | pause window |
| `80016a80` | `dWarningBattery_c` | 5 | battery warning |
| `800414e0` | `daFireBall_Base_c` | 4 | fireball base |
| `800b28f0` | `dfukidashiInfo_c` | 4 | speech bubble |
| `800d7c80` | `dPosGoAndComeExeFrm_c` | 4 | moving platform frame |
| `800de510` | `dReplayPlay_c` | 4 | replay playback |
| `80159f10` | `dGameDisplay_c` | 4 | game HUD |
| `80019560` | `dWarningNunchuk_c` | 4 | nunchuk warning |
| `8001a280` | `dWarningOther_c` | 4 | generic warning |
| `8001af70` | `dWarningYoKo_c` | 4 | sideways-remote warning |
| `80018330` | `dWarningErrorInfo_c` | 11 | error-info warning |
| `80067830` | `dActorMultiState_c` | 2 | multi-state actor adapter |

### 5.4 Naming families (use these when reading REL strings)

Meanings below are **inferred from the prefix and surrounding strings**, not from a
header file; the class spellings themselves are literal.

| Prefix | Meaning | Examples |
|---|---|---|
| `dSc*_c` | scene / game mode | `dScBoot_c`, `dScGameSetup_c`, `dScCrsin_c`, `dScWorld9DeMo_c` |
| `dEn*_c` / `daEn*_c` | enemy actor | `dEnBoss_c`, `daEnGesso_c`, `daEnCoin_c`, `daEnPuku_c` |
| `daEnBoss*_c` | boss actor | `daEnBossLemmy_c`, `daEnBossCastleIggy_c` |
| `daPl*` / `dAcPy_c` | player | `daPlBase_c`, `daYoshi_c`, `da2DPlayer_c` |
| `dWm*` / `dW*` | world map | `dWorldSelect_c` |
| `dFile*`, `dCourse*`, `dMiniGame*` | menus / modes | `dFileSelect_c`, `dCourseSelectManager_c`, `dMiniGameWire_c` |
| `dWarning*` | system-warning UI | `dWarningBattery_c`, `dWarningNunchuk_c` |
| `dBg*`, `dStage*`, `dCourse*` | stage/background | `dBgTexMng_c` |
| `dSys*`, `dBase*`, `dState*`, `dActor*` | engine core | `dSys_c`, `dActorMultiState_c` |
| `dReplay*`, `dDemo*` | replay/demo | `dReplayPlay_c` |

---

## 6. Asset and file-name families

Archive member extensions (from `work/arcs/index.json`, 820 archives / 2,933 members):

| Extension | Members | Meaning |
|---|---:|---|
| `.bin` | 926 | course/background payloads, generic data |
| `.tpl` | 713 | GameCube/Wii textures |
| `.brres` | 518 | NW4R resource bundle (model/texture/animation) |
| `.brlan` | 448 | NW4R layout animation |
| `.brlyt` | 94 | NW4R layout definition |
| `.lz` | 67 | LZ10/LZ11-compressed payload |
| `.blight` / `.blmap` | 36 / 36 | stage lighting / lightmap |
| `.bfog` | 26 | stage fog |
| `.mid` | 22 | MIDI sequence |
| `.csv` | 21 | BCSV binary tables (inside arcs) |
| `.bti` | 10 | texture image |
| `.bwav` | 5 | wave audio |
| `.bmg` | 3 | message archive |
| `.bdof` | 3 | depth-of-field |
| `.brfnt` | 3 | bitmap font |
| `.pcm`, `.wt` | 1, 1 | raw audio / wave table |

Path families seen as literals:

| Pattern | Example | Subsystem |
|---|---|---|
| `g3d/<name>.brres` | `g3d/kuribo.brres`, `g3d/obj_dokan.brres` | actor models |
| `Layout/<name>/<name>.arc` | `Layout/pointGet/pointGet.arc` | UI layouts |
| `course/course%d_bgdatL0.bin` | — | stage collision/background |
| `BG_chk/d_bgchk_%s.bin` | — | background collision check |
| `/US/HomeButton2/homeBtn%.*s.arc` | — | HomeButton (region-specific) |
| `%s/%sNP.rel` | — | REL module loader |
| `Sound/wii_mj2d_sound.brsar` | — | sound archive |

Note: `.bcsv` never appears as a literal extension in code; the files are stored as
`.csv` inside archives and parsed as BCSV by the resource layer.

---

## 7. Runtime / compiler symbols recovered

These were recovered from instruction idioms rather than strings, and are safe to name
in the Rust port.

| Address | Proposed name | Evidence | Confidence |
|---|---|---|---|
| `80004364` | `memcpy` | alignment prologue + 4/8-byte copy loop; 192 callers | high |
| `800046b4` | `memset` | returns dest; 182 callers; wraps the word-fill at `80004600` | high |
| `80004600` | `__fill_mem` | repeated-word fill core; sole caller `800046b4` | high |
| `80004330` | `flush_cache` | `dcbst; sync; icbi; isync` per 8-byte line | high |
| `802e1938` | `strcpy` | byte copy to NUL, `0x80808080` word test | high |
| `802e19f8` | `strncpy` | n-byte copy then zero padding | high |
| `802e1a3c` | `strcat` | scan to NUL then append | high |
| `802e1a68` | `strncat` | scan to NUL then bounded append | high |
| `802e1ab4` | `strcmp` | `0x80808080` difference loop | high |
| `802dcd40` | `__savegprlr_14` | `stw r14,-0x48(r11) … stw r31,-4(r11); blr` | high |
| `802dcd8c` | `__restgprlr_14` | `lwz r14,-0x48(r11) … lwz r31,-4(r11); blr` | high |
| `802dcd18` | `__restfpr_23` | `lfd f23,-0x48(r11) … lfd f31,-8(r11); blr` | high |

The `0x802DCD40`–`0x802DCDC0` block contains the full `__savegprlr_N` /
`__restgprlr_N` family for `N = 14…31`; each `N` is a valid alternate entry point that
shares the tail. The `0x802DCDD8`–`0x802DCFFC` block is the 64-bit multiply/divide
runtime (`__mulh`, `__div2u`, …) — not named in the CSV because the exact variant
boundaries are not proven.

---

## 8. Proposed names for high-value functions

All 59 rows below are also in `ledger/agents/proposed_names.csv` (same order, same
fields). `Current name` is the Ghidra symbol; `Evidence` is the concrete fact, not a
guess.

| Module | Address | Current name | Proposed name | Evidence | Confidence |
|---|---|---|---|---|---|
| main.dol | `80004250` | `FUN_80004250` | `init_data_sections` | Entry calls it; walks PTR_FUN_80006618 table of {src,dst,len}, copies then zeroes a second table (BSS). | high |
| main.dol | `80004330` | `FUN_80004330` | `flush_cache` | Loop of dcbst / sync / icbi / isync over 8-byte lines, called immediately after the ROM copy. | high |
| main.dol | `80004364` | `FUN_80004364` | `memcpy` | Forward/backward block copy with alignment prologue; 192 call sites; textbook memcpy shape. | high |
| main.dol | `80004600` | `FUN_80004600` | `__fill_mem` | Word-fill of a repeated byte; sole caller is 800046b4. | high |
| main.dol | `800046b4` | `FUN_800046b4` | `memset` | Returns dest; 182 call sites; wraps 80004600. | high |
| main.dol | `802dd3c4` | `FUN_802dd3c4` | `init_default_heap` | String "GCN_Mem_Alloc.c : InitDefaultHeap. No Heap Available"; allocates the default heap from the OS arena. | high |
| main.dol | `802e1938` | `FUN_802e1938` | `strcpy` | Byte copy to NUL with 0x80808080 word test. | high |
| main.dol | `802e19f8` | `FUN_802e19f8` | `strncpy` | Copies at most n bytes then zero-pads the remainder. | high |
| main.dol | `802e1a3c` | `FUN_802e1a3c` | `strcat` | Scans dest to NUL then appends src. | high |
| main.dol | `802e1a68` | `FUN_802e1a68` | `strncat` | Scans dest to NUL then appends at most n bytes. | high |
| main.dol | `802e1ab4` | `FUN_802e1ab4` | `strcmp` | Word-wise 0x80808080 compare, returns byte difference. | high |
| main.dol | `801aa800` | `FUN_801aa800` | `OSInit` | Called directly from entry; initialises OS tables/registers and calls 801aa550 (boot banner). | high |
| main.dol | `801aa550` | `FUN_801aa550` | `os_print_boot_banner` | Prints "Revolution OS", "Kernel built : %s %s", "Console Type :" and console revisions. | high |
| main.dol | `801ac990` | `FUN_801ac990` | `OSInitCache` | Strings "L1 i caches initialized", "L1 d caches initialized", "L2 cache initialized"; HID0/HID2 cache ops. | high |
| main.dol | `801b1950` | `FUN_801b1950` | `OSSetStringTable` | Stores the string-table pointer at `0x800030D0`. | high |
| main.dol | `801b1960` | `FUN_801b1960` | `Relocate` | Static helper: scans one importer's import table and applies a provider's relocations. **Not** `OSLink`. | high |
| main.dol | `801b1c30` | `FUN_801b1c30` | `Link` | Static: validates module, inserts it, fixes headers/tables, runs relocation passes, zeroes BSS. | high |
| main.dol | `801b1f20` | `FUN_801b1f20` | `OSLink` | Public entry: `li r5,0; b Link`. Caller supplies the module base. | high |
| main.dol | `801b1f30` | `FUN_801b1f30` | `OSLinkFixed` | Public entry: version-3 check, `li r5,1; b Link`. | high |
| main.dol | `801b1f50` | `FUN_801b1f50` | `Undo` | Static inverse relocation pass. **Not** `OSUnlink`. | high |
| main.dol | `801b2190` | `FUN_801b2190` | `OSUnlink` | Public: dequeue + `Undo` + header restore. | high |
| main.dol | `801b2370` | `FUN_801b2370` | `__OSModuleInit` | Clears the module list head/tail and string table. | high |
| main.dol | `801b3590` | `FUN_801b3590` | `__OSHotReset` | Strings "OSReset.c" and "__OSHotReset(): Falied to reset system." | high |
| main.dol | `801b3b00` | `FUN_801b3b00` | `OSReturnToMenu` | String "OSReturnToMenu(): Falied to boot system menu." | high |
| main.dol | `801b3b40` | `FUN_801b3b40` | `OSReturnToDataManager` | String "OSReturnToDataManager(): Falied to boot system menu." | high |
| main.dol | `801b3cc0` | `FUN_801b3cc0` | `OSResetSystem` | String "OSResetSystem() is obsoleted..."; called from 80004000. | high |
| main.dol | `801bb840` | `FUN_801bb840` | `SISetSamplingRate` | String "SISetSamplingRate: unknown TV format". | high |
| main.dol | `800c9f90` | `FUN_800c9f90` | `start_game_thread` | Last call in entry; OSCreateThread(&DAT_80359938, 800c9f60, 0, stack, 0xf000) then OSResumeThread. | high |
| main.dol | `800c9f60` | `FUN_800c9f60` | `game_thread_main` | Thread entry created by 800c9f90; calls 800c9f10 (init) then 800c9f40 (infinite loop). | high |
| main.dol | `800c9f40` | `FUN_800c9f40` | `game_main_loop` | do { FUN_800e4e40(); } while (true); no exit path. | high |
| main.dol | `800c9f10` | `FUN_800c9f10` | `game_init` | Calls 800e48d0 (dSys root heaps) then 800e47a0(0). | high |
| main.dol | `800e48d0` | `FUN_800e48d0` | `dSys_init` | Allocates dSys_c::RootHeapMEM1/MEM2 by name and wires heap groups + subsystems. | high |
| main.dol | `800e4e40` | `FUN_800e4e40` | `dSys_frame_update` | Per-frame update: input, scene, sound, render and pad update calls in fixed order. | medium |
| main.dol | `8015f7c0` | `FUN_8015f7c0` | `dStateBase_ctor` | Writes vtable &DAT_80328a18, stores name pointer, assigns incrementing state ID. | high |
| main.dol | `802dc780` | `FUN_802dc780` | `dStateMng_addState` | Intrusive list insert at the sdata list head; called once per registered state. | medium |
| main.dol | `8015f600` | `FUN_8015f600` | `dStateMng_initList` | Stores {array pointer, count, 0} into a 3-word header before bulk state registration. | medium |
| main.dol | `8008f870` | `FUN_8008f870` | `dDyl_ctor` | Writes vtable &DAT_80310e98 and calls 80160080; called with REL names ("d_enemies", "d_en_boss"). | medium |
| main.dol | `8015ddd0` | `FUN_8015ddd0` | `dScBoot_initStates` | Registers 24 dScBoot_c::StateID_*; also loads the d_enemies and d_en_boss RELs. | high |
| main.dol | `800592c0` | `FUN_800592c0` | `daPlBase_initStates` | Registers 37 daPlBase_c::StateID_* strings through 8015f7c0 + 802dc780. | high |
| main.dol | `800129a0` | `FUN_800129a0` | `dCourseSelectGuide_initStates` | Registers 32 dCourseSelectGuide_c::StateID_* strings. | high |
| main.dol | `80153400` | `FUN_80153400` | `daYoshi_initStates` | Registers 19 daYoshi_c::StateID_* strings. | high |
| main.dol | `800c75b0` | `FUN_800c75b0` | `dLineMng_initStates` | Registers 25 dLineMng_c::StateID_* strings. | high |
| main.dol | `8009dd60` | `FUN_8009dd60` | `dEnBossKoopaJrBase_initStates` | Registers 11 dEnBossKoopaJrBase_c::StateID_* strings. | high |
| main.dol | `8010e830` | `FUN_8010e830` | `dWarningManager_initStates` | Registers 8 dWarningManager_c::StateID_* strings. | high |
| main.dol | `8010b890` | `FUN_8010b890` | `dGoalManager_initStates` | Registers 8 dGoalManager_c::StateID_* strings. | high |
| main.dol | `8009a320` | `FUN_8009a320` | `dEnBoss_initStates` | Registers 7 dEnBoss_c::StateID_* strings. | high |
| main.dol | `80037750` | `FUN_80037750` | `daEnObjCoinBlock_initStates` | Registers 7 daEnObjCoinBlock_c::StateID_* strings. | high |
| main.dol | `8003c050` | `FUN_8003c050` | `daEnShell_initStates` | Registers 7 daEnShell_c::StateID_* strings. | high |
| main.dol | `800ba5e0` | `FUN_800ba5e0` | `dIggyWanKusari_initStates` | Registers 6 dIggyWanKusari_c::StateID_* strings. | high |
| main.dol | `800ea060` | `FUN_800ea060` | `dWanKusari_initStates` | Registers 6 dWanKusari_c::StateID_* strings. | high |
| main.dol | `8015b320` | `FUN_8015b320` | `Pausewindow_initStates` | Registers 6 Pausewindow_c::StateID_* strings. | high |
| main.dol | `80016a80` | `FUN_80016a80` | `dWarningBattery_initStates` | Registers 5 dWarningBattery_c::StateID_* strings. | high |
| main.dol | `800414e0` | `FUN_800414e0` | `daFireBall_Base_initStates` | Registers 4 daFireBall_Base_c::StateID_* strings. | high |
| main.dol | `800b28f0` | `FUN_800b28f0` | `dfukidashiInfo_initStates` | Registers 4 dfukidashiInfo_c::StateID_* strings. | high |
| main.dol | `800d7c80` | `FUN_800d7c80` | `dPosGoAndComeExeFrm_initStates` | Registers 4 dPosGoAndComeExeFrm_c::StateID_* strings. | high |
| main.dol | `800de510` | `FUN_800de510` | `dReplayPlay_initStates` | Registers 4 dReplayPlay_c::StateID_* strings. | high |
| main.dol | `80159f10` | `FUN_80159f10` | `dGameDisplay_initStates` | Registers 4 dGameDisplay_c::StateID_* strings. | high |
| main.dol | `80019560` | `FUN_80019560` | `dWarningNunchuk_initStates` | Registers 4 dWarningNunchuk_c::StateID_* strings. | high |
| main.dol | `8001a280` | `FUN_8001a280` | `dWarningOther_initStates` | Registers 4 dWarningOther_c::StateID_* strings. | high |
| main.dol | `8001af70` | `FUN_8001af70` | `dWarningYoKo_initStates` | Registers 4 dWarningYoKo_c::StateID_* strings. | high |
| main.dol | `80067830` | `FUN_80067830` | `dActorMultiState_initStates` | Registers 2 dActorMultiState_c::StateID_* strings. | high |
| main.dol | `80018330` | `FUN_80018330` | `dWarningErrorInfo_initStates` | Registers 11 dWarningErrorInfo_c::StateID_* strings. | high |
| main.dol | `802dcd40` | `FUN_802dcd40` | `__savegprlr_14` | Raw ELF: stw r14,-0x48(r11) ... stw r31,-4(r11); blr. CodeWarrior register-save helper; 18 entry points share the tail. | high |
| main.dol | `802dcd8c` | `FUN_802dcd8c` | `__restgprlr_14` | Raw ELF: lwz r14,-0x48(r11) ... lwz r31,-4(r11); blr. Matching register-restore helper. | high |
| main.dol | `802dcd18` | `FUN_802dcd18` | `__restfpr_23` | Raw ELF: lfd f23,-0x48(r11) ... lfd f31,-8(r11); blr. Float-register restore helper. | high |

---

## 9. What is deliberately left unnamed

| Item | Why no proposal |
|---|---|
| ~10,217 of the 10,276 `main.dol` functions | No string, vtable, or distinctive pattern. A name would be invented. |
| All REL-module functions | No Ghidra export exists for `d_basesNP`/`d_enemiesNP`/`d_en_bossNP`/`d_profileNP` yet. The class names are known from strings, but there are no addresses to attach. |
| The 0x8028–0x802A render band | Paired-single/GQR mis-render (see `QUALITY.md` §5.3); role is not provable from this C. |
| `dAcPy_c`, `dFileSelect_c`, `dCourseSelectManager_c`, `dEnTorideKokoopa_c` | High-confidence class names, but their state-table functions are outside this export. |
| `0x802DCDD8`+ 64-bit helpers | Function family is clear, but exact variant names (`__div2u` vs `__div2i`) are not proven. |
| Asset names | They name files, not functions; catalogued in §6 instead. |

---

## 10. Method notes and next steps

- Sources: `work/dol/strings.txt`, `work/rels/*.strings.txt`, `work/arcs/index.json`,
  and the Ghidra C export. The machine-readable companion is
  `ledger/agents/proposed_names.csv` (59 rows). No code, tool, config, or export was
  modified.
- All 509 class names were extracted with `\b[A-Za-z_][A-Za-z0-9_]*_c\b` from the five
  `*.strings.txt` files; 2,544 state labels with `\b(Class_c)::StateID_`.
- Function/class association was established by reading the exported C: a function is
  credited with a class only if its body references that class's `StateID_*` strings.
- String addresses were mapped with `vaddr = file_offset + 0x80003F00` and cross-checked
  against Ghidra's `s_*_<address>` symbol suffixes.
- Next step that would multiply the value of this work: run the same pipeline on the
  four REL modules. `d_basesNP` alone holds 249 class names and the majority of
  gameplay code; once its functions have addresses, the same state-label method yields
  hundreds of high-confidence function names.
