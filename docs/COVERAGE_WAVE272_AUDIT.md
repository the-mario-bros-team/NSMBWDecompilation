# Audit - Wave 272 (twenty-nine vtables, survey shift)

## 1. Three more tables closed

| Table | Slots | Class |
|---|---:|---|
| `0x803017d8` | **92/92** | `daBossDemo_c` (derived) |
| `0x80301504` | **77/77** | `daBossDemo_c` (base) |
| `0x8034F9FC` | **33/33** | NW4R `lyt::Window` (`wnd1`) |
| `0x803189A8` | **42/42** | base of `dPosGoAndCome` marker |

**Twenty-nine vtables complete.**

## 2. Base/derived pairs are now fully translated in three places

- `0x80301504` (base) / `0x803017d8` (derived): 44 same words, 33 overrides,
  15 appended. Slot 60 in the derived table is a bare `bclr` that deliberately
  suppresses the base's genuine 92-byte reset at `0x8001D090`.
- `0x803189A8` / `0x8031C288` (`daYoshi_c`): 20/42 same-index slots; 22 words
  differ, and slot 21 is the only difference that still reaches the base body
  (base `0x800D5DD0` vs the four-byte Yoshi trampoline `0x800FEA90`). The other
  21 are genuine Yoshi overrides.
- `0x8032B43C` (TextBox) / `0x8032B524` (Window): 11 shared targets.

## 3. Survey shift

| Metric | Wave 266 | Wave 272 |
|---|---:|---:|
| Distinct prologue targets (window) | 1,133 | **979** |
| Max prologue count in one run | 20 | **11** |
| Fully covered runs | 15 | **37** |
| Runs fully uncovered | - | 66 |
| Runs partially uncovered | - | 295 |

Twelve runs completed outright since wave 266. The largest remaining run has 11
prologue targets, against 20 at wave 266, so the per-round yield of
prologue-shaped work is now bounded by run size rather than by table size.

New top runs: `0x8031bf70` (`character_SV`/`dBase_c` base, 11 prologue),
`0x80329d78` (`homebutton::gui::PaneManager`, 11), `0x8034e5e8` (NW4R G3D, 11),
`0x8032df1c` (LYT window manager, 9), and three 16-slot G3D animation vtables.

## 4. Gap corpus

Gap batch 2 added 12 independent functions (48-60 B). Census: **25 of 260** band
candidates now have bodies, **235 free**.

One gate interaction worth recording: translating `0x802D6C80` made the semantic
duplicate pass read an existing `.ctors` tail stub's documentation as a competing
claim for that address, because the stub's `- Ghidra: none` line named its callee
first. Fixed by naming the stub's own address before the callee - a
documentation-ordering requirement of the semantic check.

## 5. Evidence audit

66 wave-271 bodies re-derived: **0 findings**. Censuses 158/158, 29/29 and 27/27
reproduce; the 11 shared Window/TextBox targets and the 44/33/15 parent/derived
split are exact; slot 60's suppression is confirmed against the base body.

Three doc-only fixes: two wrong `+offset` values in `hud.rs` (slot 6 `+0x018` ->
`+0x020`, slot 16 `+0x040` -> `+0x048`) and a wrong 13-entry shared list in the
TextBox header, replaced with the actual eleven (`2,3,5,9,10,13,15,16,17,21,22`).

## 6. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | close `daBossDemo_c` derived (18) | 18 | ~200 |
| B | close its base (17) | 17 | 108 |
| C | close `wnd1` + `0x803189A8` (17) | 17 | ~176 |
| D | survey refresh | 0 | - |
| E | gap batch 2 | 12 | ~660 |
| F | audit wave 271 | 0 (doc fixes) | - |

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-dupe-bodies` exit 0; `check-page-offsets` 34 pairs / 13,672 tokens,
  0 mismatches.
- `scripts/check.ps1` exit 0.
