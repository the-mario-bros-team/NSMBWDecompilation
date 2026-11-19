# Wave 190, lane C coverage (`game::bases`)

Lane band `0x80779EF0..0x8078D710`, all inside REL `d_basesNP`. Ten of ten
requested bodies translated; none deferred.

## Bodies

| Address | Bytes | Function | Summary |
| --- | --- | --- | --- |
| `0x80779EF0` | 160 | `fn_80779ef0` | Re-arms the `+0x1B8` effect slot (`1.0` then `0.0`), runs `0x8077A000`/`fn_80779f90`, and for variant `3` clears `+0x1AC` and sets `+0x124` from `0x800FAE40(5, 0x17)`. |
| `0x8077A110` | 148 | `fn_8077a110` | Constructor: base `0x800F5FD0`, vtable `0x808180B8` at `+0x60`, sub-objects `+0x18C`/`+0x1AC`/`+0x1C4`, vtables `0x803295D8` then `0x80329548` at `+0x1B8`. |
| `0x8077AB80` | 176 | `fn_8077ab80` | Rebuilds the `+0x7C` matrix and Euler angles, copies it into the `+0x1AC` adapter and enables it. |
| `0x8077B2B0` | 172 | `fn_8077b2b0` | Builds the `+0x138` accessor, loads `g3d/model.brres` for `cobStop`, resolves it with `0x80239E30` and binds the `+0x154` adapter. |
| `0x8077B360` | 176 | `fn_8077b360` | Same matrix/angle rebuild as `0x8077AB80` but for the `+0x154` adapter. |
| `0x8077CED0` | 144 | `fn_8077ced0` | Selects effect mode `4` via `0x8077E410`, then spawns the effect through `0x800F69F0` from the caller's record and the `+0x294` factor. |
| `0x807837F0` | 160 | `fn_807837f0` | `cobHatenaSwitch` swing setup: direction `[-1,0,0]`, radius `2π·130/300` at `+0x10C`, step `0x10000/300` at `+0x42C`, pivot copied to `+0xAC..+0xB4`. |
| `0x80789A80` | 136 | `fn_80789a80` | Per-mode actor factory: unsigned `switch 0..=7` over the `0x808192BC` jump table, default `0`. |
| `0x8078D0E0` | 168 | `fn_8078d0e0` | Compares an id against `0x8078D0A0`, otherwise looks it up in the scene's `+0x3394` table (`0x16518` stride), then classifies the difference. |
| `0x8078D710` | 176 | `fn_8078d710` | Byte-identical clone of `0x8077AB80` (adapter `+0x1AC`); only `bl` displacements differ. |

## Evidence and checks

- Every body was re-read from the raw words with
  `node ppc159.js work/rels/elf/d_basesNP.elf <addr>`; the truncated Ghidra
  exports were used only as an index.
- Constants read from the ELF: `0x807CF5C8 = 1.0`, `0x807CF5C4 = 0.0`,
  `0x807CF698 = 0.0`, `0x807CF880 = -1.0`, `0x807CF830 = 0.0` and the signed
  halfword `0x808187B8 = 300`.
- Strings read from the ELF: `0x80818150 "cobStop"`,
  `0x80818140 "g3d/model.brres"`.
- The jump table at `0x808192BC` holds eight words
  (`0x80789AA0`, `0x80789AAC`, `0x80789AB8`, `0x80789AC4`, `0x80789AD0`,
  `0x80789ADC`, `0x80789AE8`, `0x80789AF4`), matching the eight `b` stubs.
- `0x8077AB80`, `0x8077B360` and `0x8078D710` share one shape: the three floats
  at stack `0x1C/0x20/0x24` are the lost arguments of `0x801C0BD0`, and stack
  `0x08/0x0C/0x10` receive the halfwords from `+0x100/+0x102/+0x104` for
  `0x8016EF50`. `0x8077AB80` and `0x8078D710` are instruction-for-instruction
  identical.
- `0x8077B2B0` mirrors the already translated `fn_80650e60` call sequence
  (`0x800690C0`, `0x800DF180`, `0x80239E30`, `0x8016A3A0`, `0x800F2A20`,
  `0x800690E0`).
- `0x8077CED0` mirrors the already translated `fn_8077d680` effect call; the
  record is `lha` at `+0x00`/`+0x08` and `lfs` at `+0x04`/`+0x0C`/`+0x10`.
- `0x8077A000` is invoked with `this` only at the `0x80779F28` call site because
  `r4`-`r6` still hold the leftovers of the preceding `bl`; Ghidra likewise
  prints one argument.
- `0x8078D0E0` calls `0x8078D0A0` through `abi_call1!` because the compared
  value is the `r3` left by that routine's final `bl 0x801002C0`; the existing
  Rust body `fn_8078d0a0` returns `()`.
- `0x8042A18C`, `0x8042A038`, `0x80377C48` and `0x803295D8`/`0x80329548` are
  main.dol globals/vtables, kept as literal addresses.

## Deferred

None.

## Files

- `work/wave190/laneC_block.rs`
- `ledger/agents/wave190-lanec.csv`
- `docs/COVERAGE_WAVE190_LANEC.md`
