# Coverage - Wave 125 (GX / render band, `0x802A0000` bucket)

## How to read this file

This page covers the Wave 125 pass over the last open entry of the
`0x802A0000..0x802AFFFF` bucket of the GX/render lane (`src/render/gx_band.rs`).
Addresses are virtual addresses of the unloaded `main.dol` (image base
`0x80004000`); byte counts come from `work/ghidra/main-gb/export/functions.tsv`,
never from prose. The machine-readable mirror is
`ledger/agents/wave125-gx-band.csv` (one row, `partial`, `medium`).

The bucket held exactly one untranslated entry when this pass started:

| address | bytes | state after this pass |
|---|---:|---|
| `0x802A71D0` | 2,748 | **landed (partial)** - per-strip quadratic emitter |

The `0x802A0000` bucket is now 102 of 102 (101 full, one partial:
`0x802A71D0`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/gx_band.rs` | `render::gx_band` | 1 | 2,748 | 0 | 1 |

## What this pass covers

- **The guard and the two reciprocals.** `cmpi cr0,0,r4,1` + `blt` at
  `0x802A74D4` returns at once when `count < 1`. The prologue converts `r4`
  (outer count) and `r5` (per-strip sample count) through the `0x43300000`
  double bias at `0x8042F040` and keeps `fres` of both
  (`0x802A7280..0x802A729C`), i.e. `1/count` and `1/samples`.
- **The control records.** `r6`, `r7` and `r8` each point at a 0x28-byte
  record: three floats at `+0x00`, `+0x0C`, `+0x18` plus one float at `+0x24`.
  Every record read is a `psq_l` pair plus a `W = 1` broadcast for `z`, and all
  six sites carry `I = 0` (GQR0 = plain float), so the export's `ldexpf`
  ladders are dead.
- **The quadratic weights.** For each outer index `i` the body forms
  `t = i / count` and, literally at `0x802A7588..0x802A75B4`,
  `w2 = 0.5*t*t`, `w1 = 0.5 + t - t*t`, `w0 = 0.5*(t - 1)*(t - 1)`
  (built from the `sdata2` 0.5 at `0x8042F038` and `-1.0` at `0x8042F03C`,
  `r2 = 0x80433080`).
- **The strip begin.** `li r3,152` (`0x98` = `GX_TRIANGLESTRIP`), `li r4,0`,
  `r5 = (2*samples + 2) & 0xFFFF` and `bl 0x801C5570` at `0x802A78F4`. The
  count matches the `2*(n+1)` vertices the two emit blocks produce.
- **The two emit arms.** Bit 5 of the flags word selects the arm
  (`0x802A78F8..0x802A7900`), and bit 0 (`rlwinm r15,r9,0,31,31`) decides
  whether each vertex also writes the texture pair. A negative sample count
  skips the inner loop (`blt` at `0x802A7910` / `0x802A7A9C`); otherwise
  `samples + 1` pairs are emitted.
- **The texture pair.** `(sample / samples, samples)` - the first from the
  `xoris` conversion of the loop index times `f27`, the second from the
  `stfs f3,724(r1)` that stores `(float)samples`.
- **The FIFO traffic.** Five `stfs` sites per arm (`0x802A79A0`, `0x802A79A8`,
  `0x802A79B0`, plus the optional `0x802A79D0`/`0x802A79D4`) around
  `stfs f, -32768(r31)` with `lis r31,0xCC01`, i.e. the GX FIFO at
  `0xCC008000`.

## The export defects this pass resolves

- **The pseudo-return is `__savegprlr`.** `bl 0x802DCD40` at `0x802A7220` only
  stores the non-volatile GPRs (and `0x802DCD8C` restores them), so the
  export's leading `FUN_802dcd40()` result is an artefact and `r3..r10` survive
  as the arguments.
- **Every `psq` operand is GQR0.** All 197 `psq_l`/`psq_st` sites in
  `0x802A71D0..0x802A7C88` decode with `I = 0`, so the export's `unaff_GQR0`
  conditionals, its 144 removed blocks and its `ldexpf` calls are dead helper
  ladders. The body was transcribed from `work/dol/main.elf`.
- **The constant window.** With `r2 = 0x80433080` (set by
  `lis r2,0x8043` / `ori r2,0x3080` at `0x8000423C`):
  `0x8042F028 = 0.0`, `0x8042F038 = 0.5`, `0x8042F03C = -1.0`,
  `0x8042F040 = 0x4330000080000000` (the unsigned-int double bias).
- **The `bc` forms.** `bc 12,0` at `0x802A74D4`/`0x802A7910` is `blt`,
  `bc 12,2` at `0x802A7900`/`0x802A79B4` is `beq`, `bc 16,0` at
  `0x802A7A88` is `bdnz`, and `bc 4,1` at `0x802A7C30` is `bng` (loop while
  `index <= count`).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802A71D0` | `fn_802a71d0` | 2748 | medium (partial) |

## Deliberate skips

### `0x802A71D0` - the two vertical edge offsets

The inner arms at `0x802A7928..0x802A7A88` build each vertex from the blended
record's three vectors as `P0 + a*P1 + b*P2` (arm order swapped for the second
vertex), where `a` and `b` are lane 0 of the paired-single registers `ps5` and
`ps6`. Those two registers are produced by the prologue's
`ps_add ps5,ps27,ps27` / `ps6 = ps12 + ps8` chain, which itself reads `ps1`,
`ps3`, `ps9`, `ps10`, `ps11`, `ps27` and `ps28` - seven paired-single
registers the function **never writes**. The nine call sites
(`0x802A8C70`, `0x802A8CC8`, `0x802A8FC0`, `0x802A9038`, `0x802A907C`,
`0x802A93B4`, `0x802A9474`, `0x802A95CC`, `0x802A9600`) set only `r3..r10`;
none of them writes a `ps` register either, and their own bodies
(e.g. `fn_802a8a10` at `0x802A8B2C..0x802A8B34`) read `ps28` the same way. The
functions are therefore a hand-written family with a shared paired-single
register state, and the two edge weights cannot be pinned from this entry
alone.

The body keeps the whole structure - guard, both loop bounds, the two
reciprocals, the quadratic weights, the `2*(n+1)` vertex count, the texture
pair and every FIFO store - and collapses the two edge points onto the blended
record's first vector. The row is `partial` for exactly that reason.

All other blocks (prologue, weight construction, `GXBegin`, both emit arms and
both texture-pair blocks) are transcribed from `work/dol/main.elf`.

## Verification

- Every address and byte size matches `functions.tsv` (1 row, 2,748 bytes).
- The evidence file exists at the cited path:
  `work/ghidra/main-gb/export/802a71d0_FUN_802a71d0.c`.
- The row has exactly one `pub unsafe fn fn_802a71d0` body in
  `src/render/gx_band.rs`; the module now holds 345 reconstructed entries
  (342 full, three partial).
- `0x802A71D0` was not in `ledger/functions.csv` before the pass; the whole
  `0x802A0000` bucket was re-read (102 entries, this one open).
- `node tools/check-lane-csv.mjs wave125` reports `OK` for this lane's CSV.
- `cargo check --message-format=short` reports no error from
  `src/render/gx_band.rs`; the three `unused_assignments` warnings in
  `fn_80284f30`/neighbours predate this pass and no new warning comes from
  `fn_802a71d0`.
