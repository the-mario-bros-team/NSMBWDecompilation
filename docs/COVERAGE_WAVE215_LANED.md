# Wave 215 lane D coverage (`engine::dsys`)

Slice: the ten `engine::dsys` bodies at `0x800F18B0`, `0x800F1940`,
`0x800F1980`, `0x800F1A00`, `0x800F1AA0`, `0x800F1AB0`, `0x800F1B60`,
`0x800F1B70`, `0x800F1B80` and `0x800F1B90` (708 bytes). All ten are
translated; nothing was deferred.

Bodies live in `work/wave215/laneD_block.rs` and are appended to
`src/engine/dsys.rs` by the merge step; ledger rows are in
`ledger/agents/wave215-laneD.csv` (`module = engine::dsys`).

This is the small registry family that sits just above the `dSys` start-up:
one static metric record with a registered destructor, a 0x14-byte named node
type with its constructor/destructor, the binder that hangs a 0x94-byte child
off such a node, three 8-byte forwarders into that child, and the 10-byte tag
equality test used by the binder.

## Translated

| Address | Bytes | Rust body | Summary |
| --- | --- | --- | --- |
| `0x800F18B0` | 112 | `fn_800f18b0` | One-shot initialiser of the 0x24-byte record at `0x8031BA48`: stores the `.data` floats `2160.0` / `-30.0` / `-478.0` at `+0x18`/`+0x1C`/`+0x20`, the word `0x8042CF6C` (`0x17`) at `+0xC`, registers `(object 0, destructor 0x800F1920)` on the node `0x80373C80` through `0x802DC780`, and copies `0x8042CF84` (`0x26`) into `0x8042A108`. |
| `0x800F1940` | 60 | `fn_800f1940` | Node constructor: `node[+0xC] = 0`, then a ten-byte bounded copy of the name into `node+0` through `0x802E19F8`; returns the node. |
| `0x800F1980` | 128 | `fn_800f1980` | Node destructor: null node is a no-op; otherwise release `child = node[+0xC]` (`0x80168D80(child+0x58, 0)` then `0x802B9280(child)`) and, when `param_2 > 0`, free the node itself. Returns the node. |
| `0x800F1A00` | 152 | `fn_800f1a00` | Allocate `0x14` bytes with `0x802B9210`, name it from the first resource entry (`(*root)[+8]`, zero offset -> null name) via `fn_800f1940`, then call the binder — unconditionally, even when the allocation failed. |
| `0x800F1AA0` | 8 | `fn_800f1aa0` | `li r4,1` + tail branch to `fn_800f1980`: destroy and free. |
| `0x800F1AB0` | 168 | `fn_800f1ab0` | Binder: `count = 0x800DFD20(root)`; for each index fetch `0x800DFDD0(root, i)`, resolve `entry + entry[+8]` (0 when the offset is 0) and break when `0x802E1BD0(entry, *0x804288C0, 4) == 0`; then `child = 0x800F1C60(context, root, param_4)` and `node[+0xC] = child`. |
| `0x800F1B60` | 8 | `fn_800f1b60` | `lwz r3,0xC(r3)` + `b 0x800F1D10`: child step. |
| `0x800F1B70` | 8 | `fn_800f1b70` | `lwz r3,0xC(r3)` + `b 0x800F1E10`: child submit. |
| `0x800F1B80` | 8 | `fn_800f1b80` | `lwz r3,0xC(r3)` + `b 0x800F1E90`, with the caller's `r4` still live and forwarded as the second argument. |
| `0x800F1B90` | 56 | `fn_800f1b90` | 10-byte name equality: `0x802E1BD0(param_2, param_1, 10)` folded with `cntlzw(x) >> 5`, i.e. returns `1` exactly when the comparison returned zero. |

## Sizes and extents

Every `# Original` block keeps the canonical `main-gb` `functions.tsv` value,
and each one was re-checked against the prologue and the closing `blr`/`b` in
`work/dol/main.elf`:

- `0x800F18B0`: `stwu r1,-0x20` / `mfspr r0,256` at `+0`, `bclr 20,0` at
  `0x800F191C` — the instruction ends exactly at `0x800F1920`; 112 bytes, no
  padding. The `LAB_800f1920` block that Ghidra prints inside this export is a
  *separate* untabled function at `0x800F1920..0x800F1940`, outside the 112
  bytes; it is translated as `fn_800f1920` (below) and is not a ledger row.
- `0x800F1940`: `bclr 20,0` at `0x800F1978`, ends at `0x800F197C`; 60 bytes
  (the `OP_0` at `0x800F197C` is inter-function padding).
- `0x800F1980`: `bclr 20,0` at `0x800F19FC`, ends at `0x800F1A00`; 128.
- `0x800F1A00`: `bclr 20,0` at `0x800F1A94`, ends at `0x800F1A98`; 152.
- `0x800F1AA0`: `li r4,1` at `+0`, `b 0x800F1980` at `+4`; 8 bytes.
- `0x800F1AB0`: `bclr 20,0` at `0x800F1B54`, ends at `0x800F1B58`; 168.
- `0x800F1B60` / `0x800F1B70` / `0x800F1B80`: `lwz` + tail `b`; 8 bytes each
  (the four `OP_0` bytes after each are padding).
- `0x800F1B90`: `bclr 20,0` at `0x800F1BC4`, ends at `0x800F1BC8`; 56.

The four 8-byte entries were decoded literally from the raw words:
`8063000c` = `lwz r3,0xC(r3)` and `480001ac` / `4800029c` / `4800030c` are the
three `b` forms to `0x800F1D10` / `0x800F1E10` / `0x800F1E90`; `38800001` +
`4bfffedc` in `0x800F1AA0` are `li r4,1` + `b 0x800F1980`. None of the ten
sizes is a prologue artefact.

## Evidence notes

- `0x800F1AB0` opens with `addi r11,r1,0x20` / `bl 0x802DCD70` and closes with
  `bl 0x802DCDBC`. `0x802DCD70` is the outlined `__savegprlr_26`: it stores
  `r26..r31` into the caller's frame and never writes `r3`/`r4`
  (`work/ghidra/main-gb/export/802dcd70_FUN_802dcd70.c`). The "pair" the module
  header describes is therefore just the incoming `r3`/`r4`; the body keeps the
  module's `platform::call_u64` reading (identical values, matching the in-file
  precedent of `fn_800eb040`) and says so at the use site. `0x802DCDBC` is the
  matching `__restgprlr_26` and is emitted through `platform::call0`.
- `0x800F1B80` drops the callee's `r4` in the Ghidra export because the
  decompiler only sees one live argument; the raw words
  (`8063000c` / `4800030c`) forward the whole register state, and
  `FUN_800f1e90`'s own export takes `(child, arg)`, so the body forwards the
  incoming `param_2`.
- `0x800F18B0` also writes the three floats into the caller's frame
  (`stfs f2/f1/f0,8/0xC/0x10(r1)` at `0x800F18EC..0x800F18F4`); those are dead
  local stores and are omitted. The float and word sources are re-read at run
  time, as the original does; their image values were read from
  `work/dol/main.elf` and are recorded in the doc blocks
  (`0x8042CEF8 = 0x45070000 = 2160.0`, `0x8042CEFC = 0xC1F00000 = -30.0`,
  `0x8042CF00 = 0xC3EF0000 = -478.0`, `0x8042CF6C = 0x17`,
  `0x8042CF84 = 0x26`).
- `0x800F1980`'s `addic. r3,r31,0x58` / `beq` tests the computed sub-object
  address (zero only through wrap-around); the Rust body keeps that test with
  `wrapping_add` instead of silently dropping it.
- `0x800F1920` (auxiliary, not a ledger row) tail-branches to `0x802DCA98` with
  `(0x8031BA48, 0x80014200, 36, 1)`; `0x802DCA98` computes
  `base + stride * count` and walks backwards, so exactly one 36-byte element
  is destroyed. `0x80014200` with a negative flag only returns its argument —
  the chain entry is therefore a no-op at shutdown, but it is reproduced
  exactly.

## Callees left untranslated (called by address)

`0x800F1C60` (0x94-byte child build), `0x800F1D10` / `0x800F1E10` /
`0x800F1E90` (child step/submit/query) and `0x802DCA98`. `0x802E19F8`
(bounded name copy), `0x802E1BD0` (length-bounded compare), `0x800DFD20` /
`0x800DFDD0` (resource list count/lookup), `0x802B9210` (allocator),
`0x802B9280` (free), `0x80168D80`, `0x802DC780`, `0x802DCD70` and
`0x802DCDBC` are already represented in this module and are reused by their
existing `FN_*` constants where one exists.

## Reuse

The lane reuses `platform::*`, `read_f32`, `SDA_BASE`, and the existing
`FN_80168D80`, `FN_802B9210`, `FN_802B9280`, `FN_800DFD20`, `FN_800DFDD0`,
`FN_802E1BD0`, `FN_802DC780`, `FN_802DCD70` and `FN_802DCDBC` constants. New
constants carry the `_D` suffix (`FN_802E19F8_D`, `FN_802DCA98_D`,
`FN_800F1C60_D`, `FN_800F1D10_D`, `FN_800F1E10_D`, `FN_800F1E90_D`,
`FN_800F1920_D`, `FN_80014200_D`, `REC_8031BA48_D`, `NODE_80373C80_D`,
`F_METRIC_A_D`/`B`/`C`, `W_METRIC_MODE_D`, `W_METRIC_STATE_D`,
`G_METRIC_STATE_D`, `TAG_RESOURCE_D`, `NODE_CHILD_OFF_D`,
`CHILD_SUB_OFF_D`, `NODE_SIZE_D`). No existing const or helper was redefined.
`fn_800f1ab0` / `fn_800f1b60` / `fn_800f1b70` / `fn_800f1b80` call the
translated lane bodies directly instead of re-entering them by address.

## Verification

- `functions.tsv` sizes: all ten match the re-measured extents (table above).
- Scratch copy: `src/`, `Cargo.toml` and `Cargo.lock` were copied to
  `%TEMP%\opencode\wave215-lanD`, `work/wave215/laneD_block.rs` was appended to
  its `src/engine/dsys.rs`, and `cargo check --lib` exited `0`. A deliberate
  type error appended after the block was reported at `dsys.rs:11427`, which
  confirms the block really is compiled; the probe was then removed and the
  copy re-checked to exit `0`. No warning mentions `dsys.rs`.
- Files written: `work/wave215/laneD_block.rs`,
  `ledger/agents/wave215-laneD.csv`, this document. `src/engine/dsys.rs` was
  not modified.
