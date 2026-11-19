# Audit - Wave 254 (dedupe restored, fourth vtable complete)

## 1. Duplicate owners back to zero

58 addresses had a `pub unsafe fn fn_<addr>` in more than one file. None is a
ledger row (all untabled pointer-table/alias bodies), so canonical choice used
the nearest-band rule. **32** non-canonical bodies were rewritten as delegations
(26 were already delegates from wave 234).

| Family | Addresses | Canonical | New delegates |
|---|---:|---|---:|
| `0x8001D1C0`-`0x80020BE0` | 7 | `player` / `worldmap` | 0 (existing) |
| `0x80022170`, `0x8002D970` | 2 | `player` | 2 |
| `0x80048B90`-`0x80058880` | 22 | `player` | 20 |
| `0x800645F0`-`0x801628B0` | 17 | mixed (wave-234 arbitration) | 0 (existing) |
| `0x802D8050`-`0x802D81D0` | 10 | `archive` | 10 |

ABI differences were handled with casts (`as u32`, `as usize`, `value as f64`);
four `worldmap` aliases keep an unused receiver because the canonical body is
nullary.

**Important distinction:** a name still appears in delegating files by design.
Verified independently that every remaining duplicate name is either a
one-line delegation or a `cfg(target_arch)` pair (`fn_802e890c` has a PowerPC
`twi` body and a host twin).

## 2. `0x80329958` complete: 45/45

The fourth complete vtable (after player-class, WorldMap actor and
`WM_PLAYER_VTABLE`). Six remaining slots closed, including:

- word 36 - layout-table enable/disable with a `u16` loop bound
  `base + base->[0] + 4` and `cntlzw(flag) >> 5` making `flag == 0` mean
  "enable";
- word 43 - a second global list `0x80377C3C` (distinct from the registry at
  `0x80377B1C`), 0x1C-byte entries with vtable `0x80339A14` at `+0x08`;
- word 39 - **a naming artefact**: `0x8016D180` was already translated as
  `ui::lyt::layout_register` (ledgered, 256 B); `fn_8016d180` now delegates so
  slot scans see a body.

## 3. The `0x8016D180` class, quantified

Lane F resolved the "tabled but unledgered" question definitively:

| Class | Rows | Bytes |
|---|---:|---:|
| Literal: tabled, no ledger row, no body | **2** | 344 |
| No `fn_<addr>` name (semantic name instead) | 1,189 | 255,192 |
| ... translated and ledgered under that name | 1,187 | 254,848 |
| ... the two deferrals above | 2 | 344 |

So no ledger row is actually missing. The 2 deferrals are the frozen
`game::bases` cases (`0x806E2BF0` writes only its own frame; `0x80697090` reads
stack slots it never writes).

## 4. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| B | die-state vtable 161-170 | 10 | 1,100 |
| C | `dEn_c` 260-270 | 10 | 700 |
| D | `0x803502F8` band 0x80350820+ | 10 | 432 |
| E | `0x80329958` words 36-46 | 6 | 1,224 |

Coverage: die-state 177/235; band 248/249; `0x803507B8` 37/37 (lane E closed
the band's word 36, `0x802D83D0`, after lane D's post-pass). The one remaining
band word is the wave-247 `FN_802D49B0_F247` scan false negative at `0x8035063C`.

Lane C proved `fn_800ad250` is `0x800AD090` modulo five `bl` displacements by
word-level diff before delegating, rather than assuming a twin.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- Ledger 20,250 rows: 20,226 translated / 22 partial / 2 stub.
- `scripts/check.ps1` exit 0.
