# Coverage - Wave 192 lane A (`game::bases`, 192-256 byte sweep)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Deferred |
|---|---|---:|---:|---:|---:|---:|
| `src/game/bases.rs` (lane A block) | `game::bases` | 10 | 2180 | 10 | 0 | 0 |

Ten bodies, all from the 192-256 byte band. Source of the bodies:
`work/wave192/laneA_block.rs`; per-function ledger:
`ledger/agents/wave192-lanea.csv`.

## What this pass covers

- **Select arrow intro finish (1 body).** `0x80601010` (220) is the
  character-change select sibling of `fn_80601500`: state word `+0x250`
  (`1` -> animation `0` plus pane `+0x248` shown, `3` -> animation `6`), state
  word `+0x254` (`1` -> animation `1` plus pane `+0x24C`, `3` -> animation
  `7`), then pane `+0x244` shown and `+0x26A = 1`.
- **Coin-date layout loader (1 body).** `0x80609260` (200) is
  `dCollectionCoinDate_c`: gated on `+0xF4`, loads
  `worldCollectionCoinDate_12.brlyt` through the `+0x78` vtable slot `+0x14`
  (receiver `+0x70`), stores the `0x80007220` handle at `+0x244` and binds the
  `N_coinBarDate_00` (1), `P_coinBase1_00` (7), `T_worldNum_00` (4) and
  `W_fileter_00` (1) pane arrays.
- **Course-clear effect spawn (1 body).** `0x8060C6D0` (240) is the
  `dCourseClear_c` upload called by `fn_8060c5f0`: builds the 16-word transform
  record from `[+0x370]+0x84..+0xB0` (translation words duplicated into slots
  7/11, `0x807C6954` in slot 2, slot 3 never written), runs
  `0x800B37E0(record, 0)` and calls interface `[+0x208]` virtual `+0xA0` with
  `Wm_2d_courseclear` (`0x807D38D8`).
- **Arrow slot switch (1 body).** `0x80613290` (216) is the
  `fn_80613290(this, slot)` target already used by two translated bodies. It
  writes `{1,2,3,4}[slot]` (`0x807C6AB8`) into slot word `+0x29C + slot*4`,
  after copying the `+0x8` float of the first slot whose index is in `1..4`
  through the `+0x1F4` table (stride `0x28`), starts animation `new_index`
  with `0x800C9380`, and pushes the float onto the new entry with
  `0x801637D0(f32, holder)` / `0x801644D0(entry)`.
- **Slot height broadcast (1 body).** `0x80615A80` (204) starts at `0.0`
  (`0x807C6B00`), walks the three slot objects at `+0x24C..+0x254`, keeps the
  first height `>= 0.0` read through `[[slot+0x1F8]+0xC8]+0x8` (default `-1.0`,
  `0x807C6BE4`) that a `0x800C9610(slot+0x74, 5)` query says is loaded, and
  writes the chosen value to `+0x29C` and to each slot's `+0xE40`.
- **Game-over effect spawn (1 body).** `0x8061B530` (240) is the
  `dGameOver_c` twin of `0x8060C6D0`: same record from `[+0x370]`, constant
  `0x807C6C58`, interface `[+0x208]` virtual `+0xA0` with `Wm_2d_gameover`
  (`0x807D7AC4`).
- **Message-window loader (1 body).** `0x8061DA20` (212) loads
  `messageWindow/messageWindow.arc`, `messageWindow_02.brlyt`, the
  `messageWindow_02_inWindow.brlan` (3) and `A00_inWindow` (3,
  `0x807C6C98`) animations, stores the `+0x244` handle, and binds
  `T_messageS_00` (2) and `N_proportionC_00` (1) at `+0x248`/`+0x250`
  (layout `+0xAC`, vtable `+0xB4`).
- **Select row counter draw (1 body).** `0x8061ED70` (212) is the
  `fn_8061ed70(this)` target used by `fn_8061f620`. For the five rows whose
  `+0x2DB` byte is set it reads the row record through the main-DOL index table
  `0x80354E60[i]*4`, counts the decimal digits of the signed `+0x2C4` count
  with the literal divide-by-ten loop, formats through
  `0x800CDDB0(value, buf, 0x10, digits, 0)` and draws it with
  `0x800C9A60(pane, token, 300, 4, 1, buf)`.
- **Model-play arrow loader (1 body).** `0x80624400` (212) is the
  `dModelPlayArrow_c` twin of `0x8061DA20`: `modelPlayArrow.arc`,
  `modelPlayArrow_00.brlyt`, `modelPlayArrow_00_inArrow.brlan` (4),
  `A00_arrow` (8, `0x807C6F38`), handle at `+0x244`, `P_arrowL_00` (2) and
  `N_arrow_00` (1).
- **Movie-coin sparkle spawn (1 body).** `0x80627160` (224) is the
  `fn_80627160(this)` target used by the model-play tick. Only on `+0x27C == 3`
  it builds the same 16-word record from `[+0x268]` (`0x807C6FCC` in slot 2),
  runs `0x800B37E0(record, 0)` and spawns `Wm_2d_moviecoinkira` (`0x807DB248`)
  through `0x8016C890`. Sibling of the translated `fn_80627080` (`+0x27C == 4`,
  `Wm_2d_movieopen`).

## Reuse and helper notes

- No new helpers were introduced. The block uses only `platform::read_*` /
  `write_*`, `abi_call1..6`, `call0_ret`, `call_f32_u32` and `platform::call0`,
  all already present in `src/game/bases.rs`.
- The 16-word transform record matches the existing `fn_80627080` /
  `fn_8060ea50` convention (0x40-byte stack record, duplicated translation
  words at indices 7/11, unwritten slot 3); `0x800B37E0`'s role stays
  `// unknown:` exactly as in those bodies.
- `0x801637D0` is called with `(f32 value, holder)`; the argument order is
  confirmed by the translated `fn_80612000`, which uses the same call shape.
- `0x802DCD70` / `0x802DCD74` / `0x802DCDBC` / `0x802DCDC0` are
  `_savegpr`/`_restgpr` compiler thunks; the `r3`-preserving `_savegpr` at
  `0x80615A80` and `0x8061ED70` is why Ghidra prints them as `void` and
  `iVar2 = func(...)`. Both are emitted as `this: usize` bodies and the thunks
  are never emitted.
- All other callees (`0x800C8CB0`, `0x800C8D60`, `0x800C8DD0`, `0x800C8E40`,
  `0x800C8EB0`, `0x800C8FB0`, `0x800C90F0`, `0x800C92F0`, `0x800C9380`,
  `0x800C9610`, `0x800C9A60`, `0x800CDDB0`, `0x800CDC60`, `0x80007220`,
  `0x800B37E0`, `0x8016C890`) use the same ABI as in neighbouring translated
  bodies.

## Verification

- The block was appended to a scratch copy of the crate and `cargo check`
  passes with no errors.
- `node tools/doc-lint.mjs <scratch>/src` reports zero violations with the
  block merged.
- Every raw word was re-read from `work/rels/elf/d_basesNP.elf` via
  `ppc159.js`; each body's byte size matches `functions.tsv`
  (220, 200, 240, 216, 204, 240, 212, 212, 212, 224 = 2180).
- The REL constants used here were read directly: `0x807C6AB8 = [1,2,3,4]`,
  `0x807C6AC8 = 0.0`, `0x807C6B00 = 0.0`, `0x807C6BE4 = -1.0`,
  `0x807C6954 = 0.0`, `0x807C6C58 = 0.0`, `0x807C6FCC = 0.0`,
  `0x807C6C98 = 0.0`, `0x807C6F38 = 0.0`.
- All ten evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`; the CSV targets `src/game/bases.rs`.

## Signatures of the two already-referenced bodies

- `pub unsafe fn fn_80613290(this: usize, slot: u32)` - matches both existing
  call sites `abi_call2!(0x8061_3290, this, i as u32)`; `slot` indexes the four
  words at `+0x29C` and the `0x807C6AB8` table.
- `pub unsafe fn fn_80627160(this: usize)` - matches the existing call site
  `abi_call1!(0x8062_7160, this)`; returns nothing (the raw tail is a bare
  `blr`).

## Merge note (action required by the coordinator)

The first run of `work/wave192/merge.cjs` truncated two bodies. Its
`block.indexOf('pub unsafe fn fn_<addr>(')` probe matched the **signature text
inside the doc comment** of `fn_80613290` and `fn_80627160`, so it emitted only
their four-line summary docs and then cut at the next `///` line.

`work/wave192/laneA_block.rs` is fixed (the docs no longer contain the
`pub unsafe fn fn_<addr>(` literal), but `src/game/bases.rs` still holds the
two stray doc fragments, and they still make `merge.cjs`'s `present` regex
believe both names exist. To finish lane A:

1. delete lines 254458-254462 (stray `fn_80613290` doc) and 254723-254727
   (stray `fn_80627160` doc) from `src/game/bases.rs`;
2. re-run `node work/wave192/merge.cjs` - it then appends the two complete
   bodies from the block and leaves the other eight untouched.

Both bodies are otherwise complete and compile; only the merge was affected.

## Deferred

None. All ten bodies were proven from the raw instruction stream.
