# Coverage - Wave 78 (`game::bases`, `0x8075` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the
`d_basesNP` REL. Every address is a **static analysis address** under the
canonical synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a
runtime address: the Wii REL loader chooses the real load address, so the
numbers only identify positions inside the REL image. Cross-module pointers
(`0x8037xxxx`, `0x8042xxxx`) are `main.dol` addresses imported by the REL and
keep their absolute values.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`. In the table below,
"role" is the plain-English behaviour, and confidence is `high` when guards,
stores, constants, strings and call arguments are literal, `medium` when the
role of a field or callee had to be inferred (those bodies carry `// observed:`,
`// inferred:` or `// unknown:` notes), and `low` when only the instruction
sequence is certain.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 5464 | 20 | 10 |

No body in this pass is `low` confidence.

## What this pass covers

The `0x8075` bucket held 207 functions at lane start; 57 were already in
`ledger/functions.csv` (the `daWaterBubbleCannon_c`, `daWaterMove_c`,
`daWaterXinfinitMove_c`, `daWaterAllRail_c`, `daWireMove_c` and `daWireCircle_c`
state tables with their destructors and constructors, the `obj_waterfloat`,
`kanaami_enkei`, `kanaami_urakabe`, `kanaami2`, `remo_kanaami` and
`da2DPlayer_c`-adjacent actor bodies, and the `0x80757AB0`/`0x80759C60`
constructors recorded by waves 4, 31 and 32), leaving 150. This pass takes one
contiguous run with a single hole, `0x8075C930..0x8075F1C0`, and fills three
member sets of related actors that share the base-actor layout:

- **Bubble / `character_SV` member tail, `0x8075C930..0x8075E530`** — the
  nineteen bodies of the actor whose constructor is [`fn_8075cb90`]:
  - the transform writer [`fn_8075c930`] (node `+0x1AC`), its scale reset
    [`fn_8075c9e0`], the node clear [`fn_8075ca40`], the constructor
    [`fn_8075cb90`] (vtable `0x80814A40`, interface `0x80814AB0` at `+0x188`,
    four `0x802DC9A0` arrays at `+0x1AC`/`+0x26C`/`+0x314`/`+0x3B8`), the
    0x7C-byte ring record constructor/destructor pair
    [`fn_8075ccb0`] / [`fn_8075cd20`];
  - the start-up body [`fn_8075cef0`] (anchor at `+0x3A0`, 0x40-byte effect
    record at `+0x3B4`, vtable `0x80814A34`) and the placement
    [`fn_8075cff0`], which re-parents variants `1`/`2` onto the player actor at
    `0x8042A1A0` and publishes the low flag byte through `0x80832610`;
  - the `bubble` / `bubble_tail` / `character_SV` loader [`fn_8075d300`]
    (`g3d/bubble.brres`, three panes from the name table `0x808149BC`, ten
    0x7C-byte `bubble_tail` records at `+0x3C8`, then `0x800F6710` with
    `character_SV` / `g3d/model.brres`);
  - the ten-slot ring itself: the push [`fn_8075e430`] (index at `+0x890`,
    jitter from `0x800B2EE0(8)`, lifetime `0x808147C0`) and the age-out
    [`fn_8075e530`] (two passes of five `+0xC` words, clamp at `0x807CECB8`);
  - the pickup machine: the state entries [`fn_8075da00`] / [`fn_8075da30`] /
    [`fn_8075dc40`] / [`fn_8075def0`], the anchor rebuild [`fn_8075df70`], the
    per-frame gate [`fn_8075dd60`] (attach latch `+0x3AD`, effects
    `0x11`/`0x5E`/`0x5F`, rise word `+0x3B0`), the player-box trigger
    [`fn_8075e010`] (`0x80832618`) and its box test [`fn_8075e170`].

- **`cobCannon` object, `0x8075E900..0x8075EBF0`** — six bodies of a one-node
  actor: the constructor [`fn_8075e900`] (vtable `0x80814B58`, node `+0x1A4`),
  the execute [`fn_8075ea40`] (class virtual `+0x60`, model virtual
  `+0x1C`, placement `0x801006C0`), the animation step [`fn_8075ead0`], the
  previous-position copy [`fn_8075eb10`], the `g3d/model.brres` /
  `cobCannon` loader [`fn_8075eb40`] and the transform writer
  [`fn_8075ebf0`].

- **`cobCastle` object, `0x8075EDE0..0x8075F1C0`** — five bodies of the castle
  object: the constructor [`fn_8075ede0`] (vtable `0x80814CA8`, container
  `+0x1A8`, model `+0x1AC`, four 0x38-byte door slots at `+0x1B8`), the
  start-up [`fn_8075ef20`], the animation step [`fn_8075f040`], the four-door
  `cobCastleOpen` loader [`fn_8075f080`] (names from `0x80814C70`, flag bytes
  from `0x807CECF8`, rate `0x807CED08`) and the transform writer
  [`fn_8075f1c0`].

Recurring layout facts used by this pass: `+0x04` is the packed flags word
(low byte = player index, bits 16..31 = variant), `+0x60` the class vtable,
`+0x7C` the NW4R transform matrix, `+0xAC`/`+0xB0`/`+0xB4` the position,
`+0xDC`/`+0xE0`/`+0xE4` the scale, `+0x100`/`+0x102`/`+0x104` the three rotation
angles and `+0x128`/`+0x12C`/`+0x130`/`+0x134` the previous position. Transform
writers always run `0x801C0BD0` (build matrix), `0x8016EF50` (apply angles),
`0x8016A170`/`0x8016A150`/`0x8016A1A0` (bind matrix, bind scale, finish), and
model loaders always bracket their work with `0x800690C0`/`0x800690E0`,
`0x800DF180` (BRRES), `0x80239E30`/`0x8023A0B0`/`0x8023A270` (pane/clip) and
`0x800F2A20` (flush).

## Functions

| address | function | bytes | confidence | role |
|---|---|---:|---|---|
| `0x8075C930` | `fn_8075c930` | 176 | high | transform writer on bubble node `+0x1AC` |
| `0x8075C9E0` | `fn_8075c9e0` | 96 | high | player-flag clear and scale reset |
| `0x8075CA40` | `fn_8075ca40` | 12 | high | clear `+0x1E8` |
| `0x8075CB90` | `fn_8075cb90` | 220 | high | bubble actor constructor |
| `0x8075CCB0` | `fn_8075ccb0` | 108 | high | 0x7C-byte ring record constructor |
| `0x8075CD20` | `fn_8075cd20` | 104 | high | ring record destructor |
| `0x8075CEF0` | `fn_8075cef0` | 256 | medium | start-up, anchor and effect record |
| `0x8075CFF0` | `fn_8075cff0` | 232 | high | place on the player and select variant |
| `0x8075D300` | `fn_8075d300` | 704 | medium | `bubble` / `character_SV` model loader |
| `0x8075DA00` | `fn_8075da00` | 12 | high | clear state word `+0x398` |
| `0x8075DA30` | `fn_8075da30` | 184 | medium | pop state and pop effect |
| `0x8075DC40` | `fn_8075dc40` | 288 | medium | attached state and attach effect |
| `0x8075DD60` | `fn_8075dd60` | 400 | medium | per-frame attach/respawn/rise |
| `0x8075DEF0` | `fn_8075def0` | 108 | high | popped state with two effects |
| `0x8075DF70` | `fn_8075df70` | 152 | high | anchor plus variant-table position |
| `0x8075E010` | `fn_8075e010` | 348 | medium | player-box pickup trigger |
| `0x8075E170` | `fn_8075e170` | 216 | high | horizontal box test |
| `0x8075E430` | `fn_8075e430` | 248 | medium | push one bubble-ring record |
| `0x8075E530` | `fn_8075e530` | 168 | high | age out all ten ring records |
| `0x8075E900` | `fn_8075e900` | 84 | high | `cobCannon` constructor |
| `0x8075EA40` | `fn_8075ea40` | 144 | medium | `cobCannon` execute |
| `0x8075EAD0` | `fn_8075ead0` | 48 | high | `cobCannon` animation step |
| `0x8075EB10` | `fn_8075eb10` | 40 | high | `cobCannon` previous-position copy |
| `0x8075EB40` | `fn_8075eb40` | 172 | high | `cobCannon` model loader |
| `0x8075EBF0` | `fn_8075ebf0` | 176 | high | `cobCannon` transform writer |
| `0x8075EDE0` | `fn_8075ede0` | 136 | high | `cobCastle` constructor |
| `0x8075EF20` | `fn_8075ef20` | 100 | medium | `cobCastle` start-up |
| `0x8075F040` | `fn_8075f040` | 48 | high | `cobCastle` animation step |
| `0x8075F080` | `fn_8075f080` | 308 | medium | `cobCastle` four-door loader |
| `0x8075F1C0` | `fn_8075f1c0` | 176 | high | `cobCastle` transform writer |

## Address range

`0x8075C930..0x8075F1C0`, one run with one hole:

- `0x8075C6F0` (the `cobBoard` loader, 576 bytes) sits immediately below the
  run and is **not** rejected, only left for a later pass, so the run starts at
  `0x8075C930`.
- `0x8075D5C0` (1080 bytes) is the deliberate hole: see below.
- `0x8075F270` (768 bytes, the `cobCastle` per-frame body) sits immediately
  above the run and is left for a later pass; [`fn_8075ef20`] calls it by
  address with an explicit `// unknown:` note.
- The rest of the bucket, `0x80750000..0x8075C6F0` and
  `0x8075F570..0x8075FD00`, is untouched.

## Deliberate skips

- `0x8075D5C0` (1080 bytes) — the per-frame draw body of the bubble ring. Its
  export is a quantised-load artefact: it reads `unaff_GQR0` (the paired-single
  quantisation register) and rewrites the float locals through `ldexpf` scaling
  branches, and it also drops the stack arguments into `in_f30`/`in_f31`. The
  real instruction sequence cannot be recovered from this export, so no body
  was written. The call site in [`fn_8075cef0`] keeps the address with an
  explicit `// unknown:` note.
- `0x8075C6F0` (`cobBoard` loader) and `0x8075F270` (`cobCastle` per-frame
  body) are **not** rejected; they were left out to keep this lane at 30
  bodies, and the second one is called by address from [`fn_8075ef20`].
- The rest of the bucket is untouched. Nothing else in this range was judged
  unreconstructable.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 5,464 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the 57 pre-existing rows of
  this bucket were listed before writing and none is in this pass.
- `node tools/check-lane-csv.mjs wave78` reports
  `wave78-bases.csv: 30 rows` and `OK`.
- `cargo check --message-format=short` produces no diagnostic for
  `src/game/bases.rs`.
