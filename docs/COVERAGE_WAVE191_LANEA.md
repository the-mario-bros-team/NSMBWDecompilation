# Coverage - wave 191 lane A (`game::bases`, `0x807ADAD0..0x807B06C0`)

Lane A of wave 191 translated six bodies from `rel/d_basesNP.elf` (bucket
`0x80600000`, synthetic analysis base; not a runtime address). All six are
`translated` at `high` confidence. Nothing was deferred.

Address span: `0x807ADAD0..0x807B0780`. Block:
`work/wave191/laneA_block.rs`. Ledger rows: `ledger/agents/wave191-lanea.csv`.

## Bodies

| Address | Bytes | Name | Description | Confidence |
| --- | --- | --- | --- | --- |
| `0x807ADAD0` | 188 | `fn_807adad0` | World-start for the `0x8081E000` layout block: `0x800DF180(archive+4,"Env_world","scene/scene.brres")`, `0x8023A2E0(&file,"MainSelect")`, `0x802426D0(&a,&b)`, then `0x802C8840(0.0, world, &entry, -1, 3)`; when `course == 1` also `fn_807ada10(_unused, course)`. | high |
| `0x807AE400` | 164 | `fn_807ae400` | Straight-line display-list reset: five `0x8016xxxx` prologue calls, font-slot clears (`0x80164B50`/`C10`/`CD0`/`D60`/`D50`), `0x800067D0(0x80,0x92)`, `0x80093F10(0,3)`, `0x80093F10(0,2)`, `0x800067D0(0x92,0xFF)`, `0x80163170`, `0x80164B90(1)`, `0x80164B50(0)`. | high |
| `0x807AECB0` | 188 | `fn_807aecb0` | Binds the `0x8081E398` block: `Env_world` + `light/Layout3D.blight` into the world, `.blmap` into `world+0x14`, `fog/Layout3D.bfog` into the fog manager, each through virtual `+0x08`. | high |
| `0x807AED70` | 188 | `fn_807aed70` | Same world-start as `0x807ADAD0` but for the `0x8081E398` block (`scene/scene.brres` at `+0x74`, `MainSelect` at `+0x88`, float `0x807D0794`); `course == 1` also runs `fn_807aecb0`. | high |
| `0x807AFF90` | 152 | `fn_807aff90` | Gated by `*(u8*)(*(u32*)(this+0xB8)+0x2B8) == 0`; picks prompt `0x80833AA0` when `*(u8*)(this+0xD0) != 0`, else sets bit `0x20` of `0x80109340()+0x1D8`, calls `0x80108160(0x80107F10(),1)`, then prompt `0x80833B60`; both prompts through pane virtual `+0x18` at `this+0x74`. | high |
| `0x807B06C0` | 188 | `fn_807b06c0` | Byte-identical to `fn_807aecb0` with the `0x8081E658` string block (`Env_world` at `+0x94`, `.blight`/`.blmap`/`.bfog` at `+0x50`/`+0x68`/`+0x80`). | high |

## Evidence and decoding notes

- **Reused neighbours.** `0x807ADAD0` and `0x807AED70` are the same body as the
  already-translated `fn_807abe20`, and `0x807AECB0`/`0x807B06C0` are the same
  body as `fn_807abd60`/`fn_807ada10`; the raw words differ only in the
  `lis/addi` string base (`0x8081E000`, `0x8081E398`, `0x8081E658`) and the
  `lfs` offset (`0x768` / `0x794`, both `0.0f32`).
- **`0x807AC26x` are the `vmethod0(this, 8)` stubs.** `0x807AECB0`/`0x807B06C0`
  call them as `bl 0x807AC270` with `r4` already holding the loaded BRRES
  resource. The stub is `lwz r12,0(r3); lwz r12,8(r12); mtctr; bctr`, so all
  registers survive and the real shape is `vmethod1(obj, 0x08, resource)` --
  the same encoding `fn_807ada10` uses.
- **String block bases are not printed by Ghidra.** `lis r31,0x8082` plus
  `addi r31,r31,-8192` / `-7272` / `-6568` resolves to `0x8081E000`,
  `0x8081E398` and `0x8081E658`; the raw `addi` displacements give `+0xA4`,
  `+0xB0`, `+0xC4` (block 1), `+0x24`, `+0x3C`, `+0x54`, `+0x68`, `+0x74`,
  `+0x88` (block 2) and `+0x50`, `+0x68`, `+0x80`, `+0x94` (block 3). All
  strings were read back out of the ELF and match the Ghidra symbols.
- **`0x807AE400` has no stack frame use beyond `r0`.** The 41-instruction span
  is exactly the 20 calls plus `stwu`/`mfspr`/`stw` and the epilogue; there is
  no `_savegpr` helper to drop. Incoming `r3` is live and forwarded to the
  leading `0x80163220`, which is why the zero-argument signature of the twin
  `fn_807aa310` is kept and documented.
- **`0x807AFF90` branch polarity.** `bc 4,2` after `cmpi r0,0` returns when the
  `+0x2B8` byte is non-zero; `bc 12,2` after the `+0xD0` test takes the
  `0x8010xxxx` path when the byte is zero, so `0x80833AA0` belongs to the
  non-zero branch and `0x80833B60` to the zero branch -- the reverse of a naive
  reading of the Ghidra `if/else` ordering.
- **`0x80833AA0`/`0x80833B60` are BSS prompt descriptors.** They read as empty
  in the file image (their contents are built at runtime by `fn_807b0c50`'s
  registry), so they are passed as literal addresses, not string pointers;
  `fn_807b02d0` passes `0x80833BA0`/`0x80833C20` the same way.
- **`0x807AFF90` pane call.** `lwzu r12,0x74(r3)` updates `r3` to `this+0x74`
  before `bctrl`, so the callee receives the pane address -- reproduced with
  `abi_call2!(method, (this + 0x74) as u32, <prompt>)` per the `0x807B02D0`
  precedent (vtable at `+0x74`, slot `+0x18`).

## Deferred

None.
