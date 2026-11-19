# Coverage - Wave 102 (resource::archive, `0x80260000` bucket)

## How to read this file

Every row names one Ghidra function by its shipped-image address, the Rust body
that replaces it in `src/resource/archive.rs`, its exact size from
`work/ghidra/main-gb/export/functions.tsv`, and how confident the
reconstruction is. `address` is a virtual address in `sys/main.dol` (image base
`0x80004000`), not a file offset. The 22 bodies are appended after the wave-99
section at the end of `src/resource/archive.rs`, in address order.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/resource/archive.rs` | `resource::archive` | 22 | 14956 | 1 | 19 | 2 |

The file now holds 531 translated bodies: 509 before this pass plus the 22
below. The assigned `0x80260000` bucket had exactly 24 untranslated functions
when this lane started; 22 are taken here and 2 are skipped with reasons (see
deliberate skips), so the bucket is complete apart from those two.

## What this pass covers

1. **Channel ramps and the mode change** - `0x80260AF0`, `0x80260FA0`,
   `0x802612F0`, `0x802614E0`, `0x80261740` (5 bodies). One tick advances the
   three `0x10`-byte channel blocks and the master ramp, one drains every
   member node on a mode change, and the reset pair plus the per-channel
   callback program the `0x801A1910/1920/1930` levels and the `0x801A0CA0/D10/D80`
   callbacks.
2. **Note-window and frequency setters** - `0x80261FB0`, `0x802620D0`,
   `0x802622A0`, `0x802629F0`, `0x80262DB0`, `0x802631F0`, `0x802638A0`,
   `0x80263A70`, `0x80263D20`, `0x80263F50` (10 bodies). The three-way
   `mode` ladder (`3` = `%14` pair packing, `2` = direct, `1` = half) is shared
   by the first four; the latter six add the `0x802DCC4C` conversion, the
   `0x10000` clamps, the track table at `0x803FC600` and the `0x802839D0`
   window reader.
3. **Record factory and stream records** - `0x80264CD0`, `0x802654F0`,
   `0x802667F0`, `0x80266DC0`, `0x80267F40`, `0x80268A50`, `0x8026A420`
   (7 bodies). `0x80264CD0` mirrors the already-translated `0x80265100` drain
   and then binds one record through `0x801A0380`; `0x802654F0` turns a
   nine-word descriptor into a stream record; `0x802667F0` is the virtual
   mixer state machine; `0x80266DC0` uploads the mixed voice parameters;
   `0x80267F40` runs the envelope and the fourteen `0x80281xxx` render setters;
   `0x80268A50` acquires and binds one stream record; `0x8026A420` rebuilds
   its payload list.

### Shared layout notes

`// inferred:` the owners are the NW4R sound manager at `0x803FC810`, the JAS
channel tables at `0x803FC8C4..0x803FC904` and the stream list at
`0x803FC958`; the exports prove only the offsets, the ladders, the constants
and the callee addresses.

- the `0x802DCD4C/60/64/68/6C/70/74` family are the Metrowerks register-save
  stubs; this file's established convention is **high word = incoming `r3` =
  first argument, low word = incoming `r4` = second argument**, which is what
  the seven pair-consuming bodies use;
- `0x80262FE0`'s comparison and `0x80264400`'s 24-halfword block are the same
  window this pass advances; the pair block is built in the child's order
  `[0x1E, 0x20, 0x24, 0x26, 0x2A, 0x2C, 0x30, 0x32, 0x22, 0x28, 0x2E, 0x34]`;
- `0x800046B4` is the memset wrapper and `0x80004364` the block copy, both
  already documented in this file;
- the `CONCAT44(0x43300000, x) - 2^52` idiom is written as a plain
  `as f32`/`as f64` cast, following the wave-95, wave-97 and wave-99 sections.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80260AF0` | `fn_80260af0` | 972 | medium |
| `0x80260FA0` | `fn_80260fa0` | 260 | medium |
| `0x802612F0` | `fn_802612f0` | 496 | medium |
| `0x802614E0` | `fn_802614e0` | 348 | high |
| `0x80261740` | `fn_80261740` | 988 | low |
| `0x80261FB0` | `fn_80261fb0` | 284 | medium |
| `0x802620D0` | `fn_802620d0` | 296 | medium |
| `0x802622A0` | `fn_802622a0` | 500 | medium |
| `0x802629F0` | `fn_802629f0` | 948 | medium |
| `0x80262DB0` | `fn_80262db0` | 224 | medium |
| `0x802631F0` | `fn_802631f0` | 1568 | medium |
| `0x802638A0` | `fn_802638a0` | 460 | medium |
| `0x80263A70` | `fn_80263a70` | 288 | medium |
| `0x80263D20` | `fn_80263d20` | 548 | medium |
| `0x80263F50` | `fn_80263f50` | 400 | medium |
| `0x80264CD0` | `fn_80264cd0` | 696 | medium |
| `0x802654F0` | `fn_802654f0` | 504 | medium |
| `0x802667F0` | `fn_802667f0` | 1412 | medium |
| `0x80266DC0` | `fn_80266dc0` | 968 | low |
| `0x80267F40` | `fn_80267f40` | 1756 | medium |
| `0x80268A50` | `fn_80268a50` | 492 | medium |
| `0x8026A420` | `fn_8026a420` | 548 | medium |

## Unknowns

1. **The `r2` sound constants.** Every ramp bound, bias and scaler lives in
   `SDA2_BASE - 0x4800..-0x4898`. The bodies read the slots by address; their
   values and units are not recovered, and the double-valued ones are used
   through the `as f64` cast of the int-to-double idiom.
2. **The `0x80281xxx` render setters.** Fourteen functions in the `+0xD0`
   render record path take `(f64, record)` or `(f64, record, index)`. Their
   field offsets are documented in `render::gx_band`; this lane only forwards
   the computed values, so the roles of `+0x90`/`+0x94`/`+0x98` remain
   unresolved.
3. **`0x80281390`.** The release path calls it with no arguments in the export;
   the argument is dropped, and the body therefore uses `call0`.
4. **`fn_80261740`'s listener.** The per-channel ring slot `+0x14` receives
   `(f64, object, count, words, 0x180, 0, manager state)`; the export prints the
   first argument as a `r2` float and no callee prototype, so only the shape is
   certain.

## Deliberate skips

- **`0x802640E0` (316 bytes)** - the body copies `0x20` bytes from `param_6`
  and then decodes nibbles from `0x80000000 + index`, i.e. an absolute MEM1
  address that is outside `sys/main.dol` (image base `0x80004000`). Ghidra
  folded a pointer/offset pair into that constant, so the table the body reads
  cannot be identified from the export. Writing it as a raw `0x80000000` read
  would invent an address, so no body was written.
- **`0x80264950` (388 bytes)** - a genuine varargs consumer: it builds a
  `va_list` on the stack, consults `cr1` for the float-save area and pulls two
  or five halfwords through `0x802DC6B8`. Its Ghidra prototype has sixteen
  `undefined8` parameters. The callee is still named from
  [`fn_80263f50`] through a `call7` with the five by-value halfwords, but the
  variadic entry itself is not reconstructible without the register-save-area
  layout, so no body was written.
- **The GQR prologues** - `0x80260AF0`, `0x80262DB0`, `0x80263A70`,
  `0x802667F0` and `0x80267F40` open with paired-single quantisation pairs
  whose results are never stored. They are omitted, exactly as the wave-97 and
  wave-99 sections did for their own dead GQR blocks.
- **`0x80266DC0`'s eight float spills** - the `psq_st` block at the top of the
  export writes eight stack slots that are never read; it is reproduced as a
  comment only, so the body accepts no float arguments.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (read from the TSV, not from prose); the 22 sizes sum to 14956 bytes.
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/archive.rs` with a complete `# Original` / `# Arguments` /
  `# Returns` / `# Side effects` / `# Safety` / `# Confidence` doc block.
- No address was already in `ledger/functions.csv`; all 22 addresses came from
  the untranslated part of the `0x80260000` bucket.
- The module header gained item 17; its body count is 22 and the file total is
  531.
- `node tools/check-lane-csv.mjs wave102` reports 22 rows for
  `wave102-archive.csv` and `OK`.
- `cargo check --message-format=short` reports no error for
  `src/resource/archive.rs`; the remaining errors name other lanes' files from
  the same wave, which are being edited in parallel.
