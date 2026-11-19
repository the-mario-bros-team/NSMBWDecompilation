# Wave 217 lane D coverage (`engine::dsys`)

Slice: the ten `engine::dsys` bodies at `0x800F9A60`, `0x800F9BA0`, `0x800F9CD0`,
`0x800F9DB0`, `0x800F9EA0`, `0x800F9F10`, `0x800FA0E0`, `0x800FA150`,
`0x800FA250` and `0x800FA330` (2104 bytes of `functions.tsv` extent). All ten are
translated; nothing was deferred.

Bodies live in `work/wave217/laneD_block.rs` and are appended to
`src/engine/dsys.rs` by the merge step; ledger rows are in
`ledger/agents/wave217-laneD.csv` (`module = engine::dsys`).

This is the course-name / screen-placement cluster on top of the `0x800F9370`
name builder and the `0x800F99F0` record lookup, plus the two-pass course scan
driver.

## Translated

| Address | Bytes | Rust body | Summary |
| --- | --- | --- | --- |
| `0x800F9A60` | 320 | `fn_800f9a60` | Builds three `u16` record ids from the defaults at `0x8042CFF8`/`0x8042CFFC` (image value `0x02EE02EE` plus `0x02EE`, i.e. `750/750/750`) and the class byte at `param_1 + 2`. `'K'` -> `0x27B/0x27C`, `'T'` -> `0x280/0x273`, `'C'` -> `0x272/0x29E`, `'A'` -> `0x283`, `'G'` -> `0x281/0x277`, `'S'` -> `0x279`, `'X'` -> `0x274`; any other byte returns `0`. Then `0x800F9530(param_1)` gives the wanted id and the three ids are tried through `0x800F99F0`; the first non-null record is returned. |
| `0x800F9BA0` | 60 | `fn_800f9ba0` | `0x800F9370(param_1, param_2, &name)` fills the five-byte name in a local slot; a zero result returns `0`, otherwise [`fn_800f9a60`] resolves it. Both incoming argument registers are still live at the `bl`. |
| `0x800F9CD0` | 84 | `fn_800f9cd0` | Fills a 16-byte local with `0x80164B20(0)` in word 0, then `0x80253ED0(&local, param_1, param_2)`. In that callee the local is the measurement object, `param_1` is the three-float destination (`x`, `y`, spare) and `param_2` the three-float point. Returns `1`. The function's own `r5` is never read. |
| `0x800F9DB0` | 240 | `fn_800f9db0` | Reads the two `u16` display fields, computes `u = (p[0] - 0.5w)/(0.5w)` and `v = (p[1] - 0.5h)/(0.5h)`, then writes `p3[0] + (u + 1)(p4[0] - p3[0])/2` and the `v` twin into the two output floats. The band ends are `-1.0`/`1.0`, so `(u - (-1)) / (1 - (-1))`. The provisional `stfs` of the raw `u` at `0x800F9E34` is overwritten at `0x800F9E74`. |
| `0x800F9EA0` | 108 | `fn_800f9ea0` | Calls [`fn_800f9cd0`] with `param_2`, copies the first two projected floats to a fresh pair and forwards that pair with `param_3`/`param_4` to [`fn_800f9db0`]. The `r5` handed to the projection call is dead. |
| `0x800F9F10` | 164 | `fn_800f9f10` | Leaf. Writes `0.25w/0.25h` to `param_1` and `0.75w/0.75h` to `param_2`, re-reading the display-info pointer before each pair of stores. |
| `0x800FA0E0` | 112 | `fn_800fa0e0` | Builds the quarter/three-quarter pairs and projects `param_1`; returns `1` exactly when `0.25h <= y <= 0.75h`, where `y` is the second projected word. |
| `0x800FA150` | 116 | `fn_800fa150` | Projects `param_1` and returns `|x - 0.5w| < 20.0`, i.e. the `LT` bit of the `fcmpo` against the `20.0f` word. |
| `0x800FA250` | 220 | `fn_800fa250` | Recursive sweep: descends the `+0x68` chain of `*(*param_1) + *(*param_1 + 0x60)` to its last node, then for that node and every `+0x64` successor calls itself with the address of the stack slot (read-only for the callee) and sets or clears bit `0x100` of `+0x14`. |
| `0x800FA330` | 680 | `fn_800fa330` | Course scan. Selects `scene + *(i32*)(scene+0x338C)*0x16518 + 0x3394`, builds the name of `param_1 & 0xFF` with `0x800F9370`, fetches the notification handle with `0x800E0380(*(r13-0x5640), -1)`, and runs passes `0` and `1`: `0x800FA630` count, `0x800FA6C0` next match, `0x800F56B0` + `0x800F9530` record id, `0x800FA8C0`/`0x800FA9C0` classification, `0x800CE3A0` notification, then the `0x800FB150`/`0x800FB1F0` (pass 0) or `0x800FB240`/`0x800FB1A0` (pass 1) probe pair. `param_2 <= 0` short-circuits to `0x800FA5E0(0, 2)`; a match at index `0` whose id is `0x26` merges `2` without probing; if nothing was handled and no pass saw a positive count, `2` is merged at the end. |

## Sizes and extents

Every `# Original` block keeps the canonical `main-gb` `functions.tsv` value, and
each was re-checked against the prologue/leaf head and the closing `blr` in
`work/dol/main.elf`:

- `0x800F9A60`: `stwu r1,-0x20` + `mflr` at `+0`; `bclr 20,0` at `0x800F9B9C`;
  ends at `0x800F9BA0` = 320.
- `0x800F9BA0`: `stwu r1,-0x10` at `+0`; `bclr 20,0` at `0x800F9BD8`; ends at
  `0x800F9BDC` = 60 (one `OP_0` pad word to `0x800F9BE0`).
- `0x800F9CD0`: `stwu r1,-0x20` at `+0`; `bclr 20,0` at `0x800F9D20`; ends at
  `0x800F9D24` = 84 (three `OP_0` words to `0x800F9D30`).
- `0x800F9DB0`: leaf, `stwu r1,-0x30` at `+0`; `bclr 20,0` at `0x800F9E9C`;
  ends at `0x800F9EA0` = 240.
- `0x800F9EA0`: `stwu r1,-0x30` at `+0`; `bclr 20,0` at `0x800F9F08`; ends at
  `0x800F9F0C` = 108 (one pad word to `0x800F9F10`).
- `0x800F9F10`: leaf, `stwu r1,-0x20` at `+0`; `bclr 20,0` at `0x800F9FB0`;
  ends at `0x800F9FB4` = 164 (three pad words to `0x800F9FC0`).
- `0x800FA0E0`: `stwu r1,-0x40` at `+0`; `bclr 20,0` at `0x800FA14C`; ends at
  `0x800FA150` = 112.
- `0x800FA150`: `stwu r1,-0x20` at `+0`; `bclr 20,0` at `0x800FA1C0`; ends at
  `0x800FA1C4` = 116 (three pad words to `0x800FA1D0`).
- `0x800FA250`: `stwu r1,-0x20` at `+0`; `bclr 20,0` at `0x800FA328`; ends at
  `0x800FA32C` = 220 (one pad word to `0x800FA330`).
- `0x800FA330`: the row computation at `0x800FA330..0x800FA34C` precedes the
  `stwu r1,-0x50` at `0x800FA350` (reached by the `b` at `0x800FA34C`); the
  `bclr 20,0` is at `0x800FA5D4`; ends at `0x800FA5D8` = 680 (two pad words to
  `0x800FA5E0`).

No entry is a prologue artefact.

## Evidence notes

- `0x800F9530` takes the record pointer in `r3`: at `0x800F9B48` the register is
  still the incoming `param_1`, which is why the Ghidra export prints the call
  with no arguments.
- `0x800F9A60`'s defaults were read from `work/dol/main.elf`: the word at
  `0x8042CFF8` is `0x02EE02EE` and the halfword at `0x8042CFFC` is `0x02EE`.
  Ghidra's `local_18._2_2_ = *(undefined2 *)(r2 - 0x6088)` in the `'A'`/`'S'`/
  `'X'` arms denotes the untouched second halfword of that word; the Rust reads
  three explicit `u16`s.
- `0x800F9CD0`'s call shape: `addi r3,r1,8` / `mr r4,r30` / `mr r5,r31`, where
  `r30` is the incoming `r3` (the destination) and `r31` the incoming `r4` (the
  point). `0x80253ED0` therefore writes the projected floats through the
  incoming destination and keeps the local as its measurement object; that is
  how `0x800FA0E0` can read `+4` and `0x800FA150` `+0` as floats. The
  `FUN_800f9cd0(..., param_3)` in `0x800F9EA0` only fills the dead `r5`.
- `0x800F9DB0` was decoded instruction by instruction because the export reuses
  `f2`/`f8` for both the normalised value and the provisional store. The image
  computes `x = (p2[0] - 0.5w)/(0.5w)` once into `f8` (provisional `stfs` at
  `0x800F9E34`), the final `x` at `0x800F9E74` and the `y` value at `0x800F9E94`.
- The `0x800FA0E0` compares are `fcmpo cr0,f0,f1` (`0x800FA114`) and
  `fcmpo cr0,f1,f0` (`0x800FA124`) followed by
  `cror cr0[EQ],cr0[LT],cr0[EQ]` (`0x4C401382`, `BT=2`, `BA=0`, `BB=2`) and
  `bc 4,2`, i.e. "branch when the value is not `>=` the bound". The Rust keeps
  the inclusive double-bound test; Ghidra's `<` rendering with reversed operands
  describes the same set.
- `0x800FA150` returns the `LT` bit of the final `fcmpo` (`mfcr` + `rlwinm
  r3,r3,1,31,31`), not an `fcmpo` result register, and the preceding `fabs` is
  rounded with an explicit `frsp` before the compare.
- `0x800FA330`'s `FUN_802dcd50` / `FUN_802dcd9c` pair is the outlined
  `__savegprlr_18` / `__restgprlr_18` (`work/dol/main.elf` at `0x802DCD50`
  stores `r18` at `r11 - 0x38`, i.e. `r1 + 0x18`). Ghidra's 64-bit "result" of
  the save call is an artefact; its two halves are just the untouched `r3`/`r4`
  (`param_1` and `param_2`), which is why the export names `uVar5` (the high
  half) in the `0x800FB150` calls while the image passes `r18 = param_1`.
- `0x800FA330`'s outer loop is a do-while (`b 0x800FA588` tests `r21 < r29`
  before the first body) that runs exactly twice, and `r22` is overwritten with
  `0x800FA6C0`'s return before being incremented, so the "next index" is the
  returned match index plus one, not the loop counter.
- The float/word constants were read from `work/dol/main.elf` and are recorded
  in the constant docs: `0x8042D010 = -1.0f`, `0x8042D014 = 1.0f`,
  `0x8042D000 = 0.5f`, `0x8042D018 = 0.25f`, `0x8042D01C = 0.75f`,
  `0x8042D008 = 2^52` (double), `0x8042D020 = 20.0f`. All are still read at run
  time; no value is inlined.

## Callees left untranslated (called by address)

`0x800F9370`, `0x800F99F0`, `0x800FA5E0`, `0x800FA630`, `0x800FA6C0`,
`0x800FA8C0`, `0x800FA9C0`, `0x800FB150`, `0x800FB1A0`, `0x800FB1F0`,
`0x800FB240`, `0x800CE3A0`, `0x80164B20`, `0x80253ED0`. `fn_800f9cd0`,
`fn_800f9db0`, `fn_800f9ea0`, `fn_800f9f10`, `fn_800f9a60` and `fn_800fa250`
are called directly as Rust functions. `0x800F9530` and `0x800F56B0` are reused
through the module's existing `FN_800F9530_F` / `FN_800F56B0_F` constants and
`0x800E0380` through `FN_800E0380`.

## Reuse

The lane reuses `platform::*`, `read_f32`, `read_f64`, `R2_BASE`, `SDA_BASE`,
`G_DISPLAY_INFO`, `FN_800E0380`, `FN_800F9530_F` and `FN_800F56B0_F`. New
constants carry the `_D` suffix (`DISPLAY_IDS_D`, `DISPLAY_ID_TAIL_D`,
`F_BAND_MIN_D`, `F_BAND_MAX_D`, `F_HALF_D`, `F_QUARTER_D`, `F_THREE_QUARTER_D`,
`D_2P52_D`, `F_CENTER_EDGE_D`, `G_SCENE_54F4_D`, `G_SOURCE_5640_D`,
`RECORD_STRIDE_D`, `RECORD_ROW_FIELD_D`, `RECORD_TABLE_BASE_D`, `FN_800F9370_D`,
`FN_800F99F0_D`, `FN_800FA5E0_D`, `FN_800FA630_D`, `FN_800FA6C0_D`,
`FN_800FA8C0_D`, `FN_800FA9C0_D`, `FN_800FB150_D`, `FN_800FB1A0_D`,
`FN_800FB1F0_D`, `FN_800FB240_D`, `FN_800CE3A0_D`, `FN_80164B20_D`,
`FN_80253ED0_D`). No existing const or helper was redefined and no new local
helper was needed.

## Verification

- `functions.tsv` sizes: all ten match the re-measured extents; none is a
  prologue artefact.
- Scratch copy: `src/`, `Cargo.toml` and `Cargo.lock` were copied to
  `%TEMP%\opencode\w217d`, `work/wave217/laneD_block.rs` was appended to its
  `src/engine/dsys.rs`, and `cargo check --lib` exited `0` with no diagnostic
  that mentions `dsys.rs`. The first run reported seven errors located in the
  appended range (`write_f32` not in scope, `call2` given three values), which
  confirms the block really is compiled; both were fixed and the copy re-checked
  to exit `0`.
- Files written: `work/wave217/laneD_block.rs`,
  `ledger/agents/wave217-laneD.csv`, this document. `src/engine/dsys.rs` was not
  modified.
