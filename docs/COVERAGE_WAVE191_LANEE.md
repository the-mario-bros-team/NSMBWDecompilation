# Coverage - wave 191, lane E

Module: `game::bases` (REL `d_basesNP`, synthetic base `0x80600000`).
Band: 128-192 bytes. All five assigned functions are translated; nothing
deferred.

## Bodies

| Address | Size | Name | Summary |
| --- | --- | --- | --- |
| `0x807C2C00` | 140 | `fn_807c2c00` | If `0x800E0450(*(0x8042A040)) == 0`, pushes pane slot `+0x18` with `0x808347C0` (`StateID_SaveEndWindowOnStage`) when `*(*(0x80429FB8)+0x78) == 0`, else `0x80834A40` (`StateID_SaveError`). |
| `0x807C2CC0` | 148 | `fn_807c2cc0` | Guarded by `*(*(this+0x530)+0x297) == 0`: clears game `+0x1D8` bit `0x20`, resets input via `0x80107F10`/`0x80108160`, allocates `0x900` through `*(0x8042A468)` slot `+0x20`, then pushes `0x80834800` (`StateID_SaveEndAnimeWait`). |
| `0x807C3020` | 140 | `fn_807c3020` | `0x807C2C00` skeleton with the interrupt descriptor: `0x80834940` (`StateID_InterruptSaveEndWindowOnStage`) on the zero branch, `0x80834A40` (`StateID_SaveError`) otherwise. |
| `0x807C30E0` | 148 | `fn_807c30e0` | `0x807C2CC0` skeleton with the interrupt descriptor `0x80834980` (`StateID_InterruptSaveEndAnimeWait`). |
| `0x807C5EA0` | 136 | `fn_807c5ea0` | Binds the `+0x154` model holder from `0x80354E70[*(0x80354E60)] & 0xFF` via `0x800D6DF0(holder, 0, id, 0)`, then calls model slot `+0x5C` with `(1.2, 10.0, 0.0)`, `model`, `0xA9`. |

## Reused neighbours

- `fn_807c1fe0` (`0x807C1FE0`), `fn_807c2070` (`0x807C2070`) and `fn_807c2b70`
  (`0x807C2B70`) pin the `+0x70` pane dispatch: `vtable = *(this + 0x70)`,
  `abi_call2!(*(vtable + 0x18), this + 0x70, descriptor)`. The four new bodies
  use exactly that shape.
- `fn_80618600` (`0x80618600`) and `fn_80617d70` (`0x80617D70`) pin the
  `0x80109340` / `0x80107F10` / `0x80108160` and `*(0x8042A468)` slot `+0x20`
  sequences, including the `0x900` allocation size and the `& 0xFFFF_FFDF`
  bit clear on game `+0x1D8`.
- `fn_807954c0` (`0x807954C0`) is the same table-id + `0x800D6DF0` opener as
  `fn_807c5ea0`, differing only in the holder offset (`+0x1CC` vs `+0x154`) and
  the fourth argument (`1` vs `0`).
- `fn_807c5e60` (`0x807C5E60`) already calls `0x807C_5EA0`, and
  `fn_8077e5e0` (`0x8077E5E0`) pins the `+0x5C` pose call as
  `call_f32x3_u32x2(slot, x, y, 0.0, model, index)`.

## Notes / inferred

- The five descriptor addresses are all registered in the existing
  `dCourseSelectManager_c` state table in `bases.rs` (`register_state_desc`),
  which names them; none of them had to be guessed.
- `0x80109340` is a no-argument game getter: its call sites leave different
  junk in `r3` (here `this`), and every translated body passes `0`. The new
  `0x807C2CC0`/`0x807C30E0` follow that convention instead of forwarding the
  stale receiver.
- `0x807D0BD8 = 0.0`, `0x807D0BDC = 1.2`, `0x807D0BE0 = 10.0` were read
  directly from the REL image. The float triple is passed in `f1/f2/f3`, i.e.
  `0x807D0BDC`, `0x807D0BE0`, `0x807D0BD8` in that order.
- `0x80354E60` is the four-word player→character table; `0x80354E70` is the
  following per-character table. `fn_80633570` fills the first one and
  `0x807954C0` reads it the same way, so the index chain is not inferred.

## Deferred

None.
