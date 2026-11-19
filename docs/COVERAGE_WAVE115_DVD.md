# Coverage - Wave 115 (runtime::dvd, the last address of the `0x801D0000` page)

## How to read this file

The `runtime::dvd` lane owns `src/runtime/dvd.rs`, the single writable file for
the `0x801D0000` 64 KiB page of `main.dol`. That page is much wider than the
disc band, so the body below is not disc code; it is here only because this is
the one file assigned to the bucket.

`high` means every store, mask and branch in the body is literal and its role
is proven; `medium` means the instruction sequence is literal but at least one
field role, callee identity or data layout is inferred; `low` means only the
store sequence is certain.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/dvd.rs` | `runtime::dvd` | 1 | 2524 | 0 | 1 |

## What this pass covers

This pass takes the **one** address that the `0x801D0000` bucket still had
open, `0x801DEB70` (2524 bytes). Waves 109 and 111 both skipped it on the
grounds that the export builds the twelve-word kind-`0x10` request descriptor
from stack words the function never writes. Re-reading the export together
with the consumer shows that the unwritten words are dead on this call, so the
address is translated as [`fn_801deb70`].

- **The function is the WPAD/Bluetooth periodic worker.** In Bluetooth state
  `3` it either counts down a pending reconnect at `r13-0x4BFC` or runs the
  periodic update: a `60000`-tick WiFi-channel refresh (the channel byte is
  cache-invalidated through `0x801AC440` before it is read), a four-knob
  settings-store re-read gated by `0x801DB8E0`, and a visit to all four
  `0x8039E330` channel blocks. A channel with a zero `+0x8D8` word is skipped.
- **The two control-block rings.** Ring `+0x5EC` is drained only while the
  per-channel state byte at `r13-0x4C08+i` is `5`; its block must be kind
  `0x12` or `+0x840` must be set. Ring `+0x160` is drained when the first ring
  produced nothing. Both use the same `tail u8`/`head u8`/`base u32`/
  `capacity u32` header already used by the `runtime::input` queue.
- **The link decision.** The link word at `+0x8BC` is read under the interrupt
  lock (`0x801B1140`/`0x801B1180`). Zero submits the copied block to
  `0x801DDD20` and pops the ring slot with `memset` (`0x800046B4`); `-2` runs
  the stale-link retry. The retry divides `(now - stored)` by
  `bus_clock / 4` with `0x802DCEC4`; `cmpwi r4,1` then tests the low word of the
  `r3:r4` quotient with a signed greater-than.
- **The kind-`0x10` status query.** When both rings are empty and the
  `+0x8D0` status request has been active for five ticks, the body sends a
  kind-`0x10` block with a one-byte payload and a zero completion word.
- **The per-channel tail.** `0x801DE9D0` and `0x801DDF00` run for every active
  channel, and `+0x991`/`+0x992` is a small countdown that clears the enable
  byte after the stored timer value has already gone negative. The three
  global period counters wrap at `10`, `5` and `60000` (inclusive), then
  `0x801FF91C` is notified.
- **Bluetooth state `2`.** The routine arms the connect/disconnect callbacks
  `0x801E0820`/`0x801E0CE0` through `0x801F3BF0`/`0x801F3B90` exactly once,
  gated by `r13-0x4BD4`, and seeds the reconnect countdown with `0x32`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801DEB70` | `fn_801deb70` | 2524 | medium |

## Notes on the recovered operands and stores

- **The Wave 109/111 skip defect, measured and resolved.** At
  `0x801DF3D8`-`0x801DF444` the original copies twelve words from the frame
  area `r1+0x18`-`r1+0x44` into `r1+0x48`-`r1+0x74` and passes `r1+0x48` to
  `0x801DDD20`. Only four stores touch the source area: `stw r30,0x18(r1)`
  with `r30 = 0x10`, `stb r29,0x1C(r1)` with `r29 = 0`, `sth r31,0x32(r1)`
  with `r31 = 1`, and `stw r29,0x44(r1)`. That is 11 defined bytes out of 48;
  the other 37 are never written anywhere in the function.
- **Why the undefined words do not block the translation.** The consumer
  `0x801DDD20` masks word 0 with `0xFF` and, for kind `0x10`, takes the first
  branch (it only restores `+0x8BC`). It reads the length half-word at `+0x1A`
  (which is the `1` above), then `memcpy`s that many bytes from block `+4`
  (the zero above). The fields `puVar8[7]`, `puVar8[8]`, `puVar8[9]`,
  `puVar8[10]` and `puVar8[0xB]` are read only for kinds `0x17`/`0x15`; the
  zero completion word at `+0x2C` is not read for kind `0x10`
  (`work/ghidra/main-gb/export/801ddd20_FUN_801ddd20.c` lines 25-77).
- **The `extraout_r4`.** `0x802DCEC4` is `__divdi3`; the 64-bit quotient is
  returned in `r3:r4`, and `0x801DF248`/`0x801DF30C` test the low word
  `extraout_r4`. The body uses [`dvd_call4_u64`], the same wrapper the
  `0x801D06A0` translation already uses, and tests `quotient as u32 > 1`.
- **The prologue calls.** `0x802DCD4C`/`0x802DCD98` are the CodeWarrior
  `_savegpr`/`_restgpr` helpers. The explicit `stw` blocks at
  `0x801DEE3C`-`0x801DEE68`, `0x801DEEE4`-`0x801DEF10` and
  `0x801DF140`-`0x801DF214` are compiler-generated register/stack traffic, not
  source-level state, and are not reproduced.
- **Inferred roles.** The `+0x8D0` status-request counter, the `+0x840`
  link-loss gate, the `+0x8C0` link-present byte and the `+0x8C3` device id
  are named from the consumer and the sibling `runtime::input` translations.
  The kind values `0x10`, `0x12`, `0x15` and `0x17` are kept as raw bytes.

## Deliberate skips

None. The bucket's remaining open address was translated. The only
non-literal part of the body is the zero fill of the kind-`0x10` block's dead
stack bytes, documented in the function's `# Confidence` block and above.

`0x802DCD4C`/`0x802DCD98` are prologue helpers, not functions to translate, and
the report/battery ticks `0x801DE9D0`/`0x801DDF00` already live in
`src/runtime/input.rs`; they are called by address here.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (`801deb70`, `FUN_801deb70`, 2524 bytes).
- The evidence file exists at
  `work/ghidra/main-gb/export/801deb70_FUN_801deb70.c`.
- The row has exactly one `pub unsafe fn fn_801deb70` body in
  `src/runtime/dvd.rs`.
- No address was already in `ledger/functions.csv`; `801deb70` was the bucket's
  only untranslated address before this pass.
- `node tools/check-lane-csv.mjs wave115` reports no problem for
  `ledger/agents/wave115-dvd.csv`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/runtime/dvd.rs`. (The run still fails on three pre-existing
  `src/game/stage.rs` syntax/arity errors from another lane.)
