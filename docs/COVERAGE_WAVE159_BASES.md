# Coverage - Wave 159 (`game::bases` small-body clusters)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 39 | 2,440 | 36 | 3 |

## What this pass covers

- **Rank 1 - `0x807C39D0..0x807C3B60` (12 bodies, 392 B).** A two-pair state
  flag machine over `+0x4EC..+0x540`: the guarded open routines, the unguarded
  reset, the `+0x500` getter, the `+0x506`/`+0x50E` pair, the request byte and
  the `0x80354E60` slot lookup. No paired-single and no indirect dispatch.
- **Rank 2 - `0x807AC260..0x807AC280` (3 bodies, 48 B).** Three byte-identical
  vtable tail calls to slot `+0x08`.
- **Rank 3 - `0x806A6A70..0x806A6AA0` (3 bodies, 156 B).** The `0x136` factory
  request, the two-record velocity scale and the interface store. The first two
  exports are truncated by `halt_baddata`; the raw words are complete tail
  branches, not missing bytes.
- **Ranks 4-8 - five destructor families (15 bodies, 1,120 B).** Ten
  `80606A10`-style wrappers (free when `count > 0`) and five `80606A90`-style
  wrappers that also run the `+0x18` destructor. Each family was decoded from
  its own raw words; the bodies differ only in address.
- **Rank 9 - `0x8071FB10..0x8071FB70` (3 bodies, 292 B).** The `+0x6B8`
  interface tail call, the `+0x534` reset, and the transform rebuild from the
  `+0xAC` position plus three signed `i16` angles.
- **Rank 10 - `0x80699F80..0x80699FE0` (3 bodies, 432 B).** Two height-band
  predicates and the timer/roll state chooser.

The `docs/SPEC_WAVE158_BASES.md` 516-540 byte eight were deliberately not
started; the ranked clusters are cheaper to verify and keep the wave countable.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x807C39D0` | `fn_807c39d0` | 52 | high |
| `0x807C3A10` | `fn_807c3a10` | 44 | high |
| `0x807C3A40` | `fn_807c3a40` | 44 | high |
| `0x807C3A70` | `fn_807c3a70` | 24 | high |
| `0x807C3A90` | `fn_807c3a90` | 44 | high |
| `0x807C3AC0` | `fn_807c3ac0` | 44 | high |
| `0x807C3AF0` | `fn_807c3af0` | 12 | high |
| `0x807C3B00` | `fn_807c3b00` | 8 | high |
| `0x807C3B10` | `fn_807c3b10` | 32 | high |
| `0x807C3B30` | `fn_807c3b30` | 16 | high |
| `0x807C3B40` | `fn_807c3b40` | 20 | high |
| `0x807C3B60` | `fn_807c3b60` | 52 | high |
| `0x807AC260` | `fn_807ac260` | 16 | high |
| `0x807AC270` | `fn_807ac270` | 16 | high |
| `0x807AC280` | `fn_807ac280` | 16 | high |
| `0x806A6A70` | `fn_806a6a70` | 12 | high |
| `0x806A6A80` | `fn_806a6a80` | 32 | high |
| `0x806A6AA0` | `fn_806a6aa0` | 112 | medium |
| `0x80606A10` | `fn_80606a10` | 64 | high |
| `0x80606A50` | `fn_80606a50` | 64 | high |
| `0x80606A90` | `fn_80606a90` | 96 | high |
| `0x80608FD0` | `fn_80608fd0` | 64 | high |
| `0x80609010` | `fn_80609010` | 64 | high |
| `0x80609050` | `fn_80609050` | 96 | high |
| `0x8061D790` | `fn_8061d790` | 64 | high |
| `0x8061D7D0` | `fn_8061d7d0` | 64 | high |
| `0x8061D810` | `fn_8061d810` | 96 | high |
| `0x80668F60` | `fn_80668f60` | 64 | high |
| `0x80668FA0` | `fn_80668fa0` | 64 | high |
| `0x80668FE0` | `fn_80668fe0` | 96 | high |
| `0x80757D30` | `fn_80757d30` | 64 | high |
| `0x80757D70` | `fn_80757d70` | 64 | high |
| `0x80757DB0` | `fn_80757db0` | 96 | high |
| `0x8071FB10` | `fn_8071fb10` | 48 | high |
| `0x8071FB40` | `fn_8071fb40` | 40 | high |
| `0x8071FB70` | `fn_8071fb70` | 204 | medium |
| `0x80699F80` | `fn_80699f80` | 44 | high |
| `0x80699FB0` | `fn_80699fb0` | 44 | high |
| `0x80699FE0` | `fn_80699fe0` | 344 | medium |

## Corrections to the Wave 158 classification

- `0x8071FB10` and the `0x807AC26x` group are pure vtable **tail** calls. The
  Ghidra exports either print `return 1` or warn about an unrecovered jump
  table; neither is a literal return or a table. The raw `bctr` was trusted.
- `0x806A6A70` and `0x806A6A80` end in a plain tail `b`; the export's
  `halt_baddata` is a Ghidra artifact, not a truncated body.
- `0x80699FE0`'s float-to-int conversion uses the `0x43300000` magic pair. It is
  reproduced as a truncating cast clamped to `0x807CA0A8`, not as a literal
  double arithmetic sequence.

## Deliberately skipped

- `0x80699FE0` was the only body in this pass with no Rust-visible type for its
  `+0x60` component chain; it is written at medium confidence rather than
  promoted to high.
- The `docs/SPEC_WAVE158_BASES.md` eight (516-540 B) remain queued.
- Class P (34 rows) and class S clones (97 rows) were not attempted: the paid
  model's weekly limit stopped the planned read-only lanes, and neither class
  can be claimed honestly from the Wave 158 report alone.

## Verification

- Every address and byte size matches `work/ghidra/rel-d_basesNP/export/functions.tsv`.
- Every evidence file exists at the cited `work/ghidra/rel-d_basesNP/export/` path.
- Every row has exactly one body in `src/game/bases.rs`.
- No address was already in `ledger/functions.csv` (all 39 were absent).
- `node tools/check-lane-csv.mjs wave159` prints `OK`.
- `cargo check --message-format=short` is free of errors.
