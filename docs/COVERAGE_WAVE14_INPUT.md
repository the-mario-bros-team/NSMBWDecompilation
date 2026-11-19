# Coverage — wave 14 input (`0x801D4000–0x80200000`, `runtime::input`)

## How to read this file

This page records exactly what the wave-14 input lane added to
[`src/runtime/input.rs`](../src/runtime/input.rs), not what the whole address
band contains. Read it in five steps:

1. **Scope** gives the raw inventory count, the eligibility rule and the
   before/after totals for the band.
2. The **function table** lists the 25 new bodies in address order, with the
   plain-English role, the exact Ghidra export and the confidence.
3. **Clusters** explains the five groups and how they connect to the WPAD/KPAD
   code already in the module.
4. **Corrections and caveats** records the one shared-helper correction and the
   places where the decompiler output needed interpretation.
5. **Ledger** points at the machine-readable rows.

Status vocabulary (`docs/READING_GUIDE.md`): `translated` means the full control
flow and every call are accounted for; `partial` means entry/exit and major
effects only; `stub` means address/role only. Confidence is evidence strength
(high / medium / low), not code quality.

Every body cites `FUN_<address>` and
`work/ghidra/main-gb/export/<addr>_<name>.c`. No official symbol names were
invented: descriptive names are used only where a string or the body shape
proves the role; everything else keeps `fn_80xxxxxx`. `// observed:`,
`// inferred:` and `// unknown:` markers inside the Rust carry the same
information at the call site.

## Scope

Source index: `work/ghidra/main-gb/export/functions.tsv`; translated baseline:
`ledger/functions.csv` before this wave.

| Item | Count |
|---|---:|
| Entry addresses in `[0x801D4000, 0x80200000)` | 580 |
| Excluded, size `< 16` bytes | 45 |
| Eligible, size `>= 16` bytes | 535 |
| Already translated in this band before wave 14 | 72 |
| **Selected and translated by this wave** | **25** |
| Eligible untranslated after this wave | 438 |
| Remaining small (`< 16`) entries | 45 |

The 25 new bodies represent **4,736 bytes**, all of them eligible entries. The
band is still mostly untranslated (438 eligible entries): this wave was scoped
to the WPAD/KPAD queue, report-state, link-key and balance-board helpers around
the already translated report pipeline, not to a full sweep.

Selection rule: the address must be an entry in `functions.tsv`, must not have
a row in `ledger/functions.csv`, and the Ghidra body must have complete control
flow. Large paired-single DSP-style bodies and the two giant state machines
were deliberately deferred (see the end of this file).

## Function table

| Address | Rust name | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x801DDAC0` | `wbc_read_dummy_801ddac0` | Balance-board `WBCRead` placeholder: print the name, return `-1`. | translated | high | `801ddac0_FUN_801ddac0.c` |
| `0x801DDAF0` | `wbc_set_zero_point_dummy_801ddaf0` | Balance-board `WBCSetZEROPoint` placeholder: print, return `-1`. | translated | high | `801ddaf0_FUN_801ddaf0.c` |
| `0x801DDB20` | `wbc_get_tgc_weight_dummy_801ddb20` | Balance-board `WBCGetTGCWeight` placeholder: print, return `-1`. | translated | high | `801ddb20_FUN_801ddb20.c` |
| `0x801DDB50` | `fn_801ddb50` | Enable/disable the WPAD link from the Bluetooth state and mode. | translated | medium | `801ddb50_FUN_801ddb50.c` |
| `0x801DDF00` | `fn_801ddf00` | Battery percentage from `+0x983`/`+0x984` with low-battery hysteresis. | translated | medium | `801ddf00_FUN_801ddf00.c` |
| `0x801DE910` | `fn_801de910` | Counts stable report families; resets the latch after 600 samples. | translated | medium | `801de910_FUN_801de910.c` |
| `0x801E4EC0` | `fn_801e4ec0` | Queue a one-byte extension-register write (kind `0x16`). | translated | high | `801e4ec0_FUN_801e4ec0.c` |
| `0x801E5080` | `fn_801e5080` | Queue a buffer write (kind `0x16`). | translated | high | `801e5080_FUN_801e5080.c` |
| `0x801E5240` | `fn_801e5240` | Queue an extension-register read (kind `0x17`). | translated | high | `801e5240_FUN_801e5240.c` |
| `0x801E53F0` | `fn_801e53f0` | Clear a command ring's indices and all `capacity * 0x30` slots. | translated | high | `801e53f0_FUN_801e53f0.c` |
| `0x801E7960` | `fn_801e7960` | Decode one balance-board load-cell sample into the pad frame. | translated | medium | `801e7960_FUN_801e7960.c` |
| `0x801EA520` | `fn_801ea520` | XOR-decode an extension buffer with the two eight-byte channel keys. | translated | medium | `801ea520_FUN_801ea520.c` |
| `0x801EA7F0` | `fn_801ea7f0` | Clear the five per-channel motion fields at `+0x998`. | translated | high | `801ea7f0_FUN_801ea7f0.c` |
| `0x801EA820` | `fn_801ea820` | Link/attached check, then queue a kind `0x16` command with a callback. | translated | high | `801ea820_FUN_801ea820.c` |
| `0x801F6160` | `fn_801f6160` | Locked word read from the 16-entry link-pointer table. | translated | high | `801f6160_FUN_801f6160.c` |
| `0x801F61C0` | `fn_801f61c0` | Locked `u16` read from the first link-value table. | translated | high | `801f61c0_FUN_801f61c0.c` |
| `0x801F6220` | `fn_801f6220` | Locked `u16` read from the second link-value table. | translated | high | `801f6220_FUN_801f6220.c` |
| `0x801F6280` | `fn_801f6280` | Locked read of the connected-device count byte. | translated | high | `801f6280_FUN_801f6280.c` |
| `0x801F62D0` | `fn_801f62d0` | Store or clear one channel's six-byte link key and latch the change. | translated | high | `801f62d0_FUN_801f62d0.c` |
| `0x801F6350` | `fn_801f6350` | Compare a six-byte key with the stored link key. | translated | high | `801f6350_FUN_801f6350.c` |
| `0x801F63B0` | `fn_801f63b0` | Flush a pending key change and restart sampling. | translated | medium | `801f63b0_FUN_801f63b0.c` |
| `0x801F6400` | `fn_801f6400` | Store one word in the link-pointer table. | translated | high | `801f6400_FUN_801f6400.c` |
| `0x801F6420` | `fn_801f6420` | Read one word from the link-pointer table. | translated | high | `801f6420_FUN_801f6420.c` |
| `0x801F6440` | `fn_801f6440` | Store one `u16` in the first link-value table. | translated | high | `801f6440_FUN_801f6440.c` |
| `0x801F6460` | `fn_801f6460` | Store one `u16` in the second link-value table. | translated | high | `801f6460_FUN_801f6460.c` |

Totals for this wave: **25 translated, 0 partial, 0 stubs**; confidence split
19 high / 6 medium / 0 low. Evidence paths are relative to
`work/ghidra/main-gb/export/`; the full file name is the address (lowercase, no
`0x`) plus `_FUN_<address>.c`, as in the table.

## Clusters

### Command queue — `0x801E4EC0`–`0x801EA820`

The queue header is the same `tail u8, head u8, base u32, capacity u32` object
used by wave 5's `fn_801e4c50` (`control + 0x160` or `+0x5EC`). The three new
builders fill a 0x30-byte block and append it:

* `0x801E4EC0` writes one byte (`0x55`/`0xAA` at the call sites) to one
  extension register; the payload starts with the register word and carries a
  fixed 21-byte outgoing length at `+0x1A`.
* `0x801E5080` copies a caller buffer after the register word; the low five
  bits of the length go to `+8`, the full length to the copy.
* `0x801E5240` is the read form: register at `+4` and `+0x24`, length at `+8`
  and `+0x20`, destination at `+0x1C`, outgoing length 6 at `+0x1A`.
* `0x801E53F0` clears the ring and all slots under the lock.
* `0x801EA820` reads `+0x8BC`/`+0x8DC` under the lock, then queues a kind
  `0x16` block with `register | 0x4A40000`. The completion is invoked with
  `(channel, result)` when the result is non-zero.

The single-ring helpers are intentionally separate from `fn_801e4c50`: that
body handles kind `0x12` command encoding, while these three handle the
extension write/read forms.

### Report state and extension bytes — `0x801DE910`–`0x801EA7F0`

* `0x801DE910` increments `+0x912` once per sample when the state word at
  `+0x8B8` matches report family `3..=5` with report word `0x1C10`, or family
  `6..=8` with either `0x1C10` or (`0x1450`, clean marker). Above 600 it
  clears `+0x911`/`+0x912` under the lock.
* `0x801DDF00` converts the two battery fields to a percentage and runs the
  75/80/85 hysteresis that drives `+0x8CD`/`+0x8CE`.
* `0x801E7960` decodes the four balance-board cells. The `0xFF` sentinel is
  zeroed, the two axes are expanded with `<< 3` and `*-8 + 0x2FF`, and the
  weight is the `scale * n * n` product using the constant at `0x8042E338`.
* `0x801EA520` XORs each byte with the key at `+0x92C` and adds the key at
  `+0x924`; the key phase is `(base + i) & 7` because both tables are exactly
  eight bytes (cleared by `0x801DF570`).
* `0x801EA7F0` is the per-channel reset hook installed by `0x801DF570`; it
  clears the five motion fields at `+0x998`–`+0x9A8`.

### WPAD enable/disable — `0x801DDB50`

The function is the state gate around the Bluetooth connection state. State
`0` and the forced first argument return `1`; states `1`, `2` and `4` return
`0`; state `3` continues. If the link is already ready, `0x801F3860` is
notified and the function returns `0`. Otherwise mode `0` installs the two
event callbacks, modes `1`, `4`, `6` disable, mode `5` follows
`r13-0x4F8C`, and every other mode enables. The disable path either prints the
"Wait for %d ms until start reconnect!" message or does nothing when
`r13-0x4C00` is clear. The enable path clears all four link keys, stops the
alarm, and runs the two Bluetooth teardown steps.

### Bluetooth link-key and link-table slots — `0x801F6160`–`0x801F6460`

The layer has three parallel 16-entry tables: a word table at `0x803A3EE0`
and two `u16` tables at `0x803A5288`/`0x803A52A8`. The `0x801F6160`/`61C0`/
`6220` readers range-check the index (`< 0x10`); the `0x801F6400`/`6420`/
`6440`/`6460` accessors mask it with `0xFF` and do not range-check, exactly as
Ghidra shows. `0x801F62D0` writes a six-byte key into the `0x46`-byte record at
`0x803A39D9 + (channel + 10) * 0x46`, `0x801F6350` compares against the same
record, and `0x801F63B0` flushes the change through `0x801DD500` before
restarting sampling.

### Balance-board API dummies — `0x801DDAC0`–`0x801DDB20`

These three bodies are placeholders in this build: each prints its own name
(`"WBCReadDummy"`, `"WBCSetZEROPointDummy"`, `"WBCGetTGCWeightDummy"`) through
`0x801EA8F0` and returns `-1`. No balance-board read, zero-point write or TGC
weight calculation is present, so the Rust keeps that behaviour rather than
inventing one.

## Corrections and caveats

1. **Queue-kind placement (corrected).** The queue consumer `0x801DDD20`
   loads the first 32-bit word and tests `word & 0xFF`. Because the target is
   big-endian, the kind is block byte `+3`, not byte `+0`. The shared
   `kpad_cmd_block` helper now writes the kind at `+3`, which also corrects the
   existing builders that use it (`fn_801e1dd0`, `fn_801dfe00`,
   `fn_801e0820`, `fn_801e3680` and `fn_801e4c50`); the two manual builders
   `fn_801e12c0` and `fn_801e1630` were moved to `+3` as well. Their ledger
   rows stay `translated`; only the block byte position changed.
2. **`0x801EA520` key phase.** The decompiler prints a misleading
   `param_4 * 0x20000000` term for the key index. The raw instructions at
   `0x801EA520` compute the index from `base + i`; the eight-byte table size
   (two `memset` calls of 8 bytes in `0x801DF570`) makes the phase `& 7`. The
   Rust marks this `// inferred:` in place.
3. **`0x801E7960` output shape.** The first argument is an address of a word
   that holds the report-block pointer; the function is not the same shape as
   the other decoders, which take the block pointer directly. The translation
   keeps the double indirection as `out_slot`.
4. **`0x801E7960` signed-byte conversion.** Ghidra expands
   `(int)(signed char)` through a `0x43300000` double bias. The Rust uses
   `as i8 as f32`, which produces the same value and avoids importing a
   floating-point bit-cast helper.
5. **`0x801DDB50` mode numbers.** The numeric modes `0`–`6` and the word at
   `r13-0x4F8C` are observed; their public names are not. The Rust keeps the
   numeric comparisons and documents the branch shapes.
6. **`0x801DDF00` field units.** The arithmetic and all thresholds are exact;
   whether `+0x983`/`+0x984` are raw battery counts or a scaled percentage is
   not proven, so the fields keep numeric names/offsets.
7. **Eight-byte readers dropped.** The four `8`-byte `WBC` getters at
   `0x801DDA50`–`0x801DDA80` were considered but not selected: they are below
   the `< 16` eligibility floor. The three `WBC` dummies at `0x801DDAC0`–
   `0x801DDB20` (48 bytes each) were kept because they have real bodies.
8. **Deferred neighbours.** `0x801E55C0` (2,792 bytes) uses paired-single
   `ldexpf` artifacts and was deferred. `0x801E9730`/`0x801E9E20` (1,776/1,784
   bytes) are the two RNG-based MotionPlus key derivations; they are complete
   but dense and were deferred to a dedicated pass. `0x801E1E20` (5,628) and
   `0x801E39C0` (4,224) are the two large WPAD/KPAD state machines.
9. **No new staging.** All 25 bodies live in `src/runtime/input.rs`; no
   `src/subsystems/*` file was touched.

## Ledger

Machine-readable rows: `ledger/agents/wave14-input.csv` (25 rows, module
`runtime::input`, same address order as the table above).
