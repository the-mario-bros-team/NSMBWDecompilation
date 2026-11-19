# Coverage - Wave 194 lane A (`game::bases`, 192-256 byte sweep)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Deferred |
|---|---|---:|---:|---:|---:|---:|
| `src/game/bases.rs` (lane A block) | `game::bases` | 10 | 2088 | 7 | 3 | 0 |

Ten bodies, all from the 192-256 byte band. Source of the bodies:
`work/wave194/laneA_block.rs`; per-function ledger:
`ledger/agents/wave194-lanea.csv`.

## What this pass covers

- **`+0x160` factory (1 body).** `0x807C5E00` (96) is the allocating twin of
  the in-place constructor family of the translated `fn_80764350` /
  `fn_8077b110`: `0x801628C0(0x160)`, base constructor `0x800F2710(this)`,
  vtable `0x80822858` at `+0x60`, `0x80069020(this + 0x138)`, then
  `0x800D6CC0(this + 0x154, 0)` - the only argument that differs from the
  `0x8016A2F0` member used by those two classes. Null allocation short-circuits.
- **Wendy-ride rail init (1 body).** `0x8071E060` (204) is the initialiser of
  the actor whose per-frame body is the translated `fn_8071e130`: it publishes
  `f32@0x8082FFE4 + f32@0x80809628 + 216.0` into `+0xB0`, copies the rail limit
  `0x8080964C` (`800`) into `+0x1AC98`, plays cue `0x554` through the same
  `0x8006A3F0`/`0x80199390` block that `play_worldmap_sound` models, and sets
  `+0x1A488 = -1`.
- **Model-node placement (1 body).** `0x8071EEE0` (204) type-tests the
  `+0x3B8` model against `0x802F8FF0` (virtual `+0x08`), builds the scoped
  `0x8025C660(adapter, model, 1)` scene adapter, resolves its pane through
  `0x8025C470(adapter, 0)`, builds `0x801C0C60(x, y, z, transform)` from the
  caller's three floats and applies it with `0x8023CFA0(pane, 0, transform)`.
  It never resolves the draw node, so nothing is made visible.
- **`+0x30` child-chain walks (3 bodies).** `0x807203D0`, `0x80722700`,
  `0x80723A30` (224 each) are byte-identical copies (same SHA-1 over all 224
  bytes) of the walk already translated as `fn_806990b0` / `fn_80659260` /
  `fn_806535e0`: five unrolled `child = object->+0x30` levels, stop when the
  child's virtual `+0xC` answers non-zero and return the parent, else a
  self-recursive tail on the advanced node.
- **Polar offsets (2 bodies).** `0x8072CE80` (256) rotates the `+0x4A30` /
  `+0x4A34` pair by `+0x104 * 0.00390625` and publishes
  `+0x4A1C`/`+0x4A20`/`+0x4A24`/`+0x4A28`; `0x80730360` (212) resolves a parent
  through `0x80162D00([+0x4E8])`, copies `parent[+0x104] + [+0x4F0]` into
  `+0x104`, and places the part at radius `+0x4EC` around the parent using
  `parent[+0x104] + [+0x4F2]`. Both feed the `0x80237C40`/`0x80237BD0`
  sine/cosine pair with a GQR3 unsigned-16-bit sample (`sth` + `psq_l ...,1,3`).
- **Tilt ramp + audio frame (2 bodies).** `0x80734240` (224) is the
  `+0x63C`-delayed tilt ramp of the same actor family as the translated
  `0x80733A40` (`+0x644`/`+0x646`/`+0x634`/`+0x348`), with the step table
  `s16[4] @0x807CD880 = {512, -512, -512, 512}`, clamp `+-9727`, reset table
  `s16[4] @0x807CD888 = {9728, -9728, -9728, 9728}` and message `0x80830EE0` on
  overflow. `0x80734B00` (220) is the per-frame emitter: `+0x394` virtual
  `+0x10`, `fn_80734e30(this)`, `+0x104 +-= +0x51C` (sign chosen by the
  `+0x348` byte), `0x8007FDA0(this + 0x430)`, then for each of the four
  `+0x524` outputs that is not `0.0`, `0x8005F900(i)` and
  `0x80057820(value, i)`. Returns `1`.

## Reuse and helper notes

- No new helpers were introduced. The block uses only `platform::read_u8` /
  `read_u16` / `read_u32` / `read_i32` / `read_f32` / `write_u16` / `write_u32`
  / `write_i32` / `write_f32`, `vmethod0`, `vmethod1`, `abi_call1`,
  `abi_call2`, `abi_call3`, `call_f32x3_u32`, `call_f32_u32`, `f32_of_f32` and
  the existing private `play_worldmap_sound`, all already present in
  `src/game/bases.rs`.
- `0x80734B00` calls the already-translated `fn_80734e30(this)` directly rather
  than through `abi_call1!`; the signatures agree (`fn_80734e30(this: usize)`).
- The three `+0x30` walks reuse the exact loop shape of `fn_806990b0`, and
  `0x80722700` / `0x80723A30` recurse into themselves (their `bl` displacement
  is the same as `0x807203D0`'s because the instruction streams are identical).
- The GQR3 loads are modelled as `read_u16` (type 5 = unsigned 16-bit sample);
  per the brief's decoding facts this is `value as u16 as f32` and needs no
  shift.
- Ghidra's "removing unreachable block" warnings in `0x8072CE80`,
  `0x80730360` and `0x80734B00` are all the `psq_l`/`psq_st` GQR guards and
  duplicated epilogue stores; none hides a branch. There is no conditional
  branch at all in `0x8072CE80` or `0x80730360`.

## Verification

- `work/wave194/laneA_block.rs` was appended to a scratch copy of the crate;
  `cargo check --offline` passes with no errors, and
  `node tools/doc-lint.mjs <scratch>/src` reports
  `{"ok": true, "files": 68, "violations": 0}`.
- Every raw word was re-read from `work/rels/elf/d_basesNP.elf`; each body's
  byte size matches `functions.tsv` (96, 204, 204, 224, 224, 224, 256, 212, 224,
  220 = 2088).
- Constants were read directly: `0x807CD530 = 0x807CD644 = 0.00390625`,
  `0x807CD534 = 0x807CD8D0 = 0.0`, `0x80809628 = 0.0`,
  `0x8080963C = 216.0`, `0x8080964C = 800`, `0x807CD880 = {512, -512, -512,
  512}`, `0x807CD888 = {9728, -9728, -9728, 9728}`. `0x8082FFE4` (the rail base
  height) lives in the REL's zero-fill segment and is kept as an address.
- All ten evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`; the CSV targets `src/game/bases.rs`.
- The three `medium` rows are `0x8071EEE0` (scene-adapter/pane role and the
  68-byte adapter buffer are `// inferred:` from the translated
  `fn_806d9870`), `0x8072CE80` and `0x80730360` (arithmetic literal, but the
  sine/cosine identity and the polar-placement reading are inferred). Nothing
  in those three is guessed at the instruction level.

## Deferred

None. All ten bodies were proven from the raw instruction stream.
