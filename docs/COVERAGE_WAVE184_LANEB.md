# Coverage - Wave 184 lane B (`game::bases`)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane B block) | `game::bases` | 9 | 884 | 9 | 0 |

Nine of the ten claimed small bodies from the `0x806E`/`0x806F` part of
`d_basesNP` were translated. The lane is dominated by reused shapes: two
byte-identical matrix/refresh twins, two deleting destructors, one list purge,
two constructors that differ only by one vtable word, one child-actor spawn and
one matrix writer that continues the already translated `0x598`-byte lift
constructor. One body (`0x806E2BF0`) is deferred because the raw words contain
no store outside its own frame.

## What this pass covers

- **Model refresh twins (`0x806E5580`, `0x806E7350`).** `0x8006CED0` fills a
  three-float frame vector from the actor's world position, `0x801C0BD0` builds
  the matrix at `+0x7C`, `0x8016A170` copies it into the model at `+0x3F0`,
  `0x8016A150` installs the parent transform `+0xDC`, model virtual `+0x14`
  runs, and the body returns the literal `1`. Both bodies are byte-for-byte
  identical and are the same call shape as the already translated
  `fn_806c9b70` / `fn_806e2d00`, minus the angle setters. The three floats
  spilled to the frame before `0x801C0BD0` are that call's lost position
  arguments (the export prints only the matrix).
- **Lift motion record (`0x806E64D0`).** Runs `fn_806e60d0` (the three-phase
  range registration of the `0x598`-byte lift) and then copies the three-word
  table at `0x807FD564` into `+0x584..+0x58C`. The words were re-read from the
  REL: `0` , `0xFFFFFFFF`, `0x806E5F30`. The third is the per-frame matrix
  writer `fn_806e5f30`, so the 12-byte record is `{ 0, -1, update }`.
- **Deleting destructors (`0x806E9AC0`, `0x806F86C0`).** The
  `0x8015F800(object, 0)` + `0x802B9280(object)` wrapper already used by
  `fn_806511d0`. `0x806E9AC0` keeps one extra, redundant `beq` at
  `0x806E9AE4` and `0x806F86C0` has the single-guard form; the branch targets
  are the same in both.
- **Actor-list purge (`0x806F2560`).** Identical body to `fn_806f1ef0`:
  `0x801625B0` first node, `0x80162510` release, `0x801625D0` next, while
  non-null. It is adjacent to `fn_806f25b0`.
- **Constructors (`0x806F3010`, `0x806F3080`).** Base vtable `0x803295D8`, two
  cleared words, embedded sub-object construct `0x8016A5E0(this + 0x0C)`,
  `+0x28 = 0`, then the derived vtable `0x80329578` / `0x80329518`. The shape
  is already present inline in `fn_807531d0` and `fn_8070b0a0`. The final field
  is `+0x28`, so `0x2C` is the written extent, not a proven allocation size.
- **Child-actor spawn (`0x806F8050`).** `0x8016F130(+0x538, +0x52C)` copies the
  marker transform into the spawn matrix, `0x80064610(0x15F, +0x04, +0x52C, 0,
  0)` creates the child, the child's `+0x528` back-pointer is filled from the
  parent's first word when it is still zero, and the child's first word is
  returned (`0` on failure). Same `+0x538`/`+0x52C` transform pair as
  `fn_806f80d0`.

## Verification

- Every body was read from the raw words (`node ppc159.js`) as well as from the
  Ghidra export. The export was wrong or lossy in three places: `0x806E2BF0`
  (float loop rendered as a bare counter loop), `0x806E5580`/`0x806E7350`
  (`0x801C0BD0` printed with only its matrix argument), and `0x806E64D0`
  (callee printed without its `this`; the raw `mr r31,r3` shows `r3` survives,
  so `fn_806e60d0(this)` is passed).
- Constants re-read from `work/rels/elf/d_basesNP.elf`: `0x807DBBD4` was a
  mis-signed displacement in the local decoder - the real address is
  `0x807CBBD4 = 8.0`; and `0x807FD564/68/6C = 0 / 0xFFFFFFFF / 0x806E5F30`.
- Only existing helpers were used (`platform::read_u32/write_u32`,
  `abi_call1..5`, `call0_ret`, `call_f32x3_u32`, `vmethod0`, and the already
  translated `fn_806e60d0`). No new helper was introduced.
- `src/game/bases.rs` was not edited by this lane; `cargo check` finishes with
  exit 0 (warnings only, none from the new bodies).

## Deferred

- **`0x806E2BF0` (96 bytes).** The raw block is a loop over the four
  `lift_suisha` plank records `+0x430..+0x7B0` (stride `0xE0`): it loads
  `+0xCC`, `+0xD0`, `+0xD4`, `+0xD8`, adds the `8.0` at `0x807CBBD4` to the
  `+0xD0` and `+0xD8` words, and then throws the results away - there is **no
  store into any plank**. The only stores are six `stfs` into the function's own
  `0x50`-byte frame (`+0x0C..+0x44`), plus one dead `lfs f10, 0xB4(this)` per
  iteration. Ghidra's export shows only the `+= 0xE0` counter loop for exactly
  that reason. The `ori r0,r0,0` nop at `0x806E2C04`, inside the loop setup,
  suggests a post-link patch, but the surviving words do not say which field
  was meant to receive the `8.0` add. Emitting a no-op body would hide a
  probably patched store, so the body is left out until the intended store is
  established from another source.
