# Coverage - Wave 185, lane D (`game::bases`, sub-300-byte sweep)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 10 | 944 | 8 | 2 |

All ten lane-D addresses are 68-120 bytes in
`work/ghidra/rel-d_basesNP/export/functions.tsv` and match the ten expected
rows of `work/wave185/merge.cjs` exactly, in order. Lane D added no helpers and
did not touch `src/game/bases.rs`; the block is in
`work/wave185/laneD_block.rs`, the ledger is
`ledger/agents/wave185-laned.csv`, and the bodies are appended by the
coordinator. The block was type-checked against a copy of the crate in
`%TEMP%\opencode\w185check` (clean `cargo check --lib`, no new warnings).

## Translated

- `0x8076_C090` (68) - subclass constructor on the shared base: [`fn_8076b3f0`],
  vtable `0x8081_67E8` at `+0x60`, `+0x290 = 0`, returns `this`. Structural twin
  of [`fn_8076b210`] with a different vtable word.
- `0x8076_C360` (120) - seeds the ship's cached transform: copies
  `+0xAC..+0xB4` to `+0x128..+0x130`, `+0x134 = 100.0` (`0x807CF12C`), then
  `0x8076C4E0`/`0x8076C580`/`0x8076C620` and `+0xDC..+0xE4 = 1.9`
  (`0x807CF130`); returns `1`.
- `0x8076_D280` (76) - two-query predicate:
  `0x800FC9C0() != 0 && 0x800FCA20(0) == 0` returns `1`, else `0`.
- `0x8076_D3B0` (116) - second subclass constructor: `0x800F5FD0`, vtable
  `0x8081_6B08`, `0x80069020(+0x188)`, `+0x1A4 = 0`, `0x80169CD0(+0x1A8)`,
  `0x802DC9A0(+0x1E8, 0x806F0060, 0x8064B270, 0x38, 6)`; returns `this`. Same
  layout as [`fn_8076b3f0`] but with six array elements instead of two.
- `0x8076_D620` (76) - virtual slot `+0x14` on the `+0x1A8` animation object,
  then `0x800F6920(this, 1)`; returns `1`. Mirror of `0x8076EC00`; next to the
  translated [`fn_8076d680`].
- `0x8076_E660` (84) - registry scan: `0x80162D50(0x284, previous)` until
  `(record + 4) & 0xF == kind`, returning the record or `0`.
- `0x8076_EC00` (72) - `0x800F6920(this, 1)` first, then virtual slot `+0x14` on
  the `+0x1A0` interface object; returns `1`.
- `0x8076_FE70` (116) - clears `+0x474`, seeds `+0x3CC = 0.1` (`0x80816C34`) and
  `+0x3D0 = 0.0` (`0x807CF2A4`), then `0x80106B40(&position,
  *(0x808327E0))` and copies the three floats to `+0xAC..+0xB4`.
- `0x8076_FF10` (104) - `0x801004B0(world, "shipRoute", count - 1)` from
  `0x80816D44`, then `0x801006C0(world, route, position)`.
- `0x8077_0BE0` (112) - publishes the ride: `0x808327E8 = (kind == 0 || kind
  == 2)`, `0x808327EC = this + 0xAC`, three floats to
  `0x808327F0..0x808327F8`, then tail-calls
  `0x80105440(sound, 0x24, 0x808327E8, 0x808327F0, 1, 1)`.

## Deferred

None.

## Notes and limitations

- **`0x8077_0BE0` export is truncated.** Ghidra ends it in
  `halt_baddata()`; the raw words show the body is a leaf (no frame, no
  `_savegpr`) that finishes with `b 0x80105440`, i.e. a tail call. All six
  argument registers (`r3 = *(0x8042_A27C)`, `r4 = 0x24`, `r5 = 0x808327E8`,
  `r6 = 0x808327F0`, `r7 = 1`, `r8 = 1`) are read from the disassembly, so the
  body is literal; only the meaning of the `0x24` effect id is unknown. It is
  the writer side of the globals that the already-translated [`fn_80770c50`]
  consumes.
- **`0x8076_FE70`** is `medium` because the contract of `0x80106B40` is not
  established: the raw call passes `&stack_vec3` and `*(0x808327E0)`, and the
  three stack floats are then copied to the actor. The stores and constants are
  certain; the callee's role is `// unknown:`.
- **`0x8076_FF10`** is `medium` for the same reason: the dereferences
  (`*(0x808327E0)`, `+8`, `*(0x80816D44)`, `*(0x8042_A18C)`) and the `-1` are
  literal, but the two `main.dol` callees `0x801004B0`/`0x801006C0` have no
  further contract. `0x80816D44` holds a relocated pointer to `"shipRoute"`
  (`0x80816D38`), confirmed in the ELF.
- **New callees without Rust bodies.** `0x8076C4E0`, `0x8076C580`,
  `0x8076C620`, `0x80106B40`, `0x801004B0`, `0x801006C0` and `0x80105440` are
  reached through `abi_callN!`; no helper was added. `0x8076C4E0` in turn calls
  the freshly translated `0x8076D280`.
- **Constants re-read from the ELF:** `0x807CF12C = 100.0`,
  `0x807CF130 = 1.9`, `0x807CF2A4 = 0.0`, `0x80816C34 = 0.1`,
  `0x80816D44 -> "shipRoute"` at `0x80816D38`.
- **Pre-existing exports that are not bodies.** `0x8076FEF0` (24 bytes, next to
  `0x8076FE70`) is pure `halt_baddata()` in this export and is not one of this
  lane's addresses.
