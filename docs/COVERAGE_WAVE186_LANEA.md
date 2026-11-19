# Wave 186 lane A coverage (`game::bases`)

Lane A of the sub-300 sweep: ten bodies from REL `d_basesNP`, all translated.
Total: 1,064 bytes, 10 bodies. Nothing deferred.

## Translated

| # | Address | Bytes | Body | Shape |
|---|---------|-------|------|-------|
| 1 | `0x807C3260` | 104 | [`fn_807c3260`] | gate `*(u8 *)(*(u32 *)(this+0xC0)+0x2B3) == 0`; clear `+0x53F`; `fn_807c3a90(this)`, `fn_807c3a10(this)`; `vmethod1(this+0x70, 0x18, 0x8083_4540)` |
| 2 | `0x807794A0` | 84 | [`fn_807794a0`] | ctor: `0x800F5FD0`, `-1`→`+0x184`, vtable `0x8081_7DE0`→`+0x60`, `0x80069020(+0x188)`, `0x8016A2F0(+0x1A4)`, returns `this` |
| 3 | `0x80779900` | 124 | [`fn_80779900`] | ctor: `0x800F5FD0`, `-1`→`+0x184`, vtable `0x8081_7F48`, `+0x18C` member, `+0x1A8=0`, `+0x1AC` member, `0x802DC9A0(+0x1B8, 0x806F0060, 0x8064B270, 0x38, 1)` |
| 4 | `0x8077B110` | 76 | [`fn_8077b110`] | ctor: `0x800F2710`, vtable `0x8081_8158`→`+0x60`, `+0x138` and `+0x154` members |
| 5 | `0x80782ED0` | 120 | [`fn_80782ed0`] | mode `+0x428` 0/1/2 → `vmethod0(this+0x1B0/0x1F0/0x230, 0x14)`; returns `1` |
| 6 | `0x80783930` | 88 | [`fn_80783930`] | `(f32)(3.1415926 * (2.0f * 130.0f) / count)` widened by `frsp` |
| 7 | `0x80788AE0` | 108 | [`fn_80788ae0`] | actor kind `0x280` → bind `Koopa` pane at `+0x1A8` |
| 8 | `0x8078CD10` | 128 | [`fn_8078cd10`] | hold/timer state machine, plays `0x800F64C0(this, s16, 1)`, negates `+0x7A8` |
| 9 | `0x8078CFA0` | 116 | [`fn_8078cfa0`] | `'F'`-skipping lookup into the 22-entry `0x8081A1E8` name table |
| 10 | `0x8078D020` | 128 | [`fn_8078d020`] | counts `'F'` matches against a 4-byte key with `0x802E1BD0` |

## Evidence read

- Raw words for all ten bodies via `work/rels/elf/d_basesNP.elf` (Ghidra exports
  are truncated for `0x807C3A90`, which really takes `this` in `r3`).
- `0x807CF888 = 3.1415926` (f64), `0x807CF890 = 2.0` (f32),
  `0x808187C0 = 130.0` (f32), `0x807CF898 = 4503601774854144.0`
  (`2^52 + 2^31`, the `xoris` unsigned-int-to-double magic), all read from the ELF.
- `0x8081A1E8` holds 22 name pointers: `Fa00, K201, K202, Fa01, K203, K204, K205,
  Fa02, K206, K207, K208, Fa03, K209, K210, Fa04, K306, K305, K304, Fa05, K303,
  K302, K301`. Five start with `'F'`. Index `22` is one word past the table.
- String `0x80818C78 = "Koopa"` (same address already used by
  [`fn_80785580`]).

## Reused neighbours

- [`fn_807c3a90`] / [`fn_807c3a10`] already exist in `src/game/bases.rs`
  (`0x807C3A90`, `0x807C3A10`); `0x807C3260` calls them with `this`, matching the
  existing wave-166 caller at `0x807C3Axx` group.
- [`fn_80779900`] is byte-for-byte the shape of [`fn_8075a6c0`] shifted by four
  bytes in the member offsets, so it reuses that idiom (`0x802DC9A0` array build).
- [`fn_80788ae0`] reuses the exact four-call pane binding of [`fn_80785580`]
  (`0x80168400` / `0x80164F00` / `0x80168230`, same `Koopa` string).

## Deferred

None.

## Confidence notes

- All ten are **high** except [`fn_80788ae0`] (**medium**): its call sequence,
  offsets and pane string are literal, but `0x80162D50(0x280, 0)` is only known
  as the actor-list lookup (kind, previous) and the meaning of the forwarded
  third argument to `0x80168230` is inferred from [`fn_80785580`].
- `0x802DCD74` / `0x802DCDC0` in `0x8078D020` and `0x802DC9A0`'s element
  callbacks are documented as compiler/address-only; they are never emitted as
  semantic calls.

## Files

- `work/wave186/laneA_block.rs`
- `ledger/agents/wave186-lanea.csv`
- `docs/COVERAGE_WAVE186_LANEA.md`
