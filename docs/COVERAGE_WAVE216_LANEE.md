# Coverage - wave 216 lane E (`engine::dsys`, `0x800F2DF0..0x800F71A0`)

Lane E of wave 216 translated the ten `0x800F0000`-bucket helpers that work on
the module's 0x1C0-byte record array and its 0x18-byte name table. All ten are
`translated` with `high` confidence; nothing was deferred.

Block: `work/wave216/laneE_block.rs`. Ledger rows:
`ledger/agents/wave216-lanee.csv`. Target file: `src/engine/dsys.rs` (not edited
by this lane).

Total: 1,908 bytes selected across 10 functions.

## Bodies

| Address | Bytes | Name | Description | Confidence |
| --- | ---: | --- | --- | --- |
| `0x800F2DF0` | 188 | `fn_800f2df0` | Table pass: for each of the `+0x10` entries, `0x800F5830` resolves the 0x18-byte entry, `0x800F9270`/`0x800F92C0` copy its 4-byte label (`+1`) and 4-byte key (`+5`) into a 16-byte and an 8-byte stack buffer, both are resolved to record indices by `0x800F2EB0` and the pair goes to the ordering pass `0x800F2F20` when the label byte is non-zero. The count is re-read before every iteration. | high |
| `0x800F32F0` | 64 | `fn_800f32f0` | Counted loop over the 0xC0 records of 0x54 bytes: return `base + index * 0x54 + 4` for the first record whose `+0x50` word equals the key, else `0`. | high |
| `0x800F3330` | 140 | `fn_800f3330` | Two lookups - `(base, arg2)` and `(base, arg3)` - then scan the six `+0x18` slots of the first record (skipping zero slots) for the second record; returns the slot index or `6`. | high |
| `0x800F3E20` | 84 | `fn_800f3e20` | Copy up to 5 bytes into the active record's `+0x18` label (`0x800F3460`) and add the consumed byte count to the caller's cursor. | high |
| `0x800F4B50` | 592 | `fn_800f4b50` | CSV field parser: `""` stores nothing and skips two bytes; a quoted field copies up to nine 10-byte rows until the closing quote; an unquoted field copies rows until a comma or the `0x800F5F10` probe fires. Cells go to `+0xE5` (`mode == 0`) or `+0x31`, the row count to `+0x190`/`+0x18C`. `*(object + 0xC)` is re-read for every byte store. | high |
| `0x800F4DA0` | 468 | `fn_800f4da0` | CSV string parser: copy into 0x14-byte rows until a comma, a quote or a CRLF, NUL-terminate the row, bump `+0x1C4`/`+0x1C0` when the row was non-empty, then either stop (quote, plain comma after an unquoted field, CRLF) or advance one row of 10 bytes. Cells go to `+0x1AC` (`mode == 0`) or `+0x198`. | high |
| `0x800F4F80` | 204 | `fn_800f4f80` | Read a 10-byte key into a 56-byte stack buffer, compare it with the `count - 1` existing 0x18-byte entries at `+0x15018`, append it at index `count` when nothing matches, and advance the cursor by the bytes read. The count is read twice, as in the original. | high |
| `0x800F53A0` | 128 | `fn_800f53a0` | Scan the `+0xC` records starting at `+0x18` with 0x1C0 stride; return the index whose name compares equal through `0x802E1AB4`, else `-1`. | high |
| `0x800F57C0` | 20 | `fn_800f57c0` | `mask & *(u32 *)(object + index * 0x1C0 + 0x1D0)`. | high |
| `0x800F71A0` | 20 | `fn_800f71a0` | Publish `this + index * 0x14 + 4` at `this + 0x504`. | high |

## Decoding notes

- **Sizes are all real.** `functions.tsv` gives 188 / 64 / 140 / 84 / 592 / 468
  / 204 / 128 / 20 / 20. Every body's last instruction is a `bclr 20,0` exactly
  four bytes before the next table entry (`0x800F2EAC`, `0x800F3330`,
  `0x800F33BC`, `0x800F3E74`, `0x800F4DA0`, `0x800F4F74`, `0x800F504C`,
  `0x800F5420`, `0x800F57D4`, `0x800F71B4`), so none of the ten sizes is a
  prologue artefact. Three of them (`0x800F32F0`, `0x800F57C0`, `0x800F71A0`)
  are leaf functions with no stack frame at all.
- **`0x802DCD6C` / `0x802DCDB8` in `0x800F4B50` and `0x800F4F80`.** They are
  the outlined `__savegpr_25..31` / `__restgpr_25..31` helpers: `0x802DCD6C` is
  `stw r25,-0x1C(r11)` and the four words that follow, so it preserves `r3`/`r4`
  unchanged. The `undefined8` the exports print is therefore just
  `(r3, r4) = (object, text)`, the ordinary first two arguments. Neither stub is
  emitted in the block.
- **`0x800F3330`'s first argument-less call.** Ghidra prints `FUN_800f32f0()`
  for the first lookup because the export never rewrites `r3`/`r4` before the
  `bl`; the key is argument 2. The second lookup uses `(argument 1, argument 3)`,
  which arrive in `r3`/`r5` (`mr r31,r3` / `mr r29,r5`).
- **The `and` in `0x800F57C0`.** `0x800F57CC` decodes to `and r3, r5, r0`
  (`rA = 3`, `rS = 5`, `rB = 0`), i.e. the result goes to `r3`, matching the
  export's `return param_3 & *(uint *)(...)`. The disassembler's destination-first
  rendering reads `and r5, r3, r0`, which would make the export wrong; the
  encoding and the exported body agree.
- **Delimiter handling in `0x800F4DA0`.** `iVar4 == 0` inside the comma/CRLF exit
  path is the "empty row" case: only a comma is skipped, never a CRLF.
- **`0x800F4B50`'s probe.** The end-of-field probe `0x800F5F10(object, text)` is
  called again for the do/while exit test, so a non-zero answer is queried twice,
  exactly as the original does.

## Verification

`cargo check --lib` on a scratch copy of `src/`, `Cargo.toml` and `Cargo.lock`
with this block appended to `src/engine/dsys.rs`: exit 0, no new warnings.
