# Coverage - Wave 78 (resource::registry)

## How to read this file

Every row is one real `pub unsafe fn` body in `src/resource/registry.rs`. The
address and byte count come from
`work/ghidra/main-gb/export/functions.tsv`; the evidence path is the Ghidra C
export that was read before the body was written. `confidence` matches the
`# Confidence` line of the function's doc block, and `role` is the same
plain-English summary as the module-header table.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/registry.rs` | `resource::registry` | 25 | 4540 | 2 | 23 |

## What this pass covers

- **Sound-bank list transition** (`0x80161DA0`, 1 body): the `stage == 2`
  unlink of the two nodes at `+0x24`/`+0x34` with the `r13 - 0x6228` group
  check, and the `stage == 1` child-chain mark through `fn_80162510`.
  Wave 52 had listed this address as unresolvable; the current export shows a
  plain two-way transition, so it is translated rather than carried forward.
- **Three-row transform installers** (`0x80161E40`, `0x80162020`, `0x80162120`,
  `0x80162220`, 4 bodies): each copies nine words out of one of the four
  `0x80329350`/`0x80329374`/`0x80329398`/`0x803293BC` row blocks and hands the
  low/middle/high rows to `0x80161CC0`.
- **Callback walk** (`0x80163D40`, 1 body): the recursive list walk with the
  optional prune predicate, the leaf predicate and the `-4` payload bias on each
  `+0x04` child link.
- **Auxiliary slot allocation** (`0x80164990`, 1 body): the index guard against
  `r13 - 0x52A8`/`r13 - 0x52A4`, the `0x10`-byte block from `0x802B9240` and the
  `0x802C6550` constructor.
- **Element attach and cast layer** (`0x80165520`, `0x801655E0`, `0x801656C0`,
  `0x80167FA0`, `0x80168280`, `0x80168400`, `0x80168520`, 7 bodies): the two
  `0x802481B0` resolutions plus the size/heap helper, the `0x802F86B4` /
  `0x802F86C8` cast pair, the size rounding around `0x8016E630`, the kind-5
  filing path, and the `+0xE8`/`+0x3C` generic-element accessors.
- **Class teardown and per-entry release** (`0x80165EE0`, `0x80166110`,
  `0x80166AE0`, `0x80166D10`, `0x80167710`, `0x80167940`, 6 bodies): the three
  `0x38`-stride array teardowns (slot `+0x10` per entry, then `fn_80167f30`) and
  their three per-entry releases (owner slot `+0x40`, then the class release).
  These are the exact bodies that wave 64's destructors called through
  `W64_FN_80165EE0` / `W64_FN_80166AE0` / `W64_FN_80167710`.
- **List and command leaves** (`0x8016C4A0`, `0x8016C850`, `0x8016CEA0`,
  `0x8016EDD0`, `0x8016EEF0`, 5 bodies): the `0x80377C3C` intrusive-list
  teardown, the three-word layout submission, the `+0x74` descriptor
  re-submission, and the two message-id formatter/submit twins.

Two translator notes apply to the whole pass and are repeated in the section
banner:

- `FUN_802dcd5c/64/68/6c/70/74` are Metrowerks **register-save stubs** that
  return the incoming `r3:r4` pair. The dropped arguments are spelled out as
  parameters, following the wave-38 rule for `fn_8016e500`.
- The `p = *(int **)(this + 4); local = &type; (*p->vtbl[2])(p, &local)` idiom
  is the EGG `dynamic_cast<T>`; the stack word holds the type-info pointer and
  its **address** is the argument. It is factored into the private helper
  `w78_cast_to`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80161DA0` | `fn_80161da0` | 144 | medium |
| `0x80161E40` | `fn_80161e40` | 168 | medium |
| `0x80162020` | `fn_80162020` | 168 | medium |
| `0x80162120` | `fn_80162120` | 168 | medium |
| `0x80162220` | `fn_80162220` | 168 | medium |
| `0x80163D40` | `fn_80163d40` | 208 | medium |
| `0x80164990` | `fn_80164990` | 164 | medium |
| `0x80165520` | `fn_80165520` | 184 | medium |
| `0x801655E0` | `fn_801655e0` | 216 | medium |
| `0x801656C0` | `fn_801656c0` | 180 | medium |
| `0x80165EE0` | `fn_80165ee0` | 236 | medium |
| `0x80166110` | `fn_80166110` | 188 | medium |
| `0x80166AE0` | `fn_80166ae0` | 236 | medium |
| `0x80166D10` | `fn_80166d10` | 188 | medium |
| `0x80167710` | `fn_80167710` | 236 | medium |
| `0x80167940` | `fn_80167940` | 188 | medium |
| `0x80167FA0` | `fn_80167fa0` | 188 | medium |
| `0x80168280` | `fn_80168280` | 348 | medium |
| `0x80168400` | `fn_80168400` | 128 | medium |
| `0x80168520` | `fn_80168520` | 188 | medium |
| `0x8016C4A0` | `fn_8016c4a0` | 240 | medium |
| `0x8016C850` | `fn_8016c850` | 64 | high |
| `0x8016CEA0` | `fn_8016cea0` | 168 | high |
| `0x8016EDD0` | `fn_8016edd0` | 88 | medium |
| `0x8016EEF0` | `fn_8016eef0` | 88 | medium |

Address range: `0x80161DA0..0x8016EEF0`. All 25 rows are `translated`; none is
a stub and none is `partial`.

## Deliberate skips

The pass stayed inside the `0x80160000` bucket and skipped the following
neighbours, in every case because the body could not be reconstructed without
guessing:

- **4-byte thunks** `0x80160AD0`, `0x80161740`, `0x80162110`, `0x801644F0`,
  `0x8016BCB0`, `0x8016C820`, `0x8016E8E0` stay below the size floor, as in
  waves 45/52/64.
- **`r2` float and GQR leaves** `0x80160D10`, `0x80160E80`, `0x80160EA0`,
  `0x80161060`, `0x801610B0`, `0x80161180`, `0x801613A0`, `0x80161400`,
  `0x80161460`, `0x80161490`, `0x801614E0`, `0x8016AD30`, `0x8016D8A0`,
  `0x8016E230`, `0x8016E3A0`, `0x8016DEC0`, `0x8016F220`, `0x8016FA10`,
  `0x8016FD30`: the exports read unnamed `r2` constants and carry GQR/`ldexpf`
  artefacts, so the arithmetic would have to be invented.
- **Float-tail class builders** `0x801659C0`, `0x801665C0`, `0x801671B0`,
  `0x801658A0`, `0x801664A0`, `0x80167080`, `0x80167D60`, `0x80165280`,
  `0x80167420`: each ends in a call to `0x80168F10` (or a sibling) whose first
  three arguments travel in `f1..f3`. `crate::platform` can only set integer
  argument registers, so the tail call would be wrong. Their `0x80168F10`
  partners `0x80165AE0`, `0x801666E0`, `0x801672D0`, `0x80167E50` are skipped
  for the same reason.
- **Metrowerks critical-section pairs** `0x80163520`, `0x80163E60`,
  `0x80163FF0`, `0x801641B0`, `0x80164500`, `0x801645F0`, `0x80164770`,
  `0x80164A40`, `0x80164FD0`, `0x801650D0`, `0x80165C30`, `0x80166830`,
  `0x80165FD0`, `0x80166BD0`, `0x80167800`, `0x80167C40`: their `0x802DCD*`
  enter/leave stubs are called with different argument lists in each branch, and
  the release argument is not visible in the export.
- **Ambiguous dispatchers** `0x80161CC0`, `0x80161F60`, `0x80161AD0`,
  `0x80162A70`, `0x80168480`, `0x801693F0`, `0x80169F80`: `0x80161CC0` is
  called with three different argument lists in its own export; `0x80168480`
  feeds a two-word local whose role is unresolved; `0x801693F0` is a
  ten-argument forwarder and `crate::platform` stops at `call7`; `0x80169F80`
  forwards two words to `fn_80169c80`, which takes one.
- **`0x8016AB90`** still calls its base constructor with an empty argument list
  in the export, so the missing parameters remain unrecoverable.

One earlier decision is intentionally reversed: wave 64 noted that `0x80168400`
and `0x80168480` "dereference the cast result at `+0xE8` without a null guard"
and left both out. `0x80168400` is translated here because the function itself
is literal (cast, then read `+0xE8`) and the unguarded read is documented as a
preserved property of the original, not as a new inference. `0x80168480` stays
out for the reason above.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/resource/registry.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors (and of diagnostics) for
  `src/resource/registry.rs`.
- `node tools/check-lane-csv.mjs wave78` reports `OK`.
