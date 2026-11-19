# Coverage - Wave 70 (enemies: `0x80A35B10..0x80A35EA0`, `0x80A38C90..0x80A3FB30`)

## How to read this file

This page records the Wave 70 `game::enemies` lane: the `0x80A30000` bucket of
REL module 3, `d_enemiesNP`, promoted to `src/game/enemies.rs`. Every row below
corresponds to one real `pub unsafe fn` body in that file, and the `evidence`
column names the Ghidra C export the body was reconstructed from. The CSV that
mirrors this table is `ledger/agents/wave70-enemies.csv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 2,520 | 12 | 18 |

## What this pass covers

The bucket `0x80A30000` holds 149 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; two of them were already in
`ledger/functions.csv` (`80a399e0` `register_cliff_chikuwa_states` and
`80a39de0` `alloc_cliff_chikuwa_actor`). 147 were untranslated. This pass claims
30 of those and leaves 117.

- **`0x80A35B10` skeleton (`0x80A35B10..0x80A35EA0`).** The same five-entry
  state-descriptor shape already reconstructed in waves 46, 54, 55, 68 and 69:
  two normalisers ([`fn_80a35b10`], [`fn_80a35b70`]), two constant producers
  ([`fn_80a35bd0`] = `0x80358004`, [`fn_80a35be0`] = `0x80357BF4`), the `+0x30`
  chain walker [`fn_80a35cd0`], the `:`-suffix name predicate
  [`fn_80a35db0`] and the two 48-byte deleting-destructor forwarders
  [`fn_80a35e70`]/[`fn_80a35ea0`].
- **`0x80A38C90` skeleton (`0x80A38C90..0x80A39020`).** A second copy with
  producers `0x80357BB4` ([`fn_80a38d50`]) and `0x80357BF4`
  ([`fn_80a38d60`]); both words are shared base states used by earlier blocks
  of this module, not class-private descriptors.
- **`0x80A39050` constructor.** The `0x630`-byte enemy actor: vtable
  `0x80B3E428`, holders at `+0x524`, `+0x544`, a `0x803295D8`/`0x80329548`
  pair at `+0x584` around the sub-object built at `+0x590`, and the `0x800DB130`
  motion helper at `+0x5B0`. The store sequence is the same one shared by
  [`fn_80acd9a0`] and `alloc_rail_poly_child_actor`, so the class is an unnamed
  member of that constructor family.
- **`0x80A39560` and `0x80A395A0`, the constructor's two state helpers.** The
  first is a frame-gated toggle of the word at `+0x61C` that also copies a
  halfword from the two-entry table at `0x80B2C288` into `+0x628`. The second
  walks the `+0x5C0` child list and reports whether any child has class byte
  `1` (with `+0x1090 != 3`) or class byte `2`.
- **The `0x80A39C60` normaliser and its two forwarders.** The
  `daEnCliffChikuwa_c` registration at `0x80A399E0` is preceded by another copy
  of the skeleton normaliser and followed by the class's two 48-byte
  deleting-destructor thunks ([`fn_80a39d80`], [`fn_80a39db0`]).
- **The three-float `sqrt` normalisers (`0x80A3A000`/`0x80A3A0A0`, duplicated at
  `0x80A3CDD0`/`0x80A3CE70`).** Each pair is one vector normaliser plus a
  difference-vector wrapper. The two copies differ only in the "already zero"
  comparison constant (`0x80B2C314` vs `0x80B2C460`), which is what shows they
  are two translation units of the original program rather than one function
  reached twice.
- **Owner-change holder release (`0x80A3CA30`, `0x80A3DB90`).** Byte-identical
  80-byte steps: compare `+0x538` against the scene handle from `0x8016A5D0`,
  and on mismatch run the `+0x544` holder's vtable slot `+0x10`. Unlike the
  `obj_obake_door` copy (`0x80AF5280`) there is no second unlink call.
- **Two small state helpers.** `0x80A3DE90` raises the `+0xA90` word and the
  `+0xA97` byte; `0x80A3FB30` counts the `+0xA88` halfword timer down and calls
  `0x80A3FAC0` when it reaches zero.

### Identity evidence

- `0x80A39050` is literal: `0x801628C0(0x630)`, `&DAT_80b3e428` into `+0x60`,
  `0x80169020`-family holders at `+0x524`/`+0x544`, the `0x803295D8` →
  `0x80329548` vtable swap around the `+0x590` sub-object, and `0x800DB130` at
  `+0x5B0`.
- `0x80A39560` is literal: `_DAT_8042a200 & 2`, an xor-by-one on `+0x61C`, and
  `*(undefined2 *)(&DAT_80b2c288 + flag * 2)` read into `+0x628`.
- `0x80A3A000`/`0x80A3CDD0` are literal: the same sum `z*z + x*x + y*y`, one
  call to `0x802E8868`, and the three-way divide.
- No class name is printed anywhere in this range. The constructor is described
  by size, offsets and its shared-layout family only.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A35B10` | `fn_80a35b10` | 88 | high |
| `0x80A35B70` | `fn_80a35b70` | 92 | high |
| `0x80A35BD0` | `fn_80a35bd0` | 12 | high |
| `0x80A35BE0` | `fn_80a35be0` | 12 | high |
| `0x80A35CD0` | `fn_80a35cd0` | 224 | medium |
| `0x80A35DB0` | `fn_80a35db0` | 136 | medium |
| `0x80A35E70` | `fn_80a35e70` | 48 | medium |
| `0x80A35EA0` | `fn_80a35ea0` | 48 | medium |
| `0x80A38C90` | `fn_80a38c90` | 88 | high |
| `0x80A38CF0` | `fn_80a38cf0` | 92 | high |
| `0x80A38D50` | `fn_80a38d50` | 12 | high |
| `0x80A38D60` | `fn_80a38d60` | 12 | high |
| `0x80A38E50` | `fn_80a38e50` | 224 | medium |
| `0x80A38F30` | `fn_80a38f30` | 136 | medium |
| `0x80A38FF0` | `fn_80a38ff0` | 48 | medium |
| `0x80A39020` | `fn_80a39020` | 48 | medium |
| `0x80A39050` | `fn_80a39050` | 160 | medium |
| `0x80A39560` | `fn_80a39560` | 52 | medium |
| `0x80A395A0` | `fn_80a395a0` | 80 | medium |
| `0x80A39C60` | `fn_80a39c60` | 88 | high |
| `0x80A39D80` | `fn_80a39d80` | 48 | medium |
| `0x80A39DB0` | `fn_80a39db0` | 48 | medium |
| `0x80A3A000` | `fn_80a3a000` | 160 | medium |
| `0x80A3A0A0` | `fn_80a3a0a0` | 92 | high |
| `0x80A3CA30` | `fn_80a3ca30` | 80 | medium |
| `0x80A3CDD0` | `fn_80a3cdd0` | 160 | medium |
| `0x80A3CE70` | `fn_80a3ce70` | 92 | high |
| `0x80A3DB90` | `fn_80a3db90` | 80 | medium |
| `0x80A3DE90` | `fn_80a3de90` | 16 | high |
| `0x80A3FB30` | `fn_80a3fb30` | 44 | medium |

## Deliberate skips

- **Five exports carry `halt_baddata()` and are not reconstructable at all:**
  `0x80A337B0` (a 4-byte thunk to the next address), `0x80A337C0`,
  `0x80A36A50`, `0x80A3F630` and `0x80A3F6D0`. Ghidra truncated control flow
  inside them, so no body is claimed for these addresses.
- **`0x80A35570`** decodes as an indirect jump Ghidra could not recover
  ("Could not recover jumptable ... Too many branches"). The visible part is a
  single tail dispatch through vtable slot `+0x3F4`; claiming it as a
  translation would misrepresent the real body, so it is skipped.
- **The large actor blocks around `0x80A31190..0x80A33470`,
  `0x80A34240..0x80A35680` and `0x80A36C10..0x80A38050`.** These are clean
  exports and are the natural next contiguous run, but they are per-frame
  handlers of classes whose constructors are outside this bucket. They were
  left for a later pass rather than claimed with inferred class identities.
- **`0x80A3A4A0` (2,620 bytes) and `0x80A3DEA0` (2,940 bytes).** Both are large
  switch-style handlers. The lane budget was better spent on the shared
  descriptor skeletons, which unblock every class in this bucket.
- **`0x80A3FAC0`.** It is the callee of [`fn_80a3fb30`] but lies outside the
  claimed run; the call is kept as an address, and the doc marks its role as
  inferred.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 30 sizes sum to
  2,520 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none was already in
  `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave70` reports `OK` with
  `wave70-enemies.csv: 30 rows`.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/game/enemies.rs`.
