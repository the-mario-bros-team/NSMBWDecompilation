# Coverage - Wave 91 (runtime::dvd, the ISFS cores and the SYSCONF settings mount in the `0x801D0000` page)

## How to read this file

The `runtime::dvd` bucket is the whole `0x801D0000` 64 KiB page of `main.dol`.
Only the middle of that page is disc code (`0x801CA640`-`0x801D6780`); the rest
is SDK code that happens to share the page. `src/runtime/dvd.rs` is the only
writable file assigned to the bucket, so those routines are reconstructed there
too, and each function doc says plainly which routine it is and which fields it
touches.

This pass adds no disc code. It closes the lower ISFS wrapper band
(`0x801D8930`-`0x801DBD70`) that Wave 87 deferred, and it opens the three
decoder/NAND constructors at `0x801D6850`-`0x801D6F20`.

**Big-endian register pairs.** Several routines in this band are entered through
the Metrowerks callee register-save helper `0x802DCD74`, which Ghidra models as
returning a 64-bit value. On this big-endian target the high word of that value
is `r3` (the first argument) and the low word is `r4` (the second). The bodies
therefore take those words as ordinary parameters, never call the helper, and
the doc blocks say so. This is the same treatment Wave 72 gave the settings
core.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/dvd.rs` | `runtime::dvd` | 30 | 4488 | 4 | 26 |

## What this pass covers

- Decoder/NAND constructors, `0x801D6850`-`0x801D6F20` (3 bodies, 152 bytes).
  `fn_801d6850` and `fn_801d6880` initialise the byte-stream decoder state used
  by `0x801D68C0` (`+0x04` shift register, `+0x0C` repeat count, `+0x0E`
  control byte, `+0x0F` bit counter). `fn_801d6f20` initialises the NAND
  error-log context with the self-pointer at `+0x88C` and the `+0x8A6`/`+0x8A7`
  state pair.
- Thin flash wrappers, `0x801D8930`-`0x801D8BD0` (4 bodies, 552 bytes). These
  build a full path with [`fn_801da6e0`] and forward to `0x80226F70`,
  `0x802278A0`, `0x80227970` and the `0x80225410` mode thunk.
- File/attribute cores and completions, `0x801D9040`-`0x801D9F00` (6 bodies,
  852 bytes). The descriptor stat reader, the mode-tagged core
  [`fn_801d94a0`] with its synchronous/asynchronous entry points
  [`fn_801d96d0`]/[`fn_801d9750`], and the completion handlers
  [`fn_801d9320`]/[`fn_801d9f00`].
- ISFS mount and SYSCONF settings read, `0x801DA860`-`0x801DBD70` (16 bodies,
  2920 bytes), plus the lock release [`fn_801dcc60`] (12 bytes). [`fn_801da860`] is the mount itself (state `0` -> `1` -> `2`,
  home directory, hook table). Around it sit the open wrappers
  [`fn_801dae70`]/[`fn_801daec0`] and their completion [`fn_801daf10`], the
  home-directory getter [`fn_801daf90`], the WIBN header builder
  [`fn_801dafa0`], the free-space probes [`fn_801db080`]/[`fn_801db140`], the
  one-shot error latch [`fn_801db250`], and the two-chunk settings read
  (`0x801DB840`-`0x801DBD70`) driven by the `0x803962xx` globals.

The chunk layout of the settings read is observed, not guessed: slot 0 is the
`0x4000`-byte `/shared2/sys/SYSCONF` chunk staged in the mount buffer, slot 1 is
the `0x100`-byte title settings chunk staged at `0x80003800`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801D6850` | `fn_801d6850` | 36 | medium |
| `0x801D6880` | `fn_801d6880` | 52 | medium |
| `0x801D6F20` | `fn_801d6f20` | 64 | medium |
| `0x801D8930` | `fn_801d8930` | 192 | medium |
| `0x801D89F0` | `fn_801d89f0` | 104 | medium |
| `0x801D8AE0` | `fn_801d8ae0` | 104 | medium |
| `0x801D8BD0` | `fn_801d8bd0` | 152 | medium |
| `0x801D9040` | `fn_801d9040` | 132 | medium |
| `0x801D9320` | `fn_801d9320` | 116 | high |
| `0x801D94A0` | `fn_801d94a0` | 272 | medium |
| `0x801D96D0` | `fn_801d96d0` | 120 | medium |
| `0x801D9750` | `fn_801d9750` | 120 | medium |
| `0x801D9F00` | `fn_801d9f00` | 92 | medium |
| `0x801DA860` | `fn_801da860` | 300 | medium |
| `0x801DAE70` | `fn_801dae70` | 68 | medium |
| `0x801DAEC0` | `fn_801daec0` | 72 | medium |
| `0x801DAF10` | `fn_801daf10` | 120 | medium |
| `0x801DAF90` | `fn_801daf90` | 12 | high |
| `0x801DAFA0` | `fn_801dafa0` | 220 | medium |
| `0x801DB080` | `fn_801db080` | 188 | medium |
| `0x801DB140` | `fn_801db140` | 264 | medium |
| `0x801DB250` | `fn_801db250` | 84 | high |
| `0x801DB840` | `fn_801db840` | 160 | medium |
| `0x801DB8E0` | `fn_801db8e0` | 284 | medium |
| `0x801DBA00` | `fn_801dba00` | 276 | medium |
| `0x801DBB20` | `fn_801dbb20` | 208 | medium |
| `0x801DBBF0` | `fn_801dbbf0` | 232 | medium |
| `0x801DBCE0` | `fn_801dbce0` | 140 | medium |
| `0x801DBD70` | `fn_801dbd70` | 292 | medium |
| `0x801DCC60` | `fn_801dcc60` | 12 | high |

## Deliberate skips

- `0x801D68C0` (660 bytes) and `0x801D6B60` (940 bytes): the decoder core and
  its caller. `0x801D68C0` is translatable in principle, but its inner loop
  merges the four bit-shift arms with `*(char *)(state + 0xF) == 4` and a
  `param_3`/`iVar7` byte-count adjustment that I could not reconcile with the
  byte ordering of the four arms, so no body is claimed.
- `0x801D6F60` (2036 bytes), `0x801D7760`-`0x801D7D50` (4 bodies, 2444 bytes):
  the NAND task tail and the SLPI scan helpers. The `0x801D7950` binary search
  divides by `param_3 / *(u32 *)(state + 8)` and mixes widened halves
  (`(uVar4 + uVar5) >> 1` used as both index and byte offset); the remaining
  bodies are 0x801D9xxx/0x801DExxx chains whose file-object layout is not
  proven.
- `0x801D82E0`-`0x801D8D20` (13 remaining bodies): the file-system cores. Most
  take their first two words through the `0x802DCD74` prologue and their
  callers pass the path and buffer in an order that the exports do not
  disambiguate consistently (for example `0x801D8A60` stores its `r3` at
  `context + 0x04` while `0x801D9750` stores its `r4` at `context + 0x08`).
  Only the four pair-free wrappers were taken.
- `0x801D91C0` (340 bytes): the attribute core is understood (path build,
  `/shared2` test, `0x80226BF0`/`0x80226D50`), but its asynchronous tail takes
  nine arguments. `crate::platform` exposes `call0`-`call7` only, so the async
  arm cannot be expressed honestly; the synchronous arm alone would make the
  body `partial`, and Wave 87 already reaches it through [`ISFS_GET_ATTR`].
- `0x801D9950`-`0x801DAB90` (10 remaining bodies, including the rules engine
  and the `0x801DAB90` open core): each is 200-950 bytes of untranslated
  calibration/file logic whose helper calls and string constants are not
  resolved. Not attempted.
- `0x801DC280` and `0x801DC420` (404 and 632 bytes): the settings-store
  compaction pair, deliberately deferred in Wave 87 and still deferred.
- `0x801DCC70` (540 bytes) and `0x801DCE90`: the SYSCONF mount drivers. The
  `functions.tsv` size for `0x801DCE90` is 88 bytes, but its export contains a
  switch that runs to `0x801DD1B0` (800 bytes), so the size/export pair is
  internally inconsistent and the byte count cannot be cited honestly.
- `0x801DE010` (2296 bytes) and `0x801DEB70` (2524 bytes): the two large
  remaining routines in the page. Neither was attempted.
- `0x801DFAC0`-`0x801DFB30` (seven 4-byte thunks): pure branches into REL code;
  Wave 87 already recorded them as out of scope, and a 4-byte tail branch has no
  body worth reconstructing.
- `0x801DA7B0` (164 bytes) remains skipped from Wave 87: the export's backward
  separator scan leaves the index at `-1` for a path with no `/`, which cannot
  be the real behaviour.

## Verification

- Every address and byte size matches
  `work/ghidra/main-gb/export/functions.tsv` (30 rows, 4488 bytes, 4 high and
  26 medium).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/dvd.rs`.
- No address was already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave91` prints `OK` and lists
  `wave91-dvd.csv: 30 rows`.
- `cargo check --message-format=short` reports no error or warning naming
  `src/runtime/dvd.rs` (the run stops on errors in another lane's
  `src/engine/main_loop.rs`, which this lane does not own).
