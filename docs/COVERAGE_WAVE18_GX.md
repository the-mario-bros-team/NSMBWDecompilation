# Coverage wave 18 — GX / render band (`0x80260000..0x802B0000`)

## How to read this file

- **Scope** — 25 previously untranslated, non-GQR functions from the
  GX/render lane of `main.dol`, all now in `src/render/gx_band.rs` (ledger
  module `render::gx_band`). The lane is `0x80260000..0x802B0000`; the
  `0x80260000..0x80280000` half is shared with the NW4R resource/archive and
  math lanes, so this pass selects from the render half
  (`0x80285CE0..0x802AFED0`).
- **What this pass adds** — draw-state list release, sum and dispatch loops
  (`0x80285CE0..0x80287410`), four state/scene object helpers
  (`0x80292810..0x80294590`), the two material-name table lookups and the
  record insert (`0x80294CA0..0x80295160`), and six material factory /
  child-dispatch paths (`0x802ABE90..0x802AFED0`).
- **Addresses** are virtual `main.dol` addresses, not file offsets. Image base
  is `0x80004000`; the main text segment maps virtual `0x80006780` to file
  offset `0x2B00` in `work/dol/main.elf`
  (`file = 0x2B00 + (addr - 0x80006780)`).
- **Status** follows `docs/RUST_STYLE.md`: `translated` = entry/exit, every
  call and every observable memory/MMIO write is accounted for; `partial` = a
  known piece is missing; `stub` = address only.
- **Confidence** measures the strength of the evidence, not code quality:
  `high` = the raw instructions and the Ghidra C agree, or the flow is
  instruction-verified; `medium` = the flow is exact but a callee identity or
  virtual-slot role is inferred; `low` = mostly guesswork.
- **Evidence** means the Ghidra C under `work/ghidra/main-gb/export/`, plus a
  raw-instruction check against `work/dol/main.elf`. Every row names its
  Ghidra file and raw range.
- **Start here** — [`fn_80285ce0`](../src/render/gx_band.rs) is the clearest
  draw-state list drain; [`fn_802ad720`](../src/render/gx_band.rs) shows the
  five material type codes; [`fn_80294ca0`](../src/render/gx_band.rs)
  documents the material-name record layout.
- **Companion ledger** — `ledger/agents/wave18-gx.csv`.

## Summary

| module | functions | bytes (`functions.tsv`) | address span | status | confidence |
|---|---|---:|---|---|---|
| `render::gx_band` | 25 | 4,528 | `0x80285CE0..0x802AFED0` | 25 translated, 0 partial, 0 stub | 22 high, 3 medium, 0 low |

Band totals (`work/ghidra/main-gb/export/functions.tsv`, entries
`0x80260000 <= addr < 0x802B0000`):

| Metric | Count |
|---|---:|
| Function entries in band | 881 |
| Translated before wave 18 (all lanes) | 198 |
| Added by wave 18 | 25 |
| Translated now | 223 |
| Coverage of entries | 25.3 % |
| Deferred | 658 |
| Deferred because of paired-single/GQR (`G`) | 104 |
| Deferred because entry `< 16` bytes (`S`) | 98 |
| Deferred for other scope reasons (`R`) | 458 |

Of the 658 deferred entries, 104 contain `ldexpf(` or `unaff_GQR0`, 98 are
smaller than 16 bytes (two entries are in both buckets), and 458 are other
unscoped entries. None of the 25 entries selected here contains `ldexpf(` or
`unaff_GQR0`.

## Function table

Evidence paths are relative to `work/ghidra/main-gb/export/`.

| address | size | Rust function | role | status | confidence | evidence |
|---|---|---:|---|---|---|---|
| `0x80285CE0` | 200 | `fn_80285ce0` | drain the `+0x24` list and release state-1 nodes | translated | high | `80285ce0_FUN_80285ce0.c` |
| `0x80285DB0` | 132 | `fn_80285db0` | sum the `+0x24` list cost | translated | high | `80285db0_FUN_80285db0.c` |
| `0x802862C0` | 144 | `fn_802862c0` | apply `0x80289320` to every `+0x24` node | translated | medium | `802862c0_FUN_802862c0.c` |
| `0x80286430` | 112 | `fn_80286430` | GX state write plus release childless nodes | translated | medium | `80286430_FUN_80286430.c` |
| `0x80286640` | 264 | `fn_80286640` | allocate and clear the `0xC018` bucket array | translated | high | `80286640_FUN_80286640.c` |
| `0x80286810` | 292 | `fn_80286810` | create a node and insert it into one bucket | translated | medium | `80286810_FUN_80286810.c` |
| `0x80286940` | 232 | `fn_80286940` | release state-1 nodes of one bucket | translated | high | `80286940_FUN_80286940.c` |
| `0x80286B50` | 24 | `fn_80286b50` | return a bucket's live count | translated | high | `80286b50_FUN_80286b50.c` |
| `0x80286C10` | 132 | `fn_80286c10` | dispatch virtual `+0x1C` over one bucket | translated | high | `80286c10_FUN_80286c10.c` |
| `0x80286D30` | 68 | `fn_80286d30` | insert a node into a `+0x14/+0x18` list | translated | high | `80286d30_FUN_80286d30.c` |
| `0x802871D0` | 132 | `fn_802871d0` | sum the `+0xC0` list cost | translated | high | `802871d0_FUN_802871d0.c` |
| `0x80287410` | 200 | `fn_80287410` | release state-1 nodes of the `+0xC0` list | translated | high | `80287410_FUN_80287410.c` |
| `0x80292810` | 88 | `fn_80292810` | install vtable `0x8034F618` and clear two lists | translated | high | `80292810_FUN_80292810.c` |
| `0x80292920` | 176 | `fn_80292920` | release state-1 nodes of the `+0x38` list | translated | high | `80292920_FUN_80292920.c` |
| `0x802929D0` | 336 | `fn_802929d0` | initialise a draw-state object from a descriptor | translated | high | `802929d0_FUN_802929d0.c` |
| `0x80294590` | 64 | `fn_80294590` | normalise state-1 nodes from `+0xD8=3` to 1 | translated | high | `80294590_FUN_80294590.c` |
| `0x80294CA0` | 184 | `fn_80294ca0` | material-name lookup in the first table | translated | high | `80294ca0_FUN_80294ca0.c` |
| `0x80294E90` | 184 | `fn_80294e90` | material-name lookup in the second table | translated | high | `80294e90_FUN_80294e90.c` |
| `0x80295160` | 124 | `fn_80295160` | insert and resolve one material-name record | translated | high | `80295160_FUN_80295160.c` |
| `0x802ABE90` | 180 | `fn_802abe90` | notify `+0x20` children and the `+0x28` child | translated | high | `802abe90_FUN_802abe90.c` |
| `0x802AC720` | 204 | `fn_802ac720` | destroy the `+0x04` child list | translated | high | `802ac720_FUN_802ac720.c` |
| `0x802ACB80` | 296 | `fn_802acb80` | destroy a material node and its three children | translated | high | `802acb80_FUN_802acb80.c` |
| `0x802AD000` | 128 | `fn_802ad000` | allocate a `0x20`-byte node and link it | translated | high | `802ad000_FUN_802ad000.c` |
| `0x802AD720` | 488 | `fn_802ad720` | create a material by FourCC type | translated | high | `802ad720_FUN_802ad720.c` |
| `0x802AFED0` | 144 | `fn_802afed0` | dispatch the `+0x100` child array and release | translated | high | `802afed0_FUN_802afed0.c` |

Totals: **25 translated, 0 partial, 0 stubs**; **22 high, 3 medium, 0 low**.

## What the wave-18 pass adds

### 1. Draw-state list loops (`0x80285CE0..0x80287410`)

The band uses one list node layout throughout:

| Offset | Meaning |
|---|---|
| `+0x00` | next pointer |
| `+0x0C` | state: 1 = live, 2 = released |
| `+0x10` | external-reference count |
| `+0x1C` | vtable pointer |
| `+0x40` | bucket index (only for the `0xC018` array) |

`0x80285CE0`, `0x80286940` and `0x80287410` are the same release loop over
three different sentinels (`+0x24`, bucket, `+0xC0`). Each drains the list with
`0x802957D0` into a stack array, then for every state-1 node decrements the
owner's live counter, calls virtual `+0x0C`, stores state 2, and calls virtual
`+8` when `+0x10` is zero. The only differences are the counter offsets
(`+0x3C`, bucket `+0x18`, `+0xD8`) and the drain maximums (`+0x2C`, bucket
`+0x08`, `+0xC8`).

`0x80285DB0` sums `0x802871D0` over the `+0x24` list; `0x802871D0` sums
`0x80292920` over the `+0xC0` list.

`0x80286640` allocates the bucket array: the object's vtable slot `+0x60`
allocates `count*0x1C + 0x20`, `0x802DC7E0` installs the `0x80286750`
constructor with stride `0x1C`, and each bucket gets two `0x14`-byte list
headers plus zeroed pointers and counters. `0x80286B50`/`0x80286C10`/
`0x80286940` then treat those buckets as lists.

`0x80286D30` is the generic insert for the `+0x14/+0x18` doubly-linked list
used by `0x80286D80`'s sort; it is kept separate because it is a complete,
self-contained primitive.

### 2. State and scene objects (`0x80292810..0x80294590`)

`0x80292810` installs vtable `0x8034F618` at `+0x1C`, initialises the two
`0x14`-byte lists at `+0x38`/`+0x44` and clears the `u16` at `+0x50`.
`0x80292920` releases that `+0x38` list using the unusual link offset at
`node + u16[manager+0x42] + 4`.

`0x802929D0` is the largest scalar initialiser in this pass. It clears the
list/count block, writes two `1.0f` and three `0.0f` values (the constants are
`0x8042ECA8` and `0x8042ECAC`), raises the descriptor's reference count at
`+0x10`, copies nine bytes and four floats from the source object, and resolves
one handle through the `+0xBC` chain. It also clears the global index word at
`0x8042AD88` when it matches the object's low 16 bits.

`0x80294590` is the small state-normalisation pass: a live node whose `+0xD8`
value is 3 becomes 1.

### 3. Material-name tables (`0x80294CA0..0x80295160`)

The two lookups share the record layout already documented in
`docs/COVERAGE_WAVE9_GX.md`:

1. header = `table + u32[table]`;
2. `u16` count at `header+0x04`;
3. records start at `header+0x08`;
4. each record is `u16 length` + name bytes + an 8-byte payload header;
5. the first payload word is added to `header` to reach the entry.

`0x80295160` inserts one sub-record into the list at `owner+0x10`, adds the
sub-record's payload size to the byte total at `owner+0x1C`, then calls
`0x80294E00` to resolve the entry pointers.

### 4. Material factory and child dispatch (`0x802ABE90..0x802AFED0`)

`0x802AD720` is the clearest entry point: it compares the incoming FourCC
against `pan1` (`0x70616E31`), `pic1` (`0x70696331`), `txt1` (`0x74787431`),
`wnd1` (`0x776E6431`) and `bnd1` (`0x626E6431`), allocates `0xD8`, `0xF0`,
`0x104`, `0x108` or `0xD8` bytes, and calls `0x802AB550`, `0x802AD920`,
`0x802ADEA0`, `0x802AF6F0` or `0x802B1C90`. Every arm except `pan1` copies
four descriptor words to a local block first.

`0x802AC720` and `0x802ACB80` are the matching destructors: they unlink nodes
through `0x802290C0`, convert a link back to its owning object with `-0x04`,
call virtual `+8(item, -1)`, and free through `0x801D5550`. `0x802ABE90` and
`0x802AFED0` are the dispatch side: one walks a circular `+0x20` list and calls
the child's `+0x1C` slot, the other walks the `+0x100` array and calls `+0x14`.

## Raw/Ghidra differences handled

1. **Hidden arguments behind `0x802DCDxx`.** The CodeWarrior save helpers make
   Ghidra print `void FUN_80285CE0(void)` and then use the low half of the
   helper result as the object. The raw prologue shows `r3` is the manager
   (`lhz r5,44(r3)` / `addi r3,r3,36`), so the Rust takes an explicit
   `manager` argument. The same applies to `0x80286640` (object, count),
   `0x80286940` (manager, index), `0x80287410` and `0x80292920`.
2. **Dropped return values.** Ghidra declares the release loops `void`, but
   the raw epilogue leaves the released count in `r3`; `0x80292810`,
   `0x802AD000` and `0x802AD720` likewise return their object/node. The Rust
   keeps those return values.
3. **`0x80286810` publishes then withdraws a node.** The raw code increments
   the bucket count, sets state 1, then immediately decrements the same count
   and sets state 2. This is reproduced literally; it is not a decompiler
   artefact.
4. **`0x80286640` duplicate store.** The raw stream stores `0` to bucket
   `+0x18` twice (`0x802866D0` and `0x802866FC`). Both stores are kept.
5. **`0x802AD720` descriptor copy order.** The four descriptor words are
   copied before the allocation for the non-`pan1` arms; the Rust keeps that
   order so a failing allocator still observes the same reads.
6. **`0x80286430` GX helper.** `0x801C0500(param_2, object+0x54)` is called
   before the list walk. The helper's name is unknown; only its call shape is
   claimed.
7. **`0x802929D0` float constants.** The C reads `r2-0x43D8`/`r2-0x43D4`;
   the ELF data words are `0x3F800000` (1.0f) and `0` (0.0f). The Rust writes
   those literals and documents the source addresses.
8. **`0x802AC720` node gate.** Ghidra prints the gate at the link pointer
   `piVar4+0xD2`, which would be `object+0xD6`. The raw instruction at
   `0x802AC76C` is `lbz r0,41(r29)` after `r29 = node - 4`, i.e.
   `object+0x29`. The Rust follows the raw byte; `0x802ACB80` really does use
   `object+0xD6`, which is why the two destructors differ.

## Deferred addresses

The full GQR list is unchanged. Entries adjacent to this pass that remain
deferred for scope reasons include:

- `0x80285B50`, `0x80286350`, `0x802864A0` and `0x802874E0` — create/apply
  helpers around the same lists; left because they need their own grid /
  descriptor layout and are not required by the release machinery.
- `0x802925D0` — material alpha scale; still needs the template-curve pass.
- `0x80295400` — three-level texture resolver; too large for this evidence
  budget.
- `0x8029BAB0` — the `F`-bucket GX state readback/emit routine (1,484 bytes,
  69 FIFO writes); still deferred until the GX enum map exists.
- `0x802AB680`, `0x802ACA70`, `0x802ADEA0`, `0x802AF240` and `0x802AF6F0` —
  sibling destructor/search/constructor paths whose descriptor and upload
  bodies need their own pass.
- `0x802AFED0` is translated; `0x802AFF60` (the matrix/uv dispatch, 268 bytes)
  remains deferred because it mixes four float parameters with a virtual call
  and a UV-offset subtraction.

## Uncertainties

1. **Class and field names remain inferred.** The Rust uses offset-based
   descriptions; no Nintendo class name is claimed.
2. **Virtual-slot roles are inferred.** `+0x0C`, `+8`, `+0x14`, `+0x1C` and
   `+0x60` are only known by call shape and neighbouring code.
3. **The three global list objects `0x8042AD78/7C/80` are raw addresses.**
   They already hold vtables installed by `0x80287000`; their human-facing
   names are unknown.
4. **`0x80286430`'s GX helper is opaque.** Only `0x801C0500(param_2,
   object+0x54)` is claimed.
5. **`0x802AD720`'s FourCC names are read from the comparison constants.**
   They match the NW4R `pan1`/`pic1`/`txt1`/`wnd1`/`bnd1` material types, but
   no symbol table confirms them.

## Method and reproducibility

- Scope: `functions.tsv` entries with `0x80260000 <= entry < 0x802B0000`;
  881 entries.
- Selection excluded every address already present in `ledger/functions.csv`
  or in `src/render/gx_band.rs`, every file containing `ldexpf(` or
  `unaff_GQR0`, and every entry smaller than 16 bytes. From the remaining
  render-half candidates, 25 list/state/name/factory functions were selected.
- Each selected entry was disassembled from `work/dol/main.elf` and compared
  instruction-by-instruction with the Ghidra C. The differences listed above
  were corrected from the raw stream.
- Constants used:
  - text mapping `file = 0x2B00 + (addr - 0x80006780)`;
  - `r2 = 0x80433080`; `r2-0x43D8 = 0x8042ECA8` (`1.0f`);
    `r2-0x43D4 = 0x8042ECAC` (`0.0f`);
  - `r13 = 0x8042F680`; `r13-0x48F8 = 0x8042AD88`,
    `r13-0x48B8 = 0x8042ADC8`;
  - `0x80286640` globals `0x8042AD78`, `0x8042AD7C`, `0x8042AD80`.
- Reproduce this pass: `ledger/agents/wave18-gx.csv` plus the function table
  above.
- `cargo check` was run both on the workspace and on an isolated
  `platform + gx_band` crate; `node tools/doc-lint.mjs src` reports 0
  violations.
