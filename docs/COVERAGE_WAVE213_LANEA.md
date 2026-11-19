# Coverage - Wave 213 lane A (`game::player`, main.dol)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 10 | 920 | 9 | 1 |

Addresses `0x800693E0`-`0x80069814`: the second "find an interaction target"
query, the ownership latch byte and its mask lookup, four sound-manager tail
branches, the kind-to-mask decision tree, and the `0xD01000` record constructor
with its five hooks. They sit between the translated nearest-target query
`0x80069270` and lane B's `0x80069820` sound-record cluster, and touch nothing
of the sequence-player table at `0x803FC990`.

SDA/data owned or reused by this lane:

| address | bias | role | status |
|---|---|---|---|
| `0x8042A4A8` | `r13 - 0x51D8` | game-sound object | reused (`W74_SOUND_8042A4A8`) |
| `0x8042A488` | `r13 - 0x51F8` | UI/event dispatcher | reused (`W18_UI_8042A488`) |
| `0x80429D68` | `r13 - 0x5918` | ownership latch byte (`0x80069500` sets, `0x80069510` clears) | new (`W213A_LATCH_80429D68_A`) |
| `0x80424D80` | `r13 - 0xA900` | word whose bit `4` picks `0x14` over `0x12` | new (`W213A_FLAGS_80424D80_A`) |
| `0x80315B98` | image | shared mode word read by three arms | new (`W213A_MODE_80315B98_A`) |
| `0x802EF3A8` | image | slot -> ownership mask table | new (`W213A_TABLE_802EF3A8_A`) |
| `0x802EF3B8` | image | vendor word table, `selector & 0xF` | new (`W213A_TABLE_802EF3B8_A`) |
| `0x802EF3E0` | image | vendor word table, `selector & 3` | new (`W213A_TABLE_802EF3E0_A`) |
| `0x8042BBF0` | `r2 - 0x7490` | `65536.0f`, squared-distance ceiling | new (`W213A_CEILING_8042BBF0_A`) |
| `0x8030F044` | image | constructor descriptor of `0x8016E630` | new (`W213A_DAT_8030F044_A`) |
| `0x800695A0`..`0x800695E0` | image | the five hook thunks | new (`W213A_HOOK_*_A`) |

## What landed

- **Second target query.** `0x800693E0` (276 B) walks the four player slots with
  `0x8005F900(slot)`, skips the excluded actor, the actors for which
  `0x8004DD00` is non-zero and the actors carrying event flag `0x53`, and keeps
  the *nearest* survivor measured on the squared x/y distance from the
  three-float origin. Its bound starts at `65536.0f`, so nothing is accepted
  beyond `256.0f` of real distance.
- **Latch pair.** `0x80069510` clears and `0x80069520` reads the byte at
  `0x80429D68`; the setter is the 12-byte `0x80069500` (other lane).
  `0x80069530` (76 B) returns `0x802EF3A8[slot]` only while that byte is clear.
- **Sound forwards.** `0x80069580`, `0x80069590`, `0x800695F0`, `0x80069600`
  (8 B each) load the sound object from `0x8042A4A8` and tail-branch to
  `0x8019CE70`, `0x8019CEF0`, `0x8019CFB0` and `0x8019D040`.
- **Kind tree.** `0x80069610` (268 B) is a compare tree over `r3` with two
  table lookups on the low bits of `r4`; see the table in the body's doc block.
- **Record constructor.** `0x80069720` (248 B) allocates the `0xD01000`-flagged
  object with `0x8016E630`, brackets the work with `0x802B9170()`, clears and
  fills a 32-byte stack block (`+0x00 = object`, `+0x1C = 0xD00000`), posts it
  through `0x8042A488`'s vtable slot `+8`, installs three hooks at
  `+0x1E4/+0x1E8/+0x1EC` of `0x80109340()` and two at `+0x34/+0x38` of
  `0x80107F10()`, then runs the object's own vtable slot `+0x2C`.

## Corrections and recoveries

- **`0x800693E0`'s comparison direction.** The export renders the update as
  `dVar6 <= dVar7`, and the raw words agree: `0x800694B0 fc00f840` is
  `fcmpu cr0,f0,f31`, `0x800694B4 4c401382` is `cror 2,0,2` (i.e.
  `cr0.EQ := cr0.LT | cr0.EQ`, "`<=`") and `0x800694B8 4082000c` is `bc 4,2`,
  which is `bne`, not `beq` - `BO=4` branches when the tested CR bit is clear.
  The update therefore runs on `distance <= bound` and the search keeps the
  **nearest** candidate. Reading `bc 4,2` as "branch if equal" would invert the
  loop into a farthest-first search; the `ble` at `0x80069090` in the already
  translated `0x80069060` (`bc 4,1` = "skip when `count <= 0`") is the same
  encoding and confirms the polarity.
- **`0x800693E0`'s GQR prologue is a `f31` save.** The export's
  `ldexpf`/`CONCAT13`/`CONCAT12` block is Ghidra's model of `psq_st f31,0x48(r1)`
  + `stfd f31,0x40(r1)`: one single-precision accumulator is saved and restored
  around the loop and no scaling arithmetic happens.
- **`0x800693E0`'s `r3` argument is dead.** The body stores the position buffer
  address into `r3` before each vtable call and re-loads the actor into `r3` at
  `0x8006946C` before `0x80056CF0`, so only `r4` (excluded actor) and `r5`
  (origin) carry information. [`fn_800583a0`] calls it with `(probe, player,
  body + 0xCC)`, which matches.
- **`0x80069530`'s guard selects the table when the latch is *clear*.**
  `0x80069548 cmpi cr0,3,r3,0` + `0x8006954C 4182000c` is `beq` to the table
  load, so a zero latch reads `0x802EF3A8[slot]` and a non-zero one returns `0`.
  Ghidra's `if (iVar1 == 0)` is right; the module's earlier note on
  `0x80069530` ("or `0` while `0x80069520` is non-zero") agrees.
- **`0x80069610` really hangs on kinds `1`, `2` and above `0x2C7`.**
  `0x80069718 48000000` is a branch to itself, reached from
  `0x80069630`/`0x80069668`. The block keeps a `loop { spin_loop() }` rather
  than inventing a return value; every handled kind has its own `li`/`blr` pair.
- **`0x80069720`'s block is 32 bytes at `r1 + 8`.** The two stores are
  `stw r30,0x8(r1)` and `stw r31,0x24(r1)`, i.e. block offsets `+0x00` and
  `+0x1C`; the export's `local_88[7]` / `local_6c` split is that same block,
  and `0x802D4830` clears it first.
- **`0x80069720`'s hooks are code, not data.** The export prints
  `&DAT_800695a0`, `&DAT_800695b0`, `&LAB_800695c0`, `&DAT_800695d0`,
  `&LAB_800695e0`; all five are eight-byte tail branches (to `0x8019CD30`,
  `0x8019CD70`, `0x8019CE00`, `0x80194D40` and `0x80194DF0`). They are named by
  address with a note so they are not mistaken for tables.

## Verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to a scratch directory
(`%TEMP%\opencode\w213a`), this lane's block was appended to the copy's
`src/game/player.rs`, and `cargo check --lib` finished with exit code 0 and no
diagnostics from the appended region (the 23 remaining warnings all belong to
other pre-existing files).
