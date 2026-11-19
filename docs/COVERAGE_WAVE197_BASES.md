# Coverage - Wave 197 (`game::bases` mid-size remainder)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 35 | 9,034 | 32 | 3 |

Clears every remaining body between 196 and 312 bytes.

## Highlights

- **Large bodies (304-312 B, 9).** `0x80638820` (four `0x3C` tilt words),
  `0x8065C740` (manager cache upload), `0x8068C940` (held-item hand-over),
  `0x806CAD10` (rotated position + matrix), `0x806D3370` (ground probe +
  `6500.0` panel), `0x80706850` (carried-item ring), `0x8071C180` (lift
  presence/mode scan), `0x80733900` (`houdai_slope_gun` joints), `0x80754430`
  (lift re-seat + collision box).
- **Mid-size (196-252 B, 26).** Ghost/vine bodies (`0x80762EA0`, `0x80763A30`,
  `0x8079BC10`, `0x8079C150`, `0x8079F500`, `0x807A4450`, `0x807AB590`),
  world-map bookmarks and light binding (`0x807B8A90`, `0x807BA440`,
  `0x807BA660`, `0x807BAA30`), ghost-house aim bodies (`0x807C09B0`,
  `0x807C0B20`, `0x807C1C80`, `0x807C21B0`, `0x807C5B10`), and the
  near-duplicate `0x807AF520` (`fn_807aa310` minus one trailing call).

## Call sites wired

`0x807C09B0`, `0x80762EA0`, `0x8079BC10`, `0x807BAA30`, `0x807BFC20` (six
sites total).

## Corrections and recoveries

- `0x807C09B0`'s real signature is `(_unused: u32, id: u32) -> u32`: `lis r3,1`
  clobbers the dead first argument while `r4` is forwarded into
  `0x800F56B0(course, id)`.
- `0x8071C180` recovers three out/mask pointers plus a `u32` return hidden by
  `_savegpr` thunks.
- `0x8065C740`'s `stwbrx` is byte-identical to `platform::write_u32` in this
  big-endian host model.
- `0x8068C940` emits the raw bit-30 test of `-status | status`, not
  `status == 0`.
- `0x806CAD10`'s `0x801C0BD0` receives three floats (live `f1`/`f2`/`f3`),
  not the one Ghidra printed.
- `0x806D3370`'s probe `f1` result is provably dead (both readers sit behind
  the scene bit that reloads `f1`).
- `0x80733900`'s `neg`/`mulli 90`/`srawi 14`/`addze` is truncating
  `(angle * -90) / 0x4000`.
- `0x807AF520` is byte-identical to `fn_807aa310` minus its trailing
  `0x80093F10(0,2)`.
- `0x807C5B10`'s export stops at `bl 0x8016EF50`, but the raw tail
  `0x807C5BA8..0x807C5BF0` does execute; it only scales four floats into slots
  nothing reads, reproduced verbatim and flagged (**medium**).

## Verification

- `cargo check` succeeds; the ledger holds 5,107 `game::bases` rows.
- `node tools/check-lane-csv.mjs wave197`: 35 rows, 35 unique addresses,
  32 high + 3 medium.
- All 35 evidence exports exist; 9,034 bytes total.
