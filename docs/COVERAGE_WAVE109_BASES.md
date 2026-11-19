# Coverage - Wave 109 (`game::bases`, `0x806E` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address on this page is a **static analysis address** under the
canonical synthetic base **`0x80600000`** (produced by `tools/rel2elf.mjs`),
never a runtime address: the Wii REL loader chooses the real load address, so
the numbers only identify positions inside the REL image.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c` (globbed by address
prefix, never assumed). "Confidence" is `high` when guards, stores, constants
and call arguments are literal, and `medium` when a callee identity, a field
role or a dropped argument had to be inferred; those bodies carry
`// observed:` / `// inferred:` / `// unknown:` notes. No body in this pass is
`low`.

The bucket holds 229 functions; 112 were already in `ledger/functions.csv` when
this pass began, leaving 117. This pass takes 30 of them, in four families.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 5,036 | 19 | 11 |

## What this pass covers

- **The kind-`0x1F7` actor tail (`0x806E08A0..0x806E0DC0`, 3 bodies).** The
  `0x708`-byte constructor [`fn_806e08a0`] (base vtable `0x807FC428`, derived
  vtable `0x807FC688`, four `0x60`-byte children at `+0x474` built by
  `0x802DC9A0`), its loader-guard teardown [`fn_806e0d60`], and the `+0x6FC`
  key search [`fn_806e0dc0`] that walks actor kind `0x1F7` through `0x80162D50`
  and compares `+0x6FC` with the candidate's `+0x6F8`.
- **The four-slot rail actor and a `lift_zen_kaiten` sibling
  (`0x806E3C50..0x806E4A00`, 7 bodies).** [`fn_806e3c50`] places the four
  `0x60`-byte rail slots from the variant word at `+0x04` (bits `12..15`,
  `16..19`, `20..23`, `8..9`, side bit `24`), registers the three-phase range at
  `+0x4A0` through `0x8007FA40` and installs `0x806E4D60`/`0x806E4D80`. Next to
  it, the `lift_zen_kaiten` sibling pair [`fn_806e4670`]/[`fn_806e4a00`] writes
  the `0x807CBC6x`/`0x807CBC8x` placement words and builds two and one
  `0xE4`-byte animated parts; [`fn_806e48d0`] is its
  `g3d/lift_zen_kaiten_kanaami.brres` loader, and [`fn_806e46e0`],
  [`fn_806e3ce0`], [`fn_806e3d80`] are the three teardowns (array destructor,
  ride release, rail-block release).
- **The `0x598`-byte lift (`0x806E5AC0..0x806E66A0`, 8 bodies).** Constructor
  [`fn_806e5ac0`]; matrix/animation refresh [`fn_806e5f30`]; the two travel
  parameter writers [`fn_806e5fc0`] (raw pair) and [`fn_806e5fe0`] (scaled corner
  quad plus index step); the three-phase range registration [`fn_806e60d0`]; the
  callback triple [`fn_806e6640`]; and the two capture handlers
  [`fn_806e6530`]/[`fn_806e66a0`], which resolve the handle at `+0x468` and
  dispatch `daLiftZenShiro_c`'s third state (`0x8082E2D0`, `Vanish`) and second
  state (`0x8082E290`, `Move`).
- **The character carrier (`0x806E8780..0x806E90E0`, 11 bodies, plus the
  `0x806EEE20` position copy).** State query
  [`fn_806e8780`], rider adoption [`fn_806e87d0`], yaw step [`fn_806e8840`],
  contact-range creation [`fn_806e89a0`], the multi-contact lift test
  [`fn_806e8a30`], the fall update [`fn_806e8c40`], ride exit
  [`fn_806e8de0`]/[`fn_806e8e30`], the ride transfer [`fn_806e8e80`], the landing
  preparation [`fn_806e8fd0`], the grounded update [`fn_806e90e0`] and the
  `0x802DCD70` bracket that copies a source position into the owner and every
  live rider [`fn_806eee20`].

Start with [`fn_806e5ac0`] for the lift constructor and [`fn_806e8c40`] for the
carrier frame loop; [`fn_806e48d0`] is the clearest model-loader example.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806E08A0` | `fn_806e08a0` | 172 | high |
| `0x806E0D60` | `fn_806e0d60` | 84 | high |
| `0x806E0DC0` | `fn_806e0dc0` | 92 | high |
| `0x806E3C50` | `fn_806e3c50` | 132 | medium |
| `0x806E3CE0` | `fn_806e3ce0` | 160 | medium |
| `0x806E3D80` | `fn_806e3d80` | 84 | high |
| `0x806E4670` | `fn_806e4670` | 112 | medium |
| `0x806E46E0` | `fn_806e46e0` | 148 | high |
| `0x806E48D0` | `fn_806e48d0` | 172 | high |
| `0x806E4A00` | `fn_806e4a00` | 104 | medium |
| `0x806E5AC0` | `fn_806e5ac0` | 140 | high |
| `0x806E5F30` | `fn_806e5f30` | 136 | high |
| `0x806E5FC0` | `fn_806e5fc0` | 32 | high |
| `0x806E5FE0` | `fn_806e5fe0` | 240 | high |
| `0x806E60D0` | `fn_806e60d0` | 156 | high |
| `0x806E6530` | `fn_806e6530` | 268 | medium |
| `0x806E6640` | `fn_806e6640` | 76 | high |
| `0x806E66A0` | `fn_806e66a0` | 216 | medium |
| `0x806E8780` | `fn_806e8780` | 80 | high |
| `0x806E87D0` | `fn_806e87d0` | 112 | high |
| `0x806E8840` | `fn_806e8840` | 100 | high |
| `0x806E89A0` | `fn_806e89a0` | 144 | high |
| `0x806E8A30` | `fn_806e8a30` | 436 | medium |
| `0x806E8C40` | `fn_806e8c40` | 416 | medium |
| `0x806E8DE0` | `fn_806e8de0` | 72 | high |
| `0x806E8E30` | `fn_806e8e30` | 72 | high |
| `0x806E8E80` | `fn_806e8e80` | 324 | medium |
| `0x806E8FD0` | `fn_806e8fd0` | 196 | high |
| `0x806E90E0` | `fn_806e90e0` | 360 | medium |
| `0x806EEE20` | `fn_806eee20` | 200 | medium |

**Totals:** 30 translated, 0 partial, 0 stub (19 high, 11 medium, 5,036 bytes).

## Deliberate skips

Read and rejected, with the reason:

- **GQR / paired-single bodies.** `0x806E26B0` (228 bytes), `0x806E2950` (440),
  `0x806E3090` (728), `0x806E3EB0` (188), `0x806E3FC0` (316), `0x806EC2B0`
  (200), `0x806EC380` (148), `0x806EC5F0` (220), `0x806EC6D0` (172),
  `0x806EC780` (172), `0x806ED340` (288), `0x806EDE70` (308) and `0x806EECA0`
  (384) open with `unaff_GQR0`/`unaff_GQR3` brackets and `ldexpf` calls. The
  incoming GQR state cannot be reconstructed from the export, so their
  argument/loop scaling is lost. Same reason as `0x80678300`/`0x806787C0` in
  wave 108 and `0x80790280`/`0x80799D70` in wave 107.
- **`0x802DCBFC` indirect-jump stubs.** `0x806E0840`, `0x806E0870`,
  `0x806E11E0`, `0x806E1210`, `0x806E1720`, `0x806E1750`, `0x806E3910`,
  `0x806E3940`, `0x806E4450`, `0x806E4480`, `0x806EAAC0`, `0x806EAAF0`,
  `0x806EBD60`, `0x806EBD90`, `0x806EE5D0`, `0x806EE600`, `0x806EF640`,
  `0x806EF670`, `0x806EFE90`, `0x806EFEC0`, plus `0x806E3BE0` (44 bytes) and
  the `0x806E3370` thunk, are `0x802DCBFC` tail-call trampolines with no
  recoverable target arguments; they stay untranslated like the wave-108 pair
  `0x806776D0`/`0x80677700`.
- **`0x806E8990`** (16 bytes) is a `halt_baddata()` export: Ghidra could not
  decode the body. It is still *called* by [`fn_806e8c40`] and [`fn_806e90e0`]
  by address, with an `// unknown:` note, but no body is claimed.
- **`0x806EE...` per-frame drivers.** `0x806ED160` (252 bytes), `0x806ED260`
  (176), `0x806ED340` (288) and `0x806EDE70` (308) are the four driver/stepping
  bodies of one actor. They read their owner through the `0x802DCD6C`/
  `0x802DCD74` thunks, call `0x806ED160` — which Ghidra also prints without
  arguments — with registers the export does not carry, and
  `0x806ED340`/`0x806EDE70` additionally bracket paired-single (`ldexpf`) math.
  Without register liveness across those calls the argument lists are
  unrecoverable, so they stay untranslated.
- **Dropped-`this` wrappers.** `0x806E6450` (128 bytes) calls
  `FUN_806e5d60()` and `0x806E64D0` (72 bytes) calls `FUN_806e60d0()` with the
  receiver dropped, and `0x806E0EA0` (92 bytes) calls `FUN_806e0dc0()` the same
  way. The wave-108 precedent rejected `0x80678400` for exactly this shape, so
  they are left for a pass that can recover the register from the disassembly.

The remaining untranslated functions of the bucket were not opened in this pass
(budget, not doubt); the pass deliberately stops at `0x806E90E0` and does not
enter the `0x806E9xxx` tail already worked by waves 33 and 58.

## Notes on inference

- `0x8082E290` and `0x8082E2D0` are the second and third `daLiftZenShiro_c`
  state descriptors registered by [`fn_806e6810`] (`Move` and `Vanish`), which
  is literal evidence for the two ids used by [`fn_806e6530`] and
  [`fn_806e66a0`]. The actor class that owns them here is still `// unknown:`.
- `0x8082E408`, `0x8082E448` and `0x8082E488` are compared or dispatched by the
  carrier family; no exported registration table in this image names them, so
  their state names stay `// unknown:`.
- The carrier family keeps its class vtable at `+0x60` (not at `+0x00`), so
  [`fn_806e87d0`], [`fn_806e8e80`] and [`fn_806e90e0`] dispatch through
  `actor_method1(obj, 0xD4, id)`; the C spells this
  `(**(code **)(*(int *)(obj + 0x60) + 0xd4))(obj, id)`.
- The `0x807CBCB0`/`0x807CBD40` subtrahends turn the
  `(double)CONCAT44(0x43300000, n)` construction into a plain unsigned index.
  Both bodies keep the bit construction explicit instead of replacing it with a
  cast, so the provenance stays visible.
- `0x807FD0C0`/`0x807FD09C` are the literal `lift_zen_kaiten_kanaami` resource
  and `g3d/lift_zen_kaiten_kanaami.brres` archive strings used by
  [`fn_806e48d0`]; the sibling-loader strings `lift_zen_hammer`,
  `lift_zen_kaihei` and `lift_zen_kaiten` confirm the naming family.
- Float constants are read from their REL addresses with
  `platform::read_f32`, never inlined as guessed decimals.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 5,036 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 112 translated
  addresses were re-read first and none is duplicated here. The file now holds
  2,713 bodies.
- `node tools/check-lane-csv.mjs wave109` reports 30 rows for
  `wave109-bases.csv` and `OK`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/bases.rs`; diagnostics raised by other lanes' files are ignored.
