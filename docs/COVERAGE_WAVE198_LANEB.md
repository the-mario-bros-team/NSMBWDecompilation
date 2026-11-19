# Wave 198 lane B coverage (`game::bases`)

Slice: the three remaining 1772-3432 byte bodies of the `0x8065DD50` ..
`0x80661070` cluster (the `Wm_ob_*` rising-base controller). Three translated,
none deferred.

| address | size | body |
| --- | --- | --- |
| `0x8065_DD50` | 1772 | `fn_8065dd50` - state-`0`/`1` anchor submit: converts the moving anchor into a `127`->`0` volume with `fn_8065e4e0`/`fn_8065e460`, posts message `795`, pulls below-limit players toward `height+800`, sorts and seeds the `+0x9CD4` player list, then starts the `0x69`/`0x68` actors along the `+0x9C38`/`+0x9C48` windows |
| `0x8065_F090` | 1772 | `fn_8065f090` - normalises the four player slots, then gates states `0`..`2`: nearest `0x112` actor answering `0x8082917C`, the `0x101` actor's `0x80C6EDCC`/`EE0C`/`EF4C`/`EF8C` clips at `+0x778`, and the `0x80828E1C` event after `90` frames |
| `0x8066_1070` | 3432 | `fn_80661070` - eleven-way state machine (jump table at `0x807E6E74`, states `0,2..5,10..17`): drives the anchor to `+0xAC+6208`, seats the slots on the `-24/24/-48/48` ladder for `0..425` frames, then runs the `0x80C6F230`/`F270`/`F2B0`/`F2F0` clip ladder and finishes with `0x80102260(8,2,0,5)` |

## Compile fixes applied after the first merge attempt

* `call0(0x8010_2220)` is not in scope inside `bases.rs`; it now goes through
  `platform::call0(0x8010_2220)`.
* 18 nested-address sites passed a `u32` where the outer helper wants a
  `usize`: `platform::{read_u32,write_u8,write_u32}(platform::read_u32(X) + off)`
  now reads `platform::read_u32(X) as usize + off`. These are the `+0x394` and
  `+0x60` vtable walks, the `0x8042_9F10 + 0x15C` cell, `0x8042_A070 + 0xC`,
  `0x8042_9D48 + 0xBCA` and `0x8042_A328 + 0x444`/`+0x44C`. One
  `0x807E_73F0 + active * 4` site also needed `active as usize * 4`.

## Signed-timer correction

The raw words compare the frame timer with `cmp` (signed 32-bit), not `cmpl`,
and Ghidra casts both sides to `int`. Every `+0x3D0` gate - and the
`+0x9CCC <= +0x3D0` test in state `12` - is therefore read with
`platform::read_i32`, not `read_u32`. This matters because the state machine
parks the timer at the shipped `-1` sentinel (`0xFFFF_FFFF`) between states: as
an unsigned value that sentinel would satisfy `>= 0`, `>= 7`, `>= 30` and
`>= 660` and the gates would run a frame early. The four `read_i32` sites also
removed the `comparison is useless due to type limits` warnings.

## Verification

The bodies were checked in a throwaway copy of the crate, never in the repo:

```powershell
$d = "$env:TEMP\opencode\w198b"
Copy-Item -Recurse src $d\src
Copy-Item src\game\bases.rs $d\src\game\bases.rs -Force
Get-Content work\wave198\laneB_block.rs -Raw | Add-Content $d\src\game\bases.rs
$env:CARGO_TARGET_DIR="$d\target"; cd $d
cargo check --lib            # exit 0, 0 errors, no warnings from the block's lines
node tools/doc-lint.mjs "$d\src"   # {"ok":true,"files":68,"violations":0}
```

The scratch `src/game/bases.rs` was never copied back; the repo's file only
receives these bodies through the coordinator's `work/wave198/merge.cjs`.

## Deferred

None. Every branch target, jump-table entry, constant and float in all three
bodies is backed by the raw words; the `0x800E7E2x`/`0x800E7E4x`/`0x800E7E5x`/
`0x800E7Exx`/`0x800E80xx` accessor contracts and the `0x80C6xxxx` clip ids are
not described by this REL, which is why all three are rated `medium` rather than
`high`. Two `rlwinm`/`xnor`/`subf.` parity tricks in `0x80661070` state `2` are
emitted from the raw words because Ghidra inverts them: the `-0.5` bias applies
to an odd player count, and the zig-zag offset is `k+1` for even `k` and `-k`
for odd `k`.
