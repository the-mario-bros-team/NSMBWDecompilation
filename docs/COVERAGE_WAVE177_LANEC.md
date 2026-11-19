# Wave 177 lane C coverage (`game::bases`)

Four functions from REL `d_basesNP`, all translated into
`work/wave177/laneC_block.rs` (the coordinator appends the block to
`src/game/bases.rs`). Nothing in this lane was deferred.

## Translated

| Address | Bytes | Body |
| --- | --- | --- |
| `0x8075BB60` | 448 | Layer-class id collector: probes the `0x400`/`0x800` layer bit over candidates `0..0xC0`, then draws `count` of the matches through `0x800B2ED0`, skipping the "current" id (`[0x8042A1A0]+0x22C`) when the caller's byte flag is set. Returns `1`/`0`. |
| `0x80796850` | 448 | State dispatcher on `+0x27C` (13-entry jump table at `0x8081B7D4`), plus the unconditional `0x80C55FC0(this->0x29C)` tail and the `0x8042A24E` byte store into `[0x80429F7C]+0x40`. |
| `0x80651A60` | 452 | Pops the head of the 40-entry pending-effect queue at `0x80828430`, packs its two words into the 12-byte descriptor `{0x807E5408, w0, w1}` and calls `0x806514D0` (spawn, result to `+0x3D8`) and `0x80651830` (sprite bind). |
| `0x806CF400` | 452 | Adds a node offset to a transform handle: `body` shifts the second position component by `-90 * *(s16*)(+0x104) / 16384`, `tire_R`/`tire_L` shift the first component by the same conversion of `*(s16*)(+0x656)` (negated for `tire_L`). |

Reused existing helpers: `fn_80796a10`, `fn_80799340`, `fn_806514d0`,
`fn_806515b0`, `fn_80651830`. Everything else is an address-only call through
`abi_callN!`.

## Decoding notes worth keeping

- `0x8075BB60` has four arguments after the unused `this`: `out`, `count`,
  `class`, byte `flag`. This is the convention `fn_8075BD20` already documents;
  the `this` register is clobbered by the first `lis` and never read.
- The original accumulates candidates in an 88-byte (22-word) stack slot at
  `r1+8` but can collect up to 192 ids, so more than 22 matches would overwrite
  the saved registers. The translation widens the buffer to 192 words; results
  are identical for any run that fits the original slot.
- `0x80796850` case `8` (and every value `>0xC`) jumps straight to the tail.
  Cases `0`/`9`/`10` and `3`/`4` share two byte-different but equivalent thunks
  that both call `0x80796A10`.
- `0x80651A60`: the queue is 40 entries of 12 bytes at `+8`; the compaction
  copies only the first two words of each entry (seven entries per `0x54`-byte
  block, five blocks, then four tail entries) and clears the last entry's two
  words with `-1`. The 20-byte scratch slot handed to `0x806514D0` is
  uninitialised in the original and is zeroed here.
- `0x806CF400`: the second argument (`r4`) is never read. The
  `extsh`/`neg` pair at `0x806CF54C` disassembles with the scratch decoder's
  reversed `extsh` field order (`extsh r3,r0`, then `neg r4,r3`, then
  `mulli r5,r4,90`); the values used are the sign-extended halfword, matching
  both Ghidra's `sVar3 * -0x5a` and the sibling at `0x806CF5D0`
  (`neg r0,r0` / `mulli r0,r0,90`). The three halfword loads at `+0x100`,
  `+0x102`, `+0x104` also feed dead stack stores; only `+0x104` is consumed.

## Deferred

None.
