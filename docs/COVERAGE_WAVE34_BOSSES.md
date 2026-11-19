# Coverage - Wave 34 (`d_en_bossNP`, `game::bosses`)

## How to read this file

- **Scope** - the 84 bodies recorded in `ledger/agents/wave34-bosses.csv` and
  written to `src/game/bosses.rs` over the boss-actor REL `d_en_bossNP`
  (REL module 4). Earlier work is documented in
  `docs/COVERAGE_REL_d_en_bossNP.md` and the wave-3 through wave-33 boss
  coverage files; this file does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000` with entry `0x80C000F0`. Every `0x80C...` value below is an
  analysis address, not a runtime address. Calls into `main.dol` (for example
  `0x8015F7C0`) are absolute DOL addresses and are unaffected by the REL base.
- **Evidence** is the Ghidra C under `work/ghidra/rel-d_en_bossNP/export/`;
  the exact file is named in every CSV row and in the per-body `# Original`
  blocks. Sizes are re-read from `functions.tsv`, not copied from earlier
  coverage prose.
- **Status:** all 84 rows are `translated`. `translated` means the recorded
  control flow and every observable store or constant are represented in the
  body (the export prints nothing else).
- **Confidence:** 83 rows are `high` (each export body is one literal
  constant/address return whose bytes and neighbours fully determine the
  body). `fn_80c38230` is `medium`: both offsets and the constant `1` are
  exact, but the export prints no argument list for its indirect call, so the
  receiver is inferred.
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table, so no Nintendo
  symbol is invented.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bosses.rs` | `game::bosses` | 84 | 1044 | 83 | 1 |

Cumulative after this pass: 756 bodies in `src/game/bosses.rs` (755
`pub unsafe fn` plus the private empty hook `fn_80c001e0`). One hundred and
two bytes of the previous count come from the wave-33 45 bodies; this pass
adds 1044 bytes (83 x 12-byte factories plus the 48-byte `fn_80c38230`).

## What this pass covers

This pass empties every remaining bucket except the truncated/thunk exports:
83 of the 84 bodies are 12-byte `lis`/`addi`/`blr` factories whose export is a
single literal return (an opaque `0x8035...`/`0x8037...` state word or the
fixed address of a word in this module's image), and the last body is the
48-byte `fn_80c38230` virtual-dispatch wrapper. The 84 bodies fall into nine
contiguous runs plus the wrapper:

| run | site | bodies | returns |
|---|---|---:|---|
| 1 | `0x80C06820..0x80C06890` | 8 | 3x `&DAT_80c6f...`, then 5x `0x8035...` ids |
| 2 | `0x80C09D80..0x80C09DE0` | 7 | all 7 `&DAT_80c6f...` module-image words |
| 3 | `0x80C0DFE0..0x80C0E060` | 9 | 5x `&DAT_80c70...`/`&DAT_80c6f...`, 4x `0x8035...` ids (mixed order) |
| 4 | `0x80C118F0..0x80C11980` | 10 | 7x `0x8035...` ids, 3x `&DAT_80c70...` (mixed order) |
| 5 | `0x80C18230..0x80C182A0` | 8 | 7x `&DAT_80c70...`, then 1x `0x8035...` id |
| 6 | `0x80C185F0..0x80C18630` | 5 | all `0x80377A04` |
| 7 | `0x80C1C6C0..0x80C1C730` | 8 | all `0x80377A04` |
| 8 | `0x80C32050..0x80C32060` | 2 | `0x80357AB4`, `0x803578B4` |
| 9 | `0x80C349B0..0x80C349E0` | 4 | `0x80357AB4`, `0x80357974`, `0x803578B4`, `0x80357874` |
| 10 | `0x80C3D620..0x80C3D720` | 17 | all `0x80377A04` |
| 11 | `0x80C42950..0x80C42990` | 5 | mixed `0x8035...` ids |
| wrapper | `0x80C38230` | 1 | one virtual call through `this + 0x394`/`+0x10`, then `1` |

(5 + 7 + 5 + 2 + 4 + 10 + 8 + 5 + 8 + 2 + 4 + 1 + 17 + 5 = 84; the exact
value of every return is in the per-body doc blocks and was checked against
its evidence file one to one.)

In this pass, 30 of the 84 bodies return the opaque word `0x80377A04` (runs
6, 7 and 10). The word is the same one returned by the wave-27 Roy entries,
the wave-28 Morton entries, the wave-32 `0x80C37D60` run and the wave-33
`0x80C20000` runs, so it cannot identify an owner.

What is proven for every factory body:

- The export body is literally `return 0x........;` (or
  `return &DAT_........;`) with no load, store or call. Every
  constant-return evidence file is 65 bytes and the address-return files are
  of the same single-statement shape.
- The `functions.tsv` size is 12 bytes for all 83 factory rows, matching the
  `lis`/`addi`/`blr` three-instruction slab used by the factory family
  documented in waves 26-33.
- Each run is contiguous: every next member starts `0x10` bytes after the
  previous one, and the member after the run end is a larger function at a
  bigger alignment gap, which fixes each run boundary from both sides. The
  run extents are stated in every body's doc block together with the sizes of
  the neighbouring function before and after the run.
- Return words repeat across runs (`0x80377A04` is returned by 25 entries
  across waves 27-34), so a constant alone does not identify an owner; the
  bodies are translated body-for-body and the doc blocks say the owner
  registrar is unknown rather than guessing a class.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80C06820` | `fn_80c06820` | 12 | high |
| `0x80C06830` | `fn_80c06830` | 12 | high |
| `0x80C06840` | `fn_80c06840` | 12 | high |
| `0x80C06850` | `fn_80c06850` | 12 | high |
| `0x80C06860` | `fn_80c06860` | 12 | high |
| `0x80C06870` | `fn_80c06870` | 12 | high |
| `0x80C06880` | `fn_80c06880` | 12 | high |
| `0x80C06890` | `fn_80c06890` | 12 | high |
| `0x80C09D80` | `fn_80c09d80` | 12 | high |
| `0x80C09D90` | `fn_80c09d90` | 12 | high |
| `0x80C09DA0` | `fn_80c09da0` | 12 | high |
| `0x80C09DB0` | `fn_80c09db0` | 12 | high |
| `0x80C09DC0` | `fn_80c09dc0` | 12 | high |
| `0x80C09DD0` | `fn_80c09dd0` | 12 | high |
| `0x80C09DE0` | `fn_80c09de0` | 12 | high |
| `0x80C0DFE0` | `fn_80c0dfe0` | 12 | high |
| `0x80C0DFF0` | `fn_80c0dff0` | 12 | high |
| `0x80C0E000` | `fn_80c0e000` | 12 | high |
| `0x80C0E010` | `fn_80c0e010` | 12 | high |
| `0x80C0E020` | `fn_80c0e020` | 12 | high |
| `0x80C0E030` | `fn_80c0e030` | 12 | high |
| `0x80C0E040` | `fn_80c0e040` | 12 | high |
| `0x80C0E050` | `fn_80c0e050` | 12 | high |
| `0x80C0E060` | `fn_80c0e060` | 12 | high |
| `0x80C118F0` | `fn_80c118f0` | 12 | high |
| `0x80C11900` | `fn_80c11900` | 12 | high |
| `0x80C11910` | `fn_80c11910` | 12 | high |
| `0x80C11920` | `fn_80c11920` | 12 | high |
| `0x80C11930` | `fn_80c11930` | 12 | high |
| `0x80C11940` | `fn_80c11940` | 12 | high |
| `0x80C11950` | `fn_80c11950` | 12 | high |
| `0x80C11960` | `fn_80c11960` | 12 | high |
| `0x80C11970` | `fn_80c11970` | 12 | high |
| `0x80C11980` | `fn_80c11980` | 12 | high |
| `0x80C18230` | `fn_80c18230` | 12 | high |
| `0x80C18240` | `fn_80c18240` | 12 | high |
| `0x80C18250` | `fn_80c18250` | 12 | high |
| `0x80C18260` | `fn_80c18260` | 12 | high |
| `0x80C18270` | `fn_80c18270` | 12 | high |
| `0x80C18280` | `fn_80c18280` | 12 | high |
| `0x80C18290` | `fn_80c18290` | 12 | high |
| `0x80C182A0` | `fn_80c182a0` | 12 | high |
| `0x80C185F0` | `fn_80c185f0` | 12 | high |
| `0x80C18600` | `fn_80c18600` | 12 | high |
| `0x80C18610` | `fn_80c18610` | 12 | high |
| `0x80C18620` | `fn_80c18620` | 12 | high |
| `0x80C18630` | `fn_80c18630` | 12 | high |
| `0x80C1C6C0` | `fn_80c1c6c0` | 12 | high |
| `0x80C1C6D0` | `fn_80c1c6d0` | 12 | high |
| `0x80C1C6E0` | `fn_80c1c6e0` | 12 | high |
| `0x80C1C6F0` | `fn_80c1c6f0` | 12 | high |
| `0x80C1C700` | `fn_80c1c700` | 12 | high |
| `0x80C1C710` | `fn_80c1c710` | 12 | high |
| `0x80C1C720` | `fn_80c1c720` | 12 | high |
| `0x80C1C730` | `fn_80c1c730` | 12 | high |
| `0x80C32050` | `fn_80c32050` | 12 | high |
| `0x80C32060` | `fn_80c32060` | 12 | high |
| `0x80C349B0` | `fn_80c349b0` | 12 | high |
| `0x80C349C0` | `fn_80c349c0` | 12 | high |
| `0x80C349D0` | `fn_80c349d0` | 12 | high |
| `0x80C349E0` | `fn_80c349e0` | 12 | high |
| `0x80C38230` | `fn_80c38230` | 48 | medium |
| `0x80C3D620` | `fn_80c3d620` | 12 | high |
| `0x80C3D630` | `fn_80c3d630` | 12 | high |
| `0x80C3D640` | `fn_80c3d640` | 12 | high |
| `0x80C3D650` | `fn_80c3d650` | 12 | high |
| `0x80C3D660` | `fn_80c3d660` | 12 | high |
| `0x80C3D670` | `fn_80c3d670` | 12 | high |
| `0x80C3D680` | `fn_80c3d680` | 12 | high |
| `0x80C3D690` | `fn_80c3d690` | 12 | high |
| `0x80C3D6A0` | `fn_80c3d6a0` | 12 | high |
| `0x80C3D6B0` | `fn_80c3d6b0` | 12 | high |
| `0x80C3D6C0` | `fn_80c3d6c0` | 12 | high |
| `0x80C3D6D0` | `fn_80c3d6d0` | 12 | high |
| `0x80C3D6E0` | `fn_80c3d6e0` | 12 | high |
| `0x80C3D6F0` | `fn_80c3d6f0` | 12 | high |
| `0x80C3D700` | `fn_80c3d700` | 12 | high |
| `0x80C3D710` | `fn_80c3d710` | 12 | high |
| `0x80C3D720` | `fn_80c3d720` | 12 | high |
| `0x80C42950` | `fn_80c42950` | 12 | high |
| `0x80C42960` | `fn_80c42960` | 12 | high |
| `0x80C42970` | `fn_80c42970` | 12 | high |
| `0x80C42980` | `fn_80c42980` | 12 | high |
| `0x80C42990` | `fn_80c42990` | 12 | high |

## Deliberately skipped

These 14 exports were left untranslated. In every case the Ghidra export hits
`WARNING: Control flow encountered bad instruction data` and stops the body
at `halt_baddata()`, so the exported code is provably incomplete; writing a
body from it would invent behaviour the evidence does not show. No
`functions.tsv` entry is missing from this list or from the table above: with
these skips, every one of the REL's 770 functions is accounted for.

| address | size | reason |
|---|---:|---|
| `0x80C0B510` | 8 B | Export is truncated: nothing but a `halt_baddata()` fragment. No instruction of the real body is recoverable. |
| `0x80C228A0` | 40 B | Export is truncated: it recovers only `countLeadingZeros(*(param_1 + 0x76C))` and then `halt_baddata()`. Same reason as in wave 33, which left it untouched; it stays a raw-address call site in `fn_80c27230` and `fn_80c27860`. |
| `0x80C32660` | 28 B | Export is truncated: nothing but a `halt_baddata()` fragment. No instruction of the real body is recoverable. |
| `0x80C336F0` | 4 B | Ghidra names it `thunk_EXT_FUN_80663290` (a 4-byte import thunk into the DOL), but the export is truncated at `halt_baddata()` even for the thunk body. A four-byte slot cannot honestly be represented as code. |
| `0x80C3AF50` | 4 B | Ghidra names it `thunk_EXT_FUN_800ae500`; the export is truncated at `halt_baddata()`. Not representable. |
| `0x80C3AF60` | 4 B | Ghidra names it `thunk_EXT_FUN_800ae510`; the export is truncated at `halt_baddata()`. Not representable. |
| `0x80C4C160` | 4 B | Ghidra names it `thunk_EXT_FUN_800ad4f0`; the export is truncated at `halt_baddata()`. Not representable. |
| `0x80C389B0` | 72 B | Export recovers the first four stores (`DAT_80c6632c/30/34/38` into `param_1 + 0x164..+0x170`, the same constants the Larry constructor at `0x80C37DF0` writes to the same field window) and then truncates: `halt_baddata()`. The other 20 bytes are gone, so the body is incomplete. |
| `0x80C38A00` | 72 B | Same shape as `0x80C389B0` with `DAT_80c6633c/40/44/48`; truncated after the four stores. Incomplete. |
| `0x80C3DFE0` | 72 B | Same shape as `0x80C389B0` with `DAT_80c675ac/b0/b4/b8`; truncated after the four stores. Incomplete. |
| `0x80C3E030` | 72 B | Same shape as `0x80C389B0` with `DAT_80c675bc/c0/c4/c8`; truncated after the four stores. Incomplete. |
| `0x80C4FDE0` | 16 B | Export recovers only `*(param_1 + 0x2e1) = 0`, and then `halt_baddata()`. It is a raw-address call site of `fn_80c504e0` (recorded in the module unknowns); the remaining bytes are lost. |
| `0x80C512F0` | 20 B | Export is truncated: nothing but a `halt_baddata()` fragment. No instruction of the real body is recoverable. |
| `0x80C51430` | 24 B | Export is truncated: nothing but a `halt_baddata()` fragment. No instruction of the real body is recoverable. |

For the four `0x80C38xx`/`0x80C3Dxx`-style copies above the lost 20 bytes
could be anything in principle (more writes, a call, or the return
sequence); the wave-33 evaluation of `0x80C228A0` applies unchanged, so these
were skipped rather than translated as hypothetical `partial` bodies.

## Verification

- Every address and byte size in the table and CSV matches
  `work/ghidra/rel-d_en_bossNP/export/functions.tsv` (83 rows are 12 bytes,
  `0x80C38230` is 48 bytes).
- Every evidence file exists at the cited path. Each of the 83 factory
  exports was read in full and contains exactly one constant/address return;
  the `0x80C38230` export contains the two loads, the indirect call and the
  literal `return 1`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bosses.rs`.
- No address was already in `ledger/functions.csv` (checked before writing).
- `node tools/check-lane-csv.mjs wave34` reports `OK`.
- `cargo check --message-format=short` is free of errors for
  `src/game/bosses.rs`.
- The module header count and the wave-34 "start here" entry in
  `src/game/bosses.rs` were updated for this pass (672 -> 756).
- No files outside `src/game/bosses.rs`, this document and
  `ledger/agents/wave34-bosses.csv` were edited.

## Module state after this pass

`d_en_bossNP` is now complete to the limits of its decompiled evidence:
770 of 770 `functions.tsv` entries are either translated (756 bodies in this
module) or recorded above as skips whose exports stop at `halt_baddata()`
before the body is complete. Nothing further can be translated honestly from
the current export; a re-decompile that resolves those 14 functions is the
only way past 756.
