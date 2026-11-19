# Coverage - Wave 22, `d_en_bossNP` (`game::bosses`)

## How to read this file

- **Scope** - the 25 functions added to `src/game/bosses.rs` by the wave-22
  pass over the boss-actor REL `d_en_bossNP` (REL module 4). Earlier passes are
  documented in `docs/COVERAGE_REL_d_en_bossNP.md` (23 functions),
  `docs/COVERAGE_WAVE14_BOSSES.md` through `docs/COVERAGE_WAVE21_BOSSES.md`
  (25 functions each, except wave 18). This file does not repeat them.
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
  exact but a dropped argument, a virtual receiver or a helper role was
  reconstructed from the surrounding code.
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table, so no Nintendo
  symbol is invented.
- Start with the **summary**, then the **clusters**, then the **function
  table**. Read the **unknowns** before treating any medium row as complete.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 25 |
| text bytes added | 2,460 |
| status `translated` | 25 |
| status `partial` | 0 |
| status `stub` | 0 |
| confidence `high` | 9 |
| confidence `medium` | 16 |
| confidence `low` | 0 |
| cumulative functions in `game::bosses` | 391 |
| cumulative bytes in `game::bosses` | 111,924 |

The remaining untranslated bodies are dominated by three repeated shapes: the
48-byte destructor adapter (`0x802DCBFC`), the 88/92-byte release wrapper
(`0x8015F800` or `0x80C55F30` plus `0x802B9280`), and the 224/136-byte
`+0x30` chain resolver/name comparator pair. This pass takes one last pair of
that chain family (`0x80C15A20`/`0x80C15B00`) and instead spends most of its
budget on the four allocating factories, the Kameck route builder, the route
tile probe, two effect teardowns, five phase entries, three timer/flag helpers
and the motion integrator.

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

## Cluster 1 - allocating factories (236 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C2E490` | 48 | `fn_80c2e490` | `0xD98` W4 factory | translated | medium |
| `0x80C300B0` | 48 | `fn_80c300b0` | `0xD78` W6 factory | translated | medium |
| `0x80C32350` | 48 | `fn_80c32350` | `0xCB8` W8 factory | translated | medium |
| `0x80C42C80` | 92 | `fn_80c42c80` | `0xEA0` factory with two vtables | translated | medium |

What is proven:

- Each factory calls `0x801628C0(size)` and only runs its constructor when the
  allocation is non-null.
- `0x80C2E490`, `0x80C300B0` and `0x80C32350` dispatch to the already
  translated constructors `0x80C2E4D0`, `0x80C30150` and `0x80C32380`.
- `0x80C42C80` installs class vtable `0x80C68998` at `+0x60`, interface vtable
  `0x80C68FE8` at `+0xE74` and the self pointer at `+0xE78`, the same shape as
  the wave-21 `0x80C18590` factory.

## Cluster 2 - teardowns, phase entries, timers and motion (544 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C05380` | 52 | `fn_80c05380` | effect teardown A | translated | high |
| `0x80C08D80` | 52 | `fn_80c08d80` | clear child flag `+0x89D` | translated | medium |
| `0x80C0FEB0` | 68 | `fn_80c0feb0` | one-shot edge marker | translated | medium |
| `0x80C102A0` | 52 | `fn_80c102a0` | effect teardown B | translated | high |
| `0x80C109A0` | 76 | `fn_80c109a0` | phase entry after `0x80C0EBF0` | translated | medium |
| `0x80C3FD90` | 52 | `fn_80c3fd90` | reset and raise `+0x360` | translated | medium |
| `0x80C52670` | 192 | `fn_80c52670` | position/global integrator | translated | medium |

What is proven:

- `0x80C05380` and `0x80C102A0` are the two teardown variants:
  `0x800AE420`/`0x8008C380` and `0x800AE500`/`0x8008C330`, both against the
  embedded object at `+0x148`.
- `0x80C08D80` dispatches actor slot `+0x658` and clears `+0x89D` on the
  returned child.
- `0x80C0FEB0` writes `leading_zeros(*(byte *)(this + 0xED9)) >> 5` to
  `+0xED8`, calls `0x800AC0B0`, then sets `+0xED9 = 1`.
- `0x80C109A0` always calls the translated `0x80C0EBF0`; only a zero `+0x500`
  halfword dispatches slot `+0x494`.
- `0x80C3FD90` calls `0x800AC7A0` and stores `1` at `+0x360`.
- `0x80C52670` advances `+0x24`/`+0x28` by `0x80C592B8`, `+0x2C` by the
  difference of the scene object's `+0xB4`/`+0xC0` fields (scene pointer read
  from `0x8042A1A0`), mirrors the same deltas into
  `0x80C71170..0x80C71178`, and dispatches slot `+0x4C` when asked.

## Cluster 3 - Kameck route builder and tile probe (744 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C51460` | 188 | `fn_80c51460` | two-slot route builder | translated | medium |
| `0x80C516E0` | 204 | `fn_80c516e0` | `0x38`-byte route element | translated | medium |
| `0x80C56EC0` | 352 | `fn_80c56ec0` | route-tile collision probe | translated | medium |

What is proven:

- `0x80C51460` initializes the static path pointers at
  `0x80C71128`/`0x80C7112C` from `0x80C6C148`/`0x80C6C15C` behind the
  `0x80C71130` guard, then fills both slots with `0x80C51520` and
  `0x80C516E0`.
- `0x80C516E0` allocates `0x38` bytes through `0x802B9210`, stores the
  `0x80103AC0` handle at `slot + 0x14`, and passes a nine-word descriptor made
  from `0x80C6C194..0x80C6C1A4`, `0x80377DC8..0x80377DD0` and the caller's path
  value to `0x80103D40`.
- `0x80C56EC0` reads the route table and actor from the `0x802DCD6C` pair,
  maps the input mask with `0x80C56B20`/`0x80C56B90`, resolves the five-byte
  row with `0x801002C0`, tests it with `0x800F57A0`, skips rows whose
  `0x800F9970` result is `1`, and publishes the final flag through
  `0x802DCDB8`.

## Cluster 4 - state entries, timers, flags and partner forwarding (576 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C2C8A0` | 48 | `fn_80c2c8a0` | dispatch `+0x548` slot `+0x14` | translated | medium |
| `0x80C38FD0` | 44 | `fn_80c38fd0` | `Wm_le_magicattack_st` entry | translated | medium |
| `0x80C3DAB0` | 76 | `fn_80c3dab0` | controller step, clear, return `1` | translated | high |
| `0x80C43670` | 140 | `fn_80c43670` | partner handle forwarder | translated | high |
| `0x80C517B0` | 64 | `fn_80c517b0` | latch/countdown helper | translated | high |
| `0x80C517F0` | 72 | `fn_80c517f0` | cancel active `+0x5C` timer | translated | medium |
| `0x80C56240` | 40 | `fn_80c56240` | set/clear `+0x74` flag | translated | high |
| `0x80C56270` | 40 | `fn_80c56270` | set/clear `+0x78` flag | translated | high |
| `0x80C56910` | 52 | `fn_80c56910` | flag-driven vector copies | translated | high |

What is proven:

- `0x80C38FD0` calls slot `+0xA0` of the `+0xEC4` controller with the state
  name `0x80C663B0`, `0`, the `+0x774` payload and two zeros.
- `0x80C3DAB0` is byte-identical to the already-translated `0x80C350E0`:
  slot `+0x10` on `+0x394`, then `0x80064F50(this, 0)`, return `1`.
- `0x80C43670` is byte-identical to the already-translated `0x80C191A0`:
  `0x8005F900(+0x764)`, the `+0x770` gate around `0x80162D00`, a sixteen-byte
  `0x8006CED0` block and partner slot `+0x128`. The Rust dispatch reads the
  partner vtable at `target + 0x60`, as the C expression requires.
- `0x80C517B0` latches `initial` into a `-1` halfword, decrements positive
  values, returns `1` on expiry and resets the field to `-1`.
- `0x80C56240` and `0x80C56270` are the same OR/AND-NOT pair against `+0x74`
  and `+0x78`; `0x80C56910` copies `+0x18` to `+0x0C` on bit `4` and `+0x20`/
  `+0x1C` to `+0x14`/`+0x10` on bit `8`.

## Cluster 5 - last chain resolver and comparator (360 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C15A20` | 224 | `fn_80c15a20` | five-link `+0x30` resolver | translated | high |
| `0x80C15B00` | 136 | `fn_80c15b00` | `:`-suffix name compare | translated | medium |

What is proven:

- `0x80C15A20` is byte-identical to the wave-21 `0x80C09ED0` after relocation:
  four probes, then one probe at the fifth node, then recursion on its `+0x30`
  link.
- `0x80C15B00` is the same comparator shape as `0x80C09FB0`: `0x802E1C40`
  finds `:`, vtable slot `+0x20` supplies the object name, and `0x802E1AB4`
  compares the two suffixes.

## Function table

| address | Ghidra name | Rust function | role | status | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80C05380` | `FUN_80c05380` | `fn_80c05380` | effect teardown A | translated | high | `80c05380_FUN_80c05380.c` |
| `0x80C08D80` | `FUN_80c08d80` | `fn_80c08d80` | child flag clear | translated | medium | `80c08d80_FUN_80c08d80.c` |
| `0x80C0FEB0` | `FUN_80c0feb0` | `fn_80c0feb0` | edge marker | translated | medium | `80c0feb0_FUN_80c0feb0.c` |
| `0x80C102A0` | `FUN_80c102a0` | `fn_80c102a0` | effect teardown B | translated | high | `80c102a0_FUN_80c102a0.c` |
| `0x80C109A0` | `FUN_80c109a0` | `fn_80c109a0` | phase entry | translated | medium | `80c109a0_FUN_80c109a0.c` |
| `0x80C15A20` | `FUN_80c15a20` | `fn_80c15a20` | five-link resolver | translated | high | `80c15a20_FUN_80c15a20.c` |
| `0x80C15B00` | `FUN_80c15b00` | `fn_80c15b00` | name compare | translated | medium | `80c15b00_FUN_80c15b00.c` |
| `0x80C2C8A0` | `FUN_80c2c8a0` | `fn_80c2c8a0` | `+0x548` dispatch | translated | medium | `80c2c8a0_FUN_80c2c8a0.c` |
| `0x80C2E490` | `FUN_80c2e490` | `fn_80c2e490` | W4 factory | translated | medium | `80c2e490_FUN_80c2e490.c` |
| `0x80C300B0` | `FUN_80c300b0` | `fn_80c300b0` | W6 factory | translated | medium | `80c300b0_FUN_80c300b0.c` |
| `0x80C32350` | `FUN_80c32350` | `fn_80c32350` | W8 factory | translated | medium | `80c32350_FUN_80c32350.c` |
| `0x80C38FD0` | `FUN_80c38fd0` | `fn_80c38fd0` | magic-attack entry | translated | medium | `80c38fd0_FUN_80c38fd0.c` |
| `0x80C3DAB0` | `FUN_80c3dab0` | `fn_80c3dab0` | controller step | translated | high | `80c3dab0_FUN_80c3dab0.c` |
| `0x80C3FD90` | `FUN_80c3fd90` | `fn_80c3fd90` | reset and latch | translated | medium | `80c3fd90_FUN_80c3fd90.c` |
| `0x80C42C80` | `FUN_80c42c80` | `fn_80c42c80` | `0xEA0` factory | translated | medium | `80c42c80_FUN_80c42c80.c` |
| `0x80C43670` | `FUN_80c43670` | `fn_80c43670` | partner forwarder | translated | high | `80c43670_FUN_80c43670.c` |
| `0x80C51460` | `FUN_80c51460` | `fn_80c51460` | route builder | translated | medium | `80c51460_FUN_80c51460.c` |
| `0x80C516E0` | `FUN_80c516e0` | `fn_80c516e0` | route element | translated | medium | `80c516e0_FUN_80c516e0.c` |
| `0x80C517B0` | `FUN_80c517b0` | `fn_80c517b0` | countdown helper | translated | high | `80c517b0_FUN_80c517b0.c` |
| `0x80C517F0` | `FUN_80c517f0` | `fn_80c517f0` | timer cancel | translated | medium | `80c517f0_FUN_80c517f0.c` |
| `0x80C52670` | `FUN_80c52670` | `fn_80c52670` | motion integrator | translated | medium | `80c52670_FUN_80c52670.c` |
| `0x80C56240` | `FUN_80c56240` | `fn_80c56240` | `+0x74` flag | translated | high | `80c56240_FUN_80c56240.c` |
| `0x80C56270` | `FUN_80c56270` | `fn_80c56270` | `+0x78` flag | translated | high | `80c56270_FUN_80c56270.c` |
| `0x80C56910` | `FUN_80c56910` | `fn_80c56910` | flag-driven copies | translated | high | `80c56910_FUN_80c56910.c` |
| `0x80C56EC0` | `FUN_80c56ec0` | `fn_80c56ec0` | route-tile probe | translated | medium | `80c56ec0_FUN_80c56ec0.c` |

## Data words read from the analysis image

| address | value | used by |
|---|---|---|
| `0x80377DC8` | first word of the runtime vector | `fn_80c516e0` |
| `0x80377DCC` | second word of the runtime vector | `fn_80c516e0` |
| `0x80377DD0` | third word of the runtime vector | `fn_80c516e0` |
| `0x80C592B8` | `0.0` (all bits clear) | `fn_80c52670` |
| `0x80C6C148` | relocated pointer to `kameckRoute1` | `fn_80c51460` |
| `0x80C6C15C` | relocated pointer to `kameckRoute2` | `fn_80c51460` |
| `0x80C68998` | class vtable installed by `fn_80c42c80` | `fn_80c42c80` |
| `0x80C68FE8` | `+0xE74` interface vtable | `fn_80c42c80` |

## Unknowns

- **Dropped arguments.** `0x801628C0` results are passed to the translated
  constructors by ABI (`r3`) but the export prints the calls without arguments.
  The same applies to the base constructor `0x800A88A0` in `fn_80c42c80` and
  to `0x80103AC0` in `fn_80c516e0`.
- **Unmodelled calls.** `0x800AC0B0`, `0x800AC7A0`, `0x800AE420` and
  `0x800AE500` are printed with no arguments. Their roles are inferred from
  the surrounding teardown/reset code, not from a translated body.
- **Pair split.** `0x802DCD6C` returns a 64-bit register pair. In
  `fn_80c56ec0` the low word is treated as the route table (fields `+0x0C`,
  `+0x10`) and the high word as the actor (`+0x194`, `+0x1B4`). That split is
  inferred from field use, not from a second caller.
- **Route roles.** `0x801002C0`, `0x800F9970` and `0x800F57A0` are collision
  helpers by call shape only. The five-byte row stride and the `0x16518` world
  stride are literal.
- **Virtual receivers.** `0x80C08D80`, `0x80C2C8A0` and `0x80C52670` print
  argument-less virtual calls. The receiver is supplied from the vtable
  location (`+0x60`, `+0x548`, `+4`) and is marked `medium`.
- **Partner selection.** `0x80162D00` remains `call0`, exactly as in the
  wave-21 `0x80C191A0`, because the export proves only the `+0x770` gate.
- **State entry.** Ghidra could not recover the jump table at
  `0x80C38FF8`; the six argument registers for slot `+0xA0` are nevertheless
  printed explicitly and are preserved.

## Remaining candidates

These are the highest-value bodies still outside the translated set:

| address | size | why deferred |
|---|---:|---|
| `0x80C21BC0` | 1,056 | large castle AI; Ghidra output is dominated by GQR/paired-single artifacts |
| `0x80C54050` | 864 | aim/collision position update; same GQR artifact problem |
| `0x80C51520` | 444 | route population; the `0x802DCD74` pair and dropped arguments are unresolved |
| `0x80C43B20` | 372 | vertical aim clamp; GQR artifact problem |
| `0x80C2EBE0` | 184 | six-slot effect sweep; GQR artifact problem |
| `0x80C30940` | 184 | six-slot effect sweep; GQR artifact problem |
| `0x80C44110` | 116 | aim wrapper for `0x80C43B20`; GQR artifact problem |
| 27 bodies | 224 / 136 | 14 remaining `+0x30` chain resolvers and 13 name comparators |
| 40+ bodies | 48-72 | remaining `0x802DCBFC` destructor adapters and small callbacks |

The two GQR-heavy gameplay functions (`0x80C21BC0`, `0x80C54050`) need a
disassembly-level pass rather than a direct C translation; the C export loses
the paired-single loads and stores.

## Verification

- `cargo check` exits 0 and reports no diagnostic for `src/game/bosses.rs`
  (the remaining crate warnings belong to other lanes).
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- All 25 evidence files were read in full; every `# Original` block names the
  file and the size from `functions.tsv`.
- The 25 rows in `ledger/agents/wave22-bosses.csv` match the Rust functions
  one to one: 25 `translated`; 9 `high`, 16 `medium`.
- No files outside `src/game/bosses.rs`, this document and the wave-22 ledger
  were edited.
