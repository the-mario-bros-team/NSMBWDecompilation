# Coverage - Wave 99 (`game::bases`, `0x807a` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** under the canonical
synthetic base **`0x80600000`** (produced by `tools/rel2elf.mjs`), never a
runtime address: the Wii REL loader chooses the real load address, so the
numbers only identify positions inside the REL image. In this image a virtual
address maps to the REL file at `file_offset = address - 0x80600000`; that is
how the float and halfword constants cited below were checked. Cross-module
pointers (`0x8037xxxx`, `0x8042xxxx`) keep their absolute values, and the data
pointers at `0x8081xxxx` are relocated at load time.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c` (checked with a glob,
never assumed). "Confidence" is `high` when guards, stores, constants and call
arguments are literal, and `medium` when a field role, a global, a callee or a
virtual slot had to be inferred (those bodies carry `// observed:` /
`// inferred:` / `// unknown:` notes). No body in this pass is `low`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 28 | 2,844 | 19 | 9 |

## What this pass covers

The `0x807a` bucket holds 212 functions; 86 were already in
`ledger/functions.csv` when this pass began, leaving 126. This pass takes 28
from the three clusters described below.

- **`cobSurrender` prop, `0x807A13B0..0x807A1B30` (7 bodies).** The actor is
  identified only by its resource strings: `cobSurrender` (`0x8081C160`),
  `g3d/model.brres` (`0x8081C150`), the relocated clip-name table at
  `0x8081C148` (`cobSurrenderFlap` / appear) and the `flag02` argument string
  (`0x8081C170`). The cluster contributes the constructor [`fn_807a13b0`]
  (vtable `0x8081C178`, resource block `+0x188`, animation object `+0x1A8`,
  `0x802DC9A0` table `+0x1E8`), the model/animation loader [`fn_807a1610`]
  with both `0x38`-byte clip slots at `+0x1E8`/`+0x220`, the start
  [`fn_807a14e0`] (position snapshot and the `100.0f` respawn timer), the
  transform writer [`fn_807a17f0`], the player-state predicate
  [`fn_807a18a0`], and the appear/flap effect pair
  [`fn_807a1a90`]/[`fn_807a1b30`] with sound ids `0x27`/`0x28`.
- **Lifecycle helpers and motion-set entries, `0x807A61E0..0x807A7550`
  (9 bodies).** A registered destructor [`fn_807a61e0`], the `0x802B9280`
  release wrapper [`fn_807a6790`], the detach-then-free helper
  [`fn_807a6f50`], the mirrored motion records [`fn_807a6430`] (motion set
  `2`, `+136.0f`) and [`fn_807a6640`] (motion set `3`, `-136.0f`), the
  class-table store [`fn_807a6780`], the animation-counter refresh
  [`fn_807a7170`], the constant-`1` virtual [`fn_807a71c0`] and the
  state-clearing [`fn_807a7550`].
- **Camera/player-scan tail, `0x807A7860..0x807A80D0` (12 bodies).** The
  state seams [`fn_807a7860`] (state `1`), [`fn_807a78c0`] (state `2`) and
  [`fn_807a7cf0`] (state `6`), the player-list walk through `0x800F6020`
  ([`fn_807a7df0`] dispatches virtual `+0x6C`, [`fn_807a7d70`] queries virtual
  `+0x64`), the counter reset/re-baseline pair
  [`fn_807a7e60`]/[`fn_807a7eb0`], the event dispatch
  [`fn_807a7f10`]/[`fn_807a7f80`] driven by `0x801015E0`, the four-timer query
  [`fn_807a7fb0`], the second class-table store [`fn_807a80c0`] and the
  `0x802B9280` free wrapper [`fn_807a80d0`].

Start with [`fn_807a1610`] for the richest body (model, clips and the two-slot
selection), [`fn_807a7d70`] for the player-list iteration pattern, and
[`fn_807a7f10`] for the event-driven state counter.

## Address range

`0x807A13B0..0x807A1B30` (7 bodies), `0x807A61E0..0x807A7550` (9 bodies) and
`0x807A7860..0x807A80D0` (12 bodies). After this pass the `0x807a` bucket still
has 98 untranslated functions.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x807A13B0` | `fn_807a13b0` | 124 | high |
| `0x807A14E0` | `fn_807a14e0` | 108 | high |
| `0x807A1610` | `fn_807a1610` | 468 | medium |
| `0x807A17F0` | `fn_807a17f0` | 176 | medium |
| `0x807A18A0` | `fn_807a18a0` | 140 | medium |
| `0x807A1A90` | `fn_807a1a90` | 152 | medium |
| `0x807A1B30` | `fn_807a1b30` | 280 | high |
| `0x807A61E0` | `fn_807a61e0` | 88 | high |
| `0x807A6430` | `fn_807a6430` | 100 | high |
| `0x807A6640` | `fn_807a6640` | 100 | high |
| `0x807A6780` | `fn_807a6780` | 16 | high |
| `0x807A6790` | `fn_807a6790` | 64 | high |
| `0x807A6F50` | `fn_807a6f50` | 88 | high |
| `0x807A7170` | `fn_807a7170` | 68 | medium |
| `0x807A71C0` | `fn_807a71c0` | 8 | high |
| `0x807A7550` | `fn_807a7550` | 12 | high |
| `0x807A7860` | `fn_807a7860` | 72 | high |
| `0x807A78C0` | `fn_807a78c0` | 12 | high |
| `0x807A7CF0` | `fn_807a7cf0` | 52 | high |
| `0x807A7D70` | `fn_807a7d70` | 120 | medium |
| `0x807A7DF0` | `fn_807a7df0` | 100 | medium |
| `0x807A7E60` | `fn_807a7e60` | 76 | high |
| `0x807A7EB0` | `fn_807a7eb0` | 96 | medium |
| `0x807A7F10` | `fn_807a7f10` | 104 | medium |
| `0x807A7F80` | `fn_807a7f80` | 48 | high |
| `0x807A7FB0` | `fn_807a7fb0` | 92 | high |
| `0x807A80C0` | `fn_807a80c0` | 16 | high |
| `0x807A80D0` | `fn_807a80d0` | 64 | high |

## Deliberate skips

The candidates below were read and rejected rather than guessed; the rest of
the bucket was simply not reached.

- `0x807A7C40` builds a twelve-byte camera name with
  `0x802E16E8(buffer, 8, "cameraA")` and then increments the C local
  `local_12`, while the same buffer is passed on as `auStack_18`. The two
  stack slots overlap in the export and their order cannot be resolved, so the
  byte that is incremented (the first character of the name or a separate
  suffix slot) is not provable. No body was emitted.
- `0x807A58E0` and `0x807A6BE0` use the paired-single save bracket: the export
  contains `unaff_GQR0`, `ldexpf` and `CONCAT44` sequences, so the frame steps
  cannot be reconstructed. Their callers reach them by address only.
- `0x807ABF40`, `0x807ADBF0` and `0x807AEE90` exist only as `halt_baddata()`
  (8 bytes each); the export truncates the control flow, so there is nothing
  honest to translate.
- The 48-byte `0x802DCBFC` forwarders `0x807A3820`, `0x807A3850`,
  `0x807A4F00`, `0x807A4F30`, `0x807A6300`, `0x807A6330`, `0x807ADF40`,
  `0x807ADF70`, `0x807AF1E0` and `0x807AF210` were skipped under the wave-97
  policy of not re-emitting that stub.
- Not reached: the remaining `0x807A` scene/demo bodies, including
  `0x807A0090` (560), `0x807A02C0` (428), `0x807A7220` (816),
  `0x807A84B0` (1,124), `0x807ABF50` (460) and `0x807ADC00` (460).

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (28 rows, 2,844 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 28 were on the
  bucket's untranslated list before writing.
- The constants cited in [`fn_807a14e0`], [`fn_807a1610`], [`fn_807a6430`]
  and [`fn_807a6640`] were read from the REL bytes at
  `file_offset = address - 0x80600000` (`0x807D02D8` = 1, `0x807D02DC` = 0,
  `0x807D02E0` = 100.0f, `0x807D02E4` = 0.0f, `0x807D02E8` = 1.0f,
  `0x807D02EC` = 5.0f, `0x807D0560` = 0.0f, `0x807D0564` = 136.0f,
  `0x807D0568` = 32.0f, `0x807D056C` = 272.0f, `0x807D0570` = 0.0f,
  `0x807D0574` = -136.0f, `0x807D0578` = 32.0f, `0x807D057C` = 272.0f).
- `node tools/check-lane-csv.mjs wave99` reports `wave99-bases.csv: 28 rows`,
  `translated/high=19 translated/medium=9` and `OK`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/bases.rs`; the only errors in the crate at the end of the pass
  belong to the other wave-99 lanes. The file was also compiled on its own
  against `src/platform.rs` in a scratch crate, with no errors.
