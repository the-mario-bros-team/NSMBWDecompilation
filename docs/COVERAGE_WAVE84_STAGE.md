# Coverage - Wave 84 (`game::stage`, the `0x8009xxxx` actor-subclass gaps)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The evidence
column of `ledger/agents/wave84-stage.csv` points at the Ghidra C export each
body was written from. Roles that the export only hints at are marked
`inferred` or `unknown` in the body itself, never here.

Object layouts used by this pass, all *observed* from the stores:

* **`Wm` sub-actor** (as established by wave 69): vtable at `+0x60`,
  interface sub-object at `+0x394`, position floats at `+0xAC`/`+0xB0`,
  probe flag byte at `+0x38F`, cached coordinates at `+0xDC..+0xE4` mirrored
  to `+0x370..+0x378`.
* **Course-actor subclass** (`0x5FC` bytes): `+0x524` list header, `+0x540`
  countdown, `+0x544` motion sub-object (two vtables installed by the
  constructor), `+0x598`/`+0x59C` counts, `+0x5A8`/`+0x5B0`/`+0x5B4` vector,
  `+0x5B8` callback pair, `+0x5F0` halfword, `+0x5F4` latch, `+0x5F8` state
  source, and the variant record `+0x414..+0x432`.
* **Motion/effect actor** (`0x8009Bxxx`): `+0x544` motion object, `+0x5F8`
  state source, `+0x604` transform block, `+0x6B0`/`+0x6B4` channel present
  words with their objects at `+0x700`/`+0x72C`, `+0x794` state argument,
  `+0x7A4` player index, `+0x7B6` flag byte.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 29 | 2976 | 26 | 3 |

No body in this pass is `low` confidence and none is `partial`; every row is
`translated`.

## What this pass covers

Three untranslated runs of the `0x8009xxxx` band plus the duplicated
`0x802DCBFC` forwarding pair:

- **`Wm` sub-actor helpers** (`0x80097700..0x80097F70`). The two-float ground
  query with its `1..=2` window and `+0x21C` callback, the four-player owner
  scan through `0x8005F900`/`0x80162D00` on `+0x2A78`, the cached-position
  refresh with the `+0x98`/`+0xD4` virtual pair, the `0x269` effect submit
  through `0x80198C30` and the variant-record publisher that copies the six
  `.rodata` words at `0x802F04A4..0x802F04B8` into `+0x418..+0x42C`.
- **Course-actor subclass** (`0x80098350..0x80098D90`). Seven 12-byte
  `getType()`-style stubs that all return the shared type-info record
  `0x80377A04`, the constructor that installs three vtables and the
  `0x524`/`0x544`/`0x5A8` members, the two destructors (one releases `+0x74`,
  one releases `+0x5B8`/`+0x544`/`+0x524` before the `Wm` teardown
  `0x80095130`), the `0x8016A960` refresh pass and the frame driver with its
  `+0x540` countdown and `+0x2DC`/`+0x2E0` dispatch.
- **Motion/effect helpers** (`0x8009B980..0x8009BFE0`). The `+0x408` step
  publish on `+0x102`, the six halfword clears at `+0x758`, the `0x34`-byte
  matrix copy from `+0x604`, the `+0x7A4` player x test, the `0x4EA`/`0x4EB`
  motion-id select with the `Wm_jr_damage` submit, the `0x80168230` state
  clear, the two-index channel reset loop and the `+0x7B6` byte store.
- **Forwarding pair** (`0x8009AC40`/`0x8009AC70`): both 48-byte wrappers send
  `(second, second, first)` to `0x802DCBFC`, matching the three copies the
  module already carries at `0x800B0750`/`0x800B0780`/`0x800C7550`.

The public entry points of this pass are [`fn_80097700`] (ground test),
[`fn_800983c0`] (subclass constructor), [`fn_80098d90`] (subclass frame driver)
and [`fn_8009bd80`] (motion-id select plus damage effect).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80097700` | `fn_80097700` | 104 | high |
| `0x800979B0` | `fn_800979b0` | 108 | high |
| `0x80097C40` | `fn_80097c40` | 108 | high |
| `0x80097EA0` | `fn_80097ea0` | 116 | high |
| `0x80097F70` | `fn_80097f70` | 320 | medium |
| `0x80098350` | `fn_80098350` | 12 | high |
| `0x80098360` | `fn_80098360` | 12 | high |
| `0x80098370` | `fn_80098370` | 12 | high |
| `0x80098380` | `fn_80098380` | 12 | high |
| `0x80098390` | `fn_80098390` | 12 | high |
| `0x800983A0` | `fn_800983a0` | 12 | high |
| `0x800983B0` | `fn_800983b0` | 12 | high |
| `0x800983C0` | `fn_800983c0` | 240 | medium |
| `0x800984B0` | `fn_800984b0` | 120 | high |
| `0x800985B0` | `fn_800985b0` | 164 | high |
| `0x80098700` | `fn_80098700` | 120 | high |
| `0x80098830` | `fn_80098830` | 168 | high |
| `0x80098AD0` | `fn_80098ad0` | 132 | high |
| `0x80098D90` | `fn_80098d90` | 476 | medium |
| `0x8009B980` | `fn_8009b980` | 88 | high |
| `0x8009BA50` | `fn_8009ba50` | 32 | high |
| `0x8009BC90` | `fn_8009bc90` | 68 | high |
| `0x8009BD10` | `fn_8009bd10` | 112 | high |
| `0x8009BD80` | `fn_8009bd80` | 184 | high |
| `0x8009BEB0` | `fn_8009beb0` | 16 | high |
| `0x8009BF60` | `fn_8009bf60` | 112 | high |
| `0x8009BFE0` | `fn_8009bfe0` | 8 | high |
| `0x8009AC40` | `fn_8009ac40` | 48 | high |
| `0x8009AC70` | `fn_8009ac70` | 48 | high |

Address range: `0x80097700..0x8009BFE0` (two contiguous runs plus the
`0x8009AC40`/`0x8009AC70` pair).

## Deliberately skipped

Skipped because the body could not be reconstructed honestly:

- `0x80098900` (464 bytes) - the subclass effect pass. It returns a register
  pair from `0x802DCD74` (`(int)((ulonglong)uVar8 >> 0x20)`), then forwards the
  mangled `extraout_f1` between branches. Neither the pair split nor the float
  that reaches `0x80275780` can be stated from the C text.
- `0x80097250` (404) and `0x800978c0` (236) - both wrap their float work in
  `unaff_GQR0`/`ldexpf` guards, and `0x80097250` also prints its receiver as
  an `undefined8`. The quantisation order is not visible.
- `0x80097a20` (264) - the four-call matrix compose
  (`0x8012DE80`/`0x801C0BD0`/`0x801C0540`/`0x8016F130`) needs the `double`
  plumbing of two 48-byte matrix blocks; the C shows `(double)` casts on values
  that are never declared as floats, so the argument order would be invented.
- `0x800976b0` (76) - calls `0x80097700()` with both arguments dropped and
  seeds `+0xF8`/`+0x118` from an `r2-0x6EAC` word whose value is not in the
  export; the receiver and the point cannot be recovered.
- `0x80097170` (116) - calls `0x80097250()` with the receiver dropped; since
  that callee itself is skipped, the call cannot be grounded.
- `0x8009ACA0..0x8009AD20` (nine 12-byte stubs) - the same
  `return &DAT_80377a04;` shape as the seven translated ones. They are left for
  a later pass on purpose: they add no new evidence and the seven translated
  copies already document the family. Their two neighbours
  (`0x8009AC40`/`0x8009AC70`) are translated.
- The larger `0x8009Cxxx`/`0x8009Dxxx`/`0x8009Exxx` and the
  `0x80093530`/`0x80093A50`/`0x80093F10`/`0x8009A320`/`0x8009B070`/
  `0x8009DD60` entries were not attempted in this pass; they are outside the
  three chosen runs and each needs its own layout study.

## Verification

- Every address and byte size matches `functions.tsv` (29 rows, 2976 bytes);
  `tools/check-lane-csv.mjs` cross-checks each `# Original` size.
- Every evidence file exists at the cited path under `work/ghidra/main-gb/`.
- Every row has exactly one body in `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs`.
- `node tools/check-lane-csv.mjs wave84` prints `OK`; the wave-84 stage rows
  are clean.
