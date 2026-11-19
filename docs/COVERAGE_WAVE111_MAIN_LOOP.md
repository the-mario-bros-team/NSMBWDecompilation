# Coverage - Wave 111 (engine::main_loop)

## How to read this file

`src/engine/main_loop.rs` is the `0x802E0000` bucket: the statically linked C
runtime at the low end, MetroTRK in the middle and the interrupt / exception
knobs at the top. Before this pass the bucket had 200 of its 203 functions
translated (201 bodies: the two `#[cfg]` halves of `fn_802e890c` share one
address). Wave 111 adds one body and leaves two.

Each row below cites the Ghidra name, the `functions.tsv` byte size and the
exact export path. "Confidence" is copied from the body's `# Confidence` line
and from the CSV row. The body is `pub unsafe fn fn_<address>` in
`src/engine/main_loop.rs`; the address is lowercase hex in the file name and in
the ledger row.

Where Ghidra prints an empty body or drops a block, the body is recovered from
the instruction words in `work/dol/main.elf`. This pass did that for the
`init_jk` and `PIo2` tables and for the `.sdata2` scratch constants, and it did
it for the two skipped exception routines described under **Deliberate skips**.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/main_loop.rs` | `engine::main_loop` | 1 | 5816 | 0 | 1 |

`high + medium + low = 0 + 1 + 0 = 1`. Before this pass the file held 200
bodies; it now holds 201.

## What this pass covers

- **Payne-Hanek range reduction** (`0x802E624C`): the body the last three waves
  could only reach through `platform::call6`. `q[0..=jk]` holds the 24-bit
  chunks of `(2/pi)*x`, `iq[]` is the distilled integer image, and the fraction
  is rebuilt with `scalbn` and the eight shipped `pi/2` chunks. Every table is
  read from the shipped image instead of being embedded:
  - `init_jk` at `0x802FE1B8` is `{2, 3, 4, 6}` in this build, **not** the
    fdlibm `{3, 4, 4, 6}`, which is why the constant is read with
    `platform::read_u32(0x802F_E1B8 + prec*4)` rather than written out;
  - `PIo2` at `0x802FE1C8`, eight doubles from `1.570796251296997` down to
    `2.1674168387780482e-51`;
  - the ten `.sdata2` scratch constants at `0x8042FA78..0x8042FAB0` (`zero`,
    `twon24`, `two24`, `0.125`, `8.0`, `0.5`, `1.0`, `2^52 + 2^31`);
  - the `2/pi` chunk array that the caller passes, `0x802FE030`.
- **Call site closed**: [`fn_802e5db8`] (`__ieee754_rem_pio2`) previously built
  the three-word `tx` array and called `platform::call6(FN_802E624C, ...)`.
  It now calls `fn_802e624c` directly with the same six arguments, so the last
  call-by-address site in the MSL trig chain is gone.
- **Integer conversions**: the export spells every `(double)(int32_t)w` as
  `CONCAT44(0x43300000, w ^ 0x80000000) - *(r2 - 0x35D0)`. The subtrahend at
  `0x8042FAB0` is `0x43300000_80000000` = `2^52 + 2^31` = `4503601774854144.0`,
  which is what makes the pair equal to the signed conversion; the body reduces
  it to `f64::from_bits(0x4330_0000_0000_0000 | (w ^ 0x8000_0000) as u64)` minus
  that value, and documents the identity once.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802E624C` | `fn_802e624c` | 5816 | medium |

Address range: `0x802E624C` (5816 bytes). With it the bucket is translated
everywhere except the two gaps `0x802EC9CC..0x802ECCA4` (the exception
prologue, which also holds the shared entry point `0x802ECBFC`) and
`0x802ECCB4..0x802ECDCC` (the exception epilogue).

## Deliberate skips

Two addresses in the bucket stay untranslated. Wave 105 and Wave 106 skipped
the same two because "the source side cannot be named honestly by a Rust
signature". This pass read the instruction words for both and found a stronger
reason: **their Ghidra exports do not describe their own instruction stream**, so
a body written from the export would be wrong, and a body written from the words
would be an assembly transliteration rather than a C-level reconstruction.
Concrete evidence, all from `work/dol/main.elf`:

- **The export prints `stmw` / `lmw` runs as individual field stores.** At
  `0x802ECCB4` the epilogue opens with
  `lis r3,0x8042; ori r3,r3,0x7198; stmw r0,0(r3); mfmsr r0;
  stw r0,0x8C(r3)` (`0x3C608042`, `0x60637198`, `0xBC030000`, `0x7C0000A6`,
  `0x9003008C`): one 32-word run stores `r0..r31` at
  `0x80427198..0x80427214`. The export instead prints
  `DAT_804271a4 = &DAT_80427198` and a list of `in_r0`, `unaff_r2`, `param_2`,
  ... assignments, and it never mentions the `stmw`. The prologue has the same
  shape twice: `0xBC820010` at `0x802ECADC` is
  `stmw r4,0x10(0x80427240)` (so `0x80427250..0x804272BC` holds `r4..r31`,
  including `r4`, which the export never stores), and `0xBF620080` at
  `0x802ECAF4` is `stmw r27,0x80(0x80427240)` after five `mfspr`s.
- **The export turns machine-register transfers into globals.** At
  `0x802ECA1C` the body is `mflr r3; stw r3,0x42C(0x80427240)` and at
  `0x802ECA30` it is `lwz r3,0x42C(0x80427240); mtlr r3`. Ghidra prints the
  first half as `DAT_8042766c = in_LR` and the second half as an `in_LR`
  merge. Likewise `0x7C5A03A6` / `0x7C9B03A6` at `0x802EC9CC` are
  `mtspr SRR0,r2` / `mtspr SRR1,r4` (a scratch stash), not the `in_r0` /
  `in_SPRG1` values the export stores.
- **Every exception return is printed as a pseudo-call.** Both exports show
  `returnFromInterrupt(msr, srr0)`; the words are `rfi` (`0x4C000064`) at
  `0x802ECA90`, `0x802ECBF8` and `0x802ECD74`, each preceded by a different
  teardown (`lwz` + `mtcrf` + `lwz r3` + `lwz r2` in the first, the
  `mfspr SRR0; addi 4; mtspr SRR0` + `mfsprg 3/1/2` + `mtcrf` sequence in the
  second, and `lmw r27,0x80(...)` + `mtspr SRR0` / `mtlr` / `mtcrf` / `mtctr` /
  `mtxer` in the third). There is no Rust construct that represents "restore
  the interrupted machine state and leave through `rfi`".
- **`0x802EC9CC`'s declared 744-byte extent contains a third entry point.**
  `0x802ECBFC` starts with `stwu r1,-0x20; mflr r0; stw r0,0x24(r1)`
  (`0x9421FFE0`, `0x7C0802A6`, `0x90010024`) and ends with its own epilogue at
  `0x802ECC9C`. Both the prologue (branch at `0x802ECB5C` →
  `0x802ECBFC`) and the epilogue (branch at `0x802ECDC8` → `0x802ECBFC`)
  branch into it. Ghidra merged it into the first function, which is why both
  exports contain the same event-queue block.

The decoded save map, kept here so the next pass does not have to redo it:

| address | contents | evidence |
|---|---|---|
| `0x80427240` | `r0` | `stw r0,0(r2)` at `0x802EC9F0` |
| `0x80427244` | `r1` (stack pointer) | `stw r1,4(r2)` at `0x802EC9F4` |
| `0x80427248` | `SPRG1` | `mfspr r0,SPRG1` + `stw r0,8(r2)` |
| `0x8042724C` | `SPRG2` | `mfspr r0,SPRG2` + `stw r0,0x0C(r2)` |
| `0x80427250..0x804272BC` | `r4..r31` | `stmw r4,0x10(r2)` |
| `0x804272C0` | `SRR0` | `mfspr r27,SRR0` + `stmw r27,0x80(r2)` |
| `0x804272C4` | `LR` | second word of that run |
| `0x804272C8` | `SPRG3` (holds the `mfcr` image) | third word |
| `0x804272CC` | `CTR` | fourth word |
| `0x804272D0` | `XER` | fifth word |
| `0x80427538` / `0x8042753A` | exception number, twice | `sth r3,0x2F8(r2)` / `0x2FA(r2)` |
| `0x8042B058` | exception number | `sth r3,0(r2)` at `0x802ECA08` |
| `0x8042766C` | `LR` in the `0x500` branch | `mflr r3` + `stw r3,0x42C(r2)` |
| `0x80427198..0x80427214` | `r0..r31` (epilogue frame) | `stmw r0,0(r3)` |
| `0x80427218` / `0x8042721C` / `0x80427220` | `LR` / `CTR` / `XER` | `mflr` / `mfctr` / `mfxer` + `stw` |
| `0x80427224` / `0x80427228` / `0x8042722C` | `MSR` / `DAR` / `DSISR` | `mfmsr` / `mfspr 19` / `mfspr 18` + `stw` |

Everything else in the bucket is represented by a body or by a call site that
names the address explicitly.

## Verification

- Every address and byte size matches
  `work/ghidra/main-gb/export/functions.tsv` (`802e624c` = 5816 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/engine/main_loop.rs`; the file now holds 201 bodies and no duplicates.
- No address was already in `ledger/functions.csv` (it held 200 of the 203
  bucket entries before this pass; 200 + 1 translated + 2 skipped = 203).
- `cargo check --message-format=short` reports no error and no warning for
  `src/engine/main_loop.rs`.
- `node tools/check-lane-csv.mjs wave111` reports `OK` for
  `ledger/agents/wave111-main-loop.csv`.
