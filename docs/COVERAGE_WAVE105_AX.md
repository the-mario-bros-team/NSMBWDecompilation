# Coverage - Wave 105 (`audio::ax`)

## How to read this file

This is the status page for the wave-105 pass over the `audio::ax` lane of the
`main.dol` reconstruction. The `0x801A0000` bucket holds 271 inventory entries;
14 were still unclaimed after wave 104. This pass reconstructs seven of them,
leaves six to the `runtime::os` range row that already owns their bytes, and
records the last one here with a reason.

The sections answer, in order:

1. **What did this pass translate?** - *Functions* lists every body with its
   address, Rust name, byte size and confidence.
2. **What does the band contain?** - *What this pass covers* groups the bodies
   into the FPU-context trio, the two `OSExec` stages, the font blitter and the
   framebuffer/banner bring-up.
3. **What was left out, and why?** - *Deliberate skips* records the seven
   addresses that get no body.

**Virtual addresses.** All addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`), not file offsets.

**Evidence.** The Ghidra C is under `work/ghidra/main-gb/export/`; the function
index is `work/ghidra/main-gb/export/functions.tsv`. Inline Rust markers follow
`docs/RUST_STYLE.md`: `// observed:`, `// inferred:` and `// unknown:`.

**Bucket boundary.** This is the last pass over the bucket. Of its 271 entries,
263 already had `audio::ax` bodies, six are the `runtime::os` range row
`801a9bc0-801a9c10`, and the seven below were the remainder.

**Ledger.** The machine-readable rows are `ledger/agents/wave105-ax.csv`
(module `audio::ax`, 7 rows, all `translated`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/audio/ax.rs` | `audio::ax` | 7 | 6,752 | 0 | 7 |

Status split: 7 `translated`, 0 `partial`, 0 `stub`. Address range:
`0x801ACAD0..0x801AF7C0`.

## What this pass covers

- **FPU context trio, `0x801ACAD0..0x801AD3B0`.** These are the paired-single
  aware halves of the `OSContext` family that wave 103 deferred. `0x801ACAD0`
  tests the `+0x1A2` state bit, restores FPSCR with `lfd`/`mtfsf`, then loads
  the 32 `psq` slots at `+0x1C8` and the 32 doubles at `+0x90`. `0x801ACC00`
  is the save half: it sets the state bit, stores the 32 doubles, captures
  FPSCR with `mffs`, reloads `f0` (the `mffs` target) and stores the 32 paired
  singles. `0x801AD3B0` is the same save preceded by
  `mfmsr`/`ori 0x2000`/`mtmsr`/`isync`, i.e. the exception-path variant that
  forces MSR[FP] on. All three keep the raw `psq_l`/`psq_st` form, which is the
  exact quantisation the Ghidra export expands into its `ldexpf` chain.
- **First-stage `OSExec`, `0x801AE0E0`.** Allocates the four boot blocks
  (`0x20`, `0x800`, `0x4A00`, `0xE0` bytes), probes the IOS DVD device, reads
  the header and the `0x800`-byte directory, scans the two directory halves
  (8-byte stride, second half at `+0x20`) for the word at `r13 - 0x4FE0`,
  publishes the chosen id to `0x80003194`/`0x80003198`, then reads the `0xD8`
  records into the `0x4A00` block and runs the launch call. Every wait polls
  the shared `r13 - 0x4FD8` callback word and reports the source line
  (`0x398`..`0x465`) on a non-1 status.
- **Second-stage `OSExec`, `0x801AE840`.** Builds the 0x1C-byte exec record,
  formats argv through [`fn_801adba0`]/[`fn_801ade80`], initialises the DVD
  path, runs [`fn_801ae0e0`], then reads the image header in `0x910`-word
  chunks (or the drive-reported size). A ten-byte `"2004_02_01"` tag selects
  the compressed path: the ELF entry point at `header+0x10` is called to hand
  back start/read/stop function pointers, the second segment is streamed to
  `0x81330000`, and `0x801AE050` jumps into the loaded stage. The untagged path
  copies the segment straight to `0x81330000`.
- **Font blitter, `0x801AF180`.** The `0x802DCD40` save thunk supplies
  `(frame, pitch)`; the body clears a 24x24 four-bit glyph plane, decodes one
  character with `0x801B0860`, then walks the 24 rows and columns. Each set
  nibble writes a luma byte `(colour * nib * 0xEF) / 0xEF1 + 0x10` and two
  chroma bytes whose order depends on the pixel parity. A newline or the
  `pitch - 0x30` cursor bound ends the line.
- **Framebuffer and banner, `0x801AF7C0`.** Copies the saved launch string,
  allocates the `0xA1004`-byte font block and the `0x96000`-byte 16-bit
  framebuffer, converts the two saved colour words through the thirteen
  `sdata2` coefficients, fills all `0x1E0` rows, programs the VI through
  [`fn_801af4b0`], draws one text line with [`fn_801af180`], flushes the
  buffer, prints the banner and calls `0x801A9BA0` (`halt`).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801ACAD0` | `fn_801acad0` | 292 | medium |
| `0x801ACC00` | `fn_801acc00` | 296 | medium |
| `0x801AD3B0` | `fn_801ad3b0` | 300 | medium |
| `0x801AE0E0` | `fn_801ae0e0` | 1,668 | medium |
| `0x801AE840` | `fn_801ae840` | 1,876 | medium |
| `0x801AF180` | `fn_801af180` | 808 | medium |
| `0x801AF7C0` | `fn_801af7c0` | 1,512 | medium |

Totals: **7 bodies**; **0 high, 7 medium**; **6,752 bytes**.

`src/audio/ax.rs` gained three host-side helpers that carry no ledger row:
`exec_wait` (poll one callback word and report a non-1 status),
`dvd_transfer` (queue and poll one DVD descriptor) and `exec_chunk_size` (the
cached `0x910`-word chunk probe), plus `boot_yuv`/`clamp_band` for the colour
conversion. It also gained the `OS_EXEC_*`/`BOOT_*` address constants and the
`FN_*` callee constants used by the two loaders.

## Deliberate skips

Reason code **D** means the bytes are already claimed by another module's row,
so a second body would translate the same bytes twice; **S** means the only
non-branch word writes CPU state with no observable memory effect, so there is
no honest body to write.

| Address | Reason | Note |
|---|---|---|
| `0x801A9BC0..0x801A9C10` | D | Six 8-byte `mtspr` leaves already owned by the `runtime::os` range row `801a9bc0-801a9c10` (`clear_performance_monitor`, SPRs 952/956/953/954/957/958). Individual rows would duplicate the same six words. |
| `0x801A9CE0` | S | Eight bytes whose one non-branch word is `0xFFA0004C`, followed by `blr`. Wave 98/101/103 could not confirm the mnemonic; the only effect would be a non-volatile FPR write with no observable result, so no body is claimed. |

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (292 / 296 / 300 / 1,668 / 1,876 / 808 / 1,512; total 6,752).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every CSV row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/audio/ax.rs`, and its doc block carries the same confidence.
- No address in this CSV is present in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning in
  `src/audio/ax.rs` (the remaining errors name other lanes' files).
- `node tools/check-lane-csv.mjs wave105` reports `wave105-ax.csv: 7 rows` and
  `OK`.
