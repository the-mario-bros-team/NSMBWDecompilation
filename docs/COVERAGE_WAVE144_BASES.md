# Coverage - Wave 144 (`game::bases`)

## How to read this file

This page documents the Wave 144 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`**; the Wii REL loader chooses
the real load address, so these numbers identify positions inside the REL image,
not runtime addresses. Every `# Original` block in the source carries the
`Synthetic base: 0x80600000; this is not a runtime address.` line.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee contract, a field role or a virtual dispatch
had to be named rather than resolved. Comments in the bodies keep
`// observed:` / `// inferred:` / `// unknown:` markers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 16 | 9,160 | 6 | 10 |

All 16 rows are `translated`; none is `partial`. The module header now reports
3627 bodies (3613 + 16).

## What this pass covers

- **Six "ready" bodies of `docs/SPEC_WAVE143_BASES.md`** (6,556 bytes), each
  re-decoded from `work/rels/elf/d_basesNP.elf` at
  `(address - 0x80600000) + 0x1000` and cross-checked against the export:

  | cluster | body |
  |---|---|
  | `otasukeInfo` backdrop rebuild | `0x807A84B0`: two BG colours + camera depth, `{0,3,2,1}` round-up table, `+0x180` rect, both owner slots |
  | 0x16518-stride area scan | `0x80772370`: three passes, sentinel `0x800F56B0`, keys at `element+1`/`element+5` |
  | player-contact machine | `0x806639A0`: five states, 80/20 countdown, two Y windows, name-gated `0x80663DF0` |
  | four-slot join panel | `0x8074C540`: two 4-bit walks, 9-bit confirmed mask, draw id `159` |
  | ten-pane layout pass | `0x8075D5C0`: three hinges + ten panes, four matrix sites, `0x800F6910` tail |
  | world-map event handler | `0x80763E50`: ids `0x5F`/`0x15..0x19`, open + poll, effects `39`/`40`/`41` |

- **The two "conditional" spec rows** (2,148 bytes), written because the single
  open input of each turned out to be transcribable verbatim rather than
  guessable:

  | body | how the open input is handled |
  |---|---|
  | `0x806765B0` | the signed-halfword read at `0x807C92C0 + 2*f0` is written as the `u16` read it is; `+0x5E0` is never read back in this body, so no meaning is invented |
  | `0x807BEF60` | the four layout owners and three rect setters are transcribed with their literal arguments; no pane role is assigned, which is why the row stays `medium` |

- **Eight `0x8075` planner-bucket bodies** (456 bytes), taking that bucket from
  79 to 70 untranslated addresses:

  | cluster | bodies |
  |---|---|
  | release/free helper | `0x80757900` (`0x8015F800` detach, `0x802B9280` free, returns `this`) |
  | `+0x188` initialiser | `0x8075B6B0` (`0x800F5FD0`, class word `0x80814568`, `0x80069020`) |
  | flag setters | `0x8075B8B0` (`+0x1A8 = 0`), `0x8075B8D0` (`+0x1A4 = +0x1A8 = 1`) |
  | clamp wrappers | `0x80752CA0` (`+0xB0`), `0x80752D10` (`+0xAC`) through `0x80752AC0` |
  | timer | `0x80753B20` (`+0x1A498` down, `0x807537D0` at zero) |
  | effect/handle refresh | `0x80753780` (`0x800EDEB0(+0x514)`, `0x800EBB30` on `+0x1A4B8`, returns `1`) |

## Where the binary differed from the wave-143 prose

1. **`0x80772370` pass 3 has no `else`.** The spec claimed the two sub-passes
   were mutually exclusive. The raw words at `0x807726d4` are reached both from
   the "3a guard failed" exit and from the end of the 3a loop, so the second
   sub-pass runs independently. The body implements two `if`s.
2. **`0x806639A0` compares the wrong field in the spec.** The `4 < state` test
   in the region-set arm loads `+0x360` from the `0x80162D00(*(this+0x3D8))`
   object (`r25`), not from `this` (`r31`). The body uses the object.
3. **`0x8074C540`'s draw flag is bit 31**, not bit 0: the raw draw argument is
   `(mode << 12) | (use_base << 31) | mode_byte`. The draw path also returns `0`
   *after* the nine-slot LCG read, so that read still advances the generator.
4. **`0x8075D5C0`'s `0x800F6910` call** is `(r3 = this, r4 = 0x80814798,
   f1 = -40.0, f2 = 0.8)`; the block address is not just an `lfs` base, it is
   still in `r4` at the call, so the body passes it.
5. **`0x807A84B0`'s pool index:** `0x807D0678` is `1.0` and `0x807D067C` is
   `20000.0`; `0x807D0670` is `0.0` and `0x807D0690` the `{0,3,2,1}` table.
   The spec's offsets were right but two labels were swapped.
6. **`0x807BEF60`'s screen word is `+6`** for the extent and `+4` for the
   scaled value, matching `0x807A84B0`'s `+4`; the double dereference through
   `0x8042A480` is preserved (two `lwz` in the raw stream).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806639A0` | `fn_806639a0` | 1,096 | medium |
| `0x806765B0` | `fn_806765b0` | 1,052 | medium |
| `0x8074C540` | `fn_8074c540` | 1,092 | medium |
| `0x80752CA0` | `fn_80752ca0` | 52 | high |
| `0x80752D10` | `fn_80752d10` | 52 | high |
| `0x80753780` | `fn_80753780` | 72 | medium |
| `0x80753B20` | `fn_80753b20` | 96 | high |
| `0x80757900` | `fn_80757900` | 88 | high |
| `0x8075B6B0` | `fn_8075b6b0` | 68 | medium |
| `0x8075B8B0` | `fn_8075b8b0` | 12 | high |
| `0x8075B8D0` | `fn_8075b8d0` | 16 | high |
| `0x8075D5C0` | `fn_8075d5c0` | 1,080 | medium |
| `0x80763E50` | `fn_80763e50` | 1,056 | medium |
| `0x80772370` | `fn_80772370` | 1,108 | medium |
| `0x807A84B0` | `fn_807a84b0` | 1,124 | medium |
| `0x807BEF60` | `fn_807bef60` | 1,096 | medium |

**Totals:** 16 bodies, 9,160 bytes. The address window runs from `0x806639A0` to
`0x807BEF60`.

## Notes on inference

- The ten `medium` rows each call at least one unclaimed helper or main.dol
  address (`0x800B54B0`, `0x802C6AE0`, `0x802BF420`, `0x802BFFF0`,
  `0x802C0150`, `0x802C6C70`, `0x802D1210`, `0x802DCC4C`, `0x800F56B0`,
  `0x800F3270`, `0x800F57F0`, `0x800F5810`, `0x800F5930`, `0x80774BD0`,
  `0x80774D00`, `0x8004DB40`, `0x80052080`, `0x8005CA50`, `0x80162D00`,
  `0x80064610`, `0x800B2EE0`, `0x801C0BD0`, `0x8016EF50`, `0x8016A170`,
  `0x8016A150`, `0x8016A1A0`, `0x80168400`, `0x80168180`, `0x800F6910`,
  `0x800F60E0`, `0x800FD800`, `0x800FD820`, `0x80169020`, `0x80105240`,
  `0x8015F730`, `0x8007FB10`, `0x8007F900`, `0x80676E10`, `0x800F5FD0`,
  `0x80069020`, `0x800EDEB0`, `0x800EBB30`, `0x80752AC0`, `0x807537D0`). Their
  argument shapes are fixed by the instruction stream; their bodies are not
  claimed.
- The `0x807C8B80`, `0x807CE620`, `0x807CEC48`, `0x807D0670`, `0x807D0A68`,
  `0x807C92B0`, `0x807CEE80` and `0x80814798` literal pools are read from the
  REL image by address; the `0x8035xxxx`/`0x8042xxxx` words are `main.dol` BSS
  and are read as zeros there, never guessed.
- Virtual slots are dispatched through `ptr_at`, exactly as the raw
  `lwz r12,slot; mtspr CTR; bctr` sequences require.

## Deliberate skips

- **`0x807571C0` (28 bytes)** — the export truncates on bad instruction data;
  no body can be reconstructed.
- **The `0x80750B90`/`0x80750BC0` family (48 bytes each)** — every one is a
  two-instruction wrapper around `0x802DCBFC`, the unresolved indirect-jump stub
  already documented in this module's header. Writing them would require
  inventing the jump table, so all of them remain open.
- **`0x80754620` (80 bytes)** — the `GQR3`-ladder body (`s16 << 9`, scale
  `0x3ff00000`) was read but is left open: the `ldexpf` reconstruction cannot be
  validated against the pair-store convention without a runtime check.
- Remaining `0x8075` bucket rows above 200 bytes (`0x8075F270`, `0x8075C6F0`,
  `0x80752730`, `0x80758880`, `0x8075BB60`, `0x8075B040`, `0x807583D0`,
  `0x807586D0`, `0x8075BD20`, `0x80753B80`, `0x8075C550`, `0x8075C070`,
  `0x8075B2B0`, `0x80758010`, `0x80754430`, `0x807539E0`, `0x807529B0`,
  `0x8075FA40`, `0x807537D0`) were not started; they are the natural next wave.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (16 rows, 9,160 bytes),
  re-read from the TSV for this pass rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 16 were checked against
  the ledger's address column before being written, and none is duplicated.
- `node tools/check-lane-csv.mjs wave144` reports `OK`: 16 rows from this lane,
  all `translated`, all unique across both translation lanes.
- `cargo check --message-format=short` reports no error or warning naming
  `src/game/bases.rs`.
