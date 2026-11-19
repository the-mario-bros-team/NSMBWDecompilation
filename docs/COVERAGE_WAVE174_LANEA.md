# Coverage - Wave 174 lane A (`game::bases`)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (pending merge of `work/wave174/laneA_block.rs`) | `game::bases` | 6 | 2,228 | 5 | 1 |

All six lane-A bodies (five 368-byte bodies plus the 388-byte `0x80740C90`)
were translated. Nothing was deferred.

## What this pass covers

- **Lane report pass (1 body).** `0x806377A0` walks the four `0x3C`-byte lane
  records, builds the same sixteen-word transform as [`fn_80637910`] for each
  empty one (`+0x90`/`+0xA0` into words 0/1, `0.0` into word 2, `+0x84..+0xB0`
  into words 4..16, word 3 uninitialised) and spawns `Wm_2d_result` through the
  `+0x208 + lane*0x128` interface slot `+0xA0`. It differs from the sibling in
  two places only: the record table is `+0x7AC` (sibling: `+0x6E8`) and the tag
  is clamped to `0..3` (sibling: permuted through `5/6/8/7`).
- **Cursor-grid outline (1 body).** `0x8064DEF0` allocates
  `count*8 + 0x10` bytes with `0x802B9250`, constructs the array through
  `0x802DC7E0` (stride `8`, ctor `0x8064D780`, dtor `0x80601D10`), stores it at
  `+0x410` and emits the four edges of the cursor grid from the `+0x404`/`+0x408`
  extents that [`fn_8064de60`] wrote, stepped by the `0x807C8678` table.
- **Rotating-arm socket update (1 body).** `0x80731780` gates on the `+0xA04`
  mode byte and on the selected arm angle (`+0x9D8 + index*2` equal to `0` when
  `+0x9CC == 0`, to `0x8000` otherwise), rotates the arm quarter turn, scales by
  `1/256`, and moves `+0x978`/`+0x97C` by `40.0 * cos` / `40.0 * sin` before
  calling the player lookup `0x8005F900` and the arm binder `0x80050D40` with
  the `+0x4D8 + index*0xE4` record.
- **Two-layer id pair emitter (1 body).** `0x8075BD20` runs `0x8075BB60` twice
  (kind `0` then `1`, ids `0x807CEBF0`/`0x807CEBF4`), clears the `0x80814560`
  pending byte, queues cue `0x28E`, and per non-negative id writes the name
  cache (`0x800BBDD0`), the decimal label (`0x800EF570`, `digit - '0'`),
  projects the point (`0x80100200`) and posts it (`0x80105240`, type `0x58`).
- **`koopaJr` model + six clips (1 body).** `0x8076D680` builds the `+0x188`
  accessor, loads `g3d/koopaJr.brres` (`koopaJr`), resolves the model, creates
  the `+0x1A8` animation object and binds the six clips from the relocated
  table `0x80816A50` (`wait`, `run`, `jump_st`, `jumpA`, `jump_ed`,
  `shock_wmap`) into the `+0x1E8` slots, clears bit 8 of the `mask` node word
  `+0x14`, then installs `character_SV` / `g3d/model.brres`.
- **Marker-slot harvest (1 body).** `0x80740C90` shifts the four `+0x394`
  slot records (stride `0x10`: index, `+0x04` live byte, `+0x08`/`+0x0C` floats)
  and the `+0x3D8` latch, then walks the `+0x4A0` list and refreshes the slot
  named by each live object's type virtual (`+0x60` vtable slot `+0x6C`).

## Verification notes

- `0x806377A0`: the tag clamp was re-decoded from the raw words.
  `0x7C6000F8` is `nor` with the destination in the `rA` field (bits 11..15),
  i.e. `nor r0,r3,r0`, not `nor r3,r0,r0`; with `subfic r0,r4,2` and
  `srawi r3,r0,31` that is exactly Ghidra's `~(tag-2 | 2-tag) >> 31`, so
  tag `2` maps to index `2` and every other value to `3`. The same stack word
  is reused for words 0/1 (`r1+0x24`, `r1+0x34`), so the transform is one
  sixteen-word block, matching [`fn_80637910`].
- `0x8064DEF0`: the A-form `fadds`/`fsubs` third operand was re-checked with the
  fixed decoder - `f0 = record + offset` uses the pre-increment `f2`, then
  `f2 += f3`; `f3` (loaded by `lfsx` at `0x7C65042E`) is the `0x807C8678` step
  and `f2` the running offset seeded from `0x807C86A8` (`0.0f`). The four loop
  bounds recharge from memory but nothing writes `+0x404`/`+0x408`, so they are
  read once.
- `0x80731780`: `0x807CD740` is `0.00390625` (`1/256`) and `0x807CD728` is
  `40.0`; both `psq_l ...,qr3,W=1` loads are `(u16) as f32` under
  `GQR3 = 0x00050005`, the same idiom as [`fn_807318f0`]. The `+0xA04` byte is
  `extsb`-ed before `0x8005F900`.
- `0x8075BD20`: `0x807CEBF0`/`0x807CEBF4` are `0`/`1`; `0x80814560` is read as
  a byte; the inner `blt` test is the signed `id >= 0` guard. Callee roles
  (`0x800BBDD0`, `0x800EF570`, `0x80100200`) are unknown and stay `// unknown:`
  comments; only the argument registers are transcribed.
- `0x8076D680`: this is the six-clip twin of the already-translated
  [`fn_807a1610`]; the string addresses are read out of the `0x808169D8`
  relocated block (`koopaJr`, `g3d/koopaJr.brres`, `character_SV`,
  `g3d/model.brres`) and the clip table at `0x80816A50`, and `0x807CF1A0` is a
  six-byte zero run. `0x807CF230` is `0.0f`.
- `0x80740C90`: the second parameter (`r4`, Ghidra's `uVar9`) is the caller's
  four `0xC`-byte output records; the export prints no argument list because
  `0x802DCD70` is a compiler save fragment. `0x807CE08C` is `0.0f` and the
  `rlwinm ...,0,24,22` mask is `0xFFFFFEFF`. `+0x60` slot `+0x6C` is read with
  `actor_method0`, the same virtual used elsewhere for the character/type byte.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806377A0` | `fn_806377a0` | 368 | high |
| `0x8064DEF0` | `fn_8064def0` | 368 | high |
| `0x80731780` | `fn_80731780` | 368 | high |
| `0x8075BD20` | `fn_8075bd20` | 368 | medium |
| `0x8076D680` | `fn_8076d680` | 368 | high |
| `0x80740C90` | `fn_80740c90` | 388 | high |

## Deferred

None.

## Reused helpers / neighbours

- [`fn_80637910`] - transform word layout, `0x807C7EB8` zero word, `0x800B37E0`.
- [`fn_8064de60`] - same `+0x404`/`+0x408`/`+0x40C` counts and `0x807C8678`.
- [`fn_807318f0`] - `GQR3` `(u16) as f32` angle scaling and the `1/256` table.
- [`fn_807a1610`] - BRRES/`0x80169D90`/`0x801650D0`/`0x801680E0` clip pipeline.
- [`fn_80741910`] - the `+0x394` `0x10`-byte slot record layout.
- New helpers were not needed; `ptr_at`, `f32_of_f32`, `call_f32_u32` and
  `actor_method0` already exist in `src/game/bases.rs`.
