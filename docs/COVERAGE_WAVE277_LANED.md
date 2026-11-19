# Wave 277, lane D: `0x80301C00` (`daBullet_c` vtable)

Full decode, tables and gate logs: `work/wave277/laneD_report.md`.

## Correction to the wave-227 inventory

`0x80301C00` is a **166-slot class vtable**, not a "`daBullet_c` state table".
Evidence:

* header `0x80301BF8 = 0x80301BFC = 0`; the method array is the maximal
  166-word text-pointer run `0x80301C00..0x80301E98` (the next word is the
  string `"Wm_en_burst_s"`, the next vtable header is `0x80301EA8`);
* dispatch uses fixed offsets from the pointer stored at object `+0x60`
  (`lwz r12,0x60(rX)` / `lwz r12,8+4*slot(r12)` / `mtctr` / `bcctr`);
* the only materialisations of `0x80301BF8` in the image set are four
  `d_enemiesNP` constructor pairs (`0x80A17180/88`, `0x80A17620/28`,
  `0x80A17A20/28`, `0x80A17D70/78`) that store it at `+0x60` and later replace
  it with the concrete REL vtable (`0x80B368F8`, `0x80B36BA8`, `0x80B36E58`,
  `0x80B37108`). Those four vtables are also 166 words and share 140-141 words
  with this array, overriding 25-26 slots (16 = the deleting destructor, 20-29,
  44, 55, 57, 66, 74, 75, 82, 136, 141-143, 158-165).

The adjacent `daBullet_c::StateID_*` strings (`0x80301A67..0x80301B57`) are the
class's state-name rodata; the state machine they describe is separate from this
vtable.

## Census

| | covered | uncovered | prologue-shaped |
|---|---:|---:|---:|
| start | 147 | 19 | 8 |
| end | **155** | **11** | **0** |

## Bodies (8, 916 bytes, `_D277`, `src/ui/hud.rs`)

| slot | target | bytes | role |
|---:|---|---:|---|
| 0 | `0x8001F900` | 320 | registry insert, model build, three vtable calls |
| 3 | `0x8001FCE0` | 88 | release `+0x148`/`+0x608`, unregister |
| 6 | `0x8001FC50` | 68 | `0x80020070` + `0x8001FD40` + `0x80064F50(this,0)` |
| 9 | `0x8001FCA0` | 48 | `+0x544` emitter virtual `+0x14` |
| 16 | `0x80020700` | 152 | deleting destructor |
| 58 | `0x8001FE80` | 168 | contact kind `1`/`2` via slots 74/75 |
| 74 | `0x8001FF30` | 36 | forward to `stage::fn_80097e50`, answer `0` |
| 75 | `0x8001FF60` | 36 | forward to `stage::fn_80097ea0`, answer `0` |

Eleven non-prologue leaves remain (seven `blr`, two `li r3,0`/`blr`, one
`lfs` getter, one `+0x220` tail-dispatch).

## Gates

`cargo check` exit 0 (no warning in `ui/hud.rs`), `doc-lint` 0 violations,
`check-dupe-bodies` exit 0, `check-page-offsets` 0 mismatches.
