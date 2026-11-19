# Coverage - Wave 90 (audio::ax DSP pump and voice kernels `0x801A08F0..0x801A93F0`)

## How to read this file

This is the status page for the wave-90 pass over the audio lane of the
`main.dol` reconstruction. It answers three questions:

1. **What did this pass translate, and where is it?** - *Functions* gives one row
   per function with address, Rust name, byte size and confidence.
2. **How do the functions fit the existing AX/JAS driver?** - *What this pass
   covers* groups them into the block pump, the mixer-state builders and the
   resampler kernels.
3. **What was deliberately left out, and why?** - *Deferred inventory* lists the
   remaining untranslated addresses of the `0x801A0000` bucket with a reason.

Column meanings:

- **Confidence** - strength of the evidence, not code quality: `high` = the
  guards, arithmetic, call order and stores are explicit in the Ghidra C;
  `medium` = the flow is explicit but a field role, a coefficient or an
  `sdata2` constant is inferred; `low` = mostly guesswork (none this pass).
- **Reason codes** (deferred inventory):
  - **D** - duplicate: already translated by another module lane.
  - **F** - float/paired-single kernel too tangled to resolve honestly in this
    pass.
  - **M** - large body reserved for a later pass.
  - **O** - owned by another module (OS, DVD, archive, init), not audio.

**Virtual addresses.** All addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`), not file offsets.

**Evidence.** The Ghidra C is under `work/ghidra/main-gb/export/`; the function
index is `work/ghidra/main-gb/export/functions.tsv`. Inline Rust markers follow
`docs/RUST_STYLE.md`: `// observed:` (visible in the Ghidra output),
`// inferred:` (role guessed from call sites), `// unknown:` (unresolved).

**Ledger.** The machine-readable rows are `ledger/agents/wave90-ax.csv`
(module `audio::ax`, 14 rows, all `translated`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/audio/ax.rs` | `audio::ax` | 14 | 12,024 | 0 | 14 |

## What this pass covers

- **DSP block pump and command builder** (`0x801A08F0..0x801A2E70`). The
  three-bank block pump that publishes six block pointers per frame and services
  each bank through an installed fill callback or a dirty-bank clear, the
  `0x80`-byte big-endian DSP command builder that emits opcodes 0..14, and the
  96-voice table initialiser that links the parameter, auxiliary and DSP state
  tables into one ring.
- **Pan curve and mixer-state builders** (`0x801A3290..0x801A8150`). The
  `sin`/`sqrt` pan-gain helper and the four "voice to mixer" builders: the
  2-tap and 4-tap builders with eight band checks, and the 3-tap and 4-tap
  builders with per-slot peak/gain tables and `pow`-shaped ramp curves.
- **8-bit blend resamplers** (`0x801A5C10`, `0x801A6580`). The 3- and 4-channel
  96-frame resamplers that read the output planes, blend the sample with the
  previous tap and forward the input sample through two gains.
- **Float effect mixers** (`0x801A6D30`, `0x801A7A80`). The 3- and 4-channel
  96-frame chains with direct path, optional sidechain, four one-pole filters
  and a per-channel envelope.
- **FIR resamplers** (`0x801A88B0`, `0x801A93F0`). The 3- and 4-channel 96-frame
  kernels that use the already-translated phase generators [`fn_801a8e10`] and
  [`fn_801a9990`] and a four-tap FIR over a rotating history ring.

Two of these close previously dangling call edges: [`fn_801a34f0`] (wave 5)
calls `0x801A4180`, and [`fn_801a69c0`] calls `0x801A73E0`; both now resolve to
translated bodies in this file.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801A08F0` | `fn_801a08f0` | 940 | medium |
| `0x801A0E90` | `fn_801a0e90` | 2492 | medium |
| `0x801A2E70` | `fn_801a2e70` | 700 | medium |
| `0x801A3290` | `fn_801a3290` | 188 | medium |
| `0x801A4180` | `fn_801a4180` | 804 | medium |
| `0x801A5280` | `fn_801a5280` | 820 | medium |
| `0x801A5C10` | `fn_801a5c10` | 632 | medium |
| `0x801A6580` | `fn_801a6580` | 536 | medium |
| `0x801A6D30` | `fn_801a6d30` | 876 | medium |
| `0x801A73E0` | `fn_801a73e0` | 684 | medium |
| `0x801A7A80` | `fn_801a7a80` | 900 | medium |
| `0x801A8150` | `fn_801a8150` | 688 | medium |
| `0x801A88B0` | `fn_801a88b0` | 864 | medium |
| `0x801A93F0` | `fn_801a93f0` | 900 | medium |

Totals: **14 translated, 0 partial, 0 stubs**; **0 high, 14 medium, 0 low**.

## Deferred inventory

The `0x801A0000` bucket holds 271 `main.dol` functions; 89 were untranslated when
this lane started. Fourteen of those are audio and are translated above. The
remaining 75 are not this module's code; the table lists them by band.

| Address(es) | Reason | Note |
|---|---|---|
| `0x801A3880`, `0x801A48F0` | F | 3x3 and 4x4 float matrix mixers. The export walks three interleaved pointer arrays per channel (`piVar29 += 3`, `piVar30 += 2`); the record layout cannot be resolved without an instruction-level pass. |
| `0x801A9AF0`, `0x801A9B00` | D | Allocator hook get/set pair, already translated by `resource::archive` (wave 73) for `0x80265330`. |
| `0x801A9B80`, `0x801A9BC0..0x801A9C10`, `0x801A9C40`, `0x801A9CA0`, `0x801A9CE0` | O | Empty `blr` entry points and the performance-monitor clears, owned by `runtime::os`. |
| `0x801A9C20`, `0x801A9C90` | O | FPSCR snapshot and `sync`, owned by `runtime::os`. |
| `0x801A9E70`, `0x801A9EE0` | O | RTC timestamp formatting and DVD error mapping, owned by `runtime::os`/`runtime::dvd`. |
| `0x801AB1B0..0x801AB260` | O | DI status byte and boot-argument string builders, owned by `runtime::os`/`runtime::init`. |
| `0x801ABCF0`, `0x801ABD70`, `0x801ABE60..0x801ABF20` | D | JAI block unlink, arena get/set words and the cache/heap helpers; wave 31 and wave 40 already assigned these to `runtime::os` and `resource::archive`. |
| `0x801ABF60..0x801AC350` | O | EXI/DVD register bring-up (`0xCD800180`, `0xCC005000`), owned by `runtime::dvd`. |
| `0x801AC710..0x801AD...` | O | Cache, exception and context helpers, owned by `runtime::os`/`runtime::init`. |

## Uncertainties

- **`sdata2` constants.** Every kernel reads unnamed `f32`/`f64` words at
  `SDA2_BASE - N`. Their roles (limit, scale, cap, `pow` base) are inferred from
  the comparisons and multiplications around them and are named `SDA2_*` with
  that caveat in the constant docs.
- **Callback argument vectors.** `0x801A08F0` passes the address of a stack
  vector of block pointers to the installed fill callback; the Rust body builds
  the same 3- or 4-word vector in a local array and passes its address.
- **Pointer/register mapping.** The mixer builders take their object pointer
  from the `0x802DCD*` save prologue, so the Rust signatures make it explicit.
  The resampler kernels take the incoming `r3` plane vector and `r4` state the
  same way.
- **Bank stride asymmetry.** In `0x801A08F0` the dirty-clear path indexes the
  block with `0x180` and the callback path with `0x600`. Both strides are
  reproduced literally and flagged `// observed:`; one of them may be a
  decompiler scaling artefact.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/audio/ax.rs`.
- No address was already in `ledger/functions.csv` when this pass started.
- `cargo check --message-format=short` reports no error in `src/audio/ax.rs`.
- `node tools/check-lane-csv.mjs wave90` reports OK for `wave90-ax.csv`.
