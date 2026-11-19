# Coverage - wave 30 NW4R layout record builders (`0x80191250..0x801942C0`)

## How to read this file

- **Scope** - the thirtieth `lyt` translation pass. It adds five bodies to the
  semantic module `src/ui/lyt.rs` and creates no staging module. The pass
  finishes the two record builders that wave 23 left deferred, then adds the
  record timing builder and the `MTrk` track parser that feed the same window
  records.
- **Addresses** are virtual `sys/main.dol` addresses (image base
  `0x80004000`), written as plain hex (`80191250`) in the CSV and with the
  `0x` prefix in this document and in the Rust doc comments.
- **Evidence** is the Ghidra C export under
  `work/ghidra/main-gb/export/`; every row names its file.
- **Status** follows `docs/RUST_STYLE.md`: `translated` = complete control
  flow and every call accounted for; `partial` = flow and major effects
  present, one operand not literal; `stub` = address/role only.
- **Confidence** measures the evidence, not code quality. `medium` marks a
  body whose branches and stores are exact but whose float width or hidden
  operands were recovered from the caller and the constants.
- Start with the **summary**, then the **function table**, then the **band
  audit**. The audit explains why this pass has five bodies rather than the
  nominal twenty: the layout band was already translated apart from nine
  addresses, and four of those are outside this pass's reach (two use
  paired-single state, one is a 4,484-byte builder, one is audio code).

## Summary

| metric | value |
|---|---:|
| functions added to `src/ui/lyt.rs` | 5 |
| bytes translated | 6,196 |
| `translated` / `partial` / `stub` | 5 / 0 / 0 |
| high confidence | 3 |
| medium confidence | 2 |
| new staging modules | 0 |
| files written | `src/ui/lyt.rs`, this document, `ledger/agents/wave30-lyt.csv` |

Address span: `0x80191250..0x801942C0`. Every selected address was absent from
`ledger/functions.csv` and from the `fn_` definitions in `src/ui/lyt.rs`, and
every body exists in the Ghidra export. No ledger row was created from a guess.

## Function table

| address | Rust function | role | status | conf. | bytes | evidence |
|---|---|---|---|---|---|---|
| `0x80191250` | `fn_80191250` | set the global text-window mode | translated | high | 12 | `80191250_FUN_80191250.c` |
| `0x80191260` | `fn_80191260` | build one record and push its initial twelve channel angles | translated | high | 2,376 | `80191260_FUN_80191260.c` |
| `0x80191D20` | `fn_80191d20` | flush the sixteen records' flags and angle/delta channels | translated | high | 2,808 | `80191d20_FUN_80191d20.c` |
| `0x80193220` | `fn_80193220` | build one record's frame-timing fields | translated | medium | 716 | `80193220_FUN_80193220.c` |
| `0x801942C0` | `fn_801942c0` | parse `MTrk` chunks into 0x28-byte command records | translated | medium | 284 | `801942c0_FUN_801942c0.c` |

All evidence paths are relative to `work/ghidra/main-gb/export/`.

## Band audit (why five)

The range `0x80180000..0x80195000` contains 197 functions in `functions.tsv`.
Before this pass the merged ledger held 188 rows covered by `ui::lyt`,
`runtime::ids` and `audio::ax`. The remaining nine addresses were:

| address | size | disposition |
|---|---:|---|
| `0x80191250` | 12 | translated here |
| `0x80191260` | 2,376 | translated here |
| `0x80191D20` | 2,808 | translated here |
| `0x80193220` | 716 | translated here |
| `0x801942C0` | 284 | translated here |
| `0x80180780` | 4,484 | deferred: pane/anim-transform resource constructor |
| `0x80185870` | 1,092 | deferred: paired-single/GQR text-measure body |
| `0x80185F10` | 360 | deferred: paired-single/GQR text-measure wrapper |
| `0x80194DA0` | 68 | not layout: audio teardown (same `r13 - 0x51D8/-0x51D0` globals as `0x80194DF0`) |

The scan was re-run by exact address rather than by `fn_` substring. That
matters: `src/ui/lyt.rs` already contained uppercase `FN_80191260`,
`FN_80193220` and `FN_801942C0` call constants, so a case-insensitive name
test hides the missing bodies. After this pass, every non-paired-single
layout body below `0x80195000` is translated except the 4,484-byte
`0x80180780` described below.

The two paired-single bodies were already deferred by
`docs/COVERAGE_r80180_801c0.md` and `docs/COVERAGE_WAVE25_LYT.md`; their Ghidra
output contains `unaff_GQR0`/`ldexpf` reconstruction blocks (24 and 13
occurrences) and cannot be lowered faithfully until the paired-single model
exists.

The rest of the lane is not layout code: `0x80195010` and its neighbours
call the sound layer (`0x8019B220`, `0x80275760`), `0x801A0000..0x801B0000`
is AX audio, and `0x801B0000..0x801C0000` is WPAD/KPAD/NAND. A scan of every
untranslated `.c` file above `0x80195000` for the layout globals
(`0x803780E8`, `0x803780EC`, `0x80378730`, `0x80378C08`, `0x8032C940`) returns
no match, so no `lyt` body is hidden there.

## Cluster notes

### A. Record mode and creation (`0x80191250`, `0x80191260`)

`0x80191250` is the missing writer of `0x803780E8`, the mode word read by
every window-record layout branch. It stores its argument with no range check,
so values outside `0/1/2` leave both builders' mode branches unentered.

`0x80191260` is the create path that wave 25's `0x80192E60` already calls by
address after building a message record. It resolves the record slot through
`0x80378088`/`0x803780EC`, stamps the record with the layout object and the
four caller values, runs `0x80190FB0` to select the colour/offset tables, and
then:

- stores the signed `index` angle at record `+0x30` through the halfword table
  at `0x8032C940`;
- computes six angles in mode `0`/`1` or eight in mode `2` from record fields
  `+0x0C..+0x2C`, writing `+0x34..+0x5C` and clearing the unused slots;
- under `0x801B1140`/`0x801B1180`, copies twelve angle values to the layout
  object `+0x3C..+0x68`, clears each delta halfword, and raises the
  corresponding presence bit in the object word at `+0x34`;
- sets object state `0x112`, clears 0x20 bytes at `+0x102`, and sets state
  `0x3000000`.

Mode `2` starts the object flag word at `0x80000000`; modes `0` and `1` start
at zero. The twelve channel order is not sequential: channels seven to twelve
read record `+0x58`, `+0x5C`, `+0x3C`, `+0x48`, `+0x54`, `+0x60` and write
object `+0x54`, `+0x58`, `+0x5C`, `+0x60`, `+0x64`, `+0x68`.

### B. Per-frame flush (`0x80191D20`)

`0x80191D20` is the body menu code calls once per update. It returns
immediately unless the wave-25 table guard at `0x80378730` is set, then walks
all sixteen 100-byte records and skips records with a null layout pointer.

The record flag word at `+0x04` carries four dirty states:

| bit | action |
|---|---|
| `0x20000000` | copy pending angle `+0x32` to `+0x30`; clear the bit |
| `0x10000000` | rebuild `+0x32` from the `+0x08` index; replace with `0x20000000` |
| `0x80000000` | shift the twelve angles one slot forward (`+0x36`→`+0x34`, ...); clear |
| `0x40000000` | rebuild six/eight angles by mode; replace with `0x80000000` |

After the flag work it writes the angle pair to the object at `+0x92`/`+0x94`
(the latter is `(record+0x32 - record+0x30) / 0x60` on zero-extended
halfwords) and state `0x100`. If the shift or rebuild ran, it writes the
twelve object angle/delta pairs, the presence word at `+0x34`, and state
`0x12`. Mode `2` seeds the presence word with `0x80000000`, exactly as the
create body does.

### C. Frame timing (`0x80193220`)

The `0x80192E60` create body calls `0x80193220(record)` immediately after
filling a message record. The body reads the text object at `record + 0x14`
and writes six fixed-point words at `record + 0x30..+0x48`.

When `object + 0x18` is the `0x80000000` sentinel the record becomes a
two-state marker (`+0x30 = 1`, `+0x34 = 0`, upgraded to state `2` when
`object + 0x1C` is also the sentinel). Otherwise the fixed-point rate is
converted through `0x802E55A4(2.0, x)`, scaled by `1000`, divided by three,
and stored as `0x640000 / divisor` (or the `0x640000` default when the
divisor is zero). `+0x34` receives the `-0x3C00000` (-960.0 in 16.16)
sentinel. The conversion is repeated from `object + 0x1C` and the row byte at
`record + 0x0D` into `+0x40` when `+0x30` is below two. The last two words
copy `object + 0x20`/`+0x24` unchanged.

### D. `MTrk` parser (`0x801942C0`)

The parser walks the chunk stream passed by `0x801943E0`: each chunk is a
`u32` tag, a `u32` payload size and the payload, and the scan advances by
`size + 8` until the tag equals `0x4D54726B` (`MTrk`). Each accepted chunk
fills one 0x28-byte command record at `object + 0x241C`:

- `+0x00` owner, `+0x04` payload start, `+0x08` payload end, `+0x0C` cursor;
- `+0x18` the frame value
  `0x802DCC4C((float)(65536 * (96 / (16000 / object[+0x0A]))))`;
- `+0x24` zero.

The record stride is ten words. `0x801943E0` passes `1` when the descriptor's
first halfword is zero and omits the count otherwise; the parser itself has
no bound on the chunk scan, which is preserved.

## Unknowns and limits

1. **Angle table.** `0x8032C940` is named by use (signed halfword angles in
   1/256-turn units). `0x8032D050` is the same array plus `0x388` entries, so
   the two builders index one table. The values and the clamp
   (`< -0x387` → 0, `< 0x3C` → table, else `0xFF64` = -156) are literal; the
   unit name is inferred from the surrounding rotation code.
2. **Divide-by-96.** The Ghidra sequence `x / 0x60 + (x >> 31)` followed by
   the opposite correction is modelled as Rust's truncating `/`. A brute
   force over all `u16 - u16` differences confirms the two are identical for
   this input range.
3. **Timing float width.** `0x80193220` reads `0x802E55A4` by address. The
   DOL stores `78643200`, `1000` and `78642000` as `f32` at
   `0x802F5F48`, `0x802F5F4C` and `0x802F5F50`; the Rust body uses `f32`
   intermediates and casts to `f64` for the helper call. A bit-exact result
   for the helper's role (not recovered) is not claimed. The `f32 → i32`
   casts use Rust's saturating `as`; the original `fctiwz` result is
   undefined outside the `i32` range.
4. **`0x801942C0` hidden operands.** The Ghidra entry prints three parameters
   but the object and stream arrive through the `0x802DCD6C` save helper. The
   Rust signature restores `(object, stream, count)` from the caller
   `0x801943E0` and the helper. The four `psq_l` argument loads at the top of
   the body are not referenced again; they are not modelled.
5. **Deferred bodies.** `0x80180780` (4,484 bytes) is the pane animation
   transform constructor that `0x801846A0`/`0x80184C60`/`0x801860B0` call.
   Its control flow is complete in the export, but it is a multi-loop
   builder with three unrolled copy sections and is deferred by size, as wave
   23 deferred the two builders completed here. `0x80185870` and `0x80185F10`
   remain paired-single/GQR bodies.
6. **Non-layout neighbour.** `0x80194DA0` is inside the numerical band but is
   audio teardown: it calls `0x8019B4F0`, `0x8019DEC0`, `0x80260860` and
   `0x802610C0` with the audio small-data globals. It is not a `lyt` body.
7. **Host safety.** All five bodies touch target memory through
   `crate::platform`; running them on a host requires the platform helpers to
   be backed by an emulator or a mock.

## Verification

```text
cargo check
# Finished `dev` profile [unoptimized + debuginfo] target(s)
# no diagnostic named src/ui/lyt.rs

node tools/doc-lint.mjs src
# {"ok": true, "files": 68, "violations": 0, "byRule": {}}
```

This pass touched only `src/ui/lyt.rs`, this document and
`ledger/agents/wave30-lyt.csv`. `src/lib.rs`, every `mod.rs`, `Cargo.toml`,
`src/platform.rs`, the generated ledger index and all other lanes' files were
left untouched. No staging module was created.
