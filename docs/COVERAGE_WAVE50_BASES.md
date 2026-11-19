# Coverage - Wave 50 (bases)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL (synthetic base `0x80600000`). Every address below is a static analysis
address, not a runtime address. Byte counts come from
`work/ghidra/rel-d_basesNP/export/functions.tsv`; the quoted evidence files are
the Ghidra C exports under `work/ghidra/rel-d_basesNP/export/`. Confidence is
`high` when the guards, stores and call targets are literal, `medium` when a
callee role or a field role is inferred from argument shape.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 3484 | 22 | 8 |

## What this pass covers

- The helper tail of the `0xE18`-byte boat-light actor built by
  [`fn_80701820`] and loaded by [`fn_80702750`]. The create body `0x807041B0`
  and the per-frame bodies `0x80702E40`, `0x80702EA0`, `0x807032E0`,
  `0x80703390` and `0x80703CA0` call these helpers by fixed address, which is
  what fixes their field roles.
- The two inline object arrays at `+0x574` (two `0xE8`-byte elements) and
  `+0x744` (five `0x90`-byte elements): both constructors
  ([`fn_80701a00`], [`fn_80701b10`]), both destructors ([`fn_80701a90`],
  [`fn_80701b70`]), the `+0x74` interface rebind [`fn_80701c10`] and the
  `+0x30` child-chain walk [`fn_80700df0`].
- The palette block at `+0xA14`, `0x80701C90..0x80701F80`: colour init and
  clear ([`fn_80701c90`], [`fn_80701cd0`]), index switch with fade
  ([`fn_80701d10`]), the `Wm_en_burst_m` burst [`fn_80701e10`] and the two
  index-triggered sound cues [`fn_80701e60`]/[`fn_80701ef0`].
- The beam transform writer [`fn_80702110`] and its release
  [`fn_807022d0`], plus [`fn_807020c0`] which rebinds the `+0x1C` interface and
  tail-calls [`fn_80701f80`].
- The three-byte light colour records at `+0xA2C + index * 3`
  ([`fn_80702330`], [`fn_80702350`], [`fn_80702380`]) and the four two-byte
  player switch records at `+0xDAC` ([`fn_80702470`], [`fn_807024c0`],
  [`fn_80702510`], [`fn_80702520`]).
- Five registered-state/owner destructors and one constructor stub in the
  same window: [`fn_80700c50`], [`fn_80700cb0`], [`fn_80701530`],
  [`fn_807016a0`], [`fn_80700ff0`], plus the object-name comparison
  [`fn_80700ed0`] and the four-member reset loop [`fn_80700820`].

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x807001C0` | `fn_807001c0` | 92 | high |
| `0x80700820` | `fn_80700820` | 176 | high |
| `0x80700C50` | `fn_80700c50` | 88 | high |
| `0x80700CB0` | `fn_80700cb0` | 92 | high |
| `0x80700DF0` | `fn_80700df0` | 224 | high |
| `0x80700ED0` | `fn_80700ed0` | 136 | medium |
| `0x80700FF0` | `fn_80700ff0` | 76 | high |
| `0x80701530` | `fn_80701530` | 88 | high |
| `0x807016A0` | `fn_807016a0` | 88 | high |
| `0x80701A00` | `fn_80701a00` | 132 | high |
| `0x80701A90` | `fn_80701a90` | 128 | high |
| `0x80701B10` | `fn_80701b10` | 88 | high |
| `0x80701B70` | `fn_80701b70` | 92 | high |
| `0x80701C10` | `fn_80701c10` | 72 | medium |
| `0x80701C90` | `fn_80701c90` | 52 | high |
| `0x80701CD0` | `fn_80701cd0` | 56 | high |
| `0x80701D10` | `fn_80701d10` | 256 | medium |
| `0x80701E10` | `fn_80701e10` | 76 | high |
| `0x80701E60` | `fn_80701e60` | 132 | medium |
| `0x80701EF0` | `fn_80701ef0` | 132 | medium |
| `0x807020C0` | `fn_807020c0` | 76 | high |
| `0x80702110` | `fn_80702110` | 448 | medium |
| `0x807022D0` | `fn_807022d0` | 80 | high |
| `0x80702330` | `fn_80702330` | 20 | high |
| `0x80702350` | `fn_80702350` | 36 | high |
| `0x80702380` | `fn_80702380` | 236 | medium |
| `0x80702470` | `fn_80702470` | 72 | high |
| `0x807024C0` | `fn_807024c0` | 76 | high |
| `0x80702510` | `fn_80702510` | 16 | high |
| `0x80702520` | `fn_80702520` | 148 | medium |

## Deliberate skips

- `0x807003C0` (492 bytes), `0x807005C0` (600 bytes) and `0x80701390`
  (416 bytes) decompile with an unrecovered `GQR0` register and `ldexpf`
  bookkeeping; the paired-single register pair feeding each body is not
  visible in the export, so their float semantics cannot be settled and no
  body was invented.
- `0x80700320` (160 bytes) clamps a speed against a signed-byte table at
  `0x8042C1A0` through a `CONCAT44(0x43300000, ...)` integer-to-double
  conversion. The control flow is readable but the table's value domain is
  not, so it is left as an address-only neighbour for a later pass.
- `0x80700F90`, `0x80700FC0`, `0x807017C0` and `0x807017F0` are 48-byte
  adjustor thunks whose whole body is `0x802DCBFC(a2, a2, a1)`. They carry no
  recoverable object role, so they were skipped rather than padded into rows.
- `0x80701C60` (44 bytes) ends in an unrecovered jump table
  (`0x80701C88`, "too many branches"); the target set is missing.
- `0x80702320` (16 bytes) sets both index bytes to `0xFF`; it is covered
  semantically by [`fn_80702330`] with `0xFF` and was skipped to stay inside
  the 30-body budget.
- The rest of the `0x80700000` bucket is outside this lane: 193 entries were
  untranslated before this pass and 163 remain after it. The chosen window is
  otherwise contiguous.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv`.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000` line.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error for
  `src/game/bases.rs`.
- `node tools/check-lane-csv.mjs wave50` reports no problem for
  `wave50-bases.csv` (the remaining reports name `wave50-input.csv`).
