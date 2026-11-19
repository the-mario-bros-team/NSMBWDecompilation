# Coverage - Wave 108 (`runtime::bt`, the last SDP handler and the render/text tail)

## How to read this file

Every row below is one `pub unsafe fn fn_<address>` body appended to
`src/runtime/bt.rs`. The address is the unrelocated `main.dol` virtual address
(image base `0x80004000`); the byte count was re-read from
`work/ghidra/main-gb/export/functions.tsv` for this pass; the `evidence` column
of `ledger/agents/wave108-bt.csv` names the exact Ghidra C export.

The `0x80220000` bucket holds 216 functions. When this pass started, seven were
untranslated: the five below, plus `0x80228644` and `0x8022FA70`, which are
documented under *Deliberate skips*. All five reconstructible entries landed,
so the bucket now has two open bodies instead of seven.

`// observed:`, `// inferred:` and `// unknown:` markers follow
`docs/RUST_STYLE.md`. Field roles the evidence does not prove are marked rather
than named.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 5 | 4192 | 0 | 4 | 1 |

## What this pass covers

### SDP ServiceSearchAttribute handler (`0x80222E38`, 1052 bytes)

The third server request handler, the sibling of [`fn_80222698`]
(ServiceSearch, PDU id `3`) and [`fn_80222aa8`] (ServiceAttribute, PDU id `5`).
This one answers PDU id `7`:

- the UUID pattern is decoded into a 16-entry table by [`fn_802237f0`], the
  maximum attribute byte count is clamped to `MTU - 10`, and [`fn_80223b40`]
  decodes the attribute-id pairs,
- every matching service record is walked with [`fn_8021fa94`]; each requested
  id is resolved by [`fn_8021fd04`] and encoded by [`fn_80223560`] into the CCB
  receive buffer,
- each record is preceded by a literal `0x36` sequence header. The export
  writes that header at the record's first data byte, so it overlaps the first
  three attribute bytes; the body reproduces the store rather than "fixing" it,
- the outer `0x35` (short) or `0x36` (long) header and the continuation state
  are built exactly as in [`fn_80222aa8`], and the finished PDU is handed to
  [`L2CAP_DATA_WRITE`].

The two `0x42`-byte stack copies around the record loop are the compiler's
register save/restore pair, the same artifact as the `GQR0` blocks; no call
reads either region, so they have no observable effect and are omitted.

### Synchronous IPC submit (`0x80225A80`, 764 bytes)

The synchronous companion of the wave-84 builders. It allocates a `0x40`-byte
kind-7 block, prepares it with [`fn_80225770`], copies it into the static
`0x803D9F00` shadow block and takes the lock at `0x803D9F2C`. Under the OS
critical section it runs the same 16-slot ring guard as [`fn_80224910`], pushes
the heap block, updates the monotonic counters and dequeues one entry when a
waiter is pending. It then unlocks, restores the critical section and returns
the shadow copy's status word at `+4`. Error codes are `-10` (a synchronous
submit is already in flight), `-22` (allocation) and `-8` (ring full).

The global layout this pass confirmed: `r13 - 0x4AC8` is the done-request flag,
`r13 - 0x4AC4` holds the shadow copy pointer and `r13 - 0x4AC0` the original
block pointer. The two addresses are now named
[`SDA_IPC_SYNC_COPY`] / [`SDA_IPC_SYNC_BLOCK`].

### Texture-object setup (`0x8022D720`, 636 bytes)

The palette-layer draw path resumed by wave 102. A three-word cache
(`image`, `object + 0x38`, `object + 0x3C`) plus [`GX_STATE_DIRTY`] guards the
work: when a word changed, the body builds a stack `0x20`-byte GX texture
object with `0x801C6D90`, applies the filter/bias triple with `0x801C6FF0`,
uploads it with `0x801C74C0` and rewrites the cache. It then selects attribute
group `0x80` (`0x801C5570(0x80, 0, 4)`) and writes one packed word to the GX
write-gather FIFO at `0xCC008000`: high half
`(u16(+0x12) + byte(+7)) * 0x8000 / u16(+0xE)`, low half the low `u16` of
`object + 0x10`.

The opening `ldexpf`/`CONCAT` block and the closing four `ldexpf` guards are
the compiler's paired-single save/restore of `f27..f31`; they are not data
flow and are not reproduced.

### Text layout (`0x8022F350`) and its bounding box (`0x8022F960`)

`0x8022F350` is the per-character layout worker behind the text objects that
waves 97/102 already documented (`+0x24` scale, `+0x2C`/`+0x30` cursor,
`+0x43`/`+0x44` wrap policy, `+0x48` glyph provider, `+0x4C` line limit,
`+0x50` wrap step, `+0x60` layout helper). Control codes (`< 0x20`) go through
the layout helper's slot `+0x10`, which fills a four-float rectangle; printable
codes take the provider's slot `+0x48` advance, scale it and widen the box. The
worker replays a character as code `10` when it no longer fits.

`0x8022F960` resets the object cursor to the `SDA2_F32_4C34` sentinel, calls
`0x8022F350` in a loop and folds each returned rectangle into the caller's
four-float box. The export drops the worker's fourth argument at this call
site; the loop bound and the four-argument call in `0x8022FA70` show it is the
remaining byte count, so the body now calls [`fn_8022f350`] directly.

Both exports open with the same dead `GQR0`/`ldexpf` block. `0x8022F350` is the
only `low` row in this pass: its control flow and every offset are literal, but
the virtual slots, the unnamed decoder `0x802DCBFC` and three context words
that the export never writes keep the roles inferred.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80222E38` | `fn_80222e38` | 1052 | medium |
| `0x80225A80` | `fn_80225a80` | 764 | medium |
| `0x8022D720` | `fn_8022d720` | 636 | medium |
| `0x8022F350` | `fn_8022f350` | 1468 | low |
| `0x8022F960` | `fn_8022f960` | 272 | medium |

## Deliberate skips

Two entries of the bucket stay untranslated and have no CSV row.

- `0x80228644` (744 bytes) - the EXI block worker. Its export carries twelve
  `Removing unreachable block` warnings, every loop body was removed, and what
  remains is `do { } while (true)`. Ghidra did not recover the body, so any
  translation would be invented. Wave 92's EXI transfer helpers already call it
  by address.
- `0x8022FA70` (2052 bytes) - the console sink that formats and advances whole
  pages of text. Its cursor is kept in a `CONCAT44` pair (`local_224`), the
  per-glyph state copies are interleaved with the paired-single save blocks, and
  the export drops several arguments at the two virtual calls. It remains
  reachable by address as [`TEXT_OUT_8022FA70`] from [`fn_8022f120`], and
  `0x8022F350` covers the layout work it shares with `0x8022F960`. A dedicated
  pass with the paired-single prologues decoded is the right way to close it.

Both addresses remain reachable by address, so no translated body depends on an
untracked stub.

## Verification

- Every address and byte size in this doc and in the CSV was re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass; the five bodies
  total 4192 bytes.
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/bt.rs`.
- No address was already in `ledger/functions.csv`; all five came from the
  bucket's seven-entry untranslated list. No other Wave 108 lane claims a
  `0x8022xxxx` address.
- `cargo check --message-format=short` produces no error and no new warning in
  `src/runtime/bt.rs`; the only `bt.rs` diagnostics are the ten pre-existing
  `unnecessary parentheses` warnings in the untouched wave-6 L2CAP code.
  Warnings and errors elsewhere in the crate belong to other lanes and do not
  involve this file.
- `node tools/check-lane-csv.mjs wave108` reports the lane clean.
