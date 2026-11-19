# Coverage - Wave 163 (`game::bases` 4-36 byte band)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 35 | 580 | 33 | 2 |

## What this pass covers

- **Eleven empty bodies (44 bytes).** `0x806001E0`, `0x806001F0`,
  `0x806BB220`, `0x8072C1F0`, `0x80750DA0`, `0x807622A0`, `0x8076B090`,
  `0x8076BA60`, `0x8077D660`, `0x80792A10`, `0x8079EF00` are single `blr`
  instructions.
- **Seven thunks (28 bytes).** `0x80667DD0`, `0x806E3370`, `0x80741CA0`,
  `0x80776C00` and `0x80773200` forward to REL bodies; `0x807812D0` and
  `0x80792EE0` forward to `main.dol`.
- **Seventeen singles (508 bytes).** The `0x800B3980` signed-byte scale,
  `0x800D8B00` effect spawns, `0x800F6710`/`0x8016C890` table calls, the
  `0x801038F0` replay, the `1.0f32` triple store, the `+0x724 == 0.0` test and
  the 24-byte `0x800B3B60` animation start.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806001E0` | `fn_806001e0` | 4 | high |
| `0x806001F0` | `fn_806001f0` | 4 | high |
| `0x806BB220` | `fn_806bb220` | 4 | high |
| `0x8072C1F0` | `fn_8072c1f0` | 4 | high |
| `0x80750DA0` | `fn_80750da0` | 4 | high |
| `0x807622A0` | `fn_807622a0` | 4 | high |
| `0x8076B090` | `fn_8076b090` | 4 | high |
| `0x8076BA60` | `fn_8076ba60` | 4 | high |
| `0x8077D660` | `fn_8077d660` | 4 | high |
| `0x80792A10` | `fn_80792a10` | 4 | high |
| `0x8079EF00` | `fn_8079ef00` | 4 | high |
| `0x80667DD0` | `fn_80667dd0` | 4 | high |
| `0x806E3370` | `fn_806e3370` | 4 | high |
| `0x80741CA0` | `fn_80741ca0` | 4 | high |
| `0x80776C00` | `fn_80776c00` | 4 | high |
| `0x80773200` | `fn_80773200` | 4 | high |
| `0x807812D0` | `fn_807812d0` | 4 | medium |
| `0x80792EE0` | `fn_80792ee0` | 4 | medium |
| `0x80625970` | `fn_80625970` | 24 | high |
| `0x80651590` | `fn_80651590` | 24 | high |
| `0x807211D0` | `fn_807211d0` | 24 | high |
| `0x8076FEF0` | `fn_8076fef0` | 24 | high |
| `0x8066C090` | `fn_8066c090` | 28 | high |
| `0x806BAEB0` | `fn_806baeb0` | 28 | high |
| `0x806D1ED0` | `fn_806d1ed0` | 28 | high |
| `0x807882E0` | `fn_807882e0` | 28 | high |
| `0x807C1200` | `fn_807c1200` | 28 | high |
| `0x807C1380` | `fn_807c1380` | 28 | high |
| `0x806B9720` | `fn_806b9720` | 32 | high |
| `0x80783590` | `fn_80783590` | 32 | high |
| `0x806998D0` | `fn_806998d0` | 36 | high |
| `0x806D0590` | `fn_806d0590` | 36 | high |
| `0x806D21A0` | `fn_806d21a0` | 36 | high |
| `0x806D38D0` | `fn_806d38d0` | 36 | high |
| `0x806D6E30` | `fn_806d6e30` | 36 | high |

## Corrections and notes

- The thunk exports use the `_thunk_FUN_<target>`/`_thunk_EXT_FUN_<target>`
  filenames; the lane CSV cites those exact paths.
- `0x80651590`'s export truncates at `halt_baddata`. The full 24-byte body is
  `lwz`, `cmpi`, `bc` with a `li r3,0; blr` return and a tail branch to
  `0x80162D00`.
- `0x8076FEF0` passes `0x80430000` in `r6`. This is not a mistake: `r6` is the
  `lis` base used for the `lwz` and is never overwritten, so the callee really
  receives that constant as its fourth argument.
- `0x806D1ED0` uses `lbz` + `extsb` on `+0x430` and the constants at
  `0x807CB4F0`/`0x807CB4F4`, which are exactly `5.0f32`/`1.0f32`.
- `0x807C1380`'s `fcmpu`/`mfcr`/`rlwinm` sequence is an equality test against
  `0.0f32`; the export prints the same result.
- `0x807C1200` contains a dead `li r0,0` overwritten by the `lfs`; the Rust
  body stores the loaded `1.0f32` bits three times.
- `0x80783590` uses `call_f32_u32x3` because the callee takes `f32, r3, r4, r5`.

## Verification

- Every address and byte size matches `work/ghidra/rel-d_basesNP/export/functions.tsv`.
- Every evidence file exists at the cited path (including the `_thunk_` names).
- Every row has exactly one body in `src/game/bases.rs`.
- No address was already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave163` prints `OK`.
- `cargo check --message-format=short` and `node tools/doc-lint.mjs src` are clean.
