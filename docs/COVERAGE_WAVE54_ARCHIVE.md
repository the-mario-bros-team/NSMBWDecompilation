# Coverage - Wave 54 (resource::archive, 0x802B0000 bucket)

## How to read this file

Every row names one Ghidra function by its shipped-image address, the Rust body
that replaces it in `src/resource/archive.rs`, its exact size from
`work/ghidra/main-gb/export/functions.tsv`, and how confident the
reconstruction is. `address` is a virtual address in `sys/main.dol` (image base
`0x80004000`), not a file offset. The 28 bodies are appended after the wave-49
section at the end of `src/resource/archive.rs`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/archive.rs` | `resource::archive` | 28 | 1128 | 26 | 2 |

The file now holds 236 translated bodies: 208 before this pass plus the 28
below. The assigned `0x802B0000` bucket still has 151 untranslated functions of
306; 127 were already translated before this pass and 28 are added here.

## What this pass covers

The `0x802B0000` bucket is mixed layout / resource / controller code. Wave 40
took the archive-owned U8, LZ-binding and EGG-root-heap blocks; wave 46 took the
RLYT header accessors. This pass takes the remaining small helpers that read the
section table's count word and base pointer, plus the entry getters around them:

1. **Section-offset queries** - `0x802B3300..0x802B3570` (11 bodies). Every one
   reads the packed count word at `+0x38` and the base pointer at `+0x40` and
   returns `base + sum(count * stride)`. The strides are `0x1C` (28-byte
   entries), `0x14` (20-byte entries) and `4` (word entries). Each function adds
   one more group to the previous one, which is why they form a contiguous run.
2. **Section fill helper** - `0x802B35E0` (1 body). Copies `0x14` bytes into
   `+0x44` through `0x802E19F8` and clears the byte at `+0x58`. It shares the
   same object as cluster 1.
3. **Guarded entry getters** - `0x802B5AF0..0x802B5CA0` (6 bodies). A four-word
   clearer, then five getters that defer to two child pointers at `+0x08` /
   `+0x0C` and return 0 when the child is absent.
4. **Ring and stride helpers** - `0x802BC210`/`0x802BC230` (2 bodies). A
   round-up-times-count row size and a circular-list membership test.
5. **Child accessors and flag getters** - `0x802BC8C0..0x802BC920` (6 bodies)
   and `0x802BF9B0`/`0x802BF9C0` (2 bodies). A two-word block copy, a
   float-to-double getter, a signed-byte getter, two single-flag calls into
   `0x801E14E0`, one child factory call, and two bit getters over `+0x6C`.

### Shared layout of the section-table cluster

`// inferred:` the object is a layout/resource section table. The exports prove
only the offsets and the arithmetic:

- `+0x38` packs small counts (four bits at 28..31, four bits at 24..27, and
  one-bit or two-bit fields around 3..6 and 12..21).
- `+0x40` is the base of the section arrays.
- Each function multiplies one masked count by `0x1C`, `0x14` or `4` and adds
  the result to the base. The record contents are never touched here, so the
  doc blocks do not claim what the records mean.

The 0x1C and 0x14 strides suggest 28-byte and 20-byte records; that reading is
marked `// inferred:` and no field name is invented.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802B3300` | `fn_802b3300` | 8 | high |
| `0x802B3310` | `fn_802b3310` | 24 | high |
| `0x802B3330` | `fn_802b3330` | 36 | high |
| `0x802B3360` | `fn_802b3360` | 44 | high |
| `0x802B3390` | `fn_802b3390` | 52 | high |
| `0x802B33D0` | `fn_802b33d0` | 60 | high |
| `0x802B3410` | `fn_802b3410` | 68 | high |
| `0x802B3460` | `fn_802b3460` | 76 | high |
| `0x802B34B0` | `fn_802b34b0` | 84 | high |
| `0x802B3510` | `fn_802b3510` | 92 | high |
| `0x802B3570` | `fn_802b3570` | 100 | high |
| `0x802B35E0` | `fn_802b35e0` | 60 | medium |
| `0x802B5AF0` | `fn_802b5af0` | 24 | high |
| `0x802B5C20` | `fn_802b5c20` | 28 | high |
| `0x802B5C40` | `fn_802b5c40` | 32 | high |
| `0x802B5C60` | `fn_802b5c60` | 32 | high |
| `0x802B5C80` | `fn_802b5c80` | 28 | high |
| `0x802B5CA0` | `fn_802b5ca0` | 32 | high |
| `0x802BC210` | `fn_802bc210` | 20 | high |
| `0x802BC230` | `fn_802bc230` | 52 | high |
| `0x802BC8C0` | `fn_802bc8c0` | 20 | high |
| `0x802BC8E0` | `fn_802bc8e0` | 8 | high |
| `0x802BC8F0` | `fn_802bc8f0` | 12 | high |
| `0x802BC900` | `fn_802bc900` | 12 | high |
| `0x802BC910` | `fn_802bc910` | 12 | high |
| `0x802BC920` | `fn_802bc920` | 88 | medium |
| `0x802BF9B0` | `fn_802bf9b0` | 12 | high |
| `0x802BF9C0` | `fn_802bf9c0` | 12 | high |

## Unknowns

1. **Section record meanings.** The strides are literal but the record layouts
   are not visible in these functions. `+0x38`/`+0x40` are described by offset
   only.
2. **The `0x802E19F8` callee.** `fn_802b35e0` passes `(dst, src, 0x14)`, which
   looks like a block copy, but the export never shows the callee body. The
   confidence is `medium` for that reason.
3. **The `0x802BDD00` argument.** `fn_802bc920`'s export prints
   `FUN_802bdd00()` with no arguments, yet that function returns its first
   argument and the value is immediately stored at `+0xB48`. The allocation is
   forwarded as that argument and marked `// inferred:`.
4. **The `0x801E14E0` role.** `fn_802bc900`/`fn_802bc910` forward a handle and
   the literal 0/1; the callee's meaning is not visible here, so the doc blocks
   call it by address.

## Deliberate skips

- **`0x802B2980`, `0x802B29B0`, `0x802B2A20..0x802B2B50`** - one-line 4-byte /
  16-byte copy and `a + b` template instantiations. They are real functions but
  add no distinct field cluster and would pad the lane.
- **`0x802BC980`, `0x802BC9A0`, `0x802BC9C0`** - guard on `+0xB48` then a call
  printed with no arguments, while the callees (`0x802BDF70`, `0x802BDFC0`)
  declare four parameters. Argument forwarding cannot be reconstructed
  honestly, so these are left for a pass with call-site register evidence.
- **`0x802B3620..0x802B3940`** - variable-stride table resize and default-fill
  loops. They are contiguous with the size queries but each needs its own
  field-by-field check; they are left to a dedicated pass rather than rushed.
- **`0x802B0680`, `0x802B1D60`, `0x802B1E70`, `0x802B20B0`, `0x802B2C70`,
  `0x802B6C80`** - long parsers and float matrix normalisers (1376-1844 bytes).
  They need more than a small-helper pass.
- **`0x802B9700`** - a four-byte thunk to `0x801D4A90`, not translated.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (read from the TSV, not from prose).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/802bxxxx_FUN_802bxxxx.c` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/archive.rs` with a complete `# Original` / `# Arguments` /
  `# Returns` / `# Side effects` / `# Safety` / `# Confidence` doc block.
- No address was already in `ledger/functions.csv`; the 28 addresses are all
  from the untranslated part of the `0x802B0000` bucket.
- `node tools/check-lane-csv.mjs wave54` reports
  `wave54-archive.csv: 28 rows`, 28 unique addresses and `OK`.
- `cargo check --message-format=short` reports no error and no diagnostic for
  `src/resource/archive.rs`.
