# Coverage - Wave 123 (`game::bases`)

## How to read this file

This page documents the Wave 123 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`** produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. In this
image the address-to-file mapping is the one recorded in
`docs/COVERAGE_WAVE97_BASES.md`, which is how the float and string constants
cited below were checked. Cross-module pointers (`0x8035xxxx`, `0x8042xxxx`)
keep their absolute values.

Every cited file is a real Ghidra C export under
`work/ghidra/rel-d_basesNP/export/`, and every byte count was re-read from
`work/ghidra/rel-d_basesNP/export/functions.tsv`.

The `0x8068` bucket holds 176 functions. Before this pass 77 were translated,
leaving 99; this pass takes 30 of them. The block the 0x80688610..0x8068FDA0
window was explicitly deferred, not rejected, by
`docs/COVERAGE_WAVE70_BASES.md` ("they are untried, not rejected; they belong
to later waves") and by `docs/COVERAGE_WAVE97_BASES.md`, so this pass works that
deferred block and two isolated neighbours.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee identity, a field role, a lost argument or a
helper's semantics had to be inferred. Comments in the bodies keep
`// observed:`/`// inferred:`/`// unknown:` markers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 4,660 | 14 | 16 |

All 30 rows are `translated`; none is `partial`.

## What this pass covers

- **Rider/panel actor, `0x80688610..0x80689110` (11 bodies).** The family keeps
  position `+0xAC..+0xB4`, Euler halfwords `+0x100..+0x104`, a matrix at `+0x7C`
  and a model context at `+0x544`. [`fn_80688610`] rebuilds that transform;
  [`fn_806888c0`] is the band stepper (`+0xEC` toward `+0xF8` by `+0x560`);
  [`fn_80688a40`] is the per-frame rise with the `0.005`/`0.1` speed ladder and
  the three height bands `48`/`64`/`80` above `+0x578`; [`fn_80688d50`] is the
  settle transition that leaves at `+0xB0 <= +0x578`; [`fn_80688f80`] is the
  state reset that pins `+0x560` to `0.1875` and `+0xB4` to `1948.0`;
  [`fn_80689000`] is the ground probe against `0x80429E00` with the `+0x56C`
  latch; [`fn_80688920`] copies the attached target's `+0xAC`; [`fn_80688990`]
  detaches and resets the `+0x394` interface; [`fn_80688860`] is a guarded
  three-word forwarder; [`fn_80688820`] is the plain release wrapper; and
  [`fn_80689110`] submits the position to the `0x80429EE8` effect system.
  [`fn_806898c0`] closes the family with the `0x8015F800` teardown.
- **`+0x3F0`-context actor, `0x8068B860..0x8068BC40` (6 bodies).**
  [`fn_8068b860`] writes the class matrix from the attached target when the
  interface probe `0x8082AC30` answers, and otherwise rebuilds it from the
  position exactly as [`fn_80688610`] does; [`fn_8068b9f0`] produces the
  twelve-word copy it consumes; [`fn_8068bb00`] publishes the `+0x3F0` context
  into the `+0x478` attachment; [`fn_8068bb70`] is the lift probe (bit `0x8000`
  plus kind `3`); [`fn_8068bbf0`] reports the position at the fixed height
  `5628.0`; and [`fn_8068bc40`] is the contact body that spawns `Wm_en_hit` and
  ends the hit state.
- **`0xE4`-record owner, `0x8068E6A0..0x8068EB80` (4 bodies).**
  [`fn_8068e6a0`] marks the four-slot byte map at `+0xC60`;
  [`fn_8068e8e0`] finds a live type-`0x1A3`/`0x1A4` actor matching `+0xC3C`;
  [`fn_8068e970`] latches that target's word and angle; and [`fn_8068eb80`]
  snaps the owner's yaw plus the per-slot offsets read from `0x807C9BF6`.
- **Tail, `0x8068F810..0x8068FDA0` (6 bodies).** [`fn_8068f810`] builds the
  `+0x404` matrix with the extra `+6`/`-6` axis rotations;
  [`fn_8068f960`] is the ground ceiling test and [`fn_8068fda0`] its fall
  transition; [`fn_8068fa60`] counts the `+0x41C` timer and drifts the
  `+0xDC..+0xE4` offsets; [`fn_8068fb80`] resets the motion and flag block; and
  [`fn_8068fc40`] spins the actor and spawns effect `0x1E9`.
- **Isolated pair (2 bodies).** [`fn_806809c0`] is the divide-by-three helper
  behind [`fn_80680ab0`]'s three-way slot, and [`fn_80681600`] is the
  `0x4F8`-byte `circle_ground` board constructor (vtable `0x807ECFC8`) that
  completes the family already covered by [`fn_80681b90`].

Shared fields: class vtable `+0x60`, interface block `+0x394`, matrix `+0x7C`,
local transform `+0xDC`, position `+0xAC..+0xB4`, angles `+0x100..+0x104`,
ground table `0x80429E00`, and the `0xE4` stride of the four helper records at
`+0x3D0`.

Start with [`fn_80688610`] (the matrix write), [`fn_80688a40`] (the richest
per-frame body) and [`fn_8068b860`] (the two-path matrix writer).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806809C0` | `fn_806809c0` | 240 | medium |
| `0x80681600` | `fn_80681600` | 108 | high |
| `0x80688610` | `fn_80688610` | 220 | high |
| `0x80688820` | `fn_80688820` | 64 | high |
| `0x80688860` | `fn_80688860` | 96 | medium |
| `0x806888C0` | `fn_806888c0` | 84 | high |
| `0x80688920` | `fn_80688920` | 76 | high |
| `0x80688990` | `fn_80688990` | 96 | medium |
| `0x80688A40` | `fn_80688a40` | 260 | high |
| `0x80688D50` | `fn_80688d50` | 132 | medium |
| `0x80688F80` | `fn_80688f80` | 108 | medium |
| `0x80689000` | `fn_80689000` | 260 | high |
| `0x80689110` | `fn_80689110` | 100 | medium |
| `0x806898C0` | `fn_806898c0` | 88 | high |
| `0x8068B860` | `fn_8068b860` | 400 | medium |
| `0x8068B9F0` | `fn_8068b9f0` | 264 | medium |
| `0x8068BB00` | `fn_8068bb00` | 100 | medium |
| `0x8068BB70` | `fn_8068bb70` | 128 | medium |
| `0x8068BBF0` | `fn_8068bbf0` | 76 | medium |
| `0x8068BC40` | `fn_8068bc40` | 296 | medium |
| `0x8068E6A0` | `fn_8068e6a0` | 100 | high |
| `0x8068E8E0` | `fn_8068e8e0` | 140 | medium |
| `0x8068E970` | `fn_8068e970` | 20 | high |
| `0x8068EB80` | `fn_8068eb80` | 164 | medium |
| `0x8068F810` | `fn_8068f810` | 328 | high |
| `0x8068F960` | `fn_8068f960` | 52 | high |
| `0x8068FA60` | `fn_8068fa60` | 288 | medium |
| `0x8068FB80` | `fn_8068fb80` | 172 | high |
| `0x8068FC40` | `fn_8068fc40` | 140 | medium |
| `0x8068FDA0` | `fn_8068fda0` | 60 | high |

**Totals:** 30 bodies, 4,660 bytes (14 high, 16 medium).

## Deliberate skips

Read and rejected, with the reason:

- **Paired-single / GQR bodies.** `0x80680720` (660), `0x80680E80` (292),
  `0x80681C50` (384), `0x80688BA0` (356), `0x80688E10` (356), `0x8068E0B0`
  (912), `0x8068E440` (228) and `0x8068EA10` (332) convert halfwords through
  Ghidra's `unaff_GQR0`/`unaff_GQR3` ladder (`ldexpf` plus `CONCATxx`). The
  stores and call sites are visible, but which GQR slot feeds each conversion is
  not decidable from the export, so the numeric result would be a guess. This is
  the same family the wave-122 pass recorded at `0x806D6530` and neighbours.
- **`0x802DCBFC` forwarders.** `0x806815A0`, `0x806815D0`, `0x806820C0`,
  `0x806820F0`, `0x806899E0` and `0x80689A10` forward two unnamed arguments to
  the unresolved indirect-jump stub; earlier waves already recorded that family
  as `partial`, and repeating it adds no evidence.
- **Duplicate release twins.** `0x80681480` and `0x80681FA0` are byte-identical
  copies of the `0x8015F800`-then-`0x802B9280` pattern already written as
  [`fn_806898c0`]; only the first was translated, to stay inside the body
  budget.
- **Long bodies outside the windows.** `0x806822B0` (1,108), `0x80684680`
  (936) and `0x80686550` (1,884) were read only far enough to exclude them from
  the clusters above; they need a separate pass.
- **The rest of the bucket.** The other 69 untranslated `0x8068` addresses were
  not attempted here.

## Notes on inference

- [`fn_806809c0`] is `medium`: the export renders the compiler's divide-by-3
  sequence as two subtract-3 loops. The first subtracts `0x18` (a multiple of
  three) while draining a copy of the quotient, so it leaves the residue
  unchanged; the second subtracts three `value / 3` times. The folded
  `value % 3` is therefore exact, but the first loop is not literally dead code
  in the C, so the body is not marked `high`.
- [`fn_80688860`], [`fn_80689110`], [`fn_8068bb70`], [`fn_8068bbf0`] and
  [`fn_8068bc40`] are `medium` because their callees are unprototyped: the
  argument words are literal, but what `2`, `3`, bit `0x8000`, kind `3` or the
  scene constants mean stays `// unknown:`.
- [`fn_80688d50`], [`fn_80688f80`], [`fn_8068eb80`] and [`fn_8068fa60`] are
  `medium` because the export prints one dispatch without its receiver
  (`FUN_806888c0()`, virtual `+0x98`, `FUN_8068e8e0()`, virtual `+0x28`). The
  receiver is `r3`, which is the same object the neighbouring calls pass, and
  each body says so with an `// inferred:` note.
- [`fn_8068b9f0`] is `medium` for the same reason at `0x8016F150`; its twelve
  copied words, the `+0x468` handle and the copy order are literal.
- [`fn_8068bb00`] is `medium`: the 60-byte stack block and the index `0` are
  literal, but the block layout filled by `0x80168180` is not recoverable.
- [`fn_80681600`] is `high` except for the class name: the allocation size,
  vtable `0x807ECFC8` and every sub-object offset are explicit, and the
  `circle_ground` identity is inferred from the adjacent model loader
  [`fn_80681b90`].

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 4,660 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 77 translated
  addresses were re-read first and none is duplicated here.
- `node tools/check-lane-csv.mjs wave123` reports `OK` for
  `wave123-bases.csv` (30 rows, 30 unique addresses).
- `cargo check --message-format=short` reports no error and no warning naming
  `src/game/bases.rs`; the two `E0425` errors it prints are raised by
  `src/game/enemies.rs`, which belongs to a parallel lane.
