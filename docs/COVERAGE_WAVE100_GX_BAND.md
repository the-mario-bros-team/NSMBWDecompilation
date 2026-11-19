# Coverage - Wave 100 (GX / render band, `0x80280000` bucket)

## How to read this file

This page covers the Wave 100 pass over the `0x80280000..0x8028FFFF` bucket of
the GX/render lane (`src/render/gx_band.rs`). Addresses are virtual addresses of
the unrelocated `main.dol` (image base `0x80004000`); byte counts come from
`work/ghidra/main-gb/export/functions.tsv`, never from prose. The module already
held 248 translated entries from waves 1-98; this pass adds 18 more (266).

`status` is `translated` for every row; `confidence` follows the body's
`# Confidence` line. Every row cites the exact export file it was read from.

The bucket started this pass with 38 untranslated entries out of 158. Eighteen
are translated here; the remaining 20 are listed under "Deliberate skips".

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/gx_band.rs` | `render::gx_band` | 18 | 7020 | 2 | 16 |

## What this pass covers

- **Texture-grid flush family** (`0x80280910`, `0x80280BA0`, `0x80281C90`). The
  three passes the pool driver `0x80283300` runs over every linked grid. The
  flush walks the `+0xA2` "changed" mask (bits `0x004`, `0x008`, `0x010`,
  `0x020`, `0x080`, `0x100`) and clears each bit after running the matching
  per-cell call; the apply pass has two phases keyed on the `+0x9E`/`+0x9F`
  bytes and releases textures in mode `1` or `2`; the uv push scales the batch
  factor from `+0xE8` by the row phase at `+0x2C` and sends the pair to
  `0x80263A70`. All three `ldexpf` blocks in these exports are dead entry/exit
  register spills. The three functions share the field map documented in
  section 2 of the module.
- **Ramp and pitch tables** (`0x802838A0`, `0x802837C0`, `0x802835B0`). Two
  samplers over the three 257-float ramps at `0x802FBAF4`/`0x802FBEF8`/
  `0x802FC2FC` (pointer array `0x8034F288`), including the `+0x200` midpoint
  normalisation and the two output windows, plus the 1/3072-octave frequency
  ratio built from the semitone table at `0x802FA7B0` and the 1/256-semitone
  table at `0x802FA7E0`.
- **Screen-box wrap** (`0x80282040`). The band/cap branch tree on
  `|f2 - 1|` vs `|f1|` with the `0.35/0.15/0.85` slopes, the `-0.12` bias and
  the final `+1.0`.
- **Pool list maintenance** (`0x80283420`, `0x80286D80`). The level-ordered
  re-insert of one `0x104`-byte chunk (node `chunk+0xFC`, level `chunk+0xA8`)
  followed by the `0x10` filter on every visible neighbour, and the two-way
  bubble sort of one `0x1C`-byte bucket list driven by the caller's comparator.
- **Material timeline** (`0x80283EC0`, `0x80284700`, `0x80284F30`). The
  `+0x354` manager lock/unlock dance, the teardown of `+0xCC`/`+0xCD`/`+0x114`,
  the four state words at `+0xE8..+0xF4`, the eight-argument create/rebuild
  calls, and the `(e8 * state[+0xC]) / 1000` duration mode.
- **Child spawn and scroll plane** (`0x80287930`, `0x80286CA0`). The
  descriptor-guarded spawn that links the parent, composes the node matrix and
  copies the ten record bytes into `+0xFC..+0x105`, and the three-word plus
  `3x4` matrix fill of the scroll descriptor at `+0xC020`.
- **Keyframe samplers** (`0x8028DC70`, `0x8028DFC0`). One shared index resolver
  (five clamp/wrap/ping-pong/fractional arms) plus the byte and word channel
  packers, using the `0x8034F3D8` stride and `0x8034F4D8` top-bit tables that
  the image fills with `popcount(key)` and `highest_bit(key)`.
- **Grid flag setters** (`0x80284040`, `0x80284050`). Two 8-byte tail calls that
  reach the grid header at `+0x110` and set the `+0xDC`/`+0xD0` bytes.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80280910` | `fn_80280910` | 648 | medium |
| `0x80280BA0` | `fn_80280ba0` | 696 | medium |
| `0x80281C90` | `fn_80281c90` | 240 | medium |
| `0x80282040` | `fn_80282040` | 416 | medium |
| `0x80283420` | `fn_80283420` | 264 | medium |
| `0x802835B0` | `fn_802835b0` | 420 | medium |
| `0x802837C0` | `fn_802837c0` | 224 | medium |
| `0x802838A0` | `fn_802838a0` | 148 | medium |
| `0x80283EC0` | `fn_80283ec0` | 160 | medium |
| `0x80284040` | `fn_80284040` | 8 | high |
| `0x80284050` | `fn_80284050` | 8 | high |
| `0x80284700` | `fn_80284700` | 412 | medium |
| `0x80284F30` | `fn_80284f30` | 440 | medium |
| `0x80286CA0` | `fn_80286ca0` | 136 | medium |
| `0x80286D80` | `fn_80286d80` | 384 | medium |
| `0x80287930` | `fn_80287930` | 732 | medium |
| `0x8028DC70` | `fn_8028dc70` | 840 | medium |
| `0x8028DFC0` | `fn_8028dfc0` | 844 | medium |

## Deliberate skips

The bucket still contains 20 untranslated entries. Each was read in full before
being rejected.

- **Live paired-single vertex/emitter bodies** (`0x80289410`, `0x8028A6F0`,
  `0x8028B6C0`, `0x8028CEE0`, `0x8028E890`, `0x8028ED30`, `0x8028F2D0`,
  `0x8028F860`, `0x8028FEA0`, `0x80288490`, `0x802888E0`, `0x80288E30`,
  `0x80288390`, `0x8028E310`, `0x80287C10`, `0x802821E0`, `0x802850F0`). In
  these the `ldexpf`/`unaff_GQR0` arms *decide* the stored value: packed vertex
  bytes/halfwords (`0x8028E310`, `0x802888E0`), GQR-scaled matrices
  (`0x80288490`, `0x80288390`), or the 40-170 GQR pairs of the large emitters.
  A faithful body needs a raw `psq` pass, exactly as recorded in
  `docs/COVERAGE_WAVE98_GX_BAND.md` for the previous bucket.
- **`0x8028C7A0`** (1,468 bytes). The third keyframe sampler, but its frame
  selector uses a binary search over a self-relative key table plus a
  hash-driven variant (`countLeadingZeros` on the step parity, a folded
  `0x371097E7`-style hash) and writes through three different base pointers
  (`+0x88`/`+0x8C`/`+0x90`). The hash fold is printed with overlapping
  `byte`/`ushort` aliases on the same stack slots; a wrong reconstruction would
  silently change which key wins, so it is deferred rather than guessed.
- **`0x802830A0`** (444 bytes). The pool insert that calls `0x80280E60` and then
  performs the same sorted insert as `0x80283420`. Its node is fetched from
  `u32[owner+0x14]` while the loop terminator is `u32[owner+0x08]`; the two
  disagree with the list layout used by the rest of the family (`0x80282FF0`,
  `0x80283260`, and `0x80283420` here), and the export's `0x802DCD64` stub hides
  the real `r4`. Deferred until the pool list header is proven from the raw
  stream.
- **`0x80283BD0`** (460 bytes). A pure pointer-fixup walk over nested
  self-relative word offsets that writes a variable-length record to `r4`; the
  export renders the source as overlapping `ushort *`/`undefined4 *` casts, the
  same defect that blocked `0x80294B30` in wave 98.

## Verification

- Every address and byte size matches `functions.tsv` (18 rows, 7,020 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/render/gx_band.rs`.
- No address was already in `ledger/functions.csv`; the module now holds 266
  translated entries and this pass added 18.
- `node tools/check-lane-csv.mjs wave100` prints `OK` (18 rows for this lane, no
  duplicate, malformed or already-merged row).
- `cargo check --message-format=short` reports no error or warning in
  `src/render/gx_band.rs`.
- Constants used by this pass were read from `work/dol/main.elf` through the
  ELF program headers (the `sdata2` window `0x8042EB08..0x8042EB60`, the two
  ratio tables at `0x802FA7B0`/`0x802FA7E0`, the ramp pointer array at
  `0x8034F288`, and the channel tables at `0x8034F3D8`/`0x8034F4D8`); the
  values are listed in the module header.
