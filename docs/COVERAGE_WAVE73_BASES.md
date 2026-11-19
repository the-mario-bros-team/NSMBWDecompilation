# Coverage - Wave 73 (`game::bases`, `0x807a` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** under the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader chooses the real load address, so the numbers only
identify positions inside the REL image. Cross-module pointers (`0x800Bxxxx`,
`0x8010xxxx`, `0x8016xxxx`, `0x8031xxxx`, `0x8042xxxx`) are `main.dol`
addresses imported by the REL and keep their absolute values.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`. Confidence is `high`
when the guards, stores, constants, strings and call arguments are literal, and
`medium` when the role of a field, of a callee or of a global had to be
inferred (those bodies carry `// observed:`, `// inferred:` or `// unknown:`
notes).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 27 | 3536 | 17 | 10 |

## What this pass covers

The `0x807a` bucket held 212 functions at lane start; 59 were already in
`ledger/functions.csv`, leaving 153. This pass fills two clusters:

- **`dOtasukeInfo_c` help screen, `0x807A8DB0..0x807A9970`** - the sixteen
  member bodies of the screen whose eight state descriptors are registered by
  the already-translated [`fn_807a9bc0`] and whose in-place constructor is
  [`fn_807a8a90`]:

  - the `otasukeInfo/otasukeInfo.arc` + `otasukeInfo_18.brlyt` loader
    [`fn_807a8db0`], which binds seven `inWindow` animations, twelve
    `A00_inWindow` pane animations, the two `P_yesBase_00` panes and the six
    `T_infoS_00` textboxes;
  - the six-line message writer [`fn_807a9020`], whose rows live at
    `0x807D06E0` and whose selector is the global mode word `0x8042A204`
    (`1` -> row 1, `4` -> row 2, anything else -> row 0);
  - the yes/no window body [`fn_807a9220`], the wait bodies
    [`fn_807a9150`] / [`fn_807a9730`] / [`fn_807a9970`], the closing bodies
    [`fn_807a9310`] / [`fn_807a93a0`] / [`fn_807a9660`], and the confirm/back
    dispatcher [`fn_807a9420`];
  - the page animations [`fn_807a95c0`] / [`fn_807a96d0`] with cues
    `0x78`/`0x79`, the closing animation `0xB` and cue `0x7A` in
    [`fn_807a9840`], the restore body [`fn_807a98a0`] and the global clear
    [`fn_807a9960`];
  - the five release wrappers at the head of the run ([`fn_807a89b0`],
    [`fn_807a8b40`], [`fn_807a8b80`], [`fn_807a8bc0`], [`fn_807a8c20`]), plus
    the second layout/model binder [`fn_807a8920`] and the two-triple position
    seed [`fn_807a8440`].

- **Neighbouring screen helpers, `0x807A6B20..0x807A7A20`** - the projected
  position writer [`fn_807a6b20`] (camera `0x80429E00`, stage origin
  `0x8042A008`, scale `0x807D05A0`), the three-holder animation binder
  [`fn_807a6e90`], the in-place constructor [`fn_807a7030`] (vtable
  `0x8081D360`, published through `0x8042A1AC`) and the global-gated reset
  [`fn_807a7a20`].

Recurring layout: the help screen keeps its NW4R layout at `+0x70` (vtable word
at `+0x78`; the second binder reads the same object's vtable through `+0xA8`),
its three model/animation holders at `+0xF8`, `+0x180` and `+0x268`, the pane
handle at `+0x244`, six textbox handles at `+0x248..+0x25C`, two pane handles at
`+0x260..+0x264`, and the interface/state object at `+0x208` whose slot `+0x18`
receives descriptor pointers. Page latch `+0x26A`, closing latch `+0x269`,
window latch `+0x26C` and exit latch `+0x26D` drive the state routing. The
screen draws on the main.dol mode word `0x8042A204` (`0`, `1` or `4`), the pad
table `0x80429F50`, the pause controller at `0x80429FD8` and the sound manager
`0x8042A488`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x807A6B20` | `fn_807a6b20` | 180 | medium |
| `0x807A6E90` | `fn_807a6e90` | 192 | medium |
| `0x807A7030` | `fn_807a7030` | 76 | medium |
| `0x807A7A20` | `fn_807a7a20` | 88 | medium |
| `0x807A8440` | `fn_807a8440` | 112 | medium |
| `0x807A8920` | `fn_807a8920` | 144 | medium |
| `0x807A89B0` | `fn_807a89b0` | 88 | high |
| `0x807A8B40` | `fn_807a8b40` | 64 | high |
| `0x807A8B80` | `fn_807a8b80` | 64 | high |
| `0x807A8BC0` | `fn_807a8bc0` | 96 | high |
| `0x807A8C20` | `fn_807a8c20` | 100 | high |
| `0x807A8DB0` | `fn_807a8db0` | 220 | high |
| `0x807A9020` | `fn_807a9020` | 184 | high |
| `0x807A90E0` | `fn_807a90e0` | 92 | medium |
| `0x807A9150` | `fn_807a9150` | 180 | medium |
| `0x807A9220` | `fn_807a9220` | 144 | high |
| `0x807A9310` | `fn_807a9310` | 108 | high |
| `0x807A93A0` | `fn_807a93a0` | 88 | high |
| `0x807A9420` | `fn_807a9420` | 388 | medium |
| `0x807A95C0` | `fn_807a95c0` | 148 | high |
| `0x807A9660` | `fn_807a9660` | 84 | high |
| `0x807A96D0` | `fn_807a96d0` | 96 | high |
| `0x807A9730` | `fn_807a9730` | 244 | high |
| `0x807A9840` | `fn_807a9840` | 84 | high |
| `0x807A98A0` | `fn_807a98a0` | 180 | medium |
| `0x807A9960` | `fn_807a9960` | 16 | high |
| `0x807A9970` | `fn_807a9970` | 76 | high |

## Address range

`0x807A6B20..0x807A9970`, in two runs:

- `0x807A6B20..0x807A7A20` - four bodies; `0x807A6BE0` (680) was left out for
  its paired-single block, and `0x807A6F50`/`0x807A7170`/`0x807A71C0`/
  `0x807A7220`/`0x807A7550`/`0x807A7860`/`0x807A78C0` are untouched (not
  rejected).
- `0x807A8440..0x807A9970` - twenty-three bodies; the only interrupt is the
  rejected `0x807A84B0` (1124) between `0x807A8440` and `0x807A8920`, and the
  already-translated constructor [`fn_807a8a90`] and state table
  [`fn_807a9bc0`] bracket the run.

This pass adds 27 bodies, leaving 126 of the 153 remaining `0x807a` functions
for later waves.

## Deliberate skips

- `0x807A84B0` (1124) - the screen's matrix/fade rebuild. Its export begins with
  the paired-single block (`unaff_GQR0`, repeated `ldexpf`, `CONCAT44`) and the
  lane split cannot be reconstructed honestly. Same rejection recorded for
  `0x80728CC0`, `0x80730360`, `0x80651640`, `0x80682F40` and `0x806843B0` in
  earlier waves.
- `0x807A9B60` and `0x807A9B90` (48 each) - the `0x802DCBFC` indirect-jump
  forwarders. Only the hidden `r12` this-adjust is observable, so they carry no
  Rust-visible behaviour; waves 3, 71 and 72 left the same stubs untranslated.
- `0x807A6BE0` (680) - the paired-single matrix/fade body of the neighbouring
  screen, rejected for the same reason as `0x807A84B0`.
- The rest of the bucket is untouched, not rejected: `0x807A0090..0x807A1B30`,
  `0x807A2200`, `0x807A28B0..0x807A2CA0`, `0x807A3820..0x807A3D00`,
  `0x807A4450..0x807A4F30`, `0x807A57F0..0x807A6430`, and
  `0x807AA310..0x807AFF90` belong to later waves.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (27 rows, 3536 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the 59 pre-existing rows of
  this bucket were checked before writing and none is in this pass.
- `node tools/check-lane-csv.mjs wave73` reports `OK` with
  `wave73-bases.csv: 27 rows`.
- `cargo check --message-format=short` produces no diagnostic for
  `src/game/bases.rs` (the only errors belong to another lane's file).
