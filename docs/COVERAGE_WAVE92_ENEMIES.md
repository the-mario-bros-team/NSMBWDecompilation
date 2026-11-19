# Coverage - Wave 92 (`game::enemies`, the `daEnBomhei_c` actuator of the `0x80A20000` bucket)

## How to read this file

This page records the Wave 92 `game::enemies` lane: 26 functions of the
`0x80A20000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs`. Every row in the tables below is one real
`pub unsafe fn fn_<addr>` body in that file, and the `evidence` column of the
machine-readable mirror (`ledger/agents/wave92-enemies.csv`) names the Ghidra C
export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Float
constants and strings were read from `work/rels/elf/d_enemiesNP.elf` at the
cited synthetic address with a small ELF reader, never guessed from the C text.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 26 | 2,856 | 13 | 11 | 2 |

The bucket holds 171 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 63 were already in
`ledger/functions.csv` before this pass (the wave-58 opening
`0x80A200D0..0x80A22B50`, the wave-76 continuation
`0x80A23180..0x80A25F30`, and the three named helpers
`register_bomhei_states`, `register_bounce_ball_states` and
`alloc_bounce_ball_actor`), 26 are claimed here and 82 remain.

## What this pass covers

Three runs chosen around one actor, so the field map carries across every body.

**Run 1, `0x80A2AE10..0x80A2B1D0`, the bomhei state skeleton and its actor
constructor.**

- `0x80A2AE10` and `0x80A2AE70` are the two deleting destructors of the
  five-entry state-descriptor skeleton: `0x8015F800(state, 0)` and, for a
  positive `mode`, `0x802B9280`. `register_bomhei_states` (already translated at
  `0x80A2A350`) passes those two addresses as the destructor of its fourteen
  records.
- `0x80A2AED0` returns the shared state word `0x80358104`; `0x80A2AEE0`
  returns `0x80352DB4`, the "extra" member the registration stores on the
  `Carry` record.
- `0x80A2AFD0` is the `+0x30` owner-chain walker (five unrolled `+0xC` checks,
  then a recursion on the fifth owner itself), `0x80A2B0B0` the `:`-suffix name
  predicate built from `0x802E1C40`/`0x802E1AB4`, and `0x80A2B170`/`0x80A2B1A0`
  the two 48-byte `0x802DCBFC` forwarders.
- `0x80A2B1D0` allocates and constructs the `0x858`-byte actor: class vtable
  `0x80B3B604`, model context `+0x524`, model record `+0x544`, animation holder
  `+0x584` (vtable `0x80B330E0`), the embedded transform block
  `+0x5BC..+0x5CC`, the self-referencing record `+0x718..+0x724`, and
  `+0x850 = 1`.

**Run 2, `0x80A2B4C0..0x80A2C300`, that actor's helper block.**

- `0x80A2B4C0` copies the `+0x5EC` origin onto the carried child with a `4.0`
  Y drop; `0x80A2B570` solves the heading between the `+0x5EC` and `+0x61C`
  points with the `atan2`-shaped `0x801611C0` plus a `0x4000` quarter turn;
  `0x80A2B630` is the child query over `+0x718`/`+0x410`.
- `0x80A2B6D0` switches to `0x80352BB4` through the class slot `+0xD4` and
  halves the two `i32` counters `+0x6E0`/`+0x6DC`; `0x80A2B780` raises effect
  `0x6C` with `side | 0xE0000`.
- `0x80A2BAE0` and `0x80A2BBC0` are the contact responders keyed on the other
  actor's `+0x38C` byte: the first builds a six-argument damage report through
  the other actor's `+0x3FC` slot, the second derives a `1`/`0` side flag from
  the relative x position (`// observed:` the export computes it with a
  `countLeadingZeros` of the `0.0 < dx` compare) and hands it to the contact
  walker `0x80A2BC50`.
- `0x80A2C100`, `0x80A2C150` and `0x80A2C1A0` are the three positional sound
  emitters (`0x537`, `0x538`, `0x538`), `0x80A2C210` the one-shot flag setter
  at `+0x6F7`/`+0x6F8`, and `0x80A2C300` the `Carry` entry that reads side from
  bit 28 of the spawn word and seeds `(1.0 - 2.0 * side) * 0.8` into `+0xE8`.
- `0x80A2D940`/`0x80A2D9B0` are the constructor and deleting destructor of the
  embedded model record at `+0x40` (vtables `0x803295D8` → `0x80329578`,
  released by `0x801676A0`/`0x80169D20`).

**Run 3, `0x80A2DC00..0x80A2DCC0`, the `fire snake` impact path.**

- `0x80A2DC00` and `0x80A2DC30` forward an impact id to the contact child's
  `+0x3F4` slot (byte-identical bodies).
- `0x80A2DCC0` plays sound `0x1D1`, raises `Wm_en_firesnk_icehit_h` and
  `Wm_en_firesnk_icehitsmk_h` at `+0xB44`, then calls the six-argument effect
  `0x800E20A0` with the two `main.dol` offsets `0x8042CC88`/`0x8042CC8C` and the
  id word at `0x8042A050`.

### Identity evidence

- **The class name is inferred, never asserted.** The data block at
  `0x80B3B568` holds the two `fState`-object vtables that
  `register_bomhei_states` passes as its fourth argument
  (`0x80B3B568`/`0x80B3B59C`); their methods are `0x80A2AE10`, `0x80A2AE70`,
  `0x80A2B0B0`, `0x80A2B170` and `0x80A2B1A0`. Immediately after that block sit
  the `Wm_en_bombheibreak` and `Wm_en_explosion` names, the create slot
  `0x80B3B5F8` holding the address of `0x80A2B1D0`, and the class vtable
  `0x80B3B604` that the constructor installs at `+0x60`. The
  `daEnBomhei_c::StateID_*` strings at `0x80B3B3A8..0x80B3B548` are the names
  the registration table uses, which is why the actor is described as the
  `daEnBomhei_c` actuator.
- **The bodies are tied to the vtable.** `0x80B3B604` also holds
  `0x80A2B900`/`0x80A2B960`, two virtual methods inside this same run that
  `functions.tsv` does not list, so they are not claimed.
- **The 48-byte forwarders** are the same `0x802DCBFC(param_2, param_2,
  param_1)` bodies already documented in this module; the hidden `r12`
  this-adjust pointer is not modelled and each doc says so.
- **The two `fire snake` strings** were dumped from the image at `0x80B3C2F4`
  and `0x80B3C30C`; the same data block continues with `g3d/bubble.brres`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A2AE10` | `fn_80a2ae10` | 88 | high |
| `0x80A2AE70` | `fn_80a2ae70` | 92 | high |
| `0x80A2AED0` | `fn_80a2aed0` | 12 | high |
| `0x80A2AEE0` | `fn_80a2aee0` | 12 | high |
| `0x80A2AFD0` | `fn_80a2afd0` | 224 | medium |
| `0x80A2B0B0` | `fn_80a2b0b0` | 136 | medium |
| `0x80A2B170` | `fn_80a2b170` | 48 | low |
| `0x80A2B1A0` | `fn_80a2b1a0` | 48 | low |
| `0x80A2B1D0` | `fn_80a2b1d0` | 288 | high |
| `0x80A2B4C0` | `fn_80a2b4c0` | 168 | medium |
| `0x80A2B570` | `fn_80a2b570` | 100 | high |
| `0x80A2B630` | `fn_80a2b630` | 72 | medium |
| `0x80A2B6D0` | `fn_80a2b6d0` | 88 | high |
| `0x80A2B780` | `fn_80a2b780` | 100 | high |
| `0x80A2BAE0` | `fn_80a2bae0` | 216 | medium |
| `0x80A2BBC0` | `fn_80a2bbc0` | 144 | medium |
| `0x80A2C100` | `fn_80a2c100` | 80 | high |
| `0x80A2C150` | `fn_80a2c150` | 80 | high |
| `0x80A2C1A0` | `fn_80a2c1a0` | 80 | high |
| `0x80A2C210` | `fn_80a2c210` | 16 | high |
| `0x80A2C300` | `fn_80a2c300` | 232 | medium |
| `0x80A2D940` | `fn_80a2d940` | 104 | medium |
| `0x80A2D9B0` | `fn_80a2d9b0` | 104 | high |
| `0x80A2DC00` | `fn_80a2dc00` | 40 | medium |
| `0x80A2DC30` | `fn_80a2dc30` | 40 | medium |
| `0x80A2DCC0` | `fn_80a2dcc0` | 244 | medium |

## Deliberate skips

- **`0x80A2B490` (48 bytes).** The whole body is
  `(double)(float)(4503601774854171.0 - DAT_80b2bb18)`. That literal is the
  `2^52`/`2^31` unsigned-int-to-double bias plus `27`; the integer operand the
  original converted is folded away, so writing any value would be a guess.
- **`0x80A2C2A0` (76 bytes).** Same artefact in the other direction
  (`4.50360177485824e+15 - DAT_80b2bbb8`), so the yaw scale cannot be recovered.
- **`0x80A2BC50` (456 bytes).** Its export carries `unaff_GQR0` pseudo-registers
  and a removed unreachable block, exactly as the wave-76 coverage doc already
  recorded. [`fn_80a2bbc0`] still calls it by address with the three arguments
  the call site shows.
- **`0x80A2D460` (20 bytes).** Both paths are `halt_baddata`; the export is
  truncated.
- **`0x80A2D660` (132 bytes).** It calls `0x80A2D460`, so its `+0x6C0` branch
  cannot be reconstructed honestly.
- **`0x80A2DC00`/`0x80A2DC30` carry a caveat.** They are claimed, but the export
  also warns that `0x80A2DC24` is an unrecovered jump table treated as an
  indirect call; the guard and the `+0x3F4` call are literal, the tail jump is
  not modelled. Both docs say so and both rows are `medium`.
- **Eleven bodies are `medium` because a field role or a callee argument is
  inferred**, not because an instruction is guessed: `fn_80a2afd0`,
  `fn_80a2b0b0` (the `+0xC`/`+0x20` slot arguments), `fn_80a2b4c0` (the child's
  `+0x100` slot), `fn_80a2b630` and `fn_80a2d940` (a callee whose argument the
  export drops), `fn_80a2bae0`, `fn_80a2bbc0`, `fn_80a2c300`, `fn_80a2dc00`,
  `fn_80a2dc30` and `fn_80a2dcc0`. Counted exactly: eleven `medium` rows.
- **Two bodies are `low`.** `fn_80a2b170` and `fn_80a2b1a0` are the 48-byte
  argument-swapping forwarders into `0x802DCBFC`; the reshuffle is literal but
  neither the target's role nor the hidden `r12` this-adjust pointer is
  established. Every body carries the same `// partial:` note.
- **`0x80A2B900` and `0x80A2B960` are not claimed.** They are live virtual
  methods (slots of `0x80B3B604`) but they are absent from `functions.tsv`, so
  there is no export to reconstruct them from and no byte count to cite.
- **The rest of the bucket stays untranslated.** After this pass the largest
  untouched runs are the `bombhei` family `0x80A263C0..0x80A2A180`, the damaged
  `0x80A2BC50`, the `0x80A2D660..0x80A2F170` block and the closing skeleton at
  `0x80A2F670..0x80A2FD40`.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 26 sizes sum to
  2,856 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 26 was
  already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave92` prints
  `wave92-enemies.csv: 26 rows` and `OK`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/enemies.rs`. The workspace-wide errors during this pass are in
  `src/game/player.rs`, which belongs to another Wave 92 lane; a scratch copy of
  the crate with that file stubbed compiles with no `enemies.rs` warning or
  error.
