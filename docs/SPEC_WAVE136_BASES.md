# Wave 136 - `game::bases` - implementation spec for the next 12 class-B addresses

Read-only lane. The only repository file written is this one. The throwaway
PowerPC decoder (`%TEMP%\opencode\ppc136.js`, with `dump136.js`,
`dol136dis.js`, `verify136.js`, `cmp136.js`, `callers136.js`, `audit136.js`,
`pool136.js` beside it) lives outside the repository; no Rust file, no
`ledger/functions.csv` row and no existing document was modified. No merge or
check tool was run.

## Why these twelve

`docs/ANALYSIS_WAVE127_BASES.md` reclassified 121 `d_basesNP` skips as class B
("reconstructible now from the raw stream"). Waves 128-135 carried 100 of them
into `docs/SPEC_WAVE128_BASES.md` .. `docs/SPEC_WAVE135_BASES.md`; 88 of the
121 have since been claimed by `ledger/functions.csv`, and all 88 are also
named by a spec heading. That leaves **21** class-B addresses, **1 608** bytes,
with neither a ledger row nor a section of their own.

This lane takes the twelve largest, ordered by `functions.tsv` size descending
and then by address ascending. This is exactly the order wave 135's hand-off
(`docs/SPEC_WAVE135_BASES.md` §"Hand-off") pre-declared:

| # | address | bytes | words | export file | verdict |
|--:|---:|---:|---:|---|---|
| 1 | `0x806B3AC0` | 136 | 34 | `806b3ac0_FUN_806b3ac0.c` | conditional (medium) |
| 2 | `0x80608C40` | 132 | 33 | `80608c40_FUN_80608c40.c` | ready (high) |
| 3 | `0x8066B0F0` | 128 | 32 | `8066b0f0_FUN_8066b0f0.c` | conditional (medium) |
| 4 | `0x806FCCA0` | 100 | 25 | `806fcca0_FUN_806fcca0.c` | ready (high) |
| 5 | `0x80681480` | 88 | 22 | `80681480_FUN_80681480.c` | ready (high) |
| 6 | `0x80681FA0` | 88 | 22 | `80681fa0_FUN_80681fa0.c` | ready (high) |
| 7 | `0x806FF3F0` | 88 | 22 | `806ff3f0_FUN_806ff3f0.c` | ready (high) |
| 8 | `0x80600990` | 64 | 16 | `80600990_FUN_80600990.c` | ready (high) |
| 9 | `0x806009D0` | 64 | 16 | `806009d0_FUN_806009d0.c` | ready (high) |
| 10 | `0x80600A10` | 64 | 16 | `80600a10_FUN_80600a10.c` | ready (high) |
| 11 | `0x80600A50` | 64 | 16 | `80600a50_FUN_80600a50.c` | ready (high) |
| 12 | `0x80600A90` | 64 | 16 | `80600a90_FUN_80600a90.c` | ready (high) |

Together **1 080 bytes** (270 words). Every export path is exactly
`work/ghidra/rel-d_basesNP/export/<file>` relative to the repository root, and
every `bytes` value matches the `size` column of that directory's
`functions.tsv` verbatim.

### Selection rule

The candidate set is: **class B in wave 127**, **no row anywhere in
`ledger/functions.csv` whose address field equals the address**, and **no `##`
title in any `docs/SPEC_WAVE*_BASES.md` that names the address**. The heading
test is title-based, on all eight surviving spec files (128-135), not a
substring search over the prose: the two methods agree for all twelve here.

| address | ledger rows | wave-127 class | prior skip reason |
|---|---:|---|---|
| `0x806B3AC0` | 0 | B | wave 118 - "`undefined8` fragment" |
| `0x80608C40` | 0 | B | wave 11 - collection-coin handler |
| `0x8066B0F0` | 0 | B | wave 126 - receiver-less dispatch bodies |
| `0x806FCCA0` | 0 | B | wave 113 - `0x806FCC60..0x806FCDF0` neighbours not in this window |
| `0x80681480` | 0 | B | wave 123 - duplicate release twins |
| `0x80681FA0` | 0 | B | wave 123 - duplicate release twins |
| `0x806FF3F0` | 0 | B | wave 11 - wire-mesh lifecycle |
| `0x80600990` | 0 | B | wave 116 - delete wrappers |
| `0x806009D0` | 0 | B | wave 116 - delete wrappers |
| `0x80600A10` | 0 | B | wave 116 - delete wrappers |
| `0x80600A50` | 0 | B | wave 116 - delete wrappers |
| `0x80600A90` | 0 | B | wave 116 - delete wrappers |

"Ledger rows" is an exact address-field match in any module, not just
`game::bases`. All twelve also have zero `##`-title matches in the eight
existing `SPEC_WAVE*_BASES.md` files.

**The gate is the ledger plus the spec titles, not the Rust file.** A parallel
wave-136 translation lane merged eleven of these twelve bodies
(`0x806FCCA0` was still unmerged) while this document was being written; that
does not change the list, and the cross-check against those merges is recorded
in "Verification performed" below. This is the same concurrency caveat wave 135
recorded.

### Ties, and what is left

The cut is clean: `0x80600A90` (64 B) is the 12th and `0x80600AD0` (64 B, the
next byte-identical wrapper) is the 13th, broken by ascending address. The next
nine after the cut, in the same order, are:

| address | bytes | note |
|---|---:|---|
| `0x80600AD0` | 64 | sixth copy of the §8 shape; table word at `0x807D0FC0` |
| `0x80601D10` | 64 | wave-116 delete wrapper |
| `0x80601EB0` | 64 | wave-116 delete wrapper |
| `0x80605500` | 64 | wave-116 delete wrapper |
| `0x806FCC60` | 64 | wave-113 neighbour of §4 |
| `0x806FCD70` | 64 | wave-113 neighbour of §4 |
| `0x806FCDB0` | 64 | wave-113 neighbour of §4 |
| `0x8069ACA0` | 40 | wave-119 virtual tail call (`+0x60`/`+0xD4`) |
| `0x806FF700` | 40 | wave-11 wire-mesh lifecycle |

## Method

**Address mapping (re-verified, not copied).** `d_basesNP.elf` has six
`PT_LOAD` headers. The first is `p_offset = 0x10F0`, `p_vaddr = 0x806000F0`,
`filesz = 0x1C6004`, so

```
file_offset = 0x10F0 + (address - 0x806000F0) = (address - 0x80600000) + 0x1000
```

The other five are `0x807C60F4` (1 276 B), `0x807C65F0` (12 B), `0x807C6600`
(42 492 B), `0x807D0C00` (335 032 B) and the zero-fill segment at `0x80822900`
(`filesz = 0`, `memsz = 74 884`, ending `0x80834D84`). The shipped image
therefore ends at `0x808228B8`; everything at or above `0x80822900` is BSS.
Every constant below was read at `p_offset + (addr - p_vaddr)`.

**Extent check.** For all twelve the reachable span equals the TSV size exactly:
the last word of every range is `0x4E800020` (`blr`) at `entry + size - 4`, and
no `functions.tsv` row starts inside a body. No over-read, no `halt_baddata`, no
jump table. Each body contains exactly one `bclr`, the terminal `blr`.

**Duplicate check.** The SHA-1 of each of the twelve ranges was compared against
every `d_basesNP` TSV row of the same size: **no match**. Nine of the twelve are
nevertheless *shape* twins of each other and of bodies that are already
translated, which is the interesting part of this batch:

| this lane | merged twin | differing words |
|---|---|---|
| `0x806FCCA0` (§4) | `fn_806357e0`, `src/game/bases.rs:6728`, 96 B | one extra `beq` at index 8; two `b` displacements |
| `0x80681480` (§5) | `fn_806511d0`, `src/game/bases.rs:4242`, 88 B | index 10 (`bl 0x8015F800`), index 14 (`bl 0x802B9280`) |
| `0x80681FA0` (§6) | `fn_806511d0` | same two indices |
| `0x806FF3F0` (§7) | `fn_80650600`, `src/game/bases.rs:3385`, 120 B | index 10 (`bl 0x800642B0`), index 14 (`bl 0x80162920`) - shared shape only |
| `0x80600990` (§8) | `fn_80651350`, `src/game/bases.rs:4335`, 64 B | index 9 (the single `bl`) |
| `0x806009D0`/`A10`/`A50`/`A90` (§9-§12) | `fn_80651350` and each other | index 9 only |

Concretely, `0x80681480` and `0x80681FA0` differ from each other in exactly two
words (`0x4BADE359` vs `0x4BADD839`, `0x4BC37DC9` vs `0x4BC372A9`); the five
64-byte wrappers differ only in the `bl` displacement at index 9
(`0x4BCB88CD`, `0x4BCB888D`, `0x4BCB884D`, `0x4BCB880D`, `0x4BCB87CD`, all
targeting `0x802B9280`). No body here is a *relocated copy* of a whole existing
function, so each still needs its own `pub unsafe fn`.

**Paired-single audit.** Across the 270 words there is **no `psq*` word at all**
and no `mtspr` to a `GQRn`. The only SPR writes are `LR` (`mtspr r0,LR` in
every epilogue, spr 8) and, in §2 and §3 only, `CTR` (`mtspr r12,CTR`, spr 9)
for the `bcctr` dispatches. Every "unaff_GQR0" rejection that produced these
skip entries was a decompiler artefact.

**Floating point.** Only §1 touches FP: `stfd f31/f30` (8-byte spill of a 32-bit
value, `.0` half) plus `fmr` moves; no arithmetic, no `0x43300000` ladder, no
pool cell. §2-§12 are integer-only.

**Cross-artifact cells.** Two cells read below are not in this REL:

| cell | artifact | value |
|---|---|---|
| `0x80377C48` | `main.elf` BSS (`PT_LOAD` `0x80351680`, `filesz = 0`, `memsz = 0xD6000`) | the live heap/arena handle, mirrored by `0x8016E9F0` |
| `0x80829644` | *this* REL's zero-fill segment (`0x80822900`, `filesz = 0`) | a writable, zero-initialised byte buffer |

Neither has shipped bytes in the image, so each must be cited as an address and
read through `platform::read_u32(...)`, never as a literal. Everything else was
read from the shipped `d_basesNP` image and is listed with its address.

**House style used below.** `pub unsafe fn fn_XXXXXXXX(this: usize)`,
`platform::read_u32/read_i32/read_f32/read_u8`,
`platform::write_u32/write_u8`, `abi_call1!..abi_call5!`, `vmethod0(rcv, slot)`,
`ptr_at(addr)`, addresses written `0x8060_8DF0`, field offsets as `usize`
literals, `// unknown:` on the first use of every address-only target or
external cell.

---

## 1. `0x806B3AC0` - 136 bytes - `work/ghidra/rel-d_basesNP/export/806b3ac0_FUN_806b3ac0.c`

**Body:** `0x806B3AC0..0x806B3B48` (34 words, `blr` at `0x806B3B44`).
Ledger: absent (zero rows). Selection gate: no ledger row, no spec title.
Export warning: one "globals starting with `_` overlap smaller symbols" note
about `_DAT_80377c48`; benign, it is the same cell as `DAT_80377c48`.

```rust
pub unsafe fn fn_806b3ac0(this: usize, kind: i32, f1: f32, f2: f32)
```

**Argument map.** `r3 -> this` (`mr r30,r3` @`0x806B3AF8`), `r4 -> kind`
(`mr r31,r4` @`0x806B3AEC`, forwarded as the third argument of the second call),
`f1 -> f1` (`fmr f30,f1` @`0x806B3AE4`), `f2 -> f2` (`fmr f31,f2`
@`0x806B3ADC`). The only in-REL caller (`0x806B3F14`, in an unlisted body)
builds the second argument with `lhz r4,4952(r29)`, so `kind` is a
zero-extended `u16` field of the caller; the callee only forwards it.

**Frame.** `stwu r1,-32(r1)`; `mfspr r0,LR` / `stw r0,36(r1)`;
`stfd f31,24(r1)` + `stfd f30,16(r1)`; `stw r31,12(r1)` / `stw r30,8(r1)`.
Epilogue `0x806B3B28..0x806B3B48`.

**Shape.** A three-call "rebuild the model pair" body: construct the resource
accessor at `+0x3D4`, build the lift/model group at `+0x524` from that accessor
and the caller's kind + two floats, then release the accessor. Nothing is
returned; all three results are dropped.

1. `0x806B3AC8-0x806B3B04` - accessor construction:
   ```
   r5 = *(u32*)0x80377C48;        // lis r5,0x8037 @0x806B3AC8
                                  // lwz r5,31816(r5) @0x806B3B00
   r6 = 0;                        // li r6,0   @0x806B3ACC
   r7 = 0x20;                     // li r7,0x20 @0x806B3AD4
   r4 = -1;                       // li r4,-1  @0x806B3AF0
   r3 = this + 0x3D4;             // addi r3,r3,980 @0x806B3AFC
   bl 0x800690C0;                 // abi_call5
   ```
2. `0x806B3B08-0x806B3B1C` - the model group:
   ```
   f1 = f30;  f2 = f31;           // fmr f1,f30 / fmr f2,f31
   r5 = kind;                     // or r5,r31,r31
   r3 = this + 0x524;             // addi r3,r30,1316
   r4 = this + 0x3D4;             // addi r4,r30,980
   bl 0x800BC950;
   ```
   This call carries **three integer and two float arguments**
   (`r3, r4, r5`, `f1, f2`). The file already has the matching mixed-ABI
   wrapper: `call_f32x2_u32x3(addr, a, b, c, d, e)` at
   `src/game/bases.rs:35822`, declared
   `extern "C" fn(f32, f32, u32, u32, u32) -> u32`. Use it (two floats first,
   then the three integers), not `abi_call5!`, which is integer-only.
3. `0x806B3B20-0x806B3B24` - accessor release:
   ```
   r3 = this + 0x3D4;             // addi r3,r30,980
   bl 0x800690E0;
   ```
4. `0x806B3B28-0x806B3B48` - restore `r0` into `LR`, pop 32 bytes, `blr`.

**Callees:**

| site | callee | arguments |
|---|---|---|
| `0x806B3B04` | `0x800690C0` (thunk to `0x8016A7F0`) | `r3 = this+0x3D4`, `r4 = -1`, `r5 = *(u32*)0x80377C48`, `r6 = 0`, `r7 = 0x20` |
| `0x806B3B1C` | `0x800BC950` (232 B, no `main-gb` row - see below) | `r3 = this+0x524`, `r4 = this+0x3D4`, `r5 = kind`, `f1`, `f2` |
| `0x806B3B24` | `0x800690E0` (thunk to `0x8016A8E0`) | `r3 = this+0x3D4` |

`0x800690C0` with exactly these five arguments is **already merged verbatim**
at `src/game/bases.rs:3566` (`fn_80650e60`), with the comment
`// unknown: 0x800690C0 constructs a resource accessor; 0x80377C48 is a main.dol
allocator/arena handle, 0x20 the alignment.` Copy that call, not a new guess.

**The second callee is a real entry without a row.** In
`work/ghidra/main-gb/export/functions.tsv` the last row before `0x800BC950` is
`800bc6b0` size 664, ending `0x800BC948`; `0x800BC948`/`0x800BC94C` are zero
padding and `0x800BC950` opens a normal frame (`stwu r1,-64`) that ends in a
`blr` at `0x800BCA34`. This lane decoded it from `work/dol/main.elf`:

1. `0x800BC9A8` `0x800DF180(*(r13-0x5648)+4, 0x80315C58, 0x80315C74)` opens the
   BRRES `"g3d/lift_yogan_kinoko.brres"` (`0x80315C58`) group
   `"lift_yogan_kinoko"` (`0x80315C74`) and stores the handle at `*dst`.
2. `0x800BC9B8` `0x80239F20(dst, 0x80315C88)` (`"spin_kinoko_B"`) fetches the
   animation.
3. `0x800BC9D4` `0x8000D7E0(dst+4, &anim, 128, -1, src)` copies the animation
   record; `0x800BC9F4` `0x800BC5D0(dst+0x0D30, src, dst, 0x802F0D70)` and
   `0x800BCA04` `0x800BCA40(dst, r5, f1)` build the nodes.

So the call is a *model-group initialiser* `(dst, source_accessor, kind, f1, f2)`
that also consumes the two floats. It writes only through `dst` (`this+0x524`)
and reads only the accessor at `this+0x3D4`; no other field of `this` is
touched.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| heap/arena handle | `0x80377C48` (main.dol BSS, no shipped bytes) | `lis r5,0x8037` @`0x806B3AC8` + `lwz r5,31816(r5)` @`0x806B3B00` |
| accessor offset `+0x3D4` | immediate | `addi r3,r3,980` @`0x806B3AFC`, `addi r4,r30,980` @`0x806B3B18`, `addi r3,r30,980` @`0x806B3B20` |
| group offset `+0x524` | immediate | `addi r3,r30,1316` @`0x806B3B14` |
| accessor size `0x150` | derived (`0x524 - 0x3D4`) | not stored |
| `-1`, `0`, `0x20` | immediates | `li r4,-1`, `li r6,0`, `li r7,0x20` |

**Confidence: medium.** All 34 words decode and both FP arguments are literal
restores of the entry values, but the largest callee (`0x800BC950`) has no
Ghidra row, so the lane must cite it as an external call and keep an
`// unknown: 0x800BC950 initialises the +0x524 model group` comment. Nothing
about the *body* is uncertain.
**Verdict: conditional** (the one inference is that `0x800BC950`'s ABI is the
five-register form decoded above; the argument registers are all explicit).

---

## 2. `0x80608C40` - 132 bytes - `work/ghidra/rel-d_basesNP/export/80608c40_FUN_80608c40.c`

**Body:** `0x80608C40..0x80608CC4` (33 words, `blr` at `0x80608CC0`).
Ledger: absent. Selection gate: no ledger row, no spec title. Export
warnings: none.

```rust
pub unsafe fn fn_80608c40(this: usize) -> bool
```

**Argument map.** `r3 -> this` (`mr r31,r3` @`0x80608C50`). `r4`-`r6` are
clobbered before any read; the body takes no other argument. Return value is
`0`/`1` (`li r3,0` @`0x80608C60`, `li r3,1` @`0x80608CAC`), i.e. `bool`.

**Frame.** `stwu r1,-16(r1)`; `mfspr r0,LR` / `stw r0,20(r1)`;
`stw r31,12(r1)`. Epilogue `0x80608CB0..0x80608CC4`. No `psq`.

**Shape.** The collection-coin layout binder: bail out with `false` when the
"load requested" word at `+0x124` is clear, otherwise bind the
`worldCollectionCoinBase_01.brlyt` layout on the interface object at `+0xA0`,
stamp the resulting animation id into `+0x238`, resolve thirteen pane names into
`+0x23C`, and answer `true`.

1. `0x80608C54-0x80608C64` - the guard:
   ```
   if *(u32*)(this+0x124) == 0 { return false; }   // lwz r0,292(r3)
                                                   // cmpi cr0,r0,0 ; bne
                                                   // li r3,0 ; b 0x80608CB0
   ```
2. `0x80608C68-0x80608C84` - layout bind (virtual):
   ```
   vtable = *(u32*)(this + 0xA8);        // lwz r12,168(r3)
   r4 = 0x807D28BC;                      // lis 0x807D / addi r4,r4,10428
   r5 = 0;                               // li r5,0
   fn = *(u32*)(vtable + 0x14);          // lwz r12,20(r12)
   mtspr CTR,fn;
   r3 = this + 0xA0;                     // addi r3,r3,160
   bcctr 20,0,l;
   ```
3. `0x80608C88-0x80608C90` - animation id:
   ```
   r3 = this + 0xA0;                     // addi r3,r31,160
   r3 = 0x80007220(r3);                  // returns *(u32*)(r3+0x20)
   *(u32*)(this+0x238) = r3;             // stw r3,568(r31)
   ```
4. `0x80608C94-0x80608CA8` - pane-name resolution:
   ```
   r3 = this + 0xA0;                     // addi r3,r31,160
   r5 = this + 0x23C;                    // addi r5,r31,572
   r4 = 0x807D2888;                      // lis 0x807D / addi r4,r4,10376
   r6 = 13;                              // li r6,0xD
   bl 0x800C8D60;                        // (layout, names, out, count)
   ```
5. `0x80608CAC` - `li r3,1`, fall through to the epilogue.

**Callees:**

| site | callee | arguments |
|---|---|---|
| `0x80608C84` | vtable slot `0x14` of `*(u32*)(this+0xA8)` (`bcctr 20,0,l`) | `r3 = this+0xA0`, `r4 = 0x807D28BC`, `r5 = 0` |
| `0x80608C8C` | `0x80007220` (main.dol: `return *(u32*)(p+0x20)`) | `r3 = this+0xA0` |
| `0x80608CA8` | `0x800C8D60` (already specified as `FN_800C8D60`, `src/engine/display.rs:2735`) | `r3 = this+0xA0`, `r4 = 0x807D2888`, `r5 = this+0x23C`, `r6 = 13` |

**Data context.** The 13-pointer name table at `0x807D2888` is image-resident
and ends exactly where the layout name begins:

| table word | target | string |
|---|---|---|
| `0x807D2888` | `0x807D27B0` | `"N_coinDateAll_00"` |
| `0x807D288C` | `0x807D27C8` | `"N_coinBarPos_00"` |
| `0x807D2890` .. `0x807D28B4` | `0x807D27D8` .. `0x807D2868` | `"N_coinBarPos_01"` .. `"N_coinBarPos_10"` |
| `0x807D28B8` | `0x807D2878` | `"N_coinBarPos_11"` |
| `0x807D28BC` | - | `"worldCollectionCoinBase_01.brlyt"` |

The only in-REL caller is `bl 0x80608C40` at `0x80608BE8`, inside the unlisted
body that starts at `0x80608BC0`: it tests the `+0x270` byte first, returns `0`
when this function answers `false`, and otherwise sets bit 0 of the render flag
byte at `+0xBB` of the record just stamped at `+0x238`.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| load-request word `+0x124` | immediate | `lwz r0,292(r3)` @`0x80608C54` |
| vtable pointer `+0xA8`, slot `0x14` | immediate | `lwz r12,168(r3)` @`0x80608C68`, `lwz r12,20(r12)` @`0x80608C78` |
| interface receiver `+0xA0` | immediate | `addi r3,r3,160` @`0x80608C80`, `addi r3,r31,160` @`0x80608C88`/`0x80608C98` |
| layout name | `0x807D28BC` (image) | `lis r4,0x807D` @`0x80608C6C` + `addi r4,r4,10428` @`0x80608C70` |
| pane-name table, count `13` | `0x807D2888` (image) | `addi r4,r4,10376` @`0x80608CA0`, `li r6,0xD` @`0x80608CA4` |
| animation-id cell `+0x238` | immediate | `stw r3,568(r31)` @`0x80608C90` |
| pane-pointer array `+0x23C` | immediate | `addi r5,r31,572` @`0x80608C9C` |

**Confidence: high.** Every word decodes; both string/table constants are
shipped bytes in this image; and `src/game/bases.rs:2541` (`fn_80602160`)
already contains the identical idiom, including the exact comment
`// observed: vtable lives at [screen+0x98] but "this" is screen+0x90.` - here
the pair is `+0xA8`/`+0xA0`.
**Verdict: ready.**

---

## 3. `0x8066B0F0` - 128 bytes - `work/ghidra/rel-d_basesNP/export/8066b0f0_FUN_8066b0f0.c`

**Body:** `0x8066B0F0..0x8066B170` (32 words, `blr` at `0x8066B16C`).
Ledger: absent. Selection gate: no ledger row, no spec title. Export
warnings: none.

```rust
pub unsafe fn fn_8066b0f0(this: usize) -> u32
```

**Argument map.** `r3 -> this` (`mr r31,r3` @`0x8066B100`). No other register is
read before being written. Always returns `1` (`li r3,1` @`0x8066B15C`).

**Frame.** `stwu r1,-16(r1)`; `mfspr r0,LR` / `stw r0,20(r1)`;
`stw r31,12(r1)`. Epilogue `0x8066B158..0x8066B170`. No `psq`.

**Shape.** Four dispatches in a fixed order, then `return 1`. Three of them go
through the vtable word stored **inside** the object at `+0x60`; the fourth goes
through the interface vtable at `+0x394`.

1. `0x8066B104-0x8066B110` - slot `0x2A4` of the `+0x60` vtable:
   ```
   vt = *(u32*)(this + 0x60);      // lwz r12,96(r3)
   fn = *(u32*)(vt + 0x2A4);       // lwz r12,676(r12)
   mtspr CTR,fn; bcctr 20,0,l;     // r3 = this (entry r3, untouched)
   ```
2. `0x8066B114-0x8066B124` - slot `0x2A8`:
   ```
   vt = *(u32*)(this + 0x60);      // lwz r12,96(r31)
   r3 = this;                      // or r3,r31,r31
   fn = *(u32*)(vt + 0x2A8);       // lwz r12,680(r12)
   mtspr CTR,fn; bcctr 20,0,l;
   ```
3. `0x8066B128-0x8066B140` - slot `0xD4` with the BSS buffer:
   ```
   vt = *(u32*)(this + 0x60);      // lwz r12,96(r31)
   r4 = 0x80829644;                // lis r4,0x8083 / addi r4,r4,-27068
   r3 = this;                      // or r3,r31,r31
   fn = *(u32*)(vt + 0xD4);        // lwz r12,212(r12)
   mtspr CTR,fn; bcctr 20,0,l;
   ```
4. `0x8066B144-0x8066B154` - slot `0x1C` of the `+0x394` interface:
   ```
   vt = *(u32*)(this + 0x394);     // lwz r12,916(r31)
   r3 = this + 0x394;              // addi r3,r31,916
   fn = *(u32*)(vt + 0x1C);        // lwz r12,28(r12)
   mtspr CTR,fn; bcctr 20,0,l;
   ```
5. `0x8066B158-0x8066B170` - `li r3,1`, restore `r31`/`LR`, pop, `blr`.

**Receiver note (the one modelling choice).** For slots 1-3 the vtable word is
`*(u32*)(this+0x60)` but the dispatch passes **entry `r3` = `this`**, not
`this+0x60`. This is the established layout of this family: the base
constructor `0x80066FC0` installs the `+0x394` interface and the derived
constructor overwrites `+0x60` with the derived vtable (see the doc comment of
`fn_80651390`). Write it the way merged code already does
(`src/game/bases.rs:64984`):

```rust
let vtable = platform::read_u32(this + 0x60) as usize;
abi_call1!(platform::read_u32(vtable + 0x2A4) as usize, this);
```

Do **not** call the local helper `vmethod0(this + 0x60, 0x2A4)`: it passes
`this+0x60` as `r3` and is one word off the shipped receiver. Slot 4 has no such
quirk - the object *is* `this+0x394`, so `vmethod0(this + 0x394, 0x1C)` is
correct there.

**Callees:**

| site | callee | arguments |
|---|---|---|
| `0x8066B110` | vtable slot `0x2A4` of `*(u32*)(this+0x60)` | `r3 = this` |
| `0x8066B124` | vtable slot `0x2A8` of `*(u32*)(this+0x60)` | `r3 = this` |
| `0x8066B140` | vtable slot `0xD4` of `*(u32*)(this+0x60)` | `r3 = this`, `r4 = 0x80829644` |
| `0x8066B154` | vtable slot `0x1C` of `*(u32*)(this+0x394)` | `r3 = this+0x394` |

**Data context.** The word at `0x807E8E68` is this address: it is the first slot
of an eight-word record `0x807E8E68..0x807E8E88` =
`{0x8066B0F0, 0x80064350, 0x800951D0, 0x8066B440, 0x80064390, 0x800643E0,
0x8066B350, 0x80095240}`, placed immediately after the two zero words that
terminate the string block `"boss_shutter"` / `"boss_shutter_koopa"`
(`0x807E8E3C`) / `"Wm_en_landsmoke"` (`0x807E8E50`). The last sibling,
`0x80095240`, reads both of these fields and dispatches them exactly the way this
body does (`*(u32*)(p+0x394)` then `*(u32*)(p+0x60)`, plus `+0x150` and `+0x28`),
which is what fixes the class layout above.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| vtable word `+0x60` | immediate | `lwz r12,96(r3)` @`0x8066B104`, `lwz r12,96(r31)` @`0x8066B114`/`0x8066B128` |
| interface word `+0x394` | immediate | `lwz r12,916(r31)` @`0x8066B144`, `addi r3,r31,916` @`0x8066B148` |
| slots `0x2A4`, `0x2A8`, `0xD4`, `0x1C` | immediates | `lwz r12,676/680/212/28(r12)` |
| dispatch buffer | `0x80829644` (this REL, zero-fill segment `0x80822900`) | `lis r4,0x8083` @`0x8066B12C` + `addi r4,r4,-27068` @`0x8066B138` |
| `CTR` scratch | - | `mtspr r12,CTR` @`0x8066B10C`/`0x8066B120`/`0x8066B13C`/`0x8066B150` |

**Confidence: medium.** All 32 words decode and all four slots are explicit
offsets in the raw stream (no jump table, no `unaff_GQR`), but two things a
reviewer must accept: the deliberate `this`-vs-`this+0x60` receiver choice for
slots 1-3, and the fact that `0x80829644` is an uninitialised BSS buffer whose
contents at call time cannot be known from the image. Both must survive as
comments:
`// unknown: +0x60 holds the derived vtable; the shipped receiver is this` and
`// unknown: 0x80829644 is a zero-initialised buffer in this REL's BSS`.
**Verdict: conditional.**

---

## 4. `0x806FCCA0` - 100 bytes - `work/ghidra/rel-d_basesNP/export/806fcca0_FUN_806fcca0.c`

**Body:** `0x806FCCA0..0x806FCD04` (25 words, `blr` at `0x806FCD00`).
Ledger: absent. Selection gate: no ledger row, no spec title. Export
warnings: none.

```rust
pub unsafe fn fn_806fcca0(this: usize, free: i32) -> usize
```

**Argument map.** `r3 -> this` (`mr r30,r3` @`0x806FCCBC`), `r4 -> free`
(`mr r31,r4` @`0x806FCCB4`). Returns `r30` (`or r3,r30,r30` @`0x806FCCE8`),
i.e. `this` unchanged.

**Frame.** `stwu r1,-16(r1)`; `mfspr r0,LR` / `stw r0,20(r1)`;
`stw r31,12(r1)` / `stw r30,8(r1)`. Epilogue `0x806FCCE8..0x806FCD04`.

**Shape.** The standard "destroy the `+0x18` sub-object, then optionally free the
container" teardown, with the compiler's duplicated null test.

1. `0x806FCCA8` - `cmpi cr0,r3,0` (the only `cr0` write before the first
   branch; the two `or`s and `stw`s in between do not touch it).
2. `0x806FCCC0` - `beq 0x806FCCE8`: `this == 0` skips everything.
3. `0x806FCCC4` - second `beq 0x806FCCD8`. **Dead as taken**: `cr0` still holds
   `(this == 0)` and we only get here when it is false. CodeWarrior emitted the
   duplicated guard; reproduce the *semantics*, not the extra word (the merged
   twin `fn_806357e0` has no such word).
4. `0x806FCCC8-0x806FCCD4` - sub-object destructor:
   ```
   r3 = this + 0x18;                 // addic. r3,r3,24  (sets cr0)
   if (this + 0x18 != 0) {           // beq 0x806FCCD8
       r4 = 0;                       // li r4,0
       bl 0x8015F9F0;                // (this+0x18, 0)
   }
   ```
5. `0x806FCCD8-0x806FCCE4` - conditional release:
   ```
   if (free <= 0) goto ret;          // cmpi cr0,r31,0 ; ble 0x806FCCE8
   r3 = this;                        // or r3,r30,r30
   bl 0x802B9280;                    // (this)
   ```
6. `0x806FCCE8` - `r3 = this`; restore and `blr`.

**Callees:**

| site | callee | arguments |
|---|---|---|
| `0x806FCCD4` | `0x8015F9F0` (MSL delete helper: `if (p && 0 < n) 0x802B9280(p); return p;`) | `r3 = this+0x18`, `r4 = 0` |
| `0x806FCCE4` | `0x802B9280` (`0x802B8F70(p, 0)` - the sized release) | `r3 = this` |

**Constants:**

| value | image address | materialised at |
|---|---|---|
| sub-object offset `+0x18` | immediate | `addic. r3,r3,24` @`0x806FCCC8` |
| free-flag compare `0` | immediate | `cmpi cr0,r31,0` @`0x806FCCD8` |

**Registration.** The word at `0x80803460` is this address - the first slot of
the eight-word record `0x80803460..0x8080347C` =
`{0x806FCCA0, 0x806FEC80, 0x806FCF70, 0x806FEC90, 0x806FDD20, 0x806FECA0,
0x806FECB0, 0x806FECC0}`, placed after the two zero words at `0x80803458`.

**Confidence: high.** This is the merged `fn_806357e0`
(`src/game/bases.rs:6728`, 96 bytes) plus one dead `beq`: word-for-word equality
outside the inserted branch and the resulting displacement change, identical
callees, identical offsets. A lane can copy that merged body and change nothing
but the name, keeping the comment about the folded `this+0x18` guard.
**Verdict: ready.**

---

## 5. `0x80681480` - 88 bytes - `work/ghidra/rel-d_basesNP/export/80681480_FUN_80681480.c`

**Body:** `0x80681480..0x806814D8` (22 words, `blr` at `0x806814D4`).
Ledger: absent. Selection gate: no ledger row, no spec title. Export
warnings: none.

```rust
pub unsafe fn fn_80681480(this: usize, free: i32) -> usize
```

**Argument map.** `r3 -> this` (`mr r30,r3` @`0x8068149C`), `r4 -> free`
(`mr r31,r4` @`0x80681494`). Returns `this` (`or r3,r30,r30` @`0x806814BC`).

**Frame.** `stwu r1,-16(r1)`; `mfspr r0,LR` / `stw r0,20(r1)`;
`stw r31,12` / `stw r30,8`. Epilogue `0x806814BC..0x806814D8`.

**Shape.** `daFloorDokanEight_c::StateID_Move`'s state-record destructor: run the
generic name/state destructor on the object, then release the storage only when
the caller's flag is positive. Note that the destructor call is **not** guarded
by `free` - it runs for any non-null object.

1. `0x80681488` `cmpi cr0,r3,0`; `0x80681490`/`0x80681498` copy the arguments;
   `0x806814A0` `beq 0x806814BC` leaves on `this == 0`.
2. `0x806814A4-0x806814A8` - `li r4,0`; `bl 0x8015F800` with `r3` still the
   entry receiver (it is never written between entry and this call).
3. `0x806814AC-0x806814B8` - `cmpi cr0,r31,0`; `ble 0x806814BC`;
   `or r3,r30,r30`; `bl 0x802B9280`.
4. `0x806814BC` - `or r3,r30,r30`; restore and `blr`.

**Callees:**

| site | callee | arguments |
|---|---|---|
| `0x806814A8` | `0x8015F800` (same MSL helper as above) | `r3 = this` (entry `r3`, unmodified), `r4 = 0` |
| `0x806814B8` | `0x802B9280` | `r3 = this` |

**Constants:**

| value | image address | materialised at |
|---|---|---|
| free-flag compare `0` | immediate | `cmpi cr0,r31,0` @`0x806814AC` |
| destructor second argument `0` | immediate | `li r4,0` @`0x806814A4` |

**Registration.** The word at `0x807ECE08` is this address - the first slot of
the eight-word state record `0x807ECE08..0x807ECE28` =
`{0x80681480, 0x8015F840, 0x8015F850, 0x8015F8C0, 0x8015F8D0, 0x806814E0,
0x8015F920, 0x8015F930}`. It sits directly after the name string
`"daFloorDokanEight_c::StateID_Move"` at `0x807ECDDC` (terminated by the two
zero words at `0x807ECE00`/`0x807ECE04`), and its sibling slot `0x806814E0` is
the record's own body.

**Confidence: high.** Word-for-word equal to the merged `fn_806511d0`
(`src/game/bases.rs:4242`) except the two `bl` displacements, and the
`functions.tsv`/string data confirm the class and state. The wave-123 note
("byte-identical copies ... only the first was translated") is confirmed: the
only differences are the two branch words.
**Verdict: ready.**

---

## 6. `0x80681FA0` - 88 bytes - `work/ghidra/rel-d_basesNP/export/80681fa0_FUN_80681fa0.c`

**Body:** `0x80681FA0..0x80681FF8` (22 words, `blr` at `0x80681FF4`).
Ledger: absent. Selection gate: no ledger row, no spec title. Export
warnings: none.

```rust
pub unsafe fn fn_80681fa0(this: usize, free: i32) -> usize
```

**Argument map, frame and shape** are identical to §5 word for word:
`mr r30,r3` @`0x80681FBC`, `mr r31,r4` @`0x80681FB4`, guard
`cmpi cr0,r3,0` @`0x80681FA8` + `beq 0x80681FDC` @`0x80681FC0`,
`li r4,0` @`0x80681FC4`, `bl 0x8015F800` @`0x80681FC8`,
`cmpi cr0,r31,0` @`0x80681FCC` + `ble 0x80681FDC` @`0x80681FD0`,
`or r3,r30,r30` @`0x80681FD4`, `bl 0x802B9280` @`0x80681FD8`,
return `this` @`0x80681FDC`.

**Callees:** `0x8015F800(this, 0)` @`0x80681FC8`, `0x802B9280(this)`
@`0x80681FD8` - the same pair as §5. The two bodies differ only in the `bl`
displacements (`0x4BADD839` and `0x4BC372A9` here versus `0x4BADE359` and
`0x4BC37DC9` in §5).

**Registration.** The word at `0x807ED0F8` is this address - the first slot of
the eight-word record `0x807ED0F8..0x807ED118` =
`{0x80681FA0, 0x8015F840, 0x8015F850, 0x8015F8C0, 0x8015F8D0, 0x80682000,
0x8015F920, 0x8015F930}`, directly after `"daFloorGyration_c::StateID_Move"` at
`0x807ED0D0`. This is the `daFloorGyration_c` twin of §5.

**Constants:** none beyond the two immediates (`0` free threshold, `0`
destructor argument) already listed in §5.

**Confidence: high.** Same evidence as §5, with the class/state name fixed by
the adjacent string.
**Verdict: ready.**

---

## 7. `0x806FF3F0` - 88 bytes - `work/ghidra/rel-d_basesNP/export/806ff3f0_FUN_806ff3f0.c`

**Body:** `0x806FF3F0..0x806FF448` (22 words, `blr` at `0x806FF444`).
Ledger: absent. Selection gate: no ledger row, no spec title. Export
warnings: none.

```rust
pub unsafe fn fn_806ff3f0(this: usize, free: i32) -> usize
```

**Argument map.** `r3 -> this` (`mr r30,r3` @`0x806FF40C`), `r4 -> free`
(`mr r31,r4` @`0x806FF404`). Returns `this` (`or r3,r30,r30` @`0x806FF42C`).

**Frame.** `stwu r1,-16(r1)`; `mfspr r0,LR` / `stw r0,20(r1)`;
`stw r31,12` / `stw r30,8`. Epilogue `0x806FF42C..0x806FF448`.

**Shape.** Same control flow as §5/§6 but with the *base actor* destructor and
the *game* allocator instead of the MSL pair:

1. `0x806FF3F8` `cmpi cr0,r3,0`; arguments copied at `0x806FF404`/`0x806FF40C`;
   `0x806FF410` `beq 0x806FF42C`.
2. `0x806FF414-0x806FF418` - `li r4,0`; `bl 0x800642B0` with `r3 = this`.
3. `0x806FF41C-0x806FF428` - `cmpi cr0,r31,0`; `ble 0x806FF42C`;
   `or r3,r30,r30`; `bl 0x80162920`.
4. `0x806FF42C` - `or r3,r30,r30`; restore and `blr`.

**Callees:**

| site | callee | arguments |
|---|---|---|
| `0x806FF418` | `0x800642B0` (main.dol object destructor: overwrites `+0x60` with `0x80309F6C`, then tears down `+0x148`, `+0x2DC`, `+0x1EC`, and calls `0x8006C7F0(this, 0)`; when its second argument is positive it also calls `0x80162920`) | `r3 = this`, `r4 = 0` |
| `0x806FF428` | `0x80162920` (`0x802B8F70(p, *(u32*)0x80377C48)` - release to the current heap) | `r3 = this` |

**Constants:**

| value | image address | materialised at |
|---|---|---|
| free-flag compare `0` | immediate | `cmpi cr0,r31,0` @`0x806FF41C` |
| destructor second argument `0` | immediate | `li r4,0` @`0x806FF414` |

**Registration.** The word at `0x8080378C` is this address, inside the table run
`0x80803788..0x808037A8` = `{0x801628B0, 0x806FF3F0, 0x800645F0, 0x8006CA50,
0x8006CA60, 0x80649600, 0x806495F0, 0x80065080}`; the preceding entries
(`0x806FF360` at `0x80803770`, `0x806FF370` at `0x8080377C`) are its siblings in
the same manager block.

**Confidence: high.** The body is the merged `fn_80650600`
(`src/game/bases.rs:3385`) minus the `+0x3AC` sub-object destructor: same guard,
same two callees (`0x800642B0` then `0x80162920`), same return of `this`. That
merged function's doc comment already names both callees.
**Verdict: ready.**

---

## 8. `0x80600990` - 64 bytes - `work/ghidra/rel-d_basesNP/export/80600990_FUN_80600990.c`

**Body:** `0x80600990..0x806009D0` (16 words, `blr` at `0x806009CC`) - the body
ends exactly where `0x806009D0` (§9) begins. Ledger: absent. Selection gate: no
ledger row, no spec title. Export warnings: none.

```rust
pub unsafe fn fn_80600990(this: usize, free: i32) -> usize
```

**Argument map.** `r3 -> this` (`mr r31,r3` @`0x806009A4`), `r4 -> free` (read
only by the `cmpi` at `0x806009AC`). Returns `this`
(`or r3,r31,r31` @`0x806009B8`).

**Frame.** `stwu r1,-16(r1)`; `mfspr r0,LR` / `stw r0,20(r1)`;
`stw r31,12(r1)`. Epilogue `0x806009B8..0x806009D0`.

**Shape.** The minimal release wrapper - no destructor, one conditional free:

1. `0x80600998` `cmpi cr0,r3,0`; `0x806009A8` `beq 0x806009B8`.
2. `0x806009AC-0x806009B4` - `cmpi cr0,r4,0`; `ble 0x806009B8`;
   `bl 0x802B9280`.
3. `0x806009B8` - `or r3,r31,r31`; restore and `blr`.

**The dropped argument is not dropped.** Wave 116 wrote that "every call
argument [is] dropped" and that "the missing argument makes the ownership
semantics unprovable". The raw stream settles it: between entry and
`bl 0x802B9280` the only write to `r3` is `or r31,r3,r3` (a *copy* to `r31`), so
the callee receives **entry `r3` = `this`**. Ghidra's `func_0x802b9280()` with
no argument is a decompiler artefact, exactly as `fn_80651350`'s merged
confidence note already says.

**Callees:** `0x802B9280(this)` @`0x806009B4` only.

**Constants:** free-flag compare `0` @`0x806009AC` (immediate).

**Registration.** The word at `0x807D1030` is this address; the neighbouring
16-byte records are `0x807D1020 = {0x806009D0, 0x80601600, 0, 0}`,
`0x807D1040 = {0x80600A10, 0, 0, 0}`, `0x807D1058 = {0x80600A50, ...}` and
`0x807D0FC0 = {0x80600AD0, 0x80601610, 0x80601670, 0}` - the whole family of
wrappers lives in one table block at `0x807D0FC0..0x807D1074`.

**Confidence: high.** Word-for-word equal to the merged `fn_80651350`
(`src/game/bases.rs:4335`) except the single `bl` displacement
(`0x4BCB88CD` here versus `0x4BC67F0D`). Copy that body and its
`// observed:` comment.
**Verdict: ready.**

---

## 9. `0x806009D0` - 64 bytes - `work/ghidra/rel-d_basesNP/export/806009d0_FUN_806009d0.c`

**Body:** `0x806009D0..0x80600A10` (16 words, `blr` at `0x80600A0C`).
Ledger: absent. Selection gate: no ledger row, no spec title. Export
warnings: none.

```rust
pub unsafe fn fn_806009d0(this: usize, free: i32) -> usize
```

**Argument map, frame and shape:** identical to §8 with the addresses shifted by
64 bytes - `cmpi cr0,r3,0` @`0x806009D8`, `mr r31,r3` @`0x806009E4`,
`beq 0x806009F8` @`0x806009E8`, `cmpi cr0,r4,0` @`0x806009EC`,
`ble 0x806009F8` @`0x806009F0`, `bl 0x802B9280` @`0x806009F4`,
`or r3,r31,r31` @`0x806009F8`, epilogue `0x806009F8..0x80600A10`.

**Callees:** `0x802B9280(this)` @`0x806009F4`.

**Constants:** free-flag compare `0` @`0x806009EC` (immediate).

**Registration.** The word at `0x807D1020` opens the 4-word record
`{0x806009D0, 0x80601600, 0, 0}`; slot 1 is `0x80601600`, the sibling entry of
this record in the same table block (its role is not fixed by the data alone).

**Confidence: high.** Differs from §8 in exactly one word (`0x4BCB888D` versus
`0x4BCB88CD` at index 9, both `bl 0x802B9280`). Same template
(`fn_80651350`, `src/game/bases.rs:4335`).
**Verdict: ready.**

---

## 10. `0x80600A10` - 64 bytes - `work/ghidra/rel-d_basesNP/export/80600a10_FUN_80600a10.c`

**Body:** `0x80600A10..0x80600A50` (16 words, `blr` at `0x80600A4C`).
Ledger: absent. Selection gate: no ledger row, no spec title. Export
warnings: none.

```rust
pub unsafe fn fn_80600a10(this: usize, free: i32) -> usize
```

**Argument map, frame and shape:** identical to §8/§9 with the addresses shifted
a further 64 bytes - `cmpi cr0,r3,0` @`0x80600A18`, `mr r31,r3`
@`0x80600A24`, `beq 0x80600A38` @`0x80600A28`, `cmpi cr0,r4,0`
@`0x80600A2C`, `ble 0x80600A38` @`0x80600A30`, `bl 0x802B9280`
@`0x80600A34`, `or r3,r31,r31` @`0x80600A38`, epilogue
`0x80600A38..0x80600A50`.

**Callees:** `0x802B9280(this)` @`0x80600A34`.

**Constants:** free-flag compare `0` @`0x80600A2C` (immediate).

**Registration.** The word at `0x807D1040` opens the 4-word record
`{0x80600A10, 0, 0, 0}`; the two zero words at `0x807D1050`/`0x807D1054`
separate it from the next entry, `0x807D1058 = {0x80600A50, ...}` (§11).

**Confidence: high.** Differs from §8 in exactly one word (`0x4BCB884D` at index
9). Same template (`fn_80651350`).
**Verdict: ready.**

---

## 11. `0x80600A50` - 64 bytes - `work/ghidra/rel-d_basesNP/export/80600a50_FUN_80600a50.c`

**Body:** `0x80600A50..0x80600A90` (16 words, `blr` at `0x80600A8C`).
Ledger: absent. Selection gate: no ledger row, no spec title. Export
warnings: none.

```rust
pub unsafe fn fn_80600a50(this: usize, free: i32) -> usize
```

**Argument map, frame and shape:** identical to §8-§10 with the addresses shifted
a further 64 bytes - `cmpi cr0,r3,0` @`0x80600A58`, `mr r31,r3`
@`0x80600A64`, `beq 0x80600A78` @`0x80600A68`, `cmpi cr0,r4,0`
@`0x80600A6C`, `ble 0x80600A78` @`0x80600A70`, `bl 0x802B9280`
@`0x80600A74`, `or r3,r31,r31` @`0x80600A78`, epilogue
`0x80600A78..0x80600A90`.

**Callees:** `0x802B9280(this)` @`0x80600A74`.

**Constants:** free-flag compare `0` @`0x80600A6C` (immediate).

**Registration.** The word at `0x807D1058` is this address, in the same table
run as §10: `0x807D1040 = {0x80600A10, 0, 0, 0}`,
`0x807D1058 = {0x80600A50, 0, 0, 0}`, and `0x807D106C` is `0xFFFFFFFF` followed
by `0x80601000`. The wrapper is therefore a "release only" slot in the same
table block as §8-§10.

**Confidence: high.** Differs from §8 in exactly one word (`0x4BCB880D` at index
9). Same template (`fn_80651350`).
**Verdict: ready.**

---

## 12. `0x80600A90` - 64 bytes - `work/ghidra/rel-d_basesNP/export/80600a90_FUN_80600a90.c`

**Body:** `0x80600A90..0x80600AD0` (16 words, `blr` at `0x80600ACC`) - the body
ends exactly where the 13th candidate `0x80600AD0` begins. Ledger: absent.
Selection gate: no ledger row, no spec title. Export warnings: none.

```rust
pub unsafe fn fn_80600a90(this: usize, free: i32) -> usize
```

**Argument map, frame and shape:** identical to §8-§11 with the addresses shifted
a further 64 bytes - `cmpi cr0,r3,0` @`0x80600A98`, `mr r31,r3`
@`0x80600AA4`, `beq 0x80600AB8` @`0x80600AA8`, `cmpi cr0,r4,0`
@`0x80600AAC`, `ble 0x80600AB8` @`0x80600AB0`, `bl 0x802B9280`
@`0x80600AB4`, `or r3,r31,r31` @`0x80600AB8`, epilogue
`0x80600AB8..0x80600AD0`.

**Callees:** `0x802B9280(this)` @`0x80600AB4`.

**Constants:** free-flag compare `0` @`0x80600AAC` (immediate).

**Registration.** The word at `0x807D0FD8` opens the 4-word record
`{0x80600A90, 0x80601680, 0x806016A0, 0x806016C0}`; the record immediately
before it, `0x807D0FC0 = {0x80600AD0, 0x80601610, 0x80601670, 0}`, holds the 13th
candidate and confirms that this family is a run of sibling wrappers rather than
one relocated body.

**Confidence: high.** Differs from §8 in exactly one word (`0x4BCB87CD` at index
9). Same template (`fn_80651350`).
**Verdict: ready.**

---

## Summary

| address | bytes | words | verdict | confidence | what a lane must record |
|---|---:|---:|---|---|---|
| `0x806B3AC0` | 136 | 34 | **conditional** | medium | `0x800BC950` has no `main-gb` row; `0x80377C48` is the main.dol heap handle; `0x800690C0` 5-arg form already merged at `bases.rs:3566` |
| `0x80608C40` | 132 | 33 | **ready** | high | table `0x807D2888` (13 pane names) + `"worldCollectionCoinBase_01.brlyt"` at `0x807D28BC`; vtable `[this+0xA8]` slot `0x14`, receiver `this+0xA0` |
| `0x8066B0F0` | 128 | 32 | **conditional** | medium | `+0x60` holds the derived vtable but the shipped receiver is `this`; `0x80829644` is an uninitialised BSS buffer |
| `0x806FCCA0` | 100 | 25 | **ready** | high | merged twin `fn_806357e0`; one extra dead `beq` at `0x806FCCC4` need not be modelled |
| `0x80681480` | 88 | 22 | **ready** | high | merged twin `fn_806511d0`; class `daFloorDokanEight_c::StateID_Move` |
| `0x80681FA0` | 88 | 22 | **ready** | high | same twin; class `daFloorGyration_c::StateID_Move` |
| `0x806FF3F0` | 88 | 22 | **ready** | high | `0x800642B0` + `0x80162920` pair, as in merged `fn_80650600` |
| `0x80600990` | 64 | 16 | **ready** | high | twin of merged `fn_80651350`; `r3` is entry `this` at the free |
| `0x806009D0` | 64 | 16 | **ready** | high | same body, `bl` displacement only |
| `0x80600A10` | 64 | 16 | **ready** | high | same body, `bl` displacement only |
| `0x80600A50` | 64 | 16 | **ready** | high | same body, `bl` displacement only |
| `0x80600A90` | 64 | 16 | **ready** | high | same body, `bl` displacement only |

**10 ready, 2 conditional.** Ten of the twelve can be written today by copying a
merged neighbour and changing nothing but names, offsets and constants; the two
conditional ones are implementable today as well, but each carries exactly one
inference a reviewer must accept (`0x800BC950`'s ABI in §1, the
`this`/`this+0x60` receiver in §3), and the lane must leave the `// unknown:`
comment named above. No body in this batch needs another artifact to be
*decoded* - the whole 1 080 bytes are accounted for word by word.

At write time a parallel translation lane had already merged eleven of the
twelve (`0x806FCCA0` is the only one still missing from `src/game/bases.rs`),
and the merges match this spec; see the cross-check in "Verification performed".
The verdicts above are about *reconstructibility*, not about who writes the
code.

## Verification performed

**Concurrency note (2026-11-12, at write time).** Wave 136 runs translation
lanes in parallel with this one. Every "Ledger: absent" line in this document
means "absent from `ledger/functions.csv` and from every existing spec title",
not "absent from a work-in-progress Rust file at this minute".

- Candidate pool recomputed from `docs/ANALYSIS_WAVE127_BASES.md`: 121 class-B
  rows (51 060 bytes); 88 of them have an exact address match in
  `ledger/functions.csv`; 100 are named by a `##` title in
  `docs/SPEC_WAVE128_BASES.md` .. `docs/SPEC_WAVE135_BASES.md`; the
  intersection covers all 88 ledger rows. **21 addresses / 1 608 bytes** remain,
  of which this lane takes the twelve largest (136, 132, 128, 100, 88, 88, 88,
  64 x 5), breaking ties by ascending address. After this batch **9 addresses /
  528 bytes** remain.
- `ledger/functions.csv` checked **row-based** across all modules: zero address
  matches for all twelve.
- Every byte count equals the `functions.tsv` `size` column, every row name
  matches its export file, and every export file exists at the cited path
  (12/12).
- Extent check: for all twelve `u32(entry + size - 4) == 0x4E800020`, exactly
  one `bclr` per body, and no `functions.tsv` row starts inside a body. §8 ends
  exactly at `0x806009D0`, §9 at `0x80600A10`, §10 at `0x80600A50`, §11 at
  `0x80600A90` and §12 at `0x80600AD0`, so the five wrappers are contiguous.
- Duplicate check: SHA-1 of each of the twelve ranges compared against every
  same-size `d_basesNP` TSV row - no match. The hashes are `8c5723d0..` (§1),
  `0e6fbdca..` (§2), `059d0291..` (§3), `66606246..` (§4), `04622f79..` (§5),
  `82b5d7ff..` (§6), `19512f9b..` (§7), `ccfa9307..` (§8), `9dc9aef5..` (§9),
  `f76d687c..` (§10), `5cf03f24..` (§11), `23cc1642..` (§12).
- Shape/twin audit (word-level, not hash-level): §4 is `fn_806357e0` plus one
  word; §5/§6 are `fn_806511d0` with two branch words changed; §7 shares that
  shape with different callees; §8-§12 are `fn_80651350` with one branch word
  changed. The exact differing indices are tabulated in "Method".
- Paired-single audit: **zero** `psq*` words in 270 words; no `GQRn` write; the
  only `mtspr` operands are `LR` (every epilogue) and `CTR` (§2 once, §3 four
  times).
- Callee cross-checks from `work/ghidra/main-gb/export/`: `0x800690C0` ->
  `0x8016A7F0`; `0x800690E0` -> `0x8016A8E0`; `0x80007220` =
  `*(u32*)(p+0x20)`; `0x802B9280` = `0x802B8F70(p, 0)`; `0x8015F800` and
  `0x8015F9F0` both = `if (p && 0 < n) 0x802B9280(p); return p;`;
  `0x80162920` = `0x802B8F70(p, *(u32*)0x80377C48)`; `0x800642B0` = full actor
  destructor (writes `+0x60`, tears down `+0x148`/`+0x2DC`/`+0x1EC`, then
  `0x8006C7F0(this,0)`); `0x800C8D60` = pane-name binder (already specified as
  `FN_800C8D60` in `src/engine/display.rs:2735`).

  **`0x802B9280` is a one-argument function.** Both its export
  (`FUN_802b9280(param_1)`) and its body (`0x802B8F70(param_1, 0)`) take a single
  pointer; the shipped call sites leave `r4` holding whatever the previous call
  left there. Render it as `abi_call1!(0x802B_9280, p)`. In §5/§6 the preceding
  `li r4,0` belongs to the *preceding* destructor call, so `r4` at the free is
  not even known to be 0; a second argument would be an invention.
- `0x800BC950` decoded from `work/dol/main.elf`: 232 bytes
  (`0x800BC950..0x800BCA38`), a real entry between the `blr`/padding at
  `0x800BC944..0x800BC94C` and `FUN_800bca40`. Its strings are in the image at
  `0x80315C58` (`"g3d/lift_yogan_kinoko.brres"`), `0x80315C74`
  (`"lift_yogan_kinoko"`) and `0x80315C88` (`"spin_kinoko_B"`).
- In-image constants read directly: §2 `0x807D2888` -> `0x807D27B0`
  (`"N_coinDateAll_00"`), `0x807D288C..0x807D28B8` -> `"N_coinBarPos_00"` ..
  `"N_coinBarPos_11"`, `0x807D28BC` = `"worldCollectionCoinBase_01.brlyt"`;
  §5 `0x807ECDDC` = `"daFloorDokanEight_c::StateID_Move"`; §6 `0x807ED0D0` =
  `"daFloorGyration_c::StateID_Move"`; §3 `0x807E8E3C` =
  `"boss_shutter_koopa"`, `0x807E8E50` = `"Wm_en_landsmoke"`.
- Registration sites read as data words: §3 `0x807E8E68`, §4 `0x80803460`,
  §5 `0x807ECE08`, §6 `0x807ED0F8`, §7 `0x8080378C`, §8 `0x807D1030`,
  §9 `0x807D1020`, §10 `0x807D1040`, §11 `0x807D1058`, §12 `0x807D0FD8`. §1 and
  §2 have no data word; each has exactly one direct `bl` call site
  (`0x806B3F14` and `0x80608BE8`), both inside bodies that have no
  `functions.tsv` row of their own.
- Cross-check against the parallel translation lane (read-only): eleven of the
  twelve bodies now have a `pub unsafe fn` in `src/game/bases.rs`
  (`fn_806b3ac0` :172605, `fn_80608c40` :172658, `fn_8066b0f0` :172715,
  `fn_80681480` :172763, `fn_80681fa0` :172802, `fn_806ff3f0` :172839,
  `fn_80600990` :172877, `fn_806009d0` :172909, `fn_80600a10` :172940,
  `fn_80600a50` :172971, `fn_80600a90` :173002); §4 `0x806FCCA0` had none at
  write time. They agree with this spec on every receiver, offset, string and
  slot. Two deliberate differences were checked and are not errors:
  (a) `fn_806b3ac0` uses `call_f32x2_u32x3` for the `0x800BC950` call - the
  mixed-ABI helper at `src/game/bases.rs:35822` - which is the same ABI order
  specified in §1; (b) the merged §5/§6/§7/§8-§12 bodies render the free as
  `0x802B9280(p, 0)`/`(p, count)`, while the callee takes one argument (see the
  ABI note above); the extra register is harmless in the image but should not be
  read as part of the prototype.
- Cross-artifact cells: `0x80377C48` is inside `main.elf`'s zero-fill segment
  `0x80351680` (+ `0xD6000`); `0x80829644` is inside this REL's zero-fill
  segment `0x80822900` (+ `0x12484`). Neither has shipped bytes, so both are
  cited as addresses only.
- This lane wrote no repository file except this document and ran no merge or
  check tool; the `src/game/bases.rs` changes cited above were made by the
  parallel translation lane, not by this one.

## Hand-off

The pool is now **9** addresses (**528** bytes). In the same order (size
descending, then ascending address) they are `0x80600AD0` (64), `0x80601D10`
(64), `0x80601EB0` (64), `0x80605500` (64), `0x806FCC60` (64), `0x806FCD70`
(64), `0x806FCDB0` (64), `0x8069ACA0` (40) and `0x806FF700` (40).

Seven of the nine are more copies of the §8 shape. This lane decoded all seven
and they are **word-for-word identical to §8 after masking the branch
displacements** (the same 16 masked words): `0x80600AD0` (`bl 0x802B9280` at
`0x80600AF4`, `0x4BCB878D`), `0x80601D10` (`0x4BCB754D`), `0x80601EB0`
(`0x4BCB73AD`), `0x80605500` (`0x4BCB3D5D`), `0x806FCC60` (`0x4BBBC5FD`),
`0x806FCD70` (`0x4BBBC4ED`) and `0x806FCDB0` (`0x4BBBC4AD`) - all calling the
same free entry `0x802B9280`. Each therefore takes the same
`fn_XXXXXXXX(this: usize, free: i32) -> usize` signature, and
`0x80600AD0` opens the record `{0x80600AD0, 0x80601610, 0x80601670, 0}` at
`0x807D0FC0`. They need no new spec, only the usual per-address `bl`
displacement and table word.

The two 40-byte bodies were also decoded here:

- **`0x8069ACA0`** (10 words, `bcctr 20,0` as the last word - a *tail* dispatch,
  no `blr`): `lis r5,0x807F` + `addi r5,r5,0x1340` -> `r5 = 0x807F1340`;
  `lha r0,0x20(r5)` reads the signed halfword at `0x807F1360`;
  `stw r0,0xD84(r3)` stores it at `this+0xD84`; `r4 = 0x8082B5AC`;
  `vt = *(u32*)(this+0x60)`, `fn = *(u32*)(vt+0xD4)`, `mtspr CTR,fn`,
  `bcctr` with `r3 = this`. It is the "stamp the id, then forward to the `+0xD4`
  slot" variant of §3's third call, and is **ready** as soon as the reader
  accepts that the tail call does not return here.
- **`0x806FF700`** (10 words): `addi r3,r3,960` -> `this+0x3C0`;
  `bl 0x8007F950`; `li r3,1`; epilogue `0x806FF714..0x806FF728`. A
  return-`1` wrapper around one external call, identical in shape to the tail of
  §3 and **ready**.

A wave 137 following this rule would take the four remaining 64-byte wrappers
(`0x80600AD0`, `0x80601D10`, `0x80601EB0`, `0x80605500`) and then the three
`0x806FCC60`/`0x806FCD70`/`0x806FCDB0` neighbours, leaving the two 40-byte
bodies for the final pass - or, since all seven are now fully described above,
take the two 40-byte bodies first and close the pool.
