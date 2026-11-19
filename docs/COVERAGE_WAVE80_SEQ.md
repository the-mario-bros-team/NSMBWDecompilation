# Coverage - Wave 80 (audio::seq, record pool, bank-slot entry class and sequence manager)

## How to read this file

Every row is one `pub unsafe fn` added to `src/audio/seq.rs` by this pass. The
address is the original DOL virtual address, the byte count comes from
`work/ghidra/main-gb/export/functions.tsv`, and the evidence path is the Ghidra
C export the body was reconstructed from. `high` means every load, guard, store
and call is literal in that export; `medium` means at least one field role, list
node offset or callee identity is inferred and marked as such in the function's
doc block.

Two conventions carry over from earlier passes in this file:

- Ghidra prints the CodeWarrior register-save stubs `0x802DCD74`/`70`/`6C`/`68`/`60`
  as function calls returning a 64-bit value. That "returned pair" is really the
  caller's own incoming `r3`/`r4`, so the bodies take those values from their own
  parameters. The bare-return relays `0x802DCDB0..C0` are not callees either;
  they only leave their argument in `r3`, so the bodies return that argument
  directly.
- `param_1` in the exports is typed `int *`, so `param_1 + 4` is byte `+0x10`.
  Every offset below is quoted in bytes, and the conversion is noted in the doc
  block where it matters.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/audio/seq.rs` | `audio::seq` | 25 | 2480 | 14 | 11 |

Bytes, High and Medium counts cover only the 25 bodies added in this pass.

## What this pass covers

- **Record pool, `0x8027A160..0x8027A4F0` (11 bodies).** `0x8027A250` builds the
  container: vtable `0x8034F148` at `+0x00`, a mutex header at `+0x04`, a list
  header at `+0x1C`. `0x8027A3A0`/`0x8027A410`/`0x8027A470` are its whole API —
  allocate a record of a caller-given size with two payload words, append one
  zeroed `0x14`-byte record and return its index, and truncate the pool to a
  count — and all three take the `+0x04` mutex; the truncator also takes the
  sequence-manager mutex inside it, which is the only place in this module where
  the two locks nest. `0x8027A2A0`/`0x8027A310`/`0x8027A320` are its destructor
  and its two list-only teardowns. `0x8027A4F0` feeds the same triple into the
  two global registries behind `0x80268C60`.
- **Node attach/detach trio, `0x8027A160`/`0x8027A1B0`/`0x8027A200`.** These are
  the smallest and most reusable bodies in the pass: a holder object stores the
  node it is bound to at `+0x00`, and the node stores the holder back in one of
  two links (`+0x08` or `+0x0C`). The two attach bodies detach whatever holder
  the link already named, and `0x8027A200` is the detach half both of them call
  indirectly. They explain the `0x80267700`/`20`/`40`/`50` accessor family, which
  is nothing more than "is this link set?" and "detach through it".
- **Bank-slot entry class, `0x8027A610..0x8027AEC0` (8 bodies).** The 100-byte
  slot built by `0x8027a560` holds three circular lists (`+0x04`, `+0x0C`,
  `+0x18`), a cursor at `+0x24`, a limit at `+0x28` and two `f32` arrays. This
  pass adds the release family (`0x8027A610`, `0x8027A670`, `0x8027A750`,
  `0x8027A7D0`, `0x8027A850`), which all walk the `+0x10` list and address each
  child as `node - 0xF8`; the two scalar accessors into `+0x44`/`+0x58`; the
  priority re-insert `0x8027A900`; and the cursor/admission trio
  `0x8027AE20`/`0x8027AEB0`/`0x8027AEC0`, which clamp a requested entry count
  against `+0x28` and compare a candidate's clamped priority with the head
  entry's.
- **Sequence manager, `0x8027B510..0x8027B6B0` (3 bodies).** `0x8027B5F0` is the
  singleton every critical section in this module already called: the mutex it
  initialises at `+0x354` is exactly the word `lock_seq_manager` takes, which is
  what identifies the block at `0x80412BF0` as the manager. It clears its fixed
  fields, points two circular list headers at themselves, initialises three
  sub-structures and registers `0x8027B6B0` as its destructor. `0x8027B6B0`
  reverses the two lists; `0x8027B510` is the ready byte in the same small-data
  area.

The module header gains a "Record-pool, bank-slot and manager layout" section so
every offset above can be read without cross-referencing the bodies, and the
range line now ends at `0x8027B6B0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8027A160` | `fn_8027a160` | 76 | high |
| `0x8027A1B0` | `fn_8027a1b0` | 76 | high |
| `0x8027A200` | `fn_8027a200` | 80 | high |
| `0x8027A250` | `fn_8027a250` | 72 | high |
| `0x8027A2A0` | `fn_8027a2a0` | 112 | high |
| `0x8027A310` | `fn_8027a310` | 8 | medium |
| `0x8027A320` | `fn_8027a320` | 8 | high |
| `0x8027A3A0` | `fn_8027a3a0` | 108 | medium |
| `0x8027A410` | `fn_8027a410` | 84 | high |
| `0x8027A470` | `fn_8027a470` | 116 | high |
| `0x8027A4F0` | `fn_8027a4f0` | 104 | medium |
| `0x8027A610` | `fn_8027a610` | 88 | high |
| `0x8027A670` | `fn_8027a670` | 212 | medium |
| `0x8027A750` | `fn_8027a750` | 124 | medium |
| `0x8027A7D0` | `fn_8027a7d0` | 120 | medium |
| `0x8027A850` | `fn_8027a850` | 136 | medium |
| `0x8027A8E0` | `fn_8027a8e0` | 16 | high |
| `0x8027A8F0` | `fn_8027a8f0` | 16 | high |
| `0x8027A900` | `fn_8027a900` | 248 | medium |
| `0x8027AE20` | `fn_8027ae20` | 144 | medium |
| `0x8027AEB0` | `fn_8027aeb0` | 8 | high |
| `0x8027AEC0` | `fn_8027aec0` | 220 | medium |
| `0x8027B510` | `fn_8027b510` | 8 | medium |
| `0x8027B5F0` | `fn_8027b5f0` | 184 | high |
| `0x8027B6B0` | `fn_8027b6b0` | 112 | high |

## Deliberate skips

- **`0x8027AA00`** (388 bytes). It is the other half of `0x8027A750`: it drains a
  slot's list into the 128-bucket table at `0x803FCCB0` and back. Its export
  reads `unaff_r13` for the one-shot bucket-table flag (`r13 - 0x4948`), so the
  flag's address is recovered without trouble, but the same body also walks a
  three-word bucket entry through `0x802290C0`/`0x80229160` with a local used as
  the position word, and Ghidra's fold of that local into an array makes the
  insert position ambiguous. Left for a pass that can read the assembly.
- **`0x8027ABB0`** (468 bytes). The priority-ordered insert that `0x8027A900`
  performs for the second list. Two of its branches test the head node with
  `(node - 0x100) == 0`, a fold of the compiler's null-object test that does not
  correspond to any comparable expression when the list is empty through its
  sentinel; the body also re-locks the manager inside a critical section it never
  released. Reconstructing it would mean guessing which of the two readings the
  compiler produced.
- **`0x8027B520`** (116 bytes) and **`0x8027B510`'s siblings.** `0x8027B520` is a
  wait loop built on `0x801B5FA0` (the OS time base), `0x80260860` and the
  bus-clock divisor at `0x800000F8`. The arithmetic is reconstructible but the
  loop's exit condition mixes an unsigned 32-bit subtraction with a shift of the
  clock word, and one path is a plain `return` from inside a `do/while`; a wrong
  tick conversion would be worse than no body, so it is left out. `0x8027B510`
  itself is translated.
- **`0x80279xxx` group loaders** (`0x80279510`, `0x80279880`, `0x80279A30`,
  `0x80279B70`, `0x80279CE0`) and the **`0x8027A1xx`/`0x8027A9xx` helpers not
  listed above** remain untouched: they are sequence-start paths built on
  paired-single audio-rate arithmetic and on `0x802DCD60`/`70` register-save
  pairs whose argument split is not visible in the exports. They belong with the
  `audio::ax` rate/voice work rather than here.
- **`0x802700F0..0x80270C20`** and the large `0x80271Exx..0x80274CEx` group are
  still the skips recorded by waves 45/52/66 (four-slot JAS voice band, paired
  singles, `unaff_r13` sentinels).

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path (checked by
  `node tools/check-lane-csv.mjs wave80`).
- Every row has exactly one body in `src/audio/seq.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error or warning for
  `src/audio/seq.rs`; the only build error in the tree during this pass was in
  another wave-80 lane's file.
- `node tools/check-lane-csv.mjs wave80` reports `OK`.
