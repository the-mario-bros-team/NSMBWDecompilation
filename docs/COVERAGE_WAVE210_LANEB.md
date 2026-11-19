# Coverage - Wave 210, lane B (`game::player`)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 10 | 1792 | 7 | 3 | 0 |

Status split: 10 `translated`, 0 `deferred`, 0 `partial`.

Bytes are the `functions.tsv` sizes summed: 116 + 96 + 32 + 120 + 136 + 344 +
416 + 312 + 36 + 184 = 1792.

## How to read this file

* **Address** - virtual entry point in the DOL image (`work/dol/main.elf`), not a
  file offset.
* **Name** - the Ghidra name (`FUN_<address>`). No symbol table survives for
  these functions, so the Rust body keeps the `fn_<address>` name.
* **Confidence** - strength of the evidence, not code quality: `high` = every
  address, offset and branch is literal; `medium` = a field role, a callee's
  meaning or the caller's register setup had to be inferred.
* Every body uses the module's fixed SDA bases, `r2 = 0x80433080` and
  `r13 = 0x8042F680`. The new constants are suffixed `_B` and were re-read from
  `work/dol/main.elf`.

## Size note

The lane brief listed `0x80060AB0` and `0x80060DB0` as 20 bytes each. Both are
prologue artefacts: `work/ghidra/main-gb/export/functions.tsv` records 344 and
312 bytes, and the real bodies run to the `blr` at `0x80060C04` and
`0x80060EE4`. The `# Original` lines carry the `functions.tsv` values, as the
shared instructions require.

Both functions open with a `bl` to `0x802DCD6C`/`0x802DCD74` and close with a
`bl` to `0x802DCDB8`/`0x802DCDC0`. Those are the compiler's
`_savegpr_25`/`_savegpr_27` and `_restgpr_25`/`_restgpr_27` register-save stubs
(4- and 24-byte butted entries in `functions.tsv`), not calls; the corresponding
`GQR0`, `f31` and `unaff_r2` noise in the Ghidra exports comes from their spill
slots. They are omitted from the Rust bodies and noted in the doc blocks.

## What this pass covers

- **The presence-bit pair `0x80060750` / `0x80060860` (116 + 120 B).** Two
  mirrored scan-and-clear routines over the four wave-88 slots. The first
  watches the `+0x1070` state word for `> 60` and clears presence bit 0 plus
  cue `4`; the second watches event flag `0x4B` through `fn_80056cf0` and
  clears bit 1 plus cue `0x200`. Bit 1 is raised by **`0x80060830` (32 B)**,
  which applies cue `0x200` through `0x8019BC50`.
- **The per-slot select cue `0x800607D0` (96 B).** A one-argument wrapper: the
  export prints no parameters because `bl 0x8005F900` is entered with the
  incoming `r3` untouched, so the slot index is forwarded straight from the
  caller. Plays the `0x74E` cue when vtable `+0xE0` returns `0` and `+0x1554` is
  non-zero.
- **The two-player latch `0x800608E0` (136 B).** Gates on
  `fn_8005ffb0() >= 2`; the export's
  `((n ^ 1) >> 1) - ((n ^ 1) & n) < 0` is the compiler's `n >= 2` idiom (it
  evaluates to `0, 0, -1, -1, -2` for `n = 0..4`). Direction comes from
  `0x80429CA0 < 2`: raise bit 2 and cue `0x400`, else clear both.
- **The nearest-actor retire `0x80060AB0` (344 B).** Builds a search point from
  the scene object at `0x80429E00` (`+0x38 + +0x40 * 0.5`,
  `+0x3C - +0x44 * 0.5`), then picks the farthest-eligible actor over the four
  ids at `0x80354E20`: alive (`+0x0B == 0`), vtable `+0x6C` returns a pointer to
  a signed byte `-1`, and flag `0xB9` set. Length comes from `0x802BE470` and
  the comparison is done as a double against a `0.0` start. The winner is
  retired with `0x80162510`, and the predicate returns `found != 0`. The HUD
  still calls it by address as `0x80060AB0` (`src/ui/hud.rs`).
- **The slot effect reset `0x80060C10` (416 B).** `index == -1` is a no-op;
  `mode == 2` is a single `7`-row `0x800D8BB0` request. Otherwise the slot's
  `0x80354EB0` countdown becomes `5` and its `0x80354EC0` flag `0`. While the
  `0x8042A21F` byte is still zero, every *other* slot with a non-zero countdown
  is also forced to `5`, and the number of such slots selects the rest: none →
  row `3,3` (`mode == 1`, `0x800D8B00`) or row `4` (`0x800D8BB0`); some → seed
  the three `0x80355CB0` rows (`0x152`, `0x151`, `0x150`), post row
  `others - 1` through `0x80195370`, and replay `0x80060DB0`. The `stwx`
  stores use `index & 0xFF` while the exclusion compares use the full word;
  both forms are reproduced.
- **The ship-attack replay `0x80060DB0` (312 B).** For each armed slot
  (countdown non-zero, flag zero) it sets the flag to `1`, resolves the actor,
  upgrades through `0x80139950` when flag `0x4B` is set, and spawns
  `Wm_mr_vshipattack` at `+0xAC` followed by `Wm_mr_vshipattack_ind` at the same
  point with `z = 3800.0`. Each pair is followed by the `0x800D8B00` request
  `(row 3, bands 3, tails 0x12)` on the owner table.
- **The readiness bit `0x80060EF0` (36 B).** `0x80354E90[0] >= 99` writes `1`
  to `0x80429CE8`, else `0`; the compare is signed (`cmpi`).
- **The draw-order assignment `0x80060F20` (184 B).** Takes an argument in
  `r3` (again hidden by the `_savegpr_27` stub), skips its own id, marks
  `taken[peer[0x1036]]` for every other resolvable id, then claims the lowest
  free byte and renumbers the peers through `0x80060FE0`. If all four are
  taken nothing is written, exactly as in the original. The `0x1036` role is
  inferred from `0x80060FE0`, which increments the peers below the freed index.

## Not covered here

`0x80060FE0` (the renumbering pass) is called by address and is not part of
this lane. No body in this lane was deferred.
