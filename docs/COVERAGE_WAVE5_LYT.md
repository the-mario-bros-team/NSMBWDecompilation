# Coverage — wave 5 NW4R `lyt` fourth pass (`0x8017F250..0x8018ABE0`)

## How to read this file

- **Scope** — the fourth `lyt` translation pass. It adds 25 functions to the
  semantic module `src/ui/lyt.rs` and creates no staging module.
- **Addresses** are virtual `sys/main.dol` addresses (image base
  `0x80004000`), not file offsets.
- **Evidence** is the Ghidra C export under
  `work/ghidra/main-gb/export/`; each row names its file. Data constants and
  vtable words were read from `work/dol/main.elf`.
- **Status** follows `docs/RUST_STYLE.md`:
  `translated` = full control flow, every call accounted for;
  `partial` = flow and major effects present, one operand not literal;
  `stub` = address/role only.
- **Confidence** measures the evidence, not code quality. `medium` marks a
  function whose bytes and control flow are exact but whose signature or
  object role was recovered from a prologue/register convention.
- Start with the **summary**, then the **function table**. The **cluster
  notes** explain how the bodies connect to waves 2–4.

## Summary

| metric | value |
|---|---:|
| functions added to `src/ui/lyt.rs` | 25 |
| bytes translated | 4,360 |
| `translated` / `partial` / `stub` | 25 / 0 / 0 |
| high confidence | 18 |
| medium confidence | 7 |
| new staging modules | 0 |
| files written | `src/ui/lyt.rs`, this document, `ledger/agents/wave5-lyt.csv` |

Address span: `0x8017F250..0x8018ABE0`. All 25 functions are at least 16
bytes and outside the runtime-lane exclusions.

## Function table

| address | Rust function | role | status | conf. | evidence |
|---|---|---|---|---|---|
| `0x8017F250` | `fn_8017f250` | bind one texture-pattern record into a GX texture object | translated | medium | `8017f250_FUN_8017f250.c` |
| `0x8017F390` | `fn_8017f390` | construct `nw4hbm::lyt::DrawInfo` | translated | high | `8017f390_FUN_8017f390.c` |
| `0x8017F450` | `fn_8017f450` | construct `nw4hbm::lyt::Group` and load its texture list | translated | medium | `8017f450_FUN_8017f450.c` |
| `0x8017F610` | `fn_8017f610` | release a group's child list, optional free | translated | medium | `8017f610_FUN_8017f610.c` |
| `0x8017F980` | `fn_8017f980` | tear down `nw4hbm::lyt::Layout` | translated | medium | `8017f980_FUN_8017f980.c` |
| `0x801836B0` | `fn_801836b0` | construct `nw4hbm::lyt::Pane` | translated | high | `801836b0_FUN_801836b0.c` |
| `0x801837D0` | `fn_801837d0` | destroy `nw4hbm::lyt::Pane` | translated | medium | `801837d0_FUN_801837d0.c` |
| `0x801840E0` | `fn_801840e0` | notify children and material of a pane | translated | high | `801840e0_FUN_801840e0.c` |
| `0x80184270` | `fn_80184270` | remove matching children from a pane | translated | medium | `80184270_FUN_80184270.c` |
| `0x80184310` | `fn_80184310` | insert a node into the pane's second child list | translated | high | `80184310_FUN_80184310.c` |
| `0x80184350` | `fn_80184350` | find a child by key, material fallback | translated | high | `80184350_FUN_80184350.c` |
| `0x801845A0` | `fn_801845a0` | compute picture flip/wrap offsets | translated | high | `801845a0_FUN_801845a0.c` |
| `0x801848A0` | `fn_801848a0` | apply one texture record through virtual `+0x68` | translated | high | `801848a0_FUN_801848a0.c` |
| `0x80184C00` | `fn_80184c00` | conditional release wrapper | translated | high | `80184c00_FUN_80184c00.c` |
| `0x80185CC0` | `fn_80185cc0` | grow the `u16` glyph-index buffer | translated | high | `80185cc0_FUN_80185cc0.c` |
| `0x80185D40` | `fn_80185d40` | free the glyph-index buffer | translated | high | `80185d40_FUN_80185d40.c` |
| `0x80185E00` | `fn_80185e00` | fill glyph indices and write the terminator | translated | high | `80185e00_FUN_80185e00.c` |
| `0x801868C0` | `fn_801868c0` | forward an argument to every texture, then children | translated | high | `801868c0_FUN_801868c0.c` |
| `0x80188860` | `fn_80188860` | validate a `BRFNT` section header | translated | high | `80188860_FUN_80188860.c` |
| `0x8018A400` | `fn_8018a400` | select one of four font encoders | translated | high | `8018a400_FUN_8018a400.c` |
| `0x8018A730` | `fn_8018a730` | construct `nw4hbm::ut::ResFont` | translated | high | `8018a730_FUN_8018a730.c` |
| `0x8018A7D0` | `fn_8018a7d0` | load a `BRFNT` into a `ResFont` | translated | high | `8018a7d0_FUN_8018a7d0.c` |
| `0x8018A920` | `fn_8018a920` | convert `BRFNT` section offsets to pointers | translated | high | `8018a920_FUN_8018a920.c` |
| `0x8018AA30` | `fn_8018aa30` | construct the `nw4hbm::ut::Font` base | translated | high | `8018aa30_FUN_8018aa30.c` |
| `0x8018ABE0` | `fn_8018abe0` | map a character code to a glyph index | translated | medium | `8018abe0_FUN_8018abe0.c` |

All evidence paths are relative to `work/ghidra/main-gb/export/`.

## Cluster notes

### Class identification (RTTI in `main.elf`)

Every vtable word used by this pass points at a RTTI string:

| vtable | RTTI string | class |
|---|---|---|
| `0x8032B1E0` | `nw4hbm::lyt::DrawInfo` | draw info |
| `0x8032B210` | `nw4hbm::lyt::Group` | draw-info group |
| `0x8032B238` | `nw4hbm::lyt::Layout` | layout |
| `0x8032B310` | `nw4hbm::lyt::Pane` | pane base |
| `0x8032C710` | `nw4hbm::ut::ResFont` | resource font |
| `0x8032C800` | `nw4hbm::ut::Font` | font base |

Each table starts with an eight-byte header (RTTI pointer, zero), so the first
method is at `+0x08`. That explains the smallest virtual slot used here.

### `DrawInfo` and layout teardown (`0x8017F250..0x8017F980`)

`fn_8017f250` is the texture binder used by the `RLTS`/`RLTP` apply paths. It
fixes up a texture-pattern resource lazily (`0x802281D0`), selects
`entry = entries + (frame % count) * 8`, and then either calls
`0x801C6D90` (no palette) or `0x801C6FA0` plus `0x801C7160` (palette). The
final `0x801C6FF0` programs the LOD fields. The `mipmap` flag is the
instruction-level `a != b` test on bytes `+0x21`/`+0x22` of the record.

`fn_8017f390` builds a `DrawInfo`; `fn_8017f450` builds a `Group` and asks the
resource accessor's virtual `+0x3C` to resolve each 16-byte record before
linking a 12-byte node into the group list. `fn_8017f610` and `fn_8017f980`
are the corresponding teardown walks.

### `Pane` and children (`0x801836B0..0x801868C0`)

`fn_801836b0` constructs the two circular child lists that the rest of the
cluster uses:

```text
first list:  count +0x10, sentinel +0x14
second list: count +0x1C, sentinel +0x20
```

`fn_801837d0` walks the first list, `fn_801840e0` notifies it and the
material, `fn_80184270` filters/removes from the second list,
`fn_80184310` inserts into the second list, and `fn_80184350` looks a child
up by key with a material fallback. `fn_801868c0` forwards an argument to
each texture object before invoking `fn_80184270` with the same key.

`fn_801845a0`, `fn_801848a0` and `fn_80184c00` are the picture-texture
helpers: flip/wrap offsets, one texture apply through virtual `+0x68`, and the
shared conditional-release wrapper.

### Fonts (`0x80185CC0..0x8018ABE0`)

`fn_80185cc0`/`fn_80185d40`/`fn_80185e00` manage the `u16` glyph-index buffer
at `+0xD4`/`+0xF8`. `fn_80188860`, `fn_8018a920` and `fn_8018a7d0` parse the
`BRFNT` sections (`RFNU`, `FINF`, `RFNT`, `TGLP`, `CWDH`, `CMAP`, `GLGR`), and
`fn_8018aa30`/`fn_8018a730`/`fn_8018a400` build the `ut::Font` object and pick
its text encoder. `fn_8018abe0` maps a character code through the glyph-group
chain and stores the result at `FINF + 2`.

## Signature recovery

Five bodies enter through a CRT register-save helper, which Ghidra prints as
a call returning `r3:r4`:

| helper | saves | effect in this pass |
|---|---|---|
| `0x802DCD64` | `r23..r31` | `fn_8017f450` args are exactly `r3..r5` |
| `0x802DCD70` | `r26..r31` | `fn_8017f610`, `fn_8017f980`, `fn_801837d0`, `fn_80184270` are `(this, flag/key)` |

Those functions therefore get an explicit `this` first argument even though
Ghidra prints `void`. The mapping follows the convention already recorded for
`0x802DCD70`/`0x802DCD74` in `src/game/stage.rs` and `src/game/scene.rs`.

## Unknowns and limits

1. **`0x8018AEA0` is not translated.** `fn_8018abe0` calls it by address with
   its range node and character code; its three mapping modes remain unknown.
2. **`0x8017F250` field roles.** The operand order of the GX calls is exact,
   but the record field names (image, format, wrap, filter) are inferred from
   the callee's stores, not from symbols.
3. **`DrawInfo` fields at `+0x34..+0x4C`.** The values `0.0`/`1.0` are verified
   in `main.elf`; their meaning (colour/alpha defaults) is not proven.
4. **Heap handle.** `0x80378008` holds the graphics heap object. The raw
   instruction at `0x8017F4E8` loads the value (`lwz r3,-0x7FF8(r31)`), so the
   Rust passes `read_u32(0x80378008)`, not its address.
5. **`BRFNT` version check.** `fn_80188860` compares the full 32-bit version
   argument against the `u16` at `+0x06`; the two call sites pass `0x104` or
   `0x102`.
6. **List ownership.** As in waves 2 and 4, no function here allocates or frees
   a node except through the explicit `0x801D5540`/`0x801D5550` calls; node
   lifetime otherwise remains the caller's job.

## Verification

```text
cargo check
# Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.49s

node tools/doc-lint.mjs src
# {"ok": true, "files": 68, "violations": 0, "byRule": {}}
```

The crate check completed with 0 errors and no diagnostic from
`src/ui/lyt.rs`. As an extra check, `src/platform.rs`, `src/ui/lyt.rs` and a
minimal `lib.rs`/`ui/mod.rs` were copied to a scratch crate outside the
repository and `cargo check` finished there with 0 errors and 0 warnings.

Only `src/ui/lyt.rs`, this document and `ledger/agents/wave5-lyt.csv` were
written. `src/lib.rs`, every `mod.rs`, `Cargo.toml`, `src/platform.rs` and all
other lanes' files were left untouched.
