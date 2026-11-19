# Coverage - Wave 98 (`game::worldmap`, `0x80130000` bucket remainder)

## How to read this file

Each row is one `pub unsafe fn` body appended to `src/game/worldmap.rs`, named
`fn_` plus its original `main.dol` address. Byte sizes come from
`work/ghidra/main-gb/export/functions.tsv`; every evidence path is the Ghidra C
export the body was transcribed from. Field offsets are literal offsets from the
object pointer. `r13` is `SDA_BASE` (`0x8042F680`) and `r2` is `SDA2_BASE`
(`0x80433080`); every `r2` float and every `DAT_8037xxxx` vector entry used below
was resolved against `work/dol/main.elf`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 26 | 7,708 | 14 | 12 |

Every row is `translated`; no body is `stub`, `partial` or `low` confidence.

## What this pass covers

- **`0x80130CE0..0x80133EC0` - the ride/lean frame group.** These handlers all
  work on the `+0xEA4` sub-controller, the `+0x2A64` ride object (whose vtable
  lives at offset `0` of *that* object, hence the bare `vcall_*` helpers rather
  than `vcall60_*`), the `+0x10D4`/`+0x10D8` input words and the
  `+0x14A0`/`+0x2B8` variant pair. `0x80130CE0` clamps `|+0x2068|` up to `0.1`
  and mirrors the `0x801611C0` angle around `0x4000`. `0x801313D0`,
  `0x80131700` and `0x80131790` are the three mount entries (vector entries
  160, 31 and 32). `0x80132440` posts the `0x80375FF0` touch-down event, and
  `0x80132530` is the `0x802F4E64`/`0x802F4E80` speed lookup used by
  `0x801326B0`, which snaps `+0xB0` to a `0x10` grid and advances on surface
  `8`. `0x801327F0` resets the actor, `0x80132C70`/`0x80132D40`/`0x80132DF0`
  are the per-frame steps, and `0x80133EC0` maps the `+0x1DC8` direction byte
  through the `r2-0x5790` table onto a vector entry.
- **`0x80139DE0..0x8013F6E0` - the partner sequence.** The second run is keyed
  by the `+0x1458` phase / `+0x145C` phase timer pair and the `+0x100C` link
  resolved through `0x80162D00`. `0x80139DE0`/`0x80139E90` are the mount/dismount
  vectors, `0x8013A180` is the leader contact reaction (two `4.0`-lifted probe
  points through `0x80075FD0`/`0x80075CA0` and a `3.0` sideways step),
  `0x8013A430`/`0x8013A540` start and end the sequence, `0x8013A670` steps the
  mount against the `+0x2A78` ownership id, and `0x8013BF10` is the ride
  controller that gates on the `0x80168080` height. `0x8013EA90` runs the six
  `+0x1458` phases over `0x8005D090`/`0x8005D0C0` and the `+0x154C` reset,
  `0x8013EE00` is the three-phase cutscene, `0x8013EF50` is the hand-over,
  `0x8013F1C0` places the actor on the `0x800CFEA0` contact point, and
  `0x8013F350`/`0x8013F570`/`0x8013F6E0` are the approach phases.
- **Shared notation.** The `0x80370DCC` table ([`WM_VECTOR_TABLE`]) is the
  3-float vector array every `DAT_8037xxxx` operand in these exports belongs to;
  the `W98_VECn_*` constants spell each entry as `WM_VECTOR_TABLE + n * 12`
  (entry 0 = `0x80370DCC`, 6 = `0x80370E14`, 160 = `0x8037154C`). The
  `0x4330000080000000` bias double at `r2-0x58E0` is wrapped by
  `w98_s32_to_f64`, and the `+0x5C` "apply vector" slot has three shapes in this
  group: three floats plus id (`vcall_f64_f64_f64_u32`), two floats plus id
  (`vcall_f64_f64_u32`), and three floats only (`w98_vcall_f64x3`).
- Already-translated neighbours are reused by name instead of by address:
  [`wm_set_mode_byte`] (`0x80147250`), [`wm_anim_table_entry`] (`0x80142510`),
  [`wm_landing_gate_passed`] (`0x8013F100`), [`wm_find_actor_state_one`]
  (`0x8012DBE0`), [`fn_8013aaa0`] (`0x8013AAA0`), [`fn_8013b9c0`],
  [`fn_8013a920`], [`fn_8013a9d0`], [`fn_8012e510`], [`fn_8012e5a0`] and
  [`fn_801474f0`].

## Functions

| address | function | bytes | confidence |
|---|---|---|---:|
| `0x80130CE0` | `fn_80130ce0` | 184 | medium |
| `0x801313D0` | `fn_801313d0` | 148 | high |
| `0x80131700` | `fn_80131700` | 144 | high |
| `0x80131790` | `fn_80131790` | 220 | high |
| `0x80132440` | `fn_80132440` | 232 | high |
| `0x80132530` | `fn_80132530` | 68 | high |
| `0x801326B0` | `fn_801326b0` | 312 | medium |
| `0x801327F0` | `fn_801327f0` | 644 | high |
| `0x80132C70` | `fn_80132c70` | 204 | high |
| `0x80132D40` | `fn_80132d40` | 176 | high |
| `0x80132DF0` | `fn_80132df0` | 360 | medium |
| `0x80133EC0` | `fn_80133ec0` | 144 | medium |
| `0x80139DE0` | `fn_80139de0` | 168 | high |
| `0x80139E90` | `fn_80139e90` | 152 | high |
| `0x8013A180` | `fn_8013a180` | 688 | medium |
| `0x8013A430` | `fn_8013a430` | 260 | medium |
| `0x8013A540` | `fn_8013a540` | 292 | high |
| `0x8013A670` | `fn_8013a670` | 524 | medium |
| `0x8013BF10` | `fn_8013bf10` | 668 | medium |
| `0x8013EA90` | `fn_8013ea90` | 636 | medium |
| `0x8013EE00` | `fn_8013ee00` | 324 | high |
| `0x8013EF50` | `fn_8013ef50` | 132 | high |
| `0x8013F1C0` | `fn_8013f1c0` | 388 | medium |
| `0x8013F350` | `fn_8013f350` | 360 | medium |
| `0x8013F570` | `fn_8013f570` | 172 | medium |
| `0x8013F6E0` | `fn_8013f6e0` | 108 | high |

## Deliberate skips

The bucket had 51 untranslated entries at the start of the pass. Twenty-six were
written and the remaining twenty-five are listed here.

- **Paired-single / GQR records: `0x80130F40` (1,160), `0x80131990` (1,096),
  `0x80131F20` (780), `0x80132580` (292), `0x80132A80` (496), `0x80133950`
  (136), `0x80133B60` (324), `0x80135070` (260), `0x80135810` (196),
  `0x801358E0` (224), `0x801359C0` (608), `0x801363C0` (644), `0x80137320`
  (492), `0x80137E20` (676), `0x80138510` (416), `0x801396C0` (268),
  `0x8013A880` (160), `0x8013B0B0` (200), `0x8013BB90` (572), `0x8013E850`
  (204).** Every one of these carries a `GQR` save/restore pair
  (`unaff_GQR0`/`unaff_GQR3` plus `ldexpf`) whose arithmetic and float operands
  cannot be recovered from the C alone. The three that wave 93 already skipped
  (`0x80137E20`, `0x80138510`, `0x801396C0`) stay skipped for the same reason;
  they are still called by raw address from the bodies above.
- **`0x80132340` (244).** The body ends in an `0x8008FBA0` call with eleven
  arguments, one of them the never-initialised `in_r10`; the last three are
  `float` sums forwarded as `double`. Submitting the argument list would
  misstate the effect spawn, so no body was written.
- **`0x80133E10` (76).** The export reports "Could not recover jumptable at
  `0x80133E58`" and replaces the tail with an indirect call, so the vector
  selector it is supposed to compute is missing.
- **`0x80137550` (52).** Calls `0x80137590` with no arguments; the callee needs
  the actor. Same defect wave 93 documented.
- **`0x8013EFF0` (268).** Calls `0x80132340` with no arguments although that
  callee takes the actor, so the forwarded operand is lost.
- **`0x80130340` (444).** The body reads its float limit from an unrecorded
  `unaff_r2 - 0x58C8` slot inside a GQR-wrapped block and then spawns an effect
  with `DAT_803089B0`; the operand list is not recoverable.
- The other 175 entries of the bucket were already translated by waves 27, 93,
  94 and 97, and no address is duplicated here.

## Verification

- Every address and byte size matches `functions.tsv`; the 26 sizes sum to
  7,708 bytes.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv`; all 26 were untranslated
  entries of the `0x80130000` bucket at the start of the pass.
- `cargo check --message-format=short` reports no error or warning that names
  `src/game/worldmap.rs`. The remaining crate errors at the end of the pass were
  in `src/audio/ax.rs` and `src/runtime/os.rs` (other parallel lanes).
- `node tools/check-lane-csv.mjs wave98` reports `OK`.
