# Coverage - Wave 212 lane B (`game::player`, main.dol)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 10 | 820 | 10 | 0 |

Addresses `0x80066EA0`-`0x80067268`: the actor-family constructor and scalar
deleting destructors that sit between the slot-selection driver `0x80066C60`
and the interface-owning teardown `0x80067270`. The block closes the last gap in
the `0x80066xxx` run, so `0x80066C60` now has a translated caller
([`fn_80066f70`]) and `0x800671B0` - already referenced by `fn_800622a0` - is
now a translated callee.

## What landed

- **Selection broadcast.** `0x80066EA0` (32 B) reads the "slot chosen" byte at
  `+0x0D`; when it is zero it sends the sentinel `-1`, otherwise the sign-extended
  `+0x0E` byte, to `0x800DBE80(this, slot)`. That callee walks the four 3-byte
  cursor records at `this` (`for (i = 0; i != 4; i++) 0x800DBD80(this + 3*i, i,
  slot)`) and steers the record whose index matches into the selected state. The
  body ends on `b 0x800DBE80`, a tail call, so the Rust body is a plain call.
- **Scene sweep.** `0x80066EC0` (176 B) runs two registry walks. The first asks
  the `0x19B` table for objects and destroys each one whose `+0x0B` byte is zero
  through `vtable(+0x60) + 0x5C`. The second walks class `2`, skips the objects
  whose `u16` class id at `+0x08` is `0x19B` (already handled), and destroys the
  rest the same way. Both walks use the `previous result -> next` iterator form
  and stop on `0`.
- **Selection tick.** `0x80066F70` (52 B) calls `0x80066C60()` (the driver that
  fills `+0x0D`/`+0x0E` and plays the select jingle) and then
  `0x80066EA0(this)`; `this` survives in `r31`.
- **Handler getters.** `0x80066FB0` and `0x80067260` (12 B each) are the
  `lis`/`addi`/`blr` constant getters for `0x80377A04`, returned through the
  module's existing `W92_DEFAULT_HANDLER_80377A04`.
- **Constructor.** `0x80066FC0` (156 B) runs the base constructor
  `0x80064110(this)`, overwrites `+0x60` with vtable `0x8030ECA0`, lays out the
  interface group (`+0x394` vtable `0x8030EDB0`, `+0x398` descriptor
  `0x802FEAF8`, `+0x39C` descriptor `0x8030EDE0`, `+0x3A0` descriptor
  `0x8030EDF8`, `+0x3A4` back-pointer, `+0x3A8` zero), builds the interface at
  `+0x3AC` with `0x8015FC90(+0x3AC, +0x398, +0x39C, 0x80377A04)`, and finally
  re-points `+0x394` at the "built" vtable `0x8030ED80`.
- **Releasers.**
  - `0x800670A0` (64 B): free `this` through `0x802B9280` when `this != 0` and
    `count > 0`, return `this`.
  - `0x800670E0` (96 B): release `+0x18` with `0x8015F9F0(member, 0)`, then the
    same conditional free.
  - `0x80067140` (100 B): instruction-for-instruction twin of `0x800670E0`.
  - `0x800671B0` (120 B): release `+0x3AC` with `0x8015F9F0(member, 0)`, reset
    the actor with `0x800642B0(this, 0)`, then free with `0x80162920` when
    `count > 0`. This is the base reset `fn_800622a0` already calls
    (`W72_MEMBER_RESET_800671B0`).

## Corrections and recoveries

- **`0x80066EA0` ends on a tail branch.** Seven instructions plus the four-byte
  `b 0x800DBE80` at `0x80066EBC` are exactly the canonical 32 bytes; there is no
  padding in this body.
- **`0x800671B0`'s 120 bytes end exactly at the padding.** The `blr` is at
  `0x80067224`; `0x80067230`, `0x80067240` and `0x80067250` are three separate
  bare-`blr` stubs (their own `FUN_` entries), not part of this body.
- **`0x800670E0`/`0x800671B0` release their members with count `0`.** That makes
  the `0x8015F9F0` call a no-op in the shipped binary (`0x8015F9F0` itself frees
  only when its second argument is positive). It is reproduced verbatim rather
  than dropped, so the call sequence stays faithful.
- **`0x80067140` keeps a dead branch.** `0x80067160` re-tests the `cr0` left by
  the prologue's `cmpi r3, 0`, i.e. the `this != 0` result, so it can never be
  taken. Ghidra folds it and so does the Rust body; behaviour is identical.
- **The `this + 0x18` / `this + 0x394` / `this + 0x3AC` tests are tautologies.**
  Both copies of the member releaser and the base reset keep
  `if (member != 0)` after an `addic.` on `this + constant`; kept as written.
- **`0x800670A0`'s free takes `this` in `r3`.** Ghidra prints the call as
  `FUN_802b9280()` because `r3` is never reloaded after the prologue; the export
  of `0x802B9280` takes one pointer, so the body passes `this`.
- **`0x80066FC0`'s base call carries `this`.** `0x80064110` is an
  `int FUN_80064110(int param_1)` constructor that initialises the object and
  writes its own `+0x60` vtable (`0x80309F6C`), which this body then replaces.
  The call is `call1(0x80064110, this)`; the return value is unused.

## Deferrals

None. All ten bodies were reconstructible from the shipped code. Callees still
untranslated are called by address, as the instructions allow: `0x800DBE80`,
`0x80162D50` (already named `FN_80162D50_E` in this module), `0x80162DB0`,
`0x80066C60`, `0x8015F9F0`, `0x802B9280`, `0x800642B0` and `0x80162920` (the last
four already have module constants and are reused).

## Verification

- Temp copy `%TEMP%\opencode\w212b` (this module's `src/`, `Cargo.toml`,
  `Cargo.lock` with the block appended to `src/game/player.rs`): `cargo check
  --lib` exits 0 with the 23 pre-existing warnings from other modules and no
  new warning in `src/game/player.rs`.
- The lane CSV has the canonical seven-column header, ten rows, plain lowercase
  hex addresses, and `# Original` sizes equal to the canonical `functions.tsv`
  (32/176/52/12/156/64/96/100/120/12 = 820 bytes).
- Files written: `work/wave212/laneB_block.rs`,
  `ledger/agents/wave212-laneB.csv`, this document. `src/game/player.rs` is
  untouched.
