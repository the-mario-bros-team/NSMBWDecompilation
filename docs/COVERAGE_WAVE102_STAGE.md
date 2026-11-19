# Coverage - Wave 102 (`game::stage`, the `0x800C0000` line-geometry and GX tail)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The `evidence`
column of `ledger/agents/wave102-stage.csv` is the Ghidra C export each body was
written from. Field roles the export only hints at stay marked `observed`,
`inferred` or `unknown` in the body.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 15 | 7488 | 1 | 14 |

No body in this pass is `low` confidence and none is `partial`; every row is
`translated`. The pass covers `0x800C1C80..0x800CDDB0`.

## What this pass covers

- **The line segment/intercept test** (`0x800C1C80`, `0x800C1F20`). `0x800C1C80`
  re-bases both endpoints on a reference pair, rejects vertical and same-side
  segments, computes the slope/intercept through the translated `0x800C19E0`,
  and reports a hit when the **intercept** (the second output, not the slope)
  lies in the `-0x6814..(-0x6810 + limit)` window and `0x800C1A40` confirms the y
  span. `0x800C1F20` builds the `+0x6818`/`-0x6818` half-extent query pair and
  writes `+0x40`/`+0x44`/`+0x50`/`+0x54`, then flips the line through
  `0x800C1780` when `+0x3C`/`+0x60` straddle zero.
- **The line-state ladder** (`0x800C20F0`, `0x800C21C0`, `0x800C2280`,
  `0x800C2360`, `0x800C23F0`, `0x800C2460`, `0x800C2580`, `0x800C2690`). Each is
  the two-step shape wave 98 established for this cluster: run the shared line
  test, then transition the `+0x6C` interface to one `0x80358Exx`/`0x80359xxx`
  state record. Six handlers materialise their two distance limits from the `r2`
  block (`-0x6808`, `-0x67FC`, `-0x6804`, `-0x67F8`, `-0x67F4` against
  `-0x6818`/`-0x6800`); the two `0x800C1F20` callers forward the float register
  untouched. The second float is kept by calling `0x800C1E90` through the
  original address, because the merged `fn_800c1e90` only declares one float.
- **The `dLineMng_c` state-table registrar** (`0x800C75B0`). The 4772-byte
  initialiser builds all twenty-five state records (`Idle` through
  `Circle4x4Rig`/`Circle4x4Lef`) from the shipped `0x80316D28..0x803170A8` rows,
  installs the common vtable `0x80317404`, and chain-registers each record with
  `0x802DC780` using the shared destroy helper `0x800C8860` and the descriptor at
  `record - 0x10`, all under the `0x802DCD74`/`0x802DCDC0` lock pair. It reuses
  the helper shape already proven by `w45_state_record`.
- **The viewport projection** (`0x800C9680`). Two span ratios derived from the
  source box (`+0x64..+0x78`), clamped by `-0x6778`, rescaled by `-0x6774` when
  below `-0x6788`, divided into the live screen `u16` size read through
  `r13-0x5200`, combined with the frame rectangle `+0x4C..+0xB0`, and written as
  five floats after `0x801C9CD0` supplies the four-word view matrix. The four
  `GQR0`/`ldexpf` prologue blocks are dead locals.
- **The GX ribbon strip** (`0x800CAB30`). Opens the FIFO, selects attribute
  `0xA0` with `count + 1`, writes the state colour word per segment, and (when
  `flag` is set) a second float `base + amplitude * sin(angle)` whose angle
  starts at `r2-0x6738` and advances by `r2-0x6734 / count`.
- **The tail helpers** (`0x800CB5B0`, `0x800CDDB0`). `0x800CB5B0` is a
  tail-dispatch through `this+0x04` → `+0x0C` → slot `+0x08` (the export could
  not classify the indirect jump). `0x800CDDB0` formats a number with the
  translated `0x800CDC70` and then rewrites the same buffer through the
  translated `0x800CDDF0`.

Address range: `0x800C1C80..0x800CDDB0` (15 bodies, 7488 bytes).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800C1C80` | `fn_800c1c80` | 400 | medium |
| `0x800C1F20` | `fn_800c1f20` | 260 | medium |
| `0x800C20F0` | `fn_800c20f0` | 108 | medium |
| `0x800C21C0` | `fn_800c21c0` | 96 | medium |
| `0x800C2280` | `fn_800c2280` | 96 | medium |
| `0x800C2360` | `fn_800c2360` | 132 | medium |
| `0x800C23F0` | `fn_800c23f0` | 108 | medium |
| `0x800C2460` | `fn_800c2460` | 132 | medium |
| `0x800C2580` | `fn_800c2580` | 132 | medium |
| `0x800C2690` | `fn_800c2690` | 108 | medium |
| `0x800C75B0` | `fn_800c75b0` | 4772 | high |
| `0x800C9680` | `fn_800c9680` | 688 | medium |
| `0x800CAB30` | `fn_800cab30` | 380 | medium |
| `0x800CB5B0` | `fn_800cb5b0` | 24 | medium |
| `0x800CDDB0` | `fn_800cddb0` | 52 | medium |

## Deliberately skipped

The bucket opened with 32 untranslated entries; this pass lands 15 and leaves
17. Skipped on purpose:

- **`GQR3` fixed-point angle bodies** (`0x800C0570`, `0x800C3A10`, `0x800C4F50`,
  `0x800C51A0`). Each converts the `s16` angle at `+100` through the
  `CONCAT22`/`CONCAT31` forms of `psq_l` with a live `GQR3` quantisation mode.
  The decompiler prints stack garbage as the low half, so the angle scale is not
  recoverable from the export; wave 85 and wave 71 already deferred the same
  shape.
- **`0x800C18B0`.** It loops over `FUN_800c17a0()` with no printed argument; the
  loop is eight `0x38`-byte entries but the query's two float registers are not
  recoverable, as wave 85 noted.
- **`0x800C1A90`.** Its two `FUN_800c1a40` calls differ in arity: one prints
  three arguments and the other only two, so the value in `f3` that the second
  call consumes is undefined in the export.
- **`0x800C07B0`.** It calls the skipped `0x800C0570` three times; the first call
  prints only three of the four arguments, so the angle word passed to the
  shadow pass cannot be grounded.
- **`0x800C9A60`, `0x800C9DF0`, `0x800C9B50`.** These are the GX display-packet
  builders: `in_cr1` gates the optional float pair, sixteen parameters stand for
  the register save area, and `0x800C9B50` additionally reads `in_f30`/`in_f31`
  through `GQR0`. The packet layout is not present in the export.
- **`0x800CB490`, `0x800CB670`, `0x800CB740`, `0x800CB760`, `0x800CB780`.**
  Each depends on the `0x802DCD74` return pair: `extraout_f1` is the float half
  of that helper's result, not an argument, so the store at `+0x10` cannot be
  grounded.
- **`0x800C3BA0` and `0x800C8A90`.** Two 4-byte branch thunks: `0x800C3BA0`
  jumps to the already translated `fn_800c3bac`, and `0x800C8A90` jumps to the
  already translated `crate::render::geometry::fn_80007320`. Both are skipped
  under the wave-84/98/99/100/101 precedent: the targets already have ledger
  rows, so counting the branch thunks would double-count translated bodies.

The one large body in the bucket, `0x800C75B0` (4772 bytes), was attempted and
landed; no other large entry was left for a later pass.

None of the 17 skipped entries is counted as translated in the CSV
(15 landed + 17 skipped = the 32 open entries).

## Verification

- Every address and byte size matches `functions.tsv` (15 rows, 7488 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`; the 15 addresses
  were chosen from the 32 open entries of the `0x800C0000` bucket.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs` (the errors it reports are in other lanes' files:
  `game::worldmap` and `resource::archive`).
- `node tools/check-lane-csv.mjs wave102` prints `OK`; the wave-102 stage rows
  are clean.
