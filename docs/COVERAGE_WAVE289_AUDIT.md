# Audit - Wave 289 (sweep complete, source-truncation incident)

## 1. INCIDENT: `src/ui/lyt.rs` truncated to 0 bytes

At 13:45:21 an in-flight whole-file write left `src/ui/lyt.rs` at **0 bytes**,
destroying ~19,000 lines / ~700 KB of translated layout code and breaking the
build (37 errors, all missing `crate::ui::lyt::*`).

**Recovery.** No git in this workspace, so the file was rebuilt from:

| Source | Contents |
|---|---|
| `%TEMP%/opencode/d285check/src/ui/lyt.rs` | newest full copy, 17,897 lines / 655,433 B |
| `work/wave285/b_lyt_append.rs` | the seven `lyt::Material` bodies (844 lines) |
| `work/wave289/a_block.rs` | the `ResFont` bodies (590 lines) |

No duplicate constants or function names existed between the pieces (checked
before joining). Result: **19,329 lines / 710,378 bytes**, `cargo check` exit 0,
doc-lint clean, duplicate and page-offset checks clean, full gate green.

Only wave-285 lane F's doc-only fixes to `lyt.rs` are lost.

**Prevention added.**

1. `tools/check-source-integrity.mjs`, wired into `scripts/check.ps1`:
   fails on a 0-byte `.rs`, a file with no bodies where `pub unsafe fn` is
   declared, a NUL byte, U+FFFD, or a file/total shrinking below 25%/90% of a
   recorded baseline (`work/source_sizes.json`, `--record` to reset).
2. `work/backup_src/` - a rolling mirror of `src/` (69 files, 51.7 MB),
   refreshed with robocopy `/MIR` so every wave start has a full fallback.

## 2. `_savegpr`/`_restgpr` sweep: literal sites complete

| Metric | Start of sweep (wave 276) | Wave 289 end |
|---|---:|---:|
| Literal modelled helper calls | 616 | **0** |
| Bodies with literal calls | 324 | **0** |

Part 14 swept the last 24 bodies / 47 sites (23 in `bosses.rs`, plus the untabled
`fn_800f8db0`). Corrections this round: `fn_80c56ec0`'s fake `r3:r4` pair became
`(table, actor, index, mask)`; two bodies now return `best`; three `li r3,1`
tails became `-> u32`; 15 bodies gained an `r3` parameter.

**Residue (important):** a literal scan is no longer sufficient. Renamed
constants still carry aliased helper uses:

| File | Constants | Residual uses | Bodies |
|---|---:|---:|---:|
| `runtime/input.rs` | 24 | 113 | 40 |
| `game/stage.rs` | 42 | 58 | 33 |
| `runtime/bt.rs` | 8 | 41 | 11 |
| `resource/registry.rs` | 4 | 6 | 4 |
| `resource/bres.rs` | 2 | 2 | 1 |
| **total** | **91** | **220** | **89** |

Queued as the next sweep phase (alias-aware scan by address, not by name).

## 3. Tables closed

| Table | Slots | Identity |
|---|---:|---|
| `0x8034CE74` / `0x8034CED0` | **21/21** each | BTE palette-device base/derived |

**Eighty-six vtables complete.** The pair shares slots 1-20 in the same order and
differs only in slot 0 (base release tail vs derived deleting destructor). Slot
14's `fn_8022BE10` decodes with `rlwinm` semantics:
`((packed >> 8) & 0xFF)` sign-extended, not `>> 24`.

## 4. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | `ResFont` (recovered) | 19 | ~4,000 |
| B | palette pair close | 19 | 1,120 |
| C | audio element + seq players close | 20 | 1,516 |
| D | savegpr sweep final | 24 fixed | - |
| E | gap batch 19 | 12 | ~1,340 |
| F | audit + collision aliases | running | - |

## Verification

- `cargo check` exit 0 (31 warnings); `doc-lint` 0 violations.
- `check-dupe-bodies` exit 0; `check-page-offsets` 52 pairs / 13,837 tokens,
  0 mismatches.
- `check-source-integrity` 68 files / 51,764,357 bytes, 0 findings.
- `scripts/check.ps1` exit 0.
