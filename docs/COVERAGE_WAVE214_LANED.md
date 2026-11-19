# Wave 214 lane D coverage (`game::player`)

Slice: the nine `game::player` bodies at `0x8006D440`, `0x8006D460`,
`0x8006D6D0`, `0x8006D770`, `0x8006D8A0`, `0x8006D9B0`, `0x8006DB30`,
`0x8006DD50` and `0x8006DE10`. All nine are translated; nothing was deferred.

Bodies live in `work/wave214/laneD_block.rs` and are appended to
`src/game/player.rs` by the merge step; ledger rows are in
`ledger/agents/wave214-laneD.csv` (`module = game::player`).

This is the tile-step / contact half of the stage walker that
[`crate::game::stage::fn_80070130`] (`0x80070130`) drives, plus the
per-frame animation-node reset (`0x8006DB30`) at the end of the same run.

## Translated

| Address | Bytes | Rust body | Summary |
| --- | --- | --- | --- |
| `0x8006D440` | 20 | `fn_8006d440` | `node[+0xE1] = 0xFF`, `node[+0xE0] = 0`: reset the "highest accepted kind" byte and the per-kind hit mask. |
| `0x8006D460` | 616 | `fn_8006d460` | Closes one step for `index`. Probes `(x, y)` through `0x80070BA0`/`0x80070BF0`, gates on mask bit `0x1000`, then accepts by index: `0` needs `0.0 > node[+0x2C]`, `1` needs `node[+0x2C] < 0.0`, `2` is unconditional, `4..=6` run a swept accept on `edge = 8.0 + (i16)((i32)x & 0xFFF0)` with `span` from `fmod(-y, 16.0)`, and `kind >= 7` is unconditional. On a hit it sets bit `index` of `+0xE0` and raises `+0xE1`; on a miss it ORs `0x80` into `+0xE0` when `0x802EF5E0[index]` intersects the mask. |
| `0x8006D6D0` | 152 | `fn_8006d6d0` | Thin forwarder: reads the tag from `0x80070530(y, node)`, the linked byte from `0x80070C80(node)`, the layer from `node[+0xEC]`, and calls `fn_8006d770`. |
| `0x8006D770` | 300 | `fn_8006d770` | Builds the 0x14-byte effect-placement record for `0x80088970`: gate `flags & 0x4000`, reject a zero mask and the `0x0002`/`0x2000` bits, derive the selector from `0x0010 -> 1`, `0x0004 -> 0`, `0x0008 -> 2`, then store `{ (i32)x & 0xFFF0, -((i32)(-y) & 0xFFF0), kind, selector, mode, direction, linked }` and pass it to `0x80088970(manager, record)`. |
| `0x8006D8A0` | 272 | `fn_8006d8a0` | Builds the 0x10-byte tile record for `0x800883E0` after gating on mask bit `0x0004`, `(mode != 2) => !(mask & 0x2000)` and `flags & 0x0010_0000`; stores `{ x, y, kind, mode, 0, tag, linked }`, discards the consumer's result and returns `1`. |
| `0x8006D9B0` | 372 | `fn_8006d9b0` | Gated on mask bit `0x0010`. With `flags & 0x0080_0000` it writes `placed = 0`, runs `0x800883E0`, and returns `0x802EF5F0[mode & 3]` on success or `0x802EF600[mode & 3]` on failure; with that bit clear it requires `flags & 0x0040_0000`, writes `placed = 1` and always returns `0x802EF5F0[mode & 3]`. |
| `0x8006DB30` | 536 | `fn_8006db30` | The animation-node reset. Clears `+0x90`/`+0x94`/`+0xA0`/`+0xDC`/bits `0x04` of `+0xBC`/`+0xE0`, saves `+0xA4` into `+0xA5` and sets `+0xA4 = 0x10`, calls `0x8006E720(node, *node[+8])`, and when `flags & 0x6000 == 0` sets `+0xBD`/`+0xBE`/`+0xE2` and runs `fn_8006dd50` (movement bit `0` clear) or `fn_8006de10` (bit set). It then merges the `0x80355F08` member's result into the low 13 bits, updates `+0xD8` (blend weight) through `0x8015F470`, merges `flags & 0x3FFE000` into `+0x88`, sets `+0xE3`, zeroes `+0x20`/`+0x24`/`+0x28` and runs the `0x8030F288 + 12 * node[+0xA4]` member. |
| `0x8006DD50` | 188 | `fn_8006dd50` | One-point swept record: `x = (i32)record[+4] / 4096.0`, `y = (i32)record[+8] / 4096.0`; runs `0x8006DFC0`, resets the two `0xFFFF` latches at `0x80429DAA`/`0x80429DA8`, runs `0x8006E490`, and returns the two results OR-ed with `0x0080_0000` cleared from the first. |
| `0x8006DE10` | 432 | `fn_8006de10` | Segment sweep: orders `record[+4]`/`[+8]` with a signed compare, walks the span in `0.125` steps through `0x8006D120`, runs `0x8006DFC0` at each step (clearing `0x0100_0000` if any step lacked it), resets the latches, and — when the total carries `0x0080_0000` — clears that bit, widens the band by `1.0` on both sides and sweeps again through `0x8006E490`. |

## Sizes and extents

Every `# Original` block keeps the canonical `main-gb` `functions.tsv` value,
and each one was re-checked against the prologue and the closing `blr`:

- `0x8006D440`: `bclr 20,0` at `0x8006D450`, next entry `0x8006D460`; 20 bytes.
- `0x8006D460`: `bclr 20,0` at `0x8006D6C4`, next entry `0x8006D6D0`;
  616 bytes (the `blr` ends exactly at `0x8006D6C8`).
- `0x8006D6D0`: `bclr 20,0` at `0x8006D764`; 152 bytes.
- `0x8006D770`: `bclr 20,0` at `0x8006D898`; 300 bytes.
- `0x8006D8A0`: `bclr 20,0` at `0x8006D9AC`; 272 bytes.
- `0x8006D9B0`: `bclr 20,0` at `0x8006DB20`; 372 bytes.
- `0x8006DB30`: `bclr 20,0` at `0x8006DD44`; 536 bytes.
- `0x8006DD50`: `bclr 20,0` at `0x8006DE08`; 188 bytes.
- `0x8006DE10`: `bclr 20,0` at `0x8006DFBC`; 432 bytes.

None of the nine table values is a prologue artefact: no body is 16 or 20 bytes
except `0x8006D440`, and that one really is five instructions.

## Evidence notes

- `work/ghidra/main-gb/export/8006d460_FUN_8006d460.c` and
  `8006de10_FUN_8006de10.c` carry `unaff_GQR0`/`ldexpf` blocks and
  "removing unreachable block" warnings; those are the GQR/`psq_st` spills and
  dead code, not logic. Both bodies were re-derived from
  `work/dol/main.elf` (`LEN`/`STOP` disassembly), including every `rlwinm` mask.
- `0x8006D460`, `0x8006D770`, `0x8006D8A0` and `0x8006D9B0` begin with
  `bl 0x802DCD74` (and end with `bl 0x802DCDC0`), the outlined
  `__savegprlr`/`__restgprlr` family; the spills are omitted and the incoming
  argument registers are read directly, as the module header requires.
- The PowerPC `extsb rA,rS` field order (destination in bits 11-15) is what
  makes `0x8006D734` (`7fe60774`) an `extsb r6,r31` and not the reverse; that is
  why the tag reaches `fn_8006d770` as a sign-extended byte.
- `0x8006DB30` reaches both member functions through `0x802DCBFC`. Its body is
  `lwz r0,0(r12)` / `lwz r11,4(r12)` / `lwz r12,8(r12)` / `add r3,r3,r0` /
  `lwzx r12,r3,r12` / `lwzx r12,r12,r11` / `mtctr` / `bctr`; lane-local
  `w214d_member_target_D` reproduces that resolution so both dispatches stay
  faithful instead of dropping `r12`. `0x8030F288` was read from the image
  (`{0,-1,0x8006EE90}` x3, then `0x8006EEA0`, `0x8006EEE0`, `0x8006EF10`);
  `0x80355F08` lives in BSS, so only its address is literal (`// unknown:`).
- The `r2` constants were re-read from `work/dol/main.elf`: `0x8042BC40 = 0.0`,
  `0x8042BC44 = 8.0`, `0x8042BC48 = 0x4330000080000000`, `0x8042BC50 = 16.0`
  (double), `0x8042BC58 = 16.0`, `0x8042BC5C = 0.25`, `0x8042BC60 = 2.0`,
  `0x8042BC64 = 0.125`, `0x8042BC68 = 1.0`, `0x8042BC6C = 0.03`,
  `0x8042BC70 = 0.01`, `0x8042BC74 = 4096.0`. The hit tables are
  `0x802EF5E0 = {0x819, 0x819, 0x8899, 0x4899}`,
  `0x802EF5F0 = {0x200, 0x200, 0x80000000, 0x800000}`,
  `0x802EF600 = {0x100, 0x100, 0x40000000, 0x400000}`, and the halfword at
  `0x802EF55C` is `0`.

## Callees left untranslated (called by address)

`0x8006E720` (animation-state update), `0x8006DFC0` / `0x8006E490` (the two
segment walkers), `0x8006D120` (cursor step) and `0x802E8614` (`fmod` thunk).
`0x80070BA0`, `0x80070BF0`, `0x80070530`, `0x80070C80`, `0x80088970` and
`0x800883E0` are already translated in [`crate::game::stage`]; the two record
consumers are still called by address because `stage.rs` types them differently
(`fn_800883e0` is `void` there, while the original leaves a boolean in `r3`).

## Reuse

The lane reuses `platform::*`, the module's `read_f32` / `write_f32` /
`read_i8` / `call_move_f32`, and the existing `W106_MEMBER_CALL_802DCBFC`
observation. New helpers carry the `_D` suffix (`read_f64_D`, `w214d_probe_D`,
`w214d_fmod_D`, `w214d_tag_D`, `w214d_linked_byte_D`, `w214d_walk_D`,
`w214d_cursor_step_D`, `w214d_node_flags_D`, `w214d_member_target_D`,
`w214d_member_call_D`, `w214d_member_call_f64_D`), and new constants carry
`W214D_` or the `_D` suffix. No existing const or helper was redefined.

## Verification

- `functions.tsv` sizes: all nine match the re-measured extents (table above).
- Scratch copy: `src/`, `Cargo.toml` and `Cargo.lock` were copied to a temporary
  directory, `work/wave214/laneD_block.rs` was appended to its
  `src/game/player.rs`, and `cargo check --lib` exited `0`. A deliberate
  type error inserted at the end of that copy was reported at the appended
  region, which confirms the block really is compiled; the copy was then
  restored and re-checked to `0`. No warning mentions `player.rs`.
- Files written: `work/wave214/laneD_block.rs`,
  `ledger/agents/wave214-laneD.csv`, this document. `src/game/player.rs` was
  not modified.
