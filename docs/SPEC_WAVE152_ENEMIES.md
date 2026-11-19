# Wave 152 - implementation specification for 12 untranslated `d_enemiesNP` bodies

## What this page is

`docs/ANALYSIS_WAVE127_ENEMIES.md` reclassified 384 formerly-skipped
`d_enemiesNP` addresses and waves 128-151 worked through that block. This page
takes the twelve largest bodies that are **still untranslated**, carry a
`decompiled = true` Ghidra export, and walk cleanly from their entry.

A live scan of `work/ghidra/rel-d_enemiesNP/export/functions.tsv` (3,555 rows),
the `address` column of `ledger/functions.csv` (17,075 distinct addresses), and
every `0x80Axxxxx` / `0x80Bxxxxx` address in a `^#` heading of any
`docs/*.md` leaves **66** bodies that are absent from the ledger, absent from
every heading, and carry an export.

Seven of those 66 are **already defined** in `src/` without a ledger row - the
`daEn...` tail of `src/game/enemies.rs` (`0x80AA1030`, `0x80AA3A90`,
`0x80AAE360`, `0x80AAE390`, `0x80AAF2B0`, `0x80AAFCB0`, `0x80AAFCE0`), the same
unledgered-body situation that `docs/SPEC_WAVE151_ENEMIES.md` hit with
`0x80A10220`. They are **not** work items and none of them is claimed below;
the effective work list is **59**. Every other `fn_xxxxxxxx` / `FN_XXXXXXXX`
token a candidate owns is a `const ...: usize = 0x...` address stub or a
doc-comment reference inside an untouched caller, not a body. The twelve
sections below are the twelve largest of the 59.

This page is **read-only**: it writes no Rust, no ledger row and no other doc.

## Selection

The ranking metric is the **reachable body**, not the raw `functions.tsv`
extent, because the TSV size is a bucket extent that can include a neighbour's
padding or an unreachable arm (see `docs/REL_ELF_MAPPING.md`). Each candidate
was walked from its entry through every conditional branch, every internal `b`
and every **linked** indirect branch (`bctrl`), until all paths ended in
`bclr`, `blr`, a tail `b` or a tail `bcctr`. Conditional `bclr`/`bcctr` forms
were followed on both the taken and the fall-through edge.

| # | address | TSV size | reachable insns | `qr3` sites | role | verdict |
|---:|---|---:|---:|---:|---|---|
| 1 | `0x80A965D0` | 280 | 70 | 0 | platform per-player ride registration after `0x80A96500` snaps a falling player | **conditional** |
| 2 | `0x80AC1800` | 276 | 69 | 0 | `daEnMechaKoopa_c` clip binder: `walk`/`damage`/`sign`/`recover` chr + tex-pat pair | **ready** |
| 3 | `0x80B16A80` | 276 | 69 | 0 | `daEnUnizoo_c` eye/mouth node offset writer (`strcmp` against `"eye"`/`"mouth"`) | **ready** |
| 4 | `0x80B27DC0` | 272 | 68 | 0 | per-slot angle -> speed factor, `1 +/- k`, sign from `+0x8B8` | **ready** |
| 5 | `0x80A80F40` | 264 | 66 | 1 | `daEnIgaKuribo_c` swing/heading frame step, 208..768 amplitude clamp, `+0x65C` countdown | **ready** |
| 6 | `0x80B1AD30` | 264 | 66 | 0 | `daEnWalkPakkun_c` ground-snap step plus the ride/attach completion tail | **ready** |
| 7 | `0x80AF5C60` | 260 | 65 | 2 | `daEnRotBlock_c` guide-angle placement: pivot + `0x644/0x648` radius offset | **ready** |
| 8 | `0x80A5E590` | 252 | 63 | 0 | `daEnGabon_c` state entry: binds `"wait"`, seeds `+0x638`/`+0x5FC`/`+0x360` | **conditional** |
| 9 | `0x80A7ACA0` | 252 | 63 | 0 | two-table ground height probe, returns `1` on a hit and `0` after a facing flip | **ready** |
| 10 | `0x80AE0780` | 248 | 62 | 0 | per-player `0x3C`/`0x87` effect burst at every player with flag `0x53` clear | **ready** |
| 11 | `0x80AB12E0` | 244 | 61 | 0 | per-section texcoord/colour override over the model at `+0x684` | **ready** |
| 12 | `0x80AC0750` | 240 | 60 | 0 | launch-angle solver: 32-unit sphere clamp, `atan2` from the guide position | **ready** |

**Split: 10 ready, 2 conditional.** Total 3,128 bytes of TSV extent and 782
reachable instructions. All twelve walk to a terminal instruction at exactly
`entry + size - 4`, so `reachable == extent` for every row and there is no
truncated arm, no out-of-range branch and no `bctr` jump table.

### Rows below the cut

| address | TSV size | reachable | why it is not claimed |
|---|---:|---:|---|
| `0x80A4B140` | 236 | 59 | crow lateral-offset query; one `bcctrl` on an unnamed crow sub-object, left for wave 153 |
| `0x80A637E0` | 236 | 59 | three `bcctrl` sites through a state controller; needs the slot map first |
| `0x80AD45E0` | 224 | 56 | clean two-call body; no constants worth pinning |
| `0x80A80E50` | 220 | 55 | one `qr3` site; sibling of body 5, next in size order |
| `0x80A5DBC0` | 212 | 53 | one `qr3` site; two-call body |
| `0x80A06220` | 212 | 53 | wave-127 class C row ("export prints a callee with no arguments") |

## The `qr3` pool at this wave

`docs/REL_ELF_MAPPING.md` pins **`GQR3 = 0x00050005`** (LD/ST type 5 = unsigned
16-bit, both scales 0), installed by `main.dol` at `0x8025EEEC`/`0x8025EEF0`.
The RAW stream of this REL contains no `mtspr GQRn`. Two of the twelve bodies
read `qr3`; both are single-element (`W = 1`) loads of a stack halfword that
the body itself stored one instruction earlier:

| address | size | `qr3` sites | where |
|---|---|---:|---|
| `0x80A80F40` | 264 | 1 | `psq_l f1,0(r4),3,1` at `0x80A80FD8`, `r4 = r1 + 8` |
| `0x80AF5C60` | 260 | 2 | `psq_l f1,0(r4),3,1` at `0x80AF5CB8` (`r4 = r1 + 10`) and `0x80AF5CD8` (`r3 = r1 + 8`) |

In every case the sequence is `sth rX,off(r1)` -> `psq_l f1,off(r1),3,1` ->
`fmuls f1,f0,f1` with `f0 = 0.00390625 = 1/256` (`0x80B2E320` in body 5,
`0x80B31460` in body 7). With `GQR3 = 0x00050005` the load returns
`f1 = (f32)(u16)halfword` with no scale, so the whole pair reduces to
`turns = (f32)(u16)angle / 256.0f`. The Ghidra `unaff_GQR3` / `ldexpf` blocks
are cosmetic.

No other body in the twelve uses a non-zero quantiser. Every other
`psq_l`/`psq_st` word is a `qr0` `f31`/`f30` stack spill beside a matching
`stfd`/`lfd`.

## Reading conventions

* Synthetic base is `0x80A00000`. The REL has five file-backed `PT_LOAD`
  segments (`0x80A000F0`, `0x80B2A518`, `0x80B2A7A4`, `0x80B2A7B0`,
  `0x80B32DC0`); `docs/REL_ELF_MAPPING.md`'s `+0x1000` formula applies to the
  first. The file-backed image ends at `0x80B6E438`; everything above that is
  BSS.
* `this` is the incoming `r3` unless stated otherwise; `+0xNNN` are byte
  offsets from `this`. `+0xAC`/`+0xB0`/`+0xB4` are x/y/z, `+0xD0`/`+0xD4`/
  `+0xD8` the position offset triple, `+0xE8`/`+0xEC`/`+0xF0` the velocity
  triple, `+0x100`/`+0x102`/`+0x104` the three `s16` angles, `+0x348` the
  side/facing byte, `+0x38C` the actor kind byte and `+0x38F` the ground kind
  byte.
* **`r0` in an `addi`/`addis`.** An `addi rX,r0,imm` is `li rX,imm`, not a
  read of `r0`. The listings quote the register form; every pseudo-code block
  below resolves it to the literal.
* **CodeWarrior prologues.** `0x802DCD54/68/70/74` are `_savegpr_*` helpers and
  `0x802DCDA0/.../C0` their `_restgpr_*` partners; they are not
  argument-destroying stubs and their incoming `r3..r10` survive. Ghidra's
  "64-bit return" from them is the saved register pair.
* Sizes are the `size` column of `functions.tsv`; "reachable" is the count of
  distinct instruction words on a path from the entry.

---

## 1. `0x80A965D0` - platform per-player ride registration

| field | value |
|---|---|
| TSV size | 280 bytes |
| reachable | 70 instructions, ends at `0x80A966E4` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80a965d0_FUN_80a965d0.c` |
| signature | `void fn_80a965d0(usize rec_a, usize rec_b)` |
| verdict | **conditional** |

### What the stream does

```c
// rec_a is the platform-side callback record, rec_b the player-side one.
usize platform = *(u32*)(rec_a + 4);      // r25 = r3, r28 = r3[+4]
usize player   = *(u32*)(rec_b + 4);      // r29 = r4[+4]
u8    kind     = *(u8*)(player + 0x38C);  // 1 = player, 2 = carried object

if ((kind == 1 || kind == 2) &&
    !(kind == 2 && *(u8*)(rec_b + 0x2D) == 0x0F))
{
    u16 mark = (kind == 2) ? 2 : 1;                       // r27

    // player state id: controller vtable +0x60, slot +0x6C, first byte
    u8 *state = (u8*)(*(u32* (*)(usize))(*(u32*)(player + 0x60) + 0x6C))(player);
    i8  slot  = (i8)*state;

    if (fn_80a966f0(platform) != 0) {                     // platform in a ride state
        u16 *flag = (u16*)(platform + 0x61C + slot * 2);
        if (*flag == 0 && *(u16*)(platform + 0x612 + slot * 2) == 0) {
            if ((u16)fn_80a96500(platform, rec_a, player) != 0) {
                *flag = mark;                             // +0x61C + 2*slot
                *(f32*)(platform + 0x5C0 + slot * 4) = *(f32*)(player + 0xAC);
                *(f32*)(platform + 0x5DC + slot * 4) = *(f32*)(platform + 0xAC);
                *(u16*)(platform + 0x62C) = 1;
                *(f32*)(player   + 0x10C) = 0.0f;         // 0x80B2ECD4
                if (fn_8005e740(player + 0xEA4) != 0)
                    *(u16*)(platform + 0x608 + slot * 2) = 1;
            }
        }
    }
}
```

The per-slot arrays are indexed by the **player's state id**: three `u16` pair
arrays at `+0x608`, `+0x612` and `+0x61C`, and two `f32` arrays at `+0x5C0`
and `+0x5DC`.

### Notes that matter for a faithful port

* `fn_80a96500` is already translated (`src/game/enemies.rs`,
  `pub unsafe fn fn_80a96500(actor, other, player)`); it returns 1 when the
  falling player was snapped onto `platform + 0x5A4`. Its `other` argument is
  `rec_a` itself, whose `+0x28` is the height-tolerance radius.
* `fn_80a966f0` is translated and answers "is the platform in one of its two
  ride states", through the `+0x394` controller.
* `fn_8005e740(player + 0xEA4)` is the translated "object is still linked"
  query; its non-zero result arms the `+0x608` slot.
* The vtable slot `+0x6C` on `*(player + 0x60)` is the same state accessor that
  `0x800A0A30` uses (`work/ghidra/main-gb/export/800a0a30_FUN_800a0a30.c`
  stores `*(int*)(param_1 + 0x4b4) = (int)*pcVar3` from it); the first byte of
  the returned object is the state id.
* There is **no `bl` caller** anywhere in the REL. The only reference is a
  data word at `0x80B2ED30` (immediately before the float pool at
  `0x80B2ED34`), so the two callback-record types are not established by this
  module.

### Constants

| address | value | use |
|---|---|---|
| `0x80B2ECD4` | `0.0f` | written to `player + 0x10C` |
| `0x80A96500` | 200 B, translated | player snap query |
| `0x80A966F0` | 164 B, translated | platform ride-state query |
| `0x8005E740` | 40 B, translated | link/flag query on `player + 0xEA4` |
| `0x80B2ED30` | data word `0x80A965D0` | only reference to this body |

### Verdict: conditional

Everything inside the body is literal and writable, but the two record types
that supply `rec_a`/`rec_b` are not. Step needed: identify the structure that
owns the `0x80B2ED30` slot (it is a data-initialised hook, not a vtable entry -
the neighbouring words are `8.0f`, `1.0f`, `-0.5f`, `16.0f`, `2.0f`, `10.0f`),
and name the `+0x2D` substate byte that suppresses the kind-2 path.

---

## 2. `0x80AC1800` - `daEnMechaKoopa_c` clip binder

| field | value |
|---|---|
| TSV size | 276 bytes |
| reachable | 69 instructions, ends at `0x80AC1910` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80ac1800_FUN_80ac1800.c` |
| signature | `void fn_80ac1800(usize this, u32 variant, u32 loop, f32 rate, f32 frame)` |
| verdict | **ready** |

`r3 = this`, `r4 = variant`, `r5 = loop`, `f1 = rate`, `f2 = frame`. Callers:
`0x80AC1D2C`, `0x80AC1E9C`, `0x80AC2E28`, `0x80AC3A28`, `0x80AC3B5C` - all
inside the `daEnMechaKoopa_c` code cluster (`g3d/mecha_koopa.brres`, state
strings at `0x80B5910E`).

### What the stream does

```c
const char *name = *(u32*)(0x80B30210 + variant * 4);  // walk/damage/sign/recover
*(u8*)(this + 0x5FC) = (u8)variant;

u32 chr = brres_get_anm_chr(this + 0x544, name);        // 0x8023A0B0
fn_801651f0(this + 0x588, this + 0x548, &chr, loop);
fn_80169f80(rate, this + 0x548, this + 0x588);
fn_801680e0(frame, this + 0x588);

u32 pat = brres_get_anm_tex_pat(this + 0x544, name);    // 0x8023A200
fn_80166bd0(this + 0x5C0, this + 0x548, &pat, 0, loop);
fn_80169f80(0.0f, this + 0x548, this + 0x5C0);
fn_80166ef0(0.0f, this + 0x5C0, 0);
fn_80166f10(frame, this + 0x5C0, 0);
```

Decoder trace (after the `_savegpr` prologue and the two `f31`/`f30` spills):

| instruction | effect |
|---|---|
| `0x80AC1810` `rlwinm r0,r4,2,0,29` | `variant * 4` |
| `0x80AC1814` `addi r6,r6,528` | `r6 = 0x80B30210` |
| `0x80AC1818` `lwzx r31,r6,r0` | `name = table[variant]` |
| `0x80AC1848` `stb r4,1532(r3)` | `this[0x5FC] = variant` |
| `0x80AC184C` `mr r4,r31` / `0x80AC1850` `addi r3,r3,1348` | `brres_get_anm_chr(this+0x544, name)` |
| `0x80AC185C` `mr r6,r30` (`r30 = r5`) | the `loop` word is forwarded to both binders |
| `0x80AC1870` `fmr f1,f30` (`f30 = f1`) | `rate` |
| `0x80AC1880` `fmr f1,f31` (`f31 = f2`) | `frame` |

### Notes that matter for a faithful port

* The two animation holders are `+0x548` (the shared NR holder) and
  `+0x588` (the chr element); the tex-pat chain uses `+0x548` and `+0x5C0`.
* `0x801651F0(this+0x588, this+0x548, &anm, loop)` - the argument order is
  destination holder, shared holder, pointer to the 4-byte animation id, loop
  flag. `0x80169F80(rate, a, b)` then sets the blend rate on the pair;
  `0x801680E0(frame, holder)` sets the frame.
* `0x80166BD0(this+0x5C0, this+0x548, &anm, 0, loop)` takes a fifth `r7`
  (`loop`) unlike the chr binder; `0x80166EF0(0.0f, holder, 0)` and
  `0x80166F10(frame, holder, 0)` finish the tex-pat chain.
* `0x80B30210` holds exactly four pointers, so `variant` is `0..3`.
* The export's `unaff_GQR0` / `ldexpf` noise is cosmetic: the only paired-single
  words in the body are the `psq_st f31,56(r1),0,0` / `psq_l f31,56(r1),0,0`
  and `f30` pairs beside `stfd`/`lfd`, i.e. `qr0` stack spills with quantiser 0.
  There is no `GQR1..GQR7` traffic, which is why `COVERAGE_WAVE152_ENEMIES.md`
  listed this row as skipped for "GQR damage".

### Constants

| address | value | use |
|---|---|---|
| `0x80B30210` | `0x80B58CC0`, `0x80B58CC8`, `0x80B58CD0`, `0x80B58CD8` | `variant -> name` table |
| `0x80B58CC0` | `"walk"` | variant 0 |
| `0x80B58CC8` | `"damage"` | variant 1 |
| `0x80B58CD0` | `"sign"` | variant 2 |
| `0x80B58CD8` | `"recover"` | variant 3 |
| `0x80B3027C` | `0.0f` | rate of the tex-pat chain |
| `0x80B58CE0` | `"g3d/mecha_koopa.brres"` | the archive the names resolve in |

### Verdict: ready

Eight direct calls, no indirect branch, no `qr` traffic, all four variant
names and the archive path are literal. The only judgement call is naming
`variant`, and the table supplies it.

---

## 3. `0x80B16A80` - `daEnUnizoo_c` eye/mouth node offset writer

| field | value |
|---|---|
| TSV size | 276 bytes |
| reachable | 69 instructions, ends at `0x80B16B90` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80b16a80_FUN_80b16a80.c` |
| signature | `void fn_80b16a80(usize this, u32 unused_r4, usize holder, u32 node_index)` |
| verdict | **ready** |

`r4` is never read. The body is installed at `0x80B6A3F4`, in the REL
function-pointer block that starts at `0x80B6A3F0`, immediately after the
main.dol vtable block ending at `0x80B6A3E4`; the class strings
`daEnUnizoo_c::StateID_*` sit at `0x80B6A522`, so the owning class is
`daEnUnizoo_c`.

### What the stream does

```c
node = mdl_node_by_index_alt(node_index);              // 0x8023B300
const char *name = *(u32*)(node + 8) ? (char*)(node + *(u32*)(node + 8)) : 0;

if (strcmp_aligned(name, "eye") == 0) {                // 0x802E1AB4, 0x80B69E6C
    f32 v[3];                                          // r1+20
    fn_802418b0(holder, v);                            // read node translation
    f32 k = 0.25f + *(f32*)(*(u32*)(this + 4) + 0x5F4);
    v[0] += k;
    v[1] += *(f32*)(*(u32*)(this + 4) + 0x5F0);
    fn_80241be0(holder, v);                            // write it back + dirty bits
}

usize actor = *(u32*)(this + 4);
if (*(u32*)(actor + 0x628) == 1) {
    const char *name2 = *(u32*)(node + 8) ? (char*)(node + *(u32*)(node + 8)) : 0;
    if (strcmp_aligned(name2, "mouth") == 0) {         // 0x80B69E70
        f32 m = *(f32*)(actor + 0x614);
        f32 v3[3] = { m, m, m };
        fn_80241a00(holder, v3);                       // write it back + dirty bits
    }
}
```

### Notes that matter for a faithful port

* The name pointer is `node + *(u32*)(node + 8)` when that word is non-zero,
  otherwise `NULL`; both `strcmp` sites recompute it from the same `r31`.
* `0x802418B0(holder, out)` is the "get translation" half of the node
  transform pair (`work/ghidra/main-gb/export/802418b0_FUN_802418b0.c`):
  it copies `holder[10/0xE/0x12]` into `out[0..2]`, unless `holder[0] & 0x40`
  is set, in which case it fills the triple from the SDA neutral value.
* `0x80241BE0(holder, in)` and `0x80241A00(holder, in)` are the matching
  "set translation + dirty bits" halves; they store the triple at
  `holder[10/0xE/0x12]` and `holder[1/2/3]` respectively and maintain the
  `0x40`/`0x18`/`0x1A` bits.
* The x offset is `0.25f + [+0x5F4]` and the y offset is `[+0x5F0]`; the two
  fields are swapped relative to the obvious "x then y" order, so a port must
  not read `+0x5F4` as y.
* The mouth path is gated on `actor + 0x628 == 1` and uses one scalar
  (`+0x614`) broadcast into all three components.

### Constants

| address | value | use |
|---|---|---|
| `0x80B322E0` | `0.25f` | x bias on the eye node |
| `0x80B69E6C` | `"eye"` | first node name |
| `0x80B69E70` | `"mouth"` | second node name |
| `0x80B6A3F4` | data word `0x80B16A80` | the body's only reference |
| `0x80B6A380` | main.dol vtable | `daEnUnizoo_c` class vtable |

### Verdict: ready

Six direct calls, two literal node names, one float constant. The `r4` slot is
unused and should be documented as such rather than invented.

---

## 4. `0x80B27DC0` - per-slot angle -> speed factor

| field | value |
|---|---|
| TSV size | 272 bytes |
| reachable | 68 instructions, ends at `0x80B27ECC` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80b27dc0_FUN_80b27dc0.c` |
| signature | `f32 fn_80b27dc0(usize this, i32 angle, f32 scale)` - returns in `f1` |
| verdict | **ready** |

`r3 = this`, `r4 = angle`, `f1 = scale`. Callers: `0x80B27FAC`, `0x80B28040`,
`0x80B28274`, `0x80B282CC`.

### What the stream does

```c
i32 a = (i32)(i16)fn_802de514(angle);      // branchless abs, kept as s16
if (a > 0x4000)
    a = (i16)(0x8000 - a);                 // 0x10000 - 0x8000 - a, folded

f32 u = (f32)(u32)a;                       // 2^52 magic at 0x80B32C70
f32 f;
if (*(u32*)(this + 0x8AC) == 1)
    f = scale * (0.4f  * u / 16384.0f) / 96.0f;     // 0x80B32C5C, 0x80B32C60, 0x80B32C24
else
    f = scale * (0.1f  * u / 16384.0f) * 0.0625f;   // 0x80B32C64, 0x80B32C60, 0x80B32C68

if (*(u16*)(this + 0x8B8) == 0)
    return 1.0f - f;                       // 0x80B32C34
return 1.0f + f;
```

The clamp arm is the `addis r3,r0,1` / `addi r0,r3,-32768` / `subf r0,r4,r0`
triple at `0x80B27E00..0x80B27E0C`: `r3 = 0x10000`, `r0 = 0x8000`, then
`r0 = 0x8000 - a`, then `extsh`.

### Notes that matter for a faithful port

* The `u32 -> f32` conversion is the standard `0x43300000 | (u ^ 0x80000000)`
  double trick minus `0x4330000080000000` (`0x80B32C70`), i.e. exactly
  `(f32)(u32)a`; no clamp or rounding is involved.
* `0x802DE514` is the translated branchless integer absolute value
  (`runtime::cpp::fn_802de514`). The result is truncated to `s16` before the
  `0x4000` compare, so an angle of `0x12345` behaves as `0x2345`.
* The `0x8AC` word is a per-slot mode: `1` selects the `0.4/96/96` curve, any
  other value the `0.1/16384/0.0625` curve. Both arms end in `1 +/- f`, so the
  return is a multiplier around `1.0`.
* `+0x8B8` is a `u16` sign selector, not a float.

### Constants

| address | value | use |
|---|---|---|
| `0x80B32C24` | `96.0f` | divisor, mode-1 arm |
| `0x80B32C34` | `1.0f` | `1 +/- f` |
| `0x80B32C5C` | `0.4f` | numerator scale, mode-1 arm |
| `0x80B32C60` | `16384.0f` | `s16` full-scale angle |
| `0x80B32C64` | `0.1f` | numerator scale, mode-2 arm |
| `0x80B32C68` | `0.0625f` | final scale, mode-2 arm |
| `0x80B32C70` | `0x4330000080000000` | `u32 -> f64` bias |
| `0x802DE514` | 8 B, translated | `abs()` |

### Verdict: ready

One call, seven pinned constants, no indirect branch, no `qr` traffic. The
mode byte `+0x8AC` and the sign halfword `+0x8B8` are read, never written, so
the body is a pure function of `(this, angle, scale)`.

---

## 5. `0x80A80F40` - `daEnIgaKuribo_c` swing/heading frame step

| field | value |
|---|---|
| TSV size | 264 bytes |
| reachable | 66 instructions, ends at `0x80A81044` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80a80f40_FUN_80a80f40.c` |
| signature | `void fn_80a80f40(usize this)` - a state handler |
| verdict | **ready** (`qr3` site fully typed) |

Installed in the `daEnIgaKuribo_c` state-method table at `0x80B4DAB4`
(`{ -1, fn, 0 }` triples from `0x80B4DA60`; the state names at `0x80B4DB74`
are `KyoroSt`, `Kyoro`, `Sleep`, `Jump`, `IgaWalk`, `IgaTurn`, `DieFumi`).

### What the stream does

```c
fn_8006cde0(this);                     // +0xEC += +0x114, clamped at +0xF8
fn_800955f0(this);
fn_80096e60(this);

if ((u16)(*(s16*)(this + 0x65A) + 0x4000) <= 0x7FFF) {   // phase in 0..0x3FFF
    s16 a = *(s16*)(this + 0x658) + 16;
    *(s16*)(this + 0x658) = a;
    if (a > 0x300) *(s16*)(this + 0x658) = 0x300;        // clamp 768
} else {
    s16 a = *(s16*)(this + 0x658) - 16;
    *(s16*)(this + 0x658) = a;
    if (a < 0xD0) *(s16*)(this + 0x658) = 0xD0;          // clamp 208
}

s16 phase = *(s16*)(this + 0x65A) + *(s16*)(this + 0x658);
*(s16*)(this + 0x65A) = phase;

s16 hw = phase;                       // sth hw,8(r1)
f32 turns = (f32)(u16)hw * 0.00390625f;   // psq_l f1,0(r1+8),3,1 ; fmuls 1/256
f32 s = fn_80237bd0(turns);                // sine table interpolation

*(s16*)(this + 0x100) = (s16)(s32)(1280.0f * s);

if (--*(s32*)(this + 0x65C) < 1)
    (*(void (**)(usize, usize))(*(u32*)(this + 0x60) + 0xD4))(this, 0x80B733B8);
```

### Notes that matter for a faithful port

* The amplitude is `+0x658` (`s16`, 208..768) and the phase is `+0x65A`
  (`s16`, wraps naturally); the sign of the phase picks the direction of the
  amplitude ramp, so the oscillator builds up on the positive half-turn and
  decays on the negative half.
* `0x80237BD0` is the translated angle reducer + `0x802F74A0` table
  interpolation (`resource::bres::fn_80237bd0`); it takes turns in
  `f1` and returns `sin` in `f1`.
* `*(s16*)(this + 0x100) = (s16)(s32)(1280.0f * s)` is the `fctiwz` +
  `stfd`/`lwz 20(r1)` pair at `0x80A81004..0x80A81010`; truncation toward zero
  is part of the behaviour.
* `+0xD4` is the `switch_state` vtable slot used throughout the translated
  enemy code; the second argument is the BSS state descriptor `0x80B733B8`,
  which is registered by the module's static initialiser and must be modelled
  as a single global, not copied by value.
* `+0x65C` is a signed countdown, decremented once per frame; the state switch
  fires on the transition into `0` (the `addic.` sets `CR0` before the store).

### The `qr3` site

`0x80A80FD8 psq_l f1,0(r4),3,1` with `r4 = r1 + 8` and the halfword stored at
`0x80A80FD4 sth r0,8(r1)`. `GQR3 = 0x00050005` (LD type 5 = unsigned 16-bit,
scale 0), `W = 1` selects the single-element form, so the instruction is
exactly `f1 = (f32)(u16)hw`. The following `fmuls f1,f0,f1` with
`f0 = 0.00390625f` converts to turns. The Ghidra export's `unaff_GQR3` and
`ldexpf` block is cosmetic.

### Constants

| address | value | use |
|---|---|---|
| `0x80B2E320` | `0.00390625f` | halfword angle -> turns |
| `0x80B2E324` | `1280.0f` | sine amplitude -> `s16` angle |
| `0x80B733B8` | BSS state object | argument of the `+0xD4` state switch |
| `0x80B4DAB4` | data word `0x80A80F40` | state-table entry |
| - | `0x300` / `0xD0` / `16` | amplitude clamp and step |

### Verdict: ready

The only dispatch is the standard `switch_state` slot `+0xD4`, and the
`qr3` load is resolved by `GQR3 = 0x00050005`. `0x800955F0` and `0x80096E60`
are translated stage helpers; `0x8006CDE0` is untranslated but is a 36-byte
`[+0xEC] += [+0x114]` clamp against `[+0xF8]`, quoted in the pseudo-code above.

---

## 6. `0x80B1AD30` - `daEnWalkPakkun_c` ground snap + ride tail

| field | value |
|---|---|
| TSV size | 264 bytes |
| reachable | 66 instructions, ends at `0x80B1AE34` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80b1ad30_FUN_80b1ad30.c` |
| signature | `void fn_80b1ad30(usize this, u32 a1, u32 a2)` |
| verdict | **ready** |

Installed at `0x80B6AF5C` in the `daEnWalkPakkun_c` vtable (state strings at
`0x80B6B1E6`).

### What the stream does

```c
*(u32*)(this + 0x5E4) = fn_80b1ae40(this);      // animation-phase predicate
*(u32*)(this + 0x4FC) |= 0x10000;               // oris r4,r4,1

if ((*(u32*)(this + 4) >> 16) & 1) {            // bit 0x10000 of the +4 word
    f32 out = 0.0f;                             // r1+8
    if (fn_80075270(*(f32*)(this + 0xAC),
                    4.0f + *(f32*)(this + 0xB0),
                    *(u8*)(this + 0x38F), &out) != 0)
    {
        u32 snap = 0;
        if (4.0f + *(f32*)(this + 0xB0) < out) {
            f32 rec[3];                          // r1+12, rec.y = r1+16
            fn_8006ced0(rec, this);              // rec = pos + (+0xD0..+0xD8)
            if (out <= 8.0f + rec[1])
                snap = 1;
        }
        if (snap) {
            *(u32*)(this + 0x4FC) &= ~0x10000u;  // rlwinm 0,16,14
            *(f32*)(this + 0xB0) = out - *(f32*)(this + 0xD4);
        }
    }
}

fn_800a0a30(this, a1, a2);                      // ride/attach completion
```

### Notes that matter for a faithful port

* `fn_80b1ae40` is translated (`0x80B1AE40`, 308 B). It returns 1 when the
  `+0x394` component matches one of three BSS-registered animation names
  (`0x80B791F8`, `0x80B791B8`, `0x80B79238`) with a `+0x584` frame threshold
  (`0x80B324FC`, `0x80B32500`); the raw `1`/`0` is what `+0x5E4` stores.
* `fn_80075270` is the translated ground probe
  `(x, y, ground_kind, &out)`; `out` is seeded with `0.0f` before the call.
* `fn_8006CED0(&rec, this)` is a two-word position read: `rec[0..2] =
  this[0xAC..0xB4] + this[0xD0..0xD8]`
  (`work/ghidra/main-gb/export/8006ced0_FUN_8006ced0.c`). Only `rec[1]` (y) is
  consumed.
* `fn_800a0a30(this, a1, a2)` is the **untaken** main.dol ride completion
  (`work/ghidra/main-gb/export/800a0a30_FUN_800a0a30.c`, 252 B): it sets
  `this + 0x480` from the `+0xE8` speed against an SDA threshold, walks the
  four player slots for the one mounted on `this`, calls `0x8012E510(player,
  this)`, copies the rider's state id into `this + 0x4B4`, and calls slot
  `+0x30` of `this + 0x394` with `0x80358004`. It must be ported with this
  body, but it has no unresolved behaviour.
* The `+0x4FC` bit `0x10000` is set unconditionally at entry and cleared only
  when the snap succeeds; it is not the same flag as `this + 4`'s bit 15,
  which is only *tested*.

### Constants

| address | value | use |
|---|---|---|
| `0x80B324B4` | `0.0f` | `out` seed |
| `0x80B324F4` | `4.0f` | probe height above `+0xB0`, and the "below ground" guard |
| `0x80B324F8` | `8.0f` | extra tolerance when matching `rec.y` |
| `0x80B6AF5C` | data word `0x80B1AD30` | vtable slot |
| - | `0x10000` | `+0x4FC` gate bit |

### Verdict: ready

`0x80075270` and the animation predicate `0x80B1AE40` are translated;
`0x8006CED0` is a four-load position read and `0x800A0A30` is read out in full
above, so both remaining untranslated callees are fully specified. No indirect
branch, no `qr` traffic, no unmapped data.

---

## 7. `0x80AF5C60` - `daEnRotBlock_c` guide-angle placement

| field | value |
|---|---|
| TSV size | 260 bytes |
| reachable | 65 instructions, ends at `0x80AF5D60` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80af5c60_FUN_80af5c60.c` |
| signature | `void fn_80af5c60(usize this)` - a state handler |
| verdict | **ready** (two `qr3` sites fully typed) |

Installed at `0x80B63C40` in the `daEnRotBlock_c` state-method table (names
`StateID_Search` / `StateID_Move` at `0x80B63C50`).

### What the stream does

```c
usize guide = fn_80162d00(*(u32*)(this + 0x640));
f32 gx = *(f32*)(guide + 0xAC);
f32 gy = *(f32*)(guide + 0xB0);

s16 ang = *(s16*)(guide + 0x104) + 0x4000;
*(s16*)(this + 0x104) = ang;

s16 hw = ang;                             // sth hw,10(r1)
f32 turns = (f32)(u16)hw * 0.00390625f;   // psq_l f1,0(r1+10),3,1 ; fmuls 1/256
f32 c = fn_80237c40(turns);               // cosine, 0x802F74A4 table

/* the same halfword is stored a second time at 8(r1) and reloaded */
f32 s = fn_80237bd0(turns);               // sine,   0x802F74A0 table

*(f32*)(this + 0xB4) -= *(f32*)(this + 0xD8);
*(f32*)(this + 0xAC) = (gx + *(f32*)(this + 0x644) * c
                           - *(f32*)(this + 0x648) * s) - *(f32*)(this + 0xD0);
*(f32*)(this + 0xB0) = (gy + *(f32*)(this + 0x644) * s
                           + *(f32*)(this + 0x648) * c) - *(f32*)(this + 0xD4);
```

### Notes that matter for a faithful port

* `0x80162D00(id)` is the translated "resolve object by id" (`id & 7` selects
  a `0x20`-byte hash bucket); `this + 0x640` holds the guide id.
* The guide's angle at `+0x104` is **read**, then `+0x4000` (a quarter turn) is
  written into `this + 0x104`, not back into the guide.
* Both trig calls consume the same pre-wrapped halfword: the second `psq_l`
  re-loads the identical stack word, so `c` and `s` differ only by the table
  base (`0x802F74A4` vs `0x802F74A0`).
* The layout is a pivot: the guide position plus a radius
  `(+0x644`, `+0x648`) rotated by the angle, minus the `(+0xD0,+0xD4,+0xD8)`
  offset triple. `z` only receives `- +0xD8`.
* `+0x644`/`+0x648` are the two radius components, `+0xD0`/`+0xD4`/`+0xD8` the
  offset triple shared with the other enemies.

### The `qr3` sites

`0x80AF5CB8 psq_l f1,0(r4),3,1` with `r4 = r1 + 10` (after `sth r30,10(r1)` at
`0x80AF5CAC`) and `0x80AF5CD8 psq_l f1,0(r3),3,1` with `r3 = r1 + 8` (after
`sth r30,8(r1)` at `0x80AF5CCC`). With `GQR3 = 0x00050005` and `W = 1` both are
`f1 = (f32)(u16)angle`, then `* 0.00390625f` -> turns.

### Constants

| address | value | use |
|---|---|---|
| `0x80B31460` | `0.00390625f` | halfword angle -> turns |
| `0x80237C40` | translated | cosine table interpolation (`0x802F74A4`) |
| `0x80237BD0` | translated | sine table interpolation (`0x802F74A0`) |
| `0x80B63C40` | data word `0x80AF5C60` | state-table entry |

### Verdict: ready

Three translated callees, two `qr3` loads resolved by `GQR3 = 0x00050005`, no
indirect branch, no unmapped data.

---

## 8. `0x80A5E590` - `daEnGabon_c` state entry / initialiser

| field | value |
|---|---|
| TSV size | 252 bytes |
| reachable | 63 instructions, ends at `0x80A5E688` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80a5e590_FUN_80a5e590.c` |
| signature | `void fn_80a5e590(usize this)` - a state handler |
| verdict | **conditional** |

Installed at `0x80B47BAC` in the `daEnGabon_c` state-method table
(`StateID_Appear` .. `StateID_DieFall` at `0x80B47C5B`).

### What the stream does

```c
f32 v = 3.0f;                                   // 0x80B2D1C8

usize comp = (*(usize* (*)(usize))(*(u32*)(this + 0x394) + 0x2C))(this + 0x394);
if ((*(u32* (*)(usize, usize))(*(u32*)comp + 0x14))(comp, 0x80358004) != 0)
    v = 6.0f;                                   // 0x80B2D1F4

fn_80a5e1a0(v, this, "wait", 0);                // 0x80B47890
fn_80a5e230(this, "wait", 1);
fn_80166f10(0.0f, this + 0x5C0, 0);

*(f32*)(this + 0xE8) = 0.0f;                    // 0x80B2D19C
*(f32*)(this + 0xEC) = 0.0f;
*(f32*)(this + 0xF0) = 0.0f;

*(s32*)(this + 0x638) = fn_800b2ee0(60) + 12;
*(s32*)(this + 0x5FC) = fn_800b2ee0(60) + 90;
*(u32*)(this + 0x360) = 1;
```

### Notes that matter for a faithful port

* The two indirect calls are a two-level component query, not a `switch_state`:
  slot `+0x2C` of the `+0x394` controller returns a component whose own slot
  `+0x14` performs a name/id match against `0x80358004` (BSS, registered at
  module init). A non-zero result selects the `6.0f` walk speed.
* `fn_80a5e1a0(f32 rate, usize this, const char *name, u32 loop)` is the
  **untaken sibling at `0x80A5E1A0`** (136 B). Its body is: bind
  `brres_get_anm_chr(this + 0x540, name)`, `0x801651F0(this+0x584,
  this+0x548, &anm, loop)`, `0x80169F80(rate, this+0x548, this+0x584)`,
  `0x801680E0(0.0f /* 0x80B2D198 */, this+0x584)`. It must be ported first or
  stubbed with the same signature.
* `fn_80a5e230(this, name, 1)` is the translated tex-pat binder (`8023A200`
  plus `+0x5BC` store plus `0x80166BD0`).
* `fn_800b2ee0(60)` is the translated LCG wrapper (duplicate of `0x800B2ED0`; it
  forwards to `0x80161460`, the `1664525/1013904223` LCG plus `mulhi` scale), so
  both timers are independent draws under the same bound `60`:
  `+0x638 = rand(60) + 12`, `+0x5FC = rand(60) + 90`.
* `+0x360 = 1` is a plain word store (`li r0,1`), not a halfword.

### Constants

| address | value | use |
|---|---|---|
| `0x80B2D1C8` | `3.0f` | default walk speed |
| `0x80B2D1F4` | `6.0f` | speed when the component query hits |
| `0x80B2D19C` | `0.0f` | velocity triple and tex-pat rate |
| `0x80B2D198` | `0.0f` | frame constant inside `0x80A5E1A0` |
| `0x80B47890` | `"wait"` | animation name for both binders |
| `0x80358004` | BSS id | name/id argument of the `+0x14` match |
| `0x80B47BAC` | data word `0x80A5E590` | state-table entry |

### Verdict: conditional

The body is writable, but two things must be established first: (a) the
`+0x394` component returned by slot `+0x2C` - it is not the state object that
slot `+0x28` returns, so the existing `controller_state()` helper does not
apply; (b) the BSS-registered id `0x80358004`. Both determine *which* speed the
actor gets. Everything else (the four call shapes and the three timers) is
literal.

---

## 9. `0x80A7ACA0` - two-table ground probe

| field | value |
|---|---|
| TSV size | 252 bytes |
| reachable | 63 instructions, ends at `0x80A7AD98` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80a7aca0_FUN_80a7aca0.c` |
| signature | `u32 fn_80a7aca0(usize this)` - returns `1` on a hit, `0` on a flip |
| verdict | **ready** |

Called from `0x80A7BCA8`.

### What the stream does

```c
u8 row = *(u8*)(this + 0xEA4);
u8 col = *(u8*)(this + 0x348);

f32 y = *(f32*)(this + 0xB0) + ((const f32*)0x80B2E0D8)[row * 2 + col];
i32 r = fn_80a7ab30(y);                     // -1 / 0 / 1 / >=2

if ((u32)(r + 1) <= 2) {                    // r in {-1, 0, 1}
    u8 v = ((const u8*)0x80B2E100)[row * 2 + col];
    *(f32*)(this + 0xB0) = y;
    *(u8*)(this + 0xEA2) = v;
    *(u8*)(this + 0xEA3) = (r == 1) ? 1 : (r == -1) ? 2 : 0;
    return 1;
} else {
    *(u8*)(this + 0x348) = (col == 0) ? 1 : 0;
    return 0;
}
```

### Notes that matter for a faithful port

* The float table is indexed `row * 8 + col * 4` bytes with `row = +0xEA4`
  and `col = +0x348`; the byte table is indexed `row * 2 + col`. Both are
  five-to-six entry tables - see below for the exact contents.
* `fn_80a7ab30(y)` is translated (`0x80A7AB30`, "`0x80A7A860` returns f32
  offset; pair `{offset}`"): it answers `-1`, `0`, `1` for the three
  hit bands and `>= 2` for "no ground".
* The `(u32)(r + 1) <= 2` test at `0x80A7ACF0..0x80A7ACF8` is the same as
  `r == -1 || r == 0 || r == 1`; the unsigned form matters if `r` can be
  negative.
* `addi r0,r0,1` / `addi r0,r0,2` / `addi r0,r0,0` in the hit arm are
  `li r0,1` / `li r0,2` / `li r0,0`, **not** `col + 1`. The Ghidra export agrees
  (`+0xEA3 = 1`, `= 2`, `= 0`).
* `+0xEA2` is the height-band byte, `+0xEA3` the band direction, `+0xEA4` the
  row selector and `+0x348` the facing byte that flips on a miss.

### Tables

`0x80B2E0D8` - five rows of two floats (`row * 2 + col`):

| row | col 0 | col 1 |
|---:|---:|---:|
| 0 | `0.0f` | `0.0f` |
| 1 | `0.0f` | `16.0f` |
| 2 | `0.0f` | `-16.0f` |
| 3 | `16.0f` | `0.0f` |
| 4 | `-16.0f` | `0.0f` |

`0x80B2E100` - six rows of two bytes (`row * 2 + col`):
`00 00 01 02 | 02 01 02 01 | 01 02 00 00` ->

| row | col 0 | col 1 |
|---:|---:|---:|
| 0 | 0 | 0 |
| 1 | 1 | 2 |
| 2 | 2 | 1 |
| 3 | 2 | 1 |
| 4 | 1 | 2 |
| 5 | 0 | 0 |

### Constants

| address | value | use |
|---|---|---|
| `0x80B2E0D8` | float table above | per-(row,col) y offset |
| `0x80B2E100` | byte table above | per-(row,col) band byte |
| `0x80A7AB30` | translated | ground band query |

### Verdict: ready

One translated callee, two data tables read out in full, no indirect branch,
no `qr` traffic.

---

## 10. `0x80AE0780` - per-player effect burst emitter

| field | value |
|---|---|
| TSV size | 248 bytes |
| reachable | 62 instructions, ends at `0x80AE0874` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80ae0780_FUN_80ae0780.c` |
| signature | `void fn_80ae0780(void)` - no arguments |
| verdict | **ready** |

Called from `0x80AE099C`; described in `src/game/enemies.rs` as the body
actor's per-player "sweat" burst (`W89_FN_80AE0780`).

### What the stream does

```c
for (u32 i = 0; i < 4; i++) {
    usize p = fn_8005f900(i);                       // player by slot
    if (p == 0) continue;
    if ((*(u8*)0x80429CA8 & (1u << i)) == 0) continue;
    if (fn_80056cf0(p, 0x53) != 0) continue;        // "cannot receive" flag

    usize mgr = *(u32*)0x80429E00;                  // effect manager
    f32 z = *(f32*)(p + 0xB4);
    f32 rec[6] = {
        *(f32*)(p + 0xAC),                          // r1+8
        2.0f + *(f32*)(mgr + 0x3C),                 // r1+12
        0.0f,                                       // r1+16
        *(f32*)(p + 0xAC),                          // r1+20
        *(f32*)(p + 0xB0),                          // r1+24
        z                                           // r1+28
    };
    fn_80064610(0x3C, 0x87, (u32)rec, 0, 0);
}
```

### Notes that matter for a faithful port

* The six-float stack record is contiguous (`r1+8 .. r1+28`) and is passed as
  one pointer in `r5`; the first triple is `(x, 2 + mgr[0x3C], 0)` and the
  second is the player position `(x, y, z)`. Do **not** read `mgr[0x3C]` as
  `p.z`: `f3` is loaded from the manager before `r4` is overwritten.
* `0x80064610(id, arg, pos, 0, 0)` is an untranslated 104-byte `main.dol`
  request that wraps `0x80064600(r7)` + `0x8006CA80(id, arg, &pos, 0)`; the id
  is `0x3C` (60) and the resource word `0x87` (135). This matches the documented
  `(id, resource, out, 0, 0)` spawn shape used in `src/game/bosses.rs`.
* `0x80429CA8` is a per-slot player mask byte, `0x80429E00` the effect-manager
  pointer; both are `main.dol` BSS.
* `fn_80056CF0(p, 0x53)` is the translated flag test over
  `p[0x1014 + 4*(flag>>5)]`; a non-zero result skips the slot.

### Constants

| address | value | use |
|---|---|---|
| `0x80B30CC0` | `0.0f` | third component of the first triple |
| `0x80B30D04` | `2.0f` | added to `mgr[0x3C]` |
| `0x80429CA8` | pointer/byte | per-slot player mask |
| `0x80429E00` | pointer | effect manager; `+0x3C` read as `f32` |
| - | `0x3C`, `0x87`, `0x53` | effect id, resource, player flag |

### Verdict: ready

Three translated calls (`0x8005F900`, `0x80056CF0` and the `0x802DCD74`
prologue helper) plus the untranslated `0x80064610` effect request, whose
5-argument shape is pinned above. Two `main.dol` globals, two float constants
and the exact six-float record. No indirect branch and no `qr` traffic.

---

## 11. `0x80AB12E0` - per-section texcoord/colour override

| field | value |
|---|---|
| TSV size | 244 bytes |
| reachable | 61 instructions, ends at `0x80AB13D0` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80ab12e0_FUN_80ab12e0.c` |
| signature | `void fn_80ab12e0(usize this, usize tex_res, usize color_res, u8 mode)` |
| verdict | **ready** |

Called from `0x80AB12C4`; referenced in `src/game/enemies.rs` as the
variant/tail helper of the two-matrix builder (`W131_FN_80AB12E0`).

### What the stream does

```c
u32 list[1];
list[0] = fn_80168400(this + 0x684);            // section dictionary

for (u32 i = 0; i < fn_8023b8c0(list); i++) {
    usize section = fn_80168480(this + 0x684, i);
    u32   off     = *(u32*)(section + 0x3C);
    usize attr    = (off ? section + off : 0) + 0x20;

    u32 tex;
    res_texcoord_get(&attr, tex_res, &tex);     // 0x8023D300
    u8 *bytes = (u8*)&tex;
    bytes[3]  = mode;                           // stb r30,19(r1)
    u32 t2 = tex;
    fn_801685e0(this + 0x684, i, tex_res, &t2, 0);

    u32 col;
    res_color_get(&attr, color_res, &col);      // 0x8023D460
    u32 c2 = col;
    fn_80168880(this + 0x684, i, color_res, &c2, 0);
}
```

### Notes that matter for a faithful port

* `fn_80168400` is the translated "cast to element type, read `+0xE8`"
  accessor; `fn_8023B8C0(&list)` is the translated section counter
  (model `+0x30` dictionary, count at `dict+4`).
* `fn_80168480(this+0x684, i)` returns the section; `+0x3C` is a self-relative
  payload offset, and `+0x20` is added after the relocation. A zero offset
  collapses the attribute pointer to `0x20` (the original has no null guard),
  which the Rust port should reproduce literally.
* `0x8023D300` packs two 12-bit pairs into the returned word;
  `0x8023D460` packs fields `+0x42..+0x49` of a `0x10`-byte record.
* The `mode` byte overwrites the **least significant byte** of the texcoord
  word between the getter and the submit (big-endian: `stw r0,16(r1)` at
  `0x80AB1364`, `stb r30,19(r1)` at `0x80AB1354`, `lwz r0,16(r1)`,
  `stw r0,12(r1)`), so the getter's low byte is intentionally discarded and the
  caller-supplied `mode` takes its place.
* Both submits take the section index, the resource word and a pointer to the
  patched value with a trailing `0`.

### Constants

| address | value | use |
|---|---|---|
| `0x80AB12E0` | - | no float or pointer constants in the body |
| - | `+0x684` | section dictionary on `this` |
| - | `+0x3C`, `+0x20` | section payload offset / attribute bias |

### Verdict: ready

Nine direct calls, all translated, no indirect branch, no data tables. The
only subtlety is the byte patch between the getter and the submit, which is
explicit above.

---

## 12. `0x80AC0750` - launch-angle solver

| field | value |
|---|---|
| TSV size | 240 bytes |
| reachable | 60 instructions, ends at `0x80AC083C` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80ac0750_FUN_80ac0750.c` |
| signature | `void fn_80ac0750(usize this, usize target)` |
| verdict | **ready** |

Called from `0x80AC0DE8`. Sibling of the wave-151 body `0x80AC0840`
(`lift_maruta` per-slot collision response).

### What the stream does

```c
usize holder = fn_80162d00(*(u32*)(this + 4));
*(u32*)(this + 0) = *(u32*)(target + 0);        // copy the target's first word

f32 dy = *(f32*)(target + 0xB0) - *(f32*)(holder + 0xB0);
f32 r2 = 32.0f * 32.0f;                         // 0x80B3019C squared
f32 dx2 = r2 - dy * dy;
if (dx2 <= 0.0f) dx2 = 0.0f;                    // 0x80B30190

f32 dx;
if (dx2 > 0.0f)
    dx = dx2 * fn_80237ba0(dx2);                // Newton reciprocal sqrt -> |dx|
else
    dx = 0.0f;

s16 ang = (s16)fn_801611c0(dx, dy);             // atan2, 1/65536 turn
ang = (s16)(-ang);
if (ang > 0) ang = 0;                           // clamp to the negative half
*(s16*)(this + 8) = (s16)(ang + *(s16*)(holder + 0x100));
```

### Notes that matter for a faithful port

* `0x80237BA0` is the translated **Newton reciprocal square root**
  (`resource::bres::fn_80237ba0`, "hardware fsqrt modelled by sqrt64"), so
  `dx2 * rsqrt(dx2) = sqrt(dx2) = |dx|`. Do not replace it with a plain
  `sqrt`: the double multiply is what clamps the horizontal reach to a
  `32`-unit sphere.
* `0x801611C0` is the translated eight-octant angle table. **Argument order:**
  this body passes `(|dx|, dy)` - horizontal leg first, vertical leg second -
  which is the *opposite* order from the wave-151 trail emitter (`0x80A10880`,
  which passes `(dy, dx)`). The result is negated and clamped to the negative
  half, so the angle is the launch elevation below the guide's own `+0x100`
  heading.
* `-ang` is formed by `neg` and truncated with `extsh` before the `ble`
  clamp, so the result is `min(-ang, 0)` as an `s16`.
* `this + 0` receives the target's first word before anything else; `this + 8`
  is the `s16` output angle, biased by the holder's `+0x100` angle.
* `fn_80162D00(this + 4)` resolves the holder from the id stored at `+4`.

### Constants

| address | value | use |
|---|---|---|
| `0x80B3019C` | `32.0f` | reach radius; squared to `1024` |
| `0x80B30190` | `0.0f` | clamp floor / comparison zero |
| `0x80237BA0` | translated | Newton reciprocal sqrt |
| `0x801611C0` | translated | eight-octant angle table |
| `0x80162D00` | translated | object-by-id lookup |

### Verdict: ready

Three translated callees, two float constants, no indirect branch, no `qr`
traffic. The reciprocal-square-root idiom is the only trap and is spelled out
above.

---

## Summary of the twelve

| # | address | bytes | signature | verdict |
|---:|---|---:|---|---|
| 1 | `0x80A965D0` | 280 | `fn_80a965d0(rec_a, rec_b)` | conditional |
| 2 | `0x80AC1800` | 276 | `fn_80ac1800(this, variant, loop, rate, frame)` | ready |
| 3 | `0x80B16A80` | 276 | `fn_80b16a80(this, _r4, holder, node_index)` | ready |
| 4 | `0x80B27DC0` | 272 | `fn_80b27dc0(this, angle, scale) -> f32` | ready |
| 5 | `0x80A80F40` | 264 | `fn_80a80f40(this)` | ready |
| 6 | `0x80B1AD30` | 264 | `fn_80b1ad30(this, a1, a2)` | ready |
| 7 | `0x80AF5C60` | 260 | `fn_80af5c60(this)` | ready |
| 8 | `0x80A5E590` | 252 | `fn_80a5e590(this)` | conditional |
| 9 | `0x80A7ACA0` | 252 | `fn_80a7aca0(this) -> u32` | ready |
| 10 | `0x80AE0780` | 248 | `fn_80ae0780()` | ready |
| 11 | `0x80AB12E0` | 244 | `fn_80ab12e0(this, tex_res, color_res, mode)` | ready |
| 12 | `0x80AC0750` | 240 | `fn_80ac0750(this, target)` | ready |

Two `qr3` bodies (5, 7), one untranslated sibling that must ship with its
caller (`0x80A5E1A0` for body 8), one untranslated `main.dol` tail that must
ship with body 6 (`0x800A0A30`), and one body whose caller contract is not in
this module (body 1).
