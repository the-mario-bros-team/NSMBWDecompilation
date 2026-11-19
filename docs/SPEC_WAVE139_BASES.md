# Wave 139 - `game::bases` - the last 12 class-C `qr3` specifications

Read-only specification lane. No Rust file, ledger row or existing document was
modified. The only file written is this one.

Waves 128-137 consumed the 121 class-B rows of
`docs/ANALYSIS_WAVE127_BASES.md`. Wave 138 pinned `GQR3 = 0x00050005` and took
the twelve largest class-C bodies. The class-C block has 25 rows, so exactly
**twelve** `qr3` bodies remained below that cut - the twelve listed by wave
138's hand-off ("the remaining 12 `qr3` class-C rows"). This wave specifies
those twelve; no fallback to another class was needed, because the pool of
GQR3-unblocked bodies is exactly twelve.

With this document every class-C body except `0x807BD110` is specified. That
last row is the `dFileSelect_c` 44-entry registration table, not a body, and
still needs a table generator.

## The twelve

| # | address | bytes | words | export file | class | verdict |
|--:|---:|---:|---:|---|:--:|---|
| 1 | `0x806B3770` | 308 | 77 | `806b3770_FUN_806b3770.c` | C | ready (high) |
| 2 | `0x806FB610` | 296 | 74 | `806fb610_FUN_806fb610.c` | C | ready (high) |
| 3 | `0x80680E80` | 292 | 73 | `80680e80_FUN_80680e80.c` | C | conditional (medium) |
| 4 | `0x8066C9A0` | 232 | 58 | `8066c9a0_FUN_8066c9a0.c` | C | ready (high) |
| 5 | `0x806B2D10` | 200 | 50 | `806b2d10_FUN_806b2d10.c` | C | ready (medium) |
| 6 | `0x806DE0B0` | 132 | 33 | `806de0b0_FUN_806de0b0.c` | C | ready (medium) |
| 7 | `0x806DE320` | 132 | 33 | `806de320_FUN_806de320.c` | C | ready (medium) |
| 8 | `0x806D65A0` | 112 | 28 | `806d65a0_FUN_806d65a0.c` | C | ready (high) |
| 9 | `0x807040C0` | 112 | 28 | `807040c0_FUN_807040c0.c` | C | ready (high) |
| 10 | `0x80704050` | 108 | 27 | `80704050_FUN_80704050.c` | C | ready (high) |
| 11 | `0x806D6530` | 104 | 26 | `806d6530_FUN_806d6530.c` | C | ready (high) |
| 12 | `0x806DDA10` | 88 | 22 | `806dda10_FUN_806dda10.c` | C | ready (high) |

Together **2 116 bytes** (529 words). Every `bytes` value is the `size` column
of `work/ghidra/rel-d_basesNP/export/functions.tsv` verbatim; every body range
was re-decoded from `work/rels/elf/d_basesNP.elf`, and all twelve end in
`0x4E800020` (`blr`) at the last word of the TSV extent.

**Verdict split: 11 ready, 1 conditional.** The single conditional row has a
residual unknown that is *not* GQR3:

| address | remaining condition |
|---|---|
| `0x80680E80` | calls `0x80050D40`, an unclaimed `main.dol` function with no export under `work/ghidra/` and no ledger row. The call shape is visible (player actor, per-index entry pointer, `vec3*`, one byte kind) and the body is fully specified, but the callee contract - especially the meaning of the byte `this[0x374C]` - is inferred, not verified. |

"medium" on the other rows marks a class-vtable dispatch or a field role that
is inferred, never the arithmetic.

### Selection rule

Candidate set: the 25 class-C rows of `docs/ANALYSIS_WAVE127_BASES.md`. Remove
the twelve addressed by `docs/SPEC_WAVE137_BASES.md` /
`docs/SPEC_WAVE138_BASES.md` and the non-body table `0x807BD110`. The
remainder is exactly the twelve above, ordered by `functions.tsv` size,
descending. Every one was re-checked against three exclusion sets:

| test | result |
|---|---|
| address field in `ledger/functions.csv` | 0 of 12 present |
| `0x<addr>` in any `#{1,6} ` heading of any `docs/*.md` | 0 of 12 |
| `fn_<addr>` definition in `src/game/bases.rs` | 0 of 12 |

`0x806DDA10` is *mentioned* in `src/game/bases.rs:174631` and in four spec
paragraphs, but only as a call comment inside already-translated neighbours
("`fn_806DDA10(this, half)`"); there is no `pub unsafe fn fn_806dda10`. The
address appeared in wave 138's hand-off table, not in a section heading.

## GQR3, restated (no re-derivation here)

`docs/SPEC_WAVE138_BASES.md` pinned the word from `main.dol`:

```
GQR3 = 0x00050005      type 5 = unsigned 16-bit, scale 0, load and store
```

Every `psq_l`/`psq_st` in these twelve uses `I = 3`, `w1` and `DQ = 0`, so the
substitution is always the same two lines:

```rust
// psq_l fN,0(rB),w1,qr3
let raw = platform::read_u16(rb) as f32;   // zero-extend; NOT i16
// psq_st fN,0(rB),w1,qr3
platform::write_u16(rb, f as u16);         // truncate; NOT i16
```

The `psq_st/psq_l ... qr0` pairs in the prologues are the compiler's
`f31/f30/f29` paired-single spills (`GQR0 = 0`, `f32`), i.e. `write_f32` /
`read_f32`, not quantisation.

The two table-trig callees are the ones wave 138 identified:
`0x80237BD0` = sine of a 1/256-turn angle, `0x80237C40` = cosine, both reading
the 256-entry table at `0x802F74A0`. In every body below the argument is
`(u16)halfword * (1/256)`, so the halfword is a 1/65536-turn angle.

## Method

**Address mapping (re-read from the program headers, not copied).**
`d_basesNP.elf` has six `PT_LOAD` headers; the first is
`p_offset = 0x10F0`, `p_vaddr = 0x806000F0`, so

```
file_offset = 0x10F0 + (address - 0x806000F0) = (address - 0x80600000) + 0x1000
```

as `docs/REL_ELF_MAPPING.md` requires. The last segment
(`p_vaddr = 0x80822900`, `filesz = 0`, `memsz = 0x12484`) is zero-fill; the two
BSS cells cited below (`0x8082DDA8`, `0x8082DEA8`) fall inside it and are
cited as addresses only.

**Extent check.** Full linear decode entry-to-last-word; last word `0x4E800020`
for all twelve; no `halt_baddata`, no jump table, no `functions.tsv` row
starting inside any body.

**Duplicate check.** SHA-1 of each of the twelve ranges against every
same-size `d_basesNP` TSV row: **no match**. The near-twins `0x806DE0B0` /
`0x806DE320` differ in exactly two words - the argument pointer
(`0x8082DDA8` vs `0x8082DEA8`) and one branch displacement - so they are two
distinct state bodies, not copies.

**Paired-single inventory** (`psq_l` = opcode 56, `psq_st` = 60, `I` = bits
12-14, `W` = bit 15, `DQ` = bits 0-11, signed):

| address | `qr3` sites | `qr0` prologue spills |
|---|---|---|
| `0x806B3770` | `0x806B37CC`, `0x806B37EC` | `f31`, `f30` |
| `0x806FB610` | `0x806FB68C`, `0x806FB6AC` | `f31`, `f30`, `f29` |
| `0x80680E80` | `0x80680EF4`, `0x80680F38` | none |
| `0x8066C9A0` | `0x8066CA10` | none |
| `0x806B2D10` | `0x806B2D74`, `0x806B2D90` | none |
| `0x806DE0B0` | `0x806DE100` | none |
| `0x806DE320` | `0x806DE370` | none |
| `0x806D65A0` | `0x806D65C8` | none |
| `0x807040C0` | `0x807040E8` | none |
| `0x80704050` | `0x80704078` | none |
| `0x806D6530` | `0x806D6558` | none |
| `0x806DDA10` | `0x806DDA30` | `f31` |

**Decoder.** A throwaway PowerPC decoder
(`%TEMP%\opencode\w139_dis.js`) decoded opcodes 14-15, 18, 20-21, 24-29, 31
(with `or`/`and`/`or.` `rA` destination ordering), 32-56/60, 59 (A-form
`fmuls`/`fmadds` third operand from the `FRC` field, not `FRB`), 63 and 4.
The A-form correction matters: `fmuls f1,f0,f1` is `0xEC200072`, which a
`FRB`-based decoder prints as `fmuls f1,f0,f0` and would turn every
`1/256 * angle` into `(1/256)^2`.

## 1. `0x806B3770` - 308 bytes - `work/ghidra/rel-d_basesNP/export/806b3770_FUN_806b3770.c`

**Body:** `0x806B3770..0x806B38A4` (77 words, `blr` at `0x806B38A0`).
Ledger: absent. `src/game/bases.rs`: absent.

Frame `0x50`; saves LR, `f31`/`f30` (`stfd` + `psq_st qr0`), `r31`-`r28`.

**Exact behaviour** (`this` in r3, `pos` in r4, `angle` in r5):

1. `x = pos[0]`, `z = pos[1]` (read at entry, re-read after the trig calls).
2. `this[0x444] = this[0x134C]` (plain `f32` copy, not a pointer).
3. `this[0x43C] = pos[0]`, `this[0x440] = pos[1]`,
   `this[0x430] = (u16)angle`.
4. `c = cos((f32)(u16)angle * (1/256))` - the halfword is staged at `r1+0x0A`
   (`sth r5,10(r1)`), loaded with `psq_l ...,qr3`.
5. `sn = sin((f32)(u16)angle * (1/256))` - staged at `r1+8`.
6. `w = this[0x1350]`, `r = this[0x134C]`.
7. `this[0x490] = (u16)(angle - 0x2000)`,
   `this[0x4F0] = (u16)(angle + 0x2000)` (halfword stores of the 16-bit sums).
8. Write the rotated pair about `pos` in the XZ plane:

```
this[0x49C] = pos[0] + ( w*c - r*sn)
this[0x4A0] = pos[1] + ( w*sn + r*c)
this[0x4FC] = pos[0] + (-w*c - r*sn)
this[0x500] = pos[1] + (-w*sn + r*c)
```

9. The four `stfs` to `r1+0x10..0x1C` at `0x806B3888..0x806B3894` happen after
   the register restores and before `addi r1,r1,0x50`; they are compiler
   spill into the frame being destroyed. Omit them in Rust.

**Signature.**

```rust
/// Writes the two rail endpoints of a gate/door-like base object.
/// # Original
/// `FUN_806b3770`, `0x806B3770`, 308 bytes,
/// `work/ghidra/rel-d_basesNP/export/806b3770_FUN_806b3770.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - object with valid `+0x134C`, `+0x1350` and `+0x430..+0x500`.
/// * `pos` - two readable `f32` (x, z).
/// * `angle` - 1/65536-turn angle; the body converts it to 1/256-turn units.
/// # Confidence
/// high - the raw stream is fully decoded; the only callees are sin/cos.
pub unsafe fn fn_806b3770(this: usize, pos: *const f32, angle: i16)
```

**Constants.** `0x807CAA34 = 0x3B800000` (`1/256`); `±0x2000` halfword angle
limits; field offsets above.

**Verdict: ready (high).**

---

## 2. `0x806FB610` - 296 bytes - `work/ghidra/rel-d_basesNP/export/806fb610_FUN_806fb610.c`

**Body:** `0x806FB610..0x806FB738` (74 words, `blr` at `0x806FB734`).
Ledger: absent. `src/game/bases.rs`: absent.

Frame `0x60`; prologue `bl 0x802DCD74` (`_savegpr_27`), epilogue
`bl 0x802DCDC0` (`_restgpr_27`); `f31`/`f30`/`f29` spilled through
`stfd` + `psq_st qr0`.

**Exact behaviour** (`this` r3, `angle` r4, `index` r5):

1. `handle = this[0x6E8 + index*4]` (`lwz r3,1768(r3+r5*4)`).
2. `target = fn_80162d00(handle)` (`0x80162D00`, translated); r4 is left in
   `angle` but the callee's signature is `fn_80162d00(id: u32)`, so it is
   ignored.
3. If `target == 0`, skip to the epilogue.
4. `target[0x102] = (u16)(this[0x884 + index*2] + angle)`
   (`lha`, 32-bit add, `sth`).
5. `a = this[0x774 + index*4]`, `b = this[0x7FC + index*4]`.
6. `c = cos((f32)(u16)angle/256)`, `s = sin((f32)(u16)angle/256)`; the cosine
   is kept in `f31`.
7. `target[0xAC] = this[0x738] + c*a + s*b`
   `target[0xB4] = this[0x740] + (c*b - s*a)`
8. `v = fn_806fb540(this, (i16)target[0x102], a)` -
   `r3 = this`, `r4 = (i16)this[0x102]`, `f1 = a`. The translated
   `fn_806fb540(this: usize, code: i32, value: f32) -> f32` matches.
9. `target[0xDC] = target[0xE0] = target[0xE4] = v`.

Note that the object passed to `fn_806fb540` is `this`, not `target`, and the
angle read is `this[0x102]` (the same slot just written on `target` only when
`target == this`).

**Signature.**

```rust
/// Positions slot `index` of a segmented base object and copies the derived
/// radius/height into the resolved child.
/// # Original
/// `FUN_806fb610`, `0x806FB610`, 296 bytes,
/// `work/ghidra/rel-d_basesNP/export/806fb610_FUN_806fb610.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - parent table object; `+0x6E8`, `+0x884`, `+0x774`, `+0x7FC`,
///   `+0x738`, `+0x740` must be valid for the given index.
/// * `angle` - 1/65536-turn angle added to the slot's stored base angle.
/// * `index` - slot selector, expected `0..N`.
/// # Confidence
/// high - all offsets, both trig calls and the save/restore helpers are
/// literal; `0x80162D00` and `0x806FB540` are already translated.
pub unsafe fn fn_806fb610(this: usize, angle: i16, index: i32)
```

**Constants.** `0x807CC3F0 = 0x3B800000` (`1/256`); slot strides `4` (word
tables) and `2` (halfword table); sub-object writes at `+0x102`, `+0xAC`,
`+0xB4`, `+0xDC..+0xE4`.

**Verdict: ready (high).**

---

## 3. `0x80680E80` - 292 bytes - `work/ghidra/rel-d_basesNP/export/80680e80_FUN_80680e80.c`

**Body:** `0x80680E80..0x80680FA4` (73 words, `blr` at `0x80680FA0`).
Ledger: absent. `src/game/bases.rs`: absent.

Frame `0x30`; saves LR, `r31`, `r30`.

**Exact behaviour** (`this` r3):

1. `if (this[0x374D] as i8) == -1 { return; }`
2. `idx = this[0x3750]` (u32).
3. `x = this[0xAC]`, `y = this[0xB0] - 400.0`, `z = this[0xB4]`.
4. `phase = (i16)this[0x36F4 + idx*2] + 0x4000` (recomputed identically for the
   second call).
5. `vec = ( x + this[0x3724]*cos(phase/256),
             y + this[0x3724]*sin(phase/256),
             z )` - the stack vector lives at `r1+0x18` (`x`), `r1+0x1C` (`y`),
   `r1+0x20` (`z`).
6. `player = fn_8005f900((i8)this[0x374D] as usize)` (translated; index form
   of the player lookup). If null, return.
7. `0x80050D40(player,
                this + 0x2194 + idx*0xE4,
                &vec,
                this[0x374C] as u8)` -
   `mulli r0,r0,228` (`0xE4`) plus `addi r4,r4,0x2194`.

**Signature.**

```rust
/// Spawns the per-slot effect for player `this[+0x374D]` at the scaled
/// position around `this[+0xAC..+0xB4]`.
/// # Original
/// `FUN_80680e80`, `0x80680E80`, 292 bytes,
/// `work/ghidra/rel-d_basesNP/export/80680e80_FUN_80680e80.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - manager object; `+0x374C` kind byte, `+0x374D` player index
///   (`-1` = inactive), `+0x3750` slot index, `+0x36F4` angle table,
///   `+0x3724` radial scale, `+0x2194` entry stride `0xE4`.
/// # Confidence
/// medium - everything in the body is literal; the callee `0x80050D40` is an
/// unclaimed main.dol spawn helper, so its fourth argument's meaning is
/// inferred from the call shape.
pub unsafe fn fn_80680e80(this: usize)
```

**Constants.** `0x807C9660 = 0x43C80000` (`400.0f`, the y offset subtracted);
`0x807C96AC = 0x3B800000` (`1/256`); `0x4000` halfword phase bias; `0xE4`
entry stride; `0x2194` entry base.

**Verdict: conditional (medium)** - the body is complete; the open item is the
`0x80050D40` contract. It is not in `ledger/functions.csv`, has no export in
`work/ghidra/**`, and the nearest claimed neighbour is `0x80050D80` (an
unrelated event-flag query), so nothing in the corpus constrains it.

---

## 4. `0x8066C9A0` - 232 bytes - `work/ghidra/rel-d_basesNP/export/8066c9a0_FUN_8066c9a0.c`

**Body:** `0x8066C9A0..0x8066CA88` (58 words, `blr` at `0x8066CA84`).
Ledger: absent. `src/game/bases.rs`: absent.

Frame `0x30`; saves LR, `r31`, `r30`.

**Exact behaviour** (`this` r3; this is the sibling of the already-translated
`fn_8066c870` and uses the same emitter idiom):

1. `fn_8066c720(r1+0x18, this)` - builds a 12-byte sample
   `{this[0xAC], this[0xB0] + *(0x807C8EE8), this[0xB4]}`.
2. `emitter = this + 0x71C`;
   `send = *(*(this + 0x71C) + 0xA0)`;
   `send(emitter, 0x807E92A0, 0, &sample, 0, 0)` - the string is
   `"Wm_ob_keywait"` (Ghidra names it `s_Wm_ob_keywait_807e92a0`; the same
   address and the same six-word call are used by `fn_8066c870`).
3. `this[0x102] = (u16)(this[0x102] + 0x400)`.
4. `phase = (i16)this[0x5F0] + 0x100`; `this[0x5F0] = (u16)phase`.
5. `this[0xB0] = this[0x5EC] - 10.0 * sin(phase/256)`.
6. `0x8006A4B0(&sample2, this + 0xAC)` - copies the **updated** three-float
   position.
7. `0x80199390(*(0x80429D60), 0x2FE, this[0], &sample2, 0)` - sound/event
   `0x2FE` owned by the word at `this+0x00`.

**Signature.**

```rust
/// Per-frame "key wait": resubmits `Wm_ob_keywait`, slows the bob and plays
/// sound `0x2FE`.
/// # Original
/// `FUN_8066c9a0`, `0x8066C9A0`, 232 bytes,
/// `work/ghidra/rel-d_basesNP/export/8066c9a0_FUN_8066c9a0.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - live key actor; emitter `+0x71C`, spin `+0x102`, bob angle
///   `+0x5F0`, height `+0x5EC`, position `+0xAC`, sound owner `+0x00`.
/// # Confidence
/// high - the emitter call, both angle steps, the `10.0` drift and the sound
/// id are literal, and the identical idiom is already translated in
/// [`fn_8066c870`].
pub unsafe fn fn_8066c9a0(this: usize)
```

**Constants.** `0x807C8F2C = 0x3B800000` (`1/256`);
`0x807C8ED0 = 0x41200000` (`10.0`, reached as `0x807C8EA8 + 0x28`);
`0x807E92A0` = `"Wm_ob_keywait"`; sound id `0x2FE`; manager cell
`0x80429D60`.

**Verdict: ready (high).**

---

## 5. `0x806B2D10` - 200 bytes - `work/ghidra/rel-d_basesNP/export/806b2d10_FUN_806b2d10.c`

**Body:** `0x806B2D10..0x806B2DD8` (50 words, `blr` at `0x806B2DD4`).
Ledger: absent. `src/game/bases.rs`: absent.

Frame `0x20`; saves LR, `r31`, `r30`, `r29`; `r31 = 0x807CA980` (float pool).

**Exact behaviour** (`this` r3):

1. `this[0xDC] = this[0xE0] = this[0xE4] = *(0x807CA9F8)` (`0.0f`).
2. `this[0x3D8] = *(0x807CAA08)` (`0.393f`).
3. `(*(*(this + 0x60) + 0x9C))()` with `r3 = this` - a plain class-vtable
   dispatch (`this->vfunc_9C()`), no extra arguments.
4. `this[0x120] |= 0x80`.
5. `phase = (i16)this[0x504]`.
6. `this[0x3D4] = sin((f32)(u16)phase / 256)`
   `this[0x3D0] = cos((f32)(u16)phase / 256)`
   (the same halfword is stored to `r1+0x0A` and `r1+8` for the two calls).
7. `this[0x10C] = *(0x807CAA0C)` (`3.0f`); `this[0x50C] = 1`;
   `this[0x36D] = 4`.

**Signature.**

```rust
/// Initialises the swinging base: clears the tri-float, seeds the sway
/// factors and dispatches the class vfunc.
/// # Original
/// `FUN_806b2d10`, `0x806B2D10`, 200 bytes,
/// `work/ghidra/rel-d_basesNP/export/806b2d10_FUN_806b2d10.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - object with a class vtable at `+0x60`; `+0x504` phase,
///   `+0x3D0`/`+0x3D4` sin/cos outputs, `+0x120` flags.
/// # Confidence
/// medium - every constant, offset and the vfunc slot are literal; the vfunc
/// itself is a runtime dispatch.
pub unsafe fn fn_806b2d10(this: usize)
```

**Constants.** `0x807CA9F8 = 0x00000000` (`0.0`);
`0x807CAA00 = 0x3B800000` (`1/256`);
`0x807CAA08 = 0x3EC9374C` (`0.393f`);
`0x807CAA0C = 0x40400000` (`3.0f`);
initialised bytes `+0x50C = 1`, `+0x36D = 4`; vfunc slot `0x9C` of the vtable
at `+0x60`.

**Verdict: ready (medium).**

---

## 6. `0x806DE0B0` - 132 bytes - `work/ghidra/rel-d_basesNP/export/806de0b0_FUN_806de0b0.c`

**Body:** `0x806DE0B0..0x806DE134` (33 words, `blr` at `0x806DE130`).
Ledger: absent. `src/game/bases.rs`: absent.

Frame `0x20`; saves LR, `r31`.

**Exact behaviour** (`this` r3):

1. `t = this[0x724].wrapping_sub(1)`; `this[0x724] = t`.
2. If `t as u8 == 0`:
   `(*(*(this + 0x394) + 0x18))(this + 0x394, 0x8082DDA8)` -
   a one-argument interface dispatch on the sub-object at `+0x394`
   (`lwzu r12,916(r3)` also advances the receiver to `this+0x394`).
3. Else:
   `phase = (i16)this[0x722]`;
   `sin((f32)(u16)phase / 256)` is evaluated and discarded;
   `this[0x722] = (u16)(phase + 0x800)`.

**Signature.**

```rust
/// Counts down the effect timer; on expiry resets the `+0x394` interface,
/// otherwise advances the visual phase.
/// # Original
/// `FUN_806de0b0`, `0x806DE0B0`, 132 bytes,
/// `work/ghidra/rel-d_basesNP/export/806de0b0_FUN_806de0b0.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - object with timer `+0x724`, phase `+0x722` and interface
///   `+0x394`.
/// # Confidence
/// medium - the countdown, branch, phase step and sin argument are literal;
/// the reset target is a virtual call and `0x8082DDA8` is a BSS cell whose
/// contents are never written by this REL.
pub unsafe fn fn_806de0b0(this: usize)
```

**Constants.** `0x807CB9E0 = 0x3B800000` (`1/256`); phase step `0x800`;
interface argument `0x8082DDA8` (zero-fill segment), vtable slot `0x18`.

**Verdict: ready (medium).**

---

## 7. `0x806DE320` - 132 bytes - `work/ghidra/rel-d_basesNP/export/806de320_FUN_806de320.c`

**Body:** `0x806DE320..0x806DE3A4` (33 words, `blr` at `0x806DE3A0`).
Ledger: absent. `src/game/bases.rs`: absent.

Instruction-for-instruction the same body as #6, with two different words:

| word | #6 `0x806DE0B0` | #7 `0x806DE320` |
|---|---|---|
| interface argument (`+0x394` reset) | `0x8082DDA8` | `0x8082DEA8` |
| `bl` displacement at `+0x60` | `4BB59AC1` | `4BB59851` (same target `0x80237BD0`) |

Behaviour, signature, constants and verdict are therefore identical to #6, with
`0x8082DEA8` in place of `0x8082DDA8`:

```rust
pub unsafe fn fn_806de320(this: usize)
```

**Verdict: ready (medium).**

---

## 8. `0x806D65A0` - 112 bytes - `work/ghidra/rel-d_basesNP/export/806d65a0_FUN_806d65a0.c`

**Body:** `0x806D65A0..0x806D6610` (28 words, `blr` at `0x806D660C`).
Ledger: absent. `src/game/bases.rs`: absent.

Frame `0x20`; saves LR, `r31`.

**Exact behaviour** (`this` r3):

```
this[0xF02] = (u16)(this[0xF02] + 0x200)
phase       = (i16)this[0xF02]
this[0x104] = (u16)(i32)( sin(phase/256) * 128.0 )
```

The multiply is `fmuls f0,f1,f0` against `128.0`, then `fctiwz`, `stfd`, and
`lwz r0,20(r1)` takes the low word before the `sth`. Truncation is toward zero.

**Signature.**

```rust
/// Advances the aim/tilt phase and stores the quantised sine.
/// # Original
/// `FUN_806d65a0`, `0x806D65A0`, 112 bytes,
/// `work/ghidra/rel-d_basesNP/export/806d65a0_FUN_806d65a0.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - object with phase `+0xF02` and output `+0x104`.
/// # Confidence
/// high - straight-line arithmetic with one sin call.
pub unsafe fn fn_806d65a0(this: usize)
```

**Constants.** `0x807CB704 = 0x3B800000` (`1/256`);
`0x807CB6BC = 0x43000000` (`128.0`); phase step `0x200`.

**Verdict: ready (high).**

---

## 9. `0x807040C0` - 112 bytes - `work/ghidra/rel-d_basesNP/export/807040c0_FUN_807040c0.c`

**Body:** `0x807040C0..0x80704130` (28 words, `blr` at `0x8070412C`).
Ledger: absent. `src/game/bases.rs`: absent.

Frame `0x20`; saves LR, `r31`.

**Exact behaviour** (`this` r3):

```
this[0xE02] = (u16)(this[0xE02] + 0x200)
phase       = (i16)this[0xE02]
this[0x104] = (u16)(i32)( sin(phase/256) * 128.0 )
```

Identical shape to #8 with a different phase slot and pool cells.

**Signature.**

```rust
/// Same phase-to-sine quantiser as [`fn_806d65a0`], on the `+0xE02` phase.
/// # Original
/// `FUN_807040c0`, `0x807040C0`, 112 bytes,
/// `work/ghidra/rel-d_basesNP/export/807040c0_FUN_807040c0.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Confidence
/// high - straight-line arithmetic with one sin call.
pub unsafe fn fn_807040c0(this: usize)
```

**Constants.** `0x807CC5E8 = 0x3B800000` (`1/256`);
`0x807CC590 = 0x43000000` (`128.0`, as `0x807CC588 + 8`); phase step `0x200`.

**Verdict: ready (high).**

---

## 10. `0x80704050` - 108 bytes - `work/ghidra/rel-d_basesNP/export/80704050_FUN_80704050.c`

**Body:** `0x80704050..0x807040BC` (27 words, `blr` at `0x807040B8`).
Ledger: absent. `src/game/bases.rs`: absent.

Frame `0x20`; saves LR, `r31`.

**Exact behaviour** (`this` r3):

```
this[0xE00] = (u16)(this[0xE00] - 0x100)
phase       = (i16)this[0xE00]
this[0xB0]  = this[0xDFC] + 2.0 * sin(phase/256)
```

The oscillator drives the height field `+0xB0` about the centre stored at
`+0xDFC`; amplitude `2.0`.

**Signature.**

```rust
/// Oscillates the base height around `+0xDFC` with amplitude 2.
/// # Original
/// `FUN_80704050`, `0x80704050`, 108 bytes,
/// `work/ghidra/rel-d_basesNP/export/80704050_FUN_80704050.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Confidence
/// high - straight-line arithmetic with one sin call.
pub unsafe fn fn_80704050(this: usize)
```

**Constants.** `0x807CC5E8 = 0x3B800000` (`1/256`);
`0x807CC58C = 0x40000000` (`2.0`, as `0x807CC588 + 4`); phase step `-0x100`.

**Verdict: ready (high).**

---

## 11. `0x806D6530` - 104 bytes - `work/ghidra/rel-d_basesNP/export/806d6530_FUN_806d6530.c`

**Body:** `0x806D6530..0x806D6598` (26 words, `blr` at `0x806D6594`).
Ledger: absent. `src/game/bases.rs`: absent.

Frame `0x20`; saves LR, `r31`.

**Exact behaviour** (`this` r3):

```
this[0xF00] = (u16)(this[0xF00] - 0x100)
phase       = (i16)this[0xF00]
this[0xB0]  = this[0xEF8] + 2.0 * sin(phase/256)
```

Same oscillator as #10 on the `+0xF00` phase and the `+0xEF8` centre.

**Signature.**

```rust
/// Oscillates the base height around `+0xEF8` with amplitude 2.
/// # Original
/// `FUN_806d6530`, `0x806D6530`, 104 bytes,
/// `work/ghidra/rel-d_basesNP/export/806d6530_FUN_806d6530.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Confidence
/// high - straight-line arithmetic with one sin call.
pub unsafe fn fn_806d6530(this: usize)
```

**Constants.** `0x807CB704 = 0x3B800000` (`1/256`);
`0x807CB6B8 = 0x40000000` (`2.0`); phase step `-0x100`.

**Verdict: ready (high).**

---

## 12. `0x806DDA10` - 88 bytes - `work/ghidra/rel-d_basesNP/export/806dda10_FUN_806dda10.c`

**Body:** `0x806DDA10..0x806DDA68` (22 words, `blr` at `0x806DDA64`).
Ledger: absent. `src/game/bases.rs`: mentioned only as a call comment at
`src/game/bases.rs:174631`; no definition.

Frame `0x20`; saves LR and `f31` (`stfd` + `psq_st qr0`).

**Exact behaviour** (`this` r3, `angle` r4; returns `f32` in `f1`):

```
amp = this[0x70C]
return amp - amp * cos((u16)angle / 256)
```

The raw sequence is `f31 = this[0x70C]`, `cos(...)`, `fneg f0,f1`,
`fmuls f1,f31,f0`, `fadds f1,f1,f31`. Ghidra types the function `double`
because of the `psq_st` prologue; the value is a single-precision `f32` and
callers use `f1` directly.

**Signature.**

```rust
/// Damped height term `amp * (1 - cos(angle))`.
/// # Original
/// `FUN_806dda10`, `0x806DDA10`, 88 bytes,
/// `work/ghidra/rel-d_basesNP/export/806dda10_FUN_806dda10.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - object with amplitude `+0x70C`.
/// * `angle` - 1/65536-turn angle.
/// # Returns
/// `this[0x70C] * (1.0 - cos(angle/256))`.
/// # Confidence
/// high - two-instruction arithmetic after one cosine; the call sites in
/// `src/game/bases.rs:174631` read the `f32` result.
pub unsafe fn fn_806dda10(this: usize, angle: i16) -> f32
```

**Constants.** `0x807CB9E0 = 0x3B800000` (`1/256`); field `+0x70C`.

**Verdict: ready (high).**

---

## Ready vs conditional

| address | verdict | residual work |
|---|---|---|
| `0x806B3770` | ready (high) | none |
| `0x806FB610` | ready (high) | none |
| `0x80680E80` | **conditional (medium)** | `0x80050D40` contract (unclaimed main.dol spawn helper) |
| `0x8066C9A0` | ready (high) | none |
| `0x806B2D10` | ready (medium) | class vfunc `+0x60`/`0x9C` is a runtime dispatch |
| `0x806DE0B0` | ready (medium) | interface vfunc `+0x394`/`0x18`; `0x8082DDA8` is BSS |
| `0x806DE320` | ready (medium) | same, `0x8082DEA8` |
| `0x806D65A0` | ready (high) | none |
| `0x807040C0` | ready (high) | none |
| `0x80704050` | ready (high) | none |
| `0x806D6530` | ready (high) | none |
| `0x806DDA10` | ready (high) | none |

**No GQR3 blocker remains in any row.** `GQR3 = 0x00050005`, type 5 (`u16`),
scale 0, both directions, for the whole build.

## Verification performed

* `ledger/functions.csv` parsed read-only; the address column contains none of
  the twelve. `0x806DDA10` appears only inside another row's evidence text and
  in a `// observed:` comment.
* `src/game/bases.rs`: no `pub unsafe fn fn_<addr>` for any of the twelve; the
  single `fn_806DDA10` occurrence is the call comment at line 174631.
* Every `docs/*.md` heading (`^#{1,6} `) was scanned for `0x80xxxxxx`; none of
  the twelve appears in a heading. `0x806DDA10` is listed in wave 138's
  hand-off table and in wave 137's call notes, not in a section title.
* `work/ghidra/rel-d_basesNP/export/functions.tsv`: all twelve byte counts read
  from the `size` column and re-checked against the decoded `blr` extent.
* Raw decode: `work/rels/elf/d_basesNP.elf`, `PT_LOAD` table re-read, formula
  `file_offset = (address - 0x80600000) + 0x1000`.
* Extent: last word `0x4E800020` for all twelve; no `halt_baddata`, no jump
  table, no TSV row starting inside a body.
* Duplicate check: SHA-1 of each range against every same-size `d_basesNP` TSV
  row - no match; the two 132-byte twins differ in exactly two words.
* Constants were read through the program headers and are quoted with their
  image addresses. `0x807E92A0` is the `"Wm_ob_keywait"` string already used by
  the translated `fn_8066c870`.
* Callee status checked against the ledger: `0x80162D00`, `0x8005F900`,
  `0x8066C720`, `0x806FB540`, `0x80199390` are translated; `0x8006A4B0` is a
  documented main.dol vector copy; `0x80050D40` is the only unclaimed callee.

## Hand-off

* With this wave, **24 of the 25** class-C rows of
  `docs/ANALYSIS_WAVE127_BASES.md` are specified (12 in wave 137/138, 12 here).
* `0x807BD110` (2,908 bytes) is the `dFileSelect_c` 44-entry registration
  table and still needs the table generator, not a control-flow spec.
* `0x80050D40` is worth a separate main.dol claim: it is the only unresolved
  callee introduced by this block and it gates #3.
* The class-D block of `docs/ANALYSIS_WAVE127_BASES.md` (173 addresses) is
  untouched.
