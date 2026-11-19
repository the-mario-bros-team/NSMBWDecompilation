# Wave 132 - `game::bases` - implementation spec for the next 12 class-B addresses

Read-only lane. The only file written is this one. The throwaway PowerPC decoder
(`%TEMP%\opencode\ppc132.js`) and the ELF/section readers live outside the
repository; no Rust file, no `ledger/functions.csv` row and no existing document
was modified. No merge or check tool was run.

## Why these twelve

`docs/ANALYSIS_WAVE127_BASES.md` reclassified 121 `d_basesNP` skips as class B
("reconstructible now from the raw stream"). Waves 128-131 carry 51 section
headings over that pool: 36 distinct class-B bodies plus 15 overflow/duplicate
sections. Of the 121 class-B rows exactly **70** are still neither claimed nor
present in the ledger, totalling **13 388** bytes. This lane takes the twelve
largest of those, ordered by `functions.tsv` size descending and then by address
ascending:

| # | address | bytes | words | export file | verdict |
|--:|---:|---:|---:|---|---|
| 1 | `0x80761510` | 340 | 85 | `80761510_FUN_80761510.c` | ready (high) |
| 2 | `0x8078B1D0` | 340 | 85 | `8078b1d0_FUN_8078b1d0.c` | ready (medium) |
| 3 | `0x806B7890` | 336 | 84 | `806b7890_FUN_806b7890.c` | conditional (medium) |
| 4 | `0x806B7690` | 312 | 78 | `806b7690_FUN_806b7690.c` | ready (medium) |
| 5 | `0x806A23B0` | 308 | 77 | `806a23b0_FUN_806a23b0.c` | ready (high) |
| 6 | `0x806DF630` | 308 | 77 | `806df630_FUN_806df630.c` | ready (medium) |
| 7 | `0x80722180` | 296 | 74 | `80722180_FUN_80722180.c` | ready (medium) |
| 8 | `0x80780BD0` | 292 | 73 | `80780bd0_FUN_80780bd0.c` | conditional (medium) |
| 9 | `0x80780E20` | 292 | 73 | `80780e20_FUN_80780e20.c` | ready (medium) |
| 10 | `0x80702AA0` | 276 | 69 | `80702aa0_FUN_80702aa0.c` | ready (high) |
| 11 | `0x80702BC0` | 276 | 69 | `80702bc0_FUN_80702bc0.c` | ready (high) |
| 12 | `0x80724460` | 276 | 69 | `80724460_FUN_80724460.c` | ready (medium) |

Together **3 652 bytes** (913 words). Every export path is exactly
`work/ghidra/rel-d_basesNP/export/<file>` relative to the repository root, and
every `bytes` value matches the `size` column of that directory's
`functions.tsv` verbatim.

### Selection rule

The candidate set is: **class B in wave 127**, **no row in
`ledger/functions.csv` whose address column equals the address**, and **no
`## N. \`0xADDR\`` heading in waves 128-131**. The ledger test is *row-based*
(comma field 2 of each line), not a substring search: five of the twelve are
named inside the free-text evidence column of unrelated rows, and a substring
test would silently drop them.

| address | ledger mentions | where |
|---|---:|---|
| `0x80761510` | 1 | evidence of `fn_80761310` (ledger line 4254) |
| `0x806B7690` | 2 | evidence of `fn_806b7ad0` (2919) and an unrelated row |
| `0x80780BD0` | 3 | evidence of `fn_80780bb0` (4465) and two others |
| `0x80780E20` | 1 | evidence of `fn_8077e760` (4443) |
| `0x80724460` | 1 | evidence of `fn_80724020` (3732) |

As recorded at `docs/SPEC_WAVE129_BASES.md:1464-1468` and reused at
`docs/SPEC_WAVE130_BASES.md:1283-1286` and `docs/SPEC_WAVE131_BASES.md:40-59`,
**a textual mention is not a claim** - only a section heading is.

### Ties, and what is left

Four bodies in the pool are exactly 276 bytes and four are exactly 268; the
address-ascending tiebreak above is the only reason
`0x80702AA0`/`0x80702BC0`/`0x80724460` are in this batch and `0x80780D00` (276)
is not. The next six, in the same order, are:

| address | bytes | note |
|---|---:|---|
| `0x80780D00` | 276 | fully decoded already for this lane; it is the `0x800F69F0` "place along path" wrapper |
| `0x80711590` | 272 | fully decoded already; pre/post position comparison plus `+0xD4` message |
| `0x80643D40` | 268 | fully decoded already; 7-slot clear plus four pane flags |
| `0x806B39B0` | 268 | fully decoded already; returns the 4-bit "occupied seat" mask |
| `0x806FCDF0` | 268 | not opened |
| `0x807485D0` | 268 | not opened |

Nothing is lost by the tiebreak: the four "already decoded" rows are summarised
in §13 so a later lane does not have to redo the register walk.

## Method

**Address mapping (re-verified, not copied).** `d_basesNP.elf` has six `PT_LOAD`
headers. The first is `p_offset = 0x10F0`, `p_vaddr = 0x806000F0`, so

```
file_offset = 0x10F0 + (address - 0x806000F0) = (address - 0x80600000) + 0x1000
```

The other five are `0x807C60F4` (1 276 B), `0x807C65F0` (12 B), `0x807C6600`
(42 492 B), `0x807D0C00` (335 032 B) and the zero-fill segment at `0x80822900`
(`filesz = 0`, `memsz = 74 884`). Every constant below was read at
`p_offset + (addr - p_vaddr)`; the `+0x1000` shortcuts in
`docs/REL_ELF_MAPPING.md` are correct for this REL but were not assumed.

**Extent check.** For all twelve bodies the reachable span equals the TSV size
exactly: the last word of every range is `0x4E800020` (`blr`) at
`entry + size - 4`, and no TSV row starts inside a body. No over-read, no
`halt_baddata`, no jump table.

**Decoder corrections that changed the reading of this batch.** The wave-131
notes warned about `rlwinm` masks; four further traps materialised here and are
worth recording because three of them flip a predicate:

* **`bc` BO/BI, not `bo & 0x14`.** `BO = bits 6-10`, `BI = bits 11-15`, and the
  branch flavour is `BO & 0x10` (test CR?) then `BO & 0x08` (branch if the bit is
  **set**). A decoder that tests `(bo & 0x14) == 0xC` prints `bne` for `0x4182`
  (which is `beq`, BI = 2) and inverts every `if` in the batch. `0x4182` = `beq`,
  `0x4082` = `bne`, `0x4180` = `blt`, `0x4080` = `bge`.
* **A-form float ops take three operands.** `fmuls`/`fadds`/`fsubs`/... are
  `fD,fA,fC`; the `frB` field is not an operand. Ghidra's
  `fmuls f0,f2,f0,f0` rendering in `0x80780D00` loses one factor and turns
  `70.0 * 0.5` into `70.0 * 0.5 * 0.5`. The difference is 35.0 vs 17.5.
* **`cror 2,0,2` after `fcmpo` is a range merge, not a no-op.** It executes
  `CR0.EQ := CR0.LT | CR0.EQ`, so a following `bne` (branch if EQ clear) means
  "branch when `fD > fB`". This is how `0x806A23B0`'s `<= 0` guard is built.
* **A dead `mfcr` + `rlwinm` pair survives in `0x80780E20`.** `0x80780EE0`
  `mfcr r0` / `0x80780EE4` `rlwinm r0,r0,2,31,31` writes `r0`, which the next
  instruction (`lha r0,258(r3)`) overwrites; the `beq` at `0x80780EE8` reads
  `CR0` from the preceding `fcmpo`. Emit the comparison, drop the pair.

**Paired-single audit.** Across the 913 words of the twelve bodies there are
**56 `psq*` words**: 28 `psq_st` prologue saves and 28 `psq_l` epilogue reloads.
Every one is `qr0` (`I = 0`) with `W = 0`, and every one is adjacent to a
`stfd`/`lfd` of the same register. The saved set is `f31, f30, f29, f28, f27,
f26`. Pair counts: one each in `0x80761510`, `0x806B7690`, `0x80722180`,
`0x80780BD0`, `0x80724460`; two in `0x806DF630`; three in `0x8078B1D0`,
`0x806B7890` and `0x806A23B0`; six in `0x80702AA0` and `0x80702BC0`; none in
`0x80780E20`. **There is no data-side quantisation anywhere in this batch and no
`mtspr` to a `GQRn`**: the only `mtspr` operands are `LR` (spr 8) and `CTR`
(spr 9). `0x80780E20` is the only body of the twelve with zero `psq` words (in
§13, `0x80643D40` and `0x806B39B0` are the other two). All twelve were rejected
by the wave-114/115/118/121/122/124/125 "`unaff_GQR0`" bullets; the rejection is
a decompiler artefact in every case.

**Compiler save chain, re-decoded from `work/dol/main.elf`.** The three members
used here were verified instruction by instruction (each is a straight run of
`stw rN,-off(r11)` ending in `blr`, and none writes `r3..r13` or `LR`):

| save stub | saves | matching restore | used by |
|---|---|---|---|
| `0x802DCD68` | r24-r31 | `0x802DCDB4` | `0x806B7890`, `0x80702AA0`, `0x80702BC0` |
| `0x802DCD70` | r26-r31 | `0x802DCDBC` | `0x80761510`, `0x806B39B0` (see §13) |
| `0x802DCD74` | r27-r31 | `0x802DCDC0` | `0x806DF630`, `0x80722180`, `0x806B7690` |

`r11` must be `r1 + frame_size`; each body sets it immediately before the call.
This is what makes the receivers recoverable: after the `bl`, entry `r3` is
still the `this` pointer, and every body copies it into the first callee-saved
register on the next instruction. In `0x80761510` the copy (`or r26,r3,r3` at
`0x8076152C`) happens *after* the helper returns, which is why Ghidra printed
`void FUN_80761510(void)`.

**House style used below.** `pub unsafe fn fn_XXXXXXXX(this: usize)`,
`platform::read_u32/read_i32/read_f32/read_u8/read_u16`,
`platform::write_*`, `abi_call1!..abi_call8!`, `vmethod0(rcv, slot)`,
`ptr_at(addr)`, addresses written `0x8076_1510`, field offsets as `usize`
literals, `// unknown:` on the first use of every address-only target or
external cell.

---

## 1. `0x80761510` - 340 bytes - `work/ghidra/rel-d_basesNP/export/80761510_FUN_80761510.c`

**Body:** `0x80761510..0x80761664` (85 words, `blr` at `0x80761660`).
Ledger: absent as a row; mentioned only in the evidence of `fn_80761310`
(ledger line 4254). `src/game/bases.rs`: no definition; doc-comment mentions at
lines 1455, 139423 and 139472 ("the C prints a stray `1`").

```rust
pub unsafe fn fn_80761510(this: usize)
```

**Frame.** `stwu r1,-64(r1)`; `mfspr r0,LR` / `stw r0,68(r1)`;
`addi r11,r1,48`; `stfd f31,48(r1)` + `psq_st f31,56(r1),0,0`;
`bl 0x802DCD70` at `0x80761528` (saves **r26-r31**, returns with `r3` intact);
`or r26,r3,r3` at `0x8076152C` makes **r26 = this**.
Epilogue `0x80761644..0x80761660`: `addi r11,r1,48`, `psq_l f31,56(r1)`,
`lfd f31,48(r1)`, `bl 0x802DCDBC`, `lwz r0,68(r1)`, `mtspr LR,r0`,
`addi r1,r1,64`, `blr`.

**Shape.** A "collect the last ring-burst" predicate over the whole actor list,
followed by a colour/scale pass over every matching node.

1. `0x8076152C-0x80761538` - `found = false` (`r29 = 0`); `i = 0` (`r30 = 0`);
   `r28 = 0x80430000` (the `lbz` base).
2. `0x8076153C-0x807615A4` - `do { } while (i < 192)` (`cmpi cr0,r30,192` /
   `blt` at `0x807615A4`). Per iteration:
   * `o = 0x800F9BE0(638, i)` (`li r3,638` / `or r4,r30,r30`).
   * `if (o != 0 && (*(u32*)(o + 4) & 0xFF) != 40)` - the mask is
     `rlwinm r0,r0,0,24,31` at `0x80761558` and the compare is `cmpi ...,40`; the
     two tests are separate branches to the same skip label `0x8076159C`.
   * `if (0x800FD7E0(o) == 1)` -> `found = true` (`li r29,1` at `0x80761598`).
   * `else if (*(u8*)0x8042A24D == 7 && (*(u32*)(o + 4) & 0xFF) == 23 &&
    0x80791CE0(o) != 0)` -> `found = true`.
     The `23` compare re-loads `4(r31)` (not `4(r3)`), so transcribe two
     independent loads.
3. `0x807615A8` - `if (!found)` -> epilogue (`beq 0x80761644`).
4. `0x807615B0-0x807615D8` - `node = 0x80162D50(638, 0)`;
   `f31 = *(f64*)0x807CEDD8` = `0x4330000000000000` (2^52) - **dead**, `f31` is
   only spilled and reloaded, never used;
   `r28 = 0x807CEDB0`; `r29 = 0x807D0000`; `r30 = 0x8000003C`
   (`li r0,60` / `xoris r30,r0,0x8000`); `r31 = 0x43300000`.
5. `0x807615E0-0x80761640` - `while (node != 0)` (`cmpi cr0,r27,0` / `bne` at
   `0x80761640`):
   * `if (this != 0)` (`cmpi cr0,r26,0` / `beq` at `0x807615E8` ->
     the *other* arm): `0x80166310(f1 = [0x807CEDB4] (1.0), r3 = node + 0x20C,
     r4 = 0)`.
   * `else`: `0x80166310(f1 = [0x807CEDD0] (0.0), r3 = node + 0x20C, r4 = 0)`
     and then `0x801662F0(f1 = 0.0, r3 = node + 0x20C, r4 = 0)`.
     The `0.0` is built from the ladder at `0x80761610-0x80761624`:
     `stw r30,12(r1)` / `stw r31,8(r1)` / `lfd f0,8(r1)` / `fsubs f1,f0,f0`.
     Both operands are the *same* register, so the `0x43300000_8000003C` word
     and its `60.0` payload are dead: the argument is exactly `0.0`.
   * `node = 0x80162D50(638, node)` (`or r4,r27,r27` at `0x8076162C`).

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80761528` | `0x802DCD70` | save r26-r31; `r11 = r1+48` |
| `0x80761544` | `0x800F9BE0` | `r3 = 638` (0x27E), `r4 = i` -> object or 0 |
| `0x80761564` | `0x800FD7E0` | `r3 = o` (r3 still holds the object) -> `u32` |
| `0x8076158C` | `0x80791CE0` | `r3 = o` -> `u32` |
| `0x807615B8` | `0x80162D50` | `r3 = 638`, `r4 = 0` -> first node |
| `0x807615F8` | `0x80166310` | `f1 = 1.0` or `0.0`, `r3 = node+0x20C`, `r4 = 0` |
| `0x80761628` | `0x801662F0` | `f1 = 0.0`, `r3 = node+0x20C`, `r4 = 0` |
| `0x80761634` | `0x80162D50` | `r3 = 638`, `r4 = node` -> next node |
| `0x80761650` | `0x802DCDBC` | restore r26-r31; `r11 = r1+48` |

No virtual calls, no `psq` data ops (one `psq_st`/`psq_l` pair for `f31`).

**Constants:**

| value | image address | materialised at |
|---|---|---|
| resource type id `638` | immediate | `li r3,0x27E` @`0x80761540`, `0x807615B0`, `0x80761630` |
| filter word `40` / `23` | immediates | `cmpi` @`0x8076155C`, `0x80761588` |
| loop bound `192` | immediate | `cmpi cr0,r30,0xC0` @`0x807615A0` |
| stage byte | `0x8042A24D` | `lbz r0,-23987(r28)` @`0x80761570` (main.dol) |
| `f32 1.0` | `0x807CEDB4` | `lfs f1,4(r28)` @`0x807615EC` |
| `f32 0.0` | `0x807CEDD0` | `lfs f1,32(r29)` @`0x80761600` |
| `f64 2^52` = `0x4330000000000000` | `0x807CEDD8` | `lfd f31,-4648(r4)` @`0x807615C8` |

**Confidence: high.** All 85 words decode, the only branches are the four
literal guards, and the only external dependency is the node list built by
`0x80162D50`. Add `// unknown: 0x80166310(node+0x20C, f32)` and
`// unknown: 0x801662F0(node+0x20C, f32)` for the two setters.

---

## 2. `0x8078B1D0` - 340 bytes - `work/ghidra/rel-d_basesNP/export/8078b1d0_FUN_8078b1d0.c`

**Body:** `0x8078B1D0..0x8078B324` (85 words, `blr` at `0x8078B320`).
Ledger: absent (no mention anywhere in the CSV). `src/game/bases.rs`: no
definition; one comment at line 139472.

```rust
pub unsafe fn fn_8078b1d0(this: usize, index: u32, a: f32, b: f32, c: f32)
```

**Argument map** (verified from both in-REL call sites, `0x8078A800` and
`0x8078A8C4`): `r3 -> this`, `r4 -> index` (0 and 1 respectively),
`f1 -> a` (`fmr f29,f1`), `f2 -> b` (`fmr f30,f2`), `f3 -> c` (`fmr f31,f3`).
At `0x8078A800` all three floats are `[0x807CFB60]` = 0.0; at `0x8078A8C4`
`a = [0x807CFB64]` = 5.0 and `b = c = 0.0`.

**Frame.** `stwu r1,-80(r1)`; `mfspr r0,LR` / `stw r0,84(r1)`;
three `stfd`+`psq_st` pairs (`f31` 64/72, `f30` 48/56, `f29` 32/40);
`stw r31,28` / `r30,24` / `r29,20` / `r28,16`. No save-helper.
`r31 = 56 * index` (`mulli r31,r4,56`), `r29 = index`, `r28 = this`.
Epilogue `0x8078B2EC..0x8078B320`.

**Shape.** Two identical "emitter pair + host block" groups 0xA8 bytes apart,
one conditional spawn, one conditional tail call into the sibling
`0x80776C80`.

1. `0x8078B220-0x8078B254` - **group A**, at `E = this + 56*index`:
   * `0x801680E0(r3 = E+0x4BC, f1 = b)`  (`0x8078B22C`, `f1` set at `0x8078B200`)
   * `0x80168FE0(r3 = E+0x4BC, f1 = c)`  (`0x8078B238`)
   * `0x80168520(r3 = this+0x1A4, r4 = 0)` (`0x8078B244`)
   * `0x80169F80(r3 = this+0x1A4, r4 = E+0x4BC, f1 = a)` (`0x8078B254`)
2. `0x8078B258-0x8078B290` - **group B**, identical with `E+0x564` and
   `this+0x610`.
3. `0x8078B294-0x8078B2C8` - `if (b > [0x807CFB60] (0.0))` (`fcmpo` at
   `0x8078B29C`, `ble` past at `0x8078B2A0`):
   `r3 = [0x8042A26C]`; `0x80103410(r3, r4 = 25, r5 = this+0x610,
   r6 = 0x8081973C "broom", r7 = 0, r8 = 0)`; `*(u32*)(this+0x660) = r3`.
4. `0x8078B2CC-0x8078B2E8` - `if (*(u32*)(this+0x448) != 0)`
   (`cmpi`/`beq` at `0x8078B2D4`):
   `0x80776C80(f1 = a, f2 = b, f3 = c, r4 = this+0x448, r5 = index - 1)`
   (`addi r4,r29,-1` at `0x8078B2DC`).

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x8078B22C` | `0x801680E0` | `r3 = this+56*i+0x4BC`, `f1 = b` |
| `0x8078B238` | `0x80168FE0` | `r3 = this+56*i+0x4BC`, `f1 = c` |
| `0x8078B244` | `0x80168520` | `r3 = this+0x1A4`, `r4 = 0` |
| `0x8078B254` | `0x80169F80` | `r3 = this+0x1A4`, `r4 = this+56*i+0x4BC`, `f1 = a` |
| `0x8078B268` | `0x801680E0` | `r3 = this+56*i+0x564`, `f1 = b` |
| `0x8078B274` | `0x80168FE0` | `r3 = this+56*i+0x564`, `f1 = c` |
| `0x8078B280` | `0x80168520` | `r3 = this+0x610`, `r4 = 0` |
| `0x8078B290` | `0x80169F80` | `r3 = this+0x610`, `r4 = this+56*i+0x564`, `f1 = a` |
| `0x8078B2C4` | `0x80103410` | `r3 = [0x8042A26C]`, `r4 = 25`, `r5 = this+0x610`, `r6 = 0x8081973C`, `r7 = 0`, `r8 = 0` |
| `0x8078B2E8` | `0x80776C80` | `f1 = a`, `f2 = b`, `f3 = c`, `r4 = *(u32*)(this+0x448)`, `r5 = index-1` |

No virtual calls. Six `psq*` words, all `qr0`/`W=0`, all `f31/f30/f29` spills.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 0.0` (spawn gate) | `0x807CFB60` | `lfs f0,-1184(r3)` @`0x8078B298` |
| `f32 5.0` (call-site arg) | `0x807CFB64` | `lfs f1,-1180(r4)` @`0x8078A89C` (not in this body) |
| spawner handle | `0x8042A26C` | `lwz r3,-23956(r3)` @`0x8078B2AC` (main.dol) |
| resource name | `0x8081973C` "broom" | `addi r6,r6,-26820` @`0x8078B2B4` |
| per-slot stride | `56` (0x38) | `mulli r31,r4,0x38` @`0x8078B208` |

**Confidence: medium.** Every operand and both branches are literal; the two
things that stay unresolved are the *semantics* of the `a/b/c` triple (position
vs. scale vs. colour - the call sites pass either all-zero or `a = 5.0`) and the
contracts of `0x801680E0`/`0x80168FE0`/`0x80168520`/`0x80169F80`, which are not
visible in this REL. Keep them as `abi_call` with `// unknown:` notes; do not
invent a `Vec3` type. The `index - 1` passed to `0x80776C80` is an `u32`
wrap-around (`0 - 1` = `0xFFFFFFFF`) and must be kept as `index.wrapping_sub(1)`.

---

## 3. `0x806B7890` - 336 bytes - `work/ghidra/rel-d_basesNP/export/806b7890_FUN_806b7890.c`

**Body:** `0x806B7890..0x806B79E0` (84 words, `blr` at `0x806B79DC`).
Ledger: absent. `src/game/bases.rs`: no definition; comments at lines 32285 and
33108 ("`0x806B7340`, `0x806B7690`, `0x806B7890` and `0x806B9800` are still
rejected").

```rust
pub unsafe fn fn_806b7890(this: usize)
```

**Frame.** `stwu r1,-112(r1)`; `mfspr r0,LR` / `stw r0,116(r1)`;
`addi r11,r1,64`; `stfd f31,96` + `psq_st f31,104`, `f30` 80/88, `f29` 64/72;
`bl 0x802DCD68` at `0x806B78B8` (saves **r24-r31**); `or r24,r3,r3`.
Epilogue `0x806B79B0..0x806B79DC` (`addi r11,r1,64`, three `psq_l`/`lfd` pairs,
`bl 0x802DCDB4`, ...).

**Shape.** A four-slot "ring burst" updater. Slots 0-1 retry an angle probe up to
three times and then submit a 2-value record; slots 2-3 build a record from a
name and constant and submit it. All four submit through
`*(vtable + 0xB0)`.

1. `0x806B78BC-0x806B78E4` - pool: `f30 = [0x807CAB48]` (0.0),
   `f31 = [0x807CAB50]` (16.0); `r29 = r24` (data cursor, +12/iter),
   `r28 = this + 0x14AC` (+12/iter), `r27 = this + 0x104C` (+276/iter),
   `r26 = 0` (slot), `r30 = 0x807F0000`, `r31 = 0x80370000`.
2. Loop head `0x806B78E8`, back edge `0x806B79AC` (`cmpi cr0,r26,4` / `blt`).
   * `if (slot <= 1)` (`cmpli cr0,r26,1` / `bgt 0x806B7958`):
     - `f29 = *(f32*)(r29 + 0x14B0)`; `r25 = 0`.
     - `do { ... } while (r25 < 3)` (head `0x806B78F8`, back edge
       `0x806B7930` `cmpi cr0,r25,3` / `blt`):
       `*(f32*)(r1+8) = f30` (0.0);
       `rc = 0x80075270(r3 = *(u8*)(this+0x38F), f1 = *(f32*)(r29+0x14AC),
       f2 = f29, r4 = &r1+8)`;
       `if (rc != 0) { *(f32*)(r29+0x14B0) = *(f32*)(r1+8); break; }`
       else `r25 += 1; f29 -= f31;`.
     - submit: `r12 = *(u32*)(r27)`; `r12 = *(u32*)(r12+0xB0)`;
       `mtctr r12`; `bcctr` with `r3 = r27`, `r4 = r28`, `r5 = 0`, `r6 = 0`
       (`0x806B7934-0x806B7950`).
   * `else if (slot - 2 <= 1)` (`addi r0,r26,-2` / `cmpli cr0,r0,1` /
     `bgt` past):
     `0x800E78E0(r3 = &r1+0xC, r4 = this+0x3F0, r5 = 0x807F5E00 "wendy_ring",
     r6 = 0x80377DC8)` at `0x806B7974`; then the same `+0xB0` submit with
     `r4 = &r1+0xC` (note: **not** `r28`) at `0x806B7990`.
   * `0x806B7998-0x806B79A8`: `slot += 1`, `r28 += 12`, `r27 += 276`,
     `r29 += 12`.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x806B78B8` | `0x802DCD68` | save r24-r31; `r11 = r1+64` |
| `0x806B790C` | `0x80075270` | `r3 = *(u8*)(this+0x38F)`, `f1 = *(f32*)(r29+0x14AC)`, `f2 = f29`, `r4 = &r1+8` -> `u32` |
| `0x806B7974` | `0x800E78E0` | `r3 = &r1+0xC`, `r4 = this+0x3F0`, `r5 = 0x807F5E00`, `r6 = 0x80377DC8` |
| `0x806B79CC` | `0x802DCDB4` | restore r24-r31; `r11 = r1+64` |

**Virtual slots:** `*(u32*)(this + 0x104C + 276*slot) -> +0xB0`, called with
`(self, arg, 0, 0)`.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 0.0` (initial probe value) | `0x807CAB48` | `lfs f30,-21688(r5)` @`0x806B78C8` |
| `f32 16.0` (probe decrement) | `0x807CAB50` | `lfs f31,-21680(r4)` @`0x806B78CC` |
| effect name | `0x807F5E00` "wendy_ring" | `addi r5,r30,24064` @`0x806B796C` |
| record argument | `0x80377DC8` | `addi r6,r31,32200` @`0x806B7970` |
| probe counter bound | `3` | `cmpi cr0,r25,3` @`0x806B792C` |
| slot count | `4` | `cmpi cr0,r26,4` @`0x806B79A0` |

Note the address used as `r6` is `0x80377DC8`, **not** `0x80377DEC`; it lies in
`main.dol`'s zero-fill segment (`0x80351680..0x80427680`, `filesz = 0`), so the
shipped artifacts carry no bytes for it.

**Residual ambiguity -> conditional (medium).** Three items:

1. `0x80377DC8` is a runtime cell: the REL cannot say whether it is a scale, a
   colour or a transform. Read it as `platform::read_f32(0x8037_7DC8)` or pass
   the address, never as a literal.
2. `0x80075270`'s contract is only visible as "`(u8, f32, f32, &mut f32) ->
   bool`"; the body uses it as a retrying probe (a returned non-zero means
   "accept the written value"). Note the pre-decrement happens **after** the
   call, so the third argument of the last attempt is `start - 32.0`.
3. The four `+0xB0` calls pass the *embedded object* (`this+0x104C+276i`) as the
   receiver, not a pointer loaded from it. A Rust `vmethod0(rcv, 0xB0)` with
   `rcv = this + 0x104C + 276*i` is exactly right, but the object at
   `this+0x14AC+12i` is a separate 12-byte record - do not fold them together.

---

## 4. `0x806B7690` - 312 bytes - `work/ghidra/rel-d_basesNP/export/806b7690_FUN_806b7690.c`

**Body:** `0x806B7690..0x806B77C8` (78 words, `blr` at `0x806B77C4`).
Ledger: absent. `src/game/bases.rs`: no definition; comments at lines 1521,
32285 and 33108.

```rust
pub unsafe fn fn_806b7690(this: usize)
```

**Frame.** `stwu r1,-96(r1)`; `mfspr r0,LR` / `stw r0,100(r1)`;
`addi r11,r1,80`; `stfd f31,80` + `psq_st f31,88`;
`bl 0x802DCD74` at `0x806B76A8` (saves **r27-r31**); `or r27,r3,r3`.
Epilogue `0x806B77A8..0x806B77C4`.

**Shape.** Emits one ring-burst effect per active slot, refreshes the burst
object, raises one UI message, then disposes or advances the actor.

1. `0x806B76AC-0x806B76C8` - `f31 = [0x807CAB5C]` (5500.0); `r30 = this`
   (+4/iter), `r28 = 0` (i), `r29 = 0x807CAB7C` (+4/iter), `r31 = 0x807F0000`.
2. `0x806B76CC-0x806B772C` - `while (i < *(u32*)(this+0x53C))`
   (head `0x806B7724` `lwz r0,1340(r27)` / `blt` at `0x806B772C`). Per
   iteration:
   * `idx = *(u32*)(r30 + 0xCC0)` (`lwz r0,3264(r30)`), `r8 = this + 12*idx`;
     `f0 = *(f32*)(r8 + 0x540)`, `f1 = *(f32*)(r8 + 0x544)`
     (`0x806B76F4`/`0x806B76F8`).
   * `*(f32*)(r1+0x28) = f0`, `*(f32*)(r1+0x2C) = f1`,
     `*(f32*)(r1+0x30) = f31` (5500.0).
   * `f2 = *(f32*)(r29 + 8)` -> `*(f32*)(r1+0x1C) = *(f32*)(r1+0x20) =
     *(f32*)(r1+0x24) = f2`.
   * `0x8016C890(r3 = 0x807F5E10 "Wm_we_ringburst", r4 = 0,
     r5 = &r1+0x28, r6 = 0, r7 = &r1+0x1C)` at `0x806B7714`.
   * `r30 += 4`, `r29 += 4`, `i += 1`.
3. `0x806B7730-0x806B7740` - submit on the embedded object at `this+0x1388`:
   `r12 = *(u32*)(this+0x1388)`; `r12 = *(u32*)(r12+0x10)`;
   `mtctr r12`; `bcctr` with `r3 = this+0x1388`.
4. `0x806B7744-0x806B777C` - message:
   `0x8006CED0(r3 = &r1+0x10, r4 = this)`;
   `r30 = *(u8*)(this+0x1040)`; `r31 = [0x80439D60]`;
   `0x8006A4B0(r3 = &r1+8, r4 = &r1+0x10)`;
   `0x80198FC0(r3 = r31, r4 = 1359, r5 = &r1+8, r6 = (s16)r30, r7 = 0)`.
   The `extsh` at `0x806B7770` sign-extends the byte before the call.
5. `0x806B7780-0x806B77A4` - `if (0x806B7830(this) == 0)`
   `0x80064D10(this, 0)` else `0x80162510(this)`.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x806B76A8` | `0x802DCD74` | save r27-r31; `r11 = r1+80` |
| `0x806B7714` | `0x8016C890` | `r3 = 0x807F5E10`, `r4 = 0`, `r5 = &r1+0x28`, `r6 = 0`, `r7 = &r1+0x1C` |
| `0x806B774C` | `0x8006CED0` | `r3 = &r1+0x10`, `r4 = this` |
| `0x806B7764` | `0x8006A4B0` | `r3 = &r1+8`, `r4 = &r1+0x10` |
| `0x806B777C` | `0x80198FC0` | `r3 = [0x80439D60]`, `r4 = 1359`, `r5 = &r1+8`, `r6 = (s16)*(u8*)(this+0x1040)`, `r7 = 0` |
| `0x806B7784` | `0x806B7830` | `r3 = this` -> `u32` |
| `0x806B7798` | `0x80064D10` | `r3 = this`, `r4 = 0` |
| `0x806B77A4` | `0x80162510` | `r3 = this` |

**Virtual slots:** `*(u32*)(this+0x1388) -> +0x10`, `r3 = this+0x1388`.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 5500.0` (effect lifetime) | `0x807CAB5C` | `lfs f31,-21668(r4)` @`0x806B76B8` |
| scale ramp, 5 entries | `0x807CAB84` | `lfs f2,8(r29)` @`0x806B76D8` |
| effect name | `0x807F5E10` "Wm_we_ringburst" | `addi r3,r31,24080` @`0x806B76D4` |
| message singleton | `0x80439D60` | `lwz r31,-25248(r3)` @`0x806B7758` (main.dol) |
| message id `1359` | immediate | `li r4,0x54F` @`0x806B7774` |

**Verified table contents** (`0x807CAB84 + 4*i`): `0.75, 0.70, 0.42, 0.28, 0.20`.
The loop bound `*(u32*)(this+0x53C)` is 5 in the shipped data, so the five live
entries are exactly consumed; the sixth word (`0x807CAB98`) is 0.0 and would be
read by a sixth slot.

**Confidence: medium.** All addresses, offsets and stores are literal. The two
soft spots are (a) `0x8016C890`'s five-argument contract (`name, u32, &vec3,
u32, &vec3` - reproduced literally, not interpreted) and (b) the meaning of the
index array at `this+0xCC0` (it indexes a 12-byte record array at `this+0x540`;
both offsets are shipped, the semantic role is not). Emit
`// unknown: 0x8016C890(name, 0, &params, 0, &scale)`.

---

## 5. `0x806A23B0` - 308 bytes - `work/ghidra/rel-d_basesNP/export/806a23b0_FUN_806a23b0.c`

**Body:** `0x806A23B0..0x806A24E4` (77 words, `blr` at `0x806A24E0`).
Ledger: absent. `src/game/bases.rs`: no definition; the export is named in a
doc comment at line 1652 (module note) and the address in one more comment.

```rust
pub unsafe fn fn_806a23b0(this: usize, other: usize)
```

**Argument map** (from the single in-REL caller `0x806A2BB0`, which passes
`r3 = r31` and `r4 = r30`): `r3 -> this`, `r4 -> other`. No other reading of the
body is possible: every `this`-relative access uses `r30` and every
`other`-relative access uses the untouched `r4` (`0xAC`, `0x16C`).

**Frame.** `stwu r1,-80(r1)`; `mfspr r0,LR` / `stw r0,84(r1)`;
`stfd f31,64` + `psq_st f31,72`, `f30` 48/56, `f29` 32/40;
`stw r31,28` / `r30,24`; `r31 = 0x807CA2D8`; `or r30,r3,r3`.
No save-helper, no virtual-slot helper calls other than the one below.

**Shape.** A one-axis "shove the actor out of the other actor", used when the
pair overlaps. It reads the x delta, gates on `delta_x * rate <= 0`, computes a
clamped penetration, and moves x only.

1. `0x806A23E8-0x806A2408` - gate:
   `f31 = *(f32*)(this+0xAC) - *(f32*)(other+0xAC)`;
   `f1 = f31 * *(f32*)(this+0xE8)`; `f0 = [0x807CA390]` (0.0);
   `fcmpo cr0,f1,f0`; `cror 2,0,2` (`CR0.EQ := LT|EQ`);
   `bne 0x806A24B4` -> the whole body is skipped when **`f1 > 0.0`**, i.e. it
   runs when `f1 <= 0.0`. For NaN both LT and EQ are clear, so the `bne` is
   taken and the body is skipped - the gate is exactly `!(f1 > 0.0)`.
   Ghidra renders this correctly as `<= (double)DAT_807ca390`.
2. `0x806A240C-0x806A241C` - `f30 = |f31|`
   (`fcmpo f31,0` / `ble` -> `fneg f30,f31`, else `fmr f30,f31`).
3. `0x806A2420-0x806A2434` - `f1 = vmethod0_f(this, 0x314)`
   (`lwz r12,96(r30)` = `*(this+0x60)`; `lwz r12,788(r12)`; `r3 = this`;
   `bcctr`).
4. `0x806A2428-0x806A24B0` - penetration:
   `f29 = *(f32*)(other+0x16C)`;
   `f1 = f1 + f29`; `f3 = f1 - f30`;
   `if (f3 > [0x807CA394] (6.0)) f3 = 6.0`;
   copy `*(f32*)(this+0xAC/0xB0/0xB4)` to `r1+8/0xC/0x10`;
   `f0 = (f31 > 0.0) ? [0x807CA398] (1.0) : [0x807CA39C] (-1.0)`
   (`fcmpo` at `0x806A245C`, `ble` selects `-1.0`; the `fcmpo` is repeated at
   `0x806A2498` with the same operands and its result is unused);
   `f3 = f3 * f0`;
   `x = *(f32*)(r1+8) + f3`;
   `*(f32*)(this+0xB0) = *(f32*)(r1+0xC)` (redundant store);
   `*(f32*)(this+0xAC) = x` (and x is also written back to `r1+8`);
   `*(f32*)(this+0xB4) = *(f32*)(r1+0x10)` (redundant store);
   `*(f32*)(this+0xE8) = 0.0`.

The three `r1+8/0xC/0x10` slots are read back only for x; the y/z stores from the
body are bit-identical reloads of the values it just stored. Transcribe them as
written - they are observable only to a concurrent reader, but the compiler
emitted them and they cost nothing.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x806A2434` | `vmethod` `*(this+0x60)+0x314` | `r3 = this` -> `f1` |

**Virtual slots:** `*(u32*)(this+0x60) -> +0x314`, `r3 = this`. One call, no
indirect dispatch elsewhere. Six `psq*` words, all `qr0`/`W=0`
(`f31/f30/f29` saves).

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 0.0` | `0x807CA390` | `lfs f0,184(r31)` @`0x806A23F8` |
| `f32 6.0` (max penetration) | `0x807CA394` | `lfs f0,188(r31)` @`0x806A243C` |
| `f32 1.0` (+x sign) | `0x807CA398` | `lfs f0,192(r31)` @`0x806A2474` |
| `f32 -1.0` (-x sign) | `0x807CA39C` | `lfs f0,196(r31)` @`0x806A247C` |

**Confidence: high.** Every operand is literal, the only indirection is one
vtable slot with an explicit receiver, and the `cror`-merged gate is now
resolved. Two notes for the implementer: keep the `<= 0` comparison as
`!(f1 > 0.0)` so NaN skips the body, and keep the duplicated `fcmpo` pair -
the second one feeds nothing but is part of the shipped stream.

---

## 6. `0x806DF630` - 308 bytes - `work/ghidra/rel-d_basesNP/export/806df630_FUN_806df630.c`

**Body:** `0x806DF630..0x806DF764` (77 words, `blr` at `0x806DF760`).
Ledger: absent. `src/game/bases.rs`: no definition, no mention.

```rust
pub unsafe fn fn_806df630(this: usize, y: f32)
```

**Argument map** (from both in-REL callers, `0x806DEDBC` and `0x806E09E8`, which
compute a float delta in `f31` and do `fmr f1,f31` immediately before the call):
`r3 -> this`, `f1 -> y`. Ghidra's `void FUN_806df630(void)` loses both because
`0x802DCD74` returns a `u64` in its model.

**Frame.** `stwu r1,-64(r1)`; `mfspr r0,LR` / `stw r0,68(r1)`;
`addi r11,r1,32`; `stfd f31,48` + `psq_st f31,56`, `f30` 32/40;
`bl 0x802DCD74` at `0x806DF650` (saves **r27-r31**); `or r27,r3,r3`.
Epilogue `0x806DF73C..0x806DF760`.

**Shape.** Builds three beam panels around the actor at heights `y-8`, `y+8` and
`8`, each through the same ten-argument builder, and marks them with the same
stage byte.

1. `0x806DF654-0x806DF674` - `f3 = [0x807CBA40]` (8.0); `f30 = y`;
   `f31 = y - f3`; `*(u8*)(this+0x51E) = [0x80429D28]`.
2. `0x806DF678-0x806DF6A0` - **panel 1**:
   `0x800DB180(f1 = f30, f2 = [0x807CBA54] (0.0), f3 = 8.0, f4 = f31,
   f5 = -f31, r3 = this+0x4D4, r4 = this, r5 = 0, r6 = 1, r7 = 0)`;
   then `0x800DAE80(this+0x4D4)`.
3. `0x806DF6A4-0x806DF6F0` - **panel 2**:
   `*(u8*)(this+0x57E) = [0x80429D28]`;
   `0x800DB180(f1 = f30 + f31, f2 = 0.0, f3 = [0x807CBA74] (5.0),
   f4 = [0x807CBA78] (14.0), f5 = [0x807CBA7C] (-7.0), r3 = this+0x534,
   r4 = this, r5 = -10240, r6 = 1, r7 = 0)`;
   `*(u8*)(this+0x57B) = 10`; `*(u32*)(this+0x570) = 0x900`
   (`li r31,256` / `ori r0,r31,0x800`); `0x800DAE80(this+0x534)`.
4. `0x806DF6F4-0x806DF738` - **panel 3**:
   `*(u8*)(this+0x5DE) = [0x80429D28]`;
   `0x800DB180(f1 = f30 - f31, f2 = 0.0, f3 = 5.0, f4 = [0x807CBA80] (7.0),
   f5 = [0x807CBA84] (-14.0), r3 = this+0x594, r4 = this, r5 = 10240,
   r6 = 1, r7 = 0)`;
   `*(u8*)(this+0x5DB) = 10`; `*(u32*)(this+0x5D0) = 0x900`;
   `0x800DAE80(this+0x594)`.

Note the asymmetry of the last two argument pairs: panel 2 gets `(14.0, -7.0)`
and panel 3 gets `(7.0, -14.0)`. This is not a transcription error - the pool is
`5.0, 14.0, -7.0, 7.0, -14.0` at `0x807CBA74..0x807CBA84`, and the loads are
`+0x6C/+0x70/+0x74` and `+0x6C/+0x78/+0x7C`.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x806DF650` | `0x802DCD74` | save r27-r31; `r11 = r1+32` |
| `0x806DF698` | `0x800DB180` | `f1 = y`, `f2 = 0.0`, `f3 = 8.0`, `f4 = y-8`, `f5 = 8-y`, `r3 = this+0x4D4`, `r4 = this`, `r5 = 0`, `r6 = 1`, `r7 = 0` |
| `0x806DF6A0` | `0x800DAE80` | `r3 = this+0x4D4` |
| `0x806DF6D4` | `0x800DB180` | `f1 = 2y-8`, `f2 = 0.0`, `f3 = 5.0`, `f4 = 14.0`, `f5 = -7.0`, `r3 = this+0x534`, `r4 = this`, `r5 = -10240`, `r6 = 1`, `r7 = 0` |
| `0x806DF6F0` | `0x800DAE80` | `r3 = this+0x534` |
| `0x806DF724` | `0x800DB180` | `f1 = 8.0`, `f2 = 0.0`, `f3 = 5.0`, `f4 = 7.0`, `f5 = -14.0`, `r3 = this+0x594`, `r4 = this`, `r5 = 10240`, `r6 = 1`, `r7 = 0` |
| `0x806DF738` | `0x800DAE80` | `r3 = this+0x594` |
| `0x806DF750` | `0x802DCDC0` | restore r27-r31; `r11 = r1+32` |

No virtual calls. Four `psq*` words, all `qr0`/`W=0` (`f31/f30`).

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 8.0` (panel half-height) | `0x807CBA40` | `lfs f3,56(r28)` @`0x806DF664` |
| `f32 0.0` | `0x807CBA54` | `lfs f2,76(r28)` @`0x806DF678` |
| `f32 5.0` | `0x807CBA74` | `lfs f3,108(r28)` @`0x806DF6BC` |
| `f32 14.0` | `0x807CBA78` | `lfs f4,112(r28)` @`0x806DF6C0` |
| `f32 -7.0` | `0x807CBA7C` | `lfs f5,116(r28)` @`0x806DF6C8` |
| `f32 7.0` | `0x807CBA80` | `lfs f4,120(r28)` @`0x806DF710` |
| `f32 -14.0` | `0x807CBA84` | `lfs f5,124(r28)` @`0x806DF718` |
| stage byte | `0x80429D28` | `lbz r0,-25304(r29)` @`0x806DF660`, `0x806DF6A4`, `0x806DF6F4` |
| state word `0x900` | immediate | `li r31,256` + `ori r0,r31,0x800` @`0x806DF6E0`, `0x806DF728` |

**Confidence: medium.** All three panel descriptions and both bookkeeping stores
are literal. The residual doubt is `0x800DB180`'s contract: it takes five floats
plus `(this+off, this, i32, i32, i32)` and is called with `r5 = 0`, `-10240`,
`10240` and `r6 = 1`, `r7 = 0`. Emit it as
`abi_call8!(0x800D_B180, ...)` with `// unknown: 3-beam builder (5 floats, 5 ints)`
and keep `r5` signed (`-10240` is `0xFFFFD800`).

---

## 7. `0x80722180` - 296 bytes - `work/ghidra/rel-d_basesNP/export/80722180_FUN_80722180.c`

**Body:** `0x80722180..0x807222A8` (74 words, `blr` at `0x807222A4`).
Ledger: absent. `src/game/bases.rs`: no definition, no mention.

```rust
pub unsafe fn fn_80722180(this: usize, _unused: u32, xf: usize, model: usize)
```

**Argument map** (register truth; `r4` is never read anywhere in the body):
`r3 -> this`, `r5 -> xf` (a transform/rotation sink, used as `r3` for both
`0x80241710` and `0x80241AA0`), `r6 -> model` (an object whose name table is
probed). The body has no in-REL caller; it is reached from a vtable.

**Frame.** `stwu r1,-80(r1)`; `mfspr r0,LR` / `stw r0,84(r1)`;
`addi r11,r1,64`; `stfd f31,64` + `psq_st f31,72`;
`bl 0x802DCD74` at `0x80722198` (saves **r27-r31**);
`or r27,r3,r3` (this), `or r28,r5,r5` (xf).
Epilogue `0x80722288..0x807222A4`.

**Shape.** A "face_1 / hair_1" name probe against the model's name blob, then a
conditional yaw offset applied to the transform.

1. `0x807221A4-0x807221D8` - `o = 0x8023B300(model)`; `off = *(u32*)(o+8)`;
   `r31 = o`; `name = (off == 0) ? 0 : o + off`;
   `face = (0x802E1AB4(name, 0x8080A238 "face_1") == 0)`.
   The equality is the `cntlzw` + `rlwinm r29,r0,27,5,31` pair at
   `0x807221D8`/`0x807221DC`: `cntlzw(0) = 32` and the rotate/mask maps any value
   `>= 32` to 1, everything else to 0.
2. `0x807221DC-0x80722204` - the same probe with `0x8080A268 "hair_1"` ->
   `hair` in `r30`. The blob address is re-derived from `r31 + r30` again
   (`0x807221E4`).
3. `0x80722208-0x80722210` - `if (!face && !hair) goto 0x80722288` (epilogue).
   Note the two tests are separate: `bne 0x80722214` when `face != 0`, else
   `beq 0x80722288` when `hair == 0`.
4. `0x80722214-0x8072224C` - angle:
   `p = *(u32*)(this+4)`;
   `a = (s16)*(p+0x608)`; `a = (a * 90) / 16384` with truncation toward zero
   (`mulli r0,r0,90` / `srawi r0,r0,14` / `addze r0,r0` at
   `0x80722234-0x8072223C`);
   `f31 = (f32)a` via `xoris 0x8000` + `lis 0x4330` + `lfd` + `fsubs`
   against `[0x807CD148]` (`0x4330000080000000` = 2^52 + 2^31).
5. `0x80722250-0x80722284` - `0x80241710(r3 = xf, r4 = &r1+8)`; then
   `if (face) *(f32*)(r1+0x10) -= f31`; `if (hair) *(f32*)(r1+0x10) += f31`;
   then `0x80241AA0(r3 = xf, r4 = &r1+8)`.
   Only the **third** float (`r1+0x10`) is touched; the verifier should see the
   same value written by the first call and read back by the second.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80722198` | `0x802DCD74` | save r27-r31; `r11 = r1+64` |
| `0x807221A8` | `0x8023B300` | `r3 = model` -> object with a name blob at `+8` |
| `0x807221D0` | `0x802E1AB4` | `r3 = name`, `r4 = 0x8080A238` -> `u32` |
| `0x807221F8` | `0x802E1AB4` | `r3 = name`, `r4 = 0x8080A268` -> `u32` |
| `0x80722250` | `0x80241710` | `r3 = xf`, `r4 = &r1+8` |
| `0x80722284` | `0x80241AA0` | `r3 = xf`, `r4 = &r1+8` |
| `0x80722294` | `0x802DCDC0` | restore r27-r31; `r11 = r1+64` |

No virtual calls. Two `psq*` words, `qr0`/`W=0` (one `f31` pair).

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f64 2^52+2^31` = `0x4330000080000000` | `0x807CD148` | `lfd f1,-11960(r4)` @`0x8072222C` |
| name key "face_1" | `0x8080A238` | `addi r4,r4,-24008` @`0x807221CC` |
| name key "hair_1" | `0x8080A268` | `addi r4,r4,-23960` @`0x807221F4` |
| angle divisor | `16384` (0x4000) | `srawi r0,r0,14` @`0x80722238` |
| angle scale | `90` | `mulli r0,r0,0x5A` @`0x80722234` |

**Confidence: medium.** Both probes, the integer ladder and the two transform
calls are literal; the ambiguity is the *contract* of `0x80241710` /
`0x80241AA0` (a decompose/recompose pair on a 3-float vector; the body proves the
vector is 12 bytes at `r1+8` and that the callee writes it, but not what the
components mean). Emit `// unknown: 0x80241710(xf, &mut [f32;3])` and note that
`this+4` must be a live pointer whose `+0x608` field is a signed 16-bit angle in
units of `2^14 / 90` degrees.

---

## 8. `0x80780BD0` - 292 bytes - `work/ghidra/rel-d_basesNP/export/80780bd0_FUN_80780bd0.c`

**Body:** `0x80780BD0..0x80780CF4` (73 words, `blr` at `0x80780CF0`).
Ledger: absent. `src/game/bases.rs`: no definition; doc comments at 75995-76001
and 116963.

```rust
pub unsafe fn fn_80780bd0(out: *mut f32, this: usize, src: *const f32, extra: i16)
```

**Argument map** (from the head of the body and from the retrieve/place wrapper
at `0x8077E...` that calls it): `r3 -> out` (three `stfs` at `0(r28)`),
`r4 -> this` (`*(u32*)(r4+0x1F8)`), `r5 -> src` (three `lfs` at `0(r30)`),
`r6 -> extra` (`add r0,r3,r31` then `sth`).

**Frame.** `stwu r1,-128(r1)`; `mfspr r0,LR` / `stw r0,132(r1)`;
`stfd f31,112` + `psq_st f31,120`; `stw r31,108` / `r30,104` / `r29,100` /
`r28,96`; `r31 = extra`, `r30 = src`, `r29 = this`, `r28 = out`.
Epilogue `0x80780CCC..0x80780CF0`.

**Shape.** A placement wrapper: either pass the caller's vector through, or
build a rotated offset and add it to the caller's vector.

1. `0x80780C08-0x80780C18` - `0x80775800(r3 = [0x8043A18C],
   r4 = *(u32*)(this+0x1F8))`; `if (r3 != 0)` -> step 2, else step 3.
2. `0x80780C1C-0x80780C34` - `out[0..2] = src[0..2]` (`stfs` at `0(r28)`,
   `4(r28)`, `8(r28)`), then epilogue.
3. `0x80780C38-0x80780C80` - build the offset:
   `f31 = 0x807812E0(this)` (`0(r29)`);
   `a = 0x80781330(this)` (`0(r29)`);
   angle = `(u16)(a + extra)` -> `sth r0,8(r1)`;
   `*(f32*)(r1+0x18) = [0x807CF698]` (0.0);
   `*(f32*)(r1+0x1C) = 0.0`;
   `*(f32*)(r1+0x20) = f31`;
   `0x8016ED10(r3 = &r1+0x28, r4 = &r1+8)`;
   `0x801C1160(r3 = &r1+0x28, r4 = &r1+0x18, r5 = &r1+0x18)`.
4. `0x80780C84-0x80780CC8` - `out[i] = *(f32*)(r1+0x18+4i) + src[i]`
   (`fadds` order: z at `0x80780C90`, y at `0x80780C9C`, x at `0x80780CA8`);
   the sums are also re-stored at `r1+0x0C/0x10/0x14` and `r1+0x18/0x1C/0x20`
   (dead writes, keep or drop).

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80780C10` | `0x80775800` | `r3 = [0x8043A18C]`, `r4 = *(u32*)(this+0x1F8)` -> `u32` |
| `0x80780C3C` | `0x807812E0` | `r3 = this` -> `f1` |
| `0x80780C48` | `0x80781330` | `r3 = this` -> `u32` |
| `0x80780C70` | `0x8016ED10` | `r3 = &r1+0x28` (48-byte out), `r4 = &r1+8` (`s16` angle) |
| `0x80780C80` | `0x801C1160` | `r3 = &r1+0x28`, `r4 = &r1+0x18`, `r5 = &r1+0x18` (in place) |

No virtual calls. One `psq*` pair, `qr0`/`W=0` (`f31`).

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 0.0` (pad value) | `0x807CF698` | `lfs f0,-2408(r4)` @`0x80780C54` |
| manager singleton | `0x8043A18C` | `lwz r3,-24180(r7)` @`0x80780C08` (main.dol) |

**Residual ambiguity -> conditional (medium).** This body is the exact sibling of
wave-131 §3 (`0x807816B0`): the 48-byte object at `r1+0x28` is produced entirely
by `0x8016ED10` and consumed by `0x801C1160`, and this REL never writes it, so its
shape (`[f32; 12]` basis vs `[u8; 48]`) is a layout inference. Two further
points: `0x801C1160` gets the *same* pointer as `r4` and `r5` (in place - do not
introduce a temporary), and the angle is truncated to 16 bits by the `sth`, so
the add `a + extra` must be `wrapping_add` on `u32` before truncation, exactly as
shipped. Everything else - both branches, the four call sites and the three
output stores - is literal.

---

## 9. `0x80780E20` - 292 bytes - `work/ghidra/rel-d_basesNP/export/80780e20_FUN_80780e20.c`

**Body:** `0x80780E20..0x80780F44` (73 words, `blr` at `0x80780F40`).
Ledger: absent as a row; mentioned in the evidence of `fn_8077e760`
(ledger line 4443: "`80780E20`; +0x210 = (members-1)*8"). `src/game/bases.rs`:
no definition; one mention.

```rust
pub unsafe fn fn_80780e20(this: usize)
```

**Frame.** `stwu r1,-48(r1)`; `mfspr r0,LR` / `stw r0,52(r1)`;
`stw r31,44` / `r30,40` / `r29,36` / `r28,32`;
`r31 = this`, `r29 = 0x807CF608`, `r28 = [0x8043A18C]`.
No `psq` at all. Epilogue `0x80780F24..0x80780F40`.

**Shape.** A level/scale initialiser for one pane: copy two fields out of a
lookup, resolve a name, store a scale, then either inherit a facing angle or
keep the current one, and finally re-resolve the pane record.

1. `0x80780E38-0x80780E74` - `p = 0x80782050(this+0x214)`;
   `*(u32*)(this+0x1F8) = *(u32*)(p+0)`;
   `*(u32*)(this+0x26C) = *(u32*)(p+4)`.
2. `0x80780E78-0x80780E88` - `idx = *(u32*)(r28+0x338C)`;
   `tbl = r28 + idx*0x16518 + 0x3394` (`mullw` by `0x16518` then `+13204`);
   `0x800F57A0(r3 = tbl, r4 = *(u32*)(this+0x1F4), r5 = 0x100)`.
3. `0x80780E8C-0x80780EA4` - `*(f32*)(this+0x310) = (r3 == 0) ?
   [0x807CF69C] (1.0) : [0x807CF7D0] (0.95)`.
   Careful with the polarity: `beq` at `0x80780E90` jumps to the `1.0` store.
4. `0x80780EA8-0x80780ED4` - `m = [0x8043A1A0]`;
   `*(f32*)(r1+8) = *(f32*)(m+0x2D0)`;
   `*(f32*)(r1+0xC) = *(f32*)(m+0x2D4)`;
   `*(f32*)(r1+0x10) = *(f32*)(m+0x2D8)`;
   `0x801C1650(r3 = m+0xAC, r4 = &r1+8)`;
   then `f1 = 0x802BE470(f1)` with no register setup between the two calls, i.e.
   it consumes the `f1` left by `0x801C1650` (or ignores it - the callee is not in
   this REL).
5. `0x80780ED8-0x80780EF4` - `f0 = [0x807CF698]` (0.0);
   `fcmpo cr0,f1,f0`; `beq 0x80780EF8` -> `if (f1 != f0)`
   `*(s16*)(this+0x1E6) = *(s16*)(m+0x102)`.
   The `mfcr r0` / `rlwinm r0,r0,2,31,31` pair between the `fcmpo` and the `beq`
   is dead (its result is overwritten by `lha r0,258(r3)` at `0x80780EF0`) - the
   branch tests `CR0` from the `fcmpo`. Ghidra renders this as
   `DAT_807cf698 < dVar5`, i.e. `> 0`; the shipped instruction is `!= 0`. The two
   differ only for NaN.
6. `0x80780EF8-0x80780F20` - `*(u32*)(this+0x204) = *(u32*)(this+0x200)`;
   `idx = *(u32*)(r28+0x338C)`;
   `0x800F3330(r3 = r28 + 0x5C7F4 + idx*0x3F08, r4 = *(u32*)(this+0x1F4),
   r5 = *(u32*)(this+0x1F8))`;
   `*(u32*)(this+0x200) = r3`.
   The address is built as `addis r3,r28,6` / `add r3,r3,r0` /
   `addi r3,r3,-14348`, i.e. `r28 + 0x60000 - 0x380C + idx*0x3F08`.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80780E54` | `0x80782050` | `r3 = this+0x214` -> `*const u32` (2 words) |
| `0x80780E88` | `0x800F57A0` | `r3 = tbl`, `r4 = *(u32*)(this+0x1F4)`, `r5 = 0x100` -> `u32` |
| `0x80780ED0` | `0x801C1650` | `r3 = m+0xAC`, `r4 = &r1+8` |
| `0x80780ED4` | `0x802BE470` | `f1` (returned by the previous call) -> `f1` |
| `0x80780F1C` | `0x800F3330` | `r3 = tbl2`, `r4 = *(u32*)(this+0x1F4)`, `r5 = *(u32*)(this+0x1F8)` -> `u32` |

No virtual calls, no `psq`.

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 1.0` | `0x807CF69C` | `lfs f0,148(r29)` @`0x80780EA0` |
| `f32 0.95` | `0x807CF7D0` | `lfs f0,456(r29)` @`0x80780E94` |
| `f32 0.0` | `0x807CF698` | `lfs f0,144(r29)` @`0x80780ED8` |
| manager singleton | `0x8043A18C` | `lwz r28,-24180(r4)` @`0x80780E50` (main.dol) |
| actor | `0x8043A1A0` | `lwz r5,-24160(r30)` @`0x80780EB0` (main.dol) |
| stride 1 | `0x16518` (91 416) | `mullw r0,r3,r0` @`0x80780E7C` |
| stride 2 | `0x3F08` (16136) | `mulli r0,r0,0x3F08` @`0x80780F10` |

**Confidence: medium.** Every operand, both strides and all three stores are
literal, and the only inference is the contract of `0x80782050` (a two-word
lookup) and `0x802BE470` (a float reduction over the `0x801C1650` result -
probably a length). Write the gate as `if (v != 0.0)` and add a comment that
Ghidra prints `> 0.0`; do **not** reorder it after the `*(this+0x204)` copy, the
store order is observable.

---

## 10. `0x80702AA0` - 276 bytes - `work/ghidra/rel-d_basesNP/export/80702aa0_FUN_80702aa0.c`

**Body:** `0x80702AA0..0x80702BB4` (69 words, `blr` at `0x80702BB0`).
Ledger: absent. `src/game/bases.rs`: no definition; one mention.

```rust
pub unsafe fn fn_80702aa0(this: usize)
```

**Frame.** `stwu r1,-144(r1)`; `mfspr r0,LR` / `stw r0,148(r1)`;
`addi r11,r1,48`; six `stfd`+`psq_st` pairs
(`f31` 128/136, `f30` 112/120, `f29` 96/104, `f28` 80/88, `f27` 64/72,
`f26` 48/56); `bl 0x802DCD68` at `0x80702AE0` (saves **r24-r31**).
Epilogue `0x80702B6C..0x80702BB0`.

**Shape.** A fixed five-entry initialiser. No branches inside the loop body.

1. `0x80702AE4-0x80702AF0` - `r4 = 0x807CC588`; `r25 = this + 0x744`;
   `r24 = this + 0xA14` (loop limit).
2. `0x80702AF4-0x80702B20` - load the pool and the immediates once:
   `f31..f26 = *(f32*)(r4 + 0x10,0x14,0x18,0x1C,0x20,0x24)`;
   `r26 = 128`, `r27 = 768`, `r28 = 64`, `r29 = 32`, `r30 = 2`, `r31 = 8`.
3. `0x80702B24-0x80702B68` - `do { ... } while (r25 < r24)` (back edge
   `cmpl`/`blt` at `0x80702B68`), stride `0x90` = 144:
   * `*(f32*)(e+0x3C) = f31`, `+0x40 = f30`, `+0x44 = f29`, `+0x48 = f28`,
     `+0x4C = f27`, `+0x50 = f26`.
   * `*(u16*)(e+0x54) = 128`, `+0x56 = 768`, `+0x26 = 64`, `+0x1C = 32`,
     `+0x1E = 2`, `+0x22 = 8`, `+0x24 = 128`.
   * `r3 = e`; `0x80701C10(e)` at `0x80702B5C`.
   * `e += 0x90`.

The loop is exactly 5 iterations: `0xA14 - 0x744 = 0x2D0 = 5 * 0x90`.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80702AE0` | `0x802DCD68` | save r24-r31; `r11 = r1+48` |
| `0x80702B5C` | `0x80701C10` | `r3 = e` (entry) |
| `0x80702BA0` | `0x802DCDB4` | restore r24-r31; `r11 = r1+48` |

No virtual calls. Twelve `psq*` words, all `qr0`/`W=0`
(six `f26..f31` pairs).

**Constants:**

| value | image address | materialised at |
|---|---|---|
| pool base | `0x807CC588` | `addi r4,r4,-14968` @`0x80702AEC` |
| `f32 0.5` | `0x807CC598` | `lfs f31,16(r4)` @`0x80702AF4` |
| `f32 100.0` | `0x807CC59C` | `lfs f30,20(r4)` @`0x80702AFC` |
| `f32 1.25` | `0x807CC5A0` | `lfs f29,24(r4)` @`0x80702B04` |
| `f32 2.0` | `0x807CC5A4` | `lfs f28,28(r4)` @`0x80702B0C` |
| `f32 0.125` | `0x807CC5A8` | `lfs f27,32(r4)` @`0x80702B14` |
| `f32 0.125` | `0x807CC5AC` | `lfs f26,36(r4)` @`0x80702B1C` |

**Confidence: high.** Five copies of the same 144-byte literal, no branch inside
the body, no indirection. The only unknown is `0x80701C10(entry)`'s contract
(a per-entry post-initialiser); everything it reads is written immediately
before it.

---

## 11. `0x80702BC0` - 276 bytes - `work/ghidra/rel-d_basesNP/export/80702bc0_FUN_80702bc0.c`

**Body:** `0x80702BC0..0x80702CD4` (69 words, `blr` at `0x80702CD0`).
Ledger: absent. `src/game/bases.rs`: no definition; one mention.

```rust
pub unsafe fn fn_80702bc0(this: usize)
```

**Frame and shape are identical to §10**: `stwu r1,-144`; `addi r11,r1,48`; the
same six `stfd`/`psq_st` pairs; `bl 0x802DCD68` at `0x80702C00`; `r25 = this +
0x744`, `r24 = this + 0xA14`; the same pool `0x807CC588` (+`0x10..0x24`) and the
same seven halfword stores per entry; the same stride `0x90`.

**The single difference** is the per-entry callback:

| `0x80702AA0` | `0x80702BC0` |
|---|---|
| `bl 0x80701C10` @`0x80702B5C` | `bl 0x80088E80` @`0x80702C7C` |

Everything else is identical word for word: of the 69 words, only three differ -
the two `0x802DCDxx` calls (same target, different PC-relative displacement) and
word 47, the callback itself. All thirteen stores per entry (six `stfs` + seven
`sth`), the loop head, the single `addi r25,r25,0x90` bump and the `cmpl`/`blt`
back edge are the same. Implement them as two functions (the repo's style for
`fn_XXXXXXXX` keeps one address per function) but consider a shared private
helper that takes the callback address; do **not** merge them into one public
symbol, because both addresses must exist in the ledger.

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80702C00` | `0x802DCD68` | save r24-r31; `r11 = r1+48` |
| `0x80702C7C` | `0x80088E80` | `r3 = e` (entry) |
| `0x80702CC0` | `0x802DCDB4` | restore r24-r31; `r11 = r1+48` |

**Constants:** identical to §10 (`0x807CC588` pool, six floats, seven
halfword immediates).

**Confidence: high.** Same reasoning as §10, and the twin relationship itself is
evidence: the only word that differs in the 69-word bodies is the call
displacement.

---

## 12. `0x80724460` - 276 bytes - `work/ghidra/rel-d_basesNP/export/80724460_FUN_80724460.c`

**Body:** `0x80724460..0x80724574` (69 words, `blr` at `0x80724570`).
Ledger: absent. `src/game/bases.rs`: no definition; doc comments at 153645-153654
("asks `0x80724460` for a placement and writes it back").

```rust
pub unsafe fn fn_80724460(out: *mut f32, this: usize)
```

**Argument map:** `r3 -> out` (three `stfs` at `0(r29)`), `r4 -> this`
(`(s8)*(u8*)(r4+0x38D)` at the top).

**Frame.** `stwu r1,-160(r1)`; `mfspr r0,LR` / `stw r0,164(r1)`;
`stfd f31,144` + `psq_st f31,152`; `stw r31,140` / `r30,136` / `r29,132`;
`f31 = [0x807CD1BC]` (0.0); `r30 = this`, `r29 = out`.
Epilogue `0x80724550..0x80724574`.

**Shape.** Ask the player's placement path for a world position; if the player is
in the "attached" state, return the actor's own position instead.

1. `0x80724490-0x807244A8` - `player = 0x8005F900((s8)*(u8*)(this+0x38D))`
   (`lbz` + `extsb`); `0x80056CF0(player, 4)` at `0x807244A4`.
2. `0x807244AC-0x807244C8` - `if (r3 != 0)` (`beq 0x807244CC` skips):
   `out[0..2] = *(f32*)(this+0xAC/0xB0/0xB4)`; epilogue.
3. `0x807244CC-0x80724504` - build and transform:
   `0x8012DE80(r3 = &r1+8, r4 = player)` -> three floats at `r1+8/0xC/0x10`;
   `0x801C0BD0(r3 = &r1+0x50, f1 = *(r1+8), f2 = *(r1+0xC), f3 = *(r1+0x10))`;
   `m = 0x80056E30(player)`;
   `0x801C0540(r3 = &r1+0x50, r4 = m+0x108, r5 = &r1+0x50)` (in place).
4. `0x80724508-0x80724528` - `0x801C0BD0(r3 = &r1+0x20, f1 = f2 = f3 = 0.0)`
   (`f31` in all three registers); `0x801C0540(r3 = &r1+0x50, r4 = &r1+0x20,
   r5 = &r1+0x50)`.
5. `0x8072452C-0x8072454C` - `0x8016F130(r3 = &r1+0x50, r4 = &r1+0x14)`;
   `out[0] = *(f32*)(r1+0x14)`, `out[1] = *(f32*)(r1+0x18)`,
   `out[2] = *(f32*)(r1+0x1C)` (loads at `0x80724538-0x80724540`, stores at
   `0x80724544-0x8072454C`).

**Direct callees:**

| site | callee | arguments |
|---|---|---|
| `0x80724498` | `0x8005F900` | `r3 = (s8)*(u8*)(this+0x38D)` -> player |
| `0x807244A4` | `0x80056CF0` | `r3 = player`, `r4 = 4` -> `u32` |
| `0x807244D4` | `0x8012DE80` | `r3 = &r1+8`, `r4 = player` (3 floats out) |
| `0x807244E8` | `0x801C0BD0` | `r3 = &r1+0x50`, `f1/f2/f3 = r1+8/0xC/0x10` |
| `0x807244F0` | `0x80056E30` | `r3 = player` -> object |
| `0x80724504` | `0x801C0540` | `r3 = &r1+0x50`, `r4 = m+0x108`, `r5 = &r1+0x50` |
| `0x80724518` | `0x801C0BD0` | `r3 = &r1+0x20`, `f1 = f2 = f3 = 0.0` |
| `0x80724528` | `0x801C0540` | `r3 = &r1+0x50`, `r4 = &r1+0x20`, `r5 = &r1+0x50` |
| `0x80724534` | `0x8016F130` | `r3 = &r1+0x50`, `r4 = &r1+0x14` (3 floats out) |

No virtual calls. One `psq*` pair, `qr0`/`W=0` (`f31`).

**Constants:**

| value | image address | materialised at |
|---|---|---|
| `f32 0.0` (translation) | `0x807CD1BC` | `lfs f31,-11844(r5)` @`0x80724478` |

Two immediates: the player-slot byte `this+0x38D` and the query id `4`
(`li r4,4`). Note the position fields (`this+0xAC/0xB0/0xB4`) are the same triple
every other body in this batch uses.

**Confidence: medium.** All nine call sites, both branches and the three output
stores are literal. The ambiguity is confined to the matrix helpers
(`0x801C0BD0` build-from-three-floats, `0x801C0540` concatenate, `0x8016F130`
extract-translation): their contracts are inferred from the register pattern and
from `0x801C0540`'s in-place use (`r3 == r5`). Emit
`// unknown: 0x801C0BD0(mtx_out, f32, f32, f32)` and keep the scratch matrix at
`r1+0x50` untyped; the only thing the body proves about its size is that
`r1+0x50..r1+0x80` is never read as anything else.

---

## 13. The four tied bodies (already decoded for this lane, kept as a hand-off)

These four were decoded in full during selection and lost the tiebreak only on
size/address order. Nothing below is a claim; it is the register walk, so the
next lane can write the sections without re-deriving them.

### 13.1 `0x80780D00` - 276 bytes

`fn_80780d00(this, kind: u32, pos: *const f32, extra: i32, scale: f32)`.
`stwu -64`; `f31` 48/56; `r28..r31` manual. `*(u32*)(this+0x270) = kind`;
`table = {4, 9, 10}` at `0x807CF7C4` (3 live words);
`0x8077E410(f1 = [0x807CF698] (0.0), r3 = this, r4 = table[kind], r5 = -1)`;
`*(f32*)(this+0x1D8/0x1DC/0x1E0) = pos[0..2]`; `f1 = 0x80780A90(this)`;
threshold `= [0x807CF674] (70.0) * [0x807CF7B0] (0.5)` = 35.0;
`r5 = (f1 < 35.0) ? 15 : 0`; delta `pos - *(this+0xAC/0xB0/0xB4)` at `r1+8`;
`r8 = *(u32*)(this+0x294)`; `f2 = f3 = *(f32*)(r8+0x18)`;
`0x800F69F0(r3 = this, r4 = pos, r5, r6 = extra, r7 = &delta, f1 = scale,
f2, f3)`. Caller `0x8077EE24` passes `kind = 0`, `pos = this+0x2AC`,
`extra = (s16)*(*(this+0x294)+0x1E)`, `f1 = *(f32*)(*(this+0x294)+0x14)`.
The `lfs f2,76(r3)` reads `0x807CF674` (70.0) - not `0x807CF7B0`; the
`fmuls f0,f2,f0` form is the 3-operand A-form.

### 13.2 `0x80711590` - 272 bytes

`fn_80711590(this)`; `stwu -48`; `f31` 32/40; `r30 = this`.
`if (*(u32*)(this+0x694) < [0x80805EA0]) 0x807103D0(this)` -- the comparison is
`[0x80805EA0]` as a *u32* (it holds `0x1D` in the shipped `.data`).
Snapshot `*(f32*)(this+0xAC)` into `r1+8` **before** that call.
`flag = ((g < c) && (snap < cur)) || ((c < g) && (cur < snap))` where
`g = *(f32*)(0x80805EA0+0x510)` (`0x808063B0`), `c = [0x807CCA7C]` (0.0) and
`cur = *(f32*)(this+0xAC)` read *after* the call. If `flag` and
`0x800B2EF0(0x80805EA0) < [0x808063DC] (0.5)` and
`*(u32*)(this+0x694) < [0x80805EA0]`, then
`vmethod(this, *(this+0x60)+0xD4)(this, 0x8082F788)`. `0x8082F788` is inside the
REL's zero-fill segment, i.e. `.bss`.

### 13.3 `0x80643D40` - 268 bytes

`fn_80643d40(this)`; `stwu -32`; `r28..r31` manual; `r6 = 0x80354E60`.
Four guards `if (*(u8*)(this+0x8D8+i) != 0) *(u8*)(*(u32*)(this + 0x2E4 +
4*[0x80354E60+4i]) + 0x269) = 1` for `i = 0..3`; then
`for (i = 0; i < 7; i++) { *(u8*)(*(u32*)(this+0x2F4+4i) + 0x209) = 0;
0x807678C0(); }`; then `*(u8*)(*(u32*)(this+0x310)+0x261) = 0`;
`0x8019CC70([0x8043A4A8])`. `0x80354E60..0x80354E6C` and `0x8043A4A8` are
main.dol cells (not mapped in this REL).

### 13.4 `0x806B39B0` - 268 bytes

`fn_806b39b0(this) -> u32`; `stwu -32`; `addi r11,r1,32`; `bl 0x802DCD70`
(r26-r31); returns in `r29`. Three 96-byte buckets (`this+0x3F0`, `+0x450`,
`+0x4B0`, limit `this+0x510`): walk each bucket's `+0x10` list via `+0xC`, take
`e = *(u32*)(node+4)`, then
`if (*(u8*)(e+0x38C) == 1) { id = (s8)*(u8*)vmethod(e, *(e+0x60)+0x6C);
if (id < 4) mask |= 1 << id; o = 0x8012DBE0(e); if (o != 0) { id2 =
(s8)*(u8*)vmethod(o, *(o+0x60)+0x6C); if (id2 < 4) mask |= 1 << id2; } }
else if (*(u8*)(e+0x38C) == 2) { ...single probe... }`.
Three distinct `bcctr` sites, all slot `+0x6C`.

---

## Ready vs conditional

| # | address | bytes | verdict | what is still open |
|--:|---:|---:|---|---|
| 1 | `0x80761510` | 340 | **ready (high)** | only `0x80166310`/`0x801662F0` contracts; all operands literal |
| 2 | `0x8078B1D0` | 340 | ready (medium) | meaning of the `a/b/c` float triple; four emitter contracts |
| 3 | `0x806B7890` | 336 | **conditional (medium)** | `0x80377DC8` is a main.dol zero-fill cell (values not shipped) |
| 4 | `0x806B7690` | 312 | ready (medium) | `0x8016C890` 5-arg contract; role of the `this+0xCC0` index array |
| 5 | `0x806A23B0` | 308 | **ready (high)** | nothing - gate, clamp, sign and store are all literal |
| 6 | `0x806DF630` | 308 | ready (medium) | `0x800DB180` 10-argument builder contract |
| 7 | `0x80722180` | 296 | ready (medium) | `0x80241710`/`0x80241AA0` decompose/recompose contract |
| 8 | `0x80780BD0` | 292 | **conditional (medium)** | 48-byte scratch written only by `0x8016ED10`; layout is a guess |
| 9 | `0x80780E20` | 292 | ready (medium) | `0x80782050` 2-word lookup; `!=` vs Ghidra's `>` |
| 10 | `0x80702AA0` | 276 | **ready (high)** | five literal copies; only `0x80701C10`'s contract |
| 11 | `0x80702BC0` | 276 | **ready (high)** | as §10, callback `0x80088E80` |
| 12 | `0x80724460` | 276 | ready (medium) | matrix helper contracts; in-place `0x801C0540` |

**Split: 10 ready, 2 conditional.** Ready: `0x80761510`, `0x8078B1D0`,
`0x806B7690`, `0x806A23B0`, `0x806DF630`, `0x80722180`, `0x80780E20`,
`0x80702AA0`, `0x80702BC0`, `0x80724460`. Four of those are high confidence
(`0x80761510`, `0x806A23B0`, `0x80702AA0`, `0x80702BC0`) and six medium.
Conditional: `0x806B7890` and `0x80780BD0`.

The two conditional ones share one cause each:

* `0x806B7890` - `0x80377DC8` lies in `main.dol`'s zero-fill segment
  (`0x80351680..0x80427680`, `filesz = 0`), so the shipped artifacts do not carry
  its value. The Rust can only read it by address; with `0.0` there the effect
  call would collapse to a degenerate record.
* `0x80780BD0` - the 48-byte basis object at `r1+0x28` is written only by
  `0x8016ED10` and consumed only by `0x801C1160`; its shape is a layout
  inference, exactly like wave-131 §3.

## Verification performed

* All twelve `bytes` values match `work/ghidra/rel-d_basesNP/export/functions.tsv`
  verbatim (`80643d40` was checked too, for §13.3), and all twelve export files
  exist under `work/ghidra/rel-d_basesNP/export/`.
* In all twelve ranges the last word is `0x4E800020` (`blr`) at exactly
  `entry + size - 4`, so no body is truncated by the TSV extent. The next TSV
  row starts 0, 8 or 12 bytes later (alignment padding or an unclaimed symbol),
  never inside the body - the `functions.tsv` "extent, not length" caveat in
  `docs/REL_ELF_MAPPING.md` costs nothing here.
* No address has a `game::bases` row in `ledger/functions.csv` (row-based test on
  comma field 2). Five appear only as free text in other rows' evidence columns,
  listed in "Selection rule" above.
* No `## N. \`0xADDR\`` heading in `docs/SPEC_WAVE128_BASES.md` ..
  `docs/SPEC_WAVE131_BASES.md` names any of the twelve, and none appears in
  `docs/ANALYSIS_WAVE127_BASES.md`'s class-C or class-D blocks.
* The three save-helper stubs used (`0x802DCD68`, `0x802DCD70`, `0x802DCD74`)
  and their restores (`0x802DCDB4`, `0x802DCDBC`, `0x802DCDC0`) were re-decoded
  from `work/dol/main.elf`; the register ranges in the Method table are read off
  the actual `stw` lists, not copied.
* All 56 `psq*` words in the batch are `qr0` with `W = 0`; a full-text scan of
  the twelve ranges finds no `mtspr` to `GQR0..GQR7`, only `LR` and `CTR`.
* No file in the repository was written except this one; the decoder and the
  body dumps live in `%TEMP%\opencode`.

## Hand-off

The remaining class-B pool after this wave is **58 addresses / 9 736 bytes**
(70 minus 12; 13 388 minus 3 652). Ranked next, with the same size-then-address
tiebreak, and with the four bodies already decoded in §13 marked:

| next | address | bytes | note |
|--:|---:|---:|---|
| 1 | `0x80780D00` | 276 | decoded, §13.1 |
| 2 | `0x80711590` | 272 | decoded, §13.2 |
| 3 | `0x80643D40` | 268 | decoded, §13.3 |
| 4 | `0x806B39B0` | 268 | decoded, §13.4 |
| 5 | `0x806FCDF0` | 268 | not opened |
| 6 | `0x807485D0` | 268 | not opened |
| 7 | `0x806329E0` | 264 | not opened (wave-111 paired-single rejection) |
| 8 | `0x8065C100` | 264 | not opened (wave-120 paired-single rejection) |
| 9 | `0x8065C260` | 264 | not opened (wave-120 paired-single rejection) |
| 10 | `0x806FFB50` | 256 | not opened (wave-11 wire-mesh, 7 calls) |
| 11 | `0x807241C0` | 256 | not opened (wave-125 paired-single rejection) |
| 12 | `0x807811D0` | 256 | not opened (`character_SV`, 1 call) |

Reference points for whoever takes them: `0x806B7340` and `0x80749750` are
already specified at `docs/SPEC_WAVE131_BASES.md` §13 and §14 and are **not** in
the 58.
