# Wave 138 - `game::bases` - the GQR3 word and 12 class-C specifications

Read-only specification lane. No Rust file, ledger row or existing document was
modified. The only file written is this one.

Waves 128-137 consumed the class-B pool of `docs/ANALYSIS_WAVE127_BASES.md`
(121 addresses, 51 060 bytes). The class-C block of that report has 25
addresses; 24 of them are bodies whose only stated blocker is the type/scale
word of **`GQR3`**, and one (`0x807BD110`, 2 908 bytes) is a registration table,
not a body. This wave pins `GQR3` from `main.dol`, then specifies the twelve
largest untranslated `qr3` bodies. Three of them (`0x806B7CD0`, `0x806DD730`,
`0x80680720`) already carry wave-137 sections marked *conditional* on exactly
this word; the other nine are new. Every one of the twelve becomes mechanically
translatable once the word below is substituted.

## The twelve

| # | address | bytes | words | export file | class | verdict |
|--:|---:|---:|---:|---|:--:|---|
| 1 | `0x806B7CD0` | 1 704 | 426 | `806b7cd0_FUN_806b7cd0.c` | C | conditional (medium) |
| 2 | `0x806DD730` | 736 | 184 | `806dd730_FUN_806dd730.c` | C | ready (medium) |
| 3 | `0x80680720` | 660 | 165 | `80680720_FUN_80680720.c` | C | ready (medium) |
| 4 | `0x8072E520` | 644 | 161 | `8072e520_FUN_8072e520.c` | C | ready (medium) |
| 5 | `0x8072BC40` | 604 | 151 | `8072bc40_FUN_8072bc40.c` | C | ready (high) |
| 6 | `0x806B9A30` | 476 | 119 | `806b9a30_FUN_806b9a30.c` | C | conditional (medium) |
| 7 | `0x80681C50` | 384 | 96 | `80681c50_FUN_80681c50.c` | C | ready (medium) |
| 8 | `0x8072C000` | 372 | 93 | `8072c000_FUN_8072c000.c` | C | ready (medium) |
| 9 | `0x80688BA0` | 356 | 89 | `80688ba0_FUN_80688ba0.c` | C | ready (high) |
| 10 | `0x80688E10` | 356 | 89 | `80688e10_FUN_80688e10.c` | C | ready (high) |
| 11 | `0x806B2BB0` | 340 | 85 | `806b2bb0_FUN_806b2bb0.c` | C | ready (high) |
| 12 | `0x8068EA10` | 332 | 83 | `8068ea10_FUN_8068ea10.c` | C | ready (medium) |

Together **6 964 bytes** (1 741 words). Every `bytes` value is the `size`
column of `work/ghidra/rel-d_basesNP/export/functions.tsv` verbatim, and every
body range below was re-decoded from
`work/rels/elf/d_basesNP.elf`; all twelve end in `0x4E800020` (`blr`) at the
last word of the TSV extent.

**Verdict split: 10 ready, 2 conditional.** The two conditional rows have a
residual unknown that is *not* GQR3:

| address | remaining condition |
|---|---|
| `0x806B7CD0` | the two `+0xA0` virtual calls in blocks X/Y use mixed float/integer argument shapes that no existing helper in `src/game/bases.rs` expresses; they need dedicated host helpers. Wave 137 marked this row conditional only for GQR3; that reason is now gone. |
| `0x806B9A30` | calls `0x80C3DFC0` / `0x80C3DFB0`, whose synthetic addresses lie outside both this REL's `PT_LOAD` range and `main.dol`'s image (import relocations), and reads the `main.dol` bitmask word `0x80429CA8` which is not shipped in this REL. Behaviour is fully specified; the two import contracts are named, not resolved. |

All other ten are ready to implement as written; "medium" marks bodies where a
callee contract or a field role is inferred, never the arithmetic.

### Selection rule

Candidate set: the 24 `qr3` rows of the class-C block of
`docs/ANALYSIS_WAVE127_BASES.md` (`0x8066C9A0` .. `0x8072E520`), sorted by the
`functions.tsv` size, descending, and filtered by **no row in
`ledger/functions.csv` whose address field equals the address** and **no
`fn_<addr>` in `src/game/bases.rs`**. The top twelve are exactly the table
above. `0x807BD110` (2 908) is excluded by the report's own classification: it
is the `dFileSelect_c` 44-entry state table and needs a table generator, not a
control-flow spec. The three wave-137 rows remain in the cut because they are
still untranslated; this document adds the missing GQR3 substitution and does
not repeat wave 137's block tables for them.

No class-C address has a ledger row or a `fn_` body:

| address | ledger rows | `src/game/bases.rs` |
|---|---:|---|
| `0x806B7CD0` | 0 | absent |
| `0x806DD730` | 0 | absent |
| `0x80680720` | 0 | absent |
| `0x8072E520` | 0 | mentioned (`fn_8072ebc0`, `fn_8072eaa0` comments) - not defined |
| `0x8072BC40` | 0 | absent |
| `0x806B9A30` | 0 | absent |
| `0x80681C50` | 0 | absent |
| `0x8072C000` | 0 | mentioned (`fn_8072ba10` comment) - not defined |
| `0x80688BA0` | 0 | absent |
| `0x80688E10` | 0 | absent |
| `0x806B2BB0` | 0 | absent |
| `0x8068EA10` | 0 | absent |

The two "mentioned" rows are address-only call comments in already translated
neighbours; neither address has a `pub unsafe fn`.

## The GQR3 word

### Answer

```
GQR3 = 0x00050005
```

Decomposed with the image's own field layout:

| half | bits | value | meaning |
|---|---:|---:|---|
| load (low 16) | 0-2 | `5` | type 5 = unsigned 16-bit integer |
| load | 8-13 | `0` | scale 0, multiplier `2^0 = 1.0` |
| store (high 16) | 16-18 | `5` | type 5 = unsigned 16-bit integer |
| store | 24-29 | `0` | scale 0 |

Type table (the PSQ quantiser encodings, as used by the same image): `0` =
`f32`, `4` = `u8`, `5` = `u16`, `6` = `s8`, `7` = `s16`. A `psq_l ... ,qr3`
therefore **zero-extends the 16-bit source to `f32` with no scaling**, and a
`psq_st ... ,qr3` truncates an `f32` to an **unsigned** 16-bit integer.

### Derivation from `main.dol`

`psq_l`/`psq_st` encode the GQR index in bits 12-14 (0 = `qr0`, 3 = `qr3`), the
`W` bit in bit 15, the base register in bits 16-20, and the signed `DQ`
displacement in bits 0-11. The word is *not* in `d_basesNP`: a full-text scan of
`work/rels/elf/d_basesNP.elf` finds zero `mtspr` to SPR 912-919. The value comes
from `main.dol`.

A full scan of `C:\Users\WitherULTRA7\Downloads\mariobroswii\sys\main.dol`
(text segments only, every word with opcode 31 / XO 467) returns **37**
`mtspr` hits to SPR 912-919, in exactly six groups:

| site | SPRs written | value |
|---|---|---|
| `0x801AB174`..`0x801AB190` | `GQR0..GQR7` | `0` (`li r3,0` then eight `mtspr`) |
| `0x801ACE88`..`0x801ACEB8` | `GQR1..GQR7` | restored from thread context `+0x1A8..+0x1C0` |
| `0x8025EEE8`..`0x8025EF24` | `GQR2..GQR7` | `0x00040004`, `0x00050005`, `0x00060006`, `0x00070007`, `0x05070507`, `0x08070807` |
| `0x802AB50C`..`0x802AB530` | `GQR2..GQR5` | `0x00040004`, `0x00050005`, `0x00060006`, `0x00070007` |
| `0x802C2770`..`0x802C2794` | `GQR2..GQR5` | same four |
| `0x802EA6FC`..`0x802EA718` | `GQR0..GQR7` | save-area restore in the interrupt path |

The GQR3-specific writes are therefore only six:

| address | word | write |
|---|---:|---|
| `0x801AB180` | `0x7C73E3A6` | `mtspr 915,r3` with `r3 = 0` (boot clear) |
| `0x801ACE98` | `0x7C93E3A6` | `mtspr 915,r4` (thread-context restore) |
| `0x8025EEF4` | `0x7C73E3A6` | `mtspr 915,r3` with `r3 = 0x00050005` |
| `0x802AB518` | `0x7C73E3A6` | `mtspr 915,r3` with `r3 = 0x00050005` |
| `0x802C277C` | `0x7C73E3A6` | `mtspr 915,r3` with `r3 = 0x00050005` |
| `0x802EA708` | `0x7EF3E3A6` | `mtspr 915,r23` (save-area restore) |

The only non-zero constant ever loaded into SPR 915 is `0x00050005`. The
instruction triple is identical at all three installation sites:

```
0x8025EEEC  38600005   li   r3,5
0x8025EEF0  64630005   oris r3,r3,5
0x8025EEF4  7C73E3A6   mtspr 915,r3      ; GQR3 = 0x00050005
```

and it sits inside a run that also installs `GQR2 = 0x00040004`,
`GQR4 = 0x00060006`, `GQR5 = 0x00070007`, `GQR6 = 0x05070507`,
`GQR7 = 0x08070807`. The four consecutive halfwords `0x0004`, `0x0005`,
`0x0006`, `0x0007` are the canonical `u8`/`u16`/`s8`/`s16` quartet. That is the
internal proof that halfword `5` is `u16`: a reading in which 5 were `s16`
would leave 7 meaningless.

**Which write wins.** The boot clear at `0x801AB180` belongs to
`FUN_801AB150` (`stwu`/`mflr`, `bl 0x801A9C70`, `oris r3,r3,0xA000`,
`bl 0x801A9C80`, `bl 0x801AC5A0`, `sync`, `li r3,0`, eight `mtspr`), called
from the SDK/CRT init at `0x80004310`. The non-zero installation belongs to
`FUN_8025EEB0`, called once from `FUN_80164500` at `0x80164568` during display/GX
initialisation, and duplicated by `FUN_802AB4F0` (`0x8016319C`) and the
`0x802C2750` block. The GX initialiser runs after the CRT clear, so the value
live when `d_basesNP` game code executes is `0x00050005`. The two context
restores (`0x801ACE98`, `0x802EA708`) only preserve whichever value the thread
carried, so they cannot introduce any other type.

**Independent consistency check.** Every one of the 24 `qr3` bodies in
`d_basesNP` loads a 16-bit halfword (in almost all of them a signed angle just
stored to the stack, in two bodies a `rlwinm`-extracted field), then multiplies
the dequantised result by the code-level constant `1/256`
(`0x3B800000`, at a per-cluster `0x807Cxxxx` cell), then calls one of the two
table-trig helpers:

| helper | contract |
|---|---|
| `0x80237BD0` | sine of an angle expressed in **1/256-turn units** (256.0 = 2*pi), with range reduction by `65536.0` (`0x8042E4A0`), truncation to `u16` through `GQR3`, 256-entry interpolation table at `0x802F74A0`, and odd-sign restoration |
| `0x80237C40` | cosine, same convention and table, even function (no sign fix-up) |

That contract is exactly `u16`, scale 0: the raw halfword is a 1/65536-turn
angle, `raw * 1/256` converts it to the table's 1/256-turn unit, and the table
period of 256 makes the unsigned wrap equal the signed angle. A `s16` with
scale 8 reading (`0x0807`) would divide the angle by a further 256 and collapse
every rotation to `sin(0) ~= 0`; it is numerically wrong for these bodies.

**Correction of the wave-137 note.** `docs/SPEC_WAVE137_BASES.md` wrote
"with `GQR3.load = 0x0807` ...". `0x0807` is not a GQR3 value: it is the *load
half* of `GQR7 = 0x08070807` (type 7 = `s16`, scale 8), installed in the same
block. `GQR3` is `0x00050005`, type 5 = `u16`, scale 0. The wave-137 sections
for `0x806B7CD0`, `0x806DD730` and `0x80680720` remain correct in every block
except that substitution.

**Corroboration in the existing corpus.** `docs/COVERAGE_WAVE106_WORLDMAP.md`
already recorded the same finding from the world-map lane: "The image writes
`0x00050005` to `SPR 915` ... i.e. type `5` (unsigned 16-bit) with scale `0`".
This wave re-derived it from `main.dol` directly and extends it to
`d_basesNP`.

### How to translate a `qr3` site

```rust
// psq_l fN,DQ(rB),w1,qr3     (GQR3 = 0x00050005: u16, scale 0)
let raw = read_u16(rb + dq) as f32;   // zero-extend; NOT i16
// psq_st fN,DQ(rB),w1,qr3
write_u16(rb + dq, f as u16);         // truncate; NOT i16
```

The `1/256` multiplier and the sin/cos call are *body* code, not part of GQR3,
and must be kept exactly where the stream has them. The `psq_st/psq_l` pair with
`qr0` in every prologue is the compiler's `f31`/`f30`/`f29`/`f28` spill and is
`f32` (GQR0 is cleared to 0 by `0x801AB150` and never written non-zero), so it
is `write_f32`/`read_f32`, not quantisation.

## Method

**Address mapping (re-read, not copied).** `d_basesNP.elf` has six `PT_LOAD`
headers:

| `p_offset` | `p_vaddr` | `filesz` | `memsz` |
|---|---:|---:|---:|
| `0x10F0` | `0x806000F0` | `0x1C6004` | `0x1C6004` |
| `0x1C70F4` | `0x807C60F4` | `0x4FC` | `0x4FC` |
| `0x1C75F0` | `0x807C65F0` | `0xC` | `0xC` |
| `0x1C7600` | `0x807C6600` | `0xA5FC` | `0xA5FC` |
| `0x1D1C00` | `0x807D0C00` | `0x51CB8` | `0x51CB8` |
| `0` | `0x80822900` | `0` | `0x12484` |

so `file_offset = 0x10F0 + (address - 0x806000F0) = (address - 0x80600000) +
0x1000`, as `docs/REL_ELF_MAPPING.md` states. Everything at or above
`0x80822900` is zero-fill and is cited as an address only.

**Extent check.** Every range was decoded from entry to the last word; the last
word is `0x4E800020` for all twelve. No `halt_baddata`, no jump table, no
`functions.tsv` row starts inside a body.

**Duplicate check.** The SHA-1 of each of the twelve ranges was compared against
every `d_basesNP` TSV row of the same size: **no match** for any of them.

**Paired-single inventory.** `psq_l`/`psq_st` are opcodes 56/60 in this build.
The `qr3` sites are:

| address | sites |
|---|---|
| `0x806B7CD0` | `0x806B7F78`, `0x806B7F94` |
| `0x806DD730` | `0x806DD828`, `0x806DD938` |
| `0x80680720` | `0x806807DC`, `0x8068080C`, `0x806808C0`, `0x806808F0` |
| `0x8072E520` | `0x8072E5CC`, `0x8072E5EC` |
| `0x8072BC40` | `0x8072BDAC`, `0x8072BDC8` |
| `0x806B9A30` | `0x806B9A90`, `0x806B9ABC` |
| `0x80681C50` | `0x80681D48` |
| `0x8072C000` | `0x8072C0AC`, `0x8072C0C8` |
| `0x80688BA0` | `0x80688BE4`, `0x80688C2C`, `0x80688C7C`, `0x80688C9C` |
| `0x80688E10` | `0x80688E54`, `0x80688E9C`, `0x80688EEC`, `0x80688F0C` |
| `0x806B2BB0` | `0x806B2C98` |
| `0x8068EA10` | `0x8068EAB4` |

Every one is `w1` (single-element gather/scatter) with `I = 3`; none of the
twelve touches `qr1`, `qr2` or `qr4`-`qr7`.

---

## 1. `0x806B7CD0` - 1 704 bytes - `work/ghidra/rel-d_basesNP/export/806b7cd0_FUN_806b7cd0.c`

**Body:** `0x806B7CD0..0x806B8378` (426 words, `blr` at `0x806B8374`).
Ledger: absent. `src/game/bases.rs`: absent. Wave 137: section 10, currently
conditional only because GQR3 was unknown.

This is the `0x1048`-byte pose/animation initialiser. The wave-137 block table
(P..Z) is authoritative and is not repeated; the only change is block T's
quantiser.

**GQR3 block (T).** Two halfwords are built from `this+0x1048` and dequantised:

| site | instruction | source |
|---|---|---|
| `0x806B7F78` | `psq_l f1,0(r5),w1,qr3` | `r5 = r1+0x0A`, filled by `sth r0,10(r1)` at `0x806B7F68`; `r0 = rlwinm(r0,13,0,18)` of `this[0x1048]` at `0x806B7F64` |
| `0x806B7F94` | `psq_l f1,0(r3),w1,qr3` | `r3 = r1+8`, filled by `sth r25,8(r1)` at `0x806B7F88`; `r25 = (s16)this[0x1048]` |

With `GQR3 = 0x00050005` each load is `(u16)halfword` then `* 1/256`
(`0x807CAB68 = 0x3B800000`), then `0x80237C40` (cos) / `0x80237BD0` (sin). The
two results are scaled by `f30` and written to `this+0xE8` and `this+0xEC`.
Wave 137's prose "`(u16)(n >> 3)`" for the first halfword is an
over-simplification of the actual `rlwinm r0,r0,13,0,18` (`extlwi` with
`n = 19, b = 13`); keep the instruction-level form when translating block T.

**Signature.**

```rust
/// Initialises the 0x1048-byte pose/animation object.
/// # Original
/// `FUN_806b7cd0`, `0x806B7CD0`, 1 704 bytes,
/// `work/ghidra/rel-d_basesNP/export/806b7cd0_FUN_806b7cd0.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - object of at least 0x1048 bytes.
/// # Confidence
/// medium - the two `qr3` loads are u16 (GQR3 = 0x00050005); the two `+0xA0`
/// virtuals need dedicated host helpers.
pub unsafe fn fn_806b7cd0(this: usize)
```

**Constants** (as in wave 137, all read from the image): `0x807CAB48 = 0.0`,
`0x807CAB54 = 1.0`, `0x807CAB68 = 1/256`, `0x807CAB6C = 4096.0`,
`0x807CAB70 = 176.0`, `0x807CAB7C = 1.2`, `0x807CAB84 = 0.75`,
`0x807CAB88 = 0.70`, `0x807CAB8C = 0.42`, `0x807CAB90 = 0.28`,
`0x807CAB94 = 0.20`, `0x807CABCC = 10.0`, `0x807CABD4 = 1.0`; slot periods
`0, 28, 44, 49, 55` mod `120`.

**Verdict: conditional (medium)** - GQR3 is resolved; the remaining work is the
two mixed-ABI `+0xA0` virtuals and the `0x800E78E0` registration call.

---

## 2. `0x806DD730` - 736 bytes - `work/ghidra/rel-d_basesNP/export/806dd730_FUN_806dd730.c`

**Body:** `0x806DD730..0x806DDA10` (184 words, `blr` at `0x806DDA0C`).
Ledger: absent. `src/game/bases.rs`: absent. Wave 137: section 11.

Re-selects the pose/offset for the current 348-step cycle. The wave-137
dispatch table (eight `r29` ranges over `[0,80)`, `[80,112)`, `[112,128)`,
`[128,174)`, `[174,254)`, `[254,286)`, `[286,302)`, `[302,348)`) is
authoritative.

**GQR3 block.** Cases `[80,112)` and `[254,286)` write a second halfword
`((r29 - base) & 0x1F) << 11` at `this+0x722` and to the stack, then:

```
0x806DD824  sth   r0,10(r1)
0x806DD828  psq_l f1,0(r4),w1,qr3      ; r4 = r1+0x0A
0x806DD830  lfs   f0,-0x4620(r3)       ; 0x807CB9E0 = 1/256
0x806DD834  fmuls f1,f0,f1
0x806DD838  bl    0x80237BD0           ; sine
```

and the same shape at `0x806DD934`/`0x806DD938` with `r4 = r1+8`. The sine
result is **discarded**: only the two halfword stores survive. With
`GQR3 = 0x00050005` the load is `(u16)halfword`, then `* 1/256`. Transcribe the
store and the call, not the trig value.

**Signature.**

```rust
/// Selects the pose halfwords and the sub-object descriptor for the current
/// 348-step cycle.
/// # Original
/// `FUN_806dd730`, `0x806DD730`, 736 bytes,
/// `work/ghidra/rel-d_basesNP/export/806dd730_FUN_806dd730.c`.
/// # Arguments
/// * `this` - object with the `+0x394` sub-object, `+0x720/+0x722` halfwords,
///   `+0x724` byte and `+0x710` float result.
/// # Confidence
/// medium - the two `qr3` loads are u16 (GQR3 = 0x00050005); the eight BSS
/// descriptors are cited by address.
pub unsafe fn fn_806dd730(this: usize)
```

**Constants:** `0x807CB9E0 = 1/256`; `0x8042A200` is `main.dol` data (cycle
counter, read via `platform::read_u32`); descriptors `0x8082DCE8`,
`0x8082DD28`, `0x8082DD68`, `0x8082DDA8`, `0x8082DDE8`, `0x8082DE28`,
`0x8082DE68`, `0x8082DEA8` (zero-fill, address-only).

**Verdict: ready (medium).**

---

## 3. `0x80680720` - 660 bytes - `work/ghidra/rel-d_basesNP/export/80680720_FUN_80680720.c`

**Body:** `0x80680720..0x806809B4` (165 words, `blr` at `0x806809B0`).
Ledger: absent. `src/game/bases.rs`: absent. Wave 137: section 12.

Rebuilds a two-segment transform/emit state from an angle triple. Wave 137's
blocks A..H stand; four `qr3` loads are substituted as follows:

| site | instruction | stack source |
|---|---|---|
| `0x806807DC` | `psq_l f1,0(r8),w1,qr3` | `r1+14` = `(s16)this[0x104] - angle` |
| `0x8068080C` | `psq_l f1,0(r3),w1,qr3` | `r1+12`, same value |
| `0x806808C0` | `psq_l f1,0(r3),w1,qr3` | second group copy |
| `0x806808F0` | `psq_l f1,0(r3),w1,qr3` | second group copy |

Each is `(u16)halfword * 1/256` (`0x807C96AC = 0x3B800000`), then `0x80237C40`
(cos) / `0x80237BD0` (sin), scaled by `336.0` (`0x807C96A4`) and `8.0`/`328.0`
(`0x807C96A8`). Because the trigger table period is 256, the u16 wrap of a
negative angle is equivalent to the signed value; implement `(angle as u16)`
then `* 1/256`, not `as i16`.

**Signature.**

```rust
/// Rebuilds the two-segment transform/emit state from an angle triple.
/// # Original
/// `FUN_80680720`, `0x80680720`, 660 bytes,
/// `work/ghidra/rel-d_basesNP/export/80680720_FUN_80680720.c`.
/// # Arguments
/// * `this` - emitter with `+0x7C` transform, `+0xAC/+0xB0/+0xB4` position and
///   `+0x104` signed angle halfword.
/// * `actor` - object with `+0x80/+0x82/+0x84` halfwords and a `+0x14` vtable;
///   `actor+0x40` is the second actor.
/// * `angle` - signed offset subtracted from `this[0x104]`.
/// # Confidence
/// medium - the four `qr3` loads are u16 (GQR3 = 0x00050005); all copies and
/// both `+0x14` virtuals are explicit.
pub unsafe fn fn_80680720(this: usize, actor: usize, angle: i32)
```

**Constants:** `0x807C9648 = 1.0`, `0x807C9660 = 400.0`, `0x807C96A0 = 40.0`,
`0x807C96A4 = 336.0`, `0x807C96A8 = 8.0`, `0x807C96AC = 1/256`.

**Verdict: ready (medium).** Caller inside `d_basesNP`: `0x8068046C`.

---

## 4. `0x8072E520` - 644 bytes - `work/ghidra/rel-d_basesNP/export/8072e520_FUN_8072e520.c`

**Body:** `0x8072E520..0x8072E784` (161 words, `blr` at `0x8072E780`).
Ledger: absent. `src/game/bases.rs`: referenced only as an address-only call in
`fn_8072ebc0`/`fn_8072eaa0` comments.

Per-frame burst emitter. `this` is entry `r3`; the save helper is `0x802DCD5C`
(`bl` at `0x8072E548`, `mr r25,r3` at `0x8072E554`). The prologue spills
`f31/f30/f29` through `qr0`.

**GQR3 blocks.** The function picks one of four quarter-turn arms from the
angle at `this+0x934`:

```
0x8072E54C  lha r0,0x934(r3)
r28 = 0; r27 = 0
if (angle == 0x2000 || 0x6000 || 0xA000 || 0xE000) { r28 = 0x2000; r27 = 1 }
for (i = 0; i < 4; i++, r28 += 0x4000) {
    sth r28,10(r1); psq_l f0,0(r23),w1,qr3 ; r23 = r1+0x0A
    f1 = (1/256) * f0 ; f1 = cos(f1) ; f1 *= 16.0 ; pos.x = this[0x8D4] + f1
    sth r28,8(r1) ;  psq_l f0,0(r22),w1,qr3  ; r22 = r1+8
    f1 = (1/256) * f0 ; f1 = sin(f1) ; f1 *= 16.0 ; pos.y = this[0x8D8] + f1
    ...
}
```

| site | instruction | source |
|---|---|---|
| `0x8072E5CC` | `psq_l f0,0(r23),w1,qr3` | `sth r28,10(r1)` (`r28` = 0 or 0x2000, then `+= 0x4000`) |
| `0x8072E5EC` | `psq_l f0,0(r22),w1,qr3` | `sth r28,8(r1)` |

`0x807CD610 = 1/256` and `0x807CD614 = 16.0`. Each arm emits two effects:
`0x80064610(0x15B, id<<4 | 0x200 | (this[4] & 0x300000), &pos, 0, 0)`,
`0x8006A4B0(&out, &pos)`, `0x80198C30(*(u32*)0x80429D60, 0x2A4, &out, 0)`,
where `id = 3` or `5` from `0x807CD5E8`/`0x807CD5F8` and the slot mask is
`(*(u8*)0x807CD608 table)[bits]`. A second loop over four slots
(`this+0x8E0 + 0xC*i`) spawns `"Wm_en_burst_ss"` through
`0x8016C890(0x8080CBE0, 0, &rec, 0, 0)` with `0x807CD618 = 5500.0`, and stores
the selected mask byte at `this+0x954`.

**Signature.**

```rust
/// Emits the four-arm burst effect and the four trailing particle records for
/// the current angle state.
/// # Original
/// `FUN_8072e520`, `0x8072E520`, 644 bytes,
/// `work/ghidra/rel-d_basesNP/export/8072e520_FUN_8072e520.c`.
/// # Arguments
/// * `this` - object with `+0x934` angle, `+0x92C` mode, `+0x8D4/+0x8D8`
///   arm offsets, `+0x8E0` particle records and `+0x954` output mask.
/// # Confidence
/// medium - both `qr3` loads are u16 (GQR3 = 0x00050005); the effect ids are
/// inferred from the shipped tables.
pub unsafe fn fn_8072e520(this: usize)
```

**Constants:** `0x807CD5D0 = 0.0`, `0x807CD5E8 = 3`, `0x807CD5F8 = 5`,
`0x807CD608 = 0x07050D09` (byte array `07 05 0D 09`), `0x807CD610 = 1/256`,
`0x807CD614 = 16.0`, `0x807CD618 = 5500.0`; `0x80429D60` is `main.dol` data.

**Verdict: ready (medium).**

---

## 5. `0x8072BC40` - 604 bytes - `work/ghidra/rel-d_basesNP/export/8072bc40_FUN_8072bc40.c`

**Body:** `0x8072BC40..0x8072BE9C` (151 words, `blr` at `0x8072BE98`).
Ledger: absent. `src/game/bases.rs`: absent.

Rebuilds the two mount transforms from the object position and a signed angle.

```
if (this[0x4D0] == 0) a = (s16)this[0x4A0]; else a = (s16)this[0x4A0] - 0x8000;
sth a,10(r1); psq_l f1,0(r3),w1,qr3;  r3 = r1+8/10
f1 = (1/256) * f1;  cos = 0x80237C40(f1);  sin = 0x80237BD0(f1);
x = this[0x4A8] + (0.0*cos - 13.0*sin);
y = this[0x4AC] + (0.0*sin + 13.0*cos);
```

| site | instruction | source |
|---|---|---|
| `0x8072BDAC` | `psq_l f1,0(r3),w1,qr3` | `sth a,...` (cos pass) |
| `0x8072BDC8` | `psq_l f1,0(r3),w1,qr3` | same halfword (sin pass) |

It then builds three rotation matrices: one from the position via
`0x801C0BD0`, one from `0x801C0BD0(0.0, +8.0, 0.0)`, one from
`0x801C0BD0(0.0, -8.0, 0.0)` (cells `0x807CD480 = 0.0`, `0x807CD484 = 8.0`,
`0x807CD488 = -8.0`), concatenating the last two with `0x801C0540`. It installs
the halfword triple on `this+0x7C` via
`0x8016EDD0`/`0x8016ECB0`/`0x8016EEF0`, calls `0x8016A170`/`0x8016A150`/
`0x8016A1A0`, then `FUN_8072C180(this)`, and repeats the whole block for
`this+0x430` with `FUN_8072BEA0(this)`.

**Signature.**

```rust
/// Rebuilds the two mount transforms from the position and the signed angle.
/// # Original
/// `FUN_8072bc40`, `0x8072BC40`, 604 bytes,
/// `work/ghidra/rel-d_basesNP/export/8072bc40_FUN_8072bc40.c`.
/// # Arguments
/// * `this` - object with `+0xAC..+0xB4` position, `+0x4A0` angle, `+0x4A8/
///   +0x4AC` offsets, `+0x4D0` half-turn flag, and mounts `+0x3F0`/`+0x430`.
/// # Confidence
/// high - both `qr3` loads are u16 (GQR3 = 0x00050005); every call and
/// constant is literal.
pub unsafe fn fn_8072bc40(this: usize)
```

**Constants:** `0x807CD480 = 0.0`, `0x807CD484 = 8.0`, `0x807CD488 = -8.0`,
`0x807CD498 = 13.0`, `0x807CD49C = 1/256`.

**Verdict: ready (high).**

---

## 6. `0x806B9A30` - 476 bytes - `work/ghidra/rel-d_basesNP/export/806b9a30_FUN_806b9a30.c`

**Body:** `0x806B9A30..0x806B9C0C` (119 words, `blr` at `0x806B9C08`).
Ledger: absent. `src/game/bases.rs`: absent.

Rotating/oscillating actor update. `this` = entry `r3` (`mr r31,r3` at
`0x806B9A40`).

```
uVar5 = 0x80162590();                       // global/player lookup
vcall(this+0x434, +0x14)                    // sub-object tick
sVar7 = (s16)0x80C3DFC0(uVar5);             // import, returns an angle
0x8015F4F0(this+0x47E, (s16)this[0x47C], sVar7);   // angle blend
a = (s16)this[0x47E];
this[0xE8] = this[0x10C] * cos((1/256) * a);
this[0xEC] = this[0x10C] * sin((1/256) * a);
0x8006CD40(this);
if (this[0x470] > 0) { this[0x470] -= 1; FUN_806b96f0(this); }
FUN_806b96c0(this);
if (FUN_806b94e0(this) == 0) {
    this[0x480] -= 1;
    if (this[0x480] < 1) { vcall(+0xE4); vcall(this+0x394,+0x18, &DAT_8082C700); }
    else if (this[0x480] % 0x80C3DFB0(uVar5) == 0) {
        if (0x8005F900(this[0x484]) == 0 || (*(u32*)0x80429CA8 & (1 << this[0x484])) == 0)
            this[0x484] = FUN_806b9740(this);
        if (this[0x484] < 4) this[0x47C] = FUN_806b9800(this);
    }
    FUN_806b92c0(this);
} else { vcall(+0xE0); 0x80162510(this); }
```

**GQR3 blocks.**

| site | instruction | source |
|---|---|---|
| `0x806B9A90` | `psq_l f1,0(r3),w1,qr3` | `this+0x47E` (cos) |
| `0x806B9ABC` | `psq_l f1,0(r3),w1,qr3` | same halfword (sin) |

Each is `(u16)halfword * 1/256` (`0x807CAC50 = 0x3B800000`), then
`0x80237C40` / `0x80237BD0`, then `* this[0x10C]`.

**Signature.**

```rust
/// Per-frame update of a rotating actor whose two angles drive `+0xE8/+0xEC`.
/// # Original
/// `FUN_806b9a30`, `0x806B9A30`, 476 bytes,
/// `work/ghidra/rel-d_basesNP/export/806b9a30_FUN_806b9a30.c`.
/// # Arguments
/// * `this` - object with `+0x434` sub-object, `+0x47C/+0x47E` angles,
///   `+0x10C` radius, `+0x470/+0x480` counters and `+0x484` selector.
/// # Confidence
/// medium - both `qr3` loads are u16 (GQR3 = 0x00050005); the two imports and
/// the `0x80429CA8` bitmask are named, not resolved.
pub unsafe fn fn_806b9a30(this: usize)
```

**Constants:** `0x807CAC50 = 1/256`; `0x8082C700` zero-fill descriptor;
`0x80429CA8` is `main.dol` data.

**Verdict: conditional (medium)** - the two `0x80C3DFC0`/`0x80C3DFB0` calls
have synthetic targets outside every shipped image segment (import
relocations), and `0x80429CA8` is a `main.dol` bitmask cell. Everything else is
literal.

---

## 7. `0x80681C50` - 384 bytes - `work/ghidra/rel-d_basesNP/export/80681c50_FUN_80681c50.c`

**Body:** `0x80681C50..0x80681DD0` (96 words, `blr` at `0x80681DCC`).
Ledger: absent. `src/game/bases.rs`: absent.

Four-slot gated pivot update. `this` = entry `r3`; the save helper is
`0x802DCD74` (`bl` at `0x80681C70`, `mr r31,r3` at `0x80681C88`). Ghidra writes
`iVar6 = func_0x802dcd74()`, which is the usual `_savegpr` artifact: the
callee preserves `r3`, so `iVar6 == this`.

```
this[0x4CC] = 0; this[0x4DC..0x4E8] = 0.0; this[0x4EC..0x4EF] = 0;
for (node = this[0x3F4]; node; node = *(u32*)(node+0xC)) {
    obj = *(u32*)(node+4);
    if (obj && (obj[0x38C] == 1 || obj[0x38C] == 2)) {
        p = vcall(obj, vtable+0x6C) as *const i8;   // name/index
        if (*p != -1) this[0x4EC + *p] = 1;
    }
}
for (i = 0; i < 4; i++) {
    if (this[0x4F0+i] == 1 && this[0x4EC+i] == 0) {
        a = (s16)this[0x4D0];
        s = sin((1/256) * (u16)a);
        v = 0.5 * (this[0x4D4] * s);
        if (this[0x348] == 0) v = -v;
        this[0x4DC + 4*i] = v;
        this[0x4CC] = 1;
    }
}
copy this[0x4EC..0x4EF] -> this[0x4F0..0x4F3];
```

**GQR3 block.**

| site | instruction | source |
|---|---|---|
| `0x80681D48` | `psq_l f0,0(r29),w1,qr3` | `r29 = r1+8`; `sth r0,8(r1)` from `lha r0,0x4D0(r31)` |

`f30 = 1/256` is loaded from `0x807C970C`; the load is `(u16)angle * 1/256`,
then `0x80237BD0`.

**Signature.**

```rust
/// Recomputes the four gated pivot offsets from the angle at `+0x4D0`.
/// # Original
/// `FUN_80681c50`, `0x80681C50`, 384 bytes,
/// `work/ghidra/rel-d_basesNP/export/80681c50_FUN_80681c50.c`.
/// # Arguments
/// * `this` - object with the `+0x3F4` node list, `+0x4CC` dirty flag,
///   `+0x4D0` angle, `+0x4D4` rate, `+0x4DC..+0x4EF` slot arrays.
/// # Confidence
/// medium - the `qr3` load is u16 (GQR3 = 0x00050005); the `+0x6C` virtual's
/// return byte is a slot index.
pub unsafe fn fn_80681c50(this: usize)
```

**Constants:** `0x807C96FC = 0.0`, `0x807C970C = 1/256`,
`0x807C9710 = 0.5`.

**Verdict: ready (medium).**

---

## 8. `0x8072C000` - 372 bytes - `work/ghidra/rel-d_basesNP/export/8072c000_FUN_8072c000.c`

**Body:** `0x8072C000..0x8072C174` (93 words, `blr` at `0x8072C170`).
Ledger: absent. `src/game/bases.rs`: referenced only as an address-only call in
`fn_8072ba10`.

Fills `this[0x4CC]` motion records (stride `0xA4`) with radially offset copies
of the object position.

```
if (this[0x4D0] == 0) a = (s16)this[0x4A0]; else a = (s16)this[0x4A0] - 0x8000;
cos = 0x80237C40((1/256) * (u16)a);
sin = 0x80237BD0((1/256) * (u16)a);
radius = 19.0;                                 // 0x807CD4DC, first radius
step = table[this[4] & 7] as f32;              // 0x807CD4C0, 8-entry int table
if ((this[4] >> 8) & 1) step += 0.25;          // 0x807CD4E0
for (i = 0; i < this[0x4CC]; i++, rec += 0xA4) {
    rec[0x4F0] = (this[0x4A8] + (0.0*cos - radius*sin)) - this[0xAC];
    rec[0x4F4] = (this[0x4AC] + (0.0*sin + radius*cos)) - this[0xB0];
    radius += step;
}
```

**GQR3 blocks.**

| site | instruction | source |
|---|---|---|
| `0x8072C0AC` | `psq_l f1,0(r3),w1,qr3` | cos pass, halfword `a` on the stack |
| `0x8072C0C8` | `psq_l f1,0(r3),w1,qr3` | sin pass, same halfword |

`0x807CD49C = 1/256`.

**Signature.**

```rust
/// Rebuilds the per-record radial motion offsets for the current angle.
/// # Original
/// `FUN_8072c000`, `0x8072C000`, 372 bytes,
/// `work/ghidra/rel-d_basesNP/export/8072c000_FUN_8072c000.c`.
/// # Arguments
/// * `this` - object with `+0xAC/+0xB0` position, `+0x4A0` angle, `+0x4A8/
///   +0x4AC` offsets, `+0x4CC` record count, `+0x4D0` half-turn flag.
/// # Confidence
/// medium - both `qr3` loads are u16 (GQR3 = 0x00050005); the radial table is
/// eight `int`s.
pub unsafe fn fn_8072c000(this: usize)
```

**Constants:** the eight words at `0x807CD4C0..0x807CD4DC` are
`{10, 10, 12, 10, 11, 11, 11, 0x41980000}` (the last is the `19.0` cell
`0x807CD4DC`, reachable by the literal `& 7` index; transcribe it literally),
`0x807CD4DC = 19.0`, `0x807CD4E0 = 0.25`, `0x807CD4E8 = 0x43300000`
(2^52 ladder), `0x807CD49C = 1/256`.

**Verdict: ready (medium).**

---

## 9. `0x80688BA0` - 356 bytes - `work/ghidra/rel-d_basesNP/export/80688ba0_FUN_80688ba0.c`

**Body:** `0x80688BA0..0x80688D04` (89 words, `blr` at `0x80688D00`).
Ledger: absent. `src/game/bases.rs`: absent.

Positive-direction oscillation of the vertical bob and lateral offset.

```
old = this[0xB0];  a0 = (s16)this[0x564];
this[0x564] = a0 + 0x200;
b = sin((1/256) * (u16)(a0 + 0x200));
this[0xB0] = this[0x55C] + 8.0 * b;
this[0xEC] = this[0xB0] - old;
FUN_80688920(this);
a1 = (s16)this[0x566];  this[0x566] = a1 + 0x1800;
c = sin((1/256) * (u16)(a1 + 0x1800));
this[0xAC] += 1.25 * c;
this[0x102] += 0x400;  this[0x104] += 0x400;
if (this[0x554] != 0) {
    c_old = cos((1/256) * (u16)a0);
    c_new = cos((1/256) * (u16)this[0x564]);
    if (c_new <= 0.0 && 0.0 < c_old) vcall(this, vtable+0xD4, &DAT_8082AA30);
}
```

**GQR3 blocks.**

| site | instruction | source |
|---|---|---|
| `0x80688BE4` | `psq_l f1,0(r4),w1,qr3` | `+0x564`, sin |
| `0x80688C2C` | `psq_l f1,0(r4),w1,qr3` | `+0x566`, sin |
| `0x80688C7C` | `psq_l f1,0(r3),w1,qr3` | `a0`, cos |
| `0x80688C9C` | `psq_l f1,0(r3),w1,qr3` | new `+0x564`, cos |

All four are `(u16)halfword * 1/256` (`0x807C99E8 = 0x3B800000`), then
`0x80237BD0`/`0x80237C40`.

**Signature.**

```rust
/// Advances the positive-direction bob and lateral oscillation.
/// # Original
/// `FUN_80688ba0`, `0x80688BA0`, 356 bytes,
/// `work/ghidra/rel-d_basesNP/export/80688ba0_FUN_80688ba0.c`.
/// # Arguments
/// * `this` - object with `+0xAC/+0xB0` offsets, `+0x564/+0x566` angles,
///   `+0x55C` base, `+0x554` trigger flag.
/// # Confidence
/// high - every operand is literal; all four `qr3` loads are u16
/// (GQR3 = 0x00050005).
pub unsafe fn fn_80688ba0(this: usize)
```

**Constants:** `0x807C99E8 = 1/256`, `0x807C99EC = 8.0`,
`0x807C99F0 = 1.25`, `0x807C9998 = 0.0`; `0x8082AA30` zero-fill descriptor.

**Verdict: ready (high).**

---

## 10. `0x80688E10` - 356 bytes - `work/ghidra/rel-d_basesNP/export/80688e10_FUN_80688e10.c`

**Body:** `0x80688E10..0x80688F74` (89 words, `blr` at `0x80688F70`).
Ledger: absent. `src/game/bases.rs`: absent.

The mirror twin of #9:

| field | #9 (`0x80688BA0`) | #10 (`0x80688E10`) |
|---|---|---|
| angle step | `+0x564 += 0x200` | `+0x564 -= 0x200` |
| trigger flag | `+0x554` | `+0x550` |
| descriptor | `0x8082AA30` | `0x8082A9B0` |

Everything else (the `+0x55C + 8*sin`, the `+0x566 += 0x1800`, the
`1.25*sin` lateral term, the `+0x400` halfword bumps, the cos zero-crossing
trigger and the four `qr3` sites at `0x80688E54`, `0x80688E9C`, `0x80688EEC`,
`0x80688F0C`) is identical to #9.

**Signature.**

```rust
/// Advances the negative-direction bob and lateral oscillation (mirror of
/// `fn_80688ba0`).
/// # Original
/// `FUN_80688e10`, `0x80688E10`, 356 bytes,
/// `work/ghidra/rel-d_basesNP/export/80688e10_FUN_80688e10.c`.
/// # Arguments
/// * `this` - same layout as `fn_80688ba0`, with `+0x550` as the trigger flag.
/// # Confidence
/// high - all four `qr3` loads are u16 (GQR3 = 0x00050005).
pub unsafe fn fn_80688e10(this: usize)
```

**Constants:** identical to #9 (`1/256`, `8.0`, `1.25`, `0.0`).

**Verdict: ready (high).**

---

## 11. `0x806B2BB0` - 340 bytes - `work/ghidra/rel-d_basesNP/export/806b2bb0_FUN_806b2bb0.c`

**Body:** `0x806B2BB0..0x806B2D04` (85 words, `blr` at `0x806B2D00`).
Ledger: absent. `src/game/bases.rs`: absent.

Timer/limp state update for an object with a vtable at `+0x60` and a second
sub-object at `+0x394`.

```
if (this[0x508]) { vcall(this, vtable+0x98); this[0x508] = 0; this[0x50A] = 1; }
if (this[0x50A]) {
    v = this[0xDC] - 0.125;
    if (v <= 0.0) { this[0x50A] = 0; v = 0.0; }
    this[0xDC] = this[0xE0] = this[0xE4] = v;
}
a = (s16)this[0x506];
if (a != 0) {
    a += 0x800;
    if (a < 0) { a = 0; this[0xDC..0xE4] = 1.0; }
    this[0x506] = a;
}
if (this[0x509]) { this[0x509] = 0; this[0x506] = 0x800; a = 0x800; }
if (a != 0) {
    s = sin((1/256) * (u16)a);
    v = 1.0 + 0.5 * s;
    this[0xDC] = this[0xE0] = this[0xE4] = v;
}
if (this[0x50B]) vcall(this+0x394, vtable+0x18, &DAT_8082C1D8);
```

**GQR3 block.**

| site | instruction | source |
|---|---|---|
| `0x806B2C98` | `psq_l f1,0(r3),w1,qr3` | `r3 = r1+8`; `lha r3,0x506(r30)` then `sth` |

The load is `(u16)angle * 1/256` (`0x807CAA00 = 0x3B800000`), then
`0x80237BD0`; the result is multiplied by `0.5` (`0x807CAA04`) and added to
`1.0` (`0x807CA9F4`).

**Signature.**

```rust
/// Advances the limp/recover timer and the bob derived from `+0x506`.
/// # Original
/// `FUN_806b2bb0`, `0x806B2BB0`, 340 bytes,
/// `work/ghidra/rel-d_basesNP/export/806b2bb0_FUN_806b2bb0.c`.
/// # Arguments
/// * `this` - object with `+0x506` angle, `+0x508..+0x50B` flags,
///   `+0xDC..+0xE4` bob floats, vtable `+0x60`, sub-object `+0x394`.
/// # Confidence
/// high - the `qr3` load is u16 (GQR3 = 0x00050005); all branches literal.
pub unsafe fn fn_806b2bb0(this: usize)
```

**Constants:** `0x807CA9F4 = 1.0`, `0x807CA9F8 = 0.0`,
`0x807CA9FC = -0.125`, `0x807CAA00 = 1/256`, `0x807CAA04 = 0.5`;
`0x8082C1D8` zero-fill descriptor.

**Verdict: ready (high).**

---

## 12. `0x8068EA10` - 332 bytes - `work/ghidra/rel-d_basesNP/export/8068ea10_FUN_8068ea10.c`

**Body:** `0x8068EA10..0x8068EB5C` (83 words, `blr` at `0x8068EB58`).
Ledger: absent. `src/game/bases.rs`: absent.

Four-slot gated pivot update, the sibling of #7 at a different offset. `this` =
entry `r3`; save helper `0x802DCD74` (`bl` at `0x8068EA40`, `mr r27,r3` at
`0x8068EA48`), so Ghidra's `iVar6 = func_0x802dcd74()` is again `this`.

```
this[0xC44] = 0; this[0xC50..0xC5C] = 0.0;
for (i = 0; i < 4; i++) {
    if (this[0xC64+i] == 1 && this[0xC60+i] == 0) {
        a = (s16)this[0xC48];
        s = sin((1/256) * (u16)a);
        v = 0.5 * (160.0 * this[0xC4C] * s);
        if (this[0x348] == 0) v = -v;
        this[0xC50 + 4*i] = v;
        this[0xC44] = 1;
    }
}
copy this[0xC60..0xC63] -> this[0xC64..0xC67]; clear this[0xC60..0xC63];
```

**GQR3 block.**

| site | instruction | source |
|---|---|---|
| `0x8068EAB4` | `psq_l f0,0(r30),w1,qr3` | `r30 = r1+8`; `sth r0,8(r1)` from `lha r0,0xC48(r27)` |

The load is `(u16)angle * 1/256` (`0x807C9C48 = 0x3B800000`), then
`0x80237BD0`; the scale chain is `160.0` (`0x807C9C2C`) * `this[0xC4C]` *
`sin` * `0.5` (`0x807C9C4C`).

**Signature.**

```rust
/// Recomputes the four gated pivot offsets at `+0xC50` from `+0xC48`.
/// # Original
/// `FUN_8068ea10`, `0x8068EA10`, 332 bytes,
/// `work/ghidra/rel-d_basesNP/export/8068ea10_FUN_8068ea10.c`.
/// # Arguments
/// * `this` - object with `+0x348` sign flag, `+0xC44` dirty flag, `+0xC48`
///   angle, `+0xC4C` rate, `+0xC50..+0xC67` slot arrays.
/// # Confidence
/// medium - the `qr3` load is u16 (GQR3 = 0x00050005); the slot array roles
/// are inferred from the copy/clear pattern.
pub unsafe fn fn_8068ea10(this: usize)
```

**Constants:** `0x807C9C2C = 160.0`, `0x807C9C30 = 0.0`,
`0x807C9C48 = 1/256`, `0x807C9C4C = 0.5`.

**Verdict: ready (medium).**

---

## Ready vs conditional

**Ready - implement as specified (10):**
`0x806DD730`, `0x80680720`, `0x8072E520`, `0x8072BC40`, `0x80681C50`,
`0x8072C000`, `0x80688BA0`, `0x80688E10`, `0x806B2BB0`, `0x8068EA10`.

**Conditional - implement as specified, confidence `medium` (2):**

| address | condition |
|---|---|
| `0x806B7CD0` | The two `+0xA0` virtual calls in blocks X/Y use mixed float/integer shapes (one takes `f1/f2`, a spilled vec3 at `r1+0x0C`, `r4/r6` descriptors and `r8`; the other takes `r4/r6` only). No existing helper expresses either; add one host helper per call and keep the argument registers as recorded in wave 137. GQR3 is resolved. |
| `0x806B9A30` | `0x80C3DFC0` and `0x80C3DFB0` are import-relocation targets outside the REL image; `0x80429CA8` is a `main.dol` bitmask, read through `platform::read_u32`. The body is fully specified; the two callee contracts are the remaining unknown. |

**No remaining GQR3 blocker.** The word is fixed for the whole build:
`GQR3 = 0x00050005`, type 5 (`u16`), scale 0, both directions.

## Verification performed

* `ledger/functions.csv` parsed read-only; the address column (`module,address,
  ...`, no `0x`) contains none of the twelve. `0x8072E520` and `0x8072C000`
  occur only as text inside other rows' evidence and as comments in
  `src/game/bases.rs`.
* `src/game/bases.rs`: no `pub unsafe fn fn_<addr>` and no `fn_<addr>` definition
  for any of the twelve. The two mentions are address-only call comments in
  `fn_8072ebc0`/`fn_8072eaa0` and `fn_8072ba10`.
* Prior specs: `docs/SPEC_WAVE128_BASES.md` .. `docs/SPEC_WAVE137_BASES.md`
  contain `##` titles for `0x806B7CD0`, `0x806DD730`, `0x80680720` (wave 137,
  conditional on GQR3) and none for the other nine. This wave supersedes only
  those three rows' GQR3 verdict; their block tables remain valid.
* `work/ghidra/rel-d_basesNP/export/functions.tsv`: all twelve byte counts were
  read from the `size` column and re-checked against the decoded `blr` extent.
* Raw decode: `work/rels/elf/d_basesNP.elf`, `PT_LOAD` table re-read, formula
  `file_offset = (address - 0x80600000) + 0x1000`; a throwaway PowerPC decoder
  kept at `%TEMP%\opencode\ppc138.js` decoded `psq_l`/`psq_st` (opcode 56/60,
  `I` bits 12-14, `W` bit 15, `DQ` bits 0-11), `mtspr`/`mfspr`, `bl`/`b`/`bc`,
  and the integer/float load-store set.
* Extent: last word `0x4E800020` (`blr`) for all twelve; no `halt_baddata`, no
  jump table, no `functions.tsv` row starting inside a body.
* Duplicate check: SHA-1 of each of the twelve ranges against every same-size
  `d_basesNP` TSV row - **no match**.
* `main.dol` GQR scan: every text word with opcode 31 / XO 467 was decoded;
  37 hits to SPR 912-919, six groups (table above). The only non-zero GQR3
  constant is `0x00050005`, written at `0x8025EEF4`, `0x802AB518`,
  `0x802C277C`.
* GQR field layout cross-checked against the image's own decompiler model in
  the exports (`(byte)GQR & 7` for load type, `GQR & 0x3f00` for load scale,
  `(GQR >> 16) & 7` and `GQR & 0x3f000000` for store), and against
  `docs/COVERAGE_WAVE106_WORLDMAP.md`, which independently records
  `0x00050005` / type 5 / scale 0.
* Constants were read through the program headers and are quoted with their
  image addresses; `0x8082xxxx`/`0x8083xxxx` cells that fall in the zero-fill
  segment are cited as addresses only.

## Hand-off

* The remaining 12 `qr3` class-C rows (the ones below the twelve) are:
  `0x8066C9A0` (232), `0x80680E80` (292), `0x806B2D10` (200),
  `0x806B3770` (308), `0x806FB610` (296), `0x806D65A0` (112),
  `0x807040C0` (112), `0x806DE0B0` (132), `0x806DE320` (132),
  `0x806D6530` (104), `0x80704050` (108), `0x806DDA10` (88). All are the same
  `u16` pattern; `0x806DDA10` is the callee of seven cases in #2 and should be
  taken with it.
* `0x807BD110` (2 908) remains the `dFileSelect_c` state table and needs the
  registration-table generator, not this lane.
* The class-D block of `docs/ANALYSIS_WAVE127_BASES.md` (173 addresses) is
  unchanged and untouched.
