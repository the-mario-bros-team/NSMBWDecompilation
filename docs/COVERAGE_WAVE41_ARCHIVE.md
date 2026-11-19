# Coverage - Wave 41 (resource::archive, 0x80260000 bucket)

## How to read this file

Every row names one Ghidra function by its shipped-image address, the Rust body
that replaces it in `src/resource/archive.rs`, its exact size from
`work/ghidra/main-gb/export/functions.tsv`, and how confident the
reconstruction is. `address` is a virtual address in `sys/main.dol` (image base
`0x80004000`), not a file offset. Bodies are ordered by address inside the
wave-41 section of the file so the run can be read top to bottom.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/archive.rs` | `resource::archive` | 28 | 2264 | 14 | 14 |

The file now holds 150 bodies: 122 before this pass plus the 28 below.

## What this pass covers

The assigned bucket `0x80260000` still has 220 untranslated functions of 260.
Wave 39 already took the DVD reader, the buffer cache and the record/list
primitives; the rest of the band is NW4R **sound-library** code rather than
game-archive code, and no other lane claims an address in `0x8026xxxx`. This
pass therefore continues the two clusters that the translated archive reader
really reaches, and it says so in the module header.

1. **Sound-manager singleton and record lists** — `0x80260800..0x80260F40`
   (6 bodies). `0x80260860` is the lazy singleton at `0x803FC810` guarded by
   `r13 - 0x4994`; it runs the 528-byte constructor `0x802605D0` and registers
   `0x802608D0` through `0x802DC780`. The neighbours are the manager's record
   primitives: `0x80260800` (unlink through `0x80229030` then conditional
   `0x802B9280` free), `0x802608D0` (static-array destructor `0x802DCA98` with
   a `0xC` stride), `0x80260EC0` (read `+0x04`), and the interrupt-masked
   insert/remove pair `0x80260ED0`/`0x80260F40` around `0x80229160`/`0x80229190`.
   `0x80260860` is the exact getter already used by [`fn_8026d770`] and by the
   wave-39 cache walkers, so this cluster closes an existing loose end.
2. **Member-object layer** — `0x8026A670..0x8026BED0` (20 bodies). Three
   constructor/teardown families share one layout: a vtable at `+0x00`, an
   active byte at `+0x0C`, a mode word at `+0x10`, a guard mutex at `+0x10`
   or `+0x14`, and an AX voice-set context at `+0x2C`, `+0x30`, `+0x44`,
   `+0x88` or `+0xD0`. Each family supplies its constructor (`0x8026A670`,
   `0x8026AB10`, `0x8026B160`), its aligned buffer-size query (`0x8026A700`,
   `0x8026ABA0`, `0x8026B1F0`, `0x8026BDC0`), its mutex initialiser
   (`0x8026A740`, `0x8026AC10`, `0x8026B230`, `0x8026BE00`), its
   clear/save/free/restore release path (`0x8026A810`, `0x8026AD50`,
   `0x8026B300`, `0x8026B880`, `0x8026BED0`), the manager-count re-dispatch
   (`0x8026B070`, `0x8026BC00`) and the deleting destructors
   (`0x8026B0E0`, `0x8026BC70`).
3. **Stream-block defaults** — `0x8026DB50`/`0x8026DB80` (2 bodies). The two
   initialisers for the block object that the sibling reader and the stream
   player carry at `+0x10C`/`+0x1C`; both copy their defaults from the
   small-data area (`r2 - 0x46E0`, `r2 - 0x46DC`).

### Small-data addresses

`r2 = 0x80433080` is fixed by `0x800041C0` and already documented in
[`crate::engine::globals`] and [`crate::audio::seq`]. The wave-41 bodies use it
as `SDA2_BASE` to resolve:

| body | offset | absolute |
|---|---|---|
| `0x8026A670` | `r2 - 0x47C0..-0x47B4` | `0x8042E8C0..0x8042E8CC` |
| `0x8026AB10` | `r2 - 0x47A0..-0x4794` | `0x8042E8E0..0x8042E8EC` |
| `0x8026B160` | `r2 - 0x4780..-0x4778` | `0x8042E900..0x8042E908` |
| `0x8026DB50`/`DB80` | `r2 - 0x46E0`/`-0x46DC` | `0x8042E9A0`/`0x8042E9A4` |

The values are copied bit-for-bit. `// unknown:` their numeric meaning is not
printed by the export, so no float interpretation is claimed.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80260800` | `fn_80260800` | 88 | high |
| `0x80260860` | `fn_80260860` | 100 | medium |
| `0x802608D0` | `fn_802608d0` | 120 | medium |
| `0x80260EC0` | `fn_80260ec0` | 8 | high |
| `0x80260ED0` | `fn_80260ed0` | 108 | high |
| `0x80260F40` | `fn_80260f40` | 88 | high |
| `0x8026A670` | `fn_8026a670` | 144 | medium |
| `0x8026A700` | `fn_8026a700` | 44 | high |
| `0x8026A740` | `fn_8026a740` | 8 | high |
| `0x8026A810` | `fn_8026a810` | 100 | medium |
| `0x8026AB10` | `fn_8026ab10` | 144 | medium |
| `0x8026ABA0` | `fn_8026aba0` | 84 | high |
| `0x8026AC10` | `fn_8026ac10` | 8 | high |
| `0x8026AD50` | `fn_8026ad50` | 124 | medium |
| `0x8026B070` | `fn_8026b070` | 112 | medium |
| `0x8026B0E0` | `fn_8026b0e0` | 124 | medium |
| `0x8026B160` | `fn_8026b160` | 140 | medium |
| `0x8026B1F0` | `fn_8026b1f0` | 44 | high |
| `0x8026B230` | `fn_8026b230` | 8 | high |
| `0x8026B300` | `fn_8026b300` | 100 | medium |
| `0x8026B880` | `fn_8026b880` | 124 | medium |
| `0x8026BC00` | `fn_8026bc00` | 112 | medium |
| `0x8026BC70` | `fn_8026bc70` | 124 | medium |
| `0x8026BDC0` | `fn_8026bdc0` | 44 | high |
| `0x8026BE00` | `fn_8026be00` | 8 | high |
| `0x8026BED0` | `fn_8026bed0` | 100 | medium |
| `0x8026DB50` | `fn_8026db50` | 36 | high |
| `0x8026DB80` | `fn_8026db80` | 20 | high |

## Cluster notes

- The three constructors do **not** store the same fields. `0x8026A670` clears
  `+0x04`, `+0x08`, `+0x0C`, `+0x10` and `+0x14`; `0x8026AB10` clears
  `+0x04`, `+0x08`, `+0x0C`, `+0x14` and `+0x18` and deliberately leaves
  `+0x10` alone; `0x8026B160` clears `+0x04`, `+0x08`, `+0x0C`, `+0x10` and
  `+0x14`. The bodies keep each list literal.
- `0x8026A700`, `0x8026A1B0`-style size queries all use the same
  `(size + 0x87) & !0x1F` clamp, i.e. room for an `0x80`-byte header plus
  alignment. `0x8026ABA0` takes the maximum of two such sizes.
- The release paths (`0x8026A810`, `0x8026AD50`, `0x8026B300`, `0x8026B880`,
  `0x8026BED0`) are one shape: test `+0x0C`, clear it, save the guard state
  through `0x80265330`, free the AX set, then restore through `0x80265390`.
  `+0x10` selects between two voice contexts in two of them, exactly as the
  export shows.
- `0x8026B070` and `0x8026BC00` are byte-for-byte the same algorithm as the
  already-translated `0x8026D770`: manager count =
  `countLeadingZeros(count - 2) >> 5`, then the `+0x10` and `+0x0C` virtual
  slots when it differs.
- `0x8026B0E0`/`0x8026BC70` follow the already-translated `0x8026D7E0`:
  vtable store, the pointer slot at `0x8034EB28`/`0x8034EB78` called with
  `this`, the `+0x20` virtual slot, then `0x802B9280` when `deleting > 0`.
- `0x8026DB50` writes `+0x0C = 1` while its sibling `0x8026DB80` only resets
  `+0x10`/`+0x14`; both write the same small-data word, so they are the
  constructor and reset of one block object.

## Deliberate skips

- **`0x8026DE10` (120 B)** — the leading `0x80275D70` call needs a source
  header pointer (`(dst_block, src_block)`), but the export prints a single
  argument and the function's own second argument is the numeric byte offset
  added to `+0x124`/`+0x11C`, not a pointer. There is no honest source for the
  dropped argument, so the body is left out. The remaining three calls
  (`0x802756D0`, `0x80275ED0`, `0x80275E20`) and the `+0x108` store are
  unambiguous; only the first call blocks it.
- **`0x8026DF50` (176 B)** — its seven stack locals are out-parameters of
  `0x80275840`/`0x80275800`/`0x80275810`, and the export never shows which
  call writes `local_34`/`local_18`; the final sum
  `*(u32*)(this+0x108) + local_18 + local_34` cannot be justified.
- **`0x8026DBA0`, `0x8026DC40`** — float/`CONCAT44` arithmetic over
  `r2 - 0x46xx` doubles; the doubles are not identified, so no faithful body
  is possible without guessing the conversion bias.
- **`0x80269660..0x80269990`** — the sound-envelope ramp family (float
  multiplication, `CONCAT44` and a `DAT_802fa460` table).
- **`0x8026A880`, `0x8026ADD0`, `0x8026B370`** — the 420–508-byte parameter
  stores called by the three constructors. They are pure float-range clamping
  plus an AX allocation check; the constructors call them by address and the
  doc blocks say so.
- **`0x8026A010..0x8026A420`, `0x8026B900`, `0x8026BF40`, `0x8026C5F0`,
  `0x8026CD30`, `0x8026D3B0`** — 412–852-byte heap/list state machines that
  drive AX voice allocation, not the archive reader.
- **`0x8026F630` (8 B)** — thunk forward to `0x8026D9B0`; skipped per the brief.
- **`0x80268D80` (4 B)** — `thunk_FUN_80229190`; already skipped in wave 39.
- **`0x80267300..0x80267760`** — 8/12-byte vtable accessors for a different
  parameter object; left for that object's owner.
- Everything else in the band is float/paired-single code, an AX voice object
  already owned conceptually by `audio::ax`, or already translated. No BRRES
  loader was found in `0x80260000`.

## Verification

- Every address and byte size matches `functions.tsv` (read from the TSV, not
  from prose).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/8026xxxx_FUN_8026xxxx.c` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/archive.rs` with a complete `# Original` / `# Arguments` /
  `# Returns` / `# Side effects` / `# Safety` / `# Confidence` doc block.
- No address was already in `ledger/functions.csv`; none of the 28 overlaps
  the wave 39 (`0x80268C40..0x8026DE90`) ranges.
- `node tools/check-lane-csv.mjs wave41` reports `wave41-archive.csv: 28 rows`
  and `OK` with no problem naming this file.
- `cargo check --message-format=short` is free of errors for
  `src/resource/archive.rs`; the 13 warnings are pre-existing and live in
  `runtime::bt`, `runtime::cpp` and `runtime::input`.
