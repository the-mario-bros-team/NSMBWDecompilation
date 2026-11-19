# Coverage - Wave 93 (`audio::ax`, `0x80190000` bucket)

## How to read this file

This is the status page for the wave-93 pass over the `audio::ax` lane of the
`main.dol` reconstruction. It answers three questions:

1. **What did this pass translate, and where is it?** - *Functions* gives one
   row per function with address, Rust name, byte size and confidence.
2. **What does the band do?** - *What this pass covers* groups the 28 bodies
   into the four cue-dispatch wrappers, the stream stage/fade helpers, the cue
   object constructor/destructor pair, the presence probes and the two
   interpolation leaves.
3. **What was left out, and why?** - *Deferred inventory* lists the remaining
   untranslated entries of the bucket with a reason.

Column meanings:

- **Confidence** - strength of the evidence, not code quality: `high` = the
  guards, arithmetic, call order and stores are explicit in the Ghidra C;
  `medium` = the flow is explicit but one constant, field role or dropped
  argument is inferred; `low` = mostly guesswork (none this pass).
- **Reason codes** (deferred inventory):
  - **D** - duplicate: carried by another module or an existing ledger row.
  - **F** - float/paired-single kernel too tangled to resolve honestly here.
  - **M** - large body reserved for a later pass.
  - **R** - the Ghidra export drops so much of the body that the control flow
    is not recoverable.

**Virtual addresses.** All addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`), not file offsets.

**Evidence.** The Ghidra C is under `work/ghidra/main-gb/export/`; the function
index is `work/ghidra/main-gb/export/functions.tsv`. Inline Rust markers follow
`docs/RUST_STYLE.md`: `// observed:` (visible in the Ghidra output),
`// inferred:` (role guessed from call sites), `// unknown:` (unresolved).

**Ledger.** The machine-readable rows are `ledger/agents/wave93-ax.csv`
(module `audio::ax`, 28 rows, all `translated`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/audio/ax.rs` | `audio::ax` | 28 | 2,128 | 6 | 22 |

Status split: 28 `translated`, 0 `partial`, 0 `stub`. No `low`.

## What this pass covers

- **Cue dispatch wrappers, `0x80195010..0x80195190`.** Four 124-byte twins that
  take the sound-system object behind `0x802DCD74` (a register-save thunk, so
  `r3`/`r4` are explicit arguments), call vtable slot `+0xB0`, `+0xB4`,
  `+0xBC` or `+0xC0` with `(object, object + 0x240, key)`, and on acceptance
  read the status word for `id` and hand it to `0x8019B220`. The shape is
  confirmed by `0x80195510`, which uses the same slot `+0xB0` with visible
  arguments.
- **Stage and fade helpers, `0x80195CB0`, `0x801964F0`-adjacent band.** The
  five stage words at `+0x91C..+0x92C` (counter, timestamp, level, mode) are
  reset by `0x80195CB0`, written by the fade pair `0x80196550`/`0x80196580`,
  and forwarded to the level helper `0x8019E9A0`. This is the same block the
  wave-29 transport code reads.
- **Cue object lifecycle, `0x80197300`/`0x80197380`, `0x80197400`.** The
  constructor installs vtable `0x8032DFD0`, seeds `+0x54`, `+0x58`, `+0x5C`
  and `+0x60`, and the destructor reverses it through `0x8019ECA0` and
  `0x802754B0`. `0x80197400` publishes the cue record with either the object's
  `+0x58` value or an explicit override.
- **Stream presence probes, `0x80196090..0x80196150`.** Four 60-byte queries
  for the stream slots at `+0x904`, `+0x910`, `+0x908` and `+0x900`. Each
  returns the presence bit and pokes `0x80267420` when the slot is occupied.
  The probe band shares its fields with the stage helpers above, which is why
  `0x8019E9A0` selects between the `+0x908` and `+0x910` variants.
- **Self-contained leaves, `0x80194DA0`, `0x80195490`, `0x80195F90`,
  `0x80199E60`, `0x8019B180`, `0x8019B9F0`, `0x8019CE00`, `0x8019D470`,
  `0x8019EC90`, `0x8019EF90`, `0x8019F010`, `0x8019F050`.** A teardown, a
  reset-pending flag, a BGM-stream command, a mode-band query, a lazy
  companion-word creator, a constant-1 return, a cue-flag reset, a one-word
  store, an empty `blr` leaf, and the clamp plus the two byte-identical line
  interpolators below the DSP bring-up.

The bodies that close dangling call edges are the four dispatch wrappers and
`0x80197400` (they call `0x8019B220`, previously untranslated) and
`0x8019E9A0` (calls the probe band).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80194DA0` | `fn_80194da0` | 68 | medium |
| `0x80195010` | `fn_80195010` | 124 | medium |
| `0x80195090` | `fn_80195090` | 124 | medium |
| `0x80195110` | `fn_80195110` | 124 | medium |
| `0x80195190` | `fn_80195190` | 124 | medium |
| `0x80195490` | `fn_80195490` | 16 | medium |
| `0x80195CB0` | `fn_80195cb0` | 120 | medium |
| `0x80195F90` | `fn_80195f90` | 20 | medium |
| `0x80196090` | `fn_80196090` | 60 | medium |
| `0x801960D0` | `fn_801960d0` | 60 | medium |
| `0x80196110` | `fn_80196110` | 60 | medium |
| `0x80196150` | `fn_80196150` | 60 | medium |
| `0x80196550` | `fn_80196550` | 40 | medium |
| `0x80196580` | `fn_80196580` | 44 | medium |
| `0x80197300` | `fn_80197300` | 120 | medium |
| `0x80197380` | `fn_80197380` | 116 | high |
| `0x80197400` | `fn_80197400` | 148 | high |
| `0x80199E60` | `fn_80199e60` | 40 | high |
| `0x8019B180` | `fn_8019b180` | 72 | medium |
| `0x8019B9F0` | `fn_8019b9f0` | 8 | high |
| `0x8019CE00` | `fn_8019ce00` | 112 | medium |
| `0x8019D470` | `fn_8019d470` | 8 | high |
| `0x8019E940` | `fn_8019e940` | 84 | medium |
| `0x8019E9A0` | `fn_8019e9a0` | 120 | medium |
| `0x8019EC90` | `fn_8019ec90` | 4 | high |
| `0x8019EF90` | `fn_8019ef90` | 124 | medium |
| `0x8019F010` | `fn_8019f010` | 64 | medium |
| `0x8019F050` | `fn_8019f050` | 64 | medium |

Totals: **28 translated, 0 stubs**; **6 high, 22 medium, 0 low**;
**2,128 bytes**.

## Deferred inventory

The bucket holds 241 `main.dol` entries; 167 were already in
`ledger/functions.csv` when this pass started, leaving 74. This pass
reconstructs 28; the 46 below stay untranslated.

| Address(es) | Reason | Note |
|---|---|---|
| `0x80195510`, `0x801955F0`, `0x80195A20`, `0x80196190`, `0x80196690`, `0x801974A0` | M | 212..460-byte stage drivers whose operand lists are partly dropped (several call `0x8019E4C0`/`0x8019CA00`, which remain untranslated). |
| `0x801962D0` | R | The second branch passes the float argument where the first passes the handle, so the receiver of `0x802674B0` is not recoverable. Left as an opaque callee of `fn_8019e940`. |
| `0x801964F0`, `0x80196B30` | R | Both exports model a `undefined8`/64-bit argument pair that the callee reads as two registers; the split is not recoverable. |
| `0x80196600`, `0x80196BD0` | R | 96/692-byte bodies whose stack temporaries are printed uninitialised (`local_2c` is read before any store). |
| `0x80197BF0`, `0x80197E30`, `0x80199810`, `0x80198AE0`, `0x80198BB0` | M | Cue-list pumps over the four `+0x74` buckets; they need the JAS object walker `0x8027A160`/`0x8027A200` modelled first. |
| `0x80197F00`, `0x80198660`, `0x80198C30`, `0x80199390` | M | Four 520..900-byte state machines with recovered jump tables. |
| `0x8019A310`, `0x8019A490`, `0x8019A6D0`, `0x8019A970`, `0x8019AA70`, `0x8019AB20`, `0x8019ABB0` | F/M | The archive/stream readers and cue-table walks; several operate on `f64` records and the table stride is inferred. |
| `0x8019B220` | M | The cue publisher itself (412 bytes); it is the natural next cluster because this pass now calls it from five bodies. |
| `0x8019BAF0`, `0x8019BED0` | M | The two bank walks (172/640 bytes) behind the cue publisher. |
| `0x8019C370`, `0x8019CA00`, `0x8019D2D0` | M | 348/620/336-byte cue control paths that call the still-untranslated `0x8019E2E0` and `0x8019E4C0`. |
| `0x8019D480` | M | 2,052-byte JAS bank loader; a pass of its own. |
| `0x8019DF40` | D | 4-byte tail-call thunk to `0x8019E800`; no separate body to reconstruct. |
| `0x8019E080`, `0x8019E2E0`, `0x8019E4C0`, `0x8019E6F0`, `0x8019E800`, `0x8019E8E0` | R/M | The sound-mode state machines; `0x8019E8E0` needs the nine-argument `0x80195510` call with two float registers, and the others pass float records through untyped varargs. |
| `0x8019EA20`, `0x8019ECE0`, `0x8019EDB0` | M | The three cue-table builders (620/208/476 bytes). |
| `0x8019F420`, `0x8019F490` | R | `0x8019F420` is printed as a two-statement stack tweak, `0x8019F490` collapses to `do { } while (true)`; most of both bodies was dropped by Ghidra. |

## Uncertainties

- **Float-first calls.** `0x802610C0`, `0x80267420` and `0x801962D0` are
  printed by Ghidra with a `(double)*(float *)` first argument. The Rust
  adapters `call_f32_1`/`call_f32_2` pass the loaded `sdata2` word in `f1` as a
  single-precision value; if the real prototype takes a `double` the call
  would need the `f64` conversion instead. This is the one place where the ABI
  is inferred rather than observed, so every body using an adapter is `medium`.
- **Inherited operands.** `0x80266440`, `0x80267420`, `0x802610C0` and
  `0x80195820` are all printed with empty argument lists in at least one call
  site. Wave 234 traced the still-live registers at each `bl` and gave the four
  presence probes (`0x80196090`, `0x801960D0`, `0x80196110`, `0x80196150`) the
  `(value, manager, duration)` parameters they always received in `f1`/`r4`/
  `r5`; the four `0x80267420` pokes now forward them instead of carrying a
  `// unknown:` note. Rows `0x80195490` and `0x80195F90` still document their
  missing registers as `// unknown:`.
- **`0x801960D0` signature.** Its body tests `+0x910`, and its caller
  `0x8019E9A0` leaves a float and a third word live. Wave 234 therefore gave the
  whole probe band the `(value: f32, manager: u32, duration: u32)` signature its
  ABI already had, and `0x8019E9A0` forwards `value`/`param`; the export omits
  them because the wrappers never write `f1`/`r4`/`r5`.
- **`0x8019B180` (lazy companion word).** The export allocates `0x10` bytes
  and calls the `0x58`-byte sound constructor `0x8019B1E0` with no visible
  argument. The allocation is restored as the receiver; the size mismatch is
  recorded in the body doc rather than "fixed".
- **`0x8019EC90` (empty leaf).** The 4-byte body is `blr`, so it returns with
  `r3` unchanged. `0x80197300` stores that value at `+0x5C`; the doc notes that
  this is the allocator's result, not a value produced by the leaf.
- **`0x8019F010`/`0x8019F050`.** Every cast is literal, but the `sdata2` scale
  used when both x endpoints coincide is unnamed; it is documented as the
  flat-case midpoint scale and expected to be `0.5`.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn` body in `src/audio/ax.rs`.
- No address in this CSV is present in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error in `src/audio/ax.rs`.
- `node tools/check-lane-csv.mjs wave93` reports `OK` for `wave93-ax.csv`.
