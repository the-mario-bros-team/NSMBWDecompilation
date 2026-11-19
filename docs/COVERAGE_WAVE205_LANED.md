# Coverage - Wave 207 lane D (`ui::hud`, main.dol)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---|---:|---:|---:|
| `src/ui/hud.rs` | `ui::hud` | 10 | 956 | 7 | 3 |

Addresses `0x8003_7D20`-`0x8003_81B0`: the `togezo` (Spiny) release/build band.
All ten bodies are translated; nothing in this lane is deferred. No other lane
claims an address in this range: the neighbouring rows in `functions.tsv` below
`0x8003_7D20` (`0x8003_7D80`, `0x8003_7E10`) and above `0x8003_81B0`
(`0x8003_8360`) are not in this lane, and the constructor at `0x8003_7EF0` is
absent from `functions.tsv` and is not claimed here.

## Scope

| address | size | name | confidence | role |
|---|---|---:|---|---|
| `0x80037D20` | 88 | `fn_80037d20` | high | release one record, free it when `flag > 0` |
| `0x80037E40` | 48 | `fn_80037e40` | medium | adjustor thunk, descriptor `a + 0x18` |
| `0x80037E70` | 48 | `fn_80037e70` | medium | adjustor thunk, descriptor `a + 0x24` |
| `0x80037EA0` | 12 | `fn_80037ea0` | high | `0x80377A04` accessor |
| `0x80037EB0` | 12 | `fn_80037eb0` | high | second accessor |
| `0x80037EC0` | 12 | `fn_80037ec0` | high | third accessor |
| `0x80037ED0` | 12 | `fn_80037ed0` | high | fourth accessor |
| `0x80037EE0` | 12 | `fn_80037ee0` | high | fifth accessor |
| `0x80038080` | 292 | `fn_80038080` | high | `togezo` base-scene destructor |
| `0x800381B0` | 420 | `fn_800381b0` | medium | model, `"revival_shell"` clip and optional second animation builder |

Total: 956 bytes, 10/10 translated, 0 deferred.

## Size checks

`functions.tsv` reports `88/48/48/12/12/12/12/12/292/420` bytes. Every value was
re-checked against the prologue and the closing `blr` and matches the real
extent exactly, so no row is a prologue artefact:

| address | prologue | terminal `blr` | real extent |
|---|---|---|---|
| `0x80037D20` | `stwu r1,-0x10` | `0x80037D74` | 88 (`0x58`) |
| `0x80037E40` | `stwu r1,-0x10` | `0x80037E6C` | 48 (`0x30`) |
| `0x80037E70` | `stwu r1,-0x10` | `0x80037E9C` | 48 (`0x30`) |
| `0x80037EA0`..`0x80037EE0` | none (`lis`/`addi`) | `0x80037EA8`..`0x80037EE8` | 12 each |
| `0x80038080` | `stwu r1,-0x20` | `0x800381A0` | 292 (`0x124`) |
| `0x800381B0` | `stwu r1,-0x50` | `0x80038350` | 420 (`0x1A4`) |

The five 12-byte bodies are plain `lis r3,0x8037` / `addi r3,r3,0x7A04` / `blr`
accessors with four padding bytes after each; they are byte-identical to the
`0x80377A04` accessors already in `src/ui/hud.rs`, so they return the existing
`DAT_80377A04` constant.

## What the cluster is

The class is identified from shipped strings and from the one in-image caller of
`0x800381B0`, not from RTTI:

- **Caller.** `0x8003DBD0` (the `togezo` build body) calls
  `0x800381B0(this, "togezo", "g3d/togezo.brres", "togezo_shell", 0, 0.0f)`
  at `0x8003DBF0..0x8003DC08`, with `"togezo"` the inline `.sdata` string at
  `r13 - 0x7DB0` (`0x804278D0`), `"g3d/togezo.brres"` at `0x80306388`,
  `"togezo_shell"` at `0x8030639C` and `0.0f` at `r2 - 0x79C0`
  (`0x8042B6C0`).
- **Constructor.** `0x80037EF0` (not in `functions.tsv`, not claimed) installs
  the same `+0x60` vtable `0x80305950` and builds the same members
  (`+0x528` buffer, `+0x544` archive, `+0x548`/`+0x588` records, `+0x5C0`/`+0x5C4`
  second-animation pair, `+0x650` sub-object) that `0x80038080` tears down, so
  the two are the `togezo` base-scene constructor/destructor pair.
- **Release wrapper.** `0x80037D20` is the two-step pattern used throughout the
  file: `0x8015F800(this, 0)` releases, then `0x802B9280(this)` frees when
  `0 < flag`; the record pointer is returned. It has no `bl` caller in the image
  and is reached through a vtable.
- **Adjustor thunks.** `0x80037E40` and `0x80037E70` are the `+0x18` and `+0x24`
  instances of the compiler-generated
  `mr r5,r3` / `mr r3,r4` / `addi r12,r5,<offset>` / `bl 0x802DCBFC` shape; the
  sibling at `0x80037E10` (`+0x0C`, outside this lane) makes the family
  complete. Their bodies are 48 bytes each and the only difference is the
  `addi r12,r5,24` / `addi r12,r5,36` immediate.
- **Destructor.** `0x80038080` re-installs `0x80305950` at `+0x60`, walks the
  four-slot actor table with `0x8005F900(i)` (which forwards to
  `0x80162D00((&DAT_80354E10)[i])`) and detaches the scene from the actor whose
  `+0x2A78` owner word resolves to `this` via `0x8012E510(actor, this)`; then it
  re-vtables `+0x650` to the shared `0x80329958`, unlinks it with `0x8016D4B0`,
  releases it with `0x802D7B20(rec, 0)`, releases `+0x5C4` (`-1`), `+0x588`
  (`0`), `+0x548` (`-1`) and `+0x528` (`-1`), runs the base release
  `0x80095130(this, 0)` and frees through `0x80162920(this)` only when
  `0 < flag`.
- **Model builder.** `0x800381B0` is the same five-stage builder shape as
  `fn_80033310`/`fn_80035820`, with two extra inputs: the class id at `+0x08`
  selects the `"revival_shell"` clip only for ids `0x36`/`0x37`, and the fifth
  integer argument selects an optional second animation that is bound, started,
  bound to the `+0x548` record through its vtable slot `+0x18`, given the
  incoming `f1` as playback rate by `0x80166EF0` and reset to `0.0f` on the
  second track by `0x80166F10`.

## Ghidra artefacts handled in this lane

1. **Float argument order.** Ghidra prints
   `FUN_800381b0((double)*(float *)(unaff_r2 + -0x79c0), param_1, ...)` in the
   *caller* `0x8003DBD0`, putting the `f1` argument first. The disassembly
   (`lfs f1,-0x79c0(r2)` at `0x8003DBF0`, `bl 0x800381b0` at `0x8003DC08`)
   shows it is the sixth (float) argument; the body reads it with
   `fmr f31,f1` at `0x800381CC` and re-emits it with `fmr f1,f31` at
   `0x8003830C`.
2. **GQR prologue noise.** The `unaff_GQR0` / `ldexpf` block at the top of the
   `0x800381b0` export is Ghidra mis-decoding the `stfd r31`/`psq_st f31`
   register-save pair at `0x800381C0..0x800381C4`. It has no semantic content
   and is not modelled.
3. **Save/restore helpers.** `0x802DCD74`/`0x802DCDC0` are the compiler's
   `_savegpr`/`_restgpr` helpers; they only spill and reload the registers the
   prologue saves and are not modelled (same treatment as the earlier bodies in
   this file). Ghidra takes the `bl 0x802DCD74` return as producing `this`,
   which is why the export threads `iVar3` through it.
4. **`unaff_r13` / `unaff_r2`.** Re-read from the disassembly and recorded on
   the constant: `r13 - 0x5648` (`0x8042A038`, reused as `G_SDA_5648_D`) and
   `r2 - 0x7AD0` (`0x8042B5B0` `0.0`, new `F32_SDA2_ZERO_7AD0_D`).
5. **Dropped `r12` operand.** `0x802DCBFC` takes its call descriptor in `r12`;
   `crate::platform` has no helper that models `r12`, so `fn_80037e40` /
   `fn_80037e70` use `call3(stub, b, b, a)` and document the `+0x18` / `+0x24`
   descriptor offset in prose. This is the same convention as the already
   translated `fn_80016a20` / `fn_80016a50` (`+0x18` / `+0x0C`).
6. **Void-typed wrapper.** `0x8005F900` is exported as `void`, but its body is a
   tail call that leaves `0x80162D00`'s answer in `r3`; the destructor uses it
   as the actor pointer, so the body calls it through `platform::call1`.
7. **Reused constants.** `FN_8015F800`, `FN_802B9280`, `FN_802DCBFC`,
   `FN_8005F900`, `FN_80162D00_A`, `FN_8016D4B0`, `FN_802D7B20`,
   `FN_80168D80`, `FN_80169D20`, `FN_80069060`, `FN_80095130`, `FN_80162920`,
   `FN_800690C0`, `FN_800DF180`, `FN_80239E30`, `FN_80169D90`, `FN_80064BD0`,
   `FN_8023A0B0`, `FN_801650D0`, `FN_8023A200`, `FN_80166830`, `FN_80166BD0`,
   `FN_80166EF0`, `FN_800690E0`, `G_SDA_5648_D`, `DAT_80377C48`,
   `DAT_80377A04` and `DAT_80329958` already exist in `src/ui/hud.rs` / the
   `lyt` import list and are reused. New constants are lane-suffixed:
   `S_REVIVAL_SHELL_D`, `VT_TOGEZO_80305950_D`, `F32_SDA2_ZERO_7AD0_D`,
   `FN_8012E510_D`, `FN_80166A70_D`, `FN_80166F10_D`.

## Confidence notes

`high` for the five accessors (single literal `lis`/`addi` pair), for
`0x80037D20` (null test, literal `0` release argument, signed flag test) and for
`0x80038080` (every offset, loop bound, re-vtable, release flag and the signed
`0 < flag` free test is literal). `medium` for the two adjustor thunks, whose
callee `0x802DCBFC` has no recovered control flow, and for `0x800381B0`, whose
archive/model/clip lookup roles and second-track semantics are inferred from
call shapes and the caller's strings. No body is `low` and none is deferred.
