# Coverage - Wave 105 (`game::bases`, `0x806c` bucket)

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
and call arguments are literal, and `medium` when a callee identity, a dropped
argument or a field role had to be inferred; those bodies carry
`// observed:` / `// inferred:` notes. No body in this pass is `low`.

The bucket holds 205 functions; 85 were already in `ledger/functions.csv` when
this pass began, leaving 120. This pass takes 29 of them, in three clusters
inside `0x806C07D0..0x806C6770`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 29 | 2,792 | 19 | 10 |

## What this pass covers

- **The `hashigo` interface helper group (`0x806C07D0..0x806C1B80`, 9
  bodies).** Two registered destructors ([`fn_806c07d0`], [`fn_806c0830`])
  built on `0x8015F800`/`0x802B9280`; the `+0x30` child-chain walk
  [`fn_806c0970`], whose five checks are an unrolled recursion; the
  `:`-suffix name compare [`fn_806c0a50`], which reads the object's own name
  through virtual `+0x20` and compares with `0x802E1C40`/`0x802E1AB4`; the
  free-only destructor [`fn_806c0c30`]; the interface destructor
  [`fn_806c0c70`] (vtable `0x807F7A80`, then `0x800D2400`, then the free — the
  counterpart of the `0x800D2390` interface constructor called by
  [`fn_806c0b70`]); the `+0x40C` list count [`fn_806c0ed0`]; the 32-halfword
  fill [`fn_806c1020`]; and the interface finaliser [`fn_806c1b80`].
- **The sibling actor family at `0x806C40D0..0x806C4AA0` (11 bodies).** Five
  byte-identical registered destructors — `0x806C40D0`, `0x806C4460`,
  `0x806C4560`, `0x806C4660`, `0x806C4760` — each running virtual `+0x360`,
  `0x8006CDE0`, `0x800955F0` and a `0x80072BC0(+0x1EC)` probe that rewrites
  `+0xEC`. Between them sit four clip starters
  ([`fn_806c43d0`]/[`fn_806c44d0`]/[`fn_806c45d0`]/[`fn_806c46d0`]) that
  differ only in the bone-name word (`+0x880`, `+0x884`, `+0x888`, `+0x88C`)
  and the `0x801651F0` clip index; they share the `+0x540` holder, `+0x544`
  model, `+0x584` clip and `+0x6F8` latch. The window closes with the
  per-character ball reset [`fn_806c49e0`] (character byte `+0x348`, the
  `0x807CB060` speed table, `0x8007F950` on `+0x5E0`, class virtual `+0x98`)
  and the floor-gated release [`fn_806c4aa0`].
- **The `boss_lemmy_ashiba` helper tail (`0x806C59D0..0x806C6770`, 9
  bodies).** Two registered destructors ([`fn_806c59d0`], [`fn_806c5a30`]),
  the interface getter [`fn_806c5a90`] returning `0x8082C43C`, the second
  `+0x30` chain walk [`fn_806c5b80`] and `:` compare [`fn_806c5c60`], the
  `0x6A8` allocator wrapper [`fn_806c5d80`] around the in-place constructor
  [`fn_806c62d0`], and three one-line handlers: the model virtual
  [`fn_806c61e0`] and the two animation releases [`fn_806c6210`] /
  [`fn_806c6770`] on `+0x5B4` and `+0x5C0`.

Start with [`fn_806c43d0`] (the fullest clip starter) and [`fn_806c49e0`]
(the per-character reset); [`fn_806c0970`] and [`fn_806c5b80`] are the same
chain algorithm in two classes.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806C07D0` | `fn_806c07d0` | 88 | high |
| `0x806C0830` | `fn_806c0830` | 92 | high |
| `0x806C0970` | `fn_806c0970` | 224 | high |
| `0x806C0A50` | `fn_806c0a50` | 136 | medium |
| `0x806C0C30` | `fn_806c0c30` | 64 | medium |
| `0x806C0C70` | `fn_806c0c70` | 96 | medium |
| `0x806C0ED0` | `fn_806c0ed0` | 32 | high |
| `0x806C1020` | `fn_806c1020` | 28 | high |
| `0x806C1B80` | `fn_806c1b80` | 48 | high |
| `0x806C40D0` | `fn_806c40d0` | 100 | high |
| `0x806C43D0` | `fn_806c43d0` | 120 | medium |
| `0x806C4460` | `fn_806c4460` | 100 | high |
| `0x806C44D0` | `fn_806c44d0` | 120 | medium |
| `0x806C4560` | `fn_806c4560` | 100 | high |
| `0x806C45D0` | `fn_806c45d0` | 120 | medium |
| `0x806C4660` | `fn_806c4660` | 100 | high |
| `0x806C46D0` | `fn_806c46d0` | 120 | medium |
| `0x806C4760` | `fn_806c4760` | 100 | high |
| `0x806C49E0` | `fn_806c49e0` | 172 | high |
| `0x806C4AA0` | `fn_806c4aa0` | 104 | medium |
| `0x806C59D0` | `fn_806c59d0` | 88 | high |
| `0x806C5A30` | `fn_806c5a30` | 92 | high |
| `0x806C5A90` | `fn_806c5a90` | 12 | high |
| `0x806C5B80` | `fn_806c5b80` | 224 | high |
| `0x806C5C60` | `fn_806c5c60` | 136 | medium |
| `0x806C5D80` | `fn_806c5d80` | 48 | medium |
| `0x806C61E0` | `fn_806c61e0` | 48 | high |
| `0x806C6210` | `fn_806c6210` | 40 | high |
| `0x806C6770` | `fn_806c6770` | 40 | high |

**Totals:** 29 translated, 0 partial, 0 stub (19 high, 10 medium, 2,792 bytes).

## Deliberate skips

Read and rejected, with the reason:

- `0x806C0B10` (48 bytes) and `0x806C0B40` (48 bytes) are two-word stubs that
  tail-call the unresolved indirect-jump helper `0x802DCBFC`; the same trap is
  documented in the module header and in `docs/COVERAGE_r80260_802ed.md`.
- `0x806C0F00` (276 bytes) opens with an `unaff_GQR0`/`ldexpf` paired-single
  bracket and `0x806C1040` (676 bytes) additionally reads its object through
  the `0x802DCD70` save/restore thunk; the incoming GQR state cannot be
  reconstructed from the export.
- `0x806C0CD0` (100 bytes) is byte-identical to the translated
  [`fn_806c0c70`]; it is a second class's destructor and was deferred rather
  than duplicated as a second body with an unproven owner.

The remaining 86 untranslated functions of the bucket were not opened in this
pass (budget, not doubt); this pass deliberately stops before `0x806C6E40`,
where the next repeated `88/92/224/136/48/48/136/48/48/108` family begins.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (29 rows, 2,792 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 85 translated
  addresses were re-read first and none is duplicated here. The file now holds
  2,596 bodies.
- `node tools/check-lane-csv.mjs wave105` reports
  `wave105-bases.csv: 29 rows` and `OK`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/bases.rs`.
