# Coverage - Wave 76 (`game::bases`, `0x807b` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** under the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader chooses the real load address, so the numbers only
identify positions inside the REL image. Cross-module pointers
(`0x8037xxxx`, `0x8042xxxx`, `0x8031xxxx`, `0x8081xxxx`) are `main.dol`
addresses imported by the REL and keep their absolute values.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`. In the table below,
"role" is the plain-English behaviour, and confidence is `high` when guards,
stores, constants, strings and call arguments are literal, `medium` when the
role of a field or callee had to be inferred (those bodies carry `// observed:`,
`// inferred:` or `// unknown:` notes), and `low` when only the instruction
sequence is certain.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 29 | 4244 | 13 | 15 |

One body is `low` confidence (`fn_807b3f50`), so the high column, the medium
column and that one body account for all 29 rows.

## What this pass covers

The `0x807b` bucket held 161 functions at lane start; 10 were already in
`ledger/functions.csv` (the `dScWorld9DeMo_c` table and destructor, the
`dScCrsin_c` table, constructor and destructor, the `dScGameOver_c` table,
constructor and destructor, and two unnamed constructors), leaving 151. This
pass fills three clusters:

- **Course-in screen members, `0x807B1800..0x807B2ED0`** - fourteen bodies of the
  screen whose state table is [`fn_807b3140`] and whose constructor is
  [`fn_807b1700`]. The class identity is `// inferred:` from that adjacency and
  from the shared `+0x3C4..+0x3C7` player-presence bytes, the `+0x390`/`+0x391`
  arm bytes, the `+0x3B4` countdown, the `+0x3B8` kind word and the layout at
  `+0x74`:
  - the two guarded release wrappers [`fn_807b1800`] / [`fn_807b1840`] and the
    two `+0x18` sub-object variants [`fn_807b1880`] / [`fn_807b18e0`];
  - the armed-actor step [`fn_807b1af0`] (countdown, `0x800C9560` on `+0xB0`,
    virtual `+0x10` of the same member, non-zero fold of `0x800E1BE0`);
  - the archive mount [`fn_807b1c20`] over the `0x80374188` table with the
    `0x80373DC8`/`0x80373D88` name lists;
  - the layout-presence query [`fn_807b1ee0`], the `Stage` path restore
    [`fn_807b1f40`] and its gated transition [`fn_807b1fe0`];
  - the player-count publish [`fn_807b22e0`];
  - the four-player archive mask [`fn_807b29a0`] with the
    `0x801024A0`/`0x8016B410` probe pair and bit 2 of `0x80429F80`;
  - the object-kind effect map [`fn_807b2b10`] over the four `0x3B0`-stride
    player records;
  - the archive re-open [`fn_807b2d40`] and the timer start [`fn_807b2ed0`].

- **Game-over helpers, `0x807B3A10..0x807B4230`** - seven small bodies around
  the `dScGameOver_c` table [`fn_807b4440`]: the three-layer stop
  [`fn_807b3a10`], the input re-enable [`fn_807b3ae0`], the third guarded
  release wrapper [`fn_807b3d70`], the two `+0x18` variants
  [`fn_807b3db0`] / [`fn_807b3e10`], the liveness fold [`fn_807b3f50`] and the
  scene hand-off [`fn_807b4230`].

- **Movie-scene startup tail, `0x807B4A10..0x807B6D60`** - eight bodies:
  - the ordered unmount chain [`fn_807b4a10`] and the archive-`0xD` probe
    [`fn_807b4cd0`];
  - the in-place constructor [`fn_807b4de0`] (base `0x800E19B0`, vtable
    `0x8081FC90`, eight-entry parameter table at `0x8015F600`, the `+0xCC`
    member, the `+0x130` callback array and the `+0x70`/`+0x74` pair) with its
    teardown [`fn_807b4ee0`];
  - the minimal `0x8081FD78` screen constructor [`fn_807b6320`];
  - the object-enable and object-disable handlers [`fn_807b65c0`] /
    [`fn_807b6660`];
  - the named archive opener [`fn_807b6d60`].

Recurring layout facts used by this pass: every screen in this bucket derives
from the `0x800E19B0` base and stores its class vtable at `+0x60`; the NW4R
layout object sits at `+0x74` and receives transitions through its own vtable
slot `+0x18`; the `0x807B` players table is the four `0x3B0`-stride records at
`0x80429E98`; and the resource manager is the pointer at `0x8042A038` whose `+4`
member receives the path/name appends.

## Functions

| address | function | bytes | confidence | role |
|---|---|---:|---|---|
| `0x807B1800` | `fn_807b1800` | 68 | medium | guarded release wrapper |
| `0x807B1840` | `fn_807b1840` | 64 | medium | guarded release wrapper (second copy) |
| `0x807B1880` | `fn_807b1880` | 96 | high | release `+0x18` sub-object then object |
| `0x807B18E0` | `fn_807b18e0` | 100 | high | release `+0x18` sub-object then object |
| `0x807B1AF0` | `fn_807b1af0` | 132 | medium | armed-actor step and liveness fold |
| `0x807B1C20` | `fn_807b1c20` | 484 | medium | stage archive mount and name lists |
| `0x807B1EE0` | `fn_807b1ee0` | 52 | high | layout-presence query |
| `0x807B1F40` | `fn_807b1f40` | 100 | high | `Stage` path restore plus transition |
| `0x807B1FE0` | `fn_807b1fe0` | 80 | high | gated stage-in transition |
| `0x807B22E0` | `fn_807b22e0` | 176 | medium | player-count publish |
| `0x807B29A0` | `fn_807b29a0` | 368 | medium | four-player archive mask |
| `0x807B2B10` | `fn_807b2b10` | 540 | medium | object-kind to effect-index map |
| `0x807B2D40` | `fn_807b2d40` | 364 | medium | archive re-open for all ids |
| `0x807B2ED0` | `fn_807b2ed0` | 152 | high | timer start gate |
| `0x807B3A10` | `fn_807b3a10` | 56 | high | stop three background layers |
| `0x807B3AE0` | `fn_807b3ae0` | 36 | high | re-enable input, return 1 |
| `0x807B3D70` | `fn_807b3d70` | 64 | medium | guarded release wrapper |
| `0x807B3DB0` | `fn_807b3db0` | 96 | high | release `+0x18` sub-object then object |
| `0x807B3E10` | `fn_807b3e10` | 100 | high | release `+0x18` sub-object then object |
| `0x807B3F50` | `fn_807b3f50` | 44 | low | non-zero liveness fold |
| `0x807B4230` | `fn_807b4230` | 48 | medium | scene hand-off `0x800E1E60(3, handle, 0)` |
| `0x807B4A10` | `fn_807b4a10` | 116 | high | ordered unmount chain |
| `0x807B4CD0` | `fn_807b4cd0` | 136 | medium | archive `0xD` open and size probe |
| `0x807B4DE0` | `fn_807b4de0` | 232 | medium | movie-scene constructor |
| `0x807B4EE0` | `fn_807b4ee0` | 88 | high | movie-scene teardown |
| `0x807B6320` | `fn_807b6320` | 60 | high | minimal `0x8081FD78` screen constructor |
| `0x807B65C0` | `fn_807b65c0` | 152 | medium | enable the two mode objects |
| `0x807B6660` | `fn_807b6660` | 152 | medium | disable the two mode objects |
| `0x807B6D60` | `fn_807b6d60` | 88 | medium | named archive opener |

## Address range

`0x807B1800..0x807B6D60`, in three runs:

- `0x807B1800..0x807B2ED0` - fourteen bodies with four gaps
  (`0x807B2050`, `0x807B23B0`, `0x807B2550` and `0x807B27A0` are discussed
  under "Deliberate skips").
- `0x807B3A10..0x807B4230` - seven bodies; the already-translated
  `dScGameOver_c` constructor `0x807B3C70` sits in the middle of the run, and
  the two `0x802DCBFC` thunks `0x807B43E0`/`0x807B4410` close it.
- `0x807B4A10..0x807B4EE0` and `0x807B6320..0x807B6D60` - four bodies each.

They sit between the `dScWorld9DeMo_c` table (`0x807B0C50`, wave 13/15) and the
`dScGameOver_c` table (`0x807B4440`, wave 13), both already translated.

## Deliberate skips

- `0x807B2050` (620) - its export compares `DAT_80315b97 & 1` and then unrolls
  a four-player loop with negative-based globals (`iVar9 + -0x7fc8be30`,
  `-0x7fc8be21`) and overlapping `DAT_80354e50..0x80354ea0` reads and writes
  that Ghidra cannot type; the true array layout is not recoverable from the
  export alone.
- `0x807B23B0` (372) - reads the player record through three pointer levels and
  reuses one local (`iVar8`) for both the base pointer and the record pointer;
  the register-reuse makes the field roles ambiguous.
- `0x807B2550` (548) - formats four 0x20-byte strings from one buffer and then
  walks a `ushort` list with a negative-based table (`-0x7fce57cc`); the table
  identity behind the `Pa3_rail` string compares cannot be verified.
- `0x807B27A0` (456) - writes the unrolled `DAT_80354e50..0x80354e90` block but
  the export interleaves two different loop counters into one register, so the
  per-player assignment cannot be reconstructed honestly.
- `0x807B08A0` (8), `0x807B5C50` (8) - both exports are `halt_baddata()` after a
  "Control flow encountered bad instruction data" warning. Same rejection class
  as `0x80770BE0` and `0x806148F0`.
- `0x807B08B0` (460) - the export reads `unaff_GQR0` and emits `ldexpf`, i.e. a
  paired-single decompilation failure; the float constants cannot be trusted.
- `0x807B0BF0`, `0x807B0C20`, `0x807B43E0`, `0x807B4410` (48/48/48/48) - all four
  exports are the same `func_0x802dcbfc(param_2, param_2, param_1)` thunk with a
  duplicated argument, the `0x802DCBFC` stub family that wave 73 also rejected.
- `0x807B0780`, `0x807B0840`, `0x807B5BF0`, `0x807B5C60`, `0x807B5D20` - their
  exports split a `0x802DCD74` result into an `undefined8` and reconstruct a
  hidden second argument from the high word (`(int)((ulonglong)uVar >> 0x20)`),
  so both the argument list and the string selection are unreliable.
- The rest of the bucket (`0x807B7080..0x807BFFA0`) is untouched, not rejected;
  the `0x807BDF20` paired-single exporter and the `0x807BD110` state table are
  already listed as deferred by earlier waves.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (29 rows, 4,244 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the 10 pre-existing rows of
  this bucket were listed before writing and none is in this pass.
- `node tools/check-lane-csv.mjs wave76` reports
  `wave76-bases.csv: 29 rows` and `OK`.
- `cargo check --message-format=short` produces no diagnostic for
  `src/game/bases.rs`.
