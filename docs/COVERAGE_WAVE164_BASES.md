# Coverage - Wave 164 (`game::bases` 36-40 byte singles)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 10 | 384 | 10 | 0 |

## What this pass covers

The last ten untranslated bodies of 36-40 bytes.

- **`0x80720980`** selects `0x8005CA50(pool, 0)` or `0x8005C930(pool)` by the
  second argument; `pool = *(0x80429C94)`.
- **`0x80754570`** writes `1` at `*(0x80429DD0) + 0x9002C` and dispatches
  slot `+0x20` of the `+0x60` vtable.
- **`0x80759A20`** and **`0x807BE130`** are interface refreshes: the first
  clears `+0x380` and dispatches `+0x26C` slot `+0x18`; the second copies
  `+0x230` into `+0x22C` and calls `0x800B3BE0`.
- **`0x8064A4F0`** and **`0x807193E0`** reset `+0x534`/`+0x5B0` through
  `0x8007F950` and return `1`.
- **`0x806B1AE0`** drops the actor through `0x80064F50(actor, 0)` when its
  `+0xB0` height is below `*(0x80429E00 + 0x3C) - *(0x80429E00 + 0x44)`.
- **`0x8077A000`** indexes `0x80817F38` by `(+0x04 & 0xFF)` and tail-calls
  `0x801006E0`.
- **`0x807AE6A0`** and **`0x807AF940`** call `0x80162A20` and return
  `value == 0` through `cntlzw`/`rlwinm`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80720980` | `fn_80720980` | 36 | high |
| `0x80754570` | `fn_80754570` | 36 | high |
| `0x80759A20` | `fn_80759a20` | 36 | high |
| `0x807BE130` | `fn_807be130` | 36 | high |
| `0x8064A4F0` | `fn_8064a4f0` | 40 | high |
| `0x806B1AE0` | `fn_806b1ae0` | 40 | high |
| `0x807193E0` | `fn_807193e0` | 40 | high |
| `0x8077A000` | `fn_8077a000` | 40 | high |
| `0x807AE6A0` | `fn_807ae6a0` | 40 | high |
| `0x807AF940` | `fn_807af940` | 40 | high |

## Corrections and notes

- `0x80720980` and `0x80759A20` return the incoming `this` on their early-exit
  path (`bclr` leaves `r3` untouched); the Rust signatures return `u32`.
- `0x8077A000` really takes four integer arguments: `r4`/`r5` are overwritten
  before the tail call, while `r6` is forwarded unchanged. The Rust signature
  keeps all four so the ABI matches.
- `0x806B1AE0` uses `fsubs` on the two ground fields and `fcmpo`/`bclr 4,0`;
  the export prints the same `<=` comparison.
- `0x807AE6A0`/`0x807AF940` are 40 bytes because of the frame save/restore; the
  observable work is one call plus `cntlzw`/`rlwinm`.

## Verification

- Every address and byte size matches `work/ghidra/rel-d_basesNP/export/functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/bases.rs`.
- No address was already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave164` prints `OK`.
- `cargo check --message-format=short` and `node tools/doc-lint.mjs src` are clean.
