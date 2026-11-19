# Coverage - Wave 95 (audio::seq, JAS list hand-off and the sound element block)

## How to read this file

Every row is one `pub unsafe fn` added to `src/audio/seq.rs` by this pass. The
address is the original DOL virtual address, the byte count comes from
`work/ghidra/main-gb/export/functions.tsv`, and the evidence path is the Ghidra
C export the body was reconstructed from. `high` means every load, guard, store
and call is literal in that export; `medium` means at least one field role,
callee argument or register-derived value is inferred and marked as such in the
function's doc block; `low` means only the instruction sequence is certain.

Two conventions carry over from waves 66/80/90 in this file:

- The CodeWarrior register-save stubs (`0x802DCD60`, `0x802DCD68`, `0x802DCD6C`,
  `0x802DCD70`, `0x802DCD74`) return nothing; Ghidra prints them as 64-bit calls
  whose "returned pair" is really the caller's incoming `r3`/`r4`. Where a body
  needs that object it takes it from its own parameters, and where the export
  prints the pair the high word is read as `this`.
- `0x801B1140`/`0x801B1180` are the small-data state lock pair. Every body that
  takes them reproduces the hand-off (`let token = call0_r(FUN_801B1140)` ...
  `call1(FUN_801B1180, token)`).

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/audio/seq.rs` | `audio::seq` | 26 | 5024 | 14 | 11 | 1 |

Bytes, High, Medium and Low cover only the 26 bodies added in this pass.

## What this pass covers

- **JAS sequence-object helpers, `0x802700F0..0x80270310` (4 bodies).** The
  remaining-length subtraction, the `+0x04` owner-link store and clear, and the
  enable-gated state byte. These are the four routines the two manager-list
  bodies below call, so the cluster closes its own small call graph.
- **List hand-off and cached-handle probes, `0x8027B010..0x8027B1C0`
  (4 bodies).** `0x8027B010` unlinks the node embedded `0x18` bytes into a JAS
  object and hands it to `0x80270060`; `0x8027B0C0` is the exact inverse. Both
  take the same `+0x18`/`+0x1C` list header and the sequence-manager mutex, as
  the wave-90 register/unregister pair does for `+0x384`. `0x8027B160` and
  `0x8027B1C0` probe a source object's vtable slot `+0x0C`; the second one also
  tests a cached handle's `+0x9C` id word before deciding to create.
- **Start-up descriptor and clock sum, `0x8027B270..0x8027B2E0` (3 bodies).**
  `0x8027B270` is a bare slot-`+0x0C` dispatch. `0x8027B290` builds a four-word
  descriptor (`first`, `0x4000`, `second`, `0x4000`) and hands it to
  `0x8027B360` with the `0x803FD600` buffer and `0x155E0` bytes. `0x8027B2E0`
  sums the three manager clock pairs plus the block's `+0x04`/`+0x0C`.
- **`RSTM` section readers, `0x8027C210..0x8027C600` (3 bodies).**
  `0x8027C210` performs the two seek/read pairs, checks the `RSTM` magic, the
  version at `+0x06` and the caller's size budget at `+0x18`, and fills the
  handle's `+0x04`/`+0x08` words. `0x8027C4B0` resolves a type-2 section entry
  and copies its nine words plus seven half-words. `0x8027C600` seeks to
  `state + 0x18 + row * count * 4 + 8` and splits one `0x20`-byte block of
  paired half-words into two arrays.
- **Block lifecycle and arm, `0x8027C7C0..0x8027CA70` (3 bodies).** The
  constructor installs three vtables, self-links `+0x178` and registers the
  `0x34`-byte / `0x20`-element array at `+0x184`; the destructor reverses it.
  `0x8027CA70` is the arm routine: it releases any previous arm through the
  block's own slot `+0x10`, resets the elements, clamps the track count to
  `0x10`, walks the bitmap into at most eight elements, and claims one
  fixed-slot-allocator slot per element (the wave-90 allocator at
  `0x8027BCC0`/`0x8027BDB0`), rolling back on the first failure.
- **Play-state read helpers, `0x8027D3A0..0x8027D520` (3 bodies).** The play
  record reader, the position reporter with its `-1`/`-1`/`0` guard chain, and
  the reset that installs the two small-data defaults at `0x8042EAE8` /
  `0x8042EAD0` across all eight elements and clears the sixteen slot words.
- **Per-element voice broadcast, `0x8027DDD0..0x8027ED20` (4 bodies).** The
  bind pass with its rollback, the two voice-parameter pushes over the
  `+0xB68`-selected `+0x818` slot arrays, and the pause broadcast that compares
  `+0x109 | +0x107` with the remembered `+0x108`.
- **Voice detach and registry, `0x8027F270`/`0x8027FDD0` (2 bodies).** The
  detach pair (owner `+0x174` unlink, virtual slot `+0x08` with `-1`, owner
  `+0x180` drop) and the one-shot small-element registry singleton at
  `0x80416FC4`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802700F0` | `fn_802700f0` | 16 | high |
| `0x80270100` | `fn_80270100` | 8 | high |
| `0x80270110` | `fn_80270110` | 12 | high |
| `0x80270310` | `fn_80270310` | 28 | high |
| `0x8027B010` | `fn_8027b010` | 168 | high |
| `0x8027B0C0` | `fn_8027b0c0` | 160 | high |
| `0x8027B160` | `fn_8027b160` | 96 | medium |
| `0x8027B1C0` | `fn_8027b1c0` | 168 | medium |
| `0x8027B270` | `fn_8027b270` | 24 | low |
| `0x8027B290` | `fn_8027b290` | 72 | medium |
| `0x8027B2E0` | `fn_8027b2e0` | 128 | medium |
| `0x8027C210` | `fn_8027c210` | 452 | high |
| `0x8027C4B0` | `fn_8027c4b0` | 332 | medium |
| `0x8027C600` | `fn_8027c600` | 436 | high |
| `0x8027C7C0` | `fn_8027c7c0` | 240 | high |
| `0x8027C970` | `fn_8027c970` | 256 | medium |
| `0x8027CA70` | `fn_8027ca70` | 536 | medium |
| `0x8027D3A0` | `fn_8027d3a0` | 156 | high |
| `0x8027D440` | `fn_8027d440` | 212 | high |
| `0x8027D520` | `fn_8027d520` | 308 | high |
| `0x8027DDD0` | `fn_8027ddd0` | 236 | medium |
| `0x8027E590` | `fn_8027e590` | 264 | medium |
| `0x8027EA00` | `fn_8027ea00` | 240 | medium |
| `0x8027ED20` | `fn_8027ed20` | 180 | medium |
| `0x8027F270` | `fn_8027f270` | 140 | high |
| `0x8027FDD0` | `fn_8027fdd0` | 156 | high |

## Deliberate skips

- **`0x80277050`** (60 bytes). Sets `+0x18 = param_2`, clears `+0x21C` and calls
  `0x801B2E20` with no visible argument. That call is the in-place mutex
  initialiser, so the missing argument is a real field address; inventing one
  would be a guess, so no body is claimed.
- **`0x8027B360`** (428 bytes) and **`0x8027B520`** (116 bytes). The subsystem
  start-up and its wait loop. Both read `unaff_r13` flags and call
  `0x80260860` / `0x80264C00` / `0x80267D20` / `0x80280530` / `0x8027B720` with
  argument lists the export does not print, and `0x8027B360` also consumes the
  untyped register-save pair from `0x802DCD70`. `0x8027B290` calls
  `0x8027B360` by address and documents that its arguments are inferred.
- **`0x8027D160`** (456 bytes). The block teardown walks three lists and a
  `0x800`-byte element array with four calls that have no printed arguments.
  The stores are literal but the callee roles are not, so it is left for a pass
  that can read the assembly.
- **`0x8027E6A0`** (508 bytes) and **`0x8027E8A0`** (348 bytes). Both are the
  frame advance over the same element array. `0x8027E6A0` binds the register
  stub's 64-bit temporary to a plain `int` and then dereferences it as the
  object, while `0x8027E8A0` uses the *high* word as the object and the low word
  as a step index. The two cannot both be right, and choosing between them
  would be a guess about which half of the pair is `this`.
- **`0x8027EAF0`** (556 bytes). The element-record constructor. It writes
  thirteen fields of a record obtained from `0x8026DAA0` and pushes it through
  `0x8027FF60`, but the record is also read through `puVar4` when the allocation
  returns null (`puVar4[4] = iVar3;` immediately after the null branch), which
  the export cannot resolve.
- **`0x8027F450`, `0x8027F7B0`, `0x8027FC50`, `0x8027FFE0`.** Still skipped for
  the reasons recorded in `docs/COVERAGE_WAVE90_SEQ.md`.
- **`0x8027AA00`, `0x8027ABB0`.** Still skipped for the reasons recorded in
  `docs/COVERAGE_WAVE80_SEQ.md`.
- **`0x802700F0..0x80270310` neighbours `0x80270920`, `0x80270AD0`,
  `0x80270C20`, `0x80271E30`, `0x802720A0`, `0x80273560`, `0x80273DD0..0x80274CE0`,
  `0x802783C0`, `0x80279510..0x80279CE0`, `0x8027D660..0x8027E080`.** Not
  attempted this pass: the four tiny `0x80270xxx` helpers were taken, but the
  JAS voice/rate loaders and the paired-single arithmetic paths still belong
  with the `audio::ax` rate/voice work, and the time went to the contiguous
  `0x8027B010..0x8027FDD0` runs.

## Verification

- Every address and byte size matches `functions.tsv` (checked with a scripted
  comparison of all 26 doc blocks against the TSV sizes).
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/audio/seq.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/audio/seq.rs`. During this pass the tree also contained transient syntax
  and type errors in other wave-95 lanes (`engine::main_loop`, `runtime::dvd`,
  `game::bases`, `game::worldmap`); an isolated copy of the crate with only
  those unrelated defects repaired compiles the whole library with zero
  diagnostics naming `src/audio/seq.rs`.
- `node tools/check-lane-csv.mjs wave95` reports `OK` for `wave95-seq.csv`
  (26 rows, unique addresses, evidence paths present).
