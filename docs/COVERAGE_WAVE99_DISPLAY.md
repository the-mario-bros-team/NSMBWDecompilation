# Coverage - Wave 99 (engine::display)

## How to read this file

`src/engine/display.rs` is the semantic home of the engine-level 2-D
presentation layer. This pass adds 25 bodies from the `0x800D0000` engine band,
the same band that waves 16, 20, 63, 76, 89, 90 and 93 mined. "Bodies" counts
`pub unsafe fn` functions; "bytes" is the sum of the Ghidra sizes recorded in
`work/ghidra/main-gb/export/functions.tsv`.

"high" means every store, branch and call is visible in the export. "medium"
means the export drops a receiver or an operand: the missing receiver is marked
`// inferred:`, a paired-single `ldexpf` stub is documented as a no-op, or a
raw 64-bit `undefined8` operand keeps its observed bit pattern instead of being
guessed to be a float. Every field whose role is not proven stays `// unknown:`
or `// inferred:` in the Rust comment.

Only `src/engine/display.rs` was edited. `ledger/functions.csv` is read-only and
the coordinator merges this lane's rows.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/display.rs` | `engine::display` | 25 | 6820 | 12 | 13 |

## What this pass covers

The bucket entered this pass with 43 untranslated of its 251 functions and 18
remain. This pass closes five slices, all in the `0x800D16B0..0x800DFCE0`
range.

- **The actor teardown and value helpers** (`0x800D16B0..0x800D5170`,
  4 bodies). [`fn_800d16b0`] is the empty-body tail call the wave-20 cursor
  documents as `FN_800D16B0`; [`fn_800d38a0`] releases the four embedded
  interfaces and the two animation records, then runs the `+0x170/+0x174`
  frame timer; [`fn_800d4cd0`] refreshes the interpolated `+0x1E2..+0x1E4`
  triple; [`fn_800d5170`] spawns the `Wm_2d_mrstarkira` / `Wm_mr_startail`
  effects through the three `+0x180` modes.
- **The `dStateMng` selector family** (`0x800D57F0..0x800D69B0`, 5 bodies).
  [`fn_800d57f0`] rebuilds the marker's orientation and position matrices from
  the state-dependent height and the two input angles; [`fn_800d5dd0`],
  [`fn_800d65e0`] and [`fn_800d69b0`] walk the six 0x24-byte records at
  `0x802F1350` and dispatch slots `+0x60` through `+0x100`; [`fn_800d6460`]
  picks one of the two `0x802F2C38` effect rows.
- **The command packet and the ground-collision filters**
  (`0x800D9760..0x800DAC40`, 3 bodies). [`fn_800d9760`] builds the 0x80-byte
  packet from the four `0x80377C88` views and submits it to `0x801B8620`;
  [`fn_800dab40`] and [`fn_800dac40`] are the rectangular and circular marker
  band tests, both returning a hit flag and writing an angle plus a depth.
- **The `dRemoconMng_c::dConnect_c` cluster**
  (`0x800DD1C0..0x800DE850`, 11 bodies). Six `0x802DCBFC(second, second, first)`
  thunks at `0x800DD1C0..0x800DD280`, `0x800DE4B0` and `0x800DE4E0`;
  [`fn_800dd2b0`] registers the six `dConnect_c` descriptors from the
  `0x80319048..0x803190FC` source records; [`fn_800dd7d0`], [`fn_800ddb50`] and
  [`fn_800de850`] are the release shims.
- **The interface helpers** (`0x800DFBE0..0x800DFCE0`, 2 bodies).
  [`fn_800dfbe0`] type-tests the `+0x04` object against `0x802F86C8` before
  dispatching slot `+0x34`; [`fn_800dfce0`] seeds and submits one five-word
  record.

Two facts carry across the slice. First, the selector records are one table:
`0x802F1350 + mode * 0x24` has its `Rwait` name at `+0x04`, the id byte at
`+0x10`, the tail word at `+0x1C`, the timer at `+0x20` and the fallback name
at `+0x00`, so the addresses `0x802F1354/1360/136C/1370` used by the export are
just fields of the same record. Second, the six `dConnect_c` descriptors are a
permuted copy of six 0x24-byte source records: the destination `+0x0C..+0x2C`
takes the source words in the order `+0x18, +0x1C, +0x20, +0x0C, +0x10, +0x14,
+0x00, +0x04, +0x08`, which [`w99_descriptor_fields`] documents once instead of
six times.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800D16B0` | `fn_800d16b0` | 56 | medium |
| `0x800D38A0` | `fn_800d38a0` | 388 | medium |
| `0x800D4CD0` | `fn_800d4cd0` | 172 | medium |
| `0x800D5170` | `fn_800d5170` | 444 | medium |
| `0x800D57F0` | `fn_800d57f0` | 908 | medium |
| `0x800D5DD0` | `fn_800d5dd0` | 1040 | medium |
| `0x800D6460` | `fn_800d6460` | 236 | medium |
| `0x800D65E0` | `fn_800d65e0` | 580 | medium |
| `0x800D69B0` | `fn_800d69b0` | 116 | medium |
| `0x800D9760` | `fn_800d9760` | 248 | medium |
| `0x800DAB40` | `fn_800dab40` | 248 | high |
| `0x800DAC40` | `fn_800dac40` | 356 | medium |
| `0x800DD1C0` | `fn_800dd1c0` | 48 | high |
| `0x800DD1F0` | `fn_800dd1f0` | 48 | high |
| `0x800DD220` | `fn_800dd220` | 48 | high |
| `0x800DD250` | `fn_800dd250` | 48 | high |
| `0x800DD280` | `fn_800dd280` | 48 | high |
| `0x800DD2B0` | `fn_800dd2b0` | 1212 | high |
| `0x800DD7D0` | `fn_800dd7d0` | 88 | high |
| `0x800DDB50` | `fn_800ddb50` | 100 | high |
| `0x800DE4B0` | `fn_800de4b0` | 48 | high |
| `0x800DE4E0` | `fn_800de4e0` | 48 | high |
| `0x800DE850` | `fn_800de850` | 88 | high |
| `0x800DFBE0` | `fn_800dfbe0` | 144 | medium |
| `0x800DFCE0` | `fn_800dfce0` | 60 | medium |

Byte total: `1060` (actor/value helpers) + `2880` (selector family) +
`852` (packet and collision) + `1824` (`dConnect_c` cluster) + `204`
(interface helpers) = `6820`.

## Deliberate skips

18 addresses stay untranslated. Every skip has a concrete export defect, not a
budget cut.

**Paired-single / GQR quantisation (6 addresses).** `0x800D0610`,
`0x800D19E0`, `0x800D2460`, `0x800D2690`, `0x800D27B0` and `0x800D43E0` are
printed with `unaff_GQR0`/`unaff_GQR3`, `ldexpf` and `CONCAT13/CONCAT22`
quantisation stubs, and `0x800D0610` additionally carries five
`WARNING: Removing unreachable block` lines. The scalar remainder of
`0x800D43E0` also loses two operands to `unaff_r2`, so no honest body can be
written without inventing the paired-single arithmetic.

**Unrecovered extra-out registers (4 addresses).** `0x800D0470` reads its
inserted record from `extraout_r4`, `0x800D2CF0` its first float from
`extraout_f1`, `0x800D9D70` its first float from `extraout_f1` on top of two
GQR stubs, and `0x800DAA10` its result halfword from `unaff_r29`. The export
never defines any of these registers.

**Debug-context splits (5 addresses).** `0x800D3380`, `0x800D8B00`,
`0x800D8C60`, `0x800DFA40` and `0x800DFF30` take their receiver from the
`0x802DCD6C/0x802DCD70/0x802DCD74` pseudo-return, and `0x800D8B00` adds a lost
`extraout_r4`. The real `r3/r4` split is not visible, so a signature would be
invented.

**Unrecoverable jumptable dispatch (1 address).** `0x800D6550` is printed as a
`countLeadingZeros(state - 0x97)` index with
`WARNING: Could not recover jumptable`; the flattened switch is not a real
`0x98/0x99` index, so transcribing the formula would document the decompiler's
failure.

**Four-byte thunk body (1 address).** `0x800D8670` is a four-byte jump to
`0x800D8680`, but the export contains the target's loop body. Transcribing it
at `0x800D8670` would document the wrong function.

**Dropped walk result (1 address).** `0x800DFDD0` calls `0x800DFD20` and then
walks the `+0x64` chain `param_2` times, but the export declares the function
`void` and never uses the walked pointer; only the first call is observable, so
writing a body would either invent a return or ship a no-op.

## Verification

- Every address and byte size matches `functions.tsv`, re-read for this pass;
  the 25 `# Original` blocks total 6,820 bytes.
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/engine/display.rs`.
- No address was already in `ledger/functions.csv`; the bucket had 43
  untranslated functions before this pass and 18 after it.
- `node tools/check-lane-csv.mjs wave99` reports `OK`; this lane's CSV has 25
  unique rows and no duplicate address.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/engine/display.rs`.
