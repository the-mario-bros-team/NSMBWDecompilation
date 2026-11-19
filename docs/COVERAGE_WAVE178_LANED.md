# Coverage - Wave 178 lane D (`game::bases`)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane D block) | `game::bases` | 4 | 1,932 | 3 | 1 |

Fourth batch from the 480-484 byte slice of the 300-512 byte band. Source of the
bodies: `work/wave178/laneD_block.rs`.

## What this pass covers

- **Results-screen count-up (1 body).** `0x80639000` is the sibling of the
  already-translated `0x80638960`, one function later in the four-panel cluster.
  It bails out unless the `+0x70` layout animation query answers "any", counts
  the records whose `+0x7DC` word is zero, and then advances the score word
  `table[index*4 + 0x3EC]` of every idle record that has `+0x7E5` set, pinning it
  at `999`. Each such record plays one of five cue pairs - `0x360`/`0x35A`,
  `0x3A7`/`0x3A1`, `0x3EE`/`1000`, `0x434`/`0x42E` - chosen by the record's
  `+0x7E0` index and by whether more than one record is still idle, then clears
  `+0x8AF` and dispatches the `+0x18` state slot with `0x80826E60`.
- **Score-bucket lookup (1 body).** `0x80630BF0` maps the selection
  `+0x298`/`+0x29C` onto the score tables behind `0x80429F7C`. `+0x298` is
  reduced once by 10, the index becomes `(+0x298 % 10) + +0x29C * 10`, and the
  split-screen flag `0x80429F80 & 0x20` picks the 8-byte entry array at `+0x770`
  (split) or `+0x400` (paired). The selected entry supplies a next-row byte, a
  wanted key byte and a value word; the key is then searched through the ten
  entries of the next row (row stride `0x50`), falling back to entry `+10`.
- **Ship aim update (1 body).** `0x80721800` sits between the translated
  `0x807217B0` and `0x807219F0`. It samples the target at `+0x638` twice through
  the target's `+0x60` table slot `+0x70`, builds the delta `+0x628..+0x630` from
  the ship's own `+0x610`/`+0x614`, normalises it, and either clears `+0x634`
  (outside the view cone) or recomputes the pitch `+0x60E` and yaw `+0x60C`
  through `0x801611C0`, clamped to `[-0x1000, 0x2000]` and to the per-face window
  from the `0x8042C1A0` byte pair.
- **World-pane placement (1 body).** `0x807BBA40` is the sibling of the
  translated `0x807BB710`/`0x807BBC30`. It feeds `(pose+0x90, pose+0xA0, 0.0)`
  to `0x800B32E0` and stores the resulting position plus the constant
  `(0.0, -73.0, 0.0)` triad and the `16.0`/`0.0` `+0x2C0` value on each of the
  `+0x294 + 1` panes.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80639000` | `fn_80639000` | 480 | high |
| `0x80630BF0` | `fn_80630bf0` | 484 | high |
| `0x80721800` | `fn_80721800` | 484 | medium |
| `0x807BBA40` | `fn_807bba40` | 484 | high |

## Deferred

None.

## Corrections and notes

- `0x80639000` and `0x807BBA40` are typed `void(void)` by Ghidra because both
  start with a `0x802DCD7x` register-save thunk that hides the incoming `r3`.
  The raw words read `r3` immediately (`lwz r0,0x7dc(r26)`,
  `lwz r4,0x268(r3)`), so both bodies take `this`.
- `0x80639000`'s `999` clamp is `cmpwi 999` + `blt`, i.e. saturate at `999`
  (Ghidra's `0x3e6 < value`), not a modulus.
- `0x80721800`'s two target calls are emitted as raw `abi_call2!(slot, buf,
  target)` rather than `vmethod*`: the vtable pointer lives at `target + 0x60`,
  `r3` carries the sample buffer and `r4` carries the target, which is the
  reverse of the `vmethod1` argument layout. The two calls write to `r1+0x18`
  and then `r1+0x10`; the second write overlaps the first, and the body reads
  `+0x14` and `+0x18` afterwards, so a single six-float buffer preserves the
  aliasing. This ambiguity is the reason for the `medium` confidence.
- `0x80721800`'s yaw window is not a nested range: the four staged halfwords are
  `{(i8)byte[0x8042C1A1], 0x3000, 0xD000, (i8)byte[0x8042C1A0]}` and face `0`
  clamps to `[-0x3000, table[+1]]` while face `1` clamps to
  `[table[+0], 0x3000]`. `0x8042C1A0` belongs to main.dol, so the literal byte
  values are not available in the REL and are read at runtime (same convention
  as the existing `fn_807216e0`).
- `0x80721800` uses `0x801611C0` as a 16-bit angle producer returning in `r3`
  (`call_f32x2(...) as u16 as i16`), matching the other `0x801611C0` call sites
  in this file, not as an `f32` return.
- `0x80630BF0` reduces `+0x298` with a single conditional `-10`
  (`addi r7,r7,-10`), which is only a modulus for `+0x298 < 20`; the body keeps
  that literal form. The lookup key comparison is `cmpl`, but only equality is
  tested, so sign extension is irrelevant.
- `0x807BBA40` copies twelve pose words (`+0x84..+0xB0`) but only `+0x90` and
  `+0xA0` are read; the third input component is the `0.0` constant, matching
  the `0x800B32E0` staging already used by `fn_80642df0`.
- `0x807BBA40` reads its per-pane enable byte from
  `*(this + 0x268) + 0x275`, a different object from the pane array at
  `+0x248`; `16.0` is stored when that byte is zero.

## Verification

- The lane-D block was appended to a scratch copy of `src/game/bases.rs` and
  `cargo check --lib` passed with no errors and no new warnings.
- All four evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`; every constant, branch and field offset
  was read from the raw words of `work/rels/elf/d_basesNP.elf`.
