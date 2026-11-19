# Coverage - Wave 116 (GX / render band, `0x80280000` bucket)

## How to read this file

This page covers the Wave 116 pass over the last five open entries of the
`0x80280000..0x8028FFFF` bucket of the GX/render lane (`src/render/gx_band.rs`).
Addresses are virtual addresses of the unrelocated `main.dol` (image base
`0x80004000`); byte counts come from
`work/ghidra/main-gb/export/functions.tsv`, never from prose. The
machine-readable mirror is `ledger/agents/wave116-gx-band.csv` (one row).

The bucket held 158 entries, five of them untranslated, when this pass started.
One is landed here; the other four are documented under "Deliberate skips" with
the concrete state of their evidence.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/gx_band.rs` | `render::gx_band` | 1 | 4,280 | 0 | 1 |

## What this pass covers

- **The float keyframe sampler `0x8028B6C0`** (4,280 bytes, `medium`). The third
  member of the keyframe-sampler family, and the first one whose payload is a
  `float` per channel instead of a byte. Waves 104, 107, 110 and 113 deferred it
  with the note "same family, another channel width, same open questions"; the
  export turned out to be complete (no `in_fN` placeholder, no `pcVar`
  temporary), so this pass transcribed it.
- The frame-index half is the same five-arm resolver wave 100 wrote as
  [`track_frame_index`] for `0x8028DC70`, but the samplers additionally need the
  segment counter (`cell`, the export's `uVar20`) that seeds the ramp hash and
  the *continuous* position (`fVar6`) they compare against the truncated frame.
  Both are now returned by the new shared helper [`track_frame_position`]; the
  frame it returns is identical to `track_frame_index`'s.
- The descriptor is now fully mapped and is different from the byte track:
  `+0x03` channel mask, `+0x04` flags (`0x20` explicit step, `0x40` ping-pong,
  `0x80` reversed, `0x04` "do not advance the ramp noise"), `+0x05` step,
  `+0x06` track salt, `+0x08` key count, `+0x0C` base offset, `+0x10` wrap-base
  offset, `+0x20` key count reused as the search bound, `+0x24` key table.
- Every key entry is `4 * channels + 0x0C` bytes: `u16` time, a mode byte at
  `+0x02`, one curve byte per channel at `+0x04`, and one payload word per
  channel at `+0x0C` (a plain float, or a `u16` ramp-record index).
- Ramp records live at `desc + 0x20 + i32[+0x10] + i32[+0x0C]` and are
  `8 * channels + 4` bytes: two leading bytes then `(value, delta)` pairs. The
  byte read just past a record's pairs is the dither flag; a set flag advances
  the record LCG once and negates each sample while the seed's `0x10000` bit is
  clear.
- The seed is the five-term multiply/add (`0x3F81F635`, `0x30A74193`,
  `0x0007B929`, `0x371097E7`, `0x0004BF53`) followed by the three byte steps
  `b2 ^= b3`, `b1 ^= b2`, `b0 ^= b1`. Those steps were re-read from the raw
  instructions (`stw` to a stack slot, three `lbz`/`xor`/`stb` pairs, `lwz`
  back) because the export prints them as byte-typed sub-fields of an
  `undefined4` that reads like an uninitialised byte: it is `(byte)local_78`
  in C, i.e. the low byte that was stored one statement earlier, so the
  sequence is well defined and is reproduced as written here.
- Between two keys the value is blended with the curve byte of the *first* key
  through the new [`band_key_blend`] helper: linear, smoothstep, the two-tension
  cubic, hold, and "snap to zero". The export inlines that block once per arm;
  the helper keeps the arithmetic and the `sdata2` reads identical.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8028B6C0` | `fn_8028b6c0` | 4,280 | medium |

## Deliberate skips

Four entries of the bucket stay untranslated. Each was re-read from
`functions.tsv` and from its export before being left open.

| address | bytes | export lines | dropped blocks | why |
|---|---:|---:|---:|---|
| `0x80289410` | 4,792 | 745 | 1 | Byte twin of the landed body: same five arms and the same hash/LCG, plus a per-key 16-bit dither stage (`0x8042EC08` = `65536.0`) that shrinks each key byte before the curve blend. Its export is complete; it was deferred for budget, not for evidence. |
| `0x8028A6F0` | 4,012 | 608 | 1 | Sibling of the landed body with a different ramp geometry: the record stride is `8 * channels` (no `+4` payload prefix) and the payload is read at entry `+0x30`, so the landed derivation does not transfer and it needs its own pass. Export is complete. |
| `0x8028F860` | 1,600 | 1,458 | 53 | Selector `4` of the shape dispatcher. The export spills ten entry registers it never defines (`in_f27..in_f31`, `in_ps27_1..in_ps31_1`, 40 uses) and drops 53 blocks, so the quadratic's root-pair acceptance and the tangent-frame tail cannot be read without inventing the frame layout. |
| `0x8028FEA0` | 2,024 | 1,737 | 63 | Selector `5`. Same defect, larger: 63 dropped blocks and the same ten undefined entry registers, on top of a per-plane root solve whose smallest-`t` bookkeeping lives entirely in the dropped arms. |

### Why `0x8028B6C0` has only three curve arms

The export nests four blocks: "first key has a ramp only", "both keys have a
ramp", "neither", and "second key only". The raw stream tests the *second* key's
mode byte first (`0x8028BE0C`) and routes the first-key-only case into the plain
arm; the first-key-only block at `0x8028BF44` is only reachable if that test is
skipped, which no edge in the function does. The body therefore reproduces the
three live arms and states the inference in its doc block instead of shipping a
dead fourth arm.

## Verification

- Every address and byte size matches `functions.tsv` (1 row, 4,280 bytes).
- The evidence file exists at
  `work/ghidra/main-gb/export/8028b6c0_FUN_8028b6c0.c`.
- The row has exactly one `pub unsafe fn fn_8028b6c0` body in
  `src/render/gx_band.rs` (the module now holds 333 translated entries) plus
  three shared helpers added this pass: `track_frame_position`,
  `band_key_blend` and `key_hash`.
- `0x8028B6C0` was absent from `ledger/functions.csv` before the pass
  (confirmed by re-reading the ledger for the bucket).
- `node tools/check-lane-csv.mjs wave116` reports `OK` (the lane CSV has 1 row).
- `cargo check --message-format=short` reports no error from
  `src/render/gx_band.rs`; the three `unused_assignments` warnings at lines
  23286/23333/23431 belong to the wave-107 body `fn_8028cee0` and predate this
  pass.
- Constants were read from `work/dol/main.elf` with `r2 = 0x80433080`:
  `0x8042EBE8` = `0.0`, `0x8042EC04` = `FLT_EPSILON`, `0x8042EC0C` = `3.0`,
  `0x8042EC10` = `2.0`, `0x8042EC14` = `1.5`, `0x8042EC18` = `-2.0`.
- The instruction-level claims (hash byte order, entry `+0x02`/`+0x04`/`+0x0C`
  offsets, the `mid`/`low`/`high` roles in the key search, and the mode test at
  `0x8028BE0C`) were checked against the raw big-endian words in
  `work/dol/main.elf`, not only against the export.
