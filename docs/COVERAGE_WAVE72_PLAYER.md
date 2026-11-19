# Coverage - Wave 72 (`game::player`, the attachment table and the `0x8004` lifecycle leaves)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 30 | 3088 | 22 | 8 |

## How to read this file

This document covers the Wave-72 slice of the `0x80040000` bucket of
`work/ghidra/main-gb/export/functions.tsv`. Every body lives in
`src/game/player.rs`; the ledger rows are in
`ledger/agents/wave72-player.csv`. The module now holds two hundred ninety-nine
translated functions.

* **Address** - virtual entry point in the DOL image (not a file offset).
* **Name** - the Ghidra name. No original symbol table survives, so
  `FUN_<address>` is the honest identity; the Rust body keeps the
  `fn_<address>` name.
* **Role** - one plain sentence. Roles marked "inferred" come from shared field
  offsets and call shapes, not from a recovered symbol.
* **Status** - all rows are `translated`: the whole control flow and every call
  are present.
* **Confidence** - strength of the evidence, not code quality: `high` =
  instruction-level match, `medium` = a receiver, a field role or a string
  identity had to be inferred.
* **Evidence** - the Ghidra C file under `work/ghidra/main-gb/export/`.

The bodies use the two SDA bases this module already fixes: `r2 = 0x80433080`
and `r13 = 0x8042F680`. So `r2 - 0x77FC` is `0x8042B884` (the angle table in
[`fn_80047520`]) and `r2 - 0x780C` is `0x8042B878` = `1.0f`. The `0x802DCD70`
and `0x802DCDBC` symbols Ghidra prints around some of these bodies are the
CodeWarrior `__savegprlr`/restore fragments, not calls: they are documented as
such and omitted, exactly as in the earlier Waves 49 and 63.

## What this pass covers

- **Eight-slot attachment table (`0x80043CD0..0x800440C0`, 5 bodies)** - the
  byte table at `+0x442..+0x449` with `0xFF` as the empty sentinel: the
  membership test [`fn_80043cd0`], the append [`fn_80043da0`], the hole-closing
  shift [`fn_80043e60`], the remove-and-compact [`fn_80043eb0`], and the
  per-frame driver [`fn_800440c0`] that reconciles the table with the four-bit
  presence mask cached at `+0x440` and carries forward any bit the owner
  refuses to release. Start at [`fn_800440c0`].
- **Release wrappers (`0x800414A0..0x800448A0`, 7 bodies)** - the
  `(object, destroy)` shape this engine repeats once per class: a plain
  `0x802B9280` free ([`fn_800414a0`], [`fn_80046670`], [`fn_800466b0`]), a
  state-descriptor release through `0x8015F800` before the free
  ([`fn_800418c0`], [`fn_80043430`], [`fn_80043490`]), the member release
  through `0x8015F9F0` ([`fn_800466f0`], [`fn_80046750`]), and the two longer
  teardowns that stop a material member at `+0x3D0` ([`fn_80044390`]) or a
  scene member at `+0x3C8` plus the animation member at `+0x3A8`
  ([`fn_800448a0`]).
- **Child-chain helpers (`0x80041A00`, `0x80041AE0`, 2 bodies)** - the
  five-probe walk down the `+0x30` chain through vtable slot `+0x0C`
  ([`fn_80041a00`]) and the `prefix:suffix` name comparison that uses the
  already-translated `strrchr`/`strcmp` pair from `engine::main_loop`
  ([`fn_80041ae0`]).
- **Facing and angle leaves (`0x80047490..0x800477B0`, 6 bodies)** - the three
  ways to retarget the signed heading at `+0x102` from the facing byte at
  `+0x348` (actor slot `+0x444`, the shared table at `0x8042B884`, or a
  caller-supplied target), the symmetric partner test at `+0x1008`, and the two
  event-flag `0x52` leaves.
- **Attach/hold state (`0x800479A0..0x80047D60`, 6 bodies)** - enter
  ([`fn_800479a0`]), leave ([`fn_80047a00`]), per-frame upkeep
  ([`fn_80047ac0`]), the gated attach predicate plus animation name
  ([`fn_80047b70`]), and the two lighter teardowns ([`fn_80047d10`],
  [`fn_80047d60`]). The state owns the mode word at `+0x6EC`, the interface
  object at `+0xDEC`, the movement sub-object at `+0xEA4` and six event flags.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800414A0` | `fn_800414a0` | 64 | high |
| `0x800418C0` | `fn_800418c0` | 92 | high |
| `0x80041A00` | `fn_80041a00` | 224 | medium |
| `0x80041AE0` | `fn_80041ae0` | 136 | high |
| `0x80043430` | `fn_80043430` | 88 | high |
| `0x80043490` | `fn_80043490` | 92 | medium |
| `0x80043CD0` | `fn_80043cd0` | 204 | high |
| `0x80043DA0` | `fn_80043da0` | 192 | high |
| `0x80043E60` | `fn_80043e60` | 68 | high |
| `0x80043EB0` | `fn_80043eb0` | 80 | high |
| `0x800440C0` | `fn_800440c0` | 288 | medium |
| `0x80044390` | `fn_80044390` | 108 | high |
| `0x800448A0` | `fn_800448a0` | 128 | high |
| `0x80046670` | `fn_80046670` | 64 | high |
| `0x800466B0` | `fn_800466b0` | 64 | high |
| `0x800466F0` | `fn_800466f0` | 96 | high |
| `0x80046750` | `fn_80046750` | 100 | medium |
| `0x80047490` | `fn_80047490` | 80 | high |
| `0x800474E0` | `fn_800474e0` | 56 | medium |
| `0x80047520` | `fn_80047520` | 16 | medium |
| `0x80047530` | `fn_80047530` | 68 | high |
| `0x80047580` | `fn_80047580` | 48 | medium |
| `0x800477A0` | `fn_800477a0` | 8 | high |
| `0x800477B0` | `fn_800477b0` | 68 | high |
| `0x800479A0` | `fn_800479a0` | 96 | high |
| `0x80047A00` | `fn_80047a00` | 188 | high |
| `0x80047AC0` | `fn_80047ac0` | 172 | high |
| `0x80047B70` | `fn_80047b70` | 100 | medium |
| `0x80047D10` | `fn_80047d10` | 80 | high |
| `0x80047D60` | `fn_80047d60` | 20 | high |

Byte total: 3088. Address range covered: `0x800414A0..0x80047D60`
(non-contiguous; the gaps are the deliberate skips below).

## Deliberate skips and why

The bucket holds 106 untranslated functions after this pass.

* **The 12-byte address leaves (53 functions)** - `0x80040600`, `0x80040610`,
  `0x80040620`, `0x80040630` (the four `0x80353AA4..0x80353B64` descriptor
  words), `0x80040920..0x80040950`, `0x80042AE0..0x80042B30`, `0x80044940`,
  `0x80044950`, `0x80047270..0x80047380` and `0x8004D6F0..0x8004D810` (all
  returning `&DAT_80377A04`). Each body is a single `lis`/`addi`/`blr` that
  returns one fixed address. They are honest one-liners but they are vtable
  padding, not behaviour; translating 53 copies would triple the row count
  without adding information. They can be added mechanically if a later pass
  needs the count.
* `0x80047440` (36 B) - the export carries "Could not recover jumptable at
  0x80047460. Too many branches" and dispatches through an indirect jump whose
  table is not in the export. Restating it as a single virtual call would lose
  the dispatch.
* `0x80047C40` (196 B) - the effect call is printed as
  `(*slot+0x5C)((double)0x80370E14, (double)10.0f, (double)0.0f, this, 6)`.
  Three `f64` arguments before the receiver do not match this module's
  documented `(f32, f32, f32, object, id)` slot shape, so the real argument
  order cannot be stated without guessing.
* `0x800410F0` (212 B), `0x80047F10` (360 B), `0x800414E0` (888 B),
  `0x80047DC0` (288 B), `0x80047EE0` (36 B) - these read `unaff_GQR0` or call
  their own neighbours with no recovered argument list (`FUN_80047EE0()` and
  `FUN_80047F10()` are printed with no parameters). They need a
  paired-single/dispatch pass of their own.
* `0x80044550` (520 B), `0x800441E0` (432 B), `0x80045710` (732 B),
  `0x800460D0` (828 B), `0x800467C0` (652 B), `0x80046FC0` (452 B),
  `0x80047630` (348 B), `0x80048630` (704 B) - the large dispatchers between
  the clusters selected above; each needs its own call-graph pass.
* `0x80042480` (128 B), `0x80042650` (164 B), `0x800437D0` (72 B),
  `0x80046490` (88 B) - these read SDA words whose object types are not
  established (`r2 - 0x7900`, `r2 - 0x78D4`, `r13 - 0x5648`, `r13 - 0x5940`)
  or call a helper the export prints with no argument list (`FUN_8027A200()`
  in `0x80046490`, which also guards on the comma-expression
  `(FUN_8027a200(), 0 < param_2)`). A guessed SDA role would have to be
  presented as fact, so they were left.
* The remaining mid-size helpers of the bucket were not attempted in this pass;
  the five clusters above were chosen because their callees are already
  translated and their shared fields (`+0x440`, `+0x442`, `+0x6EC`, `+0xEA4`,
  `+0x102`) are cross-checked by more than one body.
* `0x8004DB00` (52 B) - a two-argument tail call into `0x802DCBFC` with a fixed
  table pointer, i.e. the same hidden-`r12` dispatch this file already records
  as partial for `fn_80049940`; not duplicated here.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/player.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error in
  `src/game/player.rs` (the only remaining crate errors are in other lanes'
  files).
- `node tools/check-lane-csv.mjs wave72` reports `OK` for
  `ledger/agents/wave72-player.csv`.
