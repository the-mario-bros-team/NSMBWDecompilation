# Coverage - Wave 72 (runtime::dvd, the `0x801D0000` settings-store band)

## How to read this file

Wave 72 is a four-lane wave; this is the `runtime::dvd` lane. The bucket is the
whole `0x801D0000` 64 KiB page of `main.dol`. That page is wider than the DVD
band (`0x801CA640`-`0x801D6780`), and the 126 translated addresses it already
held are spread across the drive, ioctl, heap and DSP code. This pass therefore
takes one self-contained run of **non-disc** code from the high end of the page
— the settings-store API at `0x801DC6A0`-`0x801DDAB0` — and writes it into
`src/runtime/dvd.rs`, the only writable file assigned to the bucket. Every doc
block says plainly which routine it is.

The store is a small key/value area: a directory of 16-bit offsets that grows
downwards from `0x803962E0`, a byte budget, and a dirty flag, all held in `r13`
small-data words. Records are TLVs decoded by `0x801DC100`: the top three bits
of the first byte are the value width (`0x60`/`0xE0` = 1, `0x80` = 2, `0xA0` =
4, `0xC0` = 8, `0x40`/`0x20` = variable) and the low five bits are the item id.
The two variable-length entry points recover their `(buffer, len)` pair from
Ghidra's model of the Metrowerks register-save helpers `0x802DCD60`/`0x68`/
`0x74`; that is why those three bodies are `medium` and the rest are `high`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/dvd.rs` | `runtime::dvd` | 30 | 2728 | 23 | 7 |

## What this pass covers

- Store core `0x801DC6A0`-`0x801DCB50` (6 bodies, 1412 bytes). The
  variable-length reader/writer pair ([`fn_801dc6a0`]/[`fn_801dc780`]) and the
  typed entry points around them: the two one-byte readers
  ([`fn_801dc8b0`], [`fn_801dc990`], identical bodies), the four-byte reader
  [`fn_801dca70`] and the one-byte writer [`fn_801dcb50`]. All five take the
  `0x801B1140`/`0x801B1180` interrupt lock, resolve the item through the
  directory, decode it with `0x801DC100` and act on the descriptor's `+8` type,
  `+9` variable marker, `+0x10` length and `+0x18` value pointer. The writer
  paths compare before copying and set the dirty byte `r13-0x4C1B`; when the
  stored record does not match they unlink it (`0x801DC280`) and append a fresh
  record (`0x801DC420`).
- Typed item accessors `0x801DD320`-`0x801DD6E0` (16 bodies, 856 bytes). Four
  clamp/default getters over `0x801DC8B0`/`0x801DCA70` (items `0x0B`, `0x0E`,
  `0x0F`, `0x11`, `0x1E`, `0x1F`, `0x20`, `0x21`, plus the `0x0B49D800` default
  for item `0`) and twelve two-call stubs over the variable pair: item `0x1C`
  at `0x461` bytes, item `0x1D` at `0x205` bytes and item `0x0C` at `0x16`
  bytes, each with a read and a write entry, plus the `0x1F`/`0x20` byte
  setters. The constants line up with the item ids the getters use, which is
  what fixes the `(buffer, len, id)` argument order.
- Text-block scanners, boot flag and probes `0x801DD860`-`0x801DDA80` (8
  bodies, 460 bytes). The three scanners run the key lookup `0x801DD6F0` over
  the fixed 256-byte text block at `0x80003800` and then stride a table of
  `id`+code records with `strcmp` (`0x802E1AB4`): five-byte records at
  `0x80343A08` for a four-byte value and four-byte records at `0x80343A50` for
  a three-byte value, both terminated by id `0xFF`; the third returns the
  seventeen-byte value block at `0x8039E2E0` instead of an id. Alongside them
  sit the `0x800030E3` bit-6 read-modify-write [`fn_801dd9d0`] and the four
  small-data probe words `0x801DDA50`-`0x801DDA80` that `src/runtime/input.rs`
  already calls by address as the classic-controller sub-id tests.
- The three runs are contiguous and share the descriptor field offsets, so
  adjacent bodies confirm each other (`fn_801dc8b0` with `fn_801dca70`,
  `fn_801dd320` with `fn_801dd610`, `fn_801dd860` with `fn_801dd940`).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801DC6A0` | `fn_801dc6a0` | 224 | medium |
| `0x801DC780` | `fn_801dc780` | 292 | medium |
| `0x801DC8B0` | `fn_801dc8b0` | 212 | high |
| `0x801DC990` | `fn_801dc990` | 212 | high |
| `0x801DCA70` | `fn_801dca70` | 212 | high |
| `0x801DCB50` | `fn_801dcb50` | 260 | medium |
| `0x801DD320` | `fn_801dd320` | 108 | medium |
| `0x801DD390` | `fn_801dd390` | 84 | high |
| `0x801DD3F0` | `fn_801dd3f0` | 84 | high |
| `0x801DD450` | `fn_801dd450` | 84 | high |
| `0x801DD4B0` | `fn_801dd4b0` | 64 | high |
| `0x801DD4F0` | `fn_801dd4f0` | 12 | high |
| `0x801DD500` | `fn_801dd500` | 12 | high |
| `0x801DD510` | `fn_801dd510` | 12 | high |
| `0x801DD520` | `fn_801dd520` | 12 | high |
| `0x801DD530` | `fn_801dd530` | 104 | high |
| `0x801DD5A0` | `fn_801dd5a0` | 84 | high |
| `0x801DD600` | `fn_801dd600` | 8 | high |
| `0x801DD610` | `fn_801dd610` | 84 | high |
| `0x801DD670` | `fn_801dd670` | 84 | high |
| `0x801DD6D0` | `fn_801dd6d0` | 8 | high |
| `0x801DD6E0` | `fn_801dd6e0` | 12 | high |
| `0x801DD860` | `fn_801dd860` | 132 | medium |
| `0x801DD8F0` | `fn_801dd8f0` | 72 | medium |
| `0x801DD940` | `fn_801dd940` | 132 | medium |
| `0x801DD9D0` | `fn_801dd9d0` | 92 | high |
| `0x801DDA50` | `fn_801dda50` | 8 | high |
| `0x801DDA60` | `fn_801dda60` | 8 | high |
| `0x801DDA70` | `fn_801dda70` | 8 | high |
| `0x801DDA80` | `fn_801dda80` | 8 | high |

## Deliberate skips

- `0x801DDA90`, `0x801DDAA0`, `0x801DDAB0` (8 bytes each): three
  constant-return stubs whose entire export is `return 0`. They close out the
  probe run that `0x801DDA50`-`0x801DDA80` begins, but have no observable
  behaviour and no caller inside this bucket, so they were left for a pass that
  can name the probe chain's role.
- `0x801D3260`, `0x801D3270`, `0x801D36D0`, `0x801D3870` (12, 12, 12 and 8
  bytes): the four sub-16-byte stubs that touch the already-translated
  `0x801D3280`/`0x801D3700`/`0x801D3880` routines. They were not read in this
  pass; the pass stayed inside the settings band instead of starting a second
  cluster it could not finish.
- `0x801D6850`-`0x801D6F60` (36, 52, 660, 940 and 64 bytes): the DSP/AX task
  band immediately above the translated `0x801D6780`. Untouched.
- `0x801D82E0`-`0x801DC100`: the large untouched block between the DSP band and
  the settings store. Its ownership is not established, so no body was written
  there.
- `0x801DE010` (2296 bytes) and `0x801DEB70` (2524 bytes): still the two largest
  routines in the page and still need a frame-by-frame read.
- `0x801DFAC0`-`0x801DFB30` (seven 4-byte entries): too short to contain more
  than a return; not read.
- No address already present in `ledger/functions.csv` was re-claimed. The 126
  previously translated `0x801D` addresses are untouched; 107 addresses remain
  untranslated in the bucket after this pass.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (30 rows, 2728 bytes total).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/dvd.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error or warning naming
  `src/runtime/dvd.rs`; the four outstanding errors in the workspace are all in
  the parallel `src/game/bases.rs` lane.
- `node tools/check-lane-csv.mjs wave72` prints `OK` for
  `ledger/agents/wave72-dvd.csv` (30 rows, 30 unique addresses, 23 high and 7
  medium).
