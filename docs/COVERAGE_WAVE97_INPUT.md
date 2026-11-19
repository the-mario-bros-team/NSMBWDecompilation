# Coverage - Wave 97 (`runtime::input`, KPAD channel blocks, pointer aim and IOS/GKI shims)

## How to read this file

1. **Summary** gives the per-file totals and the band inventory for
   `[0x801E0000, 0x801F0000)`.
2. **What this pass covers** describes the three clusters that were added and
   how they relate to the wave 4/5 bodies around them.
3. The **function table** lists all 25 bodies in address order with the byte
   count re-read from `work/ghidra/main-gb/export/functions.tsv` and the
   confidence carried by each doc block.
4. **Corrections and caveats** records every inference the export forced, and
   **Deliberate skips** lists the 24 band entries this pass did not take.

Status vocabulary (`docs/READING_GUIDE.md`): `translated` means the whole
control flow and every call in the export are accounted for. All 25 rows are
`translated`. Names stay `fn_<addr>` because the image has no symbol table for
this layer; the doc comments carry the roles and repeat the
`// observed:` / `// inferred:` / `// unknown:` markers at the code.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/input.rs` | `runtime::input` | 25 | 6,812 | 11 | 14 |

Band inventory (`[0x801E0000, 0x801F0000)`):

| Item | Count |
|---|---:|
| Functions in the band | 103 |
| Untranslated before this pass | 49 |
| Selected and translated by this pass | **25** |
| Untranslated in the band after this pass | 24 |

## What this pass covers

The band is the middleground between the WPAD/KPAD report layer that waves 4
and 5 translated (`0x801DE9D0`-`0x801E8DD0`) and the Bluetooth host stack at
`0x801F...`. Three shapes repeat:

- **KPAD channel-block layer (`0x801E5450`, `0x801EA9C0`-`0x801EE190`,
  10 bodies).** The four `0x578`-byte records start at `0x803A1FD4`; each one
  owns three raw axes, six `0x0C`-byte status records, two 16-bit press timers,
  a `+0x44` word exchanged under the interrupt lock and a small byte block at
  `+0x4A`..`+0x4F`. `fn_801ea9c0` re-seeds the whole record (pointer bounds,
  two scale/ratio words, magnitude limit, gain), `fn_801eab90` advances the two
  press timers with their 40000/20000 wraps, `fn_801ead30` derives the
  pressed/released edges before calling it, `fn_801edf40` is the asynchronous
  connect/reset step fed by the `0x802DCD74` split return, and
  `fn_801ee0b0`/`fn_801ee150`/`fn_801ee170`/`fn_801ee190` are the locked word
  exchange and the extension detach helpers. `fn_801e5450` is the single
  small-data init word they bracket.
- **Pointer/aim filters (`0x801EADD0`-`0x801EC8A0`, 8 bodies).**
  `fn_801eadd0` is the shared axis smoother (fourth-power gain inside the
  `+0xC8` radius, clamp mode when `+0x56C` is set); `fn_801eae90` and
  `fn_801eb030` normalise the two- and three-axis acceleration vectors;
  `fn_801eb160` is the extension-report decoder that feeds them and the
  control stick; `fn_801ec8a0` is the classic-controller dead-zone mapper.
  Above them the three aim matchers work on the same record: `fn_801eb5c0` and
  `fn_801eb7b0` search the four sensor-bar points for the best-scoring pair,
  `fn_801eb9e0` tests each point against the screen bounds, and
  `fn_801ebba0` pairs each aim point with its nearest sensor point.
- **IOS/GKI shims (`0x801EF6F0`-`0x801EFFA0`, 7 bodies).** These are the
  application-memory and control-message shims the Bluetooth stack calls by
  address: `fn_801eff00`/`fn_801eff50` wrap the `0x803A3814`/`0x803A3818`
  callbacks (already named by [`APP_MEM_ALLOC`] and the wave-96 pool release),
  `fn_801effa0` finishes a WUD sync, `fn_801efef0` reads the WBC personality
  word, and `fn_801ef6f0`/`fn_801ef770`/`fn_801ef810` push IOS bulk messages
  through `0x801EF390`/`0x801EF030`.

## Functions

| Address | Function | Role | Bytes | Status | Confidence | Evidence |
|---|---|---|---|---:|---|---|---|
| `0x801E5450` | `fn_801e5450` | KPAD init-word store at `r13-0x4BD8` | 8 | translated | high | `work/ghidra/main-gb/export/801e5450_FUN_801e5450.c` |
| `0x801EA9C0` | `fn_801ea9c0` | channel block reset (bounds, gain, status bytes) | 464 | translated | high | `work/ghidra/main-gb/export/801ea9c0_FUN_801ea9c0.c` |
| `0x801EAB90` | `fn_801eab90` | two 16-bit press timers with 40000/20000 wrap | 416 | translated | high | `work/ghidra/main-gb/export/801eab90_FUN_801eab90.c` |
| `0x801EAD30` | `fn_801ead30` | button word, press/release edges, timer step | 160 | translated | medium | `work/ghidra/main-gb/export/801ead30_FUN_801ead30.c` |
| `0x801EADD0` | `fn_801eadd0` | axis smoothing toward a new sample | 180 | translated | high | `work/ghidra/main-gb/export/801eadd0_FUN_801eadd0.c` |
| `0x801EAE90` | `fn_801eae90` | acceleration direction blend + hold counter | 404 | translated | medium | `work/ghidra/main-gb/export/801eae90_FUN_801eae90.c` |
| `0x801EB030` | `fn_801eb030` | three-axis normalise into `+0x54`/`+0x58` | 292 | translated | medium | `work/ghidra/main-gb/export/801eb030_FUN_801eb030.c` |
| `0x801EB160` | `fn_801eb160` | extension report decode and control stick | 1,120 | translated | medium | `work/ghidra/main-gb/export/801eb160_FUN_801eb160.c` |
| `0x801EB5C0` | `fn_801eb5c0` | sensor-bar pair search by dot product | 488 | translated | medium | `work/ghidra/main-gb/export/801eb5c0_FUN_801eb5c0.c` |
| `0x801EB7B0` | `fn_801eb7b0` | weighted pair search with cosine gate | 552 | translated | medium | `work/ghidra/main-gb/export/801eb7b0_FUN_801eb7b0.c` |
| `0x801EB9E0` | `fn_801eb9e0` | per-point border extrapolation test | 436 | translated | medium | `work/ghidra/main-gb/export/801eb9e0_FUN_801eb9e0.c` |
| `0x801EBBA0` | `fn_801ebba0` | nearest sensor-bar point per aim point | 372 | translated | medium | `work/ghidra/main-gb/export/801ebba0_FUN_801ebba0.c` |
| `0x801EC8A0` | `fn_801ec8a0` | control-stick dead zone and radial clamp | 548 | translated | medium | `work/ghidra/main-gb/export/801ec8a0_FUN_801ec8a0.c` |
| `0x801EDF40` | `fn_801edf40` | asynchronous channel reset (`0x802DCD74`) | 356 | translated | medium | `work/ghidra/main-gb/export/801edf40_FUN_801edf40.c` |
| `0x801EE0B0` | `fn_801ee0b0` | locked word exchange at `block+0x44` | 88 | translated | high | `work/ghidra/main-gb/export/801ee0b0_FUN_801ee0b0.c` |
| `0x801EE150` | `fn_801ee150` | clears `block+0x4A` | 28 | translated | high | `work/ghidra/main-gb/export/801ee150_FUN_801ee150.c` |
| `0x801EE170` | `fn_801ee170` | sets `block+0x4A` | 28 | translated | high | `work/ghidra/main-gb/export/801ee170_FUN_801ee170.c` |
| `0x801EE190` | `fn_801ee190` | extension detach notification and latches | 148 | translated | medium | `work/ghidra/main-gb/export/801ee190_FUN_801ee190.c` |
| `0x801EF6F0` | `fn_801ef6f0` | IOS bulk message tag 2 | 116 | translated | medium | `work/ghidra/main-gb/export/801ef6f0_FUN_801ef6f0.c` |
| `0x801EF770` | `fn_801ef770` | IOS bulk message tag 1, `0x10000` threshold | 160 | translated | medium | `work/ghidra/main-gb/export/801ef770_FUN_801ef770.c` |
| `0x801EF810` | `fn_801ef810` | same as `0x801EF770` after a cache flush | 160 | translated | medium | `work/ghidra/main-gb/export/801ef810_FUN_801ef810.c` |
| `0x801EFEF0` | `fn_801efef0` | reads `r13-0x4B38` (WBC personality) | 8 | translated | high | `work/ghidra/main-gb/export/801efef0_FUN_801efef0.c` |
| `0x801EFF00` | `fn_801eff00` | `App_MEMalloc` wrapper | 80 | translated | high | `work/ghidra/main-gb/export/801eff00_FUN_801eff00.c` |
| `0x801EFF50` | `fn_801eff50` | `App_MEMfree` wrapper | 80 | translated | high | `work/ghidra/main-gb/export/801eff50_FUN_801eff50.c` |
| `0x801EFFA0` | `fn_801effa0` | `__wudSyncFlushCallback` pending mode | 120 | translated | high | `work/ghidra/main-gb/export/801effa0_FUN_801effa0.c` |

Totals: **25 translated**, 0 partial, 0 stubs; 11 high / 14 medium / 0 low;
6,812 bytes. Address range `0x801E5450`-`0x801EFFA0`.

## Corrections and caveats

1. **`fn_801eab90` argument recovery.** The export calls it with an empty
   argument list from `FUN_801ead30` (`FUN_801eab90();`). Its own parameters
   are `(block, mode, delta)` and the mode gate it applies
   (`mode == 2 || mode-0x10 <= 1`) is identical to the gate `0x801EAD30`
   applies before the call, so the body forwards `(block, mode, delta)` and is
   marked `medium`.
2. **`fn_801ec8a0` square root.** The export calls
   `thunk_FUN_802e8620()` with no visible argument; the only pending value is
   the `x*x + y*y` sum the guard just computed, which is what the body passes.
   The two axis blocks are inlined twice in the original, so the Rust uses one
   private helper (`kpad_axis_deadzone`) rather than duplicating them.
3. **Pointer pairs in `fn_801eb5c0`/`fn_801eb7b0`.** Both functions carry the
   winning pair in `r25`-`r27`; the export writes `r27` to `+0x120` and `r26`
   to `+0x12C`, and swaps the two when the dot product is negative. The body
   names them `primary` (`+0x120`) and `secondary` (`+0x12C`).
4. **`fn_801eb160` axis signs.** The two horizontal axes are negated before
   scaling (`-(int)*(short *)`), the third is not. The `0x43300000` folds in
   the export are the signed-int-to-double bias, so the body uses plain
   `as f32`/`as f64` casts; the bias word at `r2-0x4D18` is
   `0x4330000080000000`, which is why the fold subtracts `2^52 + 2^31`.
5. **`fn_801edf40` sampling callback.** It registers `0x801EE230`, which is
   still untranslated, so the address is passed literally to
   [`kpad_set_sampling_callback_801e0fd0`]. The `0x802DCD74` halves are read
   once through `platform::call_u64`.
6. **`fn_801ee0b0` lock pair.** The export shows `FUN_801b1140()` /
   `FUN_801b1180()` with no arguments; the body uses the file's established
   `disable_interrupts`/`restore_interrupts(token)` wrappers, which are the
   same two routines.
7. **IOS senders and `call8`.** `0x801EF390` takes eight integer arguments and
   [`crate::platform`] stops at `call7`, so this file gains a local `call8`
   helper that mirrors `platform::call7`. `0x801EF030` needs seven and uses
   `platform::call7`. `0x801EF030`/`0x801EF390`/`0x801EF8B0` themselves stay
   untranslated because all three read `extraout_r4`.
8. **`fn_801effa0` reads the pending byte twice.** The export reads
   `0x803A312C` for the trace argument and again for the guard; the body
   reproduces the second read.
9. **Constant naming.** The `r2` floats are addressed as
   `SDA2_F32_4D38` (`0.0f`), `SDA2_F32_4D2C` (`1.0f`) and
   `SDA2_F32_4D20` (the magnitude limit) because those roles are forced by the
   comparisons; the remaining `r2` words keep numeric names. The `r13` scale
   words keep numeric names as well because their units are not proven.
10. **Module header.** The fourteenth-pass bullet and the promotion-note
    sentence were appended; no earlier count was stale (the previous text
    named no total).

## Deliberate skips

- **`0x801E1E20` (5,628 B) and `0x801E39C0` (4,224 B).** The two largest
  bodies in the band: the report state machine and the per-channel state
  machine wave 4 stopped in front of. They need a dedicated pass to keep the
  evidence tight.
- **`0x801E55C0` (2,792 B).** The extension calibration finaliser already
  named in the wave-4 docs; not touched here.
- **`0x801E9730` (1,776 B) and `0x801E9E20` (1,784 B).** These read the WPAD
  control table and the `0x801B5FA0` source, but the export drops their
  results into `extraout_var`/`extraout_var_00`, so the sample bytes are not
  recoverable without guessing.
- **`0x801EAE90`-adjacent `0x801EE230` (1,896 B).** The per-channel sampling
  callback that `0x801EDF40` installs. Its paired-single arithmetic appears as
  `unaff_GQR0` and `in_f30`/`in_ps30_1`/`in_ps31_1` frame artifacts; the
  reconstruction would be a guess at the quantised operands.
- **`0x801EDAF0` (1,092 B) and `0x801ED3C0` (1,836 B).** The KPAD init and the
  big per-frame calculate step. Both are honest candidates for the next pass;
  `0x801EDAF0` shares every field offset with the blocks added here and is the
  natural entry point.
- **`0x801ED3A0` (12 B) and `0x801ED3B0` (8 B).** Pure forwarders into
  `0x801ED3C0` that pass their arguments in registers the export does not
  show; identical to the 4-byte thunks earlier passes skipped.
- **`0x801EBD20` (1,484 B), `0x801EC2F0` (1,148 B), `0x801ECAD0` (2,256 B).**
  The remaining report decoders in the pointer group (classic-controller and
  DPD report walks). `0x801EC2F0` needs the `0x803A1C04` status-record ring the
  new constants name.
- **The IOS USB layer** `0x801EE9A0` (168 B), `0x801EEA50` (172 B),
  `0x801EEB00` (248 B), `0x801EEC00` (8 B), `0x801EEC10` (412 B),
  `0x801EEDB0` (324 B), `0x801EEF00` (292 B): `0x801EE9A0`/`0x801EEA50` are
  the 16-parameter variadic loggers the rest of that layer formats through, so
  the whole group needs one pass that models the variadic frame.
- **`0x801EFE60` (140 B).** A thin wrapper over `FUN_801EF8B0`, but it forwards
  ten arguments; `platform::call7` and the local `call8` cannot reach that
  arity, so the body would have to drop two arguments.
- **`0x801EF030` (852 B), `0x801EF390` (856 B), `0x801EF8B0` (1,444 B).** The
  IOS control-message senders behind the three wrappers added here. All three
  read `extraout_r4` (the lost low byte of the argument vector), so they stay
  with the USB layer for a dedicated pass. The wrappers call
  `0x801EF390`/`0x801EF030` by address in the meantime.

## Verification

- Every address and byte size in the table was re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass (total 6,812 bytes).
- Every evidence file exists at the cited `work/ghidra/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/input.rs` (checked by name for all 25 addresses).
- No address was already in `ledger/functions.csv`; the 49-entry untranslated
  list for the band was derived from it first.
- `node tools/check-lane-csv.mjs wave97` prints `OK` and reports
  `wave97-input.csv: 25 rows`.
- `cargo check --message-format=short` finishes with no errors; the only
  `src/runtime/input.rs` output is the two pre-existing "direct cast of
  function item into an integer" warnings from earlier passes (they were
  present before this pass and merely moved by the header edit). While another
  Wave 97 lane's `src/runtime/bt.rs` still failed to resolve, the file was also
  type-checked in a scratch copy of the crate with only those two foreign
  errors patched out.

## Ledger

Machine-readable rows: `ledger/agents/wave97-input.csv` (25 rows, module
`runtime::input`, address order as the table above, header
`module,address,name,status,confidence,evidence,rust_file`).
