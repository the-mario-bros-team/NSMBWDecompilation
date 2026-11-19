# Coverage - Wave 178, lane A (`game::bases`, 460-byte sub-band)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 4 | 1,840 | 4 | 0 |

All four lane-A addresses are exactly 460 bytes in
`work/ghidra/rel-d_basesNP/export/functions.tsv`; each body's last word before
the next function is a `blr` plus a four-byte alignment word, which the
translation does not model.

## Translated

- `0x8061_0510` - date-file screen pad refresh. Polls the two save-manager
  records from `0x800E0380`/`0x800E03B0` with the sign-extended file index at
  `+0xE44`, publishes their `+2` bit 0 into `+0xE4A`/`+0xE4B`, then drives
  animation `5`, the `[+0x1F8]+0xC8` channel float, animation `6`, five pane
  visibility bits and text rows `0x32`/`0x11`, ending in [`fn_80610320`]. A
  previously unclaimed sibling of the translated wave-16 `fn_8060fce0`
  family; the `0x802DCD6C` prologue is a register-save thunk, so the incoming
  `r3` is `screen`.
- `0x8061_DC70` - window message writer. Ids `5..=12` are formatted as
  `id - 4` into a 16-byte buffer and stamped with the six-argument
  `0x800C9A60` form; every other id uses the five-argument form with marker
  `0`, `0x130` (19) or `0x131` (20). The row table at `0x807C_6CA8` was read out
  of the REL (`1..20` -> `0x46,0x47,0x48,0x49,0x4A x8,0x4B..0x4F,0x38,0`).
  Ids above `20` leave the marker uninitialised in the original (Ghidra prints
  `unaff_r28`); the translation models that as `0` and says so in the doc.
- `0x8070_FE40` - witchcraft-emitter transform rebuild plus two model-channel
  patches, called from the translated `fn_8070fab0`. Constants verified in the
  REL: `0x807CCA78 = 1.0`, `0x80806628 = 127`, `0x80806629 = 255`,
  `0x807CCA98 = 2^52`. `0x8082F770` falls in the REL's zero-filled BSS segment
  (`vaddr 80822900 + 0x12484`, `filesz 0`), so it is read at runtime like
  `fn_80719250` does with `0x8080_8438`.
- `0x807A_BF50` - pane rectangle/scale initialiser. Word-for-word the
  translated `fn_807b08b0` with every member offset shifted by `+0x38`
  (`0xD4 -> 0x10C`, `0x134 -> 0x16C`, `0x198 -> 0x1D0`, `0x1BC -> 0x1F4`) and
  the analogous constants (`0x807D0738 = 0.0`, `0x807D0740 = 1.0`,
  `0x807D074C = 6000.0`; the sibling uses `0x807D07C0/C8/D4` with the same
  values). One fidelity fix over the sibling: `r3` for `0x800B54B0` is the
  `[0x8042A480]` screen object, so the translation passes it explicitly.

## Deferred

None.

## Notes and limitations

- No existing helper was added; every call target is either an `abi_callN!`
  thunk or one of the file's `call_f32x3_u32`/`call_f32x4_u32`/`call_f32_i32`
  wrappers.
- `0x800E0380`/`0x800E03B0` in `fn_80610510` and the `+2` status-bit roles of
  their records are `// inferred:` from the pad argument of `fn_80610320`; the
  offsets, masks and branch polarities are literal.
- `0x807ABF50`'s class link (which pane owner) is not provable from the REL
  alone; the body is reproduced and the offsets match the `fn_807b08b0`
  layout, so the translation is literal regardless of the owner's name.
