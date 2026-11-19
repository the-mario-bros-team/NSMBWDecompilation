# Coverage - Wave 160 (`game::bases` dense clusters and quick singles)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 33 | 1,872 | 31 | 2 |

## What this pass covers

- **Rank 11 - `0x80721270`/`0x80721290` (36 B).** The `+0x63C` store plus
  `+0x60`-slot-`0xD4` tail dispatch, and the `+0x638` store.
- **Rank 12 - `0x80781C80`/`0x80781C90` (48 B).** The `0x800F6D60` name-table
  tail call and the `0x80104FD0` effect dispatch through `+0x1CC`.
- **Rank 13 - `0x8066B440`/`0x8066B470` (52 B).** The `+0x594` reset and the
  two-field float copy.
- **Rank 14 - `0x8069ACD0`/`0x8069ACE0` (68 B).** The `+0xDD0` timer arming and
  the `0x800A55C0` reset that zeroes `+0x102`.
- **Rank 15 - `0x80615CC0..0x80615E40` (192 B).** Two table-selected animation
  entries (`0x807C6C0C`/`0x807C6C18`) and the two wrappers that apply them with
  the `1` flag.
- **Rank 16 - `0x806BA630`/`0x806BA660` (84 B).** The `+0xAC` position tail
  call and the frame/ground-test/teardown sequence.
- **Rank 17 - `0x8071F780`/`0x8071F7B0` (84 B).** The `+0x3B4` interface tail
  call and the `0x8071F340` reset.
- **Rank 18 - `0x807571C0`/`0x807571E0` (96 B).** The signed-byte scale through
  `0x800B3980` and its `[-0x1A2C, 0x1A2C]` clamp.
- **Rank 19 - `0x806BC610`/`0x806BC640` (108 B).** The `0xC50` allocation and
  the base-constructor wrapper that installs `0x807F6A00`.
- **Ranks 20 - destructor and flag singles (600 B).** Five `free`-only wrappers,
  five `+0x18` destructor wrappers (two of them byte-identical 100-byte
  variants), and `0x8060D6F0`'s pane-animation selector.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80721270` | `fn_80721270` | 28 | high |
| `0x80721290` | `fn_80721290` | 8 | high |
| `0x80781C80` | `fn_80781c80` | 16 | medium |
| `0x80781C90` | `fn_80781c90` | 32 | high |
| `0x8066B440` | `fn_8066b440` | 40 | high |
| `0x8066B470` | `fn_8066b470` | 12 | high |
| `0x8069ACD0` | `fn_8069acd0` | 16 | high |
| `0x8069ACE0` | `fn_8069ace0` | 52 | high |
| `0x80615CC0` | `fn_80615cc0` | 36 | high |
| `0x80615CF0` | `fn_80615cf0` | 36 | high |
| `0x806BA630` | `fn_806ba630` | 8 | medium |
| `0x806BA660` | `fn_806ba660` | 76 | high |
| `0x8071F780` | `fn_8071f780` | 48 | high |
| `0x8071F7B0` | `fn_8071f7b0` | 36 | high |
| `0x807571C0` | `fn_807571c0` | 32 | high |
| `0x807571E0` | `fn_807571e0` | 64 | high |
| `0x806BC610` | `fn_806bc610` | 48 | high |
| `0x806BC640` | `fn_806bc640` | 60 | high |
| `0x80615E00` | `fn_80615e00` | 60 | high |
| `0x80615E40` | `fn_80615e40` | 60 | high |
| `0x80606AF0` | `fn_80606af0` | 100 | high |
| `0x806090B0` | `fn_806090b0` | 100 | high |
| `0x8060A080` | `fn_8060a080` | 64 | high |
| `0x8060A0C0` | `fn_8060a0c0` | 96 | high |
| `0x8060A120` | `fn_8060a120` | 100 | high |
| `0x8060C1B0` | `fn_8060c1b0` | 64 | high |
| `0x8060C1F0` | `fn_8060c1f0` | 96 | high |
| `0x8060C250` | `fn_8060c250` | 100 | high |
| `0x8060D6F0` | `fn_8060d6f0` | 64 | high |
| `0x80612C90` | `fn_80612c90` | 64 | high |
| `0x80612CD0` | `fn_80612cd0` | 96 | high |
| `0x80615560` | `fn_80615560` | 64 | high |
| `0x806155A0` | `fn_806155a0` | 96 | high |

## Corrections and notes

- `0x80781C80` and `0x806BA630` truncate in the export at `halt_baddata`; both
  raw bodies are complete tail branches, not missing bytes. They are recorded
  at medium confidence because the first argument and the tail callee are
  address-only.
- `0x8060D6F0`'s leading `+0x274` block loads the byte, branches when it is
  non-zero, and otherwise writes `0` to the same byte. It is a shipped no-op and
  is preserved rather than dropped.
- The 100-byte destructor bodies (`0x80606AF0`, `0x806090B0`, `0x8060A120`,
  `0x8060C250`) contain an extra `beq` pair that re-tests `this` before the
  `addic.` guard; functionally they match the 96-byte variants.
- `0x8066B470`'s export prints `(double)`; the raw words are `stfs f1`, so the
  Rust signature takes `f32`.

## Verification

- Every address and byte size matches `work/ghidra/rel-d_basesNP/export/functions.tsv`.
- Every evidence file exists at the cited `work/ghidra/rel-d_basesNP/export/` path.
- Every row has exactly one body in `src/game/bases.rs`.
- No address was already in `ledger/functions.csv` (all 33 were absent).
- `node tools/check-lane-csv.mjs wave160` prints `OK`.
- `cargo check --message-format=short` and `node tools/doc-lint.mjs src` are clean.
