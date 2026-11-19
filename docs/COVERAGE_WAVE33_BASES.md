# Coverage — wave 33 (`d_basesNP`: bolt actor and `lift_torokko`)

## How to read this file

This page records the thirty-third translation pass over `d_basesNP`, the base
gameplay REL (`game::bases`). It answers:

1. **What was added?** — *Translated functions* lists the 30 new bodies in
   `src/game/bases.rs`: the `0x700`-byte `g3d/bolt.brres` actor (factory,
   start-up, transform, pad latches, rider scan, carry state and destructor)
   and the `lift_torokko` constructor family with its element helpers and
   BRRES loader.
2. **Why these functions?** — *Selection* explains the address window and why
   this cluster was taken whole.
3. **What is still missing?** — *Deferred candidates* records the compiler
   veneers that were deliberately not translated.

Column meanings:

- **Status** — `translated` = control flow and every observed call/store are
  represented; `partial` = entry/exit and major effects only; `stub` = address
  only.
- **Confidence** — strength of the evidence: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a role, constant or called
  body is inferred; `low` = mostly guesswork.
- **Evidence** — file under `work/ghidra/rel-d_basesNP/export/`; that C file is
  the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80600000`; the entry point
is `0x806000F0`. The REL has no embedded load address, and the retail loader may
place it anywhere. These are **not** runtime addresses. Main.dol addresses
(`0x8004xxxx`..`0x8043xxxx`) are imported absolute values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Wave 33 |
|---|---:|
| New translated functions | 30 |
| Bolt actor bodies (`0x806E9F40..0x806EB880`) | 25 |
| `lift_torokko` family bodies (`0x806EBC40..0x806EC080`) | 5 |
| Selected bytes | 4,956 |
| Translated functions in `src/game/bases.rs` (cumulative) | 849 (847 translated, 2 partial) |
| Cumulative selected bytes | 250,136 (22.54 % of indexed bytes) |

The 30 new bodies select 4,956 of the 1,109,860 indexed bytes (0.45 %) and 30
of the 5,409 eligible entries (0.55 %). 28 rows are `high` confidence; two are
`medium` (`0x806EAE50`, `0x806EB6B0`) for the reasons listed under
*Unknowns preserved*.

## Selection

The pass started from `functions.tsv`, the existing `bases.rs`, and every
`ledger/agents/*.csv` row with `module = game::bases`. All addresses already
translated by waves 2–32 were removed before selection. Wave 32 worked inside
`0x80750BF0..0x80757220`; this pass moved to the `0x806E0000` 64 KiB bucket,
which still held 203 untranslated entries, and took the first contiguous
complete cluster above the wave-28 window:

1. **Bolt actor (`0x806E9F40..0x806EB880`).** Twenty-five bodies that share the
   `0x700`-byte object built by `0x806EAB20` (vtable `0x807FE2CC`). The model
   loader `0x806EADA0` opens `g3d/bolt.brres`; `0x806EA110` is the start-up
   body and the same object owns the four pad latches at `+0x5F4`, the rider
   lists at `+0x408`/`+0x40C`/`+0x410`, the six `0x60`-byte blocks at
   `+0x534..+0x5F4`, and the movers at `+0x4E0`, `+0x54C` and `+0x604`.
   The model loader of the neighbouring actor (`0x806E9E90`) was already
   translated in wave 28, so the cluster starts exactly at the first
   untranslated body.
2. **`lift_torokko` (`0x806EBC40..0x806EC080`).** The `0x1398`-byte
   constructor `0x806EBDC0` builds two mirrored halves with the `0x180`-byte
   state array `+0x5A4` and the `0x5C`-byte array `+0x1044`; `0x806EBF10` and
   `0x806EBF90` are its element constructor/destructor pair, `0x806EC080`
   loads `g3d/lift_torokko.brres`, and `0x806EBC40` is the small helper
   teardown that sits between the two actors.
3. **Skipped veneers.** Four 48-byte `0x802DCBFC` compiler thunks were left
   untouched (see *Deferred candidates*); they have no Rust-visible effect and
   would only add noise to the ledger.

## Clusters

### Bolt actor (`0x806E9F40..0x806EB880`)

- `0x806E9F40` collects the active-player mask for one collision entry and its
  optional rider: it gates on `+0x38C == 1`, reads the character id through
  virtual `+0x6C`, tests `1 << id` against the global mask `0x80429CA8`, and
  follows the rider returned by `0x8012DBE0`.
- `0x806EA030` ORs that mask over the three collision lists `+0x408`,
  `+0x40C` and `+0x410`; the third list advances through
  `0x80075230(entry, actor + 0x3FC)`. `0x806EA0F0` counts the low four bits.
- `0x806EA110` starts the actor: writes the offset block `+0xD0..+0xE4`, the
  scale pair `+0x318..+0x324`, calls the model loader `0x806E9E90`, registers
  the `0x8007FA40` range over `+0x3FC`, stores the mover length `+0x5CC` and
  pane size `+0x5D0`, then fires state event `0x8082E548`.
- `0x806EA330` rebuilds the transform (`0x8006CED0` position, `0x801C0BD0`
  matrix at `+0x7C`, `0x8016EEF0` angle from `+0x104`).
- `0x806EA410` places the actor on its first mover and fires `0x8082E588` when
  the `0x80358E50` binding is gone; `0x806EA5C0` is the plain
  position-copy form. `0x806EA4E0` fires the full-range event `0x8082E5C8`
  when the global limit `0x80429CA0` is met; `0x806EA550` plays the boarding
  sound through `0x800E41C0` and hands the stored speed `+0x5CC` to
  `0x800C1510`.
- `0x806EAB20` builds the `0x700`-byte object: shared base `0x80066FC0`,
  vtable `0x807FE2CC`, accessor `+0x3D4`, `0x40`-byte array `+0x3F4`,
  `0x60`-byte array `+0x534` and mover `+0x604`.
- `0x806EABF0`/`0x806EAC40`/`0x806EAC90`/`0x806EACB0` are the pad-latch
  quartet (`0x800B3600` block, `0x800B3750` release, word clear, per-player
  update); `0x806EB010`, `0x806EB060` and `0x806EB0E0` apply them to the four
  slots every frame.
- `0x806EADA0` loads `g3d/bolt.brres` and binds the single pane.
- `0x806EAE50` scans the rider list `+0x5A4` and writes the best carried-item
  slot plus the mask held; `0x806EAF80` notifies every rider with
  `0x80056370(rider, actor, 9)`; `0x806EB600` couples the scan to the latch
  update and fires `0x8082E688` when an item is carried.
- `0x806EB540` is the second-mover position copy with release event
  `0x8082E628`; `0x806EB6B0` advances the carried-item speed with the two
  clamps `0x807CBED4`/`0x807CBE98` and the two `0x80199390` events.
- `0x806EA9A0` and `0x806EB880` are the two destructors (helper block and the
  full `0x700`-byte actor release chain).

### `lift_torokko` (`0x806EBC40..0x806EC080`)

- `0x806EBDC0` allocates `0x1398` bytes and builds two mirrored halves:
  vtable `0x807FE540`, accessors `+0x3D4`/`+0x3D8`, model holders
  `+0x3F0..+0x3F8` and `+0xEA4..+0xEAC`, animation blocks `+0x438`/`+0x470`
  and the mirrored pair at `+0xEEC`/`+0xF24`.
- `0x806EBF10` builds one `0x180`-byte state element (vtables `0x807FE65C`
  then `0x807FE634`, blocks `+0x08`, `+0x48`, `+0x80`); `0x806EBF90` is its
  destructor and frees through `0x802B9280`.
- `0x806EC080` loads `g3d/lift_torokko.brres` through `0x800DF180` and stores
  the resource at `+0x04`; `0x806EBC40` is the small helper teardown.

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave33-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x806E9F40` | `fn_806e9f40` | player mask for one entry and its rider | translated | high | `806e9f40_FUN_806e9f40.c` |
| `0x806EA030` | `fn_806ea030` | OR the masks of three collision lists | translated | high | `806ea030_FUN_806ea030.c` |
| `0x806EA0F0` | `fn_806ea0f0` | count the low-nibble player bits | translated | high | `806ea0f0_FUN_806ea0f0.c` |
| `0x806EA110` | `fn_806ea110` | start the bolt actor and open its range | translated | high | `806ea110_FUN_806ea110.c` |
| `0x806EA330` | `fn_806ea330` | rebuild the bolt transform | translated | high | `806ea330_FUN_806ea330.c` |
| `0x806EA410` | `fn_806ea410` | place on the mover / raise release event | translated | high | `806ea410_FUN_806ea410.c` |
| `0x806EA4E0` | `fn_806ea4e0` | fire the full-range event at the player limit | translated | high | `806ea4e0_FUN_806ea4e0.c` |
| `0x806EA550` | `fn_806ea550` | boarding sound and mover speed | translated | high | `806ea550_FUN_806ea550.c` |
| `0x806EA5C0` | `fn_806ea5c0` | copy the mover position into the offset | translated | high | `806ea5c0_FUN_806ea5c0.c` |
| `0x806EA9A0` | `fn_806ea9a0` | destroy the small helper block | translated | high | `806ea9a0_FUN_806ea9a0.c` |
| `0x806EAB20` | `fn_806eab20` | construct the `0x700`-byte bolt actor | translated | high | `806eab20_FUN_806eab20.c` |
| `0x806EABF0` | `fn_806eabf0` | latch a pad press and block it | translated | high | `806eabf0_FUN_806eabf0.c` |
| `0x806EAC40` | `fn_806eac40` | release a latched pad press | translated | high | `806eac40_FUN_806eac40.c` |
| `0x806EAC90` | `fn_806eac90` | clear one latch word | translated | high | `806eac90_FUN_806eac90.c` |
| `0x806EACB0` | `fn_806eacb0` | update one latch from pressed/held | translated | high | `806eacb0_FUN_806eacb0.c` |
| `0x806EADA0` | `fn_806eada0` | load `g3d/bolt.brres` and its pane | translated | high | `806eada0_FUN_806eada0.c` |
| `0x806EAE50` | `fn_806eae50` | scan riders for the best carried slot | translated | medium | `806eae50_FUN_806eae50.c` |
| `0x806EAF80` | `fn_806eaf80` | notify every rider | translated | high | `806eaf80_FUN_806eaf80.c` |
| `0x806EB010` | `fn_806eb010` | clear the four pad latches | translated | high | `806eb010_FUN_806eb010.c` |
| `0x806EB060` | `fn_806eb060` | feed the frame masks into the latches | translated | high | `806eb060_FUN_806eb060.c` |
| `0x806EB0E0` | `fn_806eb0e0` | release every pad latch | translated | high | `806eb0e0_FUN_806eb0e0.c` |
| `0x806EB540` | `fn_806eb540` | second-mover position copy | translated | high | `806eb540_FUN_806eb540.c` |
| `0x806EB600` | `fn_806eb600` | refresh latches and raise the carry event | translated | high | `806eb600_FUN_806eb600.c` |
| `0x806EB6B0` | `fn_806eb6b0` | advance the carried-item speed | translated | medium | `806eb6b0_FUN_806eb6b0.c` |
| `0x806EB880` | `fn_806eb880` | destroy the bolt actor | translated | high | `806eb880_FUN_806eb880.c` |
| `0x806EBC40` | `fn_806ebc40` | destroy the small helper block | translated | high | `806ebc40_FUN_806ebc40.c` |
| `0x806EBDC0` | `fn_806ebdc0` | construct the `0x1398`-byte `lift_torokko` | translated | high | `806ebdc0_FUN_806ebdc0.c` |
| `0x806EBF10` | `fn_806ebf10` | build one `0x180`-byte state element | translated | high | `806ebf10_FUN_806ebf10.c` |
| `0x806EBF90` | `fn_806ebf90` | destroy one `0x180`-byte state element | translated | high | `806ebf90_FUN_806ebf90.c` |
| `0x806EC080` | `fn_806ec080` | load `g3d/lift_torokko.brres` | translated | high | `806ec080_FUN_806ec080.c` |

**Totals:** 30 translated, 0 partial, 0 stub (28 high, 2 medium).

## Deferred candidates

| Address | Size | Why deferred |
|---|---:|---|
| `0x806EAAC0` | 48 | `0x802DCBFC` thin veneer; jumps to the shared destructor stub with no Rust-visible effect. |
| `0x806EAAF0` | 48 | `0x802DCBFC` thin veneer; same stub pair. |
| `0x806EBD60` | 48 | `0x802DCBFC` thin veneer; same stub pair. |
| `0x806EBD90` | 48 | `0x802DCBFC` thin veneer; same stub pair. |

The four thunks are the standard compiler veneer pair emitted around the
`0x806EAB20` and `0x806EBDC0` constructors. They carry no source-level
behaviour beyond an indirect jump, so a Rust body would be a bare `abi_call`
whose only purpose is to increment a count.

## Unknowns preserved

1. **`0x800DF180`.** The BRRES loader identity is not in this export; the two
   model loaders keep the `// unknown:` note.
2. **`0x802DE514`.** The compare-and-replace helper used by `0x806EAE50` is
   kept by address; only its ordering behaviour is inferred from the
   `<=`-then-assign shape.
3. **`0x802DCD60` / `0x802DCDAC`.** Ghidra prints these as `halt_baddata`
   save/restore fragments inside `0x806EAE50`; the reconstruction lets the
   returned frame value alias `out_slot`, which is why the row is `medium`.
4. **`0x80199390`.** The sound/event owner word and the `0x2BE` / `0x2BF` ids
   are kept by address in `0x806EB6B0`; the callee role is unresolved.
5. **`0x8007FA40` / `0x800C10F0`.** Both variadic-shaped main.dol helpers are
   modelled with the two private wrappers added at the end of this pass
   (`call_f64x4_u32x8`, `call_f32_u32x4`), because the C prints twelve and
   five arguments respectively.
6. **Bolt vtable split.** `0x806EB880` releases `+0x604` through the
   `0x802DCA98` array descriptor while `0x806EAB20` builds the same field with
   `0x800C0D70`; both are kept as printed and the mismatch is noted in the
   body doc.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
node tools/check-lane-csv.mjs wave33
```

`src/game/bases.rs` has 849 functions after this pass (847 `translated`, 2
pre-existing `partial`); this pass adds 30, all with `# Original` blocks and the
synthetic-base warning. `cargo check` reports no errors and no warnings in
`src/game/bases.rs`. `node tools/doc-lint.mjs src` reports `ok: true`, 0
violations over 68 files. `node tools/check-lane-csv.mjs wave33` reports
`wave33-bases.csv: 30 rows` with no problems attributable to this lane.
