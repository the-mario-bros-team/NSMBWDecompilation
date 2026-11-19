# Coverage - Wave 66 (`game::bases`, `0x8078` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** from the canonical synthetic
base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime address: the
Wii REL loader picks the real load address. Byte counts come from
`work/ghidra/rel-d_basesNP/export/functions.tsv`; the cited evidence files are the
Ghidra C exports under `work/ghidra/rel-d_basesNP/export/`. Confidence is `high`
when the guards, stores, constants and call arguments are literal, and `medium`
when a field role, a callee identity or an argument dropped by Ghidra had to be
inferred (the body then carries `// observed:`, `// inferred:` or `// unknown:`).

The float constants used by the new bodies were re-read as big-endian words from
`work/rels/d_basesNP.rel`: `0x807CF698` is `0.0f`, `0x807CF69C` is `1.0f`,
`0x807CF6C0` is `-1.0f`, `0x807CF668` is `1.2f`, `0x807CF66C` is `0.75f`,
`0x807CF7B0` is `0.5f`, `0x807CF7B4` is `-0.5f`, `0x807CF7E4`/`7E8`/`7EC` are the
integers `0`, `2` and `5`, `0x808187BC` is `70.0f` and `0x808187C0` is `130.0f`.
The remaining `0x8042xxxx` constants live in `main.dol` and are read through
`platform::read_*` instead of being restated.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 2568 | 21 | 9 |

## What this pass covers

- **`character_SV` player-model actor tail, `0x80780510..0x80782100`** — the
  actor that builds the `character_SV` / `g3d/model.brres` model for one player
  slot (`+0x200` player number, `+0x1CC` player view, `+0x28C` 21-entry
  animation-handle array, `+0x2D8` current animation). Ghidra has no symbol for
  it, so the class identity stays `// unknown:`; the layout strings `Fs00` /
  `Fs01` at `0x80818488` and the `wrist_l1` / `wrist_r1` animation names at
  `0x808184FC` are the only identity evidence. New bodies: the two per-player
  scale presets [`fn_80780510`] / [`fn_807805c0`], the anchored position delta
  [`fn_807806b0`], the chain count and first-flagged-node walk
  [`fn_80780a40`] / [`fn_80780b80`], the manager-anchor wrapper
  [`fn_80780bb0`], the `+0x214` ring writer [`fn_80780f50`], the empty 4-byte
  body [`fn_807811b0`], the current/previous player pair check
  [`fn_807811c0`], the animation-table selectors [`fn_807812e0`] /
  [`fn_80781330`] / [`fn_80781380`], the swap-pair test [`fn_80781880`], the
  two scene attribute probes [`fn_807818f0`] / [`fn_80781950`], the manager and
  character-byte tests [`fn_80781b40`] / [`fn_80781c30`] / [`fn_80781c50`], the
  anchor switch [`fn_80781cb0`], and the 8-byte ring pair
  [`fn_80782000`] / [`fn_80782050`] with its capacity setter [`fn_80782100`].
- **`cobHatenaSwitch` course object, `0x80782280..0x80783CD0`** — the
  `?`-block switch whose model loader `0x807826B0` (translated in an earlier
  wave) consumes the very sub-objects this cluster builds. New bodies: the
  constructor [`fn_80782280`] (vtables `0x80818660`, `0x807E3E98`,
  `0x803295D8`/`0x80329578`, sub-objects `+0x188`, `+0x1A4`, `+0x1B0`,
  `+0x1F4`), the matrix/angle writer [`fn_807828c0`], the two-slot release
  [`fn_807832b0`], the `+0x424` pressed-flag pair
  [`fn_80783520`] / [`fn_80783540`], the `+0x430` position reader with its
  `130.0f`/`70.0f` offsets [`fn_80783890`], the full reset [`fn_80783990`] and
  the first state select [`fn_80783cd0`].

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80780510` | `fn_80780510` | 172 | high |
| `0x807805C0` | `fn_807805c0` | 240 | medium |
| `0x807806B0` | `fn_807806b0` | 292 | medium |
| `0x80780A40` | `fn_80780a40` | 68 | high |
| `0x80780B80` | `fn_80780b80` | 36 | medium |
| `0x80780BB0` | `fn_80780bb0` | 20 | medium |
| `0x80780F50` | `fn_80780f50` | 92 | high |
| `0x807811B0` | `fn_807811b0` | 4 | high |
| `0x807811C0` | `fn_807811c0` | 12 | high |
| `0x807812E0` | `fn_807812e0` | 76 | medium |
| `0x80781330` | `fn_80781330` | 72 | medium |
| `0x80781380` | `fn_80781380` | 108 | medium |
| `0x80781880` | `fn_80781880` | 112 | high |
| `0x807818F0` | `fn_807818f0` | 84 | high |
| `0x80781950` | `fn_80781950` | 84 | high |
| `0x80781B40` | `fn_80781b40` | 28 | high |
| `0x80781C30` | `fn_80781c30` | 28 | high |
| `0x80781C50` | `fn_80781c50` | 36 | high |
| `0x80781CB0` | `fn_80781cb0` | 96 | high |
| `0x80782000` | `fn_80782000` | 68 | high |
| `0x80782050` | `fn_80782050` | 84 | high |
| `0x80782100` | `fn_80782100` | 120 | medium |
| `0x80782280` | `fn_80782280` | 172 | high |
| `0x807828C0` | `fn_807828c0` | 176 | medium |
| `0x807832B0` | `fn_807832b0` | 80 | high |
| `0x80783520` | `fn_80783520` | 12 | high |
| `0x80783540` | `fn_80783540` | 52 | high |
| `0x80783890` | `fn_80783890` | 72 | high |
| `0x80783990` | `fn_80783990` | 60 | high |
| `0x80783CD0` | `fn_80783cd0` | 12 | high |

## Address range

`0x80780510..0x80783CD0`, two runs: `0x80780510..0x80782100` (interrupted by the
deferred `0x80780A90`, `0x80780FA0`, `0x807812D0`, `0x807813F0`, `0x807816B0`,
`0x807819B0`, `0x80781C80` and `0x80781C90`) and
`0x80782280..0x80783CD0` (interrupted by the deferred `0x807826B0`,
`0x80782F60`, `0x80783590` and `0x80783CE0`).

At lane start the `0x8078` bucket held 203 functions, 76 of them already
translated (37 in this bucket's own ledger rows plus the wave-49
`0x807840F0..0x80786380` run recorded under the same bucket). This pass adds 30,
leaving 97 for later waves.

## Deliberate skips

- `0x807812D0` (4) — the external jump thunk `thunk_EXT_FUN_80100940`. It only
  re-enters the relocated import, matching the skip policy for the adjustor
  thunks in wave 65. [`fn_80781380`] calls the import `0x80100940` directly and
  says so in its doc block.
- `0x80780A90` (240), `0x80780FA0` (520), `0x807813F0` (696), `0x807816B0`
  (460), `0x807819B0` (388), `0x80781DB0` (564), `0x80780D00` (276) — the
  outlined paired-single block (`unaff_GQR0`/`unaff_GQR3`, `ldexpf`, the
  `bVar == 4 || bVar == 6` lane split). Same decompiler failure class recorded
  for `0x806914D0`, `0x806F0F20` and `0x80740370`: the conversion inputs are
  invented, so no honest body is possible.
- `0x80781C80` (16), `0x80781C90` (32), `0x80783590` (32) — truncated by bad
  instruction data (`halt_baddata()`); only a single store survives in each, so
  the body is not reconstructed.
- `0x80780010` (376), `0x807807E0` (604), `0x80780BD0` (292), `0x80780E20`
  (292), `0x80782280`'s neighbours `0x807826B0` (516), `0x80782F60` (836) and
  `0x80783CE0` (748) — large, reconstructable bodies deferred to a later wave
  because this pass prioritised the contiguous short-run clusters. They are not
  failure-class functions; `0x807826B0` is the `cobHatenaSwitch` model loader
  that completes [`fn_80782280`].

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (re-read, not copied from
  prose).
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000` line.
- No address was already in `ledger/functions.csv` (checked across the whole
  `0x8078` bucket before writing).
- `node tools/check-lane-csv.mjs wave66` reports `OK` with
  `wave66-bases.csv: 30 rows`.
- `cargo check --message-format=short` produces no error or warning for
  `src/game/bases.rs` (the only remaining error is in another lane's
  `src/runtime/bt.rs`).
