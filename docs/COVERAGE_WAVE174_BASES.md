# Coverage - Wave 174 (`game::bases` 368-388 byte sub-band)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 27 | 10,216 | 20 | 7 |

Third five-lane wave on the paid model; 27 bodies in the 368-388 byte
sub-band.

## Lane A (6 bodies, 2,216 bytes)

- `0x806377A0` - spawns `Wm_2d_result` for each empty `0x3C`-stride lane
  record; the `nor r0,r3,r0` clamp is real, not dead code.
- `0x8064DEF0` - cursor-grid outline array at `+0x410` and its four edges.
- `0x80731780` - rotating-dokan arm moved by `40.0 * cos`/`sin`; `0x807CD740`
  is `1/256`.
- `0x8075BD20` - two class passes of `0x8075BB60`, cue `0x28E`, up to four id
  labels. **medium** (`0x800BBDD0`/`0x800EF570`/`0x80100200` roles unknown).
- `0x8076D680` - `g3d/koopaJr.brres` + `character_SV`, six clips, clears bit 8.
- `0x80740C90` - shifts the four `+0x394` slot records, harvests `+0x4A0`.

## Lane B (6 bodies, 2,248 bytes)

- `0x8076ECF0` - `cobKoopaShip` prop constructor.
- `0x8060A950` - `Wm_2d_continue` spawn; the wave-86 deferral is resolved
  (the stack record is `0x3C` bytes, not `0x34`).
- `0x8078EEE0` - 1-Up item gate over `0x800F9840` and the `cobKinoko1up` walk.
- `0x807BFE20` - ship ride animation; `+0x5EC` lerps to `2400.0`.
- `0x8065C9A0` - run-through collection wrapper; **medium** because `bc 4,1` at
  `0x8065CA5C` branches on `total > 5`, so Ghidra swaps the two sound arms.
- `0x8076F970` - ship input dispatch (`0x1A`/`0x3F`/`0x94`/`0x95`).

## Lane C (5 bodies, 1,872 bytes)

- `0x80713B00` - `+0x6D6` angle stepper with the BSS `0x8082F8C4` term.
- `0x80773D70` - rebuilds the `0x16518`-stride element map over `0xC0` slots.
- `0x8078EB40` - manager id gate `(7,0x17)` plus the `0x17`/`0x25` cases.
- `0x80764C80` - five `0x38`-stride element transforms into `+0x7C`.
- `0x8062D660` - `dMultiCourseSelect_c` arrow-clip body.

## Lane D (5 bodies, 1,920 bytes)

- `0x80651640` - `3 x 20` spawn-cell grid probe; the Wave 3 rejection as a
  paired-single artefact is superseded. **medium**.
- `0x80684500` - two `+0x69C` sub-actor re-arm within `0.0..=12.0`.
  **medium** (both parameters recovered from `r3`/`r4` after the save thunk).
- `0x806AAB90` - broom-rider reinit; `+0xB0 = spawn_y + 32.0`.
- `0x806CC6F0` - panel stamping in the `[min, max)` band plus two `x ± 56.0`
  panels past frame `0xC`.
- `0x806D86E0` - `+0x518` rider-list slot-bit OR and four-bit count.

## Lane E (5 bodies, 1,928 bytes)

- `0x806EECA0` - `0x116` chain object with `0x115` children and `0x20`-byte
  records.
- `0x8078FF30` - pad-class pair to `+0x214` mode / `+0x218` latch.
- `0x807C0D30` - world-map floor re-seed; `0xFC1EF840` is `fcmpu`, not
  `fcmpo`. **medium**.
- `0x806C3790` - slope reaction on `+0x38C == 1` with the `10.0` clamp.
  **medium**.
- `0x80718CF0` - pipe transform from `[6,10,14,18,22]` and `0x807CCDF8`.
  **medium**.

## Corrections and cross-checks

- **Float A-form operand rule settled.** Two-operand arithmetic
  (`fadds`/`fsubs`/`fdivs`) uses FRB at bits 16-20; `fmuls` uses FRC at
  bits 21-25; four-operand forms are `fD,fA,fC,fB`. Confirmed independently by
  lanes B, C, D and E on `ec200824`, `ec4300b2`, `ec000072`, `ec211028`.
  `docs/REL_ELF_MAPPING.md` and the scratch decoder carry the per-XO rule.
- `0x8060A950`'s stack record is `0x3C` bytes, which resolves the wave-86
  deferral.
- `0x806AAB90`'s lift is `16.0 * 2.0`, not `16.0 * 16.0`; the multiply reads
  the converted `2.0` from FRC.
- `0x80684500` and `0x80740C90` take real second arguments hidden by the
  `0x802DCD6C`/`0x802DCD70` save helpers.
- `0x807BFE20`'s `0x807BE3D0` takes one `f64`; the caller's `f2 = 0.2` is dead.

## Verification

- `cargo check` succeeds with 4,216 public bodies and no duplicate names.
- `node tools/check-lane-csv.mjs wave174`: 27 rows, 27 unique addresses,
  20 high + 7 medium.
- All 27 evidence exports exist and every size matches `functions.tsv`.
