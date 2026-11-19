# Audit - Wave 282 (two tables closed, three body bugs)

## 1. Tables closed

| Table | Slots | Identity |
|---|---:|---|
| `0x8030F15C` | **11/11** | base of the 0x17C non-actor cue/effect records |
| `0x8030791C` | **71/71** | `daNetEnemy_c` family base vtable |

**Fifty-seven vtables complete.** (`0x80302d80` advanced to 166/185, sibling
`0x80301eb0` to 165/185.)

Structure recorded:

- `0x8030F15C`'s installed vptr is the header `0x8030F154`; four `lis`/`addi`
  sites (`fn_80069820` twice, `wm_effect_pool_init` twice) all immediately
  overwrite it with a derived table.
- `0x80307910` is the base of the `daNetEnemy_c` family: two `d_basesNP` ctors
  install it at `+0x60` then replace it with `0x807F9700`/`0x80807DC0`, which
  align slot-for-slot (slots 1-71 identical, only slot 0 overridden). Its sibling
  `0x80307580` shares 36/71 slots; slots 54-71 call the six
  `daLiftDownOnBase_c` state descriptors.
- The EN59 twin tables `0x80302D78`/`0x80301EA8` are **181/185 identical**,
  differing only in slots 16, 133, 141, 142.

## 2. Three body defects found and fixed

- **`wave99_start_stream`** (`audio/ax.rs`): the gate `fn_8019dc90` was called
  with `0`, but the image still has the incoming `r4` (the cue id) live at the
  `bl` (`0x8019868C` follows `mr r28,r4`). Fixed to pass `id`. This bug was
  introduced by the `_savegpr` sweep itself: removing the fake "return value"
  mis-transcribed a neighbouring argument at the same time.
- **`fn_802af6d0`** (wave 280): returned the heap-handle word `0x8042ADC8`
  instead of the type-info record `0x8042ADD8`.
- The audit also confirmed the earlier `fn_801900`-class fixes and corrected
  several "byte-identical twin" claims that actually differ in PC-relative
  branch displacement.

## 3. `_savegpr`/`_restgpr` sweep, part 7

| Metric | Before | After |
|---|---:|---:|
| Modelled helper calls | 389 | **329** |
| Bodies containing them | 199 | **174** |
| Bodies changed | - | 25 |
| Calls removed | - | **60** |
| Constants deleted | - | 11 |
| Genuine calls found | - | **0** |

Mostly `audio/seq.rs`: packed fake returns were rebound to incoming `r3`/`r4`,
and seven bodies with restore-stub exits became real `u32` returns
(`fn_80279510` had 9 exits, `fn_80279880` 6).

## 4. Survey and gap corpus

Gap corpus: **150 of 672** covered, 517 eligible. Three-quarter of the batch was
84-88 byte bodies; two were named by isolated data words.

Two new survey facts: `0x8034fb80` is a BRLAN/RLYT container (not EGG heap), and
`0x8030791c` is a 71-slot vtable (not a state table). Both were survey
misclassifications corrected from constructor/install evidence.

## 5. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | `0x8030f15c` close | 6 | 3,392 |
| B | `0x80302d80` first 12 | 12 | 436 |
| C | savegpr sweep part 7 | 25 fixed | - |
| D | `0x8030791c` close | 21 | ~1,200 |
| E | gap batch 12 | 12 | ~1,000 |
| F | audit wave 281 + doc row | 0 (doc fixes) | - |

## Verification

- `cargo check` exit 0 (30 warnings); `doc-lint` 0 violations.
- `check-dupe-bodies` exit 0; `check-page-offsets` 40 pairs / 13,764 tokens,
  0 mismatches.
- `scripts/check.ps1` exit 0.
