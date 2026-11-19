# Wave 14 — player attack and character-change handlers

## How to read this file

This document covers one Wave-14 slice of the gameplay player actor. Every
row is a main.dol function whose **entry address** is in
`0x80040000..0x80080000`; the Rust body lives in `src/game/player.rs` and no
staging module was created.

* **Address** — virtual entry point in the DOL image (not a file offset).
* **Name** — the Ghidra name. There is no surviving symbol table, so
  `FUN_<address>` is the honest identity until a role is proven.
* **Role** — one plain sentence. Words such as “attack” and “character
  change” are inferred from the fields and effects used, not from a recovered
  symbol.
* **Status** — `translated` means the full control flow and every call are
  present. All rows here are `translated`.
* **Confidence** — strength of the evidence, not code quality:
  `high` = instruction-level match, `medium` = some field/effect roles guessed.
* **Evidence** — the Ghidra C file under
  `work/ghidra/main-gb/export/`. The Rust body keeps the same
  `fn_<address>` name.

The two groups share the gameplay player object (`+0x38D` character index,
`+0x1454`/`+0x1458`/`+0x145C` change state, `+0x10D4`/`+0x10D8` flags) but
touch different subsystems:

1. `0x800480D0..0x80048E40` — short attack/landing poses. Each handler writes
   the pose id at `+0x14A4` and starts a three-float effect through the
   interface object at `[[player+0xDE8]+4]`.
2. `0x8004DE50..0x8004E980` — the character-change/power-up path. It owns one
   bit per character in the SDA masks at `0x80429CD0`/`0x80429CD8`, informs the
   manager at `0x80429C94`, and drives the interface object at
   `player+0x1418`.

## Functions translated by this wave

| Address | Name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x800480D0` | `FUN_800480D0` | Starts attack effect `0x16`, stores the animation rate at `+0x110`, sets `+0x14A8 = 8`. | translated | medium | `800480d0_FUN_800480d0.c` |
| `0x80048170` | `FUN_80048170` | Clears pose flags `0x30`/`0x97` and the halfword at `+0x100`. | translated | high | `80048170_FUN_80048170.c` |
| `0x80048410` | `FUN_80048410` | Starts attack effect `0x18` when the pose guard `0x80047EE0` is zero. | translated | medium | `80048410_FUN_80048410.c` |
| `0x800484B0` | `FUN_800484B0` | Starts attack effect `0x17` and zeroes `+0x10C`/`+0x110`. | translated | medium | `800484b0_FUN_800484b0.c` |
| `0x80048550` | `FUN_80048550` | Clears flag `0x30`; starts the `0x17` attack when the movement state is free. | translated | high | `80048550_FUN_80048550.c` |
| `0x800485A0` | `FUN_800485A0` | Same landing path with the three-byte player-state gate at `+0x120C..0x120E`. | translated | medium | `800485a0_FUN_800485a0.c` |
| `0x800488F0` | `FUN_800488F0` | Turns the player when movement state, facing direction and speed disagree, then plays the turn/idle animation. | translated | high | `800488f0_FUN_800488f0.c` |
| `0x80048A60` | `FUN_80048A60` | Predicate that starts the turn effect when the absolute speed is at least `2.5` and the direction matches. | translated | high | `80048a60_FUN_80048a60.c` |
| `0x80048BA0` | `FUN_80048BA0` | Queries actor slot `+0x444`, stores `+0x102`, and restarts the turn animation. | translated | high | `80048ba0_FUN_80048ba0.c` |
| `0x80048D70` | `FUN_80048D70` | Spawns `Wm_mr_drop` at `player+0x5B0` using the scale table at `0x802EEA68`. | translated | medium | `80048d70_FUN_80048d70.c` |
| `0x80048E10` | `FUN_80048E10` | Sets flag `0xC4` unless the pose word `+0x1090` is `3` or `0`. | translated | high | `80048e10_FUN_80048e10.c` |
| `0x80048E40` | `FUN_80048E40` | Starts effect `0x10` and writes `+0xEC = 1.0`. | translated | medium | `80048e40_FUN_80048e40.c` |
| `0x8004DE50` | `FUN_8004DE50` | Sets the character bit in mask B, clears it in mask A, writes `+0x1460 = 1`. | translated | high | `8004de50_FUN_8004de50.c` |
| `0x8004DEE0` | `FUN_8004DEE0` | Sets the character bit in mask A, clears it in mask B, writes `+0x1460 = 0`. | translated | high | `8004dee0_FUN_8004dee0.c` |
| `0x8004DF70` | `FUN_8004DF70` | Announces the active character to the manager/UI and optionally flags both lane actors `0x7D`. | translated | medium | `8004df70_FUN_8004df70.c` |
| `0x8004E050` | `FUN_8004E050` | ORs `0x0F` into the SDA state byte and clears the hold bit on the current and partner actors. | translated | medium | `8004e050_FUN_8004e050.c` |
| `0x8004E100` | `FUN_8004E100` | Clears the low nibble of the SDA state byte and sets the hold bit on both actors. | translated | medium | `8004e100_FUN_8004e100.c` |
| `0x8004E1A0` | `FUN_8004E1A0` | Resets motion, starts the `0x8035425C` animation with `1`, and applies the `0x8035476C` interface. | translated | high | `8004e1a0_FUN_8004e1a0.c` |
| `0x8004E220` | `FUN_8004E220` | Page/actor readiness predicate: page `0` and actor slot `+0x274` non-zero. | translated | medium | `8004e220_FUN_8004e220.c` |
| `0x8004E290` | `FUN_8004E290` | Offers an interface; on refusal/not-busy commits the change, otherwise recurses with the damage interface. | translated | high | `8004e290_FUN_8004e290.c` |
| `0x8004E360` | `FUN_8004E360` | Per-frame character-change state: timer, interface update, damage fallback, ready result. | translated | high | `8004e360_FUN_8004e360.c` |
| `0x8004E450` | `FUN_8004E450` | Starts the 15-frame change delay and releases the character hold. | translated | high | `8004e450_FUN_8004e450.c` |
| `0x8004E710` | `FUN_8004E710` | Confirms the character slot when the game is not in mode `2`. | translated | medium | `8004e710_FUN_8004e710.c` |
| `0x8004E920` | `FUN_8004E920` | Drop-height helper: `16.0 + f1 - min(+0xD4, 0x802EEB10[+0x14D4])`. | translated | high | `8004e920_FUN_8004e920.c` |
| `0x8004E980` | `FUN_8004E980` | Finishes a character change: clears motion, resets slot `+0x428`, sets pose `+0x1074`. | translated | high | `8004e980_FUN_8004e980.c` |

**Count: 25 new Rust bodies** (12 attack/pose handlers and 13
character-change handlers).

## Where the Ghidra C and the PowerPC bytes disagree

All listed items were checked with `work/dol/main.elf`; the Rust follows the
instruction stream. The `0x8005D050` / `0x8005D100` / `0x8005D090` calls already
show the character index in Ghidra's C and were confirmed in `r4`; they are not
listed as discrepancies.

| Address | Ghidra C | Instruction stream |
|---|---|---|
| `0x8004DE50` | `FUN_8005F900()` with no argument | `r3` is the sign-extended `+0x38D` character index |
| `0x8004E050` / `0x8004E100` | `FUN_8005F900()` with no argument | same: the character index is passed in `r3` |
| `0x8004E920` | `double` return, `double` argument | single-precision `f1` in and `f1` out (`fadds`/`fsubs`/`fcmpo`/`fsel`) |

The speed branches in `0x800488F0` and `0x80048A60` are strict: the raw `bc`
polarity confirms Ghidra's `speed > 0` / `speed < 0` comparisons. Both Rust
bodies keep those edges, so a speed of exactly `0.0` does not turn.

Two non-behavioural notes:

* `0x80048550` and `0x800485A0` call the thunk at `0x8005E830`, which jumps to
  `0x8005E500`; the Rust calls the thunk address.
* `0x80048A60` tests bit `0x00800000` of `+0x10D4`, which Ghidra prints as
  `0x800000`; both spellings are the same mask.

## Known gaps

* `// unknown:` the gameplay names of the effect ids `0x10`, `0x16`, `0x17`
  and `0x18`; only the interface slot `+0x5C` and the three-float vectors are
  observed.
* `// unknown:` the meaning of the three-float vectors at
  `0x80370E8C..0x80370EF0`; they are BSS and filled at runtime.
* `// unknown:` the roles of manager fields `+0x70`/`+0x80` and UI byte
  `+0x19`; they are written exactly but not named.
* `// inferred:` “character change” and “power-up” for the `0x1454`/`0x1458`/
  `0x145C` state block. The field movement is exact; the gameplay label is not
  proved.
* `0x8004E1A0` was previously a weak stub in `src/game/player.rs`; this wave
  replaced it with the full body and removed the stub.
