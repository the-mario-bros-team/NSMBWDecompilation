# Coverage - Wave 138 (`game::enemies`)

## How to read this file

This page records the Wave 138 `game::enemies` lane: 28 functions of REL module 3,
`d_enemiesNP`, appended to `src/game/enemies.rs` as section 120 and mirrored one
row per body in `ledger/agents/wave138-enemies.csv`. Each `evidence` cell names
the Ghidra C export the body was reconstructed from; the byte counts are the
`size` column of `work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none of
these is a runtime address; every function doc repeats that note. Raw words were
decoded from `work/rels/elf/d_enemiesNP.elf` at the mapping of
`docs/REL_ELF_MAPPING.md` (`file_offset = address - 0x80A00000 + 0x1000`), and
floating-point constants are read from the image at run time with
`platform::read_u32` / `read_f32` / `read_f64`, never transcribed as invented
values.

The lane has two halves:

- **12 bodies** are the ready rows of `docs/SPEC_WAVE137_ENEMIES.md` (ranks 1-12,
  all twelve `ready`). No row was already ledgered and none had a `fn_80...`
  body.
- **3 bodies** are the rows that the same page names as wave 138's "next band":
  `0x80A2B490`, `0x80AA39D0` and `0x80A22B20`.
- **13 bodies** are still-open rows of the `0x80A60000` planner bucket
  (23 untranslated before this pass, 10 after): eleven 48-byte `0x802DCBFC`
  deleting-destructor forwarders, one constant producer and one frame gate.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 28 | 1,732 | 13 | 15 |

Status split: 17 `translated` (13 high, 4 medium) and 11 `partial` (medium).

## What this pass covers

- **The five 22-word deleting destructors.** `0x80A43DC0`, `0x80A446B0`,
  `0x80AECF00`, `0x80AEE6D0` and `0x80AEF910` are slot 0 of the `Wm_ob_coin`,
  CoinJugem, RailPolyChild, RailPolyParent and RailPoly vtables. All five are
  word-identical to the ledgered `fn_80a1f390`: guard `this`, release with
  `0x8015F800(this, 0)`, free with `0x802B9280(this)` only for a positive
  `free_count`, and return the incoming `this`.
- **The Icicle `Vibration` step `0x80A7ECA0`.** The `+0x61C` guard and the
  `+0xD4` class slot called with `0x80B73338`, then the 8-byte thunk
  `0x80A7E630` (`addi r3,r3,0x5FC`, `b 0x800944C0`) with `r3 = this`.
- **The Teresa two-handle predicate `0x80B0D150`.** `0x80169020(this + 0x640)`
  and `0x80169020(this + 0x678)` must both be bound; the body returns `1`/`0`.
- **The bounce-ball helper `0x80A2C2A0`.** `-(i16)(i32)trunc(x * x)` with
  `x = *(f32*)(this + 0xE8)`. The `0x80A2C2A4..C8` `2^52` block converts the
  literal `4096` into `f1` and nothing reads it; the Ghidra export's
  `4.5e15 - DAT_80b2bbb8` factor is that dead fold, not a yaw scale.
- **The SyncroBarnar seed `0x80B0AF60`.** `fn_80b0ac50(this)` finds the
  class-`0x99` record; its first word lands in `+0x9B4` and `fn_80b0abf0` seeds
  the three timers.
- **The Teresa latch reset `0x80B0DAF0`.** `+0x9E0` is cleared first, then
  `0x80B0CD80(this, 0)` runs **when `+0x9E7` is clear** (`bc 4,2` = `bne` past
  the call), then `+0x9E7` is cleared. This inverts the wave-115/127 prose; the
  bytes and the Ghidra export agree.
- **The Hnswich release helper `0x80A78700`.** `+0x124 = 0`,
  `0x8008C380(this + 0x148)` and `0x8007F950(this + 0x754)`; a second entry
  whose 16 words equal the already translated `fn_80a786a0`.
- **The `0xA5` spawn thunk `0x80AA0300`.** Thirteen words decoded from the ELF
  (the export is `halt_baddata`); the tail branch resolves to `0x80064680` with
  `r3 = 0xA5`, `r4 = actor`, `r5 = u16[+0x872] | (u8[+0x348] << 4) |
  (u8[+0x876] << 8)`, `r6 = actor + 0x75C`, `r7 = r8 = 0`.
- **The two `2^52` constant producers `0x80A6F8C0` (30) and `0x80A2B490` (27).**
  Both build `2^52 + 2^31 + n`, load the bias `0x4330000080000000` and subtract
  with `fsubs`; because single-precision arithmetic rounds both operands
  first, the shipped result is `0.0`, not `n`.
- **The four-slot class-word selector `0x80AA39D0`.** `index >= 4` returns `0`,
  a null `+0x914 + 4*index` slot returns `0`, otherwise the body tail-branches to
  `0x80162D00(slot)`.
- **The height gate `0x80A22B20`.** `fcmpo(+0xB0, 8.0 + *(u32*)0x80429E00 +
  0x3C)` followed by `cror 2,1,1` + `bnelr`, i.e. "return unless strictly
  greater (unordered also returns)", else tail-branch to `0x80162510(this)`.
- **Eleven `0x802DCBFC` forwarders.** `0x80A61410`, `0x80A61440`, `0x80A656A0`,
  `0x80A656D0`, `0x80A69130`, `0x80A69160`, `0x80A6A5F0`, `0x80A6A620`,
  `0x80A6BA00`, `0x80A6BA30` and `0x80A6C7E0`. Each is twelve words:
  `mr r5,r3`, `mr r3,r4`, `addi r12,r5,{0x18|0x24}`, `bl 0x802DCBFC`, `nop`.
- **The frame gate `0x80A61E10`.** `+0x736` increments every frame; when the new
  byte passes `& 0x3F == 0` the body tail-branches to `0x80078470` with
  `r3 = *(u32*)0x80429DD0`, `r4 = r5 = 1`, `f1 = 2.0 + 8.0` and
  `f2 = *(f32*)(r3 + 0x714)` (`256.0` in the shipped image).

### Confirmations and corrections worth recording

- **`0x80B0DAF0`'s polarity.** `docs/COVERAGE_WAVE115_ENEMIES.md` and the wave-127
  table say the reset call runs when `+0x9E7` is set. The word at `0x80B0DB14` is
  `0x4082000C` (`bc 4,2` = `bne` to the instruction after the call), so the call
  runs when `+0x9E7 == 0`, and the byte cleared afterwards is `+0x9E7` while
  `+0x9E0` is a `u16` cleared before the test. The Ghidra export agrees.
- **`0x80A2C2A0` is not GQR-damaged.** All nineteen words are plain
  `lfs`/`fmuls`/`fctiwz`; `0x80A2C2C8` writes `f1` and nothing reads it, so the
  `2^52` block is dead and the result is `-(i16)(x*x)`.
- **`0x80A6F8C0` / `0x80A2B490` return `0.0`, not 30/27.** The `fsubs` at the end
  of both bodies (primary opcode 59) converts both `f64` operands to single
  precision before subtracting, so `2^52 + 2^31 + n` and `2^52 + 2^31` collapse
  to the same value. Ghidra prints the double difference
  (`4503601774854174.0 - DAT_80b2dac0` and `4503601774854171.0 - DAT_80b2bb18`).
  The ports evaluate the same single-precision subtraction, not the double one.
- **`0x80A78700` keeps its own body.** The sixteen words equal `fn_80a786a0`
  except the two `bl` displacements; the repo convention is one body per entry,
  so this is a copy rather than a forwarder.
- **`0x80AA0300` has no usable export.** `work/ghidra/.../80aa0300_FUN_80aa0300.c`
  is `halt_baddata`; the thirteen words were decoded from the ELF and the tail
  branch resolves to `0x80064680` (`+0x5C4350 >> 2` style displacement).
- **`0x80A22B20`'s comparison is "greater", not "less".** Ghidra prints
  `if (height < limit) return;`; the actual sequence is `fcmpo` + `cror 2,1,1` +
  `bnelr`, which also returns on equality and on unordered.

## Functions

| address | function | bytes | status | confidence |
|---|---|---:|---|---|
| `0x80A22B20` | `fn_80a22b20` | 44 | translated | medium |
| `0x80A2B490` | `fn_80a2b490` | 48 | translated | high |
| `0x80A2C2A0` | `fn_80a2c2a0` | 76 | translated | high |
| `0x80A43DC0` | `fn_80a43dc0` | 88 | translated | high |
| `0x80A446B0` | `fn_80a446b0` | 88 | translated | high |
| `0x80A61410` | `fn_80a61410` | 48 | partial | medium |
| `0x80A61440` | `fn_80a61440` | 48 | partial | medium |
| `0x80A61E10` | `fn_80a61e10` | 64 | translated | medium |
| `0x80A656A0` | `fn_80a656a0` | 48 | partial | medium |
| `0x80A656D0` | `fn_80a656d0` | 48 | partial | medium |
| `0x80A69130` | `fn_80a69130` | 48 | partial | medium |
| `0x80A69160` | `fn_80a69160` | 48 | partial | medium |
| `0x80A6A5F0` | `fn_80a6a5f0` | 48 | partial | medium |
| `0x80A6A620` | `fn_80a6a620` | 48 | partial | medium |
| `0x80A6BA00` | `fn_80a6ba00` | 48 | partial | medium |
| `0x80A6BA30` | `fn_80a6ba30` | 48 | partial | medium |
| `0x80A6C7E0` | `fn_80a6c7e0` | 48 | partial | medium |
| `0x80A6F8C0` | `fn_80a6f8c0` | 48 | translated | high |
| `0x80A78700` | `fn_80a78700` | 64 | translated | high |
| `0x80A7ECA0` | `fn_80a7eca0` | 84 | translated | high |
| `0x80AA0300` | `fn_80aa0300` | 52 | translated | high |
| `0x80AA39D0` | `fn_80aa39d0` | 48 | translated | high |
| `0x80AECF00` | `fn_80aecf00` | 88 | translated | high |
| `0x80AEE6D0` | `fn_80aee6d0` | 88 | translated | high |
| `0x80AEF910` | `fn_80aef910` | 88 | translated | high |
| `0x80B0AF60` | `fn_80b0af60` | 76 | translated | medium |
| `0x80B0D150` | `fn_80b0d150` | 84 | translated | high |
| `0x80B0DAF0` | `fn_80b0daf0` | 76 | translated | medium |

### Why eleven rows are `partial`

The eleven `0x802DCBFC` forwarders pass `(param_2, param_2, param_1)` exactly,
but the original also loads `r12 = param_1 + {0x18|0x24}` as a hidden
this-adjust pointer. `platform::call3` cannot model a fourth, non-ABI register
operand, so the adjustor is documented and left out. This is the same limitation
the wave-135/136/137 forwarders record.

### Why four `translated` rows are `medium`

- **`0x80A22B20`** - the comparison sequence and the tail target are literal,
  but the `f1` threshold that is live at the tail branch is not read by
  `0x80162510` (every other caller passes `r3` only).
- **`0x80A61E10`** - the sixteen words are exact, but the callee `0x80078470`
  has no `functions.tsv` row and no export, so its role is inferred from the
  `(r3..r5, f1, f2)` register shape.
- **`0x80B0AF60`** - `fn_80b0ac50` is the one-argument recursive search, so the
  call site's `li r4,0` is a dead register fill rather than a second argument.
- **`0x80B0DAF0`** - the polarity is fixed by the bytes, but the callee
  `0x80B0CD80` (292 bytes) is still untranslated and is called by address.

## Deliberate skips

Ten still-open `0x80A60000` bucket rows are left for a later pass because every
one of them reads `unaff_GQR0`/`unaff_GQR3` and emits `ldexpf` fiction; per
`docs/REL_ELF_MAPPING.md` they stay class C until the GQR3 word is pinned:

- `0x80A62A30` (312), `0x80A635D0` (488), `0x80A637E0` (236),
  `0x80A638F0` (284), `0x80A63A30` (284), `0x80A63FF0` (392),
  `0x80A641F0` (380), `0x80A673D0` (1080), `0x80A6EA20` (212) and
  `0x80A6FBC0` (624).

No other `d_enemiesNP` address was touched, and no address that already had a
`ledger/functions.csv` row or a `fn_<addr>` body in `src/game/enemies.rs` was
re-claimed.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 28 sizes sum to 1,732
  bytes (952 for the twelve specification rows, 140 for the three next-band
  rows, 640 for the thirteen bucket rows). `0x80AA39D0` is the one row whose
  reachable span (52 bytes) exceeds the TSV extent (48); the doc block says so.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice, and none of the 28 was in
  `ledger/functions.csv` before this pass.
- The module header was updated: cluster item 120, the cluster count (119 to
  120), the function totals (3,200 to 3,228 public translations, 3,129 to 3,157
  `fn_<addr>` bodies), the "start here" list and the records list.
- `cargo check --message-format=short` emits no error and no warning naming
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave138` reports 28 rows for this lane
  (`translated/high=13`, `translated/medium=4`, `partial/medium=11`), 28 unique
  addresses and `OK`.
