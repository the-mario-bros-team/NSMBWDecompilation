# Wave 154 - implementation specification for 12 untranslated `d_enemiesNP` bodies

## What this page is

`docs/ANALYSIS_WAVE127_ENEMIES.md` reclassified 384 formerly-skipped
`d_enemiesNP` addresses; waves 128-153 worked through that block. This page
takes the twelve largest bodies that are **still untranslated**, carry a
`decompiled = true` Ghidra export, and walk cleanly from their entry to a
terminal `blr` / tail `b`.

Snapshot: **2026-11-13 08:49**, `src/game/enemies.rs` at 7,269,361 bytes and
not modified since 08:38:28. A live scan of

* `work/ghidra/rel-d_enemiesNP/export/functions.tsv` - 3,555 rows, all
  `decompiled = true`,
* the `address` column of `ledger/functions.csv` - 17,131 distinct addresses,
* every `0x80Axxxxx` / `0x80Bxxxxx` address in a `^#` heading of any
  `docs/*.md` - 401 addresses,
* every `fn fn_<addr>` **body** defined under `src/` - 15,976 definitions,

leaves **15** addresses absent from all four sets. The twelve largest are below;
`0x80A07C80` (16), `0x80AF3840` (16) and `0x80A337B0` (4) are below the cut.

`fn fn_<addr>` bodies are matched only as definitions
(`^\s*(pub )?(unsafe )?(extern "C" )?fn fn_<addr>`); a
`const Wnnn_FN_80Axxxxx: usize = 0x...` address stub or a doc-comment mention is
**not** a body. Several of the rows below are already *called* by translated
bodies through such a stub - they are still untranslated.

This page is **read-only**: it writes no Rust, no ledger row and no other doc.

## Selection

The ranking metric is the `functions.tsv` extent, which for all twelve rows is
also the reachable body: each candidate was walked from its entry through every
conditional branch, every internal `b` and every linked `bctrl` until all paths
ended in `bclr`, `blr`, a tail `b` or a tail `bcctr`. Conditional `bclr` forms
were followed on both edges. In every one of the twelve the last instruction is
at `entry + size - 4`, so `reachable == extent`: no truncated arm, no
out-of-range branch, no `bctr` jump table.

| # | address | TSV size | reachable insns | `qr3` sites | role | verdict |
|---:|---|---:|---:|---:|---|---|
| 1 | `0x80ACDB00` | 148 | 37 | 0 | `daEnNetNokoLR_c` `net_walk2` walk-clip binder | **conditional** |
| 2 | `0x80A03C30` | 92 | 23 | 0 | `daEnBuku_c` 24-frame stage-effect timer | **ready** |
| 3 | `0x80AF3850` | 88 | 22 | 0 | burst cannon steer-target snap to a multiple of `800` | **ready** |
| 4 | `0x80A25F00` | 48 | 12 | 0 | PMF thunk, slot `+8` of the class table `0x80B3ABE0` | **conditional** |
| 5 | `0x80A2F9D0` | 48 | 12 | 0 | `daEnBubble_c` destructor thunk for the `+0x18` base | **conditional** |
| 6 | `0x80A2FA00` | 48 | 12 | 0 | `daEnBubble_c` destructor thunk for the `+0x24` base | **conditional** |
| 7 | `0x80A43EE0` | 48 | 12 | 0 | `Wm_ob_coin` destructor thunk for the `+0x18` base | **conditional** |
| 8 | `0x80A43F10` | 48 | 12 | 0 | `Wm_ob_coin` destructor thunk for the `+0x24` base | **conditional** |
| 9 | `0x80A48940` | 48 | 12 | 0 | `daEnCrasher_c` `+0x674` clamped accumulator | **conditional** |
| 10 | `0x80AF0370` | 44 | 11 | 0 | `daEnRedRing_c` 60-frame rumble start | **ready** |
| 11 | `0x80A933E0` | 36 | 9 | 0 | `block_cloud` splash request, factory id `0x54` | **ready** |
| 12 | `0x80B14D80` | 28 | 7 | 0 | thrown-object request, factory id `0x39` | **ready** |

**Split: 5 ready, 7 conditional.** Total 724 bytes of TSV extent and 193
reachable instructions.

The seven "conditional" rows are not blocked on unknown bytes: every operand is
decoded. Each needs one named datum fixed before a byte-exact port:

* rows 1 and 9 - the **width of the `f1` argument** (`lfs` single in the
  stream, promoted to `f64` by the repo's existing `call*_f64` sites), and for
  row 1 the one-line caller change that forwards the frame;
* rows 4-8 - the **hidden `r12` this-adjust descriptor**, which
  `platform::call3` cannot model (the repo already ships pre-existing copies of
  this shape as `partial`).

### Why these twelve and not the wave-154 bodies

A parallel wave-154 implementation lane appended 25 bodies to
`src/game/enemies.rs` at 08:38:28 (`docs/COVERAGE_WAVE154_ENEMIES.md`). Nine
addresses that were rank 4-12 of this lane's first pass became bodies while this
page was being prepared and are therefore **excluded**: `0x80B05B80`,
`0x80A4B0B0`, `0x80AF42C0`, `0x80A05DF0`, `0x80AD47E0`, `0x80AD4850`,
`0x80AEDB90`, `0x80AD48C0`, `0x80AED170`. Its two deliberate skips are rank 1
and 2 here and are resolved by the evidence in this page:

* `0x80A03C30` - the wave-154 coverage doc leaves it because "whether
  `0x80078470` reads `r6` cannot be established". The value in `r6` is the
  **literal `24`**, so the body is exact whether or not the callee reads it;
  see section 2.
* `0x80ACDB00` - the same doc leaves it because the only translated caller
  "calls it with only `r3`, so a body would have to invent the blend argument".
  The raw call site *does* supply the argument (`f1` from `0x80168080`); see
  section 1.

### Rows below the cut

| address | TSV size | why it is not claimed |
|---|---|---|
| `0x80A07C80` | 16 | tail branch into `main.dol` (`0x80168180`) with three staged arguments; no callee contract (wave144 recorded the same) |
| `0x80AF3840` | 16 | 4-instruction trampoline (`0x8015F4F0(this+0x7A4, (s16)r4, 400)`); below the size cut, and its contract is quoted in section 3 |
| `0x80A337B0` | 4 | 4-byte alias `b 0x80A337C0`; the target is unclaimed, so there is no body to type |

## The `qr3` pool at this wave

`docs/REL_ELF_MAPPING.md` pins **`GQR3 = 0x00050005`** (LD/ST type 5 = unsigned
16-bit, both scales 0, `W = 1`), installed by `main.dol` at
`0x8025EEEC`/`0x8025EEF0`. None of the twelve reads or writes a quantised pair:
the only `psq` words in the set are the two `qr0` `f31` spills in
`0x80ACDB00` (`psq_st f31,0x28(r1),1,0` at `0x80ACDB18` and its matching
`psq_l` at `0x80ACDB7C`), each beside `stfd`/`lfd f31,0x20(r1)`. Every other
row has no `psq` at all. `GQR3` therefore plays no part in this page.

## Reading conventions

* *Synthetic base* is `0x80A00000`. The REL has five file-backed `PT_LOAD`
  segments (`0x80A000F0`, `0x80B2A518`, `0x80B2A7A4`, `0x80B2A7B0`,
  `0x80B32DC0`); `file_offset = p_offset + (address - p_vaddr)`, which for the
  first segment is `address - 0x80A00000 + 0x1000`. The file-backed image ends
  at `0x80B6E438`; `0x80B6E500..0x80B799E0` is BSS (`bssSize = 0xB4E0`), so
  every `0x80B7xxxx` address below is a zero-filled state object, not image
  data.
* `this` is the incoming `r3` unless stated. `+0xAC`/`+0xB0`/`+0xB4` are x/y/z,
  `+0x348` the side/facing byte, `+0x100`/`+0x102`/`+0x104` the three `s16`
  angles.
* `addi rX,r0,imm` is `li rX,imm` and `addis rX,r0,imm` is `lis rX,imm`
  (RA = 0 means the literal zero, never `r0`). Negative `addi` offsets are
  written resolved (`addi r4,r4,-22532` = `0x80B5A7FC`).
* `_savegprlr_27` = `0x802DCD74` and `_restgprlr_27` = `0x802DCDC0` are
  register-save fragments, not argument-destroying stubs; the incoming
  `r3..r10` survive them.
* A `functions.tsv` size is a bucket extent. Each section's "reachable" line is
  the count from the control-flow walk, which agrees with the extent here.
* `nop` is `0x60000000` (`ori r0,r0,0`).

## Shared callee contracts used below

| address | contract as observed in this module |
|---|---|
| `0x8023A0B0` | `(res, name) -> u32` clip lookup (`resource::bres::brres_get_anm_chr`, translated) |
| `0x801651F0` | `(holder, node, &clip, mode)` attach (`resource::registry`, translated) |
| `0x80169F80` | `(f1 = blend, node, holder)` attach descriptor; the stream always passes a value loaded with `lfs` (single) |
| `0x80168FE0` | `(f1 = frame, holder)` stores the frame at `holder+0x30` (wave-152 bases) |
| `0x801680E0` | `(f1 = rate, holder)` forwards to the holder's `+0x28` slot (wave-152 bases) |
| `0x80168080` | `(holder) -> f32 in f1`, the holder's current clip frame |
| `0x80169020` | `(holder) -> u32`, non-zero when the holder's clip has finished |
| `0x801628C0` | `(size) -> u32` zero-filled default-heap allocation (translated) |
| `0x80066FC0` | base-actor constructor; installs the `+0x60`/`+0x394` slots |
| `0x80078470` | stage effect request, 5 arguments: `(f1 = x, f2 = y, r3 = manager, r4 = layer, r5 = count)`; no export row exists |
| `0x80064610` | effect/object factory, 5 arguments `(id, resource, out, 0, 0)`; forwards to `0x8006CA80` and returns the spawned object (`main.dol` export) |
| `0x800E41C0` | seven-argument input/rumble helper (`main.dol`; argument roles inferred in wave 17, values literal) |
| `0x802DCBFC` | CodeWarrior pointer-to-member dispatcher: reads a 12-byte descriptor at `r12` = `{i32 this_delta; i32 vtable_index; u32 target}` and calls `target(this+delta)`, or the vtable slot when `index >= 0` (translated as `runtime::cpp::fn_802dcbfc`) |

---

## 1. `0x80ACDB00` - `daEnNetNokoLR_c` `net_walk2` walk-clip binder

| field | value |
|---|---|
| TSV size | 148 bytes |
| reachable | 37 instructions, ends at `0x80ACDB90` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80acdb00_FUN_80acdb00.c` |
| signature | `pub unsafe fn fn_80acdb00(this: usize, frame: f32)` |
| verdict | **conditional** |

### What the stream does

```c
// f1 arrives as a single-precision clip frame; the body spills it to f31.
void fn_80acdb00(usize this, f32 frame)             // r31 = this, f31 = frame
{
    u32 clip = 0x8023A0B0(this + 0x544, 0x80B5A7FC /* "net_walk2" */);
    u32 mode = *(u32*)(0x80B30750 + (u32)*(u8*)(this + 0x348) * 4);

    0x801651F0(this + 0x588, this + 0x548, &clip, mode);
    0x80169F80(1.0f /* 0x80B30758 */, this + 0x548, this + 0x588);
    0x80168FE0(frame, this + 0x588);
}
```

Frame: `stwu r1,-0x30(r1)`; LR `+0x34`; `stfd f31,0x20` + `psq_st f31,0x28,1,0`
and the matching reload. `r31 = this`. The clip handle is a single stack word at
`r1+8`, passed by address in `r5`.

### Notes that matter for a faithful port

* **The float is a real argument, not decompiler noise.** Both call sites load
  it immediately before the call:
  * `0x80ACDE80`: `bl 0x80168080` with `r3 = this+0x588` leaves the holder's
    current frame in `f1`, then `r3 = this` and `bl 0x80ACDB00`;
  * `0x80ACDA9C`: `lfs f1,0x748(0x80B3)` = `*(f32*)0x80B30748 = 0.0f`, then
    `r3 = this` and `bl 0x80ACDB00`.
  The existing translation `fn_80acde00` (line 180010) calls
  `call1(W141_FN_80ACDB00, this)`, which drops `f1`. **Port prerequisite:** add
  `frame: f32`, pass `0.0` at the spawn site and the `0x80168080` result at the
  side-flip site (`call1_ret_f32`).
* `frame` reaches `0x80168FE0` in `f1` with **no conversion**: the stream is
  `fmr f1,f31` (`0x80ACDB6C`), a single-precision value. The callee stores it at
  `holder+0x30` (wave-152 bases). Use an f32 ABI
  (`extern "C" fn(f32, u32)`); the repo's `call2_f64(FN_80168FE0, ...)` sites
  promote `f32 -> f64` and would put a different bit pattern in `f1`.
* `mode` is `0` for side `0` and `2` for side `1`: the table at `0x80B30750`
  holds the two words `0x00000000`, `0x00000002`. `+0x348` is the side byte, so
  this selects the left/right walk clip's attach mode.
* Holder pair: `+0x588` is the animation element, `+0x548` the node it is bound
  over; `+0x544` is the resource handle `0x8023A0B0` searches.
* The two `psq` words are `qr0` `f31` stack spills, not quantised traffic.
* Class evidence: `0x80ACDB00` is reached from the `daEnNetNokoLR_c` state table
  at `0x80B5AAE4` (its first word is `0x80ACDE00`, the side-flip step), and the
  name pool at `0x80B5AB18` starts with `daEnNetNokoLR_c::StateID_Move` /
  `..._Turn`. `"net_walk2"` is the walk clip of that class; the neighbouring
  `"net_walk1"` / `"net_turn_st"` / `"net_turn_ed"` strings belong to the same
  pool.

### Constants

| address | value | use |
|---|---|---|
| `0x80B5A7FC` | `"net_walk2"` | clip looked up on `+0x544` |
| `0x80B30750` | `{0x00000000, 0x00000002}` | per-side attach mode (`+0x348`) |
| `0x80B30758` | `1.0f` | descriptor blend |
| `0x80B30748` | `0.0f` | frame supplied by the spawn-site caller |
| `+0x544` / `+0x548` / `+0x588` | handles | resource / node / element |

### Verdict: conditional

Everything in the body is literal. The one bounded step is the **`f1` width**
plus wiring the frame through `fn_80acde00`; once that is decided the body is
byte-exact.

---

## 2. `0x80A03C30` - `daEnBuku_c` 24-frame stage-effect timer

| field | value |
|---|---|
| TSV size | 92 bytes |
| reachable | 23 instructions, ends at `0x80A03C88` (`b 0x80078470`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80a03c30_FUN_80a03c30.c` |
| signature | `pub unsafe fn fn_80a03c30(this: usize)` |
| verdict | **ready** |

### What the stream does

```c
void fn_80a03c30(usize this)
{
    if (*(i32*)(this + 0x69C) != 0)
        *(i32*)(this + 0x69C) -= 1;
    if (*(i32*)(this + 0x69C) != 0)
        return;                                     // bnelr

    *(i32*)(this + 0x69C) = 24;

    f32 x = *(f32*)(this + 0x688)
          + ((f32*)0x80B2A9A8)[*(u8*)(this + 0x348)];
    f32 y = *(f32*)(this + 0x68C);
    u32 manager = *(u32*)0x80429DD0;

    // tail b 0x80078470, five-argument effect request shape
    0x80078470(x, y, manager, 3, 1);                // r6 still holds 24
}
```

### Notes that matter for a faithful port

* No prologue at all: the body starts with `lwz r4,0x69C(r3)`. The export's
  `halt_baddata` is the tail `b 0x80078470` at `0x80A03C88`, not a bad word.
* The timer is a **signed 32-bit** word at `+0x69C`, decremented only when
  non-zero, then re-read; the second read is a new load (`lwz`) and can differ
  from the first if the decrement wrapped - it cannot, because of the guard.
* `+0x688` and `+0x68C` are the effect anchor. The side byte `+0x348` indexes a
  **float** table: entry 0 = `32.0f`, entry 1 = `-32.0f`, so the request is
  `(anchor_x +- 32, anchor_y)`.
* `r6 = 24` is loaded with `li r6,24` for the `stw r6,0x69C(r3)` reload and is
  **not** overwritten before the tail branch, so the callee receives
  `r6 = 24`. Every other `0x80078470` site in this module uses the five-argument
  shape `(f1, f2, r3, r4, r5)`; `r6` is not part of that contract but its value
  here is a known literal. The faithful port passes it, e.g. through a
  `extern "C" fn(f32, f32, u32, u32, u32, u32)` adapter with
  `(x, y, manager, 3, 1, 24)`. Passing 24 cannot be wrong; dropping it would
  only be right if the callee provably ignores `r6`, which no export states.
* `0x80429DD0` is the stage/height manager pointer (not `0x80439DD0`).
* Class evidence: `0x80A03C30` lies in the `0x80A038B0..0x80A05xxx` `daEnBuku_c`
  (bakubaku) block; wave 12 records `0x80A05DF0` as a `daEnBuku_c` container
  helper and the existing stub comment at `enemies.rs` calls this row the
  "`daEnBuku_c` frame tail step".

### Constants

| address | value | use |
|---|---|---|
| `+0x69C` | `i32` | frame timer (24 on each fire) |
| `+0x688` / `+0x68C` | `f32` | anchor x / y |
| `+0x348` | `u8` | side index into the `32.0` / `-32.0` table |
| `0x80B2A9A8` | `{32.0f, -32.0f, 0.00390625f, 42.0f, 44.0f, ...}` | x offset table |
| `0x80429DD0` | `u32` | stage manager (runtime pointer) |
| `3` / `1` / `24` | literals | effect layer / count / `r6` |

### Verdict: ready

All four operands of the tail request are literal, the table entry is a known
float, and the `r6` value is a known constant. Write the six-register adapter
and the row is byte-exact.

---

## 3. `0x80AF3850` - burst cannon steer-target snap to a multiple of 800

| field | value |
|---|---|
| TSV size | 88 bytes |
| reachable | 22 instructions, ends at `0x80AF38A4` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80af3850_FUN_80af3850.c` |
| signature | `pub unsafe fn fn_80af3850(this: usize) -> i32` |
| verdict | **ready** |

### What the stream does

```c
i32 fn_80af3850(usize this)
{
    i16 v = (i16)fn_80AF38B0(this);          // guide angle, clamped to +-13500
    i16 adj = (v < 0) ? (i16)(v - 400) : (i16)(v + 400);
    i32 q = trunc(adj / 800);                // 0x51EB851F magic + srawi 8 + sign fix
    return (i32)(i16)(q * 800);
}
```

Instruction sequence (`0x80AF385C..0x80AF3894`):

```
bl 0x80AF38B0              ; v = (s16)r3
extsh. r4,r3               ; CR0 from v (r4 is otherwise dead)
addi  r0,r3,-400
extsh r0,r0                ; adj = (s16)(v - 400)          (v < 0)
blt   0x80AF3878
addi  r0,r3,400
extsh r0,r0                ; adj = (s16)(v + 400)          (v >= 0)
0x80AF3878:
lis   r3,0x51EC
addi  r3,r3,-31457         ; r3 = 0x51EB851F
mulhw r0,r3,r0             ; high word of magic * adj
srawi r0,r0,8              ; q = product >> 40
rlwinm r3,r0,1,31,31       ; 1 when q < 0  (bit 30 of a negative word)
add   r0,r0,r3             ; q += (q < 0)
mulli r0,r0,800
extsh r3,r0                ; return (s16)(q * 800)
```

### Notes that matter for a faithful port

* The 800 divisor is exact. The magic `0x51EB851F = ceil(2^40 / 800)` plus the
  `srawi 8` gives `floor(adj * magic / 2^40)`, and the `rlwinm r3,r0,1,31,31`
  adds 1 for every negative `q`, so `q = trunc(adj / 800)` (toward zero).
  Verified by simulating the four instructions over the whole reachable input
  range `[-13500, 13500]`: 0 mismatches against
  `(s16)(800 * Math.trunc(adj / 800))`.
* The function is therefore a **round-to-nearest-multiple-of-800 with ties away
  from zero**: `v = -1200 -> -1600`, `v = -1199 -> -800`, `v = 0 -> 0`,
  `v = 399 -> 0`, `v = 400 -> 800`, `v = 1200 -> 1600`, `v = 13500 -> 13600`.
* `0x80AF38B0` is already translated (`fn_80af38b0`): it reads `this+0x778`,
  returns `0` for `0xFFFFFFFF`, otherwise `(i16)(0x800B3980(...) scale)` clamped
  to `[-13500, 13500]`. Its return arrives in `r3`, so the `extsh` is exact.
* `extsh. r4,r3` is a **dead** copy: `r4` is never read again. Only CR0
  matters.
* Caller (`fn_80af3d60`, burst state `3`): `r = fn_80af3850(this)`;
  `*(s16*)(this+0x7A6) = (s16)r` (the steer **target**), then
  `fn_80af3690(this, (s16)r)`. The result is a world-map steer target in the
  same `s16` angle units as `+0x7A4`.
* Field context: `+0x7A4` current steer angle, `+0x7A6` steer target, `+0x360`
  uint state, `+0x778` controller-id ring (per the wave-57 `Wm_en_burst_m`
  field map).

### Constants

| address | value | use |
|---|---|---|
| `0x51EB851F` | magic | `adj / 800` (shift `40`) |
| `400` | literal | rounding bias (`-400` / `+400`) |
| `800` | literal | snap period |
| `0x80AF38B0` | translated | angle source (`+0x778`, clamp `+-13500`) |

### Verdict: ready

The whole body, including the reciprocal-multiply rounding that wave 136 left
open, is now decoded and verified; the only callee is translated.

---

## 4-8. The five `0x802DCBFC` this-adjust thunks

All five are 48 bytes / 12 instructions, share one instruction sequence, and
differ only in the **descriptor offset** carried in `r12` and in the vtable slot
that references them. They are destructor wrappers: their job is to dispatch the
pointer-to-member stored inside the object to the base-class destructor.

| # | address | `r12` offset | vtable reference(s) |
|---:|---|---:|---|
| 4 | `0x80A25F00` | `+0x24` | `0x80B3ABE8` (slot `+8` of the table at `0x80B3ABE0`, kind pair `0x025B`/`0x026B`) |
| 5 | `0x80A2F9D0` | `+0x18` | `0x80B3C74C` (slot `+0x1C` of `0x80B3C730`) and `0x80B3C780` (slot `0` of the second table) |
| 6 | `0x80A2FA00` | `+0x24` | `0x80B3C750` (slot `+0x20` of `0x80B3C730`) and `0x80B3C784` (slot `1`) |
| 7 | `0x80A43EE0` | `+0x18` | `0x80B406E8` (slot `+0x18` of `0x80B406D0`, class `Wm_ob_coin`, kind pair `0x0192`/`0x019C`) |
| 8 | `0x80A43F10` | `+0x24` | `0x80B406EC` (slot `+0x1C` of `0x80B406D0`) |

Class identification: `0x80B3C7A4..0x80B3C7C0` holds the `daEnBubble_c` name
pool (`env_underwater`, `wmy_ef_bubble`, `ef_bubble`, `...rwater.brres`), and
the table at `0x80B3C730` sits in the `0x80A2F6xx..0x80A2FAxx` bubble block.
`0x80B406FC` is the literal string `"Wm_ob_coin"`, so rows 7 and 8 are the
world-map coin object's vtable slots 6 and 7.

### What the stream does (identical for all five)

```c
// e.g. 0x80A25F00
void fn_80a25f00(u32 obj, u32 receiver)
{
    // hidden r12 = obj + 0x24; 0x802DCBFC reads its 12-byte descriptor there
    r12 = obj + 0x24;
    r3  = receiver;
    r4  = receiver;        // the same value; 0x802DCBFC only reads r12
    r5  = obj;
    0x802DCBFC();          // dispatches the PMF on receiver
}
```

Raw words (`0x80A25F00`, offset +0x24; `0x80A2F9D0`/`0x80A43EE0`, offset
`+0x18`; `0x80A2FA00`/`0x80A43F10`, offset `+0x24`):

```
stwu  r1,-16(r1)
mfspr r0,lr
or    r5,r3,r3          ; r5 = first argument
or    r3,r4,r4          ; r3 = second argument
stw   r0,20(r1)
addi  r12,r5,{0x18|0x24}; hidden descriptor pointer
bl    0x802DCBFC
nop
lwz   r0,20(r1)
mtspr lr,r0
addi  r1,r1,16
blr
```

### Descriptor semantics (from `0x802DCBFC`)

`r12` points at `{i32 this_delta; i32 vtable_index; u32 target}`. The
dispatcher computes `adjusted = receiver + this_delta`; if `vtable_index >= 0`
it replaces `target` with `*(u32*)(*(u32*)(adjusted + target) + vtable_index)`
and then calls `target(adjusted)`. Two consequences for the port:

* the **receiver is the second argument** and the descriptor lives in the
  **first**;
* whether a row is a plain destructor or a deleting destructor depends on the
  descriptor contents (runtime data), not on the thunk's bytes - all five
  thunks are byte-identical apart from the offset.

### Verdict: conditional

The bytes and the registry are exact, but the hidden `r12` operand cannot be
expressed through `platform::call3`. The repo already ships this shape as
`partial` with `call3(FN_802DCBFC, param_2, param_2, param_1)` and the comment
"cannot model the hidden r12 this-adjust pointer" (`fn_80a6c810`,
`fn_80a6d520`, `fn_80a6d550`, ...). Condition: keep that convention and record
the offset (`+0x18` / `+0x24`) per row; if the harness gains an `r12`-capable
adapter, the five become byte-exact immediately. Each section should state which
vtable slot installed it (table above) so the class is recoverable.

---

## 9. `0x80A48940` - `daEnCrasher_c` `+0x674` clamped accumulator

| field | value |
|---|---|
| TSV size | 48 bytes |
| reachable | 12 instructions, ends at `0x80A4896C` (`b 0x801680E0`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80a48940_FUN_80a48940.c` |
| signature | `pub unsafe fn fn_80a48940(this: usize, delta: f32)` |
| verdict | **conditional** |

### What the stream does

```c
void fn_80a48940(usize this, f32 delta)      // f1 = delta
{
    f32 v = *(f32*)(this + 0x674) + delta;
    *(f32*)(this + 0x674) = v;

    if (v >= 1.0f /* 0x80B2C8A4 */)           // fcmpo + cror 2,1,2 + bne
        *(f32*)(this + 0x674) = 1.0f;

    f32 rate = *(f32*)(this + 0x674);         // re-read after the clamp
    0x801680E0(rate, this + 0x584);           // tail b
}
```

### Notes that matter for a faithful port

* The clamp test is `f1 >= f0`, not `>`: `fcmpo cr0,f1,f0`, then
  `cror 2,1,2` (CR0.EQ = CR0.GT | CR0.EQ), then `bne` skips the store. The
  stored value is then re-loaded, so the rate setter always sees the clamped
  number.
* `0x80B2C8A4 = 1.0f` and the caller's delta `0x80B2C984 = 0.125f`
  (`fn_80a49490`, `0x80A4958C`: `lfs f1,0x164(r31)` with
  `r31 = 0x80B2C820`; so `0x80B2C984`). Only one call site exists.
* **Correction to `ANALYSIS_WAVE127_ENEMIES.md`:** the tail target is
  `0x801680E0` (`0x4B71F774` from `0x80A4896C`), the rate setter - not
  `0x80168FE0` as that table says. The stored value is a rate, not a frame.
* The accumulator `+0x674` is a single-precision float; the stream adds with
  `fadds` and stores with `stfs`.
* The existing translated caller (`fn_80a49490`, `enemies.rs:128621`) uses
  `call2_f64(W109_FN_80A48940, read_f32(W109_DAMAGE_STEP) as f64, actor)`.
  That promotes the delta to a double; the raw call passes the `lfs` single in
  `f1` and the body consumes it with `fadds`. Same width question as section 1.
* Class evidence: `fn_80a49490` (the only caller) is slot 0 of the state table
  at `0x80B42BA4`, whose name pool at `0x80B42C44` starts
  `daEnCrasher_c::StateID_Walk`; wave 109 already calls this row the
  "attack-walk `+0x674` clamp step".

### Constants

| address | value | use |
|---|---|---|
| `+0x674` | `f32` | accumulator / attack-walk phase |
| `+0x584` | holder | rate target of the tail call |
| `0x80B2C8A4` | `1.0f` | ceiling |
| `0x80B2C984` | `0.125f` | the caller's per-frame delta |
| `0x801680E0` | translated | rate setter (tail) |

### Verdict: conditional

All operands are literal. The bounded step is the `f1` width (`f32` in the
stream, `f64` in the existing call site); after that the body is byte-exact.

---

## 10. `0x80AF0370` - `daEnRedRing_c` 60-frame rumble start

| field | value |
|---|---|
| TSV size | 44 bytes |
| reachable | 11 instructions, ends at `0x80AF0398` (`b 0x800E41C0`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80af0370_FUN_80af0370.c` |
| signature | `pub unsafe fn fn_80af0370(this: usize)` |
| verdict | **ready** |

### What the stream does

```c
void fn_80af0370(usize this)
{
    u32 port  = (u8)(*(u8*)(this + 0x355) - 1);
    u32 mask  = *(u32*)0x8042A078;       // pointer to the two-word mask table
    // tail b 0x800E41C0: seven arguments
    0x800E41C0(mask, port, 60, 1, 0, 1, 0);
}
```

Register staging, in order (`0x80AF0370..0x80AF0398`):

```
lbz   r4,0x355(r3)     ; +0x355 byte (1-based port index)
lis   r6,0x8043
lwz   r3,-24456(r6)    ; r3 = *(u32*)0x8042A078
li    r5,60
addi  r0,r4,-1
li    r6,1
li    r7,0
li    r8,1
rlwinm r4,r0,0,24,31   ; r4 = (u8)(byte - 1)
li    r9,0
b     0x800E41C0
```

### Notes that matter for a faithful port

* The export is a bare `halt_baddata()` only because the function ends in a tail
  branch; the eleven words above are the whole body (wave 136 left it open for
  that reason).
* `0x8042A078` is a **pointer** to the two-word rider mask table, not the table
  itself (it is read through `lwz`). Its value is runtime state.
* Seven arguments: `r3 = mask pointer`, `r4 = (u8)(portbyte-1)`, `r5 = 60`,
  `r6 = 1`, `r7 = 0`, `r8 = 1`, `r9 = 0`. Wave 17 records `0x800E41C0` as a
  seven-argument input/rumble helper; the argument *roles* beyond the first two
  are inferred there, but the values here are all literals, so the call is
  reproducible either way.
* Caller `fn_80af0440` guards it: `if (*(u8*)(this+0x355) != 0) fn_80af0370(this)`
  after a `0x800B1100(*(u32*)0x80429F30, *(u8*)(*(u32*)0x8042A1C8 + 0x120E), 1,
  f1 = x, f2 = y)` sound request - so the byte is a 1-based player/port index
  and the function is only reached for a real port. The `-1` inside the callee
  converts it to the 0-based port the helper wants.
* Class evidence: `fn_80af0440` is one of the handlers listed by the state table
  whose `0x80AF0440` word sits at `0x80B62648`; that table's name pool at
  `0x80B626AC` is `daEnRedRing_c::StateID_Wait/Vanish/Kill`.

### Constants

| address | value | use |
|---|---|---|
| `+0x355` | `u8` | 1-based port/player index |
| `0x8042A078` | `u32` | pointer to the two-word rider mask table (runtime) |
| `60` | literal | rumble duration |
| `1, 0, 1, 0` | literals | `r6..r9` |

### Verdict: ready

Eleven literal instructions, one tail call, one runtime pointer. The
`0x800E41C0` role is documented (wave 17/143) and the values are exact.

---

## 11. `0x80A933E0` - `block_cloud` splash request (factory id `0x54`)

| field | value |
|---|---|
| TSV size | 36 bytes |
| reachable | 9 instructions, ends at `0x80A93400` (`b 0x80064610`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80a933e0_FUN_80a933e0.c` |
| signature | `pub unsafe fn fn_80a933e0(out: usize, field_a: u32, field_b: u32) -> usize` |
| verdict | **ready** |

### What the stream does

```c
usize fn_80a933e0(usize out, u32 a, u32 b)
{
    u32 packed = ((a & 0xF) << 20) | ((b & 0xF) << 16);
    return 0x80064610(0x54, packed, out, 0, 0);   // tail b
}
```

```
or     r6,r3,r3          ; r6 = out
rlwinm r0,r5,16,0,15     ; r0 = (b << 16) & 0xFFFF0000
rlwinm r4,r4,20,0,11     ; r4 = (a << 20) & 0xFFF00000
li     r3,84             ; id = 0x54
or     r5,r6,r6          ; r5 = out
li     r6,0
or     r4,r4,r0
li     r7,0
b      0x80064610
```

### Notes that matter for a faithful port

* The factory shape is the module's standard five-argument request
  `(id, resource, out, 0, 0)`; `0x80064610` returns the spawned object in `r3`,
  which this row returns unchanged.
* The two nibble fields are packed into the `resource` word: `a` into bits
  20-23 and `b` into bits 16-19, then the `resource` word is passed with the
  top bits zero.
* The only caller (`fn_80a1fa70`, `0x80A1FBF0`) passes
  `out = &position[0]` with `position[2]` replaced by the height probe and
  `position[1] += 8.0f`, and `a = kind & 0xF`, `b = (kind >> 4) & 0xF` where
  `kind = *(u32*)(this+4)`. The existing call site in `enemies.rs`
  (`call3(W133_FN_80A933E0, position.as_mut_ptr(), flags & 0xF, (flags >> 4) & 0xF)`)
  matches this exactly, so the signature above is already in use.
* Class evidence: the `fn_80a1fa70` port (wave 133) calls this address the
  `block_cloud` splash emitter and writes `+0x7AC = 4` immediately before the
  request; the address itself had no body until now.

### Constants

| address | value | use |
|---|---|---|
| `84` (`0x54`) | literal | factory id |
| `0xF` / `0xF` | masks | the two 4-bit fields |
| `0x80064610` | `main.dol` export | five-argument factory, returns the object |

### Verdict: ready

Nine literal instructions; the factory contract and the packed-word layout are
established by both the stream and the existing caller.

---

## 12. `0x80B14D80` - thrown-object request (factory id `0x39`)

| field | value |
|---|---|
| TSV size | 28 bytes |
| reachable | 7 instructions, ends at `0x80B14D98` (`b 0x80064610`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80b14d80_FUN_80b14d80.c` |
| signature | `pub unsafe fn fn_80b14d80(out: usize, mode: u32) -> usize` |
| verdict | **ready** |

### What the stream does

```c
usize fn_80b14d80(usize out, u32 mode)
{
    u32 resource = ((mode & 0x1113) | 0x8000);
    return 0x80064610(0x39, resource, out, 0, 0);   // tail b
}
```

```
andi.  r0,r4,0x1113      ; r0 = mode & 0x1113 (CR0 is set but never read)
or     r5,r3,r3          ; r5 = out
li     r3,57             ; id = 0x39
li     r6,0
li     r7,0
ori    r4,r0,0x8000
b      0x80064610
```

### Notes that matter for a faithful port

* Same factory shape as row 11. The `andi.` sets CR0 but nothing consumes it, so
  it is a plain mask in practice.
* The caller (`fn_80a94eb0`, `0x80A94F30..0x80A94F40`) passes
  `out = this+0xAC` (the actor position) and
  `mode = ((*(u32*)(this+4) & 3) << 4) | 1`, i.e. `0x1`, `0x11`, `0x21` or
  `0x31`; after masking, `mode & 0x1113` is `0x1`, `0x11`, `0x21` or `0x11`, so
  the `resource` word is `0x8001`, `0x8011`, `0x8021` or `0x8011`. The existing
  call site (`call2(FN_80B14D80, (actor+0xAC), mode)`) matches.
* `0x80064610` returns the spawned object; the caller immediately writes
  `*(u32*)(obj+0x9D4) = *(u32*)actor`, `*(u8*)(obj+0x9D8) = 1` and
  `*(u32*)(actor+0x7F4) = *(u32*)obj`. That is why this must be typed as
  returning the object pointer.
* **Correction to the existing stub comment** at `enemies.rs:41094`
  ("finds a player object near a position"): the stream is a factory request
  with id `0x39` and a `resource` word derived from the actor kind; it is the
  same `0x80064610` path used by `0x80A443A0` (id `0x192`).
* Byte-identical sibling in shape: `0x80A443A0` (24 bytes,
  `0x80064610(0x192, 0, r5, 0, 0)`), already recorded as class B in
  `ANALYSIS_WAVE127_ENEMIES.md`.

### Constants

| address | value | use |
|---|---|---|
| `57` (`0x39`) | literal | factory id |
| `0x1113` | mask | keeps bits 0, 1, 4, 8, 12 of `mode` |
| `0x8000` | literal | sets bit 15 of `resource` |
| `0x80064610` | `main.dol` export | five-argument factory |

### Verdict: ready

Seven literal instructions and a caller whose argument construction is already
in the repo.

---

## Corrections this page makes to earlier docs

1. `ANALYSIS_WAVE127_ENEMIES.md` row `0x80A48940` names `0x80168FE0` as the tail
   callee. The stream is `b 0x801680E0` (`0x4B71F774`), the rate setter.
2. `COVERAGE_WAVE154_ENEMIES.md` skips `0x80ACDB00` because "the existing caller
   calls it with only `r3`, so a body would have to invent the blend argument".
   Both raw call sites supply the argument (`0x80168080` return at
   `0x80ACDE80`, `0.0f` at `0x80ACDA9C`), so nothing is invented; the caller
   line needs one edit.
3. `COVERAGE_WAVE154_ENEMIES.md` skips `0x80A03C30` over "whether `0x80078470`
   reads `r6`". The register holds the literal `24` at the branch, so the call
   is exact either way; the remaining choice is whether the port adapter carries
   a sixth argument.
4. `COVERAGE_WAVE136_ENEMIES.md` skips `0x80AF0370` because "both exports are
   pure `halt_baddata`". The export is truncated at the tail branch only; the
   eleven words decode to a complete seven-argument request.
5. `COVERAGE_WAVE127_BASES.md` calls the 48-byte `0x802DCBFC` thunks class D
   "genuinely opaque". Their bodies are fully readable; only the hidden `r12`
   descriptor prevents a Rust expression, which is why rows 4-8 are
   `conditional` rather than unclaimable.

## Verification

* 12 addresses, all absent from `ledger/functions.csv`, from every `^#`
  heading address in `docs/*.md`, and from every `fn fn_<addr>` body under
  `src/` at the snapshot; each has a `decompiled = true` TSV row and an existing
  export file at the cited path.
* TSV sizes sum to 724 bytes; reachable counts sum to 193 instructions.
* For every row, the last instruction of the walk is exactly
  `entry + size - 4`; no path leaves the extent and no `bctr` appears in the
  set.
* The only `psq` words are `0x80ACDB00`'s two `qr0` `f31` spills. The set
  contains no `qr3` site, so `GQR3 = 0x00050005` is not exercised here.
* The 800-divisor sequence in section 3 was verified by simulating
  `mulhw`/`srawi`/`rlwinm`/`add`/`mulli`/`extsh` against
  `trunc(adj / 800)` for every `v` in `[-13500, 13500]` (0 mismatches).
* No Rust file, ledger CSV or existing document was modified by this pass.
