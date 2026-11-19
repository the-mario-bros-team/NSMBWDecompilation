# Coverage - Wave 176 lane C (`game::bases`, 420/424-byte classes)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane C block) | `game::bases` | 5 | 2,108 | 2 | 3 |

Five bodies: three 420-byte bodies and two 424-byte bodies. Source of the
bodies: `work/wave176/laneC_block.rs`; per-function ledger:
`ledger/agents/wave176-lanec.csv`.

## What this pass covers

- **`cobDokanRoute` model loader (1 body).** `0x80762970` is the sibling of
  [`fn_8075a9a0`] (`cobAnchor`) and of the `remo_lift_soroban` loader
  [`fn_80728cc0`]: it creates the `0x800690C0` sub-object at `+0x18C`, loads
  `g3d/model.brres` (`0x80815540`) through `0x800DF180`, binds the
  `cobDokanRoute` resource (`0x80815550`) at `+0x1AC` (size `0x224`), then
  builds the model clip at `+0x1EC` and the animation clip at `+0x224` from the
  `0x80815538` pane-name slot, seeds both, and dispatches the `+0x1AC` vtable
  slot `+0x18` twice before `0x800F2A10`.
- **Scene-list armed scan (1 body).** `0x80774DF0` is the second half of the
  `0x80774BD0` pair: the caller stops at the first element for which
  `0x802E1AB4(needle, element)` returns `0`, then this body re-walks the
  `+0x3390` scene blocks (`0x16518` stride from `+0x3394`), both sub-tables of
  all `0xC0` slots, and answers `1` for the first matching slot whose
  `0x800F9530` id is `-1`, the `0x8042CF84` kind, or accepted by
  `0x800FAD60`/`0x800FADB0`.
- **Course-actor placement (1 body).** `0x807980F0` is the reset/placement
  routine of the same course-actor class as [`fn_8079a930`] and
  [`fn_8079ac60`]: it copies the position into `+0xAC`, walks the three
  `0x807D0198`/`0x9C`/`0xA0` masks through the `+0x29C` ability object
  (`0x80C56B90`, `0x80C56600`, `0x80C560B0`) and the `0x800F57A0` route gate,
  mirrors the position to `+0xB8`, writes `-1` to `+0x234`, the two incoming
  floats to `+0xEC`/`+0x10C`, and finishes with `0x80799960(this, pose, -1)` and
  `0x800FDC80`. Its four `0x80797D30` call sites all pass `(0.0, 0.0, this,
  position, 1, 1, 0)`.
- **Per-player slot flag + level rebuild (1 body).** `0x80680CD0` zeroes
  `+0x372C` and `+0x3734..+0x374B`, walks the eight `0x74`-stride `+0x3E4`
  chains and, for every kind-`1`/`2` element (`+0x38C`), reads the actor vtable
  slot `+0x6C` (via `+0x60`) to obtain a signed slot index and sets the matching
  `+0x3744` byte. It then re-seeds the four `+0x3734 + i*4` levels from the
  signed halfword `+0x3732` with `0.5 * (±(400.0 * f(1/256 * (u16)angle)))`
  (`f = 0x80237BD0`, sign from `+0x348`), sets `+0x372C = 1` per seeded slot and
  copies `+0x3744..+0x3747` to `+0x3748..+0x374B`.
- **`remo_lift_soroban` model bind (1 body).** `0x80728CC0` completes the
  already-translated pair [`fn_80728bf0`] (constructor) and [`fn_80729490`]
  (`change_mark_A` bind): it creates `+0x3D0`, loads
  `g3d/remo_lift_soroban.brres` (`0x8080B57C`), binds `remo_lift_soroban`
  (`0x8080B598`) at `+0x3F0` (size `0x23`), binds `change_mark_A` into the
  `+0x430` clip, `remo_mark` (`0x8080B5AC`) into the `+0x468` follow clip,
  dispatches the `+0x3F0` slot `+0x18`, seeds `0x80166EF0` with the incoming
  `f1` and releases `+0x3D0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80762970` | `fn_80762970` | 420 | high |
| `0x80774DF0` | `fn_80774df0` | 420 | medium |
| `0x807980F0` | `fn_807980f0` | 420 | medium |
| `0x80680CD0` | `fn_80680cd0` | 424 | medium |
| `0x80728CC0` | `fn_80728cc0` | 424 | high |

## Deferred

None.

## Corrections and notes

- **`_savegpr` returns are the incoming register file, not 64-bit values.**
  `0x80762970` (`0x802DCD6C`), `0x80774DF0` (`0x802DCD6C`), `0x807980F0`
  (`0x802DCD5C`), `0x80680CD0` (`0x802DCD70`) and `0x80728CC0` (`0x802DCD74`) all
  begin with a save helper. Ghidra therefore lists no parameters; the raw
  words show `r3` as `this` plus, where applicable, `r4`/`r5`/`r6`/`r7`.
  `0x80774DF0`'s and `0x807980F0`'s five integer registers are recovered from
  the `mr` sequence (`r23..r27`), exactly as the `0x80797D30` call sites print.
- **`0x80774DF0`'s `0x800F9530` argument.** The raw body is
  `bl 0x800F56B0` immediately followed by `bl 0x800F9530`; `r3` survives from
  the first call, so the id reader takes the slot record as its argument, as in
  [`fn_8079a930`] and `0x807BAF50`. Ghidra prints `func_0x800f9530()` with no
  argument.
- **`0x80762970`'s `0x807CEE58` is the word `1`, not a byte constant.**
  `lwz r0, 0(r27)` / `stb r0, 0x220(r25)` stores the low byte (`1`), while the
  same word is passed whole to `0x80167B60`; the Rust reads it once. The four
  `lfs` `0x807CEE5C` loads are `0.0`.
- **`0x80680CD0`'s `GQR3` sample.** The body does
  `lha r0,+0x3732` / `sth r0,8(r1)` / `psq_l f0,0(r28),1,3`, so the value is the
  unsigned 16-bit pattern of the halfword (brief fact: `GQR3 = 0x00050005`,
  scale 0), not the signed halfword. The `unaff_GQR0`/`ldexpf` brackets around
  the prologue and epilogue are the `f29..f31` save/restore pairs.
- **`0x807980F0`'s two `unaff_GQR0` brackets are `f30`/`f31` spills.** The
  prologue is `stfd f31` / `psq_st f31,qr0,0` / `stfd f30` / `psq_st f30,qr0,0`
  and the parameters arrive in `f1` (`fmr f30,f1`) and `f2` (`fmr f31,f2`), which
  are the two doubles Ghidra prints as `param_1`/`param_2`.
- **`0x807980F0`'s `+0x234` is `-1`, not `0`.** `li r0,-1` / `stw r0,0x234(r23)`
  is at `0x80798218`-`0x8079822C`; the export prints it correctly.
- **Evidence path spelling.** The merged copy of this block inside
  `src/game/bases.rs` (written by the coordinator before this lane's final
  hyphen fix) spells the evidence directory `work/ghidra/rel_d_basesNP/`, the
  same form 15 pre-existing entries in that file use. The canonical directory
  and the lane CSV use `work/ghidra/rel-d_basesNP/`. No body text differs.

## Verification

- `work/wave176/laneC_block.rs` type-checks standalone against stub helpers
  with the real `abi_callN!` macro bodies, `call_f32_u32`, `call_f32_u32x2`,
  `call_f32_f32`, `vmethod1` and `actor_method0` (temporary `rustc` harness
  outside the repo).
- `cargo check --lib` on the merged tree reports no errors.
- `node tools/doc-lint.mjs` reports no per-function violations for the block
  (only the expected `module-doc` note, since a lane block has no `//!` header).
- All five evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`; every constant, branch and field offset
  was re-read from `work/rels/elf/d_basesNP.elf`.
- Only helpers that already exist in `src/game/bases.rs` are used
  (`vmethod1`, `actor_method0`, `call_f32_u32`, `call_f32_u32x2`,
  `call_f32_f32`, `platform::read_*`/`write_*`, the `abi_callN!` macros).
  `0x800690C0`, `0x800DF180`, `0x80239E30`, `0x80169D90`, `0x80064BF0`,
  `0x8023A0B0`, `0x8023A270`, `0x8023A200`, `0x801650D0`, `0x801651F0`,
  `0x80169F80`, `0x801680E0`, `0x80168FE0`, `0x80166830`, `0x80166BD0`,
  `0x80167420`, `0x80167B60`, `0x80167B40`, `0x80167B20`, `0x80166EF0`,
  `0x800F2A10`, `0x800690E0`, `0x800E0380`, `0x800F5770`, `0x800F5730`,
  `0x800F56B0`, `0x800F9530`, `0x800FAD60`, `0x800FADB0`, `0x802E1AB4`,
  `0x80C56B90`, `0x80C56600`, `0x80C560B0`, `0x800F57A0`, `0x80799960`,
  `0x800FDC80` and `0x80237BD0` have no Rust body and go through those
  wrappers.
- `src/game/bases.rs` was not modified by this lane.
