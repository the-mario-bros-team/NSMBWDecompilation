# Wave 223, lane C - `src/game/stage.rs`, addresses `0x800A0700..0x800A65E0`

Ten entries of `work/ghidra/main-gb/export/functions.tsv` in the `0x800A0000`
band of the `game::stage` ledger module. All ten are translated; none deferred.
Bytes translated: **1736**.

| address | tsv size | real extent | body |
|---|---|---|---|
| `0x800A0700` | 452 | 452 (`blr` at `0x800A08C0`) | publish the variant record from a sector index |
| `0x800A0DC0` | 212 | 212 (`blr` at `0x800A0E90`) | publish the variant record from a kind byte |
| `0x800A1C80` | 260 | 260 (`blr` at `0x800A1D80`) | seed nine label ids and print one |
| `0x800A2530` | 92 | 92 (`blr` at `0x800A2588`) | release and optionally free a node |
| `0x800A2810` | 48 | 48 (`blr` at `0x800A283C`) | forward into `0x802DCBFC` |
| `0x800A2BC0` | 12 | 12 (`blr` at `0x800A2BC8`) | return the shared type-info record |
| `0x800A3060` | 28 | 28 (`blr` at `0x800A3078`) | clear five flag bytes |
| `0x800A4990` | 228 | 228 (`blr` at `0x800A4A70`) | restart the player-spin effect |
| `0x800A4DF0` | 112 | 112 (`blr` at `0x800A4E5C`) | x-order test against the `+0xD10` player |
| `0x800A65E0` | 292 | 292 (`blr` at `0x800A66FC`) | ground/air reset plus spin-group clear |

No `functions.tsv` size in this lane is a prologue artefact: each was checked
against the prologue and the closing `blr` in `work/dol/main.elf`, and in every
case the `blr` ends exactly at `address + tsv_size`. What follows each `blr` is
four-byte `0x00000000` padding before the next entry's prologue, except for the
frameless twelve-byte `0x800A2BC0`, where the padding runs to `0x800A2BD0` and
that address is the next stub.

## What the run is

Three of the ten (`0x800A0700`, `0x800A0DC0`, and the reset `0x800A65E0`) belong
to the same `Wm_jr` family the file already documents under the
`+0x414..+0x432` variant record: a seven-word payload plus three kind bytes.
The two publishers write *different* `.rodata` templates, which is the only
difference between them:

* `0x800A0700` publishes the record from `0x802F0680`.
* `0x800A0DC0` publishes the same shape from `0x802F06E0`.

Both were confirmed against the sibling `fn_80097F70` (`0x80097F70`, wave 84),
which publishes the record from `0x802F04A4` and is the closest relative of
`0x800A0700`: same `+0x6C` vtable byte, same `0x800954D0` submit, same
`r13-0x5924` / `0x17B` message and the same ten stores. `0x800A0700` adds the
sector computation, the counter arm and a per-sector float.

`0x800A65E0` is the reset that pairs with them: four `sdata2` floats, the
`+0x750` threshold, the shared `0x8042C1B0` float table the record publishers
also index (already named `W84_VARIANT_TABLE` in this file), the two release
calls and two `0x114`-stride spin groups whose `+0x14` slot is called twice
each.

`0x800A4990` drives the same `0x114`-stride spin object that `0x800A65E0`
clears: it selects the object with `+0x78C`, hands it the
`"Wm_jr_playerspin"` effect descriptor with `(position, meta, scale)` and then
calls its `+0x30` slot with the low halfword of the vtable's `+0x3A8` answer.

The remaining four are small:

* `0x800A2530` is byte-for-byte the body of the already-translated
  [`fn_8009f2b0`], and carries the same `size > 0` delete convention.
* `0x800A2810` forwards to `0x802DCBFC` with `r12 = first + 0x18`.
* `0x800A2BC0` is the twelfth `return &DAT_80377a04;` stub in the file and
  reuses [`W84_TYPE_INFO`].
* `0x800A3060` clears five bytes; `0x800A1C80` seeds nine label ids and prints
  one of them; `0x800A4DF0` is the pure `+0xD10` / `+0xAC` predicate.

## New constants (lane C)

| name | value | role |
|---|---|---|
| `W223C_RECORD_A` | `0x802F0680` | template of `0x800A0700` |
| `W223C_RECORD_B` | `0x802F06E0` | template of `0x800A0DC0` |
| `W223C_LABEL_TABLE` | `0x80357DA8` | nine label ids seeded by `0x800A1C80` |
| `W223C_LABEL_SEEDED` | `r13-0x5778` | the seed byte |
| `W223C_SECTOR_INDEX` | `0x80064A30` | the two-float sector dispatcher |
| `W223C_UPDATE_SPIN_COUNTER` | `0x80065B80` | spin-counter report |
| `W223C_RESOLVE_VARIANT_WORD` | `0x8009F710` | `+0x510`-mode variant word |
| `W223C_SPIN_REFRESH` | `0x800A4AC0` | pre-`+0xA0` spin refresh |
| `W223C_PLAYER_NOTIFY` | `0x800CE8C0` | player-record announce |
| `W223C_REBUILD_654` | `0x8007F950` | `+0x654` rebuild |
| `W223C_PLAYERSPIN_NAME` | `0x80313738` | `"Wm_jr_playerspin"` |

Existing helpers reused instead of redefined: `W84_TYPE_INFO`,
`W84_VARIANT_TABLE` (`0x8042C1B0`), `W69_G_EVENT_MANAGER` (`r13-0x5924`),
`W48_G_PLAYER_STATS` (`r13-0x56D0`), `W33_CAMERA_SUM` (`0x8006CED0`),
`COLLISION_AREA_UNREGISTER` (`0x8008C380`), `TILE_METHOD_DISPATCH`
(`0x802DCBFC`), `DELETE_ONE` (`0x802B9280`) and `fn_800954d0`.

## Evidence notes

* Both publishers and the reset carry a dead second copy of their payload into
  an unused stack slot after the real stores (`r1+0x28..r1+0x45` in
  `0x800A0700`, `r1+0x18..r1+0x25` in `0x800A0DC0`). Nothing in the frame reads
  those copies; they are documented in the doc blocks and not reproduced.
* `0x800A0700`'s stack triple `(0x90, 0x94, r2-0x6DA0)` is trimmed by the callee:
  `0x800954D0` reads only the first two words and takes its own third word from
  `r2-0x6EA8`. The translated body therefore passes a two-word point, exactly
  like the file's existing `fn_80097F70`.
* `0x800A1C80`'s buffers are *not* strings: `0x8006CED0` writes three floats and
  `0x8006A4B0` writes two, so the locals are `[u32; 3]` and `[u32; 2]`.
* `0x800A2810`'s `r12 = first + 0x18` cannot be expressed by a Rust call into
  `0x802DCBFC`; the three register arguments are passed and the `r12` value is
  documented. That is why its row is `medium`.
* All addresses/constants were read from `work/dol/main.elf` disassembly, not
  from the export alone; where the export and the instruction stream disagreed
  on a compare sense (`bc 4,1`), the instruction stream won.
