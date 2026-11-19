# Coverage - wave 227, lane D (pointer-table backlog)

Audit-only lane. No Rust source, no ledger CSV, no `functions.tsv` row was
changed; nothing was translated and `tools/merge-ledger.mjs` was not run.

## Inputs

- `work/wave226/laneC_untabled_entries.csv` - 2,457 prologue-verified function
  starts outside `functions.tsv` (`address,size,region_start,pointer_word_at`).
- `work/wave226/laneC_regions.json` - the 100 largest tsv gaps that own those
  entries (region, gap size, entry count, pointer-word count).
- `work/dol/main.elf` - 13 program headers; text `0x80004000..0x802ED9E0`,
  pointer data in segments 4/6/7.

## Work done

1. Re-scanned every 4-byte word of every `PT_LOAD` segment and built **maximal
   runs of text pointers** (`0x80004000..0x802ED9E0`, 4-byte aligned). A run is
   the smallest defensible definition of "the pointer table that owns this
   entry"; a non-pointer word always ends a run so embedded descriptors and
   sentinel words do not glue two arrays together.
2. Mapped every CSV entry to every run containing a word equal to its address
   (1,950 of 2,457 entries are owned by a run; 367 of those appear in more than
   one run and are counted once).
3. Resolved reachability for every run:
   - tier 1 - base loaded by a `lis`+`addi`/`ori` pair inside a translated
     `main.dol` body, or present as a literal in `src/**/*.rs`;
   - tier 2 - base loaded only from untabled code, stored as a data pointer, or
     adjacent (<=0x100 B) to a tier-1 run;
   - tier 3 - no reference found.
4. Ranked by tier, then by summed prologue-to-`blr` size of the owned entries.
5. Attributed roles from the `///` docs already attached to the table address in
   `src/`, the module of the translated slots in the same run, and printable
   strings within +-512 B.

## Results

| metric | value |
|---|---:|
| entries | 2,457 |
| owned by a pointer run | 1,950 |
| distinct runs with owned entries | 826 |
| ... tables of >=3 words | 494 |
| ... 1-2 word slots outside `.text` | 187 |
| ... single-word pointer constants inside `.text` | 145 |
| call-site-only entries (72 gaps) | 92 |
| no reference found | 415 |
| summed owned body size | 487,260 B |
| tier 1 / 2 / 3 runs | 12 / 18 / 651 |

Top groups (full detail in `docs/UNTABLED_POINTER_TABLES.md`):

| table | entries | bytes | role |
|---|---:|---:|---|
| `0x802ED9E0` | 76 | 58,612 | DOL constructor table (`.ctors`), 181 pointers + 3 zeros |
| `0x802F8EF0` | 20 | 5,272 | nw4r `scn` class tables (six 7-entry tables) |
| `0x80343E90` | 9 | 4,840 | WPAD/KPAD extension callback table |
| `0x80314534` | 22 | 3,584 | `game::stage` object vtable, 68 slots |
| `0x80310DB0` | 3 | 1,744 | tail of the collision-area vtable block |
| `0x803280E0` | 15 | 1,112 | SDA-copied pause-window/boot state word tables |
| `0x802F8510` | 6 | 804 | nw4r animation curve/kernel table used by `fn_80241690` |
| `0x802EE830` | 1 | 696 | single `.sdata` pointer slot (enemy-66 record) |
| `0x802EF328` | 1 | 400 | player-cluster function table |
| `0x802F6D30` | 3 | 204 | HID-host method table |

The largest tier-3 blocks are contiguous actor state tables whose loader was not
reconstructed: `0x80325428` (276 slots, 83 entries, 21,596 B), `0x80314068`
(`dEn_c` base state, 59 entries, 12,788 B), `0x803084D0` (player state, 44
entries, 11,352 B) and `0x8032679C` (`Wm`/Yoshi, 64 entries, 11,088 B).

## Verification

- `node work/wave227/laneD_tables.mjs` regenerates
  `work/wave227/laneD_groups.json` and prints the tier-1 head.
- `node work/wave227/laneD_doc.mjs` regenerates `docs/UNTABLED_POINTER_TABLES.md`
  from that JSON; the accounting closes: 1,950 owned + 92 call-site-only + 415
  unreferenced = 2,457.
- `node tools/doc-lint.mjs src` - unchanged result (no `src` file touched).

## Findings worth carrying forward

- The `src/runtime/os.rs` note for the constructor table used to say "185 words,
  184 code pointers"; the ELF has **181 pointers from `0x802ED9E0` to
  `0x802EDCB0` followed by three zero words at `0x802EDCB4..0x802EDCC0`**. Wave
  237 corrected the `INIT_TABLE` note to match.
- 415 entries have no reference at all in `main.dol` text. They are secondary
  entry points of larger untabled bodies (cold blocks, per-class second halves)
  rather than separate functions; they will only become reachable once their
  owning body is translated.
