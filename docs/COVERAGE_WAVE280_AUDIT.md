# Audit - Wave 280 (three tables closed, savegpr part 5)

## 1. Tables closed

| Table | Slots | Identity |
|---|---:|---|
| `0x8034F7F0` | **27/27** | `nw4hbm::lyt::Pane` (base of `0x8034F938`) |
| `0x803157E0` | **17/17** | `Wm_en_burst` big-head layer |

**Fifty-one vtables complete.**

## 2. RTTI structure corrected

`Pane` slot 1 returns the **root `RuntimeTypeInfo` record** at `0x8042ADC0`
(`r13 - 0x48C0`), not a list anchor. The subclass records `0x8042ADD0`
(Picture), `0x8042ADD8` (TextBox), `0x8042ADE0` (Window) and `0x8042ADE8`
(Bounding) all point back to it, and the `0x80007320`/`0x800073D0`/`0x80007470`
"list walks" are **RTTI parent-chain type tests**.

That also produced a body fix: `fn_802af6d0` returned `SDA_8042ADC8` (a
**heap-handle word**) where the image computes `r13 - 0x48A8 = 0x8042ADD8`.
Corrected to the existing `SDA_8042ADD8`, with the arithmetic documented.

## 3. Head-table chain

`0x803157E0` is the big-head layer (`"hanachan_big_head"` /
`g3d/hanachan_big_head.brres`); install `0x80A0F220` allocates 0x938 bytes and
stores base `0x803158D8` -> intermediate `0x8031588C` -> final `0x803157D8`.
The two head tables differ in exactly slots 0, 1 and 7 and share 14.

A further sibling surfaced: `0x80315894` is at **16/17**, with only its slot 1
(`0x800B6C20`, small-head bind) open - a one-body close-out.

## 4. `_savegpr`/`_restgpr` sweep, part 5

| Metric | Before | After |
|---|---:|---:|
| Modelled helper calls | 489 | **442** |
| Bodies containing them | 249 | **224** |
| Bodies changed this wave | - | 25 |
| Calls removed this wave | - | **47** |
| Constants deleted | - | 7 |
| Genuine calls found | - | **0** |

All 25 were in `worldmap.rs`, `0x8010B890..0x80147700`. `fn_8011ea00` had three
modelled sites (one save plus two copies of the single early-return epilogue).
Signatures were extended for `fn_801430e0` (`r5`) and `fn_801468d0` (`r3..r6` +
`f1`/`f2`), and three bodies now return the `r3` they publish before the
epilogue.

The sweep also corrected several pre-existing low/high-word role swaps against
the instruction stream (`fn_80118860`, `fn_8011d100..d380`, `fn_8011efe0`,
`fn_80130f40`, `fn_80142f20`, `fn_80147700`).

## 5. Survey

| Metric | Wave 276 | Wave 280 |
|---|---:|---:|
| Fully covered runs | 52 | **68** |
| Distinct prologue targets | 888 | **807** |
| Runs with >=4 prologue | - | **61** |
| Max prologue in one run | 9 | **6** |

New top targets: `0x80328a88` (`dScBoot_c` state object), `0x80328188`
(`Pausewindow_c`), `0x8034fb80` (EGG heap), `0x8030f15c` (effect-record base),
and the EN59 `dEn` sub-object vtables `0x80302d80`/`0x80301eb0` (185 slots each).

## 6. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | `Pane` base close | 6 + 21 forwarded | 40 |
| B | `Wm_en_burst` close | 9 | ~1,800 |
| C | savegpr sweep part 5 | 25 fixed | - |
| D | survey refresh | 0 | - |
| E | gap batch 10 | 12 | ~920 |
| F | audit wave 279 | 0 (1 fix) | - |

## Verification

- `cargo check` exit 0 (30 warnings); `doc-lint` 0 violations.
- `check-dupe-bodies` exit 0; `check-page-offsets` 39 pairs / 13,738 tokens,
  0 mismatches.
- `scripts/check.ps1` exit 0.
