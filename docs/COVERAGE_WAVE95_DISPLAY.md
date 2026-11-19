# Coverage - Wave 95 (engine::display)

## How to read this file

`src/engine/display.rs` is the semantic home of the engine-level 2-D
presentation layer. This pass adds 26 bodies from the `0x80150000` bucket, the
same engine band that waves 16, 78, 90 and 93 mined. Each body is a literal
reconstruction of one Ghidra export; the doc block above the Rust function
names the original address, the byte count taken from
`work/ghidra/main-gb/export/functions.tsv` and the evidence file.

"high" means every store, branch and call is visible in the export. "medium"
means the export drops a virtual receiver or a callee operand, an SDA2 float's
value is not recovered (the code reads it from its absolute `r2`-relative
address instead of guessing it), or the `0x802DCD74` debug context and the
`0x802F5918` byte table are only partially visible. Every field whose role is
not proven is marked `// unknown:` or `// inferred:` in the Rust comment.

Only `src/engine/display.rs` was edited; `ledger/functions.csv` is read-only and
the coordinator merges this lane's rows.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/display.rs` | `engine::display` | 26 | 3400 | 12 | 14 |

## What this pass covers

The bucket entered this pass with 68 untranslated of its 209 functions and 42
remain. This pass closes four coherent slices: the scene-layout gates and their
`+0x1544` wrappers that wave 90 left at its budget, the `0x80151..` state
handlers, the `0x801533A0..0x80156D70` thunk/release/player-bit cluster, and the
boot/cursor tail.

- **The scene-layout gate cluster** (`0x801501B0..0x80150830`, 6 bodies).
  [`fn_801501b0`] resets the `+0x1558/+0x155C/+0x1560` pending-item triple;
  [`fn_801504d0`] finishes the layout change through `0x80150650` and installs
  the `0x80376E88` name plus the `0xB3/3/199` channel ids; [`fn_80150540`]
  reads pane `8` into a `0x34` matrix, builds and concatenates the translation
  and queues the `0x800A7540`/`0x800A7550` screen entry. [`fn_80150700`],
  [`fn_80150760`] and [`fn_80150830`] are the three `+0x1544` re-entry
  wrappers around the owner's `+0x60` slots `+0x84`, `+0x88` and `+0x90`.
- **The `0x80151..` state handlers** (`0x80151320..0x80152B30`, 11 bodies).
  [`fn_80151320`] plays the pad effect for the `+0x38D` route byte when no
  owner is present; [`fn_801515c0`] selects one of the two placement rows from
  bit 0 of the manager's `+0x160`; [`fn_80151a70`], [`fn_80151ac0`] and
  [`fn_80151c20`] are the three channel wrappers; [`fn_80151ad0`] and
  [`fn_80151c30`] are the two marker placements (corner average versus the
  single-pane row) that share the `+0x1224..+0x1238` record; [`fn_80151d00`]
  chooses the placement row, the model id `0x109`/`0x10D` and the fallback row;
  [`fn_80152020`] is the pair-contact test with its impulse and fallback;
  [`fn_801527a0`] and [`fn_80152b30`] are the two view forwarders.
- **The allocation thunk, release helper and player-bit resolver**
  (`0x801533A0..0x80156D70`, 4 bodies). [`fn_801533a0`] is the
  `0x802DCBFC(second, second, first)` thunk documented by [`fn_8015dd70`];
  [`fn_80154300`] is the `0x8015F800` destructor plus optional `0x802B9280`
  free; [`fn_80156820`] walks the `0x8005FC70` mask over the `0x802F5918`
  sixteen-byte table and resolves the id through `0x8005F900`; [`fn_80156d70`]
  counts the players whose `+0x153C` byte is `4` and reports
  `count == connected`.
- **The boot and stepping tail** (`0x8015C4C0..0x8015F140`, 5 bodies).
  [`fn_8015c4c0`] turns boot state `2` into the game-start handshake,
  [`fn_8015c680`] fades the boot screen out, [`fn_8015d7c0`] leaves it once the
  view object reports event `0x900` or the global display flag is set,
  [`fn_8015fe40`] clears the two list anchors, and [`fn_8015f140`] is the
  rate/limit/dead-band `f32` stepper the marker and replay bodies already call
  as `0x8015F140`.

Two facts carry across the slice. First, the scene object model is the one
waves 90/93 established: `+0x14D8` manager, `+0x14xx` model source, the
`+0x1520/+0x1524` current-owner pair and the `+0x1544` re-entry word. Second,
every marker body writes the same `0x42` bit into `+0x1238` and a record kind
into `+0x1235` (`0xF` for the corner average, `0x10` for the single pane), so
the two placements are documented as one record with two fills rather than two
unrelated functions.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801501B0` | `fn_801501b0` | 28 | medium |
| `0x801504D0` | `fn_801504d0` | 104 | medium |
| `0x80150540` | `fn_80150540` | 260 | medium |
| `0x80150700` | `fn_80150700` | 88 | high |
| `0x80150760` | `fn_80150760` | 88 | high |
| `0x80150830` | `fn_80150830` | 108 | high |
| `0x80151320` | `fn_80151320` | 168 | medium |
| `0x801515C0` | `fn_801515c0` | 36 | high |
| `0x80151A70` | `fn_80151a70` | 68 | high |
| `0x80151AC0` | `fn_80151ac0` | 8 | high |
| `0x80151C20` | `fn_80151c20` | 8 | high |
| `0x80151AD0` | `fn_80151ad0` | 324 | medium |
| `0x80151C30` | `fn_80151c30` | 204 | medium |
| `0x80151D00` | `fn_80151d00` | 324 | medium |
| `0x80152020` | `fn_80152020` | 260 | medium |
| `0x801527A0` | `fn_801527a0` | 76 | medium |
| `0x80152B30` | `fn_80152b30` | 236 | medium |
| `0x801533A0` | `fn_801533a0` | 48 | high |
| `0x80154300` | `fn_80154300` | 88 | high |
| `0x80156820` | `fn_80156820` | 164 | medium |
| `0x80156D70` | `fn_80156d70` | 132 | medium |
| `0x8015C4C0` | `fn_8015c4c0` | 148 | high |
| `0x8015C680` | `fn_8015c680` | 100 | high |
| `0x8015D7C0` | `fn_8015d7c0` | 124 | medium |
| `0x8015FE40` | `fn_8015fe40` | 16 | high |
| `0x8015F140` | `fn_8015f140` | 192 | medium |

Byte total: `676` (scene-layout gates) + `1712` (state handlers) +
`432` (thunk, release and resolver) + `580` (boot tail) = `3400`.

## Deliberate skips

42 addresses stay untranslated. Seven groups have a hard reason and two are
duplicate shapes that add no new behaviour.

**Hard blockers - paired-single / GQR quantisation (7 addresses).**
`0x801502E0`, `0x80151A10`, `0x801548A0`, `0x80154A70`, `0x80154E50`,
`0x80156930` and `0x80156AB0` are printed with `unaff_GQR0`/`unaff_GQR3`,
`ldexpf` and `CONCAT13/CONCAT22` quantisation stubs, each with at least one
`WARNING: Removing unreachable block` where the paired-single semantics used to
be. Any Rust body would be an invented approximation, so they stay open.

**Hard blockers - debug-context split (3 addresses).** `0x80151E50`,
`0x80156C30` and `0x80159880` recover their arguments from the `0x802DCD74`
pseudo-return. `0x80156C30` dereferences the low word as a float array and
`0x80159880` advances both halves through `CONCAT44`, so the real ABI split
(`r3`/`r4`) is not visible in the export; writing a signature for them would be
invention. `0x80152CF0` additionally loses its float operand to `extraout_f1`
(a fourth, separate reason).

**Hard blockers - unrecovered extra-out register (1 address).** `0x80152CF0`
reads its first float argument from `extraout_f1`, which the export never
defines.

**Hard blockers - descriptor tables (3 addresses).** `0x80153400` (3,832
bytes, 19 registered states), `0x8015B320` (1,276 bytes, 6 states) and
`0x8015DDD0` (4,716 bytes, 24 states) are `0x8015F7C0` + `0x802DC780`
registration banks. The block shape is known from [`fn_800d7c80`], but every
block's name string and handler pair has to be checked individually; they need
their own pass rather than a guessed loop.

**Hard blockers - drivers whose callees are blocked (2 addresses).**
`0x80154740` calls `0x801548A0` and `0x80154A70` in a four-slot loop, and both
are GQR-stub bodies, so the loop's effect cannot be stated. `0x80151630` is the
668-byte route-marker setup whose `0x800D8BB0` branches mix the same SDA2
constants and the `+0x1464` model source; it needs the GQR pass first.

**Hard blocker - varargs dump (1 address).** `0x8015F730` takes sixteen
`undefined8` arguments and prints them through `in_cr1`, which the export does
not reconstruct.

**Hard blocker - four-byte tail jumps (5 addresses).** `0x8015F130`,
`0x8015F3B0`, `0x8015F410`, `0x8015F470` and `0x8015F7B0` are four-byte jumps
whose exports contain the *target's* body (`0x801B8620`'s CRC walk, the
`0x8015F3C0`/`0x8015F420` integer steps, the `0x8015F480` float step and the
`0x8015F6E0` indirect call). Transcribing those bodies at these addresses would
document the wrong function, so they stay open.

**Duplicate shapes - one representative is enough (9 addresses).**
`0x801533D0`, `0x80159EB0`, `0x80159EE0`, `0x8015B2C0`, `0x8015B2F0` and
`0x8015DDA0` are six more `0x802DCBFC(second, second, first)` thunks identical
to [`fn_801533a0`]; `0x80154360` and `0x8015F040` are byte-identical release
helpers to [`fn_80154300`]; `0x801578F0` duplicates the already-translated
`fn_80157890`.

**Duplicate shapes - one-line table getters (11 addresses).**
`0x801543C0..0x80154460` are eleven twelve-byte `return &DAT_803545xx;`
getters. They are trivially correct but carry no behaviour beyond the returned
address, so they were left rather than padded into the lane.

## Verification

- Every address and byte size matches `functions.tsv`, re-read for this pass;
  the 26 `# Original` blocks total 3,400 bytes.
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/engine/display.rs`.
- No address was already in `ledger/functions.csv`; the bucket had 68
  untranslated functions before this pass and 42 after it.
- `node tools/check-lane-csv.mjs wave95` reports `OK`; this lane's CSV has 26
  unique rows and no duplicate address.
- `cargo check --message-format=short` finishes with warnings only and reports
  no error or warning naming `src/engine/display.rs`; the module was also
  type-checked on its own in a scratch `no_std` crate against the real
  `src/platform.rs` and `src/engine/globals.rs`.
