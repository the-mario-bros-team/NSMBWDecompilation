# Coverage - Wave 220 (`resource::archive`, main.dol part 2)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/archive.rs` | `resource::archive` | 60 | 10,080 | 50 | 10 |

Second 60 `resource::archive` bodies, `0x802C4FB0`-`0x802C8B00`, byte-balanced
across lanes (1,456-2,036 B each). The module is now **669 / 747**.

## What landed

- **Blend and layout blocks (lane A).** `0x802C4FB0`, the 1,020-byte
  `0x802C52A0`, `0x802C56D0`, the 8-byte `0x802C6030`, `0x802C6080`,
  `0x802C6660`, `0x802C6D80`, `0x802C6DF0`, `0x802C7B70`, `0x802C83A0`.
- **Descriptors and record walks (lane B).** `0x802C6020`, `0x802C64F0`,
  `0x802C6AE0`, `0x802C6CE0`, `0x802C6D30`, `0x802C6E20`, `0x802C6F60`,
  the 848-byte `0x802C7560`, `0x802C7D30`, `0x802C80C0`.
- **Projection and table re-apply (lane C).** `0x802C60D0`, `0x802C6550`,
  `0x802C6A30`, `0x802C6E50`, `0x802C70B0`, `0x802C78B0`, `0x802C7B20`,
  `0x802C7EF0`, the 640-byte `0x802C8840`, `0x802C8AC0`.
- **Corners and text rows (lane D).** `0x802C6A20`, `0x802C6DD0`,
  `0x802C6EF0`, `0x802C6FF0`, the 488-byte `0x802C7370`, `0x802C7A50`,
  `0x802C7A90`, `0x802C7B30`, `0x802C8450`, `0x802C84F0`.
- **Quad/lane objects (lane E).** `0x802C6540`, `0x802C68E0`, `0x802C6AA0`,
  `0x802C6E80`, `0x802C8000`, `0x802C8150`, `0x802C81D0`, `0x802C81F0`,
  `0x802C83D0`, `0x802C8B00`.
- **Screen records and destructors (lane F).** `0x802C6040`, `0x802C65D0`,
  `0x802C6640`, `0x802C6BF0`, `0x802C6D60`, `0x802C7190`, `0x802C78F0`,
  `0x802C7C70`, `0x802C7E30`, `0x802C8410`.

## Corrections and recoveries

- **Asymmetric clamp.** `0x802C52A0`'s blend keeps the source when `t < 0` and
  snaps to the target when `t > 1` (visible as `mr r29,r28` / `fmr f3,f4`); its
  source/target float blocks are identity maps over `0x18..0x3C`.
- **Swapped arguments.** `0x802C7C70`/`0x802C7E30` open with
  `bl 0x802DCD74` (`__savegprlr_27`), and the exports both fabricate a `u64`
  return and **swap the two live arguments**: the descriptor goes to
  `0x8023EB30`, `this` to both GX calls.
- **Short parameter lists.** `0x802C8840`, `0x802C7EF0` and `0x802C7370` lose an
  argument to the phantom `u64` return from the `0x802DCD58`/`0x802DCD68`
  register-save helpers; the bodies use the register signatures (`f1` + `r3..r6`).
- **Integer/float misreads.** `0x802C6080`'s second source is `lfs` + four
  `stfs` (`0.0f`), and `0x802C6DF0` is two `f32` globals at
  `r13-0x47C8`/`r13-0x47C0`, not word copies.
- `0x802C7B20`'s `lbzx` table at `0x8042F3E0` is
  `1E 21 24 27 2A 2D 30 33 3F 80`.
- `0x802C8150`'s re-arm value is `0`/`1` from the `neg`/`or`/`rlwinm 1,31,31`
  idiom, not `-1`; `0x802C7B70`'s eighth argument is `desc[0x10] & 1`.
- `0x802C4FB0`'s tail `bl 0x802D2F10` is a `b 0x801C76C0` GX-flush thunk.
- `0x802C6640` is 20 bytes of code in a 24-byte slot; `0x802C64F0` is 60 in 64.
- `0x802C6A20` and `0x802C81D0` end in `bcctr` tail dispatches; `0x802C6D60`
  ends in two tail branches with no `blr`.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-lane-csv.mjs wave220`: 60 rows, 60 unique, 50 high + 10 medium.
- Ledger: 20,038 rows (98.94%); `resource::archive` 669/747.
- `scripts/check.ps1` exit 0.

## Next

78 `resource::archive` bodies, then `src/game/stage.rs` (132), the two `ui::hud`
stragglers and the untabled bodies found in dsys/archive.
