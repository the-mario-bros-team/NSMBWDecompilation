# Coverage - Wave 87 (runtime::dvd, the ISFS/NAND file API and the settings-store core in the `0x801D0000` page)

## How to read this file

The `runtime::dvd` bucket is the whole `0x801D0000` 64 KiB page of `main.dol`.
Only the middle of that page is disc code (`0x801CA640`-`0x801D6780`); the rest
is SDK code that happens to share the page. `src/runtime/dvd.rs` is the only
writable file assigned to the bucket, so those routines are reconstructed there
too, and each function doc says plainly which routine it is and which fields it
touches.

This pass continues the Wave 72 settings-store work and opens the ISFS
(internal flash file system) band that sits below it. Neither group is disc
code.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/dvd.rs` | `runtime::dvd` | 30 | 3180 | 18 | 12 |

## What this pass covers

- DVD-band image accessors `0x801D3260`-`0x801D3870` (4 bodies, 44 bytes).
  One-word getters for the `0x95` status image (`0x80395D60`), the `0x96`
  control image (`0x80395D80`), the shared low-level input block
  (`0x80395C60`), plus a constant-zero stub. They sit inside the Wave 40/46
  cover/status/control cluster and complete its read side.
- ISFS wrapper band `0x801D8FD0`-`0x801DAB50` (17 bodies, 2196 bytes). The
  shared state is the word at `r13 - 0x4C40` (`0` idle, `1` mounting,
  `2` ready). The band holds:
  - the readiness predicate [`fn_801da550`] and the error mapper
    [`fn_801da570`], which copies the 0x29-entry `(code, mapped)` table at
    `0x802F6B34` to the stack, maps negative ISFS codes and logs the five
    "interesting" ones (`-0x72`, `-0x74`, `-0x75`, `-0x9`, `-0xC`);
  - the path tests [`fn_801da4b0`]/[`fn_801da4f0`] (`"/shared2"` and
    `"/shared2/"` prefixes) and the path builders [`fn_801da6e0`],
    [`fn_801daa70`], [`fn_801daaf0`];
  - the attribute pack/unpack pair [`fn_801d90d0`]/[`fn_801d9130`], the
    owner/group/other permission encoding of the ISFS attribute byte;
  - the thin operations: sync rename [`fn_801d8fd0`], sync/async attribute
    read [`fn_801d93a0`]/[`fn_801d9410`], sync/async open
    [`fn_801d95b0`]/[`fn_801d9640`], close [`fn_801d9850`], the directory
    continuation submit [`fn_801d98c0`] and the completion trampoline
    [`fn_801dab50`].
- Settings-store core `0x801DC100`-`0x801DD6F0` (9 bodies, 940 bytes). The
  record decoder [`fn_801dc100`] is the body `fn_801dc780`/`fn_801dcb50`
  already call through `SETTINGS_PARSE_RECORD`; it documents the six TLV types
  and the descriptor fields (`+0x0C` header length, `+0x10` payload length,
  `+0x14` name, `+0x18` payload, `+0x1C` record length). Around it sit the
  typed getters for items `1`, `5`, `6` and `9`, the three constant-zero
  probes at `0x801DDA90`-`0x801DDAB0`, and the obfuscated key/value scanner
  [`fn_801dd6f0`] behind `SETTINGS_TEXT_LOOKUP`: a 128-step scan over the
  256-byte block at `0x80003800`, XOR-masked with the rolling constant
  `0x73B5DBFA`, keyed on an `'='` terminator and case-folded with `0xDF`.

The slice is not contiguous; the untranslated gaps are the 0x801D68xx and
0x801D6F60 tails of the DSP/NAND task band, the SLPI/file-system helpers
`0x801D82E0`-`0x801D8D20`, and the SYSCONF drivers `0x801DCC70`/`0x801DCE90`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801D3260` | `fn_801d3260` | 12 | high |
| `0x801D3270` | `fn_801d3270` | 12 | high |
| `0x801D36D0` | `fn_801d36d0` | 12 | high |
| `0x801D3870` | `fn_801d3870` | 8 | high |
| `0x801D8FD0` | `fn_801d8fd0` | 100 | medium |
| `0x801D90D0` | `fn_801d90d0` | 84 | high |
| `0x801D9130` | `fn_801d9130` | 140 | high |
| `0x801D93A0` | `fn_801d93a0` | 100 | medium |
| `0x801D9410` | `fn_801d9410` | 132 | medium |
| `0x801D95B0` | `fn_801d95b0` | 140 | medium |
| `0x801D9640` | `fn_801d9640` | 140 | medium |
| `0x801D9850` | `fn_801d9850` | 108 | medium |
| `0x801D98C0` | `fn_801d98c0` | 136 | medium |
| `0x801DA4B0` | `fn_801da4b0` | 52 | high |
| `0x801DA4F0` | `fn_801da4f0` | 88 | high |
| `0x801DA550` | `fn_801da550` | 20 | high |
| `0x801DA570` | `fn_801da570` | 360 | medium |
| `0x801DA6E0` | `fn_801da6e0` | 200 | medium |
| `0x801DAA70` | `fn_801daa70` | 124 | medium |
| `0x801DAAF0` | `fn_801daaf0` | 84 | medium |
| `0x801DAB50` | `fn_801dab50` | 60 | high |
| `0x801DC100` | `fn_801dc100` | 384 | high |
| `0x801DDA90` | `fn_801dda90` | 8 | high |
| `0x801DDAA0` | `fn_801ddaa0` | 8 | high |
| `0x801DDAB0` | `fn_801ddab0` | 8 | high |
| `0x801DD1D0` | `fn_801dd1d0` | 84 | high |
| `0x801DD230` | `fn_801dd230` | 116 | high |
| `0x801DD2B0` | `fn_801dd2b0` | 84 | high |
| `0x801DD310` | `fn_801dd310` | 12 | high |
| `0x801DD6F0` | `fn_801dd6f0` | 364 | medium |

## Deliberate skips

- `0x801DC280` (404 bytes, record removal) and `0x801DC420` (632 bytes,
  record insert): both are the compaction half of the settings store. The
  removal body walks the 16-bit directory backwards with two overlapping
  counters and a pointer comparison against the record origin; the exact
  index direction is only inferred, and an off-by-one there would silently
  corrupt the store. They stay for a pass with the disassembly in hand.
  Their addresses are already the `SETTINGS_REMOVE_RECORD`/
  `SETTINGS_INSERT_RECORD` call targets used by `fn_801dc780`.
- `0x801DA7B0` (164 bytes, "copy the directory part of a path"): the export's
  backward separator scan leaves the index at `-1` when the path contains no
  `/` and then takes the `strncpy` branch with that count. That cannot be the
  real behaviour, and I could not reconcile it, so no body is claimed.
- `0x801DCC70` (540 bytes) and `0x801DCE90` (540/88 bytes, the SYSCONF mount
  drivers): their state enum (`0x80396298`) is only half established and the
  bulk of each body is a chain of untranslated `0x801D8xxx`/`0x801D9xxx` file
  operations with a file-object layout I have not proven. Not claimed.
- `0x801D84E0`-`0x801D8D20` (13 bodies, the sync/async file cores) are called
  by absolute address from this pass, not translated: without them the
  wrappers' argument roles stay `medium`, and translating the cores is a
  separate, larger cluster.
- `0x801DE010` (2296 bytes) and `0x801DEB70` (2524 bytes): the two large
  remaining routines in the page. Neither was attempted.
- `0x801DFAC0`-`0x801DFB30` (seven 4-byte thunks) and the `0x801D6850`-
  `0x801D6F60` DSP/NAND tail are outside this pass's clusters; the thunks are
  pure jump stubs into REL code.
- No address already present in `ledger/functions.csv` was re-claimed. After
  this pass the `0x801D0000` bucket holds 77 untranslated addresses (107
  before the pass).

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (30 rows, 3180 bytes total, 18 high and 12 medium).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/dvd.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/runtime/dvd.rs`.
- `node tools/check-lane-csv.mjs wave87` prints `OK` for
  `ledger/agents/wave87-dvd.csv`.
