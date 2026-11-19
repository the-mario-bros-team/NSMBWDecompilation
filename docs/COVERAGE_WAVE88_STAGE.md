# Coverage - Wave 88 (`game::stage`, the `0x800Bxxxx` course-manager run)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The evidence
column of `ledger/agents/wave88-stage.csv` points at the Ghidra C export each
body was written from. Roles that the export only hints at are marked
`inferred` or `unknown` in the body, never here.

Object layouts observed by this pass:

* **Select/message record** (`0x800B4DE0..0x800B54C0`): the busy word at
  `r13-0x5460`, the two prompt-kind bytes at `0x80315B9C`/`0x80315B9D`, the
  signed area id at `r13-0x6F4E`, the mode/flags word at `r13-0x5700` (bit
  `0x10`) and the eight-entry flag block at `[r13-0x5640]+0x20`. Kind `8` is
  the "all" entry of both the `0x800FAE40`/`0x800CE680` flag pair and of
  [`fn_800b4e80`].
* **Element node list** (behind `0x800BA240..0x800BB010`, the same list wave 75
  described): count at `+0x1C`, head at `+0x20`, next pointer at `+0x88`,
  source record at `+0x24`, interface sub-object at `+0x28`, state byte at
  `+0x80`, motion fields `+0x44`/`+0x48`/`+0x4C` (position) and
  `+0x50`/`+0x54`/`+0x58` (velocity), angle halfwords `+0x5C`/`+0x60`. The
  sub-object's `+0x18` slot takes the `0x80358C18` (settled) and `0x80358D18`
  (all-settled) descriptors.
* **Element source record** (`[node+0x24]`): distance vector at
  `+0x638..+0x640`, threshold field at `+0x62C`, angle pair at `+0x630`.
* **Teardown layouts** (`0x800B7C50`/`0x800B7DB0`): base vtable `0x803158D8`
  at `+0`, motion block at `+0x240`, linked sub-objects at `+0xC4`/`+0x168`,
  interface at `+0x48`, node at `+8`; the larger variant adds a `+0x2B0`
  interface (vtable `0x80329958`) and a `+0x278` interface.
* **Kanaami strip object** (`0x800BBD30..0x800BCB80`): instance block at `+8`,
  resource at `+0x24`, model at `+0x28`, panel object with vtable
  `0x80315EE8`, two embedded interface pairs at `+0x824`/`+0x850` (both
  switching from `0x803295D8` to `0x80329548`), `Kinopio` name pointer at
  `+0x778`, and the timed scale at `+0xDFC` with its state halfword at
  `+0xE00`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 29 | 3424 | 23 | 6 |

No body in this pass is `low` confidence and none is `partial`; every row is
`translated`.

## What this pass covers

Five untranslated runs of the `0x800Bxxxx` band, each a cluster that shares
field offsets with the neighbours:

- **Select/prompt helpers** (`0x800B4DE0..0x800B54C0`). The area-5 select
  guard with its three-part condition, the kind `0..8` emitter with the
  `0x802F0B0C` table, the two-word sound-bank copy, the two-sweep flag
  re-seeder that calls the emitter, and the index-fill plus shuffle helper that
  consumes `0x800B2EE0`.
- **Element instance wrappers and teardown** (`0x800B6BE0..0x800B7DB0`). The
  `+0x90` block probe with the sign-of-nonzero idiom, the
  `Wm_en_hanasnort` re-bind, the sound-table re-target, the `+8` tick, the
  `0x803158D8` teardown chain and its larger sibling that also resets,
  releases and deletes the `+0x2B0` interface.
- **Element passes over the node list** (`0x800BA240..0x800BB010`). The
  distance-triggered refresh, the tick sweep, the alternating side driver, the
  settled-count pass, the duplicated `0x802DCBFC` forwarder pair, the per-node
  step/retire helper and the angle-driven position seeder.
- **Kanaami strip and timed scale** (`0x800BBD30`/`0x800BBDD0`,
  `0x800BCA40`/`0x800BCB80`). The strip model constructor, the perspective
  divide, the timed scale/state writer and the panel constructor with its two
  embedded interface pairs.
- **Panel draw helpers** (`0x800BE340..0x800BE930`). The grid rebuild inside a
  fresh instance, the draw-list submit with the `0x802F8FF0` type filter, the
  transform rebuild, and the two small grid-size arithmetic helpers.

The public entry points of this pass are [`fn_800b4de0`] (select prompt),
[`fn_800ba480`] (settled-node state switch), [`fn_800bb010`] (node seeder) and
[`fn_800be7b0`] (effect draw submit).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800B4DE0` | `fn_800b4de0` | 148 | high |
| `0x800B4E80` | `fn_800b4e80` | 180 | high |
| `0x800B50F0` | `fn_800b50f0` | 84 | medium |
| `0x800B5400` | `fn_800b5400` | 176 | high |
| `0x800B54C0` | `fn_800b54c0` | 292 | medium |
| `0x800B6BE0` | `fn_800b6be0` | 56 | high |
| `0x800B6DF0` | `fn_800b6df0` | 96 | high |
| `0x800B6EF0` | `fn_800b6ef0` | 92 | high |
| `0x800B7A80` | `fn_800b7a80` | 48 | medium |
| `0x800B7C50` | `fn_800b7c50` | 180 | high |
| `0x800B7DB0` | `fn_800b7db0` | 240 | high |
| `0x800BA240` | `fn_800ba240` | 124 | medium |
| `0x800BA2E0` | `fn_800ba2e0` | 104 | high |
| `0x800BA3B0` | `fn_800ba3b0` | 180 | high |
| `0x800BA480` | `fn_800ba480` | 156 | high |
| `0x800BA580` | `fn_800ba580` | 48 | high |
| `0x800BA5B0` | `fn_800ba5b0` | 48 | high |
| `0x800BAF50` | `fn_800baf50` | 180 | high |
| `0x800BB010` | `fn_800bb010` | 116 | medium |
| `0x800BB3C0` | `fn_800bb3c0` | 64 | high |
| `0x800BBD30` | `fn_800bbd30` | 160 | high |
| `0x800BBDD0` | `fn_800bbdd0` | 52 | high |
| `0x800BCA40` | `fn_800bca40` | 24 | high |
| `0x800BCB80` | `fn_800bcb80` | 180 | medium |
| `0x800BE340` | `fn_800be340` | 80 | high |
| `0x800BE7B0` | `fn_800be7b0` | 204 | high |
| `0x800BE880` | `fn_800be880` | 132 | high |
| `0x800BE910` | `fn_800be910` | 24 | high |
| `0x800BE930` | `fn_800be930` | 20 | high |

Address range: `0x800B4DE0..0x800BE930` (five runs).

## Deliberately skipped

Skipped because the body could not be reconstructed honestly:

- `0x800B55F0` (232) and `0x800B56E0` (348) - both call `0x80082240`, which
  takes a `double` in `f1` plus the scene in `r3`. The export prints only the
  scene argument, so the `x` operand of the wrap comparison is not recoverable
  from either call site; writing either body would mean inventing it.
- `0x800B53A0` (92) - the export could not recover the jumptable at
  `0x800B53D8`/`0x800B53F8` and prints two indirect jumps through
  `[[+0x10]+0x20]` with literal `0x200`/`0x600` tags and no branch structure.
- `0x800B7B80` (64) - `FUN_802b9280()` is printed with no argument although its
  two neighbours in this run call the same `operator delete` with the object
  pointer; the pointer is not visible in this body.
- `0x800BC460` (92) / `0x800BC5A0` (40) - `FUN_80166ef0`'s first parameter is
  printed as `undefined8`, i.e. the decompiler merged the `f1` float with the
  integer registers, and `FUN_800bc4c0`'s second argument is dropped at the
  `0x800BC5A0` call site. `0x800BC4C0` itself is a GQR block.
- `0x800BBE10` (232), `0x800BC2E0` (384), `0x800BC4C0` (212), `0x800BC5D0`
  (212), `0x800BC6B0` (664) and `0x800BAE60` (240) - all six build their
  arguments through `unaff_GQR0`/`unaff_GQR3` paired-single blocks with
  `ldexpf` rounding, so the numeric order cannot be stated from the export.
- `0x800B96A0` (1008), `0x800B9A90` (936), `0x800B9FC0` (608) and
  `0x800BA5E0` (1236) - the four large element passes. They are mechanical but
  long; this pass calls the first two by address from
  [`fn_800ba240`]/[`fn_800ba2e0`] and leaves the quartet for a dedicated pass.
- `0x800B5C60` (1056) - still untranslated; wave 75 already deferred it as a
  forty-store registration routine.
- `0x800B04A0`/`0x800B04B0`/`0x800B04C0`, `0x800B3000`, `0x800B33D0`,
  `0x800B37E0`, `0x800B38D0`, `0x800B39C0`, `0x800B4730` and the rest of the
  lower band are outside this pass's five runs and were not touched.

Also left untranslated this pass, but reconstructable in a later one:

- `0x800B5C60`'s neighbours `0x800B61A0`, `0x800B6250`, `0x800B6460`,
  `0x800B6830`, `0x800B6930` and `0x800B6E50` - the contact/counter state
  machines; several of them compare `+0x34..+0x50` floats against `r2`
  thresholds and one (`0x800B6E50`) is another GQR block.
- `0x800B9570`, `0x800B95E0`, the `0x800B9600..0x800BAA00` gaps and the
  `0x800BBAB0..0x800BDD00` tail - these need the `r2` double block and the
  `0x80377C88` row table named first.
- `0x800BE390`, `0x800BEA00`, `0x800BEB50` and the `0x800BEC00..` band - the
  16x16 panel loops; `0x800BE390` alone mixes a GQR prologue with a
  `0x801C0BD0` matrix build.

## Verification

- Every address and byte size matches `functions.tsv` (29 rows, 3424 bytes).
- Every evidence file exists at the cited path under `work/ghidra/main-gb/`.
- Every row has exactly one body in `src/game/stage.rs`, and each body's
  `# Original` line cites the same address and size.
- No address in this CSV was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs`; the crate finishes with only unrelated warnings from
  other modules.
- `node tools/check-lane-csv.mjs wave88` prints `OK`; the wave-88 stage rows are
  clean.
