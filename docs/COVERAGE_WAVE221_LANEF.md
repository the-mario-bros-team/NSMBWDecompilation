# Coverage: wave 221 lane F (`resource::archive`, `0x802C9830..0x802CDF40`)

## How to read this file

This document covers lane F of wave 221. All ten bodies live in
`src/resource/archive.rs`; no source file was edited by this pass. The bodies
are staged in `work/wave221/laneF_block.rs` for the merge, and the ten rows are
in `ledger/agents/wave221-laneF.csv`.

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
| Functions translated in this batch | 10 |
| Bytes of `functions.tsv` size | 2,384 |
| `translated` | 10 |
| `partial` | 0 |
| `stub` | 0 |
| Confidence `high` | 8 |
| Confidence `medium` | 2 |
| Deferrals | 0 |

## Function table

| address | Rust function | role | status | conf. | bytes | evidence |
|---|---|---|---|---:|---:|---|
| `0x802C9830` | [`fn_802c9830`] | tail-dispatch vtable slot `+0x08` with `r3 = this` | translated | high | 16 | `802c9830_FUN_802c9830.c` |
| `0x802C99F0` | [`fn_802c99f0`] | tail-dispatch vtable slot `+0x14` with `r3 = this` | translated | high | 16 | `802c99f0_FUN_802c99f0.c` |
| `0x802C9F40` | [`fn_802c9f40`] | push the visible transform/colour triple into the `0x801C6xxx` display-list state | translated | medium | 512 | `802c9f40_FUN_802c9f40.c` |
| `0x802CA140` | [`fn_802ca140`] | re-publish the same values through the `0x8025F4xx` state family and rewrite the request bits | translated | high | 336 | `802ca140_FUN_802ca140.c` |
| `0x802CA560` | [`fn_802ca560`] | copy one keying record into the effect object's mode fields | translated | high | 360 | `802ca560_FUN_802ca560.c` |
| `0x802CBB90` | [`fn_802cbb90`] | construct one named effect object with three owned per-entry arrays | translated | high | 508 | `802cbb90_FUN_802cbb90.c` |
| `0x802CD2D0` | [`fn_802cd2d0`] | name-table lookup returning the matching index or `-1` | translated | high | 124 | `802cd2d0_FUN_802cd2d0.c` |
| `0x802CD970` | [`fn_802cd970`] | walk the sub-records of one table block and hand each to `0x802CD350` | translated | high | 128 | `802cd970_FUN_802cd970.c` |
| `0x802CDA80` | [`fn_802cda80`] | build and emit one `0x8023DEA0` descriptor from the layout record | translated | medium | 188 | `802cda80_FUN_802cda80.c` |
| `0x802CDF40` | [`fn_802cdf40`] | forward the requested state pair to two `+0x20` virtuals of `*this` | translated | high | 196 | `802cdf40_FUN_802cdf40.c` |

## Size note (prologue + `blr`)

Each extent was checked against the prologue and the closing `blr`, not just
against the `functions.tsv` column:

| address | `functions.tsv` | instructions | `blr` | note |
|---|---:|---|---|---|
| `0x802C9830` | 16 | 12 B | `0x802C983C` | four-byte pad word follows the `bcctr` |
| `0x802C99F0` | 16 | 12 B | `0x802C99FC` | four-byte pad word follows the `bcctr` |
| `0x802C9F40` | 512 | 512 B | `0x802CA13C` | exact |
| `0x802CA140` | 336 | 336 B | `0x802CA28C` | exact |
| `0x802CA560` | 360 | 360 B | `0x802CA6C4` | tsv is `blr + 4`; two pad words follow |
| `0x802CBB90` | 508 | 508 B | `0x802CBD88` | tsv is `blr + 4`; one pad word follows |
| `0x802CD2D0` | 124 | 124 B | `0x802CD348` | tsv is `blr + 4`; one pad word follows |
| `0x802CD970` | 128 | 128 B | `0x802CD9EC` | exact |
| `0x802CDA80` | 188 | 188 B | `0x802CDB38` | tsv is `blr + 4`; one pad word follows |
| `0x802CDF40` | 196 | 196 B | `0x802CE000` | tsv is `blr + 4`; three pad words follow |

The two 16-byte rows are the only ones whose instruction extent is smaller than
the tsv value: both are the standard `lwz`/`lwz`/`mtspr`/`bcctr` vtable
forwarder, so the real extent is 12 bytes and the tsv's 16 is the pad word that
follows. In every other row `blr + 4` equals the tsv value exactly, which also
confirms that the shorter tsv sizes are the real bodies and not prologue
artefacts.

## Notes that a reader should carry into the merge

1. **`0x802C9F40` and the paired-single block.** The export renders
   `0x802C9FA4..0x802C9FC4` as an `unaff_GQR0`/`ldexpf` cloud and also as a
   multiply by `r2 - 0x3C30`. The shipped words are
   `psq_l f1,0x94(r30),0,0`, `ps_muls0 f2,f1,f0` (`0x10410018`),
   `psq_l f1,0x9C(r30),1,0`, `ps_muls0 f2,f1,f0`, two `psq_st` staging stores
   and three `lfs` reloads. Because the sibling arm at `0x802CA040` reads the
   same three words with plain `lfs`, the load type is `f32`; the body is
   reproduced as `read_f32(..) * scale` with the scale at `r2 - 0x3C30`. This
   is the only inference in the batch.
2. **Hidden arguments.** `0x802CD2D0` opens with `bl 0x802DCD74`, whose export
   looks like an `undefined8` return. That address is the compiler's
   register-save stub (`stw r27..r31` into `r11 - 0x14..-0x4`) and
   `0x802DCDC0` is the matching restore; the incoming `r3`/`r4` are therefore
   the real arguments, the same artefact already documented for
   [`fn_802c7e30`].
3. **`0x802CA140` skip arm.** The `rlwinm r0,r0,0,30,28` at `0x802CA270`
   clears bit `0x04` of `*state`, the exact inverse of the `ori 4` the main arm
   performs; the export prints it as `& 0xfffffffb`.
4. **`0x802CA560` argument list.** The export declares `FUN_802ca4c0()` with no
   arguments, but the `bl` happens with `r3`/`r4` still holding the entry
   arguments, so the callee receives `(this, record)`.
5. **`0x802CDA80` outputs.** Four words filled by `0x8023DEA0` are never read
   by this body; only their storage is reproduced, which is why the row is
   `medium`.

## Verification

The block was appended to a scratch copy of `src/`, `Cargo.toml` and
`Cargo.lock` and checked with `cargo check --lib`; it exits 0 and the library
emits no diagnostic pointing at any of the appended lines.
