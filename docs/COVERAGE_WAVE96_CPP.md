# Coverage - Wave 96 (runtime::cpp)

## How to read this file

Each row is one `pub (unsafe) fn fn_<address>` body in `src/runtime/cpp.rs` that
this pass vouched for. The `bytes` column is the exact size from
`work/ghidra/main-gb/export/functions.tsv`, re-read for this pass. The body's
`# Original` block cites the same export file. Confidence matches the CSV row.
Field offsets are relative to the first argument. SDA cells are given as
absolute addresses, resolved with `r13 = 0x8042F680` and `r2 = 0x80433080` (the
values Waves 77, 89 and 92 used), and float constants were read out of
`work/dol/main.elf` rather than guessed.

**Every signature in this pass was checked against the raw instruction stream.**
The export loses the argument registers of any function whose first call is the
CodeWarrior `__savegprlr_N` prologue helper: Ghidra models that helper as
returning the incoming `r3`/`r4` in a 64-bit value, so the callers print as
"no arguments". `work/dol/main.elf` was disassembled at each prologue to recover
the real argument registers; the reasoning is repeated in each body's
`# Confidence` note.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/cpp.rs` | `runtime::cpp` | 14 | 2276 | 8 | 6 |

## What this pass covers

Address range: `0x802D0D90..0x802DEBB4` (the whole `0x802D0000` bucket was
scanned; only the entries below can be reconstructed honestly).

- **The four-float layout record chain (`0x802D0D90..0x802D1900`).** One
  self-contained family that shares a field vocabulary — `+0x08`/`+0x0C` size,
  `+0x34` flags, `+0x40`/`+0x44` scale, `+0x58` vector — and one `2^52`/`2^52 +
  2^31` integer-conversion idiom:
  - `0x802D0D90` builds a record: the `0x802C6A30` initialiser with
    `(10.0f, 100000.0f, this, 1, &pair, arg7)`, vtable `0x80350504`, the two
    scale words, `0.0f` in `+0x48..+0x54`, then `0x802D1250(this, arg6)`.
  - `0x802D0F40` saves the global vector with `0x802C6DF0`, installs
    `0x8042AF18` with `0x802C6DD0`, runs `0x802C6D60(this)`, restores, and
    forwards the record's twelve values to `0x802D3200` (six floats),
    `0x802D3230` (four words) and `0x802D3270` (two words).
  - `0x802D1010` is the textured twin: `0x802C6D80(this, &word)` instead of the
    restore path, then the four `0x80253D*` rectangle writers.
  - `0x802D1850` fills one axis of a parent record; the third value the export
    types as `undefined8` is a single FPR argument.
  - `0x802D1900` composes the parent record on a 0x90-byte stack frame and
    submits it through `0x802C1D10(block, mode&1, mode>>1&1, mode>>2&1,
    &descriptor, 1)`.
- **`0x802D2610` — console panel publication and AX mixer bring-up.** Stores the
  panel size at `0x8042AF38`/`0x8042AF3A`, copies the four widget bytes to
  `0x8042AF4C`, mirrors the first word through the device singleton's virtual
  `+0x10` slot into `+0x14..+0x17`, then runs `0x801C6D90`, `0x801C6FF0`,
  `0x801C76C0`, `0x802D2730`, sets `0x80429B88 = 1` and calls `0x8024A1D0(1)`.
- **`0x802D48C0` — view transform republish.** The caller Wave 89 identified as
  the consumer of [`fn_802d4830`]: copies three words into `owner+0x6E4..0x6EC`,
  runs four `0x6C`-stride iterations of `0x802750C0/D0/E0`, publishes
  `0x8042AF70`/`0x8042AF74`, and hands `record+0x60` to [`fn_802d4fd0`].
- **`0x802D6F20` and `0x802D7070`.** The first is the equal-tempered playback
  ratio `2^(abs(index)/12)`, using the twelve `f32` factors at `0x802FD2A0`
  (read out of the image: `1.0, 1.0595, 1.1225, ...`); the second is the fade
  tick that interpolates `min + count*(max-min)/limit` from the `0x80260860`
  object and promotes the stage words `+4`/`+8` to `2` when the position reaches
  `0.0f`.
- **`0x802D9570` — row submit.** `0x80286CA0(f0, f1, 0x80286630(), arg4, arg5)`
  followed by one `vtable+0x18(entry, arg6, arg7)` call per non-null entry of
  the array at `row+0x2C`.
- **Four four-byte entries.** `0x802D2F10` and `0x802D3260` are plain `b` tail
  calls into the already-translated GX helpers `0x801C76C0` / `0x801C9CD0`;
  `0x802DEBB4` is a `b` back to the `ftell` body `0x802DEB0C`; `0x802DB0E0` is
  a bare `blr`.

### New names introduced

`SDA2_FLOAT_ZERO`, `SDA2_CULL_RANGE`, `SDA2_MIX_LEVEL`, `SDA2_RATIO_ONE`,
`SDA2_RATIO_BASE`, `SDA2_U32_BIAS`, `SDA2_I32_BIAS`, `SDA2_FADE_TARGET`,
`PITCH_RATIO_TABLE`, `VIEW_TRANSFORM_OWNER`, `VIEW_TRANSFORM_SOURCE`, and the
local `call_f32x4` ABI helper. `SDA2_U32_BIAS`/`SDA2_I32_BIAS` are declared for
completeness of the idiom; only `SDA2_I32_BIAS` is used by the landed bodies.
No existing constant was renamed.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802D0D90` | `fn_802d0d90` | 188 | high |
| `0x802D0F40` | `fn_802d0f40` | 208 | high |
| `0x802D1010` | `fn_802d1010` | 304 | high |
| `0x802D1850` | `fn_802d1850` | 168 | high |
| `0x802D1900` | `fn_802d1900` | 232 | medium |
| `0x802D2610` | `fn_802d2610` | 284 | medium |
| `0x802D48C0` | `fn_802d48c0` | 236 | medium |
| `0x802D6F20` | `fn_802d6f20` | 212 | medium |
| `0x802D7070` | `fn_802d7070` | 244 | medium |
| `0x802D9570` | `fn_802d9570` | 184 | medium |
| `0x802D2F10` | `fn_802d2f10` | 4 | high |
| `0x802D3260` | `fn_802d3260` | 4 | high |
| `0x802DB0E0` | `fn_802db0e0` | 4 | high |
| `0x802DEBB4` | `fn_802debb4` | 4 | high |

## Deliberate skips

42 addresses in the bucket (56 untranslated minus the 14 landed above) still
have no body and no ledger row. They fall into exactly seven groups.

1. **The register-save chain, 32 addresses.** `0x802DCD40..0x802DCD74`,
   `0x802DCD8C..0x802DCDC0`, `0x802DCCCC`, `0x802DCCE0`, `0x802DCD18` and
   `0x802DCD2C` are the Metrowerks `__savegprlr_N` / `__restgprlr_N` /
   `__savefpr_N` / `__restfpr_N` helpers. `docs/QUALITY.md` section 5.5 excludes
   the range from translation scheduling; the raw words were re-read for this
   pass (`0x802DCD40` = a `stw r14,-0x48(r11)` run ending in `blr`,
   `0x802DCDC0` = `lwz r27..r31` + `blr`, `0x802DCD18`/`0x802DCD2C` = `lfd`
   chains) and confirmed. Each is an alternate entry point into one spill run,
   so there is no high-level body to write — a Rust function could only be a
   lie about what the bytes do.
2. **Deferred render paths, 2 addresses.** `0x802D12E0` (1240 bytes) and
   `0x802D1E20` (968 bytes) are *recoverable*, and the operand that made them
   look opaque is now identified: `0x802D12E0`'s owner is the `r3` argument
   (proved by the `0x802D17C0` call site) and `0x802D1E20`'s slot index is its
   first argument (`mulli r29,r23,12` right after the prologue). Both are long
   multi-branch float/`fctiwz` paths over the `DAT_80424820` panel slots, and
   neither was finished to the standard of the bodies above within this pass,
   so they are the first targets for the next runtime lane rather than
   untranslatable code.
3. **Unrecoverable operand, 1 address.** `0x802D2310` consumes the same padded
   prologue return in the `(int)uVar11 == (&DAT_80424824)[...]` compare, but it
   has no callers, so there is nothing to cross-check the value against.
4. **Lost float arguments, 2 addresses.** `0x802D2550` calls
   `0x802D3200()` with the six float arguments the export drops (the existing
   `fn_802d3200` body in this file is already a no-argument stub), and
   `0x802D0030` needs four GX FIFO `stfs`/`stb` bursts that the export collapses
   into one store plus two synthetic calls. Writing either from the C would
   misstate the call.
5. **Paired-single (`psq_l`) artefacts, 2 addresses.** `0x802DADD0` (paired
   two-float loads under `unaff_GQR0`, section 5.3) and `0x802DFF64` (the
   `%a`/`%f` formatter, `ldexpf`/`unaff_GQR0`). Both were left to a re-export
   with the GQR values set, as section 5.3 prescribes.
6. **Wrapper of an unrecoverable body, 2 addresses.** `0x802D79A0` takes its
   object pointer from the `__savegprlr` pseudo-return and its three call sites
   never set up `r4`/`r5`; `0x802D7860` is its only caller and inherits the gap.
7. **Unrecovered jump table, 1 address.** `0x802DCBFC` is an indirect jump
   through `r12` whose table Ghidra could not recover.

The counts add up: 32 + 2 + 1 + 2 + 2 + 2 + 1 = 42.

## Verification

- Every address and byte size was re-read from `functions.tsv` for this pass;
  `0x802D9570` is 184 bytes (0x802D9570..0x802D9628), and 212 was only the
  length of the surrounding disassembly dump.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path,
  including the two `_thunk_FUN_...` filenames.
- Every one of the 14 rows has exactly one body in `src/runtime/cpp.rs`
  (319 `fn_` bodies in the file, all unique).
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` exits `0` and reports no error in
  `src/runtime/cpp.rs`; the file's only warnings are two pre-existing
  never-read warnings outside this pass's section.
- `node tools/check-lane-csv.mjs wave96` reports `wave96-cpp.csv: 14 rows` and
  `OK`.
