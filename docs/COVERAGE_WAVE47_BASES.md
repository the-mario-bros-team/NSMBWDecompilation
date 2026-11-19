# Coverage - Wave 47 (bases)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 4200 | 19 | 11 |

## What this pass covers

- The `0x80767D60..0x80769600` Killer block in `d_basesNP`: 30 functions in
  two cooperating objects. The projectile is the `cobKiller` / `cobRotary`
  actor selected by the halfword at `+4 >> 16`; the launcher owns exactly one
  projectile pointer at `+0x1B4` and installs vtable `0x808161C0`.
- Projectile model and transform: the two-variant resource loader
  (`cobKiller`/`cobKillerShot` and `cobRotary`/`cobRotaryShot`) [`fn_80767e20`],
  the matrix/angle helper [`fn_807680a0`], the spawn-anchor computation
  [`fn_807681e0`], the keyed scene lookup [`fn_807682b0`] and the ten-entry
  detonation array [`fn_80768010`] with its per-slot queries and writes
  ([`fn_80768350`], [`fn_807683a0`], [`fn_807683c0`]).
- Projectile lifecycle: the initialiser [`fn_80767d60`], input arming
  [`fn_80768150`], the eligibility tests [`fn_80768590`] / [`fn_80768680`]
  and the burst (`smoke` + sound `99`) [`fn_807683e0`].
- Launcher lifecycle: constructor [`fn_80768800`], variant placement
  [`fn_80768a80`], `g3d_killer.brres` + `character_SV` loader
  [`fn_80768d70`], scaled transform [`fn_80768e40`], state reset
  [`fn_80768f50`], re-arm [`fn_80768fa0`], the fire transition
  [`fn_80768ff0`], the countdown hand-off [`fn_80769040`], the `skl_root`
  arming effect [`fn_80769170`], the fusing update [`fn_807691e0`], the
  detonation start [`fn_80769290`], the projectile lookup
  [`fn_807694b0`], the player-aim copy [`fn_80769520`]/[`fn_80769590`],
  the eligibility forwarder [`fn_807695f0`] and the variant-table getter
  [`fn_80769600`].

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80767D60` | `fn_80767d60` | 168 | high |
| `0x80767E20` | `fn_80767e20` | 492 | medium |
| `0x80768010` | `fn_80768010` | 140 | high |
| `0x807680A0` | `fn_807680a0` | 176 | medium |
| `0x80768150` | `fn_80768150` | 136 | high |
| `0x807681E0` | `fn_807681e0` | 196 | high |
| `0x807682B0` | `fn_807682b0` | 148 | medium |
| `0x80768350` | `fn_80768350` | 68 | high |
| `0x807683A0` | `fn_807683a0` | 32 | high |
| `0x807683C0` | `fn_807683c0` | 28 | high |
| `0x807683E0` | `fn_807683e0` | 132 | high |
| `0x80768590` | `fn_80768590` | 228 | medium |
| `0x80768680` | `fn_80768680` | 156 | high |
| `0x80768800` | `fn_80768800` | 84 | high |
| `0x80768A80` | `fn_80768a80` | 284 | medium |
| `0x80768CF0` | `fn_80768cf0` | 108 | high |
| `0x80768D70` | `fn_80768d70` | 196 | high |
| `0x80768E40` | `fn_80768e40` | 268 | medium |
| `0x80768F50` | `fn_80768f50` | 80 | high |
| `0x80768FA0` | `fn_80768fa0` | 72 | high |
| `0x80768FF0` | `fn_80768ff0` | 12 | medium |
| `0x80769040` | `fn_80769040` | 156 | medium |
| `0x80769170` | `fn_80769170` | 112 | high |
| `0x807691E0` | `fn_807691e0` | 164 | medium |
| `0x80769290` | `fn_80769290` | 220 | medium |
| `0x807694B0` | `fn_807694b0` | 108 | high |
| `0x80769520` | `fn_80769520` | 100 | medium |
| `0x80769590` | `fn_80769590` | 92 | high |
| `0x807695F0` | `fn_807695f0` | 12 | high |
| `0x80769600` | `fn_80769600` | 32 | high |

## Deliberate skips

- `0x80768470` (276 bytes): the export contains paired-single/GQR artefacts
  (`unaff_GQR0`, `ldexpf` calls), so the literal float comparisons cannot be
  reconstructed honestly. It sits between [`fn_807683e0`] and
  [`fn_80768590`] and leaves the only hole in the range.
- `0x80769620` and beyond (for example `0x80769640`, `0x807696D0`,
  `0x807699D0`, `0x80769F00`): outside this pass. They are still called by raw
  address where the translated bodies need them (`0x807699D0`, `0x80769F00`),
  and the calls are marked in the function docs.

## Verification

- Every address and byte size matches `functions.tsv` (203 entries in the
  `0x80760000` bucket; 30 translated here).
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`.
- No address was already in `ledger/functions.csv` (the only earlier entries
  in the bucket are `0x80764270` and `0x80765C10`).
- `cargo check --message-format=short` reports no error for
  `src/game/bases.rs` (remaining warnings are in other lanes' files).
