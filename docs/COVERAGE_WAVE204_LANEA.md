# Coverage - wave 204, lane A (`ui::hud`, `0x80024150..0x80024F48`)

Ten functions of `src/ui/hud.rs` translated from `sys/main.dol` in wave 204,
lane A. All ten now have a faithful Rust body appended to `src/ui/hud.rs`;
nothing in this lane is deferred.

## Scope

| address | size | name | confidence | role |
|---|---|---|---|---|
| `0x80024150` | 48 | `fn_80024150` | high | forward the sprite tick to vtable slot `+0x320`, return `1` |
| `0x80024190` | 296 | `fn_80024190` | medium | build the sprite transform and bind `+0x544` |
| `0x800242C0` | 120 | `fn_800242c0` | high | bind five `+0x544` animation records |
| `0x80024720` | 412 | `fn_80024720` | medium | hard-hit effect plus `+0x414` colour/hit record |
| `0x80024C20` | 64 | `fn_80024c20` | high | guarded array release, return the base |
| `0x80024C60` | 140 | `fn_80024c60` | high | step and clamp the `+0x102` tilt angle |
| `0x80024DE0` | 148 | `fn_80024de0` | high | landing-smoke effect with the `0xC` selector |
| `0x80024E80` | 132 | `fn_80024e80` | medium | resolve/bind one animation and apply two scales |
| `0x80024F10` | 40 | `fn_80024f10` | high | choose `"walk"` or `"walk_back"` |
| `0x80024F40` | 8 | `fn_80024f40` | high | play the walk animation with the `-0x7D44` scale |

Total: 1,408 bytes, 10/10 translated. (The per-entry size field for
`0x80024E80` is `16`; its disassembled body is 132 bytes, so the lane total uses
the observed spans.)

## What the cluster is

This is the sprite/effect half of the HUD object that the earlier wave-203 run
reached from the other side. The bodies are tied together by three records that
already appear in `src/ui/hud.rs`:

- **`+0x7C`** is the sprite record written by `0x8016ECB0`/`0x8016EDD0`/
  `0x8016EEF0` and bound into `+0x544` by `0x8016A170`/`0x8016A150`/
  `0x8016A1A0` - the same protocol as the existing `fn_8001fd40`.
- **`+0x544`/`+0x584`/`+0x540`** are the model, animation and archive records
  used by the wave-203 `daikonbou` bodies. `fn_80024e80` is the small
  name-driven rebind helper those bodies did not include.
- **`+0x414..+0x432`** is the hit descriptor filled by `fn_80024720`; its four
  leading floats come from the side-selected `0x8042C1B0` word and three
  `.sdata2` constants, and the two trailing bytes are the side and the player's
  virtual `+0x6C` byte.

## Resolved constants

`.sdata2` anchors (`SDA2_BASE = 0x80433080`). Their addresses are derived from
the `r2` displacements in the shipped code; the value roles below are inferred
from use:

| address | offset | use |
|---|---|---|
| `0x8042B300` | `SDA2 - 0x7D80` | neutral x/z (`0.0`) in `fn_80024190` |
| `0x8042B308` | `SDA2 - 0x7D78` | scale for `0x801680E0` in `fn_80024e80` |
| `0x8042B328` | `SDA2 - 0x7D58` | effect z in `fn_80024720`/`fn_80024de0` |
| `0x8042B32C` | `SDA2 - 0x7D54` | all three components of the zero vector |
| `0x8042B33C` | `SDA2 - 0x7D44` | walk scale in `fn_80024f40` |
| `0x8042C2D0` | `SDA2 - 0x6DB0` | fourth hit colour word |
| `0x8042C2D4` | `SDA2 - 0x6DAC` | second hit colour word |
| `0x8042C2D8` | `SDA2 - 0x6DA8` | third hit colour word |

Absolute tables and strings:

- `0x8042C1A4` - `i16` angle limits indexed by `+0x348 * 2` (`fn_80024c60`).
- `SDA2 - 0x7D48` - `i16` angle steps with the same index.
- `0x8042C1B0` - two-word hit-side colour table (`0`/`1`).
- `0x802EE420`, `0x802EE428` - shared words copied into `+0x424`/`+0x42C`.
- `0x803023D0` `"Wm_mr_hardhit"`, `0x803023E0` `"Wm_en_sndlandsmk_s"`,
  `0x803023F4` `"Wm_en_landsmoke_s"`, `0x80302408` `"walk_back"`.
- `SDA_BASE - 0x7E60` - the existing `S_WALK` `"walk"` string.

## Notes on the medium-confidence bodies

1. **`fn_80024190`.** The three angle writes are deliberately interleaved with
   the two `(0, +0xD4, 0)` / `(0, -0xD4, 0)` matrices and `0x801C0540`
   concatenations. The body follows the raw order (second angle, lift matrix,
   first angle, drop matrix, third angle), which is why `# Original` lists it
   step by step. The NW4R helper roles are inferred from their call shapes.
2. **`fn_80024720`.** `+0x414` is filled with `lwzx` then `stfs`, so the
   `0x8042C1B0` entry is copied bit-for-bit into a float field; the Rust uses
   `read_f32`/`write_f32` to preserve that. The `local_1c` `CONCAT13` in the
   Ghidra output is dead: no store reads it, so it is omitted. The original
   `r4` operand is overwritten before use and is named `_arg`.
3. **`fn_80024e80`.** Ghidra prints `0x8023A0B0` as a one-operand call, but the
   raw `r4` still holds the caller's name pointer at `0x80024EAC`; the name is
   therefore the second archive operand. The Ghidra body is also 132 bytes
   (`0x80024E80..0x80024F04`), not the ledger's `16`; the prologue and epilogue
   in the disassembly span the whole range.

## Evidence

- Disassembly: `work/dol/main.elf` via `ppc159.js`, `LEN=80`/`120`/`220`,
  `STOP=0`, for all ten entries plus the interior of `0x80024720` and
  `0x80024E80`.
- Decompilation: `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` (lowercase)
  for all ten addresses.
- `work/ghidra/main/` was not needed as a fallback.

## Verification

`work/wave204/laneA_block.rs` was appended to a temporary copy of `src/`,
`Cargo.toml` and `Cargo.lock` (`%TEMP%\opencode\wave204_lanea`), and
`cargo check --lib` exited `0` with 0 errors. The 23 warnings are the same
pre-existing `menu.rs`/function-pointer-cast warnings seen in wave 203; none
point at the appended lines.
