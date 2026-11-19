# Coverage - wave 217 lane E (`engine::dsys`, `0x800FA5E0..0x800FADA0`)

Lane E of wave 217 translated the ten `0x800F0000`-bucket decision helpers that
sit between the scene/course record tables (`0x800F56C0..0x800F5830`) and the
per-course mask tests (`0x800FB150..0x800FB240`). All ten are `translated`;
nothing was deferred.

Block: `work/wave217/laneE_block.rs`. Ledger rows:
`ledger/agents/wave217-laneE.csv`. Target file: `src/engine/dsys.rs` (not edited
by this lane).

Total: 1,908 bytes selected across 10 functions.

## Bodies

| Address | Bytes | Name | Description | Confidence |
| --- | ---: | --- | --- | --- |
| `0x800FA5E0` | 60 | `fn_800fa5e0` | Merge two accumulated codes: `0` yields the new code, `1` keeps `1` only while the new code is `> 1`, `2`/`3` are fixed points, anything else passes through. | high |
| `0x800FA630` | 136 | `fn_800fa630` | Count the records carrying a 4-byte key by repeatedly calling `0x800FA6C0` from `index + 1` until it answers `-1`. | high |
| `0x800FA6C0` | 280 | `fn_800fa6c0` | Search the 0x1C0-strided record array from `param_3`: first group through `0x800F5700`/`0x800F56C0`, second group through `0x800F5770`/`0x800F5730` compared at `+5`; returns the record index or `-1`. | high |
| `0x800FA7E0` | 212 | `fn_800fa7e0` | Ten-byte name search over the two record groups of the active scene block (`scene + [scene+0x338C] * 0x16518 + 0x3394`); returns the group index or `2`. | high |
| `0x800FA8C0` | 248 | `fn_800fa8c0` | Read `0x800CE3A0([0x8042A040] handle, a, b)` and fold bits 4, 5 and 7 into the `0..4` mode code; code `0` becomes `3` when `0x800FB1A0` (bit `0x100`) hits. | high |
| `0x800FA9C0` | 420 | `fn_800fa9c0` | Gated full decision: `param_3 & 0xFF` must equal `context[0x44]` and `param_2` must equal `context[0x54]`, else `8`; then `context[0x60]` plus the `0x10`/`0x20`/`0x80`/`0x100` bit tests produce `0..7`, with `9` as the sentinel. | medium |
| `0x800FAB70` | 416 | `fn_800fab70` | Same decision driven by `context[0x50]` (passed twice): `0xFF` -> `9`, `>= 0x2A` -> `0`, otherwise the same chain. Already named `FN_800FAB70_C` for its wave-216 caller `fn_800f7400`. | medium |
| `0x800FAD10` | 36 | `fn_800fad10` | `0x800FA330(a, b)` (forwarded arguments), then that collector's `r3` code goes to `0x800FAD40`. | medium |
| `0x800FAD40` | 32 | `fn_800fad40` | `((2 \| ~(x)) - ((2 - x) >> 1)) >> 31` with `x = param_1 - 1`: the sign of the folded word. Leaf, no frame. | high |
| `0x800FAD60` | 68 | `fn_800fad60` | `1` when `0x800FA8C0(a, b)` answers `1`, `2` or `4`, else `0`; the two arguments are forwarded unchanged. | high |

## Decoding notes

- **Sizes are all real.** `functions.tsv` gives 60 / 136 / 280 / 212 / 248 /
  420 / 416 / 36 / 32 / 68. Each body's last instruction is a `bclr 20,0`
  exactly four bytes before the next table entry (`0x800FA61C`'s padding,
  `0x800FA6B4`, `0x800FA7D4`, `0x800FA8B0`, `0x800FA9B4`, `0x800FAB60`,
  `0x800FAD0C`, `0x800FAD30`, `0x800FAD5C`, `0x800FADA0`), so none of the ten
  sizes is a prologue artefact. `0x800FA5E0` and `0x800FAD40` are leaf
  functions with no stack frame at all.
- **`0x802DCD70` / `0x802DCD74` / `0x802DCDBC` / `0x802DCDC0`.** They are the
  outlined `__savegprlr_26`/`27` and `__restgprlr_26`/`27` compiler helpers
  (`0x802DCD70` is `stw r26,-0x18(r11)`, `0x802DCDBC` is `lwz r26,-0x18(r11)`),
  not value-returning functions. The `undefined8` the exports print for them is
  the untouched `(r3, r4)` pair, so the *high* word is the first argument and
  the *low* word the second. That reading is confirmed by `0x800FB6C0`, where
  the same stub yields `(text pointer, digit)` for a body whose two arguments
  are exactly those. The save stubs are not emitted; the rest stubs are
  replayed with the live result, as `fn_800f5310` already does.
- **`0x800FA6C0`'s compared value is `param_1`, not `param_2`.** At
  `0x800FA6D8` the body does `mr r26,r3` before anything else and both
  `0x802E1BD0` calls pass `r4 = r26`; `param_2` only feeds the
  `cntlzw`-derived group selector (`rlwinm r31,r0,27,5,31` at `0x800FA6E4`).
  `fn_800fa7e0` uses the same idiom with its group counter
  (`cntlzw r0,r30`, `0x800FA818`).
- **`0x800FA630`'s loop test.** `cmpi cr0,5,r5,0` / `bc 4,0` is "while the
  search result is non-negative"; the count is incremented before the next
  `index + 1` call, matching the export.
- **`0x800FAD10` really forwards a value.** `0x800FA330` is declared `void`,
  but its last action is `FUN_802dcd9c(uVar15)` = `__restgprlr_18`, which
  returns with `r3` still holding the accumulated code, and `0x800FAD10`
  performs `bl 0x800FAD40` with no `r3` rewrite in between. The block therefore
  models the collector as returning `u32` through the ordinary `call2` and
  passes it on; the second collector argument is likewise the caller's `r4`,
  which the body never writes, so both arguments are forwarded.
- **`0x800FAD60` is not `void`.** Ghidra prints `FUN_800fad60(void)` only
  because the body never rewrites `r3`/`r4` before `bl 0x800FA8C0`;
  `game::bases` calls the address as `0x800FAD60(2, 3)`, which both proves the
  two live arguments and confirms that `0x800FA8C0` masks them to bytes
  (`rlwinm r31,r3,0,24,31`).
- **`or` vs `orc` in `0x800FAD40`.** `0x7C632338` is printed as the raw
  `X31_412`; XO `412` is `orc`, so `r3 = 2 | ~(param_1 - 1)`, which is exactly
  the export's `(~(param_1 - 1U) | 2)`. The closing
  `rlwinm r3,r0,1,31,31` keeps only the sign bit (`>> 0x1f`).
- **`0x800FAB70` reuses `FN_800FAB70_C`.** That constant already exists in this
  module for the wave-216 caller `fn_800f7400` ("the game-mode probe: returns
  `0..9`, `9` meaning no mode"); this lane does not redefine it and does not
  need a second name for the address. It also settles the semantics of the `9`
  sentinel used by `0x800FA9C0` and `0x800FAB70`.

## Verification

- `cargo check --lib` on a scratch copy of `src/`, `Cargo.toml` and
  `Cargo.lock` (`%TEMP%\opencode\w217e`) with this block appended to
  `src/engine/dsys.rs`: **exit 0**, 23 warnings - the same pre-existing set as
  the untouched tree, none of them in `dsys.rs`.
- `node tools/check-lane-csv.mjs wave217` run in that scratch tree (carrying
  `ledger/functions.csv`, the lane CSV, `work/ghidra/main-gb/export/
  functions.tsv` and the ten evidence files): `wave217-laneE.csv: 10 rows`,
  `total rows: 10, unique addresses: 10`, `translated/high=7
  translated/medium=3`, `OK` - including the `# Original` byte-size
  cross-check against `functions.tsv`.
- Files written: `work/wave217/laneE_block.rs`,
  `ledger/agents/wave217-laneE.csv`, this document. `src/engine/dsys.rs` was
  not edited by this lane.
