# Wave 216 lane D coverage (`engine::dsys`)

Slice: the ten `engine::dsys` bodies at `0x800F2F20`, `0x800F3460`,
`0x800F4870`, `0x800F5050`, `0x800F56B0`, `0x800F6320`, `0x800F64E0`,
`0x800F6500`, `0x800F7020` and `0x800F7270` (1832 bytes of `functions.tsv`
extent). All ten are translated; nothing was deferred.

Bodies live in `work/wave216/laneD_block.rs` and are appended to
`src/engine/dsys.rs` by the merge step; ledger rows are in
`ledger/agents/wave216-laneD.csv` (`module = engine::dsys`).

This is the CSV/record cluster that sits between the arc helpers and the
`0x800F6xxx` family: a grid-cell neighbour linker, a CSV field reader, the CSV
row writer that fills the `0x1C0`-byte records, the name-table resolver, the
record accessor, a three-float forwarding step, a 16-bit cursor accumulate, a
zero-crossing test, and the two small object helpers `0x800F7020` /
`0x800F7270`.

## Translated

| Address | Bytes | Rust body | Summary |
| --- | --- | --- | --- |
| `0x800F2F20` | 408 | `fn_800f2f20` | Links two `0x54`-byte grid cells. Reads the x/y/z differences against the pitch word `0x8042CF48` (`0x40400000`, `3.0f`) and the sign word `0x8042CF4C` (`0.0f`): both x and z within the pitch -> the y pair `+0x1C`/`+0x20` is written from the y difference; only x within -> the z pair `+0x24`/`+0x28` from the z difference; only z within -> the x pair `+0x2C`/`+0x30` from the x difference; neither -> the z pair again (with the dead `|dz| < pitch` re-test at `0x800F3034` preserved). Every case stores the other cell's `+4` pointer into the *second* slot of the smaller cell and into the *first* slot of the larger. Returns `1`. |
| `0x800F3460` | 148 | `fn_800f3460` | CSV field copy into `param_1`. An opening `"` is skipped; `""` and a leading `,` return `0`. The loop stores until `,`, `"` or a CR LF pair, writes the terminating NUL and returns the byte count. The third register argument the caller passes (`0x14`) is not used by the body. |
| `0x800F4870` | 736 | `fn_800f4870` | CSV row writer. Target is `param_1 + *(param_1 + 0xC) * 0x1C0`, i.e. the record of the cursor index; payload bytes go to `+0x1D` when `param_4 != 0` and to `+0xD1` when it is `0`, skipping every `,`. `""` only advances the cursor by two; the quoted path stores four bytes then skips one byte per group and ends on the closing quote; the bare path stores four bytes per group and then scans to `,` or CR LF. The group count lands at `+0x188` / `+0x194`. |
| `0x800F5050` | 212 | `fn_800f5050` | Pulls one CSV token into a 20-byte stack buffer with [`fn_800f3460`], advances the cursor by the copied length and, when `param_1 + *(param_1 + 0x10) * 0x18 + 0x15018` is not ASCII `'0'`, walks the null-terminated name table `0x8031BDD0` with `0x802E1AB4` (`strcmp`); the first equal entry stores `0x802F3370[index]` at `+0x15024`. |
| `0x800F56B0` | 16 | `fn_800f56b0` | `param_1 + param_2 * 0x1C0 + 0x18`: the accessor already used by `fn_800f2bc0` as `FN_800F56B0_F`. |
| `0x800F6320` | 104 | `fn_800f6320` | Copies three floats from `param_2` to `+0x168`/`+0x16C`/`+0x170`, calls `0x800F6110` with `f1 = 0x8042CFAC` (`0xC2480000`, `-50.0f`) and `r4 = object + 0xAC`, `r5 = object + 0x168`, `r6 = param_3`, then stores the halfword returned by `0x80160E80(obj + 0xAC, obj + 0x168)` in both `+0x102` and `+0x108`. |
| `0x800F64E0` | 28 (24 real) | `fn_800f64e0` | `lha` the signed halfword at `+0x100`, add the signed `param_2`, `sth` it back; `beqlr` when `param_3 == 0`, otherwise mirror the same sum into `+0x106`. |
| `0x800F6500` | 184 | `fn_800f6500` | Three times `0x801C1650(param_2, other)` + `0x802BE470()` for `other = param_3`, `object + 0xB8`, `object + 0xAC`. Returns `1` exactly when the first delta is below the threshold `0x8042CFB4` (`0.0f`) and the second is not, i.e. when the measured distances straddle zero. |
| `0x800F7020` | 64 | `fn_800f7020` | Releases `param_1` through `0x802B9280` when it is non-null and the signed `param_2 > 0`; returns `param_1`, which is still in `r3` at the release call. |
| `0x800F7270` | 128 | `fn_800f7270` | Fast path when the byte at `+0x510` is zero (returns the raw `+0x504` word); otherwise the state word at `+0x508` selects the first slot (`0` -> `+0xC`, `1` -> `+0x10`), a null first slot falls back to the other, and a null fallback with `param_2 != 0` flips the state word. |

## Sizes and extents

Every `# Original` block keeps the canonical `main-gb` `functions.tsv` value,
and each one was re-checked against the prologue/leaf head and the closing
`blr` in `work/dol/main.elf`:

- `0x800F2F20`: leaf, `mulli r8,r5,84` at `+0`, `li r3,1` at `0x800F30B0`,
  `bclr 20,0` at `0x800F30B4`; ends at `0x800F30B8` = 408 bytes exactly.
- `0x800F3460`: leaf, `lbz r5,0x0(r4)` at `+0`, `bclr 20,0` at `0x800F34F0`;
  ends at `0x800F34F4` = 148.
- `0x800F4870`: `stwu r1,-0x30` + `bl 0x802DCD6C` at `+0`; the tail is
  `bl 0x802DCDB8` / `mtspr 256` / `addi r1,r1,48` / `bclr 20,0` at
  `0x800F4B4C`; ends at `0x800F4B50` = 736.
- `0x800F5050`: ends at `0x800F5124` = 212 (the three `OP_0` words after it are
  padding).
- `0x800F56B0`: three instructions, `bclr 20,0` at `0x800F56BC`; ends at
  `0x800F56C0` = 16.
- `0x800F6320`: ends at `0x800F6388` = 104 (two padding words to `0x800F6390`).
- `0x800F64E0`: **tsv 28, real 24**. The body is `lha` / `cmpi` / `add` / `sth`
  / `bclr 12,2` / `sth` / `bclr 20,0`, i.e. `0x800F64E0..0x800F64F8`; the four
  bytes at `0x800F64F8..0x800F64FC` are the `OP_0` padding before the next
  function and the tsv size is a prologue artefact. The `# Original` line keeps
  28 so the lane checker matches, and the doc block records the real extent.
- `0x800F6500`: `stwu r1,-0x30` at `+0`, `bclr 20,0` at `0x800F65B4`; ends at
  `0x800F65B8` = 184 (two padding words to `0x800F65C0`).
- `0x800F7020`: `stwu r1,-0x10` at `+0`, `bclr 20,0` at `0x800F705C`; ends at
  `0x800F7060` = 64.
- `0x800F7270`: leaf, `lbz r0,0x510(r3)` at `+0`, `bclr 20,0` at
  `0x800F72EC`; ends at `0x800F72F0` = 128.

## Evidence notes

- `0x800F2F20` is the one body where the Ghidra export and the image disagree
  on *which* diff feeds which branch. The decompiler names the first `fsubs`
  result `fVar2` (the x difference, `0x800F2F4C`) and the second `fVar3` (the z
  difference, `0x800F2F54`), but its branch bodies then compare `fVar2` at
  `0x800F3050`, where the image loads `f6` (the z difference, `fc060040`). The
  reconstruction follows the raw words: `r9 = |dx| < pitch` (from
  `0x800F2F5C`/`0x800F2F68`) and `r10 = |dz| < pitch` (`0x800F2F70`), the three
  branches at `0x800F2FBC` / `0x800F2FF8` / `0x800F3034` then test those flags
  in that order, and each branch's `fcmpo` operand was decoded from its own
  encoding (`fc02` = dy, `fc05` = dx, `fc06` = dz).
- `0x800F3034` re-computes `|dz|` and re-tests it against the pitch after the
  path into it has already established `|dz| >= pitch` (it is entered from
  `0x800F2FFC`, the `r10 == 0` arm). Ghidra flags the block as unreachable; the
  Rust body keeps the test because the instruction is in the image.
- `0x800F4870`'s `FUN_802dcd6c()` / `FUN_802dcdb8()` pair in the export is the
  outlined `__savegprlr_25` / `__restgprlr_25` register save and restore
  (`work/dol/main.elf` at `0x802DCD6C` stores `r25`, `0x802DCDB8` reloads it),
  not a call returning a pair. The decompiler's `iVar2`/`iVar3` are therefore
  the incoming `r3` (the object) and `r4` (the text base), which
  `0x800F4888..0x800F4898` copies into `r26`/`r27` before the first use.
- `0x800F4870`'s bare-field path ends with a scan (`0x800F4AD0`) and then the
  same two predicates again (`0x800F4AE4`). Because the scan only ever exits on
  `text[*pos] == ','` or on `0x800F5F10(...) != 0`, the second test is always
  true, so the continuation at `0x800F49E4` is unreachable; both are reproduced
  so the control flow matches the image.
- `0x800F5F10` is documented as `(object, text, index)`: `r3` is never read,
  `lbzx r0,r4,r5` tests `text[index]` for CR and `0x800F5F2C` tests
  `text[index+1]` for LF. Ghidra drops the third argument at both call sites in
  `0x800F4870`; the reconstruction passes the live `*pos` through
  `platform::call3`.
- `0x800F5050` passes `r5 = 0x14` to `0x800F3460`, which ignores the register;
  the buffer is the 24-byte stack slot at `r1 + 8` (only 20 bytes are named by
  the caller's limit) and the Rust body declares 24 bytes.
- The `ldexpf` blocks at the head of `0x800F6500`'s export are the `psq_st`
  saves of `f30`/`f31` (`0x800F6510` / `0x800F6518`), i.e. non-volatile
  register saves, and are omitted. The two `fcmpo` results are combined by
  `cror cr0[EQ],cr0[GT],cr0[EQ]` (`0x800F657C`), which is why the second test
  reads as `d5 - d3 < threshold` rather than `<=`.
- `0x800F6320`: Ghidra prints six arguments for the `0x800F6110` call because
  it folds `r3`/`r4` into one `undefined8` and treats the old `r5` as a
  parameter. The raw words are unambiguous: `mr r6,r5`, `lfs fr2` (the
  `-50.0f` word at `0x8042CFAC`), `addi r5,r3,360`, `addi r4,r3,172`,
  `bl 0x800F6110`.
- The three float/word constants were read from `work/dol/main.elf` and are
  recorded in the doc blocks: `0x8042CF48 = 0x40400000` (`3.0f`),
  `0x8042CF4C = 0x00000000`, `0x8042CFAC = 0xC2480000` (`-50.0f`),
  `0x8042CFB4 = 0x00000000`. All four are still read at run time; no value is
  inlined.
- `0x8031BDD0` holds Shift-JIS part names (`0x80428A48`, `0x80428A4C`,
  `0x80428A50`, `0x8031BD80`, `0x80428A54`, `0x80428A5C`, `0x80428A64`,
  `0x80428A68`, `0x8031BD8C`, `0x80428A70`, `0x80428A78`, `0x80428A7C`) and
  `0x802F3370` the parallel ids `2, 15, 26, 4, 3, 6, 7, 8, 14, 16, 17, 18`.

## Callees left untranslated (called by address)

`0x800F5F10` (CR LF test), `0x800F6110` (arc blend step), `0x80160E80`
(16-bit index), `0x801C1650` (two-vector helper), `0x802BE470` (magnitude of the
helper's delta) and `0x802E1AB4` (`strcmp`). `fn_800f5050` calls
`fn_800f3460` directly instead of re-entering it by address, and `0x802B9280`
(the allocator free) is reused through the module's existing `FN_802B9280`
constant.

## Reuse

The lane reuses `platform::*`, `read_f32`/`write_f32`, `R2_BASE` and the
existing `FN_802B9280` constant. New constants carry the `_D` suffix
(`FN_800F5F10_D`, `FN_800F6110_D`, `FN_80160E80_D`, `FN_801C1650_D`,
`FN_802BE470_D`, `FN_802E1AB4_D`, `TBL_8031BDD0_D`, `TBL_802F3370_D`,
`F_CELL_PITCH_D`, `F_CELL_SIGN_D`, `F_ARC_SLOPE_D`, `F_ARC_ZERO_D`,
`CSV_RECORD_SIZE_D`). New local helpers carry a `_d` suffix
(`call0_f32_ret_d`, `call_f32_u32x4_d`) so that other lanes in this wave cannot
collide with them. No existing const or helper was redefined.

## Verification

- `functions.tsv` sizes: all ten match the re-measured extents; `0x800F64E0` is
  the single prologue artefact (28 tsv / 24 real) and is documented above.
- Scratch copy: `src/`, `Cargo.toml` and `Cargo.lock` were copied to
  `%TEMP%\opencode\w216d`, `work/wave216/laneD_block.rs` was appended to its
  `src/engine/dsys.rs`, and `cargo check --lib` exited `0` with no warning that
  mentions `dsys.rs`. A deliberate type error appended after the block was
  reported at the appended line range, which confirms the block really is
  compiled; the probe was then removed and the copy re-checked to exit `0`.
- Files written: `work/wave216/laneD_block.rs`,
  `ledger/agents/wave216-laneD.csv`, this document. `src/engine/dsys.rs` was
  not modified.
