# Coverage - Wave 31 audio (sequence dispatch, stream records, JAI timer records) `0x80195300..0x801ABA80`

## How to read this file

This is the status page for the wave-31 pass over the audio lane of the
`main.dol` reconstruction. It answers three questions:

1. **What did this pass translate, and where is it?** - *Selected functions*
   gives one row per function with address, Rust name, plain-English role,
   cluster, status, confidence and evidence file.
2. **How do the functions fit the existing AX/JAS driver?** - *Clusters*
   explains the shared sequence dispatcher, the pair operators, the
   stream-record hooks and the JAI timer records.
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

**Promotion status.** All wave-31 functions live in `src/audio/ax.rs`. The
machine-readable ledger is `ledger/agents/wave31-audio.csv` (module
`audio::ax`, 22 rows, all `translated`). Earlier audio rows remain in
`ledger/agents/wave3-audio.csv`, `wave4-audio.csv`, `wave5-audio.csv`,
`wave7-audio.csv`, `wave11-audio.csv`, `wave18-audio.csv`,
`wave22-audio.csv`, `wave27-audio.csv` and `wave29-audio.csv`.

## Range inventory

Wave 31 is not one contiguous band. It starts in the sequence-dispatch block
that wave 29 left below the sound-manager transport and ends in the JAI timer
band:

| Span | Purpose | Selected |
|---|---|---:|
| `0x80195300..0x801953A0` | sequence dispatcher and the four pair operators | 5 |
| `0x801953B0..0x801954A0` | sequence-handle release/start helpers | 3 |
| `0x80195820`, `0x80196050` | bank-group publisher and stream-state query | 2 |
| `0x80196410..0x80196B90` | stream record, position bitmask and transition flags | 5 |
| `0x801A9AA0..0x801ABA80` | JAS table selectors and JAI voice-slot/timer records | 7 |

Totals for this wave: **22 translated, 0 partial, 0 stubs**;
**14 high, 8 medium, 0 low**.

## Selected functions

| Address | Rust | Role (plain English) | Cluster | Status | Confidence | Evidence |
|---|---|---|---|---|---|---|
| `0x80195300` | `fn_80195300` | Dispatch a sequence operation by virtual slot or BRSAR lookup. | A | translated | medium | `80195300_FUN_80195300.c` |
| `0x80195370` | `fn_80195370` | Run the first sequence pair through `0x80195010`. | A | translated | high | `80195370_FUN_80195370.c` |
| `0x80195380` | `fn_80195380` | Run the first sequence pair through `0x80195090`. | A | translated | high | `80195380_FUN_80195380.c` |
| `0x80195390` | `fn_80195390` | Run the second sequence pair through `0x80195110`. | A | translated | high | `80195390_FUN_80195390.c` |
| `0x801953A0` | `fn_801953a0` | Run the second sequence pair through `0x80195190`. | A | translated | high | `801953a0_FUN_801953a0.c` |
| `0x801953B0` | `fn_801953b0` | Release and restart the player-sequence handle at `+0x904`. | B | translated | medium | `801953b0_FUN_801953b0.c` |
| `0x80195420` | `fn_80195420` | Release and restart the auxiliary handle at `+0x908`. | B | translated | medium | `80195420_FUN_80195420.c` |
| `0x801954A0` | `fn_801954a0` | Release and restart the player handle at `+0x900`. | B | translated | medium | `801954a0_FUN_801954a0.c` |
| `0x80195820` | `fn_80195820` | Publish one bank-group pair through the two sequence dispatches. | C | translated | medium | `80195820_FUN_80195820.c` |
| `0x80196050` | `fn_80196050` | Report the `+0x910` stream and service it. | C | translated | medium | `80196050_FUN_80196050.c` |
| `0x80196410` | `fn_80196410` | Read the `+0x219` presence byte from the stream record. | D | translated | high | `80196410_FUN_80196410.c` |
| `0x80196470` | `fn_80196470` | Latch the presence byte and enable/pause the BGM stream. | D | translated | medium | `80196470_FUN_80196470.c` |
| `0x801969B0` | `fn_801969b0` | Map the BGM playback position to a status bitmask. | D | translated | high | `801969b0_FUN_801969b0.c` |
| `0x80196AF0` | `fn_80196af0` | Advance or clear the `+0x940` transition countdown. | D | translated | high | `80196af0_FUN_80196af0.c` |
| `0x80196B90` | `fn_80196b90` | Set the manager enable word or bump the pending counter. | D | translated | high | `80196b90_FUN_80196b90.c` |
| `0x801A9AA0` | `fn_801a9aa0` | Return the JAS sequence sample-table base. | E | translated | high | `801a9aa0_FUN_801a9aa0.c` |
| `0x801A9AB0` | `fn_801a9ab0` | Return a DSP descriptor slot by index. | E | translated | high | `801a9ab0_FUN_801a9ab0.c` |
| `0x801AB040` | `fn_801ab040` | Read one JAI voice-slot word. | E | translated | high | `801ab040_FUN_801ab040.c` |
| `0x801AB280` | `fn_801ab280` | Reset JAI timer slot 8 and re-arm its record. | E | translated | medium | `801ab280_FUN_801ab280.c` |
| `0x801AB2E0` | `fn_801ab2e0` | Clear the first two words of a JAI timer record. | E | translated | high | `801ab2e0_FUN_801ab2e0.c` |
| `0x801ABA70` | `fn_801aba70` | Store the timer-record callback word. | E | translated | high | `801aba70_FUN_801aba70.c` |
| `0x801ABA80` | `fn_801aba80` | Read the timer-record callback word. | E | translated | high | `801aba80_FUN_801aba80.c` |

## Clusters

### Cluster A - sequence dispatcher and pair operators (`0x80195300..0x801953A0`)

`fn_80195300` is the shared dispatcher used by [`fn_80195760`] and
[`fn_801957c0`]. A zero `mode` forwards `(manager, first, second)` to the
manager's virtual slot `+0xC8`. A non-zero `mode` builds the three-word selector
`{1, 0, mode}` and calls `0x8027B270(*(manager + 0x5BC) + 0x0C, first, second,
&record)`, returning `countLeadingZeros(result) >> 5` (1 for "not found").

The four 16-byte wrappers `fn_80195370`, `fn_80195380`, `fn_80195390` and
`fn_801953a0` forward `(manager, manager + 0x8F8|0x8FC, first, second)` to the
four register-save operators `0x80195010`, `0x80195090`, `0x80195110` and
`0x80195190`. Those operators are not translated in this pass (their bodies use
a CodeWarrior register-save thunk), but the wrapper argument lists are explicit.

### Cluster B - sequence-handle release/start (`0x801953B0..0x801954A0`)

Each helper releases its handle through `0x80266250` when non-zero, sets one
loaded byte, and invokes a manager virtual method with the handle slot:

| Function | Handle | Release count | Flag | Virtual slot |
|---|---|---:|---|---:|
| [`fn_801953b0`] | `+0x904` | 0 | `+0x8F1 = 1` | `+0xB4` |
| [`fn_80195420`] | `+0x908` | 10 | `+0x8F3 = 1` | `+0xB0` |
| [`fn_801954a0`] | `+0x900` | 0 | none | `+0xB0` |

`fn_80195820` (cluster C) is the caller-facing publisher that runs the `+0x910`
and `+0x90C` dispatches in that order.

### Cluster C - bank-group publisher and stream query (`0x80195820`, `0x80196050`)

`fn_80195820` dispatches `handle_a` through `0x80195760` (the `+0x910` handle)
and, when `handle_b` is non-zero, `handle_b` through `0x801957c0` (the `+0x90C`
handle). Both callees are typed `void`, but the first call's `r3` is returned by
the export; the Rust keeps the raw call helper so that register value survives.

`fn_80196050` is the `+0x910` existence test and calls `0x802666C0` when the
stream exists. It is the first of the six original query wrappers; the five
neighbours that call the float setter `0x80267420` are deferred under reason
**F**.

### Cluster D - stream record, position and transition flags (`0x80196410..0x80196B90`)

`fn_80196410` captures the five-word stream record through `0x8027FC50`, reads
the presence byte at record `+0x219`, and releases the record through
`0x8027FD50`. `fn_80196470` uses that byte as a latch at `manager + 0x96C` and
calls `0x8019D0F0` (enable) or `0x8019D140` (pause) on transitions.

`fn_801969b0` maps the playback position from `0x8027D440(record + 0x110)` to
the masks below; a missing record or a negative position returns 0:

| position | mask |
|---|---:|
| below `0x5EBA00` | 0 |
| `0x5EBA00..0x5FBD26` | 1 |
| `0x5FBD27..0x6043AF` | 3 |
| `0x6043B0..0x60A5FF` | 7 |
| `0x60A600..0x610FFF` | `0xF` |
| `0x611000..0x6112C8` | `0x1F` |
| `0x6112C9` and above | `0xFF` |

`fn_80196af0` advances the `+0x941` byte while `+0x940` is set and clears both
after eleven steps. `fn_80196b90` clears `+0x944`/`+0x948` when enabled, or
increments `+0x944` while `+0x942` is set.

### Cluster E - JAS tables and JAI timer records (`0x801A9AA0..0x801ABA80`)

`fn_801a9aa0` returns the fixed sample-table base `0x80331880`;
`fn_801a9ab0` returns `0x80331A80 + index * 0x10`. Both are used by the wave-11
sequence resamplers (`0x801A8C10`, `0x801A9780`).

`fn_801ab040` reads a word from the JAI slot table at `r13 - 0x5038` using the
same 8-bit index mask as [`fn_801ab020`]. `fn_801ab280` resets slot 8 when it is
not the sentinel `0x801AB990`, clears the timer head/tail at
`r13 - 0x5000`/`r13 - 0x4FFC`, exchanges slot 8 with zero, and re-inserts the
timer record `0x80332760` through `0x801B32D0`.

`fn_801ab2e0`, `fn_801aba70` and `fn_801aba80` are the timer-record accessors:
clear the first two words, store the callback at `+0x28`, and read it back.

## Deferred inventory

| Address | Symbol in export | Reason | Note |
|---|---|---|---|
| `0x80194DA0` | `FUN_80194da0` | F | 68-byte manager reset with a float/double volume call. |
| `0x80195010` | `FUN_80195010` | R | 124-byte register-save operator behind the first pair wrapper. |
| `0x80195090` | `FUN_80195090` | S | Twin of `0x80195010` for virtual slot `+0xB4`. |
| `0x80195110` | `FUN_80195110` | S | Twin of `0x80195010` for virtual slot `+0xBC`. |
| `0x80195190` | `FUN_80195190` | S | Twin of `0x80195010` for virtual slot `+0xC0`. |
| `0x80195490` | `FUN_80195490` | R | 16-byte flag set plus a call whose arguments are dropped. |
| `0x80195510` | `FUN_80195510` | F | 212-byte volume/envelope path with FPR arguments. |
| `0x801955F0` | `FUN_801955f0` | F | 368-byte sequence setup body with double arguments. |
| `0x80195A20` | `FUN_80195a20` | M | 460-byte BGM refresh body; needs the full record layout. |
| `0x80195CB0` | `FUN_80195cb0` | F | 120-byte player-sequence reset with a double argument. |
| `0x80195F90` | `FUN_80195f90` | R | 20-byte stream receiver; its control pair is dropped. |
| `0x80196190` | `FUN_80196190` | F | 316-byte bank-group setup with floats. |
| `0x801962D0` | `FUN_801962d0` | F | 116-byte volume setter taking and storing doubles/floats. |
| `0x801964F0` | `FUN_801964f0` | R | 92-byte BRSAR group parameter writer with a 64-bit first argument. |
| `0x80196550` | `FUN_80196550` | F | 40-byte stream setter with a double argument. |
| `0x80196580` | `FUN_80196580` | F | 44-byte stream counter with a double argument. |
| `0x80196600` | `FUN_80196600` | R | 96-byte stack-callback query; needs the callback record layout. |
| `0x80196690` | `FUN_80196690` | R | 260-byte multi-stream status gate; one `0x80275760` argument is dropped. |
| `0x8019B220` | `FUN_8019b220` | M | 412-byte sound-manager body; needs the full field map. |
| `0x8019B9F0` | `FUN_8019b9f0` | S | 8-byte constant `return 1`. |
| `0x8019BED0` | `FUN_8019bed0` | M | 640-byte manager state body. |
| `0x8019CA00` | `FUN_8019ca00` | F | 620-byte cue restart with float constants and GQR artifacts. |
| `0x8019E080` | `FUN_8019e080` | R | 604-byte BRSAR sequence select; its 64-bit lock token needs a record layout. |
| `0x8019E2E0` | `FUN_8019e2e0` | R | 476-byte sequence stop/start gate; caller chain not recovered. |
| `0x8019E4C0` | `FUN_8019e4c0` | R | 552-byte cue apply path. |
| `0x8019E6F0` | `FUN_8019e6f0` | R | 228-byte track volume/fade apply. |
| `0x8019E8E0` | `FUN_8019e8e0` | F | 92-byte envelope re-apply with doubles. |
| `0x8019E940` | `FUN_8019e940` | F | 84-byte envelope/volume apply with float arguments. |
| `0x8019E9A0` | `FUN_8019e9a0` | F | 120-byte sequence volume setter with a double argument. |
| `0x8019EF90` | `FUN_8019ef90` | F | 124-byte volume clamp; calls the float accessor pair. |
| `0x8019F010` | `FUN_8019f010` | F | 64-byte double-precision spline helper. |
| `0x8019F050` | `FUN_8019f050` | F | 64-byte duplicate of `0x8019F010`. |
| `0x8019F490` | `FUN_8019f490` | R | 460-byte JAS archive setup with a register-save loop. |
| `0x8019F660` | `arc_init_handle` | D | Already translated in `src/resource/bres.rs`; not duplicated here. |
| `0x8019F700` | `FUN_8019f700` | M | 672-byte ARC path builder used by future archive work. |
| `0x8019F9B0` | `arc_file_info_by_index` | D | Already translated in `src/resource/bres.rs`; not duplicated here. |
| `0x8019FA00` | `FUN_8019fa00` | M | 624-byte ARC path walker. |
| `0x8019FC80` | `FUN_8019fc80` | R | 456-byte ARC path copier; needs the handle layout. |
| `0x8019FEA0` | `FUN_8019fea0` | R | 88-byte ARC iterator with a dropped path argument. |
| `0x8019FF00` | `FUN_8019ff00` | R | 124-byte ARC iterator; path argument dropped. |
| `0x8019FF80` | `FUN_8019ff80` | R | 188-byte ARC directory walker. |
| `0x801A08F0` | `FUN_801a08f0` | M | 940-byte reverse-bank DSP setup with indirect callbacks. |
| `0x801A2E70` | `FUN_801a2e70` | M | 700-byte JAS voice pool initialiser. |
| `0x801A9B20..0x801A9CF0` | `FUN_801a9b20..FUN_801a9cf0` | S | Empty compiler/FP helper stubs in the JAS band. |
| `0x801AB1D0` | `FUN_801ab1d0` | R | 132-byte date-string builder; not audio-facing. |
| `0x801AB2F0` | `FUN_801ab2f0` | M | 592-byte sorted timer insertion; the deadline compare is 64-bit. |
| `0x801AB540` | `set_alarm` | D | Already translated in `src/runtime/os.rs`; not duplicated here. |
| `0x801ABCF0` | `FUN_801abcf0` | R | 120-byte CodeWarrior heap unlink; runtime, not audio. |
| `0x801ABF60` | `FUN_801abf60` | R | 544-byte DSP/AI MMIO polling sequence. |
| `0x801AC180` | `FUN_801ac180` | M | 460-byte DSP reset loop with unreachable blocks. |
| `0x801AC350` | `FUN_801ac350` | M | 204-byte AI/DSP stop loop with MMIO waits. |

## Uncertainties

1. **Manager identity.** The receiver is the `0x970`-byte object whose handle
   table is at `+0x8F8..+0x910`. The field names "player/cue/auxiliary/BGM" are
   descriptive; the image has no class metadata.
2. **Virtual calls.** `fn_80195300`, `fn_801953b0`, `fn_80195420` and
   `fn_801954a0` call vtable slots `+0xC8`, `+0xB4` and `+0xB0`. The slot
   offsets are explicit, but Ghidra drops the argument lists and the roles are
   inferred.
3. **`fn_80195820` return value.** Both callees are typed `void`, yet the
   export returns the first call's `r3`. The Rust uses the raw call helper to
   preserve that register value; the value's meaning is unknown.
4. **Stream record layout.** `fn_80196410` and `fn_801969b0` treat the captured
   object as a five-word record and read `+0x219`/`+0x110`. The callee
   `0x8027FC50` owns the real layout; only the offsets used here are observed.
5. **`fn_80196470` receiver.** Ghidra drops the receiver of the first
   `0x80196410` call; the Rust restores `manager` from the other two calls.
6. **JAI slot 8.** `0x801AB280` compares slot 8 with the code address
   `0x801AB990` and calls `0x801AB020(8, 0)` even though the export shows one
   argument; the zero is inferred from the exchange helper.
7. **Timer queue names.** "Head" and "tail" follow the existing wave-5
   constants (`r13 - 0x5000` head, `r13 - 0x4FFC` tail). The queue is shared
   with the OS alarm code; the JAI ownership is inferred from the record fields.

## Ledger

Machine-readable rows: `ledger/agents/wave31-audio.csv` (module `audio::ax`,
22 rows, all `translated`). Earlier rows remain in
`ledger/agents/wave3-audio.csv`, `wave4-audio.csv`, `wave5-audio.csv`,
`wave7-audio.csv`, `wave11-audio.csv`, `wave18-audio.csv`,
`wave22-audio.csv`, `wave27-audio.csv` and `wave29-audio.csv`.
