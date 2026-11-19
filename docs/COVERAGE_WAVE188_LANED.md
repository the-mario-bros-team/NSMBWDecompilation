# Coverage - Wave 188 lane D (`game::bases`, 128-192 byte sweep)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Deferred |
|---|---|---:|---:|---:|---:|---:|
| `src/game/bases.rs` (lane D block) | `game::bases` | 10 | 1640 | 10 | 0 | 0 |

Ten bodies, all from the 128-192 byte band. Source of the bodies:
`work/wave188/laneD_block.rs`; per-function ledger:
`ledger/agents/wave188-laned.csv`.

## What this pass covers

- **Lift/propeller update pair (2 bodies).** `0x806D4D30` (164) and `0x806D7D60`
  (164) are the same body with one field swapped: run `0x8006CDE0(this)`, do
  `+0x58C += +0xEC` (resp. `+0xB0 += +0xEC`), then call the actor's `+0x60`
  vtable slot `+0xE0` with `f1 = +0xAC + 0.5 * 16.0 * ((+0x4 & 0xF) + 1)` and
  `r3 = this`, and finish with `0x80064F50(this, 0)`. The nibble is the
  `rlwinm r6,r4,0,28,31` mask; the integer conversion is the usual
  `0x43300000`/`2^52` bracket (`0x807CB678`, `0x807CB7A0`). Constants
  `0x807CB66C`/`0x807CB794` = `16.0` and `0x807CB640`/`0x807CB76C` = `0.5`.
  Ghidra prints the slot argument as `(double)` only because the indirect call
  has no prototype; the instruction stream loads it with `lfs`/`fmuls`.
- **Ring-segment rewrite (1 body).** `0x806E3EB0` (188) rewrites the four
  `0x60`-stride segments at `+0x5F0` from the `+0x104` angle: per step it scales
  the `psq_l ...,1,3` unsigned-16 sample by `0x807CBC48 = 0.00390625`, runs the
  `0x80237C40`/`0x80237BD0` trig pair, writes `*96.0` (`0x807CBC4C`) to `+0x4C`
  and `+0x50`, and advances the angle by `0x4000`. The `0x770 - 0x5F0 = 0x180`
  span gives exactly four iterations. The `int` base Ghidra recovered from
  `0x802DCD74` is the incoming `r3` (a `_savegpr` helper, not a call).
- **Sub-object teardown (1 body).** `0x806E7AA0` (148) is the standard
  `(this, free) -> this` destructor already used by `fn_806b3570`: array
  teardown `0x802DCA98(+0x418, 0x8007F810, 0xE4, 5)`, then
  `0x8016A340(+0x40C, -1)`, `0x80069060(+0x3EC, -1)`, `0x800671B0(this, 0)`
  and the `free > 0` release through `0x80162920`.
- **`+0x38D` character re-anchor (1 body).** `0x806E7F50` (160) resolves the
  signed `+0x38D` byte through `0x8005F900`, rejects index `4` through
  `0x80056CF0(index, 4)`, samples the character translation with `0x8012DE80`,
  builds the matrix with `0x801C0BD0`, concatenates the model `+0x108` matrix
  with `0x801C0540` and publishes it into `+0xAC..+0xB4` through `0x8016F130`.
  **Signature: `pub unsafe fn fn_806e7f50(this: usize)`** - `void (r3 = this)`.
  The already-translated `fn_806e7ff0` reaches it as
  `abi_call1!(0x806E_7F50, this)`, so that call site needs no change; the
  `# Original` block records this.
- **`+0x80` sample window pair and heading offsets (3 bodies).** `0x806EC380`
  (148) samples `+0x80` with `0x800C12B0` and only when both components stay in
  `-16384.0..=16384.0` (`0x807CBF30`/`0x807CBF34`) applies the speed with
  `0x800C1510(+0x80, value)` and re-arms `0x800C1230(+0x80)`. `0x806EC6D0` (172)
  and `0x806EC780` (172) are the same body apart from the sign of the constant
  (`0x807CBF48 = 20.0` vs `0x807CBF4C = -20.0`): seed the two-float target with
  `0x806EC5F0(this, target)`, then add `+/-20.0 * trig(+0x178 * 0.00390625)` to
  each component.
- **Rider-mask rebuild (1 body).** `0x806ED260` (176) ticks the four bytes at
  `+0x138E..+0x1392`, then ORs `0x806ED160(this, record)` over the first record
  at `+0x1044`, every record while the `+0x1024` cursor stays below the
  `+0x1388` end pointer (`+= 4`, `+= 0x5C`) and one final record, storing the
  result at `+0x1393`. This is the writer for the mask read by the translated
  `fn_806ed310`; the traversal and end-pointer use match `fn_806ed460` and
  `fn_806ed4b0`.
- **Position cache and motion reset (1 body).** `0x806EF9E0` (148) copies
  `+0xAC..+0xB4` into `+0x448..+0x450`, feeds the horizontal deltas to
  `0x801611C0` (`f1 = dy`, `f2 = dx`, the same shape as `FUN_806b9800`), then
  writes `+0x114 = -0.1875` (`0x807CBFE0`), `+0xF4 = 0.0` (`0x807CBFDC`),
  `+0xF8 = -8.0` (`0x807CBFE4`) and `+0xFC = 0.0`.

## Reuse and helper notes

- No new helpers were introduced. The block uses only `platform::read_*`/
  `write_*`, `abi_call1..4`, `ptr_at`, `call_f32_u32`, `call_f32x2`,
  `call_f32x3_u32` and `f32_of_f32`, all already present in
  `src/game/bases.rs`.
- `0x806EC5F0` and `0x806ED160` still have no Rust bodies, so they are reached
  through `abi_call2!`; `0x806EC5F0(this, target)` is documented from its own
  raw body (`target[i] = this[0x16C/0x170] + scale * trig`), `0x806ED160`
  returns the per-cart rider byte.
- `0x8006CDE0`, `0x80064F50`, `0x802DCA98`, `0x800671B0`, `0x80162920`,
  `0x800C12B0`/`0x800C1510`/`0x800C1230`, `0x8012DE80`, `0x8016F130`,
  `0x801C0540`, `0x801C0BD0` and `0x801611C0` are all used exactly as in the
  neighbouring translated bodies (`fn_806bccf0`, `fn_806ec420`, `fn_806ec560`,
  `fn_806ed460`, `fn_80671620`), so no ABI was guessed from scratch.
- `0x806E3EB0` and `0x806ED260` look argument-less in Ghidra because their first
  use of `r3` sits after a `0x802DCD74` `_savegpr` helper; per the wave-188
  decoding rules `r3` survives that helper, so both take `this` in `r3`.

## Verification

- `work/wave188/laneD_block.rs` type-checks standalone against stubbed helper
  signatures (`rustc --edition 2021 --crate-type lib`).
- Every raw word was re-read from `work/rels/elf/d_basesNP.elf` via
  `ppc159.js`; each body's size matches `functions.tsv` (164, 164, 188, 148,
  160, 148, 172, 172, 176, 148).
- All ten evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`; the CSV targets `src/game/bases.rs`.
- `node tools/check-lane-csv.mjs wave188` sees the 10 lane-D rows (30 with
  lanes A/B); it reports the bodies as "not yet in `src/game/bases.rs`", which
  is the expected pre-merge state - the coordinator appends the block.

## Deferred

None. All ten bodies were proven from the raw instruction stream.
