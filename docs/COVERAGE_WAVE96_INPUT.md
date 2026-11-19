# Coverage - Wave 96 (`runtime::input`, WUD pairing/save state machine and dynamic GKI pools)

## How to read this file

1. **Summary** gives the per-file totals and the band inventory for
   `[0x801F0000, 0x80200000)`.
2. **What this pass covers** describes the three clusters that were added and
   how they connect to the wave-74/89/91 bodies around them.
3. The **function table** lists all 26 bodies in address order with the byte
   count re-read from `work/ghidra/main-gb/export/functions.tsv` and the
   confidence carried by each doc block.
4. **Corrections and caveats** records every inference the export forced, and
   **Deliberate skips** lists the 26 band entries this pass did not take.

Status vocabulary (`docs/READING_GUIDE.md`): `translated` means the whole
control flow and every call in the export are accounted for. All 26 rows are
`translated`. Names stay `fn_<addr>` because the image has no symbol table for
this layer; the doc comments carry the roles and repeat the
`// observed:` / `// inferred:` / `// unknown:` markers at the code.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/input.rs` | `runtime::input` | 26 | 8,884 | 13 | 13 |

Band inventory (`[0x801F0000, 0x80200000)`):

| Item | Count |
|---|---:|
| Functions in the band | 268 |
| Untranslated before this pass | 52 |
| Selected and translated by this pass | **26** |
| Untranslated in the band after this pass | 26 |

## What this pass covers

Wave 91 stopped in front of the WUD inner state machine and explicitly deferred
the "larger `unaff_r13`/`extraout_r4` machines". This pass takes the tractable
half of that run and the allocator helpers underneath it. Three shapes repeat:

- **WUD pairing and save state machine (`0x801F0020`-`0x801F0AFF`,
  `0x801F1930`, `0x801F5260`, `0x801F59D0`, 11 bodies).** Five consecutive
  steps share the ten/six `0x60`-byte device records (`0x803A3204`,
  `0x803A35C4`), the `+0x59` state byte and the two mode bytes
  `0x803A380A`/`380B`: `fn_801f0020` scans for state `9` and posts a channel
  request, `fn_801f09a0` does the same for state `8` and then runs the pairing
  callback, `fn_801f0180` and `fn_801f02f0` probe the CNT/WBC names and register
  a record (including the 128-byte checksum fold), and `fn_801f0670` writes the
  ten link-key records. `fn_801f1930` is the state dispatcher on `0x803A312D`
  (`0`, `1`, `2`, `3`, `5`, `8`, save states `9`..`0x67`); `fn_801f5260` is the
  nine-event cleanup-stack callback; `fn_801f59d0` handles the two application
  sync/delete events that call back into it.
- **WUD bring-up and init (`0x801F2B80`-`0x801F31FF`, 3 bodies).**
  `fn_801f2b80` is the timer step (state `1` waits 500 ticks, then clears the
  control block), `fn_801f2df0` is `__wudClearControlBlock` (both slot lists and
  every state byte), and `fn_801f3190` is the one-shot `BTA_Init()`.
- **Dynamic GKI pools and HID helpers (`0x801F6A00`-`0x801FFEC4`, 12 bodies).**
  `fn_801f7aa0` creates a dynamic pool from a caller block size/count and
  inserts it into the size-ordered pool list, `fn_801f7df0` releases one,
  `fn_801f8040` is the BTU alarm scheduler the wave-34 tables feed,
  `fn_801fd0e0` builds the `0x107` message, `fn_801ffec4` is the
  `bta_hh_find_cb` lookup over the sixteen `0x20`-byte HID control blocks, and
  `fn_801f8e80`/`fn_801f9830`/the seven `0x801F9D10`-`0x801F9DD0` guards are the
  small receive/stub entries. `fn_801f6a00` dispatches the application id.

## Functions

| Address | Function | Role | Bytes | Status | Confidence | Evidence |
|---|---|---|---:|---|---|---|
| `0x801F0020` | `fn_801f0020` | state-9 scan, channel request, returns `1`/`0x1D`/`0x0E` | 344 | translated | high | `work/ghidra/main-gb/export/801f0020_FUN_801f0020.c` |
| `0x801F0180` | `fn_801f0180` | CNT/WBC name probe, returns `6`/`0xFF` | 368 | translated | medium | `work/ghidra/main-gb/export/801f0180_FUN_801f0180.c` |
| `0x801F02F0` | `fn_801f02f0` | registers one device record and stages the checksum | 888 | translated | medium | `work/ghidra/main-gb/export/801f02f0_FUN_801f02f0.c` |
| `0x801F0670` | `fn_801f0670` | stores the ten link-key records, returns `0x16`/`0x19`/`100` | 308 | translated | high | `work/ghidra/main-gb/export/801f0670_FUN_801f0670.c` |
| `0x801F09A0` | `fn_801f09a0` | state-8 scan and pairing completion | 360 | translated | high | `work/ghidra/main-gb/export/801f09a0_FUN_801f09a0.c` |
| `0x801F1930` | `fn_801f1930` | device/save state dispatcher on `0x803A312D` | 848 | translated | medium | `work/ghidra/main-gb/export/801f1930_FUN_801f1930.c` |
| `0x801F2B80` | `fn_801f2b80` | timer step, 500-tick gate, state `3`/`5`/`2` arms | 276 | translated | medium | `work/ghidra/main-gb/export/801f2b80_FUN_801f2b80.c` |
| `0x801F2DF0` | `fn_801f2df0` | `__wudClearControlBlock`, relinks both slot lists | 916 | translated | high | `work/ghidra/main-gb/export/801f2df0_FUN_801f2df0.c` |
| `0x801F3190` | `fn_801f3190` | one-shot `BTA_Init()`, returns `1`/`0` | 284 | translated | medium | `work/ghidra/main-gb/export/801f3190_FUN_801f3190.c` |
| `0x801F5260` | `fn_801f5260` | nine-event cleanup-stack callback | 1,364 | translated | medium | `work/ghidra/main-gb/export/801f5260_FUN_801f5260.c` |
| `0x801F59D0` | `fn_801f59d0` | sync/delete event handler (ids `8`, `9`, `10`, `0x10`) | 680 | translated | medium | `work/ghidra/main-gb/export/801f59d0_FUN_801f59d0.c` |
| `0x801F6A00` | `fn_801f6a00` | application-id dispatch to `0x803A380C` | 56 | translated | medium | `work/ghidra/main-gb/export/801f6a00_FUN_801f6a00.c` |
| `0x801F7AA0` | `fn_801f7aa0` | dynamic GKI pool create, returns index or `0xFF` | 840 | translated | medium | `work/ghidra/main-gb/export/801f7aa0_FUN_801f7aa0.c` |
| `0x801F7DF0` | `fn_801f7df0` | dynamic GKI pool release | 356 | translated | medium | `work/ghidra/main-gb/export/801f7df0_FUN_801f7df0.c` |
| `0x801F8040` | `fn_801f8040` | BTU alarm tick/deadline scheduler | 332 | translated | medium | `work/ghidra/main-gb/export/801f8040_FUN_801f8040.c` |
| `0x801F8E80` | `fn_801f8e80` | adjusts the HID report length, returns `0` | 60 | translated | high | `work/ghidra/main-gb/export/801f8e80_FUN_801f8e80.c` |
| `0x801F9830` | `fn_801f9830` | constant-`0` HID stub | 8 | translated | high | `work/ghidra/main-gb/export/801f9830_FUN_801f9830.c` |
| `0x801F9D10` | `fn_801f9d10` | trace guard for `0x801F9C50` | 20 | translated | high | `work/ghidra/main-gb/export/801f9d10_FUN_801f9d10.c` |
| `0x801F9D30` | `fn_801f9d30` | trace guard for `0x801F9C50` | 20 | translated | high | `work/ghidra/main-gb/export/801f9d30_FUN_801f9d30.c` |
| `0x801F9D50` | `fn_801f9d50` | trace guard for `0x801F9C50` | 20 | translated | high | `work/ghidra/main-gb/export/801f9d50_FUN_801f9d50.c` |
| `0x801F9D70` | `fn_801f9d70` | trace guard for `0x801F9C50` | 20 | translated | high | `work/ghidra/main-gb/export/801f9d70_FUN_801f9d70.c` |
| `0x801F9D90` | `fn_801f9d90` | trace guard for `0x801F9C50` | 20 | translated | high | `work/ghidra/main-gb/export/801f9d90_FUN_801f9d90.c` |
| `0x801F9DB0` | `fn_801f9db0` | trace guard for `0x801F9C50` | 20 | translated | high | `work/ghidra/main-gb/export/801f9db0_FUN_801f9db0.c` |
| `0x801F9DD0` | `fn_801f9dd0` | trace guard for `0x801F9C50` | 20 | translated | high | `work/ghidra/main-gb/export/801f9dd0_FUN_801f9dd0.c` |
| `0x801FD0E0` | `fn_801fd0e0` | builds one `0x107` message | 140 | translated | medium | `work/ghidra/main-gb/export/801fd0e0_FUN_801fd0e0.c` |
| `0x801FFEC4` | `fn_801ffec4` | HID control-block lookup (`bta_hh_find_cb`) | 316 | translated | medium | `work/ghidra/main-gb/export/801ffec4_FUN_801ffec4.c` |

Totals: **26 translated**, 0 partial, 0 stubs; 13 high / 13 medium / 0 low;
8,884 bytes. Address range `0x801F0020`-`0x801FFEC4`.

## Corrections and caveats

1. **Split returns are read once through `platform::call_u64`.** Four bodies
   take their real arguments from the `0x802DCDxx` prologue artifact Ghidra
   prints as a call: `0x801F7AA0` (low = block count, high = block size),
   `0x801F8040` (low = tick, high = timer kind), `0x801F5260` (low = payload
   pointer, high = event id) and `0x801FD0E0` (low = flag, high = `BD_ADDR`
   pointer). Each body calls the artifact once and splits the 64-bit result, so
   the two halves stay paired. The first two declared parameters of
   `0x801F7AA0`, `0x801FD0E0` and `0x801F8040` are not read by the export and
   are kept as `_arg0`/`_arg1`.
2. **`0x801F02F0` list arithmetic.** `*DAT_803a3188` is the *device* pointer of
   the ten-slot tail (`WUD_TAIL_B` holds the slot record), and
   `DAT_803a3188[1]` is the previous slot, dereferenced to its device pointer.
   Both readings are forced by the `+0x40` `BD_ADDR` offsets the callers use,
   and match the wave-89 list code in [fn_801f5000].
3. **`0x801F1930` save calls.** The four `0x801D8C70`/`801D96D0`/`801D98C0`/
   `801D8B50` entries are identified only by their argument shape and the
   strings next to them (`"WbcSaveData: %s"`); the work object
   `0x803A3F20` and the context `0x803A3FAC` are opaque. The completion labels
   (`0x801F0830`, `0x801F07B0`, `0x801F08C0`, `0x801F0940`) are addresses
   inside neighbouring pairing steps and are passed literally.
4. **`0x801F2B80` time source.** Ghidra prints the low word of the
   `0x801B5FB0` time as `extraout_r4`; the body uses [`get_time`] once, takes
   the low word and divides by `bus_clock / 4000`. The `r13-0x5004` boot
   snapshot is named from that division only.
5. **`0x801F2DF0` unrolled loops.** The export contains three unrolled
   eight-way zero blocks and two 2-records-per-iteration link loops; the Rust
   uses two counted zero loops (`16` entries at `0x803A3EE0`, `0x803A5288`,
   `0x803A52A8`) and two link loops (`6` and `10` slots, stride `0xC`). The
   resulting pointers are identical, including the `5 - index` device order of
   the six-slot bank.
6. **`0x801F5260` record fields.** `+0x59` state, `+0x46` link key and the
   `0x803A39D1`/`39D2`/`39D3` connection bytes are reused from the wave-91
   docs. The `0x15` link-down reason and the `0x10F` authentication offset are
   literal but their meanings are unknown; they are marked in the body.
7. **`0x801FFEC4` sentinel.** The second address compare uses `r2-0x3460`,
   which the export never writes; it is modelled as a constant sentinel and
   flagged `// unknown:`. The three trace calls go to the 20-byte guards by
   address with their original argument counts, because those guards drop the
   variadic frame.
8. **`0x801F8040` base words.** `0x803CDB00`/`0x803CDB04` are named only from
   their arithmetic role in the deadline and the min-tick update; no other
   function in the file documents them.
9. **`0x801F6A00` lost register.** The application id is read from `in_r8`
   (third argument register) only, so the body takes it as a parameter, the
   same convention wave 91 used for its lost-register arguments
   (`0x801FC574`, `0x801FDBBC`).
10. **Module header.** The thirteenth-pass bullet and the promotion-note
    sentence were appended; no earlier count was stale (the previous text named
    no total).

## Deliberate skips

- **`0x801F0B10` (2,788 B).** The nineteen-state switch behind `DAT_803A312c`
  (states `4`..`0x13`, including the stored-link-key write and the WBC class
  compare). It builds on the same records as this pass but is a full state
  machine of its own; it needs a dedicated pass to keep the evidence tight.
- **`0x801F1F70` (1,592 B) and `0x801F2650` (1,328 B).** The key-table
  migration walk and the connect state step. Both are honest candidates for a
  later pass; `0x801F1F70` is called by `0x801F2b80` by address and is listed in
  the doc of the caller.
- **`0x801F36F0` (332 B).** Reads `extraout_r4` for the mode word and stores it
  in `0x803A3808`; the register is not recoverable from the export without
  guessing, so the body is left out (it is still called by address from
  [fn_801f59d0]).
- **The HID-host receive path carried over from waves 61/74/89/91**
  (`0x801F8820`, `0x801F8BC0`, `0x801F8F70`, `0x801F9120`, `0x801F9490`,
  `0x801F96A0`, `0x801F9840`, `0x801F9FA0`, `0x801FF970`): `extraout_r4`
  frame/pointer artifacts and variadic forwarding. `0x801F9C50` (180 B) is the
  variadic formatter those guards call and is skipped for the same reason.
- **`0x801FC958` (228 B).** Reads `in_r10` and a stack argument and forwards
  both to `0x801F6A60`; the arity is not recoverable.
- **`0x801FCC74` (264 B) and `0x801FD16C` (240 B).** `0x801FCC74` reads the
  `r13-0x5DA8` compressed-link table through another split return and
  `0x801FD16C` (`BTA_DmAddDevice`) ignores two of its four arguments; both need
  a caller-side study that this pass did not do.
- **`0x801FD4BC` (896 B) and `0x801FD83C` (896 B).** The live-register pair wave
  89 already skipped: both use both halves of a `0x802DCD58`/`0x74` return as a
  pointer and an id in the same expression.
- **The 4-byte thunks** (`0x801F3300`, `0x801F8630`, `0x801F8640`,
  `0x801F8650`, `0x801F8660`, `0x801F87B0`, `0x801F87F0`): pure tail jumps to
  already-translated bodies.

## Verification

- Every address and byte size in the table was re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass (total 8,884 bytes).
- Every evidence file exists at the cited `work/ghidra/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/input.rs` (checked by name for all 26 addresses).
- No address was already in `ledger/functions.csv`; the 52-entry untranslated
  list for the band was derived from it first.
- `node tools/check-lane-csv.mjs wave96` prints `OK` and reports
  `wave96-input.csv: 26 rows`, with all 187 lane rows unique.
- `cargo check --message-format=short` reports no errors in
  `src/runtime/input.rs`. The remaining build errors at the time of writing
  belong to other Wave 96 lanes (`src/runtime/bt.rs`, `src/runtime/os.rs`,
  `src/game/stage.rs`).

## Ledger

Machine-readable rows: `ledger/agents/wave96-input.csv` (26 rows, module
`runtime::input`, address order as the table above, header
`module,address,name,status,confidence,evidence,rust_file`).
