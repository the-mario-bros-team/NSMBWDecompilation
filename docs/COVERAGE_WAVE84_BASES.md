# Coverage - Wave 84 (`game::bases`, `0x806F` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** under the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader chooses the real load address, so the numbers only
identify positions inside the REL image. Cross-module pointers (`0x8035xxxx`,
`0x8060xxxx`, `0x807Cxxxx`) are `main.dol`/REL data addresses imported by the
module and keep their absolute values.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`. "Role" below is the
plain-English behaviour. Confidence is `high` when guards, stores, constants
and call arguments are literal, and `medium` when the role of a field, a
global, a callee or a dropped argument had to be inferred (those bodies carry
`// observed:`/`// inferred:`/`// unknown:` notes). No body in this pass is
`low`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 29 | 3,256 | 11 | 18 |

## What this pass covers

The `0x806F` bucket holds 195 functions; 53 were already in
`ledger/functions.csv` when this pass began, leaving 142. This pass takes 29 of
the remaining bodies in five groups.

- **The mini-game manager state machine, `0x806F8EE0..0x806F9410` (9 bodies).**
  This is the behavioural core of the pass and the reason the bucket was
  picked. The class is the `0x398`-byte manager allocated by [`fn_806f8a70`]
  and built by [`fn_806f8ad0`] (vtable `0x80802530`, global state object
  `0x8082ED60`); its interface `+0xA0` receives the descriptors registered by
  [`fn_806f98c0`]. The run is the credit pass [`fn_806f8ee0`] (four
  `0x0C`-stride records at `+0x74` credited through `0x80060460`), the shared
  timeout gate [`fn_806f8f70`] with its setter [`fn_806f8fc0`] and reset
  [`fn_806f9010`], the five-state pre-round driver [`fn_806f9060`], the
  terminal publish [`fn_806f9250`], the `0x8015F140` tween and result gate
  [`fn_806f92f0`], the restart [`fn_806f93b0`] and the four-state result
  driver [`fn_806f9410`]. Only the two 12-byte alignment pads break the run.
- **Balloon/vehicle position pair, `0x806F7A40..0x806F7AB0` (2 bodies).** The
  mover writer [`fn_806f7a40`] (position `+0xAC`/`+0xB0` into mover `+0x628`,
  pole variant from `0x807CC294`) and the mover reader [`fn_806f7ab0`] are the
  two directions of the same transform and share every field.
- **Mirror toggles and frame gate, `0x806F68F0..0x806F7940` (3 bodies).** The
  two `+0x580` mirror toggles [`fn_806f68f0`]/[`fn_806f6990`] read the same
  `0x8082ED60 + 0xE0` flag as the manager above, and [`fn_806f7940`] re-arms
  the two movers when `0x8016A5D0` reports a new frame.
- **Six registered-state destructors, `0x806F2CD0..0x806F8D90` (6 bodies).**
  Four `0x8015F800`+`0x802B9280` destructors ([`fn_806f2cd0`],
  [`fn_806f50a0`], [`fn_806f5f00`], [`fn_806f7280`]) and two plain free
  guards ([`fn_806f8d50`]/[`fn_806f8d90`]) whose export lost the pointer
  argument of `0x802B9280`.
- **Allocators, forwarders and one handle gate (9 bodies).** The `0x718` and
  `0x398` allocator wrappers ([`fn_806f7400`]/[`fn_806fc9c0`]), six
  `0x802DCBFC` in-place forwarders ([`fn_806f51c0`]/[`fn_806f51f0`],
  [`fn_806f8a10`]/[`fn_806f8a40`], [`fn_806f9860`]/[`fn_806f9890`]) and the
  handle-gated release [`fn_806f4b10`].

## Functions

| address | function | bytes | confidence | role |
|---|---|---:|---|---|
| `0x806F2CD0` | `fn_806f2cd0` | 88 | high | `0x8015F800` teardown + optional free |
| `0x806F4B10` | `fn_806f4b10` | 208 | medium | handle-gated interface probe and release |
| `0x806F50A0` | `fn_806f50a0` | 88 | high | teardown destructor (copy) |
| `0x806F51C0` | `fn_806f51c0` | 48 | medium | `0x802DCBFC` forwarder |
| `0x806F51F0` | `fn_806f51f0` | 48 | medium | `0x802DCBFC` forwarder (copy) |
| `0x806F5F00` | `fn_806f5f00` | 92 | high | teardown destructor (copy) |
| `0x806F68F0` | `fn_806f68f0` | 116 | medium | mirror step + `0x8082EB30` when flag set |
| `0x806F6990` | `fn_806f6990` | 116 | medium | mirror step + `0x8082EB70` when flag clear |
| `0x806F7280` | `fn_806f7280` | 88 | high | teardown destructor (copy) |
| `0x806F7400` | `fn_806f7400` | 48 | high | allocate `0x718` + constructor |
| `0x806F7940` | `fn_806f7940` | 100 | medium | frame-change mover invalidation |
| `0x806F7A40` | `fn_806f7a40` | 112 | high | position into mover + pole variant |
| `0x806F7AB0` | `fn_806f7ab0` | 84 | high | mover translation into position |
| `0x806F8A10` | `fn_806f8a10` | 48 | medium | `0x802DCBFC` forwarder |
| `0x806F8A40` | `fn_806f8a40` | 48 | medium | `0x802DCBFC` forwarder (copy) |
| `0x806F8D50` | `fn_806f8d50` | 64 | medium | plain null/free guard |
| `0x806F8D90` | `fn_806f8d90` | 64 | medium | plain null/free guard (copy) |
| `0x806F8EE0` | `fn_806f8ee0` | 140 | medium | four per-player count credits |
| `0x806F8F70` | `fn_806f8f70` | 72 | medium | timeout gate over record flags |
| `0x806F8FC0` | `fn_806f8fc0` | 8 | high | timer store |
| `0x806F9010` | `fn_806f9010` | 72 | medium | idle reset + global flag |
| `0x806F9060` | `fn_806f9060` | 464 | medium | five-state pre-round driver |
| `0x806F9250` | `fn_806f9250` | 148 | high | terminal publish + cue `0x5DD` |
| `0x806F92F0` | `fn_806f92f0` | 176 | medium | `0x8015F140` tween + result event |
| `0x806F93B0` | `fn_806f93b0` | 84 | high | restart from state 0 |
| `0x806F9410` | `fn_806f9410` | 488 | medium | four-state result driver |
| `0x806F9860` | `fn_806f9860` | 48 | medium | `0x802DCBFC` forwarder |
| `0x806F9890` | `fn_806f9890` | 48 | medium | `0x802DCBFC` forwarder (copy) |
| `0x806FC9C0` | `fn_806fc9c0` | 48 | high | allocate `0x398` + constructor |

## Address range

`0x806F2CD0..0x806F9890` plus the isolated `0x806FC9C0`, five groups:

- Group 1 is `0x806F8EE0..0x806F9410`, nine bodies inside an eleven-slot run.
  The holes (`0x806F8FC8..0x806F9010`, `0x806F9230..0x806F9250` and the
  `0x0C`/`0x10`-byte pads after `0x806F92F0` and `0x806F93B0`) carry no
  `functions.tsv` entry, so no evidence file exists for them and no row could
  be written.
- Group 2 is the pair `0x806F7A40..0x806F7AB0`.
- Group 3 is `0x806F68F0..0x806F6990` plus `0x806F7940`.
- Group 4 is the six destructors, spread over `0x806F2CD0..0x806F8D90`.
- Group 5 is `0x806F51C0..0x806F7400`, `0x806F8A10..0x806F8A40`,
  `0x806F9860..0x806F9890` and `0x806FC9C0`.
- After this pass the bucket still has 113 untranslated functions.

## Deliberate skips

Nothing in this pass was rejected as unreconstructable. The bodies left in the
bucket are deferred for budget, not for quality:

- The `0x806F2100..0x806F2710` pair of `520`-byte bodies and their `0x806F2560`
  / `0x806F25B0` helpers are a self-contained cluster that belongs with the
  `0x806F1FD0` actor already translated in an earlier wave; a single pass
  should take the whole run.
- `0x806F3350` (2,092 bytes) and `0x806F42F0` (332 bytes) form the largest
  remaining block in the bucket and need their own read.
- `0x806F7BB0` (1,004 bytes) is the balloon actor's main body; the five
  members translated here ([`fn_806f7940`], [`fn_806f7a40`],
  [`fn_806f7ab0`], plus the already-translated `0x806F7690`/`0x806F80D0`)
  are its neighbours, so it is the obvious next target.
- `0x806F5640`, `0x806F6040..0x806F6700` and `0x806F7500` are mid-size member
  bodies with readable exports that were simply not reached.
- `0x806FA310..0x806FB130` and `0x806FB540..0x806FC360` sit between the two
  already-translated mini-game classes and were not opened in this pass.
- The six `0x802DCBFC` forwarders are included, even though the stub's hidden
  `r12` this-adjust is not modelled (the same limitation is documented for
  [`fn_806512f0`]); their confidence is `medium` for that reason.
- `0x806F8D50`/`0x806F8D90` are included with a `medium` note because the
  export prints `func_0x802b9280()` with no arguments while the pointer is
  still in `r3`; the pointer is passed, matching every other release in this
  file.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (29 rows, 3,256 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the 53 pre-existing rows of
  this bucket were listed before writing and none is in this pass.
- `node tools/check-lane-csv.mjs wave84` reports `wave84-bases.csv: 29 rows`
  and `OK`.
- `cargo check --message-format=short` produces no diagnostic for
  `src/game/bases.rs`.
