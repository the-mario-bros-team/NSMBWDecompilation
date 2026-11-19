# Coverage - Wave 98 (`game::bases`, `0x8072` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** under the canonical
synthetic base **`0x80600000`** (produced by `tools/rel2elf.mjs`), never a
runtime address: the Wii REL loader chooses the real load address, so the
numbers only identify positions inside the REL image. In this image a virtual
address maps to the REL file at `file_offset = address - 0x80600000`; that is
how the float and halfword constants cited below were checked. Cross-module
pointers (`0x8037xxxx`, `0x8042xxxx`) keep their absolute values.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c` (checked with a glob,
never assumed). "Confidence" is `high` when guards, stores, constants and call
arguments are literal, and `medium` when a field role, a global, a callee or a
virtual slot had to be inferred (those bodies carry `// observed:` /
`// inferred:` / `// unknown:` notes). No body in this pass is `low`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 27 | 6,276 | 9 | 18 |

## What this pass covers

The `0x8072` bucket holds 221 functions; 95 were already in
`ledger/functions.csv` when this pass began, leaving 126. This pass takes 27
from the two clusters described below.

- **`block_fly` prop-block state bodies, `0x807263D0..0x80727A30` (15 bodies).**
  The carried block constructed at `0x807262F0` and given
  `g3d/block_fly.brres` by [`fn_80726af0`]. The class name `daPropBlock_c` sits
  at `0x8080B298`; the neighbouring wave-4 bodies call the family
  `daPowBlock_c`, and [`fn_80727bd0`] already uses the `daPropBlock_c` name for
  the same state family, so this pass uses the plain phrase "carried
  `block_fly` actor". The cluster contributes the state initialiser
  [`fn_807263d0`] (collision records `+0x818`/`+0x824`/`+0x834`, fall speeds,
  `0x80830598` state event), the contact release [`fn_80726640`], the carrier
  placement [`fn_807266b0`], the transform writer [`fn_80726750`], the
  throw-velocity selector [`fn_80726c90`], the holder rebind
  [`fn_80726fc0`], the ceiling timer [`fn_80727030`] with its block builder
  [`fn_80727190`], the hit-to-event bridge [`fn_80727220`], the three-probe
  follow logic [`fn_807272c0`], the two fall/bounce updates
  [`fn_807274d0`]/[`fn_80727a30`], the landed reset [`fn_80727670`] and the
  carried-versus-thrown dispatcher [`fn_80727720`] plus its launch transition
  [`fn_807278f0`].
- **Rotating-pedestal helper tail, `0x8072DD00..0x8072FD90` (12 bodies).** The
  pedestal whose step state was translated in wave 54 ([`fn_8072e7b0`],
  [`fn_8072eaa0`], [`fn_8072eb20`], [`fn_8072ebc0`], [`fn_8072ec80`],
  [`fn_8072ed30`]). This pass adds the five-effect overshoot state
  [`fn_8072dd00`] and its record reset [`fn_8072df90`], the model transform and
  five cached bone positions [`fn_8072dfe0`], the arm limiter
  [`fn_8072e800`], the arm-tween state machine [`fn_8072e860`], the registered
  destructor [`fn_8072f0d0`], the arm-array/range setup
  [`fn_8072f370`]/[`fn_8072f3d0`]/[`fn_8072fa90`], the release handler
  [`fn_8072f520`], the variant-matching allocator [`fn_8072fb30`] and the
  two-bit facing step [`fn_8072fd90`].

Start with [`fn_80727190`] for the shared engine-block builder (five callers),
[`fn_8072e860`] for the ring-buffer state machine, and [`fn_8072fa90`] for the
only folded integer-to-double conversion in this pass.

## Address range

`0x807263D0..0x80727A30` (15 bodies) and `0x8072DD00..0x8072FD90` (12 bodies).
After this pass the `0x8072` bucket still has 99 untranslated functions.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x807263D0` | `fn_807263d0` | 312 | medium |
| `0x80726640` | `fn_80726640` | 108 | high |
| `0x807266B0` | `fn_807266b0` | 160 | medium |
| `0x80726750` | `fn_80726750` | 368 | medium |
| `0x80726C90` | `fn_80726c90` | 304 | medium |
| `0x80726FC0` | `fn_80726fc0` | 112 | high |
| `0x80727030` | `fn_80727030` | 100 | medium |
| `0x80727190` | `fn_80727190` | 144 | medium |
| `0x80727220` | `fn_80727220` | 100 | medium |
| `0x807272C0` | `fn_807272c0` | 436 | medium |
| `0x807274D0` | `fn_807274d0` | 408 | medium |
| `0x80727670` | `fn_80727670` | 88 | high |
| `0x80727720` | `fn_80727720` | 460 | medium |
| `0x807278F0` | `fn_807278f0` | 252 | high |
| `0x80727A30` | `fn_80727a30` | 412 | medium |
| `0x8072DD00` | `fn_8072dd00` | 456 | medium |
| `0x8072DF90` | `fn_8072df90` | 80 | high |
| `0x8072DFE0` | `fn_8072dfe0` | 580 | medium |
| `0x8072E800` | `fn_8072e800` | 88 | medium |
| `0x8072E860` | `fn_8072e860` | 572 | high |
| `0x8072F0D0` | `fn_8072f0d0` | 88 | high |
| `0x8072F370` | `fn_8072f370` | 96 | high |
| `0x8072F3D0` | `fn_8072f3d0` | 96 | medium |
| `0x8072F520` | `fn_8072f520` | 72 | high |
| `0x8072FA90` | `fn_8072fa90` | 152 | medium |
| `0x8072FB30` | `fn_8072fb30` | 108 | medium |
| `0x8072FD90` | `fn_8072fd90` | 124 | medium |

## Deliberate skips

The bodies below were read and rejected rather than guessed; the rest of the
bucket was simply not reached.

- `0x807268C0` blends `unaff_GQR0`, `ldexpf` and a `CONCAT44` conversion in the
  paired-single save bracket, so its frame step cannot be reconstructed from
  the export.
- `0x80727180` exists only as `halt_baddata()` (16 bytes). It is called by
  [`fn_807274d0`] and [`fn_80727a30`], which model it as an address-only
  `abi_call1!(0x8072_7180, this)` and are therefore `medium`, not `high`.
- `0x80726260`, `0x80726290` and `0x807262C0` are the 48-byte
  `0x802DCBFC` forwarders already documented at `0x806512F0`; wave 97's policy
  of not re-emitting them was kept.
- `0x8072E520` is the paired-single emitter behind the pedestal overshoot: its
  export contains `unaff_GQR0`/`unaff_GQR3`, `ldexpf`, `CONCAT13`/`CONCAT22`
  and `in_f29/in_f30/in_f31`, so no body was emitted. [`fn_8072ebc0`] and
  [`fn_8072dd00`] reach it only by address.
- `0x8072FBA0` reads the target's fields as integers (`(float)param_2[0x2b]`)
  and stores `*param_2` as a handle in `+0x4E8`, then resolves it with
  `0x80162D00`. Whether the record's first word is a handle or a vtable is not
  provable from the export, so the body was left rather than guessed.
- `0x80727BD0` (160 bytes) and `0x807280E0` (88 bytes) are already translated
  in this file; they are the wave-4/wave-54 members of the same two clusters
  and were not touched.
- Not reached: the remaining `0x8072` helpers outside the two clusters,
  including the `0x80724020` (324), `0x807247D0` (584), `0x80724CD0` (596),
  `0x80725270` (864), `0x807255D0` (432), `0x80725830` (408), `0x8072B670`
  (600) and `0x8072BC40` (604) bodies.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (27 rows, 6,276 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 27 were on the
  bucket's untranslated list before writing.
- The constants cited in [`fn_807263d0`], [`fn_80726c90`], [`fn_807274d0`],
  [`fn_807278f0`], [`fn_8072dd00`] and [`fn_8072fa90`] were checked against the
  REL bytes at `file_offset = address - 0x80600000` (`0x807CD270` = 0.0f,
  `0x807CD274` = 8.0f, `0x807CD278` = 1.0f, `0x807CD288` = 2.0f,
  `0x807CD290` = `{1.5f, -1.5f}`, `0x807CD298` = 3.5f, `0x807CD29C` = 0.5f,
  `0x807CD2A0` = -4.0f, `0x807CD2A4` = -1.0f, `0x807CD2A8` = -0.1875f,
  `0x807CD2B0` = -0.5f, `0x807CD2B8` = `{2.5f, -2.5f}`,
  `0x807CD590` = the eight halfword yaws `0x0000, 0x2000..0xE000`,
  `0x807CD5A8` = `{8.0f, -24.0f}`, `0x807CD5B0` = 1.0f,
  `0x807CD61C` = 1.5f, `0x807CD620` = 0.08f, `0x807CD624` = 0.75f,
  `0x807CD628` = 0.04f, `0x807CD62C` = 1.15f, `0x807CD630` = 1.075f,
  `0x807CD648` = 0.0f, `0x807CD64C` = 16.0f, `0x807CD660` = -16.0f,
  `0x807CD664` = 2.0f, `0x807CD668` = `0x4330000080000000`).
- `node tools/check-lane-csv.mjs wave98` reports `wave98-bases.csv: 27 rows`
  and `OK`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/bases.rs`; the only errors in the crate at the end of the pass
  belong to the other wave-98 lanes. The file was also compiled on its own
  against `src/platform.rs` in a scratch crate, with no errors.
