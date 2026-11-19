# Coverage - Wave 106 (engine::main_loop)

## How to read this file

`src/engine/main_loop.rs` is the `0x802E0000` bucket: the statically linked C
runtime at the low end, MetroTRK in the middle and the interrupt / exception
knobs at the top. Before this pass the bucket had 193 of its 203 functions
translated; Wave 106 adds seven and leaves three.

Each row below cites the Ghidra name, the `functions.tsv` byte size and the
exact export path. "Confidence" is copied from the body's `# Confidence` line.
The body is `pub unsafe fn fn_<address>` in `src/engine/main_loop.rs`; the
address is always lowercase hex in the file name and in the ledger row.

Where Ghidra prints an empty body or drops a block, the body was recovered from
the instruction words in `work/dol/main.elf`. The PowerPC build emits the
original instruction under `#[cfg(target_arch = "powerpc")]`; the host twins are
documented stand-ins so `cargo check` succeeds, the same pattern as
`boot::cache` and `runtime::os`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/main_loop.rs` | `engine::main_loop` | 7 | 604 | 4 | 3 |

`high + medium + low = 4 + 3 + 0 = 7`. Before this pass the file held 193
bodies; it now holds 200.

## What this pass covers

- **Privileged MSR leaves** (`0x802EC1A0`, `0x802EC1A8`): the `mfmsr r3; blr`
  and `mtmsr r3; blr` pair that Wave 95 could only call by address. The exports
  are empty because Ghidra does not model the privileged read/write; the words
  are `0x7C6000A6` and `0x7C600124`. With the bodies in place, `fn_802e95cc`,
  `fn_802ec2a8`, `fn_802ec374` and `fn_802ec5bc` call them directly instead of
  going through `call0_ret` / `platform::call1`.
- **FPSCR accessors** (`0x802ED6A4`, `0x802ED6C8`): the pair behind the
  `spr == 0x20` branch of `fn_802ed6ec`. Each saves `f31` as a paired-single
  (`psq_st` / `psq_l`), which is what the decompiler printed as `ldexpf(GQR0)`
  probes. `0x802ED6A4` is `mffs f31` + `stfd f31,0(r3)` (read the FPSCR);
  `0x802ED6C8` is `lfd f31,0(r3)` + `mtfsf 255,f31` (install an FPSCR image).
  Ghidra removed the second pair as an unreachable block, so that body comes
  from the raw words `0xCBE30000` / `0xFDFEFD8E`.
- **Cache-range publish** (`0x802E8E1C`): the `dcbst` / `dcbf` / `sync` /
  `icbi` loop over an 8-byte step, with the `0xFFFFFFF1` start mask, the
  `subf`/`add` fold of the skipped lead bytes and the trailing `isync`. Three
  call sites (`fn_802ed8b4`, the `0xD1` request handler and `fn_802e9498`) now
  call the body directly.
- **Trap gate** (`0x802E890C`): `twi 31,r0,0; blr`, `TO = 31`, i.e. an
  unconditional trap. Its only caller, `fn_802ec0f4`, treats it as a
  four-argument routine returning a status byte, so the body keeps the
  `r3..r6` argument convention and returns the `r3` byte the trap handler
  leaves behind.
- **SPR dump** (`0x802EA4F8`): the 452-byte routine that fills
  `0x804273E8..0x80427568` with 96 register images. The store order and every
  destination are literal in the export; the SPR / SR numbers come from the
  Ghidra register names (`in_sr0`, `in_TBLr`, `in_IBAT0U`, `in_spr233`, ...)
  and from the implicit GQR0..7 sources of `unaff_GQR*`. The boot path now
  calls `fn_802ea4f8` directly.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802E890C` | `fn_802e890c` | 8 | medium |
| `0x802E8E1C` | `fn_802e8e1c` | 56 | high |
| `0x802EA4F8` | `fn_802ea4f8` | 452 | medium |
| `0x802EC1A0` | `fn_802ec1a0` | 8 | high |
| `0x802EC1A8` | `fn_802ec1a8` | 8 | high |
| `0x802ED6A4` | `fn_802ed6a4` | 36 | high |
| `0x802ED6C8` | `fn_802ed6c8` | 36 | medium |

Address range: `0x802E890C..0x802ED6C8` (604 bytes).

## Deliberate skips

Three addresses in the bucket stay untranslated. Each has a concrete reason, not
a shortage of time.

- **`0x802E624C`** - the 5816-byte Payne-Hanek tail (`__kernel_rem_pio2`). The
  export is an eight-way unrolled loop in which every integer/`double`
  conversion is the raw `CONCAT44(0x43300000, x) ^ 0x80000000 - 2^52` magic
  number trick, with roughly sixty live locals. A faithful transliteration is a
  dedicated pass; a partial rewrite would silently change the reduction.
  [`fn_802e5db8`] keeps calling it by absolute address.
- **`0x802EC9CC`** - the 744-byte exception prologue. Its source side is
  `in_r0`, `in_cr0..7`, `in_XER` / `LR` / `CTR` / `SPRG1..3`, `in_MSR` and
  `unaff_r2` / `unaff_r13..r31`. Those are not the routine's own registers:
  the vector stub at the top of the bucket has already packed the interrupted
  context for it, so an asm block that reads the *current* registers would
  capture the wrong values. The function also ends in
  `returnFromInterrupt(uVar8, param_2)`. The literal destination map
  (`0x80427240..0x804272c8`) is documented in `fn_802ec2a8`'s callers, but the
  source side cannot be named honestly by a Rust signature.
- **`0x802ECCB4`** - the 280-byte exception epilogue, the same problem plus
  `in_DAR` / `in_DSISR` / `in_SRR1` and the `register0x00000004` stack pointer.
  Ghidra itself warns that it "may have set the stack pointer".

Everything else in the bucket is now represented by a body or by a call site
that names the address explicitly.

## Verification

- Every address and byte size matches
  `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/engine/main_loop.rs`; the file now holds 200 bodies and no duplicates.
- No address was already in `ledger/functions.csv` (it held 193 of the 203
  bucket entries before this pass; 193 + 7 translated + 3 skipped = 203).
- `cargo check --message-format=short` reports no error and no warning for
  `src/engine/main_loop.rs`.
- `node tools/check-lane-csv.mjs wave106` reports `OK` for
  `ledger/agents/wave106-main-loop.csv`.
