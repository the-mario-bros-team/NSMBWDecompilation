# Coverage — range lane `r80180_801c0` (`0x80180000..0x801C0000`)

## How to read this file

- **Scope** — this document covers only the DOL entry-address range
  `0x80180000..0x801C0000`, owned by range lane 12 in `docs/LANES.md`. The
  staging file `src/subsystems/r80180_801c0.rs` is now empty: all 20 translated
  functions have been promoted to semantic modules.
- **Addresses** are virtual main.dol addresses, not file offsets. The image
  base is `0x80004000`.
- **Evidence** means the Ghidra pseudocode under
  `work/ghidra/main-gb/export/`; the row's file name is given in the table.
  Nothing here was inferred from outside the shipped binary.
- **Status** follows `docs/RUST_STYLE.md`: `translated` = complete control
  flow and all calls accounted for; `partial` = entry/exit and major effects;
  `stub` = address/role only; `promoted` = moved to a semantic module, with the
  destination named in the row.
- **Confidence** is strength of evidence for the translation, not code quality.
  `medium` marks a function whose bytes are exact but whose role or object
  layout is inferred.
- **Rust names** are the original-entry form `fn_80xxxxxx` because the binary
  carries no symbol names for them. Do not read them as original function
  names; read the `# Original` block in each function.
- Start with the **summary** and **clusters**, then use the table to jump to a
  function. The **next targets** section lists the highest-value untranslated
  neighbours.

## Summary

| metric | value |
|---|---:|
| functions in range (`functions.tsv`) | 872 |
| bytes in range | 211,104 |
| excluded: body < 16 bytes | 111 (928 bytes) |
| excluded: runtime-lane addresses | 8 (2,232 bytes) |
| **eligible functions** | **753 (207,944 bytes)** |
| translated in this pass | 20 (1,340 bytes) |
| promoted to semantic modules | 20 (four → `src/runtime/ids.rs`, four → `src/audio/ax.rs`, twelve → `src/ui/lyt.rs`) |
| remaining staged in this range module | 0 |
| translated share of eligible | 2.66 % of functions, 0.64 % of bytes |
| translated share of whole range | 2.29 % of functions, 0.63 % of bytes |
| staged share of eligible | 0.00 % of functions, 0.00 % of bytes |

The eight runtime-lane exclusions are `0x801AA800`, `0x801AB150`,
`0x801AC990`, `0x801A9D40`, `0x801B3CC0`, `0x801B8970`, `0x801B89E0`,
`0x801BB930`. All are at least 16 bytes, so the two exclusion sets are
disjoint and the subtraction above is exact.

## Promoted functions - first pass (8, `runtime` and `audio`)

These functions were translated in this range lane and then moved, without
behaviour changes, to their semantic modules. Their ledger rows point at the
destination file.

| address | Rust function | Ghidra name | role | destination | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80192820` | `fn_80192820` | `FUN_80192820` | allocate a slot for an id | `src/runtime/ids.rs` | high | `80192820_FUN_80192820.c` |
| `0x80192880` | `fn_80192880` | `FUN_80192880` | id → slot lookup | `src/runtime/ids.rs` | high | `80192880_FUN_80192880.c` |
| `0x801928A0` | `fn_801928a0` | `FUN_801928a0` | free by slot | `src/runtime/ids.rs` | high | `801928a0_FUN_801928a0.c` |
| `0x801928D0` | `fn_801928d0` | `FUN_801928d0` | free by id | `src/runtime/ids.rs` | high | `801928d0_FUN_801928d0.c` |
| `0x8019F180` | `fn_8019f180` | `FUN_8019f180` | set audio-DMA start bit | `src/audio/ax.rs` | high | `8019f180_FUN_8019f180.c` |
| `0x8019F1A0` | `fn_8019f1a0` | `FUN_8019f1a0` | audio-DMA bytes left | `src/audio/ax.rs` | high | `8019f1a0_FUN_8019f1a0.c` |
| `0x8019F1B0` | `fn_8019f1b0` | `FUN_8019f1b0` | audio-DMA source address | `src/audio/ax.rs` | high | `8019f1b0_FUN_8019f1b0.c` |
| `0x8019F1D0` | `fn_8019f1d0` | `FUN_8019f1d0` | audio-DMA programmed length | `src/audio/ax.rs` | high | `8019f1d0_FUN_8019f1d0.c` |

## Promoted functions - `ui/lyt.rs` pass (12)

These were the last staged functions in this lane. They moved, without
behaviour changes, to `src/ui/lyt.rs`, which already owned the layout core:
the six `BRLAN` animation-transform helpers are in its `BRLAN` section and the
six NW4R intrusive-list primitives are in its clearly labelled `list` section.
Their ledger rows now read `module=ui` and `rust_file=src/ui/lyt.rs`.

| address | Rust function | Ghidra name | role | status | confidence | destination | evidence |
|---|---|---|---|---|---|---|---|
| `0x80182060` | `fn_80182060` | `FUN_80182060` | texture-SRT array address in the animation work buffer | translated | high | `src/ui/lyt.rs` (`BRLAN`) | `80182060_FUN_80182060.c` |
| `0x80182080` | `fn_80182080` | `FUN_80182080` | next section address (texture pattern) | translated | high | `src/ui/lyt.rs` (`BRLAN`) | `80182080_FUN_80182080.c` |
| `0x801820A0` | `fn_801820a0` | `FUN_801820a0` | indirect-material section address (sums packed counts) | translated | medium | `src/ui/lyt.rs` (`BRLAN`) | `801820a0_FUN_801820a0.c` |
| `0x80182100` | `fn_80182100` | `FUN_80182100` | grow 0x20-byte record array, zero-fill tail | translated | high | `src/ui/lyt.rs` (`BRLAN`) | `80182100_FUN_80182100.c` |
| `0x80182190` | `fn_80182190` | `FUN_80182190` | grow 4-byte record array, fill `01 04 3C 00` | translated | medium | `src/ui/lyt.rs` (`BRLAN`) | `80182190_FUN_80182190.c` |
| `0x80183500` | `fn_80183500` | `FUN_80183500` | push a descriptor onto an object's embedded list | translated | medium | `src/ui/lyt.rs` (`BRLAN`) | `80183500_FUN_80183500.c` |
| `0x8018A590` | `fn_8018a590` | `FUN_8018a590` | insert node after a head/sentinel node | translated | high | `src/ui/lyt.rs` (`list`) | `8018a590_FUN_8018a590.c` |
| `0x8018A5C0` | `fn_8018a5c0` | `FUN_8018a5c0` | initialise intrusive list header with link offset | translated | high | `src/ui/lyt.rs` (`list`) | `8018a5c0_FUN_8018a5c0.c` |
| `0x8018A5E0` | `fn_8018a5e0` | `FUN_8018a5e0` | append node at tail | translated | high | `src/ui/lyt.rs` (`list`) | `8018a5e0_FUN_8018a5e0.c` |
| `0x8018A650` | `fn_8018a650` | `FUN_8018a650` | unlink node and clear its links | translated | high | `src/ui/lyt.rs` (`list`) | `8018a650_FUN_8018a650.c` |
| `0x8018A6C0` | `fn_8018a6c0` | `FUN_8018a6c0` | get next node (or head when null) | translated | high | `src/ui/lyt.rs` (`list`) | `8018a6c0_FUN_8018a6c0.c` |
| `0x8018A6E0` | `fn_8018a6e0` | `FUN_8018a6e0` | get node by index | translated | high | `src/ui/lyt.rs` (`list`) | `8018a6e0_FUN_8018a6e0.c` |

All evidence paths are relative to `work/ghidra/main-gb/export/`.
Promoted Rust bodies live in `src/runtime/ids.rs`, `src/audio/ax.rs` and
`src/ui/lyt.rs`. No translated body is left in
`src/subsystems/r80180_801c0.rs`.

## Promotion status

| date | pass | moved | remaining | gate |
|---|---|---|---|---|
| 2026-11-01 | staging → semantic | 4 id/slot functions → `runtime/ids.rs`, 4 audio-DMA functions → `audio/ax.rs` | 12 NW4R list/BRLAN helpers | `cargo check` 0, `doc-lint` 0 |
| 2026-11-01 | staging → `ui/lyt.rs` | 12 `BRLAN`/list helpers → `src/ui/lyt.rs` | 0 | `cargo check` 0, `doc-lint` 0 |

The move cut each definition and its doc block from the range module; no copy
was left behind. The destination files previously contained only placeholder
headers.

## Clusters

### A. NW4R layout-animation transform helpers (`0x80182060..0x80182190`, `0x80183500`)

All six functions now live in the `BRLAN` section of `src/ui/lyt.rs`; the
offsets below are read there.

**Evidence for the role.** Caller `0x8017DCC0` (outside this lane) walks a
`BRLAN` resource and switches on four big-endian tags observed as:

| constant | bytes | tag |
|---|---|---|
| `0x524C4D43` | `RLMC` | material colour |
| `0x524C5453` | `RLTS` | texture SRT |
| `0x524C5450` | `RLTP` | texture pattern |
| `0x524C494D` | `RLIM` | indirect material |

For `RLTS` it calls `FUN_80182060` and stores floats at
`result + material * 0x14 + texture * 4`. For `RLIM` it calls
`FUN_801820a0` with the same index shape. `FUN_80182190` and `FUN_801823E0`
are called for the same object (`param_3`) in that dispatch.

**What the functions do.** The animation-transform object holds a work-buffer
pointer at `+0x58` and two packed count words at `+0x4C`/`+0x50`. The five
translated functions either compute the start of a section from those counts
(`0x80182060`, `0x80182080`, `0x801820A0`) or grow a record array and update the
packed count (`0x80182100`, `0x80182190`). `0x80183500` is a different object
(caller `0x8017DA20`) that keeps an embedded intrusive list at `+0x18`/`+0x1C`.

**Internal call graph (this lane only).**

```text
fn_80183500 ──> fn_8018a590            (cluster B)
fn_80182060, fn_80182080, fn_801820a0  leaf reads
fn_80182100 ──> 0x800046B4 (memset)    (boot/CRT lane)
fn_80182190   leaf writes
```

### B. NW4R intrusive list helpers (`0x8018A590..0x8018A6E0`)

All six functions now live in the `list` section of `src/ui/lyt.rs`. Two
related but distinct list shapes appear here:

1. **Offset-link list** (`0x8018A5C0`, `0x8018A5E0`, `0x8018A650`,
   `0x8018A6C0`, `0x8018A6E0`). The header is
   `{ head, tail, u16 count, u16 link_offset }`; each node embeds a two-word
   link at `link_offset`. Word `+0` points to the older neighbour, word `+4`
   to the newer one. Append and remove keep both directions and the count in
   sync.
2. **Sentinel list** (`0x8018A590`). The caller passes a word that holds a
   head/sentinel node pointer. The function splices a new node immediately
   after that head, and increments a separate count word. `0x80183500` is the
   only in-range caller and shows the intended use: `+0x18` is the count and
   `+0x1C` is the embedded head node.

The two shapes share the same link-word convention, which is why they are one
cluster. No function in this cluster allocates or frees nodes; lifetime is the
caller's job.

### C. 16-slot id/slot table (`0x80192820..0x801928D0`) — promoted to `runtime/ids.rs`

All four functions and both tables now live in `src/runtime/ids.rs`. The
description below is the evidence kept with this range.

Two adjacent byte tables in BSS:

- `0x80378078..0x80378087` — 16 forward slots; `0xFF` = free.
- `0x80378088..0x80378187` — 256 inverse entries, id → slot; `0xFF` = unused.

`fn_80192820` scans for a free slot, stores the id forward and the slot
inversely. `fn_80192880` reads the inverse map. `fn_801928a0` frees by slot;
`fn_801928d0` frees by id and bails out on a negative inverse entry. The code
performs no bounds checks: freeing an empty slot reads `0xFF` as a signed
char (-1) and clears the byte before the inverse table, which is the last byte
of the forward table. That adjacent write is preserved.

Callers outside this range: `0x80192E60`, `0x80192C70`, `0x80193E40`,
`0x80193F00` (next targets below).

### D. Audio-DMA MMIO accessors (`0x8019F180..0x8019F1D0`) — promoted to `audio/ax.rs`

All four functions now live in `src/audio/ax.rs`. These four functions are the
read/start side of the DSP audio-DMA channel in the `0xCC005000` block.
Dolphin and libogc name the registers:

| address | width | use here |
|---|---|---|
| `0xCC005030` | 16 | source address high; bits 0..12 are address bits 16..28 |
| `0xCC005032` | 16 | source address low; bits 5..15 are address bits 5..15 |
| `0xCC005036` | 16 | bits 0..14 = length in 32-byte blocks, bit 15 = start |
| `0xCC00503A` | 16 | bits 0..14 = remaining 32-byte blocks |

Neighbouring evidence (not translated here): `0x8019F100` programs the address
and length, `0x801AC350` clears the start bit, `0x8019F1F0` initialises the AI
(`0xCD006C00`) and audio timing, and `0x8019F090` divides 1,920,000 by a
sample rate.

## Selection method (why only 20)

The range contains 753 eligible functions, including very large state machines
(`0x80180780` 4,484 B, `0x801A0E90` 2,492 B, `0x801BFEC0` 3,448 B) and code
that depends on unresolved `r13` small-data globals or paired-single GQR
state. This pass deliberately picked functions that are:

1. at least 16 bytes (lane rule);
2. outside the eight runtime-lane addresses;
3. self-contained: no `unaff_r13`, no GQR/paired-single reconstruction, and at
   most one external call;
4. verifiable end-to-end against a single Ghidra file.

The result was 20 functions in four clusters, not a representative sample of
the range's byte count. All 20 are now promoted; none is staged. The
remaining 733 eligible functions are untouched.

## Uncertainties and limitations

The bodies discussed below now live in `src/ui/lyt.rs` (`BRLAN` and `list`
sections); the notes stay with this range because they come from in-range
callers.

- **Layout object semantics (cluster A).** The bit positions and strides are
  exact, but the binary does not name the individual counts at `+0x4C`/`+0x50`.
  Section names ("material colour", "texture SRT", "texture pattern",
  "indirect material") come from the `BRLAN` tags handled by caller
  `0x8017DCC0`, not from the functions themselves. Treat them as inferred.
- **`01 04 3C 00` pattern (`fn_80182190`).** The bytes are written exactly, but
  no consumer is in this range, so their meaning is unknown. `0x3C` is 60 but
  that is not proven to be a frame count.
- **Count truncation.** Both grow functions store four-bit counts while using
  the full 32-bit argument as a loop bound. Passing more than 15 writes more
  records than the stored count describes. This is preserved, not fixed.
- **`fn_80183500` descriptor layout.** The wrapper only depends on the first
  two words of the node; the `0x10`-byte stride and the fields at `+8`/`+0xC`
  come from caller `0x8017DA20` and are not validated here.
- **List header race.** `fn_8018a5e0`/`fn_8018a650` update count and links
  without interrupt masking. The original does the same; any caller requiring
  atomicity must mask externally.
- **Handle table initialisation.** The promoted functions in
  `src/runtime/ids.rs` assume the tables were already filled with `0xFF`. The
  initialiser is not in this range, so it does not show who first fills the two
  arrays.
- **Audio DMA readback asymmetry.** `0x8019F100` writes the source address with
  masks `0xE000`/`0x1F`, while the promoted `fn_8019f1b0` in `src/audio/ax.rs`
  reads back `0x1FFF`/`0xFFE0`; the low five address bits cannot be
  reconstructed from this view and come back as zero. The register names are
  the common Dolphin/libogc names, not strings in the binary.
- **MMIO functions are not host-safe.** The promoted audio-DMA accessors are
  exact PowerPC/volatile translations; running them on a host requires the
  `platform` helpers to be backed by an emulator or a mock.

## Next targets

Ordered by evidence already gathered and expected payoff. Sizes are from
`functions.tsv`.

### Finish cluster A (BRLAN transform)

| address | size | why |
|---|---:|---|
| `0x801823E0` | 436 | called by `0x8017DCC0` for `RLMC`; writes one channel value per material/texture |
| `0x801822F0` | 132 | `RLTP` apply helper; same 0x20-byte record array as `0x80182100` |
| `0x80182380` | 84 | copies eight words into a 0x20-byte record, then calls `0x801C7160` |
| `0x801819C0` | 1,688 | largest function in the immediate layout group; likely the transform/animation updater |

### Finish cluster B (intrusive lists)

| address | size | why |
|---|---:|---|
| `0x8018A4B0` | 132 | clears every node and conditionally frees the header |
| `0x8018A540` | 24 | removes the head node, returns the next |
| `0x8018A730` | 60 | installs a vtable pointer after `0x8018AA30` |
| `0x8018AA30` | 68 | base constructor: vtable + three globals, clears two words |
| `0x8018AA80` | 64 | conditional free wrapper around `0x802B9280` |

### Continue the promoted handle table (`runtime/ids.rs`)

The four table operations moved to `src/runtime/ids.rs`; their callers stay in
this range until translated.

| address | size | why |
|---|---:|---|
| `0x80192E60` | 556 | calls alloc (`0x80192820`) and free-by-slot (`0x801928A0`) |
| `0x80192C70` | 168 | calls free-by-id (`0x801928D0`) |
| `0x80193E40` | 164 | calls lookup (`0x80192880`) and free-by-slot |
| `0x80193F00` | 240 | calls free-by-id; likely table teardown |

### Continue the promoted audio/AX cluster (`audio/ax.rs`)

The four DMA accessors moved to `src/audio/ax.rs`; the neighbouring writer and
AI init code below is still untranslated.

| address | size | why |
|---|---:|---|
| `0x8019F0B0` | 68 | atomic swap of an audio state word (uses `0x801B1140/1180`) |
| `0x8019F100` | 100 | programs audio-DMA address and length (writes the registers read here) |
| `0x8019F130` | 28 | small neighbour between the DMA and AI code |
| `0x8019F1F0` | 384 | AI/DSP init: bus-clock-derived timers, `0xCD006C00`, IRQ hook |
| `0x8019F370` | 168 | IRQ handler registered by `0x8019F1F0` |

### Whole-range gaps (no translation yet)

- `0x80180020..0x801819C0` — animation-resource constructors/parse helpers;
  `0x80180240` dispatches `pan1`/`pic1`/`txt1`/`wnd1`/`bnd1` and allocates
  `0xD4..0x104`-byte objects.
- `0x80183F40..0x8018A400` — unclassified NW4R helpers, many small accessors.
- `0x8018B000..0x80190000` — large resource/transform code with GQR-heavy
  bodies; defer until paired-single lowering is available.
- `0x801A0000..0x801B0000` — audio layer: `0x801A8400` returns `0x9600` and
  `0x801A8F20` returns `0xC800` (38,400 / 51,200, consistent with sample-rate
  limits); the exact AX class boundaries still need xref work.
- `0x801B0000..0x801C0000` — WPAD/KPAD and NAND: VI line polling via
  `0xCC00202C`, `shared2` NAND access, and the 2,060-byte `0x801BBBD0` input
  update. The runtime lane owns eight addresses inside this block; coordinate
  before starting it.

## Verification notes

- **Promotion gates (2026-11-01, `ui/lyt.rs` pass).** `cargo check` and
  `node tools/doc-lint.mjs src` both pass (0 errors, 0 violations). The crate
  check was rerun after concurrent edits in other lanes settled; no diagnostic
  referenced this lane's files. While those lanes were still editing, the moved
  code was additionally compiled in an isolated scratch crate (platform plus
  `ui/lyt.rs` and the two range modules) with 0 errors.
- All 20 functions were re-read from their Ghidra files during the original
  translation; none was reconstructed from memory or from a symbol name. All 20
  promoted functions were moved without changing their bodies or their
  `# Original` blocks; the only edits were import and helper-call adaptation
  inside `src/ui/lyt.rs`.
- The promotion edits for this pass only touch:
  `src/subsystems/r80180_801c0.rs`, `src/subsystems/r80140_80180.rs`,
  `src/ui/lyt.rs`, the two coverage docs, and `ledger/agents/r80180_801c0.csv`
  / `r80140_80180.csv`. The earlier pass's destinations (`src/audio/ax.rs`,
  `src/runtime/ids.rs`) were not modified. Shared files (`Cargo.toml`,
  `src/lib.rs`, `src/platform.rs`, `src/subsystems/mod.rs`, module
  declarations) are untouched.
