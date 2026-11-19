# Coverage - Wave 113 (GX / render band, `0x80280000` bucket)

## How to read this file

This page covers the Wave 113 pass over the last open entries of the
`0x80280000..0x8028FFFF` bucket of the GX/render lane
(`src/render/gx_band.rs`). Addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`); byte counts come from
`work/ghidra/main-gb/export/functions.tsv`, never from prose. The
machine-readable mirror is `ledger/agents/wave113-gx-band.csv` (two rows).

The bucket had seven untranslated entries when this pass started. Two of them,
`0x80287C10` and `0x80288E30`, are now landed; the other five are documented
under "Deliberate skips" with the concrete blocker that remains for each.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/gx_band.rs` | `render::gx_band` | 2 | 2,480 | 1 | 1 |

## What this pass covers

- **The 3x4 matrix composer `0x80288E30`** (572 bytes, `high`). It is the
  local-matrix builder that `0x80289070` and `fn_80294440` call with a
  destination 3x4, a parent 3x4, the three basis bits from the node's `+0x64`
  byte and the mode byte at `+0x65`. The body is a guard tree over seven calls:

  1. `0x801C04D0` writes the identity into the destination.
  2. `0x80295C00` copies the parent translation (`+0x0C`, `+0x1C`, `+0x2C`).
  3. `0x80238200` concatenates the destination with a translation matrix built
     from that vector; the raw stream passes the destination twice
     (`out, out, vector`), which Ghidra drops for two of the four call sites.
  4. `0x80295970` rebuilds an orthonormal basis from the parent (flag bit 1)
     and `0x801C0540` concatenates it.
  5. `0x80295C20` orthonormalises the parent basis (flag bit 0) and
     `0x80296530` scales the destination by it.

  Two early exits are literal: both basis flags set with `mode == 0x64`
  (`'d'`) is a plain twelve-word copy of the parent, and no basis flag with
  `mode == 0` returns the identity.

  The mode byte is `0x64` (`'d'`) for "translation only", `0` for "identity
  only" and otherwise a percentage that scales the parent translation by
  `mode / 100.0` (the `lfd`/`fsubs` pair at `0x80288F50` is the compiler's
  signed-integer-to-`float` conversion; the divisor is the `100.0f` at
  `0x8042EBB0`). The extra-translation flag (bit 2) translates the destination
  before the basis work and by its negation (`-1.0f` at `0x8042EBD0`)
  afterwards.

  The flag roles are confirmed by two callers: `0x80289070` passes
  `flags & 1`, `flags >> 1 & 1` and `flags >> 2 & 1`, and `fn_80294440`
  (already translated) passes the same three bits plus
  `*(i8*)(object+0x88)` as the mode.

- **The child spawn/clone path `0x80287C10`** (1,908 bytes, `medium`). It
  prepares the work record with `0x802874E0`, either on the cached record at
  `*(*(*(param_1+0xBC)+0x20)+0xC068)` or on a 0x110-byte stack record, then:

  1. copies the context's `+0xBC` pointer and back pointer, sets the `+0x64`
     flag bits from `param_1+0x20` (`0x1000`/`0x2000`/`0x800`) and the `+0x65`
     mode byte (`100` when `*(param_1+0xB8)+0x15` is non-zero);
  2. when `descriptor[7] & 1` is set and the first halfword is non-zero,
     builds a billboard basis from
     `(param_4[0xAC..0xB0] - param_4[0xB8..0xBC], param_4[0xB4] - param_4[0xC0])`
     through `0x801C1160`, `0x802960F0`, `0x80295860` and the record's Euler
     matrix (`0x802963B0`), then writes it back with `0x80295A90`;
  3. rebuilds the record position in the owner's space with the usual
     `0x802963B0`/`0x80296530`/`0x80289070`/`0x801C07A0`/`0x801C0540` chain,
     translates by `param_6` or `param_4+0xAC`, and adds the result to the
     saved position;
  4. runs the record's virtual slots `+0x18` and `+0x24`;
  5. walks the child list at `param_1+0xC0` through `0x80228FF0` matching
     `+0x24`, the three flag bits, the combined percentage at `+0x88` and
     `descriptor[5]` at `+0x89`; when no node matches, creates one through the
     factory slot `+0x38`, initialises it with slot `+0x10`, links it with
     `0x80228DF0`, fills the same fields, and notifies `+0x9C` through slot
     `+0x08`;
  6. copies the ten descriptor bytes into `record+0xFC..+0x105` (or clears
     `record+0xF8` for an empty descriptor), updates the owner reference count
     and releases the child/owner through their virtual slots when the states
     require it.

  The two percentages at `param_2+0x22` and `*(param_1+0xB8)+0x15` are signed
  bytes combined with `100` as the identity; the other case is the truncating
  signed `product / 100` that the raw `mulhw` + `0x51EB851F` sequence
  implements.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80287C10` | `fn_80287c10` | 1908 | medium |
| `0x80288E30` | `fn_80288e30` | 572 | high |

## Deliberate skips

Five entries of the bucket stay untranslated. Each was re-read from
`functions.tsv` and its export before being left open.

### The two sphere samplers (`0x8028F860`, `0x8028FEA0`)

| address | bytes | why |
|---|---:|---|
| `0x8028F860` | 1,600 | Selector `4`. Solves `\|p4 + t*p5\|^2 = 1`, selects the smaller accepted root, then builds a tangent frame and re-transforms the direction. All 43 `psq_l` / 27 `psq_st` operands decode to `I = 0`, so no quantiser knowledge is missing; what is missing is a verified reading of the root-pair acceptance and the tangent-frame tail (the frame layout is not proved). |
| `0x8028FEA0` | 2,024 | Selector `5`. The same quadratic solved once per plane, keeping the smallest valid `t`, plus the same tail. The export is 1,454 lines of paired-single spill noise (49 `psq_l` / 33 `psq_st`, all `I = 0`), so the raw stream has to be walked instruction by instruction before the per-axis acceptance can be claimed. |

### The three keyframe track samplers (`0x80289410`, `0x8028A6F0`, `0x8028B6C0`)

| address | bytes | why |
|---|---:|---|
| `0x80289410` | 4,792 | Six-channel byte keyframe sampler. The frame-index half matches [`track_frame_index`], but the body then runs a binary search over key-table offsets, an LCG dither seeded from a hash of `(index, descriptor, channel)` and three interpolation modes. The hash/LCG constants are literal; the cubic mode still needs a full pass. |
| `0x8028A6F0` | 4,012 | Same family, another channel width; same open questions. |
| `0x8028B6C0` | 4,280 | Same family, another channel width; same open questions. |

## Verification

- Every address and byte size matches `functions.tsv` (2 rows, 2,480 bytes).
- Every evidence path exists under `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/render/gx_band.rs` (the module now holds 327 translated entries).
- Neither address was in `ledger/functions.csv` before the pass; both were
  confirmed absent.
- `node tools/check-lane-csv.mjs wave113` reports `OK` for
  `wave113-gx-band.csv` (2 rows).
- `cargo check --message-format=short` reports no error and no new warning from
  this pass. The three `unused_assignments` warnings in
  `src/render/gx_band.rs` belong to the wave-107 body `fn_8028cee0` and predate
  it.
- The constants cited above were dumped from `work/dol/main.elf` with
  `r2 = 0x80433080`: `0x8042EBB0` = `100.0`, `0x8042EBB4` = `0.0`,
  `0x8042EBD0` = `-1.0`.
- The spawn path's six-argument signature is the same one `fn_80294440`
  already uses for [`fn_80288e30`], and the three external callers
  (`0x8016D640`, `0x802970D0`, `0x802DB970`) all pass a context object first
  and `*(context+0xB8)` second.
