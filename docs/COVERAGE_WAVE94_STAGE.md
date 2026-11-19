# Coverage - Wave 94 (`game::stage`, the remaining `0x800B0000` actor-state helpers)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The evidence
column of `ledger/agents/wave94-stage.csv` points at the Ghidra C export each
body was written from. Roles the export only hints at are marked `inferred` or
`unknown` in the body.

This pass works the open entries of the `0x800B0000` bucket. The bucket is the
actor bookkeeping layer that sits between the course manager
(`0x800BB090..0x800BBC90`, landed in wave 10) and the stage-object sections
(`0x800CD470`, wave 10). Two record types recur and give the pass its structure:

* the **input record** (`0x800B5C60..0x800B6930`): id at `+0x04`, translated
  button mask at `+0x14`, previous mask at `+0x18`, newly-pressed mask at
  `+0x1C`, forced bits at `+0x20`, per-bit repeat counters at `+0x24`, and the
  sprite/velocity block at `+0x34..+0xC0`;
* the **timer record** (`0x800B8550..0x800B8EE0`): countdown at `+0x04`,
  variant selector at `+0x1C`, phase at `+0x14`, and the live-actor handle
  array at `+0x34`.

[`fn_800b5c60`] is the anchor of the first family — it is the only function in
the bucket that calls [`fn_800b61a0`] and [`fn_800b6250`] back to back, which is
what fixes the order and the field roles of the whole cluster.
[`fn_800b8550`] (start) and [`fn_800b8ee0`] (step) are the two halves of the
second family, and [`fn_800b88d0`] — already translated in wave 93 — is what
pins the `+0x1C` variant selector they share.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 28 | 4780 | 18 | 10 |

No body in this pass is `low` confidence and none is `partial`; every row is
`translated`.

## What this pass covers

- **Input-record frame refresh** (`0x800B5C60`). The twelve "last frame" copy
  pairs, the `0x803741D0` state-table split, the `0x80377C88` actor-row
  mirror, both `0x800B6080` mask translations, the derived newly-pressed mask,
  the four repeat counters, the `0x802E8620` magnitude with its `0x6AC4` floor,
  and the `+0x30`/`+0x8D` deferred-mask tail.
- **Sprite angle and orientation** (`0x800B61A0`, `0x800B6250`,
  `0x800B6930`). The `+0x8E` angle store (with its `0x8015F200` tween arm), the
  `+0x90/+0x91/+0x92` step counters with the five-frame wind-down, and the
  heading reset that also rebuilds the three `+0x80..+0x88` speed words.
- **Sprite window tests** (`0x800B55F0`, `0x800B56E0`). Two off-screen tests
  that reuse [`bg_scene_wrap_x`]: the static one against the scene limits
  `+0x3C/+0x40/+0x44`, and the scrolling one that blends `0x6AE8 /
  camera[+0x90028]` through the two `r13-0x5590`/`r13-0x5594` offsets.
- **Aspect-ratio fix** (`0x800B37E0`). The `0x801DD1D0` wide-screen flag
  selects one of two divisor pairs and divides a two-word point in place.
- **Timer record** (`0x800B8550`, `0x800B8620`, `0x800B8850`, `0x800B8EE0`).
  The start half (state mirror, `+0x2C` hold flag, `0x8006A4B0` matrix, sound
  `0x26A`), the state chooser (`0x80075270` probe plus the `2`/`3` fallback),
  the twelve-handle release loop and the two-phase step.
- **Colour blocks** (`0x800B4730`). Two twenty-byte tables built behind
  one-shot flags, then handed one entry at a time to the `+0x28` virtual setter.
- **Boss model and transform** (`0x800BABB0`, `0x800BACD0`). The
  `wanwan_boss_iggy` archive plus `magic_chainAB` animation chain, and the
  `+0x44` position / `+0x5C` angle triple bound into the `+0x04` model.
- **Small helpers** (`0x800B04A0`, `0x800B04B0`, `0x800B04C0`,
  `0x800B19B0`, `0x800B2EB0..0x800B2F00`, `0x800B7B10`, `0x800B7B80`,
  `0x800B9230`, `0x800B9570`, `0x800BC460`). Three descriptor getters, the
  `0x800C9930` sub-object destructor, the five accessors of the `r13-0x573C`
  scale record, the scaled-count submit, the guarded release, the
  `0x84`-byte animation-node teardown and the `0x2C`-stride submit loop.

The public entry points are [`fn_800b5c60`] (input refresh), [`fn_800b8550`]
(timer start), [`fn_800b8ee0`] (timer step) and [`fn_800b4730`] (colour init).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800B04A0` | `fn_800b04a0` | 12 | high |
| `0x800B04B0` | `fn_800b04b0` | 12 | high |
| `0x800B04C0` | `fn_800b04c0` | 12 | high |
| `0x800B19B0` | `fn_800b19b0` | 8 | high |
| `0x800B2EB0` | `fn_800b2eb0` | 8 | high |
| `0x800B2EC0` | `fn_800b2ec0` | 8 | high |
| `0x800B2ED0` | `fn_800b2ed0` | 12 | high |
| `0x800B2EE0` | `fn_800b2ee0` | 12 | high |
| `0x800B2F00` | `fn_800b2f00` | 8 | high |
| `0x800B37E0` | `fn_800b37e0` | 228 | medium |
| `0x800B4730` | `fn_800b4730` | 436 | high |
| `0x800B55F0` | `fn_800b55f0` | 232 | medium |
| `0x800B56E0` | `fn_800b56e0` | 348 | medium |
| `0x800B5C60` | `fn_800b5c60` | 1056 | medium |
| `0x800B61A0` | `fn_800b61a0` | 164 | high |
| `0x800B6250` | `fn_800b6250` | 328 | medium |
| `0x800B6930` | `fn_800b6930` | 184 | high |
| `0x800B7B10` | `fn_800b7b10` | 80 | medium |
| `0x800B7B80` | `fn_800b7b80` | 64 | high |
| `0x800B8550` | `fn_800b8550` | 208 | medium |
| `0x800B8620` | `fn_800b8620` | 196 | medium |
| `0x800B8850` | `fn_800b8850` | 120 | high |
| `0x800B8EE0` | `fn_800b8ee0` | 184 | high |
| `0x800B9230` | `fn_800b9230` | 196 | medium |
| `0x800B9570` | `fn_800b9570` | 56 | high |
| `0x800BABB0` | `fn_800babb0` | 280 | medium |
| `0x800BACD0` | `fn_800bacd0` | 236 | high |
| `0x800BC460` | `fn_800bc460` | 92 | high |

Address range: `0x800B04A0..0x800BC460` (thirteen runs, 28 bodies, 4780 bytes).

## Deliberately skipped

The bucket still holds 48 untranslated functions after this pass. Skipped on
purpose:

- **Paired-single (GQR) math chain** (`0x800B3000`, `0x800B33D0`,
  `0x800B38D0`, `0x800B6830`, `0x800B6E50`, `0x800B8CF0`, `0x800B9300`,
  `0x800B95E0`, `0x800B96A0`, `0x800B9A90`, `0x800B9FC0`, `0x800BAE60`,
  `0x800BD7A0`, `0x800BD970`, `0x800BE000`, `0x800BE390`, `0x800BEC00`,
  `0x800BEF00`, `0x800BFE90`). Every one builds its
  floats through `unaff_GQR0`/`unaff_GQR3` blocks with `ldexpf` rounding: that
  is the same limit wave 93 documented, and `0x800B8CF0` in particular is the
  probe helper that [`fn_800b8620`] still has to call by address.
- **Split 64-bit return of `0x802DCD68`/`0x802DCD70`/`0x802DCD74`**
  (`0x800BE690`, `0x800BE950`, `0x800BEA00`, `0x800BEB50`, `0x800BF6E0`,
  `0x800BF970`, `0x800BFA40`, `0x800BFAA0`, `0x800BFD80`). Ghidra prints these
  GX-list openers as returning `undefined8` and then uses **both** halves as
  pointers in some callers (`0x800BF6E0`, `0x800BFD80`) and only the high half
  as a pointer in others (`0x800BE950`, `0x800BE690`). No consistent reading
  exists, so the lift-panel/model animation builders are left for a pass that
  can re-check the original instruction stream.
- **Unrecovered argument lists** (`0x800B53A0`, `0x800BAE50`). Both are
  `Could not recover jumptable` virtual dispatches; the export drops the
  receiver and the argument entirely, so a body would invent the call.
- **Long ordered chains** (`0x800BA5E0`, 1236 bytes). This `0x4D4`-byte state
  machine shares the ambiguous `0x802DCD70` opener with the builders above, so
  it needs the same re-check before it can be reconstructed.
- **Tiny stubs** (`0x800B4600` 4-byte thunk, `0x800B54B0` 4-byte thunk,
  `0x800B81B0` and `0x800BBF00` empty `return;` bodies). The module-header
  policy keeps bare thunks as address calls; the two empty bodies would add a
  row with no information.

None of these is counted as translated in the CSV.

## Verification

- Every address and byte size matches `functions.tsv` (28 rows, 4780 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`; the 28
  addresses were chosen from the 76 open entries of the `0x800B0000` bucket.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs`.
- `node tools/check-lane-csv.mjs wave94` prints `OK`; the wave-94 stage rows are
  clean.
