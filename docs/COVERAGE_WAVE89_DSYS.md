# Coverage - Wave 89 (engine::dsys)

## How to read this file

`engine::dsys` reconstructs the `dSys_c` engine start-up and the display
helpers that sit around it in the main `DOL`. Each row below is one
`pub unsafe fn fn_<address>` in `src/engine/dsys.rs`; the doc block above that
body cites the Ghidra export it came from, the byte count from
`work/ghidra/main-gb/export/functions.tsv`, and a confidence level.
`high` means the guards, stores and calls are literal in the export;
`medium` means a field role, a callee identity or a dropped argument was
inferred and is marked `// inferred:` or `// unknown:` in the body.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/dsys.rs` | `engine::dsys` | 28 | 2844 | 20 | 8 |

The file already held 48 bodies (18 `dSys_c` start-up plus the 30 Wave 75
bodies); this pass adds 28 more, for 76 bodies total.

## What this pass covers

- **12-bit fixed-point value object, `0x800E37F0..0x800E3A60` (4 bodies).**
  One constructor (`+4` = `value << 12`, `+8` = plain `u16`, vtable
  `0x8031B040`, published at `0x8042A070`), its destructor, the clamped
  setter, and the "value exhausted" guard that raises a single byte flag.
- **`u16` text-stream opcode handlers, `0x800E5A70..0x800E65E0` (3 bodies).**
  The two operand handlers share the CodeWarrior `2^52` widening (the
  `CONCAT44(0x43300000, x)` artefact), resolved from the real double at
  `r2 - 0x6378 = 0x8042CD08`; `0x800E6030` is the 0x28-byte glyph-record
  colour/rectangle bind.
- **Message builder, second half, `0x800E6BE0..0x800E7130` (7 bodies).** The
  same `+4` glyph-string block as the Wave 75 cluster: two fixed handles
  (`context + 0x3BC` / `+0x3C4`), the decimal counter at `+0x3C0`, the
  two-part label with its `0x5F` separator, and `0x800E6EF0`, which fills the
  `0x28`-byte layout slot selected by `context + 0x3C8` from the frame and
  surface records.
- **Notification list, `0x800E7590..0x800E7880` (3 bodies).** `0x800E7590`
  drains the 0x20-byte nodes at `0x8042A0D8` (unlink via `0x80160F60`, then
  free the payload), `0x800E75F0` allocates and appends one node, and
  `0x800E7880` is the rectangle-plus-text helper.
- **List widget, `0x800E8BA0..0x800E8CC0` (2 bodies).** Construction
  (`0x8016A960`, the chain from Wave 75's `fn_800e8dc0`, the binding pass and
  the finaliser) and the per-node transform pass over the `+0x54` chain.
- **Per-node animation, `0x800E9E20..0x800EA540` (3 bodies).** The alternating
  parity restart, the finish-detection step that calls the widget's virtual
  slot `+0x18` with tag `0x80371C48`, and the node teardown.
- **Record hit test and angle, `0x800EB640..0x800EB6B0` (2 bodies).**
- **Static node, `0x800EDF20..0x800EE930` (4 bodies).** Constructor with
  vtable `0x8031BA1C`, two destructor shapes, and the three-pass list rebuild.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800E37F0` | `fn_800e37f0` | 48 | high |
| `0x800E3820` | `fn_800e3820` | 72 | medium |
| `0x800E3910` | `fn_800e3910` | 28 | high |
| `0x800E3A60` | `fn_800e3a60` | 68 | medium |
| `0x800E5A70` | `fn_800e5a70` | 84 | medium |
| `0x800E6030` | `fn_800e6030` | 88 | high |
| `0x800E65E0` | `fn_800e65e0` | 152 | medium |
| `0x800E6BE0` | `fn_800e6be0` | 76 | high |
| `0x800E6CE0` | `fn_800e6ce0` | 96 | high |
| `0x800E6EF0` | `fn_800e6ef0` | 328 | medium |
| `0x800E7040` | `fn_800e7040` | 32 | high |
| `0x800E7060` | `fn_800e7060` | 76 | high |
| `0x800E70B0` | `fn_800e70b0` | 116 | high |
| `0x800E7130` | `fn_800e7130` | 92 | high |
| `0x800E7590` | `fn_800e7590` | 96 | high |
| `0x800E75F0` | `fn_800e75f0` | 176 | medium |
| `0x800E7880` | `fn_800e7880` | 84 | medium |
| `0x800E8BA0` | `fn_800e8ba0` | 92 | high |
| `0x800E8CC0` | `fn_800e8cc0` | 100 | high |
| `0x800E9E20` | `fn_800e9e20` | 184 | high |
| `0x800E9EF0` | `fn_800e9ef0` | 164 | high |
| `0x800EA540` | `fn_800ea540` | 88 | high |
| `0x800EB640` | `fn_800eb640` | 104 | high |
| `0x800EB6B0` | `fn_800eb6b0` | 44 | medium |
| `0x800EDF20` | `fn_800edf20` | 72 | high |
| `0x800EE3D0` | `fn_800ee3d0` | 96 | high |
| `0x800EE430` | `fn_800ee430` | 64 | high |
| `0x800EE930` | `fn_800ee930` | 124 | high |

## Field notes

- The `CONCAT44(0x43300000, x)` / `2^52` pairs are the CodeWarrior
  unsigned-to-double idiom. Wave 89 resolves the subtrahend from its real
  address (`r2 - 0x6378 = 0x8042CD08`) through the local `read_f64` helper
  instead of assuming `2^52`, so nothing is invented.
- `0x800E5DD0`, `0x800E6090`, `0x800EA6F0`, `0x800EA980`, `0x800EA8D0` and
  `0x800EE9B0..0x800EF770` are called by absolute address; their bodies are
  outside this pass but their signatures are visible in the export.
- `unaff_r13` is the SDA base `0x8042F680`, `unaff_r2` the second base
  `0x80433080`; every offset is spelled out as a named constant near the
  bodies that use it.
- `0x802DCD68`, `0x802DCD6C`, `0x802DCD70`, `0x802DCD74`, `0x802DCDB4`,
  `0x802DCDBC` and `0x802DCDC0` are the Metrowerks `__savegprlr` /
  `__restgprlr` register-save entries, not high-level calls. Bodies that need
  the object pointer print as the fake 64-bit return of those helpers were
  skipped rather than guessed (see below).

## Deliberately skipped

- **`0x800E6290`** - the opcode dispatcher. Its record path calls
  `0x800E6030` with no printed arguments, while `0x800E6030`'s second
  parameter is a record *index* (`base + index * 0x28`). The export has only
  one caller, so the argument cannot be recovered; writing one would be a
  guess. Left for a pass that reads the raw disassembly.
- **`0x800E76A0`** - the list lookup. Its tail call to `0x800E75F0` drops all
  four arguments, and `0x800E75F0` needs a vector plus two words that are not
  identifiable from the caller. Skipped for the same reason.
- **`0x800E2DE0`, `0x800E2F10`, `0x800E3040`, `0x800E3160`, `0x800E7EE0`,
  `0x800E9060`, `0x800EA7D0`, `0x800EA980`, `0x800EA8D0`, `0x800EB6E0`** -
  the paired-single helpers. Their exports carry the `ldexpf` /
  `unaff_GQR0` mis-render and "removing unreachable block" warnings, so the
  argument registers and stack slots cannot be trusted.
- **`0x800E0540`, `0x800E1F80`, `0x800E20A0`, `0x800E24B0`, `0x800E2580`,
  `0x800E26F0`, `0x800E27C0`, `0x800E28A0`, `0x800E3BA0`, `0x800E5820`,
  `0x800E5AD0`, `0x800E5DD0`, `0x800E6090`, `0x800E63A0`, `0x800E6680`,
  `0x800E71C0`, `0x800E9130`, `0x800EB040`, `0x800EB290`, `0x800EB930`,
  `0x800EC3B0` large bodies** - 500 to 1836 bytes each, built on jump tables
  or on object layouts that this pass could not prove. `0x800EA060` is the
  1236-byte `DAT_8031B1C0` table restore and `0x800EA000` / `0x800EA030`
  forward into the unrecovered jump table at `0x802DCBFC`; all three stay
  outside this lane.
- **Remaining list-rebuild family, `0x800EE170`, `0x800EE610`, `0x800EE850`,
  `0x800EE9B0`, `0x800EEAE0`, `0x800EEE70`, `0x800EEFF0`, `0x800EF230`,
  `0x800EF570`, `0x800EF770`, `0x800EF890`, `0x800EF940`, `0x800EF9F0`,
  `0x800EFAA0`, `0x800EFB50`, `0x800EFE60`, `0x800EFF30`** - these mix the
  `__savegprlr` fake 64-bit return (`0x802DCD68`/`0x802DCD6C`/`0x802DCD74`)
  with `unaff_r2` float arguments, and several also carry the paired-single
  `ldexpf`/`unaff_GQR0` mis-render. Neither the object base nor the
  floating-point arguments can be proven from the export, so no honest body
  could be written.
- **`0x800E54E0`, `0x800E55F0`, `0x800E5790`** - font/console setup whose
  six-float calls would need a float-argument call helper that
  `crate::platform` still does not provide.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/engine/dsys.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error in
  `src/engine/dsys.rs` (the remaining errors belong to other lanes' files:
  `src/runtime/cpp.rs`, `src/runtime/input.rs`).
- `node tools/check-lane-csv.mjs wave89` reports `OK`.
