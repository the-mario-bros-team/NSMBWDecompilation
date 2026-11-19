# Coverage - wave 186 lane E (`game::bases`, `d_basesNP`)

## How to read this file

This is the lane-E slice of the wave-186 sub-300 sweep: ten small bodies from
the PowerPC REL `d_basesNP`, spanning `0x807C13A0` to `0x807C31F0`. Every new
body lives in `work/wave186/laneE_block.rs` and is appended to
`src/game/bases.rs` by the coordinator; lane E did not edit `src/game/bases.rs`,
`lib.rs`, any `mod.rs`, `Cargo.toml`, `src/platform.rs`, another lane's files,
`ledger/functions.csv` or any generated document.

- **address** - REL virtual address; the synthetic analysis base is
  `0x80600000`, so these are not runtime addresses.
- **name** - the Rust function name that will exist in `src/game/bases.rs`.
- **role** - one plain sentence.
- **confidence** - `high` when every load/store/branch/constant was read from
  the raw words; `medium` when a callee's role is inferred because it is named
  by address only.

All ten entries were translated; nothing was deferred.

The ten bodies all sit inside the `dCourseSelectManager_c` state machine whose
thirty-five state descriptors are registered by the already-translated
[`fn_807c3d70`] (`0x807C3D70`).

## Summary

| Metric | Count |
|---|---:|
| Bodies translated | 10 |
| Bytes of original code | 972 |
| `translated` | 10 |
| Deferred | 0 |

## Bodies

| Address | Size | Name | Role | Confidence |
|---|---:|---|---|---|
| `0x807C13A0` | 112 | `fn_807c13a0` | `1` when the `0x807C0A80` scene probe finds nothing, or the `0x8042A24D` world byte is `3`/`5` (`0x807D0B40`/`+4`). | high |
| `0x807C1410` | 96 | `fn_807c1410` | `1` when the `0x8042A24D` world byte is `0`/`2`/`3`/`4` (`0x807D0B48..+0xC`). | high |
| `0x807C1B20` | 96 | `fn_807c1b20` | Destructor: `0x8015F9F0(object + 0x18, 0)` then `0x802B9280(object)` when `free > 0`. | high |
| `0x807C1B80` | 100 | `fn_807c1b80` | Byte-twin of `0x807C1B20` with one redundant `beq`; same destructor. | high |
| `0x807C1FE0` | 100 | `fn_807c1fe0` | Clears `+0x53E`, closes both state halves, releases the audio driver, then requests `StateID_KeyWait`. | high |
| `0x807C2070` | 92 | `fn_807c2070` | Latches `0x807BB0C0()` (always 0) at `*(0x80429F7C) + 0xAFD`, then requests `StateID_KeyWait`. | high |
| `0x807C2710` | 88 | `fn_807c2710` | Plays cue `0x79`, restarts the first state half, sets `+0x8DD` on `*(this + 0xB4)`. | high |
| `0x807C2B70` | 108 | `fn_807c2b70` | If `*(*(this + 0x530) + 0x295) == 0`: mode `1` closes the state, otherwise requests `StateID_NowSave`. | high |
| `0x807C2DB0` | 68 | `fn_807c2db0` | Runs `0x800FD1F0(this)` and seeds the `+0x530` controller (`+0x28C = 15`, `+0x295 = 1`). | high |
| `0x807C31F0` | 112 | `fn_807c31f0` | Arms the `+0xC0` controller (`+0x2B3`, `+0x229`, `+0x27C = 0x807714F0()`), restarts the state half, plays cue `0x79`. | high |

## Deferred

None.

## Notes and corrections

- `0x807C13A0` / `0x807C1410`: Ghidra declares both `void` with no parameters,
  but the shipped caller `fn_807bffa0` (`0x807BFFA0`) passes the manager in
  `r3` and `0x807C13A0` forwards that same `r3` into `0x807C0A80`, whose own
  export is `FUN_807c0a80(undefined4 param_1)`. Both Rust bodies therefore take
  a `usize`; `0x807C1410` never reads it, so it is `_this`.
- `0x807C13A0` / `0x807C1410`: the two comparison tables were read out of the
  REL: `0x807D0B40 = 3`, `0x807D0B44 = 5`, `0x807D0B48 = 0`, `0x807D0B4C = 2`,
  `0x807D0B50 = 3`, `0x807D0B54 = 4`. The source operand is a single byte
  (`lbz 0x8042A24D`), so the Rust compares `read_u8(0x8042A24D) as u32`
  against the zero-extended `read_u32` words.
- `0x807C1B20` / `0x807C1B80`: the same object/member destructor pair already
  translated for `0x80777990`/`0x807779F0`. The extra `41820014` in the second
  body re-tests the outer `object != 0` guard, so behaviour is identical. The
  `addic. r3,r3,24` member test is kept verbatim; the callee is `0x8015F9F0`
  (not `0x8015F800`), which has no Rust body, so it is called by address.
- `0x807C1FE0` / `0x807C2070` / `0x807C2B70`: the indirect dispatch is
  `lwz r12,0x70(this)` then `lwz r12,0x18(r12)` and `bcctr` with `r3 = this +
  0x70`, i.e. `abi_call2!(slot, this + 0x70, descriptor)`. The descriptors are
  the ones `fn_807c3d70` registers: `0x80834540` is
  `dCourseSelectManager_c::StateID_KeyWait` and `0x80834780` is
  `dCourseSelectManager_c::StateID_NowSave`. Both live outside the REL image
  (`0x80834540` maps to file offset `0x235540`, past the `0x2238B8`-byte file),
  so they are emitted as literal pointers, as in `fn_807c3d70`.
- `0x807C1FE0`: the `0x8019CC70(*(0x8042A4A8))` call is the same audio-driver
  release already used by `fn_807a9970`. `0x807C3A10`/`0x807C3A90` have Rust
  bodies, so they are called directly instead of through `abi_callN!`.
- `0x807C2070`: `0x807BB0C0` is the already-translated `return 0` stub, so the
  latch always stores zero; the call is kept because the original `bl` exists.
  The target is the global at `0x80429F7C` (`lwz -0x6084(r4)`), not `this`.
- `0x807C2B70`: Ghidra prints `FUN_807c1fe0()` with no argument, but `r3` still
  holds `this` at `0x807C2BA0` (only `r4` was reloaded), so the Rust body passes
  `this`. The `lwzu r12,0x70(r3)` at `0x807C2BB0` pre-increments `r3` to
  `this + 0x70`, confirming the receiver of the `StateID_NowSave` request.
- `0x807C2DB0`: identical shape to the already-translated `fn_807c2970` with
  `+0x28C = 15` instead of `1`; the `+0x530` member is reloaded twice in the C
  and the Rust keeps one reload per store group.
- `0x807C31F0`: `rlwinm r0,r3,0,24,31` masks the `0x807714F0` result to its low
  byte before the `+0x27C` word store; `0x807714F0` already returns `u8` in
  Rust, so the mask is redundant but kept for fidelity. The `+0xC0` member is
  reloaded after the call, hence the second `read_u32(this + 0xC0)`.

## Verification

The block was type-checked with `rustc --edition 2021 --emit=metadata` against
local stubs for `platform` and the `abi_call1..3!` macros plus the existing
`fn_807c3a10`, `fn_807c3a70`, `fn_807c3a90`, `fn_807c3af0`, `fn_807714f0` and
`fn_807bb0c0` bodies; it compiles with no errors or warnings. The stub file and
its metadata were deleted afterwards.
