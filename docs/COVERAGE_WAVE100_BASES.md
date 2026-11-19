# Coverage - Wave 100 (`game::bases`, `0x8077` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the
`d_basesNP` REL. Every address is a **static analysis address** under the
canonical synthetic base **`0x80600000`** (produced by
`tools/rel2elf.mjs`), never a runtime address: the Wii REL loader chooses the
real load address, so the numbers only identify positions inside the REL image.
In this image a virtual address maps to the REL file at
`file_offset = address - 0x80600000`; that is how the float constants cited
below were checked. Cross-module pointers (`0x8035xxxx`, `0x8042xxxx`) keep
their absolute values, and the data pointers and vtable addresses at
`0x8081xxxx` are relocated at load time.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c` (checked with a glob,
never assumed). "Confidence" is `high` when guards, stores, constants and call
arguments are literal, and `medium` when a field role, a global, a callee or a
vtable slot had to be inferred (those bodies carry `// observed:` /
`// inferred:` / `// unknown:` notes). No body in this pass is `low`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 5,612 | 10 | 20 |

## What this pass covers

The `0x8077` bucket holds 182 functions; 57 were already in
`ledger/functions.csv` when this pass began, leaving 125. This pass takes 30
from one contiguous window.

- **`character_SV` player-model actor, `0x8077CD80..0x8077FD80` (30 bodies).**
  Wave 66 translated this class's tail (`0x80780510..0x80782100`) and named it
  from its resource strings. It is the per-player model actor: `character_SV`
  with `g3d/model.brres`, a 21-entry animation-handle array at `+0x28C`
  (`+0x290` = `0x15`) and a `+0x1D0` state word. This pass contributes:
  - the **in-place constructor** [`fn_8077d960`] (base constructor
    `0x800F5FD0`, class vtable `0x80818518` at `+0x60`, interface vtable
    `0x808185A0` at `+0x214`, three `-1` sentinels) and the **heap
    constructor** [`fn_8077e150`] that builds the player view through
    `0x800D6CC0`/`0x800D6DF0`, allocates the 21 slots and registers the two
    resource strings with `0x800F6710`;
  - the **idle/rebound pair** [`fn_8077e570`]/[`fn_8077e5e0`] plus the
    state-`0` reset [`fn_8077e680`], which together own the model vtable slots
    `0x5C`/`0x84` and the `0x800B2F10` frame-index update;
  - the **landing resolver** [`fn_8077e760`] (ring refresh `0x80780E20`,
    `+0x210` = eight bytes per surviving chain member, the `0x80781880`
    alignment predicate, and the `+0x26C` branch selector) with its state-`2`
    seam [`fn_8077e9d0`] and its position latch [`fn_8077e6d0`];
  - the **respawn chain** [`fn_8077cd80`] (target + working vector seed),
    [`fn_8077ce20`]/[`fn_8077cf60`] (virtual-gated re-arm and animation
    rebind), [`fn_8077ede0`] (place at the spawned target, sound ids `0`/`1`)
    and the frame waiter [`fn_8077d170`];
  - the **six-step respawn sub-machine** [`fn_8077d7b0`] driven by `+0x304`,
    plus the two trivial virtuals it calls (`0x8077D660`, `0x8077D670`);
  - the **climb and fly states** [`fn_8077cfe0`] (15 frames per chain member,
    direction `3`/`5` from `0x800FC040`, hand effect through `0x80103410`) and
    [`fn_8077d580`] (motion vectors `+1.0f`/`-1.0f` in `z`);
  - the **fall/throw states** [`fn_8077f270`], [`fn_8077f3c0`],
    [`fn_8077f4a0`] (block scale `0.01f` growing to the player scale over
    `30.0f` frames, growth step at `+0x30C`) and [`fn_8077f5b0`];
  - the **direction-offset writers** [`fn_8077f670`]/[`fn_8077f7e0`]
    (`50.0f` per chain member) and [`fn_8077f830`] (block reach `+0xBC`), the
    anchor recorder [`fn_8077f910`] and its driver [`fn_8077fa10`] (the
    `0.1f`/`10.0f`/`1.0f` distance probe against `FLT_EPSILON`);
  - the **terminal transitions** [`fn_8077fb20`] (state `5`),
    [`fn_8077fd50`] (state `12`, model hidden) and [`fn_8077fd80`] (state `6`).

Start with [`fn_8077e760`] for the richest body (it shows the chain-count,
alignment and branch-selection pattern the whole machine uses), [`fn_8077e150`]
for the class layout, and [`fn_8077f670`] for the `2^52` integer-to-double
idiom that Ghidra prints as `CONCAT44` arithmetic.

## Address range

`0x8077CD80..0x8077FD80` in one contiguous window: every function in that
window is translated except the eight listed under "Deliberate skips". After
this pass the `0x8077` bucket still has 95 untranslated functions, mostly the
`cobSandpillar` actor at `0x80777E10..0x80778BF0`, the `0x8077A4B0..0x8077C670`
block and the `0x80772370..0x80775D00` block.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8077CD80` | `fn_8077cd80` | 152 | medium |
| `0x8077CE20` | `fn_8077ce20` | 164 | medium |
| `0x8077CF60` | `fn_8077cf60` | 128 | medium |
| `0x8077CFE0` | `fn_8077cfe0` | 396 | medium |
| `0x8077D170` | `fn_8077d170` | 184 | medium |
| `0x8077D580` | `fn_8077d580` | 224 | high |
| `0x8077D680` | `fn_8077d680` | 300 | medium |
| `0x8077D7B0` | `fn_8077d7b0` | 396 | high |
| `0x8077D960` | `fn_8077d960` | 120 | high |
| `0x8077E150` | `fn_8077e150` | 256 | medium |
| `0x8077E570` | `fn_8077e570` | 112 | medium |
| `0x8077E5E0` | `fn_8077e5e0` | 160 | medium |
| `0x8077E680` | `fn_8077e680` | 80 | high |
| `0x8077E6D0` | `fn_8077e6d0` | 136 | medium |
| `0x8077E760` | `fn_8077e760` | 332 | medium |
| `0x8077E9D0` | `fn_8077e9d0` | 84 | high |
| `0x8077EDE0` | `fn_8077ede0` | 188 | medium |
| `0x8077F0C0` | `fn_8077f0c0` | 204 | medium |
| `0x8077F270` | `fn_8077f270` | 168 | medium |
| `0x8077F3C0` | `fn_8077f3c0` | 52 | high |
| `0x8077F4A0` | `fn_8077f4a0` | 272 | medium |
| `0x8077F5B0` | `fn_8077f5b0` | 188 | medium |
| `0x8077F670` | `fn_8077f670` | 364 | medium |
| `0x8077F7E0` | `fn_8077f7e0` | 76 | high |
| `0x8077F830` | `fn_8077f830` | 212 | high |
| `0x8077F910` | `fn_8077f910` | 252 | medium |
| `0x8077FA10` | `fn_8077fa10` | 264 | medium |
| `0x8077FB20` | `fn_8077fb20` | 60 | high |
| `0x8077FD50` | `fn_8077fd50` | 20 | high |
| `0x8077FD80` | `fn_8077fd80` | 68 | high |

## Deliberate skips

Four functions inside the window were read and rejected rather than guessed:

- `0x8077DC80` is the class's per-frame driver, but the export opens with three
  `unaff_GQR0`/`ldexpf`/`CONCAT13` paired-single load brackets that consume
  `in_f29..in_f31` and `in_ps29_1..in_ps31_1` before the first real statement.
  The frame-step and the incoming PSQ registers cannot be reconstructed, so the
  body would have to be invented. It is called by address only.
- `0x8077E250` (state entry) and `0x8077E410` (the animation selector used by
  almost every body above) are the same paired-single failure: both save and
  restore `unaff_GQR0` with `ldexpf`, and `0x8077E410` also has three
  unreachable blocks removed by Ghidra. Both are therefore called by address
  (`0x8077E410` through the local `call_f64_u32x3` wrapper, which preserves the
  `f1` constant its callers load).
- `0x8077CED0` reads the caller's frame pointer through the `0x802DCD74` /
  `0x802DCDC0` save/restore pair, then reinterprets the high half of the
  returned 64-bit value as the actor. That value is an artifact of the
  paired-single save bracket, not an argument, so the body has no honest Rust
  form; it is called by address from [`fn_8077cd80`] and [`fn_8077ce20`].

Two further candidates inside the same window, `0x8077D660` (4 bytes) and
`0x8077D670` (8 bytes), are trivial `blr`/`return 1` virtuals; they are called
by address from [`fn_8077d7b0`] and were left out only to stay inside the
30-body budget.

The remaining two are readable but were left for a later pass, again for
budget rather than doubt:

- `0x8077D230` (264 bytes) seeds the position and the `+0x1D8` target from two
  argument vectors, calls the paired-single selector `0x8077E410`, compares the
  target with the manager anchor and writes
  `(s16 at +0x294+0xC8) * members + 1` into `+0x2C0`. Its only unresolved piece
  is `0x80781B60`, the off-bucket fallback anchor writer.
- `0x8077D340` (568 bytes) is the state-`3`/`4` frame driver: two
  `0x800F69F0` effect spawns, the `+0x2C0` countdown and the `0x800FC040`
  left/right vector select. Its many `float` locals are ordinary single
  precision, but reconstructing all four branches honestly needs the
  `0x8079EE70`/`0x800F6410` prototypes confirmed first.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 5,612 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 30 were on the
  bucket's untranslated list before writing (the 57 translated `0x8077`
  addresses were re-read first and none is duplicated here).
- The constants cited in [`fn_8077f670`], [`fn_8077f830`], [`fn_8077f4a0`] and
  [`fn_8077fa10`] were read from the REL bytes at
  `file_offset = address - 0x80600000` (`0x807CF670` = 50.0f,
  `0x807CF698` = 0.0f, `0x807CF69C` = 1.0f, `0x807CF6B0` = 0.01f,
  `0x807CF6C0` = -1.0f, `0x807CF6C4` = 100.0f, `0x807CF770` = {0, 1, 3, 2},
  `0x807CF78C` = 60.0f, `0x807CF790` = 30.0f,
  `0x807CF798` = the 8-byte double `0x4330000080000000`,
  `0x807CF7A0` = 0.1f, `0x807CF7A4` = 10.0f, `0x807CF7A8` = FLT_EPSILON).
- `node tools/check-lane-csv.mjs wave100` reports
  `wave100-bases.csv: 30 rows` and `OK`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/bases.rs`; at the end of the pass the crate's remaining errors all
  belong to the other wave-100 lanes (`render/gx_band.rs`, `game/stage.rs`,
  `resource/bres.rs`).
