# Coverage - Wave 162 (`game::bases` short singles)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 29 | 368 | 29 | 0 |

## What this pass covers

The remaining eight-, twelve-, sixteen- and twenty-byte untranslated singles
after the Wave 161 getters.

- **Aim pairs.** `0x806E8990` and `0x80727180` load `1.0f32` from
  `0x807CBDFC`/`0x807CD278`, pass `this + 0xAC`, and tail-call the shared aim
  routine `0x80097170`.
- **Flag clears.** `0x8068AB10` and `0x80693060` clear bit `0x2` with
  `rlwinm 31,29` and refresh `+0x55C`/`+0x554` through `0x800B8250(x, 4)`.
- **Name tables.** `0x8078A270` and `0x8079B110` forward
  `0x807CFAF0`/`0x807D00A0` with counts `3`/`6` to `0x800F6D60`.
- **Object dispatch.** `0x8079B1C0` loads `+0x1D0`, adds `0x28` and tail-calls
  `0x80169020`; `0x8079BD40` applies it to the global `0x8042A1A0`.
- **RNG fallback.** `0x807422D0` returns the non-null `+0x3E8` pointer,
  otherwise `0x800B2EE0(4)`.
- **Forwarders.** Seven bodies are the identical `mr r3,r4; b 0x801648B0`
  shape: `0x807ABF40`, `0x807ADBF0`, `0x807AEE90`, `0x807B08A0`,
  `0x807B5C50`, `0x807B8EC0`, `0x807BA740`.
- **Predicates and constants.** `0x807C1470` is an exact `value == 3` test
  through `cntlzw`/`rlwinm`; `0x806B4EC0` returns `0`; `0x8077D670` and
  `0x807C5B00` return `1`; `0x8065DCA0` tail-calls `0x800FFF50(7)`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8065DCA0` | `fn_8065dca0` | 8 | high |
| `0x806E0E20` | `fn_806e0e20` | 12 | high |
| `0x80761670` | `fn_80761670` | 12 | high |
| `0x80765180` | `fn_80765180` | 12 | high |
| `0x8078E900` | `fn_8078e900` | 12 | high |
| `0x8079B1C0` | `fn_8079b1c0` | 12 | high |
| `0x8079BD40` | `fn_8079bd40` | 12 | high |
| `0x806E8990` | `fn_806e8990` | 16 | high |
| `0x80727180` | `fn_80727180` | 16 | high |
| `0x80762B20` | `fn_80762b20` | 16 | high |
| `0x8078A270` | `fn_8078a270` | 16 | high |
| `0x8079B110` | `fn_8079b110` | 16 | high |
| `0x8065E440` | `fn_8065e440` | 20 | high |
| `0x8068AB10` | `fn_8068ab10` | 20 | high |
| `0x80693060` | `fn_80693060` | 20 | high |
| `0x807422D0` | `fn_807422d0` | 20 | high |
| `0x80743920` | `fn_80743920` | 20 | high |
| `0x807C1470` | `fn_807c1470` | 20 | high |
| `0x80667CF0` | `fn_80667cf0` | 8 | high |
| `0x806B4EC0` | `fn_806b4ec0` | 8 | high |
| `0x8077D670` | `fn_8077d670` | 8 | high |
| `0x807C5B00` | `fn_807c5b00` | 8 | high |
| `0x807ABF40` | `fn_807abf40` | 8 | high |
| `0x807ADBF0` | `fn_807adbf0` | 8 | high |
| `0x807AEE90` | `fn_807aee90` | 8 | high |
| `0x807B08A0` | `fn_807b08a0` | 8 | high |
| `0x807B5C50` | `fn_807b5c50` | 8 | high |
| `0x807B8EC0` | `fn_807b8ec0` | 8 | high |
| `0x807BA740` | `fn_807ba740` | 8 | high |

## Corrections and notes

- `0x8078E900` and `0x8079BD40` use negative `lwz` displacements on a `0x8043`
  base: `-0x5B58` gives `0x8042A4A8` and `-0x5E60` gives `0x8042A1A0`. Both were
  recomputed from the raw words rather than copied from the printed EA.
- `0x8068AB10`/`0x80693060` use `stwu`, which also advances `r3`; the forwarded
  argument is therefore `this + 0x55C`/`this + 0x554`, not `this`.
- `0x80762B20`/`0x80743920` use `lwzu`, so the dispatched object is
  `this + 0x1AC`/`this + 0x5D0`.
- `0x807422D0`'s export truncates at `halt_baddata`; the full 20-byte body
  continues with `li r3,4; b 0x800B2EE0`, which is why the function accepts the
  size from `functions.tsv`.
- `0x806E8990`/`0x80727180` load two different addresses that both contain
  `0x3F800000` (`1.0f32`).

## Verification

- Every address and byte size matches `work/ghidra/rel-d_basesNP/export/functions.tsv`.
- Every evidence file exists at the cited `work/ghidra/rel-d_basesNP/export/` path.
- Every row has exactly one body in `src/game/bases.rs`.
- No address was already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave162` prints `OK`.
- `cargo check --message-format=short` and `node tools/doc-lint.mjs src` are clean.
