# Coverage - Wave 200, lane E (`game::bases`, `0x806F..0x8073`)

## Scope

Lane E of the Wave 200 batch audits 79 generated interface-thunk bodies that
belong to `src/game/bases.rs`. They are listed under key `E` in
`work/wave200/manifest.json` and emitted in `work/wave200/laneE_block.rs`.

- Count: **79** functions
- Range: `0x806FAB00` .. `0x8073ECE0` (inclusive, ascending)
- Body size: 48 bytes (12 instructions) for all 79
- Module: `game::bases`; target source `src/game/bases.rs`
- Addresses are static analysis addresses under the synthetic REL base
  `0x80600000`; they are not runtime addresses.

## Shape audit

Every body is the same shipped thunk around the `0x802DCBFC` dispatcher:

```
stwu  r1,-16(r1)     9421fff0
mflr  r0             7c0802a6
mr    r5,r3          7c651b78
mr    r3,r4          7c832378
stw   r0,0x14(r1)    90010014
addi  r12,r5,K       39850000|K
bl    0x802DCBFC     4bbe../4bbd.. (LK=1)
ori   r0,r0,0        60000000
lwz   r0,0x14(r1)    80010014
mtlr  r0             7c0803a6
addi  r1,r1,16       38210010
blr                  4e800020
```

The generated bodies model this as `interface_call(interface, object, K)`, the
existing Rust model of `0x802DCBFC` in `src/game/bases.rs` (descriptor at
`interface + K`, self-adjust word, slot index, table base, one `abi_call1!`).
No redesign was performed.

## Per-offset histogram

| `K` (bytes) | `K` (hex) | bodies |
|---|---|---|
| 12 | `0xC` | 2 |
| 24 | `0x18` | 39 |
| 36 | `0x24` | 38 |
| **total** | | **79** |

The two `K = 12` entries (`0x806FED50`, `0x80720DC0`) are emitted with the
literal `0xC` rather than `0x0C`; that is only source formatting and decodes to
the same immediate.

## Method

1. Parsed `work/wave200/manifest.json` key `E` (79 records, all `sz = 48`,
   all family `A`) and `work/wave200/laneE_block.rs`.
2. Cross-checked every emitted `fn_<addr>` and its `interface_call(..., K)`
   literal against the manifest: 79/79 present, 79/79 offsets equal, no
   duplicate or extra functions. Also confirmed ascending address order and
   that each doc block's two `interface + 0xNN` mentions and its
   `addi r12,r5,0xNN` confidence line agree with the code literal.
3. Re-read the raw words from `work/rels/elf/d_basesNP.elf` with
   `ppc159.js` (`LEN=12`, `STOP=0`) for all 79 addresses, and checked per
   address:
   - words 0-3 = `stwu r1,-16(r1)`, `mflr r0`, `mr r5,r3`, `mr r3,r4`;
   - word 5 decodes to `addi r12,r5,K` with `K` equal to the manifest offset;
   - word 6 is `bl` with `LK = 1` and computed target `0x802DCBFC`;
   - words 7-11 = `ori r0,r0,0`, `lwz r0,0x14(r1)`, `mtlr r0`,
     `addi r1,r1,16`, `blr`;
   - 12 words read, i.e. 48 bytes, matching both the manifest `sz` and the
     `size` column for that address in
     `work/ghidra/rel-d_basesNP/export/functions.tsv`.
4. Spot-checked 15 addresses individually with the single-address command form
   (`0x806FAB00`, `0x806FC960`, `0x806FED20`, `0x806FED50`, `0x80700F90`,
   `0x80704E00`, `0x80706D20`, `0x8070B040`, `0x80717AB0`, `0x80720570`,
   `0x80723BD0`, `0x8072A880`, `0x80730AC0`, `0x8073AB60`, `0x8073ECE0`);
   all matched, including the first and last lane addresses.
5. Confirmed a Ghidra export exists for all 79 addresses and that
   `interface_call` (the shared dispatcher model) is already defined in
   `src/game/bases.rs`.

Result: 79/79 bodies agree with the shipped instructions and with
`functions.tsv`.

## Corrections

**None.** No body in `work/wave200/laneE_block.rs` was wrong, so the file was
not modified. Two items are worth recording:

- `0x806FED50` and `0x80720DC0` spell their immediate as `0xC`; a naive
  `0x0[0-9A-F]` parser misses them, but they are correct.
- None of the 79 functions is present in `src/game/bases.rs` yet; the block is
  still awaiting the Wave 200 merge. Per the lane rules, `src/game/bases.rs`
  and the other lane blocks were not touched.

## Artifacts

- Generated block: `work/wave200/laneE_block.rs`
- Manifest slice: `work/wave200/manifest.json` (key `E`)
- Evidence exports: `work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`
- Ledger: `ledger/agents/wave200-lanee.csv` (79 rows)
