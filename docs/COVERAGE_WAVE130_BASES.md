# Coverage - Wave 130 (`game::bases`)

## How to read this file

This page documents the Wave 130 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`** produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
`# Original` block in the source carries the same warning line. Cross-module
pointers (`0x8035xxxx`, `0x8042xxxx`) keep their absolute `main.dol` values;
this REL's own `.bss` message cells keep their `0x8082xxxx`/`0x8083xxxx`
synthetic values.

Every cited file is a real Ghidra C export under
`work/ghidra/rel-d_basesNP/export/`, and every byte count was re-read from
`work/ghidra/rel-d_basesNP/export/functions.tsv` for this pass (the size column
of that TSV equals the reachable span for all 22 bodies here: each ends in a
`blr` in the last word of its extent).

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee identity, a field role or a local record
layout had to be inferred. Comments in the bodies keep `// observed:` /
`// inferred:` / `// unknown:` markers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 22 | 10,648 | 17 | 5 |

All 22 rows are `translated`; none is `partial`. The module header now reports
3307 bodies.

## What this pass covers

- **The twelve ready class-B bodies specified by `docs/SPEC_WAVE129_BASES.md`
  (12 bodies, 9,932 bytes).** `0x8070DC70` (fade byte, direction flip and the
  six-way arm table on the `s16` pair at `0x807CC908`/`0x807CC90C`),
  `0x806A53E0` (two-state carried-object update with the `0.9`/`0.8` scale
  ladder), `0x80684680` (carried contact, `0x130`/`0x135` spawn replies and the
  three class tokens), `0x8068E0B0` (spawn-word unpacker and the `0xE4`-stride
  pane sweep), `0x806FE480` (four-state `MSG` tutorial gate over eighteen
  `0x10`-byte records), `0x80632DC0` (pane-transform synchroniser with its two
  shapes), `0x806D3C20` (slot latch plus the `0.071`/`1.35` steering-rate
  ladder), `0x806A6C50` (intrusive-list erase walk), `0x807813F0`
  (three-point arc solver), `0x806DED20` (flipper spawn initialiser),
  `0x807807E0` (rider pose and hand attachment selector) and `0x80724CD0`
  (facing/turn update for a ridden object).
- **The `+0xA80` nine-row board family from the `0x8062` planner bucket
  (10 bodies, 716 bytes).** `0x80628770` (row-major index), `0x80628740` (cell
  finish and countdown), `0x80628940` (row wrap/occupancy adjust),
  `0x80629770` (row select and delay reload), `0x806298E0` (cell clear after
  message `0xA9`), `0x80629980` (cell resolve), `0x80628F20` (pane release),
  `0x80628EA0` (completion message `0x80825EE0`), `0x806291A0` (delay reset)
  and `0x8062BA30` (`0x807C7A18` descriptor getter).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8070DC70` | `fn_8070dc70` | 1196 | high |
| `0x806A53E0` | `fn_806a53e0` | 964 | high |
| `0x80684680` | `fn_80684680` | 936 | high |
| `0x8068E0B0` | `fn_8068e0b0` | 912 | medium |
| `0x806FE480` | `fn_806fe480` | 892 | high |
| `0x80632DC0` | `fn_80632dc0` | 884 | medium |
| `0x806D3C20` | `fn_806d3c20` | 784 | high |
| `0x806A6C50` | `fn_806a6c50` | 780 | medium |
| `0x807813F0` | `fn_807813f0` | 696 | high |
| `0x806DED20` | `fn_806ded20` | 688 | medium |
| `0x807807E0` | `fn_807807e0` | 604 | high |
| `0x80724CD0` | `fn_80724cd0` | 596 | high |
| `0x80628770` | `fn_80628770` | 24 | high |
| `0x80628740` | `fn_80628740` | 48 | high |
| `0x80628940` | `fn_80628940` | 88 | high |
| `0x80629770` | `fn_80629770` | 124 | high |
| `0x806298E0` | `fn_806298e0` | 84 | high |
| `0x80629980` | `fn_80629980` | 60 | high |
| `0x80628F20` | `fn_80628f20` | 80 | high |
| `0x80628EA0` | `fn_80628ea0` | 112 | medium |
| `0x806291A0` | `fn_806291a0` | 76 | high |
| `0x8062BA30` | `fn_8062ba30` | 20 | high |

**Totals:** 22 bodies, 10,648 bytes (17 high, 5 medium). The address window runs
from `0x80628740` to `0x807813F0` in ten separate clusters.

## Where the instruction stream corrects the specification

These are the places where the wave-129 prose disagreed with the bytes; the
bodies follow the bytes and say so at the call site:

1. **`0x8068E0B0` - the extra panes' flag word is `this + 0x584 + 0xE4*i`.**
   The export prints `*(uint *)(iVar5 + 0x584)` with `iVar5 = this + 0xE4*i`
   (`0x8068E3CC`), i.e. `pane - 0x14` for the pane at `0x4B4 + 0xE4*i`, not
   `+0x4A0 + 0xE4*i`. The two stack vectors handed to `0x8007FAD0` are
   `{-24.0, 32.0 + f28}` at the lower address and `{24.0, f28 - 8.0}` above it;
   the specification's `f28 + 1.0` is the `32.0 + f28` lane.
2. **`0x807813F0` - `0x807816B0` takes seven arguments, not eight.** The call
   site at `0x80781620..0x80781648` sets `r3 = &out`, `r4 = this`, `r5`/`r6` =
   the first two vectors and `r7 = i`; the third vector's register was
   overwritten by the loop counter at `0x807815F4` (`li r28,0`). Only the
   `u.z*v.x - u.x*v.z` cross component is tested before the sign fix.
3. **`0x80724CD0` - the spin path posts `0x80830498`.** `0x80724D7C` is
   `addi r4,r30,216` with `r30 = 0x808303C0`; the specification's prose said
   `0x80830458` there, and its own constant table already had `0x80830498`.
4. **`0x8068E0B0` - the nibble conversions use the plain `2^52`.** Both
   `0x807C9C40` reads are `0x4330000000000000`; only `0x806DED20` mixes the two
   variants (`0x807CBA60` = `2^52 + 2^31` for the signed `56`/`112`, the plain
   `0x807CBA68` for the unsigned spawn nibbles). The bodies keep them apart.

## Deliberate skips

Read and rejected, with the reason:

- **The `0x802DCBFC` forwarders in the bucket.** `0x8062A290`, `0x80624E80`,
  `0x806261E0`, `0x80626210`, `0x80623A50`, `0x80627600`, `0x80621FE0`,
  `0x8062F020`, `0x80621FB0`, `0x80623A80`, `0x8062EFF0` and `0x80627630` are
  all the identical `func_0x802dcbfc(param_2, param_2, param_1)` stub with no
  recoverable target; the module header already excludes that family.
- **`0x80625970` (24 bytes).** The export is `halt_baddata()` - bad
  instruction data, nothing to reconstruct.
- **`0x80628790`, `0x80628810`, `0x806288B0`.** Read and left for a later pass:
  each walks `0x800E0380`'s returned object through `0x800CE190` /
  `0x800CE3C0` with a `0x30`-byte record and only stores an aggregate word
  (`+0xAAC`), a flag (`+0xA84`) or a flag (`+0xA83`). Claiming them would mean
  inventing the record type those three main.dol callees expect.
- **The rest of the `0x80620000` bucket.** The remaining addresses were not
  attempted this pass; the 93 untranslated rows there are dominated by the
  forwarder and `halt_baddata` families above and by the larger multi-route
  bodies the wave-127 analysis already classed B/C.

## Notes on inference

- [`fn_8068e0b0`] is `medium`: the `0x807C9C08`/`0x807C9C10` tables are one flat
  array read with a two-word skew, so only indices 0/1 give a plausible integer;
  both reads are transcribed literally, the twelve-float `0x800A42B0` record is
  copied verbatim and the `0x807EF418` triples have no visible element type.
- [`fn_80632dc0`] is `medium`: the twelve-word blocks are copied by raw offset
  and only their three position lanes are used; path B's `bdnz` loop runs twice
  with `this += 8`, so it addresses two pairs of objects (`+0x64C`/`+0x650`,
  then `+0x654`/`+0x658`); the `0x80428732 == 3` selector is main.dol data.
- [`fn_806a6c50`] is `medium`: the emitted `for (j = count; j < count; j++)` at
  `0x806A6E98` is provably dead and is reproduced as a single comparison, and
  the first scan increments before its back-edge test.
- [`fn_806ded20`] is `medium` for the two multi-argument constructors
  (`0x800DB180` with five floats and five words, `0x800DF860` with nine words
  including one stack slot) and the two distinct `2^52` constants.
- [`fn_80628ea0`] is `medium`: `0x80628610`'s export prints no parameters, so it
  is called as `abi_call1!(0x8062_8610, this)` with the untouched receiver.
- New host-side helpers added by this pass: `s16_table` (the `lhax base,r0*2`
  form) and `call_f32x2_u32x5` (the seven-argument `0x807816B0` call).
- The `unaff_GQR0`/`ldexpf` blocks printed at the head and tail of
  `0x806A53E0`, `0x80684680`, `0x8068E0B0`, `0x80632DC0`, `0x806D3C20`,
  `0x807813F0`, `0x806DED20` and `0x80724CD0` are the prologue/epilogue spill
  noise described in `docs/REL_ELF_MAPPING.md`; every `psq_l`/`psq_st` in the
  twelve is a `qr0` `f28..f31` save/restore paired with an `stfd`/`lfd`, and no
  body has a data-side paired-single access or an `mtspr GQRn`.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (22 rows, 10,648 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 22 were checked against
  the ledger's address column before being written, and none is duplicated.
- `node tools/check-lane-csv.mjs wave130` reports `OK`;
  `wave130-bases.csv` has 22 rows and 22 unique addresses.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/game/bases.rs`.
