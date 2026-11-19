# Wave 197 lane D coverage (`game::bases`)

Slice: `dCourseSelectManager_c` remainder plus the `0x807C5B10` panel
transform (`0x807C09B0` .. `0x807C5B10`). Six functions, six translated, none
deferred.

| address | size | body |
| --- | --- | --- |
| `0x807C_09B0` | 196 | `fn_807c09b0` - `W4A0`/`W6A0`/`W8C0` course-name test, `(u32, u32) -> u32` |
| `0x807C_0B20` | 232 | `fn_807c0b20` - name test plus `0x800F9970` kind window and `0x400`/`0x800` anchor probes |
| `0x807C_0FD0` | 204 | `fn_807c0fd0` - resets the selection view (`+0x6F8`, `+0x740`, `+0x6FC`, `+0x700`, `+0x604`) |
| `0x807C_1C80` | 196 | `fn_807c1c80` - opens the `+0x53C` phase once, clears `+0x53D..+0x54D` |
| `0x807C_21B0` | 220 | `fn_807c21b0` - four-player resync through `0x80354E60`/`0x80354E90` and `+0x70` slot `+0x18` |
| `0x807C_5B10` | 248 | `fn_807c5b10` - panel matrix: offset position, `MTXTrans`, then the three angles |

## Signature recoveries

* `0x807C09B0` is declared `void FUN_807c09b0(void)` by Ghidra, but the caller
  `fn_807c0a80` (`bl 0x807C09B0` at `0x807C0AD0`) sets `r3` and `r4` first and
  tests the returned `r3`. The real signature is
  `fn_807c09b0(_unused: u32, id: u32) -> u32`:
  - `r3` is dead - `lis r3,1` at `0x807C09C8` overwrites it before any read;
  - `r4` is live and is forwarded untouched as the second argument of
    `0x800F56B0(0x8042A18C + idx*0x16518 + 0x3394, id)`.
* `0x807C0B20` loses both arguments for the same reason (`bl 0x802DCD74` is the
  `_savegpr_27` helper, not a call). It is `fn_807c0b20(unused: u32, id: u32)
  -> u32` and passes the pair straight into `fn_807c09b0`.
* `0x807C21B0` uses `bl 0x802DCD70` (`_savegpr_26`); `r3` survives, so the body
  is `fn_807c21b0(this: usize)`.

## Constants

* `0x8082152C` is a three-word pointer table; its entries point at the 4-byte
  names `W4A0`, `W6A0`, `W8C0` (`0x80821514`, `0x8082151C`, `0x80821524`) and its
  bound word `0x80821538` holds `3`.
* `0x807D0A6C` is `0.0`, `0x807D0BB8` is `0.5`, `0x807D0BBC` is `1000.0`,
  `0x80821594` is `600.0`, `0x807D0B88` is the player order `[0, 1, 3, 2]`.
* `0x80834D30..0x80834D84` is the tail of the `d_basesNP` BSS segment, so the
  `0x80834D40/44/48` position offsets and the `0x80834470` manager singleton are
  runtime values; they are read, never baked in.

## Ghidra gap in `0x807C5B10`

The export stops after `bl 0x8016EF50`, but the raw words
`0x807C5BA8..0x807C5BF0` are executed on return and share the frame that the
epilogue at `0x807C5BF4` restores. Ghidra dropped the tail because it treats
`0x8016EF50` as non-returning. The tail only loads the global at `0x80429E00`
(guarded against null) and scales four of its floats into stack slots that
nothing reads, so it is reproduced verbatim but flagged in the doc block; that
is the only reason this body is `medium` rather than `high`.

## Deliberate call-by-address sites

`0x802E1BD0` (4-byte compare), `0x800F9530`, `0x800FAD10`, `0x8010_5240`,
`0x8010_55D0`, `0x8001_0290` (the `+0xC8` helper query) and `0x8006_0110` have
no Rust bodies and stay `abi_callN!`. `0x807C3B30` is called by name because
its body already exists, and the `+0x70` virtual slot is dispatched through
`vmethod1`, matching `fn_807c3260` / `fn_807c22c0`.
