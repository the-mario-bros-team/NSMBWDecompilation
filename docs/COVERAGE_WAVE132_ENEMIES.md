# Coverage - Wave 132 (`game::enemies`, wave-131 specification and `0x80A00000` bucket)

## How to read this file

This page records the Wave 132 `game::enemies` lane: 27 functions of REL module
3, `d_enemiesNP`, appended to `src/game/enemies.rs` as section 114. Every row of
the machine-readable mirror (`ledger/agents/wave132-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none of
these is a runtime address; every function doc repeats that note. Data constants
are cited by address and read through `platform::read_u32` / `read_f32`, never
transcribed as invented values. Raw words were decoded from
`work/rels/elf/d_enemiesNP.elf` at the mapping in `docs/REL_ELF_MAPPING.md`
(`file_offset = address - 0x80A00000 + 0x1000`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 27 | 5,688 | 11 | 16 |

The lane has two halves:

- **12 bodies** are the rows of `docs/SPEC_WAVE131_ENEMIES.md`: the eleven
  marked **ready** plus the one **conditional** row (`0x80A628F0`). They were
  written from that page's instruction-level decode and re-verified word by word
  against the raw ELF stream and the Ghidra C export.
- **15 bodies** are still-open entries of the `0x80A00000` planner bucket (36 of
  that bucket's 212 functions were open when the pass started; this pass takes
  15 and leaves 21).

## What this pass covers

- **The twelve specification bodies (`0x80A2FD40..0x80B1D090`).** The
  four-sample trail initialiser `0x80AB2840`, the 60-byte bubble billboard
  record `0x80A34F80`, the `env_underwater`/`wmy_ef_bubble` spawn binder
  `0x80A2FD40`, the `lift` spawn initialiser `0x80A77A50` (two fn-pointer
  tables, state 4), the `+12`/`-12` matrix refresh `0x80B1D090`, the
  `lift_rakka_ashiba` model + damage binder `0x80A39410`, the 23-byte
  contact/pad record `0x80AD42E0`, the 300-frame retry machine `0x80B0DC40`,
  the `Wm_en_landsmoke_s` trigger `0x80A37560`, the `x +/- dz` ground probe
  `0x80AD0960`, the state 1 -> 2 `walk` transition `0x80A370D0` and the
  side-scaled speed initialiser `0x80A628F0`.
- **The `+0x115C` countdown and the `0x80A07000` cluster (5 bodies).**
  `0x80A06480` (`s16` countdown clamp), `0x80A07080` (`+0x30` owner-chain walk
  with vtable slot `+0x0C`), `0x80A07160` (`:`-suffix name predicate),
  `0x80A06760` and `0x80A06800` (the two four-slot `+0x738` occupant sweeps).
- **The `0x80A0F000`/`0x80A0FF00` actor block (5 bodies).** `0x80A0F5A0` (the
  `0x8016A960` holder wrapper and its three binders), `0x80A0F490` (the
  spawn-time store block with the `oris` flag bit), `0x80A0F8B0` (per-holder
  scales over the `+0x544` chain), `0x80A0F970` (the `+4 & 1` speed selector
  that publishes `0x80B35230`/`0x80B35234`) and `0x80A0FFD0` (the slot `+0x24`
  chain sweep).
- **The `0x80A06000` clip bind (1 body).** `0x80A06B00` binds entry clip `3`
  through `0x80A61A80` and converts `+0x5AC` to the `+0x115E` halfword with the
  `fctiwz` + `slwi 8` idiom.
- **Four `0x802DCBFC` forwarders (`0x80A03270`, `0x80A032A0`, `0x80A07220`,
  `0x80A07250`).** All four are the same three-register shuffle
  `(second, second, first)` with the hidden `r12 = second + 0x18`.

### Confirmations against the specification

Every field offset, constant address, store order and call argument of the
twelve specification bodies matched the raw ELF words. Five details worth
recording:

- **`0x80AB2840`'s raw-then-adjusted y store.** `0x80AB28A0`/`0x80AB28B4` store
  the raw `+0xB0` value and then the same value `+8.0`; the body keeps both
  writes because the intermediate value is observable.
- **`0x80A34F80`'s two arms.** Bit 8 of the word at `+4` selects
  `(scale, drop) = (0.5, 14.0)` or `(0.9, 25.2)`; the record type word is
  `0x1003` and the eight tail words are the `0x80B2C094` zero.
- **`0x80A2FD40`'s self-relative arm.** `0x8023B800` returns the node; the
  `+0x3C` word is added to it when non-zero before the `+0x20` bias. The `0`
  arm is preserved, not treated as a null check.
- **`0x80A0F490`'s `oris`.** `0x80A0F4EC` is `oris r0,r0,0x100` (opcode 25), so
  the flag write is `| 0x01000000`, matching the export, not `| 0x100`.
- **`0x80A06B00`'s conversion.** `0x80A06B40` is `fctiwz`; `0x80A06B58` is
  `rlwinm r0,r3,8,0,23` and the halfword store keeps the low 16 bits, so the
  body writes `((value as i32) << 8) as u16`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AB2840` | `fn_80ab2840` | 356 | high |
| `0x80A34F80` | `fn_80a34f80` | 348 | high |
| `0x80A2FD40` | `fn_80a2fd40` | 340 | high |
| `0x80A77A50` | `fn_80a77a50` | 340 | medium |
| `0x80B1D090` | `fn_80b1d090` | 328 | high |
| `0x80A39410` | `fn_80a39410` | 324 | medium |
| `0x80AD42E0` | `fn_80ad42e0` | 320 | medium |
| `0x80B0DC40` | `fn_80b0dc40` | 320 | medium |
| `0x80A37560` | `fn_80a37560` | 296 | medium |
| `0x80AD0960` | `fn_80ad0960` | 296 | high |
| `0x80A370D0` | `fn_80a370d0` | 292 | medium |
| `0x80A628F0` | `fn_80a628f0` | 292 | medium |
| `0x80A0F490` | `fn_80a0f490` | 264 | high |
| `0x80A07080` | `fn_80a07080` | 224 | medium |
| `0x80A0F8B0` | `fn_80a0f8b0` | 192 | medium |
| `0x80A06760` | `fn_80a06760` | 160 | high |
| `0x80A0F970` | `fn_80a0f970` | 156 | medium |
| `0x80A07160` | `fn_80a07160` | 136 | medium |
| `0x80A06800` | `fn_80a06800` | 132 | high |
| `0x80A06B00` | `fn_80a06b00` | 124 | high |
| `0x80A0FFD0` | `fn_80a0ffd0` | 112 | medium |
| `0x80A0F5A0` | `fn_80a0f5a0` | 104 | high |
| `0x80A03270` | `fn_80a03270` | 48 | medium |
| `0x80A032A0` | `fn_80a032a0` | 48 | medium |
| `0x80A07220` | `fn_80a07220` | 48 | medium |
| `0x80A07250` | `fn_80a07250` | 48 | medium |
| `0x80A06480` | `fn_80a06480` | 40 | high |

### Why sixteen rows are `medium`

- **The four `0x802DCBFC` forwarders.** The three-register shuffle and the
  `addi r12, r5, 0x18` are literal, but the hidden `r12` this-adjust pointer
  cannot be modelled by an `extern "C"` call, exactly as wave 131 recorded for
  the `0x80B1A310` group.
- **`0x80A77A50`, `0x80A39410`, `0x80AD42E0`, `0x80B0DC40`, `0x80A37560`,
  `0x80A370D0`.** Every branch, store and token is literal; the state-phase,
  pad-flag, holder and callback roles are inferred from the compared values and
  the call shapes.
- **`0x80A628F0`.** The one conditional row: the byte at `0x8042C1A0 + side`
  lives in `main.dol`, is widened to float and multiplied by `2.2`. The port
  reads the shipped address at run time (never a guessed table value), so the
  body is complete but a byte-exact comparison still needs the table contents.
- **`0x80A07080`, `0x80A07160`, `0x80A0FFD0`, `0x80A0F8B0`, `0x80A0F970`.**
  The instruction streams are exact; the vtable slot and chain-field roles are
  inferred from the receivers and the following stores.

## Deliberate skips

- **`0x80A01700`, `0x80A03C30`, `0x80A07C80`, `0x80A0E160`.** Their exports end
  in `halt_baddata()` (truncated or bad instruction data), so no reachable body
  could be reconstructed without inventing instructions.
- **`0x80A03C90`, `0x80A06560`, `0x80A00D00`.** All three consume a
  `GQR1..GQR7` quantiser (`unaff_GQR3`, type codes 4/5/6/7) or incoming
  `f28..f31` arguments. They are class C under the wave-127 rules and were left
  for a pass that pins the `main.dol` GQR words.
- **`0x80A04A90`, `0x80A04BF0`.** Both take an incoming `f1` scale that Ghidra
  widens to `double`; the register class of the caller's argument is not
  established from the export alone, so nothing was claimed.
- **`0x80A0B8A0`.** The export recurses with an extra second argument that the
  entry signature does not have, so the call shape is ambiguous.
- **`0x80A05F40`.** The single callee `0x802B9280` prints no arguments, so its
  receiver cannot be established.
- **The remaining `0x80A00000` rows** (21 still open, including the 276-byte
  `0x80A0E450`, the 968-byte `0x80A0E580`, the 708-byte `0x80A03FF0`, the
  360-byte `0x80A04D10` and the 212-byte `0x80A06220`) are readable and were
  left for a later pass once the body target was met. Nothing was written for
  them, so no ledger row claims them.
- **No address already in `ledger/functions.csv` was touched**, and no other
  lane's file, CSV or doc was modified.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 27 sizes sum to
  5,688 bytes (3,852 for the twelve specification rows and 1,836 for the fifteen
  bucket rows).
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 27 was in
  `ledger/functions.csv` before this pass.
- The module header was updated: cluster item 114, the cluster count (113 to
  114), the function totals (3,041 to 3,068 public translations, 2,970 to 2,997
  `fn_<addr>` bodies), the "start here" list and the records list.
- `cargo check --message-format=short` emits no error and no warning naming
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave132` reports `wave132-enemies.csv: 27 rows`,
  27 unique addresses and `OK`.
