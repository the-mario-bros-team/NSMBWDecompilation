# Coverage - Wave 73 (resource::archive, `0x80260000` bucket)

## How to read this file

Every row names one Ghidra function by its shipped-image address, the Rust body
that replaces it in `src/resource/archive.rs`, its exact size from
`work/ghidra/main-gb/export/functions.tsv`, and how confident the
reconstruction is. `address` is a virtual address in `sys/main.dol` (image base
`0x80004000`), not a file offset. The 30 bodies are appended after the wave-66
section at the end of `src/resource/archive.rs`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/archive.rs` | `resource::archive` | 30 | 1472 | 19 | 11 |

The file now holds 321 translated bodies: 291 before this pass plus the 30
below. The assigned `0x80260000` bucket had 134 untranslated functions of 260
when this lane started; 30 of them are taken here.

## What this pass covers

The `0x80260000` band holds the NW4R sound/stream objects. This pass takes only
the small, self-contained bodies that reuse record layouts the earlier archive
passes already documented:

1. **Word getter and fixed-layout descriptor initialiser** - `0x802610B0`,
   `0x80261D60` (2 bodies). The first returns `+0x00`; the second writes two
   `0x8000` halfwords (`+0x08`, `+0x0E`) and zeroes the seven words at
   `+0x00`/`+0x04`/`+0x10`/`+0x38..+0x44` plus the three bytes at
   `+0x0C`/`+0x1C`/`+0x1D`.
2. **Deleting-destructor template** - `0x80261DB0`, `0x802654B0`,
   `0x802677A0`, `0x80267B80`, `0x80267BC0`, `0x80267C00` (6 bodies). Six
   byte-identical copies of `if (this && flag > 0) free(this); return this;`
   with `0x802B9280` as the heap free (the same callee `fn_802be100` uses).
3. **Interrupt-guarded record setup** - `0x80261DF0` (1 body). Three word
   stores, a `0x18`-byte clear at `+0x1E` through the `memset`-shaped
   `0x800046B4`, and the `+0x1C` ready byte, all between `0x801B1140` and
   `0x801B1180`.
4. **Text writer and `RBKN` block reader** - `0x802652E0..0x80265470`
   (5 bodies). The writer state lives in `r13 - 0x4988` (object) and
   `r13 - 0x4984` (byte count); `0x802653D0` is the callback that writes the
   four-byte length prefix and adds `(len + 3) & ~3` to the counter.
   `0x80265470` parses a block through `0x802656F0` and clears `+0x0C`.
5. **Pointer tests, forwarders and one vtable setter** -
   `0x80267700..0x80267760` (5 bodies). Two `(-x | x) >> 31` non-null tests on
   the pointers at `+0x08`/`+0x0C`, two tail forwards to `0x8027A200`, and the
   setter that stores at `+0x9C` and mirrors into the child's `+0xB0` through
   vtable slot `+0x24`.
6. **Entry-list drain** - `0x80267DD0` (1 body). Walks the ring rooted at
   `+0x08` and calls `0x80267F40(entry, 1)` where the link word sits `0xD8`
   bytes into each entry.
7. **Clamped accumulator and its setters** - `0x802688C0..0x80268900`
   (3 bodies). `+0x88` accumulates and clamps to `+0x8C`; the two setters fill
   `+0x84`/`+0x8C`/`+0x38`/`+0x88` and `+0x3B`/`+0x54`.
8. **Small `r2`-constant accessors** - `0x802696F0..0x80269980` (5 bodies).
   The zero/scale pair, the sentinel test, the `0x802FA560` word table lookup,
   the `(count + 1)^2 / 4` halfword store and the single `+0x14` byte store.
9. **Count getter** - `0x8026A650` (1 body). Returns `+0x04 - 1`.
10. **Sibling-reader re-bind** - `0x8026DE10` (1 body). Resets the `+0x10C`
    block, re-installs it, re-seeks both cursors from `+0x124`/`+0x11C` plus
    the new base and stores that base at `+0x108`.

### Shared layout notes

`// inferred:` the records are NW4R sound/stream objects; the exports prove the
offsets, the masks and the callee addresses only.

- the deleting destructors all take `(this, deleting)` and free only when
  `deleting > 0`; this is the C++ deleting-destructor convention of the same
  image;
- the writer record is `{ handle @ +0x00, chunk count @ +0x04 }` and the byte
  total sits in the `r13` small-data word, not in the record;
- the accumulator record uses `+0x84` target, `+0x88` current, `+0x8C` limit,
  `+0x38`/`+0x3B` mode bytes.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802610B0` | `fn_802610b0` | 8 | high |
| `0x80261D60` | `fn_80261d60` | 68 | high |
| `0x80261DB0` | `fn_80261db0` | 64 | high |
| `0x80261DF0` | `fn_80261df0` | 128 | high |
| `0x802652E0` | `fn_802652e0` | 68 | high |
| `0x80265330` | `fn_80265330` | 84 | medium |
| `0x80265390` | `fn_80265390` | 52 | medium |
| `0x802653D0` | `fn_802653d0` | 96 | medium |
| `0x80265470` | `fn_80265470` | 56 | medium |
| `0x802654B0` | `fn_802654b0` | 64 | high |
| `0x80267700` | `fn_80267700` | 20 | high |
| `0x80267720` | `fn_80267720` | 20 | high |
| `0x80267740` | `fn_80267740` | 8 | high |
| `0x80267750` | `fn_80267750` | 8 | high |
| `0x80267760` | `fn_80267760` | 64 | medium |
| `0x802677A0` | `fn_802677a0` | 64 | high |
| `0x80267B80` | `fn_80267b80` | 64 | high |
| `0x80267BC0` | `fn_80267bc0` | 64 | high |
| `0x80267C00` | `fn_80267c00` | 64 | high |
| `0x80267DD0` | `fn_80267dd0` | 84 | medium |
| `0x802688C0` | `fn_802688c0` | 32 | high |
| `0x802688E0` | `fn_802688e0` | 24 | medium |
| `0x80268900` | `fn_80268900` | 12 | medium |
| `0x802696F0` | `fn_802696f0` | 24 | medium |
| `0x80269710` | `fn_80269710` | 44 | medium |
| `0x802698A0` | `fn_802698a0` | 24 | high |
| `0x802698C0` | `fn_802698c0` | 24 | high |
| `0x80269980` | `fn_80269980` | 8 | high |
| `0x8026A650` | `fn_8026a650` | 12 | high |
| `0x8026DE10` | `fn_8026de10` | 120 | medium |

## Unknowns

1. **The `r13` writer slots.** `r13 - 0x4988` and `r13 - 0x4984` are read and
   written literally (`0x8042ACF8` / `0x8042ACFC`); the export never names the
   owner, so the roles of the object word and the byte counter are inferred.
2. **The `r2` constants.** `r2 - 0x47E8` (`0x8042E898`) and `r2 - 0x47E4`
   (`0x8042E89C`) are called "scale" and "sentinel"; their values are not
   invented and are read at runtime.
3. **`0x801D4C10` / `0x801D4C40`.** Both are file/stream layer callees of the
   image; the export proves the argument lists but not the semantics.
4. **The `0x801A9AF0` / `0x801A9B00` pair.** `0x80265330` installs
   `0x802653D0` plus the code label `0x80265430`; that they are the write and
   flush halves of one callback pair is inferred.
5. **`0x80265470`'s hidden argument.** Ghidra prints `FUN_802656f0()` with no
   arguments even though the callee takes `(result, block)`, so only `this` in
   `r3` is forwarded and the row is `medium`.
6. **`0x80267760`'s virtual call.** Ghidra prints the `+0x24` slot call with no
   arguments; `this` is forwarded as the only live register and the row is
   `medium`.
7. **`0x80267DD0`'s entry base.** The `-0x36` word offset means the link word
   sits `0xD8` bytes inside the entry; the base arithmetic is literal but the
   entry layout is inferred.

## Deliberate skips

- **`0x80267420`** - keyframe lerp whose body is a GQR/double ladder
  (`CONCAT44(0x43300000, x ^ 0x80000000)`); the export loses the GQR scaling,
  so a byte-accurate body cannot be written honestly.
- **`0x802676C0`** - the export reports "Could not recover jumptable at
  0x802676f0. Too many branches"; the dispatch is not reconstructable.
- **`0x80268910`** - writes the global list at `0x803FC958` and registers
  `0x80267CC0`, but both `0x802810A0` calls are printed without arguments and
  the entry-state chain is only partly recovered.
- **`0x80269660`, `0x802696B0`** - constructors that call `0x802696B0` /
  `0x802696F0` with a float argument only; the receiver register is dropped by
  the export, so the bodies would be partial.
- **`0x8026F630`** - an eight-byte forwarder to `0x8026D9B0` (already
  translated); Ghidra prints only `param_1 + 8`, the other two arguments are
  missing.
- **`0x80268D80`** - a four-byte thunk to `0x80229190`; the export decompiles
  the target, not the thunk.
- **`0x802627C0`** - sixteen bytes: `if (*this) 0x801A0510(...)`. Ghidra prints
  the call with no arguments, and the callee takes `(object, index)`, so only
  the object in `r3` is proved and the body would be partial.
- **The rest of the bucket** - `0x802605D0..0x80264CD0`,
  `0x80265060..0x802667F0`, `0x80266DC0..0x8026A420` and
  `0x8026A880..0x8026D3B0` (excluding the 30 rows above). These are the sound
  state machines, voice/sequence parsers, mixers and 0.4-1.7 KB dispatchers.
  They are sound-engine bodies rather than archive data and are left for a
  dedicated sound pass; this lane's budget was spent on the small, provable
  clusters instead.
- **`0x8026DBA0`, `0x8026DC40`, `0x8026DF50`** - a time-scaling step and a
  curve/sine table lookup over `r2` doubles plus a block-offset query whose
  three stack scratch areas are only partly recovered. The exports lose the
  `CONCAT44(0x43300000, ...)` fixed-point conversion, so a byte-accurate body
  cannot be written honestly.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (read from the TSV, not from prose).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/8026xxxx_FUN_8026xxxx.c` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/archive.rs` with a complete `# Original` / `# Arguments` /
  `# Returns` / `# Side effects` / `# Safety` / `# Confidence` doc block.
- No address was already in `ledger/functions.csv`; all 30 addresses came from
  the untranslated part of the `0x80260000` bucket.
- `node tools/check-lane-csv.mjs wave73` reports 30 rows for
  `wave73-archive.csv` and `OK`.
- `cargo check --message-format=short` reports no error and no diagnostic for
  `src/resource/archive.rs`. The remaining crate errors at the time of this
  pass are in other lanes' `src/game/enemies.rs` and `src/game/worldmap.rs`.
