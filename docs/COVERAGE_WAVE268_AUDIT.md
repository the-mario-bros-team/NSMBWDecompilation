# Audit - Wave 268 (four tables nearly closed, pakkun hierarchy resolved)

## 1. `daEnDpakkunBase_c` identity arbitrated

Three tables carried overlapping labels. The constructors settle it:

| Header | Methods | Slots | Class | Role |
|---|---|---:|---|---|
| `0x8030448C` | `0x80304494..0x80304748` | 173 | `daEnDpakkunBase_c` | base |
| `0x80303F48` | `0x80303F50..0x80304240` | 188 | `daEnDpakkun_c` | base of the derived table |
| `0x803030FC` | `0x80303104..0x803033F0` | 187 | `daEnDfpakkun_c` | derived |

`fn_800281c0` stores `0x80303F48` at `+0x60` (`0x800281F0`) and then overwrites it
with `0x803030FC` (`0x80028248`) - base then derived. The four `0x760`
constructors reach `0x803030FC` through that function and then install
`0x80B4338C/0x80B43744/0x80B43AFC/0x80B43EB4`; the four `0x6E8` constructors
install `0x80303F48` directly and then
`0x80B44264/0x80B4463C/0x80B44A14/0x80B44DEC`.

Adjacent strings confirm it: `daEnDfpakkun_c::StateID_*` plus the
`dokan_fire_*` block belong to `0x803030FC`; `daEnDpakkun_c::StateID_*` precedes
`0x80303F48`; `daEnDpakkunBase_c::StateID_*` sits after the six `0x80304240`
parameter blocks and before `0x8030448C`.

Doc fixes: `_B261` relabelled `daEnDfpakkun_c` with an arbitration paragraph, its
`this`-actor and deleting-destructor docs corrected, and `_B267` gained a
"not to be confused with `0x803030FC`" note. References to the
`daEnDpakkunBase_c::StateID_*` **records** were left intact - they are
Base-owned even from a derived table.

## 2. Coverage

| Table | Coverage | Prologue left |
|---|---:|---:|
| `0x80304494` (`daEnDpakkunBase_c`) | 157/173 | 0 |
| `0x80315EF0` (Kinopio panel) | 36/46 | 0 |
| `0x803175E8` (Mario/Luigi model) | 39/46 | 0 |
| `0x8031C288` (`daYoshi_c`) | 30/42 | 0 |
| `0x80323D40` (fireball base) | 57/78 | 1 |

All five are leaf/`b`-thunk tails now; the only prologue left is
`0x800411D0` (slot 74 of the fireball table).

Four new identities this wave: Kinopio panel, Mario/Luigi character-model
record, `daYoshi_c`, and the `daFireBall_Base_c` actor (constructor `0x8011A470`,
which also installs `0x80329958` at `+0x42C` and `0x80317860` at `+0x3EC`).

## 3. Concurrent-write collisions, all resolved by the owning lane

Six addresses were translated twice by different lanes working the same shared
methods (`0x800BD790`, `0x800BD0F0`, `0x800BD6F0`, `0x800BD6E0`, `0x800D4240`,
`0x800D4520`). Lane C268 detected the duplication and removed **its own** copies,
leaving the earlier bodies canonical. The duplicate gate now reports 0 failing;
no lane deleted another's work.

## 4. Evidence audit

66 wave-267 bodies re-derived: **0 body-level findings**. All extents, terminal
words, table words, single-owner and untabled claims reproduce, and all 29
anchor constants match their documented floats/strings/words.

One doc defect fixed: `_B267` slot 73 cited word `0x803045C0`, which is slot 75;
the true slot-73 word is `0x803045B8` holding `0x8002FBE0`. Fixed in both the
lane table and the body block. Wave-268 lane A had independently flagged the
same slip before the audit ran.

Twelve `_E267` table lines in `dsys.rs` gained explicit `+0x08...+0x84` offsets.

## 5. Decode notes

- `0x800D4240` contains a **dead compare**: the `fcmpo`/`cror 2,1,2` tests the
  source `+0x20` halfword, but the only branch re-tests `rlwinm.` on `+0x160`.
  Two lanes decoded this independently and agreed.
- `0x800D4790` is 484 B, not 272 B: the forward intra-body `b` must be followed.
- `0x8002F6F0` is 60 B, not 8 B: conditional `bc` arms fall through.
- `0x800FF120` is 84 B, not 52 B: a conditional `bc` arm falls through.

## 6. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | `0x80304494` 8 prologue + 4 | 12 | 1,592 |
| B | `0x80315EF0` 6 prologue + 6 | 12 | 1,640 |
| C | `0x803175E8` 4 prologue + 8 | 6 | ~1,000 |
| D | `0x8031C288` 1 prologue + 11 | 12 | ~300 |
| E | `0x80323D40` 12 prologue | 12 | 2,328 |
| F | audit + identity arbitration | 0 (doc fixes) | - |

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-dupe-bodies` exit 0; `check-page-offsets` 32 pairs / 13,602 tokens,
  0 mismatches.
- `scripts/check.ps1` exit 0.
