# Wave 199 lane C coverage (`game::bases`)

Two bodies, both translated, no deferrals:

| Address | Size | Body |
| --- | --- | --- |
| `0x807B7B40` | 524 | `fn_807b7b40` - world-map per-frame mode switch; mode `0` runs the course-entry bookkeeping (screen/coin/star refresh, `0x800FB9E0` gate), modes `1`/other tail-call `fn_807b7f30`/`fn_807b7d60`. |
| `0x8079D3C0` | 3176 | `fn_8079d3c0` - the world-map actor state machine: 157-entry table dispatch at `0x8081BB00`, 40 real case blocks, everything else raises `+0x139`. |

## Notes carried into the bodies

- **The `0x8079D3C0` jump table is in the shipped data segment, not in BSS.**
  The dispatcher is `lis r4,0x8082` + `addi r4,r4,-17664`; the `addi` immediate
  is signed, so the base is `0x8081BB00`, *not* `0x8082BB00` (the +0xBB00
  reading, which lands in the zero-filled BSS section `0x80822900+0x9200` and
  has no relocations). `0x8081BB00` is inside the last file-backed data section
  (`0x807D0C00..0x808228B8`), so all 157 words are present and were read out in
  full. `state - 6 > 0x9C` takes the plain default branch, so `state < 6` also
  lands there.
- **Table contents.** `6`, `0x0C`, `0x26`, `0x41`, `0x57` map to `0x8079E010`
  (bare return); `0x24` (`0x8079D6AC`) and `0x5A` (`0x8079DC1C`) are separate
  blocks that only store `+0x139 = 1`, so they are folded into the same Rust
  arm as the `0x8079E008` default. All other mapped entries are translated
  verbatim, one arm per block, and `0x44`/`0x45`, `0x4B`/`0x4D`, `0x97`/`0x98`
  and `0x9F`/`0xA2` share an arm because their blocks are byte-equivalent.
- **`0x30` is an unsigned-halfword to float conversion.** `lis 0x4330` +
  `stw`, `lfd`, the double `2^52` at `0x807D0210` and `fsubs` are the MSVC/Wii
  idiom for `(float)(unsigned u16)`; the body writes `count as f32`. The other
  constants are `0x807D020C = 200.0` (the added base) and `0x807D0208 = 5.0`
  (the `0x25` distance limit), read out of the ELF.
- **`0x807D00C4 = 1.0`, `0x807D0160 = 0.0`, `0x807D016C = 0.01`.** The
  `+0xDC..+0xE4` colour triple is always one repeated float; `0x8016D3B4`
  (`0x8031D3B4`, `main.dol`) is the "full" colour and `0x807D016C` the 0.01
  fade step.
- **Dead stack stores were dropped.** The `0x3D`, `0x46`, `0x7A` and `0x30`
  blocks write the same float three times into `r1+0x08`/`r1+0x14`/`r1+0x2C`
  scratch slots that are never reloaded; only the `+0xDC..+0xE4` and
  `0x800F6410`/`0x800F9CD0` argument records survive.
- **`0x46` second stage.** The two `0x800F69F0` sites are identical: three
  `f32` scales (`0x8031D534`, and `0x8031D53C`/`0x8031D540` multiplied by the
  `0x8031D3B4` scale) followed by `this`, `this+0xAC`, `0`, the signed halfword
  `0x8031D538` and the callback `0x80377DE0`, so both use
  `call_f32x3_u32x5`.
- **`0x25` model burst.** `0x80C560B0`/`0x80C561E0`/`0x80C56B90`/`0x80C56600`
  are GX/scene helpers in `main.dol`-adjacent space; the three-float record built
  by `0x80100200` is copied field by field in the image and passed by address,
  which is modelled with a `[f32; 3]` local.
- **Callees with Rust bodies are called directly.** `fn_80799960`,
  `fn_8079b1c0`, `fn_8079b1d0`, `fn_8079b060`, `fn_8079b260`, `fn_8079b300`,
  `fn_8079b5b0`, `fn_8079afb0`, `fn_8079be80`, `fn_8079e1e0`, `fn_8079e310`,
  `fn_8079e4e0`, `fn_8079e680`, `fn_8079ee70`, `fn_8079ef10`, `fn_8079f040`,
  `fn_8079f320`, `fn_80799340`, `fn_807970a0`, `fn_807984f0` and
  `fn_80770aa0` already exist in this file and are used with their declared
  signatures. `fn_807987e0` takes no Rust argument because it recovers `r3`
  through the `0x802DCD74` save helper; the image sets `r3 = this` before the
  call and that is noted inline.
- **`0x807B7B40` mode `0`.** `0x807B7F30` and `0x807B7D60` are the already
  translated sibling arms; `fn_807b7d60` takes no parameter (its body overwrites
  `r3` with `lis r3,0x8043` before any read). `0x803741B0` is compared against
  `4` per slot and `0x807D0810` holds the three payload bytes `01 02 04`; both
  are read through `platform::read_*` so no literal is invented.
- **No body was deferred.** `0x8079D3C0` had the usual Ghidra argument drops
  (`func_0x8006cc00()`, `FUN_8079b1c0()`), but every call site's `r3..r7` was
  re-read from the raw words before choosing the helper, and no instruction in
  the reachable span was left unreproduced.

## Verification

- `work/wave199/laneC_block.rs` was appended to a scratch copy of the crate
  (`%TEMP%\opencode\scratch_laneC2`, a byte copy of `src/` + `Cargo.toml` +
  `Cargo.lock`); `cargo check` finishes with no errors and no new warnings from
  the two bodies. `src/game/bases.rs` itself was never edited by this lane.
- `node tools/check-lane-csv.mjs wave199` reports `OK` for
  `ledger/agents/wave199-lanec.csv`.
