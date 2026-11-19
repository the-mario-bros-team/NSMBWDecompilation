# Coverage - Wave 215, lane A (`engine::dsys`, the `0x800F0010..0x800F0B54` effect-record family)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/engine/dsys.rs` | `engine::dsys` | 10 | 1860 | 6 | 4 | 0 |

Status split: 10 `translated`, 0 `partial`, 0 deferred.

Body block: `work/wave215/laneA_block.rs`. Ledger rows:
`ledger/agents/wave215-laneA.csv`.

## How to read this file

* **Address** - virtual entry point in the DOL image, not a file offset.
* **Name** - the Rust name, `fn_<address>` lowercase; the Ghidra name is in
  each body's `# Original` block.
* **Size** - the `functions.tsv` value. All ten were re-derived from the
  prologue frame and the closing `blr`; every one matches exactly. No size in
  this lane is a 16/20-byte prologue artefact and none of the ten bodies is a
  forwarder stub. Each `# Original` block records the real extent.
* **Confidence** - `high` means every address, field offset, immediate and
  branch is literal in the disassembly. `medium` marks a body whose control flow
  and calls are literal but where an argument source or a field's role had to be
  inferred (see below).
* `r2 = 0x80433080` and `r13 = 0x8042F680`, so `r2 - 0x619C` = `0x8042CEE4`,
  `r2 - 0x6194` = `0x8042CEEC`, `r2 - 0x6190` = `0x8042CEF0`,
  `r13 - 0x6DD8` = `0x804238A8` and `r13 - 0x6DE0` = `0x804238A0`.

## Extents

| address | tsv | frame | `blr` | real span |
|---|---:|---:|---|---|
| `0x800F0010` | 128 | `0x20` | `0x800F008C` | `0x800F0010..0x800F0090` |
| `0x800F0090` | 216 | `0x30` | `0x800F0164` | `0x800F0090..0x800F0168` |
| `0x800F0170` | 52 | `0x10` | `0x800F01A0` | `0x800F0170..0x800F01A4` |
| `0x800F0510` | 112 | `0x20` | `0x800F057C` | `0x800F0510..0x800F0580` |
| `0x800F0580` | 164 | `0x40` | `0x800F0620` | `0x800F0580..0x800F0624` |
| `0x800F0630` | 252 | `0x40` | `0x800F0728` | `0x800F0630..0x800F072C` |
| `0x800F0730` | 200 | `0x40` | `0x800F07F4` | `0x800F0730..0x800F07F8` |
| `0x800F0860` | 376 | `0x20` | `0x800F09D4` | `0x800F0860..0x800F09D8` |
| `0x800F09E0` | 228 | `0x40` | `0x800F0AC0` | `0x800F09E0..0x800F0AC4` |
| `0x800F0AD0` | 132 | `0x20` | `0x800F0B50` | `0x800F0AD0..0x800F0B54` |

Total: 1860 bytes.

## The two objects the lane works on

* A record manager whose layout is the one already documented by the
  `0x800EEAE0` name scan in this module: `+0xB64` pointer to the name array,
  `+0xB68` entry count, `+0xB6C` selected index, `+0x7D4` mode word. The name
  entries are the same values `0x802E1BD0` compares with a fixed byte count.
* A per-object slot array: `0x48`-stride tracks at `+0xC8` (with the track
  itself at slot `+0xD8`, a flag at `+0xDC`, a float at `+0x100` and a value at
  `+0x30`), and a second `0x14`-stride array at `+0x548` whose owner track sits
  at `+0x70`.

Five of the ten bodies (`0x800F0510`, `0x800F0630`, `0x800F09E0`,
`0x800F0AD0`, and the scan half of `0x800F0860`) are variations of the same
"probe sixteen slots with `0x802E1AB4(record, key)` and act on the first match"
shape; only the stride (`0x48` vs `0x14`), the base offset (`0xC8` vs `0x548`)
and the action differ.

## Notable findings

### The `bl 0x802DCDxx` prologues are register-save stubs, not value getters

Three of this lane's exports (`0x800F0090`, `0x800F0860`, and the whole
`0x800F0xxx` cluster) open with `addi r11,r1,<frame>` followed by
`bl 0x802DCD74` / `bl 0x802DCD70`. Ghidra models those as an
`undefined8`-returning call and then feeds the two halves back into `r3`/`r4`.

Dumping the helpers settles it: `0x802DCD70` is the tail of the same block and
the bodies are

```
0x802DCD74: stw r27,-0x14(r11)
0x802DCD78: stw r28,-0x10(r11)
...
0x802DCD84: stw r31,-0x4(r11)
0x802DCD88: blr
```

i.e. the Metrowerks out-of-line `_savegpr_26/27` stubs, paired at function exit
with `_restgpr` (`0x802DCDBC`, `0x802DCDC0`). They only write `r26..r31`
through `r11` and return; `r3`/`r4` are untouched. The "returned pair" Ghidra
shows is therefore the callee's own incoming arguments, which is why
`0x800F0090` and `0x800F0860` are translated with explicit `param_3`/`param_4`
and `param_1`/`param_2` instead of a `platform::call_u64(FN_802DCD74)`. (The
older bodies in the module that do read the pair from the stub follow the
export's reading; they are outside this lane and were left untouched.)

`0x800F0860` has no xref in the export and is declared `void(void)` for that
reason alone; its body still consumes `r3` (the manager) and `r4` (the key), so
the two arguments are recovered and flagged `medium`.

### `0x800F0730`'s `cror` is the `<=` branch, not a mis-printed `>=`

`0x800F0730` contains the only subtle branch idiom of the lane:

```
0x800F0788: fcmpo cr0,f0,f30    ; |value| vs tolerance
0x800F078C: cror  2,0,2         ; cr0[2] = LT|EQ
0x800F0790: mfcr  r0
0x800F0794: rlwinm. r0,r0,3,31,31
0x800F0798: bc 4,2,0x800F07B4
```

`cror 2,0,2` folds `LT|EQ` into the condition bit, and the
`rlwinm. ...3,31,31` + `bc 4,2` pair branches when *that* bit is set (the
record-form `rlwinm` re-records `EQ` as the inverse). The net effect is the
`|value| <= tolerance` test printed in the export, which is what the body
above implements. The neighbouring `0x800F0860` and `0x800F09E0` use the
`cror 2,1,2` (`GE`) variant and are read the same way.

### Float arguments are widened for the Rust call shape

Every float-taking callee in this lane (`0x800EFF30`, `0x801680E0`,
`0x80168FE0`, `0x80166310`, `0x801662F0`) receives its value in `f1` from an
`lfs`/`fadds`/`fsubs` single-precision computation. The module models those
callees as `f64` parameters, so each call site casts at the boundary and keeps
the arithmetic itself in `f32` (`0x800F0010`'s add and clamp,
`0x800F0630`/`0x800F0730`'s `fsubs` comparisons, `0x800F0580`'s forwarded
value).
