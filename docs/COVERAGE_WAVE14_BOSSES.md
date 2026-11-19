# Coverage - Wave 14, `d_en_bossNP` (`game::bosses`)

## How to read this file

- **Scope** - the 25 functions added to `src/game/bosses.rs` by the wave-14
  pass over the boss-actor REL `d_en_bossNP` (REL module 4). Earlier passes
  are documented in `docs/COVERAGE_REL_d_en_bossNP.md` (23 functions) and the
  wave-3/4/5/6/10/12/13 boss documents (20 + 24 + 25 + 25 + 25 + 24 + 25).
  This file does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000`, with entry `0x80C000F0`. The retail loader picks a different
  base, so every `0x80C...` value is an analysis address, not a runtime
  address. Calls into `main.dol` (`0x8005xxxx`, `0x8006xxxx`, `0x8007xxxx`,
  `0x8008xxxx`, `0x8009xxxx`, `0x800Dxxxx`, `0x800Exxxx`, `0x800Fxxxx`,
  `0x8010xxxx`, `0x8013xxxx`, `0x8016xxxx`, `0x8019xxxx`, `0x8023xxxx`,
  `0x802Bxxxx`, `0x802Dxxxx`) and the absolute DOL words `0x80354E50`,
  `0x80377C48`, `0x8042A008`, `0x8042A038`, `0x8042A050`, `0x8042C1A0`,
  `0x8042C1A4`, `0x8042C230`, `0x8042CC88`, `0x8042CC8C`, `0x80429D60` are
  unaffected.
- **Evidence** is the Ghidra C under
  `work/ghidra/rel-d_en_bossNP/export/`; the exact file is named in every table
  row. Sizes come from `functions.tsv`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and observable stores are represented; `partial` means one
  documented piece is missing; `stub` is not used here.
- **Confidence** rates the translation, not the guessed role. `high` means the
  bytes, stores and calls are exact. `medium` means the stores and branches are
  exact but a dropped argument or an unproven helper role is reconstructed by
  convention.
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table, so no Nintendo
  symbol is invented. Strings are quoted only where the export points at the
  exact data address.
- Start with the **summary**, then the **clusters**, then the **function
  table**. Read the **unknowns** before treating any medium row as complete.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 25 |
| text bytes added | 5,760 |
| status `translated` | 25 |
| status `partial` | 0 |
| status `stub` | 0 |
| confidence `high` | 17 |
| confidence `medium` | 8 |
| confidence `low` | 0 |
| cumulative functions in `game::bosses` | 216 |
| cumulative bytes in `game::bosses` | 80,340 |

The pass is the state-entry half of six actor families: it binds named clips,
seeds the speed/facing/state fields, and adds two per-frame handlers, one
collision/damage response, one four-slot damage relay and one four-slot
aim/travel update. It deliberately favours complete small and medium bodies over
the remaining large AI dispatchers.

## Synthetic-base warning in one place

| item | value |
|---|---:|
| canonical base | `0x80C00000` |
| entry | `0x80C000F0` |
| BSS start | `0x80C6C900` |
| BSS size | `0x487C` |

Every `0x80C...` address in this document and in `src/game/bosses.rs` is an
analysis address. A live dump will not match it. The constants read at
`0x80C57Axx`, `0x80C57Fxx`, `0x80C58xxxx`, `0x80C5Cxxx`, `0x80C63xxx`,
`0x80C66xxx`, `0x80C67xxx`, `0x80C68xxx`, `0x80C69xxx` and `0x80C6xxxx` are
analysis-image data words, not hardware addresses.

## Cluster 1 - Castle Iggy tail and the `0x1238` actor (1,228 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C0EBF0` | 104 | `fn_80c0ebf0` | model step plus ground probe | translated | high |
| `0x80C0EE40` | 372 | `fn_80c0ee40` | `jump_quake_ed` entry and six-slot wake-up | translated | high |
| `0x80C0F3D0` | 488 | `fn_80c0f3d0` | partner-selector per-frame landing handler | translated | high |
| `0x80C1A650` | 264 | `fn_80c1a650` | `demo_escapeA1` dual-clip entry for the `0x1238` actor | translated | high |

What is proven:

- `0x80C0EBF0` is the short per-frame companion of the Castle Iggy state
  entries: it steps the model (`+0x604` slot `+0x1C`), runs `0x8006CDE0` and
  `0x800955F0`, and only on ground contact (probe `0x80072BC0` of `+0x1EC`)
  writes the rest speed `0x80C57690` to `+0xEC`.
- `0x80C0EE40` binds `jump_quake_ed` (`0x80C5C634`), copies `0x80C57690` to
  all three speeds, fires sound event `0x574`, writes timer `0xD2`, derives a
  slot from `0x80C0EB30`, then notifies the selected `+0xEA0` partner with
  `0x8082FCE0` and its two neighbours with `0x8082FC20`.
- `0x80C0F3D0` clears `+0x104`, adds `+0xAC0` to `+0x102`, selects partner
  `[slot + 1]` with 6 wrapping to 4, and tests that partner's `+0x394`
  controller against `0x8082FC20`, `0x8082FC60` and `0x8082FCA0`. On landing
  it either writes `0x80C576D0` (1.5) to `+0xEC` or sends `0x80C6D7AC`.
- `0x80C1A650` is the `0x1238` actor's demo escape entry: it uses a 30-frame
  blend (`int_to_double(0x1E)` against the bias at `0x80C57A40`), binds
  `demo_escapeA1` to both `+0x644` and `+0x6AC`, stores the second name handle
  at `+0x6A8`, and writes vertical speed `0x80C57A80` (128.0) to `+0xB4`.

## Cluster 2 - fly-boss, Larry/Roy and collision entries (2,316 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C2C5D0` | 276 | `fn_80c2c5d0` | `peach` model loader and `cage_wait` bind | translated | high |
| `0x80C2EFE0` | 200 | `fn_80c2efe0` | `prov` entry, fly boss A | translated | high |
| `0x80C30C30` | 200 | `fn_80c30c30` | `prov` entry, fly boss B | translated | high |
| `0x80C34130` | 216 | `fn_80c34130` | `prov` entry with acceleration | translated | high |
| `0x80C35B70` | 116 | `fn_80c35b70` | `glad` clip bind | translated | high |
| `0x80C365A0` | 148 | `fn_80c365a0` | gated rebind and speed clear | translated | medium |
| `0x80C38C30` | 340 | `fn_80c38c30` | collision/damage response and recoil | translated | medium |
| `0x80C396E0` | 288 | `fn_80c396e0` | `chase` entry with clip restore | translated | medium |
| `0x80C39EB0` | 200 | `fn_80c39eb0` | `wait` entry | translated | high |
| `0x80C3AE00` | 332 | `fn_80c3ae00` | conditional rebind and facing-scaled turn | translated | medium |

What is proven:

- `0x80C2C5D0` loads `g3d/peach.brres` (`0x80C633F0`) from archive `peach`
  (`0x80C63400`), builds the model at `+0x544` and animation at `+0x548`, and
  binds `cage_wait` (`0x80C63408`) to both `+0x588` and `+0x5C0`.
- `0x80C2EFE0`, `0x80C30C30` and `0x80C34130` share one body: bind `prov`
  (`0x80C63A5C`, `0x80C64148`, `0x80C64990`), derive the facing byte with
  `0x80096240`, index the direction table at `0x8042C1A4`, write vertical
  speed `0x8042C230`, play controller clip `0x4E3`, then write state 1. Only
  the third also writes `+0x114` from `0x80C58200`.
- `0x80C35B70` is the pure four-call bind chain for `glad` (`0x80C654F4`).
- `0x80C365A0` and `0x80C40AD0` guard their rebind on the first word of the
  `+0x754` object. `0x80C396E0` restores the previous clip value through
  `0x80168FE0` without a guard; `0x80C3AE00` does the same under a `+0x760`
  effect-table guard.
- `0x80C38C30` is the damage path: the `+0x5F8` query, model switch
  `0x801662F0`, class slot `+0x98`, position helper `0x8008C380`, the
  `*other.class[+0x6C] < 4` impact test (`0x800E20A0` with `5`), target
  resolution and the two class notifications.
- `0x80C39EB0` binds `wait` (`0x80C66368`), writes timer `10`, `+0xF4` from
  `0x80C5859C` and state 1.

## Cluster 3 - Morton entries and the four-slot damage relay (1,004 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C3E5F0` | 156 | `fn_80c3e5f0` | `jump_st` entry | translated | high |
| `0x80C3E970` | 272 | `fn_80c3e970` | `jump_HB2` entry and approach speed | translated | high |
| `0x80C403D0` | 268 | `fn_80c403d0` | `demo_escapeA1` dual-clip with halfword blend | translated | high |
| `0x80C40AD0` | 148 | `fn_80c40ad0` | gated rebind and speed clear | translated | medium |
| `0x80C43860` | 160 | `fn_80c43860` | four-player damage relay | translated | high |

What is proven:

- `0x80C3E5F0` binds `jump_st` (`0x80C67650`), writes `0x80C58824` (0.0) to
  all three speeds, and stores the side from `0x80096240`.
- `0x80C3E970` binds `jump_HB2` (`0x80C67664`), writes the four constants at
  `0x80C5886C`, `0x80C58824`, `0x80C6755C` and `0x80C58870`, computes `+0xF4`
  from the signed direction byte at `0x8042C1A0` through the `0x43300000`
  integer-to-double idiom (`int_to_double` with the bias at `0x80C58878`)
  multiplied by `0x80C58850 * 0x80C67550`, plays clip `0x4CA` and decrements
  the `+0x848` counter.
- `0x80C403D0` reads the blend from the halfword at `0x80C675AA` (4 in this
  image), binds `demo_escapeA1` (`0x80C67690`), repeats the lookup with
  `0x8023A200`, stores the handle at `+0x6A8`, binds `+0x6AC` and applies
  blend `0x80C58824` plus rate `0x80C58820`.
- `0x80C43860` takes its base from `0x802DCD6C`, loops four slots, clears
  `+0xE80`, and on a matching player (`0x8005F900`, live `+0x1EC` probe, same
  `+0x349` byte) calls `0x80137510(player, 1)` and marks the slot.

## Cluster 4 - Wendy entries (928 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C44570` | 212 | `fn_80c44570` | `jump_quake_ed` entry and quake relay | translated | high |
| `0x80C44C30` | 260 | `fn_80c44c30` | `demo_escapeA1` dual-clip with 4-frame blend | translated | high |
| `0x80C47FC0` | 144 | `fn_80c47fc0` | gated rebind plus class slot `+0x57C` | translated | medium |
| `0x80C48390` | 140 | `fn_80c48390` | gated rebind plus negative acceleration | translated | medium |
| `0x80C48A50` | 172 | `fn_80c48a50` | `demo_escapeA1` with 30-frame blend | translated | high |

What is proven:

- `0x80C44570` binds `jump_quake_ed` (`0x80C68944`), zeroes the speeds,
  sends `0x800D8C60(...)`, calls `fn_80c43860` to relay the quake, plays
  clip `0x4B5` and writes timer `0x3C`.
- `0x80C44C30` uses `int_to_double(4)` against the bias at `0x80C58B20`,
  binds `demo_escapeA1` (`0x80C6895C`), then binds the second animation with
  blend `0x80C58B00` and rate `0x80C58AFC`.
- `0x80C47FC0` guards on `+0x760[1]`; `0x80C48390` guards on `+0x760[4]` and
  writes `0x80C58D7C` (-0.1875) to `+0x114`.
- `0x80C48A50` reads the blend from the halfword at `0x80C6993C` (30 in this
  image) and binds `demo_escapeA1` (`0x80C699BC`).

## Cluster 5 - the four-slot aim/travel update (284 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C51310` | 284 | `fn_80c51310` | slot transform, travel vector and velocity hand-off | translated | medium |

What is proven:

- The slot object comes from `owner + 0x14 + index*4`; its progress value is
  returned unchanged from `0x80103B40(slot)`.
- `0x80103C80(slot)` overwrites the actor position at `+0xAC/+0xB0/+0xB4`.
- The old and new positions are passed to `0x801C1650`; when
  `0x80C592A0 < 0x802BE470()`, the difference is written to
  `+0x44/+0x48/+0x4C` and handed to `0x800F6410(actor, owner + 0x44)`.

## Clip-name evidence

The exact strings were read from the analysis image at the addresses the export
passes to `0x8023A0B0`:

| address | string | used by |
|---|---|---|
| `0x80C5C634` | `jump_quake_ed` | `fn_80c0ee40` |
| `0x80C5F190` | `demo_escapeA1` | `fn_80c1a650` |
| `0x80C63A5C` | `prov` | `fn_80c2efe0` |
| `0x80C64148` | `prov` | `fn_80c30c30` |
| `0x80C64990` | `prov` | `fn_80c34130` |
| `0x80C654F4` | `glad` | `fn_80c35b70` |
| `0x80C66368` | `wait` | `fn_80c39eb0` |
| `0x80C663D0` | `chase` | `fn_80c396e0` |
| `0x80C67650` | `jump_st` | `fn_80c3e5f0` |
| `0x80C67664` | `jump_HB2` | `fn_80c3e970` |
| `0x80C67690` | `demo_escapeA1` | `fn_80c403d0` |
| `0x80C68944` | `jump_quake_ed` | `fn_80c44570` |
| `0x80C6895C` | `demo_escapeA1` | `fn_80c44c30` |
| `0x80C699BC` | `demo_escapeA1` | `fn_80c48a50` |
| `0x80C633F0` | `g3d/peach.brres` | `fn_80c2c5d0` |
| `0x80C63400` | `peach` | `fn_80c2c5d0` |
| `0x80C63408` | `cage_wait` | `fn_80c2c5d0` |

The blend sources were also inspected: the halfword at `0x80C675AA` is `4`,
the halfword at `0x80C6993C` is `30`, and `0x80C58878`, `0x80C58B20` and
`0x80C57A40` hold the paired-single integer-to-double bias
`0x4330000080000000`.

## Function table

| address | Ghidra name | Rust function | role | status | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80C0EBF0` | `FUN_80c0ebf0` | `fn_80c0ebf0` | model step and ground probe | translated | high | `80c0ebf0_FUN_80c0ebf0.c` |
| `0x80C0EE40` | `FUN_80c0ee40` | `fn_80c0ee40` | `jump_quake_ed` slot wake-up | translated | high | `80c0ee40_FUN_80c0ee40.c` |
| `0x80C0F3D0` | `FUN_80c0f3d0` | `fn_80c0f3d0` | partner landing handler | translated | high | `80c0f3d0_FUN_80c0f3d0.c` |
| `0x80C1A650` | `FUN_80c1a650` | `fn_80c1a650` | `demo_escapeA1` dual bind | translated | high | `80c1a650_FUN_80c1a650.c` |
| `0x80C2C5D0` | `FUN_80c2c5d0` | `fn_80c2c5d0` | `peach` model loader | translated | high | `80c2c5d0_FUN_80c2c5d0.c` |
| `0x80C2EFE0` | `FUN_80c2efe0` | `fn_80c2efe0` | `prov` entry A | translated | high | `80c2efe0_FUN_80c2efe0.c` |
| `0x80C30C30` | `FUN_80c30c30` | `fn_80c30c30` | `prov` entry B | translated | high | `80c30c30_FUN_80c30c30.c` |
| `0x80C34130` | `FUN_80c34130` | `fn_80c34130` | `prov` entry with acceleration | translated | high | `80c34130_FUN_80c34130.c` |
| `0x80C35B70` | `FUN_80c35b70` | `fn_80c35b70` | `glad` bind | translated | high | `80c35b70_FUN_80c35b70.c` |
| `0x80C365A0` | `FUN_80c365a0` | `fn_80c365a0` | gated rebind | translated | medium | `80c365a0_FUN_80c365a0.c` |
| `0x80C38C30` | `FUN_80c38c30` | `fn_80c38c30` | collision/damage response | translated | medium | `80c38c30_FUN_80c38c30.c` |
| `0x80C396E0` | `FUN_80c396e0` | `fn_80c396e0` | `chase` entry | translated | medium | `80c396e0_FUN_80c396e0.c` |
| `0x80C39EB0` | `FUN_80c39eb0` | `fn_80c39eb0` | `wait` entry | translated | high | `80c39eb0_FUN_80c39eb0.c` |
| `0x80C3AE00` | `FUN_80c3ae00` | `fn_80c3ae00` | conditional rebind and turn | translated | medium | `80c3ae00_FUN_80c3ae00.c` |
| `0x80C3E5F0` | `FUN_80c3e5f0` | `fn_80c3e5f0` | `jump_st` entry | translated | high | `80c3e5f0_FUN_80c3e5f0.c` |
| `0x80C3E970` | `FUN_80c3e970` | `fn_80c3e970` | `jump_HB2` entry | translated | high | `80c3e970_FUN_80c3e970.c` |
| `0x80C403D0` | `FUN_80c403d0` | `fn_80c403d0` | `demo_escapeA1` dual bind | translated | high | `80c403d0_FUN_80c403d0.c` |
| `0x80C40AD0` | `FUN_80c40ad0` | `fn_80c40ad0` | gated rebind | translated | medium | `80c40ad0_FUN_80c40ad0.c` |
| `0x80C43860` | `FUN_80c43860` | `fn_80c43860` | four-player damage relay | translated | high | `80c43860_FUN_80c43860.c` |
| `0x80C44570` | `FUN_80c44570` | `fn_80c44570` | `jump_quake_ed` entry | translated | high | `80c44570_FUN_80c44570.c` |
| `0x80C44C30` | `FUN_80c44c30` | `fn_80c44c30` | `demo_escapeA1` dual bind | translated | high | `80c44c30_FUN_80c44c30.c` |
| `0x80C47FC0` | `FUN_80c47fc0` | `fn_80c47fc0` | gated rebind and slot call | translated | medium | `80c47fc0_FUN_80c47fc0.c` |
| `0x80C48390` | `FUN_80c48390` | `fn_80c48390` | gated rebind and acceleration | translated | medium | `80c48390_FUN_80c48390.c` |
| `0x80C48A50` | `FUN_80c48a50` | `fn_80c48a50` | `demo_escapeA1` bind | translated | high | `80c48a50_FUN_80c48a50.c` |
| `0x80C51310` | `FUN_80c51310` | `fn_80c51310` | slot aim/travel update | translated | medium | `80c51310_FUN_80c51310.c` |

## Unknowns

- **Dropped clip names.** `0x80C365A0`, `0x80C3AE00`, `0x80C40AD0`,
  `0x80C47FC0` and `0x80C48390` call `0x8023A0B0` with no printed name
  argument. The translations keep the same one-argument call and mark those
  rows `medium`. The same sites are the only reason those rows are not `high`.
- **Dropped rate.** `0x80C396E0` calls `0x801680E0` with just the animation
  object, unlike every sibling that passes a constant first. The call is
  translated literally (`call1`) with an `// unknown:` comment.
- **Untranslated callees.** `0x80C0EB30` (84 bytes, the slot-index clamp),
  `0x80C38FC0` (16 bytes, an indirect jump Ghidra could not recover) and
  `0x80C39410` (132 bytes) keep their raw addresses. Their roles are inferred
  from the call sites.
- **Relay signature.** `0x80C43860` is declared `(void)` in the export but is
  called with the actor by `0x80C44570`. Its body takes the object from
  `0x802DCD6C`, so the argument is dead; the translation has no parameters.
- **Vector helpers.** The exact semantics of `0x80103C80`, `0x801C1650`,
  `0x802BE470` and `0x800F6410` are not proven. The translation preserves the
  two-vector call order and the post-call difference, but does not name the
  operations.
- **Paired-single preamble.** The `unaff_GQR0`/`ldexpf` blocks printed by
  Ghidra at the top of `0x80C1A650`, `0x80C3E970`, `0x80C403D0`, `0x80C48A50`
  and `0x80C51310` are register-save/decompiler bookkeeping. They are not
  translated; the integer-to-double expressions they encode are reconstructed
  with `int_to_double` and the original bias address.
- **Opaque constants.** `0x8042C1A4`, `0x8042C230`, `0x8042C1A0`,
  `0x80C576D0`, `0x80C57A80`, `0x80C58200`, `0x80C58348`, `0x80C5859C`,
  `0x80C58D7C` and the notification addresses `0x8082FC20`, `0x8082FC60`,
  `0x8082FCA0`, `0x8082CC04`, `0x8082CC44`, `0x8082CDC8`, `0x8082CE04`,
  `0x8082FCE0`, `0x80358404`, `0x80C6D7AC` are kept at their original
  addresses. Their numeric values are only quoted where the image was
  inspected.

## Verification

- `cargo check` passes (exit 0); `src/game/bosses.rs` produces no warning.
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- All 25 evidence files were read in full; every `# Original` block names the
  file and the size from `functions.tsv`.
- The 25 rows in `ledger/agents/wave14-bosses.csv` match the Rust functions one
  to one: 25 `translated`; 17 `high`, 8 `medium`.
- No files outside `src/game/bosses.rs`, this document and the wave-14 ledger
  were edited.
