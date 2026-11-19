# Coverage - Wave 153 (`game::enemies`)

## How to read this file

This page records the Wave 153 `game::enemies` lane: 19 functions of REL module
3, `d_enemiesNP`, appended to `src/game/enemies.rs` as section 135 and mirrored
one row per body in `ledger/agents/wave153-enemies.csv`.

Every `address` is a **synthetic analysis address** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Raw words
were decoded from `work/rels/elf/d_enemiesNP.elf` through the `PT_LOAD` headers
as described in `docs/REL_ELF_MAPPING.md`
(`file_offset = address - 0x80A00000 + 0x1000` for the first segment), and every
body was followed from its entry to its terminal `blr` or `bcctr`. Byte counts
are the `size` column of
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 19 | 3864 | 9 | 10 |

Status split: 19 `translated` (9 high, 10 medium), no `partial` and no `stub`.

Covered address range: `0x80A5E1A0..0x80B27DC0` (the last body's `blr` sits at
`0x80B27ECC`). 3,264 bytes are `docs/SPEC_WAVE152_ENEMIES.md` rows (13 bodies:
the twelve specification rows plus the untaken sibling `0x80A5E1A0` that spec
row 8 needs), and the other 600 bytes are the six `0x80AB0000` planner-bucket
rows below the specification (`0x80AB12E0`, the seventh bucket body, is counted
with the specification rows).

## What this pass covers

- **The ten "ready" rows of `docs/SPEC_WAVE152_ENEMIES.md`:**
  - `0x80AC1800` - `daEnMechaKoopa_c` clip binder: `walk`/`damage`/`sign`/
    `recover` through the four-entry table `0x80B30210`, the chr chain into
    `+0x588` and the tex-pat chain into `+0x5C0`, both over `+0x548`.
  - `0x80B16A80` - `daEnUnizoo_c` eye/mouth offset writer: `strcmp` against
    `eye`/`mouth`, x taking `0.25f + [+0x5F4]` and y `[+0x5F0]`.
  - `0x80B27DC0` - per-slot angle -> speed factor: the `s16` absolute value,
    the `0x4000` fold, the two curves and the `+0x8B8` sign halfword.
  - `0x80A80F40` - `daEnIgaKuribo_c` swing step: the 208..768 amplitude ramp,
    the single `qr3` widen and the `1280.0 * sin` truncation into `+0x100`.
  - `0x80B1AD30` - `daEnWalkPakkun_c` ground snap plus the `0x800A0A30` ride
    completion.
  - `0x80AF5C60` - `daEnRotBlock_c` guide-angle placement: the quarter-turn
    bias, both `qr3` widens against the same stack halfword, and the
    pivot-plus-radius layout.
  - `0x80A7ACA0` - two-table ground probe over `0x80B2E0D8` and `0x80B2E100`.
  - `0x80AE0780` - per-player `0x3C`/`0x87` effect burst with the
    `0x80429CA8` mask and the `0x53` flag.
  - `0x80AB12E0` - per-section texcoord/colour override with the low-byte
    `mode` patch.
  - `0x80AC0750` - launch-angle solver: the 32-unit sphere clamp, the
    reciprocal square root and the `(|dx|, dy)` angle call.
- **Both "conditional" rows:**
  - `0x80A965D0` - platform per-player ride registration. The body is literal;
    only the two callback-record types are unnamed, so it is `medium`.
  - `0x80A5E590` - `daEnGabon_c` wait entry, shipped together with its
    untaken sibling `0x80A5E1A0` (136 bytes), which had been skipped by waves
    20, 78, 140 and 147.
- **All seven remaining `0x80AB0000` planner-bucket bodies** (the bucket is now
  complete; `0x80AB12E0` is listed above as a specification row):
  - `0x80AB0870` - per-mode holder release on top of [`fn_80ab0ee0`] and
    [`fn_80ab1160`].
  - `0x80AB1780` - three-clip binder (`swim`, `attack_st`, `attack`) into
    `+0x704`.
  - `0x80ABA5B0`, `0x80ABA5F0` - two byte-identical sized free helpers.
  - `0x80ABB1B0`, `0x80ABB1E0` - two `0x802DCBFC` deleting-destructor
    forwarders with hidden this-adjusts of `+0x18` and `+0x0C`.

### `qr3` sites

| address | sites | port |
|---|---|---|
| `0x80A80F40` | `0x80A80FD8` `psq_l f1,0(r4),3,1`, `r4 = r1 + 8` | the phase halfword is widened as `(f32)(u16)` and scaled by `0x80B2E320 = 1/256` |
| `0x80AF5C60` | `0x80AF5CB8` (`r4 = r1 + 10`) and `0x80AF5CD8` (`r3 = r1 + 8`) | the same quarter-turn halfword is widened twice, once for `0x80237C40` and once for `0x80237BD0`, both at `1/256` (`0x80B31460`) |

`GQR3` is `0x00050005` per `docs/REL_ELF_MAPPING.md` (type 5 = unsigned 16-bit,
scale 0, `W = 1`), so each load is exactly `(f32)(u16)halfword`. Every other
`psq_l`/`psq_st` in the 19 rows is a `qr0` `f31`/`f30` stack spill.

### Specification corrections made while porting

- **`0x80A5E1A0` uses `+0x544`, not `+0x548`, as the shared holder.** The raw
  stream at `0x80A5E1DC` is `addi r4,r30,1348` (that is `+0x544`), while
  `0x80A5E1C8` is `addi r3,r3,1344` (`+0x540`) for the `0x8023A0B0` bank. The
  spec's pseudo-code prints `this+0x548` for the shared holder; the port
  follows the stream. The binder's frame constant is the `0.0f` at
  `0x80B2D198`, so the export's `DAT_80b2d198` is correct.
- **`0x80A7AB30` is a `void` function; the band value is its leftover `r3`.**
  The raw body stores the `0x80A7A860` offset beside the height, calls
  `0x80A7A8C0(this, pair)` and, when that answers `< 1`, `0x80A7AA00(this,
  pair)`. `0x80A7ACA0` immediately reads `r3 + 1` after the call, so the band
  is the tail call's return register. The port keeps that ABI
  (`w153_row_probe` returns `i32`), and the earlier
  `fn_80a7ab30(f64, u32)` translation is register-compatible with it
  (`f1 = height`, `r3 = this`).
- **`0x80AB1780`'s frame constant is `1.0f`.** Ghidra prints the third argument
  of `0x801680E0` as `(double)DAT_80b2faf0`; the word at `0x80B2FAF0` is
  `0x3F800000`, not `0.0f`. Its clip table `0x80B54C90` holds three pointers
  (`swim`, `attack_st`, `attack`) and the loop table `0x80B2FB20` is three
  zero words.
- **`0x80ABB1B0`/`0x80ABB1E0` carry a hidden `r12`.** The raw stream sets
  `r12 = param_1 + 0x18` (respectively `+0x0C`) before `0x802DCBFC`, and stages
  `r3 = r4`, `r4 = r4`, `r5 = r3`. The spec's two-field export hides that
  operand; the port documents it and cannot pass it, exactly like the four
  wave-152 forwarders.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A5E1A0` | `fn_80a5e1a0` | 136 | high |
| `0x80A5E590` | `fn_80a5e590` | 252 | medium |
| `0x80A7ACA0` | `fn_80a7aca0` | 252 | medium |
| `0x80A80F40` | `fn_80a80f40` | 264 | high |
| `0x80A965D0` | `fn_80a965d0` | 280 | medium |
| `0x80AB0870` | `fn_80ab0870` | 204 | medium |
| `0x80AB12E0` | `fn_80ab12e0` | 244 | high |
| `0x80AB1780` | `fn_80ab1780` | 172 | medium |
| `0x80ABA5B0` | `fn_80aba5b0` | 64 | medium |
| `0x80ABA5F0` | `fn_80aba5f0` | 64 | medium |
| `0x80ABB1B0` | `fn_80abb1b0` | 48 | medium |
| `0x80ABB1E0` | `fn_80abb1e0` | 48 | medium |
| `0x80AC0750` | `fn_80ac0750` | 240 | high |
| `0x80AC1800` | `fn_80ac1800` | 276 | high |
| `0x80AE0780` | `fn_80ae0780` | 248 | medium |
| `0x80AF5C60` | `fn_80af5c60` | 260 | high |
| `0x80B16A80` | `fn_80b16a80` | 276 | high |
| `0x80B1AD30` | `fn_80b1ad30` | 264 | high |
| `0x80B27DC0` | `fn_80b27dc0` | 272 | high |

### Why ten rows are `medium`

- **`0x80A965D0`** - every guard, array index and store is literal; the two
  callback-record types that supply `rec_a`/`rec_b` are not established by this
  module (the body has no `bl` caller, only the data word at `0x80B2ED30`).
- **`0x80A5E590`** - the four call shapes and the three timers are literal; the
  component returned by controller slot `+0x2C` and the BSS id `0x80358004` are
  unnamed.
- **`0x80A7ACA0`** - the tables and branches are literal; the band is a
  leftover register (see the corrections above).
- **`0x80AB0870`** - the mode branches and four slot `+0x14` dispatches are
  literal; the holder roles come from the sibling translations.
- **`0x80AB1780`** - names, loop flags, the `1.0f` frame and the `+0x98C` store
  are literal; the clip roles are inferred from the actor family.
- **the two free helpers** - the guards, the call and the returned register are
  literal; the identity of `0x802B9280` as the allocator's free is inferred.
- **the two forwarders** - the visible call and the this-adjust constant are
  literal, but the hidden `r12` operand cannot be modelled in Rust.

## Deliberate skips

- None of the claimed addresses was skipped; all 12 specification rows and all
  seven `0x80AB0000` bucket rows produced a body.
- Still out of scope and untouched: `0x80A10880` (the 940-byte
  `0x80A10000` bucket row that waves 151 and 152 deliberately skipped) and every
  other still-untranslated address outside this lane's claims. They were not
  attempted rather than guessed.
- `0x80A5DBC0` (212 bytes) was a neighbouring bucket row, not part of the
  specification's twelve and not in the `0x80AB0000` bucket, so it is left for a
  later pass.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 19 sizes sum to 3,864
  bytes (3,264 specification rows, 600 bucket rows).
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; none of the 19 addresses was in
  `ledger/functions.csv` and none had a body in `src/**/*.rs` before this pass.
- Every `# Original` block carries the `Synthetic base: 0x80A00000` line.
- The three `qr3` sites are ported as `(f32)(u16)`; every other `psq` in these
  rows is a `qr0` spill.
- The module header was updated: cluster item 135, the cluster count (134 to
  135), the function totals (3486 to 3505 public translations, 3415 to 3434
  `fn_<addr>` bodies), the wave list, the "start here" list and the records
  list.
- `cargo check --message-format=short` reports no error or warning naming
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave153` reports 19 rows for this lane, 19
  unique addresses and `OK`.
