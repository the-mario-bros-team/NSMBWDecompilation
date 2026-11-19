# Coverage - Wave 101 (UI: HUD panel updates, wipe gates and record accessors)

Pass 13 of the `ui::hud` lane. The lane's 64 KiB bucket `0x80010000` still had
33 untranslated entries after wave 96. This pass lands the 26 bodies that could
be reconstructed honestly from the Ghidra exports, which leaves 7 entries
documented below as deliberate skips.

## How to read this file

Every row maps one Ghidra function to one `pub unsafe fn fn_<addr>` body in
`src/ui/hud.rs`. `address` is the link-time address in `sys/main.dol`, `bytes`
is the size recorded in `work/ghidra/main-gb/export/functions.tsv`, and
`confidence` matches the `# Confidence` line of the body's doc block.

Three clusters were claimed in this pass:

- the tail of the four-player panel state machine
  (`0x80010690..0x80010DD0`),
- the last two wipe-scene gate bodies plus the five remaining
  `0x80377A04` accessors and the table-driven field copy
  (`0x8001B580..0x8001FF90`),
- the small record accessors and forwarding thunks
  (`0x80012940..0x800148F0`).

`0x80010690..0x80010DD0` sits inside the band wave 90 and wave 96 deferred to
`crate::game::path`, and `0x80014330..0x800148F0` are the record accessors that
`crate::game::actor` calls. Both groups were still untranslated in
`ledger/functions.csv` when this pass ran, no other lane owns the `0x80010000`
bucket in wave 101, and every body below is a complete, literal reconstruction,
so they are recorded here. The other modules keep their one-way by-address
references and were not edited.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/hud.rs` | `ui::hud` | 26 | 3,052 | 16 | 10 |

## What this pass covers

- **Four-player panel updates, `0x80010690..0x80010DD0`.** A manager handle from
  `0x802DCD6C`/`0x802DCD64`, four channels from `0x802EDCE0`, pane slots at
  `+0x3D4..+0x3FC`, the per-channel cache at `+0x414`, the message pane indices
  at `0x802EDD68`/`0x802EDD78`, the message code at `+0x410`, the alpha word at
  `+0x430`, the dirty flag at `+0x44E` and the presence mask at `+0x434`.
  [`fn_80010950`] is the close body and drives the same `+0x1A0`,
  `+0x218..+0x2CC`, `+0x344`, `+0x308` widgets that wave 90 already records.
- **Wipe-scene gates, `0x8001B580..0x8001BA20`.** [`fn_8001b580`] is the close
  path (`+0x1C0` build flag, `+0x1BC` state word) and [`fn_8001ba20`] applies
  wipe mode `0`/`1` through the layout at `+0x14`, the root pane at `+0x1AC`
  and the alpha pane at `+0x1B0`. [`fn_8001b920`] is the `wipeCircle` gate: it
  waits for the animation, then copies [`WIPE_ZERO_WORD`] into `+0x1B4`/`+0x1B8`
  and writes the state word `3` at `+0x1BC`, matching the builder
  [`fn_8001b470`].
- **Frame-delta and pair tests, `0x8001BF20`, `0x8001CEE0`.** The first compares
  the `f32` at `+0xB0` with the delta returned by `0x800B30E0`; the second
  compares two bytes at `+0x120C`/`+0x120D` through `0x800FAE40`.
- **`0x80377A04` accessors, `0x8001DA80..0x8001DAC0`.** Copies five to nine of the
  byte-identical body; with the seven from wave 96 the run of twelve is now
  complete.
- **Table-driven field copy, `0x8001FF90`.** The index at `+0x5E4` selects one
  word from `0x802EE260` and `0x802EE26C` and one three-word block from each of
  `0x802EE278` and `0x802EE29C`, stored at `+0x5F8`, `+0x5FC`, `+0x6AC..+0x6B4`
  and `+0x6B8..+0x6C0`.
- **Panel forwarding thunks, `0x80012940`/`0x80012970`.** The same
  `0x802DCBFC(b, b, a)` shape as the eight thunks recorded in waves 28 and 96.
- **Record accessors, `0x80014330..0x800148F0`.** A one-word clear, a three-word
  clear, and the `u16`/`u8`/`f32` getters at `+0x08`, `+0x0A`, `+0x0C`, `+0x10`
  and `+0x14`. `0x800146C0` is the state byte read by `fn_80014710`,
  `fn_800147a0` and `fn_800147e0` in `crate::game::actor`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800105B0` | `fn_800105b0` | 8 | high |
| `0x80010690` | `fn_80010690` | 360 | high |
| `0x80010800` | `fn_80010800` | 184 | medium |
| `0x80010950` | `fn_80010950` | 424 | high |
| `0x80010B50` | `fn_80010b50` | 640 | medium |
| `0x80010DD0` | `fn_80010dd0` | 356 | medium |
| `0x80012940` | `fn_80012940` | 48 | medium |
| `0x80012970` | `fn_80012970` | 48 | medium |
| `0x80014330` | `fn_80014330` | 12 | high |
| `0x80014630` | `fn_80014630` | 8 | high |
| `0x80014670` | `fn_80014670` | 8 | high |
| `0x80014680` | `fn_80014680` | 8 | high |
| `0x80014690` | `fn_80014690` | 8 | high |
| `0x800146C0` | `fn_800146c0` | 8 | high |
| `0x800148F0` | `fn_800148f0` | 20 | high |
| `0x8001B580` | `fn_8001b580` | 128 | medium |
| `0x8001B920` | `fn_8001b920` | 168 | medium |
| `0x8001BA20` | `fn_8001ba20` | 228 | high |
| `0x8001BF20` | `fn_8001bf20` | 104 | medium |
| `0x8001CEE0` | `fn_8001cee0` | 112 | medium |
| `0x8001DA80` | `fn_8001da80` | 12 | high |
| `0x8001DA90` | `fn_8001da90` | 12 | high |
| `0x8001DAA0` | `fn_8001daa0` | 12 | high |
| `0x8001DAB0` | `fn_8001dab0` | 12 | high |
| `0x8001DAC0` | `fn_8001dac0` | 12 | high |
| `0x8001FF90` | `fn_8001ff90` | 112 | medium |

## Deliberate skips

- **`0x800129A0` (6,204 bytes).** The static initialiser of the course-select
  guide state table: 704 lines of `DAT_802FE4xx`/`DAT_8035xxxx` stores and
  `0x802DC780` registrations built from the `0x802FEB48..0x802FEFC8` data block.
  Reproducing it means transcribing that whole block, and it is course-select
  state data rather than a HUD scene, so it stays a next target.
- **`0x8001B620` (684 bytes).** Still corrupted by the decompiler in exactly the
  way wave 96 described: `unaff_GQR0` supplies the branch value for sixteen
  `ldexpf` quantiser copies and the 22 constant stores (`DAT_803525E8..`,
  `DAT_80352654`) read unnamed SDA2 words, so the float values and the two
  `psq_l`-shaped blocks cannot be recovered from the export.
- **`0x8001E6F0` (160 bytes) and `0x8001E790` (188 bytes).** Float/vtable bodies.
  Every call prints `(double)` conversions and stack records
  (`local_18`, `local_30`, `auStack_38`) whose layouts are visible but whose
  parameter widths and roles are not, so a body would have to guess the ABI.
- **`0x8001F8A0` (84 bytes).** Calls `0x80020100` with no operands printed at
  all; translating it would require inventing the argument list. Same reason
  wave 96 gave.
- **`0x8001FA70` (468 bytes).** The `daikonbou` model/animation builder: five
  callees take `(double)` operands (`0x80169F80`, `0x80166EF0`, `0x80167B20`,
  `0x801C0BD0`-shaped) plus six-argument calls whose records are only partially
  visible, so the argument widths stay unresolved.
- **`0x8001FD40` (244 bytes).** Same `(double)`-argument ambiguity at
  `0x801C0BD0`, and the virtual slot `+0x298` it calls is unidentified; the
  three `f32` copies and the three `u16` copies are literal, but the call
  contract is not.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/...` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in `src/ui/hud.rs`.
- No address was already in `ledger/functions.csv`; the CSV validator checks this
  live against the merged ledger.
- `cargo check --message-format=short` reports no error for `src/ui/hud.rs`.
- `node tools/check-lane-csv.mjs wave101` prints `OK`; the problems it reports for
  other lanes' files are out of scope for this lane.
