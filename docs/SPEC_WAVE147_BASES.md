# Wave 147 - `game::bases` - the next eight (0.79-0.83 KB band)

Read-only specification lane. No Rust file, no `ledger/functions.csv` row and no
existing document was modified. The only file written is this one. No merge or
check tool was run. The throwaway decoder and the ELF/literal readers live under
`%TEMP%\opencode\w147` (`ppc.js`, `dis.js`, `cand.js`), never in the repository.

Waves 128-146 have walked this bucket from the top in size order; the last row
claimed by `docs/SPEC_WAVE146_BASES.md` is 848 bytes. This wave continues one
rung lower, **844 bytes down to 812 bytes**, which is the eight-row set
`docs/SPEC_WAVE146_BASES.md:99-101` predicted verbatim. Every row is decoded
instruction by instruction, gets a field map, a numbered behaviour list, a
constant table and an exact signature.

## The eight

| # | address | bytes | words | export file | verdict |
|--:|---:|---:|---:|---|:--:|
| 1 | `0x8069BE20` | 844 | 211 | `8069be20_FUN_8069be20.c` | ready (high) |
| 2 | `0x807373B0` | 844 | 211 | `807373b0_FUN_807373b0.c` | ready (high) |
| 3 | `0x80705C60` | 840 | 210 | `80705c60_FUN_80705c60.c` | ready (medium) |
| 4 | `0x80782F60` | 836 | 209 | `80782f60_FUN_80782f60.c` | ready (high) |
| 5 | `0x80775190` | 820 | 205 | `80775190_FUN_80775190.c` | ready (medium) |
| 6 | `0x8079A930` | 816 | 204 | `8079a930_FUN_8079a930.c` | ready (medium) |
| 7 | `0x807A7220` | 816 | 204 | `807a7220_FUN_807a7220.c` | ready (medium) |
| 8 | `0x807B84C0` | 812 | 203 | `807b84c0_FUN_807b84c0.c` | ready (medium) |

**Ready: 8. Conditional: 0.**

Every `bytes` value is the `size` column of
`work/ghidra/rel-d_basesNP/export/functions.tsv` verbatim and `decompiled` is
`true` for all eight. In all eight the size is the true instruction span: the
decoder found exactly one `blr`, at `address + size - 4`
(`0x8069C168`, `0x807376F8`, `0x80705FA4`, `0x807832A0`, `0x807754C0`,
`0x8079AC5C`, `0x807A754C`, `0x807B87E8`), and no earlier one. There is no
padding tail, no `halt_baddata` cut and no unconditional branch out of the
extent: **0** of the 1,657 words in the eight bodies is undecodable, and every
`b` target is inside its own body.

### Why all eight are ready

The verdict rule is wave 144's, unchanged:

* **ready** - every reachable instruction is decoded, every literal is read out
  of the image, every callee is named by address (or by its ledger name), and
  the body can be written as code with no assumed calling convention beyond the
  standard PowerPC EABI. Data in `main.dol` BSS (`0x8035xxxx`, `0x8037xxxx`,
  `0x8042xxxx`), in this REL's BSS (`0x80822900..0x80834D84`), or in the other
  RELs (`0x80C5xxxx`, `d_en_bossNP`) is an *input*, named by address.
* **conditional** - exactly one named input would still be missing.

Two rows needed a check against the wave-101/127 GQR note. The raw stream
answers it for both:

* row 1 has one `psq_st f31,40(r1),qr0,0` / `psq_l` pair - the plain `f31`
  spill - and no data-side `psq` at all.
* row 2 has four `stfd`/`psq_st ...,qr0,0` save pairs (`f31`, `f30`, `f29`,
  `f28`) and four matching restores. `qr0` is type 0 (plain `f32` pair), so the
  bracket is a register save, not a quantised load. This is exactly the wave-127
  finding that unblocked the `d_basesNP` `unaff_GQR0` pool, and it retires
  wave 30's and wave 101's rejection of `0x807373B0` ("paired-single failure").
* rows 3-8 contain **no** `psq_l`/`psq_st` word at all.

### This is the list wave 146 predicted, exactly

Re-deriving the candidate set from the current tree gives **1,928** rows: the
5,615 `functions.tsv` rows, filtered against the 16,910 addresses in
`ledger/functions.csv` and the 1,510 distinct `0x`/bare addresses that already
appear in a `#{1,6} ` heading of `docs/*.md`, and against `decompiled=true`.
Wave 146 published 1,941 before the wave-146/147 ledger merges; the 13-row
difference is the 13 candidate addresses those translation lanes have since
claimed in `ledger/functions.csv`. The first eight are byte-for-byte
`docs/SPEC_WAVE146_BASES.md:99-101`:

```
0x8069BE20 844 | 0x807373B0 844 | 0x80705C60 840 | 0x80782F60 836
0x80775190 820 | 0x8079A930 816 | 0x807A7220 816 | 0x807B84C0 812
```

The 844-byte tie is broken by address ascending (`0x8069BE20` before
`0x807373B0`), exactly as wave 146 said it would be. Each of the eight was
re-tested against `ledger/functions.csv` (exact `address` field), against every
`#{1,6} ` heading in `docs/*.md` (including the bare and lowercase spellings), and
against `src/game/bases.rs` (`pub unsafe fn fn_<addr>` absent). All eight are
absent from all three, and each has a `decompiled=true` TSV row with a matching
`.c` export on disk.

Applying the same filter after this wave, the next eight are:
`0x806DA520` (804), `0x8071E130` (788), `0x80788CB0` (788), `0x80763290` (780),
`0x80794C00` (780), `0x8075F270` (768), `0x8066DDC0` (764), `0x807727D0` (764).

## Method

**Address mapping.** Re-derived from the program headers of
`work/rels/elf/d_basesNP.elf` (also visible in `work/rels/elf/summary.json`),
not copied:

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
0x1000` for every code address in this document, as `docs/REL_ELF_MAPPING.md`
requires. Literal pools in `0x807Cxxxx`/`0x807Dxxxx` resolve through segments
1-4 and are read out verbatim below. `0x8082xxxx` below `0x80822900` is real
data; `0x80822900..0x80834D84` is this REL's BSS and reads as zero.

**Decoder.** Reused the wave-146 throwaway decoder (`%TEMP%\opencode\w147`) and
re-validated it against the facts `docs/REL_ELF_MAPPING.md` pins: branch `LK` is
bit 0 and `AA` bit 1; conditional branches use the 14-bit `BD`; the `SPR` field
is `((word >> 11) & 0x1F) << 5 | ((word >> 16) & 0x1F)`; `psq_l` = 56 and
`psq_st` = 60 with the quantiser in bits 12-14 and `W` in bit 15; single-float
`fmuls`/`fdivs` use `FRC`. One decoder defect is worth repeating because two of
these bodies rely on it: `mtctr` is `mtspr 9, rS`, encoded as opcode 31
`XO = 467`, and a decoder that maps `XO 467` to `mtsr` prints "`mtsr 9,r12`"
where the bytes mean `mtctr r12`. Every virtual call in these eight appears as
`lwz r12,slot; lwz r12,off(r12); mtctr r12; bctrl`; the slot offsets below are
read from those two loads, not from the decompiler.

**Receiver recovery.** Rows 4-8 open with a `bl` to one of the CodeWarrior
`_savegpr_N` helpers (`0x802DCD54`, `0x802DCD70`, `0x802DCD58`, `0x802DCD6C`,
`0x802DCD74`). Those helpers preserve `r3..r10` (`docs/REL_ELF_MAPPING.md`
"CodeWarrior helpers"), so the incoming `r3` and the Ghidra "receiver-less"
signatures are recoverable: `r30` (row 4), `r26` (row 5), `r26` (row 6), `r30`
(row 7) and `r30` (row 8) hold `this` after the save. The signatures are
therefore `fn(this)` for rows 4, 5, 7 and 8, and `fn(this, r4, r5)` for row 6.

**Unexported callees.** `0x800A4E90`, `0x800F2A00`, `0x80101670`,
`0x800FB2D0`, `0x800FC370`, `0x80069500`, `0x8005EB10`, `0x800E3870`,
`0x800B0EA0`, `0x80166F60`, `0x800690C0`, `0x800690E0`, `0x80100200`,
`0x80C56600` and `0x80C565A0` have no `functions.tsv` row and no `.c` export.
Each was checked against the exported extent that precedes it: every one starts
4-0xC bytes after that extent ends, except `0x800F2A00`/`0x800FC370`/
`0x80100200`, which sit inside larger un-analysed gaps but are called twice with
different arguments in the same body (so they are real entry points, not
interior labels). They are named by address only, exactly as
`docs/SPEC_WAVE128_BASES.md` and `docs/SPEC_WAVE146_BASES.md` do.

**Call notation.** Throughout this document `callN(addr, a, b, ...)` lists the
integer argument registers `r3, r4, ...` from left to right; `callNf(addr, ...)`
does the same and appends the float arguments `f1, f2, ...` after the integer
ones. Where a callee takes a float in `f1` independently of (or instead of)
`r4`, the call is written `addr(f1 = ..., r3 = ...)` explicitly. This matters
for `0x801680E0`, `0x80168FE0` and `0x80169F80`, whose Ghidra exports show the
double as the first C parameter because Ghidra folded the `f1` argument into
the prototype.

**Cross-checks against earlier specifications.** Three rows are re-reads of an
already-transcribed sibling, and this pass matches them:

* row 1 vs `docs/SPEC_WAVE128_BASES.md` section 3 (`0x8069B750`): the shared
  tail from `0x800A4E90` through `+0x33C`/`+0x340`/`+0x344`/`0x8015F470`/
  `0x80072BC0`/`0x80699E30`/`+0x374`/`0x800A4480` is identical. Row 1's state
  block and its last two steps differ (it ends with `0x80699FB0` + the
  conditional `+0xD4` message, where the sibling calls `0x8069A230`); both are
  written out in full below.
* row 8 vs `docs/SPEC_WAVE146_BASES.md` section 7 (`0x807B77E0`): row 8 allocates
  exactly the eight owner slots that wave 146's teardown releases
  (`0x8042A1D0`, `0x8042A1D4`, `0x8042A1D8`, `0x8042A1DC`, `0x8042A1E0`,
  `0x8042A1E4`, `0x8042A1E8`, `0x8042A1EC`, plus `0x80429F30`, `0x8042A078`,
  `0x80429D30`, `0x80429D48`, `0x80429D88`). The two bodies are the mount and
  unmount halves of one scene.
* row 5 vs `docs/SPEC_WAVE140_BASES.md:1291-1294`: the field map
  (`+0x3220` count, `+0x3224` index, `+0x3180` 10-byte record table, `+0x338C`
  mode) is exactly that spec's map for the route player it is called from.

---

## 1. `0x8069BE20` - per-frame update of the `+0x544`-model actor

**Evidence.** `work/ghidra/rel-d_basesNP/export/8069be20_FUN_8069be20.c`,
844 bytes, 211 words, synthetic `0x8069BE20`. Body
`0x8069BE20..0x8069C168` (ends `blr`).

**What it is.** A per-frame update of the same class as the already-specified
`0x8069B750` carried-object body (`docs/SPEC_WAVE128_BASES.md` section 3). It
ticks the rider sub-object at `+0x544`, samples the player-like object returned
by class slot `+0x2DC` through its `+0x5F8` interface, latches five mode bytes
`+0xD30..+0xD34` from the geometry and the `+0xDCC` mode, runs a two-state gate
on `+0x360`/`+0x500`, and then executes the common tail (motion clamps, two
orientation pairs, animation state and the `+0x788` fallback message). The
export's `void FUN_8069be20(int param_1)` is accurate; `param_1` is `this`.

### Signature

```rust
pub unsafe fn fn_8069be20(this: usize)
```

`r3` is the actor. There is no return value: the epilogue restores `r29-r31`,
`f31`, LR and returns without touching `r3`.

### Field map (offsets from `this`)

| offset | type | meaning |
|---:|---|---|
| `+0x00` | - | object header (not read here) |
| `+0x60` | `u32` | class vtable pointer; every `+0xNNN` slot below is read from it |
| `+0x114`,`+0x11C` | `f32` | two "orientation" scalars written by `0x8015F470` |
| `+0x1EC` | opaque | motion/keyframe object; `0x80072BC0`/`0x8006EF40`/`0x8006FA40` |
| `+0x348` | `u8` | geometry latch: `[+0xAC] >= [+0x750]` |
| `+0x360` | `s32` | state: `0`, `1`, `2` |
| `+0x500` | `u16` | state-1 timer, seeded with `80` (`0x807F13D8`) |
| `+0x544` | opaque | rider/model sub-object; virtual slot `+0x1C` |
| `+0x5F8` | `u32` | interface pointer of the object returned by slot `+0x2DC` |
| `+0x750` | `f32` | compare threshold for `+0xAC` (role inferred: height/limit) |
| `+0x788` | `u32` | gate word for the final `+0xD4` message |
| `+0xAC` | `f32` | compared with `+0x750` (role inferred) |
| `+0xD30`..`+0xD34` | `u8` | five one-shot mode flags |
| `+0xDCC` | `s32` | mode word: `0` and `2` are latched into `+0xD33`/`+0xD34` |
| `+0xEC` | `f32` | ground/height value, clamped to `0.0` in both directions |

Globals: `0x807F13D8` (`u32` = `0x50`, the `+0x500` seed), `0x807CA08C`
(`f32 0.0`), `0x8082B5AC` (this REL's BSS: an empty message cell passed as the
second argument of the `+0xD4` slot).

### Exact behaviour

Prologue `0x8069BE20..0x8069BE44`: `stwu r1,-0x30`; save LR at `0x34(r1)`;
`stfd f31,0x20` + `psq_st f31,0x28,qr0,0`; save `r31,r30,r29`;
`r31 = this`. Epilogue `0x8069C148..0x8069C168`.

```rust
// 0x8069BE44..0x8069BE54: vcall(this+0x544, +0x1C)(this+0x544)
// 0x8069BE58..0x8069BE6C: obj = vcall(this, +0x2DC)(this)
// 0x8069BE70..0x8069BEAC: the two clip predicates are short-circuited
let iface = read_u32(obj, 0x5F8);
let mut alive = vcall(iface, 0x0C)() != 0;
if !alive { alive = vcall(iface, 0x10)() != 0; }
if alive { vcall(this, 0x380)(this); }        // 0x8069BEB4..0x8069BEC8

// 0x8069BECC..0x8069BF0C: geometry latch + two one-shots
let above = read_f32(this, 0xAC) >= read_f32(this, 0x750);
write_u8(this, 0x348, above as u8);
if above { write_u8(this, 0xD31, 1); } else { write_u8(this, 0xD32, 1); }

// 0x8069BF10..0x8069BF34: mode latch
match read_s32(this, 0xDCC) {
    0 => write_u8(this, 0xD33, 1),
    2 => write_u8(this, 0xD34, 1),
    _ => {}
}

// 0x8069BF38..0x8069BFB4: two-state gate
match read_s32(this, 0x360) {
    1 => {
        if read_u16(this, 0x500) == 0 && call1(0x800A4C10, this) != 0 {
            write_u8(this, 0xD30, 1);
            write_u16(this, 0x500, 0x50);          // [0x807F13D8]
            write_s32(this, 0x360, 2);
        }
    }
    2 => {
        if vcall(this, 0x3A0)(this) != 0 {
            write_s32(this, 0x360, 1);
        }
    }
    _ => {}
}
```

Then the shared tail:

1. `0x8069BFB8`: `call1(0x800A4E90, this)`.
2. `0x8069BFC0`: `vcall(this, 0x33C)(this)`; `0x8069BFD4`:
   `vcall(this, 0x340)(this)`.
3. `0x8069BFE8`: `call1(0x8006CD90, this)`; `0x8069BFF0`:
   `call1(0x8006CDE0, this)`; `0x8069BFF8`: `call1(0x800955F0, this)`.
4. `0x8069C000..0x8069C038`: `f2 = vcall(this, 0x350)(this)` (saved in `f31`),
   `f1 = vcall(this, 0x34C)(this)`, then
   `0x8015F470(f1, f2, this + 0x114)`.
5. `0x8069C038..0x8069C070`: the same pair with slots `+0x358` then `+0x354`
   into `this + 0x11C`.
6. `0x8069C070`: `vcall(this, 0x344)(this)`.
7. `0x8069C084..0x8069C0A8`:
   ```rust
   if call1(0x80072BC0, this + 0x1EC) != 0
      && read_f32(this, 0xEC) < 0.0 { write_f32(this, 0xEC, 0.0); }
   ```
8. `0x8069C0AC..0x8069C0D4`:
   ```rust
   if call2(0x8006EF40, this + 0x1EC, 0) != 0
      && read_f32(this, 0xEC) > 0.0 { write_f32(this, 0xEC, 0.0); }
   ```
9. `0x8069C0D8`: `call2(0x8006FA40, this + 0x1EC, 0)`.
10. `0x8069C0E4`: `call1(0x80699E30, this)`.
11. `0x8069C0EC..0x8069C10C`:
    `let sel = (vcall(this, 0x374)(this) as i16) as i32;`
    `call2(0x800A4480, this, sel)`.
12. `0x8069C110..0x8069C144`:
    ```rust
    if call1(0x80699FB0, this) != 0 && read_u32(this, 0x788) == 0 {
        vcall(this, 0xD4)(this, 0x8082B5AC);
    }
    ```

### Callees

| target | shape | note |
|---|---|---|
| `this+0x544` -> `+0x1C` | `(obj)` | rider tick |
| `this` -> `+0x2DC` | `(this) -> obj` | player-like object accessor |
| `obj+0x5F8` -> `+0x0C`,`+0x10` | `() -> u32` | two clip-idle predicates (documented in `docs/COVERAGE_WAVE119_BASES.md:71-73`) |
| `this` -> `+0x380` | `(this)` | "both clips idle" reaction |
| `this` -> `+0x33C`,`+0x340`,`+0x344` | `(this)` | per-frame phases |
| `this` -> `+0x34C`,`+0x350` / `+0x354`,`+0x358` | `(this) -> f32` | two orientation pairs |
| `this` -> `+0x374` | `(this) -> s16` | animation/index selector |
| `this` -> `+0xD4` | `(this, &msg)` | one-shot message |
| `0x800A4C10` | `fn_800a4c10` (`main.dol`) | `(this) -> u32`; returns 1 iff `+0xD2C == 0 && +0x74C == 0` |
| `0x800A4E90` | unexported `main.dol` gap | `(this)` |
| `0x800A4480` | `fn_800a4480` | `(this, s16)`; itself calls slot `+0x370` then `0x8015F4F0(this+0x102, s16, arg)` |
| `0x800955F0`, `0x8006CD90`, `0x8006CDE0` | `fn_800955f0` / `fn_8006cd90` / `fn_8006cde0` | `(this)` motion/approach helpers |
| `0x8015F470` | `thunk_FUN_8015f480` | `(dest, f1 target, f2 step)` - steps `*dest` toward `f1` |
| `0x80072BC0`, `0x8006EF40`, `0x8006FA40` | `(this+0x1EC[, 0]) -> u32` | motion queries |
| `0x80699E30` | `FUN_80699e30` | `(this)` animation-state switch |
| `0x80699FB0` | `FUN_80699fb0` | `(this) -> u32`; `+0xDA0 <= +0xAC <= +0xD9C` |

**Constants.** `0x807F13D8 = 0x00000050` (80, the `+0x500` seed);
`0x807CA08C = 0.0f`; `0x8082B5AC` = this REL's BSS message cell (zero-fill).

**Confidence: high.** The shared tail matches the transcribed `0x8069B750`
sequence call for call up to `0x800A4480`; every receiver, slot and argument
register is written immediately before its `bctrl`/`bl`; the only save bracket
is the `f31` `qr0` pair.

---

## 2. `0x807373B0` - 16x16 tile-run painter

**Evidence.** `work/ghidra/rel-d_basesNP/export/807373b0_FUN_807373b0.c`,
844 bytes, 211 words, synthetic `0x807373B0`. Body
`0x807373B0..0x807376F8` (ends `blr`).

**What it is.** A draw/refresh override: the vtable block starting at
`0x8080DF00` contains this body at `0x8080DF54` (slot `+0x54`). It paints a
vertical run of 16x16 tiles through
`0x80077860`, starting at the object's position `([+0xAC]-8, [+0xB0]-8)` and
continuing downward in 16-unit steps while the y coordinate is above
`16 + [+0x550] - [+0x644]`. The tile ids come from six 3-entry tables in the
REL's `.data` selected by `[+0x63C]`; the mode pair `[+0x644]`/`[+0x648]`
selects the top-row tile family and whether a top row is painted at all.
The vtable block `0x8080DF00..0x8080DF64` holds `0x80737270` at `0x8080DF44`
(slot `+0x44`) and this body at `0x8080DF54` (slot `+0x54`); `0x80737270` is the
measure half (it scans the same `+0xAC`/`+0xB0` position in 16-unit steps and
stores a tile count into `+0x64C`). The only model strings adjacent to that
vtable block are the `ice`
set used by the loader `0x80737F30` (`g3d/ice.brres`, `ice_A1`,
`docs/COVERAGE_WAVE30_BASES.md:128`), but the class's own name is not proven
here. The export's `void FUN_807373b0(int param_1)` is accurate.

### Signature

```rust
pub unsafe fn fn_807373b0(this: usize)
```

### Field map (offsets from `this`)

| offset | type | meaning |
|---:|---|---|
| `+0xAC` | `f32` | x position |
| `+0xB0` | `f32` | y position |
| `+0x550` | `f32` | bottom reference (inferred: run end / floor line) |
| `+0x63C` | `s32` | tile-set column, used as `*4` into each table (0..2) |
| `+0x644` | `f32` | start/compare height (inferred) |
| `+0x648` | `f32` | mode threshold (inferred); `== 0` selects the no-check arm |

Globals: `0x80429DD0` (`u32` manager pointer, `main.dol` BSS).
Tables (`u32` arrays; only the low 16 bits are used):

| table | address | entries |
|---|---|---|
| top, first column | `0x807CDB18` | `16, 52, 80` |
| top, second column | `0x807CDB24` | `17, 53, 81` |
| alt top, first column | `0x807CDB30` | `36, 72, 100` |
| alt top, second column | `0x807CDB3C` | `37, 73, 101` |
| strip, first column | `0x807CDB48` | `32, 68, 96` |
| strip, second column | `0x807CDB54` | `33, 69, 97` |

Scalars in the same block: `0x807CDAFC = 0.0f`, `0x807CDB00 = 16.0f`,
`0x807CDB04 = 24.0f` (not used here), `0x807CDB08 = 8.0f`,
`0x807CDAF8 = -16.0f` (not used here).

### Exact behaviour

Prologue `0x807373B0..0x80737414`: frame `0x80`; save LR, `f31/f30/f29/f28`
(each as `stfd` + `psq_st ...,qr0,0`), `r31,r30,r29,r28`; `r30 = this`;
`r31 = 0x807CDAF8`.
Epilogue `0x807376BC..0x807376F8`.

```rust
let x = read_f32(this, 0xAC) - 8.0;        // f31
let y = read_f32(this, 0xB0) - 8.0;        // f30
let idx = read_s32(this, 0x63C) * 4;

let draw = |pos_x: f32, pos_y: f32, tile_table: u32| {
    let id = (read_u32(tile_table + idx) & 0xFFFF) as u32;
    call5(0x80077860, read_u32(0x80429DD0),
          (pos_x as i32) as u32 & 0xFFFF,
          ((-pos_y) as i32) as u32 & 0xFFFF,
          0, id);
};
```

1. `0x80737414..0x807374B0` - `[+0x648] == 0.0` arm:
   `draw(x, y, 0x807CDB18); draw(16.0 + x, y, 0x807CDB24);` then step 5.
2. `0x807374B4..0x807374D8` - `[+0x648] != 0` and
   `[+0x644] == [+0x648]`:
   `if call3(0x80070BF0, x, y, 0) != 4 { return; }`
   then `draw(x, y, 0x807CDB18); draw(16.0 + x, y, 0x807CDB24);` and **return**
   (this arm does not run the strip).
3. `0x807374BC..0x80737570` - `[+0x644] != [+0x648]`:
   `draw(x, y, 0x807CDB30); draw(16.0 + x, y, 0x807CDB3C);` then step 5.
4. The two x integers are pre-computed once after the top-arm dispatch
   (`0x80737608..0x80737624`): `xi0 = (i32)x`, `xi1 = (i32)(16.0 + x)`.
5. `0x807376A4..0x807376B8` (bottom-tested loop):
   ```rust
   while y > 16.0 + (read_f32(this, 0x550) - read_f32(this, 0x644)) {
       y -= 16.0;                                   // 0x80737628
       call5(0x80077860, read_u32(0x80429DD0), xi0 & 0xFFFF,
             ((-y) as i32) as u32 & 0xFFFF, 0,
             read_u32(0x807CDB48 + idx) & 0xFFFF);
       call5(0x80077860, read_u32(0x80429DD0), xi1 & 0xFFFF,
             ((-y) as i32) as u32 & 0xFFFF, 0,
             read_u32(0x807CDB54 + idx) & 0xFFFF);
   }
   ```

The `0x80077860` call is the 2D tile blit: its export forwards
`(r3, r4, r5, r6 & 0xFF)` to `0x80077520` and uses `r7` as the tile id, with
`x >> 4`, `y >> 4` as tile coordinates - i.e. the arguments are 16.4
fixed-point. `0x80070BF0(x, y, 0)` is the map probe (`0x80070C60`); `4` is the
"empty/air" tile class that suppresses the special arm.

### Constants and callees

| target | shape |
|---|---|
| `0x80077860` | `fn_80077860` (2D tile blit) |
| `0x80070BF0` | `fn_80070bf0` (map probe, `x`, `y`, layer `0`) |
| `0x80429DD0` | manager pointer, read once per blit |

**Confidence: high** for the instruction stream and every constant; **medium**
for the roles of `+0x550`/`+0x644`/`+0x648`, which are read-only here and are
labelled "inferred" in the field map. The former wave-30/wave-101 rejection
reason (paired-single failure) is retired: the four `unaff_GQR0` brackets are
`qr0` register saves only.

> **Index range.** Each table is three `u32` entries. `[+0x63C]` is used as an
> unclamped `*4` byte offset, so the code expects `0..2`; `3..5` read into the
> adjacent table and `6+` read the scalar area (`0.0f`, `16.0f`, ...). Do not
> add a clamp - the shipped body has none.

---

## 3. `0x80705C60` - `obj_nut` post-load spawn/init

**Evidence.** `work/ghidra/rel-d_basesNP/export/80705c60_FUN_80705c60.c`,
840 bytes, 210 words, synthetic `0x80705C60`. Body
`0x80705C60..0x80705FA4` (ends `blr`).

**What it is.** The post-load initialiser of the `obj_nut` actor. It calls the
already-translated model loader `fn_80705fb0`
(`g3d/obj_nut.brres`, `docs/COVERAGE_WAVE29_BASES.md:182`), sizes and centres
the nut block from the bitfield at `+0x04`, installs the collision record
`+0x148` with callback `0x80706280`, writes the pose at `+0x164..0x170`, creates
one motion record (three words at `+0x41C` or four at `+0x428`), copies the
class word into the twelve records at `+0x43C`, starts the animator clip
`0x8082F360`, and returns `1`.

### Signature

```rust
pub unsafe fn fn_80705c60(this: usize) -> u32
```

`r3 = this`; the function always returns `1` (`li r3,1` at `0x80705F90`).

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x04` | `u32` | bitfield: bits 0-2 = width index `n`; bit 4 = height flag `m` |
| `+0xAC`,`+0xB0`,`+0xB4` | `f32` | spawn position (x, y, z); x/y are re-centred here |
| `+0xDC`,`+0xE0`,`+0xE4` | `f32` | all set to `1.0` |
| `+0x148` | opaque | collision record (`0x8008C3E0` fills `+0x1C..+0x3C`) |
| `+0x164`..`+0x170` | `f32` | pose: `0.0`, `12.0 * [+0xE0]`, `0.5 * [+0x404]`, `2.0` |
| `+0x1EC` | opaque | motion object handed to `0x8006D090` |
| `+0x318`,`+0x31C` | `f32` | half extents `0.5 * 16 * (n+4)` and `0.5 * 16 * (m+1)` |
| `+0x320`,`+0x324` | `f32` | `0.0`, `0.0` |
| `+0x338`,`+0x33C` | `f32` | `[0x8042BB0C]`, `[0x8042BB0C]` |
| `+0x340`,`+0x344` | `f32` | `[0x8042BB08] + 160.0` (both) |
| `+0x394` | opaque | NW4R animator; slot `+0x18` starts a clip |
| `+0x404`,`+0x408` | `f32` | full extents `16*(n+4)` and `16*(m+1)` |
| `+0x410`..`+0x418` | `f32` | centred position copy |
| `+0x41C`,`+0x420`,`+0x424` | `u32`/`s32` | motion record A (3 words) |
| `+0x428`..`+0x434` | `u32`/`s32` | motion record B (4 words) |
| `+0x43C` | `[u32; 36]` | twelve 12-byte records, each `= [this+0]` |

### Exact behaviour

Prologue `0x80705C60..0x80705C8C`: frame `0x40`; save LR, `r31,r30`;
`r30 = this`; `r31 = 0x807CC648`; the `0x43300000` high word for the two
unsigned-int-to-double conversions is parked at `24(r1)` and `32(r1)`;
`call1(0x80705FB0, this)`.

```rust
let v = read_u32(this, 0x04);
let n = (v & 7) as u32;              // 0..7
let m = ((v >> 4) & 1) as u32;       // 0/1

let w = 16.0 * (n as f32 + 4.0);                  // f11
let h = 16.0 * (m as f32 + 1.0);                  // f10
let hw = 0.5 * w;                                 // f8
let hh = 0.5 * h;                                 // f6

let x = read_f32(this, 0xAC) + hw;
let y = read_f32(this, 0xB0) - hh;
let z = read_f32(this, 0xB4);
```

1. `0x80705CD4..0x80705D3C`: store `1.0` to `+0xDC,+0xE0,+0xE4`; `w` to
   `+0x404`; `h` to `+0x408`; `x` to `+0xAC` and `+0x410`; `y` to `+0xB0` and
   `+0x414`; `z` to `+0x418`.
2. `0x80705D40..0x80705D6C`:
   `+0x338 = +0x33C = [0x8042BB0C]`;
   `+0x340 = +0x344 = [0x8042BB08] + 160.0`;
   `+0x320 = +0x324 = 0.0`;
   `+0x318 = hw`; `+0x31C = hh`.
3. `0x80705D70`: `call3(0x8008C3E0, this+0x148, this, 0x807CC648)`. The
   template at `0x807CC648` is
   `{0, 6.0f, 0, 2.0f, 0x03000000, 7, 0xFFBAFFFE, 0, 0x80706280}`; the callee
   copies `w0..w3` to `+0x1C..+0x28`, `byte(w4)` (= `0x03`) to `+0x2C`,
   `byte(w4 >> 8)` (= `0x00`) to `+0x2D`, `w5` to `+0x30`, `w6` to `+0x34`,
   the low half of `w7` to `+0x38`, and `w8` (= `0x80706280`, the touch
   callback) to `+0x3C`. (The callee reads template bytes `+0x10` and `+0x11`,
   i.e. the big-endian high two bytes of `w4`.)
4. `0x80705D78`: `call1(0x8008C330, this+0x148)` - register the record in the
   collision list.
5. `0x80705D7C..0x80705DB4`: `+0x164 = 0.0`;
   `+0x168 = 12.0 * [+0xE0]`; `+0x16C = 0.5 * [+0x404]`; `+0x170 = 2.0`.
6. `0x80705DB4..0x80705F68` - select the motion record on bit 4
   (`(v >> 4) & 1`):
   * **bit 4 == 0** (`0x80705DB8..0x80705E0C`):
     `+0x41C = 0; +0x424 = 0;`
     `+0x420 = (s32)(0.0f * [+0xDC])`; the operands are
     `(f32)(2^52 + 0x38000) - (f32)2^52`, which rounds to `0` in single
     precision, so `+0x420 = 0`.
     `call5(0x8006D090, this+0x1EC, this, 0, 0, this+0x41C)`.
   * **bit 4 == 1** (`0x80705E10..0x80705E80`):
     `+0x428 = 0xFFFF4001; +0x42C = 0x0000C000; +0x430 = 0xFFFF4000;`
     `+0x434 = (s32)(((f32)(2^52 + 0x38000) - (f32)(1.5 * 2^52)) * [+0xDC])`;
     the bracket is exactly `-2^51` in single precision and `[+0xDC]` was just
     set to `1.0`, so this store is `0x80000000` (`fctiwz` saturates). This is
     what the shipped bytes compute; reproduce the expression, do not "correct"
     it to a fixed-point value.
     `call5(0x8006D090, this+0x1EC, this, 0, 0, this+0x428)`.
   * The `uVar14 == 2` and `== 3` arms at `0x80705E84..0x80705F68` are **dead**:
     the selector is `rlwinm r0,r0,28,31,31`, i.e. one bit, so it can only be
     `0` or `1`. They decode cleanly (and are listed here so nobody thinks the
     extent is short), but no input reaches them.
7. `0x80705F68`: `call1(0x807061F0, this)` - writes `[this+0]` into twelve
   12-byte records at `+0x43C + i*12`.
8. `0x80705F70..0x80705F88`: `vcall(this+0x394, +0x18)(this+0x394,
   0x8082F360)` - start the `obj_nut` idle clip (`0x8082F360` is a BSS
   descriptor cell, filled at run time).
9. `0x80705F8C..0x80705FA4`: restore and `return 1`.

### Constants read out of the image

| address | raw | value | use |
|---|---|---|---|
| `0x807CC648` | template | see step 3 | collision record |
| `0x8042BB08` | BSS | `f32` world value | `+0x340/+0x344` |
| `0x8042BB0C` | BSS | `f32` world value | `+0x338/+0x33C` |
| `0x807CC6B4` | `0x41800000` | `16.0f` | extent scale |
| `0x807CC6B8` | `0x3F800000` | `1.0f` | `+0xDC/+0xE0/+0xE4` |
| `0x807CC6BC` | `0x3F000000` | `0.5f` | half extents |
| `0x807CC6C0` | `0x43200000` | `160.0f` | `+0x340/+0x344` bias |
| `0x807CC6C4` | `0x00000000` | `0.0f` | `+0x164`, `+0x320/+0x324` |
| `0x807CC6C8` | `0x41400000` | `12.0f` | `+0x168` |
| `0x807CC6CC` | `0x40000000` | `2.0f` | `+0x170` |
| `0x807CC6D0` | `0x4330000000000000` | `2^52` | arm-A conversion |
| `0x807CC6D8` | `0x4330000080000000` | `1.5 * 2^52` | arm-B conversion |
| `0x8082F360` | BSS | clip descriptor | animator slot `+0x18` |

**Confidence: medium.** Every instruction, constant and call is literal; the
meaning of the three-word record at `+0x41C` and the four-word record at
`+0x428` is not proven (they are passed whole to `0x8006D090`), and the
`-2^51` store is an oddity of the shipped sequence rather than a designed
fixed-point field. Implement it literally.

---

## 4. `0x80782F60` - `teresa` model and animation loader

**Evidence.** `work/ghidra/rel-d_basesNP/export/80782f60_FUN_80782f60.c`,
836 bytes, 209 words, synthetic `0x80782F60`. Body
`0x80782F60..0x807832A0` (ends `blr`).

**What it is.** The model/animation loader of the bases-side `teresa` actor. It
mounts `g3d/teresa.brres` (root `teresa`), creates three 0x40-byte model panes
(`teresaA`, `teresaA_otherA`, `teresaB`, priority `0x12B`), creates up to three
animation objects per model (`wait`, `wait_otherA1`, `shay_teresaB` for the
chr/clr/tex-pattern classes), attaches them back to the model panes, sets the
frame-0 state and parents the model block. The export's
`void FUN_80782f60(void)` lost the receiver; the raw `_savegpr` thunk
`0x802DCD54` preserves `r3`, so `this` is the incoming `r3`.

### Signature

```rust
pub unsafe fn fn_80782f60(this: usize)
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x188` | opaque | parent transform node passed to `0x8016A000` |
| `+0x190` | layout object | NW4R layout initialised by `0x800690C0(..., -1, heap, 0, 0x20)` |
| `+0x1AC` | `u32` | mounted `teresa` archive |
| `+0x1B0` | 3 x `0x40` | model panes (`0x80169D90` / `0x800F2A30`) |
| `+0x1F0` | opaque | second model block (`0x8016A000`) |
| `+0x270` | 3 x `0x38` | chr animation objects (`0x801650D0` / `0x80166340` / `0x80166310`) |
| `+0x2A4` | `u8` at stride `0x38` | per-chr byte at `+0x2A4 + i*0x38`, `= [0x807CF85C + i*4]` (all 0) |
| `+0x2E0` | opaque | chr animation base used by the final frame seed |
| `+0x308` | `f32` | frame value; the final seed is `[+0x308] - 1.0` |
| `+0x318` | 3 x `0x2C` | colour (clr) animations (`0x80165C30`) |
| `+0x39C` | 3 x `0x2C` | texture-pattern animations (`0x80166830`) |
| `+0x3F4` | opaque | second animation object (`0x80166F60` / `0x80166EF0`) |

### Exact behaviour

Prologue `0x80782F60..0x80782F90`: frame `0x60`; `_savegpr_19` at
`0x802DCD54` (saves `r19-r31`); `r30 = this`; the layout at `+0x190`
is initialised with `call5(0x800690C0, this+0x190, -1, [0x80377C48], 0, 0x20)`.

1. `0x80782FA4..0x80782FC0`:
   `+0x1AC = call3(0x800DF180, [0x8042A038]+4, "teresa", "g3d/teresa.brres")`.
2. `0x80782FC0..0x80783018` - three model panes, `i = 0..2`:
   ```rust
   let pane = call2(0x80239E30, this + 0x1AC, PANE[i]);   // 0x80782FD8
   call6(0x80169D90, this + 0x1B0 + i*0x40, &pane,
         this + 0x190, 0x12B, 1, 0);                       // 0x80782FF8
   call1(0x800F2A30, this + 0x1B0 + i*0x40);               // 0x80783000
   ```
   `PANE[0..2] = {0x80818808 "teresaA", 0x80818810 "teresaA_otherA",
   0x80818820 "teresaB"}` (pointer table `0x80818828`).
3. `0x80783018..0x80783168` - per model, three animation classes,
   `ANIM[i] = {0x80818834 "wait", 0x8081883C "wait_otherA1",
   0x8081884C "shay_teresaB"}` (pointer table `0x8081885C`):
   ```rust
   let pane = call2(0x80239E30, this+0x1AC, PANE[i]);
   if call2(0x8023A0B0, this+0x1AC, ANIM[i]) != 0 {         // chr
       call5(0x801650D0, this+0x270+i*0x38, &pane, &anim,
             this+0x190, 0);
       write_u8(this, 0x2A4 + i*0x38, read_u32(0x807CF85C + i*4) as u8);
       0x801680E0(f1 = 1.0, r3 = this+0x270+i*0x38);        // 0x8078308C
   }
   if call2(0x8023A190, this+0x1AC, ANIM[i]) != 0 {         // clr
       call6(0x80165C30, this+0x318+i*0x2C, &pane, &anim,
             this+0x190, 0, 1);
       call3(0x80166340, this+0x318+i*0x2C, read_u32(0x807CF85C+i*4), 0);
       call3f(0x80166310, this+0x318+i*0x2C, 0, 1.0);
   }
   if call2(0x8023A200, this+0x1AC, ANIM[i]) != 0 {         // tex pattern
       call6(0x80166830, this+0x39C+i*0x2C, &pane, &anim,
             this+0x190, 0, 1);
       call3(0x80166F40, this+0x39C+i*0x2C, read_u32(0x807CF85C+i*4), 0);
       call3f(0x80166F10, this+0x39C+i*0x2C, 0, 1.0);
   }
   ```
   `[0x807CF85C + i*4]` is zero for `i = 0..2` (read out of the image), so the
   three `+0x2A4` bytes and the three `index` arguments are all `0`.
4. `0x80783168..0x80783218` - attach each animation to its model pane:
   `INDEX[i] = [0x807CF868 + i*4] = 0` for `i = 0..2`:
   ```rust
   if call2(0x8023A190, this+0x1AC, ANIM[i]) != 0 {
       0x80169F80(f1 = 0.0, r3 = this+0x1B0+i*0x40,
                  r4 = this + INDEX[i]*0x2C + 0x318);
   }
   if call2(0x8023A0B0, this+0x1AC, ANIM[i]) != 0 {
       0x80169F80(f1 = 0.0, r3 = this+0x1B0+i*0x40,
                  r4 = this + INDEX[i]*0x38 + 0x270);
   }
   if call2(0x8023A200, this+0x1AC, ANIM[i]) != 0 {
       0x80169F80(f1 = 0.0, r3 = this+0x1B0+i*0x40,
                  r4 = this + INDEX[i]*0x2C + 0x39C);
   }
   ```
5. `0x8078321C..0x8078323C`:
   `0x80168FE0(f1 = read_f32(this,0x308) - 1.0, r3 = this+0x2E0)`;
   `0x801680E0(f1 = 0.0, r3 = this+0x2E0)`.
6. `0x8078323C..0x80783268`:
   `call2(0x80166F60, this+0x3F4, 0)` returns `f1`;
   `call3f(0x80166EF0, this+0x3F4, 0, f1 - 1.0)`;
   `call3f(0x80166F10, this+0x3F4, 0, 0.0)`.
7. `0x8078326C..0x80783288`:
   `call2(0x8016A000, this+0x1F0, this+0x188)`;
   `call2(0x8016A000, this+0x1B0, this+0x188)`;
   `call1(0x800690E0, this+0x190)`.
8. Epilogue `0x8078328C..0x807832A0` (`_restgpr_19` at `0x802DCDA0`).

### Constants and names

| address | value |
|---|---|
| `0x80818808` | `"teresaA"` |
| `0x80818810` | `"teresaA_otherA"` |
| `0x80818820` | `"teresaB"` |
| `0x80818828` | `{0x80818808, 0x80818810, 0x80818820}` |
| `0x80818834` | `"wait"` |
| `0x8081883C` | `"wait_otherA1"` |
| `0x8081884C` | `"shay_teresaB"` |
| `0x8081885C` | `{0x80818834, 0x8081883C, 0x8081884C}` |
| `0x80818868` | `"g3d/teresa.brres"` |
| `0x8081887C` | `"teresa"` |
| `0x807CF830` | `0.0f` |
| `0x807CF85C` | `{0, 0, 0}` |
| `0x807CF868` | `{0, 0, 0}` |
| `0x807CF874` | `1.0f` |

Callees: `0x800DF180`, `0x80239E30`, `0x8023A0B0`, `0x8023A190`,
`0x8023A200`, `0x801650D0`, `0x80165C30`, `0x80166830`, `0x80166340`,
`0x80166310`, `0x80166F40`, `0x80166F10`, `0x80169F80`, `0x801680E0`,
`0x80168FE0`, `0x80166F60` (unexported gap, 0x10 after `0x80166F40`),
`0x80166EF0`, `0x8016A000`, `0x800690C0`, `0x800690E0`, `0x800F2A30`
(unexported `main.dol`; named by address, called with the model block only).

**Confidence: high.** Every pane/animation name, priority, stride and index
table is read out of the image; the only unusual step is the
`0x80166F60 -> 0x80166EF0(f1 - 1.0)` frame seed, which is literal.

---

## 5. `0x80775190` - route/camera-key advance for the next record

**Evidence.** `work/ghidra/rel-d_basesNP/export/80775190_FUN_80775190.c`,
820 bytes, 205 words, synthetic `0x80775190`. Body
`0x80775190..0x807754C0` (ends `blr`).

**What it is.** The "install the next route record" sibling of the world-map
route player `0x80773210` (`docs/SPEC_WAVE140_BASES.md:1306`, state 5 calls it
as `0x80775190(this)`). It reads the 10-byte record name at
`+0x3180 + [+0x3224]*10`, scans its trailing decimal digits, appends the
matching camera keys to the track at `+0x1A0 + [+0x338C]*0xBF8`, walks forward
through the record table while the next names share the same alphabetic prefix,
decrements the index, and finally submits effect id `30` at the midpoint of the
two route vectors when the record name is `"Rstone"`.

### Signature

```rust
pub unsafe fn fn_80775190(this: usize)
```

`r3` is the route player (recovered through `_savegpr_26` at `0x802DCD70`).

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x1A0` | camera track | base; record `+ [+0x338C] * 0xBF8` |
| `+0x3180` | `[u8; 10*n]` | record-name table, stride 10 |
| `+0x3220` | `s32` | record capacity |
| `+0x3224` | `s32` | current record index |
| `+0x338C` | `s32` | mode/route index (selects the 0xBF8 track) |

Globals: `0x8042A27C` (`u32` effect-manager pointer), `0x80817248`
(`"Rstone"`, 6 bytes), `0x807CF38C = 1.0f`, `0x807CF390 = 0.0f`,
`0x807CF3B0 = -1.0f`, `0x807CF458 = 0.5f`, `0x807CF388 = -0.3628f` (not used
here).

### Exact behaviour

Prologue `0x80775190..0x807751A4`: frame `0x50`; `_savegpr_26` (saves
`r26-r31`); `r26 = this`; `r27 = [+0x3224]`; `r30 = this + r27*10 + 0x3180`.

1. `0x807751C0..0x80775218` - digit scan:
   ```rust
   let len = strlen(src);                 // 0x802DC69C
   let mut k = len - 1;
   let mut prefix_end = len;              // r29, overwritten as below
   while k >= 0 {
       let c = read_u8(src, k);
       if (c as u32).wrapping_sub(0x30) > 9 { prefix_end = k; break; }
       k -= 1;
   }
   ```
   So `prefix_end` is the index of the last non-digit character (the scan stops
   on the first non-digit before the trailing digit run). `0x802E1BD0` is
   `strncmp` and **compares `n + 1` bytes**, so passing `prefix_end` compares
   the whole alphabetic prefix including its last character.
2. `0x80775218..0x8077523C`:
   `0x800F0090(r3 = track, r4 = src, f1 = 1.0, f2 = 0.0)`; if the return in
   `r3` is nonzero, go to step 6 (the callee is declared `void` in the export
   but returns a status in `r3`; nonzero means "this key already exists").
3. `0x80775240..0x807752FC` - create the keys for the current name:
   ```rust
   let mut made: i32 = 0;
   if call2(0x800F0CC0, track, src) != 0 {
       call4f(0x800F0630, track, src, 1.0, -1.0);
       made = 1;
   }
   if call2(0x800F0C10, track, src) != 0 {
       call4f(0x800F09E0, track, src, 1.0, 0.0);
       made += 1;
   }
   ```
4. `0x80775300..0x80775304`: if `made <= 0` go to step 6.
5. `0x807753E0..0x807753E8` then `0x80775308..0x8077532C` - walk the table;
   `i = r27` (the initial index) is the loop counter, `[+0x3220]` is the bound:
   ```rust
   while i < read_s32(this, 0x3220) {
       let j = read_s32(this, 0x3224) + 1;
       write_s32(this, 0x3224, j);
       let dst = this + j*10 + 0x3180;
       if strncmp(dst, src, prefix_end) != 0 { break; }
       if call2(0x800F0CC0, track, dst) != 0 {
           call4f(0x800F0630, track, dst, 1.0, -1.0);
       }
       if call2(0x800F0C10, track, dst) != 0 {
           call4f(0x800F09E0, track, dst, 1.0, 0.0);
       }
       i += 1;
   }
   let u = read_s32(this, 0x3224);
   write_s32(this, 0x3224, if u > 0 { u - 1 } else { 0 });   // 0x807753EC
   ```
   (`0x8077532C` calls `0x802E1BD0`; `0x80775338..0x807753D8` re-issues the two
   key families for each newly entered name.)
6. `0x80775408..0x80775428` - the `"Rstone"` tail, always executed:
   ```rust
   if strncmp(this + read_s32(this,0x3224)*10 + 0x3180, "Rstone", 6) != 0 {
       return;
   }
   ```
7. `0x80775430..0x807754A8`:
   ```rust
   let mut a = [0.0f; 3]; let mut b = [0.0f; 3];
   call2(0x80773BB0, &a, this);          // translated
   call2(0x80773CA0, &b, this);          // translated
   let mid = [ (a[0]+b[0])*0.5, (a[1]+b[1])*0.5, (a[2]+b[2])*0.5 ];
   call4(0x80105240, read_u32(0x8042A27C), 30, &mid, 1);
   ```

### Callees

| target | name | shape |
|---|---|---|
| `0x802DC69C` | `FUN_802dc69c` | `(str) -> i32` `strlen` |
| `0x802E1BD0` | `FUN_802e1bd0` | `(a, b, n) -> i32` `strncmp`, compares `n+1` bytes |
| `0x800F0090` | unexported | `(track, name, f1=1.0, f2=0.0) -> u32` key exists |
| `0x800F0CC0` | unexported | `(track, name) -> u32` find empty key slot |
| `0x800F0630` | unexported | `(track, name, f1=1.0, f2=-1.0)` create key |
| `0x800F0C10` | unexported | `(track, name) -> u32` find empty anim slot |
| `0x800F09E0` | unexported | `(track, name, f1=1.0, f2=0.0)` create anim |
| `0x80773BB0` | `fn_80773bb0` | `(out, this)` route vector |
| `0x80773CA0` | `fn_80773ca0` | `(out, this)` camera/angle vector |
| `0x80105240` | `wm_effect_record_submit` | `(mgr, id, &vec, 1)` |

**Confidence: medium.** Every store, index step and call shape is literal and
the `strncmp` `n+1` behaviour was read from the callee's export. The residual
uncertainty is only the semantic naming of the five `0x800F0xxx` camera-track
helpers (they are named by address; their return register use was verified from
the raw `r3` tests).

---

## 6. `0x8079A930` - course-route node toggle with effect layers

**Evidence.** `work/ghidra/rel-d_basesNP/export/8079a930_FUN_8079a930.c`,
816 bytes, 204 words, synthetic `0x8079A930`. Body
`0x8079A930..0x8079AC5C` (ends `blr`).

**What it is.** A member of the `0x807995A0..0x8079A870` course-tree query
family (`docs/COVERAGE_WAVE107_BASES.md:41-52`). It toggles the per-node flag
mask of a course route, finds the next sibling that shares the same course
selector, and mirrors the node's world position into the two effect layers of
the object at `+0x668`. It is the "enable/disable a course node" body.

### Signature

```rust
pub unsafe fn fn_8079a930(this: usize, course: u32, mask: u32)
```

`r3 = this`, `r4 = course` (passed as the fourth argument of
`0x80799A50` and the third of `0x80799B00`), `r5 = mask`. Recovered from
`_savegpr_22` at `0x802DCD58`.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x22C` | `s32` | route node index |
| `+0x294` | `u8` | effect latch (set to 1 while the bit is on) |
| `+0x29C` | `u32` | ability/flag sub-object: word `+0x70` (mask), byte `+0x27C` per layer |
| `+0x668` | `u32` | effect object passed to the `0x80C56xxx` helpers |

Globals: `0x8042A18C` (`u32` scene manager). Route data lives at
`scene + [scene+0x338C]*0x16518 + 0x3394`; each route node is `0x1C0` bytes
with the name at `+0x18` and the flag word at `+0x1C8`.

### Exact behaviour

Prologue `0x8079A930..0x8079A940`: frame `0xB0`; `_savegpr_22` (`r22-r31`);
`r26 = this`; `r27 = course`; `r28 = mask`; `r30 = [0x8042A18C]`.

```rust
let bit = call2(0x80C56B20, read_u32(this, 0x29C), mask);   // 0x8079A960
let self_bit = (read_u32(read_u32(this,0x29C), 0x70) & bit) != 0;
let route = scene + read_s32(scene, 0x338C)*0x16518 + 0x3394;
let node_on = call3(0x800F57A0, route, read_s32(this,0x22C), mask) != 0;
if !node_on && !self_bit { goto tail_clear; }               // 0x8079A9B4
```

1. `0x8079A9B8`: `let alt = call3(0x80799B00, this,
   read_s32(this,0x22C), course);` - the alternate sibling search.
2. `0x8079A9CC..0x8079AA64` - positions:
   ```rust
   let node = read_s32(this, 0x22C);
   let mut p = [0.0f; 3];
   call3(0x80100200, &p, scene, node);
   if self_bit {
       let layer = call2(0x80C56B90, read_u32(this,0x668), mask);
       call3(0x80C56600, read_u32(this,0x668), layer, &p);      // second layer
       let mut q = [0.0f; 3];
       call3(0x80100200, &q, scene, node);                      // same vector
       let layer = call2(0x80C56B90, read_u32(this,0x668), mask);
       call3(0x80C565A0, read_u32(this,0x668), layer, &q);      // first layer
   } else {
       let layer = call2(0x80C56B90, read_u32(this,0x668), mask);
       call3(0x80C565A0, read_u32(this,0x668), layer, &p);
   }
   ```
3. `0x8079AAA8..0x8079AAC4`: `let mut next = call4(0x80799A50, this, node,
   node, course);` if `next < 0` go to tail.
4. `0x8079AAD0..0x8079AB44` - sibling walk:
   ```rust
   let mut prev = node;
   loop {
       let p = call2(0x800F56B0, route, next);   // node + 0x18 (name pointer)
       let kind = call1(0x800F9970, p);          // 0x8079AB14; reads *p
       if kind != 1 { break; }                   // 1 = 'K' prefix
       if next == alt { goto tail; }
       let n2 = call4(0x80799A50, this, prev, next, course);
       prev = next; next = n2;
       if next < 0 { goto tail; }
   }
   if call3(0x800F57A0, route, next, mask) != 0 {
       write_u8(this, 0x294, 1);
       call3(0x80C561E0, read_u32(this,0x29C), bit, 1);
       enabled = true;
   } else { enabled = false; }
   ```
5. `0x8079AB78..0x8079AC4C` - tail (`enabled == false` branch):
   ```rust
   if !enabled {
       let layer = call2(0x80C56B90, read_u32(this,0x29C), mask);
       if read_u8(read_u32(this,0x29C), layer + 0x27C) != 0 {
           let p = [0.0f;3]; call3(0x80100200, &p, scene, node);
           let l2 = call2(0x80C56B90, read_u32(this,0x29C), mask);
           call3(0x80C56600, read_u32(this,0x29C), l2, &p);
       }
       if self_bit {
           let q = [0.0f;3]; call3(0x80100200, &q, scene, node);
           let l3 = call2(0x80C56B90, read_u32(this,0x29C), mask);
           call3(0x80C56600, read_u32(this,0x29C), l3, &q);
       }
       write_u8(this, 0x294, 0);
       call3(0x80C561E0, read_u32(this,0x29C), bit, 0);
   }
   ```
6. Epilogue `0x8079AC48..0x8079AC5C` (`_restgpr_22` at `0x802DCDA4`).

### Callees

| target | name | shape |
|---|---|---|
| `0x80C56B20` | `FUN_80c56b20` (`game::bosses`) | `(obj, mask) -> u32` mask -> layer bit (`0x40/0x20/1/0x10/0x80/0x100`) |
| `0x80C56B90` | `FUN_80c56b90` (`game::bosses`) | `(obj[, mask]) -> u32` bit -> layer index `0..8` |
| `0x80C56600` | unexported `d_en_bossNP` | `(obj, layer, &vec)` second effect layer |
| `0x80C565A0` | unexported `d_en_bossNP` | `(obj, layer, &vec)` first effect layer |
| `0x80C561E0` | `FUN_80c561e0` (`game::bosses`) | `(obj, mask, on)` set/clear bit in `+0x70`, `+0x150` latch |
| `0x800F57A0` | unexported `main.dol` | `(route, node, mask) -> u32` `mask & [node*0x1C0 + 0x1C8]` |
| `0x800F56B0` | unexported `main.dol` | `(route, node) -> ptr` `route + node*0x1C0 + 0x18` |
| `0x800F9970` | unexported `main.dol` | `(name) -> u32` `'F'->0, 'K'->1, "WS"->2, W?->3, else 4` |
| `0x80799A50` | `fn_80799a50` | `(this, node, node, course) -> s32` sibling scan |
| `0x80799B00` | `FUN_80799b00` | `(this, node, course) -> s32` alternate sibling |
| `0x80100200` | unexported `main.dol` | `(&out, scene, node)` world position |

**Confidence: medium.** The control flow, both mask tests and every argument
register are literal; the two `0x80C56600`/`0x80C565A0` setters and the
`+0x27C` byte are named by address and role-inferred (effect layer), not
proven.

---

## 7. `0x807A7220` - world-map cue/message picker

**Evidence.** `work/ghidra/rel-d_basesNP/export/807a7220_FUN_807a7220.c`,
816 bytes, 204 words, synthetic `0x807A7220`. Body
`0x807A7220..0x807A754C` (ends `blr`).

**What it is.** A one-shot initialiser + cue picker of the world-map message
object. It spawns the child actor `0x298` into `+0x1CC`, resets the four
16-byte message records at `+0x168`, clears the save/course latch, then
chooses one cue id from the world byte `0x8042A24D`, the save object
`[0x80429F7C]` field `+0x370`, the character group of the current actor and
the course node type, and posts it through `0x801016B0` unless the chosen id
is `0x35`.

### Signature

```rust
pub unsafe fn fn_807a7220(this: usize)
```

`r3 = this`, recovered through `_savegpr_25` at `0x802DCD6C`.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x160` | `u32` | cleared to `0` |
| `+0x168` | 4 x `0x10` | message/cue records, each reset by `0x80101670` |
| `+0x1A8` | `s32` | set to `-1` |
| `+0x1AC`,`+0x1B0` | `u32` | cleared |
| `+0x1CC` | `u32` | child actor spawned as kind `0x298` |

Globals: `0x80429F7C` (`u32` save/course-state object; `+0x370` mode,
`+0x54` character word, `+0x44` course node), `0x8042A24D` (`u8` world byte),
`0x80429045` (`u8`), `0x8042A248` (`u8` gate), `0x807D0658 = -10000.0f`,
`0x807D065C = 0.0f`.

### Exact behaviour

Prologue `0x807A7220..0x807A7234`: frame `0x40`; `_savegpr_25`; `r30 = this`.

1. `0x807A7238..0x807A7250`:
   `+0x1CC = call5(0x800F2A00, 0x298, this, 0, 0, 0)`.
2. `0x807A7254..0x807A7270`: `for i in 0..4 { call1(0x80101670,
   this + 0x168 + i*0x10); }`.
3. `0x807A7274..0x807A7298`:
   `write_u8(read_u32(0x80429F7C), 0x394, 0); write_u32(this,0x160,0);
   write_s32(this,0x1A8,-1); write_u32(this,0x1AC,0);
   write_u32(this,0x1B0,0);`
4. `0x807A729C..0x807A72C4`:
   ```rust
   let save = read_u32(0x80429F7C);
   let r = call1(0x800FB2D0, read_u8(0x8042A24D));
   let world = read_u8(0x8042A24D);
   let mut who = read_u32(save, 0x54) & 0xFF;      // candidate actor id
   let mut flag = read_u8(0x80429045);             // r26
   let mut cue: u32 = 0x26;                        // default 38
   ```
5. `0x807A72C8..0x807A7364` - `if r == 0`:
   ```rust
   who = 0;                                        // 0x807A72D0
   if world == 0
      || (call0(0x800FC370) == 0 && call0(0x800FBF30) < world as i32) {
       who = call5(0x800F2A00, 0x284, this, 0,
                   &[-10000.0, 0.0, 0.0], 0);       // 0x807A7328
   }
   let mode = read_s32(save, 0x370);
   if who == 0 {
       if mode < 0 || mode > 1 { if mode == 3 { cue = 0x2E; } }
       else { cue = 0x2D; }
   } else {
       if mode < 0 || mode > 1 {
           if mode == 2 { cue = 0x17; } else if mode == 3 { cue = 0x18; }
       } else { cue = 0x15; }
       if world == 0 { cue = 0x34; }
   }
   ```
6. `0x807A73BC..0x807A7514` - `else` (r != 0):
   ```rust
   let mode = read_s32(save, 0x370);
   if mode == 3 { cue = 0x19; }
   else if mode == 1 { cue = 0x16; }
   else if read_u8(0x8042A248) != 0 { cue = 0x26; }
   else if flag == 0xFF || flag != world { cue = 0x26; }
   else {
       let group = call1(0x800F9840, who);
       if group != 0xB {
           let kind = call3(0x800FA9C0, flag, who, read_u32(save,0x44));
           cue = 0x26;
           if group == 5 {
               if ((kind - 2) as u32) <= 3 { cue = 2; }
               else if kind == 1 || kind == 6 { cue = 3; }
               else if kind == 7 { cue = 2; }
           } else if group != 7 {
               if call2(0x800FB1A0, flag, who) != 0
                  && call3(0x800FAF70, flag, who, read_u32(save,0x44)) != 0 {
                   cue = 0;
               } else if ((kind - 2) as u32) <= 2 { cue = 0; }
               else if kind == 1 || kind == 6 { cue = 1; }
               else if kind == 5 || kind == 7 { cue = 0; }
               else if kind == 0 { cue = 0x26; }
           }
       }
   }
   ```
7. `0x807A7518..0x807A7534`:
   `if cue != 0x35 { call5(0x801016B0, this, cue, 0, 0, 0x80); }`
8. Epilogue `0x807A7538..0x807A754C` (`_restgpr_25` at `0x802DCDB8`).

### Callees

| target | name | shape |
|---|---|---|
| `0x800F2A00` | unexported `main.dol` | `(kind, parent, 0, pos_or_0, 0) -> u32` actor spawn |
| `0x80101670` | unexported | `(&record)` reset one message record |
| `0x801016B0` | `fn_801016b0` | `(this, cue, 0, 0, 0x80) -> u32`; fills the first free record via `0x80101690` |
| `0x800FB2D0` | unexported | `(world byte) -> u32` gate |
| `0x800FC370` | unexported | `() -> u32` |
| `0x800FBF30` | `fn_800fbf30` | `() -> i32` last occupied player slot `0..7` |
| `0x800F9840` | `fn_800f9840` | `(id) -> u32` character group, `0xB` for negative/unknown |
| `0x800FA9C0` | `fn_800fa9c0` | `(flag, who, node) -> i32` node-kind classifier (0..9) |
| `0x800FB1A0` | `fn_800fb1a0` | `(flag, who)` |
| `0x800FAF70` | `fn_800faf70` | `(flag, who, node) -> u32` |

**Confidence: medium.** All 204 instructions decode and every cue id is a
literal immediate. The two `0x800FB2D0`/`0x800FC370` gates and the
`[0x80429F7C]+0x54/+0x44` meanings are named by address/role; no return value
is assumed beyond the `r3` tests visible in the raw stream. Note that
`r25` (the spawn result) and `r26` (the `0x80429045` byte, overwritten with the
world byte only inside the spawn test) are distinct registers - the Ghidra
export conflates them into one `uVar6`, so implement from the raw assignments,
not from the decompiler's variable names.

---

## 8. `0x807B84C0` - stage/world-map scene setup

**Evidence.** `work/ghidra/rel-d_basesNP/export/807b84c0_FUN_807b84c0.c`,
812 bytes, 203 words, synthetic `0x807B84C0`. Body
`0x807B84C0..0x807B87E8` (ends `blr`).

**What it is.** The setup half of the scene teardown
`fn_807b77e0` (`docs/SPEC_WAVE146_BASES.md` section 7). It seeds the scene
words, refreshes or releases the four player panes depending on
`0x80315B98`, publishes the stage frame offset, and allocates/constructs the
eight global owner slots that the teardown releases one by one.

### Signature

```rust
pub unsafe fn fn_807b84c0(this: usize)
```

`this` is the scene/context object (recovered through `_savegpr_27` at
`0x802DCD74`); it is read at `+0x120D`, `+0x120E` and `+0xC4` only.

### Field map (offsets from `this`)

| offset | type | meaning |
|---:|---|---|
| `+0xC4` | opaque | sub-object passed to `0x800D0A20` |
| `+0x120D` | `u8` | bookmark/area byte (also compared against `0x20..0x23`) |
| `+0x120E` | `u8` | stage-info index (0..?) |

Globals written:

| global | value |
|---|---|
| `0x80429010` | `6` |
| `0x8042900C` | `2` |
| `0x8042A200` | `0` |
| `0x8042A204` | `[0x80315B98]` (word) |
| `0x8042A224` | `byte[0x80315B97]` |
| `0x80429008` | `[0x80315B90]` (word) |
| `0x8042A226` | `0` (byte) |
| `0x8042A21D` | `0` (byte) |
| `0x80429F30` | new `0x100` block (if allocated) |
| `0x8042A078`, `0x8042A1D8` | new `0x6D0` block |
| `0x80429D30`, `0x8042A1DC` | new `0x300` block |
| `0x80429D48`, `0x8042A1E0` | new `0xBDC` block |
| `0x80429D88`, `0x8042A1E8` | new `0x24` block |
| `0x8042A1D0` | new `0x100` block |
| `0x8042A1D4` | result of `0x80081D20()` when `new(0x98) != 0`, else `0` |
| `0x8042A1E4` | result of `0x800D02D0()` when `new(1) != 0`, else `0` |
| `0x8042A1EC` | result of `0x8008DC80()` when `new(1) != 0`, else `0` |
| `0x80429F7C+0xB5A` | `0` (byte) |
| `0x8042A227` | `0` (byte) |

Read-only globals: `0x80429E98` (stage-info table base; `+0x2C` is the
validity word), `0x80429F7C` (save/course state; `+0xAF4` story word),
`0x80429F80` (flag word, bit 6 tested), `0x80377C48` (game heap handle),
`0x80315B90`/`0x80315B97`/`0x80315B98` (scene block), `0x8042A028` (four player
pane pointers, first word skipped), `0x8042A070` (pane object), `0x8042A038`
(archive manager).

### Exact behaviour

Prologue `0x807B84C0..0x807B84D4`: frame `0x20`; `_savegpr_27`; `r30 = this`.

1. `0x807B84D4..0x807B8500`:
   ```rust
   let idx = read_u8(this, 0x120E);
   let rec = read_u32(0x80429E98) + idx as u32 * 0x3B0;
   let stage = if read_u32(rec, 0x2C) != 0 { rec } else { 0 };
   ```
   (`rec + 0xC` is dereferenced at step 5 with no null guard; the shipped code
   assumes the stage record is valid.)
2. `0x807B8500..0x807B8540` - player panes:
   ```rust
   if read_u32(0x80315B98) == 0 {
       for i in 0..4 { call1(0x800DC990,
           read_u32(read_u32(0x8042A028) + 4 + i*4)); }
   } else {
       for i in 0..4 { call1(0x800DC9A0,
           read_u32(read_u32(0x8042A028) + 4 + i*4)); }
       call0(0x80069500);
   }
   ```
3. `0x807B8574..0x807B85D0` - scene words:
   ```rust
   write_u32(0x80429010, 6);  write_u32(0x8042900C, 2);
   write_u32(0x8042A200, 0);  write_u32(0x8042A204, read_u32(0x80315B98));
   write_u8 (0x8042A224, read_u8(0x80315B97));
   write_u32(0x80429008, read_u32(0x80315B90));
   write_u8 (0x8042A226, 0);  write_u8(0x8042A21D, 0);
   ```
4. `0x807B85D4..0x807B85E0`: `call0(0x8005EB10); call1(0x800CFC10,
   read_u32(0x80377C48));`.
5. `0x807B85E4..0x807B8658` - stage frame offset:
   ```rust
   let base = read_i16(read_u32(stage, 0xC), 10);
   let bookmark = read_u8(this, 0x120D);
   let mut frame = base;
   if read_s32(read_u32(0x80429F7C), 0xAF4) < 0
      && (bookmark < 0x20 || bookmark > 0x23) {
       frame = base + 200;
       let p = call0(0x8005FFB0) as i32;          // active player count 0..4
       if (((p ^ 1) >> 1) - ((p ^ 1) & p)) < 0 && (read_u32(0x80429F80) & 0x40) == 0 {
           frame = base + 300;                     // unreachable for p in 0..4
       }
   }
   call2(0x800E3870, read_u32(0x80377C48), frame);
   ```
6. `0x807B865C..0x807B867C`:
   `if 0x1F < bookmark && bookmark < 0x23 { write_u8(read_u32(0x8042A070),
   0x0A, 1); }` (as unsigned tests, i.e. `0x20, 0x21, 0x22`).
7. `0x807B867C..0x807B86FC` - allocations (each `0x802B9210(size)` is the
   archive heap allocator; the stored value is the block or the constructor's
   return):
   ```rust
   let p = call1(0x802B9210, 0x100);
   write_u32(0x8042A1D0, p); if p != 0 { write_u32(0x80429F30, p); }
   call0(0x800B0EA0);

   let a = call1(0x802B9210, 0x98);
   write_u32(0x8042A1D4, if a != 0 { call0(0x80081D20) } else { 0 });

   let b = call1(0x802B9210, 0x6D0);
   write_u32(0x8042A1D8, b); if b != 0 { write_u32(0x8042A078, b); }
   call1(0x800E3AE0, read_u32(0x8042A078));

   let c = call1(0x802B9210, 0x300);
   write_u32(0x8042A1DC, c); if c != 0 { write_u32(0x80429D30, c); }

   let d = call1(0x802B9210, 0xBDC);
   write_u32(0x8042A1E0, d); if d != 0 { write_u32(0x80429D48, d); }
   call1(0x80068040, read_u32(0x80429D48));

   let e = call1(0x802B9210, 1);
   write_u32(0x8042A1E4, if e != 0 { call0(0x800D02D0) } else { 0 });

   let f = call1(0x802B9210, 0x24);
   write_u32(0x8042A1E8, f); if f != 0 { write_u32(0x80429D88, f); }
   call1(0x8006C0B0, read_u32(0x80429D88));

   let g = call1(0x802B9210, 1);
   write_u32(0x8042A1EC, if g != 0 { call0(0x8008DC80) } else { 0 });
   ```
8. `0x807B87B4..0x807B87CC`:
   `call1(0x800D0A20, this+0xC4);`
   `write_u8(read_u32(0x80429F7C), 0xB5A, 0); write_u8(0x8042A227, 0);`
9. `0x807B87D0`: `call0(0x80069E50)`.
10. Epilogue `0x807B87D4..0x807B87E8` (`_restgpr_27` at `0x802DCDC0`).

### Callees

| target | name | shape |
|---|---|---|
| `0x800DC990` | `fn_800dc990` (`engine::display`) | `(pane)` create/attach |
| `0x800DC9A0` | `fn_800dc9a0` (`engine::display`) | `(pane)` release |
| `0x8005EB10`, `0x80069500`, `0x80069E50`, `0x800B0EA0` | unexported | `()` |
| `0x800CFC10` | `fn_800cfc10` (`game::stage`) | `(heap)` |
| `0x800E3870` | unexported | `(heap, i16 frame)` |
| `0x800E3AE0` | unexported | `(block)` |
| `0x80068040` | unexported | `(block)` |
| `0x8006C0B0` | unexported | `(block)` |
| `0x800D0A20` | unexported | `(&this+0xC4)` |
| `0x800D02D0`, `0x8008DC80`, `0x80081D20` | unexported / `create_bg_scene` | `() -> u32` |
| `0x802B9210` | `fn_802b9210` (`resource::archive`) | `(size) -> ptr` heap alloc |

**Confidence: medium.** Every store, size and call shape is literal, and the
eight owner slots match the teardown in `docs/SPEC_WAVE146_BASES.md` slot for
slot. The manager constructors `0x80081D20`/`0x800D02D0`/`0x8008DC80` are named
by address only (their bodies are not in this lane's scope), and `stage` is
assumed non-null exactly as the shipped code assumes it.

---

## Ready vs conditional

| address | verdict | residual work |
|---|---|---|
| `0x8069BE20` | ready (high) | none; share the `0x8069B750` tail shape. |
| `0x807373B0` | ready (high) | only the inferred names of `+0x550/+0x644/+0x648`; values/tables are literal. |
| `0x80705C60` | ready (medium) | the three-word record at `+0x41C` / four-word record at `+0x428` and the `-2^51` store are literal-but-unexplained; nothing is missing. |
| `0x80782F60` | ready (high) | none. |
| `0x80775190` | ready (medium) | five `0x800F0xxx` camera-track helpers are named by address; their return use is verified. |
| `0x8079A930` | ready (medium) | `0x80C56600`/`0x80C565A0` and `+0x27C` are role-inferred effect-layer pieces. |
| `0x807A7220` | ready (medium) | `0x800FB2D0`/`0x800FC370` are named by address; cue ids are literal. |
| `0x807B84C0` | ready (medium) | three manager constructors are named by address. |

**Split: 8 ready, 0 conditional.** No row is blocked by GQR3, a table
generator, a missing receiver or an undecodable word.

## Verification performed

* `work/ghidra/rel-d_basesNP/export/functions.tsv` was parsed read-only; the
  eight `size` values in the table above are its `size` column verbatim, and
  `decompiled` is `true` for all eight.
* Each `.c` export exists at the cited path; the file names are the exact
  lowercase forms on disk.
* The raw stream was decoded for all 1,657 words: 0 undecodable, exactly one
  `blr` per body at `address + size - 4`, and no unconditional branch out of
  any extent.
* `psq` audit: rows 1-2 use `qr0` save/restore only; rows 3-8 have no `psq` at
  all. No `qr3` or other quantiser appears in the eight.
* `ledger/functions.csv` (16,910 addresses) contains none of the eight.
* Every `#{1,6} ` heading in `docs/*.md` (1,510 addresses, `0x`-prefixed and
  bare) contains none of the eight. Several of them appear only in prose skip
  lists (`docs/SPEC_WAVE146_BASES.md:99-101`, `:1318`), which is exactly what
  made them candidates rather than claims.
* `src/game/bases.rs` contains no `pub unsafe fn fn_<addr>` for any of the
  eight.
* No Rust file, ledger CSV, existing document or tool was modified or run by
  this lane. The only artefact is `docs/SPEC_WAVE147_BASES.md`; the throwaway
  decoder lives under `%TEMP%\opencode\w147`.

## Hand-off to the Wave 148 spec lane

The next eight (same filter, current tree) are
`0x806DA520` (804), `0x8071E130` (788), `0x80788CB0` (788), `0x80763290` (780),
`0x80794C00` (780), `0x8075F270` (768), `0x8066DDC0` (764), `0x807727D0` (764).
`0x80775190` is now claimed, so `docs/SPEC_WAVE140_BASES.md`'s "conditional"
note about it is resolved. Of that state machine's remaining siblings,
`0x80774090` and `0x80775540` are already claimed; `0x807754D0` (108) and
`0x80775D00` (708) are still open.
