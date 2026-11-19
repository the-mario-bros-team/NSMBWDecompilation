# Audit - Wave 246 (r12 forwarders unlocked, batch 18)

## 1. The hidden-`r12` class is solved

The 149 `partial` rows split as bosses 44, enemies 92, player 11, profile 2.
Reading the raw words showed **145 are the same 48-byte adapter**:

```
mr r5,r3 ; mr r3,r4 ; addi r12,r5,K ; bl 0x802DCBFC
```

`r12` is therefore always a literal `addi r12,r5,K` - no `lis`/`addi` pair and no
table-derived descriptor anywhere in the class. Offsets: `0x18` x71, `0x24`
x71, `0x0C` x3. `game::bases::interface_call` / `interface_call_desc` fit
exactly and are now `pub(crate)`.

**25 converted** (first bosses batch), re-adjudicated `translated` / `high`.

Residue:

| Residue | Count | Nature |
|---|---:|---|
| Identical forwarders | 120 | same one-line conversion, no new analysis |
| Not forwarders | 4 | `0x80A0DB40` (jump table), `0x80AEEA30` (uninitialised stack slot), `0x80049940` (multi-helper), `0x8004DB00` (literal `0x80308300` descriptor with scaled `r5`) |

## 2. Bodies translated (65)

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| B | `WM_PLAYER_VTABLE` 248-264 | 12 | 1,576 |
| C | `dEn_c` 173-182 | 10 | 284 |
| D | WorldMap vtable 253-268 | 9 | 1,360 |
| E | player-class vtable 250-261 | 12 | 492 |
| F | `0x803502F8` band 0x803504A8+ | 10 | 2,648 |

## 3. Shared-body coordination worked

`0x80057B90` belongs to three tables (`0x803084D0`, `0x8032679C`,
`0x80325428`). Lane E published it in `game::player`; lane B and lane D each
removed their duplicate and now reference it. Three lanes hit the same conflict
and all resolved it the same way - one Rust owner per address.

## 4. Measurement and decoding lessons

- Slot 262 of the WorldMap table is an **8-byte thunk** (`li r4,1` /
  `b 0x801435E0`) that naive descent over-reads as 368 B.
- Slot 248 of the WorldMap table starts with `beqlr`; descent reports 8 B, the
  linear walk gives the true 76 B.
- The player-class method array **ends at slot 274** (`0x8030891C` is the zero
  terminator), leaving only 8 uncovered slots.
- Two apparent pointer hits for `0x80057C30` were instruction words
  (`lwz r0,0x7C30(r5)`), not data references.

## 5. Ledger after this wave

**20,250 rows: 20,076 translated / 172 partial / 2 stub** (was 20,051 / 197 /
2). The remaining partials are the 120 mechanical forwarders, 4 distinct
bodies, and the documented machine-state/va_list limitations.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `scripts/check.ps1` exit 0, body enforcement armed.
