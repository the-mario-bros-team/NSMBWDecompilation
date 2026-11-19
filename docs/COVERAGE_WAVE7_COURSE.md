# Coverage — wave 7 course / sequence (`0x8008D000..0x8008F000`, `0x80270D00..0x80273200`)

## How to read this file

This page reports wave 7 of the course/sequence lane. It covers two address
bands of `main.dol`:

* **Course band** `0x8008D000..0x8008F000` — the course-file parser's stage and
  area tables (sections, object/area records, collision helpers).
* **Sequence band** `0x80270D00..0x80273200` — the `RSEQ` header/lookup entry
  points and the JAS sequence-control object that plays them.

Columns in the selected-functions table:

* **Address** — virtual address of the entry point. `main.dol` image base is
  `0x80004000`, so these are not file offsets.
* **Rust** — the symbol in the destination file.
* **Role** — one plain-English sentence.
* **Cluster** — reading group, defined under *Clusters*.
* **Status** — `translated` means entry/exit, every call and every observable
  write are accounted for. `partial` means some callee is invoked by address
  without a reconstructed body; `stub` is address/role only.
* **Confidence** — strength of evidence: `high` = every store/load visible in
  the Ghidra C; `medium` = layout observed but a field/role is inferred;
  `low` = mostly unknown.
* **Evidence** — the Ghidra `.c` file under
  `work/ghidra/main-gb/export/`.

To check a row, open the Rust function's `# Original` block and then the
evidence file. If they disagree, the C wins.

**Promotion status.** Every body in this wave is in
`src/audio/seq.rs`; no course bodies were added (see *Why the course band added
zero bodies*). The machine-readable rows are
`ledger/agents/wave7-course.csv` and `ledger/functions.csv`.

## Scope and counts

Counts are entry addresses inside each band, from
`work/ghidra/main-gb/export/functions.tsv`, filtered to functions of at least
16 bytes (one- and two-instruction thunks are excluded).

| Band | Entries | Eligible (`>= 16 B`) | Untranslated before wave 7 | Translated by wave 7 | Untranslated after |
|---|---:|---:|---:|---:|---:|
| Course `0x8008D000..0x8008F000` | 36 | 36 | 2 | 0 | 2 |
| Sequence `0x80270D00..0x80273200` | 46 | 39 | 37 | 19 | 18 |

Totals for this wave: **19 translated, 0 partial, 0 stubs**;
**14 high, 5 medium, 0 low**. New body size: **2672 bytes**.

## Why the course band added zero bodies

All 34 eligible course-band entries other than `0x8008D460` and `0x8008D8B0`
already have bodies recorded in `ledger/functions.csv` (they live in
`src/game/stage.rs` and `src/resource/course.rs`). This includes the whole
section-table family (`0x8008E310..0x8008E800`), the object-id remap
(`0x8008EAF0`, `0x8008EBE0`), the spatial list (`0x8008DCF0`, `0x8008DDB0`) and
the stage-slot helpers (`0x8008ED70..0x8008F810`).

The two remaining bodies are area-collision predicates:

* `0x8008D460` (952 bytes) — oriented-box overlap test built from
  `0x8008C4D0..0x8008C570` accessors and `0x8008D3D0`;
* `0x8008D8B0` (912 bytes) — the same test against `0x8008D820`.

Both Ghidra listings are dominated by paired-single (`ps_*`) sequences that the
decompiler renders as `unaff_GQR0`/`ldexpf` noise and by six unreachable-block
warnings. Reproducing them without an instruction-level pass would be guesswork,
so they are deferred with reason **P** (paired-single needs an instruction
pass). No body was invented for them.

The sequence band supplied the rest of the quota: 19 untranslated
sequence-control bodies with complete, readable control flow.

## Selected functions

| Address | Rust | Role | Cluster | Status | Confidence | Evidence |
|---|---|---|---|---|---|---|
| `0x80270D90` | `fn_80270d90` | Payload base plus the offset word at payload `+8`. | F | translated | high | `80270d90_FUN_80270d90.c` |
| `0x80270E60` | `fn_80270e60` | Set the 16 global fallback track values to `0xFFFF`. | T | translated | high | `80270e60_FUN_80270e60.c` |
| `0x80270EB0` | `fn_80270eb0` | Construct the sequence-control defaults and tables. | C | translated | high | `80270eb0_FUN_80270eb0.c` |
| `0x80271190` | `fn_80271190` | Re-initialise play state from two caller words. | C | translated | high | `80271190_FUN_80271190.c` |
| `0x80271550` | `fn_80271550` | Start track slot 0 through the track manager. | T | translated | high | `80271550_FUN_80271550.c` |
| `0x80271640` | `fn_80271640` | Release both registrations and all 16 tracks. | C | translated | high | `80271640_FUN_80271640.c` |
| `0x80271790` | `fn_80271790` | Store a mode byte and forward it to every track. | P | translated | high | `80271790_FUN_80271790.c` |
| `0x80271820` | `fn_80271820` | Advance the play clock/counter while tracks are live. | P | translated | medium | `80271820_FUN_80271820.c` |
| `0x80271910` | `fn_80271910` | Invoke the optional per-frame callback with a stack block. | P | translated | medium | `80271910_FUN_80271910.c` |
| `0x802719B0` | `fn_802719b0` | Read one track `i16` value. | T | translated | high | `802719b0_FUN_802719b0.c` |
| `0x802719C0` | `fn_802719c0` | Write one track `i16` value. | T | translated | high | `802719c0_FUN_802719c0.c` |
| `0x802719D0` | `fn_802719d0` | Kill every live track whose id is in a range. | P | translated | high | `802719d0_FUN_802719d0.c` |
| `0x80271B90` | `fn_80271b90` | Track pointer by slot, or null above slot 15. | T | translated | high | `80271b90_FUN_80271b90.c` |
| `0x80271CE0` | `fn_80271ce0` | Address of a track value; slots 16..31 use the global table. | T | translated | high | `80271ce0_FUN_80271ce0.c` |
| `0x80272340` | `fn_80272340` | Forward a notify call through the player's `+0xF0` vtable. | W | translated | medium | `80272340_FUN_80272340.c` |
| `0x802723A0` | `fn_802723a0` | Construct the outer sequence player around a control object. | W | translated | medium | `802723a0_FUN_802723a0.c` |
| `0x802730D0` | `fn_802730d0` | Set a track's start and end offsets. | T | translated | high | `802730d0_FUN_802730d0.c` |
| `0x802730E0` | `fn_802730e0` | Arm a track. | T | translated | high | `802730e0_FUN_802730e0.c` |
| `0x80273100` | `fn_80273100` | Tear down a control object's linked track list. | C | translated | medium | `80273100_FUN_80273100.c` |

Cluster letters: **F** file header/lookup, **C** control lifecycle, **T** track
table, **P** playback state, **W** player wrapper.

## What this lane does

A course file is parsed into the 0x3B0-byte stage object in
`src/game/stage.rs`; the section and object tables produced there are read by
`src/resource/course.rs`. A sequence file is validated by `0x80270D20`,
resolved by `0x80270DA0`, and then played through the control object translated
here.

The control object is the map in `src/audio/seq.rs`:

* `+0x100..+0x13C` — 16 track pointers;
* `+0x140..+0x15E` — 16 track `i16` values, with slots 16..31 in the global
  table at `0x803FCC68`;
* `+0xCC`/`+0xCD` — guard bytes for two registrations released before play;
* `+0xD0..+0xE0` — default float words and the play clock;
* `+0xF4` — track-manager object whose vtable slot `+0x10` destroys a track;
* `+0xF8`/`+0xFC` — optional callback and its context.

The mutex every mutating routine uses is at `manager(0x8027B5F0) + 0x354`,
taken with `0x801B2E60` and released with `0x801B2F40`. The nested lock pairs in
`0x80271640`, `0x802719D0` and `0x80273100` are reproduced exactly, including
the second `0x8027B5F0` call each time.

## Clusters

1. **File header/lookup (`F`)** — `0x80270D20`, `0x80270DA0`, `0x80270D90`.
   Read the two pre-existing functions first; `0x80270D90` is the payload
   accessor used after a successful header validation.
2. **Control lifecycle (`C`)** — `0x80270EB0` (construct), `0x80271190`
   (reset), `0x80271640` (release registrations + tracks), `0x80273100`
   (release a linked track list). This is the order the JAS player uses.
3. **Track table (`T`)** — `0x80270E60`, `0x802719B0`, `0x802719C0`,
   `0x80271B90`, `0x80271CE0`, `0x80271550`, `0x802730D0`, `0x802730E0`.
   Slots 0..15 are embedded; 16..31 are global.
4. **Playback state (`P`)** — `0x80271790` (mode broadcast), `0x80271820`
   (clock/counter), `0x80271910` (callback), `0x802719D0` (kill by id range).
5. **Player wrapper (`W`)** — `0x802723A0` constructs a player that embeds a
   control object at `+0x110`; `0x80272340` forwards to its `+0xF0` vtable.

## Uncertainties

1. **Small-data defaults.** `0x8042E9F0` and `0x8042E9F4` hold the two words
   copied into `+0xD0`/`+0xD4`/`+0xD8`/`+0xE0`. The small-data section is not
   in the export, so the values are read at run time and stored verbatim; they
   are almost certainly `1.0f`/`0.0f` but that is not claimed.
2. **Clock conversion bias.** `0x80271820` subtracts the `double` at
   `0x8042E9F8` after building `0x4330_0000_xxxxxxxx`. The code reads the
   constant; the `2^52`/`2^52 + 2^31` interpretation is `// inferred:` from the
   same idiom in `resource/course.rs`.
3. **Track id.** `0x802719D0` compares `u32 @ track + 0x1C` against the caller
   range. That field is the track's sequence offset written by `0x802730D0`;
   whether the comparison is a "time window" or an "id range" is inferred.
4. **Dropped arguments.** Ghidra prints `0x80273100()` with no argument inside
   `0x80271640`/`0x802719D0`, but `r3` still holds the track being destroyed.
   The track is passed explicitly and the inference is marked in both
   docblocks. `0x8027B5F0()` is printed with no argument and returns a handle,
   so the module uses a local `call0_r` wrapper (the same pattern as
   `engine/display.rs`).
5. **Callback contract.** `0x80271910` passes a 16-byte stack block and
   restores one byte of the track after the call. The field meanings inside the
   block are inferred from their sources, not from a documented ABI.
6. **Player wrapper fields.** `0x802723A0` stores two table pointers
   (`0x8034F000`, `0x8034F038`) and clears a large field set. The offsets are
   observed; the class roles are inferred from the surrounding call sites.

## Deferred inventory

| Address | Size | Reason | Note |
|---|---:|---|---|
| `0x8008D460` | 952 B | P | Area OBB overlap test; paired-single/GQR listing, six unreachable-block warnings. |
| `0x8008D8B0` | 912 B | P | Same test via `0x8008D820`; paired-single/GQR listing. |
| `0x80271000` | 392 B | D | Full destructor; uses the opaque `0x802DCD74` save helper's 64-bit return as `(this, delete-flag)`. Needs a register-level pass. |
| `0x802712B0` | 660 B | D | Sequence start; overlaps the teardown translated here plus vtable track creation and a popcount of the track mask. |
| `0x80271BB0` | 300 B | D | Track poll; passes the low word of the `0x802DCD68` return to `0x802732A0`. |
| `0x80271E30`..`0x80272F40` | — | D | Track/sequence command interpreters; large switch bodies, next pass. |
| `0x802731D0`, `0x802732A0` | — | D | Track update/start helpers used by the interpreters. |

Reason codes: **P** paired-single needs an instruction-level pass, **D**
deferred to a later sequence pass (large switch or opaque 64-bit helper).

## Ledger

Machine-readable rows: `ledger/agents/wave7-course.csv` (module `audio::seq`,
19 rows, all `translated`) and the matching rows appended to
`ledger/functions.csv`. No course rows were added because no course body was
translated in this wave.
