# Coverage - Wave 88 (`game::bases`, `0x8069` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** under the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader chooses the real load address, so the numbers only
identify positions inside the REL image. Cross-module pointers (`0x8035xxxx`,
`0x8037xxxx`, `0x8042xxxx`, `0x807Cxxxx`) are `main.dol` data addresses
imported by the module and keep their absolute values.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`. "Role" below is the
plain-English behaviour. Confidence is `high` when guards, stores, constants
and call arguments are literal, and `medium` when the role of a field, a
global, a callee or a dropped argument had to be inferred (those bodies carry
`// observed:`/`// inferred:`/`// unknown:` notes). No body in this pass is
`low`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 3,856 | 24 | 6 |

## What this pass covers

The `0x8069` bucket holds 185 functions; 49 were already in
`ledger/functions.csv` when this pass began, leaving 136. This pass takes 30 of
the remaining bodies: one contiguous actor cluster plus the shared release and
RTTI helper families that bracket it.

- **Sand-pillar actor, `0x80694DC0..0x806960F0` (17 bodies).** The actor loads
  `g3d/sandpillar.brres` and owns a ten-entry target ring at `+0x548`
  (stride `0x154`) with the live count at `+0x544`. [`fn_80695170`] rejects a
  target that is already cached, [`fn_80695260`] appends to the first free
  slot, and [`fn_806950f0`] walks the `+0x12AC` list and notifies state-1/2
  targets with `0x80056C70(target, 0x5B)`. [`fn_80694dc0`] rebuilds the model
  transform from the position at `+0xAC` and the three angles at
  `+0x100..+0x104`; [`fn_80694eb0`] looks up the `ef_sandpillar` record and
  snapshots its transform at `+0x1290`. The sign model is seeded by
  [`fn_80695b20`] (clip `sandpillar` + animation `sandpillar_end`) and
  [`fn_80696050`], spawned by [`fn_80695830`], counted down by
  [`fn_80695a20`], and resized by
  [`fn_80695700`]/[`fn_80695760`]/[`fn_806957b0`]. [`fn_80695c20`] drives the
  climb and the two `Wm_ob_sandpillar01/02` bursts; [`fn_806955d0`] and
  [`fn_80695990`] retire one target and the `+0x129C` effect holder, and
  [`fn_806960f0`] is the per-frame sign dispatcher.
- **Release wrappers, `0x80693530`, `0x80696830`, `0x80697550`, `0x80698EE0`,
  `0x80698F40`, `0x8069D7B0`, `0x8069D810` (7 bodies).** Seven byte-identical
  two-argument wrappers: null guard, `0x8015F800(object, 0)`, and
  `0x802B9280(object)` when `free > 0`.
- **Name/type getters, `0x80698FA0..0x80698FC0` and `0x8069D870..0x8069D8A0`
  (7 bodies).** Twelve-byte bodies that return one literal `main.dol` pointer
  (`0x80357E9C`, `0x80357E5C`, `0x80357E1C`, `0x80357DDC`).

No new host-side helper was needed: the pass reuses `abi_call1..6`,
`vmethod0`, `vmethod1`, `actor_method0`, `call_f32x3_u32`, `call_f32_u32`,
`call_f32_u32x2` and `call_u32x2_f32`.

## Functions

| address | function | bytes | confidence | role |
|---|---|---:|---|---|
| `0x80693530` | `fn_80693530` | 88 | high | release wrapper |
| `0x80694DC0` | `fn_80694dc0` | 228 | high | rebuild model transform |
| `0x80694EB0` | `fn_80694eb0` | 128 | medium | snapshot sand-burst alias, height |
| `0x806950F0` | `fn_806950f0` | 116 | high | notify target list |
| `0x80695170` | `fn_80695170` | 232 | high | ring duplicate test |
| `0x80695260` | `fn_80695260` | 100 | high | append ring record |
| `0x806955D0` | `fn_806955d0` | 212 | medium | state smoke / retire target |
| `0x80695700` | `fn_80695700` | 92 | medium | seed two scale records |
| `0x80695760` | `fn_80695760` | 76 | high | step and clamp sign size |
| `0x806957B0` | `fn_806957b0` | 124 | medium | publish effect block |
| `0x80695830` | `fn_80695830` | 284 | high | reset ring, spawn sign |
| `0x80695990` | `fn_80695990` | 68 | high | release effect holder |
| `0x80695A20` | `fn_80695a20` | 244 | high | sign countdown and event |
| `0x80695B20` | `fn_80695b20` | 228 | high | bind clip and end animation |
| `0x80695C20` | `fn_80695c20` | 412 | medium | climb step and two bursts |
| `0x80696050` | `fn_80696050` | 132 | high | rebind end clip |
| `0x806960F0` | `fn_806960f0` | 472 | medium | per-frame sign dispatcher |
| `0x80696830` | `fn_80696830` | 88 | high | release wrapper |
| `0x80697550` | `fn_80697550` | 88 | high | release wrapper |
| `0x80698EE0` | `fn_80698ee0` | 88 | high | release wrapper |
| `0x80698F40` | `fn_80698f40` | 92 | high | release wrapper |
| `0x80698FA0` | `fn_80698fa0` | 12 | high | constant `0x80357E5C` |
| `0x80698FB0` | `fn_80698fb0` | 12 | high | constant `0x80357E1C` |
| `0x80698FC0` | `fn_80698fc0` | 12 | high | constant `0x80357DDC` |
| `0x8069D7B0` | `fn_8069d7b0` | 88 | high | release wrapper |
| `0x8069D810` | `fn_8069d810` | 92 | high | release wrapper |
| `0x8069D870` | `fn_8069d870` | 12 | high | constant `0x80357E9C` |
| `0x8069D880` | `fn_8069d880` | 12 | high | constant `0x80357E5C` |
| `0x8069D890` | `fn_8069d890` | 12 | high | constant `0x80357E1C` |
| `0x8069D8A0` | `fn_8069d8a0` | 12 | high | constant `0x80357DDC` |

## Address range

`0x80693530`, the contiguous run `0x80694DC0..0x806960F0`, and three helper
tails at `0x80696830`, `0x80697550..0x80698FC0` and `0x8069D7B0..0x8069D8A0`.
After this pass the bucket still has 106 untranslated functions.

## Deliberate skips

The bodies below were read and rejected rather than guessed; the rest of the
bucket was simply not reached.

- `0x80694F30` (432 bytes) is the target-acquisition test. Its first call to
  `0x8008C550` is printed with no arguments (`func_0x8008c550()`), so the actor
  whose frustum is tested is unrecoverable; the second call shows
  `(param_2)`. Writing the first comparison would require guessing a lost
  register.
- `0x806952D0` (488 bytes), `0x806954C0` (192 bytes) and `0x80695E00`
  (584 bytes) are the ring teardown/update bodies. Their exports carry six to
  twelve `unaff_GQR0`/`ldexpf` placeholders and unreachable-block warnings, so
  the float conversions cannot be proven; `0x806954C0` additionally loses an
  argument through `extraout_r4`.
- `0x80695580` (64 bytes) and `0x806956B0` (64 bytes) both hit
  `halt_baddata` / "could not recover jumptable" in their first branch. The
  surviving second branch is one `+0x240` virtual, but the truncated branch
  cannot be reconstructed.
- `0x80695C20`'s sibling `0x80695E00` was rejected for the same GQR reason
  even though its high-level shape repeats [`fn_80695c20`].
- `0x80696950` and `0x80696980` (48 bytes each) are the `0x802DCBFC(param_2,
  param_2, param_1)` adapters already documented as unreconstructible in
  `docs/COVERAGE_WAVE86_BASES.md` and `docs/COVERAGE_WAVE87_BASES.md`.
- `0x80697090`, `0x80697C30`, `0x80697D00`, `0x80697DC0`, `0x80697F10` and the
  rest of the `0x80697..0x8069F` tail belong to a different actor family whose
  first bodies (`0x80697090`) print uninitialised locals and whose neighbours
  reference paired-single conversion tables; they were left for a later pass
  that can recover the GQR context.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 3,856 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the 49 pre-existing rows of
  this bucket were listed before writing and none is in this pass.
- `node tools/check-lane-csv.mjs wave88` reports
  `wave88-bases.csv: 30 rows`, `unique addresses: 58` (shared with the player
  lane), `OK`.
- `cargo check --message-format=short` produces no diagnostic for
  `src/game/bases.rs`.
