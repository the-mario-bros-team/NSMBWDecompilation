# Coverage - Wave 210, lane C (`game::player`, the `0x80060FE0..0x800618B0` run)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 10 | 1928 | 9 | 1 |

Bodies: `work/wave210/laneC_block.rs`. Ledger rows:
`ledger/agents/wave210-laneC.csv`.

Byte count is the sum of the `functions.tsv` sizes for the ten addresses
(`152 + 92 + 36 + 72 + 420 + 12 + 436 + 264 + 384 + 60 = 1928`); see the size
notes below, where two of those numbers are not the real extents (the real
extents sum to `1572`).

## How to read this file

Ten functions from one contiguous run of the `0x80060000` bucket in
`work/ghidra/main-gb/export/functions.tsv`. All ten are `translated`; none is
deferred.

* **Address** - virtual entry point in the DOL image (not a file offset).
* **Name** - the Ghidra `FUN_<address>` name, kept as `fn_<address>` in Rust.
* **Confidence** - `high` everywhere except `0x80061080`, where the meaning of
  two global flags is inferred (the control flow itself is literal).

## The object families this run works on

### The four character slots (`0x80354E10..0x80354EA0`)

The tables were already named by the wave-88 lane and are reused unchanged:

| address | role |
|---|---|
| `0x80354E20` | four slot actor ids (`W88_SLOT_TABLE_B_80354E20`) |
| `0x80354E60` | slot -> character id (`W88_SLOT_CHAR_ID_80354E60`) |
| `0x80354E70` | per-character block word (`W88_BLOCK_80354E70`) |
| `0x80354E80` | per-character flag words (`W88_CHAR_FLAGS_80354E80`) |
| `0x80354E90` | per-character counters (`W88_CHAR_COUNTER_80354E90`) |
| `0x80354EA0` | per-character aux words (`W88_CHAR_AUX_80354EA0`) |

Three SDA words (`r13 = 0x8042F680`) drive the repair pass and are new to this
lane: `0x80427900` (counter limit), `0x80427904` (aux-total limit),
`0x80427908` (floor for `0x80429CC0`). The presence mask
`0x80429CA8` (`SDA_SLOT_MASK_80429CA8`), the slot state `0x80429CC4`
(`W98_HANDLE_WORD_80429CC4`) and `0x80429CC0` (`W88_WORD_80429CC0`) already
exist.

### The anchor object (`+0x3D0..+0x3E7`)

The two transform bodies and the notify hook work on the object family that the
wave-210 lane-E block already introduced (`+0x3D0` local offset, `+0x104` signed
16-bit facing angle, vtable at `+0x60`). This lane extends it: `+0x3D4`/`+0x3D8`/
`+0x3DC` are the three offset components, `+0x3E0` the cached angle, `+0x3E2`
the registered tag, `+0x3E4` the placement mode, `+0x3E5` the single-axis byte
and `+0x3E6`/`+0x3E7` two further tag bytes (written by the sibling initialiser
`0x80061820`, not in this lane).

## What this pass covers

- **The counter hand-off** - [`fn_80060fe0`] clears the caller's `+0x1036` byte
  and increments it on every other slot player (resolved through `0x80162D00`)
  whose byte is still below the old value. The self-id skip compares the slot
  table entry with the object's id word.
- **The filter predicate** - [`fn_80061080`] gates on the byte at `+0xAFC` of
  the object published in `0x80429F7C`, resolves the slot player through
  [`fn_8005f900`], and ANDs the SDA filter byte `0x80429CFA` with the player's
  `+0x38E` byte. Ghidra declares no parameter; `r3` is live-in and is modelled
  as `index`.
- **The mask bit test** - [`fn_800610e0`] is the `1 << (index & 0xFF)` test of
  `0x80429CA8`. It is the smallest body in the run and is why the PowerPC `slw`
  modulo-64 rule is spelled out explicitly rather than using `<<` on the full
  byte.
- **The slot predicate** - [`fn_80061110`] is the function the wave-109 lane
  already calls by address (`W109_SLOT_QUERY_80061110`) as the per-slot
  "already committed" test of the two spawn-placement passes. Bit `4` of the
  character's flag word plus a positive counter decide it.
- **The repair pass** - [`fn_80061160`] clamps the four character records and
  the two SDA words every frame. It is the only body in the run that calls
  `0x80060200` (the sum of the four aux words).
- **The getter** - [`fn_800613b0`] is another copy of the shared handler
  constant `0x80377A04`, reusing the wave-92 constant.
- **The dispatch** - [`fn_80061540`] tail-calls one of the four class scans at
  `0x800613C0` / `0x80061420` / `0x80061480` / `0x800614E0`.
- **The anchor pair** - [`fn_80061590`] records the local offset from the
  anchor (three modes) and [`fn_800616a0`] re-applies it, including the
  quarter-turn X/Y rotation for modes `2`/`3`.
- **The notify hook** - [`fn_800618b0`] calls `[[this+0x60]+0xE0](this)` when
  `+0x3E2` is set and always returns `1`.

## The trig helpers and the angle unit

The rotation in [`fn_800616a0`] uses the identities this file already fixed in
wave 103 and lane E: `0x80237C40` is the cosine and `0x80237BD0` the sine, both
taking a 256-units-per-turn argument, and the `lfs` from `r2 - 0x75E0`
(`0x8042BAA0`) holds `0x3B800000` = `1/256`, which converts the 65536-per-turn
signed 16-bit angle. The body reuses `W103_SIN_80237BD0` / `W103_COS_80237C40`
and the existing `w103_sin_cos` wrapper rather than adding new helpers.

## Size notes

`functions.tsv` sizes are used verbatim in the `# Original` lines because the
lane checker compares against them. Two of them need a prose correction:

* `0x80061540` is listed as 436 bytes, but its real body is 80 bytes
  (`0x80061540..0x8006158C`, `blr` at `0x80061588`). The Ghidra export for that
  address decompiled the four `b <scan>` targets in-line, which is what inflates
  the size and what makes the export show four `do`/`while` loops that are not
  in this function at all. The Rust body follows the disassembly.
* `0x800616A0` was listed as 16 bytes in the lane target list, but the prologue
  (`stwu r1,-0x40`) and the `blr` at `0x8006181C` confirm the `functions.tsv`
  value of 384 bytes (`0x800616A0..0x80061820`). The body follows the 384-byte
  extent.

The other eight extents agree with the disassembly, including the 12-byte
getter (8 bytes of code plus 4 bytes of padding).

## Verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to a scratch directory, the
lane block was appended to the copy's `src/game/player.rs`, and
`cargo check --lib` exited `0` there (23 warnings, all pre-existing and none
from this lane's bodies or from `src/game/player.rs`).
