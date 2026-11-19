# Coverage - Wave 47 (`runtime::bt`, `0x80220000` bucket)

## How to read this file

Every row below is one `pub unsafe fn fn_<address>` body in
`src/runtime/bt.rs` whose doc block cites the Ghidra export under
`work/ghidra/main-gb/export/`. Addresses are unrelocated `main.dol` virtual
addresses (image base `0x80004000`). None of this pass is Bluetooth protocol
code: it is the remainder of the address lane that Waves 42 and earlier left
untranslated, kept here because the module already owns the band.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 31 | 2444 | 23 | 8 |

## What this pass covers

- **Word table and string helper** (`0x80224340`-`0x80224390`): the indexed
  reader/writer for the table at `0xCD000000` and a `strnlen`-style bounded
  length. These are the last clean functions of the `0x80224xxx` block.
- **Size-checking constructors** (`0x802278A0`, `0x80227970`): both reject a
  zero or non-32-byte-aligned size with `-101` and otherwise tail-call the
  allocator at `0x80225010` / `0x80225220`.
- **One-method classes** (`0x80229360`-`0x80229820`, 4 bodies): two vtable
  installers (`0x8034CC2C`, `0x8034CC18`) and their identical release tails.
  This finishes the cluster Wave 42 named but never started.
- **0x16C-byte class** (`0x8022A6B0`-`0x8022B680`, 16 bodies): constructor pair
  (vtable `0x8034CD10`), destructor, the `OPEN`/`CLOSE`/`APPEND` wrappers over
  the `0x801D8xxx` array helpers, the range rewind, four guard-protected
  drain-queue helpers around `0x803DA0DC`/`0x803DA0D8`, the four-record vtable
  selector at `0x8022B410`, the 0x22-byte record constructor and its release
  tail, and the mode-to-colour-word query.
- **0x22-byte record family** (`0x8022BB80`-`0x8022C340`, 6 bodies): two record
  constructors, their base destructor path, the two-word setter, the cached
  range resolver at `0x8022BD40`, and the derived class at `0x8022C300` /
  `0x8022C340`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80224340` | `fn_80224340` | 16 | high |
| `0x80224350` | `fn_80224350` | 16 | high |
| `0x80224390` | `fn_80224390` | 44 | high |
| `0x802278A0` | `fn_802278a0` | 28 | medium |
| `0x80227970` | `fn_80227970` | 28 | medium |
| `0x80229360` | `fn_80229360` | 16 | high |
| `0x80229370` | `fn_80229370` | 64 | high |
| `0x80229810` | `fn_80229810` | 16 | high |
| `0x80229820` | `fn_80229820` | 64 | high |
| `0x8022A6B0` | `fn_8022a6b0` | 88 | high |
| `0x8022A9F0` | `fn_8022a9f0` | 112 | medium |
| `0x8022AA60` | `fn_8022aa60` | 112 | medium |
| `0x8022AAD0` | `fn_8022aad0` | 120 | medium |
| `0x8022AB50` | `fn_8022ab50` | 232 | medium |
| `0x8022AEA0` | `fn_8022aea0` | 80 | high |
| `0x8022AEF0` | `fn_8022aef0` | 124 | medium |
| `0x8022B150` | `fn_8022b150` | 8 | high |
| `0x8022B230` | `fn_8022b230` | 96 | high |
| `0x8022B290` | `fn_8022b290` | 120 | high |
| `0x8022B310` | `fn_8022b310` | 112 | high |
| `0x8022B380` | `fn_8022b380` | 64 | high |
| `0x8022B410` | `fn_8022b410` | 164 | high |
| `0x8022B4C0` | `fn_8022b4c0` | 84 | high |
| `0x8022B560` | `fn_8022b560` | 64 | high |
| `0x8022B680` | `fn_8022b680` | 80 | high |
| `0x8022BB80` | `fn_8022bb80` | 84 | high |
| `0x8022BBE0` | `fn_8022bbe0` | 64 | high |
| `0x8022BC20` | `fn_8022bc20` | 12 | high |
| `0x8022BD40` | `fn_8022bd40` | 184 | medium |
| `0x8022C300` | `fn_8022c300` | 60 | high |
| `0x8022C340` | `fn_8022c340` | 88 | high |

## Deliberate skips

These sit in the same bucket and were read, but cannot be reconstructed
honestly from the export:

- `0x802293B0`, `0x80229860` — the two 300-byte companions of the
  `0x80229360`/`0x80229810` classes. Both are `0x28`/`0x2E` dispatchers built
  out of paired-single quantiser state (`unaff_GQR0`) plus an `r2`-relative
  `double` constant; Ghidra also removed two unreachable blocks from each. The
  float rounding cannot be reproduced without guessing the GQR setup.
- `0x8022A590`, `0x8022A620`, `0x8022B5A0` — these read or write statics
  through `unaff_r13` and the small-data base is not established anywhere in
  the visible body, so the target addresses are unknown.
- `0x802242A0`, `0x802242F0`, `0x80224360`, `0x80224370`, `0x80224380` — the
  same `unaff_r13` problem: the neighbouring `0x80224xxx` statics cluster.
- `0x802241B4`, `0x80224244` — reconstructible (a CLZ-based comparator and an
  adjacent-pair swap pass) but deliberately left for the lane that owns the
  surrounding `0x80224xxx` cluster, which is still mostly untranslated.
- `0x8022D3C0`, `0x8022D440`, `0x8022D500`, `0x8022DDE0` — the `0x8022Dxxx`
  scene block. All four depend on `unaff_r2` (the `2^52` double conversion
  constant) or `unaff_r13`, and the virtual accessors they read are not
  translated, so their arithmetic is only half reconstructible.

`0x80228DD0`, `0x80228FF0`-`0x8022A450` were already translated by earlier
waves and were not touched.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn` body in `src/runtime/bt.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` is free of errors for
  `src/runtime/bt.rs`.
- `node tools/check-lane-csv.mjs wave47` reports `OK` for
  `wave47-bt.csv` (31 rows, 31 unique addresses).
