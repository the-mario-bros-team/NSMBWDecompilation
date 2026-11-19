# Coverage - Wave 207 lane F (`ui::hud`, `0x8003A240..0x8003ACD0`)

## How to read this file

This page records lane F of wave 207. The lane was assigned ten `main.dol`
bodies that the plan filed under `ui::hud`; the code itself belongs to the
world-map enemy base class documented in `game::enemies` (direction byte
`+0x348`, class vtable `+0x60`, controller `+0x394`, ground probe `+0x1EC`,
motion block `+0xDC`). The bodies are appended to `src/ui/hud.rs` as one
section, mirrored one row per body in `ledger/agents/wave207-laneF.csv`, and the
block source is kept at `work/wave207/laneF_block.rs`.

Every `address` is a plain `main.dol` address. The Ghidra exports are
`work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` (lowercase). The byte counts
are the `size` column of `work/ghidra/main-gb/export/functions.tsv`; every one
of them was re-checked against the prologue and the terminal `blr`, and each is
exactly four bytes larger than the true extent (alignment padding), which the
`# Original` block of every function notes.

## Summary

| File | Module | Bodies | Bytes (`functions.tsv`) | Real extents | High | Medium |
|---|---|---:|---:|---:|---:|---:|
| `src/ui/hud.rs` | `ui::hud` | 10 | 1908 | 1868 | 8 | 2 |

Status split: 10 `translated`, no `partial`, no `stub`.

## Bodies

| Address | Rust function | Size (tsv / real) | Role |
|---|---|---:|---|
| `0x8003A240` | [`fn_8003a240`] | 276 / 272 | adopt `source`'s motion record: lane speed + `0.35f` gain, `+0x150` source slot, `+0xE8..+0xF0` |
| `0x8003A360` | [`fn_8003a360`] | 64 / 60 | recompute the lane speed only, with the `0.5f` gain |
| `0x8003A3A0` | [`fn_8003a3a0`] | 112 / 108 | integrate the `+0x102` angle by `|speed| / 2.5 * 3072` |
| `0x8003A410` | [`fn_8003a410`] | 120 / 116 | post sound/effect `0x195` at the actor position for kinds `< 4` |
| `0x8003A490` | [`fn_8003a490`] | 224 / 220 | install the `0x802EE750` hit template into `+0x414` |
| `0x8003A570` | [`fn_8003a570`] | 84 / 80 | run `0x80097C40(this)`, then copy the source's digit byte into `+0x38D` |
| `0x8003A900` | [`fn_8003a900`] | 340 / 336 | rebuild the collision extents `+0x164..+0x170` from the `0x80075FD0` probe |
| `0x8003AB80` | [`fn_8003ab80`] | 120 / 116 | adopt the source's digit byte, zero three offsets, switch state to `0x80353A24` |
| `0x8003AC00` | [`fn_8003ac00`] | 204 / 200 | four mask tests over `+0x34A`/`+0x274` plus the `+0x1EC` probe |
| `0x8003ACD0` | [`fn_8003acd0`] | 364 / 360 | impact responder: effect `Wm_en_hit`, sound `0x17B`, hit record at `+0x414` |

Covered range: the ten bodies span `0x8003A240..0x8003ACD0`; 1868 bytes of code
plus 40 bytes of inter-function alignment. The gaps between them
(`0x8003A5D0`, `0x8003AA60`, `0x8003AE40`, ...) are other functions and are not
part of this lane.

## Why lane F is filed under `ui::hud`

The plan split the ten bodies by address, not by class. The class itself is the
one `game::enemies` documents for the world-map quicksand enemy; its bodies
`0x80AD0xxx..0x80AD1xxx` already call `0x8003A240` from `fn_80ad1920`. Only the
translation target differs, so `game::enemies` stays a one-way reference and no
existing row had to move.

## Constants recovered from the image

Ghidra prints the `r2`-relative float operands as `unaff_r2 + -0xNNNN`, so every
constant was read out of `work/dol/main.elf` at `SDA2_BASE - offset`
(`SDA2_BASE = 0x80433080`):

| Address | Offset | Value | Used by |
|---|---|---|---|
| `0x8042B5A0` | `-0x7AE0` | `{2.5, -2.5, 3.0, -3.0}` | `fn_8003a240`, `fn_8003a360` (lane speeds) |
| `0x8042B5B0` | `-0x7AD0` | `0.0f` | threshold / reset value |
| `0x8042B628` | `-0x7A58` | `0.35f` | `fn_8003a240` gain |
| `0x8042B614` | `-0x7A6C` | `0.5f` | `fn_8003a360` gain, `fn_8003a900` halving |
| `0x8042B630` | `-0x7A50` | `2.5f` | `fn_8003a3a0` divisor |
| `0x8042B62C` | `-0x7A54` | `0.2f` | `fn_8003a240` `+0xEC` |
| `0x8042B5B8` | `-0x7AC8` | `16.0f` | `fn_8003a900` probe span |
| `0x8042B650` | `-0x7A30` | `{-8.0f, 8.0f}` | `fn_8003a900` lane offset |
| `0x8042B600` | `-0x7A80` | `8.0f` | `fn_8003a900` extents |
| `0x8042B5E8` | `-0x7A98` | `5500.0f` | `fn_8003acd0` effect `z` |
| `0x8042C1A0` | - | bytes `{1, -1}` | `fn_8003a900` span sign |
| `0x802EE750` | - | `{0.0f, 3.0f, -4.0f, -0.1875f, 0x80357BF4, -1, -1}` | `fn_8003a490` template |
| `0x802EE770` | - | `+0x10..+0x18` = `0x80357BF4`, `-1`, `-1`; `+0x1D` = `0xFF` | `fn_8003acd0` descriptor |

## Cross-checks

- [`fn_8003a900`] is the `main.dol` twin of `game::enemies::fn_80a26b30`
  (`0x80A26B30`): same `{-8, 8}` lane table, same `{1, -1}` sign bytes, same
  `0x80075FD0(near, far, &fraction, mode, 1, 0)` call and the same four
  midpoint stores. Only the source record differs (`+0x7A0` here, `+0x610`
  there).
- [`fn_8003acd0`] is the twin of [`fn_80033f10`] in this module and of
  `game::enemies::fn_80b13620`: identical hit record, identical `0x800954D0` /
  `0x8006A4B0` / `0x80197F00(0x17B)` sequence. It differs in the `y` term
  (`+0xB0 + +0xD4` instead of plain `+0xB0`) and in the descriptor block
  (`0x802EE770` instead of `0x802EE6A8`).
- [`fn_8003a570`], [`fn_8003a490`] and [`fn_8003ab80`] all read the same
  `+0x60 -> +0x6C` virtual slot whose first byte the module already documents as
  the controller "digit" byte (`fn_80020aa0`).

## Notes on the side byte

[`fn_8003acd0`] derives its side from `fcmpo` + `cror cr0.EQ,cr0.GT,cr0.EQ` +
`mfcr` + `rlwinm` + `cntlzw` + `rlwinm` (rotate right five). The merged bit is
`GT|EQ`, so `side` is `0` exactly when `(+0xAC - +0xB8) >= 0.0f` and `1` for
both the negative and the unordered case. The `fn_80033f10` doc block in this
module records the same instruction sequence with the opposite polarity
(`!((x - +0xB8) < 0.0f)`); the derivation here follows the `cror` field numbers
(`BT=2`, `BA=1`, `BB=2`) and is noted in the body so the next pass can settle it
against a runtime check.

## Deferrals

None. All ten bodies are straight-line and fully decoded; nothing needed an
exact-value guess. The only omitted instructions are the dead stack copies of
`fn_8003a490` (`stb` to `0x25(r1)`) and `fn_8003acd0` (eight re-copies into
`0x28..0x44(r1)`), which no later instruction reads.

## Verification

`cargo check --lib` was run on a scratch copy (`src/`, `Cargo.toml`,
`Cargo.lock`) with this lane's block appended to `src/ui/hud.rs`: exit code `0`,
23 warnings, none of them in the appended region.
