# Engine startup — how the boot thread starts the game

Lane: engine.  Target: `sys/main.dol` of SMNE01 (r37675), PowerPC Broadway.

## In plain English

The DOL entry (`0x80004050`) never calls a game `main`. At the very end it calls
`0x800C9F90`, which does five things: it snapshots the 64-bit time base into a
global, fixes up the heap arena bounds, creates an `OSThread` whose entry point
is `0x800C9F60` on a 60 KiB stack taken out of its own frame, resumes that
thread, and then demotes itself to the lowest priority and suspends itself.

So **`0x800C9F90` is a thread hand-off, not the game main** (confidence: high
for the mechanics, medium-high for the naming).  The game main is `0x800C9F60`,
a 40-byte function that runs the `dSys` engine initialiser once and then loops
on `0x800E4E40`, one call per frame, forever.

`0x802E9300` and `0x802E9398` have nothing to do with the game loop.  They are
MetroTRK (Nintendo's target-resident debugger) boot entries: the DOL entry
branches to them only for debugger boot types 2, 3 and 4.  They save the whole
CPU state, ask `0x802E96E4` which devkit revision this is, and then hand the
machine to the debugger's own main loop `0x802E8E54`.  On a retail disc boot
neither function runs.

## Call chain at a glance

Every row was checked against the instruction stream in `work/dol/main.elf`;
the "Evidence" column is the Ghidra C file under
`work/ghidra/main-gb/export/`.

| # | Address | Name | Role | Status | Confidence | Evidence |
|---:|---|---|---|---|---|---|
| 1 | `0x80004050` | `entry` | DOL entry point (CRT): registers, data init, boot-info branch, then the two engine calls | boot lane | high | `80004050_entry.c` |
| 2 | `0x800041C0` | `__init_registers` | clears the GPRs and sets `r1 = 0x8043FC40`, `r2 = 0x80433080`, `r13 = 0x8042F680` | boot lane | high | `800041c0_FUN_800041c0.c` |
| 3 | `0x80004300` | — | OS/SDK init (`0x801AB150`, `0x801A9D40`, `0x801AC990`) | runtime lane | medium | `80004300_FUN_80004300.c` |
| 4 | `0x80004250` | `__init_data` | copy table `0x80006618` + BSS-zero table `0x8000669C` | boot lane | high | `80004250_FUN_80004250.c` |
| 5 | `0x802E9300` | `fn_802e9300` | MetroTRK boot, hardware id from `r5`, interrupts off | **partial** | high | `802e9300_FUN_802e9300.c` |
| 6 | `0x802E9398` | `fn_802e9398` | MetroTRK boot, hardware id 2, interrupts on | **partial** | high | `802e9398_FUN_802e9398.c` |
| 7 | `0x801B8970` | — | runtime bootstrap | runtime lane | low | `801b8970_FUN_801b8970.c` |
| 8 | `0x800C9F90` | `start_main_thread` | **creates the game thread, then parks the boot thread** | **translated** | high | `800c9f90_FUN_800c9f90.c` |
| 8a | `0x800C9F60` | `game_main` | thread entry = game main: `dSys` start-up, then frames forever | **translated** | high | `800c9f60_FUN_800c9f60.c` |
| 8b | `0x800C9F10` | `game_startup` | `0x800E48D0()` + `0x800E47A0(0)` | **translated** | high | `800c9f10_FUN_800c9f10.c` |
| 8c | `0x800C9F40` | `frame_loop` | `do { 0x800E4E40(); } while (true)` | **translated** | high | `800c9f40_FUN_800c9f40.c` |
| 9 | `0x801B89E0` | — | runtime teardown (ctor/dtor table at `0x802EDCC0`), tail call | runtime lane | low | `801b89e0_FUN_801b89e0.c` |

Rust: `src/engine/startup.rs` (rows 8, 8a, 8b, 8c) and `src/engine/main_loop.rs`
(rows 5, 6).  Rows 5 and 6 sit in `main_loop.rs` only because that file was
assigned to them; they are debugger code.

## How to read this file

1. §0 gives the register baseline; without `r13` every `r13±offset` global is
   an unresolved address.
2. §1 is the observed instruction chain through the entry point.
3. §2 is the main subject: arguments, the eight calls, the frame trick and the
   role conclusion.
4. §3 identifies the game main.
5. §4 explains the two MetroTRK boot entries and why they are not the loop.
6. §5–§8 are the claim/confidence table, the unknowns, lane ownership and
   cross-document notes.

Markers follow `docs/RUST_STYLE.md`: **observed** = read from the instructions,
**`// inferred:`** = naming/role from call sites, strings or structure,
**`// unknown:`** = unresolved.  "DOL", "REL", "MEM1", "r13", "GX", "AX",
"MetroTRK" are defined in `docs/GLOSSARY.md`; the first two are repeated inline
where they matter.

## 0. Register baseline and constants (why the magic numbers are what they are)

`0x800041C0` is the first call from the DOL entry. It clears `r0`, `r3-r12`,
`r14-r31` and loads the three segment bases — **observed**:

```text
0x80004234  lis r1,0x8043 / ori r1,r1,0xFC40     r1  (initial SP) = 0x8043FC40
0x8000423C  lis r2,0x8043 / ori r2,r2,0x3080     r2  (SDA2 base)  = 0x80433080
0x80004244  lis r13,0x8042 / ori r13,r13,0xF680  r13 (SDA base)   = 0x8042F680
```

`r13` is the PowerPC small-data base register; `r13 - 0x56E8` therefore means
the absolute address `0x80429F98`.  This resolves the `unaff_r13` placeholders
in the Ghidra output for every global in this document.

| Constant | Meaning |
|---|---|
| `0x8042F680` | `r13`, the small-data base, set by `0x800041C0` |
| `0x8043FC40` | initial stack pointer, set by `0x800041C0` |
| `0x56E8` / `0x56E4` | SDA displacement of the boot time-base snapshot (`0x80429F98` / `0x80429F9C`) |
| `0x800000E4` | OS global holding the current `OSThread*` (`0x801B4B30` is just this load) |
| `0x2D4` | `OSThread` field: base priority (`0x801B5F70` is `lwz r3,0x2D4(r3)`) |
| `0x1F` | lowest SDK thread priority (0 = highest, 31 = lowest) |
| `0xF000` | 60 KiB, the game main's stack size |
| `0x20` | padding between that stack array and the stack top handed to the OS |
| `0x80359938` | the `OSThread` object the game main runs on (BSS) |
| `0x80427240` | MetroTRK saved-GPR/CR block (BSS) |
| `0x804273E8` | MetroTRK saved-SPR block (BSS) |
| `0x80441C40` | stack top the MetroTRK boot entries install (above the image end `0x8042FC3C`) |
| `0xDEADBABE` | stack-overflow guard word written by `OSCreateThread` below the stack |
| `0x8000` | `MSR.EE`, the external-interrupt-enable bit |
| `0x806E0000` / `0x817F0000` | the arena window `0x800E51A0` guarantees (MEM1) |

## 1. Observed chain: DOL entry → the two subject areas

`0x80004050` is `e_entry`, 364 bytes, and it ends with a **tail call**
(`0x800041B8  b 0x801B89E0`, `LK = 0`), so it never returns through `blr`.

```text
0x80004050  bl 0x800041C0        __init_registers (r1/r2/r13 above, GPRs cleared)
0x80004054  bl 0x80004300        0x801AB150, 0x801A9D40, 0x801AC990
0x80004058  li r0,-1; stwu r1,-8; stw r0,4(r1); stw r0,0(r1)   (sentinel frame)
0x80004068  bl 0x80004250        copy table @0x80006618 + zero table @0x8000669C
0x80004078  *(u32*)0x80000044 = 0

            boot-type switch:
            r6 = *(u32*)0x800000F4            (pointer to the boot descriptor, BI2)
            r6 != 0                 -> r7 = *(u32*)(r6 + 0xC)      (boot type)
            r6 == 0 && *0x80000034  -> r7 = *(u32*)0x800030E8     (fallback type)
            r7 == 2                 -> r5 = 0, call 0x802E9300
            r7 == 3                 -> r5 = 1, call 0x802E9300
            r7 == 4                 -> r5 = 2, call 0x80004030     (sets a flag)
            else                    -> skip

            module relocation (BI2 + 0x8 = word offset to a list):
            n = list[0]; list[1..=n] += BI2;  r15 = align32(&list[1])
            *0x80000034 = *0x80003110 = r15;  r14 = n  (both 0 with no list)

0x8000416C  bl 0x801BB930        runtime lane
0x80004170  bl 0x801AA800        runtime lane
0x80004174  h = *(u16*)0x800030E6
            if ((h & 0x8000) == 0) || ((h & 0x7FFF) == 1) -> bl 0x80004000
            0x80004000: if ((*(u32*)0x800030E4 & 0xEEF) == 0xEEF) 0x801B3CC0(0,0,0)
0x80004198  bl 0x80004040        returns the flag set by 0x80004030
            if (ret == 1) -> bl 0x802E9398          <-- MetroTRK, hardware id 2
0x800041A8  bl 0x801B8970        runtime lane
0x800041AC  mr r3,r14 / mr r4,r15
0x800041B4  bl 0x800C9F90        <-- SUBJECT A (two arguments)
0x800041B8  b  0x801B89E0        runtime lane (ctor/dtor table @0x802EDCC0), tail call
```

**observed** (instruction stream).  `// inferred:` the field names: `0x800000F4`
is the pointer to the boot descriptor ("BI2"), `BI2+0x8` the module list and
`BI2+0xC` the boot type, `0x800030E8` the fallback boot type, `0x800030E4` /
`0x800030E6` OS boot flags.  The meanings of the boot types themselves are
`// unknown:`; only the branches are observed.

`0x801B3CC0` is a stub whose whole body is
`FUN_801ad4e0("OSReset.c", 0x4A1, "OSResetSystem() is obsoleted ...")` —
**observed**.  So `0x80004000` is a fatal "reset requested" path, not a normal
boot step; it is the same function the MetroTRK helper `0x802E9614` calls.

Consequence: boot types 2, 3 and 4 never reach `0x800C9F90`.  Type 4 goes to
`0x802E9398`, which tail-calls into the debugger and does not come back, so the
rest of the entry is skipped.  The game is only started on a normal boot.

## 2. Subject A — `0x800C9F90` (`start_main_thread`, 204 bytes)

### 2.1 Arguments

The entry passes `r3 = r14` (module count) and `r4 = r15` (32-byte-aligned
module-list pointer).  Both are `0` unless the boot descriptor carries a module
list.

**observed:** `0x800C9F90` never reads `r3`/`r4`.  The first instruction that
touches them is `bl 0x801B5F80`, which returns the time base in `r3:r4` and is
immediately followed by two stores of that result.  The function is effectively
**0-argument**; the two registers are pass-through leftovers.  (Ghidra even
decompiled the two stores as one 8-byte store of `FUN_801b5f80()`'s result —
that part of the C output is correct.)

### 2.2 Call sequence

| # | Address | Target | Arguments at the call | `// inferred:` role |
|---|---|---|---|---|
| 1 | `0x800C9FCC` | `0x801B5F80` | — | `OSGetTime()` → u64 in `r3:r4`; stored at `0x80429F98` (low word first in address order, so big-endian u64) |
| 2 | `0x800C9FD8` | `0x800E51A0` | — | arena bounds maintenance (MEM1 window) |
| 3 | `0x800C9FDC` | `0x801B4B30` | — | `OSGetCurrentThread()` = `*(u32*)0x800000E4` |
| 4 | `0x800C9FE4` | `0x801B5F70` | `r3` = current thread | `*(thread + 0x2D4)` = base priority |
| 5 | `0x800CA01C` | `0x801B5130` | `r3=&0x80359938`, `r4=0x800C9F60`, `r5=0`, `r6=stack_top`, `r7=0xF000`, `r8=prio`, `r9=0` | `OSCreateThread(thread, func, val, stack, stack_size, prio, attr)` |
| 6 | `0x800CA024` | `0x801B5860` | `r3=&0x80359938` | `OSResumeThread` |
| 7 | `0x800CA030` | `0x801B5E80` | `r3`=current thread, `r4=0x1F` | `OSSetThreadPriority(cur, 31)` |
| 8 | `0x800CA038` | `0x801B5B00` | `r3`=current thread | `OSSuspendThread(cur)` |

`0x801B5F80` (which Ghidra prints as an empty function) is **observed** to be
the standard stable 64-bit time-base read:

```text
0x801B5F80  mftb r3,TBU / mftb r4,TBL / mftb r5,TBU / cmp cr0,r3,r5 / bne -0x10 / blr
```

### 2.3 Why `0x801B5130` is `OSCreateThread`

The body validates the priority argument and fills the standard `OSThread` and
context layout — **observed**:

```text
0x801B5130  r31 = thread (r3); r26 = val (r5); r27 = stack (r6); r28 = size (r7)
0x801B5144  if (prio < 0 || prio > 31) return 0
0x801B5180  thread+0x2C8 = 1        (state)
0x801B5188  thread+0x2CA = attr&1
0x801B518C  thread+0x2D4 = prio     (base priority)
0x801B5190  thread+0x2D0 = prio     (priority)
0x801B5194  thread+0x2CC = 1        (suspend count)
0x801B5198  thread+0x2D8 = -1
0x801B51B0  *(stack-8) = 0 ; *(stack-4) = 0             (initial back chain / LR)
0x801B51C8  context+0x84 = 0x801B53A0                   (thread trampoline)
0x801B51D0  context+0x0C = val                          (gpr[3])
0x801B51D8  thread+0x304 = stack
0x801B51DC  thread+0x308 = stack - size
0x801B51E0  *(stack - size) = 0xDEADBABE                (overflow guard)
```

`0x801B53A0` is the trampoline: it loads `*(u32*)0x800000E4`, handles the
"detached" attribute bit and calls the thread function.

**The `stack` argument is the top of the area**, because the guard word is
written at `argument - size` and the initial frame words at `argument - 8` /
`argument - 4`.  The Rust translation follows that reading; the alternative
(bottom-of-area) reading would put the guard inside the caller's frame.

`0x801B5860` decrements `thread+0x2CC` and enqueues the thread on its priority
queue when the count reaches 0 (`OSResumeThread`).  `0x801B5B00` increments it,
runs the state machine and calls `0x801B4EA0` — a full reschedule that picks the
highest-priority ready thread and swaps contexts — when the "reschedule needed"
flag is set, i.e. `OSSuspendThread`.  Both are wrapped in `0x801B1140` /
`0x801B1180` (MSR external-interrupt off / restore).

### 2.4 The prologue, and where 60 KiB of stack comes from

```text
0x800C9F90  rlwinm r11,r1,0,27,31     r11 = r1 & 0x1F   (SP misalignment)
0x800C9F94  mr     r12,r1             r12 = caller SP
0x800C9F98  subfic r11,r11,-64        r11 = -64 - (r1 & 0x1F)
0x800C9F9C  stwux  r1,r1,r11          open a small frame: 32-byte aligned + 0x40 pad
0x800C9FA8  stw    r0,4(r12)          LR   -> caller SP + 4
0x800C9FB0  stw    r31,-4(r12)        r31  -> caller SP - 4
0x800C9FB8  stw    r30,-8(r12)
0x800C9FBC  stw    r29,-12(r12)
0x800C9FC4  stwux  r0,r1,-0xF020      second frame: 0xF020 = 0xF000 + 0x20
0x800CA03C  lwz    r10,0(r1)          epilogue: r10 = caller SP
0x800CA054  mr     r1,r10             r1 restored
0x800CA058  blr
```

The big frame **is** the game thread's stack: the top handed to
`OSCreateThread` is `r31 + 0x20` (`0x800C9FEC` / `0x800C9FF0` / `0x800CA000`,
with `lis r5,0x1` / `addi r7,r5,-0x1000` producing the `0xF000` size).  The
`0x20` is the pad between the array and the top.

The register saves at `caller SP + 4 / -4 / -8 / -12` are a CodeWarrior
prologue idiom; the epilogue reloads them from the same addresses, so they do
not overlap the thread stack.

### 2.5 Role conclusion

**`0x800C9F90` is not the game main.**  It is a *thread hand-off*: snapshot the
time base, fix the arena, create the game-main thread at the *calling* thread's
priority, resume it, then demote itself to priority 31 and suspend itself.
`0x800C9F60` (the thread entry) is the game main.

Confidence: **high** for the mechanics (every instruction is accounted for),
**medium-high** for the name (`start_main_thread` is a description, not a
recovered symbol — the image has no symbol table).

Evidence *against* reading `0x800C9F90` as `main`:

* it returns (`0x800CA058 blr`) instead of looping or exiting;
* it creates a thread and immediately suspends the caller — a `main` does not;
* its body contains none of the engine's start-up (`0x800E48D0`'s `dSys_`
  heaps, `0x800E4E40`'s frame) — those live in the *new* thread's entry;
* the DOL entry has a tail call (`b 0x801B89E0`) after it, i.e. the normal CRT
  "shutdown" tail, which only runs if the boot thread is resumed.

## 3. The thread entry — `0x800C9F60` (game main)

```text
0x800C9F60  stwu r1,-16(r1); mfspr r0,LR; stw r0,20(r1)
0x800C9F6C  bl 0x800C9F10
0x800C9F70  bl 0x800C9F40
0x800C9F74  lwz r0,20(r1); li r3,0; mtspr LR,r0; addi r1,r1,16; blr
```

* `0x800C9F10` — `FUN_800e48d0(); FUN_800e47a0(0);`
* `0x800C9F40` — `do { FUN_800e4e40(); } while (true);` (`0x800C9F50  b 0x800C9F4C`)

`0x800E48D0` is the engine start-up: it initialises an allocator object at
`r13-0x4838`, creates the two root heaps with the strings
`dSys_c::RootHeapMEM1` / `dSys_c::RootHeapMEM2`, sizes them for MEM1/MEM2,
brings up the subsystems (`0x800b…`, `0x800d…`, `0x8010…`, `0x8016…`, plus
`0x801ee150` for four units) and finishes with `0x800e4830(1)` /
`0x800e4850({0xFF})` / `0x80006790` — **observed**.

`0x800E4E40` is one **frame**: it reads `0x80109340()->+0x1D0`, calls the
render/audio/input/actor/pad steps and increments a frame counter at
`r13-0x5318` as its last statement — **observed**.

So the game's main loop is `0x800C9F40 → 0x800E4E40`; the frame body itself
belongs to the engine/subsystem lanes, not to this one.

`0x800E51A0` (call #2 above) is **observed** to be arena bound maintenance:

```text
hi = OSGetArenaHi()   (0x801ABE90, SDA -0x6180)
g[SDA-0x55C8] = 0x806E0000 ; if (hi < 0x806E0000) OSSetArenaHi(0x806E0000)   (0x801ABEF0)
lo = OSGetArenaLo()   (0x801ABE60, SDA -0x4FE8)
g[SDA-0x55C0] = 0x817F0000 ; if (lo > 0x817F0000) OSSetArenaLo(0x817F0000)   (0x801ABEC0)
g[SDA-0x55CC] = hi ; g[SDA-0x55C4] = lo
```

`// inferred:` it guarantees the arena covers at least
`[0x806E0000, 0x817F0000]` before `0x800E48D0` carves the MEM1/MEM2 root heaps
out of it.

## 4. Subject B — `0x802E9300` / `0x802E9398` (MetroTRK boot, not the game loop)

### 4.1 Why these are debugger code

Both functions sit in the MetroTRK cluster at the end of the text section
(`0x802E8E1C..0x802EA9E4`), whose strings — **observed** — are:

```text
MetroTRK for Revolution v0.4
MetroTRK : Sizeof Reply - %ld bytes
MetroTRK : Set to NDEV hardware
MetroTRK : Set to UNKNOWN hardware. (%ld)
MetroTRK : Invalid hardware ID passed from OS
MetroTRK : Defaulting to GDEV Hardware
MetroTRK - Event Queue full
```

`0x802E9CBC` prints the version banner; `0x802E8E90` is the message dispatch
loop; `0x802E9BEC` probes the transports (`0x802E9CC8`, `0x802E9A50`,
`0x802EA884`, `0x802E95CC`, `0x802E9820`, …).  MetroTRK is the target-resident
debugger that talks to a host over USB or serial; it is inert unless a debugger
boot was requested.

### 4.2 The save block

Both entry points write the CPU state to a static block before anything else:

| Offset | Content | Instruction |
|---:|---|---|
| `0x00..0x7C` | `r0..r31` (32 words) | `stmw r0,0(r3)` with `r3 = 0x80427240` |
| `0x04` | `r1` (SP at entry) | `stw r1,4(r3)` |
| `0x0C` | `r3` (first argument) | `stw r4,12(r3)` after restoring it from the red zone |
| `0x80` | `LR` | `mfspr r4,LR / stw r4,0x80(r3)` |
| `0x84` | `LR` again (the failure path reloads it into LR before `blr`) | `stw r4,0x84(r3)` |
| `0x88` | `CR` | `mfcr r4 / stw r4,0x88(r3)` |

`// unknown:` `0x8C..0x1A8` of the block are not written by these two functions.
The next written block starts at `0x804273E8` (SPRs), which `0x802EA4F8` fills
with `SR0..SR15`, `TBU/TBL`, `HID0/1`, `SRR1`, `PVR`, `IBAT0..3`, `DBAT0..3`,
the cache state, `SDR1`, `DAR`, `DSISR`, `SPRG0..3`, `DEC`, `IABR`, `EAR`,
`DABR`, `PMC1..4`, `SIA`, `MMCR0/1`, `ICTC`, `L2CR`, `SRR0`, `DC_DAT`,
`UMMCR0/1`, `UPMC1..4`, `USIA`, `HID2`, `WPAR`, `DMAU`, `DMAL`.

### 4.3 Control flow

```text
0x802E9300  save GPRs/CR to 0x80427240
            mfmsr r4; r3 = (r4 | 0x8000) ^ 0x8000; mtmsr r3    ; MSR.EE = 0
            mtspr SRR1,r4                                      ; SRR1 = entry MSR
            bl 0x802EA4F8                                      ; dump SPRs
            lmw r0,0(r3)                                       ; restore GPRs
            mtspr L2CR,0 ; mtspr spr1013,0                     ; L2 off, breakpoint off
            r1 = 0x80441C40                                    ; debugger stack top
            r3 = r5 (hardware id) ; bl 0x802E96E4
            if (ret == 1) -> lwz r4,0x84(r3) / mtspr LR,r4 / lmw / blr
            else          -> b 0x802E8E54                      ; debugger main loop
```

`0x802E9398` is the same with `mtmsr (MSR | 0x8000)` (interrupts stay enabled)
and `li r3,2` (GDEV).

`0x802E96E4` (`// inferred:` "select target hardware") prints
`"Devkit set to : %ld"` and `"MetroTRK : Sizeof Reply - 64 bytes"`, then:

* id 1 → `"MetroTRK : Set to NDEV hardware"` and installs eight handler pointers
  at `0x804256D0..0x804256F4`, returns 0;
* id 2 → no message, returns 0;
* anything else → `"Set to UNKNOWN hardware"` + `"Invalid hardware ID passed
  from OS"` + `"Defaulting to GDEV Hardware"`, returns 1.

`0x802E8E54` calls `0x802E9BEC`, and when that returns 0 prints the version
banner (`0x802E9CBC`) and enters `0x802E8E90`, which processes debugger messages
until message type 1 arrives.  It does not return to its caller's caller.

### 4.4 Boot-type mapping and consequence

| Boot type | Call | Hardware id | `MSR.EE` | Result |
|---:|---|---|---|---|
| 2 | `0x802E9300(r5 = 0)` | 0 | cleared | id 0 is *invalid*: "Defaulting to GDEV", returns 1 → the failure path executes `lwz r4,0x84(r3)` with `r3 == 1` |
| 3 | `0x802E9300(r5 = 1)` | 1 | cleared | NDEV handler table, then the debugger loop |
| 4 | `0x802E9398()` | 2 | kept | GDEV, then the debugger loop |

**observed** for the branch values.  `// unknown:` whether the boot-type-2
failure path is a genuine bug or unreachable in practice — it dereferences the
return value `1` as a context pointer.

**Conclusion:** `0x802E9300` / `0x802E9398` are debugger boot paths, not the
game loop and not "main" in any form.  `src/engine/main_loop.rs` is therefore a
documented placeholder plus a **partial** translation: the call sequence and the
`rc == 1` test are translated, the machine-state save/restore is documented
instruction by instruction and marked `// partial:`.

## 5. Observed vs inferred

| Claim | Status | Basis |
|---|---|---|
| `r13 = 0x8042F680`, `r1 = 0x8043FC40` at entry | observed | `0x800041C0` instructions |
| entry passes `(r14, r15)` = module count / list to `0x800C9F90` | observed | `0x800041AC..0x800041B4` |
| `0x800C9F90` ignores both arguments | observed | no `r3`/`r4` read before the time-base read |
| `0x801B5F80` = `OSGetTime` (u64 in `r3:r4`) | observed | `mftb` TBU/TBL/TBU loop |
| `0x801B5130` = `OSCreateThread` | observed (very strong) | priority range check, `OSThread` fields, `0xDEADBABE` guard, `0x801B53A0` trampoline |
| `0x801B5860` = `OSResumeThread`, `0x801B5B00` = `OSSuspendThread` | inferred (strong) | suspend-count direction + `0x801B4EA0` reschedule call |
| `0x800C9F60` = game main | inferred (strong) | `dSys_` heap strings + infinite `0x800E4E40` frame loop |
| `0x800C9F90` = main-thread hand-off | inferred (high) | the whole 204-byte body is accounted for |
| `0x802E9300` / `0x802E9398` = MetroTRK boot | inferred (very strong) | MetroTRK strings, handler table, tail call to `0x802E8E54` |
| meaning of the `0x800030E4/E6/E8` fields, and of boot types 2/3/4 | unknown | only raw accesses observed |

## 6. Unknowns / open items

1. **Symbol names.** The image has no symtab; every name here is descriptive.
   `start_main_thread`, `game_main`, `game_startup`, `frame_loop`,
   `fn_802e9300`, `fn_802e9398` are not recovered identifiers.
2. **Whether the boot thread is ever resumed.** `0x800C9F90` restores `r1` from
   the saved caller SP and returns, but the entry then tail-calls `0x801B89E0`.
   Whether that tail ever executes depends on code outside this lane
   (`0x801B8970`, `0x801B89E0`, `0x801B4EA0`).
3. **Boot-info semantics.** The meaning of the boot type values and of the
   `0x800030E4` / `0x800030E6` flags is not established; only the branches are.
4. **Hardware ids.** 0 and 2 behave as "GDEV / default", 1 as NDEV; the rest of
   the id space is only visible through the error strings.
5. **`0x802E9300` failure path** uses the return value `1` as a context pointer
   (`lwz r4,0x84(r3)`); flagged, not explained.
6. **`0x800E4E40` / `0x800E48D0` internals** are other lanes' scope.  This lane
   only records that they are the frame body and the `dSys` start-up.
7. **Stack geometry.** The Rust uses a local `[u8; 0xF000]` and reproduces the
   observed `frame_top + 0x20` top; the original's exact `0xF020` frame layout
   is documented but not reproduced byte for byte.

## 7. Lane ownership

* `0x801B8970`, `0x801B89E0` — runtime lane.  Referenced only through
  `platform::call0` and comments in this lane.
* `0x801B4B30`, `0x801B5130`, `0x801B5860`, `0x801B5E80`, `0x801B5B00`,
  `0x801B5F70`, `0x801B5F80` — OS/runtime; called, not translated here.
* `0x800E48D0`, `0x800E4E40`, `0x800E47A0`, `0x800E51A0` — engine subsystems;
  called, not translated here.
* `0x80004050`, `0x800041C0`, `0x80004250`, `0x80004000`, `0x80004030`,
  `0x80004040` — boot lane.

## 8. Cross-document notes (resolved)

* `r13` corrected in `docs/GLOSSARY.md`, `docs/ARCHITECTURE.md` and
  `docs/BOOT_CHAIN.md`: it is `0x8042F680`, set by `0x800041C0`.
* `0x800C9F90` corrected to "creates the game-main thread and parks the boot
  thread"; the game main is `0x800C9F60`.
* Boot types 2/3/4 confirmed as MetroTRK gating; a retail normal boot skips it.

