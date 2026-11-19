# Coverage - Wave 78 (engine::display)

## How to read this file

`src/engine/display.rs` is the semantic home of the engine-level 2-D
presentation layer. This pass adds the boot-screen half of coverage band H, the
addresses that fall in the `0x80150000` bucket: the `wipe*` readiness gates, the
three state-object destructors and the `dScBoot_c` strap/controller-info state
handlers. Every function below is a literal reconstruction of one Ghidra body;
the doc block above each Rust body names the original address, the byte count
from `work/ghidra/main-gb/export/functions.tsv` and the evidence file. "high"
confidence means every store, branch and call is visible in the export;
"medium" means a callee operand was dropped by the export or a field role was
inferred from a sibling handler.

This pass only edits `src/engine/display.rs`; the ledger is merged by the
coordinator. The scene-side functions of the same class stay in
`crate::game::boot_scene`, which owns `0x8015C130..0x8015D480` already; this pass
does not touch that file and does not duplicate any address it translated.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/display.rs` | `engine::display` | 28 | 2316 | 19 | 9 |

## What this pass covers

The `0x80150000` bucket entered this pass with 126 untranslated of its 209
functions. This pass takes five coherent slices of the same boot-screen band and
leaves the large descriptor tables and the address-returning accessors for a
later pass.

- **The readiness queries** (`0x8015BB90..0x8015BBF0`, 3 bodies).
  [`fn_8015bb90`] and [`fn_8015bbc0`] ask `0x8008F620` about one id and
  normalise the answer with the exported `countLeadingZeros(value - 1) >> 5`
  test; [`fn_8015bbf0`] requires the first id to answer exactly `1` before asking
  the second.
- **The scene-reference pair and the transition helper**
  (`0x8015BCE0..0x8015BD10`, 3 bodies). [`fn_8015bce0`] and [`fn_8015d080`]
  are the `+1`/`-1` pair over the scene reference count at `0x8042A2E0`;
  [`fn_8015bd10`] runs `0x8006A280` and always reports success.
- **The four `wipe*` layout gates** (`0x8015BF10..0x8015BFD0`, 4 bodies).
  Each gate tests one adjacent SDA slot (`0x80429C30..0x80429C48`) and then one
  transition-scene setup call: `0x8001C240` (`wipeKuppa`), `0x8001BC60`
  (`wipeDoor`), `0x8001C760` (`wipeFader`) and `0x8001B470` (`wipeCircle`).
- **The three state-object destructors** (`0x8015C260..0x8015C300`, 3 bodies).
  [`fn_8015c2a0`] and [`fn_8015c300`] release the embedded state object at
  `+0x18` through `0x8015F9F0` and then optionally free the storage through
  `0x802B9280`; [`fn_8015c260`] is the plain variant without the sub-object.
- **The `dScBoot_c` strap/controller-info handlers**
  (`0x8015CA50..0x8015DAA0`, 15 bodies). [`fn_8015cab0`], [`fn_8015ccc0`],
  [`fn_8015cfc0`] and [`fn_8015ced0`] route on the player-manager query;
  [`fn_8015cb20`] is the error-window/controller-info router with the
  `0x4B0/0x3C` timer pair; [`fn_8015ca50`] and [`fn_8015cd40`] feed the SDA2
  float through `+0xF0 -> +0x1F4`; [`fn_8015cdf0`] waits on the `+0x108`
  counter, the view event `0x7F1F` and input flag `0x80000`;
  [`fn_8015ce80`], [`fn_8015cf70`], [`fn_8015d030`], [`fn_8015d0a0`],
  [`fn_8015d780`], [`fn_8015d9c0`] and [`fn_8015daa0`] are the display-flag and
  timer helpers around them.

Two facts carry across the whole slice. First, `dScBoot_c + 0x74` is the state
interface: slot `+0x18` changes state, slot `+0x28` returns the descriptor
object and its slot `+0x14` answers an event query. Second, the descriptor
addresses are not invented: every `0x80377xxx` constant in this pass is a
`dScBoot_c::StateID_*` record registered by `0x8015DDD0`, and the truncated
names (`ControllerInf...`, `WiiStrapKeyWa...`) are quoted exactly as the export
prints them.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8015BB90` | `fn_8015bb90` | 48 | high |
| `0x8015BBC0` | `fn_8015bbc0` | 48 | high |
| `0x8015BBF0` | `fn_8015bbf0` | 72 | high |
| `0x8015BCE0` | `fn_8015bce0` | 16 | high |
| `0x8015BD10` | `fn_8015bd10` | 36 | high |
| `0x8015BF10` | `fn_8015bf10` | 64 | medium |
| `0x8015BF50` | `fn_8015bf50` | 64 | medium |
| `0x8015BF90` | `fn_8015bf90` | 64 | medium |
| `0x8015BFD0` | `fn_8015bfd0` | 64 | medium |
| `0x8015C260` | `fn_8015c260` | 64 | medium |
| `0x8015C2A0` | `fn_8015c2a0` | 96 | high |
| `0x8015C300` | `fn_8015c300` | 100 | high |
| `0x8015CA50` | `fn_8015ca50` | 96 | medium |
| `0x8015CAB0` | `fn_8015cab0` | 104 | high |
| `0x8015CB20` | `fn_8015cb20` | 388 | high |
| `0x8015CCC0` | `fn_8015ccc0` | 104 | high |
| `0x8015CD40` | `fn_8015cd40` | 80 | medium |
| `0x8015CDF0` | `fn_8015cdf0` | 128 | medium |
| `0x8015CE80` | `fn_8015ce80` | 80 | high |
| `0x8015CED0` | `fn_8015ced0` | 136 | medium |
| `0x8015CF70` | `fn_8015cf70` | 68 | high |
| `0x8015CFC0` | `fn_8015cfc0` | 104 | high |
| `0x8015D030` | `fn_8015d030` | 68 | high |
| `0x8015D080` | `fn_8015d080` | 24 | high |
| `0x8015D0A0` | `fn_8015d0a0` | 72 | high |
| `0x8015D780` | `fn_8015d780` | 40 | high |
| `0x8015D9C0` | `fn_8015d9c0` | 44 | high |
| `0x8015DAA0` | `fn_8015daa0` | 44 | high |

Byte total: `220` (readiness queries and the scene-reference pair) + `256` (the
four `wipe*` gates) + `260` (the three destructors) + `1580` (the fifteen
`dScBoot_c` handlers) = `2316`.

## Deliberate skips

- **`0x801543C0..0x80154460`** - eleven 12-byte accessors that only return
  `&DAT_8035425C..&DAT_803545DC`, a bank of adjacent `0x40`-byte records. They
  are honest but carry no behaviour, so they were left for a bulk accessor pass
  rather than padded into this lane.
- **`0x8015B320`** (1,276 bytes) and **`0x8015DDD0`** (4,716 bytes) - the pause
  descriptor registration and the `dScBoot_c` 24-descriptor initialiser. Both
  are pure `0x40`-byte table writes; the descriptor names they install are
  already quoted in the module header and in `crate::game::boot_scene`.
- **`0x80153400`** (3,832 bytes) - the large triangle/effect builder; it needs a
  table type the export does not provide and does not fit the boot-screen slice.
- **The remaining `dScBoot_c` handlers** `0x8015C4C0`, `0x8015C680`,
  `0x8015D1D0..0x8015D3A0`, `0x8015D5B0..0x8015D650` are reconstructable but
  were left inside the bucket for the next pass; they are not blocked, only
  outside this pass's 28-body budget.
- **`0x8015B2C0` / `0x8015B2F0`** (48 bytes each) - forwarding thunks that call
  `0x802DCBFC` with the first operand repeated
  (`FUN_802dcbfc(param_2, param_2, param_1)`). The duplicated operand is
  preserved literally in the export but the call shape adds no information.
- **`0x8015F130`, `0x8015F3B0`, `0x8015F410`, `0x8015F470`, `0x8015F7B0`** -
  five 4-byte thunks to `0x801B8620`, `0x8015F3C0`, `0x8015F420`, `0x8015F480`
  and `0x8015F6E0`; they are thunk-bank entries, not new bodies.
- **`0x8015C4C0`** - the `0x8015F620` task-run branch that then allocates two
  controller resources. Its branch chain is literal, but the two resource ids
  (`0x2C0`/`0x2E5`) reuse the readiness ids without a named owner, so it was
  left for the pass that also takes `0x8015D5B0..0x8015D650`.

## Verification

- Every address and byte size matches `functions.tsv` (checked mechanically:
  zero mismatches over the 28 rows, total 2,316 bytes).
- Every evidence file exists at the cited path and every row has exactly one
  `pub unsafe fn fn_<addr>` body in `src/engine/display.rs`.
- `ledger/functions.csv` was read only; none of the 28 addresses was present
  before this pass, and `ledger/agents/wave78-display.csv` has 28 unique rows.
- `node tools/check-lane-csv.mjs wave78` reports `OK` for this CSV.
- `cargo check --message-format=short` reports no error in
  `src/engine/display.rs`; the only error in the tree is
  `src/game/bases.rs:89439` from a parallel lane.
