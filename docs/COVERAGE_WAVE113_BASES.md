# Coverage - Wave 113 (`game::bases`, `0x806F` bucket)

## How to read this file

This page documents the Wave 113 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`** produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
cited file is a real Ghidra C export under
`work/ghidra/rel-d_basesNP/export/`, and every byte count was re-read from
`work/ghidra/rel-d_basesNP/export/functions.tsv`.

"Confidence" is `high` when the guards, stores, constants and call arguments are
literal, and `medium` when a field role, a callee identity or a lost receiver
had to be inferred. No body in this pass is `low`. Comments in the bodies keep
`// observed:`, `// inferred:` and `// unknown:` markers.

The `0x806F` bucket holds 195 functions; 82 were already translated before this
pass, leaving 113. This pass takes 29 of them: the contiguous
`0x806FA310..0x806FD6E0` window, which is the mini-game panel-match window of
the same mode that owns the `daMiniGameGunBatteryMgrObj_c` manager recorded in
wave 10.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 29 | 6,288 | 16 | 13 |

## What this pass covers

- **The panel-match window object (`0x806FAB30`, `0x806FA9B0`,
  `0x806FC360`).** The unnamed `0x908`-byte class built by [`fn_806fab30`]:
  class vtable `0x80802E78` at `+0x60`, layout accessor `+0x524`, animation
  blocks `+0x544`/`+0x58C`/`+0x5AC`, two fixed rings at `+0x5F8` (8 × `0xC`)
  and `+0x658` (6 × `0xC`), pane animation `+0x8D0`; its destructor and the
  state-object release wrapper.
- **The per-frame match driver (`0x806FBA90`, `0x806FC020`).** A `s16` step
  from the per-player tables `0x807CC3F4`/`0x807CC3FC` advances the panel angle
  `+0x8B4`, the heading `+0x102` and the round timer `+0x8B0`, drives the miss
  stages at `0x71B`/`0x1FFF`/`0x3FFF`, writes the player character byte
  `+0x1035`, and on completion resets the round, resolves a panel slot and
  spawns `Wm_mg_panelopen` (`0x80802D20`).
- **The 19-slot manager (`0x806FCF80`, `0x806FCF90`, `0x806FCFF0`,
  `0x806FD310`).** Selector getter, record publisher (`0x10`-byte records at
  `+0x74`), the 19-slot match scan with cues `0x5EB..0x5EF` and the six
  per-variant counters at `+0x1EC..+0x204`.
- **The effect and animation pair (`0x806FBCC0`, `0x806FBDC0`, `0x806FBDA0`,
  `0x806FBEB0`, `0x806FBF90`).** Frames `1..9` and `10..18` of the pane
  animation, and the nine `Wm_mg_itemget_*` / `Wm_mg_panelmiss*` names at
  `0x80802D78..0x80802E64`, all spawned through `0x8016C890`.
- **The per-player capture and angle helpers (`0x806FB130`, `0x806FB880`,
  `0x806FB910`, `0x806FB9D0`).** Capture probe `0x8005F900`/`0x801336E0`,
  the `0x10`-stride offset arrays `+0x744`/`+0x754`/`+0x764`, the
  `0x80133860`/`0x801339E0` angle pushes, and the round-start cue `0x5EA`.
- **The fade pair (`0x806FD440`, `0x806FD6E0`) and cursor recolour
  (`0x806FD510`, `0x806FD3D0`).** The `+0x6A1` arm / `+0x6A2` alpha bytes,
  the `0x19` per-frame step with its `6` cutoff, the four character colour
  pairs from `+0x14D4`, and the `uWm_mg_cursor_2d` pane at `0x808032E8`.
- **The select-screen variants (`0x806FA310`, `0x806FA3D0`, `0x806FA500`,
  `0x806FAD30`, `0x806FAE40`, `0x806FC1A0`, `0x806FC2A0`).** The mode gate,
  the four-way layout selector fed by the character ids `4..6` and the player
  table byte `+0x120D`, the area variant with the `0x8042C1A4` limit table, the
  one-shot flag copy, the matrix rebuild from `+0x100..+0x104`, the
  `+0x360` state pair and the packed variant nibble.

Start with [`fn_806fab30`] for the object layout and [`fn_806fcff0`] for the
match logic. [`fn_806fba90`] is the per-frame entry point.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806FA310` | `fn_806fa310` | 168 | medium |
| `0x806FA3D0` | `fn_806fa3d0` | 284 | medium |
| `0x806FA500` | `fn_806fa500` | 260 | medium |
| `0x806FA9B0` | `fn_806fa9b0` | 88 | high |
| `0x806FAB30` | `fn_806fab30` | 252 | medium |
| `0x806FAD30` | `fn_806fad30` | 108 | high |
| `0x806FAE40` | `fn_806fae40` | 192 | high |
| `0x806FB130` | `fn_806fb130` | 204 | medium |
| `0x806FB880` | `fn_806fb880` | 144 | medium |
| `0x806FB910` | `fn_806fb910` | 96 | high |
| `0x806FB9D0` | `fn_806fb9d0` | 172 | high |
| `0x806FBA90` | `fn_806fba90` | 560 | medium |
| `0x806FBCC0` | `fn_806fbcc0` | 220 | high |
| `0x806FBDA0` | `fn_806fbda0` | 24 | high |
| `0x806FBDC0` | `fn_806fbdc0` | 228 | high |
| `0x806FBEB0` | `fn_806fbeb0` | 212 | high |
| `0x806FBF90` | `fn_806fbf90` | 100 | high |
| `0x806FC020` | `fn_806fc020` | 348 | medium |
| `0x806FC1A0` | `fn_806fc1a0` | 184 | medium |
| `0x806FC2A0` | `fn_806fc2a0` | 192 | medium |
| `0x806FC360` | `fn_806fc360` | 220 | high |
| `0x806FCF80` | `fn_806fcf80` | 8 | high |
| `0x806FCF90` | `fn_806fcf90` | 92 | high |
| `0x806FCFF0` | `fn_806fcff0` | 788 | medium |
| `0x806FD310` | `fn_806fd310` | 184 | high |
| `0x806FD3D0` | `fn_806fd3d0` | 108 | high |
| `0x806FD440` | `fn_806fd440` | 196 | medium |
| `0x806FD510` | `fn_806fd510` | 464 | medium |
| `0x806FD6E0` | `fn_806fd6e0` | 192 | high |

**Totals:** 29 translated, 0 partial, 0 stub (16 high, 13 medium, 6,288 bytes).

## Deliberate skips

Read and rejected, with the reason:

- **Paired-single bodies.** `0x806FB540` (200 bytes) and `0x806FB610` (296)
  open with `unaff_GQR0`/`unaff_GQR3` brackets, `ldexpf` lane conversions and
  an `extraout_r4` halfword, so the dequantisation of the camera angle cannot
  be reconstructed. Their caller [`fn_806fb880`] and the slot walker
  `0x806FB740` keep them as address calls with explicit notes. `0x806FB7C0`
  (188) has the same GQR0 bracket and a dropped receiver on `0x80162D00`.
- **`0x806FCC60..0x806FCDF0` neighbours not in this window.** `0x806FCC60`,
  `0x806FCCA0`, `0x806FCD70`, `0x806FCDB0` and `0x806FCDF0` belong to the
  surrounding manager block and were left for a later pass (budget, not doubt).
- **Large bodies not opened.** `0x806F3350` (2,092 bytes), `0x806F7BB0`
  (1,004), `0x806FE480` (892), `0x806FD820` (912), `0x806FDD50` (536) and the
  rest of `0x806FE020..0x806FFC50` were not opened in this pass; several are
  the manager-side bodies reached from the new code, which call them by
  address with `// unknown:` notes.
- **Two-argument `0x80166F10` call site.** In `0x806FA500` Ghidra prints one
  `0x80166F10` call as `(anim, 0)` because it could not recover the `f1`
  argument. The body reuses the `0x807CC358` speed of the same branch and says
  so with an `// inferred:` comment; the call itself, its target and its
  position are literal.

## Notes on inference

- The class is unnamed in every export. Its strings (`Wm_mg_panelmiss`,
  `Wm_mg_itemget_*`, `uWm_mg_cursor_2d`) and the shared global manager
  `0x8082F078` place it in the mini-game mode; the module header and the bodies
  call it the "mini-game panel window" and mark the name `// unknown:`.
- `0x802DCD70`/`0x802DCD74`/`0x802DCDC0` are the compiler save/restore helper
  family documented in `docs/COVERAGE_r80260_802ed.md`. They hide the incoming
  `this` in `0x806FBA90` and both arguments of `0x806FCFF0`; the bodies restore
  `actor`/`manager`/`player` as parameters and say so in their doc blocks. This
  follows the existing `fn_806f9d50`/`fn_806f9e20` convention in this file.
- The `+0x43300000` / `0x807CC3E8` expression in `0x806FBCC0`/`0x806FBDC0` is
  the compiler's u32→`f32` conversion idiom, not a real double subtraction.
- `0x8042A440`, `0x8042A220`, `0x8042A1C8`, `0x8042C1A4`, `0x8082F078`,
  `0x8042A4A8` and `0x80429D60` keep their raw addresses; only `0x8082F078`
  (panel manager), `0x80429D60` (sound manager) and `0x8042A4A8` (sequence
  manager) have a proven role, and they are named as such.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (29 rows, 6,288 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 82 translated
  addresses were re-read first and none is duplicated here. The file now holds
  2,827 bodies.
- `node tools/check-lane-csv.mjs wave113` reports `OK` and names no problem in
  `wave113-bases.csv`.
- `cargo check --message-format=short` reports no error or warning naming
  `src/game/bases.rs`; diagnostics raised by other lanes' files are ignored.
