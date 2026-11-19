# Coverage - Wave 97 (`runtime::bt`, EXI console tail and the render/palette objects)

## How to read this file

Every row below is one `pub unsafe fn fn_<address>` body appended to
`src/runtime/bt.rs`. The address is the unrelocated `main.dol` virtual address
(image base `0x80004000`); the byte count was re-read from
`work/ghidra/main-gb/export/functions.tsv` for this pass; the `evidence` column
of `ledger/agents/wave97-bt.csv` names the Ghidra C export. All addresses are in
the `0x80220000` bucket, which holds 216 functions and had 58 untranslated
entries when this pass started. The pass claims 26 of them.

The bucket is not one subsystem. Waves 42/47/60/69/84/92 used the same bucket
for the IPC/EXI service, the SDP database and three small vtable classes; this
pass fills four holes that were left between those clusters and touches no
Bluetooth protocol code.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 26 | 7244 | 13 | 12 | 1 |

## What this pass covers

### EXI debug-console tail (`0x80227CC0..0x8022892C`, 5 bodies)

| Address | Role |
|---|---|
| `0x80227CC0` | rewrites `\n` to `\r`, then sends the buffer through a `0x30000100` status read and a `0xB0000100` burst header, `0x20` bytes at a time |
| `0x80227EF0` | 4-bit differential sample encoder; three magnitude bits with weights `step`, `step/2`, `step/4`, sign in bit `3`, step scaled by the eight `double` factors at `0x802F71A8` |
| `0x802281D0` | rebases an `0x0020AF30` header: table at `+8`, eight-byte entries, per-entry `+8` pointer fixed once behind a guard byte (`+0x23` or `+2`) |
| `0x8022852C` | bumps the transfer tag at `0x80429988`, selects the `0xD10000`/`0xD10800` DMA window from its parity, stages via `0x80228CF0`, writes a `0x1F000000` command via `0x80228B0C` |
| `0x8022892C` | `0x18000`-cycle delay, `0xB4000000`/`0xD4000000` immediate writes with control `0xC0`, final `0x405` mask |

The four transfer bodies reuse the constants and the translated helpers from
wave 92 ([`fn_802289e8`], [`fn_80228b0c`], [`fn_80228cf0`]) instead of calling
an address.

### Range-object constructors (`0x80229FF0..0x8022B5A0`, 5 bodies)

`0x80229FF0` is the copy constructor of the wave-47 `0x70`-byte range object
(vtable `0x8034CC40`): it clears the same eleven fields as [`fn_80229f30`],
copies fifteen source words into `+0x28..+0x60`, and reloads the range at
`+0x14`. `0x8022A590` and `0x8022A620` are its two derived constructors over
[`fn_80229f30`] and `fn_80229ff0`, both with vtable `0x8034CCA8` and a one-time
`0x803DA0C0` registration bracketed by the OS debug interrupt pair.

`0x8022AC40` builds one `0x16C`-byte object: a 34-word copy into `+0xD8`, the
three trailing bytes into `+0x160`, an optional vertex-format bind result at
`+0x14`, and the `+0x16B`/`+0x16C` state bytes. `0x8022B5A0` opens the object's
source through `0x801B0D60` and queries vtable slots `+0x24`, `+0x28`, `+0x58`.

### RFN container and colour object (`0x8022C3A0..0x8022C600`, 3 bodies)

- `0x8022C3A0` attaches a `FINF` payload to a render object; `0x8022C4F0` is
  the locator it uses, which walks the `FINF`/`PLGT`/`HDWC`/`PAMC`/`GLGR` node
  list, rebases each node's relative fields and flips the container magic to
  `RFNU` (`0x52464E55`).
- `0x8022C600` initialises the `0x4C`-byte two-palette colour object: the first
  eight words and two flag bytes become `0xFF`, the four corner colours at
  `+8..+0x17` are selected from the palettes at `+0x18`/`+0x1C` by the byte at
  `+0x20` (`0`/`1`/`2`), and the alphas are scaled by the blend byte at
  `+0x42`.

### Palette/render cluster (`0x8022D440..0x8022F120`, 13 bodies)

| Address | Role |
|---|---|
| `0x8022D440`/`0x8022D4A0`/`0x8022D500`/`0x8022D560` | one `double` getter each: the `+0x24`/`+0x28` field times the inner object's vtable slot `+0x0C`/`+0x10`/`+0x14`/`+0x18` |
| `0x8022D5C0` | stores `(arg != 0)` at `+0x38` and `+0x3C` |
| `0x8022D9A0` | rebuilds the four corner colours from the two palettes |
| `0x8022DAF0` | the fixed 34-call render-state setup in `0x801C37C0..0x801C8DC0`, with the level initialised once at `0x8042AC4C` |
| `0x8022DDE0` | runs `0x8022C600`, then stores `+0x4C..+0x60` |
| `0x8022DE40` | destructor over [`fn_8022c980`], frees only when `release > 0` |
| `0x8022DF30` | `+0x54` plus inner slot `+0x34` times the `+0x28` scale |
| `0x8022E730` | copies the `0x44`-byte block into a `0x90` stack frame, renders through `0x8022F960`, returns the level delta |
| `0x8022F120` | formats one argument into the `0x8042AC50` scratch buffer and appends the clamped text through `0x8022FA70` |
| `0x8022F910` | seeds a four-word record with `0` and source `+4`/`+8`/`+0xC` |

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80227CC0` | `fn_80227cc0` | 556 | medium |
| `0x80227EF0` | `fn_80227ef0` | 728 | medium |
| `0x802281D0` | `fn_802281d0` | 276 | high |
| `0x8022852C` | `fn_8022852c` | 272 | medium |
| `0x8022892C` | `fn_8022892c` | 188 | high |
| `0x80229FF0` | `fn_80229ff0` | 300 | high |
| `0x8022A590` | `fn_8022a590` | 132 | medium |
| `0x8022A620` | `fn_8022a620` | 132 | medium |
| `0x8022AC40` | `fn_8022ac40` | 600 | medium |
| `0x8022B5A0` | `fn_8022b5a0` | 212 | medium |
| `0x8022C3A0` | `fn_8022c3a0` | 328 | low |
| `0x8022C4F0` | `fn_8022c4f0` | 264 | high |
| `0x8022C600` | `fn_8022c600` | 896 | medium |
| `0x8022D440` | `fn_8022d440` | 96 | high |
| `0x8022D4A0` | `fn_8022d4a0` | 96 | high |
| `0x8022D500` | `fn_8022d500` | 96 | high |
| `0x8022D560` | `fn_8022d560` | 96 | high |
| `0x8022D5C0` | `fn_8022d5c0` | 36 | high |
| `0x8022D9A0` | `fn_8022d9a0` | 336 | high |
| `0x8022DAF0` | `fn_8022daf0` | 748 | medium |
| `0x8022DDE0` | `fn_8022dde0` | 92 | medium |
| `0x8022DE40` | `fn_8022de40` | 88 | high |
| `0x8022DF30` | `fn_8022df30` | 124 | high |
| `0x8022E730` | `fn_8022e730` | 324 | medium |
| `0x8022F120` | `fn_8022f120` | 172 | medium |
| `0x8022F910` | `fn_8022f910` | 56 | high |

## Why the twelve `medium` rows are `medium`

- `0x80227CC0`, `0x8022852C`, `0x80227EF0` and `0x8022AC40` open with a
  `0x802DCD40..0x802DCD74` register-save stub. The high half of the stub's
  fake 64-bit result is the incoming `r3`, the low half `r4` (the convention
  wave 92 established on `0x80225640`); the doc blocks name each recovery.
- `0x8022A590`/`0x8022A620` chain a base constructor that the export prints
  without an argument list. The base constructors' own exports read `r3` and
  `r4` (and `r5` for the copy constructor), so the bodies forward the caller's
  `source`/`owns`.
- `0x8022C600`, `0x8022DDE0` and `0x8022DAF0` read unnamed `sdata2` float words.
  The addresses are cited as `SDA2_F32_4C34/4C38/4C54/4C58`; their numeric
  values are not in the export and are not invented.
- `0x8022DAF0` and `0x8022B5A0` call the `0x801C37C0..0x801C8DC0` and
  `0x801B0D60` blocks by address. Every argument is literal; the callee roles
  are inferred from the argument shapes only.
- `0x8022E730` and `0x8022F120` build stack buffers. `0x8022F120`'s export
  places its buffer at a computed stack offset of `SDA_TEXT_SIZE` bytes, so the
  body uses a fixed `0x100`-byte stand-in and documents it.
- `0x8022F960` is called by address: its own export contains `unaff_GQR0`, so
  the paired-single payload cannot be recovered.

## The single `low` row

`0x8022C3A0` prints its `0x8022BC20` call with two arguments, but that helper's
own export reads three (`r4` and `r5` are stored at `+0x10` and `+0x14`). The
only value still live in `r5` at that point is the located `FINF` payload, so
the body passes it explicitly and the doc block says the third argument is
recovered rather than printed. Everything else in the function — the
`+0x10` guard, the `RFNU`/`FINF` walk, the nested-tag validation and the
`+0x58` selector — is literal.

## Deliberate skips

None of these has a row in `ledger/agents/wave97-bt.csv`.

- `0x8022E5B0` (384) - the object printer that pairs with `0x8022E730`. Its
  export calls the formatter `0x802E15DC` with only `(buffer, size)`, so the
  format string and its argument are gone; the text it would print is unknown.
  (Same defect as `0x8021E4D0` in wave 96.)
- `0x80225A80` (764) - the five-argument IPC text helper. It reads the `r13`
  globals `0x8042ABB8..0x8042ABC4`, a 16-entry ring and
  `thunk_FUN_80225EB0`, and its `&stack0x00000000 == 0x28` test is a
  decompiler artefact; too much of the state is unrecovered to claim a body.
- `0x80221440` (1424), `0x80222698` (1040), `0x80222AA8` (912) and
  `0x80222E38` (1052) - the four large SDP server request handlers that sit
  between the wave-60 client PDU machinery and the wave-60 CCB pool. They are
  the next targets of that cluster and were not started.
- `0x80220524` (572) - the SDP attribute deleter. Its compacting move is a
  hand-unrolled `memmove` whose source and length are both expressed through
  the same `0x802DCD6C` prologue pair, and the two branches disagree by one on
  the byte count; not reconstructed.
- `0x8022C9C0` (2352) - the largest function left in the bucket; 27 decompiler
  artefacts.
- `0x8022DFB0` (516) and `0x8022E1C0` (516) - both consume `extraout_f1`, a
  float register the decompiler never assigns.
- `0x8022F010` (272) - an 8-`undefined8`/8-word varargs formatter; the register
  save area is not modelled, so the argument list cannot be rebuilt.
- `0x8022D5F0` (304), `0x8022D720` (636), `0x8022EAF0` (540), `0x8022EED0`
  (320), `0x802293B0` (300), `0x80229860` (300), `0x80229990` (484),
  `0x8022F350` (1468), `0x8022F960` (272) and `0x8022FA70` (2052) - every one
  uses `unaff_GQR0`, the paired-single quantisation register whose runtime
  value decides the payload scale. The same reason waves 93/96 skipped their
  `GQR0` functions.
- `0x802260B0`, `0x80227800`, `0x80227A40`, `0x8022863C`, `0x80228640`,
  `0x8022B3C0`..`0x8022B3E0` (all 4 bytes) - tail-call thunks to functions
  that are either already translated or not; a one-instruction bounce is not a
  translation.
- `0x80228644` (744) - the EXI block-transfer worker. Its export is a dead
  `while(1)` loop (every block below it was removed as unreachable), so it is
  called by address as `EXI_BLOCK_80228644` from the five translated helpers,
  exactly as wave 92 did.
- `0x8022D440`'s neighbours `0x8022D2F0` (196) and `0x8022D3C0` (120) also use
  `unaff_GQR0` and are skipped for the same reason.

## Repairs made in this pass

No structural repair was needed. The append point was the true end of the file
and every pre-existing item still parses. The only rename is local: the pass
adds `SDA2_U32_BIAS_4C30` as documentation for the `0x43300000` conversion
sequence, but the bodies use plain `as f32`/`as f64` casts because that
sequence is the MSL unsigned-int helper, not data.

## Verification

- Every address and byte size in this doc and in the CSV was re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass; the 26 bodies total
  7244 bytes.
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/bt.rs`.
- No address was already in `ledger/functions.csv`; all 26 came from the
  bucket's untranslated list.
- `cargo check --message-format=short` reports no error in
  `src/runtime/bt.rs`; the last run finished the crate with warnings only
  (the parallel lanes' files churn while they land, so any error they introduce
  is transient and not ours).
- `node tools/check-lane-csv.mjs wave97` reports `wave97-bt.csv: 26 rows` and
  `OK` for the whole wave.
