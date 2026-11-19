# Coverage - Wave 216 lane F (engine::dsys)

## How to read this file

`engine::dsys` reconstructs the `dSys_c` engine start-up together with the
display, text, sprite and resource helpers around it in the main `DOL`. This
lane takes ten bodies from the `0x800F0000` bucket, between `0x800F30C0` and
`0x800F73F0`.

Each row below is one `pub unsafe fn fn_<address>`. The doc block above the body
cites the Ghidra export, the byte count from
`work/ghidra/main-gb/export/functions.tsv`, and a confidence level:

- `high` means every guard, store and call argument is literal in the export;
- `medium` means a field role, a callee identity or a structure layout was
  inferred and is marked `// inferred:` or `// unknown:` in the body.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/dsys.rs` | `engine::dsys` | 10 | 1816 | 5 | 5 |

The lane's bodies live in `work/wave216/laneF_block.rs`; the row list is
`ledger/agents/wave216-laneF.csv`.

## What this pass covers

Three groups share the same `0x54`-byte descriptor records, the `0x1C0`-byte
source-table entries and the `+0x15018` name table:

| address | function | bytes | role |
|---|---|---:|---|
| `0x800F30C0` | `fn_800f30c0` | 200 | resolve the six sub-entries of every record |
| `0x800F3190` | `fn_800f3190` | 72 | store one record's id at `+0x4C` |
| `0x800F5130` | `fn_800f5130` | 468 | parse one CSV field and set the `A`/`B`/`C` flags |
| `0x800F5310` | `fn_800f5310` | 132 | look a name up in the `+0x15018` name table |
| `0x800F56C0` | `fn_800f56c0` | 64 | `+0x1D`/`+0xD1` byte slot of a `0x1C0` entry |
| `0x800F57A0` | `fn_800f57a0` | 20 | `mask & entry[+0x1C8]` |
| `0x800F5830` | `fn_800f5830` | 20 | address of one `0x18`-byte name-table slot |
| `0x800F6110` | `fn_800f6110` | 512 | curve start/end solver (two full bodies) |
| `0x800F6CD0` | `fn_800f6cd0` | 132 | pick a mode, solve the height, store it |
| `0x800F7330` | `fn_800f7330` | 196 | membership probe over 64 id slots |

### 1. Byte sizes against the prologue and the `blr`

Every size was re-checked against `work/dol/main.elf`:

| address | prologue | last `blr` | real extent | `functions.tsv` |
|---|---|---|---|---|
| `0x800F30C0` | `stwu r1,-0x50` | `0x800F3184` | 200 | 200 |
| `0x800F3190` | `stwu r1,-0x10` | `0x800F31D4` | 72 | 72 |
| `0x800F5130` | `stwu r1,-0x50` | `0x800F5300` | 468 | 468 |
| `0x800F5310` | `stwu r1,-0x20` | `0x800F5390` | 132 | 132 |
| `0x800F56C0` | none | `0x800F56E0` / `0x800F56FC` | 64 | 64 |
| `0x800F57A0` | none | `0x800F57B0` | 20 | 20 |
| `0x800F5830` | none | `0x800F5840` | 20 | 20 |
| `0x800F6110` | none (head is a compare/tail jump) | `0x800F6314` | **520** | 512 |
| `0x800F6CD0` | `stwu r1,-0x20` | `0x800F6D50` | 132 | 132 |
| `0x800F7330` | none (opens with `mtspr ctr,8`) | `0x800F73F0` | 196 | 196 |

Nine of the ten match exactly; none is a 16/20-byte prologue artefact.

`0x800F6110` is the exception. Its head is only four instructions
(`lfs f3,4(r4)`, `lfs f0,4(r5)`, `fcmpu`, `bne`) and each of the two arms is a
complete body with its own `stwu`/epilogue: the equal-height arm runs
`0x800F6130..0x800F621C` and the unequal-height arm `0x800F6220..0x800F6314`.
The last `blr` is therefore at `0x800F6314`, i.e. the real extent is
`0x208` = 520 bytes, eight bytes more than the tsv's 512. The tsv value is kept
in the `# Original` line (the lane checker compares against it) and the real
extent is stated in the doc block and here.

### 2. The `0x802DCD68/6C/74` "pairs" are register-save stubs

`0x800F30C0`, `0x800F5130` and `0x800F5310` all begin by calling one of
`0x802DCD68`/`0x802DCD6C`/`0x802DCD74`, which Ghidra renders as a
`undefined8`-returning function whose two words the export then splits into a
base and a context. The disassembly shows what they really are:

| stub | body | live values |
|---|---|---|
| `0x802DCD68` | `_savegprlr_24` | none; `r3`/`r4` are untouched |
| `0x802DCD6C` | `_savegprlr_26` | none; `r3`/`r4` are untouched |
| `0x802DCD74` | `_restgprlr_27` | none; `r3`/`r4` are untouched |

so the returned "pair" is simply the argument pair the caller already placed in
`r3`/`r4`. The three bodies are reconstructed with explicit parameters, and the
corresponding tail stubs (`0x802DCDB4`/`B8`/`C0`) are replayed through
`platform::call0`/`call1`. The call-site evidence agrees: `0x800F39D0` calls
`0x800F5130(param_1, iVar2, &local_38)` where `param_1` is the manager with the
`+0x10` counter and `+0x15018` table.

### 3. `0x800F5310` and `0x800F30C0` share the name table

`0x800F5310(manager, name)` walks `0x18`-byte slots from `manager + 0x15018`
with `0x802E1AB4`, returning the slot's `+0x15024` word or `-1`.
`0x800F30C0` builds a ten-byte key with `0x800F9310` (`'R'` + four bytes of the
record's `+8` name + four bytes of the sub-entry's `+4` name), asks
`0x800F5310`, and on `-1` rebuilds the key with the two names swapped before
storing the answer at `record + 0x34 + i * 4`. Both the key builder and the
lookup are literal; only the meaning of the two name halves is inferred.

### 4. `0x800F5130` is a CSV field reader

The three SDA strings compared by the body are at `r13 - 0x6BF4`,
`r13 - 0x6BF0` and `r13 - 0x6BEC` = `0x80428A8C`, `0x80428A90`, `0x80428A94`,
and the image bytes there are the NUL-terminated `"A"`, `"B"`, `"C"`. A match
ORs `1`, `2` or `4` into the current slot's `+0x1502C` byte, but only when the
slot's `+0x15018` byte is not `'0'`. The cursor arithmetic (`*param_3`) is
reproduced step for step, including the quoted-mode test that uses the *first*
byte of the call rather than the current one.

### 5. `0x800F6110` chains two full bodies through one `fcmpu`

Both arms compute `dx`, `dy`, `dz`, `sqrt(dx*dx + dz*dz)` and store the same
four fields (`+0x118`, `+0x114`, `+0xEC`, `+0x10C`); only the `+0x114`
expression differs:

- equal heights: `-(2 * param_1) / param_6`;
- unequal heights: `-2 * (param_1 * param_6 - dy) / (param_6 * param_6)`.

`0x802BE470` is a double-precision square root followed by `frsp`; since a
PowerPC single already lives in the double-format encoding of its value, no
conversion instruction appears and the reconstruction passes the `f32` sum
through `call_f64_arg_ret` and rounds back with `as f32` (the same treatment
`engine::display` gives `0x802BE470`). The `ldexpf` blocks in the export are the
`psq_st`/`psq_l` saves of `f28..f31`; the three `stfs` copies at
`r1 + 8..r1 + 0x10` are dead.

### 6. `0x800F6CD0` forwards an `f1` result between two calls

`0x800F6B50` returns the interpolated height in `f1` and `0x800F6C40` consumes
that same `f1` (its export prints it as a leading `double`). The reconstruction
adds two small lane helpers: `call_u32x4_f32_ret_f` (four words in, float out)
and `call_f32_u32x2_f` (float in `f1`, two words in `r3`/`r4`).

## Deliberately skipped

None. All ten bodies were reconstructed.

## Field notes

- `0x800F7330`'s image loop is a `ctr` loop with a dead `r5` accumulator that
  advances by seven; the reconstruction uses the equivalent nested
  `8 blocks x 8 slots` form, which keeps the comparison order
  (`+0xC + k * 0x14`, then `+0xA0` per block) identical.
- The `0x800F5130` field cell is 72 bytes in the image; the Rust array is 73
  bytes so the terminating NUL stays inside the array when a field fills it.
- `0x800F30C0`'s `+0x30` sentinel is never written here: the six resolved words
  occupy `+0x34..+0x48`, exactly the slots `0x800F2B60` pre-fills with `-1`.
- The key-cell loop in `0x800F30C0` is `0xC0` records deep, not the record
  count stored in the manager; the immediate `192` is used directly.

## Verification

- Every address and byte size matches
  `work/ghidra/main-gb/export/functions.tsv` (`800f30c0` 200, `800f3190` 72,
  `800f5130` 468, `800f5310` 132, `800f56c0` 64, `800f57a0` 20, `800f5830` 20,
  `800f6110` 512, `800f6cd0` 132, `800f7330` 196); the `800f6110` real extent
  of 520 bytes is documented above.
- Every evidence file exists at the cited path.
- Ten `pub unsafe fn fn_<address>` bodies, one per ledger row.
- No helper or constant defined here duplicates an existing one; new constants
  carry the lane suffix `_F` and `FN_800F57A0_F`, `FN_800F9530_F`,
  `FN_802DCDB4`, `FN_802DCDC0` are reused as they already stand.
- `cargo check --lib` in a scratch copy of the crate with the lane block
  appended to `src/engine/dsys.rs` exits `0` with no warning from the new code.
- `node tools/check-lane-csv.mjs wave216` reports `OK` with 10 rows for this
  lane in that same scratch tree.
