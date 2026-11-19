# Coverage - Wave 140 (`game::bases`)

## How to read this file

This page documents the Wave 140 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`**, produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
`# Original` block in the source carries the
`Synthetic base: 0x80600000; this is not a runtime address.` line.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee contract, a field role or a virtual dispatch
had to be named rather than resolved. Comments in the bodies keep
`// observed:` / `// inferred:` / `// unknown:` markers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 27 | 3,576 | 20 | 7 |

All 27 rows are `translated`; none is `partial`. The module header now reports
3575 bodies (3548 + 27).

## What this pass covers

- **The twelve remaining class-C `qr3` bodies of `docs/SPEC_WAVE139_BASES.md`**
  (2,116 bytes). This closes the `qr3` backlog in this module: every
  `psq_l ...,qr3,w1` in `d_basesNP` now has a body. Each load is written
  `read_u16(..) as f32 * 1/256`, the pinned `GQR3 = 0x00050005` (type 5 = `u16`,
  scale 0) from `main.dol`'s init at `0x8025EEEC`.

  | cluster | bodies |
  |---|---|
  | rail-endpoint writers | `0x806B3770` (gate/door pivot pair about a caller position), `0x806FB610` (segmented slot: `+0x6E8` handle table, `+0x884` angle table) |
  | `Wm_ob_keywait` step | `0x8066C9A0` (emitter `+0x71C`, bob `-10.0 * sin`, sound `0x2FE`) |
  | swinging base | `0x806B2D10` (tri-float clear, vfunc `+0x9C`, `+0x3D0`/`+0x3D4` pair) |
  | effect timer twins | `0x806DE0B0`, `0x806DE320` (`+0x724` countdown, interface reset descriptor, `+0x722` phase) |
  | quantised-sine oscillators | `0x806D65A0` (`+0xF02`), `0x807040C0` (`+0xE02`) |
  | height oscillators | `0x80704050` (`+0xE00`/`+0xDFC`), `0x806D6530` (`+0xF00`/`+0xEF8`) |
  | damped height term | `0x806DDA10` (`amp * (1 - cos)`) |

- **Fifteen `0x8062` planner-bucket bodies (1,460 bytes)**, taking that bucket
  from 83 to 68 untranslated addresses:

  | cluster | bodies |
  |---|---|
  | deleting destructors, release only | `0x80624150`, `0x806268C0`, `0x80627D70`, `0x8062B420` |
  | deleting destructors with `+0x18` clear | `0x806241D0`, `0x80626940`, `0x80627DB0`, `0x8062B460` |
  | movie-open record | `0x80627080` (`Wm_2d_movieopen`, 16-word matrix record) |
  | board/cell helpers | `0x80628790` (nine-slot sum), `0x806291F0` (pane release), `0x80629460` (row mode install), `0x8062A040` (release + message `0x7A`) |
  | layer/interface plumbing | `0x80627BC0` (`+0x20C` gate), `0x80624810` (two `0x800C9610` probes) |

## Where the binary differed from the spec prose

1. **`0x80680E80`'s `-1` guard is a signed byte.** The stream is `lbz` + `extsb`
   + `cmpi -1`, which matches the spec's `(i8)` reading and Ghidra's
   `*(char *)`; the same `extsb` feeds the `0x8005F900` lookup.
2. **`0x806DE0B0`/`0x806DE320` step the phase from an unsigned read.** The sine
   argument comes from an `lha` of `+0x722`, but the increment path is `lhz`
   + `addi 0x800` + `sth`; the Rust keeps the `u16` add, which is the low
   halfword of both readings.
3. **`0x806FB610`'s lean helper is given `this`, not the resolved child.** The
   export's `FUN_806fb540(dVar11,iVar5,(int)*(short *)(iVar5 + 0x102))` argument
   order is the shared Gekko ABI quirk; the instruction stream is
   `r3 = this`, `r4 = lha this[0x102]`, `f1 = slot value`, which matches the
   already-translated `fn_806fb540(this, code, value)`.
4. **`0x80627080` writes 15 of its 16 record words.** Word 3 of the stack record
   (`r1+0x14`) is never assigned; Ghidra shows the same hole
   (`// unknown: slot 3`). The record layout is identical to the already
   translated [`fn_80630e60`], so the explicit 16-word copy was reused.
5. **`0x80629460` calls [`fn_80628770`] with `this`.** Ghidra prints
   `FUN_80628770()` with no arguments; `r3` still holds `this` at the `bl`, and
   `0x80628770` is the translated `row = +0xA90 + 5 * +0xA88` helper.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80624150` | `fn_80624150` | 64 | high |
| `0x806241D0` | `fn_806241d0` | 96 | high |
| `0x80624810` | `fn_80624810` | 112 | medium |
| `0x806268C0` | `fn_806268c0` | 64 | high |
| `0x80626940` | `fn_80626940` | 96 | high |
| `0x80627080` | `fn_80627080` | 224 | medium |
| `0x80627BC0` | `fn_80627bc0` | 52 | high |
| `0x80627D70` | `fn_80627d70` | 64 | high |
| `0x80627DB0` | `fn_80627db0` | 96 | high |
| `0x80628790` | `fn_80628790` | 124 | high |
| `0x806291F0` | `fn_806291f0` | 80 | high |
| `0x80629460` | `fn_80629460` | 116 | medium |
| `0x8062A040` | `fn_8062a040` | 112 | high |
| `0x8062B420` | `fn_8062b420` | 64 | high |
| `0x8062B460` | `fn_8062b460` | 96 | high |
| `0x8066C9A0` | `fn_8066c9a0` | 232 | high |
| `0x80680E80` | `fn_80680e80` | 292 | medium |
| `0x806B2D10` | `fn_806b2d10` | 200 | medium |
| `0x806B3770` | `fn_806b3770` | 308 | high |
| `0x806D6530` | `fn_806d6530` | 104 | high |
| `0x806D65A0` | `fn_806d65a0` | 112 | high |
| `0x806DDA10` | `fn_806dda10` | 88 | high |
| `0x806DE0B0` | `fn_806de0b0` | 132 | medium |
| `0x806DE320` | `fn_806de320` | 132 | medium |
| `0x806FB610` | `fn_806fb610` | 296 | high |
| `0x80704050` | `fn_80704050` | 108 | high |
| `0x807040C0` | `fn_807040c0` | 112 | high |

**Totals:** 27 bodies, 3,576 bytes (20 high, 7 medium). The address window runs
from `0x80624150` to `0x807040C0`.

## Notes on inference

- [`fn_80680e80`] is `medium`: the body is complete, but its fourth call is
  `0x80050D40`, an unclaimed `main.dol` spawn helper with no export and no
  ledger row, so the meaning of the kind byte `this[0x374C]` is inferred.
- [`fn_806b2d10`] is `medium`: the class vfunc `+0x60`/`0x9C` is a runtime
  dispatch.
- [`fn_806de0b0`] and [`fn_806de320`] are `medium`: the reset target is the
  `+0x18` virtual of `+0x394`, and both descriptors (`0x8082DDA8`,
  `0x8082DEA8`) sit in the zero-fill segment (`0x80822900`, `filesz = 0`).
- [`fn_80624810`] is `medium`: the two `0x800C9610` probes are literal, but the
  interface virtual and the `0x80825748` descriptor are named from the call
  shape only.
- [`fn_80627080`] is `medium`: every offset is literal; the transform
  post-processing role of `0x800B37E0` is `// unknown:` (same as
  [`fn_80630e60`]).
- [`fn_80629460`] is `medium`: the `+0x28C`/`+0x290` owner fields are inferred
  from the two mutually exclusive mode values (`6`/`4`).
- The eight deleting destructors are `high`: the two guards, the release calls
  and the return copy are the whole body. Their classes are `// unknown:`
  because no table in the export names the slots.
- The remaining rows are `high`: every guard, offset, constant and call target
  is literal.

## Deliberate skips

Read and rejected, with the reason:

- **The fourteen 48-byte `0x802DCBFC` forwarders in this bucket**
  (`0x80621FB0`, `0x80621FE0`, `0x80623A50`, `0x80623A80`, `0x80624E80`,
  `0x80624EB0`, `0x806261E0`, `0x80626210`, `0x80627600`, `0x80627630`,
  `0x8062A260`, `0x8062A290`, `0x8062EFF0`, `0x8062F020`). Ghidra renders each
  as `func_0x802dcbfc(param_2,param_2,param_1)` after a hidden `r12 = r3 + imm`;
  the visible argument words do not survive, so any `platform::call3` rendering
  would invent two of them. Waves 137-139 rejected the same family.
- **`0x80625970` (24 bytes)** is `halt_baddata()` in the export: the control
  flow hits bad instruction data, so there is nothing honest to transcribe.
- **`0x80620570`, `0x80624230`, `0x806269A0`, `0x80627E10`, `0x8062B4C0`**
  (the five 100-byte destructor twins). Each is the 96-byte body already
  translated at `0x806241D0`/`0x80626940`/`0x80627DB0`/`0x8062B460` plus one
  redundant duplicate `this == 0` test (`41820014` immediately after the same
  guard). The clean sibling is translated; the duplicate-guard form was left out
  rather than presented as a new behaviour.
- **`0x80624880` (172) and `0x80624A20` (164)**: both begin
  `bl 0x802DCD70` / `bl 0x802DCDBC` (the `_savegpr_N`/`_restgpr_N` pair), and
  Ghidra models the helper's saved-register pair as a 64-bit return. Recovering
  the real `this` needs a dedicated look at the register bracket.
- **`0x8062C800` (216), `0x8062C770` (132), `0x80626DB0` (704), `0x8062BE20`
  (644), `0x80628F70` (560), `0x8062C0B0` (448), `0x80627350` (136)** and the
  rest of the `0x8062` bucket above `0x8062A040`: each needs its own call graph
  (`0x8062C690`, `0x8062ED70`, `0x8062ED90`, `0x80627240`) and was left for a
  dedicated pass rather than guessed at the tail of this one.
- **`0x807BD110`** (the `dFileSelect_c` 44-entry registration table) is still a
  data table, not a body, and still needs the table generator.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (27 rows, 3,576 bytes),
  re-read from the TSV for this pass rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 27 were checked against
  the ledger's address column before being written, and none is duplicated.
- Each body's last word was re-decoded from `work/rels/elf/d_basesNP.elf` at
  `(address - 0x80600000) + 0x1000`; all 27 end in `0x4E800020` (`blr`) at
  `entry + size - 4`, and no `functions.tsv` row starts inside any body.
- `node tools/check-lane-csv.mjs wave140` reports `OK`: 27 rows, 27 unique
  addresses, 20 `high` and 7 `medium`.
- `cargo check --message-format=short` finishes with no error and no warning
  naming `src/game/bases.rs`.
