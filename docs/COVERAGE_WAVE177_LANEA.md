# Coverage - wave 177 lane A (`game::bases`)

## Scope

Five `d_basesNP` bodies appended to `src/game/bases.rs` by the coordinator from
`work/wave177/laneA_block.rs`. Addresses are synthetic (`base 0x80600000`) and
are **not** runtime addresses; main.dol cells (`0x8031xxxx`, `0x8037xxxx`,
`0x8042xxxx`, `0x807Cxxxx`) are imported absolute values.

| Metric | Lane A |
|---|---:|
| Translated bodies | 5 |
| Deferred bodies | 0 |
| Selected bytes | 2,192 |
| Confidence | 5 high, 0 medium |

## Translated functions

| Address | Bytes | Rust | Role | Confidence | Evidence |
|---|---:|---|---|---|---|
| `0x806F2E50` | 436 | `fn_806f2e50` | allocate `0xBEC0` and construct the vtable-`0x80801210` actor with five inline arrays | high | `806f2e50_FUN_806f2e50.c` |
| `0x806FE860` | 436 | `fn_806fe860` | four-state gate: reset players, post cues `0x5F2`/`0x5F3`/`0x5DA`/`0x5DB`, arm `0x80102260` | high | `806fe860_FUN_806fe860.c` |
| `0x806E2950` | 440 | `fn_806e2950` | pick the `lift_suisha` plank angle from the four occupancy slots | high | `806e2950_FUN_806e2950.c` |
| `0x8075B040` | 440 | `fn_8075b040` | load `cobAntlion` (`g3d/model.brres`) and build its three bindings | high | `8075b040_FUN_8075b040.c` |
| `0x8079B3F0` | 440 | `fn_8079b3f0` | seed position/rotation limits, the `+0x2E0` animation and the spawn effect | high | `8079b3f0_FUN_8079b3f0.c` |

## Reused neighbours

- `fn_806e2950` calls the already-translated `fn_806e27d0`, `fn_806e2830` and
  `fn_806e2890` directly (same module, identical calling convention).
- `fn_806fe860` follows `fn_806fe480` exactly: `0x806FD7A0` first, then the
  `+0x21C` match, with `fn_806fdc50` reused for the state-1 probe.
- `fn_806f2e50`, `fn_8075b040` and `fn_8079b3f0` reuse only existing helpers
  (`abi_callN!`, `call0_ret`, `call_f32_u32`, `call_f32_u32x2`, `vmethod0/1`,
  `platform::{read,write}_*`).

## Deferred

None. All five bodies were resolved from the raw words; no body was omitted.

## Decisions and preserved unknowns

1. **`0x800CD710` base initialiser** (`fn_806f2e50`) is printed by Ghidra without
   arguments. `r3` still holds the fresh allocation at the call, so the Rust
   passes `this`, following the documented base-constructor convention.
2. **`+0x158..+0x1F4` clear** is GCC loop-versioning (two overlapping `CTR`
   word loops, one 8-word and one 1-word). It is reproduced as the equivalent
   `u32` fill; no observable difference is provable from the machine code.
3. **`fn_806e2950` save/restore thunks.** `0x802DCD5C`/`0x802DCDA8` are the
   `_savegpr`/`_restgpr` helpers and are not emitted. Their `u64` result in the
   export is the spilled register frame, not a value.
4. **`(-x | x) < 0`** in `fn_806fe860` is the `neg`/`or`/`rlwinm` sign-bit idiom,
   i.e. `fn_806fdc50(this) != 0`.
5. **`0x8031D39C` transform block** (`fn_8079b3f0`) lives in main.dol, so the
   three words at `+4`/`+0xC`/`+0x10` are read at runtime rather than inlined.
6. **Pointer words** `0x808143DC` (`cobAntlionAppear`) and `0x808143EC`
   (`cobAntlion`) are read as pointer words, matching the `lwz` at both call
   sites and the `PTR_s_*` form in the export.

## Verification

`laneA_block.rs` type-checks against the module's helper signatures
(`rustc --emit=metadata` on a stub harness, 0 warnings).
