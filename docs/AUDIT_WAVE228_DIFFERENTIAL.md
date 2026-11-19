# Wave 228 — differential verification of the untabled pointer-table bodies (read-only)

Answer to one question: **do the bodies wave 228 added to `src/render/geometry.rs`
and `src/runtime/input.rs` compute what the shipped PowerPC code computes?**

Read-only lane. No Rust file, ledger CSV or existing document was modified; the
only files written are this report and the harness in `work/wave229/`. Both
target files are unchanged since wave 228 wrote them (`geometry.rs` mtime
11/15 1:20 PM, `input.rs` 11/15 1:16 PM, both before this lane started).

Method: `work/wave228/laneF_diffcheck.mjs` was adapted into
`work/wave229/laneE_diffcheck.mjs`. For every body it interprets the raw words
from `work/dol/main.elf` and the emitted Rust body on the same pseudo-random
inputs and compares the results. Changes from the wave-228 harness:

- the Rust side is parsed from the **shipped source file**, not from the lane's
  scratch block, so the check covers what actually landed;
- the `const F32_*_228` declarations are resolved from that same file and their
  **values are read out of `main.dol`**, so a wrong constant address shows up as
  a mismatch instead of being masked by a hard-coded value table;
- both sides are evaluated in single precision (`Math.fround` after every
  operation), matching `lfs`/`stfs`/`fadds`/`fmuls` against Rust `f32`;
- 256 trials per body (seeded PRNG) instead of 40, and the measured extent is
  compared with the extent the doc block claims.

## Geometry kernels — `src/render/geometry.rs`

38 bodies, all modelled. **9,728 evaluations, 0 mismatches.** Every trial matched
bit-for-bit (`maxrel` is 0 for all bodies), and every measured extent equals the
extent declared in the body's own doc block.

| body | extent (B) | trials | mismatches | exact | maxrel |
|---|---:|---:|---:|---:|---:|
| `0x80254be0` | 56 | 256 | 0 | 256 | 0 |
| `0x80254c20` | 148 | 256 | 0 | 256 | 0 |
| `0x80254cc0` | 56 | 256 | 0 | 256 | 0 |
| `0x80254d00` | 188 | 256 | 0 | 256 | 0 |
| `0x80254dc0` | 176 | 256 | 0 | 256 | 0 |
| `0x80254e70` | 80 | 256 | 0 | 256 | 0 |
| `0x80254fa0` | 120 | 256 | 0 | 256 | 0 |
| `0x80255020` | 304 | 256 | 0 | 256 | 0 |
| `0x80255150` | 40 | 256 | 0 | 256 | 0 |
| `0x80255180` | 404 | 256 | 0 | 256 | 0 |
| `0x80255320` | 336 | 256 | 0 | 256 | 0 |
| `0x80255470` | 136 | 256 | 0 | 256 | 0 |
| `0x80255500` | 424 | 256 | 0 | 256 | 0 |
| `0x802558f0` | 56 | 256 | 0 | 256 | 0 |
| `0x80255930` | 136 | 256 | 0 | 256 | 0 |
| `0x802559c0` | 56 | 256 | 0 | 256 | 0 |
| `0x80255a00` | 160 | 256 | 0 | 256 | 0 |
| `0x80255b70` | 80 | 256 | 0 | 256 | 0 |
| `0x80255c90` | 120 | 256 | 0 | 256 | 0 |
| `0x80255d10` | 272 | 256 | 0 | 256 | 0 |
| `0x80255e20` | 36 | 256 | 0 | 256 | 0 |
| `0x80255e50` | 316 | 256 | 0 | 256 | 0 |
| `0x80255f90` | 300 | 256 | 0 | 256 | 0 |
| `0x802560c0` | 136 | 256 | 0 | 256 | 0 |
| `0x80256150` | 348 | 256 | 0 | 256 | 0 |
| `0x80256510` | 72 | 256 | 0 | 256 | 0 |
| `0x80256560` | 172 | 256 | 0 | 256 | 0 |
| `0x80256610` | 56 | 256 | 0 | 256 | 0 |
| `0x80256650` | 180 | 256 | 0 | 256 | 0 |
| `0x80256710` | 204 | 256 | 0 | 256 | 0 |
| `0x802567e0` | 88 | 256 | 0 | 256 | 0 |
| `0x80256840` | 204 | 256 | 0 | 256 | 0 |
| `0x80256910` | 128 | 256 | 0 | 256 | 0 |
| `0x80256ac0` | 36 | 256 | 0 | 256 | 0 |
| `0x80256af0` | 324 | 256 | 0 | 256 | 0 |
| `0x80256c40` | 320 | 256 | 0 | 256 | 0 |
| `0x80256d80` | 148 | 256 | 0 | 256 | 0 |
| `0x80256e20` | 356 | 256 | 0 | 256 | 0 |

Extent sum: 6,772 B for the 38 new bodies (7,684 B for the whole 42-slot block
including the four targets already translated in `src/runtime/os.rs`).

Instruction coverage used by the raw interpreter: `lfs`, `stfs`, `fadds`,
`fsubs`, `fmuls`, `fdivs`, `fmadds`, `fmsubs`, `fnmsubs`, `fneg`, `fabs`,
`frsp`, `addi`/`addis`, `lis`, `stwu`, `lwz`/`lwzu`, `stw` (frame and LR traffic
only in these bodies), `mr`, `mfspr`/`mtspr` (LR bookkeeping), `blr`, `crxor`,
and `bl 0x80237CA0`. No body needed an instruction outside that set, so no body
was skipped for decoding reasons.

### The check is not vacuous

Two control experiments are run on the same data:

- Perturbing a single output component of a body's result by `1e-3` is detected
  on **38/38** bodies, i.e. the comparison really rejects a translation error of
  that size.
- Evaluating a raw body against the *neighbouring* body's Rust produces a
  mismatch on **38/38** bodies, i.e. each raw body is being matched against its
  own translation and not against something degenerate.

## Input callbacks — `src/runtime/input.rs`

12 bodies. **1 modelled, 11 not modelable by this interpreter.** The measurement
is not a "0 mismatches" claim for the unmodelled ones — it establishes that the
lane-F interpreter cannot even start on them, and why.

| body | extent (B) | words | branch | `bl` sites | loads w/h/b | stores w/h/b | blocker |
|---|---:|---:|---:|---:|---:|---:|---|
| `0x801e6d10` | 832 | 208 | 22 | 10 | 12/1/18 | 7/2/18 | `bl` at `0x801e6d20` |
| `0x801e7380` | 320 | 80 | 4 | 6 | 3/1/8 | 2/1/2 | `bl` at `0x801e7390` |
| `0x801e8270` | 92 | 23 | 2 | 0 | 2/0/4 | 1/1/3 | `bc` at `0x801e82a4` |
| `0x801e82d0` | 260 | 65 | 2 | 0 | 3/0/10 | 1/4/3 | `bc` at `0x801e8304` |
| `0x801e83e0` | 792 | 198 | 18 | 4 | 4/1/25 | 1/7/9 | `bc` at `0x801e8438` |
| `0x801e8700` | 332 | 83 | 2 | 1 | 3/0/11 | 1/4/3 | `bc` at `0x801e8748` |
| `0x801e8850` | 480 | 120 | 15 | 3 | 3/0/17 | 1/5/12 | `bc` at `0x801e88a4` |
| `0x801e8ee0` | 940 | 235 | 16 | 7 | 5/1/25 | 1/7/6 | `bl` at `0x801e8ef0` |
| `0x801e9290` | 276 | 69 | 6 | 4 | 4/0/3 | 0/0/4 | `bc` at `0x801e92d4` |
| `0x801e93b0` | 436 | 109 | 6 | 6 | 3/0/8 | 1/4/4 | `bl` at `0x801e93c0` |
| `0x801e9570` | 432 | 108 | 7 | 6 | 3/0/8 | 1/4/4 | `bl` at `0x801e9580` |
| `0x801e9720` | 4 | 1 | 0 | 0 | 0/0/0 | 0/0/0 | none |

Extent sum: 5,196 B (4,840 B for the nine prologue-bearing bodies plus the 356 B
of leaf/no-op targets), matching `work/wave228/laneE_tables.md`. Every measured
extent ends on the documented `blr` and equals the documented size.

### Why 11 bodies are not modelable here

The lane-228 interpreter is a straight-line arithmetic model: 32 float registers,
an eight-float `dst`, a five-float `src`, and nothing else. These decoders are
the opposite shape — their observable effect is *global memory*, and their
control flow depends on what they read:

- **Branch on global state.** Every one of the 11 contains at least one `bc`
  whose condition is a byte or word of the WPAD control block
  (`control + 0x8dc`, `+0x840`, `+0x8c1`, `+0x98d`, the `r13-0x4bb4`/`-0x4bb8`
  handshake pair, …). A model without that memory cannot pick a path, so it
  cannot produce a result to compare.
- **Calls out.** Direct `bl` targets in these bodies are the
  `0x802DCD58`..`0x802DCDC0` register-save / call-bridge thunks (what
  `platform::callN` stands in for), `memcpy` (`0x80004364`), `memset`
  (`0x800046b4`), interrupt disable/restore (`0x801b1140` / `0x801b1180`), the
  `OSReport` printer (`0x801ea8f0`), the XOR decoder (`0x801ea520`), a `memcmp`
  helper (`0x802df098`) and eight already-translated KPAD helpers
  (`fn_801e4c50`, `fn_801e4ec0`, `fn_801e5240`, `fn_801e53f0`, `fn_801e74c0`,
  `fn_801e7960`, `fn_801e7be0`, `fn_801e80b0`). Reproducing the effect would mean
  modelling the whole WPAD state machine, not one body.
- **The measured traffic is global, not stack.** After excluding `r1`-relative
  traffic (frame, LR saves, the local descriptors), the bodies still perform up
  to 31 non-stack loads and 27 non-stack stores each — into the per-channel
  control block, the SDA latches and the `out` report block.

The interpreter therefore reports the first blocking instruction per body (the
`blocker` column above) and stops. Attempting a numeric comparison would require
a full integer/CR/memory emulator plus a Rust-subset interpreter for `if`/`while`
and helper calls; a partial model would produce harness artefacts, not evidence.

### The one body that was modelled

`0x801e9720` (table slots 3–15 and 24–28) is the single word `0x4E800020`
(`blr`) and the emitted body is
`pub unsafe fn fn_801e9720(_channel: u32, _report: usize, _out: usize) {}`.
Raw: no reads, no writes, no return value. Rust: no reads, no writes, no return
value. **0 mismatches; the body is an identity over the whole address space.**

## Structural cross-checks on the 11 unmodelled bodies

A numeric comparison being impossible is not the same as "unchecked". Three
mechanical checks were run on the emitted source and are reported here so the
gap is bounded:

- **Call coverage: 0 uncovered direct calls.** Every direct `bl` target in the
  11 bodies resolves either to the `0x802DCDxx` save/bridge thunks, to a
  translated `fn_<addr>` present in the emitted body, or to a recognised library
  helper with a documented Rust idiom (`MEMCPY`, `MEMSET`, `disable_interrupts`,
  `restore_interrupts`, `OS_REPORT`, `KPAD_XOR_DECODE`/`fn_801ea520`,
  `fn_802df098`). No emitted body references a translated function its raw body
  does not call, and none omits a call its raw body makes.
- **No stub bodies.** None of the 11 contains `todo!()`, `unimplemented!()` or
  `unreachable!()`; all have a substantive body (17–120 statements excluding
  comments).
- **Load/store census** (raw non-stack vs emitted `platform::read_*`/`write_*`)
  is recorded in `work/wave229/laneE_input_census.json`. The two numbers do
  **not** and should not agree one-for-one: the raw count is static over all
  paths (including not-taken branches) and counts every `lhz` even when the same
  value is reused, while the Rust folds repeated reads into locals and replaces
  `memcpy`/`memset` with calls or loops. They are stated as coverage, not as a
  pass criterion.

## Finding (documentation only)

`work/wave228/laneF_tables.md` says the measured extents of the 42-slot block
"Sum ...: 7,920 B". The slot table in that same file sums to **7,684 B** (38 new
bodies 6,772 B + `0x80254EC0` 220 + `0x80255AA0` 200 + `0x80255BC0` 200 +
`0x80256990` 292). 7,920 is an arithmetic slip; the code and the per-slot extents
are unaffected. No source or doc change was made (read-only lane) — the owning
lane should correct the total if it still matters.

Everything else checked out: the 38 kernels are individually bit-exact against
the image, all 50 measured extents match their doc blocks, none of the 50
addresses appears in `work/ghidra/main/export/functions.tsv`, and none appears in
`ledger/functions.csv` (which stays at 20,250 rows), as wave 228 intended.

## What this proves and what it does not

Proves: for the 38 arithmetic kernels, the emitted Rust is a bit-exact model of
the raw body **under the modelled environment** — same five `src` floats in, same
eight `dst` floats out, for 256 random inputs each, with a comparator shown to
catch a `1e-3` error and to reject the neighbouring body's translation.

Does not prove: (a) that the model of `0x80237CA0` (sine/cosine as
`sin/cos(angle·2π/256)` in `f64`, narrowed to `f32`) is the image's real 256-entry
table — both sides of the comparison use the same model, so this validates the
transcription of each kernel, not the table; (b) anything about the 11 input
decoders beyond the structural bound above; (c) that the callers pass the
arguments the doc blocks claim; (d) anything the harness models loosely —
integer load/store is treated as frame/LR traffic, and call results, memory
aliasing and the `dst`/`src` overlap case are not modelled.

## Reproduce

```
node work/wave229/laneE_diffcheck.mjs        # 38 kernels, prints per-body table
node work/wave229/laneE_input_census.mjs     # 12 callbacks, census + call coverage
```

Machine-readable results: `work/wave229/laneE_diffcheck.json`
(per-body trials/mismatches/exact/maxrel) and
`work/wave229/laneE_input_census.json` (per-body census).
