# Coverage - Wave 27 audio (cue tables, DSP banks, JAS levels, JAI timers) `0x8019DF80..0x801AB5B0`

## How to read this file

This is the status page for the wave-27 pass over the audio lane of the
`main.dol` reconstruction. It answers three questions:

1. **What did this pass translate, and where is it?** - *Selected functions*
   gives one row per function with address, Rust name, plain-English role,
   cluster, status, confidence and evidence file.
2. **How do the functions fit the existing AX/JAS driver?** - *Clusters*
   explains the companion cue tables, the reverse-bank DSP selectors, the JAS
   level accessors, the voice counter/limit/table accessors, and the JAI
   voice-slot and timer wrappers.
3. **What was deliberately left out, and why?** - *Deferred inventory* lists
   the nearby addresses that were not selected, followed by *Uncertainties*
   and the ledger pointer.

Column meanings:

- **Status** - `translated` means the entry/exit, every call and every
  observable memory write is accounted for. There are no `partial` or `stub`
  rows in this pass.
- **Confidence** - strength of the evidence, not code quality:
  `high` = the guards, arithmetic, call order and stores are explicit in the
  Ghidra C; `medium` = the flow is explicit but a callee role, an SDA base or a
  register-save thunk is inferred; `low` = mostly guesswork.
- **Reason codes** (deferred inventory):
  - **D** - duplicate: already translated by another lane.
  - **F** - float/double argument path; the integer call helpers used by this
    lane do not model FPR arguments.
  - **M** - large body reserved for a later pass.
  - **R** - audio-adjacent but not selected (needs a caller or class layout).
  - **S** - same family as a selected accessor; not selected here.

**Virtual addresses.** All addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`), not file offsets.

**Evidence.** The Ghidra C is under `work/ghidra/main-gb/export/`; the function
index is `work/ghidra/main-gb/export/functions.tsv`. Inline Rust markers follow
`docs/RUST_STYLE.md`: `// observed:` (visible in the Ghidra output), `// inferred:`
(role guessed from call sites), `// unknown:` (unresolved), `// partial:`
(deliberately incomplete).

**Promotion status.** All wave-27 functions live in `src/audio/ax.rs`. The
machine-readable ledger is `ledger/agents/wave27-audio.csv` (module
`audio::ax`, 25 rows, all `translated`). Earlier audio rows remain in
`ledger/agents/wave3-audio.csv`, `wave4-audio.csv`, `wave5-audio.csv`,
`wave7-audio.csv`, `wave11-audio.csv`, `wave18-audio.csv` and `wave22-audio.csv`.

## Range inventory

Wave 27 is not one contiguous band: it starts below the wave-22 cue tables,
continues through the DSP bank selectors and the JAS level/voice accessors, and
ends at the JAI timer wrapper. The selected entries are:

| Span | Purpose | Selected |
|---|---|---:|
| `0x8019DF80..0x8019ECA0` | companion cue tables and one release wrapper | 3 |
| `0x801A00B0` | JAS initialisation-latch getter | 1 |
| `0x801A0730..0x801A0870` | reverse-bank DSP buffer selectors | 8 |
| `0x801A18C0..0x801A1930` | JAS 16-bit level getters and setters | 6 |
| `0x801A2180..0x801A3360` | parameter block and voice counter/limit/table accessors | 5 |
| `0x801AB020..0x801AB5B0` | JAI voice-slot exchange and periodic timer | 2 |

Totals for this wave: **25 translated, 0 partial, 0 stubs**;
**24 high, 1 medium, 0 low**.

## Selected functions

| Address | Rust | Role (plain English) | Cluster | Status | Confidence | Evidence |
|---|---|---|---|---|---|---|
| `0x8019DF80` | `fn_8019df80` | Read the first word of a cue-table C entry. | A | translated | high | `8019df80_FUN_8019df80.c` |
| `0x8019DFB0` | `fn_8019dfb0` | Read the first word of a cue-table D entry. | A | translated | high | `8019dfb0_FUN_8019dfb0.c` |
| `0x8019ECA0` | `fn_8019eca0` | Release an object when the count is positive. | A | translated | high | `8019eca0_FUN_8019eca0.c` |
| `0x801A00B0` | `fn_801a00b0` | Read the JAS initialisation latch. | B | translated | high | `801a00b0_FUN_801a00b0.c` |
| `0x801A0730` | `fn_801a0730` | Bank-A extra 3 by the first index. | C | translated | high | `801a0730_FUN_801a0730.c` |
| `0x801A0750` | `fn_801a0750` | Bank-A extra 1 by the second index. | C | translated | high | `801a0750_FUN_801a0750.c` |
| `0x801A0770` | `fn_801a0770` | Bank-A extra 2 by the second index. | C | translated | high | `801a0770_FUN_801a0770.c` |
| `0x801A0790` | `fn_801a0790` | Bank-A extra 3 by the second index. | C | translated | high | `801a0790_FUN_801a0790.c` |
| `0x801A0810` | `fn_801a0810` | Bank-B extra 3 by the first index. | C | translated | high | `801a0810_FUN_801a0810.c` |
| `0x801A0830` | `fn_801a0830` | Bank-B extra 1 by the second index. | C | translated | high | `801a0830_FUN_801a0830.c` |
| `0x801A0850` | `fn_801a0850` | Bank-B extra 2 by the second index. | C | translated | high | `801a0850_FUN_801a0850.c` |
| `0x801A0870` | `fn_801a0870` | Bank-B extra 3 by the second index. | C | translated | high | `801a0870_FUN_801a0870.c` |
| `0x801A18C0` | `fn_801a18c0` | Read JAS level 1. | D | translated | high | `801a18c0_FUN_801a18c0.c` |
| `0x801A18D0` | `fn_801a18d0` | Read JAS level 2. | D | translated | high | `801a18d0_FUN_801a18d0.c` |
| `0x801A18E0` | `fn_801a18e0` | Read JAS level 3. | D | translated | high | `801a18e0_FUN_801a18e0.c` |
| `0x801A1910` | `fn_801a1910` | Write JAS level 1. | D | translated | high | `801a1910_FUN_801a1910.c` |
| `0x801A1920` | `fn_801a1920` | Write JAS level 2. | D | translated | high | `801a1920_FUN_801a1920.c` |
| `0x801A1930` | `fn_801a1930` | Write JAS level 3. | D | translated | high | `801a1930_FUN_801a1930.c` |
| `0x801A2180` | `fn_801a2180` | Return the parameter-block address. | E | translated | high | `801a2180_FUN_801a2180.c` |
| `0x801A25B0` | `fn_801a25b0` | Read the voice generation counter. | E | translated | high | `801a25b0_FUN_801a25b0.c` |
| `0x801A2E20` | `fn_801a2e20` | Read the voice parameter-table base. | E | translated | high | `801a2e20_FUN_801a2e20.c` |
| `0x801A3350` | `fn_801a3350` | Write the voice-cost ceiling. | E | translated | high | `801a3350_FUN_801a3350.c` |
| `0x801A3360` | `fn_801a3360` | Read the voice count. | E | translated | high | `801a3360_FUN_801a3360.c` |
| `0x801AB020` | `fn_801ab020` | Exchange one JAI voice-slot word. | F | translated | high | `801ab020_FUN_801ab020.c` |
| `0x801AB5B0` | `fn_801ab5b0` | Arm a periodic JAI timer. | F | translated | medium | `801ab5b0_FUN_801ab5b0.c` |

## Clusters

### Cluster A - companion cue tables and release (`0x8019DF80..0x8019ECA0`)

`fn_8019df80` and `fn_8019dfb0` read the first word of a 28-entry,
two-word-record table at `0x802F6530` and `0x802F6610`. They differ from the
wave-22 selectors [`fn_8019dfe0`] and [`fn_8019e030`]: the receiver is ignored
and there is no `+0x38` selector byte, so the table is fixed. Out-of-range
indices return 0.

`fn_8019eca0` is the release wrapper: a null object or a non-positive count
skips the call; otherwise it calls the shared deallocator `0x802B9280(object)`
and returns the object pointer unchanged.

### Cluster B - JAS initialisation latch (`0x801A00B0`)

`fn_801a00b0` exposes the word at `r13 - 0x5188`, which
[`fn_801a0050`] sets to 1 after the one-shot JAS bring-up. The getter itself
has no side effects.

### Cluster C - reverse-bank DSP selectors (`0x801A0730..0x801A0870`)

The eight getters write an address into their output word and have no enable
gate. Banks A and B use a `0x600` stride. The six extra blocks are the same
ones the untranslated `0x801A08F0` pairs with the three `0x180`-stride stores
and the three `0x120`-stride DSP buffers from wave 22:

| Base | Block | Index |
|---|---|---|
| `0x80378EA0` | bank A extra 1 | `r13 - 0x5174` |
| `0x80379020` | bank A extra 2 | `r13 - 0x5174` |
| `0x803791A0` | bank A extra 3 | `r13 - 0x5170` and `-0x5174` |
| `0x8037A0A0` | bank B extra 1 | `r13 - 0x5174` |
| `0x8037A220` | bank B extra 2 | `r13 - 0x5174` |
| `0x8037A3A0` | bank B extra 3 | `r13 - 0x5170` and `-0x5174` |

The names "extra 1/2/3" are descriptive, not recovered symbols.

### Cluster D - JAS levels (`0x801A18C0..0x801A1930`)

Three 16-bit read/write pairs cover the levels at `r13 - 0x512C`,
`r13 - 0x512E` and `r13 - 0x5130`. They are the same four slots that
[`fn_801a1850`] initialises to `0x8000`; level 0 at `r13 - 0x512A` already has
its clamp/store setter [`fn_801a18f0`].

### Cluster E - parameter block and voice counters (`0x801A2180..0x801A3360`)

`fn_801a2180` returns the fixed address `0x8037CDA0`, the 20-pair parameter
block written by [`fn_801a2270`] and cleared by [`fn_801a23f0`]. The other four
accessors expose the voice generation counter (`r13 - 0x5068`), the parameter
table base (`r13 - 0x5078`), the cost ceiling (`r13 - 0x5060`) and the voice
count (`r13 - 0x506C`).

### Cluster F - JAI voice slots and timer (`0x801AB020..0x801AB5B0`)

`fn_801ab020` exchanges one word in the table pointed to by `r13 - 0x5038`;
the byte index is masked to eight bits and scaled by four, and the previous
word is returned. `fn_801ab5b0` arms a periodic timer: the incoming `r3` is the
timer behind the `0x802DCD70` save thunk, `+0x1C` receives the period, `+0x18`
receives the caller's zero word, `0x801B6020(base_hi, base_lo)` produces the
64-bit deadline stored at `+0x20`, and `0x801AB2F0` inserts the record with
interrupts disabled.

## Deferred inventory

| Address | Symbol in export | Reason | Note |
|---|---|---|---|
| `0x8019B220` | `FUN_8019b220` | M | 412-byte sound manager body; needs the full field map. |
| `0x8019B9F0` | `FUN_8019b9f0` | S | 8-byte constant `return 1`. |
| `0x8019BED0` | `FUN_8019bed0` | M | 640-byte manager state body. |
| `0x8019CA00` | `FUN_8019ca00` | F | 620-byte cue restart with float constants and GQR artifacts. |
| `0x8019D2D0` | `FUN_8019d2d0` | R | 336-byte per-group volume re-apply. |
| `0x8019D480` | `FUN_8019d480` | M | 2052-byte per-group scalar/volume table path. |
| `0x8019DF40` | `thunk_FUN_8019e800` | S | 4-byte thunk into the sequence fade path. |
| `0x8019E080` | `FUN_8019e080` | R | 604-byte BRSAR sequence select; its 64-bit lock token needs a record layout. |
| `0x8019E2E0` | `FUN_8019e2e0` | R | 476-byte sequence stop/start gate; caller chain not recovered. |
| `0x8019E4C0` | `FUN_8019e4c0` | R | 552-byte cue apply path. |
| `0x8019E6F0` | `FUN_8019e6f0` | R | 228-byte track volume/fade apply. |
| `0x8019E8E0` | `FUN_8019e8e0` | F | 92-byte envelope re-apply; calls `0x80195510` with doubles. |
| `0x8019E940` | `FUN_8019e940` | F | 84-byte envelope/volume apply; passes float arguments. |
| `0x8019E9A0` | `FUN_8019e9a0` | F | 120-byte sequence volume setter; first argument is a double. |
| `0x8019EF90` | `FUN_8019ef90` | F | 124-byte volume clamp; calls the float accessor pair. |
| `0x8019F010` | `FUN_8019f010` | F | 64-byte double-precision spline helper. |
| `0x8019F050` | `FUN_8019f050` | F | 64-byte duplicate of `0x8019F010`. |
| `0x801A08F0` | `FUN_801a08f0` | M | 940-byte reverse-bank DSP setup with indirect callbacks. |
| `0x801A2E70` | `FUN_801a2e70` | M | 700-byte JAS voice pool initialiser. |
| `0x801AB1D0` | `FUN_801ab1d0` | R | 132-byte date-string builder in the JAI band; not audio-facing. |
| `0x801AB280` | `FUN_801ab280` | R | 88-byte timer-list teardown; needs the slot-8 owner record. |
| `0x801AB2E0` | `FUN_801ab2e0` | S | 16-byte timer-record clear. |
| `0x801AB540` | `set_alarm` | D | Already translated in `src/runtime/os.rs`; not duplicated here. |
| `0x801ABCF0` | `FUN_801abcf0` | R | 120-byte JAI block unlink; one argument is dropped by Ghidra. |
| `0x801ABF60` | `FUN_801abf60` | R | 544-byte DSP/AI MMIO polling sequence. |

## Uncertainties

1. **Cue tables.** The bounds, stride and four bases are observed. The table
   contents and the meaning of the two new tables are not recovered; the
   Rust keeps the raw table bases.
2. **DSP bank names.** "Bank A/B", "extra 1/2/3" and "reverse bank" are
   descriptive. The blocks are inferred from the layout in `0x801A08F0`; no
   symbol table survives in the image.
3. **Level ordering.** "Level 1/2/3" follows the existing wave-3 names for
   `r13 - 0x512C`/`-0x512E`/`-0x5130`; the roles of the four levels are not
   named.
4. **JAI voice-slot table.** `r13 - 0x5038` points at a word table. The
   eight-bit index mask implies 256 entries; the table size is inferred.
5. **JAI timer record.** `0x801AB5B0` stores the period at `+0x1C`, the zero
   word at `+0x18` and the 64-bit deadline at `+0x20`. The field names follow
   the writers; `0x801B6020` only adds its argument pair to a boot-time base.
6. **Save thunks.** `0x802DCD70`/`0x802DCD74` are CodeWarrior register-save
   epilogues, not functions. The Rust takes the incoming timer pointer
   explicitly and omits the matching restore thunks.
7. **One-shot timer.** `0x801AB540` is the same routine as `runtime::os::set_alarm`,
   so it is not translated a second time in this lane.

## Ledger

Machine-readable rows: `ledger/agents/wave27-audio.csv` (module `audio::ax`,
25 rows, all `translated`). Earlier rows remain in
`ledger/agents/wave3-audio.csv`, `wave4-audio.csv`, `wave5-audio.csv`,
`wave7-audio.csv`, `wave11-audio.csv`, `wave18-audio.csv` and `wave22-audio.csv`.
