# Coverage - Wave 59 (`game::stage`, stage resource-set object)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the function
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The evidence
column points at the Ghidra C export that the body was written from.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 29 | 3272 | 16 | 13 |

No body in this pass is `low` confidence and none is `partial`.

## What this pass covers

This pass fills the untranslated run `0x800C8860..0x800C9930` plus the two
duplicated `0x802DCBFC` forwarding wrappers at `0x800C7550`/`0x800C7580`.
`0x800C8950..0x800C9930` is a single class, the **stage resource set**:

- **Object lifetime** (`0x800C8950`, `0x800C89A0`) - the constructor runs the
  shared geometry base constructor (`0x80006CD0`), installs `0x80317438` in the
  embedded base at `+0x08`, constructs the archive accessor at `+0xAC`
  (`0x80006AA0`) and clears the two table counts. The destructor releases the
  accessor with delete count `-1`, destroys the base with `0` and frees the
  storage only for a positive caller count.
- **Path building and loading** (`0x800C8BB0`, `0x800C8CB0`) - the default path
  is the small-data directory prefix at `r13-0x7A88` plus the leaf name; the
  alternate path starts from the 100-byte template at `0x802F1014` and is
  finished by `0x800B4650`. The resulting string goes to `0x80006B60`, and a
  successful load stores the accessor pointer at `+0x84`.
- **Recursive child reset** (`0x800C8AA0`) - walks the `+0x0C` child list of a
  node looking for the `r13-0x48A8` anchor, then resets every member through
  vtable slots `+0x74` (`0x1FF`) and `+0x7C` (`+0xD8` word) and recurses over
  the `+0x14` list.
- **Id-list resolver family** (`0x800C8D60`, `0x800C8DD0`, `0x800C8E40`,
  `0x800C8EB0`) - the same `count`-iteration loop mapping ids to nodes through
  `0x80007300` and its three collection-filtered variants
  (`0x80007470` / `0x800073D0` / `0x80007320`). They share the private helper
  `w59_resolve_ids`.
- **Effect dispatch** (`0x800C8F20`) - resolves each id against the
  `r13-0x48A8` collection and calls the untranslated `0x800C9A60` with the
  current `DAT_80359CB4` kind word.
- **Table construction** (`0x800C8FB0`, `0x800C9060`, `0x800C90F0`) - the
  `0x1C`-stride channel table at `+0x180` (count `+0x18C`, constructor
  `0x800C9060` with vtable `0x80317450`, destructor `0x800C90B0`) and the
  `0x28`-stride descriptor table at `+0x184` with its byte-flag array at
  `+0x188` (count `+0x190`, destructor `0x800C9230`). Allocation goes through
  `0x802B9250` + `0x802DC7E0` and each entry is bound with `0x80163E60` or
  `0x801643B0`.
- **Per-entry controls** (`0x800C9270`, `0x800C92F0`, `0x800C9380`,
  `0x800C93D0`, `0x800C9490`, `0x800C9500`, `0x800C9560`, `0x800C9610`,
  `0x800C9640`) - start an entry with one of the `r2-0x6788`/`r2-0x6784`
  constants, optionally re-align it to the upper edge with the `r2-0x677C`
  margin, step live channels through `0x801637F0`/`0x801636C0`, and clear or
  query the per-entry flag byte. The flag array and the `+0x194` cursor belong
  to this group.
- **Finish** (`0x800C9930`) - finishes every `0x1C` record with `0x80163FB0`,
  deletes both arrays with their real destructor slots, frees the flag array
  and closes the accessor with `0x80006BF0`.
- **Dispatcher wrappers** (`0x800C7550`, `0x800C7580`) - the same duplicated
  `0x802DCBFC(param_2, param_2, param_1)` forwarding pair that already exists
  at `0x800B0750`/`0x800B0780`; kept separate because they are separate
  functions in the image.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800C7550` | `fn_800c7550` | 48 | high |
| `0x800C7580` | `fn_800c7580` | 48 | high |
| `0x800C8860` | `fn_800c8860` | 88 | high |
| `0x800C8950` | `fn_800c8950` | 80 | high |
| `0x800C89A0` | `fn_800c89a0` | 104 | high |
| `0x800C8AA0` | `fn_800c8aa0` | 264 | medium |
| `0x800C8BB0` | `fn_800c8bb0` | 252 | medium |
| `0x800C8CB0` | `fn_800c8cb0` | 12 | high |
| `0x800C8D60` | `fn_800c8d60` | 104 | medium |
| `0x800C8DD0` | `fn_800c8dd0` | 104 | medium |
| `0x800C8E40` | `fn_800c8e40` | 104 | medium |
| `0x800C8EB0` | `fn_800c8eb0` | 104 | medium |
| `0x800C8F20` | `fn_800c8f20` | 136 | medium |
| `0x800C8FB0` | `fn_800c8fb0` | 168 | high |
| `0x800C9060` | `fn_800c9060` | 76 | high |
| `0x800C90B0` | `fn_800c90b0` | 64 | high |
| `0x800C90F0` | `fn_800c90f0` | 188 | medium |
| `0x800C91F0` | `fn_800c91f0` | 64 | high |
| `0x800C9230` | `fn_800c9230` | 64 | high |
| `0x800C9270` | `fn_800c9270` | 120 | medium |
| `0x800C92F0` | `fn_800c92f0` | 136 | medium |
| `0x800C9380` | `fn_800c9380` | 76 | high |
| `0x800C93D0` | `fn_800c93d0` | 184 | medium |
| `0x800C9490` | `fn_800c9490` | 100 | high |
| `0x800C9500` | `fn_800c9500` | 88 | high |
| `0x800C9560` | `fn_800c9560` | 172 | medium |
| `0x800C9610` | `fn_800c9610` | 36 | high |
| `0x800C9640` | `fn_800c9640` | 64 | high |
| `0x800C9930` | `fn_800c9930` | 224 | medium |

## Deliberately skipped

- `0x800C8A90` (4 bytes) - a bare thunk to `0x80007320`. It is below the
  module's 16-byte floor and only a tail jump; the target is called by address
  from [`fn_800c8eb0`].
- `0x800C9680` (688 bytes) - the paired-single state export: twenty
  `ldexpf`/`unaff_GQR0` blocks and `CONCAT44` reconstruction noise hide the
  real matrix terms. A body would be guesswork, so it is left.
- `0x800C9A60` (160 bytes) and `0x800C9B50` (660 bytes) - both are consumed by
  the effect system and their exports split every argument into
  `undefined8`/CR-bit pairs (`in_cr1`, stacked `param_9..param_16`) that do not
  map back to a trustworthy signature. `0x800C9A60` is called by address from
  [`fn_800c8f20`].
- `0x800C4F50`, `0x800C51A0`, `0x800C6E20..0x800C71A0` - the paired-single
  angle steppers and their small wrappers. The wrappers lose the `f1` distance
  argument entirely in the decompile, so no honest call can be written.
- `0x800C75B0` (4772 bytes) - a single huge function in this band, outside the
  cluster; it needs its own field map.
- `0x800C2030..0x800C2690`, `0x800C3A10`, `0x800C3BA0` and the rest of the
  bucket - outside the run this pass owns. `0x800C3A10` is the paired-single
  point mover whose wrappers are skipped above.

## Verification

- Every address and byte size matches `functions.tsv` (29 rows, 3272 bytes).
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/stage.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` is free of errors for
  `src/game/stage.rs` (only pre-existing warnings in other modules).
- `node tools/check-lane-csv.mjs wave59` reports `OK`.
