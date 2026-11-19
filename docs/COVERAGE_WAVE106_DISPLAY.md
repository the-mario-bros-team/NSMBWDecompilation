# Coverage - Wave 106 (engine::display)

## How to read this file

`src/engine/display.rs` is the semantic home of the engine-level 2-D
presentation layer. This pass works the last open bucket, `0x80150000`: it
entered with 17 untranslated addresses of its 209 and leaves 10. "Bodies" counts
`pub unsafe fn` functions; "bytes" is the sum of the Ghidra sizes re-read from
`work/ghidra/main-gb/export/functions.tsv` for this pass.

"high" means every store, branch and call is visible in the export. "medium"
means one field role or callee shape had to be recovered from the export plus
the surrounding call sites, and that recovery is spelled out in the body's
`# Confidence` block.

Only `src/engine/display.rs` was edited. `ledger/functions.csv` is read-only and
the coordinator merges this lane's rows.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/display.rs` | `engine::display` | 7 | 648 | 4 | 3 |

## What this pass covers

Four clusters, all in `0x80151A10..0x8015F7B0`:

- **The course reference height** (`0x80151A10`, 96 bytes).
  [`fn_80151a10`] loads the default height from the SDA2 `f32` at `r2 - 0x5658`
  and replaces it with the `+0x104` virtual getter of the object returned by
  `0x8014E9B0(scene)` when `scene + 0x1524` is set. The virtual call reads the
  vtable at `object + 0x60`, which is why the plain `vcall0` helper (vtable at
  `+0x00`) does not apply.
- **The four-slot region counter** (`0x80156C30`, 312 bytes).
  [`fn_80156c30`] takes the `0x802DCD70` context pair as `(scene, region)`,
  counts connected players whose `+0x152C`/`+0x1530` position lies in the four
  region floats, and then applies the byte at `region + 0x10`: mode `0` needs a
  single hit, mode `1` needs every connected slot, and the remaining modes use
  the exact signed-difference idiom around `0x8005FD20`. The result is handed
  back through `0x802DCDBC`.
- **The per-slot effect switch** (`0x80159880`, 224 bytes).
  [`fn_80159880`] compares four stored counters at `wm + 0x3CC + slot*4` with
  the caller's value array, calls [`wm_spawn_1up_effects`] on a rise,
  [`gamescene_hud_hide_panes`] on a drop to zero and
  [`gamescene_hud_show_panes`] on a departure from zero, then stores the value
  and rebuilds the slot transform with `0x800B3B60` and the per-slot id at
  `0x802F59D0`.
- **The four step/callback thunks** (`0x8015F3B0`, `0x8015F410`, `0x8015F470`
  and `0x8015F7B0`, 4 bytes each). Each export is the folded body of its target,
  so each Rust body forwards its arguments to the matching `crate::game::task`
  helper (`crate::game::task::step_int_clamped`, `step_int_clamped_b`,
  `step_float_clamped` and `invoke_task_callback`), the same way wave 105
  handled `0x800D8670`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80151A10` | `fn_80151a10` | 96 | medium |
| `0x80156C30` | `fn_80156c30` | 312 | medium |
| `0x80159880` | `fn_80159880` | 224 | medium |
| `0x8015F3B0` | `fn_8015f3b0` | 4 | high |
| `0x8015F410` | `fn_8015f410` | 4 | high |
| `0x8015F470` | `fn_8015f470` | 4 | high |
| `0x8015F7B0` | `fn_8015f7b0` | 4 | high |

Byte total: `96 + 312 + 224 + 4 + 4 + 4 + 4 = 648`.

## Deliberate skips

10 addresses of the bucket stay untranslated.

**Paired-single / GQR quantisation (3 addresses).** `0x801502E0` (256 bytes),
`0x80156930` (372 bytes) and `0x80156AB0` (372 bytes) are printed with
`unaff_GQR0`, `ldexpf`, `CONCAT44`/`extraout_f1` stubs and
`WARNING: Removing unreachable block` lines around the float work. Any Rust body
would be an invented approximation of `psq_l`/`ps_merge` semantics.

**Varargs frame with condition-register input (1 address).** `0x8015F730`
(128 bytes) begins with an `in_cr1` bit test that conditionally copies eight
doubleword arguments into a `0x80`-byte stack frame, then calls
`0x8015F6E0(param_9, &frame)`. The arguments are unnamed stack slots, so naming
them honestly is not possible.

**External-target thunk (1 address).** `0x8015F130` (4 bytes) branches to
`0x801B8620`, which belongs to another subsystem and has no symbol in this lane.
Its argument list is not visible in this export.

**Budget (5 addresses).** `0x80151E50` (452 bytes), `0x80152CF0` (232 bytes),
`0x801548A0` (464 bytes), `0x80154A70` (648 bytes) and `0x80154E50` (268 bytes)
are clean enough to translate but were left for a later pass after the seven
bodies above had landed.

## Verification

- Every address and byte size matches `functions.tsv`, re-read for this pass.
- Every evidence file exists at the cited path, including the four
  `thunk_FUN_*` exports.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/engine/display.rs`.
- No address was already in `ledger/functions.csv`; the bucket had 17
  untranslated functions before this pass and 10 after it.
- `node tools/check-lane-csv.mjs wave106` reports this lane clean.
- `cargo check --message-format=short` reports no error naming
  `src/engine/display.rs`.
