# Coverage - Wave 106 (`game::player`, the `0x80040000` bucket)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 11 | 2044 | 0 | 3 | 8 |

Status split: 3 `translated`, 8 `partial`.

## How to read this file

This document covers the last eleven untranslated functions of the
`0x80040000` bucket in `work/ghidra/main-gb/export/functions.tsv`. The bucket
held 256 entries and 245 were already in `ledger/functions.csv` when the pass
started; all eleven survivors were reconstructed, so **the bucket is now
complete**. Every body lives in `src/game/player.rs` and its row is in
`ledger/agents/wave106-player.csv`.

* **Address** - virtual entry point in the DOL image (not a file offset).
* **Name** - the Ghidra name. No original symbol table survives, so
  `FUN_<address>` is the honest identity; the Rust body keeps the
  `fn_<address>` name.
* **Role** - one plain sentence. A role is marked "inferred" when it comes from
  shared field offsets and call shapes rather than a recovered symbol.
* **Status** - `partial` means one piece of the original ABI is knowingly left
  out: for the eight forwarders it is the hidden `r12` member-pointer
  descriptor (and, for `0x8004DB00`, the untouched `r5` argument).
* **Confidence** - strength of the evidence, not code quality: `medium` = a
  field role, the `GQR3` halfword conversion or a callee's meaning had to be
  inferred; `low` = only the literal instruction sequence is certain.

The bodies use the SDA bases this module already fixes, `r2 = 0x80433080` and
`r13 = 0x8042F680`. The constants read for this pass were re-read from
`work/dol/main.elf`:

| address | value | use |
|---|---|---|
| `0x8042B7C8` | bytes `01 02 04 08` | per-slot presence bit table |
| `0x8042B7DC` | `0.125` | ring angle step scale |
| `0x8042B7E0` | `16384.0` | first ring angle (a quarter turn) |
| `0x8042B7E4` | `0.0` | zeroed `+0x394..+0x39C` |
| `0x8042B7EC` | `16.0` | x of the ring's closing record |
| `0x8042B7F0` | `32.0` | closing `+0x3A0`/`+0x3A4` offset |
| `0x8042B7F4` | `8.0` | closing `+0x394`/`+0x398` offset |
| `0x8042B874` | `1.0` | `abs(speed)` gate of the landing branch |
| `0x8042B878` | `0.0` | "stopped" test |
| `0x8042B8C4` | `0.5` | `+0x11C` halving when the caller passes 0 |
| `0x8042B8C8` | `1.1` | `abs(speed)` gate of the stop branch |
| `0x80429C78` | BSS | ring radius, read at run time |

## What this pass covers

- **The eight `0x802DCBFC` pointer-to-member forwarders, `0x800408F0..0x8004DB00`
  (8 bodies).** Seven of them are byte-identical 48-byte bodies whose whole
  content is `0x802DCBFC(second, second, first)`; `0x8004DB00` is the 52-byte
  variant `0x802DCBFC(first, 0x80308300)`. Disassembling `0x802DCBFC` from
  `work/dol/main.elf` shows what the missing argument is: it reads a 12-byte
  descriptor from the **hidden `r12`** (`lwz r0,0(r12)` / `lwz r11,4(r12)` /
  `lwz r12,8(r12)`), adds `r0` to `r3` as the this-adjustment, then loads the
  member function out of the adjusted object's vtable and `bctr`s to it.
  `platform::call3` cannot set `r12`, so the forwarder bodies keep the literal
  argument reshuffle and are marked `partial`/`low` - the same call the wave-103
  lane made for the `0x80059260`/`0x80059290` twins.
- **The presence-mask driver `0x800441E0` (432 B).** It walks the owner's
  `+0x3E0` chain (`+0x04` actor, `+0x0C` next), and for every linked actor of
  kind `1`/`2` that is not already "present" (actor slot `+0xE0`) and has not
  set event flag `0x45`, it sets flag `0x5C`, asks actor slot `+0x6C` for the
  address of the actor's slot byte, ORs the slot's bit out of the byte table at
  `0x8042B7C8`, and stamps `0x28` frames into `owner + 0x450 + slot`. It then
  decrements all four countdowns and raises any bit whose countdown is still
  live, calls `0x800440C0(owner, mask)`, writes that function's returned byte
  through the caller's out pointer, and returns the mask. The prologue
  `FUN_802dcd68()` is Ghidra's model of the outlined register-save stub; the
  disassembly (`lwz r26,0x3e0(r3)` / `mr r30,r4` / `stb r3,0(r30)`) pins the
  real argument order, which is the reverse of what the packed prologue value
  suggests.
- **The attachment-ring builder `0x80044550` (520 B).** Eight 8-byte records at
  `+0x404..+0x443` plus a closing record at `+0x444`. Each record holds the
  global radius `0x80429C78` (the closing record holds `16.0`) and the
  truncated running angle; the same halfword is dequantised and fed to the
  cosine/sine helpers, whose scaled results accumulate into `+0x394`/`+0x398`.
  `+0x3A0`/`+0x3A4` copy those sums and add one more `32.0`-scaled pair, then
  `+0x394`/`+0x398` take one more `8.0`-scaled pair.
- **The ground-speed damper `0x80048630` (704 B).** It sets event flag `0x30`,
  refreshes the ground contact (`0x800536B0` in stage mode `5`, `0x800536A0`
  otherwise), consumes a pending ground-angle record (`+0x10D4 & 1`, positive
  `+0x112A`, non-positive `+0x112C * +0x112E`) by scaling `+0x10C` with the
  cosine and `+0xEC` with `abs(speed) * sin`, and then dispatches the
  landing/stop behaviours: the `+0x48C = 3` / `+0x14A8 = 8` pose with the
  `0x80070820` animation query, the two attack poses `0x800484B0` /
  `0x80048410`, or `0x800485A0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800408F0` | `fn_800408f0` | 48 | low |
| `0x80041BA0` | `fn_80041ba0` | 48 | low |
| `0x80041BD0` | `fn_80041bd0` | 48 | low |
| `0x80043770` | `fn_80043770` | 48 | low |
| `0x800437A0` | `fn_800437a0` | 48 | low |
| `0x800441E0` | `fn_800441e0` | 432 | medium |
| `0x80044550` | `fn_80044550` | 520 | medium |
| `0x80045190` | `fn_80045190` | 48 | low |
| `0x800451C0` | `fn_800451c0` | 48 | low |
| `0x80048630` | `fn_80048630` | 704 | medium |
| `0x8004DB00` | `fn_8004db00` | 52 | low |

Byte total: 2044. Address range covered: `0x800408F0..0x8004DB00`.

## Deliberate skips and why

**None.** All eleven remaining functions of the bucket have a body. The eight
forwarders are `partial` rather than skipped because their entire reachable
behaviour - the argument reshuffle and the tail call - is reproduced; only the
`r12` receiver that `platform::call3` cannot express is missing, and each body
says so with a `// partial:` comment.

## Notes for a later pass

- The `psq_l` halfword conversions in `0x80044550` and `0x80048630` read through
  **`GQR3`**. A scan of every `mtspr`/`mtdcr`-encoded GQR write in
  `work/dol/main.elf` finds writes for `GQR0` and `GQR1` only
  (`0x801B2788`, `0x801B2844`, `0x802EA7E4`, ...), so the register is left at
  its boot value. Both bodies therefore reduce the conversion to
  `(s16) value * 0.00390625`, the identical convention the wave-103 bodies use
  for `0x8042B8C0`; if a future pass recovers the boot GQR setup this is the
  one assumption to revisit.
- `fn_800440c0` (wave 72) takes its two words as `(mask, owner)`, while both the
  original call site inside `0x800441E0` (`mr r3,r29` = owner,
  `rlwinm r4,r31,0,24,31` = mask) and the prologue of `0x800440C0` itself
  (`lbz r0,0x440(r3)`) show that the shipped ABI is `(owner, mask)`. `fn_800441e0`
  therefore calls the address directly with the literal register order instead
  of the helper, and says so at the constant's definition.

## Verification

- Every address and byte size matches `functions.tsv` (`work/ghidra/main-gb/export/functions.tsv`,
  rows `800408f0` 48, `80041ba0` 48, `80041bd0` 48, `80043770` 48, `800437a0` 48,
  `800441e0` 432, `80044550` 520, `80045190` 48, `800451c0` 48, `80048630` 704,
  `8004db00` 52).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/game/player.rs`.
- No address was already in `ledger/functions.csv`; all eleven were the bucket's
  remaining untranslated rows.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/player.rs`.
- `node tools/check-lane-csv.mjs wave106` reports `OK` with 11 rows in this lane.
