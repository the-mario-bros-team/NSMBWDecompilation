# Coverage - Wave 95 (`game::enemies`, the `0x80AD0000` bucket)

## How to read this file

This page records the Wave 95 `game::enemies` lane: 27 functions of the
`0x80AD0000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs` (section 76 of that file). Every row below is one real
`pub unsafe fn fn_<addr>` body in that file, and the `evidence` column of the
machine-readable mirror (`ledger/agents/wave95-enemies.csv`) names the Ghidra C
export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. The float
and descriptor constants cited below were read out of
`work/rels/elf/d_enemiesNP.elf` at the cited synthetic addresses, never guessed
from the C text.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 27 | 3,556 | 15 | 6 | 6 |

The bucket holds 189 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 92 were already in
`ledger/functions.csv` before this pass (the wave-51 quicksand run, the
wave-65 `obj_obake_door` container, the wave-80 `0x4C30`/`ben` block, the
wave-4 `register_oneway_gate_states`/`alloc_oneway_gate_actor` pair with the
`0x80AD5050`/`0x80AD51C0` element helpers it uses, and the two
`PataKuribo` helpers at `0x80ADFA90`/`0x80ADFDD0`). 27 are claimed here and
70 remain.

## What this pass covers

Two runs, both inside `0x80ADA1A0..0x80ADC224`.

**Run 1 - the rest of `daEnOnewayGate_c`, `0x80ADA1A0..0x80ADB550`.**

- `0x80ADA1A0`/`0x80ADB3D0` are two 88-byte deleting destructors of the family
  (`0x8015F800(object, 0)` then `0x802B9280` when the mode word is positive).
- `0x80ADA2C0`/`0x80ADA2F0`/`0x80ADB4F0`/`0x80ADB520` are the 48-byte
  `0x802DCBFC` argument-swapping forwarders of the same class.
- `0x80ADA570` walks the `0x1C0`-byte object chain with `0x80162D50` and stops
  on the record whose `+4` top byte equals the actor's `+0x4B98` byte.
  `0x80ADA5D0` then copies `record + 0` into `+0x524`, `record + 0x528` into
  `+0x544` and `record + 0x5A1` into `+0x4B99`, and hands the nibble
  `(record[1] >> 4) & 0xF` back to the caller.
- `0x80ADA600` clears `+0x534`/`+0x538`, stores the two extents at
  `+0x53C`/`+0x540` and calls the `+0x280` virtual slot.
- `0x80ADA640` installs the `0x8007FA40` rectangle animator with the three text
  callbacks `0x80ADA350`/`0x80ADA3B0`/`0x80ADA410`, then stores the step
  callbacks `0x80ADA4B0`/`0x80ADA4E0`/`0x80ADA510` at `+0x594..+0x59C`, arms
  `+0x618` and enables the animator through `0x8007F900`.
- `0x80ADA6F0` is the half-extent block: `+0x62C..+0x638` receive
  `0.5 * (first, -second, first, second)`, the `+0x318`/`+0x31C` pair receives
  `768.0` and `768.0 + 32.0 * index` in the order chosen by the `+0x4B99` side
  byte, `+0x320`/`+0x324` receive the raw words `0x43000000`/`0xC3000000`, and
  `+0xD0..+0xD8` repeats the half-extent triple with a zero third word.
- `0x80ADAB80` is the `0x4BA0`-byte constructor: base constructor, class vtable
  `0x80B5E240`, the `+0x548` helper reset, the `+0x648` model block and the
  `+0x664` pool built as `2 x 0x2290` through `0x802DC9A0(element,
  0x80AD5050, 0x80AD51C0, 0x2290, 2)`.
- `0x80ADAC20` is the spawn initialiser. It needs a record from
  `0x80ADA570`, converts the signed `+0x4B9A`/`+0x4B9B` bytes into cell offsets
  with the `16.0` pitch at `0x80B30A6C` (the export's `2^52` bias at
  `0x80B30A70`), feeds `0x800BFC60` with the model block, both signed bytes,
  the `+0x290` halfword and the `+0x294` word, then runs the shared tail:
  state `0x80B76A90`, the `0x80ADA6E0` hook, slot `+0x288` and the
  `+0x4B84..+0x4B8C` position copy.
- `0x80ADAD80` refreshes the rectangle, calls the `+0x284` slot with the two
  cell offsets and writes the descriptor `{0, -1, 0x80ADAE40}` at
  `+0x63C..+0x644`; `0x80ADAF50` writes the sibling `{0, -1, 0x80ADA9E0}`
  descriptor, re-fits the rectangle with `0x8007F950` and calls slot `+0x28C`.
- `0x80ADAE60`/`0x80ADAFC0` move the gate onto its partner: the `+0x524`
  handle resolves through `0x80162D00`, the partner's `+0x524` offset is added
  to `+0x528` (or `+0x52C` when the side byte is set), the partner's `+0x528`
  word is copied into `+0x544`, and the partner's `+0x394` holder is asked
  through slot `+0x14` whether it still carries `0x80B76B68`. The two bodies
  differ only in which answer selects which state word
  (`0x80B76AD0` versus `0x80B76A90`).
- `0x80ADB550` is the second allocator of the class: it allocates `0x4BA0`,
  calls the constructor above and overwrites `+0x60` with the variant vtable
  `0x80B5E600`.

**Run 2 - the next actor of the bucket, `0x80ADB650..0x80ADC224`.**

- `0x80ADB650` constructs the `0x5A8`-byte actor. Vtable `0x80B5E8A8`,
  `+0x524` seeded from the zero word at `0x80B30A80`, the four holder words
  `0x80B575B8`/`0x80B56E48`/`0x80B575E8`/`0x80B57600` at `+0x52C..+0x538`,
  the self-pointer `+0x53C = this + 0x52C`, the `0x8015FC90` controller link
  over `0x80377A04`, and the rewind of `+0x52C` to `0x80B57588`.
- `0x80ADB720` decodes the spawn word: bit 3 selects `0x80B76BA8`, a zero
  nibble selects `0x80B76B28`, and otherwise bit 19 picks `0x800D7980` or
  `0x800D7880` before the `0x80B76BE8` entry. `0x80ADB7E0` is the plain
  variant: `0x800D77D0(+0x52C, nibble, 2 bits, nibble, nibble, 2 bits)` and a
  constant `1` return.
- `0x80ADB940`/`0x80ADBB30` gate on the `main.dol` contact masks at
  `0x8042A078`/`0x8042A07C` against `+0x358`/`+0x35C`, then start
  `0x800D79B0`/`0x800D7980` or re-arm with `0x800D7820`.
- `0x80ADBF70` is the third deleting-destructor copy of this bucket and
  `0x80ADC090`/`0x80ADC0C0` are the two forwarders in front of the last
  constructor.
- `0x80ADC0F0` constructs the `0x840`-byte actor in two identical holder
  layers: interim vtable `0x80B3AEF8`, `+0x528` model block, `+0x548` owner
  holder, `+0x588` animation holder over the `0x80B330E0` bank, the
  `+0x5C0`/`+0x5CC` sub-object pair with its vtable rewritten from
  `0x803295D8` to `0x80329518`, the `0x80329958` helper at `+0x638`, a cleared
  `+0x74C..+0x760` block with the two side bytes at `+0x754`/`+0x755`, and the
  second layer (`+0x764` owner holder, `+0x7A4` animation holder,
  `+0x7DC = 1`) under the final vtable `0x80B5EE08`.

### Identity evidence

- **Only one class name is claimed.** Run 1 is `daEnOnewayGate_c`: the wave-4
  StateID table at `0x80AD9E60` names its four records `CloseW`, `Open`,
  `OpenWa` and `Close`, and its allocator at `0x80ADA320` allocates the same
  `0x4BA0` bytes and calls `0x80ADAB80`, so the constructor below is that
  class's constructor. Run 2's export prints no C++ name at all, so it is
  described by size and offsets only.
- **Every float was dumped, not assumed.** `0x80B30A4C` = `0.5f`,
  `0x80B30A50` = `0x43000000`, `0x80B30A54` = `0xC3000000`,
  `0x80B30A58` = `768.0f`, `0x80B30A5C` = `32.0f`, `0x80B30A6C` = `16.0f`,
  `0x80B30A48` = `0x00000000`, `0x80B30A60` =
  `0x4330000000000000` (the `2^52` bias of the unsigned-to-double idiom) and
  `0x80B30A70` = `0x4330000080000000` (the `2^52 + 2^31` bias of the
  *signed*-byte idiom).
- **The state words are BSS.** `0x80B76A90`, `0x80B76AD0`, `0x80B76B28`,
  `0x80B76B68`, `0x80B76BA8` and `0x80B76BE8` all fall inside the module's BSS
  range (`0x80B6E500..0x80B799E0`), so they are entered by address and no
  `StateID` name is claimed.
- **The two descriptors are data words.** `0x80B5E228` = `0`,
  `0x80B5E22C` = `-1`, `0x80B5E230` = `0x80ADAE40` and `0x80B5E234` = `0`,
  `0x80B5E238` = `-1`, `0x80B5E23C` = `0x80ADA9E0`, read from the image.

### Deliberate skips

- **`0x80ADA6E0` (8 bytes).** The export is
  `/* WARNING: Control flow encountered bad instruction data */` plus
  `halt_baddata()`, yet every step of the gate calls it with `this` between the
  state switch and slot `+0x288`. No body can be reconstructed, so it is not
  claimed; each call site carries an `// unknown:` note and calls the address.
- **Everything else in the bucket: 70 functions.** The largest untouched runs
  after this pass are `0x80AD0960`/`0x80AD0A90`, `0x80AD3C10..0x80AD48C0`,
  `0x80AD5760`, `0x80AD5CD0`, `0x80AD6CA0..0x80AD7310`, `0x80AD8900`,
  `0x80AD98B0`, `0x80ADC440..0x80ADCC80` (the next `0x4xxx` element family),
  `0x80ADD0E0..0x80ADE450` and `0x80ADE970..0x80ADF160`.
- **Nothing was claimed "by shape alone".** The six `low` rows are the 48-byte
  forwarders; their argument reshuffle is literal, but neither the target's
  role nor the hidden `r12` this-adjust pointer is established, and every body
  carries the same `// partial:` note.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80ADA1A0` | `fn_80ada1a0` | 88 | high |
| `0x80ADA2C0` | `fn_80ada2c0` | 48 | low |
| `0x80ADA2F0` | `fn_80ada2f0` | 48 | low |
| `0x80ADA570` | `fn_80ada570` | 96 | high |
| `0x80ADA5D0` | `fn_80ada5d0` | 40 | high |
| `0x80ADA600` | `fn_80ada600` | 44 | medium |
| `0x80ADA640` | `fn_80ada640` | 156 | high |
| `0x80ADA6F0` | `fn_80ada6f0` | 232 | medium |
| `0x80ADAB80` | `fn_80adab80` | 128 | high |
| `0x80ADAC20` | `fn_80adac20` | 336 | medium |
| `0x80ADAD80` | `fn_80adad80` | 176 | medium |
| `0x80ADAE60` | `fn_80adae60` | 236 | medium |
| `0x80ADAF50` | `fn_80adaf50` | 96 | high |
| `0x80ADAFC0` | `fn_80adafc0` | 236 | medium |
| `0x80ADB3D0` | `fn_80adb3d0` | 88 | high |
| `0x80ADB4F0` | `fn_80adb4f0` | 48 | low |
| `0x80ADB520` | `fn_80adb520` | 48 | low |
| `0x80ADB550` | `fn_80adb550` | 76 | high |
| `0x80ADB650` | `fn_80adb650` | 208 | high |
| `0x80ADB720` | `fn_80adb720` | 192 | high |
| `0x80ADB7E0` | `fn_80adb7e0` | 92 | high |
| `0x80ADB940` | `fn_80adb940` | 236 | high |
| `0x80ADBB30` | `fn_80adbb30` | 116 | high |
| `0x80ADBF70` | `fn_80adbf70` | 88 | high |
| `0x80ADC090` | `fn_80adc090` | 48 | low |
| `0x80ADC0C0` | `fn_80adc0c0` | 48 | low |
| `0x80ADC0F0` | `fn_80adc0f0` | 308 | high |

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 27 sizes sum to
  3,556 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 27 was
  already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave95` prints
  `wave95-enemies.csv: 27 rows` and `OK`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/enemies.rs`. During this pass the workspace build was still stopped
  by a parse error in `src/engine/main_loop.rs` (another Wave 95 lane), so the
  file was additionally checked on its own with `platform.rs` in a scratch
  crate: it compiles with warnings only, none of them for this section.
