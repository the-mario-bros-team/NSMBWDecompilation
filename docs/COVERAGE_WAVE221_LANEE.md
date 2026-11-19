# Coverage: wave 221 lane E (`resource::archive`, `0x802C8BB0..0x802CE410`)

## How to read this file

This document covers lane E of wave 221. All ten bodies live in
`src/resource/archive.rs`; no source file was edited by this pass. The bodies
are staged in `work/wave221/laneE_block.rs` for the merge, and the ten rows are
in `ledger/agents/wave221-laneE.csv`.

Every row describes one Ghidra entry point:

- **address** - virtual address of the entry in `main.dol` (image base
  `0x80004000`);
- **name** - the Rust function appended to `src/resource/archive.rs`;
- **role** - one plain sentence;
- **status** - `translated` means the whole recorded control flow is present,
  with calls outside the batch left as absolute-address calls;
- **confidence** - `high` when every load/store/branch was read directly from
  the disassembly; `medium` when a load type, a hidden argument or an output's
  role is inferred;
- **bytes** - the `functions.tsv` size (see the size note below);
- **evidence** - the Ghidra C file under `work/ghidra/main-gb/export/`.

## Summary

| Metric | Count |
|---|---:|
| Functions translated in this batch | 10 |
| Bytes of `functions.tsv` size | 2,388 |
| `translated` | 10 |
| `partial` | 0 |
| `stub` | 0 |
| Confidence `high` | 6 |
| Confidence `medium` | 4 |
| Deferrals | 0 |

## Function table

| address | Rust function | role | status | conf. | bytes | evidence |
|---|---|---|---|---|---:|---:|---|
| `0x802C8BB0` | [`fn_802c8bb0`] | re-evaluate every entry of an instance list against one transform | translated | high | 120 | `802c8bb0_FUN_802c8bb0.c` |
| `0x802C9A30` | [`fn_802c9a30`] | construct one instance record with its vtable and defaults | translated | high | 184 | `802c9a30_FUN_802c9a30.c` |
| `0x802C9C40` | [`fn_802c9c40`] | rebuild the three cached basis vectors from a transform (five modes) | translated | medium | 484 | `802c9c40_FUN_802c9c40.c` |
| `0x802CA290` | [`fn_802ca290`] | push one instance's two cached vectors into its cell | translated | medium | 324 | `802ca290_FUN_802ca290.c` |
| `0x802CA780` | [`fn_802ca780`] | store the extended axis and normalise it, caching its squared length | translated | medium | 212 | `802ca780_FUN_802ca780.c` |
| `0x802CA880` | [`fn_802ca880`] | copy one vector into `+0x08` and re-normalise | translated | high | 28 | `802ca880_FUN_802ca880.c` |
| `0x802CD160` | [`fn_802cd160`] | release one of the 128 instance slots through its `+0x08` virtual | translated | high | 132 | `802cd160_FUN_802cd160.c` |
| `0x802CDB60` | [`fn_802cdb60`] | tail-dispatch slot `+0x08` of one instance's vtable | translated | high | 16 | `802cdb60_FUN_802cdb60.c` |
| `0x802CE2A0` | [`fn_802ce2a0`] | apply one of four preset pairs and flush after each store | translated | high | 364 | `802ce2a0_FUN_802ce2a0.c` |
| `0x802CE410` | [`fn_802ce410`] | resolve every entry name and write the id pairs of the hits | translated | medium | 524 | `802ce410_FUN_802ce410.c` |

## Size note (prologue + `blr`)

Each extent was checked against the prologue and the closing `blr` (or the
closing tail branch), not just against the `functions.tsv` column:

| address | `functions.tsv` | ending | `end + 4` | note |
|---|---:|---|---|---|
| `0x802C8BB0` | 120 | `blr` at `0x802C8C24` | `0x802C8C28` | exact; two pad words before `0x802C8C30` |
| `0x802C9A30` | 184 | `blr` at `0x802C9AE4` | `0x802C9AE8` | exact; two pad words before `0x802C9AF0` |
| `0x802C9C40` | 484 | `blr` at `0x802C9E20` | `0x802C9E24` | exact; three pad words before `0x802C9E30` |
| `0x802CA290` | 324 | `blr` at `0x802CA3D0` | `0x802CA3D4` | exact; three pad words before `0x802CA3E0` |
| `0x802CA780` | 212 | `blr` at `0x802CA850` | `0x802CA854` | exact; three pad words before `0x802CA860` |
| `0x802CA880` | 28 | `b 0x802CA780` at `0x802CA898` | `0x802CA89C` | exact; one pad word before `0x802CA8A0` |
| `0x802CD160` | 132 | `blr` at `0x802CD1E0` | `0x802CD1E4` | exact; three pad words before `0x802CD1F0` |
| `0x802CDB60` | 16 | `bcctr` at `0x802CDB6C` | `0x802CDB70` | exact; next function follows immediately |
| `0x802CE2A0` | 364 | `blr` at `0x802CE408` | `0x802CE40C` | exact; one pad word before `0x802CE410` |
| `0x802CE410` | 524 | `blr` at `0x802CE618` | `0x802CE61C` | exact; one pad word before `0x802CE620` |

None of the ten is a prologue artefact: in every row the recorded size is the
full instruction extent. The two bodies that do not end in `blr` are accounted
for: `0x802CA880` tail-branches into [`fn_802ca780`], and `0x802CDB60` is a
four-instruction `bcctr` forwarder whose tsv `16` bytes are its whole body
(there is no pad word).

## Notes that a reader should carry into the merge

1. **The `_savegpr`/`_restgpr` artefact.** `0x802C8BB0` opens with
   `bl 0x802DCD70` and `0x802CE410` with `bl 0x802DCD50`. These are the
   compiler's callee-save fragments, which Ghidra renders as a call returning
   `(r3, r4)`; the exports consequently look as if one argument came from a
   `u64`. The disassembly shows the real arguments (`r26..r28` and
   `r19..r26`), and the bodies follow it. The same artefact is documented for
   the wave-219 lane in this file.
2. **Paired-single traffic.** Every `psq_l`/`psq_st` in these ten bodies uses
   `W = 0, I = 0`, i.e. a raw float pair; the `unaff_GQR0` / `ldexpf` cloud in
   the exports is Ghidra failing to model `I = 0` and not a quantised access.
   The five `ps_*` groups decode as `ps_mul f2,f2,f2`,
   `ps_madd f0,f0,f0,f2`, `ps_sum0 f0,f0,f2,f2` (`0x802CA7D4..`), the
   `ps_muls0`-by-`r2-0x3C4C` negate pair (`0x802C9DAC..`) and the
   `ps_muls0`-by-`r2-0x3C30` scale pair (`0x802CA304..`); each is written out
   as its scalar equivalent.
3. **`r2` constants resolved.** `SDA2_BASE` is `0x80433080`, so the addresses
   this batch reads hold: `r2-0x3C50` = `0.0`, `r2-0x3C4C` = `-1.0`,
   `r2-0x3C48` = `1.0`, `r2-0x3C58` = `-10000.0`, `r2-0x3C54` = `10000.0`,
   `r2-0x3C44` = `90.0`, `r2-0x3C40` = `0.5`, `r2-0x3C3C` = `16.0`,
   `r2-0x3E3C` = `0xFFFFFFFF`, `r2-0x3E40` = `0x000000FF` and `r2-0x3C30` =
   `0xD01502F9` (`-1e10` as shipped). The bodies read them from the SDA2 band
   rather than baking in the values, matching the rest of the module.
4. **`0x802C9C40` mode split.** Modes `2`, `3` and `4` store their vectors
   directly and `b 0x802C9E0C`, i.e. they skip the three-call tail; modes `0`
   and `1` reach it. The `fcmpo` plus `bc 4,1` in the mode-1 arm sends
   "not greater than zero" (including unordered) to the `(0, 0, 1.0)` stores.
5. **`0x802CA290` third arm.** For a mode byte outside `0`/`1`/`2` the original
   branches straight to the two `0x8025F4xx` setters and reads the untouched
   stack slots. The translation zero-initialises both vectors; that is called
   out as `// partial:` in the body, because the shipped behaviour is
   uninitialised-stack dependent and has no defined value to reproduce.
6. **`0x802CA780` and `0x80237BA0`.** The out-of-line reciprocal-square-root
   step consumes the estimate in `f0` as well as the value in `f1`
   (`frsqrts f0,f0,f1`). The caller leaves `f0` equal to the value it passes in
   `f1`, so the single-argument call reproduces the shipped result, but the
   hidden register is the reason the row is `medium`; the same caveat is
   recorded for `0x80237BA0` in the wave-76 and wave-108 notes.
7. **`0x802CE2A0` duplicate arms.** Modes `2` and `3` are instruction-identical
   (both pass `1` to the second preset pair), and modes above `3` only run the
   leading `0x801C4A20` latch. The Rust `match` mirrors that shape rather than
   "tidying" the duplicate.
8. **`0x802CE410` descriptor.** In the special arm the `&0x8023B800` result is
   stored at `r1+0x10` and its address becomes the resolver's `context` word,
   while the `0x6C`-byte block filled by `0x8025C660` at `r1+0x14` becomes the
   `object`; the other arm passes the `0x802CE620` word as `context` and a null
   object. Both call `0x802C7560` with the same eight-argument shape already
   translated in this file, including the `(capacity - cursor) & 0xFFFF`
   truncation. The lane writes the `u16` index at `lane + 4*i` (the unrolled
   eight-way loop of the original covers the same entries).

## Verification

The block was appended to a scratch copy of `src/`, `Cargo.toml` and
`Cargo.lock` (`%TEMP%\opencode\scratch221e`) and checked with
`cargo check --lib`; it exits 0 and the library emits no diagnostic pointing at
any of the appended lines.
