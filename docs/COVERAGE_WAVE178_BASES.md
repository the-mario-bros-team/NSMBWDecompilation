# Coverage - Wave 178 (`game::bases` 456-492 byte sub-band)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 20 | 9,448 | 17 | 3 |

Seventh five-lane wave on the paid model; 20 bodies in the 456-492 byte
sub-band (the 456-480 band alone is thin, so 480-492 was folded in).

## Lane A (4 bodies, 1,840 bytes)

- `0x80610510` - `dDateFile_c` pad refresh; `0x802DCD6C` is a save thunk, so
  `r3` is the screen.
- `0x8061DC70` - window message writer; row table ids 1..20 dumped from the
  REL. Ids > 20 leave the marker uninitialised in the original (`unaff_r28`),
  modelled as `0` and documented.
- `0x8070FE40` - witchcraft-emitter transform; channel 2 = `+0x1610 * 127`,
  channel 3 = `[0x8082F770] * 255` (`0x8082F770` is zero-fill BSS).
- `0x807ABF50` - pane rect/scale init; word-for-word `fn_807b08b0` with every
  offset shifted `+0x38`.

## Lane B (4 bodies, 1,848 bytes)

- `0x807ADC00` / `0x807AEEA0` - pane init pair, the merged `fn_807b08b0` body
  shifted by one word and one constant (`6000.0` vs `10000.0`).
- `0x80731380` - `houdai` gun-part dispatcher with the `mulli/srawi/addze`
  truncation idiom.
- `0x80758880` - last member of the wave-132 `Wm_mr_` cluster. **medium**: the
  multiplier is loaded into `f30` only in the `+0x23C == 2/3` arms but used
  unconditionally, so the raw stream would read an uninitialised non-volatile
  `f30`.

## Lane C (4 bodies, 1,876 bytes)

- `0x8078AF50` - scripted-screen command handler, sibling of `fn_807886e0`.
- `0x80774880` - castle loader over the 11-entry `0x807CF3C0` table; direct
  predecessor of `fn_80774a60`.
- `0x8066E0C0` - key-gate wall driver; the whole GQR block is `f28..f31` saves.
- `0x806DFA60` - lift seat recount into `+0x668..+0x674`.

## Lane D (4 bodies, 1,932 bytes)

- `0x80639000`, `0x80630BF0`, `0x80721800`, `0x807BBA40` - the 480-484 band,
  including the `fn_80638960` neighbour.

## Lane E (4 bodies, 1,952 bytes)

- `0x807BC7E0` - world-select carousel step.
- `0x806952D0` - sand-pillar smoke ring. **medium**: the `rlwinm rc` + `bc 4,2`
  ground test is *set*, not clear (Ghidra inverted it); the `cror 2,1,2` folds
  are `>=` tests.
- `0x807BF930` - `cobKoopaShip` ride state machine; the export swaps the two
  `0x8015F140` tuples.
- `0x80637120` - one bubble pass over the four `0x3C` records at `+0x7BC`; base
  is `this + 0x7BC + 0x3C*index`, not the export's `3 - iVar4`.

## Corrections and cross-checks

- `0x806952D0`'s ground-bit test is inverted in the export; its only argument is
  `this` (`_savegpr_26` prologue).
- `0x80637120` takes `(this, index)` and swaps fields individually rather than
  block-moving.
- `0x8061DC70`'s 20-entry row table is returned as `0` for out-of-range ids,
  matching the original's uninitialised marker.
- `0x807ABF50`, `0x807ADC00` and `0x807AEEA0` are all offset variants of the
  already-merged `fn_807b08b0`, which is a strong consistency check on that
  family.

## Verification

- `cargo check` succeeds with 4,307 public bodies and no duplicate names.
- `node tools/check-lane-csv.mjs wave178`: 20 rows, 20 unique addresses,
  17 high + 3 medium.
- All 20 evidence exports exist and every size matches `functions.tsv`.
