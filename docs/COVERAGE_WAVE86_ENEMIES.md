# Coverage - Wave 86 (enemies: the `Wm_en_burst_water` actor and the `0x878`-byte `walk` actor)

## How to read this file

This page records the Wave 86 `game::enemies` lane: two runs in the
`0x80AB0000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs`. Every row below is one real `pub unsafe fn fn_<addr>`
body in that file, and the `evidence` column of the machine-readable mirror
(`ledger/agents/wave86-enemies.csv`) names the Ghidra C export each body was
reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Data
constants, tables and lazily written BSS tables are cited by address and read
through `platform::read_u32` / `read_f32` / `read_u16` / `read_u8`, never
transcribed as invented values. Where a constant's meaning matters for the
translation, its value was read out of the REL image and recorded in the
constant's doc comment.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 29 | 5,212 | 15 | 14 |

The bucket holds 188 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 70 were already in
`ledger/functions.csv` before this pass (waves 56, 72 and neighbours), 29 are
claimed here and 89 remain.

## What this pass covers

Two runs. They are adjacent in the bucket but belong to different classes, so
they are documented separately.

**Cluster 1, `0x80AB1A10..0x80AB23F0`, the `Wm_en_burst_water` world-map
enemy.** The class is identified by two strings the code passes to the named
effect spawner `0x8016C890`: `Wm_en_burst_water01` at `0x80B54C9C` and
`Wm_en_burst_water02` at `0x80B54CB0`. The run is everything between the
actor's spawn gate and its two variant entry points:

- `0x80AB1A10` clears the byte `+0x99A` once the three-argument contact probe
  `0x80097B30` reports contact, short-circuiting on the flag first;
- `0x80AB1A80` writes the three-word displacement from `this` to the player's
  probe point, `(+0xAC/+0xD0) - this+0xAC` and its `Y`/`Z` siblings;
- `0x80AB1B10` is the spawn gate: `0x80064740` returns the nearest
  participant, `0x80056CF0` must report state `0x3A`, the displacement must
  point away from the actor (side `0`) or toward it (side `1`), stay within
  `128.0` units and within `+-0x2000` of the `0x801611C0` aim angle. On
  success the displacement is cached at `+0x8B4..+0x8BC` and the player's
  character byte is copied into `+0x99C`;
- `0x80AB1CA0` clamps the height `+0xB0` to the stage word
  `0x80429DD0 + 0x8FEC4` minus `16.0`, the same stage field waves 51 and 84
  already use;
- `0x80AB1CE0` revalidates that cached player on later frames;
- `0x80AB1D60` turns the actor toward a direction vector with the `0x4000`
  quarter-turn mirror applied on the left-facing side;
- `0x80AB1DF0` is a `bctr` tail call through vtable slot `+0x1C` of the
  `+0x544` holder;
- `0x80AB1E00` is the visibility entry: sound `0x209`, the two named effects
  at `+0x8A8`, and the `0x80B75110` dispatch;
- `0x80AB1EC0` and `0x80AB2050` are the frame pair around it, the second
  carrying the heading recentre, the two `+0x890`-based limit dispatches, the
  `+0x394` owner query that dispatches `0x80B75050`, the tail call and the
  `0x40`-frame countdown;
- `0x80AB1F10` and `0x80AB1F70` are the three-axis speed damping
  (`+0x964` target, `+0x95C` rate, `+0x8FC` current) and the velocity
  integrator (`+0x960` acceleration, `+0x978` floor, `+0x900` value);
- `0x80AB1FA0` sweeps `+0x104` between `+-0x18E1` and advances the
  four-value byte `+0x99D` at each reversal;
- `0x80AB2280` and `0x80AB23F0` are the two variant entry points; both rebind
  through `0x80AB1780` with blend rate `1.0` and arm the `+0x998` timer.

**Cluster 2, `0x80AB3100..0x80AB4510`, the `0x878`-byte `walk`-clip actor.**
This is the neighbouring class in the same bucket. Every variant binder looks
up the clip named `walk` (`0x80B5513C`) on the model at `+0x540` and attaches
it to `+0x584`, so the run is the actor's whole frame surface:

- `0x80AB3100` releases the object (`0x8015F800`, then the heap release for a
  positive flag) and `0x80AB3220`/`0x80AB3250` are the two `0x802DCBFC`
  deleting-destructor forwarders;
- `0x80AB3280` is the allocating constructor: `0x878` bytes, the
  `+0x524`/`+0x544`/`+0x584` holder layers, one `0x802D7AA0` sub-object at
  `+0x5F8` and a second at `+0x720` whose virtual slot `+0x98` runs before the
  final class vtable `0x80B55148` is installed at `+0x60`;
- `0x80AB3AE0` advances the heading by `0x800` and runs the two base steps,
  then calls `0x80AB4510` when the global mode word at `0x80429CA0` is `1`;
- `0x80AB3B40` and `0x80AB3C90` are the two variant binders. Both lazily fill
  a speed table in BSS (`0x80B75308` with four floats, `0x80B75320` with
  eight), select the pair from bits `21..24` of the spawn flags, copy the
  per-side heading from the halfword table at `0x8042C1A4` and set the
  `+0x360` state. The second additionally sets `+0x871` and resets the
  `+0x148` record;
- `0x80AB3E00` is the four-state fall-and-bounce machine: `-0.75` damping,
  then a zeroed vertical speed with `+0x11C = |X| / 10.0`, then the side swap
  once the `+0x500` counter expires, then the heading return and the
  `0x803534EC` dispatch;
- `0x80AB4070` switches to `+0x36C = 4` and dispatches `0x80358104` when a
  push record is present at `+0x368`;
- `0x80AB40F0` clears the five speed words and rebinds `walk` at `2.0`;
- `0x80AB41A0` lifts the actor to `1500.0` and clears bit 16 of `+0x4FC` when
  the contact query sets bit 0;
- `0x80AB4250` seeds the four variant limit/speed pairs from the tables at
  `0x80B2FC28`/`0x80B2FC38`/`0x80B2FC48`/`0x80B2FC58`;
- `0x80AB4380` snaps the actor to that limit, sets bit 3 of `+0x178`, raises
  `+0xB4` by `1500.0 + 16.0 * ((spawn flags >> 24) & 0xF)` and dispatches;
- `0x80AB4510` is the four-phase touch-down sequence over `+0x870`.

### Identity evidence

- **`Wm_en_burst_water01`/`02` are read, not guessed.** Both strings are
  passed to `0x8016C890` in `0x80AB1E00` and are laid out back to back at
  `0x80B54C9C` and `0x80B54CB0` in the REL image.
- **`walk` is a clip name, not a state name.** `0x80B5513C` is the exact
  address the six binders pass to the lookup `0x8023A0B0` alongside the model
  holder at `+0x540`.
- **The two classes are separate.** Cluster 1 keys on `+0x99x`, `+0x8B4` and
  `+0x890`; cluster 2 keys on `+0x360`, `+0x348`, `+0x500`/`+0x504`, `+0x868`
  and `+0x872`. The only shared offsets are the base actor fields and the
  `+0x524`/`+0x544`/`+0x584` holder chain.
- **The class names are not printed anywhere in either run.** Both are named
  by structure only; `0x80AB3280` writes `0x80304938` and then `0x80B55148`
  at `+0x60`, and cluster 1's initialiser in the untranslated range rebinds
  `0x80AB1780`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AB1A10` | `fn_80ab1a10` | 100 | high |
| `0x80AB1A80` | `fn_80ab1a80` | 132 | high |
| `0x80AB1B10` | `fn_80ab1b10` | 400 | medium |
| `0x80AB1CA0` | `fn_80ab1ca0` | 64 | high |
| `0x80AB1CE0` | `fn_80ab1ce0` | 124 | medium |
| `0x80AB1D60` | `fn_80ab1d60` | 140 | medium |
| `0x80AB1DF0` | `fn_80ab1df0` | 16 | medium |
| `0x80AB1E00` | `fn_80ab1e00` | 180 | high |
| `0x80AB1EC0` | `fn_80ab1ec0` | 72 | high |
| `0x80AB1F10` | `fn_80ab1f10` | 88 | high |
| `0x80AB1F70` | `fn_80ab1f70` | 44 | high |
| `0x80AB1FA0` | `fn_80ab1fa0` | 136 | high |
| `0x80AB2050` | `fn_80ab2050` | 328 | medium |
| `0x80AB2280` | `fn_80ab2280` | 64 | medium |
| `0x80AB23F0` | `fn_80ab23f0` | 84 | medium |
| `0x80AB3100` | `fn_80ab3100` | 88 | high |
| `0x80AB3220` | `fn_80ab3220` | 48 | medium |
| `0x80AB3250` | `fn_80ab3250` | 48 | medium |
| `0x80AB3280` | `fn_80ab3280` | 328 | high |
| `0x80AB3AE0` | `fn_80ab3ae0` | 88 | high |
| `0x80AB3B40` | `fn_80ab3b40` | 304 | medium |
| `0x80AB3C90` | `fn_80ab3c90` | 344 | medium |
| `0x80AB3E00` | `fn_80ab3e00` | 504 | medium |
| `0x80AB4070` | `fn_80ab4070` | 116 | medium |
| `0x80AB40F0` | `fn_80ab40f0` | 160 | high |
| `0x80AB41A0` | `fn_80ab41a0` | 164 | high |
| `0x80AB4250` | `fn_80ab4250` | 284 | high |
| `0x80AB4380` | `fn_80ab4380` | 392 | high |
| `0x80AB4510` | `fn_80ab4510` | 372 | medium |

## Deliberate skips

- **`0x80AB0570` (664 bytes), `0x80AB0870` (204), `0x80AB0C00` (440),
  `0x80AB0EE0` (640), `0x80AB1160` (384), `0x80AB12E0` (244),
  `0x80AB13E0` (304) and `0x80AB1510` (624) stay untranslated.** They form the
  `Wm_en_burst_water` constructor plus its two frame initialisers and the
  three shared progress helpers. `0x80AB0570` alone writes eleven constant
  blocks, two `0x8008C3E0` binders, the five-argument collision binder
  `0x8006D090` and one virtual dispatch, and the helpers it calls
  (`0x80AB0EE0`, `0x80AB1160`, `0x80AB1510`) are larger than anything this
  lane's budget allows to do honestly. Nothing translated here depends on
  their bodies: the calls that reach them are recorded as addresses or left
  to the callers, and `0x80AB1780`, which cluster 1 does call, stays an
  address for the reason below.
- **`0x80AB1780` (172 bytes) stays untranslated.** Its export begins with
  `unaff_GQR0` paired-single conversions (`ldexpf` of the `GQR0` fields), so
  the two `f64` arguments cannot be reconstructed from the C alone. Waves 55,
  83, 84 and 85 recorded the same limit for their own GQR exports;
  `fn_80ab2280` and `fn_80ab23f0` call it through an address with the literal
  `1.0` blend rate.
- **`0x80AB1830` (92 bytes) stays untranslated.** Ghidra truncated its control
  flow at a bad instruction after the `0x40` countdown store, so there is no
  instruction sequence to reconstruct. `fn_80ab2050` calls it as an address.
- **`0x80AB1890` (104 bytes) stays untranslated.** It is the `+0x164`/`+0x7DC`
  tangent helper of the untranslated initialiser block; it is only reachable
  from `0x80AB0570`, so it is deferred with its caller.
- **`0x80AB1900` (256 bytes), `0x80AB1A00` (8) and `0x80AB21D0` (176) stay
  untranslated.** The first two carry `unaff_GQR0`/`unaff_GQR3` artefacts, the
  third is a `halt_baddata` export. They sit inside the cluster-1 range and
  are the only holes left in it.
- **`0x80AB29C0` (376 bytes) and `0x80AB5710` (1,064 bytes) stay
  untranslated.** Both are `unaff_GQR0` exports: the first closes the
  cluster-1 neighbourhood, the second is the frame step that wave 56 left
  inside its `lift_taru` block.
- **`fn_80ab4510` is `medium`, not `high`.** Its export writes the
  `countLeadingZeros(v) >> 5` predicate explicitly; the body reproduces it as
  the equivalent `v == 0` and says so in a comment rather than calling a
  count-leader intrinsic that the crate does not have.
- **`fn_80ab1b10` is `medium`, not `high`.** Ghidra prints the angle limit as
  `(int)DAT_80b54bb8 ^ 0x80000000`. The REL bytes are `lha r6,0x4bb8(r4)`,
  `oris r6,r6,0x8000` and the `1.5 * 2^52` subtraction, i.e. the
  signed-int-to-double idiom for the halfword `0x4000`; the limit is therefore
  `16384 * 0.5 = 8192` (`+-45` degrees). The body documents the decode.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 29 sizes sum to
  5,212 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none was already in
  `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave86` reports 29 rows for this CSV and `OK`.
- `cargo check --message-format=short` produces no diagnostic naming
  `src/game/enemies.rs`.
- `src/game/enemies.rs` is UTF-8 without a BOM and uses LF line endings; the
  47 non-ASCII doc-comment lines it contains (em dashes and the `±` sign) are
  single-encoded, i.e. every one decodes to exactly one character.
