# Coverage - Wave 147 (`game::bases`)

## How to read this file

This page documents the Wave 147 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`**; the Wii REL loader chooses
the real load address, so these numbers identify positions inside the REL image,
not runtime addresses. Every `# Original` block in the source carries the
`Synthetic base: 0x80600000; this is not a runtime address.` line.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee contract, a field role or a table outside the
REL had to be named rather than resolved.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 16 | 7,088 | 14 | 2 |

All 16 rows are `translated`; none is `partial`. The module header now reports
3675 bodies (3659 + 16). The `0x8076` planner bucket drops from 77 to 68
untranslated addresses (nine of the sixteen bodies live in that bucket).

## What this pass covers

- **The eight "ready" rows of `docs/SPEC_WAVE146_BASES.md`** (6,932 bytes), each
  re-read against its export and the raw image:

  | cluster | body |
  |---|---|
  | `cobCourse` door enter/attach | `0x80760700`: rest-pose snapshot, world byte `0x8042A24D`, actor gate, `0x284`/`0x29F` spawns, marker mirror, cues `0x22`/`0x24`/`0x25`/`0x2B`/`0x30` |
  | `lift_zen_kaihei` clip machine | `0x806E4F30`: four states over the `+0x48C` counter, idle clip `1` / open clip `3`, `35.0` threshold |
  | world-map burst emitter | `0x807318F0`: four compass probes at radius `24.0`, `qr3` `u16` angles, `0x807CD750`/`0x807CD760` effect-id tables, mask `0x807CD770`, three extra bursts |
  | moving-collision actor | `0x80699380`: mode select, `(+176, -72)` transform offset, `1.25` scale, collision descriptor `0x807F1408`, three 12.20 records, full `+0xD4C..+0xDDC` block |
  | bouncing-actor height driver | `0x80648CB0`: ease profile, limit probe `0x806485C0`, decaying sine, hold/restart, height publisher `0x80648470` |
  | carried-object per-frame body | `0x8069C2C0`: rider tick, clip-idle gate, sector flags `+0xD30..+0xD34`, `+0x360` launcher timer, orientation/collision tail |
  | stage/world-map teardown | `0x807B77E0`: archive gate `0x807B1C20`, four player-pane bytes, scene flag, eight owner releases, `0`/`1` return |
  | two-point trail updater | `0x807BEC10`: six-float lerp, `0x801C1650` squared distance + `0x802BE470` sqrt, `0.1` snap/release, preset-B effect `0x6D` |

- **Eight small `0x8076`-bucket bodies** (156 bytes):

  | cluster | bodies |
  |---|---|
  | scale reset | `0x8076D8B0` (`+0x33C = 0`, `+0xDC/E0/E4 = 0.01` from `0x807CF234`) |
  | mode table setter | `0x8076DA10` (`+0x340 = 0x807CF238[index]`, entries `0`, `5`, `7`, `0xB`) |
  | BSS predicate | `0x8076FE40` (`0x808327E0 != 0 && *(i32*)(+8) >= 3`) |
  | flag setters | `0x80762C40` (`+0x254 = 0`), `0x80765200` (`+0x47C = 2`), `0x80765290` (`+0x47C = 3`), `0x8076C6B0` (`+0x190 = 0`) |
  | handle invalidator | `0x8076D8A0` (`+0x35C = -1`, then `b 0x8076D8B0`, implemented as the tail call it is) |

## Where the binary differed from the wave-146 prose

1. **`0x807318F0`'s first effect argument is the literal `0x15B`.** The words at
   `0x80731B08..0x80731B18` are `li r3,0x15B` / `rlwinm` / `ori r4,r4,0x301` /
   `or r4,r4,r0` (with `r0 = id & 0x300000`), so the call is
   `0x80064610(0x15B, attr, &p, 0, 0)`; `id + 0x15B` never appears. The export
   prints the same literal.
2. **`0x807B77E0`'s `0x8042A21E` byte is `v == 4`, not `v - 4 < 0`.** The
   sequence is `addi r0,r3,-4; cntlzw r0,r0; srwi r0,r0,5`, and the `cntlzw`
   idiom is true exactly when the word equals four. The wave-146 prose read the
   subtraction as a signed comparison.
3. **`0x80699380` copies `+0x750..+0x758` into `+0x768..+0x770`.** Lines
   `*(param_1 + 0x768/0x76C/0x770) = *(param_1 + 0x750/0x754/0x758)` in the
   export precede the hitbox extents; the wave-146 field map listed `+0x768` only
   as "target x" in the sibling body and omitted the three stores here.
4. **`0x8069C2C0`'s `0x8015F470` order is `(dest, +0x34C, +0x350)`.** The raw
   words at `0x8069C4E0..0x8069C508` show slot `+0x350` saved to `f31`, slot
   `+0x34C` moved to `f2`, then `addi r3,r31,0x114` immediately before the `bl`.
   The Rust call passes `f1 = +0x34C`, `f2 = +0x350`, `r3 = dest`, matching the
   stream; the export prints the destination last because it has no prototype.

Two confirmations rather than corrections:

- `0x80648CB0`'s state-2/3 angle really is `(u16)(counter << 9)`: the halfword is
  written to the stack and re-read by `psq_l f1,0(rX),qr3,1`, i.e. a plain
  `u16 -> f32` conversion after `* 1/256`.
- `0x8076FE40`'s global `0x808327E0` is inside this REL's zero-fill BSS
  (`0x80822900..0x80834D84`, `filesz = 0`), so the shipped image starts it at
  zero and the predicate returns `0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80648CB0` | `fn_80648cb0` | 860 | high |
| `0x80699380` | `fn_80699380` | 868 | medium |
| `0x8069C2C0` | `fn_8069c2c0` | 852 | high |
| `0x806E4F30` | `fn_806e4f30` | 880 | high |
| `0x807318F0` | `fn_807318f0` | 876 | high |
| `0x80760700` | `fn_80760700` | 896 | high |
| `0x80762C40` | `fn_80762c40` | 12 | high |
| `0x80765200` | `fn_80765200` | 12 | high |
| `0x80765290` | `fn_80765290` | 12 | high |
| `0x8076C6B0` | `fn_8076c6b0` | 12 | high |
| `0x8076D8A0` | `fn_8076d8a0` | 12 | high |
| `0x8076D8B0` | `fn_8076d8b0` | 32 | high |
| `0x8076DA10` | `fn_8076da10` | 24 | high |
| `0x8076FE40` | `fn_8076fe40` | 40 | high |
| `0x807B77E0` | `fn_807b77e0` | 852 | high |
| `0x807BEC10` | `fn_807bec10` | 848 | medium |

**Totals:** 16 bodies, 7,088 bytes. The address window runs from `0x80648CB0` to
`0x807BEC10`.

## Notes on inference

- `0x80699380` is `medium`: `0x800A3430` and `0x800FAE40` live outside this REL
  and the `0x8042C1A0` byte table cannot be checked here. Every argument, store
  offset and fixed-point mask is an immediate.
- `0x807BEC10` is `medium` only for the identity of `0x802BE470` as the
  square-root wrapper (its export is `(double)(float)thunk_FUN_802e8620()` with
  the argument in `f1`; `0x801C1650` returns the squared distance as a `double`).
  The arithmetic, the six stores and both branch conditions are literal; the 36
  dead `stfs` temporaries in the export are vector-copy leftovers and are not
  reproduced.
- The `qr3` loads in `0x807318F0` and `0x80648CB0` use
  `GQR3 = 0x00050005` from `docs/REL_ELF_MAPPING.md` (type 5, unsigned 16-bit,
  scale 0), which is exactly the `u16 -> f32` conversion both bodies need.
- The float constants `24.0`, `1/256`, `-50.0`, `5500.0`,
  `35.0`, `80.0`, `112.0`, `176.0`, `-72.0`, `0.85`, `16.0`, `1.25`, `8.0`,
  `4.0`, `6.0`, `-7.0`, `0.5`, `0.03125`, `0.125`, `3.0`, `1.0`, `0.1`, `0.01`
  and the tables `0x807CD750`, `0x807CD760`, `0x807CD770`, `0x807C8518`,
  `0x807CF238` were read from `work/rels/elf/d_basesNP.elf` through
  `file_offset = (address - 0x80600000) + 0x1000`. The string at `0x8080D0E8`
  is `"Wm_en_burst_s"` (observed). The `0x8037xxxx`, `0x8042xxxx`, `0x8083xxxx`
  words are `main.dol`/REL BSS and are named by address.

## Deliberate skips

- **Three 4-byte `blr` stubs** in the `0x8076` bucket (`0x807622A0`,
  `0x8076B090`, `0x8076BA60`) are empty returns with no observable effect; they
  add no evidence and were left open rather than padded into the lane.
- **`0x80761670` (12 bytes), `0x80765180` (12 bytes), `0x8076FEF0` (24 bytes)**
  truncate on bad instruction data (`halt_baddata`) in their exports; no honest
  body exists. `0x80762B20` (16 bytes) is an unresolved indirect-jump stub
  (`0x802DCBFC`-family); writing it would require inventing the jump table.
- The remaining 68 `0x8076` rows were not started; they are the natural next
  wave for this bucket.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (16 rows, 7,088 bytes),
  re-read from the TSV for this pass rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 16 were checked against
  the ledger's address column before being written, and the lane validator
  repeats that check.
- `node tools/check-lane-csv.mjs wave147` reports `OK` for
  `wave147-bases.csv`: 16 rows, 16 unique addresses, no duplicates.
- `cargo check --message-format=short` exits `0` and reports no diagnostic
  naming `src/game/bases.rs`.
- `0x8076D8A0`'s last word is a tail branch (`b 0x8076D8B0`), not `blr`; the TSV
  extent (12 bytes) is the whole body, per the extent note in
  `docs/REL_ELF_MAPPING.md`.
