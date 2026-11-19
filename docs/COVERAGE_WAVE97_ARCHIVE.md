# Coverage - Wave 97 (resource::archive, `0x80260000` bucket)

## How to read this file

Every row names one Ghidra function by its shipped-image address, the Rust body
that replaces it in `src/resource/archive.rs`, its exact size from
`work/ghidra/main-gb/export/functions.tsv`, and how confident the
reconstruction is. `address` is a virtual address in `sys/main.dol` (image base
`0x80004000`), not a file offset. The 26 bodies are appended after the wave-95
section at the end of `src/resource/archive.rs`, in address order.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/resource/archive.rs` | `resource::archive` | 26 | 9908 | 4 | 21 | 1 |

The file now holds 484 translated bodies: 458 before this pass plus the 26
below. The assigned `0x80260000` bucket had 50 untranslated functions when this
lane started; 26 of them are taken here, 24 remain.

## What this pass covers

This pass takes the `0x80260000`-band sound-manager, stream and voice bodies
whose offsets, clamps and callee addresses are visible in the exports:

1. **Manager lifecycle** - `0x802605D0`, `0x80261190`, `0x80261BF0` (3 bodies).
   The constructor self-links the manager's inline list at `0x803FC81C`; the
   dispatcher walks that list and the `0x803FC824` callback; the tick advances
   the global ramp at `0x803FC84C..0x803FC860`.
2. **Ramp stores** - `0x80260A80`, `0x802610C0`, `0x80261B20`, `0x802666C0`,
   `0x80267420` (5 bodies). Five copies of
   `low + cursor * (high - low) / duration`, with the signed compare and the
   unsigned `CONCAT44(0x43300000, x ^ 0x80000000) - 2^52` cast documented in
   each block.
3. **Note cursor and priority queue** - `0x80264220`, `0x80269B50`,
   `0x80269D00` (3 bodies). The first picks the nearest `+-0x60` cursor step;
   the other two walk the same intrusive list to pop the minimum-scored node
   under the buffer mutex.
4. **Voice-parameter commits** - `0x8026BF40`, `0x8026C5F0`, `0x8026CD30`,
   `0x8026D3B0` (4 bodies). Four copies of one clamp-and-publish body over a
   ten/eleven-float input, each gated by one or two sample-rate objects.
5. **Wave loader and stream teardown** - `0x80265B50`, `0x80268790`,
   `0x80268910` (3 bodies). The loader resolves three descriptor kinds and
   copies the `0x80`-byte record; the stream pair releases the handle, runs
   the callbacks and re-queues the record in the `0x803FC958` list.
6. **Envelope, curve and clock helpers** - `0x80269740`, `0x802698E0`,
   `0x80269990`, `0x8026DBA0`, `0x8026DC40` (5 bodies). The four-state
   envelope, the two curve stores, the fractional clock and the quarter-wave
   table evaluator.
7. **Small forwarders and per-frame work** - `0x80267190`, `0x80266250`,
   `0x80266440` (3 bodies). The note-record teardown, its abort/advance path
   and the fade-in/fade-out restart.

### Shared layout notes

`// inferred:` the owners are the NW4R sound manager at `0x803FC810`, its
`0x60`-byte note records and the stream list at `0x803FC958`; the exports prove
only the offsets, the masks, the constants and the callee addresses.

- manager ready byte `r13 - 0x4994`, inline list head `0x803FC81C`, tick
  callback `0x803FC824`;
- per-slot ramp records keep `low`/`high` floats, `duration` and `cursor`
  adjacent (`0x64..0x70`, `0x3C..0x48`, `0xA0..0xAC`, `0xF4..0xFC`);
- voice records mirror every clamped value at a second base (`0x158`/`0x2D0`,
  `0x18`/`0x1C`, `0xF8`/`0x1FC`), which is why four near-identical bodies
  exist;
- the stream list head is `0x803FC958`, its inline sentinel `0x803FC960`, and
  its ready byte `r13 - 0x4978`;
- the `CONCAT44` idiom appears in every ramp; it is written as a plain
  `as f32`/`as f64` cast, following the wave-95 section.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802605D0` | `fn_802605d0` | 528 | medium |
| `0x80260A80` | `fn_80260a80` | 108 | high |
| `0x802610C0` | `fn_802610c0` | 204 | medium |
| `0x80261190` | `fn_80261190` | 344 | medium |
| `0x80261B20` | `fn_80261b20` | 200 | medium |
| `0x80261BF0` | `fn_80261bf0` | 296 | medium |
| `0x80264220` | `fn_80264220` | 428 | medium |
| `0x80265B50` | `fn_80265b50` | 604 | medium |
| `0x80266250` | `fn_80266250` | 488 | medium |
| `0x80266440` | `fn_80266440` | 596 | medium |
| `0x802666C0` | `fn_802666c0` | 260 | medium |
| `0x80267190` | `fn_80267190` | 364 | medium |
| `0x80267420` | `fn_80267420` | 144 | high |
| `0x80268790` | `fn_80268790` | 292 | medium |
| `0x80268910` | `fn_80268910` | 320 | medium |
| `0x80269740` | `fn_80269740` | 348 | medium |
| `0x802698E0` | `fn_802698e0` | 160 | high |
| `0x80269990` | `fn_80269990` | 160 | high |
| `0x80269B50` | `fn_80269b50` | 416 | medium |
| `0x80269D00` | `fn_80269d00` | 188 | medium |
| `0x8026BF40` | `fn_8026bf40` | 716 | medium |
| `0x8026C5F0` | `fn_8026c5f0` | 852 | medium |
| `0x8026CD30` | `fn_8026cd30` | 664 | medium |
| `0x8026D3B0` | `fn_8026d3b0` | 796 | medium |
| `0x8026DBA0` | `fn_8026dba0` | 152 | medium |
| `0x8026DC40` | `fn_8026dc40` | 280 | low |

## Unknowns

1. **The `r2` float/double slots.** The ramp clamps, curve bounds and the
   `2^52` bias live at `SDA2_BASE - 0x46B8..- 0x48A8`. The bodies read the slots
   by address; their values, units and the exact envelope meaning are not
   recovered.
2. **The voice mirror copies.** The four `0x8026Bxxx`/`0x8026Cxxx` commits write
   the same ten/eleven floats twice, at a primary and a mirrored base. The
   exports prove both stores but not which copy the mixer reads.
3. **`0x8026DC40`'s byte tables.** The four signed-byte quarter tables at
   `0x802FA720..0x802FA7E0` are only addressed, never dumped. The body keeps the
   export's sign-extension and unsigned re-interpretation literally, so it is
   the lane's only `low` row.
4. **Dropped virtual receivers.** `0x80267190`, `0x80269B50` and `0x80269D00`
   print several `(**(code **)(*p + slot))()` calls with no arguments. The
   bodies pass the obvious object and say so at each call.

## Deliberate skips

- **`0x80260FA0`, `0x802612F0`, `0x802614E0`, `0x80261740`, `0x80261FB0`,
  `0x802620D0`, `0x802629F0`, `0x802631F0`, `0x802640E0`, `0x80264950`,
  `0x80264CD0`, `0x8026A420`** - the `FUN_802DCD48`/`4C`/`5C`/`64`/`6C`/`70`/
  `74` families. Those are Metrowerks prologue save helpers; Ghidra invents a
  `(r3, r4)` return pair that is really a saved/restored register pair, and
  each of these bodies consumes that pair as its object pointer or budget.
  There is no honest reconstruction without disassembly.
- **`0x80262DB0`, `0x802638A0`, `0x80263A70`, `0x802654F0`** - the
  `unaff_GQR0`/`ldexpf` quantisation path and the `CONCAT44` register-pair
  arithmetic. `src/platform.rs` does not model GQR or `ldexpf`, and the stack
  structs passed to `0x80265890`/`0x80265B50` cannot be laid out honestly.
- **`0x80263F50`** - forwards a five-halfword payload to `0x80264950`, whose
  export takes eight `undefined8` and eight `undefined4` stack arguments plus
  the `cr1` flag. That call shape is not expressible with `call0..call7`.
- **`0x80260AF0`, `0x802622A0`, `0x802667F0`, `0x80266DC0`, `0x80267F40`,
  `0x80268A50`** - the remaining large manager/mixer bodies. They were not
  attempted in this pass; several mix register-pair and inlined-matrix paths
  that the current platform helpers cannot represent.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (read from the TSV, not from prose).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/archive.rs` with a complete `# Original` / `# Arguments` /
  `# Returns` / `# Side effects` / `# Safety` / `# Confidence` doc block.
- No address was already in `ledger/functions.csv`; all 26 addresses came from
  the untranslated part of the `0x80260000` bucket.
- The module header gained item 15 and its body count is 26.
- `node tools/check-lane-csv.mjs wave97` reports 26 rows for
  `wave97-archive.csv` and `OK`.
- `cargo check --message-format=short` reports no error for
  `src/resource/archive.rs`; the remaining errors name other lanes' files from
  the same wave, which are being edited in parallel.
