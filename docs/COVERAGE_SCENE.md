# Coverage — scene and boot/save state (`game::scene`)

## How to read this file

This document covers the `game::scene` slice of the `main.dol` band
`0x800C0000..0x80100000`.  All rows are **entry addresses in the image**, not
file offsets; the image base is `0x80004000`.

* **Address** — virtual address of the function entry.
* **Rust name** — the name in `src/game/scene.rs`.  Descriptive names are used
  only where the call sites/strings prove the role; the original Ghidra name is
  in the function's `# Original` block.
* **Role** — one sentence in plain English.
* **Status** — `translated` (all control flow and calls represented),
  `partial` (entry/exit and field effects represented, but one argument or
  receiver convention is not visible in the Ghidra output), or `stub`
  (address/role only).
* **Confidence** — strength of the evidence: `high` = every store/load is in
  the Ghidra C; `medium` = layout observed but a field/argument meaning is
  inferred; `low` = mostly unknown.
* **Evidence** — the Ghidra `.c` file under
  `work/ghidra/main-gb/export/`.

Two notation rules follow `docs/RUST_STYLE.md`:

1. `r13` is `0x8042F680` (set by `0x800041C0`), so `r13 - 0x6F50` means the
   absolute address `0x80428730`.
2. `r2` is `0x80433080`.  None of the functions in this slice read `r2`; the
   neighbouring float-only helpers that do are outside this document.

## Scope and counts

| Item | Count |
|---|---:|
| Functions translated in this slice | 24 |
| `translated` | 21 |
| `partial` | 3 |
| Function bytes (sum of sizes) | 4,292 |
| Address clusters | 2 (`0x800E19B0..0x800E1F68`, `0x800CEC10..0x800CFB30`; the last save function ends at `0x800CFBA8`) |
| Functions owned by `engine::startup` and **excluded** | 4 (`0x800C9F10`, `0x800C9F40`, `0x800C9F60`, `0x800C9F90`) |

The four excluded functions are the hand-off documented in
`docs/ENGINE_STARTUP.md`; `src/engine/startup.rs` owns them and this module does
not repeat them.

## Boot-to-title flow (what the save cluster does)

The boot scene (`dScBoot_c`, `0x8015DDD0`) drives the save file through a small
state object whose state word is at `+0x78`:

```text
save_task_create (0x800CFAB0)
  -> save_task_construct (0x800CEC10)       allocate task, seed 0x8035DCC0
  -> save_task_wait_and_start (0x800CEE20)  called by dScBoot_c at 0x8015D330
  -> save_check_files (0x800CEE90)          wiimj2d.sav + banner.bin exist?
  -> save_check_nand_space (0x800CEFE0)     enough free NAND blocks?
  -> save_read (0x800CF110)                 read/decode save
       -> save_read_banner (0x800CF290)     /tmp/banner.bin
            -> save_build_banner_buffer (0x800CF410)
  -> save_verify (0x800CF590)               size + stamp + slot checksums
       -> save_compare_slots (0x800CF820)
  -> save_delete_files (0x800CF960)         error/recovery path
  -> save_task_submit (0x800CF080)          present save image to worker
```

`save_result_to_state` (`0x800CFB30`) is the shared error-code mapper; every
step above writes its result through it.  The actual state numbers `0..8` are
observed from the switch, but their names are **unknown** because the boot
scene's `StateID_*` strings live outside this band (`0x8015DDD0`).

## Scene flow (what the scene cluster does)

The base scene object is constructed by [`scene_base_construct`] and destroyed
by [`scene_base_destroy`].  `dScBoot_c`'s constructor at `0x8015C130` calls the
base constructor, then replaces the vtable at `+0x60`, which proves the base
layout.

The transition globals form a three-slot rotation:

| global | displacement | address | meaning |
|---|---|---|---|
| current | `r13 - 0x6F50` | `0x80428730` | requested scene id, or `0x2EE` when free |
| previous | `r13 - 0x6F4E` | `0x80428732` | last committed scene |
| older | `r13 - 0x6F4C` | `0x80428734` | scene before `previous` |
| flag | `r13 - 0x6F4A` | `0x80428736` | request already swapped |
| aux A | `r13 - 0x6F48` | `0x80428738` | fade-out id (`// inferred:`) |
| aux B | `r13 - 0x6F46` | `0x8042873A` | fade-in id (`// inferred:`) |
| argument | `r13 - 0x5638` | `0x8042A048` | last `scene_request` argument |

```text
scene_request(id, arg, force)
  -> global scene id 0x8006A620(id, arg)
  -> current = id

scene_transition_step()        once per frame from 0x800E4E40
  -> 0x8006C6C0(current, argument, 1)  // thunk of 0x80162B20
  -> older = previous; previous = current; current = 0x2EE; flag = 1

scene_base_update(this)
  -> state table at this+0x70 via 0x8015F620
  -> when the table returns 2, drive the fade ids and ask the manager
```

`0x8015F620` is the generic state-table runner.  Its record is observed as
`u32 table / u16 count / u16 index`; each handler returns `1` (next state) or
`2` (finished).  It lives outside this band, so it is called by address.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x800CEC10` | `save_task_construct` | Allocates the save worker, installs its vtables/sync objects and seeds the 0x3FA0-byte save image | translated | high | `800cec10_FUN_800cec10.c` |
| `0x800CED70` | `save_task_release` | Frees the save task block when asked | translated | high | `800ced70_FUN_800ced70.c` |
| `0x800CEDB0` | `save_task_destroy` | Clears the global task pointer and tears the task down | translated | high | `800cedb0_FUN_800cedb0.c` |
| `0x800CEE20` | `save_task_wait_and_start` | If a request is queued, resets state and resumes the worker | translated | high | `800cee20_FUN_800cee20.c` |
| `0x800CEE90` | `save_check_files` | Checks `wiimj2d.sav` and `banner.bin` existence | translated | high | `800cee90_FUN_800cee90.c` |
| `0x800CEFE0` | `save_check_nand_space` | Checks free NAND space and maps low-space bits to states 7/8 | translated | medium | `800cefe0_FUN_800cefe0.c` |
| `0x800CF080` | `save_task_submit` | Copies a 0x3FA0-byte image into the global buffer and wakes the worker | translated | high | `800cf080_FUN_800cf080.c` |
| `0x800CF110` | `save_read` | Reads/decodes `wiimj2d.sav`, then falls through to the banner | translated | high | `800cf110_FUN_800cf110.c` |
| `0x800CF290` | `save_read_banner` | Reads `/tmp/banner.bin`, builds it and writes the encoded banner | partial | medium | `800cf290_FUN_800cf290.c` |
| `0x800CF410` | `save_build_banner_buffer` | Builds the banner work area and encodes 0x72A0 bytes | partial | medium | `800cf410_FUN_800cf410.c` |
| `0x800CF590` | `save_verify` | Verifies size, format stamp and slot checksums | translated | high | `800cf590_FUN_800cf590.c` |
| `0x800CF820` | `save_compare_slots` | Compares three save slots and their embedded checksums | translated | high | `800cf820_FUN_800cf820.c` |
| `0x800CF960` | `save_delete_files` | Deletes `banner.bin` and `wiimj2d.sav` | translated | high | `800cf960_FUN_800cf960.c` |
| `0x800CFAB0` | `save_task_create` | Allocates and resumes the save worker thread | translated | high | `800cfab0_FUN_800cfab0.c` |
| `0x800CFB30` | `save_result_to_state` | Maps SDK/NAND result codes to boot-scene states | translated | high | `800cfb30_FUN_800cfb30.c` |
| `0x800E19B0` | `scene_base_construct` | Base scene constructor: vtable, flags, state pointer, global clears | partial | medium | `800e19b0_FUN_800e19b0.c` |
| `0x800E1A20` | `scene_base_destroy` | Base scene destructor: vtable, refcount, optional free | translated | high | `800e1a20_FUN_800e1a20.c` |
| `0x800E1AA0` | `scene_base_is_finished` | Queries the base object and sets the global finished flag | translated | high | `800e1aa0_FUN_800e1aa0.c` |
| `0x800E1BE0` | `scene_base_update` | Per-frame scene update: state table, fade ids, manager query | translated | medium | `800e1be0_FUN_800e1be0.c` |
| `0x800E1DB0` | `scene_transition_reset` | Clears current/previous/older scene ids and the flag | translated | high | `800e1db0_FUN_800e1db0.c` |
| `0x800E1DE0` | `scene_transition_step` | Validates a requested scene id and rotates the id slots | translated | high | `800e1de0_FUN_800e1de0.c` |
| `0x800E1E60` | `scene_request` | Accepts a scene change when the slot is free and no fade is busy | translated | high | `800e1e60_FUN_800e1e60.c` |
| `0x800E1F50` | `scene_set_aux_id_a` | Writes auxiliary scene id A (`0x80428738`) | translated | medium | `800e1f50_FUN_800e1f50.c` |
| `0x800E1F60` | `scene_set_aux_id_b` | Writes auxiliary scene id B (`0x8042873A`) | translated | medium | `800e1f60_FUN_800e1f60.c` |

## What is proven, what is inferred

**Observed (high):**

* the save-image layout: `+0x000` header `0x6A0`, `+0x6A0` and `+0x2320` two
  arrays of three `0x980`-byte records, each with a `0x97C` payload and a `u32`
  checksum (`save_task_construct` and `save_compare_slots`);
* the NAND calls and path strings (`wiimj2d.sav`, `banner.bin`,
  `/tmp/banner.bin`);
* the state word at `+0x78`, the result mapping, and the three-slot scene id
  rotation;
* `0x2EE` is the "transition slot free" sentinel (`scene_request`,
  `scene_transition_step`, `scene_base_update`);
* `0x8015F620`'s `table/count/index` record and its `1`/`2` return protocol.

**Inferred (medium/low):**

* `scene_base_construct`/`scene_base_destroy` are the `dScene_c` base methods:
  `0x8015C130` calls the constructor then replaces the vtable at `+0x60`.
* the two auxiliary ids are fade ids: both are set to `0x1E` by `0x8015C4C0`;
  `save`-side code only shows them being consumed by `scene_base_update`.
* `scene_base_update`'s two vtable calls at `vtable + 0x10` may take the manager
  as an implicit `r3` receiver; the Ghidra decompile shows no arguments, so the
  Rust calls them with no explicit argument.
* the save state numbers `0..8` correspond to the `dScBoot_c` state names
  (`NandCommandEnd`, `ExistFileCheck`, `NandSpaceCheck`, `CreateFile`, `Load`,
  …), but the mapping is **not** proven because those names are in
  `0x8015DDD0`, outside this band.

## Unknowns / open items

1. **`0x800CF410` argument loss.**  Ghidra prints `void FUN_800cf410(void)` and
   uses `extraout_r4` at the final encode.  The caller `0x800CF290` passes
   `(task, buffer)`, so the Rust takes `scratch: usize` and documents the
   inference.  Status `partial`.
2. **`0x800CF290` dropped argument.**  The error path is
   `FUN_800cfb30(param_1);` with no second argument in the C.  The Rust passes
   the just-computed `0x72A0` mismatch value and marks `partial`.
3. **`0x800E19B0` receiver convention.**  `0x8006C420` and `0x800B3A20` are
   shown as no-argument calls, but they are base-object helpers and may use
   `r3`.  Status `partial`.
4. **Scene state numbers.**  The meaning of `0x2EE`, `3`, `6`, `7`, `8`, `9`,
   `0x2C7` at the call sites is not established; only the branches are.
5. **Fade state values.**  `scene_request` treats manager fields `+0x08` and
   `+0x0C` as busy when they are `1` or `5`; the field names are unknown.
6. **Save stamp.**  `0x800CF590` compares the first four bytes of the save
   buffer with `0x80427C6C..0x80427C6F`; the Rust names the source
   `G_SAVE_STAMP`, but the format/version meaning is inferred.

## Dependencies and lane notes

* `src/engine/startup.rs` owns `0x800C9F10`, `0x800C9F40`, `0x800C9F60` and
  `0x800C9F90`; this module does not duplicate them.
* `src/game/boot_scene.rs` documents `dScBoot_c` (`0x8015DDD0`) and the state
  names; this module provides the base scene/transition machinery and the NAND
  steps it calls.
* `0x800E4E40` (the per-frame body) calls `0x8015F620` and
  `0x800E1DE0`; both are referenced here, but the frame body belongs to the
  engine lane.
