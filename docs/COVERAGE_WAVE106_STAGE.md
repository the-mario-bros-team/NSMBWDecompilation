# Coverage - Wave 106 (`game::stage`, the `0x800C0000` remainder)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The `evidence`
column of `ledger/agents/wave106-stage.csv` is the Ghidra C export each body was
written from. Field roles the export only hints at stay marked `observed`,
`inferred` or `unknown` in the body.

Three conventions carry over from waves 41/102/105 and matter for this bucket:

* **The `r2` float bank.** Every constant named `W106_*` is a `SDA2_BASE`
  relative float (`0x80433080 - offset`). The three that drive this batch are
  `-0x67E0` (radians per angle unit), `-0x687C` (distance per `+0x128` unit) and
  `-0x6880` (the zero used for the matrix translation rows).
* **The paired-single prologues are dead.** The `GQR0`/`GQR3` blocks in
  `0x800C0570`, `0x800C3A10` and `0x800C4F50` only rescale an `s16` into a
  register the body then reads as a single; the
  `CONCAT44(0x43300000, x) - 2^52` chains are the unsigned-integer-to-double
  idiom. They are reproduced as `i16 as f32` / `u16 as f64` casts, exactly as
  `fn_800b3000` and `fn_80090590` already do.
* **The `__savegprlr` pack still applies.** `0x802DCD74` returns the entry
  `r3:r4` pair, so a body that opens with `uVar = FUN_802dcd74()` is reading
  its own first two arguments out of the high/low halves.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 11 | 3044 | 1 | 7 |

Three rows are `low` confidence (the three `0x800CB74x/76x/78x` alpha
flavours); none is `partial`. The pass covers `0x800C0570..0x800CB780`.

## What this pass covers

- **The line-object heading push** (`0x800C0570`). One distance
  (`r2-0x687C * [+0x128]`) with a `+0x12C`-selected base and a `r2-0x686C`
  scaled heading; `flag == 0` subtracts the sine/cosine pair from the point and
  every other value adds it.
- **The three-model rebuild** (`0x800C07B0`). Three positions built from the
  `+0xE0..+0xE8`/`+0xEC..+0xF4` and `+0xF8..+0x100` sums, the `+0x11C..+0x120`
  angle triple, the shared matrix pipeline of `fn_80090440`
  (`0x80064AF0`/`0x801C0BD0`/`0x8016EDD0`/`0x8016ECB0`/`0x8016EEF0` plus the
  three `0x8016A1xx` binds) and the per-model `+0x14` virtual. The second model
  is gated on `+0x124` and translates by it; the third drops by `r2-0x6878`
  when `+0x12C == 1`.
- **The line wall query** (`0x800C18B0`). This is the callee every wave-38/41
  handler reaches through `W41_LINE_CONTACT`/`W98_LINE_CONTACT`: seven eight-byte
  nodes handed to `0x800C17A0` as two doubles; a kind of `0x22` flips the line
  with `0x800C1780` when `+0x69` is clear and raises `+0x6A`.
- **The anchored intercept test** (`0x800C1A90`). Rebase both endpoints on the
  anchor, ask `0x800C19E0` for slope/intercept, handle the vertical pair and the
  `limit - slope` division, then accept the crossing inside the
  `-0x6814 .. -0x6810 + span` window plus the `0x800C1A40` coordinate span test.
- **The place-and-step pair** (`0x800C3A10`, `0x800C4F50`) and the retry state
  (`0x800C51A0`). The first copies `+0x50/+0x54 + offset` into
  `+0x58/+0x5C/+0x40/+0x44` and steps both coordinates; the second turns
  `+100` by `+0x60 * rate` first, rebuilds `+0x38`/`+0x3C` as `-speed*sin` and
  `speed*cos`, then reloads and steps. The retry state saves the three words,
  asks `0x800C18B0` for a contact and either rolls back and negates the two
  velocities or falls into the `0x800C4B50` / `0x800C4C50` neighbour machines
  with the `0x4000` / `0x8000` heading split.
- **The render-object initialiser and its three alpha flavours**
  (`0x800CB670`, `0x800CB740`, `0x800CB760`, `0x800CB780`). The initialiser
  allocates through `0x80164FD0`, writes id/texture/alpha/`+0x14`, maps negative
  width/height to the `6`/`7` fallback flags and applies the size with
  `0x8016A2A0`; the three flavours are the compiler-generated pass-throughs
  that only replace the `f1` alpha with `r2-0x6738` or `r2-0x670C`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800C0570` | `fn_800c0570` | 328 | medium |
| `0x800C07B0` | `fn_800c07b0` | 812 | medium |
| `0x800C18B0` | `fn_800c18b0` | 292 | medium |
| `0x800C1A90` | `fn_800c1a90` | 484 | medium |
| `0x800C3A10` | `fn_800c3a10` | 188 | medium |
| `0x800C4F50` | `fn_800c4f50` | 300 | high |
| `0x800C51A0` | `fn_800c51a0` | 316 | medium |
| `0x800CB670` | `fn_800cb670` | 196 | medium |
| `0x800CB740` | `fn_800cb740` | 20 | low |
| `0x800CB760` | `fn_800cb760` | 20 | low |
| `0x800CB780` | `fn_800cb780` | 88 | low |

Address range: `0x800C0570..0x800CB780` (11 bodies, 3044 bytes).

## Deliberately skipped

The bucket held 17 open entries. Eleven are landed above; the other six are
skipped on purpose.

- **Two 4-byte branch thunks into already-translated bodies** (`0x800C3BA0`,
  `0x800C8A90`). `0x800C3BA0` is `thunk_FUN_800c3bac`, and `0x800C3BAC` already
  has a ledger row in `game::stage`. `0x800C8A90` is `thunk_FUN_80007320`, and
  `0x80007320` is already translated in `render::geometry`. A row for either
  thunk would double-count the target body; same precedent as waves 84 and
  98-105.
- **Two variadic format wrappers whose call arguments are not printed**
  (`0x800C9A60`, `0x800C9DF0`). Both build an eight-word block plus a format tag
  (`0x5000000` and `0x3000000`) and call the next wrapper with **no** argument
  list at all: the only register expressions the export keeps are the dead
  `va_list` pointers (`local_14 = &stack0x00000008`, `local_10 = &local_78`),
  which are never passed to the callee. The mapping from the sixteen incoming
  parameters to the callee's registers therefore cannot be stated without
  inventing it.
- **The 660-byte body behind those wrappers** (`0x800C9B50`). Its frame
  declares `local_c8`/`local_d0`/`local_d8`/`local_e0` *inside* the 0x68-byte
  matrix buffer (`auStack_c0`), so the two screen-rectangle reads that drive the
  `+0xE8` store cannot be located, and its first two arguments arrive only
  through the `0x802DCD68` pack. The rest of the body is legible, but the two
  float slots that decide the final branch are not present in the export.
- **The render-object allocator of the `0x800CB6xx` family** (`0x800CB490`).
  Its three wrapper calls print four register expressions while the callee reads
  `r5..r8`, and the fourth printed expression is the leftover `r3` of the
  preceding `0x800CAB10` call, so the wrapper argument list is not recoverable.
  The callee itself (`0x800CB670`) is landed above because its own body names
  every field it writes.

None of the six is counted as translated; 11 landed + 6 skipped = the 17 open
entries of the `0x800C0000` bucket.

## Verification

- Every address and byte size matches `functions.tsv` (11 rows, 3044 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`; the eleven were
  chosen from the 17 open entries of the `0x800C0000` bucket.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs` (the remaining error belongs to another lane file).
- `node tools/check-lane-csv.mjs wave106` prints `OK`; the wave-106 stage rows
  are clean.
- The module header of `src/game/stage.rs` gained a wave-106 paragraph naming
  all eleven bodies and the six skips.
