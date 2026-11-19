# Coverage: wave 11 geometry band (`0x80005000..0x8000C000`)

## How to read this file

This document covers the wave-11 geometry batch for the
`0x80005000..0x8000C000` analysis band of `main.dol`. All **24 new bodies**
live in `src/render/geometry.rs`; no other Rust file was changed by this lane.

Every row describes one Ghidra entry point:

- **address** — virtual address of the entry in `main.dol` (image base
  `0x80004000`, so these are *not* file offsets);
- **name** — the Rust function in `src/render/geometry.rs`;
- **role** — one plain sentence;
- **status** — `translated` means the recorded control flow is present, with
  calls outside the batch left as absolute-address calls; `partial` would mean
  a call cannot be reproduced exactly (none here); `stub` is not used;
- **confidence** — `high` when every load/store/branch was read directly from
  the Ghidra C *and* the raw instruction words; `medium` when a field meaning,
  argument role or callee contract is inferred;
- **evidence** — the Ghidra C file under `work/ghidra/main-gb/export/`.

Inside the Rust file, `// observed:` marks facts read from the instruction
stream, `// inferred:` marks roles reconstructed from call sites, and
`// unknown:` marks unresolved details. Each function's `# Original` block
repeats the Ghidra name, address, size and evidence path.

Addresses in this document and in `ledger/agents/wave11-geometry.csv` are
written as plain eight-digit hex (`8000aa70`), matching the task's convention.

## Scope and counts

| Metric | Count |
|---|---:|
| Ghidra entries in `[0x80005000, 0x8000C000)` | 37 |
| entries `< 16` bytes (thunks / stubs, skipped) | 7 |
| entries already translated before this wave | 2 (`0x8000AE50`, `0x8000B320`) |
| eligible untranslated entries (`>= 16` bytes) | 28 |
| **new bodies in this batch** | **24** |
| bytes of Ghidra function size | 3,324 |
| `translated` | 24 |
| `partial` | 0 |
| `stub` | 0 |
| confidence `high` | 22 |
| confidence `medium` | 2 |
| destination `src/render/geometry.rs` | 24 |

The 7 skipped sub-16-byte entries are `0x80006790` (4 B), `0x800067A0`
(4 B tail thunk), `0x80006A40` (12 B), `0x80006BE0` (8 B tail thunk into
`0x80006B60`), `0x80006EA0` (4 B tail thunk), `0x80007220` (8 B) and
`0x80007510` (4 B tail thunk into `0x80164FD0`).

## Selection method

1. `work/ghidra/main-gb/export/functions.tsv` was filtered to
   `0x80005000 <= entry < 0x8000C000`, giving 37 entries.
2. Each address was crossed against `ledger/functions.csv`. Only
   `0x8000AE50` and `0x8000B320` had a row (both in this file); the other 35
   had no ledger row, so nothing was duplicated and no row was created without
   a body.
3. Every candidate was read in its Ghidra C form and then checked against the
   big-endian instruction words in `work/dol/main.elf`. This caught four
   Ghidra artefacts (see "Where Ghidra and the bytes disagree").
4. The batch keeps the band's geometry content: scene sub-object attachment,
   screen-rect setup, named-material registration, and the object-construction
   cluster the earlier lane's gap report ranked as the next work item.

The band's biggest untranslated entry, `0x80007DE0` (1,632 B), was deliberately
**not** taken: it is a GQR/paired-single effect setup where every float crosses
a quantisation register, and the pseudocode is known-low-fidelity. It is listed
under "Left for a later pass" below with the reason.

## Functions translated

| # | Address | Ghidra | Size | Rust | Role | Status | Conf. | Evidence |
|---:|---|---|---:|---|---|---|---|---|
| 1 | `80006870` | `FUN_80006870` | 52 | `fn_80006870` | Queues a one-byte message for the handler at `0x80006850`. | translated | high | `80006870_FUN_80006870.c` |
| 2 | `800068b0` | `FUN_800068B0` | 120 | `fn_800068b0` | Constructs the `0x802FE400` object and installs its `0x802FE410` member. | translated | high | `800068b0_FUN_800068b0.c` |
| 3 | `800069a0` | `FUN_800069A0` | 128 | `fn_800069a0` | Deleting destructor for that object; tears down `+0x30` and `+0x0C`. | translated | high | `800069a0_FUN_800069a0.c` |
| 4 | `80006a20` | `FUN_80006A20` | 20 | `fn_80006a20` | Tail-dispatches virtual slot `+0x0C` of the member at `+0x0C`, `r6 = 0`. | translated | high | `80006a20_FUN_80006a20.c` |
| 5 | `80006aa0` | `FUN_80006AA0` | 76 | `fn_80006aa0` | Constructs the `0x802FE3F0` extension (vtable, `+0xBC`, `+0xC0`). | translated | high | `80006aa0_FUN_80006aa0.c` |
| 6 | `80006af0` | `FUN_80006AF0` | 104 | `fn_80006af0` | Deleting destructor for the `0x802FE3F0` object. | translated | high | `80006af0_FUN_80006af0.c` |
| 7 | `80006b60` | `FUN_80006B60` | 120 | `fn_80006b60` | Creates the resource behind `+0xBC` through the global allocator. | translated | high | `80006b60_FUN_80006b60.c` |
| 8 | `80006bf0` | `FUN_80006BF0` | 96 | `fn_80006bf0` | Releases `+0xBC` through the global vtable and tears down `+0xC0`. | translated | high | `80006bf0_FUN_80006bf0.c` |
| 9 | `80006c50` | `FUN_80006C50` | 120 | `fn_80006c50` | Global factory that builds and stores the object at `r13-0x5AC0`. | translated | high | `80006c50_FUN_80006c50.c` |
| 10 | `80006cd0` | `FUN_80006CD0` | 188 | `fn_80006cd0` | Constructs the `0x802FE3D8` object (two members, three float fields). | translated | high | `80006cd0_FUN_80006cd0.c` |
| 11 | `80006df0` | `FUN_80006DF0` | 64 | `fn_80006df0` | Bare deleting destructor (`free` only when `free_flag > 0`). | translated | high | `80006df0_FUN_80006df0.c` |
| 12 | `80006e30` | `FUN_80006E30` | 108 | `fn_80006e30` | Deleting destructor for the `0x802FE3D8` object. | translated | high | `80006e30_FUN_80006e30.c` |
| 13 | `80006f70` | `FUN_80006F70` | 36 | `fn_80006f70` | Tail-dispatches virtual slot `+0x38` with the low bit of `+0xA4`. | translated | high | `80006f70_FUN_80006f70.c` |
| 14 | `80006fa0` | `FUN_80006FA0` | 108 | `fn_80006fa0` | Copies four floats from `+0x10` into `+0x64..+0x70`, then calls slot `+0x30`. | translated | high | `80006fa0_FUN_80006fa0.c` |
| 15 | `80007300` | `FUN_80007300` | 24 | `fn_80007300` | Tail-dispatches virtual slot `+0x3C` of the child at `+0x20`. | translated | high | `80007300_FUN_80007300.c` |
| 16 | `80007320` | `FUN_80007320` | 148 | `fn_80007320` | Returns the collection if node `r13-0x48A8` is on its list. | translated | high | `80007320_FUN_80007320.c` |
| 17 | `800073d0` | `FUN_800073D0` | 148 | `fn_800073d0` | Same search against node `r13-0x48B0`. | translated | high | `800073d0_FUN_800073d0.c` |
| 18 | `80007470` | `FUN_80007470` | 148 | `fn_80007470` | Same search against node `r13-0x48A0`. | translated | high | `80007470_FUN_80007470.c` |
| 19 | `80007520` | `FUN_80007520` | 172 | `fn_80007520` | Opens a `1000x1000` GX surface and writes its rect/UV quad. | translated | medium | `80007520_FUN_80007520.c` |
| 20 | `800075d0` | `FUN_800075D0` | 64 | `fn_800075d0` | Byte-identical twin of `fn_80006df0`. | translated | high | `800075d0_FUN_800075d0.c` |
| 21 | `80007610` | `FUN_80007610` | 396 | `fn_80007610` | Registers the twenty small-data entries in a render resource. | translated | medium | `80007610_FUN_80007610.c` |
| 22 | `8000aa70` | `FUN_8000AA70` | 220 | `fn_8000aa70` | Validates an owner, attaches one 2-float point, selects state `6`/`7`. | translated | high | `8000aa70_FUN_8000aa70.c` |
| 23 | `8000aec0` | `FUN_8000AEC0` | 220 | `fn_8000aec0` | Twin of `fn_8000aa70` at a second entry point. | translated | high | `8000aec0_FUN_8000aec0.c` |
| 24 | `8000b370` | `FUN_8000B370` | 444 | `fn_8000b370` | Attaches two 2-float points, selects a mode, records `+0xCE90..+0xCE9A`. | translated | high | `8000b370_FUN_8000b370.c` |

## Clusters

### A. Scene sub-object attachment — `8000aa70`, `8000aec0`, `8000b370`

All three share one skeleton, recovered from the instructions rather than the
C:

```
if (0x80164FD0(owner, key, 0) == 0) return 0;
0x802C7C70(owner + 0x08, &point_a, copy, flip, 1, 1);
[0x802C7C70(owner + 0x28, &point_b, copy, flip, 1, 1);]
if (state < 0) { 0x8016A2A0(owner, 0, state_value); 0x8016A0F0(owner, 6, 1); }
else           { 0x8016A2A0(owner, state, 0);        0x8016A0F0(owner, 7, 1); }
return 1;
```

`fn_8000b370` is the rich version: `mode == 0` flips neither sub-object,
`mode == 4` flips the first, any other value flips the second; afterwards it
stores `0x44C` at `+0xCE90`, the mode at `+0xCE94` and two zero halfwords at
`+0xCE98`/`+0xCE9A`. `0x44C` is 1100 — `// unknown:` it is a state id or a
capacity, not identified.

The 2-word temporary that receives each point has only its **first** word
written by the original (`lwz r0,0(r28)` → `stw r0,28(r1)`). The Rust keeps
that shape with `core::mem::MaybeUninit`, so the second component stays
unspecified exactly as it does on the shipped stack frame.

### B. Screen rectangle and material registration — `80007520`, `80007610`

`fn_80007520` is the only function in the batch that touches the renderer's
surface state: `0x801645F0(arg, 1, &word, 1000, 1000, 0x80, 0x80, 2, 2)`,
then `0x80164B20(0x1F)`, then `0x80253C70` with
`(0.5, -0.5, -0.5, 0.5, 0.0, 1.0)` and `0x80253D00` with
`(0.5, 0.5, 0.5, 0.5)`, then `0x801644F0(0)`. The float constants come from
`0x8042B098`, `0x8042B09C`, `0x8042B0A0` and `0x8042B0A4`. Confidence is
`medium` because the *meaning* of the nine open arguments is unresolved, not
because any instruction is missing.

`fn_80007610` forwards its six arguments to `0x80164770`, resolves the resource
with `0x80164B70`, then brackets twenty `0x802CD050` calls with `0x802B9170`.
The twenty entries are consecutive 8-byte slots from `0x80427688`
(`r13-0x7FF8`) to `0x80427720` (`r13-0x7F60`). `0x802CD050`'s own export shows
`(void)` and drops both arguments, but its body reads `r3` as the table and
`r4` as the name and compares it with `strcmp`-shaped `0x802E1AB4`; a missing
entry is allocated (0x80 bytes) and linked in. The pass therefore reads as
"register twenty named materials".

### C. Object construction and destruction — `80006870..80006f70`

Two object families:

| family | vtable(s) | members | functions |
|---|---|---|---|
| `0x802FE400` / `0x802FE410` | `0x802FE400` at `+0`, `0x802FE410` at `+0x0C` | none extra | `800068b0`, `800069a0`, `80006a20` |
| `0x802FE3F0` extension | `0x802FE3F0` at `+0` | `0x8008F140` at `+0xC0`, handle at `+0xBC` | `80006aa0`, `80006af0`, `80006b60`, `80006bf0` |
| `0x802FE3D8` / `0x803294D8` | `0x802FE3D8` at `+0x08`, `0x803294D8` at `+0x10` | `0x802B4D30` at `+0x30` | `80006cd0`, `80006e30` |
| headerless | — | — | `80006c50`, `80006df0`, `800075d0`, `80006f70`, `80006fa0` |

The `0x802FE400` family is a three-level construction: the base constructor
writes `+0x04`/`+0x08` to zero, installs `0x802FE400`, builds the `+0x0C`
member with `0x802B6620` and re-vtables it to `0x802FE410`, then calls virtual
slot `+0x0C`. The destructor reverses that with `0x80229030(this+0x30, 0)`,
`0x802B63B0(this+0x0C, 0)` and an optional `0x802B9280(this)`.

`80006c50` is the only function here that writes a **global**: it builds a
0x20-byte description with `0x8016E500`, allocates 0x1C bytes with
`0x802B9240`, constructs with `0x8016A5E0`, stores the result at
`r13-0x5AC0` (`0x80429BC0`) and finishes with `0x8016A690`. That global is the
same one whose `+0x14` slot supplies the allocator in `80006b60` and whose
`+0x10` vtable slot releases it in `80006bf0`, so the four functions form one
closed resource-owner subsystem.

Class identities are `// unknown:` — the image carries no RTTI names for these
vtables, so the Rust keeps the `fn_<address>` names.

### D. Global collection lookups — `80007300`, `80007320`, `800073d0`, `80007470`

```
child = virtual_slot_3C(this->+0x20, arg, 1);
if (child == 0) return 0;
for (node = virtual_slot_0C(child); node; node = *node)
    if (node == ANCHOR) return child;
return 0;
```

The three anchors are adjacent small-data words: `0x8042ADD8`
(`r13-0x48A8`), `0x8042ADD0` (`r13-0x48B0`) and `0x8042ADE0`
(`r13-0x48A0`). They are zero-initialised at rest (the addresses fall in the
`memsz > filesz` part of the data segment), i.e. they are the heads of three
linked lists that are built at run time.

## Cross-lane callees

Every callee below is called by absolute address from
`src/render/geometry.rs` and documented at its constant.

| callee | used by | observed interface |
|---|---|---|
| `0x80164FD0` | `AA70`, `AEC0`, `B370` | `(owner, key, 0) -> 0/1` validity check |
| `0x802C7C70` | `AA70`, `AEC0`, `B370`, `B320` | `(dst, &vec2, copy, flip, 1, 1)` |
| `0x8016A2A0` | `AA70`, `AEC0`, `B370` | `(owner, state, value)` |
| `0x8016A0F0` | `AA70`, `AEC0`, `B370` | `(owner, selector, enable)` |
| `0x80163E10` | `6870` | `(this, handler, 0, &byte) -> u32` |
| `0x802B6620` | `68B0` | `(sub)` installs the `0x802FE410` member vtable |
| `0x80229030` | `69A0` | `(sub, 0)` member teardown |
| `0x802B63B0` | `69A0` | `(sub, 0)` member teardown |
| `0x802B9280` | all destructors | `(this)` raw delete |
| `0x8008F140` | `6AA0` | `(sub)` installs `0x80310E40` |
| `0x8008F170` | `6AF0` | `(sub, -1)` release |
| `0x8008F1B0` | `6B60` | `(sub, arg, byte, allocator) -> handle` |
| `0x80163660` | `6B60` | `(this, handle)` bind |
| `0x80163690` | `6BF0` | `(this)` unbind and release `*(this+4)` |
| `0x8016E500` | `6C50` | `(b, a, factory, 0x20, 4) -> handle` |
| `0x802B9240` | `6C50` | `(size, handle, align) -> object` |
| `0x8016A5E0` | `6C50` | `(object) -> object` construct in place |
| `0x8016A690` | `6C50` | `(object, handle, 0x20)` finish |
| `0x8008F2B0` | `6BF0` | `(sub) -> bool` release payload |
| `0x802B4DB0` | `6E30` | `(sub, -1)` release |
| `0x802ACB80` | `6E30` | `(sub, 0)` member teardown |
| `0x802ACB40` | `6CD0` | `(sub)` installs `0x803294D8` |
| `0x802B4D30` | `6CD0` | `(sub)` installs member vtable |
| `0x801DD1D0` | `6CD0` | `() -> u32`, region/screen query via the `0x800B54B0` thunk |
| `0x802AD520` | `6FA0` | `(out_floats, source)` reads four floats |
| `0x801645F0` | `7520` | nine arguments, returns a status in `r3` |
| `0x80164B20` | `7520` | `(0x1F) -> surface` |
| `0x80253C70` | `7520` | `(&surface, x0, y0, x1, y1, u0, v0)` |
| `0x80253D00` | `7520` | `(&surface, x0, y0, x1, y1)` |
| `0x801644F0` | `7520` | `(0)` close |
| `0x80164770` | `7610` | six arguments, returns a status |
| `0x80164B70` | `7610` | `(key) -> resource` |
| `0x802CD050` | `7610` | `(table, name)` lookup-or-insert |
| `0x802B9170` | `7610` | `(token) -> token` resource-edit bracket |

## Where Ghidra and the bytes disagree

All five were checked against `work/dol/main.elf`; the Rust follows the bytes.

| Address | Ghidra C | Instruction stream |
|---|---|---|
| `80006AA0` | `FUN_800068b0()` with no arguments | `bl` at `0x80006AB4` with `r3 = this`, so the base constructor receives the object |
| `80006BF0` | `FUN_80163690()` with no arguments | `bl` at `0x80006C10` with `r3 = this`; `0x80163690`'s own export declares one parameter |
| `80006C50` | `FUN_8016a5e0()` with no arguments | `bl` at `0x80006C9C` with `r3` = the allocated block; the export of `0x8016A5E0` declares one parameter |
| `80006CD0` | `thunk_FUN_801dd1d0()` as if it were a game call | `0x800B54B0` is a 4-byte `b 0x801DD1D0` thunk; the Rust calls `0x801DD1D0` directly |
| `80007610` | `FUN_80007610(param_1)` plus a stray `in_r8` | `or r29,r8,r8` at `0x80007628` shows six live argument registers (`r3..r8`) |

Two more artefacts affect `8000B370`:

* Ghidra prints `uVar5 = FUN_802dcd70(); iVar1 = high; iVar4 = low;`.
  `0x802DCD70` is a Metrowerks save-register helper, not a call. The two
  "halves" are `owner` (`r26`) and `mode` (`r27`).
* `0x80007510` is the known 4-byte `b 0x80164FD0` thunk; the attachment
  functions call `0x80164FD0` directly, as `docs/COVERAGE_r80005_80014.md`
  prescribes.

## Left for a later pass

| address | size | why not taken |
|---|---:|---|
| `80007de0` | 1632 | GQR / paired-single effect setup. Every float crosses a quantisation register, so Ghidra's output is known-low-fidelity; it needs a dedicated paired-single pass. |

Every `>= 16`-byte entry in the `0x80006870..0x80007610` object cluster is now
translated, so nothing in that cluster is left over. `8000aa70` and `8000aec0`
were translated separately on purpose: they are two distinct entry points whose
bodies are byte-identical.

Sub-16-byte entries were skipped by the task's minimum-size rule. Four of the
seven are tail thunks (`0x800067A0` → `0x80163280`, `0x80006BE0` → `0x80006B60`,
`0x80006EA0` → `0x80163850`, `0x80007510` → `0x80164FD0`), one is a bare `blr`
(`0x80006790`), one is a three-instruction field setter (`0x80006A40`:
`*(this+4) = this+0xC`), and one is a two-instruction getter (`0x80007220`:
`return *(this+0x20)`). None of them carries control flow of its own.

## Verification

* `cargo check` was run after the edit. `src/render/geometry.rs` produced **no
  errors and no warnings**; the crate reports only pre-existing warnings in
  other lanes' files.
* `node tools/doc-lint.mjs src` reports `{"ok": true, "files": 68,
  "violations": 0}`.
* Constants were resolved from the image, not guessed:
  `0x8042B080 = 0.0f`, `0x8042B084 = 0.7307692f`, `0x8042B088 = 1.0f`,
  `0x8042B098 = 0.5f`, `0x8042B09C = -0.5f`, `0x8042B0A0 = 0.0f`,
  `0x8042B0A4 = 1.0f`.
* Small-data bases: `r13 = 0x8042F680` (`0x80004244`), `r2 = 0x80433080`
  (`0x8000423C`).
* No ledger row was created without a body: all 24 rows in
  `ledger/agents/wave11-geometry.csv` correspond to a `pub unsafe fn` in
  `src/render/geometry.rs`.

## Uncertainties and limitations

1. **Class identity.** The `0x802FE400`, `0x802FE3F0` and `0x802FE3D8` vtables
   have no RTTI strings in the image. The Rust keeps `fn_<address>` names and
   documents the recovered field layout instead of inventing classes.
2. **`0x44C` in `fn_8000b370`.** The value is exact (`addi r3,r0,1100`);
   whether it is a state id, a capacity or a frame count is `// unknown:`.
3. **`0x801645F0` arguments.** The nine values `(arg, 1, &word, 1000, 1000,
   0x80, 0x80, 2, 2)` are exact, but the callee's parameter meaning and the
   kind of surface (scissor, viewport or texture quad) are not established.
   This is why `fn_80007520` is `medium`.
4. **`fn_80007610` argument roles.** The six arguments are forwarded to
   `0x80164770` unchanged and `p6` also selects the resource; what the other
   five describe is unresolved. This is why it is `medium`.
5. **Uninitialised second point component.** `0x802C7C70` is handed a 2-word
   buffer whose second word the original never writes. The Rust reproduces
   that with `MaybeUninit` rather than inventing a zero.
6. **Free flags.** Every destructor frees only when `free_flag > 0`; a caller
   that passes `0` for a heap object leaks, exactly as in the original. No
   "fix" was applied.
7. **No runtime validation.** `cargo check` proves the module type-checks; it
   does not execute the PowerPC code.
