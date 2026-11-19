# Coverage - Wave 99 (`audio::ax`, `0x8019` bucket state machine)

## How to read this file

This is the status page for the wave-99 pass over the `audio::ax` lane of the
`main.dol` reconstruction. It answers three questions:

1. **What did this pass translate, and where is it?** - *Functions* gives one
   row per function with address, Rust name, byte size and confidence.
2. **What does the band do?** - *What this pass covers* groups the 27 bodies
   into the stage/stream helpers, the cue-row allocators and list walkers, the
   cue-table destructors and start wrappers, the group-volume switch, and the
   sequence start/fade helpers.
3. **What was left out, and why?** - *Deferred inventory* lists the remaining
   untranslated entries of the `0x80190000` bucket with a reason.

Column meanings:

- **Confidence** - strength of the evidence, not code quality: `high` = the
  guards, arithmetic, call order and stores are explicit; `medium` = the flow
  is explicit but one constant, field role or dropped register operand is
  inferred; `low` = only the instruction sequence is certain (none this pass).
- **Reason codes** (deferred inventory):
  - **D** - duplicate/dead: carried by another module or nothing to
    reconstruct.
  - **F** - float/paired-single kernel too tangled to resolve honestly here.
  - **M** - large body reserved for a later pass.
  - **R** - the Ghidra export drops so much of the body (uninitialised stack
    locals, a local callback label, or a dropped result register) that the
    control flow is not recoverable.
  - **S** - the body only writes a special-purpose register, the stack pointer
    or another non-memory CPU state, so a Rust body would invent behaviour.

**Virtual addresses.** All addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`), not file offsets.

**Evidence.** The Ghidra C is under `work/ghidra/main-gb/export/`; the function
index is `work/ghidra/main-gb/export/functions.tsv`. Inline Rust markers follow
`docs/RUST_STYLE.md`: `// observed:` (visible in the Ghidra output), `// inferred:`
(role guessed from call sites or layout), `// unknown:` (unresolved).

**Ledger.** The machine-readable rows are `ledger/agents/wave99-ax.csv`
(module `audio::ax`, 27 rows, all `translated`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/audio/ax.rs` | `audio::ax` | 27 | 7,048 | 3 | 24 |

Status split: 27 `translated`, 0 `partial`, 0 `stub`, 0 `low`.

Address range: `0x801955F0..0x8019ECE0`.

## What this pass covers

- **Stage fade and stream levels, `0x801955F0`/`0x801962D0`.** The first body
  owns the `+0x91C..+0x92C` block: a signed countdown in `OSBusClock / 4000`
  units, the start tick at `+0x920`, a level at `+0x924`, a tag at `+0x928`
  and the player-audio byte at `+0x92C`. When the countdown reaches its end it
  re-applies the stage level through `0x8019E9A0` and may restart cue `9`
  through `0x8019E4C0`/`0x8019CA00`. The second body pushes one level to both
  stream slots and mirrors it into the sequence handle's `+0x44`.
- **Cue-name and status helpers, `0x801964F0`/`0x80196690`.** `0x801964F0`
  resolves a cue name through `0x80275740`, maps it to a BRSAR group through
  `0x80278780`, and writes the level/pan pair with `0x8027A8F0`. `0x80196690`
  tests the three stream slots (`+0x904`/`+0x910`/`+0x908`) and returns the
  `0x80275760` status halfword only while the sequence's `+0x58` word is set.
- **Eight-byte cue-row allocators, `0x80197BF0`/`0x8019A490`/`0x8019A310`.**
  Each enters the sound critical section, receives an `(id, table)` pair in
  `r3:r4`, scans the `+0x54` rows of stride eight, stores the `r2 - 0x52B8` or
  `r2 - 0x52A8` default into a free row, and publishes the row with
  `0x8027B160`/`0x8027B1C0`, `0x80196190` and `0x80197400`. `0x8019A310` adds
  the `0x80272C40`/`0x80272A10`/`0x80272D40` sequence record pair.
- **Four-list token and player-jump releases, `0x80197E30`, `0x80198AE0`,
  `0x80199810`, `0x8019ABB0`.** The three token variants cache the pair at
  `r13 - 0x51F0`/`-0x51EC` or `r13 - 0x51E8`/`-0x51E4` and release every
  record whose object id (`handle + 0x9C`) matches. `0x8019ABB0` instead runs
  the `"PLAYER_SE_PLY_JUMP"` comparison (via `0x8019AB20`) over all four
  lists. The list heads are `base + 0x0C + 0x10 * i` and their records start
  `0x108` bytes before the link word.
- **Cue-table destructor, `0x80198BB0`.** Clears the 0x14-stride / 14-element
  vector at `+0x64` with the `0x80069A00` destructor, runs the cue-object
  destructor `0x80197380`, and optionally frees the object.
- **BRSAR row publishers, `0x80198660`/`0x80199390`.** These use 0x14-stride
  rows, match by object id and the caller's tag at `+0x68`, enforce the
  `0x80275760 & 0xF` per-id row limit (zero meaning four), and store the
  `r2 - 0x52B8` (resp. `-0x52B0`) default in a new row.
- **Cue lookup/start wrappers, `0x8019A970`/`0x8019AA70`.** Both index the
  `0x802F5FA0` table with `row * 4 + bank * 0x11C`, then use cue-table vtable
  slots `+0x1C`/`+0x24` or `+0x28`. The started stream gets `r2 - 0x529C` in
  mode `3`, otherwise `r2 - 0x52A8`.
- **Group-volume switch, `0x8019CA00`.** Stores the active id at `+0x34`, sets
  bit 0 of `+0x30`, and walks the loaded BRSAR groups through
  `0x80278780`/`0x8027A850`. The nine switch arms select the mute mask, the
  level word and the sequence parameter (`0xF`, `0x14` or `0x1E`), and
  `0x80196150` is poked for ids `1`/`9` when `0x80196600(manager, 0xAF)` is
  set.
- **Cue-state and sequence helpers, `0x8019D2D0`, `0x8019DF40`,
  `0x8019E2E0`, `0x8019E4C0`, `0x8019E6F0`, `0x8019E800`, `0x8019E8E0`,
  `0x8019ECE0`.** These cover the group-`0x12` list release, the four-byte
  tail branch to `0x8019E800`, the slot-guarded sequence start, the
  cue-mask-byte switch, the two `0x8027F890` stream-level refreshes, the
  eight-word `0x80195510` envelope call, and the flag-complement record
  applier built on `0x8019EDB0`/`0x8019EF90`.
- **`0x8019BAF0`.** The cue-state level/envelope switch (states `5`, `7`,
  `0x14` and `6`), with modes `1`, `4` and `8` through `0x8019E9A0` and
  `0x8019E7E0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801955F0` | `fn_801955f0` | 368 | medium |
| `0x801962D0` | `fn_801962d0` | 116 | medium |
| `0x801964F0` | `fn_801964f0` | 92 | medium |
| `0x80196690` | `fn_80196690` | 260 | medium |
| `0x80197BF0` | `fn_80197bf0` | 236 | medium |
| `0x80197E30` | `fn_80197e30` | 200 | medium |
| `0x80198660` | `fn_80198660` | 520 | medium |
| `0x80198AE0` | `fn_80198ae0` | 200 | medium |
| `0x80198BB0` | `fn_80198bb0` | 120 | high |
| `0x80199390` | `fn_80199390` | 520 | medium |
| `0x80199810` | `fn_80199810` | 200 | medium |
| `0x8019A310` | `fn_8019a310` | 384 | medium |
| `0x8019A490` | `fn_8019a490` | 236 | medium |
| `0x8019A970` | `fn_8019a970` | 256 | medium |
| `0x8019AA70` | `fn_8019aa70` | 164 | medium |
| `0x8019AB20` | `fn_8019ab20` | 132 | medium |
| `0x8019ABB0` | `fn_8019abb0` | 140 | medium |
| `0x8019BAF0` | `fn_8019baf0` | 172 | high |
| `0x8019CA00` | `fn_8019ca00` | 620 | medium |
| `0x8019D2D0` | `fn_8019d2d0` | 336 | medium |
| `0x8019DF40` | `fn_8019df40` | 4 | high |
| `0x8019E2E0` | `fn_8019e2e0` | 476 | medium |
| `0x8019E4C0` | `fn_8019e4c0` | 552 | medium |
| `0x8019E6F0` | `fn_8019e6f0` | 228 | medium |
| `0x8019E800` | `fn_8019e800` | 216 | medium |
| `0x8019E8E0` | `fn_8019e8e0` | 92 | medium |
| `0x8019ECE0` | `fn_8019ece0` | 208 | medium |

Totals: **27 translated, 0 stubs**; **3 high, 24 medium, 0 low**; **7,048
bytes**.

`src/audio/ax.rs` also gained four internal host-side helpers that carry no
ledger row: `call_f32_3` (`f32 + 3` words), `call_f64_2` (`f64 + 2` words),
`call_f64x2_6` (two `f64` + six words, for `0x80195510`) and the shared
`wave99_release_matching`, `wave99_start_stream` and `wave99_for_each_group`
helpers. They mirror the existing `call_f32_1`/`call_f32_2` adapters.

## Deferred inventory

The bucket holds 241 `main.dol` entries; 195 were already in
`ledger/functions.csv` when this pass started, leaving 46. This pass
reconstructs 27; the 19 below stay untranslated.

| Address(es) | Reason | Note |
|---|---|---|
| `0x80195510` | M | 212-byte eight-word cue/envelope helper; the two `f64` arguments and the `FUN_802DCD70` pair are visible, but the tail passes a `r2` word through six register slots and is better done with the untraced `0x802DCD70` pair. |
| `0x80195A20` | R | 460-byte cue selector; `local_1c` is read as the third `0x80196B30` argument without ever being written, so the operand is not recoverable. |
| `0x80196190` | R | 316-byte row publisher; the `+0x94C` record is built on the stack and `local_2c` is read after a 12-byte fill, so one word of the `0x8019ECE0` record is uninitialised in the export. |
| `0x80196600` | R | 96-byte query wrapper; it registers `&LAB_801965D0`, a code label inside itself, as the callback, which cannot be named honestly from Rust. |
| `0x80196B30` | R | 92-byte signed 64-bit divide helper; the result is returned in the dropped `r4` (`extraout_r4`), so the return value is not recoverable. |
| `0x80196BD0` | M | 692-byte tempo/position computation over three `sdata2` doubles and a `countLeadingZeros` normalisation; a later pass. |
| `0x801974A0` | R | 196-byte comparison helper; it reads `local_1c`/`local_20`/`local_3c` from two stack records that the export never fills. |
| `0x80197F00`, `0x80198C30` | M | The two 900-byte player-SE spawn twins; they combine the eight-byte row allocator with the full stream-record copy and need the `0x8019EDB0` kernel first. |
| `0x8019A6D0` | F | 660-byte cue spawn built over GQR-quantised paired-single stores and three `ldexpf` calls; field roles and scale are not recoverable without guessing. |
| `0x8019B220` | F | 412-byte cue publish with the same GQR/paired-single prologue and `ldexpf` scaling. |
| `0x8019BED0` | M | 640-byte 16-case cue-state table with a GQR prologue; the per-case level words are unnamed `sdata2` slots. |
| `0x8019C370` | F | 348-byte frame-based level curve over two `0x8019F050` interpolations and six unnamed limits. |
| `0x8019D480` | M | 2,052-byte per-group scalar table with 16 `0x802D5080`/`0x802D5090` descriptor writes; reserved for a later pass. |
| `0x8019E080` | M | 604-byte cue-state transition used by the wave-18 hook; it interleaves the two cue tables with the sequence handle and is a pass of its own. |
| `0x8019EA20` | R | 620-byte position matcher; it reads `local_14`/`local_18`/`local_34` from a record that the export does not fully initialise. |
| `0x8019EDB0` | F | 476-byte level/pan ramp kernel with `ABS`, two named-mode clamps and four unnamed `sdata2` limits; the two-argument `0x8019F010` curve is not resolved here. |
| `0x8019F420` | S | 100-byte callback trampoline; it writes the old stack pointer to `DAT_8042A4E8` and calls the callback, which is not expressible as a Rust argument. |
| `0x8019F490` | R | 460 bytes that the export reduces to `FUN_802DCD74(); do { } while (true);`; the loop body was dropped. |

The six `0x801A9BC0..0x801A9C10` special-purpose-register leaves belong to the
`0x801A` bucket and are already covered by the aggregate `runtime::os` row
`clear_performance_monitor`; this pass adds no rows for them.

## Uncertainties

- **List node layout.** `0x8027A160` receives `node - 0x108` (the token/ID
  walkers) or `node - 0xF8` (`0x8019D2D0`); the two record sizes are inferred
  from the surrounding code, not from a symbol.
- **`sdata2` levels.** All `r2 - 0x52xx` and `r2 - 0x51xx` words are unnamed in
  the image. They are named by the role they play in this band, and every
  function that reads one marks the role `// inferred:`.
- **Dropped call operands.** `0x802667D0` is printed with no receiver in two
  places and is restored to the guarded slot. `0x802674B0` prints only the
  `f1` value in one branch; the guard's stream word is used as its second
  register argument.
- **`0x80196190` callers.** The three row publishers call the untranslated
  `0x80196190` by address; its own body is in the deferred inventory.
- **`0x8019E2E0` sequence start.** The two id words passed to `0x80195490` are
  not consumed by its translated body, so the call is written with the manager
  only and the loss is noted in the function doc.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in `src/audio/ax.rs`.
- No address in this CSV is present in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning in
  `src/audio/ax.rs` (remaining errors/warnings in the workspace name other
  lanes' files).
- `node tools/check-lane-csv.mjs wave99` reports `OK` and lists
  `wave99-ax.csv: 27 rows`.
