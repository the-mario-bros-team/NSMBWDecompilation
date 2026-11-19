# Coverage - Wave 186 lane D (`game::bases`, 76-120 byte sweep)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Deferred |
|---|---|---:|---:|---:|---:|---:|
| `src/game/bases.rs` (lane D block) | `game::bases` | 10 | 972 | 10 | 0 | 0 |

Ten bodies, all from the sub-300 sweep. Source of the bodies:
`work/wave186/laneD_block.rs`; per-function ledger:
`ledger/agents/wave186-laned.csv`.

## What this pass covers

- **World/scene material setup (1 body).** `0x807AEE30` pushes the material key
  `(0x24, 8, 2, 1, 1)` to `0x80007610([0x8042AE10], ...)` and tail-runs
  `0x807AED70(this, 1)`. Same shape as the already-translated `fn_807b0840`
  (identical six words, different scene loader).
- **Pair of screen-state destructors (2 bodies).** `0x807AFB70` (96 bytes) and
  `0x807AFBD0` (100 bytes) are the standard
  `0x8015F9F0(this + 0x18, 0)` + optional `0x802B9280(this)` teardown already
  translated as `fn_806357e0`/`fn_80635840`; they are the `0x807AE8E0`/
  `0x807AE940` pair of the next `Env/world` scene family. The 100-byte twin only
  adds the dead `beq` at `0x807AFBF4`, exactly like `fn_806fcca0`.
- **Demo-state message dispatch (1 body).** `0x807AFEF0` queries the
  `[0x8042A440]` manager's vtable `+0x10`; when the answer is exactly `1`
  (`cntlzw(v - 1) >> 5 != 0`) it sends `0x80833A60` to its own `+0x74` interface
  slot `+0x18`. This is the same idiom as `fn_8064bcd0`, which differs only in
  the `+0x394` interface and the flag store.
- **Course-token classifier (1 body).** `0x807BAE90` is the leaf that
  `fn_807bac40` calls for every four-character `RouteInfo` token: `0x2A` unless
  `token[0] == 'W'`, then `(i8)token[3] - 49` for a digit at `[2]`, and
  `'T' -> 21`, `'C' -> 23`, `'A' -> 37`, `'G' -> 20`. The decode matches
  `docs/SPEC_WAVE155_BASES.md` §3.1 line for line, so this removes one of the
  four address-only callees listed there.
- **Pane spacing (1 body).** `0x807BB870` returns
  `(s16)pane0[+0x2C4] - (s16)pane1[+0x2C4]` in `f1`, the spacing consumed by
  `fn_807bbe20`/`fn_807bb9f0`. Also matches `SPEC_WAVE155_BASES.md` §4.
- **Pane-switch finish (1 body).** `0x807BE070` returns `1`; when both pending
  bytes `+0x228`/`+0x229` are set it refreshes `+0x22C` via `fn_807be130`,
  re-arms the four child panes via `fn_807be160`, resets the `+0x70` layout with
  `0x800C9560` and runs the layout vtable's `+0x10` slot. Same
  `+0x70`/`+0x78` receiver-vtable split as `fn_8061b450`.
- **Aim interpolation cluster (3 bodies).** `0x807C1190` is the linear
  interpolation `65C + ((frame - 660) / (66C - 660)) * (668 - 65C)` (with the
  `0.0` guard on a zero span); `0x807C1130` is its mode-gated wrapper (modes
  `1`, `2`, `4`) that truncates `+0x6AC` with `fctiwz` and returns the live `f1`;
  `0x807C1330` switches `+0x72C` to mode `1` and stores `+0x724 / 60.0` into
  `+0x728`, the per-frame decrement that `fn_807c1220`'s mode-1 arm subtracts.

## Notes

- `0x807C1130` is called by the already-translated `fn_807c0600` through
  `call_f32_1`; this block now gives it the real body. Its `void` export type is
  misleading: `f1` is seeded with `0x807D0A6C` (`0.0`), and after
  `bl 0x807C1190` it holds the callee result, which the caller stores to `+0x6A8`.
- `0x807C1130`/`0x807C1190`/`0x807C1330` all end in `f1`, so they are typed
  `-> f32` (except `0x807C1330`, whose live `f1` is only the discarded `60.0`
  conversion, typed `-> ()`).
- Constants read out of the ELF for this lane: `0x8042AE10` (resource pointer),
  `0x8042A440` (mode manager), `0x8042A1A0 + 0x29C + 8` (display mode word),
  `0x807D0A6C = 0.0`, `0x807D09F8 = 0x4330000080000000` (signed int -> double
  magic used by `0x807BB870`, subtract `4503601774854144.0`),
  `0x807D0B00 = 0x4330000080000000` (same magic in `0x807C1190`/`0x807C1330`),
  and the `lha` constant `0x808215D8 = 60` (neighbour of `0x808215CC = 10`,
  `0x808215D0 = 1.0`, `0x808215D4 = 8.0` of the `fn_807c1220` ramp).

## Verification

- `work/wave186/laneD_block.rs` type-checks standalone against stubbed helper
  signatures (`rustc --edition 2021 --crate-type lib`).
- Every body's raw words were re-read from `work/rels/elf/d_basesNP.elf`
  (`ppc159.js`) and every branch offset was checked against the export.
- All ten evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`; the CSV targets `src/game/bases.rs`.

## Deferred

None. All ten bodies were proven from the raw instruction stream.
