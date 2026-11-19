# Coverage - Wave 79 (resource::archive, `0x802B0000` bucket)

## How to read this file

Every row names one Ghidra function by its shipped-image address, the Rust body
that replaces it in `src/resource/archive.rs`, its exact size from
`work/ghidra/main-gb/export/functions.tsv`, and how confident the reconstruction
is. `address` is a virtual address in `sys/main.dol` (image base `0x80004000`),
not a file offset. The 30 bodies are appended after the wave-73 section at the
end of `src/resource/archive.rs`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/archive.rs` | `resource::archive` | 30 | 1688 | 22 | 8 |

The file now holds 351 translated bodies: 321 before this pass plus the 30
below. The assigned `0x802B0000` bucket had 124 untranslated functions of 306
when this lane started; 30 of them are taken here.

## What this pass covers

The `0x802B0000` band holds the RLYT layout/polygon records and the screen-effect
objects that sit alongside the archive code. This pass takes the small,
self-contained bodies whose offsets, masks and callee addresses are all visible
in the exports:

1. **Record copies and one list-head initialiser** - `0x802B2070`,
   `0x802B2090`, `0x802B2980`, `0x802B29B0`, `0x802B2A50`, `0x802B2B10`
   (6 bodies). Two-word and four-byte copies, one sixteen-byte copy that loads
   everything before storing, the self-linking empty list head, and the
   unsigned minimum at `0x802B2B10`.
2. **Polygon-descriptor helpers behind the `+0x38` table** - `0x802B4C40`,
   `0x802B4D30`, `0x802B4DB0`, `0x802B50A0`, `0x802B50B0`, `0x802B50D0`,
   `0x802B5E10` (7 bodies). The GX upload reads the `+0x18` mode bitfield and
   the `+0x16` halfword; the constructor installs vtable `0x8034FB68` and fills
   seven default words at `+0x34..+0x4C`; the two `+0x0C`-indirect getters read
   the payload halfword and flag byte.
3. **`RLAN` section-table scan** - `0x802B5B10` (1 body). Tests the `'RLAN'`
   magic through `0x802B6810`, gates on revision byte `+0x07 == 0` and version
   byte `+0x06` in 8..10, then walks the `+0x0E` nodes and records the
   `pat1`/`pah1`/`pai1` tags.
4. **Effect-object setters and frame accumulator** - `0x802BB0E0`,
   `0x802BB0F0`, `0x802BB110`, `0x802BB600` (4 bodies). A halfword setter, a
   three-byte copy to `+0x10..+0x12`, the two-state mode setter and the
   `+0x9C += +0xA0` accumulator that wraps at the `r2 - 0x3F48` limit and
   toggles bit `0x10` of the `+0x09` flag byte.
5. **Aligned allocator, clear and float getters** - `0x802BC670`, `0x802BC6E0`,
   `0x802BD250`, `0x802BD270` (4 bodies). `0x802BC670` rounds the request to
   `0x20`, allocates `size + 0xA0` through `0x802B8CC0` and binds the result
   through `0x801C1DB0`; the two getters return `+0x78`/`+0x7C` only when the
   `+0x74` byte is set.
6. **Deleting destructor** - `0x802BD7C0` (1 body). Releases `+0x08` through
   `0x802B9290`, clears it, then frees `this` through `0x802B9280` when the
   delete flag is positive.
7. **Virtual `+0x08` getter copies** - `0x802BE8D0`, `0x802BE930`,
   `0x802BE990` (3 bodies). Each calls vtable slot `+0x08` with the object as
   its only argument and copies three words from `+0x20`, `+0x00` and `+0x10`
   respectively.
8. **Small-data accessors and stores** - `0x802BF060`, `0x802BF420`,
   `0x802BFE10`, `0x802BFF30` (4 bodies). The cached-scale getter chosen by the
   `r13 - 0x47D8` enable byte, the four-float store at `+0x50..+0x5C`, the
   single float store at `+0x60`, and the initialiser that calls
   `0x802BE420(this)` and stores the `r2 - 0x3E98` default three times.

### Shared layout notes

`// inferred:` the records are RLYT layout/polygon data plus one screen-effect
object; the exports prove the offsets, the tags and the callee addresses only.

- the `RLAN` scan fills `{ block @ +0x00, pai1 @ +0x04, pat1 @ +0x08,
  pah1 @ +0x0C }` and each node is a tag word followed by its own byte size at
  `+0x04`;
- the effect object keeps its accumulated time at `+0x9C`, the last frame's
  delta at `+0xA0` and the wrap flag in bit `0x10` of `+0x09`;
- the three virtual getter copies share the `*object + 0x08` slot and differ
  only in the source offset of the three-word record.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802B2070` | `fn_802b2070` | 20 | high |
| `0x802B2090` | `fn_802b2090` | 24 | high |
| `0x802B2980` | `fn_802b2980` | 36 | high |
| `0x802B29B0` | `fn_802b29b0` | 100 | high |
| `0x802B2A50` | `fn_802b2a50` | 36 | high |
| `0x802B2B10` | `fn_802b2b10` | 16 | high |
| `0x802B4C40` | `fn_802b4c40` | 28 | high |
| `0x802B4D30` | `fn_802b4d30` | 116 | medium |
| `0x802B4DB0` | `fn_802b4db0` | 64 | high |
| `0x802B50A0` | `fn_802b50a0` | 12 | high |
| `0x802B50B0` | `fn_802b50b0` | 24 | high |
| `0x802B50D0` | `fn_802b50d0` | 52 | high |
| `0x802B5B10` | `fn_802b5b10` | 260 | medium |
| `0x802B5E10` | `fn_802b5e10` | 56 | high |
| `0x802BB0E0` | `fn_802bb0e0` | 8 | high |
| `0x802BB0F0` | `fn_802bb0f0` | 28 | high |
| `0x802BB110` | `fn_802bb110` | 56 | high |
| `0x802BB600` | `fn_802bb600` | 88 | high |
| `0x802BC670` | `fn_802bc670` | 104 | medium |
| `0x802BC6E0` | `fn_802bc6e0` | 12 | high |
| `0x802BD250` | `fn_802bd250` | 28 | high |
| `0x802BD270` | `fn_802bd270` | 28 | high |
| `0x802BD7C0` | `fn_802bd7c0` | 120 | high |
| `0x802BE8D0` | `fn_802be8d0` | 84 | medium |
| `0x802BE930` | `fn_802be930` | 84 | medium |
| `0x802BE990` | `fn_802be990` | 84 | medium |
| `0x802BF060` | `fn_802bf060` | 28 | medium |
| `0x802BF420` | `fn_802bf420` | 20 | high |
| `0x802BFE10` | `fn_802bfe10` | 8 | high |
| `0x802BFF30` | `fn_802bff30` | 64 | medium |

## Unknowns

1. **`0x801C04D0`, `0x801C1DB0`, `0x801C7520`.** The export proves the argument
   lists but not the semantics; they are GX/buffer-layer callees of the image.
2. **The `r2` defaults.** `r2 - 0x3F98`/`-0x3F94` (`0x802B4D30`), `r2 - 0x3F78`
   (`0x802B50D0`), `r2 - 0x3F48` (`0x802BB600`), `r2 - 0x3ED0` (`0x802BF060`)
   and `r2 - 0x3E98` (`0x802BFF30`) are read at runtime; their values are not
   invented.
3. **`0x802BC670`'s dropped heap argument.** Ghidra prints only
   `FUN_802b8cc0(size + 0xA0, 0x20)` although the callee takes
   `(size, align, heap)`; the body passes heap `0` (thread heap), matching the
   convention [`fn_802b9270`] documents, and the row is `medium`.
4. **`0x802BFF30`'s forwarded argument.** Ghidra prints `FUN_802be420()` with no
   arguments; the body forwards `this` because [`fn_802be420`] is already
   translated in this file and takes exactly that receiver.
5. **The three virtual getters.** The `+0x08` slot and the call argument are
   literal; the meaning of the returned record (and so of the three-word copies)
   is inferred from the sibling call sites.

## Deliberate skips

- **`0x802B1C90`** - calls the 76-byte base constructor `0x802AB550`, but the
  export prints the call with no arguments while the callee reads a source
  record; only the vtable store `0x8034FA80` is proved, so the body would be
  partial.
- **`0x802B2A20`, `0x802B2A80`..`0x802B2B40`** - ten more byte-identical
  `return a + b` helpers and a third copy of the four-byte record copy. They are
  real but duplicate [`fn_802b2a50`] / [`fn_802b2b10`], so the budget went to
  distinct layouts instead.
- **`0x802BC980`, `0x802BC9A0`, `0x802BC9C0`** - each is a `+0xB48` guard plus a
  call to `0x802BDF70` / `0x802BDFC0` / `0x802BDDF0`, but the export prints the
  calls with no arguments although those callees take four, four and one
  argument; the guard is proved, the call is not.
- **`0x802BE880`** - virtual call on `param_1` with no argument printed, then a
  `0x80253BE0(param_2, result)` tail; the receiver of the virtual call is
  missing from the export.
- **`0x802BE470`, `0x802BE4A0`, `0x802BE4D0`, `0x802BE500`, `0x802BFDA0`,
  `0x802BF420`'s neighbours `0x802BF3D0`/`0x802BF930`/`0x802BF650`** - the
  square-root/trig helpers and their callers are printed with dropped
  floating-point arguments, and there is no `call_f64` helper in
  `src/platform.rs`, so a byte-accurate body cannot be written honestly.
- **`0x802B4AE0`, `0x802B4B50`, `0x802B4DF0`, `0x802B4EE0`, `0x802BDE60`** -
  keyframe/spline and curve evaluators whose bodies are GQR
  `CONCAT44(0x43300000, x)` ladders; the export loses the GQR scaling, the same
  reason wave 73 skipped `0x80267420`.
- **`0x802B0680`, `0x802B20B0`, `0x802B2C70`, `0x802B6C80`, `0x802BC9E0`,
  `0x802BEA00` and the other 1.0-1.8 KB bodies** - layout/sound dispatchers and
  parsers far beyond this pass's budget.
- **The rest of the bucket** - `0x802B4940..0x802B4C60`, `0x802B5CC0`,
  `0x802B60B0`, `0x802B9700`, `0x802BB030..0x802BB580`, `0x802BBA30`,
  `0x802BBFD0..0x802BC560`, `0x802BC6F0..0x802BD460` (excluding the rows
  above), `0x802BE540..0x802BE5D0` and `0x802BEA00..0x802BFF70` are larger
  render-path, camera and sound bodies left for a dedicated pass.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (read from the TSV, not from prose).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/802bxxxx_FUN_802bxxxx.c` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/archive.rs` with a complete `# Original` / `# Arguments` /
  `# Returns` / `# Side effects` / `# Safety` / `# Confidence` doc block.
- No address was already in `ledger/functions.csv`; all 30 addresses came from
  the untranslated part of the `0x802B0000` bucket.
- `node tools/check-lane-csv.mjs wave79` reports 30 rows for
  `wave79-archive.csv` and `OK`.
- `cargo check --message-format=short` reports no error for
  `src/resource/archive.rs`.
