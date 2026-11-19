# Coverage - wave 3 UI / NW4R `lyt` second pass

## How to read this file

This document covers the second translation pass over the UI band
`0x80140000..0x80180000` of `sys/main.dol`. It follows the semantic modules
created by earlier waves; it does **not** introduce a new staging range.

Read it in this order:

1. **Scope** - which destination modules received code and how much.
2. **Function table** - one row per translated function with address, Rust
   name, plain-English role, status, confidence and evidence file.
3. **What each module now covers** - the object model and entry points.
4. **Unknowns and limits** - what is inferred rather than observed.
5. **Verification** - the two checks that were run.

Status and confidence follow `docs/READING_GUIDE.md`:

| status | meaning |
|---|---|
| `translated` | full control flow, every call accounted for |
| `partial` | flow and major effects present; one operand or rendering path is not literal |
| `stub` | address/role only |

All addresses are virtual (image base `0x80004000`), not file offsets.
Evidence paths are relative to the repository root and point at
`work/ghidra/main-gb/export/<addr>_<name>.c`.

## Scope

| module | file | functions added | address span |
|---|---|---:|---|
| `ui::lyt` | `src/ui/lyt.rs` | 8 | `0x8016C730..0x8016EF50` |
| `ui::hud` | `src/ui/hud.rs` | 5 | `0x80176410..0x801788C0` |
| `ui::menu` | `src/ui/menu.rs` | 11 | `0x80175D70..0x8017E7D0` |
| **total** | | **24** | |

The pass added 24 functions: 23 `translated` and 1 `partial`. Every body keeps
its `# Original` block, cites the Ghidra export, and uses `crate::platform` for
all target-memory access and indirect calls.

## Function table

### `ui::lyt` - layout lifecycle and rotation

| address | Rust name | role | status | conf. | evidence |
|---|---|---|---|---|---|
| `0x8016C960` | `layout_init` | clear, name, tag and notify a layout | translated | medium | `8016c960_FUN_8016c960.c` |
| `0x8016CB60` | `fn_8016cb60` | create a child through virtual `+0x9C/+0x8C/+0x94` | translated | medium | `8016cb60_FUN_8016cb60.c` |
| `0x8016C730` | `fn_8016c730` | tick every registered layout, drop expired ones | translated | high | `8016c730_FUN_8016c730.c` |
| `0x8016D280` | `fn_8016d280` | create a child after building a 56-byte transform | translated | medium | `8016d280_FUN_8016d280.c` |
| `0x8016D4B0` | `layout_unregister` | unlink a layout from the global registry | translated | high | `8016d4b0_FUN_8016d4b0.c` |
| `0x8016D590` | `fn_8016d590` | initialise through `0x8016CEA0`, set registered/enabled | translated | medium | `8016d590_FUN_8016d590.c` |
| `0x8016ECB0` | `fn_8016ecb0` | apply one `s16` rotation to a transform | translated | high | `8016ecb0_FUN_8016ecb0.c` |
| `0x8016EF50` | `fn_8016ef50` | apply three `s16` rotations in observed order | translated | high | `8016ef50_FUN_8016ef50.c` |

### `ui::hud` - battery and controller widgets

| address | Rust name | role | status | conf. | evidence |
|---|---|---|---|---|---|
| `0x80176410` | `fn_80176410` | map controller battery level to the ten-segment gauge | partial | medium | `80176410_FUN_80176410.c` |
| `0x80176610` | `fn_80176610` | initialise the controller-information frame | translated | high | `80176610_FUN_80176610.c` |
| `0x80176740` | `fn_80176740` | show `N_Trans`/battery panes for four player slots | translated | medium | `80176740_FUN_80176740.c` |
| `0x801786B0` | `fn_801786b0` | show the four battery-power panes of one player | translated | medium | `801786b0_FUN_801786b0.c` |
| `0x801788C0` | `fn_801788c0` | draw the battery gauge quad with computed alpha | translated | medium | `801788c0_FUN_801788c0.c` |

### `ui::menu` - dialog tables and button pages

| address | Rust name | role | status | conf. | evidence |
|---|---|---|---|---|---|
| `0x80175D70` | `dialog_parse_entry` | split `name,name,0/1,...` and record flags | translated | high | `80175d70_FUN_80175d70.c` |
| `0x80175EE0` | `dialog_split_quoted` | record pointers to quoted UTF-16 fields | translated | high | `80175ee0_FUN_80175ee0.c` |
| `0x80179E60` | `menu_refresh_buttons` | show/hide the ten `B_btn_00` panes per page | translated | high | `80179e60_FUN_80179e60.c` |
| `0x8017CFE0` | `menu_set_button_layout` | compact/full button-bar mode | translated | medium | `8017cfe0_FUN_8017cfe0.c` |
| `0x8017D3E0` | `menu_reset_player_animations` | reset the four player widgets | translated | high | `8017d3e0_FUN_8017d3e0.c` |
| `0x8017D430` | `dialog_keyframe_lookup_u16` | binary-search a `(time, id)` table | translated | high | `8017d430_FUN_8017d430.c` |
| `0x8017E210` | `dialog_node_construct` | construct a node with an embedded list head | translated | medium | `8017e210_FUN_8017e210.c` |
| `0x8017E500` | `menu_name_eq16` | 16-byte name-prefix test | translated | high | `8017e500_FUN_8017e500.c` |
| `0x8017E530` | `menu_name_eq20` | 20-byte name-prefix test | translated | high | `8017e530_FUN_8017e530.c` |
| `0x8017E6D0` | `dialog_init_vertex_rows` | fill new 32-byte vertex rows | translated | high | `8017e6d0_FUN_8017e6d0.c` |
| `0x8017E7D0` | `dialog_copy_vertex_rows` | copy 32-byte vertex rows | translated | high | `8017e7d0_FUN_8017e7d0.c` |

## What each module now covers

### `ui::lyt`

The layout registry documented in the module header is now complete for the
create/destroy path:

* `layout_init` (`0x8016C960`) -> `fn_8016d590` (`0x8016D590`) ->
  `layout_register` (already present, `0x8016D180`);
* `fn_8016cb60` / `fn_8016d280` add a child from a resource, optionally through
  the `0x8016C9E0` transform helper;
* `layout_unregister` (`0x8016D4B0`) repairs both links and decrements the
  count; `fn_8016c730` walks the whole registry and drops finished layouts;
* `fn_8016ecb0` / `fn_8016ef50` are the rotation helpers that
  `layout_set_transform` calls; the slot order (third, first, second) is
  preserved.

The object fields used are `+0x04` name, `+0x24` id, `+0x114`/`+0x118` links,
`+0x11C` registered and `+0x11D` enabled. They match the registry code that
already existed in the file.

### `ui::hud`

This is the battery/controller indicator inside the home-button overlay, not
the `batteryInfo` warning scene. It uses the menu object:

| field | meaning |
|---|---|
| `+0x04` | resource; `+0x14` optional callback |
| `+0x238` | primary layout owner (`+0x10` is the layout) |
| `+0x23C..+0x248` | one layout owner per player |
| `+0x2AC + player*4` | per-player battery widget |
| `+0x2F0 + index*4` | resolved pane array |
| `+0x1C0 + player*0x18` | segment count |

Battery segments are shown while the segment index is below that count
(`fn_801786b0`), and the gauge draw computes `alpha = used * 0xFF / total`
(`fn_801788c0`). `fn_80176740` walks four player words and hides `N_Trans` for
disconnected slots.

### `ui::menu`

The dialog row parser (`dialog_parse_entry`, `dialog_split_quoted`) fills the
two name pointers and the flag array that the existing `menu_*` helpers read.
`menu_refresh_buttons` implements the three page masks for `B_btn_00`
(`0,1,9` / `1..6,9` / `0..6,9`). The vertex helpers (`dialog_init_vertex_rows`,
`dialog_copy_vertex_rows`) fill and copy the 32-byte rows drawn by the dialog
quad; `dialog_keyframe_lookup_u16` is the keyframe search used by the dialog
animation.

## Unknowns and limits

1. **`fn_80176410` is `partial`.** The function is written in paired-single /
   GQR code; Ghidra renders it with `ldexpf` and `unaff_GQR0` noise. The
   scalar control flow (battery byte -> gauge value -> four widgets -> ten
   segments) is translated, but the exact paired-single rounding is not
   literal. All float constants are read from `.sdata2` at run time rather
   than baked in.
2. **Virtual argument lists.** Ghidra prints several virtual calls with an
   empty parameter list (`(**...)( )`). Those calls are passed `this`
   explicitly and marked `// inferred:`; the slots themselves are exact
   (`+0x0C`, `+0x10`, `+0x28`, `+0x8C`, `+0x94`, `+0x9C`, `+0x3C`, `+0x4C`,
   `+0x50`).
3. **`0x8016D280` component order.** The original calls `0x8016C9E0` with a
   56-byte scratch matrix as the fourth argument. The Rust translation calls
   the address directly to keep that shape; which component is position,
   rotation or scale is `// unknown:`.
4. **Rotation axes.** `fn_8016ecb0`/`fn_8016ef50` preserve the observed call
   order (third, first, second) but the axis names are not recovered.
5. **`fn_80176740` second argument.** Ghidra prints no parameter list; the
   pointer to the four player words is reconstructed from the
   `__savegprlr` pair (`r3` = `this`, `r4` = array). The loop body and all
   offsets are exact.
6. **`0x801788C0` GX helpers.** `0x801C8430` and `0x801C5570` are described as
   a colour-register update and an alpha-compare setup from their arguments;
   the exact GX entry points are not proven. The final `0xCC008000` store is
   observed.
7. **Division guards.** `0x801788C0` divides `used * 0xFF` by `total`; the
   original does not guard zero. The Rust code returns `0` for a zero divisor
   instead of trapping on the host, marked `// unknown:` in the body.
8. **Allocator.** `dialog_parse_entry` calls `0x80174DF0` for the two name
   strings; the allocator body is outside this lane.

None of these limits changes the observed control flow or the number of calls.

## Verification

```text
cargo check          # exits 0
node tools/doc-lint.mjs src
# {"ok": true, "files": 68, "violations": 0, "byRule": {}}
```

Only `src/ui/lyt.rs`, `src/ui/hud.rs`, `src/ui/menu.rs`,
`docs/COVERAGE_WAVE3_UI.md` and `ledger/agents/wave3-ui.csv` were written.
`src/lib.rs`, `src/ui/mod.rs`, `Cargo.toml`, `src/platform.rs` and every other
lane were left untouched.
