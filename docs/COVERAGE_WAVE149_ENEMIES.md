# Coverage - Wave 149 (`game::enemies`)

## How to read this file

This page records the Wave 149 `game::enemies` lane: 16 functions of REL module
3, `d_enemiesNP`, appended to `src/game/enemies.rs` as section 131 and mirrored
one row per body in `ledger/agents/wave149-enemies.csv`.

Every `address` is a **synthetic analysis address** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Raw words
were decoded from `work/rels/elf/d_enemiesNP.elf` through the `PT_LOAD` headers
as described in `docs/REL_ELF_MAPPING.md`
(`file_offset = address - 0x80A00000 + 0x1000` for the first segment), and every
body was followed from its entry to its terminal instruction. Byte counts are
the `size` column of `work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 16 | 5528 | 11 | 5 |

Status split: 16 `translated` (11 high, 5 medium), no `partial` and no `stub`.

Covered address range: `0x80A53850..0x80B058A0` (4,976 bytes of
`docs/SPEC_WAVE148_ENEMIES.md` rows and 552 bytes of `0x80A70000`
planner-bucket rows).

## What this pass covers

- **All twelve `docs/SPEC_WAVE148_ENEMIES.md` rows**, including the one marked
  *conditional*: `0x80B058A0` (the rail actor's trailing-segment rebuild),
  `0x80A81150` (the descent step that lands into `walk`), `0x80A5EB00` (the
  clip-gated step with its two land-smoke emits and three exit cues),
  `0x80AE69E0` and `0x80A94100` (the two pair-contact record builders),
  `0x80A5C660` (the multi-slot attack dispatch), `0x80AB26A0` (the two-clause
  arrival gate into the dash state), `0x80A84A00` and `0x80A88BC0` (the ten-link
  chain emitter and the `qr3` spiral solver that feeds it), `0x80AA0530` (the
  `daEnKanon_c` dash/overlap step), `0x80AB29C0` (the five-link chain
  integrator) and `0x80A53850` (the rail-record snap with its surface-crossing
  effect).
- **Four `0x80A70000` planner-bucket rows.** `0x80A7AB90` (the per-frame
  vertical placement step over the `0x80B2E0C0`/`0x80B2E0CC`/`0x80B2E0D4`
  tables), `0x80A7B7C0` (the contact callback gated on the `+0x148` slot) and
  the two 48-byte `0x802DCBFC` destructor forwarders `0x80A7F690`/`0x80A7F6C0`.

### `qr3` sites

Two data `qr3` sites are ported, the same unsigned-halfword widen pinned by
`docs/REL_ELF_MAPPING.md` (`GQR3 = 0x00050005`, `W = 1`, `I = 3`):

| address | sites | port |
|---|---|---|
| `0x80A88BC0` | `0x80A88C50`, `0x80A88C70` | `w142_qr3_u16(this+0xDDC+i*2)` twice, once for the cosine and once for the sine |

Every other `psq_*` in the 16 rows is a `qr0` `f29`/`f30`/`f31` stack spill and
is not ported.

### Specification corrections made while porting

- **`0x80A53850` stage surface offset.** `docs/SPEC_WAVE148_ENEMIES.md` prints
  `*(u32 *)0x80429DD0 + 0x8FEF0` in both its behaviour list and its constants
  table. The raw stream at `0x80A538C0..0x80A538D0` is
  `addis r3,r3,9` / `lbz r0,0x2C(r3)` / `lfs f1,-0x13C(r3)`, i.e.
  `0x90000 - 0x13C = 0x8FEC4`; the Ghidra export agrees. The port reads
  `+0x8FEC4` and says so in its doc block. The `0x9002C` mode byte is correct in
  the specification and the export.
- **`0x80A94100` counter clamp.** The specification writes `if (n > 8)`; the raw
  test is `if (7 < n)`. The two forms differ only for `n == 8`, where both are
  a no-op, and the port keeps the raw `> 7` form.
- **`0x80A5C660` float operands.** The export's `extraout_f1` is confirmed to be
  a decompiler artefact: the stream carries `lfs f1,0x64(r31)` before
  `0x80267420` and `lfs f1,0x70(r31)` before `0x802674C0`, so the port passes
  those two fields explicitly, as the specification states.
- **`0x80A7B7C0` call arity.** Ghidra prints `0x800964A0()` and `0x80095C80()`
  with no arguments. The raw stream shows `r6 = 0` before the type query, so the
  query is `0x800964A0(this, slot, report, 0)`, and the base response is
  `0x80095C80(this, slot, report)`.

### Two `void` callees whose dropped return is recovered locally

Two already-translated bodies are typed `void` because the export puts no return
on them, yet their callers in this lane consume the word the callee leaves in
`r3`. Rather than change another lane's public signature, this pass adds two
private helpers that reproduce the callee and keep the value:

- `w149_aim` reproduces `0x80AB1D60` (`fn_80ab1d60`) and returns the
  `0x8015F4F0` "arrived" flag that `0x80AB26A0` counts.
- `w149_place` reproduces `0x80A7AB30` (`fn_80a7ab30`) and returns the placement
  verdict that `0x80A7AB90` tests against `-1/0/1`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A53850` | `fn_80a53850` | 368 | medium |
| `0x80A5C660` | `fn_80a5c660` | 436 | high |
| `0x80A5EB00` | `fn_80a5eb00` | 448 | high |
| `0x80A7AB90` | `fn_80a7ab90` | 272 | medium |
| `0x80A7B7C0` | `fn_80a7b7c0` | 184 | medium |
| `0x80A7F690` | `fn_80a7f690` | 48 | medium |
| `0x80A7F6C0` | `fn_80a7f6c0` | 48 | medium |
| `0x80A81150` | `fn_80a81150` | 456 | high |
| `0x80A84A00` | `fn_80a84a00` | 400 | high |
| `0x80A88BC0` | `fn_80a88bc0` | 368 | high |
| `0x80A94100` | `fn_80a94100` | 408 | high |
| `0x80AA0530` | `fn_80aa0530` | 388 | high |
| `0x80AB26A0` | `fn_80ab26a0` | 408 | high |
| `0x80AB29C0` | `fn_80ab29c0` | 376 | high |
| `0x80AE69E0` | `fn_80ae69e0` | 444 | high |
| `0x80B058A0` | `fn_80b058a0` | 476 | high |

### Why five rows are `medium`

- **`0x80A53850`** - every store, branch, constant and call argument is literal,
  and the surface offset is pinned by the raw `addis`/`-0x13C` pair, but the
  *roles* of `0x80429DD0 + 0x9002C` (stage mode) and `+0x8FEC4` (surface height)
  are inferred from the `== 3` gate and the max/replace arithmetic; nothing in
  this REL reads or writes either field.
- **`0x80A7AB90`** - the tables, the `+1 & 1` phase toggle and the `-1/0/1`
  acceptance window are literal; the accepted verdict is the word left in `r3`
  by `0x80A7AB30`, and both placement helpers it calls (`0x80A7A8C0`,
  `0x80A7AA00`) are still untranslated.
- **`0x80A7B7C0`** - both call shapes were read from the raw stream and the
  `+0x148` equality and two kinds are literal; the meaning of the two type
  values is inferred.
- **`0x80A7F690` / `0x80A7F6C0`** - the visible `0x802DCBFC` call is exact, but
  the stub's hidden `r12` this-adjustment cannot be modelled, the same limit
  every other forwarder of this pattern in the file carries.

## Deliberate skips

- **Three `0x80A70000` constructions were dropped as duplicates.** `0x80A73260`,
  `0x80A76450` and `0x80A7F6F0` are untranslated as `fn_<addr>` tokens but are
  already present in `ledger/functions.csv` under their descriptive names
  `alloc_hanachan_actor`, `alloc_himan_bros_actor` and `alloc_icicle_actor`
  (`src/game/enemies.rs:6439`, `:7643`, `:7696`). Bodies had been written for
  all three before the lane validator reported the duplicate ledger rows; they
  were removed rather than shipped twice. The bucket is therefore covered except
  for the four large rows below.
- **`0x80A729E0` (1228 bytes), `0x80A75800` (2184), `0x80A7BEC0` (1464) and
  `0x80A7EF40` (1576).** The four remaining untranslated `0x80A70000` rows. Each
  is a multi-screen body whose sub-object layouts and unledgered helpers are not
  established from the export alone; left for a later pass rather than guessed.
- **`0x80A04D10` (360 bytes) and `0x80A334D0` (352 bytes)**, the two cleanest
  rows below the `docs/SPEC_WAVE148_ENEMIES.md` size cut, were evaluated and
  left alone: their first calls land on `0x80A048D0`, `0x80A33630`,
  `0x80A337E0` and `0x80A343B0`, none of which is translated, so the callee
  contracts that carry their state would be pure invention.
- **`0x80A7F6F0`'s two 48-byte twins were still claimed** (`0x80A7F690`,
  `0x80A7F6C0`): unlike the constructors above they have no ledger row and no
  named body anywhere in `src/`.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 16 sizes sum to 5,528
  bytes (4,976 for the twelve specification rows, 552 for the four bucket rows).
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; all 16 addresses were absent from
  `ledger/functions.csv` before this pass, and the three that were not are
  listed under "Deliberate skips".
- Every `# Original` block carries the `Synthetic base: 0x80A00000` line.
- The two data `qr3` sites of `0x80A88BC0` are ported as `(f32)(u16)`; every
  other `psq` in these rows is a `qr0` spill.
- The module header was updated: cluster item 131, the cluster count (130 to
  131), the function totals (3415 to 3431 public translations, 3344 to 3360
  `fn_<addr>` bodies), the "start here" list and the records list.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave149` reports 16 rows for this lane, 16
  unique addresses and `OK`.
