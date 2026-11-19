# Audit - Wave 285 (five tables closed, decoder bug traced)

## 1. Tables closed

| Table | Slots | Identity |
|---|---:|---|
| `0x80303660` | **171/171** | `daEnDoor_c` base vtable |
| `0x80301354` / `0x8030140c` / `0x803014c4` | **7/7** each | `wipeDokan` / `wipeKuppa` / `wipeMario` |
| `0x8034E8F8` | **16/16** | `nw4hbm::g3d::ScnMdl` |
| `0x8032B2D0` | **8/8** | `nw4hbm::lyt::Material` animation table |

**Seventy-one vtables complete.**

Identity corrections: `0x80303660` is the **`daEnDoor_c` base vtable**, not
`Wm_en_obakedoor` - the literal is not RTTI, the same pattern as `"dead_ice"`
before `daEnDpakkunBase_c`. Three proofs: the `+0xD4` targets are the
`daEnDoor_c::StateID_*` descriptors, the parameter blocks and name strings begin
exactly at the array end, and all six `d_enemiesNP` door subclasses are
171-slot copies with 27-28 overrides.

The wipe trio share slot 2 (`0x8016DD10`, also `wipeCircle`'s slot 2), have
byte-identical slots 1/3/4/6, and differ only in the SDA publication
(`0x80429C38`/`C40`/`C48`) and the state-table base (`0x802EE070`/`098`/`0C0`).
The `0x80352670/6B0/6F0/730` block is **not** wipe data: it is
`daBossDemo_c`'s state-descriptor block (the four records are named
`daBossDemo_c::StateID_*`), built from the header immediately after Mario's
array - which confirms the array is exactly 7 words.

## 2. Decoder bug and a resulting body fix

Lane E found that the shared decoder printed `FRB` where opcode 59 needs `FRC`
(and vice versa for `fadds`/`fsubs`). That invalidated a body from the previous
batch: `fn_800db530` computed `a*b + b*b`, but `0x800DB568` is
`fmuls f1,f1,f1` and `0x800DB56C` is `fmuls f0,f0,f0`, so the sum is
**`a*a + b*b`**. Fixed.

Also fixed: `fn_800f1160` had an inverted gate. `bc 4,2` at `0x800F1184` jumps
**to** the bind path (`0x800F1190`) when the probe is non-zero, so zero is the
early-out - the body returned 0 on non-zero.

## 3. `_savegpr`/`_restgpr` sweep, part 10

| Metric | Before | After |
|---|---:|---:|
| Modelled helper calls | 246 | **196** |
| Bodies containing them | 124 | **99** |
| Bodies changed | - | 25 |
| Calls removed | - | **53** (50 census + 3 macro sites) |
| Genuine calls found | - | **0** |

22 bodies gained a real `r3` parameter; two correctly stay argument-free because
they never read `r3`. Nine caller sites updated after checking `r3` in the image.

Four corrections beyond the sweep:

1. `fn_80795fe0` had **swapped arguments** (`r28=r3` actor, `r29=r4` text block).
2. `fn_807b7080` **dropped its return value** (`li r3,1`).
3. `fn_807995e0`/`fn_80795fe0` epilogue "arguments" were return values.
4. `fn_8076af60` does pass the found result as `r3`; the old comment was wrong.

The lane added a **live-register scan** over all 25 bodies, confirming only
`r3`-`r6` are read before write - a systematic version of the check that caught
the `wave99`/`wave104` class.

## 4. Body reported, then fixed here

`fn_800f1160` (above). No other body-level defects in the wave-284 set.

## 5. Survey

Wave 284's count held: 82 fully covered runs, 753 distinct prologue targets,
max 5 per run, 48 runs with >=4. This wave closed five more of the sixteen
5-prologue runs.

## 6. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | `daEnDoor_c` close | 18 | 808 |
| B | ScnMdl + Material close | 11 + 7 | ~4,000 |
| C | savegpr sweep part 10 | 25 fixed | - |
| D | wipe trio close | 19 | ~1,000 |
| E | gap batch 15 | 12 | ~1,180 |
| F | audit wave 284 | 0 (1 fix) | - |

## Verification

- `cargo check` exit 0 (30 warnings); `doc-lint` 0 violations.
- `check-dupe-bodies` exit 0; `check-page-offsets` 46 pairs / 13,774 tokens,
  0 mismatches.
- `scripts/check.ps1` exit 0.
