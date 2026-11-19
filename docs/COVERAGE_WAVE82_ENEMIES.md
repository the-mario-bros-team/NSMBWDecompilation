# Coverage - Wave 82 (enemies: `0x80B00000` bucket continuation)

## How to read this file

This page records the Wave 82 `game::enemies` lane: the second pass over the
`0x80B00000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs`. Every row below corresponds to one real
`pub unsafe fn fn_<addr>` body in that file, and the evidence column of the
machine-readable mirror (`ledger/agents/wave82-enemies.csv`) names the Ghidra C
export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Data
constants are cited by address and read through `platform::read_u32` /
`read_f32` / `read_u16` / `read_u8`, never transcribed as invented values.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 4,056 | 17 | 9 |

Four rows are `low` confidence: the 48-byte argument-swapping forwarders
`0x80B069F0`, `0x80B06A20`, `0x80B086D0` and `0x80B08700`, whose hidden-`r12`
this-adjust pointer is not modelled (the same limitation already recorded for
the `0x802DCBFC` copies in waves 75, 80 and 81).

The bucket holds 207 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 84 were already in
`ledger/functions.csv` before this pass (the penguin run
`0x80B00050..0x80B02860` and the rail/segment block
`0x80B03780..0x80B06120`), 30 are claimed here and 93 remain.

## What this pass covers

One contiguous run, `0x80B068D0..0x80B08B40`, in two halves.

**Cluster 1, `0x80B068D0..0x80B080B0`, the `0x6E0`-byte star-coin collectible.**
The run opens with the shell helpers this REL repeats around every actor family:
a freeing helper (`0x8015F800` + release), two 48-byte argument-swapping
forwarders, the `0x6E0`-byte constructor with class vtable `0x80B669D0`, and the
`g3d/star_coin.brres` binder. The middle is the placement machinery: the
class-`0x23C` chain search, the word store plus its resolve-and-place step, the
`+0x528` transform placement/read-back pair, the three-slot occupancy test, the
spawn gate, the second occupancy test that also probes the scene, and the
`+0x1EC` controller seed. The tail is the per-frame handler with its effect,
two sound requests and rumble, the `+0x6C2` gate, the contact callback, the
floor probe, the three-way `+0x6C0` step and the four-frame speed ladder.

**Cluster 2, `0x80B085B0..0x80B08B40`, the `0x6A8`-byte sibling.**
The same shell (freeing helper, two forwarders), the same constructor layout
with class vtable `0x80B66DB4` instead of `0x80B671AC`, the entry that installs
heading `0xC000` and the two controller blocks, and the three
`0x2E9`/`0x2EA`/`0x2EC` sound wrappers built on the random image-plane pair.

### Identity evidence

- **The model strings were read, not guessed.** The export names
  `s_g3d_star_coin_brres_80b66998` (`g3d/star_coin.brres`),
  `s_star_coin_80b669ac` (`star_coin`) and `s_Wm_ob_starcoinget_80b669b8`
  (`Wm_ob_starcoinget`). The three clip names come from the pointer array at
  `0x80B66990`, printed as `s_star_coinA` and neighbours.
- **The five-entry skeleton is the documented one.** The constructor at
  `0x80B08730` writes exactly the stores and vtable words that its two
  still-untranslated siblings at `0x80B095B0` and `0x80B0B610` contain in the
  export (`0x80305F10` at `+0x60`, the
  `0x803295D8`/`0x80329578`/`0x80329548` holder words), so only the class word
  differs between the copies.
- **The per-player slot table is shared.** `0x803741B0` is the three-word
  table whose value `4` means "empty": `0x80B073A0` tests one word,
  `0x80B07530` counts all three and `0x80B07580` counts them after a scene
  probe. `0x80B07530`'s `clz(count - 3) >> 5` is the "all three used" answer.
- **The two scene helpers are main.dol.** `0x800E0380(0x8042A040, -1)` seeds a
  scene query whose result is consumed by `0x800CE3C0`; `0x802DCD6C` returns
  the running actor and `0x802DCDB8` receives the computed flag, exactly the
  pair already documented for `fn_80a740a0`.
- **The `+0x2B0` virtual is a float.** The three sound wrappers promote its
  result to `double` and truncate it to a signed halfword, so the body reads it
  with `call1_ret_f32` and casts `as i32 as i16 as i32`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80B068D0` | `fn_80b068d0` | 88 | high |
| `0x80B069F0` | `fn_80b069f0` | 48 | low |
| `0x80B06A20` | `fn_80b06a20` | 48 | low |
| `0x80B06A50` | `fn_80b06a50` | 108 | high |
| `0x80B071E0` | `fn_80b071e0` | 92 | medium |
| `0x80B07240` | `fn_80b07240` | 12 | high |
| `0x80B07250` | `fn_80b07250` | 160 | high |
| `0x80B072F0` | `fn_80b072f0` | 88 | high |
| `0x80B07350` | `fn_80b07350` | 76 | high |
| `0x80B073A0` | `fn_80b073a0` | 36 | high |
| `0x80B073D0` | `fn_80b073d0` | 152 | medium |
| `0x80B07470` | `fn_80b07470` | 184 | high |
| `0x80B07530` | `fn_80b07530` | 72 | high |
| `0x80B07580` | `fn_80b07580` | 208 | medium |
| `0x80B07650` | `fn_80b07650` | 556 | medium |
| `0x80B07880` | `fn_80b07880` | 60 | high |
| `0x80B078C0` | `fn_80b078c0` | 184 | medium |
| `0x80B07C20` | `fn_80b07c20` | 100 | medium |
| `0x80B07CE0` | `fn_80b07ce0` | 100 | high |
| `0x80B07ED0` | `fn_80b07ed0` | 328 | high |
| `0x80B08020` | `fn_80b08020` | 128 | high |
| `0x80B080B0` | `fn_80b080b0` | 176 | high |
| `0x80B085B0` | `fn_80b085b0` | 88 | high |
| `0x80B086D0` | `fn_80b086d0` | 48 | low |
| `0x80B08700` | `fn_80b08700` | 48 | low |
| `0x80B08730` | `fn_80b08730` | 224 | high |
| `0x80B08810` | `fn_80b08810` | 168 | high |
| `0x80B08950` | `fn_80b08950` | 148 | medium |
| `0x80B089F0` | `fn_80b089f0` | 164 | medium |
| `0x80B08B40` | `fn_80b08b40` | 164 | medium |

## Deliberate skips

- **`0x80B07C90`** (36 bytes) is a `halt_baddata()` body: Ghidra could not
  decode the instructions. Nothing can be translated. The one call site, inside
  [`fn_80b07650`], is kept as an address with an `// unknown:` comment, which is
  why that body is `medium` rather than `high`.
- **`0x80B07D70`** (252 bytes) decodes with two removed-unreachable blocks and
  `GQR` paired-single artefacts (`ldexpf` noise, `unaff_GQR0`/`unaff_GQR3`).
  The visible stores cannot be separated from the truncation, so the body was
  left rather than guessed.
- **The rest of the bucket.** The remaining 93 bucket addresses are the two
  skipped functions inside this run plus six below it
  (`0x80B04A90..0x80B05B80`) and 85 above it (`0x80B08B40..0x80B0FF50`,
  including `0x80B0C540`/`0x80B0C900` at 848 and 1,140 bytes). Those 85 lie
  outside this lane's 30-body budget; the `0x80B091F0..0x80B099C0` and
  `0x80B0A050..0x80B0A4B0` copies of the same five-entry skeleton are still
  open.
- **The rest of the bucket.** `0x80B080B0` to `0x80B085B0` is padding, and
  `0x80B08B40..0x80B0FF50` (93 functions, including `0x80B0C540`/`0x80B0C900`
  at 848 and 1,140 bytes) is untouched; it lies outside this lane's 30-body
  budget. The `0x80B091F0..0x80B099C0` and `0x80B0A050..0x80B0A4B0` copies of
  the same five-entry skeleton are still open.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 30 sizes sum to
  4,056 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none was already in
  `ledger/functions.csv` (the bucket's 84 existing rows are untouched).
- `node tools/check-lane-csv.mjs wave82` reports 30 unique addresses for this
  CSV and `OK` for the lane.
- `cargo check --message-format=short` produces no diagnostic naming
  `src/game/enemies.rs`.
