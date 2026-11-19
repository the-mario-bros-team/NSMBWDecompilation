# Wave 143 - implementation specification for 12 class-D `d_enemiesNP` bodies

## What this page is

Wave 127 (`docs/ANALYSIS_WAVE127_ENEMIES.md`) reclassified 384 formerly-skipped
`d_enemiesNP` addresses into 173 class **B**, 160 class C and 51 class D.
Waves 128-142 (`docs/SPEC_WAVE128_ENEMIES.md` .. `docs/SPEC_WAVE142_ENEMIES.md`)
specified the whole B tranche and the first three `qr3` tranches. A live re-walk
of the class-D block of the wave-127 table against `ledger/functions.csv`, every
`docs/*.md` heading and every `pub unsafe fn fn_<addr>` body in `src/` leaves
**14** unclaimed rows: twelve of them are taken here, `0x80AE2250` (108 B) is the
13th and stays for the next wave, and `0x80A337B0` is the 4-byte alias with no
body of its own (`docs/SPEC_WAVE140_ENEMIES.md:15-18` opener, same rule).

The block is drained after this wave: **one** `qr3` row (`0x80AE2250`) and the
4-byte alias remain.

This lane is the continuation of the `qr3` tranche. `docs/REL_ELF_MAPPING.md`
pinned **`GQR3 = 0x00050005`** (unsigned 16-bit, scale 0), so every
`psq_l ...,qr3` in the bodies below is exactly

```
fD = (f32)*(u16 *)rA
```

and the wave-127 blocker is gone. Nine of the twelve rows below are blocked only
by that load. The other three (`0x80A79810`, `0x80AEC4E0`, `0x80AEEFA0`) are
class D for the `extraout_` reason instead: each reads a register Ghidra never
defines, and in every case the register is an **incoming argument that the call
sites still stage** - the bodies decode end-to-end once the argument is taken
from the caller (rank 1) or from the ABI (ranks 2 and 3). Those three are the
next-largest class-D rows, so they are specified here rather than skipped.

This page is **read-only**: it writes no Rust, no ledger row and no other doc.

Each body was decoded from `work/rels/elf/d_enemiesNP.elf` with the `+0x1000`
segment shift of `docs/REL_ELF_MAPPING.md` (first `PT_LOAD` is
`p_offset = 0x10F0`, `p_vaddr = 0x80A000F0`), instruction by instruction from
the entry, following every conditional branch and every direct call until each
path ended in `blr`, a tail `b` or a tail call. Each body was cross-checked
against the Ghidra export at the cited path and against the `size` column of
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`. Constants that live inside
the REL image were read out of the ELF itself, so the integer and float values
below are the shipped values. Addresses outside the file-backed image are marked
**(ext)**; helper addresses below `0x80A00000` are `main.dol` functions.

## Selection

Ordering is by `functions.tsv` size, largest first. Every row below is absent
from the `address` column of `ledger/functions.csv` (checked against all 16,787
rows, read-only), absent from every heading of every `docs/*.md` (267 distinct
`80Axxxxx`/`80Bxxxxx` heading addresses were scanned) and has no
`fn_<addr>` definition in `src/`.

| # | address | TSV size | reachable insns | role | verdict |
|---:|---|---:|---:|---|---|
| 1 | `0x80A79810` | 360 | 90 | contact "push the other actor down" with a side-effect tail | **ready** |
| 2 | `0x80AEC4E0` | 292 | 73 | 32-entry table walk that rebuilds the `+0x5C4` record list | **conditional** |
| 3 | `0x80AEEFA0` | 292 | 73 | instruction-for-instruction twin of rank 2 on `+0x544` | **conditional** |
| 4 | `0x80A40CA0` | 208 | 52 | `daEnCoin_c` water-surface bob and drift | **ready** |
| 5 | `0x80AA3F50` | 200 | 50 | `4 / 4 / 32`-unit offset point from one heading word | **ready** |
| 6 | `0x80A7C600` | 184 | 46 | `daEnIbaramushi_c` sway pair around `1.0` | **ready** |
| 7 | `0x80B23480` | 180 | 45 | `daEnWaterPakkun_c` height bob plus pitch | **ready** |
| 8 | `0x80AB21D0` | 176 | 44 | `daEnKopone_c` state: contact message, yaw speed, sub-steps | **ready** |
| 9 | `0x80ABA950` | 156 | 39 | publish `radius*cos` / `radius*sin` of the own heading | **ready** |
| 10 | `0x80AFD170` | 156 | 39 | mode-driven heading step and `0.7 / 1.34` sine return | **ready** |
| 11 | `0x80ADDAC0` | 124 | 31 | `block_pata` height spring plus the frame tail | **ready** |
| 12 | `0x80ADDA40` | 120 | 30 | `block_pata` position commit from the `+0x888` block | **ready** |

**Split: 10 ready, 2 conditional.** Total 2,448 bytes of TSV extent and 612
reachable instructions; the reachable word count equals `size / 4` for all
twelve, so every extent is filled exactly and no row borrows its neighbour's
body.

Two rows were passed over and both are deliberate:

* `0x80B07D70` (252 B, `qr3`, the `daEnStarCoin_c` glide step) is absent from
  the ledger and from every heading but `src/game/enemies.rs` already contains a
  complete `pub unsafe fn fn_80b07d70` body
  (`src/game/enemies.rs:181444`), so it is not untranslated.
* `0x80AE2250` (108 B, `qr3`, the `0x80AE22xx` wing step) is the 13th by size
  and is left for the next tranche; it is the last `qr3` class-D row.

The two conditional rows are the only ones that need an external fact. Their
`extraout_` register is *not* the blocker: it is `r4`, the second argument, which
both call sites stage explicitly. The blockers are three unledgered `main.dol`
symbols that the body feeds (`0x8023D170`, `0x8023DC10` and the functor word at
`0x802F8FF0`), listed per row in the **Verdict** line.

## GQR3 and the qr3 pool (state at this wave)

`docs/REL_ELF_MAPPING.md` pins **`GQR3 = 0x00050005`** (unsigned 16-bit, scale
0), installed by `main.dol` at `0x8025EEEC`/`0x8025EEF0`
(`li r3,5` / `oris r3,r3,5` / `mtspr 915,r3`). The RAW stream in this REL holds
no `mtspr GQRn`, so every `psq_*` in this module depends on a `main.dol` word.

Every data `psq` in the nine `qr3` bodies below has the shape

```
psq_l fD,0(rA),1,qr3      ; W = 1 (single element), I = 3
```

which with `GQR3 = 0x00050005` is one unsigned 16-bit load scaled by `2^0`:

```
fD = (f32)*(u16 *)rA
```

The idiom is always "stage the halfword on the stack, then load it through a
pointer to it":

```
sth   rX,10(r1)          ; the s16/u16 field
addi  r3,r1,10
psq_l f1,0(r3),1,qr3     ; f1 = (f32)(u16)field
lfs   f0,..(rN)          ; 1/256
fmuls f1,f0,f1
bl    0x80237BD0         ; sin_angle16  (0x80237C40 is cos_angle16)
```

A Rust port reads the `u16` field directly and widens it; no paired-single
traffic is needed. The `psq_st/psq_l ...,qr0` pairs that appear in five of the
extents are ordinary `f31` stack spills (`docs/REL_ELF_MAPPING.md:53-55`).

Counts, read off the twelve extents (`qr3` sites / `qr0` spill pairs):

| address | `qr3` sites | `qr0` spill pairs | FPR save |
|---|---:|---:|---|
| `0x80A79810` | 0 | 1 | `f31` |
| `0x80AEC4E0` | 0 | 0 | none |
| `0x80AEEFA0` | 0 | 0 | none |
| `0x80A40CA0` | 1 | 1 | `f31` |
| `0x80AA3F50` | 2 | 1 | `f31` |
| `0x80A7C600` | 1 | 0 | none |
| `0x80B23480` | 2 | 0 | none |
| `0x80AB21D0` | 1 | 0 | none |
| `0x80ABA950` | 2 | 1 | `f31` |
| `0x80AFD170` | 1 | 1 | `f31` |
| `0x80ADDAC0` | 1 | 0 | none |
| `0x80ADDA40` | 1 | 0 | none |

## Reading conventions

* Synthetic base is `0x80A00000`. ELF file offset is
  `(address - 0x80A00000) + 0x1000`.
* The REL's file-backed image ends at `0x80B6E438` (five `PT_LOAD`s; the
  wave-141/142 pages quote `0x80B6E4B8` for the same boundary);
  anything above that is a `main.dol` address and is marked **(ext)**.
* `this` is the incoming `r3` unless stated otherwise; `+0xNNN` are byte
  offsets from `this`. `+0xAC`/`+0xB0`/`+0xB4` are x/y/z everywhere (the
  module's actor convention).
* `addi rX,r0,imm` is `li rX,imm`, and `addis rX,r0,imm` is `lis rX,imm`
  (`r0` is not read when `RA == 0`). `or rX,rY,rY` is `mr rX,rY`.
* On a `bc`/`bclr`, `bo = 12` branches on a true CR bit and `bo = 4` on a false
  one, and `bi = 0/1/2` is `LT/GT/EQ` of the addressed field; `bclr bo=20 bi=0`
  is `blr`; `bcctr bo=20 bi=0` with `LK` is `bctrl`.
* `cror 2,1,2` after an `fcmpo`/`fcmpu` rewrites `cr0.eq` as `gt|eq`, so the
  following `bc 4,2` reads `if !(a >= b)`; `cror 2,0,2` gives `lt|eq`. Both
  idioms appear below and are written in the C form that results.
* The paired-single opcodes in this build are `psq_l = 56` and `psq_st = 60`
  (`docs/REL_ELF_MAPPING.md:50`); the quantiser is bits 12-14 (counted from the
  LSB, i.e. `(word >> 12) & 7`), `W` is bit 15 (counted from the LSB, i.e.
  `(word >> 15) & 1`), and the displacement is the signed 12-bit field at bits
  0-11. For the `f31` spill pairs the displacement is simply the stack offset
  (24, 40, 56, 72 in the bodies below).
* A-form float ops: `fadds/fsubs` take their second source from the field at
  bits 16-20, `fmuls/fdivs` from the field at bits 21-25, and the four-operand
  `fmadds/fmsubs` family prints `fD,fA,fC,fB`. X-form ops (`fmr`, `fneg`,
  `fabs`, `fnabs`, `frsp`, `fctiwz`) use the 10-bit `XO` at bits 21-30.
* `0x80237BD0` is `sin_angle16(f1)` and `0x80237C40` is `cos_angle16(f1)`
  (`docs/SPEC_WAVE142_ENEMIES.md:153-155`); the angle is in 16-bit turn units
  and the pre-scale is `*(1/256)`, i.e. the helpers take a `0..255` "degree".
* TSV `size` is a bucket extent. All twelve bodies below fill their extent
  exactly and the next entry starts at `address + size`.
* A `fctiwz` + `stfd` + `lwz` + `sth` sequence is how the module converts an
  f32 to an `s16` field (rank 7 at `0x80B2350C`); only the low halfword of the
  converted integer reaches the field, i.e. `(s16)(int)value`.

## Symbolic helpers used below

Roles are the ledger's (`ledger/functions.csv`) or, where the helper is
unledgered, the role established by the cited wave.

| address | name / shape | role |
|---|---|---|
| `0x8013AAA0` | `fn_8013aaa0(actor: usize) -> u32` | ledgered `game::worldmap` (translated): returns the actor linked through `+0x1008` when bit `0x45` is set, else `0` |
| `0x80A79750` | `fn_80a79750(a: usize, b: usize)` | ledgered `game::enemies` (translated, 188 B): four-frame `E9A/E9E/E9F` flag, `EA9` countdown, SE. `a` and `b` are the **collision records**; `*(a+4)` and `*(b+4)` are the actors |
| `0x80A3F450` | `fn_80a3f450(this)` | ledgered `game::enemies` (translated): retire/limit/mode guard plus the `0x800777B0` map call |
| `0x80A79DF0` | `fn_80a79df0(this)` | ledgered `game::enemies` (translated): `0x80A79D90` first, `+0xEA6`/`+0xEA7` state ladder |
| `0x80A79630` | `sine_envelope(rec)` | unledgered 236 B, specified as rank 6 of `docs/SPEC_WAVE142_ENEMIES.md`: two-state `A*sin` / `1+A-sin` driver |
| `0x80AB1890` | `fn_80ab1890(this)` | ledgered: `+0x164 = v/4096`, `+0x7DC = -3v/4096` from `s16[+0x102]` |
| `0x80AB1A10` | `fn_80ab1a10(this)` | ledgered: three-argument contact probe, clears `+0x99A` |
| `0x80AB1EC0` | `fn_80ab1ec0(this)` | ledgered: visibility query `0x80096E60` then `fn_80ab1e00` |
| `0x80AB1DF0` | `fn_80ab1df0(this)` | ledgered: tail call through vtable slot `+0x1C` of the `+0x544` holder |
| `0x80AB1830` | `fn_80ab1830(this)` | ledgered: 64-frame countdown plus the `0x80078470` tail |
| `0x800955F0` | `fn_800955f0(this)` | ledgered `game::stage`: orbit update reading `+0x1EC`/`+0x440..0x448` |
| `0x80097170` | `fn_80097170(f1, this, ptr)` | ledgered `game::stage`: notice latch over `fn_80097250`, three parameters restored from the two call sites |
| `0x8006CDE0` | `fn_8006cde0(this)` | **unledgered** `main.dol`; the translated bodies in `src/game/enemies.rs` call it by address |
| `0x80237BD0` / `0x80237C40` | `sin_angle16(f1)` / `cos_angle16(f1)` | ledgered `resource::bres` |
| `0x802DCD74` / `0x802DCDC0` | `_savegpr_27` / `_restgpr_27` | ledgered `runtime::cpp`; saves/restores `r27..r31` at `r11-0x1C..-4`, does **not** touch `r3` |
| `0x802DCD6C` / `0x802DCDB8` | `_savegpr_25` / `_restgpr_25` | ledgered `runtime::cpp`; same convention for `r25..r31` |
| `0x80168400` | `fn_80168400(rec) -> u32` | ledgered `resource::registry`; casts the argument and reads `+0xE8` |
| `0x8023B8C0` | `mdl_section_30_count(it) -> u32` | ledgered `resource::bres`: count word at `dict+4` |
| `0x8025C660` | `fn_8025c660(dst, obj, i)` | ledgered `runtime::os`: 13-word activation-record gather |
| `0x8025C510` | `fn_8025c510(rec, 0) -> u32` | ledgered `runtime::os`: descriptor slot `+0x1C`, mark bit `0x20` |
| `0x8023D120` | `fn_8023d120(*mut u32, -1)` | ledgered `resource::bres` (`res_texcoord_set_index`): null guard, stores `0xFF` when the value exceeds `0x7F` |
| `0x8023B860` | `fn_8023b860(it, i) -> u32` | ledgered `resource::bres` (`mdl_section_30_by_index_alt`): entry at `dict+(i+1)*0x10+0x14` |
| `0x8023D170` | `fn_8023d170(*mut u32, -1)` | **unledgered** `main.dol`; paired with `0x8023D120` at both call sites |
| `0x8023DC10` | `fn_8023dc10(*mut u32, 0, *mut u32)` | **unledgered** `main.dol` |
| `0x802F8FF0` | functor word | **unledgered** `main.dol` address; passed as the second argument of a vtable slot-2 call |
| `0x80429DD0` | `*mut u8` global | `main.dol` scene/height manager; `+0x8FEC4` is the stage water level (same field waves 51, 84, 86 and 141 read) |
| `0x8042A100` | `*mut u8` global | `main.dol` water manager; `+0x8C0` is the horizontal drift float (`docs/COVERAGE_WAVE31_ENEMIES.md:252`) |

---

## 1. `0x80A79810` - contact "push the other actor down" with a side-effect tail

* **Address:** `0x80A79810`
* **TSV size:** 360 bytes
* **Reachable body:** 90 instructions, 90 words = the whole extent; 6 direct
  `bl`, 2 vtable `bctrl`, 7 conditional branches and 2 `b` joins, one `blr` at
  `0x80A79974`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a79810_FUN_80a79810.c`
  - the export is declared `void FUN_80a79810(void)` and reads
    `extraout_f1`; the RAW stream proves the float arrives in `f1` and is
    parked with `fmr f31,f1` at `0x80A79834` before any other use.
* **Proposed signature:**
  `pub unsafe fn fn_80a79810(a: usize, b: usize, amount: f32) -> bool`
  - `r3 = a` (collision record of the pusher; only forwarded),
    `r4 = b` (collision record of the target; `u32[b+4]` is its actor),
    `f1 = amount` (how hard to push).
* **Callers in this REL:** one `bl` from `0x80A79A88` inside the state body at
  `0x80A79980` (480 bytes, `0x80A79980..0x80A79B5F`, **not** in
  `functions.tsv`). That caller stages `r3 = a`, `r4 = b` and
  `f1 = f32[a+0x70]`, then does `if (fn(...) != 0) u8[a+0xA1] |= 2;` and skips
  its own state transition, so the boolean means "contact handled; the other
  actor has been displaced".
* **Constants:** one `lis r31,0x80b3` and one `lis r4,0x80b3` load:

| address | type | value / role |
|---|---|---|
| `0x80B2DE20` | f32 | `0.0` - the `amount > 0` threshold and the value written to `+0xEC` |
| `0x80B2E060` | f32 | `2.0` - gain applied to `amount` for the `y` push |

* **Fields:** `b+0x04` u32 target actor; actor `+0x60` vtable pointer (slot
  `+0xE0` = 56); actor `+0x0B0` f32 y (decremented here); actor `+0x0B4` f32 z
  (dead self-store); actor `+0x0EC` f32 (cleared to `0.0`); `a+0x70` f32 read by
  the caller only.
* **Verdict:** **ready** (confidence high) - every path decodes, both callees
  take explicit arguments, and the `extraout_f1` is the documented `f1`.

### Frame

`stwu r1,-80(r1)`; LR at `+84`; `addi r11,r1,64`; `stfd f31,64(r1)` +
`psq_st f31,72(r1),0,qr0` (the `f31` spill pair); `bl 0x802DCD74`
(`_savegpr_27`) saves `r27..r31` at `r1+0x30..0x3C`. Epilogue at
`0x80A79954..0x80A79974`: `addi r11,r1,64`, `psq_l f31,72(r1),0,qr0`,
`lfd f31,64(r1)`, `bl 0x802DCDC0` (`_restgpr_27`), restore LR, `addi r1,r1,80`,
`blr`.

### Behaviour

```
victim  = u32[b+0x04]
handled = 0

if (vslot56(victim) == 0 || fn_8013aaa0(victim) != 0) {
    if (amount > 0.0) {
        handled = 1
        if (vslot56(victim) == 0) {
            t = fn_8013aaa0(victim)
            if (t != 0) victim = t
            f32[victim+0x0B0] -= 2.0 * amount
            f32[victim+0x0B4]  = f32[victim+0x0B4]     # dead store
            f32[victim+0x0EC]  = 0.0
        } else {
            t = fn_8013aaa0(victim)
            if (t != 0) {
                f32[t+0x0B0] -= 2.0 * amount
                f32[t+0x0B4]  = f32[t+0x0B4]          # dead store
                f32[t+0x0EC]  = 0.0
            }
        }
    }
    if (handled != 0) fn_80a79750(a, b)                    # 0x80A79950
}
return handled
```

`vslot56(x)` is `(*(fn**)(u32[x+0x60] + 0xE0))(x)`.

### Notes

* The two arms differ only in null handling: when the vtable slot returns `0`
  the writes always happen (to `victim`, or to the linked actor when
  `fn_8013aaa0` returns one); when the slot returns non-zero they happen only
  if the linked actor is non-zero. The export flattens this into one `if`
  (`80a79810_FUN_80a79810.c:27-47`), which is why the two blocks look
  duplicated in C.
* `0x8013AAA0` is called at three static sites (`0x80A79860`, `0x80A798A8`,
  `0x80A798F4`); the condition calls it once and each arm re-calls it.
* The `+0x0EC` clear reloads `0.0` from `0x80B2DE20` inside each arm; the same
  word is also the `amount > 0` threshold, which is why the two uses share one
  constant slot in the export.
* The dead `stfs` pairs at `0x80A798D0/0x80A798D8` and `0x80A79924/0x80A7992C`
  spill the pre-push x/z to the stack and are never read; they are scratch
  write-through of the vector, not observable state.
* `victim` is dereferenced without a null guard in both arms (the record's
  `+4` word is assumed live), matching the sibling `fn_80a79750`.

---

## 2. `0x80AEC4E0` - 32-entry table walk that rebuilds the `+0x5C4` record list

* **Address:** `0x80AEC4E0`
* **TSV size:** 292 bytes
* **Reachable body:** 73 instructions, 73 words = the whole extent; 9 direct
  `bl`, one vtable `bctrl`, one loop back edge at `0x80AEC5E8` and one
  forward `b` at `0x80AEC518`, one `blr` at `0x80AEC600`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aec4e0_FUN_80aec4e0.c`
  - the export is declared `void FUN_80aec4e0(void)` and reads
    `extraout_r4`; the RAW stream shows the argument in `r4` and never reads
    `r3` after the prologue (`or r25,r4,r4` at `0x80AEC4F4`, then
    `or r3,r25,r25` before the first call).
* **Proposed signature:** `pub unsafe fn fn_80aec4e0(rec: usize)`
  - the observed ABI is `r3 = this` (ignored) and `r4 = rec`; a port may drop
    the unused `r3`.
* **Callers in this REL:** one `bl` from `0x80AEBF38` inside the constructor
  body at `0x80AEBE90` (frame `-112`, **not** in `functions.tsv`; preceded by a
  zero word at `0x80AEBE8C`). That constructor stages
  `r3 = this`, `r4 = this+0x5C4` and has just bound a 32-entry block at
  `this+0x5A4` with `0x80169D90(this+0x5C4, &word, this+0x5A4, 0x20, 1, 0)`.
* **Constants:** no float constants; the only literal words are the functor and
  the `-1` tag:

| address | type | value / role |
|---|---|---|
| `0x802F8FF0` | **(ext)** | `main.dol` functor word, passed by address as the second argument of the vtable slot-2 call |
| `0xFFFFFFFF` | u32 | tag stored in the descriptor block |
| `0x000003F0` | u32 | offset added to the `0x8023B860` result before it is stored in the descriptor |

* **Fields:** `rec+0x04` u32 object pointer (its `+0x00` is the vtable); the
  object's vtable slot 2 (`+0x08`).
* **Verdict:** **conditional** (confidence medium) - the body is exact and the
  `extraout_r4` is the documented `r4`, but three `main.dol` symbols it calls
  are unledgered (`0x8023D170`, `0x8023DC10` and the functor `0x802F8FF0`), and
  the meaning of the 8-byte descriptor `{ base+0x3F0, 0xFFFFFFFF }` has to be
  established before the port can be reviewed.

### Frame

`stwu r1,-128(r1)`; LR at `+132`; `addi r11,r1,128`; `bl 0x802DCD6C`
(`_savegpr_25`) saves `r25..r31` at `r1+0x64..0x7C`. Stack locals:

| slot | use |
|---|---|
| `r1+0x08` | functor word `0x802F8FF0` |
| `r1+0x0C..0x0F` | four `0xFF` bytes (cleared/`-1` tag bytes) |
| `r1+0x10` | `0xFFFFFFFF` (descriptor tag, also the `r5` argument) |
| `r1+0x14` | `base + 0x3F0` (the `r3` argument) |
| `r1+0x18` | value returned by `0x8025C510` |
| `r1+0x1C` | iterator returned by `0x80168400` |
| `r1+0x20..0x3F` | 32-byte activation record built by `0x8025C660` |

Epilogue at `0x80AEC5EC..0x80AEC600`: `addi r11,r1,128`, `bl 0x802DCDB8`
(`_restgpr_25`), restore LR, `addi r1,r1,128`, `blr`.

### Behaviour

```
it = fn_80168400(rec)                        # 0x80AEC4FC
i  = 0
while (i < fn_8023B8C0(&it)) {               # 0x80AEC5DC / 0x80AEC5E4
    obj = u32[rec+0x04]
    ok = 0
    if (obj != 0) {
        functor = 0x802F8FF0
        ok = vslot2(obj)(obj, &functor) != 0     # bctrl at 0x80AEC544
    }
    if (!ok) obj = 0

    fn_8025C660(r1+0x20, obj, i)             # 32-byte record
    v = fn_8025C510(r1+0x20, 0)
    fn_8023D120(&v, -1)
    fn_8023D170(&v, -1)

    base = fn_8023B860(&it, i)
    d.tag   = 0xFFFFFFFF                     # r1+0x10
    d.value = base + 0x3F0                   # r1+0x14
    fn_8023DC10(&d.value, 0, &d.tag)         # r3=&d.value, r4=0, r5=&d.tag
    i++
}
```

`vslot2(x)` is `(*(fn**)(u32[x]))[2](x, arg)`.

### Notes

* The call at `0x80AEC544` sets `r3 = obj`, `r4 = r1+8` (the functor word) and
  `CTR = *(u32[obj] + 8)`; the return is compared against zero and becomes the
  `ok` flag. When `ok` is false the record is built with a null object rather
  than being skipped - the loop count is unaffected.
* `stb r31,12..15(r1)` writes four `0xFF` bytes with `r31 = 0xFF`, then
  `lwz r0,12(r1)` reloads them as `0xFFFFFFFF`; only that word is used.
* `0x3F0` (1008) is added to the element returned by `0x8023B860(&it, i)`
  before the descriptor is handed to `0x8023DC10`; that bias is the fact the
  conditional verdict is about.
* `r3` is dead on entry; there is no `this`-relative access anywhere in the
  body.

---

## 3. `0x80AEEFA0` - instruction-for-instruction twin of rank 2 on `+0x544`

* **Address:** `0x80AEEFA0`
* **TSV size:** 292 bytes
* **Reachable body:** 73 instructions, 73 words = the whole extent; the same 9
  `bl`, one vtable `bctrl`, the same loop at `0x80AEF0A8`, one `blr` at
  `0x80AEF0C0`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aeefa0_FUN_80aeefa0.c`
  - `void FUN_80aeefa0(void)` with `extraout_r4`, exactly like rank 2.
* **Proposed signature:** `pub unsafe fn fn_80aeefa0(rec: usize)`
* **Callers in this REL:** one `bl` from `0x80AEEA0C` inside the constructor at
  `0x80AEE980` (frame `-32`, **not** in `functions.tsv`), which stages
  `r3 = this`, `r4 = this+0x544` after binding the 32-entry block at
  `this+0x524` (`0x80169D90(this+0x544, &word, this+0x524, 0x20, 1, 0)`).
* **Constants:** identical list to rank 2 (`0x802F8FF0` **(ext)**,
  `0xFFFFFFFF`, `0x3F0`).
* **Fields:** `rec+0x04` u32 object pointer; the object's vtable slot 2.
* **Verdict:** **conditional** (confidence medium) - same three unledgered
  symbols and the same descriptor question as rank 2.

### Frame and behaviour

Identical to rank 2; only the branch displacements differ by `0xAC0`
(`b 0x80AEF09C` / `bc 12,0,0x80AEEFDC` instead of `0x80AEC5DC` /
`0x80AEC51C`). The 292-byte streams are otherwise instruction-for-instruction
the same sequence, so a lane can implement one body and call it from both
constructors once the descriptor is understood.

### Notes

* The pair is the module's instantiation of one template for two sibling
  records: `+0x5A4`/`+0x5C4` (rank 2's caller) and `+0x524`/`+0x544` (rank 3's
  caller). Both record offsets are exactly `0x80` lower in the second caller,
  so the layout is identical with the whole block moved down by `0x80`.
* Do **not** treat the two bodies as interchangeable by address: each is called
  with its own record, and the `+0x3F0` bias is applied to the iterator base,
  not to the record.

---

## 4. `0x80A40CA0` - `daEnCoin_c` water-surface bob and drift

* **Address:** `0x80A40CA0`
* **TSV size:** 208 bytes
* **Reachable body:** 52 instructions, 52 words = the whole extent; 2 direct
  `bl` (`0x80A3F450`, `0x80237BD0` sin), one conditional branch at
  `0x80A40CEC`, one `blr` at `0x80A40D6C`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a40ca0_FUN_80a40ca0.c`
  - one `qr3` load at `0x80A40D20`; the `qr0` pair is the `f31` prologue spill.
  The export's `unaff_GQR3`/`ldexpf` block is the paired-single recoverable
  noise; the raw stream is `sth` + `addi r4,r1,8` + `psq_l f1,0(r4),1,qr3`.
* **Proposed signature:** `pub unsafe fn fn_80a40ca0(this: usize)`
* **Callers in this REL:** no `bl`; one state-table slot at `0x80B3F68C` =
  `0x80A40CA0`. The surrounding `{0xFFFFFFFF, fn, 0}` triples
  (`0x80B3F680` = `0x80A40C90`, `0x80B3F68C` = this body, `0x80B3F698` =
  `0x80A40C70`) belong to the **`daEnCoin_c`** table whose name block starts at
  `0x80B3F708` (`daEnCoin_c::StateID_Move`, `::StateID_WaitDown`,
  `::StateID_Swim`, ...), the same table wave 142 rank 8 reads at
  `0x80B3F59C`.
* **Constants:** `lis r31,0x80b3` plus `lfs f0,-14996(r31)` and
  `lfs f0,-14836(r3)`:

| address | type | value / role |
|---|---|---|
| `0x80B2C56C` | f32 | `4.0` - margin below the water line and amplitude of the bob |
| `0x80B2C60C` | f32 | `0.00390625` = `1/256`, turn-units pre-scale |
| `0x80429DD0+0x8FEC4` | f32 (via **(ext)** pointer) | stage water level |
| `0x8042A100+0x8C0` | f32 (via **(ext)** pointer) | horizontal drift added to x |

* **Fields:** `+0x102` `s16` pitch (advances `0x400` per frame); `+0x0AC` f32 x;
  `+0x0B0` f32 y; `+0x0A84` `s16` bob phase (advances `0x400` per frame).
* **Verdict:** **ready** (confidence high) - both external reads are `main.dol`
  *data* words whose role is already recorded (`docs/COVERAGE_WAVE86_ENEMIES.md:54`,
  `docs/COVERAGE_WAVE31_ENEMIES.md:252`), and the whole body decodes.

### Frame

`stwu r1,-48(r1)`; LR at `+52`; `stfd f31,32(r1)` +
`psq_st f31,40(r1),0,qr0`; `r31/r30` at `+28/+24`; `r30 = this`. Epilogue
`0x80A40D50..0x80A40D6C`: `psq_l f31,40(r1),0,qr0`, `lfd f31,32(r1)`, restore
`r31/r30`, restore LR, `addi r1,r1,48`, `blr`.

### Behaviour

```
s16[this+0x102] = (s16)(s16[this+0x102] + 0x400)
fn_80a3f450(this)                                   # 0x80A40CCC, r3 = this

water = f32[ *(u32*)0x80429DD0 + 0x8FEC4 ]
if (water >= f32[this+0x0B0]) {                     # bc 4,2 skip when water < y
    f32[this+0x0AC] += f32[ *(u32*)0x8042A100 + 0x8C0 ]

    a = s16[this+0x0A84]
    f32[this+0x0B0] = (water - 4.0)
                      + 4.0 * sin_angle16((f32)(u16)a * (1.0/256.0))
    s16[this+0x0A84] = (s16)(a + 0x400)
}
```

### Notes

* The phase is advanced **after** the sine is taken, so frame *n* uses the
  value stored by frame *n-1*; the first frame after construction uses the
  seed stored at `+0xA84`.
* The `fcmpo`/`cror 2,1,2`/`bc 4,2` sequence at `0x80A40CE4..0x80A40CEC` is
  `if (water >= y)`, i.e. the body runs while the coin is at or below the
  surface. A coin above the water keeps its y and does not advance `+0xA84`.
* `+0x102` advances every frame regardless of the water test; it is the model
  pitch, not the bob.
* The y target is `water - 4.0` (the coin floats four units below the surface)
  and the bob is a full `±4.0` about it, so the coin grazes the surface.
* `4.0` is used twice from the same word (`0x80B2C56C`): once as the margin
  (loaded before the branch) and once as the sine amplitude (loaded after the
  call).
* The `unaff_GQR0` block at the head/tail of the export is the `f31` spill
  pair, not paired-single traffic.

---

## 5. `0x80AA3F50` - `4 / 4 / 32`-unit offset point from one heading word

* **Address:** `0x80AA3F50`
* **TSV size:** 200 bytes
* **Reachable body:** 50 instructions, 50 words = the whole extent; 2 direct
  `bl` (`0x80237C40` cos at `0x80AA3F9C`, `0x80237BD0` sin at `0x80AA3FBC`), no
  branch, one `blr` at `0x80AA4014`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aa3f50_FUN_80aa3f50.c`
  - two `qr3` loads at `0x80AA3F90` and `0x80AA3FB0` (both `W = 1`, the same
    staged `s16`); the `qr0` pair is the `f31` prologue spill. The export's
    `CONCAT22`/`CONCAT31` chains are the lost `sth`.
* **Proposed signature:**
  `pub unsafe fn fn_80aa3f50(this: usize, out: *mut f32)`
  - `r3 = this`, `r4 = out` (three consecutive f32 written).
* **Callers in this REL:** two `bl` sites:
  - `0x80AA4098` inside `0x80AA4070` (444 B, the wave-140 rank 7 "four-corner
    contact probe around a swept box"), and
  - `0x80AA5464` inside `0x80AA5390` (276 B).
* **Constants:** `lis r30,0x80b3` + `addi r30,r30,-2904` (= `0x80B2F4A8`):

| address | type | value / role |
|---|---|---|
| `0x80B2F534` | f32 | `0.00390625` = `1/256`, turn-units pre-scale |
| `0x80B2F554` | f32 | `4.0` - horizontal offset radius |
| `0x80B2F558` | f32 | `32.0` - z offset (upwards) |

* **Fields:** `+0x0A4E` `s16` heading word; `+0x0AC/0x0B0/0x0B4` f32 x/y/z
  (read only); `out[0..2]` f32 (written).
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-48(r1)`; LR at `+52`; `stfd f31,32(r1)` +
`psq_st f31,40(r1),0,qr0`; `r31/r30/r29/r28` at `+28/+24/+20/+16`;
`r28 = this`, `r29 = out`. Epilogue `0x80AA3FF0..0x80AA4014`: `psq_l f31,40`,
`lfd f31,32`, restore the four GPRs, restore LR, `addi r1,r1,48`, `blr`.

### Behaviour

```
a = s16[this+0x0A4E]
c = cos_angle16((f32)(u16)a * (1.0/256.0))
s = sin_angle16((f32)(u16)a * (1.0/256.0))

out[0] = f32[this+0x0AC] + 4.0 * c
out[1] = f32[this+0x0B0] + 4.0 * s
out[2] = f32[this+0x0B4] + 32.0
```

### Notes

* The same `s16` is staged twice (`sth r31,10(r1)` at `0x80AA3F8C` for the cos,
  `sth r31,8(r1)` at `0x80AA3FA8` for the sin); `r31` is the `lha` of `+0xA4E`
  and is not modified.
* `f31` parks the `4.0 * cos` product across the sin call; the spill pair is
  cosmetic.
* z gets a constant `+32.0`, not a trig term - the point is a horizontal disc
  offset with a fixed height above the actor. The same body is the "publish a
  point 4 units ahead and 32 up" helper the wave-140 rank 7 probe wraps.
* `+0xA4E` is the same heading word the rank-5 sibling `0x80AA37E0` (wave 142)
  reads to build its 32-point trail, so this body is the single-point version of
  that generator.

---

## 6. `0x80A7C600` - `daEnIbaramushi_c` sway pair around `1.0`

* **Address:** `0x80A7C600`
* **TSV size:** 184 bytes
* **Reachable body:** 46 instructions, 46 words = the whole extent; 6 direct
  `bl` (`0x80A79DF0`, `0x8006CDE0`, `0x800955F0`, `0x80097170`, `0x80A79630`,
  `0x80237BD0` sin), no branch, one `blr` at `0x80A7C6B4`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a7c600_FUN_80a7c600.c`
  - one `qr3` load at `0x80A7C670`; no `qr0` pair. The export's `local_18`
    `CONCAT22` is the lost `sth` of the new `+0xE98`.
* **Proposed signature:** `pub unsafe fn fn_80a7c600(this: usize)`
* **Callers in this REL:** no `bl`; one state-table slot at `0x80B4CC00` =
  `0x80A7C600` in the **`daEnIbaramushi_c`** table (neighbours
  `0x80B4CBF4` = `0x80A7C5F0`, `0x80B4CC0C` = `0x80A7C5E0`; name block
  `0x80B4CC34` `daEnIbaramushi_c::StateID_Wait`, `::StateID_Walk`,
  `::StateID_DieFumi`, ...).
* **Constants:** `lis r31,0x80b3` + `addi r31,r31,-8720` (= `0x80B2DDF0`):

| address | type | value / role |
|---|---|---|
| `0x80B2DE24` | f32 | `0.00390625` = `1/256`, turn-units pre-scale |
| `0x80B2DE28` | f32 | `1.0` - neutral scale, also the `f1` argument of `0x80097170` |
| `0x80B2E160` | f32 | `0.125` - sway amplitude |

* **Fields:** `+0x0AC` f32 x (passed to `0x80097170`); `+0x6AC/0x6B0` f32 scale
  pair (written); `+0xE64` sine-envelope record (`0x80A79630`); `+0xE98` `s16`
  sway phase; `+0x100` `s16` heading.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-32(r1)`; LR at `+36`; `r31/r30/r29` at `+28/+24/+20`; `r29 = this`.
Epilogue `0x80A7C69C..0x80A7C6B4`: restore the three GPRs, restore LR,
`addi r1,r1,32`, `blr`.

### Behaviour

```
fn_80a79df0(this)                                   # 0x80A7C624

s16[this+0x0E98] = (s16)(s16[this+0x0E98] + 0x800)  # 1/8 turn per frame
s16[this+0x0100] = (s16)(s16[this+0x0100] + 0x300)

fn_8006cde0(this)                                   # 0x80A7C644
fn_800955f0(this)                                   # 0x80A7C64C
fn_80097170(1.0, this, this + 0x0AC)                # 0x80A7C65C
fn_80a79630(this + 0x0E64)                          # 0x80A7C664

a = s16[this+0x0E98]                                # the value just written
t = 0.125 * sin_angle16((f32)(u16)a * (1.0/256.0))

f32[this+0x6AC] = 1.0 + t
f32[this+0x6B0] = 1.0 - t
```

### Notes

* `r30` holds the post-increment `+0xE98` value and is the value staged for the
  `qr3` load, so the sine tracks the new phase.
* The two scales sum to `2.0` and stay in `[0.875, 1.125]`; they are a
  squash/stretch pair in antiphase, the same shape wave 142 rank 10 uses
  (`docs/SPEC_WAVE142_ENEMIES.md:810-831`) with a different amplitude.
* `0x80097170` is called with `f1 = 1.0` (the word at `0x80B2DE28`), `r3 = this`
  and `r4 = this+0xAC`; the notice latch is therefore fed this actor's position
  with unit weight.
* `fn_80a79630` is the unledgered two-state sine envelope specified as wave 142
  rank 6; the record it drives is `this+0xE64`, a 12-byte block with
  `{u32 cfg; f32 out; s16 phase; u8 state}`.

---

## 7. `0x80B23480` - `daEnWaterPakkun_c` height bob plus pitch

* **Address:** `0x80B23480`
* **TSV size:** 180 bytes
* **Reachable body:** 45 instructions, 45 words = the whole extent; 2 direct
  `bl` (both `0x80237BD0` sin), no branch, one `blr` at `0x80B23530`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b23480_FUN_80b23480.c`
  - two `qr3` loads at `0x80B234B4` and `0x80B234EC`; no `qr0` pair. The
    `CONCAT22`/`CONCAT31` chains are the lost `sth`s.
* **Proposed signature:** `pub unsafe fn fn_80b23480(this: usize)`
* **Callers in this REL:** five `bl` sites, all inside state bodies of the same
  class:
  - `0x80B23A1C` in `0x80B23960` (304 B),
  - `0x80B23B28` in `0x80B23AB0` (248 B),
  - `0x80B23C9C` in `0x80B23BE0` (304 B),
  - `0x80B23DA8` in `0x80B23D30` (248 B),
  - `0x80B23EBC` in `0x80B23E80` (128 B).

  Those five entries are the slots at `0x80B6CD14`, `0x80B6CD38`, `0x80B6CD5C`,
  `0x80B6CD80`, `0x80B6CDA4`, and the table's name block is the
  **`daEnWaterPakkun_c`** list at `0x80B6CDD8`
  (`daEnWaterPakkun_c::StateID_FooRise`, `::StateID_FooFall`, ...).
* **Constants:** `lis r31,0x80b3` + `addi r31,r31,10368` (= `0x80B32880`):

| address | type | value / role |
|---|---|---|
| `0x80B3290C` | f32 | `0.00390625` = `1/256`, turn-units pre-scale |
| `0x80B328F4` | f32 | `3.0` - bias subtracted from `+0x5C0` |
| `0x80B32910` | f32 | `16384.0` - pitch centre (`0x4000` in f32) |
| `0x80B32914` | f32 | `4096.0` - pitch amplitude (`0x1000` in f32) |

* **Fields:** `+0x0B0` f32 y (written); `+0x102` `s16` pitch (written);
  `+0x5BC` `s16` height phase (advances `0x300`); `+0x5BE` `s16` pitch phase
  (advances `0x200`); `+0x5C0` f32 height base.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-32(r1)`; LR at `+36`; `r31/r30` at `+28/+24`; `r30 = this`. Epilogue
`0x80B2351C..0x80B23530`: restore `r31/r30`, restore LR, `addi r1,r1,32`,
`blr`. `r1+8` and `r1+10` are the two `sth` staging slots.

### Behaviour

```
s16[this+0x5BC] = (s16)(s16[this+0x5BC] + 0x300)
s1 = sin_angle16((f32)(u16)s16[this+0x5BC] * (1.0/256.0))

f32[this+0x0B0] = (f32[this+0x5C0] - 3.0) + s1

s16[this+0x5BE] = (s16)(s16[this+0x5BE] + 0x200)
s2 = sin_angle16((f32)(u16)s16[this+0x5BE] * (1.0/256.0))

s16[this+0x102] = (s16)(int)(16384.0 + 4096.0 * s2)
```

### Notes

* The y write happens between the two sine calls (the `0x80B234E4` store sits
  between the `0x80B234C0` and `0x80B234F8` calls), so the height uses the
  `+0x5BC` phase only.
* The pitch is `fctiwz` + `stfd`/`lwz` + `sth`: only the low halfword of the
  converted integer reaches `+0x102`, and the amplitude `4096.0` is exactly
  `0x1000` in Q16 turn units.
* The y base is `+0x5C0` (the "rest height") minus a fixed `3.0` sink, so the
  body bobs about a point three units below the authored height.
* Phases advance by `0x300` (6.75 deg) and `0x200` (4.5 deg) per frame, i.e. an
  ~85-frame and a 128-frame period.

---

## 8. `0x80AB21D0` - `daEnKopone_c` state: contact message, yaw speed, sub-steps

* **Address:** `0x80AB21D0`
* **TSV size:** 176 bytes
* **Reachable body:** 44 instructions, 44 words = the whole extent; 7 direct
  `bl`, one vtable `bctrl`, two conditional branches (`0x80AB21EC`,
  `0x80AB2250`), one `blr` at `0x80AB227C`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ab21d0_FUN_80ab21d0.c`
  - one `qr3` load at `0x80AB2218`; no `qr0` pair.
* **Proposed signature:** `pub unsafe fn fn_80ab21d0(this: usize)`
* **Callers in this REL:** no `bl`; one state-table slot at `0x80B54F98` =
  `0x80AB21D0`. The neighbours are `0x80B54F68` = `0x80AB2040`,
  `0x80B54F74` = `0x80AB2050`, `0x80B54F80` = `0x80AB2030`,
  `0x80B54F8C` = `0x80AB21C0`; the name block is the **`daEnKopone_c`** list at
  `0x80B55038` (`daEnKopone_c::StateID_Swim`, `::StateID_Turn`,
  `::StateID_Attack_wait`, `::StateID_Die`, ...).
* **Constants:** three `lis` loads:

| address | type | value / role |
|---|---|---|
| `0x80B2FB48` | f32 | `0.00390625` = `1/256`, turn-units pre-scale |
| `0x80B54BA0` | f32 | `0.30000001192092896` - yaw speed gain |
| `0x80B74FD0` | **(ext)** | `main.dol` object handed to vtable slot `+0xD4` |

* **Fields:** `+0x60` vtable pointer (slot `+0xD4` = 53); `+0x102` `s16` yaw
  (read only); `+0x0E8` f32 yaw speed (written).
* **Verdict:** **ready** (confidence high) - the only external item is the
  `(ext)` argument word, which is a constant in the shipped code.

### Frame

`stwu r1,-32(r1)`; LR at `+36`; `r31` at `+28`; `r31 = this`; `r1+8` is the
`sth` staging slot. Epilogue `0x80AB226C..0x80AB227C`: restore `r31`, restore
LR, `addi r1,r1,32`, `blr`.

### Behaviour

```
if (fn_80ab1a10(this) != 0)                        # 0x80AB21E4
    vslot53(this)(this, (void*)0x80B74FD0)         # bctrl at 0x80AB2208

a = s16[this+0x102]
f32[this+0x0E8] = 0.3 * sin_angle16((f32)(u16)a * (1.0/256.0))

fn_800955f0(this)                                  # 0x80AB2240
if (fn_80ab1ec0(this) == 0) {                      # 0x80AB2248
    fn_80ab1890(this)                              # 0x80AB2258
    fn_80ab1df0(this)                              # 0x80AB2260
    fn_80ab1830(this)                              # 0x80AB2268
}
```

`vslot53(x)` is `(*(fn**)(u32[x+0x60] + 0xD4))(x, arg)`.

### Notes

* The contact message is sent **only** when the probe returns non-zero; it is
  the "I bumped into something" notification with the `(ext)` record that
  `main.dol` expects at `0x80B74FD0`.
* The yaw speed is a pure function of the current heading (no ramp), with a
  `0.3` gain - `+0x102` is not advanced here, so the caller owns the yaw.
* The three sub-steps are the "visible / alive" branch: `0x80AB1890` republishes
  the two direction fields, `0x80AB1DF0` tails through the `+0x544` holder's
  vtable slot `+0x1C`, and `0x80AB1830` runs the 64-frame effect countdown.
  When `fn_80ab1ec0` reports the actor is hidden/out of range, none of them run.
* `0x80AB1A10` is called before any state is read, so a probe that returns zero
  leaves `+0xE8` untouched for that frame.

---

## 9. `0x80ABA950` - publish `radius*cos` / `radius*sin` of the own heading

* **Address:** `0x80ABA950`
* **TSV size:** 156 bytes
* **Reachable body:** 39 instructions, 39 words = the whole extent; 2 direct
  `bl` (`0x80237C40` cos at `0x80ABA98C`, `0x80237BD0` sin at `0x80ABA9AC`), no
  branch, one `blr` at `0x80ABA9E8`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aba950_FUN_80aba950.c`
  - two `qr3` loads at `0x80ABA97C` and `0x80ABA9A0`; the `qr0` pair is the
    `f31` prologue spill.
* **Proposed signature:** `pub unsafe fn fn_80aba950(this: usize)`
* **Callers in this REL:** one `bl` from `0x80ABAC00` inside `0x80ABAB50`
  (236 B, the wave-142 rank 7 `daEnLiftRotHalf_c` body, which documents this
  helper in its table at `docs/SPEC_WAVE142_ENEMIES.md:175`).
* **Constants:** `lis r31,0x80b3` + `lfs f0,-96(r31)`:

| address | type | value / role |
|---|---|---|
| `0x80B2FFA0` | f32 | `0.00390625` = `1/256`, turn-units pre-scale |

* **Fields:** `+0x104` `s16` heading (read only); `+0x5F8` f32 radius;
  `+0x5D0/0x5D4` f32 direction pair (written).
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-48(r1)`; LR at `+52`; `stfd f31,32(r1)` +
`psq_st f31,40(r1),0,qr0`; `r31/r30` at `+28/+24`; `r30 = this`. Epilogue
`0x80ABA9C4..0x80ABA9E8`: `psq_l f31,40`, `lfd f31,32`, restore `r31/r30`,
restore LR, `addi r1,r1,48`, `blr`. The two `stfs` at `0x80ABA9D8/0x80ABA9DC`
sit **after** the restore and write the results to the dead stack slots
`r1+0x10`/`r1+0x14`; they are not observable.

### Behaviour

```
a = s16[this+0x104]
c = cos_angle16((f32)(u16)a * (1.0/256.0))
s = sin_angle16((f32)(u16)a * (1.0/256.0))

f32[this+0x5D4] = f32[this+0x5F8] * s
f32[this+0x5D0] = f32[this+0x5F8] * c
```

### Notes

* The stream forms the `radius * sin` product first (`0x80ABA9B4`) and the
  `radius * cos` product second (`0x80ABA9B8`), then stores `+0x5D4` before
  `+0x5D0`; the values are the two components above.
* `f31` holds the cos result across the sin call (the store to `+0x5D0` at
  `0x80ABA9C0` consumes it after the sin returns).
* `+0x5D0`/`+0x5D4` are the pair the `daEnLiftRotHalf_c` body reads back and
  the values it leaves in `f1` for the vtable slot-3 call, so a port must keep
  them bit-identical.
* Nothing writes z or the radius; the helper only mirrors the radius along the
  heading.

---

## 10. `0x80AFD170` - mode-driven heading step and `0.7 / 1.34` sine return

* **Address:** `0x80AFD170`
* **TSV size:** 156 bytes
* **Reachable body:** 39 instructions, 39 words = the whole extent; one direct
  `bl` (`0x80237BD0` sin at `0x80AFD1EC`), four conditional branches
  (`0x80AFD194`, `0x80AFD19C`, `0x80AFD1BC`, `0x80AFD1C4`), one `blr` at
  `0x80AFD208`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80afd170_FUN_80afd170.c`
  - one `qr3` load at `0x80AFD1E0`; the `qr0` pair is the `f31` prologue spill.
    The export returns `double`; the value is an f32 in `f1`.
* **Proposed signature:** `pub unsafe fn fn_80afd170(this: usize) -> f32`
* **Callers in this REL:** one `bl` from `0x80AFD940` inside `0x80AFD910`
  (388 B, the wave-140 rank 9 "fall onto a `+0x604` row table and bank the
  model" body), which uses the return value as the per-frame x delta.
* **Constants:** `lis r6,0x80b3` + `addi r6,r6,5968` (= `0x80B31750`):

| address | type | value / role |
|---|---|---|
| `0x80B31778` | `s16[4]` | `{0xFC00, 0x0800, 0xFA00, 0x0400}` = `{-1024, 2048, -1536, 1024}` - step table for `+0x604 != 0` |
| `0x80B317C0` | f32 | `0.699999988079071` - amplitude for `+0x608 == 0` |
| `0x80B317C4` | f32 | `1.340000033378601` - amplitude for `+0x608 != 0` |
| `0x80B317C8` | f32 | `0.00390625` = `1/256`, turn-units pre-scale |

* **Fields:** `+0x604` u32 mode (0 = default step `0x600`); `+0x608` u32
  amplitude selector; `+0x5C8` `s16` phase (accumulated here).
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-32(r1)`; LR at `+36`; `stfd f31,16(r1)` +
`psq_st f31,24(r1),0,qr0`; no GPR save. Epilogue `0x80AFD1F4..0x80AFD208`:
`psq_l f31,24`, `lfd f31,16`, restore LR, `addi r1,r1,32`, `blr`.

### Behaviour

```
mode = u32[this+0x604]
step = (mode == 0) ? 0x600
                  : s16[0x80B31778 + (((mode - 1) << 1) & 6)]
                     # == s16[0x80B31778 + ((mode - 1) & 3) * 2]

amp  = (u32[this+0x608] == 0) ? f32[0x80B317C0] : f32[0x80B317C4]

s16[this+0x5C8] = (s16)(s16[this+0x5C8] + step)
return amp * sin_angle16((f32)(u16)s16[this+0x5C8] * (1.0/256.0))
```

### Notes

* The table read is `lhax r5,r4,r0` with `r4 = 0x80B31778` and
  `r0 = ((mode-1) << 1) & 6` (`rlwinm 0,0,1,29,30` at `0x80AFD1AC`), i.e. the
  index is `(mode-1) & 3` and the load is sign-extended. Only modes 1-4 stay
  inside the four-entry table; a larger mode masks into it.
* The default step `0x600` is `1536` in turn units (8.4 deg), the table steps
  are `0xFC00` (-1024), `0x0800` (2048), `0xFA00` (-1536) and `0x0400` (1024).
* The amplitude selector is a separate word (`+0x608`), so a mode change and an
  amplitude change are independent.
* The result is returned in `f1` and is **not** stored by this body; the caller
  adds it to `+0xAC`.

---

## 11. `0x80ADDAC0` - `block_pata` height spring plus the frame tail

* **Address:** `0x80ADDAC0`
* **TSV size:** 124 bytes
* **Reachable body:** 31 instructions, 31 words = the whole extent; 2 direct
  `bl` (`0x80237BD0` sin at `0x80ADDB04`, `0x80ADDA40` at `0x80ADDB24`), no
  branch, one `blr` at `0x80ADDB38`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80addac0_FUN_80addac0.c`
  - one `qr3` load at `0x80ADDAF4`; no `qr0` pair.
* **Proposed signature:** `pub unsafe fn fn_80addac0(this: usize)`
* **Callers in this REL:** one `bl` from `0x80ADE3D8` inside `0x80ADE3A0`
  (136 B, the ledgered `block_pata` step whose `+0xAC` delta sets `+0x348`
  through the `fcmpo`/`cror` predicate).
* **Constants:** two `lis r3,0x80b3` loads:

| address | type | value / role |
|---|---|---|
| `0x80B30B50` | f32 | `0.00390625` = `1/256`, turn-units pre-scale |
| `0x80B30B54` | f32 | `36.0` - height spring amplitude |

* **Fields:** `+0x78C` `s16` height phase (advances `0x200`); `+0x78E` `s16`
  secondary phase (advances `0x100`); `+0x87C` f32 spring base; `+0x888` f32
  height output (written).
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-32(r1)`; LR at `+36`; `r31` at `+28`; `r31 = this`; `r1+8` is the
`sth` staging slot. Epilogue `0x80ADDB28..0x80ADDB38`: restore `r31`, restore
LR, `addi r1,r1,32`, `blr`.

### Behaviour

```
s16[this+0x78C] = (s16)(s16[this+0x78C] + 0x200)
s16[this+0x78E] = (s16)(s16[this+0x78E] + 0x100)

a = s16[this+0x78E]                                  # the value just written

f32[this+0x888] = f32[this+0x87C] + 36.0 * sin_angle16((f32)(u16)a * (1.0/256.0))

fn_80adda40(this)                                    # 0x80ADDB24
```

### Notes

* Both phases advance unconditionally, but only `+0x78E` feeds this body's
  spring; `+0x78C` is advanced here for the tail body (rank 12), which advances
  it a second time in the same frame (see rank 12's first note).
* `+0x888` is the intermediate height that rank 12 copies to `+0xB0` after
  adding its own `16.0 * sin` term; this body therefore publishes the spring
  centre and the tail publishes the final position.
* The staging `sth r0,8(r1)` uses the freshly computed `+0x78E`; the `qr3` load
  is from `r6 = r1+8`, not from the actor.

---

## 12. `0x80ADDA40` - `block_pata` position commit from the `+0x888` block

* **Address:** `0x80ADDA40`
* **TSV size:** 120 bytes
* **Reachable body:** 30 instructions, 30 words = the whole extent; one direct
  `bl` (`0x80237BD0` sin at `0x80ADDA78`), no branch, one `blr` at
  `0x80ADDAB4`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80adda40_FUN_80adda40.c`
  - one `qr3` load at `0x80ADDA68`; no `qr0` pair.
* **Proposed signature:** `pub unsafe fn fn_80adda40(this: usize)`
* **Callers in this REL:** three `bl` sites:
  - `0x80ADDB24` inside `0x80ADDAC0` (rank 11),
  - `0x80ADE35C` inside `0x80ADE300` (116 B, ledgered `block_pata` node step),
  - `0x80ADE47C` inside `0x80ADE450` (68 B, ledgered two-call frame tail).
* **Constants:** two `lis r3,0x80b3` loads:

| address | type | value / role |
|---|---|---|
| `0x80B30B50` | f32 | `0.00390625` = `1/256`, turn-units pre-scale |
| `0x80B30B3C` | f32 | `16.0` - vertical bob amplitude |

* **Fields:** `+0x78C` `s16` height phase (advances `0x200` here as well);
  `+0x888` f32 x source; `+0x88C` f32 y base; `+0x890` f32 z source;
  `+0xAC/0xB0/0xB4` f32 x/y/z (written).
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-32(r1)`; LR at `+36`; `r31` at `+28`; `r31 = this`; `r1+8` is the
`sth` staging slot. Epilogue `0x80ADDAA4..0x80ADDAB4`: restore `r31`, restore
LR, `addi r1,r1,32`, `blr`.

### Behaviour

```
s16[this+0x78C] = (s16)(s16[this+0x78C] + 0x200)
a = s16[this+0x78C]

f32[this+0x0AC] = f32[this+0x888]
f32[this+0x0B4] = f32[this+0x890]
f32[this+0x0B0] = f32[this+0x88C] + 16.0 * sin_angle16((f32)(u16)a * (1.0/256.0))
```

### Notes

* This body advances `+0x78C` itself, so when it is reached through rank 11 the
  phase has advanced twice that frame (`0x400` total). The other two callers
  (`0x80ADE300`, `0x80ADE450`) call it directly, so the phase advances once.
  A port must keep the unconditional advance - it is the callback's contract,
  not a copy-paste artefact.
* The x and z commits are plain copies of the `+0x888`/`+0x890` block; only y
  is rebuilt, from `+0x88C` plus the `16.0` bob.
* The stores are ordered x, z, y (`0x80ADDA94`, `0x80ADDA9C`, `0x80ADDAA0`)
  even though the sources load as x (`0x80ADDA80`), z (`0x80ADDA88`), y
  (`0x80ADDA90`); the compiler placed the `16.0 * sin` multiply between them.
* `+0x78C` is shared with rank 11; because both bodies advance it, a lane that
  ports one without the other will drift by one `0x200` step per frame.

---

## Verification

* Twelve addresses, all absent from `ledger/functions.csv` (checked against
  every one of the 16,787 rows, read-only), from every heading of every
  `docs/*.md` (267 distinct heading addresses scanned) and from every
  `fn_<addr>` definition in `src/` at the time of writing.
* Excluded on purpose: `0x80B07D70` (complete `fn_80b07d70` body at
  `src/game/enemies.rs:181444`), `0x80AE2250` (the 13th by size, 108 B, the last
  `qr3` class-D row) and `0x80A337B0` (4-byte `b 0x80A337C0` alias, no body).
* Excluded as already specified: the 24 `qr3` rows of
  `docs/SPEC_WAVE140_ENEMIES.md` / `docs/SPEC_WAVE141_ENEMIES.md` and the 12 of
  `docs/SPEC_WAVE142_ENEMIES.md`, i.e. `0x80A3A4A0`, `0x80AEDD40`, `0x80AEF130`,
  `0x80A0E580`, `0x80B0C540`, `0x80AEB450`, `0x80AE4A50`, `0x80A4B740`,
  `0x80A4B420`, `0x80AA93F0`, `0x80A635D0`, `0x80AFDAC0`, `0x80A429C0`,
  `0x80AA4070`, `0x80AA35E0`, `0x80A63FF0`, `0x80AFD910`, `0x80A641F0`,
  `0x80B1D520`, `0x80A436C0`, `0x80A24AD0`, `0x80ABA080`, `0x80A4BBD0`,
  `0x80A46200`, `0x80AD8900`, `0x80AFDD50`, `0x80AB1900`, `0x80A43810`,
  `0x80AA37E0`, `0x80A79630`, `0x80ABAB50`, `0x80A40290`, `0x80A20810`,
  `0x80A7ADA0`, `0x80A6EA20`, `0x80A1D530`.
* Every byte count equals the `size` column of
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`, and for every row
  `size / 4` equals the reachable instruction count, so each body ends on the
  instruction before the next entry (`0x80ADDA40` is followed by 8 bytes of
  zero padding at `0x80ADDAB8` before `0x80ADDAC0`).
* Every `psq_l ...,qr3` in the nine `qr3` extents is a single `u16` widen; no
  `mtspr GQRn` exists in the REL (whole-image scan), and no `qr1..qr7` value is
  consumed anywhere in these bodies. The five `psq_st/psq_l ...,qr0` pairs are
  `f31` stack spills at the documented offsets.
* Every constant quoted in a **Constants** table was read from
  `work/rels/elf/d_enemiesNP.elf` at the address shown, including the
  `s16[4]` table at `0x80B31778` (raw bytes `fc 00 08 00 fa 00 04 00`). The
  two **(ext)** items - `0x802F8FF0` (rank 2/3 functor) and `0x80B74FD0`
  (rank 8 vtable argument) - are above the file-backed end `0x80B6E438` and are
  `main.dol` addresses.
* The two `main.dol` globals rank 4 reads (`0x80429DD0 + 0x8FEC4`,
  `0x8042A100 + 0x8C0`) are the same fields waves 31, 51, 84, 86 and 141
  record; they are data, not code, so they do not affect the code verdict.
* No Rust file, ledger CSV, coverage doc or existing specification was
  modified by this lane.
