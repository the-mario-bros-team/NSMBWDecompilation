# Coverage - Wave 54 (`game::bases`, `0x8072` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL (synthetic base `0x80600000`; every address is a static analysis address,
not a runtime address). Byte counts come from
`work/ghidra/rel-d_basesNP/export/functions.tsv`; the quoted evidence files are
the Ghidra C exports under `work/ghidra/rel-d_basesNP/export/`. Confidence is
`high` when the guards, stores, animation ids and descriptor addresses are
literal, and `medium` when a role, a dropped argument or a callee identity had
to be inferred from the surrounding calls.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 29 | 3900 | 22 | 7 |

## What this pass covers

- The Peach-castle block helper tail, `0x80720230..0x80720A90`: the freeing
  destructor [`fn_80720230`] (same shape as the wave-30 state cleanup
  [`fn_80720290`]), the constructor tail [`fn_80720630`] that installs vtable
  `0x80809F38` and publishes the `0x168`-byte helper at `+0x60` into the global
  `0x808301F8`, the free-only wrapper [`fn_80720890`], and the `0x14`-frame
  respawn debounce [`fn_80720a90`].
- The `ship_kyoro` / `ship_find` / `carryaway` actor whose constructor is the
  wave-30 [`fn_80721030`], `0x80721140..0x80722110`: the destructor
  [`fn_80721140`] (clip `+0x5C0`, animation `+0x584`, model `+0x544`), the
  matrix writer [`fn_807215e0`], the face-table angle scale [`fn_807216e0`],
  the aim reset [`fn_80721790`] and approach [`fn_807217b0`], the `face_1`
  bone attach [`fn_807219f0`], the lock-on cue [`fn_80721a90`] (sound `0x444`)
  and the three animation starts [`fn_80721b20`] (`ship_kyoro`),
  [`fn_80721d10`] (`ship_find`) and [`fn_80722000`] (`carryaway`, cues `0x59A`
  and `0x445`), plus the per-frame update [`fn_80721e20`] (cues `0x442`/`0x443`
  and the `ship_find_wait` hand-off) and the target snap [`fn_80722110`].
- The adjacent `0x5A8`-byte object's state reset [`fn_8072ac00`], the
  `lift_zen_chika_pori` variant binder [`fn_8072cc60`] and the `hikari` child
  cleanup [`fn_8072cf80`].
- The watch/aim actor tail, `0x8072F9D0..0x8072FD20`: the lazy eight-float
  direction table and flag decoding [`fn_8072f9d0`], the two watched-yaw cache
  queries [`fn_8072fc60`] / [`fn_8072fcc0`], and the packed face step
  [`fn_8072fd20`].
- The rotating-pedestal state helpers, `0x8072E7B0..0x8072ED30`: the scale/timer
  reset [`fn_8072e7b0`], the view-window test [`fn_8072eaa0`], the five mirrored
  aim angles [`fn_8072eb20`], and the countdown + step clamps
  [`fn_8072ebc0`], [`fn_8072ec80`], [`fn_8072ed30`] with descriptors
  `0x80830A18`, `0x80830A58` and `0x808309D8`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80720230` | `fn_80720230` | 88 | high |
| `0x80720630` | `fn_80720630` | 88 | high |
| `0x80720890` | `fn_80720890` | 64 | medium |
| `0x80720a90` | `fn_80720a90` | 132 | medium |
| `0x80721140` | `fn_80721140` | 144 | high |
| `0x807215e0` | `fn_807215e0` | 156 | high |
| `0x807216e0` | `fn_807216e0` | 84 | medium |
| `0x80721790` | `fn_80721790` | 32 | high |
| `0x807217b0` | `fn_807217b0` | 72 | medium |
| `0x807219f0` | `fn_807219f0` | 156 | high |
| `0x80721a90` | `fn_80721a90` | 144 | high |
| `0x80721b20` | `fn_80721b20` | 296 | high |
| `0x80721d10` | `fn_80721d10` | 244 | high |
| `0x80721e20` | `fn_80721e20` | 472 | high |
| `0x80722000` | `fn_80722000` | 244 | high |
| `0x80722110` | `fn_80722110` | 112 | high |
| `0x8072ac00` | `fn_8072ac00` | 40 | high |
| `0x8072cc60` | `fn_8072cc60` | 124 | high |
| `0x8072cf80` | `fn_8072cf80` | 40 | medium |
| `0x8072f9d0` | `fn_8072f9d0` | 180 | high |
| `0x8072fc60` | `fn_8072fc60` | 96 | high |
| `0x8072fcc0` | `fn_8072fcc0` | 96 | high |
| `0x8072fd20` | `fn_8072fd20` | 100 | medium |
| `0x8072e7b0` | `fn_8072e7b0` | 68 | high |
| `0x8072eaa0` | `fn_8072eaa0` | 128 | medium |
| `0x8072eb20` | `fn_8072eb20` | 116 | high |
| `0x8072ebc0` | `fn_8072ebc0` | 144 | high |
| `0x8072ec80` | `fn_8072ec80` | 136 | high |
| `0x8072ed30` | `fn_8072ed30` | 104 | high |

## Deliberate skips

- `0x80720570`, `0x807205a0`, `0x80720d90` and `0x80720dc0` are 48-byte
  adjustor thunks whose whole body is `0x802DCBFC(a2, a2, a1)` with no
  recoverable object role; like the wave-52 and wave-53 thunks they add no
  evidence.
- `0x807208d0` is a second free-only wrapper identical to
  [`fn_80720890`]; only one copy was translated so the lane does not pad its
  count with duplicate bodies.
- `0x80720980` decompiles with "bad instruction data" and a `halt_baddata()`
  tail, and its caller `0x807209b0` would only wrap that truncated body.
- `0x80721290`, `0x80721020` and `0x8072a760` are a one-store setter, a constant
  vtable getter and a third copy of the freeing destructor; they were dropped
  in favour of the larger shared-field bodies.
- `0x80721800` (484 bytes) is the aim-clamp body: its two virtual `+0x70`
  calls write into overlapping four-byte slots that Ghidra models as
  `local_1c`/`local_18[0]`, so the exact pair of output components cannot be
  recovered without inventing the prototype. [`fn_80721e20`] keeps its call as
  an address-only call to `0x80721800`.
- `0x8072fb30` calls `0x80162D50(0x1A4)` and recurses without ever advancing the
  search cursor; the export drops the second argument that the translated
  sibling [`fn_806cafb0`] uses. Translating it as a loop would invent that
  argument; translating it literally would produce a non-terminating body.
- `0x8072fd90` depends on `0x807073D0` called with no arguments plus a
  `(uint)diff >> 0xE & 3` sign idiom; the argument drop and the idiom make the
  step ambiguous.
- `0x8072e520` and the projectile record writer `0x8072fba0` remain
  untranslated; they are called by address from this pass and carry
  paired-single decompilation problems.
- The remaining `0x8072` bucket (153 untranslated at lane end) is outside this
  window.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv`.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000` line.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error or warning for
  `src/game/bases.rs`.
- `node tools/check-lane-csv.mjs wave54` reports `OK` for `wave54-bases.csv`.
