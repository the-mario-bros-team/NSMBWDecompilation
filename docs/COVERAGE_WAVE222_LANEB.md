# Coverage: wave 222 lane B (`resource::archive`, `0x802CE9B0..0x802CEAF0`)

## How to read this file

This document covers lane B of wave 222. All four bodies are staged in
`work/wave222/laneB_block.rs` and are appended to `src/resource/archive.rs` at
merge time; no source file was edited by this pass. The machine-readable rows
are in `ledger/agents/wave222-laneB.csv` (`module = resource::archive`).

Every row describes one Ghidra entry point:

- **address** - virtual address of the entry in `main.dol` (image base
  `0x80004000`);
- **name** - the Rust function appended to `src/resource/archive.rs`;
- **role** - one plain sentence;
- **status** - `translated` means the whole recorded control flow is present,
  with calls outside the batch left as absolute-address calls;
- **confidence** - `high` when every load/store/branch/mask was read directly
  from the disassembly; `medium` when a load type, a hidden argument or an
  output's role is inferred;
- **bytes** - the `functions.tsv` size (see the size note below);
- **evidence** - the Ghidra C file under `work/ghidra/main-gb/export/`.

## Summary

| Metric | Count |
|---|---:|
| Functions translated in this batch | 4 |
| Bytes of `functions.tsv` size | 660 |
| Real instruction extent | 652 |
| `translated` | 4 |
| `partial` | 0 |
| `stub` | 0 |
| Confidence `high` | 4 |
| Confidence `medium` | 0 |
| Deferrals | 0 |

## Function table

| address | Rust function | role | status | conf. | bytes | evidence |
|---|---|---|---|---|---:|---:|---|
| `0x802CE9B0` | [`fn_802ce9b0`] | tail-call `0x801C8110(0)`, discarding the caller's argument | translated | high | 8 | `802ce9b0_FUN_802ce9b0.c` |
| `0x802CE9C0` | [`fn_802ce9c0`] | two GX state calls, one four-float range load, then the mode dispatch | translated | high | 124 | `802ce9c0_FUN_802ce9c0.c` |
| `0x802CEA40` | [`fn_802cea40`] | eleven-way render-preset dispatch through the table at `0x80350418` | translated | high | 172 | `802cea40_FUN_802cea40.c` |
| `0x802CEAF0` | [`fn_802ceaf0`] | scale four bytes by an `f32`, truncate, optionally clamp to `255.0` | translated | high | 356 | `802ceaf0_FUN_802ceaf0.c` |

## Size note (prologue + `blr`)

Each extent was checked against the prologue and the closing control transfer,
not just against the `functions.tsv` column:

| address | `functions.tsv` | ending | `end + 4` | note |
|---|---:|---|---|---|
| `0x802CE9B0` | 8 | `b 0x801C8110` at `0x802CE9B4` | `0x802CE9B8` | exact; tail branch, no local `blr`; two pad words before `0x802CE9C0` |
| `0x802CE9C0` | 124 | `blr` at `0x802CEA38` | `0x802CEA3C` | real extent 120; the tsv counts the pad word at `0x802CEA3C` |
| `0x802CEA40` | 172 | `blr` at `0x802CEAEC` | `0x802CEAF0` | exact; next body follows immediately |
| `0x802CEAF0` | 356 | `blr` at `0x802CEC50` | `0x802CEC54` | real extent 352; the tsv counts the pad word at `0x802CEC54` |

Only `0x802CE9C0` and `0x802CEAF0` are off by one pad word; both are *longer*
in the tsv than in the image, and the `# Original` lines keep the tsv values the
lane checker compares against. `0x802CE9B0` is a tail branch, so its 8 bytes are
the whole body even though no `blr` appears.

## Notes that a reader should carry into the merge

1. **This cluster belongs to the `0x802CF050` draw pass.** That caller
   (`work/ghidra/main-gb/export/802cf050_FUN_802cf050.c`) runs
   `FUN_802ce940(owner)`, `FUN_802ce9b0(owner)`, `FUN_802ce9c0(owner)` and
   `FUN_802ce850(owner)` back to back, then walks the owner's `+0x2C`-count
   cell array through `0x802CF1E0` and `0x802C3020(2)`. So these four bodies are
   the "reset a latch, install a `[0,1]x[0,1]` range, select the mode preset"
   prologue of one screen-effect draw. `0x802CE9B0` is called with the owner in
   `r3`, but the body overwrites `r3` with `0` before the tail branch, which is
   why its Rust signature takes no argument.
2. **`0x802CE9C0` descriptor argument.** The stack local at `r1+8` is filled
   from `*(u32 *)(r2 - 0x3BB0)` (`0xFF`) and its *address* is the `r4` argument
   of `0x801C8920`; `r3` is a literal `0`. `0x801C8920` is the callee the
   wave-220/221 lanes documented as `(l, r, b, t, mode, desc)`; here
   `l = 0.0f` (`r2 - 0x3BC0`), `r = 1.0f` (`r2 - 0x3BBC`), and `fmr f3,f1` /
   `fmr f4,f2` pass the same pair again, i.e. `(0, 1, 0, 1)`. All four floats
   and the `0` are literal, so the row is `high` even though the GX field the
   call programs is unnamed.
3. **`0x802CEA40` jump table.** The `bcctr` through `0x8035_0418` is resolved:
   the eleven words are `0x802CEA64/6C/74/7C/84/98/A0/B4/BC/D0/E4`, matching
   the `switch` arms `0..=10` one for one. The bound test is
   `cmpli cr0,0,r0,0xa` plus `bclr 12,1` (branch when *greater*), so modes above
   `10` fall through to the `default` return. Modes `0,1,2,3,5,7,10` select
   `0x802C3020` presets `0x0E,0,2,4,5,3,7`; modes `4,6,8,9` program
   `0x801C8DC0` with `(1,2,3,0)`, `(1,3,3,0)`, `(1,5,1,0)`, `(3,4,5,0)`. No arm
   has a tail beyond its single call.
4. **`0x802CEAF0` is a byte-vector scaler, not a colour unpack.** The export's
   `double param_1` is a single-precision `f1`: every multiply is `fmuls`. The
   four widening pairs are the `CONCAT44(0x43300000, byte)` minus `2^52` idiom
   (`*(f64 *)(r2 - 0x3BB8)`, `0x8042F4C8`), so the Rust bodies convert each byte
   with `f32::from(u8)` and drop the magic double. The exported `(char)` casts
   are `fctiwz` followed by `stb`; the translation uses `as i32 as u8`.
5. **`0x802CEAF0` argument order.** The flag test is `cmpi cr0,5,r5,0`, i.e. the
   third *register* argument. Ghidra calls it `param_4` because `f1` occupies
   the first position in its numbering, so the export's
   `(param_1, param_2, param_3, param_4)` maps to `(f1, r3, r4, r5)` =
   `(scale, dst, src, flag)`. The Rust signature follows the registers.
6. **`0x802CEAF0` clamp direction.** The non-zero-flag arm is
   `if (product < 255.0f) keep product`, i.e. a ceiling, and it reloads
   `*(f32 *)(r2 - 0x3BAC)` before every comparison. The Rust body hoists the
   load and applies the same `if product < ceiling` per channel; NaN products
   take the ceiling in both forms.
7. **`r2` constants resolved.** `SDA2_BASE` is `0x8043_3080`, so this batch
   reads `r2 - 0x3BB0` = `0x8042_F4D0` (`0x000000FF`), `r2 - 0x3BB8` =
   `0x8042_F4C8` (`2^52` as `f64`), `r2 - 0x3BBC` = `0x8042_F4C4` (`1.0f`),
   `r2 - 0x3BC0` = `0x8042_F4C0` (`0.0f`) and `r2 - 0x3BAC` = `0x8042_F4D4`
   (`255.0f`). The bodies read them from the SDA2 band rather than baking in the
   values, matching the rest of the module.

## Verification

The block was appended to a scratch copy of `src/`, `Cargo.toml` and
`Cargo.lock` (`%TEMP%\opencode\wave222laneB`) and checked with
`cargo check --lib`; it exits 0 and the library emits no diagnostic pointing at
any of the appended lines.
