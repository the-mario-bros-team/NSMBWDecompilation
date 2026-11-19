# REL evidence: mapping an address to the ELF file offset

The raw `work/rels/elf/*.elf` images and the synthetic addresses used in the
ledger do **not** share a simple `address - base` mapping. Always read the
program headers first.

## Correct rule

1. Read the ELF program headers (the shipped REL images have a section header
   table but no stable section-to-bias relation for this purpose).
2. Find the `PT_LOAD` segment that contains the synthetic address.
3. Compute:
   `file_offset = p_offset + (address - p_vaddr)`

For `d_enemiesNP` the first `PT_LOAD` is `p_offset = 0x10F0`,
`p_vaddr = 0x80A000F0`, so the working formula is:

```
file_offset = (address - 0x80A00000) + 0x1000
```

Using `address - 0x80A00000` directly reads **4 KB early** and produces
plausible-looking but wrong instructions. The wave-127 reclassification lane
found and corrected this; earlier lanes that used the naive formula should be
re-checked against their exports.

`d_basesNP` must be mapped the same way from its own program headers; the
wave-128 spec lane confirmed it also resolves to:

```
file_offset = (address - 0x80600000) + 0x1000
```

Do not copy the `+0x1000` without checking `p_offset` and `p_vaddr`, but both
shipped RELs currently use it.

## Second correction: `functions.tsv` size is an extent

The size column in `work/ghidra/*/export/functions.tsv` is a symbol extent,
not necessarily the exact instruction span. When decoding raw words, follow
control flow from the entry until every path ends in `blr` or a tail call and
use that reachable span; do not assume the TSV size is the body length.

## GQR facts (verified across both RELs)

- No `mtspr GQRn` exists anywhere in `d_basesNP` or `d_enemiesNP`; the GQR
  words come from `main.dol` init (`OSInitFastCast` and friends).
- `psq_l`/`psq_st` encode the quantiser index in bits 12-14 (0 = `qr0`,
  3 = `qr3`), `W` in bit 15, base in bits 16-20, `DQ` in bits 0-11.
- In this build the paired-single opcodes are **`psq_l` = 56 and `psq_st` = 60**.
  A decoder that assumes `psq_st` = 58 misses every `f24..f31` prologue save
  and misclassifies the body.
- `qr0` is type 0 (plain `f32` pair). Most "GQR-damaged" exports only spill
  `f31`/`f30` through `stfd f31` + `psq_st` in the prologue; those bodies have
  no quantisation at all.
- `qr3` users need the GQR3 word installed by `main.dol`; the wave-138 spec
  lane pinned it as **`GQR3 = 0x00050005`** (unsigned 16-bit, scale 0), set at
  `0x8025EEEC`/`0x8025EEF0` (`li r3,5` / `oris r3,r3,5` / `mtspr 915,r3`).
  The earlier wave-137 guess `0x0807` was GQR7's load half, not GQR3.

## Float A-form operand order (verified 2026-11-14)

A-form float instructions (opcode 59 single, 63 double) do **not** use one
uniform field for the second source:

- `fD` = bits 21-25, `fA` = bits 16-20, `fB` = bits 11-15, `fC` = bits 6-10,
  XO = bits 1-5.
- **Two-operand arithmetic** (`fadds`=21, `fsubs`=20, `fdivs`=18, `fsqrts`=22,
  `fres`=24, `frsqrts`=26) uses `fA` and **`fB`** (bits 11-15):
  `ec200824` = `fdivs f1,f0,f1`, `ec211028` = `fsubs f1,f1,f2`.
- **Multiply** (`fmuls`=25 / `fmul`) uses `fA` and **`fC`** (bits 6-10):
  `ec4300b2` = `fmuls f2,f3,f2`, `ec000072` = `fmuls f0,f0,f1`.
- **Four-operand** (`fmadds`=29, `fmsubs`=28, `fnmadds`=31, `fnmsubs`=30) is
  `fD,fA,fC,fB`: multiply by `fC`, add `fB`.

Getting this backwards swaps a multiplicand for an unrelated register, which
is how Ghidra's `4.0 + 16.0/cos` artefact appeared in `0x8071F120` and why
`0x8079AE00`-style bodies can look impossible. The scratch decoder
`%TEMP%\opencode\ppc159.js` now applies the per-XO rule.

## `0x802DCBFC` interface dispatcher (verified 2026-11-15)

`main.dol` contains a small dispatcher that every 48-byte `d_basesNP` thunk
tail-calls:

```
r0  = [r12+0]   self_adjust
r11 = [r12+4]   slot index (signed)
r12 = [r12+8]   vtable offset, or a direct function address
r3  = r3 + r0
if r11 >= 0:  r12 = *(*(r3 + r12) + r11)
ctr = r12; bctr
```

The shortcut branch is `4180000C` = `bt 0` (branch if `cr0.LT`), so the
condition is **`index < 0`**, not `index == 0`. Entries with negative indices
are direct function addresses (e.g. `0x807CEE4C` = `{0, -1, 0x80762C50}`,
a `blr` stub); non-negative indices are vtable slots.

`bases.rs` models it as `interface_call(interface, object, offset)` and
`interface_call_desc(descriptor, object)`. Thunks are `mr r5,r3; mr r3,r4;
addi r12,r5,K; bl 0x802DCBFC`, with `K` in `{0x0C, 0x18, 0x24}`.

## CodeWarrior helpers

`0x802DCD40..0x802DCD74` (and `0x802DCD8C..0x802DCDBC`) are `_savegpr_N` /
`_restgpr_N` helpers, not argument-destroying stubs. The incoming `r3..r10`
survive; a Ghidra "64-bit return" from them is the saved register pair, and
"receiver-less" rejections based on them are usually recoverable.

See `docs/ANALYSIS_WAVE127_BASES.md` and `docs/ANALYSIS_WAVE127_ENEMIES.md`
for the address-level reclassification and next-claim lists.
