# Coverage - Wave 215 lane F (engine::dsys)

## How to read this file

`engine::dsys` reconstructs the `dSys_c` engine start-up together with the
display, text, sprite and ribbon helpers around it in the main `DOL`. This lane
takes the ten contiguous bodies at `0x800F21A0..0x800F2D38`, which sit above the
`0x800E0000` bucket and were still untranslated.

Each row below is one `pub unsafe fn fn_<address>`. The doc block above the body
cites the Ghidra export, the byte count from
`work/ghidra/main-gb/export/functions.tsv`, and a confidence level:

- `high` means every guard, store and call argument is literal in the export;
- `medium` means a field role, a callee identity or a pointer split was inferred
  and is marked `// inferred:` or `// unknown:` in the body.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/dsys.rs` | `engine::dsys` | 10 | 1324 | 6 | 4 |

The lane's bodies live in `work/wave215/laneF_block.rs`; the row list is
`ledger/agents/wave215-laneF.csv`.

## What this pass covers

All ten bodies belong to one cluster: a small resource-descriptor system built on
0x54-byte records and 0x1C0-byte table entries.

| address | function | bytes | role |
|---|---|---:|---|
| `0x800F21A0` | `fn_800f21a0` | 268 | recursive walk that registers a node id once |
| `0x800F22B0` | `fn_800f22b0` | 60 | membership probe over the fifteen id slots |
| `0x800F22F0` | `fn_800f22f0` | 12 | `+0x90` word, bit 0 |
| `0x800F24B0` | `fn_800f24b0` | 120 | bind an archive model, reset three vector words to `1.0` |
| `0x800F2680` | `fn_800f2680` | 112 | install the `0x8031BC08` static descriptor |
| `0x800F2710` | `fn_800f2710` | 96 | construct the object at `+0x60`/`+0x128` |
| `0x800F2770` | `fn_800f2770` | 88 | unlink and optionally release |
| `0x800F2AF0` | `fn_800f2af0` | 104 | fill one 0x54-byte record |
| `0x800F2B60` | `fn_800f2b60` | 88 | reset one 0x54-byte record |
| `0x800F2BC0` | `fn_800f2bc0` | 376 | walk the source table and marshal every record |

### 1. Byte sizes are real extents, not prologue artefacts

Every one of the ten sizes was re-checked against `work/dol/main.elf`. The last
instruction of each body is the `blr` at `addr + size - 4`, so no size is a 16/20
byte prologue artefact:

| address | prologue | `blr` | `functions.tsv` |
|---|---|---|---|
| `0x800F21A0` | `stwu r1,-0x20` | `0x800F22A8` | 268 |
| `0x800F22B0` | none | `0x800F22E8` | 60 |
| `0x800F22F0` | none | `0x800F22F8` | 12 |
| `0x800F24B0` | `stwu r1,-0x10` | `0x800F2524` | 120 |
| `0x800F2680` | `stwu r1,-0x20` | `0x800F26EC` | 112 |
| `0x800F2710` | `stwu r1,-0x10` | `0x800F276C` | 96 |
| `0x800F2770` | `stwu r1,-0x10` | `0x800F27C4` | 88 |
| `0x800F2AF0` | `stwu r1,-0x20` | `0x800F2B54` | 104 |
| `0x800F2B60` | none | `0x800F2BB4` | 88 |
| `0x800F2BC0` | `stwu r1,-0x40` | `0x800F2D34` | 376 |

### 2. `0x800F21A0` is a recursion, not a loop

The body is a depth-first walk: it offers the node's `+0xC` id to
`0x800F22B0`, compares the node's relative record with the archive handle at
`r13 - 0x6DC0` through `0x802E1BD0(record, handle, 4)`, appends the id when the
comparison succeeds, then walks `0x800DFD20`/`0x800DFDD0` children and calls
itself with a one-word stack cell as the new node-pointer slot. The pointer
indirection is why the reconstruction passes `child.as_mut_ptr()`.

### 3. `0x800F2BC0` drives `0x800F2AF0`, and the flag lands at `+0x50`

The destination record layout is confirmed by the two bodies together:
`0x800F2AF0(record)` writes `record+4` (a five-byte copy of the name),
`record+0xC/+0x10/+0x14` (three floats from the 20-byte part record written by
`0x80168230`) and `record+0x4C` (the entry index). `0x800F2BC0` stores the flag
at `(dest + i*0x54) + 0x54`, which is `record + 0x50`: the last free word of the
same record, not the next one. `0x800F2B60` clears exactly `+4..+8`, `+0x18..
+0x2C` to zero and `+0x30..+0x4C` to `-1`, which leaves `+0x50` and the float
slots to the other two bodies.

### 4. Branch polarity in the probe chain

The five probe results are consumed with `bc 4,2` / `bc 12,2` pairs, i.e. the
flag is cleared (`0`) when *any* probe reports a hit and kept at `1` only when
all of them report a miss. The `0x800F9530` test is `bge`-to-clear, so an id of
`0` or more counts as a hit (Ghidra prints the same test as `-1 < iVar4`). The
`'K'` fast path is the only route that sets `1` unconditionally.

### 5. Float vs word stores in `0x800F2710` and `0x800F2680`

`0x800F2710` ends with `stfs f0,0x134(r31)` where `f0` was loaded from
`r2 - 0x6150` (`0.0`); the nearby `lwz`/`stw` trio is GPR work, so `+0x134` is a
float field and `+0x128/+0x12C/+0x130` are word copies of `0x80377DC8..0x80377DD0`.
`0x800F2680` mirrors the same pattern with three `lfs`/`stfs` pairs and one
`lwz`/`stw`, and repeats the three float stores into a dead stack area at
`r1 + 8..r1 + 0x10`; nothing reads that area, so those three stores are omitted
and noted in the doc block.

## Deliberately skipped

None. All ten bodies were reconstructed.

## Field notes

- The `0x800F2680` image constants are read at run time from `0x8042CF24`,
  `0x8042CF28`, `0x8042CF2C` (`2160.0`, `-30.0`, `-478.0`), `0x8042CF6C` (`23`)
  and `0x8042CF84` (`38`), so no float or word is invented in the body.
- `0x800F24B0` calls the literal four-byte thunk `0x80007510`, not its target
  `0x80164FD0`, because the image `bl` is to the thunk.
- `0x800F2710` calls `0x8006C6D0` with `r3 = this`; Ghidra drops the argument in
  the export, so the reconstruction passes it explicitly.
- `0x802DCD64`/`0x802DCDB0` in `0x800F2BC0` are `_savegprlr_25` /
  `_restgprlr_25`; they save and restore `r25..r31` plus `lr` and are omitted.
- `0x800F57A0`/`0x800F57C0` return `entry & *(u32 *)(entry + 0x1C8/0x1D0)`, not a
  boolean; only the caller's `!= 0` test is reproduced.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (`800f21a0` 268, `800f22b0` 60, `800f22f0` 12, `800f24b0` 120, `800f2680` 112,
  `800f2710` 96, `800f2770` 88, `800f2af0` 104, `800f2b60` 88, `800f2bc0` 376).
- Every evidence file exists at the cited path.
- Ten `pub unsafe fn fn_<address>` bodies, one per ledger row.
- No address is already in `ledger/functions.csv`.
- `cargo check --lib` in a scratch copy of the crate with the lane block
  appended to `src/engine/dsys.rs` exits `0` with no warning in the new code.
- `node tools/check-lane-csv.mjs wave215` run in a scratch tree that carries the
  appended `src/engine/dsys.rs` reports `OK` with 10 rows for this lane.
