# Coverage - Wave 45 (resource::registry)

## How to read this file

Each row is one real `pub unsafe fn` body in `src/resource/registry.rs`. The
address and byte count come from
`work/ghidra/main-gb/export/functions.tsv`; the evidence path is the Ghidra C
export that was read before the body was written. `confidence` matches the
`# Confidence` line of the function's doc block.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/registry.rs` | `resource::registry` | 28 | 2184 | 23 | 5 |

## What this pass covers

- Live-context vtable wrappers (`0x80164E50`-`0x80164F50`, 5 bodies): the
  `+0x34` call that forwards the `+0xE4` object count, the `+0x38` pop loop,
  the two-slot update helper, the looked-up `+0xC` word query and the
  channel-0-then-sixteen reset.
- Element wrapper lifecycle (`0x801693A0`-`0x80169D20`, 8 bodies): release of
  the `+0x24` pair, the `0x80329658` nested constructor, a copy-on-success
  free, the one-word virtual `+0x14` forwarder, the `+0x28`/`+0x30` owner
  teardown, the unfinished-object advance, the `0x80329638` wrapper
  constructor and its destructor.
- Second element factory/wrapper set (`0x8016A1A0`-`0x8016A3A0`, 5 bodies):
  the two-state flag setter, the two-call volume application, the
  `0x80329688` constructor, the reset-and-free helper and the two-mode `+4`
  handle factory.
- Copy-on-success frees (`0x8016AF90`-`0x8016B050`, 4 bodies): four separate
  entry points with the same `{non-null, count > 0} -> 0x802B9280` shape.
- Matrix and table reset (`0x8016F150`-`0x8016F7D0`, 6 bodies): the twelve-word
  constant matrix fill, the `0x18`-stride slot clear at `0x80377D08`, the
  four-slot loop, the task starter with its `-1` fallback, one more
  copy-on-success free and the entry initialiser that marks both state bytes.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80164E50` | `fn_80164e50` | 32 | high |
| `0x80164E70` | `fn_80164e70` | 80 | high |
| `0x80164EC0` | `fn_80164ec0` | 60 | high |
| `0x80164F00` | `fn_80164f00` | 76 | medium |
| `0x80164F50` | `fn_80164f50` | 68 | high |
| `0x801693A0` | `fn_801693a0` | 76 | high |
| `0x80169560` | `fn_80169560` | 84 | high |
| `0x801695C0` | `fn_801695c0` | 64 | high |
| `0x80169AC0` | `fn_80169ac0` | 92 | high |
| `0x80169C20` | `fn_80169c20` | 92 | medium |
| `0x80169C90` | `fn_80169c90` | 64 | high |
| `0x80169CD0` | `fn_80169cd0` | 68 | high |
| `0x80169D20` | `fn_80169d20` | 104 | high |
| `0x8016A1A0` | `fn_8016a1a0` | 120 | medium |
| `0x8016A2A0` | `fn_8016a2a0` | 72 | high |
| `0x8016A2F0` | `fn_8016a2f0` | 68 | high |
| `0x8016A340` | `fn_8016a340` | 88 | high |
| `0x8016A3A0` | `fn_8016a3a0` | 180 | medium |
| `0x8016AF90` | `fn_8016af90` | 64 | high |
| `0x8016AFD0` | `fn_8016afd0` | 64 | high |
| `0x8016B010` | `fn_8016b010` | 64 | high |
| `0x8016B050` | `fn_8016b050` | 64 | high |
| `0x8016F150` | `fn_8016f150` | 56 | high |
| `0x8016F500` | `fn_8016f500` | 68 | high |
| `0x8016F550` | `fn_8016f550` | 60 | high |
| `0x8016F5D0` | `fn_8016f5d0` | 100 | medium |
| `0x8016F6E0` | `fn_8016f6e0` | 64 | high |
| `0x8016F7D0` | `fn_8016f7d0` | 92 | high |

## Deliberate skips

- `0x801613A0`-`0x80161530` (5 bodies): floating-point conversion and
  pseudo-random helpers whose exports read unnamed `r2` constants and carry
  GQR/`ldexpf` artefacts; only the instruction sequence is visible, so the
  arithmetic was not guessed.
- `0x80160D10` (368 bytes), `0x80160EA0` (104 bytes) and `0x801611C0` (436
  bytes): already called out by wave 41; the exports remain polluted by
  unrecovered `r2` global floats and quadrant branch trees.
- `0x801693F0` (68 bytes): a ten-argument forwarder to `0x80169440`. The
  argument list is literal, but the platform layer exposes only `call0..call7`
  and adding a ten-argument trampoline was not justified for one body.
- `0x8016A0F0` is a jumptable routine already translated in wave 44; the new
  call sites use the absolute address with three arguments so the forwarded
  registers are preserved.
- The remaining 184 untranslated entries in the `0x80160000` bucket are sound,
  scene and element code outside this pass's contiguous runs.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/resource/registry.rs`.
- No address was already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave45` reports `OK` for
  `ledger/agents/wave45-registry.csv`.
- `cargo check --message-format=short` reports no errors in
  `src/resource/registry.rs`.
- Module header gained the wave-45 cluster table; the status count moved
  211 -> 239.
