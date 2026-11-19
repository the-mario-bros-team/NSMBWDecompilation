# Coverage - Wave 216 (`engine::dsys`, main.dol part 2)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/dsys.rs` | `engine::dsys` | 60 | 12,436 | 49 | 11 |

Second 60 `engine::dsys` bodies, `0x800F2DF0`-`0x800F7400`. The module is now
**269 / 397**.

## Lane balancing change

For this wave the 60 addresses were split by **byte weight** with a snake draft
(largest body first, alternating lane order) instead of the previous
address-order split. The address-order split had handed one lane 6,632 bytes,
including the 2,544-byte `0x800F3E80`; the snake split spread 1,816-3,760 bytes
per lane with ten bodies each.

## What landed

- **CSV parser and route tables (lane A).** `0x800F2EB0`, `0x800F33D0`, the
  2,544-byte `0x800F3E80`, `0x800F5A80`, `0x800F5C10`, `0x800F5D40`,
  `0x800F5F10`, `0x800F6940`, `0x800F6AB0`, `0x800F71C0`.
- **Point/route parsing and name ring (lane B).** the 1,104-byte `0x800F39D0`,
  `0x800F5420`, `0x800F5770`, `0x800F59E0`, `0x800F6810`, `0x800F6910`,
  `0x800F6D60`, `0x800F7060`, `0x800F71F0`, `0x800F72F0`.
- **Record clearing and projection (lane C).** `0x800F33C0`, the 996-byte
  `0x800F35E0`, `0x800F5700`, `0x800F5730`, `0x800F5A90`, `0x800F5E60`,
  `0x800F6390`, `0x800F6B50`, `0x800F6C40`, `0x800F7400`.
- **Grid/CSV writers (lane D).** `0x800F2F20`, `0x800F3460`, the 736-byte
  `0x800F4870`, `0x800F5050`, `0x800F56B0`, `0x800F6320`, `0x800F64E0`,
  `0x800F6500`, `0x800F7020`, `0x800F7270`.
- **CSV field readers (lane E).** `0x800F2DF0`, `0x800F32F0`, `0x800F3330`,
  `0x800F3E20`, the 592-byte `0x800F4B50`, the 468-byte `0x800F4DA0`,
  `0x800F4F80`, `0x800F53A0`, `0x800F57C0`, `0x800F71A0`.
- **Lookups and curve solver (lane F).** `0x800F30C0`, `0x800F3190`,
  `0x800F5130`, `0x800F5310`, `0x800F56C0`, `0x800F57A0`, `0x800F5830`,
  `0x800F6110`, `0x800F6CD0`, `0x800F7330`.

## Corrections and recoveries

- **`0x800F3E80` decoded from raw words.** The export hides that there are two
  *independent* flag chains (every `+0x1C8` arm branches into the `+0x1D0`
  chain, so one field can set a bit in each), and it renders the 40 keyword
  compares as `unaff_r13 + -0x6d64` expressions. Reading the image recovered the
  literals: `ura`, `stop`, `link1..5`, `scroll`, `scale`, `dokan`, `switch`,
  `crossroad`, `focus`, `anchor`, `tilt`, `demo1..7`, `camstop`, `noshift`,
  `board`, `demostop`, `scrollY`, `sand`, `ice`, `scrollA`, `kuribo1/2`,
  `Puku1/2`, `Pak1..3`, `Hbros1`, `trap1/2`. `board` is the only five-byte
  prefix compare.
- **Scratch decoder defect fixed.** `ppc159.js` printed logical X-form operands
  swapped (`and`/`andc`/`nor`/`xor`/`xnor`), e.g. `0x7CA30038` rendered as
  `and r5,r3,r0` instead of `and r3,r5,r0`. Lane E caught it from the encoding;
  the decoder now prints destination first.
- `0x800F2F20`: Ghidra feeds the **x** diff into branches the image feeds the
  **z** diff (`fc060040` at `0x800F3050`); transcribed from the ELF, with the
  dead `|dz| < pitch` re-test kept.
- `0x800F6B50` returns a **float in `f1`**, and its arm order is `mode-2<2` ->
  `mode-4<2` -> `mode<=1`, not Ghidra's nested `1 < mode-4`. `0x800F6C40`'s
  value is `f32` (`fcmpo` + `stfs`).
- `0x800F6110` is 520 bytes, not the tsv's 512: its head is a compare/tail-jump
  and each arm is a complete body.
- `0x800F6940` is medium: the divisor at `0x800F697C` is encoded as `fsubs`
  between the `2^52+2^31` magic double and `4503601774854144.0`, so it rounds to
  `0.0f` and the quotient is `+-inf`. Reproduced as encoded and flagged.
- `0x800F64E0` is 24 bytes of code in a 28-byte slot.
- `0x800F4870`'s bare path continuation at `0x800F49E4` is unreachable and is
  reproduced anyway.
- `0x800F5A80` is exactly `lwz` + `b` (no `bclr`), 8 bytes.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-lane-csv.mjs wave216`: 60 rows, 60 unique, 49 high + 11 medium.
- Ledger: 19,790 rows (97.72%); `engine::dsys` 269/397.
- `scripts/check.ps1` exit 0.

## Next

128 `engine::dsys` bodies, then `src/resource/archive.rs` (198),
`src/game/stage.rs` (132) and two `ui::hud` stragglers.
