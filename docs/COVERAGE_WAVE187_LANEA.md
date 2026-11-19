# Coverage - Wave 187, lane A (`game::bases`, sub-300-byte sweep)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 10 | 1,540 | 10 | 0 |

All ten lane-A addresses are 136-180 bytes in
`work/ghidra/rel-d_basesNP/export/functions.tsv` (the 128-192 band of the
sub-300 sweep). Lane A added no helpers and did not touch `src/game/bases.rs`;
the block is in `work/wave187/laneA_block.rs`. Every body was checked against
the raw words with `ppc159.js`; where the Ghidra C disagreed with the words the
words won and the difference is noted below.

## Translated

- `0x8060_0D40` (180 bytes) - character-change select arrow loader. Guards on
  `+0xF4`, mounts `charaChangeSelectArrow_02.brlyt` through vtable slot `+0x14`
  of `*(this+0x78)`, binds the four `.brlan` animations
  (`inArrow`/`loopArrow`/`hitArrow`/`outArrow`, table `0x807D_0E80`, count 4),
  the `A00_arrowR`/`A01_arrowL` panes (`0x807D_0EA8`, header `0x807C_6620`,
  count 8), stores the animation in `+0x244` and binds
  `P_arrowR_00`/`P_arrowL_01` into `+0x248`.
- `0x8060_10F0` (156 bytes) - closes the `+0x208` interface once the four
  layout animations `0, 1, 6, 7` are all idle, dispatching the `0x8082_2998`
  descriptor (the same one [`fn_80601380`] uses on its no-transition path).
- `0x8061_3530` (156 bytes) - `dEasyPairing_c`: runs [`fn_806131d0`], and once
  animation `0xFFFFFFFF` has finished starts `0x80613290(this, i)` for each
  player whose state word `+0x27C + i*4` is `0`, then posts
  `StateID_StartWait` (`0x8082_3DB0`).
- `0x8061_48F0` (144 bytes) - `dEventOpeningTitle_c` exit dispatch. Guarded by
  `+0x276`; mode `2` needs counter `0x8042A200 > 0x384` and then tail-calls
  `0x80102260(8,1,2,0)` when demo word `0x8042A1F4` is non-negative or
  `0x801017D0(1,0)` when it is negative; mode `3` needs
  `0x8042A200 > 0x960` and tail-calls `0x801017D0(0,0)`.
- `0x8062_45F0` (136 bytes) - returns the frame time of layout animation `2`
  (`*(*(this+0x230)+0x50)+8`) or `3` (`*(*(this+0x230)+0x78)+8`), defaulting to
  `0.0` (`0x807C_6F58`).
- `0x8062_4880` (172 bytes) - retires both latched model animations
  (`+0x257`/`+0x259`): stop layout animation `2`/`3`, write the sampled frame
  through `0x801637D0`, release the binding through `0x801644D0`, then clear
  `+0x25C` and `+0x256`.
- `0x8062_4A20` (164 bytes) - single-sided variant of the same retirement,
  chosen by `+0x25C == 2`, then the same two clears.
- `0x8062_4AD0` (144 bytes) - out-animation swap: `+0x25C == 4` takes flag `2`
  and animation `6` and clears `+0x257`/`+0x258`; otherwise flag `3` and
  animation `7` and clears `+0x259`/`+0x25A`; sets `+0x256`.
- `0x8062_4BE0` (152 bytes) - independent two-latch variant of the swap, with
  the same flags/animations and `+0x256 = 1`.
- `0x8062_7240` (136 bytes) - model-play-date layout restart: stops `+0xAC`,
  indexes the word table `0x807C_6FD0` with `+0x27C`, starts the animation via
  `0x800C92F0` when the mode is below `6` (else shows the entry via
  `0x800C93D0`), then runs [`fn_80627080`] and `0x80627160`.

## Deferred

None.

## Notes and limitations

- `0x8061_48F0`: the Ghidra export ends with `halt_baddata()` and a bogus
  expression `(uVar1 * 4 | ... )`. The raw words
  `0x80614924..0x80614938` are
  `rlwinm r0,r3,30,0,1` / `rlwinm r3,r3,1,31,31` / `subf r0,r3,r0` /
  `rlwinm r0,r0,2,0,31` / `add. r0,r0,r3`; that sequence is `0` for a
  non-negative word and `0xFFFFFFFD` for a negative one, i.e. a sign test. The
  block keeps the literal rotates and shifts so the reduction is auditable, and
  the two conditions (`<= 0x384` / `<= 0x960` return) come from the `bclr 4,1`
  "branch if not greater" words, not from the C text.
- `0x8062_45F0`: the C prints `double` and inserts GQR0 `ldexpf` guards. The
  stream uses only `lfs`/`psq_st` with GQR0 and ends in `fmr f1,f31`, so the
  body returns `f32`. The default is `lfs` from `0x807C_6F58` (`0.0`).
- `0x8062_4880`/`0x8062_4A20`: the C loses the `this` argument of the nested
  `FUN_806245f0` call and prints the frame as `double`; the raw `bl` forwards
  `r3 = this` and `f1` holds the `f32`. `0x801637D0` takes the binding pointer
  in `r3` and the frame in `f1` (`call_f32_u32`); `0x801644D0` receives the
  address of the binding slot (`model + 0x50` / `model + 0x78`, from the
  `lwzu` base update), not the pointer value.
- `0x8062_7240`: the table at `0x807C_6FD0` reads `0,1,2,3,4,5,0,5,...`; the
  body reads it at runtime with the same `lwzx` scaling
  (`(index as u32) * 4`) instead of baking the entries in. `0x80627160` has no
  Rust body yet, so it is called through `abi_call1!`; `0x80613290` (used by
  `0x8061_3530`) is likewise address-only.
- Class attribution: `0x8061_3530` is the `dEasyPairing_c` cluster around
  [`fn_806131d0`] (descriptor `0x8082_3DB0` is `StateID_StartWait` from
  [`fn_80613ca0`]); `0x8061_48F0` is the `dEventOpeningTitle_c` tail
  (`+0x276` latch, same scene-request pair as [`fn_80614cc0`]); the `0x80624`
  bodies share the `+0xAC` layout / `+0x230` model / `+0x25x` latch cluster of
  [`fn_80624790`]/[`fn_80624930`]; `0x8062_7240` shares the `+0x27A..+0x298`
  layout of [`fn_80626db0`]. The role of `0x800C9380`/`0x800C9490`/`0x800C9500`
  and `0x801637D0`/`0x801644D0` remains `// unknown:`; only their literal
  argument shapes are asserted.
- `0x8082_2998` / `0x8082_3DB0` are state descriptors past the file-backed end
  of the ELF (`0x8082_28B8`), in the zero-fill tail, and are passed through as
  address literals exactly like the existing `0x8082_2958`/`0x8082_3D70`
  arguments.
- The appended block type-checks against a scratch copy of the crate
  (`cargo check`, no new diagnostics); `src/game/bases.rs` itself was not
  modified.
