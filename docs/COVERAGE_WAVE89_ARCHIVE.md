# Coverage - Wave 89 (resource::archive, `0x80260000` bucket)

## How to read this file

Every row names one Ghidra function by its shipped-image address, the Rust body
that replaces it in `src/resource/archive.rs`, its exact size from
`work/ghidra/main-gb/export/functions.tsv`, and how confident the reconstruction
is. `address` is a virtual address in `sys/main.dol` (image base `0x80004000`),
not a file offset. The 29 bodies are appended after the wave-79 section at the
end of `src/resource/archive.rs` and are listed in address order.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/archive.rs` | `resource::archive` | 29 | 3780 | 12 | 16 |

One additional body (`fn_8026f630`) is `low` confidence, so the rows split
12 high / 16 medium / 1 low. The file now holds 380 translated bodies: 351
before this pass plus the 29 below. The assigned `0x80260000` bucket had 104
untranslated functions of 260 when this lane started; 29 of them are taken here.

## What this pass covers

The `0x80260000` band holds the NW4R sound manager, its 0x48-byte channel
descriptors and the handle-node lists that the wave-41/73 sections only touched
at the edges. This pass takes the remaining small and medium bodies whose
offsets, masks and callee addresses are all visible in the exports:

1. **Interrupt-masked handle stores and getters** - `0x80262200`, `0x802624a0`,
   `0x802626f0`, `0x80262750`, `0x802627c0` (5 bodies). The `+0x96` halfword
   store with the `0x800` flag, the inclusive `+0x10` range test, the node
   clear-and-insert, the three-mode rate conversion and the guarded handle
   release.
2. **Block comparator and copies** - `0x80262FE0`, `0x80263110`, `0x80263810`
   (3 bodies). Eleven halfword compares plus the `(a-b | b-a) >> 31` equality
   tail, the six-byte `+0xDC` colour copy for density mode 3, and the
   eight-value/sixteen-slot interleaved block copy.
3. **Mode selector and the three-list singleton** - `0x80264680`, `0x80264AE0`,
   `0x80264B70`, `0x80264C10`, `0x80264F90`, `0x80265060`, `0x802656F0`
   (7 bodies). The five-case `+0x30`/`+0x32` ladder, the singleton at
   `0x803FC91C` with its three self-linked heads and `0x802DC780`
   registration, the descriptor destructor, the one-shot 0x48-byte split, the
   node remove/destroy/re-link pair and the `RBKN` relative-pointer decoder.
4. **Fixed field initialisers and counters** - `0x80265DB0`, `0x80265E80`,
   `0x802667D0`, `0x80266D80`, `0x802676C0` (5 bodies). The 0xAC-byte block
   filled from the two `r2` defaults, the vtable-plus-three-blocks constructor,
   the `+0x88` two-mode test, the two saturating counters and the `+0x10`
   virtual forwarder.
5. **State setters, record initialisers and list scans** - `0x80268620`,
   `0x802686A0`, `0x80268710`, `0x80268D80`, `0x80269660`, `0x802696B0`,
   `0x80269E10`, `0x8026A1B0`, `0x8026F630` (9 bodies). The child re-bind, the
   two state-4 transitions, the four-byte list-unlink thunk, the two
   seven-field rate initialisers, the min-cost node scan, the full list drain
   and the `+0x08` tail forwarder.

### Shared layout notes

`// inferred:` the records are the sound-manager singleton, its channel
descriptors and the handle-node lists; the exports prove the offsets, the tags
and the callee addresses only.

- the singleton at `0x803FC91C` is `{ count, head, count, head, count, head }`
  with the three heads at `0x803FC920`, `0x803FC92C` and `0x803FC938`;
- a channel descriptor is 0x48 bytes and its list link is at `+0x40`; the
  `+0x1D` byte records that it is filed;
- the `0xAC`-byte field block starts with the two words at `r2 - 0x4840` and
  `r2 - 0x483C`, and the rate block receives `r2 - 0x47F0` scaled by
  `r2 - 0x47E8`;
- the cost scan stores each node's base 0x108 bytes before its link word, which
  is why the export reads `-0x70`, `-0xB8` and `-0x108` from the loop cursor.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80262200` | `fn_80262200` | 152 | medium |
| `0x802624A0` | `fn_802624a0` | 152 | medium |
| `0x802626F0` | `fn_802626f0` | 92 | high |
| `0x80262750` | `fn_80262750` | 100 | high |
| `0x802627C0` | `fn_802627c0` | 16 | medium |
| `0x80262FE0` | `fn_80262fe0` | 292 | high |
| `0x80263110` | `fn_80263110` | 216 | medium |
| `0x80263810` | `fn_80263810` | 136 | high |
| `0x80264680` | `fn_80264680` | 224 | medium |
| `0x80264AE0` | `fn_80264ae0` | 140 | medium |
| `0x80264B70` | `fn_80264b70` | 132 | high |
| `0x80264C10` | `fn_80264c10` | 188 | medium |
| `0x80264F90` | `fn_80264f90` | 204 | high |
| `0x80265060` | `fn_80265060` | 148 | high |
| `0x802656F0` | `fn_802656f0` | 144 | medium |
| `0x80265DB0` | `fn_80265db0` | 196 | high |
| `0x80265E80` | `fn_80265e80` | 184 | medium |
| `0x802667D0` | `fn_802667d0` | 32 | high |
| `0x80266D80` | `fn_80266d80` | 52 | high |
| `0x802676C0` | `fn_802676c0` | 52 | medium |
| `0x80268620` | `fn_80268620` | 128 | medium |
| `0x802686A0` | `fn_802686a0` | 104 | high |
| `0x80268710` | `fn_80268710` | 116 | high |
| `0x80268D80` | `fn_80268d80` | 4 | medium |
| `0x80269660` | `fn_80269660` | 68 | medium |
| `0x802696B0` | `fn_802696b0` | 64 | medium |
| `0x80269E10` | `fn_80269e10` | 196 | medium |
| `0x8026A1B0` | `fn_8026a1b0` | 240 | medium |
| `0x8026F630` | `fn_8026f630` | 8 | low |

## Unknowns

1. **The `r2` defaults.** `r2 - 0x47F0`, `-0x47EC`, `-0x483C` and `-0x4840`
   are read at runtime; the bodies read the slots but do not invent values.
2. **`0x802DC780`, `0x80281820`, `0x80281170`, `0x80281370`, `0x801A0300`,
   `0x801A0510`.** The exports prove the argument lists only; they are released
   as raw call targets.
3. **`0x80261D60`'s return.** Its own export is typed `void`, but
   `0x80264C10` consumes its result as the descriptor base. The call is
   transcribed as returning its register and the row is `medium`.
4. **`0x80268D80`.** The entry is a four-byte tail thunk to `0x80229190`; the
   export inlines the target's unlink body, which is what the Rust body
   reproduces.
5. **`0x8026F630`.** The export prints only `FUN_8026d9b0(param_1 + 8)`,
   although that callee takes `(list, pointer, length)`. The pointer is proved,
   the range is not, so the row is `low`.
6. **`0x80269E10`'s node layout.** The negative offsets `-0x70`, `-0xB8` and
   `-0x108` are literal but the record's field roles are inferred.

## Deliberate skips

- **`0x80260A80`, `0x802610C0`, `0x80261B20`, `0x80262DB0`, `0x80263A70`,
  `0x802666C0`, `0x80267420`, `0x802698E0`, `0x80269990`** - the interpolators
  and pitch setters are `CONCAT44(0x43300000, x ^ 0x80000000)` ladders with GQR
  scaling; wave 79 skipped the same shape and there is no `read_f64` helper in
  `src/platform.rs`.
- **`0x80261640`, `0x80261BF0`, `0x80261FB0`, `0x802620D0`, `0x80269F00`,
  `0x8026A2A0`** - each calls `0x802DCD6C`/`-70`/`-74`/`-BC`, which the export
  reveals as prologue/epilogue labels; the argument registers are mis-modelled,
  so an honest body cannot be written.
- **`0x80261E70`** - calls `0x80260860` and `0x80260EC0`; the latter is a
  `this`-relative getter and the export drops its receiver, so the mode
  arithmetic would rest on an invented argument.
- **`0x80267E30`** - a large literal initialiser, but its call to `0x802696B0`
  drops the destination pointer even though that callee writes 0xAC bytes.
- **`0x80269D00`** - the min-selection loop ends in an argument-less virtual
  call on a node that may be null.
- **`0x8026DF50`** - three `0x802758xx` calls share stack slots that the export
  aliases (`local_44` is read before it is written), so the local layout is not
  recoverable.
- **The remaining bucket** - `0x802605D0..0x802620D0`, `0x802629F0..0x80264B70`,
  `0x80264CD0..0x80265E80`, `0x80266DC0..0x80268790`, `0x80268910..0x80269D00`
  and `0x8026A420..0x8026D3B0` hold 0.4-1.8 KB manager, render and stream
  bodies left for a dedicated pass.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (read from the TSV, not from prose).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/archive.rs` with a complete `# Original` / `# Arguments` /
  `# Returns` / `# Side effects` / `# Safety` / `# Confidence` doc block.
- No address was already in `ledger/functions.csv`; all 29 addresses came from
  the untranslated part of the `0x80260000` bucket.
- `node tools/check-lane-csv.mjs wave89` reports 29 rows for
  `wave89-archive.csv` and `OK`.
- `cargo check --message-format=short` reports no error for
  `src/resource/archive.rs`.
