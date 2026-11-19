# Wave 221, lane A - `src/resource/archive.rs`, addresses `0x802C9840..0x802CD350`

Ten entries of `work/ghidra/main-gb/export/functions.tsv`, all in the
`0x802C0000` run of the `resource::archive` ledger module. All ten are
translated; none deferred.

| address | tsv size | real extent | body |
|---|---|---|---|
| `0x802C9840` | 420 | 420 (`blr` at `0x802C99E0`, no padding) | build the 80-byte descriptor and run both vtable slots |
| `0x802C9C00` | 60 | 60 (`blr` at `0x802C9C38`, 4 B padding) | snapshot `+0x20..+0x2F` into `+0x6C..+0x7B` |
| `0x802C9E30` | 260 | 260 (`blr` at `0x802C9F30`, 4 B padding) | fill a position triple and scale a four-byte tint |
| `0x802CA8A0` | 52 | 52 (tail branch to `0x802CA780`, no `blr`) | store two three-float vectors and forward |
| `0x802CB400` | 8 | 8 (`blr` at `0x802CB404`) | return the constant `2` |
| `0x802CB410` | 156 | 156 (`blr` at `0x802CB4A8`, 4 B padding) | channel/cull/tex-coord bootstrap |
| `0x802CC050` | 1060 | 1060 (`blr` at `0x802CC470`, no padding) | tiled-quad emitter for one window |
| `0x802CC480` | 240 | 240 (`blr` at `0x802CC56C`, no padding) | second GX state pass |
| `0x802CCE10` | 428 | 428 (`blr` at `0x802CCFB8`, no padding) | construct the `0x803503F8` table record |
| `0x802CD350` | 144 | 144 (`blr` at `0x802CD3DC`, no padding) | keyed probe of the `+0x08` record array |

No `functions.tsv` size in this lane is a prologue artefact: every size is
`end - start` where `end` is the closing tail branch or `blr + 4`.
`0x802CA8A0` is the only body without a `blr`; it ends in `b 0x802CA780`, and
the next entry (`0x802CA8E0`) starts exactly `52` bytes later, so the size is
right.

Bytes translated: 2828.

## What the run is

Two of the ten (`0x802CB410`, `0x802CC480`) are pure GX/library state ladders,
one (`0x802CB400`) is a constant leaf, and one (`0x802CCE10`) is a standalone
constructor. The remaining six read and write the same window record.

**The window record.** `+0x20` holds the live pose triple (three `f32`) and
`+0x2C` four tint bytes; `+0x6C`/`+0x78` are that pose's per-frame snapshot
(`0x802C9C00`); `+0x94` is a second three-float triple; `+0x30` is a sub-object
carrying a `u16` count at `+0x14` and a pointer array at `+0x18`; `+0x35` is a
grid/layer byte, `+0x36` bit 0 selects the colour source, `+0x37` is the draw
mode, `+0x50` is the tint scale, `+0x74`/`+0x75`/`+0x76` are `state`,
`layers`, `flag`, and `+0x78`/`+0x7A`/`+0x7C` are the filled count, the grid
cursor and the column cursor. `// inferred:` this is the screen window/effect
record the wave-105 and wave-219 lanes documented at `0x802BB000` and
`0x802C17E0`; the exports prove only the offsets.

**`0x802CC050`, the tiled-quad emitter.** The body divides the caller's count by
the `r13 - 0x4794` divisor, derives the two cursors, seeds a twelve-float block
at `r1+0x18` as `1.0` on the diagonal of a 3x4 matrix, calls
`0x801C9A80(block, 0x1E, 1)`, then scales the diagonal by the `r13 - 0x4798` /
`r13 - 0x4796` column counts and writes the two translation columns (`block[0]`
gets `+0x7A` widened to `f32`, `block[5]` gets
`(u16[r13-0x479E] - (+0x7C + u16[r13-0x4796]))` sign-extended). It then uploads
through `0x801C9A60(0)` and `0x801C9940(block, 0)`, builds a four-byte tint from
either the `r2 - 0x3E40` default `00 00 00 FF` or the three scaled bytes of
`*(*(*(this+0x30)+0x0C)+0x0C) + this[0x35]*12`, programs the texture state with
`0x802CB4B0`, emits one quad, dispatches on `+0x37` (0/2 -> `0x802C0B60` +
`0x802CB5F0`, three layers; 1 -> `0x802CB7B0`, two layers), drains the `+0x30`
table through `0x802CC730`, pads the trailing row with `0x801C8430`/`0x801C8500`
and a second quad, and ends with `+0x74 = 2`.

The quad emitted by both paths is the same shape: index byte, then two `f32`,
for the four vertices `(o, o)`, `(o, o - h)`, `(o + h, o - h)`, `(o + h, o)` with
`h` = the `r2 - 0x3BF0` slot (`0.99`) and `o` = the `r2 - 0x3C20` slot (`0.0`) —
or the `r2 - 0x3C14` slot, which holds the same `0.0`, on the mode-2 trailing
path.

**`0x802CCE10`, the table record.** Installs `0x803503F8`, defaults
`+0x04 = 0x78`, `+0x14 = 0x80`, `+0x16 = 1`, keeps the caller's word at
`+0x0C`, allocates `0x200` bytes (eight 64-byte slots, each cleared) and a
`0x80`-entry pointer array, and — when bit `0x40` of `r13 - 0x4730` is set —
overrides `+0x16 = 3` and `+0x04 = (0x78 | 0x80) & 0xBF = 0xB8`.

**`0x802CD350`, the keyed probe.** Walks the `u16`-bounded `+0x08` array,
probes each entry's `+0x54` field against `record + 0x14` with `0x802E1AB4`, and
on the first zero result calls `0x802CDB60(entry + 0x2C, record)` and returns
`1`; otherwise `0`.

## Values read from the shipped image

The `r2` slots were read from `work/dol/main.elf` rather than guessed:

| slot | address | value |
|---|---|---|
| `r2 - 0x3C1C` | `0x8042F464` | `1.0f` (matrix diagonal) |
| `r2 - 0x3C20` | `0x8042F460` | `0.0f` (off-diagonal, quad origin) |
| `r2 - 0x3C14` | `0x8042F46C` | `0.0f` (mode-2 quad origin) |
| `r2 - 0x3BF0` | `0x8042F490` | `0.99f` (quad half-extent) |
| `r2 - 0x3C48` | `0x8042F438` | `1.0f` (tint threshold) |
| `r2 - 0x3BF8` | `0x8042F488` | `0x4330_0000_0000_0000` (unsigned `2^52` magic) |
| `r2 - 0x3C38` | `0x8042F448` | `0x4330_0000_0000_0000` (second copy) |
| `r2 - 0x3E40` | `0x8042F240` | `00 00 00 FF` (default tint) |

The five `r13` slots (`0x4794`, `0x4796`, `0x4798`, `0x479E`, `0x4790`) and the
`r13 - 0x4730` flag halfword resolve to `0x8042AEEC..0x8042AF50`, which is
`.sbss` and is *unmapped* in `main.elf`. They are therefore read at runtime and
documented as `// unknown:`; nothing about their content is invented.

`SDA_BASE = 0x8042F680` was confirmed from the shipped code itself:
`0x802C9A10` returns `r13 - 0x5B20` = `0x80429B60`, and those four bytes are
`4C 4F 42 4A`, the NW4R object name `"LOBJ"`.

## Polarity and conversion details

- `fc050040` at `0x802C9E88` is `fcmpo cr0,f5,f0` with the operands in bits
  16-20 / 11-15 (the convention recorded in `COVERAGE_WAVE199_LANEE.md` and
  `COVERAGE_WAVE209_AUDIT.md`), and the following `bc 4,0` branches on the
  condition being **false** (`bge`). The plain-copy arm therefore runs whenever
  the table value is `>= 1.0`, which matches Ghidra's `if (1.0 <= value)`.
- `0x802CC050` and `0x802C9E30` widen integers through the `0x4330_0000` /
  `fsubs` magic pair. `fsubs` rounds the exact double difference to a single, so
  for these values the result is the plain integer as `f32`;
  [`to_f32_unsigned_a`](../src/resource/archive.rs) reproduces exactly that, and
  the one signed conversion reuses the module's existing `to_f32_signed_f`.

## Disassembler caveats handled

- `0x802CC050`: the export's "removing unreachable block" warning at
  `0x802CC450` and both `ldexpf` ladders come from an invented `unaff_GQR0`; the
  real instructions are `psq_st f31,0x88(r1),0,0` / `psq_l f31,0x88(r1),0,0`,
  that is, saving and restoring one single-precision register. Not reproduced.
- `0x802CD350`: `FUN_802dcd70()` / `FUN_802dcdbc()` in the export are
  `__savegprlr_26` / `__restgprlr_26` (`0x802DCD70` stores `r26..r31` at
  `r11-0x18..r11-4`, `0x802DCDBC` reloads them). The export invents a `u64`
  result and takes `param_2` from its high half; the body uses the real `r3` /
  `r4` arguments.
- `0x802C9840`: Ghidra folds the stack-slot clears into field stores at
  overlapping offsets; the raw words clear `+0x00..+0x4F` and then write only
  `+0x08`, `+0x04` and `+0x00..+0x03`. The body clears the 80 bytes once and
  writes those three fields, which is the same store set.

## Constants

New callee/data addresses in this lane are suffixed `_A`
(`FN_802C9A10_A`, `FN_802C9A20_A`, `FN_802CA780_A`, `FN_801C9A80_A`,
`FN_801C9940_A`, `FN_801C8430_A`, `FN_801C8500_A`, `FN_801C82F0_A`,
`FN_801C8330_A`, `FN_801C8370_A`, `FN_801C83D0_A`, `FN_802C3020_A`,
`FN_802CB4B0_A`, `FN_802CB5F0_A`, `FN_802CB7B0_A`, `FN_802CC730_A`,
`FN_802C0B60_A`, `FN_802B9250_A`, `FN_802E1AB4_A`, `FN_802CDB60_A`,
`DAT_803503F8_A`, `GX_FIFO_A`, the `SDA2_*_A` / `SDA_*_A` slots). Every callee
and data address that already had a const in this module — including the `_A`
names `FN_801C8600_A`, `FN_801C8640_A`, `FN_802C1F80_A`, `FN_802C2060_A`,
`FN_802C20E0_A`, `FN_802C2250_A` — is reused by name; nothing is redefined.

## Verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to a scratch directory, the
lane block was appended to `src/resource/archive.rs` there, and
`cargo check --lib` exited `0` with the same 26 pre-existing warnings as the
unmodified baseline (no diagnostic mentions `archive.rs`).
