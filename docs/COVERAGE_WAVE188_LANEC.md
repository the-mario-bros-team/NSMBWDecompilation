# Coverage - Wave 188, lane C (`game::bases`, 128-192-byte band)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 10 | 1508 | 3 | 7 |

All ten lane-C addresses are 132-188 bytes in
`work/ghidra/rel-d_basesNP/export/functions.tsv`. Lane C added no helpers and
did not touch `src/game/bases.rs`; the block is in
`work/wave188/laneC_block.rs`, the ledger is `ledger/agents/wave188-lanec.csv`,
and the bodies are appended by the coordinator through `work/wave188/merge.cjs`.

## Translated

- `0x806AE010` (136 bytes) - trimmed name compare: advances the argument past
  its first `:` with `0x802E1C40`, fetches the actor name through vtable slot
  `+0x20`, trims that too, and returns whether `0x802E1AB4` reports equality.
- `0x806AF090` (136 bytes) - the same trimmed name compare; the raw words match
  `0x806AE010` instruction for instruction apart from the call displacements.
- `0x806B9800` (132 bytes) - aim pass: resolves the player named by `+0x484`
  through `0x8005F900`, reads its position twice with `0x8006CED0`, and passes
  the `+0xAC`/`+0xB0` differences to the facing helper `0x801611C0` (result
  discarded).
- `0x806BDAC0` (148 bytes) - binds the `jump_st` clip on `+0x600` into the
  animation at `+0x644` (`0x8023A0B0`/`0x801651F0`), applies motion time `5.0`
  (`0x80169F80`) and rate `1.0` (`0x801680E0`), then writes `+0x360 = 1` and
  `+0x114 = -0.185`.
- `0x806C7120` (136 bytes) - trimmed name compare clone of `0x806AE010`.
- `0x806C7240` (136 bytes) - trimmed name compare clone of `0x806AE010`.
- `0x806C9600` (136 bytes) - matrix build under a lock: `0x800690C0` takes
  `+0x3DC`, `0x800C0010` composes `+0x3F8` from the three incoming floats, and
  `0x800690E0` releases the lock.
- `0x806D02A0` (176 bytes) - vertical-window clamp: reads the scene pointer at
  `0x80429E00`, takes the ceiling `+0x3C` and window `+0x44`, keeps `+0xB0`
  inside the window while stepping by `step`, and caps it against the target
  height from [`fn_806d0230`].
- `0x806D0350` (188 bytes) - lift-height phase: `0x806D0130` returns the lowest
  player height, `- 16.0` (Ghidra `0x807CB468`), clamped against the lower bound
  `+0x5F4`; returns `2` when that clamp fired, `1` when it did not and the
  [`fn_806d0230`] target capped the value, `0` when no player was found.
- `0x806D08D0` (184 bytes) - rider pass: fills the report with `0x800441E0`,
  counts riders with [`fn_806cfb30`], adopts the reported slot byte at `+0x430`,
  and when the count reaches `+0x65E` with a valid slot plays
  [`fn_806d06a0`] and switches the `+0x394` interface to `0x8082D550`; always
  ends with [`fn_806d0780`] and `fn_806d0510(this, 4, 0xC0)`.

## Deferred

None.

## Notes and limitations

- `0x806AE010`/`0x806AF090`/`0x806C7120`/`0x806C7240` are `medium`: Ghidra's
  `func_0x802e1c40` and `func_0x802e1ab4` are identified as `strchr`/`strcmp`
  only from their argument shapes, exactly as in the already-merged
  [`fn_80672f00`]. The four bodies were checked word-for-word (34 words each)
  and differ only in the three `bl` displacements.
- `0x806BDAC0` constants are the relocated Ghidra symbols, not the raw `lis`
  immediates: `0x807C_ADCC` = 5.0, `0x807C_AD80` = 1.0, `0x807C_AD60` = 0.185
  (read back from the ELF at the Ghidra addresses). The clip name is the
  literal `"jump_st"` at `0x807F_69C8`.
- `0x806B9800` is `medium`: the two `0x8006CED0` position reads and the
  `0x801611C0` argument order are literal (`f1` = second vector `[1]`, `f2` =
  first vector `[0]`), but the vector roles are taken from the established
  `0x8006CED0` usage in [`fn_806c9b70`], not from a resolved callee.
- `0x806C9600` is `medium`: `0x800690C0`/`0x800690E0` are emitted with the same
  five-argument lock form already used by [`fn_806f9e20`]; the exported C names
  the helper as `func_0x800690c0` with an unresolved contract.
- `0x806D02A0` dereferences `0x8042_9E00` (main.dol scene pointer) before
  reading `+0x3C`/`+0x44`, matching the raw `lwz r4,0xffff9e00(r4)` /
  `lfs ...,0x3c(r4)`. `0x806D0130` has no Rust body yet, so `0x806D0350`
  calls it through `call0_f32`.
- The `fcmpo` + `bc 4,0`/`bc 4,1` + `cror` folds in `0x806D02A0` and
  `0x806D0350` were re-read from the raw words and follow Ghidra's `<`/`<=`
  directions; the `0x806D03CC` `cror` encodes the `target <= current` cap.
