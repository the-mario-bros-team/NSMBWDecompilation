# Coverage - Wave 180 lane C (`game::bases`, `0x806B_BB90`)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 1 | 320 | 1 | 0 |

Lane C of wave 180 holds the last `psq`/GQR body of its slice: it is the only
translated member, and there is nothing deferred.

## Body

- `0x806B_BB90` (320 bytes) - `fn_806bbb90`: target-facing update. Resolves the
  `+0x6D8` handle with `0x80162D00`, reads the target position with
  `0x8006CED0`, and feeds `(sample[1] - +0xB0, sample[0] - +0xAC)` to
  `0x801611C0`. The `+0x104` halfword is biased by `+0x4000`, stepped toward
  the returned angle by `0x64` per frame through `0x8015F4F0`, then stored back
  without the bias. The stepped halfword also drives the sine/cosine pair
  `0x80237BD0`/`0x80237C40` (GQR3 type 5 read, scaled by `0x807CACCC` =
  `0.00390625`) whose outputs, times the nibble-selected `0x807F6678` factor,
  become `+0xEC`/`+0xE8`; `+0xF0` is reset to `0x807CACB0`, `+0x100`/`+0x102`
  are cleared, and virtual slot `0xF0` of the `+0x60` table is invoked.

## Decoding notes

- The export's `ldexpf`/`unaff_GQR0` blocks are compiler-save noise and were
  not modelled; the body's only `psq` words are the two GQR0 prologue/epilogue
  spills (dropped) plus the two GQR3 `psq_l` reads, which are
  `value as u16 as f32`.
- `0x806B_BBFC` decodes as `extsh r4,r3` (X-form puts the source in bits 6-10
  and the destination in 11-15). `0x806B_9994` and `0x8016_1364` use the same
  idiom with `extsh r3,r3`, so the sign-extended angle is simply moved into
  `r4` for the `0x8015F4F0` step call; Ghidra's `sVar7` short cast agrees.
- `0x8015F4F0` was read out of `work/dol/main.elf`: it loads `*(i16*)r3`,
  returns `1` when it already equals the `r4` target, otherwise steps the value
  by at most `r5` and returns `0`. Its signature is therefore
  `(i16*, i16 target, i16 step)`.

## Deferred

None.
