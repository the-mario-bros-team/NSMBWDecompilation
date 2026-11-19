# Coverage: wave 222 lane F (`ui::hud`, `0x80026CA0` and `0x8002D260`)

## How to read this file

This document covers lane F of wave 222 - the two `ui::hud` stragglers left
behind by the per-file queue. Both bodies live in `src/ui/hud.rs`; no source
file was edited by this pass. The bodies are staged in
`work/wave222/laneF_block.rs` for the merge, and the two rows are in
`ledger/agents/wave222-laneF.csv`.

Each row describes one Ghidra entry point:

- **address** - virtual address of the entry in `main.dol`;
- **name** - the Rust function appended to `src/ui/hud.rs`;
- **role** - one plain sentence;
- **status** - `translated` means the whole recorded control flow is present,
  with calls outside this batch left as absolute-address calls;
- **confidence** - `high` when every load, store, branch and argument was read
  directly from the disassembly; `medium` when a role is inferred;
- **bytes** - the `functions.tsv` size (see the size note below);
- **evidence** - the Ghidra C file under `work/ghidra/main-gb/export/`.

## Summary

| metric | value |
|---|---:|
| functions translated in this pass | 2 |
| text bytes represented | 440 |
| status `translated` | 2 |
| status `partial` | 0 |
| status `deferred` | 0 |
| confidence `high` | 2 |
| confidence `medium` | 0 |
| deferrals | 0 |

Byte total: `228 + 212 = 440`.

## Function table

| address | Rust function | role | status | conf. | bytes | evidence |
|---|---|---|---:|---:|---:|---|
| `0x80026CA0` | `fn_80026ca0` | owner-query gate, then spawn `Wm_en_hit` at the contact point or latch the child | translated | high | 228 | `work/ghidra/main-gb/export/80026ca0_FUN_80026ca0.c` |
| `0x8002D260` | `fn_8002d260` | finish the burst-sprite transform: five animation-slot binds on `+0x544` | translated | high | 212 | `work/ghidra/main-gb/export/8002d260_FUN_8002d260.c` |

## Size verification

Both `functions.tsv` sizes are exact, not artefacts:

- `0x80026CA0`: prologue `stwu r1,-0x30(r1)` (frame `0x30`, four callee-saved
  registers plus the link word), the only `blr` is at `0x80026D80`, i.e.
  `0xE4 = 228` bytes, and the next entry starts at `0x80026D90` after four
  words of padding.
- `0x8002D260`: prologue `stwu r1,-0x70(r1)`, the only `blr` is at
  `0x8002D330`, i.e. `0xD4 = 212` bytes, and the next entry starts at
  `0x8002D340` after four words of padding.

Neither body has a `psq_st` prologue and neither is a jump-table head.

## What the two bodies are

Both are members of families that `src/ui/hud.rs` already holds, and both were
missed only because the file-level queue stopped counting at 540 bodies.

### `0x80026CA0` - the actor hit gate

This is an actor-band body, not a layout body. Its sole caller is at
`0x80039404` inside the `0x800393A0` hit handler, which passes
`this` (`r28`), the collision record (`r29`) and `child = *(contact + 4)`
(`r27`) - the same three registers, in the same order. The two sibling calls
around it are `0x80064A30` (side compare) and `0x80026C00` ([`fn_80026c00`],
the digit select), so this body belongs to the run of `0x80026B50..0x800272C0`
that wave 204 lane C already translated.

The shape is a two-way gate:

1. unless `+0x524` (this actor's latch) is non-zero, the owner object at
   `+0x394` is asked twice - virtual `+0x28`, then virtual `+0x14` with the
   shipped descriptor `0x80352DB4` ([`DAT_80352DB4`], the same descriptor
   [`fn_80026c00`] uses). The `neg`/`or`/`rlwinm.` triple in the original is
   the compiler's `!= 0` test, so a non-zero answer clears the spawn flag;
2. on spawn, `0x800954D0(this, {contact.x, contact.y, 5500.0})` runs. That
   callee is the `"Wm_en_hit"` spawner already documented by [`fn_80031b70`]
   and [`fn_80033f10`] in this module, and the `+0x90`/`+0x94` pair is the same
   aim point [`fn_800396a0`] publishes;
3. off spawn, [`fn_80026bc0`] (wave 204 lane C) tests the child, and an open
   gate sets the child's own `+0x524` byte to `1`.

`// unknown:` the meaning of the two `+0x524` latches and of the owner query is
not recovered. What is certain is the polarity: an already-set latch *skips*
the query and still spawns, and the child latch is only written when the query
refused. That is what the shipped instructions do, and the reconstruction keeps
it rather than "fixing" it.

The vector is built twice: `0x8(r1)`/`0xC(r1)` receive the first two components
and nothing ever reads them, while `0x10(r1)` receives the triple that is
actually passed. The dead pair is omitted (same treatment as the identical dead
copy in [`fn_80031b70`]).

### `0x8002D260` - the five slot binds

This is the tail of [`fn_8002d0c0`], the burst-sprite transform of the
`0x8002D0C0` body: `0x8002D0C0` binds `+0x544` to `+0x7C`/`+0xDC`, resets it
and then calls `0x8002D260(this)` at `0x8002D234`, which the earlier pass left
as the by-address constant `FN_8002D260_D` with the comment "translated next
door". This body is that neighbour.

It runs one animation reset (`0x80168400`), then five slot binds around one
shared `0x30`-byte scratch matrix:

| step | call | destination |
|---|---|---|
| 1 | `0x80168180(+0x544, 7, &node)` then `0x8016F130` | `+0x5C4` |
| 2 | `0x80168180(+0x544, 6, &node)` then `0x8016F130` | `+0x5D0` |
| 3 | `0x801C0BD0(0.0, 6.0, 0.0, &offset)`, `0x801C0540(&node, &offset, &node)`, `0x8016F130` | `+0x5F4` |
| 4 | plain triple copy of `+0x5F4..+0x5FC` | `+0x5DC`/`+0x5E0`/`+0x5E4`, `z - 48.0` |
| 5 | `0x80168180(+0x544, 5, &node)` then `0x8016F130` | `+0x5E8` |

The five outputs are 12-byte neighbours (`+0x5C4`, `+0x5D0`, `+0x5DC`,
`+0x5E8`, `+0x5F4`), so they are five three-float records of one pose block;
step 4 is not a matrix operation but a `fsubs` on the third component only.
The pattern is the one [`fn_8002f730`] uses for nodes `6`/`8` and
[`fn_80033650`] uses for its named slots, so all four helpers are reused rather
than re-declared.

## Reuse

The pass adds three constants and no helpers:

| constant | address | value | why it is new |
|---|---|---|---|
| `F32_SDA2_7D38_F222` | `0x8042B348` | `5500.0` | the body loads its own slot; the existing `5500.0` constants sit at `-0x7D00`, `-0x7A98` and `-0x7BA4` |
| `F32_SDA2_6_7C64_F222` | `0x8042B41C` | `6.0` | third member of the `0.0/16.0/8.0/6.0/48.0` block, not previously named |
| `F32_SDA2_48_7C60_F222` | `0x8042B420` | `48.0` | fourth member of that block, not previously named |

Everything else exists already: `DAT_80352DB4`, `FN_800954D0_B`,
`F32_SDA2_ZERO_7C70_D`, `FN_80168400_D`, `FN_80168180`, `FN_8016F130_D`,
`FN_801C0BD0`, `FN_801C0540`, `call_3f_1u`, `p32`, `vtable_method`, `read_f32`,
`write_f32`, and the local body `fn_80026bc0`. No existing constant was
redefined and no existing helper was shadowed.

## Verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to a scratch directory, the
block was appended to the copy's `src/ui/hud.rs`, and `cargo check --lib`
finished with exit code 0 (26 warnings, none from the appended lines).
