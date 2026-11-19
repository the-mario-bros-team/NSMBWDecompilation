# Wave 212, lane F - `src/game/player.rs`

Ten bodies at `0x80068D70..0x80069270`, all in the `game::player` ledger module.
Bodies live in `work/wave212/laneF_block.rs`; rows in
`ledger/agents/wave212-laneF.csv`.

## Coverage

| address | size | Rust function | status | confidence |
|---|---|---|---|---|
| `0x80068D70` | 100 | `fn_80068d70` ramp leg `+0x0C` | translated | high |
| `0x80068DE0` | 576 | `fn_80068de0` record-driven spawn | translated | medium |
| `0x80069020` | 60 | `fn_80069020` constructor (`+0x10` vtable) | translated | high |
| `0x80069060` | 88 | `fn_80069060` release + free | translated | high |
| `0x800690C0` | 24 | `fn_800690c0` forwarder to `0x8016A7F0` | translated | high |
| `0x800690E0` | 4 | `fn_800690e0` tail branch to `0x8016A8E0` | translated | high |
| `0x800690F0` | 20 | `fn_800690f0` constructor (`+0x00` vtable) | translated | high |
| `0x80069110` | 64 | `fn_80069110` free-and-return | translated | high |
| `0x80069240` | 40 | `fn_80069240` ten-entry list append | translated | high |
| `0x80069270` | 360 | `fn_80069270` nearest candidate actor | translated | medium |

Total: **1,336 bytes**, 10 of 10 translated, 0 deferred.

Every size above is the canonical `main-gb` `functions.tsv` value and was
re-checked against the prologue and the `blr`/`bclr` in `work/dol/main.elf`:

* `0x80068D70`: `bclr 20,0` at `0x80068DD0` - exact.
* `0x80068DE0`: `bclr 20,0` at `0x8006901C` - exact.
* `0x80069020`: `bclr 20,0` at `0x80069058`; `0x8006905C` is padding - exact.
* `0x80069060`: `bclr 20,0` at `0x800690B0` - exact.
* `0x800690C0`: `b 0x8016A7F0` at `0x800690D4` ends a 24-byte tail-call body.
* `0x800690E0`: the single instruction `b 0x8016A8E0` at `0x800690E0`, then
  padding to `0x800690F0` - the 4-byte row is the whole body.
* `0x800690F0`: `bclr 20,0` at `0x80069100` - exact.
* `0x80069110`: `bclr 20,0` at `0x8006914C` - exact.
* `0x80069240`: early `bclr 4,0` at `0x80069248`, normal `bclr 20,0` at
  `0x80069264` - exact.
* `0x80069270`: `bclr 20,0` at `0x800693D4` - exact.

No row needed the "prologue artefact" note: no `functions.tsv` size in this lane
is a truncated 16/20-byte split.

## Notes

### `0x80068DE0`

The largest body. Ghidra's export drops `r3` (unused), prints the six-arg
signature correctly, and mistypes the return: the early exit is `li r3,0`, so the
body returns the spawned actor or `0`. The `0x28`-stride table at `0x8030A040`
is read twice (once for the `0x2EB` "no actor" test, once for the spawn's first
argument); only its first `u16` is read, so it is described as a descriptor
table rather than an actor-type array.

The two `0x802DD1EC(0, 1, n)` calls build a `u64` `1 << n` that is stored as two
words at `0x80429D20`/`0x80429D24` and later copied into the new actor's
`+0x358`/`+0x35C`. The high byte of the record's `u16` is processed first and the
low byte second, so the low byte wins when both are set - the body keeps that
order and the resulting "last write wins" mask.

All five integer-to-float conversions in this body (the two position words and
the two `+0x14`/`+0x18` seeds) use `lis 0x4330` + `xoris 0x8000` against the
double at `0x8042BBC8` = `0x43300000_80000000`, i.e. they subtract `2^52 + 2^31`
and therefore read the word as **signed**. The `2^52` alone (at `0x8042BBA8`,
low word `0`) would have meant unsigned; it is not the constant used here. The
same applies to the counter in `0x80068D70`.

The `bl 0x802DCD70` at `0x80068DF0` and `bl 0x802DCDBC` at `0x8006900C` are the
`__savegprlr_26` / `__restgprlr_26` prologue and epilogue helpers; they carry no
arguments and are omitted, as the module already does elsewhere.

### `0x80068D70`

The fourth leg of a four-step ramp. The three siblings (`0x80068BC0`,
`0x80068C50`, `0x80068CE0`) read the same `+0x00..+0x0C` counters:
`1900.0f - 16.0f*c`, `-2500.0f + 16.0f*c` and `400.0f - 8.0f*c`. This leg is
`-3100.0f - 8.0f*c`. All four advance their counter through a shared "add one,
wrap at 30" helper; `0x80068D50` is untranslated and is dispatched by address.

### `0x800690C0` / `0x800690E0`

`0x800690C0` is not a plain thunk: it computes `r8 = (r4 != 0xFFFFFFFF)` and
tail-branches to `0x8016A7F0`, which genuinely takes six words (`0x8016A7F0`
moves `r8` into `r31` and forwards it to `0x8016E630`). The export prints
`FUN_8016a7f0()` with no arguments and loses the computed flag, so the body was
taken from the raw stream.

`0x800690E0` is literally `b 0x8016A8E0`; `functions.tsv` names it
`thunk_FUN_8016a8e0` and Ghidra printed the inlined body of the target, so the
ledger row cites the export file under that name
(`800690e0_thunk_FUN_8016a8e0.c`) instead of the `<addr>_FUN_<addr>.c` pattern
used by the other nine rows - no file of that pattern exists for this address.
The body reuses the module's existing `W92_MODEL_FLUSH_8016A8E0` constant
instead of defining a second one.

### `0x80069270`

The one open question in this lane: the body reads an **incoming `f30`** that it
never writes. `stfd f30,0x50(r1)` / `psq_st f30,0x58(r1)` save it and the pair at
`0x800693B0`/`0x800693B4` restores it, but no instruction in
`0x80069270..0x800693D4` loads or computes it, and neither call site
(`0x8004AD90+0x3C`, `0x800583A0+0x14C`) nor the code around either call writes
`f30` either. Both of its uses are comparisons against `0.0f` (a `bias < 0`
shortcut and a `bias > distanceÂ²` test), so it is a caller-supplied threshold.

Rather than guess it away, the body takes it as an explicit `near: f32`
parameter and says so in the doc block; the loop, the two filters, the
`0x802EF398[actor+0x380]` x offset, the two `+0x70` slot calls, the `-1.0f`
sentinel and the two-level "first candidate vs. strictly nearest" selection are
all literal. Flagged `medium` for that reason.

## Verification

Scratch copy at `%TEMP%\opencode\w212F4` (`src/`, `Cargo.toml`, `Cargo.lock`,
block appended to the copy's `src/game/player.rs`):

```
cargo check --lib
warning: `nsmbw` (lib) generated 23 warnings (run `cargo fix --lib -p nsmbw` ...)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 17.52s
exit code 0
```

The 23 warnings are the unmodified baseline set; no diagnostic anywhere points at
`src/game/player.rs`, so the appended block is warning-free.

`src/game/player.rs` in the repo was not modified, as the lane instructions
require.

