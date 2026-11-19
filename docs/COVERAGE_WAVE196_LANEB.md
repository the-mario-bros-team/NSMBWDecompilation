# Wave 196 lane B coverage (`game::bases`)

Ten bodies in the `0x8079EF20..0x807BB8D0` band (the final 256-300 slice), all
translated. No deferrals.

| Address | Size | Body |
| --- | --- | --- |
| `0x8079EF20` | 288 | `fn_8079ef20` - arms the egg camera pair, spawns the effect at the lowered position, sets state `0xC`. |
| `0x8079F390` | 284 | `fn_8079f390` - resolves the course record for `+0x22C`, publishes `0x80C7115x`, forwards to `0x8079F790`. |
| `0x8079F900` | 260 | `fn_8079f900` - commits the course through `0x80100030`/`0x80100200`/`0x80100DA0` and the transition record. |
| `0x807A58E0` | 288 | `fn_807a58e0` - walks the `+0x3E8` spawn cursor and places each record through `0x807A5830`. |
| `0x807AD590` | 300 | `fn_807ad590` - either dispatches `0x80833920` through `+0x74` or rebuilds the four player slot tables. |
| `0x807B5D20` | 268 | `fn_807b5d20` - loads the demo movie light/light-map/fog trio for mode 2/3 or the looping variant. |
| `0x807B73D0` | 284 | `fn_807b73d0` - world-map screen constructor; registers nine sub-objects in main.dol globals. |
| `0x807BA920` | 260 | `fn_807ba920` - re-applies the nine-world/`0x2A`-character unlock ladder. |
| `0x807BAB20` | 280 | `fn_807bab20` - reports whether any of the four `0x3B0`-stride objects holds item `0x1A6`. |
| `0x807BB8D0` | 276 | `fn_807bb8d0` - steps the world-select cursor to the next unlocked pane and derives the turn angle. |

## Notes carried into the bodies

- **`0x807AD590` is not psq-heavy.** The earlier deferral was mistaken. The
  shipped 300 bytes contain no `psq_l`/`psq_st` at all; the only prologue
  helpers are `0x802DCD68`/`0x802DCDB4` (`_savegpr_27`/`_restgpr_27`), which the
  brief drops. The body is a plain save-record republish and is fully literal.
- **`0x807AD590` record map matches `0x8079F790`.** Both build the same 16-byte
  stack record: byte `+5` is the tag (`0xFF` here, the `tag` argument there) and
  `+0xC..+0x10` are the two parsed save bytes twice. Byte `+5` is therefore
  provably `0xFF`, not "unset".
- **`0x8079F390` dead byte ladder.** Between `0x800F56B0` and the float publish
  the binary tests record bytes `+2`/`+3` against `'C'`/`'1'`. Every arm executes
  `li r29,0xFF` and `r29` starts at `0xFF`; the instruction at `0x8079F438` is
  `extsb r4,r4` (word `0x7C800774`) and reads `r0` on one path, so the block is
  unprovable as *source* but its result is provably the constant `0xFF`. The
  body keeps the single literal tag argument.
- **`0x8079F900` / `0x8079F390` implicit `this`.** Both prologues open with
  `_savegpr` helpers (`0x802DCD74`) that Ghidra binds to the return register:
  its decompile reads `param_2 + 0x22C` where the raw words read `[r27+0x22C]`
  with `r27 = r3`. The Rust takes `this` and uses `+0x22C` directly.
- **`0x807A58E0` u32→f32 magic.** `0x807D0548` is `2^52`; the pair
  `stw 0x43300000` / `stw word` + `lfd` is the unsigned-int-to-float idiom. The
  raw opcode at `0x807A5988` is `fsubs` (not `fsub`), so the exported double
  subtract is a Ghidra typing artifact, but the value is the header word scaled
  by `0x807D0540 = 16.0`. The scene floats come from `[0x80429E00]+0x38/0x3C/0x44`
  and the constants are `8.0` (`0x807D0554`) and `500.0` (`0x807D0558`).
- **`0x807B5D20` second argument.** The export drops `param_2`; the raw
  `mr r27,r4` plus two `mr r3,r27` before `0x80164B70`/`0x80164B80` prove the
  movie index. The translated caller `fn_807b5c60` already passes it with
  `abi_call2!(0x807B_5D20, this, index)`. Both stack names are 27 bytes at most,
  so 64-byte buffers are used.
- **`0x807BB8D0` divide-by-5.** `lis 0x6666` / `addi 0x6667` / `mulhw` /
  `srawi 1` / sign-add is the canonical signed divide-by-5, so `+0x2A6` is
  `(i16)(angle / 5)` with C truncation. The `fctiwz -> sth -> lha -> negate`
  round-trip on `+0x2A4` is preserved in source order.
- **`0x807B73D0` global map.** `0x808205C8` is the vtable installed at `+0x60`;
  the nine published globals are `0x8042A008`, `0x80429F10`, `0x80429DC0`,
  `0x8042A100`, `0x80429E18`, `0x8042A050`, `0x80429EA0`, `0x8042A1C8` and
  `0x8042A1F0`. `0x807B74F0` is the shipped `bclr` stub passed as a callback
  alongside the translated `0x807B7500`.

## Verification

- The block was appended to a scratch copy of the crate and `cargo check`
  finished with no errors and no warnings in the appended range.
- `node tools/doc-lint.mjs <scratch>/src` reports `ok: true`, 0 violations.
- `node tools/check-lane-csv.mjs wave196` reports 10 unique lane-B rows. Its
  six "no body" notes are expected until the coordinator appends the block; the
  other four addresses already appear in `src/game/bases.rs` as callee literals.
- Lane C's ledger (`wave196-lanec.csv`) has five rows whose evidence files are
  missing under `work/ghidra/rel-d_basesNP/export/` (`0x807C07A0`, `0x807C0C10`,
  `0x807C0EB0`, `0x807C1220`, `0x807C3540`).

## Files

- `work/wave196/laneB_block.rs`
- `ledger/agents/wave196-laneb.csv`
- `docs/COVERAGE_WAVE196_LANEB.md`
