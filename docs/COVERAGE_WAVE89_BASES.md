# Coverage - Wave 89 (`game::bases`, `0x8078` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** under the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader chooses the real load address, so the numbers only
identify positions inside the REL image. Cross-module pointers (`0x8031xxxx`,
`0x8037xxxx`, `0x8042xxxx`) are `main.dol` data addresses imported by the module
and keep their absolute values.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`. "Role" below is the
plain-English behaviour. Confidence is `high` when guards, stores, constants
and call arguments are literal, and `medium` when the role of a field, a
global, a callee or a dropped argument had to be inferred (those bodies carry
`// observed:`/`// inferred:`/`// unknown:` notes). No body in this pass is
`low`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 2,548 | 21 | 9 |

## What this pass covers

The `0x8078` bucket holds 203 functions; 67 were already in
`ledger/functions.csv` when this pass began, leaving 136. This pass takes 30 of
the remaining bodies in three clusters plus a short tail.

- **Actor teardown and its release wrappers, `0x80787AE0` and
  `0x8078B620..0x8078C0D0` (7 bodies).** [`fn_80787ae0`] is the teardown that
  matches the wave-88 in-place constructor [`fn_80787990`]: it restores class
  vtable `0x808192F4` at `+0x60`, destroys the `+0x428` sub-object through its
  virtual `+0x08`, destroys the interface at `+0x39C`, empties the six-element
  `0x38`-byte array at `+0x1E4`, and releases `+0x1A4`, `+0x184`, `+0x158`,
  `+0x13C` and the base object. The six 88-byte wrappers
  [`fn_8078b620`]/[`fn_8078b840`]/[`fn_8078ba60`]/[`fn_8078bc80`]/
  [`fn_8078beb0`]/[`fn_8078c0d0`] are byte-identical: null guard,
  [`fn_80787ae0`]`(object, 0)`, then `0x80162920(object)` when `free > 0`.
- **`character_SV` object, `0x8078C260..0x8078C990` (5 bodies).** The in-place
  constructor [`fn_8078c260`] installs vtable `0x8031BF68` and then
  `0x8081A2A0` at `+0x60`, initialises the animation holder `+0x6E8`, the
  motion holder `+0x704`, the interface pair `0x803295D8`/`0x80329548` at
  `+0x744`, and one `0x38`-byte element at `+0x770`. [`fn_8078c6d0`] binds the
  `0x8081A1E8` animation table only for class indices 0/1; [`fn_8078c790`]
  attaches the animation once `0x800F7DA0` yields a nonzero handle;
  [`fn_8078c840`] searches the 22-entry table `0x8081A1E8` with
  `0x802E1BD0` against the class table at `0x8031BEFC`; [`fn_8078c990`] builds
  a 12-byte offset vector from the position at `+0xAC` and spawns `0x800F85F0`.
- **Player-notice state family, `0x8078E680..0x8078F1F0` (13 bodies).** The
  members share the latch pair `+0x1A8`/`+0x1AC` ([`fn_8078e680`],
  [`fn_8078e690`]), the phase byte `+0x1B0` and 10-frame / 30-frame countdown at
  `+0x1B4` ([`fn_8078e6b0`], [`fn_8078e7c0`], [`fn_8078f1d0`]; `0x8078F2D0`
  arms the same field to `0x1E` and is listed under skips),
  the mode pair `+0x1C8`/`+0x1CC` ([`fn_8078e950`]) and the test latch `+0x1D0`
  ([`fn_8078ecc0`]). [`fn_8078e6d0`], [`fn_8078e7e0`] and [`fn_8078f1f0`] are
  the three interaction steps; each polls a different global, latches one bit
  of the shared player word at `0x80834470 + 0x541/0x542/0x547`, arms the
  notice code at `+0x55C`, and finally advances the object through virtual
  `+0x68`. [`fn_8078e8d0`] maps a mode to `4`/`2`/`1`, [`fn_8078e910`]
  resets the notice subsystem, and [`fn_8078f1a0`] is the leading-zero test of
  `0x800FAD60(2, 3)`.
- **Sub-state setters, `0x807982A0..0x807984F0` (5 bodies).** Five literal
  writers of the shared `+0x228` state word (and `+0x254`/`+0xEC`/`+0x114` in
  [`fn_807982a0`]/[`fn_80798310`]).

No new host-side helper was needed: the pass reuses `abi_call1..5`, `vmethod0`,
`vmethod1`, `call0_ret` and `call_f32_u32x3`.

## Functions

| address | function | bytes | confidence | role |
|---|---|---:|---|---|
| `0x80787AE0` | `fn_80787ae0` | 240 | high | actor teardown |
| `0x8078B620` | `fn_8078b620` | 88 | high | release wrapper |
| `0x8078B840` | `fn_8078b840` | 88 | high | release wrapper |
| `0x8078BA60` | `fn_8078ba60` | 88 | high | release wrapper |
| `0x8078BC80` | `fn_8078bc80` | 88 | high | release wrapper |
| `0x8078BEB0` | `fn_8078beb0` | 88 | high | release wrapper |
| `0x8078C0D0` | `fn_8078c0d0` | 88 | high | release wrapper |
| `0x8078C260` | `fn_8078c260` | 184 | medium | in-place constructor |
| `0x8078C6D0` | `fn_8078c6d0` | 88 | medium | animation bind gate |
| `0x8078C790` | `fn_8078c790` | 84 | medium | animation attach |
| `0x8078C840` | `fn_8078c840` | 156 | high | class-name table search |
| `0x8078C990` | `fn_8078c990` | 116 | medium | offset effect spawn |
| `0x8078E680` | `fn_8078e680` | 16 | high | set latch pair |
| `0x8078E690` | `fn_8078e690` | 24 | high | clear latch pair |
| `0x8078E6B0` | `fn_8078e6b0` | 20 | high | arm 10-frame delay |
| `0x8078E6D0` | `fn_8078e6d0` | 228 | medium | first interaction state |
| `0x8078E7C0` | `fn_8078e7c0` | 20 | high | arm 10-frame delay |
| `0x8078E7E0` | `fn_8078e7e0` | 236 | medium | second interaction state |
| `0x8078E8D0` | `fn_8078e8d0` | 40 | high | mode mapping |
| `0x8078E910` | `fn_8078e910` | 56 | medium | notice subsystem reset |
| `0x8078E950` | `fn_8078e950` | 80 | high | initial mode select |
| `0x8078ECC0` | `fn_8078ecc0` | 72 | medium | test latch and re-arm |
| `0x8078F1A0` | `fn_8078f1a0` | 48 | high | `(2, 3)` state test |
| `0x8078F1D0` | `fn_8078f1d0` | 20 | high | arm 10-frame delay |
| `0x8078F1F0` | `fn_8078f1f0` | 212 | medium | third interaction state |
| `0x807982A0` | `fn_807982a0` | 16 | high | sub-state 5 |
| `0x80798310` | `fn_80798310` | 28 | high | copy animation start |
| `0x80798370` | `fn_80798370` | 12 | high | sub-state 10 |
| `0x80798410` | `fn_80798410` | 12 | high | sub-state 11 |
| `0x807984F0` | `fn_807984f0` | 12 | high | sub-state 4 |

## Address range

`0x80787AE0`, the contiguous teardown/release run `0x8078B620..0x8078C0D0`, the
`character_SV` cluster `0x8078C260..0x8078C990`, the player-notice family
`0x8078E680..0x8078F1F0`, and the sub-state setters
`0x807982A0..0x807984F0`. After this pass the bucket still has 106 untranslated
functions.

## Deliberate skips

The bodies below were read and rejected rather than guessed; the rest of the
bucket was simply not reached.

- `0x8078B5B0`, `0x8078B7D0`, `0x8078B9F0`, `0x8078BC10`, `0x8078BE30` and
  `0x8078C060` (48 bytes each) are the class registrations that sit immediately
  before the six release wrappers. Every one is exported as
  `halt_baddata` / "control flow encountered bad instruction data", so the
  descriptor stores are not recoverable; only the wrappers they register are
  translated here.
- `0x8078C500` (424 bytes) is the `g3d/bros.brres` + `character_SV` loader. Its
  body depends on `0x802DCD6C`/`0x802DCDB8` stack-frame helpers and three
  `(double)` casts that are paired-single conversions; the float arguments and
  the helper contract cannot be proven from the export, so it is deferred.
- `0x8078F2D0` (116 bytes) is reconstructible in outline (the `0x1E` arm and
  the `0x68` virtual) but its first call `0x800FC970(7)` and the second
  `0x800FB040(0, 0x28)` are unnamed predicates whose result polarity changes
  the branch; it was left for a pass that can name them.
- `0x8078E9A0`, `0x8078EB40`, `0x8078ED10`, `0x8078EDF0`, `0x8078EEE0` and
  `0x8078F060` (the remaining members of the notice family) all dereference
  `0x80429F7C` and call helpers that Ghidra prints with no arguments; their
  argument lists and global roles are unresolved, so only the six bodies whose
  argument flow is visible are translated.
- `0x8078F350` (196 bytes) calls virtual `+0x64` with `()` in the export, so the
  receiver register is lost; it is deferred rather than guessed.
- `0x807866A0..0x80786780`, `0x807875D0`/`0x80787600`,
  `0x80788090..0x80788CB0` and the `0x8078DC70` (2,576-byte) body are
  paired-single conversions (`unaff_GQR*`, `ldexpf`) and were not attempted.
- `0x80780010..0x8078B460` (the first half of the bucket) was not reached in
  this pass; earlier waves already translated 67 of those addresses.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 2,548 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the 67 pre-existing rows of
  this bucket were listed before writing and none is in this pass.
- `node tools/check-lane-csv.mjs wave89` reports
  `wave89-bases.csv: 30 rows`, `unique addresses: 30`, `OK`.
- `cargo check --message-format=short` produces no diagnostic for
  `src/game/bases.rs`.
