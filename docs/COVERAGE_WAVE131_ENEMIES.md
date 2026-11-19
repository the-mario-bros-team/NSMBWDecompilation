# Coverage - Wave 131 (`game::enemies`, wave-130 specification and `0x80B10000` tail)

## How to read this file

This page records the Wave 131 `game::enemies` lane: 26 functions of REL module
3, `d_enemiesNP`, appended to `src/game/enemies.rs` as section 113. Every row of
the machine-readable mirror (`ledger/agents/wave131-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none of
these is a runtime address; every function doc repeats that note. Data constants
are cited by address and read through `platform::read_u32` / `read_f32`, never
transcribed as invented values. Raw words were decoded from
`work/rels/elf/d_enemiesNP.elf` at the mapping in `docs/REL_ELF_MAPPING.md`
(`file_offset = address - 0x80A00000 + 0x1000`).

**Decoding note established by this pass.** The `D` field of every `lfs`/`lwz`
in this build is a *signed* 16-bit displacement, so `lis rX,0x80B7` followed by
`lfs f,0x9E34(rX)` resolves to `0x80B69E34`, not `0x80B79E34`. The same rule
turns `lis 0x8043` + `addi 0xC1A0` into `0x8042C1A0`, which matches the
`main.dol` segment map in `work/dol/main.elf` (`0x8042B080..0x8042FBE0`). Every
`main.dol` operand in this lane was re-checked against that map.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 26 | 6,256 | 15 | 11 |

The lane has two halves:

- **12 bodies** are the "ready" rows of `docs/SPEC_WAVE130_ENEMIES.md`. They
  were written from that page's instruction-level decode and re-verified word
  by word against both the raw ELF stream and the Ghidra C export.
- **14 bodies** are still-open entries of the planner bucket `0x80B10000`
  (38 of that bucket's 211 functions were open when the pass started; this pass
  takes 14 and leaves 24).

## What this pass covers

- **The twelve specification bodies (`0x80A33630..0x80B1E540`).** Three matrix
  refreshes (`0x80A7FB50` icicle kind-test scale, `0x80AE63D0` `+0xE54` scale,
  `0x80A33630` `(0, +10, -4)` pair), the two-matrix `z+32`/`z+64` builder
  `0x80AB1160`, the burst-water per-frame step `0x80AB2460`, the 60-byte
  particle record `0x80AB0C00`, the `pukupuku` model/damage binder
  `0x80B0F5A0`, the `+0x854` state callback `0x80AD4420`, the temporary-floor
  step `0x80ACC740`, the `+0x360` two-state ramp `0x80B1E540`, the `walk_*`
  clip-pair selector `0x80A7BD30` and the angle-flip effect step `0x80AC9AA0`.
- **The `0x80B10000` tail (14 bodies, `0x80B123D0..0x80B1FD00`).** The two
  `0x670`-byte constructors `0x80B15DE0`/`0x80B15EF0`, the three `0x802DCBFC`
  argument-swapping forwarders `0x80B1A310`/`0x80B1FCD0`/`0x80B1FD00`, the two
  coordinate floor helpers `0x80B177E0`/`0x80B17860` and their holder refresh
  `0x80B16EE0`, the `+0x54C` slot wrapper `0x80B16580`, the controller-gated
  state setter `0x80B17340`, the `+0x528` timer callback `0x80B19EE0`, the
  motion-field reset `0x80B17CB0`, the post-hit reset `0x80B123D0` and the
  `+0x550` offset placement `0x80B15C60`.
- **Two new section-local adapters**: `w131_vt` / `w131_vt1` (a vtable slot with
  `r3 = receiver`, and with one extra argument) and `w131_emit_effect` (the
  `+0xA0` effect-record call, used by `0x80AC9AA0` twice). The existing
  `w130_state_call` and `w130_rotate` are reused unchanged because they describe
  generic `+0x60`/`+0xD4` and one-`u16` rotation shapes.

### Confirmations against the specification

Every field offset, constant address, store order and call argument of the
twelve specification bodies matched the raw ELF words. Four details worth
recording:

- **`0x80AD4420`'s masked word.** `0x80AD4544` is `rlwinm. r0,r0,0,22,22`
  (`0x540005AD`, `Rc = 1`), and `0x80AD457C` is `beq 0x80AD4590`: bit 22 clear
  takes the `rlwinm 0,25,23` arm that clears `0x80`, bit 22 set falls through to
  `rlwinm 0,25,21` that clears `0x200`. The specification and the export agree.
- **`0x80AB0C00`'s three arms.** All three `+0x98C` arms are byte-identical
  (13 stores each), so the body keeps only the selector's read. The eight
  trailing zero words are written before the type and position words in the
  original; the local array is filled in address order and the original store
  order is noted in a comment.
- **`0x80AB1160`'s second matrix.** The second `0x801C0BD0` call adds only the
  third component (`+0x8A4`) and re-reads three *fresh* zero angle slots. The
  export drops the `f3` operand; the body passes `position[2]` explicitly.
- **`0x80AC9AA0`'s sign test.** `0x80AC9BB4` is `mullw.` of the saved and the
  new `+0x102` halfwords; the two helpers run only when the product is `<= 0`
  (`bgt` skips them), which the body reproduces as `product <= 0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AB2460` | `fn_80ab2460` | 444 | high |
| `0x80AB0C00` | `fn_80ab0c00` | 440 | high |
| `0x80B0F5A0` | `fn_80b0f5a0` | 440 | high |
| `0x80AD4420` | `fn_80ad4420` | 404 | medium |
| `0x80A7FB50` | `fn_80a7fb50` | 392 | medium |
| `0x80ACC740` | `fn_80acc740` | 388 | medium |
| `0x80AE63D0` | `fn_80ae63d0` | 388 | high |
| `0x80AB1160` | `fn_80ab1160` | 384 | high |
| `0x80A33630` | `fn_80a33630` | 380 | medium |
| `0x80B1E540` | `fn_80b1e540` | 380 | medium |
| `0x80A7BD30` | `fn_80a7bd30` | 372 | medium |
| `0x80AC9AA0` | `fn_80ac9aa0` | 372 | medium |
| `0x80B16580` | `fn_80b16580` | 48 | high |
| `0x80B1A310` | `fn_80b1a310` | 48 | medium |
| `0x80B1FCD0` | `fn_80b1fcd0` | 48 | medium |
| `0x80B1FD00` | `fn_80b1fd00` | 48 | medium |
| `0x80B16EE0` | `fn_80b16ee0` | 88 | high |
| `0x80B177E0` | `fn_80b177e0` | 72 | high |
| `0x80B17860` | `fn_80b17860` | 72 | high |
| `0x80B17340` | `fn_80b17340` | 100 | high |
| `0x80B19EE0` | `fn_80b19ee0` | 104 | high |
| `0x80B17CB0` | `fn_80b17cb0` | 136 | medium |
| `0x80B123D0` | `fn_80b123d0` | 140 | high |
| `0x80B15C60` | `fn_80b15c60` | 156 | high |
| `0x80B15DE0` | `fn_80b15de0` | 200 | high |
| `0x80B15EF0` | `fn_80b15ef0` | 212 | high |

### Why eleven rows are `medium`

- **The three `0x802DCBFC` forwarders.** The three-register shuffle is literal,
  but the hidden `r12` this-adjust pointer (`first + 0x24`) cannot be modelled
  by an `extern "C"` call, exactly as wave 130 recorded for the `0x80ADD200`
  pair.
- **`0x80AD4420`, `0x80ACC740`, `0x80B1E540`.** Every branch, store and token is
  literal; the state-phase, body-object and target-table roles are inferred from
  the compared values and the call shapes.
- **`0x80A7FB50`.** The kind test consumes only the token, but the token'd
  object's slot identity is not established.
- **`0x80A33630`, `0x80A7BD30`, `0x80AC9AA0`.** The arithmetic is literal; the
  holder/emitter field roles come from the sibling matrix builders and the
  `pukupuku` block.
- **`0x80B17CB0`.** Every store is exact; `0x800A16A0` prints no arguments in
  the export, so its `r3` is taken from the untouched incoming register.

## Deliberate skips

- **`0x80B14D80` (28 bytes).** The export is a single `halt_baddata` after the
  truncation; the reachable stream is not recoverable, so no body was invented.
- **`0x80B18F20` (228 bytes).** The export recurses with an extra implicit
  second argument (the freshly allocated object), so the call shape is not
  established from the export alone; left untranslated rather than guessed.
- **The remaining `0x80B10000` rows** (24 still open, including the `0x1184`-byte
  `0x80B147D0`, the `0x80B16060`/`0x80B162C0`/`0x80B16F40`/`0x80B179D0`
  cluster, and `0x80B1D090`/`0x80B1D520`/`0x80B1DA40`). They are readable and
  were left for a later pass once the body target was met. Nothing was written
  for them, so no ledger row claims them.
- **No address already in `ledger/functions.csv` was touched**, and no other
  lane's file, CSV or doc was modified.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 26 sizes sum to
  6,256 bytes (4,784 for the twelve specification rows and 1,472 for the
  fourteen bucket rows).
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 26 was in
  `ledger/functions.csv` before this pass.
- The module header was updated: cluster item 113, the function totals (3,015
  to 3,041 public translations, 2,944 to 2,970 `fn_<addr>` bodies), the "start
  here" list and the records list.
- `cargo check --message-format=short` emits no error and no warning naming
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave131` reports `wave131-enemies.csv: 26 rows`,
  26 unique addresses and `OK`.
