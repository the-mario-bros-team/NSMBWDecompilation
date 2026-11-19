# Wave 207 lane E coverage (`ui::hud`)

Ten bodies, all translated, no deferrals. Range `0x80038360..0x8003A210`
(`main.dol` runtime addresses; 3,100 bytes of original code).

| Address | Size | Body | Confidence |
| --- | --- | --- | --- |
| `0x80038360` | 764 | `fn_80038360` - rebuild the pose matrix (side quarter-turn, two `7`-unit avatars, `+0x7C4` hold height, `+0x7BA`/`+0x7BE` angles, mirrored quarter-turn, second body angle) and bind it plus `+0x464..+0x46C` to the `+0x548` node. | medium |
| `0x800387D0` | 48 | `fn_800387d0` - stop the `+0x548` node through its slot `+0x14`, answer `1`. | high |
| `0x800396A0` | 236 | `fn_800396a0` - copy the partner's digit into `+0x38D`, re-pick `+0x348`, arm state `4` on the digit's `+0x504` slot, notify, publish the `5500`-high aim point. | high |
| `0x800397B0` | 200 | `fn_800397b0` - aim gate: digit set, `+0x1EC` height test, `+0xE8` facing test against the `{1, -1}` table. | medium |
| `0x80039880` | 704 | `fn_80039880` - three traces against the partner, `8`/`10` delta window, then snap `+0x7D4`/`+0xAC` to `14`/`15` units from the partner's open side. | medium |
| `0x80039B40` | 164 | `fn_80039b40` - "grab" transition while the `+0x2DC` gate is clear. | high |
| `0x80039D70` | 220 | `fn_80039d70` - gated variant: place the partner with `(4.5, 0.0)`, arm state `5`; fallback forwards to `0x800A02D0`. | medium |
| `0x80039E50` | 148 | `fn_80039e50` - choose between the gated hold and `0x80064F50`. | medium |
| `0x80039EF0` | 280 | `fn_80039ef0` - box overlap/padding test with the `0x80429E00` bounds record. | medium |
| `0x8003A0C0` | 336 | `fn_8003a0c0` - arm the hold: `+0x458` side copy, aim point, sound `0x17B`, `+0x7C4` flag and speed triple, `0x803539E4` reset/attach. | medium |

## Notes carried into the bodies

- **Band.** This run continues the `0x8003` enemy-actor band that wave 206 lane F
  stopped at `0x80035BA4`. The ten bodies use the field set the previous lanes
  already documented: `+0x60` vtable, `+0x394` owner chain, `+0x7C` pose matrix,
  `+0xAC..+0xB4` position, `+0x348` side byte, `+0x38D` digit, `+0x38F` player
  mask. The cluster additionally owns the `+0x548` model/animation node, the
  `+0x504` two-slot `u16` state array and the `+0x7C4` hold byte.
- **Class identity.** `0x80AD0440` (`d_enemiesNP`) calls `0x80038360` and then
  repeats the same `0x801C0BD0`/`0x8016A170`/`0x8016A150`/`0x8016A1A0` tail on
  `+0x7FC`, and `0x80AD0070` calls `0x800387D0` after its own `0x80353AA4`
  state test. Both are REL actors of the shared `dE_*` family, so the ten bodies
  are main-DOL base-class code shared by those actors; the class name is not
  identified.
- **`0x801C0BD0` argument order.** Everywhere in this band the destination
  matrix is in `r3` and the three coordinates in `f1..f3`; the Ghidra export
  prints the pointer last. All calls are written as
  `w205f_call_f64x3_u32(.., x, y, z, dst)`.
- **Pose block reuse.** `0x80038360` builds five offset matrices in four
  distinct stack blocks (`r1+0x50`, `+0x80`, `+0xB0`, `+0xE0`). Each is
  consumed by `0x801C0540` before the next is built, so the body reuses one
  scratch block; the arithmetic is unchanged.
- **Export gaps.** The export of `0x80039B40` drops its `return 1`, and the
  export of `0x80039EF0` prints both branches of the `s16` negation in
  `0x80038360` identically (`-0x8000 - v`) although the machine code uses
  `-0x8000 - v` on one path and `0x8000 - v` on the other; the two are
  congruent modulo `0x10000`, so the body uses `0u16.wrapping_sub(v)`. All of
  these were re-read from `ppc159.js` and the bodies follow the machine code.
- **Dead flank vector.** `0x80039880` assembles `(base.x + 16*sign,
  base.y - 16, base.z)` at `r1+0x1C` immediately before the `0x800709C0` call,
  which takes only `(mask, 3, 0x819)`. The words are never read again; they are
  reproduced as an observed-but-unused scratch vector.
- **Probe masks.** The first trace of `0x80039880` runs under the *partner's*
  `+0x38F` mask (`lbz r5,0x38f(r31)` with `r31 = param_2`); the second and
  third use the actor's own byte. The export keeps this distinction, the prose
  did not at first, and the body now follows `r31`/`r30`.
- **Unknown constants.** `0x80352DB4`, `0x80353A64` (both already in this
  module) and the new `0x803539E4` are `.bss` descriptors of the same family
  used by the `+0x14` owner test; their type and contents are not identified,
  and `0x803539E4` is kept as its own constant rather than merged.
- **Sizes.** Sizes are the `functions.tsv` values. `0x800396A0` (236),
  `0x800397B0` (200), `0x80039B40` (164), `0x80039D70` (220) and `0x80039E50`
  (148) are followed by four to twelve bytes of alignment before the next
  entry; their `blr` positions confirm the inventory sizes rather than the
  padded extents.

## Scratch verification

The block was appended to a throw-away copy of `src/ui/hud.rs` at
`%TEMP%\opencode\w207laneE` (with `Cargo.toml`, `Cargo.lock` and `src/`
copied, no other edits). `cargo check --lib` finishes with exit code `0` and
the crate's pre-existing 23 warnings; the appended block adds none and no
warning points at `src/ui/hud.rs`. A deliberate type-error probe appended after
the block failed at `src/ui/hud.rs:26542`, confirming the block really is
compiled.

The block reuses `G_SDA_5880_E`, `G_SDA_5924_F`, `TABLE_SIDE_8042C1B0_D`,
`FN_80070CB0_B`, `DAT_80352DB4`, `DAT_80353A64`, `FN_800954D0_E`,
`FN_80197F00_F`, `FN_8006A4B0`, `FN_80064AF0`, `FN_801C0BD0`, `FN_801C0540`,
`FN_8016EDD0`, `FN_8016ECB0`, `FN_8016EEF0`, `FN_8016A170`, `FN_8016A150`,
`FN_8016A1A0`, `w205f_call_f64x3_u32`, `w205f_call_f64x2_u32x4` and
`call_2f_2u` from the earlier lanes and defines only new names (all `_E`
suffixed) for the rest.
