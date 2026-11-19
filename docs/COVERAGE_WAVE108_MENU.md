# Coverage - Wave 108 (UI: home-button page state machines)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/menu.rs` | `ui::menu` | 6 | 23,136 | 0 | 6 |

## What this pass covers

- **Main page state machine** (`0x801769B0`): the `switch` on the `+0x14` state
  word with states `0..0x13`. It resets the three pane lists through
  `0x801715D0`, advances the four per-player scroll/fade fields, then handles
  the boot page, the first sound setup, the battery dwell timer, the option
  page, the dialog text page, the confirm/dismiss paths and the scroll/finish
  paths.
- **Scrolling-text updater** (`0x80178C50`): the four-player loop over the
  `0x10`-byte records passed by `0x801769B0`; it advances the `+0x210`/`+0x220`
  scroll pairs, starts and clears `0x802F5B18` panes, writes the `N_Rot`,
  `N_SRot` and `N_Trans` transforms and arms the two per-player timers.
- **One-player input handler** (`0x801795B0`): the control-object slide, the
  `+0x84` scroll index, the 0x0B/0x0C/0x0D/0x0E/0x0F/0x10/0x11 sound ids and the
  four widget progress values.
- **Button-release page transitions** (`0x8017A120`, `0x8017AF70`): the two
  `switch(selected - button_count)` handlers with the `0x802F5B18` pair walks.
  The first also carries the cursor/`+0x1A` retrigger counter and the optional
  `param_3` selection update; the second is the dialog variant without sound.
- **Option/dialog machine** (`0x8017BA40`): the entry branch that builds the
  dialog line from the `+0xBC` text table plus the option-list, scroll, cursor
  and page-close arms.

The five-page family shares the `0x802F5B18` `(pane, action)` table. The
repeated 0x25-group scans, the `+0x2F0` pane activations, the `+0x2C0` entry
pane lookup, the layout-name lookup and the `(5, id)` sound call are factored
into local helpers (`find_menu_pair`, `activate_pair`, `clear_pair`,
`activate_slot`, `clear_slot`, `find_entry_pair`, `layout_pane`,
`set_pane_visible`, `menu_sound_event`, `scroll_progress`). Each helper carries
its own `# Original` note and cites the export that contains the literal block.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801769B0` | `fn_801769b0` | 7,420 | medium |
| `0x80178C50` | `fn_80178c50` | 2,388 | medium |
| `0x801795B0` | `fn_801795b0` | 2,212 | medium |
| `0x8017A120` | `fn_8017a120` | 3,660 | medium |
| `0x8017AF70` | `fn_8017af70` | 2,764 | medium |
| `0x8017BA40` | `fn_8017ba40` | 4,692 | medium |

## Deliberate skips

| address | bytes | why |
|---|---:|---|
| `0x801701A0` | 612 | The body is a paired-single particle draw. Ghidra loses the GQR conversions and the float argument lists of `0x8022D3C0` and `0x80231870`, and the two quad ordinates are only reachable from the removed `lfd`/`fadds` sequence. The six `ldexpf` conversion blocks at the entry and exit also have no faithful Rust equivalent in the export. A body would be guesswork, so it is left untranslated. |

`0x801701A0` is not reachable from the six bodies added here; it is an
independent particle/quad draw. The six bodies added here are the last entries
in the `0x80170000` bucket that the Ghidra export reconstructs at the
instruction level.

## Verification

- Every address and byte size above matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in `src/ui/menu.rs`.
- No address was already in `ledger/functions.csv`; all six were absent.
- `cargo check --message-format=short` reports no errors for `src/ui/menu.rs`.
- `node tools/check-lane-csv.mjs wave108` reports no problem for this lane.
