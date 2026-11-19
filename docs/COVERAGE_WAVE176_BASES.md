# Coverage - Wave 176 (`game::bases` 412-432 byte sub-band)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 24 | 10,132 | 18 | 6 |

Fifth five-lane wave on the paid model; 24 bodies in the 412-432 byte
sub-band.

## Lane A (5 bodies, 2,080 bytes)

- `0x80629240` - board cursor repeat/debounce (`+0x14` nibble vs `+0xAB8`).
- `0x80693F00` - two-kind dust spawner from the `0x8082B1C8` jitter table.
- `0x80701390` - four-slot player pickup scan. **medium**: receiver recovered
  from `_savegpr_23`, and `0x80051E70` is called with a live `r3` handle its
  export does not show.
- `0x8071D1A0` - `+0x1A4C0`/`+0x1A500` part transform rebuild.
- `0x8073F440` - three-state `0x8015F140` fade driver, cue `0x30B`.

## Lane B (5 bodies, 2,092 bytes)

- `0x8079F5F0` - course-entry hand-over (`"F7C0"`/`"W7C0"`, `0x80C71158`).
- `0x807C0600` - stick-driven aim pair (`11.0`/`4.0`, `10.0`).
- `0x80683200` - pendulum pivot frame, sibling of `fn_80680fd0`.
- `0x806E5370` - `lift_zen_kaihei` constructor (`48.0`, `544.0 + 16.0*index`).
- `0x807583D0` - remaining 420-byte body of the band.

## Lane C (5 bodies, 2,108 bytes)

- `0x80762970` - `cobDokanRoute` loader.
- `0x80774DF0` - scene-list armed scan over `+0x3390` blocks. **medium**.
- `0x807980F0` - course-actor placement with three `0x807D0198` masks and the
  `+0x29C` ability queries. **medium**.
- `0x80680CD0` - 8x`0x74` chains; four `0x3734` levels from `+0x3732` through
  `0x80237BD0`. **medium**.
- `0x80728CC0` - `remo_lift_soroban` bind.

## Lane D (5 bodies, 2,132 bytes)

- `0x8063F120` - `dSelectPlayer_c` four-slot highlight; the `bc 12,6` tests a
  dead `cr1`, so the `+0x27C` store is unconditional.
- `0x80733F70` - three-phase highlight fade into the `+0x620` ring.
- `0x80746C20` - world-slot reservation quantised to `1/32` units.
- `0x8078C500` - `bros` constructor, twin of `fn_80790840`.
- `0x8078D560` - per-variant palm loader, twin of `fn_80779ca0`.

## Lane E (4 bodies, 1,720 bytes)

- `0x80694F30` - sand-pillar contact registration; the asymmetric
  `cror 2,1,2` vs `cror 2,0,2` folds make both Ghidra `<=` orderings right.
- `0x807BE4C0` - player constructor (vtable `0x808218C8`, `+/-100000.0`).
- `0x8079E030` - player placement; `func_0x802dcd74()` is `_savegpr_26`, so
  `param_1..3` are `r3..r5`, and the table is `0x8031D39C`.
- `0x807A02C0` - scene-node aim refresh. **medium** (dead delta store dropped;
  `0x801611C0` read as `(dx,dz)` per siblings).

## Corrections and cross-checks

- `0x80746C20`'s `F63_8/12/15` words are opcode-63 `fabs`/`frsp`/`fctiwz`,
  not paired-single maths; `0x807C_E470` is `2^52+2^31`.
- `0x80701390` dereferences `[0x8042A078]` before masking, matching Wave 175's
  note but differing from `fn_80703840`'s direct read.
- Several exports again hide `param_1` behind `_savegpr` thunks
  (`0x802DCD60`, `0x802DCD74`).
- Lane C's merged doc comments carry `rel_d_basesNP` (underscore) in the
  evidence line, matching 15 pre-existing entries; the lane CSV uses the
  canonical `rel-d_basesNP`.

## Verification

- `cargo check` succeeds with 4,266 public bodies and no duplicate names.
- `node tools/check-lane-csv.mjs wave176`: 24 rows, 24 unique addresses,
  18 high + 6 medium.
- All 24 evidence exports exist and every size matches `functions.tsv`.
