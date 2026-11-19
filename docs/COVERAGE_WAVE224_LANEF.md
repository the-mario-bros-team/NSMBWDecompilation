# Wave 224, lane F - `game::stage`

Ten bodies from the actor/event-slot cluster `0x800A78D0..0x800A8AB0`,
appended from `work/wave224/laneF_block.rs`. Every `functions.tsv` size was
re-checked against the prologue and the `bclr` in `work/dol/main.elf`; all ten
match exactly (no prologue artefacts). The six 12-byte entries were decoded
from the raw words rather than from the Ghidra exports.

| address | name | tsv size | prologue -> `bclr` | status | confidence |
|---|---|---|---|---|---|
| `0x800A78D0` | [`fn_800a78d0`] | 196 | no prologue -> `0x800A7990` | translated | high |
| `0x800A7C00` | [`fn_800a7c00`] | 88 | `0x800A7C00` -> `0x800A7C54` | translated | high |
| `0x800A7C60` | [`fn_800a7c60`] | 196 | `0x800A7C60` -> `0x800A7D20` | translated | high |
| `0x800A7DB0` | [`fn_800a7db0`] | 12 | no prologue -> `0x800A7DB8` | translated | high |
| `0x800A7DC0` | [`fn_800a7dc0`] | 12 | no prologue -> `0x800A7DC8` | translated | high |
| `0x800A8790` | [`fn_800a8790`] | 12 | no prologue -> `0x800A8798` | translated | high |
| `0x800A87A0` | [`fn_800a87a0`] | 12 | no prologue -> `0x800A87A8` | translated | high |
| `0x800A8850` | [`fn_800a8850`] | 12 | no prologue -> `0x800A8858` | translated | high |
| `0x800A8860` | [`fn_800a8860`] | 12 | no prologue -> `0x800A8868` | translated | high |
| `0x800A8AB0` | [`fn_800a8ab0`] | 88 | `0x800A8AB0` -> `0x800A8B04` | translated | high |

Totals: 10 bodies, 640 bytes, no deferrals.

## What the lane covers

* **`0x800A78D0`** - 32-slot conflict test. Two parallel arrays sit in one
  0x100-byte record: the slot values at `+0x00..+0x7C` and the slot keys at
  `+0x80..+0xFC`. The original unrolls four arms per `bdnz` iteration over eight
  iterations; a slot conflicts when its value is non-zero, its key equals
  `param_2` and its value is not `param_3 + 1`. `1` means "no conflict".
* **`0x800A7C00`** - event-slot advance. `+0x154` is pre-incremented, the
  eighth slot calls the player-stat notification `0x800CE8C0` with the record
  behind `r13-0x56D0`, and the counter saturates at `8`.
* **`0x800A7C60`** - event submission. The nine ids `0x19D..0x1A3` are seeded
  once into `0x80357FD0` under the `r13-0x576C` guard byte; for `slot < 7` the
  caller's position is expanded into a 16-byte record by `0x8006A4B0` and
  handed to the event manager (`r13-0x5924`) through `0x80197F00`.
* **`0x800A7DB0` / `0x800A7DC0` / `0x800A8790` / `0x800A87A0` / `0x800A8850` /
  `0x800A8860`** - six byte-identical 12-byte getters. Every one decodes to
  `lis r3,0x8037` (`0x3C608037`), `addi r3,r3,0x7A04` (`0x38637A04`),
  `bclr 20,0` (`0x4E800020`), i.e. the shared type-info record `0x80377A04`
  already named [`W84_TYPE_INFO`] in this file.
* **`0x800A8AB0`** - constructor. `0x802D7AA0` builds the base sub-object with
  `this` still in `r3`, then the vtable `0x80329958` is installed at `+0x00`
  and `+0x114`, `+0x118`, `+0x11C` (byte), `+0x11D` (byte), `+0x120` and
  `+0x124` are cleared. Returns `this`.

## Export corrections recorded in the doc blocks

1. `0x800A78D0` - the Ghidra `break` on the second arm is not an early loop
   exit; the raw listing uses the same `bclr` that the other three arms use.
   The `addi r7,r7,3` at `0x800A7984` increments a dead counter and is not
   modelled.
2. `0x800A7C60` - `param_1` (`r3`) is never read. The guard byte is loaded with
   `lbz`, so it is unsigned even though the export spells it `char`.
3. `0x800A8AB0` - the export prints the two byte clears through an `int`
   pointer (`param_1 + 0x47`), but the raw words are `stb r0,0x11c(r31)` and
   `stb r0,0x11d(r31)`.

## Verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to a scratch directory, this
block was appended to the copy's `src/game/stage.rs`, and
`cargo check --lib` finished with exit code 0.
