# Coverage - Wave 56 (`game::bases`, `0x8077` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL (synthetic base `0x80600000`; every address is a static analysis address,
not a runtime address). Byte counts come from
`work/ghidra/rel-d_basesNP/export/functions.tsv`; the quoted evidence files are
the Ghidra C exports under `work/ghidra/rel-d_basesNP/export/`. Confidence is
`high` when guards, stores, vtables and call arguments are literal, and `medium`
when a callee role, a dropped argument or a table's meaning had to be inferred.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 26 | 3712 | 20 | 6 |

## What this pass covers

- The unnamed `0x5C7F4`-byte manager at [`fn_80771540`]: base constructor
  `0x800F5FD0`, vtable `0x808172A8` at `+0x60`, wrapper at `+0x184`, and three
  inline arrays built by `0x802DC9A0` at `+0x1A0` (stride `0xBF8`, 4 entries),
  `+0x3394` (stride `0x16518`, 4 entries) and `+0x5C7F4` (stride `0x3F08`,
  4 entries).
- That manager's array element lifecycle: the `0xBF8`-byte record constructor
  [`fn_807715f0`] (vtable `0x8031BBB8`, three NW4R sub-objects, a
  `0x48`-stride × 16 pool at `+0xC8` and the table at `+0x7DC`), its inner-slot
  constructor [`fn_807716f0`], its destructor [`fn_80771740`] and the inner-slot
  destructor [`fn_807717a0`]; the `0x3F08`-byte record constructor
  [`fn_80771800`] (vtable `0x80817318`, `0x54`-stride × `0xC0` pool at `+0x04`),
  its slot destructor [`fn_80771870`] and its full destructor [`fn_807718b0`].
- The three collision/index queries that use the `+0x338C` selector and the
  `0x16518` / `0x3F08` arrays: the player/trigger gate [`fn_80772180`]
  (`0x80429F7C + 0x54` player id and the `0x800F9840`/`0x800FD310`/
  `0x800FAE80`/`0x800FAF70` predicates), the boolean lookup [`fn_80772280`]
  and the result-discarding lookup [`fn_80772300`].
- The `peach` course actor: the model loader [`fn_807769f0`]
  (`g3d/peach.brres`, the two `Fly_wmap_B1` panes at `+0x1E4`, colour bytes
  `0x807CF4B8`, clip time `0x807CF4C0`), the transform writer
  [`fn_80776b50`], the constructor [`fn_80776df0`] (vtable `0x808176C0`,
  3-entry `0x38`-stride pane pool at `+0x1E8`), the start-up
  [`fn_80776f60`] and the per-frame body [`fn_807770b0`] with its flag clear
  [`fn_80777130`].
- The `cobPeachCastle` course actor: the model loader [`fn_80777150`]
  (`cobPeachCastle`, `g3d/model.brres`, the three `cobPeachCastleFlag` panes
  at `+0x1E8`, colour bytes `0x807CF4D4`, clip time `0x807CF4E0`), the
  transform writer [`fn_807772a0`], the door/flag event handler
  [`fn_80777350`] (events `0x11`/`0x13`/`0x4E`/`0x14`, sound cues `0x31`/`0x32`,
  done flag `+0x139`), the `FlagA0` pane rebuild [`fn_807774d0`], the two
  `LeftDoor` pane rebuilds [`fn_80777550`] / [`fn_807775e0`] and the gate
  opener [`fn_80777670`].

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x807714F0` | `fn_807714f0` | 12 | medium |
| `0x80771540` | `fn_80771540` | 172 | high |
| `0x807715F0` | `fn_807715f0` | 244 | high |
| `0x807716F0` | `fn_807716f0` | 76 | high |
| `0x80771740` | `fn_80771740` | 96 | high |
| `0x807717A0` | `fn_807717a0` | 64 | high |
| `0x80771800` | `fn_80771800` | 88 | high |
| `0x80771870` | `fn_80771870` | 64 | high |
| `0x807718B0` | `fn_807718b0` | 104 | high |
| `0x80772180` | `fn_80772180` | 248 | medium |
| `0x80772280` | `fn_80772280` | 124 | medium |
| `0x80772300` | `fn_80772300` | 112 | high |
| `0x807769F0` | `fn_807769f0` | 340 | high |
| `0x80776B50` | `fn_80776b50` | 176 | high |
| `0x80776C10` | `fn_80776c10` | 12 | high |
| `0x80776DF0` | `fn_80776df0` | 172 | high |
| `0x80776F60` | `fn_80776f60` | 100 | high |
| `0x807770B0` | `fn_807770b0` | 124 | medium |
| `0x80777130` | `fn_80777130` | 12 | high |
| `0x80777150` | `fn_80777150` | 336 | high |
| `0x807772A0` | `fn_807772a0` | 176 | high |
| `0x80777350` | `fn_80777350` | 380 | medium |
| `0x807774D0` | `fn_807774d0` | 120 | high |
| `0x80777550` | `fn_80777550` | 136 | high |
| `0x807775E0` | `fn_807775e0` | 136 | high |
| `0x80777670` | `fn_80777670` | 88 | medium |

## Address range

`0x807714F0..0x80777670` (two clusters: the manager/array block
`0x807714F0..0x80772300` and the Peach-castle block
`0x807769F0..0x80777670`).

## Deliberate skips

- `0x807702a0` (892), `0x807706b0` (384) and `0x80772370` (1108) are the
  manager's large state machine and four nested span scans. They are
  translatable but dominated by unprototyped callees in the `0x800F5xxx`
  family whose argument lists are half-recovered, so they were left for a
  later wave rather than guessed.
- `0x80776c80` (112) takes three `undefined8` (`f64`-class) parameters and
  twice passes them through to `0x801680E0`/`0x80168FE0`, which are `f32`
  functions at every other call site. The ABI split cannot be recovered from
  the export alone, so it was skipped.
- `0x807704xx`..`0x80770d90` (`0x807700f0`, `0x807701c0`, `0x807702a0`,
  `0x807706b0`) are the ship-propeller state bodies with paired-single
  arithmetic; only the self-contained bodies in this pass were taken.
- The 4-byte thunks `0x80773200` (to `0x80773210`) and `0x80776c00` (to
  `0x80776c10`) add no evidence and were not counted.
- `0x807777b0`, `0x80778c20`, `0x80779380`, `0x807796c0` and `0x80779820`
  were already in `ledger/functions.csv` and were not duplicated.
- The rest of the `0x8077` bucket (151 untranslated at lane end, including the
  `cobSandpillar`, `cobStart`/`cobStop`, `cobKinokoRed`, `cobKiller` and
  `character_SV` clusters above `0x80777E00`) is outside this window.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv`.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000` line.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/bases.rs`.
- `node tools/check-lane-csv.mjs wave56` reports `OK` for
  `wave56-bases.csv`.
