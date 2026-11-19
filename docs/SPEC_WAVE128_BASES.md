# Wave 128 - `game::bases` - implementation spec for 12 class-B addresses

Read-only lane. Nothing outside this file was written except the throwaway
decoder in `%TEMP%\opencode`. No Rust file, ledger row or existing document was
touched.

## Why these twelve

`docs/ANALYSIS_WAVE127_BASES.md` reclassified 121 `d_basesNP` skips as class B
("reconstructible now from the raw stream"). This lane takes the twelve largest
of those, which together account for **14 588 of the 51 060 class-B bytes**
(28.6 %):

| # | address | bytes | export file | verdict |
|--:|---:|---:|---|---|
| 1 | `0x806F3350` | 2092 | `806f3350_FUN_806f3350.c` | ready (high) |
| 2 | `0x80686550` | 1884 | `80686550_FUN_80686550.c` | conditional (medium) |
| 3 | `0x8069B750` | 1424 | `8069b750_FUN_8069b750.c` | ready (high) |
| 4 | `0x80642540` | 1236 | `80642540_FUN_80642540.c` | ready (high) |
| 5 | `0x8070D1F0` | 1200 | `8070d1f0_FUN_8070d1f0.c` | ready (high) |
| 6 | `0x806822B0` | 1108 | `806822b0_FUN_806822b0.c` | ready (medium) |
| 7 | `0x806F7BB0` | 1004 | `806f7bb0_FUN_806f7bb0.c` | ready (high) |
| 8 | `0x8069FAB0` | 996 | `8069fab0_FUN_8069fab0.c` | ready (high) |
| 9 | `0x806A0290` | 948 | `806a0290_FUN_806a0290.c` | ready (high) |
| 10 | `0x806FD820` | 912 | `806fd820_FUN_806fd820.c` | conditional (medium) |
| 11 | `0x806B5E00` | 892 | `806b5e00_FUN_806b5e00.c` | ready (high) |
| 12 | `0x8069F640` | 892 | `8069f640_FUN_8069f640.c` | ready (high) |

Every export path below is relative to the repository root and is exactly
`work/ghidra/rel-d_basesNP/export/<file>`.

Nine are unconditionally implementable from the shipped bytes. Three
(`0x80686550`, `0x806822B0`, `0x806FD820`) are implementable but carry one
external/derived input each and are specified as **medium** for that reason;
none of the twelve needs to be skipped.

## Method and corrections to the wave-127 note

**Address mapping (verified, not copied).** `d_basesNP.elf` has six `PT_LOAD`
headers; the first is `p_offset = 0x10F0`, `p_vaddr = 0x806000F0`, so

```
file_offset = 0x10F0 + (address - 0x806000F0) = (address - 0x80600000) + 0x1000
```

The wave-127 report's line "the documented `offset = address - 0x80600000` rule
is exact" is **wrong by 0x1000**; the same report's own evidence (`lis 0x4330`
at `0x8070D2C8`) only resolves with the `+0x1000` rule used here. Reading at
`address - 0x80600000` yields plausible-looking garbage (`0x8070D2C0` decodes
as `lwz r0,20(r1)` instead of `beq`). All facts below were decoded with the
program-header rule.

**Paired-single opcode assignment (corrected).** The raw stream shows the
prologue save pair as `stfd f31,48(r1)` followed by a word `0xF3E10038`, which
is **opcode 60**, `D=31`, `A=1`, `DQ=0x38`, `W=0`, `I=0`. So in this build
`psq_l = 56`, `psq_lu = 57`, `psq_st = 60`, `psq_stu = 61`; opcodes 58/59 are
not used for `psq` here (59 is the single-precision float A-form). A decoder
that assumes `psq_st = 58` will silently drop every prologue spill.

**Result for these twelve:** scanning all 3 647 words of the twelve bodies for
any `psq*` / `mtspr GQRn`:

* every `psq_l`/`psq_st` in all twelve is a `W=0, I=0` (`qr0`) spill or reload
  of `f24..f31` at `r1`-relative offsets - **zero data-side paired-single
  operations**;
* **zero `mtspr GQRn`** - the only `mtspr` operands anywhere in the twelve are
  `CTR` and `LR`.

That is the mechanical proof that the wave-111/118/119/120/121/123/124/125
"`unaff_GQR0`" rejections were decompiler artefacts.

**Compiler save chains (verified from `work/dol/main.elf`).**
`0x802DCD40..0x802DCD84` is not a function; it is a ladder of

```
0x802DCD40  stw r14,-72(r11)   0x802DCD8C  lwz r14,-72(r11)
0x802DCD44  stw r15,-68(r11)   0x802DCD90  lwz r15,-68(r11)
...                            ...
0x802DCD84  stw r31, -4(r11)   0x802DCDD0  lwz r31, -4(r11)
0x802DCD88  blr                0x802DCDD4  blr
```

so `bl 0x802DCDxx` is a **save-only** chain that (a) never writes `r3..r13`,
(b) does not save `LR` (the caller does), and (c) returns with `r3` intact.
`r11` must point at the top of frame (`r1 + frame_size`). This is what makes
the "receiver-less" bodies reconstructible: after `bl 0x802DCDxx` the receiver
is still in entry `r3`, which every body below copies into a callee-saved
register on the next instruction.

Helper argument map used throughout:

| call | saves | matching restore |
|---|---|---|
| `0x802DCD40` | r14-r31 | `0x802DCD8C` |
| `0x802DCD48` | r16-r31 | `0x802DCD94` |
| `0x802DCD50` | r18-r31 | `0x802DCD9C` |
| `0x802DCD54` | r19-r31 | `0x802DCDA0` |
| `0x802DCD74` | r27-r31 | `0x802DCDC0` |

**Two `2^52` idioms.** The bodies use both `0x43300000_00000000` (2^52, when the
value word is used unsigned) and `0x43300000_80000000` (2^52+2^31, when the value
word was XOR-ed with `0x80000000` first). Every constant below is quoted with
the exact 8 bytes found in the image; do not assume 2^52.

**House style used by the drafts below.** `pub unsafe fn fn_XXXXXXXX(this: usize)`,
`platform::read_u32/read_i32/read_f32/read_u8/read_u16`, `platform::write_*`,
`abi_call1!..abi_call6!` (and `platform::call7`), addresses written as
`0x806F_3350`, field offsets as `usize` literals, and a `// unknown:` comment on
the first use of every address-only target. For calls with more than seven
arguments the file already has the `let function: extern "C" fn(..) -> u32 =
core::mem::transmute(addr);` precedent (`src/game/bases.rs:35720`).

---

## 1. `0x806F3350` - 2092 bytes - `work/ghidra/rel-d_basesNP/export/806f3350_FUN_806f3350.c`

**Body:** `0x806F3350..0x806F3B78` (523 words, ends `blr`). Ledger: absent.
`src/game/bases.rs`: absent.

```rust
pub unsafe fn fn_806f3350(this: usize)
```

**Frame.** `stwu r1,-160(r1)`; `mfspr r0,LR` / `stw r0,164(r1)`;
`addi r11,r1,160` / `bl 0x802DCD40` (saves r14-r31, r3 survives);
`or r16,r3,r3` at `0x806F3370` makes **r16 = this** for the whole body.
Epilogue `0x806F3B5C..0x806F3B78`: `bl 0x800690E0(this+0x138)`,
`bl 0x802DCD8C`, `lwz r0,164(r1)`, `mtspr LR,r0`, `addi r1,r1,160`, `blr`.

**Blocks.**

1. `0x806F3364-0x806F3388` - `lis r4,0x8037`; `lwz r5,-31816(r4)` (= `[0x80377C48]`);
   `li r4,-1`, `li r6,0`, `li r7,0x20`, `addi r3,r3,0x138`;
   `bl 0x800690C0(r3=this+0x138, r4=-1, r5=[0x80377C48], r6=0, r7=0x20)`.
2. `0x806F338C-0x806F33E8` - three-way selector on
   `r0 = *(*(0x8042A1B8) + 0x198)` (`lis r3,0x8043` / `lwz r3,-24136(r3)` /
   `lwz r0,408(r3)`; `cmpi cr0,0,r0,1`):
   * `r0 < 2`  -> `r4 = 0x807FF870`, `r0 = 0x21`
   * `r0 == 2` -> `r4 = 0x80800718`, `r0 = 9`
   * else      -> `r4 = 0x80800C28`, `r0 = 0x0D`

   each path does `addis r3,r16,1` then `stw r4,-16712(r3)` (`this+0xBEB8`) and
   `stw r0,-16708(r3)` (`this+0xBEBC`), then branches to `0x806F33EC`.
3. `0x806F33EC-0x806F3430` - loop preheader: `r22 = this+0x10000`,
   `r23 = this+0x1F4`, `r24 = this+0x154`, `r25 = this` (pane walker),
   `r26 = 0` (byte offset), `r27 = 0x807FF348`, `r30 = r31 = 0x807D0000`;
   `b 0x806F3B44` (bottom test first).
4. Outer loop, body `0x806F3434`, test `0x806F3B44-0x806F3B50`:
   `lwz r3,76(r1)` = `this+0x10000`; `lwz r0,-16708(r3)` = `this+0xBEBC`
   (the 0x21/9/0x0D count); `cmp cr0,0,r14,r0`; `blt 0x806F3434` while
   `r14 < count`.
   * `r15 = *(*(this+0xBEB8) + r26)` at `0x806F3434-0x806F343C`
     (`lwz r3,72(r1)` / `lwz r3,-16712(r3)` / `lwzx r15,r3,r26`);
     if zero, branch to `0x806F3B08` (skip the pane).
   * `bl 0x802DF098(r3=r15, r4=0x808011E0, r5=13)`; if `r3 == 0`:
     `bl 0x800DF180(r3=*(0x8042A038)+4, r4=0x808011E0, r5=0x808011F0)`;
     else compare against `0x80801200` (len 14) and fall back to
     `bl 0x800DF180(r3=*(0x8042A038)+4, r4=r15, r5=0x808011F0)`.
   * `stw r3,340(r25)` = `this+0x154` (pane object for this index).
   * `bl 0x80239E30(r3=this+0x154, r4=*(this+0xBEB8)+r26)` at `0x806F34DC`.
   * `bl 0x80169D90(r3=this+0x1F4, r4=&r1+0x28, r5=this+0x138, r6=0x736F,
     r7=1, r8=0)` at `0x806F3500`.
   * inner slot loop: counter `r17` (0..5), exit test `cmpi cr0,0,r17,6` at
     `0x806F3AAC`, back-edge `0x806F3ABC blt 0x806F351C`; the loop head at
     `0x806F351C` is a six-way `cmpi`/`bne` switch on `r17` selecting the
     `+0x04`, `+0x14`, `+0x24`, `+0x34`, `+0x44` (and else `+0x44`) name field.
     The slot walkers advance by `+0x2C` (`r18`, `r19`, `r20`), `+0x38`
     (`r21`) and `+0x2C` for the three `0x108`-strided containers
     (`r6`, `r2`, `r8` in the export).
   * per slot: the pane name table entry `*(this+0xBEB8) + iVar14` supplies
     `this+0xBC34` from `+0x04/+0x14/+0x24/+0x34/+0x44`; `this+0xBCD4` from
     `+0x08/...+0x48/+0x58`; `this+0xBE14` from `+0x10/...+0x50/+0x60`;
     `this+0xBD74` from `+0x0C/...+0x4C/+0x5C` (the export's nested
     `if (iVar7 == n)` chains).
5. Tail: `*(u8*)(this+0xBEB5) = 0`, `bl 0x800690E0(this+0x138)`, epilogue.

**Direct callees** (all addresses are `bl` targets; argument registers as
observed in the stream):

| site | callee | arguments |
|---|---|---|
| `0x806F3360` | `0x802DCD40` | save r14-r31; r11 = r1+160 |
| `0x806F3388` | `0x800690C0` | r3=this+0x138, r4=-1, r5=`[0x80377C48]`, r6=0, r7=0x20 |
| `0x806F3454` | `0x802DF098` | r3=pane name, r4=0x808011E0, r5=13 |
| `0x806F3474` | `0x800DF180` | r3=`[0x8042A038]`+4, r4=0x808011E0, r5=0x808011F0 |
| `0x806F3488` | `0x802DF098` | r3=pane name, r4=0x80801200, r5=14 |
| `0x806F34A8` | `0x800DF180` | r3=`[0x8042A038]`+4, r4=0x80801200, r5=0x808011F0 |
| `0x806F34C4` | `0x800DF180` | r3=`[0x8042A038]`+4, r4=pane name, r5=0x808011F0 |
| `0x806F34DC` | `0x80239E30` | r3=this+0x154, r4=`*(this+0xBEB8)+r26` |
| `0x806F3500` | `0x80169D90` | r3=this+0x1F4, r4=&r1+0x28, r5=this+0x138, r6=0x736F, r7=1, r8=0 |
| `0x806F35FC` | `0x8023A0B0` | r3=this+0x154 |
| `0x806F3620` | `0x801650D0` | r3=this+0x60, r4=&r1+0x24, r5=&r1+0x20, r6=this+0x138, r7=0 |
| `0x806F3640` | `0x801651F0` | r3=this+0x60, r4=this+0xBEB8, r5=&r1+0x38, r6=1 |
| `0x806F3650` | `0x80169F80` | r3=this+0xBEB8, r4=this+0x60, r5=&r1+0x38, r6=1 |
| `0x806F365C` | `0x801680E0` | r3=this+0x60, r4=this+0x60, r5=&r1+0x38, r6=1 |
| `0x806F3668` | `0x80168FE0` | r3=this+0x60, r4=this+0x60, r5=&r1+0x38, r6=1 |
| `0x806F374C` | `0x8023A200` | r3=this+0x154 |
| `0x806F3774` | `0x80166830` | r3=this+0x60, r4=&r1+0x1C, r5=&r1+0x18, r6=this+0x138, r7=0, r8=1 |
| `0x806F3798` | `0x80166BD0` | r3=this+0x60, r4=this+0xBEB8, r5=&r1+0x34, r6=0, r7=0, r8=1 |
| `0x806F37A8` | `0x80166F10` | r3=this+0x60, r4=0, r5=&r1+0x34, r6=0, r7=0, r8=1 |
| `0x806F37B8` | `0x80166EF0` | r3=this+0x60, r4=0, r5=&r1+0x34, r6=0, r7=0, r8=1 |
| `0x806F38B4` | `0x8023A270` | r3=this+0x154 |
| `0x806F38DC` | `0x80167420` | r3=this+0x60, r4=&r1+0x14, r5=&r1+0x10, r6=this+0x138, r7=0, r8=1 |
| `0x806F3900` | `0x80167800` | r3=this+0x60, r4=this+0xBEB8, r5=&r1+0x30, r6=0, r7=0, r8=1 |
| `0x806F3910` | `0x80167B40` | r3=this+0x60, r4=0, r5=&r1+0x30, r6=0, r7=0, r8=1 |
| `0x806F3920` | `0x80167B20` | r3=this+0x60, r4=0, r5=&r1+0x30, r6=0, r7=0, r8=1 |
| `0x806F3A1C` | `0x8023A190` | r3=this+0x154 |
| `0x806F3A44` | `0x80165C30` | r3=this+0x60, r4=&r1+0x0C, r5=&r1+0x08, r6=this+0x138, r7=0, r8=1 |
| `0x806F3A68` | `0x80165FD0` | r3=this+0x60, r4=this+0xBEB8, r5=&r1+0x2C, r6=0, r7=0, r8=1 |
| `0x806F3A78` | `0x80166310` | r3=this+0x60, r4=0, r5=&r1+0x2C, r6=0, r7=0, r8=1 |
| `0x806F3A88` | `0x801662F0` | r3=this+0x60, r4=0, r5=&r1+0x2C, r6=0, r7=0, r8=1 |
| `0x806F3B60` | `0x800690E0` | r3=this+0x138 |
| `0x806F3B68` | `0x802DCD8C` | restore r14-r31; r11 = r1+160 |

**Virtual calls** (3): `0x806F37CC`, `0x806F3934`, `0x806F3A9C` -
`lwz r12,0(r23)` / `lwz r12,0x18(r12)` / `mtspr CTR,r12` / `bcctrl 20,0,l`
with `r3 = r23 = this+0x1F4`, `r4 = this+0x60` (the allocated render object).

**Constants** (`image address` is where the value lives; the second address is
the instruction that materialises it):

| value | image address | materialised at |
|---|---|---|
| vtable/name pointer | `0x807FF870` | `addi r4,r27,1320` @`0x806F33A4` |
| vtable pointer | `0x80800718` | `addi r4,r27,5072` @`0x806F33C4` |
| vtable pointer | `0x80800C28` | `addi r4,r27,6368` @`0x806F33DC` |
| `"DEMO_field_op"` string | `0x808011E0` | `addi r4,r27,7832` @`0x806F344C` |
| `"g3d/model.brres"` string | `0x808011F0` | `addi r5,r27,7848` @`0x806F3464` |
| `"DEMO_field_end"` string | `0x80801200` | `addi r4,r27,7864` @`0x806F3480` |
| `f32 1.0` | `0x807CC110` | `lfs f1,-16112(r30)` @`0x806F3644` |
| `f32 0.0` | `0x807CC114` | `lfs f1,-16108(r31)` @`0x806F3660` |
| `u32` import cell | `0x80377C48` | `lwz r5,-31816(r4)` @`0x806F336C` |
| `u32` manager ptr | `0x8042A1B8` | `lwz r3,-24136(r3)` @`0x806F3390` |
| `u32` resource mgr | `0x8042A038` | `lwz r3,-24520(r3)` @`0x806F3468` |

**Confidence: high.** No ambiguity that changes behaviour: the tri-state
pointer/count at `+0xBEB8/+0xBEBC` is a data-side vtable pick, the six-slot
loops are fixed-bound, and the only external inputs are `[0x80377C48]` and
`[0x8042A038]`. All three pane pointers and all three name strings sit in this
REL's `.data` (`0x807D0C00..0x808228B8`) and are byte-verifiable in the image,
so they may be emitted as literals (or as `&`-references to `platform` reads).

---

## 2. `0x80686550` - 1884 bytes - `work/ghidra/rel-d_basesNP/export/80686550_FUN_80686550.c`

**Body:** `0x80686550..0x80686CA8` (471 words, ends `blr`). Ledger: absent.

```rust
pub unsafe fn fn_80686550(this: usize)
```

**Frame.** `stwu r1,-320(r1)`; LR at `324(r1)`; `addi r11,r1,192`;
`bl 0x802DCD54` (saves r19-r31); `stfd f31..f24` + `psq_st f31..f24,0,0`
(spills at `0xC8,0xD8,0xE8,0xF8,0x108,0x118,0x128,0x138`).
`r21` holds `this`, `r19` walks the 14-pointer array at `this+0x420`,
`r20` is its index, `r26 = 0x807C9890`, `r28 = 0x8042C1A0`.
Epilogue `0x80686C58..0x80686CA8`: `psq_l`/`lfd` of f24-f31,
`bl 0x802DCDA0`, `lwz r0,324(r1)`, `mtspr LR,r0`, `addi r1,r1,320`, `blr`.

**Shape.** A collision/height probe over fourteen array slots, not a
straight-line body:

1. `0x806865B4-0x806865E0`: `r19 = this+0x420`, `r20 = 0`;
   `do { if (*(u32*)r19 != 0) bl 0x80685140(*(u32*)r19); r20 += 1; r19 += 4; }
   while (r20 < 0x0E)` - exit test `cmpi cr0,0,r20,14` at `0x806865DC`,
   back-edge `0x806865E0 blt 0x806865C4`.
2. `0x806865F8-0x8068668C`: an index scan of the inline array object at
   `this+0x4D4`: `local_10c = this+0x4D4`; `for (i = 0; i < liVar->[1]; i++)`
   with `if (i < 0 || i >= liVar->[4]) vcall(&liVar[3], slot +0x18)`,
   breaking when `(*(u8*)(liVar->[5] + i) & 1) != 0`.
3. `0x80686690-0x80686B04`: the same scan against `*(this+0x4D8)` and a nested
   walk; this is the export's `if ((piVar11 != local_114) || (iVar16 != local_118))`
   branch pair. Per visited element at `piVar11->[2] + i*0x14`:
   * `elem[0]`/`elem[1]` (`f32`), `elem[2]` (`f32` used as a `u8`-table index,
     `0x8042C1A0`), and `*(u8*)(elem + 12)` are read;
   * `0x800E85A0(f32 delta, this+0x458)` accumulates a height;
   * if the accumulated height lands in `(48-28, 48+28)` (constants
     `0x807C989C`/`0x807C98A0`) then `*(u8*)(player+0x68E) = 1` and
     `*(u8*)(player+0x38D) = elem[3]`;
   * `bl 0x80686240(this, player, elem)`; if nonzero
     `bl 0x8006A4B0(&r1+0x100, player+0xAC)` then
     `bl 0x80198FC0([0x80429D60], 0x5A2, &r1+0x100, (s16)player+0x668, 0)`.
4. `0x80686B04-0x80686C50`: quantise the accumulated height through
   `0x807C98AC` (4.0) and `0x807C98E0` (255.0), build four bytes in
   `local_138/local_134`, call `bl 0x80685010(player, &local_138)`, then the
   14-slot sweep `bl 0x80685090(player)` / `bl 0x80685150(player)`, then
   `bl 0x80685260(player)` over another 14 slots.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x806865A0` | `0x802DCD54` | save r19-r31; r11 = r1+192 |
| `0x806865D0` | `0x80685140` | r3=`*(this+0x420+i*4)` (the `r19` walker) |
| `0x806869AC` | `0x800E85A0` | f1=delta, r3=this+0x458 (returns f32 in f1) |
| `0x806869E8` | `0x80686240` | r3=this, r4=player, r5=elem |
| `0x80686A08` | `0x8006A4B0` | r3=&r1+0x18, r4=player+0xAC |
| `0x80686A20` | `0x80198FC0` | r3=`[0x80429D60]`, r4=0x5A2, r5=&r1+0x18, r6=(s16)player+0x668, r7=0 |
| `0x80686B38` | `0x80684FE0` | r3=player=`*(this+0x420)`, f1=height |
| `0x80686BEC` | `0x80685010` | r3=player, r4=&local_138 |
| `0x80686C14` | `0x80685090` | r3=player |
| `0x80686C1C` | `0x80685150` | r3=player |
| `0x80686C40` | `0x80685260` | r3=player |
| `0x80686C98` | `0x802DCDA0` | restore r19-r31 |

**Virtual calls**: `+0x18` through `*(u32*)(this+0x4E0)` with receiver `this+0x4E0` (`=&liVar[3]`) at
`0x8068664C`, `0x806866C4`, `0x8068674C`, `0x80686790`, `0x806867F8`,
`0x806868E0`, `0x80686940`), `+0x18` on the `+0x4D4` object itself, and
`+0x18` on `*(u32*)(this+0x4D4)` with receiver `this+0x4D4` (the bounds/refresh
call), and `+0x24` on `*(u32*)(this+0x4D4)` with `r3 = this+0x4D4`, `r4 = elem`
(the "out of range" notify).

**Constants:**

| value | image address | materialised at |
|---|---|---|
| pool base (14-pointer block) | `0x807C9890` | `addi r26,r26,-26480` @`0x806865C0` |
| `f32 5.0` | `0x807C9894` | `lfs f26,4(r3)` @`0x80686688` |
| `f32 48.0` | `0x807C989C` | `lfs f28,12(r3)` @`0x80686848` |
| `f32 28.0` | `0x807C98A0` | `lfs f29,16(r3)` @`0x80686850` |
| `f32 4.0` | `0x807C98AC` | `lfs f30,28(r3)` @`0x80686868` |
| `f32 0.0` | `0x807C98C0` | `lfs f27,48(r26)` @`0x80686860` |
| `f32 255.0` | `0x807C98E0` | `lfs f31,80(r26)` @`0x80686870` |
| `f32 -4.0` | `0x807C993C` | `lfs f2,172(r3)` @`0x80686954` |
| `f64 2^52+2^31` | `0x807C98D0` | `lfd f27,64(r26)` @`0x8068667C` |
| signed `u8` table | `0x8042C1A0` | `addi r28,r28,-15968` @`0x80686678` |
| line-id singleton | `0x80429D60` | `lwz r20,-25248(r30)` @`0x806869FC` |

**Ambiguity (this is why the verdict is *conditional*).** Two items are not
decidable from this REL alone and would push the translation to `medium`:

1. the inline-array object at `this+0x4D4/+0x4D8` is only ever touched through
   five raw offsets (`[1]` count, `[2]` element base, `[3]`/`[0]` vtable,
   `[4]` bound, `[5]` flag bytes). The Rust can express it literally as
   `platform::read_u32(this + 0x4D4 + 4 * n)`, which is faithful, but any
   typed array class would be a guess;
2. the outer `do { ... } while (true)` shape (export line "`} while( true );`")
   is a loop whose exit is the `goto LAB_80686a84` reconvergence at
   `0x80686A84`; a naive `loop { }` is correct but must preserve the exact
   `break` on the flag byte.

Everything else (all callees, all constants, all stores) is explicit.
**Do not skip**; implement with `medium`.

---

## 3. `0x8069B750` - 1424 bytes - `work/ghidra/rel-d_basesNP/export/8069b750_FUN_8069b750.c`

**Body:** `0x8069B750..0x8069BCDC` (356 words, ends `blr`). Ledger: absent.

```rust
pub unsafe fn fn_8069b750(this: usize)
```

**Frame.** `stwu r1,-64(r1)`; LR at `68(r1)`; `stfd f31,48(r1)` +
`psq_st f31,56(r1),0,0`; saves r31, r30, r29; `or r31,r3,r3`.
Epilogue `0x8069BCC0..0x8069BCDC`: `psq_l f31,56(r1)` / `lfd f31,48(r1)`,
restore r29-r31, `mtspr LR,r0`, `addi r1,r1,64`, `blr`.

**Shape.** A two-state update for a "carried object" actor.

1. `0x8069B774-0x8069B784`: `lwz r12,1348(r3)` (`this+0x544`), `lwz r12,28(r12)`,
   `addi r3,r3,0x544`, `bcctr` -> **virtual slot `+0x1C` on `this+0x544`**.
2. `0x8069B788-0x8069B7B4`: `lwz r12,0x60(this)` slot `+0x2DC` -> `r7`;
   then `r8 = vcall(*(r7+0x5F8) + 0x0C)` and, if that is 0,
   `r7 = vcall(*(r7+0x5F8) + 0x10)`; `bVar5` = both nonzero.
3. if `bVar5`: slot `+0x380` on `*(this+0x60)`.
4. `if (*(s32*)(this+0x360) == 1)`:
   * `bl 0x80699A30(this)` -> `this+0xDAC`; if 0, slot `+0xD4(this, 0x8082B5AC)`
     and branch to `0x8069BB5C`; else `this+0x360 += 1`, then
     `pc = vcall(*(*(*(this+0xDAC)+0x738)+0x60) + 0x6C)()`, `c = *pc`,
     slot `+0x2DC(this)` -> `*(obj+0x7A4) = (s32)c`.
5. `else if (*(s32*)(this+0x360) == 2)`:
   * `if (*(s16*)(this+0x500) == 0)`: `bl 0x8069A290(this)` -> store u16,
     `bl 0x80699940(this, *(this+0xDAC))`, `bl 0x806A3D60(*(this+0xDAC))`,
     then two exits (slot `+0x2DC` -> `*(obj+0x5F8)` slot `+0x0C`):
     nonzero -> `bl 0x8069A370(this, 0x8082B6F0)` else slot `+0xD4(this, 0x8082B5AC)`;
     `b 0x8069BB5C`.
   * `f4 = *(this+0xDAC)+0xB0`, `f2 = *(this+0xB0)`, `f1 = *(this+0xDAC)+0xAC - *(this+0xAC)`;
     `f3 = slot +0x314(this)` (returns f32 in f1).
   * `bl 0x8069A8F0(this)`; if 0: compare `|f1|` with `[0x807CA08C]` (0.0),
     then `*(this+0xAC)` against `*(this+0xDA0)`/`*(this+0xD9C)`, and
     `|f1|` against `[0x807F1390]` (160.0): sets `r29 = 0`/`1` (see
     `0x8069BA00-0x8069BA20`); else compares `*(this+0xDC0)` with
     `[0x807CA08C]` for the same 0/1 result.
   * `r29 == 1` -> `*(u8*)(this+0xD31) = 1`; `r29 == 0` -> `*(u8*)(this+0xD32) = 1`.
   * `f = f4 - f2`; `>= 0.0` -> `iVar7 = 2` else `3`; `3` -> `*(u8*)(this+0xD34) = 1`
     else `*(u8*)(this+0xD33) = 1`.
   * `slot +0x3A0(this)` != 0 -> `*(this+0x360) = 1`.
6. `LAB_8069BB5C` (`0x8069BB5C`): `bl 0x800A4E90(this)`, slots `+0x33C`, `+0x340`,
   `bl 0x8006CD90(this)`, `bl 0x8006CDE0(this)`, `bl 0x800955F0(this)`,
   `slot +0x350(this)` -> f1 (saved to `f31`), `slot +0x34C(this)` -> f1,
   then `fmr f2,f31` / `addi r3,r31,0x114` / `bl 0x8015F470` - i.e.
   `0x8015F470(f1 = *(this+0x60)->+0x34C result, f2 = +0x350 result,
   r3 = this+0x114)`; the same pair for `+0x358`/`+0x354` -> `this+0x11C`;
   slot `+0x344`,
   `bl 0x80072BC0(this+0x1EC)`, clamp `*(this+0xEC)` to `[0x807CA08C]`,
   `bl 0x8006EF40(this+0x1EC,0)`, clamp again, `bl 0x8006FA40(this+0x1EC,0)`,
   `bl 0x80699E30(this)`, `s16 = slot +0x374(this)`, `bl 0x800A4480(this, s16)`,
   `bl 0x8069A230(this)`.

**Direct callees:** `0x80699A30(this)` @`0x8069B818`, `0x8069A290(this)`
@`0x8069B89C`, `0x80699940(this, *(this+0xDAC))` @`0x8069B8AC`,
`0x806A3D60(*(this+0xDAC))` @`0x8069B8BC`, `0x8069A370(this, 0x8082B6F0)`
@`0x8069B91C`, `0x806A3D60(*(this+0xDAC))` @`0x8069B938`,
`0x8069A8F0(this)` @`0x8069B9F4`, `0x800A4E90(this)` @`0x8069BB60`,
`0x8006CD90(this)` @`0x8069BB90`, `0x8006CDE0(this)` @`0x8069BB98`,
`0x800955F0(this)` @`0x8069BBA0`, `0x8015F470` @`0x8069BBD8` and `0x8069BC10`,
`0x80072BC0(this+0x1EC)` @`0x8069BC2C`, `0x8006EF40(this+0x1EC,0)`
@`0x8069BC58`, `0x8006FA40(this+0x1EC,0)` @`0x8069BC84`,
`0x80699E30(this)` @`0x8069BC8C`, `0x800A4480(this, s16)` @`0x8069BCB0`,
`0x8069A230(this)` @`0x8069BCB8`.

**Virtual slots used:** `this+0x544` -> `+0x1C`; `*(this+0x60)` -> `+0x2DC`,
`+0x380`, `+0xD4`, `+0x314`, `+0x3A0`, `+0x33C`, `+0x340`, `+0x350`, `+0x34C`,
`+0x358`, `+0x354`, `+0x344`, `+0x374`; `*(*(this+0x60)+0x2DC)->+0x60` ->
`+0xD4`; `*(*(this+0xDAC)+0x738)+0x60` -> `+0x6C`.

**Constants:** `f32 0.0` `0x807CA08C` (`lfs f0,-24436(r3)` @`0x8069BA08`);
`f32 160.0` `0x807F1390` (`lfs f0,80(r3)` after `lis r3,0x807F` @`0x8069BAA8`);
object floats `0x8082B658/0x8082B65C/0x8082B660`
(`lfs f1/f3/f1, 172/176/180(r4)` @`0x8069B9C0/0x8069B9B4/0x8069B9AC`);
message cells `0x8082B5AC` and `0x8082B6F0`.

> **Note on the two message cells.** `0x8082B5AC` and `0x8082B6F0` are inside
> this REL's `.bss` (`0x80822900..0x80834D84`), i.e. zero at load and filled by
> the message registrar at run time. They match Ghidra's `&DAT_8082b5ac` /
> `&DAT_8082b6f0` labels exactly, and the shipped `lis/addi` pairs at
> `0x8069B82C`+`0x8069B838` / `0x8069B904`+`0x8069B908` reproduce them, so the
> exports can be taken at face value here. Pass the literal; no control flow
> depends on their contents.

**Confidence: high.**

---

## 4. `0x80642540` - 1236 bytes - `work/ghidra/rel-d_basesNP/export/80642540_FUN_80642540.c`

**Body:** `0x80642540..0x80642A10` (309 words, ends `blr`). Ledger: absent.

```rust
pub unsafe fn fn_80642540(this: usize)
```

**Frame.** `stwu r1,-464(r1)`; LR at `468(r1)`; `addi r11,r1,432`;
`bl 0x802DCD48` (saves r16-r31); `stfd f31,448` + `psq_st f31,456`;
`stfd f30,432` + `psq_st f30,440`. Epilogue `0x806429EC..0x80642A10`
(`psq_l`/`lfd` of f30/f31, `bl 0x802DCD94`).

**Shape.** A model-pane matrix synchroniser; **no virtual calls at all**.

1. `0x80642564-0x806425C0`: `r4 = *(this+0x8B0)`; `r0 = r4-1`;
   `r31 = 0x807C8250` (`lis r31,0x807D` / `addi r31,r31,-32176`);
   `f31 = [0x807C834C]` (0.0), `f30 = [0x807C8350]` (26.0),
   `f0 = [0x807C8348]` (0.9), `f1 = [0x807C8300]` (1.0).
2. `0x806425C4-0x806426E8`: first loop over four players
   (`r7` 0..3, `piVar5 = 0x80354E60`):
   `if (*(u8*)(this + r7 + 0x8D8) != 0) { ... }`;
   `bl 0x80642510(this, r6)`; if the result != 0x18 then
   `r4 = *(this + result*4 + 0x284)`, copies 12 words from `r4+0x84..0xB0`
   into the 12-word stack block Ghidra names `local_14c..local_6c`, calls
   `bl 0x800B32E0(r3=&r1+0x54, r4=&r1+0x84)`, and stores the three results into
   `*(object+0xAC/0xB0/0xB4)` where `object = *(this + piVar5*4 + 0x2E4)`;
   then `*(object+0x220/0x224/0x228) = 0.9f` copies and
   `*(f32*)(object+0x25C) = f30`.
3. `0x806426E8-0x806427A0`: second loop, seven panes (`r6` advances `+4` per
   step from `this`, `r7 < 7`): copies `*(*(this+0x25C + step)+0x84..0xB0)`,
   calls `bl 0x800B32E0(r3=&r1+0x3C, r4=&r1+0x48)`, and writes
   `object+0xAC/0xB0/0xB4` plus `object+0x1F4/0x1F8/0x1FC` where
   `object = *(this + step*4 + 0x2F4)`.
4. `0x806427A8-0x806429C0`: final block - reads three source transform objects
   `*(this+0x278)`, `*(this+0x27C)`, `*(this+0x310)+0x208`, then for
   `n = 0..6` copies `*(this+0x2A0+4n)+0x44/0x48` into
   `*(*(this+0x310)+0x244+4n)+0x44/0x48`, and writes
   `*(*(this+0x310)+0x208)+0x2C/0x30/0x34/0x44/0x48`.
5. `0x806429EC` epilogue.

**Direct callees:** `0x802DCD48` @`0x80642560` (r11 = r1+432);
`0x80642510(this, r6)` @`0x806425F0` (**the only call whose result is
compared: `!= 0x18`**); `0x800B32E0(&r1+0x54, &r1+0x84)` @`0x80642684`;
`0x800B32E0(&r1+0x3C, &r1+0x48)` @`0x806427B8`; `0x802DCD94` @`0x80642A00`.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 0.0` | `0x807C834C` | `lfs f31,252(r31)` @`0x806425AC` |
| `f32 26.0` | `0x807C8350` | `lfs f30,256(r31)` @`0x806425C0` |
| `f32 0.9` | `0x807C8348` | `lfs f0,248(r31)` @`0x80642578` |
| `f32 1.0` | `0x807C8300` | `lfs f1,176(r31)` @`0x80642580` |
| player-flag byte table (`this+0x8D8`) | - | `*(u8*)(this + i + 0x8D8)` |
| player transform pointer table | `0x80354E60` | `addi r20,r20,20064` @`0x80642590` |

**Confidence: high.** Stores are all explicitly encoded (`0xAC/0xB0/0xB4`,
`0x1F4/0x1F8/0x1FC`, `0x220/0x224/0x228`, `0x25C`, `0x2C/0x30/0x34`,
`0x44/0x48`). The only stylistic risk is that Ghidra's `local_*` names for the
stack matrix blocks are reversed; the *register* facts above (`&r1+0x54`,
`&r1+0x84`, `&r1+0x3C`, `&r1+0x48`) are the ones to code against.

---

## 5. `0x8070D1F0` - 1200 bytes - `work/ghidra/rel-d_basesNP/export/8070d1f0_FUN_8070d1f0.c`

**Body:** `0x8070D1F0..0x8070D69C` (300 words, ends `blr`). Ledger: absent.

```rust
pub unsafe fn fn_8070d1f0(this: usize)
```

**Frame.** `stwu r1,-64(r1)`; LR at `68(r1)`; saves r31, r30, r29, r28;
`r31 = 0x807CC900`; `or r29,r3,r3`. Epilogue `0x8070D680..0x8070D69C`.

**Shape.** A six-entry animation/digit selector (the "cursor position" screen).

1. `0x8070D218-0x8070D228`: virtual `+0x1C` on `this+0x544` (`r3 = this+0x544`).
2. `0x8070D22C-0x8070D23C`: virtual `+0x14` on `this+0x5C0` (`r3 = this+0x5C0`).
3. `0x8070D244-0x8070D264`: `bl 0x8070C410(this)`, `bl 0x8070C4F0(this)`,
   `bl 0x8006CDE0(this)`, `bl 0x800955F0(this)`, `bl 0x8070C6D0(this)`.
4. `0x8070D268-0x8070D280`: `bl 0x8015F4F0(this+0x104,
   *(s16*)(0x807CC910 + *(this+0x628)*2), 0x200)`.
5. `0x8070D284-0x8070D2A4`: if `*(f32*)(this+0xEC) < [0x807CC900]` and
   `bl 0x8070C340(this) != 0`: `*(this+0x628) ^= 1`;
   `*(f32*)(this+0xEC) = [0x807CC9E4]` (1.75).
6. `0x8070D2A8-0x8070D348`: `r0 = 0x400` if `*(u8*)(this+0x6ED) == 0`, else the
   s16 angle is widened through the standard idiom `lis r0,0x4330`
   (@`0x8070D2C8`) + `xoris r3,r3,0x8000` (@`0x8070D2CC`) + `stw`/`lfd f2,8(r1)`/
   `lfd f1,0x130(r31)` (@`0x8070D2D8`; the `f64` at `0x807CCA30` is
   `0x4330000080000000` = 2^52+2^31, the signed-offset variant), i.e.
   `v = (f64)(0x4330000000000000 | (1024 ^ 0x80000000)) - (2^52+2^31) = 1024`, then
   `r0 = (s16)(f32)(angle * [0x807CC924])` (1.5); if
   `*(u8*)(this+0x791) != 0` the same with `[0x807CC97C]` (2.0).
7. `0x8070D34C-0x8070D5B8`: jump on `*(s32*)(this+0x6E0)` (0..5), each arm
   issuing one or two `bl 0x8015F4F0` / `bl 0x8015F560` calls with
   `r3 = this+0x100` or `this+0x102`, `r4 = *(s16*)(0x807CC908 + n*2)` or
   `*(s16*)(0x807CC90C + n*2)`, `r5 = sStack_1a` (or its negation), combining
   the two results with `and`. Exact `(table, index, sign)` per arm:

   | `this+0x6E0` | arm address | calls |
   |---:|---|---|
   | 0 | `0x8070D350` | `+0x100 <- 0`, `+0x102 <- tbl908[dir]`, both `+s` |
   | 1 | `0x8070D3C8` | result = 1 (no call) |
   | 2 | `0x8070D40C` | `dir==0`: `+0x100 <- tbl90E`, `+0x102 <- tbl908[0]`; else `+0x100 <- 0`, `+0x102 <- tbl908[dir]` with `-s` |
   | 3 | `0x8070D49C` | `dir==0`: `+0x100 <- 0`, `+0x102 <- tbl908[1]` with `-s`; else `+0x100 <- tbl90C[1]` with `-s`, `+0x102 <- tbl908[dir]` |
   | 4 | `0x8070D530` | result = 1 (no call) |
   | 5 | `0x8070D538` | `dir==0`: `+0x100 <- [0x807CC90C]`, `+0x102 <- tbl908[0]`; else `+0x100 <- 0`, `+0x102 <- tbl908[1-dir]` |

8. `0x8070D5BC-0x8070D680`: if `*(u8*)(this+0x6EC) == 0` and
   `*(s16*)(this+0x794) == 0` and the combined flag != 0:
   `*(this+0x6E0) = (dir==0 ? v+1 : v+5) % 6` (the `%6` is
   `lis r3,0x2AAB`/`addi r0,r3,-21845` + `mulhw` at `0x8070D610`,
   `0x8070D638-0x8070D648`), `*(s16*)(this+0x104) = 0x200`, virtual
   `+0xD4(this, 0x8082F630)`; else virtual `+0xD4(this, 0x8082F6B0)`.

**Constants:** `f32 0.0` `0x807CC900`; `f32 1.75` `0x807CC9E4`;
`f64 2^52+2^31` `0x807CCA30` (`0x4330000080000000`); `f32 1.5` `0x807CC924`;
`f32 2.0` `0x807CC97C`;
`s16` tables `0x807CC908`, `0x807CC90C`, `0x807CC910` and the single
`0x807CC90E`; message cells `0x8082F630`, `0x8082F6B0`;
`0x2AAAAAAB` magic @`0x8070D638` for the signed `%6`.

**Confidence: high.** The `0x4330` ladder is the compiler's ordinary integer ->
float conversion, and all six arms land on `0x8070D5BC`.

---

## 6. `0x806822B0` - 1108 bytes - `work/ghidra/rel-d_basesNP/export/806822b0_FUN_806822b0.c`

**Body:** `0x806822B0..0x80682700` (277 words, ends `blr`). Ledger: absent.

```rust
pub unsafe fn fn_806822b0(this: usize) -> u32   // returns 1 (see 0x806826E4)
```

**Frame.** `stwu r1,-272(r1)`; LR at `276(r1)`; `addi r11,r1,256`;
`bl 0x802DCD50` (saves r18-r31); `stfd f31,256` + `psq_st f31,264`.
`r18 = r3` after the helper. Epilogue `0x806826E0..0x80682700`
(`psq_l`/`lfd` f31, `bl 0x802DCD9C`, `li r3,1`).

**Shape.** A spawn/initialiser that unpacks one 32-bit parameter word
`uVar8 = *(u32*)(r18+4)` into actor fields:

| bits | destination |
|---|---|
| `bit0` | `*(u8*)(this+0x348)` |
| `bits 4-7` | `d = (f64)bits - 2^52`; `*(s16*)(this+0xAB2) = (s16)(0.4 * 32.0 * (1.0 + d))` |
| `bits 8-11` | `*(u8*)(this+0xAD4)` |
| `bits 12-15` | `sVar7 = -(u32)((uVar8>>12) & 0xF) << 12` -> `*(s16*)(this+0xAB0)` **and** `*(s16*)(this+0x104)` |
| `bits 16-19` | `*(f32*)(this+0xB4) -= [0x807C9744] * (float)(bits16_19 - 2^52)` |
| `bits 20-23` | `*(u32*)(this+0xAAC)` |

then:

* `*(f32*)(this+0xDC/0xE0/0xE4) = [0x807C973C]` (1.0) and
  `fVar2 = [0x807C9744] * ([0x807C973C] / [0x807C9740])` (10.0/25.0 = 0.4);
  `*(s16*)(this+0xAB2) = (s16)(fVar2 * [0x807C9748] * (1.0 + (bits_4_7 - 2^52)))`;
* `*(u32*)(this+0xB4) = [0x807C974C]` (-10.0), overridden to
  `[0x807C9750]` (-3490.0) when `*(u8*)(this+0x38F) == 1`;
* `bl 0x80682900(this)`, `bl 0x806829C0(this)`;
* `bl 0x800DB740(f1=[0x807C9754] (400.0), f2=[0x807C9758] (0.0), f3=-f1,
  f4=[0x807C9758], f5=-f1, r6=this+0x92C, r7=this,
  r8=(s16)(*(this+0xAB0)+0x3EE3), r9=[0x807C9734], r10=[0x807C9738],
  stack 1, stack 1)` - the **12-argument** constructor;
* `*(u8*)(this+0x976) = [0x80429D28]`, `*(u32*)(this+0x968) = 0x10000`,
  `if (*(s32*)(this+0xAAC) == 0) *(u8*)(this+0x973) = 0x0D`;
* `bl 0x800DAE80(this+0x92C)`;
* the 3x2 pane loop `0x806824D8-0x80682688` (`r9` 0..2): each iteration
  builds four stack vectors from the `0x807C9760..0x807C9784` pool, calls
  `bl 0x8007FAD0(r3=this+0x3D4+0xE4*n, r4=this, r5=&vecA, r6=&vecB,
  r7=[0x807ED134]+0, r8=+4, r9=+8, r10=1, stack=*(u8*)(this+0x38F),
  stack=&local_d0)` twice, then `bl 0x8007FD80`, sets
  `*(u16*)(this+0xAB4+2*n) = *(u16*)(this+0xAB0)`, `bl 0x8007FD80` again, sets
  `*(u32*)(this+0x4A0+0xE4*n) = 10` (n=0) or `|= 0x800000`,
  `|= 0x380000`, `bl 0x8007F900(...)`;
* `*(u8*)(this+0xAD5) = 0xFF`;
* `*(u32*)(this+0xAA4) = bl 0x80066890([0x80429D40], &r1+0x30)`;
* virtual `+0x18` on `this+0x394` with `r4 = 0x8082A7A8`.

**Constants:** `0x807C973C` 1.0; `0x807C9740` 25.0; `0x807C9744` 10.0;
`0x807C9748` 32.0; `0x807C974C` -10.0; `0x807C9750` -3490.0;
`0x807C9754` 400.0; `0x807C9758` 0.0; `0x807C975C` 48.0; the ten pane offsets
`0x807C9760..0x807C9784` = -80, -18, -48, 18, -30, -2, -16, 16, 30, -32;
`0x807C9788` `0x43300000` (2^52); `0x807C9734` 64976.0; `0x807C9738` 0.0;
`0x807ED134`/`0x807ED140` the pane-descriptor table entries (`0x806822A0`,
`0x80682260`); `0x80429D28`, `0x80429D40`.

**Calibration note:** the twelve-argument `0x800DB740` call has no `abi_callN!`
macro; use the file's `extern "C" fn(f64,f64,f64,f64,f64,u32,u32,u32,u32,u32,u32,u32,u32)`
transmute precedent (`src/game/bases.rs:35720`). All argument values above are
register-accurate: `f1..f5` are float registers even though Ghidra declares them
`double`, and the two trailing `1`s are the stack arguments at `8(r1)`/`12(r1)`.

**Confidence: medium** - solely because of (a) the >7-argument constructor and
(b) the `0x807C9758`-based `(double)` ladder, which requires the `2^52`
unsigned conversion to be written explicitly rather than as a cast. No unknown
control flow.

---

## 7. `0x806F7BB0` - 1004 bytes - `work/ghidra/rel-d_basesNP/export/806f7bb0_FUN_806f7bb0.c`

**Body:** `0x806F7BB0..0x806F7F98` (251 words, ends `blr`). Ledger: absent.

```rust
pub unsafe fn fn_806f7bb0(this: usize)
```

**Frame.** `stwu r1,-48(r1)`; LR at `52(r1)`; `stw r31,44(r1)`;
`r4 = 0x807CC290` (parameter block), `r31 = this`. Epilogue
`0x806F7F88..0x806F7F98`.

**Shape.** A three-mode oscillation driver on `*(s32*)(this+0x56C)`.

* `mode 1` (`0x806F7BE0-0x806F7CA4`), fields `+0x57C` (position), `+0x580`
  (velocity), `+0x579` (direction byte):
  `if (+0x579 == 0) { +0x580 = 0.04; +0x57C -= 0.04 }`
  `else { +0x580 += 0.008; +0x57C += +0x580 }`;
  clamp: `> 3.0` (and `+0x579 != 0`) -> `+0x57C = 3.0`, `+0x580 = 0.0`,
  `+0x579 = 0`; `<= 1.0` (and `+0x579 == 0`) -> `+0x57C = 1.0`,
  `+0x580 = 0.0`, `+0x579 = 1`.
* `mode 2` (`0x806F7CB0-0x806F7DCC`), fields `+0x104` (s16 angle), `+0x584`
  (step), `+0x578`:
  `if (+0x578 == 0) { f = +0x584 * (angle < 0 ? 0.94 : 1.07); +0x584 = f;
  +0x104 -= (s16)f }`
  `else { f = +0x584 * (angle < 1 ? 1.07 : 0.94); +0x584 = f;
  +0x104 += (s16)f }`;
  clamp: `+0x104 >= 0x4000` (and `+0x578 != 0`) -> `+0x104 = 0x4000`,
  `+0x584 = 25.6`, `+0x578 = 0`; `+0x104 < -0x3FFF` (and `+0x578 == 0`) ->
  `+0x104 = 0xC000`, `+0x584 = 25.6`, `+0x578 = 1`.
* `mode 3` (`0x806F7DD0-0x806F7F34`), fields `+0x588`, `+0x57A`:
  `if (+0x57A == 0) { f = (s16)(+0x588 * 0.95); +0x588 = f;
  +0x104 += (s16)f }`
  `else { f = +0x588 * 1.06; if (f <= 0.0) f = 0.0; +0x588 = f;
  +0x104 += (s16)f }`;
  then `((2^52+2^31) - ((u16)(+0x104) ^ 0x80000000)) * [0x8042DCD8]` compared with `0.0`, `180.0`
  and `-180.0`: `>= 180.0` (and `+0x57A != 0`) -> `+0x57A = 0`;
  `< 0.0` and `>= -180.0` and `+0x57A == 0` -> `+0x588 = 300.0`, `+0x57A = 1`.
* tail: virtual `+0x1C` on `this+0x5B0`; `bl 0x806F7AB0(this)`;
  `if ([0x8082ED60] != 0 && *(u8*)([0x8082ED60]+0xE0) == 0)`
  virtual `+0xD4(this, 0x8082EC70)`.

**Direct callee:** `0x806F7AB0(this)` @`0x806F7F4C`.
**Virtual:** `+0x1C` on `this+0x5B0` (`0x806F7F40`), `+0xD4` on `*(this+0x60)`
(`0x806F7F80`, `r4 = 0x8082EC70`).

**Constants** (base register `r4 = 0x807CC290`, so the image address is that
base plus the offset in the first column):

| offset | image address | value | materialised at |
|---|---|---|---|
| `+0x50` | `0x807CC2E0` | 1.0 | `lfs f1,80(r4)` @`0x806F7C74` |
| `+0x54` | `0x807CC2E4` | 0.0 | `lfs f0,84(r4)` @`0x806F7C58` |
| `+0x68` | `0x807CC2F8` | 2000.0 | `lfs f0,104(r4)` @`0x806F7DE0` |
| `+0x6C` | `0x807CC2FC` | 0.008 | `lfs f1,108(r4)` @`0x806F7C08` |
| `+0x70` | `0x807CC300` | 0.04 | `lfs f1,112(r4)` @`0x806F7C24` |
| `+0x74` | `0x807CC304` | 3.0 | `lfs f1,116(r4)` @`0x806F7C3C` |
| `+0x78` | `0x807CC308` | 1.07 | `lfs f0,120(r4)` @`0x806F7CC4` |
| `+0x7C` | `0x807CC30C` | 0.94 | `lfs f0,124(r4)` @`0x806F7CEC` |
| `+0x80` | `0x807CC310` | 25.6 | `lfs f0,128(r4)` @`0x806F7D80` |
| `+0x84` | `0x807CC314` | 1.06 | `lfs f1,132(r4)` @`0x806F7DDC` |
| `+0x88` | `0x807CC318` | 0.95 | `lfs f0,136(r4)` @`0x806F7E20` |
| `+0x8C` | `0x807CC31C` | 180.0 | `lfs f0,140(r4)` @`0x806F7EA0` |
| `+0x90` | `0x807CC320` | -180.0 | `lfs f0,144(r4)` @`0x806F7F00` |
| `+0x94` | `0x807CC324` | 300.0 | `lfs f0,148(r4)` @`0x806F7F28` |
| `+0x98` | `0x807CC328` | `0x4330000080000000` (2^52+2^31) | `lfd f1,152(r4)` @`0x806F7E24` |
| - | `0x8042DCD8` | `f32` 1/65536-style scale | `lfs f2,-9000(r5)` @`0x806F7E7C` |

**Confidence: high.** No indirect dispatch besides the two documented slots and
no paired-single data ops; the mode-3 `0x4330` ladder is the standard
signed-short-to-double conversion.

---

## 8. `0x8069FAB0` - 996 bytes - `work/ghidra/rel-d_basesNP/export/8069fab0_FUN_8069fab0.c`

**Body:** `0x8069FAB0..0x8069FE90` (249 words, ends `blr`). Ledger: absent.

```rust
pub unsafe fn fn_8069fab0(this: usize)
```

**Frame.** `stwu r1,-64(r1)`; LR at `68(r1)`; saves r31, r30, r29, r28;
`r30 = this`, `r31 = 0x807CA160`, `r28 = [0x80429D5C]`. Epilogue
`0x8069FE74..0x8069FE90`. No `psq` at all.

**Shape.** Four-state shot/launch actor (sibling of `0x8069F640`).

1. `0x8069FAD8-0x8069FAE8`: virtual `+0x1C` on `this+0x544`.
2. `0x8069FAF4`: `bl 0x80166280(this+0x5BC, 0)`.
3. `0x8069FAFC-0x8069FB68`: gate
   `*(f32*)(this+0xB0) < *(f32*)(this+0x76C)` &&
   `*(f32*)(this+0xEC) < [0x807CA194]` (0.1) && `*(s32*)(this+0x748) == 0`
   -> slots `+0x398`, `+0x2E0`; `*(s32*)(this+0x748) = (s16)slot(+0x39C)`.
4. `0x8069FB70-0x8069FB90`: `uVar6 = slot +0x350(this)`;
   `bl 0x8015F470(f1=[0x807CA1FC]=0.0, uVar6, this+0x114)` -> `uVar2`;
   `*(f32*)(this+0xEC) *= [0x807CA1CC]` (0.9); slot `+0x344`.
5. `0x8069FBC0-0x8069FBD8`: `bl 0x8006CD90(this)`, `bl 0x8006CDE0(this)`,
   `bl 0x800955F0(this)`, `bl 0x8069EAE0(this)`.
6. `0x8069FBE0-0x8069FC48`: `uVar3 = bl 0x8015F470(f1=*(this+0x754)+[0x807CA1C4](64.0),
   f2=[0x807CA1C8](0.5), this+0xB0)`;
   `bl 0x8015F470(f1=[0x807CA23C](30.0) * (float)((u8 table[*(u8*)(this+0x348)] ^ 0x80000000) - (2^52+2^31)),
   f2=[0x807CA240](5.0), this+0x740)`.
7. `0x8069FC54`: slot `+0x33C`.
8. `0x8069FC64-0x8069FCB0`: `bl 0x8006EF40(this+0x1EC,0)`;
   clamp `*(this+0xEC)` to `[0x807CA1FC]`; `bl 0x8006FA40(this+0x1EC,0)`;
   `s16 = slot +0x374(this)`; `bl 0x800A4480(this, s16)`.
9. state machine on `*(s32*)(this+0x360)`:
   * `1`: if `(uVar2 & 1 & uVar3) != 0` (i.e. `-(a&1&b) < 0` in the export):
     `bl 0x8023A0B0(this+0x540, "launch_big" = 0x807F1B28)`,
     `bl 0x801651F0(this+0x584, this+0x544, &local, 1)`,
     `bl 0x80169F80(f1=0.0, this+0x544, this+0x584)`,
     `bl 0x801680E0(f1=[0x807CA21C]=1.0, this+0x584)`,
     `bl 0x8006A4B0(&r1+0x20, this+0xAC)`,
     `bl 0x80197F00([0x80429D5C], 0x4F0, &r1+0x20, 0)`,
     `bl 0x8006A4B0(&r1+0x28, this+0xAC)`,
     `bl 0x80197F00([0x80429D5C], 0x5BB, &r1+0x28, 0)`,
     `*(this+0x360) += 1`.
   * `2`: `bl 0x801690E0(f1=[0x807CA244]=20.0, this+0x584)` != 0 ->
     `bl 0x80168FE0(f1=20.0, ...)`, `bl 0x801680E0(f1=0.0, ...)`,
     `*(s16*)(this+0x500) = 0x14`, `*(this+0x360) += 1`.
   * `3`: if `*(s16*)(this+0x500) == 0`:
     slot `+0xA4` on `this+0xD98` with `r4 = "Wm_jr_shot" (0x807F1B1C)`,
     `r5 = 0`, `r6 = this+0x7C`;
     `bl 0x8006A4B0(&r1+0x30, this+0xAC)`,
     `bl 0x80198C30([0x80429D60], 0x5BC, &r1+0x30, 0)`,
     `bl 0x801680E0(f1=[0x807CA21C], this+0x584)`, `*(this+0x360) += 1`.
   * `4`: `bl 0x8069E5F0(this)`; `bl 0x801690E0(f1=[0x807CA250]=24.0,
     this+0x584)` != 0 -> virtual `+0xD4(this, 0x8082B878)`.

**Constants:** `0x807CA194` 0.1; `0x807CA1C4` 64.0; `0x807CA1C8` 0.5;
`0x807CA1CC` 0.9; `0x807CA1FC` 0.0; `0x807CA1A8` 0.98 (used by the sibling);
`0x807CA208` 2^52+2^31; `0x807CA21C` 1.0; `0x807CA23C` 30.0; `0x807CA240` 5.0;
`0x807CA244` 20.0; `0x807CA250` 24.0; `0x8042C1A0` byte table;
`0x807F1B1C` `"Wm_jr_shot"`; `0x807F1B28` `"launch_big"`; `0x8082B878`
message cell; `0x80429D5C`/`0x80429D60` singletons.

**Confidence: high.** State transitions are all explicit stores of `+0x360`;
the only subtlety is that the `(uVar2 & 1 & uVar3) != 0` test appears in the
export as a negative-comparison idiom.

---

## 9. `0x806A0290` - 948 bytes - `work/ghidra/rel-d_basesNP/export/806a0290_FUN_806a0290.c`

**Body:** `0x806A0290..0x806A0640` (237 words, ends `blr`). Ledger: absent.

```rust
pub unsafe fn fn_806a0290(this: usize)
```

**Frame.** `stwu r1,-64(r1)`; LR at `68(r1)`; `addi r11,r1,64`;
`bl 0x802DCD74` (saves r27-r31); `or r31,r3,r3`. Epilogue
`0x806A062C..0x806A0640` (`bl 0x802DCDC0`).

**Shape.** The "launch charge" actor, four states.

1. `0x806A02B8-0x806A02C4`: virtual `+0x1C` on `this+0x544`; then
   `bl 0x80166280(this+0x5BC, 0)`.
2. `0x806A02D8-0x806A0314`: `bl 0x8015F470(f1 = [0x807CA23C](30.0) *
   (float)((u8table[*(u8*)(this+0x348)] ^ 0x80000000) - (2^52+2^31)), f2 = [0x807CA240](5.0),
   this+0x740)`; slot `+0x33C`.
3. state on `*(s32*)(this+0xD8)`:
   * `1`: `*(this+0x363) -= 1`; `slot +0x2DC(this)` -> `obj`;
     if `*(s32*)(*(obj+0x5F8)+4) < 1` and
     `bl 0x8015F4F0(this+0x102, 0, 0x1000)` and `*(this+0x363) == 0`:
     `obj = slot +0x2DC(this)`; virtual `+0xD4(obj, 0x80357AB4)`;
     virtual `+0xD4(this, 0x80357F9C)`;
     else if `*(this+0x363) == 0`: `*(this+0xD8) += 1`.
   * `2`: `bl 0x800955F0(this)`; `obj = *(this)`; `0x8006A4B0(&r1+0x28,
     this+0xAC)`; `bl 0x80198660([0x80429D5C], 0x5AC, obj, &r1+0x28, 0)`;
     if `*(this+0x362) == 0x0F`: `obj = slot +0x2DC(this)`;
     virtual `+0xD4(obj, 0x803579B4)`; `*(this+0xD8) += 1`;
     else `*(this+0x362) -= 1`.
   * `3`: `bl 0x800955F0(this)`; `obj = *(this)`;
     `bl 0x80198660([0x80429D5C], 0x5AC, obj, &r1+0x30, 0)`;
     `*(f32*)(this+0xE8) *= [0x807CA1A8]` (0.98);
     `s16 = slot +0x370(this)`; virtual `+0x374(f1 = (float)((s16)(this+0x102) - s16),
     this)`; `bl 0x80169020(this+0x584)` != 0 ->
     `*(f32*)(this+0xE8) = [0x807CA1FC]`(0.0), `*(s16*)(this+0xD86) = 0`,
     `*(s16*)(this+0x102) = s16`, slot `+0x394`, `*(this+0xD8) += 1`.
   * `4`: `obj = slot +0x2DC(this)`; `p = slot +0x28 on *(obj+0x394)`;
     `slot +0x14(p, 0x803579F4)` != 0 -> `obj2 = slot +0x2DC(this)`;
     `bl 0x80169020(obj2+0x644)` != 0 ->
     virtual `+0xD4(this, 0x8082B734)`;
     `obj3 = slot +0x2DC(this)`; virtual `+0xD4(obj3, 0x803578F4)`.

**Constants:** `0x807CA208` 2^52+2^31; `0x807CA23C` 30.0; `0x807CA240` 5.0;
`0x807CA1A8` 0.98; `0x807CA1FC` 0.0; `0x8042C1A0` byte table;
message cells `0x80357AB4`, `0x803579B4`, `0x803578F4`, `0x8082B734`;
ids `0x5AC` (1452) and `0x4F0` (1264) on the sound system; singletons
`0x80429D5C`.

**Confidence: high.** The `_savegpr` "return value" (`puVar3 = func()` in the
export) is simply entry `r3`; use `r31`/`this` directly.

---

## 10. `0x806FD820` - 912 bytes - `work/ghidra/rel-d_basesNP/export/806fd820_FUN_806fd820.c`

**Body:** `0x806FD820..0x806FDBAC` (228 words, ends `blr`). Ledger: absent.

```rust
pub unsafe fn fn_806fd820(this: usize)
```

**Frame.** `stwu r1,-160(r1)`; LR at `164(r1)`; `stfd f31/f30` +
`psq_st f31,152` / `f30,136`; saves r31, r30, r29; `r29 = this`,
`r31 = 0x807CC408`. Epilogue `0x806FDB88..0x806FDBAC`.

**Shape.** Draws the "player cursor" quad colourised by the current player.

1. Gate (all must hold, `0x806FD85C-0x806FD894`):
   `*(s32*)(this+0x210) > 1` && `*(u8*)(this+0x220) != 0` &&
   `*(u8*)(this+0x1D5) == 0` && `*(u8*)(this+0x1D6) == 0` &&
   `bl 0x8005F900(*(this+0x208)) != 0`.
2. `r30 = the player`; jump on `*(s32*)(r30+0x14D4)` (0..3) to pick two
   RGB words into `local_94` / `local_98`
   (`0xFFAB00FF/0xFF1B00FF`, `0x66FF00FF/0x00FF15FF`, `0x00FFFFFF/0x006CFFFF`,
   `0xFFFF44FF/0xFFC000FF`).
3. `p = bl 0x80056E30(r30)`; `slot +0x50(p, &r1+0x38, 1)`;
   `bl 0x8016F130(&r1+0x38, &r1+0x10)`; `local_84 = r1+0x10`,
   `local_7C = [0x807CC40C]` (5500.0).
4. `i = *(r30+0x14D4)`; `0..1` -> `f31 = [0x807CC410]` (6.5),
   `f30 = [0x807CC414]` (7.0); `2..3` -> both `[0x807CC418]` (8.0);
   `*(r30+0x1090) == 3` -> `f31 -= [0x807CC41C]` (2.0);
   `state 0 or 3` -> `f31 += local_8C` and colour `[0x807CC420]` (0.6),
   else `f31 += f30` and colour `[0x807CC424]` (1.0).
5. `slot +0xA0` on `this+0x224` with `r4 = "Wm_mg_cursor" (0x8080330C)`,
   `r5 = 0`, `r6 = &local_84`, `r7 = 0`, `r8 = &local_78`;
   `slot +0x5C(..., &local_94, &local_98, 0, 3)`;
   `slot +0x60(..., local_94 & 0xFF, local_98 & 0xFF, 0, 3)`.
6. if `*(u8*)(this+0x460) == 0`: the same three calls on `this+0x34C` with
   `"Wm_mg_cursor_st" (0x80803320)`, then slot `+0x94`, and
   `*(u8*)(this+0x460) = 1`.

**Constants:** `0x807CC40C` 5500.0; `0x807CC410` 6.5; `0x807CC414` 7.0;
`0x807CC418` 8.0; `0x807CC41C` 2.0; `0x807CC420` 0.6; `0x807CC424` 1.0;
strings `0x8080330C` `"Wm_mg_cursor"`, `0x80803320` `"Wm_mg_cursor_st"`;
colour literals in the switch.

**Ambiguity -> conditional/medium:** the eight immediate colour words are not
in `.data` (they are `li`/`oris` immediates), which is fine, but `local_94`
and `local_98` are packed `RGBA`-style words whose channel order is only
implied by the `& 0xFF` arguments at `0x806FDB18`/`0x806FDB5C`. The Rust can
reproduce them byte-for-byte; what cannot be asserted is which channel is
which. Also `local_8C`/`local_90` are outputs of `0x8016F130` whose struct
layout is not visible in this REL (`&r1+0x38` in, `&r1+0x10` out). Implement
with the literal words and a `// unknown:` note on `0x8016F130`; verdict
`medium`, not a skip.

---

## 11. `0x806B5E00` - 892 bytes - `work/ghidra/rel-d_basesNP/export/806b5e00_FUN_806b5e00.c`

**Body:** `0x806B5E00..0x806B6178` (223 words, ends `blr`). Ledger: absent.

```rust
pub unsafe fn fn_806b5e00(this: usize)
```

**Frame.** `stwu r1,-80(r1)`; LR at `84(r1)`; `stfd f31,64` + `psq_st f31,72`;
saves r31, r30; `r31 = this`, `r30 = 0x807CAA60`. Epilogue
`0x806B615C..0x806B6178`.

**Shape.** A stomp/landing actor, two states on `*(s32*)(this+0x360)`.

1. `0x806B5E34`: `bl 0x8015F470(f1=[0x807CAA68]=1.0, f2=[0x807CAB0C]=0.1,
   this+0x6C8)`.
2. `0x806B5E40`: `r3 = bl 0x8006FA40(this+0x1EC, 0)`; if `r3 == 0`:
   * `state 1`: `bl 0x8006CDE0(this)`, `bl 0x800955F0(this)`;
     `s16 = slot +0x2A4(this)`; `d = (s16)(s16 - *(s16*)(this+0x6D4))`;
     `f = ([0x807CAA64] < (f32)d) ? [0x807CAA68] : [0x807CAB10]`;
     `bl 0x8015F340(this+0x6D4, r4 = slot +0x2A4(this),
     r5 = (s16)([0x807CAB14](256.0) * f))`;
     `*(s16*)(this+0x104) += *(s16*)(this+0x6D4)`;
     `if (bl 0x80072BC0(this+0x1EC) != 0 && *(f32*)(this+0xEC) <= [0x807CAA64])`:
     build `&r1+0x18` from `this+0xAC/0xB0/0xB4` minus
     `f1 = slot +0x288(this)`, `bl 0x800757B0(&r1+0x18, this+0x6C4,
     *(u8*)(this+0x38F), 1, -1)`, `bl 0x806B4ED0(this, 1)`,
     `*(this+0x360) = 2`, `bl 0x8006A4B0(&r1+0x30, this+0xAC)`,
     `bl 0x80198C30([0x80429D60], 0x537, &r1+0x30, 0)`,
     `bl 0x806B4F90(this)`, and if `*(f32*)(this+0x6D8) < f1(slot +0x2A8)`
     -> virtual `+0xD4(this, 0x8082C4FC)`.
   * `state 2`: `bl 0x800955F0(this)`; `d = (f32)((s16)(this+0x6D4) << 0)`;
     `f = ([0x807CAA64] < d) ? [0x807CAA68] : [0x807CAB10]`;
     `bl 0x8015F340(this+0x6D4, 0, (s16)([0x807CAB14] * -f))`;
     `*(s16*)(this+0x104) += *(s16*)(this+0x6D4)`;
     `f1 = slot +0x284(this)`, `f2 = slot +0x288(this)`;
     `*(f32*)(this+0xB0) = *(f32*)(this+0x6C4) + *(f32*)(this+0xE0) * (f32)(f2/f1)`;
     `if ([0x8082C38C] <= *(f32*)(this+0x6DC))`: `*(f32*)(this+0xEC) =
     *(f32*)(this+0x6D8)`, `*(this+0x360) = 1`,
     `*(f32*)(this+0x6D8) *= f1(slot +0x290)`.
3. else (the `0x8006FA40` branch, `0x806B6078`): `bl 0x8006A4B0(&r1+0x28,
   this+0xAC)`, `bl 0x80198C30([0x80429D5C], 0x538, &r1+0x28, 0)`,
   virtual `+0xD4(this, 0x8082C4FC)`.

**Constants:** `0x807CAA64` 0.0; `0x807CAA68` 1.0; `0x807CAB00` 2^52+2^31;
`0x807CAB0C` 0.1; `0x807CAB10` -1.0; `0x807CAB14` 256.0;
`0x8082C38C` (a float cell, external); message cells `0x8082C4FC`;
id `0x537`/`0x538`.

**Confidence: high.** All five virtual slots (`+0x2A4`, `+0x288`, `+0x284`,
`+0x290`, `+0xD4`) are visible as `lwz r12, off(r12)` with `r3 = this`.

---

## 12. `0x8069F640` - 892 bytes - `work/ghidra/rel-d_basesNP/export/8069f640_FUN_8069f640.c`

**Body:** `0x8069F640..0x8069F9B8` (223 words, ends `blr`). Ledger: absent.

```rust
pub unsafe fn fn_8069f640(this: usize)
```

**Frame.** `stwu r1,-64(r1)`; LR at `68(r1)`; saves r31, r30, r29, r28;
`r31 = this`, `r28 = 0x807CA160`, `r29 = [0x80429D60]`. Epilogue
`0x8069F99C..0x8069F9B8`. No `psq`.

**Shape.** The "jr shot" sibling of §8, four states, same constant pool.

1. `0x8069F668-0x8069F678`: virtual `+0x1C` on `this+0x544`.
2. `0x8069F67C`: virtual `+0x14` on `this+0x5EC`.
3. `0x8069F690-0x8069F6B0`: `uVar6 = slot +0x350(this)`;
   `bl 0x8015F470(f1=[0x807CA1FC]=0.0, uVar6, this+0x114)`;
   `*(f32*)(this+0xEC) *= [0x807CA1CC]` (0.9); slot `+0x344`.
4. `0x8069F6E4-0x8069F6FC`: `bl 0x8006CD90`, `bl 0x8006CDE0`,
   `bl 0x800955F0`, `bl 0x8069EAE0` (all `this`).
5. `0x8069F704-0x8069F76C`: `bl 0x8015F470(f1=*(this+0x754)+[0x807CA1C4](64.0),
   f2=[0x807CA1C8](0.5), this+0xB0)`;
   `bl 0x8015F470(f1=[0x807CA23C](30.0) * (float)((u8table[*(u8*)(this+0x348)] ^ 0x80000000) - (2^52+2^31)),
   f2=[0x807CA240](5.0), this+0x740)`; slot `+0x33C`.
6. `0x8069F788-0x8069F7B0`: `bl 0x8006EF40(this+0x1EC,0)`, clamp
   `*(this+0xEC)` at `[0x807CA1FC]`, `bl 0x8006FA40(this+0x1EC,0)`.
7. `0x8069F7B4-0x8069F7D8`: `s16 = slot +0x374(this)`;
   `bl 0x8015F4F0(this+0x102, 0, s16)`.
8. states on `*(s32*)(this+0x360)`:
   * `1`: same `(a&1&b)` test -> `bl 0x8023A0B0(this+0x540, "launch" 0x807F1AEC)`,
     `bl 0x801651F0(this+0x584, this+0x544, &local, 1)`,
     `bl 0x80169F80(f1=0.0, ...)`, `bl 0x801680E0(f1=[0x807CA21C]=1.0, ...)`,
     `bl 0x8006A4B0(&r1+0x20, this+0xAC)`,
     `bl 0x80197F00([0x80429D5C], 0x4F0, &r1+0x20, 0)`,
     `bl 0x8006A4B0(&r1+0x28, this+0xAC)`,
     `bl 0x80197F00([0x80429D5C], 0x5BA, &r1+0x28, 0)`, `*(this+0x360) += 1`.
   * `2`: `bl 0x801690E0(f1=[0x807CA244]=20.0, this+0x584)` != 0 ->
     `bl 0x80168FE0(20.0)`, `bl 0x801680E0(0.0)`,
     `*(s16*)(this+0x500) = 0x14`, `*(this+0x360) += 1`.
   * `3`: if `*(s16*)(this+0x500) == 0`: slot `+0xA4` on `this+0xD98` with
     `r4 = "Wm_jr_shot" (0x807F1B1C)`, `r5 = 0`, `r6 = this+0x7C`;
     `bl 0x8006A4B0(&r1+0x30, this+0xAC)`,
     `bl 0x80198C30([0x80429D60], 0x5BC, &r1+0x30, 0)`,
     `bl 0x801680E0(f1=[0x807CA21C], this+0x584)`, `*(this+0x360) += 1`.
   * `4`: `bl 0x8069E5F0(this)`; `bl 0x801690E0(f1=[0x807CA248]=24.0,
     this+0x584)` != 0 -> virtual `+0xD4(this, 0x8082B878)`.

**Constants:** identical pool to §8 (`0x807CA1C4/C8/CC/FC/208/21C/23C/240`,
`0x807CA244` 20.0, `0x807CA248` 24.0), plus `0x807F1AEC` `"launch"`,
`0x807F1B1C` `"Wm_jr_shot"`, `0x8082B878`, `0x80429D5C/0x80429D60`.

**Confidence: high.** Same shape as §8/§9; the state-1/2/3/4 boundaries are
`0x8069F7E4`, `0x8069F7EC`, `0x8069F7F4`, `0x8069F7FC`.

---

## Ready vs conditional

**Ready - implement as specified, confidence `high` (9):**
`0x806F3350`, `0x8069B750`, `0x80642540`, `0x8070D1F0`, `0x806F7BB0`,
`0x8069FAB0`, `0x806A0290`, `0x806B5E00`, `0x8069F640`.

**Conditional - implement as specified, confidence `medium` (3):**

| address | condition |
|---|---|
| `0x80686550` | The inline-array object at `this+0x4D4` is only reachable through five raw offsets; a literal `read_u32` translation is correct, any typed class is a guess. The `do{}while(true)` reconvergence at `0x80686A84` must keep the flag-byte `break`. |
| `0x806822B0` | Twelve-argument constructor `0x800DB740` (needs the file's 12-argument `transmute` precedent - no macro covers it) and the `0x4330` integer-to-float ladder for the packed spawn word. |
| `0x806FD820` | Colour channel order of the eight packed RGBA immediates and the `0x8016F130` output layout (`&r1+0x10`) are not in this REL; pass the literal words with `// unknown:` notes. |

**No skips.** None of the twelve needs a GQR constant, a jump-table generator
or a fragment walk: the bodies are `blr`-terminated and self-contained, and
every constant they read is at a fixed image address.

## Verification performed

* `ledger/functions.csv` - none of the twelve addresses occurs as a row
  (3 230 `src/game/bases.rs` rows are present; all twelve are new).
* `src/game/bases.rs` - no `fn_806f3350`, `fn_8069b750`, ... identifier exists.
* `work/ghidra/rel-d_basesNP/export/functions.tsv` - all twelve rows exist with
  the byte counts quoted above; each body was decoded from the entry to a
  terminating `blr`, so the extents' padding (0 to 2 340 trailing bytes before
  the next symbol) is confirmed as data, not code.
* Program headers of `work/rels/elf/d_basesNP.elf`:
  `p_offset 0x10F0`, `p_vaddr 0x806000F0`, `filesz 0x1C6004`; `.data`
  `0x807D0C00..0x808228B8`, `.bss` `0x80822900..0x80834D84` - used to classify
  every constant address quoted above as "in image", "BSS" or "imported".
* `work/dol/main.elf` - the `0x802DCD40..0x802DCD84` save ladder and
  `0x802DCD8C..0x802DCDD0` restore ladder were decoded instruction by
  instruction to establish that `r3..r13` survive and that `LR` is not saved
  by them.
* Paired-single audit: 3 647 words scanned; all `psq*` are `W=0, I=0` saves of
  `f24..f31` at `r1`; no `mtspr GQRn`; no `psq_stu`, `psq_lx`, `psq_stx`.

## Deliberately not claimed here (class C, for a later wave)

The 24 `qr3` bodies (`0x806B7CD0`, `0x806DD730`, `0x8072E520`, `0x8072BC40`,
`0x806B9A30`, `0x806B3770`, ...) still need the GQR3 type word from `main.dol`'s
`OSInitFastCast`; `0x807BD110` still needs the `dFileSelect_c` 44-entry state
table generator. Neither is a control-flow problem, which is why they are not
in this wave.
