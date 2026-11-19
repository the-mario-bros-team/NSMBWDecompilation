# Coverage - Wave 136 (`game::bases`)

## How to read this file

This page documents the Wave 136 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`**, produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
`# Original` block in the source carries the
`Synthetic base: 0x80600000; this is not a runtime address.` line. Cross-module
cells (`0x8037xxxx`, `0x8042xxxx`) keep their absolute `main.dol` values; this
REL's own `.data`/`.bss` cells keep their `0x807Cxxxx` / `0x807Dxxxx` /
`0x8081xxxx` / `0x8082xxxx` / `0x8083xxxx` synthetic values.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee contract, a field role or a `main.dol`
cell's meaning had to be inferred. Comments in the bodies keep
`// observed:` / `// inferred:` / `// unknown:` markers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 28 | 3,888 | 13 | 15 |

All 28 rows are `translated`; none is `partial`. The module header now reports
3465 bodies (3437 + 28).

## What this pass covers

- **The twelve class-B bodies of `docs/SPEC_WAVE135_BASES.md` (2,056 bytes).**
  All nine "ready" addresses plus the three "conditional" ones, each keeping the
  `// unknown:` / `// inferred:` note the specification names:

  | cluster | bodies |
  |---|---|
  | lean / pose | `0x806FB540` damped lean factor, `0x80725170` ride pose reset |
  | spatial classifiers | `0x8069A170` ±pi/8 sector test, `0x806A8AA0` counted aim point |
  | timers | `0x80761880` peer-timer gate, `0x80656260` quantised height residual |
  | spawn / ring | `0x807481B0` twelve-slot spawn-ring driver |
  | light / layout | `0x80702CE0` five-light rebuild, `0x80608D10` render-flag tick |
  | player / mount | `0x8078B120` broom variant bind, `0x80781D10` spawn-state entry, `0x80716590` jump initialiser |

- **Eleven hand-off bodies (980 bytes)** — the ones the wave-135 specification
  listed for the next pass, minus the ambiguous `0x806FCCA0`:

  | cluster | bodies |
  |---|---|
  | render / layout | `0x806B3AC0` staged render copy, `0x80608C40` collection-coin layout load |
  | dispatch | `0x8066B0F0` four-slot virtual dispatch |
  | release | `0x80681480` and `0x80681FA0` twins, `0x806FF3F0` mesh release |
  | delete wrappers | `0x80600990`, `0x806009D0`, `0x80600A10`, `0x80600A50`, `0x80600A90` |

- **Five `0x8067` planner-bucket bodies (852 bytes)**, taking that bucket from
  87 to 82 untranslated addresses:

  | cluster | bodies |
  |---|---|
  | emitters | `0x80670FA0` and `0x80671060` `Wm_en_landsmoke_s` submissions |
  | world-map actor | `0x8067AF10` initialiser |
  | player probes | `0x8067DC50` masked-player scan, `0x8067DD60` mode query |

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80600990` | `fn_80600990` | 64 | high |
| `0x806009D0` | `fn_806009d0` | 64 | high |
| `0x80600A10` | `fn_80600a10` | 64 | high |
| `0x80600A50` | `fn_80600a50` | 64 | high |
| `0x80600A90` | `fn_80600a90` | 64 | high |
| `0x80608C40` | `fn_80608c40` | 132 | high |
| `0x80608D10` | `fn_80608d10` | 136 | high |
| `0x80656260` | `fn_80656260` | 172 | medium |
| `0x8066B0F0` | `fn_8066b0f0` | 128 | medium |
| `0x80670FA0` | `fn_80670fa0` | 184 | medium |
| `0x80671060` | `fn_80671060` | 172 | medium |
| `0x8067AF10` | `fn_8067af10` | 200 | medium |
| `0x8067DC50` | `fn_8067dc50` | 156 | medium |
| `0x8067DD60` | `fn_8067dd60` | 140 | medium |
| `0x80681480` | `fn_80681480` | 88 | high |
| `0x80681FA0` | `fn_80681fa0` | 88 | high |
| `0x8069A170` | `fn_8069a170` | 184 | medium |
| `0x806A8AA0` | `fn_806a8aa0` | 184 | medium |
| `0x806B3AC0` | `fn_806b3ac0` | 136 | medium |
| `0x806FB540` | `fn_806fb540` | 200 | high |
| `0x806FF3F0` | `fn_806ff3f0` | 88 | high |
| `0x80702CE0` | `fn_80702ce0` | 168 | high |
| `0x80716590` | `fn_80716590` | 148 | medium |
| `0x80725170` | `fn_80725170` | 196 | high |
| `0x807481B0` | `fn_807481b0` | 172 | medium |
| `0x80761880` | `fn_80761880` | 176 | medium |
| `0x80781D10` | `fn_80781d10` | 152 | medium |
| `0x8078B120` | `fn_8078b120` | 168 | medium |

**Totals:** 28 bodies, 3,888 bytes (13 high, 15 medium). The address window runs
from `0x80600990` to `0x8078B120`.

## Where the instruction stream corrects the specification

1. **`0x80761880` - `0x80761670` receives the peer actor, not `this`.**
   `docs/SPEC_WAVE135_BASES.md` §5 step 4 and its callee table say the 12-byte
   forwarder is called with `r3 = this`. The shipped words do not reload `r3`
   from `r31` before the `bl` at `0x807618B4`: `r3` still holds the value
   `fn_80761480` returned two instructions earlier, and `0x80761670` is
   `li r4,0; addi r3,r3,0x20C; b 0x801662E0`. The body therefore reads the
   **peer's** `+0x20C` timer through the forwarder and compares it with its own
   `+0x20C` timer. The comparison (`fcmpu` + `bne`, so NaN skips the reset) is
   reproduced exactly.
2. **`0x8069A170` - the band test is a negation, not a half-open interval.**
   Both branches are `bge` on `fcmpu`, which is false for an unordered result, so
   an unordered `dy` falls through to the `1` arm. The body writes
   `!(lower >= dy) && !(dy >= upper)` rather than `lower < dy && dy < upper` for
   that reason.
3. **`0x80670FA0` - the third stack word is `0x807D9104`, not `DAT_807c9104`.**
   The literal is `lis r5,0x807D` + `lfs f0,0x9104(r5)`; Ghidra's symbol name in
   the export disagrees with the address it prints. The body cites
   `0x807D9104` and records Ghidra's label at the site.

## Deliberate skips

Read and rejected, with the reason:

- **`0x806FCCA0` (100 bytes).** The body calls `0x8015F9F0(this+0x18, 0)` inside
  a doubly-guarded arm and then `0x802B9280(this, r4)` only when the second
  argument is positive. `r4` is `0` on the guarded path but the original argument
  on the other, and the raw stream gives no way to prove which value reaches the
  second call on every path. Left for an address-for-address pass instead of
  guessing.
- **`0x8067DCF0` (104 bytes).** A four-comparison box test whose `&&` chain is
  folded through `fcmpu` + `crand`/`cror` words; the export's predicate is clear
  but the CR-logical combination was not re-derived word by word inside this
  lane's budget. Nothing was written from the export alone.
- **The rest of the `0x8067` bucket** (82 remaining addresses, from
  `0x8067FC80` downward in size) is untouched: several are 400-1,700-byte bodies
  that need their own call graphs, and the lane's budget went to the two
  specified batches first.
- **Pure thunks.** No 4-byte `blr` stubs were claimed. The five 64-byte delete
  wrappers are real two-guard bodies with a call, not thunks, so they were
  landed; each cites the twin comparison in its doc block.
- **`0x806B3AC0`'s `0x80377C48` cell** is carried as an address-only
  `ptr_at(...)` read; nothing is assumed about its value.

## Notes on inference

- [`fn_8069a170`] is `medium` for one inference only: `0x802BE4A0(f32) -> f32`
  is named `sinf` in a comment. The emitted code calls the helper, so the body
  does not depend on the name. `0x80170980` is the in-place 3-float normalise
  documented by the wave-135 spec.
- [`fn_806a8aa0`] is `medium`: the `fmuls f1,f1,f2` at `0x806A8B1C` multiplies
  by the `0x43300000`-ladder value, i.e. the integer count returned by
  [`fn_806a8a40`]. That is the only reading with a live consumer for the ladder,
  and the `// inferred:` note sits at the store.
- [`fn_80656260`] and [`fn_80608c40`] read `main.dol` BSS cells
  (`0x80429E00`) and in-REL layout strings (`0x807D28BC`, `0x807D2888`) as
  addresses. No shipped bytes exist for the BSS cell, so only the two float
  fields this caller touches are modelled.
- [`fn_80781d10`] is `medium` for `0x800F6410`, which has no `functions.tsv` row
  anywhere in `work/ghidra`; it is carried as an unknown call taking the
  direction cell at `0x80377DEC`.
- [`fn_80670fa0`] / [`fn_80671060`] are `medium`: both virtual slots (`0x284`,
  `0x288`) and the emitter slot (`0xA0` / `0xB0`) are literal, but the two
  scalar queries have no external contract.
- [`fn_8067af10`], [`fn_8067dc50`] and [`fn_8067dd60`] are `medium`: every
  offset, mask and call argument is literal; the roles of the two binding calls,
  of the mask at `0x80429CA8` and of the returned `8` are inferred.
- [`fn_8066b0f0`] is `medium`: the vtable words are re-loaded before every call
  (`lwz r12,96(r31)` is repeated), so the body re-reads `+0x60` each time rather
  than caching it.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (28 rows, 3,888 bytes),
  re-read from the TSV for this pass rather than from prose. Every body's last
  word was re-read from `work/rels/elf/d_basesNP.elf` at
  `(address - 0x80600000) + 0x1000` and is `0x4E800020` (`blr`) at
  `entry + size - 4` for all 28.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 28 were checked against
  the ledger's address column before being written, and none is duplicated.
- `node tools/check-lane-csv.mjs wave136` reports `OK`;
  `wave136-bases.csv` has 28 rows and 28 unique addresses.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/game/bases.rs`.
