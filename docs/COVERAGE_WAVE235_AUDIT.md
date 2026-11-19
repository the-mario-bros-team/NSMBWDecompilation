# Audit - Wave 235 (batch 8, page-offset constant sweep)

## 1. The `0x80358104` question: my hypothesis was wrong, and that mattered

I asked lane A to verify a suspected `0x80358104` -> `0x80368104` error. The
premise was false:

```
0x8010FE80   lis  r3,0x8036
0x8010FE84   addi r3,r3,-32508   ; 0x7EFC
0x8010FE88   blr
```

`addi` **sign-extends** its immediate, so the constant is
`0x80360000 - 0x7EFC = 0x80358104`. `0x80368104` is the *unsigned* (`ori`)
reading - a decode artefact, not a code bug. Corroborated by the Ghidra export,
a second producer at `0x80097DC8`, and the fact that no instruction in the image
materialises `0x80368104`. `worldmap.rs` was correct and unchanged.

## 2. But the artefact had propagated into the source, and that was real

Register-tracking over 12,880 function starts validated 9,814 image-window
literals and found **11 wrong addresses in 15 sites**, all fixed:

- `src/boot/init_tables.rs`: `0x80368104` -> `0x80358104` (slot 115 tag);
  `0x80369C50/9C9C/9C90` -> `0x80359Cxx` (three producers agree);
  `0x8032B0CC`/`0x8032B080` -> `0x8031B0CC`/`0x8031B080`;
  `0x8035EA58/48/38/28/18` -> `0x8034EAxx` (five `lis`/`addi` pairs).
- `src/ui/hud.rs`: `0x80368084` -> `0x80358084` (two prose sites).

193 further page-neighbour hits were cleared individually as genuine distinct
objects (`0x803395D8` REL word vs `0x803295D8` vtable; `0x80347BF4`
`"Unknown HID event"` vs BSS `0x80357BF4`; `0x80324534`
`"daIce_c::StateID_Float"` vs the `dEn_c` array). 2,338 literals remain
unverifiable because their address is only reachable through a loaded base
pointer; none shows page evidence.

## 3. Bodies translated (58)

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| B | `.ctors` 127-140 | 12 | 792 |
| C | `.ctors` 141-154 | 14 | ~1,100 |
| D | `WM_PLAYER_VTABLE` 78+ | 12 | 400 |
| E | WorldMap vtable 56-65 | 10 | 1,892 |
| F | `dEn_c` 55-64 | 10 | 2,708 |

## 4. Notable decodes

- **Lane E recomputed every `rlwinm` mask** and corrected four eyeball errors:
  guard bit `0x2` (not `0x1`), mask pair clears `0x10000` (not `0x8000`),
  rising branch clears `0x38000000` (not `0x1C000000`), and slot 61's two nibble
  tests were swapped. It also flagged the `1.5f` (`0x80325348`) vs `3.0f`
  (`0x80325390`) block confusion.
- **Lane F showed why `extsb` matters**: slot 64's side table `0xFF` entry
  contributes `-1.0`, not `255.0`; without the sign-extension side 1 would
  saturate at `+6.0`.
- **Lane B's emulator check** re-evaluated the committed Rust and matched the
  DOL trace on all 52 stores and 7 calls, with a mutation test proving it is not
  vacuous.
- **Lane D** confirmed two descriptors (`0x8035476C`, `0x80354BEC`) lie in BSS,
  not in a DOL data segment.
- Dispatch arms in `dEn_c` close on themselves (type 3->slot 62, 5->63, 7->64).
- Slot 141 continues lane B's `0x80378010` at `0x80378028` - one startup
  structure built by two lanes.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `scripts/check.ps1` exit 0.
- Stale cross-lane text: `work/wave235/laneC_report.md` still prints the old
  `0x8035EA5x` values that its own body no longer contains after the fix.
