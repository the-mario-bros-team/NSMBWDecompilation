# Audit - Wave 265 (three more vtables; live page bugs fixed)

## 1. Three tables closed

| Table | Slots | Notes |
|---|---:|---|
| `0x8030241c` | **199/199** | last 14 were 9 `bclr` stubs + 5 small bodies; `"throw"` bind at slot 193 |
| `0x80305958` (`daEnObjCoinBlock_c`) | **190/190** | `"Wm_mr_kickhit"` tail spawn at slot 185 |
| `0x80306400` (`daEnTogezo_c`) | **193/193** | 4 of the 16 leaves were already covered by lanes B/C's shared `0x8003xxxx` bodies |

**Fourteen vtables complete**: 275, 278, 311, 45, 306, 235, 265, 183, 182, 187,
186, 199, 190, 193.

Shared-body reuse is now the norm: lane A found four of its 16 named leaves
already defined by the other table-closing lanes in the same wave, and
forwarded rather than redefining.

## 2. Live page errors in `hud.rs` fixed

The wave-264 recon lane found two families of `addi`-as-`ori` mis-decodes still
live in `src/ui/hud.rs`:

| Wrong | Correct | Evidence |
|---|---|---|
| `0x8043C1A0` / `A4` / `B0` | **`0x8042C1A0` / `A4` / `B0`** | `lis r3,0x8043` + `addi -15968`/`-15964`/`-15952` at `0x8003B240..0x8003B4BC` |
| `0x80368004` | **`0x80358004`** | `lis r4,0x8036` + `addi r4,r4,-32764` at `0x80029BBC` |

`0x8043C1A0` lies past the last loadable segment (`0x8042FBE0`), and the real
table is file-backed holding exactly what the bodies expect: `s8 {1,-1,0,0}` at
`+0x00`, `s16` angles at `+0x04`, `1.5f`/`-1.5f` at `+0x10`. 36 occurrences were
corrected (names, values, prose), and the shape note that claimed the tables
were "`.bss` zeros" now describes the real contents.

This one is worth noting for method: the constants' **names matched their
values**, so the name-vs-value check could not see it, and the docs stated the
address without the instruction pair, so the `lis`/`addi` check could not
either. It was caught by a human-style recon read of adjacent code - which is
why the recon lane stays in the rotation.

## 3. Page tool: name-vs-value check added

`tools/check-page-offsets.mjs` now runs two checks:

| Check | Coverage | Result |
|---|---|---|
| documented `lis`/`addi` pair vs declared value | 26 pairs (11 negative immediates) | 0 mismatches |
| address-shaped name token vs declared value | 13,523 tokens | 4 fixed, 38 report-only |

The 4 fixed were all **truncated nibbles in names**, the mirror of the page
error: `D_8029_BB8` -> `D_8042_9BB8` (MSL `NaN` double at `0x80429BB8`),
`EMPTY_WIDE_8029BB0` -> `EMPTY_WIDE_8042_9BB0` (empty wide string),
`W90_VTABLE_80330E0` -> `W90_VTABLE_80B330E0`, `FN_80126A0` -> `FN_800126A0`.
Values were already correct, so only names and readers changed.

## 4. Measurement caveat from recon

`0x80304940` and `0x80305958` are both genuine vtables (zero-pair headers,
186/190 distinct 16-byte-aligned starts, install site storing the header at
`this+0x60`), not dispatch tables. `0x80304940` is the shared base
`daEnJimenPakkunBase_c` installed by three `d_enemiesNP` constructors which then
overwrite `+0x60` with REL-local copies.

The recon lane also flagged a **measurement** issue: the project's rule that a
`bcctr` ends a path is wrong when the encoding is `bcctrl` (a call), which
under-measures virtual-call bodies - e.g. `0x800323F0` is 380 B, not 40. This
matters beyond extents: a body truncated at a call would be missing the code
after it. Queued as its own audit.

## 5. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | `0x80306400` last 12 (closes) | 12 | ~300 |
| B | `0x8030241c` last 14 (closes) | 14 | 112 |
| C | `0x80305958` last 12 (closes) | 12 | ~100 |
| D | `0x80304940` remainder | running | - |
| E | `0x80305f18` first 10 | running | - |
| F | page tool hardening | 4 renames | - |

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-dupe-bodies` exit 0 (both passes).
- `check-page-offsets` exit 0 (both checks).
- `scripts/check.ps1` exit 0.
