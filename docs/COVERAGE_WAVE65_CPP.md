# Coverage - Wave 65 (runtime::cpp)

## How to read this file

Each row is one `pub unsafe fn fn_<address>` body appended to
`src/runtime/cpp.rs` by this pass. The `bytes` column is the exact size from
`work/ghidra/main-gb/export/functions.tsv`; the body's `# Original` block cites
the same file. Confidence is the same value as the body doc block.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/cpp.rs` | `runtime::cpp` | 27 | 2200 | 20 | 7 |

## What this pass covers

- the resource-view predicate twins `0x802D3F60/4030/4100`: the `+0x4CC`
  state guard (states 2/3), the `+0x4C8` default mode, the `+4` busy byte and
  the busy flag `+5` around the `0x80279B70` / `0x80279CE0` workers.
- the view reset and list loops `0x802D41D0`, `0x802D41F0`, `0x802D4250`
  (`0x802783C0` reset; `0x80278780` lookup then `0x8027A7D0(entry, state)`).
- the three archive lookup wrappers `0x802D43C0/44B0/45A0`
  (`0x8027B1C0/270/160` on `handle + 0xC`, returned as `result == 0`).
- the archive-object lifecycle `0x802D46A0` (element destructor),
  `0x802D46E0` (0x8AC-byte constructor: voice base, 0x6C-stride array at
  `+0x6F4`, map at `+0x8AC`, vtables at `+0`, `+8`, `+0xF8`) and `0x802D4780`
  (mirror teardown through `0x802D4F60`, `fn_802dca98`, `0x80229030`,
  `fn_802d6350`).
- the BRRES section-tag record `0x802D77E0..0x802D7D50`: the packed-header
  splitter `0x802D7820`, the six 8-byte setters `0x802D78A0..0x802D7900`,
  the tag-to-index lookup `0x802D7910` over `0x802FD2D0`, the entry accessor
  `0x802D7A80`, the constructor `0x802D7AA0`, the deleting destructor
  `0x802D7B20` and the two `+0x74` child-reset twins `0x802D7C50/7D50`.
- new constants `BRRES_TAG_TABLE` (`0x802FD2D0`) and `BRRES_CHILD_SENTINEL`
  (`0x8042AFC8`, the `r13 - 0x46B8` sentinel already used by
  `fn_802d85c0`); no existing constant was renamed or reused.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802D3F60` | `fn_802d3f60` | 204 | medium |
| `0x802D4030` | `fn_802d4030` | 204 | medium |
| `0x802D4100` | `fn_802d4100` | 204 | medium |
| `0x802D41D0` | `fn_802d41d0` | 20 | high |
| `0x802D41F0` | `fn_802d41f0` | 92 | high |
| `0x802D4250` | `fn_802d4250` | 104 | high |
| `0x802D43C0` | `fn_802d43c0` | 52 | high |
| `0x802D44B0` | `fn_802d44b0` | 52 | high |
| `0x802D45A0` | `fn_802d45a0` | 52 | high |
| `0x802D46A0` | `fn_802d46a0` | 64 | high |
| `0x802D46E0` | `fn_802d46e0` | 152 | medium |
| `0x802D4780` | `fn_802d4780` | 168 | medium |
| `0x802D77E0` | `fn_802d77e0` | 64 | medium |
| `0x802D7820` | `fn_802d7820` | 60 | high |
| `0x802D78A0` | `fn_802d78a0` | 8 | high |
| `0x802D78B0` | `fn_802d78b0` | 8 | high |
| `0x802D78C0` | `fn_802d78c0` | 8 | high |
| `0x802D78D0` | `fn_802d78d0` | 8 | high |
| `0x802D78E0` | `fn_802d78e0` | 8 | high |
| `0x802D78F0` | `fn_802d78f0` | 8 | high |
| `0x802D7900` | `fn_802d7900` | 8 | high |
| `0x802D7910` | `fn_802d7910` | 132 | high |
| `0x802D7A80` | `fn_802d7a80` | 20 | high |
| `0x802D7AA0` | `fn_802d7aa0` | 116 | medium |
| `0x802D7B20` | `fn_802d7b20` | 92 | high |
| `0x802D7C50` | `fn_802d7c50` | 140 | high |
| `0x802D7D50` | `fn_802d7d50` | 152 | high |

## Deliberate skips

- `0x802D79A0` - its body opens with `FUN_802dcd6c()`, one of the
  register-save stubs whose fake 64-bit return Ghidra unpacks into the real
  arguments; the object pointer and index cannot be recovered without guessing
  (same reason `0x802DA040/6F0`, `0x802DAD20` were skipped).
- `0x802D7860` - calls `FUN_802d79a0()` with no visible arguments for the same
  reason, so its single argument is unrecoverable from this evidence.
- `0x802D7410/7570` - both consume the fake 64-bit return of `0x802DCD74` and
  read `r13` slots (`-0x46D8/-0x46D4`); excluded with the other
  save-register-stub callers.
- `0x802D7680` - calls `FUN_802d78a0()` with no arguments (register arguments
  lost), so the tag-table walk cannot be reconstructed honestly.
- `0x802D9570`, `0x802DADD0` - paired-single / `ldexpf` artefacts
  (`docs/QUALITY.md` §5.3); no honest floating-point reconstruction.
- `0x802D4830` - reads `r2`-relative float constants (`-15000`, `-0x3AA0`,
  `-0x3A9C`) that the export leaves unqualified; the object it initialises is
  also outside the `+0x4C4` view cluster.
- `0x802D48C0` - consumes the fake `0x802DCD74` return and mixes `r13` slots.
- `0x802DCDD8/802DCEC4/802DCFFC/802DD0E0/802DD1EC/802DD210` - the 64-bit
  divide/modulo/shift family. The exports expand into `CARRY4` chains and
  `CONCAT44`, so a literal Rust body would be a re-guess of the arithmetic,
  not a translation; left for a pass that can cite the raw PPC sequence.
- register-save stubs `0x802DCCCC..0x802DCDBC`, empty `0x802DB0E0` and
  4-byte thunks `0x802D2F10/3260`, `0x802DEBB4` - alternate entry points, not
  callable high-level routines.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/runtime/cpp.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors for `src/runtime/cpp.rs`.
- `node tools/check-lane-csv.mjs wave65` reports `OK` for this CSV (27 rows,
  27 unique addresses).
