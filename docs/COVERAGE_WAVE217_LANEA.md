# Coverage - Wave 217 lane G (engine::dsys)

## How to read this file

`engine::dsys` reconstructs the `dSys_c` engine start-up together with the
display, text, sprite, world-record and sequence helpers around it in the main
`DOL`. This lane takes ten bodies from the `0x800F0000` bucket, between
`0x800F78E0` and `0x800F8730`.

Each row below is one `pub unsafe fn fn_<address>`. The doc block above the body
cites the Ghidra export, the byte count from
`work/ghidra/main-gb/export/functions.tsv`, and a confidence level:

- `high` means every guard, store and call argument is literal in the export;
- `medium` means a field role, a callee identity or a structure layout was
  inferred and is marked `// inferred:` or `// unknown:` in the body.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/dsys.rs` | `engine::dsys` | 10 | 1968 | 6 | 4 |

The lane's bodies live in `work/wave217/laneG_block.rs`; the row list is
`ledger/agents/wave217-laneG.csv`.

## What this pass covers

Two clusters share the same 0x6C4-relative object: the sequence state machine
(`+0x6B2` target angle, `+0x6B6` started byte, `+0x6B8` latch selector,
`+0x6BC` frame counter, `+0x6C4` state word, `+0x6DC` sound handle) and the
world-record vector read above it.

| address | function | bytes | role |
|---|---|---:|---|
| `0x800F78E0` | `fn_800f78e0` | 12 | clear the state word at `+0x6C4` |
| `0x800F7970` | `fn_800f7970` | 64 | run vtable `+0x98` and enter state `3` |
| `0x800F7A20` | `fn_800f7a20` | 32 | enter state `2`, arm `+0x6D4`, vtable `+0xA0` |
| `0x800F7AE0` | `fn_800f7ae0` | 76 | enter state `4`, seed `+0x6C8` from vtable `+0xE8` |
| `0x800F7DA0` | `fn_800f7da0` | 636 | the per-frame step: snap, retarget, sound, finish |
| `0x800F8050` | `fn_800f8050` | 212 | `record[+0xC] + vtable 0xAC vector` into a caller cell |
| `0x800F81F0` | `fn_800f81f0` | 64 | latch vtable `+0xB0` into `+0x102`/`+0x108` |
| `0x800F8230` | `fn_800f8230` | 652 | the id event handler `4`/`0x56`/`0x59`/`0x5B` |
| `0x800F8540` | `fn_800f8540` | 160 | step the angle toward the target, report "arrived" |
| `0x800F8730` | `fn_800f8730` | 60 | refresh the world record of `object + 0x184` |

### 1. Byte sizes against the prologue and the `blr`

Every size was re-checked against `work/dol/main.elf`:

| address | prologue | last return | real extent | `functions.tsv` |
|---|---|---|---|---|
| `0x800F78E0` | none | `bclr` at `0x800F78E8` | 12 | 12 |
| `0x800F7970` | `stwu r1,-0x10` | `bclr` at `0x800F79AC` | 64 | 64 |
| `0x800F7A20` | none | `bcctr` at `0x800F7A3C` | 32 | 32 |
| `0x800F7AE0` | `stwu r1,-0x10` | `bclr` at `0x800F7B28` | 76 | 76 |
| `0x800F7DA0` | `stwu r1,-0x10` | `bclr` at `0x800F8018` | 636 | 636 |
| `0x800F8050` | `stwu r1,-0x30` | `bclr` at `0x800F8120` | 212 | 212 |
| `0x800F81F0` | `stwu r1,-0x10` | `bclr` at `0x800F822C` | 64 | 64 |
| `0x800F8230` | `stwu r1,-0x20` | `bclr` at `0x800F84B8` | 652 | 652 |
| `0x800F8540` | `stwu r1,-0x10` | `bclr` at `0x800F85DC` | 160 | 160 |
| `0x800F8730` | `stwu r1,-0x10` | `bclr` at `0x800F8768` | 60 | 60 |

All ten match exactly; none is a 16/20-byte prologue artefact. Two of them do
not end in `blr`:

- `0x800F7A20` ends in a tail branch `bcctr` at `0x800F7A3C` (its `ctr` holds
  vtable slot `+0xA0`), so the `bclr` word at `0x800F7A40` is padding;
- `0x800F8730` ends with `bl 0x800F8770` followed by its own `bclr`, which is
  inside the 60 bytes.

`0x800F8730`'s neighbour `0x800F876C` also holds an alignment `bclr`; it is not
part of either body.

### 2. The dead `cntlzw` and the `rlwinm` argument of `0x800F8980`

`0x800F7DA0` builds the second argument of `0x800F8980` in the most surprising
way of this lane:

```text
0x800F7EEC  cntlzw  r0,r3                 ; r3 = (x == 1) ? 0 : 1
0x800F7F00  lfs     r0,-0x60A8(r2)        ; 0x8042CFD8 = 0x00000000
0x800F7F04  rlwinm  r4,r0,27,5,31
0x800F7F1C  bl      0x800F8980
```

The `cntlzw` result is **overwritten by the `lfs`** before the rotate reads
`r0`, so it is dead, and the argument is
`ROL32(0x00000000, 27) & 0x07FFFFFF` = `0`. That is also why the export prints
a `countLeadingZeros(...) >> 5` expression next to an unrelated `ldexpf` block:
Ghidra pairs the dead `cntlzw` with the `rlwinm` that follows the load.

The reconstruction replays both the store and the rotate/mask on the live word
at `0x8042CFD8` (rather than writing the literal `0`), so the call stays
byte-exact whatever that word holds, and the doc block records that the
`cntlzw` is dead.

### 3. One vtable slot, two argument shapes

Slot `+0xB8` of the object vtable is called three times in this lane, with two
different register states:

| call site | `r3` | `f1` |
|---|---|---|
| `0x800F7E6C` (`0x800F7DA0`) | object | not set |
| `0x800F7F48` (`0x800F7DA0`) | object | `r2 - 0x60A4` = `1.0f` |
| `0x800F8454` (`0x800F8230`) | object | `r2 - 0x60A4` = `1.0f` |

The two shapes are reproduced as they stand (`call1` for the first,
`call_f32_u32_g` for the other two) rather than normalised, because the image
does not set `f1` at the first site. `// inferred:` the slot is the "apply
frame delta / set speed" step, whose float argument is optional.

### 4. The paired-single vector add of `0x800F8050`

The export of `0x800F8050` is dominated by `ldexpf`/`GQR` blocks: those are the
`psq_l`/`psq_st` decode noise of the two paired-single loads and stores. The
real body is three float adds:

```text
psq_l   f0,0xC(r31)        ; record +0xC, +0x10
psq_l   f1,0x0(r4)         ; stack cell +0x0, +0x4
ps_add  f2,f1,f0
psq_l   f0,0x14(r31),1,0   ; record +0x14
psq_l   f1,0x8(r4),1,0     ; stack cell +0x8
ps_add  f2,f1,f0
psq_st  f2,0x0(r3) / 0x8(r3),1,0
```

The cell is filled by vtable slot `+0xAC(out, object, entry[+4])`, and the
record comes from `0x800F32F0` on the scene's
`0x5C7F4 + index * 0x3F08` table. The three sums land in `param_1[0..3]`.

### 5. `0x800F8230` is one dispatch table split in two

The handler looks like two near-identical switches; it is really one `enter`
block (guarded by `param_3`) and one `state` block. Their actions differ on
purpose: the enter block calls the "start" slots (`+0x7C`, `+0x84`, `+0x8C`) and
arms the `+0x6BC` counter, while the state block calls the "query" slots
(`+0x80`, `+0x88`, `+0x90`) and answers with `0`/`1`.

Two details were checked against the disassembly rather than the export:

- the `+0x6BC` guard is `bc 4,1` after `cmpi cr0,0,r0,0`, i.e. branch when
  `r0 <= 0`; the body runs when the counter is positive (the export's `0 <`),
  which the reconstruction writes as `counter > 0`;
- the final decrement is a second, independent
  `lwz/addi -1/stw` after the `+0x68C = +0x690` copy, so the counter loses two
  frames in that path. Both decrements are reproduced.

### 6. Scene-record lookups

`0x800F8050` and `0x800F8730` both read the scene object at `r13 - 0x54F4`
(`0x8042A18C`, the word `game::bases` calls the scene table):

| body | table | stride |
|---|---|---|
| `0x800F8050` | `scene + 0x5C7F4` | `0x3F08` |
| `0x800F8770` (called by `0x800F8730`) | `scene + 0x3394` | `0x16518` |

The scene index is `scene + 0x338C`. `0x800F71C0(object + 0x184)` produces the
key for the `0x54`-byte descriptor lookup; both helpers are already translated
in this module and are called directly.

## Deliberately skipped

None. All ten bodies were reconstructed.

## Field notes

- `0x800F7AE0`'s `extsh r3,r3` is not a decode artefact: the state word at
  `+0x6C8` is `sext16(vtable +0xE8) + 1`, which the export prints as a `short`
  intermediate plus one.
- `0x800F8540` and `0x800F7DA0` drive the same `0x8015F200` stepper with
  different ramps: `100`/`0x4000`/`0x800` in `0x800F8540`, and
  `10`/`0x2000`/`0x400` inside `0x800F7DA0`.
- All three-quarter snapshots are three separate `sth` stores
  (`+0x100 = +0x106`, `+0x102 = +0x108`, `+0x104 = +0x10A`); none of them
  writes `+0x106`, which the stepper updates itself.
- `0x800F8230` reads both SDA words (`r13 - 0x54D4` and its `+0x1AC`) before it
  tests `param_2 == -1`, so the call is placed before the early return.
- `0x800F8770` is not translated; it ignores both of its arguments and returns
  "first character == `'K'`". `0x800F8730` discards that answer, so only its
  lookup side effect is recorded.

## Verification

- Every address and byte size matches
  `work/ghidra/main-gb/export/functions.tsv` (`800f78e0` 12, `800f7970` 64,
  `800f7a20` 32, `800f7ae0` 76, `800f7da0` 636, `800f8050` 212, `800f81f0` 64,
  `800f8230` 652, `800f8540` 160, `800f8730` 60); the prologue/`blr` extents in
  section 1 agree with all ten.
- Every evidence file exists at the cited path.
- Ten `pub unsafe fn fn_<address>` bodies, one per ledger row.
- No helper or constant defined here duplicates an existing one: new constants
  carry the lane suffix `_G`, and `G_SOUND_5404`, `G_COUNTER_54D4_B`,
  `G_SHELL_BYTE_5433_C`, `FN_8006CC00_C`, `FN_8006CD40_C`, `read_f32`,
  `fn_800f71c0`, `fn_800f32f0`, `fn_800f6500`, `fn_800f7270` and
  `fn_800f6cd0` are reused as they already stand.
- `cargo check --lib` in a scratch copy of the crate with the lane block
  appended to `src/engine/dsys.rs` exits `0` with no warning from the new code.
- `node tools/check-lane-csv.mjs wave217` reports `OK` with 10 rows for this
  lane.
