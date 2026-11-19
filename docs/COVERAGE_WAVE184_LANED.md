# Coverage - Wave 184, lane D (`game::bases`, sub-300-byte sweep)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 10 | 892 | 8 | 2 |

All ten lane-D addresses are 76-116 bytes in
`work/ghidra/rel-d_basesNP/export/functions.tsv`. Lane D added no helpers and
did not touch `src/game/bases.rs`; the block is in
`work/wave184/laneD_block.rs`, the ledger is
`ledger/agents/wave184-laned.csv`, and the bodies are appended by the
coordinator.

## Translated

- `0x8070_5B00` (116 bytes) - deleting destructor of the `0x80705xxx`
  height-follower actor: `0x80169D20(+0x3B4, -1)`, `0x80069060(+0x394, -1)`,
  `0x800642B0(this, 0)`, then `0x80162920(this)` for `free > 0`; returns `this`.
- `0x8070_6230` (80 bytes) - rebuilds the node vector from the signed step:
  `+0xE8 = 0.00067138671875 * (i16)+0x3FC`, `+0xEC`/`+0xF0` = `0.0`. This is the
  helper called at the top of [`fn_80706510`] and `0x80706690`.
- `0x8070_67F0` (84 bytes) - rebinds one 12-byte lane record: resolves the
  owner handle at `+0x4` through `0x80162D00`, copies `source + 0x0` into
  `record + 0x0`, re-reads the angle from `owner + 0x100` into `record + 0x8`,
  then calls `0x80706850(record)`.
- `0x8070_6BD0` (88 bytes) - `0x8015F800` deleting destructor
  (`0x802B9280` for `free > 0`), returning `this`.
- `0x8070_6D50` (76 bytes) - factory: `0x801628C0(0x3A0)`, `0x80064110(object)`,
  vtable `0x8080_481C` at `+0x60`; returns the object or `0`.
- `0x8070_8160` (88 bytes) - `0x800671B0(this, 0)` deleting destructor
  (`0x80162920` for `free > 0`), returning `this`.
- `0x8070_82D0` (88 bytes) - same `0x8015F800` deleting destructor.
- `0x8070_F0F0` (88 bytes) - same `0x8015F800` deleting destructor.
- `0x8071_69E0` (92 bytes) - sink clamp: `0x800955F0(actor)`, and while
  `+0x670 < +0xB0` lowers `+0xB0` to `+0x670` and dispatches state `0x8082FAE0`
  through class slot `+0xD4`.
- `0x8071_6A70` (92 bytes) - same clamp against the second height threshold
  `+0x690`, dispatching state `0x8082FEA0`.

## Deferred

None.

## Notes and limitations

- The three `0x8015F800` destructors (`0x8070_6BD0`, `0x8070_82D0`,
  `0x8070_F0F0`) are one template whose raw words differ only in the two `bl`
  displacements; they match the already-translated [`fn_806511d0`] and
  [`fn_8068b260`]. The class each one belongs to is still `// unknown:`.
- `0x8070_8160` is the same template with `0x800671B0` in place of
  `0x8015F800` and `0x80162920` in place of `0x802B9280`; `0x8070_5B00` is the
  four-call variant of the same family (it is the destructor of the actor whose
  per-frame bodies are [`fn_80706510`]/[`fn_80706690`]).
- `0x8070_6230`'s conversion is the compiler's signed-int-to-double magic pair
  `CONCAT44(0x43300000, x ^ 0x80000000)` minus the double at `0x807C_C6D8`
  (`0x43300000_80000000` = `2^52 + 2^31`), which recovers the sign-extended
  `lha 0x3FC` word exactly; the following `fsubs`/`fmuls` are single precision.
  It is written as `(i16)+0x3FC as f32` scaled by `0x807C_C6E0`
  (`0.00067138671875`). `0x807C_C6C4` is `0.0`, and both `stfs` for
  `+0xEC`/`+0xF0` precede the `fmuls`, so `+0xE8` is stored last.
- `0x8070_67F0` is reached by [`fn_80706410`] as a bare tail branch
  (`b 0x807067F0`) with its second argument left over from the caller; the
  translation keeps the two-parameter signature that the body itself uses. It
  calls `0x80706850`, which has no Rust body yet, through a plain address.
- `0x8070_6D50`'s constructor call appears in the Ghidra C with no arguments;
  the raw words leave `r3` holding the fresh allocation at `bl 0x80064110`, so
  the translation passes the object explicitly (same as [`fn_80651390`]). The
  class identity behind vtable `0x8080_481C` is `// unknown:`.
- `0x8071_69E0` is byte-identical to the already-translated [`fn_80716860`]
  (only the address differs), so the body is not inferred; both are the
  sink-direction mirror of [`fn_807166b0`]. `0x8071_6A70` uses the second armed
  height `+0x690` and state `0x8082FEA0` where [`fn_807168f0`] uses `+0x680`
  and `0x8082FDE0`, and it applies the sink `bc 4,1` test rather than that
  body's raise `bc 4,0`. Both are marked `medium` only because the receiver of
  `0x800955F0` is lost in the Ghidra C and is supplied from the fields the same
  body tests next; the raw `bl` still has `r3` = `actor`.
