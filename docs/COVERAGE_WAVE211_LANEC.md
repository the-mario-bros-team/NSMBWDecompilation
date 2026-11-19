# Coverage - Wave 211, lane C (`game::player`, the `0x80064600..0x80064A30` cluster)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 10 | 636 | 3 | 7 | 0 |

Status split: 10 `translated`, 0 `partial`, 0 deferred.

Body block: `work/wave211/laneC_block.rs`. Ledger rows:
`ledger/agents/wave211-laneC.csv`.

## How to read this file

* **Address** - virtual entry point in the DOL image, not a file offset.
* **Name** - the Ghidra name (`FUN_<address>`). No symbol table survives, so the
  Rust body keeps the `fn_<address>` name; every new constant in the block is
  suffixed `_C`.
* **Size** - the `functions.tsv` value, which is also the real extent. Every one
  of the ten was re-derived from the prologue and the closing `blr`/`bcctr`;
  none is a 16/20-byte prologue artefact.
* **Confidence** - strength of the evidence, not code quality: `high` = every
  address, offset and branch is literal; `medium` = a callee's meaning, a
  return type or a field role had to be inferred from call sites.
* All ten bodies use the module's documented bases, `r13 = 0x8042F680` and
  `r2 = 0x80433080`.

## What this pass covers

### The spawn tag byte

- **`0x80064600` (8 B, high).** The whole body is
  `stb r3,0xffffa690(r13)` + `blr`, i.e. `0x80429D10 = tag`. The only reader in
  the image is `0x80064274` in the actor constructor, which copies it into the
  new actor's `+0x38F`, `+0x1E8`, `+0x2D8` and `+0x30E` bytes, so the byte acts
  as a "default tag latched for the next actor" (`// inferred:`).
- **`0x80064610` (104 B, medium).** Latch the tag from the fifth argument, then
  `0x8006CA80(r3..r6)`. This is the spawn entry the whole image uses (48 `bl`
  sites, including [`fn_8005e9a0`](crate::game::player::fn_8005e9a0)). The
  export types both functions `void`; the returned word is the callee's
  surviving `r3`, which every caller tests.
- **`0x80064680` (96 B, medium).** The five-argument twin: tag from the sixth
  argument, then `0x8006CBA0(r3..r7)`. Ghidra prints the prologue as
  `FUN_802dcd74()` returning a 64-bit value; that call is the
  `__savegprlr_27` save stub, so its "result" is just the incoming `r3`/`r4`
  and the real argument list comes from the disassembly.

### The object-kind setter

- **`0x800646E0` (64 B, high).** Writes the `+0x38C` kind byte, then calls the
  object's `+0x4C` virtual slot with the object as its only argument and stores
  the result at `+0x68`. The sibling code in `src/game/display.rs` and
  `src/game/dsys.rs` reads `+0x38C` as a kind tag (`1` = player view, `3` =
  `dsys` object); the constructor `0x80064110` initialises it to `0` through
  this same setter.

### The two installed policies

- **`0x80064720` (24 B, medium)** and **`0x80064A10` (24 B, medium).** The two
  8-instruction leaves that copy `0x802EF328[mode]` / `0x802EF334[mode]` into the
  SDA words `0x80429CFC` / `0x80429D00`. Both index with `rlwinm r0,r3,2,0,29`,
  i.e. `(mode & 0x3FFFFFFF) * 4`, with no bounds check. The tables, read from the
  image:

  | mode | find (`0x802EF328`) | compare (`0x802EF334`) |
  |---|---|---|
  | `0` | `0x800647A0` | `0x80064A40` (`f1 < f2`) |
  | `1` | `0x80064880` | `0x80064A50` |
  | `2` | `0x80064880` | `0x80064A50` |

  `0x80064B10` installs a mode's pair (plus the same-shape `0x80101BC0`
  installer at `0x8042A228`) in one call.
- **`0x80064790` (12 B, high)** and **`0x80064A30` (12 B, medium).** Two pure
  tail jumps: `lwz r12,<SDA word>` / `mtspr CTR` / `bcctr`. The first forwards
  `r3` = two-float output block and `r4` = two-float origin (confirmed by both
  call sites, `0x80096240` and `0x800A4D80`, which test the returned actor
  pointer). The second forwards `f1`/`f2` untouched - the 12-byte length rules
  out any spill, and the mode-`0` callee `0x80064A40` is
  `fcmpu f1,f2` + `mfcr` + `rlwinm` (`1` when `f1 < f2`). That FPR-argument
  relay is modelled with the same `transmute` idiom the audio module already
  uses for `f64` SDK calls.
- **`0x80064740` (80 B, medium).** Probes the object's centre with the shared
  `0x8006CED0` (already in the module as `FN_8006CED0_C`) and runs the installed
  finder with the probe's **X/Y only** as the origin: the disassembly reloads
  the first two words through `lfs`/`stfs` into an 8-byte block, dropping Z.
- **`0x800647A0` (212 B, medium).** The mode-`0` finder itself: for each slot
  `0..4` that is both resolvable through `0x8005F900` and set in the per-slot
  presence mask `0x80429CA8`, it forms
  `(actor[+0xAC] + actor[+0xD0], actor[+0xB0] + actor[+0xD4])`, subtracts the
  origin and keeps the winner while `dx*dx + dy*dy` is strictly below the
  running best, which is seeded from the BSS float `0x8042A5C8`. Everything is
  `f32` (`fadds`/`fmuls`); Ghidra's `double` locals and its two `ldexpf` blocks
  are artefacts of the save stub and of a `GQR0` reading. The export's
  "removing unreachable block at `0x80064850`" warning is a mis-read of the loop
  exit: the `blr` at `0x80064870` closes the full 212 bytes.

## Verification

Scratch crate: `src/`, `Cargo.toml` and `Cargo.lock` copied to a temp directory,
this lane's block appended to the copy's `src/game/player.rs`,
`cargo check --lib` exits `0` with no diagnostics pointing at `player.rs`
(the 23 remaining warnings are the pre-existing function-item cast warnings in
`src/ui/menu.rs` and friends).
