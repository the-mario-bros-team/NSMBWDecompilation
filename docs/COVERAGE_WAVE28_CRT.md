# Coverage — Wave 28 CRT continuation (`0x802D0000..0x802E0000`)

## How to read this file

This is the completion record for wave 28 of the Metrowerks runtime / CRT lane
in [`src/runtime/cpp.rs`](../src/runtime/cpp.rs). It answers five questions for
a reader with no context:

1. **What was added?** Thirty function bodies: two field accessors at
   `0x802DAD00`, twenty-six typed accessors and flag helpers for the C++
   record built by `0x802DBB70`, and the two floating-point print routines
   `0x802DE20C` (binary `double` to decimal record) and `0x802DE370`
   (half-even rounding of that record).
2. **Where is it?** Exclusively in [`src/runtime/cpp.rs`](../src/runtime/cpp.rs).
   No other Rust file, `mod.rs`, `lib.rs`, `Cargo.toml` or `platform.rs` was
   edited.
3. **How sure is it?** `confidence` measures the evidence, not the code style:
   `high` = every branch, store and call operand is explicit in the Ghidra C
   and the raw instructions; `medium` = the body is exact but one argument
   (the `mode` word forwarded to `0x802DC5D0`) is visible only from the raw
   register pass-through and the callee, not from Ghidra's signature.
4. **What is still unknown?** See **Uncertainties** and **Deliberately
   skipped**.
5. **What is the entry point?** Read [`fn_802dc4f0`] first for the record
   layout, then [`fn_802de20c`] and [`fn_802de370`] for the decimal print path.

Addresses are plain lowercase hex virtual image addresses (image base
`0x80004000`), not file offsets. Evidence is the per-function Ghidra C file
under `work/ghidra/main-gb/export/`. Every Rust function keeps an `# Original`
block with the address, size and evidence path. The raw `work/dol/main.elf`
was used to confirm instruction-level facts Ghidra drops: the `f1`/`r3`/`r4`
roles of the float setters, the hidden `mode` register, the `lfs`/`stfs` width
of the three-float copies, and the `stfd`/`lwz` split of the normalised
mantissa in `fn_802de20c`.

Terms: **DOL** is the Wii executable container; **CRT** is the C/C++ runtime;
**r2** is the second small-data base (`0x80433080`); **vtable** is the array of
function pointers stored at offset 0 of a polymorphic object; a **decimal
record** is the CodeWarrior float-printing structure
`{sign: u8, exponent: i16, length: u8, digits: [u8; 36]}`. See
`docs/GLOSSARY.md`.

Status follows `docs/RUST_STYLE.md`: `translated` = the full control flow and
every call are represented; `partial` = control flow is represented but one
operand or callee body is missing; `stub` = address/role only.

## Summary

| Metric | Count |
|---|---:|
| New function bodies in this pass | **30** |
| Bytes represented | **1,888** |
| Status `translated` | 30 |
| Status `partial` | 0 |
| Status `stub` | 0 |
| Confidence `high` | 21 |
| Confidence `medium` | 9 |
| Module | `runtime::cpp` |
| Evidence | `work/ghidra/main-gb/export`, `work/dol/main.elf` |

The 1,888 bytes are the sum of the 30 `functions.tsv` sizes: 8 + 12 + 8 + 64
+ 12 + 64 + 8 + 68 + 8 + 68 + 8 + 68 + 8 + 68 + 8 + 84 + 84 + 8 + 24 + 148
+ 24 + 8 + 8 + 8 + 8 + 8 + 8 + 212 + 356 + 420.

## Selection

The lane brief asks for 25–30 untranslated, meaningful functions in the band,
covering allocator, string/memory, integer/float math, C++ runtime, `stdio` and
save/restore stubs, while skipping 4-byte thunks and pure register-save
fragments. Selection was made in this order:

1. List every `functions.tsv` entry in `0x802D0000..0x802E0000` (354 entries).
2. Remove entries that already have a body in `src/runtime/cpp.rs` or a ledger
   row (the ledger alone is not sufficient: several bodies predate their row).
3. Rank the remaining entries by the lane categories. The classic CRT cluster
   `0x802DCC4C..0x802DF4B8` is already almost complete; only the decimal
   conversion pair `0x802DE20C`/`0x802DE370` remained. The C++ record
   vocabulary `0x802DAD00..0x802DC4F0` supplied the rest without leaving the
   band or duplicating another lane.
4. Read each candidate's Ghidra C and raw entry, and keep only functions whose
   body can be represented without inventing a symbol, an argument or a return
   value.
5. Skip the `lyt`-owned `0x802DB6B0..0x802DBD50` family, including `0x802DBD40`
   and `0x802DBD50`; the translated accessor run starts at `0x802DBD90`.

The result is 30 bodies. Twenty-one are `high` confidence and nine are
`medium`; the medium cases are all setters whose third argument is the hidden
`mode` word discussed under **Uncertainties**.

## Translated functions

| Address | Rust name | Role (plain English) | Status | Conf. | Evidence |
|---|---|---|---|---|---|
| `802dad00` | `fn_802dad00` | Reads the opaque handle at `+0x154`. | translated | high | `802dad00_FUN_802dad00.c` |
| `802dad10` | `fn_802dad10` | Clears the handle at `+0x154`. | translated | high | `802dad10_FUN_802dad10.c` |
| `802dbd90` | `fn_802dbd90` | Reads the `u16` property at `+0x16`. | translated | high | `802dbd90_FUN_802dbd90.c` |
| `802dbda0` | `fn_802dbda0` | Sets `+0x16` and updates flags with mask `0x8`. | translated | medium | `802dbda0_FUN_802dbda0.c` |
| `802dbde0` | `fn_802dbde0` | Reads the sign-extended byte at `+0x18`. | translated | high | `802dbde0_FUN_802dbde0.c` |
| `802dbdf0` | `fn_802dbdf0` | Sets `+0x18` and updates flags with mask `0x10`. | translated | medium | `802dbdf0_FUN_802dbdf0.c` |
| `802dbe30` | `fn_802dbe30` | Reads the `f32` property at `+0x1C`. | translated | high | `802dbe30_FUN_802dbe30.c` |
| `802dbe40` | `fn_802dbe40` | Sets `+0x1C` and updates flags with mask `0x20`. | translated | medium | `802dbe40_FUN_802dbe40.c` |
| `802dbe90` | `fn_802dbe90` | Reads the `f32` property at `+0x20`. | translated | high | `802dbe90_FUN_802dbe90.c` |
| `802dbea0` | `fn_802dbea0` | Sets `+0x20` and updates flags with mask `0x40`. | translated | medium | `802dbea0_FUN_802dbea0.c` |
| `802dbef0` | `fn_802dbef0` | Reads the `f32` property at `+0x24`. | translated | high | `802dbef0_FUN_802dbef0.c` |
| `802dbf00` | `fn_802dbf00` | Sets `+0x24` and updates flags with mask `0x80`. | translated | medium | `802dbf00_FUN_802dbf00.c` |
| `802dbf50` | `fn_802dbf50` | Reads the `f32` property at `+0x28`. | translated | high | `802dbf50_FUN_802dbf50.c` |
| `802dbf60` | `fn_802dbf60` | Sets `+0x28` and updates flags with mask `0x100`. | translated | medium | `802dbf60_FUN_802dbf60.c` |
| `802dbfb0` | `fn_802dbfb0` | Returns the address of the three-float vector at `+0x2C`. | translated | high | `802dbfb0_FUN_802dbfb0.c` |
| `802dbfc0` | `fn_802dbfc0` | Copies three floats to `+0x2C..+0x34`, mask `0x200`. | translated | medium | `802dbfc0_FUN_802dbfc0.c` |
| `802dc020` | `fn_802dc020` | Copies three floats to `+0x38..+0x40`, mask `0x400`. | translated | medium | `802dc020_FUN_802dc020.c` |
| `802dc080` | `fn_802dc080` | Returns the address of the four-byte value at `+0x44`. | translated | high | `802dc080_FUN_802dc080.c` |
| `802dc0f0` | `fn_802dc0f0` | Indexes the byte table at `+0x48` (`group*8 + element*4`). | translated | high | `802dc0f0_FUN_802dc0f0.c` |
| `802dc110` | `fn_802dc110` | Writes two three-byte entries into a table row, mask `0x2000 << index`. | translated | medium | `802dc110_FUN_802dc110.c` |
| `802dc1b0` | `fn_802dc1b0` | Reads one indexed byte from the table at `+0x4B`. | translated | high | `802dc1b0_FUN_802dc1b0.c` |
| `802dc240` | `fn_802dc240` | Returns the address of the vector at `+0x58`. | translated | high | `802dc240_FUN_802dc240.c` |
| `802dc2a0` | `fn_802dc2a0` | Returns the address of the vector at `+0x60`. | translated | high | `802dc2a0_FUN_802dc2a0.c` |
| `802dc300` | `fn_802dc300` | Returns the address of the vector at `+0x68`. | translated | high | `802dc300_FUN_802dc300.c` |
| `802dc370` | `fn_802dc370` | Returns the address of the vector at `+0x74`. | translated | high | `802dc370_FUN_802dc370.c` |
| `802dc3e0` | `fn_802dc3e0` | Returns the address of the six-float block at `+0x80`. | translated | high | `802dc3e0_FUN_802dc3e0.c` |
| `802dc480` | `fn_802dc480` | Returns the address of the six-float block at `+0x8C`. | translated | high | `802dc480_FUN_802dc480.c` |
| `802dc4f0` | `fn_802dc4f0` | Initialises the record's scalar, vector and flag fields. | translated | high | `802dc4f0_FUN_802dc4f0.c` |
| `802de20c` | `fn_802de20c` | Converts a binary `double` to a decimal digit record. | translated | high | `802de20c_FUN_802de20c.c` |
| `802de370` | `fn_802de370` | Rounds that record half-to-even at a requested digit count. | translated | high | `802de370_FUN_802de370.c` |

## What each cluster does

### The `+0x154` handle (`802dad00`, `802dad10`)

These are a get/set pair for one opaque 32-bit slot. The getter is a single
`lwz r3,0x154(r3)`; the setter writes zero with `stw r0,0x154(r3)`. They are
kept because the surrounding object code calls them by address, and because
the zeroing convention (a null handle means "no owner") is observable.

### The typed C++ record (`802dbd90..802dc4f0`)

`0x802DBB70` already builds this record in this module and installs vtable
`0x80350A08`; `0x802DBB20` initialises its `+0x8..+0x29` fields. The 26 bodies
added here are the record's typed vocabulary:

| Field | Type | Accessors |
|---|---|---|
| `+0x04`, `+0x08` | flag words | updated by [`fn_802dc5d0`] with a per-property mask |
| `+0x16` | `u16` | `802dbd90` read / `802dbda0` write |
| `+0x18` | `i8` | `802dbde0` read / `802dbdf0` write |
| `+0x1C` | `f32` | `802dbe30` read / `802dbe40` write |
| `+0x20` | `f32` | `802dbe90` read / `802dbea0` write |
| `+0x24` | `f32` | `802dbef0` read / `802dbf00` write |
| `+0x28` | `f32` | `802dbf50` read / `802dbf60` write |
| `+0x2C..+0x34` | three `f32` | `802dbfb0` address / `802dbfc0` copy |
| `+0x38..+0x40` | three `f32` | `802dc020` copy |
| `+0x44..+0x4E` | byte table | `802dc080`, `802dc0f0`, `802dc110`, `802dc1b0` |
| `+0x58..+0x70` | float vectors | `802dc240`, `802dc2a0`, `802dc300` |
| `+0x74`, `+0x80`, `+0x8C` | float blocks | `802dc370`, `802dc3e0`, `802dc480` |
| `+0x04..+0x94` | whole record | `802dc4f0` initialiser |

The setters call [`fn_802dc5d0`]`(object, mask, mode)` before storing. The mask
matches the property (for example `0x20` for `+0x1C`), and `mode` selects
whether the bit is set in or cleared from the two flag words. Ghidra drops
`mode` from the signatures because most setters never touch `r5` themselves;
the float setters make it explicit with `mr r5,r4` because they reuse `r4` for
the mask.

The address getters exist so callers can pass a vector to maths routines
without copying. They are deliberately tiny: each is an `addi` plus `blr`, but
they are real exported entry points, not thunks.

`0x802DC4F0` is the record-wide initialiser. It loads the zero float at
`0x8042F678` (`r2 - 0x3A08`) and stores it in every float slot, clears the two
flag words, the three halfwords and the byte at `+0x18`, clears
`+0x44..+0x57`, and leaves the documented gaps untouched. Its store order is
kept exactly as the raw sequence emits it.

### Decimal conversion and rounding (`802de20c`, `802de370`)

`fn_802de20c` is the `double` → decimal-record step used before `printf`
formats `%f`/`%e`/`%g`:

* zero is written as sign + exponent 0 + digit `0`;
* infinity/NaN becomes one `'I'`/`'N'` digit (classification `1` is NaN);
* a finite non-zero value is negated to a magnitude, normalised by
  `0x802E81D4` to `[0.5, 1)`, and its binary exponent is written through a
  stack slot;
* the low/high words of the normalised mantissa are bit-scanned to find the
  number of significant bits (`32 - clz` for a non-zero low word, `64 - clz`
  on the high word with the implicit bit otherwise);
* `0x802DDEF0` builds the power-of-two record for
  `exponent - (0x35 - bits)`, `0x802E825C` scales the mantissa by
  `2^(0x35 - bits)`, `0x802DD238` truncates the scaled double to `u64`, and
  `0x802DDAA0` turns that integer into digits;
* `0x802DDB7C` merges the digit record with the exponent record, and the sign
  byte is restored last.

`fn_802de370` is the rounding pass. It reads the requested count from the
input record's `+2` halfword (before calling `fn_802de20c`, because the two
records may alias), clamps it to `0x24`, and applies half-to-even rounding at
the first dropped digit. Exactly `5` looks at the remaining digits and at the
parity of the last kept digit; a carry out of the most significant digit
becomes digit `1` and bumps the exponent. It then truncates the count, adjusts
the exponent by `count - 1`, and converts digits `0..9` to ASCII by adding
`'0'`. This is the pass the wave-24 notes referred to as the `0x802DE370`
rounding step.

## Uncertainties

* The nine `medium` setters (`802dbda0`, `802dbdf0`, `802dbe40`, `802dbea0`,
  `802dbf00`, `802dbf60`, `802dbfc0`, `802dc020`, `802dc110`) take a `mode`
  word that Ghidra omits. It is recovered from the raw register pass-through
  (`r5` for the scalar setters, `mr r5,r4` for the float setters, `r7`→`r5`
  for `802dc110`) and from [`fn_802dc5d0`]'s use of its third argument. The
  body is exact; the caller-visible meaning of `mode` is not reconstructed.
* `fn_802de20c` calls `0x802E81D4` and `0x802E825C`, which live in the next
  band (`0x802E0000..`) and are outside this lane. They are called by address
  through two documented host-side helpers; their bodies are not translated
  here. The `f1`/`r3` argument split was verified against the raw prologue of
  `0x802DE20C`, but the helpers themselves remain unknown.
* `fn_802de370`'s value arrives in `f1` even though Ghidra declares only two
  integer arguments. The raw `lha r31,2(r3)` / `mr r30,r4` prologue shows the
  two records; the existing `f1` is forwarded to `0x802DE20C`.
* The record layout in the table above is inferred from the offsets touched
  by the accessors and by `0x802DBB20`/`0x802DBB70`; the C++ class name is not
  present in the image, so it is not invented.

## Deliberately skipped

These addresses are **not** translated in this pass and have no row in
`ledger/agents/wave28-crt.csv`:

* `802d2f10`, `802d3260`, `802debb4` — 4-byte `b` thunks.
* `802dcd40..802dcdbc` — the CodeWarrior `__savegprlr_N` / `__restgprlr_N`
  family; each entry is a 4-byte alternate entry into one `stw`/`lwz` run.
* `802dcccc`, `802dcce0`, `802dcd18`, `802dcd2c`, `802dcd74`, `802dcdc0` —
  pure FPR/GPR save or empty `return;` fragments with no high-level arguments.
* `802dcbfc` — the adjustor dispatcher whose jump table Ghidra could not
  recover; it is already called by address from other lanes.
* `802da040`, `802da6f0`, `802dad20` — their C bodies consume the fake 64-bit
  return of `802dcd74`/`802dcd40`. The raw prologues save the very registers
  Ghidra treats as return words, so the real argument registers and frame
  offsets cannot be recovered without guessing.
* `802dff64` — the `%a`/`%f` formatter; its C contains the `ldexpf` /
  `unaff_GQR0` paired-single artefacts described in `docs/QUALITY.md` §5.3.
* `802d0030..802d8e70` — EGG render/effect code with GQR `ldexpf` artefacts;
  it belongs to the engine lanes, not to the CRT.
* `802db6b0..802dbd50` — the `lyt` scratch-transform helper family already
  referenced from `src/ui/lyt.rs`; left to that lane. The translated run
  therefore starts at `802dbd90`.

Already-translated neighbours were not duplicated: `802dc090` (mask `0x1000`
and four-byte colour), `802dbb20`, `802dbb70`, `802dbc20`, `802dc5d0`,
`802dc620`, `802ddaa0`, `802ddb7c`, `802ddef0`, `802dd238`, `802df414` and
`802df42c` keep their existing bodies.

## Verification

* `cargo check` finishes successfully; the 11 remaining crate warnings are in
  other lanes' files and none points at `src/runtime/cpp.rs`.
* `node tools/doc-lint.mjs src` → `{"ok":true,"files":68,"violations":0}`.
* Footer counts: 30 new bodies, 1,888 bytes, 21 `high`, 9 `medium`.
