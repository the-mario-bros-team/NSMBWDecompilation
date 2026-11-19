# Coverage - Wave 113 (engine::main_loop)

## How to read this file

`src/engine/main_loop.rs` is the `0x802E0000` bucket: the statically linked C
runtime at the low end, MetroTRK in the middle and the interrupt / exception
knobs at the top. Before this pass the bucket had 201 of its 203 addresses
translated (202 `pub unsafe fn` bodies: `fn_802e890c` has a PowerPC and a host
twin). The two gaps were the exception prologue `0x802EC9CC` and the exception
epilogue `0x802ECCB4`; Waves 105, 106 and 111 left both, because their Ghidra
exports print the `stmw` / `lmw` register runs as independent `in_rNN` field
stores and every return as a `returnFromInterrupt(msr, srr0)` pseudo-call.

Wave 113 decodes the instruction words in `work/dol/main.elf` for both, lands
the ordinary C part of each as a `partial` body, and closes the two call sites
that named them by address. With this pass the bucket has 203 of 203 addresses
represented by 204 bodies in the file.

Each row below cites the Ghidra name, the `functions.tsv` byte size and the
exact export path. "Confidence" is copied from the body's `# Confidence` line
and from the CSV row. The body is `pub unsafe fn fn_<address>` in
`src/engine/main_loop.rs`; the address is lowercase hex in the file name and in
the ledger row.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/engine/main_loop.rs` | `engine::main_loop` | 2 | 1024 | 0 | 0 | 2 |

`high + medium + low = 0 + 0 + 2 = 2`. Before this pass the file held 202
bodies over 201 addresses; it now holds 204 bodies over all 203 addresses.

## What this pass covers

- **The exception prologue** (`0x802EC9CC`, 744 bytes). The body performs the
  `MSR` mask that the first real instruction block applies: it installs the
  image at `0x80427224` with `ori 0x8002; xori 0x8002`, i.e. `MSR.EE` and
  `MSR.RI` cleared. It stores the halfword exception number at `0x8042B058`,
  runs the `0x500` external-interrupt branch (the empty `0x802E981C` call, the
  first byte of the handler named by the pointer at `0x80427238`, the
  in-exception byte at `0x8035162C`), and sets or clears the dispatch byte at
  `0x80427234`.
- **The nested-exception return** (`0x802ECB60`). This is a second entry point
  inside the prologue's extent. The body stores the exception at `0x80351628`
  and the current `SRR0` at `0x80351620`, then applies the decoded equality
  ladder: `0x200`, `0x300`, `0x400`, `0x600`, `0x700`, `0x800`, `0x1000`,
  `0x1100`, `0x1200` and `0x1300` advance `SRR0` by four (the return does not
  retry the faulting instruction); every other number retries it. It then sets
  `0x8035162D` and leaves through `rfi`.
- **The shared dispatch tail** (`0x802ECBFC`). This is the third entry point
  inside the prologue's extent, and the reason both Ghidra exports contain the
  same event block: the prologue branches to it at `0x802ECB5C` and the
  epilogue at `0x802ECDC8`. It is landed once as
  [`exception_dispatch_tail`]. If the dispatch byte is set it clears it and
  returns the context base `0x80427198`; otherwise it classifies the exception,
  reads the faulting instruction through [`fn_802ec374`] for `0x700` / `0xD00`,
  compares that word with `0x0FE00000` (the `twi 31,r0,0` encoding of
  [`fn_802e890c`]) and queues event `5`, `3` or `4` through [`fn_802e9bcc`] /
  [`fn_802e9b00`].
- **The exception epilogue** (`0x802ECCB4`, 280 bytes). The body clears
  `MSR.EE` and `MSR.RI` from the live `MSR` (`li r1,0x8002; nor; and; mtmsr`),
  tests the first byte of the handler named by `0x80427238`, and either sets
  the dispatch byte and returns through the shared tail, or clears
  `0x8035162C`, calls [`fn_802ea6bc`] and leaves through `rfi`.
- **Call sites closed.** [`fn_802e9624`] now calls [`fn_802ec9cc`] directly
  instead of going through `call8_u32` / `FN_802EC9CC`, and [`fn_802ea4c4`]
  calls [`fn_802eccb4`] directly instead of `platform::call0(FN_802ECCB4)`.
  The constant `FN_802EC9CC` is kept for reference; the eight-argument adapter
  is now unused but retained for its ABI documentation.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802EC9CC` | `fn_802ec9cc` | 744 | low |
| `0x802ECCB4` | `fn_802eccb4` | 280 | low |

Address range: `0x802EC9CC..0x802ECDCC`, which is exactly the prologue's and
epilogue's declared extents (`0x802EC9CC + 744 = 0x802ECCB4`,
`0x802ECCB4 + 280 = 0x802ECDCC`). The bucket now has no untranslated address.

## Deliberate limits (why both rows are `partial` / `low`)

Neither routine can be executed faithfully from Rust. Both replace `r1` and
save or restore every GPR, and both leave through `rfi`. The bodies therefore
carry the branch / store / call content that is ordinary C and list the
register-only steps at the address they occupy. The following is *not*
executed by the bodies:

| address range | content | why |
|---|---|---|
| `0x802EC9CC..0x802EC9E0` | `mtspr SRR0,r2`; `mtspr SRR1,r4`; `mfspr r4,SPRG3`; `mfcr r2`; `mtspr SPRG3,r2` | scratch stash of `r2` / `r4` / `SPRG3` / `CR` |
| `0x802ECABC..0x802ECAF4` | `stw r0/r1`; `mfspr SPRG1/SPRG2`; `stmw r4,0x10`; `mfspr SRR0/LR/SPRG3/CTR/XER`; `stmw r27,0x80` | writes the exception frame at `0x80427240` |
| `0x802ECB0C..0x802ECB5C` | `mtmsr`; `mtspr LR/CTR/XER/DSISR/DAR`; `lmw r3,0xC`; `lwz r0/r1/r2` | restores the previous context at `0x80427198` |
| `0x802ECA78..0x802ECA90` | `mtcrf` from `0x80427178`; `lwz r3,0x804270FC`; `lwz r2,0x804270F8`; `rfi` | `0x500` fast return |
| `0x802ECBE8..0x802ECBF8` | CR from `SPRG3`; `r2` from `SPRG1`; `r3` from `SPRG2`; `rfi` | nested return |
| `0x802ECCB4..0x802ECD00` | `stmw r0,0`; MSR/LR/CTR/XER/DSISR/DAR stores | writes the context block at `0x80427198` |
| `0x802ECD44..0x802ECD74` | `lmw r27,0x80`; `mtspr SRR0/LR/CTR/XER/DSISR/DAR`; `lmw r3,0xC`; `lwz r0/r1/r2`; `rfi` | epilogue return |
| `0x802ECD78..0x802ECDC8` | the same restore sequence as the epilogue's first half | handler path before the shared tail |

The decoded save map, so the next pass does not have to redo it:

| block / address | contents | evidence |
|---|---|---|
| `0x80427240` (`TRK_GPR_CONTEXT`) | `r0`; `r1` at +4; `SPRG1` at +8; `SPRG3` at +0xC; `r4..r31` at +0x10; `SRR0` (which the entry put `r2` in) at +0x80; `LR` at +0x84; `SPRG3` (the CR image) at +0x88; `CTR` at +0x8C; `XER` at +0x90 | `stw` / `stmw` runs at `0x802ECABC..0x802ECAF4` |
| `0x80427538` / `0x8042753A` | exception number, twice | `sth r3,0x2F8` / `sth r3,0x2FA` at `0x802ECACC` |
| `0x80427198` | `r0..r31` at +0; `LR` at +0x80; `CTR` at +0x84; `XER` at +0x88; `MSR` at +0x8C; `DAR` at +0x90; `DSISR` at +0x94 | `stmw r0,0` and the five `mfspr` stores at `0x802ECCB4` |
| `0x80427224` | MSR image the prologue masks | `lwz r2,0x8C(0x80427198)` at `0x802EC9E8` |
| `0x80427234` | dispatch / "request in flight" byte | `stb` at `0x802ECA70` / `0x802ECD28`, `lwz` at `0x802ECC10` |
| `0x80427238` | handler pointer whose first byte is tested | `lwz; lbz` at `0x802ECA40` / `0x802ECD0C` |
| `0x8042766C` | `LR` scratch of the `0x500` branch | `mflr` + `stw` at `0x802ECA1C` |
| `0x80351620` | saved PC | `mfspr SRR0` + `stw` at `0x802ECB6C` |
| `0x8035162C` | in-exception byte; byte +1 (`0x8035162D`) is the nested-return flag | `stb` at `0x802ECB08` / `0x802ECD3C` and `0x802ECBE4` |

No ledger row is a stub: each body really implements the stores, comparisons
and calls listed above. The status is `partial` because the register file and
the three `rfi` teardowns are documented instead of executed, and the
confidence is `low` for the same reason.

## Verification

- Every address and byte size matches
  `work/ghidra/main-gb/export/functions.tsv` (`802ec9cc` = 744 bytes,
  `802eccb4` = 280 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/engine/main_loop.rs`; the file now holds 204 bodies over 203 addresses
  and no duplicates.
- No address was already in `ledger/functions.csv` (it held 201 of the 203
  bucket entries before this pass; 201 + 2 = 203).
- `cargo check --message-format=short` reports no error and no warning for
  `src/engine/main_loop.rs`.
- `node tools/check-lane-csv.mjs wave113` reports `OK` and lists
  `wave113-main-loop.csv: 2 rows`.
