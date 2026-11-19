# Coverage - Wave 92 (`game::stage`, the `Wm_jr`/`Wm_clown` `0x8009xxxx` band)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The evidence
column of `ledger/agents/wave92-stage.csv` points at the Ghidra C export each
body was written from. Roles the export only hints at are marked `inferred` or
`unknown` in the body, never here.

This pass continues the `0x8009xxxx` actor band that wave 69 opened and wave 84
extended. The wave-69 field vocabulary is reused throughout:

* **`+0x30` parent link** and the `+0x0C` "has child" vtable slot shared by the
  two chain walkers.
* **`+0x60` vtable** with the `+0x6C` kind-byte query and the
  `+0x3E8`/`+0x3EC`/`+0x3F4` model slots.
* **`+0x348` variant byte**, **`+0x38D` player slot**, **`+0x390`/`+0x391` mode
  bytes**, **`+0x414..+0x432` variant record**.
* The `+0xAC`/`+0xB0`/`+0xB4` position triple and the `r2` constants named in
  wave 69 (`W69_F_SAMPLE_Y` at `SDA2_BASE - 0x63F4`, `W69_G_NOTICE_OWNER` at
  `SDA_BASE - 0x5630`) and wave 48 (`W48_G_PLAYER_STATS`).

This pass adds four channels that belong to the `Wm_jr` and `Wm_clown`
subclasses:

* **`+0xA08` / `+0xB1C` / `+0x7B8`** - the three effect sub-objects whose `+0xA0`
  (six-argument) and `+0xB0` (four-argument) slots submit the spin, jump and
  break effects.
* **`+0x600` / `+0x604` / `+0x644`** - the clown animation source block, the
  animation object and its mirror object bound by `0x801651F0`.
* **`+0x758..+0x762`** - the six `u16` animation ids of the `Wm_jr` walk.
* **`+0x1080` / `+0x1082`** - the two per-slot match indices read after arming
  slot `8`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 30 | 3636 | 22 | 8 |

No body in this pass is `low` confidence and none is `partial`; every row is
`translated`.

## What this pass covers

- **`Wm_jr` animation ids and the `+8` predicate** (`0x8009BA70`,
  `0x8009BCE0`). Three `0x8015F4F0(dest, sign-extended u16, mask)` binds, and
  the literal bit-fold that turns `(u16_at_8 + 0xFECF)` into `0`/`1`.
- **Spin finish pair** (`0x8009C350`, `0x8009C530`). Same body, different final
  state (`2` vs `0`) handed to `0x800A4C40`: the `+0x148` refresh, the
  `+0x60/+0x6C` slot byte, the `slot < 4` guard around `0x800E20A0` and
  `0x800CEA20`, the `+0x7B0` gate, and the `+0x120` bit-3 clear.
- **Spin and smoke submissions** (`0x8009C650..0x8009CA30`). Two spin effects
  through the `+0xA0` and `+0xB0` variants of the same two channels, the
  `Wm_jr_landsmoke` submit, and the jump/break smoke pair with their
  `+0x794`/`+0x798` point and `+0xB4` height offset.
- **Clown actions** (`0x8009D010..0x8009D540`). The damage-down, squat and fire
  animations: `0x8023A0B0` name entry, `0x801651F0` bind, the two rate calls
  `0x80169F80`/`0x801680E0`, the `0x8009BA50` id reset, and the fire action's
  `0x4EF` motion publish plus `0x8009BD10` side selection.
- **Release and chain helpers** (`0x8009E6D0..0x8009ED10`). The
  `0x8015F800`/`0x802B9280` release pair, the two five-link parent walkers, the
  colon-cut path comparison, the two `0x802DCBFC` forwarders, the seven-word
  `+0x414..+0x432` record publisher and the `+0x391` mode re-selection.
- **`+0x1080` model-state family** (`0x8009F250..0x8009F770`). The second
  release pair, the second parent walker and path comparison, the two slot-`8`
  index readers, the two kinds of the `0x802F0514`/`0x802F0538` table lookup,
  and the two-float notice dispatch.

The public entry points of this pass are [`fn_8009c350`] (spin finish),
[`fn_8009d540`] (clown fire), [`fn_8009e830`] (first chain walker) and
[`fn_8009f5f0`] (match index used by [`fn_800967f0`]).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8009BA70` | `fn_8009ba70` | 88 | high |
| `0x8009BCE0` | `fn_8009bce0` | 44 | high |
| `0x8009C350` | `fn_8009c350` | 252 | medium |
| `0x8009C530` | `fn_8009c530` | 252 | medium |
| `0x8009C650` | `fn_8009c650` | 188 | medium |
| `0x8009C710` | `fn_8009c710` | 164 | medium |
| `0x8009C810` | `fn_8009c810` | 80 | high |
| `0x8009C860` | `fn_8009c860` | 100 | high |
| `0x8009CA30` | `fn_8009ca30` | 140 | medium |
| `0x8009D010` | `fn_8009d010` | 124 | medium |
| `0x8009D450` | `fn_8009d450` | 116 | high |
| `0x8009D540` | `fn_8009d540` | 176 | high |
| `0x8009E6D0` | `fn_8009e6d0` | 92 | high |
| `0x8009E830` | `fn_8009e830` | 224 | high |
| `0x8009E910` | `fn_8009e910` | 136 | high |
| `0x8009E9D0` | `fn_8009e9d0` | 48 | high |
| `0x8009EA00` | `fn_8009ea00` | 48 | high |
| `0x8009EC30` | `fn_8009ec30` | 224 | high |
| `0x8009ED10` | `fn_8009ed10` | 148 | medium |
| `0x8009F250` | `fn_8009f250` | 88 | high |
| `0x8009F2B0` | `fn_8009f2b0` | 92 | high |
| `0x8009F3F0` | `fn_8009f3f0` | 224 | high |
| `0x8009F4D0` | `fn_8009f4d0` | 136 | high |
| `0x8009F590` | `fn_8009f590` | 48 | high |
| `0x8009F5C0` | `fn_8009f5c0` | 48 | high |
| `0x8009F5F0` | `fn_8009f5f0` | 56 | high |
| `0x8009F630` | `fn_8009f630` | 84 | high |
| `0x8009F690` | `fn_8009f690` | 56 | high |
| `0x8009F710` | `fn_8009f710` | 96 | high |
| `0x8009F770` | `fn_8009f770` | 64 | medium |

Address range: `0x8009BA70..0x8009F770` (five runs, 30 bodies, 3636 bytes).

## Deliberately skipped

The bucket still holds 52 untranslated functions after this pass. Skipped on
purpose:

- **Twelve-byte type-info / descriptor stubs** (`0x8009ACA0..0x8009AD20`,
  `0x8009E730`, `0x8009E740`, `0x8009EAF0`, `0x8009EB00` and
  `0x8009F7C0..0x8009F830`). Every one is the two-instruction
  `return &DAT_80377A04;` (the two `0x8009E7xx` entries return
  `&DAT_803576F4`/`&DAT_803576B4`). They duplicate the seven stubs wave 84
  already translated and the module-header policy keeps single-accessor stubs
  as address calls.
- **Paired-single band** (`0x800955F0`, `0x800957B0`). Both build their floats
  through `unaff_GQR0`/`unaff_GQR3` blocks with `ldexpf` rounding, the same
  limit wave 91 documented.
- **Large passes not attempted in this pass** (`0x80093530`, `0x80093A50`,
  `0x80093F10`, `0x8009A320`, `0x8009B070`, `0x8009DD60`). Each is a
  0x400-byte-plus effect/actor pass that needs its own review.
- **Smaller helpers left for a later pass** (`0x80090440`, `0x800925F0`,
  `0x800963B0`, `0x800964A0`, `0x80096780`, `0x80096790`, `0x80097170`,
  `0x80097250`, `0x800976B0`, `0x800978C0`, `0x80097A20`, `0x80098900`,
  `0x80099F80`, `0x80099FA0`, `0x80099FE0`, `0x8009A000`, `0x8009AEC0`,
  `0x8009AF40`, `0x8009B540`, `0x8009BAD0`, `0x8009F6D0`, `0x8009F6F0`,
  `0x80096700`). `0x80096700` is the four-byte thunk to the already translated
  `0x800967F0`; `0x80096780`/`0x80096790` are the two 16-byte `+0x24C`/`+0x250`
  vtable forwarders; `0x8009F6F0` is the `8 < param_2` clamp whose first
  argument the export never reads, so its table access cannot be reconstructed
  honestly today.

None of these is counted as translated in the CSV.

## Verification

- Every address and byte size matches `functions.tsv` (30 rows, 3636 bytes;
  the validator re-checks each `# Original` size against the inventory).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`; the 30
  addresses were chosen from the 82 open entries of the `0x80090000` bucket.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs`.
- `node tools/check-lane-csv.mjs wave92` prints `OK` and the wave-92 stage rows
  are clean.
