# Wave 218 lane D coverage (`engine::dsys`)

Slice: the twelve `engine::dsys` bodies at `0x800FCDC0`, `0x800FCE60`,
`0x800FCF30`, `0x800FD0A0`, `0x800FD130`, `0x800FD180`, `0x800FD1F0`,
`0x800FD310`, `0x800FD3B0`, `0x800FD590`, `0x800FD720` and `0x800FD750`
(2032 bytes of `functions.tsv` extent). All twelve are translated; nothing was
deferred.

Bodies live in `work/wave218/laneD_block.rs` and are appended to
`src/engine/dsys.rs` by the merge step; ledger rows are in
`ledger/agents/wave218-laneD.csv` (`module = engine::dsys`).

This is the `m`/`t`/`R` resource-name bank (`0x800FD590` + the two leaf
accessors `0x800FD720`/`0x800FD750`), its per-class counting pair
(`0x800FCE60`/`0x800FCF30`), and the small state probes of the text context that
sit above them.

## Translated

| Address | Bytes | Rust body | Summary |
| --- | --- | --- | --- |
| `0x800FCDC0` | 156 | `fn_800fcdc0` | Returns `1` on the frame the text-context state drops to `0`: either the latched previous state was `1`, or the block byte `*(context + 0x395 + (param_1 & 0xFF))` is not `'*'` while `*(context + 0x6C)` is non-zero. Both arms require `0x800FD0A0() == 0`; the value is then read once more and written to the `r13 - 0x551C` latch. |
| `0x800FCE60` | 200 | `fn_800fce60` | Counts live entries of class `param_1`. Class `8` walks eight slots and filters with the `0x800FC470` thunk; every other class walks 42 slots and filters with `0x800FAE40`. Every slot must additionally pass `0x80102750(class, slot, 1)`. |
| `0x800FCF30` | 244 | `fn_800fcf30` | The ordinal twin of [`fn_800fce60`]: counts down `param_2` on every passing slot and returns the slot index whose counter reaches zero. Returns `42` when the walk runs out - also for class `8`, which only walks eight slots. |
| `0x800FD0A0` | 12 | `fn_800fd0a0` | Leaf: byte `+0x24` of `*(r13 - 0x5704)`. |
| `0x800FD130` | 76 | `fn_800fd130` | Selects `scene + *(i32*)(scene + 0x338C) * 0x16518 + 0x3394`, calls `0x800F56B0(table, param_1)` and returns the sign-extended `+3` byte minus `0x2C` (`extsb`, then `addi r3,r3,-44`). |
| `0x800FD180` | 112 | `fn_800fd180` | Leaf nine-arm switch `0..8`, jump table at `0x8031C190` (image words `0x800FD1A0..0x800FD1E0`, verified in `work/dol/main.elf`). Ids `0x286, 0x287, 0x28A, 0x288, 0x289, 0x28B, 0x28C, 0x28D, 0x28D`; anything above `8` returns `-1`. |
| `0x800FD1F0` | 228 | `fn_800fd1f0` | Writes the three-byte tag at `+3/+4/+5` of `0x800E0380(*(r13 - 0x5640), -1)`. Non-zero `0x800FAE80(7, 0x17, *(context + 0x44))` clears all three. Otherwise, when the scene is non-null, the shell byte `r13 - 0x5433` is `7` and `0x801002C0(scene, "F101")` equals `*(context + 0x44)`, it writes `7`, the `r13 - 0x5432` byte and the low byte of `0x801002C0(scene, "W8A0")`. The fallback copies the two shell bytes and the low byte of `*(*(r13 - 0x54E0) + 0x22C)`. |
| `0x800FD310` | 116 | `fn_800fd310` | `1` when `0x800F9840(param_2) == 3`, `0x800FC0A0() != 0`, `0x800FAD60(param_1, param_2) != 0` and `0x800FBFB0() != 0`. The `0x800FAD60` call is evaluated first (its result is live across the state test) and really carries both register arguments. |
| `0x800FD3B0` | 328 | `fn_800fd3b0` | `0x10` and `0x20` both set: the `0x800FB1F0`/`0x800FB240`, `0x800FB150`/`0x800FB240` and `0x800FB1F0`/`0x800FB1A0` groups must all pass. Otherwise `0x800FB1F0` must pass, or a set `0x20` must additionally pass `0x800FB240`. The repeated `||` re-calls are reproduced in order. |
| `0x800FD590` | 388 | `fn_800fd590` | Clears `+4`, then 32 slots at stride `0xE` from `+8` in each of two `0x1C0`-byte rows, then scans the resource list from `0x80168400(param_2)`: `0x8023B8C0` count, `0x8023B800` entry, string at `entry + *(u32*)(entry + 8)`, length `12` or `13` via `0x802DC69C`, prefix `"mtR"`, copied with `0x802E19F8(&bank[count], text, 0xE)` and `+4` incremented. |
| `0x800FD720` | 40 | `fn_800fd720` | Leaf: `param_2 > count - 1` (signed `cmp`) returns `0`; otherwise `param_1 + param_2 * 0xE + 8`. |
| `0x800FD750` | 132 | `fn_800fd750` | Leaf: walks `count` entries at `param_1 + 8`, stride `0xE`, with `0x802E1BD0(entry, key, 0xE)`; the first zero compare returns `1`, an exhausted bank returns `0`. |

## Sizes and extents

Every `# Original` block keeps the canonical `main-gb` `functions.tsv` value and
each was re-checked against the prologue/leaf head and the closing `blr` in
`work/dol/main.elf`:

- `0x800FCDC0`: `stwu r1,-0x20` at `+0`; `bclr 20,0` at `0x800FCE58`; padding at
  `0x800FCE5C`; ends at `0x800FCE60` = 156.
- `0x800FCE60`: `stwu r1,-0x20` at `+0`; `bclr 20,0` at `0x800FCF24`; padding to
  `0x800FCF30` = 200.
- `0x800FCF30`: `stwu r1,-0x20` at `+0`; `bclr 20,0` at `0x800FD020`; ends at
  `0x800FD024` = 244 (three pad words to `0x800FD030`).
- `0x800FD0A0`: leaf `lwz`/`lbz`/`bclr` from `0x800FD0A0`; ends at `0x800FD0AC`
  = 12 (one pad word to `0x800FD0B0`).
- `0x800FD130`: `stwu r1,-0x10` at `+0`; `bclr 20,0` at `0x800FD178`; ends at
  `0x800FD17C` = 76.
- `0x800FD180`: leaf with `bcctr` dispatch; last arm `bclr` at `0x800FD1DC`,
  default `-1` arm at `0x800FD1E8`; ends at `0x800FD1F0` = 112.
- `0x800FD1F0`: `stwu r1,-0x20` at `+0`; `bclr 20,0` at `0x800FD2D0`; ends at
  `0x800FD2D4` = 228 (two pad words to `0x800FD2DC`).
- `0x800FD310`: `stwu r1,-0x10` at `+0`; `bclr 20,0` at `0x800FD380`; ends at
  `0x800FD384` = 116.
- `0x800FD3B0`: `stwu r1,-0x10` at `+0`; `bclr 20,0` at `0x800FD4F4`; ends at
  `0x800FD4F8` = 328.
- `0x800FD720`: leaf `lwz`/`cmp`/`mulli`; first `bclr` at `0x800FD734`, second at
  `0x800FD744`; ends at `0x800FD748` = 40.
- `0x800FD750`: `stwu r1,-0x20` at `+0`; `bclr 20,0` at `0x800FD7D0`; ends at
  `0x800FD7D4` = 132.

### `0x800FD590`: the tsv value covers a tail-called body

This is the one entry whose `functions.tsv` size is **not** the extent of the
prologue-bearing function. The head runs `0x800FD590..0x800FD628` (the clear
loop) and ends at `0x800FD62C` with `b 0x800FD630` - a **tail branch**, so there
is no `blr` at the tsv boundary. `0x800FD630` opens with its own
`stwu r1,-0x20` prologue, saves `r31`/`r30`/`r29`, and runs the resource scan
through `bclr 20,0` at `0x800FD710`; `0x800FD714` is padding and the next
`functions.tsv` entry is `0x800FD720`.

So the entry is 156 bytes of head plus a 232-byte tail-called body, and
`0x800FD590 + 388 = 0x800FD714` is exactly the end of that tail body. The tail
body is included in `fn_800fd590` (Ghidra does the same, because the address is
only reachable through this branch) and the split is recorded in the body's
`# Original` block. `r3`/`r4` arrive unchanged at `0x800FD630`, so the two
parameters of `fn_800fd590` are exactly the two parameters of the tail body.

## New addresses

Only nine new constants were needed; the rest of the module's helpers and the
Wave 217 lane D record/course names are reused (`G_TEXT_CONTEXT`,
`G_SCENE_54F4_D`, `RECORD_STRIDE_D`, `RECORD_ROW_FIELD_D`,
`RECORD_TABLE_BASE_D`, `G_SOURCE_5640_D`, `G_SHELL_BYTE_5433_C`,
`G_SHELL_BYTE_5432_C`, `G_SESSION_54E0_B`, `FN_800E0380`, `FN_80168400`,
`FN_8023B8C0`, `FN_802DC69C`, `FN_802E19F8_C`, `FN_802E1BD0`,
`FN_800F56B0_F`, `FN_800FB150_D`, `FN_800FB1A0_D`, `FN_800FB1F0_D`,
`FN_800FB240_D`, and the bodies `fn_800fad60`, `fn_800fae40`, `fn_800fae80`,
`fn_800f9840`).

| Constant | Address | Note |
| --- | --- | --- |
| `G_PREV_STATE_551C_D` | `0x8042A164` | byte latch of `0x800FCDC0` |
| `G_KEY_6AE0_D` | `0x80428BA0` | ASCII `"F101"` (image bytes observed) |
| `G_KEY_6AD8_D` | `0x80428BA8` | ASCII `"W8A0"` (image bytes observed) |
| `FN_800FC470_D` | `0x800FC470` | `b 0x800FC400` thunk |
| `FN_80102750_D` | `0x80102750` | bit-mask word reader |
| `FN_801002C0_D` | `0x801002C0` | scene-row key lookup (tail to `0x800F53A0`) |
| `FN_800FC0A0_D` | `0x800FC0A0` | course-row live-entry probe |
| `FN_800FBFB0_D` | `0x800FBFB0` | shell-byte state probe |
| `FN_8023B800_D` | `0x8023B800` | resource-list element accessor |

## Verification

The block was appended to a copy of `src/` + `Cargo.toml` + `Cargo.lock` in a
scratch directory and `cargo check --lib` exits 0 with no warning pointing at
the new code.
