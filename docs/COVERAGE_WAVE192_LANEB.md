# Wave 192, lane B coverage (`game::bases`)

Lane band `0x80629A30..0x80638F00`, in the `dMultiCourseSelect_c` /
results-screen cluster of REL `d_basesNP`. Ten of ten requested bodies
translated; none deferred.

## Bodies

| Address | Bytes | Function | Summary |
| --- | --- | --- | --- |
| `0x80629A30` | 252 | `fn_80629a30` | Publishes the selected row's `+0x1B8..+0x1CC` words into the `0x80373D60` descriptor block, calls `0x80102260(6,0,2,1)`, mirrors `+0xA88`/`+0xA90` into `[0x80429F7C]+0xAEC/+0xAF0` and dispatches `+0x18` of `[this+0x74]` (`0x80825DE0`). |
| `0x8062C270` | 236 | `fn_8062c270` | For each of twenty panes with `+0x291` set, copies five floats from `[this+0x3B0]+0x84/0x90/0x98/0xA0` (and the `0.0` at `0x807C7AD8`) into `[pane+0x244]+0x2C..0x48`. |
| `0x8062C800` | 216 | `fn_8062c800` | For rows `start..end`: stores `this+0x500` into `[this+row*4+0x2DC]+0x2A0`, resolves the `(page, column)` pair with `0x8062ED90`, maps a non-`10` page through `0x8062ED70`, and stores both as words at `+0x2AC`/`+0x2B0`. |
| `0x8062EA90` | 196 | `fn_8062ea90` | Mirrors `+0x500`/`+0x4F4` into `[0x80429F7C]+0x3E4/+0x3E8`; on page `10` writes the resolved pair into `[0x80429F7C]+0x770` (or `+0x400` when bit `0x40` of `0x80429F80` is set); sets `+0x50F`. |
| `0x8062ED90` | 240 | `fn_8062ed90` | Resolves one row's two label bytes: mode `0` indexes `0x807C7B6C`/`0x807C7B58` (selected by bit `0x20` of `0x80429F80`), mode `10` reads `this+index*0x10+0x440/+0x444`, else delegates to `0x8062ED60`/`0x8062ED70`. |
| `0x80637610` | 204 | `fn_80637610` | Rebinds the four results records: writes `+0x7DC`, `+0x7E4`, and when `r4 != 0` retargets the `+0x74C`/`+0x75C` pane texts through `0x800C9A60(...,0x65,message,0)` with `message+4` for the second slot. |
| `0x80637ED0` | 240 | `fn_80637ed0` | Sets bit `0` of `[this+0x6E4]+0xBB`, registers the four channels `+0x6EC..+0x6F8` with `0x802AB810`, runs `0x80637450`, then rebinds `+0x7E0+0x19` per record via `0x802AB7C0` and `0x800C92F0`/`0x800C93D0`. |
| `0x80638B70` | 252 | `fn_80638b70` | While `0x800C9640(this+0x70)` reports idle, dispatches `0x80826DA0` if any of the four record `flag && key` pairs is set, else clears `+0x8AF` and dispatches `0x80826E60`. |
| `0x80638D90` | 252 | `fn_80638d90` | While pane channels `12..15` are idle, decrements `+0x8D8` or reloads `0x1E` and either dispatches `0x80826DE0` or steps `+0x8D0` and rebinds channel `record[+0x7E0]+12`, then runs `0x806376E0`/`0x80637910`. |
| `0x80638F00` | 252 | `fn_80638f00` | Across two `0x78`-byte blocks, writes the selected pane index into `+0x8B4` and sets/clears bit `0` of the pane's `+0xBB` byte, then `0x800C92F0(this+0x70,0x10,0,2)` and sound `0x9F`. |

## Evidence and checks

- Every body was re-read from the raw words with
  `ppc159.js work/rels/elf/d_basesNP.elf <addr>`; the Ghidra exports were used
  only as an index. `bl 0x802DCD68/70/74` and `bl 0x802DCDB4/B8/C0` are the
  `_savegpr`/`_restgpr` compiler helpers and are not emitted.
- The Ghidra exports of the four `0x8063_xxxx` results-screen bodies and
  `0x8062C800`/`0x8062ED90` lose `r3`/`r4` to the save thunks and redeclare the
  functions `(void)`. In every case the raw `mr r2x,r3` (and, for
  `0x80637610`, `mr r25,r4` with the `cmpi r25,0` gate) recovers the real
  argument list used here.
- `0x8062C270`: the export's stack spills at `r1+0x08..0x4C` are dead; only the
  five `stfs` into `[pane+0x244]` are live. `0x807C7AD8` is `0.0f32`.
- `0x8062ED90`: the two table bases are `0x807C7B6C` (`& 0x20 == 0`) and
  `0x807C7B58`; the doubled index is `rlwinm r30,r30,1,0,30`, and the branch
  masks are MSB-based `rlwinm.` forms (`0x20` and `0x40`) as Ghidra printed.
- `0x80638B70`: the third/fourth checks re-base `r3 = this+0x78` before the
  `lwz`, which is why they read `+0x838`, `+0x899` and `+0x874`; the flag bytes
  stay at `+0x85D`/`+0x821` on `this`.
- `0x80638F00`: the `0x800C92F0` call forwards `r6` = the loop counter (`2`)
  implicitly, so it is emitted as the four-argument form.
- Compile check: the block type-checks standalone under
  `rustc --edition 2021` against stub `platform`/`abi_call*` helpers, and
  `tools/doc-lint.mjs` reports no violation other than the expected
  `module-doc` rule for a bare block file.
- `node tools/check-lane-csv.mjs wave192`: 10 rows, 10 unique addresses, all
  `translated/high`, no size mismatch. The six remaining "has no body" reports
  are the bodies not yet appended to `src/game/bases.rs`.

## Notes for the coordinator

- `0x8062C800` now has a real body, so the placeholder
  `abi_call3!(0x8062_C800, screen, 0, 0x14)` at the end of `fn_8062d120`
  (`src/game/bases.rs`, "unknown: 0x8062C800 re-reads the four entries") can be
  replaced with `fn_8062c800(screen, 0, 0x14)`.
- `fn_8062ed90` is new in this block and is called directly by `fn_8062c800`
  and `fn_8062ea90`; it in turn calls the existing `fn_8062ed60`/`fn_8062ed70`.
- `0x80637450` has no Rust body yet, so `fn_80637ed0` reaches it through
  `abi_call1!` with `this` in `r3`.

## Files

- `work/wave192/laneB_block.rs`
- `ledger/agents/wave192-laneb.csv`
- `docs/COVERAGE_WAVE192_LANEB.md`
