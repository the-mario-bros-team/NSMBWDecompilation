# Coverage - Wave 104 (`audio::ax`)

## How to read this file

This is the status page for the wave-104 pass over the `audio::ax` lane of the
`main.dol` reconstruction. The `0x80190000` bucket holds 241 inventory entries;
19 were still unclaimed after wave 99. This pass reconstructs 18 of them and
records the last one here with a reason.

The sections answer, in order:

1. **What did this pass translate?** - *Functions* lists every body with its
   address, Rust name, byte size and confidence.
2. **What does the band contain?** - *What this pass covers* groups the bodies
   into the cue/tempo leaves, the two player-SE spawn twins, the cue-record
   publishers, the cue-state/position helpers, the level/pan ramp, the
   scalar-record table and the boot callback trampoline.
3. **What was left out, and why?** - *Deliberate skips* records the one remaining
   address of the bucket.

**Virtual addresses.** All addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`), not file offsets.

**Evidence.** The Ghidra C is under `work/ghidra/main-gb/export/`; the function
index is `work/ghidra/main-gb/export/functions.tsv`. Inline Rust markers follow
`docs/RUST_STYLE.md`: `// observed:`, `// inferred:` and `// unknown:`.

**GQR note.** Six bodies (`0x8019A6D0`, `0x8019B220`, `0x8019BED0`,
`0x8019C370`, `0x8019D480`, `0x8019EDB0`) open in the Ghidra export with an
`unaff_GQR0` / `ldexpf` prologue. Ghidra itself marks the matching `psq_l`
words (`0xE3E10048`, `0xE3C10038`, `0xE3A10028`, `0xE38100C8`) as removed
unreachable blocks. Those loads are the dead tail, not the reachable body, so
the pass translates the visible float arithmetic and omits the prologue.

**Ledger.** The machine-readable rows are `ledger/agents/wave104-ax.csv`
(module `audio::ax`, 18 rows, all `translated`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/audio/ax.rs` | `audio::ax` | 18 | 9,776 | 0 | 18 |

Status split: 18 `translated`, 0 `partial`, 0 `stub`. Address range:
`0x80195510..0x8019F420`.

## What this pass covers

- **Cue/envelope and tempo leaves, `0x80195510..0x801974A0`.** `0x80195510`
  releases the object at `+0x900`, asks the `+0xB0` vtable slot to accept the
  request, then rewrites the stage block at `+0x91C..+0x92C` and pushes the
  level through `0x8019E9A0`. `0x80195A20` closes the `+0x910` stream when its
  id differs from `+0x938`. `0x80196190` builds the four-word level record and
  calls `0x8019ECE0` plus the three `0x802674xx` pan pokes. `0x80196600`
  registers the local `0x801965D0` match callback through `0x802D42D0`,
  `0x80196B30` converts milliseconds through the 64-bit divide
  `0x802DCDD8`, `0x80196BD0` runs the three-path tempo conversion, and
  `0x801974A0` compares two four-word status records.
- **Player-SE spawn twins, `0x80197F00`/`0x80198C30`.** Both 900-byte bodies
  share one implementation: scan the `0x14`-stride cue table, pick the first
  free row or the smallest `+0x74` count, publish through `0x8027B160`, fill
  the level record with `0x8019ECE0`, apply the `0x80196660` flag bits and
  finish with `0x80197400`. They differ only in the `0x52B8`/`0x52B4` versus
  `0x52B0`/`0x52AC` level pair.
- **Cue-record publishers, `0x8019A6D0..0x8019C370`.** `0x8019A6D0` is the
  "FOOT" cue spawn with the two `0x8019F050` interpolations; `0x8019B220`
  splits the three 7-bit pan fields and rewrites four channels; `0x8019BED0`
  is the sixteen-arm state table that ends in `0x8019E7E0`/`0x801962D0`; and
  `0x8019C370` is the 120-frame level curve with two clamped interpolations.
- **State and position, `0x8019E080`/`0x8019EA20`.** The first advances the
  `0x802DCD74` cue object (two lookup pairs, `+0x14` state, `+0x20` seed,
  `+0x4C`/`+0x50` mirrors, `0x8019F090` tick scale and the `+0x910` stream
  rewrite). The second matches the cue's `+0x4C`/`+0x50` window against the
  `[status[1], status[0]]` pair using `countLeadingZeros` and updates the
  window when it does not overlap.
- **Ramp, table and trampoline, `0x8019EDB0..0x8019F420`.** `0x8019EDB0`
  attenuates and clamps the level/pan pair through `0x8019F010`;
  `0x8019D480` fills the eleven-word and five-word scalar records, publishes
  them at slots `0`/`1`/`2`, and applies ten `PLAYER_SE_*` levels; and
  `0x8019F420` parks `r1` in `0x8042A4E8` and calls the callback on the
  `0x8042A4EC` alternate stack.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80195510` | `fn_80195510` | 212 | medium |
| `0x80195A20` | `fn_80195a20` | 460 | medium |
| `0x80196190` | `fn_80196190` | 316 | medium |
| `0x80196600` | `fn_80196600` | 96 | medium |
| `0x80196B30` | `fn_80196b30` | 92 | medium |
| `0x80196BD0` | `fn_80196bd0` | 692 | medium |
| `0x801974A0` | `fn_801974a0` | 196 | medium |
| `0x80197F00` | `fn_80197f00` | 900 | medium |
| `0x80198C30` | `fn_80198c30` | 900 | medium |
| `0x8019A6D0` | `fn_8019a6d0` | 660 | medium |
| `0x8019B220` | `fn_8019b220` | 412 | medium |
| `0x8019BED0` | `fn_8019bed0` | 640 | medium |
| `0x8019C370` | `fn_8019c370` | 348 | medium |
| `0x8019D480` | `fn_8019d480` | 2,052 | medium |
| `0x8019E080` | `fn_8019e080` | 604 | medium |
| `0x8019EA20` | `fn_8019ea20` | 620 | medium |
| `0x8019EDB0` | `fn_8019edb0` | 476 | medium |
| `0x8019F420` | `fn_8019f420` | 100 | medium |

Totals: **18 bodies**; **0 high, 18 medium**; **9,776 bytes**.

`src/audio/ax.rs` gained two host-side helpers that carry no ledger row:
`call_u64_4` (four words in, `r3:r4` result out, for `0x80196B30`) and
`call_f32_word` (one word in, `f1` out, for `0x8019A6D0`). It also gained the
`SDA2_SCALAR_*` constants, which are transcribed by offset with no claimed
role, and the `0x801965D0` / boot-stack constants referenced by raw address.

## Deliberate skips

Reason code **R** means the Ghidra export drops the body (its unreachable
blocks were removed), so the control flow is not recoverable from the export.

| Address | Reason | Note |
|---|---|---|
| `0x8019F490` | R | The 460-byte export reduces to `FUN_802DCD74(); do { } while (true);` after Ghidra removes twelve blocks (`0x8019F508`, `0x8019F540`, `0x8019F550`, `0x8019F5B8`, `0x8019F5D8`, `0x8019F600`, `0x8019F610`, `0x8019F5A8`, `0x8019F61C`, `0x8019F624`, `0x8019F644`). The surviving call and endless loop are not a reconstructible body; a pass with the dropped AI/MMIO blocks would be needed. |

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (sum 9,776 bytes).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every CSV row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/audio/ax.rs`, and its doc block carries the same confidence.
- No address in this CSV is present in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning in
  `src/audio/ax.rs`; the remaining errors name `src/runtime/bt.rs` (another
  lane).
- `node tools/check-lane-csv.mjs wave104` reports `wave104-ax.csv: 18 rows` and
  `OK`.
