# Coverage - Wave 89 (enemies: the head of the `0x80AE0000` bucket)

## How to read this file

This page records the Wave 89 `game::enemies` lane: the first 28 functions of
the `0x80AE0000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs`. Every row below is one real `pub unsafe fn fn_<addr>`
body in that file, and the `evidence` column of the machine-readable mirror
(`ledger/agents/wave89-enemies.csv`) names the Ghidra C export each body was
reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Float,
angle and mask constants are cited by address and read through
`platform::read_u32` / `read_f32` / `read_u16` / `read_u8`, never transcribed as
invented values. The values themselves were verified against
`work/rels/elf/d_enemiesNP.elf` and are listed in the section comment that
opens the new code block in `src/game/enemies.rs`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 28 | 6,304 | 15 | 13 |

The bucket holds 160 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 46 were already in
`ledger/functions.csv` before this pass, 28 are claimed here and 86 remain.

## What this pass covers

One contiguous run, `0x80AE0560..0x80AE2D00`, minus the two functions listed
under "Deliberate skips". The run is the head of the bucket and contains two
sibling actor classes from the same asset family.

**Cluster 1, `0x80AE0560..0x80AE1080`, the `met_pata` body actor.** The class
name is literal: the binder passes `met_pata` / `met_pataBig` from the pointer
table at `0x80B5FAC0` and `g3d/met_pata.brres` / `g3d/met_pataBig.brres` from
`0x80B5FAF8`, and looks up the `wait` clip at `0x80B5FB08`.

- `0x80AE0560` is the model binder: it resets the `+0x528` loader, loads the
  archive into `+0x544`, binds the `+0x548` matrix, attaches the `wait` clip to
  `+0x588` and scales that second matrix by the truncated `+0x5B0` factor
  through the `2^52` integer-to-double conversion.
- `0x80AE06D0` is the clip phase switch: `wait` / `ridden` / `rise` from
  `0x80B5FB28`, each with its own flag word from `0x80B30CF8`, storing the new
  phase at `+0x81C`.
- `0x80AE0880` and `0x80AE0A20` are the per-player pair. The first advances
  the four counters at `0x80429F10 + 0x100` and plays the sound ladder
  `0x227..0x22F`, filling the BSS table at `0x80B76E98` on first use; the
  second sweeps the four `+0x820` contact slots and clears the ones whose
  player is gone, invulnerable or carrying collision bits `0x2000`, `0x4000`,
  `0x10000` in `+0x274`.
- `0x80AE0B30` is the contact reaction: it picks one of four knock-back speed
  pairs from `0x80B30D0C..0x80B30D28` by the `+0x818` variant and the player's
  invulnerability flags, arms the `0x3C`-frame timer at `+0x814` and advances
  the clip phase once the `+0x588` clip finishes.
- `0x80AE0D30` is the frame handler: it sweeps the three contact lists
  `+0x5D0`/`+0x62C`/`+0x688`, and, when nothing touches the actor, snaps the
  height `+0xB0` onto the `+0x808` target in the direction the `+0x818`
  variant selects; the accumulated mask is stored back to `+0x830`.
- `0x80AE1080` integrates `+0xEC` with `+0x114` against the `+0xF8` limit and
  then runs the handler and the `+0x588` slot `+0x1C`.

**Cluster 2, `0x80AE1430..0x80AE2D00`, the `0xAB0`-byte `wing_pata` sibling.**
The name is literal too: the two state entries bind the `flyA` clip at
`0x80B5FECC` and the `wing_pata` clip at `0x80B5FE7C`, and the node probe
looks up `skl_root_shell` at `0x80B5FE88`.

- `0x80AE1430`, `0x80AE1550`, `0x80AE1580` are the shared release/forwarder
  trio (`0x8015F800`/`0x802B9280`, then two 48-byte `0x802DCBFC` thunks).
- `0x80AE15B0` is the constructor: it allocates `0xAB0` bytes, runs the
  non-standard base constructor `0x80037EF0`, and builds the two
  `0x80B330E0` holders at `+0x83C`/`+0xA5C`, the `0x802D7AA0` helper at
  `+0x8D8` (vtable `0x8032989C`, slot `+0x98` invoked immediately) and the
  final class vtables `0x80B5B440`/`0x80B5FED4`. It is the fifth copy of the
  descriptor shape first documented for `daEnTogeTekkyu_c`.
- `0x80AE19B0`, `0x80AE1A00`, `0x80AE1A50`, `0x80AE1CC0`, `0x80AE1D30` are the
  owner/release/turn block. They reuse the `0x80ADxxxx` world-map base helpers
  that the `Wm_en_quicksand` block already translated: `0x80AD0070` retire,
  `0x80AD0440` matrix rebuild, `0x80AD07D0` turn, `0x80AD0E40` clip binder and
  `0x80AD1010` landing smoke.
- `0x80AE22C0`, `0x80AE2330`, `0x80AE23A0`, `0x80AE2410` are the four
  `0.015625`-per-frame speed ramps: the first pair moves the lateral speed
  `+0xE8`, the second pair the forward speed `+0xEC`, each towards `+-1.0` or
  towards `0.0` depending on the side byte `+0xA94`.
- `0x80AE2570` is the two-holder clip binder for `+0xA1C`/`+0xA5C`;
  `0x80AE25E0` and `0x80AE2800` are the state `4` and state `3` entries, and
  `0x80AE26C0`, `0x80AE2910`, `0x80AE2A70`, `0x80AE2B50`, `0x80AE2D00` are the
  `+0x360` phase machine and the two base frame stacks around the `+0x1EC`
  ground probe.

### Identity evidence

- **`met_pata` is read, not guessed.** `0x80AE0560` passes `0x80B5FAA8`
  (`met_pata`), `0x80B5FAB4` (`met_pataBig`), `0x80B5FAC8`
  (`g3d/met_pata.brres`) and `0x80B5FADC` (`g3d/met_pataBig.brres`) to
  `0x800DF180`/`0x80239E30`, and the literal `wait` at `0x80B5FB08` to
  `0x8023A0B0`.
- **`wing_pata` is read, not guessed.** `0x80AE25E0` and `0x80AE2800` pass
  `0x80B5FE7C` (`wing_pata`) and `0x80B5FECC` (`flyA`) through
  [`fn_80ae2570`] and `0x80AD0E40`; `0x80AE1CC0` passes `0x80B5FE88`
  (`skl_root_shell`) to `0x8023B200`.
- **The two classes are siblings, not copies.** Cluster 1 works on
  `+0x528`/`+0x544`/`+0x548`/`+0x588` with variant `+0x818`; cluster 2 works on
  `+0x7FC`/`+0xA1C`/`+0xA5C` with model pointer `+0xA18`. Only the shared
  `0x80ADxxxx` base helpers and the asset family are common.
- **`0x80AE15B0` is the same descriptor shape as `daEnTogeTekkyu_c`.** Both
  use base constructor `0x80037EF0`, a `0x7DC` model holder, a `0x7FC` owner, a
  `0x80B330E0` holder at `+0x83C`, a `0x802D7AA0` helper whose slot `+0x98` is
  invoked immediately, and the `0x803295D8`/`0x80329548` callback pair. The
  sizes differ (`0xAB0` here, `0x9E0` there), so it is a separate class, not a
  duplicate.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AE0560` | `fn_80ae0560` | 364 | high |
| `0x80AE06D0` | `fn_80ae06d0` | 172 | medium |
| `0x80AE0880` | `fn_80ae0880` | 408 | medium |
| `0x80AE0A20` | `fn_80ae0a20` | 268 | high |
| `0x80AE0B30` | `fn_80ae0b30` | 504 | high |
| `0x80AE0D30` | `fn_80ae0d30` | 800 | medium |
| `0x80AE1080` | `fn_80ae1080` | 148 | high |
| `0x80AE1430` | `fn_80ae1430` | 88 | high |
| `0x80AE1550` | `fn_80ae1550` | 48 | high |
| `0x80AE1580` | `fn_80ae1580` | 48 | high |
| `0x80AE15B0` | `fn_80ae15b0` | 332 | high |
| `0x80AE19B0` | `fn_80ae19b0` | 80 | medium |
| `0x80AE1A00` | `fn_80ae1a00` | 80 | high |
| `0x80AE1A50` | `fn_80ae1a50` | 124 | high |
| `0x80AE1CC0` | `fn_80ae1cc0` | 112 | medium |
| `0x80AE1D30` | `fn_80ae1d30` | 140 | medium |
| `0x80AE22C0` | `fn_80ae22c0` | 104 | high |
| `0x80AE2330` | `fn_80ae2330` | 100 | high |
| `0x80AE23A0` | `fn_80ae23a0` | 104 | high |
| `0x80AE2410` | `fn_80ae2410` | 100 | high |
| `0x80AE2570` | `fn_80ae2570` | 112 | medium |
| `0x80AE25E0` | `fn_80ae25e0` | 208 | medium |
| `0x80AE26C0` | `fn_80ae26c0` | 320 | high |
| `0x80AE2800` | `fn_80ae2800` | 252 | medium |
| `0x80AE2910` | `fn_80ae2910` | 340 | medium |
| `0x80AE2A70` | `fn_80ae2a70` | 208 | medium |
| `0x80AE2B50` | `fn_80ae2b50` | 364 | medium |
| `0x80AE2D00` | `fn_80ae2d00` | 376 | medium |

## Deliberate skips

- **`0x80AE0780` (248 bytes) is not translated.** Its export is built from
  `unaff_GQR0`, `in_f30`/`in_f31` and `CONCAT12`/`CONCAT13` fragments that
  reconstruct the scale factor of a paired-single load; the real operand is
  not recoverable from the C. `fn_80ae0880` still calls it by address through
  [`W89_FN_80AE0780`] with a comment, exactly as the original does.
- **`0x80AE2250` (108 bytes) is not translated.** Same defect: `unaff_GQR3`,
  `ldexpf` and `CONCAT22` build the float form of the halfword angle at
  `+0xAA8` before `0x80237BD0`; the scaling that connects them is not in the
  export. `fn_80ae2910` calls it by address through [`W89_FN_80AE2250`].
- **`0x80AE06D0` keeps its two `GQR0`/`ldexpf` fragments.** They are dead
  quantiser fiction (the branch is never taken with the shipped constant), so
  the body reproduces the real work: table lookup, attach, blend and the phase
  store. The confidence is `medium` for the same reason.
- **The gap `0x80AE16FC..0x80AE19B0` has no functions.** `0x80AE15B0` is 332
  bytes and therefore ends at `0x80AE16FC`, yet the next `functions.tsv` entry
  is `0x80AE19B0`. The chain walker and `:`-suffix name predicate that usually
  sit between a constructor and its state steps were therefore never
  identified by Ghidra in this bucket, so no body could be written for them.
- **The remaining 86 functions of the bucket stay untranslated.** After this
  pass the untouched runs are `0x80AE33C0..0x80AE4A50` (the `met_pata`
  constructor and its first state helpers, which sit before the already-ledgered
  `0x80AE3080` and around the ledgered `0x80AE41C0..0x80AE5110` block) and
  `0x80AEAB90..0x80AEFCF0` (about forty functions at the tail). The largest
  clean candidates there are `0x80AEB450` (700 bytes), `0x80AEDD40` (1,208) and
  `0x80AEF130` (1,180).
- **`fn_80ae0880` and `fn_80ae0d30` are `medium`, not `high`.** `0x802DCD74` is
  the section prologue, printed as a 64-bit return: the low word is used as a
  non-zero guard and the high word as the actor. The body reproduces that shape
  rather than pretending the extra word is a normal parameter, and the roles of
  `+0x808`/`+0x831` are inferred from their comparisons.
- **`fn_80ae19b0`, `fn_80ae1a50` and `fn_80ae0d30` are `medium` because of
  their base calls.** `0x80AD0070` and `0x80AD0440` are printed with no
  arguments; the receiver in `r3` can only still be this actor, since the call
  is the first instruction of the body, so it is passed explicitly and the doc
  says so.
- **`fn_80ae2570` is `medium`.** The export prints `0x8023A0B0` with only the
  holder argument, but the caller passes the clip-name pointer in that slot and
  the lookup cannot work without it, so the name is forwarded and the doc marks
  that inference.
- **`fn_80ae25e0` and `fn_80ae2800` are `medium`.** Bit 8 of the actor flag
  word at `+0x4` selects the reversed spawn direction; the bit test and every
  store are literal, but the flag itself has no printed name.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 28 sizes sum to
  6,304 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 28 was
  already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave89` reports 28 rows for this CSV and
  `OK`.
- `cargo check --message-format=short` finishes with no diagnostic naming
  `src/game/enemies.rs`.
