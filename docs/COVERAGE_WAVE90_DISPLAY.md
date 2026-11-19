# Coverage - Wave 90 (engine::display)

## How to read this file

`src/engine/display.rs` is the semantic home of the engine-level 2-D
presentation layer. This pass adds 30 bodies from the `0x80150000` bucket, the
same engine band that waves 16, 20 and 78 mined. Each body is a literal
reconstruction of one Ghidra export; the doc block above the Rust function names
the original address, the byte count taken from
`work/ghidra/main-gb/export/functions.tsv` and the evidence file. "high" means
every store, branch and call is visible in the export; "medium" means the export
dropped a virtual receiver or a constructor's operand, a printed
`countLeadingZeros` idiom only pins one special value, or a field role is
inferred from a sibling body. Every field whose role is not proven is marked
`// unknown:` or `// inferred:` in the Rust comment.

Only `src/engine/display.rs` was edited; `ledger/functions.csv` is read-only and
the coordinator merges this lane's rows.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/display.rs` | `engine::display` | 30 | 3696 | 16 | 14 |

## What this pass covers

The `0x80150000` bucket entered this pass with 98 untranslated of its 209
functions. This pass closes four coherent slices and leaves the GQR-quantised
predicates, the two very large drivers and the uniform thunk banks for a later
pass.

- **The 4 x 0x23C record container** (`0x80156ED0..0x80157360`, 8 bodies).
  [`fn_80156ed0`] is the constructor: base constructor, `0x803272D0` vtable at
  `+0x60`, helper at `+0x74`, and `0x802DC9A0` building four `0x23C`-byte records
  at `+0x148`. [`fn_80157060`] steps them with `0x800B1910`, [`fn_80157120`]
  queries them with `0x800B19B0`, [`fn_801571a0`] applies `0x800B2550` to one
  slot, [`fn_801571b0`] and [`fn_80157220`] store a value into the record whose
  `+0x381`/`+0x368`/`+0x364` triple matches a player and mode, and
  [`fn_80157310`] / [`fn_80157360`] set or clear one byte of the display object's
  per-player table at `+0xAFE + player * 0x16 + column`.
- **The scene constructor and its release pair** (`0x80157710..0x80157890`,
  3 bodies). [`fn_80157710`] builds the object that owns the layout at `+0x70`,
  the `0x80329958` sub-object at `+0x208`, the three interface tables at
  `+0x390..+0x39C` and the state manager at `+0x3A8`; [`fn_80157850`] and
  [`fn_80157890`] are its two `delete`-style helpers.
- **The boot-state tail** (`0x8015D1D0..0x8015DAF0`, 12 bodies). These are the
  handlers wave 78 left behind between `0x8015D0A0` and `0x8015DAA0`.
  [`fn_8015d530`] rebuilds the live save image's three record pairs,
  [`fn_8015d330`], [`fn_8015d650`] and [`fn_8015d860`] restore the display and
  the layout before a transition, and the rest choose between the
  `0x803776C8`, `0x80377708`, `0x80377748`, `0x803777C8`, `0x80377848`,
  `0x80377888` and `0x80377948` descriptors from the save-image
  readiness query and the save manager's `+0x74`/`+0x78`/`+0x7C` fields.
- **The intrusive list and the state objects** (`0x8015F930..0x8015FF40`,
  6 bodies plus the `0x8015DD70` thunk). [`fn_8015fef0`] appends to the list
  headed at `r13 - 0x5310` and tailed at `r13 - 0x530C`, [`fn_8015fe50`] unlinks
  from it and optionally frees, [`fn_8015fc90`] is the state-manager constructor
  already referenced by the pause window and [`fn_80157710`],
  [`fn_8015fa30`] / [`fn_8015fb80`] are its enter/leave halves and
  [`fn_8015ff40`] is the per-frame advance of one state object's halfword
  counters.

Two facts carry across the whole slice. First, every object here starts with a
vtable word at `+0x00` and the controllers keep their state interface at
`+0x74`, so `change_state(boot + 0x74, descriptor)` is the same shape the wave-78
handlers use. Second, the save image is a single global at `r13 - 0x5640`
([`G_SAVE_IMAGE`]) and the save manager a single global at `r13 - 0x56C8`
([`G_SAVE_MANAGER`]), both already used by the wave-20 save-image bodies, so the
boot handlers were translated against the existing field model instead of a new
one.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80156ED0` | `fn_80156ed0` | 104 | medium |
| `0x80157060` | `fn_80157060` | 108 | high |
| `0x80157120` | `fn_80157120` | 124 | high |
| `0x801571A0` | `fn_801571a0` | 16 | high |
| `0x801571B0` | `fn_801571b0` | 108 | high |
| `0x80157220` | `fn_80157220` | 236 | high |
| `0x80157310` | `fn_80157310` | 72 | high |
| `0x80157360` | `fn_80157360` | 72 | high |
| `0x80157710` | `fn_80157710` | 244 | medium |
| `0x80157850` | `fn_80157850` | 64 | medium |
| `0x80157890` | `fn_80157890` | 96 | high |
| `0x8015D1D0` | `fn_8015d1d0` | 164 | medium |
| `0x8015D2A0` | `fn_8015d2a0` | 128 | medium |
| `0x8015D330` | `fn_8015d330` | 84 | high |
| `0x8015D3A0` | `fn_8015d3a0` | 180 | medium |
| `0x8015D530` | `fn_8015d530` | 120 | high |
| `0x8015D5B0` | `fn_8015d5b0` | 116 | high |
| `0x8015D650` | `fn_8015d650` | 160 | high |
| `0x8015D860` | `fn_8015d860` | 120 | high |
| `0x8015D8F0` | `fn_8015d8f0` | 64 | high |
| `0x8015D930` | `fn_8015d930` | 120 | medium |
| `0x8015D9F0` | `fn_8015d9f0` | 176 | medium |
| `0x8015DAF0` | `fn_8015daf0` | 224 | medium |
| `0x8015DD70` | `fn_8015dd70` | 48 | medium |
| `0x8015FA30` | `fn_8015fa30` | 168 | medium |
| `0x8015FB80` | `fn_8015fb80` | 140 | medium |
| `0x8015FC90` | `fn_8015fc90` | 60 | medium |
| `0x8015FE50` | `fn_8015fe50` | 160 | high |
| `0x8015FEF0` | `fn_8015fef0` | 72 | high |
| `0x8015FF40` | `fn_8015ff40` | 148 | medium |

Byte total: `840` (record container) + `404` (scene constructor and releases) +
`1656` (boot-state tail) + `48` (thunk) + `748` (list and state objects) =
`3696`.

## Deliberate skips

68 addresses of the bucket stay untranslated. Three groups have a hard reason and
one group is simply outside this pass's thirty-body budget; the distinction
matters, so they are listed separately.

**Hard blockers - paired-single / GQR quantisation (8 addresses).**
`0x801502E0`, `0x80151A10`, `0x801548A0`, `0x80154A70`, `0x80154E50`,
`0x80156930`, `0x80156AB0` and `0x80159880` are printed with `unaff_GQR0`,
`ldexpf` and `CONCAT13/CONCAT22` quantisation stubs, four of them with
`WARNING: Removing unreachable block` where the paired-single semantics used to
be. `0x80156930` and `0x80156AB0` are the 372-byte rectangle predicates that
wave 16 already deferred for this reason. Any Rust body would be an invented
approximation, so they stay open.

**Hard blockers - unnamed SDA2 slots and unnamed extra-out registers (14
addresses).** `0x801501B0`, `0x80150540`, `0x80151630`, `0x80151AD0`,
`0x80151C30`, `0x80151D00`, `0x80151E50`, `0x80152020`, `0x80154740`,
`0x8015B320`, `0x8015DDD0`, `0x8015F140` and `0x8015F470` index the SDA2 float
bank through `unaff_r2` without naming the slot; `0x80152CF0` additionally loses
its float argument to an `extraout_f1` pseudo-register, and `0x8015B320` /
`0x8015DDD0` (1,276 and 4,716 bytes) are descriptor tables whose `unaff_r13`
anchors are not recovered either. Recovering those constants is a separate pass
that has to cross-reference every call site.

**Duplicate shapes - one representative is enough (23 addresses).**
`0x801543C0..0x80154460` is eleven identical twelve-byte `return &DAT_803545xx;`
getters and `0x801515C0` is a twelfth; `0x801533A0`, `0x801533D0`, `0x80159EB0`,
`0x80159EE0`, `0x8015B2C0`, `0x8015B2F0` and `0x8015DDA0` are seven more
`0x802DCBFC(second, second, first)` constructor thunks of exactly the shape
documented by [`fn_8015dd70`]; and `0x8015F130`, `0x8015F3B0`, `0x8015F410` and
`0x8015F7B0` are four-byte tail jumps into already-translated bodies. Two members
of each shape document the pattern; the rest carry no new behaviour and were left
rather than padded into the lane.

**Left at the thirty-body limit, not blocked (23 addresses).** `0x801504D0`,
`0x80150700`, `0x80150760`, `0x80150830`, `0x80151320`, `0x80151A70`,
`0x80151AC0`, `0x80151C20`, `0x801527A0`, `0x80152B30`, `0x80153400`,
`0x80154300`, `0x80154360`, `0x80156820`, `0x80156C30`, `0x80156D70`,
`0x801578F0`, `0x8015C4C0`, `0x8015C680`, `0x8015D7C0`, `0x8015F040`,
`0x8015F730` and `0x8015FE40` are all readable exports; the brief caps a lane at
thirty bodies and this one reached it. Four of them deserve a note for the next
pass: `0x80150700`/`0x80150760`/`0x80150830` are clean three-instruction
wrappers that gate a `(+0x60)` vtable slot with the `+0x1544` re-entrancy word,
but they belong to the `0x8014Exxxx` scene-object cluster whose
`+0x1520/+0x1524/+0x152D` field model is not established in this module yet;
`0x801578F0` is a byte-for-byte duplicate of [`fn_80157890`] and
`0x8015F040` duplicates `0x80154300`; `0x80156C30`/`0x80156D70` call the
`0x802DCD6C/0x802DCD70/0x802DCD74` debug-print triple with operands and a 64-bit
return the export does not resolve; `0x8015F730` is a varargs body that dumps
sixteen `undefined8` arguments through `in_cr1`; and `0x80153400` is a
3,832-byte driver that needs its own pass.

## Verification

- Every address and byte size matches `functions.tsv`; the lane validator
  cross-checks the `# Original` sizes and reported no mismatch over the 30 rows
  (total 3,696 bytes).
- Every evidence file exists at the cited path and every row has exactly one
  `pub unsafe fn fn_<addr>` body in `src/engine/display.rs`.
- `ledger/functions.csv` was read only; all 30 addresses were missing from it
  before this pass, and `ledger/agents/wave90-display.csv` has 30 unique rows.
- `node tools/check-lane-csv.mjs wave90` reports `OK` for this CSV.
- `cargo check --message-format=short` reports no error naming
  `src/engine/display.rs`. The module was additionally type-checked on its own in
  a scratch `no_std` crate against the real `src/platform.rs` and
  `src/engine/globals.rs`; it compiles with no error and no warning. The errors
  in the tree belong to other lanes (`game::worldmap`, `audio::ax`,
  `runtime::dvd`).
