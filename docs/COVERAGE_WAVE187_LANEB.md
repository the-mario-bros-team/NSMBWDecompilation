# Coverage - Wave 187 lane B (`game::bases`, 128-192 byte sweep)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Deferred |
|---|---|---:|---:|---:|---:|---:|
| `src/game/bases.rs` (lane B block) | `game::bases` | 10 | 1,548 | 9 | 1 | 0 |

Ten bodies, all from the sub-300 sweep (this lane's 128-192 byte band). Source of
the bodies: `work/wave187/laneB_block.rs`; per-function ledger:
`ledger/agents/wave187-laneb.csv`.

## What this pass covers

- **Model-play state advance and commit (2 bodies).** `0x80627350` (136) and
  `0x806273E0` (156) both drive the `+0x27C`/`+0x280` state words, raise
  `+0x27A`/`+0x27B`, and run the shared tick `0x80627240`; the commit variant
  additionally re-arms `+0x280` with the `8` sentinel and, once
  `0x800C9610(this + 0xAC, -1)` reports idle, sends `0x80825A08` through the
  `+0x70` layout vtable. This is the same `+0x70`/`+0x18` descriptor dispatch as
  `fn_80627c40`'s interface group.
- **`modelPlayGuide` layout loader (1 body).** `0x80627A80` (180) opens
  `modelPlayGuide/modelPlayGuide.arc` (`0x807DB498`) and binds
  `modelPlayGuide_02.brlyt` (`0x807DB4C0`) or `modelPlayGuide_43.brlyt`
  (`0x807DB4D8`) through the `+0x78`/`+0x14` vtable, caching the `0x80007220`
  handle at `+0x208`. Same shape as the translated `fn_80640180`
  (there: `+0x7C` vtable, `+0x20C` cache).
- **Save-data readiness pair (2 bodies).** `0x80628810` (160) scans the whole
  8 x 13 slot grid through `0x800B4F40` / `0x800FD3B0`, skipping the `0x2A`
  empty sentinel, and latches `+0xA84`; `0x806288B0` (136) asks
  `0x800CE3C0(handle, 8, index, 0x30)` for the eight blocks and latches
  `+0xA83`. Both fetch the player handle first with
  `0x800E0380([0x8042A040], -1)`.
- **State-descriptor dispatch (1 body).** `0x806296A0` (184) is gated on
  `[[this+0xE0]+0x295] == 0` and sends vtable slot `+0x18` of `[this+0x74]` one
  of `0x80825B20` (KeyWait), `0x80825CA0` (CoinCountDown) or `0x80825D60`
  (WipeWait) - the three descriptors registered by `fn_8062a2c0`, so the class
  attribution is proven by the shared table rather than guessed.
- **Ranking-table insert and refresh (2 bodies).** `0x8062C770` (132) inserts a
  record into the ten-slot `+0x440` table (stride `0x10`, key at `+0x08`),
  opening the slot with `fn_8062c690`; `0x8062D120` (156) copies the four saved
  progress words (`[0x80429F7C] + selector*4 + 0x3EC`, selectors `0,1,3,2`) into
  the entry fields at `screen + index*4 + 0x40C` (indices `2,3,4,5`) through
  `0x800B3BE0`, then runs `0x8062C800(screen, 0, 0x14)`.
- **Results-screen record pair (2 bodies).** `0x806373C0` (140) maps one
  `0x3C`-byte record's flag bits `0`/`1`/`2` onto three result panes chosen by
  the record's `+0x24` panel index and the three tables `0x807C7E88`,
  `0x807C7E98`, `0x807C7EA8`, writing bit `0` of each pane's `+0xBB` byte (the
  visibility bit of `0x80637CB0`). `0x80637560` (168) tests the `+0x04` words of the four records
  (`+0x7C0`, `+0x7FC`, `+0x838`, `+0x874`, i.e. `0x7C0 + n*0x3C`) for a
  descending sequence, gated by the mode bytes `+0x899`, `+0x85D`, `+0x821`.

## Notes

- `0x80627350`/`0x806273E0` call `0x80627240` through `abi_call1!`: the tick is
  also assigned to lane A of this wave, but no body exists in `src/game/bases.rs`
  yet, and the address call is correct either way. The export's argument-less
  `FUN_80627240()` in the `else` arm keeps `r3 == this` (`mr r31,r3` and no
  clobber before `bl 0x80627240`), which the raw words confirm.
- `0x80627A80`'s mode helper `0x800B54B0` prepares no argument - `r3` still holds
  the non-zero `0x800C8CB0` result - so it is emitted as `call0_ret` and the
  leftover register is not forwarded.
- `0x8062C800` has no Rust body yet, so `0x8062D120` calls it by address.
- `0x806373C0`'s three tables are `0x807C7E88`, `0x807C7E98` and `0x807C7EA8`
  (`r7 = 0x807C7D18` plus `0x170`/`0x180`/`0x190`), each indexed by the
  record's `+0x24` word. The `rlwimi` inserts are flag bit `0`,`1`,`2` into byte
  bit `0`: `SH = 0/31/30` against `MB = ME = 31`, which is why the third write
  is a shift by two and not by one.
- `0x80637560` uses `cmpl`, so its comparisons are emitted as `u32` even though
  the export prints signed `int`; the four fields are the `+0x04` word of each
  `0x3C`-byte record.
- `0x8062D120` is typed `(void)` by Ghidra, but `r3` is consumed (`mr r27,r3`,
  then `this + index*4 + 0x40C`), so the single `screen` argument is recovered
  from the raw words. That prototype mismatch is the only reason this body is
  `medium`.
- Constants read out of the ELF for this lane: `0x807C7A00 = [0,1,3,2]`,
  `0x807C7AE0 = [2,3,4,5]`, and the three guide strings at `0x807DB498`,
  `0x807DB4C0`, `0x807DB4D8`. Table bases `0x807C7E88/98/A8` hold no fixed
  values (they are indexed rows), so they are read at runtime.
- The `0x802DCD74`/`0x802DCDC0` prologue/epilogue of `0x8062C770` and
  `0x8062D120` are the `_savegpr_27`/`_restgpr_27` compiler thunks and are
  omitted.

## Verification

- `work/wave187/laneB_block.rs` type-checks standalone against stubbed helper
  signatures (`rustc --edition 2021 --crate-type lib`).
- Every body's raw words were re-read from `work/rels/elf/d_basesNP.elf`
  (`ppc159.js`, including the `rlwinm`/`rlwimi` mask fields) and every branch
  offset was checked against the export.
- All ten evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`; the CSV targets `src/game/bases.rs`.
- `node tools/check-lane-csv.mjs wave187` sees the 10 lane-B rows (20 with lane
  C); the "no body in src/game/bases.rs" problems it prints are the expected
  pre-merge state, since the coordinator appends the blocks.

## Deferred

None. All ten bodies were proven from the raw instruction stream.
