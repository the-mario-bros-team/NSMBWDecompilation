# Wave 199 lane F coverage (`game::bases`)

Slice: two giant bodies of the course-object sound/spawn script —
`0x8077_3210` (2096 bytes) and `0x8077_40D0` (1656 bytes). Two functions,
two translated, none deferred.

| address | size | body |
| --- | --- | --- |
| `0x8077_3210` | 2096 | `fn_80773210` - 16-entry state machine on `+0x6C444`; zone walk, position pair/triple publication, `1.2f` progress ratio, sound start/stop |
| `0x8077_40D0` | 1656 | `fn_807740d0` - walks the course object list, spawns `0x27E`/`0x282`/`0x2A5`/`0x278`/`0x2A7` families, then the area-mode actors |

## Signature recoveries

* `0x8077_3210` is `undefined4 FUN_80773210(int param_1)` with one live
  argument. Its prologue stores the argument in `r30` and reads the sound
  manager from `[0x8042_A1A0]+0x29C` into `r31`; the return value is `1` only
  for the "finished" steps. `0x8077_3200` is the four-byte thunk that already
  exists in `bases.rs` as `fn_80773200`, so this body is the true callee.
* `0x8077_40D0` is declared `void FUN_807740d0(void)`. Ghidra lost the
  argument to the `0x802D_CD58` save helper; the raw words hold the course
  object in `r21` (`mr r21,r3` at `0x807740E8`) and the routine returns
  nothing (`0x802D_CDA4` restore tail plus `0x80761510(0)`).

## Dispatch tables

* `0x8081_70A4..0x8081_70E0` is the `0x8077_3210` switch table. Entry `6`,
  `10` and `14` point at the epilogue at `0x80773A18` (return `0`); entries
  `0,1,2,3,4,5,7,8,9,11,12,13,15` point at
  `0x80773268, 0x807732BC, 0x80773304, 0x80773338, 0x807733F8, 0x807734F4,
  0x8077362C, 0x80773758, 0x80773810, 0x807738C8, 0x807738DC, 0x8077399C,
  0x807739F0`. States `0x6`, `0xA` and `0xE` are therefore dead.
* `0x8081_70E4..0x8081_7184` is the `0x8077_40D0` name-selector table, indexed
  by `name[2] - '0'` (0..40). Only `0`..`9`, `A`, `C`, `G`, `K`, `T`, `W`, `X`
  point at the spawn body `0x80774390`; every other slot and every value above
  `'X'` goes to the generic re-dispatch at `0x80774508`.

## Constants

* `0x807C_F3B4` is `1.2f` and `0x807C_F3B8` is the `2^52` magic; the pair
  converts the `u8` at `0x8031_C9B2` to `f32` before `fdivs`. `0x807C_F390` is
  `0.0f`. `0x8031_C9B4` and `0x8031_C9B6` are `lha` (signed 16-bit) frame
  thresholds; both are main.dol runtime values, never baked in.
* `0x8037_7DC8/CC/D0` is the main.dol default position triple, copied as raw
  words into the `0x2A7` spawn at the end of `0x8077_40D0`.
* `0x8042_A040` (released with `-1` through `0x800E0380`), `0x8042_A24E`
  (single-player marker byte), `0x8042_9F7C` (class byte array) and
  `0x8042_A1A0` (sound-engine root) are all read through `platform::read_u32`.

## Ghidra corrections applied

* `0x807743E8` — Ghidra swaps the `0x800FCA50` sense: the raw `beq` sends the
  **zero** result to the `0x800FC9C0` block at `0x80774488`, and the non-zero
  result falls through to the `0x395`/`0x39F` spawn block. The Rust body keeps
  the raw order.
* `0x80774304` — the `0x278` flag word is
  `srawi(0x30 - extsb(name[3]), 31) | 0xFF`, i.e. `0xFF` while `name[3] <= '0'`
  and `0xFFFF_FFFF` otherwise. Ghidra folds this into `0x10000 | 0xFF`; the raw
  words clear `r29` with `andc r29,r0,r0` and OR only the sign mask. The body
  follows the words and the discrepancy is noted in the doc block.
* `0x80773210`/`0x80773A1C` — the `ldexpf` GQR guards Ghidra emits around both
  bodies are compiler noise (`GQR0 = 0`); they are not emitted.
* `0x807734DC`/`0x80773660` — `ec211028` is `fsubs f1,f1,f2` (the `2^52`
  subtraction) and `ec200824` is `fdivs f1,f0,f1`; the argument lists of
  `0x800EFE20` (two integers) and `0x800F01B0` (`f32`, slot, record) are taken
  from the register setup, not from Ghidra's `(double)` renderings.

## Deliberate call-by-address sites

`0x8010_06C0` (position fill), `0x800F_29F0`/`0x800F_2A00` (spawn), `0x800F_57A0`
(flag query), `0x800F_31E0` (name lookup), `0x800F_53A0` (name index),
`0x8016_2D50` (object factory), `0x800E_FE20`/`0x800F_01B0`/`0x800F_0800`/
`0x800F_0AD0` (zone-record steps), `0x800F_CA50`/`0x800F_C9C0`/`0x800F_CA20`/
`0x800F_CAE0` (route predicates), `0x80C5_6AD0`/`0x80C5_6270` (sound-engine
hooks), `0x800B_BC50`/`0x800B_BC60`/`0x8076_1510` and the three save/restore
helpers (`0x802D_CD58`, `0x802D_CDA4`, `0x802D_CDB4..0x802D_CDBC`) are left as
`abi_callN!`/`call0_ret`; the save/restore helpers themselves are never emitted.

## Verification

The block was appended to a scratch copy of the crate
(`%TEMP%\opencode\wave199-lanef-crate\src\game\bases.rs`) and `cargo check`
finishes with no errors and no warning inside the appended region. The real
`src/game/bases.rs` was not touched.
