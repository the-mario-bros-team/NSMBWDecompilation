# Wave 195 lane B coverage (`game::bases`)

Ten bodies in the `0x8064D650..0x806830D0` band, all translated. No deferrals.

| Address | Size | Body |
| --- | --- | --- |
| `0x8064D650` | 292 | `fn_8064d650` - initialises the path actor's scale lanes and resolves its surface record; returns `bool`. |
| `0x8064E060` | 284 | `fn_8064e060` - turns a surface record plus side selector into the initial path index `+0x3FC`. |
| `0x8064E1D0` | 264 | `fn_8064e1d0` - advances the path index, copies the vertex, derives the facing from the (dy, dx) tangent. |
| `0x80657C10` | 260 | `fn_80657c10` - runs the `+0x298` model slot, shifts 14 bytes on state 2, forwards to `0x80095480`. |
| `0x80658140` | 280 | `fn_80658140` - player distance test against `1.5 * (radius + player query)`. |
| `0x80658690` | 276 | `fn_80658690` - model tick plus the `+0x104` facing roll and the two `+0xD4` impact tests. |
| `0x806601B0` | 260 | `fn_806601b0` - the shared integer-window ratio helper; `(int*, int*, int*) -> f32`. |
| `0x80662E10` | 296 | `fn_80662e10` - rebuilds the four-entry player distance ladder from the reader count. |
| `0x80663050` | 296 | `fn_80663050` - scores each player's x offset against the ladder; returns `u32` verdict. |
| `0x806830D0` | 264 | `fn_806830d0` - places the actor at its facing offset from the followed player. |

## Notes carried into the bodies

- **`0x806601B0` signature.** It is `f32` in `f1`: the two exit points load
  `0x807C8AE8`/`0x807C8AF4` with `lfs` and the ratio is produced by `fdivs`.
  Ghidra prints `double` because of the `CONCAT44(0x43300000, ...)` conversion
  ladder, which is the signed-int-to-float idiom (`2^52 + 2^31` bias at
  `0x807C8AF8`). Existing bodies keep reaching it as
  `call_u32x3_f32(0x8066_01B0, position, low, high)`; the new body is
  `fn_806601b0(position, low, high) -> f32`. The literal branch shape returns
  `0.0` when `*position == *high` (Ghidra folds that case into `1.0`), so the
  quirk is preserved.
- **`0x8064D650` dead load.** The `lfs r0,0x38(r31)` that Ghidra uses for
  `+0xB4` is overwritten by the following `lwz r0,0xC(r1)`; `+0xB4` receives
  the record word.
- **`0x80662E10` / `0x80663050` implicit `this`.** Both prologues open with a
  `_savegpr` helper (`0x802DCD6C` / `0x802DCD70`) that the brief rules out as a
  semantic call; `r3` still holds the actor, so the bodies take `this`. The
  loop bound is the main.dol word `0x80429CA0`. `0x80663050` returns the `r28`
  verdict through `mr r3,r28` before `_restgpr`, so its real type is `u32`.
- **`0x80657C10` / `0x80658140` / `0x80658690`.** The `+0x544` model slot is
  reached as `ptr_at(ptr_at(this + 0x544) + 0x1C)`, matching the already
  translated `fn_80671d70`.
- **`0x806830D0`.** The `+0xAB4` angle is materialised as `addis -1` /
  `addi 0x4000` and truncated by `sth`, i.e. the GQR3 unsigned halfword
  `angle + 0x4000`. `0x80237C40`/`0x80237BD0` take and return `f32` (verified in
  `work/dol/main.elf`); only their cosine/sine roles are named by use.

## Files

- `work/wave195/laneB_block.rs`
- `ledger/agents/wave195-laneb.csv`
- `docs/COVERAGE_WAVE195_LANEB.md`
