# Coverage - Wave 214, lane F (`game::player`, the `0x8006F600..0x80070124` cell-step helpers)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 8 | 2796 | 3 | 5 | 0 |

Status split: 8 `translated`, 0 `partial`, 0 deferred.

Body block: `work/wave214/laneF_block.rs`. Ledger rows:
`ledger/agents/wave214-laneF.csv`.

Scratch verification: the block was appended to a copy of `src/game/player.rs`
in a scratch crate (with `Cargo.toml`/`Cargo.lock`) and
`cargo check --lib` exits `0`; the block adds no warnings.

## How to read this file

* **Address** - virtual entry point in the DOL image, not a file offset.
* **Name** - the Ghidra name of the body, which the Rust function keeps as
  `fn_<address>`; every new constant in the block carries the `_F` lane suffix.
* **Size** - the `functions.tsv` value. All eight were re-derived from the
  prologue and the closing instruction and match exactly, so there is no
  prologue or alignment artefact in this run. `0x8006FA40` ends in a tail
  branch (`b 0x8006FAA0` at `0x8006FA90`), and the `blr` at `0x8006FA94` that
  follows it is dead padding.
* **Confidence** - strength of the evidence: `high` means every address, field
  offset, immediate and branch is literal in the disassembly. `medium` marks a
  body whose *shape* is fully literal but whose semantic role is inferred from
  its callers and the two table shapes (`fn_8006f750`, `fn_8006f7e0`,
  `fn_8006faa0`, `fn_8006fcf0`, `fn_8006fea0`).
* `r13 = 0x8042F680` and `r2 = 0x80433080` throughout, so the small-data
  floats resolve to `r2 - 0x7440` = `0x8042BC40` (`0.0f`),
  `r2 - 0x7438` = `0x8042BC48` (`0x43300000_80000000` = `2^52 + 2^31`, the
  signed int-to-double bias), `r2 - 0x7430` = `0x8042BC50` (`19.0`),
  `r2 - 0x7428` = `0x8042BC58` (`16.0f`), `r2 - 0x7418` = `0x8042BC68`
  (`1.0f`), `r2 - 0x740C` = `0x8042BC74` (`4096.0f`), `r2 - 0x7408` =
  `0x8042BC78` (`0.1f`), `r2 - 0x73D4` = `0x8042BCAC` (`0.0625f`) and
  `r2 - 0x73D0` = `0x8042BCB0` (the byte table `01 02 00 00`). Lane C's header
  documents the same `r2` base for this address band.

## What this pass covers

One coherent subsystem: the per-cell probes that `0x80070130` (already
translated as `crate::game::stage::fn_80070130`) leans on, plus the two step
tests the gameplay actor runs after a movement step.

| address | Rust | bytes | role |
|---|---|---:|---|
| `0x8006F600` | [`fn_8006f600`] | 324 | re-probe a moved point and run `0x8006D8A0`/`0x8006D9B0` |
| `0x8006F750` | [`fn_8006f750`] | 132 | snap the stored height down to a `1/8` or `1/16` cell edge |
| `0x8006F7E0` | [`fn_8006f7e0`] | 604 | align the point to the cell grid and latch the surface kind |
| `0x8006FA40` | [`fn_8006fa40`] | 84 | pick the direction from a height's sign, tail-call `0x8006FAA0` |
| `0x8006FAA0` | [`fn_8006faa0`] | 336 | apply one direction's state word and fold its flags into `+0x88` |
| `0x8006FBF0` | [`fn_8006fbf0`] | 244 | one-shot walk at a single cell offset |
| `0x8006FCF0` | [`fn_8006fcf0`] | 424 | march the offset across a cell and accumulate the results |
| `0x8006FEA0` | [`fn_8006fea0`] | 648 | scan from the actor toward the point in `16.0f` steps |

## Findings worth keeping

1. **The two "prologue" stubs are `__savegprlr` / `__restgprlr`.** The
   `bl 0x802DCD74` at `0x8006FD10` / `0x8006FEE0` and the `bl 0x802DCDC0` at
   `0x8006FE84` / `0x80070114` are the outlined register-save stubs this module
   already documents (e.g. at `0x80066120`). They are not emitted. They also
   explain why Ghidra types `0x8006FCF0` and `0x8006FEA0` as `void`: the trailing
   save/restore call swallowed the `r3` result that both callers actually read
   (`0x8006FAA0` does `mr r31,r3` right after the `bl`).
2. **`0x8006FAA0`'s `+0xE0` arm stores a literal `0x80`.** The raw stream at
   `0x8006FB94..0x8006FBA4` is `li r0,0x80` / `nor r4,r0,r0` /
   `rlwinm r0,r0,0,24,31` / `and r3,r0,r0` / `stb r0,0xE0(r30)`, so the store
   writes `0x80` and the following `and r0,r0,r4` at `0x8006FBAC` always yields
   `0` in that arm. The Ghidra export prints `e0 & ~(table | 0x80)` instead; the
   translation follows the raw store and says so in the doc block.
3. **`0x8006F7E0` reduces the x coordinate modulo `19.0`.** `bl 0x802E8614`
   (`fmod`-style thunk of `0x802E4E5C`) is called with the double at
   `0x8042BC50`; the `frsp` at `0x8006F944` keeps the single-precision result.
4. **`0x8006FEA0` takes exactly two floats.** Its prologue saves `f26..f31` but
   assigns `f26` (bias), `f27` (`16.0f`), `f28` (`0.1f`) and `f31` (`0.0f`)
   itself, so the earlier suspicion that it inherits extra argument registers
   through `0x80070130` is wrong: `fmr f29,f1` / `fmr f30,f2` are the only
   inputs.
5. **`0x8006FEA0` treats the two aligned words differently.** The x word is
   `rlwinm 0,16,27` followed by `extsh` (a signed halfword read), while the y
   word is `rlwinm 0,16,27` alone (an unsigned `0xFFF0` multiple). The block
   builder reproduces that asymmetry.

## Deferrals

None. Every body in this lane was translated in full; no `// partial:` markers
were needed.

[`fn_8006f600`]: ../src/game/player.rs
[`fn_8006f750`]: ../src/game/player.rs
[`fn_8006f7e0`]: ../src/game/player.rs
[`fn_8006fa40`]: ../src/game/player.rs
[`fn_8006faa0`]: ../src/game/player.rs
[`fn_8006fbf0`]: ../src/game/player.rs
[`fn_8006fcf0`]: ../src/game/player.rs
[`fn_8006fea0`]: ../src/game/player.rs
