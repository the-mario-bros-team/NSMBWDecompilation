# Coverage — GX / render band `0x80280000..0x802B0000`

## How to read this file

This is the status page for the GX/render address lane of the `main.dol`
reconstruction. It answers:

1. **What is in the band?** — *Range inventory*: how many function entries
   Ghidra exported, how many were selected, and how the rest are classified.
2. **What did this pass translate, and where is it?** — *Selected functions*:
   one row per function with its address, Rust name, plain-English role,
   cluster, status, confidence and evidence file.
3. **What was deliberately left out, and why?** — *Deferred inventory*: every
   deferred address with a reason code, plus *Next targets* and
   *Uncertainties* for the next pass.

Column meanings:

- **Status** — `translated` = the entry/exit, every call and every observable
  memory/MMIO write is accounted for; `partial` = a known piece is missing and
  marked `// partial:` in the Rust; `stub` = address only.
- **Confidence** — strength of the evidence, not code quality:
  `high` = the raw instructions and the Ghidra C agree, or the raw stream was
  used directly; `medium` = the flow is explicit but a callee identity, field
  meaning or virtual-slot role is inferred; `low` = mostly guesswork.
- **Reason codes** (deferred inventory):
  - **G** — the Ghidra C contains `ldexpf(...)` and/or `unaff_GQR0`; the
    original uses paired-single (`psq_l`/`psq_st`) with GQR configuration.
    Translating the C would bake in wrong float semantics.
  - **S** — entry is smaller than 16 bytes (veneer, thunk, shared-`blr`
    helper, or empty body). There is no meaningful body to translate.
  - **F** — GX-FIFO state code with no GQR marker, but too large/intertwined
    for this pass's evidence budget.
  - **R** — no GQR marker and size `>= 16`, but not selected by this pass
    (outside the chosen clusters, role unclear, duplicate, or needs a class
    layout that is not yet established).

**Virtual addresses.** All addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`), not file offsets. The main text section
maps virtual `0x80006780` to file offset `0x27C0`, so
`file = 0x27C0 + (addr - 0x80006780)` for the text range. Data addresses in
this doc were read through the ELF program headers of `work/dol/main.elf`.

**Evidence.** The Ghidra C is under `work/ghidra/main-gb/export/`; the function
index is `functions.tsv`. Raw instruction checks used `work/dol/main.elf`.
Inline Rust markers follow `docs/RUST_STYLE.md`: `// observed:` (visible in the
Ghidra output or raw stream), `// inferred:` (role guessed), `// unknown:`
(unresolved), `// partial:` (deliberately incomplete).

**Promotion status.** All 24 translated functions live in
`src/render/gx_band.rs`. There is no staging module for this lane. The
machine-readable ledger is `ledger/agents/gx-band.csv`.

## Range inventory

Source: `work/ghidra/main-gb/export/functions.tsv`, filtered by **entry**
address (`0x80280000 <= addr < 0x802B0000`). No engine-lane skips apply here.

| Metric | Count |
|---|---:|
| Function entries in band | 345 |
| Selected and translated | 24 |
| Deferred | 321 |
| Deferred because of paired-single/GQR (`G`) | 85 |
| Deferred because entry `< 16` bytes (`S`) | 25 |
| Deferred GX-FIFO state code (`F`) | 1 |
| Deferred for other scope reasons (`R`) | 210 |
| Coverage of entries | 7.0 % |

Distribution by 64 KiB block:

| Block | Entries | Selected | Deferred | Deferred `G` |
|---|---:|---:|---:|---:|
| `0x80280000..0x8028FFFF` | 158 | 20 | 138 | 23 |
| `0x80290000..0x8029FFFF` | 85 | 3 | 82 | 28 |
| `0x802A0000..0x802AFFFF` | 102 | 1 | 101 | 34 |
| **Total** | **345** | **24** | **321** | **85** |

The 85 GQR-affected entries are 24.6 % of the band and together account for
99,372 bytes. All 85 contain **both** the bogus `ldexpf(...)` expression and an
`unaff_GQR0` read; the two sets coincide exactly in this band.

## Selected functions

| Address | Rust | Role (plain English) | Cluster | Status | Confidence | Evidence |
|---|---|---|---|---|---|---|
| `0x80280140` | `fn_80280140` | Probe the three draw-queue lists; return the first non-empty `size-4` or 0. | A | translated | high | `80280140_FUN_80280140.c` |
| `0x80280250` | `fn_80280250` | Pop one draw task, run its virtual slot `+0x0C`, wake the queue. | A | translated | medium | `80280250_FUN_80280250.c` |
| `0x802803D0` | `fn_802803d0` | Wait on the queue until it is empty or the stop byte is raised. | A | translated | high | `802803d0_FUN_802803d0.c` |
| `0x80280440` | `fn_80280440` | Set the queue stop byte and wake every sleeper. | A | translated | high | `80280440_FUN_80280440.c` |
| `0x80280490` | `fn_80280490` | Clear the thread's `+0x318/+0x31C/+0x31D` control fields. | A | translated | high | `80280490_FUN_80280490.c` |
| `0x802804B0` | `fn_802804b0` | Stop/detach the active render thread; optionally release it. | A | translated | medium | `802804b0_FUN_802804b0.c` |
| `0x80280530` | `fn_80280530` | Create and resume the render thread (`OSCreateThread`-shaped). | A | translated | medium | `80280530_FUN_80280530.c` |
| `0x80280600` | `fn_80280600` | Render-thread body: wait, pop a task, repeat until stopped. | A | translated | high | `80280600_FUN_80280600.c` |
| `0x80280660` | `fn_80280660` | Initialise a `0x104`-byte scene/texture state object. | B | translated | high | `80280660_FUN_80280660.c` |
| `0x802807E0` | `fn_802807e0` | Set texture-grid size and reset the scalar parameter bank. | B | translated | high | `802807e0_FUN_802807e0.c` |
| `0x80281370` | `fn_80281370` | Set `+0x9D = 1`, clear `+0x9F`, set change bit 1. | B | translated | high | `80281370_FUN_80281370.c` |
| `0x80281460` | `fn_80281460` | Set byte `+0x9F`; raise change bit 2 when changed. | B | translated | high | `80281460_FUN_80281460.c` |
| `0x80281480` | `fn_80281480` | Return child `+0x0C`→`+0x14`, or 1 when there is no child. | B | translated | high | `80281480_FUN_80281480.c` |
| `0x802814A0` | `fn_802814a0` | Store `max(value,0)` at `+0xE8`; raise change bit 8. | B | translated | high | `802814a0_FUN_802814a0.c` |
| `0x802814D0` | `fn_802814d0` | Store the clamped pair at `+0xEC/+0xF0`; raise change bit 8. | B | translated | medium | `802814d0_FUN_802814d0.c` |
| `0x80281570` | `fn_80281570` | Store a signed word at `+0xF4`; raise change bit `0x10`. | B | translated | high | `80281570_FUN_80281570.c` |
| `0x802815B0` | `fn_802815b0` | Store a float at `+0xAC`; raise change bit `0x10`. | B | translated | high | `802815b0_FUN_802815b0.c` |
| `0x80281610` | `fn_80281610` | Store a `[0,1]` float at `+0xB8` and a byte at `+0xA5`; raise bit `0x100`. | B | translated | high | `80281610_FUN_80281610.c` |
| `0x80281680` | `fn_80281680` | Store a `0..=0x7F` byte at `+0xA4`; raise change bit `0x80`. | B | translated | high | `80281680_FUN_80281680.c` |
| `0x802817D0` | `fn_802817d0` | Copy six floats into matrix `index` at `+0x2C+index*0x18`. | B | translated | high | `802817d0_FUN_802817d0.c` |
| `0x8029F4A0` | `fn_8029f4a0` | Emit four transformed points as one `GX_QUADS` primitive. | C | translated | medium | `8029f4a0_FUN_8029f4a0.c` |
| `0x8029F5F0` | `fn_8029f5f0` | Transform one point through `0x801C1160` into `out`; return `out`. | C | translated | medium | `8029f5f0_FUN_8029f5f0.c` |
| `0x8029F8F0` | `fn_8029f8f0` | Return `object + 0x9C` (two-instruction accessor). | C | translated | high | `8029f8f0_FUN_8029f8f0.c` |
| `0x802A18E0` | `fn_802a18e0` | Instruction-for-instruction twin of `0x8029F4A0`. | C | translated | medium | `802a18e0_FUN_802a18e0.c` |

Totals: **24 translated, 0 partial, 0 stubs**; **17 high, 7 medium, 0 low**.

## Clusters

### Cluster A — draw queue and render thread (`0x80280140..0x80280600`)

Eight functions around one worker thread and one global draw queue.

The draw queue is the singleton returned by `0x8027FDD0` (lazily built at
`0x80416FC4`). Its three list headers are at `+0x00`, `+0x0C` and `+0x18`
(each a count word followed by a link/tail word). `+0x24` holds the task being
released, `+0x2C` is the sleep condition and `+0x34` is the wake queue.

The thread context is a different object. It has a stop byte at `+0x28`, a
sleep condition at `+0x2C`, a stack/base word at `+0x318`, a stop request at
`+0x31C` and an "active" byte at `+0x31D`.

Flow:

1. `fn_80280530` creates the thread with entry `0x80280600` and argument
   `thread` itself. The `OSCreateThread`-shaped argument order is
   `(thread, entry, arg, stack_top, stack_size, priority, attr)`; the
   translation names the last three `stack_base`, `stack_size`, `priority`
   but marks the naming inferred.
2. `fn_80280600` loops: check `+0x31C`, wait on the queue, check `+0x31C`,
   pop one task.
3. `fn_802803d0` is the wait predicate loop; `fn_80280140` is the predicate.
4. `fn_80280250` pops one task through `0x8027FFE0`, runs its virtual slot
   `+0x0C`, clears the queue's current-task word and wakes sleepers.
5. `fn_80280440` raises the stop byte and wakes sleepers.
6. `fn_802804b0` stops/detaches an active thread; `fn_80280490` clears the
   three control fields.

Two decompiler pitfalls are handled explicitly:

- Ghidra prints the inner interrupt restores in `0x80280140` with no argument.
  The raw stream keeps the per-list `OSDisableInterrupts` result in `r3`; the
  Rust restores that value, and only the outermost restore uses the outer
  state.
- Ghidra prints `0x8027FDD0()` and `0x80280440()` with no arguments in
  `0x802804B0`/`0x80280530`/`0x80280600`. The raw stream uses the *return
  value* of `0x8027FDD0` (the queue singleton) as `r3`, not the thread
  context. The Rust reloads the singleton at each call site.

### Cluster B — `0x104`-byte scene/texture state object (`0x80280660..0x802817D0`)

Twelve functions that construct and mutate one object. The allocator
`0x80282FF0` carves a run of `0x104`-byte chunks and calls `0x80280660` on
each, then links the chunks through `+0xFC`; that is the evidence for the
object size.

Field map (inferred from this cluster; no Nintendo class name is claimed):

| Offset | Meaning |
|---|---|
| `+0x00` / `+0x04` | two list links, cleared by the constructor |
| `+0x08` | vtable pointer `0x8034F274` |
| `+0x0C..+0x28` | eight cleared words (three list headers) |
| `+0x2C + i*0x18`, `i=0..3` | four six-float (`2x3`) matrices |
| `+0x8C` / `+0x90` | texture-grid rows / columns |
| `+0x94` / `+0x98` | callback / callback argument |
| `+0x9C..+0xA2` | control bytes and 16-bit change mask |
| `+0xA4` | byte parameter (clamped to `0..=0x7F` by `fn_80281680`) |
| `+0xA5` | index/byte parameter |
| `+0xAC..+0xB8` | scalar float parameters |
| `+0xBC..+0xF8` | integer/float parameter bank |
| `+0xFC` / `+0x100` | allocator list links |

The change-mask bits observed in this cluster are `1`, `2`, `4`, `8`, `0x10`,
`0x20`, `0x3C`, `0x80` and `0x100`. The setters reproduce the exact bit each
one raises; their consumer is outside this module.

The texture-grid evidence comes from neighbours that were **not** selected:
`0x80280E60` allocates `rows * columns` texture handles and stores them in the
grid at `+0x0C`, `0x802810A0` frees them, and `0x80281170`/`0x80281390` apply
or release handles. Those functions are deferred because their texture-API
callees (`0x80264AE0`, `0x80264CD0`, `0x80264F90`, `0x802627C0`, …) are not yet
identified.

One Ghidra control-flow defect and one load-order detail forced raw
reconstruction in this cluster:

- `fn_802814d0`: Ghidra prints a one-field arm that is unreachable after the
  clamp; raw `0x802814F0..0x80281544` always stores both fields.
- `fn_802817d0`: the six-float copy is correct in Ghidra, but the raw stream
  loads all six values before storing; the Rust preserves that order so
  aliasing `src` with the destination still copies the original values.

### Cluster C — GX vertex emission (`0x8029F4A0..0x802A18E0`)

Four functions that touch the GX write-gather FIFO at `0xCC008000`.

`fn_8029f4a0` (and its twin at `0x802A18E0`) is the clearest GX-FIFO function
in the band:

1. Four calls to `0x801C1160(matrix, points + i*12, scratch_i)`.
2. `0x801C5570(0x80, 0, 4)` — `GXBegin` with `GX_QUADS` and four vertices.
3. For each scratch buffer: three `stfs` to `0xCC008000`.
4. If `flags & 1`, one `stb` to `0xCC008000` with the vertex index `0..3`.

The stock Ghidra C collapses this to one float and a `CONCAT13(3, ...)` store.
The raw stream at `0x8029F4FC..0x8029F5B8` shows four separate 12-byte output
buffers, three `stfs` per vertex, and the optional `stb`. The translation
follows the raw instructions. The optional byte is marked
`// inferred:` a per-vertex position-matrix index because the emitted values
are exactly `0..3`, but no GX attribute name is claimed.

`0x801C1160` itself is **not** translated. Its raw body contains
`psq_l`/`psq_st` with GQR0 (`0x801C1160..0x801C11A8`), so its Ghidra C is a
paired-single mis-render. The four functions above only reproduce the call and
the FIFO writes; the transform is an opaque primitive.

`fn_8029f5f0` is the one-point wrapper around the same helper:
`0x801C1160(matrix, point, out)`, then return `out`. `fn_8029f8f0` is the
`object + 0x9C` accessor used by the surrounding G3D code; it is only eight
bytes but is a real function, not a thunk, so it was admitted despite the usual
16-byte floor.

## Deferred inventory

Every deferred address in the band is listed below with its reason code. The
code definitions are in *How to read this file*. The lists are sorted by
address.

### `G` — paired-single / GQR (85)

These entries contain both `ldexpf(...)` and `unaff_GQR0` in the Ghidra C.
The raw stream uses `psq_l`/`psq_st` and GQR-configured fixed-point loads;
the C float expressions are not trustworthy. Do not translate them until a
GQR-aware pass (or a raw-instruction reconstruction) exists.

**`0x80280000..0x8028FFFF` (23):**
`80280910`, `80280BA0`, `80281C90`, `802821E0`, `802850F0`, `80286CA0`,
`80287C10`, `80288390`, `80288490`, `80288E30`, `80289410`, `8028A6F0`,
`8028B6C0`, `8028C7A0`, `8028CEE0`, `8028DC70`, `8028DFC0`, `8028E310`,
`8028E890`, `8028ED30`, `8028F2D0`, `8028F860`, `8028FEA0`.

**`0x80290000..0x8029FFFF` (28):**
`80290690`, `80290CF0`, `80291100`, `80291A10`, `80292C90`, `802941F0`,
`802946F0`, `80295860`, `80295A90`, `80295C20`, `802960F0`, `80296160`,
`802961E0`, `80296260`, `802963B0`, `80296530`, `802965B0`, `802966F0`,
`80296820`, `80298720`, `80298B80`, `80298ED0`, `80299510`, `80299840`,
`8029C820`, `8029CFC0`, `8029F640`, `8029F900`.

**`0x802A0000..0x802AFFFF` (34):**
`802A08A0`, `802A0910`, `802A09F0`, `802A0A40`, `802A0E50`, `802A1090`,
`802A2DE0`, `802A3160`, `802A35C0`, `802A3A20`, `802A3E00`, `802A40C0`,
`802A4400`, `802A47C0`, `802A5810`, `802A64B0`, `802A6850`, `802A6CB0`,
`802A6F80`, `802A6FB0`, `802A71D0`, `802A7CA0`, `802A8020`, `802A8440`,
`802A8930`, `802A8960`, `802A8A10`, `802A8D10`, `802A90C0`, `802A9640`,
`802AA830`, `802AE9C0`, `802AF080`, `802AF4F0`.

Raw checks performed on representative entries:

- `0x8029F640` — explicit `psq_l`/`psq_st` quad emitter with `ldexpf` arms;
  the C also has 20 "Removing unreachable block" warnings. Deferred.
- `0x802A3A20` — explicit `psq_l`/`psq_st` vertex loads and stores plus GQR
  scaling; deferred.
- `0x802A47C0` — explicit `psq_l`/`psq_st` triple transform and `GXBegin`;
  deferred.
- `0x802A71D0` — explicit `psq_st` register saves in the prologue; the worst
  warning file in the image (144 removed blocks); deferred.

### `S` — entry `< 16` bytes (25)

These are veneers, jump/branch thunks, empty bodies, or shared-`blr` helpers.
They have no meaningful behaviour to translate.

**`0x80280000..0x8028FFFF` (9):**
`80282FE0`, `80283BC0`, `80284040`, `80284050`, `80284B00`, `80284B10`,
`802862A0`, `802862B0`, `80286630`.

**`0x80290000..0x8029FFFF` (5):**
`802946E0`, `80294B20`, `80296DC0`, `8029F5E0`, `8029F630`.

**`0x802A0000..0x802AFFFF` (11):**
`802A0650`, `802A0930`, `802A0940`, `802A3120`, `802A3130`, `802A35B0`,
`802A7C90`, `802A7FF0`, `802A8000`, `802A8010`, `802AF070`.

The selected `0x8029F8F0` is also eight bytes, but it is a real
`addi r3,r3,0x9C; blr` accessor, not a thunk; it is therefore counted in the
selected set, not here.

### `F` — GX-FIFO state code, no GQR (1)

- **`0x8029BAB0`** (1,484 bytes). Emits 69 writes to `0xCC008000` and calls 17
  GX getter/state helpers (`0x801C3E20`, `0x801C44A0`, `0x801C5570`,
  `0x801C86C0`, …). It has no `ldexpf`/`unaff_GQR0`, but the body is a large
  TEV/state readback-and-emit routine with many inferred GX enum values.
  Deferred until the GX state API is mapped.

### `R` — remaining entries (210)

No GQR marker and size `>= 16`, but not selected. Most are G3D/scene helpers
whose class layout or texture API is not yet established; some are duplicates
or multi-hundred-byte dispatchers. This pass did **not** claim to verify them
individually.

**`0x80280000..0x8028FFFF` (106):**
`802802D0`, `80280870`, `80280E60`, `802810A0`, `80281170`, `80281390`,
`80281550`, `80281590`, `802815D0`, `802815F0`, `802816C0`, `802816E0`,
`80281710`, `80281750`, `80281790`, `80281820`, `802818C0`, `80281950`,
`802819D0`, `802819F0`, `80281A70`, `80281AF0`, `80281B80`, `80281C00`,
`80281D80`, `80281E70`, `80281EF0`, `80282040`, `80282EF0`, `80282F70`,
`80282FF0`, `802830A0`, `80283260`, `80283300`, `802833B0`, `80283420`,
`80283530`, `802835B0`, `80283760`, `802837C0`, `802838A0`, `80283940`,
`802839D0`, `80283A30`, `80283A60`, `80283A90`, `80283B10`, `80283BA0`,
`80283BD0`, `80283DA0`, `80283E10`, `80283EC0`, `80283F60`, `80284210`,
`802842A0`, `802843C0`, `802844E0`, `802845E0`, `80284650`, `80284700`,
`80284910`, `80284F30`, `80285500`, `80285660`, `80285810`, `80285880`,
`802858E0`, `80285B50`, `80285C50`, `80285CE0`, `80285DB0`, `802862C0`,
`80286350`, `80286430`, `802864A0`, `802865A0`, `80286640`, `80286750`,
`80286810`, `80286940`, `80286A30`, `80286AC0`, `80286B50`, `80286B70`,
`80286C10`, `80286D30`, `80286D80`, `80287000`, `80287130`, `80287190`,
`802871D0`, `80287410`, `802874E0`, `802877C0`, `80287930`, `802888E0`,
`80289070`, `802891C0`, `802892D0`, `80289320`, `8028CD70`, `8028E430`,
`8028E510`, `8028E5E0`, `8028E6F0`, `8028E7E0`.

**`0x80290000..0x8029FFFF` (49):**
`802919A0`, `802919B0`, `802925D0`, `80292810`, `80292920`, `802929D0`,
`80292B20`, `80292B60`, `80294440`, `80294520`, `80294590`, `802945E0`,
`80294B30`, `80294CA0`, `80294D60`, `80294E00`, `80294E90`, `80294F50`,
`80294FD0`, `80295090`, `80295160`, `802951E0`, `80295300`, `80295400`,
`80295640`, `802956E0`, `80295780`, `802957D0`, `80295970`, `80295C00`,
`80296640`, `80296750`, `80296760`, `80296780`, `802967B0`, `802967E0`,
`80296CE0`, `80296DD0`, `80296F50`, `802970D0`, `8029B290`, `8029B340`,
`8029B420`, `8029B710`, `8029C080`, `8029D2C0`, `8029ED70`, `8029F460`.

**`0x802A0000..0x802AFFFF` (56):**
`802A0660`, `802A0830`, `802A0870`, `802A0890`, `802A0950`, `802A0970`,
`802A0A90`, `802A0E00`, `802A1050`, `802A16D0`, `802A1860`, `802A18A0`,
`802A22D0`, `802A2900`, `802A2DA0`, `802A3140`, `802A6470`, `802A88E0`,
`802A8990`, `802A89D0`, `802AB490`, `802AB4C0`, `802AB4F0`, `802AB550`,
`802AB680`, `802AB7C0`, `802AB810`, `802ABE90`, `802AC020`, `802AC080`,
`802AC1E0`, `802AC2E0`, `802AC340`, `802AC460`, `802AC480`, `802AC550`,
`802AC720`, `802AC7F0`, `802AC830`, `802AC8B0`, `802ACA70`, `802ACB40`,
`802ACB80`, `802AD000`, `802AD080`, `802AD520`, `802AD720`, `802AD920`,
`802ADEA0`, `802AEDA0`, `802AEE70`, `802AEFA0`, `802AF240`, `802AF6F0`,
`802AFED0`, `802AFF60`.

Notable `R` entries inspected and left for a later pass:

| Address | Why it is not in this module |
|---|---|
| `0x802802D0` | Draw-queue removal loop with vtable slot `+0x14` and `+0x10`; uses the `0x802DCD6C` save/restore stub. The control flow is readable but the queue's three-list unlink helper `0x802290C0` is out of lane. |
| `0x80280E60` | Texture-grid allocation (`rows × columns` handles) with `0x80264AE0`/`0x80264CD0`/`0x80264F90`; texture-API identities are not established. |
| `0x802810A0` | Frees the same grid and removes the object from the `0x80282EF0` list; same texture-API dependency. |
| `0x80281170` | Applies textures and six-float matrices per grid cell; calls `0x80261DF0`, `0x802629F0`, `0x80262E90`, `0x80263110`, `0x80262DB0`, `0x802627E0`. |
| `0x80281390` | Releases grid handles through `0x801A3130`; the callee's role is unresolved. |
| `0x80281820` / `0x802818C0` | Set/clear the texture format on every grid cell (`0x802627C0` with `0x0F`/`0x10`). |
| `0x80281950` | Walk one grid row and call `0x80263110(cell, arg)`. |
| `0x80282FF0` / `0x802830A0` | Pool allocator and pool user for the `0x104`-byte objects; class layout is only partially known. |
| `0x8029F460` | 60-byte helper around the GX state cluster; not enough evidence yet. |

## Promotion status

| Cluster | Functions | Destination file | Ledger `module` |
|---|---:|---|---|
| A — draw queue / render thread | 8 | `src/render/gx_band.rs` | `render::gx_band` |
| B — `0x104`-byte state object | 12 | `src/render/gx_band.rs` | `render::gx_band` |
| C — GX vertex emission | 4 | `src/render/gx_band.rs` | `render::gx_band` |

- `src/render/gx_band.rs` is the final semantic home; there is no staging
  module.
- `ledger/agents/gx-band.csv` has one row per translated function.
- `src/render/mod.rs` already declared `pub mod gx_band;` before this pass; it
  was not edited.

## Next targets

Ordered by value, not address.

| Target | Why |
|---|---|
| `0x8029F640` (412 B) | The explicit `psq_l`/`psq_st` quad emitter. A GQR-aware reconstruction would unlock the whole `0x8029F4A0..0x802A18E0` family and the `0x802A3A20`/`0x802A47C0`/`0x802A6CB0`/`0x802A6FB0`/`0x802A9640` vertex paths. |
| `0x802A71D0` (2,748 B) | Worst warning file in the image (144 removed blocks); a GQR-aware pass should start here because it is the best stress test. |
| `0x8029BAB0` (1,484 B) | GX state readback/emit with 69 FIFO writes and no GQR. Needs a GX enum/state map, not paired-single work. |
| `0x802802D0` (240 B) | Draw-queue removal; would complete cluster A. Needs the three-list unlink helper `0x802290C0` and the save/restore-stub convention documented in `docs/COVERAGE_r80260_802ed.md`. |
| `0x80280E60` / `0x802810A0` / `0x80281170` / `0x80281390` | Texture-grid allocate/free/apply; would explain the `0x104`-byte object's grid and the `+0x8C/+0x90` fields. |
| `0x80282FF0` / `0x802830A0` | Pool allocator/user; would confirm the `0x104` stride and the `+0xFC` links. |
| `0x80299840` (6,316 B) | Largest GQR function in the band; defer until the GQR pass is available. |

## Uncertainties

1. **No symbol table.** Class names, virtual-slot names and field names are
   inferred from layout and call shape. The module uses address-based function
   names and offset-based field descriptions rather than inventing Nintendo
   symbols.
2. **The GQR callee is opaque.** `0x801C1160` uses `psq_l`/`psq_st` and GQR0;
   its Ghidra C is not trustworthy. Cluster C reproduces the call and FIFO
   writes but not the transform arithmetic. The optional per-vertex byte is
   inferred to be a position-matrix index from its values `0..3`.
3. **Two Ghidra mis-renders were corrected from raw bytes:**
   `0x802814D0` (dead one-field arm after the clamp) and
   `0x8029F4A0`/`0x802A18E0` (four output buffers and three `stfs` per vertex,
   not one float + `CONCAT13`).
4. **`0x802B9280` is release/free-shaped.** Its exact allocator pairing is not
   proven; `fn_802804b0` calls it only when its second argument is positive.
5. **`0x80280530` argument names are inferred.** The register order matches
   `OSCreateThread`, but the stack word is passed as `stack_base + stack_size`
   (top of stack), not `stack_base`; the Rust names reflect that.
6. **The `0x104`-byte object's field meanings are inferred.** The texture-grid
   fields `+0x8C/+0x90` are backed by the allocator/free neighbours; the
   purpose of the `+0xBC..+0xF8` bank and the change-mask bits is not yet
   confirmed by a consumer in this lane.
7. **`0x80280140`'s inner restores use the per-list interrupt state.** The
   Ghidra C omits those arguments. The raw instructions keep the state in
   `r3`; the Rust restores it. A caller that relies on the cached outer state
   across the nested pair would observe a difference only if the OS primitive
   is not idempotent, which is unlikely but not proven.
8. **The 85-entry GQR list is a syntactic classification.** It is exact for
   the stock export (`ldexpf` + `unaff_GQR0`), but a future GQR-aware
   decompilation may show that a few of those entries are ordinary scalar
   float code that merely shares the pattern.

## Method and reproducibility

- Scope: `functions.tsv` entries with `0x80280000 <= entry < 0x802B0000`;
  345 entries, no engine-lane skips.
- Selection: read the Ghidra C, reject any entry containing `ldexpf` or
  `unaff_GQR0`, then read the raw ELF for every function in the three chosen
  clusters. Cluster C and the three corrected setters were reconstructed from
  the raw instruction stream rather than the C.
- Raw checks: `work/dol/main.elf` is ELF32 big-endian with program headers
  only (no section headers). Text virtual `0x80006780` maps to file `0x27C0`.
  Data was read through the program headers (`0x8042B080` at file `0x350200`).
- Constants read for this pass:
  - `r2 = 0x80433080`;
  - `r2 - 0x4574 = 0x8042EB0C` = `0.0f`;
  - `r2 - 0x4578 = 0x8042EB08` = `1.0f`;
  - the vtable installed by `0x80280660` is at `0x8034F274`.
- Reproduce the GQR set: for every band file, grep for `ldexpf(` or
  `unaff_GQR0`. The two sets coincide at 85 files in this export.
- Reproduce the selected set: `ledger/agents/gx-band.csv` plus the table above.
- `cargo check` and `node tools/doc-lint.mjs src` pass. The module was also
  checked in isolation with a temporary `platform + gx_band` crate to confirm
  that its `no_std` imports and raw-pointer helpers stand on their own.
