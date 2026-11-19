# Coverage - Wave 179 (`game::bases` 492-512 byte sub-band)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 15 | 7,508 | 13 | 2 |

Eighth five-lane wave on the paid model, and the last of the 300-512 byte
band: all 15 remaining bodies in 492-512 are now translated.

## Lane A (3 bodies, 1,476 bytes)

- `0x8068C210` - `gabon_rock` throw re-arm. Ghidra dropped the second
  argument: `lbzx` at `0x8068C340/0x8068C39C` reads through `r4`, and both
  callers pass the 0/1 flag, so the signature is `(actor, slot)`.
- `0x806F64B0` - `MG_balloon` loader, word-for-word the `fn_807a3d00` shape.
- `0x807003C0` - `boss_koopaJr` electric start, effects spread over
  `y-104.0 .. y+104.0` with `character * 13.5`.

## Lane B (3 bodies, 1,488 bytes)

- `0x8060C400` - `corseClear` layout loader (region switch `3/2/5/4` →
  Fra/Ger/Ita/Spa, else Eng; the byte is re-read after the layout call and that
  is preserved).
- `0x8061B260` - `gameOver` layout loader, same shape.
- `0x806F0F20` - camera-relative UI projection for panes 5, 1, 1, 1 and
  2..=5. **medium** (field roles inferred).

## Lane C (3 bodies, 1,500 bytes)

- `0x8070F7C0` - intensity-ramp driver on the `0x8082F760` BSS block.
- `0x807A2CA0` - firepillar two-emitter burst phase, sibling of
  `fn_807a2ac0`.
- `0x807B7540` - world-map screen-state selector with the `0x80429010` latch.

## Lane D (3 bodies, 1,512 bytes)

- `0x807C0400` - eight-arm world-map aim state machine on `+0x6A0`; the jump
  table at `0x808218A4` was read and matched.
- `0x80648060` - `0x1A500`-byte UI actor projection. **medium**: `r29` is
  live-in when `+0x1A4FA >= 7` and modelled as `0`.
- `0x8073FF60` - four `0x20`-byte marker records at `+0x480..+0x680`.
  `fn_80740160`/`fn_807401c0` now call the real body instead of `abi_call1!`.

## Lane E (3 bodies, 1,532 bytes)

- `0x80789790` - `Wm_ro_pipeappear` appear machine; state `0xF` passes an
  uninitialised `r1+0x20` that mode 1 of `0x80789990` provably never reads.
- `0x807195A0` - pipe rise body; `0x806601B0` really returns `f32`.
- `0x8079F040` - six-step arrival sub-machine, peer of `fn_8077d7b0`.

## Corrections and cross-checks

- `0x8068C210`'s real signature is `(actor, slot)`; both callers pass a 0/1
  flag built at `0x8068BDB8` and stored at `+0x484` by `fn_8068c140`.
- `0x8073FF60`'s Ghidra `void(void)` is the `0x802DCD68` savegpr thunk.
- `0x806F64B0` is the accepted `fn_807a3d00` shape, another family
  confirmation.
- `0x8060C400`/`0x8061B260` re-read the region byte after the layout call
  rather than caching it.
- `0x807003C0`'s `ldexpf`/`unaff_GQR0` prologue is the `f28..f31` GQR0 scalar
  save area.
- Doc-only fix in `fn_807195a0`: `+0x398` corrected to `+0x698`.

## Verification

- `cargo check` succeeds with 4,322 public bodies and no duplicate names.
- `node tools/check-lane-csv.mjs wave179`: 15 rows, 15 unique addresses,
  13 high + 2 medium.
- All 15 evidence exports exist and every size matches `functions.tsv`.
