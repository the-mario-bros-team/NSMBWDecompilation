# Audit - Wave 244 (batch 16, scale-table bug)

## Real bug fixed: the facing-scale table address

`C233_TABLE_8043B840` and `D242_SCALE_TABLE_8043B840` both pointed at
`0x8043B840`, which is **outside every loadable segment** (the last one ends at
`0x8042FBE0`). The real table is at `0x8042B840`:

| Address | Word |
|---|---|
| `0x8042B840` | `0x3F800000` (`1.0f`) |
| `0x8042B844` | `0xBF800000` (`-1.0f`) |
| `0x8043B840` | unmapped |

Both are the same mis-decode class as wave 236: an `addi -0x47C0` immediate read
as unsigned (`0x80430000 + 0xB840`) instead of sign-extended
(`0x80430000 - 0x47C0 = 0x8042B840`).

Wave 242's slot 222/223 bodies and wave 233's slot 33 body were reading the
wrong address. Both constants now point at `0x8042B840`, are renamed to
`…_8042B840`, and the six prose references are corrected.

## Bodies translated (54)

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| B | `WM_PLAYER_VTABLE` 219-230 | 12 | 1,948 |
| C | `dEn_c` 153-162 | 10 | 876 |
| D | WorldMap vtable 224-239 | 10 | 1,156 |
| E | player-class vtable 221-236 | 12 | 228 |
| F | `0x803502F8` band 0x80350420+ | 10 | 268 |

## Findings

- `WM_PLAYER_VTABLE` slots 219-230 are the movement gate chain that posts
  `AloneWait`/`Fall`/`Crouch`/`Slip`; those state records match the blocks
  labelled in `engine/display.rs`.
- `0x8043B840` in slot 221 was the same sign-extension error; lane B reused the
  existing `WM_FACING_SCALE_TABLE` instead of adding a second wrong constant,
  which is what led lane D to spot the pre-existing pairs.
- Lane F completed the 11-arm preset jump table at `0x80350418` (cases 2-10)
  and the 148-byte draw-record scaler left open by wave 222.
- Lane E pinned `0x803543DC` to `daYoshi_c::StateID_Slip`.
- Placement note: `dEn_c` slots 158-162 live in the `0x800AExxx` cluster that
  `docs/UNTABLED_POINTER_TABLES.md` attributes to `game::stage`, but were placed
  in `enemies.rs`. One of the two should change.
- `dEn_c` slot 156 reuses wave-243's `c243_fill_sound_table` /
  `c243_class_index` helpers.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations.
- Scale-table fix re-verified against segment bounds and both page contents.
