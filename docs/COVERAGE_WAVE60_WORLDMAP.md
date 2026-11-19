# Coverage - Wave 60 (WorldMap slip/move state machine)

## How to read this file

Each row is one original `main.dol` function, reconstructed as a
`pub unsafe fn fn_<address>` body in `src/game/worldmap.rs`. The *address* is
the PowerPC virtual address from `work/ghidra/main-gb/export/functions.tsv`;
the *bytes* column is the exact size from that file; *confidence* matches the
`# Confidence` line of the body's doc comment. "high" means the guard, stores
and calls are literal in the export; "medium" means a field role or a callee
identity is inferred and called out in the body.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 30 | 4648 | 12 | 18 |

## What this pass covers

- `0x80134DE0..0x801354D0` - the effect spawn and action-state entry points.
  [`fn_80134de0`] builds a three-float scale vector from the `0x802F4EB8` table
  and submits it for controller modes `0xB` and `0xE`. The five entry helpers
  [`fn_80135220`], [`fn_801352c0`], [`fn_80135350`], [`fn_801353b0`] and
  [`fn_801354d0`] each write a `+0x14A4` state and send one controller sound
  (`0x3E`, `0x46`, `0x40`, `0x3F`, `0x40`). [`fn_80135190`] clamps the
  horizontal speed toward `+0x110`.
- `0x80135720..0x80135FC0` - the slide probes and their sound wrappers.
  [`fn_80135720`] is the landing gate, [`fn_80135c20`] chains the two GQR
  probes into state `0`/`2`, [`fn_80135e70`]/[`fn_80135ef0`]/[`fn_80135f20`]
  post the `0x127`/`0x128`/`0x129` sound family by `+0x1DEC`, and
  [`fn_80135f50`] gates sound `0xDD` on two ground thresholds.
  [`fn_80135fc0`] is the shared `+0x14FC` release.
- `0x80136000..0x80136340` - the attach gate, the approach selector and the
  teardown. [`fn_80136000`] tests the `0x9C`/`0xA3` bits and posts
  `0x803764F0` with `0`/`1`/`2`; [`fn_80136190`] clears three bits, zeroes
  seven motion fields and branches on `+0x14A0` into states `0`/`4`/`5`;
  [`fn_80136340`] clears the same bits and zeroes `+0x2A6C`/`+0x2A70`.
- `0x80136650..0x80137230` - the ten-case position machine. [`fn_80136650`]
  ages the halfword record at `+0x1DD8`/`+0x1DDA`/`+0x1DDC`; [`fn_801366e0`]
  sends sound `0x7E`; [`fn_80136710`] and [`fn_801367e0`] start the two slide
  moves; [`fn_801368f0`] runs states `0..=10` with the original fallthroughs;
  [`fn_80136bb0`], [`fn_80136c90`], [`fn_80136dd0`] and [`fn_80136ea0`] arm or
  enter states `1`/`2`/`3`/`4`; [`fn_80137130`] raises the actor onto the
  ground and enters state `6`; [`fn_80137230`] is the three-state landing
  hand-off.

All bodies are complete control-flow records. The controller calls use the
existing `+0x5C` helpers (two or three doubles plus a sound id), the actor
events use `vcall60_2(actor, 0x434, ...)`, and the `+0x60` vtable calls reuse
`vcall60_0`/`vcall60_f64`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80134DE0` | `fn_80134de0` | 228 | medium |
| `0x80135180` | `fn_80135180` | 4 | high |
| `0x80135190` | `fn_80135190` | 132 | high |
| `0x80135220` | `fn_80135220` | 148 | medium |
| `0x801352C0` | `fn_801352c0` | 144 | medium |
| `0x80135350` | `fn_80135350` | 92 | medium |
| `0x801353B0` | `fn_801353b0` | 140 | medium |
| `0x80135440` | `fn_80135440` | 140 | medium |
| `0x801354D0` | `fn_801354d0` | 92 | high |
| `0x80135720` | `fn_80135720` | 140 | medium |
| `0x80135C20` | `fn_80135c20` | 124 | medium |
| `0x80135E70` | `fn_80135e70` | 120 | high |
| `0x80135EF0` | `fn_80135ef0` | 44 | high |
| `0x80135F20` | `fn_80135f20` | 44 | high |
| `0x80135F50` | `fn_80135f50` | 104 | high |
| `0x80135FC0` | `fn_80135fc0` | 56 | high |
| `0x80136000` | `fn_80136000` | 388 | medium |
| `0x80136190` | `fn_80136190` | 428 | medium |
| `0x80136340` | `fn_80136340` | 124 | high |
| `0x80136650` | `fn_80136650` | 136 | medium |
| `0x801366E0` | `fn_801366e0` | 44 | medium |
| `0x80136710` | `fn_80136710` | 200 | high |
| `0x801367E0` | `fn_801367e0` | 260 | high |
| `0x801368F0` | `fn_801368f0` | 616 | medium |
| `0x80136BB0` | `fn_80136bb0` | 88 | medium |
| `0x80136C90` | `fn_80136c90` | 52 | medium |
| `0x80136DD0` | `fn_80136dd0` | 52 | medium |
| `0x80136EA0` | `fn_80136ea0` | 12 | high |
| `0x80137130` | `fn_80137130` | 256 | medium |
| `0x80137230` | `fn_80137230` | 240 | medium |

## Deliberate skips

- `0x80135070` (260 bytes), `0x80135810` (196 bytes), `0x801358E0` (224
  bytes), `0x801359C0` (608 bytes) and `0x801363C0` (644 bytes) are the five
  GQR-reconstruction records in this range. Their exports begin with
  `unaff_GQR0 & 0x3f00` / `ldexpf` pairs (or a removed unreachable block), so
  the scaled float loads cannot be reconstructed without the `GQR0` base. They
  are called by raw address from [`fn_80135c20`] where needed; none of them is
  claimed as translated.
- `0x801366E0` was originally a candidate skip because Ghidra prints
  "Could not recover jumptable ... Too many branches" at the virtual call. The
  decompiled body is complete (one controller-mode store plus one `+0x5C`
  call), so it was reconstructed at `medium` confidence and the call is reused
  by [`fn_80136710`], [`fn_801367e0`] and [`fn_801368f0`].
- `0x80136C90` and `0x80136DD0` carry the same Ghidra jump-table warning but
  are complete one-store-plus-one-call bodies; both are reconstructed at
  `medium`.
- Addresses in this bucket outside the `0x80134DE0..0x80137230` run belong to
  other clusters (the `0x8013A920` spin group, the `0x8013D010` landing group
  and the ice-object group) and were left for their own lanes.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv` (all 30 were taken from the
  166 untranslated entries of the `0x80130000` bucket).
- `cargo check --message-format=short` is free of errors for
  `src/game/worldmap.rs`.
