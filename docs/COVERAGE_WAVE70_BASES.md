# Coverage - Wave 70 (`game::bases`, `0x8068` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** from the canonical synthetic
base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime address:
the Wii REL loader chooses the real load address, so the numbers below only
identify positions inside the REL image.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; the
cited evidence files are the Ghidra C exports under
`work/ghidra/rel-d_basesNP/export/`. Confidence is `high` when the guards,
stores, constants and call arguments are literal, and `medium` when the role of
a field, of a callee or of a stack block had to be inferred (those bodies carry
`// observed:`, `// inferred:`, `// unknown:` or `// partial:` notes). Constants
that the C prints as `DAT_xxxx` were resolved by reading the synthetic ELF at
`work/rels/elf/d_basesNP.elf`, and vtable contents were read the same way.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 29 | 3032 | 22 | 7 |

## What this pass covers

- **One actor family in the `0x8068` bucket, `0x80682D30..0x80685DB0`** - a
  crumbling-board stage object that spawns the two effects
  `Wm_jr_crumble01` and `Wm_jr_crumble02` (the Ghidra strings at
  `0x807ED3C8` / `0x807ED3D8`). Two sizes share one behaviour set: the base
  class (vtable `0x807ED3E8`) and an `0x8A8`-byte derived copy
  (vtables `0x807ED83C` and `0x807EDB10`).

- **Base construction and teardown, `0x80683750` / `0x806837D0`** - the
  in-place constructor runs `0x80094E80`, publishes vtable `0x807ED3E8`,
  builds the accessor `+0x524`, the model holder `+0x544`, the animation block
  `+0x584` and a two-entry `0x114`-stride effect array at `+0x678`
  (`0x80641E10` / `0x80641E50`). The full destructor reverses all five blocks
  and frees through `0x80162920`.

- **Derived in-place constructor `0x80684A30`** - the same skeleton with vtable
  `0x807EDB10`; its allocator wrapper `0x806842D0` (not translated) allocates
  `0x8A8` bytes and then overwrites `+0x60` with `0x807ED83C`.

- **Landing-edge reporters, `0x80682D30` / `0x80682E20`** - per-slot probes.
  Both require the player to be in state `1` or `2`, read the slot's two floats
  at `+0x478`/`+0x480` (`0xE4` stride), and call `0x80056370` with the edge
  event `1` or `2`. The first one also copies the player's character byte
  (virtual `+0x6C`) into `board + 0xAD5` when slot `0` has flag `0x10` set in
  `+0x4AC`.

- **Animation-probe registration, `0x80683A60` / `0x80684D90`** - both reset
  the three scale words `+0xDC..+0xE4` to `1.0`, then hand a seven-word probe
  to `0x8007FB10` for the animation block `+0x584` and start playback with
  `0x8007F900`. The base copy uses `(-32, 8, 32, -8, 0, 0, 0)`; the derived
  copy uses `(-16, 8, 16, -64, 0x80684680, 0, 0)` and additionally sets
  `+0x654 = 1`. `0x80683D70` is the matching one-call stop.

- **Matrix rebuild `0x80684E40`** - builds `+0x7C` from the position
  `+0xAC..+0xB4` with `0x801C0BD0`, applies the Y/X/Z halfword angles
  `+0x102`/`+0x100`/`+0x104` through `0x8016EDD0`/`0x8016ECB0`/`0x8016EEF0`
  and concatenates a `+16`-unit Y offset and a `-16`-unit X offset with
  `0x801C0540`, then binds the matrix and scale to the `+0x544` model.

- **Effect start/stop, `0x80683B00` / `0x80683B80`** - the two `0x114`-byte
  effect objects at `+0x678` and `+0x78C` are driven through their own
  vtables: slot `+0xA0` with `(this, name, 0, position, 0, 0)` to start and
  slot `+0xB0` with `(this, position, 0, 0)` to stop.

- **Height and edge steppers, `0x80684FA0..0x806852F0`** - the four-frame
  countdown arm (`+0x680`, `+0x684`, `+0x68F`), the height step clamped to
  `64.0` (`DAT_807c9830`), the model frame writer `0x80168880(model, 0, 3, ...)`,
  the `+0x678` advance clamped to `-0.7 ..= 0.7` (`DAT_807c9834`), the per-slot
  partner spawn (query `0x80162D50(0x12F, 0)` then `0x80686350`), the `+0x68E`
  clear and getter, the four-byte edge history shift and the `+0x686`/`+0x68A`
  edge query.

- **Release instantiations, `0x806835A0..0x80685950`** - six destructors of the
  two well-known templates: `0x8015F800` + optional `0x802B9280`
  (`0x806835A0`, `0x80684150`, `0x80685530`), free-only (`0x806858B0`), and
  `0x8015F9F0(object + 0x18, 0)` + optional free (`0x806858F0`, `0x80685950`).

- **Interface-record pair, `0x80685B50` / `0x80685DB0`** - the `0x18`-byte
  element built by `0x80685B50` (vtable `0x807EDF8C`, sub-object `0x800E8300`
  at `+0x0C`) is the one that fills the four-entry table at `+0x474` of the
  object built by the already-translated `0x806856E0`.
  `0x80685DB0` is that owner's full teardown: it releases the `+0x4E0` block
  (vtable `0x807EE144`, owned word `+0x4E8`) and the `+0x4D4` block (vtable
  `0x807EDF70`, owned word `+0x4DC`) through `0x802B9290`, drops the table with
  `0x802DCA98`, releases `+0x458`/`+0x464` through `0x802BE7C0`, runs
  `0x80067270` and frees.

- **Recurring layout.** The board object keeps three `0xE4`-byte records from
  `+0x3D4` (mirrored from `+0x680`), its floats at `+0x66C..+0x694`, its edge
  bytes at `+0x684..+0x68F`, its child pointers at `+0x69C`/`+0x6A0`, the
  effect array at `+0x678` and the NW4R group at `+0x524`/`+0x544`/`+0x584`.
  The two player input-independent queries used here are `0x80056CF0(player,
  0xA4)` and `0x80162D50(0x12F, 0)`; both stay `// unknown:`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80682D30` | `fn_80682d30` | 232 | medium |
| `0x80682E20` | `fn_80682e20` | 116 | high |
| `0x806835A0` | `fn_806835a0` | 88 | high |
| `0x80683750` | `fn_80683750` | 124 | high |
| `0x806837D0` | `fn_806837d0` | 152 | high |
| `0x80683A60` | `fn_80683a60` | 160 | medium |
| `0x80683B00` | `fn_80683b00` | 128 | high |
| `0x80683B80` | `fn_80683b80` | 104 | high |
| `0x80683D70` | `fn_80683d70` | 40 | high |
| `0x80684150` | `fn_80684150` | 88 | high |
| `0x80684A30` | `fn_80684a30` | 92 | high |
| `0x80684D90` | `fn_80684d90` | 176 | medium |
| `0x80684E40` | `fn_80684e40` | 232 | medium |
| `0x80684F60` | `fn_80684f60` | 60 | high |
| `0x80684FA0` | `fn_80684fa0` | 64 | high |
| `0x80684FE0` | `fn_80684fe0` | 40 | high |
| `0x80685010` | `fn_80685010` | 60 | medium |
| `0x80685050` | `fn_80685050` | 64 | high |
| `0x80685090` | `fn_80685090` | 172 | medium |
| `0x80685140` | `fn_80685140` | 12 | high |
| `0x80685260` | `fn_80685260` | 92 | high |
| `0x806852C0` | `fn_806852c0` | 40 | high |
| `0x806852F0` | `fn_806852f0` | 8 | high |
| `0x80685530` | `fn_80685530` | 88 | high |
| `0x806858B0` | `fn_806858b0` | 64 | high |
| `0x806858F0` | `fn_806858f0` | 96 | high |
| `0x80685950` | `fn_80685950` | 100 | high |
| `0x80685B50` | `fn_80685b50` | 76 | high |
| `0x80685DB0` | `fn_80685db0` | 264 | medium |

## Address range

`0x80682D30..0x80685DB0`, in four runs:

- `0x80682D30..0x806837D0` - five bodies, interrupted by the three
  paired-single board passes `0x80682F40` / `0x806830D0` / `0x80683200` and by
  the untried `0x8A8` allocator wrapper `0x80683720` (which forwards to
  [`fn_80683750`]);
- `0x80683A60..0x80683D70` - four bodies, interrupted by the two vtable
  dispatches `0x80683B80`-neighbours `0x80683D40` (slot `+0x14` of `+0x544`)
  and the earlier `0x80683900` row already translated in wave 20;
- `0x80684150..0x80685950` - eighteen bodies, interrupted by the two
  `0x802DCBFC` stubs `0x80684270` / `0x806842A0`, the untried allocator
  wrapper `0x806842D0`, the paired-single `0x806843B0` / `0x80684500`, the
  untried `0x80684680` probe callback, the vtable dispatch `0x80684B10`, the
  truncated `0x80685150` and the two `0x802DCBFC` stubs
  `0x80685650` / `0x80685680`;
- `0x80685B50..0x80685DB0` - two bodies that close the `0x18`-byte record
  family opened by the already-translated `0x806856E0`; the next bodies
  (`0x806860E0` onward) are untried.

At lane start the `0x8068` bucket held 176 functions, 19 of them already
translated. This pass adds 29, leaving 128 for later waves.

## Deliberate skips

- `0x806843B0` (336) and `0x80684500` (384) - the outlined paired-single
  blocks (`unaff_GQR0`, `ldexpf`, the `(bVar1 == 4 || bVar1 == 6)` lane split).
  `0x806843B0` also consumes an incoming `unaff_GQR0` scale that the C never
  defines. The same rejection is recorded for `0x80651640`, `0x806914D0`,
  `0x806D4A20`, `0x806D7A30` and `0x80740370`.
- `0x80682F40` (400), `0x806830D0` (264), `0x80683200` (420) and `0x80681C50`
  (384) - the four per-frame board passes. All four sit on the same
  paired-single recovery (`unaff_GQR0`/`unaff_GQR3`, `ldexpf`, the
  `CONCAT22(sVar1, ...)` halfword lane split), so the scaled angle would be a
  guess.
- `0x80684270` (48), `0x806842A0` (48), `0x80685650` (48) and `0x80685680` (48)
  - the external jump stub `0x802DCBFC`
  (`func_0x802dcbfc(param_2, param_2, param_1)`). Its hidden `r12` this-adjust
  cannot be modelled with `platform::call3`; the same rejection is recorded for
  nine stubs in wave 69 and ten in wave 68.
- `0x80685150` (260) - truncated by bad instruction data (`halt_baddata`)
  directly after the `+0x66C` clamp, so only the first half of the per-frame
  velocity integration survives.
- `0x80684B10` (128) and `0x80683D40` (48) - both are pure vtable dispatches
  whose C prints no argument list at all
  (`(**(code **)(*(int *)(param_1 + 0x60) + 0x280))()`). The arity is not
  recoverable from the export, so no honest signature exists. `0x80684B10` is
  vtable `0x807EDB10`'s `+0x08` entry, verified in the synthetic ELF.
- `0x80685BA0` (88) - the element destructor paired with
  [`fn_80685b50`]. It only wraps `0x802BE7C0(element, 0)` plus the optional
  free; it is called by address from [`fn_80685db0`] and was left as a
  deliberate duplicate skip to keep the lane inside the body budget.
- The remaining bodies of the bucket, `0x80680560..0x806822B0` (behind the
  already-translated wave-20/21 rows), `0x806860E0..0x80687380`, and the large
  `0x80688610..0x8068FDA0` block are untried, not rejected; they belong to
  later waves.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv`.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the 19 pre-existing rows of
  this bucket were checked before writing, and none of them is in this pass.
- `node tools/check-lane-csv.mjs wave70` reports `OK` with
  `wave70-bases.csv: 29 rows`.
- `cargo check --message-format=short` reports no diagnostic for
  `src/game/bases.rs`.
