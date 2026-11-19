# Coverage - Wave 112 (`game::bases`, `0x8064` bucket)

## How to read this file

This page documents the Wave 112 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`** produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
cited file is a real Ghidra C export under
`work/ghidra/rel-d_basesNP/export/`, and every byte count was re-read from
`work/ghidra/rel-d_basesNP/export/functions.tsv`.

"Confidence" is `high` when the guards, stores, constants and call arguments are
literal, and `medium` when a field role, a callee identity or a dropped receiver
had to be inferred. No body in this pass is `low`. Comments in the bodies keep
`// observed:`, `// inferred:` and `// unknown:` markers, and the two bodies that
call a truncated neighbour carry a `// partial:` note.

The `0x8064` bucket holds 196 functions; 83 were already translated before this
pass, leaving 113. This pass takes 30 of them, all in the item-window middle
that wave 83 deferred.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 5,396 | 14 | 16 |

## What this pass covers

- **The score-screen member tail (`0x80642DF0..0x80643F30`, 10 bodies).** The
  screen is the one whose state table, destructor and constructor were recorded
  in wave 13 as `dStaffCreditScore_c` (`0x806415D0`, destructor `0x80641C00`,
  constructor [`fn_80641d20`]). The pass adds the `Wm_2d_stockitem` spawner
  [`fn_80642df0`] and its `+0x77C` transform forwarder [`fn_80642f30`], the
  pad-confirm body [`fn_80643040`] with its five cues (`0x669..0x66E`), the
  column cursor and its repeat step
  [`fn_80643880`]/[`fn_80643ac0`], the animator stop [`fn_80643c90`], the two
  clip gates [`fn_80643cd0`]/[`fn_80643ea0`], the mid-sequence rebuild
  [`fn_80643e50`] and the confirmation cooldown [`fn_80643f30`].
- **The `dStockItem_c` redraw pair (`0x80644C40..0x80644CF0`, 2 bodies).** The
  window registered at `0x806441E0` (eight states, class vtable `0x807E2AF8`)
  forwards its draw pass in [`fn_80644c40`] and recolours the seven item records
  in [`fn_80644cf0`], including the one-time initialisation of the two
  `0x80827AE8`/`0x80827AF8` colour tables.
- **The unnamed `0x1A500`-byte UI actor (`0x80647B50..0x806485C0`, 7 bodies).**
  A player/phase gate [`fn_80647b50`], two pitch writers
  [`fn_80648260`]/[`fn_80648300`]/[`fn_806483b0`], the shared height publisher
  [`fn_80648440`], the `0x802DD1EC` mask classifier [`fn_80648550`] and the
  per-frame pitch ramp [`fn_806485c0`]. The class name is `// unknown:`; the
  page uses its size and field map instead.
- **The `block_arrow` actor (`0x8064A290..0x8064A5F0`, 2 bodies).** The variant
  and callback-table writer [`fn_8064a290`] (it calls the already translated
  model loader [`fn_8064a6d0`]) and the matrix rebuild [`fn_8064a5f0`].
- **Seven deleting-destructor instantiations
  (`0x80641F30..0x80645DB0`, `0x8064E6C0`).** Two shapes only: a plain
  `0x802B9280` release and a `0x8015F9F0(+0x18)` sub-object destructor followed
  by that release; [`fn_8064e6c0`] uses the small `0x80162920` allocator
  instead.
- **Two small utilities (`0x8064E180`, `0x8064E7D0`).** The one-step index wrap
  over the `+0x40C` span and the colour/angle reset of a small screen object.

Start with [`fn_80642df0`] for the effect spawn and [`fn_80643040`] for the
confirmation logic shared by the cursor bodies. [`fn_80644cf0`] is the field map
for the stock-item window (`+0x244` art pane, `+0x264` seven record pointers,
`+0x20C`/`+0x228` interfaces).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80641F30` | `fn_80641f30` | 96 | high |
| `0x80641F90` | `fn_80641f90` | 100 | high |
| `0x80642DF0` | `fn_80642df0` | 308 | medium |
| `0x80642F30` | `fn_80642f30` | 272 | medium |
| `0x80643040` | `fn_80643040` | 532 | medium |
| `0x80643880` | `fn_80643880` | 520 | medium |
| `0x80643AC0` | `fn_80643ac0` | 432 | medium |
| `0x80643C90` | `fn_80643c90` | 64 | high |
| `0x80643CD0` | `fn_80643cd0` | 108 | high |
| `0x80643E50` | `fn_80643e50` | 72 | medium |
| `0x80643EA0` | `fn_80643ea0` | 88 | high |
| `0x80643F30` | `fn_80643f30` | 208 | medium |
| `0x80644C40` | `fn_80644c40` | 92 | high |
| `0x80644CF0` | `fn_80644cf0` | 460 | medium |
| `0x80644FA0` | `fn_80644fa0` | 64 | high |
| `0x80645020` | `fn_80645020` | 96 | high |
| `0x80645D30` | `fn_80645d30` | 64 | high |
| `0x80645DB0` | `fn_80645db0` | 96 | high |
| `0x80647B50` | `fn_80647b50` | 108 | medium |
| `0x80648260` | `fn_80648260` | 160 | medium |
| `0x80648300` | `fn_80648300` | 168 | medium |
| `0x806483B0` | `fn_806483b0` | 132 | medium |
| `0x80648440` | `fn_80648440` | 48 | high |
| `0x80648550` | `fn_80648550` | 112 | medium |
| `0x806485C0` | `fn_806485c0` | 332 | medium |
| `0x8064A290` | `fn_8064a290` | 276 | medium |
| `0x8064A5F0` | `fn_8064a5f0` | 220 | medium |
| `0x8064E6C0` | `fn_8064e6c0` | 88 | high |
| `0x8064E180` | `fn_8064e180` | 40 | high |
| `0x8064E7D0` | `fn_8064e7d0` | 40 | high |

**Totals:** 30 translated, 0 partial, 0 stub (14 high, 16 medium, 5,396 bytes).

## Deliberate skips

Read and rejected, with the reason:

- **Paired-single bodies.** `0x80642D20` (204 bytes) opens with an `unaff_GQR0`
  bracket, `ldexpf` lane conversion and incoming `ps29`..`ps31` registers, so the
  halfword scaling of the four pane transforms cannot be reconstructed. Same
  reason as `0x806333D0` in wave 111 and `0x806E3EB0` in wave 109; the body
  remains an address-only call from [`fn_80642380`].
- **`halt_baddata` bodies.** `0x80640AC0` (116 bytes) is empty apart from
  `halt_baddata`; `0x80642CE0` (64) truncates after one float add; `0x80643260`
  (148) truncates after its final store. `0x80643260`'s two scan loops are
  readable and are documented, but it stays an address call from
  [`fn_80643880`] and [`fn_80643ac0`] with a `// partial:` note rather than
  being guessed.
- **Dropped-receiver calls.** `0x80643470` (228 bytes) and `0x80643D40` (268)
  call `FUN_807678C0()` with no printed argument while `fn_807678c0` takes a
  pointer, and each body has two candidate receivers (`this` or the item pointer
  at `+0x2F4`). That remains the wave-83 rejection: the register cannot be
  recovered honestly.
- **Budget.** `0x80642540` (1,236 bytes, the seven-pane rebuild), `0x80642A20`
  (660), `0x80647E60` (512) and the remaining `0x80648770..0x8064F820` bodies
  were not opened in this pass (budget, not doubt). `0x80642A20` and
  `0x80647E60` are called address-only from the new bodies with `// unknown:`
  notes.

## Notes on inference

- The class attribution of the `0x80642DF0..0x80643F30` cluster to
  `dStaffCreditScore_c` is `// inferred:` from the wave-13 state table at
  `0x806415D0` and its constructor [`fn_80641d20`]; no export in this run names
  the class. The effect it spawns is the literal `Wm_2d_stockitem` string at
  `0x807E2650`.
- The `0x80644C40`/`0x80644CF0` pair belongs to `dStockItem_c` because
  `0x806441E0` registers that class name and the loader `0x80644B30` binds the
  seven `T_itemRest_07` animations used by the recolour loop.
- `fn_80643040` is declared `void` by Ghidra but every call site reads `r3`;
  the translation returns the value through the `0x802DCDB8` thunk, matching the
  existing `fn_806549a0` convention.
- `0x80429F7C`, `0x80429DD0` and `0x8042A028` are followed exactly as the
  existing bodies do: `+` offsets on `0x80429F7C`/`0x80429DD0` are direct
  address arithmetic, while bare `_DAT_` scalars are loaded with
  `platform::read_u32`.
- The two `Wm_2d` block copies in [`fn_80642df0`]/[`fn_80642f30`] keep the
  export's stack-slot order instead of inventing a `Mtx` field layout.
- [`fn_80644cf0`] dispatches the four colour pushes through the same
  `(object, slot, value)` virtual shape that the export prints; the loop is
  written as a four-element table so the argument order stays visible.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 5,396 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 83 translated
  addresses were re-read first and none is duplicated here. The file now holds
  2,798 bodies.
- `node tools/check-lane-csv.mjs wave112` reports no problem naming
  `wave112-bases.csv`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/bases.rs`; diagnostics raised by other lanes' files are ignored.
