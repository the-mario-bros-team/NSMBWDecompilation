# Wave 193, lane D coverage (`game::bases`)

Lane band `0x806F7830..0x80706110`, all inside REL `d_basesNP`. Ten of ten
requested bodies translated; none deferred.

## Bodies

| Address | Bytes | Function | Summary |
| --- | --- | --- | --- |
| `0x806F7830` | 220 | `fn_806f7830` | Rebuilds the `+0x7C` matrix from `(+0xAC,+0xB0,+0xB4)` and the `+0x100..+0x104` angles, then rebinds the model holder at `+0x5B0` with `+0x7C`/`+0xDC`. |
| `0x806F8160` | 252 | `fn_806f8160` | Resolves the watched actor through `0x8016_2D00(+0x568)` and mirrors the three angles plus the position, either directly or with the target's own `+0x714..+0x71C` pre-move delta removed. |
| `0x806F8870` | 224 | `fn_806f8870` | Five unrolled `+0x0C` vtable probes of the `+0x30` child chain; returns the last passing node, with a recursive tail past the fifth probe. |
| `0x806FE310` | 212 | `fn_806fe310` | Scans player slots `0..4`, filters by tag `0x23`, matches the character byte against `+0x208` under the `+0x220`/`+0x6A1` gates, then forwards the character through `fn_806fe210`. |
| `0x806FFC50` | 212 | `fn_806ffc50` | Destructor: three 14×`0x128` array teardowns (`0x8065_7200`), a 14×`0x40` array (`0x8016_9D20`), the `+0x8C4`/`+0x524` accessors, the base teardown `0x8009_5130` and optional `0x8016_2920` free. |
| `0x80703940` | 248 | `fn_80703940` | Builds the four-bit player mask from nearby state-1/2 actors over the six `0x5C`-byte records at `+0xB84..+0xDAC`, maps it through `0x8004_40C0` and writes the sign-extended byte to `out`. |
| `0x80703A40` | 252 | `fn_80703a40` | Per-frame boat-light mask/lighting update: `count != 1` path rebuilds the mask via `fn_80703940`; the `count == 1` path copies the player index. Then applies pad `[0x80429F50][mode + 1]` through `0x800B_5C50`/`0x8015_F4F0`, or clears with `0x8015_F200`. |
| `0x80704790` | 208 | `fn_80704790` | One boat-light frame: `fn_80703dc0`, snap `+0xAC..+0xB4` from `+0xDF0..+0xDF8`, write `+0xDFC`, and when `0x800D_9560(+0xDBC)` is non-zero match `+0xDDA` against the eight-byte `0x807C_C600` table and cue sound `0x2AA`. |
| `0x807057C0` | 204 | `fn_807057c0` | Applies a three-float position to the type-filtered (`0x802F_8FF0`) object at `+0x3B8` through the `0x8025_C660`/`0x8025_C470` handle, `0x801C_0C60` matrix and `0x8023_CFA0`. |
| `0x80706110` | 220 | `fn_80706110` | Same transform rebuild as `0x806F7830` but rebinds the holder at `+0x3F0` (identical to the already translated `fn_806b1970` minus its snapshot call). |

## Signature of `0x806FE310`

Several earlier bodies referenced `0x806FE310` as an address-only callee. Its
exact prototype is:

```rust
pub unsafe fn fn_806fe310(this: usize)
```

It takes only `r3` (the panel manager), returns nothing, and touches `+0x208`,
`+0x220` and `+0x6A1`. It ends by calling [`fn_806fe210`], which is already
translated, so no new trampoline is needed at the call sites.

## Ghidra corrections made against the raw words

- `0x806F8160`: the raw `beq` at `0x806F_818C` jumps to the **subtract** arm, so
  the direct-copy arm runs when `+0x574 != 0`. The C has the two arms swapped.
- `0x80703940`: the `_savegpr_24` helper (`0x802DCD68`) is not a call with a
  return value; `mr r25,r4` and the final `stw r0,0x0(r25)` prove `r4` is the
  `out` pointer the C prints as `*(int *)uVar9`.
- `0x80703A40`: `cntlzw(count-1)` with the following `rlwinm.` folds to
  `count == 1` for the else path; the C's `uVar1 >> 5 == 0` is the same test.
- `0x806F8870`: the recursive tail calls `fn_806f8870(*(r31+0x30))`, where
  `r31` is the fourth child, not the argument.
- `0x806FFC50`: `0x8016_5E70`/`0x8006_9060` are the same accessor teardowns
  already used by the wave-50 boat-light destructors.

## Reused already-translated neighbours

`fn_806b1970` (transform-rebuild template), `fn_806fe210`, `fn_80703840`,
`fn_80702a30`, `fn_80703dc0`, `fn_806caad0` (type-filtered position apply), plus
the helpers `vmethod0`/`vmethod1`/`actor_method0`, `call0_ret` and
`call_f32x3_u32`.

## Evidence and checks

- Every body was re-read from the raw words with
  `node ppc159.js work/rels/elf/d_basesNP.elf <addr>`; the Ghidra exports were
  used only as an index.
- Constants read from the ELF: the `0x807C_C600` byte table is
  `[5,7,0x0D,0x16,0x1A,0x20,0x25,0x2A]`; the three `0x128`-byte arrays step by
  `0x1030 = 0x128 * 0xE`.
- All bodies parse cleanly against stub declarations of the reused helpers
  (`rustc --edition 2021 --crate-type lib`).

## Deferred

None.

## Files

- `work/wave193/laneD_block.rs`
- `ledger/agents/wave193-laned.csv`
- `docs/COVERAGE_WAVE193_LANED.md`
