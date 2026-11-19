# Wave 198 lane D coverage (`game::bases`)

Slice: the number-panel lift actor's redraw entry and rider-tree walker
(`0x806D_6680`, `0x806D_6950`) plus the `0x760`-byte flipper actor's per-frame
step (`0x806E_0110`). Three functions, three translated, none deferred.

| address | size | body |
| --- | --- | --- |
| `0x806D_6680` | 516 | `fn_806d6680` - lazy `0x8082_D840` sound table, digit-animation restart, `0x2B6..0x2BB` sounds, `+0x46C` five-count clamp, `warning` clip on `+0x498` |
| `0x806D_6950` | 536 | `fn_806d6950` - kind-`0x19B`/`0x1A1` rider-chain walk, unique-id list, `0x80`-clamped weight, recursive descent through `0x8011_F3F0`/`0x8016_2D00` |
| `0x806E_0110` | 532 | `fn_806e0110` - `+0x660` timer, `0.08` slide to `+0x64C`, `+0xE4`-driven `0x8082_DEE8`/`0x8082_DF28` state split, `0x2C2` sound |

## Signature recoveries

* `0x806D_6680` is declared `void FUN_806d6680(void)`. The prologue calls only
  `0x802D_CD74` (`_savegpr_26`), and `mr r30,r3` / `mr r31,r4` capture the two
  live arguments: `this` and the new count `index`. `fn_806d6f20` / `fn_806d6b70`
  callers already pass `(actor, count)`.
* `0x806D_6950` keeps all five arguments in `r27..r31` across `bl 0x802D_CD6C`;
  `r5` is accumulated in place and `r7` is both the start count and the return
  value, matching the `fn_806d6b70` call site.
* `0x806E_0110` is a plain one-argument body (no register-save thunk).

## Constants

* `0x8082_D840..0x8082_D850` is the 5-word panel sound table, seeded on first use
  with `0x2B6..0x2BA`; `0x8082_D854` is its guard byte (runtime BSS, never baked
  in). `0x2BB` is the over-five sound and `0x2C2` the flipper's timer sound.
* `0x807C_B6B0` is `1.0`, `0x807C_B6B4` is `0.0` (both fed to `fn_806d5810`),
  `0x807C_B708` is the plain `2^52` magic used for `(f32)count`.
* `0x807C_BAAC` is `0.08` (the flipper slide step) and `0x807C_BA54` is `0.0`.
* `0x807F_B0E0` is the 8-byte string `warning`, looked up in the `+0x3F0` block.

## Ghidra gaps

The `0x802D_CD6C/74` save thunks make Ghidra drop the first arguments of both
panel bodies, and its `puVar2 + 0x11b` style scaling hides the `0x46C` target;
both are recovered from the raw words. `fcmpu`/`fcmpo` plus the `4c411382` /
`4c401382` `cror` folds are expanded into the plain `>=` / `<=` window tests.

## Deliberate call-by-address sites

`0x8006_A4B0` (position copy), `0x8019_8C30`/`0x8019_9390` (sound queue),
`0x8023_A190` (clip lookup), `0x8016_5FD0`/`0x8016_6110` (clip bind/unbind),
`0x8016_6EF0` (count animation), `0x8011_F3F0`, `0x8005_7CC0`, `0x8016_2D00` and
`0x8006_CD40` have no Rust bodies and stay `abi_callN!` / `call_f32_u32x2`. The
`+0x3F4` and `+0x394` interface slots are dispatched through `vmethod1`, the
`+0x60` actor slot through `actor_method0`.
