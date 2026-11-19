# Coverage - Wave 216, lane C (`engine::dsys`, the `0x800F33C0..0x800F7494` record/interpolation cluster)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/engine/dsys.rs` | `engine::dsys` | 10 | 2280 | 10 | 0 | 0 |

Status split: 10 `translated`, 0 `partial`, 0 deferred.

Body block: `work/wave216/laneC_block.rs`. Ledger rows:
`ledger/agents/wave216-laneC.csv`.

Scratch verification: the block was appended to a copy of
`src/engine/dsys.rs` in a scratch crate (with `Cargo.toml`/`Cargo.lock`) at
`%TEMP%\opencode\w216_laneC`, and `cargo check --lib` exits `0` (`Finished dev
profile` in 20.48 s on the cold build). The block adds no warnings: all 23
warnings the crate emits are pre-existing ones in `src/ui/menu.rs`, and no line
of `src/engine/dsys.rs` appears in the log.

## How to read this file

* **Address** - virtual entry point in the `DOL` image, not a file offset.
* **Name** - the Ghidra name; the Rust body keeps it as `fn_<address>`. New
  constants in the block are suffixed `_C`.
* **Size** - the `functions.tsv` value, which the lane checker compares
  against. Every one was re-derived from the prologue and the closing `blr`;
  all ten match the tsv exactly, so no entry in this lane is a 16/20-byte
  prologue artefact. The real extents are given per body in the block.
* **Confidence** - `high` everywhere: every address, field offset, stride,
  immediate and branch in these ten bodies is literal in the disassembly. What
  is inferred is only the *purpose* of the fields and of the two float
  constants, marked `// inferred:` in the block.

| address | tsv | extent | entry evidence |
|---|---:|---:|---|
| `0x800F33C0` | 16 | 0x10 | `rlwinm`/`add`/`lwz`/`blr` at `0x800F33CC` |
| `0x800F35E0` | 996 | 0x3E4 | bare-store prologue; `blr` at `0x800F39C0`, 0x0C bytes of padding |
| `0x800F5700` | 40 | 0x28 | two `blr`s (`0x800F5714`, `0x800F5724`), 8 bytes of padding |
| `0x800F5730` | 56 | 0x38 | two `blr`s (`0x800F574C`, `0x800F5764`), 8 bytes of padding |
| `0x800F5A90` | 384 | 0x180 | `_savegpr_24` head; `blr` at `0x800F5C0C` |
| `0x800F5E60` | 176 | 0xB0 | `_savegpr_26` head; `blr` at `0x800F5F0C` |
| `0x800F6390` | 124 | 0x7C | `blr` at `0x800F6408`, 4 bytes of padding |
| `0x800F6B50` | 232 | 0xE8 | three `blr`s (`0x800F6BD0`, `0x800F6C2C`, `0x800F6C34`) |
| `0x800F6C40` | 108 | 0x6C | five `bclr` exits + `blr` at `0x800F6CA8` |
| `0x800F7400` | 148 | 0x94 | `blr` at `0x800F7490`, 4 bytes of padding |

`r13 = 0x8042F680` and `r2 = 0x80433080` throughout:

| expression | address | value / use |
|---|---|---|
| `r13 - 0x5704` | `0x80429F7C` | `G_TEXT_CONTEXT`, **already in the module** and reused |
| `r13 - 0x5544` | `0x8042A13C` | recursion depth guard of `0x800F5A90` |
| `r13 - 0x553C` | `0x8042A144` | five-byte swap scratch of `0x800F5E60` |
| `r13 - 0x5433` | `0x8042A24D` | first shell byte of `0x800F7400` |
| `r13 - 0x5432` | `0x8042A24E` | second shell byte of `0x800F7400` |
| `r13 - 0x663B` | `0x80429045` | first expected byte |
| `r13 - 0x663A` | `0x80429046` | second expected byte |
| `r2 - 0x60CC` | `0x8042CFB4` | float threshold / `0x800F6B50` fallback |
| `r2 - 0x60C0` | `0x8042CFC0` | float "delta is zero" threshold |

Existing definitions reused instead of redeclared: `SDA_BASE`, `R2_BASE`,
`G_TEXT_CONTEXT`, `FN_802E1BD0` (the bounded four-byte compare) and
`FN_802E19F8_C` (the bounded copy). New callees added: `FN_800F9270_C`,
`FN_800F92C0_C`, `FN_8006CC00_C`, `FN_8006CD40_C`, `FN_800F9840_C`,
`FN_800FAB70_C`.

## What this pass covers

The ten bodies are the accessor / projection half of the big record object that
the module already touches: one flat block holding a 0xC0-entry array of
0x1C0-byte records and a 0x70-entry table of 0x30-byte entries at `+0x15018`.

1. **The record/table layout** (`0x800F35E0`, the largest body, 996 bytes of
   pure stores, no calls). The record clears three words at `+0xC`/`+0x10`/
   `+0x14`, the byte runs `+0x18..+0x30` and `+0xD1..+0xE4`, then an inner loop
   of 0x20 sub-entries (stride 0x14) clearing two 0x14-byte runs each
   (`+0x31..+0x44`, `+0xE5..+0xF8`), and finishes at
   `+0x198..+0x1BF`, `+0x1C0`/`+0x1C4`/`+0x1C8` (words), `+0x1CC` (byte),
   `+0x1D0` (word), `+0x1D4` (byte). Two `+0x15018` passes follow: 0x50 entries
   get `0xFFFFFFFF` at `+0x24`/`+0x3C` (name-slot markers), the remaining 0x20
   entries get `0` there and the `99999` sentinel at `+0x2C`/`+0x44`.
   Both markers are read back by the accessors: `+0x24`/`+0x3C` are the "no
   name" ids, `+0x2C`/`+0x44` the "no distance" defaults.
2. **The strided accessors** (`0x800F33C0`, `0x800F5700`, `0x800F5730`). All are
   two-exit leaves: a word getter at `+0x18 + index*4`, and the two
   `+0x188`/`+0x194` and `+0x31`/`+0xE5` variants selected by `flag != 0` on
   `+0x1C0`-strided records. `0x800F5730` returns the *address* of a ten-byte
   name slot; its callers hand it to `0x802E19F8`, which is why it is modelled
   as `u32` although it dereferences nothing.
3. **The name-table walk** (`0x800F5A90`). Two four-byte keys per 0x18-byte
   entry (`+1` and `+5`, read by `0x800F9270`/`0x800F92C0`). It is recursive,
   guarded by the depth counter at `0x8042A13C`: increment, refuse when the
   incremented depth reaches `limit`, scan `manager[+0x10]` entries, and on a
   first-key hit either finish (when the second key also matches and
   `record[+0x14]` equals the live depth) or recurse with the second key as the
   new first key. A successful descent appends the four bytes to the caller's
   list (`record[+0xC]` base, `+0x10` count, five-byte stride, `'0'` stamped at
   `+5`). Both exits decrement the guard.
4. **The list reversal** (`0x800F5E60`). `count / 2` three-way five-byte swaps
   through the global scratch buffer, only when `count > 1`. The loop bound is
   the `rlwinm 1,31,31` / `add` / `srawi 1` halving sequence, which is exactly
   `count / 2` for every non-negative count.
5. **The projection pair** (`0x800F6B50` + `0x800F6C40`). `0x800F6B50` returns
   the projected `y` for one segment endpoint: modes 2/3 interpolate on `z`
   against `this+0xB4`, modes 4/5 on `x` against `this+0xAC`, modes 0/1 return
   `b[1]` and anything else returns the `0x8042CFB4` constant. The
   interpolation is `a[1] + d * |field - a[axis]| / |a[axis] - b[axis]|`, with
   `d` replaced by the constant when `|b[1] - a[1]|` falls below `0x8042CFC0`.
   `0x800F6C40` folds that value into `+0xB0`: a minimum for mode 0, a maximum
   for mode 1, and a gated maximum for modes 2..5 that only fires while
   `+0xEC < 0x8042CFB4`. Both were typed `double` by Ghidra; the disassembly
   shows `fadds`/`fmuls`/`stfs` and a straight `f1` hand-off from
   `0x800F6B50` to `0x800F6C40` at `0x800F6D24`/`0x800F6D30`, so both are
   modelled in `f32`.
6. **The shell predicate** (`0x800F7400`). Two `lbz`/`cmpl` pairs compare the
   shell bytes at
   `0x8042A24D`/`0x8042A24E` with `0x80429045`/`0x80429046`, then the mapped
   state of `*(G_TEXT_CONTEXT + 0x54)` (via `0x800F9840`) must not be 4, 6, 9,
   10 or 11, and `0x800FAB70()` must return neither 0 nor 9. Note that
   `0x800FAB70` is called **twice**; the first result is only tested for zero.

## Decompiler corrections recorded in the block

| body | Ghidra | disassembly |
|---|---|---|
| `0x800F5A90` | `uVar6 = FUN_802dcd68()` then `iVar1 = high word` | `bl 0x802DCD68` is the outlined `_savegpr_24` spill; it returns nothing and leaves `r3..r7` intact, so the object is simply `param_1` |
| `0x800F5E60` | `void FUN_800f5e60(void)` with `extraout_r4` | the record genuinely arrives in `r4`; `r3` is never read. Modelled as `(_first, record)` to keep the register assignment |
| `0x800F6B50` | `double` return, nested `if (1 < param_4 - 4)` | the return is a float in `f1`; the arm order is `mode-2 < 2`, then `mode-4 < 2`, then `mode <= 1` (the `param_3[1]` arm), else the constant |
| `0x800F6C40` | `double param_1` | the value is compared with `fcmpo` and stored with `stfs`; it is `f32` |

## Remaining gaps

None for this lane. Every one of the ten addresses is `translated` with no
deferral. What stays opaque is the interior of the callees
(`0x800F9270`, `0x800F92C0`, `0x802E1BD0`, `0x802E19F8`, `0x8006CC00`,
`0x8006CD40`, `0x800F9840`, `0x800FAB70`) and the meaning of the individual
record fields; their behaviour is called, not reconstructed, which is why the
bodies are `translated` rather than `partial`.
