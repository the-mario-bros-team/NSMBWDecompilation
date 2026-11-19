# Coverage - Wave 74 (`game::bases`, `0x8061` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** under the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader chooses the real load address, so the numbers only
identify positions inside the REL image. Cross-module pointers (`0x800Bxxxx`,
`0x800Cxxxx`, `0x8010xxxx`, `0x8042xxxx`) are `main.dol` addresses imported by
the REL and keep their absolute values.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`. Confidence is `high`
when guards, stores, constants, strings and call arguments are literal, and
`medium` when the role of a field, a callee or a global had to be inferred
(those bodies carry `// observed:`, `// inferred:` or `// unknown:` notes).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 3196 | 20 | 10 |

## What this pass covers

The `0x8061` bucket held 241 functions at lane start; 89 were already in
`ledger/functions.csv`, leaving 152. This pass fills three contiguous runs:

- **Demo-message screen tail, `0x80610FF0..0x80611790`** - eleven bodies of the
  screen constructed by [`fn_80610f40`]:
  - the two element-free helpers [`fn_80610ff0`] / [`fn_80611030`] and the two
    `+0x18` teardown helpers [`fn_80611070`] / [`fn_806110d0`];
  - the `demoMessage/demoMessage.arc` loader [`fn_80611270`], which binds the
    `demoMessage_01_inWindow` animation group, the `A00_Window` pane group, the
    `+0x248` animation word and the `T_demoS_00` textboxes at `+0x24C`;
  - the indicator query [`fn_80611330`] (same `countLeadingZeros >> 5` fold as
    [`fn_8060e900`]), the page text writer [`fn_806114a0`] (row `0x130` for page
    state `1`, `0x131` otherwise, clip table at `0x807C6A40`), the two idle gates
    [`fn_806115f0`] / [`fn_80611710`], the animation restart
    [`fn_806116c0`], and the child-predicate helper [`fn_80611790`].
  - Class attribution is `// inferred:`: the loader's `demoMessage` strings and
    the neighbouring `dDemoMessage_c` state table [`fn_80611960`] agree, while
    wave 53 had filed [`fn_80610f40`] under `dDateFile_c`.

- **Draw screen tail, `0x80611EB0..0x806124F0`** - eleven bodies of the screen
  constructed by [`fn_80611e00`]:
  - the same four release helpers ([`fn_80611eb0`], [`fn_80611ef0`],
    [`fn_80611f30`], [`fn_80611f90`]);
  - the `draw/draw.arc` loader [`fn_80612130`], which binds `draw_00.brlyt`,
    the `draw_00_inDraw` animation group, the `A00_draw` pane group, the
    `N_draw_00` pane at `+0x244` and the `T_draw_00` lines at `+0x248`;
  - the empty four-byte hook [`fn_806122f0`], the player-status gate
    [`fn_80612310`], the art-pane restart [`fn_806123a0`], the two idle gates
    [`fn_806123f0`] / [`fn_806124f0`], and the `+0x250` reset
    [`fn_80612460`].

- **Opening-title tail, `0x80614680..0x80614D40`** - eight bodies of
  `dEventOpeningTitle_c`:
  - the `openingTitle/openingTitle.arc` loader [`fn_80614680`] with the
    `T_E3verCheck` textbox at `+0x248`, the `T_2009nintendo` /
    `T_Press_00` lines and the `N_proportionC_00` pane at `+0x24C`;
  - the reveal/rewind [`fn_80614a30`], the two idle gates [`fn_80614ac0`] /
    [`fn_80614b40`], the scene-`0x2EE` pad probe [`fn_80614bc0`], the sound
    cue `0x77` close [`fn_80614c50`], the game-mode exit [`fn_80614cc0`] and the
    flag clear [`fn_80614d40`].

Recurring layout: the demo-message screen keeps its NW4R layout at `+0xB0`
(vtable word `+0xB8`), its interface at `+0x74`, its art pane at `+0x248`, two
textboxes at `+0x24C`/`+0x250`, page latch `+0x25C` and page state `+0x260`.
The draw screen and the opening-title screen share layout `+0x70` (vtable word
`+0x78`), interface `+0x208`, art/animation pane `+0x244`, and class flags at
`+0x24D..+0x276`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80610FF0` | `fn_80610ff0` | 64 | high |
| `0x80611030` | `fn_80611030` | 64 | high |
| `0x80611070` | `fn_80611070` | 96 | medium |
| `0x806110D0` | `fn_806110d0` | 100 | medium |
| `0x80611270` | `fn_80611270` | 192 | high |
| `0x80611330` | `fn_80611330` | 60 | medium |
| `0x806114A0` | `fn_806114a0` | 332 | medium |
| `0x806115F0` | `fn_806115f0` | 88 | high |
| `0x806116C0` | `fn_806116c0` | 76 | high |
| `0x80611710` | `fn_80611710` | 88 | high |
| `0x80611790` | `fn_80611790` | 96 | medium |
| `0x80611EB0` | `fn_80611eb0` | 64 | high |
| `0x80611EF0` | `fn_80611ef0` | 64 | high |
| `0x80611F30` | `fn_80611f30` | 96 | medium |
| `0x80611F90` | `fn_80611f90` | 100 | medium |
| `0x80612130` | `fn_80612130` | 220 | high |
| `0x806122F0` | `fn_806122f0` | 4 | high |
| `0x80612310` | `fn_80612310` | 108 | medium |
| `0x806123A0` | `fn_806123a0` | 76 | high |
| `0x806123F0` | `fn_806123f0` | 88 | high |
| `0x80612460` | `fn_80612460` | 60 | high |
| `0x806124F0` | `fn_806124f0` | 104 | high |
| `0x80614680` | `fn_80614680` | 268 | high |
| `0x80614A30` | `fn_80614a30` | 144 | high |
| `0x80614AC0` | `fn_80614ac0` | 88 | high |
| `0x80614B40` | `fn_80614b40` | 88 | high |
| `0x80614BC0` | `fn_80614bc0` | 120 | medium |
| `0x80614C50` | `fn_80614c50` | 104 | high |
| `0x80614CC0` | `fn_80614cc0` | 124 | medium |
| `0x80614D40` | `fn_80614d40` | 20 | high |

## Address range

`0x80610FF0..0x80614D40`, in three runs:

- `0x80610FF0..0x80611790` - eleven bodies, unbroken. The `0x802DCBFC`
  forwarders `0x80610850`/`0x80610880` sit just before the run and are skipped.
- `0x80611EB0..0x806124F0` - eleven bodies, unbroken; the already-translated
  `dDrawGame_c` table [`fn_80612740`] follows the run.
- `0x80614680..0x80614D40` - eight bodies; the only gap is the rejected
  `0x806148F0` between [`fn_80614680`] and [`fn_80614a30`], and `0x806149A0`
  was left for a later wave.

This pass adds 30 bodies (3,196 bytes), leaving 122 of the 152 remaining
`0x8061` functions for later waves.

## Deliberate skips

- `0x80610850` and `0x80610880` (48 each) - the `0x802DCBFC` indirect-jump
  forwarders. Only the hidden `r12` this-adjust is observable, so they carry no
  Rust-visible behaviour; waves 3, 71, 72 and 73 left the same stubs
  untranslated (for example `0x807A9B60`/`0x807A9B90`).
- `0x806148F0` (144) - the opening-title frame gate. Its export ends in
  `halt_baddata()` after a `ldexpf`/paired-single block
  (`_DAT_8042A1F4 * 0x40000000`), so the exit path cannot be reconstructed
  honestly. Same rejection class as `0x80651640`, `0x80728CC0` and
  `0x807A84B0`.
- `0x806149A0` (108) and the rest of the bucket are untouched, not rejected.
  `0x80613290` (216) is a paired-single rejection for the same reason as
  `0x806148F0`. The remaining `0x806181B0..0x80618AC0` `dFileSelect_c` tail,
  the `0x8061Axxx..0x8061E1B0` store/copy screens and the
  `0x8061E710..0x8061FB10` helpers belong to later waves.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 3,196 bytes);
  `0x806110D0` and `0x80611F90` are 100 bytes, not 96.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the 89 pre-existing rows of
  this bucket were checked before writing and none is in this pass.
- `node tools/check-lane-csv.mjs wave74` reports `OK` with
  `wave74-bases.csv: 30 rows`.
- `cargo check --message-format=short` produces no diagnostic for
  `src/game/bases.rs`.
