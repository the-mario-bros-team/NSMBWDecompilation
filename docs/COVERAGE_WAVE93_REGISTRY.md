# Coverage - Wave 93 (resource::registry)

## How to read this file

Every row below is one real `pub unsafe fn` body in
`src/resource/registry.rs`. The address and byte count come from
`work/ghidra/main-gb/export/functions.tsv`; the evidence path is the Ghidra C
export that was read before the body was written. `confidence` matches the
`# Confidence` line of the function's doc block, and the role is the same
plain-English summary as the module-header table.

This pass only claims what the export shows. Two kinds of evidence need a
short explanation:

- `FUN_802dcd5c/64/68/6c/70/74` are Metrowerks **prologue fragments**. The
  `undefined8` the decompiler shows as their return is the enclosing
  function's own incoming `r3:r4`; `hi` is the first parameter and `lo` the
  second. The bodies spell those two out as explicit Rust parameters, the same
  rule wave 89 used for `0x8016E500`/`0x8016E7F0`.
- Some virtual calls are printed as `(...)()` with the receiver dropped. Where
  the receiver is unambiguously the object stored in the field the code just
  tested (or the object just allocated), the body supplies it and the doc block
  says `// inferred: receiver`. The bytes and slots are literal; only the
  plumbed argument is inferred.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/registry.rs` | `resource::registry` | 26 | 7316 | 3 | 23 |

## What this pass covers

The `0x80160000` bucket still held 74 untranslated bodies when the pass
started (421 total, 347 already done). This pass takes 26 of them and leaves 48
documented skips, listed at the end.

- **Sound-group draw and description leaves** (`0x80163520`, `0x80163E60`,
  `0x801641B0`, 3 bodies). The draw helper's count/flag guard, the description
  copy with its eight-byte entry table, and the recursive `+0x58`/`+0x64`/
  `+0x6C` match probe.
- **Deferred reader view** (`0x80164500`, 1 body). Allocates the `0x1c` view,
  builds its LZ wrapper, publishes `r13 - 0x52B8` and `r13 - 0x52B4`, and frees
  the view through its `+0x10` slot on failure.
- **Layout-group class family** (`0x801650D0`, `0x801658A0`, `0x801659C0`,
  `0x80165C30`, `0x80165FD0`, `0x801664A0`, `0x801665C0`, `0x80166830`,
  `0x80166BD0`, `0x80167080`, `0x801671B0`, `0x80167420`, `0x80167800`,
  `0x80167C40`, `0x80167D60`, 15 bodies). Three element classes plus the
  four-argument variant: one class constructor, one `n`-child group builder,
  one per-element commit and the re-key helper. The classes differ only in
  their constructor, cast type word (`0x802F8588/0x802F85A8`,
  `0x802F85F0/0x802F8610`, `0x802F8658/0x802F8678`), element vtable
  (`0x80329530/60/90`) and commit twin (`0x80165AE0`, `0x801666E0`,
  `0x801672D0`, `0x80167E50`).
- **Angle table** (`0x80161180`, 1 body). The `f64` ratio is rounded to `f32`,
  scaled by `r2 - 0x546C`, truncated and used as a halfword index into
  `0x80328B48`.
- **Handle, record and control page** (`0x80160800`, `0x80161F60`,
  `0x8016C820`, `0x8016D640`, `0x8016DAA0`, `0x8016DAE0`, 6 bodies). The
  handle release with its `r13 - 0x5308` remaining size, the record detach with
  three intrusive unlinks, the empty dispatch leaf, the actor-record
  registration with the `0x80377B1C`/`0x80377C30`/`0x80377C34`/`0x80377C38`
  list, the `0x7FF` text reset and the control install with its `2`/`3` veto.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80160800` | `fn_80160800` | 164 | medium |
| `0x80161180` | `fn_80161180` | 52 | medium |
| `0x80163520` | `fn_80163520` | 320 | high |
| `0x80163E60` | `fn_80163e60` | 332 | medium |
| `0x801641B0` | `fn_801641b0` | 260 | medium |
| `0x80164500` | `fn_80164500` | 236 | medium |
| `0x801650D0` | `fn_801650d0` | 288 | medium |
| `0x801658A0` | `fn_801658a0` | 284 | medium |
| `0x801659C0` | `fn_801659c0` | 180 | medium |
| `0x80165C30` | `fn_80165c30` | 572 | medium |
| `0x80165FD0` | `fn_80165fd0` | 320 | medium |
| `0x801664A0` | `fn_801664a0` | 284 | medium |
| `0x801665C0` | `fn_801665c0` | 180 | medium |
| `0x80166830` | `fn_80166830` | 572 | medium |
| `0x80166BD0` | `fn_80166bd0` | 320 | medium |
| `0x80167080` | `fn_80167080` | 300 | high |
| `0x801671B0` | `fn_801671b0` | 180 | medium |
| `0x80167420` | `fn_80167420` | 636 | medium |
| `0x80167800` | `fn_80167800` | 320 | medium |
| `0x80167C40` | `fn_80167c40` | 280 | medium |
| `0x80167D60` | `fn_80167d60` | 232 | medium |
| `0x8016C820` | `fn_8016c820` | 4 | high |
| `0x8016D640` | `fn_8016d640` | 464 | medium |
| `0x8016DAA0` | `fn_8016daa0` | 60 | medium |
| `0x8016DAE0` | `fn_8016dae0` | 288 | medium |
| `0x80161F60` | `fn_80161f60` | 188 | medium |

Address range: `0x80160800..0x8016DAE0`. All 26 rows are `translated`; none is a
stub and none is `partial`.

## Deliberate skips

48 bodies in the bucket stay untranslated. The groups below are exact: together
they are the 48 addresses that are neither in `ledger/functions.csv` nor in
this pass's CSV.

- **4-byte thunks (8)** `0x80160AD0`, `0x80161740`, `0x80162110`, `0x801644F0`,
  `0x8016B950`, `0x8016BCB0`, `0x8016E4F0`, `0x8016E8E0`: below the size floor,
  as in waves 45/52/64/78/89. `0x80161740`, `0x80162110`, `0x801644F0` are
  literally empty bodies of four bytes, and the other five are single branches;
  neither is a meaningful reconstruction at this size.
- **GQR / `ldexpf` exports (14)** `0x80160D10`, `0x80161400`, `0x80163220`,
  `0x80168F10`, `0x80169020`, `0x801690E0`, `0x80169600`, `0x8016AD30`,
  `0x8016DEC0`, `0x8016E3A0`, `0x8016EBF0`, `0x8016ED10`, `0x8016EE30`,
  `0x8016FA10`: the export carries `ldexpf` recovered from a `ps_*` GQR pair,
  so the arithmetic would have to be invented. `0x8016ED10`/`0x8016EE30` stay
  reachable from the wave-78 formatters through their existing address
  constants.
- **Float arguments or float returns (11)** `0x80160E80`, `0x80160EA0`,
  `0x801611C0`, `0x801613A0`, `0x80163760`, `0x801643B0`, `0x801654B0`,
  `0x80165AE0`, `0x801666E0`, `0x801672D0`, `0x80167E50`: the body forwards
  `f1..f3` to a callee (or consumes a float return), and `crate::platform` can
  only set integer argument registers. The four commit twins are reached by
  address from the bodies that need them, exactly as before.
- **Float work inside a longer body (4)** `0x80169290`, `0x80169440`,
  `0x8016EBA0`, `0x8016F220`: these mix GQR/`ldexpf` arithmetic with a
  multi-double call (`0x801C6FF0`, `0x8022D3C0`) or an elided vtable argument,
  so either half would be a guess.
- **Elided float register (3)** `0x80165280`, `0x80169B20`, `0x80169D90`: the
  body reads `in_f31` (or forwards it to `0x80169B20`), a value the decompiler
  never assigns and that no integer call helper can set.
- **Elided receiver or argument list (8)** `0x80161CC0`, `0x80162A70`,
  `0x801645F0`, `0x80164770`, `0x80168BD0`, `0x80168FE0`, `0x801693F0`,
  `0x8016FD30`: `0x80161CC0` calls `0x802DCBFC` with zero, one and two
  arguments and uses `r3` as both input and output; `0x80162A70` drops the four
  arguments of `0x80162A50`; `0x801645F0` builds a nine-argument call around a
  stack struct that its first callee only partly initialises; `0x80164770`
  drops the heap argument of the save/restore thunk; `0x80168BD0` and
  `0x80168FE0` drop the receiver of their first call with no field to recover
  it from; `0x801693F0` is a ten-argument forwarder and `crate::platform` stops
  at `call7`; `0x8016FD30` consumes `extraout_r4`/`extraout_f1`, i.e. register
  values the decompiler could not recover at all.

## Verification

- Every address and byte size matches `functions.tsv` (re-read this pass, not
  copied from earlier prose).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/registry.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no diagnostic for
  `src/resource/registry.rs` (the only errors in the tree at report time were
  in another lane's `src/runtime/bt.rs`).
- `node tools/check-lane-csv.mjs wave93` reports `wave93-registry.csv: 26 rows`
  and `OK`.
