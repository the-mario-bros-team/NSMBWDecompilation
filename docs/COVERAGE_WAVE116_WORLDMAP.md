# Coverage - Wave 116 (`game::worldmap`, `0x80140000` bucket close-out)

## How to read this file

Each row is one `pub unsafe fn` body appended to `src/game/worldmap.rs`, named
`fn_` plus its original `main.dol` address. The byte size comes from
`work/ghidra/main-gb/export/functions.tsv`; the evidence path is the Ghidra C
export the body was transcribed from. `r2` is `SDA2_BASE` (`0x80433080`); the
body reuses the module's existing `flag_test`/`flag_set`/`flag_clear`,
`vcall60_1`/`vcall60_2`, `read_f32`/`write_f32`, `FUN_8004DD00` and
`FUN_8005E3B0` helpers and adds only the `f32`-pair vtable helper and the
`r2`/message constants listed below.

The bucket had exactly one entry with no body at the start of this pass. That
entry is written here, which closes the `0x80140000` bucket (177 of 177 entries
in `ledger/functions.csv`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 1 | 812 | 0 | 1 |

The row is `translated`/`medium`; no part of the body is a stub or partial.

## What this pass covers

- **`0x801469D0` - the player's action-entry dispatcher (812 bytes).** Three
  guards short-circuit to `0`: `0x8004DD00(this)` must return zero, status bit
  `4` must be clear, and not both bit `3` and bit `0x53` may be set. The body
  then sends `(0.5 * f1, 0.5 * f2)` to vtable slot `+0x410`; a non-zero reply
  returns `1` immediately. Otherwise it calls `+0x40C` with `0`, stores `f2` at
  `+0x10C` and enters the entry state machine.
- **Entry state machine.** When status bit `0x19` is already set the body sends
  `0x80376370` with argument `0` and jumps straight to the tail. Otherwise it
  derives `arm` from (`f1 <= 0.0`, bit `0x80`, `+0x10DC` bit 0), then
  re-derives `arm`/`follow` from bits `0x2B`, `0x0A` and `0x0F`. It sends
  `0x80375CAC` with argument `0` when `f1 == 0.0`, or `0x80375C6C` with the
  sixteen-byte payload `[0x803089B0, f1, r6, 1]`; it then sets bits `0xAB` and
  `0x88` and sends `0x803762F0` (`1`/`2` from `r6`) or `0x80375D6C` (`1`).
- **Tail.** Clears bit `0xA7`, reads `+0x10D4`, writes state byte
  `+0x1569 = 4`, clears `+0x10D4` bit 0, and re-arms the `+0xEA4`
  sub-controller: mode `1` plus bit `0xA7` when `r4 == 0`, mode `3` when
  `r5 == 1` and mode `4` when `r5 == 2`. Returns `1`.
- **Constants.** `r2 - 0x591C` = `0x8042D764` is `0.5f` (`0x3F000000`) and
  `r2 - 0x5928` = `0x8042D758` is `0.0f` (`0x00000000`), both read from
  `work/dol/main.elf`. The five message ids are offsets from the `0x80375C20`
  base the body loads into `r31`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801469D0` | `fn_801469d0` | 812 | medium |

## Why earlier waves skipped it, and what resolves it

Waves 95, 101 and 107 listed `0x801469D0` as a deliberate "GQR record". The
export's argument block is built from the `0x802DCD6C` return, an
`undefined8 param_1`, `extraout_f1` and four `in_ps30/31` halves, so no honest
parameter split was available from the C alone.

The instruction stream in `work/dol/main.elf` resolves every one of those
aliases:

1. **The GQR block is dead.** The function opens with `stfd f31,0x50` /
   `psq_st f31,0x58` / `stfd f30,0x40` / `psq_st f30,0x48` and closes with the
   matching `psq_l`/`lfd` restores. Ghidra renders those half-saves as one
   `unaff_GQR0` scale/`CONCAT` block per pair, and the `ldexpf` results are
   discarded. `GQR0` is never non-zero in this image (the wave 106 header note),
   so no operand comes from it.
2. **`0x802DCD6C`/`0x802DCDB8` are the register-save helpers.** They are
   `__savegprlr_25`/`__restgprlr_25`; the "64-bit return" is the incoming
   `r3:r4` pair, which the body immediately copies into `r25`/`r26`. The helper
   calls are not reproduced.
3. **The real ABI.** `r3 = this`, `r4 = arg4`, `r5 = mode`, `r6 = variant`,
   `f1 = first`, `f2 = second`. `f30`/`f31` are single-precision scratch
   copies (`fmr f30,f1`, `fmr f31,f2`), which is why `fmuls` and `stfs` appear
   instead of double operations. The payload store is `stfs f30,0xC(r1)`, not
   a `stfd`.
4. **The two dropped call arguments.** The export prints
   `FUN_8004dd00()` with no argument; the stream shows `r3` still holding
   `this`, so the body forwards it. The export also prints the `+0x410`
   virtual call with two `double` casts; the stream computes them with
   `fmuls f1,f0,f30` / `fmuls f2,f0,f31` from `0.5f`, so the body passes two
   `f32` values through the new `w116_vcall60_f32_f32` helper.

## Confidence

`medium`, not `high`, because the message ids' meanings, the role of the two
float arguments and the payload record's layout are inferred from the
neighbouring action-entry bodies (`0x80138510`, `0x8014CDD0`). Every guard,
comparison, offset, store, call and constant is literal in the export plus the
`work/dol/main.elf` instruction stream.

Three observations are annotated `// observed:` in the body:

- The `+0x10D4` read inside the `first <= 0.0` block only assigns `false`,
  which the flag already holds, so it has no effect and is not reproduced.
- The payload words are written by `stw`/`stfs`/`stw`/`stw` at `r1+8`,
  `r1+0xC`, `r1+0x10`, `r1+0x14`; the array order in the body is memory order.
- `0x803089B0` is only known as the payload's first word; its role is
  `// unknown:` in the constant comment.

## Verification

- The address and 812-byte size match
  `work/ghidra/main-gb/export/functions.tsv`.
- The evidence file exists at
  `work/ghidra/main-gb/export/801469d0_FUN_801469d0.c`.
- The row has exactly one `pub unsafe fn fn_801469d0` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv`; `0x801469D0` was the last
  unassigned entry of the `0x80140000` bucket.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/worldmap.rs`.
- `node tools/check-lane-csv.mjs wave116` reports `wave116-worldmap.csv: 1 rows`
  and `OK`.

## Deliberate skips

None. The `0x80140000` bucket is closed by this pass.
