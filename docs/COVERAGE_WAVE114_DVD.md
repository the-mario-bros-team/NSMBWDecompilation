# Coverage - Wave 114 (runtime::dvd, the last address of the `0x801C0000` page)

## How to read this file

The `runtime::dvd` lane owns the `src/runtime/dvd.rs` file, which is the single
writable file for the `0x801C0000` 64 KiB page of `main.dol`. That page is much
wider than the disc band, so most of its bodies - including the one below - are
not disc code; they are here only because this is the one file assigned to the
bucket.

Confidence labels: `high` means every store, mask and branch in the body is
literal and its role is proven; `medium` means the instruction sequence is
literal but at least one field role, callee identity or data layout is inferred;
`low` means only the store sequence is certain.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/dvd.rs` | `runtime::dvd` | 1 | 976 | 0 | 1 |

## What this pass covers

This pass takes the **one** address that the `0x801C0000` bucket still had open,
`0x801C90A0` (976 bytes). Wave 105 and Wave 106 both skipped it, on the grounds
that the Ghidra export collapses the vertex loop to two FIFO stores while the
raw instruction stream writes twelve. Re-reading the listing in
`work/dol/main.elf` shows the export has two specific defects, both now
documented in the body, and that the function itself is ordinary GX code.

- **The "float inputs" are prologue saves.** The export's parameter list is
  empty but the body reads `in_f19`-`in_f31` and `in_ps19_1`-`in_ps31_1` and
  never initialises them. The listing opens with
  `stfd f31,256(r1)` / `psq_st f31,264(r1)` ... `stfd f19,64(r1)` /
  `psq_st f19,72(r1)` at `0x801C90B0`-`0x801C9114` and restores the same
  registers at `0x801C93F4`-`0x801C9458`. Those are the function's own
  callee-saved FPR saves, so the body has no floating-point arguments at all.
- **The "64-bit call" is a register-save helper.** The export's first statement
  is `uVar16 = FUN_802dcd6c()` and it derives two counts from the 64-bit result.
  `0x802DCD6C` is inside the CodeWarrior `_savegpr` block: the `bl` at
  `0x801C9118` enters `stw r25,-28(r11)` / `stw r26,-24(r11)` / ... and returns at
  `0x802DCD88`; `0x801C945C` enters the matching `_restgpr` block at `0x802DCDB8`.
  Neither transfers control to a callee. The two counts are simply `r3` and
  `r4`, copied by `or r25,r3,r3` (`0x801C9128`) and `or r26,r4,r4`
  (`0x801C9130`) and converted to `f32` with the `0x4330` bit pattern.
- **The body is a UV-sphere emitter.** `r3` (called `lat` here) is the latitude
  band count and `r4` (`lon`) the longitude segment count.
  `0x801C9134`/`0x801C9150`/`0x801C9158` load `pi`, `2pi` and the radius from
  the SDA2 slots `r2-0x4DF4`, `r2-0x4DE0` and `r2-0x4DF8` (`0x8042E28C`,
  `0x8042E2A0`, `0x8042E288` = `3.14159274`, `6.28318548`, `1.0`), so the bands
  span `pi/lat` and the longitudes `2pi/lon`. The four `bl`/`frsp`/`fmuls` groups
  at `0x801C923C`-`0x801C9280` narrow the SDK `sin`/`cos` results to `f32`
  before scaling by the radius.
- **The vertex stream the export lost.** Every longitude emits two vertices -
  the next band at `0x801C92D0`-`0x801C92F0` and the current band at
  `0x801C9350`-`0x801C936C` - as `(cos phi * r sin theta, sin phi * r sin theta,
  r cos theta)` followed by the same direction with the radius divided out.
  When GX selector 13 (texcoord 0 size), read through `fn_801c3e20` *before* the
  tables are saved, was already non-zero, two more floats
  `(j/lon, (i+1)/lat)` (and `(j/lon, i/lat)` for the second vertex) are appended
  at `0x801C9330`/`0x801C9340` and `0x801C93A8`/`0x801C93B8`. That is six or
  eight floats per vertex, which is why the export - which shows only two stores
  - cannot be used alone.
- **The block count.** `rlwinm r30,r0,1,0,30` at `0x801C9210` and
  `rlwinm r5,r30,0,16,31` at `0x801C9274` compute `(lon + 1) * 2` truncated to a
  halfword, the vertex count handed to `fn_801c5570` with the `0x98` (quads)
  command and vertex format 3. There is one such block per latitude band.
- **The state save/restore.** `fn_801c3fe0(0x80390248)` snapshots the selector
  table, `fn_801c4700(3, 0x80390320)` the per-stage table for index 3,
  `fn_801c4070` resets the derived texture fields, selectors 9/10 (and 13 when
  it was enabled) are set through `fn_801c37c0`/`fn_801c40b0`, and
  `fn_801c3a30`/`fn_801c4250` replay both tables after the draw. Because the
  original writes those snapshots into the image's scratch globals at
  `0x80390248`/`0x80390320`, the Rust body names the same addresses as
  `GX_SELECTOR_SAVE`/`GX_STAGE_SAVE` instead of taking caller storage.

`sin`/`cos` are called by address through the existing `fp_sin_f64`/`fp_cos_f64`
helpers (`0x802E84C4`, `0x802E7FBC`), the same ones Wave 106 introduced.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801C90A0` | `fn_801c90a0` | 976 | medium |

Roles that the listing does not prove stay inferred in the doc block: the
`lat`/`lon` naming, and the position/normal/texcoord split of the emitted
floats. The split is well supported (radius-scaled direction followed by the
same direction, `0x98` with vertex format 3, texcoords only when selector 13 is
on) but it comes from GX convention, not from a symbol.

## Deliberate skips

None. The bucket's remaining open address was translated.

Two related addresses are *not* part of this lane and are not claimed here:
`0x802DCD6C`/`0x802DCDB8` are the `_savegpr`/`_restgpr` helpers described above,
not functions to translate, and the SDK `sin`/`cos` bodies live in
`src/engine/main_loop.rs`.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (`801c90a0`, `FUN_801c90a0`, 976 bytes).
- The evidence file exists at
  `work/ghidra/main-gb/export/801c90a0_FUN_801c90a0.c`.
- The row has exactly one `pub unsafe fn fn_801c90a0` body in
  `src/runtime/dvd.rs`.
- No address was already in `ledger/functions.csv`; `801c90a0` was the bucket's
  only untranslated address before this pass.
- `node tools/check-lane-csv.mjs wave114` reports `OK` for
  `ledger/agents/wave114-dvd.csv`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/runtime/dvd.rs`.
