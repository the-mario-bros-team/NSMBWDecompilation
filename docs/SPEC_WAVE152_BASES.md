# Wave 152 - `game::bases` - the next eight (648-628 byte band)

Read-only specification lane. No Rust file, no `ledger/functions.csv` row and no
existing document was modified by this lane. The only file written is this one.
No merge or check tool was run. The throwaway decoder and image readers live under
`%TEMP%\opencode\w152` (`pick.js`, `dis.js`, `facts.js`, `q.js`, `ph.js`,
`md.js`, `mddis.js`, `ptr.js`, `sda.js`, `xref.js`) plus the reused wave-143
decoder `%TEMP%\opencode\w150\ppc.js`, never in the repository.

Waves 128-151 walked this bucket from the top in size order; the last row
claimed by `docs/SPEC_WAVE151_BASES.md` is 648 bytes (`0x80638120`). This wave
continues one rung lower - **648 bytes down to 628 bytes** - and is exactly the
eight-row set that document predicted at `:147-148`:

```
0x80703DC0 648 | 0x8062BE20 644 | 0x8076B710 644 | 0x8074CBA0 636
0x8078A8F0 632 | 0x8061B890 628 | 0x80752730 628 | 0x80792570 628
```

## The eight

| # | address | bytes | words | export file | verdict |
|--:|---:|---:|---:|---|:--:|
| 1 | `0x80703DC0` | 648 | 162 | `80703dc0_FUN_80703dc0.c` | ready (high) |
| 2 | `0x8062BE20` | 644 | 161 | `8062be20_FUN_8062be20.c` | ready (high) |
| 3 | `0x8076B710` | 644 | 161 | `8076b710_FUN_8076b710.c` | ready (high) |
| 4 | `0x8074CBA0` | 636 | 159 | `8074cba0_FUN_8074cba0.c` | ready (medium) |
| 5 | `0x8078A8F0` | 632 | 158 | `8078a8f0_FUN_8078a8f0.c` | ready (high) |
| 6 | `0x8061B890` | 628 | 157 | `8061b890_FUN_8061b890.c` | ready (medium) |
| 7 | `0x80752730` | 628 | 157 | `80752730_FUN_80752730.c` | ready (high) |
| 8 | `0x80792570` | 628 | 157 | `80792570_FUN_80792570.c` | ready (medium) |

**Ready: 8. Conditional: 0.** 5,088 bytes / 1,272 words in total.

Every `bytes` value is the `size` column of
`work/ghidra/rel-d_basesNP/export/functions.tsv` verbatim, `decompiled` is
`true` for all eight, and for all eight the size **is** the true instruction
span: the decoder finds exactly one `blr`, at `address + size - 4`
(`0x80704044`, `0x8062C0A0`, `0x8076B990`, `0x8074CE18`, `0x8078AB64`,
`0x8061BB00`, `0x807529A0`, `0x807927E0`), and no earlier one. No extent is
truncated. The next TSV row never starts inside a body: the gaps after these
extents are `0x80703DC0` +8, `0x8062BE20` +12, `0x8076B710` +28,
`0x8074CBA0` +548, `0x8078A8F0` +1000, `0x8061B890` +380, `0x80752730` +12,
`0x80792570` +556 bytes (padding or unanalysed code, not part of the body).
The 28-byte hole after row 3 contains one bare `blr` stub at `0x8076B9A0`
(`0x4E800020`, no frame) and is not part of `0x8076B710`.

The decoder reports **no unknown word** in the 1,272 words: zero unknown
opcodes, zero unhandled X-form, zero unhandled paired-single form. Every `b` /
`bc` target is inside its own body (there is no `b` out of any extent at all);
every `bl` goes to a named callee outside the body.

### Why all eight are ready

The verdict rule is wave 144's, unchanged:

* **ready** - every reachable instruction is decoded, every literal is read out
  of the image or out of `main.dol`, every callee is named by address (or by its
  ledger name), and the body can be written as code with no assumed calling
  convention beyond the standard PowerPC EABI. Data in `main.dol` data/BSS
  (`0x8037xxxx`, `0x8035xxxx`), in this REL's BSS (`0x80822900..0x80834D84`), or
  in another module is an *input*, named by address.
* **conditional** - exactly one named input would still be missing.

`psq` words appear in five of the eight and are all prologue/epilogue
`f31`/`f30` spills through `qr0` with `W = 0`:

| row | `psq` words | meaning |
|--:|---|---|
| 1 | `psq_st f31,72(r1),0,qr0` / `psq_l f31,72(r1),0,qr0` | scalar `f31` spill pair, no quantisation |
| 3 | `psq_st f31,88(r1),0,qr0` / `psq_l f31,88(r1),0,qr0` | scalar `f31` spill pair |
| 4 | `psq_st f31,40(r1),0,qr0` / `psq_l f31,40(r1),0,qr0` | scalar `f31` spill pair |
| 7 | `f31` at 264/256 and `f30` at 248/240 | two scalar spill pairs |
| 8 | `psq_st f31,56(r1),0,qr0` / `psq_l f31,56(r1),0,qr0` | scalar `f31` spill pair |

Rows 2, 5 and 6 contain no `psq` word at all. No row uses `qr3` or any other
non-zero quantiser, so `GQR3 = 0x00050005` (`docs/REL_ELF_MAPPING.md:52-56`) is
not needed here. `unaff_GQR0` / `ldexpf` text still appears in the exports of
rows 1, 3, 4, 7 and 8; in every case it is the `stfd` + `psq_st` spill (rows 1,
3, 4, 8) or the two-pair prologue (row 7), i.e. decompiler noise around an
explicit `W = 0` / `qr0` field, never an unknown value. The four
`WARNING: Removing unreachable block (ram,...)` notes in rows 1, 3, 7 and 8 all
point at the epilogue `psq_l` reload.

## Method

**Address mapping.** Re-derived from the program headers of
`work/rels/elf/d_basesNP.elf`, not copied:

```
PT_LOAD  p_offset  p_vaddr     filesz    memsz
  0      0x0010F0  0x806000F0  0x1C6004  0x1C6004
  1      0x001C70F4 0x807C60F4 0x0004FC  0x0004FC
  2      0x1C75F0  0x807C65F0  0x00000C  0x00000C
  3      0x1C7600  0x807C6600  0x00A5FC  0x00A5FC
  4      0x1D1C00  0x807D0C00  0x051CB8  0x051CB8
  5      0x000000  0x80822900  0x000000  0x012484 (zero fill)
```

so `file_offset = 0x10F0 + (address - 0x806000F0) = (address - 0x80600000) +
0x1000` for every code and data address in this document, as
`docs/REL_ELF_MAPPING.md` requires. Literal pools in `0x807Cxxxx` / `0x807Dxxxx`
resolve through segments 1-4 and are read out verbatim below. `0x808xxxxx` below
`0x80822900` is real data; `0x80822900..0x80834D84` is this REL's BSS and reads
as zero at file level (`0x8083278C` in row 3 is in that BSS).

`main.dol`-resident data and code (`0x800xxxxx`, `0x802Dxxxx`, `0x8037xxxx`,
`0x8035xxxx`, `0x8042xxxx`) are read from `work/dol/main.elf` through **its
own** program headers:

```
PT_LOAD  p_offset  p_vaddr     filesz
  0      0x000200  0x80004000  0x0026C0
  1      0x0002900  0x800066C0  0x000060
  2      0x002A00  0x80006720  0x000060
  3      0x002B00  0x80006780  0x2E7260   <- all 0x800xxxxx/0x802xxxxx code
  4      0x2E9E00  0x802ED9E0  0x0002E0
  5      0x2EA100  0x802EDCC0  0x000020
  6      0x2EA200  0x802EDCE0  0x0106C0
  7      0x2FA900  0x802FE3A0  0x0532A0
  8      0x000000  0x80351680  0x000000 (memsz 0x0D6000, zero fill)
  9      0x34DC00  0x80427680  0x002540
 11      0x350200  0x8042B080  0x004B60
```

`0x80354E60..0x80354Fxx` (row 6), `0x80377C48` / `0x80377DE0` (rows 3, 8) and
`0x80429CC0` / `0x80429F7C` / `0x8042A038..0x8042A1A0` (rows 3, 5, 6, 8) are in
`main.dol`'s zero-fill BSS ranges, so they are addressed absolutely and are
runtime inputs, named by address. `0x80064AF0` (row 7) is a `bctr` thunk through
the SDA slot `r13-0x5458`; the slot is installed by `0x80101BC0` from the
table at `0x802F4A18` (`0x80101BE0` twice - a bare `blr` - and `0x80101BF0`),
so in the common case the call is a no-op on the vector and the row stays
ready: the spec transcribes the call, it does not inline the hook.

**Decoder.** Reused the wave-143 throwaway decoder
(`%TEMP%\opencode\w150\ppc.js`) and re-validated it against the facts
`docs/REL_ELF_MAPPING.md` pins: branch `LK` is bit 0 and `AA` bit 1;
conditional branches use the 14-bit `BD`; `psq_l` = 56 and `psq_st` = 60 with
the quantiser index in bits 12-14 and `W` in bit 15; `ps_madd`/`ps_mul`
(`op 4`, X-form) are decoded before `op 59/63`. One **known mislabel** was
worked around by hand: the shared decoder prints `mfspr r0,vrsave` for
`0x7C0802A6` and `mtspr lr,r0` for `0x7C0803A6`. Those two words are
`mflr r0` / `mtlr r0`, provable from the helpers themselves - `_savegpr_20` at
`0x802DCD58` spills only `r20..r31` and has no `mflr`, so the link-word slot in
every frame here is the caller's LR.

**CodeWarrior helpers.**

| row | prologue helper | registers saved | `this` kept in |
|--:|---|---|---|
| 1 | `_savegpr_20` = `0x802DCD58`, restore `_restgpr_20` = `0x802DCDA4` | `r20..r31` | `r22` |
| 3 | `_savegpr_23` = `0x802DCD64`, restore `0x802DCDB0` | `r23..r31` | `r31` |
| 4 | `_savegpr_26` = `0x802DCD70`, restore `0x802DCDBC` | `r26..r31` (+ manual `f31`) | `r29` |
| 5 | `_savegpr_22` = `0x802DCD60`, restore `0x802DCDAC` | `r22..r31` | `r31` |
| 6 | `_savegpr_17` = `0x802DCD4C`, restore `0x802DCD98` | `r17..r31` | `r23` |
| 2 | none (16-byte frame, `stw r31`) | `r31` | `r31` |
| 7, 8 | none (272/64-byte frames, explicit `stw`) | `r29..r31` | `r29` (7) / `r30` (8) |

None of the helpers has an `mflr`, so the incoming `r3..r10` and the LR slot
survive; all eight bodies save/restore the LR through the frame word.

**Call notation.** Throughout this document `callN(addr, a, b, ...)` lists the
integer argument registers `r3, r4, ...` from left to right; a callee's float
arguments are written `addr(f1 = ..., r3 = ...)` explicitly; a callee treated
as returning `f32` in `f1` is written `call_f32(...)`. `vc(...)` denotes an
indirect call through a vtable slot already loaded in `r12`
(`lwz r12,off(rtoc); lwz r12,off2(r12); mtspr ctr,r12; bctrl`). Where the
original body leaves a dead store or a dead test behind, the document says so,
because the byte-match check will otherwise look wrong.

Two paired main.dol helpers recur and are cited by their observed contract, not
by name: `0x80168FE0(f1, entry)` stores `f1` at `entry+0x30` (the clip's
end/limit frame) and `0x801680E0(f1, entry)` dispatches `entry+4`'s vtable
`+0x28` with `f1` (the clip's current frame). `0x801690E0(f1, entry)` is the
"reached end" query that compares `+0x30` with the current frame.

---

## 1. `0x80703DC0` - boat-light band histogram and colour-record claim

**Evidence.** `work/ghidra/rel-d_basesNP/export/80703dc0_FUN_80703dc0.c`, 648
bytes, 162 words, synthetic `0x80703DC0`. Body `0x80703DC0..0x80704048` (ends
`blr` at `0x80704044`). One `psq_st`/`psq_l` `f31` spill pair (`W = 0`, `qr0`).
`this` lives in `r22` after `_savegpr_20`; `r23`/`r24`/`r25`/`r26` are the band
index, the chosen actor, the list cursor and the slot end; `r27` is the slot
cursor, `r28` the player mask, `r29` the constant 1, `r30` the literal pool
base, `r31` the 5-byte histogram base.

**What it is.** One frame of the `0xE18`-byte boat-light actor (the actor whose
five `0x90`-byte light objects start at `+0x744`, whose sixteen 3-byte colour
records start at `+0xA2C`, and whose six `0x5C`-byte player-switch records
start at `+0xB84` - field map from the wave-67 boat-light cluster note in
`src/game/bases.rs` (the "`+0xB84`: six `0x5C`-byte player-switch records"
bullet). It decrements four per-kind claim timers,
walks the six actor lists, buckets every interesting actor into one of five
distance bands, records the per-band counts into the five light objects, claims
a colour record for each close actor of state 1, stores the four-bit player mask
at `+0xE11`, and returns the number of set mask bits. Wave 121 already read the
return in `r3` (`docs/COVERAGE_WAVE121_BASES.md:151`); its caller
`0x80704630` compares that value with `[0x80429CA0]` (the player count) and only
then advances the interface state.

### Signature

```rust
pub unsafe fn fn_80703dc0(this: usize) -> u32
```

`r3` = the boat-light actor. Returns `popcount(mask & 0xF)` in `r3` (Ghidra
prototypes the body `void`, which is why earlier lanes had to take `r3`
by hand).

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0xAC` | `f32` | reference position (the boat's own coordinate used for the bands) |
| `+0x744 + i*0x90 + 0x3A` | `u8` | band count written into light `i`, `i = 0..4` |
| `+0xE0A + k` | `u8` | claim timer for kind `k`, `k = 0..3` |
| `+0xE11` | `u8` | player/kind mask (low 4 bits used) |

Actor-relative fields (the walk dereferences these through the node):

| offset | type | meaning |
|---:|---|---|
| node `+0x04` | `u32` | actor pointer |
| node `+0x0C` | `u32` | next node |
| record `+0x10` (i.e. `this + 0xB84 + i*0x5C + 0x10`) | `u32` | list head of record `i` |
| actor `+0x38C` | `u8` | state byte (`lbz r0,908(r20)`); `1` and `2` are handled |
| actor `+0x60` | `u32` | vtable; slot `+0x6C` returns a pointer to the kind byte |
| actor `+0xAC` | `f32` | position component compared with `this+0xAC` |

The six slot addresses are `0xB84, 0xBE0, 0xC3C, 0xC98, 0xCF4, 0xD50`; the
loop bound is `this + 0xDAC`, so the seventh record (at `0xDAC`) is **not**
walked here. Each list node is `{ +0x04: actor, +0x0C: next }`.

### Exact behaviour

Prologue `0x80703DC0..0x80703DD8`: `stwu r1,-80(r1)`, `mflr r0`,
`stw r0,84(r1)`, `r11 = r1+64`, `stfd f31,64(r1)`,
`psq_st f31,72(r1),0,qr0`, `bl 0x802DCD58` (`_savegpr_20`, spills
`r20..r31`), `r22 = r3`.

1. `0x80703DE0..0x80703E04` - age the four timers:
   ```rust
   for k in 0..4 {
       let p = this + 0xE0A + k;
       let b = read_u8(p);
       if b != 0 { write_u8(p, b.wrapping_sub(1)); }
   }
   ```
2. `0x80703E08..0x80703E14` - `call3(0x80160AD0, r1+8, 0, 5)` (the memcpy-style
   `memset` thunk; the target `0x800046B4` is the ledger's `memset`). The five
   bytes at `r1+8` are the histogram.
3. `0x80703E18..0x80703E3C` - set the constants:
   ```rust
   let mut f31 = 42.0f32;                 // lfs f31,[0x807CC5E4]
   let mut slot = this + 0xB84;
   let end = this + 0xDAC;
   let hist = r1 + 8;
   let mut mask = 0u32;
   ```
4. `0x80703E3C..0x80703FC8` - the six-slot walk:
   ```rust
   loop {
       let mut node = read_u32(slot + 0x10);
       while node != 0 {
           let actor = read_u32(node + 4);
           let mut best = 0usize;
           let st = read_u8(actor + 0x38C);
           if st == 1 {
               let kind = (vc_get_kind(actor)) as i32;      // s8 at the returned pointer
               if (kind as u32) < 4 {                        // cmpli, unsigned
                   mask |= 1 << kind;
                   best = actor;
                   let other = call1(0x8012DBE0, actor);      // wm_find_actor_state_one
                   if other != 0 {
                       let k2 = (vc_get_kind(other)) as i32;
                       if (k2 as u32) < 4 { mask |= 1 << k2; }
                   }
               }
           } else if st == 2 {
               let kind = (vc_get_kind(actor)) as i32;
               if (kind as u32) < 4 { mask |= 1 << kind; best = actor; }
           }
           if best != 0 {
               let d = read_f32(best + 0xAC) - read_f32(this + 0xAC);
               let mut k = 0usize;
               let mut t = -105.0f32;                       // [0x807CC5E0]
               loop {
                   t += f31;                                 // -63, -21, 21, 63
                   if d < t { write_u8(hist + k, read_u8(hist + k) + 1); break; }
                   k += 1;
                   if k == 4 { write_u8(hist + 4, read_u8(hist + 4) + 1); break; }
               }
               if st == 1 {
                   let kind = (vc_get_kind(best)) as i32;
                   if (kind as u32) < 4
                       && read_u8(this + kind as usize + 0xE0A) == 0
                       && call2(0x80056CF0, best, 0x1C) != 0
                   {
                       call2(0x80702EA0, this, k as u32);     // claim colour record
                       write_u8(this + kind as usize + 0xE0A, 4);
                   }
               }
           }
           node = read_u32(node + 0x0C);
       }
       slot += 0x5C;
       if slot >= end { break; }
   }
   ```
   The `kind` getter is the same `lwz r12,96(actor); lwz r12,108(r12); bctrl`
   pair seen in row 4: the returned pointer's first byte is sign-extended
   (`lbz; extsb`) and then compared **unsigned** against 4 (`cmpli`), so
   negative kinds skip.
   `k` is the loop's final band index: `0..3` when the `d < t` test fired,
   exactly `4` when all four thresholds were exceeded.
5. `0x80703FD8..0x80703FFC` - copy the histogram into the five lights:
   ```rust
   let mut p = this + 0x744;
   for i in 0..5 { write_u8(p + 0x3A, read_u8(hist + i)); p += 0x90; }
   ```
   `0x80703FE4` is a literal `ori r0,r0,0` (nop) inside this loop.
6. `0x80704000..0x80704024` - `write_u8(this+0xE11, mask as u8)`; then
   ```rust
   let mut n = 0u32;
   let mut m = mask;
   for _ in 0..4 { n += m & 1; m >>= 1; }
   ```
   `0x8070400C` is a second literal nop before this loop.
7. `0x80704028..0x80704044` - epilogue: `psq_l f31`, `lfd f31`,
   `bl 0x802DCDA4` (`_restgpr_20`), `lwz r0,84(r1)`, `mtlr r0`,
   `addi r1,r1,80`, `blr`.

### Constants

| address / value | value | use |
|---|---:|---|
| `0x807CC5E0` | `-105.0f` | first band threshold base |
| `0x807CC5E4` | `42.0f` | band width (thresholds `-63, -21, 21, 63`) |
| `r4 = 0x1C` | `0x1C` | the flag bit tested by `0x80056CF0` |
| `r4 = 4` | `4` | per-kind claim timer written after a claim |

### Callees

| site | callee | arguments |
|---|---|---|
| `0x80703DD8` | `0x802DCD58` `_savegpr_20` | spills `r20..r31` |
| `0x80703E14` | `0x80160AD0` (memset thunk) | `r3 = r1+8`, `r4 = 0`, `r5 = 5` |
| `0x80703E8C` | `0x8012DBE0` `wm_find_actor_state_one` | `r3 = actor` |
| `0x80703FA4` | `0x80056CF0` (bit test on `+0x1014`) | `r3 = actor`, `r4 = 0x1C` |
| `0x80703FB8` | `0x80702EA0` (claim colour record, event `0x2AA`) | `r3 = this`, `r4 = band` |
| `0x80704034` | `0x802DCDA4` `_restgpr_20` | restores `r20..r31` |

Two virtual calls per state-1 actor (`actor->vt[0x6C]` on the actor and on its
companion) and one per state-2 actor; all three are plain `bctrl` through
`r12`, no argument setup beyond `r3 = object`.

### Notes for the implementer

* The five band counts are `u8` and increment without saturation (`lbzx` /
  `stbx`), so a count above 255 wraps.
* `+0xE11` is written with the *whole* mask (bits 0-3 can only be set because
  `kind < 4` gates every OR).
* The colour claim uses the *band* index, not the kind, as the second argument
  of `0x80702EA0`; `0x80702EA0` writes it into the first free 3-byte record.
* Wave 67 deferred this body for `unaff_GQR0`/`ldexpf`; the only `psq` words
  are the `f31` spill pair, so that rejection is retired here.

---

## 2. `0x8062BE20` - `dMultiCourseSelect_c` cursor step

**Evidence.** `work/ghidra/rel-d_basesNP/export/8062be20_FUN_8062be20.c`, 644
bytes, 161 words, synthetic `0x8062BE20`. Body `0x8062BE20..0x8062C0A4` (ends
`blr` at `0x8062C0A0`). No float registers, no `psq`, no save helper:
`stwu r1,-16(r1)`, `mflr r0`, `stw r0,20(r1)`, `stw r31,12(r1)`, `r31 = this`.

**What it is.** The cursor-navigation step of the multi-course-select screen
(`dMultiCourseSelect_c`, class identified in `docs/COVERAGE_WAVE104_BASES.md:30`
by its eighteen `StateID_*` descriptors). The screen is a 2 x 5 grid of course
cells (`+0x2DC` holds twenty cell pointers, this function uses entries `0..9`)
plus an off-grid sentinel `10`. `dir` is a Wii-remote D-pad mask and the four
handled values are the pad bits `1 = left`, `2 = right`, `4 = down`, `8 = up`.
The body clamps/wraps the index, consults the two per-cell availability flags,
and delegates the page crossing to `0x8062BCF0`.

Its two callers are `0x8062D8F0` and `0x8062DB40`; `0x8062D8F0` passes
`r4 = *(this+0x4F4)` (cursor), `r5 = *(this+0x508)` (last pad sample), compares
the result with `+0x4F4`, and on a change stores old -> `+0x4F8`, new ->
`+0x4F4` and plays a sound through `this+0x32C`'s vtable `+0x18`.

### Signature

```rust
pub unsafe fn fn_8062be20(this: usize, index: u32, dir: u32) -> u32
```

`r3` = the screen object, `r4` = current cursor (0..10), `r5` = D-pad mask.
Returns the new cursor (0..10). The export's `param_2` is `uint` but the body
uses signed compares (`cmpi`) on it as well, so treat the cursor as `i32` with
the range 0..10.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x2DC + i*4` | `u32[20]` | cell object pointers; this function uses entries `0..9` |
| `+0x2F0`/`+0x2F4`/`+0x2F8`/`+0x2FC`/`+0x300` | `u32` | the five bottom-row cell pointers (`i = 5..9`) |
| `+0x4F4` | `u32` | current cursor |
| `+0x4F8` | `u32` | previous cursor (written only on a page crossing) |
| `+0x500` | `u32` | page/index word maintained by `0x8062BCF0` (0, 10, or a scan result) |
| `+0x504` | `u32` | available-cell count (`>= 2` means a second page exists) |
| `+0x508` | `u32` | last pad sample (read by the caller) |
| cell `+0x248` / `+0x24C` | `u32` | two sub-object pointers (parts of the cell) |
| sub-object `+0xBB` | `u8` | bit 0 = cell is present/unlocked |

`cell_ok(cell)` below means
`(read_u8(read_u32(cell+0x248)+0xBB) & 1) != 0 || (read_u8(read_u32(cell+0x24C)+0xBB) & 1) != 0`.

### Exact behaviour

1. `dir == 2` (right), `0x8062BE3C..0x8062BE48`:
   ```rust
   if index >= 5 { index -= 5; }        // no flag test, no wrap
   ```
2. `dir == 1` (left), `0x8062BE54..0x8062BE90`:
   ```rust
   if index < 5 {
       let cell = read_u32(this + (index + 5) * 4 + 0x2DC);
       if cell_ok(cell) { index += 5; }
   }
   ```
3. `dir == 8` (up), `0x8062BE9C..0x8062BF04`:
   ```rust
   if index < 5 {
       index -= 1;
       if (index as i32) < 0 {
           if call2(0x8062BCF0, this, 0xFFFFFFFF) == 0 { index = 0; }
           else { index = 4; write_u32(this+0x4F8, read_u32(this+0x4F4)); }
       }
   } else {
       index -= 1;
       if index < 5 {
           if call2(0x8062BCF0, this, 0xFFFFFFFF) == 0 { index = 5; }
           else { index = 9; write_u32(this+0x4F8, read_u32(this+0x4F4)); }
       }
   }
   ```
4. `dir == 4` (down), `0x8062BF10..0x8062C088`:
   ```rust
   if index < 5 {
       index += 1;
       if index > 4 {
           if call2(0x8062BCF0, this, 1) == 0 { index = 4; }
           else { index = 0; write_u32(this+0x4F8, read_u32(this+0x4F4)); }
       }
   } else {
       index += 1;
       if index < 10 {
           let cell = read_u32(this + index * 4 + 0x2DC);
           if !cell_ok(cell) { index = 10; }
       }
       if index >= 10 {
           if call2(0x8062BCF0, this, 1) == 0 {
               if cell_ok(read_u32(this+0x300)) { return 9; }
               if cell_ok(read_u32(this+0x2FC)) { return 8; }
               if cell_ok(read_u32(this+0x2F8)) { return 7; }
               if cell_ok(read_u32(this+0x2F4)) { return 6; }
               if cell_ok(read_u32(this+0x2F0)) { return 5; }
           }
           index = 5;
           write_u32(this+0x4F8, read_u32(this+0x4F4));
       }
   }
   ```
5. Any other `dir` value - fall through to the return (`0x8062C08C`), leaving
   `index` untouched.
6. `0x8062C08C..0x8062C0A0`: `r3 = index`; `lwz r0,20(r1)`, `mtlr r0`,
   `lwz r31,12(r1)`, `addi r1,r1,16`, `blr`.

The five "return `9..5`" arms are literal early returns from inside the
`0x8062C080` block; they do **not** update `+0x4F8`, because the branch goes
straight to the epilogue (`0x8062C090`).

### Constants

No data literals. The four magic `dir` values are the Wii-remote D-pad bits
(`0x0001` left, `0x0002` right, `0x0004` down, `0x0008` up) delivered in `r5`.

### Callees

| site | callee | arguments |
|---|---|---|
| `0x8062BEB0`, `0x8062BEE4`, `0x8062BF28`, `0x8062BF98` | `0x8062BCF0` (page/slot stepper) | `r3 = this`, `r4 = +/-1` |

`0x8062BCF0` itself is 296 bytes and unclaimed; its contract, recovered word by
word, is:

```rust
// fn_8062bcf0(this, dir: i32) -> u32
//   cur = u32[this+0x500], count = u32[this+0x504]
//   if dir < 0:
//     if cur == 0: { if count < 2 return 0; u32[this+0x500]=10; u32[this+0x43C]=2; return 1 }
//     scan i = cur-1 down to 0 for u8[this+0x515+i] != 0; found -> i
//     if no byte found: i = 0            (the r5 < 0 arm at 0x8062BD58 is unreachable)
//     u32[this+0x43C] = 2; u32[this+0x500] = i; return 1
//   else:
//     if cur == 10: { if count < 2 return 0; u32[this+0x500]=0; u32[this+0x43C]=1; return 1 }
//     scan i = cur+1 .. 10 for u8[this+0x515+i] != 0
//     if none: { if count < 2 return 0; i = 0 }
//     u32[this+0x43C] = 1; u32[this+0x500] = i; return 1
```

The eleven availability bytes live at `+0x515..+0x51F`; they are copied from
save slot `+0xAE0` by the class's `fn_8062c5f0`
(`docs/COVERAGE_WAVE104_BASES.md:41`).

### Notes for the implementer

* `dir == 2` is the only arm with no availability test; the other three all
  consult `+0x248`/`+0x24C` bit 0 or `0x8062BCF0`.
* `+0x4F8` is written **before** `+0x4F4` would be updated by the caller, so the
  caller sees the previous cursor.
* The `cmpi r4,5` / `cmpi r4,10` tests are signed; the `cmpli r5,n` tests on
  `dir` are unsigned and use equality, so the caller's raw last-pad word only
  moves the cursor when exactly one of `1`/`2`/`4`/`8` is set (a diagonal
  sample hits no arm and returns the index unchanged).
* Wave 140 listed this row only as "not attempted"; nothing was ever found
  wrong with the body, so it is ready rather than conditional.

---

## 3. `0x8076B710` - `cobKinoko` model/clip binder

**Evidence.** `work/ghidra/rel-d_basesNP/export/8076b710_FUN_8076b710.c`, 644
bytes, 161 words, synthetic `0x8076B710`. Body `0x8076B710..0x8076B994` (ends
`blr` at `0x8076B990`). One `f31` spill pair (`W = 0`, `qr0`). `this` lives in
`r31` after `_savegpr_23`; `r27` = string base `0x808164B0`, `r28` = float base
`0x807CF0C8`, `r29` = the body model, `r26`/`r23`/`r24`/`r25` are the loop
state.

**What it is.** The one-shot model/animation set-up of the sibling actor whose
constructor is `fn_8076b3f0` (`0x8076B3F0`, vtable `0x80816538`, interface
vtable `0x807FEF40` at `+0x258`; see the `fn_8076b3f0` doc block in
`src/game/bases.rs`). It opens the
`+0x188` render block, loads two panes out of `g3d/model.brres` (the variant
pane named by `+0x28C` and the fixed `cobKinokoRed`), builds the body model and
its `+0x1A8` animation object, binds the two clips at `+0x1E8`/`+0x220` named
`kinoko` and `trunk`, builds the 8-part group at `+0x258`, attaches both clips
to the group, and releases the animation accessor. The two clip ids come from
the `u32[2]` reached through `+0x288`; the clip *names* are the fixed strings
below.

Its single caller `0x8076B560` (100 bytes) runs it, then copies the position
`+0xAC..+0xB4` into `+0x128..+0x134`, stores `1.0` at `+0x134`, and calls
`0x8076BA60`/`0x8076B9B0`.

### Signature

```rust
pub unsafe fn fn_8076b710(this: usize)
```

`r3` = the actor. No return value (Ghidra prints `void`; `r3` is left as the
`0x800690E0` result and is not read).

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x60` | `u32` | vtable; slot `+0x80` is invoked with no extra arguments |
| `+0x188` | struct | render/manager block (`0x800690C0` init, `0x800690E0` destroy) |
| `+0x1A4` | `u32` | brres handle for the variant pane named by `+0x28C` |
| `+0x280` | `u32` | brres handle for pane `cobKinokoRed` |
| `+0x1A8` | struct `0x60` | animation object on the body model |
| `+0x288` | `u32` | pointer to a `u32[2]` of animation pane ids |
| `+0x28C` | `u32` | variant pane/name pointer (subclass-provided string) |
| `+0x1E8` | `0x38[2]` | two clip entries |
| `+0x220` | `0x38` | second clip entry (same array) |
| `+0x480` | `u8` | per-entry byte, `+0x38` stride, set to `1` |
| `+0x258` | struct | eight-part group (`0x80165520` with count 8) |
| `+0x210` | `f32` | animation end-frame base; the clip end is `+0x210 - 1.0` |
| `+0x4` | `u32` | actor parameter word; low byte is the appearance/player byte |

`+0x1E8 + i*0x38` and `+0x220` are the two entries: the second entry *is*
`this+0x220`, i.e. the array is exactly two elements of `0x38`.

### Exact behaviour

Prologue `0x8076B710..0x8076B728`: `stwu r1,-96(r1)`, `mflr r0`,
`stw r0,100(r1)`, `r11 = r1+80`, `stfd f31,80(r1)`,
`psq_st f31,88(r1),0,qr0`, `bl 0x802DCD64` (`_savegpr_23`), `r31 = r3`.

1. `0x8076B72C..0x8076B74C` - `vc(this->vt[0x20], this)`
   (`lwz r12,96(r3); lwz r12,128(r12); mtspr ctr; bctrl`).
2. `0x8076B750..0x8076B768` - `call5(0x800690C0, this+0x188,
   0xFFFFFFFF, read_u32(0x80377C48), 0, 0x20)`. The third argument is the
   *value* at `0x80377C48`, not its address.
3. `0x8076B76C..0x8076B784`:
   ```rust
   let a = call3(0x800DF180, read_u32(0x8042A038) + 4,
                 read_u32(this+0x28C), 0x80816508 /* "g3d/model.brres" */);
   write_u32(this+0x1A4, a);
   ```
4. `0x8076B788..0x8076B79C`:
   ```rust
   let b = call3(0x800DF180, read_u32(0x8042A038) + 4,
                 0x80816518 /* "cobKinokoRed" */, 0x80816508);
   write_u32(this+0x280, b);
   ```
5. `0x8076B7A0..0x8076B7AC` - `body = call2(0x80239E30, this+0x1A4,
   read_u32(this+0x28C))` (the model instance for the variant pane).
6. `0x8076B7B0..0x8076B7CC` - `call6(0x80169D90, this+0x1A8, &body,
   this+0x188, 0x60, 1, 0)`.
7. `0x8076B7D0..0x8076B860` - the two-clip loop (`i = 0,1`):
   ```rust
   let mut entry = this + 0x1E8;
   let mut ids   = read_u32(this+0x288);
   for i in 0..2 {
       let clip = call2(0x8023A0B0, this+0x280, read_u32(ids + i*4));
       let mut model = body;
       call5(0x801650D0, entry, &model, &clip, this+0x188, 0);
       write_u8(this + 0x480 + i*0x38, 1);              // u32[0x807CF0D0+i*4] == 1
       call_f32_u32(0x801680E0, 0.0, entry);            // 0x8076B838: current frame = 0
       call_f32_u32(0x80168FE0, read_f32(this+0x210) - 1.0, entry);
                                                        // 0x8076B848: end frame
       entry += 0x38;
   }
   ```
   `0x8076B828` loads a **`u32`** (`lwz r6,0(r24)`) from the pool
   `0x807CF0D0`/`0x807CF0D4`, both `0x00000001`; the export's `*puVar6` is
   therefore the integer `1`, not `1.0f`.
8. `0x8076B864..0x8076B880` - `call5(0x80165520, this+0x258, &body, 8,
   this+0x188, 0)`; `call_f32_u32(0x80169F80, 0.0, this+0x1A8, this+0x258)`.
9. `0x8076B884..0x8076B8A8` - attach the two clips to the group:
   `call4(0x801656C0, f1=1.0, this+0x258, 0, this+0x1E8)` and
   `call4(0x801656C0, f1=1.0, this+0x258, 1, this+0x220)`.
10. `0x8076B8A8..0x8076B8C4` - `call_f32_u32(0x80169F80, 1.0, this+0x1A8,
    this+0x258)`; `call1(0x800DFBE0, this+0x1E8)`; `call1(0x800DFBE0,
    this+0x220)`.
11. `0x8076B8C8..0x8076B8EC` - bind the named clips:
    ```rust
    call4(0x800DF990, this+0x1A8, this+0x1E8, 0x80816528 /* "kinoko" */, 0);
    call4(0x800DF990, this+0x1A8, this+0x220, 0x80816530 /* "trunk"  */, 0);
    ```
12. `0x8076B8F0..0x8076B928` - the appearance gate:
    ```rust
    if call1(0x800FD8A0, this) != 0
        && call1(0x800FD8C0, this) == 0
        && call1(0x800FD8E0, this) == 0
    {
        call_f32_u32(0x80168FE0, 0.0, this+0x1E8);
    }
    ```
    All three tail-call `0x800FAE40`/`0x800FAE80`/`0x800FAF70` with the
    `r13-0x5433` mode byte plus the low byte of `this+4` (the `C0`/`E0` pair
    also passes the byte at bits 24-31).
13. `0x8076B92C..0x8076B960` - the save/player gate:
    ```rust
    if call0(0x800FD2E0) != 0                       // save-image byte +2, bit 4 (0x10)
        && read_u32(0x8083278C) != (read_u32(this+4) & 0xFF)
    {
        call_f32_u32(0x80168FE0, read_f32(this+0x210) - 1.0, this+0x1E8);
    }
    ```
14. `0x8076B964..0x8076B990` - `call1(0x800F2A20, this+0x1A8)` (tail
    `0x800B42B0(x, 0)`), `call1(0x800690E0, this+0x188)`, epilogue
    (`psq_l f31`, `lfd f31`, `bl 0x802DCDB0` `_restgpr_23`, `lwz r0,100(r1)`,
    `mtlr r0`, `addi r1,r1,96`, `blr`).

`0x8076B978` (the `psq_l` reload) is Ghidra's "unreachable block".

### Constants

| address | value | use |
|---|---:|---|
| `0x80816508` | `"g3d/model.brres"` | archive path for both handles |
| `0x80816518` | `"cobKinokoRed"` | fixed pane of the second handle |
| `0x80816528` | `"kinoko"` | clip name bound to `+0x1E8` |
| `0x80816530` | `"trunk"` | clip name bound to `+0x220` |
| `0x807CF0D0` / `0x807CF0D4` | `0x00000001` | fourth argument of the two `0x801651F0` calls |
| `0x807CF0D8` | `0.0f` | both `0x801680E0` frame resets |
| `0x807CF0DC` | `1.0f` | the `+0x210 - 1.0f` subtraction |
| `0x8083278C` | BSS `u32` | current player/appearance byte |

### Callees

| site | callee | arguments |
|---|---|---|
| `0x8076B728` | `0x802DCD64` `_savegpr_23` | spills `r23..r31` |
| `0x8076B74C` | vtable `this->vt[0x20]` | `r3 = this` |
| `0x8076B768` | `0x800690C0` | `this+0x188`, `-1`, `[0x80377C48]`, `0`, `0x20` |
| `0x8076B780`, `0x8076B798` | `0x800DF180` (brres loader) | `[0x8042A038]+4`, name, `"g3d/model.brres"` |
| `0x8076B7A8` | `0x80239E30` | `&this[+0x1A4]`, variant name |
| `0x8076B7CC` | `0x80169D90` | `this+0x1A8`, `&body`, `this+0x188`, `0x60`, `1`, `0` |
| `0x8076B7F0` | `0x8023A0B0` | `this+0x280`, `ids[i]` |
| `0x8076B814` | `0x801650D0` | entry, `&model`, `&clip`, `this+0x188`, `0` |
| `0x8076B82C` | `0x801651F0` | entry, `this+0x1A8`, `&clip`, `1` |
| `0x8076B838`, `0x8076B928` | `0x801680E0` | frame value, entry |
| `0x8076B848`, `0x8076B960` | `0x80168FE0` | end-frame value, entry |
| `0x8076B87C` | `0x80165520` | `this+0x258`, `&body`, `8`, `this+0x188`, `0` |
| `0x8076B890`, `0x8076B8A4` | `0x801656C0` | `f1 = 1.0`, `this+0x258`, slot `0/1`, entry |
| `0x8076B8B4` | `0x80169F80` | `f1 = 1.0`, `this+0x1A8`, `this+0x258` |
| `0x8076B8BC`, `0x8076B8C4` | `0x800DFBE0` | `this+0x1E8` / `this+0x220` |
| `0x8076B8D8`, `0x8076B8EC` | `0x800DF990` | `this+0x1A8`, entry, name, `0` |
| `0x8076B8F4`/`904`/`914` | `0x800FD8A0`/`C0`/`E0` | `r3 = this` |
| `0x8076B92C` | `0x800FD2E0` | no arguments |
| `0x8076B968` | `0x800F2A20` | `this+0x1A8` |
| `0x8076B970` | `0x800690E0` | `this+0x188` |
| `0x8076B980` | `0x802DCDB0` `_restgpr_23` | restores `r23..r31` |

### Notes for the implementer

* `0x80168400` is **not** called here: this row only loads panes and clips.
  The two `0x8023B200` node lookups and the `+0x14` bit-8 clear belong to row
  5 (`0x8078A8F0`). If an implementation tries to copy the row-5 shape into
  this body it will add two calls that the bytes do not contain.
* `+0x28C` is the only per-variant input: it is both the archive lookup name
  (step 3) and the pane name handed to `0x80239E30` (step 5). The second
  handle (`+0x280`) is always `cobKinokoRed`.
* The per-clip `0x80168FE0` calls at step 7 use the *same* `this+0x210`
  value, so both clips share one end frame. Step 13 can rewrite the first
  clip's end frame only.
* The store of `1` into `+0x480`/`+0x4B8` is a byte (`stb`), not a word.
* The `0x80377C48` read is `lwz`, so pass the word value, exactly as the
  earlier `0x800690C0` call sites in wave 151 do.
* `0x8076B9A0` (the bare `blr` in the 28-byte hole) is **not** part of this
  body.

---

## 4. `0x8074CBA0` - four-slot join-panel tick

**Evidence.** `work/ghidra/rel-d_basesNP/export/8074cba0_FUN_8074cba0.c`, 636
bytes, 159 words, synthetic `0x8074CBA0`. Body `0x8074CBA0..0x8074CE1C` (ends
`blr` at `0x8074CE18`). One `f31` spill pair (`W = 0`, `qr0`). `this` lives in
`r29` after `_savegpr_26`; `r31` = pool base `0x807CE620`, `r30` = the
per-slot `+0x3F8` cursor, `r28` = the "any player lagged" flag, `r27` = the
slot `i`.

**What it is.** The per-frame state machine of the four-slot join panel whose
draw/commit body is `fn_8074c540` (`0x8074C540`, already translated; the panel
is described in `docs/COVERAGE_WAVE144_BASES.md` and the `fn_8074c540` doc
block in `src/game/bases.rs`).
Each frame it ages a state timer, tracks each player's non-forward travel into
`+0x408 + kind*4`, re-arms a randomised 320/576-frame delay, and then picks the
next player to feature (a player whose rectangle test `0x8074C280` succeeds),
advances the panel state `1/2/3`, clears the two slot masks, randomises
`+0x42E` from the 4-entry table at `0x807CE658`, and redraws through
`0x8074C540`. The virtual-slot table at `0x808119C8` holds this address
alongside `0x8074CA70`, `0x8074CA40` and `0x8074CB90`.

### Signature

```rust
pub unsafe fn fn_8074cba0(this: usize)
```

No return value. Two call sites of `0x8074C540` pass a third argument `0` in
`r5`; `0x8074C540` never reads `r5` (first use of `r5` is
`lhz r5,1064(r26)` at `0x8074C65C`), so the extra register is harmless and the
2-argument Rust signature stays valid.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x3F8 + i*4` | `f32[4]` | last x of player slot `i` |
| `+0x408 + k*4` | `f32[4]` | accumulated travel for kind `k` (see note) |
| `+0x41C` | `s32` | last featured slot |
| `+0x420` | `u16` | panel state (`0` idle, `1` waiting, `2`/`3` active) |
| `+0x426` | `u16` | slot mask A (cleared every re-arm) |
| `+0x428` | `u16` | slot mask B (cleared every re-arm) |
| `+0x42A` | `u16` | countdown to the next selection |
| `+0x42C` | `u16` | layout kind; `(value & 7) == 5` selects the alternate arm |
| `+0x42E` | `u16` | per-state delay (randomised from `0x807CE658`) |
| `+0x431` | `u8` | mode byte; `== 6` selects the third state |
| `+0x433` | `u8` | rectangle index consumed by `0x8074C280` |

### Exact behaviour

Prologue `0x8074CBA0..0x8074CBB8`: `stwu r1,-48(r1)`, `mflr r0`,
`stw r0,52(r1)`, `r11 = r1+32`, `stfd f31,32(r1)`,
`psq_st f31,40(r1),0,qr0`, `bl 0x802DCD70` (`_savegpr_26`), `r29 = r3`.

1. `0x8074CBBC..0x8074CC14` - the state gate:
   ```rust
   if read_u16(this+0x420) != 0 {
       let mut aged = 0u32;
       if read_u16(this+0x420) == 1 && read_u16(this+0x42E) != 0 {
           write_u16(this+0x42E, read_u16(this+0x42E) - 1);
           aged = 1;
       }
       if aged == 0 {
           call3(0x8074C540, this, call1(0x8005F900, 0), 0);
       }
   }
   ```
2. `0x8074CC18..0x8074CC74` - the four-slot travel loop (`f31 = 0.0f` from
   `0x807CE624`):
   ```rust
   let mut any = 0u32;
   for i in 0..4 {
       let player = call1(0x8005F900, i);
       if player != 0 {
           let d = read_f32(player+0xAC) - read_f32(this + 0x3F8 + i*4);
           if d > 0.0 {
               write_f32(this + 0x3F8 + i*4, read_f32(player+0xAC));
           } else {
               call2(0x8074C990, this, player);
               any = 1;
           }
       }
   }
   ```
   The compare is `fcmpo cr0,f0,f31` followed by `cror 2,1,2` and `bne`: the
   `cror` turns `cr0.eq` into `lt | eq`, so `bne` is taken exactly when
   `d > 0.0`. `0x8074C990` (164 bytes, unclaimed) calls the same `+0x6C`
   kind getter and does
   `this[+0x408 + kind*4] += fabs(this[+0x3F8 + kind*4] - player[+0xAC])`
   **when `0x8074C280(this, player)` is true**, then stores
   `this[+0x3F8 + kind*4] = player[+0xAC]`.
3. `0x8074CC78..0x8074CCAC` - timer bookkeeping (note the `+0x431` load at
   `0x8074CC8C`/`0x8074CC94` is emitted twice; both compare against `6` and
   neither result is used, so the pair is dead):
   ```rust
   if any == 0 { write_u16(this+0x42A, read_u16(this+0x42A) + 1); }
   // dead: read_u8(this+0x431); cmpli 6; (repeated)
   if read_u16(this+0x42A) != 0 { write_u16(this+0x42A, read_u16(this+0x42A) - 1); }
   ```
   Net effect: `+0x42A` is unchanged when `any == 0` and decremented by one
   otherwise.
4. `0x8074CCB0..0x8074CCEC` - re-arm when the countdown reaches zero:
   ```rust
   if read_u16(this+0x42A) == 0 {
       if read_u8(this+0x431) != 6 {
           write_u16(this+0x42A, (call1(0x800B2EE0, 64) as u16) + 320);
       } else {
           write_u16(this+0x42A, (call1(0x800B2EE0, 32) as u16) + 576);
       }
       for k in 0..4 { write_f32(this + 0x408 + k*4, 0.0); }
       let mut found = 0u32;
       let mut candidate = 0usize;
       let mut j = 0usize;
       loop {
           let player = call1(0x8005F900, j);
           if player != 0 && call2(0x8074C280, this, player) != 0 {
               candidate = player;
               found = 1;
               if read_u32(this+0x41C) == j as u32 { /* keep scanning */ }
               else { write_u32(this+0x41C, j as u32); break; }
           }
           j += 1;
           if j == 4 { break; }
       }
       if found == 0 { return; }
       let featured = candidate;   // r26; the r30/r26 select at 0x8074CD64 is a no-op when j == 4
       if read_u8(this+0x431) != 6 {
           write_u16(this+0x420,
               if (read_u16(this+0x42C) & 7) == 5 { 2 } else { 1 });
       } else {
           write_u16(this+0x420,
               if (read_u16(this+0x42C) & 7) == 5 { 2 } else { 3 });
       }
       write_u16(this+0x426, 0);
       write_u16(this+0x428, 0);
       write_u16(this+0x42E, read_u16(0x807CE658 + 2 * call1(0x800B2EE0, 4)));
       call3(0x8074C540, this, featured, 0);
   }
   ```
   The `r26`/`r30` candidate dance at `0x8074CD34..0x8074CD6C` is
   `featured = (j == 4) ? r26 : r30`; both registers hold the same player, so
   the branch is a no-op and the spec can keep one `candidate`.
5. Epilogue `0x8074CE00..0x8074CE18`: `psq_l f31`, `lfd f31`,
   `bl 0x802DCDBC` (`_restgpr_26`), `lwz r0,52(r1)`, `mtlr r0`,
   `addi r1,r1,48`, `blr`.

`0x8074CDFC` is Ghidra's "removing unreachable block" target - the epilogue
reload, not a second exit.

### Constants

| address | value | use |
|---|---:|---|
| `0x807CE624` | `0.0f` | `f31`, the travel compare |
| `0x807CE658` | `u16[4] = {56, 24, 40, 8}` | randomised `+0x42E` |
| `64`, `32` | `rand` bounds | `+0x42A = rand(64)+320` / `rand(32)+576` |
| `320`, `576` | frame counts | the two re-arm bases |

### Callees

| site | callee | arguments |
|---|---|---|
| `0x8074CBB8` | `0x802DCD70` `_savegpr_26` | spills `r26..r31` |
| `0x8074CC04`, `0x8074CC2C`, `0x8074CD10` | `0x8005F900` (player slot -> object) | `r3 = slot 0..3` |
| `0x8074CC14`, `0x8074CDF8` | `0x8074C540` (join-panel draw/commit) | `this`, `player`, `0` |
| `0x8074CC58` | `0x8074C990` (travel accumulator) | `this`, `player` |
| `0x8074CCCC`, `0x8074CCE0`, `0x8074CDD8` | `0x800B2EE0` (`rand(n)`) | `64` / `32` / `4` |
| `0x8074CD28` | `0x8074C280` (player-in-rect) | `this`, `player` |
| `0x8074CE08` | `0x802DCDBC` `_restgpr_26` | restores `r26..r31` |

### Notes for the implementer

* The `+0x3F8` array is indexed by **slot** in this body (`+0x3F8 + i*4`) but by
  **kind byte** inside `0x8074C990` (`+0x3F8 + kind*4`). The two agree only
  when the player's kind equals its slot; transcribe both indices literally.
* `0x8074C280` reads `+0x433` as the rectangle selector, so `+0x433` is part of
  the panel's identity (set by the constructor, not here).
* The `+0x426`/`+0x428` masks are cleared *before* the redraw, which is why
  `0x8074C540` walks them from a zero state.
* `rand` is the `0x800B2ED0`/`0x800B2EE0` thunk (`src/game/stage.rs`), a 12-byte
  tail call that returns `0..n-1`.
* The `return` inside step 4 leaves the caller with no redraw on a
  no-player frame; the epilogue still runs.

---

## 5. `0x8078A8F0` - Kameck ride/flight model binder

**Evidence.** `work/ghidra/rel-d_basesNP/export/8078a8f0_FUN_8078a8f0.c`, 632
bytes, 158 words, synthetic `0x8078A8F0`. Body `0x8078A8F0..0x8078AB68` (ends
`blr` at `0x8078AB64`). No float registers, no `psq`. `this` lives in `r31`
after `_savegpr_22`; `r30` = string base `0x80819588`, `r28` = pool
`0x807CFB48`, `r29` = the `kameck_broom` model, `r26`/`r25`/`r24`/`r23`/`r27`
are the loop state.

**What it is.** The model/clip binder of the world-map Kameck actor
(`g3d/kameck.brres`, panes `kameck` and `kameck_broom`, clips
`ride_wait_broom`, `fly_broom`, `fly_wmap_A1/B1/B2`, `fly_wmap_broom_A1/B1/B2`,
plus the `wand_model_root` / `wand_model` nodes). It opens `+0x60C` on
`g3d/kameck.brres`, builds the broom model and its `+0x610` animation object,
binds two ride clips at `+0x44C`, hides the two wand nodes, then binds three
`fly_wmap_*` clips at `+0x4BC` and three broom clips at `+0x564`, and releases
the animation accessor. The address is a vtable slot at `0x808197CC` (the table
also holds `0x8078AF50`, `0x8078AF20`, `0x8078ABE0`, `0x8078AD80`).

### Signature

```rust
pub unsafe fn fn_8078a8f0(this: usize)
```

`r3` = the actor. No return value.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x184` | struct | render block used by every `0x801650D0` |
| `+0x1A0` | `u32` | pre-existing `kameck` model handle (read, not written here) |
| `+0x1A4` | `u32` | entry base passed to `0x80168400`; its `+4` is `+0x1A8` |
| `+0x60C` | `u32` | `g3d/kameck.brres` archive handle (written here) |
| `+0x610` | struct `0x20` | animation object (written here) |
| `+0x44C + i*0x38` | `0x38[2]` | ride clips (`ride_wait_broom`, `fly_broom`) |
| `+0x480 + i*0x38` | `u8` | per-entry byte, set to `0` |
| `+0x4BC + i*0x38` | `0x38[3]` | `fly_wmap_A1/B1/B2` clips |
| `+0x4F0 + i*0x38` | `u8` | per-entry byte, set to `1` |
| `+0x564 + i*0x38` | `0x38[3]` | `fly_wmap_broom_A1/B1/B2` clips |
| `+0x598 + i*0x38` | `u8` | per-entry byte, set to `1` |
| `+0x14` of a node | `u32` | node flag word; bit `0x100` cleared |

The per-entry byte is always at `entry + 0x34` (`0x44C+0x34 = 0x480`,
`0x4BC+0x34 = 0x4F0`, `0x564+0x34 = 0x598`).

### Exact behaviour

Prologue `0x8078A8F0..0x8078A904`: `stwu r1,-80(r1)`, `mflr r0`,
`stw r0,84(r1)`, `r11 = r1+80`, `bl 0x802DCD60` (`_savegpr_22`), `r31 = r3`.

1. `0x8078A904..0x8078A930` - `call3(0x800DF180, read_u32(0x8042A038)+4,
   0x80819704 /* "kameck" */, 0x808196F0 /* "g3d/kameck.brres" */)`; store the
   result at `+0x60C`.
2. `0x8078A934..0x8078A940` - `broom = call2(0x80239E30, this+0x60C,
   0x8081970C /* "kameck_broom" */)`; the result is also spilled at `r1+36`
   (and `r1+8`) for the later entries.
3. `0x8078A944..0x8078A960` - `call6(0x80169D90, this+0x610, &broom,
   this+0x184, 0x20, 1, 0)`.
4. `0x8078A964..0x8078A9E8` - the two ride clips (`i = 0,1`):
   ```rust
   let mut entry = this + 0x44C;
   for i in 0..2 {
       let anim = call2(0x8023A0B0, this+0x60C, read_u32(0x80819658 + i*4));
       if anim != 0 {
           let mut model = broom;
           call5(0x801650D0, entry, &model, &anim, this+0x184, 0);
           write_u8(this + 0x480 + i*0x38, read_u32(0x807CFB68 + i*4) as u8); // 0
           call_f32_u32(0x80168FE0, 0.0, entry);      // [0x807CFB60]
           call_f32_u32(0x801680E0, 1.0, entry);      // [0x807CFB70]
       }
       entry += 0x38;
   }
   ```
5. `0x8078A9EC..0x8078A9F8` - `call_f32_u32(0x80169F80, 0.0, this+0x610,
   this+0x44C)`.
6. `0x8078A9FC..0x8078AA24` - hide the wand root:
   ```rust
   let node = call1(0x80168400, this+0x1A4);   // model of the +0x1A4 entry
   let hit  = call2(0x8023B200, &node, 0x80819720 /* "wand_model_root" */);
   if hit != 0 { write_u32(hit+0x14, read_u32(hit+0x14) & !0x100); }
   ```
7. `0x8078AA28..0x8078AA48` - the same for `0x80819730 /* "wand_model" */`.
8. `0x8078AA48..0x8078AA64` - build the two map models:
   ```rust
   let kameck = call2(0x80239E30, this+0x1A0, 0x80819704 /* "kameck" */);
   let broom2 = call2(0x80239E30, this+0x60C, 0x8081970C /* "kameck_broom" */);
   ```
9. `0x8078AA68..0x8078AB44` - the three map clips and three broom clips:
   ```rust
   for i in 0..3 {
       let a = call2(0x8023A0B0, this+0x1A0, read_u32(0x80819690 + i*4));
       if a != 0 {
           let mut model = kameck;
           call5(0x801650D0, this+0x4BC + i*0x38, &model, &a, this+0x184, 0);
           write_u8(this + 0x4F0 + i*0x38, 1);
           call_f32_u32(0x80168FE0, 0.0, this+0x4BC + i*0x38);
           call_f32_u32(0x801680E0, 0.0, this+0x4BC + i*0x38);
       }
       let b = call2(0x8023A0B0, this+0x60C, read_u32(0x808196D8 + i*4));
       if b != 0 {
           let mut model = broom2;
           call5(0x801650D0, this+0x564 + i*0x38, &model, &b, this+0x184, 0);
           write_u8(this + 0x598 + i*0x38, 1);
           call_f32_u32(0x80168FE0, 0.0, this+0x564 + i*0x38);
           call_f32_u32(0x801680E0, 0.0, this+0x564 + i*0x38);
       }
   }
   ```
10. `0x8078AB48..0x8078AB64` - `call1(0x800F2A40, this+0x610)` (tail
    `0x800B4050(x, 0)`); epilogue (`bl 0x802DCDAC` `_restgpr_22`,
    `lwz r0,84(r1)`, `mtlr r0`, `addi r1,r1,80`, `blr`).

### Constants

| address | value | use |
|---|---:|---|
| `0x808196F0` | `"g3d/kameck.brres"` | archive path |
| `0x80819704` | `"kameck"` | archive pane / model pane |
| `0x8081970C` | `"kameck_broom"` | broom pane |
| `0x80819638`, `0x80819648` | `"ride_wait_broom"`, `"fly_broom"` | ride clip names |
| `0x8081966C`, `0x80819678`, `0x80819684` | `"fly_wmap_A1"`, `"fly_wmap_B1"`, `"fly_wmap_B2"` | map clip names |
| `0x8081969C`, `0x808196B0`, `0x808196C4` | `"fly_wmap_broom_A1"`, `"..._B1"`, `"..._B2"` | broom clip names |
| `0x80819720` / `0x80819730` | `"wand_model_root"` / `"wand_model"` | node names |
| `0x807CFB60` | `0.0f` | ride-entry end frame, map-entry frames |
| `0x807CFB68` / `0x807CFB6C` | `0x00000000` | ride-entry byte value (`0`) |
| `0x807CFB70` | `1.0f` | ride-entry current frame |

### Callees

| site | callee | arguments |
|---|---|---|
| `0x8078A900` | `0x802DCD60` `_savegpr_22` | spills `r22..r31` |
| `0x8078A92C` | `0x800DF180` (brres loader) | `[0x8042A038]+4`, `"kameck"`, `"g3d/kameck.brres"` |
| `0x8078A93C`, `0x8078AA50`, `0x8078AA60` | `0x80239E30` | handle address, pane name |
| `0x8078A960` | `0x80169D90` | `this+0x610`, `&broom`, `this+0x184`, `0x20`, `1`, `0` |
| `0x8078A980`, `0x8078AA8C`, `0x8078AADC` | `0x8023A0B0` | handle address, pane id |
| `0x8078A9AC`, `0x8078AAB4`, `0x8078AB08` | `0x801650D0` | entry, `&model`, `&anim`, `this+0x184`, `0` |
| `0x8078A9C0`, `0x8078AAC4`, `0x8078AB18` | `0x80168FE0` | end frame, entry |
| `0x8078A9CC`, `0x8078AAD0`, `0x8078AB24` | `0x801680E0` | current frame, entry |
| `0x8078A9F8` | `0x80169F80` | `f1 = 0.0`, `this+0x610`, `this+0x44C` |
| `0x8078AA00` | `0x80168400` | `this+0x1A4` |
| `0x8078AA10`, `0x8078AA30` | `0x8023B200` (`mdl_get_node`) | `&node`, name |
| `0x8078AB4C` | `0x800F2A40` | `this+0x610` |
| `0x8078AB54` | `0x802DCDAC` `_restgpr_22` | restores `r22..r31` |

### Notes for the implementer

* `0x80168400(this+0x1A4)` is a type-checked accessor: it reads
  `u32[this+0x1A8]` (the animation object created at step 3), checks it against
  the class at `0x80308FD8` through vtable `+8`, and returns `u32[obj+0xE8]`.
  The two `0x8023B200` calls then search that object's node dictionary. Nothing
  here is a model-pointer dereference of `+0x1A4` itself.
* Both `0x8023B200` results are guarded; the guard is data-dependent (the two
  node names must exist in the object returned by `0x80168400`), and a missing
  name leaves the `+0x14` word untouched.
* The `0x807CFB68` pool word is a `u32` store (`lwz` + `stb`), not a float.
* `+0x1A0` is read before anything in this body writes it; it is set by the
  caller (the Kameck constructor chain), so do not add an initialisation here.
* The 1,000-byte gap after this body contains the next TSV row at `0x8078AF50`
  and unanalysed code; the body itself ends at `0x8078AB64`.

---

## 6. `0x8061B890` - `dGameOver_c` save-image round trip

**Evidence.** `work/ghidra/rel-d_basesNP/export/8061b890_FUN_8061b890.c`, 628
bytes, 157 words, synthetic `0x8061B890`. Body `0x8061B890..0x8061BB04` (ends
`blr` at `0x8061BB00`). No float registers, no `psq`. `this` is not used at
all: the body is a global-state round trip. The address is the third word of a
`{0, -1, fn}` descriptor at `0x807D7CA4..0x807D7CAC`; the string block
immediately after it (`0x807D7CC8`) reads
`dGameOver_c::StateID_FadeInEndWait`, `..._OnStageAnimeEndWait`,
`..._DispEndWait`, `..._ExitAnimeEndWait`, which identifies the owning class as
`dGameOver_c`.

**What it is.** A transactional snapshot/restore of the save-image record
around a save-manager re-initialisation. It reads the 4 palette bytes and the
420-byte 10 x 42 body of the current save image into the stack, saves the four
`0x80354E60` slot indices and the four `0x80354E90` pointers they select,
re-initialises the manager, calls `0x800BB3C0` for `i = 0..6` (the ledger's
`fn_800bb3c0` is the stage-family release helper), re-installs the four
pointers, writes the image contents back, and flushes with `0x800FBD30(1)`.
Wave 131 deferred it as "a wall of unverified `.bss` offsets"
(`docs/COVERAGE_WAVE131_BASES.md:127-130`); the offsets are now pinned (below),
so the row is ready.

### Signature

```rust
pub unsafe fn fn_8061b890()
```

No arguments (Ghidra's `void`). Returns nothing; `r3` is left as the
`0x800FBD30` result and is not read.

### Field map (all global / record, none on `this`)

| address / offset | type | meaning |
|---|---|---|
| `0x8042A040` | `u32` | save manager pointer (`[0x8042A040]` is the manager) |
| manager `+0x26` | `s8` | current save-image slot |
| `0x800E0380(manager, -1)` | call | returns `image + slot*0x980 + 0x6C0` |
| record `+0x1A + i` | `u8[4]` | palette/ink byte `i` (`0x800CE320`/`0x800CE310`) |
| record `+0x7C4 + row*42 + col` | `u8` | 10 x 42 body byte (`0x800CE2C0`/`0x800CE260`) |
| record `+0x2` | `u8` | flag byte; bit `0x40` is set by `0x800CE260` for high ink indices |
| `0x80354E60/64/68/6C` | `u32[4]` | player -> object id, per slot |
| `0x80354E70` | `u32[]` | per-player current index (cleared at the four ids) |
| `0x80354E80` | `u32[]` | per-player claimed flag (cleared at the four ids) |
| `0x80354E90` | `u32[]` | per-player object pointer (saved and restored) |
| `0x80354EA0` | `u32[]` | fourth interleaved array (cleared at the four ids) |
| `0x80429F7C` | `u32` | player table base (argument of `0x800BB3C0`) |
| `0x80429CC0` | `u32` | a pointer saved and restored unchanged |

### Exact behaviour

Prologue `0x8061B890..0x8061B8A0`: `stwu r1,-496(r1)`, `mflr r0`,
`stw r0,500(r1)`, `r11 = r1+496`, `bl 0x802DCD4C` (`_savegpr_17`).

1. `0x8061B8A4..0x8061B8C0` - set the bases:
   ```rust
   let manager = read_u32(0x8042A040);          // r23
   let players = read_u32(0x80429F7C);          // r19
   let image   = call2(0x800E0380, manager, 0xFFFFFFFF);
   let slot    = (read_u8(manager + 0x26) as i8) as i32;   // r17, extsb
   ```
   The `r1+8` stack area is the 4-byte palette buffer `local_1e8`, and
   `r1+12` (`local_1e4`) is the 420-byte body buffer.
2. `0x8061B8C8..0x8061B8F0` - read the four palette bytes:
   ```rust
   for i in 0..4 {
       write_u8(r1 + 8 + i, call2(0x800CE320, image, i) as u8);
   }
   ```
   `0x800CE320(obj, i)` returns `*(s8*)(obj + i + 0x1A)`.
3. `0x8061B8F4..0x8061B940` - read the 10 x 42 body:
   ```rust
   for row in 0..10 {
       for col in 0..42 {
           write_u8(r1 + 12 + row*42 + col,
                    call4(0x800CE2C0, image, row, col, 0) as u8);
       }
   }
   ```
   `0x800CE2C0(obj, row, col, flag)` returns `*(u8*)(obj + row*42 + col + 0x7C4)`,
   except `(row == 2 && col == 3 && flag != 0)`, which returns the dedicated
   byte at `obj+0x968` (`0x800CE2F0` stores `stb r4,2408(r3)`).
4. `0x8061B944..0x8061B954` - `let special = call4(0x800CE2C0, image, 2, 3, 1);`
   (the `(2,3)` cell, read through the alternate accessor).
5. `0x8061B958..0x8061B9A8` - snapshot the slot state:
   ```rust
   let ids = [read_u32(0x80354E60), read_u32(0x80354E64),
              read_u32(0x80354E68), read_u32(0x80354E6C)];
   let ptrs = [read_u32(0x80354E90 + ids[0]*4), read_u32(0x80354E90 + ids[1]*4),
               read_u32(0x80354E90 + ids[2]*4), read_u32(0x80354E90 + ids[3]*4)];
   let saved_cc0 = read_u32(0x80429CC0);
   call0(0x800B2FB0);                       // stage release / clear (reads no argument;
                                            //   the leftover r3 = special is dead)
   call2(0x800E1840, manager, slot);        // save_manager_init(manager, slot)
   for i in 0..7 { call2(0x800BB3C0, players, i); }
   ```
6. `0x8061B9C8..0x8061BA58` - restore `0x80354E90` and clear the other three
   arrays at the four ids:
   ```rust
   for k in 0..4 {
       let off = ids[k] * 4;
       write_u32(0x80354E90 + off, ptrs[k]);
       write_u32(0x80354EA0 + off, 0);
       write_u32(0x80354E70 + off, 0);
       write_u32(0x80354E80 + off, 0);
   }
   write_u32(0x80429CC0, saved_cc0);        // 0x8061BA5C
   ```
   The four id words themselves (`0x80354E60..6C`) are *not* rewritten; they
   still hold the same values.
7. `0x8061BA60..0x8061BA80` - write the palette back:
   ```rust
   for i in 0..4 { call3(0x800CE310, image, i, read_u8(r1 + 8 + i) as i32); }
   ```
   `0x800CE310(obj, i, v)` stores `v` at `obj + i + 0x1A`.
8. `0x8061BA84..0x8061BAC8` - write the body back:
   ```rust
   for row in 0..10 {
       for col in 0..42 {
           call5(0x800CE260, image, row, col, 0, read_u8(r1 + 12 + row*42 + col));
       }
   }
   ```
9. `0x8061BACC..0x8061BAE0` - `call5(0x800CE260, image, 2, 3, 1, special);`
10. `0x8061BAE4..0x8061BB00` - `call1(0x800FBD30, 1)`; epilogue
    (`bl 0x802DCD98` `_restgpr_17`, `lwz r0,500(r1)`, `mtlr r0`,
    `addi r1,r1,496`, `blr`).

### Constants

| address | value | use |
|---|---:|---|
| `0x26` | `0x26` | manager slot byte offset |
| `0x1A` | `0x1A` | palette base in the image record |
| `0x7C4` | `0x7C4` | body base in the image record |
| `0x2A` = 42 | 42 | body row stride |
| `7` | 7 | number of `0x800BB3C0` releases |
| `1` | 1 | `0x800FBD30` mode |

### Callees

| site | callee | arguments |
|---|---|---|
| `0x8061B8A0` | `0x802DCD4C` `_savegpr_17` | spills `r17..r31` |
| `0x8061B8BC` | `0x800E0380` `save_image_record_a` | `manager`, `-1` |
| `0x8061B8DC` | `0x800CE320` | `image`, `i` |
| `0x8061B91C`, `0x8061B954` | `0x800CE2C0` | `image`, `row`, `col`, `flag` |
| `0x8061B99C` | `0x800B2FB0` | no arguments |
| `0x8061B9A8` | `0x800E1840` `save_manager_init` | `manager`, `slot` |
| `0x8061B9B8` | `0x800BB3C0` | `players`, `i` |
| `0x8061BA70` | `0x800CE310` | `image`, `i`, `value` |
| `0x8061BAA8`, `0x8061BAE0` | `0x800CE260` | `image`, `row`, `col`, `flag`, `value` |
| `0x8061BAE8` | `0x800FBD30` | `1` |
| `0x8061BAF0` | `0x802DCD98` `_restgpr_17` | restores `r17..r31` |

`0x800CE2F0` (the `(2,3)` alternate accessor used internally by `0x800CE2C0`)
is a main.dol helper; it is not called directly from this body.

### Notes for the implementer

* The stack buffers are 4 + 420 bytes beginning at `r1+8`; the frame is 496
  bytes, so the rest is unused.
* `0x800CE260` has a side effect on the record: for `flag == 0`,
  `row < 8`, `col <= 23 || col == 37` and `value >= 8`, it sets bit `0x40` of
  `record+2` before storing the byte.
* `0x800CE320` sign-extends (`lbz; extsb`), so a palette byte above `0x7F`
  becomes negative; the body only stores it back as a byte, so the round trip is
  lossless for all 256 values.
* The four ids are used as *indices* into four interleaved arrays; their values
  come from `0x80354E60..6C` (the save manager's player -> object ids) and are
  expected in `0..3`. Transcribe the indexed accesses literally.
* Wave 131's rejection (record element roles unrecoverable) is retired: the
  getters/setters are the image record's 4-byte palette at `+0x1A` and 10 x 42
  body at `+0x7C4`, as read out of `main.dol` at `0x800CE260..0x800CE32C`.

---

## 7. `0x80752730` - `obj_waterfloat` per-frame transform

**Evidence.** `work/ghidra/rel-d_basesNP/export/80752730_FUN_80752730.c`, 628
bytes, 157 words, synthetic `0x80752730`. Body `0x80752730..0x807529A4` (ends
`blr` at `0x807529A0`). Two `stfd`+`psq_st` spill pairs (`f31` 256/264,
`f30` 240/248) and two `psq_l` reloads, all `W = 0`, `qr0`. `this` lives in
`r29`; `r30` = pool `0x807CE7C8`, `r31` = `r30+20`.

**What it is.** The per-frame model transform of the `obj_waterfloat` actor
(0x4C0-byte object; constructor `fn_80752100`, loader `fn_807525e0`; see
the `fn_80752100` doc block in
`src/game/bases.rs`). It builds the `+0x7C` matrix from the position
`+0xAC..+0xB4` and the three `s16` angles `+0x100..+0x104`, applies the
actor scale `+0xDC..+0xE4`, advances the blend factor `+0x4A0` toward 0 or 1
depending on `+0x4A8`, then composes two "lean/scale" pairs around the two
spinning angles `+0x488`/`+0x48C`, and hands the matrix to the model holder
`+0x3F4`. The single direct caller is `0x807524E8` (inside
`0x807524xx`); the address has no data-pointer reference.

### Signature

```rust
pub unsafe fn fn_80752730(this: usize)
```

`r3` = the `obj_waterfloat` actor. No return value.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x7C` | `f32[12]` | 3 x 4 model matrix (rebuilt every call) |
| `+0xAC`/`+0xB0`/`+0xB4` | `f32` | world position |
| `+0x100`/`+0x102`/`+0x104` | `s16` | three Euler angles fed to `0x80064AF0` |
| `+0xDC`/`+0xE0`/`+0xE4` | `f32` | actor scale vector |
| `+0x3F4` | struct | model holder (`0x8016A170`/`0x8016A1A0`) |
| `+0x488` | `f32` | first spin angle (fractional, truncated to `s16`) |
| `+0x48C` | `f32` | second spin angle |
| `+0x4A0` | `f32` | blend factor stepped by `0x8015F140` |
| `+0x4A8` | `s32` | blend selector: `0` = slow/0, non-zero = fast/1 |

### Exact behaviour

Prologue `0x80752730..0x8075275C`: `stwu r1,-272(r1)`, `mflr r0`,
`stw r0,276(r1)`, `li r5,1`, `addi r4,r1,24`, `stfd f31,256(r1)`,
`psq_st f31,264(r1),0,qr0`, `stfd f30,240(r1)`, `psq_st f30,248(r1),0,qr0`,
`stw r31,236(r1)`, `stw r30,232(r1)`, `stw r29,228(r1)`, `r29 = r3`,
`r30 = 0x807CE7C8`.

1. `0x8075276C..0x807527A4` - copy position and angles, then adjust:
   ```rust
   let mut pos = [read_f32(this+0xAC), read_f32(this+0xB0), read_f32(this+0xB4)];
   let ang = [read_s16(this+0x100), read_s16(this+0x102), read_s16(this+0x104)];
   call3(0x80064AF0, &pos, &ang, 1);   // in-place vector adjust (see note)
   ```
   The three `s16` are loaded with `lha` (sign-extended) and stored with `sth`
   into `r1+24..28`; the position is stored at `r1+32..40`.
2. `0x807527A4..0x807527B4` - `call4(0x801C0BD0, &this[+0x7C], f1 = pos[0],
   f2 = pos[1], f3 = pos[2])` (translation matrix).
3. `0x807527B8..0x807527D8` - `call3(0x801C0C60, &tmp, f1 = +0xDC, f2 = +0xE0,
   f3 = +0xE4)` then `call3(0x801C0540, &this[+0x7C], &tmp, &this[+0x7C])`
   (scale then concat).
4. `0x807527DC..0x80752818` - step the blend factor:
   ```rust
   if read_s32(this+0x4A8) == 0 {
       call5(0x8015F140, &this[+0x4A0], f1 = 0.0, f2 = 0.025, f3 = 0.01, f4 = 1e-8);
   } else {
       call5(0x8015F140, &this[+0x4A0], f1 = 1.0, f2 = 0.5, f3 = 1.0, f4 = 0.1);
   }
   ```
   `0x8015F140(value_ptr, target, rate, limit, band)` steps `*value` toward
   `target` with a per-call clamp (main.dol `0x8015F140..0x8015F1FC`).
5. `0x8075281C..0x80752844` - first lean:
   ```rust
   let s = (read_f32(this+0x488) as i32) as i16;   // fctiwz -> sth
   call2(0x8016EEF0, &this[+0x7C], &s);            // post-concat rotation
   ```
   Only the first `s16` of the stack slot is read by `0x8016EEF0`/`0x8016EE30`;
   the four call sites use `r1+20`, `r1+16`, `r1+12`, `r1+8`.
6. `0x80752848..0x80752874` - constant Y scale + concat:
   ```rust
   let t  = read_f32(this+0x4A0);
   let k1 = 1.07f32;                        // both blend ends are 1.07
   let sy = k1 + t * (1.07 - 1.07);         // == 1.07 exactly
   call3(0x801C0C60, &tmp, f1 = 1.0, f2 = sy, f3 = 1.0);
   call3(0x801C0540, &this[+0x7C], &tmp, &this[+0x7C]);
   ```
7. `0x80752878..0x80752898` - un-lean: `s = (i32)(-(+0x488)) as i16`;
   `call2(0x8016EEF0, &this[+0x7C], &s)`.
8. `0x8075289C..0x807528E4` - advance the first spin angle:
   ```rust
   write_f32(this+0x488,
       read_f32(this+0x488) + 250.0 + read_f32(this+0x4A0) * (2550.0 - 250.0));
   ```
9. `0x807528E8` - second lean: `s = (i32)(-(+0x48C)) as i16`;
   `call2(0x8016EEF0, &this[+0x7C], &s)` (the value is built at
   `0x80752880..0x80752894`, stored at `r1+12`).
10. `0x807528EC..0x80752918` - constant Y scale 1.03 + concat:
    ```rust
    let sy = 1.03f32 + t * (1.03 - 1.03);   // == 1.03
    call3(0x801C0C60, &tmp, f1 = 1.0, f2 = sy, f3 = 1.0);
    call3(0x801C0540, &this[+0x7C], &tmp, &this[+0x7C]);
    ```
11. `0x8075291C..0x80752938` - second un-lean:
    `s = (i32)(+0x48C) as i16`; `call2(0x8016EEF0, &this[+0x7C], &s)`.
12. `0x8075293C..0x80752964` - advance the second spin angle:
    ```rust
    write_f32(this+0x48C,
        read_f32(this+0x48C) + 150.0 + read_f32(this+0x4A0) * (2550.0 - 150.0));
    ```
13. `0x80752968..0x80752974` - `call2(0x8016A170, this+0x3F4, &this[+0x7C])`
    then `call2(0x8016A1A0, this+0x3F4, 0)`.
14. `0x80752978..0x807529A0` - epilogue: `lwz r0,276(r1)`, `psq_l f31`,
    `lfd f31`, `psq_l f30`, `lfd f30`, `lwz r31`, `lwz r30`, `lwz r29`,
    `mtlr r0`, `addi r1,r1,272`, `blr`.

### Constants

| address | value | use |
|---|---:|---|
| `0x807CE7DC`, `0x807CE7EC` | `1.07f` | first Y-scale blend ends |
| `0x807CE7E4`, `0x807CE7F4` | `1.03f` | second Y-scale blend ends |
| `0x807CE7E0` | `250.0f` | first spin base step |
| `0x807CE7F0` | `2550.0f` | first spin fast step |
| `0x807CE7E8` | `150.0f` | second spin base step |
| `0x807CE7F8` | `2550.0f` | second spin fast step |
| `0x807CE810` | `0.0f` | slow blend target |
| `0x807CE82C` | `0.025f` | slow blend rate |
| `0x807CE830` | `0.01f` | slow blend limit |
| `0x807CE834` | `1e-8f` | slow blend band |
| `0x807CE828` | `1.0f` | z scale (both passes) and fast blend target |
| `0x807CE838` | `0.5f` | fast blend rate |
| `0x807CE83C` | `0.1f` | fast blend band |

### Callees

| site | callee | arguments |
|---|---|---|
| `0x807527A0` | `0x80064AF0` | `&pos`, `&ang`, `1` (runtime-dispatched SDA hook) |
| `0x807527B4` | `0x801C0BD0` | matrix, `f1..f3` = position |
| `0x807527C8` | `0x801C0C60` | matrix, `f1..f3` = scale |
| `0x807527D8`, `0x80752874`, `0x80752918` | `0x801C0540` | dst, src, dst (concat) |
| `0x807527FC`, `0x80752818` | `0x8015F140` | value ptr, target, rate, limit, band |
| `0x80752844`, `0x80752898`, `0x807528E8`, `0x80752938` | `0x8016EEF0` | matrix, `&s16angle` (rotation) |
| `0x80752968` | `0x8016A170` | `this+0x3F4`, matrix |
| `0x80752974` | `0x8016A1A0` | `this+0x3F4`, `0` |

`0x8016EEF0` is not a translation: its callee `0x8016EE30` reads the single
`s16` at `[0]`, multiplies it by `[r2-0x5360]` = `[0x8042DD20]` = `0x3B800000`
= `0.00390625` (`1/256`), evaluates the 256-entry sin/cos table at
`0x802F74A0` through `0x80237C40` (sine) and `0x80237BD0` (cosine), and writes
a 3 x 4 rotation matrix, which `0x801C0540` then post-concatenates. The scale
is **not** `pi/32768`; the sine helper's own range reducer uses the unrelated
`[0x8042E4A0] = 65536.0`.

### Notes for the implementer

* The `0x8015F140` argument order is `(r3 = &value, f1 = target, f2 = rate,
  f3 = limit, f4 = band)`; this is provable from its body (`lfs f5,0(r3)` then
  `f6 = f2 * (f1 - f5)`).
* Both Y-scale "lerps" blend two **equal** constants, so `+0x4A0` cancels; keep
  the expression as written if you want a byte-exact reassembly, or fold to the
  constant if only behaviour matters.
* The four `s16` stack slots are 4 bytes apart and each holds one `s16`; the
  callee reads only the first, so do not label the upper two components.
* `fctiwz` truncates toward zero; a negative `+0x488` therefore rounds toward
  zero before the `s16` store.
* `+0x4A8` is read as a **signed** word (`lwz` then `cmpi`); only `0` selects
  the slow arm.
* Wave 32 rejected this row as "paired-single reconstruction
  (`unaff_GQR0`, two `ldexpf` blocks)" (`docs/COVERAGE_WAVE32_BASES.md:198`);
  the only `psq` words are the two scalar spill pairs, so the rejection is
  retired.

---

## 8. `0x80792570` - map-enemy appear/fly state machine

**Evidence.** `work/ghidra/rel-d_basesNP/export/80792570_FUN_80792570.c`, 628
bytes, 157 words, synthetic `0x80792570`. Body `0x80792570..0x807927E4` (ends
`blr` at `0x807927E0`). One `f31` spill pair (`W = 0`, `qr0`). `this` lives in
`r30`; `r31` = the result flag, `r29` = pool `0x807CE7C8 - 0x124` =
`0x807CFEA8`.

**What it is.** A five-state per-frame machine (states `1..5` in `+0x6CC`) for a
world-map enemy that has just been spawned: it plays a 0 -> 1.0 appear
animation, emits the `100` world-map effect and a particle flourish, sets the
velocity toward the player, runs the scale/appear ramp through `0x800F6AB0`,
then either repeats the flight (`+0x6C8` count) or retires. It returns `1` once
state 5 has run. The address is a vtable slot at `0x8081B1F8` (the table also
holds `0x807923F0`, `0x80792460`, `0x80792520`, `0x807924C0`); the class is the
`kuribo`/map-enemy family whose loader is `fn_80792110` (see its doc block in
`src/game/bases.rs`).

### Signature

```rust
pub unsafe fn fn_80792570(this: usize) -> u32
```

`r3` = the map enemy. Returns `1` if state 5 executed this frame, else `0`
(`r31` initialised to `0` at `0x80792588`, set to `1` at `0x807927B8`).

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0xAC`/`+0xB0`/`+0xB4` | `f32` | position |
| `+0x100`/`+0x102`/`+0x104` | `s16` | applied angles (copied by `0x800F8540`) |
| `+0x108` | `s16` | current appear angle stepped by `0x800F8540` |
| `+0x10C` | `f32` | speed (set to `9.0` in state 4) |
| `+0x6B2` | `s16` | target appear angle |
| `+0x6BC` | `u32` | appear-gate countdown |
| `+0x6C4` | `u32` | gate state; `2` disables `0x800F8540` |
| `+0x6C8` | `s32` | remaining flight repetitions |
| `+0x6CC` | `u32` | state (`1..5`) |
| `+0x6DC` | `s32` | one-shot effect latch (`< 0` = not yet emitted) |
| `+0x770` | struct | appear animation object |

### Exact behaviour

Prologue `0x80792570..0x8079259C`: `stwu r1,-64(r1)`, `mflr r0`,
`stw r0,68(r1)`, `stfd f31,48(r1)`, `psq_st f31,56(r1),0,qr0`,
`stw r31,44(r1)`, `li r31,0`, `stw r30,40(r1)`, `r30 = r3`,
`stw r29,36(r1)`.

1. `0x80792598..0x807925A0` - `if call1(0x800F8540, this) == 0 { return 0; }`
   (falls straight to the epilogue at `0x807927BC`).
2. `0x807925A4..0x807925D0` - dispatch on `u32[this+0x6CC]`; any value outside
   `1..5` returns `0`.
3. **State 1** (`0x807925D4..0x8079260C`):
   ```rust
   let frame = call_f32(0x801680C0, this+0x770) + 0.1;   // [0x807CFEC4]
   call_f32_u32(0x801680E0, frame, this+0x770);
   if frame > 1.0 { write_u32(this+0x6CC, 2); }           // [0x807CFEC0]
   ```
4. **State 2** (`0x80792610..0x807926C8`):
   ```rust
   if read_s32(this+0x6DC) < 0 {
       call4(0x80105240, read_u32(0x8042A27C), 100, this+0xAC, 1);
       write_s32(this+0x6DC, read_s32(this+0x6DC) + 1);
   }
   call9(0x800F69F0, f1 = 10.0, f2 = 3.5, f3 = 3.5,
         this, this+0xAC, 0, 20, 0x80377DE0);
   let p = read_u32(0x8042A1A0);                 // player object
   let v = [read_f32(p+0xAC) - read_f32(this+0xAC),
            read_f32(p+0xB0) - read_f32(this+0xB0),
            read_f32(p+0xB4) - read_f32(this+0xB4)];
   call2(0x800F6410, this, &v);                  // set velocity
   write_u32(this+0x6CC, 3);
   ```
   The float arguments are `[0x807CFED0] = 10.0`, `[0x807CFED8] * [0x807CFEAC]`
   = `1.0 * 3.5`, `[0x807CFEDC] * [0x807CFEAC]` = `1.0 * 3.5`.
5. **State 3** (`0x807926CC..0x80792728`):
   ```rust
   if call1(0x800F6AB0, this) != 0 {
       call5(0x80103390, read_u32(0x8042A26C), 12, this+0xAC, 0, 0);
       if read_s32(this+0x6C8) < 1 {
           call1(0x800F6CB0, this);
           write_u32(this+0x6CC, 5);
       } else {
           write_u32(this+0x6CC, 2);
           write_s32(this+0x6C8, read_s32(this+0x6C8) - 1);
       }
   }
   ```
6. **State 4** (`0x8079272C..0x807927AC`):
   ```rust
   write_f32(this+0x10C, 9.0);                   // [0x807CFEA8]
   call1(0x8006CC00, this);
   call1(0x8006CD40, this);
   let p = read_u32(0x8042A1A0);
   let v = [read_f32(p+0xAC), read_f32(p+0xB0), read_f32(p+0xB4)];
   call4(0x800F6CD0, this, this+0xAC, &v, 0xFFFFFFFF);
   call2(0x801C1650, &v, this+0xAC);             // f1 = squared distance
   if call_f32(0x802BE470) < 150.0 {             // sqrt, [0x807CFEC8]
       call1(0x800F6CB0, this);
       write_u32(this+0x6CC, 5);
   }
   ```
7. **State 5** (`0x807927B0..0x807927B8`):
   ```rust
   call1(0x800F7AB0, this);   // +0x6C4 = 1, +0x6CC = 2, +0x6C8 = 10
   r31 = 1;
   ```
8. `0x807927BC..0x807927E0` - epilogue: `psq_l f31,56(r1)`, `lfd f31`,
   `r3 = r31`, `lwz r31,44(r1)`, `lwz r30,40(r1)`, `lwz r29,36(r1)`,
   `lwz r0,68(r1)`, `mtlr r0`, `addi r1,r1,64`, `blr`.

`0x807927BC` is also Ghidra's "removing unreachable block" target - it is the
single shared epilogue.

### Constants

| address | value | use |
|---|---:|---|
| `0x807CFEA8` | `9.0f` | state-4 speed |
| `0x807CFEAC` | `3.5f` | particle scale base |
| `0x807CFEC0` | `1.0f` | appear-frame end |
| `0x807CFEC4` | `0.1f` | appear-frame step |
| `0x807CFEC8` | `150.0f` | retire distance |
| `0x807CFED0` | `10.0f` | particle rate |
| `0x807CFED8`, `0x807CFEDC` | `1.0f` | particle scale factors |
| `0x80377DE0` | `f32[3]` (BSS) | particle position vector |
| `100`, `12` | effect ids | `wm_effect_record_submit` / `wm_effect_spawn_named` |
| `20` | particle count/arg | `0x800F69F0` r6 |

### Callees

| site | callee | arguments |
|---|---|---|
| `0x80792598` | `0x800F8540` (appear gate) | `this` |
| `0x807925D8` | `0x801680C0` | `this+0x770` (current frame query) |
| `0x807925F4` | `0x801680E0` | frame, `this+0x770` |
| `0x80792630` | `0x80105240` `wm_effect_record_submit` | `[0x8042A27C]`, `100`, `this+0xAC`, `1` |
| `0x80792678` | `0x800F69F0` | `f1 = 10.0`, `f2 = 3.5`, `f3 = 3.5`, `this`, `this+0xAC`, `0`, `20`, `0x80377DE0` |
| `0x807926BC` | `0x800F6410` | `this`, `&velocity` |
| `0x807926D0` | `0x800F6AB0` | `this` (appear ramp) |
| `0x807926F4` | `0x80103390` `wm_effect_spawn_named` | `[0x8042A26C]`, `12`, `this+0xAC`, `0`, `0` |
| `0x8079271C`, `0x807927A0` | `0x800F6CB0` | `this` (clear speeds) |
| `0x8079273C` | `0x8006CC00` | `this` |
| `0x80792744` | `0x8006CD40` | `this` |
| `0x80792778` | `0x800F6CD0` | `this`, `this+0xAC`, `&target`, `-1` |
| `0x80792784` | `0x801C1650` | `&v`, `this+0xAC` -> `f1` squared distance |
| `0x80792788` | `0x802BE470` | `f1` -> `f1` sqrt |
| `0x807927B4` | `0x800F7AB0` | `this` (re-arm) |

### Notes for the implementer

* `0x800F8540` is the appear gate: it returns `0` while `+0x6C4 == 2` or while
  the `+0x6BC` countdown is non-zero, and otherwise steps the `s16` at `+0x108`
  toward `+0x6B2` with `0x8015F200(..., 100, 0x4000, 0x800)` and copies
  `+0x106/+0x108/+0x10A` into `+0x100/+0x102/+0x104`.
* State 5 does not "finish" the actor: `0x800F7AB0` writes `+0x6C4 = 1`,
  `+0x6CC = 2`, `+0x6C8 = 10`, i.e. it re-arms ten more flights. The return
  value `1` is the caller's "handled this frame" signal.
* The distance in state 4 is `sqrt(|player - this|^2)`; `0x801C1650` leaves
  only `f1`, so `0x802BE470` must be called immediately (it is).
* `[0x8042A1A0]` is the player object; `[0x8042A27C]` and `[0x8042A26C]` are
  the world-map effect managers (both `main.dol` BSS).
* The state-2 `+0x6DC` latch starts negative and is incremented exactly once;
  the effect id `100` is therefore emitted once per spawn.
* Wave 90 called this "the per-frame mesh ..." body
  (`docs/COVERAGE_WAVE90_BASES.md:127`); the `psq` pair is only the `f31`
  spill, so nothing blocks it.

---

## Old rejections retired

Six of the eight carry an earlier skip. None has a genuine blocker:

* **`0x80703DC0`** - `docs/COVERAGE_WAVE67_BASES.md:122` and
  `docs/COVERAGE_WAVE29_BASES.md:92` deferred it for the "outlined
  paired-single block (`unaff_GQR0`, `ldexpf`, `bVar == 4 || bVar == 6`)". The
  only `psq` words are the `f31` `stfd`/`psq_st` spill and its `psq_l` reload;
  there is no data-side quantisation and no `GQR` dependency. Wave 121 already
  used its return value (`docs/COVERAGE_WAVE121_BASES.md:151`).
* **`0x8062BE20`** - `docs/COVERAGE_WAVE140_BASES.md:160` left it "for a
  dedicated pass rather than guessed at the tail of this one". Nothing was ever
  found wrong with the body; section 2 writes it.
* **`0x8074CBA0`** - `docs/COVERAGE_WAVE87_BASES.md:149` listed it with the
  join-panel cluster but "bodies not claimed". Every call site and field is
  literal; section 4 writes it.
* **`0x8061B890`** - `docs/COVERAGE_WAVE131_BASES.md:127-130` rejected it as
  "a wall of unverified `.bss` offsets" whose "record element roles are not
  recoverable". The roles are recoverable from `main.dol` `0x800CE260..0x800CE32C`
  (the 4-byte palette at `+0x1A` and the 10 x 42 body at `+0x7C4`) and from the
  wave-131 notes themselves (`0x80354E60` = player -> object id, `0x80354E70` =
  per-player index, `0x80354E80` = claimed flag). Section 6 writes it.
* **`0x80752730`** - `docs/COVERAGE_WAVE32_BASES.md:89` and `:198` rejected it
  as "a paired-single reconstruction ... its register class cannot be proven".
  The two `psq` pairs are scalar `f31`/`f30` spills through `qr0`; every matrix
  and float is explicit. Section 7 writes it.
* **`0x80792570`** - `docs/COVERAGE_WAVE90_BASES.md:127-132` rejected it as
  "the per-frame mesh and walk dispatcher ... exported with `unaff_GQR0` and
  `ldexpf` artifacts (paired-single register pairs that Ghidra could not type),
  so the float storage class of the pose words is not recoverable". The only
  `psq` words in the body are the `f31` `stfd`/`psq_st` prologue pair and its
  `psq_l` reload (`qr0`, `W = 0`); every pose word is an ordinary `lfs`/`stfs`
  or an integer field. Section 8 writes it.

`0x8076B710` and `0x8078A8F0` were never listed as skips in the earlier waves
(they only appear in the `FUNCTION_INDEX.md` neighbourhood of the `0x8076B` and
`0x8078` clusters as unclaimed siblings), so no rejection needs retiring.

## Verification

* Candidate selection was recomputed from scratch with
  `%TEMP%\opencode\w152\pick.js`: 1,847 TSV rows pass every filter
  (`decompiled = true`, export file present, no `ledger/functions.csv` address,
  no `0x`-address in any of the 773 headings of the 24
  `docs/SPEC_WAVE*_BASES.md` files). The eight chosen rows are the largest
  eight; the ninth is `0x807310C0` (624).
* The eight addresses match `docs/SPEC_WAVE151_BASES.md:147-148` exactly, in
  the same order.
* Every address, byte size and export name is read verbatim from
  `work/ghidra/rel-d_basesNP/export/functions.tsv`; every `blr` address and
  every gap was recomputed by decoding `work/rels/elf/d_basesNP.elf` through
  its program headers.
* `ledger/functions.csv` contains no row for any of the eight, and no
  `SPEC_WAVE*_BASES.md` heading contains any of the eight.
* The decoder reports zero unknown words in the 1,272 words; the only
  `unaff_GQR*`/`ldexpf` text in the exports is the scalar spill pairs listed
  above.
* This lane wrote only this file: no Rust file, no `ledger/functions.csv` row,
  no existing document, and no merge/check tool. Other lanes are editing
  `src/game/bases.rs` and `ledger/agents/wave152-*.csv` concurrently; those
  changes are not this lane's (which is why the Rust citations above name the
  doc blocks instead of line numbers).
