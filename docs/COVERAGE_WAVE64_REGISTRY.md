# Coverage - Wave 64 (resource::registry, 0x80160000 bucket)

## How to read this file

Each row is one real `pub unsafe fn` body in `src/resource/registry.rs`. The
address and byte count come from
`work/ghidra/main-gb/export/functions.tsv`; the evidence path is the Ghidra C
export that was read before the body was written, and every row cites
`work/ghidra/main-gb/export/<addr>_FUN_<addr>.c`. `confidence` matches the
`# Confidence` line of the function's doc block.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/registry.rs` | `resource::registry` | 30 | 3960 | 19 | 11 |

## What this pass covers

- Tree/list primitives (`0x80161530`-`0x80161620`, 3 bodies): the four-word
  header clear, the parent/first-child/prev/next append, and the unlink that
  refuses a node which still owns children.
- Actor state (`0x80161EF0`-`0x80162B00`, 4 bodies): the `+0x50` queue gate,
  the 552-byte `+0x0B`/`+0x0A` enable/disable machine that moves entries between
  `SB_LIST_2..SB_LIST_5`, the recursive `+0x14` child-chain shutdown mark, and
  the `+0x10` descriptor lookup.
- Sorted list (`0x80163850`, 1 body): ordered insert into `SB_SORTED_LIST`
  through `0x80228FF0`/`0x80228DF0`/`0x80228E60`, keyed by the byte at `+0x0C`.
- Element wrapper forwarder (`0x801651F0`, 1 body): virtual `+0x0C` value,
  cast through `0x80168520`, then the keyed-entry rebuild at `0x80165280`.
- Three `0x38`-stride element classes (`0x80165820`-`0x80167AB0`, 20 bodies).
  Each class has an instance size query (allocator query plus the `0x80` header
  allowance rounded to `0x20`), a counted size query (`count * 0x38` plus the
  aligned per-element size), a `+0x04` child release that returns the buffer to
  heap class 3, a per-element release through the `+0x28` array and virtual
  slot `+0x14`, and a deleting destructor that installs its vtable and calls
  the shared base. Classes A/B/C differ only in the query address
  (`0x80243D20`, `0x80245500`, `0x80246B00`) and the vtable word
  (`0x80329518`, `0x80329548`, `0x80329578`).
- Shared base teardown (`0x80167EC0`-`0x80167F30`, 2 bodies): restores the base
  vtable, destroys `+0x04` through `0x80249B40`/`0x80249AC0`, destroys the
  `+0x08` frame heap through [`fn_8016e740`], and destroys the `+0x0C` view.
- Small leaves (`0x8016C6F0`-`0x8016FF60`, 4 bodies): the two conditional-free
  twins, the `+0x74`/`+0x7C` descriptor copy, and the UI entry initialiser.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80161530` | `fn_80161530` | 48 | high |
| `0x80161580` | `fn_80161580` | 136 | high |
| `0x80161620` | `fn_80161620` | 152 | high |
| `0x80161EF0` | `fn_80161ef0` | 104 | medium |
| `0x801622E0` | `fn_801622e0` | 552 | high |
| `0x80162510` | `fn_80162510` | 116 | medium |
| `0x80162B00` | `fn_80162b00` | 24 | medium |
| `0x80163850` | `fn_80163850` | 144 | medium |
| `0x801651F0` | `fn_801651f0` | 132 | medium |
| `0x80165820` | `fn_80165820` | 124 | high |
| `0x80165A80` | `fn_80165a80` | 92 | high |
| `0x80165B50` | `fn_80165b50` | 224 | high |
| `0x80165E70` | `fn_80165e70` | 108 | medium |
| `0x80166280` | `fn_80166280` | 96 | high |
| `0x80166420` | `fn_80166420` | 124 | high |
| `0x80166680` | `fn_80166680` | 92 | high |
| `0x80166750` | `fn_80166750` | 224 | high |
| `0x80166A70` | `fn_80166a70` | 108 | medium |
| `0x80166E80` | `fn_80166e80` | 96 | high |
| `0x80167000` | `fn_80167000` | 124 | high |
| `0x80167270` | `fn_80167270` | 92 | high |
| `0x80167340` | `fn_80167340` | 224 | high |
| `0x801676A0` | `fn_801676a0` | 108 | medium |
| `0x80167AB0` | `fn_80167ab0` | 96 | high |
| `0x80167EC0` | `fn_80167ec0` | 108 | high |
| `0x80167F30` | `fn_80167f30` | 100 | medium |
| `0x8016C6F0` | `fn_8016c6f0` | 64 | high |
| `0x8016CFD0` | `fn_8016cfd0` | 192 | medium |
| `0x8016DCA0` | `fn_8016dca0` | 64 | high |
| `0x8016FF60` | `fn_8016ff60` | 92 | medium |

## Deliberate skips

- `0x801613A0`-`0x80161530` (except `0x80161530`) stay skipped for the wave-45
  and wave-52 reason: the exports read unnamed `r2` float constants and carry
  GQR/`ldexpf` artefacts, so the arithmetic would have to be guessed.
  `0x80161060` and `0x801610B0` are likewise matrix compositions whose callees
  take their arguments in the float registers, which `platform::callN` cannot
  set.
- `0x80161AD0` (368 B) and `0x80161DA0` (144 B) read `r13` offsets that the
  export does not resolve and call `0x802DCBFC` with three different argument
  lists; the dispatcher identity is not recoverable.
- `0x80161E40` and `0x80162020` (168 B each) copy nine words from the
  `0x8032935x`/`0x8032938x` globals into a stack object, but their only callee
  is the ambiguous `0x80161CC0` above, so no body was claimed.
- `0x80167FA0` (188 B) and `0x80167C40`/`0x80167D60` are the class constructors
  that consume the `r13 - 0x52B8` heap global, which the export leaves
  unresolved; the three size queries and destructors that do not need it were
  translated instead.
- `0x80168400`/`0x80168480` dereference the cast result at `+0xE8` without a
  null guard in the export, as already noted in wave 52; they stay out.
  `0x80168520` and `0x80165280` are untranslated class-B helpers and are called
  by address from [`fn_801651f0`].
- `0x801693F0` (68 B) is a ten-argument forwarder; `platform` exposes only
  `call0..call7`, so it stays untranslated rather than adding a trampoline for
  one body.
- `0x8016AB90` calls its base constructor with an empty argument list in the
  export; the missing two parameters are not recoverable.
- The remaining `0x80160000`-bucket entries are 4-byte thunks/empty routines,
  `r2`/`r13`-dependent math leaves, Metrowerks critical-section helpers whose
  two-register return is not modelled by `platform`, or GX/`unaff` bodies that
  cannot be written without inventing constants.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/resource/registry.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors for `src/resource/registry.rs`.
