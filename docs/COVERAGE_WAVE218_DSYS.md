# Coverage - Wave 218 (`engine::dsys`, main.dol part 4, final)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/dsys.rs` | `engine::dsys` | 68 | 13,064 | 57 | 11 |

The last `engine::dsys` bodies, `0x800FB290`-`0x800FFDB0`. The module is now
**397 / 397**.

## What landed

- **Sheet/code lookups (lane A).** `0x800FB290`, `0x800FB350`, `0x800FB380`,
  `0x800FB3C0`, `0x800FB410`, `0x800FB460`, the 440-byte `0x800FB4B0`,
  `0x800FB670`, `0x800FB6C0`, `0x800FB7F0`, `0x800FB900`, `0x800FBDA0`.
- **Bank row setters (lane B).** `0x800FBE60`, `0x800FBED0`, `0x800FBF30`,
  `0x800FBFB0`, `0x800FBFF0`, `0x800FC0A0`, `0x800FC140`, `0x800FC190`,
  `0x800FC1F0`, `0x800FC390`, `0x800FC400`, `0x800FC470`.
- **Shell and slot gates (lane C).** `0x800FC4F0`, `0x800FC810`, `0x800FC830`,
  `0x800FC940`, `0x800FC970`, `0x800FC9C0`, `0x800FCA20`, `0x800FCA50`,
  `0x800FCAF0`, the 244-byte `0x800FCCA0`, `0x800FCDA0`, `0x800FCDB0`.
- **Bank action dispatch (lane D).** `0x800FCDC0`, `0x800FCE60`, `0x800FCF30`,
  `0x800FD0A0`, `0x800FD130`, `0x800FD180`, `0x800FD1F0`, `0x800FD310`,
  `0x800FD3B0`, the split `0x800FD590`, `0x800FD720`, `0x800FD750`.
- **Sheet writers and emit helpers (lane E).** `0x800FE040`, `0x800FE050`,
  `0x800FE1D0`, `0x800FE2E0`, `0x800FE400`, `0x800FE560`, `0x800FE760`,
  the 440-byte `0x800FEC10`, `0x800FF000`, `0x800FF180`, `0x800FF660`,
  `0x800FF680`.
- **Path rail (lane F).** `0x800FF6A0`, `0x800FF6B0`, `0x800FF6C0`,
  `0x800FF730`, the 780-byte `0x800FF740`, the 496-byte `0x800FFA50`,
  `0x800FFC40`, `0x800FFDB0`.

## Corrections and recoveries

- **Untabled bodies found.** `0x800FC2D0` (152 B) and `0x800FC370` (20 B,
  `cntlzw(byte-8)` shell test) lie inside the range but have **no
  `functions.tsv` row and no ledger entry**; both were read in full. Separately
  `0x800FC040` (92 B) is already covered as `game::bases` `fn_80770970`, since
  the REL shares the slot.
- **`0x800FD590` is split.** A 156-byte head ends in `b 0x800FD630`; the
  232-byte tail has its own prologue and returns to the original caller with
  `r3`/`r4` unchanged, so it is part of the same function and is included (as
  Ghidra's export does).
- `0x800FB6C0` fills **different** buffers: the reference goes to `param_1+5`
  once, the candidate's tail in-loop; the accept test is
  `memcmp(param_1,name,10) != 0 && memcmp(reference,tail,5) == 0`.
- `0x800FB350` is `void` in Ghidra but returns the id byte (all three callers
  mask `r3`).
- `0x800FB4B0` compares one genuinely **uninitialised** stack byte (`r1+0x31`,
  never stored); modelled as `0` and documented, hence medium.
- `0x800FC1F0`'s jump table is at `0x8031C190` (`lis 0x8032` + `addi -0x3E70`),
  with the out-of-order `2 -> 0x28A` entry confirmed; the whitelist keys at
  `0x80428BA0`/`0x80428BA8` read `"F101"`/`"W8A0"`.
- `0x800FC140`'s `0x801002C0` is a `r3`-returning key lookup (it tail-branches
  to `0x800F53A0`), not `void`.
- `0x800FE1D0`'s Ghidra `undefined4` seed is an `f32`; `0x800FEC10`'s
  `0x800D65E0` emit takes `(f64,f64,f64,u32,u32)` with `f2`/`f3` swapped
  relative to the export names; `0x800FE760`'s `0x801C0540` is an in-place
  multiply (`r3 == r5`).
- `0x800FF680`'s Ghidra jumptable warning is spurious: it is a plain virtual
  tail call ending in `bcctr`.
- `0x80238130` is a twelve-lane **add**, not a multiply, which explains the nine
  `* 0.0` stores in `0x800FFA50` (only the translation column is kept).
- `0x800FFA50`'s export is incomplete: it drops the `0.1` test (`0x800FFB34`),
  the `reach/dx` division (`0x800FFB3C`) and the nine neutral stores
  (`0x800FFB84`). The body follows the instruction stream.
- `0x800FF740` has one documented evidence conflict at `0x800FF9B8`: the image
  stores the finished loop counter (`neg r0,r3` / `extsh r0,r30` /
  `sth r30,8(r1)`), while the export prints `local_268[0] = -sVar5`. The body
  follows the export and the conflict is called out in `# Confidence`.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-lane-csv.mjs wave218`: 68 rows, 68 unique, 57 high + 11 medium.
- Ledger: 19,918 rows (98.35%); `engine::dsys` 397/397 complete.
- `scripts/check.ps1` exit 0.

## Next

`src/resource/archive.rs` (198), `src/game/stage.rs` (132), the two `ui::hud`
stragglers (`0x80026CA0`, `0x8002D260`) and the two untabled dsys bodies.
