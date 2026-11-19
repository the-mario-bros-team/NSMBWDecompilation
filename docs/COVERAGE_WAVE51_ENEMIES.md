# Coverage - Wave 51 (`d_enemiesNP`, enemies lane)

## How to read this file

This page records one lane of Wave 51: the head of the `0x80AD0000` bucket of
REL module 3, `d_enemiesNP`, promoted to `src/game/enemies.rs`. Every row
corresponds to a real `pub unsafe fn` body in that file; the `evidence` column
of the lane CSV names the Ghidra C export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these are runtime addresses.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 29 | 5,056 | 24 | 5 |

The five `medium` rows are `0x80AD0440`, `0x80AD0690`, `0x80AD0AB0`,
`0x80AD1240` and `0x80AD1920`. Each one has a literal instruction stream, but at
least one field role or callee identity is inferred; the body doc says which.

## What this pass covers

The export names no class in this bucket. The only class-identifying evidence is
the effect strings the block spawns - `Wm_en_quicksand` at `0x80AD0240`,
`Wm_en_landsmoke_s` / `Wm_en_sndlandsmk_s` at `0x80AD1010` and the `walkA` clip
at `0x80AD10A0`. `// inferred:` therefore this is a world-map enemy whose attack
is the quicksand effect. The block is one contiguous run, `0x80AD0070` through
`0x80AD1DF0`, and every function shares the same actor layout:

- direction byte `+0x348`, class vtable `+0x60`, embedded state controller
  `+0x394`, model matrix `+0x7C`, motion block `+0xDC`, ground probe `+0x1EC`
  and the two animation holder pairs `+0x544/+0x588` and `+0x7FC/+0x83C`;
- **retire / turn helpers, `0x80AD0070..0x80AD0920`.** The name-tested retire
  path, the quicksand-column probe (ground word bit `0x8000` plus ground code
  3), the `Wm_en_quicksand` spawn, the matrix rebuild from the `+0x100`
  rotation triple, the `0x80064A30` side turn through class slot `+0x30C`, the
  five-argument collision binder, the `0x80B763F0` state reversal, the
  `+0x304` angle step and the direction/angle getter. `0x80AD0920` snaps the
  height to the quicksand surface using the `0x8042BB20` variant table.
- **quicksand parameter block and state entry, `0x80AD0AB0..0x80AD10A0`.** The
  fifteen-word effect block with the `0x802DCA98` scroll callback, the
  "hit by quicksand" entry (`+0x8A4` latch and zeroed `+0xE8..+0xF0`), the
  target-distance query, the two animation binders, the forward-ground probe,
  the landing-smoke selector and the four-marker `walkA` entry.
- **walk and turn machine, `0x80AD1240..0x80AD1DF0`.** The main walk step
  (two distance thresholds, the signed-byte `2^52` conversion, the `+0x274`
  collision mask and two state changes), the holder reset, the two base
  per-frame steps, the `BGM_anim_walkA_1` entry, the two turn countdowns and the
  final state reversal.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AD0070` | `fn_80ad0070` | 148 | high |
| `0x80AD01B0` | `fn_80ad01b0` | 144 | high |
| `0x80AD0240` | `fn_80ad0240` | 124 | high |
| `0x80AD0440` | `fn_80ad0440` | 232 | medium |
| `0x80AD0690` | `fn_80ad0690` | 172 | medium |
| `0x80AD0740` | `fn_80ad0740` | 136 | high |
| `0x80AD07D0` | `fn_80ad07d0` | 144 | high |
| `0x80AD0860` | `fn_80ad0860` | 160 | high |
| `0x80AD0900` | `fn_80ad0900` | 32 | high |
| `0x80AD0920` | `fn_80ad0920` | 64 | high |
| `0x80AD0AB0` | `fn_80ad0ab0` | 448 | medium |
| `0x80AD0C90` | `fn_80ad0c90` | 260 | high |
| `0x80AD0E00` | `fn_80ad0e00` | 64 | high |
| `0x80AD0E40` | `fn_80ad0e40` | 112 | high |
| `0x80AD0EB0` | `fn_80ad0eb0` | 112 | high |
| `0x80AD0F20` | `fn_80ad0f20` | 232 | high |
| `0x80AD1010` | `fn_80ad1010` | 136 | high |
| `0x80AD10A0` | `fn_80ad10a0` | 388 | high |
| `0x80AD1240` | `fn_80ad1240` | 476 | medium |
| `0x80AD1490` | `fn_80ad1490` | 84 | high |
| `0x80AD16A0` | `fn_80ad16a0` | 180 | high |
| `0x80AD17A0` | `fn_80ad17a0` | 184 | high |
| `0x80AD1860` | `fn_80ad1860` | 80 | high |
| `0x80AD1920` | `fn_80ad1920` | 160 | medium |
| `0x80AD19C0` | `fn_80ad19c0` | 116 | high |
| `0x80AD1A50` | `fn_80ad1a50` | 196 | high |
| `0x80AD1B20` | `fn_80ad1b20` | 124 | high |
| `0x80AD1BB0` | `fn_80ad1bb0` | 220 | high |
| `0x80AD1DF0` | `fn_80ad1df0` | 128 | high |

## Deliberate skips

- **`0x80AD0960`.** The Ghidra body depends on uninitialised `GQR0` state
  (`ldexpf` of the graphics-quantiser fields) exactly like the `0x80AF5C60`
  step skipped in wave 48. The paired quantiser setup is not in the export, so
  the body cannot be expressed honestly in Rust.
- **`0x80AD0A90`.** The export is a truncated `halt_baddata()` block with no
  reachable instructions; no body can be reconstructed.
- **The rest of the `0x80AD0000` bucket.** 156 other functions remain for later
  passes, starting at `0x80AD2660`; none of the four earlier rows in this
  bucket (`0x80AD9E60`, `0x80ADA320`, `0x80ADFA90`, `0x80ADFDD0`) was reused.

## Verification

- Every address and byte size matches
  `work/ghidra/rel_d_enemiesNP/export/functions.tsv`; the 29 sizes sum to
  5,056 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error or warning for
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave51` prints `OK` for this lane's CSV.
