# Wave 151 - `game::bases` - the next eight (0.66-0.63 KB band)

Read-only specification lane. No Rust file, no `ledger/functions.csv` row and no
existing document was modified. The only file written is this one. No merge or
check tool was run. The throwaway decoder and the ELF/`main.dol` readers live
under `%TEMP%\opencode\w151` (`cand.js`, `an.js`, `ph.js`, `img.js`, `gl.js`,
`ver.js`) plus the reused wave-143 decoder `%TEMP%\opencode\w150\ppc.js`, never
in the repository.

Waves 128-150 walked this bucket from the top in size order; the last row
claimed by `docs/SPEC_WAVE150_BASES.md` is 676 bytes (`0x80680FD0`). This wave
continues one rung lower - **676 bytes down to 648 bytes** - which is exactly the
eight-row set that document's `:112-115` predicted:

```
0x806C1040 676 | 0x806C7C20 668 | 0x8070B550 668 | 0x8065DA00 660
0x807B87F0 660 | 0x80682AA0 656 | 0x8068F220 656 | 0x80638120 648
```

## The eight

| # | address | bytes | words | export file | verdict |
|--:|---:|---:|---:|---|:--:|
| 1 | `0x806C1040` | 676 | 169 | `806c1040_FUN_806c1040.c` | ready (high) |
| 2 | `0x806C7C20` | 668 | 167 | `806c7c20_FUN_806c7c20.c` | ready (medium) |
| 3 | `0x8070B550` | 668 | 167 | `8070b550_FUN_8070b550.c` | ready (high) |
| 4 | `0x8065DA00` | 660 | 165 | `8065da00_FUN_8065da00.c` | ready (high) |
| 5 | `0x807B87F0` | 660 | 165 | `807b87f0_FUN_807b87f0.c` | ready (medium) |
| 6 | `0x80682AA0` | 656 | 164 | `80682aa0_FUN_80682aa0.c` | ready (medium) |
| 7 | `0x8068F220` | 656 | 164 | `8068f220_FUN_8068f220.c` | ready (medium) |
| 8 | `0x80638120` | 648 | 162 | `80638120_FUN_80638120.c` | ready (medium) |

**Ready: 8. Conditional: 0.** 5,292 bytes / 1,323 words in total.

Every `bytes` value is the `size` column of
`work/ghidra/rel-d_basesNP/export/functions.tsv` verbatim, `decompiled` is
`true` for all eight, and for all eight the size **is** the true instruction
span: the decoder finds exactly one `blr`, at `address + size - 4`
(`0x806C12E0`, `0x806C7EB8`, `0x8070B7E8`, `0x8065DC90`, `0x807B8A80`,
`0x80682D2C`, `0x8068F4AC`, `0x806383A4`), and no earlier one. No extent is
truncated (unlike wave 150 row 2, whose TSV size stopped one word short of its
`blr`). The next TSV row never starts inside a body: the gaps after these
extents are `0x806C1040` +12, `0x806C7C20` +836, `0x8070B550` +4,
`0x8065DA00` +12, `0x807B87F0` +12, `0x80682AA0` +0, `0x8068F220` +864,
`0x80638120` +152 bytes (padding or unanalysed code, not part of the body).

The decoder reports **no unknown word** in the 1,323 words: zero unknown
opcodes, zero unhandled X-form, zero unhandled paired-single form. Every `b` /
`bc` target is inside its own body (there is no `b` out of any extent at all);
every `bl` goes to a named callee outside the body.

### Why all eight are ready

The verdict rule is wave 144's, unchanged:

* **ready** - every reachable instruction is decoded, every literal is read out
  of the image or out of `main.dol`, every callee is named by address (or by its
  ledger name), and the body can be written as code with no assumed calling
  convention beyond the standard PowerPC EABI. Data in `main.dol` data/BSS
  (`0x8030xxxx`, `0x8042xxxx`, `0x8035xxxx`), in this REL's BSS
  (`0x80822900..0x80834D84`), or in another REL is an *input*, named by address.
* **conditional** - exactly one named input would still be missing.

Two of the eight touch a `psq` quantiser, and in both cases the slot is known:

* row 3 has only the `stfd f31` / `psq_st f31,168(r1),0,qr0` prologue save and
  the matching `psq_l` reload - `W = 0`, `qr0`, scalar-float type 0. There is no
  data-side quantisation at all.
* row 6 has three real `psq_l ...,1,qr3` loads (single-element, `W = 1`) whose
  GQR3 word is the constant wave 138 pinned,
  `GQR3 = 0x00050005` (type 5 = unsigned 16-bit, scale 0), installed at
  `0x8025EEEC`/`0x8025EEF0`. With that word the load is exactly
  `(u16_value) as f32`. Since `docs/REL_ELF_MAPPING.md:52-56` already carries the
  value, no input is missing and the row is ready, not conditional.
* rows 1, 2, 4, 5, 7, 8 contain **no** `psq` word at all.

`unaff_GQR0`/`ldexpf` text appears in the exports of rows 3, 4 and 6. In every
case it is the `stfd fNN` + `psq_st fNN,...,0,qr0` spill pair (rows 3, 4) or the
`W = 1` / `qr3` load pair (row 6) - i.e. decompiler noise around an explicit
instruction field, never an unknown value.

### The old rejections retired

Six of the eight carry an earlier skip. None has a genuine blocker:

* **`0x806C1040`** - `docs/COVERAGE_WAVE25_BASES.md:90-91` and `:212` rejected
  it because "the smooth-curve body reads an unrecovered `r4`
  (`extraout_r4`)", and `docs/COVERAGE_WAVE105_BASES.md:110-112` because it
  "reads its object through the `0x802DCD70` save/restore thunk". Both are the
  wave-127 finding: `0x802DCD70` is `_savegpr_26`, seven words that spill
  `r26..r31` through `r11` and never touch `r3..r10` (verified again from
  `work/dol/main.elf`: `stw r26,-24(r11)` .. `stw r31,-4(r11)`, `blr`, no `mflr`).
  The receiver in **entry `r3`** and the appended sample in **entry `r4`** both
  survive; `extraout_r4` is Ghidra's lost copy of the second argument. Section 1
  writes both.
* **`0x806C7C20`** - `docs/COVERAGE_WAVE25_BASES.md:92-93` and `:214` left it
  because it is "dominated by double conversions and the `0x80429E98` /
  `0x8042A1C8` tables". The two double conversions are the standard
  `xoris 0x8000` _signed_-`s32` -> `f64` idiom against the `2^52 + 2^31` magic at
  `0x807CB148`; the tables are the stage table `0x80429E98` and the stage-info
  pointer `0x8042A1C8` already documented in
  `docs/COVERAGE_WAVE10_STAGE.md:127` (`*(r13-0x57E8) + stage_info[0x120E] *
  0x3B0`). Section 2 spells both out.
* **`0x8070B550`** - `docs/COVERAGE_WAVE93_BASES.md:115-119` rejected it as "a
  paired-single reconstruction ... its register class cannot be proven" and "the
  0x30-entry vertex-table writes are not recoverable". The vertex table is
  **12** three-float points at `+0x6F8` (plus two scalars at `+0x6F0`/`+0x6F4`
  and two at `+0x788`/`+0x78C`), all 40 floats are listed expression by
  expression in section 3, and the only `psq` words are the `f31` spill pair.
  The `WARNING: Removing unreachable block (ram,0x8070b7d0)` is the epilogue
  reload of `f31`.
* **`0x8065DA00`** - `docs/COVERAGE_WAVE120_BASES.md:128-130` deferred it as
  "far outside this pass's window". Nothing was ever found wrong with the body;
  section 4 writes it.
* **`0x807B87F0`** - `docs/COVERAGE_WAVE143_BASES.md:101` listed it with
  "argument shapes fixed by the instruction stream; bodies not claimed", and
  `:121-125` left it "not started". Every one of its 39 call sites is a plain
  register setup; section 5 transcribes them in order.
* **`0x80638120`** - `docs/COVERAGE_WAVE138_BASES.md:159-160` left it because
  "the large ones (`0x80630820`, `0x8063A960`, `0x80638120`) need their own call
  graphs". Its call graph is 9 edges and is written out in section 8.

`0x80682AA0` and `0x8068F220` appear in **no** earlier skip list at all; they
surfaced only when the size-ordered filter reached 656 bytes.

### This is the list wave 150 predicted, exactly

Re-deriving the candidate set from the current tree gives **1,863** rows: the
5,615 `functions.tsv` rows, filtered against the **17,042** distinct addresses in
`ledger/functions.csv` and the **1,591** distinct `0x`/bare addresses that appear
in a `#{1,6} ` heading of `docs/*.md`, and against `decompiled=true`.
`docs/SPEC_WAVE150_BASES.md:89` computed 1,880 for the same filter one wave ago.
The 17-row drop is consistent with that wave's growth: `ledger/functions.csv`
gained 37 addresses (17,005 -> 17,042), the `docs/*.md` headings gained 19
(1,572 -> 1,591), and wave 150's own eight rows left the pool.

Ordering is size descending, ties broken by address ascending, exactly as waves
128-150 did. The 668-byte tie is ordered `0x806C7C20` before `0x8070B550`, the
660-byte tie `0x8065DA00` before `0x807B87F0`, and the 656-byte tie `0x80682AA0`
before `0x8068F220`. Each of the eight was re-tested against
`ledger/functions.csv` (exact `address` field), against every `#{1,6} ` heading
in `docs/*.md` (including bare and lowercase spellings), and against
`src/game/bases.rs` (no `fn_<addr>` token). All eight are absent from all three,
and each has a `decompiled=true` TSV row with a matching `.c` export on disk.

Applying the same filter after this wave, the next eight are:
`0x80703DC0` (648), `0x8062BE20` (644), `0x8076B710` (644), `0x8074CBA0` (636),
`0x8078A8F0` (632), `0x8061B890` (628), `0x80752730` (628), `0x80792570` (628).
(`0x807310C0` and `0x80794980` are 624 bytes and would follow; `0x807B2050` and
`0x80787BD0` are 620. `0x807B2050` is not in the ledger, but `src/game/bases.rs`
already links it by name in the doc comment at line 88366, so a later lane should
check it against that cluster before claiming it.)

## Method

**Address mapping.** Re-derived from the program headers of
`work/rels/elf/d_basesNP.elf`, not copied:

```
PT_LOAD  p_offset  p_vaddr     filesz    memsz
  0      0x0010F0  0x806000F0  0x1C6004  0x1C6004
  1      0x1C70F4  0x807C60F4  0x0004FC  0x0004FC
  2      0x1C75F0  0x807C65F0  0x00000C  0x00000C
  3      0x1C7600  0x807C6600  0x00A5FC  0x00A5FC
  4      0x1D1C00  0x807D0C00  0x051CB8  0x051CB8
  5      0x000000  0x80822900  0x000000  0x012484 (zero fill)
```

so `file_offset = 0x10F0 + (address - 0x806000F0) = (address - 0x80600000) +
0x1000` for every code and data address in this document, as
`docs/REL_ELF_MAPPING.md` requires. Literal pools in `0x807Cxxxx` / `0x807Dxxxx`
resolve through segments 1-4 (e.g. `0x807CC900` -> offset `0x1CD900`, `0x807C9C58`
-> `0x1CAC58`) and are read out verbatim below. `0x808xxxxx` below `0x80822900`
is real data; `0x80822900..0x80834D84` is this REL's BSS and reads as zero at
file level - which matters for rows 2 and 8, both of which pass BSS pointers.

`main.dol`-resident data and code (`0x8030A040`, `0x80358004`, `0x80B76630`,
`0x8042xxxx`, and the callees in `0x800xxxxx`/`0x802Dxxxx`) are read from
`work/dol/main.elf` through **its own** program headers:

```
PT_LOAD  p_offset  p_vaddr     filesz
  0      0x000200  0x80004000  0x0026C0
  1      0x002900  0x800066C0  0x000060
  2      0x002A00  0x80006720  0x000060
  3      0x002B00  0x80006780  0x2E7260   <- all 0x800xxxxx/0x802xxxxx code,
  4      0x2E9E00  0x802ED9E0  0x0002E0      and 0x8030A040 data
  5      0x2EA100  0x802EDCC0  0x000020
  6      0x2EA200  0x802EDCE0  0x0106C0
  7      0x2FA900  0x802FE3A0  0x0532A0   <- 0x8031xxxx data
  9      0x34DC00  0x80427680  0x002540
 11      0x350200  0x8042B080  0x004B60
```

The `0x8042xxxx` globals used below are in `main.dol`'s zero-fill BSS
(`0x80429BC0..0x8042B080`), so they are addressed absolutely and read as zero at
file level. `0x8030A040` is in `main.dol` segment 7
(`0x802FE3A0..0x80351640`, file offset `0x3065A0`) and also reads as zeros - the
record table is filled at run time. `0x80358004` is `main.dol` BSS
(`0x80351680..0x80427680`). `0x80B76630` is **data inside `d_enemiesNP`**: that
REL's segment `0x80B6E500..0x80B799E0` contains it, so row 4 passes a key object
that belongs to the enemies module. The `0x802DCDxx` / `0x802DD1xx` helpers are
`main.dol` code in segment 3.

**Decoder.** Reused the wave-143 throwaway decoder
(`%TEMP%\opencode\w150\ppc.js`) and re-validated it against the facts
`docs/REL_ELF_MAPPING.md` pins: branch `LK` is bit 0 and `AA` bit 1;
conditional branches use the 14-bit `BD`; `psq_l` = 56 and `psq_st` = 60 with the
quantiser index in bits 12-14 and `W` in bit 15; single-float `fmuls`/`fdivs`/
`fadds`/`fsubs`/`fneg`/`fmr`/`frsp`/`fctiwz`/`fcmpo`/`fcmpu` use `FRC`/`FRB`.
One **known mislabel** was worked around by hand: the shared decoder prints
`mfspr r0,vrsave` for `0x7C0802A6` and `mtspr lr,r0` for `0x7C0803A6`. Those two
words are `mflr r0` / `mtlr r0`, which is provable from the helpers themselves -
`_savegpr_26` at `0x802DCD70` spills only `r26..r31` and has no `mflr`, so the
`36(r1)` slot in every 32-byte frame here is the caller's link word. **All
eight** bodies save/restore the link word that way; rows 4 and 6 additionally
spill `f28..f31` (row 4) or `f30`/`f31` (row 6) with `stfd`+`psq_st` pairs, and
rows 1, 4, 7 and 8 use a `_savegpr` helper.

**CodeWarrior helpers.** Four rows use them; none destroys an argument:

| row | prologue helper | registers saved | `this` kept in |
|--:|---|---|---|
| 1 | `_savegpr_26` = `0x802DCD70`, restore `0x802DCDBC` | `r26..r31` | `r3` (used in place) |
| 4 | `_savegpr_23` = `0x802DCD64`, restore `0x802DCDB0` | `r23..r31` (+ manual `f28..f31`) | `r27` |
| 5 | none (frame only) | `r29..r31` | `r29` |
| 7 | `_savegpr_23` = `0x802DCD64`, restore `0x802DCDB0` | `r23..r31` | `r26` |
| 8 | `_savegpr_26` = `0x802DCD70`, restore `0x802DCDBC` | `r26..r31` | `r31` |

All five helpers are verified word by word from `work/dol/main.elf`:
`0x802DCD64` spills `r23..r31` (`stw r23,-36(r11)` .. `stw r31,-4(r11)`),
`0x802DCD70` spills `r26..r31`, and neither has an `mflr`, so the incoming
`r3..r10` (and the LR slot at `36(r1)`) survive.

**Call notation.** Throughout this document `callN(addr, a, b, ...)` lists the
integer argument registers `r3, r4, ...` from left to right; a callee's float
arguments are written `addr(f1 = ..., r3 = ...)` explicitly; a callee treated as
returning `f32` in `f1` is written `callN_f32(...)`. Where the original body
leaves a *dead* store behind (a stack slot that no path reads) the document says
so, because the byte-match check will otherwise look wrong.

---

## 1. `0x806C1040` - triangular-weight window filter over a 32-sample ring

**Evidence.** `work/ghidra/rel-d_basesNP/export/806c1040_FUN_806c1040.c`, 676
bytes, 169 words, synthetic `0x806C1040`. Body `0x806C1040..0x806C12E4` (ends
`blr` at `0x806C12E0`). No float registers are used at all; `this` stays in `r3`
for the whole body and is copied to `r11` (= `this + 0x4C4`) only as a store
base.

**What it is.** One append + one full recompute of a smoothing filter. The
object keeps the last 32 samples of a signal in a `s16` ring at `+0x4E4`, and a
window of `cnt` `s16` results at `+0x4C4`. Each call appends the new sample at the
ring cursor, then rebuilds every window slot as
`sample * (cnt^2 - (cnt - i)^2) / cnt^2` taken from the ring at stride **2**
(`ring[(n - 2i) & 31]`), and finally slew-limits the rebuilt window in place to
`+-3072` per step. That is a linear-ramp (Bartlett-like) weight over a decimated
history; the newest sample gets weight 0 and the oldest gets `(cnt^2-1)/cnt^2`.

### Signature

```rust
pub unsafe fn fn_806c1040(this: usize, value: i16)
```

`r3` = the filter owner; `r4` = the new sample, stored verbatim into the ring
(the low 16 bits of `r4`). No return value.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x4C4` | `s16[cnt]` | smoothed window, rewritten every call |
| `+0x4E4` | `s16[32]` | raw sample ring, cursor `+0x527` |
| `+0x526` | `u8` | window length `cnt` (also the ring view depth) |
| `+0x527` | `u8` | ring cursor, always masked with `& 0x1F` |

`+0x524..0x525` lie between the ring and `cnt` and are never touched, so the ring
is exactly `0x4E4..0x523` (64 bytes) and the window `0x4C4..0x4C3+2*cnt`.

### Exact behaviour

Prologue `0x806C1040..0x806C1054`: `stwu r1,-32(r1)`, `mflr r0`,
`stw r0,36(r1)`, `r11 = r1+32`, `bl 0x802DCD70` (`_savegpr_26`, spills
`r26..r31` through `r11`). `r3`/`r4` are untouched by the helper.

1. `0x806C1054..0x806C1074` - append:
   ```rust
   let mut n = read_u8(this + 0x527).wrapping_add(1) & 0x1F;
   write_u8(this + 0x527, n as u8);
   write_s16(this + 0x4E4 + n as usize * 2, value);
   ```
   The `addi` is an 8-bit add with no saturation, then `rlwinm 0,27,31` masks to
   five bits; the store base is `this + (n & 0x1F) * 2`.
2. `0x806C1078..0x806C1094` - `cnt = read_u8(this + 0x526)`,
   `den = cnt * cnt` (`mullw`, 32-bit signed, never 0 for `cnt != 0`). If
   `cnt == 0` jump to step 5; if `cnt <= 8` jump to step 4.
3. `0x806C1098..0x806C1230` - the 8x-unrolled body, `ctr = ((cnt - 8) + 7) >> 3`
   iterations of eight outputs. Each output `i` (with `i = 8k + j`) is
   ```rust
   let idx = (n as i32 - 2 * i as i32) & 0x1F;      // ring[(n - 2i) & 31]
   let w   = den - (cnt as i32 - i as i32).pow(2);   // cnt^2 - (cnt - i)^2
   let v   = read_s16(this + 0x4E4 + idx as usize * 2) as i32 * w;
   write_s16(this + 0x4C4 + i as usize * 2, (v / den) as i16);  // `divw`, signed
   ```
   `divw` truncates toward zero; use Rust `i32 / i32`, **not** a shift. The
   register `r8` walks `n, n-2, n-4, ...` masked with `& 0x1F` after every step,
   which is the same thing.
4. `0x806C1234..0x806C1278` - the remainder loop: `ctr = cnt - 8k`, same
   expression, one output per iteration, `i` continuing where step 3 stopped.
   With `cnt <= 8` this loop alone runs `cnt` times (`ctr = cnt`, `r12 = 0`).
5. `0x806C127C..0x806C12C8` - slew limiter, executed **at least once** (the exit
   test is at the bottom, `blt 0x806C1290`):
   ```rust
   let mut p = this + 0x4C4;
   let end   = p + cnt as usize * 2;
   let mut prev: i32 = 0;
   loop {
       let cur = read_s16(p) as i32;
       let d   = (cur - prev) as i16 as i32;   // `subf` + `extsh`
       let v = if d > 0xC00 { prev + 0xC00 }
               else if d < -0xC00 { prev - 0xC00 }
               else { cur };
       write_s16(p, v as i16);
       prev = v;
       p += 2;
       if !(p < end) { break; }
   }
   ```
   Because the test is post-decrement, `cnt == 0` still clamps `+0x4C4` once
   (with `end == p`, so exactly one iteration). This is observable: it rewrites
   the first window slot even when the window is disabled.
6. `0x806C12CC..0x806C12E4` - epilogue: `r11 = r1+32`, `bl 0x802DCDBC`
   (`_restgpr_26`), `lwz r0,36(r1)`, `mtlr r0`, `addi r1,r1,32`, `blr`.

### Constants

| value | use |
|---|---|
| `0x1F` | ring mask / cursor mask |
| `32` | ring length in entries |
| `0xC00` = 3072 | slew step limit (`+3072` / `-3072`), **not** a float |
| `cnt * cnt` | per-call divisor, signed `divw` |

There is no literal pool and no global: the whole body works through `this`.

### Callees

| target | name | shape |
|---|---|---|
| `0x802DCD70` | `runtime::cpp::fn_802dcd70` | `_savegpr_26`, `(r11)` -> saves `r26..r31` |
| `0x802DCDBC` | `runtime::cpp::fn_802dcdbc` | `_restgpr_26`, `(r11)` -> restores `r26..r31` |

### Notes for the implementer

* The value type is `i16` because the store is `sth`; a `u16` parameter is also
  faithful (identical bits) but `i16` matches the later `lha` reads of the ring.
* `cnt` is never validated here; another body seeds `+0x526`. A `cnt` of 0 makes
  step 3/4 no-ops and leaves step 5's single clamp.
* The `ori r0,r0,0` word at `0x806C128C` is the canonical PowerPC nop; do not
  emit anything for it.

---

## 2. `0x806C7C20` - player tick: speed ramp, flash timer and section-4b limit

**Evidence.** `work/ghidra/rel-d_basesNP/export/806c7c20_FUN_806c7c20.c`, 668
bytes, 167 words, synthetic `0x806C7C20`. Body `0x806C7C20..0x806C7EB8` (ends
`blr` at `0x806C7EB8`). Frame `0x30`; `stwu r1,-48(r1)`, `mflr r0`,
`stw r0,52(r1)`, `stw r31,44(r1)`, `stw r30,40(r1)`; `r31 = 0x807CB0B0` (literal
pool), `r30 = this`. No GPR save helper.

**What it is.** The per-frame "attack/charge" tick of a player-driven object. It
asks the input/state scanner `0x806C7890` for `(sum, pressed, flags)`, advances a
speed accumulator `+0xAC` by a table-selected amount (or by 0 when the "special"
bit is held), maintains a two-entry flash countdown, converts the speed to a
16-bit angle for `0x806C7A20`, and finally clamps `+0xB0` down to a value derived
from the current stage's section-4b entry, calling the object's virtual
`+0x18` slot when the clamp bites.

`0x806C7A20` is already translated as `game::bases::fn_806c7a20(actor, target:
i16)` (the "angle step" of the lift family, `docs/COVERAGE_WAVE25_BASES.md:85-86`),
so the `s16` handed over is an angle step in that helper's units.

### Signature

```rust
pub unsafe fn fn_806c7c20(this: usize)
```

No return value. No arguments besides `this`.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0xAC` | `f32` | speed accumulator, `+= speed` every frame |
| `+0xB0` | `f32` | clamp target (also `+= 0.25 / -0.25` in modes 0/1) |
| `+0x349` | `u8` | id passed to `section4b_entry_by_id` |
| `+0x394` | vtable ptr | interface object; `+0x18` called with `0x8082D108` |
| `+0x494` | `s16` | value reloaded into `+0x496` |
| `+0x496` | `s16` | countdown, decremented in two arms |
| `+0x498` | `u8` | mode: `0`, `1`, `2` (anything else: no extra work) |
| `+0x49A` | `u8` | second mode: `2`/`3` compare `+0x49B`, `1` decrements |
| `+0x49B` | `u8` | threshold for the `+0x49A` compare |
| `+0x49E` | `u8` | flash countdown (0..=0x78) |
| `+0x49F` | `u8` | flags: bit 7 = clamp direction, bit 7 also "flash armed" |

### Exact behaviour

1. `0x806C7C20..0x806C7C4C` - prologue and the scanner call:
   ```rust
   let mut pos  = 0.0f32;     // r1+16, `lfs f2,16(r1)`
   let mut hit  = 0i32;       // r1+12
   let mut flags = 0u32;      // r1+8
   let count = call4(0x806C7890, this, &mut pos, &mut hit, &mut flags) as i32;
   ```
   `0x806C7890` (392 bytes, same REL, untranslated) walks the linked list at
   `this+0x440` (nodes linked through `+0xC`, child at `+4`), sums
   `node[+0x24] + (-60.0f)` for every child whose `+0x38C` state byte is 1 or 2,
   sets `hit = 1` when a child's `0x80056CF0(child,0x1C)` is non-zero, and ORs
   `flags` bit 0 (child `+0x10D4 & 2` while `this+0x49F` sign bit is clear) and
   bit 1 (`child +0x10D4 & 0x60`). It returns the number of active children.
2. `0x806C7C50..0x806C7C98` - the `+0x496` countdown:
   ```rust
   let m = read_u8(this + 0x49A);
   if (m.wrapping_sub(2)) <= 1 {                 // m == 2 || m == 3
       if count < read_u8(this + 0x49B) as i32 {
           write_s16(this + 0x496, read_s16(this + 0x496) - 1);
       } else {
           write_s16(this + 0x496, read_s16(this + 0x494));
       }
   } else if m == 1 {
       write_s16(this + 0x496, read_s16(this + 0x496) - 1);
   }
   ```
   The `cmpli r0,1` after `addi r0,r4,-2` makes this an unsigned test, so
   `m == 2` and `m == 3` both take the first arm.
3. `0x806C7C9C..0x806C7CE4` - speed from the table:
   ```rust
   let mut speed = read_f32(0x807CB104);          // 0.0f
   if flags & 2 == 0 {
       speed = pos * read_f32(0x807CB114 + count as usize * 4);
       if speed < read_f32(0x807CB130) { speed = read_f32(0x807CB130); }  // -4.0f
       else if speed > read_f32(0x807CB134) { speed = read_f32(0x807CB134); } // 4.0f
   }
   ```
   The table at `0x807CB114` is `[0.0, 0.02083, 0.03333, 0.0, 0.0, 0.25, -0.25,
   -4.0, 4.0, 0.25, -0.25, 4096.0, ...]`, so `count` is an index into the *same*
   array that holds the `+-4.0` clamps at index 7/8. There is **no** bounds
   check: the `count` returned by `0x806C7890` is used raw (`lfsx` with
   `count * 4`), so a body that adds a guard is not faithful.
4. `0x806C7CE8..0x806C7DCC` - speed accumulate and mode work:
   ```rust
   write_f32(this + 0xAC, read_f32(this + 0xAC) + speed);
   match read_u8(this + 0x498) {
       0 => write_f32(this + 0xB0, read_f32(this + 0xB0) + read_f32(0x807CB138)), // +0.25
       1 => write_f32(this + 0xB0, read_f32(this + 0xB0) + read_f32(0x807CB13C)), // -0.25
       2 => {
           if flags & 1 != 0 {
               write_u8(this + 0x49E, 0x78);
               write_u8(this + 0x49F, read_u8(this + 0x49F) | 0x80);
           } else if hit != 0 && (read_u8(this + 0x49F) & 0x80) != 0 {
               write_u8(this + 0x49F, read_u8(this + 0x49F) & 0x7F);
               write_u8(this + 0x49E, 0);
           }
           let t = read_u8(this + 0x49E);
           if t != 0 {
               let t = t - 1;
               write_u8(this + 0x49E, t);
               if t == 0 { write_u8(this + 0x49F, read_u8(this + 0x49F) & 0x7F); }
           }
           let idx = if read_u8(this + 0x49F) & 0x80 != 0 { 4 } else { 0 };
           write_f32(this + 0xB0, read_f32(this + 0xB0) + read_f32(0x807CB128 + idx));
       }
       _ => {}
   }
   ```
   The index is the `rlwinm 27,29,29` extraction of bit 7 (`0x80`) - verified
   numerically; it selects `+0.25f` at `0x807CB128` or `-0.25f` at `0x807CB12C`.
5. `0x806C7DCC..0x806C7E00` - feed `0x806C7A20`:
   ```rust
   if flags & 2 == 0 {
       let a = (-speed) * read_f32(0x807CB138) * read_f32(0x807CB140); // -speed*0.25*4096
       call2(0x806C7A20, this, fctiwz_to_i16(a));    // `fctiwz` then `extsh`
   }
   ```
   `fctiwz` truncates toward zero (it does **not** round), then `extsh` takes the
   low 16 bits with sign extension - i.e. `(a.trunc() as i32) as i16` for every
   in-range value. Do not round, and do not clamp to `i16` first.
6. `0x806C7E04..0x806C7E40` - stage lookup:
   ```rust
   let stage_base = read_u32(0x80429E98) as usize
       + read_u8(read_u32(0x8042A1C8) as usize + 0x120E) as usize * 0x3B0;
   let stage = if read_u32(stage_base + 0x2C) == 0 { 0 } else { stage_base };
   let entry = call3(0x8008E410, stage, read_u8(this + 0x349) as u32, 0);
   ```
7. `0x806C7E40..0x806C7E80` - clamp `+0xB0` down to the entry's fixed-point
   value:
   ```rust
   let raw = read_u16(entry + 2) as i32;
   let limit = (raw - 0x80000) as f32 / read_f32(0x807CB140);   // /4096.0f
   if read_f32(this + 0xB0) >= limit {                          // `cror` = ge
       write_f32(this + 0xB0, limit);
       let vt = read_u32(this + 0x394) as usize;
       call3(read_u32(vt + 0x18), this + 0x394, 0x8082D108, 0);
   }
   ```
   The `cror 0x4C411382` (`cror 2,1,2`) turns `cr0.eq` into
   `cr0.gt | cr0.eq`, and the following `bne` skips when that is false, i.e. the
   store runs when `+0xB0 >= limit` (this is Ghidra's `if (fVar1 <= ...)`
   inverted). The conversion is the signed-`s32` -> `f64` idiom
   (`addis r0,r3,0xfff8`, `xoris r0,r0,0x8000`, `lfd`, `fsubs` against
   `2^52 + 2^31` at `0x807CB148`), so the value is exactly
   `(u16 - 524288) / 4096.0f` in `[-128, -112]`.
8. `0x806C7EA4..0x806C7EB8` - epilogue: `lwz r0,52(r1)`, restore `r31`, `r30`,
   `mtlr r0`, `addi r1,r1,48`, `blr`.

### Constants

| address | value | use |
|---|---|---|
| `0x807CB0B0+0x54` = `0x807CB104` | `0.0f` | speed when `flags & 2` |
| `0x807CB0B0+0x64` = `0x807CB114` | `f32[..]`: `0.0, 0.02083, 0.03333, 0.0, 0.0, 0.25, -0.25, -4.0, 4.0, 0.25, -0.25, 4096.0` | per-`count` speed factor |
| `0x807CB130` | `-4.0f` | speed lower clamp |
| `0x807CB134` | `4.0f` | speed upper clamp |
| `0x807CB138` | `0.25f` | mode-0 accel; also `-speed * 0.25` |
| `0x807CB13C` | `-0.25f` | mode-1 accel |
| `0x807CB128` | `0.25f` | flash accel, bit 7 clear |
| `0x807CB12C` | `-0.25f` | flash accel, bit 7 set |
| `0x807CB140` | `4096.0f` | fixed-point divisor |
| `0x807CB148` | `f64 2^52 + 2^31` (`0x4330000080000000`) | `s32 -> f64` magic |
| `0x80429E98` | `u32` pointer | stage table base |
| `0x8042A1C8` | `u32` pointer | stage-info; `+0x120E` = stage index |
| `0x8082D108` | BSS object | argument of the virtual `+0x18` call |

### Callees

| target | name | shape |
|---|---|---|
| `0x806C7890` | same REL, untranslated, 392 B | `(this, *mut f32, *mut i32, *mut u32) -> i32` |
| `0x806C7A20` | `game::bases::fn_806c7a20` | `(actor, target: i16)` |
| `0x8008E410` | `resource::course::section4b_entry_by_id` | `(stage, id: u32, out: 0) -> usize` |
| `this+0x394 -> +0x18` | virtual | `(this+0x394, 0x8082D108)` |

### Notes for the implementer

* `0x806C7890` is a callee, not an input: it is called by address. Its own body
  is a later wave's work, but nothing in row 2 depends on its internals beyond
  the documented `(sum, hit, flags)` contract.
* `count` indexes the pool array *including* the clamp entries, so a defensive
  `if count > 10` is **not** faithful; keep the raw `+ count * 4` load.
* The `stfs f1,16(r1)`-style stack traffic in `0x806C7890` is inside the callee.

---

## 3. `0x8070B550` - obstacle polygon build and collision-area registration

**Evidence.** `work/ghidra/rel-d_basesNP/export/8070b550_FUN_8070b550.c`, 668
bytes, 167 words, synthetic `0x8070B550`. Body `0x8070B550..0x8070B7EC` (ends
`blr` at `0x8070B7E8`). Frame `0xB0`; `r31 = 0x807CC900` (literal pool),
`r30 = this`; `f31` is the only saved float (`stfd f31,160(r1)` +
`psq_st f31,168(r1),0,qr0`, reloaded by `psq_l`/`lfd`).

**What it is.** The init body of a round/oval obstacle. It expands the object's
centre `(this+0x668, this+0x66C)` into a 12-point, 3-float vertex list at
`+0x6F8` (all `z = 1500.0f`), records two scalars at `+0x6F0`/`+0x6F4`, hands the
block to the `+0x674` sub-object constructor with a 48-byte element size, seeds
its own position from vertex 3, patches the REL-resident collision descriptor
`0x8080565C` (two radius words `28.800001 -> 12.15`), then registers the
collision area at `+0x148`.

### Signature

```rust
pub unsafe fn fn_8070b550(this: usize)
```

No return value.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x148` | collision area | registered at the end |
| `+0x1E6` | `u8` | set to 1 before the register call |
| `+0x61C` | `u8` | cleared |
| `+0x668` | `f32` | centre x |
| `+0x66C` | `f32` | centre y |
| `+0x674` | sub-object | constructor `0x800B9050(ptr, 48)` |
| `+0x6DC` | `u32` | 0 |
| `+0x6E0` | `u32` | 3 |
| `+0x6E4`, `+0x6E8` | `f32` | 0.0 |
| `+0x6EC`, `+0x6ED` | `u8` | 0 |
| `+0x6F0`, `+0x6F4` | `f32` | `y+8`, `y-88` (see below) |
| `+0x6F8` | `f32[36]` | 12 x `(x, y, 1500.0f)` |
| `+0x788`, `+0x78C` | `f32` | `y-88` (twice) |
| `+0x790..0x792` | `u8` | 0 |
| `+0xB10` | `u32` | 0 |
| `+0xAC`, `+0xB0`, `+0xB4` | `f32` | own position, seeded from vertex 3 |

### Exact behaviour

Prologue `0x8070B550..0x8070B578`: frame `0xB0`, `mflr`/`stw r0,180(r1)`,
`stfd f31,160(r1)`, `psq_st f31,168(r1),0,qr0`, `stw r31,156(r1)`,
`r31 = 0x807CC900`, `stw r30,152(r1)`, `r30 = this`.

1. `0x8070B578..0x8070B5C8` - pure float expansion of the centre. With
   `x = read_f32(this+0x668)`, `y = read_f32(this+0x66C)` and the constants of
   the table below:
   ```rust
   let a = 36.0f32 + (x - 176.0);     // x - 140
   let b = (176.0 + x) - 36.0;        // x + 140
   let c = x - 112.0;
   let d = 8.0 + y;                   // y + 8
   let e = y - 88.0;                  // y - 88
   let f = 36.0 + d;                  // y + 44
   let g = 112.0 + x;                 // x + 112
   let h = (a + b) * 0.5;             // x   (the compiler's average of a and b)
   let i = 20.0 + c;                  // x - 92
   let j = 10.0 + f;                  // y + 54
   let k = 10.0 + g;                  // x + 122
   let l = 10.0 + h;                  // x + 10
   let n = 36.0 + e;                  // y - 52
   let o = c - 10.0;                  // x - 122
   let p = h - 10.0;                  // x - 10
   let q = g - 20.0;                  // x + 92
   ```
   `h` is `(a + b) * 0.5`, i.e. `x` recomputed through a multiply rather than
   reused - reproduce the multiply, because `a + b` is exactly `2x` only in exact
   arithmetic.
2. `0x8070B5CC..0x8070B718` - the 40 stored floats. In store order the body
   writes `+0x6F0 = d`, `+0x6F4 = e`, then the 12 triples, then `+0x788 = e`,
   `+0x78C = e`:
   ```rust
   write_f32(this, 0x6F0, d);   write_f32(this, 0x6F4, e);
   let z = 1500.0f32;
   let pts = [
       (i, j), (l, f), (k, f), (b, n), (l, n), (a, n),
       (o, f), (p, f), (q, j), (b, n), (l, n), (a, n),
   ];
   for (idx, (px, py)) in pts.iter().enumerate() {
       base = this + 0x6F8 + idx * 12;
       write_f32(base, 0, *px); write_f32(base, 4, *py); write_f32(base, 8, z);
   }
   write_f32(this, 0x788, e);   write_f32(this, 0x78C, e);
   ```
   The last three points repeat points 3, 4 and 5: the shipped data really does
   carry that duplication (verified against `0x8070B764..0x8070B784`). The body
   also writes an identical 36-float copy to `r1+0x08..r1+0x94`; **those stack
   stores are dead** - no path reads them, and the following call sets only
   `r3`/`r4`. They are the artifact of a by-value struct assignment and may be
   dropped.
3. `0x8070B71C..0x8070B724` - `call2(0x800B9050, this + 0x674, 48)`. The callee
   writes `*(u32*)(ptr+0x1C) = 0x32`, calls `0x800B90C0(ptr)` and
   `0x800B9530(ptr)`, then two virtuals on `ptr+0x28` (`+0x18` with
   `0x80358BD8`, `+0x1C`). `48` is its element size; the pointer and the 48 are
   the only inputs this body supplies.
4. `0x8070B724..0x8070B760` - seed scalars and position from **vertex 3**:
   ```rust
   let v3 = this + 36 + 0x6F8;                  // `r6 = this + 0x24`, then `+0x6F8`
   write_u32(this, 0x6DC, 0);
   write_u32(this, 0x6E0, 3);
   write_f32(this, 0xAC, read_f32(v3 + 0x00));  // x + 140
   write_f32(this, 0xB0, read_f32(v3 + 0x04));  // y - 52
   write_f32(this, 0xB4, read_f32(v3 + 0x08));  // 1500.0
   write_f32(this, 0xAC, read_f32(v3 + 0x00) - 16.0); // x + 124
   ```
   The `-16.0f` comes from `0x807CC900+0xD4`.
5. `0x8070B784..0x8070B7AC` - remaining seeds:
   ```rust
   let y1 = read_f32(this + 0x6F4);             // y - 88
   write_f32(this, 0x788, y1);   write_f32(this, 0x78C, y1);
   write_f32(this, 0x6E4, 0.0); write_f32(this, 0x6E8, 0.0);
   write_u32(this, 0xB10, 0);
   write_u8(this, 0x6EC, 0); write_u8(this, 0x6ED, 0);
   write_u8(this, 0x790, 0); write_u8(this, 0x791, 0); write_u8(this, 0x792, 0);
   write_u8(this, 0x61C, 0);
   ```
6. `0x8070B7B0..0x8070B7B4` - patch the REL-resident descriptor and register:
   ```rust
   let d = 0x8080565Cu32 as usize;              // REL data, not BSS
   write_f32(d, 0x08, 12.15);                   // was 28.800001
   write_f32(d, 0x0C, 12.15);                   // was 28.800001
   call3(0x8008C3E0, this + 0x148, this, d);    // r6 = this+0x24 is set but unused
   write_u8(this, 0x1E6, 1);
   call1(0x8008C330, this + 0x148);
   ```
   `0x8008C3E0` is `game::stage::collision_area_set_descriptor(this, owner,
   descriptor)`: it copies `descriptor[0..=8]` (`+0x1C..+0x3C` of the area) and
   clears `+0xA1`. The extra `r6 = this+0x24` in the register set is dead - the
   callee reads only `r3..r5`.
7. `0x8070B7CC..0x8070B7EC` - epilogue: reload `f31`, `r31`, `r30`, `mtlr`,
   `addi r1,r1,176`, `blr`.

### Constants

| address | value | use |
|---|---|---|
| `0x807CC900+0x00` | `0.0f` | `+0x6E4`/`+0x6E8` |
| `0x807CC900+0xB0` | `88.0f` | `y - 88` |
| `0x807CC900+0xB4` | `8.0f` | `y + 8` |
| `0x807CC900+0xB8` | `36.0f` | x/y step in the expansion |
| `0x807CC900+0xBC` | `176.0f` | x half-extent |
| `0x807CC900+0xC0` | `112.0f` | x inset |
| `0x807CC900+0xC4` | `20.0f` | x inset |
| `0x807CC900+0xC8` | `10.0f` | x/y inset |
| `0x807CC900+0xCC` | `1500.0f` | every vertex `z`; also own `+0xB4` |
| `0x807CC900+0xD0` | `0.5f` | average of the two x extents |
| `0x807CC900+0xD4` | `16.0f` | subtracted from the seeded `+0xAC` |
| `0x807CC900+0xD8` | `12.15f` | descriptor radius patch |
| `0x8080565C` | REL data, 0x24-byte descriptor | `+0x08`/`+0x0C` patched in place |

### Callees

| target | name | shape |
|---|---|---|
| `0x800B9050` | `main.dol`, untranslated | `(block, element_size: 48)` |
| `0x8008C3E0` | `game::stage::collision_area_set_descriptor` | `(area, owner, descriptor)` |
| `0x8008C330` | `game::stage::collision_area_register` | `(area)` |

### Notes for the implementer

* The `psq` words are the `f31` save/restore pair only (`W = 0`, `qr0`, type 0),
  so no GQR input is needed. The export's `unaff_GQR0` text is that pair.
* `0x8080565C` is **real data** in this REL (segment 4, `p_vaddr 0x807D0C00`,
  file offset `0x20665C`), not BSS, and it is mutated by this call. A faithful
  body writes through that absolute address.
* The 12-point list is what the `+0x674` sub-object consumes; nothing in this
  body reads it back.

---

## 4. `0x8065DA00` - radius query over the actor list, dispatching by type id

**Evidence.** `work/ghidra/rel-d_basesNP/export/8065da00_FUN_8065da00.c`, 660
bytes, 165 words, synthetic `0x8065DA00`. Body `0x8065DA00..0x8065DC94` (ends
`blr` at `0x8065DC90`). Frame `0xA0`; saves `f28..f31` manually
(`stfd` + `psq_st ... 0,qr0` pairs) and `r23..r31` through `_savegpr_23`
(`0x802DCD64`, restore `0x802DCDB0`); `r27 = owner`, `r28 = max`, `r29 = out`.

**What it is.** A "collect everything within `radius` of `(x, y)`" sweep over the
engine's category-2 object list. Every object whose 2D distance passes the test is
dispatched on its `u16` type id at `+8`: type `0x200` is submitted as an effect at
a fixed `z = 5500.0f` and optionally copied to the caller's out array; `0x19B` is
poked through `0x8011EBB0` unless its gate says otherwise; `0x33` gets
`0x800A0B30(obj, owner, 0)` when a named-flag lookup returns 0; `0x69` and `0x68`
go to `0x80040EC0` and `0x80123A00`.

### Signature

```rust
pub unsafe fn fn_8065da00(
    x: f32, y: f32, radius: f32,     // f1, f2, f3
    owner: usize,                    // r3
    max_out: i32,                    // r4
    out: *mut [f32; 3],              // r5, 12-byte stride
) -> i32
```

The return is the number of type-`0x200` objects accepted (in `r3`), *not* the
number copied.

### Exact behaviour

Prologue `0x8065DA00..0x8065DA7C`: frame `0xA0`, `mflr` / `stw r0,164(r1)`,
`r11 = r1+0x60`, the four `f28..f31` spill pairs, then `bl 0x802DCD64`
(`_savegpr_23`). Then

```rust
let mut x       = f1;                                  // f28
let mut y       = f2;                                  // f29
let radius      = f3;                                  // f30
let owner       = r3;                                  // r27
let max_out     = r4 as i32;                           // r28
let mut out     = r5;                                  // r29
let z_const     = read_f32(0x807C8B0C);                // 5500.0f
let mut count   = 0i32;                                // r31
let mut node    = call2(0x80162DB0, 2, 0);             // first category-2 object
```

1. `0x8065DA7C..0x8065DAB8` - per node:
   ```rust
   while node != 0 {
       let mut pos = [0.0f32; 3];
       call2(0x8006CED0, pos.as_mut_ptr(), node);       // pos = node[+0xAC..] + node[+0xD0..]
       let dx = pos[0] - x;
       let dy = pos[1] - y;
       let d  = call1_f32(0x802E8868, dx * dx + dy * dy);   // sqrt
       if (d as f32) <= radius {                        // `frsp` then `fcmpo`
           ... dispatch (steps 2-6) ...
       }
       node = call2(0x80162DB0, 2, node);
   }
   ```
   `0x8006CED0` is the three-float `base + offset` copy: it reads
   `node[+0xB4]+node[+0xD8]`, `node[+0xB0]+node[+0xD4]`, `node[+0xAC]+node[+0xD0]`
   and stores them to `out+8`, `out+4`, `out+0`. `0x802E8868` is the engine
   square root (`frsqrte` refinement chain); its result is rounded to `f32` by
   `frsp` before the compare.
2. **Type `0x200`** (`0x8065DABC..0x8065DB68`):
   ```rust
   if read_u16(node + 8) == 0x200 {
       call1(0x80162510, node);
       let vt    = read_u32(node + 0x394) as usize;
       let iface = call1(read_u32(vt + 0x28), node + 0x394);
       let hit   = call2(read_u32(read_u32(iface) + 0x14), iface, 0x80B76630);
       let id    = if hit == 0 { 0x402u32 } else { 0x302u32 };
       let mut p = [pos[0], pos[1], z_const];
       call4(0x800925F0, read_u32(0x80429EE8), p.as_mut_ptr(), id, 0xFFFF_FFFF);
       if count < max_out {
           let mut q = [0.0f32; 3];
           call2(0x8006CED0, q.as_mut_ptr(), node);
           *out = q;                                    // 12 bytes
       }
       out = out.add(1);                                // 12 bytes, always
       count += 1;
   }
   ```
   The virtual chain is `(*(node+0x394)) + 0x28` returning an interface pointer,
   then `(*(iface)) + 0x14` with `0x80B76630`. The `hit == 0` test is the
   `neg`/`or`/`rlwinm ...,1,31,31` idiom; Ghidra folds it to
   `(int)(-u | u) < 0`. `0x800925F0` is
   `game::stage::fn_800925f0(owner, key: *mut f32, id: u32, mode: u8)`, called
   here with `mode = 0xFF`.
3. **Type `0x19B`** (`0x8065DB6C..0x8065DBC0`):
   ```rust
   if read_u16(node + 8) == 0x19B {
       let mut call = true;
       if read_u32(node + 0x60C) == 0 {
           let ok = read_u32(node + 0x61C) != 0
                 && read_u32(read_u32(node + 0x61C) + 0x18) != 0;
           if !ok { call = false; }
       }
       if call { call1(0x8011EBB0, node); }
   }
   ```
   Read the `+0x60C`/`+0x61C` gate exactly: the `+0x61C` chain is dereferenced
   twice, and a zero `+0x60C` with a zero `+0x61C+0x18` suppresses the call.
4. **Type `0x33`** (`0x8065DBC4..0x8065DC18`):
   ```rust
   let vt    = read_u32(node + 0x394) as usize;
   let iface = call1(read_u32(vt + 0x28), node + 0x394);
   let r = call2(read_u32(read_u32(iface) + 0x14), iface, 0x80358004);
   if r == 0 { call3(0x800A0B30, node, owner, 0); }
   ```
5. **Types `0x69`, `0x68`** (`0x8065DC18..0x8065DC3C`):
   ```rust
   if read_u16(node + 8) == 0x69 { call1(0x80040EC0, node); }
   if read_u16(node + 8) == 0x68 { call1(0x80123A00, node); write_u32(node + 0x3D4, 1); }
   ```
6. `0x8065DC40..0x8065DC4C` - advance, then loop; the epilogue at
   `0x8065DC58..0x8065DC94` restores `f28..f31` and `r23..r31` and returns
   `count`.

### Constants

| address | value | use |
|---|---|---|
| `0x807C8B0C` | `5500.0f` | `z` of the effect position |
| `0x80429EE8` | `u32` pointer | effect owner |
| `0x80B76630` | `d_enemiesNP` data object | interface name/key argument |
| `0x80358004` | `main.dol` BSS object | interface name/key for type `0x33` |
| `0x402` / `0x302` | effect ids | `hit == 0` / `hit != 0` |
| `2` | list key | category-2 object list |

### Callees

| target | name | shape |
|---|---|---|
| `0x802DCD64` / `0x802DCDB0` | `runtime::cpp` | `_savegpr_23` / `_restgpr_23` |
| `0x80162DB0` | `resource::registry::fn_80162db0` | `(key: u32, node: u32) -> u32` iteration |
| `0x8006CED0` | `main.dol`, untranslated (52 B) | `(*mut [f32;3], node)` = `pos + offset` |
| `0x802E8868` | `main.dol` | `(f32) -> f32` square root |
| `0x80162510` | `resource::registry::fn_80162510` | `(node)` prepare |
| `0x800925F0` | `game::stage::fn_800925f0` | `(owner, key: *mut f32, id: u32, mode: u8)` |
| `0x8011EBB0` | `game::worldmap::fn_8011ebb0` | `(node)` |
| `0x800A0B30` | `main.dol`, untranslated | `(node, owner, 0)` |
| `0x80040EC0` | `game::player::fn_80040ec0` | `(node)` |
| `0x80123A00` | `game::worldmap::fn_80123a00` | `(node)` |

### Notes for the implementer

* The out pointer advances by 12 bytes even when `count >= max_out`; only the
  copy is skipped. Keep that asymmetry.
* `count` is signed (`cmp cr0` + `bge`), so a negative `max_out` suppresses all
  copies but not the effects.
* The export's `ldexpf` noise is the `f28..f31` spill pair; there is no data-side
  `psq`.

---

## 5. `0x807B87F0` - scene/engine bootstrap: resource handles and system start

**Evidence.** `work/ghidra/rel-d_basesNP/export/807b87f0_FUN_807b87f0.c`, 660
bytes, 165 words, synthetic `0x807B87F0`. Body `0x807B87F0..0x807B8A84` (ends
`blr` at `0x807B8A80`). Frame `0x20`; `r29 = this`, `r30 = *(u32*)0x80429FC0`,
`r31 = 0` and later `0x8043xxxx` bases.

**What it is.** The one-shot bring-up of a scene: it resets the display lock,
clears two globals, creates twelve resource handles by id (five of them land in
`this+0x11D4`, `+0x11D8`, `+0x11DC`, `+0x11E0`, `+0x11E4`, one in the `+0xB0`
field of the handle at `+0x11D8`, one in the global `0x80429FDC`), then starts or
re-arms the engine subsystems in a fixed order and finishes with either the
"menu" or the "in-game" pair of display calls.

This is the function `src/game/bases.rs:183305` already calls by address
("Calls `0x807B87F0`, plays cue `699` for modes 2/3, stores the `0x2CD`
handle"), so the signature is fixed from the caller side as well.

### Signature

```rust
pub unsafe fn fn_807b87f0(this: usize)
```

No return value.

### Exact behaviour

Prologue `0x807B87F0..0x807B8814`: frame `0x20`, `mflr`, `stw r0,36(r1)`,
`stw r31,28(r1)`, `stw r30,24(r1)`, `stw r29,20(r1)`, `r29 = this`,
`r30 = read_u32(0x80429FC0)`, then `call1(0x807B8C50, this)`.

1. `0x807B8818..0x807B8828` - display lock off:
   ```rust
   call1(0x800D0A40, this + 0xC4);
   call2(0x800D0B20, this + 0xC4, 0);          // pause_menu_set_lock(ptr, 0)
   ```
2. `0x807B882C..0x807B8850` - subsystem resets, in order, with no arguments
   except where written:
   ```rust
   call0(0x80101990);
   let s = read_u32(0x8042A208);
   call1(0x80064B10, s); call1(0x800D9B30, s); call1(0x80072D00, s);
   call0(0x8008C480);
   call0(0x8007F880);
   ```
3. `0x807B8854..0x807B8874` - clear the two 4-byte globals and remember `r31 = 0`:
   ```rust
   write_u32(0x8042A01C, 0);
   write_u32(0x8042A020, 0);
   ```
4. `0x807B8878..0x807B88C4` - the resource-handle block, first half:
   ```rust
   call4(0x80162B00, 17,   this, 0, 0);
   write_u32(this, 0x11E0, call4(0x80162B00, 706, this, 0, 0));
   call0(0x8005ECB0);
   call0(0x800CD1A0);
   call4(0x80162B00, 18,   this, 0, 0);
   write_u32(0x8042A1CC, call5(0x80064610, 747, 0, 0, 0, 0));
   ```
5. `0x807B88E0..0x807B899C` - second half (note `0x11D4`, `0x11DC`, `0x11D8`
   are filled out of id order, and id `716` is stored *into* the handler created
   by id `715`):
   ```rust
   call4(0x80162B00, 748, this, 0, 0);
   call4(0x80162B00, 710, this, 0, 0);
   call4(0x80162B00, 719, this, 0, 0);
   write_u32(this, 0x11D4, call4(0x80162B00, 708, this, 0, 0));
   write_u32(this, 0x11DC, call4(0x80162B00, 707, this, 0, 0));
   write_u32(this, 0x11D8, call4(0x80162B00, 715, this, 0, 0));
   let h = read_u32(this, 0x11D8);
   write_u32(h, 0xB0, call4(0x80162B00, 716, this, 0, 0));
   write_u32(0x80429FDC, call4(0x80162B00, 709, this, 0, 0));
   write_u32(this, 0x11E4, call4(0x80162B00, 703, this, 0, 0));
   ```
6. `0x807B89A0..0x807B8A10` - subsystem starts, each with its own global:
   ```rust
   call1(0x80069150, read_u32(0x80429D50));
   call1(0x800E4830, 1);
   let mut zero = 0u32;
   call1(0x800E4850, &mut zero);                       // writes +0x14 of the dsys child
   call2(0x800E3BA0, read_u32(0x8042A078), read_u8(0x8042A21C));
   call1(0x80076BF0, read_u32(0x80429DC0));
   call1(0x800EB4C0, read_u32(0x8042A100));
   call1(0x80066490, read_u32(0x80429D40));
   call1(0x800A7450, read_u32(0x80429F10));
   call1(0x8008ECC0, read_u32(0x80429EA0));
   ```
7. `0x807B8A14..0x807B8A64` - the menu / in-game fork:
   ```rust
   if read_u8(0x8042A21C) != 0 {
       call1(0x800680B0, read_u32(0x80429D48));
   } else if read_u32(r30 + 0x14) == 2 {
       call1(0x80068140, read_u32(0x80429D48));
   } else {
       call1(0x800680B0, read_u32(0x80429D48));
       call1(0x800D0590, read_u32(0x80429FC8));
   }
   call0(0x800FFFE0);
   ```
   `r30` is `*(u32*)0x80429FC0` captured before step 1; `+0x14 == 2` is a
   three-way scene mode.
8. `0x807B8A68..0x807B8A84` - restore `r31`, `r30`, `r29`, `mtlr`,
   `addi r1,r1,32`, `blr`.

### Constants and globals

| address | type | use |
|---|---|---|
| `0x80429FC0` | `u32` | object whose `+0x14` selects the display arm |
| `0x8042A208` | `u32` | object passed to three resets |
| `0x8042A01C`, `0x8042A020` | `u32` | cleared |
| `0x8042A1CC` | `u32` | receives the `0x80064610` result |
| `0x80429FDC` | `u32` | receives handle `709` |
| `0x80429D50`, `0x8042A078`, `0x80429DC0`, `0x8042A100`, `0x80429D40`, `0x80429F10`, `0x80429EA0`, `0x80429FC8` | `u32` | one per subsystem call |
| `0x8042A21C` | `u8` | display-mode flag |
| `0x80429D48` | `u32` | display/menu object |
| ids | `17, 706, 18, 748, 710, 719, 708, 707, 715, 716, 709, 703` | resource handles (decimal, `li` immediates) |
| `747` | `u32` | `0x80064610` first argument |

`this` fields touched: `+0xC4` (passed to the two display calls), `+0x11D4`,
`+0x11D8`, `+0x11DC`, `+0x11E0`, `+0x11E4`, and `+0xB0` of the object at
`+0x11D8`.

### Callees

| target | name | shape |
|---|---|---|
| `0x807B8C50` | `game::bases::fn_807b8c50` | `(screen)` |
| `0x800D0A40` | `main.dol`, untranslated | `(this+0xC4)` |
| `0x800D0B20` | `engine::display::pause_menu_set_lock` | `(ptr, flag)` |
| `0x80101990`, `0x8008C480`, `0x8007F880`, `0x8005ECB0`, `0x800CD1A0`, `0x800FFFE0` | `main.dol`, untranslated | `()` |
| `0x80064B10`, `0x800D9B30`, `0x80072D00`, `0x80069150`, `0x80076BF0`, `0x800EB4C0`, `0x80066490`, `0x800A7450`, `0x8008ECC0`, `0x800680B0`, `0x80068140`, `0x800D0590` | `main.dol`, untranslated | `(r3 = the global shown)` |
| `0x80162B00` | `resource::registry::fn_80162b00` | `(id, this, 0, 0) -> u32 handle` |
| `0x80064610` | `main.dol`, untranslated (104 B) | `(747, 0, 0, 0, 0) -> u32` |
| `0x800E4830` | `engine::dsys::fn_800e4830` | `(u8 1)` |
| `0x800E4850` | `engine::dsys::fn_800e4850` | `(&u32 0)` |
| `0x800E3BA0` | `engine::dsys::fn_800e3ba0` | `(r3 = global, r4 = u8 flag)` - see note |

### Notes for the implementer

* `0x800E3BA0`'s raw body starts `cmpi r4,0` / `beq`, i.e. it **does** read
  `r3` and `r4` here, although the translated `src/engine/dsys.rs` entry declares
  no parameters. Pass both registers at the call site
  (`call2(0x800E3BA0, obj, flag)`); that is faithful to the instruction stream
  and does not depend on the Rust signature.
* `0x800E4850` writes `+0x14` of the dsys child to the value passed by pointer;
  here the value is a stack `0`, matching the `(value: u32)` reading.
* `0x80162B00` is called twelve times with `(id, this, 0, 0)`; every one of the
  twelve ids is listed above. Seven results are kept, five are dropped.
* `r31` doubles as the zero constant and the `0x8043xxxx` base; there is no
  branch depending on its value.

---

## 6. `0x80682AA0` - two orbiting 2D panes from a fixed-point angle

**Evidence.** `work/ghidra/rel-d_basesNP/export/80682aa0_FUN_80682aa0.c`, 656
bytes, 164 words, synthetic `0x80682AA0`. Body `0x80682AA0..0x80682D30` (ends
`blr` at `0x80682D2C`). Frame `0x90`; `r28 = 0x807C9718` (literal pool),
`r31 = this`; saves `f30`, `f31` (`stfd` + `psq_st ... 0,qr0`) and
`r28..r31`.

**What it is.** Positions two sibling 2D panes on two concentric circles around
the owner. The owner's `s16` angle at `+0x104`, offset by `+0x4000`, is converted
through the `qr3` slot to a float, scaled by `1/256` and fed to the engine's
`cos`/`sin` pair; the results scale radii `336.0` and `328.0` and the two
positions `(x + R cos, y - 400 + R sin, z - 40)` are written into the transform
blocks at `+0x7C`; the three `s16` values at `+0x100`, `+0x102`, `+0x104` are
pushed as translate/rotate/scale; then both layout objects at `+0xA20` and
`+0xA60` are committed.

### Signature

```rust
pub unsafe fn fn_80682aa0(this: usize)
```

No return value.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x7C` | transform block | `0x801C0BD0(out = this+0x7C, x, y, z)` |
| `+0xAC`, `+0xB0`, `+0xB4` | `f32` | owner position x, y, z |
| `+0x100` | `s16` | angle pushed to `0x8016ECB0` (rotation) |
| `+0x102` | `s16` | value pushed to `0x8016EDD0` |
| `+0x104` | `s16` | orbit angle **and** value pushed to `0x8016EEF0` |
| `+0xA20` | layout object | first pane |
| `+0xA60` | layout object | second pane |

### Exact behaviour

Prologue `0x80682AA0..0x80682AE8`: frame `0x90`, `mflr`, save `f31`, `f30`,
`r31..r28`, `r7 = r1+0x0E`, `r28 = 0x807C9718`, `r31 = this`, and the three
`lha` loads of `+0x100`, `+0x102`, `+0x104`.

1. `0x80682AEC..0x80682B58` - stack staging (all of it is read later):
   ```rust
   let x = read_f32(this + 0xAC);
   let y = read_f32(this + 0xB0);
   let z = read_f32(this + 0xB4);
   let a1 = read_s16(this + 0x100);
   let a2 = read_s16(this + 0x102);
   let a3 = read_s16(this + 0x104);
   let one = 1.0f32;                       // 0x807C9718+0x24, three copies at sp+0x3C..0x44
   let ybase = y - 400.0;                  // 0x807C9718+0x3C = 400.0
   let zbase = z - 40.0;                   // 0x807C9718+0x78 = 40.0
   let rad1 = 336.0f32;                    // 0x7C
   let rad2 = 336.0 - 8.0;                 // 328.0 from 0x7C/0x80
   write_u16(sp + 0x0E, (a3.wrapping_add(0x4000)) as u16);
   ```
   The three stack triples are `sp+0x3C` = `(1,1,1)`, `sp+0x48` = `(x, y, z-40)`
   and `sp+0x30` = `(x, y-400, z)`; the last two are working copies of the
   position that the `cos`/`sin` results are added into.
2. `0x80682B50..0x80682BAC` - `cos` arm and the first transform:
   ```rust
   let angle = (a3.wrapping_add(0x4000) as u16) as f32 * 0.00390625f32; // psq_l W=1 qr3, then /256
   let c = call1_f64(0x80237C40, angle as f64) as f32;       // cos
   let px = x + rad1 * c;                                    // sp+0x48
   let s = call1_f64(0x80237BD0, angle as f64) as f32;       // sin
   let py = ybase + rad1 * s;                                // sp+0x4C
   call4(0x801C0BD0, this + 0x7C, px, py, zbase);            // translation matrix
   ```
3. `0x80682BB0..0x80682C10` - push the three angles and commit pane 1:
   ```rust
   call2(0x8016EDD0, this + 0x7C, &(a2 as u16));
   call2(0x8016ECB0, this + 0x7C, &(a1 as u16));
   call2(0x8016EEF0, this + 0x7C, &(a3 as u16));
   call2(0x8016A170, this + 0xA20, this + 0x7C);     // attach transform
   call2(0x8016A150, this + 0xA20, &one_triple);     // scale (1,1,1)
   call2(0x8016A1A0, this + 0xA20, 0);
   call2(read_u32(read_u32(this + 0xA20) + 0x14), this + 0xA20);  // virtual
   ```
   The three setters take a pointer to a `u16`, not the value: the body stages
   each in its own stack slot (`sp+0x24`, `sp+0x20`, `sp+0x1C`).
4. `0x80682C24..0x80682CFC` - identical second arm with radius `328.0` and the
   second pane:
   ```rust
   let angle = (a3.wrapping_add(0x4000) as u16) as f32 * 0.00390625f32; // same staging, second slot
   let c = call1_f64(0x80237C40, angle as f64) as f32;
   let px = x + rad2 * c;
   let s = call1_f64(0x80237BD0, angle as f64) as f32;
   let py = ybase + rad2 * s;
   call4(0x801C0BD0, this + 0x7C, px, py, zbase);
   call2(0x8016EDD0, this + 0x7C, &(a2 as u16));
   call2(0x8016ECB0, this + 0x7C, &(a1 as u16));
   call2(0x8016EEF0, this + 0x7C, &(a3 as u16));
   call2(0x8016A170, this + 0xA60, this + 0x7C);
   call2(0x8016A150, this + 0xA60, &one_triple);
   call2(0x8016A1A0, this + 0xA60, 0);
   call2(read_u32(read_u32(this + 0xA60) + 0x14), this + 0xA60);
   ```
   Both arms write the *same* `+0x7C` transform, and pane 1 is committed before
   arm 2 rebuilds it, so the order is load-bearing: build, set the three values,
   attach, commit, virtual - then rebuild for pane 2. Do not hoist the second
   `0x801C0BD0`.
5. `0x80682D00..0x80682D30` - epilogue: restore `f31`, `f30`, `r28..r31`,
   `mtlr`, `addi r1,r1,144`, `blr`.

### Constants

| address | value | use |
|---|---|---|
| `0x807C9718+0x24` | `1.0f` | the `(1,1,1)` scale triple |
| `0x807C9718+0x3C` | `400.0f` | `y` origin shift |
| `0x807C9718+0x78` | `40.0f` | `z` shift |
| `0x807C9718+0x7C` | `336.0f` | outer radius |
| `0x807C9718+0x80` | `8.0f` | `328 = 336 - 8` |
| `0x807C9718+0x84` | `0.00390625f` = `1/256` | fixed-point angle scale |
| `0x4000` | `s16` bias | quarter turn before the `cos` |
| GQR3 | `0x00050005` | `psq_l` type 5 (u16), scale 0 |

### Callees

| target | name | shape |
|---|---|---|
| `0x80237C40` | `resource::bres::fn_80237c40` | `cos(f64) -> f64`, called as `call1_f64(..., angle as f64) as f32` |
| `0x80237BD0` | `resource::bres::fn_80237bd0` | `sin(f64) -> f64` |
| `0x801C0BD0` | `runtime::dvd::fn_801c0bd0` | `(out: transform, x, y, z)` - writes a 3x4 translation matrix |
| `0x8016EDD0` | `resource::registry::fn_8016edd0` | `(transform: u32, *const u16)` |
| `0x8016ECB0` | `ui::lyt::fn_8016ecb0` | `(transform, *const u16 angle)` |
| `0x8016EEF0` | `resource::registry::fn_8016eef0` | `(transform, *const u16)` |
| `0x8016A170` | `resource::registry::fn_8016a170` | `(layout, value: u32)` |
| `0x8016A150` | `resource::registry::fn_8016a150` | `(layout, *const [u32; 3])` |
| `0x8016A1A0` | `resource::registry::fn_8016a1a0` | `(layout, flag: i32)` |
| `layout -> +0x14` | virtual | `(layout)` |

### Notes for the implementer

* The `qr3` load is not a decompiler artefact: `psq_l f1,0(r7),1,qr3` is a real
  single-element (`W = 1`) quantised load, and the body multiplies the result by
  `1/256`. With `GQR3 = 0x00050005` it is exactly
  `((a3 + 0x4000) as u16) as f32 * (1.0 / 256.0)`, i.e. an angle in
  `[0, 256)` with a full turn = `256.0`.
* `0x80237C40` measured `cos`, `0x80237BD0` measured `sin`
  (`src/resource/bres.rs:13366`/`:13412`); the call convention already used in
  this tree is `w108_call_f64(0x8023_7BD0, angle as f64) as f32`
  (`src/engine/display.rs:22622`).
* `0x801C0BD0` writes a full identity-with-translation matrix into `this+0x7C`
  (`+0x00/+0x04/+0x08` = `1,0,0`; `+0x0C` = x; ... `+0x2C` = z), so calling it
  twice only changes the translation.

---

## 7. `0x8068F220` - spawn one object from the 40-byte record table

**Evidence.** `work/ghidra/rel-d_basesNP/export/8068f220_FUN_8068f220.c`, 656
bytes, 164 words, synthetic `0x8068F220`. Body `0x8068F220..0x8068F4B0` (ends
`blr` at `0x8068F4AC`). Frame `0x50`; `_savegpr_23` at `0x802DCD64`
(`r23..r31`), restore `0x802DCDB0`; `r26 = this`, `r27 = spec`, `r28 = flags`,
`r29 = extra`.

**What it is.** A generic "spawn one object of record `N`" body. The record is
`0x8030A040 + *(u16*)spec * 40`; the record's fields become the new object's
timers/scale and are also copied into four engine globals
(`0x80429D18/0x80429D20/0x80429D24/0x80429D28`). A screen-effect arm runs first
and is selected by bit 3 of the record's `+0x24` word and by `spec[0xD]`. The
already-spawned flag is bit 0 of `*flags`.

The record table is **zero at file level** (`0x8030A040` is in `main.dol`
segment 7 but reads as zeros) and is filled at run time, so every record field
is an input.

### Signature

```rust
pub unsafe fn fn_8068f220(this: usize, spec: usize, flags: *mut u8, extra: u32)
```

`r3` = owner, `r4` = the spec block, `r5` = a byte pointer (bit 0 is set on
success), `r6` = a value stored verbatim at `new + 0x350`. No return value. The
body writes the incoming `f1` to a stack slot it never reads, so it has **no**
float parameter - do not add one.

### Field map

| offset | type | meaning |
|---:|---|---|
| `spec+0x00` | `u16` | record index (`* 40`) |
| `spec+0x02` | `u16` | added to `record+0x04` |
| `spec+0x04` | `u16` | subtracted from `record+0x08` |
| `spec+0x06` | `u16` | becomes global `0x80429D18` |
| `spec+0x08` | `u32` | `0x80064680` argument 3 |
| `spec+0x0D` | `u8` | effect-arm selector; `0x80064680` argument 6 |
| `record+0x00` | `u16` | object id (`0x801017B0` index) |
| `record+0x04` | `u32` | added to `spec+0x02` |
| `record+0x08` | `u32` | subtracted from `spec+0x04` |
| `record+0x14` | `f32` | scale x source |
| `record+0x18` | `f32` | scale y source |
| `record+0x24` | `u16` | bit 3 selects the effect arm; also copied to `new+0x364` |
| `new+0x318`, `new+0x31C` | `f32` | `max(v, 512.0) * 2.0` |
| `new+0x349` | `u8` | player/section index |
| `new+0x34C` | `u32` | `flags` pointer |
| `new+0x350` | `u32` | `extra` |
| `new+0x354` | `u16` | global `0x80429D18` |
| `new+0x358`, `new+0x35C` | `u32` | globals `0x80429D20`, `0x80429D24` |
| `new+0x364` | `u16` | `record+0x24` |

### Exact behaviour

Prologue `0x8068F220..0x8068F26C`: frame `0x50`, `mflr`, `r11 = r1+0x50`,
`bl 0x802DCD64` (`_savegpr_23`), then

```rust
let spec  = r4;
let flags = r5 as *mut u8;
let extra = r6;
let this  = r3;                                  // r26
let index = read_u16(spec) as usize;
let record = 0x8030A040 + index * 40;             // r30
let id = read_u16(record);                        // r31
```

1. `0x8068F268..0x8068F26C` - `call1(0x801017B0, id)` (returns a `u32`; the
   following `stfs f1,16(r1)` stores the **incoming** `f1` and is a dead store -
   `r1+16` is never read).
2. `0x8068F270..0x8068F278` - `if read_u8(flags) & 8 != 0 { return; }`.
3. `0x8068F27C..0x8068F2DC` - the screen-effect arm, `g = read_u32(0x80429D48)`:
   ```rust
   let selector = read_u8(spec + 0x0D) != 0;
   if read_u16(record + 0x24) & 8 != 0 {
       if selector { call1(0x80068D70, g); } else { call1(0x80068CE0, g); }
   } else {
       if selector { call1(0x80068C50, g); } else { call1(0x80068BC0, g); }
   }
   ```
   The four callees are the `main.dol` screen-shake helpers; `0x80068BC0` /
   `0x80068C50` read `param[0]` / `param[4]` and `0x80068D70` reads `param+0xC`,
   so they are distinct arms of the same subsystem and must not be collapsed.
4. `0x8068F2DC..0x8068F340` - the two fixed-point values handed to the spawn
   call:
   ```rust
   let dy = (read_u32(record + 0x08) as i32 - read_u16(spec + 0x04) as i32) as f32;
   let dx = (read_u32(record + 0x04) as i32 + read_u16(spec + 0x02) as i32) as f32;
   // stored as the pair (dx at r1+0x08, dy at r1+0x0C)
   ```
   Both conversions are the signed-`s32` -> `f64` idiom against
   `0x807C9C58` (`2^52 + 2^31`), then narrowed by `fsubs`; the intermediate is a
   `f32`. Note the `neg` in the assembly: `subf r5,r7,r5` with
   `r5 = spec+4`, `r7 = record+8` gives `spec4 - rec8`, and `neg` turns it into
   `rec8 - spec4`.
5. `0x8068F344..0x8068F380` - the four engine globals:
   ```rust
   write_u16(0x80429D18, read_u16(spec + 0x06));
   write_u8 (0x80429D28, read_u8 (spec + 0x0D));
   let hi = read_u8(0x80429D19);                 // high byte of the value just stored
   let lo = read_u8(0x80429D18);                 // low byte
   write_u32(0x80429D20, 0);
   write_u32(0x80429D24, 0);
   if hi != 0 { let (a, b) = call3_pair(0x802DD1EC, 0, 1, hi as u32 - 1);
                write_u32(0x80429D20, a); write_u32(0x80429D24, b); }
   if lo != 0 { let (a, b) = call3_pair(0x802DD1EC, 0, 1, lo as u32 - 1);
                write_u32(0x80429D20, a); write_u32(0x80429D24, b); }
   ```
   `0x802DD1EC` is `runtime::cpp::fn_802dd1ec`, the 64-bit shift helper: it
   returns the low word in `r3` and the high word in `r4`, which is why `r3`
   lands at `0x80429D20` and `r4` at `0x80429D24`. The second arm overwrites the
   first when both bytes are non-zero.
6. `0x8068F3AC..0x8068F3C4` - the spawn:
   ```rust
   let new = call6(0x80064680, id, this, read_u32(spec + 0x08),
                   &pair /* r1+0x08 */, 0, read_u8(spec + 0x0D));
   if new == 0 { return; }
   ```
7. `0x8068F3C8..0x8068F420` - fill the new object:
   ```rust
   write_u8 (new, 0x349, read_u8(read_u32(0x8042A1C8) + 0x120F));
   write_u32(new, 0x34C, flags as u32);
   write_u32(new, 0x350, extra);
   write_u16(new, 0x354, read_u16(0x80429D18));
   write_u32(new, 0x358, read_u32(0x80429D20));
   write_u32(new, 0x35C, read_u32(0x80429D24));
   write_u16(new, 0x364, read_u16(record + 0x24));
   ```
8. `0x8068F424..0x8068F488` - the scales (each clamped up to `512.0`, then
   doubled):
   ```rust
   let sy = read_u32(record + 0x18) as i32 as f32;
   let sx = read_u32(record + 0x14) as i32 as f32;
   let sy = if sy < 512.0 { 512.0 } else { sy };
   let sx = if sx < 512.0 { 512.0 } else { sx };
   write_f32(new, 0x318, sx * 2.0);
   write_f32(new, 0x31C, sy * 2.0);
   ```
   `record+0x14`/`+0x18` are read as **`u32`/`s32`**, not `f32` - the fields are
   `lwz` + `xoris` + `lfd`, so the record stores integers there.
9. `0x8068F48C..0x8068F494` - `write_u8(flags, read_u8(flags) | 1)`;
   `0x8068F498..0x8068F4B0` restores `r23..r31` and returns.

### Constants

| address | value | use |
|---|---|---|
| `0x8030A040` | 40-byte record table | `index * 40`, zero at file level |
| `0x807C9C58` | `f64 2^52 + 2^31` | `s32 -> f64` magic (three uses) |
| `0x807C9C50` | `512.0f` | lower clamp for both scales |
| `0x807C9C54` | `2.0f` | scale multiplier |
| `0x80429D18` | `u16` | spec timer/word |
| `0x80429D19` | `u8` | high byte of the above |
| `0x80429D20`, `0x80429D24` | `u32` pair | `0x802DD1EC` result |
| `0x80429D28` | `u8` | `spec[0xD]` latch |
| `0x80429D48` | `u32` | screen-shake object |
| `0x8042A1C8` (`+0x120F`) | `u32` | stage-info; byte = section index |

### Callees

| target | name | shape |
|---|---|---|
| `0x802DCD64` / `0x802DCDB0` | `runtime::cpp` | `_savegpr_23` / `_restgpr_23` |
| `0x801017B0` | `engine::globals::fn_801017b0` | `(index: u32) -> u32` |
| `0x80068BC0`, `0x80068C50`, `0x80068CE0`, `0x80068D70` | `main.dol`, untranslated (100 B each) | `(screen_obj)` |
| `0x802DD1EC` | `runtime::cpp::fn_802dd1ec` | `(0, 1, n - 1) -> (low: r3, high: r4)` |
| `0x80064680` | `main.dol`, untranslated (104 B) | `(id, owner, u32, *pair, 0, u8) -> *mut Obj` |

### Notes for the implementer

* `0x80064680` forwards to `0x8006CBA0` after `0x80064600(param_6)`; the six
  arguments are taken from `r3..r8` exactly as listed.
* The `stfs f1,16(r1)` at `0x8068F308` is dead - `r1+16` is never reloaded - and
  is the only trace of the "float parameter" Ghidra invents for this body.
* Bit 3 (`0x8`) of `*flags` is the "already spawned" early-out; it is the same
  flag whose bit 0 is set at the end.

---

## 8. `0x80638120` - four-slot sequence: spawn arm and despawn arm

**Evidence.** `work/ghidra/rel-d_basesNP/export/80638120_FUN_80638120.c`, 648
bytes, 162 words, synthetic `0x80638120`. Body `0x80638120..0x806383A8` (ends
`blr` at `0x806383A4`). Frame `0x20`; `_savegpr_26` at `0x802DCD70`, restore
`0x802DCDBC`; `r31 = this`.

**What it is.** The tick of a four-slot staged sequence (four sub-objects at
`this + i*0x3C`). The body first asks the engine allocator for `0x900` bytes; a
non-zero result selects the **spawn** arm (drain each slot's pending count into
its accumulator, latch the slot, start its channel, note whether any slot still
has a gate, play cue `150`), a zero result selects the **despawn** arm (count
down `this+0x8C0` first, then per slot either advance or release, play cue `150`
or `151`, and call the screen's virtual `+0x18`). Both arms end by calling
`0x80637050(this)`.

### Signature

```rust
pub unsafe fn fn_80638120(this: usize)
```

No return value.

### Field map

Per slot base `b = this + i * 0x3C`, `i = 0..4`:

| offset | type | meaning |
|---:|---|---|
| `b+0x7C0` | `s32` | accumulator (`+= pending` in the spawn arm; `+= 1` per frame in the despawn arm) |
| `b+0x7C8` | `s32` | pending/remaining count |
| `b+0x7CC` | `u32` | spawn gate ("still busy") |
| `b+0x7E0` | `u32` | channel index base |
| `b+0x7E5` | `u8` | slot active |
| `b+0x7E6` | `u8` | "channel started" latch |

Owner-level:

| offset | type | meaning |
|---:|---|---|
| `+0x70` | stage/channel owner | `0x800C92F0(+0x70, index, 0)` |
| `+0x6A8` | vtable object | `+0x18` called with `0x80826BE0` |
| `+0x8C0` | `s32` | despawn-arm delay; decremented and returned while non-zero |

### Exact behaviour

Prologue `0x80638120..0x80638134`: frame `0x20`, `mflr`, `stw`, `r11 = r1+0x20`,
`bl 0x802DCD70`, `r31 = this`.

1. `0x80638134..0x80638158` - the allocator probe:
   ```rust
   let alloc = read_u32(0x8042A468);
   let block = call2(read_u32(read_u32(alloc) + 0x20), alloc, 0x900);  // 2304 bytes
   if block != 0 { /* spawn arm */ } else { /* despawn arm */ }
   ```
2. **Spawn arm** `0x8063815C..0x80638214`:
   ```rust
   let mut busy = false;                       // r27
   for i in 0..4 {
       let b = this + i * 0x3C;
       if read_u8(b + 0x7E5) == 0 { continue; }
       write_s32(b, 0x7C0, read_s32(b, 0x7C0) + read_s32(b, 0x7C8));
       write_s32(b, 0x7C8, 0);
       if read_u8(b + 0x7E6) == 0 {
           write_u8(b, 0x7E6, 1);
           call3(0x800C92F0, this + 0x70, read_u32(b + 0x7E0) + 25, 0);
       }
       if read_u32(b + 0x7CC) != 0 { busy = true; }
   }
   call1(0x80637050, this);
   if busy {
       call3(0x801953A0, read_u32(0x8042A488), 150, 1);
   }
   let vt = read_u32(this + 0x6A8) as usize;
   call3(read_u32(vt + 0x18), this + 0x6A8, 0x80826BE0, 0);
   ```
   The `r26` counter incremented at `0x806381C4` is dead in this arm (it is
   loaded from 0 and never read again); a faithful body may drop it.
3. **Despawn arm** `0x80638218..0x8063838C`:
   ```rust
   if read_s32(this + 0x8C0) != 0 {
       write_s32(this + 0x8C0, read_s32(this + 0x8C0) - 1);
       return;
   }
   write_s32(this + 0x8C0, 0);
   let mut idle = 0i32;                        // r26
   for i in 0..4 {
       let b = this + i * 0x3C;
       if read_u8(b + 0x7E5) == 0 { idle += 1; continue; }
       let mut c = read_s32(b + 0x7C8);
       let d = read_u32(b + 0x7E0);
       if c != 0 {
           write_s32(b, 0x7C0, read_s32(b, 0x7C0) + 1);
           c -= 1;
       } else if read_u8(b + 0x7E6) == 0 {
           write_u8(b, 0x7E6, 1);
           call3(0x800C92F0, this + 0x70, d + 25, 0);
           idle += 1;                          // only on the transition frame
       }
       write_s32(b, 0x7C8, c);
   }
   call1(0x80637050, this);
   if idle < 4 {
       call3(0x801953A0, read_u32(0x8042A488), 150, 1);
   } else {
       let mut any = 0u32;
       for i in 0..4 {
           let b = this + i * 0x3C;
           if read_u8(b + 0x7E5) != 0 && read_u32(b + 0x7CC) != 0 { any = 1; }
       }
       if any != 0 {
           call3(0x80195380, read_u32(0x8042A488), 151, 1);
       }
       let vt = read_u32(this + 0x6A8) as usize;
       call3(read_u32(vt + 0x18), this + 0x6A8, 0x80826BE0, 0);
   }
   ```
   The final four pairs are unrolled in the assembly as
   `(+0x7E5, +0x7CC)`, `(+0x821, +0x808)`, `(+0x85D, +0x844)`,
   `(+0x899, +0x880)` - i.e. exactly `b+0x7E5` / `b+0x7CC` for `i = 0..4` with
   `b = this + i*0x3C`. `idle` increments are **not** symmetric: an active slot
   contributes only on the frame its latch flips, an inactive slot contributes
   every frame.
4. `0x80638390..0x806383A8` - epilogue: `r11 = r1+0x20`,
   `bl 0x802DCDBC`, `lwz r0,36(r1)`, `mtlr`, `addi r1,r1,32`, `blr`.

### Constants

| address | value | use |
|---|---|---|
| `0x8042A468` | `u32` | allocator object; virtual `+0x20` |
| `0x900` | 2304 | allocation size |
| `0x8042A488` | `u32` | audio manager |
| `150`, `151` | cue ids | handed to the `+0x8FC` (`150`) / `+0x8F8` (`151`) operators |
| `0x80826BE0` | REL BSS | argument of the `+0x6A8` virtual |
| `0x3C` | 60 | slot stride |
| `25` | `u32` | added to `b+0x7E0` for the channel index |

### Callees

| target | name | shape |
|---|---|---|
| `0x802DCD70` / `0x802DCDBC` | `runtime::cpp` | `_savegpr_26` / `_restgpr_26` |
| `0x8042A468 -> +0x20` | virtual | `(alloc, 0x900) -> *mut u8` |
| `0x800C92F0` | `game::stage::fn_800c92f0` | `(owner, index: i32, offset: i32)` |
| `0x80637050` | `game::bases::fn_80637050` | `(screen)` |
| `0x801953A0` | `audio::ax::fn_801953a0` | `(manager, cue: u32, 1)` - `manager+0x8FC` operator |
| `0x80195380` | `audio::ax::fn_80195380` | `(manager, cue: u32, 1)` - `manager+0x8F8` operator |
| `this+0x6A8 -> +0x18` | virtual | `(this+0x6A8, 0x80826BE0)` |

### Notes for the implementer

* The allocator probe is not a null test on a cached pointer: it is a virtual
  call with a size, made on **every** frame, and its result only selects the
  arm. Do not hoist or memoise it.
* `0x80637050(this)` runs in both arms, before the cue and before the virtual.
* `0x801953A0`/`0x80195380` are the two 4-register sequence operators of the AX
  manager (`manager+0x8FC` and `manager+0x8F8`), so `150` and `151` are two
  different operator channels, not "start"/"stop" of one.

---

## Verification

* Sizes: every `bytes` column equals `functions.tsv` (`676, 668, 668, 660, 660,
  656, 656, 648`); `decompiled = true` for all eight; each `.c` export exists on
  disk.
* Spans: exactly one `blr` per body at `address + size - 4`; no `b`/`bc` leaves
  its extent; no `halt_baddata`, no jumptable warning, no unhandled opcode.
* Filter: `work/ghidra/rel-d_basesNP/export/functions.tsv` has 5,615 rows;
  1,863 survive the ledger + heading + `decompiled` filter (see "This is the
  list wave 150 predicted").
* Absence: all eight addresses are absent from `ledger/functions.csv` (exact
  `address` field), from every `#{1,6} ` heading in `docs/*.md`, and from
  `src/game/bases.rs` as `fn_<addr>` tokens. They *are* mentioned in prose in
  `docs/COVERAGE_WAVE25_BASES.md`, `docs/COVERAGE_WAVE93_BASES.md`,
  `docs/COVERAGE_WAVE105_BASES.md`, `docs/COVERAGE_WAVE120_BASES.md`,
  `docs/COVERAGE_WAVE134_BASES.md`, `docs/COVERAGE_WAVE138_BASES.md`,
  `docs/COVERAGE_WAVE143_BASES.md` and `docs/SPEC_WAVE150_BASES.md` (as the
  predicted next eight) - body text, never a heading.
* Duplicates: SHA-1 of each body against every same-size `d_basesNP` row already
  in the ledger - no hit. The eight hashes are
  `6572f0f4d8fc07cb4a7b8c2ff3900975d5dbf17f` (676),
  `713bc521bba34efd6adce504ca2d9484c4e2e4ba` and
  `569c0257d7a140f3c432b6f399e8c98ff7e822f0` (668),
  `93b11dbc81373ec461866c4922421dd24f3b1b3e` and
  `ba2590856c2398daab2b78aa7fbeb39cf28e85e4` (660),
  `ebf52c9dfd039a76be607978ae199726e859a295` and
  `df9231fc7142991e99ca5517a67459bdaf496ec2` (656),
  `fc296cfbb606f823b18a142795c32e6acd47ff7c` (648).
* Callers (static `bl` scan of the whole REL): `0x806C1040` <- `0x806C1500`,
  `0x8070B550` <- `0x8070B524`, `0x8065DA00` <- `0x8065CA10`, `0x807B87F0` <-
  `0x807B7098` (and `src/game/bases.rs:183305` from the Rust side),
  `0x80682AA0` <- `0x80682840`, `0x8068F220` <- `0x8068F1A8`. `0x806C7C20` and
  `0x80638120` have no static caller in this REL - they are virtual/state
  entries, which is why their bodies end in a virtual call.
