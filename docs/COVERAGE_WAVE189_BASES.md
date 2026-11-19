# Coverage - Wave 189 (`game::bases` 128-192 band, part 3)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 50 | 7,716 | 49 | 1 |

Third wave in the 128-192 band.

## Highlights

- **`Class:Name` comparator clones (4).** `0x80714370`, `0x807204B0`,
  `0x807227E0`, `0x80723B10`, `0x80743480` - the inlined
  `0x802E1C40`/`0x802E1AB4` helper; the module now carries ~16 copies of it.
- **Factory pairs.** `0x80717AE0`/`0x80717B80`/`0x80717CF0` (0x4A10 object with
  three arrays at `+0x0`, `+0x4A4`, `+0x207C`), and `0x8071BBB0`/`0x8071BC50`
  (0x4A18 object with a two-element `0x2290` array).
- **Lift/turi bodies.** `0x807189D0`, `0x80719D00`, `0x8071A230`,
  `0x8071B8E0` (`lift_turi_ashiba.brres`, L/M/R panes), `0x8071D340`,
  `0x8071ED20`, `0x8071EFB0`.
- **Effect/aim bodies.** `0x8072C330` (`±192`, clamp `±16384`), `0x8072FBA0`
  (watch-target record with `sqrt` distance), `0x80734170`, `0x80734340`.
- **Misc.** `0x80700320`, `0x80705890` (do/while line emitter), `0x80706360`,
  `0x80706440`, `0x80706980`, `0x80709EC0`, `0x807316F0`, `0x80733D90`,
  `0x8073B290`, `0x80745D10`, `0x80747120`, `0x80749C50`, `0x8074C990`,
  `0x80752B90`, `0x80756FD0`, `0x80758310`, `0x8075ADA0`, `0x8075BF70`.

## Corrections and recoveries

- `0x80706360`'s return is a dead accumulator in Ghidra; every arm is
  `li r3,0`, so it is a 0/1 membership test over twelve `0xC`-stride entries.
- `0x80705890` is a literal do/while (pre-loop `fctiwz`, per-iteration `-y`).
- `0x80709EC0`'s `0x807E2740` store deliberately overwrites the vtable that
  `0x802D7AA0` installs.
- `0x80756FD0`/`0x80758310` take arguments (`mr r27,r3` / `mr r28,r4`) that the
  `_savegpr` prologue hides.
- `0x80749C50`'s first `0x80749C00` call loses `r4`/`r5`, which still hold
  `param_2`/`param_3`.
- `0x80745D10`'s `ldexpf`/GQR blocks are noise; the real arguments are two
  `fctiwz` results masked to 16 bits.
- `0x8075BF70` is the teardown sibling of `fn_8075bd20` (two passes × two
  slots, name release with count `-1`).

## Verification

- `cargo check` succeeds with 4,768 public bodies and no duplicate names.
- `node tools/check-lane-csv.mjs wave189`: 50 rows, 50 unique addresses,
  49 high + 1 medium.
- All 50 evidence exports exist; 7,716 bytes total.
