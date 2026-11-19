# Coverage - Wave 116 (`game::stage`, closing the `0x800C0000` bucket)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The `evidence`
column of `ledger/agents/wave116-stage.csv` is the Ghidra C export each body was
written from.

The bucket held 290 entries and 287 already had `game::stage` rows, so wave 116
starts from the same three open entries wave 112 left: two 4-byte branch thunks
and the 660-byte message worker. The worker is landed here and the two thunks
are deliberately skipped, which closes the bucket.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 1 | 660 | 0 | 1 | 0 |

The row is `translated`; it is not `partial` or `stub`.

## What this pass covers

- **The message-pane line rebuild** (`0x800C9B50`, 660 bytes). The worker
  behind [`fn_800c9a60`]: it resolves the frame record for the `start..end`
  window through `0x802D79A0` (byte `+6` for the font id, word `+4` for the line
  width in hundredths), installs the `0x1C`-byte font record at
  `0x80358848 + id * 0x1C` through `0x802AF080`, writes the id to the shared
  message table byte `0x80359534`, clears message state `+0x3DA`, may re-mark it
  through `0x800C9B00`, stores the two frame words at `+0x3D0`/`+0x3D4`, and
  formats and draws the line with [`fn_800c9e80`].
- **The two-pass width fit.** The pane width `+0xE8` becomes
  `0.01f * u16(+4) * (font vtable +0xC)`, the pane is moved to the `1024.0f`
  x limit, and `0x802AF240` measures the line twice: pass one with the pane's
  own `+0xF4`, pass two with `+0xF4` zeroed and the `r13-0x55B8` flag set. If
  the measured width exceeds the pane's entry `+0x4C` limit and the first
  width is positive, the width is scaled by
  `(limit - (measured - normal) - 4.0) / normal` and only written back when the
  result is smaller than the current width. The `+0xEC` height, `+0x4C`,
  `+0x50` and `+0xF4` are restored on every exit, and the draw record is
  released with `0x802307E0`.
- **Why wave 112 could not land it.** Its export opens with two `psq_st`
  blocks that scale "incoming" paired-single arguments through `GQR0`. The code
  bytes show the block is `stfd f31,0xF0(r1)` + `psq_st f31,0xF8(r1),0,0` and
  the `f30` pair at `0xE0`/`0xE8`, with the matching `lfd`/`psq_l` reloads in
  the epilogue: they are the non-volatile `f30`/`f31` spills, the same artefact
  wave 115 documented for `0x8007AC40`, and they consume no arguments.
- `0x802DCD68` is `__savegprlr_24` (and `0x802DCDB4` is `__restgprlr_24`), so
  the export's `iVar5`/`uVar10` are the incoming `r3`/`r4` - the pane and the
  message object - and the argument lists the export drops for
  `0x800CE7D0`/`0x800CE7A0`/`0x802D7860` were recovered from the `mr` moves at
  `0x800C9B98..0x800C9BA4`, `0x800C9BF8..0x800C9C00` and
  `0x800C9C24..0x800C9C2C`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800C9B50` | `fn_800c9b50` | 660 | medium |

Address range: `0x800C9B50` (1 body, 660 bytes).

## Deliberately skipped

- **`0x800C3BA0`** (`thunk_FUN_800c3bac`, 4 bytes). The entry is a 4-byte
  branch into `0x800C3BAC`, which already owns a `game::stage` row (`high`,
  `work/ghidra/main-gb/export/800c3bac_FUN_800c3bac.c`). Its export is the
  target body, so a second body here would double-count the same instructions -
  the precedent of `0x800776F0` in wave 115.
- **`0x800C8A90`** (`thunk_FUN_80007320`, 4 bytes). The same 4-byte branch
  pattern into `0x80007320`, which already owns a `render::geometry` row
  (`high`, `work/ghidra/main-gb/export/80007320_FUN_80007320.c`); the export is
  the full target body.

One landed + two skipped = the three open entries of the `0x800C0000` bucket,
which is now closed (290/290).

## Verification

- Every address and byte size matches `functions.tsv` (1 row, 660 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_800c9b50` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs`.
- `node tools/check-lane-csv.mjs wave116` reports this CSV clean.
- The module header of `src/game/stage.rs` gained a wave-116 paragraph naming
  the body and the two skips (290 entries, 287 translated before this pass), a
  wave-116 line in the original-range list, and the wave-112 comment on
  `0x800C9B50` was corrected to point at the new body; no earlier count became
  stale.
