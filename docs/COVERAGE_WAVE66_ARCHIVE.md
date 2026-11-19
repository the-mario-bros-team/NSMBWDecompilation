# Coverage - Wave 66 (resource::archive, `0x802B0000` bucket)

## How to read this file

Every row names one Ghidra function by its shipped-image address, the Rust body
that replaces it in `src/resource/archive.rs`, its exact size from
`work/ghidra/main-gb/export/functions.tsv`, and how confident the
reconstruction is. `address` is a virtual address in `sys/main.dol` (image base
`0x80004000`), not a file offset. The 27 bodies are appended after the wave-62
section at the end of `src/resource/archive.rs`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/archive.rs` | `resource::archive` | 27 | 3540 | 22 | 5 |

The file now holds 291 translated bodies: 264 before this pass plus the 27
below. The assigned `0x802B0000` bucket had 151 untranslated functions of 306
when this lane started; 27 of them are taken here.

## What this pass covers

The `0x802B0000` band is mixed: most of it is layout, text, font and controller
code owned by other subsystems. This pass takes only the groups that touch the
archive-owned section table, the archive heap allocator, or the child object
that `0x802BC920` already owns:

1. **Section-count writers** - `0x802B3620..0x802B3940` (5 bodies). Five
   setters grow the same packed table the wave-54 size queries read: the count
   word at `+0x38`, the bitfield key at `+0x3C` and the base pointer at `+0x40`.
   Each initialises only the newly added rows (`old..count`) and then writes its
   count into the matching bitfield. Row strides are `0x1C`, `4`, `0x10` and
   `4`; `0x802B3940` is the general staged-index store over the `+0x28` bytes,
   `+0x10` halfwords and `+0x40` colour row.
2. **RLYT record reset** - `0x802B2B50` (1 body). Copies four default halfwords
   from `r2 - 0x3458`, fills eight halfwords with `0xFF`, masks both counters to
   five bits, clears `+0x59` and clears the section base at `+0x40`.
3. **Registered-list release** - `0x802B6320` (1 body). Walks the ring at
   `+0x04`, unlinks each entry through `0x802290C0` when the filter is zero or
   matches the entry key, and clears the entry tail. `this` and `filter` arrive
   behind the `0x802DCD70` register-save prologue.
4. **Byte scalers and colour forwarder** - `0x802B6AE0`, `0x802B6B50`,
   `0x802B71E0` (3 bodies). A four-word validity test for mode `0xFF`, a
   four-byte copy that scales byte 3 by `mode / 0xFF`, and a 16-byte colour
   block scaler that forwards to `0x802B6C80`.
5. **Ordered queue behind `fn_802bc920`** - `0x802BDB40..0x802BE450`
   (17 bodies). `fn_802bdd00` builds the 0x1C-byte list-header pair (vtable
   `0x8034FF88`), `fn_802bdd60` fills the free list with `0x28`-byte nodes
   (vtable `0x8034FF94`), and `0x802BDB40`/`70`/`D0`/`DC10` initialise, arm and
   step a node. `0x802BE010` returns a node to the active list (rotating the
   active head when the free list is empty); `0x802BDF70`/`DFC0` arm it with a
   string range or a float step; `0x802BDDF0` drains the active list back to the
   free list. The four `0x802BE2xx/3xx` bodies are array setters over the
   archive allocator, and `0x802BE420`/`450` initialise the default quad.

### Shared layout of the queue cluster

`// inferred:` the pair at `+0x04` and `+0x10` is an NW4R intrusive-list pair;
the exports prove the helper calls and the offsets only.

- list headers are 0x1C bytes (`0x80228DD0` is called with that size), with the
  element count as a halfword at `+0x0C` of each header;
- queue nodes are 0x28 bytes: `+0x04`/`+0x08` string cursor, `+0x0C` repeat
  count, `+0x10` step, `+0x14` accumulator, `+0x18` flag byte;
- flag bit 0 selects string mode, bit 2 suppresses the countdown, bit 3 is the
  enable bit, bit 4 selects wrap and bit 5 enables the countdown;
- the owning child object is stored at `+0x1C` of the two-header object.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802B2B50` | `fn_802b2b50` | 112 | high |
| `0x802B3620` | `fn_802b3620` | 144 | high |
| `0x802B36B0` | `fn_802b36b0` | 136 | high |
| `0x802B3740` | `fn_802b3740` | 292 | high |
| `0x802B3870` | `fn_802b3870` | 196 | high |
| `0x802B3940` | `fn_802b3940` | 440 | high |
| `0x802B6320` | `fn_802b6320` | 132 | medium |
| `0x802B6AE0` | `fn_802b6ae0` | 104 | high |
| `0x802B6B50` | `fn_802b6b50` | 60 | high |
| `0x802B71E0` | `fn_802b71e0` | 304 | medium |
| `0x802BDB40` | `fn_802bdb40` | 36 | high |
| `0x802BDB70` | `fn_802bdb70` | 84 | high |
| `0x802BDBD0` | `fn_802bdbd0` | 60 | high |
| `0x802BDC10` | `fn_802bdc10` | 236 | high |
| `0x802BDD00` | `fn_802bdd00` | 88 | high |
| `0x802BDD60` | `fn_802bdd60` | 140 | medium |
| `0x802BDDF0` | `fn_802bddf0` | 112 | high |
| `0x802BDF70` | `fn_802bdf70` | 80 | high |
| `0x802BDFC0` | `fn_802bdfc0` | 80 | high |
| `0x802BE010` | `fn_802be010` | 172 | high |
| `0x802BE100` | `fn_802be100` | 64 | medium |
| `0x802BE210` | `fn_802be210` | 80 | high |
| `0x802BE260` | `fn_802be260` | 116 | high |
| `0x802BE2F0` | `fn_802be2f0` | 80 | high |
| `0x802BE340` | `fn_802be340` | 116 | high |
| `0x802BE420` | `fn_802be420` | 48 | medium |
| `0x802BE450` | `fn_802be450` | 28 | high |

## Unknowns

1. **The section-table owner.** The exports never name the object behind
   `+0x38`/`+0x3C`/`+0x40`; the doc blocks describe literal offsets and leave
   the role reading `// inferred:`.
2. **The two queue constants.** `r2 - 0x3F08` and `r2 - 0x3F04` are loaded at
   runtime; the bodies call them "one" and "inert" and mark the naming as
   inferred. Their values are not invented.
3. **The `0x802B6C80` argument.** Ghidra prints the trailing call in
   `0x802B71E0` with no arguments; the scaled stack block is forwarded by
   inference, so that row is `medium`.
4. **The `0x802B9280` free argument.** `0x802BE100` prints a bare free call.
   `this` is the only guarded pointer and is forwarded by inference, so the row
   is `medium`.
5. **The `0x802BE450` call argument.** `0x802BE420` prints a bare call and then
   returns `param_1`; the returned pointer is forwarded by inference, so the row
   is `medium`.

## Deliberate skips

- **`0x802B2980..0x802B2B50`** - one-line 4-byte / 16-byte copies and
  `a + b` template instantiations. Wave 54 skipped them as padding and this
  pass keeps that decision; only `0x802B2B50` (the real record initialiser) is
  taken.
- **`0x802B9700`** - a four-byte thunk to `0x801D4A90`, not translated.
- **`0x802B4C60`, `0x802B4DF0`, `0x802B4EE0`, `0x802BDE60`** - double/GQR
  keyframe interpolation. The exports lose their `GQR0` scaling and several
  `f1` arguments, so a byte-accurate body cannot be written honestly.
- **`0x802B4940..0x802B4C40` and `0x802BF3D0..0x802BFE90`** - pane drawing and
  window geometry over GX (`0x801C6D90`, `0x801C7520`, `0x801C1350`); these are
  layout/renderer code, not archive data.
- **`0x802B0680`, `0x802B20B0`, `0x802B2C70`, `0x802B6C80`** - 1.3-1.8 KB
  parsers/blobs outside a small-cluster pass; their bodies are referenced only
  by address.
- **`0x802B8xxx` and `0x802BA5C0`/`A630`/`A710`** - already translated in
  `resource::archive`, `resource::registry` or `resource::bres`; no duplicate
  rows are emitted.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (read from the TSV, not from prose).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/802bxxxx_FUN_802bxxxx.c` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/archive.rs` with a complete `# Original` / `# Arguments` /
  `# Returns` / `# Side effects` / `# Safety` / `# Confidence` doc block.
- No address was already in `ledger/functions.csv`; all 27 addresses came from
  the untranslated part of the `0x802B0000` bucket.
- `node tools/check-lane-csv.mjs wave66` reports 27 rows for
  `wave66-archive.csv` and `OK`.
- `cargo check --message-format=short` reports no error and no diagnostic for
  `src/resource/archive.rs`; the remaining crate errors are in other lanes'
  `src/game/bases.rs` and `src/runtime/bt.rs`.
