# Coverage - Wave 125 (`game::bases`)

## How to read this file

This page documents the Wave 125 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`** produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
`# Original` block in the source carries the same warning line. Cross-module
pointers (`0x8037xxxx`, `0x8042xxxx`) keep their absolute `main.dol` values.

Every cited file is a real Ghidra C export under
`work/ghidra/rel-d_basesNP/export/`, and every byte count was re-read from
`work/ghidra/rel-d_basesNP/export/functions.tsv`.

The `0x8072` bucket holds 221 functions. Before this pass 122 were translated,
leaving 99; this pass takes 30 of them, from `0x80722900` to `0x8072D8F0`.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee identity, a field role or a truncated
neighbour had to be inferred. Comments in the bodies keep
`// observed:` / `// inferred:` / `// unknown:` markers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 5,904 | 22 | 8 |

All 30 rows are `translated`; none is `partial`.

## What this pass covers

- **Peach ship actor (6 bodies).** The `0x5D0`-byte object allocated by
  [`fn_80722900`] and built in place by [`fn_80722970`] (vtable `0x8080A720`,
  accessor `+0x524`, resource `+0x540`, model `+0x544`, clip `+0x584`).
  [`fn_80722b00`] loads `g3d/peach.brres`, binds the `peach` pane and starts
  `ship_wait`; [`fn_80722de0`] copies twelve words of the object returned by
  `0x80162D50(0x1B, 0)` into `+0x7C`; [`fn_80722ea0`] scales virtual slot `+0x2BC` by the
  signed byte of `0x8042C1A0`; [`fn_80723360`] stores the facing byte and starts
  `ship_helpme2`.
- **`daPowBlock_c` state handlers (14 bodies).** The Shock/Shock2 interaction
  object at `0x80724020..0x807246D0` (transform rebuild with the `Drop` probe,
  bone-matrix read, slot copy plus `Shock` dispatch, child-list scan, and the
  two Shock/Shock2 contact gates), and the seven-state block's own handlers at
  `0x80724B60..0x80725830` (ground-touch latch, mid-air placement and mode 5
  arm, player rebind, the two yaw-delta tables `0x807CD1F0`/`0x807CD204`, the
  three-tier drop probe [`fn_80725270`], and the two `Wm_ob_powdown` impact
  bodies with 60- and 30-frame timers).
- **Fire-rotating-cannon tail (5 bodies).** [`fn_8072ba10`] finishes the frame,
  releases both `0x2290`-byte motion records and plays sound `0x2F9`;
  [`fn_8072bad0`] frees the `0xA4`-stride collision records on delete reason 2;
  [`fn_8072bb50`] runs both `+0x14` model releases; [`fn_8072bea0`] reads the
  clip matrix and [`fn_8072bf60`] rebuilds the `0xA4`-stride array from the
  variant table at `0x807CD4A0`. The model loader [`fn_8072b8d0`] and the
  constructor were already translated in earlier waves.
- **Underground-pori companion (2 bodies).** [`fn_8072c9b0`] allocates the
  `0x570`-byte object that shares `+0x3EC`/`+0x4D0`/`+0x4E0` with
  [`fn_8072cdb0`], then installs vtable `0x8080C680`, the model at `+0x500` and
  the clip at `+0x54C`; [`fn_8072d450`] is its paired destructor.
- **Two independent tails (3 bodies).** [`fn_8072d3b0`] starts the
  `cube_kinoko` cube (gravity, variant pane and the `0x807CD558` scale pair);
  [`fn_8072d6c0`] and [`fn_8072d8f0`] are the transform writer and destructor
  of the unnamed actor whose accessor/model/clip live at
  `+0x3D0`/`+0x3F0`/`+0x430`.

Start with [`fn_80725270`] for the richest branch tree, [`fn_807255d0`] for the
shared effect/sound sequence, and [`fn_8072c9b0`] for the constructor that
matches the already-translated `lift_zen_chika_pori` family.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80722900` | `fn_80722900` | 48 | high |
| `0x80722970` | `fn_80722970` | 116 | high |
| `0x80722B00` | `fn_80722b00` | 228 | high |
| `0x80722DE0` | `fn_80722de0` | 192 | medium |
| `0x80722EA0` | `fn_80722ea0` | 84 | high |
| `0x80723360` | `fn_80723360` | 136 | high |
| `0x80724020` | `fn_80724020` | 324 | medium |
| `0x80724170` | `fn_80724170` | 68 | high |
| `0x807243F0` | `fn_807243f0` | 104 | high |
| `0x80724580` | `fn_80724580` | 108 | high |
| `0x807245F0` | `fn_807245f0` | 220 | medium |
| `0x807246D0` | `fn_807246d0` | 252 | medium |
| `0x80724B60` | `fn_80724b60` | 148 | high |
| `0x80724C00` | `fn_80724c00` | 128 | medium |
| `0x80724C80` | `fn_80724c80` | 80 | medium |
| `0x80724F90` | `fn_80724f90` | 188 | high |
| `0x807250B0` | `fn_807250b0` | 188 | high |
| `0x80725270` | `fn_80725270` | 864 | medium |
| `0x807255D0` | `fn_807255d0` | 432 | high |
| `0x80725830` | `fn_80725830` | 408 | high |
| `0x8072BA10` | `fn_8072ba10` | 192 | medium |
| `0x8072BAD0` | `fn_8072bad0` | 128 | high |
| `0x8072BB50` | `fn_8072bb50` | 84 | high |
| `0x8072BEA0` | `fn_8072bea0` | 68 | high |
| `0x8072BF60` | `fn_8072bf60` | 148 | high |
| `0x8072C9B0` | `fn_8072c9b0` | 228 | high |
| `0x8072D3B0` | `fn_8072d3b0` | 160 | high |
| `0x8072D450` | `fn_8072d450` | 180 | high |
| `0x8072D6C0` | `fn_8072d6c0` | 220 | high |
| `0x8072D8F0` | `fn_8072d8f0` | 180 | high |

**Totals:** 30 bodies, 5,904 bytes (22 high, 8 medium).

## Deliberate skips

Read and rejected, with the reason:

- **`0x8072B670` (600 bytes).** The first `0x8007FA40` mover call prints only
  two of the four range doubles in the export; the two missing lanes cannot be
  proven, so the whole start body was left rather than guessed. Its tail links
  (`0x8072BF30`/`0x8072BF40`/`0x8072BF50`) are still documented by
  [`fn_8072bf60`].
- **Paired-single / GQR conversions.** `0x80722180`, `0x807241C0`, `0x80724460`,
  `0x807247D0`, `0x80724CD0`, `0x80725170`, `0x807268C0`, `0x8072BC40`,
  `0x8072C000`, `0x8072E520` and `0x8072FE10` mix `unaff_GQR0`/`unaff_GQR3`,
  `ldexpf` and `CONCAT13`/`CONCAT22` lane splits. Their stores are visible but
  which GQR slot feeds each conversion is not decidable, so no body was
  emitted. [`fn_80724020`], [`fn_80724c00`], [`fn_80724c80`] and
  [`fn_8072ba10`] reach `0x80724460`, `0x80724320`, `0x807242C0` and
  `0x8072C000` only by address and are therefore `medium`.
- **`halt_baddata` bodies.** `0x80720980`, `0x807211D0`, `0x80727180` and the
  tails of `0x807242C0`/`0x80724320` are truncated by bad instruction data.
- **`0x802DCBFC` forwarders.** `0x80720570`, `0x807205A0`, `0x80720D90`,
  `0x80720DC0`, `0x807228A0`, `0x807228D0`, `0x80723BD0`, `0x80723C00`,
  `0x8072C950`, `0x8072C980`, `0x8072DBA0` and `0x8072DBD0` are the 48-byte
  this-adjusting thunks already documented in earlier waves; they carry no
  Rust-visible behaviour.
- **Bare `blr` and constant getters.** `0x8072C1F0` is four bytes; the five
  getters `0x80721020` and `0x80722930..0x80722960` only return the shared
  descriptor `0x80377A04`. Both kinds were already recorded as translation
  noise by earlier waves.
- **Everything else.** The remaining untranslated `0x8072` addresses were not
  attempted: `0x807203D0..0x80721290`, `0x80721800`, `0x80722180`,
  `0x80722560..0x80722EA0`, the `0x80726260..0x8072B670` span,
  `0x8072C000..0x8072D3B0` and `0x8072E520..0x8072FE10`.

## Notes on inference

- The "Peach ship" and "`cube_kinoko`" labels are `// inferred:` from the
  `g3d/peach.brres` / `ship_wait` / `ship_helpme2` and
  `g3d/cube_kinoko.brres` strings. No class-name string survives for their
  constructors. The `daPowBlock_c` handler cluster is inferred from the
  `0x80830418..0x80830558` state descriptors registered by `0x80725BC0`, not
  from the bodies themselves.
- [`fn_80722de0`] is `medium`: the twelve-word copy and the flag are literal,
  but `0x80162D50` and `0x806BFDF0` are unprototyped.
- [`fn_80724020`], [`fn_807245f0`] and [`fn_807246d0`] are `medium`: the state
  queries at `+0x394` are explicit, but the `+0x14` return convention (0 when
  the queried name is current) is `// inferred:` from the paired Shock/Shock2
  probes, and `0x80724460` is address-only.
- [`fn_80724c00`] and [`fn_80724c80`] are `medium` because `0x80724320` and
  `0x807242C0` are truncated neighbours called by address.
- [`fn_80725270`] is `medium`: the three effect tiers, sounds `0x282`/`0x283`/
  `0x284` and every store are literal, but `0x80075270`'s return values are
  `// inferred:` from the sibling `0x806EC930` body.
- [`fn_8072ba10`] is `medium` because `0x8072C000` is GQR-rejected and
  `0x8072C1F0` is a bare `blr`.
- [`fn_8072c9b0`] is `high` on its own body (size, vtables, all three
  `0x802DC9A0` records); its pairing with [`fn_8072cab0`] is `// inferred:`
  from the shared `+0x3EC`/`+0x4D0`/`+0x4E0` layout.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 5,904 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 122 translated
  addresses were re-read first and none is duplicated here.
- `node tools/check-lane-csv.mjs wave125` reports `OK` for
  `wave125-bases.csv` (30 rows, 30 unique addresses).
- `cargo check --message-format=short` reports no error naming
  `src/game/bases.rs`.
