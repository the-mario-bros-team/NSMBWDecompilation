# Coverage - Wave 183 lane B (`game::bases`)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane B block) | `game::bases` | 10 | 1020 | 9 | 1 |

Ten sub-300-byte bodies from the `0x8069xxxx`/`0x806Axxxx` part of
`d_basesNP`. Eight are leaf helpers with a single shape each (four
deleting destructors, three probes and one scaling query), one is a
virtual-call trampoline, and one is an event hook that builds a matrix from an
angle. Nothing had to be deferred, and no new helper was needed.

## What this pass covers

- **Pose event hook (`0x806907D0`).** For `event == 1` only: read the halfword
  at `this->[+0x4] + 0x104`, fold it to `0x4000 - angle` (quarter turn), build
  a rotation matrix with `0x8016EEF0` and copy it over the matrix named by
  `*out` (`0x801C0500` twice, scratch in between). This is the only open
  question in the lane: the negate sequence at `0x806907FC..0x80690800`
  decodes as `neg r0,r0` in the local disassembler (unclassified `X31_104`),
  while Ghidra and the untouched `extsh r31,r0` / `addi r0,r31,0x4000` pair
  both say the loaded halfword is negated before the `+0x4000`. Followed
  Ghidra and the two surviving instructions; recorded as `medium`.
- **Walk-cycle phase (`0x806914D0`).** `+0x500 += 0x4B0`, then
  `sin((u16)(+0x500) * 0.00390625) * 1024.0` truncated into the halfword at
  `+0x102`. Exact twin of the already translated `fn_80648710` (the
  `lha`/`addi`/`sth` + `psq_l ...,1,3` GQR3 sample), with `0x4B0` instead of
  `0x200` and the constant pair `0x807C9D68`/`0x807C9D60`.
- **Kind-`0x219` chain walk (`0x80693D50`).** The export renders this as a
  self-recursion that passes an argument its own prototype omits; the raw words
  put the found entry in `r4` before the recursive `bl` and in `r4` again
  before the `0x80162D50` query, so the body is a cursor loop. It is the callee
  of the already translated `fn_80693be0` (state 1), which passes cursor `0`.
- **State-nibble retire (`0x806940A0`).** State `1` and state `2` (only while
  the target's `+0x556` is zero) both end in `0x80162510(target)`; the
  `0x80162D00(+0x594)` lookup runs in both arms.
- **Deleting destructors (`0x806944C0`, `0x806A5D10`, `0x806A5D70`,
  `0x80699320`).** Three are the `0x8015F800(this, 0)` + `0x802B9280(this)`
  wrapper (`0x806944C0` and `0x806A5D10` are byte-identical, `0x806A5D70`
  keeps one extra redundant null guard that the raw words contain);
  `0x80699320` is the same wrapper over the `0x800A2F40` + `0x80162920` pair.
- **Ground probe hook (`0x80696E00`).** `0x8006DB30(+0x1EC)` then
  `0x800D9940(+0x2DC, probe, 0, 0)`; a hit sets `+0x443 = 1` and ORs `0x8000`
  into `+0x274`, and a non-zero probe alone also sets `+0x443`. Same probe pair
  as `fn_80667d00` / `fn_8067bc30` without the two `0x8006xxxx` resets.
- **Character scale query (`0x80698A20`).** Class vtable `+0x60`, slot
  `+0x36C`, multiplied by `(i8)0x8042C1A0[+0x348]` (`lbzx` + `extsb`), answer
  truncated with `extsh r3,r0` - the same signed-byte table idiom as
  `fn_8069f640`/`fn_8069ff00`.

## Verification

- Every body was read from the raw words (`node ppc159.js`) as well as from the
  Ghidra export; the two constants of `0x806914D0` and the
  `0x807C9D60 = 1024.0`/`0x807C9D68 = 0.00390625` pair were re-read from
  `work/rels/elf/d_basesNP.elf`.
- All helper names used (`platform::read_u8/read_u16/read_u32/write_u8/
  write_u16/write_u32/read_f32`, `abi_call1..4`, `f32_of_f32`, `actor_method0`)
  already exist in `src/game/bases.rs`; no new helper was introduced.
- `src/game/bases.rs` was not touched.

## Deferred

None.
