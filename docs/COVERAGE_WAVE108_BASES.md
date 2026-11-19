# Coverage - Wave 108 (`game::bases`, `0x8067` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the
`d_basesNP` REL. Every address on this page is a **static analysis address**
under the canonical synthetic base **`0x80600000`** (produced by
`tools/rel2elf.mjs`), never a runtime address: the Wii REL loader chooses the
real load address, so the numbers only identify positions inside the REL image.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c` (globbed by address
prefix, never assumed). "Confidence" is `high` when guards, stores, constants
and call arguments are literal, and `medium` when a callee identity, a field
role or a dropped argument had to be inferred; those bodies carry
`// observed:` / `// inferred:` / `// unknown:` notes. No body in this pass is
`low`.

The bucket holds 199 functions; 82 were already in `ledger/functions.csv` when
this pass began, leaving 117. This pass takes 29 of them, in two clusters plus a
two-function tail inside `0x80672E20..0x806790D0`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 29 | 4,620 | 11 | 18 |

## What this pass covers

- **The multi-pane slide actor, first half (`0x80676D40..0x80678280`, 16
  bodies).** The collision-box tick [`fn_80676d40`]; the pane burst spawner
  [`fn_80676f30`], which composes the `0x15B` argument word from the
  `0x807C92F8`/`0x807C9308` offset tables and `0x807C9318` variant table, plays
  `0x2A3`/`0x2A2` and spawns `Wm_en_burst_m`/`Wm_en_burst_s`; the camera-window
  test [`fn_80677090`]; the four-entry scale ring
  ([`fn_80677110`], [`fn_80677160`], [`fn_806771c0`], driving `1.0 -> 1.5`,
  `1.0 -> 0.75` and the `1.15`/`1.075` pulse); the `0xB4`-frame burst countdown
  [`fn_80677390`]; the registered destructor [`fn_806775b0`]; the clip reset
  [`fn_80677970`] with its `0x8008C3E0`/`0x8008C330` track pair; the
  `Wm_en_obakedoor` landing handler [`fn_80677a00`]; and the partner plumbing —
  the `0x20A` list scan [`fn_80677eb0`], adoption [`fn_80677f10`], lookup
  [`fn_80678060`], the left/right approach pair
  [`fn_806780c0`]/[`fn_806781a0`] and the grounded-player gate
  [`fn_80678280`].
- **The same actor's partner-handle state run (`0x80678A10..0x806790D0`, 11
  bodies).** Two pane steps ([`fn_80678a10`], [`fn_80678bb0`]) gated by
  `+0x4B4`/`+0x4B8`; the `(0, -1, 0)` direction and the
  `|+0x4AC - +0x4B0| / (int)+0x4C0` step [`fn_80678b20`]; and eight handlers
  that resolve the partner handle at `+0x4A4` through `0x80162D00`, raise `+0x52D`
  when it is gone, and otherwise select the state ids
  `0x8082A018`/`0x8082A098`/`0x8082A118`/`0x8082A158`/`0x8082A198`/`0x8082A1D8`
  or call one of the two approach bodies ([`fn_80678cf0`], [`fn_80678d50`],
  [`fn_80678e10`], [`fn_80678e90`], [`fn_80678f30`], [`fn_80678f90`],
  [`fn_80679050`], [`fn_806790d0`]).
- **Two `daCastleLemmyThrowBall_c` helpers (`0x80672E20`, `0x80672F00`).** The
  five-link `+0x30` chain walk [`fn_80672e20`] and the `:`-trimming name compare
  [`fn_80672f00`] that sits next to the wave-57 sibling import at `0x80673830`.

Start with [`fn_806771c0`] for the scale machine and [`fn_80678cf0`] for the
partner-handle convention the whole `0x80678xxx` run follows.

Two shared host-side helpers were added with the pass:
[`set_actor_state`] (the `+0x394` interface `+0x18` slot) and
[`actor_state_is`] (the `+0x394 -> +0x28 -> +0x14` query, whose result the
exports test with `(int)(-r | r) < 0`, i.e. "not zero").

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80672E20` | `fn_80672e20` | 224 | medium |
| `0x80672F00` | `fn_80672f00` | 136 | medium |
| `0x80676D40` | `fn_80676d40` | 40 | high |
| `0x80676F30` | `fn_80676f30` | 352 | medium |
| `0x80677090` | `fn_80677090` | 128 | high |
| `0x80677110` | `fn_80677110` | 68 | high |
| `0x80677160` | `fn_80677160` | 88 | medium |
| `0x806771C0` | `fn_806771c0` | 428 | medium |
| `0x80677390` | `fn_80677390` | 120 | medium |
| `0x806775B0` | `fn_806775b0` | 88 | high |
| `0x80677970` | `fn_80677970` | 144 | medium |
| `0x80677A00` | `fn_80677a00` | 228 | medium |
| `0x80677EB0` | `fn_80677eb0` | 88 | medium |
| `0x80677F10` | `fn_80677f10` | 332 | medium |
| `0x80678060` | `fn_80678060` | 84 | high |
| `0x806780C0` | `fn_806780c0` | 216 | medium |
| `0x806781A0` | `fn_806781a0` | 216 | medium |
| `0x80678280` | `fn_80678280` | 124 | medium |
| `0x80678A10` | `fn_80678a10` | 176 | medium |
| `0x80678B20` | `fn_80678b20` | 124 | medium |
| `0x80678BB0` | `fn_80678bb0` | 176 | medium |
| `0x80678CF0` | `fn_80678cf0` | 80 | high |
| `0x80678D50` | `fn_80678d50` | 180 | medium |
| `0x80678E10` | `fn_80678e10` | 108 | high |
| `0x80678E90` | `fn_80678e90` | 152 | high |
| `0x80678F30` | `fn_80678f30` | 80 | high |
| `0x80678F90` | `fn_80678f90` | 180 | medium |
| `0x80679050` | `fn_80679050` | 108 | high |
| `0x806790D0` | `fn_806790d0` | 152 | high |

**Totals:** 29 translated, 0 partial, 0 stub (11 high, 18 medium, 4,620 bytes).

## Deliberate skips

Read and rejected, with the reason:

- `0x80678300` (248 bytes) and `0x806787C0` (392 bytes) open with
  `unaff_GQR0` paired-single brackets and `ldexpf` calls around their transform
  loops, and `0x806787C0` also recovers its owner through the `0x802DCD70`
  save/restore thunk. The incoming GQR state cannot be reconstructed from the
  export, so the argument scaling is lost; the same reason is recorded for
  `0x80790280`/`0x80799D70` in `docs/COVERAGE_WAVE107_BASES.md`.
- `0x80678400` (100 bytes) calls `FUN_80678300()` with no arguments even though
  that body reads them from `r3`; the real argument list is lost, and the body
  is meaningless without the GQR caller it wraps.
- `0x806776D0` (48 bytes) and `0x80677700` (48 bytes) are `0x802DCBFC`
  indirect-jump stubs that tail-call through the register pair; they stay
  untranslated like the other `0x802DCBFC` bodies recorded in wave 105.

The remaining untranslated functions of the bucket were not opened in this pass
(budget, not doubt); the pass deliberately stops at `0x806790D0`.

## Notes on inference

- `0x8082A118`, `0x8082A158`, `0x8082A198` and `0x8082A1D8` are the third,
  sixth, eighth and ninth `daElasticKinoko_c` state descriptors registered by
  [`fn_80679200`] (wave 7), which is literal evidence for those four ids. The
  compared ids `0x8082A218` and `0x8082A258` lie just past that nine-entry table
  and are registered by no exported table, so their names stay `// unknown:`.
- The actor class itself is `// unknown:`. Only the pane count at `+0x530` and
  the `0xC`-stride pane array at `+0x3F0` link it to the `block_slide` loader
  [`fn_80677dd0`] (wave 57); those fields are only used by the two skipped
  frame bodies, so the bodies above never assert a class name.
- The partner handle at `+0x4A4` receives the partner's **first word**, which is
  not the vtable (class vtables in this REL live at `+0x60`). `0x80162D00`
  resolves it back to a live actor, so the body stores the word verbatim and
  marks its meaning unknown.
- `fn_80676f30` reads its two offset tables with an 8-byte stride. Ghidra prints
  an ambiguous `&DAT_807c9308 + iVar1`; the 8-byte reading is the only one that
  makes both tables a readable float pair (x offsets `0,0,8,8`, y offsets
  `8,8,0,0`) and the third table an int pair (`3,2,3,2`).
- Float constants were decoded from the REL image rather than guessed:
  `0x807C92C4 = 1.0`, `0x807C92D4 = 4.0`, `0x807C92E4 = 0.0`,
  `0x807C9308 = 8.0`, `0x807C9318 = {3,2,3,2}` as ints, `0x807C9328 = 5500.0`,
  `0x807C9374..9384 = 1.0, 0.0, 24.0, 32.0, 48.0`, `0x807C93CC = 0.0`,
  `0x807C93A0 = 1.0`. The `2^52` trick at `0x807C93E8` (`0x4330000080000000`)
  and `0x807C93A8` (`0x4330000000000000`) identifies the signed and unsigned
  int-to-double conversions used by the frame-count scaling.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (29 rows, 4,620 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 82 translated
  addresses were re-read first and none is duplicated here. The file now holds
  2,683 bodies.
- `node tools/check-lane-csv.mjs wave108` reports
  `wave108-bases.csv: 29 rows` and `OK`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/bases.rs`.
