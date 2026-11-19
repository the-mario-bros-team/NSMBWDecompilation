# Coverage - Wave 136 (`game::enemies`)

## How to read this file

This page records the Wave 136 `game::enemies` lane: 26 functions of REL module 3,
`d_enemiesNP`, appended to `src/game/enemies.rs` as section 118. Every row of the
machine-readable mirror (`ledger/agents/wave136-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none of
these is a runtime address; every function doc repeats that note. Floating-point
constants are cited by address and read at run time with `platform::read_u32` /
`read_f32`, never transcribed as invented values. Raw words were decoded from
`work/rels/elf/d_enemiesNP.elf` at the mapping of `docs/REL_ELF_MAPPING.md`
(`file_offset = address - 0x80A00000 + 0x1000`).

Two rows are `partial`-class and four are trampolines whose hidden `r12` operand
is outside the Rust ABI; those six are listed as `partial` in the CSV and
explained under "Why six rows are `partial`".

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 26 | 2,888 | 18 | 8 |

The lane has two halves:

- **12 bodies** are the twelve rows of `docs/SPEC_WAVE135_ENEMIES.md` (ranks
  87-98 of the wave-127 class-B list): ten **ready** and two **conditional**
  (`0x80AEEA30`, `0x80A441E0`). They were written from the specification's
  instruction decode and then re-checked against the export and the raw words.
- **14 bodies** are still-open entries of the `0x80AF0000` planner bucket (29
  untranslated before this pass, 14 after). They are one coherent cluster: the
  `daEnSanboEl_c` state callbacks that `register_sanbo_el_states` (`0x80AFE3C0`)
  names, four more `0x802DCBFC` deleting-destructor trampolines, the `0x528`-byte
  actor allocator, two effect emitters, the `+0x8C0` matrix-pair writer and its
  consumer, the `+0x7C` rebuild/commit step and the `+0x778` slot-angle clamp.

## What this pass covers

- **The specification bodies (`0x80A1E100..0x80B0DD80`).** The `:`-suffix name
  comparison over `strrchr`/`strcmp`; the `+0x6C0` `u16` countdown that retires
  through `0x80162510` or steps through `0x80A2D460`; the `+0xDC` ramp by `0.05`
  to a `1.0` ceiling with the `+0xD4` callback; the `+0x548` sub-object reset and
  the six-call per-frame chain; the `+0x874`/`+0x88C`-selected `+0x72C` step with
  the six-argument `0x8007FB10` tail call; the two-nibble `+0xDC`/`+0xE0`
  half-steps; the `Wm_en_spindamage` emit at `+0x9A0..+0x9A8`; the `CR0.LT` side
  flip; the `+0x3F0`/`+0x3F4` two-slot bridge; the `+0x9F8`-link position copy;
  the four-float `+0x728..+0x734` box; and the `+0x698` record seed.
- **The `daEnSanboEl_c` callbacks (`0x80AFEC20`, `0x80AFEC80`, `0x80AFF4F0`).**
  `0x8015F800(state, 0)` plus `0x802B9280(state)` for a positive mode, exactly the
  skeleton the surrounding module uses for every `fState` callback.
- **Four `0x802DCBFC` trampolines (`0x80AFEF70`, `0x80AFEFA0`, `0x80AFF610`,
  `0x80AFF640`).** Two `+0x18` and two `+0x24` this-adjustors, the same shape as
  the thirteen wave-135 rows.
- **The `0x528`-byte allocator `0x80AFF670`.** `0x801628C0(0x528)`, the base
  constructor, then class vtable `0x80B659A0` at `+0x60`.
- **The effect pair `0x80AFFA50`/`0x80AFFAA0`.** A `Vec3` copy through
  `0x8006A4B0` and one `0x80198C30([0x80429D60], 0x26C|0x26B, &copy, 0)`.
- **The `+0x8C0` matrix pair (`0x80AFFBF0`, `0x80AFFFB0`).** The writer seeds two
  points, three BSS words and two flags; the consumer builds two 3x4 matrices,
  merges them with `0x801C0540` and commits through the `+4` holder's slot
  `+0x14`.
- **The `+0x7C` rebuild/commit step `0x80AFF1C0`.** Byte-for-byte the same
  argument order as [`fn_80a00940`], including which angle each of the three
  `0x8016ECB0`/`0x8016EDD0`/`0x8016EEF0` setters receives.
- **The slot-angle clamp `0x80AF38B0`.** `0x800B3980(20.0, 1.0, +0x778)` scaled
  by `1.4` and clamped to `±13500` on the `s16` view, with the in-range value
  returned untruncated as the shipped `lwz` does.

### Confirmations worth recording

- **`0x80AEEA30`'s `+0xE4` store really is uninitialised.** The frame writes
  `0x08`, `0x0C`, `0x18`, `0x1C`, `0x20` and `0x24` only; the `lfs f0,0x10(r1)`
  at `0x80AEEA44` is read back by the `stfs f0,0xE4(r3)` at `0x80AEEA80` before
  anything writes it.
- **`0x80AEEA30`'s multipliers are `0.5 * (1.0 + nibble)`.** `fmuls` takes its
  multiplicand from bits 6-10, so `0x80AEEA88` is `fmuls f1,f2,f3` and
  `0x80AEEA98` is `fmuls f0,f2,f0`; reading both as `*f0` would make the low
  nibble dead, which the preceding `fadds f3,f4,f3` contradicts.
- **`0x80A441E0` writes `+0xB0` twice.** The raw value is stored at
  `0x80A44224` and overwritten by `y - [0x8042B354]` at `0x80A44234`; both stores
  are kept in the port.
- **The `0x80AF38B0` bias pair is `2^52 + 2^31`.** `0x80B313E0` holds
  `0x4330000080000000`, and `0x80AF38EC` is `xoris r3,r3,0x8000`, so the pair
  converts the signed halfword exactly rather than collapsing it.
- **`0x80AFEC80` has one dead branch.** `0x80AFECA4` is a second `beq` on the
  `CR0` set by the `cmpwi` at `0x80AFEC88`; the first `beq` at `0x80AFECA0`
  already covers it, so the second target is unreachable. The port models the
  live path and says so.
- **`0x80AFF670`'s vtable is `0x80B659A0`.** Built as `lis r3,0x80B6` +
  `addi r3,r3,0x59A0`, and inside the image.

## Functions

| address | function | bytes | status | confidence |
|---|---|---:|---|---|
| `0x80A1E100` | `fn_80a1e100` | 112 | translated | high |
| `0x80A2D660` | `fn_80a2d660` | 132 | translated | high |
| `0x80A398C0` | `fn_80a398c0` | 132 | translated | high |
| `0x80A441E0` | `fn_80a441e0` | 116 | translated | medium |
| `0x80A778F0` | `fn_80a778f0` | 116 | translated | high |
| `0x80A77BB0` | `fn_80a77bb0` | 124 | translated | high |
| `0x80A7EE20` | `fn_80a7ee20` | 128 | translated | high |
| `0x80AEEA30` | `fn_80aeea30` | 124 | partial | medium |
| `0x80AFEEB0` | `fn_80afeeb0` | 136 | translated | high |
| `0x80AF38B0` | `fn_80af38b0` | 160 | translated | medium |
| `0x80AFEC20` | `fn_80afec20` | 88 | translated | high |
| `0x80AFEC80` | `fn_80afec80` | 92 | translated | high |
| `0x80AFEF70` | `fn_80afef70` | 48 | partial | medium |
| `0x80AFEFA0` | `fn_80afefa0` | 48 | partial | medium |
| `0x80AFF1C0` | `fn_80aff1c0` | 244 | translated | high |
| `0x80AFF4F0` | `fn_80aff4f0` | 88 | translated | high |
| `0x80AFF610` | `fn_80aff610` | 48 | partial | medium |
| `0x80AFF640` | `fn_80aff640` | 48 | partial | medium |
| `0x80AFF670` | `fn_80aff670` | 76 | translated | high |
| `0x80AFFA50` | `fn_80affa50` | 76 | translated | high |
| `0x80AFFAA0` | `fn_80affaa0` | 76 | translated | high |
| `0x80AFFBF0` | `fn_80affbf0` | 152 | translated | medium |
| `0x80AFFFB0` | `fn_80afffb0` | 156 | translated | high |
| `0x80B0C4C0` | `fn_80b0c4c0` | 120 | translated | high |
| `0x80B0D1B0` | `fn_80b0d1b0` | 124 | translated | high |
| `0x80B0DD80` | `fn_80b0dd80` | 124 | translated | high |

### Why six rows are `partial`

- **The four trampolines** (`0x80AFEF70`, `0x80AFEFA0`, `0x80AFF610`,
  `0x80AFF640`) are `partial`: the visible call
  `0x802DCBFC(param_2, param_2, param_1)` and the `+0x18`/`+0x24` adjustor are
  exact, but the hidden `r12` this-adjust pointer is outside the Rust ABI and
  cannot be modelled by `platform::call3`. Same limitation as the wave-135
  rows.
- **`0x80AEEA30`** is `partial` because its `+0xE4` store copies the caller's
  `0x10(r1)` slot; pinning the caller `0x80AED540` is the missing step. The two
  ramps that *are* defined are reproduced exactly.

### Why three `translated` rows are `medium`

- **`0x80A441E0`** subtracts the `main.dol` `f32` at `0x8042B354`, which is not
  published in this checkout; the body reads it at run time instead of
  transcribing a value.
- **`0x80AF38B0`** calls `0x800B3980`, whose scale contract is inferred from its
  callers in `game::bases`, not from a decoded body.
- **`0x80AFFBF0`** copies three words from BSS (`0x80B781C8..0x80B781D0`), so
  those are runtime reads rather than image constants.

## Deliberate skips

- **`0x80AF0370` (44 bytes) and `0x80AF3840` (16 bytes).** Both exports are pure
  `halt_baddata`; nothing can be reconstructed without decoding the raw stream
  against an unrecoverable entry, so neither is claimed.
- **`0x80AF3850` (88 bytes).** The export calls `0x80AF38B0` with no visible
  argument (the value is still in `r3`) and then collapses an angle snap into a
  reciprocal-multiply division (`0x51EB851F`, `/ 800`, `* 800`) whose rounding the
  C does not state faithfully. Left for a pass that decodes that block word by
  word.
- **`0x80AF42C0` (132 bytes).** Its first action is `0x80AF3840`, a
  `halt_baddata` row; with that callee unrecoverable the gate cannot be
  explained honestly.
- **The ten `GQR` rows of the bucket.** `0x80AF16A0` (312), `0x80AF2400` (284),
  `0x80AF2CF0` (624), `0x80AF3400` (308), `0x80AF3570` (196), `0x80AF5C60`
  (260), `0x80AFD170` (156), `0x80AFD910` (388), `0x80AFDAC0` (480) and
  `0x80AFDD50` (300) all read `unaff_GQR0`/`unaff_GQR3` paired-single data. Per
  `docs/REL_ELF_MAPPING.md` those stay class C until GQR3 is pinned; none is
  claimed.
- **No address already in `ledger/functions.csv` was touched**, and no other
  lane's file, CSV or doc was modified.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 26 sizes sum to 2,888
  bytes (1,488 for the twelve specification rows and 1,400 for the fourteen
  bucket rows).
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 26 was in
  `ledger/functions.csv` before this pass (checked against the address column).
- The module header was updated: cluster item 118, the cluster count (117 to
  118), the function totals (3,149 to 3,175 public translations, 3,078 to 3,104
  `fn_<addr>` bodies), the "start here" list and the records list.
- `cargo check --message-format=short` emits no error and no warning naming
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave136` reports 26 rows for this lane, 54
  unique addresses across the wave and `OK`.
