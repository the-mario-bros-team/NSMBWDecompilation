# Wave 213 lane E coverage (`game::player`)

Slice: the ten `game::player` bodies at `0x8006A9A0`, `0x8006C0E0`,
`0x8006C120`, `0x8006C420`, `0x8006C490`, `0x8006C500`, `0x8006C540`,
`0x8006C570`, `0x8006C580` and `0x8006C5B0`. All ten are translated; nothing
was deferred.

Bodies live in `work/wave213/laneE_block.rs` and are appended to
`src/game/player.rs` by the merge step; ledger rows are in
`ledger/agents/wave213-laneE.csv`.

## Translated

| Address | Bytes | Rust body | Summary |
| --- | --- | --- | --- |
| `0x8006A9A0` | 8 | `fn_8006a9a0` | Sound-idle leaf: `lwz r3,-0x51D8(r13)` loads the game-sound object `0x8042A4A8`, then `b 0x8019D420` tail-calls the predicate, so the callee's `r3` is the result. The caller `0x8006A830` uses it before stopping an animation. |
| `0x8006C0E0` | 60 | `fn_8006c0e0` | Publishes an id into the first zero word of the five-slot table at `+0x04..+0x17`; a `ctr = 5` loop, `bdnz`, and one `stw`. The only caller, `0x80111FD0`, passes the new effect object's `+0` registry serial, so the table is the singleton's effect-id set. |
| `0x8006C120` | 148 | `fn_8006c120` | Same five slots: each non-zero id is resolved through `0x80162D00` and cleared when the registry no longer knows it; live slots are counted and folded with `cntlzw(count - 5) >> 5`, i.e. `1` only when all five survive. |
| `0x8006C420` | 104 | `fn_8006c420` | Constructor: base `0x80161AD0(this)`, `+0x64 = 0`, `+0x60 = 0x8030F1D8`, `+0x68 = vtable[+0x4C](this)` (the class-name string), `+0x6C = 0x801017B0(*(u16 *)(this + 8))`. Returns `this`. |
| `0x8006C490` | 104 | `fn_8006c490` | Destructor: for non-null `this`, clears the list addressed by `this + 0x64` (`0x801618C0`), runs `0x80161C40(this, 0)`, and frees through `0x80162920` only for `destroy > 0`; returns `this`. Also vtable slot `+0x48`. |
| `0x8006C500` | 64 | `fn_8006c500` | Registry lookup: `0x80162D50(key, previous)` and a redundant `if (previous == 0) previous = 0;` left in the caller by the compiler. Returns the found node's `+0x10` word or `0`. |
| `0x8006C540` | 44 | `fn_8006c540` | Vtable slot `+0x0C`: `0x80161D90()` booleanised with `(-x \| x) >> 31`. The callee is the constant leaf `li r3,1` / `blr`, so the result is always `1` in this build. |
| `0x8006C570` | 4 | `fn_8006c570` | Literal thunk: the single word `0x480F5830` (`b 0x80161DA0`), both argument registers forwarded. Confirms the wave 211 lane B chain `0x8006C8A0 -> 0x8006C570 -> 0x80161DA0`. |
| `0x8006C580` | 44 | `fn_8006c580` | Vtable slot `+0x18`: `0x80161EF0(this)` (the "object is finished" predicate) booleanised with the same `(-x \| x) >> 31` fold. |
| `0x8006C5B0` | 4 | `fn_8006c5b0` | Literal thunk: the single word `0x480F59B0` (`b 0x80161F60`), both argument registers forwarded; the callee releases the object and ends in vtable `+0x48` = `fn_8006c490` with `(this, 1)`. |

## Sizes and extents

All ten `# Original` blocks keep the canonical `main-gb` `functions.tsv` value.

- `0x8006A9A0` is exactly 8 bytes: `lwz` at `+0x00`, `b` at `+0x04`, and the
  next entry starts at `0x8006A9A8`. There is no `blr` in a tail call.
- `0x8006C570` and `0x8006C5B0` are exactly 4 bytes each: one `b` word, then
  twelve bytes of zero padding up to the next 16-byte-aligned entry
  (`0x8006C580`, `0x8006C5C0`, i.e. `0x8006C574..0x8006C57F` and
  `0x8006C5B4..0x8006C5BF` are all zero).
- `0x8006C0E0`'s last instruction is the "table full" `bclr` at `0x8006C118`;
  the 60-byte table entry adds the single pad word at `0x8006C11C`. The store
  arm also returns early at `0x8006C108`.
- The remaining seven end on a `blr` four bytes before the table boundary, so
  the real instruction extent is the table size minus that pad:
  `0x8006C120` 144, `0x8006C420` 100, `0x8006C490` 100, `0x8006C500` 60,
  `0x8006C540` 40, `0x8006C580` 40 - each `blr` is the last word of the entry
  at `start + size - 4` (`0x8006C1B0`, `0x8006C484`, `0x8006C4F4`,
  `0x8006C53C`, `0x8006C568`, `0x8006C5A8`).

## Notes on the raw words and the class

- The vtable at `0x8030F1D8` was read from `work/dol/main.elf`:
  `0, 0, 0x80161D80, 0x8006C540, 0x8006C570, 0x80161E30, 0x8006C580,
  0x8006C5B0, ...`, with `+0x48 = 0x8006C490` and `+0x4C = 0x8006C660`.
  `0x8006C660` is `lis r3,0x8031` / `addi r3,r3,-3656` / `blr`, i.e. it returns
  the pointer `0x8030F1B8`, the Shift-JIS debug name
  `その他(dBase_c継承の非アクター)` ("other - a non-actor derived from
  `dBase_c`"). That is why the constructor's `+0x68` is a name pointer.
- The Ghidra exports drop three argument set-ups that the image restores here:
  `0x8006C158: lwz r3,0x4(r30); bl 0x80162D00` (slot word as the id),
  `0x8006C434: bl 0x80161AD0` with `r3` still holding `this`, and
  `0x8006C58C: bl 0x80161EF0` with `r3` still holding the caller's first
  argument (hence the `this` parameter of `fn_8006c580`).
- `0x8006C490`'s guard is `addic. r3,r3,100` at `0x8006C4B4`, i.e. the tested
  value is the 32-bit address `this + 0x64`, not the word stored there. The
  Rust body reproduces the wrap with `(this as u32).wrapping_add(0x64)` so the
  single input `this == 0xFFFFFF9C` skips the clear exactly as in the image.
- `0x8006C500`'s `cmpi cr0,4,r4,0` decodes to `r4`, `cr0`: the `4` is the RA
  field, not a condition register, so the image's test is the plain
  `previous == 0` no-op shown above. `0x80162D50` repeats the same
  normalisation internally before adding its `+0x10` bias, so the two tests
  are independent.
- `0x8006C120`'s fold is `addi r0,r29,-5` / `cntlzw r0,r0` /
  `rlwinm r3,r0,27,5,31` = `cntlzw(count - 5) >> 5`, i.e. `1` only for
  `count == 5`.

## Reused neighbours

- Existing constants: `W74_SOUND_8042A4A8` (`0x8042A4A8`, the game-sound
  object) and the `ACTOR_CLASS_VTABLE = 0x60` helper
  `call_actor_slot1(this, slot)`.
- Existing bodies called directly instead of re-declaring their addresses:
  `crate::audio::ax::fn_8019d420`,
  `crate::engine::globals::fn_801017b0` and
  `crate::resource::registry::{fn_80161ad0, fn_80162d00, fn_80162d50,
  fn_801618c0, fn_80161c40, fn_80162920, fn_80161d90, fn_80161da0,
  fn_80161ef0, fn_80161f60}`.
- New constants, suffixed `_E`: `DAT_8030F1D8_E` only.
- Helpers: `platform::{read_u16, read_u32, write_u32}`. No helper or constant
  already in `src/game/player.rs` was redefined.

## Verification

Scratch copy at `%TEMP%\opencode\w213e` (`src/`, `Cargo.toml`, `Cargo.lock`)
with this block appended to `src/game/player.rs`: `cargo check --lib` exits
`0`, with no diagnostics in the appended lines (the crate's pre-existing
warnings are unchanged).

## Deferred

None.
