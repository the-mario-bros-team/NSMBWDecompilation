# Coverage - Wave 214 (`game::player`, main.dol part 5, final)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 53 | 9,608 | 36 | 17 |

The last `game::player` bodies, `0x8006C8A0`-`0x8006FEA0`. The module is now
**830 / 830**.

## What landed

- **`Base_c` virtuals and factory calls (lane A).** `0x8006C8A0`, `0x8006C8B0`,
  `0x8006C8F0`, `0x8006C980`, `0x8006C9F0`, `0x8006CA70`, `0x8006CA80`,
  `0x8006CAE0`, `0x8006CBA0`.
- **Velocity/position integrators (lane B).** `0x8006CC00`, `0x8006CCB0`,
  `0x8006CD00`, `0x8006CD40`, `0x8006CD50`, `0x8006CD90`, `0x8006CDE0`,
  `0x8006CE10`, `0x8006CE70`.
- **Effect records and probes (lane C).** `0x8006CED0`, `0x8006CF40`,
  `0x8006CFA0`, `0x8006CFE0`, `0x8006D090`, `0x8006D120`, `0x8006D160`,
  the 576-byte `0x8006D170`, `0x8006D3B0`.
- **Movement/hit drivers (lane D).** `0x8006D440`, the 616-byte `0x8006D460`,
  `0x8006D6D0`, `0x8006D770`, `0x8006D8A0`, `0x8006D9B0`, the 536-byte
  `0x8006DB30`, `0x8006DD50`, the 432-byte `0x8006DE10`.
- **Contact resolvers (lane E).** the 1,232-byte `0x8006DFC0`, `0x8006E490`,
  `0x8006E620`, the 1,668-byte `0x8006E720`, `0x8006EDB0`, `0x8006EF40`,
  `0x8006F0E0`, `0x8006F1D0`, `0x8006F3E0`.
- **Grid/effect bodies (lane F).** `0x8006F600`, `0x8006F750`, the 604-byte
  `0x8006F7E0`, `0x8006FA40`, `0x8006FAA0`, `0x8006FBF0`, `0x8006FCF0`,
  the 648-byte `0x8006FEA0`.

## Corrections and recoveries

- `0x8006C8A0` is exactly 4 bytes: `b 0x8006C570`, confirming the wave-211
  chain `0x8006C8A0 -> 0x8006C570 -> 0x80161DA0`.
- `0x8006CFE0` is **two prologued halves**: its first half ends in
  `b 0x8006D000`, which opens a fresh frame and returns to the original caller.
  A whole-`.text` branch scan shows that `b` is the only reference, so it is one
  entry point and one row.
- `0x8006D460`'s `bc 12,6` (`kind == 3`) test is unreachable behind the
  preceding `bge`; documented, not emitted.
- `0x8006FAA0` stores the literal `0x80` (`li r0,0x80` / `stb r0,0xE0(r30)`),
  where Ghidra renders `e0 & ~(table | 0x80)`. The body follows the bytes.
- `0x8006EF40` dispatches through `0x802DCBFC` with `r12 = 0x80355F14`; only
  `f1/r3/r4` are modelled, hence `medium` and an explicit `// partial:`.
- The exports for `0x8006DFC0`, `0x8006E720`, `0x8006F1D0`, `0x8006F3E0`
  mis-type their entry state because `0x802DCD64/70/74` are the outlined
  `savegpr` stubs, not getters. Real arguments recovered as `(dx, dy, this,
  flags)` / `(this, flags)`.
- `0x8006CC00`'s `GQR`/`ldexpf` noise is the `(s16)->f32` conversion of the
  heading at `+0x108` (`sth` + `psq_l ...,1,3`).
- `0x8006F7E0` reduces x modulo the `f64` `19.0` via the `0x802E8614` fmod
  thunk, then `frsp`.
- `0x8006FEA0` takes only `f1`/`f2`; `f26/f27/f28/f31` are self-loaded
  constants, and it treats x as signed `i16` but y as unsigned.

## Tool defects found by the lanes

- `tools/check-lane-csv.mjs` reads its byte-size column as `len - 2`, which is
  the `true` column of a 5-column `functions.tsv`, so its size check never
  fires. Lane A kept `# Original` sizes in the shape the checker would match.
- The local disassembler's `F63_32` fallback prints `FRA`/`FRB` reversed for
  `fcmpu`; lanes resolved operand order from the encoding.
- The existing `void` declarations of `0x8006C5C0`/`0x8006C610` are wrong: their
  `bl` results gate `0x8006C8F0`/`0x8006C9F0`, so lane A re-expressed the
  sequences inline rather than double-calling.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-lane-csv.mjs wave214`: 53 rows, 53 unique, 36 high + 17 medium.
- Ledger: 19,670 rows (97.13%); `game::player` 830/830 complete.
- `scripts/check.ps1` exit 0.

## Next

`src/engine/dsys.rs` (248), `src/resource/archive.rs` (198),
`src/game/stage.rs` (132) and two `ui::hud` stragglers (`0x80026CA0`,
`0x8002D260`).
