# Coverage - Wave 175 lane E (`game::bases`, 408/412-byte classes)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane E block) | `game::bases` | 6 | 2,456 | 4 | 2 |

Six bodies: one 408-byte pair, one 412-byte quad. Source of the bodies:
`work/wave175/laneE_block.rs`; per-function ledger:
`ledger/agents/wave175-lanee.csv`.

## What this pass covers

- **Sound-table row writer (1 body).** `0x80773EF0` is the callee the existing
  [`fn_80773d70`] calls three times per slot as
  `0x80773EF0(1.0, this, row, slot, side)`. It resolves the `0x16518`-stride row
  block at `this+0x3394`, reduces `side` to the `0x800F5770` sub-table selector
  with the double `cntlzw`/`rlwinm` idiom, walks `0..count` records with
  `0x800F5730`, asks [`fn_80773ad0`] for the state mask and [`fn_80774090`] for
  the "already bound" answer, binds with `0x800F0580(speed, slot + 0x1A0,
  record)` and then runs the `0x800F02C0`/`0x800F0440`/`0x800F0380` writers for
  the `4`/`2`/`8` mask bits while `row == this[+0x338C]`.
- **Course-node ability refresh (1 body).** `0x8079AC60` is the shortened
  sibling of the translated [`fn_8079a930`]: same `+0x29C`/`+0x22C` node fields,
  same `0x800F57A0` gate, same `0x800F56B0` + `0x800F9970` cursor walk through
  `0x80799B00`/`0x80799A50` and the same `0x80C561E0` ability set/clear.
- **Pad mask republish (1 body).** `0x8067E160` ANDs `+0x3D0`/`+0x3D4` with the
  two words of the `0x8042A078` table, publishes them to `+0x3D8`/`+0x3DC` and,
  when they changed (or the `1 << (+0x3EC & 63)` mask misses), selects one of
  the four `+0x3ED..+0x3F0` channel ids and forwards it through `0x800E41C0`.
- **Interface mask builder (1 body).** `0x8071F390` decrements the four cooldown
  bytes at `+0x558..+0x55C`, walks the three `0x60`-byte child blocks from
  `+0x434`, reads the virtual `+0x6C` id (`+0x60` vtable) of every node, and
  returns the `1 << id` mask plus `0x10` for out-of-range kind-`2` actors while
  arming the cooldown to `4` through `0x80056CF0`.
- **World-map pad update (1 body).** `0x80795AA0` is the sibling of the
  translated [`fn_8077dc80`]: handler-table dispatch for `+0x228`, the interface
  id call at vtable slot `+0x60`, the `0x800D6AB0`/`0x80100CA0` sound submit
  short-circuit, `0x80105060`, the `0x16`/`0x8B` looping-sound arm and three
  copies of the `0x80100A00` scale into `+0x2E4..+0x2EC`.
- **Smooth bump function (1 body).** `0x807BE230` is pure `f64` arithmetic:
  folds out the sign of `param_1`, solves `a = 2pq - 2p - 2q`,
  `b = -a - 1.0`, `disc = a^2 - 4bp`, `t = (-a - sqrt(disc)) / 2b`, and returns
  `(f32)(sign * t^2 / (t^2 + r^2 + 2qrt))`, with a two-sample average of itself
  at `p -/+ 1e-6f` when `|2b| <= 1e-7`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80773EF0` | `fn_80773ef0` | 408 | high |
| `0x8079AC60` | `fn_8079ac60` | 408 | high |
| `0x8067E160` | `fn_8067e160` | 412 | medium |
| `0x8071F390` | `fn_8071f390` | 412 | high |
| `0x80795AA0` | `fn_80795aa0` | 412 | medium |
| `0x807BE230` | `fn_807be230` | 412 | high |

## Deferred

None.

## Corrections and notes

- **`and`/`or`/`xor` operand order.** The disassembler output used for this lane
  prints the logical three-operand forms as `op rS,rA,rB` while the ISA encodes
  `op rA,rS,rB` (destination in bits 11-15). That reversal is what makes the
  Ghidra exports look as if they read uninitialised `r7`/`r26`/`r27`. Read with
  the ISA order, `0x8079AC60`'s `lwz r6,0x70(r6)` is live and feeds
  `and r7,r6,r3`, and `0x8067E160`'s `+0x3D0`/`+0x3D4` loads feed `r27`/`r26`,
  which are the words stored to `+0x3D8`/`+0x3DC`. The translated bodies follow
  the ISA order; the resulting value flow matches both Ghidra exports.
- All six exports carry `unaff_GQR0`/`ldexpf` guards. Every GQR-0 access in
  these bodies is a plain scalar `psq_l`/`psq_st` of `f27..f31`, so the guards
  are compiler noise and are not emitted.
- `0x80795AA0` and `0x8071F390` take `r7`/`r31`-style virtual calls through the
  `+0x60` vtable; `0x80795AA0` uses `ptr_at(ptr_at(this + 0x60) + 0x60)` and
  `0x8071F390` uses the existing `actor_method0(actor, 0x6C)` helper, both exactly
  as the existing `0x8077DC80` and `0x806A*` bodies do.
- `0x80795AA0`'s `0x802DCBFC` dispatch is the same CodeWarrior member-function
  thunk already handled in `0x8077DC80`: entries `0x807D00D0..0x807D015C` are
  twelve-byte `{0, -1, handler}` triples, so with `index < 0` the thunk reduces
  to a plain call of the `+0x8` handler with `r3 = this` (`r4` still holds the
  table base, which the translation forwards). `0x8079B110` has no Rust body
  (Ghidra reports bad instruction data - it is a paired-single body) and is only
  probed for non-zero through `call0_ret`.
- `0x8067E160`'s mask is produced by `0x802DD1EC(0, 1, byte[+0x3EC])`, whose
  main.dol export is a 64-bit shift; it is written inline as
  `1u64 << (byte & 0x3F)` and split into the high/low words the two `and` tests
  consume. Its channel accumulator starts from an uninitialised `r7` (the raw
  words contain no `li` for it); `0xFF`, the sentinel every branch in the body
  tests against, is used for the unreachable all-`0xFF` case.
- `0x807BE230`'s `F63_8` prints are ambiguous between `fmr` (XO 72) and `fneg`
  (XO 40) in this decoder. Each was re-decoded from the raw word: the two
  `0x...E050` forms are `fneg` (the `-a` numerator and the `-a - 1.0` slope),
  the `0x...0890` forms are `fmr`. Constants are read at runtime with the
  existing `read_f64`/`platform::read_f32` helpers rather than written as
  decimal literals.
- `0x807BE230`'s `bl 0x802E8868` is the main.dol double `sqrt`
  (a four-byte thunk to `FUN_802e8620`, whose body is the classic bit-serial
  square root) and is called through the existing `call_f64_f64` helper.

## Verification

- The lane-E block type-checks standalone against stubbed helper signatures
  (temporary `rustc` harness with the real helper ABIs and the real
  `abi_callN!` macro bodies); the harness lives outside the repo.
- All six evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`; every constant, branch and field offset
  was re-read from `work/rels/elf/d_basesNP.elf`.
- Only helpers that already exist in `src/game/bases.rs` are called
  ([`fn_80773ad0`], [`fn_80774090`], [`fn_80799a50`], `actor_method0`, `ptr_at`,
  `read_f64`, `call0_ret`, `call_f32_u32x2`, `call_f32_u32x4`,
  `call_f32x2_u32x2`, `call_f32_1`, `call_f64_f64`, `platform::call7`);
  `0x800F05*`, `0x800F0*`, `0x800F57A0`, `0x800F56B0`, `0x800F9970`,
  `0x800F0580`, `0x800F02C0`, `0x800F0440`, `0x800F0380`, `0x800FDC*`,
  `0x80799B00`, `0x80C56B20`, `0x80C561E0`, `0x8010*`, `0x8013AAA0`,
  `0x8012DBE0`, `0x80056CF0`, `0x800B2EE0`, `0x800E41C0`, `0x80162510`,
  `0x800D6AB0` and `0x8079B110` have no Rust body and go through the
  `abi_callN!` macros, `platform::call7` or the typed `call_*` helpers.
- `src/game/bases.rs` was not modified by this lane.
