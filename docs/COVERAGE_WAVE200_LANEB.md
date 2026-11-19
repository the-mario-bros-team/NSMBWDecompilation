# Coverage - Wave 200, lane B (`game::bases`, `0x8064`-`0x8068` interface thunks)

## How to read this file

Lane B is one of six lanes that audit the 471 generated interface-thunk bodies
in `work/wave200/`. Every address in this lane is a **static analysis address**
under the canonical synthetic base **`0x80600000`** (produced by
`tools/rel2elf.mjs`), never a runtime address: the Wii REL loader picks the
real load address, so the numbers only identify positions inside the REL image.
In this image a virtual address maps to the REL file at
`file_offset = address - 0x80600000`.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`. All 79 of those files
were checked to exist (none assumed). All 79 bodies are `high` confidence and
none is deferred.

## Summary

| Lane | File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---|---:|---:|---:|---:|---:|
| B | `work/wave200/laneB_block.rs` | `game::bases` | 79 | 3,792 | 79 | 0 | 0 |

- Address range: **`0x8064D580` .. `0x80685650`** (first and last body of the
  lane, inclusive), ascending, no duplicates.
- Rust bodies: 79 (`fn_8064d580` .. `fn_80685650`), all taking
  `(interface: usize, object: u32) -> u32` and tail-calling the shared model
  helper `interface_call(interface, object, K)`.
- Ledger: `ledger/agents/wave200-laneb.csv` (header + 79 rows, LF, one
  trailing newline).

## What this lane covers

Every one of the 79 functions is the identical 48-byte interface thunk already
modelled by the coordinator:

```
stwu  r1,-16(r1)      ; mflr r0        ; mr r5,r3      ; mr r3,r4
stw   r0,20(r1)       ; addi r12,r5,K  ; bl 0x802DCBFC ; ori r0,r0,0
lwz   r0,20(r1)       ; mtlr r0        ; addi r1,r1,16 ; blr
```

`r3` (interface) moves to `r5` for the dispatcher while `r4` (object) becomes
the new `r3`, and `r12` receives the byte offset of the interface slot. The
shipped `0x802DCBFC` dispatcher performs the actual virtual call. The Rust body
is therefore exactly `interface_call(interface, object, K)`.

### Per-offset histogram

`K` is the `addi r12,r5,K` immediate, i.e. the interface slot offset:

| `K` | Slot | Bodies | Share |
|---|---|---:|---:|
| `0x0C` | `+0x0C` | 2 | 2.5% |
| `0x18` | `+0x18` | 40 | 50.6% |
| `0x24` | `+0x24` | 37 | 46.8% |
| **total** | | **79** | 100% |

Only these three offsets occur in lane B; there is no other value, no
non-`addi r12,r5` body and no variant prologue in this window.

## Method

1. Extracted the 79 addresses, sizes and offsets from `work/wave200/manifest.json`
   (lane `B`) and the 79 `fn_<addr>` bodies from `work/wave200/laneB_block.rs`.
2. Read the **raw words** of all 79 functions (not a sample) from
   `work/rels/elf/d_basesNP.elf` with `ppc159.js` at `LEN=12`, `STOP=1`, i.e.
   12 instructions = 48 bytes per function:
   `$env:LEN='12'; $env:STOP='1'; node ppc159.js work/rels/elf/d_basesNP.elf <addr>`
3. Compared, per function, all 12 encoded words against the canonical thunk
   above (`9421fff0 7c0802a6 7c651b78 7c832378 90010014 <addi r12,r5,K>
   <bl> 60000000 80010014 7c0803a6 38210010 4e800020`). Ten of the twelve
   words are byte-identical in every body; the `addi` word differs only in `K`
   and the `bl` word only in its relative displacement (all 79 resolve to
   `0x802DCBFC`).
4. Decoded `addi r12,r5,K` (opcode 14, `rt=12`, `ra=5`) and required the
   immediate to equal **both** the body's third argument in
   `laneB_block.rs` and `off` in `manifest.json`.
5. Decoded the `bl` at `addr+0x18` (opcode 18, `LK=1`) and required
   `instr_addr + LI == 0x802DCBFC`.
6. Cross-checked each `size` against `functions.tsv` (all 48) and confirmed the
   encoded span never overlaps the next function start (`0x30` apart within a
   pair, larger gaps between pairs).
7. Re-read each doc block: `# Original` address/size, the `interface + 0xNN`
   prose, and the `# Confidence` line must all agree with the body offset, the
   manifest and the ELF.

Bonus spot checks: first body `0x8064D580` and last body `0x80685650`
disassembled word by word; `0x80650840` and `0x8066A7F0` (the two `0x0C`
outliers) also inspected individually.

## Corrections

**None.** No body in `work/wave200/laneB_block.rs` was wrong, so the file was
not modified.

Issues that were investigated and found to be non-defects:

- `fn_80650840` and `fn_8066a7f0` write the offset as `0xC` instead of `0x0c`.
  This is the same value (12, `+0x0C`) and valid Rust; the binary's immediate
  is `0x0c` in both cases, so no change was needed.
- The doc headers spell the address as `0x8064D580` rather than the wave style
  `0x8064_D580`. All six lane files from the coordinator's generator use this
  same plain 8-digit form, so lane B was left consistent with the wave instead
  of being normalised in isolation. No code literal is affected.

## Deferred

None. All 79 assigned addresses are translated, ledgered and verified.
