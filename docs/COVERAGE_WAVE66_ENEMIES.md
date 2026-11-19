# Coverage - Wave 66 (enemies: `0x80B14680..0x80B15A50`, `0x80B1C340..0x80B1CA50`)

## How to read this file

This page records the Wave 66 `game::enemies` lane: two clusters in the
`0x80B10000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs`. Every row corresponds to one real `pub unsafe fn` body in
that file, and the `evidence` column names the Ghidra C export that body was
reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 29 | 2,764 | 13 | 10 | 6 |

## What this pass covers

The bucket `0x80B10000..0x80B1FFFF` holds 211 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`, 56 of which were already in
`ledger/functions.csv` when this lane started; none of those addresses is
repeated. Two contiguous runs were taken.

- **The `obj_door_torideboss` run (`0x80B14680..0x80B15A50`, 1,716 bytes).**
  The class name is established by [`fn_80b15490`], which looks up the clip
  string `s_obj_door_torideboss` at `0x80B6989C` and binds it to the `+0x584`
  animation object. The run contains four 48-byte deleting-destructor
  forwarders, the `0x5E0`-byte constructor [`fn_80b15330`] (class vtable
  `0x80B698B0`), the `g3d/test_lift.brres` loader [`fn_80b159a0`] (file
  `0x80B69B88`, member `0x80B69B74`), the `0x291`/`0x28F` sound requests, the
  `0x8007FA40` animator installer, the packed `0x802EE810` rotation record in
  [`fn_80b14db0`], the two vtable name predicates, the position snapshot
  [`fn_80b14c70`] and the full destructor [`fn_80b157d0`].
- **The second `0x80B1C000` actor family (`0x80B1C340..0x80B1CA50`,
  1,048 bytes).** Its two registration/dispose callbacks, the constant producer
  `0x80357BF4`, the five-link `+0x30` chain walker, the `:`-suffix name
  predicate, two deleting-destructor forwarders, the `0x528`-byte constructor
  with vtable `0x80B6B310`, the two release paths and the two destructor copies
  that free the `+0x18` member before the optional allocation release.

The two runs are separated by the untranslated `0x80B15C60..0x80B1B8F0` block
and by `0x80B147D0`, both described under *Deliberate skips*.

### Identity evidence

`0x80B698B0` is the door actor's class vtable, `0x80B330E0` its `+0x584`
sub-object vtable, `0x80B6B310` the `0x528`-byte family's vtable and
`0x80357BF4` the constant returned by [`fn_80b1c400`]. All four are literal in
the exports. The `0x528`-byte family's class name is not printed anywhere in
this run, so it is described by size and offsets only.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80B14680` | `fn_80b14680` | 48 | low |
| `0x80B146B0` | `fn_80b146b0` | 48 | low |
| `0x80B14C70` | `fn_80b14c70` | 100 | medium |
| `0x80B14DB0` | `fn_80b14db0` | 252 | high |
| `0x80B14F70` | `fn_80b14f70` | 88 | medium |
| `0x80B152D0` | `fn_80b152d0` | 48 | low |
| `0x80B15300` | `fn_80b15300` | 48 | low |
| `0x80B15330` | `fn_80b15330` | 156 | high |
| `0x80B15490` | `fn_80b15490` | 120 | medium |
| `0x80B155F0` | `fn_80b155f0` | 64 | medium |
| `0x80B15630` | `fn_80b15630` | 76 | medium |
| `0x80B156E0` | `fn_80b156e0` | 80 | high |
| `0x80B15730` | `fn_80b15730` | 80 | high |
| `0x80B15780` | `fn_80b15780` | 80 | medium |
| `0x80B157D0` | `fn_80b157d0` | 136 | high |
| `0x80B159A0` | `fn_80b159a0` | 172 | high |
| `0x80B15A50` | `fn_80b15a50` | 120 | medium |
| `0x80B1C340` | `fn_80b1c340` | 88 | high |
| `0x80B1C3A0` | `fn_80b1c3a0` | 92 | high |
| `0x80B1C400` | `fn_80b1c400` | 12 | high |
| `0x80B1C4F0` | `fn_80b1c4f0` | 224 | medium |
| `0x80B1C5D0` | `fn_80b1c5d0` | 136 | medium |
| `0x80B1C690` | `fn_80b1c690` | 48 | low |
| `0x80B1C6C0` | `fn_80b1c6c0` | 48 | low |
| `0x80B1C6F0` | `fn_80b1c6f0` | 76 | high |
| `0x80B1C970` | `fn_80b1c970` | 64 | medium |
| `0x80B1C9B0` | `fn_80b1c9b0` | 64 | medium |
| `0x80B1C9F0` | `fn_80b1c9f0` | 96 | high |
| `0x80B1CA50` | `fn_80b1ca50` | 100 | high |

## Deliberate skips

- **`0x80B14D80` (28 bytes).** The export is
  `/* WARNING: Control flow encountered bad instruction data */ ... halt_baddata();`
  with the whole body truncated. There is no instruction sequence to translate,
  so no honest body exists.
- **`0x80B147D0` (1,184 bytes).** The spawn/create function of the
  `0x80B14000` actor: it derives the actor from `0x802DCD74`, writes the
  `0x802EE7A0..0x802EE830` constant records word by word, and switches on
  `flags & 3` into four variant branches that each install a different state
  descriptor (`0x80353CA8`, `0x80B78E78`, `0x803539E4`) and a different
  animation-rate word. It is readable and could be translated, but at roughly
  150 Rust lines it would consume a third of this lane's budget for one row; it
  is left for a later pass rather than rushed.
- **`0x80B10110..0x80B12470` (25 functions) and
  `0x80B15C60..0x80B1B8F0`.** Readable, but outside the two clusters chosen
  for this pass. They are the next natural continuation of this bucket.
- **Six 48-byte forwarders.** All six reproduce the literal
  `(second, second, first)` call to `0x802DCBFC`. The target's role is not
  established in this module, so those rows are `low`, not `high`.
- **`0x80B1C970`/`0x80B1C9B0`.** The export prints `0x802B9280()` with no
  arguments. The receiver is inferred from the calling convention (`r3` still
  holds `state`), so those two rows are `medium`.
- **Double-width constants.** `0x80B321DC`, `0x80B321E4` and the three
  `0x80B322xx` words are printed as `(double)` casts. The bodies read them with
  `read_f64`; the width is inferred from the cast, which is why the five bodies
  that use them are `medium`.
- **`0x80B14F70`/`0x80B15780`.** The two vtable name predicates are literal,
  but the slot roles (`+0x28` owner accessor, `+0x14` comparison) are inferred
  from the sibling `:`-suffix predicate, so they stay `medium`.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 29 sizes sum to
  2,764 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave66` reports `OK` with
  `wave66-enemies.csv: 29 rows`.
