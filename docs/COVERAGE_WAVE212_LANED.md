# Wave 212 lane D coverage (`game::player`)

Slice: the ten `game::player` bodies at `0x80067B40`, `0x80067C80`,
`0x80067D60`, `0x80067E40`, `0x80067F20`, `0x80068070`, `0x800680A0`,
`0x80068290`, `0x800682F0` and `0x80068310`. All ten are translated; nothing was
deferred.

Bodies live in `work/wave212/laneD_block.rs` and are appended to
`src/game/player.rs` by the merge step; ledger rows are in
`ledger/agents/wave212-laneD.csv`.

## Translated

| Address | Bytes | Rust body | Summary |
| --- | --- | --- | --- |
| `0x80067B40` | 92 | `fn_80067b40` | Null- and count-guarded handle release, the twin of `fn_80061d50`: `FUN_8015F800(handle, 0)` when the handle is non-null, then `0x802B9280(handle)` when `count > 0`, returning `handle` unchanged. The image carries a dead duplicate `cmpi cr0,3,r3,0` at `0x80067B64`, reproduced as the nested `if`. |
| `0x80067C80` | 224 | `fn_80067c80` | Five-deep `+0x30` chain walker: each stage calls `(*(u32 *)child)[0xC](child)` where `child = *(parent + 0x30)`; a non-zero result returns the current parent, and five zero results recurse into `0x80067C80` itself on the sixth link. Twin of `fn_80061ef0`. |
| `0x80067D60` | 220 | `fn_80067d60` | The same five-stage walk, but the sixth link goes to `0x80067E40` (the other copy of the walker) and the returned node is handed to `0x8015F930`, which reads `node + 0x08`. Ghidra types it `void`; the residue in `r3` is documented, not invented as a return. |
| `0x80067E40` | 224 | `fn_80067e40` | Byte-identical copy of the self-recursive walker apart from its own `bl` displacement: it recurses into `0x80067E40`. Kept separate because `0x80067D60` really calls this copy. |
| `0x80067F20` | 136 | `fn_80067f20` | Name-suffix comparator: `strrchr(name, ':') + 1` against `strrchr(vtable[0x20](object), ':') + 1`, result `strcmp(...) == 0`. Twin of `fn_80061fd0`. |
| `0x80068070` | 44 | `fn_80068070` | Seven-register predicate, `r3` dead: `a == b && d >= e && c <= f` returns `1`, else `0`. Called by `0x80068850` with two object layer keys and two interval pairs, i.e. a same-layer AABB overlap test. |
| `0x800680A0` | 4 | `fn_800680a0` | Literal thunk: the single word `0x4BFFFFD0` (`b 0x80068070`) and zero padding. No prologue, no `blr`, all seven argument registers forwarded. |
| `0x80068290` | 84 | `fn_80068290` | `u16` membership scan over the image list at `0x802EF388` = `{0x00A3, 0x00A2, 0x00A5, 0x00A4, 0xFFFF}`; `1` when the stopped-on entry is not the `0xFFFF` terminator. `r3` is dead. |
| `0x800682F0` | 24 | `fn_800682f0` | Five instructions then `b 0x800661C0`: forwards `(*(u32 *)0x80429D30, block, (*(u32 *)(block + 8)) >> 24, param_3, param_4)` to the record-list matcher `fn_800661c0`. |
| `0x80068310` | 16 | `fn_80068310` | `clz(value >> 24) >> 5`: the "top byte is zero" test. `r3` is dead; the three-instruction sequence ends in the only `blr` in the body. |

## Sizes and extents

All ten `# Original` blocks keep the canonical `main-gb` `functions.tsv` value.

- `0x800680A0` is exactly 4 bytes: the word at `0x800680A0` is `0x4BFFFFD0`
  and the next word is zero padding. That is the whole function.
- `0x800682F0` is exactly 24 bytes of instructions
  (`0x800682F0..0x80068307`). The last instruction is a tail branch, so there
  is no `blr` in this body.
- `0x80068310` is 16 bytes in the table but the `blr` is at `0x8006831C`, so
  the real instruction extent is 12 bytes plus the 4-byte alignment pad.
- The remaining seven end on a `blr` four bytes before the table boundary, so
  the real extent is the table size minus that pad: `0x80067B40` 88,
  `0x80067C80` 220, `0x80067D60` 216, `0x80067E40` 220, `0x80067F20` 132,
  `0x80068070` 40 and `0x80068290` 80 bytes.

## Notes on the raw words

- The five-stage walkers share one shape: each stage executes `lwz r31,0x30(r31)`
  then `lwz r3,0x30(r31)` and a `lwz r12,0x0(r3)` / `lwz r12,0xC(r12)` /
  `mtspr CTR` / `bcctr` dispatch, i.e. the child's vtable is the child's first
  word, and the tested slot is offset `0xC`. The non-zero arms of stages 2..5
  jump to `0x80067D40` and stage 1 to `0x80067D48`; both blocks are only
  `mr r3,r31` plus a jump/fall into the shared epilogue, so every non-zero stage
  returns its current parent.
- `0x80068290` was decoded from the `lhzx`/`rlwinm r5,r7,1,15,30` pair: the
  index is scaled by two, and both sides are unsigned halfwords. The list
  contents were read from `work/dol/main.elf` at `0x802EF388`.
- `0x800682F0`'s `rlwinm r5,r0,8,24,31` is a rotate-left-by-8 + low-byte mask,
  which selects the input's top byte; `>> 24` is the exact equivalent. The SDA
  bias `r13 - 0x5950` resolves to `0x80429D30` with the module's
  `r13 = 0x8042F680`; `src/game/bases.rs` shows the allocator publishing the
  `0x300`-byte block there (also aliased at `0x8042A1DC`).
- `0x80068310`'s third instruction `rlwinm r3,r0,27,5,31` is rotate-right-by-5
  with the low five bits masked off, i.e. `cntlzw >> 5`. `cntlzw(0) = 32` yields
  `1`; any non-zero top byte has `cntlzw <= 24` and yields `0`.
- `0x80068070`'s three relations are `cmp cr0,r4,r5` + `bc 4,2` (signed
  inequality), `cmp cr0,r7,r8` + `bclr 12,0` (LT) and `cmp cr0,r6,r9` +
  `bclr 12,1` (GT). `li r3,0` at `0x8006807C` is the shared false arm and
  `li r3,1` at `0x8006808C` the true arm.

## Reused neighbours

- Existing module constants: `FN_8015F800_D` and `FN_802B9280_D` (the release
  pair, wave 210 lane D), `FN_802E1C40_D` (`strrchr`) and `FN_802E1AB4_D`
  (`strcmp`). Existing body `fn_800661c0` (the record-list matcher, wave 211
  lane F) is called directly.
- New constants, all carrying the `_D` lane suffix: `FN_8015F930_D`,
  `DAT_802EF388_D` and `OWNER_80429D30_D`.
- Helpers: `platform::{read_u16, read_u32, call1, call2}`. No helper or
  constant already in `src/game/player.rs` was redefined.

## Verification

Scratch copy at `%TEMP%\opencode\w212d` (`src/`, `Cargo.toml`, `Cargo.lock`)
with this block appended to `src/game/player.rs`: `cargo check --lib` exits `0`
with no diagnostics in the appended lines.

## Deferred

None.
