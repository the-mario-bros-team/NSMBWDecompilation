# Coverage - Wave 129 (`game::bases`)

## How to read this file

This page documents the Wave 129 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`** produced by
`tools/rel2elf.mjs`; the Wii REL loader picks the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
`# Original` block in the source carries the same warning line. Cross-module
pointers (`0x8037xxxx`, `0x8042xxxx`) keep their absolute `main.dol` values.

Every cited file is a real Ghidra C export under
`work/ghidra/rel-d_basesNP/export/`, and every byte count was re-read from
`work/ghidra/rel-d_basesNP/export/functions.tsv` for this pass.

The pass has two halves. The first twelve addresses are the class-B bodies
decoded instruction by instruction by `docs/SPEC_WAVE128_BASES.md`; the
remaining thirteen come from the planner bucket `0x80770000`, the sound and
message layer of the module.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee identity, a field role or a local record
layout had to be inferred. Comments in the bodies keep `// observed:` /
`// inferred:` / `// unknown:` markers.

Three corrections to the wave-128 prose matter for anyone reading these bodies:

1. **`0x807CAB00` and friends are not all 2^52.** `0x807CAB00`,
   `0x807CA208`, `0x807C98D0`, `0x807CC328` and `0x807CCA30` hold
   `0x4330000080000000` (`2^52 + 2^31`, the signed-offset variant), while
   `0x807C9788` holds `0x4330000000000000` (plain `2^52`). Every body reads the
   exact word from the image, so no conversion constant is hard-coded.
2. **`0x807CA24C` is 40.0 and `0x807CA250` is 44.0.** The specification's §8
   quoted `0x807CA244` (20.0) and `0x807CA250` (24.0) for `0x8069FAB0`; the
   instructions actually load `0x807CA24C` and `0x807CA250`. The body follows
   the instructions.
3. **`0x806A0290`'s counters are words, not bytes.** The export's
   `puVar3[0x363]` is `*(u32*)(this + 0xD8C)` and `puVar3[0x362]` is
   `*(u32*)(this + 0xD88)`; the `+0x363`/`+0x362` byte offsets in the
   specification are off by a factor of four. Decoding `0x806A0350` from the
   raw stream (`lwz r4,3468(r31)` / `addi r0,r4,-1` / `stw r0,3468(r31)`)
   confirms the word form.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 25 | 18,300 | 21 | 4 |

All 25 rows are `translated`; none is `partial`. The module header now reports
3285 bodies.

## What this pass covers

- **The twelve ready class-B bodies (12 bodies, 14,588 bytes).**
  `0x80642540` (four player transforms plus seven model panes through the
  `0x800B32E0` staging vector), `0x806822B0` (the packed spawn word with the
  twelve-argument `0x800DB740` constructor and the nine-pane grid),
  `0x80686550` (the fourteen-slot collision-height probe and its
  `0x80686A84` reconvergence), `0x8069B750` (the carried-object two-state
  update), the four-state launch family `0x8069F640`/`0x8069FAB0`/
  `0x806A0290` and the stomp pair `0x806B5E00`, `0x806F3350` (the field-pane
  rebuild with the tri-state table and four six-slot animation chains),
  `0x806F7BB0` (the three-mode oscillation driver), `0x806FD820` (the
  player-cursor quad) and `0x8070D1F0` (the six-entry digit selector).
- **`0x8077` sound-layer queries (5 bodies).** `0x80773AD0`, `0x80773CA0`,
  `0x80773BB0`, `0x80775660` and `0x80775540`: `0x800Fxxxx` slot queries over
  the `+0x338C`/`0xBF8` and `+0x3224`/`0x10` tables, and the three-word
  listener position they produce.
- **`0x8077` capture start-gates (2 bodies).** `0x80777E10`
  (`cobSandpillar` + `g3d/model.brres`) and `0x8077AA30` (`cobStart` +
  `g3d/model.brres`) with the `0x20`/`0x27` priorities and the spawn-nibble
  rate select.
- **`0x8077` volume/height gauges (6 bodies).** `0x80777FE0` (binding state and
  three message cells), `0x8077A850` (the `1`..`6` cue selector),
  `0x8077D230` (the two-vector placement and the `0x8077E410` mode call),
  `0x80778500`, `0x80778320` and `0x8077E250` (the height accumulator, its
  per-character clamps and the `[0x8042A1A0]` mirror with its nine mode arms).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80642540` | `fn_80642540` | 1236 | high |
| `0x806822B0` | `fn_806822b0` | 1108 | medium |
| `0x80686550` | `fn_80686550` | 1884 | medium |
| `0x8069B750` | `fn_8069b750` | 1424 | high |
| `0x8069F640` | `fn_8069f640` | 892 | high |
| `0x8069FAB0` | `fn_8069fab0` | 996 | high |
| `0x806A0290` | `fn_806a0290` | 948 | high |
| `0x806B5E00` | `fn_806b5e00` | 892 | high |
| `0x806F3350` | `fn_806f3350` | 2092 | high |
| `0x806F7BB0` | `fn_806f7bb0` | 1004 | high |
| `0x806FD820` | `fn_806fd820` | 912 | medium |
| `0x8070D1F0` | `fn_8070d1f0` | 1200 | high |
| `0x80773AD0` | `fn_80773ad0` | 212 | high |
| `0x80773BB0` | `fn_80773bb0` | 228 | high |
| `0x80773CA0` | `fn_80773ca0` | 208 | high |
| `0x80775540` | `fn_80775540` | 284 | high |
| `0x80775660` | `fn_80775660` | 204 | high |
| `0x80777E10` | `fn_80777e10` | 324 | high |
| `0x80777FE0` | `fn_80777fe0` | 252 | medium |
| `0x80778320` | `fn_80778320` | 384 | high |
| `0x80778500` | `fn_80778500` | 304 | high |
| `0x8077A850` | `fn_8077a850` | 264 | high |
| `0x8077AA30` | `fn_8077aa30` | 336 | high |
| `0x8077D230` | `fn_8077d230` | 264 | high |
| `0x8077E250` | `fn_8077e250` | 448 | high |

**Totals:** 25 bodies, 18,300 bytes (21 high, 4 medium). The twelve
specification bodies alone are 14,588 bytes.

## Deliberate skips

Read and rejected, with the reason:

- **`0x80772BD0`, `0x80773D70`, `0x80774A60`, `0x80774BD0`, `0x80775730`,
  `0x80775BB0`.** Every one begins with a `0x802DCD60..0x802DCD74`
  save-register helper whose Ghidra "64-bit return" hides the receiver
  (`uVar = func_0x802dcd68(); iVar = (int)(uVar >> 32)`), and three of them are
  declared `void f(void)` with no parameter at all. There is no honest source
  for `this`: `0x80775BB0` and `0x80774BD0` would need a receiver the export
  never prints, and `0x80772BD0`/`0x80773D70`/`0x80774A60` additionally pass the
  same value both as `this` and as a loop argument.
- **`0x8077E410` (344 bytes).** Four arguments where the first is an
  `undefined8`-printed float, plus two virtual calls that take `(f32, f32,
  f32, self, u32)` and `(undefined8)`; the `param_1` value is overwritten by
  `0x800B2F10`'s return inside the `param_3 == 0` arm, so the register that the
  second virtual call receives is not recoverable from this REL.
- **`0x80774750` (292 bytes).** The message composer packs `0x279`,
  `(iVar2 != 0) << 8`, `strlen(name + 3) << 16` and `[0x8042CF84]` into one
  word; the `0x802E2108` callee is a length routine only by inference and the
  `iVar1 + 8` header offsets are the file-format header, which this REL does not
  define.
- **`0x80773AD0`'s sibling `0x80773A40`.** Called from three of the new bodies
  and not itself translated; it is a `_savegpr` body with the same problem as
  the list above.
- **Everything else in the bucket.** The remaining 82 untranslated `0x8077`
  addresses were not attempted: the `0x807740D0`/`0x80773210`/`0x8077B450`
  multi-route bodies, the `0x8077C440`/`0x8077C670`/`0x8077DC80` model
  holders, the `0x80772370`/`0x807727D0` capture loops and the
  `0x80775190`/`0x80775D00` state tables.

## Notes on inference

- [`fn_806822b0`] and [`fn_806fd820`] are `medium` for the reasons the
  specification gives: the twelve-argument `0x800DB740` call (no `abi_callN!`
  covers it, so the new `call_f64x5_u32x7` wrapper carries the register map),
  and the eight packed colour words whose channel order is only implied by the
  `& 0xFF` arguments.
- [`fn_80686550`] is `medium`: the inline array object at `this+0x4D4` is only
  reachable through five raw offsets, and the export's
  `do { ... } while (true)` with the `goto LAB_80686a84` reconvergence is
  written as the equivalent nested loops with the flag-byte `break` preserved.
- [`fn_80777fe0`] is `medium`: the `0x807714F0` export prints no arguments and
  no receiver, so it is called as the parameterless accessor its prototype
  shows; the three message cells and the `+0x18` slot are literal.
- New host-side helpers added by this pass: `call_u32x10` (the ten-argument
  `0x8007FAD0` pane build), `call_f64x5_u32x7` (the spawn constructor) and
  `scan_collision_flags`/`advance_collision_flags` (the four byte-identical
  flag scans of `0x80686550`).
- The `unaff_GQR0`/`ldexpf` pairs printed at the head and tail of
  `0x806B5E00`, `0x8077E250`, `0x80778320` and `0x80778500` are the
  prologue/epilogue spill noise described in `docs/REL_ELF_MAPPING.md`; they
  return their value to nothing and are omitted.
- `0x8077E250`'s `[0x8042A1A0]+0xNN` accesses are byte offsets on the pointer
  stored at `0x8042A1A0`, not offsets from the variable's own address; the body
  dereferences the global once and then indexes the object.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (25 rows, 18,300 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the 3260 translated
  `game::bases` rows were re-read first and none of the 25 is duplicated.
- `node tools/check-lane-csv.mjs wave129` reports `OK`;
  `wave129-bases.csv` has 25 rows and 25 unique addresses.
- `cargo check --message-format=short` reports no error naming
  `src/game/bases.rs`.
