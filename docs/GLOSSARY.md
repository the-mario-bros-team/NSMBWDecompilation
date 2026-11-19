# Glossary

| Term | Meaning |
|---|---|
| **DOL** | Wii/GameCube executable container: 7 text + 11 data sections, BSS, entry point. `sys/main.dol`. |
| **REL** | Relocatable module loaded by `OSLink`; NSMBW has four (`d_basesNP`, `d_enemiesNP`, `d_en_bossNP`, `d_profileNP`). |
| **U8 / ARC** | Nintendo archive (`55 AA 38 2D`); 820 in this game, extension `.arc`. |
| **BRRES** | Wii resource container inside arcs: models (`MDL0`), textures (`TEX0`), animations (`CHR0`, `CLR0`, `SRT0`, `PAT0`, `SCN0`). |
| **BCSV** | Binary CSV table used for gameplay/data parameters. |
| **BRLYT / RLAN** | NW4R layout and animation files for UI. |
| **BRFNT** | Bitmap font. |
| **BRSTM / BRSAR / BREFF** | Streamed audio, sound archive, effects bank. |
| **NW4R / nw4hbm** | NintendoWare 4 Revolution libraries (graphics, layout, sound, utilities). |
| **GX** | GameCube/Wii fixed-function GPU API (TEV, EFB, XFB). |
| **AX** | Wii audio DSP/voice API; JAS/JAI sit above it. |
| **MEM1** | 24 MB main memory at `0x80000000`; `MEM2` is the extra 64 MB on Wii. |
| **VI** | Video interface / retrace timing. |
| **PAD / WPAD / KPAD** | GameCube controller, Wiimote, and Wiimote-with-extension APIs. |
| **DOL entry** | `0x80004050`; CRT/startup code. It does not call a game `main`; it hands off to the game-main thread at `0x800C9F60`. |
| **OSLink** | Runtime REL linker/relocator (`0x801B1F20`). It does **not** choose the load base; the caller allocates and passes the module image. A linked module satisfies `base = sectionInfoOffset - 0x4C`. |
| **r13 / r2** | PowerPC small-data base registers. `0x800041C0` sets `r13 = 0x8042F680` and `r2 = 0x80433080`; `r13±offset` globals have known absolute addresses. Ghidra still prints `unaff_r13` because the ELF import does not set the register. |
| **paired-single** | Broadway FPU extension (`psq_l`, `ps_*`); requires the Gekko_Broadway Ghidra language. |
| **TEV** | GX texture environment unit; the programmable-ish pixel pipeline that must be reimplemented for a port. |
| **r13 offset** | e.g. `r13-0x4F48` = `0x8042A738`; resolvable via the known r13 base. |
| **`__savegprlr` / `__restgprlr`** | Metrowerks outlined register save/restore helpers at `0x802DCD40+`; Ghidra sometimes renders them as calls or as `u64` returns. They are compiler glue, not game functions. |
