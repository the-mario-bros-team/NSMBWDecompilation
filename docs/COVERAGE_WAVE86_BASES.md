# Coverage - Wave 86 (`game::bases`, `0x8060` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** under the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader chooses the real load address, so the numbers only
identify positions inside the REL image. Cross-module pointers (`0x8035xxxx`,
`0x8042xxxx`, `0x807Cxxxx`) are `main.dol` data addresses imported by the module
and keep their absolute values.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`. "Role" below is the
plain-English behaviour. Confidence is `high` when guards, stores, constants and
call arguments are literal, and `medium` when the role of a field, a global, a
callee or a dropped argument had to be inferred (those bodies carry
`// observed:`/`// inferred:`/`// unknown:` notes). No body in this pass is
`low`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 4,880 | 16 | 14 |

## What this pass covers

The `0x8060` bucket holds 240 functions; 100 were already in
`ledger/functions.csv` when this pass began, leaving 140. This pass takes 30 of
the remaining bodies in two screen families.

- **The world-collection-coin screen, `0x80606EC0..0x80607680` (12 bodies).**
  The `worldCollectionCoin_21.brlyt` loader [`fn_80606ec0`] binds the
  `A00_Window` animation, the five `N_coinbar_01` handles at `+0x460`, the
  `T_titleCoin_00` text and the two `T_world_00` handles at `+0x474`, and caches
  the animation id at `+0x45C`. Around it sit the query gate
  [`fn_80606fb0`], the teardown [`fn_80606ff0`] (gates `+0xB4C`/`+0xB4D`) and
  the readiness check [`fn_806070c0`]. The screen's own layout objects are at
  `+0x74`/`+0x78` (with the coin bar's at `+0xDC`); [`fn_80607120`] copies the
  matrices of the `+0x460`/`+0x464`/`+0x470` pane records into them.
  [`fn_806072e0`] marks the nine unlocked coin columns (`+0xB51`) and counts them
  in `+0xB48`, the cursor movers [`fn_80607380`]/[`fn_80607430`] walk `+0xB44`
  past locked columns, and [`fn_806074e0`]/[`fn_80607590`] start the open
  animation on both `T_world_00` panes. [`fn_80607680`] rebuilds the 24 column
  records (`+0x7C`, four bytes each) through [`fn_80607620`].
- **The `dContinue_c` member bodies, `0x8060A350..0x8060B440` (18 bodies).**
  Wave 14 recorded this class's state table (`0x8060B680`) and registered
  destructor; this pass adds the bodies those descriptors dispatch. The
  `continue.arc` / `continue_05.brlyt` loader [`fn_8060a350`] binds
  `continue_05_inWindow` (7 panes), `A00_window` (7), `T_conTitle_00` (6 x 8),
  `P_bg_PContinue`, `N_4P_Pos_00` (14) and `T_conTime_00` (8). The four
  per-player availability records are recomputed by [`fn_8060a840`] through the
  `main.dol` tables `0x80354E60`/`0x80354E90`; their count panes are rebound by
  [`fn_8060a720`]/[`fn_8060a7b0`], one word of the selected layout record is read
  by [`fn_8060a5a0`], and the counts are published to the save file by
  [`fn_8060b330`]. The count-up path is [`fn_8060ad60`] (clamp at 99, digit
  count, cue `0xA5`) and [`fn_8060af20`] (five steps, cue `0xA4`), while
  [`fn_8060b1e0`] runs the `+0x2D0`/`+0x2CC` timers and waits for the four
  continue cues. The remaining gates [`fn_8060aae0`], [`fn_8060abc0`],
  [`fn_8060ac30`], [`fn_8060ac90`], [`fn_8060ae40`], [`fn_8060b150`],
  [`fn_8060b3c0`] and [`fn_8060b440`] forward to the `0x808232C0..0x80823540`
  descriptors recorded by wave 14.

No new ABI helper was needed: the pass reuses `abi_call1..5`, `vmethod0`,
`vmethod1`, `ptr_at` and `call0_ret`.

## Functions

| address | function | bytes | confidence | role |
|---|---|---:|---|---|
| `0x80606EC0` | `fn_80606ec0` | 240 | high | `worldCollectionCoin_21.brlyt` loader |
| `0x80606FB0` | `fn_80606fb0` | 64 | medium | query gate (`countLeadingZeros` test) |
| `0x80606FF0` | `fn_80606ff0` | 120 | medium | two-gate teardown + pane replay |
| `0x806070C0` | `fn_806070c0` | 96 | high | layout readiness query |
| `0x80607120` | `fn_80607120` | 440 | medium | matrix copy for three pane records |
| `0x806072E0` | `fn_806072e0` | 152 | high | nine-column availability scan |
| `0x80607380` | `fn_80607380` | 160 | high | previous unlocked column |
| `0x80607430` | `fn_80607430` | 164 | high | next unlocked column |
| `0x806074E0` | `fn_806074e0` | 164 | medium | open both `T_world_00` panes |
| `0x80607590` | `fn_80607590` | 136 | medium | open one `T_world_00` pane |
| `0x80607620` | `fn_80607620` | 88 | medium | column record writer |
| `0x80607680` | `fn_80607680` | 192 | high | rebuild 24 column records |
| `0x8060A350` | `fn_8060a350` | 260 | high | `continue_05.brlyt` loader |
| `0x8060A5A0` | `fn_8060a5a0` | 36 | medium | layout-record word getter |
| `0x8060A720` | `fn_8060a720` | 144 | medium | bind `+0x2A8` count animations |
| `0x8060A7B0` | `fn_8060a7b0` | 144 | medium | bind `+0x2B8` coin animations |
| `0x8060A840` | `fn_8060a840` | 268 | medium | recompute the four availability records |
| `0x8060AAE0` | `fn_8060aae0` | 176 | high | all-ready transition |
| `0x8060ABC0` | `fn_8060abc0` | 88 | high | gate then `0x80823340` |
| `0x8060AC30` | `fn_8060ac30` | 88 | medium | replay count panes + cue `0x7F` |
| `0x8060AC90` | `fn_8060ac90` | 88 | high | gate then `0x80823380` |
| `0x8060AD60` | `fn_8060ad60` | 220 | high | coin-count tick + digit count |
| `0x8060AE40` | `fn_8060ae40` | 88 | high | gate then `0x80823400` |
| `0x8060AF20` | `fn_8060af20` | 256 | high | five-step count-up |
| `0x8060B020` | `fn_8060b020` | 288 | medium | reset four save records + cues |
| `0x8060B150` | `fn_8060b150` | 88 | high | gate then `0x808234C0` |
| `0x8060B1E0` | `fn_8060b1e0` | 332 | medium | timer + four-cue wait |
| `0x8060B330` | `fn_8060b330` | 116 | medium | publish the four counts |
| `0x8060B3C0` | `fn_8060b3c0` | 88 | high | gate then `0x80823540` |
| `0x8060B440` | `fn_8060b440` | 96 | high | exit gate then `0x808232C0` |

## Address range

`0x80606EC0..0x8060B440`, two groups:

- Group 1 is `0x80606EC0..0x80607680`, 12 bodies. After
  [`fn_80606ec0`] the untranslated `0x80606A10`/`0x80606A50`/`0x80606A90`/
  `0x80606AF0` destructor templates sit immediately before this window; the
  window itself is contiguous and includes the `0x80606FB0` query gate.
- Group 2 is `0x8060A350..0x8060B440`, 18 bodies, contiguous except for the
  deferred `0x8060A950` between [`fn_8060a840`] and [`fn_8060aae0`]. The class's
  state table `0x8060B680` and its destructor `0x8060BEE0` were recorded by
  wave 14; the `0x8060B620`/`0x8060B650` adapter pair is still untranslated.
- After this pass the bucket still has 110 untranslated functions.

## Deliberate skips

Two bodies in this window were rejected rather than guessed, and the rest of
the bucket was simply not reached:

- `0x8060A950` (372 bytes) is the per-slot "spawn `Wm_2d_continue`" body. Its
  export copies a `0x34`-byte block out of the pane node with Ghidra's stack
  slots assigned out of address order (`local_4c` receives `+0x90`, `local_48`
  receives `+0xA0`, `local_40` receives `+0x84`), and it carries the
  `0x43300000` integer-to-double sequence and the `_DAT_80428732 == 3` branch
  that swaps `0x800B32E0` for `0x800B37E0`. The mapping from stack slot to
  matrix element cannot be proven from this export, so the body is deferred and
  only its call site [`fn_8060ad60`] is translated (address-only call with an
  `// unknown:` note).
- `0x8060B620`/`0x8060B650` (48 bytes each) are the identical two-argument
  adapters `0x802DCBFC(param_2, param_2, param_1)`. The export loses both
  parameter roles and duplicates one register, so no honest signature exists;
  they are left to a later pass with the class's remaining table entries.
- Not reached in this pass: the `0x80600990..0x80600B70` and
  `0x80606A10..0x80606AF0` duplicate destructor templates, the
  `0x806083B0..0x806099A0` coin-date screen bodies, the `0x8060C1B0..0x8060C960`
  menu destructors, and the `0x8060D100..0x8060FFB0` date-file tail.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 4,880 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the 100 pre-existing rows of
  this bucket were listed before writing and none is in this pass.
- `node tools/check-lane-csv.mjs wave86` reports `wave86-bases.csv: 30 rows`
  and `OK`.
- `cargo check --message-format=short` produces no diagnostic for
  `src/game/bases.rs`.
