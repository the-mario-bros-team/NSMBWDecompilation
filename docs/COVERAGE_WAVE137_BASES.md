# Coverage - Wave 137 (`game::bases`)

## How to read this file

This page documents the Wave 137 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`**, produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
`# Original` block in the source carries the
`Synthetic base: 0x80600000; this is not a runtime address.` line. Cross-module
cells (`0x8037xxxx`, `0x8042xxxx`) keep their absolute `main.dol` values; this
REL's own `.data`/`.bss` cells keep their `0x807Cxxxx` / `0x807Dxxxx` /
`0x807Exxxx` / `0x8080xxxx` / `0x8082xxxx` synthetic values.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee contract, a field role or a zero-fill cell's
meaning had to be inferred. Comments in the bodies keep
`// observed:` / `// inferred:` / `// unknown:` markers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 27 | 2,040 | 21 | 6 |

All 27 rows are `translated`; none is `partial`. The module header now reports
3492 bodies (3465 + 27).

## What this pass covers

- **The class-B pool of `docs/SPEC_WAVE136_BASES.md` is now empty (10 bodies,
  628 bytes).** Wave 136 had already merged eleven of that specification's
  twelve addresses; this lane wrote the stragglers and the nine hand-off
  addresses the specification itself listed for the next pass:

  | cluster | bodies |
  |---|---|
  | `+0x18` teardown | `0x806FCCA0` (the one address wave 136 deferred) |
  | release-only wrappers | `0x806FCC60`, `0x806FCD70`, `0x806FCDB0`, `0x80600AD0`, `0x80601D10`, `0x80601EB0`, `0x80605500` |
  | tail dispatch | `0x8069ACA0` halfword stamp + `+0xD4` tail call |
  | sub-object reset | `0x806FF700` `+0x3C0` reset returning `1` |

- **Seventeen `0x806E` planner-bucket bodies (1,412 bytes)**, taking that bucket
  from 87 to 70 untranslated addresses:

  | cluster | bodies |
  |---|---|
  | release destructors | `0x806E0720`, `0x806E1600`, `0x806E5940`, `0x806E6A90`, `0x806E9A60`, `0x806EFD70` (all six are the `fn_806511d0` shape) |
  | vector/float helpers | `0x806E3380` float-triple copy, `0x806E6FB0` / `0x806E6FE0` latch-then-add pair |
  | resets / wrappers | `0x806E7F20` `+0x5AC` reset, `0x806E9250` spawn-state reset, `0x806E7A30` transform seed + resource bind |
  | dispatch / model | `0x806E4F10` gated tail dispatch, `0x806E7010` flag-selected model swap, `0x806E81F0` position promote |
  | update / compare | `0x806E6450` landing-smoke update, `0x806E9CF0` colon-suffix comparison |

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80600AD0` | `fn_80600ad0` | 64 | high |
| `0x80601D10` | `fn_80601d10` | 64 | high |
| `0x80601EB0` | `fn_80601eb0` | 64 | high |
| `0x80605500` | `fn_80605500` | 64 | high |
| `0x8069ACA0` | `fn_8069aca0` | 40 | high |
| `0x806E0720` | `fn_806e0720` | 88 | high |
| `0x806E1600` | `fn_806e1600` | 88 | high |
| `0x806E3380` | `fn_806e3380` | 28 | high |
| `0x806E4F10` | `fn_806e4f10` | 28 | high |
| `0x806E5940` | `fn_806e5940` | 88 | high |
| `0x806E6450` | `fn_806e6450` | 128 | medium |
| `0x806E6A90` | `fn_806e6a90` | 88 | high |
| `0x806E6FB0` | `fn_806e6fb0` | 36 | high |
| `0x806E6FE0` | `fn_806e6fe0` | 40 | high |
| `0x806E7010` | `fn_806e7010` | 112 | medium |
| `0x806E7A30` | `fn_806e7a30` | 112 | medium |
| `0x806E7F20` | `fn_806e7f20` | 40 | high |
| `0x806E81F0` | `fn_806e81f0` | 104 | medium |
| `0x806E9250` | `fn_806e9250` | 120 | medium |
| `0x806E9A60` | `fn_806e9a60` | 88 | high |
| `0x806E9CF0` | `fn_806e9cf0` | 136 | medium |
| `0x806EFD70` | `fn_806efd70` | 88 | high |
| `0x806FCC60` | `fn_806fcc60` | 64 | high |
| `0x806FCCA0` | `fn_806fcca0` | 100 | high |
| `0x806FCD70` | `fn_806fcd70` | 64 | high |
| `0x806FCDB0` | `fn_806fcdb0` | 64 | high |
| `0x806FF700` | `fn_806ff700` | 40 | high |

**Totals:** 27 bodies, 2,040 bytes (21 high, 6 medium). The address window runs
from `0x80600AD0` to `0x806FF700`.

## Where the specification needed adjusting

1. **The pool was already half-merged, not half-skipped.** The wave-136
   specification's "ready" table names twelve addresses and its hand-off names
   nine more. By the time this lane started, eleven of the twelve were already
   `pub unsafe fn`s in `src/game/bases.rs` (wave 136 landed them), so the only
   specification address actually left was `0x806FCCA0` - the same one wave 136's
   own coverage doc listed as a deliberate skip. All nine hand-off addresses
   (`0x80600AD0`, `0x80601D10`, `0x80601EB0`, `0x80605500`, `0x806FCC60`,
   `0x806FCD70`, `0x806FCDB0`, `0x8069ACA0`, `0x806FF700`) were still missing;
   this lane wrote all nine. No specification address was translated twice.
2. **`0x802B9280` is called with one argument.** Every release wrapper in this
   pass (`0x806FCC60`, `0x806FCD70`, `0x806FCDB0`, `0x80600AD0`, `0x80601D10`,
   `0x80601EB0`, `0x80605500`) is emitted as `func_0x802b9280()` by Ghidra. The
   raw stream never rewrites `r3` between entry and the `bl` - only
   `or r31,r3,r3` copies it - so the callee receives entry `r3 = this`. The
   bodies call `abi_call1!(0x802B_9280, this)` and say so in a comment.
3. **`0x806E3380` copies floats, not words.** The export types the operands as
   `undefined4`, but all six memory words are `lfs`/`stfs`. The body uses
   `read_f32`/`write_f32`.
4. **`0x806E4F10`'s receiver is `this+0x3FC`.** The guard is `beqlr` (return on
   zero), and the dispatch begins with `lwzu r12,0x3FC(r3)`, which updates `r3`
   before loading the vtable. `vmethod0(this + 0x3FC, 0x14)` reproduces that;
   passing `this` would be one word off.
5. **`0x806E81F0`'s dispatch object is `this+0x6D0`,** for the same `lwzu`
   reason, and only the `y` coordinate carries the `0x807FDBE0` bias.
6. **`0x806E9CF0` does not null-check the second scan.** `0x802E1C40(other, ':')`
   is added to unconditionally, so a miss compares address `1`. The Rust keeps
   the addition rather than "fixing" it.

## Deliberate skips

Read and rejected, with the reason:

- **`0x806E8990` (16 bytes).** Ghidra reports `halt_baddata()`; there is no
  instruction stream to translate. Left as evidence-only.
- **`0x806E3BE0` (44 bytes).** A hidden-`r12` forwarder: `addi r12,r3,1508` then
  `bl 0x802DCBFC` with the incoming `r3`/`r4` unmodified, returning `1`. The
  same pattern as [`fn_806512f0`], but this one passes no visible argument to
  the stub, so a `platform::call3` rendering would invent the two argument
  words. Not written.
- **The fourteen other 48-byte `0x802DCBFC` forwarders** in the bucket
  (`0x806E0840`/`0870`, `11E0`/`1210`, `1720`/`1750`, `3910`/`3940`,
  `4450`/`4480`, `5A60`/`5A90`, ...). Each differs from its neighbour only in
  the `r12` this-adjust immediate (`+0x18` or `+0x24`) and the branch word. They
  add no new evidence beyond the two already merged, so the budget went to
  bodies that do something visible.
- **`0x806E2BF0` (96 bytes).** A four-iteration float walk over the `0xE0`-stride
  records at `this+0x430` whose only stores go to six dead stack slots; the
  function has no observable effect. Transcribing dead spills as if they were
  state would misrepresent it, so it is left for an address-for-address pass.
- **`0x806E5580` / `0x806E7350` (120 bytes each).** Identical twins that pass
  three stack floats into `0x801C0BD0` with a Ghidra `double` widening; the
  argument shape needs a mixed `f32`/`f64` helper that this file does not have,
  and inventing one was out of proportion for this pass.
- **The rest of the `0x806E` bucket** (70 remaining addresses, up to 880 bytes)
  is untouched: the large ones need their own call graphs.

## Notes on inference

- [`fn_806e6450`] is `medium`: the gate call `0x806E5D60` is exported with no
  parameters (it never reads `r3`), the stack out-parameter written by
  `0x806E5DC0` is modelled as `let mut state`, and the record at `0x8082E290`
  lies in the REL's zero-fill segment.
- [`fn_806e7010`], [`fn_806e7a30`] and [`fn_806e9250`] are `medium` for the same
  class of reason: every offset and constant is literal, but the data cells
  `0x8082E328` / `0x8082E368` / `0x8082E3A8` / `0x8082E290` / `0x8082B5AC` are
  zero-fill, and `0x806E8A30` / `0x806E7900` / `0x80062600` / `0x8007F950` have
  no documented contract in this REL.
- [`fn_806e81f0`] is `medium`: the two `0x807FDBDC` / `0x807FDBE0` constants
  are shipped bytes, but their meaning is not named anywhere, and the promoted
  record is then dispatched with no argument.
- [`fn_806e9cf0`] is `medium`: `0x802E1C40` and `0x802E1AB4` are named
  "colon scan" and "string compare" only from their arguments and result use.
- [`fn_8069aca0`] is `high` for the body: the halfword load, the store and the
  slot are literal; it returns whatever the tail-called slot leaves in `r3`.
- [`fn_806fcca0`] is `high`: it is [`fn_806357e0`] plus one dead `beq`, and the
  dead word is documented in the body rather than dropped silently.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (27 rows, 2,040 bytes),
  re-read from the TSV for this pass rather than from prose. Every body's last
  word was re-read from `work/rels/elf/d_basesNP.elf` at
  `(address - 0x80600000) + 0x1000`; 25 bodies end in `0x4E800020` (`blr`) at
  `entry + size - 4`, and the two tail dispatchers `0x8069ACA0` and `0x806E4F10`
  end in the documented `bcctr` (`0x4E800420`) with no `blr`.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 27 were checked against
  the ledger's address column before being written, and none is duplicated.
- `node tools/check-lane-csv.mjs wave137` reports `OK`: `wave137-bases.csv` has
  27 rows, 27 unique addresses, 21 `high` and 6 `medium`.
- `cargo check --message-format=short` exits 0 and reports no error and no
  warning naming `src/game/bases.rs`.
