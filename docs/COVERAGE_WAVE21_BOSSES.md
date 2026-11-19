# Coverage - Wave 21, `d_en_bossNP` (`game::bosses`)

## How to read this file

- **Scope** - the 25 functions added to `src/game/bosses.rs` by the wave-21
  pass over the boss-actor REL `d_en_bossNP` (REL module 4). Earlier passes are
  documented in `docs/COVERAGE_REL_d_en_bossNP.md` (23 functions),
  `docs/COVERAGE_WAVE14_BOSSES.md` (25), `docs/COVERAGE_WAVE15_BOSSES.md` (25),
  `docs/COVERAGE_WAVE16_BOSSES.md` (25), `docs/COVERAGE_WAVE17_BOSSES.md` (25),
  `docs/COVERAGE_WAVE19_BOSSES.md` (25) and `docs/COVERAGE_WAVE20_BOSSES.md`
  (25). This file does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000` with entry `0x80C000F0`. The retail loader picks a different
  base, so every `0x80C...` value is an analysis address, not a runtime
  address. Calls into `main.dol` (for example `0x8015F800`) are absolute DOL
  addresses and are unaffected by the REL base.
- **Evidence** is the Ghidra C under
  `work/ghidra/rel-d_en_bossNP/export/`; the exact file is named in every
  table row. Sizes come from `functions.tsv`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and observable stores are represented; `partial` and `stub` are
  not used in this pass.
- **Confidence** rates the translation, not the guessed role. `high` means the
  stores, call arguments and branch tree are exact; `medium` means the body is
  exact but a dropped argument, a buffer size or a helper role was
  reconstructed from the surrounding code.
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table, so no Nintendo
  symbol is invented.
- Start with the **summary**, then the **clusters**, then the **function
  table**. Read the **unknowns** before treating any medium row as complete.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 25 |
| text bytes added | 3,076 |
| status `translated` | 25 |
| status `partial` | 0 |
| status `stub` | 0 |
| confidence `high` | 14 |
| confidence `medium` | 11 |
| confidence `low` | 0 |
| cumulative functions in `game::bosses` | 366 |
| cumulative bytes in `game::bosses` | 109,464 |

The remaining untranslated bodies of this REL are almost all below 420 bytes
(only `0x80C21BC0`, `0x80C51520` and `0x80C54050` are larger). This pass picks
the coherent families that were still missing: four `+0x30` chain resolvers,
their three name comparators, five release wrappers, three constructors, and
the position/controller/partner helpers that the per-frame handlers call.

## Synthetic-base warning in one place

| item | value |
|---|---|
| canonical base | `0x80C00000` |
| entry | `0x80C000F0` |
| BSS start | `0x80C6C900` |
| BSS size | `0x487C` |

Every `0x80C...` address in this document and in `src/game/bosses.rs` is an
analysis address. A live dump will not match it. The three words at
`0x80377DC8..0x80377DD0` are `main.dol` data, not REL addresses.

## Cluster 1 - chain resolvers and name comparators (1,304 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C09ED0` | 224 | `fn_80c09ed0` | five-link `+0x30` resolver | translated | high |
| `0x80C0E150` | 224 | `fn_80c0e150` | five-link `+0x30` resolver | translated | high |
| `0x80C11A70` | 224 | `fn_80c11a70` | five-link `+0x30` resolver | translated | high |
| `0x80C2A190` | 224 | `fn_80c2a190` | five-link `+0x30` resolver | translated | high |
| `0x80C09FB0` | 136 | `fn_80c09fb0` | `:`-suffix name compare | translated | medium |
| `0x80C11B50` | 136 | `fn_80c11b50` | `:`-suffix name compare | translated | medium |
| `0x80C2A270` | 136 | `fn_80c2a270` | `:`-suffix name compare | translated | medium |

What is proven:

- Each resolver probes exactly five nodes. A probe is
  `(*(node + 0x30))->vtable[+0x0C]`, the wave-3 [`chain_probe`] shape. The
  first nonzero probe result wins.
- After five failed probes the helper recurses on `*(node + 0x30)` of the last
  probe (`0x80C09ED0` line 21 of the export). `0x80C09ED0` and `0x80C2A190`
  are byte-identical; `0x80C0E150` and `0x80C11A70` are byte-identical.
- Each comparator cuts both strings at `0x3A` (`:`), advances the object side
  unconditionally, advances the candidate only when a delimiter exists, and
  compares the remainders with `0x802E1AB4`.
- All three comparators are 136 bytes and use vtable slot `+0x20`; they are
  the same shape as the already-translated `0x80C04A10`/`0x80C06A60`.

## Cluster 2 - releases and constructors (640 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C06C50` | 64 | `fn_80c06c50` | release without base teardown | translated | high |
| `0x80C09CC0` | 88 | `fn_80c09cc0` | base teardown + release | translated | high |
| `0x80C157C0` | 88 | `fn_80c157c0` | base teardown + release | translated | high |
| `0x80C181D0` | 92 | `fn_80c181d0` | base teardown + release | translated | high |
| `0x80C53800` | 88 | `fn_80c53800` | `0x80C55F30` teardown + release | translated | high |
| `0x80C18590` | 92 | `fn_80c18590` | `0xE80` actor constructor | translated | high |
| `0x80C537C0` | 64 | `fn_80c537c0` | owner-2 effect constructor | translated | high |
| `0x80C55490` | 64 | `fn_80c55490` | owner-5 effect constructor | translated | high |

What is proven:

- `0x80C06C50` is the only release of this family with **no** `0x8015F800`
  call. The disassembly at `0x80C06C74` shows `r3` still holding `param_1` for
  `0x802B9280`, although the export prints the call with no argument.
- The three base-teardown releases call `0x8015F800(this, 0)` whenever `this`
  is non-null and release with `0x802B9280(this)` only for `free_it > 0`. The
  comma expression in the C means the teardown is not conditional on
  `free_it`.
- `0x80C18590` allocates `0xE80` through `0x801628C0`, passes the new block to
  `0x800A88A0`, installs `0x80C5F218` at `+0x60`, `0x80C5F838` at `+0xE74`
  and the self pointer at `+0xE78`. The disassembly at `0x80C185B4` proves the
  base-constructor argument that the export drops.
- `0x80C537C0` and `0x80C55490` call the already-translated
  [`fn_80c55db0`] with owner `2` and `5`, then overwrite the sub-object vtable
  at `+0x04` with `0x80C6C650` and `0x80C6C540`.
- `0x80C53800` uses [`fn_80c55f30`]`(this, 0)` instead of `0x8015F800`.

## Cluster 3 - position, controller and partner helpers (1,132 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C051B0` | 76 | `fn_80c051b0` | route-manager position copy | translated | medium |
| `0x80C31630` | 76 | `fn_80c31630` | `+0xC3C` controller step with position | translated | medium |
| `0x80C31690` | 64 | `fn_80c31690` | `+0xC3C` controller reset | translated | medium |
| `0x80C339A0` | 116 | `fn_80c339a0` | partner point forward | translated | medium |
| `0x80C350E0` | 76 | `fn_80c350e0` | `+0x394` step, clear, return `1` | translated | high |
| `0x80C35640` | 140 | `fn_80c35640` | handle matrix to partner | translated | medium |
| `0x80C53E80` | 64 | `fn_80c53e80` | copy point and clamp Z | translated | high |
| `0x80C55A90` | 100 | `fn_80c55a90` | two-slot sixteen-byte relay | translated | medium |
| `0x80C56B90` | 232 | `fn_80c56b90` | mask to lowest-bit index | translated | medium |
| `0x80C56E00` | 188 | `fn_80c56e00` | eight-slot clear | translated | medium |

What is proven:

- `0x80C051B0` calls `0x800AA2A0(manager)`, feeds the returned handle to
  `0x8005F900`, and copies `+0xAC`/`+0xB0`/`+0xB4` into the output. The two
  `bl` stubs at `0x80C051C8`/`0x80C051CC` are consecutive and `r3` carries the
  handle, which proves the composition the export drops.
- `0x80C31630` calls `0x8009CED0(actor)` and then invokes slot `+0xB0` of the
  controller at `+0xC3C` with `(controller, actor + 0xAC, 0, 0)`.
- `0x80C31690` calls `0x8009D090(actor)` and then slot `+0x10` of the same
  controller with only the controller argument.
- `0x80C339A0` and `0x80C35640` both publish through slot `+0x128` of the
  partner selected by `0x80162D00`. The first copies three words; the second
  fills a sixteen-byte block with `0x8006CED0` first.
- `0x80C350E0` is the same 76-byte body as the already-translated
  [`fn_80c4aca0`]: slot `+0x10` on `+0x394`, `0x80064F50(actor, 0)`, return
  `1`.
- `0x80C53E80` copies the input point and clamps its Z: the upper bound at
  `+0x44` wins when it is below Z, then the lower bound at `+0x50` wins when
  it is above Z.
- `0x80C55A90` calls slot `+0x5C` with only a sixteen-byte stack block, then
  slot `+0x70` with `(value, object, block)`.
- `0x80C56B90` returns the index of the lowest set bit of the mask returned by
  [`fn_80c56b20`], or `8` when bits `0..7` are all clear.
- `0x80C56E00` initializes eight slots: `-1` at `+0x194 + 4*k`, zero bytes at
  `+0x1B4 + k` and `+0x27C + k`, and the same three-word vector at
  `+0x1BC + 12*k` and `+0x21C + 12*k`.

## Function table

| address | Ghidra name | Rust function | role | status | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80C051B0` | `FUN_80c051b0` | `fn_80c051b0` | route position copy | translated | medium | `80c051b0_FUN_80c051b0.c` |
| `0x80C06C50` | `FUN_80c06c50` | `fn_80c06c50` | release, no teardown | translated | high | `80c06c50_FUN_80c06c50.c` |
| `0x80C09CC0` | `FUN_80c09cc0` | `fn_80c09cc0` | teardown + release | translated | high | `80c09cc0_FUN_80c09cc0.c` |
| `0x80C09ED0` | `FUN_80c09ed0` | `fn_80c09ed0` | five-link resolver | translated | high | `80c09ed0_FUN_80c09ed0.c` |
| `0x80C09FB0` | `FUN_80c09fb0` | `fn_80c09fb0` | name compare | translated | medium | `80c09fb0_FUN_80c09fb0.c` |
| `0x80C0E150` | `FUN_80c0e150` | `fn_80c0e150` | five-link resolver | translated | high | `80c0e150_FUN_80c0e150.c` |
| `0x80C11A70` | `FUN_80c11a70` | `fn_80c11a70` | five-link resolver | translated | high | `80c11a70_FUN_80c11a70.c` |
| `0x80C11B50` | `FUN_80c11b50` | `fn_80c11b50` | name compare | translated | medium | `80c11b50_FUN_80c11b50.c` |
| `0x80C157C0` | `FUN_80c157c0` | `fn_80c157c0` | teardown + release | translated | high | `80c157c0_FUN_80c157c0.c` |
| `0x80C181D0` | `FUN_80c181d0` | `fn_80c181d0` | teardown + release | translated | high | `80c181d0_FUN_80c181d0.c` |
| `0x80C18590` | `FUN_80c18590` | `fn_80c18590` | `0xE80` constructor | translated | high | `80c18590_FUN_80c18590.c` |
| `0x80C2A190` | `FUN_80c2a190` | `fn_80c2a190` | five-link resolver | translated | high | `80c2a190_FUN_80c2a190.c` |
| `0x80C2A270` | `FUN_80c2a270` | `fn_80c2a270` | name compare | translated | medium | `80c2a270_FUN_80c2a270.c` |
| `0x80C31630` | `FUN_80c31630` | `fn_80c31630` | controller step | translated | medium | `80c31630_FUN_80c31630.c` |
| `0x80C31690` | `FUN_80c31690` | `fn_80c31690` | controller reset | translated | medium | `80c31690_FUN_80c31690.c` |
| `0x80C339A0` | `FUN_80c339a0` | `fn_80c339a0` | partner point forward | translated | medium | `80c339a0_FUN_80c339a0.c` |
| `0x80C350E0` | `FUN_80c350e0` | `fn_80c350e0` | step, clear, return `1` | translated | high | `80c350e0_FUN_80c350e0.c` |
| `0x80C35640` | `FUN_80c35640` | `fn_80c35640` | handle matrix forward | translated | medium | `80c35640_FUN_80c35640.c` |
| `0x80C537C0` | `FUN_80c537c0` | `fn_80c537c0` | owner-2 constructor | translated | high | `80c537c0_FUN_80c537c0.c` |
| `0x80C53800` | `FUN_80c53800` | `fn_80c53800` | teardown + release | translated | high | `80c53800_FUN_80c53800.c` |
| `0x80C53E80` | `FUN_80c53e80` | `fn_80c53e80` | copy point, clamp Z | translated | high | `80c53e80_FUN_80c53e80.c` |
| `0x80C55490` | `FUN_80c55490` | `fn_80c55490` | owner-5 constructor | translated | high | `80c55490_FUN_80c55490.c` |
| `0x80C55A90` | `FUN_80c55a90` | `fn_80c55a90` | two-slot relay | translated | medium | `80c55a90_FUN_80c55a90.c` |
| `0x80C56B90` | `FUN_80c56b90` | `fn_80c56b90` | mask to index | translated | medium | `80c56b90_FUN_80c56b90.c` |
| `0x80C56E00` | `FUN_80c56e00` | `fn_80c56e00` | eight-slot clear | translated | medium | `80c56e00_FUN_80c56e00.c` |

## Data words read from the analysis image

| address | value | used by |
|---|---|---|
| `0x80377DC8` | first word of the runtime vector | `fn_80c56e00` |
| `0x80377DCC` | second word of the runtime vector | `fn_80c56e00` |
| `0x80377DD0` | third word of the runtime vector | `fn_80c56e00` |
| `0x80C5F218` | class vtable for the `0xE80` actor | `fn_80c18590` |
| `0x80C5F838` | `+0xE74` interface vtable | `fn_80c18590` |
| `0x80C6C540` | owner-5 sub-object vtable | `fn_80c55490` |
| `0x80C6C650` | owner-2 sub-object vtable | `fn_80c537c0` |
| `0x80C6C820` | default sub-object vtable installed by [`fn_80c55db0`] | `fn_80c537c0`, `fn_80c55490` |

## Unknowns

- **Dropped arguments.** `0x800AA2A0`, `0x8005F900`, `0x80162D00`,
  `0x8009CED0`, `0x8009D090`, `0x800A88A0` and `0x802B9280` are printed
  without arguments in parts of the export. Where the surrounding disassembly
  proves the register (`0x80C051C8`, `0x80C185B4`, `0x80C31644`,
  `0x80C06C74`) the translation passes it and the row is `high` or documented
  `medium`; the partner lookups `0x80162D00` remain `call0` because the export
  and the callers only prove that `+0x7B0`/`+0x770` gate them.
- **Sixteen-byte buffers.** `0x80C35640` and `0x80C55A90` reserve
  `undefined1 [16]` stack slots. The Rust bodies use `[u32; 4]`; the callees
  are not translated, so the real payload size is not proven.
- **Runtime vector.** The three words at `0x80377DC8` are read as floats and
  copied into every slot. Their values and meaning are not recovered.
- **Slot semantics.** `+0xB0` and `+0x10` of the `+0xC3C` controller and
  `+0x128` of the partner are proven call targets only; their behavior is not
  named.
- **Mask form.** `0x80C56B90` describes its mask argument only through the
  caller at `0x80C56EC0`; the C definition of `0x80C56B90` itself prints
  `void`, so the two-argument form is an inference.
- **Probe owner.** The `+0x30`-chain resolvers are grouped by body shape. The
  object class that owns the chain is not identified.
- **Correction to a wave-20 body.** `fn_80c06980` recursed one link too far
  (`*(n5 + 0x30)` instead of `*(n4 + 0x30)`). The wave-21 pass corrected it;
  the wave-21 resolvers use the corrected 4+1 probe form.

## Verification

- `cargo check` exits 0 and reports no diagnostic for `src/game/bosses.rs`
  (the remaining crate warnings belong to other lanes).
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- All 25 evidence files were read in full; every `# Original` block names the
  file and the size from `functions.tsv`.
- The 25 rows in `ledger/agents/wave21-bosses.csv` match the Rust functions one
  to one: 25 `translated`; 14 `high`, 11 `medium`.
- No files outside `src/game/bosses.rs`, this document and the wave-21 ledger
  were edited.
