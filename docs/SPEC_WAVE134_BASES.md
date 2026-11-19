# Wave 134 - `game::bases` - implementation spec for the next 12 class-B addresses

Read-only lane. The only repository file left modified is this one. The
throwaway PowerPC decoder (`%TEMP%\opencode\ppc134b.js`) and the ELF data
reader (`data134b.js`) live outside the repository; no Rust file, no
`ledger/functions.csv` row and no existing document was modified. No merge or
check tool was run.

## Why these twelve

`docs/ANALYSIS_WAVE127_BASES.md` reclassified 121 `d_basesNP` skips as class B
("reconstructible now from the raw stream"). Waves 128-133 carry **75** address
sections over that pool (`## N. \`0xADDR\`` headings in
`docs/SPEC_WAVE128_BASES.md` .. `docs/SPEC_WAVE133_BASES.md`; every one of the
75 names a class-B address). Of the 121 class-B rows exactly **46** are still
neither claimed nor present in the ledger, totalling **6 556** bytes. This lane
takes the twelve largest of those, ordered by `functions.tsv` size descending
and then by address ascending:

| # | address | bytes | words | export file | verdict |
|--:|---:|---:|---:|---|---|
| 1 | `0x80608DF0` | 248 | 62 | `80608df0_FUN_80608df0.c` | ready (high) |
| 2 | `0x807268C0` | 248 | 62 | `807268c0_FUN_807268c0.c` | ready (medium) |
| 3 | `0x80780A90` | 240 | 60 | `80780a90_FUN_80780a90.c` | ready (medium) |
| 4 | `0x8069A6E0` | 236 | 59 | `8069a6e0_FUN_8069a6e0.c` | conditional (medium) |
| 5 | `0x80643470` | 228 | 57 | `80643470_FUN_80643470.c` | ready (high) |
| 6 | `0x8068E440` | 228 | 57 | `8068e440_FUN_8068e440.c` | ready (medium) |
| 7 | `0x80747B30` | 224 | 56 | `80747b30_FUN_80747b30.c` | conditional (medium) |
| 8 | `0x8069F560` | 208 | 52 | `8069f560_FUN_8069f560.c` | ready (high) |
| 9 | `0x8069F9D0` | 208 | 52 | `8069f9d0_FUN_8069f9d0.c` | ready (high) |
| 10 | `0x806B4F90` | 208 | 52 | `806b4f90_FUN_806b4f90.c` | ready (medium) |
| 11 | `0x80781B60` | 208 | 52 | `80781b60_FUN_80781b60.c` | ready (medium) |
| 12 | `0x80642D20` | 204 | 51 | `80642d20_FUN_80642d20.c` | ready (medium) |

Together **2 688 bytes** (672 words). Every export path is exactly
`work/ghidra/rel-d_basesNP/export/<file>` relative to the repository root, and
every `bytes` value matches the `size` column of that directory's
`functions.tsv` verbatim.

### Selection rule

The candidate set is: **class B in wave 127**, **no row in
`ledger/functions.csv` whose address column equals the address**, and **no
`## N. \`0xADDR\`` heading in waves 128-133**. The ledger test is *row-based*
(comma field 2 of each line), not a substring search: one of the twelve is
named inside the free-text evidence column of an unrelated row, and a substring
test would silently drop it.

| address | ledger mentions | where |
|---|---:|---|
| `0x80747B30` | 1 | evidence of `fn_80747c30` (ledger line 4064) |

As recorded at `docs/SPEC_WAVE129_BASES.md:1464-1468` and reused at
`docs/SPEC_WAVE130_BASES.md:1283-1286`, `docs/SPEC_WAVE131_BASES.md:40-59`,
`docs/SPEC_WAVE132_BASES.md:40-57` and `docs/SPEC_WAVE133_BASES.md:39-57`,
**a textual mention is not a claim** - only a section heading is.

### Ties, and what is left

The four 208-byte bodies and the 204-byte pair put the cut inside the second
204-byte group: `0x80642D20` and `0x8069E5F0` are both 204 bytes, so the second
of them is the 13th largest. It is decoded in §13 as a hand-off; it is the
sibling of §8/§9 with the two prologue clears removed and the `+0x360` flag
replaced by a `+0xD98` virtual dispatch. The next six after the cut, in the
same order, are:

| address | bytes | note |
|---|---:|---|
| `0x8069E5F0` | 204 | 13th; see §13, sibling of §8/§9 |
| `0x806FB540` | 200 | `qr0` `f31` spill only (wave 113) |
| `0x80725170` | 196 | `qr0` `f31` spill only (wave 125) |
| `0x8069A170` | 184 | `qr0` `f31` spill only (wave 119) |
| `0x806A8AA0` | 184 | `qr0` `f31` spill only (wave 124) |
| `0x80761880` | 176 | `qr0` `f31` spill only (wave 114) |

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

**Extent check.** For all twelve bodies the reachable span equals the TSV size
exactly: the last word of every range is `0x4E800020` (`blr`) at
`entry + size - 4`, and no TSV row starts inside a body. No over-read, no
`halt_baddata`, no jump table. The only indirect branches are five explicit
`bctr` virtual calls: one in §6, three in §10 and one in §12.

**Cross-artifact constants.** Five of the cells read below are not in this REL:

| cell | artifact | value |
|---|---|---|
| `0x8042C1A0` | `main.elf` PH11 (`0x8042B080` + 0x1130) | `01 FF 00 00 ...` - a two-entry signed-byte table `{+1, -1}` |
| `0x8042A18C`, `0x8042A1A0`, `0x8042A4A8`, `0x80429E00` | `main.elf` BSS (`0x80429BC0`, filesz 0; `0x8042FBE0`, filesz 0) | zero-initialised runtime cells |

Those four BSS cells have no shipped bytes in either artifact: they are always
cited as addresses, never as literals.

**Paired-single audit.** Across the 672 words there are **21 `psq*` words**:
10 `psq_st` prologue saves, 10 `psq_l` epilogue reloads, and **one data-side
load** (`0x8069A76C` in §4). Every one is `qr0` (`I = 0`) with `W = 0`.
Saved set: `f31`, `f30`. Counts: two words each in §2, §6, §7 and §12 (one
pair each); four each in §3 and §10 (two pairs each); five in §4 (two pairs
plus the data pair); none in §1, §5, §8, §9, §11. There is **no `qr3` anywhere
in this batch and no `mtspr` to a `GQRn`**: the only `mtspr` operands are `LR`
(spr 8) and `CTR` (spr 9). Nine of the twelve were rejected by wave-111/112/
113/114/115/118/119/120/123/124/125 "`unaff_GQR0`" bullets (§2, §3, §4, §6,
§7, §8, §9, §10, §12); §1 and §5 were rejected as "delete wrappers" and
"dropped-receiver calls". In every case the rejection is a decompiler artefact,
not a missing constant.

**Compiler save chain (re-decoded from `work/dol/main.elf`).** Only one
save/restore pair is used here; it is a straight run of `stw rN,-off(r11)`
ending in `blr` and it does not write `r3..r13` or `LR`:

| save stub | saves | matching restore | used by |
|---|---|---|---|
| `0x802DCD70` | r26-r31 | `0x802DCDBC` | §8, §9, §13 |

`r11` must be `r1 + frame_size`; each body sets it immediately before the call.
After the `bl`, entry `r3` is still the `this` pointer, and each body copies it
into `r26` on the next instruction. That is why Ghidra prints
`void FUN_8069f560(void)`.

**House style used below.** `pub unsafe fn fn_XXXXXXXX(this: usize)`,
`platform::read_u32/read_i32/read_f32/read_u8/read_u16`,
`platform::write_*`, `abi_call1!..abi_call10!`, `vmethod0(rcv, slot)`,
`ptr_at(addr)`, addresses written `0x8060_8DF0`, field offsets as `usize`
literals, `// unknown:` on the first use of every address-only target or
external cell.

---

## 1. `0x80608DF0` - 248 bytes - `work/ghidra/rel-d_basesNP/export/80608df0_FUN_80608df0.c`

**Body:** `0x80608DF0..0x80608EE4` (62 words, `blr` at `0x80608EE4`).
Ledger and `src/game/bases.rs`: absent (zero mentions). No export warnings.
Leaf: no `mfspr LR`, no LR save; the only `mtspr` is `CTR`.

```rust
pub unsafe fn fn_80608df0(this: usize)
```

**Frame.** `stwu r1,-96(r1)`; `stw r31,92` / `r30,88` / `r29,84`. No `psq`.

**Shape.** Twelve iterations that copy four source floats into five destination
slots (the fifth is a literal zero) plus one orientation byte, from a per-slot
source record into a per-slot render record; the loop is a `mtspr CTR,12` /
`bdz` countdown.

1. `0x80608DF4-0x80608E10` - `r4 = 0x807C0000`; `f2 = *(f32*)0x807C6730`
   (= `0.0`); `r0 = 12`; `mtspr CTR,r0`; `r29 = this`.
2. `0x80608E14-0x80608E4C` - per iteration `i` (stride 4 on `r29`):
   ```
   a = *(u32*)(this + 0x240 + 4*i);        // source record
   c = *(u32*)(this + 0x070 + 4*i);        // destination container
   ```
   and `a+0x84/0x88/0x8C/0x90/0x94/0x98/0x9C/0xA0/0xA4/0xA8/0xAC/0xB0`
   are loaded into r5/r31/r12/stack/r11/r4/r10/stack/r9/r8/r7/r6.
3. `0x80608E54-0x80608E80` - `b = *(u32*)(c + 0x244)` (destination record):
   ```
   *(f32*)(b+0x2C) = *(f32*)(a+0x90);
   *(f32*)(b+0x30) = *(f32*)(a+0xA0);
   *(f32*)(b+0x34) = 0.0;                  // image constant, not a read of a+0x34
   ```
   then `f1 = *(f32*)(a+0x84)` -> `b+0x44`, `f0 = *(f32*)(a+0x98)` -> `b+0x48`.
   The two source words are moved through `stw`/`lfs` scratch at `r1+0x2C` and
   `r1+0x3C`; the payload is bit-identical, so write them as `f32` loads.
4. `0x80608E84-0x80608ECC` - the per-iteration byte copy:
   ```
   *(u8*)(*(u32*)(*(u32*)(this + 0x70 + 4*i) + 0x248) + 0xB8)
       = *(u8*)(*(u32*)(this + 0x23C) + 0xB8);
   ```
   The right-hand side is re-loaded every iteration but never changes.
5. `0x80608ED0` - `bdz 0x80608E14`.
6. `0x80608ED4-0x80608EE4` - epilogue.

**Dead spills (do not translate as live state).** Words r31/r12/r11/r10/r9/r8/
r7/r6 are also written to `r1+0x24..0x4C`, and the four floats to
`r1+8..0x18`. Nothing reads those slots again - the body contains no call and
no branch other than the `bdz`. Ghidra drops them; keep them dropped.

**Direct callees:** none. No virtual calls, no `psq`.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 0.0` | `0x807C6730` | `lis r4,0x807C` @`0x80608DF4` + `lfs f2,0x6730(r4)` @`0x80608DFC` |
| iteration count `12` | immediate | `li r0,12` @`0x80608DF8` |

**Confidence: high.** All 62 words decode and the Ghidra export agrees
field-for-field, including the `0.0` store at `b+0x34` and the varying source
offsets `0x84/0x90/0x98/0xA0`.

---

## 2. `0x807268C0` - 248 bytes - `work/ghidra/rel-d_basesNP/export/807268c0_FUN_807268c0.c`

**Body:** `0x807268C0..0x807269B4` (62 words, `blr` at `0x807269B4`).
Ledger and `src/game/bases.rs`: absent (zero mentions). The export carries the
wave-127 "removing unreachable block" warning at `0x8072699C` (the `psq_l`/`lfd`
restore pair) - spurious.

```rust
pub unsafe fn fn_807268c0(this: usize, node_index: u32, transform: usize, model: usize)
```

**Argument map.** `r3 -> this` (`r30`), `r4 -> node_index` (forwarded untouched
as the node index of `0x8023B300`; the body never reads it), `r5 -> transform`
(`or r31,r5,r5` @`0x807268D8`, later the first argument of all three transform
calls), `r6 -> model` (`or r3,r6,r6` @`0x807268E4`).

**Frame.** `stwu r1,-64(r1)`; `mfspr r0,LR` / `stw r0,68(r1)`;
`stfd f31,48` + `psq_st f31,56(r1),0,qr0`; `stw r31,44` / `r30,40`.
No save-helper. Epilogue `0x80726998..0x807269B4`.

**Shape.** A name-gated "add a yaw to the second transform component and set an
x scale" helper. It is the sibling of `0x80722180` (already implemented, spec
`docs/SPEC_WAVE132_BASES.md` §7), with one key (`plp`) instead of two, a
different angle field and a different component index.

1. `0x807268E4-0x80726904` - `node = 0x8023B300(model, node_index)`;
   `off = *(u32*)(node+8)`; `name = off != 0 ? node + off : 0`.
2. `0x80726904-0x80726914` - `r4 = 0x8080AF18` (`"plp"`);
   `if (0x802E1AB4(name, "plp") != 0) goto epilogue;` (a `bne` at
   `0x80726914`). The compare is a plain `strcmp`-style equality.
3. `0x80726918-0x80726954` - `owner = *(u32*)(this+4)`;
   `angle = (i16)*(owner+0x854)` (`lha`); then the signed-to-single ladder:
   ```
   t   = angle * 90;            // mulli r0,r0,90  @0x80726938
   t   = t / 16384;             // srawi r0,r0,14  @0x8072693C
   t   = t + ((t < 0) & ((t & 0x3FFF) != 0));   // addze r0,r0 @0x80726940
   f31 = (f32)t;                // 0x43300000_<t^0x8000> - (2^52+2^31), fsubs
   ```
   `mulli` + `srawi` + `addze` is exactly truncation toward zero of
   `angle*90/16384`; keep the `addze`, it changes negative angles by one.
4. `0x80726954-0x80726968` - `0x80241710(transform, &r1+0x14)` decomposes the
   transform into three floats at `r1+0x14/0x18/0x1C`;
   `*(f32*)(r1+0x18) += f31`. **The adjusted component is the second one**
   (`+0x18 = buffer + 4`), not the third; this differs from `0x80722180`.
5. `0x8072696C` - `0x80241AA0(transform, &r1+0x14)` recomposes.
6. `0x80726970-0x80726994` - scale vector at `r1+8`:
   `[0] = *(f32*)(owner+0x84C)`, `[1] = [2] = 1.0`;
   `0x80241A00(transform, &r1+8)`.
7. `0x80726998-0x807269B4` - epilogue.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x807268E8` | `0x8023B300` | `r3 = model`, `r4 = node_index` -> node pointer |
| `0x8072690C` | `0x802E1AB4` | `r3 = name`, `r4 = 0x8080AF18` -> `i32` (`0` = equal) |
| `0x80726954` | `0x80241710` | `r3 = transform`, `r4 = &r1+0x14` |
| `0x8072696C` | `0x80241AA0` | `r3 = transform`, `r4 = &r1+0x14` |
| `0x80726994` | `0x80241A00` | `r3 = transform`, `r4 = &r1+8` |

No virtual calls. Two `psq*` words, `qr0`/`W=0` (one `f31` pair).

**Constants:**

| value | image address | materialised at |
|---|---|---|
| shared pool base | `0x807D0000` | `lis r4,0x807D` @`0x80726924`, `lis r3,0x807D` @`0x80726974` |
| `f64 2^52 + 2^31` = `0x4330000080000000` | `0x807CD280` | `lfd f1,-11648(r4)` @`0x80726930` |
| `f32 1.0` | `0x807CD278` | `lfs f0,-11656(r3)` @`0x80726978` |
| `"plp"` | `0x8080AF18` | `addi r4,r4,-20712` @`0x80726908` (`r4 = 0x80810000`) |
| yaw numerator `90` | immediate | `mulli r0,r0,0x5A` @`0x80726938` |
| yaw shift `14` | immediate | `srawi r0,r0,14` @`0x8072693C` |

**Confidence: medium.** All 62 words decode; the node/name resolution, the
string compare, the integer ladder and the four transform calls are literal.
The only inference is the decompose/recompose split of `0x80241710` /
`0x80241AA0`, which is inherited verbatim from the already-merged sibling
`fn_80722180`. Emit `// unknown: 0x80241710(transform, &mut [f32;3])` and
`// unknown: 0x80241AA0(transform, &[f32;3])`; do **not** copy the sibling's
component index (this one is `+4`).

---

## 3. `0x80780A90` - 240 bytes - `work/ghidra/rel-d_basesNP/export/80780a90_FUN_80780a90.c`

**Body:** `0x80780A90..0x80780B7C` (60 words, `blr` at `0x80780B7C`).
Ledger: absent (no mention anywhere in the CSV). `src/game/bases.rs`: no
definition; three doc-comment mentions. Export warnings: two "removing
unreachable block" (the `psq_l`/`lfd` restore block) - spurious.

```rust
pub unsafe fn fn_80780a90(this: usize) -> f32
```

**Frame.** `stwu r1,-48(r1)`; `mfspr r0,LR` / `stw r0,52(r1)`;
`stfd f31,32` + `psq_st f31,40`; `stfd f30,16` + `psq_st f30,24`;
`stw r31,12` / `r30,8`. Epilogue `0x80780B54..0x80780B7C`.

**Shape.** Distance to the first live actor on `this`'s `+0x188` chain, with a
state-dependent switch to a two-leg path length.

1. `0x80780AB4-0x80780ABC` - `r30 = this`;
   `r31 = 0x80780B80(this)` - the chain head whose `+0x124` byte is non-zero
   (`fn_80780b80`, already translated). Call it `other`.
2. `0x80780AC0-0x80780AD0` - `d1 = sqrt_f32(0x801C1650(this+0xAC, other+0xAC))`;
   `f30 = d1`. `0x801C1650` returns the **squared** distance, so the `f32`
   `bl 0x802BE470` is the actual length.
3. `0x80780AD4-0x80780AE4` - `d2 = sqrt_f32(0x801C1650(this+0x1D8, other+0xAC))`;
   `f31 = d2`.
4. `0x80780AE8-0x80780AF8` - `d3 = sqrt_f32(0x801C1650(this+0xAC, this+0x1D8))`;
   `f31 = f31 + d3` (`fadds f31,f1,f31`).
5. `0x80780AFC-0x80780B0C` - `n = 0x80780A40(this)` (count of live linked
   actors, 1-based).
6. `0x80780B0C-0x80780B2C` - **`n == 1` arm**:
   ```
   if (*(u32*)(this+0x200) != *(u32*)(*(u32*)0x8042A1A0 + 0x23C))
       f30 = f31;
   ```
   (equal id keeps the direct distance).
7. `0x80780B30-0x80780B50` - **`n != 1` arm**:
   ```
   if (*(u32*)(this+0x200) == *(u32*)(other+0x200)) keep f30;      // beq
   else if (*(u32*)(this+0x1D0) == 3)             keep f30;         // bne -> fmr
   else                                            f30 = f31;
   ```
   The `+0x1D0` word is the state enum already used by `fn_807811D0` (spec
   `docs/SPEC_WAVE133_BASES.md` §12) as `state == 6`; here it is state `3`.
8. `0x80780B54-0x80780B5C` - restore the `psq_l` pair, `fmr f1,f30`.
9. `0x80780B60-0x80780B7C` - epilogue; the function returns `f1`.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80780AB8` | `0x80780B80` | `r3 = this` -> chain head (`fn_80780b80`) |
| `0x80780AC8` | `0x801C1650` | `r3 = this+0xAC`, `r4 = other+0xAC` -> `f1` (squared) |
| `0x80780ACC` | `0x802BE470` | `f1` -> `f1` (square root, `fn_802be470`) |
| `0x80780ADC` | `0x801C1650` | `r3 = this+0x1D8`, `r4 = other+0xAC` -> `f1` |
| `0x80780AE0` | `0x802BE470` | `f1` -> `f1` |
| `0x80780AF0` | `0x801C1650` | `r3 = this+0xAC`, `r4 = this+0x1D8` -> `f1` |
| `0x80780AF4` | `0x802BE470` | `f1` -> `f1` |
| `0x80780B00` | `0x80780A40` | `r3 = this` -> `i32` (`fn_80780a40`) |

No virtual calls. Four `psq*` words, `qr0`/`W=0` (`f31`/`f30` pairs).

**Constants:**

| value | image address | materialised at |
|---|---|---|
| actor-manager pointer | `0x8042A1A0` | `lis r3,0x8043` @`0x80780B0C` + `lwz r3,-24160(r3)` @`0x80780B14` |
| manager id word | `[0x8042A1A0]+0x23C` | `lwz r3,572(r3)` @`0x80780B18` |
| own id / state / secondary position | `this+0x200` / `this+0x1D0` / `this+0x1D8` | `lwz r0,512(r30)` @`0x80780B10`, `lwz r0,464(r30)` @`0x80780B40, `addi r3,r30,472` @`0x80780AD4` |

`0x8042A1A0` is `main.dol` BSS: read it at run time, never as a literal.

**Confidence: medium.** All 60 words decode, the three distances and the two
gates are literal, and every callee is either already translated
(`fn_80780b80`, `fn_801c1650`, `fn_802be470`, `fn_80780a40`) or a named
`main.dol` cell. The residual doubt is only the *meaning* of the `+0x1D8`
second position and of state `3`; the arithmetic is exact. Keep the first
`0x802BE470` result (`f30`) and do not reuse `f1` after the third call.

---

## 4. `0x8069A6E0` - 236 bytes - `work/ghidra/rel-d_basesNP/export/8069a6e0_FUN_8069a6e0.c`

**Body:** `0x8069A6E0..0x8069A7C8` (59 words, `blr` at `0x8069A7C8`).
Ledger and `src/game/bases.rs`: absent (zero mentions). Export warnings: three
"removing unreachable block" around the `psq_l`/`lfd` restore - spurious.

```rust
pub unsafe fn fn_8069a6e0(this: usize, point: usize) -> u32
```

**Argument map.** `r3 -> this` (`r28`), `r4 -> point` (`or r29,r4,r4`
@`0x8069A720`; read only as `point+0xAC` and `point+0xB0`).

**Frame.** `stwu r1,-80(r1)`; `mfspr r0,LR` / `stw r0,84(r1)`;
`stfd f31,64` + `psq_st f31,72`; `stfd f30,48` + `psq_st f30,56`;
`stw r31,44` / `r30,40` / `r29,36` / `r28,32`. `r31 = r1+8` is the scratch pair
used by the paired-single block. Epilogue `0x8069A79C..0x8069A7C8`.

**Shape.** "Is any live player within 128 units of `point` (2-D)?" - returns
`1` on the first hit, `0` otherwise.

1. `0x8069A6FC-0x8069A70C` - `f31 = *(f32*)0x807CA0E8` (= `16384.0`);
   `f30 = *(f32*)0x807CA08C` (= `0.0`); `r31 = r1+8`; `r30 = 0`.
2. `0x8069A72C-0x8069A73C` - `if (0x8069A680(this, i) == 0) goto next;`
   (`fn_8069a680`, already translated: player-mask bit plus "query tag `0x4F`
   absent").
3. `0x8069A740-0x8069A74C` - `player = 0x8005F900(i)` (`fn_8005f900`);
   `dy = *(f32*)(player+0xB0) - *(f32*)(point+0xB0)`;
   `dx = *(f32*)(player+0xAC) - *(f32*)(point+0xAC)`.
4. `0x8069A75C-0x8069A768` - build the scratch pair and the dead triple:
   `*(f32*)(r1+0xC) = dy`, `*(f32*)(r1+8) = dx`, `*(f32*)(r1+0x10) = 0.0`.
   The three stores at `r1+8/0xC/0x10` are the pair input; `+0x10` is dead.
5. `0x8069A76C-0x8069A77C` - the paired-single lane block:
   ```
   psq_l    f0, 0(r31), 0, qr0     // f0 = (dx, dy)
   ps_muls1 f0, f0, f0             // 0x10000032
   ps_merge01 f1, f30, f0          // 0x103E07BA, f30 = (0.0, 0.0)
   ps_add   f1, f1, f0             // 0x10210014
   fcmpo    cr0, f1, f31           // compare against 16384.0
   bge      next                   // skip when not strictly less
   ```
   Ghidra resolves the lane algebra to `0.0*0.0 + dx*dx + dy*dy < 16384.0`,
   i.e. the **2-D squared distance < 128²**. Emit
   `dx*dx + dy*dy < 16384.0` and keep the three PS words quoted above in a
   comment so the verifier can compare bytes.
6. `0x8069A784-0x8069A788` - `return 1` on the hit.
7. `0x8069A78C-0x8069A794` - `i++`; `while (i < 4)`.
8. `0x8069A798` - `return 0`.
9. `0x8069A79C-0x8069A7C8` - epilogue.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x8069A734` | `0x8069A680` | `r3 = this`, `r4 = i` -> `u32` (`fn_8069a680`) |
| `0x8069A744` | `0x8005F900` | `r3 = i` -> player pointer (`fn_8005f900`) |

No virtual calls. **Five `psq*` words**: two `f31` and two `f30` spill pairs
plus the one **data-side** `psq_l` (the only data `psq` in this batch).

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 16384.0` = 128² | `0x807CA0E8` | `lis r5,0x807D` @`0x8069A6EC` + `lfs f31,-24344(r5)` @`0x8069A6FC` |
| `f32 0.0` | `0x807CA08C` | `lis r6,0x807D` @`0x8069A6E8` + `lfs f30,-24436(r6)` @`0x8069A708` |
| player position | `+0xAC` / `+0xB0` | `lfs f1,172(r3)` / `lfs f3,176(r3)` @`0x8069A750/0x8069A748` |
| point position | `+0xAC` / `+0xB0` | `lfs f0,172(r29)` / `lfs f2,176(r29)` @`0x8069A754/0x8069A74C` |

**Residual ambiguity -> conditional (medium).** The control flow, the mask test,
the four-player bound and the two position reads are literal. The one
unresolved item is the **lane algebra of the three PS words**: Ghidra's vector
model folds them to `dx² + dy²`, while a strict slot-by-slot reading of
`ps_muls1` / `ps_merge01` needs the exact Gekko lane convention to confirm.
Write the body as `dx*dx + dy*dy < 16384.0` (the gameplay-consistent reading,
and the one Ghidra prints) and carry
`// unknown: PS lane fold (0x10000032, 0x103E07BA, 0x10210014)`. Do not turn
the comparison into a radius on `dx` only.

---

## 5. `0x80643470` - 228 bytes - `work/ghidra/rel-d_basesNP/export/80643470_FUN_80643470.c`

**Body:** `0x80643470..0x80643550` (57 words, `blr` at `0x80643550`).
Ledger and `src/game/bases.rs`: absent (zero mentions).

```rust
pub unsafe fn fn_80643470(this: usize)
```

**Frame.** `stwu r1,-32(r1)`; `mfspr r0,LR` / `stw r0,36(r1)`;
`stw r31,28` / `r30,24` / `r29,20` / `r28,16`. No `psq`, no save-helper.
Epilogue `0x80643534..0x80643550`.

**Shape.** A "reset the seven panes and start channel 0" initialiser: rewind the
embedded channel array, force channels 1..6 to state 3, start channel 0, set a
flag bit, reset seven records, then poke the message system.

1. `0x8064348C-0x80643494` - `r28 = this`; `0x800C9500(this+0x74)` - clears
   every channel entry and rewinds the `+0x194` cursor (`fn_800c9500`).
2. `0x80643498-0x806434B4` - `for (i = 1; i <= 6; i++)`
   `0x800C93D0(this+0x74, i, 0)` (`fn_800c93d0`; `offset == 0` takes the
   descriptor-apply arm and then forces channel state 3).
3. `0x806434B8-0x806434C4` - `0x800C92F0(this+0x74, 0, 0)`
   (`fn_800c92f0`; starts channel 0 and writes state 1).
4. `0x806434C8-0x806434E4` - `p = *(u32*)(this+0x248)`;
   `*(u8*)(p+0xBB) = (*(u8*)(p+0xBB) & 0xFE) | 1` (the raw sequence is
   `lbz` / `rlwinm ...,0,24,30` / `ori ...,1` / `stb`).
5. `0x806434E8-0x80643520` - seven-record loop `i = 0..6`:
   ```
   a = *(u32*)(this + 0x2F4 + 4*i);
   *(u8*)(a+0x209) = 1;                 // stb r30 (1)
   0x807678C0(a);                       // fn_807678c0(actor)
   *(u32*)(a+0x204) =
       if (*(u32*)(this + 0x8B4 + 4*i) == 0) { 1 } else { 0 };
   ```
   The store at `+0x204` is *after* the call, and the condition is the
   4-byte word in the parallel `+0x8B4` array, not a byte flag.
6. `0x80643524-0x80643530` - `0x8019CA00(*(u32*)0x8042A4A8, 4)`
   (`fn_8019ca00`, cue id 4).
7. `0x80643534-0x80643550` - epilogue.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80643494` | `0x800C9500` | `r3 = this+0x74` |
| `0x806434A8` | `0x800C93D0` | `r3 = this+0x74`, `r4 = i`, `r5 = 0` (6 times) |
| `0x806434C4` | `0x800C92F0` | `r3 = this+0x74`, `r4 = 0`, `r5 = 0` |
| `0x806434F0` | `0x807678C0` | `r3 = *(u32*)(this + 0x2F4 + 4*i)` (7 times) |
| `0x80643530` | `0x8019CA00` | `r3 = *(u32*)0x8042A4A8`, `r4 = 4` |

No virtual calls, no `psq` at all.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| message/sound singleton | `0x8042A4A8` | `lis r3,0x8043` @`0x80643524` + `lwz r3,-23384(r3)` @`0x8064352C` |
| channel array | `this+0x74` | `addi r3,r3,116` @`0x80643490` |
| flag byte / bit | `this+0x248`, `+0xBB` bit 0 | `lwz r3,584(r28)` @`0x806434C8` |
| record array | `this+0x2F4`, stride 4, 7 entries | `lwz r3,756(r28)` @`0x806434E8` |
| condition array | `this+0x8B4`, stride 4 | `lwz r0,2228(r28)` @`0x806434F4` |
| record flags | `+0x209` = 1, `+0x204` = 0/1 | `stb r30,521(r3)` @`0x806434EC`, `stw` @`0x80643504/0x80643510` |
| cue argument `4` | immediate | `li r4,4` @`0x80643528` |

`0x8042A4A8` has no shipped bytes - keep it as an address.

**Confidence: high.** All 57 words decode, both loops and all five calls are
literal, and the three channel helpers are already translated with the exact
`(this, index, offset)` signature used here.

---

## 6. `0x8068E440` - 228 bytes - `work/ghidra/rel-d_basesNP/export/8068e440_FUN_8068e440.c`

**Body:** `0x8068E440..0x8068E520` (57 words, `blr` at `0x8068E520`).
Ledger: absent. `src/game/bases.rs`: no definition; one doc-comment mention.

```rust
pub unsafe fn fn_8068e440(this: usize) -> u32
```

**Frame.** `stwu r1,-48(r1)`; `mfspr r0,LR` / `stw r0,52(r1)`;
`stfd f31,32` + `psq_st f31,40`; `stw r31,28` / `r30,24` / `r29,20`.
Epilogue `0x8068E4FC..0x8068E520`; `li r3,1` at `0x8068E500` is the return.

**Shape.** A reset/initialise pass over the 228-byte pane records, followed by a
per-player regroup and a landing/terrain helper.

1. `0x8068E460-0x8068E474` - `r29 = this`; virtual call
   ```
   vtable = *(u32*)(this+0x394);
   vtable[0x10/4]()(this+0x394);        // lwz r12,916(r3) / lwz r12,16(r12) / bctr
   ```
   receiver is the **embedded** object at `this+0x394` (`r3 = this+0x394`), slot
   `+0x10`; the result is unused.
2. `0x8068E478-0x8068E47C` - `0x8068EA10(this)` - the sibling pane initialiser
   (class C in wave 127: it has `qr3` quantisation; opaque here).
3. `0x8068E480-0x8068E4A4` - record loop:
   ```
   r31 = this + 0x3D0;
   for (r30 = 0; r30 < *(u32*)(this+0xC68); r30++) {
       0x8007FDA0(r31);                 // fn_8007fda0(layer)
       r31 += 0xE4;
   }
   ```
   The bound is read **inside** the loop test (`lwz r0,3176(r29)` at
   `0x8068E49C`), so keep the comparison there.
4. `0x8068E4A8-0x8068E4EC` - if `*(u32*)(this+0xC44) != 0`:
   ```
   f31 = 0.0;                            // 0x807C9C30
   for (i = 0; i < 4; i++) {
       v = *(f32*)(this + 0xC50 + 4*i);
       if (!(v == 0.0)) {                // fcmpu + beq skips (EQ or unordered)
           player = 0x8005F900(i);
           0x80057820(player, v);        // clamp to +/-2.0, store at +0x14C4
       }
   }
   ```
5. `0x8068E4F0-0x8068E4F8` - `0x80064F50(this, 0)` (landing/terrain helper,
   returns `0/1`, discarded).
6. `0x8068E4FC` - `return 1`.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x8068E474` | `*(u32*)(*(u32*)(this+0x394)+0x10)` | `r3 = this+0x394` |
| `0x8068E47C` | `0x8068EA10` | `r3 = this` |
| `0x8068E490` | `0x8007FDA0` | `r3 = this + 0x3D0 + 0xE4*i` |
| `0x8068E4D4` | `0x8005F900` | `r3 = i` -> player pointer |
| `0x8068E4DC` | `0x80057820` | `r3 = player`, `f1 = *(f32*)(this+0xC50+4*i)` |
| `0x8068E4F8` | `0x80064F50` | `r3 = this`, `r4 = 0` -> `i32` |

One virtual call; two `psq*` words, `qr0`/`W=0` (one `f31` pair).

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 0.0` | `0x807C9C30` | `lis r3,0x807D` @`0x8068E4B4` + `lfs f31,-25552(r3)` @`0x8068E4BC` |
| embedded notify object | `this+0x394` | `lwz r12,916(r3)` @`0x8068E464` + `addi r3,r3,916` @`0x8068E470` |
| record base / bound | `this+0x3D0` / `this+0xC68` | `addi r31,r29,976` @`0x8068E480`, `lwz r0,3176(r29)` @`0x8068E49C` |
| record stride | `0xE4` (228) | `addi r31,r31,228` @`0x8068E494` |
| regroup gate | `this+0xC44` | `lwz r0,3140(r29)` @`0x8068E4A8` |
| value array | `this+0xC50`, 4 floats | `lfs f0,3152(r31)` @`0x8068E4C4` |

**Confidence: medium.** All 57 words decode. The virtual slot, the record loop
and the player loop are literal; the soft spots are `0x8068EA10` (a class-C
sibling with `qr3`) and `0x80064F50` (a landing/terrain probe whose internals
are outside this REL). Both are called for effect only. Keep the `beq` skip
polarity and the `li r3,1` return.

---

## 7. `0x80747B30` - 224 bytes - `work/ghidra/rel-d_basesNP/export/80747b30_FUN_80747b30.c`

**Body:** `0x80747B30..0x80747C0C` (56 words, `blr` at `0x80747C0C`).
Ledger: absent as a row; mentioned only in the evidence of `fn_80747c30`
(ledger line 4064). `src/game/bases.rs`: no definition; five mentions.

```rust
pub unsafe fn fn_80747b30(this: usize)
```

**Frame.** `stwu r1,-64(r1)`; `mfspr r0,LR` / `stw r0,68(r1)`;
`stfd f31,48` + `psq_st f31,56`; `stw r31,44` / `r30,40`.
Epilogue `0x80747BF0..0x80747C0C`.

**Shape.** Build a position from the actor's `(x, y, z)`, replace `x` with a
camera-relative height biased by `+/-1`, and submit an effect named `282` at
that point.

1. `0x80747B58-0x80747B80` - `r30 = this`;
   `pos = (*(f32*)(this+0xAC), *(f32*)(this+0xB0), *(f32*)(this+0xB4))` into
   `r1+8/0xC/0x10`; `r6 = *(u32*)0x80429E00`;
   `f2 = *(f32*)(r6+0x40)`; `f1 = 0.5`; `f0 = 128.0`.
2. `0x80747B84-0x80747B90` - `index = (*(u32*)(this+4) >> 4) & 1`
   (`rlwinm r31,r0,28,31,31`); `f31 = 128.0 + 0.5 * *(f32*)(r6+0x40)`
   (`fmuls f1,f1,f2` then `fadds f31,f0,f1`).
3. `0x80747B94` - `0x800B30C0()` returns the camera height in `f1`
   (`fn_800b30c0`: `camera+0x38 + camera+0x40 * step`).
4. `0x80747B98-0x80747BE0` - signed-byte table and double ladder:
   ```
   sign = (s8)*(u8*)(0x8042C1A0 + index);        // {+1, -1}
   pos[0] = f1 + f31 * (f32)sign;                // stfs f0,8(r1)
   ```
   The conversion is the standard `0x43300000_<sign^0x8000>` - `(2^52+2^31)`
   `fsubs` ladder; `fmuls f0,f31,f0` then `fadds f0,f1,f0`.
5. `0x80747BE4-0x80747BEC` - `0x80064610(282, index, &pos, 0, 0)`.
   `0x80064610` is `0x80064600(r7)` + `0x8006CA80(id, variant, &pos, 0)`, i.e.
   an effect spawn keyed by `282`; the second argument is the same bit-4 index,
   not the sign.
6. `0x80747BF0-0x80747C0C` - epilogue.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80747B94` | `0x800B30C0` | none -> `f1` (camera height, `fn_800b30c0`) |
| `0x80747BEC` | `0x80064610` | `r3 = 282`, `r4 = index`, `r5 = &r1+8`, `r6 = 0`, `r7 = 0` |

No virtual calls. Two `psq*` words, `qr0`/`W=0` (one `f31` pair).

**Constants:**

| value | image address | materialised at |
|---|---|---|
| manager pointer | `0x80429E00` | `lis r6,0x8043` @`0x80747B38` + `lwz r6,-25088(r6)` @`0x80747B6C` |
| `f32 0.5` | `0x807CE48C` | `lis r5,0x807D` @`0x80747B3C` + `lfs f1,-7028(r5)` @`0x80747B74` |
| `f32 128.0` | `0x807CE488` | `lis r4,0x807D` @`0x80747B44` + `lfs f0,-7032(r4)` @`0x80747B7C` |
| `f64 2^52 + 2^31` | `0x807CE490` | `lfd f2,-7024(r4)` @`0x80747BBC` |
| `u8[2] = {0x01, 0xFF}` | `0x8042C1A0` | `lis r3,0x8043` @`0x80747B98` + `addi r3,r3,-15968` @`0x80747BA0` + `lbzx r6,r3,r31` @`0x80747BA8` |
| effect id `282` | immediate | `li r3,0x11A` @`0x80747BB4` |
| sign index | `(this+4 >> 4) & 1` | `rlwinm r31,r0,28,31,31` @`0x80747B8C` |

`0x80429E00` is `main.dol` BSS (no shipped bytes); `0x8042C1A0` is shipped in
`main.elf` PH11 and reads `01 FF 00 00`, so `table[0] = +1`, `table[1] = -1`.

**Residual ambiguity -> conditional (medium).** The arithmetic, the table read,
the camera call and the spawn are literal. Two items are inferred:
(a) `r4 = index` is the spawn *variant*, not the sign - the sign has already
been folded into `pos[0]`; (b) `pos[0]` is deliberately overwritten with the
height, so the submitted point is `(height, y, z)`, not the actor position.
Write `// unknown: 0x80064610(effect=282, variant, &[f32;3], 0, 0)` and keep
the `128.0 + 0.5 * mgr[0x40]` bias as written (it is a height, not a radius).

---

## 8. `0x8069F560` - 208 bytes - `work/ghidra/rel-d_basesNP/export/8069f560_FUN_8069f560.c`

**Body:** `0x8069F560..0x8069F62C` (52 words, `blr` at `0x8069F62C`).
Ledger and `src/game/bases.rs`: absent (zero mentions).

```rust
pub unsafe fn fn_8069f560(this: usize)
```

**Frame.** `stwu r1,-128(r1)`; `mfspr r0,LR` / `stw r0,132(r1)`;
`addi r11,r1,128`; `bl 0x802DCD70` at `0x8069F570` (saves **r26-r31**, returns
with `r3` intact); `or r26,r3,r3`. Epilogue `0x8069F618..0x8069F62C`
(`addi r11,r1,128`, `bl 0x802DCDBC`, ...).

**Shape.** Reset two scalar fields, rebuild a 3x3 basis from the cast object at
`this+0x544`, normalise its three columns, and flag `+0x360`.

1. `0x8069F574-0x8069F580` - `f0 = 0.0` (`0x807CA1FC`); `r0 = 0`;
   `r26 = this`.
2. `0x8069F584-0x8069F590` - `*(u16*)(this+0x744) = 0`;
   `*(f32*)(this+0x11C) = 0.0`.
3. `0x8069F58C-0x8069F598` - `0x80168180(this+0x544, 2, &r1+0x38)`
   (`fn_80168180`: cast the `+4` object with descriptor `0x802F8FD8`; the callee
   fills a 3x4 float matrix at the out pointer).
4. `0x8069F59C-0x8069F60C` - three-column normalisation, `i = 0..2`:
   ```
   v = { matrix[i], matrix[4+i], matrix[8+i] };   // column i of the 3x3 block
   0x80170980(&v);                                // fn_80170980: normalise
   matrix[i] = v[0]; matrix[4+i] = v[1]; matrix[8+i] = v[2];
   ```
   The matrix base is `r1+0x38`; the loop advances the three pointers by 4 each
   time, so the write-back positions are exactly `+4i`, `+0x10+4i`, `+0x20+4i`.
5. `0x8069F610-0x8069F614` - `*(u32*)(this+0x360) = 1`.
6. `0x8069F618-0x8069F62C` - epilogue.

**Dead stores.** `f0/f1/f2` are also written to `r1+8/0xC/0x10` at
`0x8069F5C8-0x8069F5D0`; nothing reads that scratch. Drop it.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x8069F570` | `0x802DCD70` | save r26-r31; `r11 = r1+128` |
| `0x8069F598` | `0x80168180` | `r3 = this+0x544`, `r4 = 2`, `r5 = &r1+0x38` |
| `0x8069F5D8` | `0x80170980` | `r3 = &r1+0x14+0xC*i` (normalise a 3-vector) |
| `0x8069F61C` | `0x802DCDBC` | restore r26-r31; `r11 = r1+128` |

No virtual calls, no `psq`.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 0.0` | `0x807CA1FC` | `lis r4,0x807D` @`0x8069F574` + `lfs f0,-24068(r4)` @`0x8069F57C` |
| cast object | `this+0x544` | `addi r3,r3,1348` @`0x8069F594` |
| reset word | `this+0x744` | `sth r0,1860(r3)` @`0x8069F584` |
| reset float | `this+0x11C` | `stfs f0,284(r3)` @`0x8069F590` |
| matrix scratch | `r1+0x38`, 12 floats | `addi r5,r1,56` @`0x8069F588` |
| done flag | `this+0x360` | `stw r0,864(r26)` @`0x8069F614` |

**Confidence: high.** All 52 words decode, the save/restore pair is a straight
`_savegpr` helper, and the three columns, the `+0x360` flag and both scalar
resets are literal. `0x80168180` is already translated
(`fn_80168180(this, arg, out)`); pass the 48-byte matrix buffer.

---

## 9. `0x8069F9D0` - 208 bytes - `work/ghidra/rel-d_basesNP/export/8069f9d0_FUN_8069f9d0.c`

**Body:** `0x8069F9D0..0x8069FA9C` (52 words, `blr` at `0x8069FA9C`).
Ledger and `src/game/bases.rs`: absent (zero mentions).

```rust
pub unsafe fn fn_8069f9d0(this: usize)
```

**This is a relocated copy of §8.** A word-for-word comparison of the two
bodies shows exactly **four** differing words, and all four are `bl`
displacements shifted by the relocation delta `0x470` (the addresses differ by
`0x8069F9D0 - 0x8069F560 = 0x470`, i.e. `0x11C` words):

| offset in body | `0x8069F560` | `0x8069F9D0` |
|---|---|---|
| `+0x10` | `bl 0x802DCD70` (`0x4BC3D801`) | `bl 0x802DCD70` (`0x4BC3D391`) |
| `+0x38` | `bl 0x80168180` (`0x4BAC8BE9`) | `bl 0x80168180` (`0x4BAC8779`) |
| `+0x78` | `bl 0x80170980` (`0x4BAD13A9`) | `bl 0x80170980` (`0x4BAD0F39`) |
| `+0xBC` | `bl 0x802DCDBC` (`0x4BC3D7A1`) | `bl 0x802DCDBC` (`0x4BC3D331`) |

Everything else, including the two prologue clears, the three-column
normalisation and the trailing `li r0,1` / `stw r0,0x360(r26)`, is identical.
So this is the same function emitted twice: implement it by calling the §8
helper, or by copying §8's body verbatim. The SHA-1 of the byte range differs
(`3645c59f...` vs `5313d1dd...`) precisely because of those four branch words,
which is why the wave-127 duplicate scan - which hashed candidates against the
already-translated ledger bodies only - did not flag it.

**Frame, shape, callees, constants:** exactly as §8
(`stwu r1,-128`; `bl 0x802DCD70`; `0x80168180(this+0x544, 2, &r1+0x38)`;
three `0x80170980` column normalisations; `*(u32*)(this+0x360) = 1`;
`bl 0x802DCDBC`). The only differences visible to an implementer are the four
call sites, which resolve to the same targets.

**Confidence: high.** Byte-identical dataflow to §8 modulo relocation; no new
constant or callee.

---

## 10. `0x806B4F90` - 208 bytes - `work/ghidra/rel-d_basesNP/export/806b4f90_FUN_806b4f90.c`

**Body:** `0x806B4F90..0x806B505C` (52 words, `blr` at `0x806B505C`).
Ledger and `src/game/bases.rs`: absent (zero mentions).

```rust
pub unsafe fn fn_806b4f90(this: usize)
```

**Frame.** `stwu r1,-80(r1)`; `mfspr r0,LR` / `stw r0,84(r1)`;
`stfd f31,64` + `psq_st f31,72`; `stfd f30,48` + `psq_st f30,56`;
`stw r31,44`; `r31 = this`. Epilogue `0x806B503C..0x806B505C`.

**Shape.** Emit the `"Wm_en_landsmoke_s"` effect at the actor's `(x, +0x6C4,
z)` with three transform scalars as parameters.

1. `0x806B4FB4-0x806B4FD0` - `pos = (*(f32*)(this+0xAC), *(f32*)(this+0xB0),
   *(f32*)(this+0xB4))` at `r1+0x14/0x18/0x1C`; then
   `*(f32*)(r1+0x18) = *(f32*)(this+0x6C4)` (**y is replaced by `+0x6C4`**).
2. `0x806B4FD4-0x806B4FE8` - call 1: `f30 = vmethod0(this, 0x2AC)(this)`.
3. `0x806B4FEC-0x806B5000` - call 2: `f31 = vmethod0(this, 0x2AC)(this)`.
4. `0x806B5004-0x806B5018` - call 3: `f3 = vmethod0(this, 0x2AC)(this)`.
   Each site reloads `*(u32*)(this+0x60)` and adds `0x2AC`; the three calls are
   distinct and must all be emitted.
5. `0x806B5014-0x806B5038` - parameter block at `r1+8`:
   `[0] = f3`, `[1] = f31`, `[2] = f30`; then
   `0x8016C890(0x807F5824, 0, &pos, 0, &params)`.
6. `0x806B503C-0x806B505C` - epilogue.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x806B4FE0` | `*(u32*)(*(u32*)(this+0x60)+0x2AC)` | `r3 = this` -> `f1` |
| `0x806B4FF8` | same slot | `r3 = this` -> `f1` |
| `0x806B5010` | same slot | `r3 = this` -> `f1` |
| `0x806B5038` | `0x8016C890` | `r3 = 0x807F5824`, `r4 = 0`, `r5 = &r1+0x14`, `r6 = 0`, `r7 = &r1+8` |

Three virtual calls, four `psq*` words (`qr0`/`W=0`, `f31`/`f30` pairs).

**Constants:**

| value | image address | materialised at |
|---|---|---|
| effect name `"Wm_en_landsmoke_s"` | `0x807F5824` | `lis r3,0x807F` @`0x806B5014` + `addi r3,r3,22564` @`0x806B501C` |
| vtable pointer | `this+0x60` | `lwz r12,96(r3)` @`0x806B4FD4` |
| slot | `+0x2AC` | `lwz r12,684(r12)` @`0x806B4FD8` |
| height override | `this+0x6C4` | `lfs f0,1732(r3)` @`0x806B4FCC` |
| spawn call | `0x8016C890` | `bl` @`0x806B5038` (`fn_8016c890(a,b,c,d,e)`) |

`0x807F5824` is a shipped NUL-terminated string in PH4
(`0x807D0C00..0x808228B8`); the log string is exactly
`"Wm_en_landsmoke_s"` (18 bytes plus NUL, followed by `"lemmy_ball"`).

**Confidence: medium.** All 52 words decode and the three calls, the height
override and the five-argument spawn are literal. What is *not* in the artifact
is the contract of vtable slot `0x2AC` (three consecutive scalar getters on
`this`, each called with the same receiver) and the meaning of the three
parameter floats inside `0x8016C890`. Emit
`// unknown: vmethod(this, 0x2AC) x3 -> three f32 parameters` and keep the
store order `(f3, f31, f30)`.

---

## 11. `0x80781B60` - 208 bytes - `work/ghidra/rel-d_basesNP/export/80781b60_FUN_80781b60.c`

**Body:** `0x80781B60..0x80781C2C` (52 words, `blr` at `0x80781C2C`).
Ledger: absent. `src/game/bases.rs`: no definition; one doc-comment mention.

```rust
pub unsafe fn fn_80781b60(out: *mut f32, actor: usize)
```

**Argument map.** `r3 -> out` (`or r29,r3,r3` @`0x80781B80`; the destination
triple), `r4 -> actor` (`or r30,r4,r4` @`0x80781B78`).

**Frame.** `stwu r1,-80(r1)`; `mfspr r0,LR` / `stw r0,84(r1)`;
`stw r31,76` / `r30,72` / `r29,68`. No `psq`, no save-helper.
Epilogue `0x80781C14..0x80781C2C`.

**Shape.** A mode-selected "copy the actor's transformed position into `out`"
helper; the mode comes from a character/control byte and selects the angle bias.

1. `0x80781B68-0x80781B88` - `r31 = *(u32*)0x8042A18C` (manager cell);
   `r3 = 0`; `0x800FC040()`.
2. `0x80781B8C-0x80781B9C` - `mode = r3`:
   `mode == 0` -> arm 0; `mode == 1` -> arm 1; anything else -> return.
3. **arm 0** (`0x80781BA0-0x80781BD8`):
   ```
   src[3]; 0x80775860(&src, manager);          // fn_80775860: manager position
   dst[3]; 0x80780BD0(&dst, actor, &src, 0x7FFF);
   out[0..2] = dst;
   ```
4. **arm 1** (`0x80781BDC-0x80781C10`): identical with `extra = 0x3FFF`.
5. `0x80781C14-0x80781C2C` - epilogue (no return value).

**Mode source (`0x800FC040`, decoded from `main.elf`):**
```
obj = *(u32*)(r13 - 0x54F4);
p   = 0x801002F0(obj);
if (p == 0) return 1;
c = *(u8*)(p+3);
return c == 0x30 ? 1 : (c == 0x31 ? 0 : 2);
```
So `1` = arm 1, `0` = arm 0, anything else = no write.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80781B88` | `0x800FC040` | none -> `u32` mode |
| `0x80781BA8` | `0x80775860` | `r3 = &r1+0x2C`, `r4 = manager` (arm 0) |
| `0x80781BBC` | `0x80780BD0` | `r3 = &r1+0x20`, `r4 = actor`, `r5 = &r1+0x2C`, `r6 = 0x7FFF` |
| `0x80781BE4` | `0x80775860` | `r3 = &r1+0x14`, `r4 = manager` (arm 1) |
| `0x80781BF8` | `0x80780BD0` | `r3 = &r1+8`, `r4 = actor`, `r5 = &r1+0x14`, `r6 = 0x3FFF` |

No virtual calls, no `psq`.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| manager cell | `0x8042A18C` | `lis r5,0x8043` @`0x80781B68` + `lwz r31,-24180(r5)` @`0x80781B84` |
| arm-0 angle bias | `0x7FFF` | `li r6,32767` @`0x80781BB8` |
| arm-1 angle bias | `0x3FFF` | `li r6,16383` @`0x80781BF4` |
| mode character cells | `r13-0x54F4`, `[obj+3]` | inside `0x800FC040` |

**Confidence: medium.** All 52 words decode; both arms, the two angle biases
and the three-float copies are literal. `0x80780BD0` is already translated
(`fn_80780bd0(out, this, src, extra: i16)`) and `0x80775860` writes a 3-float
manager position. The only inference is the *meaning* of `0x800FC040`'s modes
(control-character selection); the branches are exact. Keep `0x7FFF` as an
`i16` (`extra` is truncated with `sth` inside `0x80780BD0`).

---

## 12. `0x80642D20` - 204 bytes - `work/ghidra/rel-d_basesNP/export/80642d20_FUN_80642d20.c`

**Body:** `0x80642D20..0x80642DE8` (51 words, `blr` at `0x80642DE8`).
Ledger and `src/game/bases.rs`: absent (zero mentions).

```rust
pub unsafe fn fn_80642d20(this: usize)
```

**Frame.** `stwu r1,-64(r1)`; `mfspr r0,LR` / `stw r0,68(r1)`;
`stfd f31,48` + `psq_st f31,56`; `stw r31,44` / `r30,40` / `r29,36` / `r28,32`.
Epilogue `0x80642DC4..0x80642DE8`.

**Shape.** For each of four panes whose state is not 4, emit the pane actor's
position raised by 26 units through the pane's embedded submit object.

1. `0x80642D38-0x80642D58` - `f31 = *(f32*)0x807C8350` (= `26.0`);
   `r30 = this + 0x32C` (first pane object); `r29 = 0`; `r28 = this`;
   `r31 = this`.
2. `0x80642D5C-0x80642DC0` - `i = 0..3`:
   ```
   st = *(u32*)(this + 0x890 + 4*i);          // lwz r0,2192(r31)
   if (st != 4) {
       actor = *(u32*)(this + 4*st + 0x2E4);  // rlwinm r0,st,2 ; lwz r7,740(r5)
       p[0] = *(f32*)(actor+0xAC);
       p[1] = *(f32*)(actor+0xB0) + 26.0;
       p[2] = *(f32*)(actor+0xB4);
       obj = this + 0x32C + 0x114*i;
       vtable = *(u32*)obj;
       vtable[0xB0/4]()(obj, &p, 0, 0);
   }
   r29++; r30 += 0x114; r31 += 4;
   ```
   Note the `beq` at `0x80642D64` **skips** the emit when `st == 4`, and the
   index into the `+0x2E4` actor array is `st` itself (not `i`).
3. `0x80642DC4-0x80642DE8` - epilogue.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80642DAC` | `*(u32*)(*(u32*)(this+0x32C+0x114*i)+0xB0)` | `r3 = this+0x32C+0x114*i`, `r4 = &r1+8`, `r5 = 0`, `r6 = 0` |

One virtual call site in the loop. Two `psq*` words, `qr0`/`W=0`
(one `f31` pair).

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 26.0` | `0x807C8350` | `lis r4,0x807D` @`0x80642D28` + `lfs f31,-31920(r4)` @`0x80642D38` |
| pane state words | `this+0x890`, stride 4, 4 entries | `lwz r0,2192(r31)` @`0x80642D5C` |
| pane objects | `this+0x32C`, stride `0x114` (276) | `addi r30,r3,812` @`0x80642D44`, `addi r30,r30,276` @`0x80642DB4` |
| actor array | `this+0x2E4`, index `st` | `lwz r7,740(r5)` @`0x80642D78` (`r5 = this + 4*st`) |
| skip value `4` | immediate | `cmpi cr0,0,r0,4` @`0x80642D60` |

**Confidence: medium.** All 51 words decode; the four-pane loop, the state
skip, the actor indirection and the 26-unit lift are literal. The unknown is
only the contract of vtable slot `0xB0` (a four-argument "submit position"
call, same slot family as `0x8065C100`'s `+0xB0`). Emit
`// unknown: vmethod(pane, 0xB0)(pane, &[f32;3], 0, 0)`.

---

## 13. The tied 13th body: `0x8069E5F0` - 204 bytes - `work/ghidra/rel-d_basesNP/export/8069e5f0_FUN_8069e5f0.c`

Documented here because it ties §12 at 204 bytes and is the 13th largest. It is
a hand-off, not one of the twelve. `src/game/bases.rs` already names it in a
doc comment ("the receiver-less `0x802DCD70` body `0x8069E5F0` stay out"), and
it is absent from the ledger.

**Body:** `0x8069E5F0..0x8069E6B8` (51 words, `blr` at `0x8069E6B8`).

```rust
pub unsafe fn fn_8069e5f0(this: usize)
```

**Relationship to §8 and §9.** The same 3x3 rebuild-and-normalise, with two
differences: the prologue clears (`*(u16*)(this+0x744) = 0`,
`*(f32*)(this+0x11C) = 0.0`) are **absent**, and the tail forwards the finished
matrix through a virtual call instead of setting the `+0x360` flag. Concretely:

- `bl 0x802DCD70` (save r26-r31), `r26 = this`;
- `0x80168180(this+0x544, 2, &r1+0x38)`;
- normalise the three columns of the 3x3 block with `0x80170980` (identical
  loop, `r31/r30/r29/r28` all based at `r1+0x38` / `r1+0x14`);
- `vtable(this+0xD98)[0xB4](this+0xD98, &r1+0x38)`: `lwz r12,3480(r26)` /
  `addi r3,r26,3480` / `addi r4,r1,56` / `lwz r12,180(r12)` / `mtctr` / `bctr`;
- restore via `0x802DCDBC`.

Implement it by sharing §8's normaliser and skipping the two resets; do **not**
emit the `+0x360` store (it belongs to §8/§9 only). The only constants are the
cast descriptor used inside `0x80168180` and the `+0x544` / `+0xD98` offsets;
no `f32` constant is read. Verdict: **ready (high)**.

---

## Ready vs conditional

| address | verdict | confidence | blocker, if any |
|---|---|---|---|
| `0x80608DF0` | **ready** | high | none - copy loop, export agrees field-for-field |
| `0x80643470` | **ready** | high | none - three translated channel helpers + two literals |
| `0x8069F560` | **ready** | high | none - save helper and three-column normaliser both decoded |
| `0x8069F9D0` | **ready** | high | none - relocated copy of §8 (four `bl` words differ only by the `0x470` delta) |
| `0x807268C0` | **ready** | medium | decompose/recompose contract, inherited from the merged sibling `fn_80722180` |
| `0x80780A90` | **ready** | medium | field *meaning* of `+0x1D8` / state 3; all four callees are translated |
| `0x8068E440` | **ready** | medium | `0x8068EA10` (class C) and `0x80064F50` called for effect only |
| `0x806B4F90` | **ready** | medium | vtable slot `0x2AC` x3 and the three parameter floats |
| `0x80781B60` | **ready** | medium | `0x800FC040` mode meaning; both arms literal |
| `0x80642D20` | **ready** | medium | vtable slot `0xB0` contract |
| `0x8069A6E0` | **conditional** | medium | the three PS words' lane fold (`0x10000032`, `0x103E07BA`, `0x10210014`); Ghidra resolves it to `dx²+dy² < 16384` |
| `0x80747B30` | **conditional** | medium | `0x80064610`'s spawn-variant argument and the deliberate `pos[0]` height override |

**10 ready, 2 conditional.** Every body in this batch is fully decoded; the two
conditional ones are implementable today with the `// unknown:` notes above and
should be carried at `medium` in the ledger. A translation lane can take all
twelve without waiting for another artifact.

The 13th body (`0x8069E5F0`, §13) is also ready (high) - it is §8 with the two
prologue clears removed and the `+0x360` flag replaced by a `+0xD98` dispatch.

## Verification performed

**Concurrency note (2026-11-12, at write time).** The Wave 134 translation lane
for `game::bases` may be running in parallel. Every "Ledger: absent" line in
this document means "absent from `ledger/functions.csv` and from every spec
heading", not "absent from a work-in-progress Rust file at this minute".

- Candidate pool recomputed from `docs/ANALYSIS_WAVE127_BASES.md`: 121 class-B
  rows, 75 `## N. \`0xADDR\`` headings across
  `docs/SPEC_WAVE128_BASES.md`..`docs/SPEC_WAVE133_BASES.md`, 46 addresses left
  (6 556 bytes). The twelve taken here are the twelve largest.
- `ledger/functions.csv` checked **row-based**: `^game::bases,<addr>,` matches
  zero lines for all twelve. The single textual mention (`0x80747B30`) is
  listed in "Selection rule".
- `src/game/bases.rs` contains no `fn_<addr>` definition for any of the twelve;
  doc-comment mentions exist for `0x80780A90` (3), `0x8068E440` (1),
  `0x80747B30` (5), `0x806B4F90` (2), `0x80781B60` (1) and `0x80642D20` (2).
- Every byte count equals the `functions.tsv` `size` column, and every export
  file exists at the cited path.
- Extent check: for all twelve, `u32(entry + size - 4) == 0x4E800020` and no
  `functions.tsv` row starts inside the body. Total 2 688 bytes / 672 words.
- **Duplicate found inside the batch:** `0x8069F9D0` is `0x8069F560` relocated
  by `0x470`. A word-diff of the two 208-byte ranges shows exactly four
  differing words - the `bl` displacements at body offsets `+0x10`, `+0x38`,
  `+0x78`, `+0xBC` - so the byte-level SHA-1 differs
  (`3645c59f...` vs `5313d1dd...`) while the dataflow is identical. §9 records
  the four words; a translation lane can implement §8's helper and have §9
  call it.
- Paired-single audit: 21 `psq*` words, all `qr0` (`I = 0`) with `W = 0`, all
  prologue spill/reload pairs for `f31/f30` except the one **data** `psq_l` at
  `0x8069A76C`. No `qr3`, no `mtspr GQRn`; the only `mtspr` operands are `LR`
  and `CTR`.
- Cross-artifact cells read directly: `0x807C6730` = 0.0; `0x807CD278` = 1.0;
  `0x807CD280` = `0x4330000080000000` (2^52+2^31); `0x807CA0E8` = 16384.0;
  `0x807CA08C` = 0.0; `0x807C9C30` = 0.0; `0x807CE488` = 128.0;
  `0x807CE48C` = 0.5; `0x807CE490` = double bias; `0x807CA1FC` = 0.0;
  `0x807C8350` = 26.0; `0x8042C1A0` = `01 FF 00 00`; `0x8080AF18` = `"plp"`;
  `0x807F5824` = `"Wm_en_landsmoke_s"`.
- Decoder cross-checks: `0x800FC040` decoded from `work/dol/main.elf`
  (mode = 0/1/2 from the control byte); `0x80064610` = `0x80064600(r7)` +
  `0x8006CA80(id, variant, &pos, 0)`; `0x80064F50` returns a `0/1` landing
  flag; `0x80775860` writes a 3-float manager position.
- No file in the repository was written except this document; no merge or check
  tool was run. The decoder and data readers live in `%TEMP%\opencode`
  (`ppc134b.js`, `data134b.js`); one scratch script was briefly created at the
  repository root during this lane and deleted in the same command, leaving no
  tracked change.

## Hand-off

The 46-address pool is now **34** (3 868 bytes) after this lane plus the tied
13th body. The next twelve in the same order are `0x8069E5F0` (204, already
decoded in §13 - take it first), `0x806FB540` (200), `0x80725170` (196),
`0x8069A170` (184), `0x806A8AA0` (184), `0x80761880` (176), `0x80656260` (172),
`0x807481B0` (172), `0x80702CE0` (168), `0x8078B120` (168), `0x80781D10`
(152) and `0x80716590` (148). None of the remaining 34 needs a new artifact:
every one is an integer/float body with at most `f31`-only `psq` spills, the
same shape as §1, §5 and §11.
