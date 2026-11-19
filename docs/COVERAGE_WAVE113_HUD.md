# Coverage - Wave 113 (UI: `ui::hud`)

Pass 15 of the `ui::hud` lane. The lane's `0x80010000` bucket had one
untranslated entry left after wave 107: `0x800129A0`, the course-select guide
state-table initialiser. This pass translates it, so the bucket is now
253 translated of 253 entries.

## How to read this file

Every row maps one Ghidra function to one `pub unsafe fn fn_<addr>` body in
`src/ui/hud.rs`. `address` is the link-time address in `sys/main.dol`, `bytes`
is the size recorded in `work/ghidra/main-gb/export/functions.tsv`, and
`confidence` matches the `# Confidence` line of the body's doc block. The body
is appended under the `Wave 113` section marker at the end of the file.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/hud.rs` | `ui::hud` | 1 | 6,204 | 0 | 1 |

## What this pass covers

- **Course-select guide state-table initialiser, `0x800129A0`.** Ghidra renders
  700 lines of `DAT_` stores, which is why waves 101 and 107 deferred it as
  opaque data. Parsing the export block by block shows the body is a single
  fully-unrolled loop of 32 mechanically identical descriptor registrations, so
  it is reconstructible:

  - descriptor `i` lives at `0x803516B8 + i*0x40` (32 records, through
    `0x80351EB7`);
  - its name pointer comes from the 32-entry string table
    `0x802FEFC8..0x802FF600` (`StateID_Wo`, `Gu`, `Up`, `Do`, `Le`, `Ri`, `Sh`,
    `Sc`, four states each; variable-length strings, so the pointers are
    listed);
  - `0x8015F7C0(state, name)` constructs it and the shared vtable
    `0x802FF63C` is then written at `+0x00`;
  - nine words come from `0x802FEB48 + i*0x24` and are copied into
    `+0x0C..+0x2C` in the fixed permutation `6,7,8,3,4,5,0,1,2` — the same
    descriptor protocol already documented for `fn_80016a80` and
    `fn_80018330`, with a 9-word (not 8+1) source block;
  - `0x802DC780(state, 0x80014240, event)` links it, where `event` is
    `state - 0x10` for entries 1..31 and the fixed node `0x803516AC` for
    entry 0. The `-0x10` word is the tail of the previous `0x40`-byte record,
    i.e. a per-entry link node.

  The preamble before the loop copies four `.sdata2` words into the `.data`
  block at `0x802FE4B4`/`0x802FE4C0..0x802FE4C8`, registers the `0x18`-byte
  header node `0x803516A0` with callback `0x800141E0` and a null object, copies
  `SDA2 - 0x60FC` into the SDA word at `0x80429BF8`, and writes the one-shot
  halfword pair `0x14`/`0x46` at `0x80429C00`/`0x80429C02` behind the guard byte
  at `0x80429C04`. Those roles are not established by the export and stay
  labelled unknown.

  The reconstruction keeps the unrolled order (same records, same order, same
  writes) but expresses it as a loop, because all 32 blocks were verified
  identical.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800129A0` | `fn_800129a0` | 6,204 | medium |

## Deliberate skips

None. This was the last untranslated entry in the bucket.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (`800129a0`, `FUN_800129a0`, 6204 bytes).
- Every evidence file exists at the cited `work/ghidra/main-gb/export/...` path.
- The row has exactly one `pub unsafe fn fn_800129a0` body in `src/ui/hud.rs`,
  and the body carries one `# Original` block.
- `0x800129a0` was not in `ledger/functions.csv` before this pass; the CSV
  validator re-checks this live against the merged ledger.
- `cargo check --message-format=short` reports no error for `src/ui/hud.rs`.
- `node tools/check-lane-csv.mjs wave113` prints `OK` for
  `ledger/agents/wave113-hud.csv`.
