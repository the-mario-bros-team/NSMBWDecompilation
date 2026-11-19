# Wave 129 - `game::bases` - implementation spec for the next 12 class-B addresses

Read-only lane. The only file written is this one; the throwaway decoder lives in
`%TEMP%\opencode` (`b129_ppc.js`, `b129_main.js`, `b129_dump.js`). No Rust file,
ledger row or existing document was modified.

## Why these twelve

`docs/ANALYSIS_WAVE127_BASES.md` lists 121 class-B addresses (51 060 bytes).
Wave 128 took the twelve largest (`docs/SPEC_WAVE128_BASES.md`). This wave takes
the **next twelve largest that are absent from `ledger/functions.csv` and from
the wave-128 spec**:

| # | address | TSV bytes | words | export file | verdict |
|--:|---:|---:|---:|---|---|
| 1 | `0x8070DC70` | 1196 | 299 | `8070dc70_FUN_8070dc70.c` | ready (high) |
| 2 | `0x806A53E0` | 964 | 241 | `806a53e0_FUN_806a53e0.c` | ready (high) |
| 3 | `0x80684680` | 936 | 234 | `80684680_FUN_80684680.c` | ready (high) |
| 4 | `0x8068E0B0` | 912 | 228 | `8068e0b0_FUN_8068e0b0.c` | conditional (medium) |
| 5 | `0x806FE480` | 892 | 223 | `806fe480_FUN_806fe480.c` | ready (high) |
| 6 | `0x80632DC0` | 884 | 221 | `80632dc0_FUN_80632dc0.c` | conditional (medium) |
| 7 | `0x806D3C20` | 784 | 196 | `806d3c20_FUN_806d3c20.c` | ready (high) |
| 8 | `0x806A6C50` | 780 | 195 | `806a6c50_FUN_806a6c50.c` | conditional (medium) |
| 9 | `0x807813F0` | 696 | 174 | `807813f0_FUN_807813f0.c` | ready (high) |
| 10 | `0x806DED20` | 688 | 172 | `806ded20_FUN_806ded20.c` | conditional (medium) |
| 11 | `0x807807E0` | 604 | 151 | `807807e0_FUN_807807e0.c` | ready (high) |
| 12 | `0x80724CD0` | 596 | 149 | `80724cd0_FUN_80724cd0.c` | ready (high) |

Together: **9 932 bytes**, 19.5 % of the class-B total. Every export path is
exactly `work/ghidra/rel-d_basesNP/export/<file>` relative to the repo root.

Selection notes:

* the three 892-byte class-B bodies are `0x806B5E00` and `0x8069F640` (both in
  wave 128) and `0x806FE480`, which is why #5 is the 892-byte one;
* `0x806F3350`, `0x80686550`, `0x8069B750`, `0x80642540`, `0x8070D1F0`,
  `0x806822B0`, `0x806F7BB0`, `0x8069FAB0`, `0x806A0290`, `0x806FD820`,
  `0x806B5E00`, `0x8069F640` are excluded because wave 128 already specifies
  them (section headings, not mere mentions - several of them also occur inside
  the wave-128 text as callees);
* no candidate is byte-identical to another body in this list.

## Method - facts re-verified from the shipped bytes

**Address mapping.** `d_basesNP.elf` has six `PT_LOAD` headers; the first is
`p_offset = 0x10F0`, `p_vaddr = 0x806000F0`, `filesz = 0x1C6004`, so

```
file_offset = 0x10F0 + (address - 0x806000F0) = (address - 0x80600000) + 0x1000
```

`docs/REL_ELF_MAPPING.md` is right for this REL. Bytes here were decoded with the
program-header rule, never with `address - base`.

**`functions.tsv` sizes are extents.** For all twelve the size column happens to
equal the reachable span exactly: each body decodes from its entry to a
terminating `blr` in the last word of the extent and every internal `b`/`bc`
target lies inside it. Extent and body therefore coincide here (unlike several
wave-128 candidates).

**Paired-single audit.** All twelve were scanned word by word:

| function | `psq*` ops | indices | kind |
|---|---:|---|---|
| `0x8070DC70` | 0 | - | - |
| `0x806A53E0` | 2 | `qr0`, `qr0` | `f31` save/restore at `88(r1)` |
| `0x80684680` | 4 | `qr0` x4 | `f30`/`f31` save/restore |
| `0x8068E0B0` | 8 | `qr0` x8 | `f28..f31` save/restore |
| `0x806FE480` | 0 | - | - |
| `0x80632DC0` | 6 | `qr0` x6 | `f29..f31` save/restore |
| `0x806D3C20` | 6 | `qr0` x6 | `f29..f31` save/restore |
| `0x806A6C50` | 0 | - | - |
| `0x807813F0` | 4 | `qr0` x4 | `f30`/`f31` save/restore |
| `0x806DED20` | 2 | `qr0` x2 | `f31` save/restore |
| `0x807807E0` | 0 | - | - |
| `0x80724CD0` | 2 | `qr0` x2 | `f31` save/restore |

Every `psq_l`/`psq_st` in the twelve is `W = 0`, `I = 0`, base `r1`, and pairs
with an `stfd`/`lfd` of the same register at `offset - 8`. **There is no
data-side paired-single access and no `mtspr GQRn` in any of the twelve.** The
`unaff_GQR0` blocks that Ghidra prints at the top of nine of these exports
(e.g. `8068e0b0` lines 69-142) are pure decompiler artefact: the exports consume
the *caller-saved* `f28..f31` as if they were the paired-single halves of
registers the function had just stored. Delete those blocks.

**CodeWarrior save ladder - verified in `work/dol/main.elf`.** The block at
`0x802DCD40` is `stw r14,-72(r11)` … `stw r31,-4(r11)` / `blr`, and the restore
block at `0x802DCD8C` is the mirror image. The entry points are four bytes
apart, so **`0x802DCD40 + 4k` saves `r(14+k)..r31`** and `0x802DCD8C + 4k`
restores them. Concretely (all eight values read from `main.elf`):

| call | saves | caller must set | matching restore |
|---|---|---|---|
| `0x802DCD64` | r23-r31 | `r11 = r1 + frame` | `0x802DCDB0` |
| `0x802DCD68` | r24-r31 | `r11 = r1 + frame` | `0x802DCDB4` |
| `0x802DCD6C` | r25-r31 | `r11 = r1 + frame` | `0x802DCDB8` |
| `0x802DCD70` | r26-r31 | `r11 = r1 + frame` | `0x802DCDBC` |
| `0x802DCD74` | r27-r31 | `r11 = r1 + frame` | `0x802DCDC0` |

None of them writes `r3..r13` and none saves `LR`, so after the prologue the
receiver is still entry `r3`. This is why every "receiver-less" export in this
wave is reconstructible; `iVarN = func_0x802dcdXX()` in the exports is just the
saved-register 64-bit return idiom.

**Three decoder-level details that change the reading of these bodies** (all
verified against the raw words; a naive decoder gets them wrong):

1. `rlwinm`'s mask operand lives in bits 6-10 (`MB`) and bits 1-5 (`ME`), *not*
   in the shift field. `rlwinm r5,r10,4,28,31` is `(x >> 28) & 0xF`, not
   `(x >> 4) & 0xF`.
2. `rlwinm`'s `Rc` bit is live: e.g. `0x57E5063F` is `rlwinm. r5,r31,0,24,31`,
   and the following `bne` tests `r5`, not a stale `cr0`.
3. the CodeWarrior float-compare idiom is `fcmpo cr0,a,b` + `cror 2,0,2` +
   `b<cond>` for `a <= b` and `fcmpo` + `cror 2,1,2` + `b<cond>` for `a >= b`;
   the `cror` folds `LT|EQ` (resp. `GT|EQ`) into `EQ`, and the branch then tests
   `EQ`. Reading the bare `fcmpo` is what made several wave-11..126 notes claim
   "inverted comparison".

**Out-of-image constants.** This REL's `.data` is `0x807D0C00..0x808228B8` and
its `.bss` is `0x80822900..0x80834D84` (zero `filesz`). Every `0x807Cxxxx` /
`0x807Dxxxx` / `0x808xxxxx` value quoted below was read from the image except
the ones explicitly marked **BSS cell** (`0x8082xxxx`) or **imported**
(`0x8042xxxx`, `0x8043xxxx`, main.dol SDA). BSS message cells are zero at load
and receive their `MSG` id at run time; pass the literal address, since no
control flow in this wave depends on their contents.

---

## 1. `0x8070DC70` - 1196 bytes - `work/ghidra/rel-d_basesNP/export/8070dc70_FUN_8070dc70.c`

**Body:** `0x8070DC70..0x8070E118` (299 words, last word `blr`). Ledger: absent.
`src/game/bases.rs`: absent.

```rust
pub unsafe fn fn_8070dc70(this: usize)
```

**Frame.** `stwu r1,-16(r1)`; `mfspr r0,LR` / `stw r0,20(r1)`; `stw r31,12(r1)`;
`lis r31,0x807D` / `addi r31,r31,-14080` (**r31 = `0x807CC900`**, the constant
pool); `stw r30,8(r1)`; `or r30,r3,r3` (**r30 = this**). No save helper; no
`psq`. Epilogue `0x8070E104..0x8070E118`.

**Behaviour.**

1. `0x8070DC90-0x8070DCA0` - virtual `+0x1C`: `lwz r12,1348(r3)` (`this+0x544`),
   `lwz r12,28(r12)`, `addi r3,r3,0x544`, `mtspr CTR`, `bcctrl` (a `bctrl` that
   nobody follows; the callee's return address is the next instruction).
2. `0x8070DCA4-0x8070DCB4` - virtual `+0x14` on `this+0x5C0`
   (`addi r3,r30,0x5C0`, slot `20(r12)`).
3. `0x8070DCB8-0x8070DCD4` - `0x8070C410(this)`, `0x8070C4F0(this)`,
   `0x8006CDE0(this)`, `0x800955F0(this)` (r3 = `r30` at every call).
4. `0x8070DCD8-0x8070DD0C` - fade byte:
   `r3 = *(u8*)(this+0x792)`; if `(s32)r3 < 253` -> `*(u8*)(this+0x792) = r3+2`
   and `0x8070C590(this, 0)`; else `*(u8*)(this+0x792) = 0xFF` and
   `0x8070C590(this, 1)`. (`cmpwi cr0,r3,253` / `bge` - signed.)
5. `0x8070DD10-0x8070DD28` - `0x8015F4F0(r3 = this+0x104,
   r4 = *(s16*)(0x807CC910 + (*(s32*)(this+0x628) << 1)), r5 = 0x200)`.
   The index is `rlwinm r0,r0,1,1,30` (i.e. `dir*2`, `dir = 0..1`) and the load
   is `lhax`.
6. `0x8070DD2C-0x8070DD5C` - if `*(f32*)(this+0xEC) < *(f32*)0x807CC900` (0.0)
   and `0x8070C340(this) != 0`: `*(u32*)(this+0x628) ^= 1` (`xori ... ,1`) and
   `*(f32*)(this+0xEC) = *(f32*)0x807CC9E4` (1.75).
7. `0x8070DD60-0x8070E060` - six-way switch on `*(s32*)(this+0x6E0)`
   (`cmpwi`/`beq` chain, comparison order 0,1,2,3,4,5, default -> end).
   Let `dir = *(s32*)(this+0x6DC)`, `T908 = 0x807CC908`, `T90C = 0x807CC90C`.
   Every arm calls `0x8015F4F0(r3, r4, r5)` ("set angle, no mirror") or
   `0x8015F560(r3, r4, r5)` ("set angle, negated"), with `r3` one of
   `this+0x100` / `this+0x102` and `r5` `200` or `-200`:

   | `this+0x6E0` | arm | `dir < 0` | `dir >= 0` |
   |---:|---|---|---|
   | 0 | `0x8070DD98` | `F560(+0x100, T90C[1-dir], 200)`; `F560(+0x102, T908[1-dir], -200)` | `F4F0(+0x100, 0, 200)`; `F4F0(+0x102, T908[1-dir], 200)` |
   | 1 | `0x8070DE18` | `F4F0(+0x100, 0, 200)`; `F560(+0x102, T908[1-dir], -200)` | `F4F0(+0x100, 0, 200)`; `F4F0(+0x102, T908[1-dir], 200)` |
   | 2 | `0x8070DE8C` | `F4F0(+0x100, 0, 200)`; `F560(+0x102, T908[1-dir], -200)` | `F560(+0x100, T90C[dir], 200)`; `F560(+0x102, T908[1-dir], -200)` |
   | 3 | `0x8070DF08` | `F560(+0x100, T90C[dir], -200)`; `F560(+0x102, T908[1-dir], -200)` | `F4F0(+0x100, 0, 200)`; `F4F0(+0x102, T908[dir], 200)` |
   | 4 | `0x8070DF80` | `F4F0(+0x100, 0, 200)`; `F4F0(+0x102, T908[1-dir], 200)` | `F4F0(+0x100, 0, 200)`; `F560(+0x102, T908[dir], -200)` |
   | 5 | `0x8070DFEC` | `F4F0(+0x100, 0, 200)`; `F4F0(+0x102, T908[dir], 200)` | `F560(+0x100, T90C[1-dir], -200)`; `F560(+0x102, T908[1-dir], -200)` |

   `1-dir` is computed with `subfic r0,r0,1` and the table load is
   `lhax r4, base, r0*2`. Note that arms 0/2/3/5 mix `F560` into the *positive*
   direction too; do not "simplify" them to a single helper.
8. `0x8070E064-0x8070E100` - commit block, guarded by
   `*(u16*)(this+0x794) == 0`:
   `lhz r0,1940(r30)` then `cmpwi cr0,r0,0` / `bne 0x8070E104`, so the block
   runs only when the halfword is exactly zero.
   * `*(u8*)(this+0x6EC) = 0`
   * `nd = *(s32*)(this+0x6DC) ^ 1`; store it back to `this+0x6DC`
   * `*(u16*)(this+0x104) = 0x200`
   * `*(s32*)(this+0x6E0) = (*(s32*)(this+0x6E0) + (nd == 0 ? 1 : 5)) % 6`
     - the `%6` is the signed magic sequence `lis r3,0x2AAB` /
     `addi r0,r3,-21845` (= `0x2AAAAAAB`) / `mulhw` / `rlwinm 1,1,31` / `add` /
     `mulli 6` / `subf` at `0x8070E094-0x8070E0B8` and `0x8070E0C0-0x8070E0E4`.
   * virtual `+0xD4` on `*(this+0x60)`: `r3 = this`, `r4 = 0x8082F630`
     (**BSS cell**).

**Direct callees.**

| site | callee | arguments |
|---|---|---|
| `0x8070DCBC` | `0x8070C410` | r3 = this |
| `0x8070DCC4` | `0x8070C4F0` | r3 = this |
| `0x8070DCCC` | `0x8006CDE0` | r3 = this |
| `0x8070DCD4` | `0x800955F0` | r3 = this |
| `0x8070DCF4` / `0x8070DD0C` | `0x8070C590` | r3 = this, r4 = 0 / 1 |
| `0x8070DD28` | `0x8015F4F0` | r3 = this+0x104, r4 = T910[dir], r5 = 0x200 |
| `0x8070DD40` | `0x8070C340` | r3 = this |
| arms | `0x8015F4F0` / `0x8015F560` | as in the arm table |

**Virtual calls:** `+0x1C` on `this+0x544`; `+0x14` on `this+0x5C0`;
`+0xD4` on `*(this+0x60)`.

**Constants.**

| value | image address | materialised at |
|---|---|---|
| `f32 0.0` | `0x807CC900` | `lfs f0,0(r31)` @`0x8070DD30` |
| `f32 1.75` | `0x807CC9E4` | `lfs f0,228(r31)` @`0x8070DD50` |
| `s16` table `{0x4000,-0x4000,-0x3000,0x3000,0x1800,-0x1800}` | `0x807CC908` | `addi r4,r31,8` @`0x8070DDC4`, `lhax` |
| `s16` table (same block, +4) `{-0x4000,-0x3000,0x3000,0x1800,...}` | `0x807CC90C` | `addi r4,r31,12` @`0x8070DDA8` |
| `s16` = `0x807CC90E` (element 1 of T90C) | `0x807CC90E` | via `T90C[1]` |
| `s16` table `{-0x3000, 0x3000}` indexed by `this+0x628` | `0x807CC910` | `addi r4,r31,16` @`0x8070DD14` |
| message cell | `0x8082F630` | `lis r4,0x8083` / `addi r4,r4,-2512` @`0x8070E0EC` |
| `0x2AAAAAAB` (`%6` magic) | immediate | `lis r3,0x2AAB` @`0x8070E098`/`0x8070E0C4` |

**Confidence: high.** Two traps: (a) item 8's guard is `*(u16*)(this+0x794) == 0`
(the `lhz`/`cmpwi`/`bne` triple is a plain equality test, not a sign test - do
not "correct" it to a `<` comparison); (b) six arms in item 7 reuse `F560` for
the positive direction, so a generic "mirror" helper would be wrong.

---

## 2. `0x806A53E0` - 964 bytes - `work/ghidra/rel-d_basesNP/export/806a53e0_FUN_806a53e0.c`

**Body:** `0x806A53E0..0x806A57A0` (241 words, last word `blr`). Ledger: absent.

```rust
pub unsafe fn fn_806a53e0(this: usize)
```

**Frame.** `stwu r1,-96(r1)`; LR at `100(r1)`; `stfd f31,80` + `psq_st f31,88`
(`qr0`); explicit `stw r31,76` / `stw r30,72` / `stw r29,68`;
`lis r31,0x807D` / `addi r31,r31,-23848` (**r31 = `0x807CA2D8`**);
`or r29,r3,r3` (**r29 = this**). Epilogue `0x806A5780..0x806A57A0`.

**Behaviour.** A two-state "carried object" update; all state is
`*(s32*)(this+0x360)`.

1. `0x806A540C-0x806A541C` - virtual `+0x1C` on `this+0x544`.
2. `0x806A5420-0x806A5444` - `r3 = 0x8006EF40(this+0x1EC, 0)`; if `r3 != 0` and
   `*(f32*)(this+0xEC) > 0.0` then `*(f32*)(this+0xEC) = 0.0`
   (`fcmpo` / `ble` on `[0x807CA390]`).
3. `0x806A5448-0x806A546C` - virtual `+0x33C` and `+0x340` on `*(this+0x60)`,
   `r3 = this` in both.
4. `0x806A5470-0x806A547C` - `0x8006CD90(this)`, `0x8006CDE0(this)`.
5. `0x806A5480-0x806A5498` - clamp: `f1 = [0x807CA2F8]` (3.0); if
   `*(f32*)(this+0xEC) > f1` then `*(f32*)(this+0xEC) = f1`.
6. `0x806A549C-0x806A54A0` - `0x800955F0(this)`.
7. `0x806A54A4-0x806A54D8` - `f31 = vcall +0x350 on *(this+0x60)` (returns in
   `f1`); `f1 = vcall +0x34C on *(this+0x60)`; then
   `0x8015F470(f1 = +0x34C result, f2 = f31, r3 = this+0x114)`.
8. `0x806A54DC-0x806A5510` - same pair for `+0x358` (`f31`) and `+0x354`
   (`f1`), `r3 = this+0x11C`.
9. `0x806A5514-0x806A5524` - `vcall +0x344 on *(this+0x60)`, `r3 = this`.
10. `0x806A5528-0x806A5540` - if `*(u32*)(this+0x738) != 0`:
    `0x8015F4F0(r3 = this+0x102, r4 = 0, r5 = 0x600)`.
11. `0x806A5544-0x806A5558` - dispatch on `*(s32*)(this+0x360)`: `==1` ->
    0x806A555C; `==2` -> 0x806A56AC; otherwise straight to the epilogue.
12. **state 1** (`0x806A555C-0x806A56A8`):
    * if `*(u32*)(this+0x738) != 0`:
      - `0x800A42B0(r3 = &r1+0x10, r4 = this)` fills twelve `f32` on the stack;
        copy `r1+0x10 .. r1+0x3C` verbatim into `this+0xE5C .. this+0xE88`;
      - `*(u16*)(r1+8) = *(u16*)(this+0xD4A)`; `0x8016EDD0(r3 = this+0xE5C,
        r4 = &r1+8)`;
      - `p = *(u32*)(this+0x738)`; virtual `+0x3B8` on `*(p+0x60)` with
        `r3 = p`, `r4 = this+0xE5C`.
    * `r3 = 0x80072BC0(this+0x1EC)`;
      - if `r3 == 0`: if `*(u32*)(this+0x738) != 0` then
        `0x8015F4F0(r3 = this+0x102, r4 = 0, r5 = 0x100)`;
      - else if `*(f32*)(this+0xEC) < 0.0`:
        `f = *(f32*)(this+0xE8) * [0x807CA3FC]` (0.9);
        `*(f32*)(this+0xEC) = 0.0`; `*(f32*)(this+0xE8) = f`;
        `*(u16*)(this+0x744) = 0`;
        `*(f32*)(this+0x740) = *(f32*)(this+0x740) * [0x807CA3E8]` (0.8);
        `0x806A3CD0(this)`; `*(s32*)(this+0x360) += 1`.
    * `0x806A5680`: if `*(u16*)(this+0x500) == 0` then `0x806A3CD0(this)` and
      `*(s32*)(this+0x360) += 1`.
    * `0x806A56A0`: `0x806A3BE0(this)`; end.
13. **state 2** (`0x806A56AC-0x806A577C`):
    * `o = *(u32*)(this+0x738)`;
      - if `o == 0`: virtual `+0xD4` on `*(this+0x60)`, `r3 = this`,
        `r4 = 0x8082B984` (**BSS cell**);
      - else: `f1 = vcall +0x314 on *(this+0x60)` with `r3 = this`;
        `d = *(f32*)(o+0xB0) - *(f32*)(this+0xB0)`; if `d > f1` then
        `*(f32*)(this+0xB4) = [0x807CA400]` (-32.0); `*(u32*)(this+0x738) = 0`;
        virtual `+0xD4` on `*(this+0x60)` with `r4 = 0x8082B984`.
    * `0x806A572C`: `r3 = 0x80072BC0(this+0x1EC)`; if `r3 != 0`:
      if `*(f32*)(this+0xEC) < 0.0` then
      (`*(f32*)(this+0xEC) = 0.0`; `*(f32*)(this+0xE8) *= 0.9`);
      `*(u16*)(this+0x744) = 0`;
      `*(f32*)(this+0x740) *= 0.8`.
    * `0x806A5778`: `0x806A3BE0(this)`; end.

**Direct callees.**

| site | callee | arguments |
|---|---|---|
| `0x806A5428` | `0x8006EF40` | r3 = this+0x1EC, r4 = 0 |
| `0x806A5474` | `0x8006CD90` | r3 = this |
| `0x806A547C` | `0x8006CDE0` | r3 = this |
| `0x806A54A0` | `0x800955F0` | r3 = this |
| `0x806A54D8` / `0x806A5510` | `0x8015F470` | f1, f2 = view slot results, r3 = this+0x114 / this+0x11C |
| `0x806A5540` / `0x806A567C` | `0x8015F4F0` | r3 = this+0x102, r4 = 0, r5 = 0x600 / 0x100 |
| `0x806A5570` | `0x800A42B0` | r3 = &r1+0x10, r4 = this |
| `0x806A55E4` | `0x8016EDD0` | r3 = this+0xE5C, r4 = &r1+8 |
| `0x806A5604` / `0x806A5730` | `0x80072BC0` | r3 = this+0x1EC |
| `0x806A5650` / `0x806A568C` | `0x806A3CD0` | r3 = this |
| `0x806A56A4` / `0x806A577C` | `0x806A3BE0` | r3 = this |

**Virtual calls:** `+0x1C` on `this+0x544`; `+0x33C`, `+0x340`, `+0x350`,
`+0x34C`, `+0x358`, `+0x354`, `+0x344`, `+0xD4`, `+0x314` on `*(this+0x60)`;
`+0x3B8` on `*(*(this+0x738)+0x60)`. All with `r3` = the owner object
(`this`, except `+0x3B8` where `r3 = *(this+0x738)`) and `r4 = 0x8082B984`
where noted.

**Constants.**

| value | image address | materialised at |
|---|---|---|
| `f32 0.0` | `0x807CA390` | `lfs f0,184(r31)` @`0x806A5438` |
| `f32 3.0` | `0x807CA2F8` | `lfs f1,32(r3)` with `r3 = r31+0` @`0x806A5488` |
| `f32 0.8` | `0x807CA3E8` | `lfs f0,272(r31)` @`0x806A5634` |
| `f32 0.9` | `0x807CA3FC` | `lfs f2,292(r31)` @`0x806A5628` |
| `f32 -32.0` | `0x807CA400` | `lfs f0,296(r31)` @`0x806A56E0` |
| message cell | `0x8082B984` | `lis r4,0x8083` / `addi r4,r4,-18044` @`0x806A56EC` |

**Confidence: high.** The export's `unaff_GQR0` blocks (lines 28-30, 124-126)
do not exist in the stream; the only `psq` pair is the `f31` spill. The twelve
float copies are a straight `stfs` ladder, not a `psq_st` pair, so nothing needs
a GQR word.

---

## 3. `0x80684680` - 936 bytes - `work/ghidra/rel-d_basesNP/export/80684680_FUN_80684680.c`

**Body:** `0x80684680..0x80684A24` (234 words, last word `blr`). Ledger: absent.

```rust
pub unsafe fn fn_80684680(this: usize, other: usize)
```

**This body takes two arguments**: `r3 = this`, `r4 = the other actor`. The
second is read before being copied (`lbz r0,908(r4)` at `0x806846A4`), so it is
a real parameter, not a stack artefact. Ghidra's `FUN_80684680(void)` with the
64-bit `func_0x802dcd6c()` return is the usual helper artefact.

**Frame.** `stwu r1,-80(r1)`; LR at `84(r1)`; `addi r11,r1,48`;
`stfd f31,64` + `psq_st f31,72`; `stfd f30,48` + `psq_st f30,56`;
`bl 0x802DCD6C` (saves r25-r31, **r3/r4 survive**); `lis r28,0x807D` /
`addi r28,r28,-26640` (**r28 = `0x807C97F0`**); `or r31,r3,r3` (**r31 = this**);
`or r30,r4,r4` (**r30 = other**). Epilogue `0x80684A00..0x80684A24`
(`bl 0x802DCDB8`).

**Behaviour.** Four-way spawn/impact handler on `other+8` (a `s16` actor id) and
`this+0x38C` (a byte mode).

1. `0x806846A4-0x806846BC` - `if (*(u8*)(other+0x38C) != 1)` -> jump to the id
   switch at `0x80684900`; otherwise the "carried" path below.
2. **carried path** (`0x806846C0-0x806847E4`):
   * `p = vcall +0x6C on *(other+0x60)` (`r3 = other`); `c = *(s8*)p`
     (`lbz` + `extsb`);
   * `*(u8*)(this + c + 0x686) = 1` (`stb r0,1670(r4)` with
     `r4 = r31 + c`);
   * `r = 0x806852C0(r3 = this, r4 = c)` (`cmpwi r3,0` / `beq 0x806848C0`);
   * **if `r != 0`**:
     - `if 0x80056CF0(other, 0x1C) != 0 || 0x80056CF0(other, 0x1F) != 0`:
       * `f31 = [0x807C9860]` (0.0), `f30 = [0x807C9820]` (12.0);
       * `i = 0; hit = 0;` `do { if (0x806843B0(r3 = other, r4 = this, r5 = i) == 0
         && f31 <= d && d <= f30) { hit = *(u32*)(this + i*4 + 0x69C); break; }
         i += 1; } while (i < 2);` where `d = *(f32*)(this+0xB0) -
         *(f32*)(*(u32*)(this + i*4 + 0x69C) + 0xB0)`; the two comparisons are
         the `fcmpo` + `cror 2,0,2` + `bne` idiom (i.e. `<=`);
       * `0x80684FA0(r3 = this, r4 = c)`;
       * `f = [0x807C97FC]` (-0.5);
       * if `hit != 0`: `0x80684FA0(r3 = hit, r4 = c)`;
         `f = f * [0x807C985C]` (0.5); `0x80685050(f1 = f, r3 = hit)`;
       * `0x80685050(f1 = f, r3 = this)`; end.
     - **else** -> step 4 (the class-token factor chain).
   * **else (`r == 0`)**: `if 0x806852F0(this) != 0` then
     virtual `+0x3FC` on `*(other+0x60)` with `r3 = other`, `r4 = 1`, `r5 = 2`,
     `r6 = 0`, `f1 = [0x807C981C]` (8.0), `f2 = *(f32*)(other+0xE8)`; end.
3. **id switch** (`0x80684900`): `id = *(u16*)(other+8)`;
   * `id == 0x130`: `i = *(u32*)(this+0x668)`;
     `*(u8*)(other + i + 0x5FA) = 1`;
     if `0x806852F0(this) != 0`: virtual `+0xD4` on `*(other+0x60)`
     (`r3 = other`, `r4 = 0x80828A60` **BSS cell**) and
     `*(u8*)(this+0x38D) = *(u8*)(vcall +0x6C on *(other+0x60))`; end.
     else if `*(u8*)(other + i + 0x5FA) != 0 && *(u8*)(other + i + 0x608) == 0`
     && `*(f32*)(other+0xEC) <= [0x807C9860]` (0.0):
     `0x80685050(f1 = *(f32*)(other+0xEC) * [0x807C9804] (0.18), r3 = this)`.
   * `id == 0x135`: if `0x806852F0(this) != 0 && *(u8*)0x807CA498 != 0` then
     `*(f32*)(other+0xEC) = *(f32*)0x807CA4A8`; end.
   * otherwise: if `0x806852F0(this) != 0` then
     `*(f32*)(other+0xEC) = *(f32*)0x807C9870` (5.5); end.
4. **class-token factor chain** (`0x806847E8-0x806848BC`, entered only from
   step 2's inner `else`): `f30 = *(f32*)(other+0xEC)`;
   * `q = vcall +0x28 on *(other+0x1464)` with `r3 = other+0x1464`;
   * `if (vcall +0x10 on *q with r4 = 0x8035425C) != 0`: `f30 *= [0x807C9800]`
     (0.03) -> step 5;
   * `else if (vcall +0x10 with r4 = 0x8035429C) != 0`: -> `0x80684898`,
     `f30 *= [0x807C9808]` (0.05) -> step 5;
   * `else if (vcall +0x10 with r4 = 0x8035431C) != 0`: also -> `0x80684898`,
     so **the same 0.05 factor** as `0x8035429C`;
   * `else` (`0x806848A8`): `f30 *= [0x807C9860]` (0.0).
   Only three factors are actually reached: 0.03, 0.05 and 0.0. The `0.02` at
   `0x807C980C` is *not* used by this body (Ghidra's line 125 assigns it to the
   third token; the stream refutes that).
5. `0x806848B0`: `0x80685050(f1 = f30, r3 = this)`; epilogue.

**Direct callees.**

| site | callee | arguments |
|---|---|---|
| `0x806846A0` | `0x802DCD6C` | save r25-r31; `r11 = r1+48` |
| `0x806846F0` | `0x806852C0` | r3 = this, r4 = (s8)carried id |
| `0x80684704` / `0x80684718` | `0x80056CF0` | r3 = other, r4 = 0x1C / 0x1F |
| `0x80684748` | `0x806843B0` | r3 = other, r4 = this, r5 = i |
| `0x806847A4` / `0x806847C0` | `0x80684FA0` | r3 = this / hit, r4 = (s8)carried id |
| `0x806847D4` / `0x806847E0` / `0x806848B8` | `0x80685050` | f1 = amount, r3 = actor |
| `0x806848C4` | `0x806852F0` | r3 = this |
| `0x8068491C` / `0x806849C0` / `0x806849EC` | `0x806852F0` | r3 = this |
| `0x80684A14` | `0x802DCDB8` | restore r25-r31 |

**Virtual calls:** `+0x6C` on `*(other+0x60)` (twice); `+0xD4` on
`*(other+0x60)`; `+0x3FC` on `*(other+0x60)`; `+0x28` on `*(other+0x1464)`
(four times, `r3 = other+0x1464`); `+0x10` on the returned object (three times,
`r4` = the three `0x8035xxxx` class tokens).

**Constants.**

| value | image address | materialised at |
|---|---|---|
| `f32 0.0` | `0x807C9860` | `lfs f31,112(r28)` @`0x80684728` |
| `f32 12.0` | `0x807C9820` | `lfs f30,48(r28)` @`0x8068472C` |
| `f32 -0.5` | `0x807C97FC` | `lfs f30,12(r28)` @`0x806847B0` |
| `f32 0.5` | `0x807C985C` | `lfs f0,108(r28)` @`0x806847C4` |
| `f32 0.03` | `0x807C9800` | `lfs f0,16(r3)` @`0x80684824` |
| `f32 0.05` | `0x807C9808` | `lfs f0,24(r3)` @`0x8068489C` |
| `f32 0.18` | `0x807C9804` | `lfs f0,20(r4)` @`0x806849A8` |
| `f32 8.0` | `0x807C981C` | `lfs f1,44(r5)` @`0x806848D8` |
| `f32 5.5` | `0x807C9870` | `lfs f0,128(r28)` @`0x806849F8` |
| float cell + flag byte | `0x807CA498` (byte) / `0x807CA4A8` (float) | `lis r3,0x807D` / `lbz -23400(r3)` @`0x806849D0`, `lfs f0,16(r3)` @`0x806849E0` |
| class token | `0x8035425C` | `addi r4,r4,16988` @`0x80684808` |
| class token | `0x8035429C` | `addi r4,r4,17052` @`0x8068484C` |
| class token | `0x8035431C` | `addi r4,r4,17180` @`0x80684880` |
| message cell | `0x80828A60` | `addi r4,r4,-30112` @`0x80684938` |

The three token factors are `lfs f0,16(r3)` @`0x80684824` (0.03),
`lfs f0,24(r3)` @`0x8068489C` (0.05, shared by the 2nd and 3rd tokens) and
`lfs f0,112(r28)` @`0x806848A8` (0.0), all with `r3 = r28 = 0x807C97F0`.

**Confidence: high.** Two things to preserve: (a) the loop counter is compared
with a *constant* 2 (`cmpwi cr0,r25,2`) while the array base advances 4 bytes and
the "other" pointer `r29` advances 4 as well - so both `this+0x69C+i*4`; (b) the
sign of `c` is `extsb` (`*(s8*)`), so negative ids index *before* `this+0x686`.

---

## 4. `0x8068E0B0` - 912 bytes - `work/ghidra/rel-d_basesNP/export/8068e0b0_FUN_8068e0b0.c`

**Body:** `0x8068E0B0..0x8068E43C` (228 words, last word `blr`). Ledger: absent.

```rust
pub unsafe fn fn_8068e0b0(this: usize) -> u32   // returns 1 (li r3,1 @0x8068E408)
```

**Frame.** `stwu r1,-208(r1)`; LR at `212(r1)`; `addi r11,r1,144`;
`stfd/psq_st` pairs for `f31,192/200`, `f30,176/184`, `f29,160/168`,
`f28,144/152`; `bl 0x802DCD64` (saves r23-r31, `r3` survives);
`lis r31,0x807D` / `addi r31,r31,-25624` (**r31 = `0x807C9BE8`**);
`or r29,r3,r3` (**r29 = this**). Epilogue `0x8068E404..0x8068E43C`
(`bl 0x802DCDB0`).

**Behaviour.** A spawn initialiser that unpacks `*(u32*)(this+4)`.

1. `0x8068E0E4-0x8068E150` - `w = *(u32*)(this+4)` (`r10`). Derived fields
   (all `rlwinm` from `r10`, MSB mask fields as decoded):

   | expression | instruction | destination |
   |---|---|---|
   | `idx = (w >> 6) & 0xF` | `rlwinm r7,r10,26,26,29` = `(w>>6)&0x3C` | index |
   | `(w >> 12) & 1` | `rlwinm r9,r10,20,31,31` | `*(u8*)(this+0x348)` |
   | `w & 0xFF` | `rlwinm r8,r10,0,24,31` | `*(u32*)(this+0xC3C)` |
   | `(w >> 8) & 0xF` | `rlwinm r7,r10,24,28,31` | `*(u32*)(this+0xC38)` |
   | `(w >> 20) & 0xF` | `rlwinm r5,r10,12,28,31` | used for the angle (below) |
   | `(w >> 16) & 0xF` | `rlwinm r4,r10,16,28,31` | used for the `+0xB4` offset |
   | `tblW[idx]` | `lwzx r6,r6,r7`, `r6 = r31+0x20` = `0x807C9C08` | `*(u32*)(this+0xC68)` |
   | `tblF[idx]` | `lfsx f6,r6,r7`, `r6 = r31+0x28` = `0x807C9C10` | `*(f32*)(this+0xC4C)` |

   Angle chain (`0x8068E100-0x8068E184`):
   `f1 = [0x807C9C18] (10.0) * tblF`;
   `f1 = [0x807C9C1C] (1.0) / f1`;
   `f5 = [0x807C9C18] (10.0) * f1`;
   `d  = (f64)(2^52 + ((w>>20)&0xF)) - *(f64*)0x807C9C40 (2^52)`;
   `f1 = [0x807C9C20] (32.0) * ([0x807C9C1C] + d)`;
   `f1 = f5 * f1`;
   `*(s16*)(this+0xC48) = (s16)(s32)f1` (via `fctiwz` + `stfd` + `lwz` low word).
   Note `*(f32*)(this+0xB4) = [0x807C9C24]` (-10.0) is stored *before* the
   `fctiwz`, and `if (*(u8*)(this+0x38F) == 1)` replaces it with
   `[0x807C9C28]` (-3490.0).
2. `0x8068E194-0x8068E1C0` - `*(f32*)(this+0xB4) -= [0x807C9C18] *
   (f32)((f64)(2^52 + ((w>>16)&0xF)) - *(f64*)0x807C9C40)`.
3. `0x8068E1C4` - `0x8068E820(this)`.
4. `0x8068E1CC-0x8068E258` - `f28 = [0x807C9C2C] (160.0) *
   *(f32*)(this+0xC4C)`;
   * if `*(u32*)(this+0xC3C) == 0`: virtual `+0x18` on `this+0x394`
     (`r3 = this+0x394`, `r4 = 0x8082AE70`);
   * else `h = 0x8068E8E0(this)`; if `h == 0`: virtual `+0x18` with
     `r4 = 0x8082AE30`; else `0x8068E970(r3 = this, r4 = h)` and virtual
     `+0x18` with `r4 = 0x8082AE70`.
5. `0x8068E25C-0x8068E290` - `*(u16*)(this+0x104) = *(s16*)(this+0xC7E)`
   (`lha`/`sth`);
   `vec = { *(f32*)(this+0xAC), *(f32*)(this+0xB0), f28, 0 }` built at
   `r1+0x40..0x4C`;
   `*(u32*)(this+0xC40) = 0x80066890(r3 = *(u32*)0x80429D40,
   r4 = &r1+0x40)`.
6. `0x8068E290-0x8068E2D4` - `0x8007FB80(f1 = [0x807C9C30] (0.0),
   f2 = same, f3 = f28, r3 = this+0x3D0, r4 = this, r5 = 0x8068E6A0,
   r6 = 0, r7 = 0, r8 = 1, r9 = *(u8*)(this+0x38F))`;
   then `*(u32*)(this+0x4A0) |= 0x800000` (`oris r0,r0,0x80`) and
   `0x8007F900(this+0x3D0)`.
7. `0x8068E2D8-0x8068E32C` - `f = [0x807C9C30]` (0.0):
   `*(f32*)(this+0xD0) = *(f32*)(this+0xD4) = *(f32*)(this+0xD8) = f`;
   stack vector `A = {1.0,1.0,1.0}` at `r1+0x30`, `B = {24.0, f28-8.0}` at
   `r1+0x20` (only the first two lanes are written: `32(r1)=24.0`,
   `36(r1)=f28-8.0`), and `40(r1)=-24.0`, `44(r1)=f28+1.0`.
8. `0x8068E32C-0x8068E404` - only `if (*(u32*)(this+0xC68) > 1)`:
   `for (i = 1; i < *(u32*)(this+0xC68); i++)` (bottom-tested at
   `0x8068E3F8`, back-edge `blt 0x8068E36C`), with
   `r27 = this+0x4B4+0xE4*i`, `r24 = this+0xC6E+2*i`, `r25 = this+2+2*i`,
   `r23 = this+0xE4*i`, `r26 = this+2+2*i`, `r28 = 0x807EF418+12*i`:
   * write `f30` -> `24(r1)`, `f31` -> `28(r1)`, `f28` -> `16(r1)`,
     `f29` -> `20(r1)`;
   * `0x8007FAD0(r3 = r27, r4 = this, r5 = &r1+0x18, r6 = &r1+0x10,
     r7 = *(u32*)(r28+0), r8 = *(u32*)(r28+4), r9 = *(u32*)(r28+8),
     r10 = 1, stack0 = *(u8*)(this+0x38F), stack1 = &r1+0x30)`;
   * `*(s16*)(r25 + 0xC6C) = *(s16*)(this+0xC7E) + *(s16*)(0x807C9BF6 + 2*i)`
     (i.e. `sth` at `this+0xC6E+2*i`);
   * `0x8007FD80(r3 = r27, r4 = r24)`;
   * `*(u32*)(r23+0x4A0) |= 0x800000`; `0x8007F900(r3 = r27)`.

**Direct callees.**

| site | callee | arguments |
|---|---|---|
| `0x8068E0E0` | `0x802DCD64` | save r23-r31; `r11 = r1+144` |
| `0x8068E1C8` | `0x8068E820` | r3 = this |
| `0x8068E1E8` | `0x8068E8E0` | r3 = this |
| `0x8068E21C` | `0x8068E970` | r3 = this, r4 = result of `0x8068E8E0` |
| `0x8068E28C` | `0x80066890` | r3 = `[0x80429D40]`, r4 = &r1+0x40 |
| `0x8068E2C0` | `0x8007FB80` | f1 = 0.0, f2 = 0.0, f3 = f28, r3 = this+0x3D0, r4 = this, r5 = `0x8068E6A0`, r6 = 0, r7 = 0, r8 = 1, r9 = `*(u8*)(this+0x38F)` |
| `0x8068E2D4` / `0x8068E3D8` | `0x8007F900` | r3 = this+0x3D0 / `this+0x4B4+0xE4*i` |
| `0x8068E3A8` | `0x8007FAD0` | as item 8 |
| `0x8068E3C4` | `0x8007FD80` | r3 = pane, r4 = `this+0xC6E+2*i` |
| `0x8068E42C` | `0x802DCDB0` | restore r23-r31 |

**Virtual calls:** `+0x18` on `this+0x394` (`r3 = this+0x394`), three sites.

**Constants.**

| value | image address | materialised at |
|---|---|---|
| `u32` table (16 words, see below) | `0x807C9C08` | `addi r6,r31,32` @`0x8068E108` + `lwzx` |
| `f32` table (same block +8) | `0x807C9C10` | `addi r6,r31,40` @`0x8068E0FC` + `lfsx` |
| `f32 10.0` | `0x807C9C18` | `lfs f5,48(r31)` @`0x8068E100` |
| `f32 1.0` | `0x807C9C1C` | `lfs f4,52(r31)` @`0x8068E118` |
| `f32 32.0` | `0x807C9C20` | `lfs f1,56(r31)` @`0x8068E14C` |
| `f32 -10.0` | `0x807C9C24` | `lfs f0,60(r31)` @`0x8068E11C` |
| `f32 -3490.0` | `0x807C9C28` | `lfs f0,64(r31)` @`0x8068E18C` |
| `f32 160.0` | `0x807C9C2C` | `lfs f1,68(r31)` @`0x8068E1D0` |
| `f32 0.0` | `0x807C9C30` | `lfs f0,72(r31)` @`0x8068E2D8` |
| `f32 -24.0` | `0x807C9C34` | `lfs f3,76(r31)` @`0x8068E2FC` |
| `f32 24.0` | `0x807C9C38` | `lfs f1,80(r31)` @`0x8068E304` |
| `f32 8.0` | `0x807C9C3C` | `lfs f0,84(r31)` @`0x8068E2F8` |
| `f64 2^52` (`0x4330000000000000`) | `0x807C9C40` | `lfd f3,88(r31)` @`0x8068E134` |
| `s16` step table | `0x807C9BF6` | `addi r26,r3,2` with `r3 = r31+0x0C` @`0x8068E354` |
| pane descriptor triple table | `0x807EF418 + 12*i` | `lis r4,0x807F` / `addi r4,r4,-3060` @`0x8068E330` |
| pane callback | `0x8068E6A0` | `addi r5,r5,-6496` @`0x8068E2B0` |
| message cells | `0x8082AE30`, `0x8082AE70` | `addi r4,r4,-20944` / `-20880` |
| singleton | `0x80429D40` | `lwz r3,-25280(r3)` @`0x8068E278` |

Table dump (`0x807C9C08`, 16 words; entry *n* at `0x807C9C08 + 4n`):
`5`, `9`, `0.45f`, `0.75f`, `10.0f`, `1.0f`, `32.0f`, `-10.0f`, `-3490.0f`,
`160.0f`, `0.0f`, `-24.0f`, `24.0f`, `8.0f`, `0x43300000`, `0x00000000`.
Because `tblW` starts two words before `tblF`, `tblF[idx]` is table entry
`idx+2`.

`0x807EF418 + 12*i` (three words per pane, all code pointers):
`{0x8068E070, 0x8068E060, 0x8068E050}`, `{0x8068E030, 0x8068E020, 0x8068E010}`,
`{0x8068DFF0, ...}`.

**Conditional / medium - what is not decidable from this REL:**

1. the *pair* of tables at `0x807C9C08` / `0x807C9C10` is one flat array read
   with a `+8` skew, so `tblW` is only a plausible integer for `idx` 0 and 1.
   A literal transcription (`read_u32(0x807C9C08 + 4*idx)` /
   `read_f32(0x807C9C10 + 4*idx)`) is faithful; naming them "int table" and
   "float table" without the skew is not.
2. the twelve-lane `0x800A42B0` output copied into `this+0xE5C..0xE88` has no
   visible layout (it is copied verbatim, so no guess is required, but do not
   invent a struct).
3. `0x807EF4xx` is a pane-descriptor triple table whose element type is only
   visible as three consecutive code pointers.

Everything else - every store, every branch and every callee argument - is
explicit. **Do not skip**; implement with `medium`.

---

## 5. `0x806FE480` - 892 bytes - `work/ghidra/rel-d_basesNP/export/806fe480_FUN_806fe480.c`

**Body:** `0x806FE480..0x806FE7F8` (223 words, last word `blr`). Ledger: absent.

```rust
pub unsafe fn fn_806fe480(this: usize)
```

**Frame.** `stwu r1,-32(r1)`; LR at `36(r1)`; explicit `stw r31,28` /
`stw r30,24` / `stw r29,20`; `or r31,r3,r3` (**r31 = this**). No `psq`, no save
helper. Epilogue `0x806FE7E0..0x806FE7F8`.

**Behaviour.** A four-state `MSG`-driven tutorial gate on
`*(s32*)(this+0x21C)`; `*(s32*)(this+0x214)` is a countdown timer.

1. `0x806FE49C` - `0x806FD7A0(this)` (called with `r3` untouched).
2. `0x806FE4A0-0x806FE4C4` - dispatch: `0 -> 0x806FE4C8`, `1 -> 0x806FE5F4`,
   `2 -> 0x806FE6B8`, `3 -> 0x806FE7AC`, else epilogue.
3. **state 0** (`0x806FE4C8-0x806FE5F0`):
   `*(s32*)(this+0x214) -= 1`;
   `if (*(u8*)(this+0x1D6) != 0 && 0x80196600([0x8042A488], 0x5EE) == 0
   && *(u8*)(this+0x1D6) != 0 && 0x80196600([0x8042A488], 0x5EF) == 0)
   || (*(u8*)(this+0x1D5) != 0 && 0x80196600([0x8042A488], 0x5EC) == 0)` then:
   * `if (*(u8*)(this+0x1D6) == 0)`: `0x8019BED0([0x8042A4A8], 7)`;
     `*(u8*)(this+0x6A0) = 1`;
     `for (i = 0; i < 0x12; i++)` (stride `0x10`):
     `if (*(u8*)(this+i*0x10+0x7D) != 0 &&
     (u32)(*(u32*)(this+i*0x10+0x78) - 7) > 2)` then
     `0x806FBEB0(*(u32*)(this+i*0x10+0x80))`;
     (the test is `addi r0,r3,-7` / `cmplwi r0,2` / `ble` - **unsigned**, so
     `(u32)(x-7) > 2` is true for `x >= 10` *and* for `x <= 6`);
   * `for (i = 0; i < 4; i++)`: `p = 0x8005F900(i)`; if `p != 0`:
     `0x80056C70(p, 0x5F)`; virtual `+0x3A0` on `*(p+0x60)` with `r3 = p`;
   * `*(s32*)(this+0x214) = 0x1E`; `*(s32*)(this+0x21C) = 1`.
4. **state 1** (`0x806FE5F4-0x806FE6B4`):
   `*(s32*)(this+0x214) -= 1`;
   `if ((*(u8*)(this+0x1D6) != 0 && 0x80196600([0x8042A488], 0x75F) == 0)
   || (*(u8*)(this+0x1D5) != 0 && 0x80196600([0x8042A488], 0x760) == 0))`:
   * `0x80195380([0x8042A488], 0x5F0, 1)`;
   * `if (*(u8*)(this+0x1D5) != 0)` then `0x80195380([0x8042A488], 0x5FE, 1)`;
   * `for (i = 0; i < 0x12; i++)`: `if (*(u8*)(this+i*0x10+0x7D) == 0)` then
     `0x806FBDA0(*(u32*)(this+i*0x10+0x80))`;
   * `*(s32*)(this+0x21C) = 2`; `*(s32*)(this+0x214) = 0`.
5. **state 2** (`0x806FE6B8-0x806FE7A8`):
   `if (--*(s32*)(this+0x214) > 0)` -> epilogue (the decrement is `addic.`, so
   `cr0` comes from the subtraction itself);
   * `CTR = 2` (`mtspr CTR,2`) and a `bdnz` loop that therefore runs **twice**;
     `r3 = this`, `r4 = 0`, `r5 = 0`; each pass tests nine bytes at stride
     `0x10` (`+0x7D,+0x8D,...,+0xFD`) and increments `r4` for every non-zero
     byte, then advances `r3` by `0x90`, `r5` by 8;
     after the loop: `if (r4 == 0x12)`:
     `0x80195380([0x8042A488], 0x5F1, 1)`; `*(s32*)(this+0x21C) = 3`;
     `*(s32*)(this+0x214) = 0x46`.
6. **state 3** (`0x806FE7AC-0x806FE7DC`):
   `if (--*(s32*)(this+0x214) > 0)` -> epilogue; else
   `*(s32*)(this+0x214) = 0` and virtual `+0x18` on `this+0x194`
   (`r3 = this+0x194`, `r4 = 0x8082F008` **BSS cell**).

**Direct callees.**

| site | callee | arguments |
|---|---|---|
| `0x806FE49C` | `0x806FD7A0` | r3 = this |
| `0x806FE4EC`, `0x806FE50C`, `0x806FE530` | `0x80196600` | r3 = `[0x8042A488]`, r4 = `0x5EE`/`0x5EF`/`0x5EC` |
| `0x806FE554` | `0x8019BED0` | r3 = `[0x8042A4A8]`, r4 = 7 |
| `0x806FE58C` | `0x806FBEB0` | r3 = `*(u32*)(this+i*0x10+0x80)` |
| `0x806FE5A8` | `0x8005F900` | r3 = i (0..3) |
| `0x806FE5BC` | `0x80056C70` | r3 = p, r4 = 0x5F |
| `0x806FE618`, `0x806FE63C` | `0x80196600` | r3 = `[0x8042A488]`, r4 = `0x75F`/`0x760` |
| `0x806FE658`, `0x806FE674` | `0x80195380` | r3 = `[0x8042A488]`, r4 = `0x5F0`/`0x5FE`, r5 = 1 |
| `0x806FE690` | `0x806FBDA0` | r3 = `*(u32*)(this+i*0x10+0x80)` |
| `0x806FE794` | `0x80195380` | r3 = `[0x8042A488]`, r4 = `0x5F1`, r5 = 1 |

**Virtual calls:** `+0x3A0` on `*(p+0x60)` with `r3 = p` (state 0);
`+0x18` on `this+0x194` with `r4 = 0x8082F008` (state 3).

**Constants / imported cells.**

| value | address | kind |
|---|---|---|
| singleton pointer cell | `0x8042A488` | imported (main.dol SDA) |
| singleton pointer cell | `0x8042A4A8` | imported |
| `MSG` ids | `0x5EC, 0x5EE, 0x5EF, 0x5F0, 0x5F1, 0x5FE, 0x75F, 0x760` | immediates |
| message cell | `0x8082F008` | BSS |
| timers | `0x1E` (30), `0x46` (70) | immediates |

**Confidence: high.** Notes: the countdown uses `addic.` so the `bgt` reads the
result of the subtraction, not a stored flag; the two 18-iteration loops use
`i*0x10` addressing with the counter in a register pair (`r29` count / `r30`
byte offset) and are fixed-bound.

---

## 6. `0x80632DC0` - 884 bytes - `work/ghidra/rel-d_basesNP/export/80632dc0_FUN_80632dc0.c`

**Body:** `0x80632DC0..0x80633130` (221 words, last word `blr`). Ledger: absent.

```rust
pub unsafe fn fn_80632dc0(this: usize)
```

**Frame.** `stwu r1,-272(r1)`; LR at `276(r1)`; `addi r11,r1,224`;
`stfd/psq_st` for `f31,256/264`, `f30,240/248`, `f29,224/232`;
`bl 0x802DCD68` (saves r24-r31, `r3` survives);
`lis r30,0x807C` / `addi r30,r30,31776` (**r30 = `0x807C7C20`**);
`or r26,r3,r3` (**r26 = this**). Epilogue `0x80633104..0x80633130`
(`bl 0x802DCDB4`).

**Behaviour.** Model-pane matrix synchroniser with two shapes, selected by
`*(u16*)0x80428732 == 3`.

1. `0x80632DEC-0x80632E04` - `if (*(u16*)0x80428732 != 3)` -> path B
   (`0x80632F50`).
2. **path A** (4 iterations, `0x80632E08-0x80632F4C`):
   `f29 = [0x807C7C9C]` (0.0), `f30 = [0x807C7CA8]` (1.0),
   `f31 = [0x807C7CAC]` (50.0);
   `for (i = 0; i < 4; i++)` (`r27` counter, `r31`/`r29` advance 4 bytes):
   * `n = *(s32*)(this + i*4 + 0x6F0)` (`r28 = lwz 1776(r31)`);
     `r3 = *(s32*)(this + i*4 + 0x6A4)`;
   * skip unless `n >= 0` **and** `*(s32*)(this + i*4 + 0x690) == 3`;
   * `src = *(u32*)(this + r3*4 + 0x5DC)` (`r24`);
     copy twelve words out of `src`:
     `[0x9C]=src[0x90]`, `[0xAC]=src[0xA0]`, `[0x90..0x98]=src[0x84/0x88/0x8C]`,
     `[0xA0..0xA8]=src[0x94/0x98/0x9C]`, `[0xB0..0xB8]=src[0xA4/0xA8/0xAC]`,
     `[0xBC]=src[0xB0]` (all `lwz`/`stw` pairs);
   * `*(f32*)(r1+0x54) = *(f32*)(r1+0x9C)`, `*(f32*)(r1+0x58) = *(f32*)(r1+0xAC)`,
     `*(f32*)(r1+0x5C) = f29`;
   * `0x800B32E0(r3 = &r1+0x24, r4 = &r1+0x54)`; copy the three results back
     (`0x24/0x28/0x2C` -> `0x54/0x58/0x5C`);
   * `dst = *(u32*)(this + n*4 + 0x64C)`;
     `dst[0xAC..0xB4] = r1+0x54..0x5C`; `*(f32*)(r1+0x50) = f30`;
   * `p = *(u32*)(this + r3*4 + 0x5DC)`; `dst = *(u32*)(this + n*4 + 0x64C)`;
     `dst[0x220] = *(f32*)(p+0x44)`, `dst[0x224] = *(f32*)(p+0x48)`,
     `dst[0x228] = f30`; `dst[0x25C] = f31`;
   * `*(f32*)(r1+0x10) = *(f32*)(p+0x44)`, `*(f32*)(r1+0x14) = *(f32*)(p+0x48)`,
     `*(f32*)(r1+0x48/0x4C) = same pair` (dead stores kept by the compiler).
3. **path B** (`0x80632F50-0x80633100`):
   `r31 = this + *(s32*)(this+0x6A4)*4`; `src = *(u32*)(r31+0x5DC)`;
   same twelve-word copy into `[0x60..0x8C]`, plus
   `*(f32*)(r1+0x3C) = src[0x90]`, `*(f32*)(r1+0x40) = src[0xA0]`,
   `*(f32*)(r1+0x44) = [0x807C7C9C]` (0.0);
   `0x800B32E0(r3 = &r1+0x18, r4 = &r1+0x3C)`;
   `f3/f0/f2 = 0x18/0x1C/0x20`; `f1 = [0x807C7CA8]` (1.0);
   `f0' = [0x807C7CAC]` (50.0);
   * `dst = *(u32*)(this+0x64C)`; `dst[0xAC..0xB4] = 0x18..0x20`;
   * `CTR = 2` (`mtspr CTR,2`) then a `bdnz` loop at `0x80633028..0x80633100`
     that therefore runs **twice**, with `r26 += 8` per pass. Each pass:
     - `obj = *(u32*)(this + 8*k + 0x64C)`;
       save `obj[0xAC]`, `obj[0xB0]`, `obj[0xB4]`, then write back
       `obj[0xAC] = saved[0xAC]`, `obj[0xB0] = *(f32*)(r1+0x40)`,
       `obj[0xB4] = saved[0xB4]` (i.e. only the Y lane is replaced by the
       caller's value);
     - `src = *(u32*)(r31+0x5DC)`, `obj2 = *(u32*)(this + 8*k + 0x650)`;
       `obj2[0x220] = src[0x44]`, `obj2[0x224] = src[0x48]`,
       `obj2[0x228] = 1.0`, `obj2[0x25C] = 50.0`;
     - the same three-step Y-lane replacement on `obj2`, then the same
       `+0x220/+0x224/+0x228/+0x25C` writes again after the pointer advance.

**Direct callees.**

| site | callee | arguments |
|---|---|---|
| `0x80632DE8` | `0x802DCD68` | save r24-r31; `r11 = r1+224` |
| `0x80632EC4` (path A), `0x80632FE0` (path B) | `0x800B32E0` | r3 = `&r1+0x24` / `&r1+0x18`, r4 = `&r1+0x54` / `&r1+0x3C` |
| `0x80633120` | `0x802DCDB4` | restore r24-r31 |

**Virtual calls:** none.

**Constants.**

| value | image address | materialised at |
|---|---|---|
| `f32 0.0` | `0x807C7C9C` | `lfs f29,124(r30)` @`0x80632E08` |
| `f32 1.0` | `0x807C7CA8` | `lfs f30,136(r30)` @`0x80632E10` |
| `f32 50.0` | `0x807C7CAC` | `lfs f31,140(r30)` @`0x80632E18` |
| global mode word (`u16`) | `0x80428732` | `lhz r0,-30926(r4)` with `lis r4,0x8043` |
| object vector | `this+0x5DC` table, `this+0x64C`, `this+0x650` | register field offsets |

**Conditional / medium:**

1. The twelve-word block copies are written with **byte offsets**, so the block
   types are not visible; they are copied verbatim and then handed to
   `0x800B32E0` as a 3-float (position) plus a 12-word transform, so code them
   as raw `read_u32`/`write_u32` ladders, not as a typed struct.
2. Path B's `bdnz` loop runs twice with `r26 += 8`, so its two passes address
   `this+0x64C / this+0x650` and then `this+0x654 / this+0x658`. A naive
   `for k in 0..2 { *(this + 0x64C + 8*k) }` reproduces it exactly; any
   interpretation of "one pair of objects with a 0xE4 stride" does not.
3. `0x80428732` is main.dol data; the three-way `== 3` test cannot be resolved
   inside this REL.

**Do not skip**; implement with `medium`.

---

## 7. `0x806D3C20` - 784 bytes - `work/ghidra/rel-d_basesNP/export/806d3c20_FUN_806d3c20.c`

**Body:** `0x806D3C20..0x806D3F2C` (196 words, last word `blr`). Ledger: absent.

```rust
pub unsafe fn fn_806d3c20(this: usize)
```

**Frame.** `stwu r1,-112(r1)`; LR at `116(r1)`; `addi r11,r1,64`;
`stfd/psq_st` for `f31,96/104`, `f30,80/88`, `f29,64/72`;
`bl 0x802DCD74` (saves r27-r31); after the call:
`lis r0,0x4330` / `stw r0,16(r1)` and `stw r0,24(r1)` (the two `2^52` halves);
`or r29,r3,r3` (**r29 = this**); `lis r30,0x807D` /
`addi r30,r30,-19104` (**r30 = `0x807CB560`**). Epilogue
`0x806D3F00..0x806D3F2C` (`bl 0x802DCDC0`).

**Behaviour.** A steering/spin update keyed on a "player slot" byte.

1. `0x806D3C60-0x806D3C74` - `b = 0x800441E0(this, &r1+8)`; `r31 = (u32)b`;
   `0x806D3680(this)`.
2. `0x806D3C78-0x806D3C9C` - `if ((b & 0xFF) == 0)` (the test is
   `rlwinm. r5,r31,0,24,31`): `if (0x806D3900(this, (s8)*(u8*)(this+0x430))
   != 0)` then `*(u8*)(this+0x430) = 0xFF`; -> step 6.
3. `0x806D3CA4-0x806D3CD8` - else:
   `if (((b & 0xFF) & *(u8*)(0x807CB608 + (s8)*(u8*)(this+0x430))) == 0)
   && (s8)*(u8*)(this+0x430) != -1 && 0x806D3900(this) != 0)` then
   `*(u8*)(this+0x430) = 0xFF`;
   (the table lookup is `lbzx r0,r3,r4` with `r3 = r30+0xA8 = 0x807CB608` and
   `r4 = extsb(slot)`).
4. `0x806D3CDC-0x806D3CF4` - `if ((s8)*(u8*)(this+0x430) < 0)`:
   `*(u8*)(this+0x430) = *(u8*)(r1+8)`; `0x806D39E0(this)`.
5. `0x806D3CF8-0x806D3D14` - `if ((s8)*(u8*)(this+0x430) == (s8)*(u8*)(r1+8))`
   then `*(u16*)(this+0x750) = 1`.
6. `0x806D3D18-0x806D3D2C` - `if ((s8)*(u8*)(this+0x430) < 0
   || *(u8*)(this+0x772) == 0)` -> step 10 (`0x806D3EB8`).
7. `0x806D3D30-0x806D3D88` - `a = 0x806D37D0(this)`; `prev = *(s16*)(this+0x104)`;
   `s = (s16)0x806D3840(r3 = this, r4 = (s16)(-a))`;
   `if ((s > 0xB9F && prev < 0xBA1) || (s < -0xB9F && prev > -0xBA1))`:
   `0x800439B0(r3 = this, r4 = b & 0xFF)`; `0x806D3A60(this)`.
8. `0x806D3D8C-0x806D3E00` - the rate ladder:
   `n = (u32)(-s)`; convert `n` and the literal `3000` through the
   `0x4330000080000000` (`2^52+2^31`) idiom at `0x807CB5E8`; then
   `q1 = ((f32)(n) * [0x807CB59C] (0.071)) / ((f32)(3000))`;
   `q2 = ((f32)(n) * [0x807CB5A4] (1.35)) / ((f32)(3000))`;
   `m = frsp(|q1|)`; `if (m < [0x807CB610] (0.0001)) m = [0x807CB5C8]` (0.0);
   `if (((q1 < 0.0 && 0.0 < q2) || (0.0 < q1 && q2 < 0.0))
   && m < [0x807CB5A0] (0.016)) m = [0x807CB5A0]`;
   `ok = 0x8015F470(f1 = q2, f2 = |m|, r3 = this+0xE8)`;
   `if (ok == 0 && m == 0.0)` then `0x8015F470(f1 = q2, f2 = 0.016, r3 = this+0xE8)`.
9. `0x806D3E7C-0x806D3EB4` - `lim = [0x807CB614] (0.2) + [0x807CB5A4] (1.35)`;
   `*(f32*)(this+0xF4) = lim`;
   `if (*(f32*)(this+0xE8) > lim) *(f32*)(this+0xE8) = lim;`
   `else if (*(f32*)(this+0xE8) < -lim) *(f32*)(this+0xE8) = -lim;`
   (the negative limit is computed with `fneg`).
10. `0x806D3EB8-0x806D3EC8` (the "no slot" path):
    `*(f32*)(this+0xE8) = [0x807CB5C8]` (0.0); `0x806D3840(r3 = this, r4 = 0)`.
11. `0x806D3ECC-0x806D3EFC` - tail:
    `v = (s32)(*(f32*)(this+0xE8) * [0x807CB618] (1024.0))` (`fmuls` +
    `fctiwz`);
    `*(s16*)(this+0x764) = (s16)v`;
    `*(s16*)(this+0x762) += (s16)v` (`lha`/`add`/`sth`);
    `0x806D3AC0(this)`.

**Direct callees.**

| site | callee | arguments |
|---|---|---|
| `0x806D3C48` | `0x802DCD74` | save r27-r31; `r11 = r1+64` |
| `0x806D3C68` | `0x800441E0` | r3 = this, r4 = &r1+8 |
| `0x806D3C74` | `0x806D3680` | r3 = this |
| `0x806D3C8C` | `0x806D3900` | r3 = this, r4 = (s8)slot byte |
| `0x806D3CC8` | `0x806D3900` | r3 = this (r4 stale from `extsb`) |
| `0x806D3CF4` | `0x806D39E0` | r3 = this |
| `0x806D3D34` | `0x806D37D0` | r3 = this |
| `0x806D3D48` | `0x806D3840` | r3 = this, r4 = (s16)(-a) |
| `0x806D3D80` | `0x800439B0` | r3 = this, r4 = b & 0xFF |
| `0x806D3D88` | `0x806D3A60` | r3 = this |
| `0x806D3E54` / `0x806D3E78` | `0x8015F470` | f1 = q2, f2 = rate, r3 = this+0xE8 |
| `0x806D3EC8` | `0x806D3840` | r3 = this, r4 = 0 |
| `0x806D3EFC` | `0x806D3AC0` | r3 = this |
| `0x806D3F1C` | `0x802DCDC0` | restore r27-r31 |

**Virtual calls:** none.

**Constants.**

| value | image address | materialised at |
|---|---|---|
| `u8` bit-mask table `{1,2,4,8,0x10,0x20,0x40,0x80}` | `0x807CB608` | `addi r3,r30,168` @`0x806D3CA8` |
| `f64 2^52+2^31` (`0x4330000080000000`) | `0x807CB5E8` | `lfd f4,136(r30)` @`0x806D3D98` |
| `f32 0.071` | `0x807CB59C` | `lfs f2,4(r4)` with `r4 = r30+0x38` @`0x806D3DA8` |
| `f32 0.016` | `0x807CB5A0` | `lfs f3,8(r3)` @`0x806D3E10` |
| `f32 1.35` | `0x807CB5A4` | `lfs f6,12(r4)` @`0x806D3DBC` |
| `f32 0.0` | `0x807CB5C8` | `lfs f0,104(r30)` @`0x806D3DDC` |
| `f32 0.0001` | `0x807CB610` | `lfs f0,176(r30)` @`0x806D3DDC` |
| `f32 0.2` | `0x807CB614` | `lfs f2,180(r30)` @`0x806D3E80` |
| `f32 1024.0` | `0x807CB618` | `lfs f1,184(r30)` @`0x806D3ECC` |
| literal `3000` (`0xBB8`) | immediate | `li r0,3000` @`0x806D3DA4`, `xoris` with `0x8000` |

**Confidence: high.** The two `2^52` ladders are the standard
signed-short-to-double conversions (`xoris r3,r3,0x8000` with the
`0x4330000080000000` constant), and the `0x800441E0` result byte is used in
three places (`b & 0xFF` mask, `r4` of `0x800439B0`, and the `== 0` dispatch).
The only thing worth a comment in the Rust is that `r4` of the second
`0x806D3900` call is left over from the `extsb` - pass the slot byte.

---

## 8. `0x806A6C50` - 780 bytes - `work/ghidra/rel-d_basesNP/export/806a6c50_FUN_806a6c50.c`

**Body:** `0x806A6C50..0x806A6F58` (195 words, last word `blr`). Ledger: absent.

```rust
pub unsafe fn fn_806a6c50(this: usize)
```

**Frame.** `stwu r1,-80(r1)`; LR at `84(r1)`; `addi r11,r1,80`;
`bl 0x802DCD70` (saves r26-r31, `r3` survives); `addi r0,r3,1816`
(`this+0x718`); `or r27,r3,r3` (**r27 = this**). No `psq`. Epilogue
`0x806A6F44..0x806A6F58` (`bl 0x802DCDBC`).

**Behaviour.** An erase/filter walk over the intrusive list object embedded at
`this+0x718`. The object layout, used everywhere:

| offset | meaning | use |
|---|---|---|
| `+0x718` | list vtable/head | virtual `+0x18` (refresh) and `+0x24` (erase) |
| `+0x71C` | element count / end index | loop bounds |
| `+0x720` | element pointer array | `*(u32*)(base + i*4)` |
| `+0x724` | iterator owner object | virtual `+0x18` when out of range |
| `+0x728` | array bound | bounds test of the refresh call |
| `+0x72C` | flag-byte array | `& 1` termination test |

1. `0x806A6C64-0x806A6CE4` - **scan A**: `i = 0`; loop:
   * test `i < *(u32*)(this+0x71C)` at `0x806A6C90` (`cmp cr0,r4,r0`), exit the
     scan when false (`bge 0x806A6CE8`);
   * `ok = (i >= 0) && (i < *(u32*)(this+0x728))`;
   * `if (!ok)` virtual `+0x18` on `this+0x724` (`r3 = this+0x724`);
   * test `(*(u8*)(*(u32*)(this+0x72C) + i) & 1) != 0` (`lbzx` then
     `rlwinm. r0,r0,0,31,31`); if set, exit the scan;
   * `i++` at `0x806A6C7C` (the increment block is the loop latch, so the order
     per element is: *test bound, body, flag test, increment*).
2. `0x806A6CE8-0x806A6CFC` - `r29 = i` (the element the scan stopped on);
   `r28 = this+0x718`; `r30 = 0`; `*(u32*)(r1+0x30) = r29`;
   `*(u32*)(r1+0x34) = r28`; `b 0x806A6E98` (**goto B**).
3. **B** (`0x806A6E98-0x806A6F40`) - the reconvergence point:
   * `k = *(u32*)(this+0x71C)` (`lwz r0,1820(r27)`), saved to `r1+0x08`, and
     `this+0x718` saved to `r1+0x0C`;
   * `for (k = *(u32*)(this+0x71C); k < *(u32*)(this+0x71C); k++)` - **a loop
     whose start and bound read the same field, so it never executes**. The
     export keeps it verbatim (`for (local_48 = *(int *)(iVar2 + 0x71c);
     local_48 < *(int *)(iVar2 + 0x71c); ...)`). Reproduce it as a commented
     empty loop; do not "repair" it;
   * tail test (`0x806A6F14`): `if (r28 == *(u32*)(r1+0x0C) && r29 == k)` ->
     `bl 0x802DCDBC` (restore) and `return`; else continue at **C**.
4. **C** (`0x806A6D04-0x806A6D50`) - the body:
   * `ok = (r29 >= 0) && (r29 < *(u32*)(this+0x71C))`
     (`cmpwi`/`blt` + `cmp`/`bge` pair);
   * `if (!ok)` virtual `+0x18` on `this+0x718` (`r3 = this+0x718`);
   * `p = *(u32*)(*(u32*)(this+0x720) + r29*4)`
     (`rlwinm r26,r29,2,0,29` then `lwzx`);
   * `if (0x80162D00(p) != 0)` -> step 5; else step 6.
5. `0x806A6E38-0x806A6E94` - **advance**: `r29 += 1`; then repeat
   (`0x806A6E44`): if `r29 >= *(u32*)(this+0x71C)` -> **B**; `ok`/refresh on
   `this+0x724`; if `(*(u8*)(*(u32*)(this+0x72C) + r29) & 1) == 0` then
   `r29 += 1` and repeat, else **B** (i.e. stop at the next flagged element).
6. `0x806A6D54-0x806A6E34` - **remove**:
   * same `ok`/refresh on `this+0x718` as in C;
   * virtual `+0x24` on `this+0x718` with `r3 = this+0x718`,
     `r4 = *(u32*)(this+0x720) + r29*4` (`add r4,r0,r26` with `r26 = r29*4`);
   * rescan: `for (k = 0; k < *(u32*)(this+0x71C); k++)`: `ok`/refresh on
     `this+0x724`, break when `(flag[k] & 1) != 0`
     (`0x806A6DA8-0x806A6E24`);
   * `r29 = k`; `r28 = this+0x718`; -> **B**.

So the whole body is one `loop { ... }` with three reconvergence edges into B
and one `return`; the two scans (A and the rescan) are the same emitted shape
with different flag-object owners (`this+0x724` vs `this+0x718`).

**Direct callees.**

| site | callee | arguments |
|---|---|---|
| `0x806A6C60` | `0x802DCD70` | save r26-r31; `r11 = r1+80` |
| `0x806A6D48` | `0x80162D00` | r3 = `*(u32*)(*(u32*)(this+0x720) + i*4)` |
| `0x806A6F48` | `0x802DCDBC` | restore r26-r31 |

**Virtual calls.**

| receiver | slot | r3 |
|---|---|---|
| `this+0x724` | `+0x18` | `this+0x724` |
| `this+0x718` | `+0x18` | `this+0x718` |
| `this+0x718` | `+0x24` | `this+0x718`, `r4 = elem_ptr_array + i*4` |

**Constants:** none. This function has **no image constants at all** - only the
six object offsets above and the two virtual slots.

**Conditional / medium:**

1. The first scan's increment is emitted *before* the loop test, so an
   idiomatic `for i in 0..count { ... }` with a `break` needs the refresh call
   placed after the bounds check and before the flag test; the exact order is
   `test -> (refresh if out of range) -> flag test -> increment`.
2. The `while (j < count)` inner loop at `0x806A6E98` is provably dead
   (`j` is initialised from the same field the bound reads). Implement it as a
   comment or an empty `if` - do not "repair" it, because the two instructions
   before the comparison (`refresh`) are still emitted.
3. `r28` is reloaded from `this+0x718` at two sites and compared against a
   stack copy in the loop condition; the comparison is `cmpl` (unsigned) for the
   pointer half and `cmp` (signed) for the index half.

**Do not skip**; implement with `medium`.

---

## 9. `0x807813F0` - 696 bytes - `work/ghidra/rel-d_basesNP/export/807813f0_FUN_807813f0.c`

**Body:** `0x807813F0..0x807816A4` (174 words, last word `blr`). Ledger: absent.

```rust
pub unsafe fn fn_807813f0(this: usize, a: usize, b: usize, c: usize)
```

**Four arguments**: `r3 = this`, `r4`, `r5`, `r6` are pointers to three `f32`
each (three vectors; `r26/r27/r28` after the prologue).

**Frame.** `stwu r1,-160(r1)`; LR at `164(r1)`; `addi r11,r1,128`;
`stfd/psq_st` for `f31,144/152` and `f30,128/136`;
`bl 0x802DCD6C` (saves r25-r31); `lis r30,0x807D` /
`addi r30,r30,-2552` (**r30 = `0x807CF608`**); `or r25,r3,r3`
(**r25 = this**), `or r26,r4,r4`, `or r27,r5,r5`, `or r28,r6,r6`.
Epilogue `0x80781680..0x807816A4` (`bl 0x802DCDB8`).

**Behaviour.** A fit-sphere style solver: three points in, one angle out.

1. `0x80781414-0x80781484` - build the two edge vectors
   `u = *(f32*)(r4+0..8) - *(f32*)(r5+0..8)` (lane order: `0(r4)-0(r5)` into
   `r1+0x44`, `4(r4)-4(r5)` into `r1+0x48`, `8(r4)-8(r5)` into `r1+0x4C`) and
   `v = *(f32*)(r6+0..8) - *(f32*)(r5+0..8)` (`r1+0x38/0x3C/0x40`);
   `f30 = 0x801C14C0(r3 = &r1+0x44)` (length of `u`).
2. `0x80781488-0x807814B8` - `dot = u.x*v.x + u.y*v.y + u.z*v.z`
   (`fmuls`/`fadds` ladder reading `0x44/0x38`, `0x48/0x3C`, `0x4C/0x40`);
   `0x801C14C0(r3 = &r1+0x38)` -> `f1` (length of `v`);
   `angle = (dot / f30) / f1`.
3. `0x807814C0-0x8078150C` - clamp:
   `if (angle > [0x807CF69C] (1.0))` keep `1.0`;
   `else if (angle >= [0x807CF6C0] (-1.0))` keep `angle`;
   `else` use `-1.0`; (the two branches are `ble` to the `-1.0` test and `bge`
   to "use angle");
   `f1 = 0x802E8608(clamped)` (arc cosine); `frsp`;
   `f31 = ([0x807CF7F0] (180.0) * f1) / [0x807CF7F4] (3.1415925)`.
4. `0x8078150C-0x80781584` - sign fix, entered only when
   `| |f31| - 180 | >= [0x807CF7A4] (10.0)` (`fcmpo` + `cror 2,1,2` + `bne`):
   cross product `u x v` into `r1+0x30/0x2C/0x34`
   (`u.y*v.z - v.y*u.z`, `u.z*v.x - v.z*u.x`, `u.x*v.y - v.x*u.y` in the
   compiler's order); `if (cross.x < [0x807CF698] (0.0)) f31 = -f31`.
5. `0x80781584-0x807815DC` - recompute the two edge lengths:
   `r1+0x20/0x24/0x28 = *(f32*)(r6) - *(f32*)(r5)` (per lane) and
   `r1+0x14/0x18/0x1C = *(f32*)(r4) - *(f32*)(r5)`;
   `f30 = 0x801C14C0(&r1+0x14)`; `0x801C14C0(&r1+0x20)` -> `f1`.
6. `0x807815E4-0x80781608` - `delta = (s32)(f1 - f30)` (`fsubs` + `fctiwz`);
   `if (*(s32*)(this+0x290) <= 0)` -> epilogue.
7. `0x8078160C-0x8078167C` - `for (i = 0; i < *(s32*)(this+0x290); i++)`:
   * `f1 = (f32)((f64)(2^52+2^31 + delta) - *(f64*)0x807CF798)` =
     `(f32)delta` (the `0x4330000080000000` idiom, `xoris r30,r0,0x8000`);
   * `0x807816B0(f1 = that, f2 = f31, r3 = &r1+8, r4 = this, r5 = r26,
     r6 = r27, r7 = r28, r8 = i)`;
   * `p = *(u32*)(*(u32*)(this+0x28C) + i*4)`;
     `*(f32*)(p+0) = *(f32*)(r1+8)`, `*(f32*)(p+4) = *(f32*)(r1+0xC)`,
     `*(f32*)(p+8) = *(f32*)(r1+0x10)`.

**Direct callees.**

| site | callee | arguments |
|---|---|---|
| `0x80781410` | `0x802DCD6C` | save r25-r31; `r11 = r1+128` |
| `0x80781484` | `0x801C14C0` | r3 = &r1+0x44 |
| `0x807814BC` | `0x801C14C0` | r3 = &r1+0x38 |
| `0x807814F0` | `0x802E8608` | f1 = clamped cosine |
| `0x807815DC` | `0x801C14C0` | r3 = &r1+0x14 |
| `0x807815E8` | `0x801C14C0` | r3 = &r1+0x20 |
| `0x80781648` | `0x807816B0` | f1 = (f32)delta, f2 = f31, r3 = &r1+8, r4 = this, r5/r6/r7 = the three vectors, r8 = i |
| `0x80781694` | `0x802DCDB8` | restore r25-r31 |

**Virtual calls:** none.

**Constants.**

| value | image address | materialised at |
|---|---|---|
| `f32 1.0` | `0x807CF69C` | `lfs f2,148(r30)` @`0x807814C4` |
| `f32 -1.0` | `0x807CF6C0` | `lfs f2,184(r30)` @`0x807814D8` |
| `f32 0.0` | `0x807CF698` | `lfs f0,144(r30)` @`0x8078155C` |
| `f32 10.0` | `0x807CF7A4` | `lfs f0,412(r30)` @`0x80781500` |
| `f32 180.0` | `0x807CF7F0` | `lfs f3,488(r30)` @`0x807814F8` |
| `f32 3.1415925` | `0x807CF7F4` | `lfs f1,492(r30)` @`0x807814FC` |
| `f64 2^52+2^31` (`0x4330000080000000`) | `0x807CF798` | `lfd f30,400(r30)` @`0x8078160C` |

**Confidence: high.** Two details: the clamp is `[-1.0, 1.0]` written as
"`> 1` -> 1; `>= -1` -> keep; else -1", and `0x801C14C0` is a *pure* 3-float
length helper (`r3` = pointer, result in `f1`) called five times with no side
effects, which is why the two short vectors can be rebuilt and re-measured.
`0x807816B0` is a sibling in this REL (`this+0x290` instances, `this+0x28C`
result pointers).

---

## 10. `0x806DED20` - 688 bytes - `work/ghidra/rel-d_basesNP/export/806ded20_FUN_806ded20.c`

**Body:** `0x806DED20..0x806DEFCC` (172 words, last word `blr`). Ledger: absent.

```rust
pub unsafe fn fn_806ded20(this: usize) -> u32   // returns 1 (li r3,1 @0x806DEFA8)
```

**Frame.** `stwu r1,-64(r1)`; LR at `68(r1)`; `lis r0,0x4330` / `stw r0,16(r1)`
and `stw r0,24(r1)`; `stfd f31,48` + `psq_st f31,56`;
explicit `stw r31,44` / `stw r30,40` / `stw r29,36` / `stw r28,32`;
`lis r31,0x807D` / `addi r31,r31,-17912` (**r31 = `0x807CBA08`**);
`or r30,r3,r3` (**r30 = this**); `li r29,0`. Epilogue
`0x806DEFA4..0x806DEFCC`.

**Behaviour.** Spawn initialiser (a "bumper"/"flipper" actor) that unpacks
`*(u32*)(this+4)`.

1. `0x806DED60` - `*(u32*)(this+0x654) = 0`.
2. `0x806DED64` - `0x806DF4B0(this)` (r3 untouched).
3. `0x806DED68-0x806DEDB8` - unpack `w = *(u32*)(this+4)` (`r28`):

   | expression | instruction | destination |
   |---|---|---|
   | `w >> 28` | `rlwinm r5,r28,4,28,31` | `*(u32*)(this+0x6F8)` |
   | `(w >> 24) & 0xF` | `rlwinm r4,r28,8,28,31` | `*(u32*)(this+0x658)` |
   | `(w >> 4) & 0xF` | `rlwinm r0,r28,28,28,31` | `*(u32*)(this+0x650)` |

   and `f31 = (f32)((f64)(2^52+2^31+56) - *(f64*)0x807CBA60)` = **56.0**;
   `*(f32*)(this+0x644) = (f32)((f64)(2^52+2^31+112) - *(f64*)0x807CBA60)` =
   **112.0** (the two literals are materialised with
   `li r0,56`/`li r0,112` + `xoris 0x8000`).
4. `0x806DEDBC` - `0x806DF630(r3 = this, f1 = f31)`.
5. `0x806DEDC0-0x806DEE04` - field block:
   `*(f32*)(this+0x6EC) = f31` (56.0);
   `*(f32*)(this+0x6F0) = [0x807CBA40]` (8.0);
   `*(f32*)(this+0x6F4) = [0x807CBA44]` (-8.0);
   `*(u8*)(this+0x4BE) = *(u8*)0x80429D28` (**imported**);
   `0x800DB180(f1 = 56.0, f2 = [0x807CBA48] (16.0), f3 = [0x807CBA40] (8.0),
   f4 = 16.0, f5 = [0x807CBA4C] (-16.0), r3 = this+0x474, r4 = this, r5 = 0,
   r6 = 1, r7 = 0)`.
6. `0x806DEE08` - `0x800DAE80(this+0x474)`.
7. `0x806DEE10-0x806DEE58` - "home" transform copy:
   `*(f32*)(this+0x620) = *(f32*)(this+0xAC)`,
   `*(u32*)(this+0x624) = *(u32*)(this+0xB0)`,
   `*(u32*)(this+0x628) = *(u32*)(this+0xB4)`,
   `*(u32*)(this+0x60C) = *(u32*)(this+0xB0)`,
   `*(u32*)(this+0x610) = *(u32*)(this+0xB4)`,
   `*(f32*)(this+0x608) = *(f32*)(this+0xAC) + 56.0`,
   `*(u32*)(this+0x614) = *(u32*)(this+0x618) = *(u32*)(this+0x61C) =
   [0x807CBA50]` (1.0);
   `*(u16*)(this+0x638) = *(u16*)(this+0x63A) = *(u16*)(this+0x63C) = 0`.
8. `0x806DEE5C-0x806DEE6C` - `a = (w >> 16) & 0xFF`; `b = (w >> 8) & 0xFF`;
   `if (a == 0) a = 8;` `if (b == 0) b = 8;`
9. `0x806DEE70-0x806DEE94` - `*(f32*)(this+0x648) = [0x807CBA48] (16.0) *
   (f32)((f64)(2^52 + b) - *(f64*)0x807CBA68)` (= `16.0 * b`).
10. `0x806DEE98-0x806DEEC4` - `if (*(s32*)(this+0x6F8) == 0)`:
    `m = min(a, b)` (`cmpl`/`ble`); `*(f32*)(this+0xB0) += 16.0 * m`.
11. `0x806DEE C8-0x806DEF34` - colour/scale block:
    `*(f32*)(this+0xE8) = *(f32*)(this+0xEC) = *(f32*)(this+0xF0) =
    [0x807CBA54]` (0.0);
    `*(f32*)(this+0x320) = [0x807CBA30]` (0.0);
    `*(f32*)(this+0x324) = [0x807CBA34]` (0.0);
    `*(f32*)(this+0x318) = [0x807CBA38]` (1088.0);
    `*(f32*)(this+0x31C) = [0x807CBA3C]` (1024.0);
    `*(f32*)(this+0x328) = [0x807CBA58]` (336.0);
    `*(f32*)(this+0x32C) = [0x807CBA5C]` (512.0);
    `0x800DF860(r3 = this+0x5F4, r4 = 0x40, r5 = 0x20, r6 = 2, r7 = 0x800,
    r8 = 0, r9 = 0, r10 = 8, stack0 = 0x80)`.
12. `0x806DEF38-0x806DEF5C` - `*(f32*)(this+0xDC) = *(f32*)(this+0xE0) =
    *(f32*)(this+0xE4) = [0x807CBA50]` (1.0);
    `*(u8*)(this+0x6A8) = 0`; `*(u32*)(this+0x660) = 0`;
    `*(u8*)(this+0x6A9) = 0`.
13. `0x806DEF60-0x806DEFA0` - `if (*(s32*)(this+0x6F8) == 0)`:
    `*(u32*)(this+0x658) = 0`; virtual `+0x18` on `this+0x394`
    (`r3 = this+0x394`, `r4 = 0x8082DFA8` **BSS cell**); else virtual `+0x18`
    with `r4 = 0x8082DEE8` (**BSS cell**).
14. `li r3,1` and return.

**Direct callees.**

| site | callee | arguments |
|---|---|---|
| `0x806DED64` | `0x806DF4B0` | r3 = this |
| `0x806DEDBC` | `0x806DF630` | r3 = this, f1 = 56.0 |
| `0x806DEE04` | `0x800DB180` | f1..f5 = 56.0, 16.0, 8.0, 16.0, -16.0; r3 = this+0x474, r4 = this, r5 = 0, r6 = 1, r7 = 0 |
| `0x806DEE0C` | `0x800DAE80` | r3 = this+0x474 |
| `0x806DEF34` | `0x800DF860` | r3 = this+0x5F4, r4 = 0x40, r5 = 0x20, r6 = 2, r7 = 0x800, r8 = 0, r9 = 0, r10 = 8, stack0 = 0x80 |

**Virtual calls:** `+0x18` on `this+0x394` (`r3 = this+0x394`).

**Constants.**

| value | image address | materialised at |
|---|---|---|
| `f32 0.0` | `0x807CBA30` / `0x807CBA34` / `0x807CBA54` | `lfs f6,40(r5)` @`0x806DEED8`, etc. |
| `f32 1088.0` | `0x807CBA38` | `lfs f3,12(r5)` @`0x806DEEE0` |
| `f32 1024.0` | `0x807CBA3C` | `lfs f4,8(r5)` @`0x806DEEE8` |
| `f32 8.0` | `0x807CBA40` | `lfs f3,56(r31)` @`0x806DEDC0` |
| `f32 -8.0` | `0x807CBA44` | `lfs f0,60(r31)` @`0x806DEDC8` |
| `f32 16.0` | `0x807CBA48` | `lfs f2,64(r31)` @`0x806DEDD0` |
| `f32 -16.0` | `0x807CBA4C` | `lfs f5,68(r31)` @`0x806DEDEC` |
| `f32 1.0` | `0x807CBA50` | `lfs f2,76(r31)` @`0x806DEECC` |
| `f32 336.0` | `0x807CBA58` | `lfs f1,80(r31)` @`0x806DEEF0` |
| `f32 512.0` | `0x807CBA5C` | `lfs f0,84(r31)` @`0x806DEEF4` |
| `f64 2^52+2^31` (`0x4330000080000000`) | `0x807CBA60` | `lfd f2,88(r31)` @`0x806DED88` |
| `f64 2^52` (`0x4330000000000000`) | `0x807CBA68` | `lfd f2,96(r31)` @`0x806DEE78` |
| byte cell | `0x80429D28` | `lis r8,0x8043` / `lbz r0,-25304(r8)` @`0x806DEDFC` |
| message cells | `0x8082DFA8`, `0x8082DEE8` | `addi r4,r4,-8280` / `-8472` |

**Conditional / medium:**

1. `0x800DB180` takes five float registers plus `r3..r7`; the export's
   `func_0x800db180(dVar14,dVar11,dVar12,dVar11,dVar13,param_1 + 0x474,
   param_1,0,1,0)` is right but the Rust needs the file's multi-argument
   `extern "C" fn` transmute precedent (`src/game/bases.rs:35720`).
2. `0x800DF860` is a 9-argument call with one stack argument (`0x80` at
   `8(r1)`), same precedent.
3. The two integer-to-float conversions use *different* `2^52` constants
   (`0x807CBA60` = `2^52+2^31`, `0x807CBA68` = `2^52`), because the first pair
   is built from a positive literal with `xoris 0x8000` and the second from a
   plain word. Keep them separate; do not fold to one constant.

**Do not skip**; implement with `medium`.

---

## 11. `0x807807E0` - 604 bytes - `work/ghidra/rel-d_basesNP/export/807807e0_FUN_807807e0.c`

**Body:** `0x807807E0..0x80780A38` (151 words, last word `blr`). Ledger: absent.

```rust
pub unsafe fn fn_807807e0(this: usize)
```

**Frame.** `stwu r1,-80(r1)`; LR at `84(r1)`; `addi r11,r1,80`;
`bl 0x802DCD74` (saves r27-r31); `or r30,r3,r3` (**r30 = this**).
No `psq`. Epilogue `0x80780A24..0x80780A38` (`bl 0x802DCDC0`).

**Behaviour.** A "rider pose / hand attachment" selector.

1. `0x807807F4-0x80780820` - `mgr = *(u32*)0x8042A18C` (**imported**);
   `idx = 0x80775030(r3 = mgr, r4 = *(u32*)(this+0x1F4),
   r5 = *(u32*)(this+0x200))`; `r31 = idx`;
   `val = *(u32*)(*(u32*)(this+0x294) + idx*12 + 0x298)` (`r28`).
2. `0x80780824-0x8078082C` - `if (0x80100F70() != 0)` -> step 3; else step 4.
3. `0x80780830-0x8078088C` - `0x80100200(r3 = &r1+0x14, r4 = mgr,
   r5 = *(u32*)(this+0x1F4))`; the three results are saved at
   `r1+0x2C/0x30/0x34`; `0x80100200(r3 = &r1+8, r4 = mgr,
   r5 = *(u32*)(this+0x1F8))`; results at `r1+0x20/0x24/0x28`;
   `if (*(f32*)(r1+0x0C) < *(f32*)(r1+0x30))` - the stream compares
   `first.y` (`0x30`) against `second.y` (`0x0C`) and takes the branch when
   `second.y < first.y`:
   `p = *(u32*)(*(u32*)(this+0x1CC) + 4)`;
   `val = (*(u8*)(p+0x152) == 5) ? 0x8B : 0x16`; then step 5.
4. `0x807808B0-0x807808E0` - `if (idx == 0x0E)`:
   `k = 4; if (*(u8*)(*(u32*)0x80429F7C + 0x380) != 0) k = 2;`
   `val = *(u32*)(*(u32*)(this+0x294) + k*12 + 0x298)`.
5. `0x807808E4-0x80780930` - `p = *(u32*)(*(u32*)(this+0x1CC) + 4)`;
   `cur = *(u32*)(p+0x154)`;
   `if (val != cur || (f32)0x801680C0(p+0x28) != *(f32*)(*(u32*)(this+0x294) +
   cur*12 + 0x29C))`:
   `0x8077E410(r3 = this, f1 = [0x807CF698] (0.0), r4 = idx, r5 = val)`
   (argument order in the stream: `r3 = r30`, `f1` loaded from `r4 = 0x807D`
   / `-2408`, `r4 = r31`, `r5 = r28`).
6. `0x80780934` - `0x800FDC80(this)`.
7. `0x8078093C-0x80780A1C` - attachment strings:
   * `if (idx == 6)`:
     `s = vcall +0x28 on p` (`r3 = p`);
     `*(u32*)(this+0x1AC) = 0x80103410(r3 = [0x8042A26C], r4 = 10, r5 = s,
     r6 = "wrist_l1" (0x808184FC), r7 = this+0x100, r8 = 0)`;
     `s = vcall +0x28 on p`;
     `*(u32*)(this+0x1B0) = 0x80103410(r3 = [0x8042A26C], r4 = 10, r5 = s,
     r6 = "wrist_r1" (0x80818508), r7 = this+0x100, r8 = 0)`.
   * else: `p = *(u32*)(*(u32*)(this+0x1CC) + 4)`;
     `b = *(u8*)(p+0x151)`; `s = vcall +0x28 on p`;
     `a1 = 0x80100A50(r3 = b)`; `a2 = 0x80100970(r3 = idx)`;
     `*(u32*)(this+0x1AC) = 0x80103410(r3 = [0x8042A26C], r4 = a2, r5 = s,
     r6 = a1, r7 = 0, r8 = this+0x2EC)`.
8. `0x80780A20` - `*(u32*)(this+0x2D8) = idx`.

**Direct callees.**

| site | callee | arguments |
|---|---|---|
| `0x807807F0` | `0x802DCD74` | save r27-r31; `r11 = r1+80` |
| `0x8078080C` | `0x80775030` | r3 = `[0x8042A18C]`, r4 = `*(this+0x1F4)`, r5 = `*(this+0x200)` |
| `0x80780824` | `0x80100F70` | no args |
| `0x8078083C` | `0x80100200` | r3 = &r1+0x14, r4 = mgr, r5 = `*(this+0x1F4)` |
| `0x80780864` | `0x80100200` | r3 = &r1+8, r4 = mgr, r5 = `*(this+0x1F8)` |
| `0x80780900` | `0x801680C0` | r3 = p+0x28 |
| `0x80780930` | `0x8077E410` | r3 = this, f1 = 0.0, r4 = idx, r5 = val |
| `0x80780938` | `0x800FDC80` | r3 = this |
| `0x8078097C` / `0x807809B8` | `0x80103410` | as item 7 |
| `0x807809F0` | `0x80100A50` | r3 = `*(u8*)(p+0x151)` |
| `0x807809FC` | `0x80100970` | r3 = idx |
| `0x80780A18` | `0x80103410` | as item 7 |
| `0x80780A28` | `0x802DCDC0` | restore r27-r31 |

**Virtual calls:** `+0x28` on `*p` where `p = *(u32*)(*(u32*)(this+0x1CC)+4)`
(three sites: `0x80780958`, `0x80780998`, `0x807809DC`), `r3 = p`.

**Constants.**

| value | image address | materialised at |
|---|---|---|
| `f32 0.0` | `0x807CF698` | `lfs f1,-2408(r4)` @`0x80780924` |
| string `"wrist_l1"` | `0x808184FC` | `addi r6,r6,-31492` @`0x8078096C` |
| string `"wrist_r1"` | `0x80818508` | `addi r6,r6,-31480` @`0x807809A8` |
| manager singleton | `0x8042A18C` | `lwz r27,-24180(r4)` @`0x807807FC` |
| allocator singleton | `0x8042A26C` | `lwz r3,-23956(r29)` @`0x80780968` |
| global object | `0x80429F7C` | `lwz r3,-24708(r3)` @`0x807808C0` |
| table shape | `idx*12 + 0x298` into `*(this+0x294)` | `mulli r0,r3,12` @`0x80780810` |

**Confidence: high.** Note the two different `0x80103410` argument shapes:
`(handle, 10, pose, "wrist_*", this+0x100, 0)` for index 6 and
`(handle, 0x80100970(idx), pose, 0x80100A50(byte), 0, this+0x2EC)` otherwise -
the four int arguments are shifted by one in the second form.

---

## 12. `0x80724CD0` - 596 bytes - `work/ghidra/rel-d_basesNP/export/80724cd0_FUN_80724cd0.c`

**Body:** `0x80724CD0..0x80724F20` (149 words, last word `blr`). Ledger: absent.

```rust
pub unsafe fn fn_80724cd0(this: usize)
```

**Frame.** `stwu r1,-80(r1)`; LR at `84(r1)`; `addi r11,r1,64`;
`stfd f31,64` + `psq_st f31,72`; `bl 0x802DCD74` (saves r27-r31);
`or r28,r3,r3` (**r28 = this**); `lis r30,0x8083` /
`addi r30,r30,960` (**r30 = `0x808303C0`**, the message-cell block);
`lis r31,0x807D` / `addi r31,r31,-11896` (**r31 = `0x807CD188`**).
Epilogue `0x80724F04..0x80724F20` (`bl 0x802DCDC0`).

**Behaviour.** Face-direction / turn update for a ridden object.

1. `0x80724CF0-0x80724D08` - `pl = 0x8005F900((s8)*(u8*)(this+0x38D))`
   (`r29`); `own = *(u8*)(pl+0x348)` (`r27`).
2. `0x80724D14-0x80724D40` - `ang = (s16)0x80129910()`;
   `*(s16*)(this+0x102) = (s16)(*(u32*)(0x807CD1D0 + own*4) * 0x3800)
   + ang + *(s16*)(pl+0x102)`
   (the multiply is `mulli r4,r0,14336`; the table load is `lwzx` with
   `r4 = r31+72 = 0x807CD1D0` and index `own*4`).
3. `0x80724D28` - `flags = *(u32*)(this+0x12C)`.
4. `0x80724D44-0x80724DAC` - **spin path** (only when `flags & 1`):
   `*(u8*)(this+0x348) = *(u8*)(this+0x130)`;
   if `(flags & 2) == 0`:
   `*(u8*)(this+0x348) = *(u8*)(pl+0x348) ^ 1`;
   virtual `+0x18` on `this+0x394` (`r3 = this+0x394`, `r4 = 0x80830458`
   **BSS cell**, i.e. `r30+0x98`);
   else virtual `+0x18` with `r4 = 0x808304D8` (`r30+0x118`);
   then virtual `+0x1C` on `this+0x394`; -> end.
5. `0x80724DB0-0x80724E34` - **normal path**:
   `b = (*(f32*)(pl+0xC4) >= *(f32*)0x807CD1BC (0.0))`, computed with
   `fcmpo cr0,f3,f0` + `cror 2,1,2` (which folds `GT|EQ` into `cr0.EQ`) +
   `mfcr` + `rlwinm r3,r3,3,31,31` (extract `cr0.EQ`) + `cntlzw` +
   `rlwinm r27,r3,27,24,31`. After that chain `r27 == !b`; the commits below
   write `r27 ^ 1`, i.e. `b`.
   `y = *(f32*)(pl+0xC8)`; `z = *(f32*)(pl+0xCC)`;
   `if (y > *(f32*)0x807CD1BC (0.0)
   && 0x80724580(r3 = this, r4 = &r1+0x0C) != 0
   && *(u32*)(r1+0x0C) != 0)`:
   `*(u8*)(this+0x348) = r27 ^ 1` (`xori r0,r27,1`);
   virtual `+0x18` on `this+0x394` with `r4 = 0x80830458`; -> end.
6. `0x80724E38-0x80724E48` - `if (0x80056CF0(pl, 0x46) != 0)` -> end.
7. `0x80724E4C-0x80724ECC` - `if (y > 0.0)`:
   `o = *(u32*)(pl+0x1EC)`; build `{ *(f32*)(this+0xAC), *(f32*)(this+0xB0),
   *(f32*)(this+0xB4) }` at `r1+0x10`; `f31 = [0x807CD1D8]` (17.0);
   `if (0x800728C0(r3 = pl+0x1EC, r4 = &r1+0x10, r5 = &r1+8) != 0
   && *(f32*)(r1+8) > *(f32*)(r1+0x14)
   && *(f32*)(r1+8) < *(f32*)(r1+0x14) + f31)`:
   commit `*(u8*)(this+0x348) = r27 ^ 1`, virtual `+0x18` with
   `r4 = 0x80830458`; -> end.
8. `0x80724ED0-0x80724F00` - `if (0x8006FA40(r3 = this+0x1EC, r4 = &r1+0x1C)
   != 0)`: commit the same two instructions (`this+0x348 = r27 ^ 1`,
   `+0x18` with `0x80830458`).

**Direct callees.**

| site | callee | arguments |
|---|---|---|
| `0x80724CE8` | `0x802DCD74` | save r27-r31; `r11 = r1+64` |
| `0x80724D08` | `0x8005F900` | r3 = (s8)`*(u8*)(this+0x38D)` |
| `0x80724D14` | `0x80129910` | no args |
| `0x80724DFC` | `0x80724580` | r3 = this, r4 = &r1+0x0C |
| `0x80724E40` | `0x80056CF0` | r3 = pl, r4 = 0x46 |
| `0x80724E84` | `0x800728C0` | r3 = pl+0x1EC, r4 = &r1+0x10, r5 = &r1+8 |
| `0x80724ED8` | `0x8006FA40` | r3 = this+0x1EC, r4 = &r1+0x1C |
| `0x80724F10` | `0x802DCDC0` | restore r27-r31 |

**Virtual calls:** `+0x18` on `this+0x394` (four sites, `r3 = this+0x394`,
`r4` = `0x80830458` / `0x808304D8`); `+0x1C` on `this+0x394` (spin path).

**Constants.**

| value | image address | materialised at |
|---|---|---|
| `u32` step table | `0x807CD1D0` | `addi r4,r31,72` @`0x80724D1C` + `lwzx` |
| table contents | `-1, 1, 0x41880000, 0, 0x3FA00000, 0xBFA00000, 0xBEB33333, 0xBF000000` | dump |
| `f32 0.0` | `0x807CD1BC` | `lfs f0,52(r31)` @`0x80724DB4` |
| `f32 17.0` | `0x807CD1D8` | `lfs f31,80(r31)` @`0x80724E78` |
| message cell | `0x80830458` | `addi r4,r30,152` @`0x80724E20` |
| message cell | `0x80830498` | `addi r4,r30,216` @`0x80724D7C` |
| message cell | `0x808304D8` | `addi r4,r30,280` @`0x80724D60` |

**Confidence: high with one documented caveat.** The `b` flag built at
`0x80724DC0-0x80724DF0` is `(*(f32*)(pl+0xC4) >= 0.0)`: `fcmpo cr0,f3,f0` sets
`LT/GT/EQ`, `cror 2,1,2` folds `GT|EQ` into `cr0.EQ` (so `EQ` now means `>=`),
`mfcr` + `rlwinm r3,r3,3,31,31` extracts that `EQ` bit, and the `cntlzw` +
`rlwinm r27,r3,27,24,31` pair turns it into `r27 = !b`; both commit sites then
store `r27 ^ 1`, i.e. `b`. Ghidra renders the identical chain as
`countLeadingZeros(DAT_807cd1bc <= local_34)` plus `bVar1 ^ 1`, which agrees
with this reading. What cannot be recovered is the *name* of the flag (it looks
like a "facing left/right" state read from the rider's local X); keep a
`// unknown:` note on the field, not on the value.

---

## Ready vs conditional

**Ready - implement as specified, confidence `high` (8):**
`0x8070DC70`, `0x806A53E0`, `0x80684680`, `0x806FE480`, `0x806D3C20`,
`0x807813F0`, `0x807807E0`, `0x80724CD0`.

**Conditional - implement as specified, confidence `medium` (4):**

| address | condition |
|---|---|
| `0x8068E0B0` | The `0x807C9C08` / `0x807C9C10` tables are one flat array read with a two-word skew, so only `idx` 0/1 give a plausible integer; transcribe both reads literally. The twelve-float `0x800A42B0` output and the `0x807EF418` pane-descriptor triples have no visible layout. |
| `0x80632DC0` | Twelve-word transform blocks are copied by raw offset and only their three position lanes are semantically used; path B's `bdnz` loop runs twice with `r26 += 8`, so it addresses two *pairs* of objects, not one pair with a 0xE4 stride; the `0x80428732 == 3` selector is main.dol data. |
| `0x806A6C50` | The first scan increments before the back-edge test, and the `while (j < count)` inner loop at `0x806A6E98` is provably dead (`j` is loaded from the same field as the bound). Both must be reproduced literally, including the refresh call that is emitted before the dead comparison. |
| `0x806DED20` | Two multi-argument constructors (`0x800DB180` with five float + five int arguments, `0x800DF860` with nine int arguments including one stack slot) need the file's `extern "C" fn` transmute precedent, and the two different `2^52` constants (`0x807CBA60` = `2^52+2^31`, `0x807CBA68` = `2^52`) must not be folded together. |

**No skips.** None of the twelve needs the GQR3 word, a jump-table generator
or a byte-identity check: every body ends in `blr`, every constant is at a
fixed image address, and the only non-image inputs are main.dol SDA cells
(`0x8042xxxx`, `0x8043xxxx`) and this REL's `.bss` message cells (`0x8082xxxx`).

## Verification performed

* `ledger/functions.csv` - parsed read-only; the address column
  (`module,address,...`, no `0x`) contains **none** of the twelve. The earlier
  wave-127 candidate set (121 class-B rows) was re-filtered against the ledger
  by exact address-column match, not substring, so mentions such as
  `0x80642A20` inside another row's evidence text do not remove a candidate.
* `docs/SPEC_WAVE128_BASES.md` - the twelve section headings were extracted
  (`^## N. `0xXXXXXXXX``) and subtracted; the wave-128 *mentions* of
  `0x8069E5F0` and `0x806B4F90` (both callees there) do **not** exclude those
  addresses, and indeed `0x8069E5F0` (204 B) remains a class-B candidate for a
  later wave.
* `work/ghidra/rel-d_basesNP/export/functions.tsv` - all twelve rows exist and
  the `size` column matches the decoded span exactly (299/241/234/228/223/221/
  196/195/174/172/151/149 words).
* `work/rels/elf/d_basesNP.elf` - program headers read:
  `p_offset 0x10F0`, `p_vaddr 0x806000F0`, `filesz 0x1C6004`; `.data`
  `0x807D0C00..0x808228B8`, `.bss` `0x80822900..0x80834D84` (`filesz = 0`).
  Every constant quoted above was classified against those ranges.
* `work/dol/main.elf` - the save/restore ladder
  `0x802DCD40..0x802DCD88` / `0x802DCD8C..0x802DCDD4` was decoded instruction
  by instruction, confirming that `0x802DCD40 + 4k` saves `r(14+k)..r31`,
  that `0x802DCD8C + 4k` restores them, and that none of them writes
  `r3..r13`.
* Paired-single audit: 2 483 words scanned across the twelve; every `psq_l` /
  `psq_st` is `W = 0, I = 0`, base `r1`, paired with an `stfd`/`lfd` of the same
  `f` register 8 bytes earlier/later; zero `mtspr GQRn`; zero `psq_lu`,
  `psq_stu`, `psq_lx`, `psq_stx`.
* Store audit: every `stb`/`sth`/`stw`/`stfs` in the twelve was listed with its
  base register and displacement and grouped into the tables above; all
  displacements are reproduced exactly as the stream encodes them (Ghidra's
  `local_*` stack names were only used as cross-checks - the `r1` offsets are
  the authoritative ones).
* Duplicate check: SHA-1 over each reachable span -

  | address | SHA-1 |
  |---|---|
  | `0x8070DC70` | `f76d6c52674ca6a1d5b3461b5a6361afad51c190` |
  | `0x806A53E0` | `3a70fa359aa0d02ec4a9588951236b64761d39d2` |
  | `0x80684680` | `04107347ef8887b9c910ce6932c8c66a024b58da` |
  | `0x8068E0B0` | `4aed1f77c8ed49fcbb3bdaf0a947a0dbe6f017e9` |
  | `0x806FE480` | `0c74327652af354c97acc0c55efdc1c9b8208955` |
  | `0x80632DC0` | `516b32b1bc200d8947632f50a738eeba3a1cb2ac` |
  | `0x806D3C20` | `6b2c876d6ffd00662618b21ae90a2234e2013af3` |
  | `0x806A6C50` | `2950953c6b4c6ad4d26ac095dd214183b212b59f` |
  | `0x807813F0` | `3370122090500c3473af65af66198d4df2f67940` |
  | `0x806DED20` | `e3df55294f984080821f3fe1c2895c7ca0def7c5` |
  | `0x807807E0` | `bd64cd79a66ee3024e3078132449242656136469` |
  | `0x80724CD0` | `84187a661dafbb563236a66752f5aac220699305` |

  all twelve are distinct, so none is a release-twin or a duplicate body.

## Deliberately not claimed here (class C, for a later wave)

The 24 `qr3` bodies (`0x806B7CD0`, `0x806DD730`, `0x8072E520`, `0x8072BC40`,
`0x806B9A30`, ...) still need GQR3's type/scale word from `main.dol`.
`0x807BD110` still needs the `dFileSelect_c` 44-entry state-table generator.
Both remain class C and are not part of this wave.
