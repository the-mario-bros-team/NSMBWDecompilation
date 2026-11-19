# Coverage - Wave 67 (runtime::dvd, draw-token and GX state-register helpers in the 0x801C0000 bucket)

## How to read this file

The `0x801C0000` bucket is a whole 64 KiB page of `main.dol`. Wave 67 continues
the practice of Waves 40/46/56: the page is wider than the DVD band (which
starts at `0x801CA640`), so this pass takes small, self-contained GX and
draw-token routines that live *before* the DVD band and writes them into
`src/runtime/dvd.rs`, the only writable file assigned to the bucket. Every doc
block says plainly that the routine is GX/interrupt code, not disc code.

Every body reads the GX state base from the SDA2 word at `r2 - 0x4EF8` (the
pointer documented in `src/render/gx.rs`) and, where it emits, writes one word
to the GX write-gather FIFO at `0xCC008000`. Field roles the export does not
prove stay `inferred:` in the doc blocks; every shift, mask and store is
literal.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/dvd.rs` | `runtime::dvd` | 27 | 2328 | 19 | 8 |

## What this pass covers

- Draw-token bookkeeping run `0x801C33E0`-`0x801C37B0` (9 bodies, 520 bytes).
  The two byte flags at `r13-0x4DC7`/`-0x4DC8` gate a nine-word token block at
  `0x80390224`; `fn_801c3440` refreshes and copies it, `fn_801c33f0` mirrors the
  first half-word of the `r13-0x4DEC` block into `state + 0xC` and hands back
  five status bits, `fn_801c3540` is the bring-up that installs IRQ handler
  `0x11` (`0x801C2B80`), and `fn_801c34f0`/`fn_801c3760` are the two
  interrupt-guarded swaps (completion callback, saved tick) with
  `fn_801c37b0` as the unguarded tick getter and `fn_801c34d0`/`fn_801c34e0`
  as the two trivial field getters.
- FIFO/flush and retrace-callback run `0x801C47C0`-`0x801C5280` (7 bodies,
  400 bytes). `fn_801c47c0` emits the lone `0x48` byte; `fn_801c4a20` and
  `fn_801c6ac0` are the paired `+0x254` nibble/three-bit writers;
  `fn_801c4ad0`+`fn_801c4b30` are the push-zero-drain and
  poll-then-reprogram-FIFO pair; `fn_801c4f70` is the `+0x22C` emitter;
  `fn_801c51b0` swaps the retrace callback and `fn_801c5280` installs the two
  handlers that call it.
- Texture/indirect state run `0x801C59D0`-`0x801C8DC0` (11 bodies, 1408 bytes).
  The core is the pure block-size table `fn_801c6cd0` (three shift pairs plus
  the two-tile formats `6`/`0x16`), which `fn_801c59d0` calls after programming
  `state + 0x248`/`+0x24C`/`+0x250`. Around it sit `fn_801c6900` (the six-way
  `+0xA8`/`+0xAC` writer), `fn_801c6af0` (TEV stage word with the stage 4/5
  aliases at `+0xC0`/`+0xC4`), `fn_801c7520` (three-word `0x64` header),
  `fn_801c76c0` (fixed `0x66001100` between two `fn_801c8220` pushes),
  `fn_801c7750`/`fn_801c77d0` (the `+0x5E4`-gated indirect stage size/wrap
  writers), `fn_801c5ec0` (the `0x51` offset command) and `fn_801c8dc0` (the
  packed indirect command word cached at `+0x220`).
- All three runs are contiguous address ranges, chosen so adjacent bodies share
  field offsets and confirm each other (`fn_801c4a20` with `fn_801c6ac0`,
  `fn_801c7750` with `fn_801c77d0`, `fn_801c6cd0` with `fn_801c59d0`).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801C33E0` | `fn_801c33e0` | 8 | high |
| `0x801C33F0` | `fn_801c33f0` | 76 | medium |
| `0x801C3440` | `fn_801c3440` | 144 | high |
| `0x801C34D0` | `fn_801c34d0` | 8 | high |
| `0x801C34E0` | `fn_801c34e0` | 8 | high |
| `0x801C34F0` | `fn_801c34f0` | 68 | high |
| `0x801C3540` | `fn_801c3540` | 124 | medium |
| `0x801C3760` | `fn_801c3760` | 76 | high |
| `0x801C37B0` | `fn_801c37b0` | 8 | high |
| `0x801C47C0` | `fn_801c47c0` | 16 | high |
| `0x801C4A20` | `fn_801c4a20` | 36 | high |
| `0x801C4AD0` | `fn_801c4ad0` | 92 | medium |
| `0x801C4B30` | `fn_801c4b30` | 52 | medium |
| `0x801C4F70` | `fn_801c4f70` | 36 | high |
| `0x801C51B0` | `fn_801c51b0` | 68 | high |
| `0x801C5280` | `fn_801c5280` | 100 | medium |
| `0x801C59D0` | `fn_801c59d0` | 268 | medium |
| `0x801C5EC0` | `fn_801c5ec0` | 120 | high |
| `0x801C6900` | `fn_801c6900` | 212 | high |
| `0x801C6AC0` | `fn_801c6ac0` | 36 | high |
| `0x801C6AF0` | `fn_801c6af0` | 164 | medium |
| `0x801C6CD0` | `fn_801c6cd0` | 192 | high |
| `0x801C7520` | `fn_801c7520` | 40 | high |
| `0x801C76C0` | `fn_801c76c0` | 72 | high |
| `0x801C7750` | `fn_801c7750` | 124 | high |
| `0x801C77D0` | `fn_801c77d0` | 100 | high |
| `0x801C8DC0` | `fn_801c8dc0` | 80 | medium |

## Deliberate skips

- `0x801C13F0`, `0x801C1420`, `0x801C1450`, `0x801C1470` (the GQR-format
  vector add/sub/scale/normalise helpers) and every other `unaff_GQR0` routine
  in the page: the same reason Wave 56 recorded. The exports branch on a GQR
  register the decompiler could not assign and on a per-format
  `4/5/6/7` byte/half/float path, so reconstructing them would mean inventing a
  quantisation argument the callers never pass. Left untouched.
- `0x801C2B80` (352 bytes) and `0x801C32E0` (256 bytes): both are referenced by
  name in this pass but not translated. They are the producer/consumer of the
  nine-word token block, use a 720-byte dispatch stack frame
  (`0x801ACF80`/`0x801ACD40`) and two indirect calls, so they need a pass that
  also takes the dispatch helpers. They are called by absolute address.
- `0x801C5120`, `0x801C5200`, `0x801C52F0`, `0x801C7350`: the two retrace
  handlers and the push/wait pair. They are installed by `fn_801c5280` and
  called by `fn_801c4ad0`, but each dereferences a function pointer taken from
  the state block under a lock, so their own bodies were left for the pass that
  also establishes that pointer's owner. Called by address.
- `0x801C6880`, `0x801C6FF0`, `0x801C9470`, `0x801C9530`: the large GQR-backed
  matrix emitter, the texture-coordinate clipper and the state save/restore pair
  (`0x600` bytes through `0x80004364`, plus the untranslated scheduler hooks
  `0x802DCD70`/`0x802DCD74`/`0x802DCDBC`/`0x802DCDC0`). Not reconstructed.
- `0x801C0020` (1196 bytes) and `0x801C9E20` (2076 bytes): still the two largest
  state machines in the page and still need a dedicated frame-by-frame read.
- `0x801C5B30`: needs the float-to-integer conversion helper `0x802DCC4C`; the
  rounding mode is not proven from the export, so the body was skipped rather
  than guessed.
- No address already present in `ledger/functions.csv` was re-claimed. The 145
  previously translated addresses in this bucket are untouched; 120 untranslated
  addresses remain in the page.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (27 rows, 2328 bytes total).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/dvd.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no errors or warnings in
  `src/runtime/dvd.rs` (the only outstanding error in the workspace is in
  another lane's `src/game/bases.rs`).
- `node tools/check-lane-csv.mjs wave67` prints `OK` for
  `ledger/agents/wave67-dvd.csv` (27 rows, 27 unique addresses).
