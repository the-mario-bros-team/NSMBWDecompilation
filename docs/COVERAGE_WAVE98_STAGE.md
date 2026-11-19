# Coverage - Wave 98 (`game::stage`, the `0x800C0000` bucket)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The `evidence`
column of `ledger/agents/wave98-stage.csv` is the Ghidra C export each body was
written from. Field roles the export only hints at stay marked `observed`,
`inferred` or `unknown` in the body.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 24 | 1276 | 14 | 10 |

No body in this pass is `low` confidence and none is `partial`; every row is
`translated`.

## What this pass covers

- **The shared line-segment test** (`0x800C1E90`). Every `0x80358E90..0x803591D0`
  state record in the `dLineMng` object is entered through this one function: it
  copies the two endpoint pairs, asks `0x800C1A90` for the contact, and stores
  the result at `+0x40`/`+0x44` plus the query point at `+0x50`/`+0x54`.
  `0x800C1A90` stays an address call because its export loses two argument
  registers; the call keeps exactly the `f1` + `r3`/`r4`/`r5` set the export
  materialises.
- **The line-state ladder** (`0x800C2030`, `0x800C2070`, `0x800C2160`,
  `0x800C2220`, `0x800C22E0`, `0x800C24F0`, `0x800C2610`). Seven handlers of the
  96/128/136-byte shape `test -> if hit, transition through +0x6C -> return the
  test result`. Each one differs only in the distance limit it hands to
  [`fn_800c1e90`] (`r2-0x6844` zero, `-0x680C`, `-0x6804`, `-0x67F8`, `-0x67F4`)
  and in the `0x80358E90..0x80359190` state record it transitions to. They are
  the direct siblings of the wave-38 nine circle handlers.
- **The slide step** (`0x800C6E70`). Saves `+0x40`/`+0x44`/`+100`, runs the
  paired-single slide integrator `0x800C4F50` with the `-0x6800`/`-0x67A0`
  factors, then rolls the move back and negates `+0x38`/`+0x3C` when the contact
  query `0x800C18B0` reports a hit.
- **The two-sub-object release** (`0x800CB5D0`). Dispatches slot `+0x14` on
  `[this+4]`, then on `[this+8]` when non-null — the release path of the
  `0x800CB490` registration family.
- **The mask/player byte accessors** (`0x800CCA70`, `0x800CD220`, `0x800CD600`,
  `0x800CE100..0x800CE5F0`, `0x800CFBB0`). This tail completes the accessor
  family wave 48 opened at `0x800CE060..0x800CE0F0` and wave 71 continued:
  the `+0x68` word pair, the per-index byte arrays at `+0x10` and `+0x742`
  (the latter filled by `0x800CDED0` in a ten-entry `0x2A` loop), the three
  `+0x06` mode-byte entries (note `0x800CE5F0` is a byte-for-byte duplicate of
  `0x800CE550`), the two 4-byte no-op bodies, the `0x80317890` camera global
  (the twin of the already translated `0x800CD230`) and the `0x8035DCC0`
  message-record getter.
- **One caveat that shapes the pass.** The exports in this band are full of
  `unaff_r2` small-data reads and GQR-quantised loads. The wave-38 batch already
  named the `r2` block (`SDA2_BASE - 0x68xx`), so the bodies here reuse
  `W38_F_6844`/`W38_F_6800` and add only the constants they need.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800C1E90` | `fn_800c1e90` | 144 | medium |
| `0x800C2030` | `fn_800c2030` | 64 | medium |
| `0x800C2070` | `fn_800c2070` | 128 | medium |
| `0x800C2160` | `fn_800c2160` | 96 | medium |
| `0x800C2220` | `fn_800c2220` | 96 | medium |
| `0x800C22E0` | `fn_800c22e0` | 128 | medium |
| `0x800C24F0` | `fn_800c24f0` | 136 | medium |
| `0x800C2610` | `fn_800c2610` | 128 | medium |
| `0x800C6E70` | `fn_800c6e70` | 140 | medium |
| `0x800CB5D0` | `fn_800cb5d0` | 88 | high |
| `0x800CCA70` | `fn_800cca70` | 8 | medium |
| `0x800CD220` | `fn_800cd220` | 12 | high |
| `0x800CD600` | `fn_800cd600` | 4 | high |
| `0x800CE100` | `fn_800ce100` | 8 | high |
| `0x800CE110` | `fn_800ce110` | 8 | high |
| `0x800CE240` | `fn_800ce240` | 12 | high |
| `0x800CE250` | `fn_800ce250` | 12 | high |
| `0x800CE530` | `fn_800ce530` | 12 | high |
| `0x800CE540` | `fn_800ce540` | 12 | high |
| `0x800CE550` | `fn_800ce550` | 8 | high |
| `0x800CE560` | `fn_800ce560` | 8 | high |
| `0x800CE570` | `fn_800ce570` | 4 | high |
| `0x800CE5F0` | `fn_800ce5f0` | 8 | high |
| `0x800CFBB0` | `fn_800cfbb0` | 12 | high |

Address range: `0x800C1E90..0x800CFBB0` (six runs, 24 bodies, 1276 bytes).

## Deliberately skipped

The bucket still holds 32 untranslated functions after this pass (56 at the
start). Skipped on purpose:

- **The six handlers that also materialise an `f2` argument** (`0x800C20F0`,
  `0x800C2360`, `0x800C23F0`, `0x800C2460`, `0x800C2580`, `0x800C2690`). Their
  exports load a second float (`r2-0x6808`, `-0x6818` or `-0x6800`) into `f2`
  before calling `0x800C1E90`, but that function only declares one float and
  forwards nothing else, and `0x800C1A90` may read it as its second (bound)
  argument. The shared test's arity therefore cannot be pinned, so the five
  handlers that only set `f1` are translated and these six are not. Writing a
  body would mean guessing whether that float is used.
- **The handlers routed through `0x800C1F20`** (`0x800C21C0`, `0x800C2280`).
  `0x800C1F20` calls `0x800C1C80` with its first (double) argument dropped, so
  the value cannot be recovered; the wrappers inherit that loss.
- **`0x800C1A90`, `0x800C1C80`, `0x800C18B0`.** All three are built on paired-
  single quantised loads; `0x800C1A90` additionally calls `0x800C1A40` with only
  two of its three arguments, and `0x800C18B0` is the contact query wave 41
  explicitly left as an address call for the same reason.
- **The paired-single slide/rotation chain** (`0x800C0570`, `0x800C07B0`,
  `0x800C3A10`, `0x800C4F50`, `0x800C51A0`, `0x800C9680`, `0x800C9B50`,
  `0x800CAB30`, `0x800C75B0`). Every one either converts the `s16` angle through
  the `GQR3` quantisation branches the decompiler printed as `ldexpf` chains, or
  hangs off the `0x802DCD68`/`0x802DCD70` split-return helpers, or both. Waves
  93 and 94 already document this family as out of reach; nothing in the exports
  has changed. `0x800C9A60`/`0x800C9DF0` are the same band with a 16-argument
  varargs-style export whose register save area is never written.
- **The `0x802DCD74` registration family** (`0x800CB490`, `0x800CB670`,
  `0x800CB740`, `0x800CB760`, `0x800CB780`). All five open with the split-return
  helper and then consume both halves of its `undefined8`; `0x800CB740` and
  `0x800CB760` print their whole argument list as a single dropped argument, and
  `0x800CB670` reads its float from `extraout_f1`, a value its caller never
  passes.
- **Smaller losses.** `0x800CB5B0` is an indirect jump through a jumptable the
  export could not recover; `0x800CDDB0` calls `0x800CDC70` with two of its
  three arguments missing; `0x800C3BA0` and `0x800C8A90` are 4-byte thunks of
  already-translated functions (`0x800C3BAC` and `0x80007320`).

None of these is counted as translated in the CSV.

## Verification

- Every address and byte size matches `functions.tsv` (24 rows, 1276 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`; the 24
  addresses were chosen from the 56 open entries of the `0x800C0000` bucket.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs` (the two errors it reports are in `src/resource/bres.rs`,
  another lane's file).
- `node tools/check-lane-csv.mjs wave98` prints `OK`; the wave-98 stage rows are
  clean.
