# Coverage - Wave 95 (resource::bres)

## How to read this file

This pass edits only `src/resource/bres.rs`. Every function below is a literal
reconstruction of one Ghidra body; the doc block above each body names the
original address, the byte count from
`work/ghidra/main-gb/export/functions.tsv` and the evidence file. Offsets are
relative to the object named in the row.

The `0x80230000` bucket is shared with several subsystems. Wave 90 took the
debug-console and script-stream band, wave 85 the BRRES material band and the
NW4R G3D math quartet. This pass closes almost everything that was left between
`0x80230780` and `0x80238270`: the console **text-stream object**, the console
**record registry**, the **exception report**, and the **engine message
queues**. None of these routines touches BRRES state; they are landed here
because the lane owns the address bucket.

- **high** - every load, store, branch and call is visible in the export; the
  only inference is the plain-English role sentence.
- **medium** - the export loses an argument to the Metrowerks prologue, or a
  field role / callee identity had to be reconstructed from a sibling.
- **low** - only the instruction sequence is certain; at least one argument
  register is not set by the caller and had to be modelled.

Inside the Rust file, `// observed:` marks facts read directly from the Ghidra
C, `// inferred:` marks a role reconstructed from neighbours, and `// unknown:`
marks an unresolved field.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/bres.rs` | `resource::bres` | 22 | 7,288 | 2 | 19 |

The file now holds **216** translated bodies (194 before this pass). One of the
22 new rows is `low`; no body in the file is a stub.

## What this pass covers

- **Text-stream object** (`0x80230780`, `0x802308D0`, `0x80231870`): the
  0x64-byte constructor with its `+0x4C..+0x60` defaults and class record, the
  advance probe that converts a virtual source count through the published
  `0x43300000` unsigned-integer-to-double idiom, and the copy/run/copy-back
  helper that keeps only the advanced pen words `+0x2C`/`+0x30`.
- **Script-line entry points** (`0x802319B0`, `0x80231AC0`): the variadic and
  explicit-`va_list` halves of the same call. Both format into the global
  staging buffer at `r13 - 0x4A28` (capacity `r13 - 0x5CE4`), clamp the length
  to `capacity - 1` and run the text with mode 1.
- **Text metrics** (`0x802322C0`): the bounding-box measure that seeds four
  edges from the shared small-data word, lays out quads through `0x80231CF0`,
  and widens the box with two `min` and two `max` merges.
- **Console geometry** (`0x80233A70`, `0x80233D10`): the framebuffer allocator
  (the same `0x803DA120` record wave 90's `0x802331A0` writes, plus the video
  mode pick, the `(w + 0xF & ~0xF) * h * -2` arena carve and the retrace spin)
  and the console record constructor with its descending sorted insert into
  `r13 - 0x4A10`.
- **Exception report** (`0x80234B60`, `0x80234F40`, `0x80235180`,
  `0x802352A0`): the mask-selected dump (`0x803DA8B8`: main, GPR, SRR0 map, GPR
  map, FPR, 16-frame back-chain walk), the context header with the exception
  name table at `0x8034D024` and the FPSCR flag groups, the 31-register dump
  with symbol lookup, and the 32-FPR dump in three columns.
- **Message queues** (`0x80235DC0`, `0x80236350..0x80237610`): the script-node
  opener (`DVDFastOpen` / buffered reader selection), the formatter entry points
  per heap (bounded `snprintf` on queue A, `0x802E4390` on both), the two record
  sinks with their 0x18-byte header and `ut::List` append, and the two queue
  drains that feed the stream and free exhausted records.
- **One G3D wrapper** (`0x80238270`): the double-scaling forward that wave 90
  deliberately left behind.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80230780` | `fn_80230780` | 92 | high |
| `0x802308D0` | `fn_802308d0` | 124 | medium |
| `0x80231870` | `fn_80231870` | 320 | medium |
| `0x802319B0` | `fn_802319b0` | 272 | medium |
| `0x80231AC0` | `fn_80231ac0` | 172 | high |
| `0x802322C0` | `fn_802322c0` | 288 | medium |
| `0x80233A70` | `fn_80233a70` | 492 | medium |
| `0x80233D10` | `fn_80233d10` | 424 | medium |
| `0x80234B60` | `fn_80234b60` | 988 | medium |
| `0x80234F40` | `fn_80234f40` | 572 | medium |
| `0x80235180` | `fn_80235180` | 288 | medium |
| `0x802352A0` | `fn_802352a0` | 1032 | medium |
| `0x80235DC0` | `fn_80235dc0` | 232 | medium |
| `0x80236350` | `fn_80236350` | 200 | medium |
| `0x80236420` | `fn_80236420` | 172 | medium |
| `0x80236700` | `fn_80236700` | 200 | medium |
| `0x80236880` | `fn_80236880` | 396 | medium |
| `0x802370E0` | `fn_802370e0` | 200 | medium |
| `0x802371B0` | `fn_802371b0` | 172 | medium |
| `0x80237490` | `fn_80237490` | 200 | medium |
| `0x80237610` | `fn_80237610` | 396 | medium |
| `0x80238270` | `fn_80238270` | 56 | low |

## The prologue-stub recovery

Metrowerks emits `bl _savegpr0_26` (and its siblings) in every non-leaf
prologue. Ghidra models those stores as a call with a 64-bit return, so bodies
in this band appear to take their pointer from
`FUN_802DCD68/6C/70/74()`:

```
uVar4 = FUN_802DCD70();      // _savegpr0_26: stores r26..r31, returns nothing
puVar6 = (undefined4 *)uVar4;
```

The stub does not touch `r3`/`r4`, so the "returned" pair is the incoming
argument pair. Six bodies rely on that reading, and each names its evidence in
the doc block:

- `0x80233D10` recovers the record pointer from `r3`; its only caller
  `0x802D7570` passes the freshly allocated `(n + 1) * stride + 0x2C` block, and
  the first store is `record + 0x2C` into `record + 0x00`.
- `0x80234B60` and `0x80234F40` recover `(kind, context)`; the `kind < 0x11`
  test survives in the export as the 64-bit compare `r3:r4 < 0x1100000000`, and
  `0x802348F0` calls `0x80234B60` with exactly `(kind, context, dsisr, dar)`.
- `0x80235180` recovers `r3` as the context; its only caller passes its own
  context argument.
- `0x80235DC0` recovers `r3`/`r4`; `param_1` and `param_2` are otherwise unused
  in the body while `*param_3` and `param_4` are live.
- `0x80236350`/`0x80236420` (and the queue-B twins) recover the owner and tag,
  which the sink stores in the record header it was told to fill.

## Deliberate skips

- **Packed-vector math (`0x802380A0..0x80239780`, `0x8023F000..0x8023FE50`):**
  unchanged from waves 85 and 90. These bodies decode or encode through
  `unaff_GQR0`/`unaff_GQR3`/`unaff_GQR5`, the paired-single quantisation
  registers the caller sets. `0x802384B0` and `0x80238110` are typical: the
  GQR=0 branch stores raw floats, the Q-format branch stores 8- or 16-bit
  fixed-point halves, and the two are not interchangeable. `0x8023F140` mixes
  a 15-bit fixed-point colour lerp with a GQR conversion of the factor, and
  `0x8023F4E0` keys a table search by the quantised argument. Reconstructing
  any of them would require guessing the caller's GQR.
- **Math helpers with unresolvable operands:** `0x80237A60`, `0x80237B00`,
  `0x80237BD0`, `0x80237C40`, `0x80237CA0`, `0x80237D10`, `0x80237ED0` reduce
  an angle with the small-data `2π` constant and then index a coefficient table
  through the GQR conversion; the same objection applies.
- **`0x80237BA0` (36 bytes):** a Newton refinement of `1 / sqrt(x)`. The only
  blocker is that Ghidra renders the square root as the `SQRT` intrinsic, and
  this `no_std` crate has no `f64` square root (`f64::sqrt` is std-only) and no
  libm dependency. Substituting `pow(x, 0.5)` would change the rounding, so the
  body stays untranslated rather than approximating it.
- **`0x802337B0` (glyph blitter, 704 bytes):** still skipped. The 5x7 font
  expansion builds each 16-bit pixel pair from four rotated bits of a 30-bit
  font row and then re-normalises with `uVar12 >> 1 & 0x7E000000` and a
  three-lookup `0x8034CF30` chain. The export does not contain the font table,
  and one wrong shift silently corrupts every glyph.
- **`0x802348F0` (console frame loop, 400 bytes):** still skipped. Its
  "argument" is read from `FUN_802DCD64` (`_savegpr0_23`) although the body is
  declared `void f(void)`, and the tail is a `do { ... } while (true)` loop that
  divides `_DAT_800000f8` by 4000 to convert to timebase ticks. The tick
  semantics are not in the C, and the loop has no modelled exit.
- **`0x80235AE0` (script parser, 716 bytes):** still skipped. The decompiler
  could not structure it: `LAB_80235b18`, `LAB_80235d20`, `unaff_r30` reused as
  a loop counter across `goto`s, and three calls whose argument lists are empty
  because the values never left their registers. `0x80235DC0` calls it by
  address.
- **The four variadic queue entry points (`0x80236230`, `0x802365E0`,
  `0x80236FC0`, `0x80237370`, 288 bytes each):** these copy the eight GPR and
  eight FPR save slots into a frame-local `va_list` descriptor and then tail
  into the formatters that this pass *did* translate. Translating them would
  mean inventing a Rust-side varargs ABI for a descriptor the original builds
  from its own register spill area, so they stay address calls.
- **`0x802319B0`'s frame-carved fallback buffer:** when
  `r13 - 0x4A28` is null the original allocates exactly `capacity` bytes below
  its declared locals. Ghidra never shows that capacity, so the shared helper
  uses a fixed 0x800-byte local and clamps; every shipped call uses the global
  buffer. This is the only place where a new body approximates an allocation,
  and it is marked `// partial:` in the code.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (re-read from the TSV, not copied from prose).
- Every evidence file exists at the cited repo-relative path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/bres.rs`; the file now holds 216 such bodies.
- No address in `ledger/agents/wave95-bres.csv` was already in
  `ledger/functions.csv`, and the validator reports 265 unique addresses across
  the ten wave-95 CSVs.
- `cargo check --message-format=short` reports no error and no warning for
  `src/resource/bres.rs`.
- `node tools/check-lane-csv.mjs wave95` reports `OK`.

`cargo check` proves the Rust is well-formed and type-correct; it does not run
the target code. The pointer arithmetic models the Wii's 32-bit big-endian
PowerPC and must only execute on a target-compatible runtime with the original
memory image mapped.
