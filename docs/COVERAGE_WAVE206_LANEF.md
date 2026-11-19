# Coverage - Wave 206, lane F (`ui::hud`, `0x80035320..0x80035BA4`)

Ten functions of `src/ui/hud.rs` translated from `sys/main.dol` in wave 206,
lane F. All ten have a faithful Rust body in `work/wave206/laneF_block.rs`
(appended to `src/ui/hud.rs` on merge); nothing in this lane is deferred.

> Filename note: the shared brief spells this document
> `docs/COVERAGE_WAVE205_LANE<X>.md`, but `docs/COVERAGE_WAVE205_LANEF.md`
> already exists (the wave-205 lane F report, a different address range).
> Overwriting it would destroy that record, so this lane writes
> `docs/COVERAGE_WAVE206_LANEF.md` instead.

## Scope

| address | size (`functions.tsv` / real) | name | confidence | role |
|---|---|---|---|---|
| `0x80035320` | 88 / 88 | `fn_80035320` | high | `0x8015F800(this, 0)` release wrapper, optional `0x802B9280` free |
| `0x80035380` | 92 / 92 | `fn_80035380` | high | same wrapper with a dead second null test |
| `0x800353E0` | 12 / 12 | `fn_800353e0` | high | return `0x80357BF4` |
| `0x800354D0` | 224 / 220 | `fn_800354d0` | high | walk the `+0x30` child chain by probing virtual `+0x0C` |
| `0x800355B0` | 136 / 132 | `fn_800355b0` | high | compare the `':'` tails of `key` and the object's virtual `+0x20` name |
| `0x80035670` | 48 / 44 | `fn_80035670` | high | forward `(b, b, a)` to `0x802DCBFC`, interface `a + 0x18` in `r12` |
| `0x800356A0` | 48 / 44 | `fn_800356a0` | high | byte-identical twin of `0x80035670` |
| `0x800356D0` | 300 / 296 | `fn_800356d0` | medium | build the actor: model records, variant fields, physics descriptor, two virtual slots |
| `0x80035820` | 308 / 304 | `fn_80035820` | high | load `g3d/nokonokoB.brres` and bind four animation records |
| `0x80035B10` | 152 / 148 | `fn_80035b10` | medium | convert the `+0x5F0` variant and initialise the `+0x5C4` clip/rate |

Total: 1,200 bytes of original code (`functions.tsv` sizes); 1,184 bytes up to
and including each terminal `blr`.

## Size checks

Every `functions.tsv` row was re-checked against the prologue and the closing
`blr`; the TSV value includes the alignment padding (0, 4 or 8 zero bytes)
before the next entry, except for the two accessor/wrapper bodies that end
exactly on their TSV size.

- `0x80035B10` is the one row the task brief reports wrongly: it lists `28`
  bytes, but `functions.tsv` says 152 and the stream runs
  `0x80035B10..0x80035BA4` (the prologue saves `f31` via `stfd`/`psq_st`, and
  the body contains five calls before the epilogue). The `# Original` line
  cites the TSV value, 152, and the prose records the 148-byte real extent.
- `0x800356D0` ends on its `blr` at `0x800357F8`; the next entry
  (`0x80035800`) is a separate four-byte `blr` stub that is not part of it.

## What the cluster is

The run is the `nokonokoB` (Koopa Troopa) actor prologue. The four bodies
before it are the shared object helpers that the actor class uses:

1. **Release wrappers** (`0x80035320`, `0x80035380`). Both call
   `0x8015F800(this, 0)` for a non-null record and free it through
   `0x802B9280` only for a strictly positive flag. `0x80035380` emits a second
   `beq cr0,2` against the `cr0` the first branch already resolved at
   `0x800353A0`; that arm is never taken, so the body has no extra parameter.
   Both are byte-identical to the already translated `fn_8002ec10`.
2. **Constant accessor** (`0x800353E0`) returning `0x80357BF4`, the same
   `lis`/`addi`/`blr` shape as `fn_800268b0`.
3. **Child-chain probe** (`0x800354D0`). Five unrolled levels, each doing
   `child = *(node + 0x30)` and probing virtual slot `+0x0C` of that child,
   then a tail recursion on `+0x30`. The unrolled levels are a loop, which is
   how the body is written; byte-identical to `fn_8002ed50`.
4. **Name-tail comparison** (`0x800355B0`). `0x802E1C40` finds the first `:`
   in the key and in the object's virtual `+0x20` name, and `0x802E1AB4`
   compares the two tails. Byte-identical to `fn_8002ee30`.
5. **Interface forwards** (`0x80035670`, `0x800356A0`). `mr r5,r3` /
   `mr r3,r4` / `addi r12,r5,0x18` / `bl 0x802DCBFC` / `ori r0,r0,0`, i.e. the
   `(b, b, a)` shape with the `+0x18` descriptor selector; the callee's jump
   table is still unrecovered, so the `r12` operand is documented in prose
   rather than in the call (as at every other `0x802DCBFC` site).

The three actor bodies then consume the model layer:

6. **Constructor** (`0x800356D0`). Calls `0x80035820` first, latches the
   variant `+0x5F0 = flags & 1`, resets `+0x484` through `0x800B86F0`, writes
   six floats from the SDA2 constants (`+0xDC/+0xE0/+0xE4 = 1.0f`,
   `+0xD0/+0xD8/+0x320 = 0.0f`, `+0xD4/+0x324 = 16.0f`), installs the
   physics/contact descriptor with `0x8006D090(+0x1EC, this, 0x802EE6E8,
   0x802EE6DC, 0x802EE6F4)`, installs and applies the model group with
   `0x8008C3E0(+0x148, this, 0x802EE6B8)` / `0x8008C330(+0x148)`, then derives
   three more fields from the same flags word. `0x80035B10` runs next, and the
   body closes with virtual slots `+0x2A0` and `+0x2A4` of the `+0x60` owner
   around `+0x4FC = 6`.
7. **Model binder** (`0x80035820`). `0x800690C0(+0x528, -1, *(0x80377C48), 0,
   0x20)`; `0x800DF180(*(0x8042A038) + 4, "nokonokoB",
   "g3d/nokonokoB.brres")` into `+0x544`; `0x80239E30` + `0x80169D90` (id
   `0x23`, flags `1`, `0`) + `0x80064BD0` for the primary record at `+0x548`;
   `0x8023A0B0(+0x544, "net_walk1")` with `0x801650D0` into `+0x588`;
   `0x8023A200(+0x544, "nokonokoB")` into `+0x5C0` with `0x80166830` into
   `+0x5C4`; `0x80166BD0` starts the pair; `0x800690E0(+0x528)` closes the
   buffer. The export prints the closing call as `thunk_FUN_8016a8e0`, while
   the raw `bl` targets `0x800690E0`, the one-instruction thunk to
   `0x8016A8E0`.
8. **Clip/rate initialiser** (`0x80035B10`). The `+0x5F0` variant is converted
   to `f32` with the `2^52` bias trick (`xoris` + `lfd 0x43300000` +
   `fsubs`), `0x80166F40(+0x5C4, 1, 0)` writes status byte `1`,
   `0x80169F80(0.0f, +0x548, +0x5C4)` pairs the records, and
   `0x80166EF0(rate, +0x5C4, 0)` / `0x80166F10(0.0f, +0x5C4, 0)` write the two
   tracks.

## Field map (observed)

| offset | meaning | written by |
|---|---|---|
| `+0x102` | `s16` turn angle from `s16[0x8042B540 + ((flags >> 3) & 2)]` (`-32768` or `0`) | `0x800356D0` |
| `+0x148` | model group record | `0x8008C3E0`/`0x8008C330` |
| `+0x1EC` | physics/contact block | `0x8006D090` |
| `+0x320/+0x324` | `0.0f` / `16.0f` | `0x800356D0` |
| `+0x36D` | ready byte `1` | `0x800356D0` |
| `+0x391` | per-slot selector `(flags >> 4) & 1` (the slot `0x8042B540[slot]` uses) | `0x800356D0` |
| `+0x484` | sub-record reset with `(0, 3, 3)` | `0x800B86F0` |
| `+0x4FC` | state word: `|= 2`, then `6` | `0x800356D0` |
| `+0x518` | count `3` | `0x800356D0` |
| `+0x528` | model buffer | `0x800690C0`/`0x800690E0` |
| `+0x544` | archive/model bank | `0x800DF180` |
| `+0x548` / `+0x588` | primary / secondary animation record | `0x80169D90`, `0x801650D0` |
| `+0x5C0` / `+0x5C4` | damage clip word / damage record | `0x8023A200`, `0x80166830` |
| `+0x5F0` | variant word `flags & 1` | `0x800356D0` |

`flags` is the two-bit field word at `+0x04`; bits 0, 3 and 4 are consumed.

## Constants added (`_F` suffix)

`FN_8008C3E0_F`, `FN_8008C330_F`, `FN_800B86F0_F`, `FN_80166F40_F`,
`FN_80166F10_F`, `DAT_802EE6B8_F`, `DAT_802EE6DC_F`, `DAT_802EE6E8_F`,
`DAT_802EE6F4_F`, `S_NOKONOKO_B_BRRES_F`, `S_NOKONOKO_B_F`, `S_NET_WALK1_F`,
`TABLE_TURN_LIMIT_F`, `F32_8042B544_F`, `F32_8042B548_F`, `F32_8042B54C_F`.

Every other operand reuses an existing definition: `FN_8015F800`,
`FN_802B9280`, `DAT_80357BF4`, `FN_802E1C40`, `FN_802E1AB4`, `FN_802DCBFC`,
`FN_8006D090`, `FN_800690C0`, `FN_800690E0`, `FN_800DF180`, `FN_80239E30`,
`FN_8023A0B0`, `FN_8023A200`, `FN_80169D90`, `FN_80064BD0`, `FN_801650D0`,
`FN_80166830`, `FN_80166BD0`, `FN_80166EF0`, `FN_80169F80`, `G_SDA_5648_D`,
`DAT_80377C48`, plus the `platform::*`, `p32`, `read_f32`, `write_f32`,
`vtable_method` and `call_f32_2u` helpers. Nothing is redefined.

## Values resolved from the ELF

`r2 = 0x80433080` and `r13 = 0x8042F680` (from `crate::ui::lyt`), so
`r2 - 0x7B40 = 0x8042B540` (`s16[2] = {-32768, 0}`), `r2 - 0x7B3C = 1.0f`,
`r2 - 0x7B38 = 0.0f`, `r2 - 0x7B34 = 16.0f`; `r13 - 0x5648 = 0x8042A038`
(archive root, `+4` is the arena). `0x802EE6B8` holds the float header
`11.0, 8.0, 11.0` followed by three sub-records at `+0x24`, `+0x30`, `+0x3C`.
The three strings are `"g3d/nokonokoB.brres"` (`0x80305310`), `"nokonokoB"`
(`0x80305324`) and `"net_walk1"` (`0x80305330`).

## Method

1. Read the ten exports in
   `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c`.
2. Disassembled every body with
   `node ppc159.js work/dol/main.elf <addr>` (`STOP=0`, `LEN` in
   instructions), because the decompiler drops the `r2`/`r13`
   displacements, the `rlwinm` masks and the `xoris`/`lfd` conversion that
   the translations need.
3. Diffed the three helper bodies against the twins already in
   `src/ui/hud.rs` (`fn_8002ec10`, `fn_8002ed50`, `fn_8002ee30`) and against
   the model-binder precedents `fn_8002cd70` (`pakkun`) and, in
   `crate::game::enemies`, `fn_80b002d0` (`penguin2`).
4. Read the SDA2/SDA values and the three name strings straight out of
   `work/dol/main.elf`.
5. Appended `work/wave206/laneF_block.rs` to a throw-away copy of the crate
   and ran `cargo check --lib` until it exited 0 (no warning originates in
   the appended lines).
