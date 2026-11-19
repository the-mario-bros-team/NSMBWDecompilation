# Wave 135 - `game::bases` - implementation spec for the next 12 class-B addresses

Read-only lane. The only repository file written is this one. The throwaway
PowerPC decoder (`%TEMP%\opencode\ppc135.js`, with `dump135.js`,
`audit135.js`, `dup135.js`, `const135b.js`, `main135.js`, `maindata135.js`
beside it) lives outside the repository; no Rust file, no `ledger/functions.csv`
row and no existing document was modified. No merge or check tool was run.

## Why these twelve

`docs/ANALYSIS_WAVE127_BASES.md` reclassified 121 `d_basesNP` skips as class B
("reconstructible now from the raw stream"). Waves 128-133 carried 75 of them
into `docs/SPEC_WAVE128_BASES.md` .. `docs/SPEC_WAVE133_BASES.md`, and wave 134
took the next twelve plus the tied 13th body (`0x8069E5F0`, spec
`docs/SPEC_WAVE134_BASES.md` §13). That leaves **33** class-B addresses,
**3 664** bytes, still without a section of their own.

This lane takes the twelve largest of those, ordered by `functions.tsv` size
descending and then by address ascending:

| # | address | bytes | words | export file | verdict |
|--:|---:|---:|---:|---|---|
| 1 | `0x806FB540` | 200 | 50 | `806fb540_FUN_806fb540.c` | ready (high) |
| 2 | `0x80725170` | 196 | 49 | `80725170_FUN_80725170.c` | ready (high) |
| 3 | `0x8069A170` | 184 | 46 | `8069a170_FUN_8069a170.c` | ready (medium) |
| 4 | `0x806A8AA0` | 184 | 46 | `806a8aa0_FUN_806a8aa0.c` | conditional (medium) |
| 5 | `0x80761880` | 176 | 44 | `80761880_FUN_80761880.c` | ready (medium) |
| 6 | `0x80656260` | 172 | 43 | `80656260_FUN_80656260.c` | conditional (medium) |
| 7 | `0x807481B0` | 172 | 43 | `807481b0_FUN_807481b0.c` | ready (medium) |
| 8 | `0x80702CE0` | 168 | 42 | `80702ce0_FUN_80702ce0.c` | ready (high) |
| 9 | `0x8078B120` | 168 | 42 | `8078b120_FUN_8078b120.c` | ready (medium) |
| 10 | `0x80781D10` | 152 | 38 | `80781d10_FUN_80781d10.c` | conditional (medium) |
| 11 | `0x80716590` | 148 | 37 | `80716590_FUN_80716590.c` | ready (medium) |
| 12 | `0x80608D10` | 136 | 34 | `80608d10_FUN_80608d10.c` | ready (high) |

Together **2 056 bytes** (514 words). Every export path is exactly
`work/ghidra/rel-d_basesNP/export/<file>` relative to the repository root, and
every `bytes` value matches the `size` column of that directory's
`functions.tsv` verbatim.

### Selection rule

The candidate set is: **class B in wave 127**, **no row in
`ledger/functions.csv` whose address field equals the address**, and **no
section title naming the address in any `docs/SPEC_WAVE*_BASES.md`**. The
heading test is title-based (`^## .*0xADDR`), not a substring search: several of
the twelve are named in the prose of other sections, and one
(`0x8069E5F0`) is a full section of `docs/SPEC_WAVE134_BASES.md` whose title
reads `## 13. The tied 13th body: ...` rather than `## 13. \`0x8069E5F0\``.
That body is therefore *not* a candidate here even though a
`## N. \`0xADDR\``-only test would keep it.

| address | ledger mentions | `src/game/bases.rs` mentions |
|---|---:|---:|
| `0x806FB540` | 0 | 5 (doc comments) |
| `0x80725170` | 0 | 0 |
| `0x8069A170` | 0 | 0 |
| `0x806A8AA0` | 0 | 3 (doc comments) |
| `0x80761880` | 0 | 4 (doc comments) |
| `0x80656260` | 0 | 1 (doc comment) |
| `0x807481B0` | 0 | 6 (doc comments) |
| `0x80702CE0` | 0 | 1 (doc comment) |
| `0x8078B120` | 0 | 1 (doc comment) |
| `0x80781D10` | 0 | 0 |
| `0x80716590` | 0 | 0 |
| `0x80608D10` | 0 | 0 |

"Ledger mentions" is a `^game::bases,<addr>,` row test. "Mentions" in
`src/game/bases.rs` counts any textual occurrence of the bare lowercase hex
address; **none of the twelve has a `pub unsafe fn fn_<addr>` definition** as of
this lane's write time.

### Ties, and what is left

The 200/196 pair is clear-cut; the cut is not. `0x806B3AC0` and `0x80608D10`
are both 136 bytes, so `0x80608D10` (lower address) is the 12th and
`0x806B3AC0` is the 13th. The next twelve after the cut, in the same order, are:

| address | bytes | note |
|---|---:|---|
| `0x806B3AC0` | 136 | wave 118 "`undefined8` fragment"; 3 direct calls |
| `0x80608C40` | 132 | wave 11 "collection-coin handler"; virtual tail call |
| `0x8066B0F0` | 128 | wave 126 "receiver-less dispatch"; virtual tail call |
| `0x806FCCA0` | 100 | wave 113 neighbour window; 2 direct calls |
| `0x80681480` | 88 | wave 123 "duplicate release twins"; 2 direct calls |
| `0x80681FA0` | 88 | wave 123 "duplicate release twins"; 2 direct calls |
| `0x806FF3F0` | 88 | wave 11 "wire-mesh lifecycle"; 2 direct calls |
| `0x80600990` | 64 | wave 116 "delete wrapper"; 1 direct call |
| `0x806009D0` | 64 | wave 116 "delete wrapper"; 1 direct call |
| `0x80600A10` | 64 | wave 116 "delete wrapper"; 1 direct call |
| `0x80600A50` | 64 | wave 116 "delete wrapper"; 1 direct call |
| `0x80600A90` | 64 | wave 116 "delete wrapper"; 1 direct call |

## Method

**Address mapping (re-verified, not copied).** `d_basesNP.elf` has six
`PT_LOAD` headers. The first is `p_offset = 0x10F0`, `p_vaddr = 0x806000F0`, so

```
file_offset = 0x10F0 + (address - 0x806000F0) = (address - 0x80600000) + 0x1000
```

The other five are `0x807C60F4` (1 276 B), `0x807C65F0` (12 B), `0x807C6600`
(42 492 B), `0x807D0C00` (335 032 B) and the zero-fill segment at `0x80822900`
(`filesz = 0`, `memsz = 74 884`). Every constant below was read at
`p_offset + (addr - p_vaddr)`.

**Extent check.** For all twelve the reachable span equals the TSV size exactly:
the last word of every range is `0x4E800020` (`blr`) at `entry + size - 4`, and
no `functions.tsv` row starts inside a body. No over-read, no `halt_baddata`, no
jump table. Each body contains exactly one `bclr` (the terminal `blr`).

**Duplicate check.** The SHA-1 of each of the twelve byte ranges was compared
against every `d_basesNP` TSV row of the same size: **no match** - unlike the
wave-123 "release twins", none of these is a relocated copy of an existing body.

**Paired-single audit.** Across the 514 words there are **18 `psq*` words**, in
six bodies (§1, §2, §3, §4, §5, §6, §8 - two words each: one `psq_st` prologue
spill and one `psq_l` epilogue reload of `f31`). All are `qr0` (`I = 0`) with
`W = 0`, and there is **no data-side `psq` at all**, no `qr3`, and no `mtspr` to
a `GQRn`: the only `mtspr` operands in the whole batch are `LR` (spr 8) and, in
§12 only, `CTR` (spr 9). Every "`unaff_GQR0`" rejection that produced these skip
entries was a decompiler artefact.

**Compiler save chain.** Only §7 uses one:
`0x802DCD74` saves `r27..r31` at `-20..-4(r11)` and `0x802DCDC0` reloads them;
both were re-decoded from `work/dol/main.elf` and neither touches `r3..r10` or
`LR`. The body sets `r11 = r1 + 32` immediately before the call, exactly as the
helper expects.

**Cross-artifact cells.** Six cells read below are not in this REL:

| cell | artifact | value |
|---|---|---|
| `0x80429E00` | `main.elf` BSS (`0x80429BC0`, filesz 0) | zero-initialised pointer cell |
| `0x8042A1A0` | `main.elf` BSS | zero-initialised pointer cell |
| `0x80429D60` | `main.elf` BSS | zero-initialised pointer cell |
| `0x8042A008` | `main.elf` BSS | zero-initialised pointer cell |
| `0x8042A27C` | `main.elf` BSS | zero-initialised pointer cell |
| `0x80377DEC` | `main.elf` BSS (`0x80351680`, filesz 0) | zero-initialised 3-float cell |

None has shipped bytes in either artifact; each must be cited as an address and
read through `ptr_at(...)` / `read_f32(...)`, never as a literal. Everything
else was read from the shipped `d_basesNP` image and is listed with its address.

**The `0x43300000` ladder, as stored.** Six of the twelve contain the
`lis 0x4330` / `stw` / `lfd` / `fsubs` sequence. The two `lfd` operands are
8-byte cells (`0x43300000_<x ^ 0x80000000>` on the stack and
`0x4330000080000000` = 2^52 + 2^31 in the pool), so the arithmetic is the signed
integer -> float conversion regardless of the `fsubs` opcode field (59) that the
encoding uses. Deploy it exactly the way `src/game/bases.rs` already does:

```rust
let converted = (f64::from_bits(0x4330_0000_0000_0000 | (x as u64 ^ 0x8000_0000))
                 - read_f64(0x807C_XXXX)) as f32;
```

That exact shape is already merged in the file (for example
`src/game/bases.rs:32560`, `:39723`, `:111076`, `:120718`), so a lane can copy a
merged neighbour instead of inventing the conversion.

**House style used below.** `pub unsafe fn fn_XXXXXXXX(this: usize)`,
`platform::read_u32/read_i32/read_f32/read_u8/read_u16`,
`platform::write_*`, `abi_call1!..abi_call10!`, `vmethod0(rcv, slot)`,
`ptr_at(addr)`, addresses written `0x8060_8DF0`, field offsets as `usize`
literals, `// unknown:` on the first use of every address-only target or
external cell.

---

## 1. `0x806FB540` - 200 bytes - `work/ghidra/rel-d_basesNP/export/806fb540_FUN_806fb540.c`

**Body:** `0x806FB540..0x806FB604` (50 words, `blr` at `0x806FB604`).
Ledger: absent (zero rows). `src/game/bases.rs`: no definition, five
doc-comment mentions. Export warning: one "removing unreachable block
(ram,0x806fb5ec)" - that is the `psq_l`/`lfd` restore pair, spurious.

```rust
pub unsafe fn fn_806fb540(this: usize, code: i32, value: f32) -> f32
```

**Argument map.** `r3 -> this` (`mr r30,r3` @`0x806FB568`; the body reads only
the `u16` at `this+0x8B8`), `r4 -> code` (`mr r3,r4` @`0x806FB56C`, the only
argument of the first call), `f1 -> value` (`fmr f31,f1` @`0x806FB554`).

**Frame.** `stwu r1,-48(r1)`; `mfspr r0,LR` / `stw r0,52(r1)`;
`stfd f31,32(r1)` + `psq_st f31,40(r1),w0,qr0`; `stw r31,28` / `r30,24`.
Epilogue `0x806FB5E8..0x806FB604`.

**Shape.** A damped "lean" factor: clamp an absolute code to a half-period, turn
it into `0..1`, scale it by the incoming `value`, and add or subtract it from
`1.0` depending on a per-instance flag.

1. `0x806FB55C-0x806FB570` - `r31 = 0x807CC380` (`lis r31,0x807D` @`0x806FB55C`
   + `addi r31,r31,-0x3C80` @`0x806FB560`); `r30 = this`;
   `r3 = code`; `bl 0x802DE514` -> `|(i32)code|` (the callee is literally
   `((int)x >> 31 ^ x) - ((int)x >> 31)`).
2. `0x806FB574-0x806FB58C` - the half-period fold:
   ```
   s = (i16)abs_result;                    // extsh r4,r3
   if (s > 0x4000) s = (i16)(0x8000 - s);  // cmpi / ble / lis 1 / addi -0x8000 / subf / extsh
   ```
   The branch is `ble 0x806FB590` (`bc 4,1`), i.e. the fold runs only for
   `s > 16384`.
3. `0x806FB590-0x806FB5B4` - `s` -> float through the pool ladder:
   `xoris r3,r4,0x8000`; `lis r0,0x4330` / `stw r0,8(r1)`; `lfd f1,104(r31)`
   (= `0x4330000080000000`); `stw r3,12(r1)`; `lfd f0,8(r1)`;
   `fsubs f3,f0,f1`. `f3 = (f32)s`.
   In the same window: `lhz r0,0x8B8(r30)` (the flag) and `cmpi cr0,r0,0`
   (**`lhz`**, not `lha`).
4. `0x806FB5B8-0x806FB5CC` - the scale chain, in this order:
   ```
   x = 0.1f * f3;          // lfs f2,92(r31)=0.1 ; fmuls f2,f2,f3
   x = x / 16384.0f;       // lfs f1,96(r31)=16384 ; fdivs f1,f2,f1
   x = value * x;          // fmuls f1,f31,f1
   x = x * 0.0625f;        // lfs f0,100(r31) ; fmuls f1,f1,f0
   ```
5. `0x806FB5D0-0x806FB5E4` - `bne 0x806FB5E0` on the flag:
   ```
   if (*(u16*)(this+0x8B8) == 0)  result = 1.0f - x;   // lfs f0,52(r31)=1.0 ; fsubs
   else                          result = 1.0f + x;    // fadds
   ```
6. `0x806FB5E8-0x806FB604` - epilogue, `f1 = result`.

**Callees:**

| site | callee | arguments |
|---|---|---|
| `0x806FB570` | `0x802DE514` (main.dol, 16 B) | `r3 = code` -> `i32` = `abs(code)` |

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 1.0` | `0x807CC3B4` | `lfs f0,52(r31)` @`0x806FB5D4` / `@0x806FB5E0` |
| `f32 0.1` | `0x807CC3DC` | `lfs f2,92(r31)` @`0x806FB5AC` |
| `f32 16384.0` | `0x807CC3E0` | `lfs f1,96(r31)` @`0x806FB5B8` |
| `f32 0.0625` | `0x807CC3E4` | `lfs f0,100(r31)` @`0x806FB5BC` |
| `f64 2^52 + 2^31` = `0x4330000080000000` | `0x807CC3E8` | `lfd f1,104(r31)` @`0x806FB59C` |
| half-period `0x4000` / bias `0x8000` | immediate | `cmpi cr0,r4,0x4000` @`0x806FB578`, `lis r3,1`+`addi r0,r3,-0x8000` @`0x806FB580` |

**Confidence: high.** All 50 words decode, every operand is explicit, and the
export agrees field-for-field (`sVar2`, the `0x4000` fold, `1.0 -/+ fVar1`). The
only external is a 16-byte `abs` helper that was re-decoded from `main.elf`.
**Verdict: ready.**

---

## 2. `0x80725170` - 196 bytes - `work/ghidra/rel-d_basesNP/export/80725170_FUN_80725170.c`

**Body:** `0x80725170..0x80725230` (49 words, `blr` at `0x80725230`).
Ledger: absent. `src/game/bases.rs`: no definition, zero mentions. Export
warning: one "removing unreachable block (ram,0x80725214)" - the restore pair,
spurious.

```rust
pub unsafe fn fn_80725170(this: usize)
```

**Argument map.** `r3 -> this` (`mr r30,r3` @`0x80725194`); no other arguments
and no result (Ghidra's `void FUN_80725170(int param_1)` is right).

**Frame.** `stwu r1,-32(r1)`; `mfspr r0,LR` / `stw r0,36(r1)`;
`stfd f31,16(r1)` + `psq_st f31,24(r1),w0,qr0`; `stw r31,12` / `r30,8`.
Epilogue `0x80725214..0x80725230`.

**Shape.** A ride/attachment pose reset: look up the player object, compute a
height from a per-character table plus `0.75 * object->+0xE8`, clear one
sub-object field, then stamp a fixed rotation, two flag words and a counter.

1. `0x80725188-0x80725190` - `r31 = 0x807CD188` (`lis r31,0x807D` +
   `addi r31,r31,-0x2E78`); `r30 = this`.
2. `0x80725198-0x807251A0` - `r3 = (i8)*(u8*)(this+0x38D)` (`lbz` + `extsb`);
   `bl 0x8005F900` -> object pointer. The callee is a two-step lookup:
   `0x8005F900(id)` tail-calls `0x80162D00((&DAT_80354E10)[id])`, which returns
   `*(node+8)`.
3. `0x807251A4-0x807251C4` - `idx = *(u8*)(this+0x348)`;
   `f0 = *(f32*)(0x807CD208 + 4*idx)` (`rlwinm r0,r0,2,0,29` + `lfsx f0,r4,r0`,
   `r4 = r31+0x80`); `f2 = 0.75`; `f1 = *(f32*)(obj+0xE8)`;
   `f31 = f0 + 0.75f * f1` (`fmuls f1,f2,f1` @`0x807251C0`,
   `fadds f31,f0,f1` @`0x807251C4`).
4. `0x807251C8` - `bl 0x80065CB0` with `r3 = this` -> `*(u32*)(this+0x134) = 0`
   (the callee is literally `li r0,0` + `stw r0,0x134(r3)` + `blr`).
5. `0x807251CC-0x80725210` - the writes, all on `this`:
   ```
   *(u32*)(this+0x59C) |= 0x1000;      // lwz / ori 0x1000 / stw
   *(f32*)(this+0x114)  = -0.2f;
   *(f32*)(this+0xE8)   = f31;
   *(f32*)(this+0xEC)   = -1.5f;
   *(f32*)(this+0xF0)   = 0.0f;
   *(u32*)(this+0x488) |= 0x00900000;  // oris r4,r4,0x90
   *(u32*)(this+0x494) |= 0x00900000;  // oris r3,r3,0x90
   *(u32*)(this+0x4BC)  = 0;
   ```
6. `0x80725214-0x80725230` - epilogue.

**Callees:**

| site | callee | arguments |
|---|---|---|
| `0x807251A0` | `0x8005F900` (main.dol, 20 B) | `r3 = (i8)this[0x38D]` -> object pointer |
| `0x807251C8` | `0x80065CB0` (main.dol, 12 B) | `r3 = this` -> clears `+0x134` |

**Constants:**

| value | image address | materialised at |
|---|---|---|
| float table base (idx = `this+0x348`) | `0x807CD208` | `addi r4,r31,0x80` @`0x807251A8` + `lfsx` @`0x807251BC` |
| `[0]` = `2.5`, `[1]` = `-2.5`, `[2]` = `0.75`, `[3]` = `-1.5` | `0x807CD208/0C/10/14` | read at `0x807251BC` |
| `f32 0.75` (multiplier) | `0x807CD210` | `lfs f2,136(r31)` @`0x807251B4` |
| `f32 -0.2` | `0x807CD200` | `lfs f2,120(r31)` @`0x807251E0` |
| `f32 -1.5` | `0x807CD214` | `lfs f1,140(r31)` @`0x807251E8` |
| `f32 0.0` | `0x807CD1BC` | `lfs f0,52(r31)` @`0x807251F0` |
| flags `0x1000`, `0x00900000` | immediate | `ori` @`0x807251DC`, `oris` @`0x807251E4` / `@0x807251EC` |

The multiplier cell `0x807CD210` is the *second* float of the table block - the
compiler reused it; there is no separate constant.

**Confidence: high.** All 49 words decode; the table read, the `0.75 * obj+0xE8`
product and all eight stores are literal, and the export matches (its
`dVar5 = ... + DAT_807cd210 * *(float *)(iVar4 + 0xe8)`). **Verdict: ready.**

---

## 3. `0x8069A170` - 184 bytes - `work/ghidra/rel-d_basesNP/export/8069a170_FUN_8069a170.c`

**Body:** `0x8069A170..0x8069A224` (46 words, `blr` at `0x8069A224`).
Ledger: absent. `src/game/bases.rs`: no definition, zero mentions. Export
warning: one "removing unreachable block (ram,0x8069a210)" - the restore pair,
spurious.

```rust
pub unsafe fn fn_8069a170(this: usize) -> u32
```

**Argument map.** `r3 -> this` (never copied; read directly). Result in `r3`.

**Frame.** `stwu r1,-48(r1)`; `mfspr r0,LR` / `stw r0,52(r1)`;
`stfd f31,32(r1)` + `psq_st f31,40(r1),w0,qr0`; `stw r31,28`. Epilogue
`0x8069A20C..0x8069A224`.

**Shape.** Sector classifier: take the XZ difference between two points,
normalise it, and return `1` when the Y component is inside the `±pi/8` band,
`0` when it is above that band, `2` when below.

1. `0x8069A188-0x8069A1A0` - `r31 = 0x807CA078` (`lis r31,0x807D` +
   `addi r31,r31,-0x5F88`).
2. `0x8069A190-0x8069A1B8` - the difference vector at `r1+8`:
   ```
   [0] = *(f32*)(this+0x768) - *(f32*)(this+0xAC);   // f2 - f1 -> 8(r1)
   [1] = *(f32*)(this+0x76C) - *(f32*)(this+0xB0);   // f3 - f0 -> 12(r1)
   [2] = 0.0f;                                       // lfs f0,20(r31) -> 16(r1)
   ```
   Note **both** subtractions are `target - source`: the first operand is the
   `+0x768/+0x76C` pair, the second the `+0xAC/+0xB0` pair.
3. `0x8069A1BC` - `bl 0x80170980(r1+8)`: an in-place 3-float normalise
   (the callee loads `FUN_801c14c0`, and if `|len|` exceeds a threshold scales
   all three components by `1/len`).
4. `0x8069A1C0-0x8069A1D0` - two angular probes:
   ```
   f31 = 0x802BE4A0(+0.39269909f);   // lfs f1,68(r31) ; bl ; fmr f31,f1
   f1  = 0x802BE4A0(-0.39269909f);   // lfs f1,72(r31) ; bl
   ```
   The callee is `(float)FUN_802E84C4()` - a single-argument float function of
   the angle; with `±pi/8` and an *increasing* result the band becomes
   `[-0.3826834, +0.3826834]`, i.e. `sinf`. Emit
   `// inferred: 0x802BE4A0(f32) -> f32 is sinf` and call it with the literal
   constants; do not open-code the sine.
5. `0x8069A1D4-0x8069A1EC` - the band test:
   ```
   dy = *(f32*)(r1+0xC);
   if (f1 >= dy) goto L;        // bge 0x8069A1F0
   if (dy >= f31) goto L;       // bge 0x8069A1F0
   return 1;
   ```
6. `0x8069A1F0-0x8069A208` - the out-of-band arms:
   ```
   L: if (dy >= 0.0f) return 0;  // bge 0x8069A208 (li r3,0)
      return 2;                  // li r3,2
   ```
7. `0x8069A20C-0x8069A224` - epilogue.

**Callees:**

| site | callee | arguments |
|---|---|---|
| `0x8069A1BC` | `0x80170980` (main.dol, 120 B) | `r3 = &vec3` -> normalises in place |
| `0x8069A1C4` | `0x802BE4A0` (main.dol, 36 B) | `f1 = angle` -> `f32` |
| `0x8069A1D0` | `0x802BE4A0` | `f1 = -angle` -> `f32` |

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 0.0` (vector Z) | `0x807CA08C` | `lfs f0,20(r31)` @`0x8069A1A4` |
| `f32 +0.39269909` (`pi/8`) | `0x807CA0BC` | `lfs f1,68(r31)` @`0x8069A1C0` |
| `f32 -0.39269909` (`-pi/8`) | `0x807CA0C0` | `lfs f1,72(r31)` @`0x8069A1CC` |
| sector ids `0/1/2` | immediate | `li r3,1` @`0x8069A1E8`, `li r3,2` @`0x8069A200`, `li r3,0` @`0x8069A208` |

**Confidence: medium.** Every branch, both subtraction pairs and the three
return values are literal and match the export
(`if ((double)local_24 <= dVar3) || (dVar2 <= (double)local_24)` plus the
`DAT_807ca08c <= local_24` split). The only inference is the identity of
`0x802BE4A0` as the sine; the emitted Rust does not depend on that (it calls the
helper and compares), only the `// inferred:` comment does. **Verdict: ready.**

---

## 4. `0x806A8AA0` - 184 bytes - `work/ghidra/rel-d_basesNP/export/806a8aa0_FUN_806a8aa0.c`

**Body:** `0x806A8AA0..0x806A8B54` (46 words, `blr` at `0x806A8B54`).
Ledger: absent. `src/game/bases.rs`: no definition, three doc-comment mentions.
Export warning: one "removing unreachable block (ram,0x806a8b34)" - the restore
pair, spurious.

```rust
pub unsafe fn fn_806a8aa0(out: usize, owner: usize, position: usize)
```

**Argument map.** `r3 -> out` (`mr r29,r3` @`0x806A8AD0`; three `stfs` at
`0/4/8(r29)`), `r4 -> owner` (`mr r30,r4` @`0x806A8AC4`; re-loaded into `r3` at
`0x806A8AD8`), `r5 -> position` (`mr r4,r5` @`0x806A8AE0`; the second argument
of the call).

**Frame.** `stwu r1,-48(r1)`; `mfspr r0,LR` / `stw r0,52(r1)`;
`stfd f31,32(r1)` + `psq_st f31,40(r1),w0,qr0`; `stw r31,28` / `r30,24` /
`r29,20`. Epilogue `0x806A8B34..0x806A8B54`.

**Shape.** Build an aim point from an actor: a 224-unit offset plus a
half-scaled height, then a `(count * 5500)` advance along the actor's forward
scalar.

1. `0x806A8AB8-0x806A8AF0` - the base point:
   ```
   f2 = 0.5f;  f0 = 224.0f;
   f3 = *(f32*)(owner+0x424);      // lfs f3,1060(r4)
   f1 = *(f32*)(owner+0x414);      // lfs f1,1044(r30)
   f2 = f3 * 0.5f;                 // fmuls f2,f3,f2
   f0 = f1 - 224.0f;               // fsubs f0,f1,f0
   f31 = f2 + f0;                  // fadds f31,f2,f0
   ```
2. `0x806A8AF4` - `bl 0x806a8a40(owner, position)` -> `i32` count
   (already translated as `fn_806a8a40(owner, position) -> i32`).
3. `0x806A8AF8-0x806A8B18` - the count -> float ladder:
   ```
   xoris r3,r3,0x8000;  lis r0,0x4330;  stw r3,12(r1);  stw r0,8(r1);
   lfd f2,8(r1);        lfd f3,88(r31);   fsubs f2,f2,f3;   // f2 = (f32)count
   ```
4. `0x806A8B0C-0x806A8B24` - `out[0]`:
   ```
   f1 = *(f32*)(owner+0x424);   // lfs f1,1060(r30)
   f1 = f1 * f2;                // fmuls f1,f1,f2
   f1 = f31 + f1;               // fadds f1,f31,f1
   *(f32*)(out+0) = f1;         // stfs f1,0(r29)
   ```
5. `0x806A8B28-0x806A8B30` - `out[1] = *(f32*)(owner+0x420)`;
   `out[2] = 5500.0f` (`lfs f0,104(r31)` @`0x806A8B14`, `stfs f0,8(r29)`).
6. `0x806A8B34-0x806A8B54` - epilogue.

**Callees:**

| site | callee | arguments |
|---|---|---|
| `0x806A8AF4` | `0x806A8A40` (this REL, 92 B, **already translated**) | `r3 = owner`, `r4 = position` -> `i32` |

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f64 2^52 + 2^31` = `0x4330000080000000` | `0x807CA4C8` | `lfd f3,88(r31)` @`0x806A8B04` |
| `f32 0.5` | `0x807CA4D0` | `lfs f2,96(r31)` @`0x806A8AC8` |
| `f32 224.0` | `0x807CA4D4` | `lfs f0,100(r31)` @`0x806A8AD4` |
| `f32 5500.0` | `0x807CA4D8` | `lfs f0,104(r31)` @`0x806A8B14` |

**Confidence: medium, and conditional on one reading.** The multiply at
`0x806A8B1C` is `fmuls f1,f1,f2` where `f2` was produced by the
`0x43300000` ladder; the export reads it as
`*(float *)(owner + 0x424) * (float)((double)CONCAT44(...) - DAT_807ca4c8)`,
i.e. a multiply by the **integer count**, and that is the only reading under
which the function is coherent (the dead `fsubs` chain has no other consumer).
A lane must implement it as
`read_f32(owner+0x424) * (count as f32)` and keep the `// inferred:` note on the
register-class mismatch; do not "fix" it by dropping the multiply. Everything
else is literal. **Verdict: conditional.**

---

## 5. `0x80761880` - 176 bytes - `work/ghidra/rel-d_basesNP/export/80761880_FUN_80761880.c`

**Body:** `0x80761880..0x8076192C` (44 words, `blr` at `0x8076192C`).
Ledger: absent. `src/game/bases.rs`: no definition, four doc-comment mentions.
Export warning: one "removing unreachable block (ram,0x80761918)" - the restore
pair, spurious.

```rust
pub unsafe fn fn_80761880(this: usize)
```

**Argument map.** `r3 -> this` (`mr r31,r3` @`0x80761898`). No result.

**Frame.** `stwu r1,-32(r1)`; `mfspr r0,LR` / `stw r0,36(r1)`;
`stfd f31,16(r1)` + `psq_st f31,24(r1),w0,qr0`; `stw r31,12`. Epilogue
`0x80761914..0x8076192C`.

**Shape.** A deferred sound/stat gate: only while the `+0x250` byte is set,
either clear it unconditionally (when a helper reports "idle") or clear it once
a second helper's timer has expired.

1. `0x8076189C-0x807618A4` - `if (*(u8*)(this+0x250) == 0) goto epilogue;`
   (`lbz` / `cmpi` / `beq 0x80761914`).
2. `0x807618A8` - `r3 = fn_80761480()` (already translated, `-> usize`), no
   arguments.
3. `0x807618AC-0x807618B0` - `if (r3 == 0) goto arm A;`
   (`cmpi cr0,r3,0` / `beq 0x807618F4`).
4. `0x807618B4-0x807618F0` - the "busy" arm:
   ```
   f31 = 0x80761670();                    // bl, f1 -> f31
   f1  = 0x801662E0(this+0x20C, 0);       // addi r3,r31,0x20C ; li r4,0 ; bl
   if (f1 != f31) goto epilogue;          // fcmpu cr0,f1,f31 ; bne 0x80761914
   // equal -> fall through into the reset
   ```
5. `0x807618F4-0x80761910` and the shared tail `0x807618D0-0x807618F0` - the
   reset, identical in both arms:
   ```
   f1 = 1.0f;                             // lfs f1,4(r5), r5 = 0x807CEDB0
   0x80166310(this+0x20C, 0);             // r3 = this+0x20C, r4 = 0, f1 = 1.0
   *(u8*)(this+0x250) = 0;                // li r0,0 ; stb r0,0x250(r31)
   ```
6. `0x80761914-0x8076192C` - epilogue.

`0x80761670` is **not** in the ledger and has no Rust body. It is a 12-byte tail
forwarder, decoded here from the REL:

```
0x80761670  li r4,0
0x80761674  addi r3,r3,0x20C
0x80761678  b 0x801662E0          ; tail call, no epilogue
```

so `f31 = 0x801662E0(this+0x20C, 0)` as well. Ghidra types `0x80761670` as
`double FUN_80761670(void)`; the float comes back in `f1` from
`0x80168080`/`0x801680E0` (both 16-byte wrappers around
`FUN_80168080(*(p+0x28) + idx*0x38)` / `FUN_801680E0(...)`, typed `void` by
Ghidra).

**Callees:**

| site | callee | arguments |
|---|---|---|
| `0x807618A8` | `0x80761480` (this REL, 144 B, **already translated**) | none -> `usize` |
| `0x807618B4` | `0x80761670` (this REL, 12 B, **not translated**) | `r3 = this` -> `f32` in `f1` |
| `0x807618C4` | `0x801662E0` (main.dol, 16 B) | `r3 = this+0x20C`, `r4 = 0` -> `f32` in `f1` |
| `0x807618E4` / `0x80761908` | `0x80166310` (main.dol, 16 B) | `f1 = 1.0`, `r3 = this+0x20C`, `r4 = 0` |

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 1.0` | `0x807CEDB4` | `lfs f1,4(r5)` @`0x807618E0` / `@0x80761904` (`r5 = 0x807CEDB0`) |
| `f32 0.0` | `0x807CEDB0` | pool base of the same block (not read here) |
| byte offset `0x250` | immediate | `lbz` @`0x8076189C`, `stb` @`0x807618EC` / `@0x80761910` |

**Confidence: medium.** The guard, both arms and the equality test are literal
and match the export one-for-one
(`if (dVar3 == dVar2) { func_0x80166310(...); *(undefined1 *)(param_1 + 0x250) = 0; }`).
The two inferences are (a) that the two `void`-typed 16-byte wrappers return a
float in `f1` - which the `fcmpu` proves - and (b) the body of the untranslated
`0x80761670` - decoded above. **Verdict: ready.**

---

## 6. `0x80656260` - 172 bytes - `work/ghidra/rel-d_basesNP/export/80656260_FUN_80656260.c`

**Body:** `0x80656260..0x80656308` (43 words, `blr` at `0x80656308`).
Ledger: absent. `src/game/bases.rs`: no definition, one doc-comment mention.
Export warnings: one "removing unreachable block (ram,0x806562f0)" (restore
pair, spurious) and `Globals starting with '_' overlap smaller symbols` (the
`0x80429E00` cell).

```rust
pub unsafe fn fn_80656260(this: usize) -> f32
```

**Argument map.** `r3 -> this` (never copied; forwarded untouched as the first
argument of the call). Result in `f1`.

**Frame.** `stwu r1,-48(r1)`; `mfspr r0,LR` / `stw r0,52(r1)`;
`stfd f31,32(r1)` + `psq_st f31,40(r1),w0,qr0`; `stw r31,28`. Epilogue
`0x806562F0..0x80656308`.

**Shape.** A height error at a sample offset: take a global timer's height minus
16, quantise the absolute difference to a 16-unit step, hand that to a
sub-stepper, and return the residual height.

1. `0x80656268-0x80656294` - the globals:
   ```
   r4 = *(u32*)0x80429E00;        // lis r4,0x8043 ; lwz r4,-25088(r4)
   r31 = 0x807C88F8;              // lis r31,0x807D ; addi r31,r31,-0x7708
   f0 = 16.0f;                    // lfs f0,4(r31)
   f1 = *(f32*)(r4+0x3C);         // lfs f1,60(r4)
   f2 = *(f32*)(r4+0x44);         // lfs f2,68(r4)
   f31 = f1 - 16.0f;              // fsubs f31,f1,f0
   ```
2. `0x80656298-0x806562C0` - the sample argument, kept literal:
   ```
   f1 = 32.0f;                    // lfs f1,0(r31)
   f0 = 0.0625f;                  // lfs f0,8(r31)
   f2 = f31 - f2;
   f1 = f1 + f2;
   f1 = f1 - f31;                 // == 32.0f - *(f32*)(p+0x44) after folding
   f1 = fabs(f1);                 // fabs + frsp
   f0 = f1 * 0.0625f;
   fctiwz f0,f0;  stfd f0,8(r1);  r4 = *(i32*)(r1+12);
   ```
   The three-instruction fold (`+32`, `- f2`, `- f31`) is exact in single
   precision and must be emitted in that order; do not pre-simplify.
3. `0x806562C4` - `bl 0x806561B0(this, r4)` -> `i32`
   (already translated as `fn_806561b0(actor: usize, sample: u32) -> i32`).
4. `0x806562C8-0x806562EC` - the return ladder:
   ```
   xoris r3,r3,0x8000;  lis r0,0x4330;  stw r3,20(r1);  stw r0,16(r1);
   lfd f2,16(r31);      // 0x4330000080000000
   lfs f0,4(r31);       // 16.0f
   lfd f1,16(r1);  fsubs f1,f1,f2;      // f1 = (f32)ret
   f0 = f0 * f1;                        // fmuls f0,f0,f1
   f1 = f31 - f0;                       // fsubs f1,f31,f0
   ```
5. `0x806562F0-0x80656308` - epilogue.

**Callees:**

| site | callee | arguments |
|---|---|---|
| `0x806562C4` | `0x806561B0` (this REL, 92 B, **already translated**) | `r3 = this`, `r4 = sample` -> `i32` |

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 32.0` | `0x807C88F8` | `lfs f1,0(r31)` @`0x80656298` |
| `f32 16.0` | `0x807C88FC` | `lfs f0,4(r31)` @`0x80656288` / `@0x806562DC` |
| `f32 0.0625` | `0x807C8900` | `lfs f0,8(r31)` @`0x8065629C` |
| `f64 2^52 + 2^31` = `0x4330000080000000` | `0x807C8908` | `lfd f2,16(r31)` @`0x806562D4` |
| pointer cell to the timer object | `0x80429E00` (main.dol BSS) | `lwz r4,-25088(r4)` @`0x80656284` |

**Confidence: medium, conditional on the external cell.** The body is fully
decoded and the export agrees (`dVar2 = *(float *)(_DAT_80429e00 + 0x3c) - DAT_807c88fc`,
`return (float)(dVar2 - DAT_807c88fc * (float)(...))`). What is *not* in any
shipped artifact is the object behind `0x80429E00`: it is BSS (filesz 0) and
only its two float fields `+0x3C` and `+0x44` are visible from this caller. A
lane must write `let p = ptr_at(0x8042_9E00);` with
`// unknown: timer object behind 0x80429E00 (BSS); +0x3C/+0x44 are f32 heights`
and must not invent field names. **Verdict: conditional.**

---

## 7. `0x807481B0` - 172 bytes - `work/ghidra/rel-d_basesNP/export/807481b0_FUN_807481b0.c`

**Body:** `0x807481B0..0x80748258` (43 words, `blr` at `0x80748258`).
Ledger: absent. `src/game/bases.rs`: no definition, six doc-comment mentions.
Export warnings: none.

```rust
pub unsafe fn fn_807481b0(this: usize, frames: i32)
```

**Argument map.** `r3 -> this` (`mr r27,r3` @`0x807481C8`; used for every field
access, `+0x394/+0x3A0/+0x3A4`), `r4 -> frames` (`mr r28,r4` @`0x807481CC`;
the loop counter **and** the second argument of the respawn call). Ghidra's
signature is `void FUN_807481b0(void)` because it models `_savegpr` as returning
an `undefined8` instead of taking `r3/r4`; its *body* still splits the pair the
same way (`(int)(uVar7 >> 32)` for the object, `(int)uVar7` for the counter).
The raw stream settles the mapping: the object base is `r3`, the counter `r4`.

**Frame.** `stwu r1,-32(r1)`; `mfspr r0,LR` / `stw r0,36(r1)`;
`addi r11,r1,32`; `bl 0x802DCD74` (saves `r27..r31` at `-20..-4(r11)`).
Epilogue `0x80748244-0x80748258`: `addi r11,r1,32` / `bl 0x802DCDC0` then the
LR/frame restore. **No `psq`, no float work.**

**Shape.** A fixed-step "advance N frames" driver for a 12-slot spawn ring: each
step decrements a per-actor countdown, and when it hits zero spawns one actor
from the next ring slot and reloads the countdown with 256.

1. `0x807481C4-0x807481D8` - `r30 = 0x80831750` (`lis r30,0x8083` +
   `addi r30,r30,0x1750`); `r31 = 256`; `b 0x80748238` (jump to the test).
2. `0x80748238-0x80748240` - the outer test:
   ```
   cmpi cr0,frames,0 ; addi frames,frames,-1 ; bne 0x807481DC
   ```
   `bne` means the body runs while the *pre-decrement* value is non-zero, so
   `frames` inside the body is already decremented (first iteration `n-1`, last
   `0`). Matches the export's `bVar1 = iVar5 == 0; iVar5 += -1; if (bVar1) break;`.
3. `0x807481DC-0x807481EC` - the countdown:
   ```
   n = *(u16*)(this+0x3A4);           // lhz r3,932(r27)
   *(u16*)(this+0x3A4) = n - 1;       // addi + sth
   if ((u16)(n - 1) != 0) goto test;  // rlwinm r0,r0,0,16,31. ; bne 0x80748238
   ```
4. `0x807481F0-0x80748208` - the spawn, only when the countdown expired:
   ```
   slot = *(u32*)(this+0x3A0);        // lwz r29,928(r27)
   actor = 0x80064610(455, *(u32*)slot, this+0x394, 0, 0);
   ```
   (`li r3,455` = `0x1C7`; `r4 = *(u32*)r29`; `r5 = this+0x394`; `r6 = r7 = 0`.)
5. `0x8074820C-0x80748218` - `if (actor != 0) fn_80691f90(actor, frames);`
   (`cmpi cr0,r3,0` / `beq` / `mr r4,r28` / `bl 0x80691f90`). The frame count is
   the already-decremented `frames`.
6. `0x8074821C-0x80748234` - the ring advance:
   ```
   next = slot + 4;                   // addi r3,r29,4
   if (next == 0x80831780) next = 0x80831750;   // cmpl with r30+0x30, beq
   *(u32*)(this+0x3A0) = next;
   *(u16*)(this+0x3A4) = 256;
   ```
   `0x80831750 + 12*4 == 0x80831780`, i.e. twelve `u32` slots.
7. `0x80748244-0x80748258` - epilogue.

**Callees:**

| site | callee | arguments |
|---|---|---|
| `0x807481C0` | `0x802DCD74` (main.dol, 24 B) | saves `r27..r31` from `r11` |
| `0x80748208` | `0x80064610` (main.dol, 104 B) | `r3 = 455`, `r4 = *slot`, `r5 = this+0x394`, `r6 = 0`, `r7 = 0` -> actor |
| `0x80748218` | `0x80691F90` (this REL, 104 B, **already translated**) | `r3 = actor`, `r4 = frames` (`fn_80691f90(actor, frames)`) |
| `0x80748248` | `0x802DCDC0` (main.dol, 24 B) | reloads `r27..r31` from `r11` |

`0x80064610` is itself a two-call wrapper: `FUN_80064600(r7)` then
`FUN_8006ca80(r3, r4, r5, r6)`; keep it as an `abi_call5!`.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| spawn id `455` | immediate | `li r3,0x1C7` @`0x807481F8` |
| countdown reload `256` | immediate | `li r31,0x100` @`0x807481D0`, `sth r31,932(r27)` @`0x80748234` |
| ring base | `0x80831750` | `addi r30,r30,0x1750` @`0x807481D4` |
| ring end (exclusive) | `0x80831780` | `addi r0,r30,48` @`0x8074821C` |
| slot stride `4` | immediate | `addi r3,r29,4` @`0x80748220` |

**Confidence: medium.** Every field, the counter semantics and the ring wrap are
literal, and the raw stream resolves the argument order that Ghidra inverts. The
only inference is the meaning of spawn id `0x1C7` and of the `+0x394` object,
which are passed straight through. **Verdict: ready.**

---

## 8. `0x80702CE0` - 168 bytes - `work/ghidra/rel-d_basesNP/export/80702ce0_FUN_80702ce0.c`

**Body:** `0x80702CE0..0x80702D84` (42 words, `blr` at `0x80702D84`).
Ledger: absent. `src/game/bases.rs`: no definition, one doc-comment mention.
Export warning: one "removing unreachable block (ram,0x80702d64)" - the restore
pair, spurious.

```rust
pub unsafe fn fn_80702ce0(this: usize)
```

**Argument map.** `r3 -> this` (`mr r28,r3` @`0x80702D20`; `r29 = this+0x744`).

**Frame.** `stwu r1,-112(r1)`; `mfspr r0,LR` / `stw r0,116(r1)`;
`stfd f31,96(r1)` + `psq_st f31,104(r1),w0,qr0`; `stw r31,92` / `r30,88` /
`r29,84` / `r28,80`. Epilogue `0x80702D60..0x80702D84`.

**Shape.** Five fixed lookups (ids `1,4,5,6,7`) that rebuild five 0x90-byte light
records at `this+0x744`.

1. `0x80702CE8-0x80702D20` - setup:
   ```
   r4 = 0x807CC588;  f31 = *(f32*)(r4+0x28);   // = 100.0f, lfs f31,40(r4)
   r31 = 0x807CC57C;  r30 = r31 + 5;           // 0x807CC581
   r29 = this + 0x744; r28 = this;
   ```
2. `0x80702D24-0x80702D30` - per iteration:
   ```
   id = *(u8*)r31;                     // lbz r4,0(r31)
   0x80168180(this+0x478, id, r1+0x18);
   ```
3. `0x80702D34-0x80702D3C` - `0x8016F130(r1+0x18, r1+8)`: copies three words
   (`+0xC`, `+0x1C`, `+0x2C` of the record) into `r1+8/0xC/0x10`.
4. `0x80702D40-0x80702D4C` - `fn_80701c60(100.0f, r29, r1+8)`
   (`fmr f1,f31`; `r3 = r29`; `r4 = r1+8`).
5. `0x80702D50-0x80702D5C` - `r31 += 1; r29 += 0x90;`
   `if (r31 < 0x807CC581) goto 0x80702D24;` (`cmpl cr0,r31,r30` / `blt` -
   **unsigned** compare, five iterations).
6. `0x80702D60-0x80702D84` - epilogue.

**Callees:**

| site | callee | arguments |
|---|---|---|
| `0x80702D30` | `0x80168180` (main.dol, 168 B) | `r3 = this+0x478`, `r4 = id (u8)`, `r5 = &out[3]` |
| `0x80702D3C` | `0x8016F130` (main.dol, 28 B) | `r3 = &in[3]`, `r4 = &vec[3]` (copies `+0xC/+0x1C/+0x2C`) |
| `0x80702D4C` | `0x80701C60` (this REL, 44 B, **already translated**) | `f1 = 100.0`, `r3 = light`, `r4 = &vec` (`fn_80701c60(height, light, position)`) |

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 100.0` | `0x807CC5B0` | `lfs f31,40(r4)` @`0x80702CFC` (`r4 = 0x807CC588`) |
| id table, 5 bytes: `01 04 05 06 07` | `0x807CC57C..0x807CC580` | `lbz r4,0(r31)` @`0x80702D24` |
| light stride `0x90` | immediate | `addi r29,r29,0x90` @`0x80702D54` |

**Confidence: high.** All 42 words decode; the loop bound, the byte table, the
stride and the three calls are literal, and the export matches
(`do { ... } while (puVar2 < &DAT_807cc581);`). **Verdict: ready.**

---

## 9. `0x8078B120` - 168 bytes - `work/ghidra/rel-d_basesNP/export/8078b120_FUN_8078b120.c`

**Body:** `0x8078B120..0x8078B1C4` (42 words, `blr` at `0x8078B1C4`).
Ledger: absent. `src/game/bases.rs`: no definition, one doc-comment mention.
Export warnings: none.

```rust
pub unsafe fn fn_8078b120(this: usize, flag: u32)
```

**Argument map.** `r3 -> this` (`mr r29,r3` @`0x8078B150`; `this+0x60C`,
`+0x610`, `+0x44C`), `r4 -> flag` (negated/ored at `0x8078B12C-0x8078B130`).

**Frame.** `stwu r1,-32(r1)`; `mfspr r0,LR` / `stw r0,36(r1)`;
`stw r31,28` / `r30,24` / `r29,20`. **No `psq`, no float save.**

**Shape.** A broom mount/dismount selector: pick one of two animation names from
`flag`, play a 5-second clip, and if the slot handle comes back non-null, bind
the actor to a per-variant sub-object.

1. `0x8078B12C-0x8078B140` - the index:
   ```
   r0 = -flag;                 // neg r0,r4
   r0 = r0 | flag;             // or r0,r0,r4
   idx = (r0 >> 31) & 1;       // rlwinm r4,r0,1,31,31  -> 0 or 1
   ```
   `idx == 0` when `flag == 0`, `1` otherwise.
2. `0x8078B138-0x8078B154` - `r31 = 0x807D0000`; `f1 = 5.0f`
   (`lfs f1,-1180(r31)` = `0x807CFB64`); `bl 0x80789B80` with `r3 = this`,
   `r4 = idx` - i.e. `fn_80789b80(5.0, this, idx as i32)` (already translated,
   signature `(duration: f32, actor: usize, slot: i32)`).
3. `0x8078B158-0x8078B16C` - the name lookup:
   ```
   r4 = 0x80819658;            // lis r4,0x8082 ; addi r4,r4,-0x69A8
   name = *(u32*)(r4 + 4*idx);
   handle = 0x8023A0B0(this+0x60C, name);
   ```
   `0x80819658` holds `0x80819638 -> "ride_wait_broom"` and
   `0x80819648 -> "fly_broom"`, so `idx == 0` selects `"ride_wait_broom"`.
4. `0x8078B170-0x8078B1A8` - only when `handle != 0`:
   ```
   target = this + 0x44C + 56 * idx;    // mulli r0,r30,0x38 ; add r3,r29,r0 ; addi r30,r3,0x44C
   0x801651F0(target, this+0x610, &handle, 0);
   0x80169F80(5.0f, this+0x610, target);
   ```
   `&handle` is the `r1+8` slot written by `stw r3,8(r1)` @`0x8078B17C`.
5. `0x8078B1AC-0x8078B1C4` - epilogue.

**Callees:**

| site | callee | arguments |
|---|---|---|
| `0x8078B154` | `0x80789B80` (this REL, 224 B, **already translated**) | `f1 = 5.0`, `r3 = this`, `r4 = idx` |
| `0x8078B16C` | `0x8023A0B0` (main.dol, 108 B) | `r3 = this+0x60C`, `r4 = name` -> `i32` handle |
| `0x8078B198` | `0x801651F0` (main.dol, 132 B) | `r3 = target`, `r4 = this+0x610`, `r5 = &handle`, `r6 = 0` |
| `0x8078B1A8` | `0x80169F80` (main.dol, 116 B) | `f1 = 5.0`, `r3 = this+0x610`, `r4 = target` |

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 5.0` | `0x807CFB64` | `lfs f1,-1180(r31)` @`0x8078B13C` / `@0x8078B19C` |
| name table (2 pointers) | `0x80819658` | `addi r4,r4,-0x69A8` @`0x8078B160`, `lwzx r4,r4,r0` @`0x8078B168` |
| `"ride_wait_broom"` | `0x80819638` | `[0]` of the table |
| `"fly_broom"` | `0x80819648` | `[1]` of the table |
| variant stride `56` | immediate | `mulli r0,r30,0x38` @`0x8078B178` |

**Confidence: medium.** Every operand is explicit and the export agrees
(`iVar1 = (int)(-param_2 | param_2) >> 0x1f`,
`(&PTR_s_ride_wait_broom_80819658)[-iVar1]`, `+ iVar1 * -0x38 + 0x44c`). The
inferences are the animation-name semantics and the `+0x610`/`+0x44C` sub-object
pair; the `5.0` is passed straight to the already-merged `fn_80789b80`.
**Verdict: ready.**

---

## 10. `0x80781D10` - 152 bytes - `work/ghidra/rel-d_basesNP/export/80781d10_FUN_80781d10.c`

**Body:** `0x80781D10..0x80781DA4` (38 words, `blr` at `0x80781DA4`).
Ledger: absent. `src/game/bases.rs`: no definition, zero mentions. Export
warning: `Globals starting with '_' overlap smaller symbols at the same address`
(the `_DAT_...` cells).

```rust
pub unsafe fn fn_80781d10(this: usize, arg: usize)
```

**Argument map.** `r3 -> this` (`mr r30,r3` @`0x80781D34`), `r4 -> arg`
(`mr r31,r4` @`0x80781D28`; becomes `r4` of the `0x8077E410` call).

**Frame.** `stwu r1,-16(r1)`; `mfspr r0,LR` / `stw r0,20(r1)`;
`stw r31,12` / `r30,8`. Epilogue `0x80781D90..0x80781DA4`. **No `psq`.**

**Shape.** Spawn-state entry: copy three signed shorts out of a global transform,
orient the actor to a zero vector, start an effect, then push a 12-entry camera
parameter pair.

1. `0x80781D18-0x80781D50` - the three shorts:
   ```
   r6 = *(u32*)0x8042A1A0;        // lis r6,0x8043 ; lwz r6,-24160(r6)
   *(i16*)(this+0x1E4) = *(i16*)(r6+0x100);   // lha + sth
   *(i16*)(this+0x1E6) = *(i16*)(r6+0x102);
   *(i16*)(this+0x1E8) = *(i16*)(r6+0x104);
   ```
2. `0x80781D2C-0x80781D54` - `r4 = 0x80377DEC`; `bl 0x800F6410(this, r4)`.
   The callee is **not** in any Ghidra export; it was decoded from
   `work/dol/main.elf`: it copies three floats from `r4` to `r1+0x14`, calls
   `0x80170980` (normalise), then branches on `|x|`/`|z|` against a small epsilon
   (`lfs f0,-24784(r2)`) to pick an orientation arm. Treat it as
   `// unknown: 0x800F6410(this, &vec3) - orient from a direction vector`.
3. `0x80781D58-0x80781D6C` - `f1 = 0.0f` (`lfs f1,-2408(r4)`, `r4 = 0x807D0000`
   -> `0x807CF698`); `r3 = this`; `r4 = arg`; `r5 = -1`;
   `bl 0x8077E410` - i.e. `fn_8077e410(0.0, this, arg, -1)`
   (signature from the export `(undefined8, int, int, int)`, the first argument
   being the float in `f1`).
4. `0x80781D70-0x80781D8C` - the camera pair:
   ```
   e = *(u32*)(this+0x1CC);
   s = *(u32*)(e+4);
   0x80104FD0(*(u32*)0x8042A27C, 12, *(u8*)(s+0x151), *(u8*)(s+0x152));
   ```
5. `0x80781D90-0x80781DA4` - epilogue.

**Callees:**

| site | callee | arguments |
|---|---|---|
| `0x80781D54` | `0x800F6410` (main.dol, **no export row**) | `r3 = this`, `r4 = 0x80377DEC` |
| `0x80781D6C` | `0x8077E410` (this REL, 344 B, **not translated**) | `f1 = 0.0`, `r3 = this`, `r4 = arg`, `r5 = -1` |
| `0x80781D8C` | `0x80104FD0` (main.dol, 132 B) | `r3 = *(0x8042A27C)`, `r4 = 12`, `r5 = s[0x151]`, `r6 = s[0x152]` |

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 0.0` | `0x807CF698` | `lfs f1,-2408(r4)` @`0x80781D60` (`r4 = 0x807D0000`) |
| direction cell (3 floats, BSS) | `0x80377DEC` | `lis r5,0x8037` + `addi r4,r5,0x7DEC` @`0x80781D1C`/`@0x80781D2C` |
| transform source cell | `0x8042A1A0` (main.dol BSS) | `lwz r6,-24160(r6)` @`0x80781D38` |
| camera-owner cell | `0x8042A27C` (main.dol BSS) | `lwz r3,-23940(r3)` @`0x80781D78` |
| camera index `12` | immediate | `li r4,0xC` @`0x80781D7C` |
| sentinel `-1` | immediate | `li r5,-1` @`0x80781D68` |

**Confidence: medium, conditional on `0x800F6410`.** Steps 1, 3, 4 and 5 are
literal and agree with the export; step 2 depends on a callee that has no
`functions.tsv` row anywhere in `work/ghidra` (it sits in a hole between
exported functions). Its disassembly was read from `main.elf`, but a lane must
carry `// unknown: 0x800F6410(this, &vec3)` rather than model it, and must not
assume the effect of the orientation beyond "uses the vector at `0x80377DEC`".
Two of the cells (`0x8042A1A0`, `0x8042A27C`) are BSS pointers with no shipped
value. **Verdict: conditional.**

---

## 11. `0x80716590` - 148 bytes - `work/ghidra/rel-d_basesNP/export/80716590_FUN_80716590.c`

**Body:** `0x80716590..0x80716620` (37 words, `blr` at `0x80716620`).
Ledger: absent. `src/game/bases.rs`: no definition, zero mentions. Export
warning: `Globals starting with '_' overlap smaller symbols` (two cells).

```rust
pub unsafe fn fn_80716590(this: usize)
```

**Argument map.** `r3 -> this` (`mr r30,r3` @`0x807165B0`); `r3` is untouched
through the first call, so `fn_80715ed0` receives `this`.

**Frame.** `stwu r1,-32(r1)`; `mfspr r0,LR` / `stw r0,36(r1)`;
`stw r31,28` / `r30,24`. Epilogue `0x8071660C..0x80716620`. **No `psq`.**

**Shape.** A jump/launch initialiser: zero the three velocity floats, arm a
20-frame timer, then emit one positional sound and one camera/rumble event.

1. `0x80716598-0x807165C4` - the state reset:
   ```
   f0 = 0.0f;                            // lfs f0,-13008(r4), r4 = 0x807D0000
   *(f32*)(this+0xE8) = f0;
   *(f32*)(this+0xEC) = f0;
   *(f32*)(this+0xF0) = f0;
   *(u16*)(this+0x7C4) = 20;             // li r0,20 ; sth r0,1988(r3)
   fn_80715ed0(this);                    // bl 0x80715ED0, r3 = this
   ```
2. `0x807165C8-0x807165EC` - the sound:
   ```
   r31 = *(u32*)0x80429D60;              // lis r4,0x8043 ; lwz r31,-25248(r4)
   0x8006A4B0(r1+8, this+0xAC);          // r3 = r1+8 ; r4 = this+0xAC (3 floats)
   0x80198C30(r31, 0x575, r1+8, 0);
   ```
   (`li r4,0x575` = 1397, `r5 = r1+8`, `r6 = 0`.)
3. `0x807165F0-0x80716608` - the camera/rumble:
   ```
   0x800D8C60(*(u32*)0x8042A008, 0, 1, 0, 0);
   ```
   (`lis r3,0x8043` / `lwz r3,-24568(r3)`; `li r4,0`; `li r5,1`; `r6 = r7 = 0`.)
4. `0x8071660C-0x80716620` - epilogue.

**Callees:**

| site | callee | arguments |
|---|---|---|
| `0x807165C4` | `0x80715ED0` (this REL, 80 B, **already translated**) | `r3 = this` (`fn_80715ed0(actor)`) |
| `0x807165D8` | `0x8006A4B0` (main.dol, 180 B) | `r3 = &dst[3]`, `r4 = this+0xAC` (copy 3 floats) |
| `0x807165EC` | `0x80198C30` (main.dol, 900 B) | `r3 = sound unit`, `r4 = 1397`, `r5 = &vec3`, `r6 = 0` |
| `0x80716608` | `0x800D8C60` (main.dol, 204 B) | `r3 = camera unit`, `r4 = 0`, `r5 = 1`, `r6 = 0`, `r7 = 0` |

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 0.0` | `0x807CCD30` | `lfs f0,-13008(r4)` @`0x807165A4` (`r4 = 0x807D0000`) |
| timer `20` | immediate | `li r0,20` @`0x807165A0`, `sth r0,1988(r3)` @`0x807165C0` |
| sound id `1397` (`0x575`) | immediate | `li r4,0x575` @`0x807165E4` |
| sound unit cell | `0x80429D60` (main.dol BSS) | `lwz r31,-25248(r4)` @`0x807165D0` |
| camera unit cell | `0x8042A008` (main.dol BSS) | `lwz r3,-24568(r3)` @`0x807165F8` |

**Confidence: medium.** Every store, both ids and all four calls are literal and
match the export (`*(undefined4 *)(param_1 + 0xe8) = DAT_807ccd30; ...;
*(undefined2 *)(param_1 + 0x7c4) = 0x14;`). The inferences are the roles of the
two BSS unit cells and of the `0x575`/`1` ids; the emitted Rust calls the
helpers and does not depend on them. **Verdict: ready.**

---

## 12. `0x80608D10` - 136 bytes - `work/ghidra/rel-d_basesNP/export/80608d10_FUN_80608d10.c`

**Body:** `0x80608D10..0x80608D94` (34 words, `blr` at `0x80608D94`).
Ledger: absent. `src/game/bases.rs`: no definition, zero mentions. Export
warnings: none.

```rust
pub unsafe fn fn_80608d10(this: usize) -> u32
```

**Argument map.** `r3 -> this` (`mr r31,r3` @`0x80608D20`). Always returns `1`
(`li r3,1` @`0x80608D84`, on both paths).

**Frame.** `stwu r1,-16(r1)`; `mfspr r0,LR` / `stw r0,20(r1)`; `stw r31,12`.
Epilogue `0x80608D80..0x80608D94`. **No `psq`.**

**Shape.** A two-flag update tick: set or clear bit 0 of a render flag byte and,
when both flags are set, also fire one virtual method and re-stamp the child
objects.

1. `0x80608D24-0x80608D38` - the guard:
   ```
   if (*(u8*)(this+0x270) == 0) goto clear;   // lbz / cmpi / beq 0x80608D70
   if (*(u8*)(this+0x271) == 0) goto clear;   // lbz / cmpi / beq 0x80608D70
   ```
2. `0x80608D3C-0x80608D4C` - the set arm:
   ```
   c = *(u32*)(this+0x238);
   *(u8*)(c+0xBB) = (*(u8*)(c+0xBB) & 0xFE) | 1;   // rlwinm r0,r0,0,24,30 ; ori r0,r0,1 ; stb
   ```
3. `0x80608D50-0x80608D68` - the virtual call and the re-stamp:
   ```
   vtable = *(u32*)(this+0xA8);
   fn = *(u32*)(vtable+0x10);
   mtspr CTR,fn;
   r3 = this + 0xA0;
   bcctr;                       // vmethod0(this+0xA0, 0x10), no result used
   fn_80608df0(this);           // mr r3,r31 ; bl 0x80608DF0
   ```
   `0x80608DF0` is already implemented in `src/game/bases.rs`
   (`pub unsafe fn fn_80608df0(this: usize)`) and specified in
   `docs/SPEC_WAVE134_BASES.md` §1.
4. `0x80608D70-0x80608D7C` - the clear arm:
   ```
   c = *(u32*)(this+0x238);      // lwz r3,568(r3)
   *(u8*)(c+0xBB) &= 0xFE;       // rlwinm r0,r0,0,24,30 ; stb
   ```
   Note the clear arm uses `r3` directly (the receiver is not re-copied from
   `r31`), so the byte address is the same field on the same object.
5. `0x80608D80-0x80608D94` - epilogue: `li r3,1` then restore.

**Callees:**

| site | callee | arguments |
|---|---|---|
| `0x80608D60` | vtable slot `0x10` of `*(u32*)(this+0xA8)` | `r3 = this+0xA0` (`bcctr 20,0,l` - the **only** `CTR` write in the batch) |
| `0x80608D68` | `0x80608DF0` (this REL, 248 B, **already translated**) | `r3 = this` |

**Constants:**

| value | image address | materialised at |
|---|---|---|
| flag-byte offsets `+0x270` / `+0x271` | immediate | `lbz r0,624(r3)` @`0x80608D24`, `lbz r0,625(r3)` @`0x80608D30` |
| render-record pointer `+0x238` | immediate | `lwz r4,568(r3)` @`0x80608D3C`, `lwz r3,568(r3)` @`0x80608D70` |
| flag byte `+0xBB`, mask `0xFE`, bit `1` | immediate | `rlwinm ...0,24,30` / `ori r0,r0,1` |
| vtable pointer `+0xA8`, slot `+0x10` | immediate | `lwz r12,168(r3)` @`0x80608D50`, `lwz r12,16(r12)` @`0x80608D54` |

**Confidence: high.** All 34 words decode, both arms are literal, and the export
matches including the receiver `param_1 + 0xa0` and the `& 0xfe | 1` bit dance.
The only virtual call is an explicit `bcctr` with the slot address in the raw
stream, so the wave-119 "spurious jumptable" failure mode does not apply.
**Verdict: ready.**

---

## Summary

| address | bytes | words | verdict | confidence | what a lane must record |
|---|---:|---:|---|---|---|
| `0x806FB540` | 200 | 50 | **ready** | high | `0x802DE514` is `abs(i32)`; `lhz` at `+0x8B8` |
| `0x80725170` | 196 | 49 | **ready** | high | table base `0x807CD208`; `0x8005F900` returns the object |
| `0x8069A170` | 184 | 46 | **ready** | medium | `// inferred: 0x802BE4A0 = sinf`; `0x80170980` normalises in place |
| `0x806A8AA0` | 184 | 46 | **conditional** | medium | `fmuls f1,f1,f2` multiplies by the integer count - keep the export reading |
| `0x80761880` | 176 | 44 | **ready** | medium | `0x80761670` is a 12-byte tail forwarder to `0x801662E0(this+0x20C,0)` |
| `0x80656260` | 172 | 43 | **conditional** | medium | `0x80429E00` is a BSS pointer cell; `+0x3C/+0x44` are the only visible fields |
| `0x807481B0` | 172 | 43 | **ready** | medium | `r3 = this`, `r4 = frames` (export declares `void(void)`); ring `0x80831750..0x80831780` |
| `0x80702CE0` | 168 | 42 | **ready** | high | 5-entry id table `01 04 05 06 07` at `0x807CC57C`; height `100.0` |
| `0x8078B120` | 168 | 42 | **ready** | medium | name table `0x80819658` = {`"ride_wait_broom"`, `"fly_broom"`} |
| `0x80781D10` | 152 | 38 | **conditional** | medium | `0x800F6410` has no export row; two BSS pointer cells |
| `0x80716590` | 148 | 37 | **ready** | medium | sound id `0x575`; timer `20`; two BSS unit cells |
| `0x80608D10` | 136 | 34 | **ready** | high | vtable slot `0x10` on receiver `this+0xA0`; `fn_80608df0` already merged |

**9 ready, 3 conditional.** Nine of the twelve can be written today with no
assumption beyond calling an external helper; the three conditional ones are
implementable today as well, but each carries exactly one inference that a
reviewer must accept, and the lane must leave the `// unknown:` / `// inferred:`
comment named above. No body in this batch needs another artifact to be
*decoded* - the whole 2 056 bytes are accounted for word by word.

## Verification performed

**Concurrency note (2026-11-12, at write time).** Wave 135 runs two translation
lanes in parallel with this one. Every "Ledger: absent" line in this document
means "absent from `ledger/functions.csv` and from every existing spec title",
not "absent from a work-in-progress Rust file at this minute".

- Candidate pool recomputed from `docs/ANALYSIS_WAVE127_BASES.md`: 121 class-B
  rows, 87 addresses named in `##` titles across `docs/SPEC_WAVE128_BASES.md` ..
  `docs/SPEC_WAVE134_BASES.md`, 33 addresses left (3 664 bytes) after also
  dropping the wave-134 §13 hand-off (`0x8069E5F0`). The twelve taken here are
  the twelve largest, with the 136-byte tie broken by ascending address.
- `ledger/functions.csv` checked **row-based**: `^game::bases,<addr>,` matches
  zero lines for all twelve. The doc-comment mention counts in
  `src/game/bases.rs` are listed in "Selection rule"; none is a definition.
- Every byte count equals the `functions.tsv` `size` column, and every export
  file exists at the cited path (12/12).
- Extent check: for all twelve, `u32(entry + size - 4) == 0x4E800020`, exactly
  one `bclr` per body, and no `functions.tsv` row starts inside a body.
- Duplicate check: SHA-1 of each of the twelve ranges compared against every
  same-size `d_basesNP` TSV row - no match. The hashes are
  `ed37d8e5..` (§1), `1544ffac..` (§2), `4ff38b50..` (§3), `ac403376..` (§4),
  `50ed4a2b..` (§5), `2db56f0e..` (§6), `809fa90b..` (§7), `7986fadf..` (§8),
  `8f8cc2cd..` (§9), `1c32618f..` (§10), `03b1024d..` (§11),
  `28277ab8..` (§12).
- Paired-single audit: 18 `psq*` words in six bodies, all `qr0` (`I = 0`),
  all `W = 0`, all `f31` prologue spill / epilogue reload pairs. No data-side
  `psq`, no `qr3`, no `mtspr` to a `GQRn`; `mtspr` operands are `LR` (all
  bodies) and `CTR` (§12 only).
- Compiler helper `0x802DCD74`/`0x802DCDC0` re-decoded from `work/dol/main.elf`:
  `stw r27..r31,-20..-4(r11)` and the matching `lwz` run; neither writes
  `r3..r10` or `LR`.
- Cross-artifact cells read directly: `0x80429E00`, `0x8042A1A0`, `0x80429D60`,
  `0x8042A008`, `0x8042A27C` are all in `main.elf` `PT_LOAD` segments with
  `filesz = 0` (BSS); `0x80377DEC` is in the `0x80351680` zero-fill segment.
  None has shipped bytes, so each is cited only as an address.
- In-image constants read directly: §1 `0x807CC3B4` = 1.0, `0x807CC3DC` = 0.1,
  `0x807CC3E0` = 16384.0, `0x807CC3E4` = 0.0625, `0x807CC3E8` =
  `0x4330000080000000`; §2 `0x807CD208` = {2.5, -2.5, 0.75, -1.5},
  `0x807CD200` = -0.2, `0x807CD1BC` = 0.0; §3 `0x807CA08C` = 0.0,
  `0x807CA0BC` = +0.39269909, `0x807CA0C0` = -0.39269909; §4 `0x807CA4C8` =
  `0x4330000080000000`, `0x807CA4D0` = 0.5, `0x807CA4D4` = 224.0,
  `0x807CA4D8` = 5500.0; §5 `0x807CEDB4` = 1.0; §6 `0x807C88F8` = 32.0,
  `0x807C88FC` = 16.0, `0x807C8900` = 0.0625, `0x807C8908` =
  `0x4330000080000000`; §8 `0x807CC5B0` = 100.0, `0x807CC57C` =
  `01 04 05 06 07`; §9 `0x807CFB64` = 5.0, `0x80819658` -> `0x80819638`
  `"ride_wait_broom"` / `0x80819648` `"fly_broom"`; §10 `0x807CF698` = 0.0;
  §11 `0x807CCD30` = 0.0.
- Callee cross-checks from `work/ghidra/main-gb/export/`: `0x802DE514` =
  `abs`; `0x80065CB0` = `*(p+0x134) = 0`; `0x8005F900` -> `0x80162D00` ->
  `*(node+8)`; `0x80170980` = in-place 3-float normalise; `0x801662E0` /
  `0x80166310` = `*(p+0x28) + idx*0x38` wrappers around `0x80168080` /
  `0x801680E0`; `0x8016F130` copies `+0xC/+0x1C/+0x2C`; `0x80064610` =
  `0x80064600(r7)` + `0x8006ca80(id, arg, &pos, 0)`.
- `0x80761670` decoded from the REL (12-byte tail forwarder) and `0x800F6410`
  decoded from `work/dol/main.elf` (vector-normalise + orient); both are
  recorded as such in §5 and §10.
- No file in the repository was written except this document; no merge or check
  tool was run.

## Hand-off

The pool is now **21** addresses (**1 608** bytes) after this lane. The next
twelve in the same order are `0x806B3AC0` (136), `0x80608C40` (132),
`0x8066B0F0` (128), `0x806FCCA0` (100), `0x80681480` (88), `0x80681FA0` (88),
`0x806FF3F0` (88), `0x80600990` (64), `0x806009D0` (64), `0x80600A10` (64),
`0x80600A50` (64) and `0x80600A90` (64).

None of the 21 needs a new artifact either: after this batch every remaining
class-B body is an integer-only or `f31`-spill-only function with zero `qr3` and
zero `GQR` writes, and the three shapes left (64-byte delete wrappers, 88/100/
128/132/136-byte two-to-three-call bodies) are smaller than every section above.
