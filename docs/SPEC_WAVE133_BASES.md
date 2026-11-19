# Wave 133 - `game::bases` - implementation spec for the next 12 class-B addresses

Read-only lane. The only file written is this one. The throwaway PowerPC decoder
(`%TEMP%\opencode\ppc133.js`) and the ELF readers live outside the repository; no
Rust file, no `ledger/functions.csv` row and no existing document was modified.
No merge or check tool was run.

## Why these twelve

`docs/ANALYSIS_WAVE127_BASES.md` reclassified 121 `d_basesNP` skips as class B
("reconstructible now from the raw stream"). Waves 128-132 carry 63 section
headings over that pool (`## N. \`0xADDR\`` in
`docs/SPEC_WAVE128_BASES.md` .. `docs/SPEC_WAVE132_BASES.md`, overflow and
duplicate sections included). Of the 121 class-B rows exactly **58** are still
neither claimed nor present in the ledger, totalling **9 736** bytes. This lane
takes the twelve largest of those, ordered by `functions.tsv` size descending and
then by address ascending:

| # | address | bytes | words | export file | verdict |
|--:|---:|---:|---:|---|---|
| 1 | `0x80780D00` | 276 | 69 | `80780d00_FUN_80780d00.c` | ready (medium) |
| 2 | `0x80711590` | 272 | 68 | `80711590_FUN_80711590.c` | ready (medium) |
| 3 | `0x80643D40` | 268 | 67 | `80643d40_FUN_80643d40.c` | ready (high) |
| 4 | `0x806B39B0` | 268 | 67 | `806b39b0_FUN_806b39b0.c` | ready (high) |
| 5 | `0x806FCDF0` | 268 | 67 | `806fcdf0_FUN_806fcdf0.c` | conditional (medium) |
| 6 | `0x807485D0` | 268 | 67 | `807485d0_FUN_807485d0.c` | conditional (medium) |
| 7 | `0x806329E0` | 264 | 66 | `806329e0_FUN_806329e0.c` | ready (high) |
| 8 | `0x8065C100` | 264 | 66 | `8065c100_FUN_8065c100.c` | ready (medium) |
| 9 | `0x8065C260` | 264 | 66 | `8065c260_FUN_8065c260.c` | ready (medium) |
| 10 | `0x806FFB50` | 256 | 64 | `806ffb50_FUN_806ffb50.c` | ready (high) |
| 11 | `0x807241C0` | 256 | 64 | `807241c0_FUN_807241c0.c` | ready (medium) |
| 12 | `0x807811D0` | 256 | 64 | `807811d0_FUN_807811d0.c` | ready (high) |

Together **3 180 bytes** (795 words). Every export path is exactly
`work/ghidra/rel-d_basesNP/export/<file>` relative to the repository root, and
every `bytes` value matches the `size` column of that directory's
`functions.tsv` verbatim.

### Selection rule

The candidate set is: **class B in wave 127**, **no row in
`ledger/functions.csv` whose address column equals the address**, and **no
`## N. \`0xADDR\`` heading in waves 128-132**. The ledger test is *row-based*
(comma field 2 of each line), not a substring search: three of the twelve are
named inside the free-text evidence column of unrelated rows, and a substring
test would silently drop them.

| address | ledger mentions | where |
|---|---:|---|
| `0x80780D00` | 1 | evidence of `fn_8077ede0` (ledger line 4470) |
| `0x807485D0` | 1 | evidence of `fn_80748510` (ledger line 4058) |
| `0x807811D0` | 2 | evidence of `fn_8077e6d0` (4467) and `fn_8077fa10` (4480) |

As recorded at `docs/SPEC_WAVE129_BASES.md:1464-1468` and reused at
`docs/SPEC_WAVE130_BASES.md:1283-1286`, `docs/SPEC_WAVE131_BASES.md:40-59` and
`docs/SPEC_WAVE132_BASES.md:40-57`, **a textual mention is not a claim** - only a
section heading is.

### Ties, and what is left

The pool happens to break cleanly at this cut: four bodies are exactly 268 bytes
and three are exactly 256, and both groups fit inside the twelve. The 13th
largest is 248 bytes, so no tie-break decision is required. The next six, in the
same order, are:

| address | bytes | note |
|---|---:|---|
| `0x80608DF0` | 248 | straight-line, no `psq` (wave 11 collection-coin handler) |
| `0x807268C0` | 248 | `qr0` `f31` spill only |
| `0x80780A90` | 240 | sibling of §1: distance between `this+0xAC` and `this+0x1D8` |
| `0x8069A6E0` | 236 | has one real data-side `qr0` load (`psq_l`) |
| `0x80643470` | 228 | six direct calls, no `psq` |
| `0x8068E440` | 228 | `qr0` `f31` spill only |

## Method

**Address mapping (re-verified, not copied).** `d_basesNP.elf` has six `PT_LOAD`
headers. The first is `p_offset = 0x10F0`, `p_vaddr = 0x806000F0`, so

```
file_offset = 0x10F0 + (address - 0x806000F0) = (address - 0x80600000) + 0x1000
```

The other five are `0x807C60F4` (1 276 B), `0x807C65F0` (12 B), `0x807C6600`
(42 492 B), `0x807D0C00` (335 032 B) and the zero-fill segment at `0x80822900`
(`filesz = 0`, `memsz = 74 884`). Every constant below was read at
`p_offset + (addr - p_vaddr)`.

**Cross-artifact constants.** Three of the twelve read cells that are not in this
REL. `work/dol/main.elf` (13 program headers; `.data` at `0x8042B080`+32 352 B,
`.bss` segments at `0x80351680`/0x80429BC0) resolved them:

| cell | artifact | value |
|---|---|---|
| `0x8042DCD8` | `main.elf` PH11, `0x8042B080` + 0x2C58 | `0x3BB40000` = `360/65536` |
| `0x80429E98`, `0x80429EA0`, `0x8042A1C8` | `main.elf` PH10 (`0x80429BC0`, filesz 0) | zero-initialised BSS, runtime cells |
| `0x80354E60` | `main.elf` PH8 (`0x80351680`, filesz 0) | zero-initialised BSS, filled at run time |

`0x80439D64`, `0x8042A4A8` and `0x8043A1C8` lie **outside every program segment
of `main.elf` and of this REL**; they are runtime globals with no shipped bytes.
They are still readable at run time and are always cited as addresses, never as
literals.

**Extent check.** For all twelve bodies the reachable span equals the TSV size
exactly: the last word of every range is `0x4E800020` (`blr`) at
`entry + size - 4`, and no TSV row starts inside a body. No over-read, no
`halt_baddata`, no jump table. `0x807811D0` is a leaf with **no** `mfspr LR` and
no LR save at all.

**Decoder corrections that changed the reading of this batch.** Four field-order
traps were hit and are worth recording, because each one silently produces
plausible-but-wrong code:

* **Logical X-form puts the destination in bits 11-15, not 6-10.**
  `or rA,rS,rB` assembles as `31<<26 | rS<<21 | rA<<16 | rB<<11 | 444<<1`. A
  decoder that prints the D-form order turns `0x7C7C1B78` into `or r3,r28,r3`
  instead of `or r28,r3,r3` (i.e. `mr r28,r3`). In `0x80780D00` that single error
  swaps the receiver with the index and inverts the whole body. The same applies
  to `and`/`xor`/`nor`/`nand`/`andc`/`orc`, to `slw`/`srw`/`sraw`/`srawi`, to
  `cntlzw`/`extsb`/`extsh` and to `rlwinm` (`rlwinm rA,rS,sh,mb,me`).
* **`bc` naming must come from BO bit 0x10 and bit 0x08.** `BO & 0x10 == 0`
  means "test CR", `BO & 0x08` selects "branch if the bit is set". `0x4182` is
  `beq` (BO=12, BI=2) and `0x4082` is `bne` (BO=4, BI=2). Deriving the mnemonic
  from `BO & 0x14` prints `beq` for `bne` and inverts every guard in the batch;
  `0x80643D40`'s four pane flags depend on it.
* **A-form single-precision operands.** `fmuls fD,fA,fC` uses FRC = bits 6-10,
  while `fadds`/`fsubs`/`fdivs` use FRB = bits 11-15. Printing four operands
  turns `70.0 * 0.5` (in `0x80780D00`) into `70.0 * 0.5 * 0.5`; the wave-132
  note about `0x80780D00`'s sibling is the same trap.
* **`addi rD,r0,imm` is `li`.** r0 in the base position means literal zero, so
  `0x38A00000` is `li r5,0` and `0x38600001` is `li r3,1`. A decoder that treats
  r0 as "the caller's r0" mis-reads the flag init in `0x8065C260`, the
  `li r3,1` return in `0x806FCDF0` and the `li r0,0` in `0x8065C100`.
* **`cror 2,0,2` and `cror 2,1,2` differ by one bit and invert a predicate.**
  `0x4C401382` is `cror 2,0,2` (`CR0.EQ := CR0.LT | CR0.EQ`) and `0x4C411382` is
  `cror 2,1,2` (`CR0.EQ := CR0.GT | CR0.EQ`). After `cror 2,0,2` a `bne` means
  "branch when `fA > fB`"; after `cror 2,1,2` the same `bne` means "branch when
  `fA < fB`". `0x807485D0` uses both in the same predicate chain.

**Paired-single audit.** Across the 795 words there are **26 `psq*` words**:
13 `psq_st` prologue saves and 13 `psq_l` epilogue reloads. Every one is `qr0`
(`I = 0`) with `W = 0`, and every one is adjacent to a `stfd`/`lfd` of the same
register. Saved set: `f31, f30, f29, f28`. Pair counts: one each in
`0x80780D00`, `0x80711590`, `0x806329E0`; two in `0x8065C100`, `0x8065C260`,
`0x807241C0`; four in `0x807485D0`; **none** in `0x80643D40`, `0x806B39B0`,
`0x806FCDF0`, `0x806FFB50`, `0x807811D0`. There is **no data-side quantisation
anywhere in this batch and no `mtspr` to a `GQRn`**: the only `mtspr` operands
are `LR` (spr 8) and `CTR` (spr 9). All twelve were rejected by the
wave-111/113/117/118/119/125 "`unaff_GQR0`" bullets; in every case that is a
decompiler artefact.

**Compiler save chain, re-decoded from `work/dol/main.elf`.** Only two members are
used here; both are straight runs of `stw rN,-off(r11)` ending in `blr` and
neither writes `r3..r13` or `LR`:

| save stub | saves | matching restore | used by |
|---|---|---|---|
| `0x802DCD70` | r26-r31 | `0x802DCDBC` | `0x806B39B0` |
| `0x802DCD74` | r27-r31 | `0x802DCDC0` | `0x806329E0` |

`r11` must be `r1 + frame_size`; each body sets it immediately before the call.
After the `bl`, entry `r3` is still the `this` pointer, and both bodies copy it
into the first callee-saved register (`or r28,r3,r3` / `or r31,r3,r3`) on the
next instruction. That is why Ghidra printed `void FUN_806329e0(void)`.

**House style used below.** `pub unsafe fn fn_XXXXXXXX(this: usize)`,
`platform::read_u32/read_i32/read_f32/read_u8/read_u16`,
`platform::write_*`, `abi_call1!..abi_call10!`, `vmethod0(rcv, slot)`,
`ptr_at(addr)`, addresses written `0x8076_1510`, field offsets as `usize`
literals, `// unknown:` on the first use of every address-only target or
external cell.

---

## 1. `0x80780D00` - 276 bytes - `work/ghidra/rel-d_basesNP/export/80780d00_FUN_80780d00.c`

**Body:** `0x80780D00..0x80780E10` (69 words, `blr` at `0x80780E10`).
Ledger: absent as a row; mentioned only in the evidence of `fn_8077ede0`
(ledger line 4470). `src/game/bases.rs`: no definition; five doc-comment
mentions. The export carries the wave-127 "removing unreachable block" warning at
`0x80780DF0` (the `psq_l`/`lfd` restore pair) - spurious.

```rust
pub unsafe fn fn_80780d00(this: usize, index: u32, pos: *const f32, extra: u32, arg_f: f32)
```

**Argument map.** `r3 -> this`, `r4 -> index` (stored at `+0x270` and used as the
table index), `r5 -> pos` (three `lfs` at `0(r29)`/`4(r29)`/`8(r29)`),
`r6 -> extra` (`or r30,r6,r6` at `0x80780D2C`, re-emitted as `r6` for the last
call), `f1 -> arg_f` (`fmr f31,f1` at `0x80780D18`).

**Frame.** `stwu r1,-64(r1)`; `mfspr r0,LR` / `stw r0,68(r1)`;
`stfd f31,48(r1)` + `psq_st f31,56(r1),0,0`;
`stw r31,44` / `r30,40` / `r29,36` / `r28,32`. No save-helper.
Epilogue `0x80780DEC..0x80780E10`.

**Shape.** Attach an effect slot to `this`, copy the caller's vector into the
actor's `+0x1D8` triple, gate a spawn flag on the distance to that triple, then
submit a builder with the vector delta and the actor's `+0x294` scale pair.

1. `0x80780D0C` - `r0 = index * 4` (`rlwinm r0,r4,2,0,29`); `r31` is
   `0x807CF608` (`lis 0x807D` / `addi -2552`).
2. `0x80780D44-0x80780D50` - `r28 = this`; `*(u32*)(this+0x270) = index`;
   `r4 = *(u32*)(0x807CF7C4 + 4*index)`; `r5 = 0xFFFFFFFF`; `f1 = 0.0`.
3. `0x80780D54` - `0x8077E410(f1 = 0.0, r3 = this, r4 = table[index],
   r5 = -1)`. The callee uses its third argument as an **index** into
   `*(u32*)(this+0x294) + 12*index`, so the stored table values are slot ids
   `4, 9, 10`, not byte offsets. The return value is discarded:
   `0x80780D5C` reloads `r3 = this` and the next instruction overwrites `r3`
   anyway (dead store, keep or drop).
4. `0x80780D58-0x80780D70` - `*(f32*)(this+0x1D8/0x1DC/0x1E0) = pos[0..2]`.
5. `0x80780D74` - `f1 = 0x80780A90(this)` (the callee measures
   `|this+0xAC - this+0x1D8|`, so this is a distance).
6. `0x80780D78-0x80780D94` - `r5 = (f1 < 70.0 * 0.5) ? 15 : 0`
   (`fcmpo cr0,f1,f0` / `bge` past / `li r5,15`). `70.0` is `0x807CF674`, `0.5`
   is `0x807CF7B0`; the compiled threshold is exactly `35.0`.
7. `0x80780D98-0x80780DD8` - the delta vector at `r1+8`:
   `+8 = pos[0] - this[0xAC]`, `+0xC = pos[1] - this[0xB0]`,
   `+0x10 = pos[2] - this[0xB4]`. Note the three subtracts are emitted in the
   order z, y, x; the stores are in order `+0xC`, `+8`, `+0x10`.
8. `0x80780DDC-0x80780DE8` - `r8 = *(u32*)(this+0x294)`;
   `f2 = f3 = *(f32*)(r8+0x18)`; then
   `0x800F69F0(f1 = arg_f, f2 = f3 = *(f32*)(*(u32*)(this+0x294)+0x18),
   r3 = this, r4 = pos, r5 = gate, r6 = extra, r7 = &delta)`.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80780D54` | `0x8077E410` | `f1 = 0.0`, `r3 = this`, `r4 = *(u32*)(0x807CF7C4 + 4*index)`, `r5 = -1` |
| `0x80780D74` | `0x80780A90` | `r3 = this` -> `f1` (distance) |
| `0x80780DE8` | `0x800F69F0` | `f1 = arg_f`, `f2 = f3 = *(f32*)(*(u32*)(this+0x294)+0x18)`, `r3 = this`, `r4 = pos`, `r5 = 0/15`, `r6 = extra`, `r7 = &delta` |

No virtual calls. Two `psq*` words, `qr0`/`W=0` (one `f31` pair).

**Constants:**

| value | image address | materialised at |
|---|---|---|
| shared `f32` pool base | `0x807CF608` | `addis r31,r0,-32643` @`0x80780D20` + `addi r31,r31,-2552` @`0x80780D24` |
| `f32 0.0` | `0x807CF698` | `lfs f1,144(r31)` @`0x80780D30` |
| `u32[3] = {4, 9, 10}` (slot ids) | `0x807CF7C4` | `addi r4,r31,444` @`0x80780D4C` + `lwzx r4,r4,r0` @`0x80780D50` |
| `f32 70.0` | `0x807CF674` | `lfs f2,76(r3)` with `r3 = r31+32` @`0x80780D80` |
| `f32 0.5` | `0x807CF7B0` | `lfs f0,424(r31)` @`0x80780D7C` |

**Confidence: medium.** All 69 words decode, the only branch is the literal
`< 35.0` gate, and the three table entries are read from the image. The residual
doubt is `0x800F69F0`'s eight-argument contract (five integers, three floats);
emit it as `abi_call7!` plus `// unknown: 0x800F69F0(f32, f32, f32, this, pos,
flag, arg)` and keep the gate as `u32` (`0` / `15`).

---

## 2. `0x80711590` - 272 bytes - `work/ghidra/rel-d_basesNP/export/80711590_FUN_80711590.c`

**Body:** `0x80711590..0x8071169C` (68 words, `blr` at `0x8071169C`).
Ledger: absent (no mention anywhere in the CSV). `src/game/bases.rs`: no
definition; five doc-comment mentions.

```rust
pub unsafe fn fn_80711590(this: usize)
```

**Frame.** `stwu r1,-48(r1)`; `mfspr r0,LR` / `stw r0,52(r1)`;
`stfd f31,32(r1)` + `psq_st f31,40(r1),0,0`; `stw r31,28` / `r30,24`.
Epilogue `0x80711680..0x8071169C`.

**Shape.** A "crossed a positional threshold" test with three gates, ending in a
virtual message dispatch.

1. `0x807115B0-0x807115D4` - `r30 = this`; `r5 = *(u32*)(this+0x694)`;
   `r0 = *(u32*)0x80805EA0` (= **29**); snapshot the position into
   `r1+8/0xC/0x10` (x, y, z, in that order).
2. `0x807115C4` + `0x807115D8` - `if (*(i32*)(this+0x694) < 29) 0x807103D0();`
   (`cmp cr0,r5,r0` / `bge` past; **signed** compare).
3. `0x807115E0-0x80711630` - the flag:
   ```
   f2 = *(f32*)0x808063B0;      // shipped value 0.0
   f0 = *(f32*)0x807CCA7C;      // shipped value 0.0
   flag = 0;
   if (f2 < f0 && snapshot.x < *(f32*)(this+0xAC)) flag = 1;
   else if (f2 > f0 && snapshot.x > *(f32*)(this+0xAC)) flag = 1;
   ```
   The raw stream has the second block reached by fall-through, so transcribe it
   as two `if`s whose conditions are mutually exclusive (that is exactly what the
   `bge`/`ble` pair at `0x807115FC`/`0x8071161C` does). `bge`/`ble` are BO=4, so
   they branch on the *clear* bit; a decoder that prints them as `blt`/`bgt`
   inverts the predicate. With the shipped values both cells are `0.0`, so
   neither ordered comparison can be true and the whole tail is unreachable
   unless `0x808063B0` is rewritten at run time by another module (it is read by
   `0x8070F7C0` and `0x80710290`; no writer exists in this REL).
4. `0x80711634-0x80711640` - `if (flag == 0) return;`; `f31 = *(f32*)0x808063DC`
   (= `0.5`).
5. `0x80711648-0x80711660` - `if (0x800B2EF0() >= 0.5) return;` and
   `if (*(i32*)(this+0x694) >= 29) return;`.
6. `0x80711664-0x8071167C` - `vmethod(this, 0xD4)(this, 0x8082F788)`:
   `r12 = *(u32*)(this+0x60)`; `r12 = *(u32*)(r12+0xD4)`; `mtctr`; `bctr`.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x807115DC` | `0x807103D0` | none (uses `this` in `r3`) |
| `0x80711648` | `0x800B2EF0` | none -> `f1` |

**Virtual slots:** `*(u32*)(this+0x60) -> +0xD4`, `r3 = this`,
`r4 = 0x8082F788`. One call, no other indirection.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `u32 29` (threshold) | `0x80805EA0` | `lwz r0,24224(r4)` @`0x807115B8` (`r4 = 0x80800000`) |
| `f32 0.0` | `0x808063B0` | `lfs f2,1296(r4)` @`0x807115F0` |
| `f32 0.0` | `0x807CCA7C` | `lfs f0,-13700(r3)` @`0x807115EC` (`r3 = 0x807D0000`) |
| `f32 0.5` (time gate) | `0x808063DC` | `lfs f31,1340(r3)` @`0x80711640` |
| message payload cell | `0x8082F788` | `addi r4,r4,-2168` @`0x80711674` (`r4 = 0x80830000`) |

`0x8082F788` is inside the REL's zero-fill segment
(`0x80822900..0x80834D84`, `filesz = 0`), so the shipped image carries no bytes
for it. Pass the address, never a value.

**Confidence: medium.** All 68 words decode and the three gates are literal; the
two soft spots are `0x807103D0`'s contract (it runs when the counter is low, and
it is not in this REL) and the meaning of `r4 = 0x8082F788`. Write
`// unknown: 0x807103D0(this)` and `// unknown: payload cell 0x8082F788`, and
keep the `f2 < f0` / `f2 > f0` pair exactly as shipped - do not fold it to
`f2 != f0`, the NaN cases differ.

---

## 3. `0x80643D40` - 268 bytes - `work/ghidra/rel-d_basesNP/export/80643d40_FUN_80643d40.c`

**Body:** `0x80643D40..0x80643E48` (67 words, `blr` at `0x80643E48`).
Ledger: absent. `src/game/bases.rs`: no definition; four mentions.

```rust
pub unsafe fn fn_80643d40(this: usize)
```

**Frame.** `stwu r1,-32(r1)`; `mfspr r0,LR` / `stw r0,36(r1)`;
`stw r31,28` / `r30,24` / `r29,20` / `r28,16`. No `psq`, no save-helper.
Epilogue `0x80643E2C..0x80643E48`.

**Shape.** Clear one per-pane flag on up to four player slots, reset seven
per-pane records, then reset one more object and poke the message system.

1. `0x80643D48` + `0x80643D54` - `r6 = 0x80354E60` (the four-entry slot table,
   `main.dol` BSS); `r5 = 1`.
2. `0x80643D68` - `r28 = this`.
3. `0x80643D6C-0x80643DE8` - four unrolled copies, `i = 0..3`:
   ```
   if (*(u8*)(this + 0x8D8 + i) != 0) {          // lbz + cmpwi 0 + beq
       slot = *(u32*)(0x80354E60 + 4*i);         // lwz r0,(4*i)(r6)
       p    = *(u32*)(this + 4*slot + 0x2E4);    // rlwinm + add + lwz
       *(u8*)(p + 0x269) = 1;                    // stb r5,617(r4)
   }
   ```
   The gate is `!= 0` on a zero-extended byte (`lbz` then `cmpwi ...,0` then
   `beq` past), **not** a sign test; the slot table is a runtime table, so the
   index is never a compile-time constant. The four flags live at
   `+0x8D8/+0x8D9/+0x8DA/+0x8DB`.
4. `0x80643DEC-0x80643E10` - `r30 = this`, `r29 = 0`, `r31 = 0`;
   do `{ ... } while (++r29 < 7)` (stride 4 on `r30`):
   ```
   o = *(u32*)(this + 0x2F4 + 4*i);   // lwz r3,756(r30)
   *(u8*)(o + 0x209) = 0;             // stb r31,521(r3)
   0x807678C0(o);                     // r3 still holds o
   ```
   The store precedes the call - keep that order.
5. `0x80643E14-0x80643E28` - `o = *(u32*)(this+0x310)`;
   `*(u8*)(o+0x261) = 0`; then
   `0x8019CC70(*(u32*)0x8042A4A8)`.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80643E00` | `0x807678C0` | `r3 = *(u32*)(this + 4*i + 0x2F4)` (7 times) |
| `0x80643E28` | `0x8019CC70` | `r3 = *(u32*)0x8042A4A8` |

No virtual calls, no `psq` at all.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `u32[4]` player-slot table | `0x80354E60` | `addis r6,r0,-32715` + `addi r6,r6,20064` @`0x80643D48`/`0x80643D54` |
| message singleton pointer | `0x8042A4A8` | `addis r3,r0,-32701` + `lwz r3,-23384(r3)` @`0x80643E1C`/`0x80643E24` |
| per-slot pointer array | `this+0x2E4` | `lwz r4,740(r4)` @`0x80643D84` etc. |
| flag byte written | offset `0x269` | `stb r5,617(r4)` @`0x80643D88` etc. |
| pane records | `this+0x2F4`, 7 x 4 | `lwz r3,756(r30)` @`0x80643DF8` |
| record flag | offset `0x209` | `stb r31,521(r3)` @`0x80643DFC` |
| secondary record | `this+0x310`, flag `0x261` | `lwz r4,784(r28)` @`0x80643E14` |

**Confidence: high.** Every operand, all four unrolled copies, the seven-step
loop and both tail calls are literal. `0x80354E60` is BSS in `main.dol` (filled by
the player-registration code), so read it with `read_u32(0x8035_4E60 + 4*i)`; do
not substitute constants. `0x8042A4A8` has no shipped bytes - keep it as an
address.

---

## 4. `0x806B39B0` - 268 bytes - `work/ghidra/rel-d_basesNP/export/806b39b0_FUN_806b39b0.c`

**Body:** `0x806B39B0..0x806B3AB8` (67 words, `blr` at `0x806B3AB8`).
Ledger: absent. `src/game/bases.rs`: no definition; four mentions.

```rust
pub unsafe fn fn_806b39b0(this: usize) -> u32
```

**Frame.** `stwu r1,-32(r1)`; `mfspr r0,LR` / `stw r0,36(r1)`;
`addi r11,r1,32`; `bl 0x802DCD70` at `0x806B39C0` (saves **r26-r31**, returns
with `r3` intact). `r28 = this + 0x3F0`, `r27 = this + 0x510`, `r29 = 0` (mask),
`r31 = 1`. Epilogue `0x806B3AA0..0x806B3AB8` (`addi r11,r1,32`,
`or r3,r29,r29`, `bl 0x802DCDBC`, ...).

**Shape.** Walk three 96-byte groups of intrusive lists; for each node ask its
object for a 0..3 seat index and accumulate `1 << index` into the result; in the
`state == 1` case also ask a second object returned by `0x8012DBE0`.

1. `0x806B39D4-0x806B39D8` - `node = *(u32*)(group+0x10)`;
   `b` to the loop test at `0x806B3A8C`.
2. `0x806B3A8C-0x806B3A90` - `while (node != 0)` (`cmpwi` / `bne`).
3. `0x806B39DC-0x806B39E8` - `o = *(u32*)(node+4)`;
   `if (*(u8*)(o+0x38C) != 1) goto arm2`.
4. `0x806B39EC-0x806B3A50` - **arm 1** (`state == 1`):
   ```
   slot = (s8)*(u8*)vmethod(o, 0x6C)();          // lbz 0(r3) + extsb
   if (slot >= 4) goto next;                     // cmplwi 4 / bge
   mask |= 1 << slot;                            // slw r0,r31,r0 ; or
   p = 0x8012DBE0(o);
   if (p == 0) goto next;                        // beq
   slot = (s8)*(u8*)vmethod(p, 0x6C)();
   if (slot >= 4) goto next;
   mask |= 1 << slot;
   ```
   Note `slw r0,r31,r0` with `r31 == 1` (`0x7FE00030`), and the compare is
   `cmplwi` on the sign-extended byte, so a negative `state` byte fails the
   `>= 4` test and is skipped.
5. `0x806B3A54-0x806B3A84` - **arm 2** (`state == 2`):
   same single probe and mask update, no `0x8012DBE0` step. `state` values other
   than 1 and 2 fall straight to `next`.
6. `0x806B3A88` - `node = *(u32*)(node+0xC)`.
7. `0x806B3A94-0x806B3A9C` - `group += 0x60`;
   `while (group < this + 0x510)`.
8. `0x806B3AA0-0x806B3AA8` - `r3 = mask` then restore.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x806B39C0` | `0x802DCD70` | save r26-r31; `r11 = r1+32` |
| `0x806B3A1C` | `0x8012DBE0` | `r3 = o` -> `u32` |
| `0x806B3AA8` | `0x802DCDBC` | restore r26-r31; `r11 = r1+32` |

**Virtual slots:** `*(u32*)(o+0x60) -> +0x6C`, called with `r3 = o`, returns a
pointer whose first byte is the seat index; three call sites
(`0x806B39FC`, `0x806B3A34`, `0x806B3A6C`).

**Constants:**

| value | image address | materialised at |
|---|---|---|
| group base | `this+0x3F0` | `addi r28,r3,1008` @`0x806B39C4` |
| group limit | `this+0x510` | `addi r27,r3,1296` @`0x806B39C8` |
| group stride | `0x60` | `addi r28,r28,96` @`0x806B3A94` |
| state byte | offset `0x38C` | `lbz r0,908(r30)` @`0x806B39E0` |
| list head / next | `+0x10` / `+0xC` | `lwz r26,16(r28)` @`0x806B39D4`; `lwz r26,12(r26)` @`0x806B3A88` |
| node payload | `+4` | `lwz r30,4(r26)` @`0x806B39DC` |
| seat bound | `4` | `cmplwi cr0,r0,4` @`0x806B3A08` |
| mask seed | `1` | `addi r31,r0,1` @`0x806B39D0` |

**Confidence: high.** Every branch, both arms and the three vtable calls are
literal; the only inference is that the returned `u32` is a bitmask, which follows
from `1 << slot` with `slot < 4`. Keep the `extsb` before `cmplwi` - it is what
makes negative indices safe - and keep the mask shift as `1u32 << (slot & 31)`.

---

## 5. `0x806FCDF0` - 268 bytes - `work/ghidra/rel-d_basesNP/export/806fcdf0_FUN_806fcdf0.c`

**Body:** `0x806FCDF0..0x806FCEF8` (67 words, `blr` at `0x806FCEF8`).
Ledger and `src/game/bases.rs`: absent (zero mentions anywhere).

```rust
pub unsafe fn fn_806fcdf0(this: usize) -> u32
```

**Frame.** `stwu r1,-32(r1)`; `mfspr r0,LR` / `stw r0,36(r1)`;
`stw r31,28` / `r30,24` / `r29,20`. No `psq` at all.
Epilogue `0x806FCEE0..0x806FCEF8`.

**Shape.** A reset/initialiser: clears an eight-word block plus two scalars,
draws a random state and a random player, resolves that player's character id,
then clears the remaining counters and returns `1`.

1. `0x806FCDFC` - `r0 = 0` (`addi r0,r0,0`).
2. `0x806FCE10-0x806FCE30` - byte/word clears on `this`:
   `+0x1D4`, `+0x1D5`, `+0x1D6` (bytes); `+0x1D8`, `+0x1E0`, `+0x1E4`,
   `+0x1DC`, `+0x1E8` (words); `+0x6A0` (byte). Note the word order is
   `0x1D8, 0x1E0, 0x1E4, 0x1DC, 0x1E8` - transcribe it as produced.
3. `0x806FCE34` - `n = 0x800B2ED0(1000)` (`li r3,1000`).
4. `0x806FCE40-0x806FCE58` - `i = 0`; `while (i < n) { 0x800B2ED0(1000); i++; }`
   (head `0x806FCE54` `cmp cr0,r30,r31` / `blt` back to `0x806FCE48`).
   This is a deliberate burn of `rand(1000)` draws; it is fully literal.
5. `0x806FCE5C-0x806FCE64` - `*(u32*)(this+0x1D0) = 0x800B2ED0(6)`
   (`li r3,6`).
6. `0x806FCE68-0x806FCE6C` - `*(u32*)(this+0x210) = 0x8005FEF0()`
   (that callee counts the active players by walking `0x80354E60`/`0x80354E90`,
   so the value is 0..4).
7. `0x806FCE70-0x806FCE80` - `idx = 0x800B2ED0(*(u32*)(this+0x210))`;
   `r4 = idx`; `r3 = this`; `v = 0x806FE290(this, idx)`. `0x806FE290` walks
   player slots `idx..3`, returns `-1` if none exists, else the low byte of the
   object's `vmethod(...,0x6C)` result.
8. `0x806FCE84-0x806FCE94` - `*(u32*)(this+0x208) = v`;
   `*(u32*)(this+0x20C) = v`.
9. `0x806FCE98-0x806FCEDC` - the remaining clears with `r5 = 0`, `r4 = 255`,
   `r0 = 7`, `r3 = 1`:
   `+0x220` (byte), `+0x460` (byte), `+0x6A1` (byte), `+0x6A2` (byte = 255),
   `+0x214`, `+0x218`, `+0x21C` (= 7), `+0x700`, `+0x70`, `+0x704` (byte),
   `+0x1EC`, `+0x1F0`, `+0x1F4`, `+0x1F8`, `+0x1FC`, `+0x200`, `+0x204`.
10. `0x806FCEE0` - epilogue. `r3` still holds `1` (`0x806FCE98`), so the
    function returns **1**.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x806FCE38` | `0x800B2ED0` | `r3 = 1000` -> `u32` (LCG draw scaled to `[0,n)`) |
| `0x806FCE4C` | `0x800B2ED0` | `r3 = 1000` (discarded) |
| `0x806FCE60` | `0x800B2ED0` | `r3 = 6` -> `u32` |
| `0x806FCE68` | `0x8005FEF0` | none -> `u32` (active player count) |
| `0x806FCE70` | `0x800B2ED0` | `r3 = *(u32*)(this+0x210)` -> `u32` |
| `0x806FCE80` | `0x806FE290` | `r3 = this`, `r4 = idx` -> `u32` |

No virtual calls in this body (the vtable use is inside `0x806FE290` and
`0x800B2ED0`).

**Constants:**

| value | image address | materialised at |
|---|---|---|
| draw range `1000` | immediate | `addi r3,r0,1000` @`0x806FCE34`, `0x806FCE48` |
| state range `6` | immediate | `addi r3,r0,6` @`0x806FCE5C` |
| count limit `7` | immediate | `addi r0,r0,7` @`0x806FCE8C` |
| `(u8)255` | immediate | `addi r4,r0,255` @`0x806FCE88` |
| PRNG state | `r13-0x573C` (main.dol SDA) | inside `0x800B2ED0` -> `0x80161460` |
| player-slot table | `0x80354E60` | inside `0x8005FEF0` / `0x806FE290` |

`0x800B2ED0` was resolved from the shipped artifacts: it is a 12-byte thunk into
`0x80161460`, which advances a 32-bit LCG state (`r13-0x573C`, multiplier
`r13-0x6238`, addend `r13-0x6234`) and returns `(state * n) >> 32`. So
`0x800B2ED0(n)` is a uniform draw in `[0, n)`; call it `rand_mod(n)` in the body
and document it, do not inline the LCG.

**Residual ambiguity -> conditional (medium).** Two items:

1. **Return type.** `0x806FCE98` `li r3,1` is the only evidence for a return
   value; nothing clobbers `r3` afterwards, so the body provably returns `1`. A
   `bool`-returning `init()` is the natural reading, but Ghidra prints
   `undefined4` and no in-REL caller consumes the result. Write `-> u32`
   returning `1` and add `// inferred: bool init()`.
2. **Purpose of the `rand(1000)` loop.** The loop is literal, but nothing in the
   artifacts says why it burns draws. Keep it, and note it as
   `// unknown: entropy warm-up (rand(1000) iterations)`.

---

## 6. `0x807485D0` - 268 bytes - `work/ghidra/rel-d_basesNP/export/807485d0_FUN_807485d0.c`

**Body:** `0x807485D0..0x807486D8` (67 words, `blr` at `0x807486D8`).
Ledger: absent as a row; mentioned only in the evidence of `fn_80748510`
(ledger line 4058). `src/game/bases.rs`: no definition; three mentions.

```rust
pub unsafe fn fn_807485d0(this: usize, other: usize) -> u32
```

**Argument map.** `r3 -> this` (`or r7,r3,r3` at `0x807485E4`; read once as
`*(u32*)(this+0x394)`), `r4 -> other` (`or r31,r4,r4` at `0x8074860C`; read as
`other+0xAC` and `other+0xB0`).

**Frame.** `stwu r1,-96(r1)`; `mfspr r0,LR` / `stw r0,100(r1)`;
four `stfd`+`psq_st` pairs (`f31` 80/88, `f30` 64/72, `f29` 48/56, `f28` 32/40);
`stw r31,28`. No save-helper. Epilogue `0x807486A8..0x807486D8`.

**Shape.** Fetch a rectangle from a per-stage collision object, then test the
other actor's `(x, y)` against the two intervals built from that rectangle.

1. `0x80748610-0x80748634` - `r5 = *(u32*)0x8042A1C8`;
   `r3 = *(u32*)0x80429E98`; `o = r3 + 944 * *(u8*)(r5+0x120E)`;
   `if (*(u32*)(o+0x2C) == 0) o = 0;` (the `beq` jumps to `li r3,0`).
2. `0x80748638-0x80748644` - `r4 = *(u32*)(this+0x394) & 0xFF`;
   `r5 = &r1+8`; `0x8008E890(o, r4, &r1+8)`. The callee writes **four floats**
   at `r1+8/0xC/0x10/0x14`; nothing else writes that block.
3. `0x80748648-0x80748660` - `f31 = [r1+8]`, `f30 = [r1+0xC]`,
   `f1 = [r1+0x10]`, `f0 = [r1+0x14]`;
   `f29 = f31 + f1`; `f28 = f30 - f0`; `f1 = *(f32*)(other+0xAC)`.
4. `0x80748664` - `f1 = 0x80101AA0(f1)`. The result replaces `f1`, so this is a
   coordinate transform applied to `other`'s x.
5. `0x80748668-0x807486A0` - the four-part predicate; each `bne` jumps to
   `return 0` at `0x807486A4`:
   ```
   f0 = *(f32*)(other+0xB0);
   ok =  (f31 <= f1)          // fcmpo f31,f1 ; cror 2,0,2 ; bne  = GT
      && (f1  <= f29)         // fcmpo f1,f29 ; cror 2,0,2 ; bne  = GT
      && (f30 >= f0)          // fcmpo f30,f0 ; cror 2,1,2 ; bne  = LT
      && (f0  >= f28);        // fcmpo f0,f28 ; cror 2,1,2 ; bne  = LT
   return ok ? 1 : 0;
   ```
   The first two `cror`s are `0x4C401382` (`EQ := LT|EQ`), the last two are
   `0x4C411382` (`EQ := GT|EQ`). Mixing them up produces a box test that is
   transposed.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80748644` | `0x8008E890` | `r3 = o` (or `0`), `r4 = *(u8*)(this+0x394)`, `r5 = &r1+8` -> writes 4 floats |
| `0x80748664` | `0x80101AA0` | `f1 = *(f32*)(other+0xAC)` -> `f1` |

No virtual calls. Eight `psq*` words, all `qr0`/`W=0`
(`f31/f30/f29/f28` saves).

**Constants:**

| value | image address | materialised at |
|---|---|---|
| stage-manager pointer | `0x8042A1C8` | `lwz r5,-24120(r5)` @`0x80748610` (`r5 = 0x80430000`) |
| collision-table base | `0x80429E98` | `lwz r3,-24936(r6)` @`0x80748614` (`r6 = 0x80430000`) |
| stage index byte | `[0x8042A1C8]+0x120E` | `lbz r0,4622(r5)` @`0x80748618` |
| record stride | `944` (0x3B0) | `mulli r0,r0,0x3B0` @`0x8074861C` |
| record validity word | `+0x2C` | `lwz r0,44(r3)` @`0x80748624` |
| player index | `this+0x394` | `lwz r0,916(r7)` @`0x80748638` |

Both `0x8042A1C8` and `0x80429E98` are `main.dol` BSS cells (`PH10`,
`0x80429BC0`, filesz 0) and `0x8043A1C8`-style cells have no shipped bytes:
keep them as addresses.

**Residual ambiguity -> conditional (medium).** The rectangle component order is
inferred. `0x8008E890` writes four floats and the body then builds
`[f31, f31+f1]` on x and `[f30-f0, f30]` on y, which is consistent with
`(x, y, width, height)` or `(left, top, right, bottom)` depending on the callee's
convention; the *code* is identical either way, only the comment changes. The
verifier should see exactly four float loads at `r1+8..0x14`, the `fadds`/`fsubs`
pair, `0x80101AA0` applied to `other+0xAC` only, and the four ordered compares.
Write `// unknown: 0x8008E890(stage, playerIndex, &mut [f32;4])` and keep the
`cror` polarities as listed.

---

## 7. `0x806329E0` - 264 bytes - `work/ghidra/rel-d_basesNP/export/806329e0_FUN_806329e0.c`

**Body:** `0x806329E0..0x80632AE4` (66 words, `blr` at `0x80632AE4`).
Ledger: absent. `src/game/bases.rs`: no definition; one mention.

```rust
pub unsafe fn fn_806329e0(this: usize)
```

**Frame.** `stwu r1,-48(r1)`; `mfspr r0,LR` / `stw r0,52(r1)`;
`addi r11,r1,32`; `stfd f31,32` + `psq_st f31,40`; `bl 0x802DCD74` at
`0x806329F8` (saves **r27-r31**); `or r31,r3,r3`.
Epilogue `0x80632AC8..0x80632AE4` (`psq_l`, `lfd`, `addi r11,r1,32`,
`bl 0x802DCDC0`, ...).

**Shape.** Search slots 1..3 of a per-slot object array for the first slot whose
probe value reaches the running maximum (initialised to `-1.0`), then push that
value into six render objects.

1. `0x806329FC-0x80632A10` - `f31 = -1.0`; `r28 = this + 4`; `r27 = 1`;
   `r30 = 0x807C0000`.
2. `0x80632A14-0x80632A3C` - probe:
   ```
   o = *(u32*)(r28 + 0xA4);                 // = *(u32*)(this + 4*i + 0xA4)
   if (0x800C9610(o + 0x70, 0) != 0)
       v = *(f32*)(*(u32*)(*(u32*)(o + 0x1F4)) + 8);
   else
       v = 107.0;
   ```
   `0x800C9610(obj, idx)` is a byte reader (`*(u8*)(*(u32*)(obj+0x188) + idx)`),
   so this is a presence test on a per-slot record.
3. `0x80632A40-0x80632A48` - `fcmpo cr0,v,f31`; `cror 2,1,2`; `bne` -> next
   slot. The merged condition branches when `v < f31` (`EQ := GT|EQ`, `bne` =
   "EQ clear" = LT), i.e. **strictly less** values are skipped.
4. `0x80632A4C-0x80632A84` - when `v >= f31`, re-read the *same* slot object with
   the index argument `4*i`:
   ```
   o = *(u32*)(this + 4*i + 0xA4);          // rlwinm r0,r27,2 ; add r3,r31,r0
   r4 = 4*i;                                 // addi r4,r0,0  (r0 = 4*i)
   if (0x800C9610(o + 0x70, r4) != 0)
       f31 = *(f32*)(*(u32*)(*(u32*)(o + 0x1F4)) + 8);
   else
       f31 = 107.0;
   break;
   ```
   Ghidra prints the second probe's `r4` as `0`; the raw word is
   `addi r4,r0,0` with `r0 = r27*4`, so it is `4*i`. Do not "fix" it to `0`.
5. `0x80632A88-0x80632A94` - `r27++`, `r28 += 4`, `while (r27 < 4)`.
6. `0x80632A98-0x80632AC4` - six stores, in this exact order:
   ```
   *(f32*)(*(u32*)(this+0x78) + 0x2EC) = f31;
   *(f32*)(*(u32*)(this+0x88) + 0x2B4) = f31;
   *(f32*)(*(u32*)(this+0x7C) + 0x2EC) = f31;
   *(f32*)(*(u32*)(this+0x8C) + 0x2B4) = f31;
   *(f32*)(*(u32*)(this+0x80) + 0x2EC) = f31;
   *(f32*)(*(u32*)(this+0x90) + 0x2B4) = f31;
   ```

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x806329F8` | `0x802DCD74` | save r27-r31; `r11 = r1+32` |
| `0x80632A20` | `0x800C9610` | `r3 = o+0x70`, `r4 = 0` -> `u8` |
| `0x80632A60` | `0x800C9610` | `r3 = o+0x70`, `r4 = 4*i` -> `u8` |
| `0x80632AD4` | `0x802DCDC0` | restore r27-r31; `r11 = r1+32` |

No virtual calls. Two `psq*` words, `qr0`/`W=0` (one `f31` pair).

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 -1.0` (initial maximum) | `0x807C7C9C` | `lfs f31,31900(r4)` @`0x80632A04` (`r4 = 0x807C0000`) |
| `f32 107.0` (absent-slot value) | `0x807C7CA0` | `lfs f0,31904(r30)` @`0x80632A3C`, `lfs f31,31904(r3)` @`0x80632A80` |
| slot array base | `this+0xA4`, stride 4; the three probed slots are `this+0xA8`, `+0xAC`, `+0xB0` | `lwz r29,164(r28)` @`0x80632A14` (`r28` starts at `this+4`) |
| probe object | `+0x70` | `addi r3,r29,112` @`0x80632A1C` |
| value pointer chain | `+0x1F4` -> `+0` -> `+8` | `lwz r3,500(r29)` @`0x80632A2C` |
| target objects | `this+0x78,0x7C,0x80` at `+0x2EC`; `this+0x88,0x8C,0x90` at `+0x2B4` | `0x80632A98..0x80632AC4` |

**Confidence: high.** The loop, both probe variants, the `cror`-merged
`v < f31` skip and the six stores are literal. The only inference is the meaning
of the cached value (`-1.0` sentinel, `107.0` for an absent slot); keep the
sentinel exactly and do not substitute `0.0`.

---

## 8. `0x8065C100` - 264 bytes - `work/ghidra/rel-d_basesNP/export/8065c100_FUN_8065c100.c`

**Body:** `0x8065C100..0x8065C204` (66 words, `blr` at `0x8065C204`).
Ledger and `src/game/bases.rs`: absent (zero mentions).

```rust
pub unsafe fn fn_8065c100(this: usize)
```

**Frame.** `stwu r1,-64(r1)`; `mfspr r0,LR` / `stw r0,68(r1)`;
`stfd f31,48` + `psq_st f31,56`; `stfd f30,32` + `psq_st f30,40`;
`stw r31,28`; `or r31,r3,r3`. No save-helper. Epilogue `0x8065C1E4..0x8065C204`.

**Shape.** Compute a y coordinate from a moving sub-object
(`this+0x9C20`), a clamped x offset from `this+0x9CC8`, and submit a three-float
record through the embedded object at `this+0x99E4`.

1. `0x8065C12C-0x8065C13C` - `f30 = *(f32*)(this+0xAC) + 1088.0`
   (pool base `0x807E6EC0`, `+0x2C`).
2. `0x8065C140-0x8065C158` - `f31 = 0x800E7E30(this+0x9C20) * 0.5`;
   `f0 = 0x800E7E40(this+0x9C20) + f31`;
   `if (f0 >= f30) f30 = f0;` (`fcmpo` / `bge` to the `fmr`).
3. `0x8065C170-0x8065C190` - `f31 = 0x800E7E20(this+0x9C20)`;
   `f31 = 0x800E80B0(this+0x9C20) - f31`.
4. `0x8065C194-0x8065C1AC` - `f1 = 0x8065C210(*(f32*)(this+0x9CC8),
   &[0x807E6EF0] (-16.0), &[0x807E6EF4] (0.0))`; `f1 = f1 + f31`.
   `0x8065C210` is a clamped lerp: `v < 0` -> `-16.0`, `v > 1` -> `0.0`,
   otherwise `-16.0 + v * 16.0`. Its two bounds are the two pointers, so pass the
   addresses rather than the floats.
5. `0x8065C1B0-0x8065C1D4` - build the record and dispatch:
   ```
   f0 = 7100.0;                        // 0x807C8AF0
   *(f32*)(r1+8)  = f30;
   *(f32*)(r1+0xC) = f1;
   *(f32*)(r1+0x10) = f0;
   vmethod(this+0x99E4, 0xB0)(this+0x99E4, &r1+8, 0, 0);
   ```
   The receiver is the embedded object (`lwzu r12,-26140(r3)` with
   `r3 = this+0x10000`), not a pointer loaded from it.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x8065C140` | `0x800E7E30` | `r3 = this+0x9C20` -> `f1` |
| `0x8065C158` | `0x800E7E40` | `r3 = this+0x9C20` -> `f1` |
| `0x8065C178` | `0x800E7E20` | `r3 = this+0x9C20` -> `f1` |
| `0x8065C188` | `0x800E80B0` | `r3 = this+0x9C20` -> `f1` |
| `0x8065C1A8` | `0x8065C210` | `f1 = *(f32*)(this+0x9CC8)`, `r4 = 0x807E6EF0`, `r5 = 0x807E6EF4` -> `f1` |

**Virtual slots:** `*(u32*)(this+0x99E4) -> +0xB0`, called with
`r3 = this+0x99E4`, `r4 = &r1+8`, `r5 = 0`, `r6 = 0`.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| pool base | `0x807E6EC0` | `addis r4,r0,-32642` + `addi r4,r4,28352` @`0x8065C108`/`0x8065C110` |
| `f32 1088.0` | `0x807E6EEC` | `lfs f0,44(r4)` @`0x8065C134` |
| `f32 -16.0` (lerp low) | `0x807E6EF0` | `addi r4,r4,48` @`0x8065C1A0` |
| `f32 0.0` (lerp high) | `0x807E6EF4` | `addi r4,r4,52` @`0x8065C1A4` |
| `f32 0.5` | `0x807C8AEC` | `lfs f0,-29972(r4)` @`0x8065C14C` (`r4 = 0x807D0000`) |
| `f32 7100.0` | `0x807C8AF0` | `lfs f0,-29968(r3)` @`0x8065C1B4` |
| moving sub-object | `this+0x9C20` | `addis r3,r3,1` + `addi r3,r3,-25568` |
| clamp input | `this+0x9CC8` | `lfs f1,-25400(r3)` @`0x8065C19C` |
| embedded submit object | `this+0x99E4` | `lwzu r12,-26140(r3)` @`0x8065C1D4` |

**Confidence: medium.** Every operand and all five calls are literal; the
inferences are (a) the four `0x800E7E2x/0x800E7E4x/0x800E80B0` accessors' contracts
(they are four different scalar reads on the same sub-object - one of them is a
scale or speed, which is why it is multiplied by `0.5`), and (b) whether the
`+0xAC` base is a y coordinate. Emit
`// unknown: 0x800E7E30/0x800E7E40/0x800E7E20/0x800E80B0(this+0x9C20)` and keep
the `max` as `if (sum >= f30) f30 = sum;` - reversing it changes the result.

---

## 9. `0x8065C260` - 264 bytes - `work/ghidra/rel-d_basesNP/export/8065c260_FUN_8065c260.c`

**Body:** `0x8065C260..0x8065C364` (66 words, `blr` at `0x8065C364`).
Ledger and `src/game/bases.rs`: absent (zero mentions).

```rust
pub unsafe fn fn_8065c260(this: usize)
```

**Frame.** `stwu r1,-64(r1)`; `mfspr r0,LR` / `stw r0,68(r1)`;
`stfd f31,48` + `psq_st f31,56`; `stfd f30,32` + `psq_st f30,40`;
`stw r31,28` / `r30,24`; `r31 = 0x807E6EC0`. No save-helper.
Epilogue `0x8065C340..0x8065C364`.

**Shape.** Early-out unless a moving sub-object is above a base height, then
raise one UI message whose numeric argument is a signed 16-bit angle/distance.

1. `0x8065C290-0x8065C2A0` - `f30 = *(f32*)(this+0xAC) + 1088.0`.
2. `0x8065C2A4-0x8065C2C4` - `f31 = 0x800E7E30(this+0x9C20)`;
   `f0 = 0x800E7E40(this+0x9C20) + f31`;
   `fcmpo cr0,f30,f0`; `cror 2,0,2` (`EQ := LT|EQ`); `bne` -> epilogue.
   `bne` branches on "EQ clear", i.e. `f30 > f0`, so **the body runs only when
   `f30 <= f0`**.
3. `0x8065C2C8-0x8065C2DC` - `f30 = *(f32*)(this+0xB0) + 2624.0`
   (`0x807E7368`).
4. `0x8065C2DC-0x8065C304` - `f31 = 0x800E7E20(this+0x9C20) * 0.5`;
   `f31 = 0x800E80B0(this+0x9C20) - f31`; `f1 = 0x800E7E20(this+0x9C20)`
   (**second** call, its result is used, the first is not).
5. `0x8065C308-0x8065C33C` - `f0 = f31 - f30`; `f0 = f0 + f1`;
   `v = (s16)(s32)fctiwz(f0)`; then
   ```
   p = *(u32*)0x80439D64;
   vmethod(p, 0x2C)(p, 796, v, 0);
   ```
   The `fctiwz` result is stored as a double at `r1+8`, reloaded from `r1+0xC`
   (the low word) and sign-extended with `extsh`, so the argument is an `i16`
   with truncation toward zero.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x8065C2A4` | `0x800E7E30` | `r3 = this+0x9C20` -> `f1` |
| `0x8065C2B4` | `0x800E7E40` | `r3 = this+0x9C20` -> `f1` |
| `0x8065C2DC` | `0x800E7E20` | `r3 = this+0x9C20` -> `f1` (discarded) |
| `0x8065C2F4` | `0x800E80B0` | `r3 = this+0x9C20` -> `f1` |
| `0x8065C304` | `0x800E7E20` | `r3 = this+0x9C20` -> `f1` (used) |

**Virtual slots:** `*(u32*)0x80439D64 -> +0x2C`, called with `r3 = p` (the
singleton itself), `r4 = 796`, `r5 = (i16)`, `r6 = 0`.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| pool base | `0x807E6EC0` | `addis r31,r0,-32642` + `addi r31,r31,28352` @`0x8065C280`/`0x8065C284` |
| `f32 1088.0` | `0x807E6EEC` | `lfs f0,44(r31)` @`0x8065C298` |
| `f32 2624.0` | `0x807E7368` | `lfs f0,1192(r31)` @`0x8065C2D0` |
| `f32 0.5` | `0x807C8AEC` | `lfs f0,-29972(r4)` @`0x8065C2E8` |
| message singleton pointer | `0x80439D64` | `addis r3,r0,-32701` + `lwz r3,-25244(r3)` @`0x8065C30C`/`0x8065C310` |
| message id `796` | immediate | `addi r4,r0,0x31C` @`0x8065C314` |

**Confidence: medium.** The gate polarities, all five float calls, the
`fctiwz`/`extsh` ladder and the dispatch are literal. The unknowns are the
`0x800E7E2x` family's contracts (as in §8) and what message `796` means; the
`0x80439D64` singleton has no shipped bytes. Keep the *second* `0x800E7E20` call
- removing it as "duplicate" changes the value.

---

## 10. `0x806FFB50` - 256 bytes - `work/ghidra/rel-d_basesNP/export/806ffb50_FUN_806ffb50.c`

**Body:** `0x806FFB50..0x806FFC4C` (64 words, `blr` at `0x806FFC4C`).
Ledger and `src/game/bases.rs`: absent (zero mentions).

```rust
pub unsafe fn fn_806ffb50(this: usize) -> usize
```

**Frame.** `stwu r1,-32(r1)`; `mfspr r0,LR` / `stw r0,36(r1)`;
`stw r31,28` / `r30,24` / `r29,20` / `r28,16`; `r28 = this`, `r29 = 0`.
No `psq`. Epilogue `0x806FFC2C..0x806FFC4C` (`or r3,r28,r28` -> returns `this`).

**Shape.** A constructor: base ctor, vtable, one member ctor, one 14-element
array of 64-byte records, an embedded object with a two-step vtable fixup, and
three 14-element arrays of 296-byte records.

1. `0x806FFB70` - `0x80094E80(this)` (base constructor).
2. `0x806FFB74-0x806FFB80` - `r4 = 0x80803AC0`; `*(u32*)(this+0x60) = r4`
   (vtable).
3. `0x806FFB78-0x806FFB84` - `0x80069020(this+0x524)`.
4. `0x806FFB88-0x806FFBAC` - `*(u32*)(this+0x540) = 0`; then
   `0x802DC9A0(this+0x544, 0x80169CD0, 0x80169D20, 0x40, 0x0E)`
   (`__construct_array`, confirmed from `main.dol`: it calls `ctor(ptr, 1)` for
   each element and then `dtor(ptr, -1)` on any partial-construction unwind).
5. `0x806FFBB0-0x806FFBBC` - `*(u32*)(this+0x8C4) = 0x803295D8`;
   `*(u32*)(this+0x8C8) = 0`.
6. `0x806FFBC0-0x806FFBC8` - `*(u32*)(this+0x8CC) = 0`;
   `0x8016A5E0(this+0x8D0)` (embedded object ctor).
7. `0x806FFBCC-0x806FFBDC` - `*(u32*)(this+0x8C4) = 0x80329518` (second vtable
   write to the *same* slot; the order matters).
8. `0x806FFBE0-0x806FFC28` - three identical array constructions:
   ```
   __construct_array(this+0x8FC,  0x8065BF50, 0x80657200, 0x128, 0x0E);
   __construct_array(this+0x192C, 0x8065BF50, 0x80657200, 0x128, 0x0E);
   __construct_array(this+0x295C, 0x8065BF50, 0x80657200, 0x128, 0x0E);
   ```
9. `0x806FFBEC` - `*(u32*)(this+0x8EC) = 0` (between the first and the second
   of the three array constructions - keep the position).
10. `0x806FFC30` - `r3 = this` -> return.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x806FFB70` | `0x80094E80` | `r3 = this` |
| `0x806FFB84` | `0x80069020` | `r3 = this+0x524` |
| `0x806FFBAC` | `0x802DC9A0` | `this+0x544`, `0x80169CD0`, `0x80169D20`, `0x40`, `14` |
| `0x806FFBC8` | `0x8016A5E0` | `r3 = this+0x8D0` |
| `0x806FFBF8` | `0x802DC9A0` | `this+0x8FC`, `0x8065BF50`, `0x80657200`, `0x128`, `14` |
| `0x806FFC10` | `0x802DC9A0` | `this+0x192C`, same |
| `0x806FFC28` | `0x802DC9A0` | `this+0x295C`, same |

No virtual calls, no `psq`.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| vtable | `0x80803AC0` | `addis r4,r0,-32640` + `addi r4,r4,15040` @`0x806FFB74`/`0x806FFB7C` |
| sub-object vtable (base) | `0x803295D8` | `addis r3,r0,-32717` + `addi r3,r3,-27176` @`0x806FFBB0`/`0x806FFBB8` |
| sub-object vtable (derived) | `0x80329518` | same `lis`, `addi -27368` @`0x806FFBCC`/`0x806FFBD4` |
| 64-byte element ctor / dtor | `0x80169CD0` / `0x80169D20` | `addi r4,r4,-25392`, `addi r5,r5,-25312` @`0x806FFB9C`/`0x806FFBA0` |
| 296-byte element ctor / dtor | `0x8065BF50` / `0x80657200` | `addi r4,r30,-16560`, `addi r5,r31,29184` @`0x806FFBE4`/`0x806FFBE8` |
| element counts | `14` (0x0E) | `addi r7,r0,14` at all four `__construct_array` sites |
| element sizes | `0x40`, `0x128` | `addi r6,r0,64` / `addi r6,r0,296` |

**Confidence: high.** Every address, size, count and store is literal, and the
`__construct_array` contract was verified instruction by instruction in
`main.dol`. Implement `0x802DC9A0`-shaped helpers as `abi_call5!` and keep all
four calls - the layout (`0x544..0x8C4`, `0x8C4..0x8CC`, `0x8D0..0x8EC`,
`0x8FC..0x192C`, `0x192C..0x295C`, `0x295C..0x398C`) is exactly
`count * size`.

---

## 11. `0x807241C0` - 256 bytes - `work/ghidra/rel-d_basesNP/export/807241c0_FUN_807241c0.c`

**Body:** `0x807241C0..0x807242BC` (64 words, `blr` at `0x807242BC`).
Ledger and `src/game/bases.rs`: absent (zero mentions).

```rust
pub unsafe fn fn_807241c0(this: usize)
```

**Frame.** `stwu r1,-48(r1)`; `mfspr r0,LR` / `stw r0,52(r1)`;
`stfd f31,32` + `psq_st f31,40`; `stfd f30,16` + `psq_st f30,24`;
`stw r31,12` / `r30,8`; `r31 = 0x807CD188`, `r30 = this`.
No save-helper. Epilogue `0x80724298..0x807242BC`.

**Shape.** Initialise a two-axis emitter pair (a scale-driven x range and a
y range), prime a sub-object at `+0x148`, run a sibling initialiser, then build a
ten-argument effect at `+0x4CC` and start it.

1. `0x807241F0-0x80724210` - `f2 = *(f32*)(r31+0x30)` (= `0.5`);
   `f31 = f2 * *(f32*)(this+0x474)`;
   `f30 = f2 * *(f32*)(this+0x478)`;
   `0x8008C3E0(this+0x148, this, 0x807CD18C /* 0.0 */)`.
2. `0x80724214-0x80724230` - `f0 = *(f32*)(r31+0x2C) /* 1.0 */ + f30`;
   `*(f32*)(this+0x164) = *(f32*)(r31+0x34) /* 0.0 */`;
   `*(f32*)(this+0x168) = f30`; `*(f32*)(this+0x16C) = f31`;
   `*(f32*)(this+0x170) = f0`.
3. `0x80724234` - `0x8008C330(this+0x148)`.
4. `0x80724238-0x8072423C` - `0x807242C0(this)`.
5. `0x80724240-0x8072427C` - the ten-argument builder:
   ```
   0x8007FA40(
       r3  = this+0x4CC,
       r4  = this,
       f1  = -f31,
       f2  = *(f32*)(this+0x478),
       f3  =  f31,
       f4  = 0.0,                 // 0x807CD1BC
       r5  = 0x807246D0,
       r6  = 0x807245F0,
       r7  = 0x807247D0,
       r8  = 1,
       r9  = 0,
       r10 = 0);
   ```
   Note `f1` is `fneg f1,f31` and `f2`/`f3` are *different* sources
   (`this+0x478` and `this+0x474 * 0.5`).
6. `0x80724280-0x80724290` - `*(u8*)(this+0x5AA) = *(u8*)(this+0x38F)`;
   `*(u32*)(this+0x59C) = 0x940`.
7. `0x80724294` - `0x8007F900(this+0x4CC)`.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80724210` | `0x8008C3E0` | `r3 = this+0x148`, `r4 = this`, `r5 = 0x807CD18C` |
| `0x80724234` | `0x8008C330` | `r3 = this+0x148` |
| `0x8072423C` | `0x807242C0` | `r3 = this` |
| `0x8072427C` | `0x8007FA40` | `r3 = this+0x4CC`, `r4 = this`, `f1 = -f31`, `f2 = *(f32*)(this+0x478)`, `f3 = f31`, `f4 = 0.0`, `r5 = 0x807246D0`, `r6 = 0x807245F0`, `r7 = 0x807247D0`, `r8 = 1`, `r9 = 0`, `r10 = 0` |
| `0x80724294` | `0x8007F900` | `r3 = this+0x4CC` |

No virtual calls. Four `psq*` words, `qr0`/`W=0` (`f31`/`f30` saves).

**Constants:**

| value | image address | materialised at |
|---|---|---|
| shared pool base | `0x807CD188` | `addis r31,r0,-32643` + `addi r31,r31,-11896` @`0x807241E0`/`0x807241E4` |
| `f32 1.0` | `0x807CD1B4` | `lfs f0,44(r31)` @`0x80724214` |
| `f32 0.5` | `0x807CD1B8` | `lfs f2,48(r31)` @`0x807241F0` |
| `f32 0.0` | `0x807CD1BC` | `lfs f1,52(r31)` @`0x8072421C`, `lfs f4,52(r31)` @`0x80724258` |
| `f32 0.0` (5th arg of `0x8008C3E0`) | `0x807CD18C` | `addi r5,r31,4` @`0x807241FC` |
| callback triplet | `0x807246D0`, `0x807245F0`, `0x807247D0` | `addi r5,r5,18128`, `addi r6,r6,17904`, `addi r7,r7,18384` @`0x80724264..0x8072426C` |
| state word `0x940` | immediate | `addi r0,r0,0x940` @`0x80724284` |

**Confidence: medium.** Every operand and all five calls are literal; the
inference is `0x8007FA40`'s contract (a seven-integer/four-float effect builder
whose `r5/r6/r7` are three code addresses). Emit
`// unknown: 0x8007FA40(effect, this, f32, f32, f32, f32, cb1, cb2, cb3, 1, 0, 0)`
and keep `f1 = -f31` as a negation rather than folding it into the call.

---

## 12. `0x807811D0` - 256 bytes - `work/ghidra/rel-d_basesNP/export/807811d0_FUN_807811d0.c`

**Body:** `0x807811D0..0x807812CC` (64 words, `blr` at `0x807812CC`).
Ledger: absent as a row; mentioned in the evidence of `fn_8077e6d0` (line 4467)
and `fn_8077fa10` (line 4480). `src/game/bases.rs`: no definition; two mentions.

```rust
pub unsafe fn fn_807811d0(this: usize) -> u32
```

**Frame.** `stwu r1,-32(r1)` only. **No `mfspr LR` and no LR save** - this is a
leaf. `r8 = 0x807CF608`. Epilogue `0x807812C8..0x807812CC`
(`addi r1,r1,32`; `blr`).

**Shape.** A pure angular predicate: convert three signed 16-bit angle fields to
degrees, wrap each into `[0, 360)`, and answer "is the opponent more than 45
degrees away" with a state-dependent second test.

1. `0x807811D4-0x807811FC` - constants and fields:
   `r6 = 0x43300000`; `r5 = *(u32*)0x80429EA0`; `r4 = (s16)*(this+0x1E6)`;
   `r7 = (s16)*(r5+0x102)`; `r0 = (s16)*(this+0x1EC)`; then
   `r4 ^= 0x8000`, `r7 ^= 0x8000`, `r0 ^= 0x8000` (`xoris`).
2. `0x80781208-0x8078124C` - three int-to-double-to-single conversions with the
   `0x43300000_80000000` bias (`lfd f4,400(r8)`), each multiplied by
   `*(f32*)0x8042DCD8` (= `360/65536`):
   ```
   f3 = (f32)((s16)*(r5+0x102)) * (360.0/65536.0);   // opponent
   f1 = (f32)((s16)*(this+0x1E6)) * (360.0/65536.0);
   f2 = (f32)((s16)*(this+0x1EC)) * (360.0/65536.0);
   ```
   The doubles are built as `0x43300000_<value ^ 0x8000>` at `r1+8/+0x10/+0x18`
   and biased by `2^52 + 2^31`, so the result is the exact signed integer as a
   double. The multiply is `fmuls` (single precision).
3. `0x80781250-0x80781280` - wrap each into `[0, 360)`:
   ```
   if (f3 < 0.0) f3 += 360.0;
   if (f1 < 0.0) f1 += 360.0;
   if (f2 < 0.0) f2 += 360.0;
   ```
   `fcmpo` + `bge` (BO=4) means "skip when not less than 0"; the added constant
   is `*(f32*)0x807CF7DC`.
4. `0x80781284-0x80781298` - `f0 = fabs(f3 - f1)`; `frsp f0,f0`;
   `if (f0 > 45.0) return 1;` (`bgt`, BO=12/BI=1; `45.0` is `0x807CF7E0`).
5. `0x8078129C-0x807812A4` - `if (*(u32*)(this+0x1D0) != 6) return 0;`.
6. `0x807812A8-0x807812C0` - `f0 = fabs(f3 - f2)`; `frsp`;
   `if (f0 > 45.0) return 1;` else `return 0`.

**Direct callees:** none. Leaf, no `bl`, no `bctr`, no `psq`, no stack arguments.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| pool base | `0x807CF608` | `addis r8,r0,-32643` + `addi r8,r8,-2552` @`0x807811DC`/`0x807811E4` |
| `f64 2^52 + 2^31` = `0x4330000080000000` | `0x807CF798` | `lfd f4,400(r8)` @`0x80781208` |
| `f32 0.0` (wrap test) | `0x807CF698` | `lfs f0,144(r8)` @`0x80781220`, `0x8078125C`, `0x80781270` |
| `f32 360.0` (wrap) | `0x807CF7DC` | `lfs f0,468(r8)` @`0x80781254`, `0x80781268`, `0x8078127C` |
| `f32 45.0` (tolerance) | `0x807CF7E0` | `lfs f1,472(r8)` @`0x80781288` |
| `f32 360/65536` = `0x3BB40000` | `0x8042DCD8` | `lfs f2,-9000(r5)` @`0x80781210` (`r5 = 0x80430000`) |
| opponent object pointer | `0x80429EA0` | `lwz r5,-24160(r4)` @`0x807811E0` |
| own angle A | `this+0x1E6` | `lha r4,486(r3)` @`0x807811E8` |
| opponent angle | `[0x80429EA0]+0x102` | `lha r7,258(r5)` @`0x807811EC` |
| own angle B | `this+0x1EC` | `lha r0,492(r3)` @`0x807811F4` |
| state word | `this+0x1D0` | `lwz r0,464(r3)` @`0x8078129C` |

`0x8042DCD8` is **not** zero: `main.elf` PH11 (`p_offset 0x350200`,
`p_vaddr 0x8042B080`, filesz 0x4B60) holds `0x3BB40000`, exactly
`360.0 / 65536.0`. That is the conversion from the 16-bit angle unit used by
`+0x1E6`/`+0x1EC`/`+0x102` to degrees. `0x80429EA0` is `main.dol` BSS (PH10,
`0x80429BC0`, filesz 0) and is filled at run time.

**Confidence: high.** All 64 words decode, there is no call and no indirection,
and all five constants were read from the shipped images. The only inference is
the field *names* (`+0x1E6`/`+0x1EC` are two yaw-like angles); the arithmetic is
exact. Keep `frsp` before the compare (it rounds the `fabs` result to `f32`) and
keep the two separate `fabs` chains - the second uses `f2`, not `f1`.

---

## Ready vs conditional

| address | verdict | confidence | blocker, if any |
|---|---|---|---|
| `0x806FFB50` | **ready** | high | none - constructor, every address and count literal |
| `0x806B39B0` | **ready** | high | none - three vtable slots with explicit receivers |
| `0x80643D40` | **ready** | high | slot table `0x80354E60` is `main.dol` BSS: read at run time |
| `0x806329E0` | **ready** | high | `0x800C9610` read as a byte presence test |
| `0x807811D0` | **ready** | high | none - leaf, no calls, all constants located |
| `0x80780D00` | **ready** | medium | `0x800F69F0` eight-argument contract |
| `0x80711590` | **ready** | medium | `0x807103D0` contract; `0x8082F788` has no shipped bytes |
| `0x8065C100` | **ready** | medium | four `0x800E7E2x/4x` accessor contracts |
| `0x8065C260` | **ready** | medium | same accessor family; message `796` meaning |
| `0x807241C0` | **ready** | medium | `0x8007FA40` ten-argument contract |
| `0x806FCDF0` | **conditional** | medium | return type (`-> u32` = 1) and the purpose of the `rand(1000)` loop are inferred; the body itself is literal |
| `0x807485D0` | **conditional** | medium | the four floats written by `0x8008E890` have no provable component order |

**10 ready, 2 conditional.** The two conditional bodies are fully decoded and
implementable; the blocker is a signature/comment decision, not a missing byte.
A translation lane can take all twelve today, marking the two conditional ones
`medium` in the ledger and carrying the `// unknown:` notes above.

Note that the parallel Wave 133 translation lane had already taken
`0x80780D00`, `0x80711590`, `0x80643D40` and `0x806B39B0` by the time this
document was written (see the concurrency note under "Verification performed").
Their bodies can be checked against §1-§4 word for word; the remaining eight are
still open.

## Verification performed

**Concurrency note (2026-11-12 21:07, at write time).** The Wave 133 translation
lane for `game::bases` was running in parallel with this read-only lane. At the
moment this document was written, that lane had just added bodies for four of
the twelve - `0x80780D00`, `0x80711590`, `0x80643D40` and `0x806B39B0` - to
`src/game/bases.rs`, with matching rows in `ledger/agents/wave133-bases.csv`.
Those four were unclaimed when the pool was computed and are still absent from
`ledger/functions.csv` (which the coordinator merges later). The other eight are
unclaimed in every file. Every "Ledger: absent" line below therefore means
"absent from `ledger/functions.csv` and from every spec heading", not "absent
from the work-in-progress Rust file at this minute".

- Candidate pool recomputed from `docs/ANALYSIS_WAVE127_BASES.md`: 121 class-B
  rows, 63 `## N. \`0xADDR\`` headings across
  `docs/SPEC_WAVE128_BASES.md`..`docs/SPEC_WAVE132_BASES.md`, 58 addresses left
  (9 736 bytes). The twelve taken here are the twelve largest.
- `ledger/functions.csv` checked **row-based**: `^game::bases,<addr>,` matches
  zero lines for all twelve. The three textual mentions are listed in
  "Selection rule".
- `src/game/bases.rs` contains no `fn_<addr>` definition for any of the twelve.
- Every byte count equals the `functions.tsv` `size` column, and every export
  file exists at the cited path.
- Extent check: for all twelve, `u32(entry + size - 4) == 0x4E800020` and no
  `functions.tsv` row starts inside the body. Total 3 180 bytes / 795 words.
- Paired-single audit: 26 `psq*` words, all `qr0` (`I = 0`) with `W = 0`, all
  prologue spill/reload pairs for `f31/f30/f29/f28`. No `mtspr` to `GQRn`; the
  only `mtspr` operands are `LR` and `CTR`.
- `cror` polarities resolved from the raw words
  (`0x4C401382` = `cror 2,0,2`, `0x4C411382` = `cror 2,1,2`), not from the
  Ghidra exports.
- Cross-artifact constants read directly: `0x8042DCD8` (main.elf PH11) =
  `0x3BB40000`; `0x807CF7C4` = `{4, 9, 10}`; `0x807CF674` = 70.0;
  `0x807CF7B0` = 0.5; `0x807CF7DC` = 360.0; `0x807CF7E0` = 45.0;
  `0x807C7C9C` = -1.0; `0x807C7CA0` = 107.0; `0x807E6EEC` = 1088.0;
  `0x807E6EF0` = -16.0; `0x807E6EF4` = 0.0; `0x807E7368` = 2624.0;
  `0x807C8AEC` = 0.5; `0x807C8AF0` = 7100.0; `0x807CD1B4` = 1.0;
  `0x807CD1B8` = 0.5; `0x807CD1BC` = 0.0; `0x80805EA0` = 29;
  `0x808063B0` = 0.0; `0x808063DC` = 0.5.
- `0x802DC9A0` confirmed as `__construct_array(ptr, ctor, dtor, size, count)`
  from `work/dol/main.elf`; `0x800B2ED0` confirmed as a 12-byte thunk into the
  32-bit LCG `0x80161460` (returns `(state * n) >> 32`); `0x8005FEF0` confirmed
  as the active-player counter; `0x800C9610` confirmed as a byte fetch at
  `*(obj+0x188) + idx`.
- No file in the repository was written except this one; no merge or check tool
  was run.

## Hand-off

The 58-address pool is now 46 (6 556 bytes). The next twelve in the same order
are `0x80608DF0` (248), `0x807268C0` (248), `0x80780A90` (240),
`0x8069A6E0` (236), `0x80643470` (228), `0x8068E440` (228), `0x80747B30` (224),
`0x8069F560` (208), `0x8069F9D0` (208), `0x806B4F90` (208), `0x80781B60` (208)
and `0x80642D20` (204). `0x8069A6E0` is the only one of those with a real
data-side `qr0` load and needs the `psq_l ...,w0` read spelled out; the rest are
integer bodies plus `f31` spills.
