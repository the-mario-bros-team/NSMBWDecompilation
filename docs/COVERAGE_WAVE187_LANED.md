# Coverage - Wave 187 lane D (`game::bases`, 128-192 byte sweep)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Deferred |
|---|---|---:|---:|---:|---:|---:|
| `src/game/bases.rs` (lane D block) | `game::bases` | 10 | 1684 | 10 | 0 | 0 |

Ten bodies, all from the 128-192 byte band. Source of the bodies:
`work/wave187/laneD_block.rs`; per-function ledger:
`ledger/agents/wave187-laned.csv`.

## What this pass covers

- **Lift-record pair updates (3 bodies).** `0x80649190` (180) subtracts
  `0.5 * +0x1A4B4` from `+0x1A4B8`, clamps at `-8.0` (`0x807C8538`) and only on a
  clamped frame posts `0x80827E90`; `0x80649380` (188) publishes the height and
  either ticks the `+0x1A4EE` hold counter (state `2`, message `0x80827F10` at
  zero) or posts `0x80827F50`; `0x80649560` (132) runs
  `0x806485C0(this, 1)` and posts `0x80827E10` on latch `2`. All three are the
  same `+0xB0`/`+0x1A4B8` height pipeline as the translated `fn_806488e0`,
  `fn_80648aa0` and `fn_80648cb0`, and reuse `fn_80648710` plus the existing
  `0x80648550`/`0x806485C0` bodies. Ghidra's `(short)` cast on `+0x1A4EE` is a
  misreading of the `lhz`/`sth` pair; the body uses `u16`.
- **Visibility-table walk (1 body).** `0x80643260` (148) steps the signed index
  at `*cursor` down while `+0x314 == 1` and up otherwise, probing the seven-word
  table at `+0x8B4`, and on exhaustion reloads the index from `+0x8A0` and queues
  sound `(0x7B, 1)` through `0x80195380`. Ghidra's `halt_baddata` is the
  unmodelled tail call, not bad code.
- **`block_arrow` constructor (1 body).** `0x8064a150` (192) allocates `0x780`
  bytes through `0x801628C0`, runs `0x80094E80`, installs `0x80301EA8` and then
  the class vtable `0x807E39AC`, initialises `+0x534` (`0x8007F7A0`), `+0x6A0`
  (`0x80069020`), `+0x6C0` (`0x80169CD0`) and `+0x710` (`0x8016A5E0`), and stores
  the `0x803295D8` → `0x80329548` motion pair at `+0x704`. The class name is
  `// inferred:` from the adjacent loader `fn_8064a6d0`, which binds exactly this
  `+0x698`/`+0x6A0`/`+0x6BC`/`+0x6C0`/`+0x704` layout.
- **Shared-slot claim (1 body).** `0x8064a820` (180) implements the three-way
  `[0x80429D40] + 0x20` claim (free / self with clear `+0x764` / other resolved
  through `0x80162D00` and refused while its `+0x764` is set).
- **Angle-to-offset refresh (1 body).** `0x8064da10` (180) is the
  `lha`/`sth`/`psq_l ...,1,3` reduction of `+0x104` (unsigned 16-bit sample,
  GQR3) scaled by `1/256`, then `cos`/`sin` (`0x80237C40`/`0x80237BD0`) combined
  with `A = 0.0` and `B = 8.0` into `+0x164`/`+0x168`.
- **Camera-window timer (1 body).** `0x80656610` (164) gates on
  `fn_80656080`, decrements `+0x3E0` and, at zero, reloads `0x20` when
  `[0x80429E00]+0x38 < [0x80429DD0]+0x8FEA8` and `8` otherwise, then runs
  `fn_80656310(this, 1)`.
- **Contact forwarding (1 body).** `0x80656eb0` (148) is the mirror of the
  translated `fn_806b9420`: state byte `1`/`2` on the other actor calls its
  `+0x60` vtable `+0x3F4` with `(other, owner, 0)`; kind `0x12A` additionally
  passes the sign-extended first byte of the owner's `+0x6C` name to
  `0x8069EA30`.
- **`launch_bomb` clip bind (1 body).** `0x80658080` (172) is byte-for-byte the
  clip block of `fn_806574d0`, keyed on the exported `s_launch_bomb_807e60e0`,
  plus the `+0xE8`/`+0xEC`/`+0xF0`/`+0x114` velocity reset and the
  `+0x5F4`/`+0x500` (800) clear.

## Notes

- Constants read out of the ELF for this lane: `0x807C8484 = 0.5`,
  `0x807C8538 = -8.0`, `0x807C86A8 = 0.0`, `0x807C86AC = 8.0`,
  `0x807C86B4 = 0.00390625`, `0x807C8910 = 0.0`, `0x807C8928 = 0.15`,
  `0x807C8988 = 0.0`, `0x807C8998 = 1.0`.
- `0x80648710` has a Rust body (`fn_80648710`), so the three height pipelines
  call it directly. `0x806485C0(this, 1)` and `0x80648550(this)` also have
  bodies but keep the `abi_call2!`/`abi_call1!` form already used by
  `fn_80649010` and `fn_80648cb0`. `0x80648470` still has no body, so it goes
  through `call_f32_u32(0x8064_8470, f32, this)`, the same shape as
  `fn_806488e0`.
- `0x80195380` (sound queue) is used exactly like `fn_806376e0` and
  `fn_80602db0`; `0x8042A488` is the sound manager pointer.
- The `0x1A500`-byte lift record fields used here (`+0x314`, `+0x8A0`, `+0x8B4`,
  `+0xB0`, `+0x1A4B4..+0x1A4EE`, `+0x394`) match the offsets already documented
  for `fn_80647e60`, `fn_80649010` and `fn_80648cb0`; no new struct layout was
  invented.

## Verification

- `work/wave187/laneD_block.rs` type-checks standalone against stubbed helper
  signatures (`rustc --edition 2021 --crate-type lib`).
- Every body's raw words were re-read from `work/rels/elf/d_basesNP.elf`
  (`ppc159.js`) and every branch offset and function end was checked against
  `functions.tsv` sizing (e.g. `0x8064DA10 + 0xB4 = 0x8064DAC4`, the next
  export start).
- All ten evidence exports exist under
  `work/ghidra/rel_d_basesNP/export/`; the CSV targets `src/game/bases.rs`.

## Deferred

None. All ten bodies were proven from the raw instruction stream.
