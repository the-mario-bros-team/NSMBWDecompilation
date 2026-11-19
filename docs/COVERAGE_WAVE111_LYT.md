# Coverage - Wave 111 (`ui::lyt`)

## How to read this file

- **Scope** - the three functions that were still untranslated in the
  `0x80180000` bucket. The other 130 addresses of that bucket are already in
  `src/ui/lyt.rs` from waves 3-30 and are not repeated here.
- **Addresses** are virtual `sys/main.dol` addresses (image base `0x80004000`),
  not file offsets.
- **Evidence** is the Ghidra C export under
  `work/ghidra/main-gb/export/`; each row names its file.
- **Status** follows `docs/RUST_STYLE.md`: `translated` = full control flow and
  every call accounted for; `partial` = flow and major effects present, one
  operand not literal; `stub` = address and role only.
- **Confidence** measures the evidence, not code quality. `medium` marks a
  function whose bytes, offsets and control flow are exact but whose array
  roles or register-only operands were inferred from the call shape.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/lyt.rs` | `ui::lyt` | 3 | 5,936 | 0 | 3 |

## What this pass covers

The `0x80180000` bucket held 133 functions, 130 of them already translated by
waves 3-30. This pass translates the last three, so the bucket is closed. All
three sit in the NW4R `lyt` animation/text band and reuse bodies that earlier
waves had already landed in `src/ui/lyt.rs`.

- **Animation-object constructor** - `0x80180780` builds the 0x5C-byte object
  that `fn_801846a0`, `fn_80184c60` and `fn_801860b0` allocate at `+0x28`. It
  installs the vtable `0x8032B2C8`, starts the embedded list at `+0x18..+0x24`,
  seeds and then overwrites the twelve colour halfwords at `+0x24`, copies the
  0x14-byte name and the four resource words at `+0x2C..+0x38`, decodes the pack
  word at resource `+0x3C` into six counts, and sizes the work buffer through
  the wave-20 `fn_801819c0`. When the buffer exists it copies the resource
  arrays into the slots the packed words `+0x4C`/`+0x50` reserve: the `'timg'`
  texture patterns (bound through `fn_8017f250` after a virtual `+0x0C`
  resource lookup), two 0x14-byte transform arrays, two 4-byte pattern arrays,
  a 0x10-byte array, and up to five optional 4-byte words.
- **Line measurement pair** - `0x80185870` measures one line of a `u16` string
  into four floats: it seeds the rectangle from the writer's line extent, walks
  the characters, unions the tag processor's rectangle (`writer + 0x5C`,
  virtual `+0x10`), advances printable characters through the glyph-width
  virtual, and stops as soon as the rectangle is wider than the `f1` width,
  setting the caller's overflow byte and reporting the character count that
  fit. `0x80185F10` is the string-level wrapper: it clears the rectangle,
  resets the writer cursor, calls the line helper once per line, re-measures the
  single character at `0x8032B430` when a line overflows, unions the result and
  advances the cursor by `2 * consumed` bytes.

## Functions

| address | function | role | status | confidence | bytes | evidence |
|---|---|---|---|---:|---:|---|
| `0x80180780` | `fn_80180780` | construct the layout animation object from its resource | translated | medium | 4484 | `80180780_FUN_80180780.c` |
| `0x80185870` | `fn_80185870` | measure one line of a `u16` string and flag overflow | translated | medium | 1092 | `80185870_FUN_80185870.c` |
| `0x80185F10` | `fn_80185f10` | measure a whole `u16` string into a four-float rectangle | translated | medium | 360 | `80185f10_FUN_80185f10.c` |

Evidence paths are relative to `work/ghidra/main-gb/export/`; the CSV carries
the full repo-relative path.

## Cluster notes

### Why `medium` and not `high`

Every offset, clamp, mask and destination expression is copied literally from
the export, but three things are inference rather than proof:

1. **Array roles.** The animation object names its six resource arrays only by
   packed bit position. The Rust body keeps the packed arithmetic and labels the
   arrays `A`..`F` by stride (0x20, 0x14, 4, 0x14, 4, 0x10); no NW4R member
   names are invented.
2. **The `f1` width operand.** Ghidra prints it as the save-helper's
   `extraout_f1` instead of a parameter. Both measurement bodies take it as
   `width: f32`, captured at entry, and pass it through.
3. **The character thunk.** `0x802DCBFC` is called with the address of the
   32-bit cursor word (`&local_a8`), and the thunk advances it; the encoding
   table it dispatches through is selected in `r12` by the caller and is not
   modelled. This is documented on the `next_text_char` helper.

### Constants kept as addresses

- `0x802F5E50`: four `u16` defaults written to `object + 0x24` before the
  resource copy. Their values are not in the export, and the copy replaces them
  unconditionally, so the body reads the constant block instead of inventing
  values.
- `0x8032B430`: the one-character string measured when a line overflows. Only
  its address appears in the export.
- `0x800046B4` (`memset`), `0x801C7100`, `0x802DC9A0`, `0x8017E490`,
  `0x80180770` and `0x802E19F8` are out-of-lane callees and stay address
  constants; the first five were already declared by the wave-7 section.

### Deliberate skips

None. All three bucket functions were reconstructible, so the bucket is closed.
`0x80185870` keeps its `/ 2` character accounting and the `80185F10` `* 2`
cursor advance exactly as the export prints them; the two agree, which is the
reason the pair is documented as a `u16` string API.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (`80180780` = 4484, `80185870` = 1092, `80185f10` = 360).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in `src/ui/lyt.rs`.
- No address was already in `ledger/functions.csv` (checked before writing).
- `cargo check --message-format=short` reports no error and no diagnostic naming
  `src/ui/lyt.rs`.
- `node tools/check-lane-csv.mjs wave111` reports no problem for
  `wave111-lyt.csv`.
