# Coverage - Wave 220, lane B, `src/resource/archive.rs`

## How to read this file

- **Scope** - the ten bodies added to `src/resource/archive.rs` by wave 220,
  lane B, module `resource::archive`: the `0x802C6020..0x802C8144` band.
  Earlier work in this file is documented in the lane files of the other
  waves; this file does not repeat it.
- **Addresses are analysis addresses.** They are absolute `main.dol`
  addresses, as printed by Ghidra and `work/dol/main.elf`.
- **Evidence** is the Ghidra C under `work/ghidra/main-gb/export/`; the exact
  file is named in every table row. Sizes come from
  `work/ghidra/main-gb/export/functions.tsv`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and every observable store are represented.
- **Confidence** rates the translation, not the guessed role. Nine bodies are
  literal (`high`); the named-record builder `0x802C7560` is `medium` because
  it depends on the unexported record layout and the four `0x801C71xx`
  appliers.
- **Names** stay `fn_802cxxxx`. The shipped image has no symbol table, so no
  Nintendo symbol is invented. New constants carry the lane `_B` suffix.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 10 |
| text bytes represented | 1868 |
| status `translated` | 10 |
| status `partial` | 0 |
| status `deferred` | 0 |
| confidence `high` | 9 |
| confidence `medium` | 1 |

## Size verification

Every `functions.tsv` size was checked against the prologue and the closing
`blr` in `work/dol/main.elf`. Nine values match the instruction extent
exactly. `0x802C64F0` is the one row whose 64 bytes include 4 bytes of
alignment padding: its `blr` at `0x802C652C` ends a 60-byte body.

| address | tsv | real extent | note |
|---|---:|---:|---|
| `0x802C6020` | 8 | 8 | leaf, `addi` then `blr` |
| `0x802C64F0` | 64 | 60 | `blr` at `0x802C652C`; 4 padding bytes to `0x802C6530` |
| `0x802C6AE0` | 260 | 260 | ends at the `blr` at `0x802C6BE0` |
| `0x802C6CE0` | 76 | 76 | leaf, ends at the `blr` at `0x802C6D28` |
| `0x802C6D30` | 48 | 48 | leaf, `blr` at `0x802C6D5C`, body ends at `0x802C6D60` |
| `0x802C6E20` | 44 | 44 | ends at the `blr` at `0x802C6E48` |
| `0x802C6F60` | 144 | 144 | ends at the `blr` at `0x802C6FEC` |
| `0x802C7560` | 848 | 848 | ends at the `blr` at `0x802C78AC` |
| `0x802C7D30` | 244 | 244 | ends at the `blr` at `0x802C7E20` |
| `0x802C80C0` | 132 | 132 | ends at the `blr` at `0x802C8140` |

## Function table

| address | bytes | confidence | what the body does |
|---|---:|---|---|
| `0x802C6020` | 8 | high | returns the four-byte SDA table at `r13 - 0x5B30` |
| `0x802C64F0` | 64 | high | guarded delete callback, returns its first argument |
| `0x802C6AE0` | 260 | high | initialises a rect/quad descriptor from six floats |
| `0x802C6CE0` | 76 | high | maps one point through the descriptor's 0/1 mode |
| `0x802C6D30` | 48 | high | mode-1 distance profile `(|b| * a) / (0.5 * w * h)` |
| `0x802C6E20` | 44 | high | builds the `0x802C6FF0` block and submits it |
| `0x802C6F60` | 144 | high | fills a `0x40`-byte quad descriptor from that block |
| `0x802C7560` | 848 | medium | matches records by name and builds two transforms per hit |
| `0x802C7D30` | 244 | high | packs a three-float vector into clamped colour bytes |
| `0x802C80C0` | 132 | high | calls a virtual method once per 12-byte entry |

## Notes on `0x802C7560`

The Ghidra export opens with `FUN_802dcd44` and closes with `FUN_802dcd90`.
The disassembly at those addresses is the standard `__savegprlr_14` /
`__restgprlr_14` pair, so the export's `uVar14` return value is a register-save
artefact rather than a result. The high/low split there is undone as the
`r3`/`r4` pair: `r3` is the `context` word and `r4` the `object` word. The
function's real result is `r18` (the match count) moved to `r3` before the
epilogue.

The record walk itself is literal: `*context` is the table, `+0x2C` its record
count, `+0x30` the record-array offset and `0x34` the record stride. The two
per-hit transforms and the optional ID pairs use the offsets recorded inline in
the Rust body. The roles of `0x802E1AB4` (name lookup) and the four
`0x801C71xx` appliers are inferred from their call shapes; their bodies are
outside this lane.
