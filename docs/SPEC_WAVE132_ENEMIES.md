# Wave 132 - implementation specification for 12 class-B `d_enemiesNP` bodies

## What this page is

Wave 127 (`docs/ANALYSIS_WAVE127_ENEMIES.md`) reclassified 384 formerly-skipped
`d_enemiesNP` addresses: 173 class **B** (reconstructible now), 160 class C and
51 class D. Waves 128, 129, 130 and 131
(`docs/SPEC_WAVE128_ENEMIES.md` .. `docs/SPEC_WAVE131_ENEMIES.md`) specified
ranks 1-48 of that pool.

This lane is the continuation: **ranks 49-61** of the same wave-127 ordering,
i.e. the next twelve class-B addresses that are still neither in
`ledger/functions.csv` nor in any existing spec heading. One rank inside the
window has been consumed since wave 127 - rank 54, `0x80ADDD10` (256 bytes, 64
reachable instructions), is a `translated` ledger row now - so the tranche runs
one rank deeper and ends at rank 61.

It is **read-only**. It writes no Rust, no ledger row and no other doc. Each
body below was decoded from `work/rels/elf/d_enemiesNP.elf` with the corrected
`+0x1000` segment shift, cross-checked against the Ghidra export at the cited
path and against `work/ghidra/rel-d_enemiesNP/export/functions.tsv`. Every
constant that lives inside the REL image was read out of the ELF itself, so the
float values below are the shipped values, not guesses.

All twelve were re-checked against the `address` column of
`ledger/functions.csv` (absent) and against `functions.tsv` (present). Nothing
here duplicates a wave-128 .. wave-131 claim.

## Selection

Ordering is the wave-127 "reachable instruction count" - the only size metric
that is not inflated by the TSV extent quirk - with ties broken by TSV size and
then by address. The method was re-validated against this tranche's
predecessors: walking the twelve wave-131 addresses reproduces all twelve
published counts exactly, and walking the 48 spec'd addresses of waves 128-131
reproduces all 48 (byte sizes included).

| rank | address | TSV size | reachable insns | body character | status |
|---:|---|---:|---:|---|---|
| 49 | `0x80AB9AF0` | 284 | 71 | `lift_trampoline` variant init + `0x8007FA40` emitter | **ready** |
| 50 | `0x80A7EB60` | 280 | 70 | ramp-driven scale block, two 4-float tables | **ready** |
| 51 | `0x80A0E450` | 276 | 69 | player distance probe + pad-field reset | **ready** |
| 52 | `0x80AB22D0` | 276 | 69 | fishbone per-frame step (SE, aim, two vtable slots) | **ready** |
| 53 | `0x80A37700` | 256 | 64 | 3-state `+0x360` machine with `walk` reset | **ready** |
| 55 | `0x80A36FC0` | 248 | 62 | `go_out_ed` clip + `Wm_en_landsmoke_s` trigger, state 1 | **ready** |
| 56 | `0x80B1B3F0` | 248 | 62 | per-frame step, fixed-point pivot target | **conditional** |
| 57 | `0x80A368B0` | 244 | 61 | 4-slot proximity test with a player mask byte | **ready** |
| 58 | `0x80AFFAF0` | 244 | 61 | `ice` / `ice_C1` model + `attack_*` rig binder | **ready** |
| 59 | `0x80A2E970` | 240 | 60 | 8-slot respawn allocator (random x jitter, 1.2 scale) | **ready** |
| 60 | `0x80A600B0` | 240 | 60 | side-flip gate with a lazily built 4-float table | **ready** |
| 61 | `0x80A35370` | 232 | 58 | `run` clip at a random frame offset | **ready** |

Rank 54 (`0x80ADDD10`) is skipped: it is already `translated` in
`ledger/functions.csv` (`game::enemies`, 256-byte body, GQR4 rule). It is the
only ledger-consuming address inside the window.

**Split: 11 ready, 1 conditional.**

"Conditional" is used exactly as in waves 128-131: a named datum in `main.dol`
that is **consumed arithmetically or used as an index** and must be read before
a byte-exact port. Only #7 does that - `0x8042C1A0`, a signed-byte table
indexed by the side byte and multiplied by `13653` (`0x3555`). Every other
external address in this set is either (a) a `main.dol` callee called by
address, (b) a pointer or state token that is only dereferenced, stored or
passed through as an argument, or (c) a mask word ANDed against a value where
the literal expression is reproduced in full (the wave-129 #3 precedent).
#8's `0x80429CA8` is a mask of that (c) kind, and #11's `0x80B71E90` /
`0x80B71E98` block is written from in-REL constants by the body itself before it
is read, so both stay ready.

The four risk classes that made waves 110-126 skip a body are all absent:

* no `psq_l`/`psq_st` other than `f31`/`f30` stack spills with `qr0`
  (2 spill pairs in 2 bodies, both `qr0`, both prologue/epilogue saves);
* no `bctr` jump table - every conditional branch target is inside its body
  (checked mechanically: zero out-of-body `bc` targets), and every `bctrl` is a
  vtable dispatch with a resolved `+0xNN` slot;
* no incoming float argument - no body reads `f1`/`f2`/`f3` before writing it;
* no unledgered callee landmine - every in-REL `bl` target is a
  `functions.tsv` entry.

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
* **`addi rX,r0,imm` is `li rX,imm`.** Several bodies below build small
  integers and the `0x43300000` page word this way; the source register `r0` is
  not read when `RA == 0`.
* **`rlwinm` field extraction is the common source of mis-reads.** In this
  build `rlwinm rX,rY,SH,31,31` extracts bit `32-SH` of `rY` (so `SH=20` is bit
  12, `SH=24` is bit 8, `SH=1` is bit 31), and `rlwinm rX,rY,24,28,31` is
  `(rY >> 8) & 0xF`. Both were verified against the Ghidra exports at
  `0x80AB9AF0` and `0x80AB9B50`.
* **Three-operand single-precision ops differ.** `fadds`/`fsubs`/`fdivs` take
  `FRT,FRA,FRB`, but `fmuls` takes `FRT,FRA,FRC`: the third operand is encoded
  in bits 6-10, not bits 11-15. Reading `fmuls f7,f6,f3` as `f7,f6,f0` is the
  single easiest way to get #1, #2 and #12 wrong.
* Helper roles below are the ledger's, not new claims:
  `0x8005F900` = resolve a player-table entry by index (returns a pointer);
  `0x80096240` = ground probe on an `(x,y)` pair, returns the side byte;
  `0x80072BC0` = "0x8006DB30 status" query on the shared body object;
  `0x80072CB0` = "0x8006FA40 update" on the same object;
  `0x80070CB0` = `+0x88 & 0x1FE000` non-zero test;
  `0x8015F4F0` = `step_short_clamped_wrap(target_ptr, target, step)`;
  `0x8008C330` = collision area register; `0x8008C380` = its unregister twin;
  `0x800DF180` = load a model `(archive, name, path)`;
  `0x80239E30` = find a node by name in a loaded model;
  `0x8023A0B0` = `AnmChr` animation by name; `0x8023A270` = `AnmTexSrt` by name;
  `0x80169D90` = bind a model record `(holder, &node, owner, mode, 1, 0)`;
  `0x801651F0` = attach an animation; `0x80169F80` = apply an animation blend;
  `0x801680E0` = set an animation rate; `0x80168FE0` = set an animation frame;
  `0x80167420` = third-class child-group build; `0x80167800` = child commit;
  `0x8016C890` = spawn an effect by name `(name, 0, &pos, 0, &scale)`;
  `0x80197F00` = player-SE spawn `(system, cue, &pos, 0)`;
  `0x80198660` = SE row scan `(system, tag, id, &pos, 0)`;
  `0x8007FA40` = layer bootstrap `(f1..f4, layer, actor, 0, 0, 0, 3, 0, 0)`;
  `0x8007F900` = release it; `0x800B2ED0`/`0x800B2EE0` = small range-limited
  value used as a jitter count (the second is the linker duplicate of the
  first; the ledger role line for it does not match this call pattern).

## Symbolic helpers used below

```text
vt(obj, slot)                  # (*(u32 *)obj)->vt[slot], receiver r3 = obj
fn_80a7e630(this)              # untranslated B row (212 bytes): contact test
fn_80a36a50(this) -> u32       # untranslated B row (32 bytes): state-2 gate
fn_80a0e160(this)              # untranslated B row (36 bytes): 9-insn trampoline
fn_80ab1830(this)              # untranslated: last step of the fishbone chain
fn_80ab1900(this)              # untranslated: matrix refresh of the fishbone
fn_80ab1a00(this)              # untranslated (8 bytes)
fn_80ab9c10(this, entry)       # translated: g3d/lift_trampoline.brres binder
fn_80ab1b10(this)              # translated: player query + state gates
fn_80ab1d60(this, ptr, s16)    # translated: aim via 0x801611C0
fn_80ab1df0(this)              # translated: tail call through +0x544 slot +0x1C
fn_80a5fe90(this, &v, &w)      # translated: 0x80070BF0 probe, returns a bool
fn_80b1b0e0(this)              # untranslated B row: trampoline to 0x8015F4F0
fn_8007fa40(...) / fn_8007f900(...)   # translated stage layer helpers
```

---

## 1. `0x80AB9AF0` - `lift_trampoline` variant init + layer emitter

* **TSV size:** 284 bytes
* **Reachable body:** 71 instructions; 3 `bl` + 0 `bctrl`; ends at
  `0x80AB9C08 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ab9af0_FUN_80ab9af0.c`
* **Proposed signature:** `pub unsafe fn fn_80ab9af0(this: usize) -> u32`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x20(r1)`; LR at `+0x24`; `r31` at `+0x1C`, `r30` at `+0x18`,
`r29` at `+0x14`. `r29 = this`. Page register `r31 = 0x80B2FF20`.
`r30` holds the selected 12-byte table entry (`r31 + 12 * k`, `k` from bit 12 of
`this+4`). Returns `1` in `r3`.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2FF20` | `u32[3]` | entry 0: `{0x80B56918, 0x80B56928, 64.0}` |
| `0x80B2FF2C` | `u32[3]` | entry 1: `{0x80B56930, 0x80B56948, 128.0}` |
| `0x80B56918` | `char[]` | `"lift_trampoline"` |
| `0x80B56928` | `char[]` | `"ridden"` |
| `0x80B56930` | `char[]` | `"lift_trampoline_long"` |
| `0x80B56948` | `char[]` | `"ridden_long"` |
| `0x80B2FF50` | `f32` | `0.0` (stored to `+0xB4`; `f1` of `0x8007FA40`) |
| `0x80B2FF54` | `f32` | `0.5` (`+0x53C` when the nibble is zero) |
| `0x80B2FF58` | `f32` | `64.0` (stored to `+0x320`) |
| `0x80B2FF5C` | `f32` | `128.0` (stored to `+0x318`) |
| `0x80B2FF60` | `f32` | `16.0` (stored to `+0x31C`) |
| `0x80B2FF64` | `f32` | `1.0` (dividend of the `+0x53C` division) |
| `0x80B2FF68` | `f32` | `8.0` (`f2` of `0x8007FA40`) |
| `0x80B2FF6C` | `f32` | `-8.0` (`f4` of `0x8007FA40`) |
| `0x80B2FF70` | `f64` | `0x4330000000000000` = `2^52` (unsigned-int bias) |

The 12-byte entries are `{name, node, scale}`: the first two words are the two
strings consumed by `0x80AB9C10`, the third word is the float used here.

### Behaviour

```c
*(f32 *)(this + 0xB4) = 0.0f;                    // 0x80AB9AF0..0x80AB9B1C

u32 k = (*(u32 *)(this + 4) >> 12) & 1;          // rlwinm r0,r0,20,31,31
u8 *entry = (u8 *)0x80B2FF20 + 12 * k;
fn_80AB9C10(this, entry);                        // 0x80AB9B34

f32 scale = *(f32 *)(entry + 8);                 // 64.0 or 128.0

*(f32 *)(this + 0x318) = 128.0f;                 // 0x80B2FF5C
*(f32 *)(this + 0x31C) = 16.0f;                  // 0x80B2FF60
*(f32 *)(this + 0x320) = 64.0f;                  // 0x80B2FF58
*(f32 *)(this + 0x324) = 0.0f;                   // 0x80B2FF50
*(f32 *)(this + 0xD0)  = 0.5f * scale;           // 0x80B2FF54 * entry[8]
*(f32 *)(this + 0xD4)  = 0.0f;
*(f32 *)(this + 0xD8)  = 0.0f;
*(f32 *)(this + 0xDC)  = 1.0f;                   // 0x80B2FF64
*(f32 *)(this + 0xE0)  = 1.0f;
*(f32 *)(this + 0xE4)  = 1.0f;

u32 nib = (*(u32 *)(this + 4) >> 8) & 0xF;       // rlwinm. r3,r0,24,28,31
if (nib == 0)
    *(f32 *)(this + 0x53C) = 0.5f;               // 0x80B2FF54
else
    *(f32 *)(this + 0x53C) = 1.0f / (f32)nib;    // 0x43300000 | nib, - 2^52
                                                 // (the `rlwinm.` already set
                                                 //  cr0, so the test is `cr0`
                                                 //  from the nibble itself)

fn_8007FA40(0.0f, 8.0f, scale, -8.0f,
            this + 0x458, this, 0, 0, 0, 3, 0, 0);
fn_8007F900(this + 0x458);
return 1;
```

Store order is exactly `+0x320, +0x324, +0x318, +0x31C, +0xD0, +0xD4, +0xD8,
+0xDC, +0xE0, +0xE4`, then the `+0x53C` arm, then the two calls. Keep it if the
port is byte-compared: the `+0xD0` multiply uses `f3` (`entry[+8]`) which is
loaded *before* the calls and survives them, while `f0` is reloaded from
`0x80B2FF50` for `+0xB4`.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80AB9B34` | `0x80AB9C10` | `r3 = this`, `r4 = 0x80B2FF20 + 12*k` |
| `0x80AB9BE0` | `0x8007FA40` | `r3 = this+0x458`, `r4 = this`, `r5..r7 = 0`, `r8 = 3`, `r9 = r10 = 0`, `f1 = 0.0`, `f2 = 8.0`, `f3 = scale`, `f4 = -8.0` |
| `0x80AB9BE8` | `0x8007F900` | `r3 = this+0x458` |

### Ambiguity

No external datum. Bit 12 of `this+4` (the same flag word used by `0x80A628F0`
and the sibling inits) selects the `lift_trampoline` or `lift_trampoline_long`
record pair; the scale word of the selected record is multiplied by `0.5` into
`+0xD0` and is also the third float argument of the emitter. `0x80AB9C10` is a
translated ledger entry (`g3d/lift_trampoline.brres` with the
`lift_trampoline/ridden` and `lift_trampoline_long/ridden_long` records), so the
strings are not re-derived here. The `+0x318..+0x324` quad `{128, 16, 64, 0}`
has no arithmetic use inside the body - it is an argument block for later
per-frame code - so its *meaning* is inferred (a four-component bound), but
every value is literal.

---

## 2. `0x80A7EB60` - ramp-driven scale block, two 4-float tables

* **TSV size:** 280 bytes
* **Reachable body:** 70 instructions; 2 `bl` + 1 `bctrl`; ends at
  `0x80A7EC74 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a7eb60_FUN_80a7eb60.c`
* **Proposed signature:** `pub unsafe fn fn_80a7eb60(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x20(r1)`; LR at `+0x24`; `f31` at `+0x10` (`stfd` + `psq_st qr0`,
`psq_l` + `lfd` in the epilogue); `r31` at `+0x0C`, `r30` at `+0x08`.
`r30 = this`; page register `r31 = 0x80B2E1D8`. `f31` is the ramp value that
every store in the tail block is multiplied by.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2E284` | `f32` | `0.004166666883975267` = `1/240` (ramp step) |
| `0x80B2E248` | `f32` | `1.0` (ramp ceiling; also `+0x194`) |
| `0x80B2E200` | `f32[4]` | `{0.0, -8.0, 6.0, 8.0}` (table A, used when `+0x610 == 0`) |
| `0x80B2E224` | `f32[4]` | `{0.0, -16.0, 6.0, 16.0}` (table B, used otherwise) |
| `0x80B2E254` | `f32` | `-8.0` (`+0x188` multiplier) |
| `0x80B2E258` | `f32` | `8.0` (`+0x18C` multiplier) |
| `0x80B2E25C` | `f32` | `-1.0` (`+0x190`) |
| `0x80B732F8` | state token | **(ext)** vtable `+0xD4` argument when the ramp saturates |

### Behaviour

```c
f32 r = *(f32 *)(this + 0xDC) + (1.0f / 240.0f);   // 0x80B2E284
if (r >= 1.0f) {                                   // fcmpo / cror 2,1,2 / bne
    r = 1.0f;
    vt(this, 0xD4) (this, 0x80B732F8);             // r3 = this, r4 = token
}
*(f32 *)(this + 0xDC) = r;
*(f32 *)(this + 0xE0) = r;
*(f32 *)(this + 0xE4) = r;

fn_80A7E630(this);

f32 *t = (*(u32 *)(this + 0x610) == 0) ? (f32 *)0x80B2E200
                                       : (f32 *)0x80B2E224;
f32 t1 = t[1];                                     // -8.0 / -16.0
f32 t2 = t[2];                                     // 6.0
f32 t3 = t[3];                                     // 8.0 / 16.0

*(f32 *)(this + 0x16C) = t2 * r;
*(f32 *)(this + 0x170) = t3 * r;
*(f32 *)(this + 0x168) = t1 * r;
*(f32 *)(this + 0x188) = -8.0f * r;                // 0x80B2E254
*(f32 *)(this + 0x18C) =  8.0f * r;                // 0x80B2E258
*(f32 *)(this + 0x190) = -1.0f;                    // 0x80B2E25C
*(f32 *)(this + 0x194) =  1.0f;                    // 0x80B2E248

fn_8008C330(this + 0x148);
```

The saturation test is the compiler's NaN-safe "greater or equal" idiom:
`fcmpo cr0,r,1.0`; `cror 2,1,2` (`EQ = GT | EQ`); `bne` over the clamp. It
therefore clamps when `r >= 1.0` **ordered** and skips when `r` is NaN. The
`psq_st`/`psq_l` pair is the `f31` prologue/epilogue spill with `qr0`; the
export's `unaff_GQR0`/`ldexpf` noise is cosmetic.

Store order is `0x16C, 0x170, 0x168, 0x188, 0x18C, 0x190, 0x194`; the three
`t*` slots are written in a different order than they are read from the table.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80A7EBBC` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80B732F8` |
| `0x80A7EBD0` | `0x80A7E630` | `r3 = this` |
| `0x80A7EC54` | `0x8008C330` | `r3 = this+0x148` |

### Ambiguity

No external datum. The two tables differ only in the y/z magnitudes
(`-8/8` vs `-16/16`), selected by the `+0x610` word - the "large/small
collision box" pair used by the sibling scale writers in this module. `f31` is
a 0..1 ramp and is the multiplier of the five scaled tail stores; a port that
treats `+0x188`/`+0x18C` as literal `-8`/`8` is wrong (they are `-8*r`/`8*r`),
while `+0x190`/`+0x194` really are the literals `-1.0`/`1.0`.
`0x80A7E630` is an unclaimed B row (212 bytes, the contact test) and is cited
by address.

---

## 3. `0x80A0E450` - player distance probe + pad-field reset

* **TSV size:** 276 bytes
* **Reachable body:** 69 instructions; 4 `bl` + 1 `bctrl`; ends at
  `0x80A0E560 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a0e450_FUN_80a0e450.c`
* **Proposed signature:** `pub unsafe fn fn_80a0e450(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x30(r1)`; LR at `+0x34`; `r31` at `+0x2C`. `r31 = this`.
Scratch: `r1+0x08/0x0C/0x10` (a dead 3-float vector copy), `r1+0x14/0x18/0x1C`
(a dead position copy - only `r1+0x18`, the player y, is read back).

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2AD10` | `f32` | `4.0` (subtracted from the probe vector's y) |
| `0x80B2AD60` | `f32` | `0.15000000596046448` (stored to `+0x5EC`) |
| `0x80429DC0` | pointer | **(ext)** collision/pad manager, `r3` of `0x80076D70` |

### Behaviour

```c
fn_8008C380(this + 0x148);                       // collision area unregister

u32 player = fn_8005F900((s8)*(u8 *)(this + 0x614));
f32 py = *(f32 *)(player + 0xB0);
f32 dy = py - *(f32 *)(this + 0xB0);

*(f32 *)(this + 0x5E8) = fabsf(dy);              // fabs + frsp

u32 *v = vt(player, 0x3C0) (player);             // r3 = player
f32 ny = fabsf((*(f32 *)((u8 *)v + 4) - 4.0f) - py);
if (*(f32 *)(this + 0x5E8) >= ny)                // fcmpo / cror 2,1,2 / bne
    *(f32 *)(this + 0x5E8) = ny;                 // keep the smaller of the two

u16 pad = fn_80076D70(*(u32 *)0x80429DC0, *(u32 *)(this + 0), (s8)*(u8 *)(this + 0x614));
*(u16 *)(this + 0x610) = pad;
*(u8  *)(this + 0x615) = 0;
*(f32 *)(this + 0x5EC) = 0.15f;                  // 0x80B2AD60
*(u16 *)(this + 0x612) = 0;
*(u32 *)(this + 0x5F4) = 0;
*(u32 *)(this + 0x5F8) = 0;
*(u32 *)(this + 0x5FC) = 0;
*(u32 *)(this + 0x600) = 0;
*(u32 *)(this + 0x604) = 0;
*(u32 *)(this + 0x608) = 0;

fn_80A0E160(this);
```

Two notes on the probe block. The vtable call is reached with `r3 = player`
(the loads before it only touch floating-point registers), and the returned
pointer is a 3-float vector: `v[1]` is its y. The `-4.0` offset is applied to
`v[1]` *before* the subtraction, not to the result. Both `fabs` results pass
through `frsp` before the compare, so the comparison is single precision.

`fn_8008C380` precedes the query, so the collision area is unregistered on
every path. The function is total: all eight tail fields are written
unconditionally.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80A0E468` | `0x8008C380` | `r3 = this+0x148` |
| `0x80A0E474` | `0x8005F900` | `r3 = (s8)*(u8 *)(this+0x614)` |
| `0x80A0E4B0` | vtable `+0x3C0` of `*(player+0x60)` | `r3 = player` |
| `0x80A0E510` | `0x80076D70` | `r3 = *(u32 *)0x80429DC0`, `r4 = *(u32 *)(this+0)`, `r5 = (s8)*(u8 *)(this+0x614)` |
| `0x80A0E54C` | `0x80A0E160` | `r3 = this` |

### Ambiguity

No blocker. `0x80429DC0` is dereferenced and passed as the first argument of
`0x80076D70`; its *contents* are never inspected here, so it is a pass-through
pointer, not a consumed datum. `+0x614` is a signed byte player index
(`lbz` + `extsb`), `+0x610` receives the low half of the `0x80076D70` result,
`+0x5E8` is a running minimum of two y distances, and `+0x5EC`/`+0x5F4..+0x608`
are a timer/velocity block that is zeroed. The dead stack copies at
`r1+0x08..0x1C` are kept in the listing above because the exporter emits them;
they are not read again.

---

## 4. `0x80AB22D0` - fishbone per-frame step

* **TSV size:** 276 bytes
* **Reachable body:** 69 instructions; 7 `bl` + 3 `bctrl`; ends at
  `0x80AB23E0 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ab22d0_FUN_80ab22d0.c`
* **Proposed signature:** `pub unsafe fn fn_80ab22d0(this: usize)`
* **Confidence:** **medium / ready**

### Frame

`stwu r1,-0x20(r1)`; LR at `+0x24`; `r31` at `+0x1C`, `r30` at `+0x18`,
`r29` at `+0x14`. `r29 = this`. `r31` caches `*(u32 *)(this+0)` (the actor id
passed to the SE call) and `r30` caches `*(u32 *)0x80429D5C` (the sound system).
Local `r1+0x08` is the 12-byte position record filled by `0x8006A4B0`.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80429D5C` | pointer | **(ext)** sound system, `r3` of `0x80198660` |
| `0x80B54BBA` | `s16` | **(in-REL)** `0` - third argument of `0x80AB1D60` |
| `0x80B75090` | state token | **(ext)** vtable `+0xD4` argument on aim success |
| `0x207` | immediate | SE cue `519` |

`0x80B54BBA` is the two-byte pad in front of the `"fishbone_break"` string at
`0x80B54BC0`; its shipped value is zero.

### Behaviour

```c
if (*(u16 *)(this + 0x998) != 0)
    *(u16 *)(this + 0x998) -= 1;                 // 0x80AB22EC..0x80AB22FC

fn_80AB1B10(this);

u32 id  = *(u32 *)(this + 0x00);
u32 snd = *(u32 *)0x80429D5C;
fn_8006A4B0((u8 *)(r1 + 0x08), this + 0xAC);
fn_80198660(snd, 519, id, (u8 *)(r1 + 0x08), 0);

u32 aimed = fn_80AB1D60(this, this + 0x8B4, *(s16 *)0x80B54BBA /* 0 */);
if (aimed != 0 && *(u16 *)(this + 0x998) == 0)
    vt(this, 0xD4) (this, 0x80B75090);           // r3 = this, r4 = token

fn_80AB1900(this);
vt(this + 0x73C, 0x14) (this + 0x73C);           // r3 = this+0x73C
if (*(u32 *)(this + 0x988) == 1)
    vt(this + 0x794, 0x14) (this + 0x794);       // r3 = this+0x794
fn_80AB1DF0(this);
fn_80AB1830(this);
```

The decrement is a load/compare/store triple, not `addic.`, so a zero stays
zero. The second `+0x998` test re-loads the field *after* `0x80AB1D60`, so a
call that touches the timer is observed. Both vtable dispatches pass the
sub-object address as `r3` (the object word is loaded from the same address,
then `r3` is recomputed with `addi`).

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80AB2304` | `0x80AB1B10` | `r3 = this` |
| `0x80AB231C` | `0x8006A4B0` | `r3 = r1+0x08`, `r4 = this+0xAC` |
| `0x80AB2334` | `0x80198660` | `r3 = *(0x80429D5C)`, `r4 = 519`, `r5 = *(u32 *)(this+0)`, `r6 = r1+0x08`, `r7 = 0` |
| `0x80AB2348` | `0x80AB1D60` | `r3 = this`, `r4 = this+0x8B4`, `r5 = (s16)*(0x80B54BBA)` |
| `0x80AB2378` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80B75090` |
| `0x80AB2380` | `0x80AB1900` | `r3 = this` |
| `0x80AB2394` | vtable `+0x14` of `*(this+0x73C)` | `r3 = this+0x73C` |
| `0x80AB23B4` | vtable `+0x14` of `*(this+0x794)` | `r3 = this+0x794` |
| `0x80AB23BC` | `0x80AB1DF0` | `r3 = this` |
| `0x80AB23C4` | `0x80AB1830` | `r3 = this` |

### Ambiguity

No blocker, but three callee roles are inferred. `0x80AB1B10`,
`0x80AB1D60` and `0x80AB1DF0` are translated ledger entries; `0x80AB1900` and
`0x80AB1830` are unclaimed bodies cited by address (the last two steps of the
same per-frame chain). `0x73C`/`0x794` are the two pad sub-objects already
written by `0x80AB2620`; the `+0x988 == 1` guard selects the second one. `+0x998`
is a frame countdown, `+0x8B4` an aim state block. `*(u32 *)(this+0)` is quoted
as "the actor id word" because that is the only reading consistent with
`0x80198660`'s third argument.

---

## 5. `0x80A37700` - 3-state `+0x360` machine with `walk` reset

* **TSV size:** 256 bytes
* **Reachable body:** 64 instructions; 5 `bl` + 2 `bctrl`; ends at
  `0x80A377FC blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a37700_FUN_80a37700.c`
* **Proposed signature:** `pub unsafe fn fn_80a37700(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x10(r1)`; LR at `+0x14`; `r31` at `+0x0C`. `r31 = this`.
No float temporaries; the only local is the `vtable` pointer in `r12`.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2C160` | `f32` | `0.0` (stored to `+0xEC`) |
| `0x80B70498` | state token | **(ext)** vtable `+0xD4` argument in state 3 |

`0x80B70498` is the same token that wave-131 #11 (`0x80A370D0`) dispatches, so
states 3 here and the transition arm there enter the same handler.

### Behaviour

```c
vt(this + 0x544, 0x1C) (this + 0x544);           // r3 = this+0x544
fn_8006CDE0(this);
fn_800955F0(this);

if (fn_80072BC0(this + 0x1EC) != 0)
    *(f32 *)(this + 0xEC) = 0.0f;                // 0x80B2C160

switch (*(u32 *)(this + 0x360)) {
case 1: {
    u8 old = *(u8 *)(this + 0x348);
    *(u8 *)(this + 0x86C) = old;
    u8 now = (u8)fn_80096240(this, this + 0xAC);
    *(u8 *)(this + 0x348) = now;
    if (now == old)
        *(u32 *)(this + 0x360) = 3;              // 0x80A377A8
    else
        *(u32 *)(this + 0x360) = 2;              // 0x80A3779C
    break;
}
case 2:
    if (fn_80A36A50(this) != 0)
        *(u32 *)(this + 0x360) = 3;
    break;
case 3:
    vt(this, 0xD4) (this, 0x80B70498);
    break;
}
```

The case-1 comparison is `cmpl` (unsigned) of the `clrlwi`-narrowed probe
result against the saved byte, so both sides are in `0..255` and the compare
never sees a stale high byte. The side byte is written to `+0x348` **before**
the branch, so both arms leave the new side in place.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80A37724` | vtable `+0x1C` of `*(this+0x544)` | `r3 = this+0x544` |
| `0x80A3772C` | `0x8006CDE0` | `r3 = this` |
| `0x80A37734` | `0x800955F0` | `r3 = this` |
| `0x80A3773C` | `0x80072BC0` | `r3 = this+0x1EC` |
| `0x80A37784` | `0x80096240` | `r3 = this`, `r4 = this+0xAC` |
| `0x80A377B8` | `0x80A36A50` | `r3 = this` |
| `0x80A377E8` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80B70498` |

### Ambiguity

No external datum. `0x80A36A50` is an unclaimed B row (32 bytes, a state-2
gate) and is cited by address. `+0x360` is the state word, `+0x348` the side
byte (the same field used by #7, #11 and #12), `+0x86C` the previous side,
`+0x1EC` the shared body object. The case order (`1`, `2`, `3`, default) is a
compare chain, not a jump table.

---

## 6. `0x80A36FC0` - `go_out_ed` clip + `Wm_en_landsmoke_s` trigger, state 1

* **TSV size:** 248 bytes
* **Reachable body:** 62 instructions; 7 `bl` + 0 `bctrl`; ends at
  `0x80A370B4 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a36fc0_FUN_80a36fc0.c`
* **Proposed signature:** `pub unsafe fn fn_80a36fc0(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x40(r1)`; LR at `+0x44`; `r31` at `+0x3C`, `r30` at `+0x38`.
`r30 = this`; page register `r31 = 0x80B2C100`. Scratch: `r1+0x08` (animation
handle), `r1+0x18/0x1C/0x20` (effect position), `r1+0x24/0x28/0x2C` (effect
scale), `r1+0x10` (sound position).

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B3DDEC` | `char[]` | `"go_out_ed"` (animation name) |
| `0x80B3DDF8` | `char[]` | `"Wm_en_landsmoke_s"` (effect name) |
| `0x80B2C160` | `f32` | `0.0` (`0x80169F80` blend) |
| `0x80B2C158` | `f32` | `1.0` (`0x801680E0` rate) |
| `0x80B2C1B0` | `f32` | `1.7000000476837158` (all three scale components) |
| `0x80B2C1AC` | `f32` | `5500.0` (effect z) |
| `0x80429D5C` | pointer | **(ext)** sound system |
| `0x21B` | immediate | SE cue `539` |

### Behaviour

```c
u32 anim = fn_8023A0B0(this + 0x540, 0x80B3DDEC /*"go_out_ed"*/);   // r1+0x08
fn_801651F0(this + 0x584, this + 0x544, (u8 *)(r1 + 0x08), 1);
fn_80169F80(0.0f, this + 0x544, this + 0x584);     // 0x80B2C160
fn_801680E0(1.0f, this + 0x584);                   // 0x80B2C158

f32 s = *(f32 *)0x80B2C1B0;                        // 1.7
*(f32 *)(r1 + 0x24) = s;
*(f32 *)(r1 + 0x28) = s;
*(f32 *)(r1 + 0x2C) = s;

*(f32 *)(r1 + 0x18) = *(f32 *)(this + 0xAC);
*(f32 *)(r1 + 0x1C) = *(f32 *)(this + 0xB0);
*(f32 *)(r1 + 0x20) = 5500.0f;                     // 0x80B2C1AC

fn_8016C890(0x80B3DDF8 /*"Wm_en_landsmoke_s"*/, 0, (u8 *)(r1 + 0x18), 0,
            (u8 *)(r1 + 0x24));

fn_8006A4B0((u8 *)(r1 + 0x10), this + 0xAC);
fn_80197F00(*(u32 *)0x80429D5C, 539, (u8 *)(r1 + 0x10), 0);

*(u32 *)(this + 0x5C0) = 10;
*(u32 *)(this + 0x360) = 1;
```

The effect z is the literal `5500.0` (the sibling `0x80A37560` in wave-131 #9
uses the same literal). The three scale slots are three copies of one `lfs`.
`fn_801651F0` gets `r6 = 1` here (the sibling in #12 passes `0`).

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80A36FEC` | `0x8023A0B0` | `r3 = this+0x540`, `r4 = 0x80B3DDEC` |
| `0x80A37004` | `0x801651F0` | `r3 = this+0x584`, `r4 = this+0x544`, `r5 = r1+0x08`, `r6 = 1` |
| `0x80A37014` | `0x80169F80` | `f1 = 0.0`, `r3 = this+0x544`, `r4 = this+0x584` |
| `0x80A37020` | `0x801680E0` | `f1 = 1.0`, `r3 = this+0x584` |
| `0x80A37064` | `0x8016C890` | `r3 = 0x80B3DDF8`, `r4 = 0`, `r5 = r1+0x18`, `r6 = 0`, `r7 = r1+0x24` |
| `0x80A37078` | `0x8006A4B0` | `r3 = r1+0x10`, `r4 = this+0xAC` |
| `0x80A3708C` | `0x80197F00` | `r3 = *(0x80429D5C)`, `r4 = 539`, `r5 = r1+0x10`, `r6 = 0` |

### Ambiguity

No blocker. `"go_out_ed"` is the shipped spelling (9 characters, NUL at
`0x80B3DDF5`; it is *not* `go_out_edge`). The two tail stores (`+0x5C0 = 10`,
`+0x360 = 1`) start the countdown that `0x80A37700` state 1 consumes, so #5 and
#6 are a pair and should be ported together. Roles of `0x801651F0`/
`0x80169F80`/`0x801680E0` are the ledger's; `+0x540/+0x544/+0x584` is the
model/animation/holder triple used by every binder in this module.

---

## 7. `0x80B1B3F0` - per-frame step with a fixed-point pivot target

* **TSV size:** 248 bytes
* **Reachable body:** 62 instructions; 7 `bl` + 2 `bctrl`; ends at
  `0x80B1B4E4 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b1b3f0_FUN_80b1b3f0.c`
* **Proposed signature:** `pub unsafe fn fn_80b1b3f0(this: usize)`
* **Confidence:** **medium / conditional**

### Frame

`stwu r1,-0x10(r1)`; LR at `+0x14`; `r31` at `+0x0C`, `r30` at `+0x08`.
`r30 = this`; `r31` saves the `0x80B1B0E0` result across the pivot call.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B324B4` | `f32` | `0.0` (stored to `+0xEC`) |
| `0x80B32488` | `s16[8]` | `{12743, -12743, 13653, -13653, 120, 180, 13653, 0}` (pivot table) |
| `0x3555` | immediate | `13653` multiplier of the external byte |
| `0x600` | immediate | `1536` step passed to `0x8015F4F0` |
| `0x8042C1A0` | `s8[]` | **(ext)** side -> fixed-point count table (indexed by `+0x348`) |
| `0x80B79138` | state token | **(ext)** vtable `+0xD4` argument when both gates pass |

`0x80B32488` entries are read with `lhax r4,r5,side*2`, i.e. the table is
`u16`-strided and signed. The eight values are what the image holds; only the
entry for the live `+0x348` side is used.

### Behaviour

```c
vt(this + 0x544, 0x1C) (this + 0x544);           // r3 = this+0x544
fn_8006CDE0(this);
fn_800955F0(this);

fn_80072CB0(this + 0x1EC, 0);                    // r4 = 0
fn_80072BC0(this + 0x1EC);
if (fn_80070CB0(this + 0x1EC) != 0)
    *(f32 *)(this + 0xEC) = 0.0f;                // 0x80B324B4

u32 g1 = fn_80B1B0E0(this);                      // r31

u8  side = *(u8 *)(this + 0x348);
s16 tgt = (s16)((s32)(s8)*(0x8042C1A0 + side) * 13653   // lbzx + extsb + mulli
                - *(s16 *)(0x80B32488 + side * 2));    // lhax
u32 g2 = fn_8015F4F0(this + 0x5D2, tgt, 0x600);

if (g1 != 0 && g2 != 0)
    vt(this, 0xD4) (this, 0x80B79138);           // r3 = this, r4 = token
```

The pivot computation is 32-bit before the narrowing: the product
`byte * 13653` is formed with `mulli`, the table entry is subtracted with
`subf`, and only then is the result `extsh`-narrowed into `r4`. The step is the
literal `0x600`, materialised *after* `r5` was used as the table base.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80B1B418` | vtable `+0x1C` of `*(this+0x544)` | `r3 = this+0x544` |
| `0x80B1B420` | `0x8006CDE0` | `r3 = this` |
| `0x80B1B428` | `0x800955F0` | `r3 = this` |
| `0x80B1B434` | `0x80072CB0` | `r3 = this+0x1EC`, `r4 = 0` |
| `0x80B1B43C` | `0x80072BC0` | `r3 = this+0x1EC` |
| `0x80B1B444` | `0x80070CB0` | `r3 = this+0x1EC` |
| `0x80B1B460` | `0x80B1B0E0` | `r3 = this` |
| `0x80B1B4A0` | `0x8015F4F0` | `r3 = this+0x5D2`, `r4 = (s16)tgt`, `r5 = 0x600` |
| `0x80B1B4CC` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80B79138` |

### Ambiguity

**Conditional.** Everything except one datum is literal: the three status
helpers, the `0x80B1B0E0` gate and the `0x8015F4F0` step are all explicit, and
`0x80B32488` is inside this REL with its eight entries quoted above. The
blocker is `0x8042C1A0`, a signed-byte table in `main.dol` (the REL image ends
at `0x80B6E434`). Its entry is **used as an index and consumed
arithmetically** - sign-extended, multiplied by `13653` and subtracted from the
in-REL table value - so the pivot target cannot be typed byte-exactly without
reading that table for the reachable `+0x348` values. This is the same table
that blocked wave-131 #12, and the same table is read at both sites
(`0x80A628F0` widens it to float, here it is a fixed-point angle), so one read
unlocks both. `+0x5D2` is the pivot angle field, `+0x348` the side byte, `+0x1EC`
the shared body object, `+0x544` the animation holder.

---

## 8. `0x80A368B0` - 4-slot proximity test with a player mask byte

* **TSV size:** 244 bytes
* **Reachable body:** 61 instructions; 1 `bl` + 0 `bctrl`; ends at
  `0x80A369A0 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a368b0_FUN_80a368b0.c`
* **Proposed signature:** `pub unsafe fn fn_80a368b0(this: usize) -> u32`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x40(r1)`; LR at `+0x44`; `f31` at `+0x30`, `f30` at `+0x20` (each
`stfd` + `psq_st qr0`); `r31` at `+0x1C`, `r30` at `+0x18`, `r29` at `+0x14`,
`r28` at `+0x10`. `r28 = this`; `r31 = 1` (the mask seed); `r30 = 0x80430000`;
`r29` is the slot index.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2C1A8` | `f32` | `160.0` (y threshold) |
| `0x80B2C14C` | `f32` | `256.0` (x threshold) |
| `0x80429CA8` | `u8` | **(ext)** bitmask of active player slots |

### Behaviour

```c
f32 ty = 160.0f;                                  // 0x80B2C1A8
f32 tx = 256.0f;                                  // 0x80B2C14C

for (u32 i = 0; i != 4; ++i) {                    // 0x80A36900..0x80A36970
    u32 *p = fn_8005F900(i);                      // 0x8005F900(i)
    if (p == 0) continue;
    if ((*(u8 *)0x80429CA8 & (1u << i)) == 0) continue;
    if (fabsf(*(f32 *)((u8 *)p + 0xAC) - *(f32 *)(this + 0xAC)) >= tx) continue;
    if (fabsf(*(f32 *)((u8 *)p + 0xB0) - *(f32 *)(this + 0xB0)) >= ty) continue;
    return 1;
}
return 0;
```

`fn_8005F900` is called with `r3 = i` (the loop counter, `or r3,r29,r29`).
The mask shift is `slw r0,r31,r0` with `r0 = clrlwi(i, 24)`, i.e. `1 << i`. The
two rejection tests are `bge` on the `fcmpo` result, so a NaN distance falls
through to the next slot (it does not return 1). Both distances pass through
`fabs` + `frsp`.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80A36904` | `0x8005F900` | `r3 = i` (0..3) |

### Ambiguity

No external arithmetic datum. `0x80429CA8` is a `main.dol` mask that is only
ANDed against `1 << i` to decide which slots count; the literal expression is
reproduced above, which is byte-exact for any mask value (the wave-129 #3
precedent for masks). The `psq_st`/`psq_l` pairs are `f30`/`f31` stack spills
with `qr0`. `+0xAC`/`+0xB0` are the actor x/y; the thresholds are a 256 by 160
box.

---

## 9. `0x80AFFAF0` - `ice` / `ice_C1` model + `attack_*` rig binder

* **TSV size:** 244 bytes
* **Reachable body:** 61 instructions; 7 `bl` + 0 `bctrl`; ends at
  `0x80AFFBE0 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80affaf0_FUN_80affaf0.c`
* **Proposed signature:** `pub unsafe fn fn_80affaf0(this: usize, owner: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x30(r1)`; LR at `+0x34`; `r31` at `+0x2C`, `r30` at `+0x28`,
`r29` at `+0x24`, `r28` at `+0x20`. `r28 = this`; `r29 = owner` (the incoming
`r4`); page register `r30 = 0x80B65C20`. Locals: `r1+0x08` (AnmTexSrt handle),
`r1+0x0C` (node copy), `r1+0x10` (node), `r1+0x14` (srt handle copy).

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B65C2C` | `char[]` | `"g3d/ice.brres"` (archive path, `r5`) |
| `0x80B65C3C` | `char[]` | `"ice"` (model name, `r4`) |
| `0x80B65C40` | `char[]` | `"ice_C1"` (node name) |
| `0x80B65C48` | `char[]` | `"attack_fast"` (first rig animation) |
| `0x80B65C54` | `char[]` | `"attack_slow"` (second rig animation) |
| `0x80B65C20` | word | `0x80AFF790` - head word of the block, **not read here** |
| `0x8042A038` | pointer | **(ext)** archive manager; the loader takes `+4` |
| `0x24` | immediate | model-record mode `36` |

`0x80B65C20` is the block base: `r30+0x0C` = path, `r30+0x1C` = name,
`r30+0x20` = node name. The two animation strings are not referenced by this
body; they are the pair the rig uses later.

### Behaviour

```c
u32 model = fn_800DF180(*(u32 *)0x8042A038 + 4,
                        0x80B65C3C /*"ice"*/,
                        0x80B65C2C /*"g3d/ice.brres"*/);
*(u32 *)(this + 0x00) = model;

u32 node = fn_80239E30(this, 0x80B65C40 /*"ice_C1"*/);
*(u32 *)(r1 + 0x10) = node;

fn_80169D90(this + 0x04, (u8 *)(r1 + 0x10), owner, 0x24, 1, 0);
fn_80064BF0(this + 0x04);

u32 srt = fn_8023A270(this, 0x80B65C40 /*"ice_C1"*/);
*(u32 *)(this + 0x44) = srt;
*(u32 *)(r1 + 0x08) = srt;
*(u32 *)(r1 + 0x0C) = node;

fn_80167420(this + 0x48, (u8 *)(r1 + 0x0C), (u8 *)(r1 + 0x08), owner, 0, 1);

*(u32 *)(r1 + 0x14) = *(u32 *)(this + 0x44);
fn_80167800(this + 0x48, this + 0x04, (u8 *)(r1 + 0x14), 0, 0);
```

Argument order for `0x800DF180` is `(archive, name, path)`, so `r4` is `"ice"`
and `r5` is `"g3d/ice.brres"` - the reverse of the register order in the
disassembly. `owner` is forwarded but never dereferenced here.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80AFFB30` | `0x800DF180` | `r3 = *(0x8042A038)+4`, `r4 = 0x80B65C3C`, `r5 = 0x80B65C2C` |
| `0x80AFFB40` | `0x80239E30` | `r3 = this`, `r4 = 0x80B65C40` |
| `0x80AFFB64` | `0x80169D90` | `r3 = this+4`, `r4 = r1+0x10`, `r5 = owner`, `r6 = 0x24`, `r7 = 1`, `r8 = 0` |
| `0x80AFFB6C` | `0x80064BF0` | `r3 = this+4` |
| `0x80AFFB78` | `0x8023A270` | `r3 = this`, `r4 = 0x80B65C40` |
| `0x80AFFBA0` | `0x80167420` | `r3 = this+0x48`, `r4 = r1+0x0C`, `r5 = r1+0x08`, `r6 = owner`, `r7 = 0`, `r8 = 1` |
| `0x80AFFBC0` | `0x80167800` | `r3 = this+0x48`, `r4 = this+4`, `r5 = r1+0x14`, `r6 = 0`, `r7 = 0` |

### Ambiguity

No blocker. This is the `ice` counterpart of wave-131 #6: the same
`0x800DF180` → `0x80239E30` → `0x80169D90`/`0x80064BF0` chain, then an
`AnmTexSrt` handle bound through `0x80167420`/`0x80167800`. `0x8042A038` is
dereferenced with a fixed `+4`; it is the archive manager, not an unresolved
base. The two rig animations (`attack_fast`, `attack_slow`) are not referenced
by this body - they belong to the same `0x80B65C20` block, so they are listed
for the cluster but must not be emitted here.

---

## 10. `0x80A2E970` - 8-slot respawn allocator

* **TSV size:** 240 bytes
* **Reachable body:** 60 instructions; 1 `bl` + 0 `bctrl`; ends at
  `0x80A2EA5C blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a2e970_FUN_80a2e970.c`
* **Proposed signature:** `pub unsafe fn fn_80a2e970(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x20(r1)`; LR at `+0x24`; `r31` at `+0x1C`, `r30` at `+0x18`.
`r30 = this`. The loop runs under `mtspr CTR, 8` and uses `r5` as the slot
pointer (stride `0x88`) and `r4` as the index. `r1+0x08/0x0C` are the two
halves of the `2^52`-biased double.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2BC5C` | `f32` | `4.0` (x inset) |
| `0x80B2BC78` | `f32` | `1.2000000476837158` (the three scale words) |
| `0x80B2BC80` | `f64` | `0x4330000000000000` = `2^52` (unsigned bias) |

The only immediates are `3` (the timer), `8` (both the random range and the
loop count) and `0x88` (slot stride).

### Behaviour

```c
if (*(u32 *)(this + 0xB50) != 0) {
    *(u32 *)(this + 0xB50) -= 1;                  // cooldown
    return;
}

*(u32 *)(this + 0xB50) = 3;

for (u32 i = 0; i != 8; ++i) {                    // CTR = 8, 0x80A2E9B8
    u8 *slot = (u8 *)this + i * 0x88;
    if (*(u16 *)(slot + 0x788) != 0) continue;

    *(u16 *)(slot + 0x788) = 1;
    u32 r = fn_800B2EE0(8);                       // r3 = 8
    f32 x = (*(f32 *)(this + 0xAC) - 4.0f) + (f32)r;
    *(f32 *)(slot + 0x770) = x;
    *(u32 *)(slot + 0x774) = *(u32 *)(this + 0xB0);   // raw word copy
    *(u32 *)(slot + 0x778) = *(u32 *)(this + 0xB4);   // raw word copy
    *(f32 *)(slot + 0x77C) = 1.2f;
    *(f32 *)(slot + 0x780) = 1.2f;
    *(f32 *)(slot + 0x784) = 1.2f;
    return;
}
```

The `y` and `z` writes are word copies of the actor fields, not recomputed
floats. The `x` jitter is `x - 4.0 + (float)rand(8)`, i.e. a 0..7 offset applied
to the inset position. The two table halves are `0x788 + i*0x88` (the "slot
taken" flag) and `0x770..0x784` (position + scale) inside the same 136-byte
stride.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80A2E9D8` | `0x800B2EE0` | `r3 = 8` |

### Ambiguity

No blocker. `0x800B2EE0` is the linker duplicate of `0x800B2ED0` and is
translated; this module only ever uses its return value as a range-limited
count (the sibling `0x80B0DC40` tests `0x800B2EE0(2)` against zero, and here the
result multiplies a frame count), so it behaves as a small PRNG. The ledger's
role line for `0x800B2ED0` reads "setter 0x80161460 on the scale record", which
does not match this call pattern - treat the return as an opaque `0..n-1` value
until the `main.dol` body is confirmed. The `2^52` conversion uses the
*unsigned* bias (`addis r0,0x4330` with the raw random word as the low half), so
the jitter is `(f32)r` for `r` in `0..7`. `+0xB50` is a 3-frame cooldown,
`+0x788` the per-slot occupancy flag, `+0x770` the per-slot spawn position. The
8-slot array and the `0x88` stride are literal.

---

## 11. `0x80A600B0` - side-flip gate with a lazily built 4-float table

* **TSV size:** 240 bytes
* **Reachable body:** 60 instructions; 2 `bl` + 1 `bctrl`; ends at
  `0x80A6019C blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a600b0_FUN_80a600b0.c`
* **Proposed signature:** `pub unsafe fn fn_80a600b0(this: usize)`
* **Confidence:** **medium / ready**

### Frame

`stwu r1,-0x20(r1)`; LR at `+0x24`; `r31` at `+0x1C`, `r30` at `+0x18`.
`r30 = this`; page register `r31 = 0x80B2D228`. Locals `r1+0x08`/`r1+0x0C` are
the two probe floats handed to `0x80A5FE90`.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2D278` | `f32` | `8.0` (lazy table word 0) |
| `0x80B2D27C` | `f32` | `24.0` (lazy table words 1 and 3) |
| `0x80B2D280` | `f32` | `-8.0` (lazy table word 2) |
| `0x80B2D230` | `u32[2]` | `{0, 2}` (per-side byte written to `+0x5B8`) |
| `0x80B71E90` | `u8` | **(ext)** lazy-init guard byte |
| `0x80B71E98` | `f32[4]` | **(ext)** the lazily built table: `{8.0, 24.0, -8.0, 24.0}` |

### Behaviour

```c
if (*(s8 *)0x80B71E90 == 0) {                     // 0x80A600D4..0x80A60108
    *(f32 *)0x80B71E98 = 8.0f;                    // 0x80B2D278
    *(f32 *)(0x80B71E98 + 4) = 24.0f;             // 0x80B2D27C
    *(f32 *)(0x80B71E98 + 8) = -8.0f;             // 0x80B2D280
    *(f32 *)(0x80B71E98 + 12) = 24.0f;            // 0x80B2D27C
    *(u8 *)0x80B71E90 = 1;
}

vt(this + 0x544, 0x1C) (this + 0x544);           // lwzu r12,0x544(r3)
fn_800955F0(this);

u8 side = *(u8 *)(this + 0x348);
f32 a = *(f32 *)(0x80B71E98 + side * 8 + 0);
f32 b = *(f32 *)(0x80B71E98 + side * 8 + 4);
*(f32 *)(r1 + 0x08) = a;
*(f32 *)(r1 + 0x0C) = b;

if (fn_80A5FE90(this, (u8 *)(r1 + 0x08), (u8 *)(r1 + 0x0C)) == 0) {
    u8 s2 = side ^ 1;
    *(u8 *)(this + 0x348) = s2;
    u32 w = *(u32 *)(0x80B2D230 + s2 * 4);
    *(f32 *)(this + 0xE8) = -*(f32 *)(this + 0xE8);
    *(u8 *)(this + 0x5B8) = (u8)w;
}
```

The two probe floats are the selected `(a,b)` pair from the 8-byte side
stride: side 0 reads `{8.0, 24.0}`, side 1 reads `{-8.0, 24.0}`. The
`lwzu r12,0x544(r3)` both loads the vtable and advances `r3` to `this+0x544`,
so the dispatch receiver is the holder.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80A60118` | vtable `+0x1C` of `*(this+0x544)` | `r3 = this+0x544` |
| `0x80A60120` | `0x800955F0` | `r3 = this` |
| `0x80A60154` | `0x80A5FE90` | `r3 = this`, `r4 = r1+0x08`, `r5 = r1+0x0C` |

### Ambiguity

No external arithmetic datum. The `main.dol` block `0x80B71E90/0x80B71E98` is
read only after this body has written it from in-REL constants, so the table
content is fully determined by `0x80B2D278/0x80B2D27C/0x80B2D280`; the guard
byte only avoids a redundant re-store. A port must reproduce the guard and the
four stores, because the block is shared across actors (a second actor sees the
already-initialised table). `0x80A5FE90` is a translated ledger entry: it probes
`0x80070BF0` with the promoted float pair and the direction byte `+0x38F`, and
returns a bool. `+0x348` is the side byte, `+0xE8` the lateral speed, `+0x5B8`
a per-side mode byte (`0` or `2`).

---

## 12. `0x80A35370` - `run` clip at a random frame offset

* **TSV size:** 232 bytes
* **Reachable body:** 58 instructions; 6 `bl` + 0 `bctrl`; ends at
  `0x80A35454 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a35370_FUN_80a35370.c`
* **Proposed signature:** `pub unsafe fn fn_80a35370(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x30(r1)`; LR at `+0x34`; `r31` at `+0x2C`, `r30` at `+0x28`.
`r30 = this`. `r1+0x08` is the animation handle; `r1+0x10..0x24` hold the
`fctiwz` double and the two biased conversion words.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B3D994` | `char[]` | `"run"` (animation name) |
| `0x80B2C090` | `f32` | `1.0` (rate and blend) |
| `0x80B2C0F0` | `f64` | `0x4330000080000000` = `2^52 + 2^31` (unsigned bias) |
| `0x66666667` | immediate | signed divide-by-10 magic |

### Behaviour

```c
u32 anim = fn_8023A0B0(this + 0x540, 0x80B3D994 /*"run"*/);   // r1+0x08
fn_801651F0(this + 0x584, this + 0x544, (u8 *)(r1 + 0x08), 0);
fn_801680E0(1.0f, this + 0x584);                  // 0x80B2C090
fn_80169F80(1.0f, this + 0x544, this + 0x584);

u32 r  = fn_800B2ED0(10);                         // li r3,10
s32 n  = (s32)*(f32 *)(this + 0x5AC);             // fctiwz

s32 q  = n / 10;                                  // mulhw 0x66666667 / srawi 2
                                                  // + sign fix (truncates to 0)
q     ^= 0x80000000;                              // unsigned-bias scratch
r     ^= 0x80000000;
f32 f = (f32)q * (f32)r;                          // both via 0x43300000 - bias
fn_80168FE0(f, this + 0x584);
```

The division is the compiler's signed magic sequence: `mulhw r6,0x66666667,n`;
`srawi r0,r6,2`; `rlwinm r4,r0,1,31,31` (the sign bit of the quotient);
`add r0,r0,r4` - i.e. truncation toward zero, not a floor. `n` comes from
`fctiwz` of the `+0x5AC` float, so a negative frame count divides toward zero.
Both conversions use the `2^52 + 2^31` bias with the `xoris`-flipped low word,
so the operands are treated as unsigned 32-bit.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80A35394` | `0x8023A0B0` | `r3 = this+0x540`, `r4 = 0x80B3D994` |
| `0x80A353AC` | `0x801651F0` | `r3 = this+0x584`, `r4 = this+0x544`, `r5 = r1+0x08`, `r6 = 0` |
| `0x80A353BC` | `0x801680E0` | `f1 = 1.0`, `r3 = this+0x584` |
| `0x80A353CC` | `0x80169F80` | `f1 = 1.0`, `r3 = this+0x544`, `r4 = this+0x584` |
| `0x80A353D4` | `0x800B2ED0` | `r3 = 10` |
| `0x80A3543C` | `0x80168FE0` | `f1 = (f32)(n/10) * (f32)rand(10)`, `r3 = this+0x584` |

### Ambiguity

No blocker. `0x800B2ED0` is translated; the value it returns is used here as a
`0..9` count (the sibling `0x80B0DC40` tests `0x800B2EE0(2)` against zero, so it
behaves as a small PRNG - the ledger's role line "setter 0x80161460 on the scale
record" does not describe this call pattern). The frame offset is
`(s32)(*(f32 *)(this+0x5AC)) / 10 * rand(10)` - the quotient, not the raw
frame, is the multiplier, which is the one place a naive port goes wrong. The
rate and blend are both the literal `1.0`, and `fn_801651F0` gets `r6 = 0`
(compare #6, which passes `1`).

---

## Verification

* 12 addresses. All absent from `ledger/functions.csv` (checked against the
  `address` column of every row) and from the headings of
  `docs/SPEC_WAVE128_ENEMIES.md` .. `docs/SPEC_WAVE131_ENEMIES.md`. Each is
  present in `work/ghidra/rel-d_enemiesNP/export/functions.tsv` at the quoted
  size. No ledger row, CSV, Rust file or other doc was written by this pass.
* Every address has a Ghidra export at
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
* Reachable spans were walked from the entry through every conditional branch,
  following `bl` as a call and stopping only at `blr`, an indirect branch, or a
  `b` to a `functions.tsv` entry. All 12 terminate inside their own extent
  (zero tail calls) and every in-REL `bl` target is a `functions.tsv` entry.
  The walker reproduces the 48 published instruction counts of waves 128-131
  exactly (and all 48 of their TSV sizes), so the rank ordering above is
  the same metric the earlier waves used.
* `psq_l`/`psq_st` inventory: 2 spill pairs over 2 bodies (#2, #8), all `qr0`,
  both `f30`/`f31` prologue/epilogue saves. No `GQR1..GQR7` quantiser, no
  `mtspr GQRn`.
* Every in-REL constant in the tables was read from
  `work/rels/elf/d_enemiesNP.elf` at `(addr - 0x80A00000) + 0x1000`; values
  quoted are the shipped values. `(ext)` addresses are above `0x80B6E434` and
  are not present in this image.
* Every in-REL `b`/`bc` target is inside its own body; every `bctrl` is a
  vtable dispatch at a fixed `+0xNN` slot of a receiver that is resolvable in
  the same body.
* Split: **11 ready, 1 conditional** (`0x80B1B3F0`, external `s8` table
  `0x8042C1A0` indexed by the side byte and multiplied by `13653`).
