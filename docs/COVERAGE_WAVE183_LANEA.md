# Coverage - Wave 183, lane A (`game::bases`, sub-300-byte sweep)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 10 | 940 | 10 | 0 |

All ten lane-A addresses are 76-128 bytes in
`work/ghidra/rel-d_basesNP/export/functions.tsv`. Lane A added no helpers and
did not touch `src/game/bases.rs`; the block is in
`work/wave183/laneA_block.rs`, the ledger is
`ledger/agents/wave183-lanea.csv`, and the bodies are appended by the
coordinator.

## Translated

- `0x8068_4B10` (128 bytes) - four-stage virtual hand-off: class vtable at
  `+0x60` slots `+0x280` (receiver only), `+0x284` (`this`) and `+0xD4`
  (descriptor `0x8082_A8E8`), then the `+0x1C` slot of the embedded interface
  object at `+0x394`; returns `1`.
- `0x8068_5BA0` (88 bytes) - deleting destructor of the `0x18`-byte interface
  records: `0x802BE7C0(element, 0)`, then `0x802B9280(element)` when
  `free > 0`, returning `element`. It is the sibling of the record constructor
  [`fn_80685b50`] and byte-identical in shape to [`fn_80672be0`].
- `0x8068_61E0` (88 bytes) - returns `*out - (i8)table[variant] * 48.0`, where
  `variant = *(this + 0x348)` and the signed-byte table is `0x8042_C1A0`.
- `0x8068_7380` (120 bytes) - `0x14`-stride array-manager constructor: count at
  `+4`, `0x802B9270(count * 0x14, heap, 4)` result at `+8`, then vtable slot
  `+0x14` with `(this, heap)`; `heap == 0` selects the default `0x8042_AE10`.
- `0x8068_76D0` (88 bytes) - `0x8015F800(state, 0)` deleting destructor, with
  `0x802B9280(state)` when `free > 0`; returns `state`.
- `0x8068_B260` (88 bytes) - same `0x8015F800` deleting destructor.
- `0x8068_D2C0` (88 bytes) - writes `0.0` (`0x807C_9A70`) to `+0xE8`, `+0xEC`
  and `+0xF0`, dispatches class slot `+0x98`, then stores `1900.0`
  (`0x44ED8000` at `0x807C_9BB4`) at `+0xB4`.
- `0x8068_DCA0` (88 bytes) - same `0x8015F800` deleting destructor.
- `0x8068_EF50` (88 bytes) - same `0x8015F800` deleting destructor.
- `0x8068_F0D0` (76 bytes) - factory: `0x8016_28C0(0x398)`, then
  `0x80064110(object)` and class vtable `0x807E_F664` at `+0x60`; returns the
  object or `0`. Same template as [`fn_8064e5f0`].

## Deferred

None.

## Notes and limitations

- The four `0x8015F800` destructors (`0x8068_76D0`, `0xB260`, `0xDCA0`,
  `0xEF50`) are one template whose raw words differ only in the two `bl`
  displacements; they match the already-translated [`fn_806511d0`]. The class
  each belongs to is still `// unknown:`.
- `0x8068_61E0` is the standard `0x8042_C1A0` variant-table lookup already used
  by [`fn_806b5a80`]. The `lfd` pair is the compiler's signed-int-to-double
  magic: `CONCAT44(0x43300000, x ^ 0x80000000)` minus the double at
  `0x807C_98D0` (`0x43300000_80000000` = `2^52 + 2^31`) yields the signed char
  exactly, and the arithmetic is then `fsubs`/`fmuls` single precision. Ghidra
  types the return `double` only because of the `lfd`; the value leaves the
  function in `f1` as a single, so the Rust signature returns `f32`.
- `0x8068_7380` is the `0x14`-stride twin of [`fn_806a7f30`] (`4`-byte stride,
  same default-heap load and same `+0x14` vtable dispatch).
- `0x8068_4B10` and `0x8068_D2C0` dispatch through the class vtable stored at
  `+0x60`, which is exactly `actor_method0`/`actor_method1`. `0x8068_4B10`'s
  fourth call targets the object at `+0x394` whose vtable pointer is stored at
  that same address, so `vmethod0(this + 0x394, 0x1C)` matches the raw
  `lwz r12,0x394(r31)` + `lwz r12,0x1C(r12)` + `bcctr` sequence. The
  descriptor `0x8082_A8E8` lives past the file-backed end of the REL and is
  passed through as an address literal, never read.
- `0x8068_F0D0`'s constructor call appears in the Ghidra C with no arguments;
  the raw words leave `r3` holding the fresh allocation at the `bl 0x80064110`,
  so the translation passes the object explicitly (same as
  [`fn_8064e5f0`]). The class identity behind vtable `0x807E_F664` is
  `// unknown:`.
