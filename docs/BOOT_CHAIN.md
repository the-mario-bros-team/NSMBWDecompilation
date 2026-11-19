# Boot chain

Entry: `0x80004050` (364 bytes). All addresses are virtual; evidence paths are
under `work/ghidra/main-gb/export/`.

| Call | Evidence | Observed role | Confidence |
|---|---|---|---|
| `0x800041c0` | `800041c0_FUN_800041c0.c` | `__init_registers`: clears GPRs, sets `r1=0x8043FC40`, `r2=0x80433080`, `r13=0x8042F680` | high (ELF bytes) |
| `0x80004300` | `80004300_FUN_80004300.c` | calls `0x801ab150`, `0x801a9d40`, `0x801ac990`; SDK/OS init stage | medium |
| `0x80004250` | `80004250_FUN_80004250.c` | walks table `0x80006618` (src/dst/len triples) and `0x8000669c` (dst/len pairs) | high |
| `0x80004364` | `80004364_FUN_80004364.c` | `memcpy` with cache-block touch; forward/backward paths | high |
| `0x80004330` | `80004330_FUN_80004330.c` | `dcbst`/`sync`/`icbi` range flush | high |
| `0x80004600` | `80004600_FUN_80004600.c` | `memset` (byte fill, 32-byte unroll) | high |
| `0x800046b4` | `800046b4_FUN_800046b4.c` | wrapper around `memset` | medium |
| `0x801bb930` | runtime lane | OS/runtime init | low |
| `0x801aa800` | runtime lane | OS/runtime init | low |
| `0x80004000` | `80004000_FUN_80004000.c` | fatal reset path: calls `0x801b3cc0(0,0,0)`, which reports `OSResetSystem()` obsolete | medium |
| `0x80004040` | `80004040_FUN_80004040.c` | reads byte at `r13-0x4F48` = `0x8042A738` (boot flag set by `0x80004030`) | high |
| `0x800c9f90` | `800c9f90_FUN_800c9f90.c` | **creates the game-main thread, then parks the boot thread** | high |
| `0x800c9f60` | `800c9f60_FUN_800c9f60.c` | game main thread: `dSys` init once, then frames forever | high |
| `0x801b89e0` | `801b89e0_FUN_801b89e0.c` | teardown/exit path; ctor/dtor table at `0x802EDCC0` | low |


## Boot-info branch

`0x800000F4` holds a pointer to a boot descriptor:

- `+0x08` → offset to a relocation table; entry count at table start, then the
  loader adds the descriptor base to each entry.
- `+0x0C` → boot type:
  - 2 → `0x802e9300(hw=0)`, MetroTRK debug boot (invalid id path)
  - 3 → `0x802e9300(hw=1)`, MetroTRK debug boot (NDEV)
  - 4 → `0x80004030` sets the flag at `0x8042A738`; later `0x80004040` returns
    1 and the entry calls `0x802e9398` (GDEV)
  - anything else → normal boot continues to `0x800c9f90`

Debug boot types tail-call the debugger and never reach the game hand-off.
A retail disc boot is the "anything else" case.


## Small-data base

`r13` is recovered: `0x800041C0` sets it to `0x8042F680` (verified bytes:
`3D A0 80 42` / `61 AD F6 80` at `0x80004244`). Therefore:

- `0x80004030` writes the flag at `0x8042F680 - 0x4F48 = 0x8042A738`.
- `0x80004040` reads that same byte.
- Other `r13±offset` globals can be resolved the same way.

Ghidra still prints `unaff_r13` because the ELF import did not set the register;
range-lane translations must use this base when promoting their globals.

## Reading order

1. `src/boot/entry.rs`
2. `src/boot/init_tables.rs`
3. `src/boot/mem.rs` + `cache.rs`
4. `src/runtime/os.rs` / `init.rs`
5. `src/engine/startup.rs` — thread hand-off and game main
6. `src/engine/main_loop.rs` — MetroTRK debug paths, not the game loop

