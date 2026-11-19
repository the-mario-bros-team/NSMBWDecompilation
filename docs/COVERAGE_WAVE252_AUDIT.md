# Audit - Wave 252 (descriptor newtype, die-state vtable found)

## 1. The argument-order bug is now a compile error

`interface_callN(interface, object, ...)` and
`interface_call_descN(descriptor, object, ...)` take their first two arguments in
opposite roles, and mixing them used to compile. Wave 248's reversal was exactly
that mistake.

`runtime::cpp::Descriptor(usize)` now wraps every descriptor, and
`fn_802dcbfc`, `fn_802dcbfc_target` and `interface_call_desc{2,3,4,5}` take it.
Reintroducing the swap produces `E0308: expected Descriptor, found u32`. All 25
external desc-family sites and 5 `fn_802dcbfc` sites were wrapped.

Also: `0x80746810` (`fn_80746640`, `d_basesNP`) now uses `interface_call4`
(`r3..r6 = actor, &probe, name, 50`, descriptor `0x807CE410 + 12*index`);
`interface_call5`'s doc no longer claims the world-map predicates (they call
`interface_call_desc5` with literal `0x80371810`); `interface_call_desc4` is
documented as previously dead with a single caller.

## 2. A new vtable found: the `dEn_c` die-state table

Scanning `0x80308000..0x8032F000` for pointer runs with uncovered targets:

- The largest raw run, `0x803218EC` (157 slots), is **not a vtable**: no zero
  header, no reference, and its targets are `li r3,id; blr` case stubs of the
  `FUN_801056C0` state-id switch.
- The largest real one is **`0x80313768..0x80313B14`, 235 slots**, header
  `0x80313760`, identified as the `dEn_c` **die-state** vtable by the untabled
  constructor `0x800A2D20` (`lis/addi 0x8031` / `addi 14176` / `stw +0x60`) and
  the adjacent `dEn_c::StateID_DieFumi/DieFall/DieBigFall/DieSmoke` strings.
  88 slots were uncovered; 10 translated (uncovered 88 -> 78).

Remaining largest uncovered vtables: this one (78 slots), then `0x80312750`
(`dEnBoss_c`, 77). The other large raw runs (`0x803218EC`, `0x803169A0`,
`0x8031B928`, `0x8031BF70`, `0x8032AE68`) are jump/model data, not vtables.

## 3. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| B | `dEn_c` 237-248 | 10 | 1,528 |
| C | `0x803502F8` band 0x80350740+ | 10 | 440 |
| D | vtable `0x80329958` words 3-12 | 10 | 888 |
| E | die-state vtable `0x80313768`, first 10 | 10 | ~2,412 |

## 4. Findings

- `0x80329958` is a 47-word derived vtable (45 slots) over the `0x802D7AA0`
  sub-object; extent pinned by installers (`FUN_8016CD40` writes `0x80329A14` as
  the next vptr).
- `0x80350650..0x803506EC` is now fully covered; the `0x802D4DDC` dispatcher
  indexes `0x8035066C` with the **raw** record tag, so entries map 1:1 to tags
  0..8.
- The sibling sound vtable's base is **`0x8034F0E8`**, not `0x8034F0F0`; its
  slots 5/6, 11/12, 13 corroborate the seven-argument `0x80278AB0` fallback.
- Lane E avoided a duplicate: two words it planned were covered mid-round by
  lane D's `0x80329958` work, so it took the next two instead.
- Evidence audit (lane F): 0 failures across 45 bodies and 174 table-word
  citations. Two doc fixes, and one real project disagreement:
  **`0x801406F0` is 612 B in the `main` tsv but 828 B** per `main-gb` and
  descent. Wave 253 lane A resolved it from the image: the body runs to
  `bclr 20,0` at `0x80140A28` (828 B), so the `main` row is a truncated
  artefact. See `docs/COVERAGE_WAVE253_LANEA.md`.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- Ledger 20,250 rows: 20,226 translated / 22 partial / 2 stub.
- `scripts/check.ps1` exit 0.
