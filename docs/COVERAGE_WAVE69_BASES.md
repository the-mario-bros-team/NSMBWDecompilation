# Coverage - Wave 69 (`game::bases`, `0x806D` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** from the canonical synthetic
base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime address: the
Wii REL loader chooses the real load address, so the numbers below only identify
positions inside the REL image.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; the
cited evidence files are the Ghidra C exports under
`work/ghidra/rel-d_basesNP/export/`. Confidence is `high` when the guards,
stores, constants and call arguments are literal, and `medium` when the role of
a field, of a callee or of a stack block had to be inferred (those bodies carry
`// observed:`, `// inferred:`, `// unknown:` or `// partial:` notes). Constants
that the C prints as `DAT_xxxx` were resolved by reading the synthetic ELF at
`work/rels/elf/d_basesNP.elf`, and strings were read the same way.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 29 | 3732 | 21 | 8 |

## What this pass covers

- **Five constructors of one unnamed lift-actor family,
  `0x806D44B0..0x806DB0B0`** - the `0x5C8` copy [`fn_806d44b0`] (vtable
  `0x807FAE94`), the `0x5A0` copy [`fn_806d7570`] (`0x807FB32C`), the `0x590`
  copy [`fn_806d82c0`] (`0x807FB53C`), the `0x5C0` copy [`fn_806d9260`]
  (`0x807FB7B0`) and the `0x608` allocator wrapper [`fn_806db0b0`] around the
  already-translated in-place constructor [`fn_806db0e0`]. Every constructor
  builds the same skeleton: base `0x80066FC0`, an NW4R holder at `+0x3D4`, a
  `0x40`-stride element array from `0x802DC9A0` with the element helpers
  `0x80169CD0`/`0x80169D20`, and a second holder at `+0x508`.

- **Five registered destructors and two full teardowns,
  `0x806D4330..0x806D90E0`** - [`fn_806d4330`], [`fn_806d73f0`],
  [`fn_806d8140`], [`fn_806d90e0`] and [`fn_806daf30`] are the
  `0x8015F800` + `0x802B9280` pattern already recorded for this bucket; the two
  full teardowns are [`fn_806d7050`] (number-panel actor, nine `0x5C` motion
  records at `+0xBB4`) and [`fn_806d8db0`] (`0x800DAE00` over `+0x508`, three
  `0x40` elements at `+0x3F4`, base `0x800671B0`).

- **Holder and motion resets, `0x806D4AF0..0x806D89D0`** - the two
  `0x800BFFC0` / `0x800DAED0` pairs [`fn_806d4af0`] / [`fn_806d7b00`], the
  `+0xE8` velocity triple reset [`fn_806d7be0`], and the `0x590` reset
  [`fn_806d89d0`] that also republishes the three words at
  `0x807FB530`/`0x807FB534`/`0x807FB538` (`0`, `0xFFFFFFFF`, `0x806D8910`).

- **Input-mask state switches, `0x806D4B50..0x806D8A40`** - the pose-snap plus
  `Wm_en_burst_ss` switch [`fn_806d4b50`], the burst-only switch
  [`fn_806d7b70`], and the load/empty switch [`fn_806d8a40`] whose two outcomes
  are `0x8082D928` and `0x8082D9A8`.

- **Two-slot rider cycles, `0x806D4C10` / `0x806D7C40`** - both build the same
  four-word command block (`index`, record word, the two slots) for
  `0x800C0260`, advance the index with the compiler's `(index + 1) & 1`, and
  spawn the burst when the counter word is set with no player input.

- **Rider and load counters, `0x806D6B70..0x806D8860`** - the nine-record load
  scan [`fn_806d6b70`], the rising/falling panel states [`fn_806d6f20`] /
  [`fn_806d6fd0`], the rider-join switch [`fn_806d6e90`], and the rider-chain
  attach pass [`fn_806d8860`].

- **Achievement cue pair, `0x806D39E0..0x806D3B60`** - the sound-`0x2A6` cue
  [`fn_806d39e0`], its two-tick rider-animation release [`fn_806d3a60`], and the
  per-frame rider pass [`fn_806d3b60`] that ties both together.

- **Recurring layout.** Three helper shapes repeat across the whole window and
  are reused rather than re-derived: `abi_call5!` for the `0x802DC9A0`
  element-array builder (`dest`, init, destructor, element size, count),
  `abi_call4!` for the `0x802DCA98` array release, and `vmethod1(obj + 0x394,
  0x18, state)` for the interface state switch. The two player input masks are
  `0x8042A078` / `0x8042A07C` compared against `+0x358` / `+0x35C`, exactly as
  in the wave-27/28 lift actors.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806D39E0` | `fn_806d39e0` | 128 | medium |
| `0x806D3A60` | `fn_806d3a60` | 96 | high |
| `0x806D3B60` | `fn_806d3b60` | 152 | medium |
| `0x806D4330` | `fn_806d4330` | 88 | high |
| `0x806D44B0` | `fn_806d44b0` | 156 | high |
| `0x806D4AF0` | `fn_806d4af0` | 56 | high |
| `0x806D4B50` | `fn_806d4b50` | 148 | high |
| `0x806D4C10` | `fn_806d4c10` | 224 | medium |
| `0x806D6B70` | `fn_806d6b70` | 156 | medium |
| `0x806D6E90` | `fn_806d6e90` | 88 | high |
| `0x806D6F20` | `fn_806d6f20` | 128 | medium |
| `0x806D6FD0` | `fn_806d6fd0` | 128 | medium |
| `0x806D7050` | `fn_806d7050` | 180 | high |
| `0x806D73F0` | `fn_806d73f0` | 88 | high |
| `0x806D7570` | `fn_806d7570` | 156 | high |
| `0x806D7B00` | `fn_806d7b00` | 56 | high |
| `0x806D7B70` | `fn_806d7b70` | 112 | high |
| `0x806D7BE0` | `fn_806d7be0` | 68 | high |
| `0x806D7C40` | `fn_806d7c40` | 224 | medium |
| `0x806D8140` | `fn_806d8140` | 88 | high |
| `0x806D82C0` | `fn_806d82c0` | 156 | high |
| `0x806D8860` | `fn_806d8860` | 152 | high |
| `0x806D89D0` | `fn_806d89d0` | 96 | high |
| `0x806D8A40` | `fn_806d8a40` | 196 | medium |
| `0x806D8DB0` | `fn_806d8db0` | 168 | high |
| `0x806D90E0` | `fn_806d90e0` | 88 | high |
| `0x806D9260` | `fn_806d9260` | 220 | high |
| `0x806DAF30` | `fn_806daf30` | 88 | high |
| `0x806DB0B0` | `fn_806db0b0` | 48 | high |

## Address range

`0x806D39E0..0x806DB0B0`, in four runs:

- `0x806D39E0..0x806D3B60` - three bodies, interrupted by the rejected
  `0x806D3680` (paired-single export) and the truncated `0x806D38D0`
  (`halt_baddata`, called by `fn_806d39e0`);
- `0x806D4330..0x806D4C10` - five bodies, interrupted by the two external jump
  stubs `0x806D4450` / `0x806D4480` and by the rejected burst `0x806D4A20`;
- `0x806D6B70..0x806D7050` - five bodies, interrupted by the already-translated
  `0x806D64D0..0x806D6E00` wave-51/52 rows and by the panel redraw
  `0x806D6680`, whose `0x802DCD74` pair (`undefined8` return split into two
  registers) makes an honest signature impossible;
- `0x806D73F0..0x806DB0B0` - sixteen bodies, interrupted by the jump stubs
  `0x806D7510` / `0x806D7540` / `0x806D8260` / `0x806D8290` / `0x806D9200` /
  `0x806DB050` / `0x806DB080`, the rejected burst `0x806D7A30`, and the
  already-translated `0x806D96A0` / `0x806DB0E0` / `0x806DBED0` / `0x806DC210`.

At lane start the `0x806D` bucket held 219 functions, 59 of them already
translated. This pass adds 29, leaving 131 for later waves.

## Deliberate skips

- `0x806D4450` (48), `0x806D4480` (48), `0x806D7510` (48), `0x806D7540` (48),
  `0x806D8260` (48), `0x806D8290` (48), `0x806D9200` (48), `0x806DB050` (48),
  `0x806DB080` (48) - the external jump stub `0x802DCBFC`
  (`func_0x802dcbfc(param_2, param_2, param_1)`). Its hidden `r12`
  this-adjust cannot be modelled with `platform::call3`; the same rejection is
  recorded for ten stubs in wave 68 and for `0x80704E00`/`0x80704E30`/
  `0x80705510` in wave 67.
- `0x806D4A20` (204) and `0x806D7A30` (204) - the `Wm_en_burst_ss` bursts of the
  `0x5C8` and `0x5A0` copies. Both are the outlined paired-single block
  (`unaff_GQR0`, `ldexpf`, incoming `longlong` bias) already rejected for
  `0x80651640`, `0x806914D0`, `0x806F0F20` and `0x80740370`. They are still
  called by address from [`fn_806d4b50`], [`fn_806d4c10`], [`fn_806d7b70`] and
  [`fn_806d7c40`].
- `0x806D6530` (104) and `0x806D65A0` (112) - the `+0xF00`/`+0xF02` angle
  steppers built on the same `unaff_GQR3` + `ldexpf` + `bVar == 4 || bVar == 6`
  lane split. Only the instruction sequence is certain, so the scaled value
  would be a guess.
- `0x806D6680` (516) - the number-panel redraw called by [`fn_806d6f20`] and
  [`fn_806d6fd0`]. It starts with `func_0x802dcd74()`, which returns an
  `undefined8` whose halves are split by hand (`(int)puVar2` vs
  `(ulonglong)uVar6 >> 0x20`), so neither the receiver nor the argument order is
  recoverable from the C alone.
- `0x806D86E0` (384) - the platform-load query called by [`fn_806d8a40`]. Its
  real return value is written by the epilogue `func_0x802dcdc0(uVar3)` and the
  body indexes `(&DAT_807cb7d0)[character]` per rider; the table stride and the
  `+0x1524` role split would be guesses, so only the call site is modelled.
- `0x806D3AC0` (120) - truncated by bad instruction data (`halt_baddata`) after
  the `+0x75C` / `+0x430` guards; called by address from [`fn_806d3b60`].
- `0x806D38D0` (36) - truncated by bad instruction data (`halt_baddata`); called
  by address from [`fn_806d39e0`].
- `0x806DB340` (572) / `0x806DB6E0` (752) and `0x806DBED0`'s neighbours - untried,
  not rejected: that constructor pair belongs to the next cluster in this
  bucket.
- The remaining bodies of the bucket (`0x806D00E0..0x806D3C20`, the
  `0x806D3C20..0x806D4D60` tails, and the large `0x806DDxxx` / `0x806DExxx` /
  `0x806DFxxx` blocks) are untried, not rejected; they belong to later waves.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (re-read for this pass; the
  `0x806D7B70` size is 112, not the 148 an earlier draft assumed).
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv` (checked across the whole
  `0x806D` bucket before writing; all 59 pre-existing rows are `game::bases`).
- `node tools/check-lane-csv.mjs wave69` reports `OK` with
  `wave69-bases.csv: 29 rows`.
- `cargo check --message-format=short` reports no diagnostic for
  `src/game/bases.rs`.
