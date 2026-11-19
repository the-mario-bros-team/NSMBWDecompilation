# Coverage - Wave 45 (audio::seq, RSAR sound-archive access)

## How to read this file

Every row is one `pub unsafe fn` in `src/audio/seq.rs`. The address is the
original DOL virtual address, the byte count comes from
`work/ghidra/main-gb/export/functions.tsv`, and the evidence path is the Ghidra
C export that the body was reconstructed from. `high` means the stores, guards
and calls are literal in that export; `medium` means at least one field role,
packed-return split or callee argument was inferred and is marked as such in the
function's doc block.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/audio/seq.rs` | `audio::seq` | 31 | 2600 | 26 | 5 |

Bytes, High and Medium counts cover only the 31 bodies added in this pass.

## What this pass covers

- Archive playback defaults and field setters, `0x80275040..0x802750E0`: the
  default block writer (`fn_80275040`) and the three f32 setters
  (`fn_802750c0`, `fn_802750d0`, `fn_802750e0`).
- Allocator/reader plumbing, `0x802750F0..0x802751B0`: the vtable/intrusive-list
  header (`fn_802750f0`), the `(w0 + w2 + w5) * 0x28` info-record size
  (`fn_80275150`) and the `0x28`-byte element append (`fn_802751b0`).
- Four-slot `RSAR` node, `0x802753F0..0x802756E0`: constructor
  (`fn_802753f0`), destructor (`fn_802754b0`), the slot read/write pair
  (`fn_80275560`, `fn_80275570`), the small track object's constructor,
  destructor, non-zero query and reset (`fn_80275640`, `fn_80275670`,
  `fn_802756b0`, `fn_802756d0`, `fn_802756e0`).
- Accessor forwarders, `0x80275700..0x802757D0`: the five thin forwarders
  (`fn_80275700`, `fn_80275710`, `fn_80275720`, `fn_80275730`, `fn_80275740`)
  and the seven-word info-header reader (`fn_802757d0`).
- Entry lookup and header decoding, `0x80275A80..0x80275FD0`: the two
  load-by-id paths (`fn_80275a80`, `fn_80275ba0`), the name/separator copy
  (`fn_80275cc0`), the pointer-block clear (`fn_80275d40`), the `RSAR` header
  validation and copy (`fn_80275d70`), the offset-to-pointer conversion
  (`fn_80275e20`), the archive base store (`fn_80275ed0`) and the two
  version-gated entry queries (`fn_80275ee0`, `fn_80275fd0`).

The block is the front end for the same sound resources the rest of the file
plays: `0x80275D70` tests the `RSAR` magic `0x52534152`, and the lookup paths at
`0x80275A80`/`0x80275BA0` hand a resolved archive path to one of two loader
vtable slots.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80275040` | `fn_80275040` | 128 | high |
| `0x802750C0` | `fn_802750c0` | 8 | high |
| `0x802750D0` | `fn_802750d0` | 8 | high |
| `0x802750E0` | `fn_802750e0` | 8 | high |
| `0x802750F0` | `fn_802750f0` | 88 | high |
| `0x80275150` | `fn_80275150` | 84 | high |
| `0x802751B0` | `fn_802751b0` | 52 | high |
| `0x802753F0` | `fn_802753f0` | 192 | high |
| `0x802754B0` | `fn_802754b0` | 172 | high |
| `0x80275560` | `fn_80275560` | 16 | high |
| `0x80275570` | `fn_80275570` | 24 | high |
| `0x80275640` | `fn_80275640` | 36 | high |
| `0x80275670` | `fn_80275670` | 64 | high |
| `0x802756B0` | `fn_802756b0` | 20 | high |
| `0x802756D0` | `fn_802756d0` | 8 | high |
| `0x802756E0` | `fn_802756e0` | 24 | high |
| `0x80275700` | `fn_80275700` | 8 | high |
| `0x80275710` | `fn_80275710` | 8 | high |
| `0x80275720` | `fn_80275720` | 8 | high |
| `0x80275730` | `fn_80275730` | 16 | high |
| `0x80275740` | `fn_80275740` | 16 | high |
| `0x802757D0` | `fn_802757d0` | 8 | medium |
| `0x80275A80` | `fn_80275a80` | 280 | medium |
| `0x80275BA0` | `fn_80275ba0` | 280 | medium |
| `0x80275CC0` | `fn_80275cc0` | 124 | high |
| `0x80275D40` | `fn_80275d40` | 36 | high |
| `0x80275D70` | `fn_80275d70` | 176 | high |
| `0x80275E20` | `fn_80275e20` | 168 | high |
| `0x80275ED0` | `fn_80275ed0` | 12 | high |
| `0x80275EE0` | `fn_80275ee0` | 232 | medium |
| `0x80275FD0` | `fn_80275fd0` | 296 | medium |

## Deliberate skips

- `0x80273560` (1400 B), `0x80273DD0` (1032 B) and `0x80275850` (552 B) sit
  between the two translated runs but are large dispatchers with packed
  singleton returns and mode tables; they need a dedicated pass rather than a
  guess in this lane.
- `0x802741E0`, `0x80274320`, `0x80274480`, `0x80274630`, `0x802746D0` and
  `0x80274A10` are audio-rate code dominated by paired-single / `ldexpf` /
  `unaff_GQR0` arithmetic (for example `0x80274480` contains 27 `unaff_GQR0`
  uses and 22 `ldexpf` calls); they cannot be reconstructed honestly from the
  export and are left for a pass that reads the PowerPC paired-single
  instructions directly.
- `0x80275750..0x80275840` (the remaining member forwarders) is untouched: the
  21-entry thunk run was sampled at `0x80275700..0x80275740`, plus
  `0x802757D0`, which is the only entry this block's size function uses. The
  other thunks are single-target wrappers for a later pass.
- `0x80275EE0`/`0x80275FD0` keep the original's unchecked negative-index path
  out of scope: the `index` arguments are unsigned at the call sites, so the
  bounds test is reproduced exactly as written.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/audio/seq.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` is free of errors for
  `src/audio/seq.rs`.
- `node tools/check-lane-csv.mjs wave45` reports `OK` for this lane.
