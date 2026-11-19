# Coverage - Wave 90 (audio::seq, sequence-thread support layer)

## How to read this file

Every row is one `pub unsafe fn` added to `src/audio/seq.rs` by this pass. The
address is the original DOL virtual address, the byte count comes from
`work/ghidra/main-gb/export/functions.tsv`, and the evidence path is the Ghidra
C export the body was reconstructed from. `high` means every load, guard, store
and call is literal in that export; `medium` means at least one field role,
callee argument or hidden register value is inferred and marked as such in the
function's doc block.

Two conventions carry over from earlier passes in this file:

- The CodeWarrior register-save stubs `0x802DCD74`/`0x802DCDC0` are printed by
  Ghidra as function calls returning a 64-bit value. That "returned pair" is
  really the caller's own incoming `r3`/`r4`, so [`fn_8027bbf0`] takes its
  object pointer and base from its own parameters. The bare relay `0x802DCDC0`
  only returns whatever is in `r3`; no return value is claimed for the void
  bodies that end with it.
- `0x801B1140`/`0x801B1180` are the small-data state lock pair. Ghidra shows the
  mask returned by the first call being handed straight to the second, and the
  bodies reproduce that (`let token = call0_r(FUN_801B1140)` ... `call1(...,
  token)`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/audio/seq.rs` | `audio::seq` | 28 | 3632 | 20 | 8 |

Bytes, High and Medium counts cover only the 28 bodies added in this pass.

## What this pass covers

- **Manager-list register/unregister pair, `0x8027BA20`/`0x8027BA90`.** These are
  the two-line halves of one operation: link a node into the manager's `+0x384`
  list with the key word at `+0x388`, and unlink it again. Both take the manager
  mutex at `+0x354`, which is the same mutex every wave-80 critical section
  takes, and both are called from the arm routine `0x8027CD20`.
- **Fixed-slot bitmap allocator, `0x8027BBF0..0x8027BDB0` (4 bodies).** The
  constructor records base, total size, per-slot stride, slot count, used count
  and a 32-bit bitmap; `0x8027BC70` forgets the buffer words; `0x8027BCC0`
  scans the bitmap for the first clear bit and returns `base + stride * index`;
  `0x8027BDB0` reverses that arithmetic for a given slot address. This is the
  only cluster in the bucket whose arithmetic closes on itself, which is what
  makes the field roles provable rather than guessed.
- **Entry decoders, `0x8027BE30..0x8027BF50` (3 bodies).** `0x8027BE40` maps the
  source byte `0/1/2` to kind `2/1/3`, turns the second byte into a 0/1 flag and
  copies eleven words; `0x8027BF50` decodes the two layouts of the variable
  table into a `0x28`-byte record. `0x8027BE30` zeroes the two-word node header
  the decoders sit next to.
- **Handle accessors, `0x8027C3E0..0x8027C910` (5 bodies).** The type-byte
  reader, the two presence-gated decode wrappers and the two deleting
  destructors. `0x8027C8B0` and `0x8027C910` are byte-identical exports at
  neighbouring addresses; they are kept as separate bodies because they are
  separate entries in the image.
- **Sub-object arm/release pair, `0x8027CC90`/`0x8027CD20`.** `0x8027CD20` stores
  three caller words, clears four state bytes, sets `+0x101`, unlinks the
  `+0xB4` node through `0x8027BA90`, mirrors the three words to `+0x168..+0x170`
  and inserts `+0x154` as element 1 of the host's `0x0C`-byte table.
  `0x8027CC90` is the inverse and is the only body in this pass that calls a
  virtual slot before taking the manager mutex.
- **Per-frame helpers, `0x8027EEF0..0x8027F730` (7 bodies).** The mode split, the
  mask-selected `f32` store, the eight-slot address helper, the four-word lookup
  wrapper, the rebuild wrapper and the eight-counter advance. Together they
  explain why the sub-object carries a `0x0C`-byte element table, a
  `0x38`-byte slot array and per-slot `+0xE38`/`+0xE3C` counters.
- **Small destructors, `0x8027FD50..0x8027FF60` (5 bodies).** The handle detach,
  the free-when-flagged destructor, the element destructor registration, the
  per-element list destructor and the node insert with its waiter wake.

The module header gains a "Slot allocator and support objects (wave 90)"
section, the range line now ends at `0x8027FF60`, and the body table lists all
28 new entries.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8027BA20` | `fn_8027ba20` | 104 | high |
| `0x8027BA90` | `fn_8027ba90` | 92 | high |
| `0x8027BBF0` | `fn_8027bbf0` | 128 | medium |
| `0x8027BC70` | `fn_8027bc70` | 68 | high |
| `0x8027BCC0` | `fn_8027bcc0` | 232 | high |
| `0x8027BDB0` | `fn_8027bdb0` | 120 | high |
| `0x8027BE30` | `fn_8027be30` | 16 | high |
| `0x8027BE40` | `fn_8027be40` | 268 | medium |
| `0x8027BF50` | `fn_8027bf50` | 704 | medium |
| `0x8027C3E0` | `fn_8027c3e0` | 72 | high |
| `0x8027C430` | `fn_8027c430` | 60 | high |
| `0x8027C470` | `fn_8027c470` | 60 | medium |
| `0x8027C8B0` | `fn_8027c8b0` | 88 | high |
| `0x8027C910` | `fn_8027c910` | 88 | high |
| `0x8027CC90` | `fn_8027cc90` | 144 | medium |
| `0x8027CD20` | `fn_8027cd20` | 216 | high |
| `0x8027EEF0` | `fn_8027eef0` | 152 | medium |
| `0x8027EF90` | `fn_8027ef90` | 128 | high |
| `0x8027F020` | `fn_8027f020` | 32 | high |
| `0x8027F040` | `fn_8027f040` | 80 | high |
| `0x8027F650` | `fn_8027f650` | 124 | medium |
| `0x8027F6D0` | `fn_8027f6d0` | 96 | high |
| `0x8027F730` | `fn_8027f730` | 128 | high |
| `0x8027FD50` | `fn_8027fd50` | 56 | high |
| `0x8027FD90` | `fn_8027fd90` | 64 | medium |
| `0x8027FE70` | `fn_8027fe70` | 100 | high |
| `0x8027FEE0` | `fn_8027fee0` | 88 | high |
| `0x8027FF60` | `fn_8027ff60` | 124 | high |

## Deliberate skips

- **`0x8027B720`, `0x8027B850`, `0x8027B960`.** The sequence thread's start,
  frame callback and thread body. `0x8027B720` builds the thread with
  `0x801B5130` and writes the global block at `0x80412F40`; `0x8027B850`
  re-creates that block and drains a list of virtual calls; `0x8027B960`
  registers `0x8027B850` and loops on `0x801B24C0` until the message is `2`.
  Reconstructing them would require the OS thread API's argument list, which
  the exports do not show, and the one-shot flags come from `unaff_r13` with no
  visible base. Left for a pass that can read the assembly.
- **`0x8027BAF0`** (256 bytes). The per-frame update. It takes the manager mutex
  and walks both manager lists with virtual calls, then calls nine routines
  (`0x80264AE0`, `0x80265100`, `0x80260860`, `0x80260AF0`, `0x80267C40`,
  `0x80267DD0`, `0x80283A30`, `0x80282EF0`, `0x80283300`) whose signatures are
  not visible anywhere in this export. The elapsed-time arithmetic at the end is
  clear, but the nine unknown calls are not, so no body is claimed.
- **`0x8027F450`, `0x8027F7B0`, `0x8027FC50`, `0x8027FFE0`.** `0x8027F450` reads
  a template word from `unaff_r2` and loops with a computed count that Ghidra
  prints as `(int)param_1 + (0xebf - (int)puVar2) >> 4`; `0x8027F7B0` depends on
  GQR state (`unaff_GQR0`, `ldexpf`) and a `0x43300000` double constant in
  `unaff_r2`; `0x8027FC50` walks a list whose sentinel is `r13 - 0x4928`; and
  `0x8027FFE0` is a three-list drain whose `local_28` is folded into an array so
  the position word is ambiguous. Each of these would need a guess about a
  register the export does not expose.
- **`0x802700F0..0x80270310`, `0x80271E30`/`0x802720A0`, `0x80273560`,
  `0x80273DD0..0x80274CE0`, `0x80279510..0x80279CE0`.** The JAS voice/rate group
  loaders and the four-slot paired-single arithmetic paths recorded as skips by
  waves 45/52/66/80. They belong with the `audio::ax` rate/voice work.
- **`0x8027AA00`** and **`0x8027ABB0`.** Still skipped for the reasons recorded in
  `docs/COVERAGE_WAVE80_SEQ.md` (bucket-table insert ambiguity and the
  `(node - 0x100) == 0` fold).
- **`0x802783C0`, `0x80277050`, `0x80270920`, `0x80270AD0`, `0x80270C20`.** Not
  attempted this pass: they sit in the middle of already-covered ranges and the
  time went to the contiguous `0x8027BA20..0x8027FF60` run instead.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/audio/seq.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error or warning for
  `src/audio/seq.rs`. The only build errors in the tree during this pass were in
  other wave-90 lanes' files; an isolated copy of the crate with the failing
  module stubbed out compiles the whole library with zero errors, and no
  diagnostic names `src/audio/seq.rs`.
- `node tools/check-lane-csv.mjs wave90` reports `OK` for `wave90-seq.csv`
  (28 rows, unique addresses, evidence paths present).
