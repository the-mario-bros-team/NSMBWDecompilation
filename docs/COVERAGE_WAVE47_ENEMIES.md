# Coverage - Wave 47 (`d_enemiesNP`, enemies lane)

## How to read this file

This page records one lane of Wave 47: the `0x80B10000` bucket of REL module 3,
`d_enemiesNP`, promoted to `src/game/enemies.rs`. Every row corresponds to a
real `pub unsafe fn` body in that file; the `evidence` column of the lane CSV
names the Ghidra C export each body was reconstructed from.

All addresses are **synthetic analysis addresses** from `tools/rel2elf.mjs`
(base `0x80A00000`). The REL has no embedded load address, so none of these are
runtime addresses.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 26 | 4,032 | 9 | 17 |

Two of the seventeen `medium` rows are `partial`: the 48-byte
deleting-destructor forwarders at `0x80B12F20` and `0x80B12F50` keep their
visible call but cannot model the hidden `r12` this-adjust operand (the same
limitation the earlier `0x80A6C810`-family and `0x80A11500` forwarders carry).

## What this pass covers

- **One complete actor block, `0x80B12BC0..0x80B13E60`.** The constructor at
  `0x80B12F80` allocates `0x698` bytes and installs class vtable `0x80B690E8`;
  the binder at `0x80B13370` loads `g3d/iron_ball.brres` with the `iron_ball`
  node. The handlers pass the effect names `Wm_en_iron`,
  `Wm_en_sndlandsmk_s` and `Wm_en_landsmoke_s`, which identify the block as a
  world-map hazard. `// inferred:` - the export names no class, so the label
  `iron_ball` rests on the model and effect strings.
- **Registration/dispose pair and constants.** The two callbacks
  (`0x80B12BC0`, `0x80B12C20`), the constant producers `0x80357BF4` and
  `0x80358004` (`0x80B12C80`, `0x80B12C90`), the `+0x30` chain walker
  (`0x80B12D80`) and the name-suffix predicate (`0x80B12E60`).
- **Forwarders and construction.** The two 48-byte deleting-destructor
  forwarders (`0x80B12F20`, `0x80B12F50`) and the `0x698`-byte constructor
  (`0x80B12F80`), which also primes the `+0x55C` helper through its `+0x98`
  slot.
- **Spawn path.** The spawn initialiser (`0x80B13040`, which recovers the
  actor from `0x802DCD74`), the input-mask/aim predicate (`0x80B13220`) and
  the side-byte selector (`0x80B13300`). The initialiser seeds the motion
  block at `+0xDC..+0xE4`, the `+0x318..+0x344` speed/scale pair and enters
  one of the four state descriptors `0x80B78D78`, `0x80B78DB8`, `0x80B78DF8`
  or `0x80B78E38` from the spawn flags at `+0x04`.
- **Contact and damage.** The contact predicate (`0x80B135E0`) and the impact
  responder (`0x80B13620`), which repositions the actor, requests sound
  `0x17B` and installs the nine-word hit record at `+0x414..+0x432`.
- **Animation and fall state machine.** The rate selector (`0x80B13840`), the
  `Wm_en_iron` effect (`0x80B138E0`), the landing smoke (`0x80B13950`), the
  empty hook (`0x80B13A00`), the effect-amount helper (`0x80B13A10`), the yaw
  step (`0x80B13A60`), the two fall steps (`0x80B13C30`, `0x80B13CE0`), the
  idle/release step (`0x80B13DB0`) and the main per-frame handler
  (`0x80B13E60`, 652 bytes).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80B12BC0` | `fn_80b12bc0` | 88 | high |
| `0x80B12C20` | `fn_80b12c20` | 92 | high |
| `0x80B12C80` | `fn_80b12c80` | 12 | high |
| `0x80B12C90` | `fn_80b12c90` | 12 | high |
| `0x80B12D80` | `fn_80b12d80` | 224 | medium |
| `0x80B12E60` | `fn_80b12e60` | 136 | medium |
| `0x80B12F20` | `fn_80b12f20` | 48 | medium (partial) |
| `0x80B12F50` | `fn_80b12f50` | 48 | medium (partial) |
| `0x80B12F80` | `fn_80b12f80` | 192 | high |
| `0x80B13040` | `fn_80b13040` | 480 | medium |
| `0x80B13220` | `fn_80b13220` | 216 | medium |
| `0x80B13300` | `fn_80b13300` | 100 | medium |
| `0x80B13370` | `fn_80b13370` | 172 | high |
| `0x80B135E0` | `fn_80b135e0` | 52 | medium |
| `0x80B13620` | `fn_80b13620` | 372 | medium |
| `0x80B137A0` | `fn_80b137a0` | 156 | medium |
| `0x80B13840` | `fn_80b13840` | 160 | medium |
| `0x80B138E0` | `fn_80b138e0` | 108 | medium |
| `0x80B13950` | `fn_80b13950` | 164 | medium |
| `0x80B13A00` | `fn_80b13a00` | 4 | high |
| `0x80B13A10` | `fn_80b13a10` | 80 | high |
| `0x80B13A60` | `fn_80b13a60` | 104 | high |
| `0x80B13C30` | `fn_80b13c30` | 128 | medium |
| `0x80B13CE0` | `fn_80b13ce0` | 144 | medium |
| `0x80B13DB0` | `fn_80b13db0` | 88 | medium |
| `0x80B13E60` | `fn_80b13e60` | 652 | medium |

## Deliberate skips

- **The rest of the `0x80B10000` bucket.** 180 other functions were left for
  later passes. In particular this lane did not take the `obj_door_torideboss`
  / `test_lift` block at `0x80B152D0..0x80B15A50` (its `0x8007FA40` effect
  call takes three `double` constants plus a negated `double` and a
  twelve-argument tail, so it needs a dedicated adapter), nor the later
  duplicated bands `0x80B18580..0x80B1B8F0` and `0x80B1C340..0x80B1FF30`.
- **The four state descriptors.** `0x80B78D78`, `0x80B78DB8`, `0x80B78DF8`
  and `0x80B78E38` are kept by address. Their file bytes are unrelocated
  pointer offsets, so no `StateID` string can be recovered from them and no
  name is claimed.
- **Opaque callees.** `0x802DCD74`/`0x802DCDC0` (spawn prologue/epilogue),
  `0x80064740` (spatial query), `0x80070CB0`, `0x80070760`, `0x800707E0`,
  `0x8006EF40`, `0x800957B0` and `0x80197F00` are kept by address, with their
  argument counts taken from the Ghidra call sites.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 26 sizes sum to
  4,032 bytes.
- Every evidence file exists at the cited repo-relative path.
- Every CSV row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`.
- No address was already in `ledger/functions.csv`; the bucket held five
  earlier rows (`0x80B14220`, `0x80B14560`, `0x80B146E0`, `0x80B150A0`,
  `0x80B151B0`), none of which is reused.
- `cargo check --message-format=short` reports no error or warning for
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave47` prints `OK`.
