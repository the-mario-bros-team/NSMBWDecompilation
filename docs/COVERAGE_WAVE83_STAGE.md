# Coverage - Wave 83 (`game::stage`, the `0x8007xxxx` scene/status gaps)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The evidence column
of `ledger/agents/wave83-stage.csv` points at the Ghidra C export each body was
written from. Roles that the export only hints at are marked `inferred` or
`unknown` in the body itself, never here.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 26 | 2628 | 18 | 8 |

No body in this pass is `low` confidence and none is `partial`; every row is
`translated`.

## What this pass covers

Four untranslated runs inside the `0x8007xxxx` band, all between the wave-61
map-cell probe cluster and the wave-70 actor-manager run. They are tied
together by three object layouts that the stores make explicit:

* **Owner/layer record** (`+0x04` owner, `+0x50` map handle, `+0x74` slot count,
  `+0x78` slot table stride `0x10`, `+0x88` status word, `+0x99` player index,
  `+0xBC` flag byte).
* **Scene object**: the `0x900xx`/`0x90Axx` block seeded by `0x80077060`
  (mode byte `+0x900FE`, index `+0x900FF`, the four `0x38`-byte table entries at
  `+0x90A40`, the extra handle at `+0x90A4C`).
* **Map-cell table**: `0x44` bytes per cell, `row * 0x2200 + 0x1078 + column *
  0x440`, sixteen cells per group.

The four runs:

- **Status predicates** (`0x800705B0..0x800713B0`). The owner/player selector,
  the two `>> 16 & 0xFF` tag readers on the heavy (`+0xA0`) and light (`+0xA8`)
  partner slots, the `+0x88 & 0x18000` flag test, the four-way status
  accumulator, and the 280-byte entry test that walks the linked edge record
  before calling `0x800D9A30`.
- **Layer slots and offsets** (`0x800731E0`, `0x80073AC0`, `0x80077610`,
  `0x80077D80`, `0x80078250`). Two world-point offset writers, the cell-id
  masker, the `+0x90A4C`/`+0x90A40` vtable destroy walk, and the `0x100`-slot
  layer claimer with its `0x2A`/`0x28` size arms.
- **Scene accessors** (`0x80078510..0x80079020`, `0x8007A6B0`). The `+0x90AA8`
  byte, the `+0x90A40[index] + 0x38` record address, the two `state 2/6`
  player predicates, the three current-stage lookups through
  `*(r13-0x57E8) + [info+0x120E]*0x3B0` (`0x80078780`/`0x80078800`/`0x80078830`),
  the `+0x90040`/`+0x90050` float pair, the two selector+`0x80078860` drivers
  and the `+0x900C6` flag clear.
- **Cell-list inserters** (`0x8007D5B0`/`0x8007D770`, plus `0x8007EA50`). The
  horizontal and vertical shift-in of a probe cell, both 444 bytes and
  structurally identical apart from the key offset (`+0x14` vs `+0x2C`) and the
  compared cell word (5 vs 11); the `0x8007EA50` thunk copy of the actor-entry
  reset `0x8007EA30`.

The public entry points are [`fn_80078540`] (is any player in the walk/enter
state), [`fn_80078250`] (claim a layer slot), [`fn_8007d5b0`] (insert a cell) and
[`fn_800713b0`] (may this owner enter the map).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800705B0` | `fn_800705b0` | 68 | medium |
| `0x80070720` | `fn_80070720` | 12 | high |
| `0x80070730` | `fn_80070730` | 12 | high |
| `0x80070CA0` | `fn_80070ca0` | 12 | high |
| `0x80070E70` | `fn_80070e70` | 128 | medium |
| `0x800713B0` | `fn_800713b0` | 280 | medium |
| `0x800731E0` | `fn_800731e0` | 40 | high |
| `0x80073AC0` | `fn_80073ac0` | 128 | high |
| `0x80077610` | `fn_80077610` | 52 | high |
| `0x80077D80` | `fn_80077d80` | 148 | high |
| `0x80078250` | `fn_80078250` | 168 | high |
| `0x80078510` | `fn_80078510` | 12 | high |
| `0x80078520` | `fn_80078520` | 24 | high |
| `0x80078540` | `fn_80078540` | 152 | high |
| `0x800785E0` | `fn_800785e0` | 140 | high |
| `0x80078780` | `fn_80078780` | 124 | medium |
| `0x80078800` | `fn_80078800` | 48 | high |
| `0x80078830` | `fn_80078830` | 44 | high |
| `0x80078CE0` | `fn_80078ce0` | 12 | medium |
| `0x80078CF0` | `fn_80078cf0` | 12 | medium |
| `0x80078EB0` | `fn_80078eb0` | 52 | medium |
| `0x80079020` | `fn_80079020` | 52 | medium |
| `0x8007A6B0` | `fn_8007a6b0` | 16 | high |
| `0x8007D5B0` | `fn_8007d5b0` | 444 | high |
| `0x8007D770` | `fn_8007d770` | 444 | high |
| `0x8007EA50` | `fn_8007ea50` | 4 | high |

Address range: `0x800705B0..0x8007EA50`.

## Deliberately skipped

Skipped because the body could not be reconstructed honestly:

- `0x80078D00`, `0x80078EF0`, `0x80079250`, `0x80079380`, `0x800794B0`,
  `0x800795E0` (432/296/292 bytes each) - the mode-table selectors. Every one
  copies sixteen float constants from `0x802EF658..0x802EF7BC` into a stack
  block and then picks one entry by `+0x900FF`. The exports only name the
  `DAT_802EF6xx` symbols; the table bytes are not carried by the C file, so the
  selected value (and hence the published float) cannot be stated. Only their
  two callers (`0x80078EB0`, `0x80079020`) are translated, as address calls.
- `0x800789A0`, `0x80078A70`, `0x80078B40`, `0x80078C10` (200 bytes each) - the
  four camera-rate helpers. All four are wrapped in `ldexpf` blocks guarded by
  `unaff_GQR0`; the paired-single quantisation order is not visible, so neither
  the scale nor the clamp can be named.
- `0x8007FBF0` (204 bytes) - the four-corner writer behind `0x8007FAD0`. Same
  `GQR0` problem, plus three `ldexpf` calls whose argument is only
  `bVar1 & 0x3f`.
- `0x8007FA40` (132 bytes) and therefore `0x8007FAD0`/`0x8007FB10` - `0x8007FA40`
  forwards `extraout_f1` (the mangled `f1` return of `0x8007F9B0`) and `in_r10`,
  and its callee chain passes twelve arguments with four `double` values in
  `f1..f4`; the argument plumbing is not recoverable. `0x8007FB10` itself is
  clean but has no translated callee to end in, so the run stays an address
  call.
- `0x80072BC0` (240 bytes) - the smoke/spsmoke spawn test. It needs the
  `r2-0x73A4` float seed whose value is unknown, and `0x8006DB30()` is printed
  with no arguments at all.
- `0x80073B40` (128) and `0x800743D0` (148) - the three-way fallback probes.
  Both call `0x80073FC0`/`0x80074AF0` first with only the `r2-0x7440` floor and
  print the later calls with `param_1`/`param_2` attached to a different
  position; the argument lists do not line up, so a body would invent them.
- `0x800709C0` (128), `0x800775A0` (100), `0x800776F0` (4), `0x80077700` (40) -
  the callee is printed with its receiver dropped (`FUN_80070ba0()`,
  `FUN_80077520()`), or the thunk's `CONCAT44(0x43300000, ...)` conversion hides
  the operand.
- `0x8007CA90` (736), `0x800722C0` (384), `0x80072790` (296), `0x8007E860`
  (464) - the `0x802DCD60/4C/70/74` family. Each returns a register pair in
  `r3:r4` that the export splits with `(int)((ulonglong)uVar >> 0x20)`, and
  `0x8007CA90` also casts a player pointer to `float`; the FIFO protocol is not
  visible in the C.
- `0x80077060` (808 bytes) - the scene constructor that seeds the whole block
  this pass reads. Fully mechanical, but it needs the `r2-0x7368`/`r2-0x736C`
  float constants, whose bytes are not in any export here.
- `0x80070130`, `0x80071060`, `0x80071210`, `0x800714D0`, `0x800717A0`,
  `0x80071A60`, `0x80071DC0`, `0x80072180`, `0x80072440`, `0x800728C0`,
  `0x80073290`, `0x80073670`, `0x80073BC0`, `0x80073FC0`, `0x80074470` - the
  `GQR0`/`r2`-double cluster below and around the cell probe code. Same reason
  as the wave-75 skip list: the `ldexpf` guards and the `CONCAT44` constants are
  not reconstructable from the C text.

Also left untranslated, but reconstructable in a later pass:

- `0x800713B0`'s neighbours `0x80071xxx`/`0x80072xxx` that use only integer
  flags (for example the `0x80071DC0` state machine) once the float tables they
  read are dumped.
- `0x80078CE0`/`0x80078CF0` were accepted as `f32` returns; if a call site shows
  a real `float->double` conversion, the pair should be re-read.

## Verification

- Every address and byte size matches `functions.tsv` (26 rows, 2628 bytes);
  `tools/check-lane-csv.mjs` cross-checks each `# Original` size.
- Every evidence file exists at the cited path under `work/ghidra/main-gb/`.
- Every row has exactly one body in `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs` (the remaining errors in the workspace belong to
  `src/game/enemies.rs`, a different lane).
- `node tools/check-lane-csv.mjs wave83` prints `OK`; the wave-83 stage rows are
  clean.
