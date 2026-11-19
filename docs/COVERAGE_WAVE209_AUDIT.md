# Audit - Wave 209 (final pass)

## Summary

Two defects flagged by earlier lanes were resolved from the shipped words, and
the full gate was re-run.

## 1. `fn_80033F10` side polarity - fixed

`fn_80033F10` and `fn_8003ACD0` share the byte-identical comparison sequence:

```
fc010040  fcmpo cr0,f1,f0
4c411382  cror  2,1,2      ; EQ = EQ | GT
7c000026  mfcr  r0
54001ffe  rlwinm r0,r0,3,31,31   ; keeps LSB = old CR0 EQ
7c000034  cntlzw r0,r0
541dde3e  rlwinm r29,r0,27,24,31
```

That yields `1` exactly when `f1 < f0` (strictly negative):

- `less` sets LT\(\), so `EQ|GT = 0`, `cntlzw(0) = 32`, and the final rotate
  produces `1`;
- `greater|equal` produces `0`.

`fn_8003ACD0` already had this polarity. `fn_80033F10` was inverted
(`!((x - +0xB8) < 0.0)`); its body and doc are corrected.

## 2. `fn_8001D850` child-walk - fixed

The shipped body:

- probes `r3 = [node + 0x30]` (the **child**, not the parent) through the
  child's own vtable slot `+0xC`, passing the child in `r3`;
- after five failed probes returns `r31`, which is the **fourth** child;
- recurses with the fifth child only when that child is null.

The existing Rust called `call0_r(test)` (no argument) and recursed one link too
deep (`[object+0x30]` after the fifth advance). Rewritten to the shipped shape.

The doc block for `fn_8001D930` was accidentally consumed by the splice during
that edit and has been restored.

## Gate

- `cargo check` exit 0.
- `doc-lint` 0 violations across 68 files.
- `scripts/check.ps1` exit 0 (ledger 19,377 rows; 0 duplicate addresses;
  0 staging rows).

## Standing state

| Module | Translated |
|---|---|
| `game::bases` | 5,607 / 5,615 (2 documented deferrals) |
| `game::enemies` | 3,555 / 3,555 |
| `game::bosses` | 770 / 770 |
| `game::profile` | 36 / 36 |
| `ui::hud` | 540 / 540 |

Remaining main.dol work: 825 bodies / 114,976 bytes, mapped by nearest
translated neighbour to `src/game/player.rs` (281), `src/engine/dsys.rs` (238),
`src/resource/archive.rs` (178), `src/game/stage.rs` (128) and smaller files.
