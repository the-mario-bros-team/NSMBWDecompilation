# Coverage - Wave 90 (resource::bres)

## How to read this file

This pass edits only `src/resource/bres.rs`. Every function below is a literal
reconstruction of one Ghidra body; the doc block above each body names the
original address, the byte count from
`work/ghidra/main-gb/export/functions.tsv` and the evidence file. Offsets are
relative to the object named in the row.

The `0x80230000` bucket is shared: wave 85 took the BRRES material band and the
NW4R G3D math quartet, so this pass takes the **engine debug-console and
script-stream band** that sits in the same bucket (`0x802307E0..0x80237A00`).
Those routines do not touch BRRES state. They are landed here because the lane
owns the address bucket, not because they belong to the loader subsystem.

- **high** - every load, store, branch and call is visible in the export; the
  only inference is the plain-English role sentence.
- **medium** - a field role, a constant from the small-data area or a callee's
  identity had to be reconstructed from a sibling, or the register-save
  prologue hides one argument.

Inside the Rust file, `// observed:` marks facts read directly from the Ghidra
C, `// inferred:` marks a role reconstructed from neighbours, and `// unknown:`
marks an unresolved field.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/bres.rs` | `resource::bres` | 28 | 5,060 | 14 | 14 |

The file now holds **194** translated bodies (166 before this pass); no body in
it is a stub.

## What this pass covers

- **Console framebuffer defaults** (`0x802331A0`, `0x80233260`, `0x80233290`,
  `0x80233460`): the 640x480 geometry block at `0x803DA120`, the readiness test,
  the clipped `0x1080` rectangle fill with its 2x scale rule, and the
  cache-block flush of the whole pixel buffer. This is the same config record
  `runtime::bt` documents at `0x80233420`.
- **Console text layout** (`0x80233570`, `0x802336D0`): the per-line pen
  advance, the tab measure to four-column stops, the 128-byte glyph-index table
  at `0x802F71E8` (`-3` = tab, `-1` = missing display glyph) and the six-pixel
  cell pitch.
- **Console list management** (`0x80233C60`, `0x80233EC0`, `0x80233EF0`): the
  unlink that asserts `console.cpp:0x182 "illegal console handle"`, its wrapper,
  and the incremental repaint that walks the `+0x06`-row ring from
  `(+0x10 + pending) mod +0x06` to `+0x0C`.
- **Console grid writer** (`0x80234120`): newline, four-column tabs, Shift-JIS
  two-byte cells (`c >= 0x80`), the row terminator, the oldest-row push into
  `+0x10`/`+0x14`, and the two overflow policies selected by `+0x0A` bit 0.
- **Frame path** (`0x80234630`, `0x80234720`): installation of the 0x360-byte
  state block, the 0x4000-byte render buffer and four retrace hooks, plus the
  per-frame 0x10-byte record `{u16 mode, u16 pad, screen, arg3, arg4}`.
- **Script queue** (`0x80235710..0x80235EB0`): the `r13 - 0x49FC` singly linked
  queue (insert at front or after the head, unlink, clear, empty test, dispatch
  walk), the shared script reader helper, the n-th-token scan and the hex
  parser, both of which call the live reader pointer at `r13 - 0x49F8`.
- **Script byte stream** (`0x80235830`): the 0x200-byte window at
  `offset & 0x7FFFFFE0`, the `0x801CAA30` read with the `0x801CEE90` poll loop
  and the 0x20-byte rounding of the final short window.
- **Script heaps and records** (`0x80235FE0`, `0x80236AD0`, `0x80236C70`,
  `0x80236D70`, `0x80237860`, `0x80237A00`): the two `EXPH` heaps at
  `r13 - 0x49F0`/`r13 - 0x49E8`, the 0x1C-byte record layout and the two
  identical initialiser/reserve pairs.
- **Small wrappers** (`0x802307E0`, `0x80231B70`): the dispose-and-free helper
  for archive-heap objects and the eight-byte console-script thunk.

## Functions

| address | function | bytes | role | confidence |
|---|---|---:|---|---|
| `0x802307E0` | `fn_802307e0` | 88 | dispose an object, free it when the count is positive | high |
| `0x80231B70` | `fn_80231b70` | 8 | tail call to the console-script executor | medium |
| `0x802331A0` | `fn_802331a0` | 180 | install the 640x480 console defaults | high |
| `0x80233260` | `fn_80233260` | 40 | console ready test | high |
| `0x80233290` | `fn_80233290` | 396 | clipped framebuffer rectangle fill | medium |
| `0x80233460` | `fn_80233460` | 20 | cache-block flush of the pixel buffer | high |
| `0x80233570` | `fn_80233570` | 340 | draw a text block at a console pen position | medium |
| `0x802336D0` | `fn_802336d0` | 220 | lay out one text run and blit its glyphs | medium |
| `0x80233C60` | `fn_80233c60` | 168 | unlink a console record from the list | high |
| `0x80233EC0` | `fn_80233ec0` | 48 | destroy a console record, return its handle | high |
| `0x80233EF0` | `fn_80233ef0` | 420 | repaint the pending rows of one record | medium |
| `0x80234120` | `fn_80234120` | 948 | write text into the scrolling character grid | medium |
| `0x80234630` | `fn_80234630` | 232 | install the console frame path and retrace hooks | medium |
| `0x80234720` | `fn_80234720` | 316 | run one console frame for a retrace slot | medium |
| `0x80235710` | `fn_80235710` | 140 | link a script-path record into the queue | high |
| `0x802357A0` | `fn_802357a0` | 64 | unlink a script-path record | high |
| `0x802357F0` | `fn_802357f0` | 12 | clear the script queue head | high |
| `0x80235800` | `fn_80235800` | 20 | script queue non-empty test | high |
| `0x80235830` | `fn_80235830` | 260 | read one byte through the script read window | medium |
| `0x80235940` | `fn_80235940` | 196 | find the n-th token of the script | medium |
| `0x80235A10` | `fn_80235a10` | 204 | parse a hexadecimal number | high |
| `0x80235EB0` | `fn_80235eb0` | 132 | offer a selector to every queued node | high |
| `0x80235FE0` | `fn_80235fe0` | 152 | create script queue A and its first record | medium |
| `0x80236AD0` | `fn_80236ad0` | 84 | initialise one script record | high |
| `0x80236C70` | `fn_80236c70` | 68 | reserve storage from queue A's heap | medium |
| `0x80236D70` | `fn_80236d70` | 152 | create script queue B and its first record | medium |
| `0x80237860` | `fn_80237860` | 84 | initialise one script record (B sibling) | high |
| `0x80237A00` | `fn_80237a00` | 68 | reserve storage from queue B's heap | medium |

## Deliberate skips

- **Packed-vector math (`0x802380A0..0x80239780`, `0x8023F000..0x8023FE50`):**
  every one of these bodies decodes or encodes through `unaff_GQR0`/`unaff_GQR3`,
  the paired-single quantisation register that the caller sets. The exports
  either discard the scale factor or read the raw 16-bit halves of a float, so a
  reconstruction would have to guess the GQR value. This is the same reason
  wave 85 skipped `0x8023F000`, `0x8023F140`, `0x8023F4E0`, `0x8023FA30` and
  `0x8023FE50`. The one non-GQR member, `0x80238270`, is a two-line wrapper that
  forwards a scaled argument to `0x801C0B50` and returns its second argument
  unchanged; it carries no field evidence beyond that call.
- **Glyph blitter `0x802337B0`:** the 5x7 font expansion builds each 16-bit
  pixel pair from four rotated bits of a 30-bit font row, then re-normalises the
  word with `uVar12 >> 1 & 0x7E000000` for the 1x path and a three-lookup
  `0x8034CF30` chain. Reproducing it faithfully needs the font table contents,
  which the export does not contain, and a wrong shift would silently corrupt
  every glyph. `0x80233570` and `0x802336D0` call it by address.
- **Frame loop `0x802348F0`:** 1.7 KB of nested loops that mix the console queue,
  OS thread counters, `0x801B5F80` timestamps and a `_DAT_800000f8 / 4000` tick
  division, and it ends in a `do { ... } while (true)` tail with no exit that
  Ghidra could model. Reconstructing it honestly would need the tick register
  semantics that are not in the C.
- **Script parser `0x80235AE0` and dispatcher `0x80235DC0`:** both are spaghetti
  the decompiler could not structure (`LAB_80235b18`, `LAB_80235d20`,
  `unaff_r30` reused as a loop counter across `goto`s, and calls whose argument
  lists are empty because the values never left their registers). The callers
  and the dispatcher address stay explicit in the translated bodies.
- **`0x8022xxxx`, `0x8021xxxx` callees:** the console and DVD helpers are called
  by absolute address as the style guide requires. The `EXPH` heap routines
  (`0x801D4380`, `0x801D4460`, `0x801D49A0`), the OS interrupt pair and the DVD
  read/poll trio are named in comments as `// inferred:` from their argument
  shapes, not from a symbol table.
- **The `0x80236230..0x80236880` message-queue family:** these functions build
  formatted debug lines on a heap and post them through a queue whose record
  layout is only visible through `_savegpr` fragments. They are left for a later
  pass rather than guessed.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (re-read from the TSV, not copied from prose).
- Every evidence file exists at the cited repo-relative path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/bres.rs`.
- No address in `ledger/agents/wave90-bres.csv` was already in
  `ledger/functions.csv`, and no address is duplicated across the eight wave-90
  CSVs that had landed when this lane was validated.
- `cargo check --message-format=short` reports no error and no warning for
  `src/resource/bres.rs`.
- `node tools/check-lane-csv.mjs wave90` reports `OK` (229 rows across the eight
  lanes present at validation time, of which 28 are this lane).

`cargo check` proves the Rust is well-formed and type-correct; it does not run
the target code. The pointer arithmetic models the Wii's 32-bit big-endian
PowerPC and must only execute on a target-compatible runtime with the original
memory image mapped.
