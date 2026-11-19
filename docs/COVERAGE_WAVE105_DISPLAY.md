# Coverage - Wave 105 (engine::display)

## How to read this file

`src/engine/display.rs` is the semantic home of the engine-level 2-D
presentation layer. This pass closes the `0x800D0000` bucket, the last bucket of
this lane: it entered with 18 untranslated functions of its 251 and leaves 11.
"Bodies" counts `pub unsafe fn` functions; "bytes" is the sum of the Ghidra
sizes re-read from `work/ghidra/main-gb/export/functions.tsv` for this pass.

"high" means every store, branch and call is visible in the export. "medium"
means one register or callee role had to be recovered from the export's
`extraout_*` placeholder plus the surrounding call sites, and that recovery is
spelled out in the body's `# Confidence` block.

Only `src/engine/display.rs` was edited. `ledger/functions.csv` is read-only and
the coordinator merges this lane's rows.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/display.rs` | `engine::display` | 7 | 1368 | 1 | 6 |

## What this pass covers

Five clusters, all in `0x800D0470..0x800DFF30`:

- **The effect-record list** (`0x800D0470`, 360 bytes). [`fn_800d0470`] walks the
  intrusive `0x24`-byte records headed at `r13 - 0x56B0` (the list already
  drained by [`fn_800d0340`]): `+0x00` previous, `+0x04` next, `+0x08` self,
  `+0x0C..+0x14` position, `+0x18` kind, `+0x1C` extra and a `u16` code at
  `+0x20`. A matching record is retargeted in place; otherwise a record is
  allocated through `0x802B9210(0x24)` and appended through `0x80160FE0`. The
  export prints the position copy as `extraout_r4`; the function's only caller
  (`0x800369D0`) passes the same `+0x6E8` position pointer that the search
  compares, so `r4` is the `position` argument.
- **The water pool** (`0x800D2CF0`, 136 bytes). [`fn_800d2cf0`] is the missing
  "first free element" half of the wave-20 pool: ten elements of `0x13C` bytes
  starting at `pool + 4`, each offered to [`water_effect_spawn`], with the
  `1`/`0` result returned through `0x802DCDC0`. Its `extraout_f1` scale is
  recovered from the two callers (`0x8012FB10`, `0x8012FCF0`), which pass a
  float, and from [`water_effect_spawn`]'s `double` first parameter.
- **The four-slot record bands** (`0x800D8B00`, 164 bytes, and `0x800D8C60`, 204
  bytes). [`fn_800d8b00`] writes one band pair for a single object/player and
  then arms the `+0x3C`/`+0x40` countdown words; [`fn_800d8c60`] writes the same
  bands for every connected player slot (queried through `0x800610E0`) plus the
  `0xFF` "all players" marker. Both reach the band writers
  [`fn_800d8bb0`]/[`fn_800d8e40`] through the `0x802DCD70`/`0x802DCD74`
  register-save pair, and the callers pass the object and slot explicitly.
- **The replay interface pair** (`0x800DFA40`, 180 bytes, and `0x800DFF30`, 320
  bytes). Both are the callees the wave-93 replay helpers already call by
  address: [`fn_800dfa40`] validates the `+0x04` interface with the
  `0x802F86C8` tag, builds the `0x80168400` key and hands a nine-word record to
  slot `+0x50`; [`fn_800dff30`] validates with the `0x802F8FF0` tag and walks
  the `+0xE8` container, publishing one id per child through
  [`save_banner_set_slot_visible`].
- **The four-byte thunk** (`0x800D8670`, 4 bytes). [`fn_800d8670`] is a single
  branch to `0x800D8680`; its export holds the target's body, so the Rust body
  forwards the one argument to [`fn_800d8680`] instead of duplicating it.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800D0470` | `fn_800d0470` | 360 | medium |
| `0x800D2CF0` | `fn_800d2cf0` | 136 | medium |
| `0x800D8670` | `fn_800d8670` | 4 | high |
| `0x800D8B00` | `fn_800d8b00` | 164 | medium |
| `0x800D8C60` | `fn_800d8c60` | 204 | medium |
| `0x800DFA40` | `fn_800dfa40` | 180 | medium |
| `0x800DFF30` | `fn_800dff30` | 320 | medium |

Byte total: `360 + 136 + 4 + 164 + 204 + 180 + 320 = 1368`.

## Deliberate skips

11 addresses of the bucket stay untranslated. Every one is a concrete export
defect, not a budget cut.

**Paired-single / GQR quantisation (9 addresses).** `0x800D0610`, `0x800D19E0`,
`0x800D2460`, `0x800D2690`, `0x800D27B0`, `0x800D3380`, `0x800D43E0`,
`0x800D9D70` and `0x800DAA10` are printed with `unaff_GQR0`/`unaff_GQR3`,
`ldexpf`, `CONCAT13`/`CONCAT22` stubs, `in_f17..in_f31` / `in_ps17_1..in_ps31_1`
inputs and `WARNING: Removing unreachable block` lines around the float work.
`0x800D19E0` alone carries 15 dropped paired-single inputs. Any Rust body would
be an invented approximation of `psq_l`/`ps_merge` semantics, so they stay open.
The nine are the same group wave 93 listed, re-confirmed against the exports for
this pass.

**Unrecovered jump table (1 address).** `0x800D6550` prints
`Could not recover jumptable at 0x800d6590 ... Treating indirect jump as call`
and its dispatch target is not visible; the two-float argument it builds from
`0x80370DCC` is only reachable through that lost dispatch.

**No observable body (1 address).** `0x800DFDD0` is a pure `+0x64` self-relative
list walk with no store, no call and no return value in its own export, while
all five of its callers use its result (for example [`fn_800dff30`] reads
`+0x0C` of the returned node). Reconstructing it would mean inventing the
returned node and the loop's effect.

## Verification

- Every address and byte size matches `functions.tsv`, re-read for this pass;
  the 7 `# Original` blocks total `1368` bytes.
- Every evidence file exists at the cited path, including
  `800d8670_thunk_FUN_800d8680.c`, whose Ghidra name is the folded target.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/engine/display.rs`.
- No address was already in `ledger/functions.csv`; the bucket had 18
  untranslated functions before this pass and 11 after it.
- `node tools/check-lane-csv.mjs wave105` reports `OK`; this lane's CSV has 7
  unique rows and no duplicate address.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/engine/display.rs`. The parallel lanes' errors (`runtime::dvd`,
  `audio::ax`, `game::stage`, `resource::archive`) are outside this file.
