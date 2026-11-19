# Wave 199 lane B coverage (`game::bases`)

Slice: the course-scene setup helper and the module's largest body. Two
translated, none deferred.

| address | size | body |
| --- | --- | --- |
| `0x807B_8CB0` | 516 | `fn_807b8cb0` - mounts `scene/scene.brres` for the current course, resolves `"MainScene"`, and installs the `.bfog`/`.blmap`/`.blight` entry selected by `0x8008E6B0` from the 23-entry tables at `0x807D_08E0`/`0x807D_0884`/`0x807D_0828` through `0x807AC270`/`AC260`/`AC280`; also latches `0x8008E6E0` into the byte `0x8043_A225` |
| `0x8077_B450` | 4076 | `fn_8077b450` - scene-actor frame: a 0x99-entry jump-table switch (states `10..=0xA2`) of one-shot bodies, then the shared `vtable+0x64` abort test and a binary-search decision tree over the same state that either runs the per-frame follow-up, closes the state through `vtable+0x68`, or latches the `+0x139` flag |

## `fn_807b8cb0` notes

The three 23-word tables are copied to the stack by three identical
`lwzu`/`stwu` loops (11 pairs plus one trailing word) and then indexed by
`0x8008E6B0(scene, variant) & 0xFFFF`. The Rust body reads the table entry from
its REL address instead of reproducing the stack copy, which is identical for
every index inside the table; an index `>= 23` would have read adjacent stack
slots in the original. `0x802C8840` takes `r3`/`f1`/`r4..r6`, so the call is a
local `extern "C" fn(u32, f32, u32, u32, u32)` transmute rather than one of the
shared `call_f32_*` wrappers (which all put the float first *and* before the
integers). The Ghidra signature for this function is unusable: `r3` is dead on
entry, the only argument is `r4`.

## `fn_8077b450` notes

* The dispatch is a real jump table, not a `switch` chain: `0x8081_81F8` holds
  `0x99` words and was read out entry by entry. Every unlisted state maps to
  `0x8077BD20`, so the Rust `match` has a single `_ => {}` arm.
* Ghidra's `param_1/param_2/param_3` are `r3 = this`, `r4 = state`,
  `r5 = run_state_machine`; its `iVar10` is the state and its `iVar3` is `this`.
  The `uVar12 = func_0x802dcd68()` struct return is the `_savegpr` helper and is
  not emitted, nor is `_restgpr_*` at `0x802DCDB4`.
* The global references Ghidra prints as `_DAT_8042xxxx` are `0x8043xxxx` in the
  raw `lis`/`lwz` pairs (`0x8043_A1AC`, `0x8043_A1A0`, `0x8043_A18C`,
  `0x8043_A26C`, `0x8043_A27C`, `0x8043_A24D`, `0x8043_A1C8`, `0x8043_9E98`).
  The bodies use the raw addresses.
* `0x807CF608` is the constant base: `+0x90` = `0.0`, `+0x94` = `1.0`,
  `+0x98` = `50.0`, `+0x9C` = `5.0`, `+0xA0` = `10000.0`, `+0xA4` = `0.001`,
  `+0xA8` = `0.01`. All read from the ELF, not from Ghidra's names.
* Branch polarity was re-derived from the `BO`/`BI` fields, because the local
  printer renders both `bc 12,n` (branch when the CR bit is set) and `bc 4,n`
  (branch when it is clear) as bare `bc`; the second decision tree at
  `0x8077BD3C` only reconstructs correctly with that reading.
* `0x800F69F0` takes three floats plus *five* integers (`this`, the position,
  two `lha` angles, the `0x800FC2D0` output); Ghidra prints it as eight
  parameters, so the call sites use `call_f32x3_u32x5`.
* `0x800FC2D0` is called with a second argument equal to `this + 0x200`
  (`rb`+0x240 of `0x8043_A1A0`); Ghidra drops it at two of the three sites
  because it inferred a one-argument prototype elsewhere.
* Three stack stores that Ghidra names `local_134`/`local_130`/`local_12c` in
  the `0x3D` tail are dead (nothing reads `r1+0x2C..0x34` again) and are
  omitted; the six live `+0xDC`/`+0xE0`/`+0xE4` writes are kept.

## Verification

Checked in a throwaway copy of the crate, never in the repo:

```powershell
$d = "$env:TEMP\opencode\scratch199"
Copy-Item -Recurse src $d\src
Get-Content work\wave199\laneB_block.rs -Raw | Add-Content $d\src\game\bases.rs
cd $d; cargo check --lib        # exit 0, no diagnostics from the block's lines
node tools/doc-lint.mjs "$d\src"  # {"ok":true,"files":68,"violations":0}
```

## Deferred

None. All 1,019 instructions of `0x8077B450` (`0x8077B450..0x8077C438`) are
covered: 24 distinct switch bodies, the 26 state arms of the post-switch tree,
and the `0x8077C41C` flag latch. The `0x80164B70`/`0x80164B80`/
`0x807AC26x` contracts and the `0x8081FEC8..80820580` name strings are outside
this REL's exports, which is why `fn_807b8cb0` is rated `medium`.
