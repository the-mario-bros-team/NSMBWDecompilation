# Coverage - Wave 212, lane C (`game::player`, the `0x80067270..0x80067A80` teardown cluster)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 10 | 1120 | 5 | 5 | 0 |

Status split: 10 `translated`, 0 `partial`, 0 deferred.

Body block: `work/wave212/laneC_block.rs`. Ledger rows:
`ledger/agents/wave212-laneC.csv`.

## How to read this file

* **Address** - virtual entry point in the DOL image, not a file offset.
* **Name** - the Ghidra name (`FUN_<address>`). No symbol table survives, so the
  Rust body keeps the `fn_<address>` name; every new constant in the block is
  prefixed `W212C_` (this lane's letter is `C`).
* **Size** - the `functions.tsv` value, which is also the real extent. All ten
  were re-derived from the prologue and the closing `blr`; none is a 16/20-byte
  prologue artefact.
* **Confidence** - strength of the evidence, not code quality: `high` = every
  address, offset and branch is literal; `medium` = one register-resident
  argument cannot be expressed in the Rust model.
* All ten are main.dol bodies; the descriptor block they touch lives in `.data`
  at `0x8030EE90..0x8030EF98`, the two descriptors in BSS at
  `0x80355E74` / `0x80355EB4`.

## What this pass covers

### The three-member deleting destructor

- **`0x80067270` (168 B, high).** `if (this)` releases the member at `+0x3EC`
  through `0x8015F9F0(member, 0)`, then the member at `+0x3B0`, then runs the
  family base destructor `0x800642B0(this, 0)` and, when the signed second
  argument is `> 0`, `0x80162920(this)`. The GCC/CodeWarrior `addic.` chain is a
  null test on each **adjusted** pointer, which is why Ghidra prints the
  negative constants `-0x394`, `-0x3D4`, `-0x3EC`, `-0x398`; the block keeps the
  `wrapping_add` form so the control flow stays visible and cannot be UB-folded.
  The body returns `this` unchanged in every path.

### The two identical one-line destructors

- **`0x80067510` (64 B, high)** and **`0x80067550` (64 B, high).** Byte-identical:
  if the object and the signed flag are both non-zero, call the shared
  deallocator `0x802B9280(this)`; return `this`. The callee receives the
  untouched incoming `r3`, so the free target is the object itself, not a
  member.

### The five `0x802DCBFC` member-descriptor forwarders

- **`0x80067740` (48 B, medium), `0x80067770` (48 B, medium),
  `0x800677A0` (48 B, medium), `0x800677D0` (48 B, medium),
  `0x80067800` (48 B, medium).** All five are the same twelve instructions:

  | address | descriptor (`r12`) | twin |
  |---|---|---|
  | `0x80067740` | `first + 0x18` | `0x800677D0` |
  | `0x80067770` | `first + 0x0C` | `0x80067800` |
  | `0x800677A0` | `first + 0x24` | - |

  They shuffle `r3`/`r4` (`r5 = first`, `r3 = second`) and set
  `r12 = first + offset`, then call `0x802DCBFC`. That helper reads the
  three-word descriptor at `r12` (`+0` this-adjust, `+4` slot index, `+8`
  vtable offset), adds the adjust to `r3`, and tail-jumps either through
  `*(*(r3 + offset) + slot)` or directly through `offset` when `slot` is zero.
  In other words each thunk invokes the member function described by the
  descriptor at `first + offset`, on `second`, with `first` as its argument.

  Unlike the wave-106 batch, the descriptor **offset is now decoded** and
  recorded per thunk; the only unmodelled part is `r12` itself, because
  `platform::call3` (like every other `callN` in `platform`) forwards `r3..rN`
  only. The block therefore keeps the established
  `call3(0x802D_CBFC, second, second, first)` shape with an explicit
  `// partial:` comment naming the missing descriptor. This is the reason the
  five sit at `medium`, not `high`.

### The static registration function

- **`0x80067830` (492 B, high).** Prologue `stwu r1,-0x70` plus the
  `0x802DCD74` save stub, epilogue through `0x802DCDC0`; no incoming argument
  is read, so the Rust signature takes none. For each of two descriptors the
  body snapshots nine words, calls a factory getter, initialises the descriptor
  header, writes the snapshot back in reversed row order (turning the two
  descending offset tables into ascending ones), installs the vtable and the
  factory result, and finally prepends a registry node with `0x802DC780`.

  | | descriptor 1 | descriptor 2 |
  |---|---|---|
  | descriptor (BSS) | `0x80355E74` | `0x80355EB4` |
  | source words | `0x8030EE90` | `0x8030EEB4` |
  | tag string | `0x8030EED8` `"dActorState_c::StateID_Gegneric"` | `0x8030EEF8` `"dActorMultiState_c::StateID_Gegneric"` |
  | vtable written at `+0` | `0x8030EF8C` | `0x8030EF24` |
  | factory (`+0x30`) | `0x80066FB0` | `0x80067260` |
  | registered callback | `0x80067A80` | `0x80067B40` |
  | registry node argument | `0x80355E68` | `0x80355EA8` |

  Both factories are three-instruction `lis`/`addi` getters returning the same
  BSS word `0x80377A04`, and both vtables carry the registered callback in
  their `+0x08` slot. The BSS registry head is `[r13 - 0x46A8]`, as
  `0x802DC780` pushes `{next, callback, argument}`.

### The release-then-free destructor

- **`0x80067A80` (92 B, high).** `0x8015F800(this, 0)` releases the state
  descriptor, then `0x802B9280(this)` runs when the signed flag is `> 0`; the
  body returns `this`. The disassembly contains **two** `beq`s testing the same
  `cmpi cr0,r3,0` result (`0x80067AA0` and `0x80067AA4`), the second of which is
  unreachable-skip dead code; the Rust body models the single effective test and
  the prose records the duplicate. The neighbouring `0x80067AE0` has the same
  shape with only one branch, which supports that reading.

## Verification

Scratch crate: `src/`, `Cargo.toml` and `Cargo.lock` copied to
`C:\Users\WitherULTRA7\AppData\Local\Temp\opencode\w212c`, this lane's block
appended to the copy's `src/game/player.rs`, `cargo check --lib` exits `0`
(`Finished dev profile`) with the same 23 pre-existing warnings as the base tree
and no diagnostic pointing at `player.rs`.
