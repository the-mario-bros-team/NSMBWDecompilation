# Coverage - Wave 207 lane A (`ui::hud`, main.dol)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/hud.rs` | `ui::hud` | 10 | 1028 | 10 | 0 |

Addresses `0x80035CA0`-`0x80036930`, the scene reply/release/accessor tail of
the `0x80033xxx`-`0x80036xxx` actor family whose earlier members landed in waves
205 (lanes D/F, `0x8002B690`-`0x80030140`) and 206 (lane A,
`0x800301B0`-`0x800310C0`).

## What landed

- **Collision reply.** `0x80035CA0` (180 B) runs virtual `+0xA4` of the scene,
  takes the contact record's class byte from virtual `+0x6C`, packs it as
  `(byte << 24) | 0x20000000` and spawns effect `0x36` at
  `(+0xAC, +0xB0, +0xB4)` through `0x80064610`. A successful spawn inherits the
  contact's `+0x348` owner byte and this scene's `+0x368` handle, then
  `0x80162510(this)` activates the scene.
- **Mode dispatcher.** `0x80035DC0` (188 B) asks `0x800964A0(this, arg2,
  contact, 0)` for the mode: `0` runs the base reply `0x80095C20`, `1` spawns
  the effect `0x800A0EA0` with an all-zero two-float record and the shared
  `0x80357BB4` descriptor, `3` calls `0x800A1190`.
- **Release pair.** `0x80036570` (88 B) and `0x800365D0` (92 B) are the
  `0x8015F800` / `0x802B9280` destructor wrapper already translated as
  `fn_80030e90`; the four extra bytes of `0x800365D0` are a dead duplicate null
  test.
- **Accessors.** `0x80036630` and `0x80036640` (12 B each) return `0x80357BF4`
  and `0x80357BB4`; both constants already exist in this file behind
  `fn_800268b0` / `fn_800268c0`.
- **Chain probe.** `0x80036730` (224 B) is byte-identical to `fn_80030fe0`: five
  `*(node + 0x30)` child tests through slot `+0x0C`, then a self-call entered one
  link below the fifth tested node.
- **Name-tail compare.** `0x80036810` (136 B) is byte-identical to
  `fn_800310c0`: two `0x802E1C40` scans for `':'`, then `0x802E1AB4`.
- **Interface forwards.** `0x800368D0` and `0x80036900` (48 B each) are the
  `0x802DCBFC(b, b, a)` thunks with the descriptors at `a + 0x18` and `a + 0x24`,
  identical to `fn_80035670` / `fn_800356a0`.

## Corrections and recoveries

- **No prologue artefacts in this lane.** All ten `functions.tsv` sizes match
  the prologue/`blr` extent exactly. The `blr`s sit at `0x80035D50`,
  `0x80035E78`, `0x800365C4`, `0x80036628`, `0x80036638`, `0x80036648`,
  `0x8003680C`, `0x80036894`, `0x800368FC` and `0x8003692C`.
- **`0x80035CA0`'s virtual `+0xA4` keeps `r4` live.** Ghidra prints the call as
  `(**(code **)(...))()` with no operands, but `r4` still holds the contact
  record at the `bcctr` (`mr r31,r4` never clobbers it), so the reconstruction
  passes both registers rather than dropping the second.
- **`0x80035DC0`'s mode-1 pair is two floats, not two words.** Ghidra shows
  `local_18 = *(undefined4 *)(unaff_r2 + -0x7b38)` and copies that value into
  both halves; the disassembly is `c00284c8 lfs f0,0xffff84c8(r2)` with
  `r2 = 0x80433080`, i.e. `0x8042B548`, and the ELF holds `00000000` there. The
  existing constant `F32_8042B548_F` is reused.
- **Two extra call-site registers are documented, not dropped.** `0x800A0EA0`
  is exported with four parameters but the call site sets `r7 = 1`, and
  `0x800A1190` is exported with two but the call site sets `r5 = 1` (the callee
  overwrites `r5` before reading it). Both are passed through
  `platform::call5`/`platform::call3` for register fidelity.
- **`0x800365D0` carries a dead duplicate null test.** The two branches at
  `0x800365F0` (`41820020`) and `0x800365F4` (`4182000C`) share `cr0` from one
  `cmpi`, so the inner `beq` is unreachable. Not reproduced; behaviour is
  unchanged.
- **`0x80036730`'s recursion skips a node by design.** After the fifth failed
  test the original advances `r31` once more and calls itself with
  `*(r31 + 0x30)`, so the sixth node is never itself tested - the callee tests
  its child. The loop below keeps that exact step rather than folding it into a
  plain `return fn_80036730(child)`.

## Deferrals

None. All ten bodies are fully reconstructible from shipped code.

## Verification

- Temp-copy `cargo check --lib` exit 0 (23 pre-existing warnings elsewhere; none
  from `src/ui/hud.rs` - the appended block re-checked clean after a forced
  rebuild).
- The lane CSV was validated by hand against `tools/check-lane-csv.mjs`'s rules:
  canonical 7-column header, 10 rows, 7 fields each, unique plain lowercase hex
  addresses, evidence files present, and `# Original` sizes equal to
  `functions.tsv` (180/188/88/92/12/12/224/136/48/48).
- No files outside `work/wave207/laneA_block.rs`,
  `ledger/agents/wave207-laneA.csv` and this document were written;
  `src/ui/hud.rs` is untouched.
