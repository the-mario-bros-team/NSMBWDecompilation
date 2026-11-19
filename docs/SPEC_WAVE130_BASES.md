# Wave 130 - `game::bases` - implementation spec for the next 12 class-B addresses

Read-only lane. The only file written is this one; the throwaway decoder lives
in `%TEMP%\opencode` (`b130_ppc.js`, `b130_dump.js`, `b130_const.js`,
`b130_rank2.js`, `b130_sha.js`). No Rust file, ledger row or existing document
was modified.

## Why these twelve

`docs/ANALYSIS_WAVE127_BASES.md` lists 121 class-B addresses (51 060 bytes).
Waves 128 and 129 specified the first 24. This wave takes **the next twelve by
TSV size** after applying the same selection rule wave 129 documented:

* **ledger** - an address is claimed only if it is a row (exact match in the
  `address` column of `ledger/functions.csv`); a mention inside another row's
  evidence text does **not** claim it;
* **specs** - an address is claimed only if it is a section heading in
  `docs/SPEC_WAVE128_BASES.md` / `docs/SPEC_WAVE129_BASES.md`; a callee mention
  does not claim it;
* **Rust** - no `pub unsafe fn fn_<address>` already exists in
  `src/game/bases.rs`.

| # | address | TSV bytes | words | export file | verdict |
|--:|---:|---:|---:|---|---|
| 1 | `0x8072FE10` | 684 | 171 | `8072fe10_FUN_8072fe10.c` | ready (high) |
| 2 | `0x80642A20` | 660 | 165 | `80642a20_FUN_80642a20.c` | ready (medium) |
| 3 | `0x80673F50` | 656 | 164 | `80673f50_FUN_80673f50.c` | ready (high) |
| 4 | `0x806B5180` | 620 | 155 | `806b5180_FUN_806b5180.c` | ready (high) |
| 5 | `0x8070D790` | 584 | 146 | `8070d790_FUN_8070d790.c` | ready (high) |
| 6 | `0x807247D0` | 584 | 146 | `807247d0_FUN_807247d0.c` | conditional (medium) |
| 7 | `0x80781DB0` | 564 | 141 | `80781db0_FUN_80781db0.c` | ready (high) |
| 8 | `0x80740370` | 552 | 138 | `80740370_FUN_80740370.c` | conditional (medium) |
| 9 | `0x806FDD50` | 536 | 134 | `806fdd50_FUN_806fdd50.c` | ready (high) |
| 10 | `0x80748B20` | 520 | 130 | `80748b20_FUN_80748b20.c` | conditional (medium) |
| 11 | `0x80780FA0` | 520 | 130 | `80780fa0_FUN_80780fa0.c` | ready (high) |
| 12 | `0x80647E60` | 512 | 128 | `80647e60_FUN_80647e60.c` | ready (high) |

Together: **6 992 bytes**, 13.7 % of the class-B total. Every export path below
is exactly `work/ghidra/rel-d_basesNP/export/<file>` relative to the repo root.

Selection notes:

* the class-B table was re-ranked by `bytes`; after subtracting the 24 wave
  128/129 headings, the exact ledger rows and existing Rust definitions, the
  head of the remaining list is the twelve above;
* the next three behind #12 are `0x8066FA10` (508), `0x806A4CA0` (504) and
  `0x807816B0` (460); they are outside this batch;
* `0x80642A20` and `0x807816B0` are only *mentioned* in the wave-129 text
  (the former in the meta-paragraph about ledger matching on
  `docs/SPEC_WAVE129_BASES.md:1468`), so under the heading-only rule they remain
  candidates - the same call wave 129 made for `0x8069E5F0`/`0x806B4F90`;
* none of the twelve is a ledger row, and none has a Rust definition (the two
  comment-only cross-references `fn_806a4ca0`/`fn_807616e0` in `bases.rs`
  belong to addresses outside this batch);
* no two bodies in the batch are byte-identical (SHA-1 of the twelve extents
  differs pairwise).

## Method - facts re-verified from the shipped bytes

**Address mapping.** `d_basesNP.elf` has six `PT_LOAD` headers; the first is
`p_offset = 0x10F0`, `p_vaddr = 0x806000F0`, so

```
file_offset = 0x10F0 + (address - 0x806000F0) = (address - 0x80600000) + 0x1000
```

as recorded in `docs/REL_ELF_MAPPING.md`. Every byte below was read with the
program-header rule, never with `address - base`.

**Sizes are exact bodies.** For all twelve the TSV size equals the reachable
span: the last word of the extent is a terminating `blr` and every internal
`b`/`bc` target is inside the extent. All twelve end `blr`; the only
out-of-extent branches are the direct `bl` calls listed per function.

**Paired-single audit** (word-by-word scan of all 1 748 words):

| function | `psq*` / `ps_*` ops | indices | kind |
|---|---:|---|---|
| `0x8072FE10` | 0 | - | - |
| `0x80642A20` | 0 | - | - |
| `0x80673F50` | 6 | `qr0` only | `f29`/`f30`/`f31` save + restore |
| `0x806B5180` | 2 | `qr0` only | `f31` save + restore |
| `0x8070D790` | 0 | - | - |
| `0x807247D0` | 0 | - | - |
| `0x80781DB0` | 20 | `qr0` only | **real data-side pairs**: `f31`/`f30` saves plus 3-float vector subtraction |
| `0x80740370` | 6 | `qr0` only | `f29`/`f30`/`f31` save + restore |
| `0x806FDD50` | 0 | - | - |
| `0x80748B20` | 2 | `qr0` only | `f31` save + restore |
| `0x80780FA0` | 4 | `qr0` only | `f31`/`f30` save + restore |
| `0x80647E60` | 0 | - | - |

There is **no `qr3` access and no `mtspr` other than `LR`/`CTR` anywhere in the
twelve**. `0x80781DB0` is the only body with genuine data-side paired-single
arithmetic; its `psq_l`/`psq_st` use `W=0` for the `(x,y)` pair and `W=1` for
the single `z`, base `r1`, and `ps_sub` assembles the three-float result. Type 0
(32-bit float) is the only assignment that makes the following `lfs` of the
stored lanes correct, so that body is fully reconstructible.

The `unaff_GQR0` text in the Ghidra exports of `0x80673F50`, `0x806B5180`,
`0x80740370`, `0x80748B20`, `0x80780FA0` and `0x80781DB0` is the same
decompiler artefact waves 127-129 documented: in five of the six it is produced
by the `stfd`/`psq_st` spill pair, and in `0x80781DB0` the `ps_*` instructions
simply were not lifted.

**Compiler save chains** (verified in `work/dol/main.elf`):

```
0x802DCD40  stw r14,-72(r11)  ...  0x802DCD84  stw r31,-4(r11)
0x802DCD88  blr                    0x802DCD8C  lwz r14,-72(r11) ...
```

so `bl 0x802DCDxx` is a save-only ladder that (a) never writes `r3..r13`,
(b) does not touch `LR`, and (c) returns with `r3` intact. Chains used here
(`0x802DCD40 + 4k` saves `r(14+k)..r31`; restore = entry + `0x4C`):

| call | saves | restore | `r11` used | slot range |
|---|---|---|---|---|
| `0x802DCD4C` | `r17..r31` | `0x802DCD98` | `r1+128` | `r1+68..r1+124` |
| `0x802DCD5C` | `r21..r31` | `0x802DCDA8` | `r1+112` | `r1+64..r1+108` |
| `0x802DCD68` | `r24..r31` | `0x802DCDB4` | `r1+64` | `r1+40..r1+60` |
| `0x802DCD6C` | `r25..r31` | `0x802DCDB8` | `r1+96` / `r1+48` | `r1+48..r1+72` / `r1+24..r1+44` |
| `0x802DCD70` | `r26..r31` | `0x802DCDBC` | `r1+48` | `r1+24..r1+44` |

This is what makes the bodies whose Ghidra prototype drops `this`
(`0x8072FE10`, `0x80642A20`, `0x806B5180`, `0x80740370`, `0x8066FA10`) and the
receiver-less `0x807247D0` implementable: after the helper the object is still in
entry `r3` and the first body instruction copies it into a callee-saved
register.

**House style used by the drafts below.** `pub unsafe fn fn_XXXXXXXX(this: usize)`,
`platform::read_u32/read_i32/read_f32/read_u8/read_u16`, `platform::write_*`,
`abi_call1!..abi_call6!`, addresses written as `0x8070_D790`, field offsets as
`usize` literals, and a `// unknown:` comment on the first use of every
address-only target. Virtual calls are the file's existing
`let f: extern "C" fn(usize) -> f32 = core::mem::transmute(ptr);` shape.

---

## 1. `0x8072FE10` - 684 bytes - `work/ghidra/rel-d_basesNP/export/8072fe10_FUN_8072fe10.c`

**Body:** `0x8072FE10..0x807300BC` (171 words, ends `blr`). Ledger: absent.

```rust
pub unsafe fn fn_8072fe10(this: usize)
```

**Frame.** `stwu r1,-96(r1)`; LR at `100(r1)`; `addi r11,r1,96`;
`bl 0x802DCD6C` (saves `r25..r31`). Register map: **r30 = this**
(`mr r30,r3`), **r31 = 0x807CD640** (`lis r31,0x807D; addi r31,r31,-10688`),
**r7 = 0x80830A88** (`lis r7,0x8083; addi r7,r7,2696`). Epilogue
`0x807300A4..0x807300BC`.

**Shape.** Per-frame 3D "cross-hair/box" drawer: two lazily-initialised 4x4
matrices in this REL's `.bss`, four corner offsets selected by the player byte
`*(u8*)(this+0x348)`, one line per corner per count, then two closing lines.

1. `0x8072FE34-0x8072FE94` - lazy init #1:
   `if (*(u8*)0x80830B70 == 0)` (`lbz r0,232(r7)` / `extsb.` / `bne`):
   writes **16 floats** at `0x80830B78` (`addi r4,r7,240`) from
   `f1 = *(f32*)0x807CD6C0` (-8.0) and `f0 = *(f32*)0x807CD6C4` (8.0), in the
   order `f1, f0,f0,f0,f0,f0,f0, f1,f1,f1, f0, f1,f1, f0, f1,f1`
   (`stfs` at `240(r7)` then `4(r4)`..`60(r4)`); then `*(u8*)0x80830B70 = 1`.
2. `0x8072FE98-0x8072FED8` - lazy init #2:
   `if (*(u8*)0x80830BB8 == 0)` (`lbz r0,304(r7)`):
   writes **8 floats** at `0x80830BC0` (`addi r4,r7,312`) from
   `f2 = *(f32*)0x807CD648` (0.0), `f1 = *(f32*)0x807CD64C` (16.0),
   `f0 = *(f32*)0x807CD660` (-16.0) in the order
   `f2, f1,f1, f2,f2, f0,f0, f2`; then `*(u8*)0x80830BB8 = 1`.
3. `0x8072FEDC-0x8072FF44` - corner setup:
   `i = *(u8*)(this+0x348)` (`lbz r9,840(r3)`);
   `fx = *(f32*)(this+0xAC)`, `fy = *(f32*)(this+0xB0)`;
   `p = 0x80830B78 + 16*i`;
   `x0 = fx + *(f32*)(p+0)`, `y0 = fy + *(f32*)(p+4)`,
   `x1 = fx + *(f32*)(p+8)`, `y1 = fy + *(f32*)(p+12)`
   (`lfsx f1,r4,r0` / `lfs f2,4(r6)` / `lfs f0,8(r6)` / `lfs f0,12(r6)`;
   stores at `16(r1)`, `20(r1)`, `8(r1)`, `12(r1)`);
   `w0 = *(u32*)(0x807CD680 + 8*i)`, `w1 = *(u32*)(0x807CD684 + 8*i)`
   (`lwzx r27,r5,r8` / `lwz r26,4(r4)`).
4. `0x8072FF44-0x80730010` - per-count loop:
   `for (n = 0; n < *(u32*)(this+0x400); n++)` (`lwz r0,1024(r30)` /
   `cmpw cr0,r25,r0` / `blt` back-edge at `0x80730010`):
   * `bl 0x80077860([0x80429DD0], (u16)x0, (u16)(-y0), 0, (u16)w0)`
     (`fctiwz` on `x0` and on `-y0`, `stfd`/`lwz` to recover the integers,
     `rlwinm ... 0,16,31`).
   * the same with `x1`, `-y1`, `w1`.
   * `i = *(u8*)(this+0x348)`; `sx = *(f32*)(0x80830BC0 + 8*i)`,
     `sy = *(f32*)(0x80830BC4 + 8*i)`;
     `x0 += sx`, `x1 += sx`, `y0 += sy`, `y1 += sy`
     (`lfsx f5,r29,r0` / `lfs f3,4(r3)` / four `fadd` / four `stfs`).
5. `0x80730014-0x807300A0` - two closing lines with the `0x807CD6A0` table:
   `k = (i & 0x3F) * 8` (`rlwinm r28,r9,3,21,28`);
   `bl 0x80077860(gfx, (u16)x0, (u16)(-y0), 0, (u16)*(u32*)(0x807CD6A0 + k))`;
   then the same with `*(u32*)(0x807CD6A4 + k)` and `x1`/`-y1`.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x8072FF80` | `0x80077860` | `r3 = *(u32*)0x80429DD0`, `r4 = (u16)x0`, `r5 = (u16)(-y0)`, `r6 = 0`, `r7 = (u16)w0` |
| `0x8072FFBC` | `0x80077860` | `r3 = gfx`, `r4 = (u16)x1`, `r5 = (u16)(-y1)`, `r6 = 0`, `r7 = (u16)w1` |
| `0x8073005C` | `0x80077860` | `r3 = gfx`, `r4 = (u16)x0`, `r5 = (u16)(-y0)`, `r6 = 0`, `r7 = (u16)*(u32*)(0x807CD6A0 + k)` |
| `0x807300A0` | `0x80077860` | `r3 = gfx`, `r4 = (u16)x1`, `r5 = (u16)(-y1)`, `r6 = 0`, `r7 = (u16)*(u32*)(0x807CD6A4 + k)` |
| `0x8072FE20` | `0x802DCD6C` | save `r25..r31`; `r11 = r1+96` |
| `0x807300A8` | `0x802DCDB8` | restore `r25..r31`; `r11 = r1+96` |

No virtual calls.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 -8.0` | `0x807CD6C0` | `lfs f1,128(r31)` @`0x8072FE44` |
| `f32 8.0` | `0x807CD6C4` | `lfs f0,132(r31)` @`0x8072FE4C` |
| `f32 0.0` | `0x807CD648` | `lfs f2,8(r31)` @`0x8072FEA4` |
| `f32 16.0` | `0x807CD64C` | `lfs f1,12(r31)` @`0x8072FEAC` |
| `f32 -16.0` | `0x807CD660` | `lfs f0,32(r31)` @`0x8072FEB4` |
| 4x4 corner table (16 x `f32`) | `0x80830B78` | `addi r4,r7,240` @`0x8072FE48` |
| 4x2 step table (8 x `f32`) | `0x80830BC0` | `addi r4,r7,312` @`0x8072FEA8` |
| per-player line-id pair | `0x807CD680` | `addi r5,r31,64` @`0x8072FEE8` |
| per-player closing-id pair | `0x807CD6A0` | `addi r30,r31,96` @`0x80730028` |
| graphics singleton | `0x80429DD0` | `lwz r3,-25136(r28)` @`0x8072FF5C` |

Verified tables: `0x807CD680 = {0x1A,0x1B,0x1C,0x1D,0x1A,0x1B,0x1C,0x1D}` (u32),
`0x807CD6A0 = {0x10,0x11,0x16,0x17,0x12,0x13,0x14,0x15}` (u32).

**Confidence: high.** Both `.bss` matrices and both id tables are addressed by
literal `lis/addi` pairs; the only masked index is the player byte
`*(u8*)(this+0x348)` in the closing block (`& 0x3F`), which the translator should
reproduce as written. No `psq`, no `GQR`, no virtual dispatch. This body has
four direct draw calls per iteration and nothing else.

---

## 2. `0x80642A20` - 660 bytes - `work/ghidra/rel-d_basesNP/export/80642a20_FUN_80642a20.c`

**Body:** `0x80642A20..0x80642CB4` (165 words, ends `blr`). Ledger: absent.

```rust
pub unsafe fn fn_80642a20(this: usize)
```

**Frame.** `stwu r1,-112(r1)`; LR at `116(r1)`; `addi r11,r1,112`;
`bl 0x802DCD5C` (saves `r21..r31`). **r23 = this** (`mr r23,r3`).
Epilogue `0x80642C9C..0x80642CB4` (`addi r11,r1,112` / `bl 0x802DCDA8`).

**Shape.** A four-player model refresh, split on `*(u32*)(this+0x8A0) == 6`
("all four active" mode) versus the normal per-player path; both arms finish with
a shared counter decrement.

1. `0x80642A34-0x80642A44` - `if (*(u32*)(this+0x8A0) != 6) goto 0x80642B48`.
2. **mode 6 arm** `0x80642A48-0x80642B44`:
   `r22 = 0x80354E80` (flags), `r27 = 0x80354E60` (player-index table),
   `r28 = this`, `r29 = 0`, `r24 = 1`.
   `for (i = 0; i < 4; i++)` (`cmpwi cr0,r29,4` / `blt` at `0x80642B40`):
   * `id = *(s32*)(0x80354E60 + 4*i)` (`lwz r0,0(r27)`);
   * `if ((*(u32*)(0x80354E80 + 4*id) & 1) == 0 && *(u8*)(this+i+0x8D8) != 0)`
     (`rlwinm. r0,r0,0,31,31` after `lwzx r0,r22,r0`, then
     `lbz r0,2264(r3)` with `r3 = this+i`);
   * `if (!wrote_any) bl 0x80101110()` (`cmpwi cr0,r26,0` / `bne`);
   * `*(u32*)(0x80354E80 + 4*id) = 1` (`stwx r24,r22,r0` with `r0 = id*4`);
   * `obj = *(u32*)(this + 4*id + 0x2E4)`
     (`add r30,r23,r0` / `lwz r4,740(r30)`);
   * vec at `r1+48/52/56` = `{ *(f32*)(obj+0xAC), *(f32*)(obj+0xB0),
     *(f32*)(obj+0xB4) }` (`lfs f2,172` / `lfs f1,176` / `lfs f0,180`);
   * `sub = *(u32*)(*(u32*)(obj+0x128) + 4)`;
     `vcall +0x40(sub, obj)` then `vcall +0x48(sub, obj)`
     (`lwz r3,296(r4)` / `lwz r3,4(r3)` / `lwz r12,0(r3)` /
     `lwz r12,64(r12)` and `lwz r12,72(r12)`);
   * `*(u32*)(this+0x890) = id` (`stw r25,2192(r28)`);
   * `wrote_any = 1`; `bl 0x80642CE0(this, *(u32*)(obj+4) & 0xF, &vec, i)`
     (`addi r5,r1,24`, `mr r6,r29`, `rlwinm r4,r0,0,28,31`).
3. **normal arm** `0x80642B48-0x80642C2C`:
   `r25 = *(u32*)(this+0x328)` (`lwz r25,808(r3)`);
   `r28 = 0x80354E60`, `r30 = 0x80354E80`, `r31 = 0x80354E70`,
   `r22 = 0x807C8358`, `r24 = 0`;
   `for (i = 0; i < 4; i++)`:
   * `id = *(s32*)(0x80354E60 + 4*i)`; `o = id*4`;
   * `flag = *(u32*)(0x80354E80 + o) & 1` (`stb r0,8(r1)`);
   * `v = bl 0x800FC850(r25, *(u32*)(0x80354E70 + o), &flag)`
     (`lwzx r4,r31,r21` / `addi r5,r1,8`);
   * `if (v != *(u32*)(0x80354E70 + o) && *(u8*)(this+i+0x8D8) != 0)`
     (`cmpw cr0,r3,r0` / `beq`; `lbz r0,2264(r4)`);
   * `*(u32*)(0x80354E70 + o) = v` (`stwx r3,r31,r21`);
   * `obj = *(u32*)(this + o + 0x2E4)`;
     `bl 0x80759770(obj, *(u32*)(0x807C8358 + 4*(*(u32*)(this+0x8A0))))`
     (`rlwinm r0,r0,2` / `lwzx r4,r22,r0`);
   * vec at `r1+12/16/20` = `{ obj+0xAC, obj+0xB0, obj+0xB4 }`
     (the parallel copy at `r1+36/40/44` is dead);
   * `*(u32*)(this+0x890) = id`; `wrote_any = 1`;
   * `bl 0x80642CE0(this, *(u32*)(obj+4) & 0xF, &r1+12, i)`.
4. `0x80642C30-0x80642C98` - if `wrote_any`:
   * `bl 0x800BB330(*(u32*)0x80429F7C, *(u32*)(this+0x8A0))`
     (`lwz r4,2208(r23)` / `lwz r3,-24708(r3)`);
   * `m = *(u32*)(this+0x8A0)`;
     `*(u32*)(this + 4*m + 0x8B4) -= 1`
     (`lwz r4,2228(r5)` / `addi r0,r4,-1` / `stw r0,2228(r5)`);
   * `bl 0x80642CC0(this, *(u32*)(this + 4*m + 0x8B4))`;
   * `if (*(u32*)(this + 4*m + 0x8B4) == 0)`
     `*(u32*)(*(u32*)(this + 4*m + 0x2F4) + 0x204) = 1`
     (`lwz r3,756(r3)` / `li r0,1` / `stw r0,516(r3)`).

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80642A90` | `0x80101110` | none |
| `0x80642B2C` | `0x80642CE0` | `r3 = this`, `r4 = *(u32*)(obj+4) & 0xF`, `r5 = r1+24`, `r6 = i` |
| `0x80642B94` | `0x800FC850` | `r3 = *(u32*)(this+0x328)`, `r4 = *(u32*)(0x80354E70+4*id)`, `r5 = r1+8` |
| `0x80642BCC` | `0x80759770` | `r3 = obj`, `r4 = *(u32*)(0x807C8358 + 4*mode)` |
| `0x80642C18` | `0x80642CE0` | `r3 = this`, `r4 = *(u32*)(obj+4) & 0xF`, `r5 = r1+12`, `r6 = i` |
| `0x80642C44` | `0x800BB330` | `r3 = *(u32*)0x80429F7C`, `r4 = *(u32*)(this+0x8A0)` |
| `0x80642C74` | `0x80642CC0` | `r3 = this`, `r4 = *(u32*)(this + 4*m + 0x8B4)` |
| `0x80642A30` | `0x802DCD5C` | save `r21..r31`; `r11 = r1+112` |
| `0x80642CA0` | `0x802DCDA8` | restore `r21..r31`; `r11 = r1+112` |

**Virtual calls:** `+0x40` and `+0x48` on
`sub = *(u32*)(*(u32*)(obj+0x128) + 4)`, each with `r3 = sub`, `r4 = obj`.

**Constants:** `0x80354E60`, `0x80354E70`, `0x80354E80` (main.dol `.bss`,
four `s32`/`u32` words each; **zero at load**, filled at run time);
`0x80429F7C` (`.bss` pointer cell); `0x807C8358` (REL `.data`,
`{1,2,4,6,5,3,4,5}` `s32`, indexed by `*(u32*)(this+0x8A0)`).

**Confidence: medium.** Every operand is explicit, but three of the inputs are
`.bss` tables whose element meaning is only inferable: `0x80354E60` is the
player→object-id map, `0x80354E70` a per-player "current index" written back at
`0x80642BC0` and consumed by `0x800FC850`, and `0x80354E80` a per-player
"claimed" flag. The translator can implement all three literally as
`read_u32(0x80354E60 + 4*i)` etc.; the `// unknown:` note belongs on the tables,
not on any control flow. No `psq`, no `GQR`.

---

## 3. `0x80673F50` - 656 bytes - `work/ghidra/rel-d_basesNP/export/80673f50_FUN_80673f50.c`

**Body:** `0x80673F50..0x806741E0` (164 words, ends `blr`). Ledger: absent.

```rust
pub unsafe fn fn_80673f50(this: usize, flag: usize, mode: i32)
```

**Frame.** `stwu r1,-176(r1)`; LR at `180(r1)`; `addi r11,r1,128`;
`stfd f31,160` + `psq_st f31,168`; `stfd f30,144` + `psq_st f30,152`;
`stfd f29,128` + `psq_st f29,136`; `bl 0x802DCD4C` (saves `r17..r31`).
Register map: **r18 = this** (`mr r18,r3`), **r19 = flag** (`mr r19,r4`),
**r20 = mode** (`mr r20,r5`). Epilogue `0x806741B0..0x806741E0`.

**Shape.** A grid emitter (the "Lemmy throw-ball spawn" pattern): walking a
2-D grid of 16-pixel cells from the actor position, drawing one point per active
cell and occasionally spawning a burst effect.

1. `0x80673F7C-0x80673FD0` - screen constants:
   `f1 = *(f32*)(this+0xAC)`; `f0 = -*(f32*)(this+0xB0)`;
   `fctiwz` both; `x = (int)*(f32*)(this+0xAC) & 0xFFF0`
   (`rlwinm r28,r6,0,16,27`, `r6 = *(r1+52)`);
   `y = (int)(-*(f32*)(this+0xB0)) & 0xFFF0` (`rlwinm r27,r5,0,16,27`);
   `r26 = r28` (row-reset copy).
2. `0x80673FB0-0x80673FDC` - `r24 = r23 = (*(u32*)(this+0x3B4) == 2) ? 0 : 1`
   (`li r24,1` / `li r23,1` / `cmplwi cr0,r0,2` / `bne` / `li r24,0` /
   `li r23,0`).
3. `0x80673FE0-0x80673FFC` - `uVar12 = flag != 0
   ? *(u16*)(0x807C91E8 + 2*(*(u32*)(this+0x3AC))) : 0`
   (`lwz r4,940(r3)` / `rlwinm r4,r4,1,0,30` / `lhzx r25,r3,r4`).
4. `0x80674004-0x8067402C` - constant pool:
   `f29 = *(f64*)0x807C91F8` (2^52), `f30 = *(f32*)0x807C91F4` (8.0),
   `f31 = *(f64*)0x807C9200` (2^52+2^31); loop counters `r22 = 0` (row),
   `r21 = 0` (column).
5. `0x80674030-0x806741AC` - nested grid loop:
   * outer test `(row & 0xFFFF) < *(u32*)(this+0x3A4)`
     (`lwz r3,932(r18)` / `rlwinm r4,r22,0,16,31` / `cmplw` / `blt`),
     inner test `(col & 0xFFFF) < *(u32*)(this+0x3A0)`
     (`lwz r3,928(r18)` at `0x80674174`).
   * `if (r24 != 0)` (`rlwinm. r0,r24,0,16,31` / `beq 0x80674158`):
     `bl 0x80077860([0x80429DD0], (u16)x, (u16)y, 0, uVar12)`
     (`r7 = r25`, `r4 = r28 & 0xFFFF`, `r5 = r27 & 0xFFFF`, `r6 = 0`);
     then, still inside this guard (`cmpwi cr0,r20,0` / `bne 0x80674158`):
     - `if (mode == 0)`:
       + `if (flag == 0)`:
         `if (*(u32*)(this+0x3AC) != 2)`:
         `vec.x = 8.0 + (f32)(u16)x`;
         `vec.y = -(f32)(u16)y - 8.0`;
         `vec.z = *(f32*)(this+0xB4)`;
         `bl 0x8016C890(0x807EAA14, 0, &vec, 0, 0)`;
         `bl 0x8006A4B0(&r1+8, &vec)`;
         `bl 0x80198C30(*(u32*)0x80429D60, 628, &r1+8, 0)`.
       + `else` (flag != 0): same vector but
         `vec.y = 8.0 + (f32)(u16)(-y)`, and `bl 0x8016C890` only when
         `*(u32*)(this+0x3AC) != 2` (no sound call).
   * `if (*(u32*)(this+0x3B4) != 0) r24 = (r24 ^ 1) & 0xFFFF`
     (`lwz r0,948(r18)` / `cmpwi` / `beq` / `xori r3,r24,1` /
     `rlwinm r24,r3,0,16,31`);
   * `x += 0x10` (`addi r28,r28,16`); `col += 1`.
   * at row end: `x = r26`, `y += 0x10`,
     and `if (*(u32*)(this+0x3B4) != 0) { r23 ^= 1; r24 = r23 }`
     (`xori r23,r23,1` / `mr r24,r23`).

The 2^52 conversions in step 5 are the standard ladder:
`(f64)(u16)x` uses `stw r3,36(r1)` with `lis r0,0x4330` + `lfd f1,32(r1)` +
`fsub f1,f1,f29`; the signed `-(u16)y` uses `neg` + `xoris ...,0x8000` +
`lfd f0,40(r1)` + `fsub f0,f0,f31` (constant `0x4330000080000000`).

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80674054` | `0x80077860` | `r3 = *(u32*)0x80429DD0`, `r4 = (u16)x`, `r5 = (u16)y`, `r6 = 0`, `r7 = uVar12` |
| `0x806740C8` | `0x8016C890` | `r3 = 0x807EAA14`, `r4 = 0`, `r5 = r1+16`, `r6 = 0`, `r7 = 0` |
| `0x806740D8` | `0x8006A4B0` | `r3 = r1+8`, `r4 = r1+16` |
| `0x806740EC` | `0x80198C30` | `r3 = *(u32*)0x80429D60`, `r4 = 628`, `r5 = r1+8`, `r6 = 0` |
| `0x80674154` | `0x8016C890` | `r3 = 0x807EAA14`, `r4 = 0`, `r5 = r1+16`, `r6 = 0`, `r7 = 0` |
| `0x80673F78` | `0x802DCD4C` | save `r17..r31`; `r11 = r1+128` |
| `0x806741CC` | `0x802DCD98` | restore `r17..r31`; `r11 = r1+128` |

**Constants:**

| value | image address | materialised at |
|---|---|---|
| 6-entry `u16` id table | `0x807C91E8` | `addi r3,r3,-28184` @`0x80673FEC` |
| `f32 8.0` | `0x807C91F4` | `lfs f30,-28172(r4)` @`0x80674014` |
| `f64 2^52` | `0x807C91F8` | `lfd f29,-28168(r5)` @`0x80674010` |
| `f64 2^52+2^31` | `0x807C9200` | `lfd f31,-28160(r3)` @`0x8067401C` |
| sprite name `"Wm_en_burst_ss"` | `0x807EAA14` | `addi r3,r30,-21996` @`0x80674088` |
| sound singleton | `0x80429D60` | `lwz r17,-25248(r31)` @`0x806740CC` |
| graphics singleton | `0x80429DD0` | `lwz r3,-25136(r29)` @`0x80674040` |

Verified `u16` table: `0x807C91E8 = {124, 2, 12, 123, 15, 0}` (six entries; the
next word at `0x807C91F4` is the `8.0` float above).

**Confidence: high.** The six paired-single words are a pure spill of
`f29`/`f30`/`f31` (`psq_st` at `0x80673F64/6C/74`, `psq_l` at
`0x806741B4/BC/C4`). The two toggles are independent 16-bit flags (`r24` per
cell, `r23` per row), both derived from `*(u32*)(this+0x3B4)`, and the two
`0x8016C890` arms differ only in the sign of the y term and the trailing sound
call.

---

## 4. `0x806B5180` - 620 bytes - `work/ghidra/rel-d_basesNP/export/806b5180_FUN_806b5180.c`

**Body:** `0x806B5180..0x806B53EC` (155 words, ends `blr`). Ledger: absent.

```rust
pub unsafe fn fn_806b5180(this: usize) -> u32
```

**Frame.** `stwu r1,-64(r1)`; LR at `68(r1)`; `addi r11,r1,48`;
`stfd f31,48` + `psq_st f31,56`; `bl 0x802DCD6C` (saves `r25..r31`).
**r25 = this**, **r28 = 0x8082C308** (`lis r28,0x8083; addi r28,r28,-15608`),
**r29 = 0x807CAA60** (`lis r29,0x807D; addi r29,r29,-21920`).
Returns `1` in `r3` (`li r3,1` at `0x806B53CC`). Epilogue
`0x806B53C8..0x806B53EC`.

**Shape.** The "stomp actor" initialiser: a fixed virtual-call sequence, one
lazy static block of six 40-byte matrix records, then five more virtual calls.

1. `0x806B519C-0x806B51BC` - `vcall +0x300` on `*(this+0x60)` with `r3 = this`
   (`lwz r12,96(r3)` / `lwz r12,768(r12)` / `bctr`).
2. `0x806B51C0-0x806B51DC` - stores:
   `*(f32*)(this+0x6C8) = *(f32*)(r29+8)` (1.0);
   `*(f32*)(this+0x6C4) = *(f32*)(r29+4)` (0.0);
   `*(u32*)(this+0x6CC) = 0`; `*(u32*)(this+0x6D0) = 0`.
3. `0x806B51E0-0x806B51F0` - `u = vcall +0x2A4` on `*(this+0x60)` with
   `r3 = this`; `*(u16*)(this+0x6D4) = (u16)r3`.
4. `0x806B51F4-0x806B5208` - `f = vcall +0x284`; store `f` in all three of
   `*(f32*)(this+0xDC)`, `+0xE0`, `+0xE4`.
5. `0x806B520C-0x806B5240` - `f31 = 2.0 * vcall +0x288`
   (`lfs f0,80(r29)` = 2.0 / `fmul f31,f0,f1`).
6. `0x806B5244-0x806B5260` - `g = 2.0 * vcall +0x288` (a second call);
   `*(f32*)(this+0x320) = 0.0`; `*(f32*)(this+0x324) = 0.0`;
   `*(f32*)(this+0x318) = g`; `*(f32*)(this+0x31C) = f31`.
7. `0x806B5264` - `bl 0x806B53F0(this)`.
8. `0x806B5268-0x806B5278` - `vcall +0x304` on `*(this+0x60)` with `r3 = this`.
9. `0x806B527C-0x806B528C` - `*(f32*)(this+0xB4) = 0.0`
   (`lfs f13,4(r29)` / `stfs f13,180(r25)`).
10. `0x806B5288-0x806B5358` - lazy static block:
    `if (*(u32*)0x8082C32C == 0)` (`r30 = 0x8082C314`, `lwz r0,24(r30)`):
    * 24 `stfs` seed six 40-byte records at `0x8082C340`
      (`addi r27,r28,56`), in this order relative to `r27`:
      `+36 = 0.0`, `+76 = 2.0`, `+116 = 5.0`, `+156 = 8.0`, `+196 = 11.0`,
      `+236 = 14.0`, `+12/+16/+20 = 1.0`, `+52 = 1.2`, `+56 = 0.8`,
      `+60 = 1.0`, `+92 = 0.95`, `+96 = 1.15`, `+100 = 1.0`,
      `+132 = 1.1`, `+136 = 0.9`, `+140 = 1.0`, `+172 = 0.95`,
      `+176 = 1.05`, `+180 = 1.0`, `+212/+216/+220 = 1.0`
      (`lfs f7,8(r29)` = 1.0, `f4 = 108(r29)` = 0.95, `f12..f0` from
      `80..124(r29)`);
    * `r31 = r30 + 12` (`0x8082C320`); `for (k = 0; k < 6; k++)`
      (`cmpwi cr0,r26,6` / `blt 0x806B5334`):
      `bl 0x802BE800(r31, r27)`; `*(u32*)(r30+24) += 1`; `r27 += 0x28`.
11. `0x806B535C-0x806B5368` - `bl 0x800E81A0(f1 = 0.0, f2 = 14.0,
    r3 = this+0x6DC)` (`lfs f1,4(r29)` / `lfs f2,96(r29)`).
12. `0x806B536C-0x806B5378` - `bl 0x806B5A80(this)`;
    `*(u16*)(this+0x6D6) = 1`.
13. `0x806B537C-0x806B5394` - `vcall +0x28C` on `*(this+0x60)`;
    `*(f32*)(this+0x6D8) = f1`; `*(u16*)(this+0x6F4) = 0`.
14. `0x806B5398-0x806B53C4` - `vcall +0xD4` on `*(this+0x60)` with
    `r3 = this`, `r4 = 0x8082C43C`; then `vcall +0x2F8` with `r3 = this`.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x806B5264` | `0x806B53F0` | `r3 = this` |
| `0x806B533C` | `0x802BE800` | `r3 = 0x8082C320`, `r4 = 0x8082C340 + 0x28*k` |
| `0x806B5368` | `0x800E81A0` | `f1 = 0.0`, `f2 = 14.0`, `r3 = this+0x6DC` |
| `0x806B5370` | `0x806B5A80` | `r3 = this` |
| `0x806B5198` | `0x802DCD6C` | save `r25..r31`; `r11 = r1+48` |
| `0x806B53D8` | `0x802DCDB8` | restore `r25..r31`; `r11 = r1+48` |

**Virtual calls** (all on `*(this+0x60)` with `r3 = this`):
`+0x300`, `+0x2A4`, `+0x284`, `+0x288` (twice), `+0x304`, `+0x28C`,
`+0xD4` (`r4 = 0x8082C43C`), `+0x2F8`.

**Constants** (base `r29 = 0x807CAA60`):

| value | image address | materialised at |
|---|---|---|
| `f32 0.0` | `0x807CAA64` | `lfs f13,4(r29)` @`0x806B527C` |
| `f32 1.0` | `0x807CAA68` | `lfs f1,8(r29)` @`0x806B51C0` |
| `f32 2.0` | `0x807CAAB0` | `lfs f0,80(r29)` @`0x806B5230` |
| `f32 5.0` | `0x807CAAB4` | `lfs f11,84(r29)` @`0x806B52AC` |
| `f32 8.0` | `0x807CAAB8` | `lfs f10,88(r29)` @`0x806B52B0` |
| `f32 11.0` | `0x807CAABC` | `lfs f9,92(r29)` @`0x806B52B4` |
| `f32 14.0` | `0x807CAAC0` | `lfs f8,96(r29)` @`0x806B52B8` |
| `f32 1.2` | `0x807CAAC4` | `lfs f6,100(r29)` @`0x806B52BC` |
| `f32 0.8` | `0x807CAAC8` | `lfs f5,104(r29)` @`0x806B52C0` |
| `f32 0.95` | `0x807CAACC` | `lfs f4,108(r29)` @`0x806B529C` |
| `f32 1.15` | `0x807CAAD0` | `lfs f3,112(r29)` @`0x806B52C4` |
| `f32 1.1` | `0x807CAAD4` | `lfs f2,116(r29)` @`0x806B52C8` |
| `f32 0.9` | `0x807CAAD8` | `lfs f1,120(r29)` @`0x806B52CC` |
| `f32 1.05` | `0x807CAADC` | `lfs f0,124(r29)` @`0x806B52D0` |
| static guard | `0x8082C32C` | `lwz r0,24(r30)` @`0x806B5288` |
| 6 x 40-byte matrix block | `0x8082C340` | `addi r27,r28,56` @`0x806B5294` |
| message cell | `0x8082C43C` | `addi r4,r28,308` @`0x806B539C` |

**Confidence: high.** The `unaff_GQR0` region is the `stfd f31` + `psq_st`
spill pair. The lazy block writes exactly 24 floats at fixed offsets and then
calls `0x802BE800` six times with a `+0x28` stride; every constant is a verified
`.data` float. The only structural note is that the 24 `stfs` are emitted once
(they sit before the `blt` back-edge at `0x806B5358`), while the `0x802BE800`
loop is the six-iteration repeat.

---

## 5. `0x8070D790` - 584 bytes - `work/ghidra/rel-d_basesNP/export/8070d790_FUN_8070d790.c`

**Body:** `0x8070D790..0x8070D9D8` (146 words, ends `blr`). Ledger: absent.

```rust
pub unsafe fn fn_8070d790(this: usize)
```

**Frame.** `stwu r1,-64(r1)`; `mfspr r0,LR` / `stw r0,68(r1)`; `stw r31,60(r1)`;
`mr r31,r3` at `0x8070D7A0` makes **r31 = this**. Epilogue
`0x8070D9C4..0x8070D9D8`.

**Blocks.**

1. `0x8070D7A4-0x8070D7B4` - virtual slot `+0x1C` on `this+0x544`
   (`lwz r12,1348(r3)` / `lwz r12,28(r12)` / `addi r3,r3,1348` / `bctr`).
2. `0x8070D7B8-0x8070D7C8` - virtual slot `+0x14` on `this+0x5C0`
   (`lwz r12,1472(r31)` / `addi r3,r31,1472` / `lwz r12,20(r12)` / `bctr`).
3. `0x8070D7CC-0x8070D7D0` - `r3 = this`; `bl 0x8070C6D0`.
4. `0x8070D7D4-0x8070D7E4` - `if (*(u16*)(this+0x794) == 0) *(u8*)(this+0x790) = 1`
   (`lhz r0,1940(r31)` / `cmpwi cr0,r0,0` / `bne` / `li r0,1` / `stb r0,1936(r31)`).
5. `0x8070D7E8-0x8070D82C` - speed ramp on `*(f32*)(this+0x6E4)`:
   * `*(u8*)(this+0x6ED) != 0` -> `+0x6E4 += 1.0 * 1.5`
     (`lfs f0,1764` / `lis r3,0x807D; addi r3,r3,-14056` (base `0x807CC918`) /
     `lfs f2,20(r3)` = 1.0 / `lfs f1,12(r3)` = 1.5 / `fmul f1,f2,f1` /
     `fadd f0,f0,f1`);
   * else -> `+0x6E4 += 1.0`
     (`lfs f1,1764` / `lfs f0,20(r3)` / `fadd f0,f1,f0`);
   * both paths merge at `0x8070D830`.
6. `0x8070D830-0x8070D848` - clamp: `f1 = 10.0` (`lfs f1,16(r3)`), `fcmpo f0,f1`,
   `ble 0x8070D84C`; otherwise `fmr f0,f1`. Store `stfs f0,1764(r31)`.
7. `0x8070D84C-0x8070D8A8` - build the contact vector:
   `r4 = *(this+0x6DC)`, `r0 = *(this+0x6E0)`, `mulli r4,r4,0x48`,
   `mulli r0,r0,0x0C`, `add r4,r31,r4`, `add r4,r4,r0` (cell base).
   `f5 = *(f32*)(r4+0x700)`, `f3 = *(f32*)(r4+0x6FC)`, `f1 = *(f32*)(r4+0x6F8)`;
   `f4 = f5 - *(this+0xB4)`, `f2 = f3 - *(this+0xB0)`, `f0 = f1 - *(this+0xAC)`;
   scratch stores `stfs f4,40(r1)` / `stfs f0,32(r1)` / `stfs f2,36(r1)`;
   committed stores `stfs f0,232(r31)` (+0xE8), `stfs f2,236(r31)` (+0xEC),
   `stfs f4,240(r31)` (+0xF0); `r3 = this+0xE8`; `bl 0x80170980`.
8. `0x8070D8AC-0x8070D938` - advance the position by `speed * dir`:
   `f3 = *(this+0x6E4)`;
   `f6 = *(this+0xF0)*f3`, `f7 = *(this+0xEC)*f3`, `f3 = *(this+0xE8)*f3`;
   `*(f32*)(this+0xAC) = *(this+0xAC) + f3`;
   `*(f32*)(this+0xB0) = *(this+0xB0) + f7`;
   `*(f32*)(this+0xB4) = *(this+0xB4) + f6`;
   then recompute the cell base with the same `0x6DC/0x6E0` arithmetic and write
   three scratch floats at `r1+8/+12/+16` (cell `+0x6F8/0x6FC/0x700` minus the
   new position); `r3 = r1+8`; `bl 0x801C14C0`.
9. `0x8070D93C-0x8070D9A8` - `fcmpo f1, 10.0` (`lfs f0,16(r3)`); `bge 0x8070D99C`
   skips when `len >= 10.0`. Otherwise snap back to the cell
   (`*(this+0xAC/0xB0/0xB4) = *(cell+0x6F8/0x6FC/0x700)`, three `lfs`/`stfs`
   pairs at `0x8070D974-0x8070D988`) and issue virtual `+0xD4` on
   `*(this+0x60)` with `r4 = 0x8082F670`.
10. `0x8070D99C-0x8070D9C0` - `*(f32*)(this+0x6E8) += 0.2`
    (`lfs f1,1768` / `lfs f0,56(r3)` = 0.2 / `fadd`), clamp at 0.5
    (`lfs f2,52(r3)` = 0.5, `fcmpo`, `ble`, `fmr`), `stfs f0,1768(r31)`.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x8070D7D0` | `0x8070C6D0` | `r3 = this` |
| `0x8070D8A8` | `0x80170980` | `r3 = this+0xE8` |
| `0x8070D938` | `0x801C14C0` | `r3 = r1+8` (3-float vector) |

**Virtual calls:** `+0x1C` on `this+0x544`; `+0x14` on `this+0x5C0`;
`+0xD4` on `*(this+0x60)` with `r3 = this`, `r4 = 0x8082F670`.

**Constants** (base `r3 = 0x807D0000 - 0x36E8 = 0x807CC918`):

| value | image address | materialised at |
|---|---|---|
| `f32 1.5` | `0x807CC924` | `lfs f1,12(r3)` @`0x8070D804` |
| `f32 10.0` | `0x807CC928` | `lfs f1,16(r3)` @`0x8070D83C` |
| `f32 1.0` | `0x807CC92C` | `lfs f2,20(r3)` @`0x8070D800` |
| `f32 0.5` | `0x807CC94C` | `lfs f2,52(r3)` @`0x8070D9AC` |
| `f32 0.2` | `0x807CC950` | `lfs f0,56(r3)` @`0x8070D9A8` |
| message cell | `0x8082F670` | `lis r4,0x8083; addi r4,r4,-2448` @`0x8070D954` |

**Confidence: high.** Every operand, branch and store is in the stream. The
`bl 0x80170980` result is discarded (it normalises `this+0xE8` in place) and
nothing downstream depends on its return value. No data-side `psq`, no `GQR`.

---

## 6. `0x807247D0` - 584 bytes - `work/ghidra/rel-d_basesNP/export/807247d0_FUN_807247d0.c`

**Body:** `0x807247D0..0x80724A18` (146 words, ends `blr`). Ledger: absent.

```rust
pub unsafe fn fn_807247d0(this: usize, owner: usize, slot: u32)
```

**Frame.** `stwu r1,-32(r1)`; LR at `36(r1)`; `addi r11,r1,32`;
`bl 0x802DCD74` (saves `r27..r31`; `r3` survives). Register map:
**r30 = this** (`mr r30,r3`), **r29 = owner** (`mr r29,r4`),
**r27 = slot** (`mr r27,r5`), **r31 = 0x808303C0**
(`lis r31,0x8083; addi r31,r31,960`). Epilogue `0x80724A00..0x80724A18`
(`addi r11,r1,32` / `bl 0x802DCDC0`).

**Shape.** A "coin/score flip" test driven by a per-player scale table and three
menu-query string probes, with a single exit label `0x80724A00`.

1. `0x807247E4-0x8072480C` - `p = vcall +0x28(this+0x394)`; then
   `vcall +0x14(p, 0x80830518)`; if the result is nonzero skip the second probe.
   Framework of each probe: `lwz r12,916(r30)` (`+0x394`) / `lwz r12,40(r12)`
   (`+0x28`) / `bctr` returns the query object into `r3`; then
   `lwz r12,0(r3)` / `lwz r12,20(r12)` (`+0x14`) / `addi r4,r31,<string>` /
   `bctr`.
   The zero test is the compiler idiom at `0x80724824-0x80724830`:
   `neg r0,r3` / `or r0,r0,r3` / `rlwinm. r0,r0,1,31,31` / `bne`.
2. `0x80724834-0x8072486C` - second probe `0x80830558`; `r28` (init `1` at
   `0x80724800`) is cleared only when both probes return 0.
3. `0x80724870-0x80724874` - `if (r28 != 0) goto 0x80724A00` (epilogue).
4. `0x80724878-0x8072489C` - the gate:
   `r0 = (slot & 0xFF) * 4` (`rlwinm. r0,r27,2,22,29`),
   `f2 = *(f32*)(this+0xE8)`,
   `f1 = *(f32*)(0x807CD1C0 + r0)` (`lfsx f1,r4,r0`, `r4 = 0x807CD1C0`),
   `f0 = *(f32*)0x807CD1BC`,
   `if (f2 * f1 < f0) { ... } else goto 0x807249AC`.
5. `0x807248A0-0x80724960` - inner gate, re-armed with `r28 = 1`
   (`0x807248A8`) and `r27 = 1` (`0x807248AC`). Probe C `0x808304D8`; if it
   returns nonzero `bne 0x8072491C` leaves `r27 = 1`. Otherwise probe D
   `0x80830458`; if it also returns zero, `li r27,0` at `0x80724918`.
   At `0x8072491C` `if (r27 != 0) goto 0x80724960` (skip the third probe),
   otherwise probe E `0x80830498`: nonzero -> `bne 0x80724960` with `r28 = 1`;
   zero -> `li r28,0` at `0x8072495C`. At `0x80724960` `if (r28 == 0) goto
   0x807249AC`. Net effect: the flip happens iff C == 0 && D == 0 && E != 0.
6. `0x80724968-0x807249A8` - flip:
   `f0 = -*(f32*)(this+0xE8)` / `lbz r0,840(r30)` (`+0x348`) /
   `xori r0,r0,1` / `stb r0,840(r30)` / `stfs f0,232(r30)`;
   then `if (*(u8*)(owner+0x38C) == 1)` call
   `vcall +0x3F4` on `*(owner+0x60)` with `r3 = owner`, `r4 = this`, `r5 = 3`;
   `b 0x80724A00`.
7. `0x807249AC-0x807249FC` - fallback:
   `if ((*(u32*)(this+0x5A4) & 0xE) == 0) goto 0x80724A00`
   (`lwz r0,1444(r30)` / `rlwinm. r0,r0,0,28,30` / `beq`);
   `if (*(u8*)(owner+0x38C) == 1 && bl 0x80056CF0(owner, 0x3C) != 0)`
   the same `+0x3F4` vcall with `(owner, this, 3)`;
   then `bl 0x807243F0(this, owner)`.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x807247E0` | `0x802DCD74` | save `r27..r31`; `r11 = r1+32` |
| `0x807249CC` | `0x80056CF0` | `r3 = owner`, `r4 = 0x3C` |
| `0x807249FC` | `0x807243F0` | `r3 = this`, `r4 = owner` |
| `0x80724A04` | `0x802DCDC0` | restore `r27..r31`; `r11 = r1+32` |

**Virtual calls:** `+0x28` on `this+0x394` (five receiver loads: `0x807247E4`,
`0x80724834`, `0x807248A0`, `0x807248E0`, `0x80724924`);
`+0x14` on the returned query object with `r4` in
`{0x80830518, 0x80830558, 0x808304D8, 0x80830458, 0x80830498}`;
`+0x3F4` on `*(owner+0x60)` with `(owner, this, 3)` at `0x807249A4` and
`0x807249F0`.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| per-slot scale table | `0x807CD1C0` | `addi r4,r4,-11840` @`0x80724888` |
| scale threshold | `0x807CD1BC` | `lfs f0,-11844(r3)` @`0x8072488C` |
| probe string A | `0x80830518` | `addi r4,r31,344` @`0x80724814` |
| probe string B | `0x80830558` | `addi r4,r31,408` @`0x8072484C` |
| probe string C | `0x808304D8` | `addi r4,r31,280` @`0x807248C0` |
| probe string D | `0x80830458` | `addi r4,r31,152` @`0x807248F8` |
| probe string E | `0x80830498` | `addi r4,r31,216` @`0x8072493C` |

Table contents verified in the image: `0x807CD1C0 = 1.0`, `0x807CD1C4 = -1.0`,
`0x807CD1C8 = -0.1875`, `0x807CD1CC = -4.5` (`0x807CD1BC = 0.0`).

**Residual ambiguity -> conditional (medium).** The gate indexes the table with
`(slot & 0xFF) * 4`, i.e. the source expression is `table[(u8)slot]`, but only
the first four entries form a coherent 4-player scale table; the bytes at
`0x807CD1D0` onward are unrelated `.rodata` (`0xFFFFFFFF`, then 17.0, ...).
The translate should reproduce the raw `lfsx` exactly
(`read_f32(0x807CD1C0 + ((slot & 0xFF) as usize) * 4)`) and add a `// unknown:`
note that only indices 0..3 are meaningful. Nothing else is ambiguous: all five
probe strings, both `+0x3F4` calls and the `+0x5A4 & 0xE` flag test are explicit.

---

## 7. `0x80781DB0` - 564 bytes - `work/ghidra/rel-d_basesNP/export/80781db0_FUN_80781db0.c`

**Body:** `0x80781DB0..0x80781FE4` (141 words, ends `blr`). Ledger: absent.

```rust
pub unsafe fn fn_80781db0(this: usize) -> f32
```

**Frame.** `stwu r1,-160(r1)`; LR at `164(r1)`; `stfd f31,144` + `psq_st f31,152`;
`stfd f30,128` + `psq_st f30,136`; saves `r31,r30,r29`; `mr r29,r3`.
Epilogue `0x80781FB8..0x80781FE4`.

**Shape.** The cosine between two 3-vector differences of three position ids on
one owner object; this is the "angle to target" probe.

1. `0x80781DB8-0x80781DE8` - `r30 = *(u32*)0x8042A18C`
   (`lis r4,0x8043` / `lwz r30,-24180(r4)`);
   `r5 = *(u32*)(this+0x1F4)`; `*(u32*)(this+0x1FC)`;
2. `0x80781DEC-0x80781E08` - early-out:
   `if (*(u32*)(this+0x1FC) == r5) return 0.0;`
   `if (*(u32*)(this+0x1F8) == r5) return 0.0;`
   (`cmpw cr0,r0,r5` at `0x80781DEC` and `0x80781DF8`; the `0.0` is
   `lfs f1,-2408(r3)` with `r3 = 0x807D` -> `0x807CF698`).
3. `0x80781E0C-0x80781E70` - first vector:
   `bl 0x80100200(&r1+44, r30, *(u32*)(this+0x1F4))`;
   `bl 0x80100200(&r1+56, r30, *(u32*)(this+0x1FC))`;
   then the paired-single subtract:
   `psq_l f1,0(r31)` / `psq_l f0,0(r6)` / `ps_sub f2,f0,f1` /
   `psq_l f0,8(r6),0,1` / `psq_l f1,8(r31),0,1` / `ps_sub f2,f0,f1` /
   `psq_st f2,0(r5)` / `psq_st f2,8(r5),0,1` with `r31 = r1+44`, `r6 = r1+56`,
   `r5 = r1+68`; then three `lfs`/`stfs` copy `r1+68/72/76` to `r1+92/96/100`.
4. `0x80781E74-0x80781ED0` - second vector:
   `bl 0x80100200(&r1+8, r30, *(u32*)(this+0x1F4))`;
   `bl 0x80100200(&r1+20, r30, *(u32*)(this+0x1F8))`;
   `ps_sub f3,f0,f1` twice over `r31 = r1+8`, `r6 = r1+20`, stored at `r1+32`.
   Then `f2 = 0.0`, and four stores
   (`stfs f3,80(r1)`, `stfs f0,88(r1)`, `stfs f2,96(r1)`, `stfs f2,84(r1)`)
   zero the middle lane of **both** vectors, giving
   `v2 = (*(r1+32), 0.0, *(r1+40))` at `r1+80` and overwriting `v1.y` at
   `r1+96` so that `v1 = (*(r1+92), 0.0, *(r1+100))`.
5. `0x80781EC8-0x80781F18` - zero-vector rejection against main.dol's
   `0x80377DC8/CC/D0` (`fcmpu` chains at `0x80781ED4`, `0x80781EF8`,
   `0x80781F08` and `0x80781F2C`, `0x80781F40`, `0x80781F50`): if either vector is
   `(0,0,0)` -> `return 0.0`.
6. `0x80781F70-0x80781FB4` - `f31 = bl 0x801C14C0(&r1+92)`;
   `f30 = *(r1+92)* *(r1+80) + *(r1+96)* *(r1+84) + *(r1+100)* *(r1+88)`
   (three `fmul`, two `fadd`); `f1 = bl 0x801C14C0(&r1+80)`;
   `f0 = f30 / f31`; `f1 = f0 / f1` (returned).

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80781E14` | `0x80100200` | `r3 = r1+44`, `r4 = *(u32*)0x8042A18C`, `r5 = *(u32*)(this+0x1F4)` |
| `0x80781E28` | `0x80100200` | `r3 = r1+56`, `r4 = r30`, `r5 = *(u32*)(this+0x1FC)` |
| `0x80781E78` | `0x80100200` | `r3 = r1+8`, `r4 = r30`, `r5 = *(u32*)(this+0x1F4)` |
| `0x80781E8C` | `0x80100200` | `r3 = r1+20`, `r4 = r30`, `r5 = *(u32*)(this+0x1F8)` |
| `0x80781F74` | `0x801C14C0` | `r3 = r1+92` (returns `f32` in `f1`) |
| `0x80781FAC` | `0x801C14C0` | `r3 = r1+80` (returns `f32` in `f1`) |

**Constants:** `f32 0.0` `0x807CF698` (`lfs f1,-2408(r3)` @`0x80781E04`,
@`0x80781F68`); main.dol cells `0x80377DC8`, `0x80377DCC`, `0x80377DD0`
(`lis r3,0x8037` / `lfs f0,32200(r3)` etc.); matrix/owner cell
`0x8042A18C` (`lwz r30,-24180(r4)` @`0x80781DE8`).

**Confidence: high.** The paired-single block is fully specified by the `psq`
fields (`qr0`, `W=0`/`W=1`, `D=2/3`, base `r1`, offsets `0`/`8`) and the
subsequent `lfs` confirms 32-bit floats. `0x80100200` writes three floats at
its destination, which the `ps_sub` triple then consumes; the `(x, 0, z)` shape
is produced by two explicit `stfs f2` zero inserts, not a decompiler artefact.

---

## 8. `0x80740370` - 552 bytes - `work/ghidra/rel-d_basesNP/export/80740370_FUN_80740370.c`

**Body:** `0x80740370..0x80740598` (138 words, ends `blr`). Ledger: absent.

```rust
pub unsafe fn fn_80740370(this: usize)
```

**Frame.** `stwu r1,-96(r1)`; LR at `100(r1)`; `addi r11,r1,48`;
`stfd f31,80` + `psq_st f31,88`; `stfd f30,64` + `psq_st f30,72`;
`stfd f29,48` + `psq_st f29,56`; `bl 0x802DCD70` (saves `r26..r31`).
**r31 = this**, **r28 = this+0x680** (`addi r28,r3,1664`),
**r30 = this**, `r29 = 0`. Epilogue `0x80740568..0x80740598`.

**Shape.** A per-frame "trail" builder: for each active entry in the pointer
array at `this+0x680..0x680+4*n`, consume a budget counter and push the current
angle onto two 64-halfword ring buffers at `this+0x74C` / `this+0x76C`, then
mirror-fill the remaining slots.

1. `0x8074039C-0x807403D0` - setup:
   `local = *(s16*)(this+0x43E)` (`lha r5,1086(r3)` / `sth r5,8(r1)`);
   `n = *(u8*)(this+0x7ED)`; `end = this+0x680 + 4*n` (`r27`);
   `f30 = *(f32*)0x807CDFC0` (0.0); `f31 = *(f32*)0x807CDFD0` (8.0);
   `b 0x80740428`.
2. `0x80740428-0x8074042C` - outer test `r28 < r27` (`cmplw` / `blt`).
3. `0x807403D4-0x807403E0` - `p = *(u32*)r28`;
   `ang = *(s16*)(p+2)` (`lha r26,2(r3)`);
   `v = *(f32*)(p+0x14)` (`lfs f29,20(r3)`); `b 0x80740418`.
4. `0x80740418-0x80740420` - budget test:
   `fcmpo cr0,f29,f30` / `cror 2,1,2` (`CR0.EQ = CR0.GT | CR0.EQ`, i.e.
   "at least", unordered counted as at-least) / `beq 0x807403E4`; the `beq`
   branches **back** to the body, so the body repeats while
   `v >= f30` (and `r29 < 16`).
5. `0x807403E4-0x80740414` - body:
   `if (r29 == 16) goto 0x80740424` (`cmplwi cr0,r29,16` / `beq`);
   `local = *(s16*)(r30+0x74C)` (`lha r0,1868(r30)` / `sth r0,8(r1)`);
   `bl 0x8015F4F0(&local, ang, 0x200)` (`r4 = r26`, `r5 = 512`);
   `*(s16*)(r30+0x74C) = *(s16*)(r1+8)` (`lha r0,8(r1)` / `sth r0,1868(r30)`);
   `f30 += f31`; `r30 += 2`; `r29 += 1`.
6. `0x80740430-0x80740464` - publish:
   `c = *(u8*)(this+0x7EC)`; `*(u8*)(this+0x7EC) = c+1`
   (`lwz r0,2028(r31)` / `addi r7,r7,1` / `stb r7,2028(r31)`);
   `k = (c+1) & 0x3F`; `*(s16*)(this + 2*k + 0x76C) = local`
   (`rlwinm r7,r7,0,26,31` / `rlwinm r0,r7,1,0,30` / `sth r5,1900(r4)`);
   `if (r29 >= 16) goto 0x80740568` (`subfic r3,r29,16` / `bge`).
7. `0x80740468-0x80740564` - mirror fill of `m = 16 - r29` halfwords:
   `q = k - 1` (`addi r7,r7,-1`);
   `for (b = 0; b < m/8; b++)` (`rlwinm. r0,r3,29,3,31` + `mtspr CTR`;
   `beq 0x80740544`; back-edge `bdnz 0x80740474`):
   eight copies, each `q = (q - 2) & 0x3F`,
   `*(s16*)(this+0x74C + 2*r29 + 2*j + 16*b) = *(s16*)(this+0x76C + 2*q)`
   (`lha r0,1900(r4)` / `sth r0,1868(r6)`; `r6 += 16` per block);
   then `andi. r3,r3,7` and `beq 0x80740568` when the remainder is 0.
8. `0x80740544-0x80740564` - remainder loop (`mtspr CTR,r3`;
   `bdnz 0x80740548`): one halfword per iteration with the same
   `q = (q - 2) & 0x3F` recurrence and `r6 += 2`.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80740400` | `0x8015F4F0` | `r3 = r1+8`, `r4 = ang`, `r5 = 0x200` |
| `0x80740398` | `0x802DCD70` | save `r26..r31`; `r11 = r1+48` |
| `0x80740584` | `0x802DCDBC` | restore `r26..r31`; `r11 = r1+48` |

No virtual calls, no other direct calls.

**Constants:** `f32 0.0` `0x807CDFC0` (`lfs f30,-8256(r6)` @`0x807403BC`);
`f32 8.0` `0x807CDFD0` (`lfs f31,-8240(r4)` @`0x807403C4`).

**Residual ambiguity -> conditional (medium).** The pointer walk
(`this+0x680..+4*(u8)(this+0x7ED)`) and the `0x8015F4F0` call are explicit, but
the ring publish/mirror-fill is structurally opaque: two 64-halfword buffers at
`this+0x74C` and `this+0x76C`, a `u8` write cursor at `this+0x7EC` masked to
`0x3F`, and a fill loop whose source index steps by **2** halfwords
(`(q - 2) & 0x3F`) while the destination advances by 1. That arithmetic must be
transcribed literally (the `& 0x3F` on the *index*, then `*2` for the byte
offset); a translator who models the ring as a plain array will get it wrong.
Mark it `medium`, implement with a `// unknown:` note on the two buffers, and do
not "simplify" the `-2` stride.

---

## 9. `0x806FDD50` - 536 bytes - `work/ghidra/rel-d_basesNP/export/806fdd50_FUN_806fdd50.c`

**Body:** `0x806FDD50..0x806FDF68` (134 words, ends `blr`). Ledger: absent.

```rust
pub unsafe fn fn_806fdd50(this: usize)
```

**Frame.** `stwu r1,-16(r1)`; LR at `20(r1)`; `stw r31,12(r1)`; `mr r31,r3`.
Epilogue `0x806FDF54..0x806FDF68`.

**Shape.** A six-state timer/sequence machine on `*(s32*)(this+0x21C)`; the
state writer is `stw ...,540(this)` and the countdown is `*(s32*)(this+0x214)`.

1. `0x806FDD64-0x806FDD98` - dispatch: six `cmpwi`/`beq` on
   `*(s32*)(this+0x21C)` for values 0..5, else `b 0x806FDF54` (epilogue).
2. **state 0** `0x806FDD9C-0x806FDDE8`:
   `addic. r0,r4,-1` / `stw r0,532(r3)` / `bgt 0x806FDF54`;
   `p = *(u32*)0x8042A440`; `vcall +0x10(p)` (`lwz r12,0(r3)` / `lwz r12,16(r12)`);
   `addi r0,r3,-1` / `cntlzw r0,r0` / `rlwinm. r0,r0,27,5,31` (i.e.
   `(clz(ret-1) >> 5) != 0`, only true when `ret == 1`); if false -> epilogue.
   Then `bl 0x800B4A40`; `*(this+0x214) = 150`; `*(this+0x21C) = 1`.
3. **state 1** `0x806FDDEC-0x806FDE10`: `--*(this+0x214)`; `bgt` epilogue;
   `bl 0x800B4B70`; `*(this+0x214) = 0`; `*(this+0x21C) = 2`.
4. **state 2** `0x806FDE14-0x806FDE78`:
   `bl 0x800B4B40`; `if (ret != 0) goto epilogue`.
   `if (*(u8*)(this+0x704) != 0) bl 0x800B4AA0 else bl 0x800B4A70`;
   `bl 0x8019BED0(*(u32*)0x8042A4A8, 2)`;
   `bl 0x80195380(*(u32*)0x8042A488, 1500, 1)`;
   `bl 0x806FDCA0(this, 45)`; `*(this+0x214) = 540`; `*(this+0x21C) = 3`.
5. **state 3** `0x806FDE7C-0x806FDEF4`:
   `--*(this+0x214)`; `r3 = bl 0x806FDC50`; the combined test
   `if (*(s32*)(this+0x214) < 1 || r3 != 0)` is emitted as
   `cmpwi cr0,r0,0` (the timer) / `neg r4,r3` / `or r3,r4,r3` /
   `rlwinm r0,r3,1,31,31` **without `Rc`** / `ble` / `cmpwi cr0,r0,0` /
   `beq` (see `0x806FDE8C-0x806FDEB0`). The missing `Rc` is what makes the
   idiom work: CR0 still holds the timer comparison when `ble 0x806FDEAC`
   fires, and the second `cmpwi` then tests only `r3 != 0`.
   If neither holds -> epilogue. Then `if (r3 != 0)` play `1498` else `1499`
   (`bl 0x80195380(*(u32*)0x8042A488, id, 1)` at `0x806FDEC4` / `0x806FDEDC`);
   `bl 0x800B4BA0`; `*(this+0x214) = 0`; `*(this+0x21C) = 4`.
6. **state 4** `0x806FDEF8-0x806FDF30`: `bl 0x800B4B40`; `if (ret != 0) goto epilogue`;
   `if (*(s32*)(this+0x210) > 1) { r3 = bl 0x8005F900(*(u32*)(this+0x208));
   if (r3 != 0) bl 0x800B4C30(*(u32*)(r3+0x14D4)); }`;
   `*(this+0x21C) = 5`.
7. **state 5** `0x806FDF34-0x806FDF50`:
   `lwzu r12,404(r3)` sets `r3 = this+0x194`, `r12 = *(this+0x194)`;
   `lwz r12,24(r12)` (`+0x18`); `r4 = 0x8082EF88`; `bctr`;
   `*(this+0x21C) = 6`; `b 0x806FDF54`.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x806FDDD4` | `0x800B4A40` | none |
| `0x806FDDFC` | `0x800B4B70` | none |
| `0x806FDE14` | `0x800B4B40` | none |
| `0x806FDE2C` | `0x800B4AA0` | none |
| `0x806FDE34` | `0x800B4A70` | none |
| `0x806FDE44` | `0x8019BED0` | `r3 = *(u32*)0x8042A4A8`, `r4 = 2` |
| `0x806FDE58` | `0x80195380` | `r3 = *(u32*)0x8042A488`, `r4 = 1500`, `r5 = 1` |
| `0x806FDE64` | `0x806FDCA0` | `r3 = this`, `r4 = 45` |
| `0x806FDE88` | `0x806FDC50` | `r3 = this` (returns `u32`) |
| `0x806FDEC4` | `0x80195380` | `r3 = *(u32*)0x8042A488`, `r4 = 1498`, `r5 = 1` |
| `0x806FDEDC` | `0x80195380` | `r3 = *(u32*)0x8042A488`, `r4 = 1499`, `r5 = 1` |
| `0x806FDEE0` | `0x800B4BA0` | none |
| `0x806FDEF8` | `0x800B4B40` | none |
| `0x806FDF14` | `0x8005F900` | `r3 = *(u32*)(this+0x208)` |
| `0x806FDF24` | `0x800B4C30` | `r3 = *(u32*)(r3+0x14D4)` |

**Virtual calls:** `+0x10` on `*(u32*)0x8042A440` (state 0, no explicit `r3`
argument beyond the receiver); `+0x18` on `this+0x194` with `r4 = 0x8082EF88`
(state 5).

**Constants:** `0x8042A440`, `0x8042A4A8`, `0x8042A488` (SDA pointer cells);
`0x8082EF88` (message cell); literals `150`, `540`, `45`, `1498`, `1499`.

**Confidence: high.** The state numbers are literal stores; the only subtlety is
the fused `(timer < 1 || fn_806fdc50() != 0)` test, whose two halves are the
`addi`/`neg`/`or`/`rlwinm` (no `Rc`) and the preceding `cmpwi`. The
`beq`/`ble` pair at `0x806FDEC4` selects the sound id from the *same*
`r3 != 0` predicate.

---

## 10. `0x80748B20` - 520 bytes - `work/ghidra/rel-d_basesNP/export/80748b20_FUN_80748b20.c`

**Body:** `0x80748B20..0x80748D28` (130 words, ends `blr`). Ledger: absent.

```rust
pub unsafe fn fn_80748b20(this: usize)
```

**Frame.** `stwu r1,-80(r1)`; LR at `84(r1)`; `stfd f31,64` + `psq_st f31,72`;
saves `r31,r30,r29`; **r29 = this**, **r30 = 0x807CE4C0**
(`lis r30,0x807D; addi r30,r30,-6976`), **r31 = 0x8043** (`lis r31,0x8043`).
Epilogue `0x80748D04..0x80748D28`.

**Shape.** A "score popup" drawer: it places one to four sprite quads at a
global-prop height or at a tracked actor, in a mode-dependent x offset.

1. `0x80748B50-0x80748B84` - base vector:
   `y = *(f32*)(*(u32*)0x80429E00 + 0x38) + *(f32*)(...+0x40)`
   (`lfs f1,56(r4)` / `lfs f0,64(r4)` / `fadd`);
   `z = *(f32*)(r30+24)` (8.0) + y (`fadd f0,f0,f1`);
   `*(f32*)(r1+32) = *(f32*)(r30+16)` (0.0);
   `*(f32*)(r1+36) = *(f32*)(r30+20)` (1500.0);
   `*(f32*)(r1+28) = z`.
2. `0x80748B84-0x80748C28` - mode select on `*(u8*)(this+0x3EA)`:
   * `== 0`: `u = bl 0x800B2EE0(127)`; `*(f32*)(r1+32) =
     *(f32*)(obj+0x3C) - (*(f32*)(r30+28) (32.0) + (f32)u)`
     (`lfs f2,60(r3)` / `fadd f0,f0,f1` / `fsub f0,f2,f0`).
   * `== 1`: `a = bl 0x80064740(this, &r1+8)`; `if (a == 0) goto epilogue`;
     `*(f32*)(r1+32) = *(f32*)(a+0xB0) + *(f32*)(a+0xD4)`
     (`lfs f1,176(r3)` / `lfs f0,212(r3)` / `fadd`).
   * else: `u = bl 0x800B2EE0(*(u32*)(this+0x3E4))`;
     `*(f32*)(r1+32) = (*(f32*)(this+0x3E0) + 16.0 * (f32)u) - 8.0`
     (`lfs f2,32(r30)` = 16.0 / `lfs f1,992(r29)` / `lfs f0,24(r30)` = 8.0).
3. `0x80748C2C-0x80748CC4` - build the quad vector at `r1+16/20/24`:
   `*(f32*)(r1+16) = *(f32*)(r1+28)`; `*(f32*)(r1+24) = *(f32*)(r1+36)`;
   `mode = *(u8*)(this+0x3EC)`; `sub = *(u8*)(this+0x3ED)`;
   `*(f32*)(r1+20) = *(f32*)(r1+32) + *(f32*)(0x807CE4C0 + 4*mode)`
   (`lfsx f0,r3,r4` with `r3 = r30`);
   `d = 16.0 * (f32)sub` (`lfs f2,32(r30)` / `lfs f0,40(r1)` / `fsub` /
   `fmul`, after `stw sub,44(r1)` + `lis r6,0x4330` + `stw r6,40(r1)` +
   `lfd f0,40(r1)` - `lfd f3,48(r30)`);
   `if ((mode & 1) == 0) { if (mode == 2) *(r1+20) += d; }`
   (`rlwinm. r0,r5,0,31,31` / `cmplwi cr0,r5,2`);
   `else { *(r1+20) += 0.5 * d; if (mode == 3) *(r1+20) += d; }`
   (`lfs f0,36(r30)` = 0.5 / `fmul` / `fadd`; `cmplwi cr0,r5,3`);
   `step = 14.0 + d` (`lfs f0,40(r30)` / `fadd f31,f0,f2`).
4. `0x80748CC8-0x80748D00` - `for (i = 0; i < *(u8*)(this+0x3EC) + 1; i++)`:
   `bl 0x80064610(127, 4, &r1+16, 0, 0)` (`li r3,127` / `li r4,4` /
   `addi r5,r1,16` / `li r6,0` / `li r7,0`);
   `*(f32*)(r1+20) -= step` (`lfs f0,20(r1)` / `fsub f0,f0,f31` / `stfs`).

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80748B8C` | `0x800B2EE0` | `r3 = 127` |
| `0x80748BD0` | `0x80064740` | `r3 = this`, `r4 = r1+8` |
| `0x80748BF4` | `0x800B2EE0` | `r3 = *(u32*)(this+0x3E4)` |
| `0x80748CE0` | `0x80064610` | `r3 = 127`, `r4 = 4`, `r5 = r1+16`, `r6 = 0`, `r7 = 0` |

**Constants** (base `r30 = 0x807CE4C0`):

| value | image address | materialised at |
|---|---|---|
| `f32 0.0` / `7.0` / `14.0` / `21.0` (4-entry x table) | `0x807CE4C0..0x807CE4CC` | `addi r3,r30,0` @`0x80748C38` |
| `f32 0.0` | `0x807CE4D0` | `lfs f3,16(r30)` @`0x80748B50` |
| `f32 1500.0` | `0x807CE4D4` | `lfs f2,20(r30)` @`0x80748B68` |
| `f32 8.0` | `0x807CE4D8` | `lfs f0,24(r30)` @`0x80748B70` |
| `f32 32.0` | `0x807CE4DC` | `lfs f0,28(r30)` @`0x80748BA0` |
| `f32 16.0` | `0x807CE4E0` | `lfs f2,32(r30)` @`0x80748C4C` |
| `f32 0.5` | `0x807CE4E4` | `lfs f0,36(r30)` @`0x80748C9C` |
| `f32 14.0` | `0x807CE4E8` | `lfs f0,40(r30)` @`0x80748CBC` |
| `f64 2^52` | `0x807CE4F0` | `lfd f3,48(r30)` @`0x80748C44` |
| global-prop object | `0x80429E00` | `lwz r4,-25088(r31)` @`0x80748B54` |

**Residual ambiguity -> conditional (medium).** The `== 0` and `else` arms read
three floats from the object at `*(u32*)0x80429E00` (`+0x38`, `+0x3C`, `+0x40`)
and the `== 1` arm reads `+0xB0` and `+0xD4` from the actor returned by
`0x80064740`; those layouts are not visible in this REL. The offsets are exact,
so implement them literally with `// unknown:` notes; what cannot be asserted is
which of the three floats is the anchor height and which two are a bounding
box. Everything else (the 4-entry x table `{0,7,14,21}`, the `0.5 *` and `+d`
mode adjustments, the `14.0 + d` step, and the `0x80064610(127, 4, ...)` loop)
is explicit.

---

## 11. `0x80780FA0` - 520 bytes - `work/ghidra/rel-d_basesNP/export/80780fa0_FUN_80780fa0.c`

**Body:** `0x80780FA0..0x807811A8` (130 words, ends `blr`). Ledger: absent.

```rust
pub unsafe fn fn_80780fa0(this: usize)
```

**Frame.** `stwu r1,-64(r1)`; LR at `68(r1)`; `stfd f31,48` + `psq_st f31,56`;
`stfd f30,32` + `psq_st f30,40`; saves `r31,r30,r29,r28`;
**r28 = this**; **r30 = 0x807CF608** (`lis r30,0x807D; addi r30,r30,-2552`).
Epilogue `0x80781178..0x807811A8`.

**Shape.** A "how close is the enemy" estimator that feeds one of four
animation/denominator values through `0x807805C0`, then smooths
`*(f32*)(this+0x10C)`.

1. `0x80780FD8-0x80780FFC`:
   `f31 = bl 0x80780A90(this)`; `f30 = *(f32*)(this+0x10C)`;
   `u = bl 0x8079A3E0(*(u32*)0x8042A1A0)`;
   `r3 = bl 0x80781880(this, *(u32*)(this+0x200), u)`.
2. `0x80781000-0x80781004` - `if (r3 != 0) goto 0x80781178` (epilogue only).
3. `0x80781008-0x80781014` - `r3 = bl 0x807818F0(this)`;
   `if (r3 == 0) goto 0x80781064` (block B), else fall through to block A.
4. **block A** `0x80781018-0x80781060`:
   `f1 = *(f32*)0x807CF69C` (1.0); `f0 = *(f32*)(r30+32+68)` = `0x807CF66C` (0.75);
   `f31 = f1 - f0` (0.25);
   `r3 = bl 0x80780A40(f1 = f31, r3 = this)` (returns `u32`);
   `f1 = (f32)(u32)r3` via `xoris r3,r3,0x8000` + `lis r0,0x4330` +
   `lfd f2,400(r30)` (= `0x807CF798`, `2^52+2^31`);
   `f0 = *(f32*)(r30+148)` (1.0); `f1 = f31 * f1`; `f1 = f0 - f1`;
   `bl 0x807805C0(f1, this)`; `b 0x8078115C`.
5. **block B** `0x80781064-0x80781158`:
   `if (*(s32*)(this+0x200) == 0 || *(s32*)(this+0x200) == 1) goto 0x8078115C`;
   `r29 = bl 0x80780B80(this)`; if `*(s32*)(r29+0x188) == 0` then `r31 = 0`
   else `r31 = bl 0x80780B80(r29)` (note: `r3` is *not* reloaded, it still holds
   `r29` from the previous call);
   if `r31 != 0`: `bl 0x801C1650(r31+0xAC, this+0xAC)` then `bl 0x802BE470()`;
   if `r31 != 0`: `bl 0x801C1650(r31+0xAC, r29+0xAC)` then `bl 0x802BE470()`;
   `f1 = bl 0x807812E0(this)`;
   `fcmpo f31,f1` / `bge 0x807810F8`:
   * `f31 < f1`: `bl 0x807805C0(*(f32*)0x807CF7A0 = 0.1, this)`;
     `*(s32*)(this+0x2A8) = 2`; `b 0x8078115C`.
   * else at `0x807810F8`: `f0 = 70.0 * 1.0` (`lfs f1,76(r3)` / `lfs f0,60(r3)`,
     `r3 = 0x807CF628`); `fcmpo f31,f0`; `bge 0x80781128`:
     - `f31 < 70`: `bl 0x807805C0(0.75, this)`; `*(this+0x2A8) = 2`.
     - else: `f0 = 70.0 * 1.2`; `fcmpo f31,f0`; `ble 0x80781150`:
       + `f31 > 84`: `bl 0x807805C0(1.2, this)`; `*(this+0x2A8) = 1`.
       + else: `bl 0x807805C0(1.0, this)`.
6. `0x8078115C-0x80781174` - `*(f32*)(this+0x10C) =
   0.9 * f30 + 0.1 * *(f32*)(this+0x10C)` (`lfs f1,464(r30)` = `0x807CF7D8`;
   `lfs f2,268(r28)`; `lfs f0,408(r30)` = `0x807CF7A0`; two `fmul`, one `fadd`,
   `stfs f0,268(r28)`). Because `f30` was loaded from the same field, the net
   value is unchanged; reproduce the two-term expression literally.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80780FD8` | `0x80780A90` | `r3 = this` (returns `f32`) |
| `0x80780FEC` | `0x8079A3E0` | `r3 = *(u32*)0x8042A1A0` |
| `0x80780FFC` | `0x80781880` | `r3 = this`, `r4 = *(u32*)(this+0x200)`, `r5 = u` |
| `0x8078100C` | `0x807818F0` | `r3 = this` |
| `0x8078102C` | `0x80780A40` | `f1 = 0.25`, `r3 = this` (returns `u32`) |
| `0x8078105C` | `0x807805C0` | `f1`, `r3 = this` |
| `0x8078107C` | `0x80780B80` | `r3 = this` |
| `0x80781090` | `0x80780B80` | `r3 = r29` (previous result) |
| `0x807810B0` | `0x801C1650` | `r3 = r31+0xAC`, `r4 = this+0xAC` |
| `0x807810B4` | `0x802BE470` | none |
| `0x807810C8` | `0x801C1650` | `r3 = r31+0xAC`, `r4 = r29+0xAC` |
| `0x807810CC` | `0x802BE470` | none |
| `0x807810D4` | `0x807812E0` | `r3 = this` (returns `f32`) |
| `0x807810E8` | `0x807805C0` | `f1 = 0.1`, `r3 = this` |
| `0x80781118` | `0x807805C0` | `f1 = 0.75`, `r3 = this` |
| `0x80781140` | `0x807805C0` | `f1 = 1.2`, `r3 = this` |
| `0x80781158` | `0x807805C0` | `f1 = 1.0`, `r3 = this` |

**Constants** (base `r30 = 0x807CF608`):

| value | image address | materialised at |
|---|---|---|
| `f32 1.2` | `0x807CF660` | `lfs f0,56(r3)` @`0x80781128` (`r3 = r30+32`) |
| `f32 1.0` | `0x807CF664` | `lfs f0,60(r3)` @`0x80781100` |
| `f32 1.2` | `0x807CF668` | `lfs f1,64(r3)` @`0x80781138` |
| `f32 0.75` | `0x807CF66C` | `lfs f0,68(r3)` @`0x80781020` |
| `f32 70.0` | `0x807CF674` | `lfs f1,76(r3)` @`0x807810FC` |
| `f32 1.0` | `0x807CF69C` | `lfs f1,148(r30)` @`0x8078101C` |
| `f64 2^52+2^31` | `0x807CF798` | `lfd f2,400(r30)` @`0x80781040` |
| `f32 0.1` | `0x807CF7A0` | `lfs f1,408(r30)` @`0x807810E0` |
| `f32 0.9` | `0x807CF7D8` | `lfs f1,464(r30)` @`0x8078115C` |
| manager cell | `0x8042A1A0` | `lwz r3,-24160(r3)` @`0x80780FE8` |

**Confidence: high.** The `unaff_GQR0` region in the Ghidra export is the
`stfd f31/f30` + `psq_st` spill pair (verified: `psq_st f31,56(r1),0,0` and
`psq_st f30,40(r1),0,0` in the prologue, `psq_l` at `0x8078117C`/`0x80781184`).
The twice-written `*(this+0x10C)` is a genuine `0.9*x + 0.1*x` expression, not a
decompiler fold.

---

## 12. `0x80647E60` - 512 bytes - `work/ghidra/rel-d_basesNP/export/80647e60_FUN_80647e60.c`

**Body:** `0x80647E60..0x80648060` (128 words, ends `blr`). Ledger: absent.

```rust
pub unsafe fn fn_80647e60(this: usize, packed: u32, reset: i32)
```

**Frame.** `stwu r1,-32(r1)`; LR at `36(r1)`; saves `r31,r30,r29,r28`;
**r29 = this** (`mr r29,r3`), **r30 = packed** (`mr r30,r4`),
**r31 = 0x80827D00** (`lis r31,0x8082; addi r31,r31,32000`).
Epilogue `0x80648040..0x80648060`.

**Shape.** A spawn/initialiser for a "packed descriptor" actor: it chooses one of
four message strings from the reset flag and two SDA bytes, then unpacks the
packed word into five actor fields.

1. `0x80647E68-0x80647E90` - `if (reset != 0)`:
   `if (((packed >> 20) & 0xF) != 0 || ((packed >> 24) & 0xF) != 0)`
   (`rlwinm. r0,r4,12,28,31` / `rlwinm. r0,r4,8,28,31`):
   * `*(u8*)(this+0x1A4F7) = *(u8*)0x80429D18` (`lbz r0,-25320(r5)`);
   * `if (*(u8*)0x80429D18 == 0)` vcall `+0x18` on `this+0x394` with
     `r4 = 0x80827DD0`; else the same vcall with `r4 = 0x80827E10`;
   * `b 0x80647FCC`.
2. `0x80647EEC-0x80647FB4` - the "no reset / zero descriptor" path:
   `if (*(u32*)0x80429D20 == 0 && *(u32*)0x80429D24 == 0)` ->
   vcall `+0x18` with `r4 = 0x80827D90` (`0x80647FB8`), `b 0x80647FCC`.
   Otherwise:
   * `*(u8*)(this+0x1A4F7) = *(u8*)0x80429D19`
     (`addi r4,r4,-25320` -> `0x80429D18`, `lbz r4,1(r4)`, `stb r4,-23305(r3)`
     with `r3 = this+0x20000`);
   * `if (bl 0x800E45E0(*(u32*)0x8042A078) != 0)` (`rlwinm. r0,r3,0,24,31`):
     - `*(u8*)(this+0x1A4F6) = bl 0x800E45B0(*(u32*)0x8042A078, u8)` (returns `u8`);
     - `bl 0x800E45C0(mgr, u8)` returns a 64-bit pair: `r3` stored at
       `this+0x1A490`, `r4` stored at `this+0x1A494`
       (`stw r4,-23404(r5)` / `stw r3,-23408(r5)`);
     - `*(f32*)(this+0xB0) = bl 0x800E4590(mgr, u8)` (`stfs f1,176(r29)`);
     - `f1 = bl 0x800E45A0(mgr, u8)`; `frsp f0,f1`;
       `*(f32*)(this+0x1A4B8) = f1`; `*(f32*)(this+0x1A49C) = *(f32*)(this+0xB0) + f0`;
     - `bl 0x80648470(this)`;
   * vcall `+0x18` on `this+0x394` with `r4 = 0x80827D50` (`0x80647F9C`).
3. `0x80647FCC-0x80647FF0` - direction table:
   `idx = ((packed >> 26) & 7) * 4` (`rlwinm r0,r30,6,27,29`);
   `f0 = *(f32*)(0x807C8498 + idx)`; `*(f32*)(this+0x1A4B4) = f0`;
   `if ((packed as i32) < 0)` (the CR0-setting instruction is
   `rlwinm. r0,r30,1,31,31`; mask field `31,31` is the LSB, so this is the sign
   bit of `packed`)
   `*(f32*)(this+0x1A4B4) = -f0`.
4. `0x80647FF4-0x8064802C` - `*(f32*)(this+0x1A4B0) =
   16.0 * (f32)(u32)((packed >> 12) & 0xFF)`:
   `rlwinm r4,r30,20,24,31` (no `Rc`); `stw r4,12(r1)`; `lis r0,0x4330`;
   `stw r0,8(r1)`; `lfd f1,8(r1)`; `lfd f2,0x807C84E0`; `fsub f1,f1,f2`;
   `lfs f0,0x807C84D8`; `fmul f0,f0,f1`; `stfs f0,-23376(r5)`.
5. `0x80648030-0x8064803C` - copy the two SDA words into the actor:
   `*(u32*)(this+0x1A48C) = *(u32*)0x80429D24`;
   `*(u32*)(this+0x1A488) = *(u32*)0x80429D20`.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80647F20` | `0x800E45E0` | `r3 = *(u32*)0x8042A078` |
| `0x80647F38` | `0x800E45B0` | `r3 = mgr`, `r4 = *(u8*)(this+0x1A4F7)` |
| `0x80647F4C` | `0x800E45C0` | `r3 = mgr`, `r4 = *(u8*)(this+0x1A4F7)` |
| `0x80647F64` | `0x800E4590` | `r3 = mgr`, `r4 = *(u8*)(this+0x1A4F7)` |
| `0x80647F78` | `0x800E45A0` | `r3 = mgr`, `r4 = *(u8*)(this+0x1A4F7)` |
| `0x80647F98` | `0x80648470` | `r3 = this` |

**Virtual calls:** `+0x18` on `this+0x394` with
`r4 in {0x80827DD0, 0x80827E10, 0x80827D90, 0x80827D50}`.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| direction table (8 x `f32`) | `0x807C8498` | `addi r4,r4,-31592` @`0x80647FDC` |
| `f32 16.0` | `0x807C84D8` | `lfs f0,-31528(r3)` @`0x80648014` |
| `f64 2^52` | `0x807C84E0` | `lfd f2,-31520(r3)` @`0x80648004` |
| string `0x80827DD0` | `0x80827DD0` | `addi r4,r31,208` @`0x80647ED8` |
| string `0x80827E10` | `0x80827E10` | `addi r4,r31,272` @`0x80647EC0` |
| string `0x80827D50` | `0x80827D50` | `addi r4,r31,80` @`0x80647FA4` |
| string `0x80827D90` | `0x80827D90` | `addi r4,r31,144` @`0x80647FBC` |
| SDA flag byte | `0x80429D18` | `lbz r0,-25320(r5)` @`0x80647EAC` |
| SDA flag byte | `0x80429D19` | `lbz r4,1(r4)` @`0x80647F14` |
| SDA word A | `0x80429D20` | `lwz r0,-25312(r5)` @`0x80647EF4` |
| SDA word B | `0x80429D24` | `lwz r4,4(r4)` @`0x80647EF8` |
| resource manager | `0x8042A078` | `lwz r3,-24456(r28)` @`0x80647F1C` |

Verified table: `{0.0, 1.0, 1.0, 1.0, 0.75, 0.125, 0.25, 0.5}` at
`0x807C8498..0x807C84B4`.

**Confidence: high.** The `or. r0,r4,r0` at `0x80647EFC` is `(A | B) == 0`; the
`rlwinm. r0,r30,1,31,31` sign test is standard (mask field `31,31` is the LSB,
so the rotated word's LSB is the original sign bit). The `(f32)` conversion uses
the plain `2^52` constant at `0x807C84E0` with the *unsigned* value, unlike the
`2^52+2^31` variant used elsewhere in this file.

---

## Ready vs conditional

| # | address | bytes | verdict | reason |
|--:|---:|---:|---|---|
| 1 | `0x8072FE10` | 684 | **ready** | four draw calls, two literal `.bss` matrices, two id tables; no dispatch |
| 2 | `0x80642A20` | 660 | **ready** | all operands explicit; the three `.bss` tables get `// unknown:` notes only |
| 3 | `0x80673F50` | 656 | **ready** | grid loop, two sign arms and two toggles all literal |
| 4 | `0x806B5180` | 620 | **ready** | fixed virtual sequence + one 24-float static block |
| 5 | `0x8070D790` | 584 | **ready** | every operand/constant/call explicit |
| 6 | `0x807247D0` | 584 | **conditional** | `0x807CD1C0` indexed by `(u8)slot`; only indices 0..3 are a coherent table |
| 7 | `0x80781DB0` | 564 | **ready** | `qr0` paired-single block fully field-encoded; one `f32` return |
| 8 | `0x80740370` | 552 | **conditional** | two 64-halfword rings and a `-2`-stride mirror fill; structurally opaque |
| 9 | `0x806FDD50` | 536 | **ready** | six literal state stores; all ids/pointers explicit |
| 10 | `0x80748B20` | 520 | **conditional** | three-float layout of `[0x80429E00]` and of the `0x80064740` actor |
| 11 | `0x80780FA0` | 520 | **ready** | four literal blend values; `psq` is only the `f31/f30` spill |
| 12 | `0x80647E60` | 512 | **ready** | four literal strings + 8-entry direction table; bitfield masks explicit |

**9 ready / 3 conditional** (one of the nine, `0x80642A20`, is specified at
`medium` because three of its inputs are `.bss` tables). All three conditionals
are implementable today - each has a literal transcription that matches the
shipped bytes - but each requires one naming/typing guess (`// unknown:`), so
they are specified at `medium` rather than `high`. None needs the GQR3 word, a
jump-table generator or another artifact.

## Verification

- `ledger/functions.csv`: none of the twelve occurs in the `address` column.
  Mentions in other rows' evidence text (for example `0x80642A20` in the
  `80643e50` row) do not count as claims, matching the rule recorded at
  `docs/SPEC_WAVE129_BASES.md:1464-1468`.
- `docs/SPEC_WAVE128_BASES.md` / `docs/SPEC_WAVE129_BASES.md`: the twelve `## N.`
  headings of each were extracted (12 + 12) and subtracted; callee mentions do
  not exclude a candidate.
- `src/game/bases.rs`: no `pub unsafe fn fn_<address>` for any of the twelve.
- `work/ghidra/rel-d_basesNP/export/functions.tsv`: every `bytes` value above
  matches the TSV row exactly.
- Each body was decoded from `work/rels/elf/d_basesNP.elf` with the
  program-header offset rule; all twelve end in `blr` at the last word of their
  extent, and all internal branch targets are inside the extent.
- Paired-single audit: 40 `psq*` words total; all use index 0 (`qr0`);
  zero `mtspr` to any `GQRn`.
- SHA-1 of each extent differs pairwise (no duplicate body).
