# Coverage - Wave 112 (`game::worldmap`, `0x80120000` bucket close-out)

## How to read this file

Each row is one `pub unsafe fn` body appended to `src/game/worldmap.rs`, named
`fn_` plus its original `main.dol` address. Byte sizes come from
`work/ghidra/main-gb/export/functions.tsv`; every evidence path is the Ghidra C
export the body was transcribed from. Field offsets are literal offsets from
the object pointer. `r13` is `SDA_BASE` (`0x8042F680`) and `r2` is
`SDA2_BASE` (`0x80433080`); the bodies reuse the module's already-verified
`R2_*`/`SDA_*`/`FUN_*` names and the `W102_*`/`W91_*` constant block rather than
declaring duplicate names.

The bucket had exactly four untranslated entries when wave 108 finished. All
four are written here, which closes the `0x80120000` bucket.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 4 | 748 | 0 | 4 |

Every row is `translated`; none is a `stub` or `partial`.

## What this pass covers

The four entries were skipped by waves 102 and 108 because the Ghidra C export
drops an operand in each. Re-reading the matching instructions in
`work/dol/main.elf` (the same image the exports were produced from) resolves
every one of those operands, so the pass covers four small clusters:

- **`0x80121280` - the fall actor's ground/landing pass.** When `+0xEC` is
  negative, it calls `0x8015F140(target = r2 - 0x5A3C, rate = r2 - 0x5A8C,
  limit = r2 - 0x5A94, dead-band = 0.0, &+0xEC)` and snaps `+0xEC` to the
  target when the returned remaining distance is below `r2 - 0x5A48`. It then
  runs `0x8006CD90`/`0x8006CD40`, probes the ground with `0x80075270` into a
  one-float record seeded from `r2 - 0x5B54`, and either releases the actor
  through `0x80162590`/`0x80162510` or reports the landing with `0x8011EB00`.
- **`0x80125D20` - the player-slot/pad-mode query.** Resolves the player with
  `0x80125F10`, reads the pad mode with `0x800CD240`, and returns `0` when the
  mode is `1`, when the mode is `0` and the `r13 - 0x56E0` object's `+0x20`
  byte is clear, or when no player was resolved; otherwise `1`.
- **`0x80125FA0` - the player ride/vehicle query.** Resolves the player with
  `0x80125F10` and returns the `0`/`1` coercion of the player's `+0xE0`
  virtual slot, or `0` when the player is missing.
- **`0x8012DE80` - the animation-point builder.** Reads the model's `0x0B`
  and `0x0E` animation records with `0x800D5790`. When the `+0x3C8` slot is
  clear it copies the `0x0E` point and wraps X; otherwise it averages both
  points at half scale (`r2 - 0x591C`), wraps the averaged X with
  `0x80101AA0`, and lifts Y to `+0xB0 + 0x8012DE20(object)` when controller
  flag `4` is set.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80121280` | `fn_80121280` | 264 | medium |
| `0x80125D20` | `fn_80125d20` | 112 | medium |
| `0x80125FA0` | `fn_80125fa0` | 76 | medium |
| `0x8012DE80` | `fn_8012de80` | 296 | medium |

The four sizes sum to 748 bytes.

## How the previously missing operands were recovered

These are the exact points waves 102/108 called blockers. Each is now a literal
instruction, not an inference:

- **`0x8015F140` dead-band at `0x80121280`.** `0x80121288` executes
  `lfs f4,-0x5B54(r2)` in the prologue, before the `+0xEC` test. The `0x801212A4`
  through `0x801212B0` loads fill `f1`/`f2`/`f3`, and no instruction between the
  prologue and the `bl 0x8015F140` writes `f4`. The fourth argument is therefore
  the `r2 - 0x5B54` slot, which the module already documents as `0.0`. The C
  export omits it because the value never needs a reload at the call site.
- **`0x80125F10` receiver at `0x80125D20`/`0x80125FA0`.** Both functions open
  with a bare `bl 0x80125F10` and neither writes `r3` first, so the callee
  receives the caller's own `r3`. `0x80125670` calls `0x80125FA0(param_1)` with
  an explicit actor, and the translated sibling [`fn_80125cc0`] calls
  `0x80125F10(object)` the same way. The parameter is the forwarded actor.
- **`0x800D5790` buffer overlap at `0x8012DE80`.** The two calls pass
  `r1+0x2C` and `r1+0x20`. Each call writes three words (12 bytes), visible in
  `0x800D5790` as `param_2[0..2]`, so the records span `r1+0x2C..0x37` and
  `r1+0x20..0x2B` and do not overlap. The earlier 20-byte estimate was what made
  the frame look aliased.
- **Operand-less `0x80101AA0` call at `0x8012DE80`.** In the averaging branch
  `f1` is `0.5 * (entry_b[0] + entry_e[0])` from `0x8012DED8..0x8012DF1C` and is
  not reloaded before `bl 0x80101AA0`; the returned value is the X written to
  `out[0]`. The sibling [`fn_8012dfb0`] wraps its X midpoint identically.
- **`0x80162510` argument at `0x80121280`.** `r3` still holds the non-zero
  result of `0x80162590` when the second call is made, so the release target is
  that owner and is forwarded explicitly.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`;
  the four sizes sum to 748 bytes.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path
  (`80121280_FUN_80121280.c`, `80125d20_FUN_80125d20.c`,
  `80125fa0_FUN_80125fa0.c`, `8012de80_FUN_8012de80.c`).
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv`; all four were the remaining
  untranslated entries of the `0x80120000` bucket at the start of the pass.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/worldmap.rs`.
- `node tools/check-lane-csv.mjs wave112` reports `wave112-worldmap.csv: 4 rows`
  and `OK`.

## Deliberate skips

None. The `0x80120000` bucket is closed by this pass: 186 of 186 entries are
now in `ledger/functions.csv`.

[`fn_80125cc0`]: crate::game::worldmap::fn_80125cc0
[`fn_8012dfb0`]: crate::game::worldmap::fn_8012dfb0
