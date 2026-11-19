# Wave 204 lane C coverage (`ui::hud`)

Ten bodies, all translated, no deferrals. Range `0x80026B50..0x800272C0`
(`main.dol` runtime addresses; 872 bytes of original code).

| Address | Size | Body | Confidence |
| --- | --- | --- | --- |
| `0x80026B50` | 48 | `fn_80026b50` - forward `(b, b, a+0x18)` to the descriptor trampoline `0x802DCBFC`. | high |
| `0x80026B80` | 48 | `fn_80026b80` - twin with the descriptor at `a+0x24`. | high |
| `0x80026BB0` | 12 | `fn_80026bb0` - tenth `0x80377A04` accessor. | high |
| `0x80026BC0` | 64 | `fn_80026bc0` - child gate: state `3` plus flag bit `0`, or stage id `0x87`/`0x85`. | high |
| `0x80026C00` | 152 | `fn_80026c00` - owner chain `+0x394` slot `+0x28`, slot `+0x14` against `0x80352DB4`; non-zero selects `+0x38D`, zero selects the `other` record's `+0x60` slot `+0x6C` byte. | medium |
| `0x80026FB0` | 92 | `fn_80026fb0` - record release: `0x8015F800(this, 0)` then optional `0x802B9280`. | high |
| `0x800270F0` | 224 | `fn_800270f0` - five `+0x30` links, probing each child's virtual `+0xC`; falls back to recursing on the fifth child. | high |
| `0x800271D0` | 136 | `fn_800271d0` - compare the `':'` tail of `name` against the object's `+0x20` name through `0x802E1AB4`. | high |
| `0x80027290` | 48 | `fn_80027290` - second `(b, b, a+0x18)` descriptor forward. | high |
| `0x800272C0` | 48 | `fn_800272c0` - second `(b, b, a+0x24)` descriptor forward. | high |

## Notes carried into the bodies

- **Band.** This run continues the `0x8002` band that wave 203 filled up to
  `0x80023F80`. All ten bodies read the player/controller record family the
  wave-203 lane C body `fn_80020f40` already uses: state byte `+0x38C`, target
  byte `+0x38D`, flags word `+0x4FC`, owner sub-object `+0x394` and the `+0x60`
  vtable with the digit/name getter at `+0x6C` and the name at `+0x20`.
- **Descriptor forwards.** The four 48-byte thunks put `a + 0x18` or
  `a + 0x24` in `r12`, not `a`. `0x802DCBFC` is a three-word descriptor
  trampoline (`r12[0]` is added to `r3`, `r12[1]` picks a vtable slot, `r12[2]`
  is the table base), so the descriptor address is the operand that must travel
  in `r12`. The earlier copies `fn_80023c00`/`fn_80023c30` in this module pass
  the bare object pointer even though their own documentation names the
  `+0x18`/`+0x24` selector; that is a fidelity gap in wave 203 lane F, not
  reproduced here.
- **Chain walk.** `0x800270F0` is byte-identical to `0x8001D850`, which
  `fn_8001d850` already translated. The earlier loop advances the chain five
  times and then recurses on the *sixth* node, while the original's fifth probe
  is followed by `lwz r3,0x30(r31)` / `bl` on the *fifth* child. The new body
  therefore runs four advancing probes plus an explicit fifth probe that
  recurses on the fifth child. `fn_8001d850` still carries the one-link-deep
  recursion argument and is worth the same fix.
- **Probe argument.** The `+0xC` virtual slot is entered with the child still
  in `r3` (the body loads the child into `r3` before loading its vtable), so the
  probe is issued as `call1(slot, p32(child))` rather than the argument-less
  `call0_r` used by `fn_8001d850`.
- **Unsigned stage compare.** `0x80026BC0` reads `+0x08` with `lhz` and
  compares with `cmpli`; the Ghidra export's `short *` is signed, the code is
  not, so `read_u16` is used.
- **Sign-extended result.** `0x80026C00` returns both candidate bytes through
  `extsb`, so the Rust signature is `i32` and the byte is cast through `i8`.
- **Unknown descriptor.** `0x80352DB4` has the same role as `0x80353A64` in
  `fn_80020f40` (descriptor for the `+0x14` virtual test) but its type and
  contents are not identified. It is kept as a named constant and not merged
  with `DAT_80353A64`.
- **Evidence gap.** The `main-gb` export set stops at `0x80026FB0`; the four
  bodies `0x800270F0`, `0x800271D0`, `0x80027290`, `0x800272C0` were verified
  from `work/ghidra/main/export/*` and from the `ppc159.js` disassembly of
  `work/dol/main.elf`.

## Scratch verification

The block was appended to a throw-away copy of `src/ui/hud.rs` at
`%TEMP%\opencode\wave204lanec` (with `Cargo.toml` and `src/` copied, no other
edits). `cargo check --lib` finishes with exit code `0` and the crate's
pre-existing 23 warnings; the appended block adds none. A deliberate
type-error probe appended after the block failed at
`src/ui/hud.rs:14589`, confirming the block really is compiled.
