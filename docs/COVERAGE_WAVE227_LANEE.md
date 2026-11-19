# Wave 227 - Lane E: `cror` / `crxor` / `crand` and indexed float memory forms

Working dir: `C:\Users\WitherULTRA7\wii-decomp\nsmbw`.
Tool under test: `%TEMP%\opencode\ppc159.js` (scratch disassembler, not part of the
repo). No `src/` edits were made in this lane.

## 1. What was added

Both additions are decoder-table entries; no existing case was changed.

**Condition-register ops (primary opcode 19, XL-form).** The opcode-19 handler
already decoded `bclr` (XO 16), `bcctr` (XO 528) and `rfi` (XO 50) and otherwise
fell through to the synthetic `xl<XO>` mnemonic. It now also decodes the whole
XL condition-register group, which shares the same `BT,BA,BB` field slots as
`bclr`'s `BO,BI`:

| mnemonic | XO  | fields |
| -------- | --- | ------ |
| `crnor`  | 33  | `BT,BA,BB` = bits 21-25, 16-20, 11-15 |
| `crandc` | 129 | same |
| `crxor`  | 193 | same |
| `crnand` | 225 | same |
| `crand`  | 257 | same |
| `creqv`  | 289 | same |
| `crorc`  | 417 | same |
| `cror`   | 449 | same |

The task named `cror`, `crxor` and `crand`; the other five members of the group
were added at the same time because they are the same encoding and would
otherwise have kept printing `xl<XO>`.

**Indexed float memory (primary opcode 31, X-form).** XO entries added:

| mnemonic | XO  | printed as |
| -------- | --- | ---------- |
| `lfsx`   | 535 | `lfsx fRT,rA,rB` |
| `lfdx`   | 599 | `lfdx fRT,rA,rB` |
| `stfsx`  | 663 | `stfsx fRT,rA,rB` |
| `stfdx`  | 727 | `stfdx fRT,rA,rB` |

These print `f<n>` for the RT field (it is an FPR), matching how `lfs`/`stfd`
are already printed in the same file. They previously printed `X31_535` etc.

`node --check %TEMP%\opencode\ppc159.js` passes.

## 2. How many words in `work/dol/main.elf` this changes

Counted over the executable `PT_LOAD` segment only (`work/wave227/count227.js`,
763,464 words):

| | | | |
| --- | --- | --- | --- |
| `cror` 997 | `crxor` 774 | `creqv` 12 | `crand` 0 |
| `crnor` 0 | `crandc` 0 | `crnand` 0 | `crorc` 0 |
| `lfsx` 488 | `stfsx` 349 | `lfdx` 30 | `stfdx` 1 |

`crand` (XO 257) does not occur in `main.elf` at all, so that entry is
forward-looking only. `stfdx` occurs exactly once, at `0x802E7104`.

## 3. Verification of the encoding

Two independent checks were run (`work/wave227/../verify227.js` in the temp
dir): each word was re-encoded from ISA field positions by a formula written
separately from the decoder, and compared with the word actually in the ELF.
All six matched the re-encoded word exactly.

Independent semantic corroboration, per site:

* `crxor 6,6,6` encodes to `0x4CC63182`, the documented compiler idiom for
  clearing the varargs float-argument flag in CR6.
* `lfsx`/`stfsx` sites are preceded by `rlwinm ...,2,0,29` (index scaled by 4 =
  `f32` element size); `lfdx`/`stfdx` sites are preceded by `rlwinm ...,3,0,28`
  (index scaled by 8 = `f64` element size). The matched element sizes confirm
  the load/store widths and that RT is being used as an FPR.
* `lfsx` is already named in 29 committed doc-block lines from earlier waves (e.g.
  `src/ui/hud.rs:17054`, `src/game/bases.rs:255800`), so the new decode agrees
  with prose that was written before this tool caught up.

## 4. The five re-disassembled sites

| # | address | word | new output | next instruction (context) |
| - | ------- | ---- | ---------- | -------------------------- |
| 1 | `0x8000D8C8` | `4C411382` | `cror 2,1,2` | after `fcmpo cr0,f0,f2`, before `bc 4,2` |
| 2 | `0x80010BE8` | `4CC63182` | `crxor 6,6,6` | before `bl 0x800C9A60` (5 int args) |
| 3 | `0x8001B824` | `7C44042E` | `lfsx f2,r4,r0` | after `rlwinm r4,r4,2,0,29` / `add r0,r5,r0` |
| 4 | `0x8022812C` | `7C1844AE` | `lfdx f0,r24,r8` | after `rlwinm r8,r8,3,0,28` |
| 5 | `0x802E7104` | `7C4405AE` | `stfdx f2,r4,r0` | inside the 8-way unrolled `fq[]` store |
| 6 | `0x8000D0E8` | `7C26552E` | `stfsx f1,r6,r10` | lane-base + `mulli` index (extra site) |

(1) `cror 2,1,2` sets `cr0.EQ := cr0.GT | cr0.EQ`, so the following `bc 4,2`
branches when the comparison was `LT`; the Ghidra export
(`work/ghidra/main-gb/export/8000d8a0_FUN_8000d8a0.c`) independently reads the
same first test as `if (local_8 < fVar1)`, i.e. `dx < eps`. Decode and export
agree on the operand order, including the fact that this X-form encodes the
first compared FPR in the bits-16-20 slot.

(3) `r5 = 0x803525E8` (built by `lis 0x8035` / `addi 9704`), `r0 = r5 + row*28`,
`r4 = column*4`; `EA = r4 + r0 = 0x803525E8 + (row*7 + column)*4`, which is
exactly `TABLE_WIPE_CIRCLE_FOLLOW + (row * 7 + column) * 4` in
`src/ui/hud.rs:10139`.

(4) `read_f64(ADPCM_SCALE_TABLE + idx * 8)` in `src/runtime/bt.rs:38734`.

As a further spot check, `0x800295C0` now prints `lfsx f1,r5,r0` with
`r5 = 0x802EE4F0` and `r0 = index*4`, matching the pre-existing prose in
`src/ui/hud.rs:17054` ("`TABLE[index]` loaded with `lfsx` ... the table is
`f32`").

## 5. Are any committed bodies' recorded semantics affected?

**No.** Six of the affected words sit inside translated bodies, and in every
case the recorded Rust body and doc prose already describe the behaviour the
newly named instruction implies:

| site | owning body | file | recorded semantics vs. new decode |
| ---- | ----------- | ---- | --------------------------------- |
| `0x8000D8C8` | `vec2_differs_from` | `src/render/geometry.rs:552` | doc already says "uses `fcmpo` + `cror` ... a NaN input takes the differs branch" (lines 549-551); Rust `!(dx < eps && -eps < dx && ...)` matches |
| `0x80010BE8` | `fn_80010b50` | `src/ui/hud.rs:8915` | `crxor 6,6,6` is the varargs CR6 clear before `call5(FN_800C9A60, ..., 4, 0x3D, 0)`; no float args, so no body change |
| `0x8001B824` | `fn_8001b620` | `src/ui/hud.rs:10139` | Rust already indexes `TABLE_WIPE_CIRCLE_FOLLOW` by `(row*7+column)*4` as `f32` |
| `0x8022812C` | `fn_80227ef0` | `src/runtime/bt.rs:38640` | Rust already reads `read_f64(ADPCM_SCALE_TABLE + idx*8)` |
| `0x802E7104` | `fn_802e624c` | `src/engine/main_loop.rs:14221` | store into the local `fq[]`/`f[]` `f64` arrays (`stfdx`, index*8) |
| `0x8000D0E8` | `store_four_lane_vertices` | `src/render/geometry.rs:637` | doc already describes the four `stfs` groups / lane base + 12-byte stride |

Two committed doc blocks cite the *fallback* mnemonics as raw evidence. Both
are prose-only staleness, not semantic drift, and both were already correct
about the meaning:

* `src/game/player.rs:46736-46737` - "the `fcmpo`/`cror 2,1,2` pair ... is the
  compiler's ordered `>=` fold (the raw disassembler prints `cror` as
  `xl449`)". The parenthetical is now obsolete (the mnemonic prints natively),
  but the described semantics are unchanged and correct.
* `src/game/bases.rs:269395` - "`xl449` folds each `fcmpo` pair against
  `0x807C9A70` (`0.0`) and `0x807C9B20` (`40.0`)" in the synthetic-base REL
  body `fn_8068c940`. Same: the name is stale, the meaning is not.

No body claims an instruction is unidentified in a way that this change
resolves, and no body's Rust contradicts the new decode. Because no `src/` edit
was made, nothing was re-verified with `cargo check` / `doc-lint` beyond
confirming that these are the only two fallback-name mentions for this
instruction set.

## 6. Notes for other lanes

* `fcmpo cr0,f0,f2` prints FRA then FRB where FRA is the bits-16-20 field. The
  commit-group sites that fold `cror` into ordered comparisons
  (`0x8000D8C8`, `0x8006D128`) both compare on the `LT` bit after
  `cror BT=EQ,BA=GT,BB=EQ`, so **unordered (NaN) yields neither GT nor EQ and
  therefore takes the `LT` path**. That is the divergence lane F is auditing;
  the two committed bodies above already document the NaN behaviour as
  intentional and match it in Rust.
* `crand` does not occur in `main.elf`; the REL images were not rescanned for it
  (they are not ELF, and this tool reads ELF program headers), so the "no
  occurrence" result is `main.elf`-only.
