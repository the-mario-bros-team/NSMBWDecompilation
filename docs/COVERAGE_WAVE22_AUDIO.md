# Coverage - Wave 22 audio (sound lifecycle, sequence handle, DSP buffers) `0x8019B1D0..0x801A08D0`

## How to read this file

This is the status page for the wave-22 pass over the audio lane of the
`main.dol` reconstruction. It answers three questions:

1. **What did this pass translate, and where is it?** - *Selected functions*
   gives one row per function with address, Rust name, plain-English role,
   cluster, status, confidence and evidence file.
2. **How do the functions fit the existing AX/JAS driver?** - *Clusters*
   explains the game-sound object lifecycle, the JAS object and sequence
   queries, the sequence-handle constructors and cue-table selectors, the
   fixed-constant helper, and the DSP buffer address getters.
3. **What was deliberately left out, and why?** - *Deferred inventory* lists
   the nearby addresses that were not selected, followed by *Uncertainties*
   and the ledger pointer.

Column meanings:

- **Status** - `translated` means the entry/exit, every call and every
  observable memory write is accounted for. There are no `partial` or `stub`
  rows in this pass.
- **Confidence** - strength of the evidence, not code quality:
  `high` = the guards, arithmetic, call order and stores are explicit in the
  Ghidra C; `medium` = the flow is explicit but a callee role, an SDA default
  word or a query record is inferred; `low` = mostly guesswork.
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

**Promotion status.** All wave-22 functions live in `src/audio/ax.rs`. The
machine-readable ledger is `ledger/agents/wave22-audio.csv` (module
`audio::ax`, 25 rows, all `translated`). Earlier audio rows remain in
`ledger/agents/wave3-audio.csv`, `wave4-audio.csv`, `wave5-audio.csv`,
`wave7-audio.csv`, `wave11-audio.csv` and `wave18-audio.csv`.

## Range inventory

Wave 22 is not one contiguous band: it starts just below the wave-18 cue
hooks, continues through the JAS sequence helpers, and ends at the DSP buffer
selectors. The selected entries are:

| Span | Purpose | Selected |
|---|---|---:|
| `0x8019B1D0..0x8019B560` | game-sound object lifecycle | 7 |
| `0x8019D290..0x8019DC90` | JAS release and sound queries | 3 |
| `0x8019DDD0..0x8019E7E0` | sequence handle, cue tables and fade fields | 8 |
| `0x8019F090` | fixed-constant division helper | 1 |
| `0x801A06D0..0x801A08D0` | DSP buffer address selectors | 6 |

Totals for this wave: **25 translated, 0 partial, 0 stubs**;
**17 high, 8 medium, 0 low**.

## Selected functions

| Address | Rust | Role (plain English) | Cluster | Status | Confidence | Evidence |
|---|---|---|---|---|---|---|
| `0x8019B1D0` | `fn_8019b1d0` | Clear the two companion sound words. | A | translated | high | `8019b1d0_FUN_8019b1d0.c` |
| `0x8019B1E0` | `fn_8019b1e0` | Construct the base sound object. | A | translated | medium | `8019b1e0_FUN_8019b1e0.c` |
| `0x8019B440` | `fn_8019b440` | Return or create the shared sound object. | A | translated | medium | `8019b440_FUN_8019b440.c` |
| `0x8019B490` | `fn_8019b490` | Clear the shared sound object words. | A | translated | high | `8019b490_FUN_8019b490.c` |
| `0x8019B4A0` | `fn_8019b4a0` | Construct the `0x58`-byte sound object. | A | translated | medium | `8019b4a0_FUN_8019b4a0.c` |
| `0x8019B4F0` | `fn_8019b4f0` | Reset every sound-object field. | A | translated | high | `8019b4f0_FUN_8019b4f0.c` |
| `0x8019B560` | `fn_8019b560` | Shut the sound object down by state. | A | translated | high | `8019b560_FUN_8019b560.c` |
| `0x8019D290` | `fn_8019d290` | Release a collected JAS object. | B | translated | high | `8019d290_FUN_8019d290.c` |
| `0x8019D420` | `fn_8019d420` | Report whether the sound is idle. | B | translated | high | `8019d420_FUN_8019d420.c` |
| `0x8019DC90` | `fn_8019dc90` | Test whether a cue may start. | B | translated | medium | `8019dc90_FUN_8019dc90.c` |
| `0x8019DDD0` | `fn_8019ddd0` | Return or create the shared sequence handle. | C | translated | high | `8019ddd0_FUN_8019ddd0.c` |
| `0x8019DE20` | `fn_8019de20` | Clear the shared sequence words. | C | translated | high | `8019de20_FUN_8019de20.c` |
| `0x8019DE30` | `fn_8019de30` | Construct the `0x6C`-byte sequence handle. | C | translated | medium | `8019de30_FUN_8019de30.c` |
| `0x8019DEC0` | `fn_8019dec0` | Reset the sequence-handle fields. | C | translated | medium | `8019dec0_FUN_8019dec0.c` |
| `0x8019DF50` | `fn_8019df50` | Re-seed the sequence track defaults. | C | translated | medium | `8019df50_FUN_8019df50.c` |
| `0x8019DFE0` | `fn_8019dfe0` | Read a cue-table value (table pair A). | D | translated | high | `8019dfe0_FUN_8019dfe0.c` |
| `0x8019E030` | `fn_8019e030` | Read a cue-table value (table pair B). | D | translated | high | `8019e030_FUN_8019e030.c` |
| `0x8019E7E0` | `fn_8019e7e0` | Store the sequence fade parameters. | D | translated | high | `8019e7e0_FUN_8019e7e0.c` |
| `0x8019F090` | `fn_8019f090` | Divide the `0x1D4C00` tick constant. | E | translated | medium | `8019f090_FUN_8019f090.c` |
| `0x801A06D0` | `fn_801a06d0` | Buffer bank A with the enable gate. | F | translated | high | `801a06d0_FUN_801a06d0.c` |
| `0x801A0710` | `fn_801a0710` | Buffer bank A with the second index. | F | translated | high | `801a0710_FUN_801a0710.c` |
| `0x801A07B0` | `fn_801a07b0` | Buffer bank B with the enable gate. | F | translated | high | `801a07b0_FUN_801a07b0.c` |
| `0x801A07F0` | `fn_801a07f0` | Buffer bank B with the second index. | F | translated | high | `801a07f0_FUN_801a07f0.c` |
| `0x801A0890` | `fn_801a0890` | Buffer bank C with the enable gate. | F | translated | high | `801a0890_FUN_801a0890.c` |
| `0x801A08D0` | `fn_801a08d0` | Buffer bank C with the second index. | F | translated | high | `801a08d0_FUN_801a08d0.c` |

## Clusters

### Cluster A - game-sound object lifecycle (`0x8019B1D0..0x8019B560`)

The shared sound object lives at `r13 - 0x51D8`, with a companion copy of the
pointer at `r13 - 0x51D4`. `fn_8019b440` is the lazy getter: it allocates
`0x58` bytes through `0x802B9210`, constructs the object with
`fn_8019b4a0`, and stores the pointer in both SDA words. `fn_8019b490`
clears the pair without freeing the object.

The constructor chain is `fn_8019b1e0` (base vtable `0x8032E0C8` plus the
four-channel reset `0x802D6C30`) and `fn_8019b4a0` (derived vtable
`0x8032E27C` plus `fn_8019b4f0`). `fn_8019b4f0` clears the state word
`+0x14`, the cue mask `+0x1C`, the flag byte `+0x30`, the cue id `+0x34`,
the `+0x38..+0x44` control bytes, sets `+0x1E = 0xFF`, and initialises the
12-byte sub-object at `+0x48` with `0x80228DD0(object + 0x48, 4)`.

`fn_8019b560` is the shutdown dispatcher. It clears the object flags, calls
the per-group scalar re-apply `0x8019D480`, and then branches on the saved
state `+0x14` to set a sound-system parameter, reset the sequence track
defaults, stop the sequence, or play cue `0x74F`/`0x788`. State `0x11` is a
no-op.

### Cluster B - JAS release and sound queries (`0x8019D290..0x8019DC90`)

`fn_8019d290` releases one collected JAS object: a null object or a non-zero
child word at `+0x9C` returns immediately, otherwise `0x80266250(object, 0)`
runs the release path.

`fn_8019d420` is the idle query: flag byte `+0x30` with no bit above bit 0
returns 1; otherwise the sound system is asked through `0x801963B0` and the
answer is inverted.

`fn_8019dc90` is the cue admission test. State `0x11` with cue-mask bit 0 clear
and a zero `+0x3C` byte produces the initial 1. The function then fills a
seven-word record through `0x80275780` from sound-system `+0x100`; result
codes `1`, `4`, `6` and `9` clear the answer. A non-zero `+0x3D` then forces
the result to 1.

### Cluster C - sequence handle (`0x8019DDD0..0x8019DF50`)

The sequence handle lives at `r13 - 0x51D0` with its companion at
`r13 - 0x51CC`. `fn_8019ddd0` lazy-creates a `0x6C`-byte handle through
`0x802B9210` and `fn_8019de30`; `fn_8019de20` clears both pointer words.
`fn_8019de30` installs vtable `0x8032E288` and seeds `+0x10` with `-1`,
`+0x14`/`+0x18` with `0x76`, `+0x1C..+0x34` with zero, `+0x38`/`+0x3C` with
`0x76`, `+0x40`/`+0x44` with the SDA2 default at `r2 - 0x51A8`, and `+0x48`
with 1. `fn_8019dec0` repeats that seed and adds the `+0x4C..+0x68` fields
(`0x75`, 1, `-1`, 1, `0x0F`); `fn_8019df50` only re-seeds `+0x38..+0x48`.

### Cluster D - cue tables and fade fields (`0x8019DFE0..0x8019E7E0`)

`fn_8019dfe0` and `fn_8019e030` read one word from two 28-entry tables of
two-word records. The bound is `0x1B`; the table pair is selected by the
sound object's `+0x38` byte. Pair A uses `0x802F66F0`/`0x802F67D8`; pair B
uses `0x802F68C0`/`0x802F69A8`. `fn_8019e7e0` stores the sequence fade
parameters at `+0x60` (duration), `+0x64` (mode byte) and `+0x68` (value), and
clamps a non-positive duration to zero.

### Cluster E - fixed-constant helper (`0x8019F090`)

`fn_8019f090` divides decimal `1,920,000` (`0x1D4C00`) by its argument and
returns `-1` for a zero divisor. The surrounding functions in this band do
fixed-point audio math; the constant's role is not named in the image.

### Cluster F - DSP buffer selectors (`0x801A06D0..0x801A08D0`)

Six getters write a buffer address to their output word. Banks A and B use a
`0x180` stride at `0x80378D20` and `0x80379F20`; bank C uses a `0x120` stride
at `0x8037B120`. The `*D0` variants are gated by the JAS callback slots A/B/C
at `r13 - 0x5140`/`-0x5144`/`-0x5148` and write zero when the slot is null;
the `*10`/`*F0`/`*D0` companion variants always write the address and use the
second index at `r13 - 0x5174`. The first index is `r13 - 0x5170`.

## Deferred inventory

| Address | Symbol in export | Reason | Note |
|---|---|---|---|
| `0x8019B220` | `FUN_8019b220` | M | 412-byte sound manager body; needs the full field map. |
| `0x8019B9F0` | `FUN_8019b9f0` | S | 8-byte constant `return 1`. |
| `0x8019BED0` | `FUN_8019bed0` | M | 640-byte manager state body. |
| `0x8019C370` | `FUN_8019c370` | R | 348-byte cue helper; no selected caller yet. |
| `0x8019CA00` | `FUN_8019ca00` | F | 620-byte cue restart with float constants and GQR artifacts. |
| `0x8019CE00` | `FUN_8019ce00` | R | 112-byte flag helper; needs a caller. |
| `0x8019D2D0` | `FUN_8019d2d0` | R | 336-byte per-group volume re-apply. |
| `0x8019D480` | `FUN_8019d480` | M | 2052-byte per-group scalar/volume table path. |
| `0x8019E8E0` | `FUN_8019e8e0` | F | 92-byte envelope re-apply; calls `0x80195510` with doubles. |
| `0x8019E940` | `FUN_8019e940` | F | 84-byte envelope/volume apply; passes float arguments. |
| `0x8019E9A0` | `FUN_8019e9a0` | F | 120-byte sequence volume setter; first argument is a double. |
| `0x8019EF90` | `FUN_8019ef90` | F | 124-byte volume clamp; calls the float accessor pair. |
| `0x8019F010` | `FUN_8019f010` | F | 64-byte double-precision spline helper. |
| `0x8019F050` | `FUN_8019f050` | F | 64-byte duplicate of `0x8019F010`. |
| `0x8019F660` | `arc_init_handle` | D | Already translated in `src/resource/bres.rs`. |
| `0x8019F9B0` | `arc_file_info_by_index` | D | Already translated in `src/resource/bres.rs`. |
| `0x801A0730` | `FUN_801a0730` | S | Same DSP selector family, reverse bank, `0x600` stride. |
| `0x801A0750` | `FUN_801a0750` | S | Same DSP selector family, reverse bank, `0x600` stride. |
| `0x801A0770` | `FUN_801a0770` | S | Same DSP selector family, reverse bank, `0x600` stride. |
| `0x801A0790` | `FUN_801a0790` | S | Same DSP selector family, reverse bank, `0x600` stride. |
| `0x801A0810` | `FUN_801a0810` | S | Same DSP selector family, `0x600` stride. |
| `0x801A0830` | `FUN_801a0830` | S | Same DSP selector family, `0x600` stride. |
| `0x801A0850` | `FUN_801a0850` | S | Same DSP selector family, `0x600` stride. |
| `0x801A0870` | `FUN_801a0870` | S | Same DSP selector family, `0x600` stride. |
| `0x801A2E70` | `FUN_801a2e70` | M | 700-byte JAS voice pool initialiser. |
| `0x801AB540` | `FUN_801ab540` | R | 112-byte JAI timer setter; the 64-bit deadline split needs a record layout. |
| `0x801AB5B0` | `FUN_801ab5b0` | R | 132-byte timer-add wrapper with a 64-bit argument pair. |
| `0x801ABCF0` | `FUN_801abcf0` | R | 120-byte JAI block unlink; one argument is dropped by Ghidra. |
| `0x801ABF60` | `FUN_801abf60` | R | 544-byte DSP/AI MMIO polling sequence. |

The sound-system primitives `0x801954A0`, `0x80195F10`, `0x801963B0`,
`0x80196600`, `0x8019D480`, `0x8019E2E0`, `0x80266250` and `0x80275780`
remain cross-lane callees and are called by address. The two former opaque
helpers `0x8019B4F0` and `0x8019E7E0` are now translated in this lane.

## Uncertainties

1. **Sound-object ownership.** `r13 - 0x51D8`/`-0x51D4` and
   `r13 - 0x51D0`/`-0x51CC` are pointer pairs written by the constructors and
   cleared by the teardown helpers. The class layout is inferred from the
   writers and readers; no RTTI record is present in the image.
2. **Base constructor.** `0x802BA2D0` is called with the object pointer at
   every constructor site, but its export omits the argument. The Rust passes
   the object explicitly and marks the role inferred.
3. **SDA2 default.** `r2 - 0x51A8` seeds the sequence id/mode words
   `+0x40`/`+0x44`. Its producer is outside this band, so the field role and
   the default value are inferred.
4. **Query record.** `0x80275780` is exported with one argument, but the call
   site passes a sound-system object, the cue id and a stack destination. The
   Rust keeps the three-argument call and reserves seven words for the record;
   the caller's checked word is the second one.
5. **Cue tables.** The four table bases, the 28-entry bound and the two-word
   record stride are observed. The table contents and the meaning of the
   `+0x38` selector are not recovered.
6. **Fixed constant.** `0x1D4C00` is decimal `1,920,000`; the division shape
   suggests a tick/tempo conversion but no caller in this band fixes the unit.
7. **DSP bank roles.** The getters index the three backing stores cleared by
   `fn_801a05b0`; the names "bank A/B/C" are descriptive, not recovered
   symbols. The `0x600`-stride getters in the same family are deferred because
   their banks are not touched by the selected callers.
8. **Float paths.** The sequence envelope/volume helpers `0x8019E8E0`,
   `0x8019E940`, `0x8019E9A0`, `0x8019EF90` and the spline helpers
   `0x8019F010`/`0x8019F050` take or pass FPR arguments. This lane models
   calls with the integer `platform::call0..call7` helpers, so those functions
   are deferred rather than given a host-inaccurate ABI.

## Ledger

Machine-readable rows: `ledger/agents/wave22-audio.csv` (module `audio::ax`,
25 rows, all `translated`). Earlier rows remain in
`ledger/agents/wave3-audio.csv`, `wave4-audio.csv`, `wave5-audio.csv`,
`wave7-audio.csv`, `wave11-audio.csv` and `wave18-audio.csv`.
