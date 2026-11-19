# Coverage - Wave 69 (`runtime::bt`, IOS IPC request layer and EXI serial transfer)

## How to read this file

Every row below is one `pub unsafe fn fn_<address>` body appended to
`src/runtime/bt.rs`. The address is the unrelocated `main.dol` virtual address
(image base `0x80004000`); the byte count comes from
`work/ghidra/main-gb/export/functions.tsv`; the evidence column names the
Ghidra C export. All addresses are inside this lane's `0x80220000` 64 KiB
bucket, which held 216 functions with 140 untranslated when this pass started
(76 already merged in `ledger/functions.csv`).

The lane is **not** Bluetooth protocol code. Like waves 42, 47 and 60 it
translates image-lane filler that happens to land in `0x80220000`; the module
header's *Classification notes* apply. This pass covers the two platform
drivers between the game and the Hollywood IOS co-processor.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 29 | 4000 | 21 | 8 |

## What this pass covers

- **Arena and IPC bootstrap** (`0x802241B4..0x80224890`, 9 bodies). The OS
  arena bounds (`__ArenaLo`/`__ArenaHi` behind `0x801B6610`/`0x801B6600`) are
  cached into five SDA1 words by `fn_802242a0` and re-armed by `fn_802242f0`;
  `fn_80224360`/`fn_80224370`/`fn_80224380` are the getters and setter for the
  live end/frontier pair. `fn_802247d0` creates the first 0x1000-byte heap,
  installs IRQ `0x1B` (`0x80224620`), unmasks `0x10`, writes `0xCD000004 =
  0x38`, clears both request tables and initialises the synchronisation object
  at `0x803D9ED0`; `fn_80224890` is its 0x800-byte sibling without the bring-up.
  `fn_802241b4` and `fn_80224244` are the record matcher and the adjacent-pair
  sorter that the completion path uses. The direction of the arena pair is
  proven by the guard `end < frontier + size` that both heap creators share.
- **IPC request builders** (`0x80224C70..0x80225330`, 8 bodies). One `0x40`-byte
  block per request, allocated from the pool at `r13 - 0x5D14` through
  `0x80225EB0`, tagged with a kind word at `+0`, the caller id at `+8`, a MEM2
  pointer converted to a bus address at `+0x0C`, a length at `+0x10` and up to
  three completion words at `+0x20..+0x28`. Kind `1` is the bounded-string
  request, kinds `2`/`3`/`4`/`5` differ only in the kind word and in whether
  the caller's buffer is cache-flushed (`0x801AC470`, host→device), cache-
  invalidated (`0x801AC440`, device→host) or untouched. All of them submit
  through `0x80224910`.
- **Callback-carrying wrappers** (`0x80227810..0x80227AE0`, 5 bodies). Each
  allocates a `0x140`-byte completion block, fills `+0x100`/`+0x104` with the
  caller's two words and `+0x108` with zero, then calls the matching builder
  with the fixed completion callback `0x802265C0` and the block as context.
  `fn_802278c0`/`fn_80227990` first reject a null or non-32-byte-aligned buffer
  with `-101`. `fn_80227ae0` sends command `0x0D` through the eight-argument
  submitter `0x80225500`.
- **EXI serial-transfer driver** (`0x802282F0..0x802284AC`, 5 bodies). The ring
  index helper, the publication of the transfer object at `r13 - 0x4A80` with
  its controller reset (`0x8022892C`), the interrupt enable/install pair (mask
  bits `0x18000`/`0x40`, handler `0x80228334` on IRQ `0x19`) and the cached
  one-shot identity probe plus block read.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802241B4` | `fn_802241b4` | 144 | high |
| `0x80224244` | `fn_80224244` | 80 | high |
| `0x802242A0` | `fn_802242a0` | 76 | high |
| `0x802242F0` | `fn_802242f0` | 72 | high |
| `0x80224360` | `fn_80224360` | 8 | high |
| `0x80224370` | `fn_80224370` | 8 | high |
| `0x80224380` | `fn_80224380` | 8 | high |
| `0x802247D0` | `fn_802247d0` | 192 | high |
| `0x80224890` | `fn_80224890` | 116 | high |
| `0x80224C70` | `fn_80224c70` | 288 | high |
| `0x80224DA0` | `fn_80224da0` | 192 | high |
| `0x80224E60` | `fn_80224e60` | 168 | high |
| `0x80224F10` | `fn_80224f10` | 256 | high |
| `0x80225010` | `fn_80225010` | 264 | high |
| `0x80225120` | `fn_80225120` | 256 | high |
| `0x80225220` | `fn_80225220` | 264 | high |
| `0x80225330` | `fn_80225330` | 224 | medium |
| `0x80226400` | `fn_80226400` | 16 | high |
| `0x80226410` | `fn_80226410` | 124 | high |
| `0x80227810` | `fn_80227810` | 140 | medium |
| `0x802278C0` | `fn_802278c0` | 164 | medium |
| `0x80227990` | `fn_80227990` | 164 | medium |
| `0x80227A50` | `fn_80227a50` | 132 | high |
| `0x80227AE0` | `fn_80227ae0` | 144 | medium |
| `0x802282F0` | `fn_802282f0` | 32 | high |
| `0x80228358` | `fn_80228358` | 92 | medium |
| `0x802283B4` | `fn_802283b4` | 84 | high |
| `0x80228408` | `fn_80228408` | 164 | medium |
| `0x802284AC` | `fn_802284ac` | 128 | medium |

## Why some of these are `medium`

- `fn_80225330`, `fn_80227810`, `fn_802278c0`, `fn_80227990` and
  `fn_80227ae0` open with `FUN_802DCD74()`/`FUN_802DCD70()`, the CodeWarrior
  register-save prologues. Ghidra folds the incoming `r3`/`r4` into a fake
  64-bit result, so the two real arguments are recovered from that pair (high
  word = `r3`). The same finding and the same recovery are already documented
  for waves 60 and 68 in the module header; the store order in each body makes
  the assignment unambiguous, but the argument *order* is still an inference.
- `fn_80228358` calls `0x8022892C` with no arguments in the export, so which
  object that reset routine consumes is inferred from the sibling bodies.
- `fn_80228408`/`fn_802284AC` reconstruct the two stack buffers as local
  arrays and keep the device addresses `0x34000000`, `0x34000200` and the
  `0xD11000` block base marked `// unknown:`; only the guards, the arithmetic
  and the stores are proven.
- `fn_80227ae0` reproduces an original omission: the `0x140`-byte completion
  block is used without a null check. The export shows the allocation, then
  the resource guard, then the stores in that order, so this is not a
  decompiler artifact, but it is called out in the body.

## Deliberate skips

These neighbours were read and left untranslated. None has a row in
`ledger/agents/wave69-bt.csv`.

- `0x802243C0` (600 bytes), `0x80224620` (428 bytes) - the IPC interrupt
  handler and the MEM2 free path. Both walk freelists with the
  `-0x80000000` bus-address bias inside the loop and call `0x801AC440` with a
  byte count that Ghidra prints as one argument, so the per-node size cannot be
  recovered from the C.
- `0x80224910` (576 bytes) - the submit/wait routine every builder above calls.
  It mixes the 32-slot wait table with a semaphore wait (`0x801B4B20` /
  `0x801B5C90`) and its `param_2 == 0` synchronous path reads a status word
  whose writer is not in this cluster; translating it would have required
  guessing the wait protocol.
- `0x80225A80`, `0x80225D80`, `0x80225EB0`, `0x802260C0`, `0x802262B0`,
  `0x80226370` - the heap-freelist implementation behind the pool. The
  allocator (`0x80225EB0`) is a 508-byte first-fit walk with split nodes
  (`0xBABE0000/1/2` tags) and an alignment term
  `param_3 - 1 & param_3 - ((node + 0x10) & param_3 - 1)`; its `thunk_` export
  at `0x802260B0` is the same body, so both would have duplicated one
  translation. Left for a pass that reads the disassembly.
- `0x80226490` (296 bytes), `0x802265C0` (384 bytes) - the arena setup and the
  shared completion callback. The callback's mode-3 branch reads six bytes at
  fixed offsets from an aligned interior pointer and forwards them to six
  different output words; the mapping is only visible in the disassembly.
- `0x80226740..0x80227730` (17 request helpers) and `0x8022852C` (272 bytes) -
  the IOS command wrappers and the EXI poll loop. Several call `0x80004364`
  (memcpy) with a length that arrives as `extraout_r4`, i.e. an argument the
  decompiler lost.
- `0x8022863C` and `0x80228640` (4 bytes each) - byte-for-byte empty functions
  (`return;`). Nothing to reconstruct beyond the empty body, and they carry no
  evidence of their role.
- `0x80227800`, `0x80227A40`, `0x8022B3C0`, `0x8022B3D0`, `0x8022B3E0`,
  `0x802260B0` - 4-byte branch thunks. Each is a single `b` to an already
  documented callee, so a body here would be a duplicate rather than a
  translation.
- The `0x8022D2F0..0x8022E1C0` float cluster (scissor/offset accessors) is a
  different subsystem (paired-single `GQR0` code) and was not opened this pass.

## Verification

- Every address and byte size matches `functions.tsv` (re-read for this pass;
  total 4000 bytes).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/bt.rs`.
- No address was already in `ledger/functions.csv` (all 29 were absent from the
  bucket's 140 untranslated entries).
- `cargo check --message-format=short` reports no errors and no new warnings in
  `src/runtime/bt.rs`.
- `node tools/check-lane-csv.mjs wave69` reports `wave69-bt.csv: 29 rows` with
  no problem naming that file.
