# Coverage - Wave 186 (`game::bases` sub-300 sweep, part 6)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 50 | 5,156 | 48 | 2 |

Sixth sub-300 wave, 10 bodies per lane; `0x807794A0` through `0x807C3260`.

## Highlights

- **`0x80799960` now has a real body** - signature
  `fn_80799960(this, index, forced)`; `forced == 0xFFFFFFFF` selects a row from
  `0x8031D39C`, and `2` overrides floats from `0x8031D650`/`D654`. It matches
  the existing `fn_807999d0` record layout. Existing call sites still use
  `abi_call3!` and can be wired later.
- **`0x807C1130` now has a real body** - mode-gated (`1,2,4`) wrapper whose live
  `f1` is the return; this explains its `void` export.
- `0x8079F4B0` was `halt_baddata`; recovered as `b 0x80102690` tail call
  (table `0x807D0240` = `4,4,5,5,7,7,10,10,10,10`).
- `0x807C3260`, `0x807C13A0`/`0x807C1410`, `0x807C2B70` resolved dropped
  arguments (`r3`/`this`) from the raw words.
- `0x8078CFA0`/`0x8078D020` operate on the 22-entry `0x8081A1E8` name table
  (`Fa00,K201,K202,Fa01,...,K301`, five leading `F` names).
- `0x80783930` builds `(f32)(pi*260/count)` through the `xoris`/`0x4330`
  u32->double ladder (`0x807CF888` = `3.1415926` as f64).

## Corrections and medium rows

- `0x807970A0` (**medium**): four `divw` sites divide by `r9`, which is set to
  `0`; PPC div-by-zero yields `0`, reproduced with `checked_div(...).unwrap_or(0)`.
- `0x80788AE0` (**medium**): call sequence and offsets literal; kind `0x280` and
  the third `0x80168230` argument inferred from the byte-identical
  `fn_80785580` binding.
- `0x807C3A90` is called as the existing `fn_807c3a90(this)`; its export is
  truncated to zero args.
- `0x807BE070`'s pane-switch uses `fn_807be130`/`fn_807be160` and layout slot
  `+0x10`.
- `0x807BAE90`/`0x807BB870` match `docs/SPEC_WAVE155_BASES.md` §3.1/§4.

## Verification

- `cargo check` succeeds with 4,618 public bodies and no duplicate names.
- `node tools/check-lane-csv.mjs wave186`: 50 rows, 50 unique addresses,
  48 high + 2 medium.
- All 50 evidence exports exist; 5,156 bytes total.
