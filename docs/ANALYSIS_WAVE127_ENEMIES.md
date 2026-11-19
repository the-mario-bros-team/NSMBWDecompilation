# Analysis - Wave 127 (`game::enemies` skip-list reclassification)

## What this pass is

Wave 127 is a **read-only** reclassification lane. It adds no body, no ledger
row and no Rust edit. It revisits the addresses that waves 110-126 recorded as
skipped in `docs/COVERAGE_WAVE110..126_ENEMIES.md`, re-reads the raw
instruction stream in `work/rels/elf/d_enemiesNP.elf`, and says whether the
address is reconstructible now, needs one bounded extra step, or is genuinely
unreconstructable.

Nothing here overrides a coverage doc: every earlier skip stays a skip. This
page records what the **bytes** show, so a later wave can claim the B rows and
promote the C rows after the stated step.

## Scope and how it was collected

* The `## Deliberate skips` sections of `COVERAGE_WAVE110_ENEMIES.md` through
  `COVERAGE_WAVE126_ENEMIES.md` were parsed for every `0x80Axxxxx` /
  `0x80Bxxxxx` address. That is **411** distinct addresses.
* 27 were dropped: either already in the ledger (e.g. `0x80AFEFD0`,
  `0x80A7AB30`) or data words quoted by the docs (`0x80B328B0`,
  `0x80B72D68`, `0x80352CB4`).
* The remaining **384** are absent from `ledger/functions.csv` - checked
  against the `address` column of all 15,964 rows, read-only - and present in
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`. They are the rows below;
  `bytes` is the `functions.tsv` size (106,516 bytes total).

## The address -> file mapping needed to read the stream

`work/rels/elf/d_enemiesNP.elf` has six `PT_LOAD` headers and no section
table. The first segment starts at **file offset `0x10F0`** for
**`0x80A000F0`**, so the real mapping is

```
file offset = vaddr - 0x80A00000 + 0x1000
```

The same +0x1000 shift holds for every segment (`0x80B32DC0` lives at
`0x133DC0`). Using `vaddr - 0x80A00000` alone reads 4 KB early and produces
disassembly that looks plausible but belongs to a different function; every row
below was re-read with the corrected mapping and cross-checked against its Ghidra
export.

Second correction: a `functions.tsv` size is a **bucket extent**, not a proven
body length. The table keeps those extents (they are what the earlier docs
quote), while the notes describe the body reached by walking from the entry
through conditional branches and linked calls until every path ends in `blr`, a
tail `b`, or a tail call.

## Classes

* **B - reconstructible now from the raw stream.** The body reachable from the
  entry decodes end-to-end; every path terminates in `blr` or a tail call to a
  real function entry; no path lands on data; nothing depends on state the
  module does not contain (no `GQR1..GQR7` quantiser, no jump table, no
  unresolved base, no incoming float argument). `psq_st/psq_l` pairs that only
  spill `f31`/`f30` to the stack do not disqualify a row.
* **C - needs one bounded step first.** The body is complete, but one thing must
  be established: an incoming float/double argument (`f1`-style register), a
  `bctr` jump table that must be read out of `.data`, a `0x802DCBFC`
  forwarder convention, a mutex stub's return pair, or an indirect call whose
  receiver is only known to the caller.
* **D - genuinely unreconstructable from this module.** A `psq_l/psq_st` whose
  quantiser is `GQR1..GQR7` (see finding 2), a bad or zero word inside the
  reachable body, a register the decompiler never defines (`extraout_`), or a
  4-byte alias of another unclaimed entry.

## Summary

| Class | Addresses | Bytes |
|---|---:|---:|
| B - reconstructible now | 173 | 38,036 |
| C - one more step | 160 | 47,548 |
| D - unreconstructable | 51 | 20,932 |
| **total** | **384** | **106,516** |

Four findings change the earlier picture more than any single row:

1. **Most "GQR-damaged" bodies are only FPR spills.** When the only
   `psq_st/psq_l` in the stream are `f31`/`f30` saves to `d(r1)` with
   quantiser 0, the export's `unaff_GQR0` + `ldexpf` noise is cosmetic. The
   body underneath is ordinary scalar code. Examples: `0x80A368B0`,
   `0x80AD0960`, `0x80B1D090`, `0x80A2BC50`.
2. **No GQR is programmed in this REL.** A scan of the whole 1.2 MB code range
   finds zero `mtspr GQRn`. A `psq_l` that really uses `GQR1..GQR7` therefore
   cannot be typed from this module - that is the D rule below, and it is the
   same cutoff waves 110-126 used.
3. **"halt_baddata" usually means the decompiler lost a tail call.** In
   `0x80A01700`, `0x80A3F630`, `0x80A61E10`, `0x80AA39D0` and
   `0x80A443A0` the export stops at `halt_baddata`, while the stream continues
   to a plain `b <engine function>`. Those rows are B.
4. **The `2^52` "folded constant" skips are recoverable.** In
   `0x80A2B490`, `0x80A6F8C0` and `0x80A2C2A0` the integer is a `li`
   literal or a dead stack word, and the stream shows which; in
   `0x80A46BF0`/`0x80A3F8F0` the "unresolved `fRam00000398`" is literally
   `lfs f1,0x398(r0)`.

## Table

`class` is this pass's verdict; `prior reason` is a short tag for what the
wave-110..126 doc said; `note` cites what the raw stream shows at the entry.

| address | bytes | prior reason | class | note |
|---|---:|---|---|---|
| `0x80A00D00` | 424 | GQR/paired-single (w112) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80A01700` | 80 | halt_baddata (w112) | **B** | stream runs 20 insns to `b`; no calls |
| `0x80A06220` | 212 | dropped operands (w112) | **C** | export prints a callee with no arguments |
| `0x80A06760` | 160 | dropped operands (w112) | **C** | export prints a callee with no arguments |
| `0x80A06800` | 132 | dropped operands (w112) | **C** | export prints a callee with no arguments |
| `0x80A06B00` | 124 | dropped operands (w112) | **B** | calls `0x80A61A80(f1,f2,actor,3,1)`, then sets `+0x115C=-1`, `+0x115E=(u16)((int)+0x5AC)<<8`, `+0x1154/+0x1158` from `0x80B2AAA8` |
| `0x80A07080` | 224 | dropped operands (w112) | **B** | stream runs 56 insns to `blr`; 1 direct call (`0x80a07080`) |
| `0x80A07160` | 136 | dropped operands (w112) | **B** | stream runs 34 insns to `blr`; 2 direct calls (`0x802e1c40`, `0x802e1ab4`) |
| `0x80A0B8A0` | 108 | prototype (w112) | **B** | class-list walk: `0x80162D50(0x83)`, compare `+0x754`, then a **tail recursion on `r3` only** (`b 0x80A0B8A0`) - the "two-argument" call in the export is a decompiler artefact |
| `0x80A0BD20` | 460 | prototype (w112) | **B** | stream runs 115 insns to `blr`; 7 direct calls (`0x800709c0`, `0x80a0b690`, `0x80a0b8a0`, ...) |
| `0x80A0E160` | 36 | halt_baddata (w112) | **B** | stream runs 9 insns to `b`; no calls |
| `0x80A0E450` | 276 | truncated export (w112) | **B** | stream runs 69 insns to `blr`; 4 direct calls (`0x8008c380`, `0x8005f900`, `0x80076d70`, ...) |
| `0x80A0E580` | 968 | GQR/paired-single (w112) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80A10220` | 484 | clean, not attempted (w116) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80A10880` | 940 | clean, not attempted (w116) | **C** | incoming `f1` (float argument) plus FPR spill; export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80A12110` | 8 | clean, not attempted (w116) | **B** | stream runs 2 insns to `b`; no calls |
| `0x80A18730` | 24 | clean, not attempted (w116) | **B** | stream runs 6 insns to `b`; no calls |
| `0x80A18750` | 24 | clean, not attempted (w116) | **B** | stream runs 6 insns to `b`; no calls |
| `0x80A18770` | 28 | clean, not attempted (w116) | **B** | stream runs 7 insns to `b`; no calls |
| `0x80A18E30` | 68 | clean, not attempted (w116) | **B** | two `0x8008C380(+0x148 / +0x5F0)` resets plus `+0x10C = 0x80B2B498` |
| `0x80A197F0` | 48 | clean, not attempted (w116) | **C** | `0x802DCBFC` forwarder |
| `0x80A19820` | 48 | clean, not attempted (w116) | **C** | `0x802DCBFC` forwarder |
| `0x80A1B2E0` | 20 | clean, not attempted (w116) | **B** | stream runs 5 insns to `b`; no calls |
| `0x80A1C8F0` | 636 | mutex stub (w116) | **C** | export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80A1D530` | 208 | GQR/paired-single (w116) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80A1DA70` | 48 | mutex stub (w116) | **C** | `0x802DCBFC` forwarder |
| `0x80A1DAA0` | 48 | mutex stub (w116) | **C** | `0x802DCBFC` forwarder |
| `0x80A1E100` | 112 | clean, not attempted (w116) | **B** | `0x800146B0(x-DAT, -(DAT+y), z, +0x698)` then `0x800146F0(f1, +0x698, -(s16)+0x104)` |
| `0x80A1EF30` | 52 | clean, not attempted (w116) | **C** | export prints a callee with no arguments |
| `0x80A1F390` | 88 | clean, not attempted (w116) | **B** | standard `0x8015F800`/`0x802B9280` teardown; byte-identical to 0x80A43DC0/0x80A446B0 |
| `0x80A1FFB0` | 88 | clean, not attempted (w116) | **B** | same teardown as 0x80A1F390 |
| `0x80A20810` | 228 | clean, not attempted (w123) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80A20900` | 520 | clean, not attempted (w123) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments |
| `0x80A22820` | 700 | clean, not attempted (w123) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80A22B20` | 44 | clean, not attempted (w123) | **B** | stream runs 11 insns to `b`; no calls |
| `0x80A24AD0` | 312 | clean, not attempted (w123) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80A25F00` | 48 | clean, not attempted (w123) | **C** | `0x802DCBFC` forwarder |
| `0x80A281C0` | 32 | halt_baddata (w123) | **B** | stream runs 8 insns to `b`; no calls |
| `0x80A2B490` | 48 | halt_baddata (w123) | **B** | returns `(f32)27.0`; the `2^52+27` bias is built from the literal `li r3,27`, so the operand is not folded away |
| `0x80A2BC50` | 456 | GQR/paired-single (w123) | **B** | stream runs 114 insns to `blr`; 3 direct calls (`0x806b4ed0`, `0x80170980`, `0x806b4f40`); the only `psq` ops are f31/f30 stack spills; `ldexpf` in the export is the `2^52` integer-to-double idiom |
| `0x80A2C2A0` | 76 | halt_baddata (w123) | **B** | returns `-(s16)(int)(x*x)` with `x = +0xE8`; the `2^52` constant is built but **never consumed**, so it is not an operand |
| `0x80A2D460` | 20 | truncated export (w123) | **B** | stream runs 5 insns to `b`; no calls |
| `0x80A2D660` | 132 | truncated export (w123) | **B** | stream runs 33 insns to `blr`; 5 direct calls (`0x8006ef40`, `0x80a2d460`, `0x80162510`, ...) |
| `0x80A2E970` | 240 | GQR/paired-single (w123) | **B** | stream runs 60 insns to `blr`; 1 direct call (`0x800b2ee0`) |
| `0x80A2EA60` | 224 | GQR/paired-single (w123) | **B** | stream runs 56 insns to `blr`; no calls; the only `psq` ops are f31/f30 stack spills; `ldexpf` in the export is the `2^52` integer-to-double idiom |
| `0x80A2EB40` | 188 | GQR/paired-single (w123) | **C** | incoming `f1` (float argument) plus FPR spill |
| `0x80A2EC00` | 472 | GQR/paired-single (w123) | **C** | FPR spill / pair traffic only |
| `0x80A2F670` | 88 | clean, not attempted (w123) | **B** | standard teardown (`0x8015F800`/`0x802B9280`) |
| `0x80A2F6D0` | 92 | clean, not attempted (w123) | **B** | teardown with the extra `cmpi` guard, same two callees |
| `0x80A2F730` | 12 | clean, not attempted (w123) | **B** | constant producer: `return 0x80357C34` |
| `0x80A2F740` | 12 | clean, not attempted (w123) | **B** | constant producer: `return 0x80357BF4` |
| `0x80A2F830` | 224 | clean, not attempted (w123) | **B** | bubble helper; only `ldexpf`/`2^52` noise in the export, plain `lfs/fmuls/fctiwz` in the stream |
| `0x80A2F910` | 136 | clean, not attempted (w123) | **B** | same body family; the folded `2^52` word is a literal in the stream |
| `0x80A2F9D0` | 48 | clean, not attempted (w123) | **C** | `0x802DCBFC` forwarder |
| `0x80A2FA00` | 48 | clean, not attempted (w123) | **C** | `0x802DCBFC` forwarder |
| `0x80A2FA30` | 308 | clean, not attempted (w123) | **C** | export prints a callee with no arguments |
| `0x80A2FD40` | 340 | clean, not attempted (w123) | **B** | stream runs 85 insns to `blr`; 9 direct calls (`0x8005f900`, `0x800690c0`, `0x800df180`, ...) |
| `0x80A334D0` | 352 | truncated export (w117) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments |
| `0x80A33630` | 380 | truncated export (w117) | **B** | stream runs 95 insns to `blr`; 10 direct calls (`0x80064af0`, `0x801c0bd0`, `0x8016edd0`, ...); the only `psq` ops are f31/f30 stack spills; `ldexpf` in the export is the `2^52` integer-to-double idiom |
| `0x80A337B0` | 4 | truncated export (w117) | **D** | 4-byte alias: `b 0x80A337C0`; the target is itself unclaimed, so there is no body to type |
| `0x80A337C0` | 32 | truncated export (w117) | **B** | stream runs 8 insns to `b`; no calls |
| `0x80A34F80` | 348 | truncated export (w117) | **B** | billboard emitter: builds a `0x1003` record on the stack from `+0xAC..+0xB4` and the `0x80B2C094` fill word, then `0x800B8700(+0x484, rec, 1)` and `0x802DCA98(rec, 0x80A0B5A0, 0x3C, 1)` |
| `0x80A35170` | 464 | truncated export (w117) | **B** | stream runs 116 insns to `blr`; 5 direct calls (`0x800c1230`, `0x800c12b0`, `0x8006a4b0`, ...) |
| `0x80A35370` | 232 | truncated export (w117) | **B** | crow clip rate: `0x800B2ED0(10)`, unsigned->float via `0x43300000`, `/10` by `0x66666667`, a second conversion of `(int)+0x5AC`, then `0x80168FE0(rate, +0x584)` |
| `0x80A35570` | 28 | truncated export (w117) | **C** | no terminator inside the 28-byte extent: the body falls into the next function, so the extent is too small for this entry |
| `0x80A35ED0` | 228 | truncated export (w117) | **C** | export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80A360D0` | 228 | truncated export (w117) | **B** | stream runs 57 insns to `blr`; 8 direct calls (`0x800690c0`, `0x800df180`, `0x80239e30`, ...) |
| `0x80A365B0` | 192 | truncated export (w117) | **C** | export prints a callee with no arguments |
| `0x80A36670` | 132 | truncated export (w117) | **C** | export prints a callee with no arguments |
| `0x80A368B0` | 244 | truncated export (w117) | **B** | proximity scan over 4 slots: `0x8005F900(i)`, mask byte `0x80429CA8`, \|dx\|<`0x80B2C14C`, \|dy\|<`0x80B2C1A8`; the four `psq_st/psq_l` are only f30/f31 stack spills |
| `0x80A369B0` | 152 | truncated export (w117) | **B** | heading step: `0x80096240(actor,+0xAC)` returns an index, two short tables at `0x8042C1A8`/`0x8042C1A4`, `0x802DE514` compares and clamps `+0x102` |
| `0x80A36A50` | 32 | truncated export (w117) | **B** | stream runs 8 insns to `b`; no calls |
| `0x80A36C10` | 608 | truncated export (w117) | **B** | stream runs 152 insns to `blr`; 7 direct calls (`0x8016c890`, `0x8006a4b0`, `0x80197f00`, ...) |
| `0x80A36FC0` | 248 | truncated export (w117) | **B** | stream runs 62 insns to `blr`; 7 direct calls (`0x8023a0b0`, `0x801651f0`, `0x80169f80`, ...) |
| `0x80A370D0` | 292 | truncated export (w117) | **B** | stream runs 73 insns to `blr`; 9 direct calls (`0x8006cde0`, `0x800955f0`, `0x80072bc0`, ...) |
| `0x80A372A0` | 448 | truncated export (w117) | **C** | incoming `f1` (float argument) |
| `0x80A37460` | 228 | truncated export (w117) | **B** | stream runs 57 insns to `blr`; 4 direct calls (`0x801680e0`, `0x8016c890`, `0x8006a4b0`, ...) |
| `0x80A37560` | 296 | truncated export (w117) | **B** | stream runs 74 insns to `blr`; 8 direct calls (`0x8006cd90`, `0x8006cde0`, `0x800955f0`, ...) |
| `0x80A37700` | 256 | truncated export (w117) | **B** | stream runs 64 insns to `blr`; 5 direct calls (`0x8006cde0`, `0x800955f0`, `0x80072bc0`, ...) |
| `0x80A37830` | 1148 | truncated export (w117) | **B** | stream runs 287 insns to `blr`; 11 direct calls (`0x8006cde0`, `0x800955f0`, `0x80072bc0`, ...); the only `psq` ops are f31/f30 stack spills; `ldexpf` in the export is the `2^52` integer-to-double idiom |
| `0x80A37CB0` | 304 | truncated export (w117) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80A390F0` | 300 | truncated export (w117) | **C** | export prints a callee with no arguments |
| `0x80A39410` | 324 | truncated export (w117) | **B** | stream runs 81 insns to `blr`; 11 direct calls (`0x800690c0`, `0x800df180`, `0x80239e30`, ...) |
| `0x80A39650` | 312 | truncated export (w117) | **C** | export prints a callee with no arguments |
| `0x80A397B0` | 228 | truncated export (w117) | **C** | export prints a callee with no arguments |
| `0x80A398C0` | 132 | truncated export (w117) | **B** | stream runs 33 insns to `blr`; 1 direct call (`0x800dae80`) |
| `0x80A3A4A0` | 2620 | truncated export (w117) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80A3DEA0` | 2940 | truncated export (w117) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80A3F630` | 32 | clean, not attempted (w117) | **B** | three-range guard on `+0xA7C` (0, 0xF, 0xD) then a tail call to `0x80027A60` - the export's `halt_baddata` hides this tail call |
| `0x80A3F6D0` | 16 | clean, not attempted (w117) | **B** | stream runs 4 insns to `b`; no calls |
| `0x80A3F8F0` | 176 | skipped (w117) | **B** | `0x80162D50(0x79,0)` then `fabs`/`frsp` compare; on the null result the stream really does `lfs f1,0x398(r0)`, so `fRam00000398` is absolute, not unresolved |
| `0x80A40290` | 232 | GQR/paired-single (w126) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80A40CA0` | 208 | GQR/paired-single (w126) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80A40FA0` | 28 | halt_baddata (w126) | **B** | stream runs 7 insns to `b`; no calls |
| `0x80A429C0` | 476 | halt_baddata (w126) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80A436C0` | 328 | halt_baddata (w126) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80A43810` | 252 | halt_baddata (w126) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80A43DC0` | 88 | clean, not attempted (w126) | **B** | standard teardown; byte-identical to 0x80A446B0 |
| `0x80A43EE0` | 48 | clean, not attempted (w126) | **C** | `0x802DCBFC` forwarder |
| `0x80A43F10` | 48 | clean, not attempted (w126) | **C** | `0x802DCBFC` forwarder |
| `0x80A441E0` | 116 | clean, not attempted (w126) | **B** | copies `+0x7F8..+0x800` from `0x80162D00(+0x9F8)` into the actor position, `+0xB0 -= 0x8042B354`, copies `+0x102` |
| `0x80A442D0` | 208 | clean, not attempted (w126) | **B** | stream runs 52 insns to `blr`; 8 direct calls (`0x8007f950`, `0x8006cde0`, `0x800955f0`, ...) |
| `0x80A443A0` | 24 | halt_baddata (w126) | **B** | tail call `0x80064610(402, 0, r5, 0, 0)`; the export's three readable stores plus `halt_baddata` are the whole body |
| `0x80A446B0` | 88 | clean, not attempted (w126) | **B** | standard teardown; byte-identical to 0x80A43DC0 |
| `0x80A45DB0` | 500 | clean, not attempted (w126) | **B** | stream runs 125 insns to `blr`; 8 direct calls (`0x80077810`, `0x8007fb10`, `0x80161800`, ...) |
| `0x80A46200` | 300 | halt_baddata (w126) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80A46BF0` | 172 | clean, not attempted (w126) | **B** | `0x80162D50(0x79,0)`; on the zero result the stream really does `lfs f1,0x398(r0)` (absolute `0x398`), so `fRam00000398` is exact |
| `0x80A48940` | 48 | halt_baddata (w126) | **C** | `+0x674 += f1` clamped to `0x80B2xxxx`, then `0x80168FE0(+0x584)`; the incoming `f1` is the blocker |
| `0x80A4B0B0` | 136 | GQR/paired-single (w126) | **C** | incoming `f1` (float argument) plus FPR spill |
| `0x80A4B140` | 236 | GQR/paired-single (w126) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80A4B420` | 548 | GQR/paired-single (w126) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80A4B740` | 604 | GQR/paired-single (w126) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80A4BBD0` | 304 | GQR/paired-single (w126) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80A4EC90` | 12 | clean, not attempted (w126) | **B** | constant producer: `return 0x80358084` |
| `0x80A600B0` | 240 | clean, not attempted (w124) | **B** | stream runs 60 insns to `blr`; 2 direct calls (`0x800955f0`, `0x80a5fe90`) |
| `0x80A61410` | 48 | clean, not attempted (w124) | **C** | `0x802DCBFC` forwarder |
| `0x80A61440` | 48 | clean, not attempted (w124) | **C** | `0x802DCBFC` forwarder |
| `0x80A61E10` | 64 | halt_baddata (w124) | **B** | `+0x736 += 1`; every 64th frame tail-calls `0x80078470(0x80B2xxxx, 1, 1, f1, f2)` - the truncation hides a tail call, not a branch |
| `0x80A628F0` | 292 | GQR/paired-single (w124) | **B** | stream runs 73 insns to `blr`; 2 direct calls (`0x80a61a80`, `0x80a61f90`); the only `psq` ops are f31/f30 stack spills; `ldexpf` in the export is the `2^52` integer-to-double idiom |
| `0x80A62A30` | 312 | GQR/paired-single (w124) | **C** | incoming `f1` (float argument) plus FPR spill; export prints a callee with no arguments |
| `0x80A635D0` | 488 | clean, not attempted (w124) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80A637E0` | 236 | GQR/paired-single (w124) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments |
| `0x80A638F0` | 284 | GQR/paired-single (w124) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments |
| `0x80A63A30` | 284 | GQR/paired-single (w124) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments |
| `0x80A63FF0` | 392 | GQR/paired-single (w124) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80A641F0` | 380 | GQR/paired-single (w124) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80A656A0` | 48 | clean, not attempted (w124) | **C** | `0x802DCBFC` forwarder |
| `0x80A656D0` | 48 | clean, not attempted (w124) | **C** | `0x802DCBFC` forwarder |
| `0x80A673D0` | 1080 | GQR/paired-single (w124) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80A69130` | 48 | clean, not attempted (w124) | **C** | `0x802DCBFC` forwarder |
| `0x80A69160` | 48 | clean, not attempted (w124) | **C** | `0x802DCBFC` forwarder |
| `0x80A6A5F0` | 48 | clean, not attempted (w124) | **C** | `0x802DCBFC` forwarder |
| `0x80A6A620` | 48 | clean, not attempted (w124) | **C** | `0x802DCBFC` forwarder |
| `0x80A6BA00` | 48 | clean, not attempted (w124) | **C** | `0x802DCBFC` forwarder |
| `0x80A6BA30` | 48 | clean, not attempted (w124) | **C** | `0x802DCBFC` forwarder |
| `0x80A6C7E0` | 48 | clean, not attempted (w124) | **C** | `0x802DCBFC` forwarder |
| `0x80A6EA20` | 212 | GQR/paired-single (w124) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80A6EFB0` | 100 | halt_baddata (w124) | **B** | three-branch y-threshold state machine returning 0/1/2; the `halt_baddata` is the `cror` at `0x80A6EFDC` that Ghidra cannot decode, followed by a normal `bc` |
| `0x80A6F8C0` | 48 | 2^52 bias (w124) | **B** | returns `(f32)30.0` from the same `2^52+n` idiom (literal 30) |
| `0x80A6FBC0` | 624 | GQR/paired-single (w124) | **C** | incoming `f1` (float argument) plus FPR spill |
| `0x80A707E0` | 652 | clean, not attempted (w121) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments |
| `0x80A778F0` | 116 | clean, not attempted (w121) | **B** | stream runs 29 insns to `b`; no calls |
| `0x80A77A50` | 340 | clean, not attempted (w121) | **B** | clean release/teardown body; only f31 spill noise |
| `0x80A77BB0` | 124 | clean, not attempted (w121) | **B** | stream runs 31 insns to `b`; no calls |
| `0x80A77C30` | 548 | clean, not attempted (w121) | **B** | stream runs 137 insns to `blr`; 4 direct calls (`0x80168fe0`, `0x80167b20`, `0x80167b80`, ...) |
| `0x80A78350` | 616 | clean, not attempted (w121) | **C** | export prints a callee with no arguments |
| `0x80A78700` | 64 | clean, not attempted (w121) | **B** | `+0x124 = 0`, `0x8008C380(+0x148)`, `0x8007F950(+0x754)` |
| `0x80A79630` | 236 | clean, not attempted (w121) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80A79750` | 188 | clean, not attempted (w121) | **B** | stream runs 47 insns to `blr`; 1 direct call (`0x80066630`) |
| `0x80A79810` | 360 | clean, not attempted (w121) | **D** | reads an `extraout_` register that is never defined |
| `0x80A7A460` | 312 | clean, not attempted (w121) | **C** | export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80A7A8C0` | 320 | GQR/paired-single (w121) | **C** | collision-row scan; the actor arrives through the `0x802DCD70` prologue and the coordinate pair is read from `ps29` spills |
| `0x80A7AA00` | 292 | GQR/paired-single (w121) | **C** | same collision-row scan as 0x80A7A8C0 |
| `0x80A7AB90` | 272 | skipped (w121) | **C** | FPR spill / pair traffic only |
| `0x80A7ACA0` | 252 | skipped (w121) | **C** | FPR spill / pair traffic only |
| `0x80A7ADA0` | 220 | clean, not attempted (w121) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80A7B7C0` | 184 | clean, not attempted (w121) | **C** | export prints a callee with no arguments |
| `0x80A7BD30` | 372 | clean, not attempted (w121) | **B** | stream runs 93 insns to `blr`; 10 direct calls (`0x8023a0b0`, `0x801651f0`, `0x80169f80`, ...); the only `psq` ops are f31/f30 stack spills; `ldexpf` in the export is the `2^52` integer-to-double idiom |
| `0x80A7BEC0` | 1464 | clean, not attempted (w121) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments |
| `0x80A7C600` | 184 | clean, not attempted (w121) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80A7C7D0` | 628 | clean, not attempted (w121) | **B** | scratch-matrix builder: a `param_2 == 0x15` path plus a 9-entry byte table at `0x80B2DFB0`; all five callees (`0x801C0500`, `0x801C0540`, `0x801C0BD0`, `0x801C0C60`, `0x8016EDD0`) take explicit arguments |
| `0x80A7E630` | 8 | halt_baddata (w121) | **B** | stream runs 2 insns to `b`; no calls |
| `0x80A7E640` | 212 | clean, not attempted (w121) | **B** | contact test; the prologue keeps only an f31 spill, the body is plain `lfs/fcmpo` comparisons |
| `0x80A7EB60` | 280 | clean, not attempted (w121) | **B** | stream runs 70 insns to `blr`; 2 direct calls (`0x80a7e630`, `0x8008c330`); the only `psq` ops are f31/f30 stack spills; `ldexpf` in the export is the `2^52` integer-to-double idiom |
| `0x80A7ECA0` | 84 | clean, not attempted (w121) | **B** | stream runs 21 insns to `blr`; 1 direct call (`0x80a7e630`) |
| `0x80A7ED20` | 176 | clean, not attempted (w121) | **B** | stream runs 44 insns to `blr`; 4 direct calls (`0x80169020`, `0x8006a4b0`, `0x80199390`, ...) |
| `0x80A7EE20` | 128 | clean, not attempted (w121) | **B** | stream runs 32 insns to `blr`; 6 direct calls (`0x8006cde0`, `0x800955f0`, `0x80072bc0`, ...) |
| `0x80A7F690` | 48 | clean, not attempted (w121) | **C** | `0x802DCBFC` forwarder |
| `0x80A7F6C0` | 48 | clean, not attempted (w121) | **C** | `0x802DCBFC` forwarder |
| `0x80A7FB50` | 392 | clean, not attempted (w121) | **B** | stream runs 98 insns to `blr`; 9 direct calls (`0x80064af0`, `0x801c0bd0`, `0x8016edd0`, ...); the only `psq` ops are f31/f30 stack spills; `ldexpf` in the export is the `2^52` integer-to-double idiom |
| `0x80A90AE0` | 48 | clean, not attempted (w118) | **C** | `0x802DCBFC` forwarder |
| `0x80A90B10` | 48 | clean, not attempted (w118) | **C** | `0x802DCBFC` forwarder |
| `0x80A915E0` | 48 | clean, not attempted (w118) | **C** | `0x802DCBFC` forwarder |
| `0x80A91610` | 48 | clean, not attempted (w118) | **C** | `0x802DCBFC` forwarder |
| `0x80A91D10` | 32 | halt_baddata (w118) | **B** | stream runs 8 insns to `b`; no calls |
| `0x80A91D30` | 56 | lost jump table (w118) | **C** | guard `+0x708 < 0x5A` then `bctr` through a jump table at `0x80A91D64`; the table is data and must be read first |
| `0x80A92F40` | 1172 | GQR/paired-single (w118) | **C** | export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80A95D00` | 48 | clean, not attempted (w118) | **C** | `0x802DCBFC` forwarder |
| `0x80A95D30` | 48 | clean, not attempted (w118) | **C** | `0x802DCBFC` forwarder |
| `0x80A96230` | 572 | clean, not attempted (w118) | **B** | stream runs 143 insns to `blr`; 17 direct calls (`0x8005f900`, `0x80056cf0`, `0x8012de80`, ...); the only `psq` ops are f31/f30 stack spills; `ldexpf` in the export is the `2^52` integer-to-double idiom |
| `0x80A969C0` | 844 | GQR/paired-single (w118) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments |
| `0x80A96DC0` | 848 | GQR/paired-single (w118) | **C** | export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80A981F0` | 48 | clean, not attempted (w118) | **C** | `0x802DCBFC` forwarder |
| `0x80A98220` | 48 | clean, not attempted (w118) | **C** | `0x802DCBFC` forwarder |
| `0x80A99360` | 228 | GQR/paired-single (w118) | **C** | incoming `f1` (float argument) plus FPR spill; export prints a callee with no arguments |
| `0x80A9A7A0` | 48 | clean, not attempted (w118) | **C** | `0x802DCBFC` forwarder |
| `0x80A9A7D0` | 48 | clean, not attempted (w118) | **C** | `0x802DCBFC` forwarder |
| `0x80A9B3C0` | 1196 | GQR/paired-single (w118) | **C** | export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80A9E0D0` | 24 | halt_baddata (w118) | **B** | stream runs 6 insns to `b`; no calls |
| `0x80A9F750` | 28 | halt_baddata (w118) | **B** | stream runs 7 insns to `b`; no calls |
| `0x80AA0080` | 28 | halt_baddata (w125) | **B** | stream runs 7 insns to `b`; no calls |
| `0x80AA0300` | 52 | halt_baddata (w125) | **B** | stream runs 13 insns to `b`; no calls |
| `0x80AA0530` | 388 | GQR/paired-single (w125) | **C** | incoming `f2` (float argument) plus FPR spill; export prints a callee with no arguments |
| `0x80AA0C20` | 872 | GQR/paired-single (w125) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80AA1030` | 72 | truncated export (w125) | **C** | export prints a callee with no arguments |
| `0x80AA35E0` | 408 | GQR/paired-single (w125) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80AA37E0` | 240 | GQR/paired-single (w125) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80AA39D0` | 48 | halt_baddata (w125) | **B** | `+0x914` array indexed by `r4<4`, then `b 0x80162D00`; the guard is readable and the tail call is the missing instruction |
| `0x80AA3A90` | 216 | GQR/paired-single (w125) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments |
| `0x80AA3F50` | 200 | GQR/paired-single (w125) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80AA4070` | 444 | GQR/paired-single (w125) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80AA8470` | 592 | GQR/paired-single (w125) | **B** | stream runs 148 insns to `blr`; 12 direct calls (`0x80056cf0`, `0x80056bd0`, `0x8008f9f0`, ...); the only `psq` ops are f31/f30 stack spills; `ldexpf` in the export is the `2^52` integer-to-double idiom |
| `0x80AA93F0` | 496 | GQR/paired-single (w125) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80AAA3E0` | 20 | halt_baddata (w125) | **B** | stream runs 5 insns to `b`; no calls |
| `0x80AAA480` | 20 | halt_baddata (w125) | **B** | stream runs 5 insns to `b`; no calls |
| `0x80AAC360` | 680 | GQR/paired-single (w125) | **B** | stream runs 170 insns to `blr`; 8 direct calls (`0x801c0bd0`, `0x8016edd0`, `0x8016a170`, ...); the only `psq` ops are f31/f30 stack spills; `ldexpf` in the export is the `2^52` integer-to-double idiom |
| `0x80AAC610` | 596 | clean, not attempted (w125) | **B** | stream runs 149 insns to `blr`; 8 direct calls (`0x801c0bd0`, `0x8016edd0`, `0x8016a170`, ...) |
| `0x80AAC870` | 900 | clean, not attempted (w125) | **B** | stream runs 225 insns to `blr`; 8 direct calls (`0x80064af0`, `0x801c0bd0`, `0x8016edd0`, ...) |
| `0x80AAD490` | 484 | clean, not attempted (w125) | **C** | incoming `f3` (float argument); export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80AAD9B0` | 656 | GQR/paired-single (w125) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments |
| `0x80AADC50` | 628 | GQR/paired-single (w125) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments |
| `0x80AAE360` | 48 | clean, not attempted (w125) | **C** | `0x802DCBFC` forwarder |
| `0x80AAE390` | 48 | clean, not attempted (w125) | **C** | `0x802DCBFC` forwarder |
| `0x80AAE520` | 1156 | GQR/paired-single (w125) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80AAF2B0` | 108 | GQR/paired-single (w125) | **C** | incoming `f2` (float argument) plus FPR spill |
| `0x80AAFCB0` | 48 | clean, not attempted (w125) | **C** | `0x802DCBFC` forwarder |
| `0x80AAFCE0` | 48 | clean, not attempted (w125) | **C** | `0x802DCBFC` forwarder |
| `0x80AB0570` | 664 | clean, not attempted (w120) | **C** | export prints a callee with no arguments |
| `0x80AB0870` | 204 | clean, not attempted (w120) | **C** | export prints a callee with no arguments |
| `0x80AB0C00` | 440 | clean, not attempted (w120) | **B** | particle spawn: three state branches (`+0x98C` = 0/1/other) all build the same `+0xAC..+0xB4` record with the `0x80B2FB00` y offset, then `0x800B8700` + `0x802DCA98` |
| `0x80AB0EE0` | 640 | clean, not attempted (w120) | **C** | export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80AB1160` | 384 | clean, not attempted (w120) | **B** | stream runs 96 insns to `blr`; 9 direct calls (`0x80064af0`, `0x801c0bd0`, `0x8016edd0`, ...) |
| `0x80AB12E0` | 244 | clean, not attempted (w120) | **C** | export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80AB13E0` | 304 | clean, not attempted (w120) | **C** | export prints a callee with no arguments |
| `0x80AB1510` | 624 | clean, not attempted (w120) | **B** | stream runs 156 insns to `blr`; 16 direct calls (`0x800690c0`, `0x800df180`, `0x80239e30`, ...) |
| `0x80AB1780` | 172 | clean, not attempted (w120) | **C** | incoming `f1` (float argument) plus FPR spill |
| `0x80AB1830` | 92 | clean, not attempted (w120) | **B** | stream runs 23 insns to `b`; no calls |
| `0x80AB1890` | 104 | clean, not attempted (w120) | **B** | angle -> speed: `+0x102` through the `0x43300000` bias, `x 0x80B2FB3C` negated into `+0x164`, and `+0x7DC` |
| `0x80AB1900` | 256 | clean, not attempted (w120) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80AB1A00` | 8 | clean, not attempted (w120) | **B** | stream runs 2 insns to `b`; no calls |
| `0x80AB21D0` | 176 | clean, not attempted (w120) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80AB22D0` | 276 | clean, not attempted (w120) | **B** | stream runs 69 insns to `blr`; 7 direct calls (`0x80ab1b10`, `0x8006a4b0`, `0x80198660`, ...) |
| `0x80AB2460` | 444 | clean, not attempted (w120) | **B** | stream runs 111 insns to `blr`; 10 direct calls (`0x80ab1ce0`, `0x80170a00`, `0x8006cd40`, ...) |
| `0x80AB2620` | 112 | clean, not attempted (w120) | **B** | resets two pad fields (`0x80166340(+0x73C/+0x794,3,0)`), calls `0x80AB1780(f1, actor, 1)`, clears `+0x104` |
| `0x80AB26A0` | 408 | clean, not attempted (w120) | **C** | incoming `f1` (float argument) |
| `0x80AB2840` | 356 | clean, not attempted (w120) | **B** | stream runs 89 insns to `blr`; 3 direct calls (`0x8008c380`, `0x80ab1a00`, `0x80166340`) |
| `0x80AB29C0` | 376 | clean, not attempted (w120) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80AB5710` | 1064 | clean, not attempted (w120) | **B** | large per-frame step; four `psq` spill words and four `ldexpf` artefacts in the export, 266 clean instructions in the stream |
| `0x80AB9AF0` | 284 | clean, not attempted (w120) | **B** | spawn-table entry `0x80B2FF28 + (bit12 ? 12 : 0)`, stores the `0x80B2FF50` block into `+0xD0..+0xE4`/`+0x318..+0x324`, `+0x53C = f6 / (u16)(+4>>8 & 0xF)`, then `0x8007FA40`/`0x8007F900` on `+0x458` |
| `0x80ABA080` | 308 | clean, not attempted (w120) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80ABA5B0` | 64 | clean, not attempted (w120) | **C** | export prints a callee with no arguments |
| `0x80ABA5F0` | 64 | clean, not attempted (w120) | **C** | export prints a callee with no arguments |
| `0x80ABA950` | 156 | clean, not attempted (w120) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80ABAB50` | 236 | clean, not attempted (w120) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80ABB1B0` | 48 | clean, not attempted (w120) | **C** | `0x802DCBFC` forwarder |
| `0x80ABB1E0` | 48 | clean, not attempted (w120) | **C** | `0x802DCBFC` forwarder |
| `0x80AC95D0` | 540 | skipped (w114) | **B** | stream runs 135 insns to `blr`; 10 direct calls (`0x8006cde0`, `0x800955f0`, `0x801690e0`, ...) |
| `0x80AC9870` | 464 | mutex stub (w114) | **C** | export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80AC9AA0` | 372 | skipped (w114) | **B** | stream runs 93 insns to `blr`; 8 direct calls (`0x8006cde0`, `0x800955f0`, `0x80072bc0`, ...) |
| `0x80ACB000` | 180 | skipped (w114) | **C** | export prints a callee with no arguments |
| `0x80ACB130` | 196 | mutex stub (w114) | **C** | export prints a callee with no arguments |
| `0x80ACB280` | 172 | skipped (w114) | **C** | export prints a callee with no arguments |
| `0x80ACB330` | 252 | GQR/paired-single (w114) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80ACB4B0` | 140 | mutex stub (w114) | **C** | export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80ACB5D0` | 188 | mutex stub (w114) | **C** | export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80ACC520` | 492 | GQR/paired-single (w114) | **B** | stream runs 123 insns to `blr`; 13 direct calls (`0x8006a4b0`, `0x80198870`, `0x8006cde0`, ...); the only `psq` ops are f31/f30 stack spills; `ldexpf` in the export is the `2^52` integer-to-double idiom |
| `0x80ACC740` | 388 | GQR/paired-single (w114) | **B** | stream runs 97 insns to `blr`; 11 direct calls (`0x8006cde0`, `0x800955f0`, `0x80096f60`, ...); the only `psq` ops are f31/f30 stack spills; `ldexpf` in the export is the `2^52` integer-to-double idiom |
| `0x80ACC8F0` | 652 | GQR/paired-single (w114) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80ACDB00` | 148 | GQR/paired-single (w114) | **C** | incoming `f1` (float argument) plus FPR spill |
| `0x80AD0960` | 296 | GQR/paired-single (w111) | **B** | three ground probes: `0x80070BA0`/`0x80070BF0` at `x±0x164`, `y+0x168`, hit codes compared against 2; only f29/f30/f31 spills |
| `0x80AD0A90` | 32 | GQR/paired-single (w111) | **B** | stream runs 8 insns to `b`; no calls |
| `0x80AD3C10` | 208 | GQR/paired-single (w111) | **B** | stream runs 52 insns to `blr`; 3 direct calls (`0x800c10f0`, `0x800c1560`, `0x800c1580`); the only `psq` ops are f31/f30 stack spills; `ldexpf` in the export is the `2^52` integer-to-double idiom |
| `0x80AD3D50` | 688 | clean, not attempted (w111) | **B** | actor step; eight direct calls, no GQR use in the body |
| `0x80AD4000` | 532 | clean, not attempted (w111) | **C** | export prints a callee with no arguments |
| `0x80AD42B0` | 36 | halt_baddata (w111) | **B** | stream runs 9 insns to `b`; no calls |
| `0x80AD42E0` | 320 | clean, not attempted (w111) | **B** | stream runs 80 insns to `blr`; 4 direct calls (`0x80022770`, `0x80022390`, `0x80022780`, ...) |
| `0x80AD4420` | 404 | clean, not attempted (w111) | **B** | stream runs 101 insns to `blr`; 1 direct call (`0x80ad42e0`) |
| `0x80AD45E0` | 224 | clean, not attempted (w111) | **C** | export prints a callee with no arguments |
| `0x80AD46C0` | 212 | GQR/paired-single (w111) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments |
| `0x80AD47E0` | 80 | clean, not attempted (w111) | **C** | export prints a callee with no arguments |
| `0x80AD4850` | 80 | clean, not attempted (w111) | **C** | export prints a callee with no arguments |
| `0x80AD48C0` | 72 | clean, not attempted (w111) | **C** | export prints a callee with no arguments |
| `0x80AD5760` | 500 | skipped (w111) | **B** | stream runs 125 insns to `blr`; 5 direct calls (`0x800d77d0`, `0x80ad5480`, `0x80ad54a0`, ...); the only `psq` ops are f31/f30 stack spills; `ldexpf` in the export is the `2^52` integer-to-double idiom |
| `0x80AD5CD0` | 316 | skipped (w111) | **C** | export prints a callee with no arguments |
| `0x80AD6CA0` | 100 | skipped (w111) | **B** | stream runs 25 insns to `blr`; 2 direct calls (`0x800db180`, `0x800dae80`) |
| `0x80AD7310` | 148 | skipped (w111) | **B** | short helper; no psq in the body, only the f31 spill word |
| `0x80AD8900` | 308 | skipped (w111) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80AD98B0` | 544 | skipped (w111) | **C** | incoming `f4` plus ten spill words; the body is a plain probe loop |
| `0x80ADA6E0` | 8 | skipped (w111) | **B** | stream runs 2 insns to `b`; no calls |
| `0x80ADD0E0` | 88 | GQR/paired-single (w111) | **B** | standard teardown (`0x8015F800`/`0x802B9280`) |
| `0x80ADD350` | 536 | GQR/paired-single (w111) | **B** | `block_pata` state entry: `0x80077810(0x80429DD0, x&0xFFFF, -y&0xFFFF, 0x80429D28)` spawn query, copies `0x80B30B30..`, `0x8007FA40`/`0x8007F900` on `+0x790`, par avail `0x80075270`, `0x80ADE240` twice; 134 instructions, all resolved |
| `0x80ADDA40` | 120 | GQR/paired-single (w111) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80ADDAC0` | 124 | GQR/paired-single (w111) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80ADDD10` | 256 | GQR/paired-single (w111) | **B** | stream runs 64 insns to `blr`; 2 direct calls (`0x80065fc0`, `0x80adde10`) |
| `0x80ADE240` | 124 | GQR/paired-single (w111) | **C** | incoming `f1` (float argument) plus FPR spill |
| `0x80ADE3A0` | 136 | GQR/paired-single (w111) | **B** | same `block_pata` step shape with one `0x80ADE240` effect call |
| `0x80AE0780` | 248 | clean, not attempted (w122) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80AE2250` | 108 | clean, not attempted (w122) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80AE4A50` | 660 | clean, not attempted (w122) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80AE63D0` | 388 | clean, not attempted (w122) | **B** | stream runs 97 insns to `blr`; 11 direct calls (`0x80064af0`, `0x801c0bd0`, `0x8016edd0`, ...) |
| `0x80AE69E0` | 444 | clean, not attempted (w122) | **C** | export prints a callee with no arguments |
| `0x80AEAED0` | 284 | GQR/paired-single (w122) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80AEAFF0` | 216 | GQR/paired-single (w122) | **B** | clean trail helper; a single f31 spill only |
| `0x80AEB450` | 700 | GQR/paired-single (w122) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80AEC4E0` | 292 | mutex stub (w122) | **D** | reads an `extraout_` register that is never defined |
| `0x80AECF00` | 88 | clean, not attempted (w122) | **B** | standard teardown (`0x8015F800`/`0x802B9280`) |
| `0x80AED170` | 64 | clean, not attempted (w122) | **C** | export prints a callee with no arguments |
| `0x80AED1B0` | 96 | clean, not attempted (w122) | **B** | teardown of the `+0x18` sub-object via `0x8015F9F0` then `0x802B9280` |
| `0x80AED210` | 100 | clean, not attempted (w122) | **B** | same `+0x18` teardown with one extra guard |
| `0x80AEDA60` | 48 | clean, not attempted (w122) | **C** | `0x802DCBFC` forwarder |
| `0x80AEDA90` | 48 | clean, not attempted (w122) | **C** | `0x802DCBFC` forwarder |
| `0x80AEDAC0` | 208 | clean, not attempted (w122) | **B** | stream runs 52 insns to `blr`; 6 direct calls (`0x8015f9f0`, `0x8007f810`, `0x80169d20`, ...) |
| `0x80AEDB90` | 76 | clean, not attempted (w122) | **C** | export prints a callee with no arguments |
| `0x80AEDC90` | 48 | clean, not attempted (w122) | **B** | stream runs 12 insns to `blr`; no calls |
| `0x80AEDD40` | 1208 | clean, not attempted (w122) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80AEE6D0` | 88 | clean, not attempted (w122) | **B** | standard teardown |
| `0x80AEE7F0` | 48 | clean, not attempted (w122) | **C** | `0x802DCBFC` forwarder |
| `0x80AEE820` | 48 | clean, not attempted (w122) | **C** | `0x802DCBFC` forwarder |
| `0x80AEE850` | 296 | clean, not attempted (w122) | **C** | export prints a callee with no arguments |
| `0x80AEEA30` | 124 | clean, not attempted (w122) | **B** | angle pair: `+4 & 0xF` and `+4>>4 & 0xF` through `0x43300000`, times `0x80B2xxxx`, stored to `+0xDC/+0xE0/+0xE4`; no psq, no calls |
| `0x80AEEFA0` | 292 | clean, not attempted (w122) | **D** | reads an `extraout_` register that is never defined |
| `0x80AEF130` | 1180 | clean, not attempted (w122) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80AEF910` | 88 | clean, not attempted (w122) | **B** | standard teardown |
| `0x80AEFCF0` | 600 | clean, not attempted (w122) | **B** | stream runs 150 insns to `blr`; 16 direct calls (`0x800690c0`, `0x800df180`, `0x80239e30`, ...) |
| `0x80AFD170` | 156 | GQR/paired-single (w119) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80AFD910` | 388 | GQR/paired-single (w119) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80AFDAC0` | 480 | GQR/paired-single (w119) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80AFDD50` | 300 | GQR/paired-single (w119) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80AFEC20` | 88 | clean, not attempted (w119) | **B** | standard teardown |
| `0x80AFEC80` | 92 | clean, not attempted (w119) | **B** | teardown with the extra `cmpi` guard |
| `0x80AFEDD0` | 224 | clean, not attempted (w119) | **B** | stream runs 56 insns to `blr`; 1 direct call (`0x80afedd0`) |
| `0x80AFEEB0` | 136 | clean, not attempted (w119) | **B** | stream runs 34 insns to `blr`; 2 direct calls (`0x802e1c40`, `0x802e1ab4`) |
| `0x80AFEF70` | 48 | clean, not attempted (w119) | **C** | `0x802DCBFC` forwarder |
| `0x80AFEFA0` | 48 | clean, not attempted (w119) | **C** | `0x802DCBFC` forwarder |
| `0x80AFF0D0` | 184 | skipped (w119) | **B** | stream runs 46 insns to `blr`; 6 direct calls (`0x800690c0`, `0x800df180`, `0x80239e30`, ...) |
| `0x80AFF4F0` | 88 | skipped (w119) | **B** | standard teardown |
| `0x80AFFAF0` | 244 | skipped (w119) | **B** | stream runs 61 insns to `blr`; 7 direct calls (`0x800df180`, `0x80239e30`, `0x80169d90`, ...) |
| `0x80AFFC90` | 788 | skipped (w119) | **B** | stream runs 197 insns to `blr`; 9 direct calls (`0x80167b60`, `0x80169f80`, `0x800b8250`, ...) |
| `0x80B04A90` | 512 | clean, not attempted (w115) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80B04E90` | 1592 | clean, not attempted (w115) | **C** | export prints a callee with no arguments |
| `0x80B05740` | 328 | mutex stub (w115) | **C** | export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80B058A0` | 476 | clean, not attempted (w115) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments |
| `0x80B05A80` | 180 | mutex stub (w115) | **C** | export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80B05B80` | 172 | mutex stub (w115) | **C** | export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80B07C90` | 36 | clean, not attempted (w115) | **B** | stream runs 9 insns to `b`; no calls |
| `0x80B07D70` | 252 | clean, not attempted (w115) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80B0AF60` | 76 | skipped (w115) | **B** | calls `0x80B0AC50(actor, 0)` - the second argument is the literal `li r4,0` - then `+0x9B4 = *(u32)result`; the callee-declaration mismatch does not affect the call site |
| `0x80B0C0D0` | 40 | clean, not attempted (w115) | **B** | `0x8008C380(+0x8D4)` then `return 1` |
| `0x80B0C4C0` | 120 | clean, not attempted (w115) | **B** | stream runs 30 insns to `blr`; no calls |
| `0x80B0C540` | 848 | clean, not attempted (w115) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80B0C900` | 1140 | clean, not attempted (w115) | **C** | export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80B0CD80` | 292 | mutex stub (w115) | **C** | export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80B0CEB0` | 288 | mutex stub (w115) | **C** | export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80B0CFD0` | 160 | clean, not attempted (w115) | **B** | stream runs 40 insns to `blr`; no calls |
| `0x80B0D070` | 100 | clean, not attempted (w115) | **B** | sets `+0x674=1`, `0x80166F40(+0x6B4,1,0)`, `+0x770=1`, `+0x6AC=1`, `0x80166F40(+0x6E0,1,0)`, `+0x7A8=1` |
| `0x80B0D0E0` | 100 | clean, not attempted (w115) | **B** | same initialiser with value 3 written to `+0x674`, `+0x770`, `+0x6AC`, `+0x7A8` |
| `0x80B0D150` | 84 | clean, not attempted (w115) | **B** | two `0x80169020` lookups on `+0x640`/`+0x678`, returns 1 only when both succeed |
| `0x80B0D1B0` | 124 | clean, not attempted (w115) | **B** | stream runs 31 insns to `blr`; no calls |
| `0x80B0D2A0` | 212 | GQR/paired-single (w115) | **C** | export prints a callee with no arguments |
| `0x80B0D380` | 328 | GQR/paired-single (w115) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80B0D4D0` | 520 | GQR/paired-single (w115) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80B0D6E0` | 696 | GQR/paired-single (w115) | **C** | incoming `f1` (float argument) plus FPR spill |
| `0x80B0D9A0` | 332 | GQR/paired-single (w115) | **C** | export prints a callee with no arguments |
| `0x80B0DAF0` | 76 | GQR/paired-single (w115) | **B** | if `+0x9E7` is set, `0x80B0CD80(0)`; always clears `+0x9E0` |
| `0x80B0DB50` | 64 | GQR/paired-single (w115) | **C** | export prints a callee with no arguments |
| `0x80B0DB90` | 156 | GQR/paired-single (w115) | **B** | stream runs 39 insns to `blr`; 4 direct calls (`0x80b0cd80`, `0x8006a4b0`, `0x80197f00`, ...) |
| `0x80B0DC40` | 320 | GQR/paired-single (w115) | **B** | stream runs 80 insns to `blr`; 6 direct calls (`0x800b2ee0`, `0x80b0ceb0`, `0x80b0d150`, ...) |
| `0x80B0DD80` | 124 | GQR/paired-single (w115) | **B** | queries `0x80B0D2A0`, compares the returned float against `0x80B2E434`, flips `+0x348`/`+0x9E0`, then `0x80B0D0E0` |
| `0x80B0DE10` | 180 | GQR/paired-single (w115) | **C** | export prints a callee with no arguments |
| `0x80B0F5A0` | 440 | clean, not attempted (w115) | **B** | stream runs 110 insns to `blr`; 16 direct calls (`0x800690c0`, `0x800df180`, `0x80239e30`, ...) |
| `0x80B0F800` | 560 | clean, not attempted (w115) | **B** | stream runs 140 insns to `blr`; 5 direct calls (`0x8006cde0`, `0x800955f0`, `0x8006a4b0`, ...) |
| `0x80B18F20` | 228 | prototype (w113) | **B** | AABB overlap: `0x80162D50(0x19B)`, expands `+0x5A8/+0x5AC/+0x5B0` by `+0x920..+0x92C`, and **tail recurses with `r3` only** |
| `0x80B19EE0` | 104 | skipped (w113) | **C** | export prints a callee with no arguments |
| `0x80B19FA0` | 176 | skipped (w113) | **B** | five calls in a fixed order; each receiver is resolved from `r3`/`r4` in the stream |
| `0x80B1AD30` | 264 | skipped (w113) | **C** | export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80B1B0E0` | 32 | halt_baddata (w113) | **B** | small trampoline; the export's single `halt_baddata` is the tail call `b 0x8015F4F0` |
| `0x80B1B3F0` | 248 | truncated export (w113) | **B** | stream runs 62 insns to `blr`; 7 direct calls (`0x8006cde0`, `0x800955f0`, `0x80072cb0`, ...) |
| `0x80B1D090` | 328 | GQR/paired-single (w113) | **B** | message step; only two psq spill words in the export, the body is scalar and complete |
| `0x80B1D520` | 352 | GQR/paired-single (w113) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80B1DA40` | 180 | GQR/paired-single (w113) | **C** | incoming `f1` (float argument) plus FPR spill |
| `0x80B1DFE0` | 540 | GQR/paired-single (w113) | **B** | stream runs 135 insns to `blr`; 6 direct calls (`0x8006cd90`, `0x8006cde0`, `0x8006cd40`, ...); the only `psq` ops are f31/f30 stack spills; `ldexpf` in the export is the `2^52` integer-to-double idiom |
| `0x80B1E540` | 380 | GQR/paired-single (w113) | **B** | stream runs 95 insns to `blr`; 5 direct calls (`0x8006cd40`, `0x8015f4f0`, `0x800e9020`, ...); the only `psq` ops are f31/f30 stack spills; `ldexpf` in the export is the `2^52` integer-to-double idiom |
| `0x80B23480` | 180 | GQR/paired-single (w110) | **D** | `psq` against GQR4, programmed outside this REL |
| `0x80B23890` | 156 | skipped (w110) | **C** | stream passes the state-table entry in `r6` to `0x801651F0` (the export lost it), so the clip binding is recoverable; blocker is the incoming **f1 double** |
| `0x80B250F0` | 56 | halt_baddata (w110) | **B** | short trampoline to `0x80078470`; the export halt is cosmetic |
| `0x80B25280` | 328 | GQR/paired-single (w110) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80B253D0` | 328 | GQR/paired-single (w110) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80B25D90` | 68 | lost jump table (w110) | **C** | two jump tables, no terminator inside the 68-byte extent |
| `0x80B265A0` | 24 | lost jump table (w110) | **C** | no terminator inside the 24-byte extent |
| `0x80B26A10` | 168 | lost jump table (w110) | **B** | short record helper; complete |
| `0x80B27980` | 168 | GQR/paired-single (w110) | **B** | prop actor step with a single `0x8016xxxx` call and `0x80B2C0A0`-class constants |
| `0x80B28710` | 208 | GQR/paired-single (w110) | **B** | stream runs 52 insns to `blr`; 5 direct calls (`0x802dca98`, `0x80169d20`, `0x80069060`, ...) |
| `0x80B292D0` | 184 | GQR/paired-single (w110) | **B** | stream runs 46 insns to `blr`; 6 direct calls (`0x800690c0`, `0x800df180`, `0x80239e30`, ...) |
| `0x80B2A360` | 324 | GQR/paired-single (w110) | **C** | FPR spill / pair traffic only; export prints a callee with no arguments |

## Ranked next claims

### B rows, largest first

These need only a body and a ledger row; nothing else has to be established
first. Ordering is by reachable instruction count.

| # | address | bytes | body | what the stream shows |
|---:|---|---:|---:|---|
| 1 | `0x80A37830` | 1148 | 287 insns | stream runs 287 insns to `blr`; 11 direct calls (`0x8006cde0`, `0x800955f0`, `0x80072bc0`, ...); the only `psq` ops are f31/f30 stack spills; `ldex... |
| 2 | `0x80AB5710` | 1064 | 266 insns | large per-frame step; four `psq` spill words and four `ldexpf` artefacts in the export, 266 clean instructions in the stream |
| 3 | `0x80AAC870` | 900 | 225 insns | stream runs 225 insns to `blr`; 8 direct calls (`0x80064af0`, `0x801c0bd0`, `0x8016edd0`, ...) |
| 4 | `0x80AFFC90` | 788 | 197 insns | stream runs 197 insns to `blr`; 9 direct calls (`0x80167b60`, `0x80169f80`, `0x800b8250`, ...) |
| 5 | `0x80AD3D50` | 688 | 172 insns | actor step; eight direct calls, no GQR use in the body |
| 6 | `0x80AAC360` | 680 | 170 insns | stream runs 170 insns to `blr`; 8 direct calls (`0x801c0bd0`, `0x8016edd0`, `0x8016a170`, ...); the only `psq` ops are f31/f30 stack spills; `ldexp... |
| 7 | `0x80A7C7D0` | 628 | 157 insns | scratch-matrix builder: a `param_2 == 0x15` path plus a 9-entry byte table at `0x80B2DFB0`; all five callees (`0x801C0500`, `0x801C0540`, `0x801C0B... |
| 8 | `0x80AB1510` | 624 | 156 insns | stream runs 156 insns to `blr`; 16 direct calls (`0x800690c0`, `0x800df180`, `0x80239e30`, ...) |
| 9 | `0x80A36C10` | 608 | 152 insns | stream runs 152 insns to `blr`; 7 direct calls (`0x8016c890`, `0x8006a4b0`, `0x80197f00`, ...) |
| 10 | `0x80AEFCF0` | 600 | 150 insns | stream runs 150 insns to `blr`; 16 direct calls (`0x800690c0`, `0x800df180`, `0x80239e30`, ...) |

### C rows with the highest expected yield

| address | bytes | what blocks it |
|---|---:|---|
| `0x80A3DEA0` | 2940 | FPR spill / pair traffic only; export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80B04E90` | 1592 | export prints a callee with no arguments |
| `0x80A7BEC0` | 1464 | FPR spill / pair traffic only; export prints a callee with no arguments |
| `0x80A9B3C0` | 1196 | export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80A92F40` | 1172 | export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80AAE520` | 1156 | FPR spill / pair traffic only; export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80B0C900` | 1140 | export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |
| `0x80A673D0` | 1080 | FPR spill / pair traffic only; export prints a callee with no arguments; mutex stub `0x802DCDxx` loses its operands |

The C rows that take an incoming `f1`-style argument form their own group
(`0x80B23890`, `0x80AD98B0`, `0x80B1DA40`, `0x80ACDB00`, `0x80B0D6E0`,
`0x80AB1780`, `0x80A2EB40`, `0x80A62A30`, `0x80A6FBC0`, `0x80AA0530`,
`0x80AAF2B0`, `0x80A4B0B0`, `0x80A48940`, `0x80A99360`, `0x80A10880`,
`0x80A372A0`, `0x80AB26A0`): one float-ABI study unlocks all of them.

### D rows that will not move without new evidence

The 51 D rows are dominated by `psq_l/psq_st` against `GQR1..GQR7`
(`0x80B23480`, `0x80A40290`, `0x80A40CA0`, `0x80AFD170`, `0x80A0E580`,
`0x80A1D530`, ...). The quantiser is programmed outside this REL, so the
converted value cannot be typed here. Three rows read a register the decompiler
never defines (`0x80A79810`, `0x80AEC4E0`, `0x80AEEFA0`), and
`0x80A337B0` is a 4-byte `b 0x80A337C0` alias of another unclaimed entry.

## Verification

* 384 addresses, all absent from `ledger/functions.csv`; no ledger row, CSV or
  Rust file was written by this pass.
* Every address has a Ghidra export at
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c` except
  `0x80A337B0`, which Ghidra emits as a 4-byte alias; the three doc
  addresses that are data words were excluded.
* Every B row terminates in `blr` or in a tail call to a real entry: no B row
  branches into the middle of the bucket (checked mechanically).
* Total sizes match `functions.tsv` for every row.
