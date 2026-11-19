# Audit - Wave 284 (two tables closed, misattribution found)

## 1. Tables closed

| Table | Slots | Identity |
|---|---:|---|
| `0x8034EAD0` | **6 methods** (8 words) | NW4R sound DVD archive reader |

**Sixty-three vtables complete.**

Correction to the survey: `0x8034EAD0` is **not** a 10-slot table. It is a 2-word
header plus 6 methods; `0x8034EAF0` is the next class's header, proven by
decoding that class's `+0x10`/`+0x20` slots against base `0x8034EAF0` only. The
survey's cited "install sites" `0x802692D4`/`0x80269394` are not function
entries at all - they are the `lis` instructions inside two factories that
install the *window* vtable.

The class is the reader embedded at `archive_manager + 0xC`: delete, two
zero-answer methods, required window size `0x78`, and two factories that build
the 0x78-byte window stream over an already-open file or a resolved path.

## 2. Three argument defects fixed

All three from the wave-283 audit, each reproduced from the instruction stream:

| Body | Correction | Evidence |
|---|---|---|
| `fn_8027b360` | three clock seeds must use `second` | `add r30,r30,r0` builds `second`; `mr r4,r28` feeds it to `0x80264C10` |
| `fn_8027ddd0` | elements at `this+0xB58`; `[+0xC]`; `+0xB58` callback; slot word for rollback | `addi r27,r24,0xB58`, `lwz r4,12(r28)`, `mr r8,r28`, `lwz r3,4(r27)` |
| `fn_80274ce0` | 4th argument is `r3 + 4` | `addi r6,r28,4` |

## 3. A misattributed body found by the sweep

`fn_806e3c50` was a **second copy of `0x806E5370`** (already present as
`fn_806e5370`), with 23 of its 25 literal call targets not existing at
`0x806E3C50`. The real function is a 144-byte leaf installing collision boxes for
the four `0x60`-byte slots at `+0x5F0..+0x770`. Replaced, with its stubs removed.

The same lane restored a **dropped call** in `fn_80712130`: the kind-1 branch
gates a mask bit on `0x8013AAA0(object) == 0`, and the translation omitted the
call entirely. Ghidra's export shows the same condition, so it was an old
translate defect.

## 4. `_savegpr`/`_restgpr` sweep, part 9

| Metric | Before | After |
|---|---:|---:|
| Modelled helper calls | 286 | **246** |
| Bodies containing them | 149 | **124** |
| Bodies changed | - | 25 |
| Calls removed | - | **40** |
| Genuine calls found | - | **0** |

All 25 bodies were in `bases.rs` and verified against `d_basesNP.elf`. No
constants needed deleting (inline literals only).

The lane also **rejected** a wave-283 follow-up: `fn_806cff90` should *not* gain
an `actor` parameter, because `0x806CFF90` has no save stub and `li r3,0x1CF`
overwrites the incoming `r3`. The stale comment claiming otherwise was removed.

## 5. Survey

| Metric | Wave 280 | Wave 284 |
|---|---:|---:|
| Fully covered runs | 68 | **82** |
| Distinct prologue targets | 807 | **753** |
| Runs with >=4 prologue | 61 | **48** |
| Max prologue in one run | 6 | **5** |

Every one of wave 280's top-ten runs is now closed. The new top is EN65
`Wm_en_obakedoor` (`0x80303660`, 171 slots, 5 prologue), followed by an NW4R G3D
table, a `d_profile` vtable, the three wipe-scene tables, and two
worldmap-cursor tables. No regressions; interior labels unchanged at 2,127.

## 6. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | three `seq.rs` argument fixes | 3 fixed | - |
| B | DVD reader close | 5 | ~450 |
| C | savegpr sweep part 9 | 25 fixed | - |
| D | survey refresh | 0 | - |
| E | gap batch 14 | 12 | ~1,100 |
| F | audit wave 283 | 0 (doc fixes) | - |

## Verification

- `cargo check` exit 0 (30 warnings); `doc-lint` 0 violations.
- `check-dupe-bodies` exit 0; `check-page-offsets` 42 pairs / 13,757 tokens,
  0 mismatches.
- `scripts/check.ps1` exit 0.
