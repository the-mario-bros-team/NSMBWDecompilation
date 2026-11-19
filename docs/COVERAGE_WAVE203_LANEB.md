# Coverage - Wave 203, lane B (`ui::hud`, `0x800208C0..0x80020EA0`)

## How to read this file

This page records the Wave 203 lane B pass over `src/ui/hud.rs`: ten bodies of
`main.dol` in the single run `0x800208C0..0x80020EA0` (1,244 bytes total).
The block is staged in `work/wave203/laneB_block.rs` for the merge pass; it is
appended verbatim at the end of `src/ui/hud.rs`. One row per body is mirrored in
`ledger/agents/wave203-laneb.csv`.

Every address is a **runtime address** of `sys/main.dol`, the same address the
Ghidra exports in `work/ghidra/main-gb/export/` use, and each function doc
carries `# Original` (Ghidra name, address, byte count, evidence path) and
`# Confidence` plus an argument/return/safety block.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/hud.rs` | `ui::hud` | 10 | 1,244 | 10 | 0 |

All ten rows are `translated`; none is `partial` or `stub`. Covered range:
`0x800208C0..0x80020EA0`.

Raw words were decoded with `node $env:TEMP\opencode\ppc159.js work/dol/main.elf
<addr>`, and each body was followed from its entry to its terminal `blr`/tail
call. Byte counts are the sizes of the corresponding Ghidra exports:
12, 12, 12, 12, 12, 396, 308, 364, 48, 48.

## What this pass covers

- **Five `0x80377A04` accessors, `0x800208C0..0x80020900`.** Each is 12 bytes,
  `3C608037 38637A04 4E800020` = `lis r3,0x8037` / `addi r3,r3,0x7A04` / `blr`,
  so each returns the constant `0x80377A04` and nothing else. They are the
  continuation of the accessor runs already translated at
  `0x8001CBB0..0x8001DA70` and `0x8001DA80..0x8001DAC0`; the existing
  `DAT_80377A04` constant in `src/ui/hud.rs` is reused, not redefined.

- **`0x80020910` - shared child notification.** The body is the common tail of
  the controller-record callbacks. It dispatches on the child's state byte
  `+0x38C`:
  - state `3`: gated on `0x800979B0(child) == 0`, manager `+0x689 == 0` and
    manager `+0x67F != 0`. It writes the manager's `+0x6C` vtable digit to
    `child + 0x38D`, arms `child + 0x120` bit `0x4` once (incrementing the
    child counter `+0x134`, clamped to `8`, and playing the slot through
    `0x8009F770(child+0x510, child, count, digit)`), sets `+0x366`, stores the
    `0x80064A30` answer at `+0x458` from the two `fadds` sums of `+0xAC`/`+0xD0`,
    and sets manager `+0x667` for a `0x3C` child whose animation `+0xDCA` is
    `0xE`, `0xB` or `1`.
  - state `0x19B`: same three gates, then only the digit (`+0x38D`) and
    `+0x366` are written.
- **`0x80020AA0` - button gate.** Runs for child states `1`/`2`; requires bit
  `0x8` or `0x20` in the manager input word `+0x60C` and answers `0` otherwise
  (and for a `0xFF` digit). Bit `0x8` also stores `1` at `+0x64C`; a set
  `+0x660` with `+0x689 == 1` raises mask `+0x68A` bit `0x2`. The digit is
  re-resolved through the child's `+0xE0` slot, `0x8005F900`, `0x8013AAA0()` and
  that controller's `+0x6C` slot, then stored in `this + 0x66C + digit` and
  `+0x38D`, with `+0x680 = 1`.
- **`0x80020BF0` - controller-side gate.** Same state guard, an `0x1C`/`0x1F`
  input probe for mask `+0x68A` bit `0x1`, the digit records at
  `this + 0x668 + digit` / `this + 0x685 + digit`, the `+0x1090` idle test
  (modes `0` and `3`), the `0x1F`/`0x1E`/`0x1C` probes that write `+0x680 = 2`,
  and a final `0x80020910(this, child)` tail call.
- **`0x80020E70` / `0x80020EA0` - twin wrappers.** Byte-identical 48-byte
  bodies: forward the two incoming operands to `0x80020AA0` and, on a non-null
  answer, store `1` at `manager + 0x67F`.

## Field and callee notes

| item | meaning | basis |
|---|---|---|
| `this` (manager) | digits `+0x66C`, records `+0x668`/`+0x685`, input `+0x60C`, count `+0x660`, flags `+0x667`/`+0x67F`/`+0x680`/`+0x689`/`+0x68A`/`+0x68B` | all offsets literal; roles named from use |
| `child` (record) | state `+0x38C`, digit `+0x38D`, flags `+0x120`, counter `+0x134`, id `+8`, animation `+0xDCA`, `+0x366`, key `+0x458`, mode `+0x1090`, slot player `+0x510` | all offsets literal |
| `+0x60` vtable | slot `+0x6C` = current digit, slot `+0xE0` = owning object | read through `vtable_method` |
| `0x800979B0` | player-state query, `0` while idle | guard shape is literal |
| `0x8009F770` | four-operand slot player `(record, player, index, digit)` | argument registers literal |
| `0x80064A30` | two-`f32` comparison answering one byte | Ghidra types both operands `double`; the wrapper passes them in `f1`/`f2` and the two integer registers are unused |
| `0x80056CF0` | `(record, input id)` test | ids `0x1C`, `0x1E`, `0x1F` literal |
| `0x8013AAA0` | current-controller query, no operands | `bl` with no operand setup |

## Deferrals

None. All ten addresses in the assigned run are translated; there are no
`partial` or `stub` rows and no address was skipped. `crate::game::enemies`
already lists `0x80020910`, `0x80020BF0` and `0x80020E70` as raw vtable-slot
words (`W110_FN_80020910`, `EN58_FN_80020BF0`, `EN58_FN_80020E70`); those
references stay one-way and are not changed by this pass.

## Verification

The block was appended to a temporary copy of the crate
(`%TEMP%\opencode\w203b\crate`, `work/` and `target/` excluded) and
`cargo check --lib` finished with **exit code 0** and no diagnostic naming
`src/ui/hud.rs`.

## Scratch result

- Raw-word dump: `ppc159.js` offsets `0x800208B0` (confirms the accessor word
  pattern is identical to `0x80020850`/`0x800208B0`), `0x80020910`, `0x80020A4C`,
  `0x80020AA0`, `0x80020BF0`, `0x80020D2C`, `0x80020E70`.
- Temp crate: `%TEMP%\opencode\w203b\crate`; `cargo check --lib` → exit 0,
  23 pre-existing warnings, none from the appended block.
