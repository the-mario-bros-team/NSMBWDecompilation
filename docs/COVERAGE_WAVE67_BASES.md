# Coverage - Wave 67 (`game::bases`, `0x8070` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** from the canonical synthetic
base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime address: the
Wii REL loader picks the real load address. Byte counts come from
`work/ghidra/rel-d_basesNP/export/functions.tsv`; the cited evidence files are the
Ghidra C exports under `work/ghidra/rel-d_basesNP/export/`. Confidence is `high`
when the guards, stores, constants and call arguments are literal, and `medium`
when a field role, a callee identity or an argument dropped by Ghidra had to be
inferred (the body then carries `// observed:`, `// inferred:` or
`// unknown:` notes).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 4248 | 14 | 16 |

## What this pass covers

- **Boat-light actor tail, `0x80701C60..0x807053C0`** — the remaining bodies of
  the `0xE18`-byte boat-light actor whose helper tail wave 50 started
  (`0x807001C0..0x80702520`). This pass adds:
  - the light position writer [`fn_80701c60`] and the colour-record clear
    [`fn_80702320`];
  - the beam-matrix rebuild [`fn_80702970`] and the `Wm_ob_boat` effect
    [`fn_80702a30`];
  - the five-light release [`fn_80702d90`] with the sixteen-record
    bind/claim pair [`fn_80702e40`] / [`fn_80702ea0`];
  - the six-record player-switch array [`fn_80702f40`], [`fn_807031e0`],
    [`fn_80703230`] and [`fn_80703290`];
  - the current-light fade selector [`fn_807032e0`] and the three bone-gated
    palette writes [`fn_80703390`];
  - the `light_fog_st` fog create / release / tick triple
    [`fn_80703460`] / [`fn_80703540`] / [`fn_80703640`];
  - the two-mode player machine [`fn_80703750`] / [`fn_80703840`];
  - the two-beam placement and release [`fn_80703b40`] / [`fn_80703c00`];
  - the four-record sound array [`fn_80703c50`] / [`fn_80703ca0`] /
    [`fn_80703d70`];
  - the actor tail: bone-translation writer [`fn_80704130`], full destructor
    [`fn_80704860`], two registered destructors [`fn_80704ce0`] /
    [`fn_807053c0`], the `0x3F0`-byte constructor [`fn_80704e60`] and the
    counter-gated effect [`fn_80705080`].

- **Recurring layout.** The new bodies use the layout recorded in the wave-50
  section of the module header: two `0xE8`-byte beams at `+0x574`, five
  `0x90`-byte lights at `+0x744`, the palette at `+0xA14` with sixteen
  three-byte colour records at `+0xA2C`, six `0x5C`-byte switch records at
  `+0xB84` and four two-byte sound records at `+0xDAC`. The `+0x478` model,
  `+0x4E4`/`+0x510` fog members and `+0xA5C` effect manager stay
  `// unknown:` because their NW4R classes are not in the REL.

- **Constants re-read from `work/rels/d_basesNP.rel`** as big-endian words:
  `0x807CC530` = `0.0f`, `0x807CC594` = `0.0f`, `0x807CC5B0` = `100.0f`,
  `0x807CC5B4` = `80.0f`, `0x807CC5CC` = `105.0f`, `0x807CC5D0` = `8.0f`,
  `0x807CC5D4` = `25.0f`, `0x807CC5E0` = `-105.0f`, `0x807CC5E4` = `42.0f`,
  `0x807CC5E8` = `0.00390625f`. The `0x8042xxxx` words (`+0x9D19`, `+0x9D20`,
  `+0x9D24`, `+0x9D28`, `+0x9D60`, `+0x9DD0`, `+0x9FB0`, `+0xA078`,
  `+0xA22A`) live in `main.dol` and are read through `platform::read_*`
  instead of being restated. `0x807CC5D8` is the high word of the PowerPC
  integer-to-double bias `0x4330000000000000`, so the body computes
  `counter + 1` directly.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80701C60` | `fn_80701c60` | 44 | high |
| `0x80702320` | `fn_80702320` | 16 | high |
| `0x80702970` | `fn_80702970` | 180 | medium |
| `0x80702A30` | `fn_80702a30` | 104 | medium |
| `0x80702D90` | `fn_80702d90` | 84 | high |
| `0x80702DF0` | `fn_80702df0` | 72 | high |
| `0x80702E40` | `fn_80702e40` | 88 | high |
| `0x80702EA0` | `fn_80702ea0` | 160 | high |
| `0x80702F40` | `fn_80702f40` | 136 | medium |
| `0x807031E0` | `fn_807031e0` | 72 | medium |
| `0x80703230` | `fn_80703230` | 84 | high |
| `0x80703290` | `fn_80703290` | 72 | medium |
| `0x807032E0` | `fn_807032e0` | 176 | medium |
| `0x80703390` | `fn_80703390` | 196 | medium |
| `0x80703460` | `fn_80703460` | 220 | medium |
| `0x80703540` | `fn_80703540` | 252 | medium |
| `0x80703640` | `fn_80703640` | 268 | medium |
| `0x80703750` | `fn_80703750` | 228 | medium |
| `0x80703840` | `fn_80703840` | 256 | medium |
| `0x80703B40` | `fn_80703b40` | 180 | high |
| `0x80703C00` | `fn_80703c00` | 72 | high |
| `0x80703C50` | `fn_80703c50` | 72 | high |
| `0x80703CA0` | `fn_80703ca0` | 208 | high |
| `0x80703D70` | `fn_80703d70` | 80 | high |
| `0x80704130` | `fn_80704130` | 128 | medium |
| `0x80704860` | `fn_80704860` | 316 | medium |
| `0x80704CE0` | `fn_80704ce0` | 88 | high |
| `0x80704E60` | `fn_80704e60` | 76 | medium |
| `0x80705080` | `fn_80705080` | 232 | medium |
| `0x807053C0` | `fn_807053c0` | 88 | high |

## Address range

`0x80701C60..0x807053C0`, in two runs:

- `0x80701C60..0x80703D70` — the boat-light helper tail, interrupted by the
  deferred `0x80702AA0`, `0x80702BC0`, `0x80702CE0`, `0x80702FD0`,
  `0x80703940`, `0x80703A40` and `0x80703DC0`, and by the already-translated
  wave-50 rows at `0x807020C0..0x80702520`;
- `0x80704130..0x807053C0` — the actor destructor/constructor tail, interrupted
  by the deferred `0x80704050`, `0x807040C0`, `0x80704630`, `0x807046B0`,
  `0x80704710`, `0x80704E00`, `0x80704E30` and `0x80705510`.

At lane start the `0x8070` bucket held 204 functions, 41 of them already
translated (the wave-50 `0x807001C0..0x80702520` run plus `0x80702750`,
`0x80705540`, `0x80705710`, `0x80705FB0`, `0x80709FC0`, `0x8070B0A0`,
`0x8070B7F0`, `0x8070F270` and `0x8070F630`). This pass adds 30, leaving 133 for
later waves.

## Deliberate skips

- `0x80702AA0` (276), `0x80702BC0` (276), `0x80702FD0` (516), `0x80703DC0`
  (648), `0x80704050` (108), `0x807040C0` (112) — the outlined paired-single
  block (`unaff_GQR0`/`unaff_GQR3`, `ldexpf`, the `bVar == 4 || bVar == 6` lane
  split, incoming `in_f31`/`in_ps31_1`). Same decompiler failure class recorded
  for `0x806914D0`, `0x806F0F20` and `0x80740370`: the conversion inputs are
  invented, so no honest body is possible.
- `0x80702CE0` (168) — its loop body and constant are clear, but the table base
  `0x807CC57C` and the converted `DAT_807CC5B0` come through the same
  paired-single path and the record indices are only recoverable from raw
  bytes. Deferred rather than guessed.
- `0x80703940` (248), `0x80703A40` (252) — the export passes its return value
  through the `0x802DCD68` register-save thunk (`*(int *)uVar9`) and
  `0x80703A40` reads the uninitialised stack byte `uStack_15` after the call.
  The calling convention needs pinning down before an honest body exists.
- `0x80704E00` (48), `0x80704E30` (48), `0x80705510` (48) — the external
  jump stub `0x802DCBFC` (`func_0x802dcbfc(param_2, param_2, param_1)`), the
  same rejection recorded in earlier waves for unresolved indirect-jump stubs.
- `0x80704630` (96), `0x807046B0` (64), `0x80704710` (96) — they wrap the
  deferred `0x80703DC0` and would be unreconstructable without it.

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
  `0x8070` bucket before writing).
- `node tools/check-lane-csv.mjs wave67` reports `OK` with
  `wave67-bases.csv: 30 rows`.
- `cargo check --message-format=short` produces no error for
  `src/game/bases.rs`; the remaining warnings belong to other lanes' files.
