# Coverage - Wave 64 (WorldMap magma-wave object)

## How to read this file

Each row is one original `main.dol` function, reconstructed as a
`pub unsafe fn fn_<address>` body in `src/game/worldmap.rs`. The *address* is the
PowerPC virtual address from `work/ghidra/main-gb/export/functions.tsv`; the
*bytes* column is the exact size from that file; *confidence* matches the
`# Confidence` line of the body's doc comment. "high" means the loads, stores
and calls are literal in the export; "medium" means a field role or a callee
identity is inferred and called out in the body.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 26 | 6916 | 3 | 23 |

## What this pass covers

- `0x8011D450..0x8011D590` - the two ground-collider builders. Both read the
  object header type at `+0x04`, submit the empty descriptor set for type `1`,
  and otherwise fill `+0x554..+0x584` from the height at `+0x5B4` before calling
  `0x8006D090`. The second variant also reads the player slot byte `+0x38D`.
- `0x8011DBB0..0x8011DE50` - the contact gates. [`fn_8011dbb0`] selects the
  `+0x5C0` response from the actor kind `+0x38C` and height `+0xEC`;
  [`fn_8011dce0`] is the `+0x61C` child accessor; [`fn_8011dcf0`] and
  [`fn_8011de50`] test the `+0x394` name component and forward event `0x11`
  or `3`; [`fn_8011de40`] is the literal zero placeholder used by the
  dispatchers.
- `0x8011DF00..0x8011E420` - the contact-response dispatch tree. The two
  `DF` variants probe the ground with `0x80075270` and select sound modes
  `7/2` or `7/3`. [`fn_8011e020`], [`fn_8011e1a0`] and [`fn_8011e420`] share
  the `+0x60C`/`+0x61C` gate, the `+0x538` contact bits and the `+0x608` bit
  that chooses the four-name or single-name path, then call the response
  helpers by address or the shared submitter [`fn_8011eb00`].
- `0x8011EB00..0x8011EFE0` - the submitter and the three contact scans.
  [`fn_8011eb00`] writes the player's `+0x459`/`+0x38D`, the child's `+0x18`
  and the object's `+0x60C`, and optionally posts a positional sound through
  `0x800E20A0`. [`fn_8011ee80`] walks `+0x46C`, [`fn_8011eef0`] walks `+0x470`
  and returns the first name id, and [`fn_8011efe0`] walks `+0x474` with the
  height-side attack bits of `+0x10D4`.
- `0x8011F130..0x8011FED0` - attack strength, contact sound, post-hit reset and
  the two per-frame handlers. [`fn_8011f130`] implements the `-2`/`-1`/`0`
  strength ladder; [`fn_8011f2e0`] gates on `+0x608` bit 1 and three names;
  [`fn_8011f520`] posts sound `0x283`; [`fn_8011f630`] resets to the post-hit
  state; [`fn_8011f700`] runs the brake/contact frame; [`fn_8011f8d0`] follows
  the ground, selects the two height directions and can emit the existing
  [`wm_magma_wave_effect`]; [`fn_8011fc00`] applies the `+0x584..+0x58C`
  offsets; [`fn_8011fed0`] steps the `+0x360` state pair with the
  `0x802F4CA0`/`0x802F4C80` tables and the `+0x5F4` ground probe.

The r2-relative constants in the exports were resolved against `SDA2_BASE`
(`0x80433080`) and their values were read from `work/dol/main.elf`
(for example `-0x5B54` is `0.0f`, `-0x5AD8` is `0.7f`, `-0x63F4` is `24.0f`).
The r13 words are runtime state in BSS and are read through `SDA_BASE`
(`0x8042F680`).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8011D450` | `fn_8011d450` | 312 | medium |
| `0x8011D590` | `fn_8011d590` | 336 | medium |
| `0x8011DBB0` | `fn_8011dbb0` | 292 | medium |
| `0x8011DCE0` | `fn_8011dce0` | 8 | high |
| `0x8011DCF0` | `fn_8011dcf0` | 336 | medium |
| `0x8011DE40` | `fn_8011de40` | 8 | high |
| `0x8011DE50` | `fn_8011de50` | 172 | medium |
| `0x8011DF00` | `fn_8011df00` | 144 | medium |
| `0x8011DF90` | `fn_8011df90` | 144 | medium |
| `0x8011E020` | `fn_8011e020` | 380 | medium |
| `0x8011E1A0` | `fn_8011e1a0` | 640 | medium |
| `0x8011E420` | `fn_8011e420` | 600 | medium |
| `0x8011EB00` | `fn_8011eb00` | 176 | medium |
| `0x8011EBB0` | `fn_8011ebb0` | 96 | medium |
| `0x8011EDA0` | `fn_8011eda0` | 220 | medium |
| `0x8011EE80` | `fn_8011ee80` | 108 | medium |
| `0x8011EEF0` | `fn_8011eef0` | 228 | medium |
| `0x8011EFE0` | `fn_8011efe0` | 324 | medium |
| `0x8011F130` | `fn_8011f130` | 148 | medium |
| `0x8011F2E0` | `fn_8011f2e0` | 272 | medium |
| `0x8011F520` | `fn_8011f520` | 116 | medium |
| `0x8011F630` | `fn_8011f630` | 148 | medium |
| `0x8011F700` | `fn_8011f700` | 356 | medium |
| `0x8011F8D0` | `fn_8011f8d0` | 772 | medium |
| `0x8011FC00` | `fn_8011fc00` | 156 | high |
| `0x8011FED0` | `fn_8011fed0` | 424 | medium |

## Deliberate skips

- `0x8011D930` (236 bytes), `0x8011DA30` (120 bytes), `0x8011DAB0` (252
  bytes), `0x8011EC10` (248 bytes) and `0x8011F1D0` (268 bytes) begin with
  `unaff_GQR3` / `ldexpf` reconstruction blocks. Their scaled halfword loads
  cannot be recovered without the `GQR3` base, so they are called by raw
  address where needed and are not claimed as translated.
- `0x8011EA00` (168 bytes) is the shared dispenser callback. Its export loses
  the argument of the first `FUN_8011DCE0()` call, so the two compared child
  ids cannot be reconstructed honestly; the address is left for a pass that
  can recover the register.
- Addresses in the `0x80110000` bucket outside this run (for example the
  `0x80112440`/`0x801132C0` cluster and the `0x80114940` ice object already
  covered by wave 51) remain for their own passes.

## Verification

- Every address and byte size matches `functions.tsv`; the 26 sizes sum to
  6,916 bytes.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one body in `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv` (all 26 were untranslated
  entries of the `0x80110000` bucket).
- `cargo check --message-format=short` is free of errors for
  `src/game/worldmap.rs`.
- `node tools/check-lane-csv.mjs wave64` reports `OK` with this lane's 26 rows.
