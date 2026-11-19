# Coverage - Wave 55 (WorldMap assigned `0x8010` state controller)

## How to read this file

Each row is one original `main.dol` function, reconstructed as a
`pub unsafe fn fn_<address>` body in `src/game/worldmap.rs`. The *address* is the
PowerPC virtual address from `work/ghidra/main-gb/export/functions.tsv`; the
*bytes* column is the exact size from that file; *confidence* matches the
`# Confidence` line of the body's doc comment. "high" means the guard, stores and
calls are literal in the export; "medium" means a virtual receiver, an `r13`
object role or a callee identity is inferred and called out in the body.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 30 | 1532 | 16 | 14 |

## What this pass covers

- `0x80108070..0x801087E0` - the core of a vtable-dispatched state controller.
  [`fn_80108070`] initialises the two-word state history at `+0x08`/`+0x0C`,
  mode `5` at `+0x14`, the five one-shot bytes `+0x18..+0x1C`, the counters at
  `+0x20`/`+0x24`, the stage word at `+0x28` and the status bytes at
  `+0x2D..+0x30`, then registers both callbacks. [`fn_80108230`] and
  [`fn_80108260`] advance stage `1`; [`fn_801082d0`] releases the
  `r13 - 0x53A8` object. [`fn_80108420`] dispatches the `+0x14` mode word to the
  five `0x801B3xxx` handlers, and [`fn_801083d0`] is the teardown path.
- `0x801080F0..0x80108330` - the child-object forwarding layer. The child is at
  `+0x04`; its virtual slots `+0x0C`, `+0x18`, `+0x1C` are called from
  [`fn_80108330`], [`fn_801080f0`] and [`fn_80108130`]. [`fn_80108180`] is the
  three-part idle gate (local `+0x2F`, the `0x80109340` global state's `+0x1D0`,
  and the object at `r13 - 0x5640`).
- `0x80108660..0x801087E0` - the state setters. [`fn_80108670`] and
  [`fn_801086b0`] write the mode/stage and refresh the four-entry table through
  [`fn_80108d00`]; [`fn_801086f0`] resets the mode to `5` through the empty
  `0x801084D0` hook; [`fn_801087e0`] clears the global byte at `0x8042A2DC`.
- `0x80108D00..0x801092E0` - the trigger and global checks. [`fn_80108d00`]
  notifies the four entries of the `r13 - 0x5658` table, [`fn_80108d70`] arms
  the controller's `+0x2F` byte, [`fn_80108ea0`] tests the `r13 - 0x56C8` state
  object's `+0x74`/`+0x78`, and [`fn_801092e0`] forwards the `r13 - 0x53D0`
  display object to `0x80108F70`.

The export does not name the class or its owner. The bodies therefore keep the
literal field offsets and are named by address; no subsystem name is invented.
The two 4-byte functions (`0x801084D0`, `0x80108D60`) are genuine empty virtual
hooks, not stubs.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80108070` | `fn_80108070` | 280 | high |
| `0x801080F0` | `fn_801080f0` | 64 | medium |
| `0x80108130` | `fn_80108130` | 20 | medium |
| `0x80108150` | `fn_80108150` | 12 | high |
| `0x80108160` | `fn_80108160` | 8 | high |
| `0x80108170` | `fn_80108170` | 8 | high |
| `0x80108180` | `fn_80108180` | 164 | medium |
| `0x80108230` | `fn_80108230` | 40 | medium |
| `0x80108260` | `fn_80108260` | 100 | medium |
| `0x801082D0` | `fn_801082d0` | 96 | medium |
| `0x80108330` | `fn_80108330` | 56 | medium |
| `0x80108380` | `fn_80108380` | 12 | high |
| `0x80108390` | `fn_80108390` | 12 | high |
| `0x801083A0` | `fn_801083a0` | 12 | high |
| `0x801083B0` | `fn_801083b0` | 12 | high |
| `0x801083C0` | `fn_801083c0` | 12 | high |
| `0x801083D0` | `fn_801083d0` | 72 | medium |
| `0x80108420` | `fn_80108420` | 68 | medium |
| `0x80108470` | `fn_80108470` | 92 | medium |
| `0x801084D0` | `fn_801084d0` | 4 | high |
| `0x80108660` | `fn_80108660` | 8 | high |
| `0x80108670` | `fn_80108670` | 60 | high |
| `0x801086B0` | `fn_801086b0` | 64 | high |
| `0x801086F0` | `fn_801086f0` | 56 | medium |
| `0x801087E0` | `fn_801087e0` | 12 | high |
| `0x80108D00` | `fn_80108d00` | 84 | medium |
| `0x80108D60` | `fn_80108d60` | 4 | high |
| `0x80108D70` | `fn_80108d70` | 12 | high |
| `0x80108EA0` | `fn_80108ea0` | 76 | medium |
| `0x801092E0` | `fn_801092e0` | 12 | medium |

## Deliberate skips

- `0x801084E0` (376 bytes) is the poll helper that drives the same controller.
  It contains the only large branch tree in the cluster and several dropped
  receiver arguments; it was left for a pass that can resolve `0x80108E80` /
  `0x80108EF0` together.
- `0x80108D80` (244 bytes) mixes three global gates, a dropped virtual receiver
  and two callback pointers; the smaller hooks it calls
  (`0x80108D70`, `0x80108EA0`, `0x801092E0`) were translated instead.
- `0x80108F70` (752 bytes) is a GX/EFB display setup with more than forty
  renderer calls. It is outside the state-controller boundary and was not
  reconstructed from its call list alone.
- `0x80106620`, `0x801068B0` and `0x80106B40` were skipped because their exports
  carry GQR reconstruction and several removed blocks; the numeric constants
  cannot be read without the GQR base.
- `0x80107050..0x80107C90` and `0x80108F40..0x80109350` are the adjacent
  resource/string-table and renderer-install helpers. They are real code but
  form separate clusters with their own callee sets, so they were left for a
  later pass instead of being mixed into this state-controller lane.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv` (the 30 addresses were taken
  from the 175 untranslated entries of the `0x80100000` bucket).
- `cargo check --message-format=short` is free of errors for
  `src/game/worldmap.rs`.
