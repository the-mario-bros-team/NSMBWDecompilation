# Coverage - Wave 207 lane B (`ui::hud`, main.dol)

> File name note: the shared instructions name this document
> `docs/COVERAGE_WAVE205_LANE<X>.md`, but that path is already owned by the
> wave-205 lane B report. Wave 207 lane C already landed as
> `docs/COVERAGE_WAVE207_LANEC.md`, so this lane follows the same
> `COVERAGE_WAVE207_LANE<X>.md` convention instead of overwriting unrelated work.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/hud.rs` | `ui::hud` | 10 | 2244 | 9 | 1 |

Addresses `0x8003_6930`-`0x8003_7190`, the ten bodies immediately before the
`0x80037360`/`0x80037400`/`0x800374A0` wrappers and the state entry points
`0x80037690`/`0x80037700` that call them.

## What landed

- **Pool counter.** `0x80036930` walks pool `0x250` and then pool `0x251` with
  `0x80162D50`, restarting the cursor at `0` for each walk, and increments
  `+0x3E4` on the record whose `+0x3D0` equals the actor's `+0x724`.
- **Position publish.** `0x800369D0` forwards `+0x6E8`/`+0x6EC` either to
  `0x800D0470` (mode `+0x710 == 1`, as a `vec3` with the player byte from
  `*(SDA_BASE - 0x54B8) + 0x120E`), or as two `fctiwz`-truncated 16-bit cell
  coordinates to `0x800777B0`; the second path then calls `0x80036930` when
  `+0x718 == 8`.
- **Hold/re-arm split.** `0x80036A70` tests bit 0 of `+0x60C`: clear re-links
  `+0x534` through `0x8007F900`, set activates the scene with `0x80162510`.
  The boolean result is the inverted bit.
- **Carried action.** `0x80036AB0` gates on the two-level `+0x394` query for
  `0x80353970`, then discriminates on `fn_800212c0(this)`: mode `3` commits, and
  modes `1`/`2`/`4..6` commit when the `0x8005F900(+0x694)` record's signed
  `+0x1090` byte is `1`, `2`, `4`, `5` or `6`. Committing builds the 16-byte
  `{f32 x, f32 y, u8 mode, 1, 1, u8 palette, u16 0}` record with
  `x = (+0xAC - 8.0) + +0x624`, `y = (8.0 + +0xB0) + +0x628`, dispatches it to
  `0x80087F40`, republishes with `0x800369D0`, unlinks `+0x534` and calls
  `0x80064D10(this, 1)`. The other arms only write `+0x68A`/`+0x689` and issue
  one virtual `+0xD4` request for `0x80352AF4` or `0x80352B34`.
- **Two registrations.** `0x80036C90` and `0x80036D40` both call
  `0x8007FB10(this + 0x534, this, …, flag, arg, extra)` and reset the
  `+0x600`/`+0x604` cursors. `0x80036C90` passes a descriptor selected by
  `+0x710` (`0x803539BC`/`0x803539A0`), copies two bytes into `+0x611`/`+0x612`
  and writes `5`/`0`. `0x80036D40` fills the `+0x6C0` quad (`{-8, 16, 8, 0}` or
  `{16, 8, 8, -7}`), the `+0x6DC` triple `{1, 1, 1}`, installs the
  `0x80020BF0`/`0x80020E70`/`0x800210F0` callback triple and writes `0`/`0xE0`.
- **Latch driver.** `0x80036E20` probes `0x800D87B0(*(SDA_BASE - 0x5680), 0)`
  and drives the `+0x714` latch: probe/no-latch arms the `0x80353970` query and
  registers the string form, probe/no-latch registers the other query, and the
  two mirrored pairs run in reverse on the clear-probe/`+0x714` side.
- **Three pool resolvers.** `0x80037060` matches `(record[+0x04] >> 4) & 0xF`
  against `+0x71C`, copies the record's `+0x3EC..+0x3F4` triple as
  `this - record` into `+0x700..+0x708`; `0x80037110` matches
  `record[+0x04] >> 24` and copies `+0x524`, byte `+0x5A1` into `+0x70C`,
  `+0x720`; `0x80037190` matches the full word `+0x4AC` and copies
  `+0x470..+0x478` into `+0x700..+0x708`. All three store the record's first
  word into `+0x6BC` and return `1`/`0`.

## Corrections and recoveries

- **All ten `functions.tsv` sizes are exact.** Each body was walked to its
  terminal `bclr`; the code extents are `0xA0`, `0xA0`, `0x40`, `0x1E0`, `0xA8`,
  `0xD8`, `0x234`, `0xAC`, `0x80`, `0x84`. No prologue artefact appears in this
  run. Two bodies are followed by four bytes of zero padding (`0x80036D38` and
  `0x8003710C`); the padding is not code and is not reproduced.
- **`0x800369D0`, `0x80036AB0`, `0x80036D40` and `0x80036E20` export
  `unaff_r13`/`unaff_r2`.** Those are the `r13`/`r2` small-data bases, not real
  parameters: `r13 - 0x56B8`, `r13 - 0x54B8`, `r13 - 0x58B0`,
  `r13 - 0x5868`, `r13 - 0x5680` and the `r2 - 0x7AF8`/`0x7AF4`/`0x7AF0`/
  `0x7AEC`/`0x7AE8`/`0x7AE4` float slots were resolved from the instruction
  immediates and the ELF image (`8.0`, `1.0`, `-8.0`, `16.0`, `0.0`, `-7.0`).
- **`0x80036AB0`'s and `0x80036E20`'s "armed" test is a plain non-zero test.**
  The three instructions `neg r0,r3` / `or r0,r0,r3` / `srwi r0,r0,31` implement
  `(int)(-v | v) < 0`, which is `v != 0` for every input; the Rust body writes
  the comparison directly.
- **The `+0x1090` window is `(v - 4) & 0xFF <= 2`, not `< 3` of a signed byte.**
  `addi r0,r3,-4` / `rlwinm r0,r0,0,24,31` / `cmpli r0,2` / `bc 12,1` branches
  on `GT`, so the flag is set for `r0 <= 2`; the Rust body keeps the byte mask
  so a negative byte cannot slip through.
- **`0x80036C90`'s first parameter is byte-valued.** The export types `param_2`
  as `undefined4` because the caller loads `0x690` with `lwz`, but the store is
  `stb r30,0x611`; the Rust signature keeps the `u32` load and truncates on
  store, exactly as the instruction pair does.
- **`0x80036AB0`'s 16-byte record is contiguous on the stack.** `local_18`,
  `local_14`, `local_10`, `local_f`, `local_e`, `local_d` and `local_c` pack as
  `f32`, `f32`, four bytes, `u16`; the reconstruction uses one `[u8; 16]` buffer
  so the field offsets are explicit.
- **Class identification is inferred.** The state entry table at
  `0x8030_56D0..0x8030_57D4` holds this run's callers (`0x800373F0`,
  `0x80037400`, … `0x80037690`, `0x80037700`) and is immediately followed by
  `daEnObjCoinBlock_c::StateID_SerchRailID` / `…SerchObjPolID` /
  `…SerchWaterMoveID` strings, so the actor is most likely
  `daEnObjCoinBlock_c`. No body proves the class, and the five `0x8035_3xxx`
  descriptors are BSS, so their types stay unknown.
- **`0x80036A70`'s callee role is ambiguous.** `0x8007F900`'s own body inserts
  `this + 0x534` into the list headed at `SDA_BASE - 0x58A0`/`0x58A4` unless
  `+0xDC` is set (its sibling `0x8007F950` removes), yet the branch it sits on
  reads like a retire/re-arm pair. The translation follows the literal callee;
  the "held" wording in the doc block is marked inferred and the ledger row is
  `medium`.

## Verification

- Scratch build: `src/`, `Cargo.toml`, `Cargo.lock` copied to a temporary
  directory, `work/wave207/laneB_block.rs` appended to
  `src/ui/hud.rs` there, `cargo check --lib` exits `0` (no warning originates in
  the appended block).
- Ledger: `ledger/agents/wave207-laneB.csv` (10 rows, `translated`).
- This document: `docs/COVERAGE_WAVE207_LANEB.md`.
