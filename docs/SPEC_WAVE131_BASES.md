# Wave 131 - `game::bases` - implementation spec for the 12 largest class-B addresses

(…plus three overflow bodies in §13-§15 that the strictest reading of "absent
from the existing spec files" would substitute; see "Why these twelve".)

Read-only lane. The only file written is this one; the throwaway PowerPC decoder
and the ELF/section readers live in `%TEMP%\opencode` and never entered the
repository. No Rust file, no `ledger/functions.csv` row and no existing document
was modified.

## Why these twelve

`docs/ANALYSIS_WAVE127_BASES.md` reclassified 121 `d_basesNP` skips as class B
("reconstructible now from the raw stream"), totalling **51 060 bytes**. Waves
128, 129 and 130 already specified 36 of them (**14 588 + 9 932 + 6 992** bytes,
31 512 total), so the remaining class-B pool is **85 addresses / 19 548 bytes**
once the three existing spec lanes are subtracted and the ledger
(`game::bases`) is checked. This lane takes the twelve largest of what is left:

| # | address | bytes | words | export file | verdict |
|--:|---:|---:|---:|---|---|
| 1 | `0x8066FA10` | 508 | 127 | `8066fa10_FUN_8066fa10.c` | ready (medium) |
| 2 | `0x806A4CA0` | 504 | 126 | `806a4ca0_FUN_806a4ca0.c` | ready (high) |
| 3 | `0x807816B0` | 460 | 115 | `807816b0_FUN_807816b0.c` | conditional (medium) |
| 4 | `0x80762CE0` | 440 | 110 | `80762ce0_FUN_80762ce0.c` | conditional (medium) |
| 5 | `0x806333D0` | 416 | 104 | `806333d0_FUN_806333d0.c` | conditional (medium) |
| 6 | `0x8069AB00` | 408 | 102 | `8069ab00_FUN_8069ab00.c` | ready (high) |
| 7 | `0x807616E0` | 408 | 102 | `807616e0_FUN_807616e0.c` | ready (medium) |
| 8 | `0x80656310` | 396 | 99 | `80656310_FUN_80656310.c` | ready (medium) |
| 9 | `0x80656D20` | 392 | 98 | `80656d20_FUN_80656d20.c` | ready (medium) |
| 10 | `0x806A75B0` | 392 | 98 | `806a75b0_FUN_806a75b0.c` | ready (high) |
| 11 | `0x807819B0` | 388 | 97 | `807819b0_FUN_807819b0.c` | ready (high) |
| 12 | `0x80780010` | 376 | 94 | `80780010_FUN_80780010.c` | ready (medium) |

Together **5 088 bytes**. Every export path is exactly
`work/ghidra/rel-d_basesNP/export/<file>` relative to the repository root, and
every `bytes` value matches the `size` column of
`work/ghidra/rel-d_basesNP/export/functions.tsv` verbatim.

### Selection rule, and the three addresses it excludes

The candidate set is "class B in wave 127, no `game::bases` ledger row, no
`## N.` heading in waves 128-130". That is the rule recorded at
`docs/SPEC_WAVE129_BASES.md:1464-1468` and reused at
`docs/SPEC_WAVE130_BASES.md:1283-1286`: **a textual mention is not a claim** -
only a section heading is. Applying it in rank order gives exactly the twelve
above, and it is what puts items 1-3 in this batch: each is *mentioned* in an
earlier lane but never specified.

| address | bytes | where it is mentioned | why it still counts |
|---|---:|---|---|
| `0x8066FA10` | 508 | `SPEC_WAVE130_BASES.md:46` and `:125` | both lines say it is "behind #12" / "outside this batch" |
| `0x806A4CA0` | 504 | `SPEC_WAVE130_BASES.md:46`, `:53` | same; `:53` calls its Rust occurrence a "comment-only cross-reference" |
| `0x807816B0` | 460 | `SPEC_WAVE129_BASES.md:1080/1096/1117`, `SPEC_WAVE130_BASES.md:47-48` | mentioned only as a *callee* of `0x80781648`; wave 130 confirms it is "outside this batch" |

A reviewer who instead reads "absent from the spec files" as "not mentioned at
all" should drop items 1-3 and take `0x806B7340` (368), `0x80749750` (360) and
`0x806B7B70` (344) instead. All three of those are written up as **§13, §14 and
§15**, so this document covers both readings and no work is lost.

## Method

**Address mapping (verified, not copied).** `d_basesNP.elf` has six `PT_LOAD`
headers. The first is `p_offset = 0x10F0`, `p_vaddr = 0x806000F0`, so

```
file_offset = 0x10F0 + (address - 0x806000F0) = (address - 0x80600000) + 0x1000
```

The remaining five `PT_LOAD`s (`0x807C60F4`, `0x807C65F0`, `0x807C6600`,
`0x807D0C00`, and the zero-fill segment at `0x80822900`) were resolved with the
same `p_offset + (addr - p_vaddr)` rule, so every `.data` / `.bss` constant
quoted below was read at its true file offset, not at `addr - 0x80600000`.

**Decoder facts re-derived on this batch.** The throwaway decoder implements
the Gekko/Broadway subset needed here. Two details matter for reading the
tables below:

* `rlwinm rA,rS,SH,MB,ME` - the mask operands are **MSB-numbered**. A mask of
  `31,31` is the *least significant bit*, not a 1-bit-wide field at the top.
  `rlwinm r0,r0,3,31,31` therefore extracts `CR0.EQ`, and
  `rlwinm r29,r29,1,31,31` extracts `CR0.GT`. Getting this wrong inverts every
  floating-point predicate in the batch.
* In this build `psq_l = 56`, `psq_lu = 57`, `psq_st = 60`, `psq_stu = 61`;
  opcode 4 is the paired-single ALU (`ps_merge*`, `ps_add`, ...). Opcodes 59
  (single-precision A-form) and 63 (double-precision X-form) are *not* `psq`.

**Compiler save chain, extended.** Waves 128 and 130 documented
`0x802DCD40..0x802DCD84`. This batch additionally calls three members that were
not in that table. The full ladder was re-decoded from `work/dol/main.elf`
(text segment `0x80006780..0x802ED9E0`):

| save stub | saves | matching restore |
|---|---|---|
| `0x802DCD40` | r14-r31 | `0x802DCD8C` |
| `0x802DCD48` | r16-r31 | `0x802DCD94` |
| `0x802DCD50` | r18-r31 | `0x802DCD9C` |
| `0x802DCD54` | r19-r31 | `0x802DCDA0` |
| **`0x802DCD68`** | r24-r31 | **`0x802DCDB4`** |
| **`0x802DCD6C`** | r25-r31 | **`0x802DCDB8`** |
| **`0x802DCD70`** | r26-r31 | **`0x802DCDBC`** |
| `0x802DCD74` | r27-r31 | `0x802DCDC0` |

Each stub is a straight run of `stw rN,-off(r11)` ending in `blr`; none writes
`r3..r13` and none touches `LR`. `r11` must be `r1 + frame_size`. This is what
makes `0x806333D0`, `0x8066FA10`, `0x806A75B0`, `0x806B7340` and `0x80749750`
reconstructible: after the `bl`, the receiver is still in entry `r3` and every
body copies it into a callee-saved register on the next instruction. Note that
`0x806333D0` uses `r26`, which is exactly the lowest register `0x802DCD70`
preserves, and `0x8066FA10` uses `r24`, exactly the lowest `0x802DCD68`
preserves - the two facts confirm each other.

**Paired-single audit for this batch.** Across the 2 624 words of the fifteen
bodies there are 34 `psq*` words, all `qr0` (`I = 0`) and all `W = 0`: 17
prologue `psq_st fNN` saves and the 17 matching epilogue `psq_l fNN` reloads.
They cover seventeen saved float registers - one per body, except `0x806A75B0`
(`f31`+`f30`) and `0x8066FA10` (`f31`+`f30`+`f29`+`f28`). Every one is paired
with an adjacent `stfd` / `lfd` of the same register. Two bodies (`0x80656D20`,
`0x80780010`) contain **no** `psq` at all. There is **no** `mtspr` to any
`GQRn`; the only `mtspr` operands are `LR` (spr 8) and `CTR` (spr 9). The
wave-119/120/123/124/125 "`unaff_GQR0`" rejections of these addresses were
decompiler artefacts, exactly as wave 127 predicted.

**One polarity correction to the exports.** `0x807819B0` and `0x806333D0` both
sit behind a `fcmpo` + `cror eq,lt,eq` + `mfcr` + `rlwinm` predicate. In
`0x807819B0` Ghidra's rendered comparison is the *negation* of the shipped
instructions (see §11); the instruction stream is authoritative.

**House style used below.** `pub unsafe fn fn_XXXXXXXX(this: usize)`,
`platform::read_u32/read_i32/read_f32/read_u8/read_u16`,
`platform::write_*`, `abi_call1!..abi_call6!`, `vmethod0(rcv, slot)`,
`ptr_at(addr)`, addresses written `0x806F_3350`, field offsets as `usize`
literals, and a `// unknown:` comment on the first use of every address-only
target or external cell.

---

## 1. `0x8066FA10` - 508 bytes - `work/ghidra/rel-d_basesNP/export/8066fa10_FUN_8066fa10.c`

**Body:** `0x8066FA10..0x8066FC08` (127 words, `blr` at `0x8066FC08`).
Ledger: absent. `src/game/bases.rs`: absent as a definition; a previous lane
left a `// partial:` note at `src/game/bases.rs:156231` calling this "the
GQR-rejected four-player sort pass", which this section resolves - the eight
`psq*` words are four `psq_st` + four `psq_l` spills of `f31/f30/f29/f28` only.

```rust
pub unsafe fn fn_8066fa10(this: usize)
```

**Frame.** `stwu r1,-128(r1)`; `mfspr r0,LR` / `stw r0,132(r1)`;
`addi r11,r1,64`; four save pairs `stfd f31,112` + `psq_st f31,120`,
`f30,96`/`104`, `f29,80`/`88`, `f28,64`/`72`;
`bl 0x802DCD68` at `0x8066FA40` (the ladder entry for **r24-r31**);
`or r30,r3,r3` (**r30 = this**).
Epilogue `0x8066FBD4..0x8066FC08`: the four `psq_l`/`lfd` pairs,
`bl 0x802DCDB4` (restore r24-r31), `lwz r0,132(r1)`, `mtspr LR,r0`,
`addi r1,r1,128`, `blr`.

**Shape.** Rebuilds the per-player ordering table and then resamples a
`count`-entry spread of positions. Five phases.

1. `0x8066FA44-0x8066FA70` - pool: `r31 = [0x80429CA0]` (**count**),
   `r29 = 0x807C9028`, `r25 = r26 = this`, `r28 = 0` (slot), `r27 = 0`,
   `f31`/`f28` set in phase 4.
2. `0x8066FA74-0x8066FAA8` - slot fill, `do { } while (slot < 4)`:
   `p = 0x8005F900(slot)`;
   if `p != 0` then `p = 0x8005F900(slot)` **again** (second call, `0x8066FA88`)
   and `*(u32*)(r25 + 0x420) = p`, `r25 += 4`;
   else `*(u32*)(r26 + 0x420) = 0`.
   `slot += 1`; `r26 += 4` **unconditionally**, `r25 += 4` **only in the
   non-zero branch**.
   > This asymmetry is real and observable: `r25` compacts the live slots
   > downwards while `r26` writes the zeros at their *natural* index, so a zero
   > written at index `i` can be overwritten by a later live entry. Transcribe
   > both pointers literally; do not "tidy" it into one array.
3. `0x8066FAAC-0x8066FB08` - descending-index bubble sort over the four words
   at `this+0x41C..0x428`, ascending by `*(f32*)(entry + 0xAC)`:
   outer `i = 0..2` (`addi r6,r6,1` / `cmpi cr0,0,r6,3` / `blt`),
   `CTR = 3 - i` (`subfic r4,r6,4` / `addi r0,r4,-1` / `mtspr CTR,r0`),
   `r3 = this + 4` at the start of each pass;
   inner body `0x8066FAC8-0x8066FAF8`:
   `a = *(u32*)(r3+0x420)`, `b = *(u32*)(r3+0x41C)`;
   if both non-zero and `*(f32*)(a+0xAC) < *(f32*)(b+0xAC)` then swap the two
   words; `r3 += 4`, `bdnz 0x8066FAC8` (`bc 16,lt`).
4. `0x8066FB0C-0x8066FB3C` - loop setup: `r27 = 0x807C9028`,
   `f31 = [0x807C902C]` (96.0), `f0 = [0x807C9064]` (0.5),
   `f28 = f31 * f0` (48.0), `f30 = *(f64*)0x807C9070` (2^52 + 2^31),
   `f29 = [0x807C9068]` (8.0), `r5 = 0x807C9030` (2.0),
   `r25 = this + 0x400`, `r26 = this`, `r24 = 0` (k).
5. `0x8066FB40-0x8066FBD0` - `do { } while (k < count)` (head `0x8066FB40`,
   test `cmp cr0,0,r24,r31` / `blt` at `0x8066FBCC`). For each k:
   * `f2 = *(f32*)(this+0x3FC) - 48.0` (`lfs f0,1020(r30)` / `fsubs f2,f0,f28`).
   * `f4 = 96.0 / (f32)(count+1)` (`fdivs f4,f31,f0` at `0x8066FB74`).
   * `t = (f32)(k+1)`; `g = 96.0 / (f32)count` (`fdivs f1,f31,f1`
     at `0x8066FB90`).
   * `*(f32*)(this+0x400+4k) = f4*t + f2` (`fmuls f3,f4,f3` /
     `fadds f0,f3,f2` / `stfs f0,1024(r26)` at `0x8066FBA0`).
   * `f0 = *(f32*)(*(u32*)(this+0x430) + 0xAC) + 8.0 - 96.0`
     (`lwz r6,1072(r30)` / `lfs f0,172(r6)` / `fadds f0,f29,f0` /
     `fsubs f0,f0,f31`).
   * `*(f32*)(this+0x410+4k) = g*t + f0` (`fadds f0,f1,f0` /
     `stfs f0,1040(r26)` at `0x8066FBB8`).
   * `bl 0x80052080(r3 = *(u32*)(this+0x420+4k), r4 = this+0x400+4k,
     r5 = 0x807C9030)` at `0x8066FBBC`.
   * `k += 1`, `r26 += 4`, `r25 += 4`.

The three `(f32)<unsigned>` conversions are the compiler's
`lis 0x4330` / `xoris 0x8000` / `lfd` / `fsubs` ladder against
`0x807C9070` = `0x4330000080000000`. Because `fsubs` performs the subtraction
at full double width and only rounds the *result* to single, `(2^52 + 2^31 + n)
- (2^52 + 2^31)` yields exactly `n`; do **not** pre-round the operands.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x8066FA40` | `0x802DCD68` | save r24-r31; `r11 = r1+64` |
| `0x8066FA78` | `0x8005F900` | `r3 = slot` -> player pointer or 0 |
| `0x8066FA88` | `0x8005F900` | `r3 = slot` (repeat, result stored) |
| `0x8066FBBC` | `0x80052080` | `r3 = *(u32*)(this+0x420+4k)`, `r4 = this+0x400+4k`, `r5 = 0x807C9030` |
| `0x8066FBF8` | `0x802DCDB4` | restore r24-r31; `r11 = r1+64` |

No virtual calls. Eight `psq*` words (four `psq_st` + four `psq_l`), all `qr0`
spills of `f31/f30/f29/f28`.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| player count word | `0x80429CA0` | `lwz r31,-25440(r4)` @`0x8066FA5C` |
| pool base | `0x807C9028` | `addi r29,r29,-28632` @`0x8066FA68` |
| `f32 96.0` | `0x807C902C` | `lfs f31,4(r27)` @`0x8066FB14` |
| call argument (2.0) | `0x807C9030` | `addi r5,r27,8` @`0x8066FB6C` |
| `f32 0.5` | `0x807C9064` | `lfs f0,60(r29)` @`0x8066FB1C` |
| `f32 8.0` | `0x807C9068` | `lfs f29,64(r29)` @`0x8066FB2C` |
| `f64 2^52+2^31` = `0x4330000080000000` | `0x807C9070` | `lfd f30,72(r29)` @`0x8066FB24` |

**Confidence: medium.** Every branch, store and callee is explicit. The two
items to preserve verbatim are the `r25`/`r26` compaction asymmetry of phase 2
and the fact that `96.0 / count` is evaluated even when `count == 0` (the loop
then runs zero times, so the `inf` is discarded). `0x80052080`'s contract
(`slot`, `&f32`, `2.0`) is not visible in this REL; add
`// unknown: 0x80052080(slot, scale, 2.0)`.

---

## 2. `0x806A4CA0` - 504 bytes - `work/ghidra/rel-d_basesNP/export/806a4ca0_FUN_806a4ca0.c`

**Body:** `0x806A4CA0..0x806A4E94` (126 words, `blr` at `0x806A4E94`).
Ledger: absent. `src/game/bases.rs`: absent as a definition (mentioned only at
`src/game/bases.rs:1652` in a module comment and `:152444` in a doc comment).

```rust
pub unsafe fn fn_806a4ca0(this: usize)
```

**Frame.** `stwu r1,-32(r1)`; `mfspr r0,LR` / `stw r0,36(r1)`;
`stfd f31,16(r1)` + `psq_st f31,24(r1),0,0`;
`stw r31,12` / `stw r30,8`;
`r31 = 0x807CA2D8` (`addis 0x807D` / `addi -23848`), `or r30,r3,r3`.

**Shape.** A per-frame tick for the "ground pound / stomp" actor. It is the
closest sibling of wave-128 §9 (`0x806A0290`) and wave-130 §9 (`0x806FDD50`):
one clamp, one conditional sound, two `0x8015F470` envelope pairs, and a fixed
virtual sequence. **Eleven virtual calls, all on `*(this+0x60)` except the
first.**

1. `0x806A4CC8-0x806A4CD8` - `vmethod0(this+0x544, 0x1C)`:
   `lwz r12,1348(r3)` / `lwz r12,28(r12)` / `addi r3,r3,1348` / `bctr`
   (receiver is `this+0x544`, not `this`).
2. `0x806A4CDC-0x806A4D04` - `if (0x8006EF40(this+0x1EC, 0) != 0 &&
   *(f32*)(this+0xEC) > [0x807CA390] (0.0))` then
   `*(f32*)(this+0xEC) = 0.0` (`ble` past the store at `0x806A4CFC`).
3. `0x806A4D04-0x806A4D68` - `if (0x80072BC0(this+0x1EC) != 0 &&
   *(f32*)(this+0xEC) < 0.0)`:
   `vmethod0(this, 0x348)` (`0x806A4D2C`);
   `*(u16*)(this+0x744) = 0` (`li r0,0` / `sth r0,1860(r30)`);
   `*(f32*)(this+0x740) *= [0x807CA3E8]` (0.8) (`fmuls f0,f1,f0` @`0x806A4D50`);
   `vmethod0(this, 0xD4)` with `r4 = 0x8082B984` (`0x806A4D60`).
4. `0x806A4D6C` - `bl 0x806A3BE0(this)`.
5. `0x806A4D74-0x806A4D98` - `vmethod0(this, 0x33C)` (`0x806A4D7C`) and
   `vmethod0(this, 0x340)` (`0x806A4D90`).
6. `0x806A4D9C-0x806A4DA8` - `bl 0x8006CD90(this)`, `bl 0x8006CDE0(this)`.
7. `0x806A4DAC-0x806A4DC4` - `if (*(f32*)(this+0xEC) > [0x807CA2F8] (3.0))
   *(f32*)(this+0xEC) = 3.0` (`fcmpo` / `ble` past, then `fmr f0,f1` /
   `stfs f0,236(r30)`).
8. `0x806A4DC8` - `bl 0x800955F0(this)`.
9. `0x806A4DD0-0x806A4E18` - two envelope pairs:
   `f31 = vmethod0_f(this, 0x350)` (`0x806A4DD8`), then
   `f1 = vmethod0_f(this, 0x34C)` (`0x806A4DF0`),
   `f2 = f31`, `bl 0x8015F470(..., r3 = this+0x114)`;
   then `f31 = vmethod0_f(this, 0x358)` (`0x806A4E10`),
   `f1 = vmethod0_f(this, 0x354)` (`0x806A4E28`), `f2 = f31`,
   `bl 0x8015F470(..., r3 = this+0x11C)`.
10. `0x806A4E40-0x806A4E50` - `vmethod0(this, 0x344)`.
11. `0x806A4E54-0x806A4E74` - `r3 = vmethod0(this, 0x374)`;
    `r4 = (s8)r3` (`or r0,r3,r3` / `extsb r4,r0`);
    `bl 0x800A4480(this, r4)`.
    Note this is **`extsb`**, not the `extsh` that wave 128 §9 uses on the same
    slot - the two bodies differ here.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x806A4CE4` | `0x8006EF40` | `r3 = this+0x1EC`, `r4 = 0` -> `u32` |
| `0x806A4D08` | `0x80072BC0` | `r3 = this+0x1EC` -> `u32` |
| `0x806A4D70` | `0x806A3BE0` | `r3 = this` |
| `0x806A4DA0` | `0x8006CD90` | `r3 = this` |
| `0x806A4DA8` | `0x8006CDE0` | `r3 = this` |
| `0x806A4DCC` | `0x800955F0` | `r3 = this` |
| `0x806A4E04` | `0x8015F470` | `f1 = (+0x34C result)`, `f2 = f31 (+0x350 result)`, `r3 = this+0x114` |
| `0x806A4E3C` | `0x8015F470` | `f1 = (+0x354 result)`, `f2 = f31 (+0x358 result)`, `r3 = this+0x11C` |
| `0x806A4E74` | `0x800A4480` | `r3 = this`, `r4 = (s8)(+0x374 result)` |

**Virtual slots:** `this+0x544` -> `+0x1C`; `*(this+0x60)` -> `+0x348`, `+0xD4`,
`+0x33C`, `+0x340`, `+0x350`, `+0x34C`, `+0x358`, `+0x354`, `+0x344`, `+0x374`.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| pool base | `0x807CA2D8` | `addi r31,r31,-23848` @`0x806A4CBC` |
| `f32 0.0` | `0x807CA390` | `lfs f0,184(r31)` @`0x806A4CF4` |
| `f32 0.8` | `0x807CA3E8` | `lfs f0,272(r31)` @`0x806A4D40` |
| `f32 3.0` | `0x807CA2F8` | `lfs f1,32(r3)` @`0x806A4DB4` |
| message cell | `0x8082B984` | `addi r4,r4,-18044` @`0x806A4D54` |

**Confidence: high.** Every store offset, every predicate and all eleven
virtual slots are literal, and the two `0x8015F470` pairs follow the same
`(f1 = later result, f2 = f31)` convention that wave 128 §9 and wave 130 §9
already established for the same call.

---

## 3. `0x807816B0` - 460 bytes - `work/ghidra/rel-d_basesNP/export/807816b0_FUN_807816b0.c`

**Body:** `0x807816B0..0x80781878` (115 words, `blr` at `0x80781878`).
Ledger: absent. `src/game/bases.rs`: absent as a definition; mentioned only in
doc comments at `:161880`, `:163066` and `:163076`, and reachable through the
already-translated caller `fn_807813f0`.

```rust
pub unsafe fn fn_807816b0(
    out: *mut f32,
    ctx: usize,
    from: *const f32,
    base: *const f32,
    index: i32,
    delta: f32,
    angle_rate: f32,
)
```

**Argument map** (verified from the body and from the single call site at
`0x80781648`): `r3 -> out` (`stfs` targets `0(r26)`), `r4 -> ctx`
(`lwz r8,656(r4)` = `*(ctx+0x290)`), `r5 -> from`, `r6 -> base`,
`r7 -> index`, `f1 -> delta` (`fmr f31,f1`), `f2 -> angle_rate` (used at
`0x80781724`). Ghidra's printed signature (`undefined8, double, undefined4,
undefined4, float*, float*, uint`) does **not** match the registers; the table
above is authoritative.

The repository already carries this ABI as
`call_f32x2_u32x5(addr, a: f32, b: f32, c..g: u32)` at
`src/game/bases.rs:161892`, with the doc note at
`src/game/bases.rs:161877-161890` recording exactly this call site
("`r7 = i`, no eighth argument"). Reuse that helper rather than introducing a
new `extern "C" fn` type.

**Frame.** `stwu r1,-160(r1)`; `mfspr r0,LR` / `stw r0,164(r1)`;
`addi r11,r1,144`; `stfd f31,144` + `psq_st f31,152`;
`bl 0x802DCD70` at `0x807816C8` (saves **r26-r31**);
`or r26,r3,r3`, `or r27,r4,r4`, `or r28,r5,r5`, `or r29,r6,r6`,
`or r30,r7,r7`.
Epilogue `0x8078185C..0x80781878` (`psq_l f31,152`, `lfd f31,144`,
`bl 0x802DCDBC`, ...).

**Shape.** A single-sample "spread along a line" evaluator: quantise a
per-sample angle, build a rotation, rotate the `from-base` delta, normalise it,
scale it and add `base`. Called once per sample from `0x80781648`.

1. `0x807816CC-0x807816F4` - `f5 = *(f64*)0x807CF798` (2^52 + 2^31);
   `r11 = 0x43300000` (the high word, reused three times);
   `r10 = index ^ 0x80000000`.
2. `0x8078170C-0x8078173C` - angle:
   `f4 = (f32)index` (`lfd f0,88(r1)` / `fsubs f4,f0,f5`);
   `f4 = angle_rate * f4` (`fmuls f4,f2,f4`);
   `f2 = (f32)(*(u32*)(ctx+0x290) - 1)` (`lwz r8,656(r4)` / `addi r0,r8,-1` /
   `xoris r0,r0,0x8000` / `lfd f3,96(r1)` / `fsubs f2,f3,f5`);
   `f8 = f4 / f2`;
   `f1 = [0x807CF7F8] (32767.0) * f8`;
   `f0 = [0x807CF7F0] (180.0)`; `f0 = f1 / f0`;
   `fctiwz f0,f0` / `stfd f0,104(r1)` / `lwz r0,108(r1)` /
   `sth r0,8(r1)` -> the angle is stored as a **16-bit** value at `r1+8`.
3. `0x80781728-0x80781780` - `d = { from[0]-base[0], from[1]-base[1],
   from[2]-base[2] }` at `r1+0x18/0x1C/0x20`
   (`stfs f1,24(r1)` @`0x80781780`, `stfs f4,28(r1)` @`0x8078176C`,
   `stfs f6,32(r1)` @`0x80781768`).
4. `0x80781788` - `bl 0x8016ED10(r3 = &r1+0x28, r4 = &r1+8)`.
   The 48-byte area at `r1+0x28` is **only ever written by this callee**; the
   body passes it on untouched.
5. `0x8078178C-0x80781798` - `bl 0x801C1160(r3 = &r1+0x28, r4 = &r1+0x18,
   r5 = &r1+0x18)` - same pointer in and out (`or r5,r4,r4`).
6. `0x8078179C-0x807817A0` - `bl 0x80170980(r3 = &r1+0x18)`.
7. `0x807817A4-0x807817DC` - rebuild the delta at `r1+0x0C/0x10/0x14` and
   `f1 = 0x801C14C0(&r1+0x0C)` (a scalar length).
8. `0x807817DC-0x80781838` - scale:
   `f8 = delta * (f32)index`; `f7 = (f32)(*(u32*)(ctx+0x290) - 1)`;
   `f7 = f8 / f7`; `f1 = f7 + f1` (the length from step 7);
   `f6 = *(f32*)(r1+0x18)`, `f5 = *(f32*)(r1+0x1C)`, `f4 = *(f32*)(r1+0x20)`;
   `f6 *= f1`, `f5 *= f1`, `f4 *= f1`.
9. `0x80781850-0x80781858` - `out[0] = f6 + base[0]`,
   `out[1] = f5 + base[1]`, `out[2] = f4 + base[2]`
   (`stfs f4,0(r26)`, `stfs f2,4(r26)`, `stfs f0,8(r26)`).

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x807816C8` | `0x802DCD70` | save r26-r31; `r11 = r1+144` |
| `0x80781788` | `0x8016ED10` | `r3 = &r1+0x28` (48-byte basis out), `r4 = &r1+8` (`s16` angle in) |
| `0x80781798` | `0x801C1160` | `r3 = &r1+0x28`, `r4 = &r1+0x18`, `r5 = &r1+0x18` |
| `0x807817A0` | `0x80170980` | `r3 = &r1+0x18` |
| `0x807817D8` | `0x801C14C0` | `r3 = &r1+0x0C` -> `f1` |
| `0x80781868` | `0x802DCDBC` | restore r26-r31; `r11 = r1+144` |

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f64 2^52+2^31` = `0x4330000080000000` | `0x807CF798` | `lfd f5,-2152(r31)` @`0x807816E8` |
| `f32 180.0` | `0x807CF7F0` | `lfs f0,-2064(r8)` @`0x80781718` |
| `f32 32767.0` | `0x807CF7F8` | `lfs f1,-2056(r9)` @`0x80781710` |

**Residual ambiguity -> conditional (medium).** Three things are inferred rather
than stated:

1. the 48-byte object at `r1+0x28` is produced entirely by `0x8016ED10` and
   consumed by `0x801C1160`; this REL never writes it, so its shape (`[f32; 12]`
   basis vs `[u8; 48]`) is a layout guess;
2. `0x801C1160` receives the *same* pointer as `r4` and `r5`, i.e. an in-place
   transform - correct per the registers, but it means the Rust must not
   introduce a second temporary;
3. the angle quantiser (`32767.0 * (angle_rate * index) / (count - 1) / 180.0`,
   truncated by `fctiwz` to 16 bits) is exact but semantically odd; keep it
   literal.

Everything else - all seven arguments, both `(f32)` ladders, the normalise and
the three output stores - is fully determined.

---

## 4. `0x80762CE0` - 440 bytes - `work/ghidra/rel-d_basesNP/export/80762ce0_FUN_80762ce0.c`

**Body:** `0x80762CE0..0x80762E94` (110 words, last word `0x4E800020`,
`blr`). Ledger: absent. `src/game/bases.rs`: absent (mentioned only in a
doc-comment at `src/game/bases.rs:140017`).

```rust
pub unsafe fn fn_80762ce0(this: usize)
```

**Frame.** `stwu r1,-80(r1)`; `mfspr r0,LR` / `stw r0,84(r1)`;
`stfd f31,64(r1)` + `psq_st f31,72(r1),0,0`; `stw r31,60(r1)`;
`or r31,r3,r3` at `0x80762D14` makes **r31 = this**.
Epilogue `0x80762E7C..0x80762E94`: `lwz r0,84(r1)`, `psq_l f31,72(r1)`,
`lfd f31,64(r1)`, `lwz r31,60(r1)`, `mtspr LR,r0`, `addi r1,r1,80`, `blr`.

**Shape.** A per-spawn placement initialiser. It reads a 24-byte-stride record
indexed by a byte out of `*(this+4)`, applies a position offset, picks a facing
pair from a four-way switch, converts that pair to a 16-bit angle and installs
it on the pane object at `this+0x224`.

1. `0x80762CE8-0x80762D0C` - pool setup: `r6 = 0x80815494`, `r4 = r5 =
   0x80377DEC`, `r7 = 0x807CEE40`; `f31 = [0x807CEE5C]` (0.0).
2. `0x80762D18-0x80762D54` - `idx = *(u32*)(this+4) & 0xFF`
   (`lwz r0,4(r3)` / `rlwinm r0,r0,0,24,31`);
   `rec = 0x80815494 + 24*idx` (`mulli r0,r0,24` / `add r6,r6,r0`);
   `*(f32*)(this+0xAC) = *(f32*)(this+0x260) + *(f32*)(rec+4)`;
   `*(f32*)(this+0xB0) = *(f32*)(this+0x264) + *(f32*)(rec+8)`;
   `*(f32*)(this+0xB4) = *(f32*)(this+0x268) + *(f32*)(rec+0xC)`.
   The three `fadds` are ordered `f4`(+0xB4), `f2`(+0xB0), `f0`(+0xAC) and the
   stores are `0x80762D4C` (+0xB4), `0x80762D50` (+0xAC), `0x80762D54` (+0xB0).
3. `0x80762D58-0x80762D80` - `face = *(s16*)(rec+26)` (`lha r0,26(r6)`);
   store the three `fadds` results again at `r1+0x20/0x24/0x28` (dead copies)
   and seed the facing pair `{f5, f3, f1}` = `{[0x80377DEC], [0x80377DF0],
   [0x80377DF4]}` at `r1+0x2C/0x30/0x34`
   (`stfs f5,44(r1)` @`0x80762D78`, `stfs f3,48(r1)` @`0x80762D7C`,
   `stfs f1,52(r1)` @`0x80762D80`).
4. `0x80762D84-0x80762E54` - four-way switch on `face` (`cmpi` 0/1/2/3 then
   `beq`; anything else falls to `0x80762E58`). Let `fp` be the facing pair at
   `r1+0x2C/0x30/0x34`:
   * `0` (`0x80762DA4`): `fp` re-stored unchanged; `f31 = [0x807CEE60]` (2.0).
   * `1` (`0x80762DB8`): `fp = {-f5, -f3, -f1}` (each `fneg` then `frsp`);
     `f31 = [0x807CEE64]` (1.0).
   * `2` (`0x80762DF0`): `fp = {[0x80377DD4], [0x80377DD8], [0x80377DDC]}`;
     `f31` keeps its prologue value 0.0.
   * `3` (`0x80762E14`): `fp = {-[0x80377DD4], -[0x80377DD8],
     -[0x80377DDC]}` (each `fneg` then `frsp`).
5. `0x80762E58-0x80762E70` - `f1 = fp.x` (`lfs 44(r1)`), `f2 = fp.z`
   (`lfs 52(r1)`); `bl 0x801611C0`; the returned `r3` (16-bit) is stored
   twice: `sth r3,258(r31)` = `this+0x102` and `sth r3,264(r31)` = `this+0x108`.
6. `0x80762E64-0x80762E78` - `f1 = f31`, `r4 = 0`, `r3 = this+0x224`;
   `bl 0x80167B20`.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80762E60` | `0x801611C0` | `f1 = out.x`, `f2 = out.z` -> `u16` in `r3` |
| `0x80762E78` | `0x80167B20` | `f1 = f31`, `r3 = this+0x224`, `r4 = 0` |

No virtual calls, no indirect dispatch, no `psq` other than the `f31` spill.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| record table base | `0x80815494` | `addi r6,r6,21652` @`0x80762CF8` |
| `f32 0.0` | `0x807CEE5C` | `lfs f31,28(r7)` @`0x80762D0C` |
| `f32 2.0` | `0x807CEE60` | `lfs f31,32(r7)` @`0x80762DA8` |
| `f32 1.0` | `0x807CEE64` | `lfs f31,36(r7)` @`0x80762DBC` |
| `f32` facing pair A | `0x80377DEC` | `lfs f5,32236(r5)` @`0x80762D68` |
| `f32` facing pair B | `0x80377DF0` | `lfs f3,4(r4)` @`0x80762D60` |
| `f32` facing pair C | `0x80377DF4` | `lfs f1,8(r4)` @`0x80762D5C` |
| `f32` quadrant pair A | `0x80377DD4` | `lfs f2,32212(r4)` @`0x80762DF8` |
| `f32` quadrant pair B | `0x80377DD8` | `lfs f1,4(r3)` @`0x80762DFC` |
| `f32` quadrant pair C | `0x80377DDC` | `lfs f0,8(r3)` @`0x80762E00` |

**Verified table contents** (`0x80815494 + 24*i`): the three `f32` fields at
`+4/+8/+0xC` are **0.0** for indices 0..5, and the `s16` at `+26` is
`{0, 3, 1, 3, 3, 3}`. Index 6 and above read other tables' bytes, so the array
has six live entries. The `+0x1A` displacement lies 2 bytes *past* a 24-byte
entry - i.e. the record is not a clean 24-byte struct and the `lha` overlaps the
next entry's `+2` field. This is reproduced literally by
`platform::read_i16(0x8081_5494 + 24*idx + 26)`.

**Residual ambiguity -> conditional (medium).** Two items:

1. The six floats at `0x80377DD4..0x80377DF4` are **not in this REL**. They fall
   inside `main.dol`'s zero-fill segment (`0x80351680..0x80427680`, `filesz = 0`
   in `work/dol/main.elf`), so they are runtime cells whose values the shipped
   artifacts do not carry. The Rust reads them as
   `platform::read_f32(0x8037_7DD4)` etc., which is faithful; what cannot be
   asserted is *what* they hold, and with `0.0` in all six the four switch arms
   would collapse to the same angle.
2. The record at `0x80815494` has the stride/field-overlap mismatch described
   above, so any typed `struct` for it would be a guess. Read it by address.

Everything else - every store, every callee, every branch - is explicit.

---

## 5. `0x806333D0` - 416 bytes - `work/ghidra/rel-d_basesNP/export/806333d0_FUN_806333d0.c`

**Body:** `0x806333D0..0x8063356C` (104 words, `blr` at `0x8063356C`).
Ledger: absent. `src/game/bases.rs`: absent as a definition (mentioned at
lines 640, 134639, 134668 as a callee).

```rust
pub unsafe fn fn_806333d0(this: usize)
```

**Frame.** `stwu r1,-128(r1)`; `mfspr r0,LR` / `stw r0,132(r1)`;
`addi r11,r1,112`; `stfd f31,112(r1)` + `psq_st f31,120(r1),0,0`;
`bl 0x802DCD70` at `0x806333E8` (saves **r26-r31**, returns with `r3` intact);
`or r28,r3,r3` at `0x806333F0` makes **r28 = this**.
Epilogue `0x80633550..0x8063356C`: `addi r11,r1,112`, `psq_l f31,120(r1)`,
`lfd f31,112(r1)`, `bl 0x802DCDBC`, `lwz r0,132(r1)`, `mtspr LR,r0`,
`addi r1,r1,128`, `blr`.

**Shape.** A four-slot matrix synchroniser. For each of four pane slots it
builds a 3x4 matrix describing a collision object's orientation plus position,
transforms it, and scatters the resulting 3-vector into four parallel arrays.
This is the same 3x4 pipeline as `0x80642540` (wave 128 §4), but written
through `0x800B32E0` in the *other* direction.

1. `0x806333EC-0x80633400` - `f31 = [0x807C7C9C]` (0.0); `r31 = this`
   (advances `+4` per iteration); `r29 = 0` (slot counter); `r30 = 0`
   (byte offset, advances `+0xC`).
2. Loop head `0x80633404`, back edge `0x8063354C blt 0x80633404`, exit test
   `0x806334C8 cmpi cr0,0,r29,4`. A `do { } while (i < 4)`:
   * `0x80633404-0x80633418`: `sel = *(u32*)(this + 4*i + 0x6A4)`;
     zero the output 3-vector at `r1+0x14/0x18/0x1C`;
     `cmpi cr0,0,sel,10` / `beq 0x806334BC` skips the whole rebuild when
     `sel == 10`.
   * `0x8063341C-0x80633430`: `e = *(u32*)(this + 4*sel + 0x5DC)`
     (`rlwinm r0,r0,2,0,29` / `add r5,r28,r0` / `lwz r26,1500(r5)`).
   * `0x80633430-0x8063349C`: assemble the **twelve** words of a 3x4 matrix
     `m[]` at `r1+0x14` in this exact order:
     `m[0] = *(f32*)(e+0x90)`, `m[1] = *(f32*)(e+0xA0)`, `m[2] = 0.0`,
     `m[3] = e+0x84`, `m[4] = e+0x88`, `m[5] = e+0x8C`,
     `m[6] = e+0x90`, `m[7] = e+0x94`, `m[8] = e+0x98`,
     `m[9] = e+0x9C`, `m[10] = e+0xA0`, `m[11] = e+0xA4`.
     (`m[0]`/`m[6]` and `m[1]`/`m[10]` are bit-identical; `m[0]`/`m[1]` are
     written as `f32` from `lfs 44(r1)`/`lfs 60(r1)`, the rest as raw `lwz`
     words.)
   * `0x806334A0`: `bl 0x800B32E0(r3 = &r1+8, r4 = &r1+0x14)`; on return read
     `f2 = *(f32*)(r1+8)`, `f1 = *(f32*)(r1+0xC)`, `f0 = *(f32*)(r1+0x10)` and
     store them to `r1+0x14/0x18/0x1C`.
   * `0x806334BC-0x80633548`: scatter `{r1+0x14, r1+0x18, r1+0x1C}` to
     `*(u32*)(this+0x74) + r30 + 0x2A4/0x2A8/0x2AC`,
     `*(u32*)(this+0x78) + r30 + 0x2A4/0x2A8/0x2AC`,
     `*(u32*)(this+0x7C) + r30 + 0x2A4/0x2A8/0x2AC`,
     `*(u32*)(this+0x80) + r30 + 0x2A4/0x2A8/0x2AC`.
   * `0x806334C0/0x806334D0/0x806334E8`: `i += 1`, `r31 += 4`, `r30 += 0xC`.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x806333E8` | `0x802DCD70` | save r26-r31; `r11 = r1+112` |
| `0x806334A0` | `0x800B32E0` | `r3 = &r1+8` (3-float out), `r4 = &r1+0x14` (12-word 3x4 in) |
| `0x8063355C` | `0x802DCDBC` | restore r26-r31; `r11 = r1+112` |

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 0.0` | `0x807C7C9C` | `lfs f31,31900(r4)` @`0x806333F4` |

**Residual ambiguity -> conditional (medium).** The 3x4 at `r1+0x14` is *not* a
straight copy of `e+0x84..0xB0`: the compiler interleaves the two position
fields (`e+0x90`, `e+0xA0`) into `m[0]`/`m[1]` and repeats them at `m[6]`/
`m[10]`, and forces `m[2] = 0.0`. The transcription is exact - twelve words at
twelve stack slots, all listed above - but which row/column of the 3x4 is which
is not recoverable from this REL, and `0x800B32E0`'s parameter convention
(`r3` out, `r4` matrix, 12 words in / 3 floats out, confirmed by
`docs/SPEC_WAVE128_BASES.md` §4) does not identify the semantic roles. Emit the
matrix with a `// unknown: 3x4 layout` note. Nothing else is ambiguous.

---

## 6. `0x8069AB00` - 408 bytes - `work/ghidra/rel-d_basesNP/export/8069ab00_FUN_8069ab00.c`

**Body:** `0x8069AB00..0x8069AC94` (102 words, `blr` at `0x8069AC94`).
Ledger: absent. `src/game/bases.rs`: absent.

```rust
pub unsafe fn fn_8069ab00(this: usize, velocity: *const f32)
```

**Frame.** `stwu r1,-48(r1)`; `mfspr r0,LR` / `stw r0,52(r1)`;
`stfd f31,32(r1)` + `psq_st f31,40(r1),0,0`;
`stw r31,28` / `stw r30,24` / `stw r29,20`;
`or r30,r4,r4` at `0x8069AB1C` (**r30 = velocity**),
`or r29,r3,r3` at `0x8069AB24` (**r29 = this**).
Epilogue `0x8069AC74..0x8069AC94`.

**Shape.** Converts a 2D launch velocity into a per-axis rate, derives the
speed, and raises two one-shot events. Six virtual calls over five distinct
slots, four of them reached through `*(this+0x60)` (an interface pointer whose
receiver is `this`) and two through the object returned by slot `+0x2DC`.

1. `0x8069AB28-0x8069AB34` - `speed = vmethod0_f(this, 0x35C)`, i.e.
   `*(*(this+0x60) + 0x35C)` called with `r3 = this`; returns `f1`.
2. `0x8069AB38-0x8069AB44` - `*(f32*)(this+0xE8) = velocity[0] / speed`.
3. `0x8069AB48-0x8069AB64` - repeat the same call with the same arguments;
   `*(f32*)(this+0xEC) = velocity[1] / speed2`.
4. `0x8069AB68-0x8069AB74` - `f1 = velocity[0]² ; f0 = velocity[1]² ;
   f1 = f1 + f0 ; bl 0x802BE470` -> `f1 = speed magnitude`; saved in `f31`.
5. `0x8069AB78-0x8069ABD0` - `obj = vmethod0(this, 0x2DC)`
   (`lwz r12,96(r29)` / `lwz r12,732(r12)` / `bctr`, `r3 = this`);
   `p = *(u32*)(obj+0x5F8)`; `b = (vmethod0(p, 0x0C) != 0)`;
   if `!b` then `b = (vmethod0(p, 0x10) != 0)`;
   the two calls are `lwz r3,1528(r3)` then `lwz r12,0(r3)` with slots
   `+0x0C` (`0x8069ABA0`) and `+0x10` (`0x8069ABBC`), receiver `p`.
6. `0x8069ABD4-0x8069ABEC` - if `b`: `vmethod0(this, 0x37C)`
   (`lwz r12,892(r12)`, `r3 = this`).
7. `0x8069ABF0` - `bl 0x800A4740(this)`.
8. `0x8069AC00-0x8069AC30` - if `*(f32*)(this+0xE8) > [0x807CA0C4]` (5.0):
   `bl 0x8006A4B0(r3 = &r1+8, r4 = this+0xAC)` then
   `bl 0x80197F00(r3 = [0x80429D5C], r4 = 0x4EA, r5 = &r1+8, r6 = 0)`.
   The comparison is `ble` past the block, i.e. *strictly greater*.
9. `0x8069AC34-0x8069AC68` - if `f31 > [0x807CA0EC]` (2.5):
   `*(u32*)(this+0xDBC) = [0x807F1404]` (0x3C);
   `*(f32*)(this+0xDC0) = *(f32*)(this+0xE8)`;
   `*(f32*)(this+0xDC4) = *(f32*)(this+0xEC)`;
   `*(f32*)(this+0xDC8) = *(f32*)(this+0xF0)`.
10. `0x8069AC6C-0x8069AC70` - `*(u8*)(this+0xD28) = 1` (`li r0,1` / `stb`).

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x8069AB74` | `0x802BE470` | `f1 = vx² + vy²` -> `f1 = sqrt` |
| `0x8069ABF4` | `0x800A4740` | `r3 = this` |
| `0x8069AC1C` | `0x8006A4B0` | `r3 = &r1+8`, `r4 = this+0xAC` |
| `0x8069AC30` | `0x80197F00` | `r3 = [0x80429D5C]`, `r4 = 0x4EA`, `r5 = &r1+8`, `r6 = 0` |

**Virtual slots:** `*(this+0x60)+0x35C` (twice, `r3 = this`),
`*(this+0x60)+0x2DC` (`r3 = this`), `*(*(this+0x60)+0x2DC)->+0x5F8` slots
`+0x0C` and `+0x10` (`r3 = *(obj+0x5F8)`), `*(this+0x60)+0x37C` (`r3 = this`).

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 5.0` | `0x807CA0C4` | `lfs f0,-24380(r3)` @`0x8069AC00` |
| `f32 2.5` | `0x807CA0EC` | `lfs f0,-24340(r3)` @`0x8069AC38` |
| `u32 0x3C` (60) | `0x807F1404` | `lwz r0,196(r3)` @`0x8069AC54`, base `0x807F1340` |
| line-id singleton | `0x80429D5C` | `lwz r31,-25252(r4)` @`0x8069AC14` |

**Confidence: high.** Both `cmpi`/`fcmpo` polarities are the plain `ble` form,
the two `sth`-style stores are absent (all stores are `stw`/`stfs`/`stb` with
literal offsets), and the only external inputs are `[0x80429D5C]` and the
vtable contents of `*(this+0x60)`. `0x807F1340` is in this REL's `.data`
(segment `0x807D0C00..0x808228B8`) so `0x807F1404` is byte-verifiable: it holds
`0x0000003C`.

---

## 7. `0x807616E0` - 408 bytes - `work/ghidra/rel-d_basesNP/export/807616e0_FUN_807616e0.c`

**Body:** `0x807616E0..0x80761874` (102 words, `blr` at `0x80761874`).
Ledger: absent. `src/game/bases.rs`: no definition (a doc-comment reference at
line 139265).

```rust
pub unsafe fn fn_807616e0(this: usize, mirrored: u32)
```

**Frame.** `stwu r1,-48(r1)`; `mfspr r0,LR` / `stw r0,52(r1)`;
`stfd f31,32(r1)` + `psq_st f31,40(r1),0,0`; `stw r31,28` / `stw r30,24`;
`r31 = 0x807CEDB0` (`addis 0x807D` / `addi -4688`), `or r30,r3,r3`.

**Shape.** A two-branch door-driver: pick a start rate, then dispatch one of
three animation starts depending on which of two "is the door being pushed"
probes succeeds.

1. `0x807616E8-0x80761724` - `f31 = (mirrored != 0) ? [0x807CEDD0] (0.0)
   : [0x807CEDB4] (1.0)`. The `beq` at `0x80761714` selects the literal
   `lfs f31,4(r31)` = 1.0 when `mirrored == 0`.
2. `0x80761728-0x8076173C` - gate:
   `(*(u32*)(this+4) & 0xFF) == 3` (`lwz r0,4(r3)` / `rlwinm r0,r0,0,24,31` /
   `cmpi cr0,0,r0,3`) **and** `*(u8*)0x8042A24D == 2`
   (`lbz r0,-23987(r3)` / `cmpli cr0,0,r0,0x2`). Otherwise jump straight to
   the epilogue.
3. `0x80761740-0x80761750` - `if (*(u8*)([0x80429F7C] + 0x380) == 0)`.
4. `0x80761754-0x80761778` (door closed) - `if (0x800FD980(this) != 0)`:
   `bl 0x80761260(f1 = [0x807CEDD0] (0.0), f2 = [0x807CEDD4] (1.0),
   r3 = this, r4 = 0)`.
5. `0x8076177C-0x807617A0` - `else if (0x800FD9A0(this) != 0)`:
   `bl 0x80761260(f1 = [0x807CEDD4] (1.0), f2 = [0x807CEDD0] (0.0),
   r3 = this, r4 = 1)`.
6. `0x807617A4-0x807617D0` - else: `bl 0x80761260(f1 = f31,
   f2 = (f32)(2^52 + 2^31 - 2^52), r3 = this, r4 = 2)`. `r4 = 2` is
   `addi r4,r0,2` at `0x807617C4`.
   The `f2` operand is the compiler's unsigned-widening ladder in its
   "low word forced to `0x80000000`" form: `lis r0,0x4330` (`0x807617A8`),
   `xoris r3,r3,0x8000` on `r3 = 0` (`0x807617AC`), `stw` to `12(r1)`,
   `lfd f0,8(r1)`, `lfd f2,40(r31)`, `fsubs f2,f0,f2`. With `r3 = 0` the
   constructed double is `0x4330000080000000` (2^52 + 2^31) and the loaded
   constant at `0x807CEDD8` is the double `0x4330000000000000` (2^52), so the
   result is exactly **2^31 = 2147483648.0**.
7. `0x807617D8-0x80761854` - the identical three-way dispatch when
   `*(u8*)([0x80429F7C]+0x380) != 0`, with the probes swapped:
   `0x800FD960(this)` (`0x807617DC`) and `0x800FD940(this)` (`0x80761804`),
   and the same three `0x80761260` calls with the same three slots
   (`0x807617F8` slot 0, `0x80761820` slot 1, `0x80761854` slot 2).

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80761758` | `0x800FD980` | `r3 = this` -> `u32` |
| `0x80761774` | `0x80761260` | `f1 = 0.0`, `f2 = 1.0`, `r3 = this`, `r4 = 0` |
| `0x80761780` | `0x800FD9A0` | `r3 = this` -> `u32` |
| `0x8076179C` | `0x80761260` | `f1 = 1.0`, `f2 = 0.0`, `r3 = this`, `r4 = 1` |
| `0x807617D0` | `0x80761260` | `f1 = f31`, `f2 = 2^31`, `r3 = this`, `r4 = 2` |
| `0x807617DC` | `0x800FD960` | `r3 = this` -> `u32` |
| `0x807617F8` | `0x80761260` | `f1 = 0.0`, `f2 = 1.0`, `r3 = this`, `r4 = 0` |
| `0x80761804` | `0x800FD940` | `r3 = this` -> `u32` |
| `0x80761820` | `0x80761260` | `f1 = 1.0`, `f2 = 0.0`, `r3 = this`, `r4 = 1` |
| `0x80761854` | `0x80761260` | `f1 = f31`, `f2 = 2^31`, `r3 = this`, `r4 = 2` |

`0x80761260` is already translated (`src/game/bases.rs:139393`,
`fn_80761260(rate: f32, start: f32, door: usize, slot: i32)`), so the argument
order `f1 = rate`, `f2 = start`, `r3 = door`, `r4 = slot` is fixed here.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 1.0` | `0x807CEDB4` | `lfs f31,4(r3)` @`0x80761724` |
| `f32 0.0` | `0x807CEDD0` | `lfs f31,32(r31)` @`0x80761718` |
| `f32 1.0` | `0x807CEDD4` | `lfs f2,36(r31)` @`0x8076176C` |
| `f64 2^52` = `0x4330000000000000` | `0x807CEDD8` | `lfd f2,40(r31)` @`0x807617B4` |
| door byte | `0x8042A24D` | `lbz r0,-23987(r3)` @`0x80761734` |
| door manager | `0x80429F7C` | `lwz r3,-24708(r3)` @`0x80761744` |

**Confidence: medium.** Every operand and every branch is explicit; the one
thing to get right is item 6 - the ladder yields `2^31`, not `0.0` and not
`-2^31`, because the subtraction constant is the *unsigned* `2^52` while the
value word was XOR-ed with `0x80000000`. Write it as the literal
`2147483648.0f32` (or as the explicit `2^52 + 2^31 - 2^52` computation) rather
than as a cast.

---

## 8. `0x80656310` - 396 bytes - `work/ghidra/rel-d_basesNP/export/80656310_FUN_80656310.c`

**Body:** `0x80656310..0x80656498` (99 words, `blr` at `0x80656498`).
Ledger: absent. `src/game/bases.rs`: absent.

```rust
pub unsafe fn fn_80656310(this: usize, kind: u32)
```

**Frame.** `stwu r1,-96(r1)`; `mfspr r0,LR` / `stw r0,100(r1)`;
`stfd f31,80(r1)` + `psq_st f31,88(r1),0,0`;
`stw r31,76` / `stw r30,72` / `stw r29,68` / `stw r28,64`;
`r31 = 0x807C88F8`, `r30 = 0`, `or r29,r4,r4` (**r29 = kind**),
`or r28,r3,r3` (**r28 = this**).

**Shape.** Sample a 3-float sample out of a table (or fall back to a helper),
convert one inter-sample delta into a time offset, and issue one of two sound
calls.

1. `0x8065634C-0x80656358` - `sel = *(u32*)(this+0x3D8) & 0xFF`
   (`lwz r0,984(r3)` / `rlwinm. r6,r0,0,24,31`).
2. `0x80656350-0x806563A0` - resolve the sample record:
   * `sel == 0` -> `flag = true` (`b 0x806563A0`, `li r30,1` at `0x806563A0`).
   * else `rec = [0x80429E98] + (u32)*(u8*)([0x8042A1C8] + 0x120E) * 0x3B0`
     (`lbz r0,4622(r5)` / `mulli r0,r0,944` / `add r3,r3,r0`); if
     `*(u32*)(rec+0x2C) == 0` then `rec = 0` (the two-instruction
     `cmpi`/`beq`/`b` pair at `0x80656374-0x80656380`), and
     `0x8008E890(rec, sel, &r1+8)` is called at `0x8065638C`
     (`r4` = `sel` via `or r4,r6,r6`, `r5 = &r1+8`).
     If it returns 0, `flag = true` at `0x80656398`.
3. `0x806563A4-0x806563B8` - if `flag`: `f1 = 0x80656260(this)` and
   `*(f32*)(r1+0x1C) = f1`; jump to `0x8065641C`.
4. `0x806563BC-0x80656418` - otherwise the buffer written by
   `0x8008E890` at `&r1+8` is read:
   `v = *(f32*)(r1+0x0C)`, `w = *(f32*)(r1+0x14)`,
   `n = (i32)(|((v - w) - v)| * [0x807C8900] (0.0625))` via `fabs`, `frsp`,
   `fmuls`, `fctiwz`, `stfd`/`lwz 44(r1)`;
   `i = 0x806561B0(this, n)` (`0x806563EC`);
   `*(f32*)(r1+0x1C) = v - 16.0 * (f32)(i32)i`
   where `16.0` is `[0x807C88FC]`, the `(i32)` widening is the
   `xoris 0x8000` / `lis 0x4330` / `lfd 48(r1)` / `fsubs` ladder against the
   double `[0x807C8908]` = `0x4330000080000000` (2^52 + 2^31), and the final
   subtraction is `fsubs f0,f31,f0` at `0x80656414`.
5. `0x8065641C-0x80656430` - `f1 = 0x80656210(this, kind)`; store it at
   `r1+0x18`; store `[0x807C8910]` (0.0) at `r1+0x20`. The 3-float argument
   vector is therefore `{r1+0x18, r1+0x1C, r1+0x20}` = `{f, delta, 0.0}`.
6. `0x80656434-0x80656474` - `if (*(u32*)(this+0x3DC) == 0)`:
   `bl 0x80064610(r3 = 0x202, r4 = 0x1000, r5 = &r1+0x18, r6 = 0,
   r7 = *(u8*)(this+0x38F))`; else
   `bl 0x80064610(r3 = 0x191, r4 = 0x200000, r5 = &r1+0x18, r6 = 0,
   r7 = *(u8*)(this+0x38F))`.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x8065638C` | `0x8008E890` | `r3 = rec` (or 0), `r4 = sel`, `r5 = &r1+8` |
| `0x806563B0` | `0x80656260` | `r3 = this` -> `f1` |
| `0x806563EC` | `0x806561B0` | `r3 = this`, `r4 = (i32)n` -> `u32` |
| `0x80656424` | `0x80656210` | `r3 = this`, `r4 = kind` -> `f1` |
| `0x80656454` | `0x80064610` | `r3 = 0x202`, `r4 = 0x1000`, `r5 = &r1+0x18`, `r6 = 0`, `r7 = *(u8*)(this+0x38F)` |
| `0x80656470` | `0x80064610` | `r3 = 0x191`, `r4 = 0x200000`, `r5 = &r1+0x18`, `r6 = 0`, `r7 = *(u8*)(this+0x38F)` |

**Constants:**

| value | image address | materialised at |
|---|---|---|
| pool base | `0x807C88F8` | `addi r31,r31,-30472` @`0x80656330` |
| `f32 0.0625` | `0x807C8900` | `lfs f0,8(r31)` @`0x806563C8` |
| `f32 16.0` | `0x807C88FC` | `lfs f0,4(r31)` @`0x80656404` |
| `f64 2^52+2^31` = `0x4330000080000000` | `0x807C8908` | `lfd f2,16(r31)` @`0x806563FC` |
| `f32 0.0` | `0x807C8910` | `lfs f0,24(r31)` @`0x80656428` |
| player-actor table | `0x80429E98` | `lwz r3,-24936(r3)` @`0x80656364` |
| manager | `0x8042A1C8` | `lwz r5,-24120(r5)` @`0x80656350` |

**Confidence: medium.** All control flow and every store is explicit. The one
soft spot is the *layout* of the buffer `0x8008E890` fills at `&r1+8`: only
`+4` (`r1+0x0C`) and `+0xC` (`r1+0x14`) are read, and the record stride
`0x3B0` / `+0x2C` null-check is reproduced literally. Add
`// unknown: 0x8008E890 sample layout` and keep the offsets literal. Note that
`rlwinm. r6,r0,0,24,31` at `0x80656354` also sets `CR0`, which the following
`beq` consumes - do not drop the mask.

---

## 9. `0x80656D20` - 392 bytes - `work/ghidra/rel-d_basesNP/export/80656d20_FUN_80656d20.c`

**Body:** `0x80656D20..0x80656EA4` (98 words, `blr` at `0x80656EA4`).
Ledger: absent. `src/game/bases.rs`: absent. No `psq` at all.

```rust
pub unsafe fn fn_80656d20(this: usize, event: usize)
```

**Frame.** `stwu r1,-64(r1)`; `mfspr r0,LR` / `stw r0,68(r1)`;
`stw r31,60` / `stw r30,56` / `stw r29,52`; `r31 = 0x807C8920`
(`addis 0x807D` / `addi -30432`). No frame pointer.

**Shape.** An event handler for event id `0x12A` that pushes two actors apart
along the line joining them.

1. `0x80656D40-0x80656D4C` - `other = *(u32*)(event+4)`;
   `if (*(u16*)(other+8) != 0x12A)` -> `0x80656E88`:
   `bl 0x80095950()` and return.
2. `0x80656D50-0x80656D5C` - `actor = *(u32*)(this+4)`.
   `if (*(u8*)(actor+0x5F9) == 0)` -> `bl 0x80657390(actor)` and return.
3. `0x80656D6C-0x80656D9C` - `d = vmethod0(actor+0x394, 0x28)`
   (`lwz r12,916(r30)` / `addi r3,r30,916` / `lwz r12,40(r12)` / `bctr`);
   `if (vmethod0(d, 0x18, r4 = 0x808289A0) == 0)` -> return.
   The second call is `lwz r12,0(r3)` / `lwz r12,24(r12)` / `bctr` with
   `r4 = 0x808289A0` loaded at `0x80656D84`+`0x80656D88`; `r3` is `d`.
4. `0x80656DA0-0x80656DC4` - `v2 = { *(f32*)(actor+0xAC) - *(f32*)(other+0xAC),
   *(f32*)(actor+0xB0) - *(f32*)(other+0xB0) }` at `r1+0x18/0x1C`;
   `bl 0x802BE640(&r1+0x18)` normalises it in place.
5. `0x80656DC8-0x80656DD4` - `0x801C14C0(other+0xE8)` is called twice
   (`0x80656DCC` result discarded, `0x80656DD4` result in `f1`).
6. `0x80656DD8-0x80656DF8` - clamp `s` into `[2.0, 6.0]`
   (`fcmpo`/`bge` against `[0x807C8978]`, then `fcmpo`/`ble` against
   `[0x807C897C]`, with `fmr` fixes at `0x80656DE4` and `0x80656DF8`).
7. `0x80656DFC-0x80656E4C` - with `dx = *(f32*)(r1+0x18)`, `dy = *(f32*)(r1+0x1C)`
   and `s` from step 6:
   `k = -s` (`fneg`);
   `*(f32*)(r1+0x20) = (dx*s) * 1.5`;
   `*(f32*)(r1+0x24) = (dy*s) * 0.5`;
   `*(f32*)(r1+0x28) = (dx*-s) * 1.5`;
   `*(f32*)(r1+0x2C) = (dy*-s) * 0.5`;
   where `1.5` is `[0x807C8968]` and `0.5` is `[0x807C896C]`.
   The four stores land at `0x80656E38`, `0x80656E44`, `0x80656E48`,
   `0x80656E4C`.
8. `0x80656E50-0x80656E68` - `vmethod0(other, 0x2F0)` with
   `r3 = other`, `r4 = &r1+0x28` (a **2-float** argument: `{dx*-s*1.5,
   dy*-s*0.5}`); slot read as `lwz r12,752(r12)` on `*(other+0x60)`.
9. `0x80656E6C-0x80656E84` - `vmethod0(actor, 0x28C)` with
   `r3 = actor`, `r4 = &r1+0x20` (a **4-float** argument:
   `{dx*s*1.5, dy*s*0.5, dx*-s*1.5, dy*-s*0.5}`); slot read as
   `lwz r12,652(r12)` on `*(actor+0x60)`.

Stores at `r1+8`, `r1+0xC`, `r1+0x10`, `r1+0x14` (`0x80656E5C`, `0x80656E60`,
`0x80656E28`, `0x80656E54`) are **dead** - no instruction reads them. Keep
them or drop them; they do not affect behaviour.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80656D64` | `0x80657390` | `r3 = actor` |
| `0x80656DC4` | `0x802BE640` | `r3 = &r1+0x18` (2-float vector, normalised in place) |
| `0x80656DCC` | `0x801C14C0` | `r3 = other+0xE8` -> `f1` (discarded) |
| `0x80656DD4` | `0x801C14C0` | `r3 = other+0xE8` -> `f1` |
| `0x80656E88` | `0x80095950` | no arguments |

**Virtual slots:** `*(actor+0x394)+0x28` (`r3 = actor+0x394`);
`d+0x18` (`r3 = d`, `r4 = 0x808289A0`); `*(other+0x60)+0x2F0`
(`r3 = other`, `r4 = &r1+0x28`); `*(actor+0x60)+0x28C`
(`r3 = actor`, `r4 = &r1+0x20`).

**Constants:**

| value | image address | materialised at |
|---|---|---|
| pool base | `0x807C8920` | `addi r31,r31,-30432` @`0x80656D34` |
| `f32 1.5` | `0x807C8968` | `lfs f6,72(r3)` @`0x80656E10` |
| `f32 0.5` | `0x807C896C` | `lfs f7,76(r3)` @`0x80656E18` |
| `f32 2.0` (clamp low) | `0x807C8978` | `lfs f0,88(r31)` @`0x80656DD8` |
| `f32 6.0` (clamp high) | `0x807C897C` | `lfs f0,92(r31)` @`0x80656DEC` |
| request key | `0x808289A0` | `addi r4,r4,-30304` @`0x80656D88` |

**Confidence: medium.** Control flow, clamps and every store offset are
explicit. The residual doubt is the *arity* split: the collider call receives
only the two "negative" lanes while the actor call receives all four. That is
what the registers say (`r4 = &r1+0x28` vs `r4 = &r1+0x20`), but the callee
signatures are not visible in this REL, so a typed `Vec2`/`Vec4` choice would be
a guess. Emit raw pointers plus a `// unknown:` note on `0x807C8920`'s users.

---

## 10. `0x806A75B0` - 392 bytes - `work/ghidra/rel-d_basesNP/export/806a75b0_FUN_806a75b0.c`

**Body:** `0x806A75B0..0x806A7734` (98 words, `blr` at `0x806A7734`).
Ledger: absent. `src/game/bases.rs`: absent.

```rust
pub unsafe fn fn_806a75b0(this: usize)
```

**Frame.** `stwu r1,-80(r1)`; `mfspr r0,LR` / `stw r0,84(r1)`;
`addi r11,r1,48`; `stfd f31,64(r1)` + `psq_st f31,72(r1),0,0`;
`stfd f30,48(r1)` + `psq_st f30,56(r1),0,0`;
`bl 0x802DCD70` at `0x806A75D0` (saves **r26-r31**).
Epilogue `0x806A7710..0x806A7734`: `addi r11,r1,48`, `psq_l f31,72`,
`lfd f31,64`, `psq_l f30,56`, `lfd f30,48`, `bl 0x802DCDBC`,
`lwz r0,84(r1)`, `mtspr LR,r0`, `addi r1,r1,80`, `blr`.

**Shape.** A two-state lift/oscillation driver with a shared fall-out tail.

1. `0x806A75D4-0x806A75EC` - `switch (*(u32*)(this+0x360))`:
   `1` -> `0x806A75F0`, `2` -> `0x806A76B4`, anything else -> `0x806A76E0`.
2. **State 1** (`0x806A75F0-0x806A76B0`):
   * pool: `r4 = 0x807CA448`, `r5 = 0x807CA468`, `f30 = [0x807CA468]`
     (double 2^52 + 2^31), `f31 = [0x807CA44C]` (2.0), `r29 = 0x8042C1A0`,
     `r27 = this+0x6E0`, `r26 = 0`, `r30 = 0x43300000`.
   * `do { ... } while (i < 2)` - head `0x806A761C`, back edge
     `0x806A7670 blt 0x806A761C`:
     `x = (i32)*(s8*)(0x8042C1A0 + i)` (`lbz r0,0(r29)` + `extsh r0,r0`);
     `f1 = (f32)((double)(0x43300000 | (x ^ 0x80000000)) - f30)` = `(f32)x`;
     `*(f32*)(this + 0x6E0 + 12*i) += f1 * 2.0`
     (`lfs f0,1760(r28)` / `fadds f0,f0,f1` / `stfs f0,1760(r28)`, with
     `r28` advancing `+0xC`);
     `bl 0x806A6A70(this)`; `bl 0x806A6AA0(this, this + 0x6E0 + 12*i)`.
   * `0x806A7674-0x806A76B0`: if `*(f32*)(this+0xAC) + 25.0 * 0.5 <
     *(f32*)(this+0x6E0)` (`fcmpo` at `0x806A7694`, `bge` past the block):
     `*(u32*)(this+0x360) += 1` (`0x806A76A4`/`0x806A76A8`) and
     `*(u32*)(this+0x6D8) = (u32)*(u16*)(this+0x714)`
     (`lhz r0,1812(r31)` / `stw r0,1752(r31)`).
   * falls through to `0x806A76E0`.
3. **State 2** (`0x806A76B4-0x806A76DC`):
   * `bl 0x806A6C50(this)`.
   * `if ((*(u32*)(this+0x6D8) & 0xFFFF) == 0)` (`rlwinm. r0,r0,0,16,31` at
     `0x806A76BC`) -> `0x806A76D4`.
   * else `if (0x806A70C0(this) == 0)` (`0x806A76C8`) -> `0x806A76E0`;
     otherwise -> `0x806A76D4`.
   * `0x806A76D4`: `bl 0x80162510(this)` and jump to the epilogue
     (`b 0x806A7710`), skipping the message.
4. **Tail** (`0x806A76E0-0x806A770C`):
   `bl 0x8006A4B0(r3 = &r1+8, r4 = this+0xAC)`;
   `bl 0x80199390(r3 = [0x80429D60], r4 = 0x5B2, r5 = *(u32*)(this),
   r6 = &r1+8, r7 = 0)`.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x806A75D0` | `0x802DCD70` | save r26-r31; `r11 = r1+48` |
| `0x806A764C` | `0x806A6A70` | `r3 = this` |
| `0x806A7658` | `0x806A6AA0` | `r3 = this`, `r4 = this+0x6E0+12*i` |
| `0x806A76B4` | `0x806A6C50` | `r3 = this` |
| `0x806A76C8` | `0x806A70C0` | `r3 = this` -> `u32` |
| `0x806A76D8` | `0x80162510` | `r3 = this` |
| `0x806A76F4` | `0x8006A4B0` | `r3 = &r1+8`, `r4 = this+0xAC` |
| `0x806A770C` | `0x80199390` | `r3 = [0x80429D60]`, `r4 = 0x5B2`, `r5 = *(this)`, `r6 = &r1+8`, `r7 = 0` |
| `0x806A7724` | `0x802DCDBC` | restore r26-r31; `r11 = r1+48` |

`0x806A6A70` (12 bytes) and `0x806A6AA0` (32 bytes) are still-untranslated
`halt_baddata` stubs (wave 127 class D). They are called by address only, so
this body does not depend on them being translated.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 25.0` | `0x807CA448` | `lfs f2,-23480(r4)` @`0x806A767C` |
| `f32 2.0` | `0x807CA44C` | `lfs f31,4(r4)` @`0x806A7608` |
| `f32 0.5` | `0x807CA460` | `lfs f0,-23456(r3)` @`0x806A7680` |
| `f64 2^52+2^31` = `0x4330000080000000` | `0x807CA468` | `lfd f30,-23448(r5)` @`0x806A7600` |
| signed byte table | `0x8042C1A0` | `addi r29,r29,-15968` @`0x806A760C` |
| line-id singleton | `0x80429D60` | `lwz r29,-25248(r3)` @`0x806A76E8` |

**Confidence: high.** Every store, both loop bounds (`< 2`), the state writes
at `+0x360`/`+0x6D8`, and all nine callees are explicit. The `(f32)(s8)` ladder
is the standard signed form (`x ^ 0x80000000` against `2^52 + 2^31`), not the
unsigned form used in `0x807616E0`.

---

## 11. `0x807819B0` - 388 bytes - `work/ghidra/rel-d_basesNP/export/807819b0_FUN_807819b0.c`

**Body:** `0x807819B0..0x80781B30` (97 words, `blr` at `0x80781B30`).
Ledger: absent. `src/game/bases.rs`: absent.

```rust
pub unsafe fn fn_807819b0(this: usize) -> u32
```

**Frame.** `stwu r1,-64(r1)`; `mfspr r0,LR` / `stw r0,68(r1)`;
`stfd f31,48(r1)` + `psq_st f31,56(r1),0,0`;
`stw r31,44` / `stw r30,40` / `stw r29,36` / `stw r28,32`;
`or r28,r3,r3` (**r28 = this**), `r30 = [0x8042A1A0]` (**r30 = the manager**).

**Shape.** A proximity/pending test. Returns 1 when the actor is either too far
along the manager's facing axis or below it, gated by a countdown.

1. `0x807819E0-0x80781A04` - `v = { *(f32*)(r30+0x2D0), *(f32*)(r30+0x2D4),
   *(f32*)(r30+0x2D8) }` at `r1+0x14/0x18/0x1C`;
   `bl 0x801C1650(r3 = r30+0xAC, r4 = &r1+0x14)`;
   `bl 0x802BE470` (float argument `f1`, returns `f1`);
   `f1 = fabs(f1)` then `frsp`.
2. `0x80781A10-0x80781A2C` - `if (f1 <= [0x807CF7A8] (FLT_EPSILON))`:
   the predicate is `fcmpo cr0,f1,f0` + `cror cr0.eq,cr0.lt,cr0.eq` +
   `mfcr r0` + `rlwinm. r0,r0,3,31,31` (extract `CR0.EQ`) + `beq` *past* the
   block, i.e. enter the block when `lt|eq`.
3. `0x80781A30-0x80781A58` - `n = *(i32*)(this+0x2D4)`
   (`lwz r3,724(r28)`):
   * `n < 0` -> `*(u32*)(this+0x2D4) = 2` (`li r0,2` / `stw`);
   * `n == 0` -> `hit = true` (`li r29,1` at `0x80781A58`);
   * `n > 0` -> `*(i32*)(this+0x2D4) = n - 1`.
4. `0x80781A5C-0x80781A6C` - `u = 0; if (hit) u = cntlzw(*(u32*)(this+0x220)) >> 5`
   i.e. `u = (*(u32*)(this+0x220) == 0)`. Encoded as
   `cntlzw r0,r0` + `rlwinm r29,r0,27,5,31`, which is non-zero exactly when the
   count is 32.
5. `0x80781A70-0x80781AD0` - `if (u != 0)`:
   `d3 = { this+0xAC - r30+0xAC, this+0xB0 - r30+0xB0, this+0xB4 - r30+0xB4 }`
   at `r1+8/0xC/0x10` (`stfs` at `0x80781AA0/A4/A8`, `r3 = &r1+8`);
   `len = 0x801C14C0(&r1+8)` -> `f31`;
   `r = 0x807812E0(this)` -> `f1`;
   `f0 = [0x807CF7FC] (2.0) * f1`; `fcmpo cr0,f31,f0`;
   `u = CR0.GT` via `mfcr r29` + `rlwinm r29,r29,1,31,31`.
   **This is `f31 > 2.0*r`, i.e. the opposite polarity to Ghidra's rendered
   `dVar6 < (float)(2.0 * dVar7)`.** The instruction stream is authoritative;
   the mask `31,31` is the LSB, so the rotated word's LSB is the original
   `CR0.GT` bit.
6. `0x80781AD4-0x80781AF8` - `v = 0; if (0x80781950(this) != 0 &&
   *(f32*)(this+0xB0) < *(f32*)(r30+0xB0)) v = 1;`
   (`ble` at `0x80781AF4` past the `li r31,1`).
7. `0x80781AFC-0x80781B30` - `return ((u | v) != 0) as u32`, encoded as
   `or r3,r29,r31` / `neg r0,r3` / `or r0,r0,r3` / `rlwinm r3,r0,1,31,31`
   (extract the sign bit).

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80781A00` | `0x801C1650` | `r3 = r30+0xAC`, `r4 = &r1+0x14` -> `f1` |
| `0x80781A04` | `0x802BE470` | `f1 = previous result` -> `f1` |
| `0x80781AAC` | `0x801C14C0` | `r3 = &r1+8` -> `f1` |
| `0x80781AB8` | `0x807812E0` | `r3 = this` -> `f1` (already translated, `fn_807812e0`) |
| `0x80781ADC` | `0x80781950` | `r3 = this` -> `u32` (already translated, `fn_80781950`) |

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 1.1920929e-7` (FLT_EPSILON) | `0x807CF7A8` | `lfs f0,-2136(r3)` @`0x80781A10` |
| `f32 2.0` | `0x807CF7FC` | `lfs f0,-2052(r3)` @`0x80781AC0` |
| manager singleton | `0x8042A1A0` | `lwz r30,-24160(r4)` @`0x807819DC` |

**Confidence: high.** Every operand is a literal offset; two of the five
callees are already in `src/game/bases.rs` with matching signatures, and the
final `(x.wrapping_neg() | x) >> 31` idiom is the same one `fn_80781950` uses at
`src/game/bases.rs:76285`. Only the polarity noted in step 5 needs care.

---

## 12. `0x80780010` - 376 bytes - `work/ghidra/rel-d_basesNP/export/80780010_FUN_80780010.c`

**Body:** `0x80780010..0x80780184` (94 words, `blr` at `0x80780184`).
Ledger: absent. `src/game/bases.rs`: absent. No `psq`.

```rust
pub unsafe fn fn_80780010(this: usize)
```

**Frame.** `stwu r1,-80(r1)`; `mfspr r0,LR` / `stw r0,84(r1)`;
`stw r31,76` / `stw r30,72` / `stw r29,68` / `stw r28,64`;
`r4 = 1` at `0x80780018`; `or r31,r3,r3` (**r31 = this**);
`r29 = 0x80430000`; `r28 = [0x8042A1A0]` (**r28 = the manager**).

**Shape.** A respawn/reposition initialiser: snapshot the previous handle,
allocate a new one, copy the manager's angle, then run the two placement
helpers with three position snapshots and a ground probe.

1. `0x8078003C-0x80780048` - `*(u32*)(this+0x204) = *(u32*)(this+0x200)`;
   `*(u8*)(this+0x18C) = 1`.
2. `0x80780048-0x80780054` - `*(u32*)(this+0x200) = 0x8079A3E0(r28)`
   (already translated, `fn_8079a3e0(this) -> u32`).
3. `0x80780058-0x80780060` - `*(u8*)(this+0x20C) = 0`;
   `*(u8*)(this+0x2A4) = 0` (`li r0,0` at `0x80780050`).
4. `0x80780064-0x8078006C` - `*(u16*)(this+0x1E6) = *(u16*)(r28+0x102)`
   (`lha r0,258(r4)` / `sth r0,486(r31)`).
5. `0x80780070` - `bl 0x80780510(this)` (already translated, `fn_80780510`).
6. `0x80780074-0x80780088` - `bl 0x8077E410(f1 = [0x807CF698] (0.0),
   r3 = this, r4 = 2, r5 = -1)` (`lfs f1,-2408(r30)`,
   `addi r4,r0,2`, `addi r5,r0,-1`).
7. `0x8078008C-0x80780090` - `*(u32*)(this+0x1D0) = 7`.
8. `0x80780094-0x807800F0` - three position snapshots:
   `A = { r28+0xAC, r28+0xB0, r28+0xB4 }` at `r1+0x2C/0x30/0x34`;
   `B = { this+0xAC, this+0xB0, this+0xB4 }` at `r1+0x20/0x24/0x28`;
   `C = { r28+0xAC, r28+0xB0, r28+0xB4 }` at `r1+0x14/0x18/0x1C`
   (`A` and `C` are the same three loads emitted twice).
9. `0x807800F4-0x80780120` - ground probe:
   `course = [0x8042A18C] + *(i32*)([0x8042A18C] + 0x338C) * 0x16518 + 0x3394`
   (`lwz r7,-24180(r4)`; `addis r3,0,1` / `addi r0,r3,25880` materialises
   `0x16518`; `lwz r3,13196(r7)`; `mullw`; `addi r3,r3,13204`);
   `0x800F57A0(course, *(u32*)(this+0x1F4), 0x800)`;
   `if (result != 0)`: `*(f32*)(this+0xB0) = 0.0`, and the `y` lanes of both
   snapshots are zeroed: `*(f32*)(r1+0x30) = 0.0` and
   `*(f32*)(r1+0x18) = 0.0` (`0x80780118`, `0x80780120`).
10. `0x80780124-0x8078013C` - `bl 0x80780BD0(r3 = &r1+8, r4 = this,
    r5 = &r1+0x2C, r6 = (s16)*(u16*)(r28+0x102))`
    (`lha r6,258(r6)` at `0x80780138`).
11. `0x80780140-0x80780150` - `bl 0x807813F0(r3 = this, r4 = &r1+0x20,
    r5 = &r1+0x14, r6 = &r1+8)`.
12. `0x80780154-0x80780164` - `*(u8*)(this+0x284) = 0`;
    `*(f32*)(this+0x2C4) = [0x807CF698] (0.0)`.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x8078004C` | `0x8079A3E0` | `r3 = r28` -> `u32` (translated) |
| `0x80780070` | `0x80780510` | `r3 = this` (translated) |
| `0x80780088` | `0x8077E410` | `f1 = 0.0`, `r3 = this`, `r4 = 2`, `r5 = -1` |
| `0x80780108` | `0x800F57A0` | `r3 = course`, `r4 = *(u32*)(this+0x1F4)`, `r5 = 0x800` -> `u32` |
| `0x8078013C` | `0x80780BD0` | `r3 = &r1+8`, `r4 = this`, `r5 = &r1+0x2C`, `r6 = (s16)*(u16*)(r28+0x102)` |
| `0x80780150` | `0x807813F0` | `r3 = this`, `r4 = &r1+0x20`, `r5 = &r1+0x14`, `r6 = &r1+8` |

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 0.0` | `0x807CF698` | `lfs f1,-2408(r30)` @`0x8078007C` |
| manager singleton | `0x8042A1A0` | `lwz r28,-24160(r29)` @`0x80780038` |
| scene/course root | `0x8042A18C` | `lwz r7,-24180(r4)` @`0x807800A4` |
| course stride | `0x16518` | `addis r3,r0,1` / `addi r0,r3,25880` @`0x80780098` |
| course table offset | `0x3394` | `addi r3,r3,13204` @`0x80780104` |

**Confidence: medium.** Every operand is explicit and the course arithmetic
matches the already-translated `fn_80781950` verbatim
(`src/game/bases.rs:76283`). The two soft spots are (a) `A` and `C` being the
same three manager loads emitted twice - which is faithful but looks like a
compiler artefact, and (b) the three-vector locals being consumed by two
callees whose signatures are not in this REL. Emit them as `[f32; 3]` stack
arrays with `// unknown: 0x80780BD0/0x807813F0 vector roles`.

---

## 13. `0x806B7340` - 368 bytes - `work/ghidra/rel-d_basesNP/export/806b7340_FUN_806b7340.c`

> Overflow body: ranks 13th by size. Under the heading-only rule it is already
> excluded by items 1-3; if those three are dropped instead, this takes their
> place in the twelve. Specified to the same standard.

**Body:** `0x806B7340..0x806B74AC` (92 words, `blr` at `0x806B74AC`).
Ledger: absent. `src/game/bases.rs`: absent.

```rust
pub unsafe fn fn_806b7340(this: usize)
```

**Frame.** `stwu r1,-80(r1)`; `mfspr r0,LR` / `stw r0,84(r1)`;
`addi r11,r1,64`; `stfd f31,64(r1)` + `psq_st f31,72(r1),0,0`;
`bl 0x802DCD74` at `0x806B7358` (saves **r27-r31**);
`or r27,r3,r3` (**r27 = this**). Epilogue `0x806B7490..0x806B74AC`
(`bl 0x802DCDC0`).

**Shape.** A per-frame emitter: for every active entry of a list it orients one
display object at a recorded position and scales it.

1. `0x806B735C-0x806B737C` - `f31 = [0x807CAB58]` (0.1);
   `r31 = this` (advances `+4`); `r30 = this+0x3F0` (advances `+0x40`);
   `r29 = 0x807CAB7C` (advances `+4`); `r28 = 0` (counter);
   `b 0x806B7484` (bottom-test loop).
2. Loop head `0x806B7380`, test `0x806B7484-0x806B748C`:
   `count = *(i32*)(this+0x53C)` (`lwz r0,1340(r27)`);
   `cmp cr0,0,r28,count` / `blt 0x806B7380`.
3. `0x806B7380-0x806B7398` - `sel = *(i32*)(this + 0xCC0 + 4*i)`
   (`lwz r4,3264(r31)`); `if (!(0.1 < *(f32*)(this + 0xAE0 + 4*sel)))` skip to
   the loop tail. The predicate is `fcmpo cr0,f0,f31` + `cror cr0.eq,cr0.lt,
   cr0.eq` + `beq 0x806B7474`, i.e. "branch when `value <= 0.1`".
4. `0x806B739C-0x806B73E0` - build the argument block:
   `e = this + 12*sel` (`mulli r0,r4,12` / `add r6,r27,r0`);
   `v = { e+0x540, e+0x544, e+0x548 }` -> `r1+0x1C/0x20/0x24`;
   `ang = { (s16)*(this+0x100), (s16)*(this+0x102), (s16)*(this+0x104) }`
   -> three `u16` at `r1+0x14/0x16/0x18`
   (`lha r0,256(r27)` / `258` / `260`, then `sth` to `20(r1)`, `22(r1)`,
   `24(r1)`);
   `bl 0x80064AF0(r3 = &r1+0x1C, r4 = &r1+0x14, r5 = 1)` transforms `v` in
   place.
5. `0x806B73E4-0x806B7430` - `bl 0x801C0BD0(f1 = v.x, f2 = v.y, f3 = v.z,
   r3 = this+0x7C)`; then three angle setters on the same object:
   `bl 0x8016EDD0(r3 = this+0x7C, r4 = &r1+0x10)` (`ang[1]`),
   `bl 0x8016ECB0(r3 = this+0x7C, r4 = &r1+0x0C)` (`ang[0]`),
   `bl 0x8016EEF0(r3 = this+0x7C, r4 = &r1+0x08)` (`ang[2]`).
6. `0x806B7434-0x806B7470` - `bl 0x8016A170(r3 = this+0x3F0+0x40*i,
   r4 = this+0x7C)`;
   `s = *(f32*)(this + 0xAE0 + 4*sel) * *(f32*)(0x807CAB84 + 4*i)`
   (`lfs f0,8(r29)` with `r29` starting at `0x807CAB7C`; the `lfs` offset is
   `+8`, so the first element read is `0x807CAB84`);
   `bl 0x8016A130(f1 = s, f2 = s, f3 = s, r3 = this+0x3F0+0x40*i)`;
   `bl 0x8016A1A0(r3 = this+0x3F0+0x40*i, r4 = 0)`.
7. `0x806B7474-0x806B7480` - `r31 += 4`, `r30 += 0x40`, `r29 += 4`, `r28 += 1`.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x806B7358` | `0x802DCD74` | save r27-r31; `r11 = r1+64` |
| `0x806B73E0` | `0x80064AF0` | `r3 = &r1+0x1C` (vec3, in/out), `r4 = &r1+0x14` (3x `s16` angles), `r5 = 1` |
| `0x806B73F4` | `0x801C0BD0` | `f1/f2/f3 = v.x/v.y/v.z`, `r3 = this+0x7C` |
| `0x806B7408` | `0x8016EDD0` | `r3 = this+0x7C`, `r4 = &r1+0x10` |
| `0x806B741C` | `0x8016ECB0` | `r3 = this+0x7C`, `r4 = &r1+0x0C` |
| `0x806B7430` | `0x8016EEF0` | `r3 = this+0x7C`, `r4 = &r1+0x08` |
| `0x806B743C` | `0x8016A170` | `r3 = this+0x3F0+0x40*i`, `r4 = this+0x7C` |
| `0x806B7464` | `0x8016A130` | `f1 = f2 = f3 = ` scale, `r3 = this+0x3F0+0x40*i` |
| `0x806B7470` | `0x8016A1A0` | `r3 = this+0x3F0+0x40*i`, `r4 = 0` |
| `0x806B749C` | `0x802DCDC0` | restore r27-r31; `r11 = r1+64` |

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 0.1` | `0x807CAB58` | `lfs f31,-21672(r4)` @`0x806B7368` |
| scale table base | `0x807CAB7C` | `addi r29,r29,-21636` @`0x806B7374` |
| `f32 0.75` | `0x807CAB84` | `lfs f0,8(r29)` @`0x806B7448` |
| `f32 0.7` | `0x807CAB88` | `lfs f0,8(r29)` @`0x806B7448`, i = 1 |
| `f32 0.42` | `0x807CAB8C` | `lfs f0,8(r29)` @`0x806B7448`, i = 2 |

Verified scale table: `0x807CAB84 = 0.75`, `0x807CAB88 = 0.69999999`,
`0x807CAB8C = 0.41999999`. Only the first three entries are reachable for the
shipped list lengths; read the table by address.

**Confidence: medium.** Control flow, the two `s16` triples and all nine call
sites are explicit. The soft spot is `0x80064AF0`'s contract: it is called with
a `vec3` **and** a 3x `s16` angle block and a literal `1`, and it modifies the
`vec3` in place (`0x806B73E4` re-reads `f1/f2/f3` from the same three slots).
That is visible from the registers, but the meaning of the third argument is
not. Add `// unknown: 0x80064AF0(v, angles_s16x3, 1)`.

---

## 14. `0x80749750` - 360 bytes - `work/ghidra/rel-d_basesNP/export/80749750_FUN_80749750.c`

**Body:** `0x80749750..0x807498B4` (90 words, `blr` at `0x807498B4`).
Ledger: absent. `src/game/bases.rs`: absent.

```rust
pub unsafe fn fn_80749750(this: usize)
```

**Frame.** `stwu r1,-96(r1)`; `mfspr r0,LR` / `stw r0,100(r1)`;
`addi r11,r1,80`; `stfd f31,80(r1)` + `psq_st f31,88(r1),0,0`;
`bl 0x802DCD6C` at `0x80749768` (saves **r25-r31**);
`or r30,r3,r3` (**r30 = this**). Epilogue `0x80749898..0x807498B4`
(`bl 0x802DCDB8`).

**Shape.** Build a four-entry (value, id) table - one per player slot - then sort
it descending and publish the four ids.

1. `0x8074976C-0x8074978C` - `f31 = [0x807CE53C]` (-500000.0) is the "absent"
   sentinel; `r27 = &r1+8` (values), `r26 = &r1+0x18` (ids), `r31 = 0`
   (count), `r25 = 0` (slot), `r28 = 0x80430000`, `r29 = 1`.
2. `0x80749790-0x807497F0` - `do { ... } while (slot < 4)` (head
   `0x80749790`, back edge `0x807497F0 blt`):
   `p = 0x8005F900(slot)`;
   `values[i] = -500000.0` (`stfs f31,0(r27)`);
   `ids[i] = slot` (`stw r25,0(r26)`);
   `if (p != 0 && (*(u8*)0x80429CA8 & (1u32 << (slot & 0x1F))) != 0)`
   (`lbz r4,-25432(r28)` / `slw r0,r29,r0` with `r29 = 1` /
   `and. r0,r4,r0`):
   `values[i] = (*(i32*)(this+0x400) == 1) ? *(f32*)(p+0xAC)
   : *(f32*)(p+0xB0)`; `count += 1` (`addi r31,r31,1` at `0x807497DC`).
   `slot += 1`, `r26 += 4`, `r27 += 4`.
3. `0x807497F4-0x80749870` - descending selection sort over the four pairs:
   outer `i = 0..2` (`addi r8,r8,1` / `cmpi cr0,0,r8,3` / `blt`), inner
   `j = i+1..3` driven by `mtspr CTR,(4-j)` at `0x8074981C` and the
   `bdnz` at `0x8074985C` (`bc 16,lt`);
   if `values[i] < values[j]` (`fcmpo` / `bge` past) swap both the `f32` and
   the `u32` (`lwz r7,0(r4)` / `lwz r0,0(r6)` / `stw` / `stfs` / `stw` /
   `stfs` at `0x80749838-0x8074984C`).
4. `0x80749874-0x80749894` - publish the four sorted ids:
   `*(u32*)(this+0x3DC) = ids[0]` (`lwz r6,24(r1)`),
   `*(u32*)(this+0x3E0) = ids[1]` (`lwz r5,28(r1)`),
   `*(u32*)(this+0x3E4) = ids[2]` (`lwz r4,32(r1)`),
   `*(u32*)(this+0x3E8) = ids[3]` (`lwz r0,36(r1)`).
5. `0x80749878-0x807498A4` - `r3 = count` is left in `r3` before the
   `bl 0x802DCDB8` restore; the value is not consumed by this body.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80749768` | `0x802DCD6C` | save r25-r31; `r11 = r1+80` |
| `0x80749794` | `0x8005F900` | `r3 = slot` -> player pointer or 0 |
| `0x807498A4` | `0x802DCDB8` | restore r25-r31; `r11 = r1+80` |

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 -500000.0` | `0x807CE53C` | `lfs f31,-6852(r4)` @`0x80749774` |
| active-player bitmask | `0x80429CA8` | `lbz r4,-25432(r28)` @`0x807497AC` |

**Confidence: high.** Both loops are fixed-bound (4 and 3/`CTR`), the sentinel
and the two field offsets (`this+0x400`, `p+0xAC`, `p+0xB0`) are literal, and
the sort is an ordinary descending bubble/selection. The stack arrays are
`f32[4]` at `r1+8` and `u32[4]` at `r1+0x18` - both fully populated before any
read, so no uninitialised lane reaches the sort.

---

## 15. `0x806B7B70` - 344 bytes - `work/ghidra/rel-d_basesNP/export/806b7b70_FUN_806b7b70.c`

**Body:** `0x806B7B70..0x806B7CC4` (86 words, `blr` at `0x806B7CC4`).
Ledger: absent. `src/game/bases.rs`: absent.

```rust
pub unsafe fn fn_806b7b70(this: usize, value: f32)
```

**Note on the export signature.** Ghidra prints
`FUN_806b7b70(double param_1,int param_2)`. The registers say otherwise:
`r3` is dereferenced four times (`lfs f2,172(r3)` at `0x806B7BB4`,
`lbz r5,911(r29)` after `or r29,r3,r3`, `lbz r0,4160(r29)`,
`lfs f1,172(r29)`), and the float arrives in `f1` (`fmr f31,f1` at
`0x806B7B90`). So the true signature is `(this: usize, value: f32)`.

**Frame.** `stwu r1,-80(r1)`; `mfspr r0,LR` / `stw r0,84(r1)`;
`stfd f31,64(r1)` + `psq_st f31,72(r1),0,0`;
`stw r31,60` / `stw r30,56` / `stw r29,52`;
`r6 = 1`, `r7 = -1` (`0x806B7B78`/`0x806B7B7C`);
`f31 = f1` (`0x806B7B90`); `r31 = 0x807CAB48`; `r30 = 0`;
`or r29,r3,r3` (**r29 = this**).

**Shape.** A squash/impact probe: cast a ray down from the actor, and if the
floor is close, raise the hit effect and the impact sound for the current
player.

1. `0x806B7BA8-0x806B7BD0` - build the query: `origin = { *(f32*)(this+0xAC),
   f31, [0x807CAB60] (6500.0) }` at `r1+0x24/0x28/0x2C`;
   `r1+8 = f31` (scratch for the returned distance);
   `r5 = *(u8*)(this+0x38F)`;
   `bl 0x800757B0(r3 = &r1+0x24, r4 = &r1+8, r5 = *(u8*)(this+0x38F),
   r6 = 1, r7 = -1)`.
2. `0x806B7BD4-0x806B7BF4` - `if (result != 0 && (f31 - *(f32*)(r1+8)) <
   [0x807CAB64] (32.0))` -> `r30 = 1` (`fcmpo` at `0x806B7BE8`, `bge` past the
   `li r30,1` at `0x806B7BF0`).
3. `0x806B7BF4-0x806B7C10` - `b = *(u8*)(this+0x1040)`;
   `r4 = 0`; `if (b == 0 && r30 != 0) r4 = 1` (the `bne`/`beq` pair at
   `0x806B7C04`/`0x806B7C0C` guards the `li r4,1` at `0x806B7C10`).
4. `0x806B7C14-0x806B7C48` - `rgb = { [0x807CAB54], [0x807CAB54],
   [0x807CAB54] }` (all three are 1.0) at `r1+0x18/0x1C/0x20`;
   `arg4 = (*(u8*)(this+0x38F) << 16) | (r4 != 0 ? 1 : 0)`.
   The packing is `rlwinm r5,r5,16,0,15` (`0x806B7C28`) followed by
   `rlwimi r5,r0,1,31,31` (`0x806B7C38`) where `r0 = (-r4) | r4`
   (`neg` + `or` at `0x806B7C18`/`0x806B7C24`). Mask `31,31` is the LSB, so
   the merge writes exactly bit 0 - the flag is a single bit, not a shifted
   copy.
   `bl 0x800929F0(r3 = [0x80429EE8], r4 = &r1+0x24, r5 = arg4, r6 = -1,
   r7 = &r1+0x18)`.
5. `0x806B7C4C-0x806B7C70` - `slot = (b != 0) ? 6 : 7` (`li r4,7` then
   `beq` past the `li r4,6`);
   `bl 0x80078410(f1 = *(f32*)(this+0xAC), f2 = f31, r3 = [0x80429DD0],
   r4 = slot)`.
6. `0x806B7C74-0x806B7CA0` - `bl 0x8006A4B0(r3 = &r1+0x10, r4 = &r1+0x24)`;
   `bl 0x80198FC0(r3 = [0x80429D60], r4 = 0x282, r5 = &r1+0x10,
   r6 = (s8)*(u8*)(this+0x1040), r7 = 0)` (`extsb r6,r30` at `0x806B7C94`).

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x806B7BD0` | `0x800757B0` | `r3 = &r1+0x24`, `r4 = &r1+8`, `r5 = *(u8*)(this+0x38F)`, `r6 = 1`, `r7 = -1` -> `u32` |
| `0x806B7C48` | `0x800929F0` | `r3 = [0x80429EE8]`, `r4 = &r1+0x24`, `r5 = arg4`, `r6 = -1`, `r7 = &r1+0x18` |
| `0x806B7C70` | `0x80078410` | `f1 = *(f32*)(this+0xAC)`, `f2 = f31`, `r3 = [0x80429DD0]`, `r4 = 6 or 7` |
| `0x806B7C88` | `0x8006A4B0` | `r3 = &r1+0x10`, `r4 = &r1+0x24` |
| `0x806B7CA0` | `0x80198FC0` | `r3 = [0x80429D60]`, `r4 = 0x282`, `r5 = &r1+0x10`, `r6 = (s8)*(u8*)(this+0x1040)`, `r7 = 0` |

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 1.0` (rgb triple) | `0x807CAB54` | `lfs f0,12(r31)` @`0x806B7C14` |
| `f32 6500.0` (ray length) | `0x807CAB60` | `lfs f0,24(r31)` @`0x806B7BA8` |
| `f32 32.0` (proximity) | `0x807CAB64` | `lfs f0,28(r31)` @`0x806B7BE0` |
| effect emitter | `0x80429EE8` | `lwz r3,-24856(r3)` @`0x806B7C34` |
| sound singleton | `0x80429DD0` | `lwz r3,-25136(r3)` @`0x806B7C58` |
| line-id singleton | `0x80429D60` | `lwz r31,-25248(r3)` @`0x806B7C7C` |

**Confidence: medium.** Every operand, branch and store is explicit. The one
item that needs a `// unknown:` note is the packed `r5` word handed to
`0x800929F0`: only bit 0 (the "close enough" flag) and bits 16-31 (the player
byte) are set, and the callee's use of the remaining bits is not visible here.
Write it as the explicit
`((*(u8*)(this+0x38F) as u32) << 16) | ((flag != 0) as u32)`.

---

## Ready vs conditional

Twelve largest first, then the three that the strictest reading of "absent from
the spec files" would substitute.

| # | address | bytes | verdict | reason |
|--:|---:|---:|---|---|
| 1 | `0x8066FA10` | 508 | **ready** | four-phase table rebuild + resample; `r25`/`r26` compaction quirk documented; one `// unknown:` on `0x80052080` |
| 2 | `0x806A4CA0` | 504 | **ready** | twelve explicit virtual slots, two `0x8015F470` envelope pairs, three clamps |
| 3 | `0x807816B0` | 460 | **conditional** | the 48-byte basis at `r1+0x28` is written *only* by `0x8016ED10`; its layout is a guess |
| 4 | `0x80762CE0` | 440 | **conditional** | six facing floats live in `main.dol`'s zero-fill segment (`0x80377DD4..F4`); the `0x80815494` record has a `+0x1A` field that overlaps the next 24-byte entry |
| 5 | `0x806333D0` | 416 | **conditional** | the 12-word 3x4 passed to `0x800B32E0` is assembled from interleaved position fields (`m[0]==m[6]`, `m[1]==m[10]`, `m[2]=0.0`); the transcription is exact but the row/column roles are not recoverable |
| 6 | `0x8069AB00` | 408 | **ready** | every store, both thresholds (`5.0`, `2.5`) and all five virtual slots explicit |
| 7 | `0x807616E0` | 408 | **ready** | three-way door dispatch; only the `2^31` widening ladder needs a literal, not a cast |
| 8 | `0x80656310` | 396 | **ready** | two sound calls + one sample-layout read; all offsets literal |
| 9 | `0x80656D20` | 392 | **ready** | event `0x12A` push-apart; clamp `[2.0, 6.0]` and four store lanes explicit |
| 10 | `0x806A75B0` | 392 | **ready** | two-state lift driver; `qr0` is only the `f31/f30` spill |
| 11 | `0x807819B0` | 388 | **ready** | five literal callees, two of them already translated; one Ghidra polarity inversion documented |
| 12 | `0x80780010` | 376 | **ready** | respawn init; course arithmetic identical to the already-merged `fn_80781950` |
| 13 | `0x806B7340` | 368 | **ready** | per-entry emitter; three-entry scale table `0.75 / 0.7 / 0.42` verified |
| 14 | `0x80749750` | 360 | **ready** | fixed 4-slot table + descending sort; sentinel and mask explicit |
| 15 | `0x806B7B70` | 344 | **ready** | ray probe; `6`/`7` sound slots and the `rlwimi` bit-0 flag explicit |

**12 ready / 3 conditional** across the fifteen bodies written up here.
Restricted to the headline twelve (items 1-12) the split is **9 ready /
3 conditional**: `0x807816B0`, `0x80762CE0` and `0x806333D0`.

Eight of the twelve are specified at `medium` (`0x8066FA10`, `0x807616E0`,
`0x80656310`, `0x80656D20`, `0x80780010`, and - among the overflow bodies -
`0x806B7340` and `0x806B7B70`) because each needs one `// unknown:` note on a
callee whose argument *layout* is not visible in this REL, not because any
operand is missing. The `high` entries (`0x806A4CA0`, `0x8069AB00`,
`0x806A75B0`, `0x807819B0`, `0x80749750`) are entirely literal.

None of the three conditionals needs a new artifact to be *transcribed* - each
has a byte-exact form listed above - but each would carry a naming/structure
guess, so they are held at `medium`. Crucially, **no body in this batch needs
the GQR3 word, a jump-table generator, or `main.dol` disassembly**; the six
floats in `0x80762CE0` are read as runtime memory, exactly as the
wave-128/129/130 lanes handle `[0x80429D60]` and `[0x80377C48]`.

## Verification

Checks were run on all fifteen bodies (the headline twelve plus §13-§15).

- `ledger/functions.csv`: none of the fifteen occurs in the `address` column of
  any row (exact `,<hex>,` search over all 16 190 lines).
- `docs/SPEC_WAVE128_BASES.md` / `SPEC_WAVE129_BASES.md` /
  `SPEC_WAVE130_BASES.md`: the `## N.` headings of each were extracted
  (12 + 12 + 12 = 36) and subtracted. The only residual textual hits among the
  fifteen are the three "outside this batch" mentions tabulated in
  "Why these twelve" (`0x8066FA10`, `0x806A4CA0`, `0x807816B0`); none of them
  has a heading in the earlier lanes.
- `src/game/bases.rs`: no `pub unsafe fn fn_<address>` for any of the fifteen.
  `0x806333D0`, `0x807616E0` and `0x80762CE0` appear only inside doc comments
  (lines 640/134639/134668, 139265, 140017 respectively); `0x806A4CA0`,
  `0x807816B0` and `0x8066FA10` only inside module/doc comments (lines 1652,
  152444; 161880, 163066, 163076; 156217-156231).
- `work/ghidra/rel-d_basesNP/export/functions.tsv`: every `bytes` value above
  matches the TSV `size` column exactly.
- Body extents: all fifteen end in `0x4E800020` (`blr`) at the last word of
  their TSV extent, and every internal branch target is inside the extent. No
  `halt_baddata`, no `unaff_GQR*`, no "could not recover jumptable" in any of
  the fifteen exports.
- Paired-single audit: 34 `psq*` words total, **all** `I = 0` (`qr0`) and
  `W = 0`: 17 prologue `psq_st fNN` saves and the 17 matching epilogue
  `psq_l fNN` reloads, covering seventeen saved float registers (`f31` in most
  bodies, plus `f30` in `0x806A75B0` and `f30/f29/f28` in `0x8066FA10`).
  `0x80656D20` and `0x80780010` contain none. Zero `mtspr` to any `GQRn`; the
  only `mtspr` operands are `LR` (spr 8) and `CTR` (spr 9).
- SHA-1 of each 4-byte-aligned extent, in section order:
  `aed28b6d8cb3`, `8c0c9fb97936`, `a52e73c442d0`, `512758f46171`,
  `fa3879186c92`, `270eea9154ad`, `1113665c1446`, `37f9e0d21fc6`,
  `3b6ce66457d6`, `fc80950d08bb`, `eff81cfeb85d`, `2e1c8acab8ab`,
  `f2cb39efa3e6`, `4158f93d10ee`, `87635d859eba` - pairwise distinct, so no
  body in this batch is a duplicate of another.
- Save-helper ladder re-decoded from `work/dol/main.elf`
  (`0x802DCD40..0x802DCDD4`) to confirm `0x802DCD68`/`6C`/`70` and their
  restores; see the table in "Method". `0x802DCD68` (used by `0x8066FA10`)
  saves **r24-r31** and restores at `0x802DCDB4`.
- All `.data` constants were read through the `PT_LOAD` rule, not
  `addr - 0x80600000`: `0x807C7C9C`, `0x807C88F8..0x807C8910`,
  `0x807C8968..0x807C897C`, `0x807C9028..0x807C9070`,
  `0x807CA0C4`, `0x807CA0EC`, `0x807CA2D8..0x807CA3E8`,
  `0x807CA448..0x807CA468`, `0x807CAB54..0x807CAB8C`,
  `0x807CEDB0..0x807CEDE0`, `0x807CEE5C..0x807CEE64`, `0x807CE53C`,
  `0x807CF698`, `0x807CF798`, `0x807CF7A8`, `0x807CF7F0`, `0x807CF7F8`,
  `0x807CF7FC`, `0x807F1404`, `0x80815494..0x80815524`.

## Hand-off

After this batch the unclaimed class-B pool is **70 addresses / 13 388 bytes**
(85 - 15 addresses, 19 548 - 6 160 bytes). The next six by size are
`0x80761510` (340), `0x8078B1D0` (340), `0x806B7890` (336),
`0x806B7690` (312), `0x806A23B0` (308) and `0x80722180` (296); the first four
share the `f31`-spill-only pattern that made items 10, 13 and 15 of this batch
straightforward.
