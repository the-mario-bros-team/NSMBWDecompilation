# Coverage - Wave 46 (`d_enemiesNP`, enemies lane)

## How to read this file

This page records one lane of Wave 46: the `0x80A10000` bucket of REL module 3,
`d_enemiesNP`, promoted to `src/game/enemies.rs`. Every row corresponds to a
real `pub unsafe fn` body in that file; the evidence column of the lane CSV
names the Ghidra C export each body was reconstructed from.

All addresses are **synthetic analysis addresses** from `tools/rel2elf.mjs`
(base `0x80A00000`). They are not runtime addresses; the REL has no embedded
load address.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 29 | 4,360 | 17 | 12 |

Two of the twelve `medium` rows are `partial`: the 48-byte
deleting-destructor forwarders at `0x80A11500` and `0x80A11530` keep their
visible call but cannot model the hidden `r12` this-adjust operand (the same
limitation the earlier `0x80A6C810`-family forwarders carry).

## What this pass covers

- **One complete actor block, `0x80A111B0..0x80A12C80`.** The constructor at
  `0x80A11560` allocates `0x9F0` bytes and installs class vtable `0x80B356C0`;
  the binder at `0x80A11EE0` loads `g3d/obj_turara_rakka_Big.brres` with the
  `obj_turara_rakka_Big` node and the `gura_anim` clip. The block is a large
  falling icicle used by the world-map scenes. `// inferred:` - the export
  names no class, so the label rests on the model and clip strings and the six
  state descriptors.
- **State plumbing.** The two descriptor normalisers (`0x80A111B0`,
  `0x80A11210`), the constant producer (`0x80A11270`), the `+0x30` chain
  walker (`0x80A11360`), the name-suffix predicate (`0x80A11440`) and the two
  48-byte deleting-destructor forwarders (`0x80A11500`, `0x80A11530`).
- **Model and animation setup.** The `0x9F0`-byte constructor
  (`0x80A11560`), the destructor (`0x80A11A10`, which releases the `+0x394`
  object, the four `0xE4`-byte holders at `+0x588` and the `+0x954` object),
  the `obj_turara_rakka_Big` binder (`0x80A11EE0`) and the clip selector
  (`0x80A12070`).
- **Aim and projectile pair.** The four-slot player proximity test
  (`0x80A12120`), the id-500 lookups (`0x80A12200`, `0x80A124D0`), the aim
  window test (`0x80A12240`), the spawner (`0x80A122F0`) and the lifetime
  watch (`0x80A12520`). The aim state itself is `0x80A12420`.
- **Fall animation.** The yaw rebuild (`0x80A12350`), the spin step
  (`0x80A12370`), the growth clamp (`0x80A12600`), the hit-state check
  (`0x80A126F0`), the vanish handler (`0x80A12770`), the re-entry initialiser
  (`0x80A12820`), the main per-frame handler (`0x80A128D0`, 740 bytes: landing
  probe, three one-shot effects and the three-phase `+0x9D0` tilt ramp), the
  sway step (`0x80A12BE0`) and the vertical-speed damper (`0x80A12C80`).
- **Contact callbacks.** `0x80A11CB0` and its wrapper `0x80A11E20` forward the
  touching actor through the `+0x3F0`/`+0x3F4` vtable slots.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A111B0` | `fn_80a111b0` | 88 | high |
| `0x80A11210` | `fn_80a11210` | 92 | high |
| `0x80A11270` | `fn_80a11270` | 12 | high |
| `0x80A11360` | `fn_80a11360` | 224 | medium |
| `0x80A11440` | `fn_80a11440` | 136 | medium |
| `0x80A11500` | `fn_80a11500` | 48 | medium (partial) |
| `0x80A11530` | `fn_80a11530` | 48 | medium (partial) |
| `0x80A11560` | `fn_80a11560` | 216 | high |
| `0x80A11A10` | `fn_80a11a10` | 156 | high |
| `0x80A11CB0` | `fn_80a11cb0` | 116 | medium |
| `0x80A11E20` | `fn_80a11e20` | 40 | high |
| `0x80A11EE0` | `fn_80a11ee0` | 400 | high |
| `0x80A12070` | `fn_80a12070` | 152 | medium |
| `0x80A12120` | `fn_80a12120` | 212 | medium |
| `0x80A12200` | `fn_80a12200` | 56 | medium |
| `0x80A12240` | `fn_80a12240` | 176 | medium |
| `0x80A122F0` | `fn_80a122f0` | 96 | medium |
| `0x80A12350` | `fn_80a12350` | 32 | high |
| `0x80A12370` | `fn_80a12370` | 136 | high |
| `0x80A12420` | `fn_80a12420` | 172 | high |
| `0x80A124D0` | `fn_80a124d0` | 52 | high |
| `0x80A12520` | `fn_80a12520` | 132 | high |
| `0x80A12600` | `fn_80a12600` | 196 | high |
| `0x80A126F0` | `fn_80a126f0` | 84 | medium |
| `0x80A12770` | `fn_80a12770` | 176 | medium |
| `0x80A12820` | `fn_80a12820` | 148 | high |
| `0x80A128D0` | `fn_80a128d0` | 740 | high |
| `0x80A12BE0` | `fn_80a12be0` | 160 | high |
| `0x80A12C80` | `fn_80a12c80` | 64 | high |

## Deliberate skips

- **`0x80A12110` (8 bytes).** The Ghidra export for this function is a single
  `halt_baddata()` with the warning "control flow encountered bad instruction
  data". No instruction sequence, call or store can be recovered, so no body
  was written and no ledger row exists for it. Its three callers
  (`0x80A126F0`, `0x80A12770`, `0x80A128D0`) still call the raw address and
  carry an `// unknown:` comment instead of guessing behaviour.
- **`0x80A10220`..`0x80A111B0` (the earlier part of the bucket).** Those
  functions belong to a different actor and were left for a later pass; this
  lane deliberately took the contiguous, self-contained icicle block so that
  every promoted body shares one class layout and can be cross-checked against
  its neighbours.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 29 sizes sum to
  4,360 bytes.
- Every evidence file exists at the cited repo-relative path.
- Every CSV row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`.
- No address was already in `ledger/functions.csv`; the bucket held only two
  earlier rows (`0x80A1BDA0`, `0x80A1C0E0`), neither of which is reused.
- `cargo check --message-format=short` reports no error or warning for
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave46` prints `OK`.
