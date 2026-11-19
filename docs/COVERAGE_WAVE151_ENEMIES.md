# Coverage - Wave 151 (`game::enemies`)

## How to read this file

This page records the Wave 151 `game::enemies` lane: 17 functions of REL module 3,
`d_enemiesNP`, appended to `src/game/enemies.rs` as section 133 and mirrored one
row per body in `ledger/agents/wave151-enemies.csv`.

Every `address` is a **synthetic analysis address** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none of
these is a runtime address; every function doc repeats that note. Raw words were
decoded from `work/rels/elf/d_enemiesNP.elf` through the `PT_LOAD` headers as
described in `docs/REL_ELF_MAPPING.md`
(`file_offset = address - 0x80A00000 + 0x1000` for the first segment), and every
body was followed from its entry to its terminal instruction. Byte counts are the
`size` column of `work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 17 | 4260 | 1 | 16 |

Status split: 17 `translated` (1 high, 16 medium), no `partial` and no `stub`.

Covered address range: `0x80A00D00..0x80B27ED0` (3,488 bytes of
`docs/SPEC_WAVE150_ENEMIES.md` rows and 772 bytes of `0x80A10000`
planner-bucket rows).

## What this pass covers

- **The seven still-open "ready" rows of `docs/SPEC_WAVE150_ENEMIES.md`.** The
  page's other two "ready" rows (`0x80A390F0`, `0x80AEE850`) were already landed
  by wave 150 and are not re-claimed:
  - `0x80A00D00` - `daEnBuku_c` four-player carry: normalise, pull `2.0`, snap
    each marked player to `(x, y + 17.0, z)` when both axis distances fall under
    `3.0`.
  - `0x80A04A90` - predicted-position query turned into a normalised speed triple
    (`+0x670` raw delta, `+0xE8`/`+0xEC` scaled, `+0xF0` forced to `0.0`).
  - `0x80A37CB0` - nearest player on the actor's `+0x348` side, returning the
    signed clip byte behind the player's `+0x6C` virtual or `-1`.
  - `0x80AB13E0` - `fishbone` four-node binder plus the `attack_st` animation
    group on the first iteration.
  - `0x80B0CD80` - `daEnTeresa_c` body binder (`wait` / `shay_teresaA`).
  - `0x80B0CEB0` - Teresa tail binder (`shay_teresaB` / `shay_teresaB_wait`).
- **All three "conditional" rows**, whose instruction streams are fully literal
  and whose only open items were callee/field naming questions:
  - `0x80B16BA0` - node matrix commit from the position plus the three `s16`
    angles, with the `+0x628` animation select. The `0x80168180` output words
    `3`/`7`/`11` are the `3x4` matrix translation, exactly as in
    [`fn_80a00940`](../src/game/enemies.rs); that resolves the page's open item.
  - `0x80AF3400` - player-slot launch with the `15/7/7/7` radius table, the
    `qr3` `(f32)(u16)` angle widen and motion id `1000`.
  - `0x80B27ED0` - per-slot relay with the two position arms and the shared
    `0x80B27DC0` speed tail.
- **Seven `0x80A10000` planner-bucket rows**: the trail-record step
  `0x80A10220` (0x1C-byte records at `+0x92C`, `+0x928` of them, the
  `+0x348 ^ 1` speed lane) and the six `0x802DCBFC` deleting-destructor
  forwarders `0x80A197F0`, `0x80A19820`, `0x80A1DA70`, `0x80A1DAA0`,
  `0x80A1F4B0`, `0x80A1F4E0`. The bucket now has one untranslated address left
  (`0x80A10880`).

### `qr3` sites

Two data `qr3` sites are ported, both as the unsigned-halfword widen pinned by
`docs/REL_ELF_MAPPING.md` (`GQR3 = 0x00050005`, `W = 1`, `I = 3`):

| address | sites | port |
|---|---|---|
| `0x80AF3400` | `0x80AF348C`, `0x80AF34B0` | `psq_l f1,0(r1+0x0A)` / `0x08` re-read the `+0x7A4` `s16`; rendered `(f32)(u16)` times `1/256` before `0x80237BD0`/`0x80237C40` |
| `0x80B27ED0` | `0x80B27FD4`, `0x80B27FF4` | the same widen, but of the **incoming `r4` angle** |

Every other `psq_*` in the 17 rows is a `qr0` `f31`/`f30`/`f29` stack spill.

### Specification corrections made while porting

- **`0x80B27ED0` `qr3` source.** `docs/SPEC_WAVE150_ENEMIES.md` says both loads
  read the `+0x102` heading the body has just written. The raw stream at
  `0x80B27FD0`/`0x80B27FE8` is `sth r28,0x0A(r1)` / `sth r28,0x08(r1)`, where
  `r28` still holds the incoming `r4` angle; the `+0x102` store (`0x80B27F34`)
  goes through `r0` and is never copied to the stack. The port therefore widens
  the incoming `angle`, and the body says so.
- **`0x80B27ED0` rotate arm writes y.** The page's note claims the `+0x8AC == 1`
  arm does not set y. The raw stream at `0x80B27F84..0x80B27F90` loads `+0x73C`,
  `fadds` the second rotated component and stores to `+0xB0`. The port writes all
  three components.
- **`0x80B16BA0` open item closed.** The page left the meaning of `out[0x0C]`,
  `out[0x1C]`, `out[0x2C]` open. `0x80168180` is a `12`-word `3x4` matrix read
  (already used that way by [`fn_80a00940`](../src/game/enemies.rs)), so those
  offsets are exactly the translation column; the row is ported with `medium`
  confidence only for the `0x80064AF0` argument roles.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A00D00` | `fn_80a00d00` | 424 | medium |
| `0x80A04A90` | `fn_80a04a90` | 348 | medium |
| `0x80A10220` | `fn_80a10220` | 484 | medium |
| `0x80A197F0` | `fn_80a197f0` | 48 | medium |
| `0x80A19820` | `fn_80a19820` | 48 | medium |
| `0x80A1DA70` | `fn_80a1da70` | 48 | medium |
| `0x80A1DAA0` | `fn_80a1daa0` | 48 | medium |
| `0x80A1F4B0` | `fn_80a1f4b0` | 48 | medium |
| `0x80A1F4E0` | `fn_80a1f4e0` | 48 | medium |
| `0x80A2EC00` | `fn_80a2ec00` | 472 | medium |
| `0x80A37CB0` | `fn_80a37cb0` | 304 | medium |
| `0x80AB13E0` | `fn_80ab13e0` | 304 | high |
| `0x80AF3400` | `fn_80af3400` | 308 | medium |
| `0x80B0CD80` | `fn_80b0cd80` | 292 | medium |
| `0x80B0CEB0` | `fn_80b0ceb0` | 288 | medium |
| `0x80B16BA0` | `fn_80b16ba0` | 316 | medium |
| `0x80B27ED0` | `fn_80b27ed0` | 432 | medium |

### Why sixteen rows are `medium`

- **`0x80A00D00`** - the loop, the `80170A00` normalise, the `2.0` pull and the
  `3.0` snap window are literal; the `+0x92E` marker table and the `17.0` carry
  height are field-role inferences.
- **`0x80A04A90`** - all loads, stores and calls are literal; the role of the
  untranslated sibling `0x80A04940` and its `512` mode word are inferred.
- **`0x80A10220`** - the loop and both animation calls are literal; the record
  layout is inferred from the `0x1C` stride and its writes.
- **the six forwarders** - the visible `0x802DCBFC(mem, mem, size)` call is
  exact, but the hidden `r12` this-adjust operand cannot be modelled in Rust.
- **`0x80A2EC00`** - both band arms and the two sound ids are literal; the two
  `main.dol` manager fields (`+0x9002C`, `+0x8FEC4`) are inferred roles.
- **`0x80A37CB0`** - the mask, half-plane tests and `1e7` seed are literal; the
  returned byte is a clip/kind index by inference from its caller.
- **`0x80AF3400`** - the radius table, the `qr3` widen and the trig pair are
  literal; `0x8013FAA0`'s contract and the radius-word meaning are inferred.
- **`0x80B0CD80`/`0x80B0CEB0`** - every call and store is literal; which name is
  the base body and which is the alternative is inferred from the call sites.
- **`0x80B16BA0`** - the call order and matrix words are literal and match
  `fn_80a00940`; the `0x80064AF0` argument roles are inferred from that sibling.
- **`0x80B27ED0`** - the raw instruction stream was decoded directly for both
  corrections above; the field roles beyond the position are inferred.
- **`0x80AB13E0`** is the only `high` row: the loop, both constant tables, the
  descriptor words and the five animation calls are literal.

## Deliberate skips

- **`0x80A10880` (940 bytes).** The last untranslated body of the `0x80A10000`
  bucket. It is the trail *frame* interpolation over the records
  `0x80A10220` fills, but three of its operands do not resolve into a consistent
  expression: the `2^52` `u32 -> f64` fold is divided by `0x80B2AD9C` (`29.0`)
  and offset by `0x80B2AE68` (not the `2^52` double the fold needs), and the
  outer loop advances through `piVar15[0x8E]`, a link the export does not type.
  A body written here would be a guess rather than a translation, so it is left
  for a focused pass. The other seven bucket addresses are translated.
- **`0x80B27DC0` (272 bytes)** is *called* by `0x80B27ED0` but not claimed: only
  its ABI (`f32` radius in `f1`, `s16` in `r4`, `f32` result) is pinned here from
  the raw call at `0x80B27FAC`. Its own body is outside this pass.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 17 sizes sum to 4,260
  bytes (3,488 for the ten specification rows, 772 for the seven bucket rows).
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; all 17 addresses were absent from `ledger/functions.csv`
  and from `src/**/*.rs` before this pass.
- Every `# Original` block carries the `Synthetic base: 0x80A00000` line.
- Both data `qr3` sites are ported as `(f32)(u16)`; every other `psq` in these
  rows is a `qr0` spill.
- The module header was updated: cluster item 133, the cluster count (132 to
  133), the function totals (3451 to 3468 public translations, 3380 to 3397
  `fn_<addr>` bodies), the wave list, the "start here" list and the records list.
- `cargo check --message-format=short` reports no error naming
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave151` reports 17 rows for this lane, 17
  unique addresses and `OK`.
