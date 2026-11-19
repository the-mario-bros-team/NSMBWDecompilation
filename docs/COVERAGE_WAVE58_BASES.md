# Coverage - Wave 58 (`game::bases`, `0x806e` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL (synthetic base `0x80600000`; every address is a static analysis address,
not a runtime address). Byte counts come from
`work/ghidra/rel-d_basesNP/export/functions.tsv`; the quoted evidence files are
the Ghidra C exports under `work/ghidra/rel-d_basesNP/export/`. Confidence is
`high` when guards, stores, call arguments and constants are literal, and
`medium` when a callee identity, a dropped `this` argument or a field role had
to be inferred.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 28 | 3472 | 16 | 12 |

## What this pass covers

- The `daLineTrain_c` cart family at `0x806EC0D0..0x806EE8F0`. The shared
  animation binder [`fn_806ec0d0`] selects one of the two names in the
  `0x807CBEF0` table and drives `0x801651F0`, `0x80169F80` and `0x801680E0`;
  the WaitParent handler [`fn_806ec190`] and its third-state sibling
  [`fn_806ec8b0`] flip the `+0x17A..+0x17E` / `+0x1B1` flags and play sounds
  `0x235` / `0x233`.
- The two model loaders [`fn_806ec830`] (`lift_torokkoA`) and
  [`fn_806eceb0`] (`lift_torokkoB`), which share the `0x807CBEF0` `normal`
  material and the `0x80165FD0` / `0x801650D0` attach helpers.
- The cart-array driver: [`fn_806ed010`] walks `0x80162D50(0x24D, ...)` for the
  actor carrying the requested `+0x1392` byte, [`fn_806ed080`] publishes it at
  `+0x1380` and enters WaitParent, [`fn_806ed0f0`] flags the cart whose
  accumulated `+0x16C` span is reached, [`fn_806ed460`] / [`fn_806ed4b0`]
  re-apply the animation and set the leave-state flag on every cart, and
  [`fn_806ed310`] / [`fn_806edcd0`] / [`fn_806edd80`] count the `+0x1393`
  rider mask and enter the `0x8082E7A8` Move state.
- The transform/state helpers [`fn_806ec420`] (`+0x16C`/`+0x170` clamp),
  [`fn_806ec560`] (matrix from `+0x16C..+0x174` plus the `+0x178` rotation),
  [`fn_806ec4a0`] (base virtuals, `0x80358E50` tag, frame write),
  [`fn_806ec930`] (probe/burst/sound), [`fn_806ede10`] (2^52 lane conversion)
  and [`fn_806ed4e0`] (slope mode).
- The adjacent `0x806E9E10` / `0x806EEBC0` / `0x806EF350..0x806EFA90` helpers:
  the `0x5D8`-byte constructor [`fn_806e9e10`], the `0x807FE7E0` in-place
  constructor [`fn_806eebc0`], the registered-state destructor
  [`fn_806ef350`], the shared descriptor getter [`fn_806ef3b0`], the `+0x30`
  chain probe [`fn_806ef4a0`], the name-suffix comparator [`fn_806ef580`] and
  the clamped angle/position update [`fn_806efa90`].
- Two host-side helpers were added: `call_f32x4_u32` (four `f32` plus one
  `u32`, modelling `0x8015F140`). All other ABI wrappers already existed in the
  file (`call_f32x3_u32`, `call_f32_u32`, `call_f32_u32x2`,
  `call_f32x2_u32x2`).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806E9E10` | `fn_806e9e10` | 116 | medium |
| `0x806EC0D0` | `fn_806ec0d0` | 152 | high |
| `0x806EC190` | `fn_806ec190` | 276 | high |
| `0x806EC420` | `fn_806ec420` | 124 | high |
| `0x806EC4A0` | `fn_806ec4a0` | 192 | medium |
| `0x806EC560` | `fn_806ec560` | 136 | medium |
| `0x806EC830` | `fn_806ec830` | 120 | high |
| `0x806EC8B0` | `fn_806ec8b0` | 124 | high |
| `0x806EC930` | `fn_806ec930` | 276 | medium |
| `0x806ECEB0` | `fn_806eceb0` | 172 | high |
| `0x806ED010` | `fn_806ed010` | 100 | high |
| `0x806ED080` | `fn_806ed080` | 108 | high |
| `0x806ED0F0` | `fn_806ed0f0` | 112 | medium |
| `0x806ED310` | `fn_806ed310` | 44 | high |
| `0x806ED460` | `fn_806ed460` | 72 | high |
| `0x806ED4B0` | `fn_806ed4b0` | 40 | high |
| `0x806ED4E0` | `fn_806ed4e0` | 228 | medium |
| `0x806EDCD0` | `fn_806edcd0` | 88 | medium |
| `0x806EDD80` | `fn_806edd80` | 144 | medium |
| `0x806EDE10` | `fn_806ede10` | 72 | high |
| `0x806EE8D0` | `fn_806ee8d0` | 32 | high |
| `0x806EE8F0` | `fn_806ee8f0` | 80 | high |
| `0x806EEBC0` | `fn_806eebc0` | 108 | medium |
| `0x806EF350` | `fn_806ef350` | 92 | high |
| `0x806EF3B0` | `fn_806ef3b0` | 12 | high |
| `0x806EF4A0` | `fn_806ef4a0` | 224 | medium |
| `0x806EF580` | `fn_806ef580` | 136 | medium |
| `0x806EFA90` | `fn_806efa90` | 92 | medium |

## Address range

`0x806E9E10..0x806EFA90`: one standalone constructor at `0x806E9E10`, the
contiguous `daLineTrain_c` run `0x806EC0D0..0x806EE8F0` (twenty-one addresses, one
`0x806ED160`/`0x806ED260`/`0x806ED340` gap), and the `0x806EEBC0..0x806EFA90`
helper family.

## Deliberate skips

- `0x806EC2B0` (200), `0x806EC380` (148), `0x806EC5F0` (220), `0x806EC6D0`
  (172), `0x806EC780` (172), `0x806ED340` (288), `0x806EDE70` (308) and
  `0x806EECA0` (384) load floats through an undeclared `unaff_GQR0`/`unaff_GQR3`
  register and print `ldexpf` plus `CONCAT22` splices; the paired-single
  sequence cannot be recovered from the export honestly.
- `0x806ED160` (252) and `0x806ED260` (176) are a caller/callee pair whose
  export declares `void FUN_806ed160(void)` while `FUN_806ed260` calls it with
  two arguments and consumes a byte result. The real prototype is not
  recoverable, so both were left for a later wave.
- `0x806EF9E0` (148) calls `0x801611C0`, whose two `double` arguments are
  dropped from the export (the main.dol body is an `atan2`-style helper); the
  missing operands make the body ambiguous.
- `0x806EFD70` (88) is a byte-identical copy of [`fn_806ef350`] at a different
  address, and `0x806EE5D0`/`0x806EE600`/`0x806EF640`/`0x806EF670`/
  `0x806EFE90`/`0x806EFEC0` (48 bytes each) are the four-word `0x802DCBFC`
  forwarders already represented by `fn_806512f0`/`fn_80651320`. They add no
  new evidence and were not duplicated.
- `0x806EEE20` (200) walks `0x1398`-byte manager slots through an unprototyped
  `func_0x80064680` spawn call whose argument count is dropped; left for a
  later pass.
- These addresses were already in `ledger/functions.csv` and were not
  duplicated: `0x806E0FB0`, `0x806E10C0`, `0x806E1240`, `0x806E2320`,
  `0x806E23E0`, `0x806E3480`, `0x806E3630`, `0x806E37F0`, `0x806E3970`,
  `0x806E4540`, `0x806E4DA0`, `0x806E5DF0`, `0x806E6810`, `0x806E6C10`,
  `0x806E6F00`, `0x806E7530`, `0x806E76F0`, `0x806E7870`, `0x806E7900`,
  `0x806E8140`, `0x806E9E90`, `0x806E9F40`, `0x806EA030`, `0x806EA0F0`,
  `0x806EA110`, `0x806EA330`, `0x806EA410`, `0x806EA4E0`, `0x806EA550`,
  `0x806EA5C0`, `0x806EA9A0`, `0x806EAB20`, `0x806EABF0`, `0x806EAC40`,
  `0x806EAC90`, `0x806EACB0`, `0x806EADA0`, `0x806EAE50`, `0x806EAF80`,
  `0x806EB010`, `0x806EB060`, `0x806EB0E0`, `0x806EB540`, `0x806EB600`,
  `0x806EB6B0`, `0x806EB880`, `0x806EBC40`, `0x806EBDC0`, `0x806EBF10`,
  `0x806EBF90`, `0x806EC080`, `0x806EE170`, `0x806EE4B0`, `0x806EE630`,
  `0x806EF6A0`, `0x806EFEF0`.
- The rest of the `0x806e` bucket (145 untranslated at lane end, including the
  `0x806E0110..0x806E1FFF` front block and the `0x806EFBxx` tail) is outside
  this window.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv`.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000` line.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error for
  `src/game/bases.rs`.
