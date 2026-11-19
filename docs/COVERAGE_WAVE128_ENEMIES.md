# Coverage - Wave 128 (`game::enemies`, the `0x80A80000` bucket continuation)

## How to read this file

This page records the Wave 128 `game::enemies` lane: 29 functions of the
`0x80A80000` bucket of REL module 3, `d_enemiesNP`, appended to
`src/game/enemies.rs` as section 109 of that file. Every row of the
machine-readable mirror (`ledger/agents/wave128-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Data
constants are cited by address and read through `platform::read_u32` /
`read_f32` / the module's `read_f64`, never transcribed as invented values.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 29 | 3,972 | 25 | 4 |

The bucket holds 201 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 149 were already in
`ledger/functions.csv` when this pass started (wave 108 claimed the `fugu`
actor at `0x80A819E0..0x80A83200`). This pass claims 29 of the 52 open
addresses and leaves 23.

## What this pass covers

- **The four-clip effect helpers of an `iga`-family world-map enemy
  (`0x80A803F0..0x80A81150`, 15 bodies).** The actor owns four animation
  holders at `+0x584`, `+0x5BC`, `+0x5F4` and `+0x624`, binds the clips `walk`,
  `look`, `wakeUp` and `damage` from the string block
  `0x80B4D760..0x80B4D7A0`, and places its two smoke effects on the
  `+0x660`/`+0x664` position. The run adds the reset quad at
  `+0x164..+0x170` with the `+0x36D` byte and the `+0x120` mask, the
  `Wm_en_atitismoke` vtable emitter at `+0x66C`, the `Wm_en_igafirehit`
  spawner, the fifteen-word `0x800B8700`/`0x802DCA98` record, the four clip
  binds, the `damage` entry with its `+0xE8..+0xF0` stop, the `look` entry, the
  two holder-release tails, the two attack frame steps (button bits `1` and
  `4`, the `0x8042C1A4` side table and the `0x1F2`/`499`/`500` sounds) and the
  `wakeUp` entry with the `6.0` velocity and `-0.275` turn rate.
  `// inferred:` the `iga` in the two effect names, plus the
  `daEnIgaKuribo_c`/`daEnIgaPuku_c` state strings that follow in the same data
  block, place this actor in the `iga` family. Its class name is not printed
  in this run, so every body is described by offset.
- **The fifth state-descriptor skeleton and its `0xE68`-byte constructor
  (`0x80A83A30..0x80A83DE0`, 8 bodies).** Two record callbacks (the
  `0x8015F800`/`0x802B9280` pair), the `0x80357BF4` descriptor producer, the
  five-link `+0x30` owner walk, the `:`-suffix name predicate and two
  `0x802DCBFC` forwarders. The constructor allocates `0xE68` bytes, installs
  vtable `0x80B4E530`, builds the `0x40`-stride ten-element holder array at
  `+0x584`, the holder block `+0x804..+0xAB8`, the two `0xC`-stride
  ten-element arrays at `+0xC24`/`+0xC9C` and the sub-objects at `+0x86C`,
  `+0x994`, `+0xABC`, `+0xE2C` and `+0xE48`.
- **Six helpers of that constructor's actor (`0x80A84ED0..0x80A86510`, 6
  bodies).** The side-selected turn rate (`0.15`/`-0.15` in state `0xE`,
  otherwise `1.0`/`-1.0`), the damaged-speed selector over the `+0x274` mask and
  the `0x800665A0` query, the `+0x544` slot tail call, the `+0xDFF`/`+0xE03`
  fade-byte mismatch predicate, the `+0xDD4` hold countdown and the
  `0x8005FEF0` random-stamp decrement.

### Why four rows are `medium`

- **An unprinted owner probe.** [`fn_80a804d0`] reads the object at `+0x394`,
  calls its slot `+0x28` (receiver inferred) and then slot `+0x14` with the
  `0x80B733F8` state. The sibling `0x80A7FB50` shows the same shape, but the
  probe's own fields are never printed, so the record scales stay inferred.
- **An unprinted `+0xC` slot role.** [`fn_80a83be0`] walks five `+0x30` links.
  The raw words prove the slot is entered with `r3 = *(object + 0x30)`, but
  what the slot means is not printed by the export.
- **Two hidden-`r12` forwarders.** [`fn_80a83d80`] and [`fn_80a83db0`] are
  `0x802DCBFC(second, second, first)` thunks whose this-adjust pointer cannot
  be modelled, exactly like the wave-13 forwarders.

### Store-granularity note

CodeWarrior copies some constant pairs with `lfd`/`stfd`, which moves eight
bytes. Three bodies keep the resulting neighbour write and say so at the call
site:

- [`fn_80a80c60`] and [`fn_80a80d60`] copy `0x80B2E304`, so the `+0xEC` jump
  speed is accompanied by `5500.0` at `+0xF0`.
- [`fn_80a803f0`] stores the quad `+0x164..+0x170` from a four-step `stfd`
  chain; the final pair also covers `+0x174`, which the export does not name.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A803F0` | `fn_80a803f0` | 128 | high |
| `0x80A80470` | `fn_80a80470` | 96 | high |
| `0x80A804D0` | `fn_80a804d0` | 476 | medium |
| `0x80A806B0` | `fn_80a806b0` | 84 | high |
| `0x80A80710` | `fn_80a80710` | 112 | high |
| `0x80A80780` | `fn_80a80780` | 108 | high |
| `0x80A807F0` | `fn_80a807f0` | 112 | high |
| `0x80A80860` | `fn_80a80860` | 112 | high |
| `0x80A808D0` | `fn_80a808d0` | 128 | high |
| `0x80A809F0` | `fn_80a809f0` | 112 | high |
| `0x80A80A60` | `fn_80a80a60` | 160 | high |
| `0x80A80B10` | `fn_80a80b10` | 220 | high |
| `0x80A80C60` | `fn_80a80c60` | 208 | high |
| `0x80A80D60` | `fn_80a80d60` | 228 | high |
| `0x80A81050` | `fn_80a81050` | 228 | high |
| `0x80A83A30` | `fn_80a83a30` | 88 | high |
| `0x80A83A90` | `fn_80a83a90` | 92 | high |
| `0x80A83AF0` | `fn_80a83af0` | 12 | high |
| `0x80A83BE0` | `fn_80a83be0` | 224 | medium |
| `0x80A83CC0` | `fn_80a83cc0` | 136 | high |
| `0x80A83D80` | `fn_80a83d80` | 48 | medium |
| `0x80A83DB0` | `fn_80a83db0` | 48 | medium |
| `0x80A83DE0` | `fn_80a83de0` | 420 | high |
| `0x80A84ED0` | `fn_80a84ed0` | 60 | high |
| `0x80A85520` | `fn_80a85520` | 112 | high |
| `0x80A85590` | `fn_80a85590` | 16 | high |
| `0x80A85610` | `fn_80a85610` | 100 | high |
| `0x80A85C30` | `fn_80a85c30` | 56 | high |
| `0x80A86510` | `fn_80a86510` | 48 | high |

## Deliberate skips

- **The three GQR-damaged frame handlers of the first cluster.**
  `0x80A80E50` (220), `0x80A80F40` (264) and `0x80A81150` (456) build their
  angle words from `unaff_GQR0`/`unaff_GQR3` quantiser fiction and `ldexpf`
  calls that are not in the original code, so no honest body can be typed.
  They are the only members of the `0x80A803F0..0x80A81150` run left behind.
- **Truncated exports elsewhere in the bucket.** `0x80A85200` (92),
  `0x80A85EF0` (20), `0x80A863D0` (36), `0x80A86400` (32) and `0x80A8B0C0`
  (100) end in `halt_baddata`; the control flow past the truncation point is
  not recoverable from the export.
- **GQR-damaged bodies elsewhere in the bucket.** `0x80A84F10` (744),
  `0x80A87030` (732) and `0x80A88BC0` (368) quantise through the same
  `unaff_GQR*` fiction.
- **Readable bodies left for a later pass.** `0x80A84A00` (400),
  `0x80A84DB0` (204), `0x80A85260` (528), `0x80A85470` (168),
  `0x80A85650` (976), `0x80A8A910` (48), `0x80A8A940` (48),
  `0x80A8B9F0` (48), `0x80A8BA20` (48), `0x80A8BD20` (640),
  `0x80A8EB70` (48) and `0x80A8EBA0` (48) were readable but not needed once
  the 29-body target was met. Six of them are plain `0x802DCBFC`
  forwarders. Nothing was written for them, so no ledger row claims them.
  With them, 23 functions of the bucket remain untranslated.
- **No address already in `ledger/functions.csv` was touched**, and no body
  from another lane's file was edited.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 29 sizes sum to
  3,972 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 29 was in
  `ledger/functions.csv` before this pass.
- `cargo check --message-format=short` emits no error or warning naming
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave128` reports
  `wave128-enemies.csv: 29 rows`, 29 unique addresses and `OK`.
- The module header was updated: cluster item 109, the function totals (2,925
  to 2,954 public translations, 2,854 to 2,883 `fn_<addr>` bodies), the
  "start here" list and the records list.
