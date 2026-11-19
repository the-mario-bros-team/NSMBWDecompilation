# Coverage - Wave 150 (`game::bases`)

## How to read this file

This page documents the Wave 150 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL (REL module 2). Every address is a **static
analysis address** under the canonical synthetic base **`0x80600000`**; the Wii
REL loader chooses the real load address, so these numbers identify positions
inside the REL image, not runtime addresses. Every `# Original` block in the
source carries the
`Synthetic base: 0x80600000; this is not a runtime address.` line.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee contract, a vtable slot or a field role had
to be named rather than resolved.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 17 | 6,564 | 13 | 4 |

All 17 rows are `translated`; none is `partial`. The module header now reports
3724 bodies (3707 + 17). The `0x8074` planner bucket drops from 74 to 65
untranslated addresses.

## What this pass covers

- **The eight "ready" rows of `docs/SPEC_WAVE149_BASES.md`** (5,832 bytes), each
  re-read against its export and the raw instruction stream:

  | cluster | body |
  |---|---|
  | world-9 water fill/hold | `0x8071DCC0`: phase word `+0x1A59C`, rise timer `+0x1A488`, curve block `0x80809620` (`216.0`/`8.0`/`75.0`/`180`/`15`/`0.1`), `divw` remainder cue gate, effect slot `0x1C`, cue `0x553`, messages `0x80830100`/`0x808300C0` |
  | world-map pose scheduler | `0x80798F10`: record table `0x8031D634`, index from `0x80775030`, two-anchor override (`0x8B`/`0x16`), `0x0E` remap, effect `0x800F6110`, state setter `0x80799960`, `wrist_l1`/`wrist_r1` attach |
  | `daJrClownB_c` frame | `0x8069C780`: rider tick `+0x544`, clip gate on `+0x5F8`, flag bytes `+0x348`/`+0xD31..+0xD34`, `+0x500` gate, shared animation/collision tail |
  | `daLiftWaterWheel_c` driver | `0x806E3090`: rider mask `0x8042A078`, angle `+0x8C4`, delta `+0x8C6` clamped to `-256..256`, step `-d/512`, geometry `+0x7BC`, cues `0x2B3`/`0x2B4` |
  | world-map message dispatcher | `0x80796DC0`: 11-way jump table `0x8081B808`, messages `0x04`/`0x08`/`0x0C`/`0x1F`/`0x21`/`0x12`/`0x13`, route probe `0x800F57A0` |
  | `Wm_ob_cmnsparkloop` init | `0x806D29C0`: packed spawn word bit map, loop scale `+0x774`, variants `+0x76A`/`+0x76B`/`+0x76C`, timer halfwords `+0x766`/`+0x768`, two arcs through `0x800C1560`, return `1`/`2` |
  | course-id selector | `0x8079A040`: one-shot guard `[0x80429F7C]+0x48 == -1`, eight cases, `"F101"` case 5, course code `0x800F9370` and lookup `0x801002C0` |
  | dummy-block animation bind | `0x8064F0A0`: `+0x360` dispatch, hand bytes `+0x3F4`/`+0x3F5`, handles `+0x584`/`+0x5BC`, messages `0x808282F8`/`0x80828338`, clip poll `0x80169020` |

- **Nine `0x8074`-bucket bodies** (732 bytes):

  | cluster | bodies |
  |---|---|
  | release wrappers | `0x8074BF30`, `0x8074D040`, `0x8074D520`, `0x8074DFA0` (`0x8015F800(state, 0)` then `0x802B9280` when `free > 0`) and `0x8074B250` (the same plus a redundant second null test) |
  | list reset | `0x8074E7A0`: `0x8007F950(this + 0x434)` and `return 1` |
  | recursive child counter | `0x8074E3F0`: `0x80162D50(0x156, node)`, `(*count)++`, recurse |
  | actor allocator | `0x8074A330`: `0x801628C0(0x4A8)`, base constructor `0x80066FC0`, vtable `0x808114C8` at `+0x60` |
  | float table getter | `0x8074F3C0`: `0x800B45F0() & 0xFF` indexes ten `f32` at `0x807CE6D8`, default `1.0` |

## Where the binary differed from the wave-149 prose

1. **`0x80796DC0`'s route-record stride is `0x16518`, not `0x6518`.** The words
   at `0x80797004..0x8079701C` are `lis r3,1` (`0x3c600001`),
   `lwz r5,0x338c(r29)`, `addi r0,r3,0x6518`, `lwz r4,0x22c(r31)`,
   `mullw r0,r5,r0`. The addend is therefore `0x10000 + 0x6518`, and
   `0x8042A18C + w * 0x16518 + 0x3394` is the probe base.
2. **`0x8064F0A0`'s first sub-state is keyed the other way round.** At
   `0x8064F11C..0x8064F124` the body loads `lbz r0,0x3f4(r31)`, compares it to
   zero and branches (`beq`) to the `+0x584` arm at `0x8064F194`, so the **clear**
   byte means handle `+0x584` with animation id `+0x5F4` and message
   `0x808282F8`. The spec attached the `+0x5BC`/`0x80828338` pair to the clear
   case. The second sub-state (`+0x3F5`) in the spec is correct.
3. **`0x8079A040` switches on `0x8042A24D`, not `0x8043A24D`.** The prologue is
   `lis r3,0x8043` / `lbz r0,-0x5DB3(r3)`, i.e. `0x8042A24D`, the same byte the
   dispatcher `0x80796DC0` reads.

Two confirmations rather than corrections:

- `0x806D29C0`'s `0x800DB180` call really is **four** `f64` plus **five** words:
  `lfs f1,0x14(r31)` (`0.0`), `lfs f2,0x18(r31)` (`0.0`),
  `lfs f3,0x64(r31)` (`8.0`), the negated loop scale in `f4`, and
  `addi r3,r29,0x3D0` / `mr r4,r29` / `li r5,0` / `li r6,1` / `li r7,0`. A new
  host-side wrapper `call_f64x4_u32x5` was added for it; the older
  `call_f64x5_u32x5` used elsewhere in this file does not match this call.
- `0x8071DCC0`'s two big offsets are `+0x1A488`/`+0x1A59C`, not
  `+0x2A488`/`+0x2A59C`: the base is `this + 0x20000` and the displacements
  `0xA488`/`0xA59C` are **negative** 16-bit values (`-0x5B78`/`-0x5A64`).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8064F0A0` | `fn_8064f0a0` | 708 | high |
| `0x8069C780` | `fn_8069c780` | 740 | high |
| `0x806D29C0` | `fn_806d29c0` | 724 | high |
| `0x806E3090` | `fn_806e3090` | 728 | high |
| `0x8071DCC0` | `fn_8071dcc0` | 744 | high |
| `0x8074A330` | `fn_8074a330` | 76 | medium |
| `0x8074B250` | `fn_8074b250` | 92 | high |
| `0x8074BF30` | `fn_8074bf30` | 88 | high |
| `0x8074D040` | `fn_8074d040` | 88 | high |
| `0x8074D520` | `fn_8074d520` | 88 | high |
| `0x8074DFA0` | `fn_8074dfa0` | 88 | high |
| `0x8074E3F0` | `fn_8074e3f0` | 100 | medium |
| `0x8074E7A0` | `fn_8074e7a0` | 40 | high |
| `0x8074F3C0` | `fn_8074f3c0` | 72 | medium |
| `0x80796DC0` | `fn_80796dc0` | 728 | high |
| `0x80798F10` | `fn_80798f10` | 744 | medium |
| `0x8079A040` | `fn_8079a040` | 716 | high |

**Totals:** 17 bodies, 6,564 bytes. The address window runs from `0x8064F0A0` to
`0x8079A040` (the bucket subset is `0x8074A330..0x8074F3C0`).

## Notes on inference

- `0x80798F10` is `medium`: the unexported `0x80100200` out-parameter is
  reconstructed as a three-`f32` point (`[0]` is dead here, `[1]` is compared,
  and both blocks are later passed to `0x800F6110`), following the wave-130
  precedent. Every record index, string and store is literal.
- `0x8074A330` is `medium`: `0x801628C0` is the size-taking allocator and
  `0x80066FC0` the base-actor constructor that installs the `+0x60`/`+0x394`
  vtables; the call at `0x8074A354` leaves the new pointer in `r3` with no
  printed argument, so the constructor role is `// inferred:` from its export.
- `0x8074E3F0` is `medium`: `0x80162D50` has no export; the
  `(0x156, node) -> next` contract is inferred from the recursion itself.
- `0x8074F3C0` is `medium`: the mask, bound, index scale and both loads are
  literal, but the meaning of `0x800B45F0` is `// unknown:`. Table values read
  from the image are `{0, 1, 4, 2, 3, 5, 1, 7, 1, 6}` with default `1.0`.
- `0x8064F0A0` enters `0x80169020` with the selected handle in `r3` and
  discards the bool, exactly as the export prints.
- `0x8069C780` reuses the wave-146 sibling [`fn_8069c2c0`] tail; the only
  structural difference is the missing `+0x360` launcher gate, so
  `0x8069ACA0` is called whenever `+0x500` is zero.

## Deliberate skips

- `0x80741CA0` (4 bytes) is only `b 0x80741A70`: a branch thunk into a
  `qr3`/`GQR` body whose export still carries `unaff_GQR0` lane splits. It was
  left open with its target.
- The 25 untranslated 48-byte bodies in the `0x8074` bucket
  (`0x80742920`, `0x8074B530`, `0x8074D640`, ...) are the
  `func_0x802DCBFC(param_2, param_2, param_1)` wrappers already rejected in
  wave 149; the duplicated argument makes the true argument order unprovable
  from the export, so none was written.
- `0x807422D0` (20 bytes) ends in Ghidra's `halt_baddata` after a truncated
  control-flow cut, and `0x80743920` (20 bytes) is an unrecovered indirect
  `jumptable` tail call. Neither has a reconstructable body.
- The other 37 `0x8074` rows (56..636 bytes) were not started; they are listed,
  with sizes, for the next pass. (`0x807492B0` and its `0x80749310` helper are
  additionally unattractive because the helper's export is a bare
  `func_0x802dcc24()` save/restore stub, i.e. a Ghidra mis-cut.)

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (17 rows, 6,564 bytes),
  re-read from the TSV for this pass rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 17 were checked against
  the ledger's address column before being written, and the lane validator
  repeats that check.
- `node tools/check-lane-csv.mjs wave150` reports `OK` for
  `wave150-bases.csv`: 17 rows, 17 unique addresses, no duplicates.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/bases.rs` (the two remaining errors name `src/game/enemies.rs`,
  which belongs to the parallel lane).
- All 17 bodies were decoded from the raw words with
  `file_offset = (address - 0x80600000) + 0x1000`.
