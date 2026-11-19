# Coverage - Wave 99 (`game::stage`, the `0x80090000` bucket)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The `evidence`
column of `ledger/agents/wave99-stage.csv` is the Ghidra C export each body was
written from. Field roles the export only hints at stay marked `observed`,
`inferred` or `unknown` in the body.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 30 | 5136 | 28 | 2 |

No body in this pass is `low` confidence and none is `partial`; every row is
`translated`.

## What this pass covers

- **The two state-record registrars** (`0x8009A320`, `0x8009DD60`). Between them
  they are 3816 of the 5136 bytes translated here, and they are the same
  five-step block repeated 7 and 11 times: nine `.rodata` words are copied into
  `record + 0xC..+0x2C` with the permutation `+0x18, +0x1C, +0x20, +0x0C,
  +0x10, +0x14, +0x00, +0x04, +0x08`; the record's name is stored by
  `0x8015F7C0`; `+0x00` receives the state vtable (`0x803125D8` for `dEnBoss`,
  `0x80313088` for `dEnBossKoopaJrBase`); `+0x30` receives a type-info
  descriptor; and `0x802DC780(record, dtor, record - 0xC)` chain-registers the
  record. The `.rodata` source blocks are `0x80312418`/`0x80312D38` with a
  `0x24` stride, the `.data` records are `0x803576B4`/`0x80357874` with a `0x40`
  stride. Only the loop shape is a compaction: the export prints every block
  unrolled and no block differs.
- **The `0x800964A0` dispatch ladder.** `0x800A29C0` is handed the `+0x514`
  sub-object and the three-word block `{0x80311E6C, request, target}`; its answer
  (`1`, `2`, `3`) selects a run on the object's `+0x60` state vtable. Answer `1`
  branches again on the candidate's `+0x38C` byte, `mode == 0` runs the slot
  pairs, `mode == 1` swaps them for the event helpers `0x80096CA0`/`0x80096D00`,
  and answer `3` reaches the same `+0x24C`/`+0x250` slots through the two
  wrappers below. Thirteen distinct slots are pinned literally.
- **The two slot wrappers** (`0x80096780`, `0x80096790`). Both exports print a
  single indirect jump with **no** arguments. The arity is recovered from two
  call sites (`0x800964A0` and `0x80039B40`), which both call
  `FUN_80096780(param_1, iVar2)`, and from `0x800964A0`'s answer-1 path, which
  reaches the identical `+0x24C`/`+0x250` slots directly with the same two
  arguments. They are marked `medium` because the export itself does not show
  the forwarded registers.
- **The course-actor teardown** (`0x8009AF40`). One `__destroy_arr` of a
  2 × `0x114`-byte array at `+0xA08`, the two `0x80329958` interface
  sub-objects at `+0x8E0`/`+0x7B8`, six node resets (`+0x72C`, `+0x700`,
  `+0x6C0`, `+0x67C`, `+0x644`, `+0x604`), the nested `0x800985B0` teardown and
  an optional `0x80162920` free. It is the larger sibling of the wave-88
  `fn_800b7db0` chain and reuses its constants.
- **The interface/state reset** (`0x8009B540`). The `+0x394` interface is asked
  to reset itself through its own vtable slot `+0x10`, then the actor's `+0x60`
  vtable slot `+0x3FC` runs; the function returns the constant `1`.
- **The descriptor and accessor pair** (`0x8009E730`, `0x8009E740`,
  `0x8009F6D0`, `0x8009F6F0`). Two `getType()`-style descriptors
  (`0x803576F4` and the already named `0x803576B4`), the `0x802F0508` word
  table lookup driven by the argument's `+0x00` id, and the eight-entry clamp.
- **Three type-info stub runs** (`0x8009ACA0..0x8009AD20`,
  `0x8009EAF0`/`0x8009EB00`, `0x8009F7C0..0x8009F830`). Nineteen 12-byte
  `return &DAT_80377A04;` bodies. Wave 84 deferred the first nine as
  "no new evidence"; this pass lands them because `0x8009DD60` calls all nine by
  address in record order, so they are live members of the `Wm`/`dEnBoss` type
  family described by [`W84_TYPE_INFO`], not dead duplicates.

Address range: `0x800964A0..0x8009F830` (seven runs, 30 bodies, 5136 bytes).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800964A0` | `fn_800964a0` | 604 | high |
| `0x80096780` | `fn_80096780` | 16 | medium |
| `0x80096790` | `fn_80096790` | 16 | medium |
| `0x8009A320` | `fn_8009a320` | 1500 | high |
| `0x8009ACA0` | `fn_8009aca0` | 12 | high |
| `0x8009ACB0` | `fn_8009acb0` | 12 | high |
| `0x8009ACC0` | `fn_8009acc0` | 12 | high |
| `0x8009ACD0` | `fn_8009acd0` | 12 | high |
| `0x8009ACE0` | `fn_8009ace0` | 12 | high |
| `0x8009ACF0` | `fn_8009acf0` | 12 | high |
| `0x8009AD00` | `fn_8009ad00` | 12 | high |
| `0x8009AD10` | `fn_8009ad10` | 12 | high |
| `0x8009AD20` | `fn_8009ad20` | 12 | high |
| `0x8009AF40` | `fn_8009af40` | 304 | high |
| `0x8009B540` | `fn_8009b540` | 84 | high |
| `0x8009DD60` | `fn_8009dd60` | 2316 | high |
| `0x8009E730` | `fn_8009e730` | 12 | high |
| `0x8009E740` | `fn_8009e740` | 12 | high |
| `0x8009EAF0` | `fn_8009eaf0` | 12 | high |
| `0x8009EB00` | `fn_8009eb00` | 12 | high |
| `0x8009F6D0` | `fn_8009f6d0` | 24 | high |
| `0x8009F6F0` | `fn_8009f6f0` | 20 | high |
| `0x8009F7C0` | `fn_8009f7c0` | 12 | high |
| `0x8009F7D0` | `fn_8009f7d0` | 12 | high |
| `0x8009F7E0` | `fn_8009f7e0` | 12 | high |
| `0x8009F7F0` | `fn_8009f7f0` | 12 | high |
| `0x8009F800` | `fn_8009f800` | 12 | high |
| `0x8009F810` | `fn_8009f810` | 12 | high |
| `0x8009F820` | `fn_8009f820` | 12 | high |
| `0x8009F830` | `fn_8009f830` | 12 | high |

## Deliberately skipped

The bucket still holds 22 untranslated functions after this pass (52 at the
start). Skipped on purpose:

- **The `0x802DCD74` split-return family** (`0x80090440`, `0x800925F0`,
  `0x80098900`). All three open with `uVar = FUN_802dcd74();` and then consume
  both halves of the `undefined8` (`(int)((ulonglong)uVar >> 0x20)`); the
  halves cannot be separated from the C text. Waves 93, 94 and 98 document the
  same blocker for the `0x800Cxxxx` band.
- **The GQR-quantised math bodies** (`0x80093F10`, `0x800955F0`, `0x800957B0`,
  `0x800963B0`, `0x80097250`, `0x800978C0`). Each converts `s16`/`u16` values
  through `unaff_GQR0`/`unaff_GQR3` `ldexpf` branches that Ghidra printed as
  unreachable blocks, or through the `CONCAT44(0x43300000, …) ^ 0x80000000`
  word-to-double idiom. The quantisation order and the `r2-0x6xxx` constants are
  not visible, so no honest body can be written.
- **The two stack-vtable builders** (`0x80093530`, `0x80093A50`). Both allocate a
  `0x288`/`0x208`-byte frame, fill a set of `0x30`-byte matrices and then
  install a vtable whose entries are function pointers taken from locals the
  export names `undefined4`. The locals are never written with visible symbol
  addresses, so the table contents would have to be invented.
- **`0x80097A20`.** Its four-call matrix compose
  (`0x8012DE80`/`0x801C0BD0`/`0x801C0540`/`0x8016F130`) casts values to `double`
  that the export never declares as floats; the argument order would be a guess.
- **`0x80097170`, `0x800976B0`.** Both call a callee with arguments dropped
  (`0x80097250` with the receiver, `0x80097700` with both), and the callees are
  themselves skipped above, so neither call can be grounded.
- **`0x80099F80`, `0x80099FA0`, `0x80099FE0`, `0x8009A000`.** Four 16-byte
  indirect jumps of the same shape as the two wrappers this pass *did*
  translate, but these four appear in no call site anywhere in the export
  corpus (`FUN_<addr>(` has zero hits outside their own bodies). Their arity is
  therefore unproven and the slots (`+0x16C`, `+0x170`) would have to be called
  with a guessed argument list.
- **`0x8009AEC0`.** `FUN_802d7aa0();` is printed with no arguments although its
  own export takes `int *param_1` and constructs through it; the destination is
  the dropped register, and the only pointer in scope (`param_1`) is an
  inference, not evidence.
- **`0x8009BAD0`.** The two virtual `+0x70` calls fill stack slots that Ghidra
  aliases across `local_28`/`local_2c`/`auStack_30`, so which float belongs to
  which call cannot be decided; it also reads `unaff_r2-0x6E48` and the
  `0x8042C1A0` table.
- **`0x80096700`.** A 4-byte thunk of `0x800967F0`, which is already translated
  in this file; per the wave-84/98 precedent, thunks of translated bodies are
  not counted twice.
- **`0x8009B070`.** The `KoopaJr` model/effect builder needs the
  `r13-0x7C40` string block and the roles of the `+0x600`/`+0x604`/`+0x608`
  dispatch chain before its ~20 stores can be named; it was not attempted in
  this pass.

None of these is counted as translated in the CSV.

## Verification

- Every address and byte size matches `functions.tsv` (30 rows, 5136 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`; the 30
  addresses were chosen from the 52 open entries of the `0x80090000` bucket.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs` (the two errors it reports are in `src/audio/ax.rs`,
  another lane's file).
- `node tools/check-lane-csv.mjs wave99` prints `OK`; the wave-99 stage rows are
  clean.
