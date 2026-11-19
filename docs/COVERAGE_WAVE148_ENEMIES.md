# Coverage - Wave 148 (`game::enemies`)

## How to read this file

This page records the Wave 148 `game::enemies` lane: 16 functions of REL module
3, `d_enemiesNP`, appended to `src/game/enemies.rs` as section 130 and mirrored
one row per body in `ledger/agents/wave148-enemies.csv`.

Every `address` is a **synthetic analysis address** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Raw words
were decoded from `work/rels/elf/d_enemiesNP.elf` through the `PT_LOAD` headers
as described in `docs/REL_ELF_MAPPING.md`
(`file_offset = address - 0x80A00000 + 0x1000` for the first segment), and every
body was followed from its entry to its terminal instruction. Byte counts are
the `size` column of `work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 16 | 7340 | 12 | 4 |

Status split: 16 `translated` (12 high, 4 medium), no `partial` and no `stub`.

Covered address range: `0x80A06560..0x80B16CE0` (5,964 bytes of
`docs/SPEC_WAVE147_ENEMIES.md` ready rows and 1,376 bytes of `0x80A90000`
planner-bucket rows).

## What this pass covers

- **Eleven of the twelve "ready" `docs/SPEC_WAVE147_ENEMIES.md` rows.** Rank 12
  (`0x80A56920`) was already translated in wave 147, so this lane implements
  ranks 1-11: `0x80A56B10` (the firebar emitter that reads the `+0x1EF0` ring
  written by `fn_80a56920`), `0x80A6FBC0` (the animation-gated sway ramp and its
  two-state machine), `0x80ADDE10` (the bubble hurt-state entry) and
  `0x80AD4000` (the `+0x664/+0x662` kill gate), the two `qr3` position solves
  `0x80A1EBC0` and `0x80A06560`, `0x80A20900` (the four-player proximity damage
  emitter for effect `0x43B`), `0x80B0D4D0` (the turn-order verdict),
  `0x80B04A90` (the chain-segment initialiser), `0x80B16CE0` (the `unizo`
  brres/clip binder) and `0x80B115A0` (the yaw-pitch-yaw spawn matrix).
- **Five `0x80A90000` planner-bucket rows.** `0x80A91D30` (the
  `Wm_en_movecloud` tail emit), `0x80A9C690` (the side-selected
  `wait`/`throw`/`dead` clip binder), `0x80A99360` (the side-dependent hurt
  scale), `0x80A97180` (the twelve-frame collapse ladder) and `0x80A94D10` (the
  contact-driven lean with effect `0x1B9`).

### `qr3` sites

Nine data `qr3` sites are ported, every one as the unsigned-halfword widen pinned
by `docs/REL_ELF_MAPPING.md` (`GQR3 = 0x00050005`, `W = 1`, `I = 3`):

| address | sites | port |
|---|---|---|
| `0x80A1EBC0` | `0x80A1EC0C`, `0x80A1EC38`, `0x80A1ECB4`, `0x80A1ECE0`, `0x80A1ED1C`, `0x80A1ED38` | `(angle as u16) as f32 * 1/256` |
| `0x80A06560` | `0x80A06660`, `0x80A06680` | same halfword, cosine then sine |
| `0x80A94D10` | one site on `-this+0x102` | `(flipped as u16) as f32 * 1/256` |

Every other `psq_*` in the 16 rows is a `qr0` `f27..f31` stack spill and is not
ported.

### Specification corrections made while porting

Four places disagree with the specification page or the Ghidra decompilation and
the raw stream won:

- **`0x80B0D4D0` front test.** The specification's Port note says the raw
  direction is `dx > 0` for a side-`0` actor and `dx < 0` otherwise. The words
  say the reverse: `0x80B0D564` is `bc 4,1` (branch if not greater, i.e.
  `dx > 0` sets the flag) on the non-zero-side arm, and `0x80B0D574` is
  `bc 4,0` (branch if not less, i.e. `dx < 0` sets the flag) on the side-`0`
  arm. The export's `<`/`>` and the port follow the raw stream.
- **`0x80B04A90` fourth argument.** The specification prints
  `0x80B04270(seg, owner, &pos, mode)` and derives `mode = (word >> 20) & 1`.
  The callee overwrites `r6` with `0x80B04660` before its `0x8007FA40` call
  (`0x80B042BC..0x80B042E0`), so the incoming `mode` is dead and the translated
  `fn_80b04270` correctly takes three arguments. The port calls it directly.
- **`0x80A91D30` extent.** `docs/SPEC_WAVE147_ENEMIES.md` groups this row with
  the "short bodies followed by padding". Its 56 bytes are a complete body: the
  guard at `0x80A91D30..0x80A91D38` and the tail `bcctr` (`0x4E800420`) at
  `0x80A91D64` are the whole extent, and Ghidra's "could not recover jumptable"
  warning is that tail branch.
- **`0x80A06560` mode polarity.** The raw `bc 12,2` at `0x80A065E4` takes the
  `0x80A06644` block when `+0x1162 == 0`, so the circle/smoothing block is the
  zero case and the record block is the non-zero case, as the specification
  notes; the port keeps the raw direction rather than the export's inverted
  nesting.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A06560` | `fn_80a06560` | 500 | high |
| `0x80A1EBC0` | `fn_80a1ebc0` | 520 | high |
| `0x80A20900` | `fn_80a20900` | 520 | high |
| `0x80A56B10` | `fn_80a56b10` | 672 | high |
| `0x80A6FBC0` | `fn_80a6fbc0` | 624 | high |
| `0x80A91D30` | `fn_80a91d30` | 56 | high |
| `0x80A94D10` | `fn_80a94d10` | 404 | medium |
| `0x80A97180` | `fn_80a97180` | 476 | medium |
| `0x80A99360` | `fn_80a99360` | 228 | medium |
| `0x80A9C690` | `fn_80a9c690` | 212 | high |
| `0x80AD4000` | `fn_80ad4000` | 532 | high |
| `0x80ADDE10` | `fn_80adde10` | 580 | high |
| `0x80B04A90` | `fn_80b04a90` | 512 | medium |
| `0x80B0D4D0` | `fn_80b0d4d0` | 520 | high |
| `0x80B115A0` | `fn_80b115a0` | 480 | high |
| `0x80B16CE0` | `fn_80b16ce0` | 504 | high |

### Why four rows are `medium`

- **`0x80B04A90`** - every offset, stride and float is literal; the area-null
  guard would read address `0x3C` in the original, so the port returns early
  instead of reproducing the fault, and the dead `mode` argument is documented.
- **`0x80A99360`** - every constant and store is literal; the `+0x114`/`+0x118`
  word roles are inferred from the neighbouring hurt-scale bodies.
- **`0x80A97180`** - the ladder boundaries (`4`, `7`, `0xC`, `0x11`), the four
  constants and every store are literal; the `+0x630`/`+0x5F8`/`+0x590` roles
  are inferred from the shape of the ladder.
- **`0x80A94D10`** - the clamps, the `0.125`/`0.5` table pair and the single
  `qr3` site are literal; the meaning of `fn_80a92320`'s first output is
  inferred from the two comparison thresholds.

## Deliberate skips

The `0x80A90000` bucket still has six untranslated rows, all below the size cut
or blocked:

- **`0x80A933E0` (36 bytes).** The export is `halt_baddata()`: the extent is not
  an instruction body, so no body was invented.
- **`0x80A965D0` (280 bytes) and `0x80A94100` (408 bytes).** Both start with the
  `_savegpr` register-pair artefact: Ghidra's 64-bit "return" is the incoming
  `r3:r4` pair, so the bodies have a second actor argument that the export does
  not name. `0x80A94100` additionally declares three incoming arguments. The
  argument layout is not established, so neither was written.
- **`0x80A969C0` (844 bytes).** Five unledgered helpers (`0x80141F70`,
  `0x800B3100`, `0x800B3600`, `0x800B3750`, `0x80101AA0`) and the
  `0x80429F7C + index*0x16 + 0xAFE` per-player table drive the body; their
  contracts are not established.
- **`0x80A92F40` (1172 bytes) and `0x80A9B3C0` (1196 bytes).** The two largest
  bucket rows. Their global float constants and the `+0x1C3`/`+0xEA4` sub-object
  layouts are not established from this export alone; left for a later pass.
- **`0x80A56920` (476 bytes).** Not a skip: it is rank 12 of
  `docs/SPEC_WAVE147_ENEMIES.md` and was already translated in wave 147, so it
  was only used as the cross-check for `0x80A56B10`'s `12.0`/`8.0` biases.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 16 sizes sum to 7,340
  bytes (5,964 for the eleven specification rows, 1,376 for the five bucket
  rows).
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; all 16 addresses were absent from
  `ledger/functions.csv` and from every earlier `fn_<addr>` body before this
  pass.
- Every `# Original` block carries the `Synthetic base: 0x80A00000` line.
- The nine data `qr3` sites listed above are ported as `(f32)(u16)`; every other
  `psq` is a `qr0` spill.
- The module header was updated: cluster item 130, the cluster count
  (129 to 130), the function totals (3399 to 3415 public translations, 3328 to
  3344 `fn_<addr>` bodies), the "start here" list and the records list.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave148` reports 16 rows for this lane
  (`translated/high=12`, `translated/medium=4`), 16 unique addresses and `OK`.
