# Coverage - Wave 29 audio (sound-manager stream and sequence transport) `0x80194DF0..0x80196830`

## How to read this file

This is the status page for the wave-29 pass over the audio lane of the
`main.dol` reconstruction. It answers three questions:

1. **What did this pass translate, and where is it?** - *Selected functions*
   gives one row per function with address, Rust name, plain-English role,
   cluster, status, confidence and evidence file.
2. **How do the functions fit the existing AX/JAS driver?** - *Clusters*
   explains the sound-manager release helpers, the BGM stream start/stop
   gates, the sequence-handle transport methods, and the status queries.
3. **What was deliberately left out, and why?** - *Deferred inventory* lists
   the nearby addresses that were not selected, followed by *Uncertainties*
   and the ledger pointer.

Column meanings:

- **Status** - `translated` means the entry/exit, every call and every
  observable memory write is accounted for. There are no `partial` or `stub`
  rows in this pass.
- **Confidence** - strength of the evidence, not code quality:
  `high` = the guards, arithmetic, call order and stores are explicit in the
  Ghidra C; `medium` = the flow is explicit but a dropped argument, a virtual
  slot or a record layout is inferred; `low` = mostly guesswork.
- **Reason codes** (deferred inventory):
  - **D** - duplicate: already translated by another lane.
  - **F** - float/double argument path; the integer call helpers used by this
    lane do not model FPR arguments.
  - **M** - large body reserved for a later pass.
  - **R** - audio-adjacent but not selected (needs a caller or record layout).
  - **S** - same family as a selected accessor; not selected here.

**Virtual addresses.** All addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`), not file offsets.

**Evidence.** The Ghidra C is under `work/ghidra/main-gb/export/`; the function
index is `work/ghidra/main-gb/export/functions.tsv`. Inline Rust markers follow
`docs/RUST_STYLE.md`: `// observed:` (visible in the Ghidra output), `// inferred:`
(role guessed from call sites), `// unknown:` (unresolved), `// partial:`
(deliberately incomplete).

**Promotion status.** All wave-29 functions live in `src/audio/ax.rs`. The
machine-readable ledger is `ledger/agents/wave29-audio.csv` (module
`audio::ax`, 25 rows, all `translated`). Earlier audio rows remain in
`ledger/agents/wave3-audio.csv`, `wave4-audio.csv`, `wave5-audio.csv`,
`wave7-audio.csv`, `wave11-audio.csv`, `wave18-audio.csv`,
`wave22-audio.csv` and `wave27-audio.csv`.

## Range inventory

Wave 29 is one contiguous band inside the game sound manager. The selected
entries are:

| Span | Purpose | Selected |
|---|---|---:|
| `0x80194DF0..0x80194F20` | manager release and BRSAR group lookups | 3 |
| `0x80195760..0x80195F10` | sequence dispatch, BGM stream start/stop and full reset | 13 |
| `0x80195FB0..0x80196010` | stream enable forwarding and cue-sequence query | 2 |
| `0x80196350..0x80196830` | virtual stream queries, counters and sequence status | 7 |

Totals for this wave: **25 translated, 0 partial, 0 stubs**;
**9 high, 16 medium, 0 low**.

## Selected functions

| Address | Rust | Role (plain English) | Cluster | Status | Confidence | Evidence |
|---|---|---|---|---|---|---|
| `0x80194DF0` | `fn_80194df0` | Release every shared audio object and the manager's cue handle. | A | translated | high | `80194df0_FUN_80194df0.c` |
| `0x80194E30` | `fn_80194e30` | Report a missing BRSAR bank-group lookup. | A | translated | high | `80194e30_FUN_80194e30.c` |
| `0x80194F20` | `fn_80194f20` | Report a missing second-form BRSAR group lookup. | A | translated | high | `80194f20_FUN_80194f20.c` |
| `0x80195760` | `fn_80195760` | Reset the `+0x910` handle and dispatch a sequence operation. | B | translated | medium | `80195760_FUN_80195760.c` |
| `0x801957C0` | `fn_801957c0` | Reset the `+0x90C` handle and dispatch a sequence operation. | B | translated | medium | `801957c0_FUN_801957c0.c` |
| `0x80195890` | `fn_80195890` | Start the BGM stream when a stream object is present. | B | translated | medium | `80195890_FUN_80195890.c` |
| `0x80195900` | `fn_80195900` | Release the BGM stream object at `+0x90C`. | B | translated | high | `80195900_FUN_80195900.c` |
| `0x80195960` | `fn_80195960` | Report a stream and mark a pending stop. | B | translated | high | `80195960_FUN_80195960.c` |
| `0x80195990` | `fn_80195990` | Service a pending BGM-stream stop. | B | translated | medium | `80195990_FUN_80195990.c` |
| `0x80195BF0` | `fn_80195bf0` | Release the cue sequence and re-seed track defaults. | A | translated | medium | `80195bf0_FUN_80195bf0.c` |
| `0x80195C50` | `fn_80195c50` | Release the auxiliary sequence at `+0x908`. | A | translated | medium | `80195c50_FUN_80195c50.c` |
| `0x80195D30` | `fn_80195d30` | Stop and release the BGM stream at `+0x910`. | A | translated | medium | `80195d30_FUN_80195d30.c` |
| `0x80195D90` | `fn_80195d90` | Stop the stream and clear its three parameters. | B | translated | medium | `80195d90_FUN_80195d90.c` |
| `0x80195E20` | `fn_80195e20` | Release the BGM sequence handle at `+0x90C`. | A | translated | high | `80195e20_FUN_80195e20.c` |
| `0x80195E80` | `fn_80195e80` | Run the stop/start transition for the manager. | B | translated | medium | `80195e80_FUN_80195e80.c` |
| `0x80195F10` | `fn_80195f10` | Reset every sound-manager handle and flag. | B | translated | medium | `80195f10_FUN_80195f10.c` |
| `0x80195FB0` | `fn_80195fb0` | Forward an enable/disable word to the active streams. | B | translated | medium | `80195fb0_FUN_80195fb0.c` |
| `0x80196010` | `fn_80196010` | Report whether the cue sequence exists and query it. | C | translated | medium | `80196010_FUN_80196010.c` |
| `0x80196350` | `fn_80196350` | Report whether the stream's virtual query is true. | C | translated | medium | `80196350_FUN_80196350.c` |
| `0x801963B0` | `fn_801963b0` | Report whether the stream's virtual query is false. | C | translated | medium | `801963b0_FUN_801963b0.c` |
| `0x801965B0` | `fn_801965b0` | Clear the sound-manager cue counters. | C | translated | high | `801965b0_FUN_801965b0.c` |
| `0x80196660` | `fn_80196660` | Read one sequence query word. | C | translated | medium | `80196660_FUN_80196660.c` |
| `0x801967A0` | `fn_801967a0` | Read the status halfword for the `+0x90C` handle. | C | translated | high | `801967a0_FUN_801967a0.c` |
| `0x80196820` | `fn_80196820` | Read the manager status byte at `+0x93C`. | C | translated | high | `80196820_FUN_80196820.c` |
| `0x80196830` | `fn_80196830` | Build the sequence status bitmask. | C | translated | medium | `80196830_FUN_80196830.c` |

## Clusters

### Cluster A - manager release and group lookups (`0x80194DF0..0x80195E20`)

The receiver is the `0x970`-byte sound-manager object. Its handle table is at
`+0x8F8`, `+0x8FC` and `+0x900..+0x910`:

| Field | Role |
|---|---|
| `+0x8F8` | first sequence pair |
| `+0x8FC` | second sequence pair |
| `+0x900` | player-sequence handle |
| `+0x904` | cue-sequence handle |
| `+0x908` | auxiliary-sequence handle |
| `+0x90C` | BGM sequence handle |
| `+0x910` | BGM stream handle |
| `+0x8F1..+0x8F4` | per-handle loaded/active/stop bytes |
| `+0x940..+0x942` | transition flags |

`fn_80194df0` clears the three SDA1 handle pairs and releases the cue handle
at `+0x94C`. `fn_80195bf0`, `fn_80195c50`, `fn_80195d30` and `fn_80195e20`
release one handle each through `0x80266250(handle, 0)`. `fn_80195bf0` also
re-seeds the active sequence track through `fn_8019df50`.

`fn_80194e30` and `fn_80194f20` look up one bank-group entry on
`*(manager + 0x5BC) + 0x0C` and normalise the result with
`countLeadingZeros(result) >> 5`, which is 1 only when the lookup returned 0.

### Cluster B - BGM stream transport (`0x80195760..0x80195FB0`)

`fn_80195760`/`fn_801957c0` reset the `+0x910`/`+0x90C` handle and call the
shared sequence dispatcher `0x80195300`.

`fn_80195890` starts the BGM stream: it requires `+0x910`, sets `+0x942`,
queries the stream through `fn_80196350`, releases the old stream object
through `0x80266230`, marks `+0x8F2` and clears `+0x938`. `fn_80195900`
releases a `+0x90C` stream object and sets `+0x940`. `fn_80195960` marks a
pending stop at `+0x8F4`. `fn_80195990` services that pending stop after the
stream's vtable `+0x14` query returns true.

`fn_80195d90` clears the two transition bytes, then applies
`0x80266440(stream, 1, param)` and the two parameter words
`0x80267760(stream, 0)` and `0x80267390(stream, 0)`, re-reading the handle
before each call. `fn_80195e80` is the stop/start gate: mode 0 enables the
system through `0x80196B90(manager, 1)`, re-seeds the track defaults, releases
the BGM sequence and stops the stream; a non-zero mode skips the enable and
track reset. `fn_80195f10` runs the four release helpers and clears the four
per-handle bytes.

`fn_80195fb0` forwards the control pair to the `+0x910` stream through
`0x80195F90` and to the `+0x90C` sequence through `0x80266440`.

### Cluster C - sequence status queries (`0x80196010..0x80196830`)

`fn_80196010` reports whether the cue sequence exists and calls
`0x802666C0(handle)` with it. `fn_80196350` and `fn_801963b0` call vtable slot
`+0x14` on the `+0x910` stream and return the two complementary truth values.

`fn_801965b0` clears the four cue counters at `+0x950..+0x95C`.
`fn_80196660` fills a one-word record through `0x802757E0` and returns it.
`fn_801967a0` reads the `+0x9C` id from the `+0x90C` handle, rejects `-1`, and
queries `0x80275760(system, id)`. `fn_80196820` returns the `+0x93C` byte.

`fn_80196830` is the status bitmask builder. It requires the `+0x910` stream,
the `+0x8F2` loaded byte and a non-busy `0x802667D0(stream)` result, builds a
temporary record from `+0x90C`, maps the first status halfword to the masks
`0/9/3/5/0x11/0x21/0x41/0x81` (other values map to 1), forces 0 when `+0x940`
is set, clears bit 0 while the active sequence state is `0x17` and the status
is not 8, then releases the temporary record.

## Deferred inventory

| Address | Symbol in export | Reason | Note |
|---|---|---|---|
| `0x801948B0..0x801949B0` | `FUN_801948b0..FUN_801949b0` | D | Already translated in `src/ui/lyt.rs`; not claimed twice. |
| `0x80194DA0` | `FUN_80194da0` | F | 68-byte manager reset with a float/double volume call. |
| `0x80195010` | `FUN_80195010` | R | 124-byte stream wrapper with a `0x802DCD74` register-save pair. |
| `0x80195090` | `FUN_80195090` | S | Twin of `0x80195010`; same register-save shape. |
| `0x80195110` | `FUN_80195110` | S | Twin of `0x80195010` for the second output pair. |
| `0x80195190` | `FUN_80195190` | S | Twin of `0x80195010` for the fourth virtual slot. |
| `0x80195300` | `FUN_80195300` | R | Shared sequence dispatcher; its virtual slot and record layout are not recovered. |
| `0x80195370` | `FUN_80195370` | S | 16-byte wrapper around `0x80195010`. |
| `0x80195380` | `FUN_80195380` | S | 16-byte wrapper around `0x80195090`. |
| `0x80195390` | `FUN_80195390` | S | 16-byte wrapper around `0x80195110`. |
| `0x801953A0` | `FUN_801953a0` | S | 16-byte wrapper around `0x80195190`. |
| `0x801953B0` | `FUN_801953b0` | R | 112-byte sequence-handle helper; needs its caller chain. |
| `0x80195420` | `FUN_80195420` | S | 112-byte twin of `0x801953b0`. |
| `0x80195490` | `FUN_80195490` | R | Enables the manager and calls the cue publisher; caller chain not recovered. |
| `0x801954A0` | `FUN_801954a0` | M | 104-byte cue-play primitive used by wave 18; not selected here. |
| `0x80195510` | `FUN_80195510` | F | 212-byte volume/envelope path with FPR arguments. |
| `0x801955F0` | `FUN_801955f0` | M | 368-byte sequence setup body. |
| `0x80195820` | `FUN_80195820` | M | 112-byte bank-group parameter publisher used by wave 18. |
| `0x80195A20` | `FUN_80195a20` | M | 460-byte manager body between the stream gates. |
| `0x80195CB0` | `FUN_80195cb0` | R | 120-byte player-sequence reset; uses an SDA2 default word. |
| `0x80195F90` | `FUN_80195f90` | S | 20-byte single-call receiver of `0x80266440`. |
| `0x80196050..0x80196150` | `FUN_80196050..FUN_80196150` | S | Six 60-byte `0x802666C0`/`0x80267420` wrappers. |
| `0x80196190` | `FUN_80196190` | M | 316-byte bank-group setup body. |
| `0x801962D0` | `FUN_801962d0` | F | 116-byte volume setter taking and storing doubles/floats. |
| `0x80196410` | `FUN_80196410` | R | 92-byte pane/record query with a five-word local record. |
| `0x80196470` | `FUN_80196470` | R | 124-byte state latch that calls the wave-22 cue helpers. |
| `0x801964F0` | `FUN_801964f0` | R | 92-byte BRSAR group parameter writer with a dropped argument. |
| `0x80196550` | `FUN_80196550` | F | 40-byte stream setter with float/double conversion. |
| `0x80196580` | `FUN_80196580` | F | 44-byte stream counter with float/double conversion. |
| `0x80196600` | `FUN_80196600` | R | 96-byte stack-callback query; needs the callback record layout. |
| `0x80196690` | `FUN_80196690` | R | 260-byte multi-stream status gate; one `0x80275760` argument is dropped by Ghidra. |
| `0x801969B0` | `FUN_801969b0` | M | 312-byte sound-system frame body after the selected band. |

## Uncertainties

1. **Manager identity.** The receiver is the `0x970`-byte object whose handle
   table is at `+0x8F8..+0x910`. The field names "cue/auxiliary/BGM" are
   descriptive; the image has no class metadata.
2. **Dropped arguments.** Ghidra prints one-argument release calls for
   `0x80266230`/`0x80266250`; the Rust uses the two-argument shape already
   established by wave 22. The `0x80195F90` and `0x80196B90` receivers are
   restored from the surrounding manager argument.
3. **Virtual queries.** `fn_80195990`, `fn_80196350` and `fn_801963b0` call
   vtable slot `+0x14` on the `+0x910` stream. The slot offset is explicit; the
   method's role is inferred.
4. **Temporary record.** `fn_80196830` uses a three-word record and a two-word
   status buffer, matching the Ghidra stack shapes. The callee `0x80272C40`
   owns the real layout.
5. **Status masks.** The switch values in `fn_80196830` are observed; their
   meaning (sequence state bits) is not named in the image.
6. **Group lookups.** `fn_80194e30`/`fn_80194f20` normalise the lookup result
   with `countLeadingZeros >> 5`; the BRSAR entry type is not recovered.

## Ledger

Machine-readable rows: `ledger/agents/wave29-audio.csv` (module `audio::ax`,
25 rows, all `translated`). Earlier rows remain in
`ledger/agents/wave3-audio.csv`, `wave4-audio.csv`, `wave5-audio.csv`,
`wave7-audio.csv`, `wave11-audio.csv`, `wave18-audio.csv`,
`wave22-audio.csv` and `wave27-audio.csv`.
