# Coverage - Wave 134 (`game::bases`)

## How to read this file

This page documents the Wave 134 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`** produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
`# Original` block in the source carries that warning line. Cross-module cells
(`0x8035xxxx`, `0x8042xxxx`, `0x8080xxxx`) keep their absolute `main.dol`
values; this REL's own `.data` cells keep their `0x807Cxxxx` / `0x8081xxxx` /
`0x8082xxxx` synthetic values.

Every cited file is a real Ghidra C export under
`work/ghidra/rel-d_basesNP/export/`, and every byte count was re-read from
`work/ghidra/rel-d_basesNP/export/functions.tsv` for this pass. Every body's
last word was also re-read from `work/rels/elf/d_basesNP.elf` at
`(address - 0x80600000) + 0x1000` and is `0x4E800020` (`blr`) at
`entry + size - 4`.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee contract, a field role or a `main.dol`
cell's meaning had to be inferred. Comments in the bodies keep
`// observed:` / `// inferred:` / `// unknown:` markers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 25 | 4,076 | 7 | 18 |

All 25 rows are `translated`; none is `partial`. The module header now reports
3412 bodies (3387 + 25).

## What this pass covers

- **The eight remaining ready bodies of `docs/SPEC_WAVE133_BASES.md`
  (2,096 bytes).** The wave-133 translation lane had already landed
  `0x80780D00`, `0x80711590`, `0x80643D40` and `0x806B39B0`, so this pass took
  the rest of that specification:

  | cluster | bodies |
  |---|---|
  | probe / copy | `0x806329E0` highest-slot probe copied into six panes |
  | moving sub-object | `0x8065C100` three-float record, `0x8065C260` message-796 dispatch |
  | reset | `0x806FCDF0` cursor reset with the `rand(1000)` warm-up |
  | construction | `0x806FFB50` `0x398C`-byte four-array constructor |
  | emitters | `0x807241C0` two-axis emitter plus ten-argument effect |
  | predicates | `0x807485D0` stage-rectangle point test, `0x807811D0` 45-degree angular test |

- **Seventeen `0x806C` planner-bucket bodies (1,980 bytes)**, expanding the
  bucket from 114/205 to 131/205 translated addresses:

  | cluster | bodies |
  |---|---|
  | release / reset | `0x806C0CD0` vtable-and-free wrapper, `0x806CA470` two-call reset |
  | chain / list walks | `0x806C2E10` five-link probe, `0x806C9940` `+0x5D4` accumulator, `0x806CB270` pickup chain |
  | selection | `0x806C2EF0` name-tail compare, `0x806C8810` two-string selector, `0x806CCA70` input-mask gate |
  | construction | `0x806C7360` `0x4A0`-byte heap constructor |
  | motion / effects | `0x806C3710` crossing notifier, `0x806CAC40` motion record, `0x806CC620` effect dispatch |
  | timers / clamps | `0x806CFE60` step clamp, `0x806CFED0` flash timer, `0x806CFF20` halfword advance |
  | descriptors | `0x806C17E0` and `0x806C3920` getters returning `0x80377A04` |

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806329E0` | `fn_806329e0` | 264 | high |
| `0x8065C100` | `fn_8065c100` | 264 | medium |
| `0x8065C260` | `fn_8065c260` | 264 | medium |
| `0x806C0CD0` | `fn_806c0cd0` | 100 | medium |
| `0x806C17E0` | `fn_806c17e0` | 12 | high |
| `0x806C2E10` | `fn_806c2e10` | 224 | high |
| `0x806C2EF0` | `fn_806c2ef0` | 136 | medium |
| `0x806C3710` | `fn_806c3710` | 124 | medium |
| `0x806C3920` | `fn_806c3920` | 12 | high |
| `0x806C7360` | `fn_806c7360` | 108 | medium |
| `0x806C8810` | `fn_806c8810` | 112 | medium |
| `0x806C9940` | `fn_806c9940` | 132 | medium |
| `0x806CA470` | `fn_806ca470` | 52 | medium |
| `0x806CAC40` | `fn_806cac40` | 196 | medium |
| `0x806CB270` | `fn_806cb270` | 124 | medium |
| `0x806CC620` | `fn_806cc620` | 196 | medium |
| `0x806CCA70` | `fn_806cca70` | 184 | medium |
| `0x806CFE60` | `fn_806cfe60` | 100 | medium |
| `0x806CFED0` | `fn_806cfed0` | 80 | high |
| `0x806CFF20` | `fn_806cff20` | 88 | medium |
| `0x806FCDF0` | `fn_806fcdf0` | 268 | medium |
| `0x806FFB50` | `fn_806ffb50` | 256 | high |
| `0x807241C0` | `fn_807241c0` | 256 | medium |
| `0x807485D0` | `fn_807485d0` | 268 | medium |
| `0x807811D0` | `fn_807811d0` | 256 | high |

**Totals:** 25 bodies, 4,076 bytes (7 high, 18 medium). The address window runs
from `0x806329E0` to `0x807811D0`.

## Where the instruction stream corrects the specification

Five places where the wave-133 prose disagreed with the shipped bytes; the
bodies follow the bytes and say so at the site:

1. **`0x807811D0` - the opponent cell is `0x8042A1A0`, not `0x80429EA0`.**
   `docs/SPEC_WAVE133_BASES.md` pairs that name with the instruction
   `lwz r5,-24160(r4)`, but `-24160` from `lis r4,0x8043` is `0x80430000 -
   0x5E60 = 0x8042A1A0`. Ghidra's `_DAT_8042a1a0` is right; the specification's
   address was an arithmetic slip.
2. **`0x806329E0` - the second probe's index is `0`, not `4*i`.** The spec
   claimed the raw word `addi r4,r0,0` must be read as `4*i` because `r0` holds
   `rlwinm r27,2`. But `rA = 0` in `addi` means literal zero - the spec's own
   "`addi rD,r0,imm` is `li`" rule. `0x80632A54` is `add r3,r31,r0`, which is
   where the `4*i` value is consumed; `r4` stays `0` in both probes. The body
   treats the re-probe exactly like the first probe.
3. **`0x806FCDF0` - the raw draw is stored at `+0x208` before the probe.** The
   spec's numbered list collapses `0x806FCE74` (`stw r3,520(r29)`) and
   `0x806FCE90` into one store. Because `0x806FE290(this, index)` receives
   `this`, that interim value is observable, so the body keeps both stores.
4. **`0x806CA470` - both callees receive `this` in `r3`.** Ghidra prints
   `func_0x8006cde0()` and `func_0x8006cd40(param_1)`; the shipped stream keeps
   `r3` live across the first call (`mr r3,r31` before the second), so both are
   passed `this`.
5. **`0x806CFF20` - the call argument is a ten-halfword stack record.** Ghidra
   declares `short local_18[10]`; only element `0` is loaded and stored back, so
   the body allocates the full record and reads element `0`.

A small host-side helper `f32_of_obj(addr, object)` was added for the four
`0x800E7E2x`/`0x800E7E4x`/`0x800E80B0` accessors that take the sub-object in
`r3`, and `call_f32_u32x2_f32` / `call_f32x4_u32x8` for the `0x8065C210`
clamped lerp and the `0x8007FA40` builder. No other helper was needed.

## Deliberate skips

Read and rejected, with the reason:

- **The 48-byte `0x802DCBFC` forwarder pairs** (for example
  `0x806C0B10`/`0x806C0B40` and `0x806C2FB0`/`0x806C2FE0`). All print the same
  `func_0x802dcbfc(param_2, param_2, param_1)` shape; the module header already
  records `0x802DCBFC` as an unresolved indirect-jump stub, so a body would only
  restate the stub call. Wave 133 skipped the eight siblings for the same reason.
- **Byte-identical duplicates of landed shapes.** `0x806C7040` is the same
  recursive five-link walk as `0x806C2E10`, `0x806C7120` the same name compare
  as `0x806C2EF0`, and `0x806C2CD0`/`0x806C6E40`/`0x806C6F00`/`0x806C8200`/
  `0x806C8F00` the same release wrapper as `0x806C0CD0` (with or without the
  `0x8015F800` sub-reset). One representative of each shape was landed; the
  clones are left for a future address-for-address pass so the lane stays
  reviewable.
- **The twelve remaining 12-byte descriptor getters**
  (`0x806C17F0`/`1800`/`1810`/`3930`/`3940`/`3950`/`3960`/`3970`/`3980`/
  `62A0`/`62B0`/`62C0`). Each is the same three words as the two landed getters
  (`lis r3,0x8037`; `addi r3,r3,0x7A04`; `blr`) and returns the same shared
  descriptor `0x80377A04`.
- **The `qr3` bodies `0x806CAD10` and `0x806CB140`** (and `0x806CFFD0`, whose
  `psq_l` type field is read through `GQR0 & 7`). `docs/REL_ELF_MAPPING.md`
  keeps `qr3` users class C until the `main.dol` GQR3 word is pinned; these
  three would require inventing that quantiser constant.
- **Larger bucket bodies not verified within this lane's budget:**
  `0x806C1040`, `0x806C12F0`, `0x806C13B0`, `0x806C1B00`, `0x806C3790`,
  `0x806C7890`, `0x806C7C20`, `0x806C8880`, `0x806C97D0`, `0x806CB050`,
  `0x806CC530`, `0x806CC6F0`, `0x806CF400` and `0x806CFB30`. Each needs its own
  call graph; several depend on the `0x802DCD64`/`0x802DCD74` save-helper
  `undefined8` return model that the wave-133 doc already flag as unsafe to
  guess.

## Notes on inference

- [`fn_8065c100`] and [`fn_8065c260`] are `medium`: the four `0x800E7E2x`/
  `0x800E80B0` scalar accessors are read literally but their roles are not in
  this REL; the `+0x99E4` `+0xB0` payload and message `796` are likewise
  `// unknown:`.
- [`fn_806329e0`] is `high`: the only inference is the meaning of the cached
  value (`-1.0` sentinel, `107.0` for an absent slot).
- [`fn_806fcdf0`] is `medium`: `-> u32` returning `1` is inferred to be a
  `bool init()`, and the `rand(1000)` loop's purpose is unrecorded. The interim
  `+0x208` store is kept because the probe sees `this`.
- [`fn_807485d0`] is `medium`: `0x8008E890` writes four floats, but their
  component order is inferred; the four ordered compares are literal.
- [`fn_806c0cd0`], [`fn_806ca470`], [`fn_806c7360`], [`fn_806c8810`],
  [`fn_806c9940`], [`fn_806cb270`], [`fn_806cca70`], [`fn_806cfe60`],
  [`fn_806cff20`] and [`fn_806cc620`] are `medium`: every call target, offset and
  immediate is literal, but a callee contract or an effect id's meaning is not
  described by this REL.
- [`fn_806c2ef0`] is `medium`: `0x802E1C40`/`0x802E1AB4` are read as the string
  scan and compare from the `0x3A` separator argument, not from a symbol table.
- The `unaff_GQR0` blocks at the head and tail of `fn_806329e0`,
  `fn_8065c100`, `fn_8065c260`, `fn_807241c0`, `fn_807485d0` and
  `fn_806cac40` are the `qr0` `f31`/`f30`/`f29`/`f28` prologue spill and
  epilogue reload pairs described in `docs/REL_ELF_MAPPING.md`; no body uses
  `qr3` and no body contains an `mtspr` to a `GQRn`.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (25 rows, 4,076 bytes),
  re-read from the TSV rather than from prose; every body's last word was
  re-read from `work/rels/elf/d_basesNP.elf` at
  `(address - 0x80600000) + 0x1000` and is `0x4E800020`.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 25 were checked against
  the ledger's address column before being written, and none is duplicated.
  The four wave-133 hand-off bodies (`0x80780D00`, `0x80711590`, `0x80643D40`,
  `0x806B39B0`) were re-checked and left untouched.
- `node tools/check-lane-csv.mjs wave134` reports `OK`;
  `wave134-bases.csv` has 25 rows and 25 unique addresses.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/game/bases.rs`.
