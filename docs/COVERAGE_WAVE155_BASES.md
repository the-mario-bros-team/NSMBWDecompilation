# Coverage - Wave 155 (`game::bases`)

## How to read this file

This page documents the Wave 155 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL (REL module 2). Every address is a **static
analysis address** under the canonical synthetic base **`0x80600000`**; the Wii
REL loader chooses the real load address, so these numbers identify positions
inside the REL image, not runtime addresses. Every `# Original` block in the
source carries the line
`Synthetic base: 0x80600000; this is not a runtime address.`

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee contract, a field role or a table meaning had
to be named rather than resolved.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 11 | 4,976 | 6 | 5 |

All 11 rows are `translated`; none is `partial`. The module header now reports
3,782 bodies (3,771 before this pass + 11). The 11 rows are the eight "ready"
rows of `docs/SPEC_WAVE154_BASES.md` (4,780 bytes) plus three `0x8060`-bucket
bodies (196 bytes), which takes that planner bucket from 68 to 65 untranslated
addresses.

## What this pass covers

- **The eight "ready" rows of `docs/SPEC_WAVE154_BASES.md`**, the
  604-to-588-byte band, each re-read against
  `work/ghidra/rel-d_basesNP/export/*` and the raw instruction stream with
  `file_offset = (address - 0x80600000) + 0x1000`:

  | cluster | body |
  |---|---|
  | `gabon_rock` ground contact | `0x8068D060`: re-aim, speed integration, spin direction, 16-bit step floor 192, contact/landing sounds, `-0.5` roll damping, band/mode write, landing state |
  | world-map course node | `0x80799340`: two six-slot passes, `0x800F3330` id swap, `0.05`-collinear position snap, `0x80832EB0` publish |
  | `Wm_jr_electric*` emitter | `0x807005C0`: host commit, SE `0x5B3`, four nodes of glow/spark and entries 1-3 of the line array |
  | rotating fire cannon | `0x8072B670`: variant/flip decode, `min(16*(n+4),128)/64` size factor, two motion ranges, callbacks, model/clip bind at frame `1.0` |
  | `dEasyPairing_c` panels | `0x80613680`: four-slot state machine, SE pairs `0x86..0x8D`, `StopStartWait` fallback dispatch |
  | `houdai_slope` pedestal | `0x80733490`: five parameter nibbles, `(n+1)*30` count, two `0x8007FB10` ranges, tilt/facing tables |
  | rail placement | `0x80746640`: 388-byte block clear, scene-name build, per-variant descriptor slots, type-`0x145` spawns |
  | world-map proximity gate | `0x8079BF00`: layer masks `0x1000`/`0x2000`, two latch bytes, sound flags, 192-entry sibling scan |

- **Three `0x8060`-bucket bodies** (196 bytes):

  | cluster | body |
  |---|---|
  | dDraw channel pair | `0x8060C910`: `0x800C9490(set, 1)` then `0x800C92F0(set, 2, 0)` |
  | resource release | `0x8060A040` and `0x8060F630`: `if (p != 0 && n > 0) 0x802B9280(p);` return `p` |

## Where the binary differed from the spec prose

1. **`0x8072B670`'s two motion-range arms are reversed in the spec table.** The
   raw words branch to `0x8072B7A0` when `flip == 0` (`beq` after the
   `rlwinm ...,28,31,31.` at `0x8072B700`), and that arm's second `0x8007FA40`
   call loads `f2 = [0x807CD48C] = 24.0` (`lfs 68(r31)` at `0x8072B7FC`), i.e.
   `(-8, 24, 8, 8)`. The flipped arm's second call is `(-8, 8, 8, -8)`
   (`lfs 60(r31)` / `lfs 64(r31)` at `0x8072B768..0x8072B780`). This body
   implements the binary; both Ghidra exports agree with the raw words.
2. **`0x807005C0` is written with `this: usize` taken from `r3`.** The spec
   signature is `fn()`, but the only way to model the owner passed through
   `__savegprlr_24` is the file's existing helper-preserved convention
   (`fn_80628a00`, `fn_807310c0`), so the body takes the entry register.
3. **Two arguments the export drops are kept.** `0x800C9490` and `0x800C9610`
   are printed with one argument in `80613680_FUN_80613680.c`, but `r4` still
   holds the slot word at `0x806136D0`/`0x806137D4`; the body passes it.

Confirmations rather than corrections: `0x80733490`'s seven `+0x600..+0x62C`
stores really load `0x807CD7EC = 1.0f` (not a zero sentinel); `0x8079BF00`
copies the position triple in `(x, y, z)` order despite Ghidra's reordered stack
slots; `0x80799340` calls `0x800F3330` with `(record, id)` and then `(id,
record)`; `0x8068D060` tests the angle-step floor on the `extsh` of the
truncated value.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8060A040` | `fn_8060a040` | 64 | high |
| `0x8060C910` | `fn_8060c910` | 68 | high |
| `0x8060F630` | `fn_8060f630` | 64 | high |
| `0x80613680` | `fn_80613680` | 596 | medium |
| `0x8068D060` | `fn_8068d060` | 604 | high |
| `0x807005C0` | `fn_807005c0` | 600 | medium |
| `0x8072B670` | `fn_8072b670` | 600 | high |
| `0x80733490` | `fn_80733490` | 596 | high |
| `0x80746640` | `fn_80746640` | 592 | medium |
| `0x80799340` | `fn_80799340` | 604 | medium |
| `0x8079BF00` | `fn_8079bf00` | 588 | medium |

**Totals:** 11 bodies, 4,976 bytes. The address window runs from `0x8060A040` to
`0x8079BF00`; the bucket subset is `0x8060A040..0x8060F630`.

## Notes on inference

- `0x80799340` is `medium`: the control flow, both float constants and the BSS
  word are literal, but `0x80774FA0`, `0x800F3330` and `0x80799200` stay
  address-only. `0x8079B7C0` is modelled as returning an `f64` (its export is
  `double`) and narrowed with the caller's `frsp`.
- `0x807005C0` is `medium`: all twelve element dispatches and the host commit are
  literal; `0x80199390`/`0x8006A4B0` are address-only and the host `+0x14` slot
  is `// inferred:` from its only call site. The `208.0 / 3.0` step is the
  compiler's inline literal (no pool entry); the other six constants are read
  from `0x807CC4AC..0x807CC4E0`.
- `0x80613680` is `medium`: the two tables (`0x807C6AD0`, `0x807C6AB8`) and the
  `0x802EF3A8` SE argument table are read; `0x806131D0`, `0x80613290` and
  `0x80069530` stay address-only.
- `0x80746640` is `medium`: the `0x802DCBFC` trampoline is decoded inline and
  only its `index == -1` direct branch is reachable from these three
  descriptors; `0x80746910`, `0x80747120` and `0x800B2EE0` stay address-only.
  `0x80747120` is typed `void` by Ghidra but its `r3` is read here, marked
  `// observed:` in the body.
- `0x8079BF00` is `medium`: every mask, bound and store is literal, but four
  `0x800Fxxxx` scene-API callees and the two `0x800FA070`/`0x800FA0E0`
  predicates are only fixed to "returns zero" (`countlz(v) >> 5`).
- `0x8068D060`, `0x8072B670`, `0x80733490`, `0x8060C910`, `0x8060A040` and
  `0x8060F630` are `high`: every guard, store and literal is visible in the
  export and the raw words. `0x802B9280` keeps an `// unknown:` comment.

## Deliberate skips

- The ninth and tenth rows named by `docs/SPEC_WAVE154_BASES.md`
  (`0x80695E00` and `0x8078F880`, 584 bytes each) were not started; the pass
  budget went to the eight ready rows and three bucket leaves. Both remain
  untranslated and fully specified.
- The rest of the `0x8060` planner bucket was not started (65 addresses still
  open). Its largest remaining rows are `0x80605BA0` (1,164 bytes),
  `0x8060C400` (496) and `0x8060FB50` (396); the remaining 48-byte entries are
  mostly the same release-wrapper leaf as `0x8060A040`.
- `0x80601780` and `0x806017B0` (48 bytes each) were left open: both are bare
  `0x802DCBFC` indirect-dispatch stubs that move `r3 = r4` and
  `r12 = r3 + 0x18`, which the module header already documents as an unresolved
  stub family. A body would only restate the stub.
- Nothing was written that the raw words did not support; the three deviations
  above are implemented as the binary behaves.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (11 rows, 4,976 bytes),
  re-read from the TSV for this pass rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 11 were checked against
  the ledger's address column before being written, and the lane validator
  repeats that check.
- `node tools/check-lane-csv.mjs wave155` reports `OK` (11 rows for
  `wave155-bases.csv`, 11 unique addresses in the lane).
- `cargo check --message-format=short` exits 0 and reports no diagnostic naming
  `src/game/bases.rs`.
- All 11 bodies were decoded from the raw words with
  `file_offset = (address - 0x80600000) + 0x1000`; every literal was read out of
  `work/rels/elf/d_basesNP.elf` or `work/dol/main.elf`.
