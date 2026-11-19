# Wave 227, lane F - NaN / unordered audit of IEEE compares

Working dir: `C:\Users\WitherULTRA7\wii-decomp\nsmbw`.
Scope: committed Rust bodies whose `# Original` block reproduces a PowerPC
floating-point compare, looking for places where the Rust `a < b` / `a <= b`
answers `0` for a NaN operand while the shipped `fcmpo` / `cror` / `cntlzw`
sequence answers `1`.

## 1. What the hardware actually computes

`fcmpo cr0,fA,fB` sets `CR0 = {LT, GT, EQ, UN}` with exactly one bit set, and
`UN` is set whenever either operand is a NaN. Metrowerks' two-valued compare
idiom then merges and inverts a pair of bits:

```
fcmpo  cr0,fA,fB
cror   2,1,2          ; CR0[EQ] := CR0[GT] | CR0[EQ]   i.e. "fA >= fB"
mfcr   r0
rlwinm r0,r0,3,31,31  ; rotate the merged CR0[EQ] bit down to bit 31
cntlzw r0,r0          ; 32 when the bit is clear, 31 when it is set
rlwinm rD,r0,27,5,31  ; >> 5  ->  1 when clear, 0 when set
```

So the materialised word is `1` iff `NOT(GT|EQ)`, which is `LT|UN`:

| fA vs fB | `LT` | `GT` | `EQ` | `UN` | merged `GT\|EQ` | materialised value |
|---|---|---|---|---|---|---|
| fA < fB   | 1 | 0 | 0 | 0 | 0 | **1** |
| fA == fB  | 0 | 0 | 1 | 0 | 1 | 0 |
| fA > fB   | 0 | 1 | 0 | 0 | 1 | 0 |
| unordered | 0 | 0 | 0 | 1 | 0 | **1** |

The mirrored form `cror 2,0,2` merges `LT|EQ` (`fA <= fB`) and materialises
`GT|UN`. Branch forms (`cror` + `bc 4,BT` / `bc 12,BT`) select the same two
sets: `bc 4` takes `NOT merged`, `bc 12` takes `merged`.

The translation rule this wave applied:

| hardware set (`value == 1`) | exact Rust | what the repo used | ordered inputs | unordered |
|---|---|---|---|---|
| `LT\|UN` | `!(a >= b)` | `a < b` | identical | Rust 0, HW 1 (diverges) |
| `GT\|UN` | `!(a <= b)` | `a > b` | identical | Rust 0, HW 1 (diverges) |
| `LT\|EQ` | `a <= b` | `a <= b` | identical | identical (0) |
| `GT\|EQ` | `a >= b` | `a >= b` | identical | identical (0) |

Only the first two rows are defects. They are defects precisely **because**
the compiler's merge is `>=`/`<=` and then inverted, i.e. the shipped code is
the ordered-NaN-safe predicate while the Rust is the plain strict one.

## 2. Method / evidence

* `work/wave227/laneF_dis.mjs` linearly disassembles the whole
  `main.dol` text segment (`0x80006780 + 0x2E7260`, ELF program headers read
  from `work/dol/main.elf`) and records every `fcmpo` / `fcmpu` with its next
  8 instructions and its owning `functions.tsv`-extent function:
  `work/wave227/laneF_fcmp_sites.txt` (3,984 sites).
* `work/wave227/laneF_class.mjs` folds each site's `cror` merge + `cntlzw` /
  `bc` into the truth set above: 1,126 sites use a `cror` merge, 21 of them
  materialise the value with `cntlzw` (`work/wave227/laneF_materialised.tsv`).
* `work/wave227/laneF_rust.mjs` extracts every committed body that contains a
  float-ish strict comparison and prints it next to the classified hardware
  sites of the same function: `work/wave227/laneF_rust_strict.txt`,
  `work/wave227/laneF_branch_class.tsv` (333 functions).

The decoder is `%TEMP%/opencode/ppc159.js` (lane E's copy); every site quoted
below was re-checked with it against the raw words, and the operands were
read from the preceding `lfs` (both `fcmpo` operands in this game are always
a `+offset(rN)` load and an `r2`-relative constant, so operand order is
unambiguous).

## 3. Materialised (`cntlzw`) sites in committed bodies

`own` = the Rust body that owns the address; `disposition` is what this wave
did.

| site | owner | operands | value == 1 when | Rust form before | disposition |
|---|---|---|---|---|---|
| `0x8002D570` | `fn_8002d480` (`src/ui/hud.rs:19843`) | `+0xE8` vs `0.0` | `LT\|UN` | `v > 0.0` | **not fixed** - already inverted for ordered inputs, see §5 |
| `0x8002FAAC` | `fn_8002fa50` (`src/ui/hud.rs:21498`) | `+0xE8` vs `0.0` | `LT\|UN` | `v < 0.0` | **fixed** |
| `0x80031B9C` | `fn_80031b70` (`src/ui/hud.rs:22770`) | `x-+0xB8` vs `0.0` | `LT\|UN` | `!((x-+0xB8) >= 0.0)` | already exact |
| `0x80033A58` | `fn_80033a30` (`src/ui/hud.rs:24192`) | `+0xE8` vs `0.0` | `LT\|UN` | `!(0.0 <= v)` | already exact |
| `0x80033F3C` | `fn_80033f10` (`src/ui/hud.rs:24456`) | `x-+0xB8` vs `0.0` | `LT\|UN` | `(x-+0xB8) < 0.0` | **fixed** (lane-F known case) |
| `0x8003ACFC` | `fn_8003acd0` (`src/ui/hud.rs:29820`) | `x-+0xB8` vs `0.0` | `LT\|UN` | `>= ... else 1` | already exact (reference) |
| `0x800A0578` | `fn_800a0550` (`src/game/stage.rs:72444`) | `+0xE8` vs `X` | `LT\|UN` | `v < X` | **fixed** (lane-F known case) |
| `0x8011E0AC` | `fn_8011e020` (`src/game/worldmap.rs:17283`) | `object+0xAC` vs `other+0xAC` | `LT\|UN` | `other > object` | **fixed** |
| `0x8011E34C` | `fn_8011e1a0` (`src/game/worldmap.rs:17370`) | `object+0xAC` vs `other+0xAC` | `LT\|UN` | `other > object` | **fixed** |
| `0x8011E48C` | `fn_8011e420` (`src/game/worldmap.rs:17426`) | `object+0xAC` vs `other+0xAC` | `LT\|UN` | `other > object` | **fixed** |
| `0x8011EA2C` | `fn_8011ea00` (`src/game/worldmap.rs:37252`) | `partner+0xAC` vs `object+0xAC` | `GT\|UN` | `object <= partner` | **not fixed** - pre-existing polarity mismatch, see §5 |
| `0x8011F730` | `fn_8011f700` (`src/game/worldmap.rs:17991`) | `+0xE8` vs `0.0` | `LT\|UN` | `v < 0.0` | **fixed** |
| `0x8011F744` | `fn_8011f700` (`src/game/worldmap.rs:17992`) | `+0xEC` vs `0.0` | branch: body when `LT\|EQ` | `v <= 0.0` | already exact |
| `0x801214E4` | `fn_801214a0` (`src/game/worldmap.rs:31268`) | `+0xB4` vs `0.0` | branch: taken when `LT\|UN` | `v < 0.0` | **fixed** |
| `0x8012155C` | `fn_801214a0` (`src/game/worldmap.rs:31281`) | `player+0xC4` vs `0.0` | `LT\|UN` | `height < 0.0` | **fixed** |
| `0x8002D6BC`, `0x80097BB4`, `0x800A08F8`, `0x800AAFB0`, `0x8011E74C`, `0x801558A4` | none (`functions.tsv` has no extent covering these addresses) | - | - | no committed Rust body | nothing to fix; listed in `laneF_materialised.tsv` |
| `0x801C700C` | `fn_801c6ff0` (`src/runtime/dvd.rs`) | - | - | `cntlzw` here folds an integer (`aniso-1` / `zero_flag`), not the CR merge | false positive of the scan |

## 4. Fixes applied (all `cargo check` clean, `doc-lint` clean)

Every edit is the same rewrite, ordered-identical by construction
(`!(a >= b)` == `a < b` for every ordered pair; Rust defines `!(a >= b)` as
`true` for NaN, which is the hardware `LT|UN` / `GT|UN` answer):

1. `src/ui/hud.rs:24456` `fn_80033f10` - `(x - +0xB8) < 0.0` -> `!((x - +0xB8) >= 0.0)`.
2. `src/ui/hud.rs:21498` `fn_8002fa50` - `+0xE8 < 0.0` -> `!(+0xE8 >= 0.0)`; the
   old comment "`cntlzw(0.0 <= v) >> 5` is exactly `v < 0.0`" was only true for
   ordered `v` and is corrected.
3. `src/game/stage.rs:72444` `fn_800a0550` - `height < X` -> `!(height >= X)`.
4. `src/game/worldmap.rs:17991` `fn_8011f700` - `+0xE8 < 0` -> `!(+0xE8 >= 0)`.
5. `src/game/worldmap.rs:31268` `fn_801214a0` (mode) - `+0xB4 < 0` -> `!(+0xB4 >= 0)`.
6. `src/game/worldmap.rs:31281` `fn_801214a0` (side) - `player+0xC4 < 0` -> `!(player+0xC4 >= 0)`.
7. `src/game/worldmap.rs:17283` `fn_8011e020` (side) - `other > object` -> `!(object >= other)`.
8. `src/game/worldmap.rs:17370` `fn_8011e1a0` (side) - same.
9. `src/game/worldmap.rs:17426` `fn_8011e420` (side) - same.

Reachability: sites 1-6 are height / one-dimensional side words of actors
that are live in shipped stages (the same `+0xE8` / `+0xB8` / `+0xB4` fields
that the return-value and hit-record writers use); sites 7-9 are the contact
"which way" side byte of the WorldMap magma-wave handlers, called from the
shipped contact path. None of these values is loaded from a constant table, so
a NaN produced anywhere upstream (division, `sqrt`, `fres`) survives into the
compare, which is why the difference is observable.

Prose updated with the change: `fn_80033f10` (`# Confidence`),
`fn_800a0550` (`# Original`, the "flag is `height < X`" sentence),
`fn_8002fa50` (sequence step 3), `fn_8011f700` and `fn_801214a0`
(observed notes), and each `side` site carries an `// observed:` comment.

## 5. Found but deliberately not fixed

* `fn_8002d480` (`src/ui/hud.rs:19843`): hardware `0x8002D570` materialises
  `+0xE8 < 0.0` **or unordered** (Ghidra prints
  `countLeadingZeros((uint)(0.0 <= v))`, i.e. `1` iff `NOT(0 <= v)`), but the
  Rust reads `(player + 0xE8 > 0.0) as u32`. This is inverted for *ordered*
  inputs too, so it is a pre-existing polarity defect and not a NaN-only
  divergence; changing it is not provably order-preserving and needs its own
  adjudication against the caller (`0x80066630`'s selector). The prose at
  `src/ui/hud.rs:19761` and the comment at `:19842` repeat the wrong `> 0.0`
  reading.
* `fn_8011ea00` (`src/game/worldmap.rs:37252`): hardware `0x8011EA2C` sets
  `value = GT|UN` for `(partner+0xAC) vs (object+0xAC)` and the call passes
  `value ^ 1` = `LT|EQ` = `partner <= object`, while the Rust passes
  `object <= partner`. The operand roles look swapped somewhere in this body
  (the body also passes `object` in `r3` where the image passes `r29`), so the
  NaN question cannot be settled without re-deriving the argument mapping.
  Documented, untouched.

Already-exact bodies (no action): `fn_8003acd0`, `fn_80031b70`,
`fn_80033a30`, and the `+0xEC <= 0` gate of `fn_8011f700`.

## 6. The branch-form population (documented, not edited)

The same merge appears in branch form ~1,126 times in `main.dol`. In 333
committed bodies a strict Rust float compare sits next to an unordered-true
branch site; those are listed with file:line, Rust expression, site address and
hardware taken-set in `work/wave227/laneF_branch_class.tsv`. Sample of the
highest-value ones (all `<` / `<=` written for a `LT|UN` / `GT|UN` branch):

| body | Rust | hardware |
|---|---|---|
| `fn_8001e1c0` `src/game/actor.rs:3260` | `this+0xAC < other+0xAC` | `0x8001E1E4` `bc 4,2` of `cror 2,1,2` = `LT\|UN` |
| `fn_8001e2e0` `src/game/actor.rs:3302` | `other+0xE8 < 0.0` | `0x8001E304` = `LT\|UN` |
| `fn_8001e400` `src/game/actor.rs:3337` | `other+0xE8 < 0.0` | `0x8001E424` = `LT\|UN` |
| `fn_801a55f0` `src/audio/ax.rs:4031` | `limit < voice+0x38` | `0x801A5624` `bc 4,2` of `cror 2,0,2` = `GT\|UN` |
| `fn_8004c620` `src/game/player.rs:3648` | `player+0xEC <= 1.5` | `0x8004C644` `bc 4,2` of `cror 2,1,2` = `GT\|UN` |
| `fn_80051bd0` `src/game/player.rs:9426` | `player+0xEC < 0.0` | `0x80051C44` `bc 4,0` = `GT\|EQ\|UN` |
| `fn_8004bac0` `src/game/player.rs:3326` | `player+ACTOR_SPEED < 0.0` | `0x8004BAF8` `bc 4,0` = `GT\|EQ\|UN` |
| `fn_801a34f0` `src/audio/ax.rs:3802` | `limit <= voice+0x114` | `0x801A3524` `bc 4,0` = `GT\|EQ\|UN` |

Whether a given row is a real divergence depends on which arm the Rust guard
owns (taken set vs. its complement) and on whether the compared operand can be
NaN; the appendix records the raw taken-set so the next wave can finish the
adjudication without re-disassembling. Editing them was out of scope here:
unlike the materialised words in §3 there is no single-value truth table to
compare against, and most of the operands are quantities with a guarded finite
range (audio levels, counters, tuned constants).

## 7. Verification

* `cargo check` -> `Finished dev profile ... exit=0`.
* `node tools/doc-lint.mjs src` -> `{ "ok": true, "files": 68, "violations": 0 }`.
* No `functions.tsv` size, ledger row or `# Original` byte count was changed,
  so `node tools/check-lane-csv.mjs` is unaffected by this lane.
