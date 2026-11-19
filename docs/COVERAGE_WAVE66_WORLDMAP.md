# Coverage - Wave 66 (WorldMap iceball projectile)

## How to read this file

Each row is one original `main.dol` function, reconstructed as a
`pub unsafe fn fn_<address>` body in `src/game/worldmap.rs`. The *address* is the
PowerPC virtual address from `work/ghidra/main-gb/export/functions.tsv`; the
*bytes* column is the exact size from that file; *confidence* matches the
`# Confidence` line of the body's doc comment. "high" means the loads, stores
and calls are literal in the export; "medium" means a field role or a callee
identity is inferred and called out in the body.

The cluster is the WorldMap ice projectile. Two effect spawners name it: the
per-frame update calls `0x80094560`, which posts `Wm_mr_iceball_b` and
`Wm_mr_iceball_a` at `+0xAC`, and `0x80123A00` calls `0x800943B0`, which posts
the sibling effect referenced from `0x8035631C`. The actor keeps its transform
at `+0xAC`/`+0xB0`/`+0xB4`, its height/collision state at `+0x1EC` and
`+0x2DC`, its flag word at `+0x274`, its motion record at `+0xE8..+0x11C`, its
state index at `+0x348`, its layer byte at `+0x38F` and its two vtable
sub-objects at `+0x60` and `+0x394`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 27 | 3624 | 22 | 5 |

## What this pass covers

- `0x80123130` and `0x80124A80` - the two identical release helpers: a null
  check, the `0x8015F800` teardown, and a signed delete test before
  `0x802B9280`. `0x80123250`, `0x80123280`, `0x80124BA0` and `0x80124BD0` are
  the three identical dispenser forwarders that pass `(b, b, a)` to
  `0x802DCBFC`.
- `0x801232B0` and `0x80124C00` - the two constructors. The first allocates
  `0x430` bytes, installs the `+0x60`/`+0x40C` vtables (`0x80324770`,
  `0x80317860`), initialises the `+0x3F0` child and clears `+0x420`/`+0x424`;
  the second allocates `0x70` bytes and installs `0x803249E0` at `+0x60`.
- `0x80123740..0x80123D70` - the ground/contact ladder. `0x80123740` gates on
  the `+0x1EC` height field and runs the six-argument probe `0x80075FD0`;
  `0x801237F0` probes `y + 10.0` with `0x800757B0` and accepts surfaces in
  `(y - 3.0, y + 10.0]`; `0x801238B0` publishes the position and a `120.0`
  lifetime to the `+0x40C` child; `0x80123970` sets `+0x274` bit `0x8000` from
  the `0x800D9940` collider test and returns the `0x3C1FE03F` mask;
  `0x80123B70` clamps the height by layer (`2000.0`/`-1800.0`); `0x80123CA0`
  is the gate over flags `0x3F`/`0x3C000000`, the `0x800D9920` collider and the
  `0x80070720`/`0x800708C0` animation indices; `0x80123D70` stores the
  `0x80075270` result at `+0x3DC` and edge-detects results `1..4`.
- `0x80123E60`, `0x80123F30`, `0x80123FF0` - the three contact handlers. They
  differ only in the posted event (`0x265`, `0x284`, `0x283`), the low bits of
  the `(layer << 16) | n` mode (`1`/`2`, `4`, `6`) and the final
  `0x80078410` frame (`8`, `0x12`, `0x19`).
- `0x80124100` and `0x80124340` - the two per-frame variants. Both run the
  iceball effects, count `+0x3D0` down and decrement the per-player
  `0x80375AB8` counter on expiry, then dispatch on the ground result, the
  `0x274` mask `0x1FE000` and the `+0x2DC` collider. The second variant adds
  the `0x8006CD90` transform slot and orders the `0x80123970` test before the
  collider probe, exactly as the export does.
- `0x80124250` - the motion writer: the `0x802F4D00`/`04`/`08`
  (`0.75`/`1.05`/`0.6`) flat-and-slope curve over the player's `+0xE8`, then
  the `0x8042D6C0` and `0x8042D6C8` state tables and the
  `3.7`/`0.0`/`-0.15`/`-3.0`/`0.04` stores.
- `0x801244D0`, `0x80124560`, `0x801245C0` - the exit/post paths. `0x801244D0`
  posts `0x80375B98` in state `2`; `0x80124560` runs slot `0x98` and hands the
  actor to `0x80162510`; `0x801245C0` is the per-player spawn limit
  (`0x80375AA8[player] < 6`, then `< 2` unless kind `1`).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80123130` | `fn_80123130` | 88 | high |
| `0x80123250` | `fn_80123250` | 48 | high |
| `0x80123280` | `fn_80123280` | 48 | high |
| `0x801232B0` | `fn_801232b0` | 116 | medium |
| `0x80123740` | `fn_80123740` | 176 | medium |
| `0x801237F0` | `fn_801237f0` | 180 | high |
| `0x801238B0` | `fn_801238b0` | 96 | high |
| `0x80123910` | `fn_80123910` | 84 | high |
| `0x80123970` | `fn_80123970` | 144 | high |
| `0x80123A00` | `fn_80123a00` | 8 | high |
| `0x80123B70` | `fn_80123b70` | 144 | high |
| `0x80123C00` | `fn_80123c00` | 88 | high |
| `0x80123CA0` | `fn_80123ca0` | 208 | high |
| `0x80123D70` | `fn_80123d70` | 228 | high |
| `0x80123E60` | `fn_80123e60` | 204 | medium |
| `0x80123F30` | `fn_80123f30` | 184 | medium |
| `0x80123FF0` | `fn_80123ff0` | 184 | medium |
| `0x80124100` | `fn_80124100` | 328 | high |
| `0x80124250` | `fn_80124250` | 224 | high |
| `0x80124340` | `fn_80124340` | 356 | high |
| `0x801244D0` | `fn_801244d0` | 88 | high |
| `0x80124560` | `fn_80124560` | 64 | high |
| `0x801245C0` | `fn_801245c0` | 76 | high |
| `0x80124A80` | `fn_80124a80` | 88 | high |
| `0x80124BA0` | `fn_80124ba0` | 48 | high |
| `0x80124BD0` | `fn_80124bd0` | 48 | high |
| `0x80124C00` | `fn_80124c00` | 76 | high |

## Deliberate skips

- No record in this run was skipped for a removed block, a GQR
  reconstruction or a dropped callee argument. Every export in
  `0x80123130..0x80124C00` was translated.
- Two small in-file helpers, `decrement_player_counter` and
  `post_contact_event`, are not ledger rows: they are the blocks that both
  per-frame exports (`0x80124100`, `0x80124340`) contain inline, factored out
  once as private `unsafe fn` helpers.
- The `0x801200C0..0x80121D50` and `0x801250D0..` groups in the same bucket are
  untouched by this pass: most of them carry `unaff_GQR0` / `ldexpf`
  reconstruction blocks, and the remaining ones belong to their own
  contiguous runs.
- `0x80123130`/`0x80124A80` and the four dispenser forwarders are byte-identical
  duplicates; each still gets its own body and ledger row because each is a
  separate original address in `functions.tsv`.

## Verification

- Every address and byte size matches `functions.tsv`; the 27 sizes sum to
  3,624 bytes.
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv` (all 27 were untranslated
  entries of the `0x80120000` bucket).
- `cargo check --message-format=short` reports no error in
  `src/game/worldmap.rs`; the only errors in the crate at the time of writing
  are in another lane's `src/runtime/bt.rs`.
- `node tools/check-lane-csv.mjs wave66` reports `OK` with this lane's 27 rows
  included.
