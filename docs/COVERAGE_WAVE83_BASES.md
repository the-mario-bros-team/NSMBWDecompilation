# Coverage - Wave 83 (`game::bases`, `0x8064` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** under the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader chooses the real load address, so the numbers only
identify positions inside the REL image. Cross-module pointers (`0x8037xxxx`,
`0x8042xxxx`) are `main.dol` addresses imported by the REL and keep their
absolute values.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`. "Role" below is the
plain-English behaviour. Confidence is `high` when guards, stores, constants,
strings and call arguments are literal, and `medium` when the role of a field,
a global, a callee or a dropped argument had to be inferred (those bodies carry
`// observed:`/`// inferred:`/`// unknown:` notes). No body in this pass is
`low`.

Data tables quoted below (`0x807C8264`, `0x807C8280`, `0x807C82F8`,
`0x807C8300`, `0x807C8308`, `0x807C8418`, `0x807C8420`, `0x807C8428`,
`0x807C8438`) and the string constants were re-read from the REL image at
`work/rels/elf/d_basesNP.elf`, not from prose.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 3,772 | 17 | 13 |

## What this pass covers

The `0x8064` bucket holds 196 functions; 53 were already in
`ledger/functions.csv` when this pass began (waves 13, 59 and neighbours),
leaving 143. This pass takes 30 of the remaining bodies in four runs.

- **`dStockItem_c` element pair and window head, `0x80641E10..0x80642510`
  (10 bodies).** The shared `0x114`-byte element constructor
  [`fn_80641e10`] and destructor [`fn_80641e50`] are installed by the
  `0x802DC9A0` array builder in [`fn_80641d20`] (four elements at `+0x32C`) and
  in four larger item actors at `0x8065BE80`, `0x80683750`, `0x806B6DE0` and
  `0x8070F270` (one, two, four and twelve elements), so the pair is reused far
  outside this bucket. The window head then opens
  `stockItem/stockItem.arc` ([`fn_80642250`]), seeds the seven owned-item words
  from the id table `0x807C8264` ([`fn_806420b0`], [`fn_80642230`]), probes the
  free player slot ([`fn_80642340`]), drives one frame
  ([`fn_80642380`]), resets the icon pane scale ([`fn_80642470`]) and formats
  the per-player message ids from `0x807C8308` ([`fn_80642510`]).
- **`dStockItem_c` window tail, `0x80642CC0..0x806437B0` (6 bodies).** The
  seven-slot block writer [`fn_80642cc0`], the reopen handler
  [`fn_80643300`], the four-player seeding pass [`fn_80643350`], the
  first-owned-slot scan [`fn_80643560`], the highlight mover
  [`fn_80643710`] and the commit handler [`fn_806437b0`].
- **`dTheEnd_c` member tail, `0x80645200..0x806455E0` (6 bodies).** The
  `TheEnd/TheEnd.arc` loader [`fn_80645200`] (`TheEnd_00.brlyt`, the two
  `inDraw`/`outDraw` animations, the `A00_draw` pair `0x807C8418`, message
  `0x12F`), the out-animation reset [`fn_806453e0`], the two draw starts
  [`fn_80645480`]/[`fn_806455a0`] and the two finished gates
  [`fn_806454e0`]/[`fn_806455e0`]. The class identity is fixed by
  [`fn_80645820`], which registers the four `dTheEnd_c` descriptors, and by the
  constructor [`fn_80645c80`], which installs the `+0x208` interface group and
  `+0x258` used here.
- **`dTimeUp_c` member tail, `0x80645F90..0x806463B0` plus `0x80646CD0`
  (8 bodies).** The `timeUp/timeUp.arc` loader [`fn_80645f90`]
  (`timeUp_02.brlyt`, three animations, the `A00_timeUp` triple `0x807C8428`,
  four `T_timeUP_00` panes), the free-player probe [`fn_80646050`], the four
  player labels [`fn_80646180`] (ids `0x0A/0x0C/0x0D/0x0D` from `0x807C8438`),
  the reopen [`fn_80646240`], the two gates
  [`fn_806462a0`]/[`fn_806463b0`], the `0x364` sound cue [`fn_80646310`], and
  the array-owning destructor [`fn_80646cd0`]. The class identity is fixed by
  [`fn_806465f0`], which registers the four `dTimeUp_c` descriptors.

Object layouts used by this pass: the item window keeps the layout context at
`+0x74` with its vtable at `+0x7C`, the state interface at `+0x208` and the
seven-slot block pointer at `+0x310`; its root pane is `+0x248`, the fourteen
`N_forUse_1PPos` panes start at `+0x24C`, the eight item panes are `+0x280`
plus `+0x284..+0x29C`, the seven owned-item words are `+0x8B4..+0x8CC`, the
four player slot words are `+0x890..+0x89C`, and `+0x8A0`/`+0x8A4` are the
highlight and committed slot. The two ending screens use the standard
`dCourseTimeUp`-family layout: context at `+0x70`, vtable at `+0x78`,
interface at `+0x208`, root pane at `+0x244`.

Note on `0x80429F7C`: the export names it `_DAT_80429F7C`, but the instruction
stream is `lwz r3, -0x6084(r4)` after `lis r4, 0x8043`, i.e. a **pointer load**.
This pass therefore reads it with `platform::read_u32(0x8042_9F7C)` and uses
`+0xAF8`/`+0xAFC` on the loaded manager. (An older body in this file treats the
same symbol as an absolute address; that older body is outside this lane.)

## Functions

| address | function | bytes | confidence | role |
|---|---|---:|---|---|
| `0x80641E10` | `fn_80641e10` | 60 | high | shared `0x114`-byte element constructor |
| `0x80641E50` | `fn_80641e50` | 92 | high | shared `0x114`-byte element destructor |
| `0x80641EF0` | `fn_80641ef0` | 64 | high | plain release helper |
| `0x806420B0` | `fn_806420b0` | 372 | medium | one-shot item-window initialiser |
| `0x80642230` | `fn_80642230` | 20 | high | item-id table read |
| `0x80642250` | `fn_80642250` | 240 | medium | `stockItem.arc` loader |
| `0x80642340` | `fn_80642340` | 64 | medium | free-player probe |
| `0x80642380` | `fn_80642380` | 144 | medium | item-window frame driver |
| `0x80642470` | `fn_80642470` | 152 | medium | icon pane scale reset |
| `0x80642510` | `fn_80642510` | 36 | medium | per-player message id cell |
| `0x80642CC0` | `fn_80642cc0` | 28 | medium | slot value writer + dirty flag |
| `0x80643300` | `fn_80643300` | 76 | high | reopen handler |
| `0x80643350` | `fn_80643350` | 248 | medium | four-player seeding pass |
| `0x80643560` | `fn_80643560` | 388 | medium | first-owned-slot scan |
| `0x80643710` | `fn_80643710` | 160 | high | highlight mover |
| `0x806437B0` | `fn_806437b0` | 140 | high | slot commit handler |
| `0x80645200` | `fn_80645200` | 204 | medium | `TheEnd.arc` loader |
| `0x806453E0` | `fn_806453e0` | 76 | high | out-animation reset |
| `0x80645480` | `fn_80645480` | 84 | high | show root pane, start animation 0 |
| `0x806454E0` | `fn_806454e0` | 88 | high | draw-finished gate |
| `0x806455A0` | `fn_806455a0` | 64 | high | start animation 1 |
| `0x806455E0` | `fn_806455e0` | 88 | high | exit-finished gate |
| `0x80645F90` | `fn_80645f90` | 192 | medium | `timeUp.arc` loader |
| `0x80646050` | `fn_80646050` | 60 | medium | free-player probe (in-module query) |
| `0x80646180` | `fn_80646180` | 156 | medium | four player time labels |
| `0x80646240` | `fn_80646240` | 84 | high | reopen animation 0 |
| `0x806462A0` | `fn_806462a0` | 88 | high | open-finished gate |
| `0x80646310` | `fn_80646310` | 80 | high | `0x364` sound cue + animation 1 |
| `0x806463B0` | `fn_806463b0` | 96 | high | display transition gate |
| `0x80646CD0` | `fn_80646cd0` | 128 | high | array-owning destructor |

## Address range

`0x80641E10..0x80646CD0`, four runs:

- Run 1 is `0x80641E10..0x80642510` with deliberate holes at the two
  destructor wrappers `0x80641F30`/`0x80641F90` (left for the next pass, they
  are `0x802B9280`-only releases like [`fn_80641ef0`]) and at the untranslated
  `0x80642540`. The only gaps in the run are the two 12-byte alignment pads
  between `0x806420B0`/`0x80642230` and `0x80642230`/`0x80642250`.
- Run 2 is `0x80642CC0..0x806437B0`, with holes at `0x80642CE0`
  (bad instruction data), `0x80642D20` (paired-single draw pass),
  `0x80642DF0`/`0x80642F30` (effect spawners, deferred for budget) and
  `0x80643260` (truncated by `halt_baddata`). Above the run,
  `0x80642540..0x80642CC0` (the 1,236-byte pane rebuild at `0x80642540`, the
  660-byte body at `0x80642A20`) is still untranslated.
- Run 3 is `0x80645200..0x806455E0`, four consecutive bodies.
- Run 4 is `0x80645F90..0x806463B0`, plus the isolated `0x80646CD0`.
- After this pass the bucket still has 113 untranslated functions, chiefly the
  `0x80642540..0x80643F30` item-window middle (the 1,236-byte pane rebuild and
  the seven state handlers), the `0x806441E0..0x80644CF0` registration and
  screen body, and the `0x80647B50..0x8064F820` course-select manager.

## Deliberate skips

- `0x80642CE0` (64 bytes) is truncated by bad instruction data
  (`halt_baddata`) after one float add.
- `0x80642D20` (204 bytes) is a paired-single reconstruction (`unaff_GQR0`,
  `ldexpf` lanes). The integer structure is readable but the paired lanes are
  not recoverable, so it stays an address-only call from [`fn_80642380`].
- `0x80643260` (148 bytes) ends in `halt_baddata`; the scan loops are readable
  but the final store is followed by an invalid instruction, so it was not
  guessed.
- `0x80643470` (228 bytes) is otherwise readable, but its `FUN_807678C0()` call
  is printed without arguments while the callee's own export takes one pointer.
  Ghidra dropped the register, so the argument cannot be reconstructed
  honestly; the body was left out rather than emitted with a wrong ABI.
- `0x80642DF0` (308 bytes) and `0x80642F30` (272 bytes) are recoverable effect
  spawners (`Wm_2d_stockitem`, a fifteen-word matrix copy around
  `0x800B32E0`); they were left for the next pass only because this lane's
  budget was spent on the four runs above, not because they were rejected. The
  indirect call `0x80642F30` from [`fn_80642380`] is an address-only call.
- `0x80642540` (1,236 bytes) is the seven-pane rebuild; it is called
  address-only from [`fn_80642380`] and is large enough to deserve its own
  pass.
- Nothing else in the four runs was judged unreconstructable.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 3,772 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the 53 pre-existing rows of
  this bucket were listed before writing and none is in this pass.
- `node tools/check-lane-csv.mjs wave83` reports
  `wave83-bases.csv: 30 rows` and `OK`.
- `cargo check --message-format=short` produces no diagnostic for
  `src/game/bases.rs` (the other lanes' files and the pre-existing
  `src/runtime` warnings are their own).
