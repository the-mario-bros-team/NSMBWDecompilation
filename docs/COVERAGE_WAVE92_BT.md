# Coverage - Wave 92 (`runtime::bt`, IOS completion-block wrappers, EXI helpers and list/table utilities)

## How to read this file

Every row below is one `pub unsafe fn fn_<address>` body appended to
`src/runtime/bt.rs`. The address is the unrelocated `main.dol` virtual address
(image base `0x80004000`); the byte count comes from
`work/ghidra/main-gb/export/functions.tsv`; the evidence column of
`ledger/agents/wave92-bt.csv` names the Ghidra C export. All addresses are in
the `0x80220000` bucket, which held 216 functions with 86 untranslated when this
pass started (130 `runtime::bt` rows in that bucket were already in
`ledger/functions.csv`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 28 | 6508 | 11 | 17 |

## What this pass covers

- **IOS completion-block wrappers** (`0x80226740..0x80227730`, 16 bodies). Every
  one allocates a `0x140`-byte block from the heap slot at `0x8042ABE4`
  (`r13 - 0x4A9C`, [`SDA_IPC_BLOCK_HEAP`]), writes the device/path name at
  `+0`, then submits a request through the wave-84 builders. The block layout
  the sixteen bodies share:

  | Offset | Field |
  |---:|---|
  | `+0x00` | `(name bytes, NUL)` or the aligned name for kind 4/12 |
  | `+0x40` | second name, or the first vector-table buffer |
  | `+0x46`/`+0x47`/`+0x48`/`+0x49` | four reply bytes read by copy-out mode 3 |
  | `+0x100` | completion callback word |
  | `+0x104` | completion context |
  | `+0x108` | copy-out mode (`0`, `2`, `3`) matching [`fn_802265c0`] |
  | `+0x10C..+0x120` | copy-out pointers for modes 2 and 3 |

  Six of them form synchronous/asynchronous pairs (kinds 3, 4, 6, 7, 8 and 9),
  one is the kind-12 pair reader `0x80227260`, one is the kind-11 eight-byte
  read `0x80227730`, and the last pair (`0x802275A0`/`0x80227670`) reaches the
  open path `0x80224C70`/`0x80224B50` with a copy of the name in the block
  rather than the caller's buffer.
- **EXI probe and transfer layer** (`0x80227B70`, `0x80227C70`,
  `0x802289E8`, `0x80228B0C`, `0x80228C10`, `0x80228CF0`). The probe pair
  publishes the `0x8042ABE8`/`0x8042ABEC`/`0x8042ABF0`/`0x8042ABF4` state words
  after sending the immediate commands `0xF2`/`0xF3` to `0xB0000000`. The four
  transfer helpers share one sequence: set `0xCD006828` to
  `(old & 0x405) | 0xC0`, send the address word with write `1`, poll
  `0xCD006834` bit 0, transfer the data word, poll again and restore the
  `0x405` mask. They call `0x80228644` by address because that routine's own
  export is a dead `while(1)` loop after the decompiler removed every block
  below it as unreachable.
- **Intrusive list and table utilities** (`0x80228DF0`, `0x80228E60`,
  `0x80228F80`, `0x8022C980`, `0x8022C120`, `0x8022C1F0`). The list header is
  `{ head, tail }` at `+0`, a `u16` count at `+8` and the `u16` byte offset of
  the node links at `+0xA`; the nodes are `{ prev, next }` at that offset. The
  append body, the insert-before/append/prepend body and the unlink body cover
  the whole structure. `0x8022C120` resolves an index through one of three
  packed encodings (direct, array, binary-searched `{limit, value}` pairs) and
  `0x8022C1F0` uses it to fill an 18-byte atlas cell descriptor.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80226740` | `fn_80226740` | 244 | medium |
| `0x80226840` | `fn_80226840` | 244 | medium |
| `0x80226940` | `fn_80226940` | 340 | medium |
| `0x80226AA0` | `fn_80226aa0` | 328 | medium |
| `0x80226BF0` | `fn_80226bf0` | 344 | medium |
| `0x80226D50` | `fn_80226d50` | 308 | medium |
| `0x80226E90` | `fn_80226e90` | 216 | high |
| `0x80226F70` | `fn_80226f70` | 208 | medium |
| `0x80227040` | `fn_80227040` | 260 | high |
| `0x80227150` | `fn_80227150` | 260 | high |
| `0x80227260` | `fn_80227260` | 312 | high |
| `0x802273A0` | `fn_802273a0` | 244 | medium |
| `0x802274A0` | `fn_802274a0` | 244 | medium |
| `0x802275A0` | `fn_802275a0` | 200 | high |
| `0x80227670` | `fn_80227670` | 184 | medium |
| `0x80227730` | `fn_80227730` | 208 | high |
| `0x80227B70` | `fn_80227b70` | 252 | medium |
| `0x80227C70` | `fn_80227c70` | 72 | high |
| `0x80228DF0` | `fn_80228df0` | 112 | high |
| `0x80228E60` | `fn_80228e60` | 288 | high |
| `0x80228F80` | `fn_80228f80` | 108 | high |
| `0x8022C980` | `fn_8022c980` | 64 | high |
| `0x802289E8` | `fn_802289e8` | 292 | medium |
| `0x80228B0C` | `fn_80228b0c` | 260 | medium |
| `0x80228C10` | `fn_80228c10` | 224 | medium |
| `0x80228CF0` | `fn_80228cf0` | 224 | medium |
| `0x8022C120` | `fn_8022c120` | 204 | medium |
| `0x8022C1F0` | `fn_8022c1f0` | 264 | medium |

## Why the seventeen `medium` rows are `medium`

- `0x80226740`, `0x80226840`, `0x802273A0`, `0x802274A0` each consume both the
  low half of a `FUN_802dcd5c/64/6c()` prologue pair *and* Ghidra's
  `extraout_r4` byte. The two uses cannot both be the same physical register, so
  the name/tag split is inferred; every load, store and call in the body is
  literal.
- `0x80226940`, `0x80226AA0`, `0x80226BF0`, `0x80226D50`, `0x80226F70`,
  `0x80227670` take their first two arguments from the folded prologue pair.
  The order (high half first) is the one [`fn_80225640`] proves, but which
  argument lands in `r3` is still inferred per body.
- `0x80226940` and `0x80226AA0` also multiply the caller's count by `0xD` for
  the second vector entry. The scale is literal; what the 13-byte records hold
  is not proven.
- `0x80227B70`, `0x802289E8`, `0x80228B0C`, `0x80228C10`, `0x80228CF0` call
  `0x801A9EE0`/`0x801BA670`/`0x80228644` by address. The register sequence and
  every mask are literal; the protocol meaning of the command bytes and of the
  `0x405` mask is inferred from the addresses only.
- `0x8022C120`, `0x8022C1F0` restore the compiler's midpoint arithmetic and the
  range-table walk literally, but the role of the resolved value (tile id,
  palette index or offset) is unknown.

## Deliberate skips

These neighbours were read and left untranslated. None has a row in
`ledger/agents/wave92-bt.csv`.

- `0x80228644` (744) - the EXI block transfer used by four bodies above. Ghidra
  removed every block below its top-level loop as unreachable and printed a
  bare `do { } while (true)`, so the routine cannot be reconstructed from this
  export; its callers use it by address with an `// inferred:` note instead.
- `0x8022892C` (272) - the EXI controller reset. Its two transfers are exactly
  the `0x80228644` calls that cannot be described, so the body would be three
  opaque stubs; left for a pass that can read the routine itself.
- `0x802293B0` (300), `0x8022D2F0` (196), `0x8022D3C0` (120), `0x8022D440`,
  `0x8022D4A0`, `0x8022D500`, `0x8022D560` (96 each) - the quantized-load
  floating-point routines. Their exports carry `unaff_GQR0` and `ldexpf`
  artefacts; the source arithmetic (and therefore the scale constants) cannot
  be recovered honestly from the C.
- `0x8022A590` (132) and `0x8022A620` (132) - two constructors whose export
  calls the base constructor with no printed argument list, because the
  prologue folds `r3`/`r4` into the call. The vtable store and the `+0x6F`
  clear are literal, but the base-call arguments would be a guess.
- `0x80227CC0` (556) - the EXI console text writer. It reads
  `unaff_r2 - 0x4F58` (a stack-slot origin the decompiler does not name), so
  the output path cannot be followed.
- `0x80220524` (572), `0x80221440` (1424), `0x80222698` (1040), `0x80222AA8`
  (912), `0x80222E38` (1052) and `0x80225A80` (764) - the remaining large
  bodies of this bucket. They were not started in this pass; the first five are
  SDP service-database routines whose record layout is documented only inside
  their own exports, and `0x80225A80` sits on the IPC heap slot table.
- `0x802260B0`, `0x80227800`, `0x80227A40`, `0x8022863C`, `0x80228640`,
  `0x8022B3C0`, `0x8022B3D0`, `0x8022B3E0` - 4-byte branch thunks
  (`thunk_FUN_*`) with no body of their own.

## Verification

- Every address and byte size in this doc and in the CSV was re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass; the 28 bodies total
  6508 bytes.
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/bt.rs`.
- No address was already in `ledger/functions.csv` (all 28 were among the
  bucket's 86 untranslated entries).
- `cargo check --message-format=short` reports no errors and no new warnings
  for `src/runtime/bt.rs`; the remaining errors in the workspace come from
  `src/game/player.rs`, `src/runtime/cpp.rs` and `src/game/bases.rs`, which
  belong to other Wave 92 lanes.
- `node tools/check-lane-csv.mjs wave92` reports `wave92-bt.csv: 28 rows` and
  `OK`.
