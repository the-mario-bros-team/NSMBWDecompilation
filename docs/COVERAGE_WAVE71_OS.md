# Coverage - Wave 71 (runtime::os, remaining `0x801B0000` bucket)

## How to read this file

This page records wave 71 of the `main.dol` reconstruction for the
`runtime::os` lane: the fifth pass over the `0x801b0000..0x801bffff` bucket,
which still had 137 of its 227 inventory entries untranslated.

- **Status** - `translated` means every branch, store and call in the Ghidra
  export has a counterpart in `src/runtime/os.rs`. All 29 rows here are
  `translated`; none is `partial` and none is `stub`.
- **Confidence** - strength of the evidence, not of the Rust. `high` means the
  flow, the stores and the callee argument shapes are explicit and
  self-contained. `medium` means the flow is explicit but a callee's role is
  inferred from its size/usage, or an argument register was swept away by
  Ghidra and had to be restored by symmetry with a sibling body.
- **Evidence** - the C file under `work/ghidra/main-gb/export/`; it is the
  source of truth for every row. Byte sizes come from
  `work/ghidra/main-gb/export/functions.tsv`, re-read for this pass.
- Addresses are virtual addresses of the unrelocated `main.dol` (image base
  `0x80004000`), not file offsets.

The machine-readable companion is `ledger/agents/wave71-os.csv` (same rows,
same order). This wave edits only `src/runtime/os.rs` plus those two files;
`ledger/functions.csv` is untouched and is merged by the coordinator.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/os.rs` | `runtime::os` | 29 | 3,712 | 21 | 8 |

Status split: 29 `translated`, 0 `partial`, 0 `stub`. No `low`.

Candidates were chosen as whole clusters with a shared global block rather than
as a byte count: sections A-E each read one SDA/absolute block and then justify
every body in the group from that single reading.

## What this pass covers

- **Section A - character-code converters, `0x801B64D0..0x801B65C0`.** A UTF-16
  reader that consumes a surrogate pair and stores the code point
  (`+0x10000` bias), a single-byte mapper with a 32-entry table at
  `0x80333DC0`, and a two-level lookup through the 256-page pointer array at
  `0x8033F800`. The second body keeps a Ghidra-visible dead guard
  (`0x151 < code < 0x2123` inside the `0x80..0x9F` window) verbatim.
- **Section B - StateTM event hooks, `0x801B6600..0x801B6BF0`.** The missing
  half of the StateTM driver already in this file: `r13-0x4F60` gets the
  `/dev/stm/immediate` descriptor, `r13-0x4F5C` the `/dev/stm/eventhook`
  descriptor, `r13-0x4F4C`/`-0x4F50` the two hook pointers (default stubs
  `0x801B6BD0`/`0x801B6BE0`), `r13-0x4F58` the "armed" flag. Nine bodies: two
  identical hook installers, the two-channel initialiser, the one-shot `0x5001`
  immediate-event arm, the `0x3002` disarm, the async callback
  `0x801B6BF0`, and three small reader/`memcpy` leaves.
- **Section C - NWC24 shutdown and time sync, `0x801B7470..0x801B7A20`.** The
  32-byte NAND record at `0x8038F600` whose word 0 is the additive checksum of
  words 1..7, its writer and validating reader, the
  `/dev/net/kd/request` command-1 wrapper, the async `0x28` request wrapper with
  its completion callback `0x801B7C10`, and the bounded-retry state machine that
  ends with `NWC24Shutdown: Give up!`.
- **Section D - SI sampling rate, `0x801BB160..0x801BB920`.** The
  `0xCD006430` sampling control write (mask `0xFC0000FF`, fields at bits 16..21
  and 8..13) and the TV-format table select. The report string
  `SISetSamplingRate: unknown TV format` at `0x80340018` is the only explicit
  symbol evidence in this pass.
- **Section E - interrupt-masked global swap, `0x801BC3E0..0x801BC430`.** Two
  identical setters that return the previous word, one SDA slot apart.
- **Section F - six leaves.** The hot-reset fallback `0x801B3C30`, the latched
  vertical-timing read `0x801B3C90`, the power-callback status word
  `0x801B4300`, the `+0x2C8` state predicate `0x801B4B40`, the `+0x2D4` reader
  `0x801B5F70` and the empty body `0x801B5FA0`.

## What the pass adds beyond the raw bodies

1. **The StateTM shutdown path is now closed.** The already-translated
   `stm_shutdown_hot_reset`/`stm_shutdown_normal_reset` only ever run because
   event `0x20000` reaches the callback installed here; `fn_801b6bf0` is that
   callback, and it re-arms the channel after every dispatch. A port that skips
   the re-arm will shut down once and then go deaf.
2. **`0x801B6A40`'s request block is fully enumerated.** Words `+0x00..+0x1C`
   of `0x8038F3C0` are written every time: the encode word, four zeros,
   `0xFFFFFFFF`, `0xFFFF0000` and a zero. The odd pairing of the two `0xFFFF`
   constants is preserved rather than normalised.
3. **The NAND record checksum rule is explicit.** `0x801B7470` copies the body,
   replaces word 0 with the sum of words 1..7, then writes; `0x801B7560`
   recomputes the same sum and compares. Any port must keep the "copy, then
   checksum, then write" order - computing the checksum before the copy would
   validate stale bytes.
4. **The SI divider depends on the VI field parity.** `0x801BB840` multiplies
   the table divisor by `1 + (0xCC00206C & 1)`, so the same rate yields two
   register values depending on the current field. That term is easy to drop.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801B3C30` | `fn_801b3c30` | 88 | high |
| `0x801B3C90` | `fn_801b3c90` | 48 | high |
| `0x801B4300` | `fn_801b4300` | 16 | high |
| `0x801B4B40` | `fn_801b4b40` | 44 | high |
| `0x801B5F70` | `fn_801b5f70` | 8 | high |
| `0x801B5FA0` | `fn_801b5fa0` | 8 | high |
| `0x801B64D0` | `fn_801b64d0` | 112 | high |
| `0x801B6540` | `fn_801b6540` | 124 | high |
| `0x801B65C0` | `fn_801b65c0` | 64 | high |
| `0x801B6600` | `fn_801b6600` | 8 | high |
| `0x801B6610` | `fn_801b6610` | 8 | high |
| `0x801B6620` | `fn_801b6620` | 24 | high |
| `0x801B6640` | `fn_801b6640` | 236 | high |
| `0x801B6730` | `fn_801b6730` | 236 | high |
| `0x801B6820` | `fn_801b6820` | 280 | high |
| `0x801B6A40` | `fn_801b6a40` | 248 | medium |
| `0x801B6B40` | `fn_801b6b40` | 120 | high |
| `0x801B6BF0` | `fn_801b6bf0` | 340 | high |
| `0x801B7470` | `fn_801b7470` | 228 | medium |
| `0x801B7560` | `fn_801b7560` | 296 | medium |
| `0x801B7690` | `fn_801b7690` | 180 | medium |
| `0x801B78B0` | `fn_801b78b0` | 256 | medium |
| `0x801B79B0` | `fn_801b79b0` | 108 | medium |
| `0x801B7A20` | `fn_801b7a20` | 184 | high |
| `0x801BB160` | `fn_801bb160` | 92 | high |
| `0x801BB840` | `fn_801bb840` | 212 | high |
| `0x801BB920` | `fn_801bb920` | 8 | high |
| `0x801BC3E0` | `fn_801bc3e0` | 68 | medium |
| `0x801BC430` | `fn_801bc430` | 68 | medium |

## Report shape for the eight-argument iOS calls

`0x80225500` takes eight integer arguments, but `crate::platform` only exports
`call0..call7`. Sections B and C therefore add one private helper at the top of
the new file section:

```rust
unsafe fn call8(addr, a, b, c, d, e, f, g, h) -> u32 {
    let function: extern "C" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
        core::mem::transmute(addr);
    function(a, b, c, d, e, f, g, h)
}
```

The same shape already exists privately in `src/runtime/bt.rs`; it is repeated
here rather than shared because lane files may not modify `platform.rs`.

## Unknowns

1. **The two SDA word caches.** `0x801B6600`/`0x801B6610` return
   `r13-0x4F70` and `r13-0x6148`; `0x801B6620` fills them from the low-memory
   words `0x80003130`/`0x80003134`. What those bootstrap words mean is not
   recoverable from this window.
2. **Callee roles.** `0x80004364`, `0x800046B4`, `0x80224C70`, `0x80224E60`,
   `0x80225500`, `0x80225640`, `0x801A9E70`, `0x801AD4E0`, `0x8015F730`,
   `0x801BE010`, `0x801BE450`, `0x801D87E0`, `0x801D89F0`, `0x801D8AE0`,
   `0x801D95B0` and `0x801D9850` are dispatched by
   address. `memcpy`/`memset`/`open`/`ioctl`/`close`-style roles are marked
   `// inferred:` and are the sole reason those rows are `medium`.
3. **Two swept mask arguments.** `0x801B6A40` and the `0x801BC3E0`/`0x801BC430`
   pair print `FUN_801b1140()`/`FUN_801b1180()` with no value at all. The saved
   mask is restored by symmetry with `0x801B6640`, which keeps it; the rows are
   `medium` for that reason alone.
4. **Event ids.** `0x20000` and `0x800` are named by value only; nothing in the
   window says which StateTM event they are, beyond the `0xCC003000` bit-16 test
   that gates the hot-plug path.
5. **Table contents.** The 32-entry table at `0x80333DC0`, the 256-page array at
   `0x8033F800` and the two SI tables at `0x8033FFB8`/`0x8033FFE8` are read but
   not dumped; only their shapes are established here.
6. **`+0x2C8`/`+0x2D4` record identity.** The two small leaves in section F are
   literal, but the record they index is unnamed.

## Deliberate skips

- `0x801B7750` (184 bytes) was read and left out: Ghidra drops the argument of
  its `0x801B32D0` call and keeps a nonsense `r13 == 0x6138` comparison, so the
  two stores immediately before the call would have to be assumed to be the
  argument by inference alone.
- `0x801B7810` (148 bytes) depends on `extraout_r4` surviving a call to
  `0x802DCEC4`; the subtraction cannot be expressed without knowing which
  register the callee leaves behind.
- `0x801B7280` (492 bytes) mixes a `0x800000F8/4000*500` timing loop with a
  64-bit carry chain that the export prints as a negative-word comparison; it
  needs the timing-base story settled first.
- `0x801B7CE0` (344 bytes) is a `0x1020`-byte 7-word checksum plus a
  `countLeadingZeros` result shifted by 5; the same shape as section C but a
  different record, and it was not read closely enough to claim.
- `0x801B8B20` (636 bytes) is a bit-packed block copy with an eight-way unrolled
  shift chain, and `0x801B8DA0`/`0x801B8E50`/`0x801B8F50` are its wrappers; the
  register-field layout (`DAT_cd006800`/`DAT_cd00680c`, 3 bits per channel) is
  not established yet.
- `0x801BC480` (280 bytes) is a 22-case switch returning string/table addresses;
  documenting it honestly needs the contents of 17 data blocks.
- The large workers `0x801B6D60` (1,204), `0x801BAB00` (1,004), `0x801BBBD0`
  (2,060), `0x801BC7A0` (1,352), `0x801BD280` (1,740), `0x801BE860` (1,332) and
  `0x801BEFC0` (3,448) were not attempted: their callees are no better named
  after this pass.
- The 90 addresses already merged in this bucket are not repeated.

Remaining untranslated in the bucket after this wave: 137 - 29 = 108 entries.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (re-read for this pass, not copied from prose).
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every CSV row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/os.rs`; no address appears twice.
- No address in the CSV was already present in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave71` -> `wave71-os.csv: 29 rows`,
  `total rows: 29, unique addresses: 29`,
  `translated/high=21 translated/medium=8`, `OK`.
- `cargo check --message-format=short` -> `Finished dev profile`; no error and
  no warning from `src/runtime/os.rs`. The remaining warnings are pre-existing
  and outside this lane (`src/runtime/bt.rs`, `src/runtime/cpp.rs`,
  `src/runtime/input.rs`).
