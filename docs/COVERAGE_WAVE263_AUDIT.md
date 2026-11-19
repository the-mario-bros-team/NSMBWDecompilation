# Audit - Wave 263 (polarity sweep, tenth vtable)

## 1. Another full vtable: `0x80303104` (`daEnDpakkunBase_c`) 187/187

The last 3 prologue starts plus 14 leaf/thunk slots closed the table. Shipped
strings recovered: `g3d/pakkun.brres` at `0x80303068`, `dead_ice` at `0x803030E4`,
`dead_leaf` at `0x803030F0`.

**Ten vtables are complete**: 275, 278, 311, 45, 306, 235, 265, 183, 182, 187.

## 2. Polarity sweep: 6 real branch-sense bugs

The convention, re-derived from the bytes:

| sequence | merge | `bc 4,2` | Rust |
|---|---|---|---|
| `fcmpo` + `cror 2,1,2` + `bc 4,2` | `a >= b` | taken when false | `!(a >= b)` |
| `fcmpo` + `cror 2,0,2` + `bc 4,2` | `a <= b` | taken when false | `!(a <= b)` |
| `cror ...` + `bc 12,2` | merge | taken when true | `a >= b` / `a <= b` |
| `mfcr` + `rlwinm 1,31,31` | CR0.LT | - | `!(a < b)` |

Bodies corrected:

| File | Body | Correction |
|---|---|---|
| `player.rs` | `fn_80064030` | first site is `cror 2,0,2`, so the `fsubs` arm runs on `target > current` |
| `bases.rs` | `fn_8074cba0` | anchor refresh runs on `d < 0` (`!(d >= 0.0)`) |
| `enemies.rs` | `fn_800acd70` | clamp runs on `vertical >= limit` (body returned on `>=`) |
| `enemies.rs` | `fn_800ae130` | same inversion for `y >= limit` |
| `worldmap.rs` | `fn_80151040` | arm runs on `first <= 0` (body had the negation) |
| `stage.rs` | `fn_800adb00` | NaN sense `< 0` -> `!(>= 0)` |

Two were re-verified here against the raw words: `0x800ACDB4`'s
`fcmpo f3,f1` / `cror 2,1,2` / `bc 4,2` branches to the epilogue when
`f3 < f1`, so the clamp really runs on `>=`; and `0x80151040`'s `cror 2,0,2` +
`bne` really runs its arm on `<= 0`.

Stale prose fixed in `fn_800244d0`'s header, `docs/COVERAGE_WAVE211_PLAYER.md`
and the wave-261 lane C report.

## 3. Body bug found by a translation lane and fixed here

`fn_80024a70` (`ui::hud`, slot 134 of `0x8030241C`) seeded `+0x348` with the
**inverted** side. The image does:

```
mr r31,r4 ; mr r30,r3            ; r31 = other, r30 = this
lfs f3,0xAC(r30) ; lfs f2,0xD0(r30)   ; this sum
lfs f1,0xAC(r31) ; lfs f0,0xD0(r31)   ; other sum
fadds f2,f3,f2 ; fadds f1,f1,f0       ; f1 = other, f2 = this
bl 0x80064A30
```

and `0x80064A30` answers `f1 > f2` (its target does
`fcmpo cr0,f1,f2` / `mfcr` / `rlwinm r3,r3,1,31,31`, i.e. CR0.GT). The body
passed `this` first and `other` second, so the recorded side was the opposite.
Fixed, with the operand order now documented in the block.

## 4. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | `0x80306400` next 12 | 12 | 3,464 |
| B | `0x80303104` remainder (closes table) | 17 | ~700 |
| C | `0x8030241c` next 12 | 12 | 408 |
| D | `0x80304e18` next 12 | 11 + 1 shared | ~1,400 |
| E | `0x80304e18` next 5 | 5 | 560 |
| F | polarity sweep | 6 fixes | - |

Coverage after the wave: `0x80306400` 165/193 (28 uncovered, 2 prologue),
`0x8030241c` 173/199 (26 uncovered, 2 prologue), `0x80304e18` 170/186
(16 leaves), `0x80303104` 187/187.

## 5. Findings

- Lanes D and E worked `0x80304e18` concurrently and both independently found
  the same descriptor typo: `0x80034750` hands virtual `+0xD4` the object at
  `0x803535EC`, not `0x803536EC` (`lis 0x8035` + `addi 13804`). Verified here in
  the image; both bodies now agree. A duplicate constant alias remains
  (`DAT_803535EC_E` / `DAT_803535EC_D263`) and is queued for consolidation.
- `0x83` vs `0x803536EC` is the same `addi`-as-`ori` family that has produced
  most of the project's page errors.
- Lane C catalogued three untabled leaves that sit in padding gaps and are not
  table targets (`0x800206E0`, `0x800206F0`, `0x80025AA0`); queued.
- `0x80306400` (uninstalled `daEnTogezo_c` vtable) is down to 2 prologue starts.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-dupe-bodies` exit 0.
- `scripts/check.ps1` exit 0.
