# Wave 223, lane F - `game::stage`

Ten bodies from the actor-state cluster `0x800A1370..0x800A4480`, appended from
`work/wave223/laneF_block.rs`. Every `functions.tsv` size was re-checked against
the prologue and the `bclr` in `work/dol/main.elf`; all ten match exactly (no
prologue artefacts, no partial functions).

| address | name | tsv size | prologue → `bclr` | status | confidence |
|---|---|---|---|---|---|
| `0x800A1370` | [`fn_800a1370`] | 244 | `0x800A1370` → `0x800A1460` | translated | high |
| `0x800A16A0` | [`fn_800a16a0`] | 380 | `0x800A16A0` → `0x800A1818` | translated | high |
| `0x800A2750` | [`fn_800a2750`] | 136 | `0x800A2750` → `0x800A27D4` | translated | high |
| `0x800A29C0` | [`fn_800a29c0`] | 84 | `0x800A29C0` → `0x800A2A10` | translated | high |
| `0x800A2A20` | [`fn_800a2a20`] | 248 | `0x800A2A20` → `0x800A2B14` | translated | high |
| `0x800A2B80` | [`fn_800a2b80`] | 12 | no prologue → `0x800A2B88` | translated | high |
| `0x800A2B90` | [`fn_800a2b90`] | 12 | no prologue → `0x800A2B98` | translated | high |
| `0x800A3D30` | [`fn_800a3d30`] | 380 | `0x800A3D30` → `0x800A3EA8` | translated | high |
| `0x800A4150` | [`fn_800a4150`] | 148 | `0x800A4150` → `0x800A41E0` | translated | high |
| `0x800A4480` | [`fn_800a4480`] | 88 | `0x800A4480` → `0x800A44D4` | translated | high |

Totals: 10 bodies, 1 732 bytes, no deferrals.

## What the lane covers

* **`0x800A1370`** — actor initialiser. `0x800A8170(this)` refreshes the
  `+0x484` sub-object, `0x800B2EE0(2)` draws a PRNG byte through
  `0x80161460`, the summed translation is submitted as a notice and then the
  seven-word class-default block is copied from `0x802F07A0` into
  `+0x414..+0x42C` (`0, 0, 0, 0, 0x80357CB4, -1, -1`). The spawn parameters
  `+0x459`/`+0x38D` are snapshotted into `+0x430`/`+0x431`.
* **`0x800A16A0`** — the shared per-frame step referenced from the enemies
  lane: four player slots, an owner compare through `0x80162D00`, a re-parent
  through `0x8012E510`, virtual slot `0x98`, the fixed `+0x274/+0xE8..+0x11C`
  reset run, the `+0x102` angle re-seed from `0x8042C1A4`, two effect spawners
  and the `+0xB4` off-screen bound (`-384.0` / `4500.0`).
* **`0x800A2750`** — `:`-suffix name comparator; the original adds `1` to the
  `strrchr` result without a NULL guard.
* **`0x800A29C0` / `0x800A2A20`** — the state query already referenced by
  [`fn_800964a0`]: `+0x38C` selects the evaluator, the evaluator gates on flags
  `0x45`/`0x1C`/`0x2B` and on mode `+0x1090 == 3`, and answers `0..=3`.
* **`0x800A2B80` / `0x800A2B90`** — two byte-identical operand getters (a third
  lives at the unclaimed `0x800A2BA0`) returning `0x80377A04`.
* **`0x800A3D30`** — pending-yaw update: free integration with a `0.95` decay
  while `|speed| > 1.0`, otherwise a snap onto `180 * ceil(+0x73C / 180)`
  through the `0x8015F470` thunk (target `0x8015F480`).
* **`0x800A4150` / `0x800A4480`** — position setter (two `fdivs` by virtual
  slot `0x35C`) and the `s16` angle steer (`extsh` of slot `0x370` into
  `0x8015F4F0`).

## Export corrections recorded in the doc blocks

1. `0x800A1370` — four dead spill stores at `+0x28..+0x35`
   (`0x800A1440..0x800A1454`) are never read and are not modelled.
2. `0x800A16A0` — the export's `*(param_1 + 0x274) = 0` and
   `*(param_1 + 0x45C) = 0` are correct, but only because `li r3,0` survives the
   `lfs f3,0x41C` at `0x800A172C`. The effect spawners take `(f1, f2, r3, r4)`,
   the narrower shape of the shared `call2_f64_ptr4`.
3. `0x800A2A20` — the `FUN_802dcd74()` / `(ulonglong)uVar6 >> 0x20` pair in the
   export is the `_savegpr` helper (mirror `_restgpr` = `0x802DCDC0`), not a
   64-bit return; the entry arguments are intact after it.
4. `0x800A3D30` — the snap guard is emitted as a bit-30 probe on
   `(mask ^ remainder)`, not a plain sign compare; the literal probe is kept and
   the equivalence over the reachable `-179..=179` remainder range is noted.
5. `0x800A4150` — the two divisions are `fdivs`, not the widened `double`
   division the export prints.

## Verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to
`%TEMP%\opencode\w223laneF`, the block was appended to the copy's
`src/game/stage.rs`, and `cargo check --lib` finished with exit code 0 and no
diagnostic pointing at the appended lines.
