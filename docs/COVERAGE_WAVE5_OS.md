# Coverage — Wave 5 OS / core band (`0x801A0000–0x801C0000`)

## How to read this file

This file records the 27 functions translated by the Wave 5 `runtime::os` lane
inside the main.dol OS/core band. Read it in four steps:

1. **Scope** gives the band inventory and what "eligible" means.
2. The **table** lists every new function in address order: plain-English
   role, status, confidence, the Ghidra C file, and the Rust entry point.
3. **Clusters** groups the table by the hardware/OS mechanism each function
   touches and explains why the lane picked it.
4. **Next targets** names concrete, still-untranslated entry addresses from
   `functions.tsv`, so a following pass does not have to re-inventory the band.

Status vocabulary (`docs/READING_GUIDE.md`): `translated` means the full
control flow, stores and calls are accounted for; `partial` means entry/exit
and major effects only; `stub` means address/role only. Confidence is evidence
strength (high / medium / low), not code quality.

Every Rust body carries an `# Original` block with the Ghidra `FUN_<address>`,
the byte size and the evidence path. No Nintendo symbol was invented:
descriptive names are used where the body, a string, or a caller proves the
role; otherwise the function keeps its address-based name in the prose and the
doc says `// inferred:`.

## Scope

| Item | Count |
|---|---:|
| Index entries in `[0x801A0000, 0x801C0000)` | 498 |
| Excluded, size `< 16` bytes | 78 |
| Eligible, size `>= 16` bytes | 420 |
| Eligible bytes in the band | 122,620 |
| New functions translated by this lane | **27** |
| Bytes represented by the new set | **3,252** |
| Ledger rows in `ledger/agents/wave5-os.csv` after this pass | **57** |

The 57 ledger rows are the 30 pre-existing `runtime::os` rows for the alarm,
message-queue, mutex, condition and scheduler clusters, plus the 27 rows added
here. The 30 pre-existing bodies were already in `src/runtime/os.rs`; they are
not claimed again by this pass.

The band is not one subsystem. Besides the OS primitives selected here it
contains DSP/AX register work around `0x801B25E0–0x801B2D50`, the
`OSLink`/`OSUnlink` relocation path at `0x801B1960–0x801B2190`, the
`OSExec`/play-time code at `0x801AE0E0` and `0x801B84C0`, and the large
`StateTM`/reset state machines. Those were left to their own semantic lanes
except where a reset or StateTM primitive was needed to complete the selected
control flow.

## Translated functions

Evidence paths are relative to `work/ghidra/main-gb/export/`; the full file
name is the lowercase address followed by `_FUN_<lowercase address>.c`.
`src/runtime/os.rs` is the only Rust file touched.

| Address | Rust name | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x801AC4A0` | `dc_store_range_sync` | Stores data-cache lines over a range, then executes the `sc` barrier. | translated | high | `801ac4a0_FUN_801ac4a0.c` |
| `0x801AC4D0` | `dc_flush_range_nosync` | Flushes (writes back) data-cache lines over a range without the barrier. | translated | high | `801ac4d0_FUN_801ac4d0.c` |
| `0x801AC500` | `dc_store_range_nosync` | Stores data-cache lines over a range without the barrier. | translated | high | `801ac500_FUN_801ac500.c` |
| `0x801AC560` | `ic_invalidate_range` | Invalidates instruction-cache lines, then `sync` + `isync`. | translated | high | `801ac560_FUN_801ac560.c` |
| `0x801AC5D0` | `lc_enable` | Enables HID2 locked cache, clears the normal window, programs the locked windows and zeros the locked blocks. | translated | medium | `801ac5d0_FUN_801ac5d0.c` |
| `0x801AC6A0` | `lc_enable_locked` | Interrupt-masked wrapper around `lc_enable`. | translated | high | `801ac6a0_FUN_801ac6a0.c` |
| `0x801AC6E0` | `lc_disable` | Invalidates the locked-cache window and clears the HID2 locked-cache field. | translated | medium | `801ac6e0_FUN_801ac6e0.c` |
| `0x801ACD40` | `install_context` | Publishes a saved context as the live exception context. | translated | high | `801acd40_FUN_801acd40.c` |
| `0x801ACF80` | `clear_context` | Clears a context's exception-state halfwords and current-context pointer. | translated | high | `801acf80_FUN_801acf80.c` |
| `0x801ACFB0` | `init_context` | Builds the initial LR/SP/MSR/GPR/GQR image for a new thread. | translated | high | `801acfb0_FUN_801acfb0.c` |
| `0x801B4890` | `init_thread_system` | Fills the system context, boot stack guard, run-queue table and idle context. | translated | high | `801b4890_FUN_801b4890.c` |
| `0x801B50D0` | `reschedule` | Dispatches when the pending-reschedule flag is set. | translated | high | `801b50d0_FUN_801b50d0.c` |
| `0x801B50F0` | `reschedule_forced` | Forced dispatch with interrupts masked/restored. | translated | high | `801b50f0_FUN_801b50f0.c` |
| `0x801B6020` | `get_time_with_delta` | Reads the cached time base and returns it plus a 64-bit addend. | translated | high | `801b6020_FUN_801b6020.c` |
| `0x801B47B0` | `copy_hot_reset_trampoline` | Copies the 0x1C-byte hot-reset stub to `0x80000C00`, flushes and invalidates it. | translated | high | `801b47b0_FUN_801b47b0.c` |
| `0x801BB9C0` | `boot_info_flag` | Returns one masked bit from the boot-info flags word. | translated | high | `801bb9c0_FUN_801bb9c0.c` |
| `0x801AC840` | `machine_check_handler` | Reports HID2/SRR1, dumps and halts on non-DMA faults, prints DMA causes and rewrites HID2. | translated | high | `801ac840_FUN_801ac840.c` |
| `0x801AD360` | `install_fpu_unavailable_handler` | Installs exception vector 7 and clears the current-context pointer. | translated | high | `801ad360_FUN_801ad360.c` |
| `0x801B32D0` | `insert_power_callback` | Key-sorted insert into the power/reset callback list. | translated | high | `801b32d0_FUN_801b32d0.c` |
| `0x801B3360` | `run_power_callbacks` | Runs the equal-key callback chain and folds the results. | translated | high | `801b3360_FUN_801b3360.c` |
| `0x801B3B00` | `return_to_menu_or_panic` | Boots the menu, then panics if the boot never completes. | translated | high | `801b3b00_FUN_801b3b00.c` |
| `0x801B3B40` | `return_to_data_manager_or_panic` | Returns to the data manager, then panics if the boot never completes. | translated | high | `801b3b40_FUN_801b3b40.c` |
| `0x801B3B80` | `hot_reset_panic_path` | Flushes the state block and StateTM channel, then panics twice. | translated | medium | `801b3b80_FUN_801b3b80.c` |
| `0x801B4440` | `read_stm_register` | Reads one StateTM halfword register under the register lock. | translated | medium | `801b4440_FUN_801b4440.c` |
| `0x801B44C0` | `write_stm_register` | Writes one StateTM halfword register and notifies the block. | translated | medium | `801b44c0_FUN_801b44c0.c` |
| `0x801B6940` | `stm_shutdown_hot_reset` | Stops the StateTM channel with ioctl `0x2003` and spins. | translated | high | `801b6940_FUN_801b6940.c` |
| `0x801B69C0` | `stm_shutdown_normal_reset` | Stops the StateTM channel with ioctl `0x2001` and spins. | translated | high | `801b69c0_FUN_801b69c0.c` |

## Clusters

### Cache range and locked cache — `0x801AC4A0–0x801AC6E0` (7 functions)

The first cache bank (`0x801AC440`, `0x801AC470`, `0x801AC530`) was already in
`os.rs` from an earlier pass. The retail image keeps four more range variants
because the SDK distinguishes store vs flush and barrier vs no-barrier:

* `0x801AC4A0` = `dcbst` loop + `sc`.
* `0x801AC4D0` = `dcbf` loop, no barrier.
* `0x801AC500` = `dcbst` loop, no barrier.
* `0x801AC560` = `icbi` loop + `sync` + `isync`.

The same region contains the locked-cache pair. `0x801AC5D0` sets `HID2 |
0x1000`, clears `0x80000000..0x80008000`, sets `HID2 | 0x100F`, writes SPRs
1023/1022, and zeros 0x4000 bytes at `0xE0000000` with `dcbz_l`. `0x801AC6E0`
reverses the window setup with 0x200 `dcbi` blocks and clears the HID2 field.
`0x801AC6A0` is the interrupt-masked wrapper around the enable path.

These functions are the only cache primitives in the band that were still
missing after the earlier cache pass. The cache opcodes are recorded by the
stand-ins already used by `dc_invalidate_range`/`dc_flush_range`; a host build
cannot issue them.

### Context installation and thread-system startup — `0x801ACD40–0x801B602C` (8 functions)

The earlier scheduler pass had the run/wait queues, create/exit/join and
priority inheritance, but it deliberately called the context primitives by
address as `// unknown:`. This pass supplies the missing leaves:

* `0x801ACD40` publishes a context into `0x800000C0`/`0x800000D4` and toggles
  the MSR bit `0x2000` in the context.
* `0x801ACF80` clears the two exception-state halfwords (`+0x1A0`, `+0x1A2`).
* `0x801ACFB0` builds the initial LR (`+0x198`), SP (`+0x04`), MSR `0x9032`,
  SDA2/SDA bases, zero GPRs and zero GQR slots.
* `0x801B4890` is the one-time thread-system start: it fills the static
  context at `0x8038E938`, sets the `0x8043FC40`/`0x8042FC3C` stack and
  `0xDEADBABE` guard, calls the switch handler `r13-0x6150`, publishes the
  thread at `0x800000E4`, clears the boot stack and the 0x100-byte run-queue
  table at `0x8038EC50`, and clears the idle context `0x8038ED50`.
* `0x801B50D0`/`0x801B50F0` are the two reschedule triggers.
* `0x801B6020` reads the cached time base and adds a 64-bit delta without
  storing it back.
* `0x801B47B0` copies the 0x1C-byte hot-reset stub to `0x80000C00`, flushes
  it, and invalidates the instruction cache.

The scheduler's existing `// unknown:` call sites still dispatch to the
original addresses. Wiring them to these new Rust functions would be a
behaviour-preserving cleanup for a later pass; this lane keeps the public
surface additive.

### Boot-info flag test — `0x801BB9C0` (1 function)

`0x801BB9C0` tests one bit of the boot-info flags word at
`0x80000040 + 4`; later boot-type checks use it as a reason mask. Its companion
writer `0x801BB930` is the first OS/runtime init call from the DOL entry
(`0x80004050`), but that body already lives in `runtime::init` and is not
repeated here.

### Machine check and FPU handler — `0x801AC840`, `0x801AD360` (2 functions)

`0x801AC840` is the DMA/locked-cache machine-check handler named by its own
strings. It reports HID2 and SRR1, dumps and halts when the fault is not a
DMA/locked-cache error, and otherwise prints the four asserted cause messages
before writing HID2 back. `0x801AD360` installs the FPU-unavailable vector and
clears the current-context pointer.

### Power/reset callback chain — `0x801B32D0–0x801B3B80` (5 functions)

`0x801B32D0` is a key-sorted insert into the callback list at
`r13-0x4F88`/`r13-0x4F84`; `0x801B3360` runs the equal-key chain and folds the
results. The three wrappers cover the visible reset exits: return-to-menu,
return-to-data-manager, and the hot-reset panic path that first flushes the
StateTM channel.

### StateTM registers and shutdown — `0x801B4440–0x801B69C0` (4 functions)

`0x801B4440`/`0x801B44C0` are the locked halfword accessors for the six
StateTM registers at `0x8038E8F4+0x1C`. `0x801B6940`/`0x801B69C0` are the two
shutdown paths; they stop the StateTM channel with ioctl `0x2003` (hot reset)
or `0x2001` (normal reset), mask interrupts, invalidate the instruction cache
and spin forever.

## Uncertainties and caveats

1. **SPRs 1022/1023 in `0x801AC5D0`.** The two `mtspr` words encode SPRs
   1022 and 1023 with values `0xE00001FE` and `0xE0000002`. No symbol,
   string or caller names them; the Rust records the writes through a
   documented no-op and marks them `// unknown:`.
2. **Locked-cache return pair.** `0x801AC5D0` and `0x801AC6E0` leave values
   in `r3:r4` that Ghidra types as `ulonglong`. No caller uses them. The Rust
   returns the observed pair and documents that it is a register-pair
   artifact, not a C value.
3. **StateTM register meanings.** The lock protocol, offsets and the
   `0x801B4020(0|1, 0x14)` notification are explicit; what each of the six
   halfwords controls is not. The accessors are therefore medium confidence.
4. **Already-locked StateTM path.** `0x801B4440`/`0x801B44C0` restore
   interrupts and then use base 0 when the lock is already held; the Rust
   preserves that observed path rather than "fixing" it.
5. **Hot-reset state block.** `0x801B3B80` passes a stack block to
   `0x801B7560`/`0x801B7470`. The raw bytes show the two mode fields at block
   offsets `+5` (value 3) and `+6` (value 2). The rest of the block layout is
   not recovered.
6. **Thread-system stack clear.** `0x801B4890` bounds the clear with the
   result of `0x801ACF10`. That leaf is called by address; its exact SDK name
   is not recovered.
7. **Machine-check halt.** The fatal branch calls `halt()` (the existing
   `sync` + endless-loop helper) instead of a second indirect call to
   `0x801A9BA0`, which has the same effect.
8. **Power callback contract.** The list key and the two forwarded arguments
   are visible; the semantic meaning of the key and of the callback return
   value is inferred from this one call site.

## Next targets

All addresses are entry addresses in `functions.tsv`; sizes are bytes.

| Address | Size | Why it is next |
|---|---:|---|
| `0x801AC710` | 36 | GQR/SPR setup adjacent to the cache bank; raw body writes SPRs 860/892 and GQR fields. |
| `0x801AC740` | 36 | Three-argument block helper called by `0x801AC770`; raw body masks the address and programs an SPR. |
| `0x801AC770` | 160 | 0x1000-stride wrapper around `0x801AC740`; used by texture decode paths. |
| `0x801ACDB0` | 128 | Saves the live GPRs/GQRs/CR/LR/CTR/XER into a context; needs the same prologue artifact treatment as the scheduler. |
| `0x801ACE30` | 216 | Context restore/`returnFromInterrupt`; completes the pair with `0x801ACDB0`. |
| `0x801AD070` | 608 | Context dumper used by the machine-check fatal path; prints GPRs, LR/CR, SRR0/SRR1, GQRs and FPRs. |
| `0x801B3410` | 384 | Full power/reset transition that walks the callback list and switches to the reset context. |
| `0x801B4020` | 736 | StateTM block transfer/commit used by both register accessors. |
| `0x801B4310` | 384 | StateTM read/verify path used by the reset state machines. |
| `0x801B6640` | 88 | StateTM event-callback installer; needs an eight-argument indirect-call helper. |
| `0x801B6820` | 280 | StateTM channel bring-up (`/dev/stm/immediate`, `/dev/stm/eventhook`); same eight-argument call shape. |
