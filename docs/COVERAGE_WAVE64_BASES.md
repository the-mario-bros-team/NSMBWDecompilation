# Coverage - Wave 64 (`game::bases`, `0x8074` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address here is a **static analysis address** from the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader picks the real load address. Byte counts come from
`work/ghidra/rel-d_basesNP/export/functions.tsv`; the cited evidence files are
the Ghidra C exports under `work/ghidra/rel-d_basesNP/export/`. Confidence is
`high` when the guards, stores, call arguments and constants are literal and
`medium` when a callee identity, a field role or a lost argument had to be
inferred (the body then carries `// inferred:` / `// unknown:`).

The float constants quoted in the bodies were re-read from the raw words in
`work/rels/elf/d_basesNP.elf` rather than trusting the decompiler's decimal
print, which is how the `0.0`, `2.0`, `60.0`, `550.0` and `130.0` values below
were confirmed (`0x807CE08C`, `0x807CE0C8`, `0x807CE094`, `0x807CE098`,
`0x807CE09C`). The two BRRES strings were read from the image too:
`0x8080FF4C` is `g3d/W7_shiroboss_bg_U.brres` and `0x8080FF68` is
`W7_shiroboss_bg_U`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 29 | 3792 | 21 | 8 |

## What this pass covers

- **Multi-player marker actor, `0x80740160..0x807419C0`** — the object whose
  four 16-byte player-slot records sit at `+0x394` (index or `-1`, active byte,
  two floats) and whose effect records are 0x20-byte entries at `+0x480` and
  `+0x514` (eight halfwords, one word, two floats, an active byte at `+0x1C`
  and a slot index at `+0x1D`). New bodies: the slot-word builder
  [`fn_80740160`], the rebuild driver [`fn_807401c0`] (five reset helpers, then
  collect / sort / smooth), the collect loop [`fn_80740250`], the bubble sort
  on `+0x14` [`fn_80740290`], the neighbour-gap smoother
  [`fn_807402f0`] (`>> 12`), the stepper/clamp [`fn_807405a0`] (`0x200` rate
  limit, stepper `+0x43C`, clamp `+0x44A`), the four-record initialiser
  [`fn_80740c70`], the sixteen-record colour reset [`fn_80740e20`] (via
  `0x800DF860`), the stale-slot prune [`fn_80740ed0`], the `60 + 550*x/130`
  timer curve [`fn_80740f20`], the previous-frame copy [`fn_80741280`], the
  `2.0f` record stepper with its `+0x4D0` snapshot [`fn_807412a0`], the
  covered-record retire loop [`fn_80741380`], the `+0x4C4` slot words
  [`fn_80741910`] and the frame pipeline [`fn_807419c0`].
- **Marker actor tail, `0x80741A30..0x80742800`** — the second collect loop
  [`fn_80741a30`] over `+0x514..+0x714`, the transform writer
  [`fn_80742010`] (position `+0xAC..+0xB4` and angles `+0x100..+0x104` into the
  `+0x7C` matrix, copied into `+0x450`, call order Y, X, Z preserved), the
  `0x807CE100` table lookup [`fn_807422f0`], the variant bit-mask builder
  [`fn_80742340`] (with the `4 -> 0xF` fast path and the eight-try cap), the
  per-player spawn [`fn_807423e0`] (types `0x9F`/`0xA1` through `0x80064680`,
  handle stored at `+0x3D0 + slot*4`), the all-dead gate [`fn_80742600`] and
  the release wrapper [`fn_80742800`].
- **`W7_shiroboss_bg` course object, `0x80742980..0x80743260`** — the
  `0xAA0`-byte object: allocation [`fn_80742980`], the constant record getter
  [`fn_807429b0`] (`0x80377A04`), the constructor [`fn_807429c0`] (vtable
  `0x8080FF80`, accessor `+0x524`, model `+0x544`, four `0x5C`-byte controllers
  at `+0x584`, four `0xE4`-byte records at `+0x6F4`), the
  `g3d/W7_shiroboss_bg_U.brres` loader [`fn_80742b90`], the array teardown
  [`fn_80742ec0`] (interface slot `+0x10`, vtable slot `+0x294`, four `0x5C`
  and four `0xE4` releases) and the transform writer [`fn_80742f70`], plus the
  second destructor twin [`fn_80743260`].

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80740160` | `fn_80740160` | 92 | high |
| `0x807401C0` | `fn_807401c0` | 144 | high |
| `0x80740250` | `fn_80740250` | 56 | high |
| `0x80740290` | `fn_80740290` | 92 | high |
| `0x807402F0` | `fn_807402f0` | 128 | high |
| `0x807405A0` | `fn_807405a0` | 244 | medium |
| `0x80740C70` | `fn_80740c70` | 32 | high |
| `0x80740E20` | `fn_80740e20` | 168 | high |
| `0x80740ED0` | `fn_80740ed0` | 76 | medium |
| `0x80740F20` | `fn_80740f20` | 236 | high |
| `0x80741280` | `fn_80741280` | 32 | high |
| `0x807412A0` | `fn_807412a0` | 220 | high |
| `0x80741380` | `fn_80741380` | 396 | medium |
| `0x80741910` | `fn_80741910` | 176 | high |
| `0x807419C0` | `fn_807419c0` | 108 | medium |
| `0x80741A30` | `fn_80741a30` | 56 | high |
| `0x80742010` | `fn_80742010` | 208 | high |
| `0x807422F0` | `fn_807422f0` | 76 | medium |
| `0x80742340` | `fn_80742340` | 160 | medium |
| `0x807423E0` | `fn_807423e0` | 132 | high |
| `0x80742600` | `fn_80742600` | 140 | medium |
| `0x80742800` | `fn_80742800` | 88 | high |
| `0x80742980` | `fn_80742980` | 48 | high |
| `0x807429B0` | `fn_807429b0` | 12 | medium |
| `0x807429C0` | `fn_807429c0` | 148 | high |
| `0x80742B90` | `fn_80742b90` | 172 | high |
| `0x80742EC0` | `fn_80742ec0` | 176 | high |
| `0x80742F70` | `fn_80742f70` | 84 | high |
| `0x80743260` | `fn_80743260` | 92 | high |

## Address range

`0x80740160..0x80743260`, two contiguous runs. The first run
(`0x80740160..0x807405A0`, `0x80740C70..0x80740ED0`, `0x80740F20`,
`0x80741280..0x80741380`, `0x80741910..0x80742010`, `0x807422F0..0x80742800`)
is only interrupted by the deliberately skipped bodies `0x80740370`,
`0x80740C90`, `0x80741010`, `0x80741510`, `0x80741A70`, `0x807422D0` and
`0x80742470`; the second run (`0x80742980..0x80743260`) is contiguous.

## Deliberate skips

- `0x80740370` (552), `0x80741A70` (552) and `0x80741510` (1016) — all three
  open with the outlined paired-single `GQR`/`ldexpf` conversion block
  (`unaff_GQR0`, `in_f29`/`in_f30`/`in_f31`), exactly the failure class already
  recorded for `0x806F0F20` and the tail of [`fn_806f0c10`]. The surviving
  arithmetic is a 16-step angle interpolation, but the input registers are
  invented by the decompiler, so no honest body is possible.
- `0x80740C90` (388) and `0x80742470` (364) — both recover `this` through
  `0x802DCD70`/`0x802DCD58` as a 64-bit value whose halves Ghidra then treats
  as two different objects (the low half as a record array, the high half as
  the owner). Splitting that value into two Rust objects would be a guess.
- `0x80741010` (612) — the record feeder. Its `+0x514` halfword is converted
  with the `CONCAT44(0x43300000, x) ^ 0x80000000` magic and compared against
  the `176.0 ± 3.25` window; the surrounding four-slot rewrite is readable, but
  the converted value's sign convention is not, so the body is left for a later
  pass. [`fn_807419c0`] still calls it by address with an `// unknown:` note.
- `0x807422D0` (208) — the Ghidra export is truncated by bad instruction data
  (`halt_baddata()`), so no control flow survives.
- Adjustor thunks `0x802DCBFC(p2, p2, p1)` — `0x80742920` and `0x80742950`.
  They only re-enter the same indirect-jump stub and have no Rust-visible
  effect, matching the policy already documented in the module header.

At lane start the `0x8074` bucket held 233 functions, 66 of them already
translated. This pass adds 29, leaving 138 for later waves.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (re-read, not copied from
  prose).
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000` line.
- No address was already in `ledger/functions.csv` (checked against the whole
  ledger before writing).
- `node tools/check-lane-csv.mjs wave64` reports `OK` and
  `wave64-bases.csv: 29 rows`.
- `cargo check --message-format=short` reports no error or warning for
  `src/game/bases.rs`.
