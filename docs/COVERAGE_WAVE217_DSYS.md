# Coverage - Wave 217 (`engine::dsys`, main.dol part 3)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/dsys.rs` | `engine::dsys` | 60 | 10,896 | 43 | 17 |

Third 60 `engine::dsys` bodies, `0x800F78E0`-`0x800FB240`. The module is now
**329 / 397**.

## What landed

- **State-machine bodies (lane A).** `0x800F78E0`, `0x800F7970`, `0x800F7A20`,
  `0x800F7AE0`, the 636-byte `0x800F7DA0`, `0x800F8050`, `0x800F81F0`, the
  652-byte `0x800F8230`, `0x800F8540`, `0x800F8730`.
- **Match/scene bodies (lane B).** `0x800F8770`, `0x800F87C0`, `0x800F87E0`,
  `0x800F8980`, `0x800F8A80`, `0x800F8B70`, `0x800F8B80`, `0x800F8C80`,
  `0x800F8E90`, `0x800F8F70`.
- **Stage-code formatter/parser (lane C).** `0x800F9270`, `0x800F92C0`,
  `0x800F9310`, the 436-byte `0x800F9370`, `0x800F9530`, `0x800F9620`,
  `0x800F9730`, `0x800F9840`, `0x800F9970`, `0x800F99F0`.
- **Projection and course scan (lane D).** `0x800F9A60`, `0x800F9BA0`,
  `0x800F9CD0`, `0x800F9DB0`, `0x800F9EA0`, `0x800F9F10`, `0x800FA0E0`,
  `0x800FA150`, `0x800FA250`, the 680-byte `0x800FA330`.
- **Mode probes (lane E).** `0x800FA5E0`, `0x800FA630`, `0x800FA6C0`,
  `0x800FA7E0`, `0x800FA8C0`, the 420-byte `0x800FA9C0`, the 416-byte
  `0x800FAB70`, `0x800FAD10`, `0x800FAD40`, `0x800FAD60`.
- **Progress-flag bank queries (lane F).** `0x800FADB0`, `0x800FAE10`,
  `0x800FAE40`, `0x800FAE80`, `0x800FAF70`, `0x800FB040`, four 80-byte
  wrappers `0x800FB150`-`0x800FB240`.

## Corrections and recoveries

- **Progress-flag bank.** `0x800E0380(*(r13-0x5640), -1)` selects a 0x980-byte
  block, `0x800CE3A0` reads `+group*0xA8 + index*4 + 0x6C`, and `0x800CE3C0` is
  the boolean bit test. `G_PROGRESS_BANK` (`0x8042A040`) is the module's first
  use of that global.
- **`0x800F7DA0` dead `cntlzw`.** `0x800F7EEC cntlzw r0,r3` is overwritten by
  `lfs r0,-0x60A8(r2)` (`0.0f`) before `rlwinm r4,r0,27,5,31`, so the callee
  argument is the masked constant, not `clz(x)>>5` as the export prints.
  Replayed byte-exactly on the live word.
- **Stage-code tables.** Both dispatch tables are at `0x8031C0C0` (11 words) and
  `0x8031C0EC` (41 words), reached via `lis 0x8032` + `addi -16192`. Both
  guards are `bc 12,1` (branch if greater), so `>10` / `>0x28` take the default
  arm.
- `0x800F9730` returns `r25` although Ghidra types it `void`; `0x800F9620` takes
  `(stage, dst)`. Both were pinned from the `_savegpr` fragments.
- `0x800FAD10` forwards `0x800FA330`'s `r3`: its `0x802DCD9C` tail is
  `__restgprlr_18`, which returns the accumulated code in `r3`, and the following
  `bl 0x800FAD40` never rewrites it.
- `0x800FAD60` is not `void`: `game::bases` calls `0x800FAD60(2, 3)` and both
  bytes are forwarded to `fn_800fa8c0`.
- `0x800FA0E0`'s two compares are `fcmpo` + `cror cr0[EQ],cr0[LT],cr0[EQ]`, so
  the test is the inclusive `0.25h <= y <= 0.75h`; Ghidra renders `<` with
  reversed operands but the same set.
- `0x800F9CD0` takes **two** arguments (the `r5` forwarded by `0x800F9EA0` is
  dead), and `0x80253ED0` writes the projected floats through the caller's
  buffer.
- `0x800F8E90` is single precision (`fmr` + `fmuls`) where Ghidra widens to
  `double`.
- `0x800F8230` re-reads `+0x6BC` for its `== 0` test and decrements it twice in
  the `0x5B` completion path; both reproduced.

## Merge note

Lane A mislabelled itself "lane G" and wrote `laneG_*` files. The bodies and
addresses were correct; the files were renamed to lane A before the merge, and
the coverage doc is filed as `COVERAGE_WAVE217_LANEA.md`.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-lane-csv.mjs wave217`: 60 rows, 60 unique, 43 high + 17 medium.
- Ledger: 19,850 rows (98.02%); `engine::dsys` 329/397.
- `scripts/check.ps1` exit 0.

## Next

68 `engine::dsys` bodies, then `src/resource/archive.rs` (198),
`src/game/stage.rs` (132) and two `ui::hud` stragglers.
