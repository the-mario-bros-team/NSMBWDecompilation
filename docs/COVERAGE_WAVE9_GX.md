# Coverage wave 9 — GX / render fifth pass (`0x80280000..0x802B0000`)

## How to read this file

- **Scope** — 25 previously untranslated, non-GQR functions from the GX/render
  band of `main.dol`, all now in `src/render/gx_band.rs` (ledger module
  `render::gx_band`). This pass continues `docs/COVERAGE_GX_BAND.md` (range
  inventory) and the four earlier passes: `docs/COVERAGE_WAVE4_GX.md`,
  `docs/COVERAGE_WAVE5_GX.md` and `docs/COVERAGE_WAVE7_GX.md`.
- **What this pass adds** — the texture-state flush `0x80280870`, the
  material-name table helpers `0x80294D60..0x80295090`, the draw-state list
  primitives `0x80295640..0x802957D0`, the texture command writer/dispatcher
  `0x8028CD70..0x802970D0`, the material release path `0x80284910`, and the
  `0x5C`-byte material constructor `0x802AD920`.
- **Addresses** are virtual `main.dol` addresses, not file offsets. Image base
  is `0x80004000`; the main text segment maps virtual `0x80006780` to file
  offset `0x27C0` in `work/dol/main.elf`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` = entry/exit, every
  call and every observable memory/MMIO write is accounted for; `partial` = a
  known piece is missing and marked `// partial:`; `stub` = address only.
- **Confidence** measures the strength of the evidence, not code quality:
  `high` = the raw instructions and the Ghidra C agree, or the raw stream was
  used directly; `medium` = the flow is exact but a callee identity, field
  meaning or role is inferred; `low` = mostly guesswork.
- **Evidence** means the Ghidra C under `work/ghidra/main-gb/export/`, plus a
  raw-instruction check against `work/dol/main.elf`. Every row names its
  Ghidra file and raw range.
- **Start here** — [`fn_80296dd0`](../src/render/gx_band.rs) is the clearest
  record builder; [`fn_8028cd70`](../src/render/gx_band.rs) shows how a
  texture command is selected; [`fn_80294d60`](../src/render/gx_band.rs)
  documents the material-name table layout.
- **Companion ledger** — `ledger/agents/wave9-gx.csv`.

## Summary

| module | functions | bytes (`functions.tsv`) | address span | status | confidence |
|---|---|---:|---|---|---|
| `render::gx_band` | 25 | 4,560 | `0x80280870..0x802AD920` | 25 translated, 0 partial, 0 stub | 22 high, 3 medium, 0 low |

Band totals after this pass (`work/ghidra/main-gb/export/functions.tsv`,
entries `0x80280000 <= addr < 0x802B0000`):

| Metric | Count |
|---|---:|
| Function entries in band | 345 |
| Translated before wave 9 | 124 |
| Added by wave 9 | 25 |
| Translated now | 149 |
| Coverage of entries | 43.2 % |
| Deferred | 196 |
| Deferred because of paired-single/GQR (`G`) | 85 |
| Deferred because entry `< 16` bytes (`S`) | 25 |
| Deferred GX-FIFO state code, no GQR (`F`) | 1 |
| Deferred for other scope reasons (`R`) | 85 |

None of the 25 entries contains `ldexpf(` or `unaff_GQR0`; all were previously
in the `R` bucket. The `G`, `S` and `F` counts are unchanged from
`docs/COVERAGE_GX_BAND.md`.

## Function table

Evidence paths are relative to `work/ghidra/main-gb/export/`.

| address | size | Rust function | role | status | confidence | evidence |
|---|---|---:|---|---|---|---|
| `0x80280870` | 156 | `fn_80280870` | flush the texture state and run the `+0x94` callback | translated | high | `80280870_FUN_80280870.c` |
| `0x80284210` | 144 | `fn_80284210` | decode a `0x52575344` material/header record | translated | high | `80284210_FUN_80284210.c` |
| `0x80284910` | 296 | `fn_80284910` | release manager-owned material state and the `+0x114` handle | translated | medium | `80284910_FUN_80284910.c` |
| `0x80285C50` | 136 | `fn_80285c50` | release one list node when its state is 1 | translated | high | `80285c50_FUN_80285c50.c` |
| `0x802865A0` | 136 | `fn_802865a0` | release every entry of the `+0xC014` texture list | translated | high | `802865a0_FUN_802865a0.c` |
| `0x80286750` | 72 | `fn_80286750` | reset one `0x1C`-byte texture-list entry | translated | high | `80286750_FUN_80286750.c` |
| `0x80286A30` | 144 | `fn_80286a30` | sum the `+0x24` list cost of one bucket | translated | high | `80286a30_FUN_80286a30.c` |
| `0x80286AC0` | 144 | `fn_80286ac0` | sum the sibling bucket list cost | translated | high | `80286ac0_FUN_80286ac0.c` |
| `0x8028CD70` | 356 | `fn_8028cd70` | hash-select a texture command and append it | translated | medium | `8028cd70_FUN_8028cd70.c` |
| `0x80292B20` | 64 | `fn_80292b20` | conditional-free wrapper | translated | high | `80292b20_FUN_80292b20.c` |
| `0x80294D60` | 148 | `fn_80294d60` | material-name table index lookup | translated | high | `80294d60_FUN_80294d60.c` |
| `0x80294E00` | 140 | `fn_80294e00` | resolve every material-name record pointer | translated | high | `80294e00_FUN_80294e00.c` |
| `0x80294F50` | 124 | `fn_80294f50` | insert one resolved material-name record | translated | high | `80294f50_FUN_80294f50.c` |
| `0x80294FD0` | 180 | `fn_80294fd0` | name lookup with optional resource table | translated | high | `80294fd0_FUN_80294fd0.c` |
| `0x80295090` | 200 | `fn_80295090` | name lookup in the `+0x10` list | translated | high | `80295090_FUN_80295090.c` |
| `0x80295640` | 160 | `fn_80295640` | unlink a node and subtract its weight | translated | high | `80295640_FUN_80295640.c` |
| `0x802956E0` | 160 | `fn_802956e0` | unlink from the `+0x10` list | translated | high | `802956e0_FUN_802956e0.c` |
| `0x80295780` | 80 | `fn_80295780` | reset the two global material lists | translated | high | `80295780_FUN_80295780.c` |
| `0x802957D0` | 132 | `fn_802957d0` | drain a list into an array | translated | high | `802957d0_FUN_802957d0.c` |
| `0x80296DD0` | 380 | `fn_80296dd0` | append a direct texture command record | translated | high | `80296dd0_FUN_80296dd0.c` |
| `0x80296F50` | 384 | `fn_80296f50` | append a virtual-dispatch command record | translated | high | `80296f50_FUN_80296f50.c` |
| `0x802970D0` | 308 | `fn_802970d0` | dispatch and clear the command buffer | translated | high | `802970d0_FUN_802970d0.c` |
| `0x802AC020` | 96 | `fn_802ac020` | release the `+0x28` sub-object and `+0x1C` helper | translated | high | `802ac020_FUN_802ac020.c` |
| `0x802AD080` | 88 | `fn_802ad080` | apply an operation through a matrix scratch slot | translated | high | `802ad080_FUN_802ad080.c` |
| `0x802AD920` | 332 | `fn_802ad920` | construct a `0x5C`-byte texture/material object | translated | medium | `802ad920_FUN_802ad920.c` |

Totals: **25 translated, 0 partial, 0 stubs**; **22 high, 3 medium, 0 low**.

## What the wave-9 pass adds

### 1. Texture state and material name tables (`0x80280870..0x80295090`)

`0x80280870` is the texture-state flush: it requires `+0x9C` and `+0x9E` to
be set, asks `0x80261E70` about the handle at `+0x0C`, then calls the `+0x94`
callback with `(object, 0, object[+0x98])` and clears `+0x9E`/`+0x9D`. The
Ghidra C drops the handle argument to `0x80261E70`; the raw stream keeps it in
`r3`.

The five `0x80294D60..0x80295090` functions share one name-table layout:

1. the table base is `table + u32[table]`;
2. a `u16` count sits at `base+0x04`;
3. records start at `base+0x08` and are
   `u16 length + name bytes + 8-byte payload header`;
4. the first payload word is added to `base` to reach the entry.

`0x80294D60` returns one entry by index. `0x80294E00` stores, for every
record, the name pointer at the entry, `entry+0x40` at `+0x1C`, and
`entry+0x40+u32[entry+8]` at `+0x20`. `0x80294F50` inserts one record into a
material list and raises the list byte total at `+0x0C`.
`0x80294FD0`/`0x80295090` are the resource-first name lookups for the two
material lists.

`0x80284210` is a separate header decoder for magic `0x52575344`: it evaluates
the exact `((~version | 0x103) - ((0x103 - version) >> 1)) >> 31` test and
resolves the `+0x10` and `+0x18` self-relative offsets.

### 2. Draw-state lists (`0x80285C50..0x802957D0`)

`0x80285C50`-adjacent list primitives are the state machine used by the
wave-5 texture grid:

- `0x80285C50` releases a node only in state 1, decrements the manager counter
  at `+0x3C`, and calls virtual slots `+0xC`/`+8`.
- `0x80295780` re-initialises the two global material lists at
  `0x80423170`/`0x80423180`.
- `0x802957D0` drains a list into a caller array; it always stores the first
  node before testing the maximum.
- `0x80295640`/`0x802956E0` unlink a node and subtract its self-relative
  weight from the list total at `+0x0C`/`+0x1C`.
- `0x802865A0` releases every entry of the `+0xC014` list and optionally frees
  the owner.

### 3. Texture command buffer (`0x8028CD70..0x802970D0`)

This is the pass's FIFO-adjacent group. The command record is `0x30` bytes:

| Offset | Meaning |
|---|---|
| `+0x00` | kind: 0 = direct call, 1 = virtual dispatch |
| `+0x01` | flags: 1 = colour0 present, 2 = colour1 present |
| `+0x02` | `u16` object id |
| `+0x04` | source stream (ten bytes: one `u16` + eight bytes) |
| `+0x10` | owner pointer; `owner+0x10` is its reference count |
| `+0x14` | opaque argument word |
| `+0x18` | optional first colour triple |
| `+0x24` | optional second colour triple |

`0x80296DD0` and `0x80296F50` append a record; they differ only in the kind
byte at `+0x04`. Their shared Rust helper keeps the exact store order. The
original reloads the buffer count between stores; the Rust helper assumes the
buffer and the owner are distinct objects (documented in `# Safety`).

`0x8028CD70` selects the source record. A zero mode byte uses
`record+0x0C`; otherwise it forms

```
seed * 0x7B929
+ u16[record+0x0C] * 0x371097E7
+ param_2 * 0x3F81F635
+ u16[descriptor+0x06] * 0x30A74193
+ 0x4BF53
```

and folds the four bytes with XOR. When bit 1 of the mode byte is set, the
folded high half is reduced modulo `u16[table]` and indexes
`table + remainder*0x0C + 4`. The selected `u16` at `+0x0A` indexes
`textures + index*4 + 4`; a non-zero entry is appended to the nested buffer
`*(*(*(*(object+0xC8)+0x20)+0xBC)+0x20)+0x10`.

`0x802970D0` dispatches every stored record. Kind 0 calls the GQR-affected
`0x80287C10(first, +0x14, record+4, owner, id, optional colour)`; kind 1 calls
the owner object's virtual slot `+0x14`. Both arms decrement the owner's
reference count and call virtual slot `+8` at zero/state 2. The buffer count
is cleared at the end.

### 4. Material state and constructor (`0x80284910..0x802AD920`)

`0x80284910` removes the `+0xCD` and `+0xCC` slots from their manager lists
under two locks, then releases the `+0x114` handle under a third lock and
unlocks three times. `0x802AD920` constructs the `0x5C`-byte material object:
it installs vtable `0x8034F8B8`, initialises the delegate at `+0xD8`, creates
the slot array at `+0xE8`, copies four descriptor words, and builds the
material through the descriptor's `+0x5C` table chain.

## Raw/Ghidra differences handled

1. **`0x80280870` hidden handle argument.** Ghidra prints
   `FUN_80261e70()`; the raw call keeps `*(object+0x0C)` in `r3`.
2. **`0x80284210` version test.** Ghidra prints a signed-mask expression that
   would admit versions `0x100..0x103`. The raw branch at `0x80284240` accepts
   only `version > 0x103`; the Rust follows the raw value.
3. **`0x80284910` hidden release argument.** Ghidra prints `FUN_80268c40()`;
   the raw stream loads `*(object+0x114)` into `r3`.
4. **`0x802AD920` delegate arguments.** Ghidra prints
   `FUN_802dc9a0(param_1 + 0x36, &LAB_802ad910, FUN_800075d0, 4, 4)`; the raw
   stream uses `object+0xD8` (the `0x36` word index) and the local label
   `0x802AD910`.
5. **Save-helper returns in `0x80296DD0`/`0x80296F50`.** The `0x802DCD6C`
   call in the prologue is a CodeWarrior register-save helper; `r3`/`r4` still
   carry the buffer and source arguments. The Rust uses `(entries, source)`
   directly.
6. **`0x80294F50` record weight.** The weight is read at
   `entry + u32[entry] + 4`, not from the payload header. The raw `lwz`/`lhz`
   pair confirms this.
7. **`0x802957D0` do-while order.** The original stores the first node even
   when `max` is zero, then compares the low 16 bits of the count. The Rust
   preserves that order.

## Deferred addresses

The full 85-entry `G` list is unchanged from `docs/COVERAGE_GX_BAND.md`; every
one of those files still contains both `ldexpf(...)` and `unaff_GQR0`. The
following entries were opened or referenced by this pass and deliberately
**not** translated:

### `G` — paired-single / GQR (unchanged)

- **Opaque callee:** `0x80287C10` (material colour/command emitter) — called
  by [`fn_8028cd70`](../src/render/gx_band.rs) and
  [`fn_802970d0`](../src/render/gx_band.rs) by address.
- **Adjacent to the new clusters:** `0x80280910`, `0x80280BA0`, `0x80281C90`,
  `0x802821E0`, `0x8028E310`, `0x8028E890`, `0x802946F0`, `0x80295860`,
  `0x80295A90`, `0x80295C20`, `0x802960F0`, `0x80296160`, `0x802961E0`,
  `0x80296260`, `0x802963B0`, `0x80296530`, `0x802965B0`, `0x802966F0`,
  `0x80296820`, `0x80298720`, `0x80298B80`, `0x80298ED0`, `0x80299510`,
  `0x80299840`, `0x8029C820`, `0x8029CFC0`, `0x8029F640`, `0x8029F900`,
  `0x802A08A0`, `0x802A0910`, `0x802A09F0`, `0x802A0A40`, `0x802A0E50`,
  `0x802A1090`, `0x802A2DE0`, `0x802A3160`, `0x802A35C0`, `0x802A3A20`,
  `0x802A3E00`, `0x802A40C0`, `0x802A4400`, `0x802A47C0`, `0x802A5810`,
  `0x802A64B0`, `0x802A6850`, `0x802A6CB0`, `0x802A6F80`, `0x802A6FB0`,
  `0x802A71D0`, `0x802A7CA0`, `0x802A8020`, `0x802A8440`, `0x802A8930`,
  `0x802A8960`, `0x802A8A10`, `0x802A8D10`, `0x802A90C0`, `0x802A9640`,
  `0x802AA830`.
- The `G` count remains 85; no entry was reclassified by this pass.

### Non-GQR entries inspected and left for a later pass

- `0x802802D0` — draw-queue removal; still needs the out-of-lane unlink helper
  `0x802290C0` and the `0x802DCD6C`/`0x802DCDB8` save/restore convention.
- `0x802925D0` — material alpha scale; the template curves and the
  `0x319C`/`0x10000` divisor table need their own pass.
- `0x8029BAB0` — the `F`-bucket GX state readback/emit routine (1,484 bytes,
  69 FIFO writes).
- `0x80295400` — texture resolver over three nested tables; too large for this
  pass's evidence budget.
- `0x802AB680`, `0x802ABE90`, `0x802AC550`, `0x802AC720`, `0x802AC8B0`,
  `0x802ACB80` — release/dispatch paths whose manager list semantics are not
  yet established.
- `0x802AD720`, `0x802ADEA0`, `0x802AF240`, `0x802AF6F0`, `0x802AFED0`,
  `0x802AFF60` — four-character-code material/texture factories and their
  large upload paths.
- `0x80282040`, `0x802835B0`, `0x80283760`, `0x80283940`, `0x802839D0` —
  scalar curve/table helpers outside the five requested roles.

## Promotion status

| Cluster | Functions | Destination file | Ledger `module` |
|---|---:|---|---|
| Texture state / material names | 8 | `src/render/gx_band.rs` | `render::gx_band` |
| Draw-state lists | 10 | `src/render/gx_band.rs` | `render::gx_band` |
| Texture command buffer | 4 | `src/render/gx_band.rs` | `render::gx_band` |
| Material state / constructor | 3 | `src/render/gx_band.rs` | `render::gx_band` |

- `src/render/gx_band.rs` is the final semantic home; there is no staging
  module.
- `ledger/agents/wave9-gx.csv` has one row per translated function (25 rows).
- No `lib.rs`, `mod.rs`, `Cargo.toml`, `platform.rs`, generated doc or other
  lane file was edited by this pass.

## Uncertainties

1. **Class and field names remain inferred.** The Rust uses offset-based
   descriptions; no Nintendo class name is claimed.
2. **`0x80287C10` stays opaque.** It is GQR-affected; the translation
   reproduces only the six-argument call shape.
3. **`0x8028CD70` hash inputs are inferred.** The constants and the XOR fold
   are exact; whether the four inputs are UV, slot, LCG state or a texture
   key is not proven.
4. **`0x802AD920` delegate/table roles are inferred.** The `0x802DC9A0`
   delegate layout and the descriptor `+0x5C` table are reproduced from the
   raw chain, but their human-facing names are unknown.
5. **`0x802AD910` is a local callback label.** It is not an exported function
   in `functions.tsv`; the address is kept verbatim.
6. **`0x80296DD0`/`0x80296F50` assume non-aliasing.** The original reloads the
   buffer count between stores; a caller that passes the owner as the buffer
   itself would observe different addresses. No such caller exists in the
   band.

## Method and reproducibility

- Scope: `functions.tsv` entries with `0x80280000 <= entry < 0x802B0000`;
  345 entries.
- Selection excluded every file containing `ldexpf(` or `unaff_GQR0` (85
  files), every address already present in `ledger/functions.csv` and the
  four GX ledgers (124 addresses), and every entry smaller than 16 bytes (25).
  From the remaining 136 candidates, 25 were selected for the texture,
  material-name, list, command-buffer and constructor roles.
- Each selected entry was disassembled from `work/dol/main.elf` and compared
  instruction-by-instruction with the Ghidra C. The seven differences listed
  above were corrected from the raw stream.
- Constants used:
  - `0x80296DD0`/`0x80296F50` record stride `0x30`, maximum `0x400` records.
  - `0x8028CD70` multipliers `0x7B929`, `0x371097E7`, `0x3F81F635`,
    `0x30A74193`, addend `0x4BF53`.
  - `0x80294D60..0x80295090` record header `u16 length` at `+0x00`, name at
    `+0x02`, payload header at `+0x02+length`, next record at `payload+0x08`.
  - `0x80295780` globals `0x80423170`, `0x80423180`, totals `0x8042317C`,
    `0x8042318C`.
- Reproduce this pass: `ledger/agents/wave9-gx.csv` plus the function table
  above.
- `cargo check` and `node tools/doc-lint.mjs src` pass with 0 violations.
