# Coverage - Wave 112 (resource::archive, `0x80260000` bucket close-out)

## How to read this file

Every row names one Ghidra function by its shipped-image address, the Rust body
that replaces it in `src/resource/archive.rs`, its exact size from
`work/ghidra/main-gb/export/functions.tsv`, and how confident the
reconstruction is. `address` is a virtual address in `sys/main.dol` (image base
`0x80004000`), not a file offset. The two bodies are appended after the wave-105
section at the end of `src/resource/archive.rs`, in address order.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/archive.rs` | `resource::archive` | 2 | 704 | 0 | 2 |

The file now holds 548 translated bodies: 546 before this pass plus the 2
below. The assigned `0x80260000` bucket held exactly 2 untranslated functions
when this lane started; both are taken here, so the bucket is closed.

## What this pass covers

1. **Packed-nibble track decoder** - `0x802640E0` (316 bytes). Copies a
   `0x28`-byte filter state from the source track record, decodes `count`
   nibbles from a byte stream, and feeds each nibble to the three-tap resampler
   at `0x80285660`. For an 8-byte-aligned stream a control byte replaces the
   filter config at `+0x22` after every 14 nibbles (7 data bytes + 1 control
   byte); the body keeps the export's literal `index & 0xF` test, so an
   unaligned stream only shifts the control-byte positions. The three halfwords
   at `+0x22`/`+0x24`/`+0x26` are written back through the three output
   pointers.
2. **Varargs track-window store** - `0x80264950` (388 bytes). Consumes two or
   five variadic halfwords depending on `mode` and stores them into the sound
   handle's `+0x140...` window, raising `0x20000000` (mode 1) or `0x40000000`
   (mode 2) in the owner's `+0x04` flag word. Interrupts are disabled across
   the stores.

### Shared layout notes

- The `0x802640E0` stream arithmetic is literal, but its two raw
  `0x80000000 + (index >> 1)` loads are the export's folded form of
  `stream + 1 + n` (`stream` is the MEM1 pointer and `folded` is
  `stream - 0x80000000`). The body keeps the stream-relative address, so it
  does not read an invented table at `0x80000000`. The `0`-stream arm keeps the
  export's literal address.
- The magic constant `0x124924925` in the loop bound is the compiler's
  divide-by-14; after the `& 0xfffffff0` mask it is `16 * (count / 14)`.
- `0x80264950` builds its `va_list` with `gpr = 2`, `reg_save_area = &saved r3`
  and `input_arg_area = sp + 8`. The translated `va_arg` fetch `0x802DC6B8`
  therefore returns `r5`, `r6`, ... in turn. The only call site,
  [`fn_80263f50`], passes mode `2` and five words, which is why the body takes
  five explicit register arguments instead of a variadic signature.
- The `cr1` float-save block and the `param_9..16` saves of `0x80264950` are
  never read; they are the CodeWarrior variadic prologue and are omitted, as
  the file already does for dead GQR blocks.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802640E0` | `fn_802640e0` | 316 | medium |
| `0x80264950` | `fn_80264950` | 388 | medium |

## Unknowns

1. **The three output halfwords of `0x802640E0`.** `+0x22` is the filter
   config (proven by `0x80285660`); `+0x24`/`+0x26` are the two history
   samples. The caller immediately forwards the block to `0x80262E90`, but that
   callee is not reconstructed here, so the consumer roles stay inferred.
2. **The stream header byte.** Nibble 0 is read at `stream + 1`; the byte at
   `stream` itself is never touched by this body. Its meaning is unknown.
3. **The `mode` values of `0x80264950`.** Only `1` and `2` are accepted; any
   other value restores interrupts and returns without a store. The caller uses
   `2`; no in-image caller of mode `1` was found.
4. **The filter tap window.** `0x80285660` reads `state + (config >> 4) * 4`
   and `+2`. The body allocates the same `0x28` bytes the export copies; a
   config whose high nibble exceeds 9 would step outside that block, so the
   caller is responsible for the tap index, as the helper's own safety note
   already states.

## Deliberate skips

None. Both functions left in the bucket were reconstructible; the earlier
wave-102 skip notes for `0x802640E0` (folded `0x80000000` address) and
`0x80264950` (unknown register-save layout) are resolved above.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (read from the TSV, not from prose); the 2 sizes sum to 704 bytes.
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/archive.rs` with a complete `# Original` / `# Arguments` /
  `# Returns` / `# Side effects` / `# Safety` / `# Confidence` doc block.
- No address was already in `ledger/functions.csv`; both addresses came from
  the untranslated part of the `0x80260000` bucket.
- The module header gained item 19; its body count is 2 and the file total is
  548.
- `node tools/check-lane-csv.mjs wave112` reports 2 rows for
  `wave112-archive.csv` and `OK`.
- `cargo check --message-format=short` reports no error for
  `src/resource/archive.rs`; the remaining warnings name other lanes' files
  from the same wave, which are being edited in parallel.
