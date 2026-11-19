# Coverage - Wave 52 (resource::registry, 0x80160000 bucket)

## How to read this file

Each row is one real `pub unsafe fn` body in `src/resource/registry.rs`. The
address and byte count come from
`work/ghidra/main-gb/export/functions.tsv`; the evidence path is the Ghidra C
export that was read before the body was written. `confidence` matches the
`# Confidence` line of the function's doc block.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/registry.rs` | `resource::registry` | 28 | 3000 | 14 | 14 |

## What this pass covers

- Intrusive list primitives (`0x80161700`-`0x80161C40`, 8 bodies): the payload
  walk, tail append, keyed unlink, whole-list clear, the two `u16` state-machine
  steps, the "run until state 1" wrapper and the sound-bank teardown that
  unlinks its `+0x54` list and restores the `0x80329410` vtable.
- Sound-object helpers (`0x80163A60`-`0x801644D0`, 10 bodies): the `+0x94` bit
  forwarder, the four-word transform query plus virtual `+0x30`, the child
  factory with its `+0x84` fallback, the recursive-result flag mapper, the
  `+0x14` release, the two circular-list sweeps, the two-level float store, the
  value re-apply with its enable bit and the three-word descriptor forwarder.
- Element wrappers (`0x80168100`-`0x8016A090`, 5 bodies): the teardown vtable
  and indirect slot, the `0x802F8FD8` cast/install path, the 52-byte
  copy-out wrapper, the wrapper destructor and the `+0x04` child release.
- Interpolator leaves (`0x80168B40`-`0x80168D80`, 5 bodies): the two reset
  constants, the five-field restore, the reset-value test, the buffer-view
  constructor and the reset-then-free destructor.

The list nodes use `{next +0x00, prev +0x04, payload +0x08, key +0x0C}`; the
interpolator uses `+0x04` current, `+0x0C` step, `+0x1C`/`+0x1D` state bytes
and `+0x28`/`+0x30` published values. Both layouts are shared with bodies that
were already in this file (`fn_80160f10..` and `fn_8016f150..`).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80161700` | `fn_80161700` | 60 | high |
| `0x80161750` | `fn_80161750` | 160 | high |
| `0x80161800` | `fn_80161800` | 180 | high |
| `0x801618C0` | `fn_801618c0` | 92 | high |
| `0x80161920` | `fn_80161920` | 140 | medium |
| `0x801619B0` | `fn_801619b0` | 188 | medium |
| `0x80161A70` | `fn_80161a70` | 88 | high |
| `0x80161C40` | `fn_80161c40` | 120 | high |
| `0x80163A60` | `fn_80163a60` | 36 | medium |
| `0x80163A90` | `fn_80163a90` | 108 | medium |
| `0x80163C70` | `fn_80163c70` | 196 | medium |
| `0x80163E10` | `fn_80163e10` | 72 | medium |
| `0x80163FB0` | `fn_80163fb0` | 60 | medium |
| `0x80164080` | `fn_80164080` | 152 | medium |
| `0x80164120` | `fn_80164120` | 144 | medium |
| `0x801642C0` | `fn_801642c0` | 228 | medium |
| `0x80164460` | `fn_80164460` | 108 | high |
| `0x801644D0` | `fn_801644d0` | 24 | high |
| `0x80168100` | `fn_80168100` | 116 | medium |
| `0x80168180` | `fn_80168180` | 168 | medium |
| `0x80168230` | `fn_80168230` | 80 | medium |
| `0x8016A030` | `fn_8016a030` | 96 | high |
| `0x8016A090` | `fn_8016a090` | 68 | high |
| `0x80168B40` | `fn_80168b40` | 60 | high |
| `0x80168B90` | `fn_80168b90` | 32 | high |
| `0x80168CB0` | `fn_80168cb0` | 24 | high |
| `0x80168D10` | `fn_80168d10` | 112 | medium |
| `0x80168D80` | `fn_80168d80` | 88 | high |

## Deliberate skips

- `0x801613A0`-`0x80161530` remain skipped for the same reason as wave 45: the
  exports carry unrecovered `r2` float constants, GQR/`ldexpf` artefacts and
  quadrant branch trees, so the arithmetic would have to be guessed.
- `0x80160D10` (368 B), `0x80160EA0` (104 B) and `0x801611C0` (436 B) are still
  polluted by unnamed `r2` floats and unmodelled conversion helpers.
- `0x80168BD0` (60 B), `0x80168C10` (148 B), `0x80168F10` (196 B),
  `0x80168FE0` (60 B) and `0x80169020` (192 B) are the remaining interpolator
  math leaves: their branches read the same `r2` floats but combine them in a
  way that cannot be written without inventing the constant values.
- `0x80168400` and `0x80168480` dereference the cast result without a null
  guard in the export; reproducing that literally would read from address
  `0xE8`, so both were left out rather than guessed.
- `0x8016AB90` calls its base constructor with an empty argument list in the
  export; the missing two parameters are not recoverable, so it was skipped.
- `0x80161740`, `0x801644F0`, `0x8016B950`, `0x8016BCB0`, `0x8016C820`,
  `0x8016E4F0` and `0x8016E8E0` are 4-byte empty routines or thunks below the
  size floor used by the earlier waves.
- `0x80163D40` (208 B) and `0x801641B0` (260 B) are recursive
  Metrowerks-critical-section helpers; the critical-section entry returns a
  two-register value that the platform layer does not model, so they are called
  by address with their roles documented as `unknown:`.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/resource/registry.rs`.
- No address was already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave52` reports `OK` for
  `ledger/agents/wave52-registry.csv`.
- `cargo check --message-format=short` reports no errors in
  `src/resource/registry.rs`.
- Module header gained the wave-52 cluster table; the status count moved
  239 -> 267.
