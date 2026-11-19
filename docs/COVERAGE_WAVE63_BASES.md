# Coverage - Wave 63 (`game::bases`, `0x806F` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address here is a **static analysis address** from the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader picks the real load address. Byte counts come from
`work/ghidra/rel-d_basesNP/export/functions.tsv`; the cited evidence files are
the Ghidra C exports under `work/ghidra/rel-d_basesNP/export/`. Confidence is
`high` when the guards, stores, call arguments and constants are literal and
`medium` when a callee identity, field role or lost argument had to be inferred
(the body then carries `// inferred:` / `// unknown:`).

Float constants quoted in the bodies were re-read from the raw word in
`work/rels/elf/d_basesNP.elf` (and `work/dol/main.elf` for imported constants)
instead of trusting the decompiler's decimal print, which is how the `0.6`,
`2.0`, `128.0`, `48`/`32`, `1500`/`-1800` and `80` values below were confirmed.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 28 | 2996 | 14 | 14 |

## What this pass covers

- **`manjirou` lift tail, `0x806F0BC0..0x806F1560`** — the class whose
  constructor `0x806EFEF0`, model loader [`fn_806f03e0`] and transform writer
  [`fn_806f0c10`] were recorded in waves 29 and 53. New bodies: the two-record
  motion reset [`fn_806f0bc0`], the four-point aim writer [`fn_806f0d00`], the
  256-entry rolling-wheel angle history [`fn_806f1110`] / [`fn_806f1140`] /
  [`fn_806f1170`] (distance divided by the `0.6f` circumference factor at
  `0x807CBFF0`), the two-state driver [`fn_806f1340`] (velocity integrate,
  `+0x728` countdown, height zero-crossing event `0x8082E8F0`), the exit
  countdown [`fn_806f1480`] (event `0x8082E930`) and the settle state
  [`fn_806f1560`] (blink byte `+0x964`, eight contact bytes at `+0x952`).
- **Respawn spawner, `0x806F1D70..0x806F1FD0`** — a class whose `+0x398` count
  indexes 16-byte `[x, y, z, flags]` records at `+0x39C`. Bodies: the start
  [`fn_806f1d70`] (collision plane `0x8042BB0C` and `0x8042BB08 + 128.0`, timer
  from the spawn flags, `0x806F2100` collection, spawn or kill), the per-frame
  driver [`fn_806f1e10`], the respawn countdown [`fn_806f1ea0`], the list-wide
  release [`fn_806f1ef0`], the owner-matched release [`fn_806f1f40`] and the
  spawn loop [`fn_806f1fd0`] (actor id `0x20F`, `96x64` collision box).
- **Rider-timer object, `0x806F2460..0x806F2A90`** — the start
  [`fn_806f2460`], the global "any rider has `+0x958`" query [`fn_806f2920`],
  the two countdowns [`fn_806f2990`] / [`fn_806f2a30`] with events
  `0x8082E9F0` / `0x8082E9B0`, and the destructor [`fn_806f2a90`] that tears
  down the `0x40`-element `0x10`-stride array at `+0x3DC`.
- **`0x768`-byte screen, `0x806F5220..0x806F5330`** — allocation
  [`fn_806f5220`], derived allocation with vtable `0x80801598` [`fn_806f5250`],
  the constant record getter [`fn_806f52a0`], the base constructor
  [`fn_806f52b0`] (vtable `0x8080183C`, sub-objects `+0x524`/`+0x544`, state
  machine `+0x584` with `0xE4`-byte records) and the destructor
  [`fn_806f5330`].
- **`0x806F8A70..0x806F8EB0`** — a `0x398`-byte object with allocation
  [`fn_806f8a70`], constructor [`fn_806f8ad0`] (vtable `0x80802530`, global
  manager `0x164` bytes at `0x8082ED60`), a state reset [`fn_806f8dd0`] and the
  record counter [`fn_806f8eb0`].

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806F0BC0` | `fn_806f0bc0` | 80 | medium |
| `0x806F0D00` | `fn_806f0d00` | 176 | medium |
| `0x806F1110` | `fn_806f1110` | 36 | high |
| `0x806F1140` | `fn_806f1140` | 40 | high |
| `0x806F1170` | `fn_806f1170` | 152 | medium |
| `0x806F1340` | `fn_806f1340` | 268 | high |
| `0x806F1480` | `fn_806f1480` | 112 | medium |
| `0x806F1560` | `fn_806f1560` | 140 | medium |
| `0x806F1D70` | `fn_806f1d70` | 160 | medium |
| `0x806F1E10` | `fn_806f1e10` | 60 | medium |
| `0x806F1EA0` | `fn_806f1ea0` | 80 | high |
| `0x806F1EF0` | `fn_806f1ef0` | 76 | high |
| `0x806F1F40` | `fn_806f1f40` | 144 | medium |
| `0x806F1FD0` | `fn_806f1fd0` | 292 | medium |
| `0x806F2460` | `fn_806f2460` | 116 | medium |
| `0x806F2920` | `fn_806f2920` | 72 | high |
| `0x806F2990` | `fn_806f2990` | 128 | medium |
| `0x806F2A30` | `fn_806f2a30` | 96 | high |
| `0x806F2A90` | `fn_806f2a90` | 116 | high |
| `0x806F5220` | `fn_806f5220` | 48 | high |
| `0x806F5250` | `fn_806f5250` | 76 | medium |
| `0x806F52A0` | `fn_806f52a0` | 12 | medium |
| `0x806F52B0` | `fn_806f52b0` | 116 | high |
| `0x806F5330` | `fn_806f5330` | 140 | high |
| `0x806F8A70` | `fn_806f8a70` | 48 | high |
| `0x806F8AD0` | `fn_806f8ad0` | 88 | high |
| `0x806F8DD0` | `fn_806f8dd0` | 88 | medium |
| `0x806F8EB0` | `fn_806f8eb0` | 36 | high |

## Address range

`0x806F0BC0..0x806F8EB0`, four windows separated by the already-translated
members of the same classes (`0x806F0140`, `0x806F03E0`, `0x806F0C10`,
`0x806F1CB0`, `0x806F46A0`, `0x806F48A0`, `0x806F6A50`, `0x806F7690`,
`0x806F80D0`, `0x806F8AA0`, `0x806F8B90`).

## Deliberate skips

- `0x806F0DB0` (356 bytes) — the C starts with the outlined register-save
  prolog `0x802DCD5C` modelled as a 64-bit return; its low word is then written
  (`*(undefined4 *)uVar8 = 1`), which cannot be attributed to a real object
  field honestly.
- `0x806F0F20` (496) and `0x806F1210` (276) — both are dominated by
  paired-single `GQR`/`ldexpf` scaffolding (`in_f30`/`in_f31` inputs the
  decompiler invented) and `0x806F1210`'s divisor is built from the unsigned-int
  bias constant `0x807CC0C0` (`2^52 + 2^31`), so the printed `/ (float)dVar1`
  does not describe game speed. This is the same failure class already recorded
  for the tail of [`fn_806f0c10`].
- `0x806F2100` (520) — the screen-space collector that fills the `+0x39C`
  records. Its `int -> float` conversions and the `0x8008E770` position/player
  query are visible, but the `+0xD` flag byte of the level record and the
  paired-single scaffolding would force guesses in a 100-line body.
- Adjustor thunks `0x802DCBFC(p2, p2, p1)` — `0x806F1C50`, `0x806F1C80`,
  `0x806F2DF0`, `0x806F2E20`, `0x806F51C0`, `0x806F51F0`, `0x806F8A10`,
  `0x806F8A40`. They only re-enter the same stub and have no Rust-visible
  effect, matching the policy already documented in the module header.
- Large bodies left for later waves: `0x806F3350` (2092), `0x806F7BB0` (1004),
  `0x806FCFF0` (788), `0x806FD820` (912), `0x806FE480` (892), plus the
  `0x806F29xx`..`0x806F33xx` timer class members `0x806F2560`, `0x806F25B0`,
  `0x806F2710` and `0x806F2420` that the new bodies call but that are not
  themselves reconstructed here.

At lane start the `0x806F` bucket held 195 functions, 25 of them already
translated. This pass adds 28, leaving 142 for later waves.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (re-read, not copied from
  prose).
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000` line.
- No address was already in `ledger/functions.csv` (checked against the whole
  ledger before writing).
- `node tools/check-lane-csv.mjs wave63` reports `OK` and
  `wave63-bases.csv: 28 rows`.
- `cargo check --message-format=short` reports no error or warning for
  `src/game/bases.rs`.
