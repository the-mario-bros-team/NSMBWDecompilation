# Coverage - Wave 89 (runtime::cpp)

## How to read this file

Each row is one `pub unsafe fn fn_<address>` body appended to
`src/runtime/cpp.rs` by this pass. The `bytes` column is the exact size from
`work/ghidra/main-gb/export/functions.tsv`; the body's `# Original` block cites
the same export file. Confidence is the same value as the body doc block.

Roles that the Ghidra export does not prove are marked `// inferred:` in the
bodies and `// unknown:` where nothing can be concluded. Field offsets are
relative to the first argument unless stated otherwise. SDA cells are given as
absolute addresses; the resolver values used are `r13 = 0x8042F680` (the same
one Wave 77 established for the `r13 - 0x5B00` latch) and `r2 = 0x80433080`
(the existing `SDA2_BASE` constant).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/cpp.rs` | `runtime::cpp` | 26 | 7188 | 7 | 18 |

One further body (`fn_802d42d0`) is `partial` / `low`: see the deliberate-skips
section for the one loop path that the export prints ambiguously.

## What this pass covers

Address range: `0x802D2730..0x802DB710`.

- **AX engine setup (`0x802D2730..0x802D2B60`).** Seven routines from the audio
  DSP band, all of them literal `0x801Cxxxx` AX-library calls. `0x802D2730`
  tears the engine down (`0x801C47C0`, `0x801C9A60`, `0x801C58B0`,
  `0x801C9D60`), copies the live engine's four mode bytes out of the vtable slot
  at `+0x10`/`+0x14`, and restarts the chain through `0x802D2800`.
  `0x802D2800` is the once-only gate: bit 0 of the `u16` at `0x8042AF50`
  (`r13 - 0x4730`) selects the seven one-shot initialisers, and `0x802D2E50`
  runs unconditionally. The five initialisers that follow
  (`0x802D2850`, `0x802D2990`, `0x802D2A50`, `0x802D2AD0`, `0x802D2B60`) are
  the twelve aux descriptor writes, the mix-slot format, the output mode table,
  the eight bus resets and the workspace/voice-slot claims. Every SDA cell of
  the group is resolved to a named constant.
- **The last three `+0x4C4` view loaders (`0x802D39B0..0x802D3CE0`).** The
  family started in Wave 65 and extended in Wave 77 is finished here. Each
  loader guards on the busy word at `+8`, points `+8` at a different child
  (`+0xC`, `+0x198`, `+0x374`), binds one or two archives, then runs the
  `0x80277CF0` / `0x80277E40` / `0x80277AB0` size-and-data sequence and
  publishes a different state at `+0x4CC` (`1`, `2`, `4`). `0x802D39B0` is the
  only place in the band with string evidence: the four ASCII tags `ARCH`,
  `ARCL`, `APLM`, `APLS` as the loader keys `0x41524348`, `0x4152434C`,
  `0x41504C4D`, `0x41504C53`.
- **The SDA compare hook (`0x802D42C0`, `0x802D42D0`).** A 16-byte indirect call
  through the SDA pair `0x8042AF78` / `0x8042AF7C` (`r13 - 0x4708` /
  `r13 - 0x4704`), plus the driver that publishes that pair and walks one of the
  two record lists of the `+0x4C4` object, materialising a handle per node with
  `0x8027A160` from `node - 0xF8` and releasing it with `0x8027A200`.
- **The 0x6C-stride transform record (`0x802D4830`).** `0x802D61C0` runs first,
  then `+0x24` takes `0x20`, the twelve words at `+0x28..+0x54` all take the
  third word of the SDA2 triplet at `0x8042F5E0`, `+0x58`/`+0x5C` take the first
  two, and `+0x60`/`+0x64`/`+0x68` take `0x25800`.
- **The eight per-mode effect builders (`0x802D50B0..0x802D5DD0`).** The eight
  targets of [`fn_802d4cd0`]. All eight are 480 bytes and differ only in four
  constants; they share one documented skeleton (`build_effect_slot`) so the
  difference is readable in one place:

  | address | mode | engine size | constructor | configure | size query |
  |---|---:|---:|---|---|---|
  | `0x802D5DD0` | 1 | 0x18C | `0x8026BCF0` | `0x8026BF40` | `0x8026BDC0` |
  | `0x802D5BF0` | 2 | 0x308 | `0x8026C2F0` | `0x8026C5F0` | `0x8026C3C0` |
  | `0x802D5A10` | 3 | 0x128 | `0x8026CAE0` | `0x8026CD30` | `0x8026CBB0` |
  | `0x802D5830` | 4 | 0x230 | `0x8026D0B0` | `0x8026D3B0` | `0x8026D180` |
  | `0x802D5650` | 5 | 0x0CC | `0x8026A670` | `0x8026A880` | `0x8026A700` |
  | `0x802D5470` | 6 | 0x184 | `0x8026AB10` | `0x8026ADD0` | `0x8026ABA0` |
  | `0x802D5290` | 7 | 0x084 | `0x8026B160` | `0x8026B370` | `0x8026B1F0` |
  | `0x802D50B0` | 8 | 0x0E8 | `0x8026B640` | `0x8026B900` | `0x8026B6D0` |

  This is also where the `__savegprlr` artefact is finally explained rather than
  skipped: `FUN_802dcd70()` is an alternate entry point, so Ghidra prints its
  saved registers as a 64-bit return value and shows the real first argument as
  unused. The slot pointer is recovered from the `fn_802d4cd0` call site, which
  passes it as `r3`.
- **The sound record (`0x802D7410..0x802D7570`).** A singleton factory
  (`0x802D7410`), the builder that installs vtable `0x8035079C`, allocates
  `(count + 1) * stride + 0x2C` bytes through `0x802B8CC0` / `0x80233D10`, renames
  the voice with the default string `0x803410E4` and allocates the `0x14`-byte
  auxiliary table, and the matching destroy (`0x802D74C0`).
- **Two small helpers (`0x802DB6B0`, `0x802DB710`).** The `+0xBC` clear and the
  deleting-destructor twin that Wave 65/77 left untranslated.

### New names introduced

`AX_INIT_LATCH`, `AX_OUTPUT_HALF`, `AX_OUTPUT_HALF2`, `AX_MODE_WORD`,
`AX_MODE_WORD2`, `AX_MODE_BYTES`, `AX_STATE_HALF`, `AX_STATE_WORD`,
`AX_UP_FLAG`, `AX_DEVICE_HANDLE`, `AX_MIX_DESCRIPTOR`, `AX_MIX_DESCRIPTOR2`,
`SDA2_AUDIO_DEFAULT`, `VIEW_COMPARE_HOOK`, `VIEW_COMPARE_CONTEXT`,
`SDA2_TRIPLET`, `SOUND_RECORD`, `SOUND_VOICE`, `SOUND_TABLE`, `SOUND_COUNT`,
`SOUND_CURSOR`, `SOUND_DEFAULT_HEAP`. One local helper, `call0_word`, wraps a
zero-argument call whose `r3` the bodies need; `crate::platform` only offers the
value-returning form from one argument up. No existing constant was renamed.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802D2730` | `fn_802d2730` | 208 | medium |
| `0x802D2800` | `fn_802d2800` | 72 | high |
| `0x802D2850` | `fn_802d2850` | 316 | high |
| `0x802D2990` | `fn_802d2990` | 184 | medium |
| `0x802D2A50` | `fn_802d2a50` | 120 | high |
| `0x802D2AD0` | `fn_802d2ad0` | 136 | high |
| `0x802D2B60` | `fn_802d2b60` | 184 | high |
| `0x802D39B0` | `fn_802d39b0` | 404 | medium |
| `0x802D3B50` | `fn_802d3b50` | 392 | medium |
| `0x802D3CE0` | `fn_802d3ce0` | 276 | medium |
| `0x802D42C0` | `fn_802d42c0` | 16 | medium |
| `0x802D42D0` | `fn_802d42d0` | 240 | low |
| `0x802D4830` | `fn_802d4830` | 144 | medium |
| `0x802D50B0` | `fn_802d50b0` | 480 | medium |
| `0x802D5290` | `fn_802d5290` | 480 | medium |
| `0x802D5470` | `fn_802d5470` | 480 | medium |
| `0x802D5650` | `fn_802d5650` | 480 | medium |
| `0x802D5830` | `fn_802d5830` | 480 | medium |
| `0x802D5A10` | `fn_802d5a10` | 480 | medium |
| `0x802D5BF0` | `fn_802d5bf0` | 480 | medium |
| `0x802D5DD0` | `fn_802d5dd0` | 480 | medium |
| `0x802D7410` | `fn_802d7410` | 152 | medium |
| `0x802D74C0` | `fn_802d74c0` | 164 | high |
| `0x802D7570` | `fn_802d7570` | 264 | medium |
| `0x802DB6B0` | `fn_802db6b0` | 12 | high |
| `0x802DB710` | `fn_802db710` | 64 | medium |

## Deliberate skips

No body was written and no ledger row was created for the addresses below.

- `0x802D42D0` is claimed as `partial` / `low`: the second (reverse) loop in the
  export only re-assigns its cursor when the hook result is non-zero, which
  cannot terminate for a zero result. The body reproduces the export as printed
  and the doc block records the doubt; the forward loop is fully literal.
- `0x802D5080`, `0x802D5090` - the two 16-byte `fn_802d4cd0(slot, 1)` wrappers.
  They pass only two registers, so the builder's third argument (the resource
  word) is whatever `r5` already held; filling in a value would be a guess.
  Wave 77 reached the same conclusion and this pass did not overturn it.
- `0x802D7680`, `0x802D7860`, `0x802D79A0` - the BRRES section-table
  constructor and its two helpers. `0x802D7680` calls `FUN_802d78a0()` with both
  arguments dropped by the export, and the very next statement reads the loop
  count out of the pointer that call installs at `+0`, so the argument is not
  recoverable from this evidence. `0x802D7860`/`0x802D79A0` depend on the same
  dropped first argument.
- `0x802D0F40`, `0x802D1900`, `0x802D1E20`, `0x802D2310`, `0x802D0AE0`,
  `0x802D12E0`, `0x802D6F20`, `0x802D7070`, `0x802DADDD0`, `0x802D48C0` - these
  forward floating-point arguments in FPRs (and/or carry the `GQR0` / `ldexpf` /
  `CONCAT44` paired-single artefacts documented in `docs/QUALITY.md` §5.3).
  `crate::platform`'s `call0..call7` place integer arguments only, so a body that
  passes those floats through them would misrepresent the call.
- `0x802D2550`, `0x802D3810`, `0x802D32F0`, `0x802D3700` - their first argument
  is lost to a `__savegprlr` artefact and the body needs it before any sibling
  call site pins it down (`0x802D2550` additionally calls `0x802D3200` with all
  six float arguments dropped).
- `0x802D2610` - calls `0x801C6D90` with eight arguments; `crate::platform`
  stops at seven, and dropping the eighth would silently change the call.
- `0x802D9570`, `0x802DB750`, `0x802DB8C0`, `0x802DB970`, `0x802DB6C0` - a
  `double`-in-`F1` argument, an indirect jump printed with no arguments, or
  `CONCAT44` integer-to-double conversions; same reason as the float group.
- The `__savegprlr` / `__restgprlr` alternate-entry family
  (`0x802DCD18..0x802DCDC0`, `0x802DCD40..0x802DCDBC`), the 64-bit
  divide/modulo/shift family (`0x802DCDD8..0x802DD210`), the four-byte `b`
  thunks (`0x802D2F10`, `0x802D3260`, `0x802DB0E0`, `0x802DEBB4`) and the
  `%a`/`%f` formatter above `0x802DFF64` stay excluded exactly as Waves 65, 77
  and the module header describe.

## Verification

- Every address and byte size matches `functions.tsv` (re-read for this pass;
  `0x802D2800` is 72 bytes, not the 372 that an earlier draft of this doc
  carried - corrected before the row was written).
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/runtime/cpp.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error in `src/runtime/cpp.rs`.
- `node tools/check-lane-csv.mjs wave89` reports `wave89-cpp.csv: 26 rows` and
  `OK`.
