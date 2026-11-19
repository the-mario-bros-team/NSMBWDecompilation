# Coverage - Wave 138 (`game::bases`)

## How to read this file

This page documents the Wave 138 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`**, produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
`# Original` block in the source carries the
`Synthetic base: 0x80600000; this is not a runtime address.` line.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee contract, a field role or a `qr3`
quantiser descriptor had to be inferred. Comments in the bodies keep
`// observed:` / `// inferred:` / `// unknown:` markers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 26 | 4,992 | 18 | 8 |

All 26 rows are `translated`; none is `partial`. The module header now reports
3518 bodies (3492 + 26).

## What this pass covers

- **The three class-C addresses of `docs/SPEC_WAVE137_BASES.md`.** These are the
  first `qr3`-dependent bodies translated in this module. Their
  `psq_l ...,qr3,w1` loads convert a signed 16-bit halfword and the stream
  multiplies every one of them by `1/256` (`0x807CAB68`, `0x807C96AC`,
  `0x807CB9E0`), so the bodies write the conversion as `(s16)half / 256`. That
  multiplier pins the descriptor `main.dol` must install to S16/scale 8
  (`GQR3.load = 0x0807`); no `mtspr GQRn` exists anywhere in `d_basesNP`, which
  is why the rows are `medium` rather than `high`.

  | cluster | bodies |
  |---|---|
  | `wendy_ring` initialiser | `0x806B7CD0` (1,704 B: frame-rate cell, five slot remainders, 120 pose slots, ring direction at `+0xE8`, `+0x0A4`-stride effect records, twelve counters) |
  | 348-step pose select | `0x806DD730` (736 B: modulo-348 dispatch over eight zero-fill descriptors, countdown byte, `fn_806DDA10` result to `+0x710`) |
  | mount-transform groups | `0x80680720` (660 B: two translate/mount groups on `this+0x7C` for `actor` and `actor+0x40`) |

- **Twenty-three `0x8063` planner-bucket bodies (1,892 bytes)**, taking that
  bucket from 86 to 63 untranslated addresses:

  | cluster | bodies |
  |---|---|
  | registered destructors, plain release | `0x806369D0`, `0x80636A10`, `0x8063A1D0`, `0x8063A210`, `0x8063B420`, `0x8063B460`, `0x8063E870`, `0x8063E8B0` |
  | registered destructors, `+0x18` clear first | `0x80636A50`, `0x80636AB0`, `0x8063A250`, `0x8063A2B0`, `0x8063B4A0`, `0x8063B500`, `0x8063E8F0`, `0x8063E950` |
  | animation finish-notifiers | `0x80638050`, `0x8063EE90`, `0x8063EF10`, `0x8063F4E0` |
  | animation-slot clears | `0x80638000`, `0x8063EE20` |
  | global teardown pair | `0x8063D3B0` |

## Where the specification needed adjusting

1. **Block T of `0x806B7CD0` uses `half`, not `this[0x1048]`, for the flag
   byte.** The specification's review line writes
   `this[0x348] = ((((u32)(n - 0x4000)) & 0xFFFF) <= 0x7FFF)`. The stream is
   `addi r0,r25,-16384` / `clrlwi 16` / `cmplwi 0x7fff` / `cntlzw`, where `r25`
   is `extsh` of the halfword that went through the `qr3` conversion, i.e.
   `(u16)(half - 0x4000) <= 0x7FFF`. It is written that way.
2. **The same halfword is `(n << 13) as u16`, not `n >> 3`.** Word
   `0x54006824` is `rlwinm r0,r0,13,0,18`, which is exactly `slwi r0,r0,13`
   (Ghidra also prints `*(int *)(iVar10 + 0x1048) << 0xd`), so only the low
   three bits of `+0x1048` reach the `sth`. Both `qr3` conversions therefore
   read the same halfword, and the two `336.0`-scaled trig results are dead
   before the `cos`/`sin` pair is stored.
3. **Block Y of `0x806B7CD0` runs `fn_806B7690` on *any* non-zero probe.** The
   specification reads the last test as `q3 == 0`. The stream's third branch is
   `bc 12,2,0x806b8348`, i.e. `beq` skipping the call, so the call runs when
   `q1 != 0 || q2 != 0 || q3 != 0`. The branches are transcribed, not the
   prose.
4. **Blocks F and G of `0x80680720` both address `this+0x7C`.** Only the three
   `0x8016A1xx` binders and the `+0x14` virtual move to `actor+0x40`; the mount
   setters keep writing the emitter transform, and the second `0x801C0BD0`
   call is an exact repeat of the first (its `336.0`/`328.0` trig scalars are
   dead).
5. **`0x8063D3B0` takes no parameters.** Ghidra declares it `void f(void)` and
   the stream loads `r3` from `0x8042A040` before both calls, so the body reads
   the `main.dol` cell once instead of taking an argument.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806369D0` | `fn_806369d0` | 64 | high |
| `0x80636A10` | `fn_80636a10` | 64 | high |
| `0x80636A50` | `fn_80636a50` | 96 | high |
| `0x80636AB0` | `fn_80636ab0` | 100 | high |
| `0x80638000` | `fn_80638000` | 80 | high |
| `0x80638050` | `fn_80638050` | 88 | medium |
| `0x8063A1D0` | `fn_8063a1d0` | 64 | high |
| `0x8063A210` | `fn_8063a210` | 64 | high |
| `0x8063A250` | `fn_8063a250` | 96 | high |
| `0x8063A2B0` | `fn_8063a2b0` | 100 | high |
| `0x8063B420` | `fn_8063b420` | 64 | high |
| `0x8063B460` | `fn_8063b460` | 64 | high |
| `0x8063B4A0` | `fn_8063b4a0` | 96 | high |
| `0x8063B500` | `fn_8063b500` | 100 | high |
| `0x8063D3B0` | `fn_8063d3b0` | 60 | medium |
| `0x8063E870` | `fn_8063e870` | 64 | high |
| `0x8063E8B0` | `fn_8063e8b0` | 64 | high |
| `0x8063E8F0` | `fn_8063e8f0` | 96 | high |
| `0x8063E950` | `fn_8063e950` | 100 | high |
| `0x8063EE20` | `fn_8063ee20` | 104 | high |
| `0x8063EE90` | `fn_8063ee90` | 88 | medium |
| `0x8063EF10` | `fn_8063ef10` | 88 | medium |
| `0x8063F4E0` | `fn_8063f4e0` | 88 | medium |
| `0x80680720` | `fn_80680720` | 660 | medium |
| `0x806B7CD0` | `fn_806b7cd0` | 1704 | medium |
| `0x806DD730` | `fn_806dd730` | 736 | medium |

**Totals:** 26 bodies, 4,992 bytes (18 high, 8 medium). The address window runs
from `0x806369D0` to `0x806DD730`.

## Notes on inference

- [`fn_806b7cd0`] is `medium` only for block T: the two `qr3` conversions. The
  `1/256` compensation makes `(s16)half / 256` exact if `GQR3.load` is `0x0807`;
  the nine `0x43300000` ladder fields are written as `raw as i32 / 4096.0 *
  4096.0 * 0.75` with `fctiwz` truncation, which is what `xoris 0x8000` plus the
  `2^52 + 2^31` magic double at `0x807CAB70`/`0x807CAB74` produce.
- [`fn_806dd730`] is `medium` for the two `qr3` cases and for the `% 348` magic
  multiplier `0x2F159903`, which is trusted as the unsigned remainder the wave-137
  spec lane verified numerically. Six of the eight cases need no `qr3` at all.
- [`fn_80680720`] is `medium` for its four `qr3` loads and because the meaning of
  `0x801C0BD0` (three floats plus a destination pointer) is inferred from its
  call shape; the three mount setters and three binders are called with literal
  pointers and literal halfword arguments.
- [`fn_80638050`], [`fn_8063ee90`], [`fn_8063ef10`] and [`fn_8063f4e0`] are
  `medium`: the comparison helper `0x800C9610` is named from its
  `(handle, tag)` shape and zero/non-zero result use, and the descriptors
  `0x80826B60` / `0x80827520` / `0x80827560` / `0x80827660` live in this REL's
  zero-fill segment, so they are cited as addresses only.
- [`fn_8063d3b0`] is `medium`: `0x8042A040` is `main.dol` data and both callees
  are unnamed `main.dol` routines.
- The sixteen destructors and [`fn_80638000`] / [`fn_8063ee20`] are `high`: every
  guard, offset, loop bound and call target is literal, and each address is
  confirmed as a table word in the `0x807D`/`0x807E` vtable region (the slot is
  named in each body).

## Deliberate skips

Read and rejected, with the reason:

- **The fourteen 48-byte `0x802DCBFC` forwarders in this bucket**
  (`0x806318F0`, `0x80631920`, `0x80634C90`, `0x80634CC0`, `0x806360D0`,
  `0x80636100`, `0x80639390`, `0x806393C0`, `0x8063AEB0`, `0x8063AEE0`,
  `0x8063D740`, `0x8063D770`, `0x8063F7F0`, `0x8063F820`).
  Ghidra renders each as `func_0x802dcbfc(param_2, param_2, param_1)` after a
  hidden `r12 = r3 + imm`; the visible argument words do not survive, so a
  `platform::call3` rendering would invent two of them. Wave 137 rejected the
  same family for the same reason.
- **`0x8063D260` (72 bytes).** Its first call is `FUN_8063c090()` with no visible
  arguments although `r3` holds `param_1` at that point; the same
  argument-dropping shape as the stubs above, and `0x8063C090` is itself
  untranslated. Left for an address-for-address pass.
- **The rest of the `0x8063` bucket** (63 addresses, up to 972 bytes) is
  untouched: the large ones (`0x80630820`, `0x8063A960`, `0x80638120`) need their
  own call graphs.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (26 rows, 4,992 bytes),
  re-read from the TSV for this pass rather than from prose. Every body's last
  word was re-read from `work/rels/elf/d_basesNP.elf` at
  `(address - 0x80600000) + 0x1000`; all 26 end in `0x4E800020` (`blr`) at
  `entry + size - 4`.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 26 were checked against
  the ledger's address column before being written, and none is duplicated.
- `node tools/check-lane-csv.mjs wave138` reports `OK`: `wave138-bases.csv` has
  26 rows, 26 unique addresses, 18 `high` and 8 `medium`.
- `cargo check --message-format=short` exits 0 and reports no error and no
  warning naming `src/game/bases.rs`.
