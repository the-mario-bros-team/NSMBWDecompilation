# Coverage - Wave 87 (`game::bases`, `0x8074` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** under the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader chooses the real load address, so the numbers only
identify positions inside the REL image. Cross-module pointers (`0x8037xxxx`,
`0x8042xxxx`, `0x807Cxxxx`) are `main.dol` data addresses imported by the module
and keep their absolute values.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`. "Role" below is the
plain-English behaviour. Confidence is `high` when guards, stores, constants and
call arguments are literal, and `medium` when the role of a field, a global, a
callee or a dropped argument had to be inferred (those bodies carry
`// observed:`/`// inferred:`/`// unknown:` notes). No body in this pass is
`low`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 29 | 3,192 | 20 | 9 |

## What this pass covers

The `0x8074` bucket holds 233 functions; 95 were already in
`ledger/functions.csv` when this pass began, leaving 138. This pass takes 29 of
the remaining bodies. The bucket is crowded with paired-single (GQR) arithmetic
that Ghidra decompiles with `unaff_GQR0`/`ldexpf` placeholders, so the pass
deliberately stayed on the clean sub-runs: two screen-rectangle actor families
with identical field layouts, plus four small allocators.

- **Screen-rectangle family 1, `0x80749510..0x8074A390` (12 bodies).**
  [`fn_80749510`] refreshes a cached rectangle from a player's view with
  `0x8008E890` (four floats: `x`, `y`, `width`, `height`, stored as left, top,
  right and `top - height` at `+0x408..+0x414`); [`fn_80749650`] is its
  closed-rectangle point test and [`fn_807495b0`] asks whether any active
  player is outside it, using the `0x80429CA8` per-player bit mask. The slot
  history at `+0x3EC..+0x3FC` is shifted by [`fn_807496b0`]; its two timers are
  [`fn_807496e0`] (halved nibble count) and [`fn_80749710`] (nibble count, or
  `0x8C` plus RNG jitter). [`fn_807498c0`] counts how often each of the eight
  indices appears in that history and samples the `0x807CE540` shuffle table up
  to 30 times. [`fn_80749c00`] copies a player's `+0xAC..+0xB4` words, and
  [`fn_8074a1b0`] is the family's `0x8015F800`/`0x802B9280` release helper. The
  two 12-byte bodies [`fn_8074a380`]/[`fn_8074a390`] both return the constant
  `0x80377A04`, and [`fn_8074e380`] counts a `0x59`-type child chain.
- **Screen-rectangle family 2, `0x8074B1F0..0x8074DA00` (17 bodies).** The
  same shape with the rectangle anchored at `+0x3D0`: [`fn_8074b770`] tests all
  four players against it, [`fn_8074c280`] and [`fn_8074da00`] test one player
  using the index byte at `+0x433` / `+0x3DC`, and [`fn_8074ba10`] rejects an
  x that is already covered by one of three recorded distances at `+0x3E4`.
  [`fn_8074c350`] compacts the two-entry slot array through `0x80162D00`, while
  [`fn_8074c1f0`] and its byte-identical twin [`fn_8074d940`] release the actor
  when its character byte `+0x349` is no longer used by a player.
  [`fn_8074b390`] walks five `+0x30` links before recursing,
  [`fn_8074b470`] compares a class name after the last colon, and
  [`fn_8074b880`] plays cue `0x86`. The four small allocators are
  [`fn_8074b590`] (`0x3F8`, vtable `0x808116C0`), [`fn_8074c0b0`] (`0x438`,
  `0x808118B0`), [`fn_8074d1c0`] (`0x3D8`, `0x80811A70`) and [`fn_8074d6a0`]
  (`0x3A8`, `0x80811BF0`, base constructor `0x80064110` instead of
  `0x80066FC0`). [`fn_8074b1f0`] repeats the release helper and
  [`fn_8074d9d0`] returns the scene/camera height difference; [`fn_8074db10`]
  is the eight-step counter.

One host-side helper was added for this pass: `stage_view()` reproduces the
three-instruction `0x80429E98[stage_info[0x120E]] + 0x3B0 * index` expression
that appears inline in six bodies. No other helper was needed; the pass reuses
`abi_call1..5`, `vmethod0`, `ptr_at` and `call0_ret`.

## Functions

| address | function | bytes | confidence | role |
|---|---|---:|---|---|
| `0x80749510` | `fn_80749510` | 152 | high | screen rectangle refresh |
| `0x807495B0` | `fn_807495b0` | 160 | medium | any active player outside rect |
| `0x80749650` | `fn_80749650` | 96 | high | closed-rectangle point test |
| `0x807496B0` | `fn_807496b0` | 40 | high | five-word slot history shift |
| `0x807496E0` | `fn_807496e0` | 48 | high | timer from nibble, halved |
| `0x80749710` | `fn_80749710` | 56 | high | timer from nibble with jitter |
| `0x807498C0` | `fn_807498c0` | 272 | medium | eight-index use count + table shuffle |
| `0x80749C00` | `fn_80749c00` | 72 | high | copy player position words |
| `0x8074A1B0` | `fn_8074a1b0` | 88 | high | release helper |
| `0x8074A380` | `fn_8074a380` | 12 | high | constant `0x80377A04` |
| `0x8074A390` | `fn_8074a390` | 12 | high | constant `0x80377A04` |
| `0x8074E380` | `fn_8074e380` | 100 | high | recursive `0x59` child count |
| `0x8074B1F0` | `fn_8074b1f0` | 88 | high | release helper |
| `0x8074B390` | `fn_8074b390` | 224 | medium | five-link child walk |
| `0x8074B470` | `fn_8074b470` | 136 | medium | class-name compare after colon |
| `0x8074B590` | `fn_8074b590` | 76 | high | alloc `0x3F8` + vtable |
| `0x8074B770` | `fn_8074b770` | 268 | medium | player-in-rect query (`+0x3D0`) |
| `0x8074B880` | `fn_8074b880` | 76 | high | sound cue `0x86` |
| `0x8074BA10` | `fn_8074ba10` | 120 | medium | three recorded slot distances |
| `0x8074C0B0` | `fn_8074c0b0` | 76 | high | alloc `0x438` + vtable |
| `0x8074C1F0` | `fn_8074c1f0` | 140 | high | duplicate-character gate |
| `0x8074C280` | `fn_8074c280` | 196 | medium | player-in-rect query (`+0x433`) |
| `0x8074C350` | `fn_8074c350` | 128 | medium | two-slot compaction |
| `0x8074D1C0` | `fn_8074d1c0` | 76 | high | alloc `0x3D8` + vtable |
| `0x8074D6A0` | `fn_8074d6a0` | 76 | high | alloc `0x3A8` + vtable |
| `0x8074D940` | `fn_8074d940` | 140 | high | duplicate-character gate twin |
| `0x8074D9D0` | `fn_8074d9d0` | 36 | high | scene/camera height difference |
| `0x8074DA00` | `fn_8074da00` | 196 | medium | player-in-rect query (`+0x3DC`) |
| `0x8074DB10` | `fn_8074db10` | 32 | high | eight-step counter wrap |

## Address range

`0x80749510..0x8074E380`, two groups plus a short tail:

- Group 1 is `0x80749510..0x8074A390` (11 bodies, contiguous apart from the
  deferred `0x80749750`), with `0x8074E380` just outside the run. The passed
  over `0x80749C50` (164 bytes) sits between [`fn_80749c00`] and
  [`fn_80749f60`].
- Group 2 is `0x8074B1F0..0x8074DA00` (17 bodies). Inside the run the two
  `0x802DCBFC` adapters `0x8074B530`/`0x8074B560`, the paired-single body
  `0x8074B8D0`, `0x8074BB00` and the three-state driver `0x8074DD30` remain
  untranslated.
- `0x8074DB10` is the last body of the pass. After this pass the bucket still
  has 109 untranslated functions.

## Deliberate skips

The specific bodies below were read and rejected rather than guessed; the rest
of the bucket was simply not reached.

- `0x80749C50` (164 bytes) interpolates two player positions. Its first call to
  `FUN_80749c00` is printed with no arguments (`FUN_80749c00(&local_1c)`), so
  the player index for the first sample is unrecoverable; only the second call
  shows `(param_2, param_4)`. Writing the first call would require guessing
  which caller argument fills the lost register.
- `0x80749F60` (216 bytes) is the family's per-frame driver. It consumes the
  result of `FUN_807498c0`, which its own export declares `void`, and calls the
  untranslated `0x80749750` and `0x807499D0`, so both the value and the two
  callees would be invented.
- `0x8074B250`, `0x8074B530`, `0x8074B560`, `0x8074C050`, `0x8074C080`,
  `0x8074D160`, `0x8074D190`, `0x8074D640`, `0x8074D670` (48 bytes each) are
  the identical two-argument adapters `0x802DCBFC(param_2, param_2, param_1)`.
  The export duplicates one register and loses both roles, so no honest
  signature exists; this repeats the finding in `docs/COVERAGE_WAVE86_BASES.md`.
- `0x8074DAD0` (56 bytes) mixes a signed division-by-two idiom with an
  ambiguous `>> 0x1f` precedence; the reconstructed value cannot be proven from
  the printed expression.
- `0x80745D10`, `0x80745DC0`, `0x80746640`, `0x80746910`, `0x80746C20`,
  `0x80746F90`, `0x80747120`, `0x80747B30`, `0x807481B0`, `0x807485D0`,
  `0x80748750`, `0x80748B20`, `0x80749750`, `0x80749D00`, `0x80749DE0`,
  `0x80749E90`, `0x8074A4F0`, `0x8074B8D0`, `0x8074BB00`, `0x8074C3D0`,
  `0x8074C540`, `0x8074CA70`, `0x8074CBA0`, `0x8074DB30`, `0x8074DBA0` and
  `0x8074F600` all carry `unaff_GQR0`/`ldexpf` paired-single placeholders in
  their exports and were left for a pass that can recover the GQR context.
- Not reached in this pass: the `0x80740370..0x80742470` collision/timer
  block, the `0x807433A0..0x80743F20` torch family (only partially reached; the
  `center_fire` loader `0x80743790` and constructor `0x807435A0` remain), the
  `0x8074E3F0..0x8074ED80` tail and the `0x8074F3C0` RNG table reader.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (29 rows, 3,192 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the 95 pre-existing rows of
  this bucket were listed before writing and none is in this pass.
- `node tools/check-lane-csv.mjs wave87` reports
  `wave87-bases.csv: 29 rows`, `unique addresses: 29`,
  `translated/high=20 translated/medium=9`, `OK`.
- `cargo check --message-format=short` produces no diagnostic for
  `src/game/bases.rs`.
