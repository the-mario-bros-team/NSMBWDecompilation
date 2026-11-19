# Wave 154 - `game::bases` - the next eight (604-588 byte band)

Read-only specification lane. No Rust file, no `ledger/functions.csv` row and no
existing document was modified by this lane. The only file written is this one.
No merge or check tool was run. The throwaway tools live under
`%TEMP%\opencode\w154` (`pick.js`, `d.js`, `info.js`, `cx.js`, `led.js`,
`callers.js`, `which.js`, `md.js`, `verify.js`); none of them is in the
repository, and `d.js`/`md.js` here are a fresh decoder written by this lane
(the wave-150 `ppc.js` it was modelled on mis-orders the `neg` family, see the
decoder note). Every number below was recorded from this lane's own run.

Waves 128-153 walked this bucket from the top in size order; the last row
claimed by wave 153 is 604 bytes, and that document's verification section names
the next two candidates explicitly (`docs/SPEC_WAVE153_BASES.md:182-184`, "the
ninth is `0x8068D060` (604) and the tenth is `0x80799340` (604)"). This wave
starts one rung lower, with those two, and ends at 588.

```
0x8068D060 604 | 0x80799340 604 | 0x807005C0 600 | 0x8072B670 600
0x80613680 596 | 0x80733490 596 | 0x80746640 592 | 0x8079BF00 588
```

## The eight

| # | address | bytes | words | export file | verdict |
|--:|---:|---:|---:|---|:--:|
| 1 | `0x8068D060` | 604 | 151 | `8068d060_FUN_8068d060.c` | ready (high) |
| 2 | `0x80799340` | 604 | 151 | `80799340_FUN_80799340.c` | ready (medium) |
| 3 | `0x807005C0` | 600 | 150 | `807005c0_FUN_807005c0.c` | ready (medium) |
| 4 | `0x8072B670` | 600 | 150 | `8072b670_FUN_8072b670.c` | ready (high) |
| 5 | `0x80613680` | 596 | 149 | `80613680_FUN_80613680.c` | ready (medium) |
| 6 | `0x80733490` | 596 | 149 | `80733490_FUN_80733490.c` | ready (high) |
| 7 | `0x80746640` | 592 | 148 | `80746640_FUN_80746640.c` | ready (medium) |
| 8 | `0x8079BF00` | 588 | 147 | `8079bf00_FUN_8079bf00.c` | ready (medium) |

**Ready: 8. Conditional: 0.** 4,780 bytes / 1,195 words in total.

Every `bytes` value is the `size` column of
`work/ghidra/rel-d_basesNP/export/functions.tsv` verbatim and `decompiled` is
`true` for all eight. All eight extents end exactly on their own `blr`, so no
row needs the `docs/REL_ELF_MAPPING.md:37-42` extent correction:

| # | `blr` | `address + size - 4` | span is exact |
|--:|---|---|:--:|
| 1 | `0x8068D2B8` | `0x8068D2B8` | yes |
| 2 | `0x80799598` | `0x80799598` | yes |
| 3 | `0x80700814` | `0x80700814` | yes |
| 4 | `0x8072B8C4` | `0x8072B8C4` | yes |
| 5 | `0x806138D0` | `0x806138D0` | yes |
| 6 | `0x807336E0` | `0x807336E0` | yes |
| 7 | `0x8074688C` | `0x8074688C` | yes |
| 8 | `0x8079C148` | `0x8079C148` | yes |

Gaps after the eight extents (`next row - (address + size)`): `4`, `4`, `8`,
`8`, `28`, `172`, `0`, `4` bytes. The four-byte gaps are a single zero word. Row
3's 8 bytes are the two zero words `0x80700818`/`0x8070081C` (the next row,
`0x80700820`, is the translated `fn_80700820`). Row 5's 28-byte hole holds zero
words plus one stray `blr` at `0x806138E0`; row 6's 172-byte hole begins at
`0x807336E4` with zero words and then the untranslated delete body at
`0x807336F0` (`0x807336F0..0x80733758`), which is not part of this function.

The decoder reports **no unknown word** in the 1,195 words: zero unknown
opcodes, zero unhandled X-form, zero unhandled paired-single form. No `b`/`bc`
target leaves its own body; every `bl` goes to a named callee outside the body.
Only rows 3 and 7 contain `psq` words, and all ten/six of them are `qr0`
prologue spills of `f31`/`f30`/`f29`/`f28`/`f27` (or `f31`/`f30`/`f29`), i.e.
plain `f32` stack saves with no data-side quantisation:

| row | `psq` words |
|--:|---|
| 3 | 5 `psq_st` prologue spills + 5 `psq_l` epilogue reloads, all `W = 0`, `qr0` |
| 7 | 3 `psq_st` prologue spills + 3 `psq_l` epilogue reloads, all `W = 0`, `qr0` |
| 1, 2, 4, 5, 6, 8 | none |

No row reads or writes a `qr3` value, so the wave-138 `GQR3 = 0x00050005` pin is
not needed by anything in this set.

### Why all eight are ready

The verdict rule is wave 144's, unchanged:

* **ready** - every reachable instruction is decoded, every literal is read out
  of the image or out of `main.dol`, every callee is named by address (or by its
  ledger name), and the body can be written as code with no assumed calling
  convention beyond the standard PowerPC EABI. Data in `main.dol`
  (`0x800xxxxx`, `0x802xxxxx`, `0x803xxxxx`, `0x804xxxxx`), in this REL's data
  segments, or in this REL's BSS (`0x80822900..0x80834D84`) is an *input*, named
  by address.
* **conditional** - exactly one named input would still be missing.

Three rows deserve a sentence each, because a reader may expect a condition:

* Row 2's three callees `0x80774FA0`, `0x800F3330` and `0x8079B7C0` have no
  ledger row, and `0x8079B7C0` *is* decoded here (it is a two-position angle
  metric). `0x80774FA0`/`0x800F3330` are used only as "resolve record" and
  "resolve state id" and both stay address-only, which the ready rule allows.
  `docs/SPEC_WAVE140_BASES.md:1365` listed `0x80799340` itself as the missing
  *condition* of two other bodies; this document is that condition, closed.
* Row 7 dispatches through the `main.dol` trampoline `0x802DCBFC`. That
  trampoline is fully decoded below (nine instructions, `bctr`); no target list
  is missing, because the three-word descriptor that selects the target lives in
  this REL at `0x807CE410` and is quoted in full.
* Row 8's `0x80799B00` and `0x800F9530` are address-only, and the two engine
  predicates `0x800FA070`/`0x800FA0E0` are used only through their zero test
  (`countLeadingZeros(v) >> 5`), which the listing fixes exactly.

## Method

**Address mapping.** Re-derived from the program headers of
`work/rels/elf/d_basesNP.elf`, not copied:

```
PT_LOAD  p_offset  p_vaddr     filesz    memsz
  0      0x0010F0  0x806000F0  0x1C6004  0x1C6004
  1      0x001C70F4 0x807C60F4 0x0004FC  0x0004FC
  2      0x1C75F0  0x807C65F0  0x00000C  0x00000C
  3      0x1C7600  0x807C6600  0x00A5FC  0x00A5FC
  4      0x1D1C00  0x807D0C00  0x051CB8  0x051CB8
  5      0x000000  0x80822900  0x000000  0x012484 (zero fill)
```

so `file_offset = 0x10F0 + (address - 0x806000F0) = (address - 0x80600000) +
0x1000` for every code and data address below `0x807C60F4`, and segments 1-4
cover the literal pools in `0x807Cxxxx`/`0x807Dxxxx`. `0x80822900..0x80834D84`
is this REL's BSS and reads as zero at file level; `0x80832EB0` (rows 2, 8) and
`0x80830E20`/`0x80830E60` (row 6) are in it.

`main.dol`-resident data and code are read from `work/dol/main.elf` through
**its own** program headers (segment 3, `p_offset 0x002B00`, `p_vaddr
0x80006780`, covers all `0x800xxxxx`/`0x802xxxxx` code; segment 7, `p_offset
0x2FA900`, `p_vaddr 0x802FE3A0`), so `0x802DCBFC`, `0x802EF3A8`,
`0x8015F4F0` and `0x800C9xxx` all resolve.

**Calling-convention note used by rows 1, 2, 3, 5, 7, 8.** `0x802DCD64`,
`0x802DCD68`, `0x802DCD70` and `0x802DCD74` are MWCC `__savegprlr_N` helpers
(`_23`, `_24`, `_26`, `_27` respectively); they preserve `r3..r10` and return no
value. Ghidra prints them as `iVar = func_0x802dcd74()` and then uses that
"return" as `this`. The translation must take `this` from `r3` instead; where a
row also needs the *second* incoming register, it is `r4` (row 2 and row 8 both
keep `r4`/`r5` that way). The matching `__restgprlr_N` tails are `0x802DCDB0`
(`_23`), `0x802DCDB4` (`_24`), `0x802DCDBC` (`_26`) and `0x802DCDC0` (`_27`).

**Decoder note.** This lane wrote its own decoder (`%TEMP%\opencode\w154\d.js`)
instead of reusing `%TEMP%\opencode\w150\ppc.js`, because the older file prints
the `neg`/`addme`/`addze`/`subfme`/`subfze`/`mfcr` family as `reg,rA` while the
architecture puts the destination in bits 21-25. Row 6 contains two such words
(`0x80733624` = `neg r0,r3` and `0x80733644` = `neg r0,r0`, both from the
`-TILT` / `0x8000 - TILT` computation), and they are read with the destination
first. Numbers read from
`work/ghidra/rel-d_basesNP/export/functions.tsv` are copied verbatim; sizes are
never inferred from a `blr` position.

## Candidate selection (recomputed)

`%TEMP%\opencode\w154\pick.js` re-reads four sources from scratch:

* `work/ghidra/rel-d_basesNP/export/functions.tsv` - 5,615 data rows, all
  `decompiled = true`, and all 5,615 export files named in the `file` column
  exist on disk;
* `ledger/functions.csv` - 17,131 distinct `address` fields (case-insensitive);
* `src/game/bases.rs` - every `fn fn_<addr>` definition token (3,759 at the
  first selection pass, 3,772 at the verification pass; other lanes are editing
  the file concurrently, so this number drifts by a few per wave);
* every `#{2,4} ` heading of **all** 26 `docs/SPEC_WAVE*_BASES.md` files that
  existed before this one (270 distinct `0x` addresses). The task names
  `SPEC_WAVE12*_BASES.md`, of which only waves 128 and 129 exist; the superset
  is used so that no address already claimed by a wave-130..153 heading can be
  picked twice.

That leaves **1,828** rows at the first pass (1,815 once the 13 sibling bodies
added by other lanes during this session are excluded). The eight largest are the table above. Ties are
broken by **ascending address**; this is the only place the ordering is not
forced by size, and it is stated so wave 155 can reproduce it:

* the 604-byte pair is `{0x8068D060, 0x80799340}` - both named by wave 153;
* the 600-byte pair is `{0x807005C0, 0x8072B670}`;
* the 596-byte pair is `{0x80613680, 0x80733490}`;
* the 592-byte row is `0x80746640` alone, and the 588-byte row is
  `0x8079BF00` alone;
* the 584-byte group is `{0x80695E00, 0x8078F880, 0x807BAC40, 0x807BBE20}` - so
  **the ninth is `0x80695E00` (584)** and the tenth is `0x8078F880` (584), both
  fully decodable in the same session and neither in the ledger, the Rust file or
  any spec heading.

No `0x80695E00`/`0x8078F880` input is lost by deferring them; they are simply
the next two rows of the same band.

Three of the eight are already *referenced* (never defined) by doc comments in
`src/game/bases.rs`, and the citations below quote the comment text rather than
line numbers because other lanes are editing that file concurrently:

* `0x80799340` - `docs/SPEC_WAVE140_BASES.md` names it twice as the outstanding
  condition of the world-map state table (`0x59`) and of `0x8079D3C0`;
* `0x8068D060` - the `//!` block of the wave-97 `gabon_rock` cluster
  (`0x8068C030..0x8068D510`) lists every body around it
  (`fn_8068cf90`, `fn_8068d330`, `fn_8068d450`, `fn_8068d510`) but not this one;
* `0x80746640` - the `///` block of `fn_80747260` calls it "the placement
  builder `0x80746640`" and already links it by address.

A `fn fn_<addr>` body for any of the eight does not exist, so all pass the
filter.

---

## 1. `0x8068D060` - `gabon_rock` ground-contact / landing state body

**Evidence.** `work/ghidra/rel-d_basesNP/export/8068d060_FUN_8068d060.c`,
604 bytes, 151 words, synthetic `0x8068D060`. Body `0x8068D060..0x8068D2B8`
(ends `blr`, exactly one). No `psq`, no jump table, no indirect call; the 4-byte
gap after the extent is one zero word and the next row is the 88-byte
`0x8068D2C0`. Frame: `stwu r1,-64(r1)`, `__savegprlr_27` (`r27..r31`), epilogue
`__restgprlr_27`.

**What it is.** The ground-contact half of the `gabon_rock` `0x720`-byte actor
described by the wave-97 cluster block (`src/game/bases.rs`, "Wave 97, cluster 3:
`gabon_rock` state bodies (0x8068C030..0x8068D510)", vtable `0x807EF0B8`, class
string `gabon_rock` at `0x807EF010`, interface `+0x394`). It is called once per
frame while the thrown rock is rolling: it re-aims (`0x8068C450`), integrates the
two speeds, picks the spin direction from the speed sign, steps the two angles,
detects the ground transition, plays the landing sound pair, damps the roll, and
finally enters the landing state through `0x8068C140`. Its sibling
`fn_8068d450` is the same body without the contact half, and `fn_8068cf90` is the
respawn that seeds the fields this body consumes.

**Signature.**

```rust
/// Advances the rolling `gabon_rock` actor and reports the ground transition.
///
/// # Original
/// - Ghidra: `FUN_8068d060` at `0x8068D060` (604 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/8068d060_FUN_8068d060.c`
///
/// # Arguments
/// * `actor` - the `0x720`-byte actor; speeds `+0xE8`/`+0xEC`, angles
///   `+0x100`/`+0x102`, ground probe `+0x1EC`, side byte `+0x348`,
///   direction `+0x484`, ground height `+0x488`, aim band `+0x490`,
///   aim mode `+0x494`.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// The ground probe at `actor + 0x1EC` must be live and the sound manager cell
/// at `0x80429D60` must hold a live manager.
pub unsafe fn fn_8068d060(actor: usize)
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x000` | `u32` | object handle word, forwarded to `0x801995A0` |
| `+0x0AC`..`+0x0B4` | vec3 | position handed to `0x8006A4B0` |
| `+0x0E8` | `f32` | horizontal speed (drives the direction and the angle step) |
| `+0x0EC` | `f32` | vertical speed (damped by this body) |
| `+0x0F8` | `f32` | vertical floor, used inside `0x8006CDE0` |
| `+0x100` | `s16` | primary spin angle, stepped by `(s16)(896*abs(+0xE8))` |
| `+0x102` | `s16` | secondary angle, stepped toward the `+0x348`-selected target |
| `+0x114` | `f32` | vertical acceleration, used inside `0x8006CDE0` |
| `+0x1EC` | object | entry/collision record; `+0x88` status word, `+0x08` live word |
| `+0x348` | `u8` | side byte 0/1 (owner slot copied by `fn_8068cf90`) |
| `+0x484` | `u32` | spin direction: `0` = add, `1` = subtract |
| `+0x488` | `f32` | last ground height (written on the falling edge) |
| `+0x490` | `u32` | ground band `0..5`; `fn_8068c450` uses it as the aim-table index |
| `+0x494` | `u32` | aim mode; `2` when the band is 0, else the positive-height flag |

### Exact behaviour

```
0x8068C450(actor);                    // re-aim: +0xE8 -> +0xF4, +0x11C, +0x490/+0x494
0x8006CDE0(actor);                    // +0xEC += +0x114, clamped up to +0xF8
0x8006CD40(actor);                    // position step from the +0xE8 vector

// 1. spin direction from the horizontal speed sign (2-way, 0 is "neither")
if *(f32*)(actor + 0xE8) > 0.0 {
    *(u32*)(actor + 0x484) = 0;
} else if *(f32*)(actor + 0xE8) < 0.0 {
    *(u32*)(actor + 0x484) = 1;
}

// 2. aim the secondary angle at the side-dependent quarter turn
let side = *(u8*)(actor + 0x348);
0x8015F4F0(actor + 0x102, SIDE_ANGLE[side], 0x1C0);   // step 448 units/frame

// 3. primary angle step, floor 192
let mut step = (896.0f32 * *(f32*)(actor + 0xE8).abs()) as i32;
if (step as i16) < 192 { step = 192; }
if *(u8*)(actor + 0x348) as u32 == *(u32*)(actor + 0x484) {
    *(u16*)(actor + 0x100) = *(u16*)(actor + 0x100).wrapping_add(step as u16);
} else {
    *(u16*)(actor + 0x100) = *(u16*)(actor + 0x100).wrapping_sub(step as u16);
}

// 4. ground probes
let contact = 0x80070CB0(actor + 0x1EC);   // bits 13..20 of +0x88
let status  = 0x80072BC0(actor + 0x1EC);   // 0 when +0x08 is clear
if status == 0 {
    if contact != 0 {
        *(f32*)(actor + 0x488) = *(f32*)(actor + 0xB0);   // remember the ground
    }
} else {
    if contact == 0 {
        // first ground frame: smoke, then the landing sound
        0x8068C720(actor);
        let volume = 0x8068C400(actor);                    // (i32)(10*max(|+0xEC|,|+0xE8|))
        let mut sample = [0u32; 2];
        0x8006A4B0(sample.as_mut_ptr(), actor + 0xAC);
        abi_call5!(0x8019_8FC0,
                   *(u32*)0x8042_9D60, 0x2D8, sample.as_ptr() as u32,
                   volume, 0);
    }

    // 5. damp the roll by -0.5 and clap it to zero below 0.75
    *(f32*)(actor + 0xEC) *= -0.5;
    if *(f32*)(actor + 0xEC).abs() < 0.75 {
        *(f32*)(actor + 0xEC) = 0.0;
        let volume = 0x8068C400(actor);
        let mut sample = [0u32; 2];
        0x8006A4B0(sample.as_mut_ptr(), actor + 0xAC);
        abi_call6!(0x8019_95A0,
                   *(u32*)0x8042_9D60, 0x2D7, *(u32*)(actor + 0x000),
                   sample.as_ptr() as u32, volume, 0);
    }

    // 6. ground band -> aim band + aim mode
    *(u32*)(actor + 0x490) = 0x80070760(actor + 0x1EC);      // 0..5
    if *(u32*)(actor + 0x490) == 0 {
        *(u32*)(actor + 0x494) = 2;
    } else {
        *(u32*)(actor + 0x494) = 0x80070820(actor + 0x1EC,
                                            *(u8*)(actor + 0x484) as u32);
    }

    // 7. impact effect or iron spark
    if 0x8068BB70(actor) != 0 {
        0x8068BBF0(actor);
    } else if *(f32*)(actor + 0xE8) != 0.0 {
        0x8068C6B0(actor);                                   // Wm_en_iron
    }
}

// 8. enter the landing state when the contact record reports a hit
if 0x8006FA40(actor + 0x1EC, core::ptr::null_mut()) != 0 {
    0x8068C140(actor, *(u32*)(actor + 0x484) ^ 1);
}
```

Step 3's comparison is on the **sign-extended low half** (`extsh` then `cmpi
192`), so a step whose low 16 bits are negative keeps the 192 floor; writing
`if step < 192` on the full `i32` would differ once `896*|+0xE8|` reaches
`32768` (i.e. `|+0xE8| >= 36.57`), which the shipped speeds never reach but the
code allows. Step 5's `0.75` test is `fcmpo` on `fabs(frsp(+0xEC))`, i.e. single
precision.

`0x8015F4F0` is the translated `game::task` `step_short_clamped_wrap`
(`*v` steps toward the target by the step, clamps and returns 1 on arrival); the
side table is a two-entry `s16` array, so the target is `+0x4000` (90 degrees)
for side 0 and `0xC000` (-90 degrees) for side 1.

### Constants

| address | value | use |
|---|---|---|
| `0x807C9A70` | `0.0f` | the three "is zero" tests |
| `0x807C9BA4` | `s16[2] = {0x4000, 0xC000}` | `+0x102` target by side byte |
| `0x807C9BA8` | `896.0f` | primary angle step scale |
| `0x807C9BAC` | `-0.5f` | roll damping |
| `0x807C9BB0` | `0.75f` | roll stop threshold |
| `0x8042_9D60` | cell | sound manager (`*(u32*)0x80429D60`) |
| - | `0x1C0` (448) | `0x8015F4F0` step |
| - | `192` (0xC0) | primary angle step floor |
| - | `0x2D8`, `0x2D7` | sound ids (landing, roll stop) |

The `+0x102` table is exactly 4 bytes: `0x807C9BA8` (the `896.0f` above) starts
one word later, so `+0x348` can only be 0 or 1; `fn_8068cf90` and `fn_8068d450`
read the same byte against the same two-entry tables at `0x807C9B98`/`0x807C9AE0`.

### Callees

`0x802DCD74`/`0x802DCDC0` (`__savegprlr_27`/`__restgprlr_27`), `0x8068C450`
(`fn_8068c450`, aim rebuild), `0x8006CDE0` (`+0xEC += +0x114`, clamp),
`0x8006CD40` (position step `0x8006CD00(actor, actor+0xE8)`), `0x8015F4F0`
(`fn_8015f4f0`, short step), `0x80070CB0` (`fn_80070cb0`), `0x80072BC0`
(`fn_80072bc0`), `0x8068C720` (`fn_8068c720`), `0x8068C400` (`fn_8068c400`),
`0x8006A4B0` (transform sample used by the whole cluster), `0x80198FC0`/
`0x801995A0` (5-/6-argument SE spawns, both called identically in
`fn_8068c140`), `0x80070760` (`fn_80070760`), `0x80070820` (`fn_80070820`),
`0x8068BB70`/`0x8068BBF0` (`fn_8068bb70`/`fn_8068bbf0`), `0x8068C6B0`
(`fn_8068c6b0`), `0x8006FA40` (contact query), `0x8068C140` (`fn_8068c140`).

### Notes for the implementer

* `this` is not an argument: `__savegprlr_27` preserves `r3`. Take it from `r3`.
* Steps 1-3 run **unconditionally**; only steps 4-8 are gated by the contact
  status. Do not fold step 5 into step 7.
* `0x80070CB0` returns "any of bits 13..20 of `+0x88` set", `0x80072BC0` returns
  the status word of `0x8006DB30` and `0` when `+0x08` is clear. `status != 0`
  therefore means "the record is live"; `contact == 0` inside it means "the
  first frame of contact".
* The `+0x488` store is a plain `f32` copy of `+0xB0`, not a delta.
* The second sound call (`0x2D7`) is the only place this family passes
  `*(u32*)(actor + 0x000)` as the third argument; `0x80689C10` and `0x80706510`
  pass their own `+0x000` word the same way, so keep the field read.
* `0x8068C140(actor, dir ^ 1)` receives the *inverted* direction word; that is
  the landing owner handle parameter, not a boolean.

**Verdict: ready (high).** All 151 words decoded, every field and constant read
from the image, every callee named (the two audio spawns by address exactly as
`fn_8068c140` already does), and the two soft spots (the 16-bit step floor and
the `f32` damping) are spelled out.

---

## 2. `0x80799340` - world-map course-node resolver

**Evidence.** `work/ghidra/rel-d_basesNP/export/80799340_FUN_80799340.c`,
604 bytes, 151 words, synthetic `0x80799340`. Body `0x80799340..0x80799598`
(ends `blr`, exactly one; the next row is the 20-byte `0x807995A0` and the 4-byte
gap is one zero word). No `psq`, no jump table. Frame: `stwu r1,-48(r1)`,
`__savegprlr_24`, epilogue `__restgprlr_24`. Three `bl` callers exist in the
image: `0x807968EC` (inside `0x80796850`, 448 bytes), `0x80798090` (inside
`0x80797D30`, 960) and `0x8079DC00` (inside `0x8079D3C0`, 3,176).

**What it is.** The "install the adjacent course node" step of the world-map
actor family that covers `0x80797810`..`0x8079E030` (the same family as
`fn_80772280`/`fn_80772370`'s tree queries, `0x8079A040`'s course-id selector and
`0x8079BDF0`'s `0x80354E60` slot walk). It walks the six child slots twice -
once through the crossing test `0x80772370`, once through the plain query
`0x80772280` - and, for the first slot whose resolved record id differs from the
actor's current id, moves the actor onto that node: it stores the record, swaps
the id pair through `0x800F3330`, optionally snaps the position, applies the
state through `0x80799200` and publishes the node id to the shared BSS word
`0x80832EB0`. All three callers ignore the return value and reload `+0x23C`
instead (see the note below), so the *output* of this body is `+0x230`/`+0x23C`
plus `0x80832EB0`.

**Signature.**

```rust
/// Moves the world-map actor onto the first adjacent course node whose state
/// differs from the current one.
///
/// # Original
/// - Ghidra: `FUN_80799340` at `0x80799340` (604 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/80799340_FUN_80799340.c`
///
/// # Arguments
/// * `this` - the world-map node actor; node id `+0x22C`, record `+0x230`,
///   current state `+0x23C`, position `+0xAC..+0xB4`.
/// * `position` - the snapshot to install when the angle gate fires.
///
/// # Returns
/// The **displaced** state id (the first `0x800F3330` result), or `6` when no
/// slot matched. Every caller discards it and re-reads `+0x23C`.
///
/// # Safety
/// The scene manager cell `0x8042A18C` must be live and `this + 0x22C` must be
/// a valid node id for its `+0x338C` sub-array.
pub unsafe fn fn_80799340(this: usize, position: *const [f32; 3]) -> u32
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x0AC`..`+0x0B4` | vec3 | position, overwritten from `position` when the gate fires |
| `+0x22C` | `u32` | node/name id used by all three resolvers and published to BSS |
| `+0x230` | `u32` | resolved record pointer (installed before the id swap) |
| `+0x23C` | `u32` | current state id; compared before the install and rewritten after |

Globals: `0x8042A18C` (scene manager pointer cell), `0x80832EB0` (this REL's BSS
word, "last published node id"), `0x807D00C4` and `0x807D01C4` (the two floats).

### Exact behaviour

```
let scene = *(u32*)0x8042_A18C as usize;
let current = *(u32*)(this + 0x23C);
let container = scene + (*(u32*)(scene + 0x338C) as usize) * 0x3F08 + 0x5C7F4;

// pass 1: crossing test, pass 2: plain query
for pass in 0..2 {
    for slot in 0..6 {
        let hit = if pass == 0 {
            0x80772370(scene, *(u32*)(this + 0x22C), slot)   // three-pass crossing test
        } else {
            0x80772280(scene, *(u32*)(this + 0x22C), slot)   // key lookup
        };
        if hit == 0 { continue; }

        let record = 0x80774FA0(scene, *(u32*)(this + 0x22C), slot);
        let id = 0x800F3330(container, record, *(u32*)(this + 0x22C));
        if id == current { continue; }                       // already there

        *(u32*)(this + 0x230) = record;
        let new_id = 0x800F3330(container, *(u32*)(this + 0x22C), record);

        let angle = 0x8079B7C0(this);                        // f64 cosine of the node angle
        if 0.05 < ((angle.abs() as f32).abs() - 1.0).abs() {
            *(f32*)(this + 0x0AC) = (*position)[0];
            *(f32*)(this + 0x0B0) = (*position)[1];
            *(f32*)(this + 0x0B4) = (*position)[2];
        }

        0x80799200(this, new_id, 1);
        *(u32*)(this + 0x23C) = new_id;
        *(u32*)0x8083_2EB0 = *(u32*)(this + 0x22C);
        return id;                                          // note: the OLD id
    }
}
6
```

The two `0x800F3330` calls take their last two arguments in **opposite order**
(`record, node_id` then `node_id, record`); that is visible at `0x807993B4` and
`0x807993E4` and is not a transcription slip. The second result is what
`0x80799200` receives and what `+0x23C` becomes.

`0x8079B7C0` is decoded: it returns the **cosine** of the angle between the node
`0x80832EB0` and the node `+0x22C` (both positions through `0x80100200`), i.e.
`dot / (|a| * |b|)`, computed as `f32` through `0x801C14C0` (the magnitude
helper) and four `fsqrt`/divide steps; it returns `DAT_807D0160` (`0.0`) when
either endpoint is the origin triple `0x80377DC8..0x80377DD0`, when
`0x80832EB0 == +0x22C` or `+0x230 == +0x22C`, and `0.0` again when the `bVar8`
early-out fires. The `f32` narrowing before the `fabs` is the `frsp` at
`0x80799408`, and the `1.0` it is compared against is the "parallel" value, so
the snap fires whenever the two directions are **not** within `0.05` of
collinear - including the degenerate `0.0` returns.

### Constants

| address | value | use |
|---|---|---|
| `0x807D00C4` | `1.0f` | cosine reference ("still parallel") |
| `0x807D01C4` | `0.05f` | angle dead band |
| `0x8042_A18C` | cell | scene manager pointer |
| `0x8083_2EB0` | BSS word | published node id (zero at file level) |
| - | `6` | "no slot matched" return |
| - | `0x338C`, `0x3F08`, `0x5C7F4` | scene sub-array index, stride and base |

### Callees

`0x802DCD68`/`0x802DCDB4` (`__savegprlr_24`/`__restgprlr_24`), `0x80772370`
(`fn_80772370`), `0x80772280` (`fn_80772280`), `0x80774FA0` (address-only,
"resolve record for `(scene, node, slot)`"), `0x800F3330` (address-only,
"resolve state id"; the `0x800F` family is the world-map container API),
`0x8079B7C0` (angle metric, decoded above), `0x80799200` (address-only;
its export calls `0x80798D40`, `0x80798E20` and `0x80798F10`, stores the result
in `+0x2E0`, walks the `+0x184` child chain through `0x80780F50` and toggles one
`0x80C561E0` flag after an `0x800F57A0` test).

### Notes for the implementer

* `this` comes from `r3` (the save helper preserves it); `position` arrives in
  `r4` and is saved in `r31` before the first call.
* `+0x22C` is read three times per iteration and once at the end; a Rust
  translation should cache it only if the callees cannot write it, which is not
  provable from this REL. Keep the reloads.
* The six-slot bound is a literal `6` and there is **no** bound check on
  `+0x338C`; `fn_80772280` documents the same contract.
* The position snap is a plain 12-byte copy of `*(f32*)[3]`; the export's
  `puVar7[1]`/`puVar7[2]` are the second and third words of `r4`.
* Return the *first* `0x800F3330` result. All three callers ignore `r3`, but a
  faithful body must not return `new_id`.

**Verdict: ready (medium).** Complete control flow, both float constants and the
BSS word are read; medium only because `0x80774FA0`/`0x800F3330`/`0x80799200` stay
address-only and the two `0x800F3330` argument orders are unusual enough to
warrant a second pair of eyes.

---

## 3. `0x807005C0` - four-node `Wm_jr_electric*` emitter

**Evidence.** `work/ghidra/rel-d_basesNP/export/807005c0_FUN_807005c0.c`,
600 bytes, 150 words, synthetic `0x807005C0`. Body `0x807005C0..0x80700814`
(ends `blr`, exactly one; the 8-byte gap is two zero words and the next row is
the translated `fn_80700820`). Frame: `stwu r1,-160(r1)`,
`__savegprlr_24`, `f27..f31` saved as `stfd` + `psq_st` pairs. No branch leaves
the body; the four `bctrl` sites are the `+0x14` virtual of `+0x8C4`
(`0x80700620`) and the three `+0xA0` element virtuals (`0x80700754`,
`0x8070077C`, `0x807007BC`).

**What it is.** The per-frame companion of `fn_80700820` in the same
`0xE18`-byte owner (the wave-50 window `0x807001C0..0x80702520`, whose constructors
`fn_80701820`/`fn_80702750` and per-frame bodies `0x80702E40` etc. are already
translated). `fn_80700820` resets the owner's four-entry arrays at `+0x8FC`,
`+0x192C` and `+0x295C` plus the object at `+0x8C4`; this body *drives* the same
four entries, emitting the three effects whose names sit at `0x80803A84`,
`0x80803A98` and `0x80803AAC`:

```
"Wm_jr_electricglow"   0x80803A84
"Wm_jr_electricspark"  0x80803A98
"Wm_jr_electricline"   0x80803AAC
```

and playing SE `0x5B3` once per frame through `0x80199390`.

**Signature.**

```rust
/// Commits the electric-line host and re-emits the three effects on its four
/// nodes.
///
/// # Original
/// - Ghidra: `FUN_807005c0` at `0x807005C0` (600 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/807005c0_FUN_807005c0.c`
///
/// # Arguments
/// This is not an argument: `__savegprlr_24` preserves `r3`, the `0xE18`-byte
/// owner. Position `+0xAC..+0xB4`, side byte `+0x348`, host `+0x8C4`, the three
/// `0x128`-stride arrays at `+0x8FC`/`+0x192C`/`+0x295C`.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// The host at `+0x8C4` and all twelve array elements must expose a valid
/// vtable with slot `+0xA0` (and `+0x14` for the host).
pub unsafe fn fn_807005c0()
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x0AC`..`+0x0B4` | vec3 | base position of the four nodes |
| `+0x348` | `u8` | slot byte, indexes the `main.dol` per-player byte table |
| `+0x8C4` | object | host object; its vtable `+0x14` is invoked once (commit) |
| `+0x8FC + i*0x128` | object | "glow" element (i = 0..3) |
| `+0x192C + i*0x128` | object | "spark" element (i = 0..3) |
| `+0x295C + i*0x128` | object | "line" element (i = 1..3; slot 0 skipped) |

The same four-entry/`0x128`-stride layout is documented in
`fn_80700820`'s `///` block ("four entries of the three arrays at `+0x8FC`,
`+0x192C` and `+0x295C` in lock-step; the third array is reset from the second
entry onwards").

### Exact behaviour

```
let base = *(f32*)(this + 0xAC);
let y    = *(f32*)(this + 0xB0);
let z    = *(f32*)(this + 0xB4);

// 1. commit the host (no arguments beyond the object itself)
let host_vt = *(u32*)(this + 0x8C4);
(*(host_vt + 0x14))(this + 0x8C4);

// 2. one positional SE per frame
let mut sample = [0u32; 2];
0x8006A4B0(sample.as_mut_ptr(), this + 0xAC);
abi_call5!(0x8019_9390, *(u32*)0x8042_9D60, 0x5B3,
           *(u32*)(this + 0x000), sample.as_ptr() as u32, 0);

// 3. pre-computed row geometry
let top   = 8.0 + (y - 112.0);            // y - 104
let div   = if 1.0f32 < 3.0f32 { 3.0f32 } else { 1.0f32 };   // the inline 3.0
let step  = 208.0 / div;                  // 69.3333
let node_x = base + (*(i8*)(0x8042_C1A0 + *(u8*)(this + 0x348)) as f32) * 13.5;

// 4. four nodes
for i in 0..4 {
    let mut v = [node_x, top + (i as f32) * step, z];
    emit(this + 0x8FC  + i * 0x128, 0x80803A84);   // "Wm_jr_electricglow"
    emit(this + 0x192C + i * 0x128, 0x80803A98);   // "Wm_jr_electricspark"
    if i > 0 {
        v[1] -= step * 0.5;                        // 34.6667
        emit(this + 0x295C + i * 0x128, 0x80803AAC);   // "Wm_jr_electricline"
    }
}

// "emit" is the +0xA0 slot of the element's vtable:
//   let f = ptr_at(ptr_at(element) + 0xA0);
//   abi_call6!(f, element, name_ptr, 0, v.as_ptr() as u32, 0, 0);
```

The `float -> int -> float` round trip at `0x80700658..0x80700694` is the
`0x43300000`/`xoris 0x8000` idiom for the constant `3.0`; it is *not* a
parameter. The `8.0`/`112.0` pair and the `208.0`/`0.5` pair are the only
geometry constants; the four node heights are therefore `-104`, `-34.67`,
`+34.67` and `+104` relative to `y`.

`0x8006A4B0` is the cluster's transform sample (same call shape as
`fn_8068c140`); here it is fed `this + 0xAC` and the 8-byte result is passed as
the fourth argument of `0x80199390`.

### Constants

| address | value | use |
|---|---|---|
| `0x807CC4AC` | `13.5f` | per-slot x offset scale |
| `0x807CC4B8` | `4503599627370496.0` (`2^52`) | `u32/i32 -> double` bias |
| `0x807CC4C0` | `1.0f` | lower bound of the `max` |
| `0x807CC4D0` | `8.0f` | node-row base offset |
| `0x807CC4D4` | `112.0f` | base-position subtraction |
| `0x807CC4D8` | `208.0f` | full span of the four nodes |
| `0x807CC4E0` | `0.5f` | line drop (half a node gap) |
| `0x80803A84` | string | `Wm_jr_electricglow` |
| `0x80803A98` | string | `Wm_jr_electricspark` |
| `0x80803AAC` | string | `Wm_jr_electricline` |
| `0x8042_C1A0` | `s8[]` | `main.dol` per-player character byte table |
| `0x8042_9D60` | cell | sound manager |
| - | `3.0` | inline literal, divisor of the `208.0` span |
| - | `0x5B3` (1459) | sound id |

`0x807CC4A0` itself holds the integer `4` (the loop bound, materialised as a
compare) and `0x807CC4A4`/`0x807CC4A8` (`1.0f`, `0.3f`) belong to the
neighbouring bodies, not to this one.

### Callees

`0x802DCD68`/`0x802DCDB4` (`__savegprlr_24`/`__restgprlr_24`), `0x8006A4B0`,
`0x80199390` (the 5-argument SE spawn, `(manager, id, object, position, 0)`),
the host vtable slot `+0x14` of `+0x8C4`, and the element vtable slot `+0xA0` of
all twelve elements (same slot and argument shape as `fn_8068c6b0`'s
"emit named effect at position": `(obj, name, 0, position, 0, 0)`).

### Notes for the implementer

* The three vtable dispatches use `lwz r12,0(obj)` + `lwz r12,0xA0(r12)` +
  `mtspr ctr` + `bctrl`; the argument registers are `r4 = name pointer`,
  `r5 = 0`, `r6 = &vec3`, `r7 = 0`, `r8 = 0`.
* The `+0x8C4` host call is `(*(*(u32*)(this + 0x8C4) + 0x14))(this + 0x8C4)`
  with no other argument registers set; do **not** reuse the vector.
* The name pointers are *not* `lis`/`addi` of a base here: `r28 = 0x80803A10`
  and the code adds `0x74`/`0x88`/`0x9C`. The three addresses above are the
  absolute results.
* `*(u32*)(this + 0x000)` is the object handle handed to the SE call, exactly as
  in `0x80706510`/`0x80689C10`; it is not a vtable.
* The vector is rebuilt from `+0xAC`/`+0xB0`/`+0xB4` every iteration; the `line`
  arm modifies the *local* copy only.

**Verdict: ready (medium).** All 150 words decoded, both tables and all six
geometry constants read, and the effect API matches an already-translated
sibling. Medium only because `0x80199390` and `0x8006A4B0` stay address-only and
the `+0x8C4` host slot is inferred from its single call site.

---

## 4. `0x8072B670` - rotating fire cannon create body

**Evidence.** `work/ghidra/rel-d_basesNP/export/8072b670_FUN_8072b670.c`,
600 bytes, 150 words, synthetic `0x8072B670`. Body `0x8072B670..0x8072B8C4`
(ends `blr`, exactly one; the 8-byte gap is two zero words and the next row is
the untranslated 320-byte `0x8072B8D0`, which this body calls first). Frame:
`stwu r1,-32(r1)` with manual `stw r31`/`stw r30`. Returns `1`.

**What it is.** The create/start callback of the rotating fire cannon whose
finish bodies live at `0x8072BA10..0x8072BF60` and whose model is bound by
`fn_8072c180`/`fn_8072c200` (`fire_rot_cannon_gun`, `Wm_en_fireburner*`). The
cluster documentation already names its fields: "position words `+0x4B4..+0x4BC`,
angle `+0x4A0`, flip word `+0x4D0`, flag word `+4`, effect objects `+0x1374` and
`+0x149C`". This body reads two bit fields of the parameter word `+0x04`
(variant `& 7`, flip `>> 4 & 1`), clears the gun angle, sizes the two motion
records, registers the three callbacks `0x8073BF40`/`0x8073BF30`/`0x8073BF50`,
initialises the model/clip pair and then dispatches the interface state
`0x80830918`.

**Signature.**

```rust
/// Starts the rotating fire cannon: decodes the variant/flip bits, sizes the
/// two motion records and dispatches the initial state.
///
/// # Original
/// - Ghidra: `FUN_8072b670` at `0x8072B670` (600 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/8072b670_FUN_8072b670.c`
///
/// # Arguments
/// * `actor` - the fire cannon; flags `+0x04`, scale `+0xDC..+0xE4`,
///   position `+0xB4`, angle `+0x104`, motion records `+0x11A4` and
///   `+0x1288`, callbacks `+0x12D4..+0x12DC`, model `+0x3F0`, clip `+0x474`,
///   interface `+0x394`.
///
/// # Returns
/// `1` - create success.
pub unsafe fn fn_8072b670(actor: usize) -> u32
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x004` | `u32` | parameters: `&7` variant, `>>4 & 1` flip |
| `+0x0B4` | `f32` | far z plane, set to `1900.0` (`+0xB0` is not touched) |
| `+0x0D0`/`+0x0D4`/`+0x0D8` | `f32` | `0.0` / `8.0` / `0.0` |
| `+0x0DC`..`+0x0E4` | vec3 | scale, `1.0` |
| `+0x104` | `s16` | facing angle, `0` or `-0x8000` |
| `+0x348` | `u8` | set to `1` |
| `+0x3F0` | model | NW4R model holder |
| `+0x394` | object | interface; vtable slot `+0x18` receives `0x80830918` |
| `+0x430` | clip | animation holder |
| `+0x474` | element | clip element (`0x80167B60`/`0x80169F80`) |
| `+0x4A0` | `s16` | gun angle, cleared |
| `+0x4C0`/`+0x4C4`/`+0x4C8` | `f32` | `1.0` / size factor / `1.0` |
| `+0x4CC` | `s32` | collision record count (set by `0x8072BF60`) |
| `+0x4D0` | `u32` | flip word, `(flags >> 4) & 1` |
| `+0x11A4`, `+0x1288` | element | the two `0x2290`-byte motion records |
| `+0x12D4`/`+0x12D8`/`+0x12DC` | fnptr | `0x8073BF40` / `0x8073BF30` / `0x8073BF50` |

### Exact behaviour

```
0x8072B8D0(actor);                        // sibling defaults pass (320 bytes)

let flags = *(u32*)(actor + 0x04);
let n     = flags & 0x7;
let flip  = (flags >> 4) & 0x1;

*(s16*)(actor + 0x4A0) = 0;
*(u8*)(actor + 0x348)  = 1;

let one = 1.0f;                           // 0x807CD46C
*(f32*)(actor + 0xDC) = one;
*(f32*)(actor + 0xE0) = one;
*(f32*)(actor + 0xE4) = one;

// 16 * (4 + n), clamped to 128, then / 64 -> 1.0 .. 2.0
let mut size = 16.0 * ((n + 4) as f32);
if size > 128.0 { size = 128.0; }
size *= 0.015625;                         // 1/64

*(f32*)(actor + 0x4C0) = 1.0;
*(f32*)(actor + 0x4C4) = size;
*(f32*)(actor + 0x4C8) = 1.0;
*(f32*)(actor + 0xB4)  = 1900.0;
*(u32*)(actor + 0x4D0) = flip;

if flip == 0 {
    *(s16*)(actor + 0x104) = 0;
    *(f32*)(actor + 0xD0) = 0.0;
    *(f32*)(actor + 0xD4) = 8.0;
    *(f32*)(actor + 0xD8) = 0.0;
    // element 1: (-8, 16, 8, 0), one instance, three null callbacks
    0x8007FA40(actor + 0x11A4, actor, -8.0, 16.0, 8.0, 0.0, 0, 0, 0, 1, 0, 0);
    // element 2: (-8, 8, 8, -8)
    0x8007FA40(actor + 0x1288, actor, -8.0,  8.0, 8.0, -8.0, 0, 0, 0, 1, 0, 0);
} else {
    *(s16*)(actor + 0x104) = -0x8000;
    *(f32*)(actor + 0xD0) = 0.0;
    *(f32*)(actor + 0xD4) = 8.0;
    *(f32*)(actor + 0xD8) = 0.0;
    0x8007FA40(actor + 0x11A4, actor, -8.0, 16.0, 8.0, 0.0, 0, 0, 0, 1, 0, 0);
    0x8007FA40(actor + 0x1288, actor, -8.0, 24.0, 8.0, 8.0, 0, 0, 0, 1, 0, 0);
}

0x8072BF60(actor);                        // rebuild the 0xA4-stride collision array
0x8007F900(actor + 0x11A4);
0x8007FD80(actor + 0x1288, actor + 0x4A0);
0x8007F900(actor + 0x1288);

*(u32*)(actor + 0x12D4) = 0x8073BF40;
*(u32*)(actor + 0x12D8) = 0x8073BF30;
*(u32*)(actor + 0x12DC) = 0x8073BF50;

0x8016A000(actor + 0x3F0, actor + 0x136C);
0x80167B60(actor + 0x474, 0, 0);
0x80169F80(1.0, actor + 0x430, actor + 0x474);    // f1 = lfs 36(r31) = 0x807CD46C
vmethod1(actor + 0x394, 0x18, 0x80830918);
1
```

Two details the listing settles and a careless reader gets wrong:

* The two `0x8007FA40` calls in the `flip == 0` arm are **not** identical:
  the second is `(-8, 8, 8, -8)` while the flipped arm's second is
  `(-8, 24, 8, 8)`. The first call is identical in both arms.
* `0x80169F80`'s frame argument is loaded by `lfs f1,36(r31)` at `0x8072B880`,
  i.e. `0x807CD46C = 1.0f`; the export prints it as `(double)DAT_807cd46c`.
  The clip is therefore set to frame `1.0`, not `0.0`.

`0x8072B8D0` (the first call) is the untranslated 320-byte sibling; it is
address-only, which the ready rule allows. The `+0x136C` argument of `0x8016A000`
is the block 0x1C8 bytes past the first motion record (`0x136C - 0x11A4`), which
is the model's own animation/material container.

### Constants

| address | value | use |
|---|---|---|
| `0x807CD46C` | `1.0f` | scale triple and the clip frame |
| `0x807CD470` | `16.0f` | size base |
| `0x807CD474` | `128.0f` | size clamp |
| `0x807CD478` | `0.015625f` (`1/64`) | size normaliser |
| `0x807CD47C` | `1900.0f` | `+0xB4` |
| `0x807CD480` | `0.0f` | `+0xD0`/`+0xD8` |
| `0x807CD484` | `8.0f` | `+0xD4`, third range float |
| `0x807CD488` | `-8.0f` | first range float |
| `0x807CD48C` | `24.0f` | flipped-arm second range float |
| `0x807CD490` | `2^52` | `u32 -> double` bias for the size factor |
| `0x8083_0918` | BSS descriptor | interface `+0x18` argument |
| `0x8073BF30`/`0x8073BF40`/`0x8073BF50` | code | the three callbacks |
| - | `0x8000` | flipped facing angle |
| - | `1` | return value |

### Callees

`0x8072B8D0` (address-only sibling), `0x8007FA40` (the element/path constructor
already documented in `docs/SPEC_WAVE153_BASES.md` §6: `(block, owner, f1..f4,
cb1, cb2, cb3, count, 0, 0)`), `0x8072BF60` (`fn_8072bf60`), `0x8007F900`
(register element), `0x8007FD80` (attach the `+0x4A0` angle block), `0x8016A000`
(model/clip bind), `0x80167B60` (element reset), `0x80169F80` (clip frame), the
interface vtable slot `+0x18` of `+0x394`.

### Notes for the implementer

* The size factor is `min(16*(4 + (flags & 7)), 128) / 64`, i.e. `1.0` for
  variant 0 and `2.0` for variants 4-7. The `mulli`/`xoris` pair at
  `0x8072B6A4..0x8072B6C8` is the unsigned `n + 4` conversion, not a byte field.
* `+0x4D0` is `(flags >> 4) & 1` (`rlwinm r0,r0,28,31,31`).
* The three callback words are written **after** `0x8072BF60`, so the collision
  rebuild sees the old (null) slots; keep that order.
* `0x8007FA40` takes the four floats in `f1..f4` and the six integers in
  `r5..r10`; the Ghidra call prints them in that same order after `r3`/`r4`.
* `0x80169F80` is declared `(f32, modelOrClip, clip)` in the wave-97 cluster
  (`fn_8068c820`), matching `f1 = 1.0`, `r4 = +0x430`, `r5 = +0x474`.

**Verdict: ready (high).** Every operand is a literal or an image constant, all
eight call targets are named, and the two divergent arms are tabulated above.

---

## 5. `0x80613680` - `dEasyPairing_c` four-slot panel advance

**Evidence.** `work/ghidra/rel-d_basesNP/export/80613680_FUN_80613680.c`,
596 bytes, 149 words, synthetic `0x80613680`. Body `0x80613680..0x806138D0`
(ends `blr`, exactly one; the 28-byte gap holds zero words and one stray `blr` at
`0x806138E0`). No `psq`. Frame: `stwu r1,-48(r1)`, `__savegprlr_23`, epilogue
`__restgprlr_23`.

**What it is.** The per-frame slot driver of the `dEasyPairing_c` screen family
(descriptors `0x80823D30..0x80823EDC`, registered by `fn_80613ca0`; the
neighbouring `0x80612740` registers the `dDrawGame_c` descriptors
`0x80823C30/3C70/3CB0/3CF0`). It walks four panel slots, drives each slot's
dDraw channel through `0x80613680`'s helpers (`0x800C9490` stop, `0x800C92F0`
start, `0x800C9610` flag query, `0x800C93D0` start-and-hold), plays the
per-slot SE pair `0x86`/`0x87` … `0x8C`/`0x8D`, and when no slot changed it
forwards the state `0x80823E30` (`dEasyPairing_c`'s fifth descriptor,
`StopStartWait`) after testing the current player record's input bit `0x10`.

**Signature.**

```rust
/// Advances the four panel slots of the pairing screen.
///
/// # Original
/// - Ghidra: `FUN_80613680` at `0x80613680` (596 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/80613680_FUN_80613680.c`
///
/// # Arguments
/// * `this` - the screen object; dDraw set `+0x70`, interface `+0x208`,
///   per-slot arrays `+0x248`, `+0x268`, `+0x27C`, `+0x28C`, `+0x29C`.
///
/// # Returns
/// Nothing.
pub unsafe fn fn_80613680(this: usize)
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x070` | dDraw set | channel table `+0x184`, flag array `+0x188`, cursor `+0x194` |
| `+0x208` | object | interface; vtable slot `+0x18` receives the state descriptor |
| `+0x248 + i*4` | ptr | per-slot object; bit 0 of its `+0xBB` byte is set on resolve |
| `+0x268 + i*4` | ptr | per-slot object; its `+0xB8` byte is cleared on start |
| `+0x27C + i*4` | `s32` | slot state: `0` idle, `1` starting, `2` running, `3` stopping |
| `+0x28C + i*4` | `s32` | dDraw channel index for slot `i` (from `0x807C6AD0`), `10` = none |
| `+0x29C + i*4` | `s32` | sprite/entry index for slot `i` (from `0x807C6AB8`), `10` = none |

### Exact behaviour

```
let mut changed = false;
for i in 0..4 {
    let slot = this + i * 4;
    let state = *(s32*)(slot + 0x27C);

    if state == 1 {
        *(s32*)(slot + 0x27C) = 2;
        if *(s32*)(slot + 0x29C) != 10 {
            0x800C9490(this + 0x70, *(s32*)(slot + 0x29C));
            *(s32*)(slot + 0x29C) = 10;
        }
        *(u8*)(*(u32*)(slot + 0x268) + 0xB8) = 0;
        *(s32*)(slot + 0x28C) = CHANNEL[i];              // 0x807C6AD0
        let arg = 0x80069530(i);                         // 0x80069520()==0 ? 2<<i : 0
        abi_call3!(0x8019_5380, *(u32*)0x8042_A488, 0x86 + 2*i, 1);
        abi_call3!(0x8019_5380, *(u32*)0x8042_A488, 0x87 + 2*i, arg);
        0x800C92F0(this + 0x70, *(s32*)(slot + 0x28C), 0);
        changed = true;
    } else {
        // the "channel still busy" test is on the *channel* word, not the state
        if (*(u32*)(slot + 0x28C)).wrapping_sub(5) <= 3 {
            if 0x800C9610(this + 0x70, *(s32*)(slot + 0x28C)) != 0 {
                changed = true;
            } else {
                *(s32*)(slot + 0x28C) = 10;
            }
        }

        if state == 3 {
            if *(s32*)(slot + 0x28C) != 10 {
                0x800C9490(this + 0x70, *(s32*)(slot + 0x28C));
            }
            *(s32*)(slot + 0x27C) = 0;
            0x80613290(this, i);                          // pick the next sprite
            *(s32*)(slot + 0x28C) = CHANNEL[i];
            0x800C93D0(this + 0x70, CHANNEL[i], 1);
            let obj = *(u32*)(slot + 0x248);
            *(u8*)(obj + 0xBB) = (*(u8*)(obj + 0xBB) & 0xFE) | 1;
            changed = true;
        }
    }
}

if !changed {
    let frame_array = *(u32*)0x8042_9F50 as usize;
    let player      = *(u32*)0x8042_A464 as usize;
    let record      = *(u32*)(frame_array + player * 4 + 4) as usize;
    if *(u32*)(record + 0x1C) & 0x10 != 0 {
        vmethod1(this + 0x208, 0x18, 0x80823E30);
    }
}
```

The two `0x80195380` calls are the *same* pair in every arm except for the id and
the second argument: slot 0 sends `(0x86, 1)` / `(0x87, v)`, slot 1 `(0x88,
1)` / `(0x89, v)`, slot 2 `(0x8A, 1)` / `(0x8B, v)`, slot 3 `(0x8C, 1)` /
`(0x8D, v)` where `v = 0x80069530(i)` ∈ {`2`,`4`,`8`,`0x10`} (or `0` when
`0x80069520()` is non-zero).

`0x80069530(i)` is `FUN_80069520() == 0 ? *(u32*)(0x802EF3A8 + i*4) : 0`, and
`0x802EF3A8..0x802EF3B4` holds `{2, 4, 8, 0x10}` (read through `main.elf`).

`0x800C9610(this, index)` needs `index >= 0`; here the index is always 5..8, and
the four channels come from `0x807C6AD0 = {5, 6, 7, 8}`.

### Constants

| address | value | use |
|---|---|---|
| `0x807C6AD0` | `u32[4] = {5, 6, 7, 8}` | per-slot dDraw channel |
| `0x807C6AB8` | `u32[4] = {1, 2, 3, 4}` | per-slot sprite seed (used by `0x80613290`) |
| `0x8042_A488` | cell | sound manager |
| `0x8042_9F50` | cell | frame/player array |
| `0x8042_A464` | cell | current player index |
| `0x802EF3A8` | `u32[4] = {2, 4, 8, 0x10}` | second SE argument |
| `0x8082_3E30` | BSS descriptor | `dEasyPairing_c` `StopStartWait` state |
| - | `0x86`..`0x8D` (134..141) | the four SE pairs |
| - | `10` | "no sprite"/"no channel" sentinel |

### Callees

`0x802DCD64`/`0x802DCDB0` (`__savegprlr_23`/`__restgprlr_23`), `0x806131D0`
(the four-slot pre-pass that seeds states `1`/`3`; address-only, decoded in
§5.1), `0x800C9490`/`0x800C92F0`/`0x800C9610`/`0x800C93D0`
(`fn_800c9490`/`fn_800c92f0`/`fn_800c9610`/`fn_800c93d0`), `0x80195380`
(`fn_80195380`), `0x80069530` (address-only), `0x80613290` (address-only sprite
picker, decoded in §5.1), the interface vtable slot `+0x18` of `+0x208`.

### 5.1 The two address-only siblings this body depends on

`0x806131D0` (called once at the top) walks the same four slots and sets
`+0x27C = 3` when the slot is in state `2` and its interface name test against
`0x80371870` fails, or `+0x27C = 1` when the slot is idle and the test passes.
`0x80613290(this, i)` (called on the `3 -> 0` edge) writes
`+0x29C[i] = 0x807C6AB8[i]`, restarts the dDraw entry, and applies the `+0x1F4`
record's scale through `0x801637D0`/`0x801644D0`. Both exports are in
`work/ghidra/rel-d_basesNP/export/`; neither has a ledger row.

### Notes for the implementer

* `+0x27C` is read once per slot and reused for the `state == 1` / `state == 3`
  tests; the `else` arm re-reads nothing, so a slot that was `1` never reaches
  the `3` arm in the same frame.
* The `(u32)(+0x28C - 5) <= 3` test is on the **channel** word and gates the
  `0x800C9610` flag query; it does not gate the state-3 arm.
* The `+0x248` byte update is `(b & 0xFE) | 1` (`rlwinm 0,24,30` + `ori 1`).
* `+0x268`'s `+0xB8` byte is cleared on the `1 -> 2` edge only.
* The fallback dispatch uses the `+0x208` interface, not `+0x70`; `+0x208` also
  holds the `0x807D5790` state-table pointer installed by the constructor.
* Return value: none; the four helpers' results are all discarded.

**Verdict: ready (medium).** Full decode, both tables and the `main.dol` SE
argument table read; medium because `0x806131D0`/`0x80613290`/`0x80069530` stay
address-only (all three are documented above) and the slot-state machine is
split across three bodies.

---

## 6. `0x80733490` - `houdai_slope` rotating-pedestal create body

**Evidence.** `work/ghidra/rel-d_basesNP/export/80733490_FUN_80733490.c`,
596 bytes, 149 words, synthetic `0x80733490`. Body `0x80733490..0x807336E0`
(ends `blr`, exactly one; the 172-byte gap starts with three zero words and then
holds the untranslated delete body `0x807336F0..0x80733758`). No `psq`. Frame:
`stwu r1,-96(r1)` with manual `stw r31`/`r30`/`r29`. Returns `1`.

**What it is.** The create/start callback of the sloped rotating pedestal built
by `fn_807333c0` (vtable `0x8080D628`, sub-vtable `0x8080D708`, model
`g3d/houdai_slope.brres` bound by `fn_80733a40`, motion blocks `+0x438`/`+0x51C`,
accessor `+0x3D8`, model `+0x3F8`, parent `+0x3D0`). It decodes the parameter
word `+0x04` into five nibbles, seeds the scale/angle/lift fields, builds the
two element ranges through `0x8007FB10`, computes the two tilt halfwords from the
two `s16` tables at `0x807CD7E0`/`0x807CD7E8` and dispatches one of the two
interface states `0x80830E20`/`0x80830E60`.

**Signature.**

```rust
/// Starts the sloped rotating pedestal: decodes the parameter nibbles, builds
/// the two element ranges and dispatches the tilt state.
///
/// # Original
/// - Ghidra: `FUN_80733490` at `0x80733490` (596 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/80733490_FUN_80733490.c`
///
/// # Arguments
/// * `actor` - the pedestal; flags `+0x04`, scales `+0xD0..+0xD8`, angle
///   `+0x104`, side byte `+0x348`, interface `+0x394`, ranges `+0x438` and
///   `+0x51C`, derived fields `+0x600..+0x648`.
///
/// # Returns
/// `1` - create success.
pub unsafe fn fn_80733490(actor: usize) -> u32
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x004` | `u32` | parameters: `&0xF`, `>>4 &0xF`, `>>8 &0xF`, `>>12 &0xF`, `>>16 &0xF` |
| `+0x0B0` | `f32` | position y, `+= 16.0` in mode 1 |
| `+0x0D0`/`+0x0D4`/`+0x0D8` | vec3 | scale `0.0` / `+10.0` (mode 0) or `-10.0` (mode 1) / `0.0` |
| `+0x104` | `s16` | facing angle from the two-entry table |
| `+0x348` | `u8` | variant byte, `flags & 0xF` |
| `+0x38F` | `u8` | surface/kind byte, forwarded to `0x8007FB10` |
| `+0x394` | object | interface; vtable slot `+0x18` receives the state |
| `+0x438` | element | first range (three instances) |
| `+0x51C` | element | second range (one instance) |
| `+0x600` | `f32` | set to `1.0f` (see the constant note) |
| `+0x604`/`+0x608` | `f32` | set to `1.0f` |
| `+0x620`..`+0x62C` | `f32` | set to `1.0f` |
| `+0x630` | `u32` | `flags & 0xF` (variant) |
| `+0x634` | `u32` | `(flags >> 4) & 0xF` (mode; `1` = raised variant) |
| `+0x638` | `u32` | `(flags >> 8) & 0xF` (state selector) |
| `+0x63C` | `u32` | cleared when the second state is dispatched |
| `+0x640` | `u32` | `(flags >> 12) & 0xF` |
| `+0x644` | `s16` | tilt angle from the four-entry table |
| `+0x646` | `s16` | the negated tilt fed to `0x8007FD80` |
| `+0x648` | `u32` | `((flags >> 16 & 0xF) + 1) * 30` |

### Exact behaviour

```
let flags = *(u32*)(actor + 0x04);
let variant = flags & 0xF;
let mode    = (flags >> 4) & 0xF;
let sel     = (flags >> 8) & 0xF;
let nib3    = (flags >> 12) & 0xF;
let count   = (((flags >> 16) & 0xF) + 1) * 30;

*(f32*)(actor + 0x600) = 1.0;
*(u32*)(actor + 0x630) = variant;
*(u32*)(actor + 0x634) = mode;
*(u32*)(actor + 0x638) = sel;
*(u32*)(actor + 0x640) = nib3;
*(u32*)(actor + 0x648) = count;
*(u8*)(actor + 0x348)  = variant as u8;
*(f32*)(actor + 0x604) = 1.0;
*(f32*)(actor + 0x608) = 1.0;
*(f32*)(actor + 0x620) = 1.0;
*(f32*)(actor + 0x624) = 1.0;
*(f32*)(actor + 0x628) = 1.0;
*(f32*)(actor + 0x62C) = 1.0;

*(s16*)(actor + 0x104) = FACE[mode];                 // 0x807CD7E8: {0x8000, 0x0000}
if mode == 1 {
    *(f32*)(actor + 0xB0) += 16.0;
}

// first range: three instances, quad A = (-8, 6, 8, -10), scale = (1, 1, 1)
0x8007FB10(actor + 0x438, actor,
           [-8.0, 6.0, 8.0, -10.0], 3, *(u8*)(actor + 0x38F),
           [1.0, 1.0, 1.0, 0.0, 0.0, 0.0]);
0x8007F900(actor + 0x438);

// second range: one instance, quad A = (-8, 14, 8, -1), same scale
0x8007FB10(actor + 0x51C, actor,
           [-8.0, 14.0, 8.0, -1.0], 1, *(u8*)(actor + 0x38F),
           [1.0, 1.0, 1.0, 0.0, 0.0, 0.0]);

*(s16*)(actor + 0x644) = TILT[mode * 2 + variant];    // 0x807CD7E0
if mode == 1 {
    *(f32*)(actor + 0xD0) = 0.0;
    *(f32*)(actor + 0xD4) = -10.0;                    // 0x807CD800
    *(f32*)(actor + 0xD8) = 0.0;
    *(s16*)(actor + 0x646) = (s16)(0x8000 - *(s16*)(actor + 0x644));
} else {
    *(f32*)(actor + 0xD0) = 0.0;
    *(f32*)(actor + 0xD4) = 10.0;                     // 0x807CD810
    *(f32*)(actor + 0xD8) = 0.0;
    *(s16*)(actor + 0x646) = -(*(s16*)(actor + 0x644));
}
0x8007FD80(actor + 0x51C, actor + 0x646);
0x8007F900(actor + 0x51C);

0x80733A40(actor);                                    // bind g3d/houdai_slope.brres

if sel == 0 {
    vmethod1(actor + 0x394, 0x18, 0x80830E20);
} else {
    vmethod1(actor + 0x394, 0x18, 0x80830E60);
    *(u32*)(actor + 0x63C) = 0;
}
1
```

The two `s16` tables are adjacent and read through the same base register
(`r31 = 0x807CD7C8`):

* `0x807CD7E8`: `s16[2] = {0x8000, 0x0000}` indexed by `mode`, i.e. the facing
  angle is a half turn for mode 0 and zero otherwise;
* `0x807CD7E0`: `s16[4] = {0x2000, 0xE000, 0xE000, 0x2000}` indexed by
  `mode * 2 + variant`, i.e. `+45`, `-45`, `-45`, `+45` degrees.

Because `mode`/`variant` are only used as small indices, any nibble above 3
reads past the table (a shipped over-read); the body does not clamp.

The `0x807CD7C8` pool word ahead of the tables is `0xFFFFFFFF`, so the tables
must be read at `0x807CD7E0` and `0x807CD7E8` exactly as above, not at an
aligned-down address.

### Constants

| address | value | use |
|---|---|---|
| `0x807CD7E0` | `s16[4] = {0x2000, 0xE000, 0xE000, 0x2000}` | tilt by `mode*2+variant` |
| `0x807CD7E8` | `s16[2] = {0x8000, 0x0000}` | facing by `mode` |
| `0x807CD7EC` | `1.0f` | the seven `+0x600`/`+0x604`/`+0x608`/`+0x620`..`+0x62C` stores |
| `0x807CD7F0` | `16.0f` | mode-1 `+0xB0` lift |
| `0x807CD7F4` | `-8.0f` | first range coordinate |
| `0x807CD7F8` | `6.0f` | first range coordinate |
| `0x807CD7FC` | `8.0f` | first range third quad word |
| `0x807CD800` | `-10.0f` | mode-1 `+0xD4` and the first range's fourth quad word |
| `0x807CD804` | `14.0f` | second range second quad word |
| `0x807CD808` | `-1.0f` | second range fourth quad word |
| `0x807CD80C` | `0.0f` | `+0xD0`/`+0xD8` |
| `0x807CD810` | `10.0f` | mode-0 `+0xD4` |
| `0x8083_0E20`, `0x8083_0E60` | BSS descriptors | interface `+0x18` arguments |

**Constant note.** The seven stores above load `lfs f0,36(r31)` with
`r31 = 0x807CD7C8`, i.e. the word at `0x807CD7EC`, which is `0x3F800000`
=`1.0f` - not `0.0`. The export agrees (`uVar2 = DAT_807cd7ec` is reused for all
seven). Read the constant; do not assume a zero sentinel.

### Callees

`0x8007FB10` (range builder: `(block, owner, quad: *const f32, count,
surfaceByte, scale: *const f32)` - the same constructor family as `0x8007FA40`,
but with the three callbacks replaced by the two pointer arguments `r5` and
`r8`), `0x8007F900` (register range), `0x8007FD80`
(store the `+0x646` angle into `+0x51C`), `0x80733A40` (`fn_80733a40`, the
`g3d/houdai_slope.brres` loader), and the interface vtable slot `+0x18` of
`+0x394`.

### Notes for the implementer

* `0x8007FB10`'s argument order is `r3 = block`, `r4 = owner`,
  `r5 = &quad[4]`, `r6 = count`, `r7 = surface byte`, `r8 = &scale[6]`. The
  Ghidra export prints the pointer arguments before `r3`, which is its usual
  argument-order scrambling; the registers are unambiguous at
  `0x80733568..0x807335AC` and `0x807335CC..0x807335FC`.
* The scale block is `{1, 1, 1, 0, 0, 0}` for both ranges; the first range's
  quad is `{-8, 6, 8, -10}` and the second's is `{-8, 14, 8, -1}`. The `-8.0`
  at `0x807CD7F4` is the first word of both quads.
* `+0x646` is `(s16)(0x8000 - tilt)` for mode 1 and `(s16)(-tilt)` otherwise.
  The `addis r3,r3,0x1` / `addi r3,r3,-0x8000` pair at `0x80733614` is the
  `+0x8000` bias, not a pointer adjustment.
* `+0x63C` is cleared only in the `sel != 0` arm, **after** the dispatch.
* The `+0x38F` byte is forwarded unchanged to both ranges.

**Verdict: ready (high).** Every operand, both `s16` tables, all nine floats and
both descriptors are read; the only soft spots (the `1.0f` at `0x807CD7EC` and
the unclamped table indices) are called out.

---

## 7. `0x80746640` - rail-placement record builder

**Evidence.** `work/ghidra/rel-d_basesNP/export/80746640_FUN_80746640.c`,
592 bytes, 148 words, synthetic `0x80746640`. Body `0x80746640..0x8074688C`
(ends `blr`, exactly one; the gap to the next row is **0** bytes). Frame:
`stwu r1,-304(r1)`, `__savegprlr_27`, `f29..f31` saved as `stfd`+`psq_st`, one
`bctrl` at `0x80746810` (the `0x802DCBFC` descriptor trampoline) and one
unconditional `b 0x80746858` loop latch. Caller: `0x807472B4` inside
`fn_80747260` (136 bytes), the camera-gated body whose `///` block already calls
this function "the placement builder `0x80746640`".

**What it is.** The builder that fills the rail-placement actor's `+0x3E0`
count / `+0x3E4` pointer array (through `0x80746890`), asks `0x80746910` for the
scene name, and then, for each of two or three descriptor slots of the current
world variant, calls one of three `main.dol`-side name/position builders through
the `0x802DCBFC` trampoline, spawns a type-`0x145` actor at
`(max_x + 96 + i*108, y, 1500)` through `0x80747120`, and records the spawned
object's first word in `+0x3DC`.

**Signature.**

```rust
/// Rebuilds the rail-placement actor's placement records for the current world.
///
/// # Original
/// - Ghidra: `FUN_80746640` at `0x80746640` (592 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/80746640_FUN_80746640.c`
///
/// # Arguments
/// * `actor` - the rail-placement actor; count `+0x3E0`, array `+0x3E4`,
///   spawned-record word `+0x3DC`, surface byte `+0x38F`, interface `+0x394`.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// The spawned-actor helper `0x80162D50` and the `type 0x145` factory must be
/// live; `0x80746890` may allocate up to 100 actors.
pub unsafe fn fn_80746640(actor: usize)
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x3DC` | `u32` | first word of the last spawned object |
| `+0x3E0` | `s32` | count of the actor array (grown by `0x80746890`) |
| `+0x3E4 + i*4` | ptr | the spawned actor chain (`0x80746890`) |
| `+0x38F` | `u8` | surface byte forwarded to the spawn helper |
| `+0x394` | object | interface (used by the caller, not by this body) |

### The tables (read from this REL)

`r30 = 0x807CE3A8` in the body; the four tables and three float constants it
uses are:

| address | contents | use |
|---|---|---|
| `0x807CE3D0` | `u32[4] = {2, 2, 3, 3}` | per-variant slot count |
| `0x807CE3E0` | `u32[12]`, four 3-word rows | per-variant descriptor-index list |
| `0x807CE410` | 12-byte records `{offset, index, target}` | the `0x802DCBFC` descriptors |
| `0x807CE438` | `u32[8] = {0, 0, 0, 1, 1, 1, 0, 1}` | variant by `0x800B2EE0(8)` |
| `0x807CE458` | `96.0f` | first rail x offset |
| `0x807CE45C` | `1500.0f` | the rail z plane |
| `0x807CE460` | `108.0f` | the rail x pitch |

The variant rows are:

| variant | count | descriptor indices (`0x807CE3E0 + v*12`) | targets |
|--:|---:|---|---|
| 0 | 2 | `0, 1` | `0x80746A70`, `0x80746C20` |
| 1 | 2 | `1, 0` | `0x80746C20`, `0x80746A70` |
| 2 | 3 | `2, 0, 1` | `0x80746DD0`, `0x80746A70`, `0x80746C20` |
| 3 | 3 | `2, 1, 0` | `0x80746DD0`, `0x80746C20`, `0x80746A70` |

and the descriptors are `{0, -1, 0x80746A70}`, `{0, -1, 0x80746C20}`,
`{0, -1, 0x80746DD0}` - i.e. offset 0, no index, direct code pointer - followed
by unrelated records at `0x807CE434` and beyond that belong to the neighbouring
bodies.

### Exact behaviour

```
// 1. clear the 0x3E0..0x570 block
*(u32*)(actor + 0x3E0) = 0;
for pass in 0..2 {                                  // ctr = 2
    for k in 0..48 {                                // 48 unrolled stores
        *(u32*)(actor + 0x3E4 + pass * 0xC0 + k * 4) = 0;   // 0x3E4..0x4A0, 0x4A4..0x560
    }
}
*(u32*)(actor + 0x564) = 0;                         // the 4-word tail
*(u32*)(actor + 0x568) = 0;
*(u32*)(actor + 0x56C) = 0;
*(u32*)(actor + 0x570) = 0;

0x80746890(actor);                                // spawn up to 100 actors of type 0x145
0x80746910(actor, &mut name, 50);                // build the 50-byte scene name at r1+0x18

// 2. pick the variant from the current world index
let variant = VARIANT[0x800B2EE0(8)];
let count   = COUNT[variant];
let list    = 0x807CE3E0 + variant * 12;          // s32[variant][..]

// 3. scene span
let scene = *(u32*)0x8042_9E00 as usize;
let x0 = *(f32*)(scene + 0x38) + *(f32*)(scene + 0x40);   // min_x + width

let mut x = x0 + 96.0;
let y_step = 108.0;
let z = 1500.0;

for i in 0..count {
    let index = *(s32*)(list + i * 4);
    if index != -1 {
        // 0x802DCBFC:  r0 = desc[0], r11 = desc[1], r12 = desc[2]
        //              r3 += r0;  if r11 >= 0 { r12 = *(r3+r12); r12 = *(r12+r11) }
        //              ctr = r12; bctr     -> tail call through the descriptor
        let desc = 0x807CE410 + index * 12;
        let mut probe = [0u32; 2];                     // stack r1+8, filled by the callee
        // the callee gets r3 = actor, r4 = &probe, r5 = &name, r6 = 50
        let hit = trampoline_802DCBFC(actor, &mut probe, &name, 50, desc);
        if hit != 0 {
            let mut pos = [x, *(f32*)&probe[0], z];    // r1+0xC .. r1+0x14
            let spawned = 0x80747120(actor, &pos);     // type 0x145 spawn, r3 result
            if spawned != 0 {
                *(u32*)(actor + 0x3DC) = *(u32*)spawned;
            }
        }
    }
    x += y_step;
}
```

Step 1's exact shape: the loop body stores `0` to `+0x3E4..+0x4A0` and the
`ctr = 2` outer loop re-runs it at `+0x4A4..+0x560`; `+0x3E0` is cleared before
the loop and `+0x564..+0x570` after it.

`x` starts at `min_x + width + 96.0` and steps by `108.0`; `z` is the constant
`1500.0`; the `y` component comes from the descriptor callee's first output
word (`r1+8`), not from the actor.

`0x802DCBFC` is the `main.dol` descriptor trampoline, fully decoded:

```
lwz  r0,0(r12)      ; added to r3
lwz  r11,4(r12)     ; index, -1 = direct
lwz  r12,8(r12)     ; offset to the function-pointer slot
add  r3,r3,r0
cmpi cr0,r11,0
blt  direct
lwzx r12,r3,r12
lwzx r12,r12,r11
direct:
mtspr ctr,r12
bctr
```

So a descriptor `{0, -1, target}` is a plain indirect call of `target` with
`r3 = actor`; the 125 48-byte `0x802DCBFC` *stubs* documented in
`docs/ANALYSIS_WAVE127_BASES.md:77-80` are just wrappers that build such a
descriptor on the fly. Here the body calls the trampoline directly, so the
targets are the three literals listed above.

### Constants

| address | value | use |
|---|---|---|
| `0x807CE3D0` | `{2, 2, 3, 3}` | per-variant slot count |
| `0x807CE3E0` | 4 x 3 words | per-variant index list |
| `0x807CE410` | 3-word records | `0x802DCBFC` descriptors |
| `0x807CE438` | `{0,0,0,1,1,1,0,1}` | variant by world index |
| `0x807CE458` | `96.0f` | first rail offset |
| `0x807CE45C` | `1500.0f` | rail z plane |
| `0x807CE460` | `108.0f` | rail x pitch |
| `0x8042_9E00` | cell | scene object (`+0x38` min_x, `+0x40` width) |
| `0x80746A70`, `0x80746C20`, `0x80746DD0` | code | the three descriptor targets |
| - | `0x145` (325) | spawned actor type |

### Callees

`0x802DCD74`/`0x802DCDC0` (`__savegprlr_27`/`__restgprlr_27`), `0x80746890`
(`fn_80746890`, the 100-actor chain), `0x80746910` (address-only, 50-byte
scene-name builder; its TSV row is 344 bytes, and the first descriptor target
`0x80746A70` starts 8 bytes past that extent), `0x800B2EE0`
(`fn_800b2ee0`, world-index helper), `0x802DCBFC` (the
trampoline above), `0x80747120` (address-only, "spawn type `0x145` at position"
— it floor-probes with `0x80075270`, clamps `y`, and calls
`0x80064610(0x145, 0x10001010, &pos, 0, 0)`), `0x80746A70`/`0x80746C20`/
`0x80746DD0` (the three descriptor targets).

### Notes for the implementer

* `0x80747120` is typed `void` by Ghidra, but the raw body ends with
  `bl 0x80064610` + epilogue, so `r3` is whatever `0x80064610` →
  `0x8006CA80(0x145, 0x10001010, &pos, 0)` left there. This body tests that
  value and reads `*(u32*)r3`. Mark the read `// observed:` and keep the
  register dependency; do not invent a null check on `0x80747120`'s return.
* The trampoline is a **tail** call (`bctr`), but because the call site uses
  `bl`, the callee's `blr` returns to `0x80746814`. That is the intended
  "call through a pointer" idiom; a Rust `transmute`-to-`fn` call is equivalent.
* `0x80162D50`-spawned actors (`0x80746890`) and `0x8006CA80`-spawned actors
  (`0x80747120`) are different arrays: the first fills `+0x3E4`, the second only
  feeds `+0x3DC`.
* The `ori r0,r0,0x0` at `0x80746814` is a placeholder word, not an operation.
* The loop offset table is 4-byte strided inside a 12-byte row, so variant rows
  deliberately overlap the next row's first word for variant 3
  (`0x807CE3E0 + 36 = 0x807CE404`). Read the values from the table above; do not
  re-derive a stride.

**Verdict: ready (medium).** All 148 words decoded, the trampoline decoded
instruction-by-instruction, and all four tables plus the three floats read from
the image. Medium because `0x80746910`/`0x80747120`/`0x800B2EE0` stay
address-only and `0x80747120`'s return register is a documented quirk.

---

## 8. `0x8079BF00` - world-map node proximity gate

**Evidence.** `work/ghidra/rel-d_basesNP/export/8079bf00_FUN_8079bf00.c`,
588 bytes, 147 words, synthetic `0x8079BF00`. Body `0x8079BF00..0x8079C148`
(ends `blr`, exactly one; the 4-byte gap is one zero word and the next row is
the 224-byte `0x8079C150`). No `psq`. Frame: `stwu r1,-80(r1)`,
`__savegprlr_26`, epilogue `__restgprlr_26`. Called from `0x8079BEDC` inside
`0x8079BE80` (120 bytes), the sibling that produces the `(this, node, kind)`
triple this body receives.

**What it is.** A world-map gate that tests the two mask halves (`0x1000`,
`0x2000`) of the current course node's layer word, probes the two
`0x800FA070`/`0x800FA0E0` engine predicates against the node position, sets the
two per-screen latch bytes `+0x25A`/`+0x25B` and fires the two sound flags
`(4, 1)` / `(8, 1)`. When either latch is set it then rescans up to 192 scene
entries for the sibling that owns the same handle and, on the first match,
applies the one-shot course id `0x8042A24D` through `0x800CE3C0` and clears both
latches.

**Signature.**

```rust
/// Tests the current course node for the two proximity flags and resolves the
/// sibling entry that owns the same handle.
///
/// # Original
/// - Ghidra: `FUN_8079bf00` at `0x8079BF00` (588 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/8079bf00_FUN_8079bf00.c`
///
/// # Arguments
/// * `this` - the world-map actor; latches `+0x25A`/`+0x25B`, sound object
///   `+0x29C`.
/// * `node` - the node handle passed to `0x80799B00`.
/// * `kind` - the third `0x80799B00` argument.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// The scene manager cell `0x8042A18C` and the save-image record cell
/// `0x8042A040` must be live.
pub unsafe fn fn_8079bf00(this: usize, node: u32, kind: u32)
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x25A` | `u8` | first proximity latch (mask `0x1000` half) |
| `+0x25B` | `u8` | second proximity latch (mask `0x2000` half) |
| `+0x29C` | `u32` | object handed to `0x80C561E0(obj, 4|8, 1)` |

Globals: `0x8042A18C` (scene manager), `0x8042A040` (save-image record cell),
`0x8042A24D` (one-shot course id byte), and the `0x800F57A0/57E0/56B0/9530`
scene API on `scene + *(u32*)(scene+0x338C)*0x16518 + 0x3394`.

### Exact behaviour

```
let scene  = *(u32*)0x8042_A18C as usize;
let record = 0x800E0380(*(u32*)0x8042_A040, 0xFFFFFFFF);   // current save image
let id     = 0x80799B00(this, node, kind);                 // sibling id or -1

let mut pos = [0u32; 3];
0x80100200(&mut pos, scene, id);                           // node world position
let table = scene + (*(u32*)(scene + 0x338C) as usize) * 0x16518 + 0x3394;

// half 1
if 0x800F57A0(table, id, 0x1000) != 0 {
    let p = pos;
    if 0x800FA070(&p) == 0 {
        *(u8*)(this + 0x25A) = 1;
        0x80C561E0(*(u32*)(this + 0x29C), 4, 1);
    }
}

// half 2
if 0x800F57A0(table, id, 0x2000) != 0 {
    let p = pos;
    if 0x800FA0E0(&p) == 0 {
        0x80C561E0(*(u32*)(this + 0x29C), 8, 1);
        *(u8*)(this + 0x25B) = 1;
    }
}

// sibling resolution
if *(u8*)(this + 0x25A) != 0 || *(u8*)(this + 0x25B) != 0 {
    let target = 0x800F57E0(table, id);
    if target != 0 {
        for slot in 0..192 {
            if 0x800F57A0(table, slot, 1) == 0 { continue; }
            if 0x800F57E0(table, slot) != target { continue; }

            let course = *(u8*)0x8042_A24D as u32;
            0x800F56B0(table, slot);
            let value = 0x800F9530() & 0xFF;      // rlwinm r5,r3,0,24,31
            if 0x800CE3C0(record, course, value, 0x20) != 0 {
                *(u8*)(this + 0x25A) = 0;
                *(u8*)(this + 0x25B) = 0;
            }
            break;                                 // first handle match only
        }
    }
}
```

The two predicates are tested through `countLeadingZeros(v) >> 5`, which is `1`
exactly when `v == 0`; the body therefore acts when the engine call returns
**zero**. The Ghidra export prints the same test as `uVar2 >> 5 != 0`.

The `else`-print `local_3c = local_30; local_38 = local_2c; local_34 = local_28`
in the C is Ghidra's stack-slot reordering of a plain 12-byte `vec3` copy:
`r1+0x20/0x24/0x28` → `r1+0x14/0x18/0x1C`, i.e. `(x, y, z)` in order
(`0x8079BF8C..0x8079BFA4`).

### Constants

| address | value | use |
|---|---|---|
| `0x8042_A18C` | cell | scene manager pointer |
| `0x8042_A040` | cell | save-image record cell |
| `0x8042_A24D` | `u8` | one-shot course id |
| - | `0x1000`, `0x2000` | the two layer masks |
| - | `1` | the "any layer" mask of the sibling scan |
| - | `0xC0` (192) | sibling scan bound |
| - | `0x20` (32) | fourth argument of `0x800CE3C0` |
| - | `4`, `8` | `0x80C561E0` ids |

### Callees

`0x802DCD70`/`0x802DCDBC` (`__savegprlr_26`/`__restgprlr_26`), `0x800E0380`
(`save_image_record_a`, engine::display), `0x80799B00` (address-only, sibling-id
resolver), `0x80100200` (node position getter), `0x800F57A0` (layer test),
`0x800FA070`/`0x800FA0E0` (the two engine predicates), `0x800F57E0` (handle
resolution), `0x800F56B0` (container element), `0x800F9530` (kind/byte getter),
`0x800CE3C0` (`fn_800ce3c0`), `0x80C561E0` (`game::bosses` flag setter).

### Notes for the implementer

* `this` comes from `r3` (save helper); `node` is `r4` (saved in `r27`) and
  `kind` is `r5` (saved in `r26`). The Ghidra signature's three `undefined4`
  parameters are correct here; only the `0x80799B00` call print is scrambled by
  the helper's "64-bit return".
* The two predicate arms differ in the order of the store: half 1 stores
  `+0x25A` **before** the sound call, half 2 stores `+0x25B` **after** it.
* `0x800F57E0(table, id)` is called **once** and its result kept in `r26`; the
  short-circuit `&&` means it is not called when both latches are clear.
* The scan breaks on the first `0x800F57E0(table, slot) == target` even when
  `0x800CE3C0` then returns 0; the latch clear is inside that arm.
* `0x800CE3C0` is the "mask then non-zero" helper (`fn_800ce3c0`); the third
  argument is masked to 8 bits by `rlwinm r5,r3,0,24,31`.

**Verdict: ready (medium).** All 147 words decoded and every offset, mask and
bound is literal; medium because four `0x800Fxxxx` scene-API callees stay
address-only and the two predicate semantics are only fixed to "returns zero".

---

## Cross-checks performed

* Address mapping re-derived from both ELF program-header tables (quoted above);
  every literal was read through it. `0x80832EB0`, `0x80830918`, `0x80830E20`,
  `0x80830E60` and `0x80823E30` read through the REL BSS zero fill and are marked
  as BSS where they occur.
* `ledger/functions.csv` contains no row for any of the eight (checked against
  the full 17,131-address set, case-insensitive).
* No `#{2,4} ` heading of any of the 26 `docs/SPEC_WAVE*_BASES.md` files that
  existed before this one contains any of the eight; the only textual
  appearances are
  * `docs/SPEC_WAVE153_BASES.md:181-186`, which names `0x8068D060` (604) and
    `0x80799340` (604) as the ninth and tenth candidates, and
  * `docs/SPEC_WAVE140_BASES.md:682`, `:732` and `:1365`, which name
    `0x80799340` as the outstanding sibling condition of the world-map state
    table and of `0x8079D3C0`.
* `src/game/bases.rs` has no `fn fn_<addr>` definition for any of the eight.
  The only doc-comment appearances are the neighbour citations listed in the
  selection section (`0x80746640` in `fn_80747260`'s block, and the wave-97
  `gabon_rock` cluster block that omits row 1).
* Extent/`blr` recomputation for all eight is in the table near the top; all
  eight extents are exact, so the `docs/REL_ELF_MAPPING.md:37-42` correction is
  recorded as *not* needed rather than skipped silently.
* Every constant table value was read from the image (`d.js`/`cx.js`) or from
  `main.elf` through its own program headers (`md.js`), including the four
  `0x802EF3A8` words, the `0x807CE3xx`/`0x807CE4xx` tables and both `0x807CD7Ex`
  `s16` tables.
* Prose citations are limited to `GQR3 = 0x00050005`
  (`docs/REL_ELF_MAPPING.md:52-56`, not needed by this set), the 48-byte
  `0x802DCBFC` stub family (`docs/ANALYSIS_WAVE127_BASES.md:77-80`, superseded
  here by the decoded trampoline), the wave-97 `gabon_rock` cluster block, the
  wave-50 `fn_80700820` block, and the `fn_8072bf60`/`fn_8072c200` fire-cannon
  blocks, all in `src/game/bases.rs`.
* `src/game/bases.rs` is being edited by other lanes concurrently (its
  `fn fn_<addr>` count moved from 3,759 to 3,772 during this session, none of
  them in this set), so this document quotes its comments instead of citing
  line numbers.
* This lane wrote only this file.
