# Coverage - Wave 92 (runtime::cpp)

## How to read this file

Each row is one `pub (unsafe) fn fn_<address>` body in `src/runtime/cpp.rs` that
this pass vouched for. The `bytes` column is the exact size from
`work/ghidra/main-gb/export/functions.tsv`; the body's `# Original` block cites
the same export file. Confidence matches the CSV row. Field offsets are relative
to the first argument. SDA cells are given as absolute addresses, resolved with
`r13 = 0x8042F680` and `r2 = 0x80433080` (the values Waves 77 and 89 used).

Twelve bodies are new in this pass. The other eight bodies already existed in
`src/runtime/cpp.rs` from earlier runtime passes but had never been ledgered
(Wave 89 recorded that gap); this pass re-checked each against its export and
its `functions.tsv` size and added the missing rows.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/cpp.rs` | `runtime::cpp` | 20 | 3788 | 14 | 6 |

Twelve new bodies account for 2452 bytes (6 high, 6 medium); the eight
pre-existing bodies account for 1336 bytes (all high).

## What this pass covers

Address range: `0x802D0AE0..0x802DF42C` (with the remaining gaps listed under
*Deliberate skips*).

- **The layout table (`0x802D0AE0`).** Copies two `u16` panel dimension pairs
  into the 0x18-byte-stride table at `0x803504E0`, derives four scale ratios
  from the outer size at `0x8042AF38`/`0x8042AF3A`, runs the one-time draw-record
  initialiser (`0x802D0CC0`), fills the record at `0x80424710`, and publishes the
  caller's record at `0x8042AF0C` before calling `0x802D1AA0(index)`.
- **The 0x802D32F0/0x802D3700 pair.** `0x802D32F0` runs the base initialiser
  `0x802C0240(this)`, copies the four widget attribute bytes from `r13-0x4734`
  and sets the `+0x0A..+0x0D` state bytes plus the `+0x18` dirty bit.
  `0x802D3700` pops a node from `0x802D3590`, stores width/height/format at
  `+4`/`+6`/`+9`, runs the node's virtual `+0x0C` slot, allocates the GX texture
  with `0x801C6BA0(width, height, format, 0, 1)` and passes it to
  `fn_802d35c0`.
- **The `+0x4C4` view constructor (`0x802D3810`).** Finishes the family that
  Waves 65/77/89 only ever touched through loaders: it installs vtable
  `0x80350538`, constructs the three sub-objects at `+0xC` (`0x80268FF0`),
  `+0x198` (`0x8026F690`) and `+0x374` (`0x8026DD60`), stores the two child
  pointers at `+0x4C4`/`+0x4C8`, and publishes `state = 0`, `ready = 1` at
  `+0x4CC`/`+0x4D0`. The call site `0x802D6210` proves the three pointer
  arguments.
- **The two `fn_802d4cd0` wrappers (`0x802D5080`, `0x802D5090`).** These are
  16-byte `slwi`/`add`/`b` sequences. Wave 89 skipped them because the export
  shows only two arguments; the raw body never writes `r5`, so the wrapper's own
  third argument is forwarded to [`fn_802d4cd0`] as its `argument`. They are now
  translated with that third parameter.
- **The console/viewer loop (`0x802D71F0`).** A key-sequence wait against the
  `u16` list, a full view-range scan through `0x802345C0`/`0x802340A0`, then an
  endless pad loop that toggles the console byte at `0x8042AFC0` on the `0x8000`
  input bit, optionally invokes the `0x8042AFBC` callback, and moves the cursor
  by one or five with the exported clamps (`-150..=10`).
- **The BRRES section constructor (`0x802D7680`).** Installs vtable
  `0x803507A8`, stores the caller's table at `+0` through `0x802D78A0`, then
  walks `table + 0x20` for `table[+0xC]` entries. Each tag goes through
  `0x802D7910` and the six setters `0x802D78B0..0x802D7900` write the matching
  field at `+4..+0x18`. The register trace at `0x802D76D0` proves `r3`/`r4` still
  hold `this`/`table`, so the empty argument list in the export is only a
  signature omission (Wave 89 had judged the arguments unrecoverable).
- **Three `lyt` state helpers (`0x802DB6C0`, `0x802DB750`, `0x802DB8C0`,
  `0x802DB970`).** A constructor that installs vtable `0x803509E0` and calls its
  `+0x24` slot; a one-shot phase step that copies `0x802D87C0(object)[+0x28]`
  into `+0x10` and keeps the fractional part of `+0xE4 * scale` at `+0xC`; a
  phase dispatcher that either tail-calls `vtable + 0x88` or copies three floats
  to `+0x14..+0x1C`; and a timer that adds the source delta, wraps at `1.0f`
  and calls `0x80287C10(source, source[+0xB8], this + 0x20, 0, 0, this + 0x14)`.
- **Eight previously unledgered bodies.** `0x802DCC4C` (saturated `double` to
  `u32`), `0x802DF42C` (Metrowerks `__fpclassify`) and the six 64-bit helpers
  `0x802DCDD8..0x802DD210` (unsigned/signed divide and remainder, shift left,
  logical shift right). Each was re-read against its export and `functions.tsv`
  before the row was added; their doc blocks are unchanged apart from the
  `Evidence:` label.

### New names introduced

`WIDGET_ATTRIBUTES`, `PANEL_MAX_X`, `PANEL_MAX_Y`, `DRAW_RECORD_INITIALISED`,
`LAYOUT_INDEX`, `LAYOUT_ROOT`, `LAYOUT_TABLE`, `DRAW_RECORD`, `CONSOLE_FLAG`,
`CONSOLE_CALLBACK`, `TIME_THRESHOLD`. No existing constant was renamed.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802D0AE0` | `fn_802d0ae0` | 476 | medium |
| `0x802D32F0` | `fn_802d32f0` | 116 | high |
| `0x802D3700` | `fn_802d3700` | 164 | medium |
| `0x802D3810` | `fn_802d3810` | 136 | high |
| `0x802D5080` | `fn_802d5080` | 16 | high |
| `0x802D5090` | `fn_802d5090` | 16 | high |
| `0x802D71F0` | `fn_802d71f0` | 532 | medium |
| `0x802D7680` | `fn_802d7680` | 344 | medium |
| `0x802DB6C0` | `fn_802db6c0` | 72 | high |
| `0x802DB750` | `fn_802db750` | 276 | medium |
| `0x802DB8C0` | `fn_802db8c0` | 68 | high |
| `0x802DB970` | `fn_802db970` | 236 | medium |
| `0x802DCC4C` | `fn_802dcc4c` | 92 | high |
| `0x802DCDD8` | `fn_802dcdd8` | 236 | high |
| `0x802DCEC4` | `fn_802dcec4` | 312 | high |
| `0x802DCFFC` | `fn_802dcffc` | 228 | high |
| `0x802DD0E0` | `fn_802dd0e0` | 268 | high |
| `0x802DD1EC` | `fn_802dd1ec` | 36 | high |
| `0x802DD210` | `fn_802dd210` | 40 | high |
| `0x802DF42C` | `fn_802df42c` | 124 | high |

## Deliberate skips

No body and no ledger row was created for the addresses below. They remain the
only untranslated entries in the `0x802D0000` bucket.

- `0x802D0030` - EGG render setup with `ldexpf`/`GQR0` artefacts and two `double`
  arguments in FPRs; out of the CRT scope this module documents.
- `0x802D0D90`, `0x802D0F40`, `0x802D1010`, `0x802D12E0`, `0x802D1850`,
  `0x802D1900`, `0x802D1E20`, `0x802D2310` - forward `double`/`float` values in
  FPRs or carry the paired-single `GQR0`/`CONCAT44` artefacts described in
  `docs/QUALITY.md` §5.3. `crate::platform` has no floating-point argument
  helpers, so a body would misrepresent the call.
- `0x802D2550`, `0x802D2610` - `0x802D2550` drops all six float arguments of
  `0x802D3200`; `0x802D2610` calls `0x801C6D90` with eight arguments and
  `platform` stops at seven.
- `0x802D48C0`, `0x802D6F20`, `0x802D7070`, `0x802D9570`, `0x802DADD0` - same
  floating-point/FPR reason, plus the `__savegprlr` return artefact.
- `0x802D7860`, `0x802D79A0` - `0x802D79A0` depends on the `__savegprlr`
  pseudo-return for its object pointer and on `r4`/`r5` that its three call
  sites do not set up; `0x802D7860` is its wrapper and inherits the gap.
- `0x802D2F10`, `0x802D3260`, `0x802DB0E0`, `0x802DEBB4` - 4-byte `b` thunks.
- `0x802DCBFC` - indirect jump through `r12` with an unrecovered jump table.
- `0x802DCCCC`, `0x802DCCE0`, `0x802DCD18`, `0x802DCD2C`, `0x802DCD40..0x802DCDBC`,
  `0x802DCDC0` - the `__savegprlr` / `__restgprlr` alternate-entry family and
  the empty bodies that only exist to spill or restore registers.
- `0x802DFF64` and above - the `%a`/`%f` formatter, whose C contains
  `ldexpf`/`unaff_GQR0` artefacts.

## Verification

- Every address and byte size was re-read from `functions.tsv` for this pass.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every one of the 20 rows has exactly one body in `src/runtime/cpp.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error in `src/runtime/cpp.rs`.
- `node tools/check-lane-csv.mjs wave92` reports `wave92-cpp.csv: 20 rows` and
  `OK`.
