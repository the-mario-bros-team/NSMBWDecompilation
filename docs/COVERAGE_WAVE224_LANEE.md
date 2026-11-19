# Wave 224, lane E - `0x800A77A0..0x800AA7F0` (`game::stage`)

Ten bodies from the `0x800Axxxx` band of `src/game/stage.rs`: the per-unit
instant-effect submission loop, six identical 12-byte static-record getters, the
active-player nearest-slot selector, the two-slot release wrapper and the `u16`
kind predicate.

All ten are `translated`; nothing is deferred.

Block file: `work/wave224/laneE_block.rs`
Ledger: `ledger/agents/wave224-laneE.csv`

## Bodies

| address | size (tsv) | real extent | name | confidence | role |
|---|---|---|---|---|---|
| `0x800A77A0` | 192 | `0x800A77A0..0x800A7860` (192) | [`fn_800a77a0`] | high | one effect spawn per unit, word assembly |
| `0x800A7DA0` | 12 | `0x800A7DA0..0x800A7DAC` (12) | [`fn_800a7da0`] | high | return `0x80377A04` |
| `0x800A7DD0` | 12 | `0x800A7DD0..0x800A7DDC` (12) | [`fn_800a7dd0`] | high | same stub, second copy |
| `0x800A8780` | 12 | `0x800A8780..0x800A878C` (12) | [`fn_800a8780`] | high | same stub, third copy |
| `0x800A87B0` | 12 | `0x800A87B0..0x800A87BC` (12) | [`fn_800a87b0`] | high | same stub, fourth copy |
| `0x800A8840` | 12 | `0x800A8840..0x800A884C` (12) | [`fn_800a8840`] | high | same stub, fifth copy |
| `0x800A8870` | 12 | `0x800A8870..0x800A887C` (12) | [`fn_800a8870`] | high | same stub, sixth copy |
| `0x800AA2A0` | 204 | `0x800AA2A0..0x800AA36C` (204) | [`fn_800aa2a0`] | medium | nearest active player slot by `+0xAC` |
| `0x800AA480` | 84 | `0x800AA480..0x800AA4D4` (84) | [`fn_800aa480`] | high | call vtable `+0x4DC`/`+0x540`, clear `+0x100` |
| `0x800AA7F0` | 96 | `0x800AA7F0..0x800AA850` (96) | [`fn_800aa7f0`] | high | `1` for `0x00C3` and `0x00F1..=0x00F7` |

Every `functions.tsv` size was re-checked in `work/dol/main.elf` from the
prologue down to the closing `bclr`.

- `0x800A77A0`: `stwu r1,-0x30` … `bclr` at `start + 0xBC`, extent 192 exactly.
- The six getters: no prologue at all - `lis` / `addi` / `bclr` (12 bytes), with
  four zero alignment bytes behind the `bclr` and the next entry at `+0x10`.
- `0x800AA2A0`: `stwu r1,-0x20` … `bclr` at `start + 0xC8`, extent 204 exactly.
- `0x800AA480`: `stwu r1,-0x10` … `bclr` at `start + 0x50`, extent 84 exactly.
- `0x800AA7F0`: leaf (no frame) with `bclr` at `start + 0x5C`; the three
  `li r3,0` / `li r3,1` return sites end the body, extent 96 exactly.

No size in this lane is a prologue artefact.

## Byte weight

192 + 12 + 12 + 12 + 12 + 12 + 12 + 204 + 84 + 96 = **648 bytes**.

## Evidence notes

- `0x802DCD60` / `0x802DCDAC` (`0x800A77A0`) and `0x802DCD74` / `0x802DCDC0`
  (`0x800AA2A0`) are the CodeWarrior `_savegpr_22` / `_restgpr_22` and
  `_savegpr_27` / `_restgpr_27` stubs. They only spill and reload callee-saved
  registers, so they have no Rust counterpart and are elided; they also explain
  why Ghidra prints both bodies with fewer arguments than the ABI carries
  (`_savegpr` preserves `r3`/`r4`).
- `0x800A77A0` ORs two `.data` mask tables into the spawn word. Read from
  `work/dol/main.elf`: `0x802F08A8` = `0, 0, 0x00400000, 0x00400000` and
  `0x802F0898` = `0, 0x80000000, 0, 0x80000000`. Ghidra's `extraout_r4` as the
  third spawn argument is the caller's second argument (`mr r5,r22`).
- `0x800AA2A0` reuses the existing `W105_G_ACTIVE_PLAYERS`
  (`SDA_BASE - 0x59D8`, `lbz r4,0xffffa628(r13)`), read once per iteration and
  tested twice with `slw`/`and`. The best slot is looked up before the
  candidate, and a null or inactive best slot is replaced without any distance
  comparison. Ghidra declares the function `void`, but `mr r3,r29` before the
  restore stub makes the winner index the return value; this is the only
  inferred point in the lane, hence `medium`.
- `0x800AA7F0`: Ghidra's two upper arms collapse to `if (...) return 0;` plus a
  trailing `return 0`; the raw ladder answers `0` on every path above `0x00FC`.
  The `0x00F1` test is `bf 0` (`>=`), so `0x00F1` itself answers `1`, matching
  Ghidra's `0xf7 < uVar1` formulation of the same set.
- `0x800AA480` reloads the vtable from `+0x60` twice and keeps `this` in `r3`
  for the first `bcctr`; the second call re-materialises it with `mr r3,r31`.

## Scratch verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to a temp directory, the block
was appended to the copy's `src/game/stage.rs`, and `cargo check --lib` there
finished with exit code 0 (warnings only, none from this lane).
