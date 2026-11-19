# Wave 193, lane B coverage (`game::bases`)

Lane band `0x806BF1C0..0x806D7A30`, REL `d_basesNP`. Ten of ten requested
bodies translated; none deferred. Two of them are callees that already had
call sites in `bases.rs`: [`fn_806cb140`] (called by `fn_806cb210`) and
[`fn_806d0130`] (called by `fn_806d0350`).

## Bodies

| Address | Bytes | Function | Summary |
| --- | --- | --- | --- |
| `0x806BF1C0` | 252 | `fn_806bf1c0` | Seeds two `0x18`-stride callback arrays and two sealed vectors; same shape as `fn_806bff00`. |
| `0x806C13B0` | 232 | `fn_806c13b0` | Variant constructor: model, clip ids, `+0x318..+0x324` float block, nine-argument collision call, `+0x3BC`/`+0x394` bind. Returns `1`. |
| `0x806C7040` | 224 | `fn_806c7040` | Walks the `+0x30` handle chain until virtual slot `0xC` reports busy; recurses after five hops. |
| `0x806CB050` | 232 | `fn_806cb050` | Re-anchors two `0x5C`-stride records from the `+0x548` point list and the `+0x54C` height table. |
| `0x806CB140` | 208 | `fn_806cb140` | Copies the reference actor onto `this` with `angle + 0x4000`, then rotates `+0x53C`/`+0x540` by sin/cos. |
| `0x806CC530` | 228 | `fn_806cc530` | Drives the `+0x568` mover: translation from the `0x807CB2F8` table and two arc keys from `+0x670`. |
| `0x806D0130` | 244 | `fn_806d0130` | Returns the lowest player height inside the band, scanning four slots with `0x8005F900`. |
| `0x806D0410` | 256 | `fn_806d0410` | Per-tick lift gate: snap below the band, early-out on a 160-unit difference, otherwise advance through `fn_806d0350`. |
| `0x806D4A20` | 204 | `fn_806d4a20` | Spawns `Wm_en_burst_ss` per `+0x5AC * 0.0625` in a row stepping `+16.0` on x. |
| `0x806D7A30` | 204 | `fn_806d7a30` | Same row from `+0x58C` with the `0x807CB788` constant pool. |

## Signatures and evidence

- `fn_806cb140(this: usize, other: usize)` - `r3`/`r4` are the follower and the
  resolved reference actor; `fn_806cb210` already passes them in that order.
  Ghidra's prototype is correct here.
- `fn_806d0130(this: usize) -> f32` - the value leaves in `f1`
  (`fmr f1,f30` at `0x806D01FC`), so the `void` prototype in the export is
  wrong. `this` reaches `0x806D00E0` in `r3`, but that callee immediately
  overwrites `r3` with the `0x80429E00` scene pointer (`lis r3,0x8043` at
  `0x806D00E0`), so the argument is ABI-only. The existing `call0_f32` in
  `fn_806d0350` therefore stays correct; `fn_806d0410` calls the Rust body
  directly.
- Every body was re-read from the raw words with
  `ppc159.js work/rels/elf/d_basesNP.elf <addr>`; the Ghidra exports were used
  only as an index. In particular:
  - `0x806C13B0` needs `call_u32x9` for `0x800D2610`: `r3..r10` are eight
    integer arguments and the ninth is the constant `0` stored at `0x8(r1)`.
    There is no `abi_call8`/`call8`; `call_u32x9` already exists in
    `bases.rs`. The `+0x526` byte comes from `0x807CAF80 = [8,12,16,0]`.
  - `0x806BF1C0` uses `0x807CAE3C` (0.0), `0x807CAE44` (120.0) and
    `0x807CAE48` (4.0); `+0x5F0` and `+0x608` are `stw` of `0x60` and
    `-0x60`, not `stfs`.
  - `0x806CB140` is literal float arithmetic, not double: `fmuls`/`fadds`/
    `fsubs` only. Ghidra's `(double)` casts are its lost-FPR artefacts. The
    `psq_l ...,1,3` reads the stored halfword through GQR3 as an unsigned
    16-bit sample, so the angle is `(other[0x104] + 0x4000) as u16 as f32`.
  - `0x806CC530`'s table index is `((word >> 16) & 1) * 0x18 +
    ((word >> 24) & 0xF) * 4` into `0x807CB2F8`; the `rlwinm` pair at
    `0x806CC55C`/`0x806CC568` was decoded by hand because Ghidra printed the
    index as an unreachable block.
  - `0x806D0130`/`0x806D0410` use the active mask byte `0x80429CA8` with
    `1 << slot`, and `0x807CB450` = 0.0, `0x807CB468` = 16.0,
    `0x807CB46C` = 160.0, `0x807CB4A0` = 18.0.
  - `0x806D4A20`/`0x806D7A30` use `fctiwz` for the count
    (`0.0625` scale) and the descriptor strings `0x807FAE84` and `0x807FB31C`,
    both `"Wm_en_burst_ss"`.
- Constants read from the ELF for this lane: `0x807CB250 = 0.00390625`,
  `0x807CB258 = 134.0`, `0x807CB2E4 = 0.5`, `0x807CB2E8 = 0.0`,
  `0x807CB2EC = 8.0`, `0x807CB2F8 = [1.0, 1.5, 2.0, 3.0, 0.5, 0.75, -1.0]`,
  `0x807CB660 = 0.0625`, `0x807CB664 = 8.0`, `0x807CB668 = 32.0`,
  `0x807CB66C = 16.0`, `0x807CB788 = 0.0625`, `0x807CB78C = 8.0`,
  `0x807CB790 = 32.0`, `0x807CB794 = 16.0`, `0x807CAF68 = 0.0`,
  `0x807CAF74 = 512.0`, `0x807CAF84 = 1.0`, `0x807CAF88 = -128.0`,
  `0x807CAF8C = 256.0`.
- Helpers reused from `bases.rs`: `fn_806c0e10`, `fn_806c1020`,
  `fn_806cac40`, `fn_806d00e0`, `fn_806d0230`, `fn_806d02a0`,
  `fn_806d0350`, `vmethod0`, `f32_of_f32`, `call_f32x2_u32`,
  `call_f32_u32x4`, `call_u32x9`, `abi_call1..abi_call5`.
  `0x806C0F00` has no Rust body and is called through `abi_call1!`.

## Deferred

None.

## Files

- `work/wave193/laneB_block.rs`
- `ledger/agents/wave193-laneb.csv`
- `docs/COVERAGE_WAVE193_LANEB.md`
