# Coverage - Wave 113 (`runtime::input`, the HCI receive dispatch and the trace forwarder)

## How to read this file

1. **Summary** gives the per-file totals and the closing inventory of the
   `[0x801F0000, 0x80200000)` bucket.
2. **What this pass covers** describes the two bodies: the HCI receive state
   machine and the variadic trace forwarder the seven trace guards branch into.
3. The **function table** lists both in address order with the byte count
   re-read from `work/ghidra/main-gb/export/functions.tsv` and the confidence
   carried by each doc block.
4. **Corrections and caveats** records the evidence that retired the wave-102
   skip and every piece of the forwarder's frame that is modelled rather than
   reproduced; **Deliberate skips** states what is left.

Status vocabulary (`docs/READING_GUIDE.md`): `translated` means the whole
control flow and every call in the export are accounted for; `partial` means
part of the body is provably not expressible. The receive machine is
`translated`; the forwarder is `partial` because its `va_list` carries the
caller's stack pointer. Both are `medium`: the slot roles, the block fields and
the forwarder's argument roles are recovered from layout and disassembly, not
from names.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/input.rs` | `runtime::input` | 2 | 1,100 | 0 | 2 |

Bucket inventory (`[0x801F0000, 0x80200000)`):

| Item | Count |
|---|---:|
| Functions in the bucket | 268 |
| Untranslated before this pass | 2 |
| Selected by this pass | **2** |
| Landed `translated` | 1 |
| Landed `partial` | 1 |
| Deliberately skipped | 0 |
| Entries without any row after this pass | **0** |

## What this pass covers

- **HCI receive dispatch (`0x801F8820`, 920 B).** Wave 102 skipped it because
  its "state-block pointer and channel index are read only through the
  `0x802DCD40` prologue stub" and the sole caller `0x801F8E80` passes "a single
  length word". Disassembling both settles the first half: `0x802DCD40` is
  `__savegprlr_14`, a register-save helper that leaves `r3`/`r4` untouched, so
  Ghidra's `undefined8` "return" is exactly the pair `(r3, r4)`. `0x801F8E80`
  loads `r4 = 0x803CDDA8` itself (`lis 0x803d; addi -0x2258`) and computes the
  first argument in `r3`, so `r3` is the slot index and `r4` the receive block.
  With that, every field of the 0x20-byte block is decidable: three buffer slots
  at `+n*4`, three counts at `+0x0C+n*2`, the three
  `+0x14+n`/`+0x17+n`/`+0x1A+n` type/saved-length/state bytes, the `+0x12` bias
  halfword the caller subtracts (never written here), the gate at `+0x1E` and
  the report queue id at `+0x1F`. The body walks the five states, stamps the
  packet id from the `r2-0x4CBA` table (types `1`..`4` →
  `0x0013`/`0x0011`/`0x0012`/`0x0010`) and the header size from `r2-0x4CC0`
  (`3`/`4`/`3`/`2`), and posts or frees the finished buffer.
- **Variadic trace forwarder (`0x801F9C50`, 180 B).** The seven trace guards
  `0x801F9D10`–`0x801F9DE0` are 20-byte `b` tails into this address. The body is
  the CodeWarrior varargs prologue and nothing else: it saves `r3`..`r10` at
  `r1+8`, conditionally saves `f1`..`f8` at `r1+0x28` behind a `CR1` bit, builds
  the three-word `va_list` `{ (GPR area) | 0x02000000, r1+0x88, GPR area }`, then
  calls `0x802E1664` (vsprintf-shaped), `0x802E17DC` (append, format `"%s\n"`
  at `r13-0x5D78`) and `0x8015F730` (flush), all on the buffer `0x803CFE40`.
  The Rust body builds the same frame from an explicit eight-word argument slice
  and issues the same three calls.

## Functions

| Address | Function | Role | Bytes | Status | Confidence | Evidence |
|---|---|---|---|---|---|---|
| `0x801F8820` | `fn_801f8820` | HCI receive state machine over the 3-slot block | 920 | translated | medium | `work/ghidra/main-gb/export/801f8820_FUN_801f8820.c` |
| `0x801F9C50` | `fn_801f9c50` | varargs trace forwarder (`0x802E1664`/`0x802E17DC`/`0x8015F730`) | 180 | partial | medium | `work/ghidra/main-gb/export/801f9c50_FUN_801f9c50.c` |

Totals: **1 translated**, 1 partial, 0 stubs; 0 high / 2 medium / 0 low;
1,100 bytes. Address range `0x801F8820`–`0x801F9C50`. The `0x801F0000` bucket
now has a row for all 268 of its entries.

## Corrections and caveats

1. **The wave-102 skip reason was a misreading, recorded here deliberately.**
   `0x802DCD40` is `__savegprlr_14` (`work/ghidra/main-gb/export/802dcd40_FUN_802dcd40.c`
   stores `r14`..`r31` through `r11`), not an accessor. Ghidra turns its
   untouched `r3:r4` into an `undefined8` return; the high word is `r3` and the
   low word `r4`, which the first instruction pair confirms: `lbz r0,0x1e(r4)`
   is the gate Ghidra prints as `*(char *)(iVar9 + 0x1e)`.
2. **The caller is indirect but still fixes the arguments.** The only branch to
   `0x801F8820` is `0x801F8EA4`, inside `0x801F8E80`; that trampoline has no
   direct branch anywhere in the image, but it *is* slot `+0x10` of the BTA-HH
   method table at `0x802F6D30` (`r13-0x4B00`), whose first four entries
   (`0x801F8D70`, `0x801F8DA0`, `0x801F8E10`, `0x801F8E50`) Ghidra never defined
   as functions. So the value the transport passes in `r3` is not provable from
   a call site; what *is* provable is how the callee consumes it (a three-entry
   slot index) and the constant it loads into `r4`. The parameter is documented
   `// inferred:` for that reason.
3. **The byte-read callee is a stub.** `0x801F9830` really is `li r3,0; blr`,
   so the receive loop never iterates in the shipped image. The body keeps the
   call with all three arguments, calls it by address, and says so at the
   constant; the loop is transcribed, not "simplified away".
4. **The allocation-failure path faults in the original.** After storing state
   `2` the export falls into the shared byte-append block with a zero slot
   pointer. The Rust keeps that order and the doc block calls out the hazard;
   no extra guard was added, because a guard would hide an observable path.
5. **The two `r2` tables have no export.** `r2-0x4CC0` and `r2-0x4CBA` were read
   from the image (`SDA2_BASE - 0x4CC0` = `0x8042E3C0`, `SDA2_BASE - 0x4CBA` =
   `0x8042E3C6`) and their four entries are stated in the constants; their roles
   (header size, packet id) are `// inferred:`.
6. **`0x801F9C50` is `partial` on purpose.** Two frame pieces are not
   expressible: the `CR1`-gated `f1`..`f8` save area (no export reads it back)
   and the `va_list` overflow word, which is the *caller's* stack pointer
   `r1+0x88`. The Rust passes `0` there with an `// unknown:` comment. The
   `0x02000000` tag, the eight GPR save slots, `"%s\n"` and all three call
   targets are literal.
7. **`or r3,r5,r3` before the append call.** The export ORs `r5` into the buffer
   address and then never reads `r5` again; `0x802E17DC` only lets `r3` (the
   buffer) and `r4` (the format) reach the formatter call. The Rust passes `0`
   as the third word with an `// inferred:` note.
8. **`0x803CFE40` is shared.** The same address is the IOS transmit context of
   `fn_801f9840` and the string buffer this forwarder formats into; the constant
   doc records both uses rather than renaming either.

## Deliberate skips

None at function level: both remaining entries of the bucket have a body.
Within `fn_801f9c50` the FPR save area and the caller's stack word are omitted,
which is what the `partial` status records.

## Verification

- Every address and byte size in the table was re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass (920 + 180 = 1,100
  bytes).
- Every evidence file exists at the cited `work/ghidra/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/input.rs`.
- Neither address was in `ledger/functions.csv`; the two-entry untranslated
  list for the bucket was derived from it first (268 bucket entries, 266 rows).
- `node tools/check-lane-csv.mjs wave113` prints `OK` and reports
  `wave113-input.csv: 2 rows`.
- `cargo check --message-format=short` reports no error in
  `src/runtime/input.rs` and no warning at either wave-113 body (the two
  `direct cast of function item into an integer` warnings in the file belong to
  earlier passes).
- The module header gained a twentieth-pass bullet and the promotion note now
  records the receive-dispatch pairing and the forwarder.

## Ledger

Machine-readable rows: `ledger/agents/wave113-input.csv` (2 rows, module
`runtime::input`, address order as the table above, header
`module,address,name,status,confidence,evidence,rust_file`).
