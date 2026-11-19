# Wave 131 - implementation specification for 12 class-B `d_enemiesNP` bodies

## What this page is

Wave 127 (`docs/ANALYSIS_WAVE127_ENEMIES.md`) reclassified 173 of the 384
formerly-skipped `d_enemiesNP` addresses as class **B**: the reachable
instruction stream decodes end-to-end, every path ends in `blr` or a jump to a
real function entry, and nothing depends on a `GQR1..GQR7` quantiser, a jump
table, an unresolved base or an incoming float argument.

Waves 128, 129 and 130 (`docs/SPEC_WAVE128_ENEMIES.md`,
`docs/SPEC_WAVE129_ENEMIES.md`, `docs/SPEC_WAVE130_ENEMIES.md`) specified
ranks 1-36 of that pool. This lane is the continuation: **ranks 37-48** of the
class-B pool, in the same ordering.

It is **read-only**. It writes no Rust, no ledger row and no other doc. Each
body below was decoded from `work/rels/elf/d_enemiesNP.elf` with the corrected
`+0x1000` segment shift, cross-checked against the Ghidra export at the cited
path and against `work/ghidra/rel-d_enemiesNP/export/functions.tsv`. Every
constant that lives inside the REL image was read out of the ELF itself, so the
float values in the tables below are the shipped values, not guesses.

All twelve were re-checked against the `address` column of
`ledger/functions.csv` (absent) and against `functions.tsv` (present). Nothing
here duplicates a wave-128, wave-129 or wave-130 claim.

## Selection

Ordering is the wave-127 "reachable instruction count" - the only size metric
that is not inflated by the TSV extent quirk. Ties are broken by TSV size, then
by address. These are ranks 37-48. The next candidates after this set are
`0x80A368B0` and `0x80AFFAF0`, both at 61 reachable instructions.

| # | address | TSV size | reachable insns | body character | status |
|---:|---|---:|---:|---|---|
| 1 | `0x80AB2840` | 356 | 89 | four-sample trail initialiser (positions, A/B tables, 0.015) | **ready** |
| 2 | `0x80A34F80` | 348 | 87 | 60-byte bubble billboard record, two scale arms | **ready** |
| 3 | `0x80A2FD40` | 340 | 85 | `env_underwater` / `wmy_ef_bubble` spawn binder | **ready** |
| 4 | `0x80A77A50` | 340 | 85 | `lift` spawn init (two fn-pointer tables, state 4) | **ready** |
| 5 | `0x80B1D090` | 328 | 82 | matrix refresh with `+/-12` translate between Y/X/Z | **ready** |
| 6 | `0x80A39410` | 324 | 81 | `lift_rakka_ashiba` model + damage binder | **ready** |
| 7 | `0x80AD42E0` | 320 | 80 | contact/pad record emitter (20-byte record) | **ready** |
| 8 | `0x80B0DC40` | 320 | 80 | timered two-state machine (300-frame retry) | **ready** |
| 9 | `0x80A37560` | 296 | 74 | `Wm_en_landsmoke_s` effect trigger | **ready** |
| 10 | `0x80AD0960` | 296 | 74 | two-position ground probe (`x+/-dz`) -> side byte 1/2/3 | **ready** |
| 11 | `0x80A370D0` | 292 | 73 | state 1 -> 2 plus `walk` clip bind | **ready** |
| 12 | `0x80A628F0` | 292 | 73 | side-scaled speed init (external char table) | **conditional** |

**Split: 11 ready, 1 conditional.**

"Conditional" is used exactly as in waves 128-130: a named datum in `main.dol`
that is **consumed arithmetically or used as an index** and must be read before
a byte-exact port. Only #12 does that (`0x8042C1A0`, a byte table indexed by
the side returned from `0x80A61F90`, widened to float and multiplied). Every
other external address in this set is either (a) a `main.dol` callee called by
address, (b) a pointer or state token that is only dereferenced, stored or
passed through as an argument, or (c) a float constant that lives **inside**
this REL and is already quoted with its value below. The four risk classes
that made waves 110-126 skip a body are all absent:

* no `psq_l`/`psq_st` other than `f31`/`f30`/`f29` stack spills with `qr0`
  (6 spill pairs in 4 of the 12 bodies, all `qr0`);
* no `bctr` jump table - every conditional branch target is inside its body
  (checked mechanically: zero out-of-body `bc` targets);
* no incoming float argument - no body reads `f1`/`f2`/`f3` before writing it;
* no unledgered callee landmine - all 14 in-REL `bl` targets are
  `functions.tsv` entries.

## Reading conventions

* Synthetic base is `0x80A00000`. ELF file offset is
  `(address - 0x80A00000) + 0x1000` for the first `PT_LOAD`
  (`p_offset = 0x10F0`, `p_vaddr = 0x80A000F0`); the same shift holds for the
  data segments. The image ends at `0x80B6E434`; anything above that is a
  `main.dol` address.
* `this` is the incoming `r3` unless stated otherwise. All `+0xNNN` are byte
  offsets from `this`.
* An address that does not resolve in `d_enemiesNP.elf` is a `main.dol`
  address. It is marked **(ext)** and must be treated as a named external
  datum, not as a missing base.
* In-REL float constants are quoted with their shipped value. They were read
  at file offset `(addr - 0x80A00000) + 0x1000` and are `f32` unless marked
  `f64`.
* Branch mnemonics are the C form after the exact `bc` operand decode:
  `bc 12,2` = `beq`, `bc 4,2` = `bne`, `bc 4,0` = `bge`, `bc 12,1` = `bgt`,
  `bc 4,1` = `ble`, `bc 12,0` = `blt`, `bc 16,*` = `bdnz`.
* **Record-form instructions matter.** `rlwinm.` sets `cr0`; the Ghidra
  exports print four sites in this set without the dot
  (`0x80A34FA8`, `0x80AB9B50`, `0x80AD0A00`, `0x80AD0A14`). A port that drops
  the dot reads a stale `cr0` at the following `bne`/`beq`.
* Helper roles below are the ledger's, not new claims:
  `0x800690C0` = reset a model holder `(holder, -1, resetWord, 0, 0x20)`;
  `0x800690E0` = finish it; `0x800DF180` = load a model by name
  `(archive, name, path)`; `0x80239E30` = find a model node;
  `0x8023A0B0` = find an animation by name;
  `0x8023A200` = find a second animation by name;
  `0x80169D90` = bind a model record; `0x80064BF0` = finish a model record;
  `0x80064AF0` = position + Euler to vector `(vec, ang, mode)`;
  `0x801C0BD0` = translation-matrix builder `(out, x, y, z)`;
  `0x801C0540` = concat `(out, a, b)`; `0x8016A170` = copy into a holder;
  `0x8016A150` = offset a holder; `0x8016A1A0` = clear a holder flag;
  `0x8016ECB0`/`0x8016EDD0`/`0x8016EEF0` = set a matrix pitch/yaw/roll from a
  `u16`; `0x801651F0` = attach an animation; `0x80169F80` = apply an animation
  blend `(f1, holder, holder2)`; `0x801680E0` = set an animation rate;
  `0x80168FE0` = set an animation frame; `0x8006A4B0` = position to sound
  argument `(&out, pos)`; `0x80197F00` = sound with a position
  `(system, cue, &pos, 0)`;
  `0x8007FA40` = start an emitter `(f1..f4, owner, actor, 0, 0, 0, 3, 0, 0)`;
  `0x8007F900` = release it; `0x800B8700` = commit an effect record
  `(owner, record, 1)`; `0x802DCA98` = register a record callback
  `(record, fn, bytes, count)`.

## Symbolic helpers used below

```text
vt(obj, slot)            # (*(u32 *)obj)->vt[slot], receiver r3 = obj
fn_80ab1a00(this)        # untranslated init helper (B row, 8 bytes)
fn_80a778f0(this)        # untranslated (B row, 116 bytes)
fn_80a369b0(this) -> u32 # untranslated heading step (B row, 152 bytes)
fn_80ab9c10(this, rec)   # translated: g3d/lift_trampoline.brres binder
fn_80b0ceb0(this, on)    # untranslated
fn_80b0cfd0(this)        # untranslated
fn_80b0c890(this)        # translated: clears +0x9E0 on the overlap test
fn_80b0d150(this) -> u32 # untranslated: two lookups on +0x640/+0x678
fn_80b0d4d0(this) -> u32 # untranslated
fn_80a61a80(f1, f2, this, 4, 1)  # translated
fn_80a61f90(this) -> u8  # translated: +0x732 phase, returns the side byte
```

---

## 1. `0x80AB2840` - four-sample trail initialiser

* **TSV size:** 356 bytes
* **Reachable body:** 89 instructions; 3 `bl` + 0 `bctrl`; ends at
  `0x80AB29A0 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ab2840_FUN_80ab2840.c`
* **Proposed signature:** `pub unsafe fn fn_80ab2840(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x10(r1)`; LR at `+0x14`; `r31` at `+0x0C`. `r31 = this`
(`or r31,r3,r3` at `0x80AB286C`; `r3` is restored from `r31` at `0x80AB296C`
after the first call advanced it). Page register `r6 = 0x80B2FA50`;
`r7 = this+0x0C`, `r9 = this+4`, `r11 = this+6`. Loop control is `li r0,2` at
`0x80AB285C` and `mtspr CTR,r0` at `0x80AB288C`, so the `bdnz` at
`0x80AB292C` runs the body exactly twice; each pass writes two 12-byte samples,
for four total.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2FA50` | page base | `r6` |
| `0x80B2FAE4` | `f32` | `0.0` (`w` slot of every sample) |
| `0x80B2FAE8` | `f32` | `8.0` (added to the y of every sample) |
| `0x80B2FB5C` | `f32[4]` | `{-0.25, 0.1, 0.5, -0.3}` - the `u` slot of samples 0-3 |
| `0x80B2FB74` | `f32[4]` | `{1.0, 1.5, 1.0, 1.5}` - the `v` slot of samples 0-3 |
| `0x80B2FB88` | `f32` | `0.015` |
| `0x8042C1B8` | `f32` | **(ext)** stored to `+0x114` and `+0x960` |
| `0x8042C1BC` | `f32` | **(ext)** stored to `+0xEC` |
| `0x8042C1C0` | `f32` | **(ext)** stored to `+0xF8` and `+0x978` |

### Behaviour

```c
for (k = 0; k != 4; ++k) {                     // 0x80AB2890..0x80AB292C
    u8 *s = (u8 *)this + 0x8CC + k * 12;       // r7 = this+0x0C + (k/2)*0x18
    *(f32 *)(s + 0x00) = *(f32 *)(this + 0xAC);
    *(f32 *)(s + 0x04) = *(f32 *)(this + 0xB0);           // raw ...
    *(f32 *)(s + 0x08) = *(f32 *)(this + 0xB4);
    *(f32 *)(s + 0x04) = *(f32 *)(this + 0xB0) + 8.0f;    // ... then +8.0

    *(f32 *)(this + 0x908 + k * 12 + 0) = A[k];           // 0x80B2FB5C
    *(f32 *)(this + 0x908 + k * 12 + 4) = B[k];           // 0x80B2FB74
    *(f32 *)(this + 0x908 + k * 12 + 8) = 0.0f;           // 0x80B2FAE4

    *(u16 *)(this + 0x93E + k * 6 + 0) = 0;               // r5 = 0
    *(u16 *)(this + 0x93E + k * 6 + 2) = 0x4000;          // r4 = 0x4000
    *(u16 *)(this + 0x93E + k * 6 + 4) = 0;

    *(f32 *)(this + 0x968 + k * 4) = A[k];
}

*(f32 *)(this + 0xF8)  = *(f32 *)0x8042C1C0;   // **(ext)**
*(f32 *)(this + 0xEC)  = *(f32 *)0x8042C1BC;   // **(ext)**
*(f32 *)(this + 0x114) = *(f32 *)0x8042C1B8;   // **(ext)**
*(f32 *)(this + 0x978) = *(f32 *)0x8042C1C0;   // **(ext)**
*(f32 *)(this + 0x960) = *(f32 *)0x8042C1B8;   // **(ext)**
*(f32 *)(this + 0x95C) = 0.015f;               // 0x80B2FB88

fn_8008C380(this + 0x148);
fn_80AB1A00(this);
if (*(u32 *)(this + 0x988) == 1)
    fn_80166340(this + 0x794, 3, 0);
```

The raw-then-adjusted y store is real (`stfs f3,2244(r7)` at `0x80AB28A0`
followed by `stfs f3,2244(r7)` with `y+8.0` at `0x80AB28B4`, repeated at
`0x80AB28F0`/`0x80AB2900`); the last write wins, but the sequence is kept above
because the intermediate value is observable to a breakpoint debugger. The
`+0x968` array is a second copy of the `u` values: `0x80AB28BC` and
`0x80AB2908` re-store `A[k]` there immediately after the `u` store, and the
`u16` triple follows the `w` store in each half.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80AB2968` | `0x8008C380` | `r3 = this+0x148` |
| `0x80AB2970` | `0x80AB1A00` | `r3 = this` (restored from `r31`) |
| `0x80AB298C` | `0x80166340` | `r3 = this+0x794`, `r4 = 3`, `r5 = 0` |

### Ambiguity

No blocker. The `+0x8CC`/`+0x908`/`+0x93E`/`+0x968` blocks are a four-sample
trail ring; the field *roles* (`u`/`v`/`w`, and whether `w` is a rotation or a
weight) are inferred from the sibling four-sample writers in this module and
from the fact that `v` is exactly `{1.0, 1.5}`. Every value is literal. The
three `0x8042C1xx` words are `main.dol` constants stored verbatim into float
fields - they are never combined, compared or branched on here. `0x80AB1A00`
is an unclaimed B row (8 bytes); it and `0x80166340` are cited by address.

---

## 2. `0x80A34F80` - 60-byte bubble billboard record, two scale arms

* **TSV size:** 348 bytes
* **Reachable body:** 87 instructions; 2 `bl` + 0 `bctrl`; ends at
  `0x80A350D8 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a34f80_FUN_80a34f80.c`
* **Proposed signature:** `pub unsafe fn fn_80a34f80(this: usize) -> u32`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x70(r1)`; LR at `+0x74`; `r31` at `+0x6C`. Page register
`r7 = 0x80B2C040`. Scratch: `r1+0x08/0x0C/0x10` (the three record scale
floats), `r1+0x14/0x18/0x1C` (x, y, z), `r1+0x20..0x5B` (the 60-byte effect
record). `r31` saves the `0x800B8700` return word across the second call.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2C0A0` | `f32` | `14.0` (base y offset) |
| `0x80B2C0A4` | `f32` | `0.5` (base scale) |
| `0x80B2C08C` | `f32` | `1.8` (the "large" multiplier) |
| `0x80B2C094` | `f32` | `0.0` (eight fill words) |
| `0x80A0B5A0` | `u8[]` | **(in-REL)** record callback byte block |

### Behaviour

The arm is selected by bit 8 (`0x100`) of the word at `this+4`
(`lwz r0,4(r3)` + `rlwinm. r0,r0,24,31,31` at `0x80A34F98`/`0x80A34FA8`):

```c
p[0] = *(f32 *)(this + 0xAC);                  // r1+0x14
p[1] = *(f32 *)(this + 0xB0);                  // r1+0x18
p[2] = *(f32 *)(this + 0xB4);                  // r1+0x1C

if (((*(u32 *)(this + 4) >> 8) & 1) == 0) {
    s[0] = s[1] = s[2] = 0.5f;                 // 0x80B2C0A4
    p[1] -= 14.0f;                             // 0x80B2C0A0
} else {
    s[0] = s[1] = s[2] = 0.5f * 1.8f;          // 0.9
    p[1] -= 14.0f * 1.8f;                      // 25.2
}

u8 *rec = (u8 *)(r1 + 0x20);
*(u32 *)(rec + 0x00) = 0x1003;                 // li r0,0x1003
*(f32 *)(rec + 0x04) = p[0];
*(f32 *)(rec + 0x08) = p[1];
*(f32 *)(rec + 0x0C) = p[2];
*(f32 *)(rec + 0x10) = s[0];
*(f32 *)(rec + 0x14) = s[1];
*(f32 *)(rec + 0x18) = s[2];
for (i = 0; i != 8; ++i)                       // 0x80A35084..0x80A350A0
    *(f32 *)(rec + 0x1C + i * 4) = 0.0f;       // 0x80B2C094

u32 r = fn_800B8700(this + 0x484, rec, 1);
fn_802DCA98(rec, 0x80A0B5A0, 0x3C, 1);
return r;
```

The record body (`r1+0x24..0x58`) is pre-zeroed with 14 `stw r6,...` stores
before the fields are written. The type word `0x1003` is stored at `r1+0x20`
after that zero pass and before the position/scale stores, and the eight fill
words are written last. Keep that order if the port is byte-compared.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80A350A4` | `0x800B8700` | `r3 = this+0x484`, `r4 = r1+0x20`, `r5 = 1` |
| `0x80A350C0` | `0x802DCA98` | `r3 = r1+0x20`, `r4 = 0x80A0B5A0`, `r5 = 0x3C`, `r6 = 1` |

### Ambiguity

None in-REL. The bit-8 test selects a small (0.5, -14.0) or large (0.9, -25.2)
emitter, which is the usual "bubble grows on contact" pair. Record field roles
are the same 15-word layout used by wave-130 #2 (`type`, position, scale,
eight tail words). `0x800B8700` and `0x802DCA98` are `main.dol` callees.

---

## 3. `0x80A2FD40` - `env_underwater` / `wmy_ef_bubble` spawn binder

* **TSV size:** 340 bytes
* **Reachable body:** 85 instructions; 10 `bl` + 1 `bctrl`; ends at
  `0x80A2FE90 blr`; the epilogue at `0x80A2FE78` is reached through the
  `bctrl` (the call is linked, so it returns to the epilogue)
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a2fd40_FUN_80a2fd40.c`
* **Proposed signature:** `pub unsafe fn fn_80a2fd40(this: usize) -> u32`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x20(r1)`; LR at `+0x24`; `r31` at `+0x1C`, `r30` at `+0x18`.
`r30 = this`. Page register `r31 = 0x80B3C788`. Locals: `r1+0x08` (the
`0x80239F20` handle), `r1+0x0C` (the `+0x20`-biased pointer), `r1+0x10` (the
`0x80239E30` node).

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B3C7B0` | `char[]` | `"env_underwater"` (node name) |
| `0x80B3C794` | `char[]` | `"g3d/env_underwater.brres"` (archive path) |
| `0x80B3C7C0` | `char[]` | `"wmy_ef_bubble"` (model name) |
| `0x80B3C7D0` | `char[]` | `"ef_bubble"` (node name) |
| `0x80B2BD54` | `f32` | `-999999.0` (12 filler words) |
| `0x80377C48` | word | **(ext)** animator reset argument |
| `0x8042A038` | pointer | **(ext)** archive manager; the loader takes `+4` |

### Behaviour

```c
fn_8005F900(0);

fn_800690C0(this + 0x524, 0xFFFFFFFF, *(u32 *)0x80377C48, 0, 0x20);

u32 model = fn_800DF180(*(u32 *)0x8042A038 + 4,
                        0x80B3C7B0 /*"env_underwater"*/,
                        0x80B3C794 /*"g3d/env_underwater.brres"*/);
*(u32 *)(this + 0x540) = model;

u32 h1 = fn_80239F20(this + 0x540, 0x80B3C7C0 /*"wmy_ef_bubble"*/);
fn_800099D0(this + 0x544, &h1, 0xFFFFFFFF, 0x84, this + 0x524);

u32 node = fn_80239E30(this + 0x540, 0x80B3C7D0 /*"ef_bubble"*/);
u32 p = fn_8023B800(&node, 0);
u32 off = *(u32 *)(p + 0x3C);
u32 q = (off != 0 ? p + off : 0) + 0x20;
u32 slot = q;
fn_8023D300(&slot, 1, this + 0x6940);
fn_8023D300(&slot, 2, this + 0x6944);

for (i = 0; i != 12; ++i)                      // 0x80A2FE30..0x80A2FE5C
    *(f32 *)(this + 0x6910 + i * 4) = -999999.0f;   // 0x80B2BD54

fn_800690E0(this + 0x524);
vt(this, 0x20) (this);                          // 0x60 -> +0x20
return 1;
```

`0x800099D0` is called with `r6 = 0x84` (132) and `r7 = this+0x524`; the four
arguments after `r3` are `r4 = &h1`, `r5 = -1`, `r6 = 0x84`, `r7 = this+0x524`.
The `p` pointer is the node returned by `0x8023B800`; when its `+0x3C`
self-relative word is zero the base is 0, so `q` is `0x20` in that case - keep
the `0` arm, it is not a null check on the result.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80A2FD64` | `0x8005F900` | `r3 = 0` |
| `0x80A2FD80` | `0x800690C0` | `r3 = this+0x524`, `r4 = -1`, `r5 = *(0x80377C48)`, `r6 = 0`, `r7 = 0x20` |
| `0x80A2FD98` | `0x800DF180` | `r3 = *(0x8042A038)+4`, `r4 = 0x80B3C7B0`, `r5 = 0x80B3C794` |
| `0x80A2FDA8` | `0x80239F20` | `r3 = this+0x540`, `r4 = 0x80B3C7C0` |
| `0x80A2FDC4` | `0x800099D0` | `r3 = this+0x544`, `r4 = r1+0x08`, `r5 = -1`, `r6 = 0x84`, `r7 = this+0x524` |
| `0x80A2FDD0` | `0x80239E30` | `r3 = this+0x540`, `r4 = 0x80B3C7D0` |
| `0x80A2FDE0` | `0x8023B800` | `r3 = r1+0x10`, `r4 = 0` |
| `0x80A2FE10` | `0x8023D300` | `r3 = r1+0x0C`, `r4 = 1`, `r5 = this+0x6940` |
| `0x80A2FE20` | `0x8023D300` | `r3 = r1+0x0C`, `r4 = 2`, `r5 = this+0x6944` |
| `0x80A2FE60` | `0x800690E0` | `r3 = this+0x524` |
| `0x80A2FE74` | vtable `+0x20` of `*(this+0x60)` | `r3 = this` |

### Ambiguity

No blocker. The two strings distinguish a model (`wmy_ef_bubble`) from a
sub-node (`ef_bubble`); `0x80239F20` is the "find by name in a loaded model"
variant that returns a handle by value (contrast `0x80239E30`, which returns a
byte offset). `0x800099D0` is cited by address; its `r6 = 0x84` is a size and
`r7 = this+0x524` the owner. The 12 `-999999.0` words are a sentinel fill for
the emitter state block at `+0x6910`, not a numeric result.

---

## 4. `0x80A77A50` - `lift` spawn init (two fn-pointer tables, state 4)

* **TSV size:** 340 bytes
* **Reachable body:** 85 instructions; 3 `bl` + 0 `bctrl`; ends at
  `0x80A77BA0 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a77a50_FUN_80a77a50.c`
* **Proposed signature:** `pub unsafe fn fn_80a77a50(this: usize)`
* **Confidence:** **medium / ready**

### Frame

`stwu r1,-0x20(r1)`; LR at `+0x24`; `f31` at `+0x10` (`stfd` + `psq_st qr0`);
`r31` at `+0x0C`, `r30` at `+0x08`. `r30 = this`; page register
`r31 = 0x80B2DCE8`. `f31 = 1.0` (`0x80B2DD80`) is loaded once and used for
three field groups.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2DD80` | `f32` | `1.0` (orientation scales `+0xDC..+0xE4`, `+0x838..+0x840`) |
| `0x80B2DD70` | `f32` | `0.0` (`+0xD0`, `+0xD8`) |
| `0x80B2DD74` | `f32` | `8.0` (`+0xD4`, and the non-state-4 `+0x748..+0x750`) |
| `0x80B2DCFC` | `f32` | `0.0` (state-4 `+0x744`) |
| `0x80B2DCEC` | `f32` | `0.0` (non-state-4 `+0x744`) |
| `0x80B2DD78` | `f32` | `16.0` (state-4 `+0x748` and `+0x750`) |
| `0x80B2DDB0` | `f32` | `24.0` (state-4 `+0x74C`) |
| `0x80B2DD4C` | `u32` | `0` (data word passed to `0x8008C3E0`) |
| `0x80B2DD20` | `u32[2]` | `{0x80A77540, 0}` - `+0x738` table, non-state-4 |
| `0x80B2DD28` | `u32[2]` | `{0, 0x80A77540}` - `+0x73C` table, non-state-4 |
| `0x80B2DD30` | `u32[2]` | `{0x80A77680, 0}` - `+0x738` table, state 4 |
| `0x80B2DD38` | `u32[2]` | `{0, 0x80A77680}` - `+0x73C` table, state 4 |

### Behaviour

```c
*(f32 *)(this + 0xDC) = 1.0f;                  // 0x80B2DD80
*(f32 *)(this + 0xE0) = 1.0f;
*(f32 *)(this + 0xE4) = 1.0f;
*(f32 *)(this + 0xD0) = 0.0f;                  // 0x80B2DD70
*(f32 *)(this + 0xD4) = 8.0f;                  // 0x80B2DD74
*(f32 *)(this + 0xD8) = 0.0f;

fn_8008C3E0(this + 0x148, this, 0x80B2DD4C);

if (*(u32 *)(this + 0x88C) == 4) {
    *(f32 *)(this + 0x744) = 0.0f;             // 0x80B2DCFC
    *(f32 *)(this + 0x748) = 16.0f;            // 0x80B2DD78
    *(f32 *)(this + 0x74C) = 24.0f;            // 0x80B2DDB0
    *(f32 *)(this + 0x750) = 16.0f;
} else {
    *(f32 *)(this + 0x744) = 0.0f;             // 0x80B2DCEC
    *(f32 *)(this + 0x748) = 8.0f;             // 0x80B2DD74
    *(f32 *)(this + 0x74C) = 8.0f;
    *(f32 *)(this + 0x750) = 8.0f;
}

*(f32 *)(this + 0x164) = *(f32 *)(this + 0x744);
*(f32 *)(this + 0x168) = *(f32 *)(this + 0x748);
*(f32 *)(this + 0x16C) = *(f32 *)(this + 0x74C);
*(f32 *)(this + 0x170) = *(f32 *)(this + 0x750);

*(f32 *)(this + 0x838) = 1.0f;
*(f32 *)(this + 0x83C) = 1.0f;
*(f32 *)(this + 0x840) = 1.0f;

u32 idx = *(u32 *)(this + 0x874);
if (*(u32 *)(this + 0x88C) == 4) {
    *(u32 *)(this + 0x738) = ((u32 *)0x80B2DD30)[idx];
    *(u32 *)(this + 0x73C) = ((u32 *)0x80B2DD38)[idx];
} else {
    *(u32 *)(this + 0x738) = ((u32 *)0x80B2DD20)[idx];
    *(u32 *)(this + 0x73C) = ((u32 *)0x80B2DD28)[idx];
}
*(u32 *)(this + 0x740) = 0;

fn_80A778F0(this);
fn_8007F900(this + 0x754);
```

The index is `idx*4` (`rlwinm r0,r0,2,0,29` at `0x80A77B34` and
`0x80A77B58`); the state word `+0x88C` is re-loaded for the second branch, so
the two `== 4` tests read the same field twice. The two tables are disjoint:
`+0x738` receives the function pointer for `idx == 0` and `+0x73C` for
`idx == 1` (and the complement 0), which is how the two callbacks
`0x80A77540` / `0x80A77680` are paired per side.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80A77AA8` | `0x8008C3E0` | `r3 = this+0x148`, `r4 = this`, `r5 = 0x80B2DD4C` |
| `0x80A77B78` | `0x80A778F0` | `r3 = this` |
| `0x80A77B80` | `0x8007F900` | `r3 = this+0x754` |

### Ambiguity

No external datum. `0x80A77540` and `0x80A77680` are in-REL function entries
(two per-side lift callbacks); `+0x738`/`+0x73C` are the two callback words the
state entry later uses. `0x8008C3E0` is the two-argument sibling of the
single-argument `0x8008C380` used by #1 and is cited by address; its `r5` data
word `0x80B2DD4C` is zero in this image. `+0x164..+0x170` is a four-float
scratch copy of the `+0x744` block, so the block is still live after the
copy - do not fold it away if the port is byte-compared.

---

## 5. `0x80B1D090` - matrix refresh with `+/-12` translate between Y/X/Z

* **TSV size:** 328 bytes
* **Reachable body:** 82 instructions; 12 `bl` + 0 `bctrl`; ends at
  `0x80B1D1D4 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b1d090_FUN_80b1d090.c`
* **Proposed signature:** `pub unsafe fn fn_80b1d090(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0xA0(r1)`; LR at `+0xA4`; `f31` at `+0x90` (`stfd` + `psq_st qr0`);
`r31` at `+0x8C`, `r30` at `+0x88`. `r30 = this`. Scratch:
`r1+0x14/0x16/0x18` (three `u16` angles), `r1+0x1C/0x20/0x24` (three `f32`
position), `r1+0x58` and `r1+0x28` (48-byte matrices), `r1+0x10/0x0C/0x08`
(single `u16` rotation arguments).

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B32558` | `f32` | `0.0` (x and y of both translate matrices) |
| `0x80B32650` | `f32` | `12.0` (z of the two translate matrices) |

### Behaviour

```c
p[0] = *(f32 *)(this + 0xAC);                  // r1+0x1C
p[1] = *(f32 *)(this + 0xB0);
p[2] = *(f32 *)(this + 0xB4);
a[0] = *(s16 *)(this + 0x100);                 // r1+0x14, sth
a[1] = *(s16 *)(this + 0x102);
a[2] = *(s16 *)(this + 0x104);
fn_80064AF0(&p, &a, 1);

f31 = 12.0f;                                   // 0x80B32650

fn_801C0BD0(this + 0x7C, p[0], p[1], p[2]);
fn_801C0BD0(&m1, 0.0f, 0.0f, f31);             // 0x80B32558 x2
fn_801C0540(this + 0x7C, &m1, this + 0x7C);

u16 yaw = (u16)a[1]; fn_8016EDD0(this + 0x7C, &yaw);
u16 pitch = (u16)a[0]; fn_8016ECB0(this + 0x7C, &pitch);

fn_801C0BD0(&m2, 0.0f, 0.0f, -f31);
fn_801C0540(this + 0x7C, &m2, this + 0x7C);

u16 roll = (u16)a[2]; fn_8016EEF0(this + 0x7C, &roll);

fn_8016A170(this + 0x544, this + 0x7C);
fn_8016A150(this + 0x544, this + 0xDC);
fn_8016A1A0(this + 0x544, 0);
```

Rotation order is **the `+12` translate, then Y, then X, then the `-12`
translate, then Z**. The first translate sits between the position matrix and
the Y rotation, and the second sits between X and Z; a port that applies the
three rotations first, or puts both translates together, produces a different
matrix.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80B1D0EC` | `0x80064AF0` | `r3 = r1+0x1C`, `r4 = r1+0x14`, `r5 = 1` |
| `0x80B1D108` | `0x801C0BD0` | `r3 = this+0x7C`, `f1..f3 = p[0..2]` |
| `0x80B1D120` | `0x801C0BD0` | `r3 = r1+0x58`, `f1 = 0.0`, `f2 = 0.0`, `f3 = 12.0` |
| `0x80B1D130` | `0x801C0540` | `r3 = this+0x7C`, `r4 = r1+0x58`, `r5 = this+0x7C` |
| `0x80B1D144` | `0x8016EDD0` | `r3 = this+0x7C`, `r4 = r1+0x10` (`u16` of `+0x102`) |
| `0x80B1D158` | `0x8016ECB0` | `r3 = this+0x7C`, `r4 = r1+0x0C` (`+0x100`) |
| `0x80B1D16C` | `0x801C0BD0` | `r3 = r1+0x28`, `f1 = 0.0`, `f2 = 0.0`, `f3 = -12.0` |
| `0x80B1D17C` | `0x801C0540` | `r3 = this+0x7C`, `r4 = r1+0x28`, `r5 = this+0x7C` |
| `0x80B1D190` | `0x8016EEF0` | `r3 = this+0x7C`, `r4 = r1+0x08` (`+0x104`) |
| `0x80B1D19C` | `0x8016A170` | `r3 = this+0x544`, `r4 = this+0x7C` |
| `0x80B1D1A8` | `0x8016A150` | `r3 = this+0x544`, `r4 = this+0xDC` |
| `0x80B1D1B4` | `0x8016A1A0` | `r3 = this+0x544`, `r4 = 0` |

### Ambiguity

No blocker. The only `psq` pair is the `f31` prologue/epilogue spill with
`qr0` (the export's `unaff_GQR0` and `ldexpf` noise is cosmetic). The two
`0x801C0BD0` calls load `f3` from `f31` (12.0) and `-f31`; `f1`/`f2` come from
`0x80B32558` (0.0). Field roles (`+0x7C` matrix, `+0x544` render holder,
`+0xDC` offset vector) match the sibling builders in waves 129/130.

---

## 6. `0x80A39410` - `lift_rakka_ashiba` model + damage binder

* **TSV size:** 324 bytes
* **Reachable body:** 81 instructions; 11 `bl` + 1 `bctrl`; ends at
  `0x80A39550 blr` (the `bctrl` at `0x80A39508` is linked and returns into the
  tail of the body)
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a39410_FUN_80a39410.c`
* **Proposed signature:** `pub unsafe fn fn_80a39410(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x30(r1)`; LR at `+0x34`; `r31` at `+0x2C`, `r30` at `+0x28`,
`r29` at `+0x24`. `r29 = this`. Page register `r31 = 0x80B3E410`.
Locals: `r1+0x10` (model node), `r1+0x14` (damage animation handle),
`r1+0x0C` (copy of the model node), `r1+0x08` (copy of the damage handle).

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B3E410` | `char[]` | `"lift_rakka_ashiba"` (model/node name) |
| `0x80B3E3F4` | `char[]` | `"g3d/lift_rakka_ashiba.brres"` (archive path) |
| `0x80B2C26C` | `f32` | `0.0` (both animation rates/frames) |
| `0x80377C48` | word | **(ext)** animator reset argument |
| `0x8042A038` | pointer | **(ext)** archive manager; the loader takes `+4` |

### Behaviour

```c
fn_800690C0(this + 0x524, 0xFFFFFFFF, *(u32 *)0x80377C48, 0, 0x20);

u32 model = fn_800DF180(*(u32 *)0x8042A038 + 4,
                        0x80B3E410 /*"lift_rakka_ashiba"*/,
                        0x80B3E3F4 /*"g3d/lift_rakka_ashiba.brres"*/);
*(u32 *)(this + 0x540) = model;

u32 node = fn_80239E30(this + 0x540, 0x80B3E410);
fn_80169D90(this + 0x544, &node, this + 0x524, 0x23, 1, 0);
fn_80064BF0(this + 0x544);

u32 dmg = fn_8023A200(this + 0x540, 0x80B3E410);

fn_80166830(this + 0x584, &node, &dmg, this + 0x524, 0, 1);
fn_80166BD0(this + 0x584, this + 0x544, &dmg, 0, 1);

vt(this + 0x544, 0x18) (this + 0x584);         // r3 = +0x544, r4 = +0x584

fn_80166EF0(0.0f, this + 0x584, 0);            // 0x80B2C26C
fn_80166F10(0.0f, this + 0x584, 0);
fn_800690E0(this + 0x524);
```

`r30` holds the model node so the second binder can pass a copy at `r1+0x0C`;
`r31` holds the damage handle and supplies the copy at `r1+0x14`. The two
`0x80166EF0`/`0x80166F10` calls take the same `0.0` float and `r4 = 0`.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80A39444` | `0x800690C0` | `r3 = this+0x524`, `r4 = -1`, `r5 = *(0x80377C48)`, `r6 = 0`, `r7 = 0x20` |
| `0x80A39464` | `0x800DF180` | `r3 = *(0x8042A038)+4`, `r4 = 0x80B3E410`, `r5 = 0x80B3E3F4` |
| `0x80A39474` | `0x80239E30` | `r3 = this+0x540`, `r4 = 0x80B3E410` |
| `0x80A39498` | `0x80169D90` | `r3 = this+0x544`, `r4 = r1+0x10`, `r5 = this+0x524`, `r6 = 0x23`, `r7 = 1`, `r8 = 0` |
| `0x80A394A0` | `0x80064BF0` | `r3 = this+0x544` |
| `0x80A394AC` | `0x8023A200` | `r3 = this+0x540`, `r4 = 0x80B3E410` |
| `0x80A394D4` | `0x80166830` | `r3 = this+0x584`, `r4 = r1+0x0C`, `r5 = r1+0x08`, `r6 = this+0x524`, `r7 = 0`, `r8 = 1` |
| `0x80A394F0` | `0x80166BD0` | `r3 = this+0x584`, `r4 = this+0x544`, `r5 = r1+0x14`, `r6 = 0`, `r7 = 1` |
| `0x80A39508` | vtable `+0x18` of `*(this+0x544)` | `r3 = this+0x544`, `r4 = this+0x584` |
| `0x80A3951C` | `0x80166EF0` | `f1 = 0.0`, `r3 = this+0x584`, `r4 = 0` |
| `0x80A3952C` | `0x80166F10` | `f1 = 0.0`, `r3 = this+0x584`, `r4 = 0` |
| `0x80A39534` | `0x800690E0` | `r3 = this+0x524` |

### Ambiguity

No blocker. This is the `lift_rakka_ashiba` counterpart of wave-130 #3: the
same `+0x524/+0x540/+0x544/+0x584` holder chain, the same reset word
`0x80377C48`, and a second `0x8023A200` handle bound through `0x80166830` /
`0x80166BD0`. `0x80166830`/`0x80166BD0` are cited by address; their argument
roles (holder, model copy, handle copy, owner, flags) are inferred from the
register setup and from the same two calls in wave-130 #3.

---

## 7. `0x80AD42E0` - contact/pad record emitter

* **TSV size:** 320 bytes
* **Reachable body:** 80 instructions; 4 `bl` + 0 `bctrl`; ends at
  `0x80AD441C blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ad42e0_FUN_80ad42e0.c`
* **Proposed signature:** `pub unsafe fn fn_80ad42e0(this: usize)`
* **Confidence:** **medium / ready**

### Frame

`stwu r1,-0x30(r1)`; LR at `+0x34`; `r31` at `+0x2C`. `r31 = this`. The record
is 23 bytes at `r1+0x08..0x1E`:

| stack | record field |
|---|---|
| `r1+0x08` | `f32` x |
| `r1+0x0C` | `f32` y |
| `r1+0x10` | `f32` z |
| `r1+0x14` | `u8` kind (`+0x694`) |
| `r1+0x18` | `u32` state (`+0x854`) |
| `r1+0x1C` | `u8` pad = 0 |
| `r1+0x1D` | `u8` flag |
| `r1+0x1E` | `u8` count |

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B3094C` | `f32` | `14.0` (added to the record y in the raised arm) |

### Behaviour

```c
u8 *rec = (u8 *)(r1 + 0x08);                    // 23-byte record

if (fn_80022770(this, *(u32 *)(this + 0x854)) != 0)
    return;                                     // 0x80AD42FC..0x80AD4300

u32 n = fn_80022390(this);                      // r5 for both emitters
b32 flag = *(u32 *)(this + 0x858) != 0;

*(f32 *)(rec + 0x00) = *(f32 *)(this + 0xAC);
*(f32 *)(rec + 0x04) = *(f32 *)(this + 0xB0);
*(f32 *)(rec + 0x08) = *(f32 *)(this + 0xB4);
*(u8  *)(rec + 0x0C) = *(u8  *)(this + 0x694);  // lbz r4,1684(r31)
*(u32 *)(rec + 0x10) = *(u32 *)(this + 0x854);  // lwz r4,2132(r31)
*(u8  *)(rec + 0x14) = 0;

if (*(u32 *)(this + 0x858) != 0 && *(u8 *)(this + 0x689) == 0) {
    *(u8  *)(rec + 0x15) = 1;
    *(f32 *)(rec + 0x04) += 14.0f;              // 0x80B3094C
    flag = 0;
} else {
    *(u8  *)(rec + 0x15) = 0;
}

*(u8 *)(rec + 0x16) = 0;
if (*(u32 *)(this + 0x71C) != 0)
    *(u8 *)(rec + 0x16) = (u8)(*(u32 *)(this + 0x718) + 1);

if (*(u32 *)(this + 0x848) == 1 || n != 0)
    flag = 1;
if (*(u32 *)(this + 0x850) == 2 || *(u32 *)(this + 0x850) == 4)
    flag = 0;

if (flag)
    fn_80022780(this, rec, n, 0);
else
    fn_800226D0(this, rec, n);
```

The `+0x850` override runs **after** the `+0x848`/`n` test, so a state-2 or
state-4 actor always takes the `0x800226D0` arm even when `n != 0`.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80AD42F8` | `0x80022770` | `r3 = this`, `r4 = *(u32 *)(this+0x854)` |
| `0x80AD4308` | `0x80022390` | `r3 = this` |
| `0x80AD43F8` | `0x80022780` | `r3 = this`, `r4 = r1+0x08`, `r5 = n`, `r6 = 0` |
| `0x80AD4408` | `0x800226D0` | `r3 = this`, `r4 = r1+0x08`, `r5 = n` |

### Ambiguity

No blocker. The four callees are `main.dol` engine functions (`0x80022770` is a
"can this pad take a hit" test returning a byte, `0x80022390` a query
returning the pad index, `0x80022780`/`0x800226D0` two record sinks). The
record layout is fixed by the store offsets; the *meaning* of `flag`/`count`
is inferred. `+0x689` is a byte gate, `+0x718`/`+0x71C` a pointer/count pair,
`+0x848`/`+0x850` two state words.

---

## 8. `0x80B0DC40` - timered two-state machine (300-frame retry)

* **TSV size:** 320 bytes
* **Reachable body:** 80 instructions; 9 `bl` + 1 `bctrl`; ends at
  `0x80B0DD7C blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b0dc40_FUN_80b0dc40.c`
* **Proposed signature:** `pub unsafe fn fn_80b0dc40(this: usize)`
* **Confidence:** **medium / ready**

### Frame

`stwu r1,-0x10(r1)`; LR at `+0x14`; `r31` at `+0x0C`, `r30` at `+0x08`.
`r30 = this`; `r31` is the "should dispatch" flag.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B789C8` | state token | **(ext)** vtable `+0xD4` argument |

No in-REL float or table is read; the only immediates are `1`, `2`, `300` and
`0`.

### Behaviour

```c
if (*(u32 *)(this + 0x9AC) != 0)
    *(u32 *)(this + 0x9AC) -= 1;                // 0x80B0DC58..0x80B0DC68

if (*(u32 *)(this + 0x9AC) == 0) {
    if (*(u16 *)(this + 0x9DE) == 0) {
        if (fn_800B2EE0(2) == 0) {
            fn_80B0CEB0(this, 1);
            *(u16 *)(this + 0x9DE) = 1;
        } else {
            *(u32 *)(this + 0x9AC) = 300;
        }
    } else if (*(u16 *)(this + 0x9DE) == 1 && fn_80B0D150(this) != 0) {
        fn_80B0CEB0(this, 0);
        *(u32 *)(this + 0x9AC) = 300;
        *(u16 *)(this + 0x9DE) = 0;
    }
}

fn_80B0C890(this);

if (fn_80B0D4D0(this) != 0) {
    u32 w = *(u32 *)(this + 0x9B8);
    b32 flag;
    if (w == 0) {
        flag = fn_80B0D150(this) != 0;
    } else {
        flag = 1;
        fn_80B0CEB0(this, 0);
    }
    if (flag)
        vt(this, 0xD4) (0x80B789C8);            // r3 = this, r4 = token
}

fn_80B0CFD0(this);
```

`r0` is reused as the zero/one scratch throughout; the `300` is `li`-built
(`addi r0,r0,300` at `0x80B0DCAC` and `0x80B0DCDC`). The `u16` state `+0x9DE`
is set to `1` only after the `0x800B2EE0(2)` test succeeds.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80B0DC88` | `0x800B2EE0` | `r3 = 2` |
| `0x80B0DC9C` | `0x80B0CEB0` | `r3 = this`, `r4 = 1` |
| `0x80B0DCC4` | `0x80B0D150` | `r3 = this` |
| `0x80B0DCD8` | `0x80B0CEB0` | `r3 = this`, `r4 = 0` |
| `0x80B0DCF0` | `0x80B0C890` | `r3 = this` |
| `0x80B0DCF8` | `0x80B0D4D0` | `r3 = this` |
| `0x80B0DD18` | `0x80B0D150` | `r3 = this` |
| `0x80B0DD38` | `0x80B0CEB0` | `r3 = this`, `r4 = 0` |
| `0x80B0DD5C` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80B789C8` |
| `0x80B0DD64` | `0x80B0CFD0` | `r3 = this` |

### Ambiguity

No blocker. `0x80B0C890` is translated (it clears `+0x9E0` when the
`0x80097B30` overlap test succeeds); `0x80B0CEB0`, `0x80B0CFD0`,
`0x80B0D150` and `0x80B0D4D0` are untranslated and cited by address. The
`+0x9AC` word is a frame countdown, `+0x9DE` a two-phase flag, `+0x9B8` a
"mode" word whose non-zero value forces the dispatch. `0x80B789C8` is a
`main.dol` state token that is only passed through.

---

## 9. `0x80A37560` - `Wm_en_landsmoke_s` effect trigger

* **TSV size:** 296 bytes
* **Reachable body:** 74 instructions; 8 `bl` + 2 `bctrl`; ends at
  `0x80A37684 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a37560_FUN_80a37560.c`
* **Proposed signature:** `pub unsafe fn fn_80a37560(this: usize)`
* **Confidence:** **medium / ready**

### Frame

`stwu r1,-0x40(r1)`; LR at `+0x44`; `r31` at `+0x3C`, `r30` at `+0x38`,
`r29` at `+0x34`. `r29 = this`; page register `r31 = 0x80B2C100`. Scratch:
`r1+0x10/0x14/0x18` (position, z forced to `5500.0`), `r1+0x1C/0x20/0x24`
(scale triplet, all `1.4`), `r1+0x08/0x0C/0x10` (the sound position filled by
`0x8006A4B0`).

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2C160` | `f32` | `0.0` (`+0xE8`/`+0xEC` on each effect arm) |
| `0x80B2C1C4` | `f32` | `1.4` (all three scale components) |
| `0x80B2C1AC` | `f32` | `5500.0` (z passed to `0x8016C890`) |
| `0x80B3DDF8` | `char[]` | `"Wm_en_landsmoke_s"` (effect name, `r3`) |
| `0x80B70518` | state token | **(ext)** vtable `+0xD4` argument |
| `0x80429D5C` | pointer | **(ext)** sound system |

### Behaviour

```c
vt(this + 0x544, 0x1C);                        // r3 = this+0x544
fn_8006CD90(this);
fn_8006CDE0(this);
fn_800955F0(this);

if (fn_8006FA40(this + 0x1EC, 0) != 0)
    *(f32 *)(this + 0xE8) = 0.0f;               // 0x80B2C160

if (fn_80072BC0(this + 0x1EC) != 0) {
    pos[0] = *(f32 *)(this + 0xAC);             // r1+0x10
    pos[1] = *(f32 *)(this + 0xB0);             // r1+0x14
    pos[2] = 5500.0f;                           // r1+0x18, 0x80B2C1AC

    scl[0] = scl[1] = scl[2] = 1.4f;            // r1+0x1C..0x24, 0x80B2C1C4

    fn_8016C890(0x80B3DDF8 /*"Wm_en_landsmoke_s"*/, 0, &pos, 0, &scl);

    fn_8006A4B0(&snd, this + 0xAC);             // r3 = r1+0x08
    fn_80197F00(*(u32 *)0x80429D5C, 539, &snd, 0);

    *(f32 *)(this + 0xEC) = 0.0f;               // 0x80B2C160
    vt(this, 0xD4) (0x80B70518);                // r3 = this, r4 = token
}
```

The z of the effect position is a literal `5500.0`, **not** `this+0xB4`. The
scale triplet is three copies of the same `f1`. The sound position at
`r1+0x08` is filled after the `0x8016C890` call and overlaps the z slot at
`r1+0x10`; both are dead once the spawner has read them, so the overlap is
harmless but must not be reordered.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80A37594` | vtable `+0x1C` of `*(this+0x544)` | `r3 = this+0x544` |
| `0x80A3759C` | `0x8006CD90` | `r3 = this` |
| `0x80A375A4` | `0x8006CDE0` | `r3 = this` |
| `0x80A375AC` | `0x800955F0` | `r3 = this` |
| `0x80A375B8` | `0x8006FA40` | `r3 = this+0x1EC`, `r4 = 0` |
| `0x80A375D0` | `0x80072BC0` | `r3 = this+0x1EC` |
| `0x80A3761C` | `0x8016C890` | `r3 = 0x80B3DDF8`, `r4 = 0`, `r5 = r1+0x10`, `r6 = 0`, `r7 = r1+0x1C` |
| `0x80A37630` | `0x8006A4B0` | `r3 = r1+0x08`, `r4 = this+0xAC` |
| `0x80A37644` | `0x80197F00` | `r3 = *(0x80429D5C)`, `r4 = 539`, `r5 = r1+0x08`, `r6 = 0` |
| `0x80A37668` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80B70518` |

### Ambiguity

No blocker. `0x8016C890` is the by-name effect spawner (`name, 0, &pos, 0,
&scale`); the `539` (0x21B) is the smoke cue and `*(0x80429D5C)` the sound
system pointer, both values exact. `+0x1EC` is the shared body/physics object
used by the sibling `0x80A370D0` (#11). The two `+0xE8`/`+0xEC` zero stores
are a "stop the previous motion" pair.

---

## 10. `0x80AD0960` - two-position ground probe -> side byte 1/2/3

* **TSV size:** 296 bytes
* **Reachable body:** 74 instructions; 4 `bl` + 0 `bctrl`; ends at
  `0x80AD0A84 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ad0960_FUN_80ad0960.c`
* **Proposed signature:** `pub unsafe fn fn_80ad0960(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x50(r1)`; LR at `+0x54`; `f31` at `+0x40`, `f30` at `+0x30`,
`f29` at `+0x20` (each `stfd` + `psq_st qr0`); `r31` at `+0x1C`, `r30` at
`+0x18`, `r29` at `+0x14`, `r28` at `+0x10`. `r28 = this`.

### Constants

None. The body reads only actor fields:

| field | role |
|---|---|
| `+0xAC`, `+0xB0` | actor x, y |
| `+0x164`, `+0x168`, `+0x16C` | probe offsets dx, dy, dz |
| `+0x38F` | `u8` ground-probe layer |
| `+0x391` | `u8` "raised" flag |
| `+0x1E7` | `u8` result (1/2/3) |

### Behaviour

```c
f32 bx = *(f32 *)(this + 0xAC) + *(f32 *)(this + 0x164);   // f30
f32 by = *(f32 *)(this + 0xB0) + *(f32 *)(this + 0x168);   // f29
f32 dz = *(f32 *)(this + 0x16C);                           // f31
u8  layer = *(u8 *)(this + 0x38F);

u32 a0 = fn_80070BA0(bx + dz, by, layer);      // f1 = f30+f31, f2 = f29
u8  a1 = (u8)fn_80070BF0(bx + dz, by, layer);
u32 b0 = fn_80070BA0(bx - dz, by, layer);
u8  b1 = (u8)fn_80070BF0(bx - dz, by, layer);

if (((a0 & 0x400) != 0 && a1 >= 2) || ((b0 & 0x400) != 0 && b1 >= 2)) {
    if (*(u8 *)(this + 0x391) == 0)
        *(u8 *)(this + 0x1E7) = 1;             // 0x80AD0A30
    else
        *(u8 *)(this + 0x1E7) = 2;             // 0x80AD0A3C
} else {
    *(u8 *)(this + 0x1E7) = 3;                 // 0x80AD0A48
}
```

The two `fn_80070BF0` results are narrowed with `rlwinm rX,r3,0,24,31`
(`0x80AD09DC`, `0x80AD0A04`); the `0x400` bit is tested with
`rlwinm. r0,rX,0,21,21` (`0x80AD0A00`, `0x80AD0A14`). The comparisons are
`cmpli ...,2` + `bge` for `>= 2` on the first probe and `blt` for the mirror.
`fn_80070BA0` is called with `r3 = layer` (the `u8` at `+0x38F`), `f1` the x
probe, `f2` the y.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80AD09C0` | `0x80070BA0` | `r3 = *(u8 *)(this+0x38F)`, `f1 = bx+dz`, `f2 = by` |
| `0x80AD09D4` | `0x80070BF0` | same |
| `0x80AD09E8` | `0x80070BA0` | `r3 = layer`, `f1 = bx-dz`, `f2 = by` |
| `0x80AD09FC` | `0x80070BF0` | same |

### Ambiguity

No blocker. All three `psq` pairs are `f29`/`f30`/`f31` stack spills with
`qr0`; the export's `ldexpf` noise is the spill decode, not arithmetic. The
probe pair returns a bitfield in `r3` (bit `0x400` = "hit") and a byte code in
the second helper; both are compared against `2`. `+0x1E7` is written on every
path, so the function is total.

---

## 11. `0x80A370D0` - state 1 -> 2 plus `walk` clip bind

* **TSV size:** 292 bytes
* **Reachable body:** 73 instructions; 9 `bl` + 2 `bctrl`; ends at
  `0x80A371F0 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a370d0_FUN_80a370d0.c`
* **Proposed signature:** `pub unsafe fn fn_80a370d0(this: usize)`
* **Confidence:** **medium / ready**

### Frame

`stwu r1,-0x20(r1)`; LR at `+0x24`; `r31` at `+0x1C`. `r31 = this`. Local
`r1+0x08` is the animation handle returned by `0x8023A0B0`.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2C160` | `f32` | `0.0` (`+0xEC`, and the `0x80169F80` blend) |
| `0x80B2C1B4` | `f32` | `0.8` (`0x801680E0` rate) |
| `0x80B3DDA0` | `char[]` | `"walk"` (animation name) |
| `0x80B70498` | state token | **(ext)** vtable `+0xD4` argument |

### Behaviour

```c
vt(this + 0x544, 0x1C);                        // r3 = this+0x544
fn_8006CDE0(this);
fn_800955F0(this);

if (fn_80072BC0(this + 0x1EC) != 0)
    *(f32 *)(this + 0xEC) = 0.0f;               // 0x80B2C160

switch (*(u32 *)(this + 0x360)) {               // 0x80A37124
case 1:
    *(u32 *)(this + 0x5C0) -= 1;                 // addic. r0,r3,-1
    if (*(s32 *)(this + 0x5C0) < 1) {            // bgt over the body
        *(u8 *)(this + 0x348) = (u8)fn_80096240(this, this + 0xAC);
        *(u32 *)(this + 0x360) = 2;
    }
    break;
case 2:
    if (fn_80A369B0(this) != 0) {
        vt(this, 0xD4) (0x80B70498);            // r3 = this, r4 = token

        u32 anim = fn_8023A0B0(this + 0x540, 0x80B3DDA0 /*"walk"*/);
        fn_801651F0(this + 0x584, this + 0x544, &anim, 0);
        fn_80169F80(0.0f, this + 0x544, this + 0x584);
        fn_801680E0(0.8f, this + 0x584);
    }
    break;
default:
    break;
}
```

The decrement is an `addic.` (`0x80A37140`) and the store happens **before**
the `bgt`, so `+0x5C0` reaches 0 (not -1) on the transition frame. The side
byte written at `0x80A3715C` is the low byte of the `0x80096240` result
(`stb r3,840(r31)`), and `+0x360` is set to `2` in that same arm.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80A370F4` | vtable `+0x1C` of `*(this+0x544)` | `r3 = this+0x544` |
| `0x80A370FC` | `0x8006CDE0` | `r3 = this` |
| `0x80A37104` | `0x800955F0` | `r3 = this` |
| `0x80A3710C` | `0x80072BC0` | `r3 = this+0x1EC` |
| `0x80A37154` | `0x80096240` | `r3 = this`, `r4 = this+0xAC` |
| `0x80A3716C` | `0x80A369B0` | `r3 = this` |
| `0x80A37190` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80B70498` |
| `0x80A371A0` | `0x8023A0B0` | `r3 = this+0x540`, `r4 = 0x80B3DDA0` |
| `0x80A371B8` | `0x801651F0` | `r3 = this+0x584`, `r4 = this+0x544`, `r5 = r1+0x08`, `r6 = 0` |
| `0x80A371CC` | `0x80169F80` | `f1 = 0.0`, `r3 = this+0x544`, `r4 = this+0x584` |
| `0x80A371DC` | `0x801680E0` | `f1 = 0.8`, `r3 = this+0x584` |

### Ambiguity

No blocker. `0x80A369B0` is an unclaimed B row (152 bytes, the heading step) and
is cited by address. `+0x360` is the state word, `+0x5C0` a countdown,
`+0x348` the side byte, `+0x1EC` the body object shared with #9. The `"walk"`
string and the `0.8` rate are literal.

---

## 12. `0x80A628F0` - side-scaled speed init (external char table)

* **TSV size:** 292 bytes
* **Reachable body:** 73 instructions; 2 `bl` + 3 `bctrl`; ends at
  `0x80A62A10 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a628f0_FUN_80a628f0.c`
* **Proposed signature:** `pub unsafe fn fn_80a628f0(this: usize)`
* **Confidence:** **medium / conditional**

### Frame

`stwu r1,-0x30(r1)`; LR at `+0x34`; `f31` at `+0x20` (`stfd` + `psq_st qr0`);
`r31` at `+0x1C`, `r30` at `+0x18`. `r30 = this`; page register
`r31 = 0x80B2D2B0`. `r1+0x08/0x0C` are the two halves of the 2^52-biased
double.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2D2B0` | `f32` | `2.2` (speed multiplier) |
| `0x80B2D2B4` | `f32` | `2.7` (stored to `+0xF8`) |
| `0x80B2D2B8` | `f32` | `0.25` (stored to `+0x11C`) |
| `0x80B2D2BC` | `f32` | `0.35` (stored to `+0x114`) |
| `0x80B2D380` | `f32` | `1.0` (`f2` of the `0x80A61A80` call) |
| `0x80B2D3D8` | `f32` | `0.0` (`+0xE8`, `+0xEC`, `+0xF0`) |
| `0x80B2D3E8` | `f32` | `2.0` (scale when the kind test hits) |
| `0x80B2D454` | `f32` | `6.0` (base scale) |
| `0x80B2D458` | `f64` | `0x4330000080000000` = `2^52 + 2^31` (signed-int bias) |
| `0x8042C1A0` | `u8[?]` | **(ext)** side -> count table, indexed by the `0x80A61F90` byte |
| `0x80B71ED0` | type token | **(ext)** kind-test token |

### Behaviour

```c
f32 scale = 6.0f;                               // 0x80B2D454

u32 obj = vt(this + 0x394, 0x2C) (this + 0x394);        // bctrl
if (vt(obj, 0x14) (obj, 0x80B71ED0) != 0)               // bctrl
    scale = 2.0f;                               // 0x80B2D3E8

fn_80A61A80(scale, 1.0f, this, 4, 1);           // 0x80B2D380

u8 side = fn_80A61F90(this);                    // r3 -> clrlwi r5

u8  n = *(u8 *)(0x8042C1A0 + side);             // **(ext)**, lbzx
f32 v = (f32)n;                                 // stw/xoris + lfd - 2^52
f32 speed = v * 2.2f;                           // 0x80B2D2B0

*(u8  *)(this + 0x348) = side;
*(f32 *)(this + 0xE8) = 0.0f;                   // 0x80B2D3D8
*(f32 *)(this + 0xEC) = 0.0f;
*(f32 *)(this + 0xF0) = 0.0f;
*(f32 *)(this + 0x11C) = 0.25f;                 // 0x80B2D2B8
*(f32 *)(this + 0x114) = 0.35f;                 // 0x80B2D2BC
*(f32 *)(this + 0xF4) = speed;
*(f32 *)(this + 0xF8) = 2.7f;                   // 0x80B2D2B4

vt(this, 0x2B0) (this);                         // 0x60 -> +0x2B0
```

The integer->float conversion is the standard 2^52 idiom:
`stw 0x43300000,r1+0x08`; `xoris r0,r4,0x8000`; `stw r0,r1+0x0C`;
`lfd f0,r1+0x08`; `fsubs f2,f0,[0x80B2D458]`. Because the low word is built
with `xoris`, the result is exactly the **unsigned** byte `n` for `n < 2^31`;
the bias word `0x4330000080000000` is required, not `0x4330000000000000`.

The stores of `+0xF4` and `+0xF8` are in that order (speed first, `2.7`
second), and `+0x114`/`+0x11C` are written *before* the speed - keep the order
if the port is byte-compared.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80A6292C` | vtable `+0x2C` of `*(this+0x394)` | `r3 = this+0x394` |
| `0x80A62944` | vtable `+0x14` of the returned object | `r3 = obj`, `r4 = 0x80B71ED0` |
| `0x80A62968` | `0x80A61A80` | `r3 = this`, `r4 = 4`, `r5 = 1`, `f1 = scale` (6.0/2.0), `f2 = 1.0` |
| `0x80A62970` | `0x80A61F90` | `r3 = this` -> `u8` side |
| `0x80A629F0` | vtable `+0x2B0` of `*(this+0x60)` | `r3 = this` |

### Ambiguity

**Conditional.** Everything except one datum is literal: the two vtable
dispatches only read the kind token and branch on its non-zero result, and
`0x80A61A80`/`0x80A61F90` are translated ledger entries. The blocker is the
byte table at `0x8042C1A0`, which lives in `main.dol` (the REL image ends at
`0x80B6E434`). The byte is indexed by the side returned from `0x80A61F90`,
widened to float and multiplied by `2.2` - so it is both an index and an
arithmetic operand under the wave-128/129/130 rule. Read the table before
claiming this body byte-exact; the rest of the port can be written now against
the address. The range of `side` is whatever `0x80A61F90` can return (its
ledger row says it derives the side from the `+0x732` phase), so size the table
read from that function, not from an assumed `0..7`.
`+0x394` is the owner object, `+0x60` the actor vtable, `+0x348` the side byte
(the same field used by #11), and `+0xE8..+0xF8` the velocity/speed block.

---

## Verification

* 12 addresses, all absent from `ledger/functions.csv` and from the wave-128,
  wave-129 and wave-130 specs; each present in `functions.tsv` at the quoted
  size. No ledger row, CSV, Rust file or other doc was written by this pass.
* Every address has a Ghidra export at
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
* Reachable spans were walked from the entry through every conditional
  branch, following `bl` as a call and stopping only at `blr`, an indirect
  branch, or a `b` to a `functions.tsv` entry. All 12 terminate inside their
  own extent (zero tail calls) and every in-REL `bl` target (14 calls) is a
  `functions.tsv` entry.
* `psq_l`/`psq_st` inventory: 23 spill pairs over 4 bodies (#4, #5, #10, #12),
  all `qr0`, all `f29`/`f30`/`f31` prologue/epilogue saves. No
  `GQR1..GQR7` quantiser, no `mtspr GQRn`.
* Every in-REL constant in the tables was read from
  `work/rels/elf/d_enemiesNP.elf` at `(addr - 0x80A00000) + 0x1000`; values
  quoted are the shipped values. `(ext)` addresses are above `0x80B6E434` or
  below `0x80A00000` and are not present in this image.
* Split: **11 ready, 1 conditional** (`0x80A628F0`, external byte table
  `0x8042C1A0` consumed arithmetically).
