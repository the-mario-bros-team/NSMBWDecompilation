# Coverage - Wave 213, lane B (`game::player`, the `0x80069820..0x8006A1F0` sound-effect-record cluster)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 10 | 1148 | 10 | 0 | 0 |

Status split: 10 `translated`, 0 `partial`, 0 deferred.

Body block: `work/wave213/laneB_block.rs`. Ledger rows:
`ledger/agents/wave213-laneB.csv`.

## How to read this file

* **Address** - virtual entry point in the DOL image, not a file offset.
* **Name** - the Ghidra name of the body, which the Rust function keeps as
  `fn_<address>`; every new constant in the block is suffixed `_B`.
* **Size** - the `functions.tsv` value. All ten were re-derived from the
  prologue and the closing `blr` and match exactly; none is a 16/20-byte
  prologue artefact.
* **Confidence** - strength of the evidence: `high` here means every address,
  field offset, immediate and branch is literal in the disassembly.
* `r13 = 0x8042F680` and `r2 = 0x80433080` throughout, so the SDA stores read
  as `r13 - 0x51F8` = `0x8042A488`, `r13 - 0x5924` = `0x80429D5C`,
  `r13 - 0x5920` = `0x80429D60`, `r13 - 0x591C` = `0x80429D64`,
  `r13 - 0x7D52` = `0x8042792E`, `r13 - 0x5917` = `0x80429D69`,
  `r13 - 0x5916` = `0x80429D6A`, `r13 - 0x5915` = `0x80429D6B`.

## What this pass covers

### The record constructor `0x80069820` (420 B, high)

Three heap records built by the same pattern:

| slot | size | base vtable | final vtable | timer pair | `0x80275570` | `+0x54`/`+0x58` |
|---|---:|---|---|---|---|---|
| `0x80429D5C` | `0x17C` | `0x8030F154` | `0x8030F120` | `0x800699D0` / `0x80069A00`, `0x14`, `0x0E` | `(p, 0, 0x0C)` | `0x0E` / `1` |
| `0x80429D60` | `0x17C` | `0x8030F154` | `0x8030F0B8` | `0x800699D0` / `0x80069A00`, `0x14`, `0x0E` | `(p, 0, 0x0C)` | `0x0E` / `1` |
| `0x80429D64` | `0x74` | `0x8030F0F0` | `0x8030F058` | `0x80069B60` / `0x80069B70`, `4`, `4` | `(p, 0, 0x02)` | `4` / `1` |

Each record calls `0x80197300(p, 1, *(*(0x8042A488) + 0x5BC))` first, stores the
base vtable at `+0x00`, registers its initialiser/destructor pair through
`0x802DC9A0`, runs `0x80275570`, writes the two size words, and only then
overwrites `+0x00` with the derived vtable. The base-class vtable store is
therefore dead in the image but is reproduced literally. A failed allocation
writes `0` into the slot and skips that record's construction.

Ghidra labels the two initialisers `LAB_800699D0` and `LAB_80069B60` because
they are only ever seen as immediate operands; both are real code (36 and 12
bytes ending in `blr`), and their addresses are what the constructor stores.

### The flush/free destructor triad `0x80069A00` / `0x80069B70` / `0x80069F30` (88 B each, high)

Byte-identical: if the object is non-null, call `0x8027A200()` and then, when
the signed flag is `> 0`, `0x802B9280(object)`; return the object unchanged.
The extra `beq` at `0x80069A24` (and its twins) targets the following
instruction and is a dead compiler artefact.

### The teardown pair `0x80069C50` / `0x80069CF0` (160 B / 124 B, high)

* `0x80069C50` walks the three SDA slots, calls `vtable[+0x08](record, 1)`
  (the deleting destructor) on each non-null record, and clears the slot. The
  middle `beq` in each block re-tests the proven non-zero slot and never fires.
* `0x80069CF0` unregisters the `+0x64` timer pair with `0x802DC9A0(…, 0x14,
  0x0E)`, frees the effect record with `0x80197380(object, 0)`, and then frees
  the block with `0x802B9280(object)` when the flag is `> 0`. That last test is
  outside the null gate in the shipped code, so `(0, flag > 0)` still reaches
  the deallocator; the translation keeps that.

### The configuration block `0x8006A110..0x8006A1F0` (28 / 136 / 8 / 8 B, high)

* `0x8006A110` resets the halfword at `0x8042792E` to `0x78` and clears the
  three bytes at `0x80429D69..0x80429D6B`.
* `0x8006A130` refills them: `0x80196690(ui)` gives the halfword (replaced by
  `0x78` when its low 16 bits are zero, which is exactly the `rlwinm.` test),
  and `0x80196820` / `0x80196830` / `0x801969B0` give bytes 0/1/2. When
  `0x801021B0()` is non-zero the word at `*(0x803741D0) + 0xA0` overrides:
  `byte1 = w & 0xFF`, `halfword = w >> 16`, `byte0 = (w >> 8) & 0xFF`; byte 2
  keeps its accessor value.
* `0x8006A1C0` and `0x8006A1F0` are single `lbz` leaves returning
  configuration bytes 0 and 1.

## Verification

The block was appended to a scratch copy of `src/` + `Cargo.toml` +
`Cargo.lock` and `cargo check --lib` exited 0. The crate's 23 existing warnings
are unchanged; none refers to `src/game/player.rs` or to this block.

## Not covered here

The neighbouring bodies in the same run (`0x800699D0`, `0x80069A60`,
`0x80069B60`, `0x80069D70`, `0x80069F90`, `0x8006A010`, `0x8006A2B0`, …) belong
to other lanes. This lane only adds the ten addresses listed in
`ledger/agents/wave213-laneB.csv`.
