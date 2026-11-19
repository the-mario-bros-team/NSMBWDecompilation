# Coverage - Wave 145 (`game::bases`)

## How to read this file

This page documents the Wave 145 pass over `src/game/bases.rs`, the semantic
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
| `src/game/bases.rs` | `game::bases` | 17 | 8,792 | 5 | 12 |

All 17 rows are `translated`; none is `partial`. The module header now reports
3644 bodies (3627 + 17), and the `0x8061` planner bucket drops from 78 to 69
untranslated addresses.

## What this pass covers

- **The eight "ready" rows of `docs/SPEC_WAVE144_BASES.md`** (8,060 bytes), each
  re-decoded from `work/rels/elf/d_basesNP.elf` at
  `(address - 0x80600000) + 0x1000` and cross-checked against the export:

  | cluster | body |
  |---|---|
  | message/help screen | `0x80609440`: eleven BRK panes, two early-outs, six message ids, `+0x284` flag mirror, second gate |
  | course/date-file table | `0x80628A00`: 70 state words, 65-entry table at `0x807C6FF0`, 16-way gate, `ceil(count/5)`, one weight debit |
  | `0x9CE8` manager | `0x8065C370`: five track words + five node floats from `0x80164B70(0)`, `+0x9CC8 = 0.0` |
  | multi-player marker actor | `0x80741510`: first active `+0x514` record, `+0x3D4` step, four player slots, oscillator, crossover sound |
  | `remo_kanaami` seat | `0x80757320`: slot `+0x63D`, mask table `0x807CE9D0`, spring/clamp ramp, step table `0x807CE9E0`, sounds `0x2F4`/`0x2F5` |
  | point-result date-file | `0x8077AC30`, `0x80786EA0`, `0x807886E0`: the command handler, the model spin/transition update and the effect command handler |

- **Nine `0x8061` planner-bucket bodies** (732 bytes), taking that bucket from
  78 to 69 untranslated addresses:

  | cluster | bodies |
  |---|---|
  | table getter | `0x8061EBD0` (`0x807C6D50`, row `+0x2D4 - 1`, `0x10` stride) |
  | list link | `0x8061CF40` (vtable `+0x0C` gate, node stored at `+0x0C`, returns `this+4`) |
  | interface show pair | `0x806149A0` (`+0x208`, block `0x80823F30`), `0x8061B630` (`+0x330`, block `0x80824B80`); both gate on `cntlzw(count-1)` == `count == 1` |
  | layout teardown | `0x8061B450` (`+0x374`/`+0x376` gates, `+0x330` and `+0x78` vtable calls, `0x8061B530` tail) |
  | layout reset | `0x8061B790` (`0x800C9380(+0x70, 1)`, `+0x378 = 0`) |
  | release wrappers | `0x80612C50`, `0x80614430` (`this != 0 && count > 0` → `0x802B9280`, returns `this`) |
  | pane show | `0x8061B6D0` (`0x800C92F0(+0x70, 0, 0)`, `[+0x36C]+0xBB |= 1`) |

## Where the binary differed from the wave-144 prose

1. **`0x80741510` multiplies by the constant `10.0`, not by `y`.** Both player
   arms ladder `li r5,10` (`0x80741674`) into `0x433000008000000A` and subtract
   `2^52 + 2^31`; `y` is only compared (`>= 0` / `<= 0`). The body uses `±10.0`.
2. **`0x80741510`'s band-zero test is the opposite pair.** The words at
   `0x807417C4..0x807417FC` zero the band on `y < 0 && x >= 0` and on
   `y >= 0 && x <= 0`, not the two windows the spec printed.
3. **`0x80786EA0`'s pool labels are swapped.** `0x807CF9E4` is `90.0` and
   `0x807CF9E8` is `270.0`; `0x807CF9C0` (frame table B) is `{2, 3, 2}`, not
   `{2, 1, 2}`. The `+0x50C == 0` transition arm uses the 270 window and angle A
   (`+0x30`), the `+0x50C != 0` arm the 90 window and angle B (`+0x3C`); the
   next-state arms pair those the other way round.
4. **`0x80757320`'s no-input arm has no `slot >= 0` guard.** The raw words at
   `0x80757354..0x80757368` load `+0x63D`, sign-extend it, call `0x80757060` and
   only then branch on the result. The spec's extra guard is not in the image.
5. **`0x8061B450` reads the second vtable at `+0x78` but passes `+0x70`.** The
   `lwz r12,0x78(r31)` / `addi r3,r31,0x70` pair is preserved verbatim.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80609440` | `fn_80609440` | 1,040 | medium |
| `0x80612C50` | `fn_80612c50` | 64 | medium |
| `0x80614430` | `fn_80614430` | 64 | medium |
| `0x806149A0` | `fn_806149a0` | 108 | medium |
| `0x8061B450` | `fn_8061b450` | 120 | medium |
| `0x8061B630` | `fn_8061b630` | 108 | medium |
| `0x8061B6D0` | `fn_8061b6d0` | 76 | high |
| `0x8061B790` | `fn_8061b790` | 60 | high |
| `0x8061CF40` | `fn_8061cf40` | 96 | medium |
| `0x8061EBD0` | `fn_8061ebd0` | 36 | high |
| `0x80628A00` | `fn_80628a00` | 972 | high |
| `0x8065C370` | `fn_8065c370` | 976 | high |
| `0x80741510` | `fn_80741510` | 1,016 | medium |
| `0x80757320` | `fn_80757320` | 1,032 | medium |
| `0x8077AC30` | `fn_8077ac30` | 1,012 | medium |
| `0x80786EA0` | `fn_80786ea0` | 1,008 | medium |
| `0x807886E0` | `fn_807886e0` | 1,004 | medium |

**Totals:** 17 bodies, 8,792 bytes. The address window runs from `0x80609440` to
`0x807886E0`.

## Notes on inference

- The twelve `medium` rows each call at least one unclaimed helper or `main.dol`
  address (`0x800FAE40`, `0x800CDC60`, `0x800CDDB0`, `0x800C9A60`,
  `0x800CE160`, `0x800CE3C0`, `0x800B4E80`, `0x800E0380`, `0x80164B70`,
  `0x80740F20`, `0x8015F470`, `0x800DF890`, `0x802DE514`, `0x80198FC0`,
  `0x80198C30`, `0x801995A0`, `0x8006A4B0`, `0x800439B0`, `0x80756FD0`,
  `0x80757060`, `0x80757140`, `0x807571E0`, `0x80757220`, `0x80757280`,
  `0x800FD920`, `0x800F1710`, `0x80103390`, `0x80105240`, `0x8015F200`,
  `0x80785600`, `0x80788AE0`, `0x80788B50`, `0x80788CB0`, `0x80789A60`,
  `0x80102E00`, `0x801037C0`, `0x8016A150`, `0x80103A10`, `0x80103A20`,
  `0x800C9380`, `0x800C92F0`, `0x800C9560`, `0x802B9280`, `0x8061B530`). Their
  argument shapes are fixed by the instruction stream; their bodies are not
  claimed.
- The `0x807CE08C`, `0x807CE090`, `0x807CE0C0`, `0x807CE0CC`, `0x807CE0D0`,
  `0x807CE0D4`, `0x807CE0D8`, `0x807CF960..0x807CF9F0`, `0x807CFA08..0x807CFA60`,
  `0x807C8AE8`, `0x807C6FF0`, `0x807E6EF8`, `0x807E6EFC`, `0x807E6F3C`,
  `0x807E6F40`, `0x807C6D50` and `0x807CE9D0`/`0x807CE9E0` pools and tables are
  read from the REL image by address; the `0x8035xxxx`/`0x8042xxxx` words are
  `main.dol` BSS and are read as zeros there, never guessed.
- `0x80786EA0`'s export carries `unaff_GQR0` and an `ldexpf` call. Both are the
  `f31` `stfd`+`psq_st` spill (`qr0`, `W=0`) at `0x80786EB4` and its restore at
  `0x8078726C`; the body contains no paired-single arithmetic and is written
  with plain `f32`/`f64` math.
- Virtual slots are dispatched through `ptr_at`, exactly as the raw
  `lwz r12,slot; mtspr CTR; bctr` sequences require.

## Deliberate skips

- **The twenty `0x802DCBFC` wrappers of the `0x8061` bucket** (`0x80610850`,
  `0x80610880`, `0x80611900`, `0x80611930`, `0x806126E0`, `0x80612710`,
  `0x80613C40`, `0x80613C70`, `0x80614EE0`, `0x80614F10`, `0x80618CA0`,
  `0x80618CD0`, `0x8061BC80`, `0x8061BCB0`, `0x8061D0B0`, `0x8061D0E0`,
  `0x8061E150`, `0x8061E180`, `0x8061FAB0`, `0x8061FAE0`) are two-instruction
  forwarders to the unresolved indirect-jump stub
  `0x802DCBFC`; writing them would require inventing the jump table, so they
  remain open, exactly as the `0x80750B90` family in
  `docs/COVERAGE_WAVE144_BASES.md`.
- **`0x80615CC0` (36 bytes) and `0x80615CF0` (36 bytes)** — both exports truncate
  on bad instruction data after a single table store; no honest body exists.
- **`0x80615D90` (100 bytes) and `0x806148F0` (144 bytes)** — same
  `halt_baddata` truncation.
- **`0x80615E00` / `0x80615E40` (60 bytes each)** — readable in themselves, but
  both call the truncated `0x80615CC0`; skipped with it so the pair is not
  published against an unreconstructible helper.
- The remaining `0x8061` bucket rows were not started; they are the natural next
  wave for this module.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (17 rows, 8,792 bytes),
  re-read from the TSV for this pass rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 17 were checked against
  the ledger's address column before being written.
- `node tools/check-lane-csv.mjs wave145` reports `OK`: 17 rows from this lane,
  all `translated`, all unique.
- `cargo check --message-format=short` reports no error or warning naming
  `src/game/bases.rs`.
