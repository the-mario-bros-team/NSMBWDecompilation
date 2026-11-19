# Coverage - Wave 146 (`game::enemies`)

## How to read this file

This page records the Wave 146 `game::enemies` lane: 17 functions of REL module 3,
`d_enemiesNP`, appended to `src/game/enemies.rs` as section 128 and mirrored one
row per body in `ledger/agents/wave146-enemies.csv`.

Every `address` is a **synthetic analysis address** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none of
these is a runtime address; every function doc repeats that note. Raw words were
decoded from `work/rels/elf/d_enemiesNP.elf` through the `PT_LOAD` headers as
described in `docs/REL_ELF_MAPPING.md` (`file_offset = address - 0x80A00000 +
0x1000` for the first segment only), and every body was followed from its entry
to `blr`. Byte counts are the `size` column of
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 17 | 8436 | 12 | 5 |

Status split: 17 `translated` (12 high, 5 medium), no `partial` and no `stub`.

Covered address range: `0x80A03FF0..0x80B179D0` (6868 bytes of
`docs/SPEC_WAVE145_ENEMIES.md` rows and 1568 bytes of `0x80A30000` planner-bucket
rows).

## What this pass covers

- **Ten of the eleven remaining ready `docs/SPEC_WAVE145_ENEMIES.md` rows.**
  `0x80A96DC0` (the spring-platform bounce phase over the 32-entry
  compression-to-speed table at `0x80B2ED50` and SE `663`/`664`),
  `0x80B179D0` (the `daEnUnizoo_c` four-way yaw select with the `0.125` scale
  decay and the four `0.39/0.78/1.17/1.56` band speeds), `0x80A87030` (the item
  swim bob with the wave's only `qr3` site), `0x80A03FF0` (the `daEnBuku_c`
  four-slot latch and `2.0` anchor pull), `0x80B162C0` (the Unizo move handler
  with three state-descriptor gates and the `3.0` mode-1 scale), `0x80AAD9B0`
  (the `daEnKild_c` twelve-byte segment fan and its three tile commands),
  `0x80AADC50` (the follower integration with the `0.21875` decay and the
  `600`-frame retire), `0x80AF2CF0` (the four-player colour/position marker
  publisher), `0x80A05000` (the `daEnBuku_c` release-marker frame step with SE
  `473`/`474`) and `0x80A78350` (the `daEnHnswich_c` switch/course command).
- **Seven `0x80A30000` planner-bucket rows.** `0x80A35ED0` (the `0x880`-byte
  constructor with the two `0x80310ED8` holders), `0x80A39650` and `0x80A397B0`
  (the brakesmoke warning and respawn steps), `0x80A372A0` (the brakesmoke
  landing step with the `Wm_en_landsmoke` burst and the `+0x5C0` fuse),
  `0x80A365B0` and `0x80A36670` (the two mode dispatchers) and `0x80A35570` (the
  `+0x3F4` contact relay). The bucket's 149 rows now have no untranslated
  `functions.tsv` entry except `0x80A337B0` (the four-byte alias) and
  `0x80A334D0`/`0x80A3A4A0`/`0x80A3DEA0`, which are listed under deliberate
  skips.

### `qr3` sites

One row contains `qr3`: `0x80A870F8  psq_l f1,0(r4),1,qr3` inside
[`fn_80a87030`](../src/game/enemies.rs). The raw word is `0xE024B000`, i.e.
opcode `56`, `fD = 1`, `rA = 4`, `W = 1`, `I = 3`, `dq = 0`; with
`GQR3 = 0x00050005` this is one unsigned halfword load, so the body renders it as
`(f32)(u16)this->+0xDF0` exactly as `docs/REL_ELF_MAPPING.md` prescribes.
Every other `psq_*` in the 17 rows is a `qr0` `f29..f31` stack spill and is not
ported.

### Export corrections made while porting

Three places in the Ghidra exports (and one in the specification page) disagree
with the raw stream, and the raw stream won:

- **`0x80A03FF0` descriptors.** The export names the two vtable-state
  descriptors `DAT_80b6e7b8`/`DAT_80b6e878`; the raw `addis r4,r0,0x80B7` +
  `addi r4,r4,0xE7B8`/`0xE878` pair at `0x80A04244`/`0x80A0426C` makes them
  `0x80B7E7B8`/`0x80B7E878`, matching the specification.
- **`0x80AF2CF0` colour mode.** The specification says `this->+0x14D4`; the raw
  stream loads `lwz r0,0x14D4(r31)` where `r31` is the **player** object
  (`mr r31,r3` at `0x80AF2D5C`). The port reads `player->+0x14D4`, so each player
  slot carries its own colour scheme.
- **`0x80A87030` smoke gate.** The specification writes the gate as
  `+0xE04 == 0 and +0xDF4 == 0`; the export and the already-translated sibling
  `fn_80a873c0` both test `+0xE04 != 0 || +0xDF4 == 0`. The port follows the
  export.
- **`0x80A22820` seed constant.** The specification calls `0x80B2B7D0` "the
  `2^52` double"; the image holds `4503601774854144.0`, i.e. `2^52 + 2^31`, which
  is what the `xoris`-flipped correction word needs.

### Repair to an existing body

`fn_80a03f40` (wave 11, `0x80A03F40`, 168 bytes) used to recover its receiver from
`0x802DCD68`, which is a `_savegpr` helper and returns no object. Both original
call sites are now translated (`0x80A03FF0` and `0x80A05000`) and both pass the
actor in `r3`, so the body now takes `actor: usize` and returns the `1`/`0`
report its callers test. Its evidence path, address, size and ledger row are
unchanged.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A03FF0` | `fn_80a03ff0` | 708 | high |
| `0x80A05000` | `fn_80a05000` | 616 | high |
| `0x80A35570` | `fn_80a35570` | 28 | high |
| `0x80A35ED0` | `fn_80a35ed0` | 228 | medium |
| `0x80A365B0` | `fn_80a365b0` | 192 | medium |
| `0x80A36670` | `fn_80a36670` | 132 | medium |
| `0x80A372A0` | `fn_80a372a0` | 448 | medium |
| `0x80A39650` | `fn_80a39650` | 312 | high |
| `0x80A397B0` | `fn_80a397b0` | 228 | medium |
| `0x80A78350` | `fn_80a78350` | 616 | high |
| `0x80A87030` | `fn_80a87030` | 732 | high |
| `0x80A96DC0` | `fn_80a96dc0` | 848 | high |
| `0x80AAD9B0` | `fn_80aad9b0` | 656 | high |
| `0x80AADC50` | `fn_80aadc50` | 628 | high |
| `0x80AF2CF0` | `fn_80af2cf0` | 624 | high |
| `0x80B162C0` | `fn_80b162c0` | 704 | high |
| `0x80B179D0` | `fn_80b179d0` | 736 | high |

### Why five rows are `medium`

- **`0x80A35ED0`** - the allocator, vtable and every holder call are literal, but
  the class and the two `0x80310ED8` holder tags are inferred from the shared
  constructor layout.
- **`0x80A365B0` / `0x80A36670`** - the arms and the `0x80357BB4` descriptor are
  literal; the forwarded `r4`/`r5` words and the meaning of the queried mode
  values are inferred from the call shapes.
- **`0x80A372A0`** - the flow, the effect name, the SE id and the offsets are
  literal; the three `+0x65C` seeds live in BSS, so the port reads them instead of
  stating a value, and the landing-probe verdict roles are inferred.
- **`0x80A397B0`** - every store and call is literal; the `1.2` radius handed to
  `0x80097170` is a role inference from the sibling respawn steps.

## Deliberate skips

- **`0x80A22820` (700 bytes, spec rank 6, conditional).** The `2^52` frame seed is
  computable now that `0x80B2B7D0` is read as `2^52 + 2^31`, but the two string
  builders around `0x8000F1B0`/`0x8000F0F0` reuse the same six stack slots that
  also hold the `0x80239E30` model pointer in the export (`local_34 = uVar3`
  immediately before `0x8000F1B0(&local_34, 3, &local_38)`). Without a
  disassembly of that fifty-instruction window the argument layout cannot be
  stated without guessing, so the body was left for a later tranche.
- **`0x80A334D0` (352 bytes, bucket).** The body is decodable, but it calls four
  still-untranslated siblings (`0x80A33630`, `0x80A337C0`, `0x80A337E0`,
  `0x80A343B0`) whose argument contracts are not established anywhere in the
  module, and its `+0x65C` seeds are BSS words.
- **`0x80A3A4A0` (2620 bytes, bucket).** The largest remaining bucket row; a
  655-instruction constructor/state block that needs its own pass.
- **`0x80A3DEA0` (2940 bytes, bucket).** Already skipped by wave 145 for the same
  reason: the 16-case jump table at `0x80B3EE94` needs a second full pass.
- **`0x80A337B0` (4 bytes).** Still the `b 0x80A337C0` alias with no body of its
  own.
- **`0x80ACC8F0` (spec rank 8).** Already landed by wave 145; not repeated.

### Observation (pre-existing, not changed)

48 evidence paths in earlier waves' bodies of `src/game/enemies.rs` spell the
export directory `work/ghidra/rel_d_enemiesNP/`, but the directory on disk is
`work/ghidra/rel-d_enemiesNP/`. Those rows are outside this wave's deliverables
and were left untouched; this wave's 17 rows use the hyphenated path.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 17 sizes sum to 8436
  bytes (6868 for the ten specification rows, 1568 for the seven bucket rows).
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; all 17 addresses were absent from
  `ledger/functions.csv` and from every earlier `fn_<addr>` body before this
  pass.
- The only `psq_l ...,qr3` in the new rows is the halfword widen of
  `0x80A87030`; every other `psq` is a `qr0` `f29..f31` spill and is not ported.
- Every float quoted in a section constant was read out of
  `work/rels/elf/d_enemiesNP.elf` at the address shown (`16.0`, `8.0`,
  `-7.0..7.5`, `4.0/3.5/3.0/4.5`, `0.0625/-0.0625`, `0.21875`, `-4.0`,
  `42.0/44.0/1500.0/2.0/64.0`, `0.5/2.0/1.0`, `33.0/37.0/20.0/18.0/41.0`,
  `1.0/0.125/70.0/66.0/0.1`, `0.39/0.78/1.17/1.56/0.02`, `3.0`, `5500.0`,
  `0.007/1.3/1.4`, `-0.1875`, `4.0/1.2`, `2^52 + 2^31`).
- The module header was updated: cluster item 128, the cluster count
  (127 to 128), the function totals (3367 to 3384 public translations, 3296 to
  3313 `fn_<addr>` bodies), the "start here" list and the records list.
- `cargo check --message-format=short` exits `0` with no error and no warning
  naming `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave146` reports 17 rows for this lane
  (`translated/high=12`, `translated/medium=5`), 17 unique addresses and `OK`.
