# Coverage - wave 204, lane D (`ui::hud`, `0x800275F0..0x80028B5C`)

Ten functions of `src/ui/hud.rs` translated from `sys/main.dol` in wave 204,
lane D. All ten have a faithful Rust body in `work/wave204/laneD_block.rs`
(ready to append to `src/ui/hud.rs`); nothing in this lane is deferred.

## How to read this file

`address` is the virtual address in the shipped DOL (base `0x80004000`), `size`
the extent from `work/ghidra/main-gb/export/functions.tsv`, and `role` a
one-line description of what the body does. Every row was reconstructed from
the Ghidra export named in `evidence` and cross-checked against the raw
PowerPC words of `work/dol/main.elf`.

## Scope

| address | size | name | confidence | role |
|---|---|---|---|---|
| `0x800275F0` | 124 | `fn_800275f0` | medium | probe the terrain cell under the scene, spawn `"Wm_en_quicksand"` on a hit |
| `0x80027A70` | 132 | `fn_80027a70` | medium | one-shot "settled" test on the sub-object at `+0x1EC`, latch at `+0x74A` |
| `0x80027D00` | 108 | `fn_80027d00` | high | spawn `"Wm_en_burst_s"` at `{+0xAC, +0xB0 + 8.0, 5500.0}`, then activate the scene |
| `0x800281C0` | 224 | `fn_800281c0` | high | construct the fire-pipe (`dokan_fire`) actor and its four vtables |
| `0x800282A0` | 64 | `fn_800282a0` | high | deleting-destructor slot of the `+0x6D8` member (vtable `0x80304228` slot `+0x08`) |
| `0x800282E0` | 144 | `fn_800282e0` | high | tear down the seven members, optionally release the actor |
| `0x80028370` | 88 | `fn_80028370` | high | deleting-destructor slot of the `+0x718` member (vtable `0x803033F0` slot `+0x08`) |
| `0x800286F0` | 140 | `fn_800286f0` | medium | lazily build four cardinal direction pairs, store one scaled into `+0xE8..+0xF0` |
| `0x80028780` | 424 | `fn_80028780` | medium | bind `"dokan_fire_st"` to both model records, apply two rates and one scale |
| `0x80028930` | 560 | `fn_80028930` | medium | bind `"dokan_fire_neck"` and `"dokan_fire_wait"`, store and apply two scales |

Total: 2,008 bytes, 10/10 translated. Addresses come from
`ledger/agents/wave204-laned.csv` (`module = ui::hud`).

The range is a complete actor: constructor, three destructor slots, one
per-frame direction helper and two animation binders. It sits directly before
lane E, which starts at `0x80028B60`.

## Two size traps (manifest says 20)

The lane manifest lists `0x80028780` and `0x80028930` as **20 bytes** each.
That is the same listing artefact wave 203 hit: it is the length of the saved
`f31`/`f30` prologue, not of the body.

- `0x80028780` runs to its `blr` at `0x80028924`, with zero padding at
  `0x80028928/0x8002892C`, so the extent is `0x1A8` = **424** bytes -
  `functions.tsv:446` says 424.
- `0x80028930` runs to its `blr` at `0x80028B5C`, so the extent is `0x22C` =
  **560** bytes - `functions.tsv:447` says 560. The next prologue (`stwu
  r1,-0x60`) is at `0x80028B60`.

The `# Original` headers use the `functions.tsv` sizes because
`tools/check-lane-csv.mjs` cross-checks them.

## The `psq_st` prologue is a register save, not arithmetic

`0x80028780` and `0x80028930` open with the paired-single save block
(`stfd f31` + `psq_st f31`), which Ghidra renders as `unaff_GQR0`, `ldexpf`
and `undefined8 param_1`. It is a float save/restore only; neither body
contains any quantised arithmetic. The same conclusion as
`docs/COVERAGE_WAVE203_LANED.md`.

A smaller instance: Ghidra gives `fn_80028930` the signature
`(undefined8, undefined8, int)`, swallowing `r3`/`r4` into the save helper.
The shipped words are `mr r29,r3`, `fmr f30,f1`, `fmr f31,f2`, so the real
signature is `(this, neck_scale, model_scale)`; the two scales are only used at
`0x80168FE0` (first) and `0x80169F80` (second).

## Ghidra type errors fixed in this lane

| function | Ghidra | shipped words |
|---|---|---|
| `0x80070BA0`, `0x80070BF0` | `void f(double, double, undefined4)` | the value passes through from `0x800776F0`/`0x80077700`; the caller consumes `r3`, so the block keeps a `u32` result |
| `0x8006EF40` | `uint f(int, uint)` | two operands, result in `r3`: `handle != 0 \|\| armed != 0` |
| `0x80094E80` | `void f(void)` | `this` arrives in `r3` (hidden by the `__savegprlr` helper), so the block passes it explicitly as `call1(FN_80094E80, p32(this))` |
| `0x800282A0` | `FUN_802b9280()` with no operand | `r3` still holds `this`, so the release is `call1(FN_802B9280, p32(this))` |
| `0x80028780`, `0x80028930` | `undefined8 param_1` | `this` is `r3`, the scale(s) are `f1`/`f2` |

## Shipped data this lane reads

Read through the ELF program headers (`%TEMP%/opencode/rd.js`):

| address | content |
|---|---|
| `0x80302D68` | `"Wm_en_burst_s"` (already named `S_WM_EN_BURST_S` by wave 203 - reused) |
| `0x8030307C` | `"dokan_fire_st"` |
| `0x80303090` | `"dokan_fire_neck"` |
| `0x803030A0` | `"dokan_fire_wait"` |
| `0x8042B364` (`SDA2 - 0x7D1C`) | `8.0` - burst height lift in `fn_80027d00` |
| `0x8042B380` (`SDA2 - 0x7D00`) | `5500.0` - burst third component in `fn_80027d00` |
| `0x8042B3B0` (`SDA2 - 0x7CD0`) | `0.0` - direction component and the `+0xF0` third component |
| `0x8042B3B4` (`SDA2 - 0x7CCC`) | `-1.0` - negative direction component |
| `0x8042B3A4` (`SDA2 - 0x7CDC`) | `1.0` - positive direction component and the `0x801656C0` rate |
| `0x8042B3A8` (`SDA2 - 0x7CD8`) | `1.0, -1.0, 0.0, -1.0` - per-index sign factors |
| `0x80429C58` (`SDA - 0x5A28`) | BSS - one-byte direction-table build guard |
| `0x80427858` (`SDA - 0x7E28`) | BSS - the record passed to `0x80239E30` by both binders |
| `0x80352FA0` | BSS - the eight-`f32` direction pair table |
| `0x80303F48` / `0x803030FC` | the base vtables written at `+0x60` (first / final) |
| `0x80303408` | the member vtable at `+0x584` and `+0x6E0` |
| `0x80304228` | vtable at `+0x6D8`; slot `+0x08` is `0x800282A0` |
| `0x803295D8` / `0x803033F0` | the vtables at `+0x718` (first / final); slot `+0x08` of the latter is `0x80028370` |

`fn_800286f0` therefore installs the four direction pairs `(0, 1)`, `(0, -1)`,
`(1, 0)`, `(-1, 0)`; `TABLE_DIRECTION_SIGN[2] == 0.0` zeroes the vector, which
is kept literally.

## Reused module helpers and constants

`S_WM_EN_BURST_S` (line 12184), `FN_80239E30` (9793), `FN_80169F80` (9803),
`FN_8023A0B0` (13852), `FN_8016C890` (10632), `FN_8006EF40` (11705),
`FN_802B9280` (624), `call_f32_2u` (9849) and `vtable_method`/`p32`/`read_f32`/
`write_f32`/`copy_vec3` from `crate::ui::lyt` are all pre-existing and are
reused, not redefined.

New in the block: the three calling helpers `call_2f_1u` (`f32, f32, u32`),
`call_f32_1u` (`f32, u32`) and `call_f32_3u` (`f32, u32, u32, u32`), plus
lane-namespaced constants (`S_DOKAN_FIRE_*`, `F32_SDA2_7D1C`, `F32_SDA2_7D00`,
`F32_SDA2_7CD0`, `F32_SDA2_7CCC`, `F32_SDA2_7CDC`, `TABLE_DIRECTION_SIGN`,
`TABLE_DOKAN_DIRECTION`, `G_SDA_5A28`, `G_SDA_7E28`, `VT_DOKAN_FIRE_*`,
`FN_80094E80`, `FN_80069020`, `FN_80069060`, `FN_80168D10`, `FN_80168D80`,
`FN_80169CD0`, `FN_80169D20`, `FN_8016A5E0`, `FN_8008C200`, `FN_8008C260`,
`FN_80095130`, `FN_80162920`, `FN_80167EC0`, `FN_80072BC0`, `FN_8006CED0`,
`FN_80070BA0`, `FN_80070BF0`, `FN_800943F0`, `FN_80162510`, `FN_801651F0`,
`FN_801656C0`, `FN_80168FE0`). A `grep` of `src/ui/hud.rs` for every one of
these names before writing returned zero hits.

## Deliberate literal details

- `fn_800282e0` tests the *computed* address `this + 0x584` for null before the
  inner teardown, so the guard can only fail for `this == -0x584`. The block
  keeps the literal test.
- `fn_800282a0` performs no member teardown: it only releases the storage when
  the delete flag is positive. That is what the shipped words do, even though
  the function sits in a deleting-destructor slot.
- `fn_80028780` binds `"dokan_fire_st"` twice, once per record, instead of
  caching the second lookup.
- The `+0x50` virtual calls re-store the same model handle into a fresh stack
  word before each call, because the callee receives `r4` = address of that
  word; the block reproduces one word per call.
- `fn_800275f0` asks `0x8006CED0` for three floats but only offers the first
  two to the cell lookups; the third stays unused in the buffer but is part of
  the vector handed to `0x800943F0`.
- Both cell lookups re-read `x`/`y` from the stack buffer rather than caching
  the register values, exactly as the original does.

## Verification

`work/wave204/laneD_block.rs` was appended to a temporary copy of the crate
(`%TEMP%/opencode/wave204_laned_verify`, `Cargo.toml` + `src/`) and
`cargo check --lib` exited **0** (17.6 s, 23 pre-existing warnings, none in the
appended range; a `grep` of the compiler output for `fn_8002*`, `call_f32*`,
`call_2f_1u`, `DOKAN` and `DIRECTION` matched nothing).

Ledger: `ledger/agents/wave204-laned.csv`, header + 10 rows, addresses as plain
lowercase 8-digit hex. `node tools/check-lane-csv.mjs wave204` reports no
format, duplicate, evidence or size problem for this file; the only lines for
`wave204-laned.csv` are the ten "has no body and no `# Original` reference in
`src/ui/hud.rs`", which disappear once the coordinator appends the block.

**Coordinator note:** the same run shows every lane of wave 204 still unmerged
(60 rows, 60 unique addresses), so those ten-per-lane lines are expected for
the whole wave, not specific to lane D.

## Evidence

- Disassembly: `work/dol/main.elf` via `ppc159.js`; `LEN` 108..560 at
  `0x800275F0`, `0x80027A70`, `0x80027D00`, `0x800281C0`, `0x800282A0`,
  `0x800282E0`, `0x80028370`, `0x800286F0`, `0x80028780`, `0x800288C0`,
  `0x80028930`, `0x80028A70`; `STOP=0` throughout.
- Decompilation: `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` (lowercase)
  for all ten addresses.
- Callee bodies consulted: `80069020`, `80069060`, `8006ced0`, `8006ef40`,
  `80070ba0`, `80070bf0`, `80070c40`, `80070c60`, `80072bc0`, `8008c200`,
  `8008c260`, `800943f0`, `80094e80`, `80095130`, `80162510`, `80162920`,
  `801651f0`, `801656c0`, `80167ec0`, `80168d10`, `80168d80`, `80168fe0`,
  `80169cd0`, `80169d20`, `80169f80`, `8016a5e0`, `80239e30`, `8023a0b0`.

## Files

- Generated block: `work/wave204/laneD_block.rs`
- Ledger: `ledger/agents/wave204-laned.csv`
- This document: `docs/COVERAGE_WAVE204_LANED.md`
- `src/ui/hud.rs` was **not** modified (coordinator merges).
