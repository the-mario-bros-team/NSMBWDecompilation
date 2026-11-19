# Wave 226, lane B - `src/resource/archive.rs`, four untabled bodies

Four `0x802C3xxx` bodies that have **no** `functions.tsv` row and therefore no
Ghidra export and no ledger row today. All four are only reachable as vtable
words; nothing in `main.elf` branches or calls to them directly.

| address | tsv size | real extent | body |
|---|---|---|---|
| `0x802C3460` | none (untabled) | 28 (`bcctr 20,0` at `0x802C3478`) | if bit 0 of `+0x00` is set, tail-call vtable slot `+0x40` (`0x802C3980`) with `f1`/`f2` |
| `0x802C3490` | none (untabled) | 28 (`bcctr 20,0` at `0x802C34A8`) | same guard, tail-call slot `+0x44` (`0x802C3CB0`) with an integer mode |
| `0x802C3510` | none (untabled) | 8 (`b 0x802C34B0`) | `r3 -= 8`, branch to [`fn_802c34b0`] |
| `0x802C3520` | none (untabled) | 8 (`b 0x802C34B0`) | `r3 -= 20`, branch to [`fn_802c34b0`] |

Bytes translated: **72**. No `functions.tsv` size is quoted anywhere in the
block, because no row exists: every extent is measured from `work/dol/main.elf`
(`0x802C3460..0x802C3478`, `0x802C3490..0x802C34A8`, `0x802C3510..0x802C3518`,
`0x802C3520..0x802C3528`).

## Why these four are in the archive module

The band `0x802C32B0..0x802C34B0` is already translated here:
[`fn_802c32b0`] (panel child-slot walk, 432 bytes, ends exactly at
`0x802C3460`) and [`fn_802c34b0`] (deleting destructor, 92 bytes). The four
untabled bodies sit in the two holes around them:

* `0x802C3460` and `0x802C3490` fill `0x802C3460..0x802C347C` and
  `0x802C3490..0x802C34AC`, i.e. exactly the gaps before the `0x802C34B0`
  destructor; the `bclr 20,0` words at `0x802C347C`, `0x802C3480` and
  `0x802C34AC` are padding (`0x802C3480` is in fact a four-byte empty body
  used as a vtable word, but it is outside this lane and stays untranslated).
* `0x802C3510` and `0x802C3520` fill `0x802C3510..0x802C3518` and
  `0x802C3520..0x802C3528`, between the `0x802C34B0` destructor (ends
  `0x802C350C`) and the still-untabled `0x802C3530` body (starts
  `0x802C3530`, ends `0x802C356C`).

## What the two wrappers are

Both are seven-instruction guard-and-forward bodies:

```
lbz    r0,0x0(r3)          ; first byte of the object
rlwinm.r0,r0,0,31,31       ; keep bit 31 only, set CR0
bclr   12,2                ; return when the bit is clear
lwz    r12,0x4(r3)         ; primary vtable
lwz    r12,0x40(r12)       ; slot +0x40   (0x44 for 0x802C3490)
mtspr  288,r12
bcctr  20,0                ; BO=20 -> unconditional: a tail call
```

`bcctr 20,0` (not `bctrl`) is the important detail: it is a real tail call, so
`r4..r10` and `f1..f13` reach the callee untouched and the callee returns
straight to the wrapper's caller. The wrapper itself only ever touches `r3`
(the guard path leaves it equal to `this`), which is why the clear-flag path is
written as a bare `return` in the Rust body.

### Which slots they occupy, and where they forward to

The class' primary vtable base is `0x80350230`, already in this module as
`VTABLE_80350230_F` and stored to the object's `+0x04` by the constructor
[`fn_802c3790`] (`*(undefined **)(param_1 + 4) = &DAT_80350230;`, with the
sub-vtables at `+0x10`, `+0x14`, `+0x18` = base `+0x0C`, `+0x1C`, `+0x50`).

| vtable word | meaning |
|---|---|
| `0x80350228` = base `+0x48` | `0x802C3460` |
| `0x8035022C` = base `+0x4C` | `0x802C3490` |
| `0x80350270` = base `+0x40` | `0x802C3980` (forward target of `0x802C3460`) |
| `0x80350274` = base `+0x44` | `0x802C3CB0` (forward target of `0x802C3490`) |

So the guard test is not self-recursive: `+0x40`/`+0x44` are two *other*
virtuals of the same table, and `0x802C3460`/`0x802C3490` are the `+0x48`/`+0x4C`
virtuals that reproduce them conditionally.

Neither forward target is a ledger row:

* `0x802C3980` is a `stwu r1,-0x30` float body; it reads `f1` and `f2` before
  writing them (`fsubs f4,f2,f1` at `0x802C39B8`, `fneg f3,f1` at
  `0x802C39C0`), loads `f5` from `r2 - 0x3D2C`, and overwrites `r4` from
  `r13 - 0x4780` - hence the two-`f32` signature of [`fn_802c3460`].
* `0x802C3CB0` keeps `r4` in `r31`, tests it against `0`, `1` and `2`
  (`cmpi cr0,4,r4,0` at `0x802C3CB8`) and dispatches to `0x802C4700` /
  `0x802C3D80` - hence the integer `mode` of [`fn_802c3490`].

The same two wrappers also appear in the sub-object vtables of the family
through `this` adjustors: `0x8008A070` is `addi r3,r3,-8; b 0x802C3490` and
`0x8008A080` is `addi r3,r3,-20; b 0x802C3460`. A second primary vtable at
`0x80310BC0..0x80310BCC` holds `0x802C3980`, `0x802C3CB0`, `0x802C3460`,
`0x802C3490` as four consecutive words, and `0x80350278`/`0x8035027C`
(base `+0x48`/`+0x4C`) plus `0x803502F0`/`0x803502F4` (the two words in front
of `VTABLE_803502F8_F`) repeat the pair.

`// inferred:` the `+0x00` byte is an enable flag maintained outside this band;
no writer of `+0x00` exists between `0x802C3000` and `0x802C3800`.

## What the two thunks are

Both are eight-byte `this` adjustors for the deleting destructor
[`fn_802c34b0`] of the neighbouring class (the one whose child block lives at
`+0x08` and whose vtable is read from `+0x04` by [`fn_802c32b0`]):

* `0x802C3510`: `addi r3,r3,-8;  b 0x802C34B0`, stored once at `0x803501F8`;
* `0x802C3520`: `addi r3,r3,-20; b 0x802C34B0`, stored once at `0x80350204`.

`-8` and `-20` are the sub-object offsets `+0x08` and `+0x14`, the same two
offsets the wrappers' adjustor thunks use, i.e. one complete object carries
both sub-objects and each sub-object's deleting-destructor slot points back at
the complete object's destructor. The branch is `b`, so `bclr 20,0` inside
`0x802C34B0` returns to the dispatcher and `r3` is the destructor's result.

## New constants

None. The block reuses `platform::read_u8`, `platform::read_u32`,
`core::mem::transmute` and the existing [`fn_802c34b0`]; the vtable base is only
named in prose (it is already `VTABLE_80350230_F`). Slot offsets `0x40`, `0x44`
and the object offsets `8`, `0x14` are written inline, as the surrounding
bodies do.

## Verification

* Extents checked against `work/dol/main.elf` with
  `node %TEMP%/opencode/ppc159.js work/dol/main.elf 0x802C3460` (96
  instructions) and `... 0x802C34B0` (64 instructions); the neighbouring
  `functions.tsv` sizes agree with the measured boundaries
  (`0x802C32B0` 432 ends at `0x802C3460`, `0x802C34B0` 92 ends at
  `0x802C350C`).
* Vtable and thunk words read straight out of the image: `0x80350228`,
  `0x8035022C`, `0x80350270`, `0x80350274`, `0x803501F8`, `0x80350204`
  (file offsets `0x34C788`, `0x34C78C`, `0x34C7D0`, `0x34C7D4`, `0x34C758`,
  `0x34C764`).
* `cargo check --lib` in a scratch copy of `src/` + `Cargo.toml` with the block
  appended to `src/resource/archive.rs`: exit 0, no diagnostic mentioning
  `archive.rs`.

## Deliverables

* `work/wave226/laneB_block.rs` - the four bodies with `# Original (untabled)`
  blocks.
* `ledger/agents/wave226-laneB.csv` - four rows, evidence
  `work/dol/main.elf` (there is no Ghidra export for any of them).
* This document.

Source files were not edited by this lane; the block is merged by the wave
step, and `node tools/check-lane-csv.mjs wave226` will only find the four
bodies once that merge has happened.
