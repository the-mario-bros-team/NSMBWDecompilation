# Coverage - Wave 93 (`game::stage`, the remaining `0x800C0000` helpers)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The evidence
column of `ledger/agents/wave93-stage.csv` points at the Ghidra C export each
body was written from. Roles the export only hints at are marked `inferred` or
`unknown` in the body.

This pass closes most of the `0x800C0000` bucket that does not depend on the
paired-single (GQR) math chain, on stack-passed varargs, or on argument lists
the export cannot recover. It also finishes the mask/message record cluster
that wave 71 started at `0x800CDA70`. It reuses the field vocabulary those
waves established:

* **`+0x6C` transition interface** and the `+0x18` slot used by
  [`w38_transition`], the same handoff as the wave-38 circle handlers.
* **`+0x38..+0x69` lift-arm block** written by [`fn_800c10f0`] directly above
  the `0x38`-byte interpolation block that [`fn_800c1810`] clears.
* **`+0x28` / `+0x30` mask-overlay children**: the `0xC`-byte helper
  ([`fn_800cb470`]) and the `0x34`-byte cave-mask record
  ([`fn_800cafd0`]), bound together by `0x800CB490`.
* **`0x8035xxxx` data**: the `0x80359CB0` message-ready flag, the
  `0x80359C9C` `daMessage` record, the `0x80359538` text buffer and the
  `r13-0x56C0` message-record pointer.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 25 | 5272 | 18 | 7 |

No body in this pass is `low` confidence and none is `partial`; every row is
`translated`.

## What this pass covers

- **Lift-arm initialiser** (`0x800C10F0`). Clears the arm's `0x38`-byte
  interpolation block through `0x800C1810`, then writes `+0x38`/`+0x3C` zero,
  the start point into `+0x40..+0x4C`, the speed at `+0x60`, the two
  grid-snapped coordinates derived from the `r2-0x6848` step, the
  `+0x66..+0x69` mode bytes and the `0x80358E10` transition.
- **Message pipeline** (`0x800C9E80`, `0x800CDB30`, `0x800CDC70`). The
  eight-argument `0x800E71C0` formatter plus the `+0x80` writer slot, the
  one-shot `Message/Message.arc` + `wii_mj2d.bmg` mount guarded by
  `0x80359CB0`, and the self-contained fixed-width decimal formatter.
- **Character model pair** (`0x800CA060`/`0x800CA120`). The `0x874`-byte
  record constructor with its `0x802DC9A0` entry array, the two pivot tables
  and the `Mario` name row at `+0x778`, plus the matching destructor.
- **GX state blocks** (`0x800CB8A0..0x800CC390`). Five literal call sequences:
  the three mask-scene variants (which differ only in
  `0x801C8DC0`/`0x801C8E70`/`0x801C86C0`/`0x801C82F0` selectors and one call
  order), the no-argument background variant and the cave-overlay variant
  whose alpha comes from `+0x14`.
- **Mask helpers** (`0x800CAD20` GX point size,
  `0x800CB470`, `0x800CB630`, `0x800CC650`, `0x800CCE00`). The tiny helper
  constructor, the cave-texture handle store, the guarded free and the
  overlay builder that opens the `0x800`-byte mask stream on the
  `r13-0x4870` heap.
- **Mask manager and message tail** (`0x800CDED0`,
  `0x800CE700..0x800CE7D0`, `0x800CFBF0..0x800CFCA0`). The `0x980`-byte mask
  manager initialiser (nine cells, four `0x802F1150` defaults, ten rows of
  four pairs), the small message object and its two `+4`/`+6` readers, and the
  single message record with its locked allocation and first-call arming.

The public entry points are [`fn_800c10f0`] (lift init), [`fn_800cb8a0`]
(first mask-scene GX block), [`fn_800cdb30`] (message mount) and
[`fn_800cded0`] (mask manager).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800C10F0` | `fn_800c10f0` | 280 | medium |
| `0x800C9E80` | `fn_800c9e80` | 132 | high |
| `0x800CA060` | `fn_800ca060` | 192 | high |
| `0x800CA120` | `fn_800ca120` | 128 | high |
| `0x800CA1A0` | `fn_800ca1a0` | 56 | high |
| `0x800CAD20` | `fn_800cad20` | 332 | high |
| `0x800CB430` | `fn_800cb430` | 60 | medium |
| `0x800CB470` | `fn_800cb470` | 28 | high |
| `0x800CB630` | `fn_800cb630` | 56 | medium |
| `0x800CB8A0` | `fn_800cb8a0` | 576 | high |
| `0x800CBAE0` | `fn_800cbae0` | 576 | high |
| `0x800CBD20` | `fn_800cbd20` | 576 | high |
| `0x800CC090` | `fn_800cc090` | 440 | high |
| `0x800CC390` | `fn_800cc390` | 468 | high |
| `0x800CC650` | `fn_800cc650` | 64 | medium |
| `0x800CCE00` | `fn_800cce00` | 172 | high |
| `0x800CDB30` | `fn_800cdb30` | 180 | high |
| `0x800CDC70` | `fn_800cdc70` | 308 | medium |
| `0x800CDED0` | `fn_800cded0` | 324 | high |
| `0x800CE700` | `fn_800ce700` | 60 | high |
| `0x800CE7A0` | `fn_800ce7a0` | 36 | high |
| `0x800CE7D0` | `fn_800ce7d0` | 36 | high |
| `0x800CFBF0` | `fn_800cfbf0` | 32 | high |
| `0x800CFC10` | `fn_800cfc10` | 72 | medium |
| `0x800CFCA0` | `fn_800cfca0` | 88 | medium |

Address range: `0x800C10F0..0x800CFCA0` (eleven runs, 25 bodies, 5272 bytes).

## Deliberately skipped

The bucket still holds 56 untranslated functions after this pass. Skipped on
purpose:

- **Paired-single (GQR) math chain** (`0x800C0570`, `0x800C07B0`,
  `0x800C18B0`, `0x800C1A90`, `0x800C1C80`, `0x800C1E90`, `0x800C1F20`, the
  fifteen `0x800C2030..0x800C2690` handlers, `0x800C3A10`, `0x800C4F50`,
  `0x800C51A0`, `0x800C6E70`, `0x800C9680`, `0x800C9B50` and
  `0x800CAB30`). Every one builds its floats through
  `unaff_GQR0`/`unaff_GQR3` blocks with `ldexpf` rounding, and several call
  helpers whose argument lists Ghidra could not recover (the `0x800C2030`
  wrappers print `FUN_800c1e90(double)` while `0x800C1E90` itself declares
  five parameters). The same limit is what wave 91 documented; no honest call
  binding exists today.
- **Unrecovered call signatures** (`0x800CB490`, `0x800CB670`,
  `0x800CB740`, `0x800CB760`, `0x800CB780`, `0x800CB5B0`, `0x800CB5D0`,
  `0x800CDDB0`). `0x800CB490` calls its three `0x800CB6xx` children with four
  visible arguments while the children declare five or six; the virtual
  forwarders `0x800CB5B0`/`0x800CB5D0` lose the receiver argument entirely, and
  `0x800CDDB0` forwards its own five parameters into a call the export prints
  as argument-less. Reconstructing them would invent an argument list.
- **Stack-argument varargs** (`0x800C9A60`, `0x800C9DF0`). Both copy eight
  `u64` register arguments into a stack block and tail-call the real worker;
  the exports show `stack0x00000008` and `in_cr1`, not source arguments.
- **Large switch body** (`0x800C75B0`, 4772 bytes). A single state-machine
  switch that needs its own review pass.
- **Tiny accessors and stubs** (`0x800C3BA0`, `0x800C8A90` 4-byte thunks;
  `0x800CCA70`, `0x800CE100`, `0x800CE110`, `0x800CE550`, `0x800CE560`,
  `0x800CE570`, `0x800CE5F0` 4-8 byte accessors; `0x800CD220`, `0x800CE240`,
  `0x800CE250`, `0x800CE530`, `0x800CE540`, `0x800CFBB0` 12-byte type-info
  stubs). The module-header policy keeps these as address calls.

None of these is counted as translated in the CSV.

## Verification

- Every address and byte size matches `functions.tsv` (25 rows, 5272 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`; the 25
  addresses were chosen from the 81 open entries of the `0x800C0000` bucket.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs` (the remaining diagnostics are in other lanes' files).
- `node tools/check-lane-csv.mjs wave93` prints `OK` and the wave-93 stage
  rows are clean.
