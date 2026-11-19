# Coverage - Wave 106 (resource::bres)

## How to read this file

This pass edits only `src/resource/bres.rs`, and adds exactly five bodies to the
end of it. Every row below is one literal reconstruction of a Ghidra body: the
doc block above the Rust body names the original address, the byte count from
`work/ghidra/main-gb/export/functions.tsv`, and the evidence file. Offsets are
relative to the object named in the row.

The `0x80230000` bucket is shared with several subsystems. The Ghidra inventory
lists **251** addresses in it; wave 100 translated 241 of them and recorded the
other ten as refused. **Wave 106 takes five of those ten** and leaves five, each
with its reason under *Deliberate skips*. The bucket now has 246 of 251
translated, and the file as a whole holds **256** translated bodies.

- **high** - every load, store, branch and call is visible in the export and no
  argument had to be recovered.
- **medium** - a field role or callee identity had to be reconstructed from a
  sibling, a value had to be recovered from a prologue stub, or a return value
  is inferred from two call sites.
- **low** - only the instruction sequence is certain; an opaque virtual slot, a
  dispatch veneer or a cursor block had to be modelled.

Inside the Rust file, `// observed:` marks facts read directly from the Ghidra
C, `// inferred:` marks a role reconstructed from neighbours, and `// unknown:`
marks an unresolved field.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/bres.rs` | `resource::bres` | 5 | 6,436 | 0 | 3 |

Two of the five are `low`: the layout engine and the renderer.

## What this pass covers

- **Text-anchor twins** (`0x80230280`, `0x80232C10`, 1 240 bytes each): the
  engine and console versions of "measure a run, move the caller's anchor by the
  alignment bits, publish the pen". Each has two rotated modes that re-measure
  through a rotated-run helper. Both return the measured width in `f1`.
- **Console layout engine** (`0x80231CF0`, 1 468 bytes): walks a two-byte
  character run through the layout object, merges every glyph box into a
  four-float rectangle, wraps at the height limit and publishes the cursor.
- **Per-run renderer** (`0x802323E0`, 2 088 bytes): anchors with `0x80232C10`,
  then feeds the run to the layout object; handles the page-scroll advance for
  printable characters and re-enters the layout engine for the two rotated
  modes.
- **Console frame pump** (`0x802348F0`, 400 bytes): reports the exception
  record, paints, spins on the retrace callback, and - when no callback is
  installed - repaints at a fixed 200 ms pace forever.

## Two decoding facts that unlocked this pass

1. **The prologue stubs are `_savegpr0_nn`, and their pair is `(r4, r3)`.**
   Ghidra models `FUN_802DCD58`..`FUN_802DCD74` as functions returning `r3:r4`,
   with `r4` in the *low* word and `r3` in the *high* word. That is why
   `0x80230280`/`0x80232C10` appeared to read two `float *` they never receive:
   their real signature is `(stream, &x, &y, start, count)`, exactly as the call
   sites `0x8022FA70` and `0x802323E0` show. The same swap explains the
   `puVar6`/`iVar9` split in the renderer (stream in `r3`, text base in `r4`).
2. **`crate::platform::call_u64` exists.** The frame pump needs the 64-bit
   return of `0x801B5F80` and a 64-bit divide through the Metrowerks `__div2u`
   helper. The first is now a platform call; the second is native `u64`
   division, which is exact for the unsigned operands the export builds.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80230280` | `fn_80230280` | 1240 | medium |
| `0x80231CF0` | `fn_80231cf0` | 1468 | low |
| `0x802323E0` | `fn_802323e0` | 2088 | low |
| `0x80232C10` | `fn_80232c10` | 1240 | medium |
| `0x802348F0` | `fn_802348f0` | 400 | medium |

## Deliberate skips

Five of the ten addresses wave 100 left stay untranslated. Every one was
re-read in the export before being skipped again.

- **`0x80235AE0` (716 bytes):** the script parser. Three of its calls lost their
  argument lists entirely (`0x80235A10` appears twice with no arguments), the
  copy loop that fills the caller's buffer is emitted after the `return` and is
  only reachable through a `goto`, and the decompiler keeps `unaff_r13`
  (the small-data base) and `unaff_r30` (an entry counter) as undefined. Its two
  callees `0x80235940`/`0x80235A10` are translated, but the glue between them is
  not recoverable from this export without inventing three argument sets. The
  caller `fn_80235dc0` still reaches it through the `SCRIPT_PARSE` address.
- **`0x802382B0` (360 bytes):** the Euler matrix builder. Its only pointer
  argument is `param_4`, yet the body reads `param_1[0..5]` and `param_2[0..5]`
  as six floats each. Those are the two halves of the *double* parameters'
  save slots, not storage any caller passes (the call sites pass three angles
  and one matrix pointer). Reconstructing it means inventing two source
  pointers, so it stays skipped. `MTX34_SCALE` is still the address call.
- **`0x80238930` (180 bytes):** the instancing wrapper. It forwards five
  `f64` values plus three words to `0x802389F0`. `crate::platform` has no
  double-register call helper and this lane may not edit the shared platform
  layer, so the call cannot be expressed. `G3D_SINE` and `G3D_BILLBOARD` remain
  address calls.
- **`0x802389F0` (1 752 bytes):** the billboard/particle matrix builder. Same
  six-double objection, plus six paired-single inputs, over 3 034 lines of
  export. It is the callee that makes `0x80238930` a shell.
- **`0x802390D0` (688 bytes):** the closest-approach/segment kernel. The export
  is 839 lines of scalar emulation with more than thirty scratch slots reused
  across `psq_st`/`psq_l` lane widths, and the two branches that select the
  near/far parameter rewrite the same temporaries in six places. A transcription
  would be a guess about which lane each scratch slot holds, not a translation.

## Verification

- Every address and byte size matches
  `work/ghidra/main-gb/export/functions.tsv` (re-read from the TSV, not copied
  from prose): 1240, 1468, 2088, 1240, 400.
- Every evidence file exists at the cited repo-relative path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/bres.rs`; the file now holds 256 such bodies.
- No address in `ledger/agents/wave106-bres.csv` was already in
  `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/resource/bres.rs`.
- `node tools/check-lane-csv.mjs wave106` reports `OK` for this lane.

`cargo check` proves the Rust is well-formed and type-correct; it does not run
the target code. The pointer arithmetic models the Wii's 32-bit big-endian
PowerPC and must only execute on a target-compatible runtime with the original
memory image mapped.
