# Coverage - Wave 106 (`runtime::input`, KPAD self-test, ring flush and USB trace forwarders)

## How to read this file

1. **Summary** gives the per-file totals and the inventory of the
   `[0x801E0000, 0x801F0000)` bucket, which is now closed.
2. **What this pass covers** describes the three clusters and how they connect
   to the wave 97/103 bodies around them.
3. The **function table** lists all six bodies in address order with the byte
   count re-read from `work/ghidra/main-gb/export/functions.tsv` and the
   confidence carried by each doc block.
4. **Corrections and caveats** records every inference the export forced, and
   **Deliberate skips** lists the four bucket entries this pass did not take.

Status vocabulary (`docs/READING_GUIDE.md`): `translated` means the whole
control flow and every call in the export are accounted for. A `partial` row
has every branch and call accounted for except one construct the platform
helpers cannot express; the doc block names it. Four rows are `translated`,
two are `partial`. Names stay `fn_<addr>` because the image has no symbol table
for this layer; the doc comments carry the roles and repeat the
`// observed:` / `// inferred:` / `// unknown:` markers at the code.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/input.rs` | `runtime::input` | 6 | 5,748 | 1 | 5 |

Bucket inventory (`[0x801E0000, 0x801F0000)`):

| Item | Count |
|---|---:|
| Functions in the bucket | 103 |
| Untranslated before this pass | 10 |
| Selected and translated by this pass | **6** |
| Deliberately skipped | 4 |
| Untranslated in the bucket after this pass | 4 |

## What this pass covers

The ten entries wave 103 left were the hardest tail of the band. Six of them
fall into three shapes:

- **KPAD random self-test builders (`0x801E9730`, `0x801E9E20`, 3,560 B).**
  Two near-identical twins, one per sampling mode: the extension calibration
  step `0x801E65F0` starts `0x801E9730` for the four extension classes
  `1..4`/`0x11`/`0x12` and `0x801E9E20` for every other class. Each reads
  the tick counter three times, seeds a byte generator
  (`state = (c + state * b) % 255`), draws ten bytes from a 0x900-byte table and
  picks the six-byte answer row `state % 7`. The ten random bytes, six mixed key
  bytes and two eight-byte XOR frames are written into the channel block at
  `+0x914`..`+0x933`. The twins differ only in the table base
  (`0x80344430`/`0x80344D60`) and in the key mixing being additions rather than
  subtractions; the frame synthesis is identical and is factored into
  `kpad_self_test_frames`.
- **KPAD ring flush (`0x801ED3C0` plus the `0x801ED3A0` forwarder, 1,848 B).**
  This is the read path behind the console loop at `0x802D71F0`: it validates
  the layer byte `r13-0x4B84`, the link state (`0x801F3530` must be `3`) and the
  per-channel busy byte `+0x4FF`, marks the newest ring run with the `0xF9`,
  `0xFE` and `0xFC` status markers, copies up to `length` `0x38`-byte slots into
  the caller's `0xB0`-byte records and runs the three wave-97/103 filter steps
  (`0x801EAD30`, `0x801ECAD0`, `0x801EB160`, `0x801EC2F0`) over them. The result
  code `0`/`-1`/`-2`/`-3`/`-4`/`-5` comes back through `0x802DCDA4`.
- **IOS USB error forwarders (`0x801EE9A0`, `0x801EEA50`, 340 B).** The two
  sixteen-argument `va_list` trampolines that print the `"USB_ERR_"` banner and
  then the caller's formatted line. The gate bytes (`r13-0x4B58`,
  `r13-0x5DDC`), the banner pointers and both call targets are reconstructed;
  the `va_list` frame itself is not expressible, so these two rows are
  `partial` and say so.

## Functions

| Address | Function | Role | Bytes | Status | Confidence | Evidence |
|---|---|---|---|---|---|---|
| `0x801E9730` | `fn_801e9730` | KPAD self-test generator (mode A) | 1,776 | translated | medium | `work/ghidra/main-gb/export/801e9730_FUN_801e9730.c` |
| `0x801E9E20` | `fn_801e9e20` | KPAD self-test generator (mode B) | 1,784 | translated | medium | `work/ghidra/main-gb/export/801e9e20_FUN_801e9e20.c` |
| `0x801ED3A0` | `fn_801ed3a0` | tail call into the ring flush | 12 | translated | high | `work/ghidra/main-gb/export/801ed3a0_FUN_801ed3a0.c` |
| `0x801ED3C0` | `fn_801ed3c0` | KPAD ring flush / read worker | 1,836 | translated | medium | `work/ghidra/main-gb/export/801ed3c0_FUN_801ed3c0.c` |
| `0x801EE9A0` | `fn_801ee9a0` | USB error trace forwarder (gate `r13-0x4B58`) | 168 | partial | medium | `work/ghidra/main-gb/export/801ee9a0_FUN_801ee9a0.c` |
| `0x801EEA50` | `fn_801eea50` | USB error trace forwarder (gate `r13-0x5DDC`) | 172 | partial | medium | `work/ghidra/main-gb/export/801eea50_FUN_801eea50.c` |

Totals: **4 translated, 2 partial**, 0 stubs; 1 high / 5 medium / 0 low;
5,748 bytes. Address range `0x801E9730`-`0x801EEA50`.

## Corrections and caveats

1. **The three seed reads.** Both self-test builders open with three calls to
   `0x801B5FA0`; Ghidra declares that function as returning nothing, so the
   first two results appear as `extraout_var_00` and `extraout_var` and the
   third is used directly (`uVar5 = FUN_801b5fa0()`). The repo consistently
   treats `0x801B5FA0` as the 32-bit tick read (`src/runtime/archive.rs`,
   `worldmap.rs`, `dvd.rs`), so the Rust reads the low byte of each call and
   masks the second with `0x3F`. This is the only uncertainty in the two bodies;
   the generator, the tables, the answer rows, the key mixing and every store
   are literal.
2. **Two spellings of one rotation.** `fn_801e9730` prints the byte rotate as
   `x >> n | x << (8 - n)` and `fn_801e9e20` as `x << n | x >> (8 - n)`. The
   twins carry the same instruction, so the Rust keeps both literal spellings
   (`rot_right8` / `rot_left8`) rather than silently unifying them.
3. **Shared frame synthesis.** Both exports compute the same sixteen `ft`/`sb`
   XOR pairs from `keys` and `rnd` over the `(selector + 1) * 0x100` and
   `(selector + 2) * 0x100` pages. `kpad_self_test_frames` is that shared body;
   the two 0x900-byte table bases are its only parameter.
4. **`fn_801ed3c0` argument recovery.** The function reads its first two words
   through the `0x802DCD58` split (high half `r3` = channel, low half `r4` =
   record buffer), so the Rust signature is
   `(channel, buffer, length, status_out, keep)`. The sibling
   [`fn_801ed3b0`](src/runtime/input.rs) already uses the same five-word
   forwarding, which is what the `0x801ED3A0` caller at `0x802D71F0` relies on.
5. **The `0xB0` record layout is inferred.** The entry copy and the closing
   copy are both exactly 44 words to and from a stack image, and every processed
   record receives a fresh copy of the block's leading `0xB0` bytes. The
   selected `length` ring slots are written at `buffer + (n - 1) * 0xB0`, so the
   caller's area holds `length` records of `0xB0` bytes; the export never states
   the layout, so the field roles inside a record stay `// inferred:`.
6. **Undefined sentinel words.** In the first ring pass the two axis words are
   only assigned for report kinds `0`, `-2` and `-7`; for any other kind the
   export reads them uninitialised. The Rust uses `0xFFFF`, which is the value
   that makes the three block-default substitutions (`*block & 0x9F1F`,
   `*block`, `block + 0x60`) fire; the export's own `0xFFFF` sentinel tests
   confirm that reading.
7. **Inner interrupt-lock pair.** `fn_801ed3c0` re-locks around the ring walk
   with `FUN_801b1140()`/`FUN_801b1180()` but discards both the token and the
   restore value, so those two calls are emitted by address (`call0`) instead of
   through `disable_interrupts`/`restore_interrupts`.
8. **`0x801EE230` is the sampling callback, not the connect callback.** Wave 103
   described it as the connect callback; `fn_801edaf0` installs it through
   `kpad_set_sampling_callback_801e0fd0` at `+0x54C`, while the connect
   callback at `+0x550` is `0x801EDF40`. The correction is recorded here because
   the body itself is still skipped.
9. **Trace payloads.** The two `partial` rows keep the gate test and the banner
   call. The Metrowerks frame
   (`local_18 = 0x01000000`, `local_14 = &stack0x00000008`,
   `local_10 = &local_78`) cannot be built with `call0`-`call7`, so the
   `0x802E1560` formatted line is dropped, exactly as wave 102 dropped the
   `0x801F9C50` payload behind its seven guards.

## Deliberate skips

- **`0x801E1E20` (5,628 B).** The WPAD command-record writer. Its body is
  thirteen copies of a `0x30`-byte stack record; twenty `CONCAT22`/`CONCAT13`
  expressions write halves of locals (`local_70`, `local_100`, `local_190`,
  `local_1f0`, ...) that the export never initialises, so the record contents
  cannot be recovered. Its callers keep calling the address.
- **`0x801E39C0` (4,224 B).** The second WPAD command writer; same undefined
  stack-union defect (`local_70`, `local_d0`, `local_130`, `local_a4`, ...,
  thirteen `CONCAT` sites).
- **`0x801E55C0` (2,792 B).** The paired-single calibration/finaliser routine.
  The export opens with six `unaff_GQR0` mode tests and `in_f28`..`in_ps31_1`
  entry registers and is laced with "removing unreachable block" warnings, so
  the GQR state at each `ps_*` site cannot be stated honestly.
- **`0x801EE230` (1,896 B).** The KPAD sampling callback. It opens with the same
  GQR/paired-single prologue (`in_f30`/`in_f31`, `in_ps30_1`/`in_ps31_1`) and
  builds two dead stack slots from entry registers before the load-cell code, so
  the entry state is not recoverable. Its two skip twins at `0x801EE9A0` /
  `0x801EEA50` were taken as `partial` because their gate logic is independent
  of the dropped frame.

## Verification

- Every address and byte size in the table was re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass (total 5,748 bytes).
- Every evidence file exists at the cited `work/ghidra/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/input.rs`.
- No address was already in `ledger/functions.csv`; the ten-entry untranslated
  list for the bucket was derived from it first.
- `node tools/check-lane-csv.mjs wave106` prints `OK` and reports
  `wave106-input.csv: 6 rows`.
- `cargo check --message-format=short` reports no error and no warning in
  `src/runtime/input.rs` (the four remaining errors in the run are in
  `src/game/stage.rs` and `src/engine/main_loop.rs`, which belong to other
  lanes).

## Ledger

Machine-readable rows: `ledger/agents/wave106-input.csv` (6 rows, module
`runtime::input`, address order as the table above, header
`module,address,name,status,confidence,evidence,rust_file`).
