# Coverage - Wave 83 (`runtime::os`, remaining `0x80240000` bucket)

## How to read this file

This page records wave 83 of the `main.dol` reconstruction for the
`runtime::os` lane: a pass over the `0x80240000..0x8024ffff` bucket, which holds
137 inventory entries of which 24 were already merged, leaving 113 untranslated
when the lane started.

- **Status** - `translated` means every branch, store and call in the Ghidra
  export has a counterpart in `src/runtime/os.rs`. All 27 rows here are
  `translated`; none is `partial` and none is `stub`.
- **Confidence** - strength of the evidence, not of the Rust. `high` means the
  flow, the offsets and the callee argument shapes are explicit in the export
  itself. `medium` means the flow is explicit but a callee's role is read from
  the callee's own export, or Ghidra printed a virtual call with its argument
  registers swept away and the receiver had to be restored from the class
  pattern.
- **Evidence** - the C file under `work/ghidra/main-gb/export/`; it is the
  source of truth for every row. Byte sizes come from
  `work/ghidra/main-gb/export/functions.tsv`, re-read for this pass.
- Addresses are virtual addresses of the unrelocated `main.dol` (image base
  `0x80004000`), not file offsets.

The machine-readable companion is `ledger/agents/wave83-os.csv` (same rows).
This wave edits only `src/runtime/os.rs` plus those two files;
`ledger/functions.csv` is untouched and is merged by the coordinator.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/os.rs` | `runtime::os` | 27 | 2,148 | 14 | 13 |

Status split: 27 `translated`, 0 `partial`, 0 `stub`. No `low`.

## What this pass covers

- **Group 1 - name-keyed sub-table accessors, `0x802422A0..0x80242530`
  (6 bodies).** Six bodies are two names (`0x8034E160`, `0x8034E180`) crossed
  with four operations: "does the sub-table exist", "chained lookup with the
  caller's key" and "return sub-entry `index`". The descriptor's `+0x00` word
  points at a table header, the header's `+0x10` word is a displacement to the
  first sub-table, and every hit is a displacement that has to be added to the
  sub-table base. The search itself is `0x80239CC0`.
- **Group 2 - three cell-table families, `0x80244800..0x802477D0`
  (13 bodies).** Five bodies per family (the third family's child walk
  `0x80247A50` was already merged): virtual size probe (slot `+0x14`, plus
  four), reserved-bit probe (`clz(cell & 0xC000) >> 5` over the `+0x14` halfword
  array), full clear (every cell set to `0x8000`, then flag bit `0x04` cleared
  through `0x80249C60`), destructor (install the family vtable, run slot
  `+0x44`, free through `0x80249A70`) and child walk (slot `+0x40` per child).
  The three families differ only in the vtable constant
  (`0x8034E3D8` / `0x8034E4F8` / `0x8034E638`); the vtable-independent bodies
  delegate to the family-A implementation, following the existing
  `fn_80244e60` -> `fn_80243690` precedent in this file.
- **Group 3 - list record flags, `0x8024B020`/`0x8024B090` (2 bodies).** One
  `0x01`/`0x02`/`0x04` bit fan-out to three float triples at `+0x04`, `+0x34`
  and `+0x64` under the ids `0`, `3`, `6`; and the "return the pointer, flush
  first" wrapper.
- **Group 4 - global installs and data leaves, `0x8024D0B0..0x8024D5D0`
  (6 bodies).** Three global stores plus three guarded `0x801C4780(id, ptr,
  size)` registrations; the `sda(-0x5C6C)` reader; the one-argument and
  six-argument forwarders onto the `0x803DFF88` descriptor; the
  `0x803E29B6`-indexed `0x803E29E8` record getter; and the `0x803E43E8` data
  address.

## What the pass adds beyond the raw bodies

1. **The cell-table class is now closed for three families.** Between this wave
   and the already-merged `0x80244E60`/`0x80246460`/`0x80247AB0` child walks,
   every family has its probe, its clear and its destructor translated, so the
   `+0x10`/`+0x14`/`+0x18`/`+0x1C` layout can be used from the game code
   without reading a single export.
2. **`0x8034E41C` is a vtable slot, not a variable.** The three destructors
   call `(*(code *)PTR_FUN_8034e41c)` with no arguments, which reads like a
   global function pointer. `0x8034E41C` is exactly `0x8034E3D8 + 0x44`, and
   the same relation holds for the other two families
   (`0x8034E53C - 0x8034E4F8` and `0x8034E67C - 0x8034E638`, both `0x44`), so
   Ghidra devirtualised the just-stored vtable. The Rust reads the slot back so
   the destructor stays a destructor.
3. **The name-table chain is two levels, not one.** `0x80242360`/`0x802424A0`
   pass the *result* of the first search back as the base of the second, and
   `0x802423F0`/`0x80242530` index into the hits at `0x24 + 0x10*n`, which is
   why the resolved table is modelled as a header plus a `0x10`-byte entry
   array rather than as a flat word list.
4. **`(-v | v) >> 31` is load-bearing three times.** The two "exists" bodies
   and the `index` bodies use it as a non-zero test; it is factored into
   `wave83_is_non_zero` so the fold stays visible instead of being hidden
   behind `!= 0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802422A0` | `fn_802422a0` | 96 | medium |
| `0x80242300` | `fn_80242300` | 96 | medium |
| `0x80242360` | `fn_80242360` | 136 | medium |
| `0x802423F0` | `fn_802423f0` | 172 | medium |
| `0x802424A0` | `fn_802424a0` | 136 | medium |
| `0x80242530` | `fn_80242530` | 172 | medium |
| `0x80244800` | `fn_80244800` | 48 | medium |
| `0x80244AB0` | `fn_80244ab0` | 28 | high |
| `0x80244AD0` | `fn_80244ad0` | 60 | high |
| `0x80244B80` | `fn_80244b80` | 100 | medium |
| `0x80244E00` | `fn_80244e00` | 96 | high |
| `0x80245E00` | `fn_80245e00` | 48 | medium |
| `0x802460B0` | `fn_802460b0` | 28 | high |
| `0x802460D0` | `fn_802460d0` | 60 | high |
| `0x80246180` | `fn_80246180` | 100 | medium |
| `0x80246400` | `fn_80246400` | 96 | high |
| `0x80247450` | `fn_80247450` | 48 | medium |
| `0x80247700` | `fn_80247700` | 60 | high |
| `0x802477D0` | `fn_802477d0` | 100 | medium |
| `0x8024B020` | `fn_8024b020` | 112 | high |
| `0x8024B090` | `fn_8024b090` | 64 | medium |
| `0x8024D0B0` | `fn_8024d0b0` | 136 | high |
| `0x8024D210` | `fn_8024d210` | 8 | high |
| `0x8024D380` | `fn_8024d380` | 16 | high |
| `0x8024D3C0` | `fn_8024d3c0` | 92 | high |
| `0x8024D4D0` | `fn_8024d4d0` | 28 | high |
| `0x8024D5D0` | `fn_8024d5d0` | 12 | high |

## Unknowns

1. **Name-table keys.** `0x8034E160` and `0x8034E180` are read as key pointers;
   the bytes at those addresses are the key itself (a name string) and are not
   part of the export, so the two keys are named by address only.
2. **`0x80239CC0`'s exact contract.** Its own export shows "if base word and
   key word are non-zero, search, return `base + hit[0x0C]`", which is enough
   to call it, but the table format above `+0x0C` is not recovered.
3. **Cell semantics.** `0x8000` as the "free" cell value and `0xC000` as the
   reservation mask are literal; whether `0x4000` alone is also reserved is
   `// unknown:` because the probe only reports the pair.
4. **Callee roles.** `0x801C4780`, `0x80249DD0`, `0x8024B0E0`, `0x8024B1E0`,
   `0x802B9280` and `0x8024D5E0` are dispatched by address and unnamed. The
   first four have explicit argument shapes at these call sites; `0x802B9280`
   is printed with no arguments at all.
5. **`0x803E29B6` bounds.** `0x8024D4D0` multiplies the halfword by `0x30`
   without clamping; whether every caller keeps it below the table length is
   not visible in this window.

## Deliberate skips

- **`0x80242840`, `0x80242930`, `0x80242A40`** (228/264/260 bytes). Read and
  left out: all three are GQR-parameterised vector/matrix kernels. Their bodies
  read `unaff_GQR0`, which is not a function argument in the export, and pick
  between byte, halfword and float loads from its quantisation fields. Writing
  them would mean inventing the GQR value.
- **`0x80242C30`, `0x80242D80`.** Read and left out: `0x80242C30` mixes
  `unaff_r2`/`unaff_r13` small-data reads with a relative-pointer construction
  and `0x80242D80` calls a function pointer with a `double`/`undefined8`
  argument whose ABI cannot be written without guessing.
- **`0x80243000`, `0x802430C0`.** Read and left out: both split the 64-bit
  return of `0x802DCD70`/`0x802DCD74` into a pointer half and a value half and
  then bracket the loop with `0x802DCDBC`/`0x802DCDC0`, which the export prints
  with mismatched argument lists; the split is not expressible through
  `call0`..`call7`.
- **`0x8024B6B0`, `0x8024BEC0`, `0x8024BF70`, `0x8024D140`, `0x8024D2C0`.**
  Read and left out: each reads `unaff_r13`, `unaff_r2` or both, i.e. a
  small-data word Ghidra did not resolve to a concrete symbol in that body. The
  `sda()` helper can only be used when the displacement survives in the export.
- **`0x80249CB0`, `0x80249D30`, `0x80249D60`** (the GX FIFO writers). Read and
  left out: `0x80249CB0` is a single `0xCC008000` store but is 128 bytes long,
  and `0x80249D30`/`0x80249D60` build their word from `unaff_r2`, `in_r7`,
  `in_r8` and `in_r9`. Neither the size discrepancy nor the incoming registers
  can be resolved from the export, so no body was invented for them.
- **`0x80249B40`** (88 bytes). Read and left out: the decompiled body is a
  six-instruction guarded virtual call that cannot fill 88 bytes, so the export
  is incomplete and a body would be a guess.
- **`0x8024D420`, `0x8024D4F0`** (164/212 bytes). Read and left out: the first
  calls the matrix emitters this lane already skipped in wave 81, the second is
  a twenty-field struct copy whose source layout is not recoverable without the
  callee `0x802DCD58`.
- **`0x8024B610`** (152 bytes). Read and left out: the export calls
  `FUN_802dcdbc(puVar1)` here while the same callee is printed with no
  arguments in `0x8024D420`, so the argument list is not trustworthy.
- **Bodies not attempted:** `0x802405B0` (2,660), `0x80241690` (1,124),
  `0x80241AA0` (320), `0x80241C70` (364), `0x80241DE0` (208), `0x80241EB0`
  (1,000), `0x80243B30`/`0x80245300`/`0x80246900` (308 each),
  `0x80243D20`/`0x80245500`/`0x80246B00` (496 each), `0x80244360` (980),
  `0x80245B40` (500), `0x80247140` (156), `0x802471E0` (428),
  `0x802481B0` (372), `0x80248990` (496), `0x80249740` (808),
  `0x80249DD0` (668), `0x8024A6E0` (984), `0x8024ABD0` (1,088),
  `0x8024B1E0` (1,044), `0x8024B720` (616), `0x8024BAE0` (460),
  `0x8024BCB0` (528), `0x8024C0E0` (464), `0x8024C2B0` (1,556),
  `0x8024C960` (684), `0x8024CDC0` (752), `0x8024D5E0` (364),
  `0x8024DCE0` (1,172), `0x8024E180` (2,212), `0x8024EE30` (872),
  `0x8024FEF0` (828). Their callees are no better named after this pass.
- The 24 addresses already merged in this bucket (`0x802418B0`, `0x80241A00`,
  `0x80241BE0`, `0x802425E0`, `0x80242630`, `0x80242680`, `0x80242B50`,
  `0x80242BF0`, `0x80243210`, `0x802432E0`, `0x80243300`, `0x802433B0`,
  `0x80243630`, `0x80243690`, `0x80244E60`, `0x80246460`, `0x80247A50`,
  `0x80247AB0`, `0x80249AC0`, `0x80249C60`, `0x80249C90`, `0x8024A2D0`,
  `0x8024A530`, `0x8024D390`) are not repeated.

Remaining untranslated in the bucket after this wave: 113 - 27 = 86 entries.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (re-read for this pass, not copied from prose).
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every CSV row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/os.rs`; no address appears twice.
- No address in the CSV was already present in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave83` ->
  `wave83-os.csv: 27 rows`, `total rows: 27, unique addresses: 27`,
  `translated/high=14 translated/medium=13`, `OK`.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/runtime/os.rs`. At the time of writing the crate as a whole still fails
  on `E0308` errors in `src/game/enemies.rs`, which belongs to another wave 83
  lane and is outside this lane's edit scope.
