# Coverage - Wave 148 (`game::bases`)

## How to read this file

This page documents the Wave 148 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`**; the Wii REL loader chooses
the real load address, so these numbers identify positions inside the REL image,
not runtime addresses. Every `# Original` block in the source carries the
`Synthetic base: 0x80600000; this is not a runtime address.` line.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee contract or a field role had to be named
rather than resolved.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 16 | 7,392 | 11 | 5 |

All 16 rows are `translated`; none is `partial`. The module header now reports
3691 bodies (3675 + 16). The `0x8079` planner bucket drops from 75 to 66
untranslated addresses (nine of the sixteen bodies live in it: the eight small
0x8079 bodies plus the course-route node toggle `0x8079A930`; the other seven are
the remaining wave-147 "ready" rows, which are spread over the whole module).

## What this pass covers

- **The eight "ready" rows of `docs/SPEC_WAVE147_BASES.md`** (6,628 bytes), each
  re-read against its export and the raw instruction stream:

  | cluster | body |
  |---|---|
  | `+0x544`-rider per-frame update | `0x8069BE20`: rider tick, `+0x5F8` clip-idle pair, `+0x348` geometry latch, five `+0xD30..+0xD34` flags, `+0x360` gate, shared orientation/animation tail |
  | 16x16 tile-run painter | `0x807373B0`: six `0x807CDBxx` tile tables, `0x80077860` 16.4 blits, `0x80070BF0` probe, bottom-tested 16-unit strip |
  | `obj_nut` post-load init | `0x80705C60`: bitfield extents, `0x807CC648` collision template, `+0x41C`/`+0x428` motion records, `0x8082F360` idle clip |
  | `teresa` model/animation loader | `0x80782F60`: `g3d/teresa.brres`, three panes at priority `0x12B`, three chr/clr/tex animation families, frame seed, model parenting |
  | world-map route installer | `0x80775190`: trailing-digit prefix scan, five `0x800F0xxx` camera-key helpers, `"Rstone"` midpoint effect `30` |
  | course-route node toggle | `0x8079A930`: mask/layer toggle, `0x80799A50`/`0x80799B00` sibling walk, two `0x80C56xxx` effect layers |
  | world-map cue picker | `0x807A7220`: child `0x298` spawn, four message records, world/save/group cue picker, cue `0x35` suppressed |
  | stage/world-map scene setup | `0x807B84C0`: scene words, four player panes, stage frame offset, eight owner-slot allocations |

- **Eight small `0x8079`-bucket bodies** (764 bytes):

  | cluster | bodies |
  |---|---|
  | constant predicate | `0x8079EF10` (`li r3,1; blr`) |
  | timer re-seed | `0x807982B0` (`+0x254` countdown; on zero copies `+0x1DA` into `+0x102`/`+0x108`) |
  | chain scans | `0x80793BC0` (`0x291` chain, low-nibble match), `0x8079BDF0` (slot bit OR-accumulated with `0x80781C50` over `+0x184`) |
  | world-map reset/release | `0x8079BCE0` (`0x80799960(root, 0, -1)` then the `+0x184` child release walk) |
  | small state helpers | `0x8079C2C0` (optional `0x8079C330` then vtable `+0x64` gate), `0x80796A10` (`+0x18C` clear, `+0x29C`/`+0x74` mode `4`, `+0x259` bit `0x10` split) |
  | cue poster | `0x80799C20` (`0x278` lookup, bit 16 selects `0x807BA810`/`0x807BA830`, cue `0x2A`) |

## Where the binary differed from the wave-147 prose

1. **`0x80705C60` stores `1` at `+0x428`, not `0xFFFF4001`.** The words at
   `0x80705E3C..0x80705E64` are `li r3,1` / `stw r3,0x428(r30)` /
   `addi r5,r4,-0x4000` (`0xC000`) / `addi r0,r3,0x4000` (`0xFFFF4000`). The
   wave-147 prose attached the last immediate to `+0x428`.
2. **`0x80705C60`'s `+0x420` and `+0x434` both evaluate to `0`.** Both arms load
   the *same* double `0x4330000000038000` (`2^52 + 0x38000`) and subtract
   `2^52` with `fsubs`; single precision rounds both operands to `2^52`, so the
   product with `[+0xDC] = 1.0` is `0.0` and `fctiwz` stores `0`. The wave-147
   prose claimed the second arm computed `-2^51` and stored `0x80000000`. The
   Rust keeps the shipped expression (and documents that it is zero) instead of
   inventing a fixed-point constant.
3. **`0x807B84C0`'s `base + 300` test is bit 30, not the sign bit.** The words at
   `0x807B861C..0x807B8630` are `xori r0,r3,1` / `srawi r4,r0,1` /
   `and r0,r0,r3` / `subf r0,r0,r4` / `rlwinm. r0,r0,1,31,31`. `rlwinm` with
   `SH=1, MB=31, ME=31` returns bit 30 of the `subf` result, so the frame is
   `base + 300` whenever that bit is set - the test is **reachable** for the
   player counts the body reads, contrary to the wave-147 "unreachable" note.
4. **`0x807A7220` clears `r25`, not `r26`.** At `0x807A72D0` the word is
   `3b200000` (`li r25,0`, the local spawn result), and the flag register `r26`
   is overwritten with the world byte at `0x807A72E8` (`lbz r26,0x8042A24D`)
   only inside the spawn test. The Ghidra export conflates the two into one
   `uVar6`; the Rust keeps them separate.

Two confirmations rather than corrections:

- `0x8069BE20`'s geometry latch really is `[+0xAC] >= [+0x750]`:
  `0x8069BECC..0x8069BEE4` loads `f1 = [+0xAC]`, `f0 = [+0x750]`, runs
  `fcmpo` and stores `1` on the `bge` path.
- `0x807373B0`'s four `unaff_GQR0` brackets are `f28..f31` `stfd` + `psq_st
  ...,qr0,0` save pairs only. There is no `qr3` and no quantised data load, so
  the wave-30/wave-101 "paired-single failure" rejection stays retired.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8069BE20` | `fn_8069be20` | 844 | high |
| `0x80705C60` | `fn_80705c60` | 840 | medium |
| `0x807373B0` | `fn_807373b0` | 844 | high |
| `0x80775190` | `fn_80775190` | 820 | medium |
| `0x80782F60` | `fn_80782f60` | 836 | high |
| `0x80796A10` | `fn_80796a10` | 140 | high |
| `0x807982B0` | `fn_807982b0` | 84 | high |
| `0x80799C20` | `fn_80799c20` | 108 | high |
| `0x8079A930` | `fn_8079a930` | 816 | medium |
| `0x8079BCE0` | `fn_8079bce0` | 96 | high |
| `0x8079BDF0` | `fn_8079bdf0` | 132 | high |
| `0x8079C2C0` | `fn_8079c2c0` | 104 | high |
| `0x8079EF10` | `fn_8079ef10` | 8 | high |
| `0x807A7220` | `fn_807a7220` | 816 | medium |
| `0x807B84C0` | `fn_807b84c0` | 812 | medium |
| `0x80793BC0` | `fn_80793bc0` | 92 | high |

**Totals:** 16 bodies, 7,392 bytes. The address window runs from `0x8069BE20` to
`0x807B84C0`.

## Notes on inference

- `0x80705C60` is `medium`: the three-word record at `+0x41C` and the four-word
  record at `+0x428` are passed whole to `0x8006D090`, so their field meaning is
  not proven, and the `+0x420`/`+0x434` conversions are an artefact of the
  shipped single-precision sequence. The two dead arms (`uVar14 == 2`, `== 3`)
  are documented in the body and not implemented, because the selector is a
  single `rlwinm` bit.
- `0x80775190` is `medium` only because the five `0x800F0xxx` camera-track
  helpers are named by address; their return-register use was read from the raw
  `r3` tests. The `0x802E1BD0` `n + 1` comparison width was read from its own
  export.
- `0x8079A930` is `medium` for the two `0x80C56600`/`0x80C565A0` effect-layer
  setters and the `+0x27C` byte, which are role-inferred. Every argument
  register and both mask tests are literal.
- `0x807A7220` is `medium` for the `0x800FB2D0`/`0x800FC370` gates and the
  `[0x80429F7C]+0x54/+0x44` meanings; all twelve cue ids are immediate.
- `0x807B84C0` is `medium` for the three manager constructors
  `0x80081D20`/`0x800D02D0`/`0x8008DC80`, named by address only, and for the
  assumption that the stage record is non-null, which the shipped code also
  makes (it dereferences `stage + 0xC` with no null guard).
- The float constants `16.0`, `8.0`, `1.0`, `0.5`, `12.0`, `2.0`,
  `160.0`, `-10000.0`, the tables `0x807CDAF8..0x807CDB54`, `0x807CF830`,
  `0x807CF85C`, `0x807CF868`, `0x807CF874`, `0x807CF38C`, `0x807CF390`,
  `0x807CF3B0`, `0x807CF458`, `0x807CC648`, `0x807CC6B4..0x807CC6D8`,
  `0x807D0658`, `0x807D065C` and the strings `0x80818808..0x8081887C`,
  `0x80817248` were read from `work/rels/elf/d_basesNP.elf` through
  `file_offset = (address - 0x80600000) + 0x1000`. The `0x8037xxxx`,
  `0x8042xxxx`, `0x8035xxxx`, `0x8083xxxx` and `0x8082B5AC` words are
  `main.dol`/REL BSS and are named by address.

## Deliberate skips

- **`0x80792A10` (4 bytes) and `0x8079EF00` (4 bytes)** are empty `blr` stubs
  with no observable effect, following the wave-147 precedent. `0x8079EF10`
  (8 bytes) was taken because it returns a value.
- **`0x80794BF0` (12), `0x8079B110` (16), `0x8079B1C0` (12)** truncate on bad
  instruction data (`halt_baddata`) in their exports; no honest body exists.
  `0x8079BD40` (12 bytes) is a wrapper that only calls the truncated
  `0x8079B1C0`, so it was left open too.
- **`0x807970A0` (128 bytes)** prints `/ 0` in the export: Ghidra lost the
  divisor register of the three `+0x1E0/+0x1E2/+0x1DE` differences. Writing it
  would mean guessing the divisor, so it is skipped rather than padded.
- **`0x80799960` (108 bytes)** ends in an unrecovered jump table
  (`0x807999C8`, "Too many branches"); its role and call shape are known but the
  dispatch arms are not, so it was not written.
- The remaining 66 `0x8079` bucket rows were not started; they are the natural
  next wave for this bucket.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (16 rows, 7,392 bytes),
  re-read from the TSV for this pass rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 16 were checked against
  the ledger's address column before being written, and the lane validator
  repeats that check.
- `node tools/check-lane-csv.mjs wave148` reports `OK` for
  `wave148-bases.csv`: 16 rows, 16 unique addresses, no duplicates.
- `cargo check --message-format=short` exits `0` and reports no diagnostic naming
  `src/game/bases.rs` (the parallel Wave 148 enemies lane's `E0384` was fixed
  before this lane finished, so the whole crate is clean).
- All 16 bodies were decoded from the raw words with
  `file_offset = (address - 0x80600000) + 0x1000`; none of them contains an
  undecodable word, and each extent ends in the single `blr` at
  `address + size - 4`.
