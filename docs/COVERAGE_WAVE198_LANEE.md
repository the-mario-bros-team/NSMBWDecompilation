# Wave 198 lane E coverage (`game::bases`)

Slice: the two `0xE0` spawn-spec collectors (`0x806F2100`, `0x806F2710`) and the
boat-light actor's six-record ring setup (`0x80702FD0`). Three functions, three
translated, none deferred.

| address | size | body |
| --- | --- | --- |
| `0x806F_2100` | 520 | `fn_806f2100` - fills the spawner's `+0x39C` ring from the stage `0xE0` spec list; `(originX + spec[2], originY - spec[4], 1500/-1800 + 80 * (n & 3))`, kept when `0x8008E770` reports the `+0x349` side |
| `0x806F_2710` | 520 | `fn_806f2710` - identical body for the rider-timer ring `+0x3D8`/`+0x3DC` (fourth word `+0x3E8`) |
| `0x8070_2FD0` | 516 | `fn_80702fd0` - reads panes `1` and `4..7` of the `+0x478` scene node and writes the six `+0xB84..+0xDAC` `0x5C`-byte player-switch segments through `0x800DB590`, with a `25.0` overhang and an `8.0` vertical bias |

## Signature recoveries

* `0x806F2100` and `0x806F2710` are declared `void FUN_...(void)`, but both open
  with `bl 0x802DCD64` (`_savegpr_26`) and immediately `mr r24,r3`; `r3` is
  `this`. Ghidra's `in_f28..in_f31` / `unaff_GQR0` scaffolding is only the
  callee-saved FPR store/restore pair at `sp+0x50..0x88`, not an argument list.
* `0x80702FD0` opens with `bl 0x802DCD74` (`_savegpr_28`); `r3` is the
  boat-light `owner`, matching the `FUN_807041b0` call site and the
  `abi_call1!(0x8070_2FD0, actor)` already emitted in that wave's body.

## Constants

* `0x807CC0D8` / `0x807CC0F8` are the signed `int -> f32` bias
  `0x4330000080000000` (`2^52 + 2^31`), not the plain `2^52` magic.
* `0x807CC0E0 = -1800.0`, `0x807CC0E4 = 1500.0`, `0x807CC0E8 = 80.0`;
  `0x807CC100..0x807CC108` repeat the same three at the second site.
* `0x8030C344` / `0x8030C348` are the type-`0xE0` record's `+4`/`+8` world
  origin (`0x8030A040 + 0xE0 * 40`), the same cells `fn_8068f220` reads.
* `0x807CC5D0 = 8.0` (vertical bias), `0x807CC5D4 = 25.0` (segment overhang).
* `0x807CC57D..0x807CC580` hold the pane-id bytes `4, 5, 6, 7`; the first pane
  is the literal `1`.

## Ghidra gaps

Both collectors drop the `this` argument and print four bogus paired-single
conversion blocks; the `subf`/`neg` pair that turns `spec[4] - originY` into
`originY - spec[4]` survives only as a Ghidra `-((uint)x - y)` expression.
`0x80702FD0` loses the `bl 0x802DCD74` receiver, splits its two 60-byte
`0x80168180` scratch blocks, and keeps a dead third component
(`pane.z - actor.z`) on both the first point and the four loop points - those
stores are reproduced in the doc block but omitted from the Rust body.

## Deliberate call-by-address sites

`0x8008E770` (section-side query, already modelled in `resource::course`),
`0x80168180`/`0x8016F130` (pane transform fetch/copy), `0x800DB590` (two-point
segment write, identified from `fn_806ed4e0`/`fn_806f0d00`) and the four
compiler save/restore thunks `0x802DCD64`, `0x802DCD74`, `0x802DCDB0`,
`0x802DCDC0` (omitted) have no Rust bodies in this module and stay
`abi_callN!`.
