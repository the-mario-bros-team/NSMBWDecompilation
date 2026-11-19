# Coverage - Wave 36 (WorldMap player action-state helpers)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 27 | 3672 | 19 | 8 |

## What this pass covers

- `0x8013A920..0x8013ABA0`: entry checks `0x89`/`0x15` with controller sound, the `0x2B` flag pair, the `0x45` link resolver at `+0x1008`, and the two transform/position copies built on `0x80139950`, `0x801C0BD0`/`0x801C0540`/`0x8016F130` and `0x8013AAA0`.
- `0x8013B230..0x8013B9C0`: the `+0x368` link lifecycle (`0x8013B230` entry with sound `0x75`, `0x8013B320` teardown with `0x80055D00`, `0x8013B3D0` confirm with `0x8014EEF0`), the idle gate at `0x8013B480`, the `+0x1008` claim at `0x8013B550`, the twelve-word record at `+0x2798` (`0x8013B610`/`0x8013B6C0`), the `0x4F` slot state machine driven by `+0x14A4` (`0x8013B7A0`/`0x8013B840`/`0x8013B8C0`/`0x8013B980` with sound `0x9F`), and the skill gate `0x8013B9C0` plus its `0x8013BA80` event post.
- `0x8013BB00..0x8013C3A0`: the `0x8005E6F0` branch taken when flag `+0x10D4` bit `0` is set, the state `0` entry with sound `0x1E` (`0x8013BDD0`) and its eight-bit reset (`0x8013BE80`), the `0x8012DBE0` chain propagation (`0x8013C1B0`), the `+0x17CC` armed event (`0x8013C280`, `0x8013C380`) and the `+0x17C4` clamp (`0x8013C3A0`).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8013A920` | `fn_8013a920` | 168 | high |
| `0x8013A9D0` | `fn_8013a9d0` | 112 | high |
| `0x8013AA40` | `fn_8013aa40` | 68 | high |
| `0x8013AA90` | `fn_8013aa90` | 8 | high |
| `0x8013AAA0` | `fn_8013aaa0` | 72 | high |
| `0x8013AAF0` | `fn_8013aaf0` | 172 | medium |
| `0x8013ABA0` | `fn_8013aba0` | 172 | medium |
| `0x8013B230` | `fn_8013b230` | 232 | medium |
| `0x8013B320` | `fn_8013b320` | 164 | high |
| `0x8013B3D0` | `fn_8013b3d0` | 168 | high |
| `0x8013B480` | `fn_8013b480` | 208 | medium |
| `0x8013B550` | `fn_8013b550` | 88 | high |
| `0x8013B610` | `fn_8013b610` | 164 | high |
| `0x8013B6C0` | `fn_8013b6c0` | 68 | high |
| `0x8013B7A0` | `fn_8013b7a0` | 160 | medium |
| `0x8013B840` | `fn_8013b840` | 116 | high |
| `0x8013B8C0` | `fn_8013b8c0` | 188 | medium |
| `0x8013B980` | `fn_8013b980` | 64 | medium |
| `0x8013B9C0` | `fn_8013b9c0` | 184 | high |
| `0x8013BA80` | `fn_8013ba80` | 116 | high |
| `0x8013BB00` | `fn_8013bb00` | 132 | high |
| `0x8013BDD0` | `fn_8013bdd0` | 164 | high |
| `0x8013BE80` | `fn_8013be80` | 132 | high |
| `0x8013C1B0` | `fn_8013c1b0` | 196 | medium |
| `0x8013C280` | `fn_8013c280` | 256 | high |
| `0x8013C380` | `fn_8013c380` | 24 | high |
| `0x8013C3A0` | `fn_8013c3a0` | 76 | high |

## Deliberate skips

- `0x8013B0B0` and `0x8013BB90` sit inside the chosen address range but render paired-single code as `ldexpf`/`unaff_GQR0` (and `0x8013BB90` also has removed unreachable blocks), so they are not translated honestly under the brief.
- `0x8013BF10` (668 bytes) also lies in the range but its ground-probe/string structure interleaves the `+0x14A4` states with the skipped `0x8013BB90` callee; it is left for a later pass rather than reconstructed from Ghidra's condensed branches.
- No stage-loader candidates were taken: every body in this cluster manipulates the WorldMap player's `+0x14A4` state word, the `+0x1008`/`+0x368` link handles and the `+0x9E`/`+0x4F` status bits, with controller sound `0x89`/`0x15`/`0x1E`/`0x9F`. None of these is a stage loader.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors for `src/game/worldmap.rs`.
