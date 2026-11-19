# Coverage - Wave 167 (`game::bases` 44-64 byte band, remainder)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 18 | 984 | 16 | 1 | 1 |

The 44-64 byte band is now complete apart from the deferred `0x802DCBFC`
forwarders and other waves' stubs.

## What this pass covers

The last 18 non-stub bodies in the 44-64 byte band. Several exports stop at
`halt_baddata`; those bodies were reconstructed from the raw instruction words
in `work/rels/elf/d_basesNP.elf` (`file_offset = address - 0x80600000 + 0x1000`).

- **Float/state predicates (4 bodies).** `0x80652310` returns unless
  `this + 0xB0 >= 8.0 + world + 0x3C`, then calls `0x80162510(this)`.
  `0x8067B1C0` tests `world + 0x38 <= this + 0xAC <= world + 0x38 + world + 0x40`.
  `0x8074DAD0` reduces the `0x8006CF10(this, 0x184)` result to one bit.
  `0x807A57F0` gates `0x80102260(8,3,2,1)` on `+0x408` and a global `0x10` flag.
- **Copy/seed loops (3 bodies).** `0x80706410` scans twelve 12-byte slots at
  `+0x438` and calls `0x807067F0` on the first empty one. `0x807061F0` seeds the
  twelve slots at `+0x43C` with `*(this)`. `0x8071B990` copies four floats from
  `0x807CCF50 + 0x14..0x20` into `+0x318..+0x324`.
- **Small state transitions (4 bodies).** `0x807C5E60` zeroes `+0xAC..+0xB4`,
  calls `0x807C5EA0(this)` and returns `1`. `0x807ABA60` clears bit `0x20` on
  the `0x80109340` frame and calls `0x80108160(0x80107F10(), 0)`.
  `0x8073DA10` decrements `+0x8F4` and requests sound action `2`.
  `0x807AB370` runs `0x800B0DB0(0)` and `0x800B0A20(0x1E)`.
- **Table and descriptor dispatchers (3 bodies).** `0x8067A790` indexes
  `0x807FBF38` with `*(this + 0x478)` and tail-calls `0x8016C890` with five
  arguments. `0x80637380` searches four `0x3C`-strided keys and returns
  `+0x7E0` or `3`. `0x80749310` forwards through the `r12` descriptor
  dispatcher `0x802DCC24`.
- **Seven-argument loader (1 body).** `0x80642CE0` adds `26.0` to
  `*(record + 4)` and calls `0x8008FBF0` with `0x114`-strided table math.
- **State retire twins (2 bodies).** `0x80695580` and `0x806956B0` are
  byte-identical: states `1`/`2` run `0x8004A350(0.0, target)`, state `3` runs
  the `+0x240` virtual. They are the 64-byte twins of `0x806955D0`.
- **One scaled-byte getter (1 body).** `0x807073D0` multiplies a signed byte
  from `0x8042C1A0` by an MSB-based nibble of `*(this + 4)` and sign-extends
  the 16-bit result.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80652310` | `fn_80652310` | 44 | high |
| `0x8067A790` | `fn_8067a790` | 44 | high |
| `0x80706410` | `fn_80706410` | 44 | high |
| `0x8071B990` | `fn_8071b990` | 44 | high |
| `0x807AB370` | `fn_807ab370` | 44 | high |
| `0x807073D0` | `fn_807073d0` | 48 | medium |
| `0x807061F0` | `fn_807061f0` | 56 | high |
| `0x80749310` | `fn_80749310` | 56 | low |
| `0x8074DAD0` | `fn_8074dad0` | 56 | high |
| `0x807A57F0` | `fn_807a57f0` | 56 | high |
| `0x807ABA60` | `fn_807aba60` | 56 | high |
| `0x807C5E60` | `fn_807c5e60` | 56 | high |
| `0x8073DA10` | `fn_8073da10` | 60 | high |
| `0x80637380` | `fn_80637380` | 64 | high |
| `0x80642CE0` | `fn_80642ce0` | 64 | high |
| `0x8067B1C0` | `fn_8067b1c0` | 64 | high |
| `0x80695580` | `fn_80695580` | 64 | high |
| `0x806956B0` | `fn_806956b0` | 64 | high |

## Corrections and notes

- `0x80652310`, `0x8067B1C0` and `0x807A57F0` contain `cror` instructions
  (`0x4C411382` / `0x4C401382`) that Ghidra renders as truncated control flow.
  Decoded as `cror cr0.EQ,cr0.GT,cr0.EQ` and `cror cr0.EQ,cr0.LT,cr0.EQ`, they
  implement the inclusive window tests.
- `0x8074DAD0`'s Ghidra expression omits the final rotate/mask. The raw words
  `srawi`/`subf`/`rlwinm r3,r0,1,31,31` return the sign bit (`0`/`1`), not the
  raw difference.
- `0x807073D0`'s `rlwinm r0,r0,16,16,19` is MSB-based: it selects bits
  `28..31` of the source word into `0x0000F000`. The `extsb` (XO 954) and
  `extsh` (XO 922) pair is decoded manually.
- `0x80749310` needs an `r12` descriptor
  (`0x807CE508 + *(param_2 + 0x400) * 12`). Rust cannot load `r12`, so the call
  is modelled as `call2(0x802DCC24, param_1, param_2)` and marked low.
- `0x80695580` and `0x806956B0` share the `0x807C9E90` (`0.0`) strength with the
  larger `0x806955D0` body; only the `+0xEA4` flag branch is absent.
- `0x8073DA10` was previously listed as truncated in the wave-128 and
  wave-129 notes; its raw words recover the full body, so those notes now point
  here.

## Verification

- `cargo check` succeeds (only pre-existing warnings).
- `node tools/check-lane-csv.mjs wave167` validates the 18-row lane CSV.
- The 18 evidence exports all exist under
  `work/ghidra/rel-d_basesNP/export/`.
