# Wave 188 lane A coverage (`game::bases`)

Lane A of the 128-192 byte band: ten bodies from REL `d_basesNP`, all
translated. Total: 1,512 bytes, 10 bodies. Nothing deferred.

## Translated

| # | Address | Bytes | Body | Shape |
|---|---------|-------|------|-------|
| 1 | `0x80673C80` | 136 | [`fn_80673c80`] | `0x802E1C40(name, ':')`, `vmethod0(this, 0x20)`, `0x802E1C40`, `0x802E1AB4(tail, wanted) == 0` |
| 2 | `0x8067BD00` | 132 | [`fn_8067bd00`] | `((row&0xFF)*8 + (u8[+0x348]==variant)*4)` indexes `0x807C9510`/`0x807C9538`; writes `+0xE8`(signed char table × f32), `+0xEC`, `+0xF0=0.0` |
| 3 | `0x8067C570` | 136 | [`fn_8067c570`] | same name-suffix compare as `0x80673C80` |
| 4 | `0x8067CD60` | 132 | [`fn_8067cd60`] | `+0x274` mask `0x200000`/`0x3F`/`0x3C000000`, probes `0x80070CB0`/`0x80070720(+0x1EC)`, low 16 bits `== 3` |
| 5 | `0x8067D3D0` | 136 | [`fn_8067d3d0`] | same name-suffix compare as `0x80673C80` |
| 6 | `0x8067ECC0` | 188 | [`fn_8067ecc0`] | `0x80429CA0 == 1` → compare against [`fn_8067dc50`]; else scan players `0..3` via `0x8005F900`; kill via `0x80064D10(this, 0)` |
| 7 | `0x806860E0` | 180 | [`fn_806860e0`] | eight shape floats (`0/1/32/48/144`), then four `0x802BE800(+0x464, +0x474+i*0x18)` registrations bumping `+0x470` |
| 8 | `0x80686350` | 132 | [`fn_80686350`] | `lwzu` receiver `this+0x4D4`, `vmethod0(+0x4D4, 0x1C)`, fills `{f32,f32,word,byte,word}` |
| 9 | `0x806947C0` | 148 | [`fn_806947c0`] | element ctor: zero `+0`, `+0x10=2.0`, `0x802D7AA0(+0x1C)`, zero `+0x130..+0x140`, vtable `0x8032989C`, `vmethod0(+0x1C, 0x98)` |
| 10 | `0x806954C0` | 192 | [`fn_806954c0`] | ten `0x154`-stride slots at `+0x548`: resolve `0x80162D00`, [`fn_806956b0`], clear handle, reset `+0xC=0.0`/`+0x10=2.0` |

## Evidence read

- Raw words for all ten bodies from `work/rels/elf/d_basesNP.elf`.
- `0x807C94D8 = 0.0`, `0x807C9560 = 4503601774854144.0` (`2^52 + 2^31`, the
  `xoris` signed-int-to-double magic), `0x807C989C = 48.0`,
  `0x807C98C0 = 0.0`, `0x807C98C4 = 32.0`, `0x807C98D8 = 1.0`,
  `0x807C98DC = 144.0`, `0x807C9E90 = 0.0`, `0x807C9E94 = 2.0`, all read from
  the ELF.
- The first two rows of both `0x8067BD00` tables were read to confirm the
  element stride (`0x807C9510 = 2.625/2.625/2.5/...`, `0x807C9538 =
  3/3/4.75/...`); the tables are indexed dynamically and are not truncated.
- `0x8042C1A0` is the `main.dol` signed-character table already used by
  [`fn_806b5a80`]; `0x80429CA0` is the `main.dol` player-mode word.

## Reused neighbours

- [`fn_806bc4f0`] / [`fn_80700ed0`] are the same body as rows 1/3/5; row 1 is
  word-for-word `fn_806bc4f0`, so the suffix-compare idiom is directly reused.
- [`fn_8067dc50`] (masked player scan) is called directly by [`fn_8067ecc0`],
  matching the wave-166 translation.
- [`fn_806956b0`] (state `1`/`2` → `0x8004A350(0.0, target)`, state `3` →
  `actor_method0(target, 0x240)`) already exists; row 10 calls it.
- [`fn_8068c450`] established the `0x43300000`/`0x807C9B38` conversion idiom
  reused by [`fn_8067bd00`].
- `80694640` confirms `0x806947C0` is the `0x802DC9A0` element constructor for
  `0x154`-byte slots, and `806960F0` confirms the `+0xC=0.0` / `+0x10=2.0`
  slot reset used by [`fn_806954c0`].

## Deferred

None.

## Confidence notes

- All ten are **high**. Row 8 carries the only ABI subtlety: Ghidra's
  unprototyped prototype prints a `double` first argument, but the sole caller
  `80685090` loads `f1` with `lfs` and passes `r4`/`r5`/`r6` as owner, index and
  flag; the translation follows the raw registers.
- Row 2's `fsubs` is the compiler's signed-byte conversion; subtracting the
  `2^52 + 2^31` constant yields an exact small integer, so the translation
  reduces it to `character as f32`, as already done in [`fn_806b5ce0`].
- `0x802D7AA0` is emitted as an ABI call (it is a `main.dol` base
  constructor); no compiler save/restore helpers appear in any of the ten.

## Files

- `work/wave188/laneA_block.rs`
- `ledger/agents/wave188-lanea.csv`
- `docs/COVERAGE_WAVE188_LANEA.md`
