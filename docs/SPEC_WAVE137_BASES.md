# Wave 137 - `game::bases` - implementation spec for 9 class-B and 3 class-C addresses

Read-only specification lane. No Rust file, ledger row or existing document was
modified. The only file written is this one.

`docs/ANALYSIS_WAVE127_BASES.md` reclassified 121 `d_basesNP` skips as class B
(51 060 bytes). Waves 128-136 turned 112 of them into `##` sections, and the
parallel translation lanes merged those bodies into `src/game/bases.rs` with
`game::bases` rows in `ledger/functions.csv`. **Nine class-B addresses are
left** (528 of the 51 060 bytes), and this wave takes all nine. Because the brief
allows fewer than twelve, the last three slots fall back to the next-largest
still-untranslated addresses that carry an export - which, inside this lane's
evidence base, are the three largest class-C bodies of the wave-127 report.

## The twelve

| # | address | bytes | words | export file | class | verdict |
|--:|---:|---:|---:|---|:--:|---|
| 1 | `0x80600AD0` | 64 | 16 | `80600ad0_FUN_80600ad0.c` | B | ready (high) |
| 2 | `0x80601D10` | 64 | 16 | `80601d10_FUN_80601d10.c` | B | ready (high) |
| 3 | `0x80601EB0` | 64 | 16 | `80601eb0_FUN_80601eb0.c` | B | ready (high) |
| 4 | `0x80605500` | 64 | 16 | `80605500_FUN_80605500.c` | B | ready (high) |
| 5 | `0x806FCC60` | 64 | 16 | `806fcc60_FUN_806fcc60.c` | B | ready (high) |
| 6 | `0x806FCD70` | 64 | 16 | `806fcd70_FUN_806fcd70.c` | B | ready (high) |
| 7 | `0x806FCDB0` | 64 | 16 | `806fcdb0_FUN_806fcdb0.c` | B | ready (high) |
| 8 | `0x8069ACA0` | 40 | 10 | `8069aca0_FUN_8069aca0.c` | B | ready (high) |
| 9 | `0x806FF700` | 40 | 10 | `806ff700_FUN_806ff700.c` | B | ready (high) |
| 10 | `0x806B7CD0` | 1704 | 426 | `806b7cd0_FUN_806b7cd0.c` | C | conditional (medium) |
| 11 | `0x806DD730` | 736 | 184 | `806dd730_FUN_806dd730.c` | C | conditional (medium) |
| 12 | `0x80680720` | 660 | 165 | `80680720_FUN_80680720.c` | C | conditional (medium) |

Together **3 628 bytes** (907 words). Every export path is exactly
`work/ghidra/rel-d_basesNP/export/<file>` relative to the repository root, and
every `bytes` value is the `size` column of that directory's `functions.tsv`
verbatim.

**Verdict split: 9 ready, 3 conditional.** The three conditional rows are
blocked by a single shared constant - the GQR3 quantiser word set by
`main.dol`'s `OSInitFastCast` - and not by anything in this REL. Every operand,
call target, branch and store in all twelve bodies is explicit in the shipped
stream.

### Selection rule

The candidate set is: **class B in wave 127** (or, in the fallback, the next
largest analysed address), **no row anywhere in `ledger/functions.csv` whose
address field equals the address**, **no `##` title in any
`docs/SPEC_WAVE*_BASES.md` that names the address**, and **no `fn_<addr>`
definition or mention in `src/game/bases.rs`**. The nine surviving class-B rows
are exactly the "next nine after the cut" table of
`docs/SPEC_WAVE136_BASES.md`:

| address | ledger rows | wave-127 class | prior skip reason |
|---|---:|---|---|
| `0x80600AD0` | 0 | B | wave 116 - delete wrappers ("sixth copy of the 8 shape") |
| `0x80601D10` | 0 | B | wave 116 - delete wrappers |
| `0x80601EB0` | 0 | B | wave 116 - delete wrappers |
| `0x80605500` | 0 | B | wave 116 - delete wrappers |
| `0x806FCC60` | 0 | B | wave 113 - `0x806FCC60..0x806FCDF0` neighbours not in this window |
| `0x806FCD70` | 0 | B | wave 113 - same window |
| `0x806FCDB0` | 0 | B | wave 113 - same window |
| `0x8069ACA0` | 0 | B | wave 119 - unrecovered jump table (spurious) |
| `0x806FF700` | 0 | B | wave 11 - wire-mesh lifecycle |

"Ledger rows" is an exact address-field match in any module, not just
`game::bases`. None of the nine appears in the class-C or class-D blocks of
`docs/ANALYSIS_WAVE127_BASES.md`, and none has a `##` heading in
`docs/SPEC_WAVE128_BASES.md` .. `docs/SPEC_WAVE136_BASES.md`.

**Concurrency.** A parallel wave-137 translation lane selected the same nine
class-B addresses and merged bodies for them into `src/game/bases.rs` with rows
in `ledger/agents/wave137-bases.csv` while this document was being written
(`src/game/bases.rs` mtime 22:09, this file 22:22). That does not change the
selection, and it is independent confirmation that the nine are the right cut.
The three class-C rows were not touched by that lane. Every "Ledger: absent"
line below therefore means **absent from `ledger/functions.csv`** (the merged
ledger, which the wave-137 agent CSV has not yet reached); the per-body
cross-check against the merged Rust is in "Verification performed".

| this lane | merged `src/game/bases.rs` | merged signature |
|---|---|---|
| `0x80600AD0` | `fn_80600ad0`, line 173456 | `(this: usize, free: i32) -> usize` |
| `0x80601D10` | `fn_80601d10`, line 173486 | `(this: usize, free: i32) -> usize` |
| `0x80601EB0` | `fn_80601eb0`, line 173516 | `(this: usize, free: i32) -> usize` |
| `0x80605500` | `fn_80605500`, line 173546 | `(this: usize, free: i32) -> usize` |
| `0x806FCC60` | `fn_806fcc60`, line 173363 | `(this: usize, free: i32) -> usize` |
| `0x806FCD70` | `fn_806fcd70`, line 173395 | `(this: usize, free: i32) -> usize` |
| `0x806FCDB0` | `fn_806fcdb0`, line 173425 | `(this: usize, free: i32) -> usize` |
| `0x8069ACA0` | `fn_8069aca0`, line 173581 | `(this: usize) -> u32` |
| `0x806FF700` | `fn_806ff700`, line 173619 | `(this: usize) -> u32` |
| `0x806B7CD0`, `0x806DD730`, `0x80680720` | not merged | - |

The merged bodies agree with the specifications below word for word; the only
difference is the first parameter's name (`this` in the merge, `state` in
sections 1-7 here - rename to `this` for consistency when re-deriving).

### The fallback, and what it deliberately skips

The remaining analysed corpus after the nine class-B rows is 198 addresses: 25
class C and 173 class D. Class D is excluded by construction (thunks,
`halt_baddata` cuts, duplicate getters - none is a specifiable body). The three
largest class-C bodies are taken. Two larger *analysed* addresses are skipped on
evidence, and one is handed off:

| address | bytes | why not in this batch |
|---|---:|---|
| `0x807BD110` | 2 908 | Not a function body. The wave-127 report classes it "the `dFileSelect_c` 44-entry state table ... needs the registration-table generator". The export opens with three bogus `ldexpf(unaff_GQR0 & 0x3f)` guards and then writes ~700 data words; there is no instruction-level behaviour to specify as `fn_`, and the three signatures of the wave-129..136 gate cannot express it. Handed off as a table-generator task. |
| `0x8079C330` | 4 232 | Larger in the raw TSV, but not analysed: `docs/COVERAGE_WAVE91_BASES.md:149` and `docs/COVERAGE_WAVE135_BASES.md:145` both record it as "the large constructor/destructor pair ... needs its own call graph" and it has never been opened. A spec written from one pass would not be implementation-ready, which is this lane's bar. |
| `0x8079D3C0` | 3 176 | Same pair, same citation (`COVERAGE_WAVE91_BASES.md:149`). |

The three chosen class-C rows are the largest untranslated *code* bodies of the
wave-127 report for which an exact body can be written today:

| address | bytes | blocker |
|---|---:|---|
| `0x806B7CD0` | 1 704 | two `psq_l ...,qr3,w1` data ops |
| `0x806DD730` | 736 | two `psq_l ...,qr3,w1` data ops |
| `0x80680720` | 660 | four `psq_l ...,qr3,w1` data ops |

Everything else in those three bodies is integer or plain single-precision work.

## Method

**Address mapping (re-verified, not copied).** `d_basesNP.elf` has six `PT_LOAD`
headers, read this pass:

| `p_offset` | `p_vaddr` | `filesz` | `memsz` | end |
|---|---:|---:|---:|---:|
| `0x10F0` | `0x806000F0` | `0x1C6004` | `0x1C6004` | `0x807C60F4` |
| `0x1C70F4` | `0x807C60F4` | `0x4FC` | `0x4FC` | `0x807C65F0` |
| `0x1C75F0` | `0x807C65F0` | `0xC` | `0xC` | `0x807C65FC` |
| `0x1C7600` | `0x807C6600` | `0xA5FC` | `0xA5FC` | `0x807D0BFC` |
| `0x1D1C00` | `0x807D0C00` | `0x51CB8` | `0x51CB8` | `0x808228B8` |
| `0` | `0x80822900` | `0` | `0x12484` | `0x80834D84` |

so `file_offset = 0x10F0 + (address - 0x806000F0) = (address - 0x80600000) +
0x1000`, which agrees with `docs/REL_ELF_MAPPING.md`. Everything at or above
`0x80822900` is the zero-fill (BSS) segment and has no shipped bytes; the
`0x8082DCE8..0x8082DEA8` descriptor block and `0x8082B5AC` live there and are
cited as addresses, never as literals.

**Extent check.** For all twelve the reachable span equals the TSV size: the last
word of every range is `0x4E800020` (`blr`) - except the honest `0x4E800420`
(`bcctr`, `0x8069ACA0`, which is a tail call and therefore has no `blr`) - and
no `functions.tsv` row starts inside a body. No `halt_baddata`, no jump table.

**Duplicate check.** The SHA-1 of each of the twelve ranges was compared against
every `d_basesNP` TSV row of the same size: **no match**. The seven 64-byte
bodies are nevertheless *word* twins of `fn_80651350`
(`src/game/bases.rs:4366`): all seven are identical to it in 15 of 16 words, and
word index 9 (the single `bl`) is the only difference:

| this lane | word 9 | target |
|---|---|---|
| `0x80600AD0` | `4BCB878D` | `0x802B9280` |
| `0x80601D10` | `4BCB754D` | `0x802B9280` |
| `0x80601EB0` | `4BCB73AD` | `0x802B9280` |
| `0x80605500` | `4BCB3D5D` | `0x802B9280` |
| `0x806FCC60` | `4BBBC5FD` | `0x802B9280` |
| `0x806FCD70` | `4BBBC4ED` | `0x802B9280` |
| `0x806FCDB0` | `4BBBC4AD` | `0x802B9280` |

(`fn_80651350`'s own word 9 is `4BC67F0D`, same target. `0x80600990` and
`0x80600A90` from wave 136 are `4BCB88CD` and `4BCB87CD`.) So each of the seven
needs its own `pub unsafe fn`, but they are seven copies of one body.

**Paired-single audit.** 16 `psq` words exist in this batch, all in the three
class-C bodies: 4 `psq_st ...,qr0,w0` prologue saves (f30/f31 spills) and 4
`psq_l ...,qr0,w0` epilogue restores (type 0, no data quantisation), plus **8**
`psq_l ...,qr3,w1` data loads. There is **no `mtspr GQRn` anywhere in
`d_basesNP`** and none in these bodies, so the `qr0` saves are plain float pairs
and the `qr3` loads depend on the one external constant analysed below.

**GQR3 (the one shared blocker).** Every `psq_l ...,qr3,w1` in this batch is
immediately followed by `fmuls` with `1/256` (`0x3B800000`, from `0x807C96AC`,
`0x807CB9E0` or `0x807CAB68`), and every source halfword was produced by `sth`
of a signed-16-bit expression. Encoding (verified against
`docs/REL_ELF_MAPPING.md` and by decoding bits 12-14/15/16-20/0-11 of each word):
index 3, `W = 1`, base and `DQ` explicit. Under `psq_l` with `W = 1` the GQR
type must be a 16-bit integer type, so the only self-consistent descriptor is
**type 7 (S16), load scale 8**, i.e. `GQR3.load = 0x0807`; the store half is
never used by these bodies. That value is *inferred from the compensation
multiplier*, not read from `main.dol`; the recommended host form is

```rust
// unknown: GQR3 is installed by main.dol OSInitFastCast; the code multiplies
// every qr3 conversion by 2^-8, so the load descriptor is 0x0807 (S16, scale 8).
const GQR3_LOAD_SCALE: f32 = 256.0;
```

and each site is written below as `(s16)x as f32` (the mathematically equivalent
form once `GQR3_LOAD_SCALE` is applied). If the wave that pins `OSInitFastCast`
finds a different descriptor, multiply the parenthesised expression by
`2^scale / 256`. This is why rows 10-12 are **conditional** and not ready.

**House style used below.** `pub unsafe fn fn_XXXXXXXX(this: usize, ...)`,
`platform::read_u32/read_i32/read_f32/read_u8`, `platform::write_u32/write_u8/
write_f32`, `abi_call1!..abi_call6!`, `vmethod0/vmethod1`, `f32_of_f32(addr, a)`
(`src/game/bases.rs:98817`), `ptr_at(addr)`, addresses written `0x8060_0AD0`,
field offsets as `usize` literals, `// unknown:` on the first use of every
address-only target or external cell.

---

## 1. `0x80600AD0` - 64 bytes - `work/ghidra/rel-d_basesNP/export/80600ad0_FUN_80600ad0.c`

**Body:** `0x80600AD0..0x80600B0C` (16 words, `blr` at `0x80600B08`).
Ledger: absent. Spec headings: none. `src/game/bases.rs`: absent.

### Instruction-level behaviour

```
0x80600AD0  9421FFF0  stwu  r1,-16(r1)
0x80600AD4  7C0802A6  mfspr r0,LR
0x80600AD8  2C030000  cmpi  cr0,r3,0        ; a == 0 ?
0x80600ADC  90010014  stw   r0,20(r1)       ; caller-SP+4 = LR slot
0x80600AE0  93E1000C  stw   r31,12(r1)
0x80600AE4  7C7F1B78  mr    r31,r3          ; r31 = a
0x80600AE8  41820010  beq   0x80600AF8      ; if a == 0 -> return a
0x80600AEC  2C040000  cmpi  cr0,r4,0        ; signed compare
0x80600AF0  40810008  ble   0x80600AF8      ; if b <= 0 -> return a
0x80600AF4  4BCB878D  bl    0x802B9280      ; FUN_802b9280(a)
0x80600AF8  7FE3FB78  mr    r3,r31          ; result = a
0x80600AFC  83E1000C  lwz   r31,12(r1)
0x80600B00  80010014  lwz   r0,20(r1)
0x80600B04  7C0803A6  mtspr LR,r0
0x80600B08  38210010  addi  r1,r1,16
0x80600B0C  4E800020  blr
```

`r3` is never modified before the call, so the callee receives the *entry* `a` -
this is not a "dropped receiver" body; the Ghidra export prints
`func_0x802b9280()` with no argument because the callee is unprototyped. The
call's return value is discarded; the function returns its own first argument.

`0x802B9280` is a main.dol stub (TSV extent 8 bytes): `FUN_802b9280(p)` is
`FUN_802b8f70(p, 0)` (`work/ghidra/main-gb/export/802b9280_FUN_802b9280.c`).

### Proposed signature

```rust
/// Releases `state` when both guards pass.
///
/// # Original
/// `FUN_80600ad0`, `0x80600AD0`, 64 bytes,
/// `work/ghidra/rel-d_basesNP/export/80600ad0_FUN_80600ad0.c`.
/// Synthetic base `0x80600000`; not a runtime address.
///
/// # Arguments
/// * `state` - object to release; `0` is accepted and skipped.
/// * `free` - signed flag; the release only runs when it is positive.
///
/// # Returns
/// `state`, unchanged.
///
/// # Confidence
/// high - the entry `r3` survives into the `bl`, and all 16 words are read.
pub unsafe fn fn_80600ad0(state: usize, free: i32) -> usize {
    if state != 0 && free > 0 {
        abi_call1!(0x802B_9280, state);
    }
    state
}
```

Word-for-word the same body as `fn_80651350` (`src/game/bases.rs:4366`); reuse
its doc block and change only the address/`bl` evidence. Constant: table word
`0x807D0FC0` holds this address (vtable/registration slot, read-only reference).

**Verdict: ready (high).**

---

## 2. `0x80601D10` - 64 bytes - `work/ghidra/rel-d_basesNP/export/80601d10_FUN_80601d10.c`

**Body:** `0x80601D10..0x80601D4C` (16 words, `blr` at `0x80601D48`).
Ledger: absent. Spec headings: none. `src/game/bases.rs`: absent.

Byte-identical to body #1 except word index 9:

```
0x80601D34  4BCB754D  bl 0x802B9280
```

All other 15 words are `9421FFF0 7C0802A6 2C030000 90010014 93E1000C
7C7F1B78 41820010 2C040000 40810008 <bl> 7FE3FB78 83E1000C 80010014
7C0803A6 38210010 4E800020`. Behaviour, guards, return value and callee are
exactly as in #1.

### Proposed signature

```rust
/// Same release stub as [`fn_80600ad0`], at a second site.
///
/// # Original
/// `FUN_80601d10`, `0x80601D10`, 64 bytes,
/// `work/ghidra/rel-d_basesNP/export/80601d10_FUN_80601d10.c`.
///
/// # Confidence
/// high - 15 of 16 words equal [`fn_80600ad0`]; only the `bl` displacement
/// differs.
pub unsafe fn fn_80601d10(state: usize, free: i32) -> usize {
    if state != 0 && free > 0 {
        abi_call1!(0x802B_9280, state);
    }
    state
}
```

**Verdict: ready (high).**

---

## 3. `0x80601EB0` - 64 bytes - `work/ghidra/rel-d_basesNP/export/80601eb0_FUN_80601eb0.c`

**Body:** `0x80601EB0..0x80601EEC` (16 words, `blr` at `0x80601EE8`).
Ledger: absent. Spec headings: none. `src/game/bases.rs`: absent.

Byte-identical to body #1 except word index 9:

```
0x80601ED4  4BCB73AD  bl 0x802B9280
```

Constant: table word `0x807D14E8` holds this address.

### Proposed signature

```rust
/// Same release stub as [`fn_80600ad0`], at a third site.
///
/// # Original
/// `FUN_80601eb0`, `0x80601EB0`, 64 bytes,
/// `work/ghidra/rel-d_basesNP/export/80601eb0_FUN_80601eb0.c`.
///
/// # Confidence
/// high - 15 of 16 words equal [`fn_80600ad0`]; only the `bl` displacement
/// differs.
pub unsafe fn fn_80601eb0(state: usize, free: i32) -> usize {
    if state != 0 && free > 0 {
        abi_call1!(0x802B_9280, state);
    }
    state
}
```

**Verdict: ready (high).**

---

## 4. `0x80605500` - 64 bytes - `work/ghidra/rel-d_basesNP/export/80605500_FUN_80605500.c`

**Body:** `0x80605500..0x8060553C` (16 words, `blr` at `0x80605538`).
Ledger: absent. Spec headings: none. `src/game/bases.rs`: absent.

Byte-identical to body #1 except word index 9:

```
0x80605524  4BCB3D5D  bl 0x802B9280
```

Constant: table word `0x807D1F68` holds this address.

### Proposed signature

```rust
/// Same release stub as [`fn_80600ad0`], at a fourth site.
///
/// # Original
/// `FUN_80605500`, `0x80605500`, 64 bytes,
/// `work/ghidra/rel-d_basesNP/export/80605500_FUN_80605500.c`.
///
/// # Confidence
/// high - 15 of 16 words equal [`fn_80600ad0`]; only the `bl` displacement
/// differs.
pub unsafe fn fn_80605500(state: usize, free: i32) -> usize {
    if state != 0 && free > 0 {
        abi_call1!(0x802B_9280, state);
    }
    state
}
```

**Verdict: ready (high).**

---

## 5. `0x806FCC60` - 64 bytes - `work/ghidra/rel-d_basesNP/export/806fcc60_FUN_806fcc60.c`

**Body:** `0x806FCC60..0x806FCC9C` (16 words, `blr` at `0x806FCC9C` is the 16th
word; range ends `0x806FCC9C + 4`). Ledger: absent. Spec headings: none.
`src/game/bases.rs`: absent.

Byte-identical to body #1 except word index 9:

```
0x806FCC84  4BBBC5FD  bl 0x802B9280
```

No data word in the image holds `0x806FCC60`; the only reference is indirect.

### Proposed signature

```rust
/// Same release stub as [`fn_80600ad0`], at a fifth site.
///
/// # Original
/// `FUN_806fcc60`, `0x806FCC60`, 64 bytes,
/// `work/ghidra/rel-d_basesNP/export/806fcc60_FUN_806fcc60.c`.
///
/// # Confidence
/// high - 15 of 16 words equal [`fn_80600ad0`]; only the `bl` displacement
/// differs.
pub unsafe fn fn_806fcc60(state: usize, free: i32) -> usize {
    if state != 0 && free > 0 {
        abi_call1!(0x802B_9280, state);
    }
    state
}
```

**Verdict: ready (high).**

---

## 6. `0x806FCD70` - 64 bytes - `work/ghidra/rel-d_basesNP/export/806fcd70_FUN_806fcd70.c`

**Body:** `0x806FCD70..0x806FCDAC` (16 words, `blr` at `0x806FCDAC`).
Ledger: absent. Spec headings: none. `src/game/bases.rs`: absent.

Byte-identical to body #1 except word index 9:

```
0x806FCD94  4BBBC4ED  bl 0x802B9280
```

Constant: table word `0x808034C0` holds this address.

### Proposed signature

```rust
/// Same release stub as [`fn_80600ad0`], at a sixth site.
///
/// # Original
/// `FUN_806fcd70`, `0x806FCD70`, 64 bytes,
/// `work/ghidra/rel-d_basesNP/export/806fcd70_FUN_806fcd70.c`.
///
/// # Confidence
/// high - 15 of 16 words equal [`fn_80600ad0`]; only the `bl` displacement
/// differs.
pub unsafe fn fn_806fcd70(state: usize, free: i32) -> usize {
    if state != 0 && free > 0 {
        abi_call1!(0x802B_9280, state);
    }
    state
}
```

**Verdict: ready (high).**

---

## 7. `0x806FCDB0` - 64 bytes - `work/ghidra/rel-d_basesNP/export/806fcdb0_FUN_806fcdb0.c`

**Body:** `0x806FCDB0..0x806FCDEC` (16 words, `blr` at `0x806FCDEC`).
Ledger: absent. Spec headings: none. `src/game/bases.rs`: absent.

Byte-identical to body #1 except word index 9:

```
0x806FCDD4  4BBBC4AD  bl 0x802B9280
```

Constant: table word `0x808034D8` holds this address (`0x808034C0 + 0x18`, i.e.
the next slot of the same table as body #6).

### Proposed signature

```rust
/// Same release stub as [`fn_80600ad0`], at a seventh site.
///
/// # Original
/// `FUN_806fcdb0`, `0x806FCDB0`, 64 bytes,
/// `work/ghidra/rel-d_basesNP/export/806fcdb0_FUN_806fcdb0.c`.
///
/// # Confidence
/// high - 15 of 16 words equal [`fn_80600ad0`]; only the `bl` displacement
/// differs.
pub unsafe fn fn_806fcdb0(state: usize, free: i32) -> usize {
    if state != 0 && free > 0 {
        abi_call1!(0x802B_9280, state);
    }
    state
}
```

**Verdict: ready (high).**

---

## 8. `0x8069ACA0` - 40 bytes - `work/ghidra/rel-d_basesNP/export/8069aca0_FUN_8069aca0.c`

**Body:** `0x8069ACA0..0x8069ACC8` (10 words; the last word is `bcctr`, a tail
call, so there is no `blr`). Ledger: absent. Spec headings: none.
`src/game/bases.rs`: absent.

### Instruction-level behaviour

```
0x8069ACA0  3CA0807F  lis   r5,0x807F           ; r5 = 0x807F0000
0x8069ACA4  3C808083  lis   r4,0x8083           ; r4 = 0x80830000
0x8069ACA8  38A51340  addi  r5,r5,0x1340        ; r5 = 0x807F1340
0x8069ACAC  A8050020  lha   r0,0x20(r5)         ; r0 = (i32)(i16)*(0x807F1360)
0x8069ACB0  3884B5AC  addi  r4,r4,-0x4A5C       ; r4 = 0x8082B5AC
0x8069ACB4  90030D84  stw   r0,0xD84(r3)        ; this[0xD84] = 10
0x8069ACB8  81830060  lwz   r12,0x60(r3)        ; vtable = *(this+0x60)
0x8069ACBC  818C00D4  lwz   r12,0xD4(r12)       ; fn = vtable[0xD4]
0x8069ACC0  7D8903A6  mtspr CTR,r12
0x8069ACC4  4E800420  bcctr                     ; tail call fn(this, r4)
```

Facts:

* `0x807F1340` is a data base; the value read is the signed halfword at
  `0x807F1360`, which is **10** in the shipped image (`0x000A`). `lha`
  sign-extends it into `r0`, and `stw` stores the full 32-bit `0x0000000A`.
* `0x8082B5AC` is in this REL's zero-fill (BSS) segment (`0x80822900`,
  `filesz = 0`): a writable, zero-initialised cell. It is the *second argument*
  of the tail call - the `addi` is not dead.
* The vtable is reached through `+0x60`, **not** through `+0x0`, so
  `vmethod0/vmethod1` (which read `ptr_at(obj)`) must not be used here.
* `bcctr` transfers control without linking: the callee returns straight to
  this function's caller. A faithful Rust body must perform the call and then
  return; the callee's return value becomes this function's return value.
* Ghidra's "Could not recover jumptable at 0x8069acc4" warning is spurious - the
  three preceding words are a plain `lwz/lwz/mtctr`.

### Proposed signature

```rust
/// Seeds the `+0xD84` word from a table and forwards to virtual `+0xD4`.
///
/// # Original
/// `FUN_8069aca0`, `0x8069ACA0`, 40 bytes,
/// `work/ghidra/rel-d_basesNP/export/8069aca0_FUN_8069aca0.c`.
/// Synthetic base `0x80600000`; not a runtime address.
///
/// # Arguments
/// * `this` - object; must expose a live vtable at `+0x60`.
///
/// # Side effects
/// Writes the sign-extended halfword from `0x807F1360` (value `10`) to
/// `this+0xD84`, then tail-calls `vtbl[0xD4](this, 0x8082B5AC)`.
///
/// # Returns
/// Whatever the tail-called virtual returns.
///
/// # Confidence
/// high - every operand is a literal in the stream; only the meaning of the
/// `+0xD84` field is unknown.
pub unsafe fn fn_8069aca0(this: usize) -> u32 {
    // observed: the value is a *halfword* at 0x807F1360 (the base 0x807F1340
    // plus the `lha` displacement 0x20), sign-extended by `lha`.
    let id = platform::read_u16(0x807F_1360) as i16 as i32;
    platform::write_u32(this + 0xD84, id as u32);

    // unknown: 0x8082B5AC is a zero-initialised buffer in this REL's BSS.
    let vtable = ptr_at(this + 0x60);
    let function = ptr_at(vtable + 0xD4);
    abi_call2!(function, this, 0x8082_B5AC)
}
```

Callers inside `d_basesNP`: `0x8069C48C` and `0x8069C8DC` (both `bl`).

**Verdict: ready (high).**

---

## 9. `0x806FF700` - 40 bytes - `work/ghidra/rel-d_basesNP/export/806ff700_FUN_806ff700.c`

**Body:** `0x806FF700..0x806FF728` (10 words, `blr` at `0x806FF724`).
Ledger: absent. Spec headings: none. `src/game/bases.rs`: absent.

### Instruction-level behaviour

```
0x806FF700  9421FFF0  stwu  r1,-16(r1)
0x806FF704  7C0802A6  mfspr r0,LR
0x806FF708  386303C0  addi  r3,r3,0x3C0      ; arg = this + 0x3C0
0x806FF70C  90010014  stw   r0,20(r1)
0x806FF710  4B980241  bl    0x8007F950      ; FUN_8007f950(this+0x3C0)
0x806FF714  80010014  lwz   r0,20(r1)
0x806FF718  38600001  li    r3,1            ; return 1 (the call result is dropped)
0x806FF71C  7C0803A6  mtspr LR,r0
0x806FF720  38210010  addi  r1,r1,16
0x806FF724  4E800020  blr
```

The sub-object address `this + 0x3C0` is passed as the first argument; the
callee's return value is not used. The function always returns the literal `1`
(zero-extended `li`; 16-byte frame, one saved register).

### Proposed signature

```rust
/// Disposes the `+0x3C0` helper and reports success.
///
/// # Original
/// `FUN_806ff700`, `0x806FF700`, 40 bytes,
/// `work/ghidra/rel-d_basesNP/export/806ff700_FUN_806ff700.c`.
/// Synthetic base `0x80600000`; not a runtime address.
///
/// # Arguments
/// * `this` - owner; the sub-object at `+0x3C0` is the only thing touched.
///
/// # Returns
/// Always `1`.
///
/// # Confidence
/// high - the entire body is five instructions plus the frame.
pub unsafe fn fn_806ff700(this: usize) -> u32 {
    abi_call1!(0x8007_F950, this + 0x3C0);
    1
}
```

Constant: table word `0x8080385C` holds this address. The companion
`0x806FF3F0` (already specified in `docs/SPEC_WAVE136_BASES.md` §7) is the
88-byte wire-mesh release body with the same guard; this one is only the
`+0x3C0` release plus the `1` result.

**Verdict: ready (high).**

---

## 10. `0x806B7CD0` - 1 704 bytes - `work/ghidra/rel-d_basesNP/export/806b7cd0_FUN_806b7cd0.c`

**Body:** `0x806B7CD0..0x806B8378` (426 words, `blr` at `0x806B8374`).
Ledger: absent. Spec headings: none. `src/game/bases.rs`: absent.
Export warnings: four "Removing unreachable block" notes
(`0x806b8358`, `0x806b8350`, `0x806b7f94`, `0x806b7f78`), all of which are live
code on the `bdnz`/branch paths - the raw stream keeps them.

This is a `0x1048`-byte-object initialiser (the last field written is
`+0x1044`). Sixteen distinct external calls are visible: `0x802DCD68` (save
helper), `0x806B7830` (×2), `0x806B7800`, `0x8008C3E0`, `0x8008C330`,
`0x8006D090`, `0x80075270`, `0x806B7340`, `0x800E78E0`, `0x8006FA40`,
`0x8006EF40`, `0x8006DB30`, `0x806B7690`, `0x80237C40`, `0x80237BD0`,
`0x802DCDB4`, plus two virtual calls (17 call sites).

### Constants (all read from the shipped image)

| cell | value | role |
|---|---|---|
| `0x807CAB48` | `0.0` (`0x00000000`) | z-component of the copied vector |
| `0x807CAB54` | `1.0` (`0x3F800000`) | default rate |
| `0x807CAB68` | `1/256` (`0x3B800000`) | `qr3` compensation |
| `0x807CAB6C` | `4096.0` (`0x45800000`) | int/scaled conversion divisor |
| `0x807CAB70` | `176.0` (`0x43300000`) | `lfs f1,40(r31)`; the same high word doubles as the `0x4330` integer→double magic for `lfd` |
| `0x807CAB7C` | `1.2` (`0x3F99999A`) | frame-rate scale (also `+0x4`) |
| `0x807CAB84` | `0.75` | per-slot weight (table U) |
| `0x807CAB88` | `0.70` | (same table) |
| `0x807CAB8C` | `0.42` | (same table) |
| `0x807CAB90` | `0.28` | (same table) |
| `0x807CAB94` | `0.20` | (same table) |
| `0x807CAB98` | `0` | slot period 0 |
| `0x807CAB9C` | `28` | slot period 1 |
| `0x807CABA0` | `44` | slot period 2 |
| `0x807CABA4` | `49` | slot period 3 |
| `0x807CABA8` | `55` | slot period 4 |
| `0x807CABD4` | `1.0` | second default rate (different cell) |
| `0x807F5DB8` | `1` | raw copy to `+0x1010` |
| `0x807F5DBC` | `0xFFFF7000` | block V source |
| `0x807F5DC0` | `0x00009000` | block V source |
| `0x807F5DC4` | `0x0000D000` | block V source |
| `0x807F5DC8` | `1` | raw copy to `+0x1020` |
| `0x807F5DCC` | `0xFFFF7000` | block V source |
| `0x807F5DD0` | `0x00009000` | block V source |
| `0x807F5DD4` | `0xFFFF3000` | block V source |
| `0x807F5DD8` | `1` | block V source |
| `0x807F5DDC` | `0x00009000` | block V source |
| `0x807F5DE0` | `0xFFFF7000` | block V source |
| `0x807F5DE4` | `0x0000D000` | block V source |
| `0x807F5D94` | `0` | `abi_call3` third argument (block U) |
| `0x80377DC8` | main.dol BSS | `0x800E78E0` argument |

### Block table (address range, words, effect)

| block | range | effect |
|---|---|---|
| P | `0x806B7CD0..0x806B7D14` | `stwu -192`; LR at `r1+196`; `r11 = r1+160`; `stfd f31`@176 + `psq_st f31`@184,qr0; `stfd f30`@160 + `psq_st f30`@168,qr0; `bl 0x802DCD68` (save helper); `lis r0,0x4330` stored at `r1+40` and `r1+48` (the `2^52` magic); `r30 = 0x807F5D88`, `r31 = 0x807CAB48`, `r29 = this` |
| Q | `0x806B7D14..0x806B7D58` | `bl 0x806B7830` → `r3`; if `r3 == 0` store `*(0x807CAB54) = 1.0` at `this+0x100C` and go to R; else `bl 0x806B7830(this)` again; if that `== 1` store `*(0x807CABD4) = 1.0` at `this+0x100C`, else store `*(0x807CAB54) = 1.0`. **All three paths store `1.0`**; the two cells are distinct but equal |
| R | `0x806B7D58..0x806B7E3C` | five signed remainders `n % 120` with `n = 120 - {0,28,44,49,55}` (idiom: `mulhw` with `0x88888889`, `add`, `srawi 6`, sign fix via `rlwinm ...,1,31,30`, `mulli 120`, `subf`); results **0, 92, 76, 71, 65** stored as `u32` at `this+0xCC0`, `+0xCC4`, `+0xCC8`, `+0xCCC`, `+0xCD0` |
| S | `0x806B7E3C..0x806B7F4C` | `CTR = 15`, then a `bdnz` loop; per iteration 8 slots, each writing `this[0xAC], this[0xB0], this[0xB4]` (three `lfs`/`stfs`) to `r8 + 0x540 + i*12` and `*(this+0x100C)` to `r9 + 0xAE0 + i*4`; then `r8 += 0x60`, `r9 += 0x20`. Net: 120 vec3 copies over `this+0x540..0xAE0` and 120 identical `1.0` floats over `this+0xAE0..0xCC0` |
| T | `0x806B7F4C..0x806B7FEC` | `ret = abi_call1(0x806B7800, this)`; `f30 = lfsx(0x807CAB7C, 4*ret)` (float table indexed by the callee's return value, **not** by `this[0x1048]`); `n = this[0x1048]`; `*(u16*)(r1+0x0A) = (u16)(n >> 3)` (word `0x54006824` = `rlwinm r0,r0,13,0,18`, whose low half is `n >> 3`); `psq_l f1,0(r1+0x0A),qr3,w1` × `1/256`; `f1 = f32_of_f32(0x80237C40, f1)`; `r25 = (i16)n`; `*(u16*)(r1+8) = (u16)r25`; `psq_l f1,0(r1+8),qr3,w1` × `1/256`; `f1 = f32_of_f32(0x80237BD0, f1)`; `this[0xE8] = f30 * cos`, `this[0xEC] = f30 * sin`, `this[0xF0] = 0.0`; `this[0x348] = ((((u32)(n - 0x4000)) & 0xFFFF) <= 0x7FFF) as u8` (i.e. `n` in `[0x4000,0xBFFF]`) |
| U | `0x806B7FF0..0x806B8058` | `for j in 0..this[0x53C]`: `abi_call3(0x8008C3E0, this+0xCD4+0xA4*j, this, 0x807F5D94)`; `t = f32(0x807CAB84 + 4*j) * *(0x807CABCC = 10.0)`; `v = this[0xAE0 + 4*(this[0xCC0+4*j] as usize)] * t`; store `v` at `this+0xCF8+0xA4*j` and `this+0xCFC+0xA4*j`; `abi_call1(0x8008C330, this+0xCD4+0xA4*j)` |
| V | `0x806B805C..0x806B8258` | twelve `u32` fields. Two are copied raw: `0x1010 = *(0x807F5DB8)` and `0x1020 = *(0x807F5DC8)` (both `1`). The other ten (`0x807F5DBC/C0/C4`, `0x807F5DCC/D0/D4`, `0x807F5DDC/E0/E4`) go through the `0x43300000` ladder: `oris r0,r0,0x8000`, `stw` to stack beside the pre-stored magic, `lfd`, `fsubs 4096.0`, `fdiv 4096.0`, `fmuls` by `0.75` (`0x807CAB84`), `fmuls` by `4096.0`, `fctiwz`, `stfd`/`lwz`, then stored at `+0x1014`, `+0x1018`, `+0x101C`, `+0x1024`, `+0x1028`, `+0x102C`, `+0x1034`, `+0x1038`, `+0x103C` |
| W | `0x806B8258..0x806B8290` | `abi_call5(0x8006D090, this+0x1EC, this, this+0x1020, this+0x1010, this+0x1030)`; then a mixed-ABI call `0x80075270(f1 = this[0xAC], f2 = this[0xB0], r3 = this[0x38F] as u32 (byte load), r4 = 0)`; then `this[0x1040] = (r3 == 1) as u8` (word `0x5400D97E`, whose mask `0x07FFFFFE` reduces `cntlzw(r3-1)` to `0/1`); then `abi_call1(0x806B7340, this)` |
| X | `0x806B8290..0x806B8308` | `abi_call4(0x800E78E0, r1+0x18, this+0x3F0, 0x807F5E00, 0x80377DC8)`; then `vtbl[0xA0](this+0x1274)` with `f1 = this[0xE0]`, `f2 = this[0xDC]`, the vec3 `{this[0xDC], this[0xE0], this[0xE4]}` spilled at `r1+0x0C/0x10/0x14`, and `r4 = 0x807F5E38`, `r5 = 0`, `r6 = r1+0x18`, `r7 = 0`, `r8 = r1+0x0C`; then `vtbl[0xA0](this+0x1388)` with `r4 = 0x807F5E4C`, `r5 = 0`, `r6 = r1+0x18`, `r7 = 0`, `r8 = 0` and no float arguments. Neither shape matches `vmethod_f32x3_u32` (`src/game/bases.rs:165737`), so a dedicated host helper is needed for each |
| Y | `0x806B8308..0x806B8348` | `q1 = abi_call2(0x8006FA40, this+0x1EC, 0)`; if `q1 != 0` go to L; `q2 = abi_call2(0x8006EF40, this+0x1EC, 0)`; if `q2 != 0` go to L; `q3 = abi_call1(0x8006DB30, this+0x1EC)` (`r4` is not reloaded before this call, so it is a one-argument call - consistent with `src/game/bases.rs:33584`); if `q3 == 0` skip; **L:** `abi_call1(0x806B7690, this)`. So `fn_806B7690` runs iff `q1 != 0 || q2 != 0 || q3 == 0` |
| Z | `0x806B8348..0x806B8378` | `this[0x1044] = 0`; restore `psq_l/lfd f31,f30`, `r11`, `bl 0x802DCDB4` (restore helper), LR, `addi r1,192`, `blr` |

Note on block Y: the stream is
`bl 0x8006FA40; cmpwi r3,0; bne L; bl 0x8006EF40; cmpwi r3,0; bne L;
bl 0x8006DB30; cmpwi r3,0; beq L+8; L: bl 0x806B7690(this)`. The intuitive
reading `if (q1 == 0 && q2 == 0 && q3 != 0) { ... }` is **wrong**: `L` is
reached when `q1 != 0` **or** `q2 != 0` **or** `q3 == 0`, and only then is
`0x806B7690` called. Transcribe the branches, not the intuition.

### Proposed signature

```rust
/// Initialises the 0x1048-byte pose/animation object.
///
/// # Original
/// `FUN_806b7cd0`, `0x806B7CD0`, 1 704 bytes,
/// `work/ghidra/rel-d_basesNP/export/806b7cd0_FUN_806b7cd0.c`.
/// Synthetic base `0x80600000`; not a runtime address.
///
/// # Arguments
/// * `this` - object to initialise; must be at least `0x1048` bytes.
///
/// # Side effects
/// Fills `+0xE8..+0xF0`, `+0x348`, `+0x53C`-driven table records at
/// `+0xCD4..`, `+0xCF8..`, the 120-slot pose arrays at `+0x540` and `+0xAE0`,
/// the twelve `u32` counters at `+0x1010..+0x103C`, `+0x1040` and `+0x1044`.
/// Registers the object with `0x800E78E0` and runs two virtual `+0xA0` calls.
///
/// # Confidence
/// medium - the two `psq_l ...,qr3,w1` conversions in block T depend on the
/// main.dol GQR3 word; every other word is explicit.
pub unsafe fn fn_806b7cd0(this: usize) { /* blocks P..Z above */ }
```

Constants to emit as `const` items: the five slot periods (`0, 28, 44, 49, 55`),
`120` (their modulus and the loop count), `15 × 8 = 120` (pose slots), `0x53C`
(record count), `0xCD4`/`0xA4` (record stride), `0xCF8`, `0x540`/`0xAE0`, and
the float cells listed above. The `0x807CAB7C + 4*n` float table is walked by
`lfsx` and must be read through `platform::read_f32`.

**Verdict: conditional (medium)** - ready except block T's two `qr3` loads,
which need the GQR3 descriptor (see the GQR3 note in "Method"). With
`GQR3.load = 0x0807` block T reduces to `cos/sin((s16)this[0x1048])`.

---

## 11. `0x806DD730` - 736 bytes - `work/ghidra/rel-d_basesNP/export/806dd730_FUN_806dd730.c`

**Body:** `0x806DD730..0x806DDA10` (184 words, `blr` at `0x806DDA0C`).
Ledger: absent. Spec headings: none. `src/game/bases.rs`: absent.
Export warnings: two "Removing unreachable block" (`0x806dd828`, `0x806dd938`) -
both are the `psq_l` group, live in the raw stream.

### Instruction-level behaviour

**Dispatch (words 0-22).**

```
stwu r1,-32; mflr r0; stw r0,36(r1); stw r31,28(r1); stw r30,24(r1);
stw r29,20(r1)
r31 = 0x80830000 - 9000 = 0x8082DCD8     (BSS descriptor block, 8 x 0x40)
r30 = this
r5  = *(u32*)0x8042A200                  (main.dol global counter)
if ((*(u32*)(this+4) >> 3) & 1) r5 += 174
r29 = r5 % 348                           (magic 0x2F159903, mulhwu >> 6)
```

`0x8042A200` is main.dol data, not in this REL; it must be read through
`platform::read_u32(0x8042_A200)`. The `% 348` idiom was verified numerically
against `n % 348` for `0 <= n < 2000` (zero mismatches).

**Switch on `r29` (signed compare chain, no jump table).** Ranges and the
`vcall` argument (the BSS descriptor base is `0x8082DCD8`):

| case start | `r29` range | `sth this+0x720` | `sth this+0x722` | `stb this+0x724` | vcall arg |
|---|---:|---:|---:|---:|---|
| `0x806DD7D4` | `[0,80)` | `0` | - | `80 - r29` | `0x8082DE28` (`+0x150`) |
| `0x806DD80C` | `[80,112)` | `0` | `((r29-80) & 0x1F) << 11` | `112 - r29` | `0x8082DE68` (`+0x190`) |
| `0x806DD870` | `[112,128)` | `0` | - | `112 - r29` | `0x8082DEA8` (`+0x1D0`) |
| `0x806DD8A8` | `[128,174)` | `(r29-128) * 712` | - | `174 - r29` | `0x8082DCE8` (`+0x010`) |
| `0x806DD8E4` | `[174,254)` | `0x8000` | - | `254 - r29` | `0x8082DD28` (`+0x050`) |
| `0x806DD91C` | `[254,286)` | `0x8000` | `((r29-254) & 0x1F) << 11` | `286 - r29` | `0x8082DD68` (`+0x090`) |
| `0x806DD980` | `[286,302)` | `0x8000` | - | `302 - r29` | `0x8082DDA8` (`+0x0D0`) |
| `0x806DD9B8` | `[302,348)` | `(r29-302) * 712 - 0x8000` | - | `348 - r29` | `0x8082DDE8` (`+0x110`) |

(The `r29 == 348` and negative cases cannot occur: `r29` is an unsigned `% 348`
and the `blt` chain sends everything `>= 302` to the last case. The `b
0x806DD9B8` after the `cmpi r29,0` is unreachable, as `r29 >= 0` always.)

**Two cases carry a `qr3` conversion instead of a literal `sth`.** In
`0x806DD80C` and `0x806DD91C` the second halfword is
`(u16)(((r29 - base) & 0x1F) << 11)` (word `0x54005C28` is
`rlwinm r0,r0,11,16,20`, mask `0x0000F800`), stored both at `this+0x722` and at
`r1+0x0A`/`r1+0x08`; then

```
psq_l f1,0(r1+8|10),qr3,w1
f0 = *(f32*)0x807CB9E0          ; 1/256
f1 = f0 * f1
f1 = f32_of_f32(0x80237BD0, f1) ; sine
```

The sine result is **discarded** - no later word in the case reads `f1`; only
the two halfword stores at `this+0x720`/`this+0x722` survive. Transcribe the
store and the call, not the trig value.

**Common tail of every case (nine words).**

```
r3 = this + 0x394                     (cases 0/112/128/174/286/302 use
                                       `lwzu r12,0x394(r3)`, which also advances
                                       r3; cases 80/254 use `addi r3,r30,0x394`
                                       + `lwz r12,0x394(r30)`)
r12 = *(u32*)(r3)                     ; vtable of the sub-object
r12 = *(u32*)(r12 + 0x18)             ; slot 6
CTR = r12 ; bctrl                     ; vcall(this+0x394, <BSS arg>)
r4 = (u32)*(u16*)(this+0x720)         ; lhz
r3 = this
f1 = abi_call2(0x806DDA10, this, r4)
*(f32*)(this+0x710) = f1
```

So each case is: write the two halfwords and the byte, call the sub-object's
`+0x18` virtual with the case's BSS descriptor, then store
`fn_806dda10(this, (u16)this[0x720])` at `this+0x710`.

`0x806DDA10` is itself a wave-127 class-C body (88 bytes, still untranslated) -
the call must go through `abi_call2!` until it is translated, then through
`fn_806dda10`.

### Proposed signature

```rust
/// Re-selects the pose/offset for the current 348-step cycle.
///
/// # Original
/// `FUN_806dd730`, `0x806DD730`, 736 bytes,
/// `work/ghidra/rel-d_basesNP/export/806dd730_FUN_806dd730.c`.
/// Synthetic base `0x80600000`; not a runtime address.
///
/// # Arguments
/// * `this` - object; a sub-object with a live vtable lives at `+0x394`.
///
/// # Side effects
/// Writes `this[0x720]` (s16), `this[0x722]` (s16), `this[0x724]` (u8) and
/// `this[0x710]` (f32), and calls the `+0x18` virtual of `this+0x394`.
///
/// # Confidence
/// medium - the two `qr3` cases need the main.dol GQR3 word; the dispatch,
/// stores, BSS descriptors and call targets are all explicit.
pub unsafe fn fn_806dd730(this: usize) { /* dispatch + eight cases + tail */ }
```

Constants: `174`, `348`, `0x2F159903` (the `%348` multiplier, signed), `712`
(`0x2C8`), `0x8000`, `80/112/128/174/254/286/302/348` (the eight boundaries),
`0x8082DCD8` (BSS base) and `0x8082DCE8 + 0x40*k` for the eight descriptors.
Caller inside `d_basesNP`: `0x806DD070`.

**Verdict: conditional (medium)** - six of the eight cases are ready (0, 112, 128, 174, 286, 302); the two
`qr3` cases need the GQR3 descriptor.

---

## 12. `0x80680720` - 660 bytes - `work/ghidra/rel-d_basesNP/export/80680720_FUN_80680720.c`

**Body:** `0x80680720..0x806809B4` (165 words, `blr` at `0x806809B0`).
Ledger: absent. Spec headings: none. `src/game/bases.rs`: absent.
Export warnings: six "Removing unreachable block" notes (`0x80680998`,
`0x80680990`, `0x806808f0`, `0x806808c0`, `0x8068080c`, `0x806807dc`), each the
pair after a `psq_l` group; all are live in the raw stream.

### Instruction-level behaviour

```
stwu r1,-144; mflr r0; stw r0,148(r1); r11 = r1+112
stfd f31,128 ; psq_st f31,136,qr0,w0
stfd f30,112 ; psq_st f30,120,qr0,w0
bl 0x802DCD70                    ; _savegpr
r31 = 0x807C9620                 ; constant pool
r28 = this (r3), r29 = param2 (r4), r30 = param3 (r5)
```

**Block A - local record (words 11-45).** `f6 = this[0xAC]`, `f4 = this[0xB4]`,
`f5 = this[0xB0]`; `h7 = (i16)param2[0x80]`, `h6 = (i16)param2[0x82]`,
`h4 = (i16)param2[0x84] + (i16)this[0x104]` (`lhz` + `extsh` + `add`). Then:

| stack cell | value |
|---|---|
| `r1+40` | `h7` (s16) |
| `r1+42` | `h6` (s16) |
| `r1+44` | `h4` (s16) |
| `r1+48` | `f6` |
| `r1+52` | `f5 - 400.0` |
| `r1+56` | `f4` |
| `r1+60`, `r1+64`, `r1+68` | `1.0` |
| `r1+72` | `f6` |
| `r1+76` | `f5` |
| `r1+80` | `f4 - 400.0` |
| `r1+14` | `(s16)this[0x104] - param3` |
| `r1+12` | `(s16)this[0x104] - param3` (second copy) |
| `r1+8`, `r1+10` | as above (per-group copies) |

**Block B - first angle group (words 49-65).**

```
psq_l f1,0(r1+14),qr3,w1 ; f0 = 1/256 (0x807C96AC) ; f1 = f0*f1
f1 = f32_of_f32(0x80237C40, f1)      ; cosine
f1 = 336.0 * f1                      ; *(0x807C96A4) = 336.0
psq_l f1,0(r1+12),qr3,w1 ; f0 = 1/256 ; f1 = f0*f1
f1 = f32_of_f32(0x80237BD0, f1)      ; sine
f2 = 336.0 * f1
f0 = *(r1+52) ; f2 = f0 + f0 ; *(r1+76) = f2
```

**Block C - rotate (word 72).** `0x801C0BD0` takes three floats in `f1..f3`:
`f1 = *(r1+72)`, `f2 = 2 * *(r1+52)` (re-stored to `r1+76`), `f3 = *(r1+80)`.
This needs a float-ABI helper, not `abi_call3!`.

**Block D - mount offsets (words 73-87).** Three halfwords are copied and
installed on the `this+0x7C` object:
`abi_call2(0x8016EDD0, this+0x7C, r1+36)`,
`abi_call2(0x8016ECB0, this+0x7C, r1+32)`,
`abi_call2(0x8016EEF0, this+0x7C, r1+28)`, with
`*(u16*)(r1+36) = h6`, `*(u16*)(r1+32) = h7`, `*(u16*)(r1+28) = h4`'s copy.

**Block E - transform copies (words 88-108).**

```
abi_call2(0x8016A170, param2,     this+0x7C)
abi_call2(0x8016A150, param2,     r1+0x3C)
abi_call2(0x8016A1A0, param2,     0)
vtable = *(u32*)param2 ; fn = vtable[0x14]
abi_call1(fn, param2)                ; bctrl with r3 = param2
```

**Block F - second angle group (words 109-129).** Same shape as B with `f31`
(`= 336.0 - 8.0 = 328.0`, `*(0x807C96A8) = 8.0`) as the scale, and
`0x80237C40`/`0x80237BD0` again; then `abi_call3(0x801C0BD0, f1, f2, f3)`.

**Block G - second mount set (words 130-156).** The same six calls as blocks D
and E (`0x8016EDD0`, `0x8016ECB0`, `0x8016EEF0`, then `0x8016A170`,
`0x8016A150`, `0x8016A1A0`), but with the object at **`param2+0x40`**
(`addi r3,r29,0x40`) and the virtual taken from `*(u32*)(param2+0x40)`, slot
`+0x14`.

**Block H - epilogue (words 157-165).** `r11 = r1+112`; restore
`psq_l/lfd f31` and `f30`; `bl 0x802DCDBC` (restore helper); LR; `addi r1,144`;
`blr`.

### Constants

| cell | value |
|---|---|
| `0x807C96A0` | `40.0` |
| `0x807C9648` | `1.0` |
| `0x807C9660` | `400.0` |
| `0x807C96A4` | `336.0` |
| `0x807C96A8` | `8.0` |
| `0x807C96AC` | `1/256` |

`this+0xAC`, `+0xB0`, `+0xB4` are the three floats read at entry; `this+0x104`
is a signed halfword (angle); `param2+0x80/0x82/0x84` are three signed
halfwords.

### Proposed signature

```rust
/// Rebuilds the two-segment transform/emit state from an angle triple.
///
/// # Original
/// `FUN_80680720`, `0x80680720`, 660 bytes,
/// `work/ghidra/rel-d_basesNP/export/80680720_FUN_80680720.c`.
/// Synthetic base `0x80600000`; not a runtime address.
///
/// # Arguments
/// * `this` - emitter object (`+0x7C` is the transform, `+0xAC/+0xB0/+0xB4`
///   are the source position, `+0x104` is a signed angle halfword).
/// * `actor` - object whose `+0x80/+0x82/+0x84` halfwords and whose vtable
///   `+0x14` slot are used; `actor+0x40` is a second actor with the same
///   interface.
/// * `angle` - signed offset subtracted from `this[0x104]`.
///
/// # Confidence
/// medium - the four `psq_l ...,qr3,w1` conversions need the main.dol GQR3
/// word; the two `+0x14` virtuals and all copies are explicit.
pub unsafe fn fn_80680720(this: usize, actor: usize, angle: i32) { /* A..H */ }
```

Caller inside `d_basesNP`: `0x8068046C`.

**Verdict: conditional (medium)** - every block except the four `qr3` loads is
ready; with `GQR3.load = 0x0807` the angle terms become
`(s16)this[0x104] - angle` as `f32`.

---

## Verification performed

* Candidate pool recomputed from `docs/ANALYSIS_WAVE127_BASES.md`: 121 class-B
  rows (51 060 bytes), 112 addresses named in `##` titles across
  `docs/SPEC_WAVE128_BASES.md` .. `docs/SPEC_WAVE136_BASES.md`, **9** left
  (528 bytes) - all nine taken here.
* `ledger/functions.csv` checked **row-based**: `^<module>,<addr>,` for each of
  the 12 addresses in every module. Zero rows. (`0x806FCD70`/`0x806FCDB0` do
  appear as *table words* at `0x808034C0`/`0x808034D8`, not as addresses.)
  Re-checked after the parallel lane merged (22:22): the nine class-B rows are
  still absent from `ledger/functions.csv` and now present in
  `ledger/agents/wave137-bases.csv`; the three class-C rows are in neither.
* `src/game/bases.rs` contains no `fn_<addr>` definition and no textual mention
  of any of the twelve at selection time. After the parallel merge: the nine
  class-B identifiers now exist and their bodies were compared against sections
  1-9 of this document - the guards, the `0x802B9280` call, the `0x60`/`0xD4`
  virtual, the `0x807F1360` halfword load and the `this+0x3C0` release all match;
  the three class-C identifiers are still absent.
* No `##` heading in `docs/SPEC_WAVE128_BASES.md` .. `docs/SPEC_WAVE136_BASES.md`
  names any of the twelve.
* Each body was decoded from `work/rels/elf/d_basesNP.elf` with a throwaway
  PowerPC decoder kept at `%TEMP%\opencode\ppc.js` (never in the repo). The
  decoder resolves `psq_l`/`psq_st` (`opcode 56`/`60`, index bits 12-14, `W`
  bit 15, `DQ` bits 0-11), `rlwinm`/`rlwnm`/`rlwimi` with MB/ME wrap,
  `mulhw`/`mulhwu`, `extsh`/`extsb`, `mfspr`/`mtspr`, `b`/`bl`/`bc`/`bcctr`,
  and the integer/float load-store set.
* Extent: the reachable span equals the TSV size for all twelve; last word
  `0x4E800020` for eleven, `0x4E800420` (`bcctr`) for `0x8069ACA0`. No
  `halt_baddata`, no jump table, no row starting inside a body.
* SHA-1 of each range compared against every same-size `d_basesNP` TSV row: no
  matches. Word-level comparison against `fn_80651350` recorded above.
* `d_basesNP.elf` program headers re-read this pass (table in "Method"); the
  `+0x1000` rule of `docs/REL_ELF_MAPPING.md` is correct for this REL but was
  not assumed.
* Constants were read with `p_offset + (addr - p_vaddr)` and are quoted with
  their image addresses; three `0x8082xxxx`/`0x8083xxxx` cells resolve into this
  REL's zero-fill segment and are cited as addresses only.
* `0x8042A200` (used by #11) is main.dol data, not present in this REL; it is
  cited as an address and read through `platform::read_u32`.

## Hand-off

* `0x807BD110` (2 908 bytes) - `dFileSelect_c` 44-entry state table. Needs the
  registration-table generator, not a control-flow body; the export opens with
  three bogus `ldexpf(unaff_GQR0 & 0x3f)` guards.
* `0x8079C330` (4 232 bytes) and `0x8079D3C0` (3 176 bytes) - the large
  constructor/destructor pair; `docs/COVERAGE_WAVE91_BASES.md:149` and
  `docs/COVERAGE_WAVE135_BASES.md:145` both record that each needs its own call
  graph and the `0x802DCD70` `undefined8` return model.
* `0x806DDA10` (88 bytes, class C) is called by seven of the eight cases of
  #11. Translating it turns those `abi_call2!` sites into direct calls.
* The remaining 22 class-C and 173 class-D addresses of the wave-127 report are
  untouched. The next class-C batch opens with `0x807BD110` (2 908, the table),
  `0x8072E520` (644) and `0x8072BC40` (604).
