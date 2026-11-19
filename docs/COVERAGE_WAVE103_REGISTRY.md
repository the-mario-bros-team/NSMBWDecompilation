# Coverage - Wave 103 (resource::registry)

## How to read this file

Every row below is one real `pub unsafe fn` body in
`src/resource/registry.rs`. The address and byte count come from
`work/ghidra/main-gb/export/functions.tsv` (re-read this pass, not copied from
earlier prose); the evidence path is the Ghidra C export that was read before
the body was written. `confidence` matches the `# Confidence` line of the
function's doc block.

This pass is the completion of the `0x80160000` page: wave 97 translated 25 of
the 48 bodies that page still had missing and documented the other 23 as skips.
Nineteen of those 23 are below. Three of wave 97's skip reasons did not survive
a read of the instruction bytes, so this file also records *why* they were
wrong - that is the part a later pass should re-check first.

Three conventions need a short explanation:

- **4-byte leaves.** Eight of the addresses here are four bytes long. Ghidra
  exports the *target's* decompiled body under the thunk's address, which is
  why they looked like duplicates of functions that were already translated.
  The real content is one instruction: three are `blr` (`0x4E800020`) and five
  are a direct `b <target>`. Each doc block quotes the word read from
  `work/dol/main.elf`.
- **Register-save thunks.** `0x802DCD58..0x802DCDBC` are Metrowerks prologue
  and epilogue fragments (`__savegprlr_N` / `__restgprlr_N`), which spill
  `r21..r31` into the caller's frame and never write `r3`/`r4`. Ghidra models
  them as calls returning an `undefined8`, so several bodies looked like they
  consumed `unaff_r2x` or `extraout_r4`/`extraout_f1` values "the decompiler
  could not recover". They are ordinary parameters; the wave-97 skips for
  `0x801645F0`, `0x80165280`, `0x80169600`, `0x80169B20` and `0x8016FD30` were
  caused by this single artifact.
- **Paired single.** Ghidra's `unaff_GQR0` / `ldexpf(...)` blocks are noise for
  every body on this page: each `psq_l`/`psq_st` here either carries a
  load/store type of `0` (single float) or is a plain `f26..f31`
  save/restore. The words are quoted in the CSV note for `0x8016E3A0`. The
  three paired-single skips of wave 97 (`0x80169600`, `0x8016AD30`,
  `0x8016F220`) were therefore not blocked by quantisation.

`0x802DCBFC` needs one more note: it reads its descriptor from `r12`, not from
`r3`, so the `FN_CRITICAL_FREE` convention the sound-bank cluster uses does not
apply when the descriptor is an argument. `fn_80161cc0` is the one body here in
that position and uses the local `w103_call_descriptor` helper, which models the
descriptor as an explicit first argument and says so in its doc block.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/registry.rs` | `resource::registry` | 19 | 4020 | 8 | 11 |

Address range: `0x80160AD0..0x8016FD30`.

## What this pass covers

- **4-byte leaves** (`0x80160AD0`, `0x80161740`, `0x80162110`, `0x801644F0`,
  `0x8016B950`, `0x8016BCB0`, `0x8016E4F0`, `0x8016E8E0`, 8 bodies). Three are
  empty returns. `0x80160AD0` is a tail branch to `memset`; `0x8016B950` and
  `0x8016BCB0` are tail branches to `mount_call_cmd_create` and
  `mount_probe_path`, both translated in this file already; `0x8016E4F0` and
  `0x8016E8E0` reach the heap switch and the frame allocator.
- **Descriptor dispatch** (`0x80161CC0`, 1 body). Three `0x802DCBFC` calls whose
  argument registers the export drops: the register trace (`mr r28,r3`,
  `mr r29,r4`, `mr r12,r5`, `mr r30,r6`) restores the probe / select / commit
  shape, including the `0 -> 3`, `1 -> 2`, else `1` selector.
- **Bank factory** (`0x80162A70`, 1 body). The table index, the two
  `fn_80162a50` stash calls and the vtable slot 0 call. The four arguments come
  from the two call sites at `0x80162B00`/`0x80162B20`, which pass
  `(param_1, param_2 + 0x10)` and `(param_1, 0, param_2, param_3)`.
- **Text-object tables** (`0x801645F0`, 1 body). The initialiser of the four
  globals wave 97's `fn_80164770` reads: render-mode descriptor, viewport
  ratios, two `count * 4` allocations and the `0x80164500` reader view, with
  both tables freed on failure.
- **Scene object rebuild** (`0x80165280`, 1 body). The old node's index scan,
  the `+0x4C`/`+0x40` save, the `0x80248990` rebuild, the `fn_801654b0` commit
  and the `+0x48`/`+0x3C` restore.
- **Matrix blend and entry arrays** (`0x80169600`, `0x80169B20`, `0x80169D90`,
  3 bodies). The three-path per-entry blend with its `0x08`/`0x20`/`0x40` flag
  bits, the `0x4C`-stride bank entry allocator, and the loader that chains
  `0x8016A3A0` into it.
- **Background quad** (`0x8016AD30`, 1 body). The 416/304 half width, the
  orthographic projection over `[-w, w] x [-912, 228]`, the full GX state and
  the twelve FIFO floats.
- **Plane test, float pack and name record** (`0x8016E3A0`, `0x8016EBA0`,
  `0x8016FD30`, 3 bodies). The six-plane box test in single precision, the
  twelve-float packer, and the `0x1D + len` record builder whose
  `extraout_r4`/`extraout_f1` are the `r4` and `f1` arguments.

## Functions

| address | name | role | bytes | status | confidence | evidence |
|---|---|---|---|---:|---|---|---|
| `0x80160AD0` | `fn_80160ad0` | tail branch to `memset` | 4 | translated | high | `work/ghidra/main-gb/export/80160ad0_thunk_FUN_800046b4.c` |
| `0x80161740` | `fn_80161740` | empty leaf | 4 | translated | high | `work/ghidra/main-gb/export/80161740_FUN_80161740.c` |
| `0x80161CC0` | `fn_80161cc0` | four-way descriptor dispatch | 180 | translated | medium | `work/ghidra/main-gb/export/80161cc0_FUN_80161cc0.c` |
| `0x80162110` | `fn_80162110` | empty leaf | 4 | translated | high | `work/ghidra/main-gb/export/80162110_FUN_80162110.c` |
| `0x80162A70` | `fn_80162a70` | build one bank from the factory table | 132 | translated | medium | `work/ghidra/main-gb/export/80162a70_FUN_80162a70.c` |
| `0x801644F0` | `fn_801644f0` | empty leaf | 4 | translated | high | `work/ghidra/main-gb/export/801644f0_FUN_801644f0.c` |
| `0x801645F0` | `fn_801645f0` | create the two text-object tables | 380 | translated | medium | `work/ghidra/main-gb/export/801645f0_FUN_801645f0.c` |
| `0x80165280` | `fn_80165280` | rebuild and re-attach a scene object | 548 | translated | medium | `work/ghidra/main-gb/export/80165280_FUN_80165280.c` |
| `0x80169600` | `fn_80169600` | blend one matrix entry | 1068 | translated | medium | `work/ghidra/main-gb/export/80169600_FUN_80169600.c` |
| `0x80169B20` | `fn_80169b20` | allocate a bank's entry array | 244 | translated | medium | `work/ghidra/main-gb/export/80169b20_FUN_80169b20.c` |
| `0x80169D90` | `fn_80169d90` | load a resource and attach it | 340 | translated | medium | `work/ghidra/main-gb/export/80169d90_FUN_80169d90.c` |
| `0x8016AD30` | `fn_8016ad30` | draw the background quad | 532 | translated | medium | `work/ghidra/main-gb/export/8016ad30_FUN_8016ad30.c` |
| `0x8016B950` | `fn_8016b950` | tail branch to the command factory | 4 | translated | high | `work/ghidra/main-gb/export/8016b950_thunk_FUN_8016b8d0.c` |
| `0x8016BCB0` | `fn_8016bcb0` | tail branch to the path probe | 4 | translated | high | `work/ghidra/main-gb/export/8016bcb0_thunk_FUN_8016bc30.c` |
| `0x8016E3A0` | `fn_8016e3a0` | test a box against six planes | 268 | translated | medium | `work/ghidra/main-gb/export/8016e3a0_FUN_8016e3a0.c` |
| `0x8016E4F0` | `fn_8016e4f0` | tail branch to the heap switch | 4 | translated | high | `work/ghidra/main-gb/export/8016e4f0_thunk_FUN_802b9170.c` |
| `0x8016E8E0` | `fn_8016e8e0` | tail branch to the frame allocator | 4 | translated | medium | `work/ghidra/main-gb/export/8016e8e0_thunk_FUN_802b9df0.c` |
| `0x8016EBA0` | `fn_8016eba0` | pack twelve floats | 68 | translated | high | `work/ghidra/main-gb/export/8016eba0_FUN_8016eba0.c` |
| `0x8016FD30` | `fn_8016fd30` | allocate and fill a name record | 228 | translated | medium | `work/ghidra/main-gb/export/8016fd30_FUN_8016fd30.c` |

All 19 rows are `translated`; none is a stub and none is `partial`.

## Deliberate skips

Four addresses of the bucket stay untranslated. They are the 23 wave-97 skips
minus the 19 rows above:

- **`0x801611C0` (436)** the quadrant-table wrapper. Unchanged from wave 97:
  several `0x80161180` calls are printed with zero or shifted arguments, so the
  numerator/denominator routing per quadrant branch cannot be recovered without
  guessing. The two angle reporters of wave 97 still reach it by address.
- **`0x8016DEC0` (880)** a frame loop whose 29 stack locals are filled by three
  different entry points; which entry initialises which slot is not decidable
  from the export, and the body would be a guess for most of its stores.
- **`0x8016F220` (484)** the four-player update loop. Every call, the two
  r13 globals and the `0.0` reset are readable, but the per-player state machine
  is reached through four table entries whose field roles are not provable, so
  the honest form would be a skeleton rather than a reconstruction.
- **`0x8016FA10` (604)** the text layout loop. It is reconstructible in
  principle (the `0x43300000` unsigned-int bias and the character records are
  visible), but the per-glyph transform mixes four `f26..f31` pairs with three
  table reads and the export elides two of them; landing it would need another
  pass over the glyph record layout.

## Verification

- Every address and byte size matches `functions.tsv` (re-read this pass).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/registry.rs` (419 bodies in the file after this pass).
- No address was already in `ledger/functions.csv`; the 19 rows are exactly the
  23 bucket leftovers minus the four skips above.
- `cargo check --message-format=short` reports no diagnostic for
  `src/resource/registry.rs`.
- `node tools/check-lane-csv.mjs wave103` reports `wave103-registry.csv: 19 rows`
  and an overall `OK`.
