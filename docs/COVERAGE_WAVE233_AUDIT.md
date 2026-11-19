# Audit - Wave 233 (batch 6, arity defects, shared bodies)

## Bodies translated (62)

| Lane | Table | Bodies | Bytes |
|---|---|---:|---:|
| A | `WM_PLAYER_VTABLE` slots 48-59 | 12 | 1,900 |
| B | player-class vtable, next 12 | 12 | 1,004 |
| C | WorldMap vtable, next 10 | 10 | 2,620 |
| D | `dEn_c` vtable, next 10 | 10 | 1,136 |
| E | `.ctors` 99-112 | 10 | ~1,300 |
| F | repairs (below) | - | - |

## Two systemic findings

### 1. Shared method bodies across vtables

The same method address appears in several class vtables. In this wave
`0x80065CC0`, `0x80065DC0`, `0x80065EC0`, `0x80065970`, `0x80065660`,
`0x8001D1D0`, `0x80020BE0`, `0x80056BC0` and `0x80056D40` were each reached from
the WorldMap vtable, the `dEn_c` vtable and/or the player vtable - so three
modules ended up with three copies of the same logic. Duplication is
behaviour-preserving but violates the project's one-body-per-address
convention, and it inflates the "bodies translated" count.

Decision: keep one canonical definition per address (the `rust_file` the ledger
assigns) and have the other modules delegate to it. Round 234 does this.

### 2. Argument-arity defects: 2,455 mismatches, 50 live

A repo-wide sweep of 42,484 resolved address-call sites against the callees'
translated signatures found **1,191 under-supplied** and **1,264 over-supplied**
sites. Most are harmless (extra/fewer unused registers), but **50
under-supplied sites are live** - the callee body reads a parameter the call
never supplies - which is the same failure mode as the two the lanes found by
hand:

- `fn_80a9be80` -> `0x80097D30`: fixed. The override is a pass-through
  (`mr r31,r3; bl 0x80097D30`), and `0x80097D30` unconditionally dereferences
  `r4`; all three sibling overrides are identical pass-throughs.
- `fn_80a9e090` -> `0x80065970`: reported, still to fix.
- Examples from the sweep: `platform::call0(0x8016_A010)` inside
  `fn_8016a2f0(this)`, and four `call0` sites for `fn_80267420`.

## Other corrections

- **`extsb` decoder fix.** `extsb` (XO 954) was in the three-register template
  set, so it printed `extsb rt,ra,rb`; it is a two-register form
  (`extsb rA,rS`). Fixed in `work/wave231/laneE_dis.mjs`. `extsh` (922) and
  `cntlzw` (26) carry the same latent bug and are still to fix.
- Coordinate/string traps: `0x80309F5C` is `"Wm_en_burst_s"` (the naive
  `0x80319F5C` lands in a pointer table); slot 48's tier flag sets its low bit
  when the probe is `< 3` (`cmpi r3,3` / `bf cr0.lt`; wave 234 arbitrated this
  against all three copies) and it reloads `+0x38F` after the query.
- Slot 39 writes only X/Y; its trailing six-word stack copies are dead spills.
- Three-tier slam effects reproduce `bases.rs::fn_80725270`'s shape
  (`0x800929F0`, SE ids 0x282/3/4, `0x80078410`, `Z = 6500.0`).

## Coverage-rule refinements

Three distinct false "already translated" cases have now been found:

1. an address mentioned only in a neighbour's **prose** (wave 232 slot 9),
2. an address that appears only as a **literal call target** elsewhere
   (wave 233 slot 49),
3. an address defined with a **semantic name** rather than `fn_<addr>`
   (correctly treated as covered, e.g. `wm_projection_globals_init`).

Coverage now means: `functions.tsv` row, or a real Rust body whose `# Original`
cites the address, under any name.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `scripts/check.ps1` exit 0.
