# Coverage - Wave 99 (audio::seq, playback front ends and the sequence thread)

## How to read this file

Every row is one `pub unsafe fn` added to `src/audio/seq.rs` by this pass. The
address is the original DOL virtual address, the byte count comes from
`work/ghidra/main-gb/export/functions.tsv`, and the evidence path is the Ghidra
C export the body was reconstructed from. `high` means every load, guard, store
and call is literal in that export; `medium` means at least one field role,
callee argument or save-stub half is inferred and marked as such in the
function's doc block; `low` means only the instruction sequence is certain.

Two conventions carry over from waves 66/80/90/95 in this file:

- The CodeWarrior register-save stubs (`0x802DCD48`, `0x802DCD60`,
  `0x802DCD68`, `0x802DCD70`, `0x802DCD74`) return nothing; Ghidra prints them
  as 64-bit calls whose "returned pair" is really the caller's incoming
  `r3`/`r4`. Where a body needs those values it takes them from its own
  parameters, and where the export prints the pair the high word is read as the
  owner object and the low word as the second object.
- Several already-translated bodies (`0x802724E0`, `0x80275710`, `0x80275750`,
  `0x8027F650`) are declared `void` in Rust while their callers consume `r3`.
  The new callers call those addresses through `platform::callN` and say so in
  the doc block rather than modifying an existing body.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/audio/seq.rs` | `audio::seq` | 25 | 9468 | 3 | 21 | 1 |

Bytes, High, Medium and Low cover only the 25 bodies added in this pass.

## What this pass covers

- **Global track table, `0x80270AD0..0x80270C20` (3 bodies).** `0x80270AD0`
  builds the four `0xA8`-byte records at `0x803FC9C8` on first use and returns
  the header at `0x803FC990`; `0x80270C20` performs the same init then refreshes
  the records whose `+0x40` word is `4` from the controller ports;
  `0x80270920` applies a state code to one record and fires its one-shot
  callback.
- **Sequence start routes, `0x80279510..0x80279CE0` (5 bodies).** The long
  `0x80279510` resolves a file by header/name or a bank by id, opens the stream,
  checks the remaining length, rebuilds the player and writes tempo/volume;
  `0x80279880` does the same for the small element record; `0x80279A30` uses the
  seven-word bank resolver; `0x80279B70` loads or probes one slot cache entry;
  `0x80279CE0` resolves a name and delegates.
- **Priority buckets, `0x8027AA00..0x8027ABB0` and `0x802783C0` (3 bodies).**
  The rebuild pair drains an intrusive list into a 0x80-entry bucket array and
  back, the insert path rejects lower priorities and releases rejected heads,
  and `0x802783C0` repeats the rebuild three times after releasing the bank
  slot array.
- **Sequence thread, `0x8027B720..0x8027BAF0` (4 bodies).** `0x8027B720` starts
  the thread once through `0x801B5130`; `0x8027B960` is its body (callback
  registration, message loop, pending-list drain); `0x8027B850` is the cleanup
  callback; `0x8027BAF0` runs one frame and stores the elapsed tick count.
- **`RSTM` block load and slot claim, `0x8027D660..0x8027EDE0` (4 bodies).**
  The loader decodes the section, the element tables and one row of paired
  half-words, then starts the block; `0x8027D960` streams two-element batches
  into the voice slots; `0x8027DBD0` derives the timing fields and claims one
  allocator slot per track with rollback; `0x8027EDE0` maps a position to
  row/index/wrap.
- **Large element player and ramps, `0x8027F450..0x8027FFE0` (6 bodies).**
  `0x8027F450` constructs the player around the embedded block and seven ramp
  records; `0x8027F7B0` advances and applies them; `0x8027F890` re-arms the
  masked records; `0x8027FC50` finds and attaches the player; `0x8027FFE0` pops
  one object from three pending lists; `0x80274CE0` computes the masked mix
  values over a node list.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80270920` | `fn_80270920` | 232 | medium |
| `0x80270AD0` | `fn_80270ad0` | 152 | high |
| `0x80270C20` | `fn_80270c20` | 248 | medium |
| `0x80274CE0` | `fn_80274ce0` | 444 | medium |
| `0x802783C0` | `fn_802783c0` | 956 | medium |
| `0x80279510` | `fn_80279510` | 872 | medium |
| `0x80279880` | `fn_80279880` | 432 | medium |
| `0x80279A30` | `fn_80279a30` | 312 | medium |
| `0x80279B70` | `fn_80279b70` | 360 | medium |
| `0x80279CE0` | `fn_80279ce0` | 112 | high |
| `0x8027AA00` | `fn_8027aa00` | 388 | medium |
| `0x8027ABB0` | `fn_8027abb0` | 468 | medium |
| `0x8027B720` | `fn_8027b720` | 300 | medium |
| `0x8027B850` | `fn_8027b850` | 268 | medium |
| `0x8027B960` | `fn_8027b960` | 184 | medium |
| `0x8027BAF0` | `fn_8027baf0` | 256 | medium |
| `0x8027D660` | `fn_8027d660` | 768 | medium |
| `0x8027D960` | `fn_8027d960` | 624 | medium |
| `0x8027DBD0` | `fn_8027dbd0` | 504 | medium |
| `0x8027EDE0` | `fn_8027ede0` | 268 | medium |
| `0x8027F450` | `fn_8027f450` | 180 | high |
| `0x8027F7B0` | `fn_8027f7b0` | 224 | medium |
| `0x8027F890` | `fn_8027f890` | 324 | low |
| `0x8027FC50` | `fn_8027fc50` | 244 | medium |
| `0x8027FFE0` | `fn_8027ffe0` | 348 | medium |

## Deliberate skips

- **`0x802741E0`, `0x80274320`, `0x80274480`, `0x80274630`** (the node mix
  helpers behind `0x80274CE0`). Their exports decode quantities through
  `unaff_GQR0` (the paired-single load/store format register) and through
  `in_psNN` register pairs that the caller never materialises as ordinary
  arguments. `0x80274CE0` calls them by address, but reproducing their bodies
  would require guessing the GQR format; they remain untranslated.
- **`0x80273560` (1400 bytes), `0x80273DD0` (1032 bytes), `0x80271E30` and
  `0x802720A0`.** The same GQR/paired-single problem dominates these bodies, so
  no body is claimed.
- **`0x8027E080` (804 bytes).** The voice bind pass has over two hundred lines
  of GQR-converted vector arithmetic before the eight `0x802817xx` calls; only
  the call list is certain, so it is left for a pass with the assembly.
- **`0x802741E0`'s sibling cluster `0x802746D0..0x80274A10`.** Same reason.
- **`0x8027D160` (456 bytes).** Still skipped: the four callees
  (`0x80281390`, `0x802810A0`, `0x802802D0`, `0x8027BDB0`) are called with
  argument lists the export does not print.
- **`0x8027E6A0`, `0x8027E8A0`, `0x8027EAF0`.** Still skipped for the wave-95
  reasons: the two frame advancers disagree about which half of the save-stub
  pair is `this`, and the element-record constructor dereferences the
  allocation result immediately after its null branch.
- **`0x8027B360`, `0x8027B520`.** Still skipped for the wave-95 reasons
  (`unaff_r13` flags and unprinted callee arguments).
- **`0x8027F450`'s neighbours `0x8027F040`-adjacent helpers.** Nothing beyond
  `0x8027F450`/`0x8027F7B0`/`0x8027F890`/`0x8027FC50`/`0x8027FFE0` was claimed
  in the `0x8027Fxxx` range.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/audio/seq.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/audio/seq.rs`; the tree still contains unrelated errors in other wave-99
  lanes, which are ignored by this lane.
- `node tools/check-lane-csv.mjs wave99` reports `OK` for `wave99-seq.csv`.
