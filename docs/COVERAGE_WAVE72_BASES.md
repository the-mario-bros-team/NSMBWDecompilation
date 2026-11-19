# Coverage - Wave 72 (`game::bases`, `0x8072` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address below is a **static analysis address** from the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader chooses the real load address, so the numbers only
identify positions inside the REL image. Cross-module pointers (`0x8006xxxx`,
`0x8019xxxx`, `0x8032xxxx`, `0x8042xxxx`) are `main.dol` addresses imported by
the REL and keep their absolute values.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; the
cited evidence files are the Ghidra C exports under
`work/ghidra/rel-d_basesNP/export/`. Confidence is `high` when the guards,
stores, constants and call arguments are literal, and `medium` when the role of
a field or of a callee had to be inferred (those bodies carry `// observed:`,
`// inferred:` or `// unknown:` notes).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 27 | 3704 | 14 | 13 |

## What this pass covers

The `0x8072` bucket held 221 functions at lane start; 68 of them were already in
`ledger/functions.csv`, leaving 153. That remainder is heavily fragmented - the
longest untouched run of adjacent bodies is four - so this pass fills the gaps
around three constructors that earlier waves had already translated.

- **`0x430`-byte actor handlers, `0x80728550..0x807286E0`** - four bodies that sit
  directly after the constructor [`fn_80728260`] and its `obj_door` loader
  [`fn_807282d0`]. Two of them are camera-player guards: they read the followed
  player index `0x80429CA4`, fetch that player's pad record from
  `0x80429F50[index + 1]`, and dispatch descriptor `0x808306D8` (or
  `0x80830718` after the halfword angle `+0x102` walks to its `-0x8000` clamp).
  The other two are the matching sound cues `0x28D` / `0x28E`, queued through
  `0x80198C30` with the packed `+0xAC` position.

- **`remo_lift_soroban` actor, `0x80728A70..0x80729820`** - fifteen bodies of the
  `0x5F8`-byte lift whose loader is the already-translated [`fn_80728cc0`]:
  the registered destructor wrapper [`fn_80728a70`], the constructor
  [`fn_80728bf0`] (eight sub-objects, vtable `0x8080B5C8`), the matrix writer
  [`fn_80728e70`], the camera-range test [`fn_80728ed0`], the player-index cache
  and change test ([`fn_80728f20`], [`fn_80728f50`]), the recursive player-mask
  collector [`fn_80728fe0`] with its `+0x4A4` list walker [`fn_807290d0`], the
  `change_mark_A` bind [`fn_80729490`], the `+0x5F1` countdown
  [`fn_807295c0`], the vertical-pad state selector [`fn_80729650`], the
  accumulator seed [`fn_80729770`] and the per-frame height update
  [`fn_80729820`] with its `0x2AC` / `0x2AD` cues. The state descriptors used by
  this group are `0x80830798`, `0x808307D8` and `0x80830818`.

- **`obj_door` actor, `0x80729E30..0x8072A3E0`** - five bodies of the `0x438`-byte
  actor built by [`fn_80729fb0`]: the release wrapper [`fn_80729e30`], the
  liveness handler [`fn_8072a180`] (`0x80064F50` then virtual `+0x10`), the
  camera-player guard [`fn_8072a270`] (descriptor `0x80830898`), and the two
  animation-end handlers [`fn_8072a310`] / [`fn_8072a3e0`] that dispatch
  `0x808308D8` and `0x80830858` when `0x8015F140` returns the end frame
  `0x807CD3C4`.

- **`0x5A8`-byte actor tail, `0x8072A760..0x8072AE50`** - three bodies around the
  constructor [`fn_8072a8e0`]: its release wrapper [`fn_8072a760`], the
  rider-slot flag flip [`fn_8072ad60`] over the four 8-byte records at `+0x584`
  (`+0x584` candidate word, `+0x588` owner word, rider byte `+0x38F`), and the
  unrelated small `0x3D0`-byte constructor [`fn_8072ae50`] (vtable
  `0x8080BB28`) that sits at the end of the run.

- **Recurring layout.** The `0x5F8` actor keeps its model holder at `+0x3F0`, the
  `change_mark_A` clip at `+0x430`, a follow-animation block at `+0x468`
  (vtable `0x80329548`), a stream helper at `+0x494`, the position driver at
  `+0x4F4`, and the height group `+0x5E0` (step), `+0x5E4` (offset), `+0x5E8`
  (height), `+0x5EC`/`+0x5ED` (bind flags), `+0x5EE`/`+0x5EF` (player index and
  previous index), `+0x5F0` (per-player button mask), `+0x5F1` (countdown) and
  `+0x5F2` (riding mask). Throughout the bucket the state object is at `+0x394`
  and the position/angle block at `+0xAC..+0xB4` / `+0x100..+0x104`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80728550` | `fn_80728550` | 124 | medium |
| `0x807285D0` | `fn_807285d0` | 80 | high |
| `0x80728630` | `fn_80728630` | 160 | medium |
| `0x807286E0` | `fn_807286e0` | 80 | high |
| `0x80728A70` | `fn_80728a70` | 88 | high |
| `0x80728BF0` | `fn_80728bf0` | 200 | high |
| `0x80728E70` | `fn_80728e70` | 96 | high |
| `0x80728ED0` | `fn_80728ed0` | 80 | medium |
| `0x80728F20` | `fn_80728f20` | 48 | medium |
| `0x80728F50` | `fn_80728f50` | 136 | high |
| `0x80728FE0` | `fn_80728fe0` | 236 | medium |
| `0x807290D0` | `fn_807290d0` | 76 | medium |
| `0x807293A0` | `fn_807293a0` | 40 | high |
| `0x807293F0` | `fn_807293f0` | 156 | high |
| `0x80729490` | `fn_80729490` | 288 | medium |
| `0x807295C0` | `fn_807295c0` | 104 | high |
| `0x80729650` | `fn_80729650` | 276 | medium |
| `0x80729770` | `fn_80729770` | 148 | high |
| `0x80729820` | `fn_80729820` | 428 | medium |
| `0x80729E30` | `fn_80729e30` | 88 | high |
| `0x8072A180` | `fn_8072a180` | 88 | medium |
| `0x8072A270` | `fn_8072a270` | 124 | medium |
| `0x8072A310` | `fn_8072a310` | 176 | medium |
| `0x8072A3E0` | `fn_8072a3e0` | 120 | high |
| `0x8072A760` | `fn_8072a760` | 88 | high |
| `0x8072AD60` | `fn_8072ad60` | 100 | medium |
| `0x8072AE50` | `fn_8072ae50` | 76 | high |

## Address range

`0x80728550..0x8072AE50`, in four runs:

- `0x80728550..0x807286E0` - four bodies, ending before the `0x802DCBFC` stub
  pair `0x80728B90`/`0x80728BC0` that precedes the `remo_lift_soroban`
  constructor;
- `0x80728A70..0x80729820` - fifteen bodies, interrupted only by the rejected
  loader `0x80728CC0`, the two `0x802DCBFC` stubs `0x80728B90`/`0x80728BC0` and
  the gap `0x80729...` that `functions.tsv` does not describe;
- `0x80729E30..0x8072A3E0` - five bodies, interrupted by the two `0x802DCBFC`
  stubs `0x80729F50`/`0x80729F80` and the already-translated constructor
  [`fn_80729fb0`] plus its loader [`fn_8072a020`];
- `0x8072A760..0x8072AE50` - three bodies either side of the already-translated
  constructor [`fn_8072a8e0`]; `0x8072AC00` was already in the ledger from
  wave 54 and was left untouched.

This pass adds 27 bodies, leaving 126 of the 153 remaining `0x8072` functions
for later waves.

## Deliberate skips

- `0x80728CC0` (424) - the `remo_lift_soroban` model loader. Its export begins
  with the paired-single block `unaff_GQR0`, `ldexpf` and the
  `CONCAT13`/`CONCAT22` lane split, so the scale applied to the two lanes would
  be a guess. Same rejection recorded for `0x80730360`, `0x80651640`,
  `0x80682F40` and `0x806843B0` in earlier waves. The constructor
  [`fn_80728bf0`] and the resource string `g3d/remo_lift_soroban.brres` are still
  documented from the surrounding bodies.
- `0x80728B90`, `0x80728BC0`, `0x80729F50`, `0x80729F80` (48 each) - the
  `0x802DCBFC` indirect-jump forwarders. Only the hidden `r12` this-adjust is
  observable, so they carry no Rust-visible behaviour; waves 3 and 71 left the
  same stubs untranslated.
- `0x8072C1F0` (4) - a single-instruction body (a bare `blr`). `functions.tsv`
  lists it, but there is nothing to reconstruct beyond a no-op, and no caller in
  this bucket reaches it.
- The rest of the bucket is untouched, not rejected: the `0x80720050..0x80722110`
  ship/aim tail, `0x80722700..0x80725790` (the `daPowBlock_c`/`block_*` state
  block), `0x80726260..0x80728260`, `0x8072A880..0x8072AC00`,
  `0x8072B670..0x8072C980` (the `lift_zen_chika_pori` block) and
  `0x8072D3B0..0x8072FFFF` belong to later waves.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (27 rows, 3704 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the 68 pre-existing rows of
  this bucket were checked before writing, and none of them is in this pass
  (`0x8072AC00` was dropped from the candidate list for exactly this reason).
- `node tools/check-lane-csv.mjs wave72` reports `OK` with
  `wave72-bases.csv: 27 rows`.
- `cargo check --message-format=short` reports no error for `src/game/bases.rs`.
