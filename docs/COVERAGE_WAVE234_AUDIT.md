# Audit - Wave 234 (dedupe, arity repairs, batch 7)

## 1. Shared bodies deduplicated - and six real bugs found

30 addresses had a Rust body in two or more files, all but two in the
GameCube-vtable families (`0x8001D1xx`, `0x80056xxx`, `0x80065xxx`,
`0x8016xxxx`). **28 were deduplicated into 35 delegates**; the two exceptions
are deliberate:

- `0x801AB150` (`os::set_gqr`) is a separate stub for `mtspr` writes that are
  inlined inside `init_hardware_paired_single`; it has its own callers.
- `0x801AD4E0` (`dvd_report3/4`, `sdk_panic`) are three arity adapters to one
  external diagnostic routine, not copies of a translated body.

Canonical choice used the ledger's `rust_file` where a row exists, else the
copy nearest the address band. **Disassembly arbitrated six contradictions**,
and in every case blindly following the domain rule would have propagated a
wrong translation:

| Address | Bug in at least one copy |
|---|---|
| `0x80065CC0` | flag is `kind < 3` (`cmpi r3,3` / `bf cr0.lt`); copies said `== 3` and `>= 3` |
| `0x80065EC0` | sound is 23 below `1.0f` and 21 above; one copy inverted it |
| `0x800645F0` | string base is `0x80309F50`; one copy returned `0x80319F50` |
| `0x80065660` | `0x80309F5C`; two copies used `0x80319F5C` |
| `0x801627F0` | publish when the `+0x44` probe is non-zero; one copy inverted |
| `0x801625F0` | one copy inverted the third-attempt `accepted` test and could publish a destroyed handle |

Post-change scan: **0 addresses have a copied body in two files**, and no
delegate targets another delegate.

## 2. Arity defects: 50 live -> 0

The wave-233 sweep found 2,455 mismatches, of which 50 were live. Both halves
are now fixed by supplying the missing argument from context (never by
reshaping a callee):

- 18 receiver hand-offs (`call0(addr)` -> `call1(addr, recv)`) covering
  `0x80162510`, `0x80162590`, `0x801625b0`, `0x8016a010`, `0x8016a090`,
  `0x80168b90`, `0x8016a2f0`, `0x802d7aa0`;
- explicit operands in `archive.rs` and `bres.rs`;
- float-register callees `0x802be470` and `0x8000e910`, where the squared
  distance in `f1` was being dropped;
- `fn_80a9e090` now forwards the `other` actor.

Six Rust signatures grew a parameter their ABI already had, with every call
site updated.

Sweep defects found and fixed along the way:

- `input.rs:16096` was a **false positive**: `fn_80204c2c() -> (usize, u32)`
  takes no arguments, but the counter counted the tuple return.
- The sweep's comment stripper blanks code after a `/*` inside a **string
  literal**; three more `0x80162510` hand-offs were hidden there. A
  string-literal-aware re-run confirms no hidden live cases remain.

## 3. Bodies translated (32)

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| E | `.ctors` 113-126 | 10 | 18,616 |
| F | `WM_PLAYER_VTABLE` 60+ | 12 | 1,500 |

Nine of lane E's ten are state-bank installers; rather than transcribe ~2,600
instructions by eye it built a concrete interpreter for the straight-line
subset and rendered 86 descriptors mechanically, then cross-checked all 756
payload words against the DOL bytes. That caught a triplet-rotation bug in the
installer before landing.

## 4. Findings to carry forward

- **Resolved (wave 235 lane A)**: `worldmap::fn_8010fe80`'s doc claim of
  `0x80358104` is **correct**. The site is `lis r3,0x8036` +
  `addi r3,r3,-32508` (`0x3C608036` / `0x38638104` at `0x8010FE80`), and
  `addi` sign-extends its immediate, so the constant is
  `0x80360000 - 0x7EFC = 0x80358104`; `0x80368104` is the value you get by
  reading the immediate as unsigned (`ori` semantics). The lane E note above
  therefore recorded a decode artefact, not an image constant. No source change
  was needed in `worldmap`; the wave-234 slot-115 descriptor that repeated the
  artefact (`init_tables::fn_8010fc90`) was corrected instead.
- `fn_80af3850`'s doc names `extsh r0,r0` where the register is `r3` (prose
  only; the Rust recomputes the s16).
- `extsh`/`cntlzw`/`extsb` decoder operands are now all correct; a whole-image
  census (579 `cntlzw`, 574 `extsh`, 1,519 `extsb`) shows 0 malformed decodes,
  and no translated semantics were affected.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- Arity sweep: 50 live -> 0.
- Dedupe scan: 0 duplicated bodies remain.
- `scripts/check.ps1` exit 0.
