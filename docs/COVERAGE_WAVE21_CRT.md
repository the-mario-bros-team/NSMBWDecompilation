# Coverage — Wave 21 CRT continuation (`0x802D0000..0x802E0000`)

## How to read this file

This is the completion record for wave 21 of the Metrowerks runtime / CRT lane in
[`src/runtime/cpp.rs`](../src/runtime/cpp.rs). It answers five questions for a
reader with no context:

1. **What was added?** Twenty-eight function bodies: CodeWarrior array and
   variadic helpers, the small-block allocator's size-class path, the buffered
   `stdio` transfer core, the wide-character locale shims and the integer /
   floating formatters used by `printf`.
2. **Where is it?** Exclusively in [`src/runtime/cpp.rs`](../src/runtime/cpp.rs),
   appended after the existing `0x802D0000..0x802E0000` block. No other Rust
   file, `mod.rs`, `lib.rs`, `Cargo.toml` or `platform.rs` was edited.
3. **How sure is it?** `confidence` measures the evidence, not the code style:
   `high` = every branch, store and call operand is explicit in the Ghidra C or
   raw instructions; `medium` = the flow is complete but one operand, callee
   argument or field name had to be recovered from surrounding code.
4. **What is still unknown?** See **Uncertainties** and **Deliberately skipped**.
5. **What is the entry point?** Read `fn_802dc7e0` (array construction) first,
   then `fn_802dd674` (small-block free) and `fn_802de614` (buffered write).

Addresses are plain lowercase hex virtual image addresses (image base
`0x80004000`), not file offsets. Evidence is the per-function Ghidra C file
under `work/ghidra/main-gb/export/`. Every Rust function keeps an `# Original`
block with the address, size and evidence path. The raw `work/dol/main.elf` was
used for the register-save skips and for the call targets that Ghidra dropped.

Terms: **DOL** is the Wii executable container; **CRT** is the C/C++ runtime;
**r13** is the PowerPC small-data base (`0x8042F680`); **r2** is the second
small-data base (`0x80433080`); a **decimal record** below means the
CodeWarrior float-printing structure `{sign: u8, exponent: i16, length: u8,
digits: [u8; 36]}`. See `docs/GLOSSARY.md`.

Status follows `docs/RUST_STYLE.md`: `translated` = the full control flow and
every call are represented; `partial` = control flow is represented but one
operand or callee body is missing; `stub` = address/role only.

## Summary

| Metric | Count |
|---|---:|
| New function bodies in this pass | **28** |
| Bytes represented | **6,912** |
| Status `translated` | 28 |
| Status `partial` | 0 |
| Status `stub` | 0 |
| Confidence `high` | 22 |
| Confidence `medium` | 6 |
| Module | `runtime::cpp` |
| Evidence | `work/ghidra/main-gb/export`, `work/dol/main.elf` |

The 6,912 bytes are the sum of the 28 `functions.tsv` sizes: 76 + 200 + 260 +
188 + 248 + 48 + 168 + 60 + 52 + 168 + 492 + 164 + 108 + 648 + 232 + 796 + 40 +
776 + 308 + 76 + 28 + 192 + 184 + 96 + 24 + 16 + 576 + 688.

## Selection

The lane brief asks for 25–30 untranslated, meaningful functions in the band,
covering allocator, string/memory, integer/float math, C++ runtime, `stdio` and
save/restore stubs, while skipping 4-byte thunks and pure register-save
fragments. Selection was made in this order:

1. List every `functions.tsv` entry in `0x802D0000..0x802E0000`.
2. Remove entries that already have a `runtime::cpp` row in
   `ledger/functions.csv` or a body in `src/runtime/cpp.rs`.
3. Prefer the numeric/allocator/`stdio` cluster `0x802DC5D0..0x802DFCB4` over
   the EGG middleware and render-adjacent code at the start of the band.
4. Read each candidate's Ghidra C and keep only functions whose body can be
   represented without inventing a symbol or a callee argument.

The result is 28 bodies. Of these, 22 are `high` confidence and 6 are `medium`.
The medium cases are listed under **Uncertainties**.

## Translated functions

| Address | Rust name | Role (plain English) | Status | Conf. | Evidence |
|---|---|---|---|---|---|
| `802dc5d0` | `fn_802dc5d0` | Sets or clears two flag groups in a state object. | translated | high | `802dc5d0_FUN_802dc5d0.c` |
| `802dc6b8` | `fn_802dc6b8` | Fetches the next variadic argument by class. | translated | medium | `802dc6b8_FUN_802dc6b8.c` |
| `802dc7e0` | `fn_802dc7e0` | Constructs a `new[]` array and its 16-byte cookie. | translated | high | `802dc7e0_FUN_802dc7e0.c` |
| `802dc8e4` | `fn_802dc8e4` | Destroys an array's live elements and optionally frees it. | translated | high | `802dc8e4_FUN_802dc8e4.c` |
| `802dc9a0` | `fn_802dc9a0` | Constructs a plain array's elements. | translated | high | `802dc9a0_FUN_802dc9a0.c` |
| `802dcb90` | `fn_802dcb90` | Tests whether any of three words is non-zero. | translated | high | `802dcb90_FUN_802dcb90.c` |
| `802dd238` | `fn_802dd238` | Converts a `double` to an unsigned 64-bit integer. | translated | high | `802dd238_FUN_802dd238.c` |
| `802dd2e0` | `fn_802dd2e0` | Registers the runtime exit slot once. | translated | medium | `802dd2e0_FUN_802dd2e0.c` |
| `802dd31c` | `fn_802dd31c` | Releases the runtime exit slot once. | translated | high | `802dd31c_FUN_802dd31c.c` |
| `802dd5cc` | `fn_802dd5cc` | Merges a free block with its free successor. | translated | high | `802dd5cc_FUN_802dd5cc.c` |
| `802dd674` | `fn_802dd674` | Returns a block to the small-block pool. | translated | medium | `802dd674_FUN_802dd674.c` |
| `802dd990` | `fn_802dd990` | Flushes and closes every registered stream. | translated | high | `802dd990_FUN_802dd990.c` |
| `802dda34` | `fn_802dda34` | Flushes every registered stream. | translated | high | `802dda34_FUN_802dda34.c` |
| `802ddb7c` | `fn_802ddb7c` | Multiplies two decimal digit strings. | translated | high | `802ddb7c_FUN_802ddb7c.c` |
| `802dde04` | `fn_802dde04` | Loads and rounds a decimal digit string. | translated | high | `802dde04_FUN_802dde04.c` |
| `802ddef0` | `fn_802ddef0` | Expands a decimal power-of-two / ten-power entry. | translated | medium | `802ddef0_FUN_802ddef0.c` |
| `802de534` | `fn_802de534` | Resets a stream's buffer window. | translated | high | `802de534_FUN_802de534.c` |
| `802de614` | `fn_802de614` | Buffered array write (the core of `fwrite`). | translated | medium | `802de614_FUN_802de614.c` |
| `802de9d8` | `fn_802de9d8` | Flushes one stream (the core of `fflush`). | translated | high | `802de9d8_FUN_802de9d8.c` |
| `802ded94` | `fn_802ded94` | Converts one single-byte character to wide. | translated | high | `802ded94_FUN_802ded94.c` |
| `802dede0` | `fn_802dede0` | Stores one byte through an output pointer. | translated | high | `802dede0_FUN_802dede0.c` |
| `802dedfc` | `fn_802dedfc` | Converts a multibyte string to wide (`mbstowcs`). | translated | high | `802dedfc_FUN_802dedfc.c` |
| `802deebc` | `fn_802deebc` | Converts a wide string to multibyte (`wcstombs`). | translated | high | `802deebc_FUN_802deebc.c` |
| `802df3b4` | `fn_802df3b4` | Classifies a float converted from a `double`. | translated | high | `802df3b4_FUN_802df3b4.c` |
| `802df414` | `fn_802df414` | Returns the sign bit of a `double`. | translated | high | `802df414_FUN_802df414.c` |
| `802df4a8` | `fn_802df4a8` | Installs the close-all-streams hook. | translated | high | `802df4a8_FUN_802df4a8.c` |
| `802dfa74` | `fn_802dfa74` | Formats a 32-bit integer into a buffer. | translated | high | `802dfa74_FUN_802dfa74.c` |
| `802dfcb4` | `fn_802dfcb4` | Formats a 64-bit integer into a buffer. | translated | medium | `802dfcb4_FUN_802dfcb4.c` |

## What each cluster does

### C++ array helpers (`802dc5d0..802dc9a0`)

`fn_802dc7e0` is the `new[]` constructor: it writes `{stride, count}` at the
start of the allocation, builds elements from `header + 0x10`, and calls the
constructor as `constructor(element, 1)`. Its plain sibling `fn_802dc9a0`
builds from a raw element pointer and has no cookie. `fn_802dc8e4` is the
matching container destructor: it walks the live prefix from the highest
element down, calls `destructor(element, -1)`, then optionally frees the
descriptor with `0x802B9280`. `fn_802dc5d0` is the two-word flag helper used by
the stream/locale state objects, and `fn_802dc6b8` is the variadic fetch that
reads the save-area index from a `va_list`-style descriptor.

### Allocator (`802dd238..802dd674`)

`fn_802dd238` is the 64-bit companion of the already translated
`fn_802dcc4c`: it extracts the IEEE-754 exponent, rejects negative values and
magnitudes below `1.0`, then shifts the 53-bit significand by
`exponent - 0x433`, saturating at `2^64`.

`fn_802dd674` is the small-block half of `free`. The six size-class limits at
`0x802FD3C0` are `4, 12, 20, 36, 52, 68`. The control node is reached through
the word at `pointer - 4`; its `+0xC` word is the class free head and its
`+0x10` word is a reference count. When the count reaches zero the node is
unlinked, its page is merged with `0x802DD47C`, and an empty page is removed
from the pool list and released with `0x802DD3C4`. `fn_802dd5cc` is the
forward-coalescing half of the same allocator (`0x802DD47C` calls it).

`fn_802dd2e0` and `fn_802dd31c` are the register/release pair for the runtime's
exit registry slot at `0x80429BA8`. The slot starts as `-2` ("not
registered"); registration passes the address of the handler word at
`0x8000675C` and `r2 = 0x80433080` to `0x802DD350`.

### `stdio` (`802dd990..802deebc`)

The `FILE` list head is `0x80350A18`, linked through `+0x4C`. `fn_802dd990`
closes every open stream with `0x802DE91C` and frees streams whose `+0x0C` byte
marks owned storage; `fn_802dda34` only flushes them with `fn_802de9d8`.
`fn_802de9d8` is the `fflush` core: a null argument fans out to
`fn_802dda34`, otherwise it inspects the `+8` state and the `+4` mode bits,
uses `fn_802deb0c` (`ftell`) for a read stream, then calls `fn_802de55c`.

`fn_802de614` is the `fwrite` core. It locks the stream with `0x802E44F8`,
honours append mode by seeking to the end, computes the candidate chunk from
the stream's free space, stops line-buffered writes after the last `'\n'`,
copies with `0x80004364`, and flushes with `fn_802de55c`. `fn_802de534` resets
the stream's buffer window after a direct/unbuffered transfer.

`fn_802ded94`, `fn_802dede0`, `fn_802dedfc` and `fn_802deebc` are the
single-byte and whole-string locale shims. The string converters load the
locale table pointer from `0x80350E68` and call slots `+0x20`
(multibyte → wide) or `+0x24` (wide → multibyte). `0x802E19F8` is the byte
copy used by the wide → multibyte path.

### Decimal float formatting (`802ddb7c..802ddef0`)

These three functions implement CodeWarrior's software decimal expansion for
`%f`/`%e` conversion. A decimal record is
`{sign: u8 at +0, exponent: i16 at +2, length: u8 at +4, digits at +5}`.
`fn_802ddb7c` multiplies two records with a convolution loop, a 76-byte stack
accumulator and a 36-digit half-to-even rounding tail. `fn_802dde04` loads an
ASCII digit string and rounds it. `fn_802ddef0` expands `2^n` by repeated
squaring and uses the digit strings at `0x802FD3D8` through `0x802FD48C` for
the negative ten-power cases.

### Integer formatting (`802dfa74`, `802dfcb4`)

Both functions emit digits backwards from `end - 1`. The conversion record is
`{recompute_width: u8 at +0, sign_mode: u8 at +1, alternate: u8 at +3,
conversion: u8 at +5, width: i32 at +8, precision/pad: i32 at +0xC}`. They
support `d`, `i`, `o`, `u`, `x` and `X`, the `#` alternate form, width padding,
and the `-`, `+` and space prefixes. The 64-bit version replaces the original
shift-subtract divide/modulo helpers with native `/` and `%`, which produce the
same quotient and remainder.

### Remaining helpers

`fn_802dcb90` is a three-word zero test. `fn_802df3b4` maps a narrowed `f32` to
CodeWarrior's class codes (`1` NaN, `2` infinity, `3` zero, `4` normal,
`5` subnormal). `fn_802df414` returns the high-word sign mask of a `double`.
`fn_802df4a8` installs `fn_802dd990` at `0x8042AFF8`.

## Uncertainties

* `fn_802dd2e0` registers the **address** `0x8000675C` rather than a code
  pointer. The bytes there are zero-initialised, so the consumer probably
  dereferences the slot later; the exact handler shape is `// unknown:` in the
  Rust doc.
* `fn_802dd674` names the node fields `+0xC` (free head) and `+0x10`
  (reference count) from their update order. Both names are `// inferred:`;
  the arithmetic and list surgery are fully transcribed.
* `fn_802dc8e4` describes the descriptor as a growable array
  (`base/stride/capacity/destructor/live-count`). The field roles are inferred
  from the descending destructor loop; the loop itself is exact.
* `fn_802ddef0`'s role name (`decimal power expansion`) is inferred from the
  `2^n` and ten-power inputs and from its only caller, `fn_802de20c`, which is
  not translated here.
* `fn_802de614` restores two arguments Ghidra dropped: the `len` argument of
  `0x80004364` and of `fn_802df06c`. Both are the surrounding
  `local_28[0]`/`amount` value; the evidence is the register trace at
  `0x802DEF5C..0x802DEF70` and the buffer-advance arithmetic.
* `fn_802dfcb4` is marked `medium` only because the original split each
  64-bit division into separate quotient/remainder helpers. The reconstruction
  uses native operators; the digit order, prefixes and rounding are unaffected.

## Deliberately skipped

These addresses are **not** translated and have no ledger row from this pass:

* `802d2f10`, `802d3260`, `802debb4` — 4-byte `b` thunks to
  `0x801C76C0`, `0x801C9CD0` and `0x802DEB0C`. The destination of each thunk is
  already represented (or is called directly where needed).
* `802dcd18`, `802dcd2c`, `802dcdc0` — empty `return;` bodies (alternate
  register-save entry points).
* `802dcd40..802dcdbc` — the CodeWarrior `__savegprlr_N` / `__restgprlr_N`
  family. `functions.tsv` lists each as a 4-byte function because every `stw`
  or `lwz` is a valid alternate entry; they all share one `blr` and are not
  callable high-level routines. See `docs/QUALITY.md` §5.5.
* `802dcccc`, `802dcce0` and the other FPR save/restore fragments — pure
  register-save prologues with no reachable high-level body.
* The formatter at `802dff64` — its C contains the paired-single `ldexpf` /
  `unaff_GQR0` artefacts described in `docs/QUALITY.md` §5.3. It is left for a
  GQR-aware pass rather than translated from the current evidence.

The EGG middleware at the start of the band (for example `802d35c0`,
`802d3700`, `802d39b0`) is also untouched: it belongs to the engine lanes, not
to the CRT.

## Verification

* `node tools/doc-lint.mjs src` → `{"ok":true,"files":68,"violations":0}`.
* `cargo check` finishes successfully. The 11 remaining crate warnings are in
  other lanes' files; no error or warning points at `src/runtime/cpp.rs`.
* `node tools/verify-ledger.mjs` → 3,542 rows, no duplicate addresses, no
  missing Rust files and no missing export files. The 28 new `runtime::cpp`
  rows are present exactly once.
