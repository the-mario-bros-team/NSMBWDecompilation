# Audit - Wave 270 (three body bugs fixed, six more tables opened)

## 1. Three body-level defects found by the audit, verified and fixed here

### `fn_80035d60` (`ui/hud.rs`) - wrong float pair

The site loads `r2 - 0x7B20` / `r2 - 0x7B24`, which are `0.2815f` and
`0.8125f`:

```
80035d68: lfs r1,-0x7B20(r2)   ; 0.2815
80035d78: lfs r0,-0x5370(r13)  ; 3.628 (per-stage)
80035d84: fadds f1,f1,f0
80035d88: lfs r0,-0x7B24(r2)   ; 0.8125
80035d98: fmuls f1,f1,f0
```

The body used `0x8042B540` (`-0.0f`) and `0x8042B544` (`1.0f`). Replaced with
exactly-named constants `F32_0P2815_7B20_F269` / `F32_0P8125_7B24_F269`; the old
`F32_NEG_ZERO_F269` had no other reader and was removed.

### `fn_800caa70` (`game/stage.rs`) - inverted arm and a bogus doubling

The image returns the single row when flag bit `0x8` is **clear**
(`bclr 12,2` after `rlwinm. 0,28,28`) and **adds** the paired row when it is set:

```
800caaa0: rlwinm. r0,r5,0,28,28   ; bit 0x8
800caaac: lfsx f1,base+0x80,col
800caab0: bclr 12,2               ; bit clear -> return the single row
800caab8: lfsx f0,base+0x90,col
800caabc: fadds f1,f1,f0
```

The body had the arm inverted and computed `value + value`. Corrected to
`0x80 + 0x90` / `(0xA0|0xB0) + 0xC0`, with the same fix for the second branch.

### `fn_80044960` (`game/player.rs`) - inverted scene gate, dead tail, wrong return

- the scene gate is `bc 12,2` to the shared tail at `0x80044A60`, i.e. it skips
  the probes when `scene+0x164 == 0`; the body returned when it was non-zero;
- both gates **jump to the tail**, they do not return - the tail always runs;
- `0x80044A98` is `li r3,1`, so the function returns `1`; the body was `-> ()`.

Corrected to a `skip` flag around the probe/dispatch block, the tail always
executed, and the signature changed to `-> u32` returning `1`.

## 2. Six tables opened / one closed

| Table | Slots | Class | Coverage |
|---|---:|---|---:|
| `0x80305344` | 169 | `daEnLkuriboBase_c` | **169/169 closed** |
| `0x8032b524` | 29 | `nw4hbm::lyt::Window` | 16/29 |
| `0x80323370` | 158 | `daEnEatCoin_c` | 150/158 |
| `0x803017d8` | 92 | `daBossDemo_c` | 62/92 |
| `0x8034f9fc` | 33 | NW4R `lyt::Window` (`wnd1`) | 17/33 |

**Twenty-two vtables complete.**

New structures recorded:

- `0x803017d8`'s parent is `0x80301504` (77 slots: 44 inherited, 33 overrides),
  itself an unqueued table.
- `0x8032b524`'s sibling is `nw4hbm::lyt::TextBox` at `0x8032B43C` (27 slots).
- `0x8034f9fc` shares the `0x8034F9B8` corner templates with `fn_802b0680`.
- The survey's claimed owner for `0x803017d8` (`0x8001E1C0`) is actually slot 84
  of the same table, materialising the `"Wm_en_hit"` string - containment is
  genuinely 0/92.

## 3. A live logic inversion in two older bodies

Lane C270 noticed that its own new body disagreed with two existing twins. The
image tests the name query with `neg r0,r3` / `or r0,r0,r3` /
`rlwinm. r0,r0,1,31,31` / `bne`: it proceeds when the answer is **zero**.
`fn_801111b0` and `fn_80111580` had `if id != 0`, so the snapshot ran on the
wrong answer. Both corrected to `if id == 0`, with the idiom documented.

A third body using the same idiom (`fn_80111e50`) was checked and is correct: its
branch goes to the return-0 path, so `if id != 0 { return 0 }` matches.

## 4. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | close `0x80305344` (14) | 14 | 496 |
| B | `0x8032b524` first 12 | 12 | ~4,000 |
| C | `0x80323370` first 12 | 12 | 2,436 |
| D | `0x803017d8` first 12 | 12 | 1,288 |
| E | `0x8034f9fc` first 12 | 12 | 7,968 |
| F | audit + 3 fixes | 3 fixes | - |

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-dupe-bodies` exit 0; `check-page-offsets` 34 pairs / 13,659 tokens,
  0 mismatches.
- `scripts/check.ps1` exit 0.
