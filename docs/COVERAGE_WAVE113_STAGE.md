# Coverage - Wave 113 (`game::stage`, closing the `0x80090000` bucket)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The `evidence`
column of `ledger/agents/wave113-stage.csv` is the Ghidra C export each body was
written from.

The bucket opened with 191 entries and 186 already had `game::stage` rows, so
wave 113 starts from five open rows. Four are landed here and one is
deliberately skipped, which closes the bucket.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 4 | 2104 | 0 | 4 | 0 |

All four rows are `translated`; none is `partial` or `stub`. The pass covers
`0x80093F10..0x80098900`.

## What this pass covers

- **The pane effect dispatcher** (`0x80093F10`, 1120 bytes). Ghidra declares it
  `(void)` because the `__savegprlr` prologue (`0x802DCD74`) drops `r3`/`r4`;
  both halves of the save-helper return are live (the `0x800B3E50` family in
  this file uses the same convention), so the two parameters are restored as
  `pane` (`r3`) and `mode` (`r4`). The body indexes the effect table behind
  `0x80286630` by `mode`, gates on the `+8` `u16` and the `r13-0x5787` ready
  byte, then builds a 0xA8-byte record whose first two 0x30-byte quads live at
  `+0x00` and `+0x30` and whose colour/flip fields live at `+0x60..+0x9F`. The
  frame offsets were recovered from Ghidra's negative local names (`local_e0`
  is the base, `local_dc` is `+4`, …), which is why the two quad copies and the
  `0x80094660` call line up with the record base.
  - modes `0`, `4`, `5`, `7`, `8`, `9`, `10`: the camera colour block
    (`0x802540B0`, source word `0x80164B30`) becomes quad 1 and the
    already-translated [`stage_effect_screen_matrix`] fills quad 2;
  - modes `1`, `0xB..0xE`: the same plus one effect id from `0x800E27C0`
    (`0x801693F0(0x803575FC, 0, id & 0xFFFF, 0)`) and the two GX flushes;
  - modes `2`, `3`, `6`: both quads are built locally (`0x801C0C10`,
    `0x801C10D0`), the two `0x801DD1D0` wide-screen tests pick the x extent and
    the second quad route, and mode `3` also submits the id.
  - The finished record goes to `0x8016C850`, which resolves pane `pane` in the
    global renderer and calls its vtable `+0x1C` with the record.
  - The two leading `ldexpf` blocks only pre-scale the saved `f30`/`f31`; the
    two locals they compute are never read, so they are the `psq_st` frame-save
    artefact and are not reproduced (the same artefact wave 105 recorded for
    [`fn_800955f0`]).
- **The notice trigger pair** (`0x80097170`, 116 bytes, and `0x80097250`,
  404 bytes). `0x80097170` latches `+0x45A` once `0x80097250` returns code `3`
  (through `0x800A0DC0(this, 0)`), or once any non-zero code meets a set
  `+0x390`. Its three parameters are restored from its only two call sites
  (`0x8002E440`, `0x800A1830`), both of which pass
  `(f1 = r2-0x7C74, r3 = actor, r4 = actor + 0xAC)`, and the callee's own
  prototype `(scale, this, point)` fixes the mapping.
  `0x80097250` runs the already-translated [`fn_80075270`] ground probe with the
  two floats at `point`, the `+0x38F` layer byte and a stack slot seeded from
  `r2-0x6EB8`, then selects one of four vtable groups: codes `1`/`2` latch
  `+0x45D`, save `+0x114`/`+0xF8`/`+0x118` into `+0x4F0`/`+0x4F4`/`+0x4F8` and
  call slots `+0x21C` and `+0xC8`; code `3` calls slot `+0xCC`; code `4` calls
  slot `+0xD0`; code `0` restores the three saved words and clears `+0x45D`.
  Every transition slot receives `(f1 = scale, r3 = this, r4 = &record)` where
  the record is `{ this+0xAC, probed height, r2-0x6E7C }`.
- **The per-frame `Wm` effect update** (`0x80098900`, 464 bytes). Same dropped
  `r3`/`r4` pair as `0x80093F10`, restored as `this`/`mode`. Only mode `2` runs
  the retarget pass: the actor position is normalised by `0x8006A4B0` into
  `+0x5E8`/`+0x5EC`, the four `+0x544` slots are scanned by the 16-byte getter
  `0x80275560`, and the first positive slot walks the `+0x5B8`-per-record array
  of length `+0x598`. Per live record the id mask comes from `0x80196660`
  through the `r13-0x51F8` manager, and the complemented mask drives the four
  actions: mask bit `0` runs the ground lerp `0x80267420` (with the
  `r13-0x51D8` `+0x14`/`+0x10` gate), bit `3` reads the layer word with
  `0x80275780` and stores it through `0x80267390` (plus `+0x5AC` when negative),
  and bit `2` stores `+0x5B4` through `0x802674C0`. The generic mode handler
  [`fn_80095480`] then runs with the same two arguments.

The three helper calls whose arguments the export truncates were restored from
the callees' own bodies: `0x80275780` is an 8-byte tail call that forwards its
`r4`/`r5` to `0x80275FD0`, so it really takes `(handle, index, out)`; and
`0x80267420`/`0x802674C0` take their value in `f1` and the object in `r3`, which
the export prints as a `(double)FUN(...)` cast because the prototypes omit the
floating-point parameter.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80093F10` | `fn_80093f10` | 1120 | medium |
| `0x80097170` | `fn_80097170` | 116 | medium |
| `0x80097250` | `fn_80097250` | 404 | medium |
| `0x80098900` | `fn_80098900` | 464 | medium |

Address range: `0x80093F10..0x80098900` (4 bodies, 2104 bytes).

## Deliberately skipped

One of the five open rows is not landed.

- **The branch thunk** (`0x80096700`, 4 bytes). Its export is the *target*
  body: `thunk_FUN_800967f0`, the 236-byte notice/effect spawner that already
  owns a `game::stage` row (`0x800967F0`, `fn_800967f0`) in this file. Writing a
  second body would double-count the same code, the same precedent as
  `0x800C3BA0`/`0x800C8A90` in wave 112 and the thunks of waves 84 and 98-111.

Four landed + one skipped = the five open entries of the `0x80090000` bucket,
which is now closed.

## Verification

- Every address and byte size matches `functions.tsv` (4 rows, 2104 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`; the four were
  chosen from the five open entries of the `0x80090000` bucket.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs`.
- `node tools/check-lane-csv.mjs wave113` reports this CSV clean.
- The module header of `src/game/stage.rs` gained a wave-113 paragraph naming
  all four bodies (2104 bytes) and the one skip; no earlier count became stale.
