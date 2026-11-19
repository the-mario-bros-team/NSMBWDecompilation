# Wave 223, lane A - `src/game/stage.rs`, addresses `0x800A1190..0x800A7420`

Ten entries of `work/ghidra/main-gb/export/functions.tsv`, all in the
`0x800A0000` run of the `game::stage` ledger module. All ten are translated;
none deferred.

| address | tsv size | real extent | body |
|---|---|---|---|
| `0x800A1190` | 272 | 272 (`blr` at `0x800A129C`) | pick a `Wm` variant record from two actor positions |
| `0x800A1830` | 216 | 216 (`blr` at `0x800A1904`) | advance the two `s16` step counters and run the notice update |
| `0x800A1980` | 100 | 100 (`blr` at `0x800A19E0`) | emit the `Wm_en_burst_m` burst effect |
| `0x800A2BE0` | 12 | 12 (`blr` at `0x800A2BE8`) | return the shared type-info record `0x80377A04` |
| `0x800A3100` | 812 | 812 (`blr` at `0x800A3428`) | clown-Jr model constructor |
| `0x800A42B0` | 280 | 280 (`blr` at `0x800A43C4`) | build the sprite/quad offset matrix |
| `0x800A47B0` | 104 | 104 (`blr` at `0x800A4814`) | re-bind the `"getin"` entry |
| `0x800A7250` | 224 | 224 (`blr` at `0x800A732C`) | walk the `+0x30` chain to the qualifying parent |
| `0x800A73F0` | 48 | 48 (`blr` at `0x800A741C`) | `0x802DCBFC(second, second, first)`, descriptor `+0x18` |
| `0x800A7420` | 48 | 48 (`blr` at `0x800A744C`) | same, descriptor `+0x24` |

Bytes translated: 2116.

No `functions.tsv` size in this lane is a prologue artefact: each one is
`blr + 4 - start`. `0x800A1830` is followed by two 16-byte stubs at
`0x800A1908`/`0x800A190C` that the next entry at `0x800A1910` does not cover,
but they belong to neither function; the tsv size ends at the `blr`.

## What the run is

These ten are the `Wm` enemy-actor half of the clown-Jr stage code:

* `0x800A1190` is the twin of the wave-84 body `fn_80097f70` (`0x80097F70`):
  the same `+0x414..+0x432` record layout, the same `W84_VARIANT_TABLE`
  (`0x8042C1B0` = `1.5, -1.5, -0.0625, ...`) and the same "variant is valid"
  byte at `+0x432`. The difference is where the variant index comes from:
  `0x80064A30` (a `r13 - 0x5980` jump-table dispatcher) compares the two
  actors' `+0xAC + +0xD0` sums instead of reading a byte field. The copied
  tail is `0x802F0760` (`0.0f, 3.8f, -4.0f, -0.243902f, 0x80357BF4, -1, -1`),
  a different record from the wave-84 `0x802F04A4` tail.
* `0x800A1830` is the per-frame body: a direction bit from `+0x459` vs
  `+0x348`, `+0xC00`/`-0xC00` plus the signed halfword from `0x8042C320`
  (`+256`/`-256` in the first two slots), a `0.5` scale when the `+0x45D`
  freeze flag is set, then `0x8006CDE0`, `fn_800955f0` and `fn_80097170`.
* `0x800A1980` is a three-float position sum (`0x8006CED0`) handed to
  `0x8016C890` with `"Wm_en_burst_m"` (`0x80313488`), followed by the virtual
  `+0x98` update.
* `0x800A2BE0` is one of the twelve-byte `0x80377A04` accessors and reuses
  `W84_TYPE_INFO`.
* `0x800A3100` is the clown-Jr model constructor and the direct sibling of the
  `Wm_clown` constructor `fn_8009d010`: same callees (`0x8023A0B0`,
  `0x801650D0`, `0x801651F0`, `0x80169F80`, `0x801680E0`, `0x80166830`,
  `0x80166BD0`, `0x80165C30`, `0x80165FD0`, `0x80166310`, `0x801662F0`), the
  same `ScnMdl` type test against `0x802F8FF0`, only the node family is
  `+0x524`/`+0x544`/`+0x584`/`+0x5BC`/`+0x5EC`/`+0x61C` instead of `+0x600`.
  The archive path is `"g3d/koopaJr_clown.brres"` from `"koopaJr_clown"`, and
  the three named entries are `"getin"`, `"face"` and `"damage"` at
  `0x80427A78`, `0x80427A80` and `0x80427A88` (`string` dump of those
  addresses confirmed the ASCII).
* `0x800A42B0` translates to the actor position, applies the three `s16`
  descriptors at `+0x100`/`+0x104`/`+0x102` in that order through
  `0x8016ECB0`/`0x8016EEF0`/`0x8016EDD0`, and concatenates `T(0,+0x774,0)`,
  `T(0,-0x774,0)` and `T(0,height,0)` where `height` is the virtual `+0x2F8`
  result. The `stfs` at `0x800A42E8` proves that virtual returns `f32`, not the
  `double` Ghidra prints.
* `0x800A47B0` is the `"getin"` slice of `0x800A3100` on its own, but its
  second scale slot is `1.0f` (`0x8042C334`), not the `0.0f` used by the
  constructor.
* `0x800A7250` inlines five `+0x30`-child tests and, if all fail, recurses on
  the same child; it always returns the *parent* of the qualifying child.
* `0x800A73F0`/`0x800A7420` are byte-identical to `fn_800d7c20`/`fn_800d7c50`
  in `src/engine/display.rs` except for the `r12` descriptor offset (`+0x18`
  vs `+0x24`).

## Deferrals

None.

## Caveats recorded in the bodies

* `0x800A1190` and the two `0x802DCBFC` forwards are `medium`: the record
  meaning is inferred from the byte-identical `fn_80097f70`, and `0x802DCBFC`
  takes its dispatch table in `r12`, which the Rust ABI cannot express. The
  module's established convention (`fn_800d7c20`) forwards only the three
  integer operands; the descriptor offset is documented per function.
* `0x800A42B0`'s three halfwords are documented as rotation descriptors, the
  role the callees make explicit; the offsets themselves are literal.

## Verification

Block: `work/wave223/laneA_block.rs`. Appended to a scratch copy of `src/` and
`cargo check --lib` run there: exit 0, no errors, no warnings from the block.
