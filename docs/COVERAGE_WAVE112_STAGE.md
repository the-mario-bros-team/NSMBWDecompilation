# Coverage - Wave 112 (`game::stage`, closing the `0x800C0000` bucket)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The `evidence`
column of `ledger/agents/wave112-stage.csv` is the Ghidra C export each body was
written from.

The bucket opened with 290 entries and 284 already had `game::stage` rows, so
wave 112 starts from six open rows. Three are landed here and three are
deliberately skipped, which closes the bucket.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 3 | 584 | 0 | 2 | 1 |

All three rows are `translated`; none is `partial` or `stub`. The pass covers
`0x800C9A60..0x800CB490`.

## What this pass covers

- **The two variadic message front-ends** (`0x800C9A60`, 160 bytes, and
  `0x800C9DF0`, 144 bytes). Both are `__va_start` front-ends of the message
  system already described by [`fn_800c9e80`]: they home the eight argument
  words `r3..r10` into the GPR save area and, under the `$cr1` bit-1 guard, the
  eight argument doubles `f1..f8` into the FPR save area that follows at
  `reg_save_area + 0x20`. The `va_list` word is `0x0500_0000` in the animation
  player (`gpr = 5`, five named words) and `0x0300_0000` in the text-line entry
  (`gpr = 3`, three named words); `input_arg_area` is the caller's stacked
  arguments and `reg_save_area` is the homed register block. Both bodies then
  forward the argument words still in registers.
  - [`fn_800c9a60`] additionally stores its third and fourth named words at the
    global message state `r13-0x5704` `+0x3D0`/`+0x3D4` and calls the worker
    `0x800C9B50`. Every call site in the image passes five words
    (`0x80014FF0` uses `(pane, handle, 0x67, 0x32, 0)`), which matches the
    `gpr = 5` tag and is why the two stored words are named `start`/`end`.
  - [`fn_800c9df0`] calls [`fn_800c9e80`] directly. That callee is already
    translated here as `(this, format, a3, a4, a5)`, so the three named words
    plus the first two register varargs fill its five parameters — the mapping
    the `gpr = 3` tag implies.
- **The overlay-object arming helper** (`0x800CB490`, 280 bytes). It sits
  directly above the wave-106 render-object family: `variant == 0` allocates two
  0x18-byte objects through `0x802B9210` and installs the `0x803177BC` /
  `0x80317798` vtables at record `+4` / `+8`, initialising each with the
  `0x800CB740` / `0x800CB760` alpha flavours; `variant == 1` allocates one
  object with `0x80317774` and the `0x800CB780` flavour, which also sets the
  `+0x14` flip byte. The texture handle handed to each flavour is the `r3`
  result of the `0x800CAB10` `cave_bti` load (index 5 at the single call site,
  `0x800CCE00`). The only call site passes the record built by
  `0x800CB470`, so the `+4`/`+8` slots line up with that 0xc-byte object.

The two front-ends were cross-checked against the already-translated
[`fn_800c9e80`] and against the `va_list` front-ends that `engine::main_loop`
landed for the MSL `sprintf`/`snprintf` entries; the same save-area order (eight
GPRs, then eight FPRs) and the same `gpr`/`fpr` tag word are used there.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800C9A60` | `fn_800c9a60` | 160 | low |
| `0x800C9DF0` | `fn_800c9df0` | 144 | medium |
| `0x800CB490` | `fn_800cb490` | 280 | medium |

Address range: `0x800C9A60..0x800CB490` (3 bodies, 584 bytes).

## Deliberately skipped

Three of the six open rows are not landed. None of them is recoverable from
this export alone.

- **Branch thunks** (`0x800C3BA0`, `0x800C8A90`, 4 bytes each). Their exports are
  the *target* bodies: `thunk_FUN_800c3bac` (the "add `0xC000` until the value
  fits a signed 16-bit angle" loop) and `thunk_FUN_80007320` (the `0x80007300`
  virtual-list walk). Both targets already own ledger rows (`game::stage`
  `0x800C3BAC`, `render::geometry` `0x80007320`), so writing a second body would
  double-count the same code. Same precedent as waves 84 and 98-111.
- **The screen-offset body** (`0x800C9B50`, 660 bytes). This is the worker the
  animation front-end calls: it seeds the message state, formats the line,
  builds a projection matrix, and derives the `+0xE8`/`+0xF4` screen offset from
  the on-screen delta of two projected points. Two of its inputs arrive as
  paired singles and are quantised through `GQR0` (`& 7` type,
  `(>> 8) & 0x3f` scale, `ldexpf` pre-scale, the type-4/6 and type-5/7 arms);
  the export then stores them to `local_8`/`local_18` and never reads those
  slots, so the two vectors the body actually consumes are unrecoverable.
  `f30`, `f31`, `ps30_1` and `ps31_1` are all unsourced registers as well. This
  is the same `GQR` blocker that wave 111 recorded for `0x800B96A0`, and it is
  why `fn_800c9a60` forwards to the address instead of to a body.

Three landed + three skipped = the six open entries of the `0x800C0000`
bucket, which is now closed.

## Verification

- Every address and byte size matches `functions.tsv` (3 rows, 584 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`; the three were
  chosen from the six open entries of the `0x800C0000` bucket.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs`.
- `node tools/check-lane-csv.mjs wave112` prints `OK` and reports 3 rows for
  this CSV.
- The module header of `src/game/stage.rs` gained a wave-112 paragraph naming
  all three bodies and the three skips; no earlier count became stale.
