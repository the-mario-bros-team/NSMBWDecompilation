# Coverage - Wave 193 lane A (`game::bases`, 192-256 byte sweep)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Deferred |
|---|---|---:|---:|---:|---:|---:|
| `src/game/bases.rs` (lane A block) | `game::bases` | 10 | 2160 | 9 | 1 | 0 |

Ten bodies, all from the 192-256 byte band. Source of the bodies:
`work/wave193/laneA_block.rs`; per-function ledger:
`ledger/agents/wave193-lanea.csv`.

## What this pass covers

- **Course-select enter skeleton (1 body).** `0x807C38A0` (104) is the bare
  `dCourseSelectManager_c` enter body: `*(0x8042A468)` is asked for `0x900`
  bytes through virtual `+0x20`, and only on a non-null answer does the pane
  vtable at `this + 0x70` receive slot `+0x18` with the descriptor
  `0x80834CC0` (`StateID_YesNoWindowExitAnimeEndWait`, the `0x8083_4CC0` row of
  the state-registration table in `fn_807c3d70`). The allocated pointer is
  tested and dropped, exactly as `fn_807c2cc0`/`fn_807c30e0` do.
- **`+0x30` child-chain walks (5 bodies).** `0x806990B0`, `0x806A5EE0`,
  `0x806AB480`, `0x806ADF30`, `0x806AEFB0` (224 each) are five per-class copies
  of one routine: five unrolled levels of `child = object->+0x30`, stop when
  `child` slot `+0xC` answers non-zero and return the parent, then a
  self-recursive tail on the advanced node. The raw words are identical in all
  five except the `bl` target, and each `bcctr` is preceded by
  `lwz r3,0x30(r31)`, so the receiver of the slot call is the child. Same shape
  as the already-translated `fn_80659260` / `fn_806535e0` walks.
- **Aim-window test (1 body).** `0x80699940` (236), `(this, other) -> u32`, is
  the query used by `fn_80699fe0` (touch vs. fall) and `0x8069B750`: gate
  `|this[+0xAC] - other[+0xAC]|` at `0x807F1390` (`160.0`), build
  `(other.x - this.x, other.y - this.y, 0.0)`, normalise it in place with
  `0x80170980`, re-read the normalised height, and accept only when it lies
  inside `sinf(-a) .. sinf(a)` where `a = fn_8069a300(this)`.
- **Rider effect-table seeds (2 bodies).** `0x806A9F50` (228) is the body the
  translated `fn_806aab90` already calls; `0x806AA040` (252) is its flight-time
  twin. Both seed three `0x18`-stride entries at `+0x6FC` (base `+0x6EC`, live
  count `+0x6F8`), register each with `0x802BE800`, seal the `+0x6C8` timing
  record with `0x800E81A0`, and write the `+0xE8` vertical timer. `0x806A9F50`
  seeds `(+64,-1,0) / (+16,0,+35) / (+64,+1,+70)` and `+0xE8 = -272.0/35.0`;
  `0x806AA040` seeds `(+64,-1,0) / (16*-3, 0, (+0x858-(+0x6B0-272))/8) /
  (+64,+1,+80)` and `+0xE8 = 8.0`.
- **`0x480` range rebuild (1 body).** `0x806BF080` (220) copies the position
  triple `+0xAC..+0xB4` to `+0x474..+0x47C`, sets `+0xDC..+0xE4 = 1.0`, fills a
  ten-word stack block `(1,1,1, -800, 0, 800, -256, 0, 0, 0)` whose tail at
  index 3 is the four-float range handed to `0x8007FB10`, then runs
  `0x8007F900(this+0x480)` and `0x806BF1C0(this)` and returns `1`.

## Reuse and helper notes

- No new helpers were introduced. The block uses only `platform::read_u32` /
  `read_u8` / `read_f32` / `write_u32` / `write_f32`, `vmethod0`, `vmethod1`,
  `abi_call1`, `abi_call2`, `abi_call6`, `call_f32x2_u32` and `call_f32_f32`,
  all already present in `src/game/bases.rs`.
- `0x802BE800(size-block, entry)` and `0x800E81A0(f32, f32, dst)` appear in the
  same shape in the translated `fn_806bff00` and `fn_806ac260`; the
  `0x8007FB10(dst, this, range, 3, byte, block)` / `0x8007F900(dst)` pair is the
  exact argument order of the translated `fn_80753780`; `0x80170980(vec3)`
  matches `fn_8064c620`'s in-place normaliser call.
- `0x802BE4A0` is used as `sinf` (`// inferred:`), matching the translated
  window body that calls it twice with a `+-` angle pair.
- `0x802DCD74` / `0x802DCDC0` in the two rider bodies are the `_savegpr` /
  `_restgpr` compiler thunks; they are never emitted, and they are why Ghidra
  prints `void FUN_806a9f50(void)` and `void FUN_806aa040(void)`.
- `0x8007FB10`'s third and sixth arguments are transcribed literally; their
  meaning stays `// unknown:`, as in every other call site in this file.

## Signatures of the already-referenced bodies

- `pub unsafe fn fn_806a9f50(this: usize)` - matches the existing call site
  `abi_call1!(0x806A_9F50, this)` in `fn_806aab90`; returns nothing (the raw
  tail is `bclr 20,0` after the argument-less epilogue thunk). The `r3` the
  export ignores is the incoming rider, preserved across `0x802DCD74`.
- `pub unsafe fn fn_80699940(this: usize, other: usize) -> u32` - matches both
  existing Ghidra call sites `FUN_80699940(param_1, candidate)` in `80699fe0`
  and `8069b750`; returns `0`/`1` in `r3`.

## Verification

- The block was appended to a scratch copy of the crate; `cargo check
  --offline` passes with no errors, and `node tools/doc-lint.mjs <scratch>/src`
  reports zero violations.
- Every raw word was re-read from `work/rels/elf/d_basesNP.elf` with
  `ppc159.js`; each body's byte size matches `functions.tsv`
  (104, 224, 236, 224, 228, 252, 224, 224, 224, 220 = 2160).
- Constants were read directly: `0x807CA4E8..0x807CA538` (`0.0`, `-1.0`,
  `1.0`, `8.0`, `-3.0`, `480.0`, `35.0`, `70.0`, `64.0`, `16.0`, `32.0`,
  `0.5`, `80.0`), `0x807CAE28..0x807CAE44` (`1.0`, `100.0`, `16.0`, `0.5`,
  `0.0`, `-256.0`), `0x807F1390 = 160.0`, `0x807CA08C = 0.0`.
- All ten evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`; the CSV targets `src/game/bases.rs`.
- The one `medium` row is `0x80699940`: the instructions are literal, but the
  `sinf` identification is inferred and the two window comparisons follow
  Ghidra's rendering (`dy <= low || high <= dy`), so the strictness at the two
  boundaries is not proven from the raw `fcmpu`/`bc` pair alone.

## Deferred

None. All ten bodies were proven from the raw instruction stream.
