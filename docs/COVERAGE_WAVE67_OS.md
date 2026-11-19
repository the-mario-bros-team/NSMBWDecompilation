# Coverage - Wave 67 (runtime::os, upper object layer `0x80253CB0..0x8025FDC0`)

## How to read this file

This page records one wave of the `main.dol` reconstruction: wave 67, the
`runtime::os` lane over the remaining bodies of the `0x80250000..0x80260000`
bucket (the upper half of the `0x80240000..0x80260000` object lane that wave 9
opened).

- **Status** — `translated` means the whole recovered body is present: every
  branch, store and call in the Ghidra export has a counterpart. Every row here
  is `translated`.
- **Confidence** — strength of the evidence, not of the Rust. `high` means the
  flow, stores and callees are explicit and self-contained. `medium` means the
  flow is explicit but a callee's role, a constant's meaning, or an argument
  register swept away by Ghidra is inferred.
- **Evidence** — the C file under `work/ghidra/main-gb/export/`; it is the source
  of truth for every row.
- Addresses are virtual addresses of the unrelocated `main.dol` (image base
  `0x80004000`), not file offsets. Byte sizes come from
  `work/ghidra/main-gb/export/functions.tsv`, re-read for this pass.

The machine-readable companion is `ledger/agents/wave67-os.csv` (same rows, same
order). This wave edits only `src/runtime/os.rs` plus those two files;
`ledger/functions.csv` is untouched and merged by the coordinator.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/os.rs` | `runtime::os` | 28 | 2,208 | 18 | 10 |

Status split: 28 `translated`, 0 `partial`, 0 `stub`. No `low`.

Scope was the 163 function entries of the `0x80250000..0x80260000` bucket. Before
this pass 17 of them were already merged (11 `runtime::os` bodies from wave 9,
6 `runtime::bt` bodies at `0x80253C30`, `0x80253E60`, `0x8025BEF0`,
`0x8025BF30`, `0x8025BFE0`, `0x8025F330`); none of those is repeated here.
Candidates were chosen as whole, self-contained clusters rather than as a byte
count: four contiguous groups share a field layout, so one reading of the export
justifies the whole group.

## What this pass covers

- **Cluster 1 — object-field stores, `0x80253CB0..0x80253D60`.** `fn_80253cb0`
  resets the sub-block at `+0x30` and raises flag `0x80`; `fn_80253d60` stores
  four narrowed floats at `+0xDC..+0xE8`, re-reads the holder, then publishes the
  same four values through the clamping converter `0x802DCC4C` at
  `+0xF4..+0x100`.
- **Cluster 2 — block-array pitch accessors, `0x8025BF60..0x8025BFA0`.** These
  complete the family started by `fn_8025bf30` in `src/runtime/bt.rs`: the same
  object carries three block arrays, at `+0xF4` (stride `0x30`, already in
  `bt.rs`), `+0xF8` (stride `0x24`, added here) and `+0xFC` (stride `0x30`,
  added here), each sized by the `u8` row count at `+0x101` and the `u16` column
  count at `+0x102`, with every pitch rounded up to 32 bytes.
- **Cluster 3 — descriptor result slots, `0x8025C420..0x8025C600`.** One 8-word
  descriptor shape, six "return the cached slot, first marking the owner's table
  when asked" accessors, and one accessor that instead resolves an empty slot
  through `0x8023B860`. The six mark bits (`0x01`, `0x04`, `0x10`, `0x20`,
  `0x100`, `0x800`) line up with the six slot offsets (`+0x08`, `+0x10`,
  `+0x18`, `+0x1C`, `+0x24`, `+0x30`).
- **Cluster 4 — state and slot wrappers, `0x8025EC30..0x8025F6B0`.** A virtual
  thunk identical in shape to `fn_80243210`; a slot dispatcher that masks by
  bit `1 << (slot-1)` of `+0xCC` and then routes slots `5`, `6`, `7` and the
  default; the null-guarded block reset; the two install calls with a scratch
  word; and the six 64-byte stubs that tear a slot down through one
  `0x801C64xx`/`0x801C67xx` callee and then fix the state word.
- **Cluster 5 — glyph-slot writers, `0x8025FD50..0x8025FDC0`.** Two bounds-checked
  single-byte writers: a column-addressed one bounded to eight slots plus a
  fixed-offset sibling for the ninth byte, both writing `0xFF` for the negative
  "no glyph" argument.

## What the pass adds beyond the raw bodies

1. **The mark-bit ladder is now readable.** The six `0x8025C4xx` accessors were
   six indistinguishable 72-byte exports. Written side by side, with the slot
   offset and the bit in the same order as the export's index chain
   (`param_1[2]`, `[4]`, `[6]`, `[7]`, `[9]`, `[0xc]`), they show that the cached
   result pointer and the dirty bit are paired per slot — a port must keep the
   "mark, then return" order or the draw code stops seeing the entry.
2. **One block-array family now has all three members.** `+0xF4`/`0xF8`/`+0xFC`
   with strides `0x30`/`0x24`/`0x30` are three views of the same object. The
   `0x24`-stride member is the odd one; its record type is still unknown.
3. **The `0x801C64xx` teardown family is explicit.** Five distinct callees
   (`0x801C6450`, `0x801C6460`, `0x801C6470`, `0x801C6610`, `0x801C6710`) all
   take the slot address `state + 4`, and all but one finish with the same
   `(state & !2) | 1`. `fn_8025f5f0` keeps the same callee as `fn_8025f4b0` but
   writes `(state & !1) | 0x0A` instead; that asymmetry is preserved rather than
   normalised.
4. **`fn_8025f0c0` is documented as a dispatcher, not a lookup.** Its first test
   uses the object's own `+0xCC` word as a "this slot is already handled" mask;
   the three handled slots then branch to a virtual call, a stored callback, or
   the `0x80257A40` fallback. The `1 << param_2 - 1` precedence trap
   (`1 << (slot - 1)`, not `(1 << slot) - 1`) is called out in the doc block.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80253CB0` | `fn_80253cb0` | 76 | high |
| `0x80253D60` | `fn_80253d60` | 156 | medium |
| `0x8025BF60` | `fn_8025bf60` | 52 | high |
| `0x8025BFA0` | `fn_8025bfa0` | 52 | high |
| `0x8025C420` | `fn_8025c420` | 72 | high |
| `0x8025C470` | `fn_8025c470` | 72 | high |
| `0x8025C4C0` | `fn_8025c4c0` | 72 | high |
| `0x8025C510` | `fn_8025c510` | 72 | high |
| `0x8025C560` | `fn_8025c560` | 72 | high |
| `0x8025C5B0` | `fn_8025c5b0` | 72 | high |
| `0x8025C600` | `fn_8025c600` | 96 | medium |
| `0x8025EC30` | `fn_8025ec30` | 48 | medium |
| `0x8025F0C0` | `fn_8025f0c0` | 256 | medium |
| `0x8025F2C0` | `fn_8025f2c0` | 112 | high |
| `0x8025F390` | `fn_8025f390` | 28 | medium |
| `0x8025F3B0` | `fn_8025f3b0` | 112 | medium |
| `0x8025F420` | `fn_8025f420` | 20 | medium |
| `0x8025F440` | `fn_8025f440` | 48 | medium |
| `0x8025F4B0` | `fn_8025f4b0` | 64 | high |
| `0x8025F4F0` | `fn_8025f4f0` | 64 | high |
| `0x8025F530` | `fn_8025f530` | 64 | high |
| `0x8025F570` | `fn_8025f570` | 64 | high |
| `0x8025F5B0` | `fn_8025f5b0` | 64 | high |
| `0x8025F5F0` | `fn_8025f5f0` | 64 | high |
| `0x8025F630` | `fn_8025f630` | 96 | medium |
| `0x8025F6B0` | `fn_8025f6b0` | 24 | medium |
| `0x8025FD50` | `fn_8025fd50` | 112 | high |
| `0x8025FDC0` | `fn_8025fdc0` | 104 | high |

## Report shapes for the mixed float/integer callees

Three bodies pass `double` arguments to callees that also take integer
arguments, and the Ghidra signature alone does not say which register class each
value uses. The Rust spells the ABI out instead of guessing:

| Function | Callee | Rust shape |
|---|---|---|
| `fn_8025f3b0` | `0x801C8920` | `extern "C" fn(f64, f64, f64, f64, u32, u32)` |
| `fn_8025f630` | `0x801C6430` | `extern "C" fn(f64, f64, f64, f64, f64, f64, u32)` |
| `fn_8025f0c0` | slot `+0x0C` / `+0xEC` | `extern "C" fn(u32, u32, u32, u32)` / `fn(u32, u32)` |

`fn_80253d60` is the other ABI case: `0x802DCC4C` is a clamping double-to-int
converter, and the export prints its **first** call without arguments because
Ghidra lost `f1`. The other three calls keep theirs, so the first argument is
restored by symmetry and the row is `medium` for that reason alone.

## Unknowns

1. **Mark-bit meanings.** The six bits in cluster 3 are translated exactly, but
   nothing in the window names them.
2. **Callee roles.** `0x801C6430`, `0x801C6450`, `0x801C6460`, `0x801C6470`,
   `0x801C6610`, `0x801C6710`, `0x801C6730`, `0x801C6870`, `0x801C8920`,
   `0x801C8B50`, `0x801C8CB0`, `0x802384F0`, `0x8023B860`, `0x8024A610`,
   `0x8024D5E0` and `0x80257A40` are dispatched by address. Only their argument
   shapes are observed.
3. **`0x8023B860`'s `+0x40` bias.** `fn_8025c600` adds `0x40` to the lookup
   result. Whether that is a header size or a flag bias is not recoverable here.
4. **`sda2` constants.** `sda2(-0x48E8)` (the `fn_8025f2c0` fill word) and
   `sda2(-0x48E0)`/`-0x48D8`/`-0x48DC` (the `fn_8025f630` scales) are read as
   data; their values and meanings are not named.
5. **The `0x0A` state pattern.** `fn_8025f5f0` sets bits `1` and `3` where its
   five siblings set bit `0`. The literal is preserved as `0x0A`; the reason is
   unknown.
6. **Record identity.** The `0x24`- and `0x30`-stride block arrays and the
   descriptor slots have no string evidence. If a later lane proves an owner,
   move the bodies and update the ledger — do not copy them.

## Deliberate skips

- The 17 addresses already merged (`0x80253C30`, `0x80253990`, `0x80253BE0`,
  `0x80253C70`, `0x80253D00`, `0x80253D20`, `0x80253D40`, `0x80253E00`,
  `0x80253E60`, `0x802544A0`, `0x802544D0`, `0x8025BEF0`, `0x8025BF30`,
  `0x8025BFE0`, `0x8025F330`, `0x8025F470`, `0x8025F690`) are not repeated.
- `0x80253830` (284 bytes) was read and left out on purpose: it writes 40 fields
  from eleven `unaff_r2` constants and two fixed-point conversions in one unrolled
  body. The stores are recoverable, but the constant set would have to be
  documented as `// unknown:` offsets only, so it is a better second pass than a
  row guessed now.
- `0x8025EEB0` (240 bytes) still reads `unaff_r13` for its first argument and was
  skipped, consistent with the wave 9 selection rule for this window.
- `0x8025EFB0` (272 bytes) allocates through `0x802DCD6C`/`0x801D5540` and
  installs two vtables (`0x8034E7C0`, `0x8034E980`); it needs the allocator
  story settled first and is not claimed here.
- `0x8025E8B0`, `0x8025EC70`, `0x8025FAD0`, `0x8025FC30` and `0x8025FE30` (the
  1,940-byte worker) were not attempted: they are large multi-callee workers
  whose callees are no better named after this pass.

Remaining untranslated in the bucket after this wave: 146 − 28 = 118 entries.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (re-read for this pass, not copied from prose).
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every CSV row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/os.rs`; no address appears twice.
- No address in the CSV was already present in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave67` → `wave67-os.csv: 28 rows`,
  `total rows: 28, unique addresses: 28`, `OK`.
- `cargo check --message-format=short` → `Finished dev profile`; no error and no
  warning from `src/runtime/os.rs`. The remaining warnings are pre-existing and
  outside this lane (`src/runtime/bt.rs`, `src/runtime/cpp.rs`,
  `src/runtime/input.rs`).
