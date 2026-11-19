# Coverage - Wave 89 (resource::registry)

## How to read this file

Every row below is one real `pub unsafe fn` body in
`src/resource/registry.rs`. The address and byte count come from
`work/ghidra/main-gb/export/functions.tsv`; the evidence path is the Ghidra C
export that was read before the body was written. `confidence` matches the
`# Confidence` line of the function's doc block, and `role` is the same
plain-English summary as the module-header table.

The pass only claims what the export shows. Where a value had to be resolved
(the `r2` float table) it was read back from `main.elf`, and the address is
written in the constant's comment. Where a role is inferred, the doc block says
so.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/registry.rs` | `resource::registry` | 27 | 5084 | 6 | 21 |

## What this pass covers

The `0x80160000` bucket still held 101 untranslated bodies when the pass
started (421 total, 320 already done). This pass takes 27 of them and leaves 74
documented skips, listed at the end.

- **LCG random helpers** (`0x80161460`, `0x80161490`, `0x801614E0`, 3 bodies).
  Both constants are literal in `main.elf`: `0x80429448` = `1664525` and
  `0x8042944C` = `1013904223`, i.e. the classic `state * a + c` generator behind
  `r13 - 0x6238`/`r13 - 0x6234`. The integer variant returns
  `high32(new_state * range)`; the two float variants force the exponent
  (`>> 9 | 0x3F800000`) and subtract the `1.0f` at `0x8042DC28` or the `1.5f` at
  `0x8042DC2C`, giving `[0,1)` and `[-0.5,0.5)`.
- **Tween / interpolator cluster** (`0x801636C0`, `0x801637F0`, `0x801638E0`,
  `0x80168C10`, 4 bodies). The mode byte at `+0x14` selects wrap, clamp and
  sawtooth behaviour in the advance step; `0x801637F0` is the matching
  "finished" predicate; `0x801638E0` constructs the tween and parks the
  `0.7307692` (19/26) gain at `+0x74` when `0x801DD1D0` reports display mode 1;
  `0x80168C10` rebuilds the three cubic coefficients from `1/sum`.
- **Message and request wrappers** (`0x80161060`, `0x801610B0`, `0x8016C400`,
  `0x8016C890`, `0x8016CAB0`, `0x8016CD40`, 6 bodies). These are the two-step
  message composition, the five-word layout submission with its `0x20`-byte
  handle, the 0x138-byte stack request, the `+0x9C` predicate/packet path and the
  `0x1C`-byte registered request object.
- **List and slot maintenance** (`0x80164A40`, `0x8016C5C0`, `0x80163FF0`,
  3 bodies). One auxiliary-slot release (single slot, or every slot plus the
  table free when the index is negative), the retire pass over the same
  `0x80377C3C` list that `fn_8016c4a0` empties, and the name scan over the
  `+0x14` array with an 8-byte stride.
- **`nw4r::lyt` draw twins** (`0x80168480`, `0x801685E0`, `0x80168710`,
  `0x80168880`, `0x801689B0`, 5 bodies). Two submit callees (`0x8023D3C0` and
  `0x8023D520`) times two shapes (one child by index, every child from the
  `+0xE8` array), plus the child-array accessor they share.
- **Actor and pane leaves** (`0x80161AD0`, `0x80164FD0`, `0x80169F80`,
  `0x8016AB90`, `0x8016D8A0`, `0x8016E230`, 6 bodies). The actor constructor with
  its serial counter and three intrusive list headers, the deferred-load reader
  factory, the pane attach, the `0x80329700` constructor, the record-tail clear,
  and the six-plane containment test.

One translator note applies to the whole pass and is repeated in the section
banner:

- `FUN_802dcd5c/64/68/6c/70/74` are **prologue fragments** (they save
  `r21..r31` and fall through), so the `undefined8` the decompiler shows as their
  return is the enclosing function's own incoming `r3:r4`. `hi` is the first
  parameter and `lo` the second; the bodies spell those two out instead of
  modelling a 64-bit call. The matching `0x802DCDB0..BC` epilogue fragments are
  not emitted as calls. This is the wave-78 rule plus the correction that makes
  the group tractable.

## Functions

| address | function | bytes | confidence | role |
|---|---|---:|---|---|
| `0x80161060` | `fn_80161060` | 80 | medium | two-step message composition |
| `0x801610B0` | `fn_801610b0` | 100 | medium | build a context, then compose it |
| `0x80161460` | `fn_80161460` | 40 | high | LCG step + `mulhi` scale |
| `0x80161490` | `fn_80161490` | 68 | high | LCG step, scale to `[0,1)` |
| `0x801614E0` | `fn_801614e0` | 68 | high | LCG step, scale to `[-0.5,0.5)` |
| `0x80161AD0` | `fn_80161ad0` | 368 | medium | construct an actor node |
| `0x801636C0` | `fn_801636c0` | 148 | high | advance a looping/clamped tween |
| `0x801637F0` | `fn_801637f0` | 96 | high | tween end-of-range predicate |
| `0x801638E0` | `fn_801638e0` | 184 | medium | construct a tween with three constants |
| `0x80163FF0` | `fn_80163ff0` | 132 | medium | find an entry by name |
| `0x80164A40` | `fn_80164a40` | 204 | medium | release one auxiliary slot |
| `0x80164FD0` | `fn_80164fd0` | 244 | medium | create the deferred-load object |
| `0x80168480` | `fn_80168480` | 156 | medium | `index`th child of an element |
| `0x801685E0` | `fn_801685e0` | 304 | medium | draw one child with a tint |
| `0x80168710` | `fn_80168710` | 364 | medium | draw all children with a tint |
| `0x80168880` | `fn_80168880` | 304 | medium | draw one child, second path |
| `0x801689B0` | `fn_801689b0` | 364 | medium | draw all children, second path |
| `0x80168C10` | `fn_80168c10` | 148 | medium | step a cubic tween |
| `0x80169F80` | `fn_80169f80` | 116 | medium | attach a pane to its parent |
| `0x8016AB90` | `fn_8016ab90` | 68 | medium | construct with the `0x80329700` vtable |
| `0x8016C400` | `fn_8016c400` | 160 | medium | submit one layout request |
| `0x8016C5C0` | `fn_8016c5c0` | 300 | medium | retire finished list entries |
| `0x8016C890` | `fn_8016c890` | 132 | medium | stack-allocate and dispatch |
| `0x8016CAB0` | `fn_8016cab0` | 176 | medium | dispatch one registered request |
| `0x8016CD40` | `fn_8016cd40` | 352 | medium | register a request object |
| `0x8016D8A0` | `fn_8016d8a0` | 52 | high | clear the record tail |
| `0x8016E230` | `fn_8016e230` | 356 | medium | oriented-box containment test |

Address range: `0x80161060..0x8016E230`. All 27 rows are `translated`; none is a
stub and none is `partial`.

## Deliberate skips

74 bodies in the bucket stay untranslated. The groups below are exact: together
they are the 74 addresses that are neither in `ledger/functions.csv` nor in this
pass's CSV.

- **4-byte thunks (9)** `0x80160AD0`, `0x80161740`, `0x80162110`, `0x801644F0`,
  `0x8016B950`, `0x8016BCB0`, `0x8016C820`, `0x8016E4F0`, `0x8016E8E0`: below
  the size floor, as in waves 45/52/64/78.
- **GQR / `ldexpf` exports (15)** `0x80160D10`, `0x80161400`, `0x80168F10`,
  `0x80169020`, `0x801690E0`, `0x80169600`, `0x80169B20`, `0x8016AD30`,
  `0x8016DEC0`, `0x8016E3A0`, `0x8016EBF0`, `0x8016ED10`, `0x8016EE30`,
  `0x8016F220`, `0x8016FA10`: the export carries `ldexpf` recovered from a
  `ps_*` GQR pair, so the arithmetic would have to be invented.
  `0x8016ED10`/`0x8016EE30` stay reachable from the wave-78 formatters through
  their existing address constants.
- **Float arguments or float returns (19)** `0x80160E80`, `0x80160EA0`,
  `0x80161180`, `0x801611C0`, `0x801613A0`, `0x80163220`, `0x80163760`,
  `0x801643B0`, `0x801654B0`, `0x801659C0`, `0x80165AE0`, `0x801665C0`,
  `0x801666E0`, `0x801671B0`, `0x801672D0`, `0x80167E50`, `0x80168BD0`,
  `0x80168FE0`, `0x8016EBA0`: the first arguments travel in `f1..f3` (or the body
  reads a float the callee returns in `f1`, or it is an eight-float matrix copy),
  and `crate::platform` can only set integer argument registers.
- **Both halves of the prologue pair are needed (22)** `0x80163520`,
  `0x80163E60`, `0x801641B0`, `0x80164500`, `0x801645F0`, `0x80164770`,
  `0x801650D0`, `0x80165280`, `0x801658A0`, `0x80165C30`, `0x80165FD0`,
  `0x801664A0`, `0x80166830`, `0x80166BD0`, `0x80167080`, `0x80167420`,
  `0x80167800`, `0x80167C40`, `0x80169290`, `0x80169440`, `0x80169D90`,
  `0x8016D640`: here the enclosing function's `r3` **and** `r4` are both required
  downstream - one as a heap, one as a key, or both as arguments of a six- to
  ten-argument callee - and the two-word stack block some of them build is not
  recoverable from the export.
- **Indirect calls whose receiver/argument list is elided (6)** `0x80160800`,
  `0x80161F60`, `0x80162A70`, `0x80167D60`, `0x8016DAA0`, `0x8016DAE0`: the
  export shows `(**(code **)(...))()` with the receiver or the arguments
  missing. Adding them back would be a guess, so these are left out rather than
  half-written.
- **One-off blockers (3)** `0x80161CC0` calls `0x802DCBFC` with zero, one and
  two arguments in the same body and uses `r3` as both input and output;
  `0x801693F0` is a ten-argument forwarder and `crate::platform` stops at
  `call7`; `0x8016FD30` consumes `extraout_r4`/`extraout_f1`, i.e. register
  values the decompiler could not recover at all.

Three earlier decisions are intentionally reversed, each because the current
export is clearer than the note it inherited:

- **`0x8016AB90`** - wave 78 left it out because its base constructor is called
  with an empty argument list. The rest of the body is three literal steps, so it
  is translated and the elided argument list is documented as such.
- **`0x80168480`** - wave 78 called it "a two-word local whose role is
  unresolved". `0x8023B860`/`0x8023B8C0` take the **address of one word** and
  dereference it, so the local is a single pointer to the `+0xE8` array object.
- **`0x80161AD0`** - wave 78 listed it as an "ambiguous dispatcher". It is a
  plain field-by-field constructor; every store is literal.

## Verification

- Every address and byte size matches `functions.tsv` (re-read this pass, not
  copied from earlier prose).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/registry.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no diagnostic for
  `src/resource/registry.rs`.
- `node tools/check-lane-csv.mjs wave89` reports `OK` (27 rows for this lane).
