# Coverage - Wave 139 (`game::enemies`)

## How to read this file

This page records the Wave 139 `game::enemies` lane: 25 functions of REL module
3, `d_enemiesNP`, appended to `src/game/enemies.rs` as section 121 and mirrored
one row per body in `ledger/agents/wave139-enemies.csv`. Each `evidence` cell
names the Ghidra C export the body was reconstructed from; the byte counts are
the `size` column of `work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none of
these is a runtime address; every function doc repeats that note. Raw words were
decoded from `work/rels/elf/d_enemiesNP.elf` at the mapping of
`docs/REL_ELF_MAPPING.md` (`file_offset = address - 0x80A00000 + 0x1000`), and
every out-of-image branch target was cross-checked against the REL's module-0
import stream (each `R_PPC_REL24` entry carries the `main.dol` addend), so the
callee addresses below are the import-table values, not a hand-signed
displacement.

The lane has two halves:

- **12 bodies** are the ready rows of `docs/SPEC_WAVE138_ENEMIES.md` (ranks 1-12;
  rank 4 is the specification's single conditional row).
- **13 bodies** are still-open rows of the `0x80A80000` planner bucket (23
  untranslated before this pass, 10 after): seven single bodies and six
  `0x802DCBFC` deleting-destructor forwarders.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 25 | 1,492 | 15 | 10 |

Status split: 19 `translated` (15 high, 4 medium) and 6 `partial` (medium).

## What this pass covers

- **The two countdown effect timers.** `0x80AB1830` decrements `+0x984`, reloads
  `64` and tail-branches to `0x80078470` with
  `x = +0x884 + f32[0x80B2FB30 + 4*side]`, `y = +0x888`, layer `1`, count `1`.
  `0x80A01700` is the same shape with `24`, `+0x918`, `x = 24.0 + +0x8EC`,
  layer `3`. Both keep the field's "unset means fire now" polarity: the second
  test reads the value *after* the decrement.
- **Three stage-effect emitters.** `0x80B250F0` is skipped while bit 0 of
  `0x8042A200` is set and otherwise passes
  `layer = (u8)(*(u32*)(this+0x524) + 10)`, count `2`. `0x80A337C0` passes
  `+0x6CC`/`+0x6D0` with layer `2`, count `4` and is reached only through the
  4-byte `b` thunk at `0x80A337B0`. `0x80A36A50` is not an effect at all: it
  loads the side-indexed `i16` pair at `0x8042C1A4` with `lhax` and steps
  `+0x102` toward it by `0x600` through the ledgered `0x8015F4F0`.
- **Two effect-band thunks.** `0x80A0E160` (`+0x614`) and `0x80B07C90`
  (`+0x6C2`) are word-identical: `0x800D8B00(*(u32*)0x8042A008,
  extsb(player), 7, 2, 0, 0)`. A third thunk, `0x80A863D0`, is the same call with
  the byte at `+0xDFF`.
- **The three small record/state helpers.** `0x80AD42B0` accepts `+0x84C` values
  `0` and `2` and tail-branches to `0x80022100(this, this+0x854,
  extsb(+0x694))`. `0x80A281C0` binds `+0x1EC` through `0x8006D090` with the
  three in-image descriptors at `0x80B2B87C`/`0x80B2B888`/`0x80B2B898`.
  `0x80A3F630` returns for `+0xA7C` in `{0, 0xF, 0xD}` and otherwise tail-calls
  the retire step `0x80027A60`.
- **The two vtable wrappers.** `0x80B0C0D0` unregisters the `+0x8D4` collision
  area and returns `1`. `0x80AEDC90` loads `*(u32*)(this + 0x394)` with `lwzu`
  (so the receiver becomes `this + 0x394`), calls its `+0x10` slot and returns
  `1`.
- **The item state selector `0x80A84DB0`.** `fn_80a89430(this) != 0` clears
  `+0xDFB` and switches to the BSS state `0x80B737B8`; otherwise `+0xDCA`
  selects `0x80B737F8` (type `2`), `0x80B73838` (type `0xB`) or `0x80B73778`.
- **The band arming binder `0x80A85200`.** Both `+0xBD4` and `+0xBE0` receive
  `0x800` for `mirror == 0` and `0x80000800` otherwise, `+0xBD8`/`+0xBDC`/
  `+0xBE4` are zeroed, `+0xBE8` becomes `0x00010000`, and the body tail-branches
  to `0x8006D090(this+0x1EC, this, this+0xBD4, this+0xBE0, 0x80B2E530)`.
- **The flag accumulator `0x80A85470`.** While `+0xDFA == 3` it calls
  `0x80072CB0(this + 0x1EC)` twice, a third time when `u8[+0x34A] & 3 != 0`, and
  ORs the first result with the last one into `+0x274`. The third call replaces
  the second (`mr r30,r3` at `0x80A854E8`), so the store is `first | second` or
  `first | third`, not a three-way OR.
- **The height gate `0x80A8B0C0`.** Returns while `+0x6E8` is set; then, for
  `+0x6EC == 3`, returns while `+0x6DC - +0xB0` is below `0.0`, and for every
  other mode while it is above `0.0`; if the difference passes, it tail-calls
  `0x8007FD10` with that difference still live in `f1`.
- **The collision-area re-arm `0x80A85EF0`.** Sets bit `0x8000` of `+0x17C` and
  registers `+0x148` through the `main.dol` `0x8008C330`.
- **Six `0x802DCBFC` forwarders.** `0x80A8A910`, `0x80A8B9F0` and `0x80A8EB70`
  use the `+0x18` adjustor; `0x80A8A940`, `0x80A8BA20` and `0x80A8EBA0` use
  `+0x24`. Each is twelve words: `mr r5,r3`, `mr r3,r4`,
  `addi r12,r5,{0x18|0x24}`, `bl 0x802DCBFC`, `nop`.

### Confirmations and corrections worth recording

- **`bclr 4,0` / `bclr 4,1`, not `<=`.** Ghidra renders `0x80A8B0C0`'s second
  test as `if (diff <= limit) return;`. The word is `0x4C810020`
  (`bclr 4,1` = return on CR0[GT]); the first path's `0x4C800020` is
  `bclr 4,0` = return on CR0[LT]. The port keeps the bits: mode `3` fires on
  `diff >= 0.0`, every other mode on `diff <= 0.0`, and unordered fires.
- **`0x80A85470`'s two staged floats are dead.** The original stores `-1.0`
  (`0x80B2E5E8`) and `1.0` (`0x80B2E590`) at `8(r1)` and passes `&local` in
  `r4` before each `0x80072CB0` call, but the callee overwrites `r4`
  (`lwz r4,0x14(r31)`) and copies `*(f32*)(*(u32*)(record+0x14))` into
  `record + 0x4C`. The port calls the callee with the record only.
- **`0x80A85EF0`'s tail target is `0x8008C330`, not `0x8008C380`.** The REL
  module-0 import entry at text offset `0x85E10` carries the addend
  `0x8008C330`; `0x8008C330` is the 72-byte `main.dol` collision-area
  **register** that pairs with the already translated
  `0x8008C380` unregister path.
- **`0x80A86400` calls `0x800D8D30`, not `0x800D8B00`.** The import entry at
  `0x8632C` names `0x800D8D30`, the untracked function directly after the
  `FUN_800d8c60` extent. Its prologue saves `r3..r6` and it repeats the
  `extsb.` player test, so the row is `medium`.
- **The `0x80A84DB0` state tokens are BSS descriptors.** `0x80B73778`,
  `0x80B737B8`, `0x80B737F8` and `0x80B73838` are above the `0x80B6E500` BSS
  start, i.e. `fState` records rather than strings; `0x80B737B8` is the same
  token the already translated `fn_80a85d60` selects for a high item type.

## Functions

| address | function | bytes | status | confidence |
|---|---|---:|---|---|
| `0x80A01700` | `fn_80a01700` | 80 | translated | high |
| `0x80A0E160` | `fn_80a0e160` | 36 | translated | high |
| `0x80A281C0` | `fn_80a281c0` | 32 | translated | high |
| `0x80A337C0` | `fn_80a337c0` | 32 | translated | high |
| `0x80A36A50` | `fn_80a36a50` | 32 | translated | high |
| `0x80A3F630` | `fn_80a3f630` | 32 | translated | medium |
| `0x80A84DB0` | `fn_80a84db0` | 204 | translated | high |
| `0x80A85200` | `fn_80a85200` | 92 | translated | high |
| `0x80A85470` | `fn_80a85470` | 168 | translated | high |
| `0x80A85EF0` | `fn_80a85ef0` | 20 | translated | high |
| `0x80A863D0` | `fn_80a863d0` | 36 | translated | high |
| `0x80A86400` | `fn_80a86400` | 32 | translated | medium |
| `0x80A8A910` | `fn_80a8a910` | 48 | partial | medium |
| `0x80A8A940` | `fn_80a8a940` | 48 | partial | medium |
| `0x80A8B0C0` | `fn_80a8b0c0` | 100 | translated | medium |
| `0x80A8B9F0` | `fn_80a8b9f0` | 48 | partial | medium |
| `0x80A8BA20` | `fn_80a8ba20` | 48 | partial | medium |
| `0x80A8EB70` | `fn_80a8eb70` | 48 | partial | medium |
| `0x80A8EBA0` | `fn_80a8eba0` | 48 | partial | medium |
| `0x80AB1830` | `fn_80ab1830` | 92 | translated | high |
| `0x80AD42B0` | `fn_80ad42b0` | 36 | translated | high |
| `0x80AEDC90` | `fn_80aedc90` | 48 | translated | medium |
| `0x80B07C90` | `fn_80b07c90` | 36 | translated | high |
| `0x80B0C0D0` | `fn_80b0c0d0` | 40 | translated | high |
| `0x80B250F0` | `fn_80b250f0` | 56 | translated | high |

### Why six rows are `partial`

The six `0x802DCBFC` forwarders pass `(param_2, param_2, param_1)` exactly, but
the original also loads `r12 = param_1 + {0x18|0x24}` as a hidden
this-adjust pointer. `platform::call3` cannot model a fourth, non-ABI register
operand, so the adjustor is documented and left out. This is the same limitation
the wave-135/136/137/138 forwarders record.

### Why four `translated` rows are `medium`

- **`0x80AEDC90`** - the receiver chain and the `li r3,1` are exact, but slot
  `+0x10`'s argument list is not established; the port passes only the receiver
  and cannot model the original's live `r4..r10`.
- **`0x80A3F630`** - the three guards and the `0x80027A60` tail branch are
  literal, but the callee has no `main.dol` export, so its retire role is
  inferred from the wave-133 group.
- **`0x80A86400`** - the words and the `0x800D8D30` tail target are exact, but
  the callee has no `main.dol` export, so its role is inferred from its position
  after `FUN_800d8c60` and its identical `extsb.` player test.
- **`0x80A8B0C0`** - the guards are literal, but `0x8007FD10`'s role is inferred
  from its disassembly (it adds the delta to `*(i32*)(object + 0x84)` and mirrors
  it at `+0xB4`).

## Deliberate skips

Six still-open `0x80A80000` bucket rows read `unaff_GQR0`/`unaff_GQR3` and emit
`ldexpf` fiction, so they stay class C:

- `0x80A80E50` (220), `0x80A80F40` (264), `0x80A81150` (456), `0x80A84F10` (744),
  `0x80A87030` (732) and `0x80A88BC0` (368).

Four GQR-free bucket rows were left for a later pass rather than padded in:

- `0x80A84A00` (400) - a ten-iteration render loop whose three-float call at
  `0x801C0BD0` is printed with `(double)` casts, i.e. an unresolved float ABI.
- `0x80A85260` (528) - the item `+0xDFA` fall/land state machine. All of its
  callees (`0x80072BC0`, `0x800976B0`, `0x8006EF40`, `fn_80a86070`,
  `fn_80a865e0`, `fn_80a85d60`) are already translated, but the four-way switch,
  the `countLeadingZeros`-style side test and six in-image constants need their
  own verification pass, so it was left for a dedicated lane rather than rushed.
- `0x80A85650` (976) - the item's large predicate/handler tree over
  `0x8005F900`/`0x8004DD00`; GQR-free but long enough that its many
  `0x80A863D0`-style calls deserve a dedicated pass.
- `0x80A8BD20` (640) - the `kurage` constructor; it is a `_savegpr` prologue
  body with a `bctrl` and seven in-image name strings
  (`0x80B4F57C..0x80B4F5F4`), and the family's other members are already
  translated, so it deserves a dedicated pass.

No other `d_enemiesNP` address was touched, and no address that already had a
`ledger/functions.csv` row or a `fn_<addr>` body in `src/game/enemies.rs` was
re-claimed.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 25 sizes sum to 1,492
  bytes (552 for the twelve specification rows, 940 for the thirteen bucket
  rows). Every reachable span equals the TSV extent.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice, and none of the 25 was in
  `ledger/functions.csv` before this pass.
- The module header was updated: cluster item 121, the cluster count (120 to
  121), the function totals (3,228 to 3,253 public translations, 3,157 to 3,182
  `fn_<addr>` bodies), the "start here" list, the wave count line and the records
  list. Three stale "untranslated" labels for `0x80A0E160`, `0x80AB1830` and
  `0x80A36A50` were corrected because those bodies now exist.
- `cargo check --message-format=short` emits no error and no warning naming
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave139` reports 25 rows for this lane
  (`translated/high=15`, `translated/medium=4`, `partial/medium=6`), 25 unique
  addresses and `OK`.
