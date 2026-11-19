# Coverage - Wave 100 (runtime::cpp)

## How to read this file

Each row of **Functions** is one `pub unsafe fn fn_<address>` body this pass
added to `src/runtime/cpp.rs`. The `bytes` column is the size from
`work/ghidra/main-gb/export/functions.tsv`, re-read for this pass; the body's
`# Original` block cites the same export file. Confidence matches the CSV row.

The `0x802D0000` bucket held 354 functions. Wave 99 left 35 of them unclaimed
and this pass was given that remainder. Three of the 35 are real, distinguishable
routines and are translated below. The other 32 are the Metrowerks callee-save
chain; they are **deliberately** left without a body and listed under
*Deliberate skips*. This lane is short on purpose: padding it would have meant
inventing behaviour for byte sequences whose only effect is on registers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/cpp.rs` | `runtime::cpp` | 3 | 1,324 | 0 | 3 |

## What this pass covers

Address range: `0x802DADD0..0x802DFF64`.

- **`0x802DCBFC` - the pointer-to-member-function call thunk.** `r12` carries a
  CodeWarrior member-pointer descriptor `{this_delta, vtable_index, target}`.
  The ten instructions are `lwz r0,0(r12)`, `lwz r11,4(r12)`,
  `lwz r12,8(r12)`, `add r3,r3,r0`, `cmpwi r11,0`, `blt` past the virtual hop,
  two `lwzx` (table at `target + adjusted_this`, slot `vtable_index`),
  `mtctr r12` and `bctr`. Because the dispatch is a `bctr` tail call, the
  callee's `r3` reaches this function's caller unchanged, so the Rust signature
  returns it. `r12` is not an argument register, which is why the export shows an
  empty argument list and `UNRECOVERED_JUMPTABLE`; the body takes the descriptor
  as an explicit second argument.
- **`0x802DADD0` - the look-at angle recompute.** The direction starts as the
  local Y axis `(0x8042F650, 0x8042F654, 0x8042F650)`, which the image
  initialises to `(0.0, 1.0, 0.0)`. Two 3x4 matrices are built on the stack: the
  object's own (`0x80289070`, with its translation column overwritten by the seed
  at `0x2C/0x3C/0x4C`) and a basis from `state+0x5C/+0x60/+0x64` scaled by
  `0x8042F658` (`0x802382B0`). The vector passes through both with `0x801C1160`,
  takes the `ps_add` of the three floats at `+0x2C` of the sub-object at
  `+0x154`, is normalised (`0x801C1470`, already classified in `runtime::dvd`),
  and, when `0x801C07A0` inverts the object matrix, is transformed again and
  decomposed: `acos(y)` to `+0x84`, `atan2(x, z)` to `+0x88`, the seed to `+0x8C`.
- **`0x802DFF64` - the `%a`/`%A` hexadecimal-float formatter.** The buffer is
  filled backwards: [sign][`0`][`x`/`X`][leading digit][decimal point][mantissa
  nibbles][`p`/`P`][exponent]. The leading digit is `0` only when
  `|value|` equals the `double` at `0x8042F6A0` (the image stores `0.0`). The
  mantissa nibbles are the bits of the double starting at bit `4*precision+11`
  and walking down by four; indices past bit 63 print `0`. Special values come
  from the string table at `0x802FD9C0` (`"-INF"`, `"-inf"`, `"INF"`, `"inf"`,
  `"-NAN"`, `"-nan"`, `"NAN"`, `"nan"`), selected by the record's type byte and
  by the `'A'` conversion flag. `spec+0xC` is the precision, `spec+1` the sign
  mode (`1` = `+`, `2` = space), `spec+3` the `#` flag and `spec+5` the `a`/`A`
  conversion.

### Why the two paired-single bodies are translatable

`docs/QUALITY.md` section 5.3 says not to translate float code from this C while
the `GQR0` value is unknown. For these two functions the value is known:

- `psq_l`/`psq_st` with `I=0` take both their type and their scale from `GQR0`.
  The image writes `GQR0` in exactly two places: `li r3,0` followed by
  `mtspr 912..919` at `0x801AB170..0x801AB190` (all eight clear) and the context
  restore at `0x802EA6FC`, which reloads the GQRs from the save area that the
  matching `mfspr`/`stmw` routine at `0x802EA684` filled at `0x2FC(r2)`. No
  instruction loads a non-zero value into `GQR0`.
- The type-0 reading is also the only self-consistent one: `0x802DADD0` stores a
  matrix-transformed float vector with `psq_st` and immediately does float
  arithmetic on it, and `0x802DFF64`'s `psq_st`/`psq_l` pair at
  `0x802DFF78`/`0x802E0308` brackets an `f31` save next to the `stfd`/`lfd` pair
  at `0x802DFF74`/`0x802E030C`, i.e. a compiler spill of a callee-saved FPR.

The two `ldexpf` blocks the export prints (`0x802DFF64` and `0x802E0304`) are
those spills: their results are discarded, so they are not modelled.

### New names introduced

`SDA2_AIM_AXIS_X`, `SDA2_AIM_AXIS_Y`, `SDA2_AIM_SCALE`, `SDA2_F64_ZERO`,
`DECIMAL_POINT_SLOT` and `BUILD_TRANSLATION_MATRIX`. `DECIMAL_POINT_SLOT` is
deliberately distinct from the existing `LOCALE_TABLE`: the formatter reads the
pointer slot at `0x80350C80`, while `LOCALE_TABLE` names `0x80350E68`. No
existing constant or body was renamed.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802DADD0` | `fn_802dadd0` | 316 | medium |
| `0x802DCBFC` | `fn_802dcbfc` | 40 | medium |
| `0x802DFF64` | `fn_802dff64` | 968 | medium |

## Deliberate skips

32 addresses. All of them belong to one Metrowerks callee-save run; they are not
"unanalysed", they are unrepresentable in Rust and excluded by
`docs/QUALITY.md` section 5.5.

| range | count | bytes each | what the bytes are |
|---|---:|---:|---|
| `0x802DCCCC`, `0x802DCCE0` | 2 | 20 | `stfd f23..f31` spill run into the caller's frame (`r11-0x48..-8`) |
| `0x802DCD18`, `0x802DCD2C` | 2 | 20 | `lfd f23..f31` restore run; Ghidra prints `return;` |
| `0x802DCD40..0x802DCD70` | 13 | 4 | `__savegprlr_14..__savegprlr_26`: alternate entry points into one `stw r14,-0x48(r11)..stw r31,-4(r11)` run |
| `0x802DCD74` | 1 | 24 | `__savegprlr_27`: the `stw r27..r31` tail of the same run |
| `0x802DCD8C..0x802DCDBC` | 13 | 4 | `__restgprlr_14..__restgprlr_26`: the matching `lwz` entries |
| `0x802DCDC0` | 1 | 24 | `__restgprlr_27`: the `lwz r27..r31` tail |

Reasons, in the order the brief asks for them:

1. **Each is an alternate entry into one instruction sequence.** `functions.tsv`
   sizes ten of them as 4 bytes because a single `stw`/`lwz` is a valid branch
   target; the decompiler still prints the whole run for each. A Rust body would
   have to claim that `fn_802dcd44` "is" one store to a caller frame slot with a
   register value it cannot name, and would have to repeat that for thirteen
   sibling labels.
2. **The only effect is on callee-saved registers and the caller's save area.**
   The reconstruction omits these calls in every caller for the same reason
   (`src/game/stage.rs`, `src/game/profile.rs`, `src/engine/display.rs` all say
   so in their prologue notes). Writing a body would not add one bit of
   behavioural information, and it would contradict the module's own header,
   which already records the range as excluded.
3. **No entry has a `# Original` size that matches a whole function.** The
   `functions.tsv` sizes are 4/20/24 bytes while the exports are 40-90 lines of
   spills; the two 20-byte restores decompile to `return;`. Neither number can be
   cited as "the function", which the ledger format requires.

Everything else in the bucket is either already in `ledger/functions.csv` (319
addresses) or one of the three bodies above. Nothing was skipped because it was
hard; the three landed functions were the only remaining code with observable
behaviour.

## Verification

- Every address and byte size was re-read from `functions.tsv` for this pass:
  `802dadd0` 316, `802dcbfc` 40, `802dff64` 968.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every one of the 3 rows has exactly one `pub unsafe fn` body in
  `src/runtime/cpp.rs`.
- No address was already in `ledger/functions.csv`; the three were the exact
  remainder of the bucket after Wave 99.
- `cargo check --message-format=short` reports no error in `src/runtime/cpp.rs`;
  the file's only warnings are the two pre-existing `unused_assignments`
  warnings at lines 8015 and 8871, outside this pass's section.
- `node tools/check-lane-csv.mjs wave100` reports `wave100-cpp.csv: 3 rows` and
  `OK` for this lane.
