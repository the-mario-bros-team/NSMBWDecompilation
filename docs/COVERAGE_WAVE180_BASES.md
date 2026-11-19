# Coverage - Wave 180 (`game::bases` deferred GQR/`psq` set)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 7 | 2,336 | 7 | 0 |

The seven bodies deliberately skipped during the 300-512 band sweep because
their decompiles are dominated by `psq`/GQR noise. All seven are now
translated, and several standing deferrals are retired.

## Bodies

| address | bytes | lane | body |
|---|---:|---|---|
| `0x806EDE70` | 308 | A | `test_lift`-class sampling: GQR3 sample at `obj+0x80`, `* 0.00390625` through `0x80237BD0`, `curve * -0.03125` into `+0x1384`, clamp `[0.5, 8.0]`, event `0x232`. |
| `0x8064DB70` | 324 | A | Lanczos-style direction picker: seeds `0x808281C8` with `(0,-12)(0,12)(-12,0)(12,0)`, scans four offsets, fills `out` and calls `fn_8064dcc0`. |
| `0x806E3FC0` | 316 | B | Four-slot `test_lift` model loader: `(3.0, 0.5, 1.0)` into the `0xC` triples at `+0x5B4`, `g3d/test_lift.brres`, resource `+0x4A4`, pane block `+0x4B4`. |
| `0x8063B900` | 324 | B | Split-screen result placement writer, the `+0x244` twin of `fn_8063ba50`. |
| `0x806BBB90` | 320 | C | Target-facing update: `+0x6D8` handle, aim step `0x64` through `0x8015F4F0`, angles rebuilt from the GQR3 halfword. |
| `0x8074B8D0` | 320 | D | Random clear-slot picker; returns `f32`; `2^52+2^31` magic at `0x807CE608` cancels exactly. |
| `0x8063BA50` | 324 | E | Per-player result placement writer (the `+0x248` twin). |

## Deferrals retired

- `0x806E3FC0` was the **wave-109** blocker: every `psq_*` is a `qr0`/`W=0`
  scalar `f32` callee-save spill, so the `ldexpf`/`unaff_GQR0` brackets are
  compiler noise.
- `0x8063BA50` was the **wave-81** "paired-single state unrecoverable" body:
  the `psq_st`/`psq_l f31,0x68(r1),0,0` pair is a scalar spill of `0.0`, and
  the `0x8063BB1C..0x8063BB48` staging is dead.
- `0x8074B8D0`'s five `ldexpf` blocks are all `f27..f31` spills.

## Corrections and cross-checks

- `0x806BBB90`'s `0x806BBBFC` is `extsh r4,r3` (X-form source in bits 6-10),
  confirmed against `0x806B9994`, `0x80161364` and Ghidra's short cast;
  `0x8015F4F0`'s `(i16*, target, step)` ABI was read from `main.elf`.
- `0x806EDE70`'s clamp polarity was re-read from raw `0x4080000C`/`0x40810008`
  (`bge`/`ble`) → `[0.5, 8.0]`.
- `0x8064DB70`'s `0x43300000`/`xoris` chain is the `u32→float` idiom; the
  single-precision magic cancels exactly.
- `0x8063B900` and `0x8063BA50` are exact `+0x244`/`+0x248` twins, which
  cross-validates both.
- `0x806E3FC0`'s `0x802DCD5C` is inside the `_savegpr_N` range and preserves
  `r3`, so the actor is `this: usize` rather than a getter.

## Verification

- `cargo check` succeeds with 4,329 public bodies and no duplicate names.
- `node tools/check-lane-csv.mjs wave180`: 7 rows, 7 unique addresses, all high.
- All seven evidence exports exist and every size matches `functions.tsv`.
