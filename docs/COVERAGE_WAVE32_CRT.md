# Coverage - Wave 32 (Metrowerks / EGG object model)

Pass 5 of the `0x802D0000..0x802E0000` runtime band, salvaged after the model
quota ended the original subagent mid-run. The bodies were already written and
were verified against `functions.tsv` and the evidence files before being
entered in the ledger.

## Summary

| Module | Bodies | Bytes | High | Medium |
|---|---:|---:|---:|---:|
| `runtime::cpp` | 25 | 4,608 | 21 | 4 |

## What this pass covers

- Reference-counted resource registry (`0x802D8140..0x802D8A64`): colour
  write-through, vtable reset, entry construct/destroy, manager registration,
  cache flush and lazy manager creation.
- The four fixed-size element pools of `fn_802d8e70` (strides `0x138`,
  `0x158`, `0xC0`, `0xE4`) with their list headers and vtable tags.
- Register-transfer bridge (`0x802DA040`, `0x802DA6F0`) and the camera-vector
  update (`0x802DAD20`).
- Layout-record mask setters and accessors (`0x802DBC90..0x802DBD50`) plus the
  `double`-to-decimal and rounding helpers used by the formatter.

## Repair performed during salvage

The lane stopped after writing the bodies but before its ledger, coverage file
and three helpers, leaving nine compiler errors:

1. `fn_802d94d0` takes `u32`, so `fn_802d88d0` now casts the object pointer.
2. `fn_80289070` already exists in `src/render/gx_band.rs`; the camera body now
   calls `crate::render::gx_band::fn_80289070` instead of an undefined local
   name.
3. `build_element_list` was reconstructed as a documented host-side helper that
   folds the four inlined pool blocks of `fn_802d8e70` (virtual `+0x60`
   allocation, `0x802DC7E0` element run, `0x80228DD0` list initialisers,
   index stores and `0x80228DF0` links).
4. `call_f64_1` / `call_f64_2` were added next to the existing
   `call_f64_ptr` / `call_f64_i32` helpers for the two `double` thunks
   `0x802E8608` and `0x802E8610`.

None of these helpers invents behavior: each mirrors the call sequence in the
cited Ghidra C, and the thunk wrappers are marked `// inferred:`/`// unknown:`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802D8140` | `fn_802d8140` | 60 | high |
| `0x802D85C0` | `fn_802d85c0` | 380 | high |
| `0x802D8740` | `fn_802d8740` | 116 | high |
| `0x802D87C0` | `fn_802d87c0` | 120 | medium |
| `0x802D8840` | `fn_802d8840` | 120 | high |
| `0x802D88D0` | `fn_802d88d0` | 128 | high |
| `0x802D8950` | `fn_802d8950` | 136 | medium |
| `0x802D89E0` | `fn_802d89e0` | 28 | high |
| `0x802D8A00` | `fn_802d8a00` | 132 | high |
| `0x802D8DE0` | `fn_802d8de0` | 140 | high |
| `0x802D8E70` | `fn_802d8e70` | 1,288 | high |
| `0x802D9630` | `fn_802d9630` | 48 | high |
| `0x802D9670` | `fn_802d9670` | 212 | high |
| `0x802D9750` | `fn_802d9750` | 28 | medium |
| `0x802D9770` | `fn_802d9770` | 8 | high |
| `0x802DA040` | `fn_802da040` | 648 | high |
| `0x802DA6E0` | `fn_802da6e0` | 8 | high |
| `0x802DA6F0` | `fn_802da6f0` | 620 | high |
| `0x802DAD20` | `fn_802dad20` | 168 | medium |
| `0x802DBC90` | `fn_802dbc90` | 8 | high |
| `0x802DBCA0` | `fn_802dbca0` | 64 | high |
| `0x802DBCE0` | `fn_802dbce0` | 8 | high |
| `0x802DBCF0` | `fn_802dbcf0` | 68 | high |
| `0x802DBD40` | `fn_802dbd40` | 8 | high |
| `0x802DBD50` | `fn_802dbd50` | 64 | high |

## Verification

- Every address and byte size above matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in `src/runtime/cpp.rs`.
- None of the addresses was present in `ledger/functions.csv` before this pass.
- `cargo check` exits 0 with no diagnostics from `src/runtime/cpp.rs`.
- `node tools/doc-lint.mjs src` reports 0 violations.
