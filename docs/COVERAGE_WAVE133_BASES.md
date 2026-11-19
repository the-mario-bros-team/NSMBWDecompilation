# Coverage - Wave 133 (`game::bases`)

## How to read this file

This page documents the Wave 133 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`** produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
`# Original` block in the source carries the same warning line. Cross-module
cells (`0x8035xxxx`, `0x8042xxxx`, `0x8080xxxx`) keep their absolute `main.dol`
values; this REL's own `.data` cells keep their `0x807Cxxxx` / `0x8081xxxx`
synthetic values.

Every cited file is a real Ghidra C export under
`work/ghidra/rel-d_basesNP/export/`, and every byte count was re-read from
`work/ghidra/rel-d_basesNP/export/functions.tsv` for this pass. All 26 bodies
end in `blr` at exactly `entry + size - 4` inside their TSV extent.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee contract, a field role or a `.bss`/`main.dol`
cell's meaning had to be inferred. Comments in the bodies keep
`// observed:` / `// inferred:` / `// unknown:` markers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 26 | 5,408 | 10 | 16 |

All 26 rows are `translated`; none is `partial`. The module header now reports
3387 bodies (3361 + 26).

## What this pass covers

- **The twelve class-B bodies of `docs/SPEC_WAVE132_BASES.md` (3,652 bytes)** -
  every headline address of that specification:

  | cluster | bodies |
  |---|---|
  | ring burst | `0x80761510` registry scan plus pane-scale pass, `0x806B7890` four-slot updater, `0x806B7690` `Wm_we_ringburst` emitter |
  | slot emitters | `0x8078B1D0` two 0xA8-spaced emitter groups, `broom` spawn and `0x80776C80` tail |
  | push / beams | `0x806A23B0` one-axis push-apart, `0x806DF630` three-panel beam builder |
  | transforms | `0x80722180` `face_1`/`hair_1` yaw offset, `0x80780BD0` rotated placement, `0x80724460` attached-player placement |
  | pane bookkeeping | `0x80780E20` level/scale refresh, `0x80702AA0` and `0x80702BC0` twin five-entry initialisers |

- **The four hand-off bodies of `docs/SPEC_WAVE132_BASES.md` section 13
  (1,084 bytes).** `0x80780D00` path placement with the `{4, 9, 10}` type table
  and the `70.0 * 0.5` bucket threshold; `0x80711590` x-crossing message;
  `0x80643D40` four pane-contact writes and the seven-slot clear;
  `0x806B39B0` the occupied-seat mask over three 96-byte buckets.

- **Ten `0x807B` planner-bucket bodies (672 bytes).** The shared-object reset
  `0x807B0280`, the layout-name selector `0x807B02D0`, the prompt gate
  `0x807B03B0`, the layout load `0x807B0840`, the `Object`/`Mario` registration
  `0x807B15D0`, the non-zero probe `0x807B83B0`, the three door-manager flag
  accessors `0x807BA810`/`0x807BA830`/`0x807BA850`, and the lazy-update wrapper
  `0x807BFBE0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806A23B0` | `fn_806a23b0` | 308 | high |
| `0x806B39B0` | `fn_806b39b0` | 268 | high |
| `0x806B7690` | `fn_806b7690` | 312 | medium |
| `0x806B7890` | `fn_806b7890` | 336 | medium |
| `0x806DF630` | `fn_806df630` | 308 | medium |
| `0x80643D40` | `fn_80643d40` | 268 | high |
| `0x80702AA0` | `fn_80702aa0` | 276 | high |
| `0x80702BC0` | `fn_80702bc0` | 276 | high |
| `0x80711590` | `fn_80711590` | 272 | medium |
| `0x80722180` | `fn_80722180` | 296 | medium |
| `0x80724460` | `fn_80724460` | 276 | medium |
| `0x80761510` | `fn_80761510` | 340 | high |
| `0x80780BD0` | `fn_80780bd0` | 292 | medium |
| `0x80780D00` | `fn_80780d00` | 276 | medium |
| `0x80780E20` | `fn_80780e20` | 292 | medium |
| `0x8078B1D0` | `fn_8078b1d0` | 340 | medium |
| `0x807B0280` | `fn_807b0280` | 68 | medium |
| `0x807B02D0` | `fn_807b02d0` | 140 | medium |
| `0x807B03B0` | `fn_807b03b0` | 148 | medium |
| `0x807B0840` | `fn_807b0840` | 84 | medium |
| `0x807B15D0` | `fn_807b15d0` | 68 | medium |
| `0x807B83B0` | `fn_807b83b0` | 44 | high |
| `0x807BA810` | `fn_807ba810` | 20 | high |
| `0x807BA830` | `fn_807ba830` | 20 | high |
| `0x807BA850` | `fn_807ba850` | 16 | high |
| `0x807BFBE0` | `fn_807bfbe0` | 64 | medium |

**Totals:** 26 bodies, 5,408 bytes (10 high, 16 medium). The address window runs
from `0x806A23B0` to `0x807BFBE0`.

## Where the instruction stream corrects the specification

Five places where the wave-132 prose or the Ghidra rendering disagreed with the
shipped bytes; the bodies follow the bytes and say so at the site:

1. **`0x806B7690` - the message singleton is `0x80429D60`, not `0x80439D60`.**
   `docs/SPEC_WAVE132_BASES.md` printed `0x80439D60`; the shipped pair is
   `lis r3,0x8043` at `0x806B7750` followed by `lwz r31,-25248(r3)` at
   `0x806B7758`, i.e. `0x80430000 - 0x62A0`. Ghidra's `_DAT_80429d60` is right.
2. **`0x80780BD0` and `0x80780E20` - the manager cell is `0x8042A18C`, not
   `0x8043A18C`.** Both bodies print `lis ...,0x8043` and then load with a
   `-24180` displacement, so the effective cell is `0x8042A18C`. The actor cell
   in `0x80780E20` is likewise `0x8042A1A0`.
3. **`0x80643D40` - `0x807678C0` receives the pane pointer in `r3`.** The
   Ghidra export prints `FUN_807678c0()` with no arguments, but the shipped
   `lwz r3,0x2F4(r30)` immediately before the call loads the pane, and the
   translated sibling [`fn_807678c0`] takes `actor`. The body passes `pane`.
4. **`0x807BFBE0` - `0x807BF560` receives `this` in `r3`.** Ghidra prints no
   argument, but `r3` is untouched from entry and the translated sibling
   [`fn_807bf560`] takes `actor`. The body passes `this`.
5. **`0x806A23B0` - the second `fcmpo` pair is dead.** The shipped stream
   repeats the `delta_x` compare after the sign select; its result feeds
   nothing. The body reproduces the gate, clamp, sign and stores but not the
   dead compare.
6. **`0x806B7690` - the `extsh` argument stays unsigned.** The message call
   loads the byte at `+0x1040` with `lbz` and then executes `extsh r6,r30`; a
   sign-extension of the byte would turn `0x80..0xFF` into negative values, but
   the halfword was already zero-extended, so the argument is `0..255`. The
   body passes `byte as u32` and records the correction.

A small host-side helper `call_f32x5_u32x5` was added for the ten-argument
`0x800DB180` builder (`f1..f5` plus `r3..r7`). No other helper was needed;
`f32_of_f32` and `call_u32x2_f32` already existed for the `0x80780E20` float
chain.

## Deliberate skips

Read and rejected, with the reason:

- **The eight `0x802DCBFC` forwarders** (`0x807B0BF0`, `0x807B0C20`,
  `0x807B30E0`, `0x807B3110`, `0x807B43E0`, `0x807B4410`, `0x807BD0B0`,
  `0x807BD0E0`). The module header already records `0x802DCBFC` as an unresolved
  indirect-jump stub; writing these would only restate the stub call and would
  not add a reconstructible body.
- **The four `halt_baddata` bodies** (`0x807B08A0`, `0x807B5C50`, `0x807B8EC0`,
  `0x807BA740`). Ghidra truncated control flow at the first word; the shipped
  bytes are not a reversible instruction stream at the declared entry, so no
  honest body can be written.
- **`0x807B06C0` and `0x807B0780`.** Both lose their real argument list in the
  `0x802DCD6C`/`0x802DCD74` `undefined8` return model (`extraout_r4` and the
  high half of the save pair). `0x807B0780` is still called by address from
  `fn_807b0840`; a body would need a speculative signature.
- **The remaining large `0x807B` bodies** (`0x807B00C0` and the
  `0x807B2050..0x807BFE20` non-trivial cluster). They each need their own call
  graph and table decode; this lane did not have budget to verify them, and
  Wave 133's second translation lane is working the same bucket.

## Notes on inference

- [`fn_8078b1d0`] is `medium`: the `a`/`b`/`c` triple's meaning and the four
  emitter contracts are not visible; `index - 1` is kept as a wrapping `u32`.
- [`fn_806b7890`] is `medium`: `0x80377DC8` is a `main.dol` zero-fill cell, so
  it is passed by address, and the 60-byte stack record is opaque.
- [`fn_806b7690`] is `medium`: the five-argument `0x8016C890` contract and the
  role of the `+0xCC0` index array are inferred from the register pattern.
- [`fn_806df630`] is `medium`: `0x800DB180`'s ten-argument contract is emitted
  literally; `r5 = -10240` is kept signed.
- [`fn_80722180`] is `medium`: the transform decompose/recompose pair is
  inferred; only the third float is touched.
- [`fn_80780bd0`] is `medium`: the 48-byte basis is written only by
  `0x8016ED10`, so it stays an opaque word block.
- [`fn_80780e20`] is `medium`: the two-word lookup and the implicit `f1` carried
  from `0x801C1650` into `0x802BE470` are inferred from the register stream.
- [`fn_80724460`] is `medium`: the matrix build/concatenate/extract contracts
  are inferred from the in-place `r3 == r5` use.
- [`fn_80780d00`] is `medium`: the `{4, 9, 10}` table is literal, but the
  placement callee's contract is not in this REL.
- [`fn_80711590`] is `medium`: the `.bss` message name `0x8082F788` and the
  `0x800B2EF0` proximity value are read literally; their roles are inferred.
- [`fn_807b0280`], [`fn_807b02d0`], [`fn_807b03b0`], [`fn_807b0840`] and
  [`fn_807b15d0`] are `medium`: every call target and immediate is literal, but
  the main.dol layout/manager objects they drive are not described by this REL.
- The `unaff_GQR0`/`ldexpf` blocks at the head and tail of the twelve class-B
  bodies are the prologue/epilogue spill noise described in
  `docs/REL_ELF_MAPPING.md`: every `psq_l`/`psq_st` is a `qr0` `f31` (or
  `f31`/`f30`/`f29`/`f28`/`f27`/`f26`) save paired with `stfd`/`lfd`. No body
  uses `qr3` and no body contains an `mtspr` to a `GQRn`.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (26 rows, 5,408 bytes),
  re-read from the TSV rather than from prose; every body's last word was
  re-read from `work/rels/elf/d_basesNP.elf` at
  `(address - 0x80600000) + 0x1000` and is `0x4E800020`.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 26 were checked against
  the ledger's address column before being written, and none is duplicated.
- `node tools/check-lane-csv.mjs wave133` reports `OK`;
  `wave133-bases.csv` has 26 rows and 26 unique addresses.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/game/bases.rs`.
