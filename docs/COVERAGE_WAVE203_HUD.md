# Coverage - Wave 203 (`ui::hud`, main.dol part 1)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/hud.rs` | `ui::hud` | 60 | 10,908 | 46 | 14 |

First 60 main.dol bodies, addresses `0x80020000`-`0x80023F80`.

## What landed

- **Registry/segment bodies (lane A).** `0x80020000` (registry count sum),
  `0x80020020`, `0x80020070`, the 652-byte `0x80020100` (segment advance,
  spark/sound, angle update), `0x800203C0`, `0x80020440`, `0x800204D0`,
  `0x80020640`, `0x80020850`, `0x800208B0`.
- **Child-state bodies (lane B).** `0x80020910` (digit dispatch and
  `0x8009F770` call), `0x80020AA0`, `0x80020BF0`, the `0x80020E70`/`EA0`
  wrappers, plus five 12-byte accessors.
- **Digit-selection bodies (lane C).** `0x80020ED0`, `0x80020F40`,
  `0x80021100`, `0x800212C0`, `0x80021470`, `0x800214F0`, `0x80021580`,
  `0x80021600`, `0x80021740`, `0x80021780`.
- **Burst/emission bodies (lane D).** `0x800217F0`, `0x80021A20`,
  `0x80021AB0`, `0x80021B30`, `0x80021BD0`, `0x80021C80` (544 B),
  `0x80021EA0`, `0x80021F50` (292 B), `0x80022180`, `0x800221E0`.
- **Sound emitters (lane E).** `0x80022370`, `0x80022390`, `0x800223C0`,
  `0x80022490`, `0x80022780`, the 764-byte `0x80022810`, the 684-byte
  `0x80022B10`, `0x800238C0`, `0x80023920`, `0x80023A60`.
- **Model/accessor bodies (lane F).** `0x80023B40`, two interface thunks,
  six 12-byte accessors, and the 296-byte `0x80023F80` (`bros` model setup).

## Corrections and recoveries

- The 12-byte bodies are **not** `blr` stubs: each is
  `lis r3,0x8037; addi r3,r3,0x7A04; blr`, i.e. an accessor returning
  `0x80377A04`, matching the existing `fn_8001cbb0` family (lanes B and F
  independently).
- Manifest sizes for `0x80021C80` and `0x80021F50` were wrong (20 B each); the
  real extents are 544 and 292 by `blr`+prologue, matching `functions.tsv`.
- `0x80022810`/`0x80022B10` are the shared player-sound block (callers include
  `0x80A1C8F0` in `game::enemies`), not layout code; documented as such.
- `0x800217F0`'s `b` is `(u8[+0x68D] - 2) as i8`, clamped to 1 when negative.
- Scan bodies pass the **slot byte** `this + i + 0x668` (sign-extended) to the
  position helper, not the loop index.
- `0x800213xx`-family: `0x80352974` is three 12-slot `0x3C` records, the shape
  `fn_8001f800`/`fn_8001f8a0` already walk.
- `0x800A77A0` needs eight integer arguments; `platform` stops at `call7`, so
  lane E marshals it through a local `extern "C"` pointer.
- `0x80023920`'s second null test is dead (no `cr0` write between branches).

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations; full `scripts/check.ps1` exit 0.
- `check-lane-csv.mjs wave203`: 60 rows, 60 unique addresses, 46 high +
  14 medium.
- Ledger: 19,089 rows (94.26%); `ui::hud` 252 translated.
