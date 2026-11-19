# Coverage: wave 222 lane D (`resource::archive`, `0x802CEF70..0x802CF1E0`)

## How to read this file

This document covers lane D of wave 222. All three bodies live in
`src/resource/archive.rs`; no source file was edited by this pass. The bodies
are staged in `work/wave222/laneD_block.rs` for the merge, and the three rows
are in `ledger/agents/wave222-laned.csv`.

Every row describes one Ghidra entry point:

- **address** - virtual address of the entry in `main.dol` (image base
  `0x80004000`);
- **name** - the Rust function appended to `src/resource/archive.rs`;
- **role** - one plain sentence;
- **status** - `translated` means the whole recorded control flow is present,
  with calls outside the batch left as absolute-address calls;
- **confidence** - `high` when every load/store/branch was read directly from
  the disassembly; `medium` when a load type, a hidden argument or an output's
  role is inferred;
- **bytes** - the `functions.tsv` size (see the size note below);
- **evidence** - the Ghidra C file under `work/ghidra/main-gb/export/`.

## Summary

| Metric | Count |
|---|---:|
| Functions translated in this batch | 3 |
| Bytes of `functions.tsv` size | 1,920 |
| `translated` | 3 |
| `partial` | 0 |
| `stub` | 0 |
| Confidence `high` | 2 |
| Confidence `medium` | 1 |
| Deferrals | 0 |

## Function table

| address | Rust function | role | status | conf. | bytes | evidence |
|---|---|---|---|---|---:|---:|---|
| `0x802CEF70` | [`fn_802cef70`] | construct the 0x78-byte arc-record draw object (base table, group count, four 16-byte records) | translated | high | 224 | `802cef70_FUN_802cef70.c` |
| `0x802CF050` | [`fn_802cf050`] | draw the object once: three state calls, then one slice pass per 8-entry chunk | translated | high | 400 | `802cf050_FUN_802cf050.c` |
| `0x802CF1E0` | [`fn_802cf1e0`] | emit up to eight ring slices of one record group plus its TEV colour and stages | translated | medium | 1,296 | `802cf1e0_FUN_802cf1e0.c` |

## Size note (prologue + `blr`)

Each extent was checked against the prologue and the closing `blr`, not just
against the `functions.tsv` column:

| address | `functions.tsv` | ending | `end + 4` | note |
|---|---:|---|---|---|
| `0x802CEF70` | 224 | `blr` at `0x802CF04C` | `0x802CF050` | 220 bytes of body plus one pad word; the next function starts at `0x802CF050`, so the tsv value is the full slot |
| `0x802CF050` | 400 | `blr` at `0x802CF1DC` | `0x802CF1E0` | 396 bytes of body plus one pad word |
| `0x802CF1E0` | 1,296 | `blr` at `0x802CF6EC` | `0x802CF6F0` | 1,292 bytes of body plus one pad word; the next function is `0x802CF6F0` |

None of the three is a prologue artefact: the recorded size is the full slot,
and the four-byte tail in each is alignment padding, not code that Ghidra
dropped.

## Notes that a reader should carry into the merge

1. **`_savegpr`/`_restgpr` in `0x802CF1E0`.** The body opens with
   `bl 0x802DCD68` (after `addi r11,r1,224`) and closes with
   `bl 0x802DCDB4`. These are `__savegprlr_24` and `__restgprlr_24`, i.e. the
   compiler's callee-save fragments; the export renders the first one as a call
   returning a `u64` and then splits that value into `r3`/`r4`, which makes the
   signature look like `(undefined8, double, ...)`. The disassembly shows the
   real incoming arguments - `r3` = object, `r4` = record index, `f1`/`f2` =
   the two scales, `r5` = chunk - and the Rust body follows the disassembly.
   The same artefact is already recorded for the wave-221 lane in this file.
2. **Paired-single traffic.** Every `psq_l`/`psq_st` in these three bodies uses
   `W = 0, I = 0`, i.e. a raw float pair. The `unaff_GQR0` / `ldexpf` cloud in
   the exports is Ghidra failing to model `I = 0`, not a quantised access; the
   prologues of `0x802CF050` and `0x802CF1E0` only save `f28..f31`/`f22..f31`.
3. **The `2^52` ladders are casts.** Three different idioms appear and all of
   them are MWCC float conversions, so the bodies write the casts directly:
   - `stw 0x43300000` + `stw <u16>` + `lfd` + `fsubs` against
     `0x8042F500` (`2^52`) is `u16 -> f32` (the two divisors of `0x802CF050`
     and the bit count of `0x802CF1E0`);
   - `xoris r0,r25,0x8000` + the same ladder against `0x8042F518`
     (`2^52 + 2^31`) is `s32 -> f32` (the running bit index at
     `0x802CF310`);
   - `fctiwz` + `stfd` + `lwz` + `stb` is the `f32 -> u8` colour store,
     always preceded by an `fcmpu` against `255.0` and an `fmr`, i.e. a
     **ceiling-only** clamp: a negative product keeps its (truncated toward
     zero) low byte and a NaN becomes `255`.
4. **`r2` constants resolved.** `SDA2_BASE` is `0x80433080`, so the slots this
   batch reads hold `r2-0x3B90` = `0xD7D7D7D7`, `r2-0x3B8C` = `1.0`,
   `r2-0x3B88` = `0.0`, `r2-0x3B80` = `2^52`, `r2-0x3B78` = `π`,
   `r2-0x3B74` = `40.7436638` (`256 / 2π`), `r2-0x3B70` = `2.0`,
   `r2-0x3B6C` = `255.0`, `r2-0x3B68` = `2^52 + 2^31`. The bodies read them
   from the SDA2 band rather than baking in the values, matching the rest of
   the module.
5. **`0x802CF050`'s chunk bound is `srawi` + `addze`.** The export writes it as
   `((int)uVar1 >> 3) + (uint)((int)uVar1 < 0 && (uVar1 & 7) != 0)`, which is
   the carry of the arithmetic shift. A record bit count of `0` therefore
   yields a bound of `0` (not `0x1FFFFFFF`), the loop still runs once, and the
   slice count passed on is `0`. Both details are reproduced.
6. **`0x802CF050`'s virtual calls.** Slot `+0x14` is called once with the two
   incoming scales still live in `f1`/`f2`; slot `+0x18` is called once per
   chunk after `0x802CF1E0` with `f1 = +0x14`, `f2 = +0x18`,
   `f3 = x * +0x1C`, `f4 = y * +0x20`. The table at `+0x28` is
   `0x80350460`, whose `+0x14`/`+0x18` words are `0x802CE8F0` and
   `0x802CE860`; neither is in this lane. `r4`/`r5` are not re-set before the
   first call, so they carry the caller's leftovers and the translation passes
   only what the body establishes.
7. **`0x802CF1E0` calls its neighbours by address.** `0x802CE740`/`0x802CE850`/
   `0x802CE940`/`0x802CE9B0`/`0x802CE9C0`/`0x802CED80` are translated by lanes
   A, B and C of this wave; this block leaves them as absolute-address calls
   like every other cross-lane callee in the module (the two `_D222`
   constants are local aliases, not redefinitions of the other lanes' names).
   `0x802C3020` is the module's existing `FN_802C3020_B`.
8. **`0x802CF1E0`'s in-place matrix apply.** `0x801C0540` is called with the
   same 12-float buffer as output and input (the shipped code does exactly
   that: `r3 = r5 = r1+0x28`). The Rust body passes the same raw pointer twice
   and the safety note says so.
9. **GX setters are named by shape.** The `0x801C8xxx` callees in
   `0x802CF1E0` are identified from their own exports' state writes and the
   argument counts already used by `src/engine/dsys.rs` and this file:
   `0x801C4A20` writes the `nChans` nibble of the state word at `+0x254`,
   `0x801C88F0` writes its TEV-stage field `(n - 1) << 10`, `0x801C47D0`
   writes `state+0xC8[fmt]` plus the fraction fields, `0x801C8500` writes a
   `0xE1 + 2 * stage` TEV colour word, `0x801C9A80` emits an `id`-tagged FIFO
   command followed by 12 floats, and `0x801C82F0`/`8330`/`8370`/`83D0` are the
   five- and six-argument TEV colour/alpha in/op writes. The `+0x14`-style GX
   names are not claimed as documented symbols; `// partial:` marks that in the
   body, which is why the row is `medium`.
10. **`0x802CF1E0`'s record and object layout.** Record `n` is
    `this + 0x38 + 0x10 * n`: bit count at `+0x00`, four colour bytes at
    `+0x01..+0x04`, angle at `+0x08`, radius at `+0x0C`. The object's
    `+0x08` word is forwarded as the TEV texture map, `+0x24` is a base angle
    in radians, and `+0x30` a per-slice colour/line gain. `// inferred:` the
    combination of one ring slice per matrix, one TEV stage per slice and a
    per-slice angle/radius is read as an arc/gauge mesh; the exports prove the
    arithmetic, not the art asset.

## Verification

The block was appended to a scratch copy of `src/`, `Cargo.toml` and
`Cargo.lock` (`%TEMP%\opencode\w222d`) and checked with `cargo check --lib`; it
exits 0 and the library emits no diagnostic pointing at any of the appended
lines (the 26 warnings in the tree all predate this lane).
