# Coverage - Wave 99 (resource::archive, `0x802B0000` bucket)

## How to read this file

Every row names one Ghidra function by its shipped-image address, the Rust body
that replaces it in `src/resource/archive.rs`, its exact size from
`work/ghidra/main-gb/export/functions.tsv`, and how confident the
reconstruction is. `address` is a virtual address in `sys/main.dol` (image base
`0x80004000`), not a file offset. The 25 bodies are appended after the wave-97
section at the end of `src/resource/archive.rs`, in address order.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/resource/archive.rs` | `resource::archive` | 25 | 4200 | 1 | 21 | 3 |

The file now holds 509 translated bodies: 484 before this pass plus the 25
below. The assigned `0x802B0000` bucket had 41 untranslated functions when this
lane started; 25 of them are taken here, 16 remain (see deliberate skips).

## What this pass covers

This pass takes the `0x802B0000`-band bodies whose offsets, masks, constants
and callee addresses are visible in the exports and whose argument lists fit
the existing call model:

1. **RLYT geometry** - `0x802B1D60`, `0x802B4EE0`, `0x802B4B50` (3 bodies).
   The billboard builder combines two trig helpers and the `r2 - 0x3FC8` wrap;
   the spline evaluator is the binary search plus the four coefficient slots at
   `r2 - 0x3F88..-0x3F7C`; the attribute upload selects the palette path from
   the kind nibble in `+0x18`.
2. **Effect controller lifecycle** - `0x802BB580`, `0x802BB840`, `0x802BB9A0`,
   `0x802BBE30`, `0x802BBF40`, `0x802BBFA0`, `0x802BAC30`, `0x802BAF70`
   (8 bodies). The constructor publishes `r13 - 0x4828`; the update walks the
   frame budget and the render-mode hand-off; the two no-argument callbacks
   and the `0x802BAC30` hook run through the manager at `r13 - 0x4838`; the
   frame-time body divides the bus clock at `0x800000F8`.
3. **List and channel state** - `0x802BC9C0`, `0x802BD970`, `0x802BDE60`
   (3 bodies). The three share the child list built by `0x802BC920`: one stops
   it, one refreshes the per-channel mirror table under two lock slots, one
   splits it by the `0x802BDC10` value.
4. **Angle wrappers** - `0x802BE470`, `0x802BE4A0`, `0x802BE4D0`,
   `0x802BE500` (4 bodies). Four one-call wrappers whose `(void)` export
   prototypes drop the `f64` in `f1`; the target identities come from the
   companion `0x802BFDA0` call sequence and each body keeps the explicit
   `(double)(float)` narrowing.
5. **Quad placement and draw** - `0x802BF080`, `0x802BF540`, `0x802BF650`,
   `0x802BF930`, `0x802BFDA0`, `0x802BFE90`, `0x802BFF70` (7 bodies). The
   matrix rebuild adds three transformed basis rows; the two draw paths hand
   `+0x48..+0x60` to the GX blit and sprite helpers; the setter and the
   constructor write the `+0x44..+0x6C` block.

### Shared layout notes

`// inferred:` the owners are RLYT layout records and NW4R screen-effect
objects; the exports prove only the offsets, the masks, the constants and the
callee addresses.

- the effect quad keeps `x`/`y` at `+0x48`/`+0x4C`, four corners at
  `+0x50..+0x5C`, a width at `+0x60` and its vtable at `+0x00`;
- the controller keeps the frame budget byte at `+0x08`, the state byte at
  `+0x09`, the last tick at `+0x0C`, and the counters at `+0xA4..+0xAC`;
- the manager is always reached as `*(r13 - 0x4838)`, then slot `+0x08`,
  `+0x10` or `+0x14`; the same address is used by the wave-95 section;
- eight- and nine-argument callees are modelled with typed `extern "C"`
  pointers (`call8`, `call9`, `call_mixed9`), because `platform` stops at
  `call7`; the arguments occupy `r3..r10` exactly as the compiler emits them;
- the `CONCAT44(0x43300000, x) - 2^52` idiom is written as a plain
  `as f32`/`as f64` cast, following the wave-95 and wave-97 sections.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802B1D60` | `fn_802b1d60` | 264 | medium |
| `0x802B4B50` | `fn_802b4b50` | 236 | medium |
| `0x802B4EE0` | `fn_802b4ee0` | 444 | medium |
| `0x802BAC30` | `fn_802bac30` | 64 | medium |
| `0x802BAF70` | `fn_802baf70` | 132 | medium |
| `0x802BB580` | `fn_802bb580` | 120 | medium |
| `0x802BB840` | `fn_802bb840` | 348 | medium |
| `0x802BB9A0` | `fn_802bb9a0` | 108 | medium |
| `0x802BBE30` | `fn_802bbe30` | 260 | medium |
| `0x802BBF40` | `fn_802bbf40` | 96 | low |
| `0x802BBFA0` | `fn_802bbfa0` | 36 | high |
| `0x802BC9C0` | `fn_802bc9c0` | 16 | medium |
| `0x802BD970` | `fn_802bd970` | 460 | low |
| `0x802BDE60` | `fn_802bde60` | 260 | medium |
| `0x802BE470` | `fn_802be470` | 36 | medium |
| `0x802BE4A0` | `fn_802be4a0` | 36 | medium |
| `0x802BE4D0` | `fn_802be4d0` | 36 | medium |
| `0x802BE500` | `fn_802be500` | 36 | medium |
| `0x802BF080` | `fn_802bf080` | 388 | medium |
| `0x802BF540` | `fn_802bf540` | 272 | low |
| `0x802BF650` | `fn_802bf650` | 100 | medium |
| `0x802BF930` | `fn_802bf930` | 128 | medium |
| `0x802BFDA0` | `fn_802bfda0` | 112 | medium |
| `0x802BFE90` | `fn_802bfe90` | 84 | medium |
| `0x802BFF70` | `fn_802bff70` | 128 | medium |

## Unknowns

1. **The `r2` float slots.** Every curve, angle scale and size bias lives in
   `SDA2_BASE - 0x3F...`. The bodies read the slots by address; their values,
   units and exact meaning are not recovered.
2. **The manager virtual slots.** `+0x08`, `+0x10`, `+0x14` and `+0x18` are
   only named by offset. Ghidra prints most of these calls with no arguments,
   so each body passes the obvious receiver and says so at the call.
3. **The trig helper identities.** `0x80237C40`/`0x80237BD0` and
   `0x802E7FBC`/`0x802E84C4`/`0x802E8590` are typed only by their `f64`
   signatures; the sine/cosine/tangent naming is inferred from the store order
   in `fn_802bfda0`.
4. **`0x802BF540`'s defaults.** When the manager slot returns 0 the export
   keeps the two `r2 - 0x3EB8`/`-0x3EB4` floats; when it returns an object the
   two `+0x04`/`+0x06` halfwords are scaled instead. Which branch the mixer
   takes first is not visible.

## Deliberate skips

- **`0x802B5CC0`, `0x802B5DA0`, `0x802B5E50`, `0x802B60B0`, `0x802B7310`,
  `0x802BBFD0`, `0x802BD4A0`, `0x802B0680`, `0x802B20B0`, `0x802B2C70`,
  `0x802B6C80`** - the `FUN_802DCD40/50/58/64/68/70/74` families. Those are
  Metrowerks prologue save helpers; Ghidra invents a `(r3, r4)` return pair
  that is really the live argument pair at the spill point, and each of these
  bodies consumes that pair as an object pointer or a loop budget. There is no
  honest reconstruction without disassembly, exactly as the wave-97 lane
  recorded for the sibling helpers.
- **`0x802B4B50` note** - it is included: both GX calls take nine words, which
  the new `call9`/`call_mixed9` helpers express. `0x802BB9A0` is included for
  the same reason through `call8`.
- **`0x802BAF70` note** - the export has no zero-divisor guard; the Rust body
  uses `checked_div` so a host run cannot trap, and the doc says so.
- **`0x802BC980`, `0x802BC9A0`** - each is a two-line guard on `+0xB48`
  followed by `FUN_802BDF70()` / `FUN_802BDFC0()` with **all** arguments
  dropped by the export. The guard and target are known, the four arguments
  are not, so no body was written. Their sibling `0x802BC9C0` is translated
  because `FUN_802BDDF0` only takes the owner.
- **`0x802BBA30`, `0x802BE530`, `0x802BE5D0`** - the 920-byte GX setup, the
  `GQR0` matrix inverse and the `GQR0` matrix load. The first is reachable only
  as an opaque eight-argument call (kept in `fn_802bb9a0`), and the two thunks
  live in the hardware-quantisation path `src/platform.rs` does not model.
- **The other 16 addresses of the bucket** - `0x802B0680`, `0x802B20B0`,
  `0x802B2C70`, `0x802B5CC0`, `0x802B5DA0`, `0x802B5E50`, `0x802B60B0`,
  `0x802B6C80`, `0x802B7310`, `0x802BBA30`, `0x802BBFD0`, `0x802BC980`,
  `0x802BC9A0`, `0x802BD4A0`, `0x802BE530`, `0x802BE5D0`. Every one is either
  a save-helper consumer, the GX matrix path, or one of the two dropped-argument
  guards named above.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (read from the TSV, not from prose).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/archive.rs` with a complete `# Original` / `# Arguments` /
  `# Returns` / `# Side effects` / `# Safety` / `# Confidence` doc block.
- No address was already in `ledger/functions.csv`; all 25 addresses came from
  the untranslated part of the `0x802B0000` bucket.
- The module header gained item 16; its body count is 25 and the file total is
  509.
- `node tools/check-lane-csv.mjs wave99` reports 25 rows for
  `wave99-archive.csv` and `OK`.
- `cargo check --message-format=short` reports no error or warning for
  `src/resource/archive.rs`; the remaining errors name other lanes' files from
  the same wave, which are being edited in parallel.
