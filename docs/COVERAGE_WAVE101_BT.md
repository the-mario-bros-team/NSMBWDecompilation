# Coverage - Wave 101 (`runtime::bt`, the L2CAP window workers and the RFCOMM DLC callbacks)

## How to read this file

Every row below is one `pub unsafe fn fn_<address>` body appended to
`src/runtime/bt.rs`. The address is the unrelocated `main.dol` virtual address
(image base `0x80004000`); the byte count was re-read from
`work/ghidra/main-gb/export/functions.tsv` for this pass; the `evidence` column
of `ledger/agents/wave101-bt.csv` names the exact Ghidra C export.

All addresses are in the `0x80210000` bucket, which holds 198 functions and had
34 untranslated entries when this pass started. This pass claims 25 of them.
After the merge, nine remain; they are listed under *Deliberate skips*.

`// observed:`, `// inferred:` and `// unknown:` markers follow
`docs/RUST_STYLE.md`. Field roles that the evidence does not prove are marked
rather than named.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 25 | 9476 | 4 | 21 |

## What this pass covers

### L2CAP connection-block API and queue workers (`0x802128A0..0x80216C98`, 9 bodies)

The four blocks at [`L2CAP_CHANNEL_TABLE`] (`0x803D4C48`, stride `0x5C`), the ten
CCBs at [`L2CAP_CCB_TABLE`] and the single reassembly slot at
[`L2CAP_REASSEMBLY`] are driven by:

| Address | Role |
|---|---|
| `0x802128A0` | connect request: PSM block, connection block, CCB, returns the local CID |
| `0x80212AD4` | connect response: identifier check, events `0x15`/`0x16` |
| `0x80213120` | address-keyed timeout writer over all four blocks |
| `0x80214F04` | congestion report, queue push/drop, QoS re-issue, queue drain |
| `0x802151D0` | window recomputation from the load share |
| `0x80215368` | credit restore from a little-endian handle/amount array |
| `0x80215460` | fragment reassembly with the `0x6A0`-byte cap |
| `0x802159BC` | receive-side dispatcher, including the hold queue |
| `0x80216C98` | command-reject builder (width `2`/`4`/`0` by reason) |

They share one window block, named here for the first time:

| Offset | Meaning |
|---|---|
| `+0x34` | `u16` transmit-window ceiling |
| `+0x36` | `u16` current transmit window (already named `L2CAP_CH_WINDOW`) |
| `+0x38` | `u16` load contribution (already named `L2CAP_CH_LOAD`) |
| `+0x3A` | `u16` window scaled to 75 %, the priority threshold |
| `+0x3C` | `u16` low congestion threshold (scaled minus three) |
| `+0x3E` | `u16` high congestion threshold (scaled plus two) |
| `+0x40` | clip flag (already named `L2CAP_CH_WINDOW_FLAG`) |
| `+0x41` | congested flag, set while congestion was reported upwards |
| `+0x4C` | `u16` count of bytes still expected from the peer |
| `+0x50` | retained reassembly buffer |
| `+0x5A` | priority byte (`1` = high) |

### Create-connection gate and configuration reject (`0x802175E0`, `0x8021843C`)

`0x8021843C` clears a new block to `{0,0,0,3}`, scans the four blocks for an
unused neighbour and asks `0x80207674` whether the peer already has a SCO link;
`0x802175E0` writes the fixed reject header, echoes the option list and patches
both length words.

### RFCOMM DLC state callbacks (`0x8021C904..0x8021CDCC`, 9 bodies)

Every callback takes the multiplexer control block in `r3` and the DLCI in `r4`,
resolves the `0xA4`-byte port through [`fn_80219e40`] and either dispatches an
event through `0x8021B738` or forwards to `0x80218DDC`/`0x80219070`/`0x8021948C`.

| Address | Role |
|---|---|
| `0x8021C904` | parameter-negotiation result (event `9`, else establishment status `1`) |
| `0x8021C96C` | DLC establishment result (event `0x0B` with the status word) |
| `0x8021C9DC` | starts parameter negotiation: credit cap, PN builder, timer `0x3C` |
| `0x8021CAE4` | disconnect result: flag bit `2`/`4`, `0x91` response, trailer `0x3F7F` |
| `0x8021CBC0` | control frame: `+0x64` bit `0`, DLC flag bit `3` |
| `0x8021CC64` | credit/flow frame: inverted credit mode at `+0x5E`, DLC flag bit `3` |
| `0x8021CD08` | line status: DLC flag bit `4`, short `0x91` response |
| `0x8021CDA0` | release indication (event `0x0C`) |
| `0x8021CDCC` | data indication (event `0x0E`, packet forwarded) |

The DLC/port flag byte at `+0x69` gains its five event bits: `1` parameter
negotiation, `2`/`4` disconnect result, `8` control/credit, `0x10` line status.

### Four isolated bodies

- `0x8021022C` — HID output-report entry: three guards (registered, handle
  `<= 0x10`, peer state `1`) around the fragmenter `0x802122B0`.
- `0x8021D270` — DLC parameter-negotiation frame builder. Its DLCI argument is
  printed as Ghidra's `extraout_r4`; that register is the function's own second
  argument, because the `0x802DCD70` "call" is the function's own `stmw`
  sequence and touches no general register.
- `0x8021D8B4` — mux frame classifier: address/control/length decode followed by
  DISC (`0x43`), SABME (`0x2F`), UA (`0x0F`), UIH (`0xEF`) and UI (`0x63`)
  tests. Note that DISC/SABME test *inequality* of the two role bits while
  UA/UI test *equality*; both forms are reproduced literally.
- `0x8021F5C0` — SDP record reader: six `u16` attributes (`0x200`..`0x205`) and
  three 0x50-byte blobs (`0x0B`, `0x101`, `0x0A`). The export's three unrolled
  eight-byte copies are one byte loop in `sdp_copy_blob`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8021022C` | `fn_8021022c` | 320 | medium |
| `0x802128A0` | `fn_802128a0` | 564 | high |
| `0x80212AD4` | `fn_80212ad4` | 400 | medium |
| `0x80213120` | `fn_80213120` | 256 | medium |
| `0x80214F04` | `fn_80214f04` | 716 | medium |
| `0x802151D0` | `fn_802151d0` | 408 | high |
| `0x80215368` | `fn_80215368` | 228 | high |
| `0x80215460` | `fn_80215460` | 464 | medium |
| `0x802159BC` | `fn_802159bc` | 816 | medium |
| `0x802167AC` | `fn_802167ac` | 300 | medium |
| `0x80216C98` | `fn_80216c98` | 252 | medium |
| `0x802175E0` | `fn_802175e0` | 436 | medium |
| `0x8021843C` | `fn_8021843c` | 280 | medium |
| `0x8021C904` | `fn_8021c904` | 104 | medium |
| `0x8021C96C` | `fn_8021c96c` | 112 | medium |
| `0x8021C9DC` | `fn_8021c9dc` | 224 | medium |
| `0x8021CAE4` | `fn_8021cae4` | 188 | medium |
| `0x8021CBC0` | `fn_8021cbc0` | 164 | medium |
| `0x8021CC64` | `fn_8021cc64` | 164 | medium |
| `0x8021CD08` | `fn_8021cd08` | 152 | medium |
| `0x8021CDA0` | `fn_8021cda0` | 44 | medium |
| `0x8021CDCC` | `fn_8021cdcc` | 56 | medium |
| `0x8021D270` | `fn_8021d270` | 232 | medium |
| `0x8021D8B4` | `fn_8021d8b4` | 1396 | medium |
| `0x8021F5C0` | `fn_8021f5c0` | 1200 | high |

## Why the twenty-one `medium` rows are `medium`

- Eleven bodies recover an argument from the function's own register-save
  sequence (`0x802DCD4C..0x802DCD74`). As waves 60/68/92/97/99 established, that
  is the function's own `stmw`, not a call: Ghidra folds the incoming `r3`/`r4`
  into a fake 64-bit result, so the argument split is recovered rather than
  printed. Where only one 32-bit value is used, it is `r3` (confirmed by the
  call sites: `0x80215368` is called as `FUN_80215368(param_1 + 10)`,
  `0x802167AC` as `FUN_802167AC(0)`/`(1)`). `0x802151D0` carries the same
  prologue but discards its value, so nothing is recovered there.
- The nine RFCOMM callbacks are exported with no parameters at some call sites
  (`FUN_80219e40()` with no arguments). The two registers are proven by the
  callers instead: `FUN_8021C904(iVar1, *(byte *)(iVar2 + 0xd), ...)` and
  `FUN_8021CDCC(*(u32 *)(iVar1 + 0x6C), *(byte *)(iVar1 + 0xd), iVar2)`.
- `0x80212AD4` passes a 14-byte result block whose first ten bytes are
  uninitialised stack in the export; the body passes zeros there and marks the
  assumption `// unknown:`.
- `0x80214F04` does the same for its ten-byte QoS block, of which only `+8` is
  written by the original code.
- `0x8021D8B4` and `0x8021843C` consume four `r13` string pointers that the
  decompiler would not inline (`0x80429948..0x80429960`); only the addresses are
  known, so they are passed to the logger unchanged.
- `0x8021022C` names its record fields from the wave-68 HID layout
  (`BTM_POOL`, stride `0x34`, `+0` in use, `+0x0A` peer state), not from new
  evidence.

## Deliberate skips

Nine addresses of the bucket's original 34 are still untranslated and have no
row in the CSV.

- `0x8021E4D0` (344) - the mux timer callback. Its `extraout_r4` is stored into
  `0x803D5831` *after* three intervening calls (`0x800046B4`, `0x80004364`,
  `0x801F6F90`), so the value cannot be attributed to the second argument with
  any confidence.
- `0x80218F20` (336) - the port parameter-negotiation indication. Its no-port
  fallback writes the absolute addresses `0x51..0x59`, exactly the reason wave
  96 skipped it; the two bodies that need it keep calling it by address as
  [`RFCOMM_PORT_NEG_IND_80218F20`].
- `0x8021A704` (672) - the mux data indication. Three of its callees
  (`0x8021AAF4`, `0x8021B738`, `0x8021DE28`) are untranslated and its routing
  bytes at `+0`/`+2`/`+4`/`+5` of the `0x803D5428` context have no proven
  meaning; translating it alone would document half a state machine.
- `0x802122B0` (668), `0x80215CEC` (2672), `0x8021AAF4` (2652),
  `0x8021B738` (2476), `0x8021DE28` (1572) and `0x80213240` (4916) - the six
  large dispatchers of the layer. They are the next targets of this bucket;
  each carries a multi-way event switch and needs its own pass. The bodies
  added here call them by address (`L2CAP_EVENT_80213240`,
  `L2CAP_DATA_80215CEC`, `HID_REPORT_802122B0`, `RFCOMM_EVENT_8021B738`) or
  directly where a translated body already existed.

## Verification

- Every address and byte size in this doc and in the CSV was re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass; the 25 bodies total
  9476 bytes.
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/bt.rs`.
- No address was already in `ledger/functions.csv`; all 25 came from the
  bucket's untranslated list.
- `cargo check --message-format=short` produces no error and no new warning for
  `src/runtime/bt.rs`. The only `bt.rs` diagnostics in the run are the ten
  pre-existing `unnecessary parentheses` warnings at lines `5170..5812`, which
  belong to the untouched wave-6 L2CAP code.
- `node tools/check-lane-csv.mjs wave101` prints `wave101-bt.csv: 25 rows` and
  `OK`.
