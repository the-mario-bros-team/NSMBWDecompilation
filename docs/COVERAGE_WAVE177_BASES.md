# Coverage - Wave 177 (`game::bases` 432-456 byte sub-band)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 21 | 9,404 | 17 | 4 |

Sixth five-lane wave on the paid model; 21 bodies in the 432-456 byte
sub-band.

## Lane A (5 bodies, 2,192 bytes)

- `0x806F2E50` - `0xBEC0` subsystem allocator with five `0x802DC9A0` arrays
  (`+0x1F4`, `+0xBF4`, `+0x4074`, `+0x69B4`, `+0x92F4`).
- `0x806FE860` - four-state gate, cues `0x5F2`/`0x5F3`/`0x5DA`/`0x5DB`.
- `0x806E2950` - four `lift_suisha` planks classified via `fn_806e27d0`.
- `0x8075B040` - `cobAntlion` loader.
- `0x8079B3F0` - anchor/spawn body; transform words live in `main.dol`
  (`0x8031D3A0/A8/AC`).

## Lane B (4 bodies, 1,776 bytes)

- `0x807A28B0` - firepillar burst, `±0.0075` bounce, `0.18` window.
- `0x80628450` - five transform words from `manager+0x24C[i]` into `+0x244`.
- `0x807A3D00` - `g3d/I_yoshi_egg.brres` constructor.
- `0x8062C0B0` - pane `+0xBB` bit 0 per present player; the
  `(present-1)*16` is masked with `0xFFFFFFF0`, so a zero count wraps to the
  `0x807C7A98` row.

## Lane C (4 bodies, 1,800 bytes)

- `0x8075BB60` - layer-class id collector called by `fn_8075bd20`; the body
  reserves 22 words for up to 192 candidates, so the translation widens the
  buffer (identical for <= 22 matches).
- `0x80796850` - state dispatcher on `+0x27C` (jump table `0x8081B7D4`).
- `0x80651A60` - pops the 40-entry effect queue at `0x80828430` and compacts
  the `5x7` table.
- `0x806CF400` - `body`/`tire_R`/`tire_L` node offsets via
  `-90 * s16 / 16384`.

## Lane D (4 bodies, 1,812 bytes)

- `0x807946D0` - `cobTsuta` pose rebuild.
- `0x8079E310` - position snapshot + effect spawn; Ghidra's paired-single
  reconstruction was wrong (the scaled vectors are dead stores). **medium**.
- `0x807B90F0` - scene shutdown with the second display-list pass.
- `0x80648AA0` - pitch-ramp step, cues `0x305`/`0x306`.

## Lane E (4 bodies, 1,824 bytes)

- `0x806B4BA0` - launch reaction, sibling of `fn_806c3790` without the
  `+0x898` ground step. **medium** (dead two-vector block omitted).
- `0x806CFB30` - `+0x3E0` child-chain scanner; the export's `void` ate both
  arguments.
- `0x807630C0` - five-state appear/wait/fade machine. **medium**.
- `0x807B7D60` - alternate arm of `fn_807b7f30`; the template copy is
  field-level, so `+0x25..+0x27` must not be block-moved.

## Corrections and cross-checks

- `bc 4,1` branches when CR0[GT] is clear (`<=`), and `bc 4,0` is `>=`; this
  fixed Ghidra's inverted `fcmpo` arms in `0x807A28B0` and reconciles
  `0x8076EAD4`.
- `0x807630C0`'s case-3 constant was one pool entry off: `0x807CEE5C` is `0.0`,
  not `1.0`.
- `0x806CF54C`/`0x806CF554` need the scratch decoder's reversed `extsh` field
  order; the consumer is the sign-extended halfword.
- `0x8075BD20` now calls the real `fn_8075bb60` instead of `abi_call5!`.
- `0x806F2E50`'s `+0x158..+0x1F4` clear is GCC loop-versioned, emitted as an
  equivalent `u32` fill.

## Verification

- `cargo check` succeeds with 4,287 public bodies and no duplicate names.
- `node tools/check-lane-csv.mjs wave177`: 21 rows, 21 unique addresses,
  17 high + 4 medium.
- All 21 evidence exports exist and every size matches `functions.tsv`.
