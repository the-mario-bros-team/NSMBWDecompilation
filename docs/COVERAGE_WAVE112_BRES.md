# Coverage - Wave 112 (resource::bres)

## How to read this file

This pass edits only `src/resource/bres.rs` and adds one body to it. The row
below is one literal reconstruction of a Ghidra body: the doc block above the
Rust body names the original address, the byte count from
`work/ghidra/main-gb/export/functions.tsv`, and the evidence file. Offsets are
relative to the two six-float segment records the function reads.

The `0x80230000` bucket is shared with several subsystems. The Ghidra inventory
lists **251** addresses in it. Wave 109 left it at **248 of 251**; wave 111 took
one (`0x802382B0`) and left **249 of 251**. **Wave 112 takes `0x802390D0`**, so
the bucket stands at **250 of 251**, and the file as a whole holds **260**
translated bodies. `0x802389F0` is the one address still open; its reason is
under *Deliberate skips*.

- **high** - every load, store, branch and call is visible in the export and no
  argument had to be recovered.
- **medium** - a field role or callee identity had to be reconstructed from a
  sibling, a value had to be recovered from a prologue stub, or a return value
  is inferred from two call sites.
- **low** - only the instruction sequence is certain; three lost argument lists,
  a control-flow restructure across `goto`s or a table layout had to be
  modelled.

Inside the Rust file, `// observed:` marks facts read directly from the raw
instruction stream or the Ghidra C, `// inferred:` marks a role reconstructed
from neighbours, and `// unknown:` marks an unresolved field.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/bres.rs` | `resource::bres` | 1 | 688 | 0 | 1 |

Address range: `0x802390D0`.

## What this pass covers

- **Closest points of two 3D segments** (`0x802390D0`, 688 bytes). Each input is
  a six-float record: `p` at `+0x00/+0x04/+0x08` and `q` at
  `+0x0C/+0x10/+0x14`. The body forms `d1 = q1 - p1`, `d2 = q2 - p2` and
  `r = p1 - p2`, evaluates `a = |d1|^2`, `e = |d2|^2`, `b = d1·d2`, `c = d1·r`
  and `f = d2·r`, and uses `denom = a*e - b*b` for the standard
  `ClosestPtSegmentSegment` clamp. `param_3`/`param_4` receive the `s`/`t`
  parameters when non-null; the return value is the squared distance between
  `p1 + d1*s` and `p2 + d2*t`. The stage sweep at `0x80075FD0` and the
  background edge snap `bg_scene_edge_snap` at `0x80080900` are the in-repo
  callers.

### The decoding facts this pass added

1. **The `psq_l` form is plain `W=0`/`W=1` float traffic, not quantisation.**
   The instruction layout is `111000 DDDDD AAAAA WIII dddddddddddd`, so
   `e003000c` is `psq_l f0,12(r3),0,0` and `e0038014` is
   `psq_l f0,20(r3),1,0`. Every `ldexpf`/`CONCAT` block in the export is
   Ghidra's model of the GQR format, not a runtime branch. The five dot products
   are `ps_mul`/`ps_madd` finished by `ps_sum0`, in the element order
   `(z*z + x*x) + y*y`.
2. **The export's clamp soup is the standard segment-segment clamp.** The
   `bge`/`ble` pairs at `0x802391C0`, `0x802391F8`, `0x80239210`,
   `0x80239228`, `0x80239260`, `0x80239278` and `0x80239284` are exactly the
   `s < 0`, `s > 1`, `t < 0` and `t > 1` arms. The comparisons are written as
   `!(x < y)` / `!(x > y)` in the Rust body so the ordered-compare NaN behaviour
   is preserved.
3. **The return is one single, not a `double`.** The tail `ps_sum0` at
   `0x80239374` leaves the squared distance in `f1`'s first paired-single lane
   (`PS0`), and the caller at `0x80076434` compares it with the `lfs` float
   `0.1f` at `0x8042BC78`. Ghidra's `double` declaration is the decompiler's
   typing, not the ABI; the body returns `f32`.
4. **Which input owns which parameter.** `s` is computed from segment
   `param_1`'s direction and stored to `param_3`; `t` is computed from
   `param_2`'s direction and stored to `param_4` (`stfs f5,0(r5)` and
   `stfs f6,0(r6)` at `0x802392D8`/`0x802392E4`).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802390D0` | `fn_802390d0` | 688 | medium |

## Deliberate skips

- **`0x802389F0` (1 752 bytes):** the billboard/beam matrix builder that
  `fn_80238930` calls. Its 438-instruction body is dominated by six saved
  `f26..f31` doubles, two calls into `0x801C07A0`/`0x801C0500`, and two
  twelve-float scratch matrices; the export is **3 034 lines** with **126
  "removing unreachable block" markers**. Reproducing it would mean guessing
  which of the reused paired-single slots feeds each matrix element and which
  pointer field each of the two helper calls returns. It stays an address call;
  `fn_80238930` documents the argument shape it needs. Closing it needs a pass
  devoted to `0x801C0500`/`0x801C07A0` first.

## Verification

- Every address and byte size matches
  `work/ghidra/main-gb/export/functions.tsv` (re-read from the TSV, not copied
  from prose): 688.
- Every evidence file exists at the cited repo-relative path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/bres.rs`; the file now holds 260 such bodies.
- No address in `ledger/agents/wave112-bres.csv` was already in
  `ledger/functions.csv` (`0x802390D0` was absent before this pass, and the
  bucket inventory recomputed from `functions.tsv` minus the ledger listed only
  `0x802389F0` and `0x802390D0`).
- `cargo check --message-format=short` reports no error and no warning naming
  `src/resource/bres.rs`.
- `node tools/check-lane-csv.mjs wave112` reports `OK` for this lane.

`cargo check` proves the Rust is well-formed and type-correct; it does not run
the target code. The pointer arithmetic models the Wii's 32-bit big-endian
PowerPC and the paired-single lane behaviour, and must only execute on a
target-compatible runtime with the original memory image mapped.
