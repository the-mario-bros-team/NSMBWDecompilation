# Wave 143 - `game::bases` - the eight largest remaining bodies

Read-only specification lane. No Rust file, no `ledger/functions.csv` row and no
existing document was modified. The only file written is this one.

Waves 128-142 claimed 120 rows out of
`work/ghidra/rel-d_basesNP/export/functions.tsv`. This wave continues one rank
lower. The window is now 1.0-1.1 KB per body (8,704 bytes / 2,176
instructions), so every row gets a full field map, a numbered behaviour list and
a constant table.

## The eight

| # | address | bytes | words | export file | verdict |
|--:|---:|---:|---:|---|:--:|
| 1 | `0x807A84B0` | 1,124 | 281 | `807a84b0_FUN_807a84b0.c` | ready (medium) |
| 2 | `0x80772370` | 1,108 | 277 | `80772370_FUN_80772370.c` | ready (medium) |
| 3 | `0x806639A0` | 1,096 | 274 | `806639a0_FUN_806639a0.c` | ready (medium) |
| 4 | `0x807BEF60` | 1,096 | 274 | `807bef60_FUN_807bef60.c` | conditional (medium) |
| 5 | `0x8074C540` | 1,092 | 273 | `8074c540_FUN_8074c540.c` | ready (medium) |
| 6 | `0x8075D5C0` | 1,080 | 270 | `8075d5c0_FUN_8075d5c0.c` | ready (medium) |
| 7 | `0x80763E50` | 1,056 | 264 | `80763e50_FUN_80763e50.c` | ready (medium) |
| 8 | `0x806765B0` | 1,052 | 263 | `806765b0_FUN_806765b0.c` | conditional (medium) |

**Ready: 6** (`0x807A84B0`, `0x80772370`, `0x806639A0`, `0x8074C540`,
`0x8075D5C0`, `0x80763E50`).
**Conditional: 2** (`0x807BEF60`, `0x806765B0`, both for a stated single input).

Every `bytes` value is the `size` column of the TSV verbatim, and in all eight
cases the size is the true instruction span: the decoder found the final `blr`
at `address + size - 4` in each of the eight bodies. There is no padding tail
and no `halt_baddata` cut this wave.

### Verdict rule used here

* **ready** - every reachable instruction is decoded, every literal is read out
  of the image, every callee is named by address, and the body can be written
  as code with no assumed convention. Data that lives in `main.dol` BSS
  (`0x8035xxxx`, `0x8042xxxx`) is an *input*, not an unknown constant, and is
  named by address.
* **conditional** - exactly one named input is missing: `0x807BEF60` needs the
  pane roles of its two layout owners (`+0x184` / `+0x294` / `+0x3A4` / `+0x404`)
  before the four `0x802D1210` / `0x802BFFF0` argument groups can be assigned
  meaning; `0x806765B0` indexes a literal pool that two interpretations share
  (see its section).

### `0x807A84B0` is the top row, and it was never claimed

Wave 142's "next" list started at `0x80772370` and skipped `0x807A84B0`. That
row is *larger* (1,124 > 1,108) and it is absent from `ledger/functions.csv`
and from every heading in `docs/`. It only ever appeared in **prose**: waves
73/74/99 rejected a "paired-single exporter"
(`docs/COVERAGE_WAVE73_BASES.md:125`, `docs/COVERAGE_WAVE74_BASES.md:136`,
`docs/COVERAGE_WAVE99_BASES.md:131`), wave 127 listed it as a mixed `unaff_GQR0`
lane (`docs/COVERAGE_WAVE127_BASES.md:113`), and the wave-73 Rust note repeats
the rejection at `src/game/bases.rs:426` and `src/game/bases.rs:84260`.

That rejection is **void on the shipped bytes**. The body contains exactly
**eight** paired-single instructions: four `psq_st` (`f31`, `f30`, `f29`, `f28`)
in the prologue at `0x807A84CC..0x807A84E4` and their four `psq_l` at
`0x807A88E0..0x807A88F8`. All eight are `W=0, qr0` - the plain `f32` pair spill
described in `docs/REL_ELF_MAPPING.md`. There is no GQR read, no
quantiser index other than 0, and no `ps_*` arithmetic anywhere in the body.
`ldexpf` in the export is the decompiler's rendering of the pair spill, and the
three `CONCAT44` ladders are ordinary CodeWarrior unsigned-to-float idioms (the
`0x43300000` magic at `0x807D0688`/`0x807D06A8`). The same correction wave 142
applied to `0x806D09B0` / `0x806B8380` applies here; the "paired-single family"
note in `COVERAGE_WAVE73/74/99/127` must not be used to skip this address again.

The two other rows that carry `psq` (`0x807BEF60`, `0x8075D5C0`) are the same
`qr0` `f31`/`f30` spill pair and nothing else.

### Selection rule

Candidate set: every `functions.tsv` row of `rel-d_basesNP` whose address is

1. absent from the `address` column of `ledger/functions.csv` (all modules), and
2. absent from every `#{1,6} ` heading of every `docs/*.md`.

Ranked by `size` descending, the first eight are the table above. Applying the
filter to the current tree yields **1,985** candidate rows, all of which have an
export `.c` on disk (no row was skipped for a missing export). Wave 142 reported
2,006 rows *before* it claimed its eight; 2,006 - 8 = 1,998, so the residual
thirteen rows of difference are the address-bearing headings that wave 142's own
document added under its correction and verification subsections.

The eight rows immediately below the cut, for a later lane:
`0x80609440` (1,040), `0x80757320` (1,032), `0x80741510` (1,016),
`0x8077AC30` (1,012), `0x80786EA0` (1,008), `0x807886E0` (1,004),
`0x8065C370` (976), `0x80628A00` (972) / `0x80630820` (972).

## Method

**Address mapping.** Re-derived from the program headers of
`work/rels/elf/d_basesNP.elf` for this pass, not copied:

```
PT_LOAD  p_offset  p_vaddr     filesz    memsz
  0      0x0010F0  0x806000F0  0x1C6004  0x1C6004
  1      0x1C70F4  0x807C60F4  0x0004FC  0x0004FC
  2      0x1C75F0  0x807C65F0  0x00000C  0x00000C
  3      0x1C7600  0x807C6600  0x00A5FC  0x00A5FC
  4      0x1D1C00  0x807D0C00  0x051CB8  0x051CB8
  5      0x000000  0x80822900  0x000000  0x012484 (zero fill)
```

so `file_offset = 0x10F0 + (address - 0x806000F0) = (address - 0x80600000) + 0x1000`
for every code address in this document, as `docs/REL_ELF_MAPPING.md` requires.
Literal pools in `0x807Cxxxx` / `0x807D0xxx` resolve through the same rule and
are read out verbatim below. `0x8035xxxx` and `0x8042xxxx` are `main.dol` BSS
and read as zero in the shipped image; they are named, never guessed.

**Decoder.** A throwaway PowerPC decoder was written under `%TEMP%\opencode`
(never in the repo) and validated against the facts `docs/REL_ELF_MAPPING.md`
pins:

* `psq_l` = 56, `psq_st` = 60; quantiser index in bits 12-14, `W` in bit 15.
  The eight bodies contain 16 `psq_` words, all `qr0/W=0` (four spill pairs in
  row 1, one pair each in rows 4 and 6, none in the other five).
* For opcode 4 the extended opcode is five bits (`(word >> 1) & 0x1F`).
* `rlwinm` fields were verified numerically per instruction rather than by
  hand: several of the bit-field extractions below (`0x806765B0`) look like
  rotate-and-mask idioms and are reported as the plain nibble reads they are.

Register recovery: `0x802DCD54/60/6C/70` are `_savegpr` helpers and
`0x802DCDA0/B8/BC` the matching `_restgpr`; the incoming `r3` survives both.
Every body below was re-read with `this` taken from the surviving register, not
from Ghidra's `void`/packed signature.

---

## 1. `0x807A84B0` - the `otasukeInfo` backdrop/scissor rebuild

**Evidence.** `work/ghidra/rel-d_basesNP/export/807a84b0_FUN_807a84b0.c`,
1,124 bytes, 281 words, synthetic `0x807A84B0`.

**What it is.** The 2D backdrop rebuild of the help screen whose helpers
`0x807A6B20..0x807A7A20` and `0x807A8DB0..0x807A9970` are already translated
(`docs/COVERAGE_WAVE73_BASES.md`, `src/game/bases.rs:84250`). Five independent
identifiers tie it to that screen family and to no other body in the REL:

* it spans the **two BG colours** `0x8042A0EC` / `0x8042A0F0` read from the
  stage record (`src/game/stage.rs:951`, `:46216`) as the quad's extents;
* it uses the camera/scene object `0x8042A1B8` field `+0x18C`
  (`src/game/bases.rs:84470`, `:161008`), the same object
  `fn_807a8440` reads at `+0x184`/`+0x188`;
* it drains the same eight-word screen rectangle (`0x803504E0`,
  `0x803504E2`, `0x803504EC`, `0x803504EE`) that `0x807BEF60` uses;
* it hits the `+0x15C`/`+0x26C` transform blocks and the `+0x180`/`+0x1E0`
  layout owners named in `fn_807a8920`'s note
  (`ledger/functions.csv` row `807a8920`);
* the only gate on the whole body is `0x800B54B0`, the DVD-settings poll
  (`src/game/stage.rs`), which is the same gate `0x807BEF60` uses.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x70` .. `+0x8F` | `f32` x8 | scratch quad written by `0x802C6AE0` (frame-local) |
| `+0x78`, `+0x7C` | `f32` | horizontal / vertical texel count of the backdrop |
| `+0x98`, `+0x9C`, `+0xA0` | `f32` | per-texel UV step on x, on y, and `1.0` |
| `+0xA4` | `u16` | dirty flags; always `|= 1` here |
| `+0xB4` | `f32` | vertical snap of the top edge (only written when settings poll is 0) |
| `+0xF8` | object | sub-object whose `vtable[5]` is run first |
| `+0x15C .. +0x17C` | `f32` x9 | copy of `+0x26C .. +0x28C` (the screen transform block) |
| `+0x180` | rect | four-float rectangle set by `0x802BF420` |
| `+0x1E0` | object | second sub-object whose `vtable[5]` is run last |
| `+0x244 .. +0x254` | `f32` x5 | five slots zeroed on every rebuild |
| `+0x258` | `f32` | `-100.0` on every rebuild |
| `+0x25C .. +0x264` | `f32` x3 | copy of `+0x284 .. +0x28C` |
| `+0x26C .. +0x28C` | `f32` x9 | source transform block (filled by `fn_807a8440`) |

### Exact behaviour

Let `S = *(u8 *)(0x800B54B0())` (the DVD settings item-1 byte; `0` = "wide").
Let `K1 = *(f32 *)0x8042A0F0`, `K2 = *(f32 *)0x8042A0EC` (the two BG colours),
and `d = *(f32 *)(*(u32 *)0x8042A1B8 + 0x18C)`.

1. Prologue: `stwu r1,-0x110(r1)`; save `vrsave`, four `stfd`+`psq_st` pair
   spills (`f31..f28`, `qr0`), `r31`, `r30`; `r30 = this`;
   `r31 = 0x807D0670` (literal pool); the two `0x43300000` magic halves are
   pre-stored at `r1+0x70` and `r1+0x78`.
2. `w = d * (0.5 * K1)` and `h = d * (0.5 * K2)`; `-w` and `-h` are materialised
   as `f30`/`f29`. These are the four half-extents handed to the quad builder.
3. Copy the nine words `+0x26C .. +0x28C` to `+0x15C .. +0x17C`.
4. `0x802C6AE0(w, -w, -h, h, 1.0, 20000.0, this + 0x70)` - the four extents,
   the `1.0` UV scale and the `20000.0` depth, writing the eight-float quad at
   `+0x70`. (Callee unclaimed; its return is unused and it only writes `r3`.)
5. Read the screen rectangle from `main.dol` BSS:
   `A = u16[0x803504E0]`, `B = u16[0x803504E2]`, `C = u16[0x803504EC]`,
   `D = u16[0x803504EE]`, each converted with the `0x43300000` unsigned ladder.
   `E = u16[*(u32 *)(*(u32 *)0x8042A480) + 4]` (the frame counter's `+4` half,
   `src/game/actor.rs:572`) is read later, inside the `S == 0` arm only.
6. **`S != 0` ("narrow" arm).** `S58 = C`, `S5C = D`,
   `S60 = |2h| / C`, `S64 = |2w| / D`, `S68 = 1.0`. Nothing else is computed;
   `+0xB4` is left alone.
7. **`S == 0` ("wide" arm).** With `r = E / C`:
   `n = (int)(D * r)` (truncating `fctiwz`),
   `m = int(0.5 * (B - B * r))`,
   `S58 = 4 * ceil(A / 4)` and `S5C = 4 * ceil((n & !1) / 4)`, both computed as
   `value + k[value & 3]` with the literal table `k = {0, 3, 2, 1}` at
   `0x807D0690` (so `A=1..3 -> 4`, `A=5..7 -> 8`, `n=6 -> 8`, `n=5 -> 4`);
   `S60 = |2h| / S58`, `S64 = |2w| / S5C`, `S68 = 1.0`, and
   `this->+0xB4 = (f32)m + (f32)(m & 1)` (snap the top edge to even).
8. **Common tail.**
   `+0x78 = S58`, `+0x7C = S5C`, `+0x98 = S60`, `+0x9C = S64`, `+0xA0 = S68`;
   `+0xA4 |= 1`; `+0x244..+0x254 = 0.0` (five words); `+0x258 = -100.0`;
   `+0x25C..+0x264 = +0x284..+0x28C`.
9. `(*(void **)this->+0x1E0)[5](this + 0x1E0)` - the second owner's update slot
   (`vtable + 0x14`), no arguments.
10. `0x802BF420(K1, 0.0, 0.0, K1, this + 0x180)` - the four-float rectangle at
    `+0x180` (evidence: the export of `0x802BF420` stores at `+0x50/+0x54/+0x58/+0x5C`,
    i.e. "set rect from two points").
11. Epilogue: `psq_l`/`lfd` restore `f31..f28`, `r31`, `r30`, `vrsave`, `blr`.

Note the step-4 virtual call happens **before** the extents are published
(`+0x78` is only written in step 8), so the `+0xF8` owner sees the previous
frame's counts - that ordering is load-bearing.

**Signature.**

```rust
/// Rebuilds the `otasukeInfo` backdrop quad: samples the two BG colours and the
/// camera depth, snaps the texel counts to a multiple of four in "wide" mode,
/// republishes the transform block and re-runs both layout owners.
/// # Original
/// `FUN_807a84b0`, `0x807A84B0`, 1,124 bytes,
/// `work/ghidra/rel-d_basesNP/export/807a84b0_FUN_807a84b0.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - the help-screen object; `r4` is dead on entry (the body reloads it
///   with `lis 0x8043` at `0x807A84B8`), so the export's single-argument shape
///   is correct.
/// # Returns
/// Nothing; the export's `void` is accurate (nothing reads `r3` on return).
/// # Confidence
/// medium - the arithmetic and both arms are literal; `0x802C6AE0` is unclaimed
/// and is described by its six arguments only.
pub unsafe fn fn_807a84b0(this: usize)
```

**Constants.** pool `0x807D0670` = `0.0`; `0x807D0678` = `1.0`;
`0x807D067C` = `20000.0`; `0x807D0680` = `10000.0`; `0x807D0688` = `2^52`
(`0x43300000_00000000`); `0x807D0690` = `{0, 3, 2, 1}` (u32, the
round-up-to-four table); `0x807D06A0` = `0.5`; `0x807D06A4` = `100.0`;
`0x807D06A8` = `2^52 + 2^31` (`0x43300000_80000000`, the signed-int ladder).
Globals: `0x8042A0EC`, `0x8042A0F0` (BG colours), `0x8042A1B8` (camera,
`+0x18C`), `0x8042A480` (frame counter, double-deref, `+4`).
Callees: `0x800B54B0`, `0x802C6AE0`, `0x802BF420`.

**Verdict: ready (medium).** Every instruction is decoded; the eight
`psq_` words are `qr0` spills; the only unclaimed callee takes six arguments and
returns nothing.

---

## 2. `0x80772370` - the 0x16518-stride area scan

**Evidence.** `work/ghidra/rel-d_basesNP/export/80772370_FUN_80772370.c`,
1,108 bytes, 277 words, synthetic `0x80772370`.

**What it is.** A three-pass scan over the same 9,416-byte (0x16518) "area"
records that `0x80774BD0` / `0x80774D00` (both untranslated, called four times
here) also walk. Every comparison in the body is a 4-byte `memcmp` against two
opaque key words, so this is the *"does this line/segment cross the area"*
predicate behind those two helpers. It is a pure query: `r4` is dead on entry,
and the single return is a `0`/`1` sentinel stored into `r28`'s caller.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x338C` | `i32` | area count `n` |
| `+0x3390` | `i32` | outer loop count (rows of the area block) |
| `+0x3394 + i*0x16518` | block | area record `i`; `+0x14` of the block (`0x33A8` at `i=0`) holds its element count |
| `+0x5C7F4 + i*0x3F08` | block | parallel 16,136-byte record `i` |
| `this + 0x3394 + n*0x16518` | sentinel | "one past the last area", built by `0x800F56B0` |
| `this + 0x5C7F4 + n*0x3F08` | sentinel | "one past the last parallel record", passed to `0x800F3270` |

### Exact behaviour

1. `this = 0x802DCD54()` (the `_savegpr` return, `r27`); `r19 = r5` (the third
   argument); `r4` is never read.
2. `end = 0x800F56B0(this + n*0x16518 + 0x3394)` with `n = *(u32 *)(this+0x338C)`.
3. `hit = 0x800F3270(this + 0x5C7F4 + n*0x3F08, end, arg3)`; if `hit == 0`
   return `0`. Otherwise `key_b = hit + 4` (the second key word), so
   `0x800F3270` returns a 4-byte-word-aligned pointer with a payload at `+4`.
4. **Pass 1** - for `i` in `0..*(this+0x3390)`, `block = this+0x3394+i*0x16518`,
   for `j` in `0..*(block+0x14)`:
   ```
   p = 0x800F57F0(block, j);
   if ((memcmp(p+1, end, 4) == 0 && memcmp(p+5, key_b, 4) == 0) ||
       (memcmp(p+5, end, 4) == 0 && memcmp(p+1, key_b, 4) == 0))
       if (0x80774BD0(this, p) != 0 && 0x80774D00(this, end, key_b) != 0)
           result = 1;
   ```
5. **Pass 2** - same loops, `c = 0x800F5930(block, j)`; when `c > 1`:
   ```
   base = 0x800F57F0(block, j); flags = 0;
   for (k = 0; k < 0x800F5930(block, j); k++) {
       e = 0x800F5810(block, j, k);
       if (memcmp(e, end, 4) == 0)   flags |= 1;
       if (memcmp(e, key_b, 4) == 0) flags |= 2;
   }
   if (flags == 3 && 0x80774BD0(this, base) != 0 &&
       0x80774D00(this, end, key_b) != 0) result = 1;
   ```
6. **Pass 3** - same loops, `c = 0x800F5930(block, j)`; when `c != 0`, two
   sub-passes which are entered on *opposite* guards:
   ```
   p = 0x800F57F0(block, j);
   // 3a: the "end" word sits at p+1 or p+5 -> look for key_b inside the list
   if (memcmp(p+1, end, 4) == 0 || memcmp(p+5, end, 4) == 0)
       for (k = 0; k < c; k++) {
           e = 0x800F5810(block, j, k);
           if (memcmp(e, key_b, 4) == 0 && 0x80774BD0(this, p) != 0 &&
               0x80774D00(this, end, key_b) != 0) result = 1;
       }
   // 3b: reached only when p+1 and p+5 are not the "end" pair
   else if (memcmp(p+1, key_b, 4) == 0 || memcmp(p+5, key_b, 4) == 0)
       for (k = 0; k < c; k++) {
           e = 0x800F5810(block, j, k);
           if (memcmp(e, end, 4) == 0 && 0x80774BD0(this, p) != 0 &&
               0x80774D00(this, end, key_b) != 0) result = 1;
       }
   ```
   (The `else` is real: the 3a guard falls through to the shared next-`j` at
   `0x807726D4` only after trying `p+5`; 3b is entered at `0x807726D4` from the
   "`p+5 != end`" exit of 3a and skipped entirely when `p+5` matches `end`.)
7. `0x802DCDA0(result); return;` - the epilogue is the shared `_restgpr`, so the
   value propagates in `r3`.

The loops are `blt`/`b` with the block pointer advanced by `+0x16518` in the
outer loop and the count reloaded from `block+0x14` each iteration, so an
implementer must *not* hoist the count out of the loop (the accessors may
shorten it).

**Signature.**

```rust
/// Three-pass crossing test over the 0x16518-stride area records, keyed by two
/// opaque 4-byte words.
/// # Original
/// `FUN_80772370`, `0x80772370`, 1,108 bytes,
/// `work/ghidra/rel-d_basesNP/export/80772370_FUN_80772370.c`.
/// # Arguments
/// * `this` - the area owner (`+0x338C` count, `+0x3390` rows, `+0x3394` blocks).
/// * `_unused` - the export's `param_2`; `r4` is dead on entry.
/// * `key_a` - first key word, forwarded to `0x800F3270` and `0x80774D00`.
/// # Returns
/// `1` when any record crosses, `0` otherwise.
/// # Confidence
/// medium - every branch and both keys are literal; the accessors
/// `0x800F56B0` / `0x800F3270` / `0x800F57F0` / `0x800F5810` / `0x800F5930` are
/// unclaimed and are named by address and argument shape only.
pub unsafe fn fn_80772370(this: usize, _unused: u32, key_a: u32) -> u32
```

**Constants.** strides `0x16518` (91,416) and `0x3F08` (16,136); block bases
`0x3394`, `0x5C7F4`, count offsets `+0x338C`, `+0x3390`, `+0x33A8`; the
`0x802DCD54`/`0x802DCDA0` save/restore pair; `memcmp` = `0x802E1BD0`.

**Verdict: ready (medium).** The five accessors are called with fixed argument
shapes and their returns only feed comparisons and further fixed calls, so the
body is writable today; the *meaning* of the two keys is whatever
`0x80774BD0`/`0x80774D00` consume.

---

## 3. `0x806639A0` - the player-contact state machine

**Evidence.** `work/ghidra/rel-d_basesNP/export/806639a0_FUN_806639a0.c`,
1,096 bytes, 274 words, synthetic `0x806639A0`.

**What it is.** A five-state machine (`+0x360`) that decides whether every
player is standing inside a trigger and then clamps one of the trigger's Y
values. The cast is all `game::player` plus two `game::bases` helpers already in
`src/game/bases.rs`:

* `0x80663180` - "bit slot of global mask `0x80429CA8`" (translated), called as
  `0x80663180(this, player_actor)`: the per-player "is this player registered to
  me" test;
* `0x80663290` - "returns `0x8042A180` when `0x80315B98` is zero, else `0`"
  (translated): a mode/region predicate used as the branch on every arm;
* `0x80429C94` - the player-state pointer, `0x80429CA0` - the player count
  (`src/game/bases.rs:82642`, `:47677`).

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x3D0` | `i32` | shared countdown, decremented at entry when positive |
| `+0x3EC + 4*i` | `f32` | per-player Y target, filled by state 1 |
| `+0x3D8` | `u32` | object id looked up through `0x80162D00` |
| `+0x360` | `i32` | state: `0..4` |
| `+0x394` | object | interface queried in states 3 and 4 |
| `+0x43C` | object | dsys widget whose `+0x5C` is written by `0x800E80C0` |
| `+0x460` | `f32` | published widget value |
| `+0x470` | `u32` | player actor pointer used by states 0-2 |
| `+0x47C` | `f32` | base Y |
| `+0x480` | `f32` | clamped Y |

### Exact behaviour

Entry: `this = 0x802DCD6C()`; `r28 = *(u32 *)0x80429C94`;
if `*(i32 *)(this+0x3D0) > 0` then `this+0x3D0 -= 1`.

`switch (*(i32 *)(this+0x360))`:

* **0** - `if (0x8004DB40(*(u32 *)(this+0x470), 3) != 0) return;`
  `this+0x3D0 = 80; this+0x360 = 1; return;`
  (`0x8004DB40` is the translated six-mode player query; the `3` is the mode
  being tested, and the arm waits until it is clear.)
* **1** - `ok = 1;` for `i` in `0..*(u32 *)0x80429CA0`:
  ```
  actor = 0x800601E0(i);
  state = 0x8005F900(actor);          // player-state object, +0x10D4 flags
  if (0x80663180(this, 0x800601E0(i)) == 0) continue;
  if ((*(u32 *)(state+0x10D4) & 1) == 0) { ok = 0; continue; }
  if (0x80663290() != 0 || *(i32 *)(this+0x3D0) == 0)
      0x80052080(state, this + 0x3EC + 4*i, 0x807C8B88);  // 3rd arg = &2.4
  else ok = 0;
  ```
  Then, unconditionally:
  ```
  if (0x80663290() == 0) {
      p = *(u32 *)(this+0x470);
      if (*(i32 *)(this+0x3D0) == 0) 0x8005E930(p+0xEA4, 1);
      else                           0x8005E910(p+0xEA4, 1);
  }
  y = *(f32 *)(*(u32 *)(this+0x470) + 0xB0);
  if (0x80663290() != 0) { if (-708.0 < y && y < -660.0) snap(-660.0); }
  else                   { if (-740.0 < y && y < -628.0) snap(-628.0); }
  ```
  with `snap(lo) = { this->+0x480 = this->+0x47C - (lo - y);
  0x800E80C0(this + 0x43C); this->+0x460 = this->+0x480; }`.
  Finally `if (ok) { this+0x3D0 = 20; this+0x360 = 2; }`.
* **2** - `all = 1;` for `i` in `0..count`:
  ```
  actor = 0x800601E0(i); state = 0x8005F900(actor);
  if (0x80663180(this, actor) == 0) continue;
  if (0x80052120(state) != 0) all = 0;
  if (0x80052030(state) != 0 && (*(u32 *)(state+0x10D4) & 1) != 0) {
      0x80051F30(state, 0);
      0x80052170(state, 0x93);        // 147
  }
  ```
  `if (!all) return; if (0x8005CDD0(*(u32 *)0x80429C94) == 0) return;`
  `if (0x80663290() != 0) 0x80C327E0(0x80162D00(*(u32 *)(this+0x3D8)));`
  `this+0x360 = 3;`
* **3** - always: `obj = 0x80162D00(*(u32 *)(this+0x3D8))`;
  `v = (*(void ***)obj)[0x3F0/4](obj)` (slot at `vtable + 0x3F0`);
  `x = (*(void ***)(v + 0x394))[10](v + 0x394)`;
  `if ((*(u32 (***)(u32*, u32))x)[5](x, 0x8082B5AC) != 0) {
     0x8005CA50(*(u32 *)0x80429C94, 1); this+0x360 = 4; }`
  Then a second, **mutually exclusive** name query on the same two virtual
  hops:
  ```
  if (0x80663290() != 0) {
      x2 = (*(void ***)(v + 0x394))[10](v + 0x394);
      if ((*(u32 (***)(u32*, u32))x2)[5](x2, 0x80C6F6EC) != 0)
          0x80663DF0(this, 0.022);
  } else {
      x2 = (*(void ***)(v + 0x394))[10](v + 0x394);
      if ((*(u32 (***)(u32*, u32))x2)[5](x2, 0x80C6F6B0) != 0
          && *(i32 *)(this + 0x360) > 4)
          0x80663DF0(this, 0.022);
  }
  ```
  so `0x80C6F6EC` is the "region set" name and `0x80C6F6B0` the "region clear"
  name, and only the latter additionally requires the state to be past `4`.
* **4** - `*(u8 *)0x8042A180 = 1;`
  `(*(void ***)(this + 0x394))[6](this + 0x394, 0x803526F0)` - `vtable + 0x18`,
  with the REL data word `0x803526F0` as its only argument.
* **default** - nothing.

### Field map for the trigger's own state

`+0x360` is written only at `0x80663a24` (`+1`), `0x80663bb8` (`+1`),
`0x80663c8c` (`+1`) and read by the `switch`; it is never reset here, so the
caller owns the reset. `+0x3D0` is the only countdown and takes the three
values `80`, `20` and `-1`-per-call.

**Signature.**

```rust
/// Five-state player-contact machine: arms an 80-frame window, drives every
/// registered player's Y target while the region predicate is set, parks them
/// with `0x93`, then publishes the region flag and plays the name-gated cue.
/// # Original
/// `FUN_806639a0`, `0x806639A0`, 1,096 bytes,
/// `work/ghidra/rel-d_basesNP/export/806639a0_FUN_806639a0.c`.
/// # Arguments
/// * `this` - the trigger object; `+0x360` state, `+0x3D0` countdown,
///   `+0x470` player actor, `+0x3EC` per-player Y targets.
/// # Returns
/// Nothing.
/// # Confidence
/// medium - every branch, store, call and constant is literal; the two name
/// constants are read from `main.dol`/REL data by address only.
pub unsafe fn fn_806639a0(this: usize)
```

**Constants.** `0x807C8B80` pool (-10.5, 6.0, **2.4** at `+8`, 1.2, 1.0,
s16 pair (20, 80) at `+0x14`); clamped Y windows `-660` / `-708` and
`-628` / `-740` (`0x807C8B9C`, `0x807C8BA0`, `0x807C8BA8`, `0x807C8BAC`);
`0.022` at `0x807C8B98` / `0x807C8BA4`; timers `80` and `20`; player mode
argument `0x93` (147); name constants `0x8082B5AC` (state advance) and
`0x80C6F6EC` / `0x80C6F6B0` (the region-set / region-clear names); REL data
word `0x803526F0` (handed to the `+0x394` `vtable[6]` slot in state 4).

**Verdict: ready (medium).**

---

## 4. `0x807BEF60` - the 0x807BE screen rebuild

**Evidence.** `work/ghidra/rel-d_basesNP/export/807bef60_FUN_807bef60.c`,
1,096 bytes, 274 words, synthetic `0x807BEF60`.

**What it is.** A screen rebuild in the entirely untranslated `0x807BExxx`
cluster (no row of that range is in `ledger/functions.csv`). It is the closest
sibling of `0x807A84B0`: same `0x803504E0` screen rectangle, same
`0x800B54B0` settings gate, the same handle idiom as `fn_807a8920`
(`0x80164B20` -> `0x802BE880` bind into a layout owner), the same
`0x802BF420` rectangle setter and the same "counts, UV steps, `1.0`" output
triple at `+0x78`-style offsets. It owns **two** layout groups, one at `+0x184`
with its animation object at `+0x20C`, one at `+0x294` with `+0x31C`, plus a
third rect at `+0x3A4` and a fourth owner at `+0x48C`.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x184` | object | layout owner A; `vtable[5]` then `0x802BE880` bind |
| `+0x1E8 .. +0x208` | `f32` x9 | the `+0x4C8 .. +0x4E8` block **rotated by three**: `[+0x4D4..+0x4E8, +0x4C8..+0x4D0]` |
| `+0x20C` | object | animation owner A (`0x802D1210`, `0x802C6C70`) |
| `+0x214`, `+0x218` | `f32` | A's published x/y counts in the `S != 0` arm |
| `+0x240` | `u16` | A's dirty flags; `|= 1` |
| `+0x244` | object | A's clip list, `vtable[4]` |
| `+0x294` | object | layout owner B |
| `+0x2F8 .. +0x30C` | `f32` x6 | straight copy of `+0x628 .. +0x63C` |
| `+0x310 .. +0x318` | `f32` x3 | copy of `+0x4C8`, `+0x4CC`, `+0x4D0` |
| `+0x31C` | object | animation owner B |
| `+0x324`, `+0x328` | `f32` | B's published x/y counts |
| `+0x350` | `u16` | B's dirty flags; `|= 1` |
| `+0x354` | object | B's clip list, `vtable[4]` |
| `+0x3A4` | rect | rectangle `[E, 1.0, 1.0, F]`; owner slot `vtable[10]` |
| `+0x404` | object | third bound owner (`vtable[5]` + bind) |
| `+0x468 .. +0x488` | `f32` x9 | colour/scale block (`1.0`, `1.0`, 6000, `1.0`x3, custom, `1.0`) |
| `+0x48C` | rect | `0x802BFFF0` rectangle (F, E) |
| `+0x4A8` | `u32` | registry id resolved through `0x80164B20` |
| `+0x4C8 .. +0x4E8` | `f32` x9 | source transform block A |
| `+0x628 .. +0x63C` | `f32` x6 | source transform block B |

### Exact behaviour

1. Prologue: 112-byte frame; `f31`/`f30` `stfd`+`psq_st` `qr0` pair spills;
   `r31 = 0x807D0A68`; `r30 = this`; `r29` saved.
2. Copy `+0x4C8..+0x4E8` (9 f32) to `+0x1E8..+0x208` **rotated by three**:
   `+0x1E8..+0x1FC = +0x4D4..+0x4E8`, `+0x200..+0x208 = +0x4C8..+0x4D0`;
   copy `+0x628..+0x63C` (6 f32) straight to `+0x2F8..+0x30C`; copy
   `+0x4C8..+0x4D0` again to `+0x310..+0x318`.
3. `h0 = 0x80164B20(*(u32 *)(this+0x4A8))`;
   `(*(void ***)(this+0x184))[5](this + 0x184)`; `0x802BE880(this + 0x184, &h0)`.
4. Screen rectangle `A,B,C,D` exactly as in `0x807A84B0` (same four `u16`s,
   same unsigned ladder). `S = *(u8 *)0x800B54B0()`.
   `E = u16[*(u32 *)(*(u32 *)0x8042A480) + 6]` (the `+6` half, where
   `0x807A84B0` uses `+4`).
5. **`S == 0`.** With `r = D / C`,
   `n = (int)(A * r) >> 2` (arithmetic shift + `addze`, i.e. truncated divide by
   four) and `n4 = (f32)n * 4.0`:
   `0x802D1210(this + 0x20C, 1, 0.0, f = (E - n4) * 0.5, A, n4)` and the
   identical call on `this + 0x31C`. (Five arguments: object, `1`, `0.0`,
   two computed floats, and `A`.)
6. **`S != 0`.** `this+0x240 |= 1`, `this+0x350 |= 1`,
   `+0x214 = C`, `+0x218 = D`, `+0x324 = C`, `+0x328 = D`. Nothing else.
7. Common: `0x802C6C70(this + 0x20C, 16.0)`;
   `(*(void ***)(this+0x244))[4](this + 0x20C, &h0)` - note the receiver is
   `+0x244` but the argument is the `+0x20C` animation object;
   `h3 = 0x80164B20(3)`; `(*(void ***)(this+0x294))[5](this + 0x294)`;
   `0x802BE880(this + 0x294, &h3)`;
   `0x802C6C70(this + 0x31C, 16.0)`;
   `(*(void ***)(this+0x354))[4](this + 0x31C, &h3)`.
8. `h1 = 0x80164B20(1)`; `f31 = 0.0` (pool `+4`), `f30 = (f32)E`;
   `f4 = (S == 0) ? (f32)F : (f32)(int)(F * *(f32 *)0x80428750)` where
   `F = u16[*(u32 *)(*(u32 *)0x8042A480) + 4]` and the conversion goes through
   `0x802DCC4C` (saturated double->u32, translated);
   `0x802BF420(E, f31, f31, f4, this + 0x3A4)`.
9. `0x802BFFF0(this + 0x48C, 0.0, 0.0, (f32)F, (f32)E)`.
10. Publish the colour block: `+0x468 = 0.0`, `+0x46C = 0.0`,
    `+0x470 = 6000.0`, `+0x474..+0x480 = 0.0` (four words),
    `+0x484 = 1.0`, `+0x488 = 0.0`; `0x802C0150(this + 0x48C, &h1)`;
    `(*(void ***)(this+0x3A4))[10](this + 0x3A4, &h1)`;
    `(*(void ***)(this+0x404))[5](this + 0x404)`;
    `0x802BE880(this + 0x404, &h1)`.
11. Epilogue restores `f31`/`f30`, `r31`, `r30`, `r29`, `vrsave`.

### Why it is conditional

Everything above is decoded, but the body has **four** layout/animation owners
and **three** rectangle setters whose role cannot be assigned from the shipped
bytes: the calls are literal, yet a faithful port must know which pane is the
"frame", which is the "fill", and what `0x802D1210`'s second argument (`1`) and
its five-argument tail mean (it is unclaimed; the sibling `0x802BF420` takes
five too but is documented as a four-float rect setter
`src/resource/archive.rs`). Without that mapping the four argument groups can be
transcribed but not verified, so the verdict is conditional rather than ready.

**Signature.**

```rust
/// Rebuilds the 0x807BE screen: binds two layout owners plus a third clip owner,
/// republishes both transform blocks, emits the two rectangles and the colour
/// block, and drives the two animation owners.
/// # Original
/// `FUN_807bef60`, `0x807BEF60`, 1,096 bytes,
/// `work/ghidra/rel-d_basesNP/export/807bef60_FUN_807bef60.c`.
/// # Arguments
/// * `this` - the screen object (see the field map).
/// # Returns
/// Nothing.
/// # Confidence
/// medium - all arithmetic, every offset and every constant is literal; the
/// roles of `+0x184`/`+0x294`/`+0x3A4`/`+0x404` and the meaning of
/// `0x802D1210`'s argument tail are the open input.
pub unsafe fn fn_807bef60(this: usize)
```

**Constants.** pool `0x807D0A68`: `+0x00` 2750.0, `+0x04` 0.0, `+0x08` 1.0
(double), `+0x10` -1.0, `+0x18` 2.0, `+0x20` 4.0, `+0x28` 0.0, `+0x30` -0.245,
`+0x38` +0.245, `+0x40` 0.245, `+0x48` 1e-6, `+0x50` 0.5 (double),
`+0x58` 0.5, `+0x5C` 2.0, `+0x60` 1.0, `+0x64` 10.0, `+0x68` 2400.0,
`+0x6C` -100000.0, `+0x70` 100000.0, `+0x74` 20000.0, `+0x78` 10000.0,
`+0x80` `2^52`, `+0x88` 0.1, `+0x8C` 4.0, `+0x90` 6000.0,
`+0x98` `2^52+2^31`, `+0xA0` 0.7111111, `+0xA4` 5.0.
`0x80821540` (`+0x08` = 16.0) - the animation rate handed to `0x802C6C70`.
Globals: `0x8042A480` (`+4`, `+6`), `0x80428750` (`f32` scale on `F`),
`0x803504E0/E2/EC/EE`. Callees: `0x80164B20`, `0x802BE880`, `0x802C6C70`,
`0x802D1210`, `0x802BFFF0`, `0x802C0150`, `0x802DCC4C`, `0x802BF420`,
`0x800B54B0`.

---

## 5. `0x8074C540` - the four-slot join panel

**Evidence.** `work/ghidra/rel-d_basesNP/export/8074c540_FUN_8074c540.c`,
1,092 bytes, 273 words, synthetic `0x8074C540`.

**What it is.** The per-frame body of the "player join" panel: it reads two
four-bit "occupied" masks, resolves a slot index for each of them, draws the
slot's portrait through `0x80064610`, and pushes the chosen slots into three
9-/4-bit masks. Every value it needs comes from three sources:

* the **LCG** `0x800B2EE0`: the ledger describes it as a "scale record setter",
  but its callee `0x80161460` advances a linear congruential record
  (`record = record*A + C`) and **returns** `high32(record * arg)`, so
  `(u8)0x800B2EE0(n)` is a pseudo-random byte seeded with `n`. It is used four
  times below, exactly as a random slot picker. (`0x800B2EE0` is a 12-byte
  tail-call thunk, so `r3` survives - the `void` in the main.dol export is a
  decompiler artifact, not an ABI fact.)
* the camera/scene object `0x80429E00` (`+0x38`, `+0x3C`, `+0x40`, `+0x44`),
  the same pointer `src/game/bases.rs:4532`, `:31155` uses;
* the slot record passed in `arg2` (`+0x348` valid byte, `+0x10C` `f32`).

### Field map

`this`:

| offset | type | meaning |
|---:|---|---|
| `+0x420` | `u16` | panel state `0..2`; decremented at the end when non-zero |
| `+0x422` | `u16` | 9-bit "confirmed" mask |
| `+0x424` | `u16` | two-phase toggle (`0`/`1`) |
| `+0x426` | `u16` | 4-bit mask A (slots already drawn as A) |
| `+0x428` | `u16` | 4-bit mask B (slots already drawn as B) |
| `+0x42C` | `u16` | frame counter |
| `+0x431` | `u8` | mode byte; `6` = the special "no panel" mode |

`arg2` (the player record): `+0x348` `u8` "assigned", `+0x10C` `f32` timer.

### Exact behaviour

Frame-local `pos = [0.0, 0.0, 1500.0]` at `r1+0x08`.

1. `this = 0x802DCD70(); r28 = arg2; r31 = 0x807CE620`.
2. **Slot A.** `i = (u8)0x800B2EE0(4); mask = *(u16 *)(this+0x426);`
   `if (mask & (1 << i)) {` then `i -= 1; if (i < 0) i = 3;` and the same
   `test/decrement/wrap` is unrolled five times, the fifth test being followed by
   one final unconditional `i -= 1; if (i < 0) i = 3;`. Each test leaves the walk
   as soon as the bit at `i` is **clear**, so with a 4-bit mask it settles on the
   lowest set bit of the mask, except when all four bits are set, where the extra
   final decrement wraps once more. `}`
3. **Slot B.** `n = (int)(*(f32 *)(*(u32 *)0x80429E00 + 0x44) * 0.0625);`
   `j = (u8)0x800B2EE0(n); mask2 = *(u16 *)(this+0x428);`
   `if (mask2 & (1 << j)) { j -= 1; if (j < 0) j = (u8)(n - 1);
    for (k = 0; k < n - 1; k++) { if ((mask2 & (1 << j)) == 0) break; j -= 1;
    if (j < 0) j = (u8)(n - 1); } }` (a `bdnz` loop with the counter preloaded
   from `n-1`; when `n-1 <= 0` the walk is skipped. It stops at the first clear
   bit, like slot A.)
4. `draw = 0;`
   `if (*(u8 *)(this+0x431) == 6) {`
   `    if (!(*(f32 *)(arg2+0x10C) < -0.5)) draw = 1;   // unordered counts as "no"`
   `    use_base = 1; }`
   `else if (*(u8 *)(arg2+0x348) != 0) {`
   `    pos.x = (*(f32 *)(*(u32 *)0x80429E00 + 0x38) - 16.0) - 16.0 * (f32)i;`
   `    use_base = 0; }`
   `else { use_base = 1; }`
5. `if (use_base) pos.x = 8.0 + 16.0 * (f32)i +
   (*(f32 *)(*(u32 *)0x80429E00 + 0x38) + *(f32 *)(*(u32 *)0x80429E00 + 0x40));`
   `sel = (u8)0x800B2EE0(9);`
   `if (draw != 0) return;`
6. `mode = match *(u16 *)(this+0x420) { 1 => 0, 2 => (u8)0x800B2EE0(2), _ => 1 };`
7. With `sel` already holding `(u8)0x800B2EE0(9)` from step 5: `sel -= 2; if
   (sel < 0) sel = 8;` then up to **nine** times
   `{ if ((*(u16 *)(this+0x422) & (1 << sel)) == 0) break; sel -= 1;
      if (sel < 0) sel = 8; }`
   (the block is unrolled three test/decrement pairs per `bdnz` iteration with a
   counter of `3`, i.e. a full nine-slot sweep). It finds the next **clear** bit
   of the nine-bit confirmed mask, starting two below the random index and
   wrapping `-1` to `8`. The loop also accumulates `2` per iteration into `r6`,
   but `r6` is overwritten at `0x8074C880` before any read, so that accumulator
   is dead in the shipped body; the walk's only live result is `sel`.
8. `*(u16 *)(this+0x426) |= 1 << (u8)i;`
   `*(u16 *)(this+0x428) |= 1 << (u8)j;`
   `*(u16 *)(this+0x422) |= 1 << (u8)sel;`
9. `pos.y = *(f32 *)(*(u32 *)0x80429E00 + 0x3C) - 16.0 * (f32)j - 32.0;`
   `arg4 = (mode << 12) | (use_base & 1) | (*(u8 *)(this+0x431) & 0xFFFFFFFE);`
   `0x80064610(159, arg4, &pos, 0, 0);`
   (`0x80064610` = `0x80064600(r7)` then `0x8006CA80(r3, r4, r5, r6)`, i.e. the
   sprite/portrait draw with id `159`.)
10. `if (*(u16 *)(this+0x420) != 0) *(u16 *)(this+0x420) -= 1;`
    `if (*(u16 *)(this+0x420) == 0) {`
    `    v = (*(u16 *)(this+0x424) + 1) & 1; *(u16 *)(this+0x42C) += 1;`
    `    *(u16 *)(this+0x424) = v; if (v == 0) *(u16 *)(this+0x422) = 0; }`

**Signature.**

```rust
/// Draws one "join" portrait per frame: picks a random slot from each 4-bit
/// occupancy mask, walks to the nearest occupied slot, draws id 159 through
/// `0x80064610`, then commits the slot into the 9-bit confirmed mask.
/// # Original
/// `FUN_8074c540`, `0x8074C540`, 1,092 bytes,
/// `work/ghidra/rel-d_basesNP/export/8074c540_FUN_8074c540.c`.
/// # Arguments
/// * `this` - the panel; `+0x420` state, `+0x422`/`+0x426`/`+0x428` masks,
///   `+0x431` mode.
/// * `player` - the player record; `+0x348` assigned byte, `+0x10C` timer.
/// # Returns
/// `0` when the frame was suppressed (state `1` after a successful draw), else
/// nothing of interest; the export's `void` is wrong - `0` is returned on the
/// `0x8074C7A8` path.
/// # Confidence
/// medium - the mask walks, the LCG indices and both position formulas are
/// literal; `0x80064610` is unclaimed and is named by its five arguments.
pub unsafe fn fn_8074c540(this: usize, player: usize) -> u32
```

**Constants.** pool `0x807CE620`: `+0x04` 0.0, `+0x08` 1500.0, `+0x0C` -0.5,
`+0x10` 8.0, `+0x14` 16.0, `+0x18` 32.0, `+0x28` 0.0625, `+0x30` `2^52`.
Literals `4`, `9`, `2`, `159`, `0x93`-free. Globals `0x80429E00`
(`+0x38`, `+0x3C`, `+0x40`, `+0x44`). Callees `0x800B2EE0` (LCG),
`0x80064610` (draw), `0x802DCD70`/`0x802DCDBC` (save/restore).

**Verdict: ready (medium).** One caveat is recorded rather than blocking: the
`(u8)0x800B2EE0(...)` indices rely on the LCG return surviving a `void`
prototype, which the 12-byte tail-call thunk guarantees.

---

## 6. `0x8075D5C0` - the 10-pane layout transform pass

**Evidence.** `work/ghidra/rel_d_basesNP/export/8075d5c0_FUN_8075d5c0.c`,
1,080 bytes, 270 words, synthetic `0x8075D5C0`.

**What it is.** A layout pass: three hinge objects at `+0x314` (stride `0x2C`)
and ten pane objects at `+0x408` (stride `0x7C`) are updated, then four shared
`3x4` matrices are rebuilt from the camera/actor position and the panes are
re-bound with `0x8016A170`/`0x8016A150`/`0x8016A1A0`. It finishes by handing
the whole object to `0x800F6910(this, 0x80814798, -40.0, 0.8)`.
`0x8016EF50` is the translated `ui::lyt` entry (`src/ui/lyt.rs`), and
`0x8016A170/0x8016A150/0x8016A1A0/0x80168400/0x80168180/0x80169020` are the
`ui::lyt` setters already used by other translated layout screens.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x7C` | matrix | shared `3x4` transform fed to `0x801C0BD0` / `0x8016EF50` |
| `+0xDC`, `+0xE0`, `+0xE4` | `f32` | actor position (the `+0xAC` triple shifted by `+0x30`) |
| `+0x100`, `+0x102`, `+0x104` | `s16` | three rotation angles |
| `+0x314 + i*0x2C` | object | `i = 0..2`, three hinge owners (`vtable[5]`) |
| `+0x3B4` | `u32` | object whose `+0x0C` is the shared offset `f6` |
| `+0x3B8 + i*0x7C` | `f32` x3 | per-pane translation, `i = 0..9` |
| `+0x3C4 + i*0x7C` | `f32` | per-pane scale numerator |
| `+0x3C8 + i*0x7C` | object | transform-loop pane `i` (`i = 0..9`, stride `0x7C`) |
| `+0x408 + i*0x7C` | object | update-loop owner `i` (`i = 0..9`, stride `0x7C`) |

Note: the two ten-element walks use different bases - `+0x408` for the *update*
loop and `+0x3C8` for the *transform* loop (`addi r23,r30,1032` vs
`addi r23,r30,968`), while the transform data of pane `i` sits at `+0x3B8` and
its scale at `+0x3C4`, i.e. `-0x10` and `-0x04` relative to the pane. That is
intentional in the shipped code and must be reproduced, not "fixed".

### Exact behaviour

1. Prologue: 320-byte frame; `f31`/`f30` `qr0` spill pair; `this = 0x802DCD60()`.
2. `p = (f32[3]){+0xDC, +0xE0, +0xE4}` copied to `r1+0xAC`;
   `f4 = 1.0 * *(f32 *)(*(u32 *)(this+0x3B4) + 0x0C)`; each of the three words
   is then `+= f4`, and `f4` is replicated to `r1+0x70/+0x74/+0x78`; the unit
   vector `1.0,1.0,1.0` goes to `r1+0x7C/+0x80/+0x84`.
3. For `i = 0..2`: `(*(void ***)(this+0x314+0x2C*i))[5](this+0x314+0x2C*i)`.
4. For `i = 0..9`: `(*(void ***)(this+0x408+0x7C*i))[5](this+0x408+0x7C*i)`.
5. **Group 1** (`+0x1AC`): stage the three `s16` angles `+0x100/+0x102/+0x104`
   on the stack; `0x801C0BD0(this + 0x7C)` (matrix reset);
   `0x8016EF50(this + 0x7C, &s16a, &s16b, &s16c)` (rotation);
   `0x8016A170(this + 0x1AC, this + 0x7C)`;
   `0x8016A150(this + 0x1AC, r1+0xAC)` (translation = the offset triple);
   `0x8016A1A0(this + 0x1AC, 0)`; `0x80168400(this + 0x1AC)`;
   `0x80168180(this + 0x1AC, 0, r1+0xB8)` - the last call fills `0x30` bytes at
   `r1+0xB8`, and its entries `r1+0xC4`, `r1+0xD4`, `r1+0xE4` are read next.
6. **Group 2** (`+0x1EC`), two consecutive binds from one matrix:
   (a) reset + rotation as above, then `0x8016A170(this+0x1EC, this+0x7C)`,
   `0x8016A150(this+0x1EC, r1+0xAC)`, `0x8016A1A0(this+0x1EC, 0)`;
   (b) then `0x8016A170(this+0x1EC, this+0x7C)` again,
   `0x8016A150(this+0x1EC, r1+0x64)` with
   `r1+0x64..+0x6C = 1.2 * (r1+0xAC..+0xB4)` (`1.2` = pool `+0x44`), and
   `0x8016A1A0(this+0x1EC, 0)`.
7. **Group 3** (`+0x22C`): the body stages
   `(r1+0xC4, r1+0xD4, r1+0xE4 - 24.0)` at `r1+0xA0..+0xA8`, but no callee reads
   that triple (the `0x8016A150` below is handed `r1+0xAC`), so the `-24.0`
   adjustment is dead in the shipped body; then reset + rotation, then
   `0x8016A170(this+0x22C, this+0x7C)`,
   `0x8016A150(this+0x22C, r1+0xAC)`, `0x8016A1A0(this+0x22C, 0)`.
8. **Ten-pane loop** (`k = 0..9`), with `base = this+0x3B8+0x7C*k`,
   `scale = this+0x3C4+0x7C*k`, `pane = this+0x3C8+0x7C*k`,
   `f6 = 1.0 * *(f32 *)(*(u32 *)(this+0x3B4) + 0x0C)`,
   `num = (f32)(s16)` at `0x808147C0` (= `20.0`):
   ```
   r  = *scale / num;
   v  = [(this+0xDC + f6) * r, (this+0xE0 + f6) * r, (this+0xE4 + f6) * r];
   0x801C0BD0(this + 0x7C, base[0], base[1], base[2]);
   0x8016EF50(this + 0x7C, &s16a, &s16b, &s16c);   // +0x100/+0x102/+0x104
   0x8016A170(pane, this + 0x7C);
   0x8016A150(pane, &v);
   0x8016A1A0(pane, 0);
   ```
9. Tail: `0x800F6910(this, 0x80814798, -40.0, 0.8)`
   (pool `0x80814798`: `+0x18` = 0.8, `+0x1C` = -40.0, `+0x28` = `s16`
   `0x0014` = 20).
10. Epilogue restores `f31`/`f30`, back-chain and `vrsave`.

**Signature.**

```rust
/// Runs the ten-pane layout pass: ticks the three hinges and ten panes, rebuilds
/// four shared matrices from the actor position and the three rotation angles,
/// then re-binds every pane and hands the object to `0x800F6910`.
/// # Original
/// `FUN_8075d5c0`, `0x8075D5C0`, 1,080 bytes,
/// `work/ghidra/rel-d_basesNP/export/8075d5c0_FUN_8075d5c0.c`.
/// # Arguments
/// * `this` - the layout owner (see the field map); no other register is read.
/// # Returns
/// Nothing.
/// # Confidence
/// medium - all four matrix sites, the ten-pane loop and every stride are
/// literal; the `+0x3C8`/`+0x408` walk offset and the `s16` angle triple are
/// read from the raw stream, not from the export.
pub unsafe fn fn_8075d5c0(this: usize)
```

**Constants.** pool base `r31 = 0x807CEC48`: `+0x40` = 1.0, `+0x44` = 1.2,
`+0x48` = 24.0, `+0x50` = `2^52 + 2^31`. Pool base `0x80814798`:
`+0x18` = 0.8, `+0x1C` = -40.0, `+0x28` = `s16` `0x0014` = **20** (the divisor
in the ten-pane loop). Strides `0x2C` (3 hinges), `0x7C` (10 panes).
Callees `0x801C0BD0`, `0x8016EF50`, `0x8016A170`, `0x8016A150`, `0x8016A1A0`,
`0x80168400`, `0x80168180`, `0x800F6910`, `0x802DCD60`/`0x802DCDAC`.

**Verdict: ready (medium).**

---

## 7. `0x80763E50` - the world-map event open/poll handler

**Evidence.** `work/ghidra/rel_d_basesNP/export/80763e50_FUN_80763e50.c`,
1,056 bytes, 264 words, synthetic `0x80763E50`.

**What it is.** A two-phase handler for six world-map event ids:
`0x5F` (95), `0x15` (21), `0x16` (22), `0x17` (23), `0x18` (24), `0x19` (25).
Phase "open" (`arg3 != 0`) starts one layout animation per id and submits one
`wm_effect_record_submit` (`0x80105240`, translated in
`src/game/worldmap.rs`) with effect ids `39`/`40`/`41`; phase "poll"
(`arg3 == 0`, and also the tail of the open phase) asks each animation whether
it finished through `0x80169020` and raises `+0x139` / `+0x3D0`.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x60` | object | base object vtable used on the `0x5F` "not ours" path (`vtable+0x68`) |
| `+0xAC` | `f32` x3 | world position handed to `0x80105240` |
| `+0x139` | `u8` | "event finished" flag, set in the poll phase |
| `+0x1A8` | object | shared layout/anim owner A (`+0x18` slot = play) |
| `+0x1B4` | object | animation B |
| `+0x1EC` | object | animation C |
| `+0x224` | object | animation D |
| `+0x25C` | object | animation E |
| `+0x294` | object | animation F |
| `+0x2CC` | object | animation G |
| `+0x3D0` | `u8` | "busy" flag, cleared for `0x5F` and set on the `0x15` family |

### Exact behaviour

Entry: `if (arg2 == -1) goto tail; if (0x800F60E0(this) == 0) { this+0x139 = 1;
goto tail; }`

**Open phase** (`arg3 != 0`), `switch (arg2)`:

```
0x5F: if (0x800FD800(this) == 4) {
          if (0x800FD820(this) == 4 || 0x800FD820(this) == 7) {
              this+0x3D0 = 0;
              0x80168520(this+0x1A8, 0);
              (*(void ***)(this+0x1A8))[6](this + 0x1A8, this + 0x2CC);
              0x80168FE0(this + 0x2CC, 0.0);
              0x801680E0(this + 0x2CC, 1.0);
              0x80105240(*(u32 *)0x8042A27C, 41, this + 0xAC, 1);
          } else (*(void ***)(this+0x60))[0x68](this);
      } else (*(void ***)(this+0x60))[0x68](this);
0x15: if (*(u8 *)(this+0x3D0) != 0) break;
      0x80168520(this+0x1A8, 0);
      (*(void ***)(this+0x1A8))[6](this + 0x1A8, this + 0x1B4);
      0x801680E0(this + 0x1B4, 2.4); 0x80168FE0(this + 0x1B4, 0.0);
      0x80105240(*(u32 *)0x8042A27C, 39, this + 0xAC, 1);
0x17: 0x80168520(this+0x1A8, 0);
      (*(void ***)(this+0x1A8))[6](this + 0x1A8, this + 0x1EC);
      0x80168FE0(this + 0x1EC, 0.0); 0x801680E0(this + 0x1EC, 1.0);
      0x80105240(*(u32 *)0x8042A27C, 41, this + 0xAC, 1);
0x16: ... (this + 0x224), 0x80168FE0(0.0), 0x801680E0(1.0), effect 40;
0x18: ... (this + 0x25C), 0x80168FE0(0.0), 0x801680E0(1.0), effect 41;
0x19: 0x8015F730(0x8081576C) first (the translated display packet; the
      `crxor` at 0x80764110 is the varargs marker), then (this + 0x294),
      0x80168FE0(0.0), 0x801680E0(1.0), **no** effect submit;
default: break;
```

**Poll phase** (`arg3 == 0`, and the common tail), `switch (arg2)`, each arm
first asks `0x80169020(object)` (animation finished?) and then sets flags:

```
0x5F: if (0x80169020(this+0x2CC)) this+0x139 = 1;
0x15: if (0x80169020(this+0x1B4) || *(u8 *)(this+0x3D0) != 0) {
          this+0x3D0 = 1; this+0x139 = 1; }
0x17: if (0x80169020(this+0x1EC)) { this+0x3D0 = 0; this+0x139 = 1; }
0x16: this+0x3D0 = 1; this+0x139 = 1;            // unconditional
0x18: if (0x80169020(this+0x25C)) { this+0x3D0 = 0; this+0x139 = 1; }
0x19: if (0x80169020(this+0x294)) { this+0x3D0 = 1; this+0x139 = 1; }
default: this+0x139 = 1;
```

Tail: restore `r31`, `r30`, `r29`, `r28`, `vrsave`, `blr`.

Order matters in every arm: the `0x80168520(bind, 0)` call precedes the
`vtable[6]` play call. The two value setters are always
`0x80168FE0(obj, 0.0)` then `0x801680E0(obj, <scale>)` in the `0x5F`, `0x17`,
`0x16`, `0x18` and `0x19` arms (`<scale>` = `1.0`), while the `0x15` arm is the
one arm that inverts the pair: `0x801680E0(this+0x1B4, 2.4)` first, then
`0x80168FE0(this+0x1B4, 0.0)` (`0x80763FCC..0x80763FE0`).

**Signature.**

```rust
/// World-map event handler for ids 0x5F/0x15/0x16/0x17/0x18/0x19: starts (or
/// polls) one layout animation per id and submits effect record 39/40/41.
/// # Original
/// `FUN_80763e50`, `0x80763E50`, 1,056 bytes,
/// `work/ghidra/rel-d_basesNP/export/80763e50_FUN_80763e50.c`.
/// # Arguments
/// * `this` - the event object (see the field map).
/// * `event` - the event id; `-1` returns immediately.
/// * `start` - `0` = poll, non-zero = open.
/// # Returns
/// Nothing.
/// # Confidence
/// medium - all six arms, both phases and every effect id are literal; the four
/// unclaimed `0x800F60E0`/`0x800FD800`/`0x800FD820`/`0x80169020` helpers are
/// named by address only.
pub unsafe fn fn_80763e50(this: usize, event: i32, start: u32)
```

**Constants.** effect ids `39` (0x27), `40` (0x28), `41` (0x29); event ids
`95`, `21`, `22`, `23`, `24`, `25`; `0x801679...`-family floats `0.0`, `1.0`,
`2.4` (pool `0x807CEE80`: `+0x20` 0.0, `+0x24` 1.0, `+0x28` 2.4); globals
`0x8042A27C` (effect owner), `0x8081576C` (display packet string).

**Verdict: ready (medium).**

---

## 8. `0x806765B0` - the `houdai` placement initialiser

**Evidence.** `work/ghidra/rel-d_basesNP/export/806765b0_FUN_806765b0.c`,
1,052 bytes, 263 words, synthetic `0x806765B0`.

**What it is.** The `houdai` (cannon/base) actor's "place from parameter word"
body. It is bracketed by translated members of the same class:
`fn_80676e10` (houdai BRRES + `mouth`/`body`/`root` pane bind at
`+0x3F8`/`+0x438`/`+0x478`, `src/game/bases.rs:26164`) and the
`+0x614`/`+0x618` float pair driven by `fn_80677110`/`fn_80677160`
(`src/game/bases.rs:130201`, `:130311`). It reads the actor's parameter word at
`+0x4`, decodes three 4-bit fields out of it, derives the placement, builds a
seven-word stage record and hands it to `0x8007FB10` (translated stage record
submit, `src/game/stage.rs`), then re-binds the `houdai` model through
`0x80676E10` and runs the `+0x394` interface slot with `0x80829F98`.

### Field map

`flags = *(u32 *)(this + 4)` (the spawn parameter word):

| field | expression | use |
|---|---|---|
| `f0` | `flags & 0xF` | table column; also `+0x61C` and the two branch selectors |
| `f1` | `(flags >> 4) & 0xF` | table row; also `+0x62C` (`1` selects the narrow form) |
| `f2` | `(flags >> 8) & 0xF` | count - 1; `+0x620 = f2 + 1` (doubled when `f1 != 1`) |

`this`:

| offset | type | meaning |
|---:|---|---|
| `+0xAC`, `+0xB0`, `+0xB4` | `f32` | spawn position |
| `+0xDC`, `+0xE0`, `+0xE4` | `f32` | velocity (here `1.0`, `0.5` or `0.25`) |
| `+0x38F` | `u8` | player/owner index forwarded to `0x8007FB10` |
| `+0x4B8`, `+0x4BC`, `+0x4C0`, `+0x4C4` | `f32` | rect handed to `0x8007FB10` |
| `+0x4D4` | object | stage record owner (`0x8007FB10` / `0x8007F900`) |
| `+0x5C4`, `+0x5C8`, `+0x5CC` | `f32` | placement copy of the spawn position |
| `+0x5D0`, `+0x5D4`, `+0x5D8` | `f32` | unit scale (`1.0` or `0.5`) |
| `+0x5DC`, `+0x5DE` | `u16` | cleared |
| `+0x5E0` | `u16` | s16 read from the literal pool (see below) |
| `+0x5E4` | `f32` | count-derived offset |
| `+0x5E8`, `+0x5EC` | `f32` | scale (`1.0` or `0.5`) |
| `+0x5F0`, `+0x5F4`, `+0x5F8` | `f32` | shifted position |
| `+0x5FC`, `+0x600`, `+0x604` | `f32` | scale |
| `+0x608`, `+0x60C`, `+0x610` | `f32` | shifted position (mirror) |
| `+0x61C` | `u32` | `f0` |
| `+0x620` | `i32` | count (`1..16`, doubled for `f1 != 1`) |
| `+0x62C` | `u32` | `f1` |

`+0x614` and `+0x618` (the pair `fn_80677160` drives) are **not** written here.

### Exact behaviour

1. `p = (f32[3]){+0xAC, +0xB0, +0xB4}`; `flags = *(u32 *)(this+4)`;
   `f0 = flags & 0xF`, `f1 = (flags>>4)&0xF`, `f2 = (flags>>8)&0xF`.
2. `p.y += kF32[0x807C92B0 + 4*(2*f1 + f0)]` (the two-index table; e.g.
   `2*f1+f0 = 0` gives `8.0`).
3. `poly = s16[0x807C92C0 + 2*f0]` (see the pool note below).
4. `count = f2 + 1`.
5. Publish: `+0x61C = f0`, `+0x62C = f1`, `+0x620 = count`,
   `+0x5C4 = p.x`, `+0x5C8 = p.y`, `+0x5CC = p.z`, `+0x5DC = 0`, `+0x5DE = 0`,
   `+0x5E0 = poly`, and the two full position triples
   `+0x5F0..+0x5F8 = p` and `+0x608..+0x610 = p`.
6. **`f1 == 1` branch.** `k = 1.0`;
   `+0xDC = +0xE4 = 1.0`, `+0x5D0/+0x5D4/+0x5D8/+0x5FC/+0x600/+0x604/
   +0x5E8/+0x5EC = 1.0`; `+0x5E4 = (f32)count`;
   if `f0 == 0`: `+0x5F0 = (p.x - 16.0*count*0.5) - 8.0`,
   `+0x608 = p.x - (16.0*count - 16.0)`;
   else: `+0x5F0 = (p.x + 16.0*count*0.5) + 8.0`,
   `+0x608 = p.x + (16.0 + 16.0*count)`.
7. **`f1 != 1` branch.** `count = count*2 + 1`, `+0x620 = count`, `k = 0.5`;
   velocity `1.0 -> 0.5` (`+0xDC/+0xE0/+0xE4 = 0.5`), the eight scale slots
   `= 0.5`, `+0x5E4 = 0.5 * (f32)count`;
   if `f0 == 0`: `+0x5F0 = (p.x - 8.0*count*0.5) - 4.0`,
   `+0x608 = p.x - (8.0*count - 8.0)`;
   else: `+0x5F0 = (p.x + 8.0*count*0.5) + 4.0`,
   `+0x608 = p.x + (8.0 + 8.0*count)`.
   (The `f0 != 0` halves are reached through `fadds`; the `f0 == 0` halves
   through `fsubs` - the export shows them as separate blocks and they differ
   only in the two signs and the `+/- 8.0`/`+/- 4.0` term.)
8. Common: `+0x638 = +0x5D0`; `+0x4C8 = +0x4CC = +0x4D0 = 0`.
9. `if (*(i32 *)(this+0x62C) == 1)`:
   ```
   +0x4C0 = 8.0; +0x4BC = 16.0; +0x4C4 = -16.0;
   if (*(u32 *)(this+0x61C) == 0)  +0x4B8 = -8.0 - 16.0 * (1.0 + (f32)count);
   else { +0x4B8 = -8.0; +0x4C0 = 8.0 + 16.0 * (1.0 + (f32)count); }
   ```
   else (`+0x62C != 1`):
   ```
   +0x4BC = 32.0; +0x4C4 = 0.0;
   if (*(u32 *)(this+0x61C) == 0) { +0x4C0 = 8.0;
       +0x4B8 = -(16.0 * (1.0 + (f32)count)); }
   else { +0x4B8 = -8.0; +0x4C0 = 16.0 * (1.0 + (f32)count); }
   ```
   (`count` is `+0x620`, already doubled on the `f1 != 1` path.)
10. `0x8007FB10(this + 0x4D4, this, &this->+0x4B8, 3, *(u8 *)(this+0x38F),
    this + 0x5D0)` - the seven-word stage record (`two points + three words`,
    `src/game/stage.rs`). Raw argument order: `r3` = record owner,
    `r4` = `this`, `r5` = `&rect`, `r6` = `3`, `r7` = the `+0x38F` byte,
    `r8` = `+0x5D0`.
11. `0x8007F900(this + 0x4D4)` - the record flush.
12. `0x80676E10(this)` - the `houdai` BRRES/pane bind.
13. `(*(void ***)(this+0x394))[6](this + 0x394, 0x80829F98)`.
14. `return 1;` (the body always reports success).

### The one unresolved constant

Step 3 reads a **signed halfword** at `0x807C92C0 + 2*f0`, i.e. a 16-entry
`s16` table, while step 2 reads `f32` at `0x807C92B0 + 4*(2*f1+f0)`, i.e. a
46-entry `f32` table. The two ranges overlap (`0x807C92C0..0x807C92DF` is
inside the `f32` table), so the shipped literal pool cannot be both. The
`f32` interpretation is confirmed by its own values (`8, 8, 0, 0, 1, 16, 0.5,
8, 4, -8, -16, 32, 0, 2^52, 0, 18, -18, ...`) which are exactly the kind of
grid constants this body uses; the `s16` read lands on the *high halves* of
those floats, which is only meaningful if the original source indexed a
`u16`/`s16` array at `0x807C92C0` that the linker overlapped with the float
pool - possible, but not provable from the image. `+0x5E0` is written once and
never read by this body, so an implementer may transcribe the read verbatim
(`i16::from_be_bytes(image[0x807C92C0 + 2*f0 ..][..2])`) and defer the meaning;
that single site is why the verdict is conditional rather than ready.

**Signature.**

```rust
/// Places the `houdai` actor from its 12-bit parameter word: decodes the three
/// nibbles, derives the grid offset, publishes two shifted position triples and
/// the count scales, then submits the stage record and re-binds the model.
/// # Original
/// `FUN_806765b0`, `0x806765B0`, 1,052 bytes,
/// `work/ghidra/rel-d_basesNP/export/806765b0_FUN_806765b0.c`.
/// # Arguments
/// * `this` - the `houdai` actor; `+0x4` is the spawn parameter word.
/// # Returns
/// Always `1`.
/// # Confidence
/// medium - every offset, sign, count and call is literal; the `s16` read at
/// `0x807C92C0 + 2*f0` overlaps the `f32` pool and is the one open input.
pub unsafe fn fn_806765b0(this: usize) -> u32
```

**Constants.** The body addresses the pool through `r10 = 0x807C9298`, so the
placements are: `+0x18` = the 46-entry `f32` placement table (index
`2*f1 + f0`, stride 4: `8.0, 8.0, 0.0, 0.0, 0.0, 1.0, 16.0, 0.5, 8.0, 4.0,
-8.0, -16.0, 32.0, 0.0, 2^52, 0.0, 18.0, -18.0, ...`, then again
`8.0, 8.0` at `+0x58`/`+0x5C`); `+0x28` = the `s16` read for `+0x5E0`
(entries `-32768, 0, 16256, 0, ...`, i.e. the high halves of the floats above -
see the note); and the scalars `+0x2C` = 1.0, `+0x30` = 16.0, `+0x34` = 0.5,
`+0x38` = 8.0, `+0x3C` = 4.0, `+0x40` = -8.0, `+0x44` = -16.0, `+0x48` = 32.0,
`+0x4C` = 0.0, `+0x50` = `2^52` (`0x43300000_00000000`). `0x80829F98` is the
REL data word handed to the `+0x394` vtable slot.

**Verdict: conditional (medium).** Blocked only on the `0x807C92C0` halfword
read; every other byte of the body is decoded.

---

## Verification

- Addresses, sizes and names re-read from
  `work/ghidra/rel-d_basesNP/export/functions.tsv` for this pass.
- All eight `.c` exports exist under
  `work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`.
- None of the eight addresses appears in the `address` column of
  `ledger/functions.csv`, and none appears in a heading of any `docs/*.md`.
- The decoder found `blr` at `address + size - 4` in all eight bodies, so each
  TSV size is the true span.
- `psq_` census: `0x807A84B0` 8 (`f31..f28` spill/restore pairs),
  `0x807BEF60` 4 (`f31`/`f30` pair), `0x8075D5C0` 4 (`f31`/`f30` pair), the
  other five 0. Every one is `qr0`, `W=0`; no GQR is read and no `ps_*`
  arithmetic exists in any of the eight.
- No file in the repository was written except this report.
