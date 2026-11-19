# Architecture

## Target

| Fact | Value |
|---|---|
| Game | SMNE01 — New SUPER MARIO BROS. Wii, build `r37675` |
| Executable | `sys/main.dol`, 3,491,808 bytes, entry `0x80004050` |
| Image | `.text` @ `0x80004000`; `.data` @ `0x802ED9E0`; BSS @ `0x80351680` |
| Modules | 4 LZ11-compressed RELs |
| Assets | 820 U8 archives, 2,933 members, 88 BRSTM streams |
| Libraries | Nintendo SDK + NW4R, CodeWarrior runtime |

## Memory map

| Range | Contents |
|---|---|
| `0x80000000`–`0x80004000` | OS low memory / boot info (`0x800000F4` holds a pointer) |
| `0x80004000`–`0x802ED9E0` | main.dol text (code + read-only data) |
| `0x802ED9E0`–`0x80351680` | main.dol data sections |
| `0x80351680`–`0x8042FC3C` | BSS (with two initialized data sections embedded) |
| `0x80500000` | canonical REL slot: `d_profileNP` (synthetic) |
| `0x80600000` | canonical REL slot: `d_basesNP` (synthetic) |
| `0x80A00000` | canonical REL slot: `d_enemiesNP` (synthetic) |
| `0x80C00000` | canonical REL slot: `d_en_bossNP` (synthetic) |

REL slots are analysis-only. `OSLink` does **not** choose the base: the caller
allocates the module image and passes it in. A linked module's base is
recoverable as `sectionInfoOffset - 0x4C`; canonical slots exist only so the
four ELF images do not overlap.

### Register baseline at entry

Set by `__init_registers` (`0x800041C0`, verified in the ELF bytes):

| Register | Value | Meaning |
|---|---|---|
| `r1` | `0x8043FC40` | initial stack pointer |
| `r2` | `0x80433080` | SDA2 base |
| `r13` | `0x8042F680` | SDA base (small data) |

`r13 ± offset` globals therefore have known absolute addresses. Example:
`0x80004030` writes `r13-0x4F48` = **`0x8042A738`**.


## Boot flow

```
DOL entry 0x80004050
  ├─ 0x800041c0             __init_registers (r1, r2, r13; clears GPRs)
  ├─ 0x80004300             SDK init calls (0x801ab150, 0x801a9d40, 0x801ac990)
  ├─ 0x80004250             init/database tables:
  │    ├─ 0x80004364        memcpy
  │    ├─ 0x80004330        data-cache store + icache invalidate
  │    └─ 0x800046b4        memset wrapper
  ├─ boot-info path         (0x800000F4; relocation table at +8, type at +0xC)
  ├─ 0x801bb930, 0x801aa800 OS/runtime init
  ├─ 0x80004000             fatal reset path (OSResetSystem is obsolete)
  ├─ 0x80004040             reads the r13-relative boot flag at 0x8042A738
  ├─ 0x800c9f90             creates the game-main thread (0x800c9f60), then parks the boot thread
  │    └─ 0x800c9f60        game main: dSys init once, then 0x800e4e40 forever
  └─ 0x801b89e0             teardown/exit path
```

Debug boot types 2/3/4 branch to MetroTRK (`0x802e9300` / `0x802e9398`) before
this hand-off; on a retail disc boot those paths do not run.


See `docs/BOOT_CHAIN.md` for the per-call evidence table.

## Data flow

```
disc image / extracted tree
   → FST (sys/fst.bin)
   → U8 .arc (820)
   → BRRES/BCSV/BLYT/TPL/BRSTM
   → resource loaders (Stage 4)
   → engine systems (scene, actor, render, audio, UI)
```

## Source layout

| Path | Role |
|---|---|
| `src/platform.rs` | big-endian volatile access, indirect calls, no_std |
| `src/boot/` | entry, CRT, memcpy/memset, cache, init tables |
| `src/runtime/` | `os`, `init`, `dvd`, `input`, `bt`, `ids`, `cpp`, `link` |
| `src/engine/` | `startup`, `main_loop`, `globals`, `display`, `frame`, `dsys` |
| `src/render/` | `gx`, `geometry`, `gx_band` (GX command/state) |
| `src/audio/` | `ax` (DSP/AX), `seq` (RSEQ) |
| `src/ui/` | `lyt` (NW4R layout/lists), `hud`, `menu` |
| `src/resource/` | `archive` (U8 reader), `course` (bgdat), `bres`, `bcsv` |
| `src/game/` | `actor`, `stage`, `player`, `path`, `worldmap`, `interaction`, `yoshi`, `boot_scene`, `scene`, `task`, `enemies`, `bosses`, `profile`, `bases`, `actors/dokan` |
| `src/subsystems/r*.rs` | extraction tombstones; no code should remain here |
| `work/ghidra/main-gb/export/` | evidence: Ghidra C + function index |
| `ledger/` | function status/confidence/evidence |

Range modules are a staging area. New code should go into a semantic module
when its role is known; range modules should shrink over time.

## Known unknowns

- `r13` **is** recovered (`0x8042F680`). Range-lane output that still contains
  `unaff_r13` placeholders must be rewritten against this base during promotion.
- Real REL base is chosen by the `dDyl` caller path, not by `OSLink`; the
  recovery formula `base = sectionInfoOffset - 0x4C` is verified, the caller
  allocation path is not yet translated.
- GX/TEV pipeline details and audio DSP semantics (port blockers, not
  decompilation blockers).
- Boot-type 2 failure path dereferences the return value `1` as a context
  pointer; dormant/broken in the shipped binary.
