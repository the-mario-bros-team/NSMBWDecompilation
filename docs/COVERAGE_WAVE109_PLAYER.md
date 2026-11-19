# Coverage - Wave 109 (`game::player`, the `0x80050000` bucket)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 6 | 14364 | 1 | 4 | 1 |

Status split: 6 `translated`, 0 `partial`. The `0x80050000` bucket is now
complete: `work/ghidra/main-gb/export/functions.tsv` lists 274 rows in the
bucket and all 274 are in `ledger/functions.csv` once this lane's six rows are
merged.

## How to read this file

* **Address** - virtual entry point in the DOL image, not a file offset.
* **Name** - the Ghidra name (`FUN_<address>`). No symbol table survives for
  these functions, so the Rust body keeps the `fn_<address>` name.
* **Role** - one plain sentence per function, in "What this pass covers". A
  role is marked "inferred" when it comes from shared field offsets and call
  shapes rather than from a recovered symbol.
* **Confidence** - strength of the evidence, not code quality: `high` = every
  address, offset and branch is literal; `medium` = a field role or a callee's
  meaning had to be inferred; `low` = only the literal instruction sequence is
  certain (see `fn_800583a0`).
* All six bodies use the same SDA bases the module already fixes,
  `r2 = 0x80433080` and `r13 = 0x8042F680`. The `W109_*` constants were
  re-read from `work/dol/main.elf` for this pass.

## What this pass covers

- **The per-frame flag interpreter `0x800544F0` (2540 B).** It calls the wall
  probe `0x80072440` from the `+0xB8` body edge back to the centre (offset
  `3.0 * side`, half the fixed-point word read from the `+0xF0` vtable slot),
  then converts the mask returned by `0x80071210(player + 0x1EC, player)` into
  `+0x10D4`/`+0x10D8` bits. It rotates `+0x1128`/`+0x112C` into
  `+0x112A`/`+0x112E`, latches `+0x27C` into `+0x110C`, selects the state id at
  `+0x111C` from the animation tag byte, and finishes with the four
  `fn_80056370` interface writes. `// inferred:` the mask groups are the
  action-state machine's direction (`0x2A`/`0x15`) and button
  (`0x3C000000`, `0x1FE000`, `0xC0`) fields.
- **The two-anchor ground probe `0x800569C0` (496 B).** Two vtable slots
  (`+0xF0`, `+0xF4`) return fixed-point anchors; their `+4`/`+8` words are
  divided by `4096.0`. The first anchor supplies two y offsets, the second the
  two x offsets `+reach`/`-reach`; four `0x80072440` segment walks follow, and
  the first blocking cell returns `1`.
- **The body-angle steering `0x800583A0` (1200 B).** With event flag `2` clear
  and neither the `+0x3CC` vtable slot nor flags `0x94`/`0x2E` set, the body
  picks one of `0x80069270`/`0x800693E0` from the `+0x160` flags, reads the
  target position through its `+0x70` slot, derives the bearing with
  `0x801611C0`, clamps it (`0xC00`/`0x2000`, or `0x2000`/`0x5000` when the
  target's `+0x380` state is `3`) and eases `+0x1FE`/`+0x200` with
  `0x8015F200`. The `GQR3` halfword read is reduced to a plain `s16`
  conversion, the same assumption the module records for the boot GQR state.
- **The state-descriptor table `0x800592C0` (7620 B).** Thirty-seven
  `daPlBase_c` state descriptors: the 18 gameplay states at `+0x40` stride
  from `0x8035421C` and the 19 demo/cutscene states at `+0x40` stride from
  `0x8035476C`. Each block initialises the name through `0x8015F7C0`, copies
  its nine-word payload from the contiguous records at `0x80308BB8` in the
  rotated order the export prints, stores the vtable `0x803095A0` and its
  factory, then registers the `-0x0C` node through `0x802DC780` with the shared
  destructor `0x8005B0F0`. `// unknown:` the payload words are only copied.
- **The platform publisher `0x8005B8A0` (1100 B).** Two passes over the
  presence byte `0x80429CA8`: the first files each accepted player under its
  own `+0x430` slot byte (flags `0x65`/`0x66`), the second insertion-sorts the
  four ids by descending height into `manager + 0x20..0x2C`, queries the first
  player's surface with `0x800757B0`, applies the `6.0`/`2.0` corrections, and
  writes a running `+0x438` height that starts `4.0` above each player and
  grows by `0.7` of its `+0x1030` body height.
- **The area placement `0x8005EF50` (1408 B).** For area modes `0`, `1` and
  `0x17` it resets the manager (`0x8005B4A0`, `0x8005F920`) and builds the
  base from `0x8005ED90`; the `0x8042A21F` byte then selects either the
  fixed `(-184, 200, -208, 224)` grid around x `504.0` or the random-key
  ordering with the `12.0`/`24.0` x strides. Every other mode posts the record
  position through `0x8005EEE0` four times.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800544F0` | `fn_800544f0` | 2540 | medium |
| `0x800569C0` | `fn_800569c0` | 496 | medium |
| `0x800583A0` | `fn_800583a0` | 1200 | low |
| `0x800592C0` | `fn_800592c0` | 7620 | high |
| `0x8005B8A0` | `fn_8005b8a0` | 1100 | medium |
| `0x8005EF50` | `fn_8005ef50` | 1408 | medium |

Byte total: 14364. Address range covered: `0x800544F0..0x8005EF50`.

## Deliberate skips and why

**None.** All six remaining rows of the bucket have a body. Two kinds of
detail are knowingly left out of the bodies, and both are stated in the
matching doc block:

* the outlined register-save prologues (`FUN_802DCD68`, `FUN_802DCD74`) and
  the `psq_l`/`GQR` blocks that only write dead stack temporaries;
* the exact register caching of `+0x10D4`/`+0x10D8` inside `0x800544F0`. The
  body re-reads each word at the point the export re-reads it, so the
  observable read-modify-write order is preserved.

## Notes for a later pass

- `0x80070900`, `0x800757B0`, `0x800757E0` and `0x80075800` are translated in
  `game::stage` as `void`, but their `r3` carries the callee's result and the
  call sites here store or test it. The wave-109 bodies therefore call the
  addresses directly (`W109_HEIGHT_80070900`, `W109_PROBE_800757xx`) instead of
  the void wrappers.
- `0x8005B8A0` writes `local_70[*(int *)(actor + 0x430) + 4]`; only the values
  `0..3` address the four-entry observation array, so the body guards the store
  and says so. A corrupt `+0x430` would write past the original array.
- The random placement in `0x8005EF50` inserts each index before the first
  stored key that is strictly greater (ascending order), which is what the four
  nested `if` blocks in the export compute; the unreachable "array already
  full" arm of that ladder is folded into the plain insertion loop.

## Verification

- Every address and byte size matches `functions.tsv`
  (`work/ghidra/main-gb/export/functions.tsv`, rows `800544f0` 2540,
  `800569c0` 496, `800583a0` 1200, `800592c0` 7620, `8005b8a0` 1100,
  `8005ef50` 1408).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/game/player.rs`.
- No address was already in `ledger/functions.csv`; all six were the bucket's
  remaining untranslated rows.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/player.rs`.
- `node tools/check-lane-csv.mjs wave109` reports `OK` with 6 rows in this lane.
