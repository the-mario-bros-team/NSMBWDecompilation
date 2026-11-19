# Coverage - Wave 221, lane B, `src/resource/archive.rs`

## How to read this file

- **Scope** - the ten bodies added to `src/resource/archive.rs` by wave 221,
  lane B, module `resource::archive`: the `0x802CA4C0..0x802CDCF8` band.
  Earlier work in this file is documented in the lane files of the other
  waves; this file does not repeat it.
- **Addresses are analysis addresses.** They are absolute `main.dol`
  addresses, as printed by Ghidra and `work/dol/main.elf`.
- **Evidence** is the Ghidra C under `work/ghidra/main-gb/export/`; the exact
  file is named in every table row. Sizes come from
  `work/ghidra/main-gb/export/functions.tsv`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and every observable store are represented.
- **Confidence** rates the translation, not the guessed role. Six bodies are
  literal (`high`); four are `medium` because they either lean on unexported
  callees (`0x802CD050`, `0x802CD1F0`, `0x802CDB70`) or required a store
  sequence to be recovered from the disassembly (`0x802CC730`).
- **Names** stay `fn_802cxxxx`. The shipped image has no symbol table, so no
  Nintendo symbol is invented. New constants carry the lane `_B` suffix.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 10 |
| text bytes represented | 2460 |
| status `translated` | 10 |
| status `partial` | 0 |
| status `deferred` | 0 |
| confidence `high` | 6 |
| confidence `medium` | 4 |

Byte total: `156 + 8 + 436 + 76 + 704 + 272 + 220 + 144 + 52 + 392 = 2460`,
which is also the instruction extent sum; the `functions.tsv` sizes agree with
every extent, so no size above is a padding artefact.

## Size verification

Every `functions.tsv` size was checked against the prologue and the closing
`blr` in `work/dol/main.elf`. All ten agree with the instruction extent, so no
row is a prologue or padding artefact.

| address | tsv | real extent | note |
|---|---:|---:|---|
| `0x802CA4C0` | 156 | 156 | `blr` at `0x802CA558`; 4 padding bytes before `0x802CA560` |
| `0x802CB3F0` | 8 | 8 | `addi` then `blr` at `0x802CB3F4` |
| `0x802CB5F0` | 436 | 436 | `blr` at `0x802CB7A0`; 12 padding bytes before `0x802CB7B0` |
| `0x802CC000` | 76 | 76 | leaf, `blr` at `0x802CC048` |
| `0x802CC730` | 704 | 704 | `blr` at `0x802CC9EC` |
| `0x802CD050` | 272 | 272 | `blr` at `0x802CD15C` |
| `0x802CD1F0` | 220 | 220 | `blr` at `0x802CD2C8` |
| `0x802CD3E0` | 144 | 144 | leaf, `blr` at `0x802CD46C` |
| `0x802CD470` | 52 | 52 | `blr` at `0x802CD4A0`; 12 padding bytes before `0x802CD4B0` |
| `0x802CDB70` | 392 | 392 | `blr` at `0x802CDCF4`; 8 padding bytes before `0x802CDD00` |

## Function table

| address | bytes | confidence | what the body does |
|---|---:|---|---|
| `0x802CA4C0` | 156 | high | resets the render-entry defaults and seeds the `+0x68` flag halfword |
| `0x802CB3F0` | 8 | high | returns the four-byte SDA table at `r13 - 0x5B14` |
| `0x802CB5F0` | 436 | high | programs the three-channel GX window/channel layout |
| `0x802CC000` | 76 | high | splits a linear index into cell coordinates |
| `0x802CC730` | 704 | medium | emits one cell sprite and the last-column edge quad |
| `0x802CD050` | 272 | medium | allocates and installs one 0x80-byte slot object |
| `0x802CD1F0` | 220 | medium | resolves one slot and applies its tagged result list |
| `0x802CD3E0` | 144 | high | refills the visible-entry array from 0xA4-byte records |
| `0x802CD470` | 52 | high | refreshes that array and clears the `+0x10` cursor |
| `0x802CDB70` | 392 | medium | resets a 0x50-byte draw descriptor through two virtual slots |

## Disassembler caveats

`0x802DCD70`, `0x802DCD74`, `0x802DCDBC` and `0x802DCDC0` are the Metrowerks
`__savegpr` / `__restgpr` helpers. The Ghidra exports render them as calls, so
`0x802CB5F0` appears to open with `FUN_802dcd70()` and `0x802CD050` /
`0x802CD1F0` appear to open with a `u64`-returning `FUN_802dcd74()`. The save
helpers only spill `r26..r31`; `r3`, `r4` and `r5` still hold the caller's
arguments afterwards. Those bodies therefore take their real arguments from
the prologue, and no restore-stub call is emitted:

- `fn_802cd050(record, owner)` - the export types the function `void` and
  hides both arguments in its phantom `u64`;
- `fn_802cd1f0(record, index, object)` - `r4` is the slot number and `r5` the
  query context;
- the two `ldexpf` arms on `unaff_GQR0` in `0x802CC730` come from the
  `psq_st f31` prologue and are not code.

## Notes on `0x802CC730`

The export keeps only a single `_DAT_cc008000` store, but the shipped body at
`0x802CC970..0x802CC9BC` writes a complete four-vertex immediate-mode strip:

```text
byte 0  (h, h)  byte 1  (h, h - e)  byte 2  (h + e, h - e)  byte 3  (h + e, h)
```

with `h` in `f31` and `e` the float at `r2 - 0x3C10`. All twelve stores are
reproduced in the Rust body. The three colour channels are scaled in single
precision: the export's `(double)CONCAT44(0x43300000, ch) - dVar8` idiom is
the compiler's integer-to-double magic, and the following `fsubs` + `fmuls` +
`fctiwz` make the effective computation `(float)ch * scale`.

## Notes on `0x802CD050`

The second scan keeps the **last** free slot, not the first: the export assigns
`uVar5 = uVar4` on every null entry, and the count at `record + 0x06` only
grows when no null was found. The store of the new object and its virtual
`+0x0C` call are unconditional, so a failed `0x802B9210(0x80)` dereferences
null exactly as the shipped body does; that fault is reproduced rather than
masked.

## Notes on `0x802CD1F0`

`0x802CE410` returns an entry count in the low halfword and writes 4-byte
`(id, tag)` entries. A tag byte of `0xFF` (the `extsb` + `cmpi -1` test) is
skipped; every other entry is resolved with `0x802CE620` and stored through
`0x802CDA80`. The function's result is the full `r3`, which the export discards
when it declares the function `void`.

## Notes on `0x802CDB70`

The shipped prologue zeroes 0x38 bytes of dead stack at `r1 + 0x1C`; no pointer
to that block is ever passed to either virtual slot, so those stores are not
reproduced. The descriptor itself is fully represented, including the
four-byte copy from `r13 - 0x5B14` and the `0.0` float at `+0x3C`.

## Verification

The block was appended to a scratch copy of `src/`, `Cargo.toml` and
`Cargo.lock` and checked with `cargo check --lib`; the crate compiles with exit
code 0 and no diagnostic that points into the appended block.
