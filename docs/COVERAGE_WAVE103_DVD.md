# Coverage - Wave 103 (runtime::dvd, the `0x801D0000` page: byte-stream decoders, ISFS paths, the settings store and the SYSCONF mount)

## How to read this file

The `runtime::dvd` bucket is the whole `0x801D0000` 64 KiB page of `main.dol`,
which is wider than the disc band. Everything below is SDK code that happens to
share the page, and `src/runtime/dvd.rs` is the only writable file assigned to
the bucket, so it is reconstructed there. Each body in that file states which
routine it is, which fields it touches and how confident the reconstruction is;
this document only summarises the pass.

`high` means every store, mask and call in the body is literal and its role is
proven; `medium` means the instruction sequence is literal but at least one
field role, callee identity or recovered argument is inferred; `low` means only
the store/instruction sequence is certain.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/runtime/dvd.rs` | `runtime::dvd` | 16 | 5140 | 0 | 16 | 0 |

## What this pass covers

The bucket held 22 untranslated addresses when this pass started. Sixteen of
them are translated here, in five groups spread over
`0x801D68C0`-`0x801DFB30`; none is disc code.

- **Byte-stream decoders, `0x801D68C0`-`0x801D6B60` (2 bodies, 1600 bytes).**
  These are the consumers of the two state constructors that Waves 91/98
  already translated ([`fn_801d6850`], [`fn_801d6880`]). The `0x30`-tagged
  stream is a literal/fill coder: a 16-bit header accumulates in `+0x04`, each
  control byte selects a run of `count + 1` literal bytes (bit 7 clear) or
  `count + 3` copies of one byte (bit 7 set), and the length limit in `+0x08`
  clamps the run. The `0x10`-tagged stream adds overlapping back-references: a
  three-step code in `+0x10` builds a word whose low nibble is the high byte of
  the distance and whose upper bits are the length.
- **ISFS paths and the save path, `0x801DA370`-`0x801D9950` (2 bodies, 1264
  bytes).** [`fn_801da370`] is the recursive resolver that
  [`fn_801da6e0`]/`0x801DAB90` call: it splits the relative path at the first
  slash and handles `.`, `..`, a leading slash and a plain component, joining
  with one of two small-data formats. [`fn_801d9950`] is the save routine: it
  reads and recreates the directory attributes, opens the source descriptor,
  builds a `/tmp/sys` destination name and copies through it.
- **Settings store, `0x801DBEB0`-`0x801DC420` (3 bodies, 1620 bytes).** The
  image validator turns a staged `0x4000`-byte SYSCONF block into the record
  directory that the Wave 72 readers use: it checks the head/tail markers,
  walks the offset table with [`SETTINGS_PARSE_RECORD`], zeroes the 0x48-byte
  trailer and resolves the 0x25 known names into `0x803962E0 - 2*index + bias`.
  The unlink and append bodies are its inverse: they move the record area, fix
  the 16-bit offsets that grow downwards from [`SETTINGS_DIR_BASE`], maintain
  the `+0x04` count, the byte budget at `r13 - 0x4C2C` and the dirty byte.
- **SYSCONF mount, `0x801DCC70`-`0x801DCE90` (2 bodies, 628 bytes).**
  [`fn_801dcc70`] is the entry: it takes the interrupt lock, installs the
  callback, and either reports immediately (nothing dirty) or copies the
  compact store into `0x8039A2E0` and starts the asynchronous open.
  [`fn_801dce90`] is the ten-state machine that the ISFS operations call back
  into; it ends in the shared tail that stores status `2` on failure or reports
  [`SYSCONF_STATUS`] to the callback and writes it into `r13 - 0x4C30`.
- **WUD thunks, `0x801DFAC0`-`0x801DFB30` (7 bodies, 28 bytes).** Seven
  four-byte branches into the `0x801F3xxx`/`0x801F8xxx` band. Each is modelled
  as the tail call Ghidra prints for the inlined target, because a four-byte
  function can only be a branch and the register file is unchanged.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801D68C0` | `fn_801d68c0` | 660 | medium |
| `0x801D6B60` | `fn_801d6b60` | 940 | medium |
| `0x801DA370` | `fn_801da370` | 316 | medium |
| `0x801D9950` | `fn_801d9950` | 948 | medium |
| `0x801DBEB0` | `fn_801dbeb0` | 584 | medium |
| `0x801DC280` | `fn_801dc280` | 404 | medium |
| `0x801DC420` | `fn_801dc420` | 632 | medium |
| `0x801DCC70` | `fn_801dcc70` | 540 | medium |
| `0x801DCE90` | `fn_801dce90` | 88 | medium |
| `0x801DFAC0` | `fn_801dfac0` | 4 | medium |
| `0x801DFAD0` | `fn_801dfad0` | 4 | medium |
| `0x801DFAF0` | `fn_801dfaf0` | 4 | medium |
| `0x801DFB00` | `fn_801dfb00` | 4 | medium |
| `0x801DFB10` | `fn_801dfb10` | 4 | medium |
| `0x801DFB20` | `fn_801dfb20` | 4 | medium |
| `0x801DFB30` | `fn_801dfb30` | 4 | medium |

## Notes on the recovered arguments and stores

- **Prologue pairs.** `0x802DCD5C`/`0x802DCD64`/`0x802DCD6C` appear in the
  exports as fake 64-bit returns. As in the Wave 91/98/100 sections, the high
  word is the first incoming argument (`r3`) and the low word the second
  (`r4`); the bodies take them as ordinary parameters. `0x802DCD98`-`0x802DCDBC`
  are the matching restore epilogues and their argument is the result.
- **Store layout.** [`SETTINGS_DIR_BASE`] (`0x803962E0`) is the origin of a
  0x4000-byte area. The record data grows upwards from it and the 16-bit
  directory offsets grow downwards from it, so entry `id` lives at
  `base - 2*id + bias` and record `id` at `base + entry[id]`. The end-offset
  table sits at `base + 6 + 2*id`; the append body writes the new end at
  `base + 8 + 2*count`, which is the literal address the export prints.
- **Zero-length guards.** Both decoders return `-1`/`-3`/`-4` exactly where the
  export does, including the "trailing partial word" case where the header
  prefill stopped with bits still pending.
- **`/tmp/sys` sequence word.** `r13 - 0x4C48` is incremented under the
  interrupt lock before the scratch name is formatted. The `// inferred:` format
  is `%s.%08x.%s` because it receives the directory, the counter and the base
  name.
- **WUD targets.** The seven thunks branch into the `0x801F3xxx` band, which is
  outside this bucket. The bodies call the documented target address rather
  than re-implementing the target's code.

## Deliberate skips

Six addresses in the bucket were not claimed:

- `0x801DA990` (204): the timestamp/sleep loop. Its `0x802DCD70` prologue pair
  is compared with the constant `2`, the tick loop calls `0x801B5F80` with a
  value the export invents (`quotient - 500`) and no caller is present in the
  exports, so both the argument list and the loop's guard would be guesses.
- `0x801DB2B0` (248) and `0x801DB3B0` (1156): the NAND error-log reporter and
  its completion. The reporter republishes eight floating and eight integer
  variadic save slots through a `va_list` descriptor (`0x02000000`, stack
  pointer, register-save pointer); the completion then formats those
  uninitialised slots with a nine-operand `sprintf`. The save-area layout is not
  recoverable from the export, so any body would invent the argument plumbing.
- `0x801D6F60` (2036): the third byte-stream decoder. Its inner step is the
  64-bit shift helper `0x802DD210` and the loop is terminated by a test on the
  sign bit of the shifted high word; reproducing 250 lines of that bit
  manipulation without knowing the test's meaning would be unreviewable.
- `0x801DE010` (2296): the record comparer. It compares roughly forty field
  pairs using the compiler's branchless `|x| > c` idiom (`(x ^ c) >> 1 -
  ((x ^ c) & x)`), and the roles of those fields are not established anywhere in
  the exports.
- `0x801DEB70` (2524): the WC24 reconnect task. It reads twenty-odd small-data
  words whose meaning is unknown and its ring-index updates decompile to
  `(char)~((x - y) >> 24 | (y - x) >> 24) >> 7`, which cannot be written down
  without guessing the intended modulo arithmetic.

## Verification

- Every address and byte size matches
  `work/ghidra/main-gb/export/functions.tsv` (16 rows, 5140 bytes, 0 high,
  16 medium, 0 low).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_<name>.c` path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/dvd.rs`.
- No address was already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave103` reports `OK` for
  `ledger/agents/wave103-dvd.csv`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/runtime/dvd.rs`.
