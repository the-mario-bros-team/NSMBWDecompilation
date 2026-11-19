# Coverage - Wave 139 (`game::bases`)

## How to read this file

This page documents the Wave 139 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`**, produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
`# Original` block in the source carries the
`Synthetic base: 0x80600000; this is not a runtime address.` line.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee contract, a field role or an import
relocation had to be named rather than resolved. Comments in the bodies keep
`// observed:` / `// inferred:` / `// unknown:` markers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 5,872 | 25 | 5 |

All 30 rows are `translated`; none is `partial`. The module header now reports
3548 bodies (3518 + 30).

## What this pass covers

- **The nine ready class-C `qr3` bodies of `docs/SPEC_WAVE138_BASES.md`**
  (3,864 bytes). Every `psq_l ...,qr3,w1` load is written as
  `read_u16(..) as f32 * 1/256` because `main.dol` installs
  `GQR3 = 0x00050005` (type 5 = `u16`, scale 0) at `0x8025EEEC`; no `mtspr GQRn`
  exists anywhere in `d_basesNP`.

  | cluster | bodies |
  |---|---|
  | gated four-slot pivots | `0x80681C50` (node walk + four slot sines), `0x8068EA10` (`160.0 * +0xC4C * sin * 0.5`) |
  | bob oscillation twins | `0x80688BA0` (`+0x564 += 0x200`), `0x80688E10` (`-= 0x200`, mirror) |
  | limp timer / bob | `0x806B2BB0` (`+0x506` angle, `1.0 + 0.5*sin`) |
  | rotating actor | `0x806B9A30` (`+0xE8`/`+0xEC` from `+0x47E`) |
  | `0x8072` emitter transforms | `0x8072BC40` (two mounts), `0x8072C000` (radial records), `0x8072E520` (four-arm burst) |

- **Twenty-one `0x8071` planner-bucket bodies (2,008 bytes)**, taking that
  bucket from 85 to 64 untranslated addresses:

  | cluster | bodies |
  |---|---|
  | four-slot sound-channel records at `+0x420` | `0x8071BFC0` (clear), `0x8071BF20` (start), `0x8071BF70` (stop), `0x8071BFE0` (two-attempt state machine) |
  | channel steppers | `0x8071C2B0`, `0x8071C300`, `0x8071C380`, `0x8071C6A0` |
  | transform refresh | `0x8071C620`, `0x8071F050` |
  | four 0x60-byte record loops at `+0x434` | `0x8071F290`, `0x8071F2E0`, `0x8071F340` |
  | screen/actor plumbing | `0x80717C10`, `0x80717C80`, `0x8071B730` |
  | environment-mode installers | `0x80718120` (mode 3), `0x80718330` (mode 1), `0x80718540` (mode 0), `0x80718750` (mode 2) |
  | per-frame reset | `0x8071FAA0` |

## Where the binary differed from the spec prose

1. **`GQR3` is `u16`/scale 0, and the wave-138 `(s16)` bodies are still
   numerically correct.** The three class-C bodies already landed in wave 138
   convert with `(half as i16) as f32 * (1/256)`. Because the helpers at
   `0x80237BD0`/`0x80237C40` have a 256-step period, reading the same halfword
   as `u16` differs by exactly `256.0` in the argument and therefore produces
   the identical sine and cosine. They were left untouched; the nine bodies in
   this pass use the `u16` form the spec pins.
2. **`0x8072BC40` also calls `0x80064AF0`.** The spec's prose starts at the
   `0x801C0BD0` matrix call, but the stream first passes a three-float position
   and a three-halfword angle triple to `0x80064AF0(&pos, &angle3, 1)` in place,
   and again before the second mount with a null angle pointer. Both calls are
   transcribed.
3. **`0x806B2BB0` clears `+0x509` before reloading the angle.** The stream
   writes the byte, stores `0x800` into `+0x506` and reuses `0x800` as the sine
   input in the same iteration; the Rust keeps that single-iteration reuse
   instead of re-reading the halfword.
4. **`0x806B9A30` reads `0x80429CA8` with `lbz`, not `lwz`.** The bitmask is one
   byte in `main.dol`; Ghidra declares it `uint` only because of the
   zero-extension. The body reads a `u8` and widens it.
5. **`0x8071BFE0` ends one word outside its `functions.tsv` extent.** The size is
   `228`, so the extent stops at `0x8071C0C0`; the `blr` is at `0x8071C0C4` and
   the next function starts at `0x8071C0D0`. The `# Original` block cites the TSV
   size verbatim as required.
6. **`0x8071BF70` has three parameters even though Ghidra prints two.**
   `r5` is never touched before `0x800B3750(channel, 10, arg3)` and is forwarded
   as the callee's third argument; `0x8071C380` and `0x8071BFE0` both pass
   `0x3C` there. The third parameter is explicit so the value is not invented.
7. **`0x8071C6A0` calls `0x8071C380` with `this`.** Ghidra prints the call with
   no arguments, but `r3` still holds `r3` from entry at the `bl` site, so the
   translated body passes `this`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80681C50` | `fn_80681c50` | 384 | medium |
| `0x80688BA0` | `fn_80688ba0` | 356 | high |
| `0x80688E10` | `fn_80688e10` | 356 | high |
| `0x8068EA10` | `fn_8068ea10` | 332 | medium |
| `0x806B2BB0` | `fn_806b2bb0` | 340 | high |
| `0x806B9A30` | `fn_806b9a30` | 476 | medium |
| `0x80717C10` | `fn_80717c10` | 104 | high |
| `0x80717C80` | `fn_80717c80` | 104 | high |
| `0x80718120` | `fn_80718120` | 108 | high |
| `0x80718330` | `fn_80718330` | 108 | high |
| `0x80718540` | `fn_80718540` | 108 | high |
| `0x80718750` | `fn_80718750` | 108 | high |
| `0x8071B730` | `fn_8071b730` | 80 | high |
| `0x8071BF20` | `fn_8071bf20` | 72 | high |
| `0x8071BF70` | `fn_8071bf70` | 72 | high |
| `0x8071BFC0` | `fn_8071bfc0` | 24 | high |
| `0x8071BFE0` | `fn_8071bfe0` | 228 | high |
| `0x8071C2B0` | `fn_8071c2b0` | 72 | high |
| `0x8071C300` | `fn_8071c300` | 120 | high |
| `0x8071C380` | `fn_8071c380` | 84 | high |
| `0x8071C620` | `fn_8071c620` | 128 | high |
| `0x8071C6A0` | `fn_8071c6a0` | 56 | high |
| `0x8071F050` | `fn_8071f050` | 96 | high |
| `0x8071F290` | `fn_8071f290` | 72 | high |
| `0x8071F2E0` | `fn_8071f2e0` | 84 | high |
| `0x8071F340` | `fn_8071f340` | 72 | high |
| `0x8071FAA0` | `fn_8071faa0` | 108 | high |
| `0x8072BC40` | `fn_8072bc40` | 604 | high |
| `0x8072C000` | `fn_8072c000` | 372 | medium |
| `0x8072E520` | `fn_8072e520` | 644 | medium |

**Totals:** 30 bodies, 5,872 bytes (25 high, 5 medium). The address window runs
from `0x80681C50` to `0x8072E520`.

## Notes on inference

- [`fn_80681c50`] and [`fn_8068ea10`] are `medium`: the `+0x6C` virtual's return
  byte is only inferred to be a slot index from this function's own use, and the
  `+0xC60`/`+0xC64` arrays are inferred from their copy/clear pattern.
- [`fn_806b9a30`] is `medium`: `0x80C3DFC0` and `0x80C3DFB0` are import
  relocations outside every shipped image segment and `0x80429CA8` is a
  `main.dol` bitmask; the body is fully specified, the two callee contracts are
  named, not resolved.
- [`fn_8072c000`] is `medium`: the eight-entry step table at `0x807CD4C0` is
  read as **signed integers** through the `xoris 0x8000` ladder, and its last
  word is the `19.0` float cell; that oddity is transcribed literally.
- [`fn_8072e520`] is `medium`: both effect ids (`3`/`5` at `0x807CD5E8`/
  `0x807CD5F8`), the mask table `0x807CD608` (`07 05 0D 09`) and the `"Wm_en_burst_ss"`
  spawn are read from the image, but the engine meaning of the id and mask bits
  is inferred.
- The remaining 25 rows are `high`: every guard, offset, constant and call
  target is literal, and the four environment-mode installers are byte-identical
  in shape apart from the mode argument and the float block.

## Deliberate skips

Read and rejected, with the reason:

- **The ten 48-byte `0x802DCBFC` forwarders in this bucket** (`0x80711E30`,
  `0x80711E60`, `0x807157B0`, `0x807157E0`, `0x80717A80`, `0x80717AB0`,
  `0x80719CA0`, `0x80719CD0`, `0x8071EBC0`, `0x8071EBF0`). Ghidra renders each
  as `func_0x802dcbfc(param_2, param_2, param_1)` after a hidden `r12 = r3 + imm`;
  the visible argument words do not survive, so any `platform::call3` rendering
  would invent two of them. Wave 137 and wave 138 rejected the same family.
- **`0x807108B0` (2,468 bytes), `0x8071D500` (1,748), `0x8071E130` (788),
  `0x8071DCC0` (744), `0x807195A0` (512)** and the rest of the `0x8071` bucket
  above `0x8071FAA0`: each needs its own call graph and was left for a dedicated
  pass rather than guessed at the tail of this one.
- **`0x8071F940` (12 bytes) and `0x8071F7B0` (36 bytes)** were left because the
  first is a bare data-pointer getter and the second forwards to
  [`fn_8071f340`] with no register evidence for its argument.
- **`0x806B7CD0`, `0x806DD730` and `0x80680720`** (the three class-C rows of
  wave 137/138) are already in `ledger/functions.csv` and were not re-claimed.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 5,872 bytes),
  re-read from the TSV for this pass rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 30 were checked against
  the ledger's address column before being written, and none is duplicated.
- Each body's last word was re-decoded from `work/rels/elf/d_basesNP.elf` at
  `(address - 0x80600000) + 0x1000`; 29 of 30 end in `0x4E800020` (`blr`) at
  `entry + size - 4`, and `0x8071BFE0`'s `blr` sits one word past its TSV extent
  (see item 5 above).
- `node tools/check-lane-csv.mjs wave139` reports `OK` for
  `wave139-bases.csv`: 30 rows, 30 unique addresses, 25 `high` and 5 `medium`.
- `cargo check --message-format=short` finishes with no error and no warning
  naming `src/game/bases.rs`.
