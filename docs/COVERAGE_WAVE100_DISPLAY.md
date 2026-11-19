# Coverage - Wave 100 (engine::display)

## How to read this file

`src/engine/display.rs` is the semantic home of the engine-level 2-D
presentation layer. This pass adds 25 bodies from the `0x80150000` bucket, the
last translation pass of that bucket. "Bodies" counts `pub unsafe fn`
functions; "bytes" is the sum of the Ghidra sizes recorded in
`work/ghidra/main-gb/export/functions.tsv`.

"high" means every store, branch and call is visible in the export. "medium"
means the export keeps an `unaff_r2` small-data operand whose value is not
recovered: the operand is read by address and its role is marked `// unknown:`
or `// inferred:`. Every field whose role is not proven stays commented in the
Rust body.

Only `src/engine/display.rs` was edited. `ledger/functions.csv` is read-only and
the coordinator merges this lane's rows.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/display.rs` | `engine::display` | 25 | 11540 | 23 | 2 |

## What this pass covers

The bucket entered this pass with 42 untranslated of its 209 functions and 17
remain. The slice is organized around four registration tables and their
callbacks, all in the `0x80151630..0x8015F040` range.

- **The `daYoshi_c` state bank** (`0x80153400`, 3832 bytes, plus eleven 12-byte
  getters). [`fn_80153400`] installs nineteen state descriptors from the
  0x24-byte handler records at `0x80326D08..0x80326FB4`. The four outer states
  and the four `Eat*` states use vtable `0x803271F0` and the release callback
  `0x80154300`; the eleven riding/physics states from `Walk` to `Cloud` use
  vtable `0x803271BC`, the release callback `0x80154360` and a tenth word at
  `+0x30` produced by the `0x801543C0..0x80154460` getters.
- **The `Pausewindow_c` registration** (`0x8015B320`, 1276 bytes). Copied from
  the same shape the file already described in its header: four globals seeded
  from `r2 - 0x54A0..0x5498` and `r2 - 0x6114`, the null sentinel registration,
  the `0x8042A338` callback slot, then the six 0x30-byte descriptors
  `InitWait` through `CloseAni` with vtable `0x80328058` and the release
  callback `0x8015B840`.
- **The `dScBoot_c` state bank** (`0x8015DDD0`, 4716 bytes). Registers the
  `d_enemies` and `d_en_boss` actor lists, copies the two SDA word tables at
  `r13 - 0x5338` and `r13 - 0x5328`, then installs twenty-four 0x30-byte boot
  descriptors from `ResetWait` to `ErrorInfinity` with vtable `0x803289E4` and
  the release callback [`fn_8015f040`].
- **Player attributes and route reset** (`0x80151630..0x80154740`, 2 bodies).
  [`fn_80151630`] rebinds the twelve-word selection record into
  `+0x1094..+0x10C0` and ORs capability bits; [`fn_80154740`] resets the route
  panes, both pad masks, the four HUD sub-blocks and the state words.
- **The release helpers and thunks** (`0x801533D0..0x8015F040`, 8 bodies).
  [`fn_80154360`] and [`fn_8015f040`] are the same "destructor then optional
  free" body as the already-translated [`fn_80154300`];
  [`fn_801578f0`] releases an embedded state manager; the six remaining
  `0x802DCBFC` thunks forward `(second, second, first)`.

The registration helpers `w100_install_yoshi_state` and
`w100_install_pause_state` are host-side factoring of the repeated block; the
`dScBoot_c` bank and the yoshi field reads reuse the wave-93
`w93_register_state` and wave-99 `w99_descriptor_fields` helpers, which already
implement the observed permuted copy order
`+0x18, +0x1C, +0x20, +0x0C, +0x10, +0x14, +0x00, +0x04, +0x08`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80151630` | `fn_80151630` | 668 | medium |
| `0x801533D0` | `fn_801533d0` | 48 | high |
| `0x80153400` | `fn_80153400` | 3832 | high |
| `0x80154360` | `fn_80154360` | 92 | high |
| `0x801543C0` | `fn_801543c0` | 12 | high |
| `0x801543D0` | `fn_801543d0` | 12 | high |
| `0x801543E0` | `fn_801543e0` | 12 | high |
| `0x801543F0` | `fn_801543f0` | 12 | high |
| `0x80154400` | `fn_80154400` | 12 | high |
| `0x80154410` | `fn_80154410` | 12 | high |
| `0x80154420` | `fn_80154420` | 12 | high |
| `0x80154430` | `fn_80154430` | 12 | high |
| `0x80154440` | `fn_80154440` | 12 | high |
| `0x80154450` | `fn_80154450` | 12 | high |
| `0x80154460` | `fn_80154460` | 12 | high |
| `0x80154740` | `fn_80154740` | 348 | medium |
| `0x801578F0` | `fn_801578f0` | 100 | high |
| `0x80159EB0` | `fn_80159eb0` | 48 | high |
| `0x80159EE0` | `fn_80159ee0` | 48 | high |
| `0x8015B2C0` | `fn_8015b2c0` | 48 | high |
| `0x8015B2F0` | `fn_8015b2f0` | 48 | high |
| `0x8015B320` | `fn_8015b320` | 1276 | high |
| `0x8015DDA0` | `fn_8015dda0` | 48 | high |
| `0x8015DDD0` | `fn_8015ddd0` | 4716 | high |
| `0x8015F040` | `fn_8015f040` | 88 | high |

Byte total: `3832 + 1276 + 4716` (registration banks) + `668 + 348` (player and
route) + `92 + 100 + 88` (release helpers) + `528` (six thunks) + `132`
(eleven getters) = `11540`.

## Deliberate skips

17 addresses stay untranslated. Every skip is a concrete export defect, not a
budget cut.

**Paired-single / GQR quantisation (5 addresses).** `0x801502E0`,
`0x80151A10`, `0x80154E50`, `0x80156930` and `0x80156AB0` are printed with
`unaff_GQR0`, `ldexpf` and `CONCAT` stubs around their float work, and
`0x801502E0` additionally carries `WARNING: Removing unreachable block` lines.
No honest body can be written without inventing the paired-single arithmetic.

**Dropped receiver registers (6 addresses).** `0x80151E50` and `0x80152CF0`
take their receiver from the `0x802DCD74` pseudo-return and `0x80152CF0` adds
an `extraout_f1` operand; `0x801548A0` and `0x80154A70` read `unaff_r13`
throughout plus six GQR stubs; `0x80156C30` and `0x80159880` take their
receiver from `0x802DCD70` and read `unaff_r2`. The real `r3/r4` split is not
visible, so a signature would be invented.

**Four-byte thunks whose export holds the target (5 addresses).**
`0x8015F130`, `0x8015F3B0`, `0x8015F410`, `0x8015F470` and `0x8015F7B0` are
four-byte jumps, but each evidence file contains the body of the jump target
(`0x801B8620`, `0x8015F3C0`, `0x8015F420`, `0x8015F480`, `0x8015F6E0`).
Transcribing those bodies at the thunk address would document the wrong
function, and `0x8015F470` also loses its SDA2 scale to `unaff_r2`.

**Variadic GPR-save bridge (1 address).** `0x8015F730` selects its eight saved
argument registers from `in_cr1` bit 1 and forwards a stack-built argument
block to `0x8015F6E0`. Neither the condition bit nor the stack frame is
expressible from the export, so the body would be a guess.

## Verification

- Every address and byte size matches `functions.tsv`, re-read for this pass;
  the 25 `# Original` blocks total 11,540 bytes.
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/engine/display.rs`.
- No address was already in `ledger/functions.csv`; the bucket had 42
  untranslated functions before this pass and 17 after it.
- `node tools/check-lane-csv.mjs wave100` reports `OK` for this lane: 25 rows,
  25 unique addresses.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/engine/display.rs`.
