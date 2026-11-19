# Coverage - Wave 172 lane B (`game::bases` 336-340 byte sub-band)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 5 | 1,688 | 5 | 0 |

Lane B of the 336-340 byte sub-band. All five bodies are translated; nothing is
deferred.

## What this pass covers

- **Lane height test (1 body).** `0x806843B0` looks the lane byte up in the
  `0x8042C1A0` table, scales it by `0.5 * 32.0`, asks `0x80072DB0` for the two
  lane offsets from the `+0x1EC` transform, shifts this actor's `+0xAC` height
  by the returned pair (the `+0x348` mirror byte picks the sign) and returns
  `lane * (height - other_height - lane * 16.0) < 0.0`.
- **Model rebuild plus frame byte (1 body).** `0x80719250` is the third member
  of the `0x806B1970`/`0x806BFD50` family: position `+0xAC`, three `s16` angles,
  `0x80064AF0`/`0x801C0BD0` matrix, the three rotation setters, `0x8016A170`/
  `0x8016A1A0` on the `+0x544` holder, then a patched low byte through
  `0x8000D5B0`/`0x80168880`.
- **Slider driver (1 body).** `0x8075B2B0` maps button messages `0x48`/`0x4A`
  onto the `+0x744` slider (`1.0`/`-1.0`), calls vtable slots `+0x70` and
  `+0x68`, and raises `+0x139` for any other message.
- **Result pane refresh (1 body).** `0x80620A00` walks the seven count words at
  `+0x358 + index * 4`, resolves each through `0x806209D0`, and copies two `f32`
  pairs plus a selector byte from `+0x248` record objects into the panes and the
  `+0x30C`/`+0x2E8` holders.
- **Record collector (1 body).** `0x806F25B0` creates one object per `+0x3DC`
  record through `0x80064680` (profile `0x20F`), writes five inherited fields,
  keeps the object with the largest bits-12..23 metric and publishes it at
  `+0x7DC`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80620A00` | `fn_80620a00` | 340 | high |
| `0x806843B0` | `fn_806843b0` | 336 | high |
| `0x806F25B0` | `fn_806f25b0` | 340 | high |
| `0x80719250` | `fn_80719250` | 336 | high |
| `0x8075B2B0` | `fn_8075b2b0` | 336 | high |

## Corrections and notes

- `0x80620A00` and `0x806F25B0` take `this`: `bl 0x802DCD6C`/`0x802DCD64` are
  `_savegpr` helpers whose `r3` survives, so the exported `void f(void)`
  prototypes are wrong. Both Rust bodies declare `this`.
- **Follow-up outside this lane:** the already-merged `fn_80621630` calls this
  function as `platform::call0(0x8062_0A00)`. Once `fn_80620a00(this)` lands the
  call site should become `abi_call1!(0x8062_0A00, this)`, otherwise the callee
  receives a stale `r3`. `src/game/bases.rs` was not edited by this lane.
- `0x806843B0`: the `0x4330` word at `0x80684414` is the *signed* magic
  (`0x807C9868 = 4503601774854144.0 = 2^52 + 2^31`) paired with
  `xoris r4,r4,0x8000`, and the byte comes through `extsb r4,r31`
  (`0x80684410`), so the table entry is sign-extended. The two multipliers are
  single precision (`fmuls f1,f2,f1`, `f2 = 32.0` at `0x807C9874`,
  `f1 = 0.5` at `0x807C985C`), so the body uses `byte as f32` instead of the
  export's `ldexpf` noise.
- `0x80719250`: the tail reads a **byte** at `0x80808438`
  (`lbz r5,0x64(r3)` with `r3 = 0x808083D4`), not the `u32` the C prints;
  converting it through `0x807CCE18` (`2^52`, unsigned magic) makes the factor
  `byte as f32`. The `fctiwz` result is stored with `stfd`/`lwz 0x3c(r1)` and
  only its low byte replaces byte 3 of the `0x8000D5B0` word
  (`stb r0,0x1b(r1)`, `0x80719378`), which is then copied from `r1+0x18` to
  `r1+0x14` for `0x80168880`. The two-word local models that stack slot pair.
- `0x806F25B0`: the `psq_st f31/f30` + `psq_l` pairs at `0x806F25C4`/`0x806F26DC`
  are GQR0 scalar saves of the two non-volatile floats, not sample decoding, so
  no GQR model is needed. The per-object `lwz 0x2314(r28)`/`0x2318(r28)`/
  `lhz 0x2324(r28)` are re-read inside the loop, after the factory call. The
  metric is `rlwinm r0,r23,20,20,31` = `(word >> 12) & 0xFFF`. The final
  `lwz r0,0x0(r27)` is unconditional, so a walk that creates no object would
  load from address zero; the body keeps that shipped behaviour.
- `0x8075B2B0`: `0x801680C0` returns in `f1`, `0x801680E0` takes `f1` plus the
  object in `r3`, and `0x80169020` returns the `u32` compared at `0x8075B374`.
  Branch polarities were read from the raw `fcmpo`/`fcmpu` + `bc 4,1`/`bc 12,2`
  pairs.

## Deferred

None.

## Verification

- `cargo check` succeeds with the block appended to a scratch copy of the tree
  (the real `src/game/bases.rs` was not modified).
- `node tools/doc-lint.mjs` on that copy reports zero violations.
- All five evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`.
- Every constant used was read out of `work/rels/elf/d_basesNP.elf`:
  `0x807C985C = 0.5`, `0x807C9860 = 0.0`, `0x807C9868 = 2^52 + 2^31`,
  `0x807C9874 = 32.0`, `0x807CCE18 = 2^52`, `0x807CEB9C = 0.0`,
  `0x807CEBA0 = 1.0`, `0x807CEBA4 = -1.0`, `0x807C6EEC = 0.0`,
  `0x807CC0F0 = 2.0`, `0x807CC0F8 = 2^52 + 2^31`.
