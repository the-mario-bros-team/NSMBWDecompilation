# Analysis - Wave 127 (`game::bases`) - read-only reclassification

## Purpose

This is a **read-only** reclassification lane. No Rust file, ledger row or
existing document was modified. Its only output is this report.

Waves 11 through 126 rejected a large number of `d_basesNP` functions, usually
with a reason of the form "Ghidra prints `unaff_GQR0`", "the export drops the
receiver" or "budget, not doubt". This pass goes back to the shipped artifact -
the raw instruction stream in `work/rels/elf/d_basesNP.elf` - and asks, for
every still-unclaimed skip, whether the bytes actually contain enough
information to write the body today.

The answer is **yes far more often than the skip lists imply**: 121 of 319
re-examined addresses are reconstructible now, and another 25 need only one
external constant or a table generator.

## Method

1. **Skip lists.** Every `## Deliberate skips` / `## Deferred candidates`
   section in `docs/COVERAGE_WAVE11_BASES.md` through
   `docs/COVERAGE_WAVE126_BASES.md` was read (waves 11, 12, 110-119 and
   120-126). Addresses cited only as *comparisons* ("same reason as X") were
   discarded; only the leading address list of each bullet or table row counts.
2. **Ledger filter.** `ledger/functions.csv` was parsed read-only. The bullet
   lists yielded 310 addresses; nine still-unclaimed wave-11/12 table rows were
   added, for 319. Everything already present in the ledger - or already
   carrying a `pub unsafe fn` in `src/game/bases.rs` - was removed first.
   Several wave-11/12 addresses had since been claimed by later waves and
   dropped out here.
3. **Evidence.** Sizes came from
   `work/ghidra/rel-d_basesNP/export/functions.tsv`; each export was also
   inspected for `halt_baddata`, `unaff_GQR` and "could not recover jumptable"
   warnings.
4. **Raw stream.** A throwaway PowerPC decoder (kept in
   `%TEMP%\opencode\ppc.js`, never in the repo) read the ELF32-BE program
   headers and decoded `psq_l`/`psq_st`, `mtspr`/`mfspr`, `bl`, `mtctr`/
   `bctr`, `lis`/`addi` and the integer load/store set.
   The ELF's first `PT_LOAD` starts at `0x806000F0`, so the documented
   `offset = address - 0x80600000` rule is exact for every address here; the
   first `0xF0` bytes of the synthetic base are not mapped.
5. **Duplicates.** The SHA-1 of each candidate's bytes was compared against the
   bytes of every one of the 2,404 `game::bases` rows already in the ledger.
   17 candidates are byte-identical to an already-translated body.

## What the raw stream settles

- **The GQR slot is not unknown - it is an instruction field.** Every rejection
  that says "which GQR slot feeds each conversion is not decidable" is a
  decompiler artifact. `psq_l`/`psq_st` encode the GQR index in bits 12-14,
  the `W` bit in bit 15, the destination/source register in bits 21-25, the
  base register in bits 16-20 and the signed `DQ` displacement in bits 0-11.
  In this REL the only indices ever used are **0** and **3** (90 of the 319
  candidates contain `psq`; no other index appears).
- **`qr0` is the float-pair slot.** A `psq_l ...,qr0` is followed by ordinary
  `fadds`/`fmuls`/`stfs` on the result, and `psq_st` stores an ordinary
  float. Type 0 (32-bit float) is the only value that makes that code correct,
  so `qr0` bodies are fully reconstructible. In most of them the only `qr0`
  ops are the prologue's `stfd f31` + `psq_st f31` pair that saves the
  paired-single half of `f31`; those bodies have no data-side quantization at
  all.
- **`qr3` is the quantized slot.** `psq_l ...,qr3` feeds
  `ps_merge`/`CONCAT22`-style lane assembly. The slot, base, `W` bit and
  offset are explicit; only the **value** of GQR3 (the type/scale word) is not
  in this REL. No `mtspr GQRn` exists anywhere in `d_basesNP` (a full-text
  scan found zero), so that value is set once by `main.dol`'s
  `OSInitFastCast`. It is a single fixed constant: pin it once and all 24
  `qr3` bodies become mechanical.
- **The `0x802DCD40..0x802DCD74` "save stubs" do not destroy arguments.**
  They are the CodeWarrior `_savegpr_N` family: they save rN..r31 and LR,
  and leave r3-r10 untouched. A body that does `bl 0x802DCD7x` and then
  `mr r27,r3` still has its receiver in **entry r3**; the export merely fails
  to show it. The "receiver-less" rejections are therefore recoverable by a
  register walk. This lane still classes the largest of them **C** (the walk
  is work, not doubt) and the small ones **B**.
- **The `0x802DCBFC` forwarders are genuinely opaque.** All 125 of them are
  48 bytes: `stwu`/`mflr`, `mr r5,r3`, `mr r3,r4`, `addi r12,r5,off`,
  `bl 0x802DCBFC`, epilogue. The two unnamed arguments are re-emitted through
  a hidden `r12` adjustment and no target list survives. These stay **D**.
- **Most `halt_baddata` bodies really are undecodable.** 28 candidates end at
  a word Ghidra cannot decode. A few (e.g. `0x806D3AC0`) have a readable
  guard, but the guarded path also ends in the same cut, so they remain **D**.
- **Some "jumptable" warnings are false.** `0x8069ACA0` was skipped as an
  unrecovered jump table; the raw stream is `lwz r12,0x60(r3)` /
  `lwz r12,0xD4(r12)` / `mtctr r12` / `bctr` - a plain virtual tail call.
  It is reclassified **B**.

## Summary

| Class | Meaning | Addresses | Bytes |
|---|---|---:|---:|
| **B** | reconstructible now from the raw stream | 121 | 51060 |
| **C** | needs one more artifact (GQR3 constant) or a table generator | 25 | 11988 |
| **D** | genuinely unreconstructable (thunk, halt, getter, duplicate) | 173 | 7284 |
| | **total** | **319** | **70332** |

### Per source wave

| Wave | B | C | D | total |
|---|---:|---:|---:|---:|
| 11 | 7 | 1 | 0 | 8 |
| 12 | 1 | 0 | 0 | 1 |
| 110 | 2 | 0 | 6 | 8 |
| 111 | 3 | 0 | 2 | 5 |
| 112 | 6 | 0 | 1 | 7 |
| 113 | 11 | 1 | 0 | 12 |
| 114 | 4 | 0 | 1 | 5 |
| 115 | 16 | 0 | 13 | 29 |
| 116 | 9 | 0 | 8 | 17 |
| 117 | 6 | 0 | 6 | 12 |
| 118 | 9 | 5 | 23 | 37 |
| 119 | 10 | 0 | 2 | 12 |
| 120 | 5 | 0 | 11 | 16 |
| 121 | 6 | 2 | 14 | 22 |
| 122 | 3 | 6 | 24 | 33 |
| 123 | 7 | 6 | 6 | 19 |
| 124 | 7 | 0 | 16 | 23 |
| 125 | 8 | 3 | 16 | 27 |
| 126 | 1 | 1 | 24 | 26 |

## Full reclassification

Rows are ordered B, C, D, then by address. "Bytes" is the `functions.tsv`
size. The prior reason is the skip bullet or table row that rejected it.

| address | bytes | prior reason | class | concrete note |
|---|---:|---|---|---|
| `0x80600990` | 64 | WAVE116 — Delete wrappers. | **B** | 1 direct calls, no psq, no save-helper prologue |
| `0x806009D0` | 64 | WAVE116 — Delete wrappers. | **B** | 1 direct calls, no psq, no save-helper prologue |
| `0x80600A10` | 64 | WAVE116 — Delete wrappers. | **B** | 1 direct calls, no psq, no save-helper prologue |
| `0x80600A50` | 64 | WAVE116 — Delete wrappers. | **B** | 1 direct calls, no psq, no save-helper prologue |
| `0x80600A90` | 64 | WAVE116 — Delete wrappers. | **B** | 1 direct calls, no psq, no save-helper prologue |
| `0x80600AD0` | 64 | WAVE116 — Delete wrappers. | **B** | 1 direct calls, no psq, no save-helper prologue |
| `0x80601D10` | 64 | WAVE116 — Delete wrappers. | **B** | 1 direct calls, no psq, no save-helper prologue |
| `0x80601EB0` | 64 | WAVE116 — Delete wrappers. | **B** | 1 direct calls, no psq, no save-helper prologue |
| `0x80605500` | 64 | WAVE116 — Delete wrappers. | **B** | 1 direct calls, no psq, no save-helper prologue |
| `0x80608C40` | 132 | WAVE11 — collection-coin handler | **B** | ends in a virtual tail call; raw stream carries the full body |
| `0x80608D10` | 136 | WAVE11 — collection-coin handler | **B** | ends in a virtual tail call; raw stream carries the full body |
| `0x80608DF0` | 248 | WAVE11 — collection-coin handler | **B** | straight-line body, no psq, no indirect dispatch |
| `0x806329E0` | 264 | WAVE111 — Paired-single bodies. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x80632DC0` | 884 | WAVE111 — Paired-single bodies. | **B** | no data psq; 3 qr0 ops are the f31 stack save/restore |
| `0x806333D0` | 416 | WAVE111 — Paired-single bodies. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x80642540` | 1236 | WAVE112 — Budget. | **B** | no data psq; 2 qr0 ops are the f31 stack save/restore |
| `0x80642A20` | 660 | WAVE112 — Budget. | **B** | prologue calls compiler save-helper 0x802DCD5C; r3-r10 survive, param_1 = entry r3 |
| `0x80642D20` | 204 | WAVE112 — Paired-single bodies. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x80643470` | 228 | WAVE112 — Dropped-receiver calls. | **B** | 6 direct calls, no psq, no save-helper prologue |
| `0x80643D40` | 268 | WAVE112 — Dropped-receiver calls. | **B** | 2 direct calls, no psq, no save-helper prologue |
| `0x80647E60` | 512 | WAVE112 — Budget. | **B** | ends in a virtual tail call; raw stream carries the full body |
| `0x80656260` | 172 | WAVE120 — Paired-single / GQR reconstruction failures. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x80656310` | 396 | WAVE120 — Paired-single / GQR reconstruction failures. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x80656D20` | 392 | WAVE120 — Jump-table / event bodies left for a later sweep. | **B** | ends in a virtual tail call; raw stream carries the full body |
| `0x8065C100` | 264 | WAVE120 — Paired-single / GQR reconstruction failures. | **B** | no data psq; 2 qr0 ops are the f31 stack save/restore |
| `0x8065C260` | 264 | WAVE120 — Paired-single / GQR reconstruction failures. | **B** | no data psq; 2 qr0 ops are the f31 stack save/restore |
| `0x8066B0F0` | 128 | WAVE126 — Receiver-less dispatch bodies. | **B** | ends in a virtual tail call; raw stream carries the full body |
| `0x8066FA10` | 508 | WAVE11 — castle-demo update | **B** | no data psq; 4 qr0 ops are the f31 stack save/restore |
| `0x80673F50` | 656 | WAVE12 — Lemmy throw-ball spawn | **B** | no data psq; 3 qr0 ops are the f31 stack save/restore |
| `0x80681480` | 88 | WAVE123 — Duplicate release twins. | **B** | 2 direct calls, no psq, no save-helper prologue |
| `0x80681FA0` | 88 | WAVE123 — Duplicate release twins. | **B** | 2 direct calls, no psq, no save-helper prologue |
| `0x806822B0` | 1108 | WAVE123 — Long bodies outside the windows. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x80684680` | 936 | WAVE123 — Long bodies outside the windows. | **B** | no data psq; 2 qr0 ops are the f31 stack save/restore |
| `0x80686550` | 1884 | WAVE123 — Long bodies outside the windows. | **B** | no data psq; 8 qr0 ops are the f31 stack save/restore |
| `0x8068E0B0` | 912 | WAVE123 — Paired-single / GQR bodies. | **B** | no data psq; 4 qr0 ops are the f31 stack save/restore |
| `0x8068E440` | 228 | WAVE123 — Paired-single / GQR bodies. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x8069A170` | 184 | WAVE119 — Paired-single / GQR reconstruction failures. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x8069A6E0` | 236 | WAVE119 — Paired-single / GQR reconstruction failures. | **B** | 1 data psq qr0 (e.g. psq_l d0,0(r31),w0); qr0 = float pair |
| `0x8069AB00` | 408 | WAVE119 — Paired-single / GQR reconstruction failures. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x8069ACA0` | 40 | WAVE119 — Unrecovered jump table. | **B** | `lwz r12,0x60(r3)`/`lwz r12,0xD4(r12)`/`mtctr`/`bctr`; Ghidra jumptable warning is spurious |
| `0x8069B750` | 1424 | WAVE119 — Paired-single / GQR reconstruction failures. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x8069E5F0` | 204 | WAVE119 — Receiver-less `0x802DCD70` bodies. | **B** | prologue calls compiler save-helper 0x802DCD70; r3-r10 survive, param_1 = entry r3 |
| `0x8069F560` | 208 | WAVE119 — Receiver-less `0x802DCD70` bodies. | **B** | prologue calls compiler save-helper 0x802DCD70; r3-r10 survive, param_1 = entry r3 |
| `0x8069F640` | 892 | WAVE119 — Long bodies left for a later sweep. | **B** | ends in a virtual tail call; raw stream carries the full body |
| `0x8069F9D0` | 208 | WAVE119 — Receiver-less `0x802DCD70` bodies. | **B** | prologue calls compiler save-helper 0x802DCD70; r3-r10 survive, param_1 = entry r3 |
| `0x8069FAB0` | 996 | WAVE119 — Long bodies left for a later sweep. | **B** | ends in a virtual tail call; raw stream carries the full body |
| `0x806A0290` | 948 | WAVE124 — Large per-frame body. | **B** | prologue calls compiler save-helper 0x802DCD74; r3-r10 survive, param_1 = entry r3 |
| `0x806A23B0` | 308 | WAVE124 — Paired-single / GQR conversions. | **B** | no data psq; 3 qr0 ops are the f31 stack save/restore |
| `0x806A4CA0` | 504 | WAVE124 — Paired-single / GQR conversions. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x806A53E0` | 964 | WAVE124 — Paired-single / GQR conversions. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x806A6C50` | 780 | WAVE124 — Long container sweep. | **B** | prologue calls compiler save-helper 0x802DCD70; r3-r10 survive, param_1 = entry r3 |
| `0x806A75B0` | 392 | WAVE124 — Paired-single / GQR conversions. | **B** | no data psq; 2 qr0 ops are the f31 stack save/restore |
| `0x806A8AA0` | 184 | WAVE124 — Paired-single / GQR conversions. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x806B39B0` | 268 | WAVE118 — Receiver-less prologue fragments. | **B** | prologue calls compiler save-helper 0x802DCD70; r3-r10 survive, param_1 = entry r3 |
| `0x806B3AC0` | 136 | WAVE118 — `undefined8` fragment. | **B** | 3 direct calls, no psq, no save-helper prologue |
| `0x806B4F90` | 208 | WAVE118 — Paired-single / GQR quantization bodies. | **B** | no data psq; 2 qr0 ops are the f31 stack save/restore |
| `0x806B5180` | 620 | WAVE118 — Receiver-less prologue fragments. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x806B5E00` | 892 | WAVE118 — Paired-single / GQR quantization bodies. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x806B7340` | 368 | WAVE118 — Paired-single / GQR quantization bodies. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x806B7690` | 312 | WAVE118 — Paired-single / GQR quantization bodies. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x806B7890` | 336 | WAVE118 — Paired-single / GQR quantization bodies. | **B** | no data psq; 3 qr0 ops are the f31 stack save/restore |
| `0x806B7B70` | 344 | WAVE118 — Paired-single / GQR quantization bodies. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x806D3C20` | 784 | WAVE122 — Long per-frame tails. | **B** | no data psq; 3 qr0 ops are the f31 stack save/restore |
| `0x806DED20` | 688 | WAVE122 — Long per-frame tails. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x806DF630` | 308 | WAVE122 — Paired-single / GQR conversion failures. | **B** | no data psq; 2 qr0 ops are the f31 stack save/restore |
| `0x806F3350` | 2092 | WAVE113 — Large bodies not opened. | **B** | prologue calls compiler save-helper 0x802DCD40; r3-r10 survive, param_1 = entry r3 |
| `0x806F7BB0` | 1004 | WAVE113 — Large bodies not opened. | **B** | ends in a virtual tail call; raw stream carries the full body |
| `0x806FB540` | 200 | WAVE113 — Paired-single bodies. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x806FCC60` | 64 | WAVE113 — `0x806FCC60..0x806FCDF0` neighbours not in this window. | **B** | 1 direct calls, no psq, no save-helper prologue |
| `0x806FCCA0` | 100 | WAVE113 — `0x806FCC60..0x806FCDF0` neighbours not in this window. | **B** | 2 direct calls, no psq, no save-helper prologue |
| `0x806FCD70` | 64 | WAVE113 — `0x806FCC60..0x806FCDF0` neighbours not in this window. | **B** | 1 direct calls, no psq, no save-helper prologue |
| `0x806FCDB0` | 64 | WAVE113 — `0x806FCC60..0x806FCDF0` neighbours not in this window. | **B** | 1 direct calls, no psq, no save-helper prologue |
| `0x806FCDF0` | 268 | WAVE113 — `0x806FCC60..0x806FCDF0` neighbours not in this window. | **B** | 7 direct calls, no psq, no save-helper prologue |
| `0x806FD820` | 912 | WAVE113 — Large bodies not opened. | **B** | no data psq; 2 qr0 ops are the f31 stack save/restore |
| `0x806FDD50` | 536 | WAVE113 — Large bodies not opened. | **B** | ends in a virtual tail call; raw stream carries the full body |
| `0x806FE480` | 892 | WAVE113 — Large bodies not opened. | **B** | ends in a virtual tail call; raw stream carries the full body |
| `0x806FF3F0` | 88 | WAVE11 — wire-mesh lifecycle | **B** | 2 direct calls, no psq, no save-helper prologue |
| `0x806FF700` | 40 | WAVE11 — wire-mesh lifecycle | **B** | 1 direct calls, no psq, no save-helper prologue |
| `0x806FFB50` | 256 | WAVE11 — wire-mesh lifecycle | **B** | 7 direct calls, no psq, no save-helper prologue |
| `0x80702AA0` | 276 | WAVE121 — Paired-single / GQR reconstruction failures. | **B** | no data psq; 6 qr0 ops are the f31 stack save/restore |
| `0x80702BC0` | 276 | WAVE121 — Paired-single / GQR reconstruction failures. | **B** | no data psq; 6 qr0 ops are the f31 stack save/restore |
| `0x80702CE0` | 168 | WAVE121 — Paired-single / GQR reconstruction failures. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x8070D1F0` | 1200 | WAVE121 — Long per-frame tails. | **B** | ends in a virtual tail call; raw stream carries the full body |
| `0x8070D790` | 584 | WAVE121 — Contact body. | **B** | ends in a virtual tail call; raw stream carries the full body |
| `0x8070DC70` | 1196 | WAVE121 — Long per-frame tails. | **B** | ends in a virtual tail call; raw stream carries the full body |
| `0x80711590` | 272 | WAVE110 — GQR / paired-single bodies. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x80716590` | 148 | WAVE110 — Dropped-`this` mixed bodies. | **B** | 4 direct calls, no psq, no save-helper prologue |
| `0x80722180` | 296 | WAVE125 — Paired-single / GQR conversions. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x807241C0` | 256 | WAVE125 — Paired-single / GQR conversions. | **B** | no data psq; 2 qr0 ops are the f31 stack save/restore |
| `0x80724460` | 276 | WAVE125 — Paired-single / GQR conversions. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x807247D0` | 584 | WAVE125 — Paired-single / GQR conversions. | **B** | prologue calls compiler save-helper 0x802DCD74; r3-r10 survive, param_1 = entry r3 |
| `0x80724CD0` | 596 | WAVE125 — Paired-single / GQR conversions. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x80725170` | 196 | WAVE125 — Paired-single / GQR conversions. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x807268C0` | 248 | WAVE125 — Paired-single / GQR conversions. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x8072FE10` | 684 | WAVE125 — Paired-single / GQR conversions. | **B** | prologue calls compiler save-helper 0x802DCD6C; r3-r10 survive, param_1 = entry r3 |
| `0x80740370` | 552 | WAVE117 — Mangled paired-single bodies. | **B** | no data psq; 3 qr0 ops are the f31 stack save/restore |
| `0x80747B30` | 224 | WAVE117 — Mangled paired-single bodies. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x807481B0` | 172 | WAVE117 — Receiver-less prologue fragments. | **B** | prologue calls compiler save-helper 0x802DCD74; r3-r10 survive, param_1 = entry r3 |
| `0x807485D0` | 268 | WAVE117 — Mangled paired-single bodies. | **B** | no data psq; 4 qr0 ops are the f31 stack save/restore |
| `0x80748B20` | 520 | WAVE117 — Mangled paired-single bodies. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x80749750` | 360 | WAVE117 — Mangled paired-single bodies. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x80761510` | 340 | WAVE114 — Paired-single bodies. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x807616E0` | 408 | WAVE114 — Paired-single bodies. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x80761880` | 176 | WAVE114 — Paired-single bodies. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x80762CE0` | 440 | WAVE114 — `0x80762CE0` (440) and `0x80762B20` (16). | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x80780010` | 376 | WAVE115 — `character_SV` GQR bodies. | **B** | 6 direct calls, no psq, no save-helper prologue |
| `0x807807E0` | 604 | WAVE115 — `character_SV` GQR bodies. | **B** | prologue calls compiler save-helper 0x802DCD74; r3-r10 survive, param_1 = entry r3 |
| `0x80780A90` | 240 | WAVE115 — `character_SV` GQR bodies. | **B** | no data psq; 2 qr0 ops are the f31 stack save/restore |
| `0x80780BD0` | 292 | WAVE115 — `character_SV` GQR bodies. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x80780D00` | 276 | WAVE115 — `character_SV` GQR bodies. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x80780E20` | 292 | WAVE115 — `character_SV` GQR bodies. | **B** | 6 direct calls, no psq, no save-helper prologue |
| `0x80780FA0` | 520 | WAVE115 — `character_SV` GQR bodies. | **B** | no data psq; 2 qr0 ops are the f31 stack save/restore |
| `0x807811D0` | 256 | WAVE115 — `character_SV` GQR bodies. | **B** | 1 direct calls, no psq, no save-helper prologue |
| `0x807813F0` | 696 | WAVE115 — `character_SV` GQR bodies. | **B** | no data psq; 2 qr0 ops are the f31 stack save/restore |
| `0x807816B0` | 460 | WAVE115 — `character_SV` GQR bodies. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x807819B0` | 388 | WAVE115 — `character_SV` GQR bodies. | **B** | no data psq; 1 qr0 ops are the f31 stack save/restore |
| `0x80781B60` | 208 | WAVE115 — `character_SV` GQR bodies. | **B** | 7 direct calls, no psq, no save-helper prologue |
| `0x80781D10` | 152 | WAVE115 — `character_SV` GQR bodies. | **B** | 3 direct calls, no psq, no save-helper prologue |
| `0x80781DB0` | 564 | WAVE115 — `character_SV` GQR bodies. | **B** | 8 data psq qr0 (e.g. psq_l d1,0(r31),w0); qr0 = float pair |
| `0x8078B120` | 168 | WAVE115 — ABI-mangled broom placement. | **B** | 4 direct calls, no psq, no save-helper prologue |
| `0x8078B1D0` | 340 | WAVE115 — ABI-mangled broom placement. | **B** | no data psq; 3 qr0 ops are the f31 stack save/restore |
| `0x8066C9A0` | 232 | WAVE126 — Paired-single / GQR conversions. | **C** | 1 data psq with qr3; slot/D/A/DQ explicit, only the GQR3 type word is external |
| `0x80680720` | 660 | WAVE123 — Paired-single / GQR bodies. | **C** | 4 data psq with qr3; slot/D/A/DQ explicit, only the GQR3 type word is external |
| `0x80680E80` | 292 | WAVE123 — Paired-single / GQR bodies. | **C** | 2 data psq with qr3; slot/D/A/DQ explicit, only the GQR3 type word is external |
| `0x80681C50` | 384 | WAVE123 — Paired-single / GQR bodies. | **C** | 1 data psq with qr3; slot/D/A/DQ explicit, only the GQR3 type word is external |
| `0x80688BA0` | 356 | WAVE123 — Paired-single / GQR bodies. | **C** | 4 data psq with qr3; slot/D/A/DQ explicit, only the GQR3 type word is external |
| `0x80688E10` | 356 | WAVE123 — Paired-single / GQR bodies. | **C** | 4 data psq with qr3; slot/D/A/DQ explicit, only the GQR3 type word is external |
| `0x8068EA10` | 332 | WAVE123 — Paired-single / GQR bodies. | **C** | 1 data psq with qr3; slot/D/A/DQ explicit, only the GQR3 type word is external |
| `0x806B2BB0` | 340 | WAVE118 — Paired-single / GQR quantization bodies. | **C** | 1 data psq with qr3; slot/D/A/DQ explicit, only the GQR3 type word is external |
| `0x806B2D10` | 200 | WAVE118 — Paired-single / GQR quantization bodies. | **C** | 2 data psq with qr3; slot/D/A/DQ explicit, only the GQR3 type word is external |
| `0x806B3770` | 308 | WAVE118 — Paired-single / GQR quantization bodies. | **C** | 2 data psq with qr3; slot/D/A/DQ explicit, only the GQR3 type word is external |
| `0x806B7CD0` | 1704 | WAVE118 — Paired-single / GQR quantization bodies. | **C** | 2 data psq with qr3; slot/D/A/DQ explicit, only the GQR3 type word is external |
| `0x806B9A30` | 476 | WAVE118 — Paired-single / GQR quantization bodies. | **C** | 2 data psq with qr3; slot/D/A/DQ explicit, only the GQR3 type word is external |
| `0x806D6530` | 104 | WAVE122 — Paired-single / GQR conversion failures. | **C** | 1 data psq with qr3; slot/D/A/DQ explicit, only the GQR3 type word is external |
| `0x806D65A0` | 112 | WAVE122 — Paired-single / GQR conversion failures. | **C** | 1 data psq with qr3; slot/D/A/DQ explicit, only the GQR3 type word is external |
| `0x806DD730` | 736 | WAVE122 — Long per-frame tails. | **C** | 2 data psq with qr3; slot/D/A/DQ explicit, only the GQR3 type word is external |
| `0x806DDA10` | 88 | WAVE122 — Paired-single / GQR conversion failures. | **C** | 1 data psq with qr3; slot/D/A/DQ explicit, only the GQR3 type word is external |
| `0x806DE0B0` | 132 | WAVE122 — Paired-single / GQR conversion failures. | **C** | 1 data psq with qr3; slot/D/A/DQ explicit, only the GQR3 type word is external |
| `0x806DE320` | 132 | WAVE122 — Paired-single / GQR conversion failures. | **C** | 1 data psq with qr3; slot/D/A/DQ explicit, only the GQR3 type word is external |
| `0x806FB610` | 296 | WAVE113 — Paired-single bodies. | **C** | 2 data psq with qr3; slot/D/A/DQ explicit, only the GQR3 type word is external |
| `0x80704050` | 108 | WAVE121 — Paired-single / GQR reconstruction failures. | **C** | 1 data psq with qr3; slot/D/A/DQ explicit, only the GQR3 type word is external |
| `0x807040C0` | 112 | WAVE121 — Paired-single / GQR reconstruction failures. | **C** | 1 data psq with qr3; slot/D/A/DQ explicit, only the GQR3 type word is external |
| `0x8072BC40` | 604 | WAVE125 — Paired-single / GQR conversions. | **C** | 2 data psq with qr3; slot/D/A/DQ explicit, only the GQR3 type word is external |
| `0x8072C000` | 372 | WAVE125 — Paired-single / GQR conversions. | **C** | 2 data psq with qr3; slot/D/A/DQ explicit, only the GQR3 type word is external |
| `0x8072E520` | 644 | WAVE125 — Paired-single / GQR conversions. | **C** | 2 data psq with qr3; slot/D/A/DQ explicit, only the GQR3 type word is external |
| `0x807BD110` | 2908 | WAVE11 — dFileSelect table family | **C** | state-registration table (44 `StateID` refs); needs table generator, not control flow |
| `0x80601780` | 48 | WAVE116 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806017B0` | 48 | WAVE116 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80604360` | 48 | WAVE116 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80604390` | 48 | WAVE116 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806064E0` | 48 | WAVE116 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80606510` | 48 | WAVE116 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80608350` | 48 | WAVE116 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80608380` | 48 | WAVE116 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80634C90` | 48 | WAVE111 — `0x802DCBFC` indirect-jump stubs. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80634CC0` | 48 | WAVE111 — `0x802DCBFC` indirect-jump stubs. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80640AC0` | 116 | WAVE112 — `halt_baddata` bodies. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x80651590` | 24 | WAVE120 — Bad-instruction truncation. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x8065B4C0` | 48 | WAVE120 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x8065B4F0` | 48 | WAVE120 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x8065BE20` | 48 | WAVE120 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x8065BE50` | 48 | WAVE120 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x8065BFB0` | 12 | WAVE120 — Six 12-byte vtable getters. | **D** | byte-identical to an already-translated getter (duplicate) |
| `0x8065BFC0` | 12 | WAVE120 — Six 12-byte vtable getters. | **D** | byte-identical to an already-translated getter (duplicate) |
| `0x8065BFD0` | 12 | WAVE120 — Six 12-byte vtable getters. | **D** | byte-identical to an already-translated getter (duplicate) |
| `0x8065BFE0` | 12 | WAVE120 — Six 12-byte vtable getters. | **D** | byte-identical to an already-translated getter (duplicate) |
| `0x8065BFF0` | 12 | WAVE120 — Six 12-byte vtable getters. | **D** | byte-identical to an already-translated getter (duplicate) |
| `0x8065C000` | 12 | WAVE120 — Six 12-byte vtable getters. | **D** | byte-identical to an already-translated getter (duplicate) |
| `0x80662700` | 12 | WAVE126 — Bare constant getters. | **D** | two-instruction constant return (`lis`+`addi`+`blr`) |
| `0x80665120` | 12 | WAVE126 — `0x802DCBFC` forwarders. | **D** | byte-identical to an already-translated getter (duplicate) |
| `0x80665130` | 12 | WAVE126 — `0x802DCBFC` forwarders. | **D** | byte-identical to an already-translated getter (duplicate) |
| `0x80665140` | 12 | WAVE126 — `0x802DCBFC` forwarders. | **D** | byte-identical to an already-translated getter (duplicate) |
| `0x80665150` | 12 | WAVE126 — `0x802DCBFC` forwarders. | **D** | two-instruction constant return (`lis`+`addi`+`blr`) |
| `0x806654F0` | 12 | WAVE126 — `0x802DCBFC` forwarders. | **D** | byte-identical to an already-translated getter (duplicate) |
| `0x80665500` | 12 | WAVE126 — `0x802DCBFC` forwarders. | **D** | byte-identical to an already-translated getter (duplicate) |
| `0x80665510` | 12 | WAVE126 — `0x802DCBFC` forwarders. | **D** | byte-identical to an already-translated getter (duplicate) |
| `0x80665520` | 12 | WAVE126 — `0x802DCBFC` forwarders. | **D** | byte-identical to an already-translated getter (duplicate) |
| `0x80666AD0` | 12 | WAVE126 — `0x802DCBFC` forwarders. | **D** | byte-identical to an already-translated getter (duplicate) |
| `0x80666AE0` | 12 | WAVE126 — `0x802DCBFC` forwarders. | **D** | byte-identical to an already-translated getter (duplicate) |
| `0x80667780` | 48 | WAVE126 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806677B0` | 48 | WAVE126 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80667CF0` | 8 | WAVE126 — `halt_baddata` bodies. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x80668420` | 48 | WAVE126 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80668450` | 48 | WAVE126 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x8066A7C0` | 48 | WAVE126 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x8066A7F0` | 48 | WAVE126 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x8066B470` | 12 | WAVE126 — Bare constant getters. | **D** | two-instruction constant return (`lis`+`addi`+`blr`) |
| `0x8066BE10` | 48 | WAVE126 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x8066BE40` | 48 | WAVE126 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x8066D530` | 48 | WAVE126 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x8066D560` | 48 | WAVE126 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x8066F6A0` | 48 | WAVE126 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806815A0` | 48 | WAVE123 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806815D0` | 48 | WAVE123 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806820C0` | 48 | WAVE123 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806820F0` | 48 | WAVE123 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806899E0` | 48 | WAVE123 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80689A10` | 48 | WAVE123 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x8069DB30` | 48 | WAVE119 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x8069DB60` | 48 | WAVE119 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806A1560` | 48 | WAVE124 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806A1590` | 48 | WAVE124 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806A2230` | 48 | WAVE124 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806A2260` | 48 | WAVE124 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806A6080` | 48 | WAVE124 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806A60B0` | 48 | WAVE124 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806A6A70` | 12 | WAVE124 — `halt_baddata` bodies. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x806A6A80` | 32 | WAVE124 — `halt_baddata` bodies. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x806A9550` | 48 | WAVE124 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806A9580` | 48 | WAVE124 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806A9670` | 48 | WAVE124 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806A96A0` | 48 | WAVE124 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806AF150` | 48 | WAVE124 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806AF180` | 48 | WAVE124 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806AF860` | 48 | WAVE124 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806AF890` | 48 | WAVE124 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806B0210` | 48 | WAVE118 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806B0240` | 48 | WAVE118 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806B14A0` | 48 | WAVE118 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806B14D0` | 48 | WAVE118 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806B1AE0` | 40 | WAVE118 — Truncated bodies. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x806B26D0` | 48 | WAVE118 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806B2700` | 48 | WAVE118 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806B3360` | 48 | WAVE118 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806B3390` | 48 | WAVE118 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806B4810` | 48 | WAVE118 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806B4840` | 48 | WAVE118 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806B4DB0` | 12 | WAVE118 — Sibling constant getters. | **D** | byte-identical to an already-translated getter (duplicate) |
| `0x806B6D80` | 48 | WAVE118 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806B6DB0` | 48 | WAVE118 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806B8E10` | 48 | WAVE118 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806B8E40` | 48 | WAVE118 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806B9720` | 32 | WAVE118 — Truncated bodies. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x806BC5B0` | 48 | WAVE118 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806BC5E0` | 48 | WAVE118 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806BEAD0` | 48 | WAVE118 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806BEB00` | 48 | WAVE118 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806BF5D0` | 48 | WAVE118 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806BF600` | 48 | WAVE118 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806D1410` | 48 | WAVE122 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806D1440` | 48 | WAVE122 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806D1ED0` | 28 | WAVE122 — `halt_baddata` bodies. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x806D21A0` | 36 | WAVE122 — `halt_baddata` bodies. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x806D38D0` | 36 | WAVE122 — `halt_baddata` bodies. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x806D3AC0` | 120 | WAVE122 — `halt_baddata` bodies. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x806D4450` | 48 | WAVE122 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806D4480` | 48 | WAVE122 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806D5230` | 48 | WAVE122 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806D5260` | 48 | WAVE122 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806D7510` | 48 | WAVE122 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806D7540` | 48 | WAVE122 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806D8260` | 48 | WAVE122 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806D8290` | 48 | WAVE122 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806D9200` | 48 | WAVE122 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806D9230` | 48 | WAVE122 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806DB050` | 48 | WAVE122 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806DB080` | 48 | WAVE122 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806DBE70` | 48 | WAVE122 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806DBEA0` | 48 | WAVE122 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806DCDE0` | 48 | WAVE122 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806DCE10` | 48 | WAVE122 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806DEC20` | 48 | WAVE122 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x806DEC50` | 48 | WAVE122 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80700F90` | 48 | WAVE121 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80700FC0` | 48 | WAVE121 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x807017C0` | 48 | WAVE121 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x807017F0` | 48 | WAVE121 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80704E00` | 48 | WAVE121 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80704E30` | 48 | WAVE121 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x807054E0` | 48 | WAVE121 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80705510` | 48 | WAVE121 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80706CF0` | 48 | WAVE121 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80706D20` | 48 | WAVE121 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80707CE0` | 48 | WAVE121 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80707D10` | 48 | WAVE121 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x807083F0` | 48 | WAVE121 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80708420` | 48 | WAVE121 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80711E30` | 48 | WAVE110 — `0x802DCBFC` indirect-jump stubs. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80711E60` | 48 | WAVE110 — `0x802DCBFC` indirect-jump stubs. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x807157B0` | 48 | WAVE110 — `0x802DCBFC` indirect-jump stubs. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x807157E0` | 48 | WAVE110 — `0x802DCBFC` indirect-jump stubs. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80717A80` | 48 | WAVE110 — `0x802DCBFC` indirect-jump stubs. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80717AB0` | 48 | WAVE110 — `0x802DCBFC` indirect-jump stubs. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80720570` | 48 | WAVE125 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x807205A0` | 48 | WAVE125 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80720980` | 36 | WAVE125 — `halt_baddata` bodies. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x80720D90` | 48 | WAVE125 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80720DC0` | 48 | WAVE125 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x807211D0` | 24 | WAVE125 — `halt_baddata` bodies. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x807228A0` | 48 | WAVE125 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x807228D0` | 48 | WAVE125 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80723BD0` | 48 | WAVE125 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80723C00` | 48 | WAVE125 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80727180` | 16 | WAVE125 — `halt_baddata` bodies. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x8072C1F0` | 4 | WAVE125 — Bare `blr` and constant getters. | **D** | byte-identical to an already-translated getter (duplicate) |
| `0x8072C950` | 48 | WAVE125 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x8072C980` | 48 | WAVE125 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x8072DBA0` | 48 | WAVE125 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x8072DBD0` | 48 | WAVE125 — `0x802DCBFC` forwarders. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x807477D0` | 48 | WAVE117 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80747800` | 48 | WAVE117 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80747F40` | 48 | WAVE117 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80747F70` | 48 | WAVE117 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80749120` | 48 | WAVE117 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80749150` | 48 | WAVE117 — `0x802DCBFC` thunks. | **D** | 48-byte forwarder: `mr r3,r4` + hidden `r12=this+off`, tail call |
| `0x80761670` | 12 | WAVE114 — `0x80761670` (4 bytes) | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x807812D0` | 4 | WAVE115 — `halt_baddata` stubs. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x80781C80` | 16 | WAVE115 — `halt_baddata` stubs. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x80781C90` | 32 | WAVE115 — `halt_baddata` stubs. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x807882E0` | 28 | WAVE115 — `halt_baddata` stubs. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x8078A270` | 16 | WAVE115 — `halt_baddata` stubs. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x8078A3A0` | 48 | WAVE115 — `halt_baddata` stubs. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x8078A740` | 48 | WAVE115 — `halt_baddata` stubs. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x8078B5B0` | 48 | WAVE115 — `halt_baddata` stubs. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x8078B7D0` | 48 | WAVE115 — `halt_baddata` stubs. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x8078B9F0` | 48 | WAVE115 — `halt_baddata` stubs. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x8078BC10` | 48 | WAVE115 — `halt_baddata` stubs. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x8078BE30` | 48 | WAVE115 — `halt_baddata` stubs. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |
| `0x8078C060` | 48 | WAVE115 — `halt_baddata` stubs. | **D** | export truncates at `halt_baddata`; raw word at the cut is undecodable |

## Ranked next claims

Ranked by claimable bytes per unit of remaining work. The first block is pure
**B**: the raw stream already contains every operand, every call target and
every store. The second block is **C**: worth taking as soon as the single
missing input is resolved.

### B - claim now

| # | address | bytes | why this one |
|---:|---|---:|---|
| 1 | `0x806F3350` | 2092 | Largest skip in the corpus; no `psq` at all, 61 direct calls, only a `_savegpr` prologue. |
| 2 | `0x8069B750` | 1424 | 37 calls, no data-side `psq` (only the f31 save); wave 119 rejected it for `unaff_GQR0`. |
| 3 | `0x80642540` | 1236 | Seven-pane rebuild; `qr0` saves only; everything else is integer math and calls. |
| 4 | `0x8070D1F0` | 1200 | Wave 121 asked for an instruction check: `lis 0x4330` at `0x8070D2C8` and `0x8070D314` proves the unsigned-to-float ladder. |
| 5 | `0x806822B0` | 1108 | No data-side `psq`; 14 direct calls; wave 123 deferred it only for size. |
| 6 | `0x8069FAB0` | 996 | 31 calls, virtual tail call visible; wave 119 called it "reconstructable in principle". |
| 7 | `0x806A0290` | 948 | Per-frame body behind `_savegpr`; r3 survives, 19 direct calls. |
| 8 | `0x806FD820` | 912 | Only f31 save ops; 10 direct calls; wave 113 never opened it. |
| 9 | `0x806B5E00` | 892 | Wave 118 rejected for `unaff_GQR0`; the only `qr0` op is the f31 spill. |
| 10 | `0x8069F640` | 892 | Wave 119 explicitly said "reconstructable in principle"; 30 calls. |
| 11 | `0x806F7BB0` | 1004 | No `psq`, 12 calls, plain prologue. |
| 12 | `0x80686550` | 1884 | 20 calls; the 8 `qr0` ops are all `r1` saves. |
| 13 | `0x80673F50` | 656 | Wave 12 rejected it for `unaff_GQR0`; raw stream uses `qr0` only, offsets `0xA8/0x98/0x88(r1)`. |
| 14 | `0x8070D790` | 584 | Contact body: no `psq`, no `lfd`/`stfd`; the float work is plain `lfs`/`stfs` plus `fadds`/`fmuls`, so the `(double)` casts in the export are ordinary widening. |
| 15 | `0x80656D20` | 392 | Event body; `0x801C14C0` and both virtual calls are explicit in the stream. |
| 16 | `0x806B3AC0` | 136 | The "undefined8" args are plain r5/r6/r7 in the prologue; `0x800BC950` call is direct. |
| 17 | `0x8066B0F0` | 128 | Three virtual calls with explicit `lwz r12,0x60(r3)` receivers. |
| 18 | `0x8069ACA0` | 40 | Ghidra's jumptable warning is spurious; it is a store plus a virtual tail call. |

### C - cheap once one input is known

| # | address | bytes | blocker |
|---:|---|---:|---|
| 1 | `0x806B7CD0` | 1704 | Two `qr3` data ops; needs the GQR3 type word from `main.dol`. |
| 2 | `0x806DD730` | 736 | Two `qr3` ops; same constant. |
| 3 | `0x8072E520` | 644 | Two `qr3` ops plus `qr0`; same constant. |
| 4 | `0x8072BC40` | 604 | Two `qr3` ops; same constant. |
| 5 | `0x806B9A30` | 476 | Two `qr3` ops; same constant. |
| 6 | `0x806B3770` | 308 | Two `qr3` ops; same constant. |
| 7 | `0x807BD110` | 2908 | Not a body: the `dFileSelect_c` 44-entry state table. Needs the registration-table generator. |

## Verification

- `ledger/functions.csv` contains 2,404 rows for `src/game/bases.rs`; none of
  the 319 addresses in the table appears there.
- `src/game/bases.rs` contains no `fn_<address>` for any of the 319.
- Every byte count in the table matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv`.
- 125 of the 173 D rows are the same 48-byte `0x802DCBFC` forwarder shape; 28
  are `halt_baddata`; 17 are byte-identical to an already-translated getter; the
  remaining 3 are 12-byte constant returns.
- No file in the repository was written except this report.
