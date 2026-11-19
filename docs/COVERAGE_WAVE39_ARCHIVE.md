# Coverage - Wave 39 (archive)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/archive.rs` | `resource::archive` | 25 | 3180 | 11 | 14 |

## What this pass covers

- the **DVD sound-archive reader** at `0x80268FF0..0x80269480` — the DVD twin of
  the file reader at `0x8026F690` already translated in this file. It reuses
  exactly the same mount-stream base (`0x80275640`/`0x80275D40`/`0x802756E0`/
  `0x80275670`), the data-cursor block at `+0x108`, the index words at
  `+0x120`/`+0x124` and `+0x118`/`+0x11C`, and the file-info block at `+0x14C`.
  The constructor installs vtable `0x8034EAD0` and clears the open byte at
  `+0x188` (the file reader uses `+0x1D8`). The opener converts the path with
  `0x801CA680`, fast-opens it, reads the 0x40-byte header through `0x801CAB20`,
  installs it, and stores the basename (last `/`/`\` component, bounded at
  `0xFF` bytes) through `0x80275CC0`. The destructor, closer and the two block
  readers mirror `0x8026F6E0`/`0x8026F930`/`0x8026FC50`/`0x8026FD00` with the
  DVD close/read calls substituted.

- the **buffer-cache singleton and flush passes** at
  `0x80268C40..0x80268EC0` — the exact callees [`fn_8026ff30`] already calls by
  address (`SOUND_BUFFER_CACHE` = `0x80268C60`, `SOUND_BUFFER_FLUSH_A/B` =
  `0x80268D90`/`0x80268EC0`). The singleton lazily initialises the list at
  `0x803FC984` under interrupt masking (`0x801B1140`/`0x801B1180`) and registers
  `0x80268CF0` through `0x802DC780`. The flush passes hold the sound-buffer
  manager mutex at `+0x354`, re-check the lazy init on every node, and call
  each node's `+0x0C` (A) / `+0x10` (B) slot with `(node, start, start+len)`.

- the **cache-list walkers** at `0x80269A50..0x80269EE0` and the dispatcher at
  `0x8026D770`/`0x8026D7E0` — list-head walks that notify each member
  (`node - 0x108`) through the `0x80267350`/`0x80267370` setters, an insert
  helper, an empty-list initialiser, a `countLeadingZeros >> 5` singleton-count
  dispatcher, and a deleting destructor for the `0x8034EC08` family.

- the **record/list primitives** at `0x8026D860..0x8026DB00` — the exact callees
  [`fn_8026f620`] already forwards to (`SOUND_COPY_RECORDS` = `0x8026D860` with
  the `0xD0` stride). Copy fixed-stride records into a list (8-unrolled loop,
  then remainder), remove a list range, pop the head, and push a node, all
  under interrupt masking.

- the **sibling reader** at `0x8026DD60..0x8026DE90` — a second mount-stream
  reader with vtable `0x8034EC90`, its block at `+0x10C` instead of `+0x108`,
  and an offset word at `+0x108` cleared by `0x8026DE90` before the path reset.

One coherent object family, so `+0x108` (block), `+0x118..+0x124` (index
words), `+0x14C` (file info), `+0x188` (DVD open byte) and `+0x354` (manager
mutex) are justified once and reused.

## What the `0x80260000` band actually contains

`0x80260000` has 260 functions in `functions.tsv`, 15 already translated before
this pass (all the sound reader at `0x8026F2D0..0x8026FF30` in
`resource::archive`). Sampling 10 exports across the band shows it is **not**
uniformly NW4R archive file I/O, and it is certainly not BRRES:

- `0x802605D0..0x80262540` is a mutex-guarded sound/stream object family
  (`0x801B1140`/`0x801B1180` interrupt masking in almost every body) plus
  float interpolation (`0x80260A80`, `0x802610C0`, `0x802612F0` — `CONCAT44`
  uint-to-float and `*(float *)` code). The float bodies are explicitly skipped
  by the lane brief.
- `0x802626F0..0x80264CD0` continues that family: singleton getters
  (`0x80264AE0`, `0x80268C60` shape), deleting destructors, and block helpers.
- `0x802652E0..0x80267390`, `0x802695B0..0x80269F00` and
  `0x8026A670..0x8026D770` are parameter/envelope object families: vtable
  accessor setters (`*(this+4) = arg`, store through the `+0x24` slot) and
  the float envelope at `0x80269660..0x802698E0` (`*(float *)(r2 - ...)`,
  `param_1[1] = (int)(fVar2 * (float)param_1[4])`). The envelope is paired-single /
  float code, skipped per the brief; the r2-relative initialisers
  (`0x80265DB0`, `0x80265E80`, `0x80265FF0`, `0x802605D0`) load constants from
  `r2 - 0x48xx` whose absolute address is not recoverable from the C export
  alone, so they cannot be justified and are skipped.
- `0x80267F40..0x80268A50` is large state code (700–1756 B) with unreachable
  blocks, calling into `0x8026xxxx` helpers — a different object, skipped.
- `0x80268C60..0x80269480` (this pass) is the DVD archive reader + buffer
  cache described above — the only cluster in the band that is unambiguously
  archive/stream owned, because it reuses the exact base, block and index
  fields of the already-translated file reader.
- `0x80269A50..0x8026DE90` (this pass, in part) is the cache-list and
  sibling-reader family; the rest is the repeated entry-object constructors
  at `0x8026A670..0x8026D3B0` (each calling a `0x801Axxxx` helper with a
  `float *` argument — float code, skipped) and the float tables at
  `0x8026DBA0`/`0x8026DC40`.
- `0x8026F2D0..0x8026FF30` (already translated) is the file-based sound
  reader. No BRRES loader/decoder was found in the band — the fellow
  `resource::bres` lane has no ground to claim here.
- `0x80268D80` is a 4-byte thunk (`thunk_FUN_80229190`); skipped per the
  brief's thunk caution.

Identification: the band is NW4R **sound/stream infrastructure** — a
DVD/file archive reader family, its buffer cache, list primitives, and the
sound parameter/envelope objects that consume them. The archive-owned
25-function cluster is the DVD reader + cache + list + sibling set above.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80268C40` | `fn_80268c40` | 24 | high |
| `0x80268C60` | `fn_80268c60` | 140 | medium |
| `0x80268CF0` | `fn_80268cf0` | 92 | medium |
| `0x80268D50` | `fn_80268d50` | 48 | medium |
| `0x80268D90` | `fn_80268d90` | 300 | medium |
| `0x80268EC0` | `fn_80268ec0` | 300 | medium |
| `0x80268FF0` | `fn_80268ff0` | 76 | medium |
| `0x80269040` | `fn_80269040` | 140 | high |
| `0x802690D0` | `fn_802690d0` | 332 | medium |
| `0x80269220` | `fn_80269220` | 76 | high |
| `0x802693F0` | `fn_802693f0` | 136 | high |
| `0x80269480` | `fn_80269480` | 136 | high |
| `0x80269A50` | `fn_80269a50` | 156 | medium |
| `0x80269AF0` | `fn_80269af0` | 96 | medium |
| `0x80269DC0` | `fn_80269dc0` | 72 | medium |
| `0x80269EE0` | `fn_80269ee0` | 28 | high |
| `0x8026D770` | `fn_8026d770` | 112 | medium |
| `0x8026D7E0` | `fn_8026d7e0` | 124 | medium |
| `0x8026D860` | `fn_8026d860` | 324 | medium |
| `0x8026D9B0` | `fn_8026d9b0` | 128 | high |
| `0x8026DAA0` | `fn_8026daa0` | 92 | high |
| `0x8026DB00` | `fn_8026db00` | 72 | high |
| `0x8026DD60` | `fn_8026dd60` | 76 | medium |
| `0x8026DDB0` | `fn_8026ddb0` | 88 | high |
| `0x8026DE90` | `fn_8026de90` | 12 | high |

## Cluster notes

- `0x80268C60` re-checks its lazy-init byte inside the flush walk on every
  node, exactly as the export shows; the init is factored as a literal
  repeated block rather than a helper call, matching the original.
- The flush virtual call's second argument is printed by Ghidra as
  `extraout_r4`; it is the range start (`param_2`) still live in `r4`, and
  the third is `start + len`, matching the `(cache, start, len)` caller in
  [`fn_8026ff30`]. The slot arithmetic (`node[2] + 0x0C` / `+0x10`) is kept
  literal.
- `0x802690D0`'s basename loop has no CWD prefix and no separator handling
  beyond the last `/`/`\`; a path with no separator still returns 1. The
  `0xFF` bound and the early `return 0` are reproduced.
- `0x8026D860`'s 8-unrolled loop is written as a counted loop of eight
  identical link steps; the arithmetic for `count` is the literal
  `(end - (aligned_src - src)) / aligned_stride`.
- `0x8026D9B0`'s `link_addr` walk is the export's `param_1` reassignment
  (`param_1 = puVar2`) written directly.

## Unknowns

1. **`0x80229030`/`0x80229160`/`0x80229190`/`0x80267350`/`0x80267370` are
   called by address.** Their bodies are not in this lane; they are marked
   `// unknown:` at the constants. The call shapes (unlink, remove, insert,
   setter) are literal.
2. **The flush-slot field (`node[2]`) is not identified.** The slot offset
   (`+0x0C`/`+0x10`) is literal; which object word is the vtable is not
   proven, so the bodies add the offset to the stored word exactly as the
   export does.
3. **`0x8026D770`'s singleton pair is dropped by Ghidra.** `0x80260860`
   returns the singleton pointer and `0x802610B0` returns its first word; the
   reconstruction follows the two signatures in `functions.tsv`.
4. **`0x8026D7E0`'s `0x8034EC18` slot call drops its argument.** It is
   forwarded `this`, as in the other deleting destructors in this file.
5. **No Nintendo class symbols are claimed.** Vtables are given as
   `0x8034Exxx` addresses; the mount-stream base is referenced as
   `0x80275640`/`0x80275D40` (already used in this file) rather than invented
   as `SndArchive*` symbols.
6. **`0x801B1140`/`0x801B1180` are named from other modules.** The archive
   code only relies on the mask/restore pairing; the bodies use the addresses
   and mark the names `// inferred:`.

## Deliberate skips

- `0x80260A80`, `0x802610C0`, `0x802612F0`, `0x802614E0`, `0x80261B20`,
  `0x80262DB0`, `0x802638A0`, `0x80263A70`, `0x80269660..0x802698E0`,
  `0x8026DBA0`, `0x8026DC40`, `0x8026A880..0x8026D3B0` (float/paired-single) —
  explicit skip per the lane brief (`CONCAT44` float code, `ldexpf`/`GQR0`
  shapes). The envelope at `0x80269740` is the same family.
- `0x802605D0`, `0x80265DB0`, `0x80265E80`, `0x80265FF0`, `0x8026DB80`,
  `0x80269660` — initialisers that load constants from `r2 - 0x48xx`. The
  absolute address is not recoverable from the C export, so the constants
  cannot be justified.
- `0x8026DE10` (120 B) — the `0x80275D70` call drops its source argument in
  the export and no caller in the band supplies it; the sibling install
  cannot be reconstructed honestly.
- `0x8026DF50` (176 B) — its stack locals (`local_48`/`local_44`/
  `auStack_40`/`local_34`/`auStack_28`/`local_24`/`local_18`) are passed as
  out-parameters to `0x80275840`/`0x80275800`/`0x80275810` but the export
  never shows which call writes which local; the final sum cannot be justified.
- `0x80268D80` (4 B, `thunk_FUN_80229190`), `0x8026F630` (8 B forward to
  `0x8026D9B0` with dropped arguments) — thunks, skipped per the brief.
- `0x80269F00` (264 B) — uses the `0x802DCD74`/`0x802DCDC0` save pair as its
  argument pair and walks a nested list with a dropped-argument virtual call;
  the pair order cannot be justified from the export alone.
- `0x80267300..0x80267750`, `0x80267530..0x80267760` — tiny vtable-accessor
  setters/getters for a parameter object that shares no archive/stream field;
  not the same cluster, left for the owner of that object.
- Everything else in the band (the `0x8026A670..0x8026D3B0` entry constructors,
  `0x80267F40..0x80268A50` state code, and the sound-parameter families) —
  float code, a different object, or already translated; left untouched so the
  archive lane never claims a BRRES loader/decoder. No BRRES code was found in
  the band.
- Wave 36's `0x802B8B40..0x802BA000` heap factories were re-read first and
  are not repeated; none of the 25 addresses above is in
  `ledger/functions.csv` or in that range.

## Verification

- Every address and byte size matches `functions.tsv` (read from the TSV, not
  from prose).
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/archive.rs` with a complete `# Original` / `# Arguments` /
  `# Returns` / `# Side effects` / `# Safety` / `# Confidence` doc block.
- No address was already in `ledger/functions.csv` (all 25 were in the
  `0x80260000` miss list; the 15 done rows in the band are the sound reader at
  `0x8026F2D0..0x8026FF30`).
- `cargo check` is free of errors for `src/resource/archive.rs` (13 pre-existing
  warnings in `src/runtime/*`, another lane's files).
- `node tools/check-lane-csv.mjs wave39` reports `wave39-archive.csv: 25 rows`
  with no problem naming `wave39-archive.csv`.
