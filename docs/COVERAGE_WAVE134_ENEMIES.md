# Coverage - Wave 134 (`game::enemies`)

## How to read this file

This page records the Wave 134 `game::enemies` lane: 24 functions of REL module 3,
`d_enemiesNP`, appended to `src/game/enemies.rs` as section 116. Every row of the
machine-readable mirror (`ledger/agents/wave134-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none of
these is a runtime address; every function doc repeats that note. Floating-point
constants are cited by address and read through `platform::read_u32` /
`f32::from_bits`, never transcribed as invented values. Raw words were decoded
from `work/rels/elf/d_enemiesNP.elf` at the mapping in
`docs/REL_ELF_MAPPING.md` (`file_offset = address - 0x80A00000 + 0x1000`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 24 | 5,740 | 21 | 3 |

The lane has two halves:

- **12 bodies** are the twelve rows of `docs/SPEC_WAVE133_ENEMIES.md` (ranks
  62-74, all marked **ready**). They were written from that page's
  instruction-level decode, then every claim was re-checked against the Ghidra
  export and the raw ELF words.
- **12 bodies** are still-open entries of the `0x80AC0000` planner bucket. They
  are one coherent cluster - the `nokonoko` learner family at
  `0x80ACB0C0..0x80ACFE60` - which is why they were picked: the three chain
  helpers, the four state steps, the `0xA18`-byte constructor, its thunk and the
  three model/spawn helpers share the class word at `+0x60`, the `+0x5C4`/`+0x5CC`
  occupancy chains and the `0x80B307xx` constant pool. After this pass the bucket
  has 20 untranslated rows left (32 before the pass).

## What this pass covers

- **The specification bodies (`0x80A2EA60..0x80B28710`).** The Choropoo
  `g3d/choropoo.brres` + `walk` binder, the `Wm_en_comattack` puff with cue
  `0x21C`, the type-`0x19B` oriented-box scan and its recursion, the eight-slot
  `0.06` countdown tick, the child-chain "first busy" query plus its
  byte-identical twin, the vertical interval emitter over
  `*(this+0x818) + 1` samples, the four-player `x`-band test, the coin effect and
  four-stage teardown, the three-key value track on `+0x728`, and the two
  deleting destructors.
- **The `nokonoko` learner cluster (12 bodies).** The `+0x5C4` chain walker
  (`0x80ACB0C0`), the `+0x5CC` re-splice gate (`0x80ACB200`), the two-slot claim
  (`0x80ACB280`), the walk/turn side-flip trio (`0x80ACEA40`, `0x80ACEB70`,
  `0x80ACED20`), the turntable countdown with its midpoint flip (`0x80ACE000`),
  the `0xA18`-byte allocating constructor (`0x80ACF800`), its deleting thunk
  (`0x80ACF910`), the spawn initialiser that decodes the spawn word
  (`0x80ACF950`), the `nokonokoA`/`walkA` binder (`0x80ACFC20`) and the
  controller-mode gate (`0x80ACFE60`).

### Corrections to the wave-133 specification

Two register-level claims of `docs/SPEC_WAVE133_ENEMIES.md` are wrong against the
raw words. Both were re-decoded and the bodies follow the stream, not the prose:

- **`0x80B18F20`'s second argument.** The specification says the recursive call
  passes `r4 = 0x80B30000` because "nothing reloads it". `0x80B18F58` is
  `or r4,r3,r3`, which reloads `r4` from the probe pointer returned by
  `0x80162D50`; the `lis r4,0x80B3` at `0x80B18F4C` is dead. The Ghidra export
  agrees (`FUN_80b18f20(param_1,iVar2)`), so the body passes the candidate as the
  cursor.
- **`0x80AEAC20`'s argument count.** The specification lists `f1`, `f2` and `f3`
  arguments. The stream is `fmr f1,f31` plus `r3..r6` only: `f2`/`f3` are the
  `pos.y`/`pos.z` stores that follow. The body passes one float and four
  integers, which is also the export's shape.

### Confirmations worth recording

- **`0x80AFEDD0` is word-for-word `0x80A2F830`.** A mechanical diff of all 56
  words reports zero differences, including the tail `bl`: the displacement is
  identical because the callee sits the same distance behind the call site.
- **`0x80ACE000`'s two `main.dol` tables.** `0x8042C1A0 + side` is read as a
  signed byte and shifted left 14, and `0x8042BB20 + 4 * side` is read with
  `lfsx`; the `lwzx`-looking word at `0x80ACE0E0` is `lfsx` (XO 535), which is
  what makes `+0xB4` a float store. The countdown-zero arm separately reads a
  `u16` yaw from `0x8042B540 + 2 * side`.
- **`0x80ACEA40`'s mode byte.** The side-1 arm stores `2` and the side-0 arm
  stores `0` into `+0x5BC`, read as a word from `0x80B30784` and `0x80B30780`
  respectively.
- **`0x80ACF950`'s counter idiom.** `0x80ACFBA8` is `oris r0,r0,0x8000` - an OR,
  not the usual XOR - followed by the `2^52 + 2^31` subtraction at
  `0x80B30840`; both give the same value for the 4-bit counter.
- **The `0x802DCDxx` calls.** `0x802DCD68` (entry of `0x80ACF950`) and
  `0x802DCDB4` (its exit, with `r3 = 1`) are the CodeWarrior
  `_savegpr`/`_restgpr` pair described in `docs/REL_ELF_MAPPING.md`; the incoming
  `r3` survives, so the constructor is written with a normal `this` argument.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A360D0` | `fn_80a360d0` | 228 | high |
| `0x80A37460` | `fn_80a37460` | 228 | high |
| `0x80B18F20` | `fn_80b18f20` | 228 | high |
| `0x80A2EA60` | `fn_80a2ea60` | 224 | high |
| `0x80A2F830` | `fn_80a2f830` | 224 | high |
| `0x80AFEDD0` | `fn_80afedd0` | 224 | high |
| `0x80AEAFF0` | `fn_80aeaff0` | 216 | high |
| `0x80A7E640` | `fn_80a7e640` | 212 | high |
| `0x80A442D0` | `fn_80a442d0` | 208 | high |
| `0x80AD3C10` | `fn_80ad3c10` | 208 | high |
| `0x80AEDAC0` | `fn_80aedac0` | 208 | high |
| `0x80B28710` | `fn_80b28710` | 208 | high |
| `0x80ACB0C0` | `fn_80acb0c0` | 100 | high |
| `0x80ACB200` | `fn_80acb200` | 120 | high |
| `0x80ACB280` | `fn_80acb280` | 172 | high |
| `0x80ACEA40` | `fn_80acea40` | 292 | medium |
| `0x80ACEB70` | `fn_80aceb70` | 296 | high |
| `0x80ACED20` | `fn_80aced20` | 264 | high |
| `0x80ACE000` | `fn_80ace000` | 272 | medium |
| `0x80ACF800` | `fn_80acf800` | 272 | high |
| `0x80ACF910` | `fn_80acf910` | 64 | high |
| `0x80ACF950` | `fn_80acf950` | 720 | medium |
| `0x80ACFC20` | `fn_80acfc20` | 332 | high |
| `0x80ACFE60` | `fn_80acfe60` | 220 | high |

### Why three rows are `medium`

- **`0x80ACEA40`.** Every branch, store and call is literal, but the two name
  records it tests (`0x80B76250`, `0x80B76258`) live in BSS and are built at run
  time, and the `0x80035BB0` helper's only established property is its `-1`
  sentinel.
- **`0x80ACE000`.** The countdown, the two flips and both table indices are
  literal, but the `main.dol` tables at `0x8042C1A0` (sign byte),
  `0x8042B540` (yaw word) and `0x8042BB20` (`z` float) are read at run time
  instead of being transcribed; a byte-exact comparison still needs those
  tables.
- **`0x80ACF950`.** Same reason at two more addresses (`0x8042BB18` byte,
  `0x8042BB20` float, both only read and stored) plus the shared 4-bit counter at
  `0x80429D40 + 0x28`, which the body both reads and increments.

## Deliberate skips

- **`0x80AC0840`.** Consumes `GQR3` (type codes 4/5/6/7 in the quantiser field),
  so it stays class C under the wave-127 rules until the `main.dol` GQR words are
  pinned.
- **`0x80ACC8F0`.** Same `GQR3`-quantised `in_f31`/`in_ps31_1` shape; not
  reconstructible without the GQR3 word.
- **`0x80ACDE00`.** Its tail call to `0x80ACDB00` takes an animation frame in
  `f1` that the preceding `0x80168080` call leaves undefined in the stream; the
  value cannot be established from the export or the words, so no body was
  written.
- **`0x80ACE690`, `0x80ACE900`, `0x80ACEE40`, `0x80ACDEF0`.** Each mixes a
  `main.dol` byte table with arithmetic (sign-extended `0x8042C1A0` times
  `0x8000 / count`) or loses the animation-name argument to a zero-argument
  export; they are readable but not byte-exact yet. Their callers in this lane
  cite them by address only.
- **`0x80ACB000`, `0x80ACB130`.** Both are chain/registry helpers whose Ghidra
  exports drop the `r3` receiver and whose live registers (`r31`, `r29`) come out
  of `_savegpr`; they are called by address from three bodies here but were not
  claimed as rows.
- **`0x80AC0750`, `0x80AC1800`, `0x80AC8850`, `0x80AC9870`, `0x80ACCC00`,
  `0x80ACCE20`.** The remaining large rows of the bucket were left for a later
  pass once the body target was met. Nothing was written for them, so no ledger
  row claims them.
- **No address already in `ledger/functions.csv` was touched**, and no other
  lane's file, CSV or doc was modified.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 24 sizes sum to 5,740
  bytes (2,616 for the twelve specification rows and 3,124 for the twelve bucket
  rows).
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 24 was in
  `ledger/functions.csv` before this pass.
- The module header was updated: cluster item 116, the cluster count (115 to
  116), the function totals (3,096 to 3,120 public translations, 3,025 to 3,049
  `fn_<addr>` bodies), the "start here" list and the records list.
- `cargo check --message-format=short` emits no error and no warning naming
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave134` reports `wave134-enemies.csv: 24 rows`,
  24 unique addresses and `OK`.
