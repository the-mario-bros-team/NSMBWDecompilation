# Coverage - Wave 81 (runtime::os, remaining `0x80250000` bucket)

## How to read this file

This page records wave 81 of the `main.dol` reconstruction for the
`runtime::os` lane: a pass over the `0x80250000..0x8025ffff` bucket, which held
163 inventory entries of which 45 were already merged, leaving 118 untranslated
when the lane started.

- **Status** - `translated` means every branch, store and call in the Ghidra
  export has a counterpart in `src/runtime/os.rs`. All 29 rows here are
  `translated`; none is `partial` and none is `stub`.
- **Confidence** - strength of the evidence, not of the Rust. `high` means the
  flow, the offsets and the callee argument shapes are explicit and
  self-contained. `medium` means the flow is explicit but a callee's role is
  inferred from a sibling body, or Ghidra printed a call with its argument
  registers swept away and they had to be restored from that callee's own
  prototype.
- **Evidence** - the C file under `work/ghidra/main-gb/export/`; it is the
  source of truth for every row. Byte sizes come from
  `work/ghidra/main-gb/export/functions.tsv`, re-read for this pass.
- Addresses are virtual addresses of the unrelocated `main.dol` (image base
  `0x80004000`), not file offsets.

The machine-readable companion is `ledger/agents/wave81-os.csv` (same rows).
This wave edits only `src/runtime/os.rs` plus those two files;
`ledger/functions.csv` is untouched and is merged by the coordinator.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/os.rs` | `runtime::os` | 29 | 2,392 | 17 | 12 |

Status split: 29 `translated`, 0 `partial`, 0 `stub`. No `low`.

## What this pass covers

- **Group 1 - transform node, `0x80257100..0x80257A00` (14 bodies).** One
  constructor (`0x80257100`), one destructor (`0x802571F0`), and the twelve
  accessors of the class: set / copy / read one of
  the **three 3x4 float matrices** at `+0x0C`, `+0x3C`, `+0x6C` (stride `0x30`),
  the two clamped byte limits at `+0xD0`/`+0xD1`, the `+0xD8` flag byte, the
  `+0xDA` halfword, the two 6-word halves of the twelve words at `+0x9C`, and
  the three-float classification at `+0xDC..+0xE4`.
- **Group 2 - child-array holder, `0x802585C0..0x802588D0` (5 bodies).** The
  array pointer is `+0xDC`, the capacity `+0xE0`, the live count `+0xE4`.
  Includes "remove child by index" (with the shift-down loop and the
  `0x10003` "child is leaving" event), "remove child by key" (linear scan, then
  a virtual call with the index), the constructor and destructor of the derived
  class, and the virtual size probe.
- **Group 3 - two dispatch wrappers, `0x802590A0`/`0x802590F0` (2 bodies).** One
  refresh pre-pass plus event `1`, and the bare event `2`, both on vtable slot
  `+0x0C`.
- **Group 4 - range-sort wrappers, `0x80259680`/`0x80259F60`/`0x8025A0E0`/
  `0x8025A780` (4 bodies).** Three of them pass `[begin, begin + count*4)` twice
  to `0x80259760` with a comparator address (two fixed variants and one that
  takes the comparators from the caller); the fourth reports two `+0xCC` flag
  bits, inverted with `clz>>5`, on a dispatcher's vtable.
- **Group 5 - four leaves (4 bodies).** The two "store a word and return the
  pointer" helpers (`0x802537D0`, `0x8025F2B0`), the `0x7FF` call
  (`0x8025EFA0`) and the two-matrix multiply wrapper (`0x802570E0`).

## What the pass adds beyond the raw bodies

1. **The matrix slot layout is now closed.** `0x801C04D0` initialises one 3x4
   matrix, `0x801C0500` copies one, and `0x801C0540` multiplies two. That fixes
   the meaning of the `+0x0C`/`+0x3C`/`+0x6C` triplet and of the "slot" argument
   in `0x802574E0`/`0x80257570`/`0x802575D0`, and it is why the twelve words at
   `+0x9C` are read as a matrix instead of as twelve unrelated fields.
2. **The `+0xCC` bit 31 is an identity flag.** `0x80257100` seeds `+0xCC` with
   `0x80000000`; resetting matrix slot `0` sets the bit, copying into slot `0`
   clears it, and the child-owning constructor adds `0x60000000` on top. A port
   that keeps only the matrices and drops the flag will re-transform an identity
   matrix on every frame.
3. **The child array is drained in reverse.** `0x80258780` calls its own vtable
   slot `+0x38` with `count - 1` until the count reaches zero, so the "remove by
   index" body (`0x802585c0`, or whatever the slot points at) must tolerate
   being called with the last index only. The same slot is what
   `0x80258680` calls after a successful key scan.
4. **Two `clz>>5` idioms are load-bearing.** `0x80257830` returns
   `clz(result-1)>>5`, i.e. `result == 1`; `0x8025A780` returns
   `clz(word & mask)>>5`, i.e. "the mask is **clear**". Both are written as
   `leading_zeros()` so the inversion stays visible.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802537D0` | `fn_802537d0` | 8 | medium |
| `0x802570E0` | `fn_802570e0` | 20 | medium |
| `0x80257100` | `fn_80257100` | 236 | high |
| `0x802571F0` | `fn_802571f0` | 128 | medium |
| `0x802574E0` | `fn_802574e0` | 140 | high |
| `0x80257570` | `fn_80257570` | 92 | high |
| `0x802575D0` | `fn_802575d0` | 76 | high |
| `0x80257640` | `fn_80257640` | 36 | high |
| `0x80257670` | `fn_80257670` | 36 | high |
| `0x802576A0` | `fn_802576a0` | 64 | high |
| `0x802576E0` | `fn_802576e0` | 64 | high |
| `0x80257720` | `fn_80257720` | 20 | high |
| `0x80257740` | `fn_80257740` | 128 | medium |
| `0x802577D0` | `fn_802577d0` | 96 | high |
| `0x80257830` | `fn_80257830` | 56 | high |
| `0x80257A00` | `fn_80257a00` | 64 | high |
| `0x802585C0` | `fn_802585c0` | 188 | medium |
| `0x80258680` | `fn_80258680` | 132 | medium |
| `0x80258710` | `fn_80258710` | 112 | medium |
| `0x80258780` | `fn_80258780` | 184 | high |
| `0x802588D0` | `fn_802588d0` | 48 | medium |
| `0x802590A0` | `fn_802590a0` | 76 | medium |
| `0x802590F0` | `fn_802590f0` | 28 | high |
| `0x80259680` | `fn_80259680` | 96 | medium |
| `0x80259F60` | `fn_80259f60` | 96 | high |
| `0x8025A0E0` | `fn_8025a0e0` | 100 | medium |
| `0x8025A780` | `fn_8025a780` | 52 | high |
| `0x8025EFA0` | `fn_8025efa0` | 8 | high |
| `0x8025F2B0` | `fn_8025f2b0` | 8 | medium |

## Unknowns

1. **Field roles.** `+0xD0`/`+0xD1` (both seeded with `0x80`), `+0xD8`, `+0xDA`,
   `+0xD4` (an owned object) and `+0xD9` (a "owns it" byte) are literal but
   unnamed. Nothing in this window says what the class is called or what the two
   clamped bytes mean.
2. **Event ids.** `9`, `1`, `2`, `0x10003` and the vtable slots `+0x0C`, `+0x14`,
   `+0x20`, `+0x28`, `+0x38` are named by value only.
3. **Shared constants.** `sda2(-0x4908)` (the twelve-word fill) and
   `sda2(-0x4904)` (the uniform-scale comparison in `0x80257A00`) are read but
   not dumped; they are adjacent words and most likely `0.0f`/`1.0f`.
4. **The two comparators of `0x80259680`.** `0x802596E0` and `0x80259720` are
   code addresses inside the `0x802596E0..0x80259760` gap that Ghidra did not
   type as functions, so they are passed by address.
5. **Callee roles.** `0x801C04D0`, `0x801C0500` and `0x801C0540` are read from
   their own exports (identity / copy / multiply); `0x8024D5E0`,
   `0x8024_9A70`, `0x80258F10`, `0x80259760` and `0x8025_7100` are dispatched by
   address and marked inferred. These are the only reason the affected rows are
   `medium`.

## Deliberate skips

- **`0x80253ED0`, `0x802540B0`, `0x80254110`, `0x802541F0`, `0x802542C0`,
  `0x80254340`, `0x802543D0`** (the `0x80254xxx` matrix emitters). Read and left
  out: each forwards six to nine `f64` values plus a destination to
  `0x801C0F20`/`0x801C0FD0`/`0x801C10D0`/`0x801C11C0`/`0x801C1260`/`0x801C1350`/
  `0x801C9660`, and `0x80253ED0` reuses the same word as both a pointer and an
  offset (`param_2`, `param_2 + 4`, `param_2 + 8`). The mixed float/integer ABI
  could only be written down by guessing.
- **`0x80258BD0`, `0x80258C30`, `0x80258C80`** (the three getters that read
  through the store helpers). Ghidra prints the same field twice with different
  scaling: `0x80258C30` as `param_1 + 0xf4 * 0x10c + 0xf8` on an `int *` and
  `0x80259110` as `param_1 + 0x3d * 0x43 + 0x3e` on an `int *` too
  (`0x3d*4 = 0xf4`, `0x43*4 = 0x10c`, `0x3e*4 = 0xf8`). One of the two must be
  byte arithmetic, and the exports cannot tell which, so the stride is not
  claimed. The two helpers they call (`0x802537D0`, `0x8025F2B0`) *are*
  translated, because their own bodies are unambiguous.
- **`0x80250CB0`, `0x80250D40`.** Read and left out: `0x802556B0` and
  `0x80254910` take four words, but the callers print with no arguments at all
  (`FUN_802556b0()`), and `0x80250D40` has only two parameters of its own, so the
  four argument registers cannot be restored honestly.
- **`0x80250DA0`..`0x80250DF0`** (five 12-byte bodies returning `0x803E4500` or
  `0x803EA500`). They are trivially translatable but carry no behaviour; skipped
  to keep the lane on bodies that exercise control flow or memory.
- **`0x802589E0`** (312 bytes). Read and left out: it splits a 64-bit
  `0x802DCD60` return into two halves, stores through the first half, and then
  calls `0x802593E0` with nine arguments.
- **Large workers not attempted:** `0x80250230` (1,244), `0x80250710` (1,436),
  `0x80250E70` (1,476), `0x80251440`/`0x80251930` (1,256/1,308),
  `0x802529F0`/`0x80252FB0` (1,468 each), `0x802544F0` (1,052),
  `0x80257A40` (732), `0x80257FC0` (1,120), `0x80258CE0` (552),
  `0x80259760`/`0x80259AE0` (896 each), `0x8025A7C0` (844),
  `0x8025B500` (1,552), `0x8025CA20` (2,952), `0x8025FE30` (1,940). Their
  callees are no better named after this pass.
- The 45 addresses already merged in this bucket are not repeated.

Remaining untranslated in the bucket after this wave: 118 - 29 = 89 entries.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (re-read for this pass, not copied from prose).
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every CSV row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/os.rs`; no address appears twice.
- No address in the CSV was already present in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave81` ->
  `wave81-os.csv: 29 rows`, `total rows: 29, unique addresses: 29`,
  `translated/high=17 translated/medium=12`, `OK`.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/runtime/os.rs`. At the time of writing the crate as a whole still fails
  on three `E0384` errors in `src/game/bases.rs`, which belongs to another wave
  81 lane and is outside this lane's edit scope.
