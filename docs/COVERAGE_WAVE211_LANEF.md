# Coverage - Wave 211 lane F (`game::player`, main.dol)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 10 | 1040 | 8 | 2 |

Addresses `0x80066120`-`0x80066630` + its 272-byte tail, i.e. the effect
bookkeeping cluster between the dokan model actor (`0x80062xxx`) and the actor
spawn helper `0x80064610`. Three adjacent SDA globals are owned by this
cluster: `0x80429D28` (`u8`, last burst kind), `0x80429D38` (head of the
`0x28`-byte record list) and `0x80429D40` (the angle-source word this module
already names `W92_ANGLE_SOURCE_80429D40`).

## What landed

- **Record push.** `0x80066120` (160 B) allocates `0x28` bytes through
  `0x802B9210`, stamps `+0x00 = +0x04 = 0` and `+0x08 = node`, pushes the node
  through `0x80160FE0` onto the list at `0x80429D38`, and copies a `0x1C`-byte
  parameter block to `+0x0C..+0x24` (`+0x1C` is the id word, `+0x20` a byte,
  `+0x24` an extra word).
- **Record dedupe.** `0x800661C0` (84 B) walks that list, comparing the block's
  `u16` at `+0x02` and `+0x04`, the id word and the mode byte against the node's
  `+0x0E`, `+0x10`, `+0x1C` and `+0x20`, and tail-calls `0x80066120` when the
  walk falls off the end. Because `+0x08` is the node itself, those four node
  offsets are exactly `+0x02`/`+0x04`/`+0x10`/`+0x14` of the copied block.
- **Releasers.** `0x800663E0` (64 B) is the `(ptr, count)` free (`0x802B9280`
  when `count > 0`, returns `ptr`). `0x80066420` (112 B) clears `0x80429D40`,
  then drops the `0x20`-element container at `+0x48` with `0x802DCA98(container,
  0x800663E0, 0x10, 0x20)`, then frees `this` when `count > 0`, returning
  `this`.
- **Object reset.** `0x80066490` (96 B) zeroes `+0x14/+0x18/+0x1C/+0x20/+0x28/
  +0x30/+0x38/+0x3C` and seeds `+0x44` with `-8000.0f` (`0x8042BB94`), then
  calls `0x80066820(this)` and `0x800DBE30(this)`.
- **Stage id.** `0x800664F0` (80 B) is `0x8008E6B0(record, +0x120F)` truncated
  to 16 bits, where `record = [0x80429E98] + 0x3B0 * [0x8042A1C8][0x120E]` and
  a zero word at `+0x2C` forces `0`. The lookup is the module's existing
  `w18_stage_record()` helper, reused verbatim.
- **Stage predicates.** `0x80066540` (44 B) and `0x80066570` (44 B) are the
  `cntlzw(value - 2) >> 5` and `cntlzw(value - 12) >> 5` tests, i.e. "stage id
  is 2" and "stage id is 12".
- **Angle ramp.** `0x800665D0` (84 B) reads the 5-bit index from the source's
  `+0x0C` byte, stores `(index + 1) & 0x1F` back, and returns
  `base - 8.0f * index` with `base = 400.0f` (`0x8042BB98`) for `mode != 1` and
  `-3100.0f` (`0x8042BB9C`) for `mode == 1`.
- **Burst spawner.** `0x80066630` (272 B) copies the position triple, replaces
  x with `0x80101AA0(x)`, seeds z with `500.0f` (`0x8042BBB0`, kind `0`) or
  `-3000.0f` (`0x8042BBB4`), latches the kind at `0x80429D28`, then spawns
  `count` type-`0x193` actors through `0x80064610` with id
  `i | (selector & 0xFF) << 4`, walking x down by `8.0f` (0x8042BBA0) on every
  actor it actually got back.

## Corrections and recoveries

- **`0x80066630`'s size differs between the two Ghidra exports.** The canonical
  `work/ghidra/main-gb/export/functions.tsv` says 272 bytes, the older
  `work/ghidra/main/export/functions.tsv` says 28. Disassembly settles it:
  `stwu r1,-0x40(r1)` at `0x80066630`, `blr` at `0x8006673C`, next entry at
  `0x80066740` - 272 bytes. The `# Original` line uses 272, which is also what
  `tools/check-lane-csv.mjs` reads (it scans `main-gb` last, so `main-gb` wins).
- **`0x800661C0`'s 84 bytes end on a tail branch, not a `blr`.** The body ends
  with `b 0x80066120` at `0x80066210`; the `blr` at `0x80066214` is unreachable
  padding followed by two zero words (next entry `0x80066220`). The size in the
  table is still the exact instruction extent.
- **`0x800661C0` really has five parameters.** The export declares four because
  it forwards `r7` untouched; the only call site in the image (`0x80068304`)
  sets `r4`, `r5`, `r6` and `r7` before the tail branch, and `r7` becomes
  `0x80066120`'s `+0x24` payload. Both bodies therefore take `(owner, block,
  id, mode, extra)`; `owner` is never read by either (the callee overwrites
  `r3` with the allocation size).
- **`0x80066120`'s `extraout_r4` is `r27`.** The export prints the four copied
  words as `*extraout_r4`, `extraout_r4[1..3]`; disassembly shows
  `mr r27,r4` at `0x80066134` and all six loads are `r27`-relative. The `# Original`
  documents this so the block argument is not mistaken for an unresolved
  register.
- **`rlwinm` arithmetic was decoded, not copied.** For `0x80066630` the export's
  `(param_3 & 0xff) << 4` is correct: the word at `0x800666AC` is
  `rlwinm r0,r31,4,20,27`, and with MSB-first `MB`/`ME` numbering that is
  `((x & 0xFF) << 4)`. The loop tail's `rlwinm r31,r0,27,24,31` is
  `leading_zeros() >> 5 & 0xFF`, so the selector's high nibble only reaches the
  first spawn.
- **`0x800665D0` returns a single in `f1`.** The export types it `double`
  because it models the `0x43300000` u32-to-double bias pair as a `double`
  temporary; the final `fsubs f1,f3,f0` leaves an `f32`. The body returns `f32`
  (the existing `fn_800437d0` call site transmutes the same address as
  `-> f64`, which is the same register on the real target).
- **`0x800664F0`'s lookup is already in the module.** `w18_stage_record()`
  performs the identical table/stride/zero-word sequence, so it is reused
  instead of being re-expanded; the variant byte `+0x120F` is read by this body.

## Deferrals

None. All ten bodies were reconstructible from the shipped code. Two callees
remain untranslated but are called by address, as the instructions allow:
`0x80066820` and `0x800DBE30` (`fn_80066490`), plus `0x80101AA0` (`fn_80066630`),
whose arity is taken from `crate::game::bases`'s existing call site.

## Verification

- Temp-copy (`%TEMP%\opencode\w211f`) `cargo check --lib` exits 0 with the 23
  pre-existing warnings and none in `src/game/player.rs`; `tools/doc-lint.mjs`
  reports 0 violations on the same tree.
- The lane CSV has the canonical seven-column header, ten rows, plain lowercase
  hex addresses, and `# Original` sizes equal to the canonical `functions.tsv`
  (160/84/64/112/96/80/44/44/84/272).
- No files outside `work/wave211/laneF_block.rs`,
  `ledger/agents/wave211-laneF.csv` and this document were written;
  `src/game/player.rs` is untouched.
