# Wave 153 - implementation specification for 12 untranslated `d_enemiesNP` bodies

## What this page is

`docs/ANALYSIS_WAVE127_ENEMIES.md` reclassified 384 formerly-skipped
`d_enemiesNP` addresses and waves 128-152 worked through that block. This page
takes the twelve largest bodies that are **still untranslated**, carry a
`decompiled = true` Ghidra export, and walk cleanly from their entry.

A live scan (2026-11-13 08:10, after the wave-152 edits landed) of
`work/ghidra/rel-d_enemiesNP/export/functions.tsv` (3,555 rows, all
`decompiled = true`), the `address` column of `ledger/functions.csv` (17,102
distinct addresses), every `0x80Axxxxx` / `0x80Bxxxxx` address in a `^#`
heading of any `docs/*.md` (388), and every `fn fn_<addr>` **body** reachable
under `src/` (15,938) leaves **40** bodies that are absent from all four sets
and therefore unclaimed.

`fn fn_<addr>` bodies are matched only as definitions
(`^\s*(pub )?(unsafe )?(extern "C" )?fn fn_<addr>`); a `const FN_80Axxxxx:
usize = 0x...` address stub or a doc-comment mention is **not** a body. The
other lane added `fn_80ab0870` while this page was being prepared (8:06), so it
is excluded here even though it was rank 8 in the first pass; `0x80ACE690`
takes the twelfth slot.

This page is **read-only**: it writes no Rust, no ledger row and no other doc.

## Selection

The ranking metric is the **reachable body**, not the raw `functions.tsv`
extent, because the TSV size is a bucket extent that can include a neighbour's
padding or an unreachable arm (see `docs/REL_ELF_MAPPING.md`). Each candidate
was walked from its entry through every conditional branch, every internal `b`
and every linked indirect branch (`bctrl`), until all paths ended in `bclr`,
`blr`, a tail `b` or a tail `bcctr`. Conditional `bclr`/`bcctr` forms were
followed on both the taken and the fall-through edge.

| # | address | TSV size | reachable insns | `qr3` sites | role | verdict |
|---:|---|---:|---:|---:|---|---|
| 1 | `0x80A4B140` | 236 | 59 | 0 | `daEnCrow_c` target scan: nearest live player under a `1.0e9` gate | **conditional** |
| 2 | `0x80A637E0` | 236 | 59 | 0 | `daEnGesso_c` near-player state select (96/144 x band) | **ready** |
| 3 | `0x80AD45E0` | 224 | 56 | 0 | carrier/rail offset commit from the `+0x6BC` holder | **ready** |
| 4 | `0x80A80E50` | 220 | 55 | 1 | `daEnIgaKuribo_c` `Kyoro` state entry: clips, angle, timer | **ready** |
| 5 | `0x80A06220` | 212 | 53 | 0 | fire-cannon slot spawn from a `0x14`-byte record | **ready** |
| 6 | `0x80A5DBC0` | 212 | 53 | 1 | `daEnGabon_c` angle step: `0x10000 / speed` -> `i8` -> `sin` -> `s16` | **conditional** |
| 7 | `0x80AD46C0` | 212 | 53 | 0 | carrier/rail frame step with the height wrap | **ready** |
| 8 | `0x80AF3570` | 196 | 49 | 2 | cannon guide world position from a `u16` angle | **ready** |
| 9 | `0x80A2EB40` | 188 | 47 | 0 | `daEnBubble_c` water probe at `y + dy` | **conditional** |
| 10 | `0x80B05A80` | 180 | 45 | 0 | `daEnSnakeBlock_c` clip binder over `N` segments | **ready** |
| 11 | `0x80B0DE10` | 180 | 45 | 0 | `daEnTeresa_c` state step (turn-order gate) | **ready** |
| 12 | `0x80ACE690` | 172 | 43 | 0 | net-Noko clip blend commit | **ready** |

**Split: 9 ready, 3 conditional.** Total 2,468 bytes of TSV extent and 617
reachable instructions. All twelve walk to a terminal `bclr` at exactly
`entry + size - 4`, so `reachable == extent` for every row: no truncated arm,
no out-of-range branch and no `bctr` jump table.

### Rows below the cut

| address | TSV size | why it is not claimed |
|---|---:|---|
| `0x80B05B80` | 172 | same size as body 12; tie broken by address; sibling of the snake-block constructor |
| `0x80ACDB00` | 148 | next in size order |
| `0x80A4B0B0` | 136 | crow animation binder; already called by address from the crow block |
| `0x80AF42C0` | 132 | cannon-sweep tail helper |
| `0x80A05DF0` | 116 | fire-cannon cluster |
| `0x80A03C30` | 92 | unclassified early block |

## The `qr3` pool at this wave

`docs/REL_ELF_MAPPING.md` pins **`GQR3 = 0x00050005`** (LD/ST type 5 = unsigned
16-bit, both scales 0), installed by `main.dol` at `0x8025EEEC`/`0x8025EEF0`.
The RAW stream of this REL contains no `mtspr GQRn`. Four `qr3` sites appear in
three of the twelve bodies; every one is a single-element (`W = 1`) load of a
stack halfword that the body itself stored one or two instructions earlier:

| address | size | `qr3` sites | where |
|---|---|---:|---|
| `0x80A80E50` | 220 | 1 | `psq_l f1,0(r4),3,1` at `0x80A80ECC`, `r4 = r1 + 8` |
| `0x80A5DBC0` | 212 | 1 | `psq_l f1,0(r3),3,1` at `0x80A5DC38`, `r3 = r1 + 8` |
| `0x80AF3570` | 196 | 2 | `psq_l f1,0(r5),3,1` at `0x80AF35A8` (`r5 = r1 + 10`) and `psq_l f1,0(r3),3,1` at `0x80AF35D4` (`r3 = r1 + 8`) |

In every case the sequence is `sth rX,off(r1)` -> `psq_l f1,off(r1),3,1` ->
`fmuls f1,f0,f1` with `f0 = 0.00390625 = 1/256`
(`0x80B2E320` in body 4, `0x80B2D1BC` in body 6, `0x80B313D4` in body 8).
With `GQR3 = 0x00050005` the load returns `f1 = (f32)(u16)halfword` with no
scale, so the whole pair reduces to `turns = (f32)(u16)angle / 256.0f`. The
Ghidra `unaff_GQR3` / `ldexpf` blocks are cosmetic.

Every other `psq_l`/`psq_st` word in the twelve is a `qr0` `f31`/`f30` stack
spill beside a matching `stfd`/`lfd`.

## Reading conventions

* Synthetic base is `0x80A00000`. The REL has five file-backed `PT_LOAD`
  segments (`0x80A000F0`, `0x80B2A518`, `0x80B2A7A4`, `0x80B2A7B0`,
  `0x80B32DC0`); `docs/REL_ELF_MAPPING.md`'s `+0x1000` formula applies to the
  first. The file-backed image ends at `0x80B6E438`; `0x80B6E500..0x80B799E0`
  is the zero-filled BSS (`bssSize = 0xB4E0` in
  `work/rels/d_enemiesNP.info.json` = 46,304 bytes). Every `0x80B7xxxx`
  address below is a BSS-resident state object, not file-backed data.
* `this` is the incoming `r3` unless stated otherwise; `+0xNNN` are byte
  offsets from `this`. `+0xAC`/`+0xB0`/`+0xB4` are x/y/z, `+0xD0`/`+0xD4`/`+0xD8`
  the position offset triple, `+0xE8`/`+0xEC`/`+0xF0` the velocity triple,
  `+0x100`/`+0x102`/`+0x104` the three `s16` angles, `+0x348` the side/facing
  byte, `+0x38C` the actor kind byte and `+0x38F` the ground kind byte.
* **`r0` in an `addi`/`addis`.** An `addi rX,r0,imm` is `li rX,imm`, and
  `addis rX,r0,imm` is `lis rX,imm`, not a read of `r0`. The listings quote the
  register form; every pseudo-code block below resolves it to the literal.
  `addi rX,r0,imm` where `rX == r0` (e.g. `0x3800003C` = `addi r0,r0,60`) is a
  real read-modify-write of `r0` and is called out where it matters.
* **CodeWarrior prologues.** `0x802DCD70`/`0x802DCD74` are `_savegprlr_26`/
  `_savegprlr_27` and `0x802DCDBC`/`0x802DCDC0` their `_restgprlr_*` partners;
  they are not argument-destroying stubs and their incoming `r3..r10` survive.
  Ghidra's 64-bit return from them is the saved register pair, and
  `iVar3 = func_0x802dcd70()` in the exports is always the caller's own `r3`.
* **`0x80168080` returns an `f32` and `0x80169F80` takes one.** `0x80168080`
  is a tail-call thunk into a `+0x20` slot, so the slot's float result reaches
  its caller in `f1`. `0x80169F80` saves its incoming `f1` and forwards it to
  `0x80169C80`. The repo wrappers type both as `()` - the specifications below
  say where that matters.
* Sizes are the `size` column of `functions.tsv`; "reachable" is the count of
  distinct instruction words on a path from the entry.

---

## 1. `0x80A4B140` - `daEnCrow_c` nearest-player scan

| field | value |
|---|---|
| TSV size | 236 bytes |
| reachable | 59 instructions, ends at `0x80A4B228` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80a4b140_FUN_80a4b140.c` |
| signature | `pub unsafe fn fn_80a4b140(this: usize) -> usize` |
| verdict | **conditional** |

### What the stream does

```c
// r26 = incoming r3 = this (the crow actor); r29 = best, r27 = candidate.
usize fn_80a4b140(usize this)
{
    usize best    = 0;                          // r29
    f32   best_d2 = *(f32*)0x80B2CA34;          // 1.0e9f
    u8    mask    = *(u8*)0x80429CA8;           // per-player slot mask

    for (u32 i = 0; i < 4; i++) {
        usize p = fn_8005f900(i);               // player table, translated
        if (p == 0) continue;
        u8 mask = *(u8*)0x80429CA8;             // re-read every iteration
        if ((mask & (1u << i)) == 0) continue;

        // controller vtable +0xE0, called with the player as receiver
        u32 ctrl = *(u32*)(p + 0x60);
        if (call1(*(u32*)(ctrl + 0xE0), p) != 0) continue;

        f32 dx = (*(f32*)(p + 0xAC) + *(f32*)(p + 0xD0)) - *(f32*)(this + 0xAC);
        f32 dz = (*(f32*)(p + 0xB0) + *(f32*)(p + 0xD4)) - *(f32*)(this + 0xB0);
        f32 d2 = dx * dx + dz * dz;

        if (d2 < best_d2) { best_d2 = d2; best = p; }   // fcmpo + bf 1
    }
    return best;
}
```

The loop is a `do/while (++i < 4)`; `best_d2` stays `1.0e9f` when nothing
passes the gates, and `best` stays `0`.

### Notes that matter for a faithful port

* `bl 0x802DCD70` at `0x80A4B158` is `__savegprlr_26`; the export's
  `iVar3 = func_0x802dcd70()` and the `iVar7`/`uVar6` locals are artefacts of
  that call. `r26` is simply `this`.
* The gate order is mask **before** the virtual predicate: `rlwinm r0,r28,0,24,31`
  / `lbz r4,-25432(r30)` / `slw r0,r31,r0` / `and r0,r4,r0` then `beq cr0.2`.
* The predicate slot is `+0xE0` on the object at `p + 0x60` (the same
  controller object whose `+0x6C` slot returns the state byte in
  `0x80A4B2B0`).
* `0x80A4B2B0` is the only caller (`bl` at `0x80A4B358`). It first advances
  the crow's x by `f32[*(0x80429E00) + 0x38] - f32[*(0x80429DD0) + 0x8FEA8]`
  (the stage's current scroll speed), calls this body, then uses the returned
  player's `+0x60 -> +0x6C` state byte as `this[+0x5D0]` and sets state
  `0x80B71070`. A `0` return skips the state.
* The repo already has `const W126_FN_80A4B140` and a `call1` site at
  `src/game/enemies.rs:155629`. The doc comment there calls it a "crow
  lateral-offset query"; the raw stream shows a nearest-player search, so a
  body would replace that call and the label.

### Constants

| address | value | use |
|---|---|---|
| `0x80B2CA34` | `1.0e9f` | initial best distance |
| `0x80429CA8` | `u8` BSS byte | per-player slot mask, bit `i` |
| `+0xE0` | vtable slot | player predicate (non-zero = reject) |
| `+0xD0` / `+0xD4` | `f32` | player x/z offset triple |
| `0x8005F900` | 20 B, translated | player slot lookup |

### Verdict: conditional

Everything in the body is literal and writable, but two things decide which
player is returned: the main-BSS mask byte at `0x80429CA8` and the `+0xE0`
predicate on the player controller. Step needed: name the mask (which player
states set bit `i`) and the predicate (it is the only filter besides the
distance). Until then the selector cannot be proven to match.

---

## 2. `0x80A637E0` - `daEnGesso_c` near-player state select

| field | value |
|---|---|
| TSV size | 236 bytes |
| reachable | 59 instructions, ends at `0x80A638C8` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80a637e0_FUN_80a637e0.c` |
| signature | `pub unsafe fn fn_80a637e0(this: usize)` |
| verdict | **ready** |

### What the stream does

```c
void fn_80a637e0(usize this)
{
    fn_80a61b20(this);                       // +0x737 countdown, translated
    call_slot(this + 0x544, 0x1C);           // sub-object frame step (r3 = this+0x544)

    // angle stepper: value at this+0x104, target (i16)this+0x728, step 0x80
    step_short_clamped_wrap(this + 0x104, (i32)(i16)read_u16(this + 0x728), 0x80);

    if (near_player_x(this, 96.0f)) {        // f31 = 0x80B2D2E4
        call_slot_d4(this, 0x80B722D0);      // +0x60 vtable +0xD4, state object
    } else if (!near_player_x(this, 144.0f)) {  // f30 = 48.0 + 96.0
        call_slot_d4(this, 0x80B721D0);
    }
}
```

`near_player_x(this, r)` is `0x80097800` (unanalysed in `main.dol`, decoded
here): over the four player slots, reject a null slot, reject a slot whose bit
is clear in `r13[-0x59D8]`, and return `1` when
`fabs(player[+0xAC] - this[+0xAC]) < r`. It reads **x only** and returns `0`
after four misses.

### Notes that matter for a faithful port

* `0x80B2D47C` = `48.0f` and `0x80B2D2E4` = `96.0f`; `fadds f30,f0,f31`
  builds `144.0f`, so the band is `[0,96)` -> state A, `[96,144)` -> no change,
  `[144,inf)` -> state B.
* The two state objects are BSS (`0x80B722D0`, `0x80B721D0`); their
  `daEnGesso_c::StateID_*` texts are not in the file-backed image. The class's
  state-name pool is `0x80B4890B..0x80B48AE0` (`SwimReady`, `SwimUp`,
  `SwimWait`, `SwimDown`, `SwimDown_Fin`, `RestReady`, `Rest`, `Hide`,
  `InDokan`, `SetUp_Normal/Right/Left`, `CancelSetUp_*`); the bodies are free to
  use the addresses as opaque constants, exactly as the repo does elsewhere
  (747 `0x80B7xxxx` constants already live in `enemies.rs`).
* The handler owns the `0x80B48770` record of a 12-byte
  `{handler, 0, 0xFFFFFFFF}` table; the surrounding records are `0x80A63560`
  (`0x80B48758`), `0x80A637D0` (`0x80B48764`), `0x80A637C0` (`0x80B4877C`) and
  `0x80A638E0` (`0x80B48788`).
* `step_short_clamped_wrap` is the repo's translation of `0x8015F4F0`
  (`value: *mut i16, target: i32, step: i32) -> u32`); its `1` return is
  unused here.

### Constants

| address | value | use |
|---|---|---|
| `0x80B2D47C` | `48.0f` | base for the `144` band and the state delta |
| `0x80B2D2E4` | `96.0f` | inner band |
| `0x80B722D0` | BSS state object | set when the x distance is under 96 |
| `0x80B721D0` | BSS state object | set when the x distance is 144 or more |
| `0x8015F4F0` | 100 B, translated (`step_short_clamped_wrap`) | angle step |
| `0x80A61B20` | 52 B, translated | per-frame angle countdown |
| `0x80097800` | 192 B, unanalysed | `near_player_x(this, f1)` |

### Verdict: ready

Every field, branch and constant is literal, the two `main.dol` helpers are
either translated (`0x8015F4F0`, `0x80A61B20`) or fully decoded above
(`0x80097800`), and the two opaque BSS state objects are used the same way the
existing module already uses state records.

---

## 3. `0x80AD45E0` - carrier/rail offset commit

| field | value |
|---|---|
| TSV size | 224 bytes |
| reachable | 56 instructions, ends at `0x80AD46BC` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80ad45e0_FUN_80ad45e0.c` |
| signature | `pub unsafe fn fn_80ad45e0(this: usize)` |
| verdict | **ready** |

### What the stream does

```c
void fn_80ad45e0(usize this)                 // r30
{
    if (*(u32*)(this + 0x71C) == 0) return;
    if (*(u32*)(this + 0x6BC) == 0) return;  // holder id, loaded into r3

    usize holder = fn_80162d00(*(u32*)(this + 0x6BC));   // registry lookup
    if (holder == 0) fn_80162510(this);                  // mark/shutdown helper

    f32 hx = *(f32*)(holder + 0xAC);
    f32 hy = *(f32*)(holder + 0xB0);
    f32 hz = *(f32*)(holder + 0xB4);

    f32 x = hx + *(f32*)(this + 0x700);
    f32 y = hy + *(f32*)(this + 0x704);
    f32 z = hz + *(f32*)(this + 0x708);

    *(f32*)(this + 0xAC) = x + *(f32*)(this + 0x618);
    *(f32*)(this + 0xB0) = y + *(f32*)(this + 0x61C);
    *(f32*)(this + 0xB4) = 500.0f;                       // 0x80B30924

    *(f32*)(this + 0x83C) = *(f32*)(this + 0x82C) - y;   // previous y delta
    *(f32*)(this + 0x828) = x;
    *(f32*)(this + 0x82C) = y;
    *(f32*)(this + 0x830) = z + *(f32*)(this + 0x620);
}
```

The three `stfs` at `0x80AD4670`/`0x80AD4678`/`0x80AD467C` and the three at
`0x80AD4680`/`0x80AD4684`/`0x80AD4688` write two scratch vectors at `r1+0x14`
and `r1+0x08`; nothing reads them back. They are the compiler's temporaries for
the two `x/y/z` triples and can be dropped in Rust if the store order is kept.

### Notes that matter for a faithful port

* `fn_80162D00(id)` is the translated registry lookup and takes the word at
  `this + 0x6BC`, not the actor. The export prints `func_0x80162d00()` with no
  arguments because the register is set eight bytes earlier
  (`lwz r3,1724(r3)` at `0x80AD4604`).
* `0x80162510(this)` is the "recursive +0x0B shutdown mark plus virtual +0x38"
  helper; the original calls it only when the lookup failed, then keeps using
  the null pointer, so a live `+0x6BC` handle is an invariant.
* The cluster has no class string in the module: the vtable that owns
  `0x80AD45C0` sits at `0x80B5BD8C`, next to `0x80AD47A0`/`0x80AD47B0`/
  `0x80AD47D0` and the `obj_door_obake` strings at `0x80B5B79C`. Treat it as
  the `obj_door_obake` carrier family.
* Called from `0x80AD39F0` (inside the `0x80AD3xxx` rail/range cluster).

### Constants

| address | value | use |
|---|---|---|
| `0x80B30924` | `500.0f` | z written to `+0xB4` |
| `+0x618`/`+0x61C`/`+0x620` | `f32` | per-axis bias added to the holder position |
| `+0x700`/`+0x704`/`+0x708` | `f32` | holder-relative offset |
| `+0x828`/`+0x82C`/`+0x830` | `f32` | last committed holder point |
| `+0x83C` | `f32` | y delta against the previous commit |
| `0x80162D00` | 72 B, translated | id -> object |
| `0x80162510` | 116 B, translated | shutdown mark |

### Verdict: ready

The gate, the lookup argument, all seven stores and the one float constant are
literal; both callees are already translated.

---

## 4. `0x80A80E50` - `daEnIgaKuribo_c` `Kyoro` state entry

| field | value |
|---|---|
| TSV size | 220 bytes |
| reachable | 55 instructions, ends at `0x80A80F28` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80a80e50_FUN_80a80e50.c` |
| signature | `pub unsafe fn fn_80a80e50(this: usize)` |
| verdict | **ready** |

### What the stream does

```c
void fn_80a80e50(usize this)
{
    fn_80a80710(this, 0x80B4D760 /* "walk" */, 0, /* f1 */ 0.0f);
    fn_80a80780(this, 0x80B4D7A0 /* "look" */, 1);
    fn_80168fe0(this + 0x5BC, /* f1 */ 0.0f);            // element slot +0x20

    *(f32*)(this + 0xE8) = 0.0f;                          // 0x80B2E2F8
    *(f32*)(this + 0xEC) = 0.0f;
    *(f32*)(this + 0xF0) = 0.0f;

    *(u16*)(this + 0x658) = 208;                          // 0x00D0
    *(u16*)(this + 0x65A) = 0xC000;                       // -16384 as s16

    u16 turns = 0xC000;                                   // sth r0,8(r1)
    f32 a = (f32)turns * (1.0f / 256.0f);                 // 192.0f, qr3 u16 load
    f32 s = sin(a);                                       // 0x80237BD0
    *(i16*)(this + 0x100) = (i16)(i32)(1280.0f * s);      // ~ -266

    *(u32*)(this + 0x65C) =
        fn_80161460(W50_F_SCALE_RECORD, 90) + 300;        // 300..389 frames
}
```

### Notes that matter for a faithful port

* The export prints `func_0x80168fe0((double)DAT_80b2e2f8, param_1 + 0x5bc)`;
  the stream is `r3 = this+0x5BC, f1 = 0.0`, so the raw call shape is
  `(target, blend)`. The repo wrapper lists `value: f64` first; the argument
  that must land in `r3` is the target, not the float.
* **`0x800B2EE0` returns the PRNG value.** Its own export is typed `void`, but
  it is a two-instruction tail that forwards to `0x80161460` (LCG
  `state * 1664525 + 1013904223`, then `mulhi(state, range)`), and the caller
  uses `r3 + 300`. The repo's `fn_800b2ee0(value: u32)` drops that value;
  the spec should call `fn_80161460(W50_F_SCALE_RECORD, 90)` directly or add a
  returning adapter. The timer is therefore `300..389`, not a constant.
* `psq_l` at `0x80A80ECC` is the `qr3` load of the `0xC000` halfword;
  `(f32)0xC000 / 256 = 192.0`, `sin(192 deg) = -0.20791`, and
  `1280 * -0.20791 = -266.1` -> `(i16)-266 = 0xFEF6`.
* The stack halfword is written by `sth r0,8(r1)` where `r0 = 0xC000` from
  `addi r0,r0,-16384` at `0x80A80EB0` (`li` form, `r0` is not read).
* Class evidence: the state table at `0x80B4DAC0` holds this body beside
  `0x80A80C50`, `0x80A80F30`, `0x80A80F40`, `0x80A81050`, `0x80A81140`,
  `0x80A81150`, and the state-name pool at `0x80B4DB53` is
  `daEnIgaKuribo_c::StateID_{KyoroSt,Kyoro,Sleep,Jump,IgaWalk,IgaTurn,DieFumi}`.
  The clip strings are `0x80B4D760 "walk"` and `0x80B4D7A0 "look"`.

### Constants

| address | value | use |
|---|---|---|
| `0x80B2E2F8` | `0.0f` | velocity reset and blend |
| `0x80B2E320` | `0.00390625f` | 1/256 scale for the `qr3` load |
| `0x80B2E324` | `1280.0f` | angle amplitude |
| `0x80B4D760` | `"walk"` | first clip |
| `0x80B4D7A0` | `"look"` | second clip |
| `0x00D0` / `0xC000` | `208` / `-16384` | `+0x658` / `+0x65A` |
| `0x80A80710` / `0x80A80780` | 112 B / 108 B, translated | clip bind helpers |
| `0x80237BD0` | translated | sine |

### Verdict: ready

All stores, the `qr3` reduction, both clip names and the timer arithmetic are
literal once `0x800B2EE0` is treated as value-returning.

---

## 5. `0x80A06220` - fire-cannon slot spawn from a `0x14`-byte record

| field | value |
|---|---|
| TSV size | 212 bytes |
| reachable | 53 instructions, ends at `0x80A062F0` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80a06220_FUN_80a06220.c` |
| signature | `pub unsafe fn fn_80a06220(this: usize) -> usize` |
| verdict | **ready** |

### What the stream does

```c
usize fn_80a06220(usize this)                     // r30
{
    i32 idx = (i32)fn_80a06190(this);             // 0..3, or -1
    if (idx < 0) return 0;                        // 0xFFFFFFFF -> 0

    u8 rec[0x14];
    fn_80a06130(rec, *(u32*)(this + 0x10) + 0x74C, idx);   // 3 words + 3 u16

    f32 x = *(f32*)(rec + 0x0);
    f32 y = *(f32*)(rec + 0x4);
    f32 z = *(f32*)(rec + 0x8);

    f32 dy = 10.0f * (f32)(idx + 1);              // 0x80B2AA84
    f32 pos[3] = { x, y, z - dy };                // r1+8 .. r1+16

    u32 scratch[2];                               // r1+0x20, out param
    usize spawned = call5(0x80064610, 0x98, idx, pos, scratch, 0);
    if (spawned != 0) {
        *(u32*)(this + idx * 4) = *(u32*)spawned; // stwx
    }
    return spawned;
}
```

### Notes that matter for a faithful port

* `fn_80A06190(this)` walks up to four words of `this`, stopping on a null
  word, on `0x80162D00` returning null, or on the returned object's `+0x5BC`
  byte being zero; it returns the count (`0..3`) or `0xFFFFFFFF`.
  `cmpi r3,0` + `bf 0` then treats the negative result as "no slot".
* `fn_80A06130(out, table, index)` computes
  `record = ((u8[table+0xA05] - u8[table+index+0xA00]) & 0x7F) * 0x14` and
  copies `3` words plus `3` halfwords. Only the first three words (the `f32`
  position) are read here.
* The `(idx+1)` double is built with the `0x43300000` bias; the subtraction
  constant is `0x80B2AA88` = `2^52 + 2^31` (`4503601774854144.0`), and
  `0x80B2AA84` = `10.0f`.
* The call shape at `0x80A062C4` is `r3 = 0x98, r4 = idx, r5 = pos,
  r6 = scratch, r7 = 0`. `0x80064610` is `0x80064600(a5)` followed by
  `0x8006CA80(a1..a4)`, which forwards to `0x80162B00(type,
  0x8006C500(r13[-0x6F4E], 0), idx, 2)`: a five-argument spawn wrapper that
  the repo has no translation for yet.
* The result word is stored into `this[idx]` (`rlwinm r0,r31,2,0,29` +
  `stwx r4,r30,r0`), so `this` begins with a four-word handle array.
* Callers: `bl` at `0x80A06338` and `0x80A06520`, both inside the
  `0x80A06xxx` fire-cannon cluster whose vtable is
  `0x80B33D98..0x80B33DB8` (`0x80A06B00`, `0x80A06B80`, `0x80A06B90`,
  `0x80A06300`, `0x80A063D0`, `0x80A06C80..0x80A06CB0`). The neighbouring
  state string is `daEnBaramakiGesso_c::StateID_Attack` and the asset strings
  are `g3d/fire_cannon.brres` / `Wm_en_fireburner*`; the class label is not
  conclusive, the spawn role is.

### Constants

| address | value | use |
|---|---|---|
| `0x80B2AA84` | `10.0f` | per-slot y drop multiplier |
| `0x80B2AA88` | `2^52 + 2^31` | int -> double bias |
| `0x98` | spawn type | first argument of `0x80064610` |
| `+0x10` / `+0x74C` | pointer / offset | record table base |
| `idx * 4` | offset | spawn handle store in `this` |
| `0x80A06190` / `0x80A06130` | 132 B / 84 B, translated | slot count / record copy |

### Verdict: ready

The body is literal end to end; the only new engine surface is the
five-argument spawn wrapper, which is documented above and can be reproduced
with `platform::call5`.

---

## 6. `0x80A5DBC0` - `daEnGabon_c` angle step

| field | value |
|---|---|
| TSV size | 212 bytes |
| reachable | 53 instructions, ends at `0x80A5DC90` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80a5dbc0_FUN_80a5dbc0.c` |
| signature | `pub unsafe fn fn_80a5dbc0(this: usize) -> i32` |
| verdict | **conditional** |

### What the stream does

```c
i32 fn_80a5dbc0(usize this)                    // r29
{
    f32 speed = *(f32*)(this + 0x5AC);
    i32 q     = (i32)speed;                    // fctiwz + lwz
    i32 r     = 0x10000 / q;                   // divw r0,r4,r0   (q != 0)
    i8  a     = (i8)r;                         // extsb r30,r0

    // 0x80168080 is a thunk: f1 = slot +0x20 of *(this+0x584+4)
    f32 k     = slot_f32(this + 0x584, 0x20);

    f64 t     = (f64)a * (f64)k;
    i32 ti    = (i32)t;                        // fctiwz
    u16 turns = (u16)(i16)ti;                  // sth r0,8(r1)
    f32 ang   = (f32)turns * (1.0f / 256.0f);  // qr3 u16 load

    f32 s     = sin(ang);                      // 0x80237BD0
    i16 m     = *(i16*)(this + 0x660);         // lha r0,1632(r29)
    return (i32)((f32)m * s);                  // fctiwz, low word
}
```

### Notes that matter for a faithful port

* **The `+0x20` slot returns `f32`.** `bl 0x80168080` at `0x80A5DBFC` is a
  tail-call thunk (`lwz r3,4(r3); lwz r12,0(r3); lwz r12,32(r12); bcctr`), so
  the slot's float result arrives in `f1` and is consumed at `0x80A5DC24`
  (`fmuls f0,f0,f1`). The repo's `fn_80168080(element: usize)` is typed `()`
  and discards it.
* The signed byte is produced by `divw r0,r4,r0` where `r4 = 0x10000`
  (`addis r4,r0,1`) and `r0 = (i32)this[+0x5AC]`, then `extsb r30,r0`. A zero
  `+0x5AC` traps; the original relies on a positive speed.
* The int -> double conversion is the standard `xoris r?,0x8000` /
  `lis r?,0x4330` bias pair; `0x80B2D1C0` is the `2^52 + 2^31` subtraction
  constant and `0x80B2D1BC` is `1/256`.
* The `(i16)` truncation before the `qr3` load is deliberate: negative angles
  wrap to the `0xC000..` half of the turn circle.
* The final multiply is signed: `lha` sign-extends `this[+0x660]`, and the
  result is truncated through `fctiwz` and returned in `r3` (low word of the
  stack double at `r1+0x30`).
* Existing Rust: `const FN_80A5DBC0` and
  `call1(FN_80A5DBC0, actor) as u16 as i16` at `src/game/enemies.rs:26628`;
  the caller consumes the low 16 bits as an angle delta, which matches the
  `i32` return here.
* Class evidence: the handler records at `0x80B47AE8`/`0x80B47AFC` point at the
  neighbouring `0x80A5DA20`/`0x80A5DDA0` bodies, and the name pool at
  `0x80B47C3F` is `daEnGabon_c::StateID_{Appear,Move,MoveEd,Attack,DieFumi,
  DieFall}` (model `g3d/nokonokoB.brres`).

### Constants

| address | value | use |
|---|---|---|
| `0x80B2D1BC` | `0.00390625f` | 1/256 scale |
| `0x80B2D1C0` | `2^52 + 2^31` | int -> double bias |
| `+0x5AC` | `f32` | speed divisor for `0x10000 / q` |
| `+0x584` | element | holder whose `+4` object answers slot `+0x20` |
| `+0x660` | `s16` | amplitude |
| `0x80237BD0` | translated | sine |
| `0x80168080` | 20 B, translated but `void`-typed | slot `+0x20` thunk |

### Verdict: conditional

Everything except `f1` is literal. `f1` is the float returned by the `+0x20`
slot on the object owned by the element at `this + 0x584`, and neither the
element type nor that slot's meaning is established. Step needed: identify the
`+0x584` element (its `+4` object is a 0x38-byte registry element) and retype
`0x80168080` as returning `f32` before translating the body.

---

## 7. `0x80AD46C0` - carrier/rail frame step with the height wrap

| field | value |
|---|---|
| TSV size | 212 bytes |
| reachable | 53 instructions, ends at `0x80AD4790` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80ad46c0_FUN_80ad46c0.c` |
| signature | `pub unsafe fn fn_80ad46c0(this: usize)` |
| verdict | **ready** |

### What the stream does

```c
void fn_80ad46c0(usize this)                   // r30
{
    if (*(u32*)(this + 0x71C) == 0) return;
    if (*(u32*)(this + 0x6BC) == 0) return;

    usize holder = fn_80162d00(*(u32*)(this + 0x6BC));
    if (holder == 0) fn_80162510(this);

    f32 v = *(f32*)(holder + 0x524);           // carrier speed/offset

    if (*(u32*)(this + 0x720) == 0) {
        f32 x0 = *(f32*)(this + 0xAC);                       // f31, saved first
        f32 x  = (f32)fn_80101aa0(*(f32*)(this + 0x81C) + v); // f64 height wrap
        *(f32*)(this + 0xAC) = x;
        *(f32*)(this + 0xB0) = *(f32*)(this + 0x820) + *(f32*)(this + 0x61C);
        *(f32*)(this + 0x838) = x - x0;                      // frame delta
    } else {
        *(f32*)(this + 0x838) = 0.0f;                        // 0x80B3092C
        *(f32*)(this + 0xB0) = *(f32*)(this + 0x61C)
                             + *(f32*)(this + 0x820) + v;
    }
}
```

### Notes that matter for a faithful port

* `fn_80101AA0` is the translated "wrap a height into the
  `0x800789A0`/`0x80078B40` camera window" helper; it takes and returns `f64`,
  so the `f32` in and out must be widened/narrowed at the call. Its loop is
  bounded by a four-iteration cap and the stage-mode byte.
* The `f31` save at `0x80AD4724` happens **before** the wrap; the delta at
  `0x80AD4730` is `frsp` of the returned `f1` minus that saved x.
* The `else` arm writes `0.0f` to `+0x838` and a three-term y; it does not
  touch `+0xAC`.
* Same holder gate and registry lookup as body 3 (`+0x71C`, `+0x6BC`,
  `0x80162D00`, `0x80162510`), so the two bodies are two halves of the same
  carrier step; body 3 commits the offset, this one advances the frame.
* Called from `0x80AD3AA8` (inside the `0x80AD3xxx` cluster).

### Constants

| address | value | use |
|---|---|---|
| `0x80B3092C` | `0.0f` | delta in the flag-set arm |
| `+0x524` | `f32` | carrier offset added to `+0x81C`/`+0x820` |
| `+0x720` | `u32` | arm selector |
| `+0x81C` / `+0x820` | `f32` | per-axis offsets |
| `+0x838` | `f32` | frame delta |
| `0x80101AA0` | 288 B, translated | height wrap |
| `0x80162D00` / `0x80162510` | translated | holder lookup / shutdown |

### Verdict: ready

All arithmetic, both arms and the one float constant are literal; both helper
families are translated.

---

## 8. `0x80AF3570` - cannon guide world position

| field | value |
|---|---|
| TSV size | 196 bytes |
| reachable | 49 instructions, ends at `0x80AF3630` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80af3570_FUN_80af3570.c` |
| signature | `pub unsafe fn fn_80af3570(out: usize, src: usize)` |
| verdict | **ready** |

### What the stream does

```c
void fn_80af3570(usize out, usize src)         // r28 = out, r29 = src
{
    u16 turns  = *(u16*)(src + 0x7A4);          // lha + sth r0,10(r1)
    f32 ang    = (f32)turns * (1.0f / 256.0f);  // qr3, 0x80B313D4
    f32 s      = sin(ang);                      // 0x80237BD0
    f32 s32    = 32.0f * s;                     // 0x80B313B0
    f32 c      = cos(ang);                      // 0x80237C40
    f32 c32    = 32.0f * c;

    *(f32*)(out + 0x0) = *(f32*)(src + 0x798) + s32;
    *(f32*)(out + 0x4) = *(f32*)(src + 0x79C) + c32;
    *(f32*)(out + 0x8) = *(f32*)(src + 0x7A0);
}
```

### Notes that matter for a faithful port

* The angle is read with `lha r0,1956(r4)` (sign-extended) and stored as a
  halfword, then loaded through `qr3` as **unsigned** 16-bit; the two reads
  agree for every turn value, and the reduction is `turns / 256` turns.
* Both `qr3` loads read the same actor field: the first stores it at `r1+10`
  (`sth` at `0x80AF35A4`), the second re-reads `src+0x7A4` and stores it at
  `r1+8` (`sth` at `0x80AF35CC`) because the original re-materialises the value
  after the `sin` call instead of keeping the pointer.
* The pattern `lfs f0,5076(r30)` is `1/256` and `lfs f0,5040(r31)` is `32.0f`;
  the export's `unaff_GQR3`/`ldexpf` blocks are cosmetic.
* Existing Rust: `const BURST_FN_80AF3570` with
  `call2(BURST_FN_80AF3570, sampled.as_mut_ptr(), this)` at
  `src/game/enemies.rs:60304`, documented as "builds a world position from the
  guide angle" - the same reading.
* Six `bl` callers: `0x80AF2A0C`, `0x80AF2A18`, `0x80AF2A24` (three back-to-back
  in one frame step) and `0x80AF39E8`, `0x80AF3FA8`, `0x80AF404C`. The cluster
  is the pointer-cannon sweep (`0x80AF3400` launch, `0x80AF3B40` interval
  constants, `0x80AF3C80` `Wm_en_burst_*`); the asset strings are
  `g3d/MG_remo_cannon.brres` / `MG_remo_cannon_guide` at `0x80B62B74`.

### Constants

| address | value | use |
|---|---|---|
| `0x80B313B0` | `32.0f` | guide radius |
| `0x80B313D4` | `0.00390625f` | 1/256 turn scale |
| `+0x798`/`+0x79C`/`+0x7A0` | `f32` | guide origin |
| `+0x7A4` | `u16` | guide angle (turns) |
| `0x80237BD0` / `0x80237C40` | translated | sine / cosine |

### Verdict: ready

Two `qr3` reductions, two float constants and three stores; the trig helpers
are already translated. Callers can keep the `call2` shape.

---

## 9. `0x80A2EB40` - `daEnBubble_c` water probe

| field | value |
|---|---|
| TSV size | 188 bytes |
| reachable | 47 instructions, ends at `0x80A2EBF8` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80a2eb40_FUN_80a2eb40.c` |
| signature | `pub unsafe fn fn_80a2eb40(this: usize, dy: f32) -> u32` |
| verdict | **conditional** |

### What the stream does

```c
u32 fn_80a2eb40(usize this, f32 dy)             // f31 = dy
{
    f32 pos[2];
    pos[0] = *(f32*)(this + 0xAC);               // x
    pos[1] = *(f32*)(this + 0xB0) + dy;          // y + dy

    u32 p3; f32 p4; u16 p5;                      // r1+16, r1+12, r1+8
    i32 hit = (i16)fn_800eb930_ret(
        *(u32*)0x8042A100,                       // manager
        (u32)pos, (usize)&p3, (usize)&p4, (usize)&p5,
        *(u8*)(this + 0x38F));                   // ground kind byte

    if (hit == 1) return 1;

    usize base = *(u32*)0x80429DD0 + 0x90000;    // addis r3,r3,9
    if (*(u8*)(base + 0x2C) != 3) return 0;      // stage water mode
    if (*(f32*)(base - 0x13C) < *(f32*)(this + 0xB0) + dy) return 1;
    return 0;
}
```

* `0x800EB930` scans up to 0x50 records of `0x1C` bytes matching the `+0x19`
  type byte. Its result (the low half of `r3`) is the matched record's kind
  byte from `0x800EB640` (or the `0x800EB6E0` result), or `-1`. The repo's
  `fn_800eb930` wrapper is typed `()`; the specification needs a value-returning
  adapter.
* The `0x90000` bias is deliberate: the code loads the pointer from
  `0x80429DD0` (main BSS) and then adds `0x90000` to it (`addis r3,r3,9`), so
  the mode byte is at `ptr + 0x9002C` and the water surface at
  `ptr + 0x8FEC4` (the code reaches the latter as `-0x13C` from the biased
  base).
* The last comparison is a strict float `<` (`fcmpo` + `bf 1`): return `1`
  only when the water surface is strictly above `this[+0xB0] + dy`.
* Existing Rust: `const W123_FN_80A2EB40` and
  `call2_f64(W123_FN_80A2EB40, read_f32(W123_FLOAT_80B2BCF4) as f64, actor)` at
  `src/game/enemies.rs:151048`; the call site passes the constant `20.0f`
  (`0x80B2BCF4`) as `dy`.
* Class evidence: the state table at `0x80B3C670` and the name pool at
  `0x80B3C6BB` (`daEnBubble_c::StateID_{UpMove,DieFall,DieBigFall}`), model
  `g3d/env_underwater.brres`.

### Constants

| address | value | use |
|---|---|---|
| `0x8042A100` | `u32` BSS pointer | collision/water manager |
| `0x80429DD0` | `u32` BSS pointer | stage object; mode at `+0x9002C`, surface at `+0x8FEC4` |
| `0x80B2BCF4` | `20.0f` | `dy` at the only caller |
| `+0x38F` | `u8` | ground kind passed as the scan's type |
| `0x800EB930` | 372 B, translated but `void`-typed | record scan |

### Verdict: conditional

The body is literal, but the answer depends on two external pieces of state:
the main-BSS manager at `0x8042A100` (its record layout) and the stage object
at `0x80429DD0` (mode byte `3` and the water surface). Step needed: name the
mode byte and the surface field, and add the value-returning
`0x800EB930` adapter.

---

## 10. `0x80B05A80` - `daEnSnakeBlock_c` clip binder over `N` segments

| field | value |
|---|---|
| TSV size | 180 bytes |
| reachable | 45 instructions, ends at `0x80B05B30` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80b05a80_FUN_80b05a80.c` |
| signature | `pub unsafe fn fn_80b05a80(this: usize)` |
| verdict | **ready** |

### What the stream does

```c
void fn_80b05a80(usize this)                     // r27
{
    fn_80b04120(this + 0x540, 0x80B66478 /* "ridden" */);
    fn_80166310(this + 0x588, 0);                // r4 = 0, f1 = 0.0

    fn_80b04120(this + 0x6FC, 0x80B66478);
    fn_80166310(this + 0x744, 0);

    u32 n = *(u32*)(this + 0x2B18);              // segment count
    for (u32 k = 0; k < n; k++) {
        usize seg = this + 0x8B8 + k * 0x1B8;
        fn_80b04120(seg, 0x80B66478);
        fn_80166310(seg + 0x48, 0);
    }

    *(u16*)(this + 0x2B24) = (u16)(n + 60);      // NOT the constant 60
}
```

### Notes that matter for a faithful port

* **Ghidra prints `0x3c` at `0x80B05B18`, but the instruction is
  `addi r0,r0,60` (`0x3800003C`), and `r0` still holds the segment count from
  `lwz r0,11032(r27)` at `0x80B05B08`.** The timer is `count + 60`
  (`this[+0x2B18] + 60`). This is the only place in the twelve where the export
  and the stream disagree on a value.
* `fn_80B04120(owner)` is the clip/attachment binder that re-binds `+0x48`
  through `0x80165FD0`; it takes only the owner in the raw call, and the
  `"ridden"` name is the `r4` in every one of the three call shapes.
* `fn_80166310(this, index)` is the element dispatch to vtable slot `+0x28`.
  The call also loads `f1 = 0.0f` (`0x80B31A58`); the wrapper's documented
  signature takes only `(this, index)`, so the float is not consumed.
* The loop body is a fixed `0x1B8` stride with the clip element at `+0x48`;
  the four initial holders (`+0x540`, `+0x588`, `+0x6FC`, `+0x744`) are bound
  before the loop and are not part of the counted array.
* `bl 0x802DCD74` / `bl 0x802DCDC0` are `_savegprlr_27` / `_restgprlr_27`.
* Class evidence: the table at `0x80B66798` and the name pool at `0x80B6682C`
  (`daEnSnakeBlock_c::StateID_{Wait,Move,Shake,Collapse1,Collapse2,Collapse3}`).

### Constants

| address | value | use |
|---|---|---|
| `0x80B66478` | `"ridden"` | clip/attachment name, four call sites |
| `0x80B31A58` | `0.0f` | blend passed to `0x80166310` |
| `+0x2B18` | `u32` | segment count |
| `+0x2B24` | `u16` | `count + 60` |
| `0x1B8` / `0x48` | strides | segment / clip element |
| `0x80B04120` / `0x80166310` | translated | binder / element dispatch |

### Verdict: ready

All four binders, the loop bound and the `count + 60` timer are literal; the
only trap is the Ghidra `0x3c`, which the stream corrects.

---

## 11. `0x80B0DE10` - `daEnTeresa_c` state step

| field | value |
|---|---|
| TSV size | 180 bytes |
| reachable | 45 instructions, ends at `0x80B0DEC0` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80b0de10_FUN_80b0de10.c` |
| signature | `pub unsafe fn fn_80b0de10(this: usize)` |
| verdict | **ready** |

### What the stream does

```c
void fn_80b0de10(usize this)                     // r31
{
    if (fn_80b0d4d0(this) == 0) {                // turn-order verdict
        *(u8*)(this + 0x9E7) = 1;
        call_slot_d4(this, 0x80B78988);          // retire state
        return;
    }

    fn_80b0c890(this);                           // overlap clears +0x9E0
    if (*(u16*)(this + 0x9E0) == 0 &&
        fn_80b0d150(this) != 0) {                // two sub-objects live
        *(u16*)(this + 0x9E2) = 2;
        *(u32*)(this + 0x9C8) = 0;
        call_slot_d4(this, 0x80B78948);          // wake state
    }
    fn_80b0cfd0(this);                           // six sub-object resets
}
```

### Notes that matter for a faithful port

* `fn_80B0D4D0(this)` is the translated "turn-order verdict over the four
  player slots"; `fn_80B0C890(this)` returns `1` when the `0x80097B30` overlap
  test succeeds, but the return is unused here (it only clears `+0x9E0`).
* `fn_80B0D150(this)` checks that both `this+0x640` and `this+0x678` answer
  non-zero through `0x80169020`.
* The two state objects are BSS (`0x80B78988`, `0x80B78948`); the class name
  pool after the table is at `0x80B68364`
  (`daEnTeresa_c::StateID_{Move,Stop,Recover}`). The `+0x9E2 = 2` write is a
  substate index; it is not proven to be the second entry of that triple.
* `bl 0x80B0D4D0` at `0x80B0DE24` passes `this` in `r3` unchanged from entry;
  the export prints no arguments.
* The state table at `0x80B68354` holds this body as `{handler, 0, -1}`.

### Constants

| address | value | use |
|---|---|---|
| `0x80B78988` | BSS state object | taken when the turn-order gate fails |
| `0x80B78948` | BSS state object | taken when `+0x9E0 == 0` and the probe passes |
| `+0x9E7` | `u8` | "retired" byte |
| `+0x9E2` | `u16` | set to `2` |
| `+0x9C8` | `u32` | cleared |
| `0x80B0D4D0` / `0x80B0C890` / `0x80B0D150` / `0x80B0CFD0` | 520/100/84/160 B, translated | gate / overlap / probe / reset |

### Verdict: ready

All four callees are translated and every field and store is literal; the two
BSS state objects are used the same way the rest of the module uses them.

---

## 12. `0x80ACE690` - net-Noko clip blend commit

| field | value |
|---|---|
| TSV size | 172 bytes |
| reachable | 43 instructions, ends at `0x80ACE738` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80ace690_FUN_80ace690.c` |
| signature | `pub unsafe fn fn_80ace690(this: usize, name: u32, mode: u32, blend: f32)` |
| verdict | **ready** |

### What the stream does

```c
void fn_80ace690(usize this, u32 name, u32 mode, f32 blend)   // r30 = this
{
    u32 node = 0;
    node = brres_get_anm_chr(this + 0x544, name);   // clip lookup by name

    fn_801651f0(this + 0x588, this + 0x548, (usize)&node, mode);
    fn_80169f80(this + 0x548, this + 0x588, blend); // (token, object, f1)

    fn_801680e0(this + 0x588,
                *(u32*)(this + 0x5F0) == 1 ? 2.0f    /* 0x80B3078C */
                                           : 1.5f);  /* 0x80B30788 */
}
```

### Notes that matter for a faithful port

* The incoming `r5` (`mode`) is the value the only analysed caller loads from
  the word table at `0x80B30780` with `this[+0x348] * 4` (`index 0 -> 0`,
  `index 1 -> 2`), and `blend` is `0.0f` there (`0x80B307A8`).
* The clip name in that caller is `0x80B5AB9C "net_walk1"`, with the
  side-indexed table entry as the mode word.
* **The repo's `fn_80169f80(token, object, descriptor)` is mis-modelled.** The
  raw callee is `r3 = token, r4 = object, f1 = blend`, saves `f1` in `f31`,
  and calls `0x80169C80(token + 0x0C, f1)` when the object's `+0x0C` slot
  answers zero, then `0x80168280(token, object)`. The third parameter is
  `f32`, not a pointer, and the attach call takes `token + 0x0C`, not `token`.
* `brres_get_anm_chr(this + 0x544, name)` is the translated
  `(holder, name) -> node` lookup.
* Class evidence: the caller cluster is the net Koopa block; the strings
  `daEnNetNokoLR_c::StateID_Turn` (`0x80B5AB38`) and
  `daEnNetNokoUD_c::StateID_{Move,Turn_St,Turn_Ed,NetMove}` (`0x80B5AF17..`)
  both front these tables, so the helper is shared by the LR and UD variants.
* `bl` callers: `0x80ACE9A8`, `0x80ACEC40`, `0x80ACEDF0`.

### Constants

| address | value | use |
|---|---|---|
| `0x80B30788` | `1.5f` | blend rate when `+0x5F0 != 1` |
| `0x80B3078C` | `2.0f` | blend rate when `+0x5F0 == 1` |
| `0x80B30780` | word table | caller's mode source |
| `+0x544` / `+0x548` / `+0x588` | holders | clip source / blend target / descriptor |
| `+0x5F0` | `u32` | rate selector |
| `0x8023A0B0` / `0x801651F0` / `0x80169F80` / `0x801680E0` | translated | clip lookup / blend setup / attach / rate |

### Verdict: ready

The body is literal; the one required adjustment is the `fn_80169F80` wrapper
signature (`f32` blend plus the `token + 0x0C` attach argument), which is a
repo-side correction rather than a missing fact.

---

## Shared corrections produced by this pass

1. **`0x800B2EE0` is value-returning.** It forwards to `0x80161460` (LCG) and
   its `r3` is the scaled random value. Body 4 consumes it as `rand + 300`.
2. **`0x80168080` returns the slot's `f32`** (`+0x20` on the element's `+4`
   object), not `void`. Body 6 consumes it.
3. **`0x80169F80`'s third argument is an `f32` blend**, and its attach call is
   `0x80169C80(token + 0x0C, blend)`. Body 12 depends on it.
4. **`0x80097800` is a real body**, not padding: `near_player_x(this, f1)`
   over the four player slots, x-axis only. Body 2 depends on it.
5. **`0x80064610` is a five-argument spawn wrapper**
   (`0x80064600` + `0x8006CA80` -> `0x80162B00`); body 5 needs it.
6. **`0x800EB930` returns the matched kind value** in `r3`; the repo wrapper
   is `void`. Body 9 needs the returning form.
7. **`addi r0,r0,60` is not `li r0,60`.** Body 10's timer is
   `segment_count + 60`, which the Ghidra export prints as `0x3c`.
8. **State objects above `0x80B6E438` are BSS.** The module's file-backed image
   ends there; `0x80B722D0`, `0x80B721D0`, `0x80B78988` and `0x80B78948` are
   runtime state objects whose text is not in this module.

## Verification performed (read-only)

```text
functions.tsv            3,555 rows, all decompiled = true
ledger/functions.csv    17,102 distinct addresses
docs/*.md headings         388 distinct 0x80A/0x80B addresses
src/ fn fn_<addr> bodies 15,938 distinct addresses
candidates after exclusion 40
claimed here                12   (2,468 bytes / 617 instructions)
every claimed body ends at entry + size - 4 with a terminal bclr
qr3 sites                   4   (0x80A80ECC, 0x80A5DC38, 0x80AF35A8, 0x80AF35D4)
```

No Rust, ledger, or other documentation file was modified by this pass.
