# Coverage - Wave 204, lane B (`ui::hud`, `0x80024F60..0x80026A90`)

## How to read this file

This page records the Wave 204 lane B pass over `src/ui/hud.rs`: ten bodies of
`main.dol` in the single run `0x80024F60..0x80026A90` (1,420 bytes by the
verified extents). The block is staged in `work/wave204/laneB_block.rs` for the
merge pass; it is appended verbatim at the end of `src/ui/hud.rs`. One row per
body is mirrored in `ledger/agents/wave204-laneb.csv`.

Every address is a **runtime address** of `sys/main.dol`, the same address the
Ghidra exports in `work/ghidra/main-gb/export/` use, and each function doc
carries `# Original` (Ghidra name, address, byte count, evidence path) and
`# Confidence` plus an argument/return/side-effect/safety block.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/hud.rs` | `ui::hud` | 10 | 1,420 | 8 | 2 |

All ten rows are `translated`; none is `partial` or `stub`. Covered range:
`0x80024F60..0x80026A90`.

Raw words were decoded with `node $env:TEMP\opencode\ppc159.js work/dol/main.elf
<addr>`, and each body was followed from its entry to its terminal
`blr`/`bcctr`. Extents: 168, 508, 116, 64, 88, 92, 12, 12, 224, 136.

## What this pass covers

- **`0x80024F60` - pose refresh.** Re-derives the walk flag `+0x6E8` from the
  pose pair `+0x714`/`+0x348`, clears `+0x6F4`/`+0x6EC`, replays the animation
  chosen by `+0x6E8` through `0x80024F10` with the `r2 - 0x7D80` (zero) delta,
  re-enters `0x80024F40` on a flag mismatch, then runs the `+0x2D0` and `+0x2DC`
  virtual slots of the `+0x60` vtable.

- **`0x80025800` - four-phase arming state machine (508 bytes).** Fixed
  prologue (`0x80168080(+0x584)`, slot `+0x1C` of `+0x544`, `0x8006CDE0`,
  `0x800955F0`), then the walk flag is armed only while slot `+0x300` answers
  `0`. Phase `+0x360 == 1` compares the `+0x2C8` slot against the elapsed time
  and jumps to phase `2`; phase `2` compares the `+0x2CC` slot (also passed to
  `0x801690E0` in `f1`) and either re-arms (`+0x28C`), decrements `+0x6F0` until
  the `+0xD4` slot receives `0x80352BB4`, or rebuilds through `0x80024190` plus
  slot `+0x2E8`. Any other phase value falls through.

- **`0x80025F60` - vtable-assigning deleting destructor.** Writes the base
  vtable `0x80329958`, unlinks through `0x8016D4B0`, releases the base
  sub-object through `0x802D7B20(this, 0)`, then frees for a positive `mode`.
  The duplicated null test at `0x80025F82` is compiler-emitted and is
  reproduced.

- **`0x80026080`, `0x800260C0`, `0x80026850` - release triplets.** The same
  `if (this) { <release>; if (mode > 0) free; } return this;` skeleton with
  three different embedded destructors: none (`0x802B9280` only), `0x80168D80`
  (the `+0x544` player) and `0x8015F800` respectively. `0x80026850` carries an
  extra inner null test, which is why it is 92 bytes and not 88.

- **`0x800268B0`, `0x800268C0` - literal accessors.** `3C608035 38637BF4
  4E800020` and `3C608035 38637BB4 4E800020`.

- **`0x800269B0` - five-deep `+0x30` chain walk.** Five unrolled slot `+0xC`
  probes, then tail recursion on the last child.

- **`0x80026A90` - post-colon name comparison.** Two `0x802E1C40` searches for
  `':'`, one virtual `+0x20` name fetch, one `0x802E1AB4` compare.

## Corrections and recoveries

- **`0x80025800` size.** The manifest says 16 bytes; the entry is a full frame
  (`stwu r1,-0x20`, `stfd f31`, `psq_st f31`) ending in `bclr` at
  `0x800259F8`, i.e. 508 bytes (`0x80025800..0x800259FC`). Disassembly, the
  Ghidra export and the prologue agree, so the whole body is translated. This is
  the only size correction in the lane; the other nine match exactly
  (168/116/64/88/92/12/12/224/136).

- **The two 12-byte bodies are not `0x80377A04`.** The lane brief predicted the
  `lis/addi/blr` accessors of `0x80377A04`; they are accessors instead, but of
  `0x80357BF4` and `0x80357BB4`. The `0x80377A04` accessor of this band is
  `0x80026BB0` (`lis r3,0x8037` / `addi r3,r3,0x7A04`), which belongs to lane C.

- **Byte-identical twins.** `0x80026850` = `0x8001D710` = `0x8001F380` (all
  translated), `0x800269B0` = `0x8001D850` = `0x8001F4C0` (all translated), and
  `0x80026A90` = `0x8001D930` (translated). `0x80025F60` = `0x80032A30` and
  `0x80026080` = `0x80024C20` (not translated yet). One out-of-line copy per
  translation unit is normal for this compiler.

- **Asymmetric separator handling in `0x80026A90`.** The needle's post-colon
  pointer is advanced only when `0x802E1C40` found a `':'`; the candidate's is
  advanced unconditionally (`addi r3,r3,1`, no null test), so a candidate
  without a separator compares from offset `1`. Reproduced literally.

- **Left-over `f1` argument.** `0x801690E0` takes its scalar in `f1`, and the
  call site at `0x80025928` never loads one: `f1` still holds the `+0x2CC` slot
  result from `0x80025920`. The reconstruction passes that value explicitly
  through a local `extern "C" fn(f32, u32)` helper, which is also what
  `0x80024F10` needs.

- **`r2 - 0x7D80`.** Four stores in `0x80025800` and one in `0x80024F60` reload
  this slot; the ELF word at `0x8042B300` is `0x00000000`, so it is a zero
  constant distinct from the existing `F32_SDA2_ZERO` (`r2 - 0x7ED8`). Added as
  `F32_SDA2_7D80`, not reusing the other slot.

- **Three constants repeat across lane blocks.** `F32_SDA2_7D80` is also
  declared by lane A, and `FN_80168D80` / `FN_80072BC0` by lane D, with
  byte-identical definitions (`SDA2_BASE - 0x7D80`, `0x8016_8D80`,
  `0x8007_2BC0`). The block stays self-contained so that it can be verified on
  its own; the merge pass drops the repeats the same way it dropped the
  wave-203 `FN_8016C890` duplicate.

## Verification

- Temp copy (`$env:TEMP\w204laneB`, `src/` + `Cargo.toml` + `Cargo.lock` only):
  block appended to `src/ui/hud.rs`, `cargo check --lib` **exit 0**, no
  warnings attributable to `src/ui/hud.rs`.
- Two compile errors were found and fixed during verification: the accessor
  constants had to be typed `u32` (as the existing `DAT_80377A04` is) because
  the functions return them directly.
- Ledger rows: 10, all `translated`, 8 `high` + 2 `medium`
  (`0x80025800` for the inferred phase roles, `0x80026A90` for the inferred
  string helpers).
