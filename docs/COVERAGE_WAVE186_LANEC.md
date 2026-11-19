# Coverage - Wave 186, lane C (`game::bases`, sub-300-byte sweep)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 10 | 1096 | 10 | 0 |

All ten lane-C addresses are 84-128 bytes in
`work/ghidra/rel-d_basesNP/export/functions.tsv`. Lane C added no helpers and
did not touch `src/game/bases.rs`; the block is in
`work/wave186/laneC_block.rs`, the ledger is
`ledger/agents/wave186-lanec.csv`, and the bodies are appended by the
coordinator.

## Translated

- `0x807A5CC0` (128 bytes) - hint-screen timer tick: decrements `+0x3FC`, and
  at zero plays `0x8010_2260(8, 3, 2, 1)`, sets the exit flag `+0x40C` and
  forwards descriptor `0x8083_3360` to the `+0x394` view's slot `+0x18`.
- `0x807AB900` (112 bytes) - screen-off teardown: clears `screen + 0xAFD`, and
  when `0x8042_9F80 & 0x10` zeroes `+0x3E4`/`+0x3E8`, releases the scene with
  `0x800B_0DB0(0)`, calls [`fn_807aae80`] and requests sound `(0xB, 0, 0)`.
- `0x807AB970` (84 bytes) - save-slot submit: sign-extends the low byte of
  `[[this+0xE8]+0x2A4]` into `0x800E_03B0(manager, index)`, then runs
  `0x800C_DED0()`, `0x800E_04B0(manager)` and `0x800E_11D0(manager)`.
- `0x807ABAB0` (92 bytes) - if `0x8019_6600([0x8042_A488], 0x323)` returns
  zero, pushes descriptor `0x8083_3860` through the `+0x74` view's `+0x18`.
- `0x807ABB10` (116 bytes) - exit-sound select: `0x800E_0380(manager, -1)`
  supplies a record whose `+2` bit 0 picks sound `(3, handle, 0)` or
  `(8, 0, 0)` after `0x800B_0DB0(0)`.
- `0x807ABEE0` (84 bytes) - binds material key `(0x24, 8, 2, 1, 1)` on
  `0x8042_AE10` and tail-calls `0x807A_BE20(this, 1)` (no Rust body yet).
- `0x807AD210` (100 bytes) - deleting destructor: `0x8015_F9F0(this + 0x18, 0)`
  and, when `free > 0`, `0x802B_9280(this)`; returns `this`.
- `0x807ADB90` (84 bytes) - the `Wm_mr_` twin of `0x807ABEE0`: same material key,
  continuation `0x807A_DAD0(this, 1)` (no Rust body yet).
- `0x807AE8E0` (96 bytes) - deleting destructor, byte-identical to the
  [`fn_806357e0`] template.
- `0x807AE940` (100 bytes) - deleting destructor, byte-identical to
  `0x807AD210` (including the duplicated null-test `beq`).

## Deferred

None.

## Notes and limitations

- `0x807AB900`'s `r3` is only forwarded to `FUN_807AAE80`, whose export is
  `void FUN_807aae80(void)`. The already-translated [`fn_807aae80`] takes no
  arguments, so the body is called with none and the parameter is named
  `_this`.
- `0x807AB970`'s `0x7C040774` is not a branch: it decodes as `extsb r4,r0`
  (opcode 31, XO 954), which matches Ghidra's `(int)(char)` narrowing of the
  `+0x2A4` word. The Rust spells it `as u8 as i8 as i32 as u32`.
- `0x807ABAB0` uses `vmethod1(this + 0x74, 0x18, ...)`, the same form already
  used for this exact descriptor (`0x8083_3860`) in [`fn_807ab2d0`]'s
  neighbour. `0x807A5CC0`'s view sits at `+0x394`, so per the lane rules the
  slot address is loaded explicitly and called with `abi_call2!`
  (`ptr_at(vtable + 0x18)`), matching the existing `+0x394` call sites.
- `0x807AD210`/`0x807AE940` repeat the `r3 != 0` test (`41820028` then
  `41820014` on the same `cr0`) before `addic. r3,r3,24`; the repeat is
  compiler output and folds into the single `state != 0` guard.
- `0x807ABEE0` and `0x807ADB90` funnel into `0x807ABE20` and `0x807ADAD0`
  (both 188 bytes, neither translated), so they are emitted as `abi_call2!`
  with the observed `(this, 1)` arguments. The material key words are literal;
  their meaning is marked `// inferred:` from [`fn_807b5bf0`].
- Callee roles for `0x800E_0380`/`0x800E_03B0`/`0x800E_04B0`/`0x800E_1E60`
  follow the file-select translations (`fn_80618330`, `fn_807b4230`); the
  argument values themselves are read straight from the raw words.
