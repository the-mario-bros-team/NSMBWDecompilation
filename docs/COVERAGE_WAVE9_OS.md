# Coverage wave 9 — OS band, `0x80240000..0x80260000` (`runtime::os`)

## How to read this file

This page records **one wave** of the `main.dol` reconstruction: wave 9, the
`runtime::os` lane over the upper half of the `0x80200000..0x80260000`
address lane. It answers four questions:

1. **What was in scope?** — the *Scope* section: the address window, the
   eligibility rule, and what was already translated.
2. **What did this wave add?** — the *Function table*: one row per function,
   with address, size, Rust name, plain-English role, status, confidence and
   the Ghidra evidence file.
3. **What do the functions belong to?** — *What this window is* and
   *Clusters*. This sub-band contains no HCI, L2CAP or RFCOMM code; it is the
   generic object layer that shipped between the console code and the render
   code, and it is the layer the console/exception path logs through.
4. **What is still open and how was it verified?** — the *Unknowns*,
   *Next targets* and *Gates* sections.

Vocabulary follows `docs/RUST_STYLE.md`:

- **Status** — `translated` = full recovered control flow with every call
  accounted for; `partial` = entry/exit and major effects only; `stub` =
  address only. Every row here is `translated`.
- **Confidence** — strength of the *evidence*, not code quality: `high` = the
  stores/reads and the flow are explicit and self-contained; `medium` = flow is
  explicit but a role, a callee or a data structure is not proven.
- **Evidence** — path under `work/ghidra/main-gb/export/`; that C file is the
  source of truth.

All addresses are virtual addresses of the unrelocated `main.dol` (image base
`0x80004000`), not file offsets. The Rust lives in `src/runtime/os.rs`, after the
wave-8 section. The machine-readable companion is `ledger/agents/wave9-os.csv`
(same rows, same order). This wave does **not** write shared files:
`ledger/functions.csv` stays untouched and is merged by the integration step.

## Scope

Source index: `work/ghidra/main-gb/export/functions.tsv`, entries whose **entry**
address is in `0x80240000..0x80260000`.

| Item | Count |
|---|---:|
| Function entries in the window | 300 |
| Entries with size `< 16` bytes (address-only veneers, excluded) | 9 |
| Eligible entries (size `>= 16`, all decompiled) | 291 |
| Translated before this wave (5 `runtime::os` + 11 `runtime::bt` rows) | 16 |
| **Selected and translated by this wave** | **25** |
| Status `translated` / `partial` / `stub` | 25 / 0 / 0 |
| Bytes added | 1,820 |
| Remaining untranslated in the window | 250 |
| Coverage of eligible entries after this wave | 14.1 % |

Selection rule: complete, self-contained bodies, no dependence on
`unaff_r13` small-data offsets, no unresolved jump table, and no `switchD`
recovery. Functions whose export still reads `unaff_r13` or
`Could not recover jumptable` were skipped. The five original bodies already in
this band (`0x80243690`, `0x80244e60`, `0x80249ac0`, `0x80246460`, `0x80247ab0`)
are **not** repeated.

| Layer (inferred) | Functions | Bytes | Address span |
|---|---:|---:|---|
| Component-triplet accessors | 3 | 344 | `0x802418B0..0x80241BE0` |
| Fixed-stride record tables | 3 | 228 | `0x802425E0..0x80242680` |
| Vtable / flag helpers | 6 | 396 | `0x80242B50..0x80243630` |
| Table selection and block copy | 2 | 416 | `0x8024A2D0..0x8024A530` |
| Object field accessors | 9 | 352 | `0x80253990..0x802544D0` |
| Small state helpers | 2 | 84 | `0x8025F470..0x8025F690` |
| **total** | **25** | **1,820** | `0x802418B0..0x8025F690` |

Confidence split: 20 `high`, 5 `medium`. No `low`, no `partial`, no `stub`.

## What this window is

`// inferred:` the window is the **generic object layer** that sits between the
debug-console code (`0x80233420..0x802356F0`, wave 7) and the render/data
tables (`0x8025xxxx`). None of the 25 bodies contains an assertion string; the
console/exception attribution of the wider band comes from its neighbours:

| String (as exported) | Evidence file | Suggested owner |
|---|---|---|
| `s_db_console_cpp_8034cf40`, `s_illegal_console_handle_8034cf50` | `80233c60_FUN_80233c60.c` | text console |
| `s__________EXCEPTION_OCCURRED______8034d068` | `80234b60_FUN_80234b60.c` | exception handler |
| `s____EXCEPTION_INFO_GPR____8034d0e0` | `80234b60_FUN_80234b60.c` | exception handler |

The functions here are the shared helpers those layers use: component
triplets, three fixed-stride record tables, a vtable-dispatch pattern with the
same `+0x40` slot seen in `fn_80247a50`, one unrolled block copy, and the
`0x802539xx`/`0x802544xx` object-field family whose flag word at `+0x70` is the
same one `fn_80253c30` touches in `src/runtime/bt.rs`.

## Function table

| Address | Size | Rust function | Role (plain English) | Status | Confidence | Evidence |
|---|---:|---|---|---|---|---|
| `0x802418B0` | 60 | `fn_802418b0` | Copy the secondary component triplet, or fill three words from the neutral constant when flag `0x40` is set | translated | high | `802418b0_FUN_802418b0.c` |
| `0x80241A00` | 152 | `fn_80241a00` | Store the primary triplet, setting `0x18`/`0x1a` for the neutral value and `0x10` when all three components match | translated | high | `80241a00_FUN_80241a00.c` |
| `0x80241BE0` | 132 | `fn_80241be0` | Store the secondary triplet, setting `0x40`/`0x44`/`0x46` or clearing `0x46` from the neutral test | translated | high | `80241be0_FUN_80241be0.c` |
| `0x802425E0` | 76 | `fn_802425e0` | Entry `index` of the `0x4C`-stride record table (count `+0x3C`, body offset `+0x14`), else `0` | translated | high | `802425e0_FUN_802425e0.c` |
| `0x80242630` | 76 | `fn_80242630` | Entry `index` of the `0x1C`-stride record table (count `+0x3E`, body offset `+0x18`), else `0` | translated | high | `80242630_FUN_80242630.c` |
| `0x80242680` | 76 | `fn_80242680` | Entry `index` of the `0x5C`-stride record table (count `+0x40`, body offset `+0x1C`), else `0` | translated | high | `80242680_FUN_80242680.c` |
| `0x80242B50` | 60 | `fn_80242b50` | Fill `count` `u16` slots with `0x8000`, then clear the container's flag bit `4` | translated | high | `80242b50_FUN_80242b50.c` |
| `0x80242BF0` | 64 | `fn_80242bf0` | Run the shared destructor on a non-null object and return the pointer | translated | high | `80242bf0_FUN_80242bf0.c` |
| `0x80243210` | 48 | `fn_80243210` | Call vtable slot `+0x14` and return its result plus four | translated | medium | `80243210_FUN_80243210.c` |
| `0x802432E0` | 28 | `fn_802432e0` | Decode bits `0xC000` of one table slot with `countLeadingZeros >> 5` | translated | high | `802432e0_FUN_802432e0.c` |
| `0x802433B0` | 100 | `fn_802433b0` | Install vtable `0x8034E2B8`, run the pointer at `0x8034E2FC`, then destroy the object; return it | translated | high | `802433b0_FUN_802433b0.c` |
| `0x80243630` | 96 | `fn_80243630` | Call vtable slot `+0x40` for every index below the count at `+0x18` | translated | high | `80243630_FUN_80243630.c` |
| `0x8024A2D0` | 196 | `fn_8024a2d0` | Pick one of six 16-byte-stride tables from a mode value and a query flag | translated | high | `8024a2d0_FUN_8024a2d0.c` |
| `0x8024A530` | 220 | `fn_8024a530` | Copy `len >> 5` 32-byte units, 128-byte blocks first and 32-byte units after | translated | high | `8024a530_FUN_8024a530.c` |
| `0x80253990` | 48 | `fn_80253990` | Copy the object's three words at `+0x74`/`+0x78`/`+0x7C` into a caller buffer | translated | high | `80253990_FUN_80253990.c` |
| `0x80253BE0` | 72 | `fn_80253be0` | Install a source block through `0x801C0500`, then set object flag bit `8` | translated | medium | `80253be0_FUN_80253be0.c` |
| `0x80253C70` | 64 | `fn_80253c70` | Store six floats at `+0xBC..+0xC8` and `+0xB4/+0xB8`, set `+0xA8 = 1`, force flag `0x40` | translated | high | `80253c70_FUN_80253c70.c` |
| `0x80253D00` | 32 | `fn_80253d00` | Store four floats at `+0xCC..+0xD8` | translated | high | `80253d00_FUN_80253d00.c` |
| `0x80253D20` | 32 | `fn_80253d20` | Store four words at `+0xF4..+0x100` | translated | high | `80253d20_FUN_80253d20.c` |
| `0x80253D40` | 24 | `fn_80253d40` | Store two words at `+0x104`/`+0x108` | translated | high | `80253d40_FUN_80253d40.c` |
| `0x80253E00` | 24 | `fn_80253e00` | Store two floats at `+0xEC`/`+0xF0` | translated | high | `80253e00_FUN_80253e00.c` |
| `0x802544A0` | 32 | `fn_802544a0` | Forward the four words at `+0xF4..+0x100` to `0x801C9C60` | translated | medium | `802544a0_FUN_802544a0.c` |
| `0x802544D0` | 24 | `fn_802544d0` | Forward the two words at `+0x104`/`+0x108` to `0x801C9D20` | translated | medium | `802544d0_FUN_802544d0.c` |
| `0x8025F470` | 60 | `fn_8025f470` | Tear down the sub-block at `+0x04` through `0x801C66E0`, then clear state bit `2` | translated | high | `8025f470_FUN_8025f470.c` |
| `0x8025F690` | 24 | `fn_8025f690` | Null-guarded install of a transform block through `0x801C66F0` | translated | medium | `8025f690_FUN_8025f690.c` |

## What the pass adds

### 1. The component block now has a written layout

`0x802418B0`, `0x80241A00` and `0x80241BE0` are three views of the same block:
a flag word at `+0x00` and two `f32` triplets, primary at
`+0x04/+0x08/+0x0C` and secondary at `+0x28/+0x38/+0x48`. The flag bits are
translated bit-for-bit; their meanings are not claimed. The two read-only
constants (`sda2(-0x4AE8)`, `sda2(-0x4AE4)`) become named constants with the
resolved `0x8042xxxx` addresses.

### 2. Three record tables become indexable

The three locators share one shape: `u16` count, relative body offset and a
`u32` tag at entry `+0x0C` that must be smaller than the count. Keeping the
three strides (`0x4C`, `0x1C`, `0x5C`) side by side documents which table a
caller is using without naming the record type. This complements the
`0x803DFFB4` table accessor (`fn_8024d390`) already in `src/runtime/bt.rs`.

### 3. The vtable-dispatch family is now explicit

`0x80243210`, `0x802433B0` and `0x80243630` show three distinct shapes of the
same object model: a single slot call returning `result + 4`, a destructor that
reinstalls a vtable before running a data-pointer teardown, and a loop over
slot `+0x40`. The `+0x40` slot matches the one used by `fn_80247a50`, so the
two waves now agree on the vtable layout.

### 4. The block copy is not `memcpy`

`0x8024A530` ignores the low five bits of its length, copies 128-byte blocks
(16 doublewords) first, then `(len >> 5) & 3` 32-byte units. It is a forward
copy only, so `src` and `dst` must not overlap. This differs from `memcpy`
semantics and is why the Rust does not delegate to the boot CRT.

## Unknowns

1. **Flag meanings.** The bits `0x04`, `0x08`, `0x10`, `0x18`, `0x1A`, `0x20`,
   `0x40`, `0x46` of the component block, the `+0x70` object flags and the
   state bit `2` of `fn_8025f470` are translated exactly, but no string in the
   window names any of them.
2. **Callee roles.** `0x801C0500`, `0x801C66E0`, `0x801C66F0`, `0x801C7220`,
   `0x801C7250`, `0x801C9C60` and `0x801C9D20` are dispatched
   by address. Only their argument shapes are observed; their names and
   behaviour are not recovered here.
3. **Virtual-call arguments.** `FUN_80243210` and `FUN_802433B0` call through
   pointers with `()` in the Ghidra output. The Rust mirrors the C and passes
   no arguments; on the target the callee still sees whatever `r3` held. This
   is why both rows are `medium`.
4. **`0x8034E2FC`.** The data word is a code pointer with no recovered name;
   `fn_802433b0` reads and calls it, and the Rust keeps the raw address.
5. **`double` arguments.** `fn_80253c70`, `fn_80253d00` and `fn_80253e00`
   receive six, four and two `double`s and narrow each to `f32` at the store.
   The Rust mirrors the Ghidra ABI exactly; whether the real call site passes
   `float` in `f1..` cannot be decided from the C export alone.
6. **Record-type identity.** The three tables and the `+0x74`/`+0xF4`/`+0x104`
   field families are structurally complete but have no string evidence. If a
   later lane proves an owner (UI layout, animation, collision), move the
   functions and update the ledger — do not copy them.

## Next targets

Closest remaining bodies in the window, all `>= 16` bytes and free of
`unaff_r13` unless noted:

| Address | Size | Why next |
|---|---:|---|
| `0x80242C30` | 336 | Neighbour of the flag helper; first body that indexes both tables (reads `unaff_r13`, needs the SDA story settled). |
| `0x80243000` | 188 | The loop that pairs `0x8023B360` with vtable slot `+0x38`; would name the record list. |
| `0x80243B30` | 308 | Twin family of `0x802433B0` with a different vtable; gives the destructor family its second member. |
| `0x80249A70` | 76 | The shared destructor called by `fn_80242bf0` and `fn_802433b0`; small and self-contained. |
| `0x80249CB0` | 128 | Single `GX` FIFO store; would pin the FIFO address and the constant. |
| `0x8024B020` | 112 | Three guarded `0x80249DD0` calls selected by flag bits `1`/`2`/`4`. |
| `0x8024D4F0` | 212 | Copy-out of a 0x30-byte record; names the record consumed by `0x8024D0B0`. |
| `0x8025C420` | 72 | One of six identical small wrappers; would identify the `0x8025C4xx` handler family. |
| `0x8025F440` | 48 | Tails `0x801C6870`; names the callback slot used by `fn_8025f690`. |
| `0x8025FE30` | 1940 | Large worker at the end of the window; only after its callees are named. |

Deliberately **not** next: the 9 `< 16`-byte veneers, every export that still
reads `unaff_r13` (for example `0x80242C30`, `0x80243D20`, `0x80245500`,
`0x8024A1D0`, `0x8024B1E0`, `0x8024D5E0`), and anything with
`Could not recover jumptable` (`0x80257740`, `0x802590F0`, `0x8025A780`).

## Gates

- `node tools/doc-lint.mjs src` → `ok: true`, 0 violations, 68 files.
- `cargo check` → `Finished dev profile`, exit 0; no error and no warning from
  `src/runtime/os.rs`. The 12 remaining warnings are pre-existing and outside
  this lane: eleven `unnecessary parentheses` in `src/runtime/bt.rs` and one
  `function_casts_as_integer` note in `src/runtime/input.rs`. They were left
  untouched.
- Files edited by this wave: `src/runtime/os.rs`, `ledger/agents/wave9-os.csv`
  and this document. No staging module, `lib.rs`, any `mod.rs`, `Cargo.toml`,
  `platform.rs`, `ledger/functions.csv` or other lane's file was touched.
