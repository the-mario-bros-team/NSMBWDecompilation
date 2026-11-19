# Coverage - Wave 212 lane A (`game::player`, main.dol)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 10 | 2316 | 8 | 2 |

Addresses `0x80060010`-`0x80066E9C`: the player-availability counter, the
list-group rebuild, the nearest-player query, the two type-`0x191` spawn runs,
the 32-entry effect-slot array and its circle test, the scene-rectangle test,
the actor reward sweep, and the character-pick state machine. They sit between
the `0x8005Fxxx` slot accessors and the `0x80066EA0` pick accessor, and touch
none of the sequence-player table at `0x803FC990`.

SDA/data owned or reused by this lane:

| address | bias | role | status |
|---|---|---|---|
| `0x80429CA8` | `r13 - 0x59D8` | one presence bit per player slot | reused (`SDA_SLOT_MASK_80429CA8`) |
| `0x80429D10` | `r13 - 0x5970` | spawn tag byte | reused (`SPAWN_TAG_BYTE_80429D10`) |
| `0x80429D40` | `r13 - 0x5940` | character-pick record | reused (`W92_ANGLE_SOURCE_80429D40`) |
| `0x80429D60` | `r13 - 0x5920` | effect owner object | reused (`W97_EFFECT_OWNER_80429D60`) |
| `0x80429E00` | `r13 - 0x5880` | scene rectangle object | reused (`W22_SCENE_80429E00`) |
| `0x80429F7C` | `r13 - 0x5704` | per-stage suppression table, stride `0x16` | reused (`W98_NOTICE_80429F7C`) |
| `0x80427910` / `0x80427918` | `r13 - 0x7D70` / `-0x7D68` | the two rebuilt list groups | new (`SDA_80427910_A`, `SDA_80427918_A`) |
| `0x80308FF0` | image | class descriptor for a vtable `+8` type test | new (`DAT_80308FF0_A`) |
| `0x802EF340` / `0x802EF368` | `r2 + 0x72C0` / `+0x72E8` | spawn type ids / score amounts | new |
| `0x8042BB38` | `r2 - 0x7548` | `1.0e9`, the nearest-player seed | new (`F32_ONE_E9_8042BB38_A`) |
| `0x8042BBB8` | `r2 - 0x74C8` | `0.0f`: rectangle floor, cue z, cleared slot | new (`F32_ZERO_8042BBB8_A`) |

## What landed

- **Availability count.** `0x80060010` (116 B) is the loop over the four slots
  that counts the players for which both `0x8005F900(index)` and the class slot
  `+0xE0` are non-zero. Its sibling `0x80060090` (the next entry, other lane)
  returns the first such player instead of the count.
- **List rebuild.** `0x80064C10` (248 B) takes the `+0xE8` word of the object
  behind `object + 4`'s vtable slot `+8`, rebuilds `0x80427910` and `0x80427918`
  through `0x80164C50`, then walks `0x8023B8C0(&handle)` entries. Each entry
  type-tests `*(object + 4)` through its own vtable `+8` against the descriptor
  `0x80308FF0` (the word is passed as `0` when the test fails), fills a 13-word
  view with `0x8025C660`, and posts `6` through `0x8023D120` when the view's
  `+0x1C` word (`0x8025C510`) is non-zero.
- **Nearest player.** `0x80065520` (244 B) queries `object`'s world position
  with `0x8006CED0`, seeds `best = 1.0e9`, and keeps the strictly closest
  present, unsuppressed player measured on the `+0xAC/+0xD0` and `+0xB0/+0xD4`
  sums. The suppression byte is `[0x80429F7C] + offset + index*0x16 + 0xAFE`,
  the same byte `0x80064F50` uses to invalidate a spawn slot.
- **Spawn runs.** `0x80066740` and `0x800667B0` (108 B each) are the same loop
  over `fn_80064610(0x191, selector | base, owner, 0, tag)` with `base` `0x0C`
  and `0x06` and a `0x10` selector step.
- **Effect-slot array.** `0x80066820` (56 B) clears `+0x10` and the 32
  sixteen-byte records at `+0x48..+0x248` (`x`, `y`, `radius` as `0.0f`, then an
  active word `0`). `0x800668F0` (188 B) is the reader: it returns `1` for the
  first record whose `+0x54` word is non-zero and whose `+0x50` radius exceeds
  the `abs`-based distance from the point, `0` after all 32.
- **Scene rectangle.** `0x800669B0` (136 B) tests an object position against the
  scene's `+0x38`/`+0x3C`/`+0x40`/`+0x44` rectangle with a `0.0f` lower bound.
- **Reward sweep.** `0x80066A40` (540 B) walks kind-`2` actors, retires the
  `+0x120 & 0x30` ones, applies the `+0x38C`/`+0x390`/`+0x432` eligibility tree,
  the `0x20` flag veto and the rectangle test, then queues
  `0x800B3540(position, type, -1, 0)` plus `0x80060690(amount, slot)` per
  accepted actor, flushing through `0x800B35B0` once eight are queued.
- **Character pick.** `0x80066C60` (572 B) drops a current pick that is missing
  or carries event flag `4`/`0x53`, draws a replacement from the present slots
  without flag `0x53` and with a clear class slot `+0xE0`, arms the `+0x34`
  delay at `3`, and at delay `1` posts cue ids `0x2A7` then `0x2A6` against the
  scene's `+0x48`/`+0x4C` position.

## Corrections and recoveries

- **`0x80064C10`'s descriptor literal.** The export prints `&DAT_802f8ff0`; the
  instruction pair at `0x80064C54`/`0x80064C58` is `lis r31,0x8030` +
  `addi r31,r31,-28688`, i.e. `0x80308FF0`. The block uses the real address.
- **`0x80066A40`'s `mode` is the second register.** The export's
  `(int)uVar10 == 2` reads the low half of the save stub's 64-bit "return",
  which is `r4`: `cmpi cr0,27,r27,2` compares `r27` (`mr r27,r4` at
  `0x80066A60`). The dead `r3` is only forwarded to `0x800669B0`
  (`mr r3,r26` at `0x80066B44`), which overwrites it with the stack position
  before calling `0x8006CED0`, so the first argument only exists for fidelity.
- **`0x80066A40`/`0x80066C60`'s `cntlzw` predicate.** Ghidra renders the
  `0x80066B94..0x80066B9C` words as `countLeadingZeros(x) >> 5 == 0`, which
  names the opposite branch from the one its own bodies imply. `rlwinm
  r0,r0,27,5,31` is `(clz << 27) & 0x7FFFFFFF`, zero only for `clz == 0` or
  `clz == 32`, i.e. `count == 1` (or a negative `count`, which
  `0x8005FEF0` cannot return). The block keeps the bit-accurate form
  `leading_zeros() & 31 == 0` and documents it, so the `0x80060690(amount, -1)`
  and `0x80060690(amount, 0)` arms stay on the right sides.
- **`0x80066C60`'s `xori`/`srawi`/`and`/`subf` run is `count >= 2`.** The
  `0x80066C7C..0x80066C8C` words compute `((c^1)>>1) - ((c^1)&c)` and test its
  sign; for the `u8` returned by `0x8005FFB0` (`0..=4`) that is exactly
  `c >= 2`. The block keeps the literal arithmetic rather than an inequality,
  because the decompiler's own rendering is opaque.
- **`0x80066820` clears with `0.0f`, not `-8000.0f`.** The `lfs` at
  `0x8006682C` is `0x8042BBB8` (`r2 - 0x74C8`), while the neighbouring
  `0x80066490` reset seeds `+0x44` from `0x8042BB94`. The two constants are
  adjacent and easy to confuse; only the first is inside the cleared array.
- **`0x80065520`'s GQR prologue is a `f31` save.** The `ldexpf`/`CONCAT`
  prologue in the export is Ghidra's model of `psq_st f31`/`stfd f31`; the body
  saves and restores one single-precision accumulator across the loop and does
  no scaling arithmetic.
- **`0x800669B0`'s sign asymmetry is real.** `local_18` is
  `position.x - scene.+0x38` but the y value is `scene.+0x3C - position.y`
  (`fsubs f2,f2,f1` at `0x800669E4`, with the operands swapped relative to the x
  axis at `0x800669D4`). The body reproduces it.
- **`0x800668F0`'s root is the translated callee.** `0x802BE470` already exists
  as `crate::resource::archive::fn_802be470`; its `f32` narrowing of the
  `thunk_FUN_802E8620` result matches the `frsp` at `0x80066968`, so the body
  calls it rather than re-declaring the address.

## Verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to a scratch directory, this
lane's block was appended to the copy's `src/game/player.rs`, and
`cargo check --lib` completed with exit code 0 (no diagnostics from the appended
region).
