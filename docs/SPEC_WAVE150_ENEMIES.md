# Wave 150 - implementation specification for 12 untranslated `d_enemiesNP` bodies

## What this page is

`docs/ANALYSIS_WAVE127_ENEMIES.md` reclassified 384 formerly-skipped `d_enemiesNP`
addresses into 173 class **B**, 160 class **C** and 51 class **D**. Waves 128-149
worked through that block and left the size-ranked tranche at 300 bytes. This
page takes the twelve largest bodies that are **still untranslated**, have a
Ghidra export, and are fully walkable from their entry.

A live scan of `work/ghidra/rel-d_enemiesNP/export/functions.tsv` (3,555 rows),
the `address` column of `ledger/functions.csv`, every `0x80Axxxxx`/`0x80Bxxxxx`
address in a `^#` heading of any `docs/*.md`, and every definition-shaped
`fn_<addr>`/`FN_<addr>` token in `src/**/*.rs` leaves **80** untranslated bodies
with an export. The twelve sections below are the twelve largest.

Waves 148-149 used a narrower filter that also excluded any address carrying an
`FN_` token, including `W*_FN_` reference stubs; that filter would hide ten of
the twelve rows below. This page follows the wave-150 rule literally - untranslated,
absent from the ledger's `address` column, absent from every spec heading - and
treats a `W*_FN_` constant as a call stub, not a translation.

This page is **read-only**: it writes no Rust, no ledger row and no other doc.

## Selection

The ranking metric is the **reachable body**, not the raw `functions.tsv` extent,
because the TSV size is a bucket extent that can include a neighbour's padding or
an unreachable arm (see `docs/REL_ELF_MAPPING.md`). Each candidate was walked from
its entry through every conditional branch, every internal `b` and every
**linked** indirect branch (`bctrl`, `bclr`/`bcctr` with `LK = 1`), until all
paths ended in `bclr`, a tail `b` or a tail `bcctr`. Conditional `bclr`/`bcctr`
forms were followed on both the taken and the fall-through edge.

| # | address | TSV size | reachable insns | `qr3` sites | role | verdict |
|---:|---|---:|---:|---:|---|---|
| 1 | `0x80A2EC00` | 472 | 118 | 0 | `Wm_en_magmawave` contact band: height clamp, effect spawn, two side sounds | **ready** |
| 2 | `0x80B27ED0` | 432 | 108 | 2 | per-slot relay: scene-object resolve plus angle-driven position/speed | **ready** |
| 3 | `0x80A00D00` | 424 | 106 | 0 | `daEnBuku_c` four-player carry: pull each marked player 2 units, snap at 3 | **ready** |
| 4 | `0x80A04A90` | 348 | 87 | 0 | predicted-position query turned into a normalised speed triple | **ready** |
| 5 | `0x80B16BA0` | 316 | 79 | 0 | node matrix commit from position + three `s16` angles, `wait`/`attack` variant | **conditional** |
| 6 | `0x80AF3400` | 308 | 77 | 2 | player-slot launch: per-slot radius table times sin/cos at speed `1000` | **conditional** |
| 7 | `0x80A37CB0` | 304 | 76 | 0 | nearest player on the actor's side, returns its clip index (or `-1`) | **ready** |
| 8 | `0x80AB13E0` | 304 | 76 | 0 | `fishbone` four-node model binder plus clip/animation setup | **ready** |
| 9 | `0x80A390F0` | 300 | 75 | 0 | `lift_rakka_ashiba` actor initialiser (box, previous positions, scale) | **ready** |
| 10 | `0x80AEE850` | 296 | 74 | 0 | `daEnRailPolyParent_c` constructor (`0x718` bytes, two relation records) | **ready** |
| 11 | `0x80B0CD80` | 292 | 73 | 0 | `daEnTeresa_c` body binder: `wait` / `shay_teresaA` + optional animation | **ready** |
| 12 | `0x80B0CEB0` | 288 | 72 | 0 | Teresa tail binder: `shay_teresaB` / `shay_teresaB_wait` | **ready** |

**Split: 10 ready, 2 conditional.** Total 4,084 bytes of TSV extent and 1,021
reachable instructions. Every one of the twelve walks to a terminal instruction
at exactly `entry + size - 4`, so `reachable == extent` for all twelve and the
"extent is padding" caveat does not apply here. The size cut is clean: 304 bytes
is claimed twice (ranks 7 and 8), 288 is the last claimed size, and the next row
is 284.

### Why ranks 1-4 and 11-12 were not already covered

Waves 147-149 listed several of these rows as "next in size" but never claimed
them, and wave 148 (`docs/SPEC_WAVE148_ENEMIES.md:17-19`) asserted that
`0x80A2EC00` and `0x80B27ED0` are "translated in `src/game/enemies.rs` without a
ledger row". That assertion is **false** and is corrected here:

* `0x80A2EC00` exists in `src/game/enemies.rs` only as
  `const W123_FN_80A2EC00: usize = 0x80A2_EC00`, a call stub whose own doc
  comment calls the four-body run "untranslated".
* `0x80B27ED0` exists only as `const W137_FN_80B27ED0` under the comment
  "untranslated 17-slot relay body (GQR row)".

The same holds for `0x80A00D00` (`W71_FN_...`, "untranslated"), `0x80A04A90`
(`W146_FN_...`), `0x80B16BA0` (`W146_FN_...`), `0x80AF3400`
(`BURST_FN_...`), `0x80A37CB0` (`W117_FN_...`), `0x80AB13E0`
(`W129_FN_...`), `0x80B0CD80` (`W135_FN_...`/`W138_FN_...`) and
`0x80B0CEB0` (`W132_FN_...`). A `W*_FN_` constant is a reference stub, not a
translation, so all twelve are genuine work items.

### Rows below the cut

| address | TSV size | reachable | why it is not claimed |
|---|---:|---:|---|
| `0x80A638F0` | 284 | 71 | clean `daEnGesso_c` frame step; next in size order, left for wave 151 |
| `0x80A63A30` | 284 | 71 | the mirror twin of the row above (x `+0.5` instead of `-0.5`) |
| `0x80AEAED0` | 284 | 71 | clean trail-segment loop; only `f31` qr0 spill |
| `0x80A965D0` | 280 | 70 | clean per-slot child activation; `pk`-model holder rebuild |
| `0x80B16A80` | 276 | 69 | clean eye/mouth node offset step; two `strcmp` names |
| `0x80A80F40` | 264 | 66 | one `qr3` site; sway/heading step with 208..768 phase clamp |
| `0x80B1AD30` | 264 | 66 | ground-snap step; final callee `0x800A0A30` uncharacterised |
| `0x80A5E590` | 252 | 63 | `daEnGabon_c` initialiser; second argument of `0x80A5E1A0` unused |

## The `qr3` pool at this wave

`docs/REL_ELF_MAPPING.md` pins **`GQR3 = 0x00050005`** (ST/LD type 5 =
unsigned 16-bit, both scales 0), installed by `main.dol` at
`0x8025EEEC`/`0x8025EEF0`, and records that the RAW stream of this REL contains
no `mtspr GQRn`. Among the twelve bodies two read `qr3`:

| address | size | `qr3` sites | where |
|---|---|---:|---|
| `0x80B27ED0` | 432 | 2 | `psq_l f1,0(r1+0x0A)` at `0x80B27FD4`, `psq_l f1,0(r1+0x08)` at `0x80B27FF4` |
| `0x80AF3400` | 308 | 2 | `psq_l f1,0(r1+0x0A)` at `0x80AF348C`, `psq_l f1,0(r1+0x08)` at `0x80AF34B0` |

Both loads read a single `s16` angle that was just stored with `sth`; with
`GQR3 = 0x00050005` the recovered value is exactly `(f32)(u16)angle`, then
multiplied by the `1/256` word at `0x80B32C78`/`0x80B313D4` and handed to the
table sine/cosine pair. The high lane of each pair load reads the untouched
neighbour halfword and is dead. Nothing on this page depends on a non-zero
quantiser scale, and no other body in the twelve uses `qr3`; all other
`psq_l`/`psq_st` words are `qr0` `f31`/`f30`/`f29`/`f28` stack spills beside
matching `stfd`/`lfd`.

## Reading conventions

* Synthetic base is `0x80A00000`. The REL has five file-backed `PT_LOAD`
  segments (`0x80A000F0`, `0x80B2A518`, `0x80B2A7A4`, `0x80B2A7B0`,
  `0x80B32DC0`); `docs/REL_ELF_MAPPING.md`'s `+0x1000` formula applies to the
  first. The file-backed image ends at `0x80B6E438`, so anything at or above the
  zero-fill segment `0x80B6E500` is **BSS** and is marked **(ext)** below.
* `this` is the incoming `r3` unless stated otherwise; `+0xNNN` are byte offsets
  from `this`. `+0xAC`/`+0xB0`/`+0xB4` are x/y/z; `+0x100`/`+0x102`/`+0x104`
  are the three `s16` angles; `+0x348` is the side/facing byte; `+0x60` is the
  class vtable; `+0xD0`/`+0xD4`/`+0xD8` are the three velocity components;
  `+0xE8`/`+0xEC`/`+0xF0` are the three speed components.
* `addi rD,r0,imm` (A field `0`) is `li`; `addis rD,r0,imm` is `lis`. The base is
  the literal zero, not `r0`. `addi r0,rS,imm` is a **real** add into `r0`.
  A negative `addi` after a `lis` subtracts from the full `lis` value; this is
  how `0x80B2C240`, `0x80B2D150`, `0x80B2D2B0`, `0x80B31150` and `0x80B32440`
  are formed.
* `rlwinm rA,rS,sh,mb,me` is `ROTL32(rS,sh) & mask(mb..me)` with mask bits
  numbered **from the MSB** (bit 0 = `0x80000000`, bit 31 = `1`). A trailing
  `.` (`rlwinm.`) also sets `CR0` from the result; the `rlwinm.` forms at
  `0x80AEAED0`, `0x80A965D0` and `0x80B1AD30` are the branch selectors.
* On a `bc`, `bo = 12` branches on a **true** CR bit and `bo = 4` on a **false**
  one; `bi & 3` is LT/GT/EQ/SO. `cror cr0.eq,cr0.lt,cr0.eq` (XO 449) rewrites
  `cr0.eq` as `lt|eq`, so the following `bne` reads `if !(a <= b)`;
  `cror cr0.eq,cr0.gt,cr0.eq` gives `gt|eq`.
* Single-precision float shares its encoding with the Gecko paired-single
  arithmetic opcodes: `fadds`/`fsubs` take `fB`, while `fmuls` and the
  `fmadds`/`fmsubs` family take `fC`. Only `psq_l`/`psq_st` (opcodes 56/57 and
  60/61) are true paired-single memory ops; every other "`ps_`" spelling in an
  export is an `fadds`/`fsubs`/`fmuls`.
* `0x4E800421` is `bctrl` (`LK = 1`): an indirect call that **returns**. Only
  `0x4E800420` (LK = 0) is a tail `bcctr`. Likewise `bnelr` returns on the
  taken edge and falls through on the other.
* `0x43300000` in the high word with the value `^ 0x80000000` in the low word,
  minus the double at `0x80B2D458`/`0x80B30978`/`0x80B31AA0` (`2^52`), is the
  `u32 -> f64` conversion. `fctiw`/`fctiwz` truncate toward zero; the bodies
  below use `fctiwz` everywhere.

## Symbolic helpers used below

| address | symbol | contract used here |
|---|---|---|
| `0x8005F900` | `player(i) -> actor` | player object for slot `i`, `0` when absent |
| `0x80064740` | `near_actor(actor, &out) -> actor` | predicted-position neighbour query; `0` when none |
| `0x8006A4B0` | `vec_from_pos(&out, pos)` | copies a three-float position |
| `0x8006CD40` | `integrate(actor)` | shared per-frame motion integrate |
| `0x8006CDE0` / `0x800955F0` | `base_update(actor)` | shared per-frame prelude pair |
| `0x80075270` | `ground(x, y, kind, &out) -> u32` | ground/height query, non-zero on a hit |
| `0x80078410` | `sound(global, id, x[, y])` | `main.dol` sound/effect entry (register order per section 1) |
| `0x80096240` | `face_variant(actor, &pos) -> u32` | archive/area variant byte |
| `0x80096E60` | `input_bits(actor) -> u32` | held-input bit query |
| `0x80097800` | `pad_query(actor, range) -> u32` | non-zero when inside the queried range |
| `0x800B2EE0` | `se_len(id) -> u32` | sound/effect helper, value used as a countdown seed |
| `0x800BC070` | `rotate(holder, &in, &out)` | per-slot local-to-world rotation |
| `0x800EB930` | `nearest_record(global, &pos, &a, &b, &c, kind) -> u32` | nearest matching record scan; kind byte in `r3` |
| `0x8013FAA0` | `attach(player, src)` | uncharacterised attach/copy (section 6) |
| `0x80146E20` | `motion(player, id, f1, f2)` | stores two motion values and refreshes `+0x348` |
| `0x8015F4F0` | `step_angle(&s16, target, step) -> u32` | shortest-path angle step, non-zero on arrival |
| `0x80162D00` | `scene(id) -> object` | scene-id resolve, `0` when absent |
| `0x801651F0` | `attach_anim(holder, node, &desc, mode)` | files an animation element under a holder |
| `0x80165C30` | `build_anim(layout, handle, name, a, b, c)` | animation group builder |
| `0x80166BD0` | `apply_anim(holder, node, &name, a, arg)` | applies one animation |
| `0x80166D10` | `stop_anim(holder, 0)` | stops the animation at the holder |
| `0x80166EF0` / `0x80166F10` | `set_anim(rate, holder, 0)` | sets one animation value |
| `0x801680E0` | `advance_anim(holder, rate)` | advances the animation interface |
| `0x80168180` | `start_anim(node, second, &out)` | starts one animation on the node |
| `0x80168FE0` | `release_anim(node)` | releases the node's animation |
| `0x80169A..` | — | see the individual sections |
| `0x80169CD0` | `init_node(node)` | constructs one `0x40`-byte matrix sub-object |
| `0x80169D90` | `init_matrix(node, &desc, owner, kind, a, b)` | matrix-node initialiser |
| `0x80169F80` | `attach_desc(f, node, desc)` | attaches the descriptor when its guard passes |
| `0x8016A150` / `0x8016A170` / `0x8016A1A0` | `copy_xform` / `mark_dirty` | layout-node transform copies |
| `0x8016C890` | `spawn_effect(name, 0, &vec, 0, 0)` | named 2-D effect spawn |
| `0x8016ECB0` / `0x8016EDD0` / `0x8016EEF0` | `set_rot_x/y/z` | per-axis `s16` rotation on a node |
| `0x80170A00` | `normalise(&vec3)` | normalises a three-float vector |
| `0x80197F00` | `player_se(manager, 464, &vec, 0)` | player sound-effect spawn |
| `0x801C0BD0` | `translation(out, x, y, z)` | builds a translation matrix from three floats |
| `0x80237BD0` / `0x80237C40` | `sin256(x)` / `cos256(x)` | table sine/cosine of an angle in 1/256-turn units |
| `0x8023A0B0` / `0x8023A200` | `node(model, name)` | resolves a named model node |
| `0x8023B300` | `node_by_index(model, i)` | resolves a model node by index |
| `0x802418B0` / `0x80241BE0` | `read_pos(target, &out3)` / `write_pos(target, &in3)` | three-float position accessors |
| `0x80241A00` | `write_scale(target, &in3)` | scale vector writer |
| `0x802E1AB4` | `strcmp(a, b) -> u32` | `0` when equal |
| `0x802DCD60`/`6C`/`68`/`70`/`74` | `_savegpr_N` | prologue save helpers; `r3` survives |
| `0x802DCDAC`/`B4`/`B8`/`BC`/`C0` | `_restgpr_N` | epilogue restore helpers |

## 1. `0x80A2EC00` - `Wm_en_magmawave` contact band

* **Address:** `0x80A2EC00`
* **TSV size:** 472 bytes; reachable body 118 instructions = the whole extent
  (one `blr` at `0x80A2EDD4`).
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a2ec00_FUN_80a2ec00.c`
  - one `qr0` `f31` spill only (`0x80A2EC10`/`0x80A2EDBC`).
* **Proposed signature:** `pub unsafe fn fn_80a2ec00(this: usize)`
* **Callers in this REL:** one `bl` at `0x80A2F150`; also the
  `W123_FN_80A2EC00` call site in `src/game/enemies.rs`.
* **Constants:**

| address | value | role |
|---|---|---|
| `0x80B2BC8C` | `6500.0` | z of the effect position |
| `0x80B3C378` | `"Wm_en_magmawave"` | effect name |
| `0x8042A100` | word | record-table pair high word for `0x800EB930` |
| `0x80429DD0` | word | course/event manager; `+0x9002C` is a mode byte, `-0x13C` a height word |
| `0x80429D5C` | word | player-SE manager for `0x80197F00` |

* **Fields:** `+0xAC` x, `+0xB0`/`+0xBC` the two band heights, `+0x38F` the
  record kind byte.
* **Verdict:** **ready** (confidence medium).

### Behaviour

```
flags = 0
lo    = min(f32[this+0xB0], f32[this+0xBC])
p     = { f32[this+0xAC], lo }
kind  = (s8)nearest_record(u32[0x8042A100], &p, &outA, &outB, &cand,
                           u8[this+0x38F])
if (kind == 1) flags |= 1

mgr = u32[0x80429DD0] + 0x90000
if (u8[mgr+0x2C] == 3) {
    f31 = f32[mgr-0x13C]
    flags |= 2
}

if      (flags == 2) cand = f31
else if (flags == 3 && cand <= f31) cand = f31     # max(cand, f31)

if (flags == 0) return

if ((f32[this+0xBC] <= cand && cand < f32[this+0xB0]) ||
    (f32[this+0xB0] <= cand && cand < f32[this+0xBC])) {
    vec = { f32[this+0xAC], cand, 6500.0 }
    spawn_effect("Wm_en_magmawave", 0, &vec, 0, 0)
}

if (f32[this+0xBC] <= cand && cand < f32[this+0xB0]) {
    sound(u32[0x80429DD0], 17, f32[this+0xAC])
} else {
    sound(u32[0x80429DD0], 16, f32[this+0xAC], f32[this+0xB0])
    vec2 = { 0, 0, 0 }
    vec_from_pos(&vec2, this+0xAC)
    player_se(u32[0x80429D5C], 464, &vec2, 0)
}
```

### Notes

* The band test is written as two exclusive arms because either of `+0xB0` and
  `+0xBC` may be the lower bound; together they are exactly
  `min(+0xB0,+0xBC) <= r5 < max(+0xB0,+0xBC)`.
* `0x800EB930` returns the record's kind byte in `r3` (the three stack outputs
  match `src/engine/dsys.rs`'s "nearest matching record"); only the `== 1` case
  is used.
* `0x80078410` is documented in `src/game/bases.rs` as a `main.dol` sound call,
  but the C prints its arguments in a different order. The register order above
  is the raw one (`r3 = manager`, `r4 = id`, `f1/f2 = x/y`).
* `0x80197F00` is the ledgered player-SE spawn (`src/audio/ax.rs:22953`).

---

## 2. `0x80B27ED0` - per-slot relay (two `qr3` sites)

* **Address:** `0x80B27ED0`
* **TSV size:** 432 bytes; reachable 108 instructions (one `blr` at `0x80B2807C`).
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b27ed0_FUN_80b27ed0.c`
  - three qr0 spills (`f31`/`f30`/`f29`) plus the two `qr3` loads.
* **Proposed signature:** `pub unsafe fn fn_80b27ed0(this: usize, angle: s16, slot: u32)`
* **Callers in this REL:** one `bl` at `0x80B280C4`; also the
  `W137_FN_80B27ED0` call site (`call3(this, angle, i)`) in
  `src/game/enemies.rs`.
* **Constants:**

| address | type | value / role |
|---|---|---|
| `0x80B32C78` | f32 | `0.00390625` = `1/256` (angle16 -> turns) |
| `+0x6E8 + 4*slot` | u32 | scene handle for the slot |
| `+0x774`/`+0x7B8`/`+0x7FC` | f32 | per-slot node offset (x/y/z), stride 4 |
| `+0x738`/`+0x73C`/`+0x740` | f32 | base x/y/z |
| `+0x884 + 2*slot` | s16 | per-slot angle bias |
| `+0x8AC` | u32 | `1` = node-local transform, else rotate |

* **Verdict:** **ready** (confidence medium).

### Behaviour

```
tgt = scene(u32[this + 0x6E8 + 4*slot])
if (!tgt) return
tgt.s16[+0x102] = (s16)(angle + s16[this + 0x884 + 2*slot])
off = this + 4*slot

if (u32[this+0x8AC] == 1) {
    src = { f32[off+0x774], f32[off+0x7B8], f32[off+0x7FC] }
    rotate(this+0x584, &src, &out)
    tgt.x = f32[this+0x738] + out[0]
    tgt.y = f32[this+0x73C] + out[1]
    tgt.z = f32[this+0x740] + out[2]
} else {
    a  = (f32)(u16)tgt.s16[+0x102] * (1.0/256.0)    # qr3 @ 0x80B27FD4
    c  = cos256(a)
    a2 = (f32)(u16)tgt.s16[+0x102] * (1.0/256.0)    # qr3 @ 0x80B27FF4
    s  = sin256(a2)
    rx = f32[off+0x774]
    rz = f32[off+0x7FC]
    tgt.x = f32[this+0x738] + (c*rx + s*rz)
    tgt.z = f32[this+0x740] + (c*rz - s*rx)
}

v = fn_80b27dc0(f32[off+0x774], s16[this+0x102])
tgt.f32[+0xDC] = tgt.f32[+0xE0] = tgt.f32[+0xE4] = v
```

### Notes

* The two `qr3` loads read the same `s16` the body has just written; the second
  lane is the untouched neighbour halfword and is dead. With
  `GQR3 = 0x00050005` the recovered value is exactly `(f32)(u16)angle`.
* `0x80B27DC0` is a 272-byte untranslated sibling that takes `(f32 radius,
  s16 angle)` and returns the per-link speed; its first argument here is the
  per-slot x radius, the second the *parent's* `+0x102`.
* The `+0x8AC == 1` arm does not set y in the rotation arm — the node-local
  transform supplies it.

---

## 3. `0x80A00D00` - `daEnBuku_c` four-player carry

* **Address:** `0x80A00D00`
* **TSV size:** 424 bytes; reachable 106 instructions (one `blr` at `0x80A00EA4`).
* **Export:** `work/ghidra/rel-d-enemiesNP/export/80a00d00_FUN_80a00d00.c`
  - four qr0 spills (`f31`/`f30`/`f29`/`f28`).
* **Proposed signature:** `pub unsafe fn fn_80a00d00(this: usize)`
* **Callers in this REL:** two `bl` sites at `0x80A021B4` and `0x80A02454`; also
  the `W71_FN_80A00D00` call sites in `src/game/enemies.rs` (the "carry the four
  players" step).
* **Constants** (base `0x80B2A7B0`):

| offset | value | role |
|---|---|---|
| `+0xA0` | `3.0` | snap window (x and y) |
| `+0xA4` | `0.0` | z component of the direction vector |
| `+0xBC` | `17.0` | carry height above `this` |
| `+0xC0` | `2.0` | per-frame pull step |

* **Fields:** `+0x92E + i` (byte, `i = 0..3`) marks a carried player;
  `+0xAC`/`+0xB0`/`+0xB4` x/y/z.
* **Verdict:** **ready** (confidence medium).

### Behaviour

```
for i in 0..4 {
    if (u8[this + 0x92E + i] != 1) continue
    p = player(i)
    v = { f32[this+0xAC] - f32[p+0xAC],
          (f32[this+0xB0] + 17.0) - f32[p+0xB0],
          0.0 }
    normalise(&v)
    nx = f32[p+0xAC] + 2.0*v[0]
    ny = f32[p+0xB0] + 2.0*v[1]
    dx = |f32[this+0xAC] - nx|
    dy = |(f32[this+0xB0] + 17.0) - ny|
    if (dx < 3.0 && dy < 3.0) {
        u8[this + 0x92E + i] = 0
        p.pos = { f32[this+0xAC], f32[this+0xB0] + 17.0, f32[this+0xB4] }
    } else {
        p.x = nx
        p.y = ny
        # z unchanged
    }
}
```

### Notes

* The pull is 2 units of a normalised 3-vector; `0x80170A00` is the engine
  normaliser used the same way at `src/game/enemies.rs:9247` and `:16026`.
* The snap arm writes z from `this`, the pull arm leaves the player's z alone.
* The marker table is four bytes at `+0x92E`..`+0x931`; nothing else in this
  body touches them.

---

## 4. `0x80A04A90` - predicted-position query into a speed triple

* **Address:** `0x80A04A90`
* **TSV size:** 348 bytes; reachable 87 instructions (one `blr` at `0x80A04BE8`).
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a04a90_FUN_80a04a90.c`
  - one qr0 `f31` spill.
* **Proposed signature:** `pub unsafe fn fn_80a04a90(this: usize, scale: f32)`
* **Callers in this REL:** two `bl` sites at `0x80A04EBC` and `0x80A05148`; also
  the `W146_FN_80A04A90` call with `this->+0x6A4` in `src/game/enemies.rs`;
  `docs/SPEC_WAVE145_ENEMIES.md:966` records it under `if (f < 20.0)`.
* **Constants:**

| address | value | role |
|---|---|---|
| `0x80B2A994` | `0.0` | forced z speed (`+0xF0`) |

* **Fields:** `+0xD0`/`+0xD4`/`+0xD8` velocity; `+0x670`/`+0x674`/`+0x678` copy
  of delta + velocity; `+0x348` facing byte; `+0x6E0` variant-dirty byte;
  `+0xE8`/`+0xEC`/`+0xF0` speed.
* **Verdict:** **ready** (confidence medium).

### Behaviour

```
f31 = scale
t = near_actor(this, &tmp)          # 0x80064740
if (!t) return

dx = f32[t+0xAC] - f32[this+0xAC]
dy = f32[t+0xB0] - f32[this+0xB0]
dz = f32[t+0xB4] - f32[this+0xB4]
dx += f32[this+0xD0]
dy += f32[this+0xD4]
dz += f32[this+0xD8]

f32[this+0x670] = dx
f32[this+0x674] = dy
f32[this+0x678] = dz

face = (u8)face_variant(this, this+0xAC)        # 0x80096240
if (u8[this+0x348] == face) {
    fn_80a04940(this, &this[0x670], 512)        # 140-byte sibling
} else {
    u8[this+0x348] = face
    u8[this+0x6E0] = 1
}

v = { dx, dy, dz }
normalise(&v)                                   # 0x80170A00
f32[this+0xE8] = v[0] * f31
f32[this+0xEC] = v[1] * f31
f32[this+0xF0] = 0.0
integrate(this)                                 # 0x8006CD40
```

### Notes

* The 3-vector written at `+0x670` is the raw delta plus velocity; the
  normalised copy is what becomes the speed triple.
* `0x80096240` returns the archive/area variant byte (see
  `src/game/bases.rs:34932`); the compare against `+0x348` is the "facing
  changed" latch.
* `0x80A04940` (140 bytes, untranslated) is called with the raw triple and the
  literal `512`; it is the only callee the behaviour depends on besides the
  normaliser.

---

## 5. `0x80B16BA0` - node matrix commit from position and three angles

* **Address:** `0x80B16BA0`
* **TSV size:** 316 bytes; reachable 79 instructions (one `blr` at `0x80B16CD8`).
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b16ba0_FUN_80b16ba0.c`
  - no `psq` words at all.
* **Proposed signature:** `pub unsafe fn fn_80b16ba0(this: usize)`
* **Callers in this REL:** one `bl` at `0x80B1647C`; also
  the `W146_FN_80B16BA0` call site in `src/game/enemies.rs` (`W146_FN_80B16BA0`, `call1(this)`).
* **Fields:** `+0x7C` layout node, `+0x100`/`+0x102`/`+0x104` `s16` angles,
  `+0x54C` animation node, `+0x628` variant selector (`1` = second animation),
  `+0x5F8`/`+0x5FC`/`+0x600` animation outputs, `+0xAC..0xB4` position,
  `+0x464`/`+0x468`/`+0x46C` scale triple.
* **Verdict:** **conditional** (confidence medium): two of the callees'
  argument roles are not established.

### Behaviour

```
p = { f32[this+0xAC], f32[this+0xB0], f32[this+0xB4] }
a = { s16[this+0x100], s16[this+0x102], s16[this+0x104] }
player_aware_adjust(&p, &a, 1)                  # 0x80064AF0
translation(this+0x7C, p[0], p[1], p[2])        # 0x801C0BD0
set_rot_x(this+0x7C, a.s16[0])                  # 0x8016ECB0
set_rot_y(this+0x7C, a.s16[1])                  # 0x8016EDD0
set_rot_z(this+0x7C, a.s16[2])                  # 0x8016EEF0
copy_xform(this+0x54C, this+0x7C)               # 0x8016A170
copy_scale(this+0x54C, &this[0x464])            # 0x8016A150
mark_dirty(this+0x54C, 0)                       # 0x8016A1A0
start_anim(this+0x54C, (u32[this+0x628] == 1) ? 2 : 1, &out)   # 0x80168180
f32[this+0x5F8] = out[0x0C]
f32[this+0x5FC] = out[0x1C]
f32[this+0x600] = out[0x2C]
```

### Notes

* The order of the three rotation writes is x, y, z; each takes the *address* of
  the `s16` copy on the stack, not the value.
* `0x80168180` is the "starts one animation on `node`" helper
  (`src/engine/display.rs:11667`); the two ids are the `+0x628` variant select.
* **Open:** `0x80064AF0` is documented as `(position, mode, flag)` but the raw
  call passes a pointer in `r4` (the angle triple) and `1` in `r5`; the meaning
  of `out[0x0C]`, `out[0x1C]`, `out[0x2C]` is not established. A translation
  must keep the raw three offsets, which is why the row is conditional.

---

## 6. `0x80AF3400` - player-slot launch with per-slot radius

* **Address:** `0x80AF3400`
* **TSV size:** 308 bytes; reachable 77 instructions (one `blr` at `0x80AF3530`).
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80af3400_FUN_80af3400.c`
  - two qr0 spills (`f31`/`f30`) plus the two `qr3` loads.
* **Proposed signature:** `pub unsafe fn fn_80af3400(this: usize, slot: u32)`
  (the only `src` call site uses `call0`; the raw body reads `r4` as the player
  slot).
* **Callers in this REL:** one `bl` at `0x80AF33C8`; also the
  `BURST_FN_80AF3400` `call0` in `src/game/enemies.rs`.
* **Constants:**

| address | type | value / role |
|---|---|---|
| `0x80B3138C`..`0x80B31398` | f32 | `15.0`, `7.0`, `7.0`, `7.0` (per-slot radius table) |
| `0x80B313D4` | f32 | `1/256` |
| `+0x78C` | — | destination object for `0x8013FAA0` |
| `+0x7A4` | s16 | launch angle |
| `+4` | u32 | low two bits select one of the four radius words |

* **Verdict:** **conditional** (confidence medium): `0x8013FAA0`'s contract and
  the meaning of the four radius words are not established.

### Behaviour

```
src  = this
p    = player(r4)
if (!p) return
attach(p, src+0x78C)                            # 0x8013FAA0 (uncharacterised)
n    = (u32[src+4] & 3)
r    = f32[0x80B3138C + 4*n]                    # lfsx: 15 / 7 / 7 / 7
a    = (f32)(u16)s16[src+0x7A4] * (1.0/256.0)   # qr3 @ 0x80AF348C
s    = sin256(a)
a2   = (f32)(u16)s16[src+0x7A4] * (1.0/256.0)   # qr3 @ 0x80AF34B0
c    = cos256(a2)
motion(p, 1000, r*c, r*s)                       # 0x80146E20
sound(p, 142)                                   # 0x80056C70
vtable(p)[0x6C](p) -> index byte
u8[src + 0x0B4C + (s8)index] = 1
```

### Notes

* The radius load is `lfsx f31, r1+16, r0` with `r0 = (u32[src+4] & 3) * 4`; the
  four words are the `f31` table on the stack, so the radius is one of
  `15.0`, `7.0`, `7.0`, `7.0`.
* `0x80146E20` is the world-map two-value motion storer
  (`src/game/worldmap.rs:37469`); the literal `1000` is its id.
* `0x80056C70` removes/marks one pad channel id (`src/engine/display.rs:16601`);
  the literal here is `142`.
* The final `stb` uses the *signed* byte at the vtable `+0x6C` result and sets a
  per-index byte at `+0xB4C`.

---

## 7. `0x80A37CB0` - nearest player on the actor's side

* **Address:** `0x80A37CB0`
* **TSV size:** 304 bytes; reachable 76 instructions (one `blr` at `0x80A37DDC`).
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a37cb0_FUN_80a37cb0.c`
  - two qr0 spills (`f31`/`f30`).
* **Proposed signature:** `pub unsafe fn fn_80a37cb0(this: usize) -> i32`
* **Callers in this REL:** one `bl` at `0x80A36AAC`; the `W117_FN_80A37CB0`
  call site in `src/game/enemies.rs` uses the result as a target/clip index.
* **Constants:**

| address | value | role |
|---|---|---|
| `0x80B2C160` | `0.0` | side threshold |
| `0x80B2C1F4` | `10000000.0` | initial "nearest" distance |
| `0x80429CA8` | byte | active-player bitmask |

* **Fields:** `+0x348` side byte (`0` = right, non-zero = left), `+0xAC` x.
* **Verdict:** **ready** (confidence medium-high).

### Behaviour

```
best = -1
dmin = 10000000.0
mask = u8[0x80429CA8]

for i in 0..4 {
    p = player(i)
    if (!p) continue
    if ((mask & (1 << (i & 0xFF))) == 0) continue

    d = f32[p+0xAC] - f32[this+0xAC]
    if (u8[this+0x348] == 0) {
        if (d < 0.0) continue                 # right side only
    } else {
        if (d > 0.0) continue                 # left side only
    }
    ad = |d|
    if (ad >= dmin) continue
    dmin = ad
    best = (s8)*(u8*)vtable(p)[0x6C](p)       # clip/kind index
}
return best
```

### Notes

* The side test is `cror 2,1,2` (right) / `cror 2,0,2` (left); the following
  `bne` skips when the half-plane does not match.
* `dmin` is seeded with `1e7`, so the first eligible player always wins.
* The returned `s8` is the byte at the pointer returned by the player's
  `+0x6C` virtual; `-1` means no player matched.

---

## 8. `0x80AB13E0` - `fishbone` four-node binder

* **Address:** `0x80AB13E0`
* **TSV size:** 304 bytes; reachable 76 instructions (one `blr` at `0x80AB150C`).
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ab13e0_FUN_80ab13e0.c`
  - no `psq` words.
* **Proposed signature:** `pub unsafe fn fn_80ab13e0(this: usize)`
* **Callers in this REL:** one `bl` at `0x80AB15AC`; also the
  `W129_FN_80AB13E0` `call1(this)` site in `src/game/enemies.rs`.
* **Data:**

| address | contents |
|---|---|
| `0x80B54C00` | four name pointers: `fishbone_breakA`..`fishbone_breakD` |
| `0x80B2FB10` | four animation descriptors (stride 4; first word `0x128`) |
| `0x80B54C1C` | `"g3d/fishbone.brres"` |
| `0x80B54C30` | clip prefix `"fishbone"` |
| `0x80B2FAF0` | `1.0` |

* **Fields:** `+0x524` owner record, `+0x540` model, `+0x584 + 0x40*i` the four
  matrix nodes, `+0x768` the animation group holder.
* **Verdict:** **ready** (confidence medium).

### Behaviour

```
for i in 0..4 {
    name = u32[0x80B54C00 + 4*i]          # fishbone_breakA..D
    node = node_by_name(this+0x540, name) # 0x80239E30
    init_matrix(this + 0x584 + 0x40*i, &node,
                this+0x524, u32[0x80B2FB10 + 4*i], 1, 0)   # 0x80169D90
    commit_matrix(this + 0x584 + 0x40*i)   # 0x80064BD0

    if (i == 0) {
        h = node_by_name(this+0x540, "attack_st")          # 0x8023A190
        build_anim(this+0x768, &node, &h, this+0x524, 0, 1) # 0x80165C30
        vtable(this+0x584)[0x18](this+0x584, this+0x768)
        set_anim(this+0x768, 3, 0)          # 0x80166340
        get_anim(this+0x768, 0)             # 0x80166360
        apply_anim(this+0x768, 0)           # 0x801662F0
        set_anim(this+0x768, 0, 1.0)        # 0x80166310
    }
}
```

### Notes

* The name table at `0x80B54C00` is followed by the model and clip strings; the
  four `fishbone_breakA`..`D` nodes are bound at `+0x584 + 0x40*i`.
* The 4-word descriptor table at `0x80B2FB10` supplies the `0x80169D90` kind
  argument (`0x128` for the first; the rest are `0x20`).
* The animation group is built only for `i == 0`; the `+0x18` virtual is the
  first matrix node's own "attach" slot, called with the animation group
  (`this+0x584` -> `this+0x768`).
* The five animation calls are the standard `0x80166340/60/F0/10` sequence also
  used by `docs/SPEC_WAVE146_ENEMIES.md`'s binder rows.

---

## 9. `0x80A390F0` - `lift_rakka_ashiba` actor initialiser

* **Address:** `0x80A390F0`
* **TSV size:** 300 bytes; reachable 75 instructions (one `blr` at `0x80A39218`).
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a390f0_FUN_80a390f0.c`
  - no `psq` words.
* **Proposed signature:** `pub unsafe fn fn_80a390f0(this: usize) -> u32`
  (always returns `1`).
* **Callers in this REL:** first slot of the class vtable at `0x80B3E430`
  (`+0xD4` slot = `0x800675B0`). The model strings `g3d/lift_rakka_ashiba.brres`
  (`0x80B3E3F4`) and `lift_rakka_ashiba` (`0x80B3E410`) sit just before the
  vtable; the base initialiser is `0x80A39410` (324 bytes, untranslated).
* **Constants** (base `0x80B2C240`):

| offset | value | role |
|---|---|---|
| `+0x28` | `1.0` | velocity x/y/z seed |
| `+0x2C` | `0.0` | box floats 1 and 2 |
| `+0x30` | `16.0` | box float 3 |
| `+0x34` | `40.0` | box float 4 |
| `+0x38` | `-40.0` | box float 5 |
| `+0x18`/`+0x1C`/`+0x20` | `0.0`/`8.0`/`80.0` | `+0x318`/`+0x31C`/`+0x320` |
| `+0x24` | `16.0` | `+0x324` |

* **Fields:** `+0x5B0` five-float box + five words; `+0x5EC` flags (bit 14 =
  `0x4000`); `+0x5F2`; `+0x5F7` = `8`; `+0x5FA`; `+0x62A` = `4`;
  `+0x62C..0x62F` = `-1`; `+0x318..0x324`; `+0xDC..0xE4`; `+0xB8..0xC0` and
  `+0x610..0x618` previous positions.
* **Verdict:** **ready** (confidence medium).

### Behaviour

```
base_init(this)                                # 0x80A39410
f32[this+0xB8] = f32[this+0xAC]                # previous x
f32[this+0xBC] = f32[this+0xB0]
f32[this+0xC0] = f32[this+0xB4]
f32[this+0x610] = f32[this+0xAC]               # second previous x
f32[this+0x614] = f32[this+0xB0]
f32[this+0x618] = f32[this+0xB4]
f32[this+0xDC] = f32[this+0xE0] = f32[this+0xE4] = 1.0

box(this+0x5B0, this, 0, 1, 0,                # 0x800DB180
    0.0, 0.0, 16.0, 40.0, -40.0)
u8[this+0x5F2] = 0
u8[this+0x5FA] = u8[0x80429D28]
u8[this+0x5F7] = 8
u32[this+0x5EC] |= 0x4000
box_commit(this+0x5B0)                         # 0x800DAE80

u8[this+0x62C..0x62F] = 0xFF
u16[this+0x62A] = 4
f32[this+0x318] = 80.0
f32[this+0x31C] = 16.0
f32[this+0x320] = 0.0
f32[this+0x324] = 8.0

vtable(this)[0xD4](this, 0x80B70618)           # (ext) state descriptor
return 1
```

### Notes

* `0x80A39410` builds the `+0x524` manager, the `+0x540` model, the `+0x544`
  matrix node and the `+0x584` animation holder from
  `g3d/lift_rakka_ashiba.brres` / `lift_rakka_ashiba`; the initialiser above
  overwrites the derived state after that base call.
* `0x800DB180` is the five-float collision-box install documented in
  `src/game/bases.rs:93343`; the five words are `(box, this, 0, 1, 0)`.
* `0x80429D28` is a `main.dol` global byte copied into `+0x5FA`.
* `+0x60` must already hold the class vtable; the spawner installs it before
  slot 0 runs.

---

## 10. `0x80AEE850` - `daEnRailPolyParent_c` constructor

* **Address:** `0x80AEE850`
* **TSV size:** 296 bytes; reachable 74 instructions (one `blr` at `0x80AEE974`).
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aee850_FUN_80aee850.c`
  - no `psq` words.
* **Proposed signature:** `pub unsafe fn fn_80aee850() -> usize`
  (`0` on allocation failure, else the new `0x718`-byte object).
* **Callers in this REL:** fourth entry of the constructor table at
  `0x80B61DE4` (`0x80AEE7C0`, `0x80AEE7F0`, `0x80AEE820`, `0x80AEE850`). The
  class block at `0x80B61D18` names
  `daEnRailPolyParent_c::StateID_RailMoveWait` / `RailMove` / `RailMoveEnd` /
  `Drop`; model `g3d/test_lift.brres` (`0x80B61DFC`), clip `test_lift`
  (`0x80B61E10`), vtable `0x80B61E1C` (`+0xD4` = `0x80AED690`).
* **Constants / records:**

| address | value | role |
|---|---|---|
| `0x80B61E1C` | vtable | installed at `+0x60` |
| `0x80B61A48` | record | class record written to `+0x694`, then replaced |
| `0x80B61A08` | record | final `+0x694` value |
| `0x80B61A88` | word | record head written last to both relation records |
| `0x80B61AB8`/`0x80B56E48`/`0x80B61AE8`/`0x80B61B00` | words | record slots `+0/+4/+8/+0xC` |
| `0x80377A04` | word | `0x8015FC90` fourth argument |

* **Verdict:** **ready** (confidence medium-high).

### Behaviour

```
p = alloc_zeroed(0x718)                        # 0x801628C0
if (!p) return 0
base_ctor(p)                                   # 0x80094E80
u32[p+0x60] = 0x80B61E1C
base_list(p+0x524)                             # 0x80069020
u32[p+0x540] = 0
init_node(p+0x544)                             # 0x80169CD0
init_element(p+0x584)                          # 0x8007F7A0
u32[p+0x694] = 0x80B61A48

for (rec = p+0x698; rec <= p+0x6D4; rec += 0x3C) {
    u32[rec+0x00] = 0x80B61AB8
    u32[rec+0x04] = 0x80B56E48
    u32[rec+0x08] = 0x80B61AE8
    u32[rec+0x0C] = 0x80B61B00
    u32[rec+0x10] = p
    u32[rec+0x14] = 0
    state_mgr(rec+0x18, rec+4, rec+8, 0x80377A04)   # 0x8015FC90
    u32[rec+0x00] = 0x80B61A88
}

u32[p+0x710] = p+0x698
u32[p+0x694] = 0x80B61A08
return p
```

### Notes

* The two relation records are 0x3C bytes each and identical apart from their
  address; `+0x710` points at the first.
* `0x801628C0` is the zeroed default-heap allocation
  (`src/resource/registry.rs`), `0x80094E80` the actor base constructor,
  `0x8015FC90` the four-argument state-manager constructor.
* `+0x694` is written twice: the first record is a provisional class record, the
  final value is `0x80B61A08`.

---

## 11. `0x80B0CD80` - `daEnTeresa_c` body binder

* **Address:** `0x80B0CD80`
* **TSV size:** 292 bytes; reachable 73 instructions (one `blr` at `0x80B0CEA0`).
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b0cd80_FUN_80b0cd80.c`
  - no `psq` words.
* **Proposed signature:** `pub unsafe fn fn_80b0cd80(this: usize, variant: u32)`
* **Callers in this REL:** two `bl` sites at `0x80B0DB1C` and `0x80B0DBB8`; also
  `call2(W135_FN_80B0CD80, this, 1)` at the `W135_FN_80B0CD80` call site in `src/game/enemies.rs` and
  `call2(W138_FN_80B0CD80, this, 0)` at the `W138` call site.
* **Data:**

| address | contents |
|---|---|
| `0x80B67FD0` | two name pointers: `wait` (`0x80B67FB4`), `shay_teresaA` (`0x80B67FBC`) |
| `0x80B31E60` | two flag words: `0`, `1` |
| `0x80B31E5C` | `1.0` |
| `0x80B31E34` | `0.0` |

* **Fields:** `+0x544` model, `+0x588` clip holder, `+0x640` animation holder,
  `+0x6B4` optional second animation holder, `+0x73C` release node,
  `+0x9B4` stored variant.
* **Verdict:** **ready** (confidence medium).

### Behaviour

```
name = u32[0x80B67FD0 + 4*variant]             # "wait" | "shay_teresaA"
flag = u32[0x80B31E60 + 4*variant]             # 0 | 1
node = node(this+0x544, name)                  # 0x8023A0B0
attach_anim(this+0x640, this+0x588, &node, flag)   # 0x801651F0
attach_desc(1.0, this+0x588, this+0x640)       # 0x80169F80
advance_anim(this+0x640, 0.0)                  # 0x801680E0

if (variant == 1) {
    node2 = node(this+0x544, name)             # 0x8023A200
    apply_anim(this+0x6B4, this+0x588, &node2, 0, flag)   # 0x80166BD0
    vtable(this+0x588)[0x18](this+0x588, this+0x6B4)
    set_anim(1.0, this+0x6B4, 0)               # 0x80166F10
    set_anim2(0.0, this+0x6B4, 0)              # 0x80166EF0
    advance_anim(this+0x73C, 0.0)              # 0x80168FE0
} else {
    advance_anim(this+0x73C, 0.0)
    stop_anim(this+0x6B4, 0)                   # 0x80166D10
}
u32[this+0x9B4] = variant
```

### Notes

* `0x801651F0(element, receiver, descriptor, mode)` files the element under the
  receiver; the `0x80169F80`/`0x801680E0` pair is the standard attach/advance
  sequence (see `src/engine/display.rs:24595`).
* `0x80166EF0`/`0x80166F10` set the two animation values to `0.0` and `1.0`;
  `0x80168FE0` releases the node and `0x80166D10` stops the second holder.
* The variant is latched at `+0x9B4`.

---

## 12. `0x80B0CEB0` - Teresa tail binder

* **Address:** `0x80B0CEB0`
* **TSV size:** 288 bytes; reachable 72 instructions (one `blr` at `0x80B0CFCC`).
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b0ceb0_FUN_80b0ceb0.c`
  - no `psq` words.
* **Proposed signature:** `pub unsafe fn fn_80b0ceb0(this: usize, variant: u32)`
* **Callers in this REL:** three `bl` sites at `0x80B0DC9C`, `0x80B0DCD8` and
  `0x80B0DD38`; also three `call2(W132_FN_80B0CEB0, this, 1/0)` sites in
  `src/game/enemies.rs`.
* **Data:**

| address | contents |
|---|---|
| `0x80B68000` | two name pointers: `shay_teresaB` (`0x80B67FD8`), `shay_teresaB_wait` (`0x80B67FE8`) |
| `0x80B31E5C` | `1.0` - initial animation value |
| `0x80B31E34` | `0.0` - the else-arm value |

* **Fields:** `+0x544` model, `+0x5C8` clip holder, `+0x678` animation holder,
  `+0x6E0` second animation holder, `+0x6A0` per-variant rate, `+0x9B8` stored
  variant.
* **Verdict:** **ready** (confidence medium).

### Behaviour

```
name = u32[0x80B68000 + 4*variant]             # shay_teresaB | shay_teresaB_wait
node = node(this+0x544, name)                  # 0x8023A0B0
attach_anim(this+0x678, this+0x5C8, &node, 1)  # 0x801651F0
attach_desc(1.0, this+0x5C8, this+0x678)       # 0x80169F80
advance_anim(this+0x678, 1.0)                  # 0x801680E0

node2 = node(this+0x544, name)                 # 0x8023A200
apply_anim(this+0x6E0, this+0x5C8, &node2, 0, 1)   # 0x80166BD0
vtable(this+0x5C8)[0x18](this+0x5C8, this+0x6E0)
set_anim(1.0, this+0x6E0, 0)                   # 0x80166F10

if (variant == 0) {
    advance_anim(this+0x678, f32[this+0x6A0])  # 0x80168FE0
    set_anim2(f32[this+0x6A0], this+0x6E0, 0)  # 0x80166EF0
} else {
    advance_anim(this+0x678, 0.0)              # 0x80168FE0
    set_anim2(0.0, this+0x6E0, 0)              # 0x80166EF0
}
u32[this+0x9B8] = variant
```

### Notes

* The two arms differ only in the value: the per-actor word at `+0x6A0` for
  variant `0`, the literal `0.0` for variant `1`.
* This is the tail twin of rank 11: node/clip tables and the `+0x9B8` latch are
  the only structural difference.

---

## Verification

* Every address is absent from the `address` column of `ledger/functions.csv`
  and was absent from every `^#` heading of every other `docs/*.md` at selection
  time (before this page existed). Four of them are mentioned in another row's
  `evidence` text (`0x80B27ED0`, `0x80A04A90`, `0x80AF3400`, `0x80B0CD80`);
  those are call-stub mentions, not ledger rows.
* No address is defined in `src/**/*.rs`. The only occurrences are
  `W*_FN_`/`BURST_FN_` reference stubs; the stubs' own comments call the bodies
  untranslated, which is the correction recorded under "Why ranks 1-4 and 11-12
  were not already covered".
* All twelve bodies walk to a terminal instruction at `entry + TSV_size - 4`
  with no unclaimed interior branch target and no padding arm, so the TSV size
  equals the reachable extent for all twelve.
* `qr3` is read in exactly two bodies (`0x80B27ED0`, `0x80AF3400`), always as
  the `GQR3 = 0x00050005` `u16` form. The remaining ten have only `qr0`
  `f31`/`f30`/`f29`/`f28` spills.
* The two conditional rows are conditional for stated reasons only:
  `0x80B16BA0` depends on the unestablished `0x80064AF0` argument/`out` roles;
  `0x80AF3400` depends on the uncharacterised `0x8013FAA0` and on the meaning of
  the four `15/7/7/7` radius words. Their instruction streams are fully literal.
