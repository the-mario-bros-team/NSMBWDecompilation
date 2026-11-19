# Coverage: wave 221 lane D (`resource::archive`, `0x802C8C30..0x802CE620`)

## How to read this file

This document covers lane D of wave 221. All ten bodies live in
`src/resource/archive.rs`; no source file was edited by this pass. The bodies
are staged in `work/wave221/laneD_block.rs` for the merge, and the ten rows are
in `ledger/agents/wave221-laneD.csv`.

Every row describes one Ghidra entry point:

- **address** - virtual address of the entry in `main.dol` (image base
  `0x80004000`);
- **name** - the Rust function appended to `src/resource/archive.rs`;
- **role** - one plain sentence;
- **status** - `translated` means the whole recorded control flow is present,
  with calls outside the batch left as absolute-address calls;
- **confidence** - `high` when every load, store, branch and argument was read
  directly from the disassembly; `medium` when the role of one or more callee
  arguments is inferred;
- **bytes** - the `functions.tsv` size (see the size note below);
- **evidence** - the Ghidra C file under `work/ghidra/main-gb/export/`.

## Summary

| Metric | Count |
|---|---:|
| Functions translated in this batch | 10 |
| Bytes of `functions.tsv` size | 2,436 |
| `translated` | 10 |
| `partial` | 0 |
| `stub` | 0 |
| Confidence `high` | 9 |
| Confidence `medium` | 1 |
| Deferrals | 0 |

## Function table

| address | Rust function | role | status | conf. | bytes | evidence |
|---|---|---|---:|---:|---:|---|
| `0x802C8C30` | [`fn_802c8c30`] | pack one lookup record's colour word and data pointer into a 12-byte slot | translated | high | 212 | `802c8c30_FUN_802c8c30.c` |
| `0x802C9A10` | [`fn_802c9a10`] | return the static record at `r13 - 0x5B20` | translated | high | 8 | `802c9a10_FUN_802c9a10.c` |
| `0x802CA860` | [`fn_802ca860`] | copy three floats to `+0x14..+0x1C` and tail-branch into the vector rebuild `0x802CA780` | translated | high | 28 | `802ca860_FUN_802ca860.c` |
| `0x802CB4B0` | [`fn_802cb4b0`] | program the GX state of the single text channel from one caller word | translated | high | 316 | `802cb4b0_FUN_802cb4b0.c` |
| `0x802CB7B0` | [`fn_802cb7b0`] | program the GX state of the two text channels from two constant tables | translated | high | 384 | `802cb7b0_FUN_802cb7b0.c` |
| `0x802CB930` | [`fn_802cb930`] | build the fourteen glyph blocks and the one variant block, then map the size to the mode byte | translated | medium | 600 | `802cb930_FUN_802cb930.c` |
| `0x802CC9F0` | [`fn_802cc9f0`] | flush a pending pane colour and re-issue the pane draw | translated | high | 180 | `802cc9f0_FUN_802cc9f0.c` |
| `0x802CDD00` | [`fn_802cdd00`] | construct the 16-byte descriptor from an object through four interface probes | translated | high | 456 | `802cdd00_FUN_802cdd00.c` |
| `0x802CDED0` | [`fn_802cded0`] | descriptor count/format query | translated | high | 108 | `802cded0_FUN_802cded0.c` |
| `0x802CE620` | [`fn_802ce620`] | descriptor glyph-record lookup by index | translated | high | 144 | `802ce620_FUN_802ce620.c` |

## Size note (prologue + `blr`)

Each extent was checked against the prologue and the closing control transfer,
not just against the `functions.tsv` column:

| address | `functions.tsv` | real extent | closing transfer | note |
|---|---:|---|---|---|
| `0x802C8C30` | 212 | 0xD4 | `blr` at `0x802C8D00` | tsv is `blr + 4` |
| `0x802C9A10` | 8 | 8 | `blr` at `0x802C9A14` | exact (`addi` + `blr`) |
| `0x802CA860` | 28 | 0x1C | `b 0x802CA780` at `0x802CA878` | tail branch, so no local `blr`; exact |
| `0x802CB4B0` | 316 | 0x13C | `blr` at `0x802CB5E8` | tsv is `blr + 4` |
| `0x802CB7B0` | 384 | 0x180 | `blr` at `0x802CB92C` | tsv is `blr + 4` |
| `0x802CB930` | 600 | 0x258 | `blr` at `0x802CBB84` | tsv is `blr + 4` |
| `0x802CC9F0` | 180 | 0xB4 | `blr` at `0x802CCAA0` | tsv is `blr + 4` |
| `0x802CDD00` | 456 | 0x1C8 | `blr` at `0x802CDEC4` | tsv is `blr + 4` |
| `0x802CDED0` | 108 | 0x6C | `blr` at `0x802CDF38` | tsv is `blr + 4` |
| `0x802CE620` | 144 | 0x90 | `blr` at `0x802CE6AC` | tsv is `blr + 4` |

No row in this batch is a 16/20-byte prologue artefact: the smallest body
(`0x802C9A10`) is a genuine two-instruction accessor and every other row's
closing transfer lands exactly at `start + tsv - 4` (or, for the tail branch at
`0x802CA860`, at `start + tsv - 4` as well). The one-word pads that follow the
`blr`s belong to the next aligned entry.

## Notes that a reader should carry into the merge

1. **The `0x802DCD68` "64-bit return" is a register-save stub.**
   `0x802CB930` opens with `addi r11,r1,0x30` / `bl 0x802DCD68` and closes with
   `bl 0x802DCDB4`. Those two addresses are the CodeWarrior `stw`/`lwz` spills
   of `r24..r31` into `r11 - 0x20..-0x04`; they write nothing else. Ghidra
   therefore renders the prologue as `undefined6 uVar8 = FUN_802dcd68()` and
   reads a "return value" that is in fact the incoming `r3`/`r4`. The real
   signature is `(value, heap)`: `sth r3,-0x5B18(r13)` keeps the low half of
   `r3`, and `cmpi cr0,4,r4,0` / `lwz r31,-0x4870(r13)` is the heap default.
   Both callers confirm it (`0x801645F0` passes `(0x20, uVar1)`). The
   translation keeps the two arguments and omits the spill/restore pair, which
   cannot change behaviour.
2. **`0x802C8C30` forwards `f1` to `0x80241DE0`.** Nothing between the entry
   and the `bl 0x80241DE0` writes an FPR, and the call site in `0x802C8840`
   loads `fmr f1,f31` immediately before the call, so the frame time reaches the
   texture/LOD picker. The earlier comment on `FN_802C8C30_C` ("this body never
   reads `f1`") is correct about the body's own arithmetic but the value is not
   dead; the translated body passes it through.
3. **`0x802CDED0` kind-2 arm is not `x != 0`.** The shipped words are
   `neg r0,r3` / `or r0,r0,r3` / `rlwinm r3,r0,1,31,31`, i.e.
   `(x.wrapping_neg() | x) >> 31`. That differs from Ghidra's prose for inputs
   at or above `0x80000000`, so the exact bit expression is kept.
4. **`0x802CE620` takes a second argument.** Ghidra declares `FUN_802ce620(int
   *param_1)` but the body never writes `r4` and forwards it to `0x8023B800`
   (the `(index + 1) * 16` glyph-table lookup). Both callers in the export pass
   one (`0x802CD1F0`: `*puVar5`, `0x802CE410`: `uVar7`), so `index` is modelled
   as the second argument.
5. **`0x802CDD00` probe shape.** Four type descriptors (`0x80308FA8`,
   `0x80308FF0`, `0x80308FD8`, `0x80309000`) are each pushed into a stack word
   and handed to the object's `+0x08` slot as `(object, &word)`; a nonzero
   result selects the descriptor kind (5 by default, then 1, 0 or 2). This is
   the Metrowerks `dynamic_cast`-style probe; the export proves the slot, the
   stack word and the nonzero test but never names the interfaces, hence the
   `// inferred:` note on the word "interface".
6. **`0x802CB930` medium row.** The control flow, both tables
   (`0x802FD208` sheet words, `0x804246C8` block pointers), the `0x40` default
   size pair, the five overrides, the `0x100..0x04` mode ladder and the variant
   record are all literal. What is inferred is the *meaning* of the two `u16`
   arguments of `0x802C0E30` (modelled as `(first + 1)` and `(second - 1)`) and
   of the sheet words; the export shows the arithmetic but no field names.
7. **`0x802CB4B0` / `0x802CB7B0` GX naming.** Both bodies are pure state
   programming: sixteen and nineteen `0x801Cxxxx` calls with literal operands,
   plus `0x802C3020(index)` which selects the `index * 0x10` row of
   `0x802FD0A8`. The `0x801Cxxxx` band is the GX/GD layer that
   `crate::engine::dsys` already documents; this file only proves the operands.

## Deferrals

None. Every body in this batch was read out in full from the disassembly and
appended; the two `// partial:` markers in the block record places where the
Ghidra signature is narrower than the shipped calling convention (the kind-2
`neg`/`or` idiom and the forwarded `r4` of `0x802CE620`), not deferrals.

## Verification

The block was appended to a scratch copy of `src/`, `Cargo.toml` and
`Cargo.lock` (`%TEMP%\opencode\w221d`) and checked with `cargo check --lib`; it
exits 0 and the library emits no diagnostic pointing at any of the appended
lines.
