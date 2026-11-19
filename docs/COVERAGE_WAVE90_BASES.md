# Coverage - Wave 90 (`game::bases`, `0x8079` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** under the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader chooses the real load address, so the numbers only
identify positions inside the REL image. Cross-module pointers (`0x8031xxxx`,
`0x8037xxxx`, `0x8042xxxx`) are `main.dol` data addresses imported by the module
and keep their absolute values.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`. "Role" below is the
plain-English behaviour. Confidence is `high` when guards, stores, constants
and call arguments are literal, and `medium` when the role of a field, a
global, a callee or a dropped argument had to be inferred (those bodies carry
`// observed:`/`// inferred:`/`// unknown:` notes). No body in this pass is
`low`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 5,848 | 8 | 22 |

## What this pass covers

The `0x8079` bucket holds 184 functions; 5 were already in
`ledger/functions.csv` when this pass began (the four sub-state setters at
`0x807982A0..0x807984F0`), leaving 179. This pass takes 30 of the remaining
bodies, the first five `character_SV`-derived actor families of the
bucket. Each family is opened by its model loader, which names both the class
string and the BRRES path, so the class identity of every member is evidence,
not inference.

- **Cloud platform (`jugem`), `0x80790470..0x80790CB0` (5 bodies).**
  [`fn_80790470`] is the in-place constructor: base vtable `0x8031BF68` then the
  class table `0x8081ADB0` at `+0x60`, the `+0x6E8` animation holder, two
  0x38-byte single-element arrays at `+0x704`/`+0x7BC` and two pane holders at
  `+0x73C`/`+0x77C`. [`fn_80790840`] loads `g3d/jugem.brres`, binds the `jugem`
  and `cloud` panes and builds the `cs_wait` and `cloud_wait` animations.
  [`fn_807907b0`] detaches all four animation objects when the marker returned
  by `0x8016A5D0` changes. [`fn_80790be0`] and [`fn_80790cb0`] are the
  animation/state starters.
- **Koopa-castle door (`cobKoopaCastle`), `0x807911F0..0x80791C60` (7 bodies).**
  The loader [`fn_807914d0`] names `cobKoopaCastle`, `g3d/model.brres` and the
  three `cobKoopaCastleOpen` panes; the constructor [`fn_807911f0`] builds the
  matching 0x38-byte element array at `+0x1B8`. [`fn_80791330`] mirrors the
  position into `+0x128..+0x134` and runs the transform writer
  [`fn_80791610`] and the state selector [`fn_807916c0`], which maps the
  `0x800FD8xx` door modes onto state codes 9/10/11 and schedules them through
  `0x801016B0`. [`fn_807918b0`] clears the state word and [`fn_80791c60`]
  emits the one-shot `Koopa0` bone effect.
- **Goomba (`kuribo`), `0x80791E60..0x80792520` (5 bodies).** The loader
  [`fn_80792110`] names `kuribo`/`g3d/kuribo.brres` and `character_SV`; the
  constructor [`fn_80791e60`] is the same `character_SV` layout with class
  vtable `0x8081B170`. [`fn_807922c0`] advances the walk animation,
  [`fn_80792320`] selects the model by the variant byte and registers the
  `Enemy` name, and [`fn_80792520`] starts the two-stage animation.
- **Walking piranha plant (`pakkun_walk`), `0x80793030..0x80793630` (6 bodies).**
  Constructor, destructor and loader ([`fn_80793030`], [`fn_80793110`],
  [`fn_807932f0`]) for the five-animation `g3d/pakkun_walk.brres` actor, plus
  the model select [`fn_80793470`], the animation attach [`fn_80793570`] and
  the animation start [`fn_80793630`].
- **Ivy/vine (`cobTsuta`), `0x80793D60..0x80794630` (7 bodies).** The loader
  [`fn_80794190`] names `cobTsuta`, `g3d/model.brres`, the three `cobTsuta_up`
  panes and the `body01` attachments; it also builds the `+0x2FC` clip and the
  `+0x2D4` eight-slot group. The constructor [`fn_80793d60`] and destructor
  [`fn_80793e90`] own the `+0x22C` array and the two interface objects,
  [`fn_80794400`]/[`fn_80794480`] reset and rewrite the pose, and
  [`fn_80794540`]/[`fn_80794630`] apply the per-class placement record at
  `0x8081B4F0` (0x1C-byte stride).

No new host-side helper was needed: the pass reuses `abi_call1..6`,
`vmethod0`, `vmethod1`, `call0_ret`, `call_f32_u32`, `call_f32_u32x2`,
`call_f32_u32x3`, `call_f32x3_u32` and `call_f32x3_u32x2`.

## Functions

| address | function | bytes | confidence | role |
|---|---|---:|---|---|
| `0x80790470` | `fn_80790470` | 188 | medium | cloud actor constructor |
| `0x807907b0` | `fn_807907b0` | 140 | medium | animation detach on marker change |
| `0x80790840` | `fn_80790840` | 444 | medium | `g3d/jugem.brres` loader |
| `0x80790be0` | `fn_80790be0` | 80 | high | class-0 animation bind |
| `0x80790cb0` | `fn_80790cb0` | 56 | medium | animation counter reset |
| `0x807911f0` | `fn_807911f0` | 136 | medium | door actor constructor |
| `0x80791330` | `fn_80791330` | 100 | high | start-up and position mirror |
| `0x807914d0` | `fn_807914d0` | 308 | medium | `cobKoopaCastle` loader |
| `0x80791610` | `fn_80791610` | 176 | medium | transform writer |
| `0x807916c0` | `fn_807916c0` | 484 | medium | door-state selector |
| `0x807918b0` | `fn_807918b0` | 12 | high | state word clear |
| `0x80791c60` | `fn_80791c60` | 128 | medium | `Koopa0` one-shot effect |
| `0x80791e60` | `fn_80791e60` | 184 | medium | Goomba constructor |
| `0x80792110` | `fn_80792110` | 432 | medium | `g3d/kuribo.brres` loader |
| `0x807922c0` | `fn_807922c0` | 96 | medium | walk pose update |
| `0x80792320` | `fn_80792320` | 208 | high | variant model bind |
| `0x80792520` | `fn_80792520` | 80 | medium | two-stage animation start |
| `0x80793030` | `fn_80793030` | 152 | medium | piranha constructor |
| `0x80793110` | `fn_80793110` | 140 | high | piranha destructor |
| `0x807932f0` | `fn_807932f0` | 344 | medium | `g3d/pakkun_walk.brres` loader |
| `0x80793470` | `fn_80793470` | 148 | high | model select by class |
| `0x80793570` | `fn_80793570` | 84 | medium | animation attach |
| `0x80793630` | `fn_80793630` | 56 | medium | animation counter reset |
| `0x80793d60` | `fn_80793d60` | 240 | medium | vine actor constructor |
| `0x80793e90` | `fn_80793e90` | 268 | high | vine destructor |
| `0x80794190` | `fn_80794190` | 624 | medium | `cobTsuta` loader |
| `0x80794400` | `fn_80794400` | 120 | high | four virtual pose resets |
| `0x80794480` | `fn_80794480` | 188 | medium | transform attach to interface |
| `0x80794540` | `fn_80794540` | 80 | medium | placement and colour start-up |
| `0x80794630` | `fn_80794630` | 152 | medium | per-class placement offset |

## Address range

`0x80790470..0x80790CB0` (cloud platform), `0x807911F0..0x80791C60` (castle
door), `0x80791E60..0x80792520` (Goomba), `0x80793030..0x80793630` (piranha
plant) and `0x80793D60..0x80794630` (vine). After this pass the `0x8079` bucket
still has 149 untranslated functions.

## Deliberate skips

The bodies below were read and rejected rather than guessed; the rest of the
bucket was simply not reached.

- `0x80790280` (268 bytes) and `0x80792570` (628 bytes) are the per-frame mesh
  and walk dispatchers of the cloud and Goomba actors. Both are exported with
  `unaff_GQR0` and `ldexpf` artifacts (paired-single register pairs that Ghidra
  could not type), so the float storage class of the pose words is not
  recoverable; only their scalar outlines are visible. Deferred, as in wave 89
  for the same pattern.
- `0x807946D0` (452 bytes) is the vine pose rebuild called by
  [`fn_80794540`]. Its body is readable, but its first two float moves are again
  `unaff_GQR0`/`ldexpf` artifacts and it consumes the whole 0x1C-byte record
  through four different tables; it is left as an address-only call.
- `0x807900B0` (164 bytes), `0x80790160` (116 bytes) and `0x807901E0` (148
  bytes) form the bucket's opening effect-handle family: they walk the
  `E_p_01` name tables at `0x8081AA40`/`0x8081ABA0` and open 17 and 21 handles
  through `0x80103410`, whose third argument (`object + 0x1A4`) is a
  resource-owner layout this export does not describe. The stores are literal
  but the class that owns `+0x214`/`+0x260` is unknown, so they were not
  claimed rather than attached to the wrong class.
- `0x807923F0` (104 bytes) spawns the Goomba's `skl_root` effect, but its first
  call `0x800F7CA0` is printed with a single `(double)` constant and no pointer
  argument. No other call site of that callee exists in this export, so the
  argument list cannot be checked and the body is deferred.
- `0x80793E50` (64 bytes) is the vine actor's base destructor: its only call
  `0x802B9280` is printed with no arguments and has no other call site in the
  export, so whether it receives the object cannot be proven. The full
  destructor [`fn_80793e90`] is translated instead.
- `0x807939D0` (16 bytes) returns `0x807D000C` as a `double`; the storage class
  of that constant (float promoted or true double) is ambiguous in the export,
  so it is left for a pass that can resolve it.
- `0x807932F0`'s neighbours `0x807939E0..0x80793BC0` and the remaining
  `0x80794C00`..`0x8079FE20` tail of the bucket were not reached.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 5,848 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the 5 pre-existing rows of
  this bucket (`0x807982A0`, `0x80798310`, `0x80798370`, `0x80798410`,
  `0x807984F0`) were listed before writing and none is in this pass.
- `node tools/check-lane-csv.mjs wave90` reports `wave90-bases.csv: 30 rows`
  and `OK`.
- `cargo check --message-format=short` produces no diagnostic for
  `src/game/bases.rs`.
