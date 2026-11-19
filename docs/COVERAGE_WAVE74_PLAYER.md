# Coverage - Wave 74 (`game::player`, the carry/throw manager)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 26 | 2732 | 21 | 5 |

## How to read this file

This document covers the Wave-74 slice of the `0x80050000` bucket of
`work/ghidra/main-gb/export/functions.tsv`: the object that reserves up to four
actor slot ids while a character is carried or thrown. Every body lives in
`src/game/player.rs`; the ledger rows are in `ledger/agents/wave74-player.csv`.
The module now holds three hundred twenty-five translated functions.

* **Address** - virtual entry point in the DOL image (not a file offset).
* **Name** - the Ghidra name. No original symbol table survives, so
  `FUN_<address>` is the honest identity; the Rust body keeps the
  `fn_<address>` name.
* **Role** - one plain sentence. Roles marked "inferred" come from shared field
  offsets and call shapes, not from a recovered symbol.
* **Status** - all rows are `translated`: the whole control flow and every call
  are present.
* **Confidence** - strength of the evidence, not code quality: `high` =
  instruction-level match, `medium` = a receiver, a field role or a callee
  identity had to be inferred.
* **Evidence** - the Ghidra C file under `work/ghidra/main-gb/export/`.

The bodies use the two SDA bases this module already fixes: `r2 = 0x80433080`
and `r13 = 0x8042F680`. So `r13 - 0x51D8` is `0x8042A4A8` (the `0x58`-byte
game-sound object that `crate::audio::ax` also uses), `r13 - 0x5988` is
`0x80429CF8` (the four-low-bit flag byte already documented for wave 14),
`r13 - 0x59D8` is `0x80429CA8` (the four-slot presence mask) and
`r13 - 0x59EC` is `0x80429C94` (the manager pointer already named
`MANAGER_80429C94`). `r2 - 0x7668` is `0x8042BA18`, whose meaning is
`// unknown:` - it is only ever copied verbatim.

## What this pass covers

- **Manager lifecycle (`0x8005B0F0..0x8005B4A0`, 8 bodies)** - the
  `(object, destroy)` release pair ([`fn_8005b0f0`], [`fn_8005b3e0`]), the
  constructor that publishes the object in SDA `0x80429C94`
  ([`fn_8005b3a0`], vtable `DAT_80309718`), the 40-store field initialiser
  ([`fn_8005b4a0`]) and its two partial variants ([`fn_8005b430`],
  [`fn_8005b470`]). Two of the eight are structural twins of bodies that wave 72
  already translated: [`fn_8005b230`] repeats the `+0x30` chain walk of
  [`fn_80041a00`], and [`fn_8005b310`] repeats the `prefix:suffix` comparison
  of [`fn_80041ae0`].
- **State helpers (`0x8005B550..0x8005B840`, 11 bodies)** - the per-frame
  dispatcher on the state word `+0x04` ([`fn_8005b550`]), the state pair writer
  and the idle transition ([`fn_8005b5c0`], [`fn_8005b5d0`]), the two equality
  tests built from `clz(diff) >> 5` ([`fn_8005b610`], [`fn_8005b630`]), the
  `0x50`-tick timer ([`fn_8005b690`]) with its expiry body that releases every
  carried actor through virtual slot `+0x12C` ([`fn_8005b6a0`]), the four-slot
  reservation search ([`fn_8005b780`]), the occupied-count test
  ([`fn_8005b7d0`]), the one-shot sound cue ([`fn_8005b810`]) and the
  "entry before this id" lookup ([`fn_8005b840`]).
- **Phase machine (`0x8005BCF0..0x8005C210`, 7 bodies)** - phase `1` raises
  event flag `0x67` on every reserved slot ([`fn_8005bcf0`]), phase `2` waits
  for the first slot that carries `0x65` but not `0x68` and then primes the
  cursor ([`fn_8005bd60`]), phase `3` hands the slots back one per `10` ticks
  ([`fn_8005be30`]), phase `4` waits for flag `0x6A` ([`fn_8005bf10`]), and
  [`fn_8005c090`] dispatches all six phases before the common timer tail. The
  two remaining bodies are the idle-with-notification helper
  ([`fn_8005c1a0`]) and the "no slot finished" predicate ([`fn_8005c210`]).

Start at [`fn_8005b3a0`] (the four-line constructor), then
[`fn_8005b4a0`] for the full field layout, then [`fn_8005c090`] to see how the
phases connect. For the partial variant with the per-slot filters, read
[`fn_8005bd60`] next to [`fn_8005be30`]: they are the two halves of the same
cursor walk.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8005B0F0` | `fn_8005b0f0` | 92 | high |
| `0x8005B230` | `fn_8005b230` | 224 | medium |
| `0x8005B310` | `fn_8005b310` | 136 | high |
| `0x8005B3A0` | `fn_8005b3a0` | 64 | high |
| `0x8005B3E0` | `fn_8005b3e0` | 72 | high |
| `0x8005B430` | `fn_8005b430` | 52 | high |
| `0x8005B470` | `fn_8005b470` | 36 | high |
| `0x8005B4A0` | `fn_8005b4a0` | 176 | high |
| `0x8005B550` | `fn_8005b550` | 96 | medium |
| `0x8005B5C0` | `fn_8005b5c0` | 12 | high |
| `0x8005B5D0` | `fn_8005b5d0` | 52 | high |
| `0x8005B610` | `fn_8005b610` | 20 | high |
| `0x8005B630` | `fn_8005b630` | 88 | high |
| `0x8005B690` | `fn_8005b690` | 12 | high |
| `0x8005B6A0` | `fn_8005b6a0` | 216 | medium |
| `0x8005B780` | `fn_8005b780` | 68 | high |
| `0x8005B7D0` | `fn_8005b7d0` | 60 | medium |
| `0x8005B810` | `fn_8005b810` | 32 | high |
| `0x8005B840` | `fn_8005b840` | 84 | high |
| `0x8005BCF0` | `fn_8005bcf0` | 108 | high |
| `0x8005BD60` | `fn_8005bd60` | 208 | high |
| `0x8005BE30` | `fn_8005be30` | 216 | high |
| `0x8005BF10` | `fn_8005bf10` | 180 | high |
| `0x8005C090` | `fn_8005c090` | 156 | medium |
| `0x8005C1A0` | `fn_8005c1a0` | 112 | high |
| `0x8005C210` | `fn_8005c210` | 160 | high |

Byte total: 2732. Address range covered: `0x8005B0F0..0x8005C210`
(non-contiguous; the gaps are the deliberate skips below).

## Deliberate skips and why

The bucket holds 106 untranslated functions after this pass.

* `0x8005B8A0` (1100 B), `0x8005C410` (692 B) - the phase-`0` and state-`2`
  handlers. Both are large dispatchers over the `+0x30`/`+0x48` float blocks
  and their own helper runs; each needs its own call-graph pass. They stay
  address calls inside [`fn_8005c090`] and [`fn_8005b550`].
* `0x8005BFD0` (192 B), `0x8005C6D0` (344 B) - both begin with
  `FUN_802DCD74()` and end with `FUN_802DCDC0()`, the CodeWarrior
  `__savegprlr`/restore pair this module documents as an artefact: the `u64`
  return really means "the argument registers are still live". `0x8005C6D0`
  additionally divides by an `f64` constant at `r2 - 0x7650`, so the receiver
  and the arithmetic cannot both be stated honestly from the C alone.
* `0x8005C2B0` (348 B), `0x8005C830` (256 B), `0x8005C9D0` (128 B),
  `0x8005D100` (380 B), `0x8005D280` (1064 B), `0x8005D880` (628 B),
  `0x8005DB00` (220 B) - the rest of this neighbourhood. `0x8005C830` also
  carries Ghidra's signed-division-by-10 idiom (`x/10 + (x>>31)`), whose exact
  PowerPC rounding the export does not make unambiguous; the others are
  dispatchers of the same family as the skipped handlers above.
* `0x8005B5C0`-family callers outside the run, and the `0x8005E4xx`/`0x8005E5xx`
  movement-bit leaves, were already translated by waves 63 and 72; no address
  in this pass duplicates them.
* The two slot resolvers `0x8005FB90` and `0x8005FB70` are still not
  translated: both are single tail calls into `0x80162D00` over the
  `0x80354E10`/`0x80354E20` tables, and Ghidra types them `void` although every
  caller consumes `r3`. They stay address calls (the module-level
  `SLOT_LOOKUP_8005FB90` constant and the new `W74_SLOT_ALT_8005FB70`) rather
  than a guessed contract. This also keeps the wave-63 bodies that already call
  through the constant unchanged.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/player.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` is free of errors and of warnings for
  `src/game/player.rs`.
- `node tools/check-lane-csv.mjs wave74` reports `OK` for
  `ledger/agents/wave74-player.csv` (26 rows, 7 fields each).
