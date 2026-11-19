# Wave 222, lane A - `src/resource/archive.rs`, addresses `0x802CE6B0..0x802CE9AC`

Five entries of `work/ghidra/main-gb/export/functions.tsv`, all in the
`0x802C0000` run of the `resource::archive` ledger module. All five are
translated; none deferred.

| address | tsv size | real extent | body |
|---|---|---|---|
| `0x802CE6B0` | 52 | 52 (`blr` at `0x802CE6E0`, 12 B padding) | mask to packed word, baseline `0x02` |
| `0x802CE6F0` | 76 | 76 (`blr` at `0x802CE738`, 4 B padding) | same decoder with a one-byte out flag |
| `0x802CE740` | 92 | 92 (`blr` at `0x802CE798`, 4 B padding) | initialise the `0x80350444` record |
| `0x802CE850` | 8 | 8 (tail branch to `0x802C2360`, no `blr`) | set mode `0xB` |
| `0x802CE940` | 108 | 108 (`blr` at `0x802CE9A8`, 4 B padding) | tev-order/channel state ladder |

Bytes translated: 336.

No `functions.tsv` size in this lane needed correcting: every size is
`end - start` where `end` is the closing tail branch or `blr + 4`. The three
`blr`-ended bodies with a trailing pad word (`0x802CE6F0`, `0x802CE740`,
`0x802CE940`) are counted including that pad by the tsv, which matches
`next_function_start - start`.

## What the run is

These five are the leaf and constructor half of the `0x80350444` draw-record
class whose remaining virtual slots begin at `0x802CE7A0`:

* `0x802CE7A0` - vtable `+0x0C` slot: scales `+0x1C`/`+0x20` by two `f32`
  arguments and re-dispatches `+0x10`/`+0x18`, then calls `0x802CE850`;
* `0x802CE840` - vtable `+0x10` slot;
* `0x802CE8F0` - vtable `+0x14` slot: binds `+0x04`/`+0x08` and `+0x0C`/
  `+0x10` through `0x802C0B60`;
* `0x802CE860` - vtable `+0x18` slot: builds a 3x4 matrix at `r1+8` through
  `0x801C0C60`, fills two translation columns from its `f32` arguments, and
  submits it with `0x801C9940`/`0x801C95E0`.

The vtable itself (`0x80350444`) holds null at `+0x00`/`+0x04`, then
`0x802C3570`, `0x802CE7A0`, `0x802CE840`, `0x802CE8F0` and `0x802CE860`.
`// inferred:` this is the RLYT/screen draw record that the wave-221 lane A
window band and the wave-105/219 GX helpers feed; the exports prove only the
offsets, constants and callees.

**`0x802CE740`, the constructor.** Five zero words at `+0x00..+0x10`, `0.0f` at
`+0x14`/`+0x18`/`+0x24`, `1.0f` at `+0x1C`/`+0x20`, and the vtable at `+0x28`.
The two floats come from the `sdata2` slots `r2 - 0x3BC0` (`0x8042F4C0`,
`0.0f`) and `r2 - 0x3BBC` (`0x8042F4C4`, `1.0f`); the body reads them instead of
folding the literals, which is why the block defines
`SDA2_F32_3BC0_A222`/`SDA2_F32_3BBC_A222`.

Two whole-frame details are worth recording for the next lane that touches this
record:

1. The disassembler prints both `lfs f0,r2-0x3BBC` and `stw r0,...` as `"r0"`,
   but the two instructions touch different register files. The `stw` stores
   write the GPR zero from `li r0,0`; only the four `stfs` stores see the
   `sdata2` floats. Reading the `stw` group as "1.0" is the obvious trap.
2. The four `stw r0,0x08/0x0C/0x10/0x14(r1)` stores write into the callee's own
   32-byte frame. Nothing reads those slots and no call intervenes, so they are
   dead stores from an optimised-out local and are not reproduced.

**`0x802CE6B0`/`0x802CE6F0`, the flag decoders.** Both read only mask bits
`0x02`, `0x04` and `0x08` and fold them into a packed word. `0x802CE6B0` seeds
`0x02` and returns `0x06` for bit `0x02`, `0x08` for bit `0x04`, `0x0E` for bit
`0x08` (overriding both). `0x802CE6F0` seeds `0x01` and returns `0x11` for bit
`0x02`, `0x08` for bit `0x04`, `0x19` for bit `0x08`; it additionally writes a
single byte through its second argument - `0` when bit `0x02` or `0x08` was
set, otherwise `1`. The caller `0x802CE010` stores `0x802CE6B0`'s result as one
word and passes it by pointer as the fourth argument of a `+0x0C` slot with
channel `6`/`7`, and feeds `0x802CE6F0`'s word to `0x8024E180`; `// inferred:`
the two words are packed GX comparison/blend descriptors, but the export proves
only the bit arithmetic.

**`0x802CE850`, the mode setter.** The eight bytes are `li r3,11;
b 0x802C2360`. `r3` is overwritten, so the argument the callers
(`0x802CF050`, `0x802D0030`, and `0x802CE7A0` from the vtable slot) leave in
`r3` is discarded and the translated body takes no parameters. The tail callee
`fn_802c2360` is already translated in this module and is called directly
instead of being re-declared as a constant.

**`0x802CE940`, the state ladder.** `0x801C6AC0(0)`, then the seven-argument
`0x801C6AF0` twice with `(4,0,0,0,0,1,2)` and `(5,0,0,0,0,1,2)`, then
`0x801C5860(2)`. `0x802CF050` runs this before `0x802CE9B0`, `0x802CE9C0` and
`0x802CE850` and before its tint/draw loop, so it is the per-object render-state
preamble. `// inferred:` stage-count, tev-order and channel-count setters; the
same three addresses are called with the same argument shapes by `0x802C1D10`
and the draw-table bodies in this module.

## Helper and constant reuse

The plain `FN_801C6AC0`, `FN_801C6AF0` and `FN_801C5860` consts and the
translated `fn_802c2360` already exist in `src/resource/archive.rs` and were
reused unchanged. The only new constants are `VTABLE_80350444_A222`,
`SDA2_F32_3BC0_A222` and `SDA2_F32_3BBC_A222`, all suffixed `_A222` per the
wave-222 instructions.

## Verification

Scratch copy of `src/`, `Cargo.toml` and `Cargo.lock` with this lane's block
appended to `src/resource/archive.rs`:

```
cargo check --lib
Finished `dev` profile [unoptimized + debuginfo] target(s)
exit=0
```

26 pre-existing warnings, none from this block.
