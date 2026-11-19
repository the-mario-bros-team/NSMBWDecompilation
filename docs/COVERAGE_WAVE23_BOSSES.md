# Coverage - Wave 23, `d_en_bossNP` (`game::bosses`)

## How to read this file

- **Scope** - the 25 functions added to `src/game/bosses.rs` by the wave-23
  pass over the boss-actor REL `d_en_bossNP` (REL module 4). Earlier passes are
  documented in `docs/COVERAGE_REL_d_en_bossNP.md` (23 functions) and
  `docs/COVERAGE_WAVE14_BOSSES.md` through
  `docs/COVERAGE_WAVE22_BOSSES.md` (25 functions each, except wave 18). This
  file does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000` with entry `0x80C000F0`. The retail loader picks a different
  base, so every `0x80C...` value is an analysis address, not a runtime
  address. Calls into `main.dol` (for example `0x8005F900`) are absolute DOL
  addresses and are unaffected by the REL base.
- **Evidence** is the Ghidra C under
  `work/ghidra/rel-d_en_bossNP/export/`; the exact file is named in every
  table row. Sizes come from `functions.tsv`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and observable stores are represented; `partial` and `stub` are
  not used in this pass.
- **Confidence** rates the translation, not the guessed role. `high` means the
  stores, call arguments and branch tree are exact; `medium` means the body is
  exact but a helper role or callee name was reconstructed from the surrounding
  code.
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table, so no Nintendo
  symbol is invented.
- Start with the **summary**, then the **clusters**, then the **function
  table**. Read the **unknowns** before treating any medium row as complete.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 25 |
| text bytes added | 4,460 |
| status `translated` | 25 |
| status `partial` | 0 |
| status `stub` | 0 |
| confidence `high` | 17 |
| confidence `medium` | 8 |
| confidence `low` | 0 |
| cumulative functions in `game::bosses` | 416 |
| cumulative bytes in `game::bosses` | 116,384 |

This pass clears the last large repeated shape in the module: the fourteen
remaining 224-byte five-link `+0x30` resolvers and eight of their 136-byte name
comparators. It also takes the last complete standalone gameplay helpers that
do not depend on the corrupted GQR/paired-single export: the partner handle
forwarder `0x80C47210`, the controller step `0x80C18970` and the word swap
`0x80C1DF40`.

## Synthetic-base warning in one place

| item | value |
|---|---|
| canonical base | `0x80C00000` |
| entry | `0x80C000F0` |
| BSS start | `0x80C6C900` |
| BSS size | `0x487C` |

Every `0x80C...` address in this document and in `src/game/bosses.rs` is an
analysis address. A live dump will not match it.

## Cluster 1 - five-link chain resolvers (3,136 bytes)

Each resolver is the same nested tree. It probes `(*(node + 0x30))->vtable
[+0x0C]` four times, advancing one `+0x30` link after every zero result; a
fifth probe runs on the fifth node, and a zero result recurses on that node's
`+0x30` link. The first node with a nonzero probe is returned.

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C18390` | 224 | `fn_80c18390` | chain resolver | translated | high |
| `0x80C213C0` | 224 | `fn_80c213c0` | chain resolver | translated | high |
| `0x80C293C0` | 224 | `fn_80c293c0` | chain resolver | translated | high |
| `0x80C2C150` | 224 | `fn_80c2c150` | chain resolver | translated | high |
| `0x80C2E290` | 224 | `fn_80c2e290` | chain resolver | translated | high |
| `0x80C2FEB0` | 224 | `fn_80c2feb0` | chain resolver | translated | high |
| `0x80C32150` | 224 | `fn_80c32150` | chain resolver | translated | high |
| `0x80C34AD0` | 224 | `fn_80c34ad0` | chain resolver | translated | high |
| `0x80C37AB0` | 224 | `fn_80c37ab0` | chain resolver | translated | high |
| `0x80C3D3A0` | 224 | `fn_80c3d3a0` | chain resolver | translated | high |
| `0x80C42A80` | 224 | `fn_80c42a80` | chain resolver | translated | high |
| `0x80C46640` | 224 | `fn_80c46640` | chain resolver | translated | high |
| `0x80C4A670` | 224 | `fn_80c4a670` | chain resolver | translated | high |
| `0x80C4F1A0` | 224 | `fn_80c4f1a0` | chain resolver | translated | high |

What is proven:

- All fourteen C bodies are identical after relocation. The probe receiver is
  always `*(node + 0x30)` and the slot is always `+0x0C`.
- The recursion target is the `+0x30` link of the fifth probed node, not the
  fifth node itself.
- The Rust bodies share the private [`chain_probe`] helper, exactly as the
  wave-20/21/22 resolvers already do, so no vtable read is skipped.

## Cluster 2 - name comparators (1,088 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C18470` | 136 | `fn_80c18470` | `:`-suffix name compare | translated | medium |
| `0x80C214A0` | 136 | `fn_80c214a0` | `:`-suffix name compare | translated | medium |
| `0x80C294A0` | 136 | `fn_80c294a0` | `:`-suffix name compare | translated | medium |
| `0x80C2C230` | 136 | `fn_80c2c230` | `:`-suffix name compare | translated | medium |
| `0x80C2E370` | 136 | `fn_80c2e370` | `:`-suffix name compare | translated | medium |
| `0x80C2FF90` | 136 | `fn_80c2ff90` | `:`-suffix name compare | translated | medium |
| `0x80C32230` | 136 | `fn_80c32230` | `:`-suffix name compare | translated | medium |
| `0x80C34BB0` | 136 | `fn_80c34bb0` | `:`-suffix name compare | translated | medium |

What is proven:

- The candidate string is cut at the first `0x3A`, and the candidate side
  advances only when a colon exists.
- The object side always advances past the character at the returned index,
  even when `0x802E1C40` returned null; this is the literal C behavior.
- The object name comes from vtable slot `+0x20` on the object itself.

## Cluster 3 - partner handle forward, controller step and word swap (236 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C18970` | 76 | `fn_80c18970` | controller step, clear, return `1` | translated | high |
| `0x80C1DF40` | 20 | `fn_80c1df40` | swap two 32-bit words | translated | high |
| `0x80C47210` | 140 | `fn_80c47210` | partner handle forward | translated | high |

What is proven:

- `0x80C18970` is byte-identical to the wave-22 `0x80C3DAB0`: slot `+0x10` on
  the `+0x394` controller, then `0x80064F50(this, 0)`, return `1`.
- `0x80C47210` is byte-identical to the wave-22 `0x80C43670` after relocation:
  `0x8005F900(+0x764)`, the `+0x770` gate around `0x80162D00`, a sixteen-byte
  `0x8006CED0` block and partner slot `+0x128`.
- `0x80C1DF40` performs the three literal moves of a word exchange.

## Function table

| address | Ghidra name | Rust function | role | status | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80C18390` | `FUN_80c18390` | `fn_80c18390` | chain resolver | translated | high | `80c18390_FUN_80c18390.c` |
| `0x80C18470` | `FUN_80c18470` | `fn_80c18470` | name compare | translated | medium | `80c18470_FUN_80c18470.c` |
| `0x80C18970` | `FUN_80c18970` | `fn_80c18970` | controller step | translated | high | `80c18970_FUN_80c18970.c` |
| `0x80C1DF40` | `FUN_80c1df40` | `fn_80c1df40` | word swap | translated | high | `80c1df40_FUN_80c1df40.c` |
| `0x80C213C0` | `FUN_80c213c0` | `fn_80c213c0` | chain resolver | translated | high | `80c213c0_FUN_80c213c0.c` |
| `0x80C214A0` | `FUN_80c214a0` | `fn_80c214a0` | name compare | translated | medium | `80c214a0_FUN_80c214a0.c` |
| `0x80C293C0` | `FUN_80c293c0` | `fn_80c293c0` | chain resolver | translated | high | `80c293c0_FUN_80c293c0.c` |
| `0x80C294A0` | `FUN_80c294a0` | `fn_80c294a0` | name compare | translated | medium | `80c294a0_FUN_80c294a0.c` |
| `0x80C2C150` | `FUN_80c2c150` | `fn_80c2c150` | chain resolver | translated | high | `80c2c150_FUN_80c2c150.c` |
| `0x80C2C230` | `FUN_80c2c230` | `fn_80c2c230` | name compare | translated | medium | `80c2c230_FUN_80c2c230.c` |
| `0x80C2E290` | `FUN_80c2e290` | `fn_80c2e290` | chain resolver | translated | high | `80c2e290_FUN_80c2e290.c` |
| `0x80C2E370` | `FUN_80c2e370` | `fn_80c2e370` | name compare | translated | medium | `80c2e370_FUN_80c2e370.c` |
| `0x80C2FEB0` | `FUN_80c2feb0` | `fn_80c2feb0` | chain resolver | translated | high | `80c2feb0_FUN_80c2feb0.c` |
| `0x80C2FF90` | `FUN_80c2ff90` | `fn_80c2ff90` | name compare | translated | medium | `80c2ff90_FUN_80c2ff90.c` |
| `0x80C32150` | `FUN_80c32150` | `fn_80c32150` | chain resolver | translated | high | `80c32150_FUN_80c32150.c` |
| `0x80C32230` | `FUN_80c32230` | `fn_80c32230` | name compare | translated | medium | `80c32230_FUN_80c32230.c` |
| `0x80C34AD0` | `FUN_80c34ad0` | `fn_80c34ad0` | chain resolver | translated | high | `80c34ad0_FUN_80c34ad0.c` |
| `0x80C34BB0` | `FUN_80c34bb0` | `fn_80c34bb0` | name compare | translated | medium | `80c34bb0_FUN_80c34bb0.c` |
| `0x80C37AB0` | `FUN_80c37ab0` | `fn_80c37ab0` | chain resolver | translated | high | `80c37ab0_FUN_80c37ab0.c` |
| `0x80C3D3A0` | `FUN_80c3d3a0` | `fn_80c3d3a0` | chain resolver | translated | high | `80c3d3a0_FUN_80c3d3a0.c` |
| `0x80C42A80` | `FUN_80c42a80` | `fn_80c42a80` | chain resolver | translated | high | `80c42a80_FUN_80c42a80.c` |
| `0x80C46640` | `FUN_80c46640` | `fn_80c46640` | chain resolver | translated | high | `80c46640_FUN_80c46640.c` |
| `0x80C47210` | `FUN_80c47210` | `fn_80c47210` | partner handle forward | translated | high | `80c47210_FUN_80c47210.c` |
| `0x80C4A670` | `FUN_80c4a670` | `fn_80c4a670` | chain resolver | translated | high | `80c4a670_FUN_80c4a670.c` |
| `0x80C4F1A0` | `FUN_80c4f1a0` | `fn_80c4f1a0` | chain resolver | translated | high | `80c4f1a0_FUN_80c4f1a0.c` |

## Unknowns

- **Resolver ownership.** The fourteen resolvers are identical after
  relocation, so their per-boss owners cannot be distinguished from the body
  alone. They are ranked `high` because the instruction flow and the vtable
  slot are exact, not because the owning class is known.
- **Name comparison callees.** `0x802E1C40` and `0x802E1AB4` are treated as
  `strchr` and `strcmp` from their calling shape only; the DOL has no symbol
  table entry for them in this workspace.
- **Dropped partner argument.** `0x80162D00` in `fn_80c47210` is printed
  without arguments, exactly as in the wave-22 `fn_80c43670`. It is kept as a
  no-argument `call0_ret` and marked `high` only because the body is
  byte-identical to the already-translated twin.
- **Collision word.** `+0x764` is called a collision handle because the same
  offset feeds `0x8005F900` in `fn_80c191a0` and `fn_80c43670`; the callee's
  internal role is not translated here.
- **Word swap callers.** `0x80C1DF40` has no translated caller yet; the
  exchange could be a matrix row swap or a generic two-word sort step.

## Remaining candidates

| address | size | why deferred |
|---|---:|---|
| `0x80C21BC0` | 1,056 | large castle AI; export is dominated by GQR/paired-single artifacts |
| `0x80C54050` | 864 | aim/collision position update; same artifact problem |
| `0x80C51520` | 444 | route population; `0x802DCD74` pair and dropped arguments unresolved |
| `0x80C43B20` | 372 | vertical aim clamp; GQR artifact problem |
| `0x80C44110` | 116 | aim wrapper for `0x80C43B20`; GQR artifact problem |
| `0x80C2EBE0` | 184 | six-slot effect sweep; GQR artifact problem |
| `0x80C30940` | 184 | six-slot effect sweep; GQR artifact problem |
| 5 bodies | 136 | remaining name comparators (`0x80C37B90`, `0x80C3D480`, `0x80C42B60`, `0x80C46720`, `0x80C4F280`) |
| 45 bodies | 48 | `0x802DCBFC` destructor adapters |
| 19 bodies | 88 / 92 | release wrappers around `0x8015F800` or `0x80C55F30` |

The five remaining comparators are the same shape as the eight in this pass;
they were left for a later wave only to keep this pass at 25 bodies.

## Verification

- `cargo check` exits 0 and reports no diagnostic for `src/game/bosses.rs`
  (the remaining crate warnings belong to other lanes).
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- All 25 evidence files were inspected; each family member matched the
  reference body line for line, and every `# Original` block names the file
  and the size from `functions.tsv`.
- The 25 rows in `ledger/agents/wave23-bosses.csv` match the Rust functions
  one to one: 25 `translated`; 17 `high`, 8 `medium`.
- No files outside `src/game/bosses.rs`, this document and the wave-23 ledger
  were edited.
