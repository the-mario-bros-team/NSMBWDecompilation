# Wave 142 - `game::bases` - the eight largest remaining bodies

Read-only specification lane. No Rust file, no `ledger/functions.csv` row and no
existing document was modified. The only file written is this one.

Wave 141 claimed the eight largest untranslated rows of
`work/ghidra/rel-d_basesNP/export/functions.tsv` that were left after wave 140.
This wave continues one rank lower. The window is now 1.1-1.5 KB per body
(10,400 bytes / 2,600 instructions), so the same eight-row width is kept and
every row gets a full field map instead of a summary.

## The eight

| # | address | bytes | words | export file | verdict |
|--:|---:|---:|---:|---|:--:|
| 1 | `0x806D09B0` | 1,508 | 377 | `806d09b0_FUN_806d09b0.c` | ready (medium) |
| 2 | `0x806B8380` | 1,500 | 375 | `806b8380_FUN_806b8380.c` | ready (medium) |
| 3 | `0x807962B0` | 1,428 | 357 | `807962b0_FUN_807962b0.c` | conditional (medium) |
| 4 | `0x80664610` | 1,292 | 323 | `80664610_FUN_80664610.c` | ready (medium) |
| 5 | `0x80792A20` | 1,212 | 303 | `80792a20_FUN_80792a20.c` | ready (medium) |
| 6 | `0x80605BA0` | 1,164 | 291 | `80605ba0_FUN_80605ba0.c` | conditional (medium) |
| 7 | `0x8067E640` | 1,160 | 290 | `8067e640_FUN_8067e640.c` | ready (medium) |
| 8 | `0x8062B5C0` | 1,136 | 284 | `8062b5c0_FUN_8062b5c0.c` | ready (medium) |

**Ready: 6** (`0x806D09B0`, `0x806B8380`, `0x80664610`, `0x80792A20`,
`0x8067E640`, `0x8062B5C0`).
**Conditional: 2** (`0x807962B0`, `0x80605BA0`).

Every `bytes` value is the `size` column of the TSV verbatim, and in all eight
cases the size is the true instruction span: the decoder found the final `blr`
at `address + size - 4` in each of the eight bodies. There is no padding tail
this wave (wave 141 had one).

### Two corrections to earlier lanes

Both of the two largest rows were previously rejected as "paired-single
decompiler failures", and both rejections are wrong on the shipped bytes:

* `0x806D09B0` was rejected by waves 27 and 28 ("`unaff_GQR0` paired-single
  body with unreachable blocks", `docs/COVERAGE_WAVE27_BASES.md:172`,
  `docs/COVERAGE_WAVE28_BASES.md`). The body contains exactly **four** paired
  single instructions: `psq_st f31/f30` in the prologue and `psq_l f31/f30` in
  the epilogue, all with `W=0, qr0` - the plain `f32` pair spill described in
  `docs/REL_ELF_MAPPING.md`. There is no quantisation and no GQR read anywhere
  in the body. The export's `/* WARNING: Removing unreachable block */` lines
  name `0x806D0F70`/`0x806D0F78`, which lie inside the `_restgpr` epilogue, not
  inside the logic.
* `0x806B8380` was rejected by wave 23 for the same reason
  (`docs/COVERAGE_WAVE23_BASES.md:81`). It contains **two** paired-single
  instructions, both the `f31` prologue/epilogue spill pair, and no GQR use.

So the "paired-single family" note in those two coverage documents should not
be used to skip this address range again.

### Selection rule

Candidate set: every `functions.tsv` row of `rel-d_basesNP` whose address is

1. absent from the `address` column of `ledger/functions.csv` (all modules), and
2. absent from every `#{1,6} ` heading of every `docs/*.md`, including the
   sections added by waves 140 and 141.

Ranked by `size` descending, the first eight are the table above. Applying the
filter to the current tree yields **2,006** candidate rows. (Wave 141 reported
2,011 rows *before* it claimed its eight; the small difference is rows whose
addresses now appear in a heading in `docs/SPEC_WAVE141_BASES.md`, so the two
measurements agree.)

The eight rows immediately below the cut, for a later lane:
`0x80772370` (1,108), `0x806639A0` (1,096), `0x807BEF60` (1,096),
`0x8074C540` (1,092), `0x8075D5C0` (1,080), `0x80763E50` (1,056),
`0x806765B0` (1,052), `0x80609440` (1,040).

## Method

**Address mapping.** Re-read from the program headers of
`work/rels/elf/d_basesNP.elf` (not copied from an older document):

```
PT_LOAD  p_offset  p_vaddr     filesz    memsz
  0      0x0010F0  0x806000F0  0x1C6004  0x1C6004
  1      0x1C70F4  0x807C60F4  0x0004FC  0x0004FC
  2      0x1C75F0  0x807C65F0  0x00000C  0x00000C
  3      0x1C7600  0x807C6600  0x00A5FC  0x00A5FC
  4      0x1D1C00  0x807D0C00  0x051CB8  0x051CB8
  5      0x000000  0x80822900  0x000000  0x012484 (zero fill)
```

so `file_offset = 0x10F0 + (address - 0x806000F0) = (address - 0x80600000) + 0x1000`
for every code address in this document, as `docs/REL_ELF_MAPPING.md`
requires. Data in `0x807Cxxxx`/`0x807D0xxx`/`0x8081xxxx` resolves through the
same rule; the one range that cannot be read statically is
`0x8037xxxx`/`0x8042xxxx` (main.dol BSS), which is called out where it matters.

**Decoder.** A throwaway PowerPC decoder was written under
`%TEMP%\opencode` (never in the repo) and validated against the facts
`docs/REL_ELF_MAPPING.md` pins, plus two more found this wave:

* `psq_l` = 56, `psq_st` = 60 (verified: the eight bodies contain 9 `psq_`
  words - two spill pairs in rows 1-3 and one `psq_l` in row 4 - all `W=0`).
* For opcode 4 the extended opcode is **five** bits (`(word >> 1) & 0x1F`), not
  ten. A decoder that masks `0x3FF` reports `ps_mul`/`ps_madd`/`ps_sum0` as
  `ps?89`/`ps?61`/`ps?74`, which is how the "paired-single" verdicts above were
  produced.

Register recovery: `0x802DCD40..0x802DCD74` / `0x802DCD8C..0x802DCDBC` are the
`_savegpr_N`/`_restgpr_N` helpers. The incoming `r3` survives them, and every
body below was re-read with `this` taken from the surviving register rather than
from Ghidra's `void`/packed signature.

---

## 1. `0x806D09B0` - `remo_lift_up` per-frame execute

**Evidence.** `work/ghidra/rel-d_basesNP/export/806d09b0_FUN_806d09b0.c`,
1,508 bytes, 377 words, synthetic `0x806D09B0`.

**What it is.** The per-frame body of the `remo_lift_up` actor family, whose
rider, tween, effect and retarget helpers `0x806CF6A0..0x806D0820` are already
translated in `src/game/bases.rs` (see `docs/COVERAGE_WAVE27_BASES.md`). The
field map matches those helpers exactly: `+0x430` rider slot, `+0x5EC` target
selector, `+0x608` ride gate, `+0x610` release counter, `+0x648` achievement
flag, `+0x650` timer, `+0x664` timer direction. The body adds the lift's own
state block at `+0x628..+0x665`.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0xAC..+0xB4` | `f32` x3 | world position (written by `fn_806d0510`'s caller chain) |
| `+0xE8/+0xEC/+0xF0` | `f32` | horizontal velocity, vertical velocity, forward velocity |
| `+0x104` | `u16` | lean angle, rewritten by `0x806CFF20` |
| `+0x1EC` | object | collision probe used by `0x8006EF40`/`0x8006DB30` |
| `+0x430` | `i8` | rider slot; `-1` = free, `0..7` = slot index (table below) |
| `+0x5D8`, `+0x5E6` | `s16` | rider pose pair republished on arrival |
| `+0x5EC` | `u32` | actor id of the ride target |
| `+0x608` | `u16` | ride gate: `0` = open, non-zero = blocked (also set by `fn_806d05c0`) |
| `+0x628` | `i32` | set to `10` while a rider is aboard |
| `+0x630` | `f32` | target height (set by `fn_806d0820`'s state objects) |
| `+0x638` | `f32` | previous frame's `+0xE8` |
| `+0x63C` | `i32` | variant selector (read by `fn_806d0820`, not by this body) |
| `+0x640` | `i32` | cleared on arrival in state 2 |
| `+0x644` | `u32` | result of the previous `0x8006FA40` probe |
| `+0x648` | `u32` | achievement flag (owned by `fn_806d06a0`/`fn_806d0720`) |
| `+0x64C` | `i32` | 60-frame cooldown |
| `+0x650` | `u16` | two-phase timer (owned by `fn_806d0510`) |
| `+0x656` | `s16` | accumulated step angle |
| `+0x658` | `s16` | accumulated ride angle |
| `+0x65A` | `u16` | arrival hold, seeded `0x1E` |
| `+0x65C` | `u8` | settle state: `0`, `1`, `2`, `4` |
| `+0x65D` | `u8` | settle enable |
| `+0x663` | `u8` | angle-timer guard read by `fn_806cff20` |
| `+0x664` | `u8` | timer direction (owned by `fn_806d0510`) |
| `+0x665` | `u8` | flags; bit 7 cleared on settle |

### Exact behaviour

1. Prologue: `stfd`/`psq_st` spill of `f30`/`f31` (qr0 pair spills), then
   `_savegpr` for the register bracket, `r29 = this`.
2. `mask = 0x800441E0(this, &rider_out)` - the four-bit rider presence mask
   of the `+0x3E0` chain, `rider_out` = the index chosen by
   `0x800440C0`.
3. `flags = 0x806CFB30(this, &flags_out)` - writes a three-bit field:
   bit 0 set when rider 0 is present **and** `int8(this+0x665) >= 0` **and**
   `(rider->+0x10D4 & 2) != 0`; bit 1 when `(rider->+0x10D4 & 0x20) != 0`;
   bit 2 when `(rider->+0x10D4 & 0x40) != 0`. The same loop also calls
   `0x800656C0(rider, this->+0x430, 1)` for actors whose `+0x38C` class byte is
   `3` and whose `+0x8` type is `0x193`/`0x191`/`0x3C`, and sets bit 4 of
   `rider->+0x34A` for type `0x85`/`0x19B`.
4. Rider bookkeeping, using the byte table at **`0x807CB4A8`**
   (`1,2,4,8,16,32,64,128`, then `63`). `+0x430` is a **slot index** `0..7`, so
   `table[slot]` is that slot's presence bit:
   * `mask == 0`: if `fn_806d05c0(this, this->+0x430) != 0` then
     `this->+0x430 = -1`.
   * `mask != 0`: if `(table[this->+0x430] & mask) == 0` and
     `this->+0x430 != -1` and `fn_806d05c0(this, slot) != 0` then
     `this->+0x430 = -1`; then if `slot < 0` and `rider_out >= 0`,
     `slot = rider_out` and `fn_806d06a0(this)` (achievement sound `0x2A6`);
     then if `slot == rider_out` set `this->+0x608 = 1`.
5. `if (int8(this->+0x430) < 0)` -> step 7 (settle path), else step 6.
6. **Ride path.**
   1. `this->+0x628 = 10; +0x65D = 1; +0x65C = 0; +0x65A = 0x1E`.
   2. `v = -(i16)0x806CFF20(this, -(i16)0x806CFE60(this))`; `this->+0x638 =
      this->+0xE8`; the *target* is `f30 = v / 1280.0` and the *step* is
      `f31 = v * 0.061 / 1280.0` (the export's `2^52` magic converts `v` as an
      **unsigned** 16-bit value, so `v` is used after `neg`/`extsh`).
   3. Clamp by `flags`:
      * `flags & 2 == 0` and `flags & 4 == 0`:
        `0x8015F470(target = f30, step = |f31|, &this->+0xE8)` - i.e.
        `step_float_clamped` pulls `+0xE8` toward `f30` in `|f31|` increments;
        if it reports no arrival and `f31 == 0.0` retry with `step = 0.1`;
        `this->+0xF4 = 2.0`; clamp `+0xE8` into `[-2.0, +2.0]`.
      * `flags & 2 == 0` and `flags & 4 != 0`: if `target > 0.0` then
        `this->+0xE8 = 0.0`.
      * `flags & 2 != 0`: if `target < 0.0` then `this->+0xE8 = 0.0`.
   4. `this->+0xAC += this->+0xE8`; if `+0xE8 != 0.0` and the new `+0xAC`
      differs from `+0xB8`, post sound **`0x2B1`**:
      `0x801995A0([0x80429D60], 0x2B1, this->+0x0, &this->+0xAC,
      (s16)(100.0f * +0xE8), 0)`.
   5. Direction reversal test: `(+0xE8 < 0 && +0x638 > 0) ||
      (+0xE8 > 0 && +0x638 < 0)` -> `fn_806d0720(this)` (2-tick pose release).
   6. Target sampling: `target = 0x80162D00(this->+0x5EC)`; save
      `+0xB0` in `f30`; write `this->+0xB0 = target->+0xB0`;
      `hit = 0x8006FA40(this + 0x1EC, 0)`; restore `this->+0xB0`; then
      `0x806CFFD0(this)` with `f1 = target->+0xB0` (the export shows this as
      `void`, but the two `lfs`/`stfs` pairs prove the float argument).
   7. If `hit != 0` and `this->+0x644 == 0` and `this->+0x64C == 0`:
      `+0x64C = 60`; `0x800439B0(this, mask)`; `+0x5D8 = +0x5E6`.
   8. `this->+0x644 = hit`;
      `this->+0x658 += (s16)(1536.0f * this->+0xE8)`.
7. **Settle path.**
   1. `0x806CFF20(this, 0); this->+0x663 = 0; this->+0x665 &= 0x7F;
      this->+0x644 = 0;` if `this->+0x65D == 0` -> step 8 (epilogue).
   2. Switch on `this->+0x65C`:
      * `0`: sample `0x80162D00(this->+0x5EC)`'s position into the frame
        scratch, call `0x806CFFD0(this)` with `f1 = target->+0xB0`; if
        `this->+0x65A == 0` then `+0x65C = 1`.
      * `1`: `0x806D0410(this, 0)`; sound `0x2B0`:
        `0x80199390([0x80429D60], 0x2B0, this->+0x0, &pos, 0)`;
        `0x806D0510(this, 0x40, 0x180)`.
      * `2`: `dir = (this->+0x630 < this->+0xB0)`;
        `step = {0.5, -0.5}[dir]` (`0x807CB4B0`);
        `ang = {-1536, +1536}[dir]` (`0x807CB4B8` as `s16`);
        `this->+0xEC = step; this->+0xB0 += step; this->+0x656 += ang`;
        `0x806D0510(this, 0x40, 0x180)`; sound `0x2B0` as above.
        Overshoot test (unordered-aware, `cror`):
        `dir == 0 ? (this->+0x630 < this->+0xB0) : (this->+0xB0 < this->+0x630)`
        -> snap `+0xB0 = +0x630`; `+0x65C = 4`; `+0x640 = 0`;
        `+0x5D8 = +0x5E6`.
      * `4`: `0x806D0410(this, 1)`; `0x806D0510(this, 4, 0xC0)`.
      * any other value: nothing.
8. `0x806D0780(this)` - the shared post-update. Its export is
   `halt_baddata`, so the call is claimed by address only.

**Signature.**

```rust
/// Per-frame `execute` of the `remo_lift_up` lift: tracks the rider mask,
/// drives the horizontal lean, republishes the pose and runs the four-state
/// settle machine while no rider is aboard.
/// # Original
/// `FUN_806d09b0`, `0x806D09B0`, 1,508 bytes,
/// `work/ghidra/rel-d_basesNP/export/806d09b0_FUN_806d09b0.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - lift actor; see the field map in `docs/SPEC_WAVE142_BASES.md`.
/// # Confidence
/// medium - every branch, store and constant is literal; `0x806D0780` is
/// unclaimed (`halt_baddata`) and the `0x806CFB30` flag bits are named by
/// their consumers.
pub unsafe fn fn_806d09b0(this: usize)
```

**Constants.** mask table `0x807CB4A8` (`1,2,4,8,16,32,64,128`); settle
velocities `0x807CB4B0` = `+0.5/-0.5`; settle angles `0x807CB4B8` =
`-1536/+1536`; `0x807CB418 = 0.061`; `0x807CB41C = 1.0`;
`0x807CB420 = 2.0`; `0x807CB450 = 0.0`; `0x807CB480 = 0x43300000`
(`2^52`); `0x807CB4BC = 0.1`; `0x807CB4C0 = 100.0`;
`0x807CB4C4 = 1536.0`; `0x807CB49C = 64.0` (inside `0x806CFFD0`);
sounds `0x2A6`, `0x2A7`, `0x2B0`, `0x2B1`; globals `0x80429D60`
(sound), `0x80429E00`, `0x8042A18C`.

**Verdict: ready (medium).** Everything that reaches memory is decoded; the
"paired-single" rejections from waves 27/28 are void (see above). The single
unclaimed callee is `0x806D0780`.

---

## 2. `0x806B8380` - `wendy` ring-trail tick

**Evidence.** `work/ghidra/rel-d_basesNP/export/806b8380_FUN_806b8380.c`,
1,500 bytes, 375 words, synthetic `0x806B8380`.

**What it is.** The per-frame trail sampler of the **`wendy` boss ring**. Three
independent proofs:

* the effect names it passes to `0x8016C890` are `Wm_we_ringhit`
  (`0x807F5E5C`) and `Wm_we_ringkira` (`0x807F5E6C`);
* it looks up the pane name `wendy_ring` (`0x807F5E00`) in the model object at
  `+0x3F0` via `0x800E78E0`, exactly as `fn_806b7170` binds `wendy` models in
  `docs/COVERAGE_WAVE23_BASES.md`;
* it fills the same `+0x53C`/`+0xCC0`/`+0x540` ring that the already-translated
  `fn_806b7690` reads (`docs/COVERAGE_WAVE23_BASES.md`).

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0xAC..+0xB4` | `f32` x3 | boss position |
| `+0xE8/+0xEC/+0xF0` | `f32` | velocity (scaled, integrated, then restored) |
| `+0x1EC` | object | collision probe |
| `+0x348` | `u8` | `^= 1` on every horizontal reversal |
| `+0x38F` | `u8` | player index forwarded to `0x80075270` |
| `+0x3F0` | object | model/scene object holding the `wendy_ring` pane |
| `+0x53C` | `u32` | number of live ring nodes, `<= 5` |
| `+0x540 + 12*i` | `f32` x3 | position history, 120 entries (ends `+0xAE0`) |
| `+0xAE0 + 4*i` | `f32` | per-sample scalar history, 120 entries (ends `+0xCC0`) |
| `+0xCC0..+0xCD0` | `u32` x5 | five ring cursors, each `(x + 1) % 120` |
| `+0xCF0 + 164*i` | struct | node `i`: `+0x00 dx`, `+0x04 dy`, `+0x08 scale`, `+0x0C scale` (rest unused) |
| `+0x1008` | `f32` | smoothed spacing |
| `+0x100C` | `f32` | normalised spacing (`1 - +0x100C` feeds the smoothing) |
| `+0x1040` | `u8` | "touching" byte used as the sound argument and the UI state |
| `+0x1044` | `i32` | caller counter, compared with `1000` |
| `+0x1604` | object | the effect-owner object whose `+0xA0` slot is invoked |

### Exact behaviour

1. Prologue keeps `f31` only (`stfd` + `psq_st qr0`).
2. `probe = 0x80075270(x = this->+0xAC, y = this->+0xB0,
   param_3 = this->+0x38F, out = &height)` - the stage surface probe of
   `src/game/stage.rs`; `height` is written.
3. `scale = (probe == 1) ? 0.5 : 1.0`; save `+0xE8..+0xF0`, multiply all
   three by `scale`, `0x8006CD40(this)` (integrate), restore the saved
   velocities.
4. `t = (1.0 - this->+0x100C) * 0.7`; if `probe == 1` then `t *= 0.5`;
   `this->+0x1008 += 0.2 * (t - this->+0x1008)` through `0x8065C210`
   (`0x8065C210(t, from, to)` returns `*from + t*(*to - *from)` with `from` in
   `r3` and `to` in `r4`, so `from = this+0x1008` and `to = &t`);
   then with `s = this->+0x100C + previous`,
   `this->+0x100C += 0.95 * (s - this->+0x100C)`.
5. Advance the five cursors: `this->+0xCC0+k = (this->+0xCC0+k + 1) % 120`
   for `k = 0,1,2,3,4` (the `%` is the `0x88888889 / srawi 6` magic for 120).
6. Store the current sample: `pos[+0xCC0] = this->+0xAC..+0xB4` and
   `scalar[+0xCC0] = this->+0x1008`.
7. For `i in 0..this->+0x53C`:
   `node[i].dx = pos[idx[i]].x - this->+0xAC`;
   `node[i].dy = pos[idx[i]].y - this->+0xB0`;
   `node[i].s = scalar[idx[i]] * 10.0 * SCALE[i]` written to **both** `+0x08`
   and `+0x0C`, with `SCALE = [0.75, 0.7, 0.42, 0.28, 0.2]` at
   `0x807CAB84`. (`idx[i]` is `this->+0xCC0 + 4*i`; the `z` difference is
   computed but only kept in a stack slot.)
8. Bounce tests, using `ceiling = [0x80429E00]->+0x3C`:
   * `this->+0xEC > 0 && 0x8006EF40(this + 0x1EC, 0) != 0` -> bounce;
   * `this->+0xEC > 0 && this->+0xB0 <= ceiling - 6.0` -> bounce;
   * `this->+0xEC < 0 && 0x8006DB30(this + 0x1EC) != 0` -> bounce.
9. **Bounce body:** `this->+0xEC = -this->+0xEC`;
   `this->+0x1008 += 0.0` (it adds the constant at `0x807CABB8`, which is
   `0.0`, i.e. a fidelity no-op); if the *negated* velocity is exactly `0.0` then
   `s = -node[0].s`; position `= (this->+0xAC, this->+0xB0 + s, 5500.0)`;
   `0x8016C890("Wm_we_ringhit", 0, &pos, 0, 0)`;
   `0x8006CED0(&p, this)` then `0x8006A4B0(&p2, &p)`;
   `0x80198FC0([0x80429D60], 1358, &p2, (s16)this->+0x1040, 0)`.
10. **Wall test:** `0x8006FA40(this + 0x1EC, 0) != 0` ->
    `this->+0xE8 = -this->+0xE8`; `this->+0x348 ^= 1`;
    `this->+0x1008 += 0.0` (same no-op); same `s` sign rule on the horizontal pair;
    position `= (this->+0xAC + s, this->+0xB0, 5500.0)`;
    `0x8016C890("Wm_we_ringkira", 0, &pos, 0, 0)`; the same two-call position
    copy and `0x80198FC0(..., 1358, ...)`.
11. Contact latch: if `this->+0x1040 == 0 && probe == 1` then
    `+0x1040 = 1; fn_806b7b70(this, height)`. If `this->+0x1040 != 0 &&
    probe == 0` then `+0x1040 = 0` and, with the ceiling re-read as
    `ceiling - [0x80429E00]->+0x44`, re-probe `0x80075270(...)`; if that
    returns `1`, `fn_806b7b70(this, height)`.
12. `0x800E78E0(&matrix, this + 0x3F0, "wendy_ring", 0x80377DC8)`; then the
    virtual call `(*[this->+0x1604])[0xA0](this->+0x1604, "Wm_we_ringkira",
    0, &matrix, 0, 0)`.
13. Flush gate: `if (this->+0xAC < 10.0 || (0x806B7860(this) == 0 &&
    this->+0x1044 < 1000)) fn_806b7690(this)`.

**Signature.**

```rust
/// Samples the `wendy` ring trail: one surface probe, velocity integration,
/// two smoothing passes, the 120-entry position/scalar history, five node
/// exports, the bounce/wall effects and the flush gate.
/// # Original
/// `FUN_806b8380`, `0x806B8380`, 1,500 bytes,
/// `work/ghidra/rel-d_basesNP/export/806b8380_FUN_806b8380.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - `wendy` boss actor; see the field map above.
/// # Confidence
/// medium - all arithmetic, the table values and the two action names are
/// literal; `0x8006CD40`, `0x8006CED0` and `0x800E78E0` are unclaimed and are
/// called with the shapes shown.
pub unsafe fn fn_806b8380(this: usize)
```

**Constants.** `0x807CAB48 = 0.0`; `0x807CAB4C = 6.0`; `0x807CAB5C = 5500.0`;
`0x807CAB84..94 = 0.75/0.7/0.42/0.28/0.2`; `0x807CABBC = 0.2`;
`0x807CABC0 = 0.95`; `0x807CABC4 = 0.7`; `0x807CABC8 = 1000`;
`0x807CABCC = 10.0`; `0x807CABD0 = 0.5`; ring length `120`;
stride `164`; sounds `1358` (`0x54E`); effect names `Wm_we_ringhit`,
`Wm_we_ringkira`, pane `wendy_ring`; node-name vector `0x80377DC8`.

**Verdict: ready (medium).** The wave-23 rejection does not hold; the body uses
one `psq_st`/`psq_l` pair for `f31` and nothing else paired-single.

---

## 3. `0x807962B0` - map-record start-up with the conditional state ladder

**Evidence.** `work/ghidra/rel-d_basesNP/export/807962b0_FUN_807962b0.c`,
1,428 bytes, 357 words, synthetic `0x807962B0`.

**What it is.** A one-shot start-up body for a **world/course record actor**;
it sits directly after the four-slot `character_SV` panel
(`0x80795140..0x80796180`, `docs/COVERAGE_WAVE91_BASES.md`) and shares that
panel's manager singleton `0x80429F7C` (state word `+0x370`) and the scene
object `0x80834470`. Unlike the panel, its state word is `+0x27C` and its
record id is `+0x22C`, i.e. the `0x8079` course-tree record layout already used
by `fn_807995a0` (`+0x22C` id) and `fn_807995e0` (`0x800F3330` relation,
`0x80775100`/`0x80775150` node probes).

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0xAC..+0xB4` | `f32` x3 | position, copied from `0x80100200(scene, +0x22C)` |
| `+0xB8..+0xC0` | `f32` x3 | previous position (tail copy) |
| `+0xDC/+0xE0/+0xE4` | `f32` | scale triple = `f31 * 1.9` |
| `+0x114`, `+0x118` | `f32` | `-7.0`, `-20.0` (from `0x8042D0B4`/`0xB8`) |
| `+0x1AC`, `+0x1B0` | `i32` | handles, reset to `-1` |
| `+0x22C` | `u32` | course-record id |
| `+0x23C` | `u32` | id returned by `0x800F3330` |
| `+0x240` | `u32` | `3` when `0x8042A24D == 7` and the ids match |
| `+0x27C` | `u32` | state, one of `1,2,3,4,5,6,7,8,9,10,11,12` |
| `+0x2E0` | `u32` | `0x80775080(scene, +0x22C)` |
| `+0x2F8` | `i32` | set to `-1` |

### Exact behaviour

1. `this->+0x2F8 = -1`; global `0x8042A19E = 0`.
2. `id` source: if `*(u8*)0x8042A248 == 0` then `this->+0x22C =
   [0x80429F7C]->+0x44`; else `this->+0x22C = 0x801002C0(scene)`.
   *`0x801002C0`'s export is `void`* - the caller keeps `r3`, which is the
   return of its tail call `0x800F53A0`. This is the unresolved value in the
   body (see the verdict).
3. In the same `else` path: if
   `0x800F57A0(scene + [scene+0x338C]*0x16518 + 0x3394, id, 0x400)` returns
   non-zero then `this->+0x27C = 2`; else if `0x80775150(scene, id)` returns
   non-zero then `this->+0x27C = 1`.
4. If `*(u8*)0x8042A24D == 7` and `0x801002C0(scene) == this->+0x22C` then
   `this->+0x240 = 3`. This second call passes a stray `r4 = 0x8081BBCC`
   (not a string - the bytes there are not text), which the callee ignores;
   under the one-argument reading the two calls are therefore identical and the
   comparison is trivially true. That is the second symptom behind the
   conditional verdict.
5. `0x80100200(&pos, scene, id)` -> `this->+0xAC..+0xB4`.
6. `0x800F57A0(..., 0x800)` again; on success `this->+0xB0 += 15.0`
   (`0x807D0164`).
7. `character = 0x800F9840(0x80775100(scene, id))`; if `character != 11`:
   `area = *(u8*)0x8042A24D`; `kind = 0x800FA9C0(area, 0x80775100(scene, id),
   id)`; then
   * `kind == 8` or `kind == 0` -> skip;
   * `character in 1..3` -> `this->+0xB4 -= 150.0`;
   * `character == 8` -> the long branch (step 8);
   * `character in {4, 9, 6}` -> nothing;
   * otherwise `f31 = 0.01` (`0x807D016C`).
8. `character == 8` branch:
   * `0x800FBFB0()` returns non-zero ->
     `0x800FB320(0x807714F0())` (`fn_807714f0` = the `0x8042A24D` world byte):
     if it returns `0` and `kind != 0` -> `state = 3`, `f31 = 0.01`;
     else `kind in {2,3}` -> `state = 4`, `f31 = 0.01`;
     else `kind == 5` -> `state = 4`, `f31 = 0.01`;
     else if `kind != 0 && kind != 8` -> `state = 3`, `f31 = 0.01`.
   * `0x800FBFB0() == 0` -> `f31 = 0.01`.
9. Manager gates, in order: `0x8079B1D0(this)` -> `state = 5`;
   `0x8079B700()` -> `state = 6`; `0x8079B240()` -> `state = 7`.
10. If `0x800F9840([0x80429F7C]->+0x54) == 5`:
    `kind = 0x800FA9C0([0x80429F7C]->+0x50, [0x80429F7C]->+0x54, id)`;
    `kind in {1,6}` -> `state = 9`;
    if `0x800FC810()` or `0x800FC830()` and `kind not in {1,6,8,0}` ->
    `this->+0x23C = 0x800F3330(scene + [+0x338C]*0x3F08 + 0x5C7F4,
    0x80832EB0, id)`, an effect is positioned at `this->+0xAC` minus the
    `0x80100200` position, `0x800F6410(this, &delta)`, `state = 8`;
    if `0x800FC7F0()` -> `state = 11`, and `kind in {1,6}` -> `state = 12`,
    `f31 = 0.01`.
11. If `[0x80429F7C]->+0x50 == 7 && +0x54 == 37 && 0x800FB040()` ->
    `state = 10`, `f31 = 0.01`.
12. `this->+0x114 = -7.0`; `this->+0x118 = -20.0`; `+0x1AC = +0x1B0 = -1`;
    if `0x800FB040()` then `0x800F2A00(682, this, 0, 0, 0)`.
13. `this->+0xDC..+0xE4 = f31 * 1.9` (`0x8031D3B4`); then
    `0x807C3B00(0x80834470)`; if it returns `0`, `0x807C3AC0(0x80834470)`.
14. If `character != 11` and `[0x80429F7C]->+0x50 == 0x8042A24D`:
    `kind = 0x800FA9C0(...)`; if `kind in {0,1,6,8}` then
    `0x807C3A40(0x80834470)`.
15. Tail: `this->+0xB8..+0xC0 = this->+0xAC..+0xB4`; `0x8079BD50(this)`;
    `this->+0x2E0 = 0x80775080(scene, this->+0x22C)`.

**Signature.**

```rust
/// Start-up for a course-tree record actor: resolves the record id, places the
/// actor, picks the handler state `+0x27C` from the character/area ladder and
/// publishes the previous position and the `0x80775080` record.
/// # Original
/// `FUN_807962b0`, `0x807962B0`, 1,428 bytes,
/// `work/ghidra/rel-d_basesNP/export/807962b0_FUN_807962b0.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - record actor; `+0x22C` id, `+0x27C` state, `+0xAC..+0xB4` position.
/// # Confidence
/// medium - every comparison, store and table id is literal; the id source in
/// step 2 is a `void` callee whose `r3` the caller keeps.
pub unsafe fn fn_807962b0(this: usize)
```

**Constants.** floats `0x807D0164 = 15.0`, `0x807D016C = 0.01`; scene stride
`0x16518` (`0x3394`) and `0x3F08` (`0x5C7F4`); globals `0x8042A18C`,
`0x8042A248`, `0x8042A24D`, `0x8042A19E`, `0x80429F7C` (`+0x44`, `+0x48`,
`+0x50`, `+0x54`), `0x80834470`, `0x80832EB0`, `0x8081BBCC`; event `682`.
Tail scale base `0x8031D3B4 = 1.9`.

**Verdict: conditional (medium).** The branch ladder is fully literal and can
be written today **except** for `this->+0x22C` in the "loading" path: the id is
the `r3` left behind by `0x801002C0`, whose export is `void` (its own tail call
`0x800F53A0` is also `void` in `work/ghidra/main-gb/export/801002c0_*.c`).
Before translating, one of the two must be re-read from
`work/ghidra/main-gb/elf` to confirm the return register; until then the
`+0x22C` value would be invented.

---

## 4. `0x80664610` - thirteen-state transition controller

**Evidence.** `work/ghidra/rel-d_basesNP/export/80664610_FUN_80664610.c`,
1,292 bytes, 323 words, synthetic `0x80664610`.

**What it is.** A scripted transition that links two actors (`+0x424` and
`+0x420`), plays four player emotes (`0x80052170` ids 165..168), waits on a
camera/actor distance and finally destroys itself. It is driven by a 13-entry
jump table at **`0x807E7970`** indexed by `this->+0x360` (states `0..12`; state
`> 12` returns immediately).

Table contents (verified word by word):

```
0  0x80664688   1  0x806646A4   2  0x806646E4   3  0x80664714
4  0x80664770   5  0x80664804   6  0x80664844   7  0x806648F0
8  0x806649B8   9  0x806649DC  10  0x80664A54  11  0x80664AD0
12 0x80664AE8
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x360` | `u32` | state `0..12` |
| `+0x3D0` | `i32` | countdown; decremented at the top of every tick |
| `+0x408` | `u32` | link slot: `[this + 0x408 + 4*([0x80429CA0]-1)]` |
| `+0x418` | `f32` | smoothed camera/zoom value (target `0x41C`) |
| `+0x41C` | `f32` | target value, set to `1.0` in state 2 |
| `+0x420` | actor | second actor (link half B) |
| `+0x424` | actor | first actor (link half A) |
| `+0x470` | actor | the player actor that owns the emote (state 10/11) |
| `+0xDE8` | object | animation object reached through `+0x470` in state 11 |

### Exact behaviour

1. If `this->+0x3D0 > 0` decrement it.
2. Virtual call `(*[0x80429D64])[0x28]([0x80429D64], 787, 0)` on every tick.
3. `switch (this->+0x360)`:
   * `0`: `0x8005C930([0x80429C94])`; `state += 1`.
   * `1`: `0x807211D0(this->+0x424)`;
     `0x80721290(this->+0x424, [this + 0x408 + 4*([0x80429CA0]-1)])`;
     `+0x3D0 = 150`; `state += 1`.
   * `2`: if `+0x3D0 == 0`: `+0x41C = 1.0`; `0x80662E10(this)`; `state += 1`.
   * `3`: `0x8015F470(step = 0.011, target = +0x41C, &this->+0x418)`;
     `[0x80429DD0] + 0x90110 = +0x418` (the camera zoom);
     if `0x80663000(this) != 0 && 0x80663050(this) != 0` then `state += 1`.
   * `4`: the same lerp and the same camera store; then for `slot in 0..4`:
     if `0x80663180(this, slot) != 0` and `0x80052030(0x8005F900(slot)) == 0`
     then `all = false`; if `all` then `+0x3D0 = 30`, `state += 1`.
   * `5`: if `+0x3D0 == 0`: virtual call
     `(*[this->+0x420 + 0x60])[0xD4](this->+0x420, 0x8082BCE4)`; `+0x3D0 = 30`;
     `state += 1`.
   * `6`: if `+0x3D0 == 0`: for `slot in 0..4`:
     if `0x80663180(this, slot) != 0` then `0x80052170(player, 165)` and, when
     `player == this->+0x470`, `(*[player + 0x60])[0x434](player, 67, 0)`;
     then `0x80721290(this->+0x424, this->+0x420)`;
     clear bit 3 of `this->+0x424->+0x120` and set it on `this->+0x420->+0x120`;
     `state += 1`.
   * `7`: for `slot in 0..4`: if `0x80663180(this, slot) != 0` and
     `0x80052030(player) != 0` then `0x80052170(player, 166)`. Then, **in the
     same block**, if `+0x3D0 == 0`: `dx/dy/dz = (+0x420) - (+0x424)`;
     `psq_l f2,(r1+8)` loads `(dx, dy)`; `ps_mul f2,f2,f2` -> `(dx², dy²)`;
     `ps_madd f1,f1,f2,f1` -> `f1.low = dz² + dx²`;
     `ps_sum0 f1,f1,f2,f2` -> `f1.low = dz² + dx² + dy²`. If that value is
     smaller than `500.0` (`0x807C8BCC`), `0x80721270(...)`, `+0x3D0 = 20`,
     `state += 1`.
   * `8`: if `+0x3D0 == 0`: `0x80662F40(this)`; `state += 1`.
   * `9`: if `0x80052030(this->+0x470) != 0`: the 165-style loop with **167**,
     then `(*[this->+0x470 + 0x60])[0x434](this->+0x470, 68, 0)`;
     `state += 1`.
   * `10`: `0x801690E0(*(*(this->+0x470 + 0xDE8) + 4) + 40, 60.0)`;
     when it returns non-zero the 168 loop runs, `+0x3D0 = 90`, `state += 1`.
   * `11`: if `+0x3D0 == 0` then `state += 1`.
   * `12`: `0x80102260(3, 0, 0, 5)` - the terminal fade; no state change.

**Signature.**

```rust
/// Thirteen-state transition: links `+0x424` to `+0x420`, tween the camera zoom
/// at `[0x80429DD0]+0x90110`, plays player emotes 165..168 and destroys the
/// actor once the distance test and the animation have finished.
/// # Original
/// `FUN_80664610`, `0x80664610`, 1,292 bytes,
/// `work/ghidra/rel-d_basesNP/export/80664610_FUN_80664610.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - controller; `+0x360` state, `+0x3D0` timer, `+0x420`/`+0x424`
///   actors, `+0x470` player.
/// # Confidence
/// medium - every state, timer, sound and emote id is literal.
pub unsafe fn fn_80664610(this: usize)
```

**Constants.** table `0x807E7970`; timers `150`, `30`, `20`, `90`;
`0x807C8BF0 = 1.0`; `0x807C8BD0 = 0.011`; `0x807C8BCC = 500.0`;
`0x807C8BDC = 60.0`; emote ids `165..168`; virtual ids `67`/`68`;
sound `787`; link count `0x80429CA0`; globals `0x80429D64`, `0x80429C94`,
`0x80429CA0`, `0x80429DD0`.

**Verdict: ready (medium).** Every state, timer, id and the state-7 distance
reduction are decoded: the three pair words are `0x104200B2`
(`ps_mul f2,f2,f2`), `0x1021107A` (`ps_madd f1,f1,f2,f1`, low lane
`dz² + dx²`) and `0x10211094` (`ps_sum0 f1,f1,f2,f2`, low lane
`dz² + dx² + dy²`), so the compare is `dx² + dy² + dz² < 500.0`. The only
open call contracts are the two-actor link API
(`0x807211D0`/`0x80721290`/`0x80721270`), which is invoked by address, and the
`[0x80429CA0]` slot rule is taken from the call shape.

---

## 5. `0x80792A20` - world-map node start-up / finish sequence

**Evidence.** `work/ghidra/rel-d_basesNP/export/80792a20_FUN_80792a20.c`,
1,212 bytes, 303 words, synthetic `0x80792A20`.

**What it is.** A world-map actor start-up script. Proof: every effect/sound
helper it uses is `game::worldmap` code (`0x80105240`, `0x80103390`,
`0x80103410`, `0x80105440`) and it names the map objects **`W103`**
(`0x8081B11C`) and **`W101`** (`0x8081B124`) resolved through `0x80100270`.
It is driven by a 16-entry table at **`0x8081B12C`** indexed by
`this->+0x7AC`; entries `7..13` all point at the epilogue (no-op states), so
only `0..6`, `14` and `15` do work.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x0` | `u32` | actor id passed to the sound helpers |
| `+0xAC..+0xB4` | `f32` x3 | world position |
| `+0x102` | `s16` | yaw/roll angle driven in states 2-5 |
| `+0x10C` | `f32` | animation rate (`9.0` in state 6) |
| `+0x114`, `+0x118` | `f32` | `0.0`, `0.0` in state 6 |
| `+0x124` | `u8` | finished flag cleared in state 15 |
| `+0x6DC`, `+0x6E0` | `u32` | spawned handles (`-1` = none) |
| `+0x770` | object | animation object |
| `+0x7AC` | `u32` | state `0..15` |
| `+0x7B0` | `i32` | countdown |
| `+0x7B4` | `i32` | per-tick angle step |

### Exact behaviour

1. Prologue keeps `r25..r31`. If `+0x7AC > 15` return `0`.
2. `switch (this->+0x7AC)`:
   * `0`: `0x80100270(&pos, scene, "W103")` (look up the `W103` map object's
     position); `pos3 = (this->+0xAC, this->+0xB0, pos.z)`;
     `0x800F69F0(this, &pos3, 13.0, 0.7, 3.5, 0, 20, 0x80377DE0)` - the float
     arguments are literally `0.2*3.5` and `1.0*3.5` and the last word is a
     constant offset vector; then
     `0x80105240([0x8042A27C], 101, &this->+0xAC, 1)`; `+0x7AC = 1`.
   * `1`: if `0x800F6AB0() == 0` stop; `+0x7B0 = 90`;
     `0x80103390([0x8042A26C], 12, &this->+0xAC, 0, 0)`; `+0x7AC = 2`.
   * `2`: if `+0x7B0 > 0` decrement and stop; else
     `0x80105240([0x8042A27C], 102, &this->+0xAC, 1)`;
     `+0x7B0 = 5`; `+0x7B4 = (16383 - this->+0x102) / 5`; `+0x7AC = 3`.
   * `3`: while `+0x7B0 > 0`: `--+0x7B0; this->+0x102 += (s16)+0x7B4`; stop.
     When it reaches 0: `+0x7B0 = 45`, `+0x7AC = 4`.
   * `4`: same countdown; at 0: `0x80105240(..., 102, ...)`;
     `+0x7B0 = 8`; `+0x7B4 = (16383 - this->+0x102) / 8`; `+0x7AC = 5`.
   * `5`: same countdown/add; at 0: `+0x7B0 = 60`, `+0x7AC = 6`.
   * `6`: if `+0x7B0 > 0` decrement and stop. Else:
     if `this->+0x6DC < 0` -> `this->+0x6DC = 0x80105440([0x8042A27C],
     (*[this+0x60])[0xD0](this), 0, &this->+0xAC, 1, 1)`.
     if `this->+0x6E0 < 0` -> `this->+0x6E0 = 0x80103410([0x8042A26C], 2,
     &this->+0x770, 0x8081B110, 0, 0)`.
     `(*[this+0x60])[0xB4](this)`; `0x801680E0(this + 0x770)`;
     `+0x10C = 9.0`; `+0x114 = +0x118 = 0.0`; `0x8006CC00(this)`;
     `0x8006CD40(this)`;
     `0x800F6410(this, -(0x80377DD4..DC))` (the runtime vector at
     `0x80377DD4`, negated);
     `q = 0x801006E0(scene, "W101", &pos)`; if `this->+0xAC < 100.0 + pos.x`
     then `0x800F8A80(this)`, `0x800F8AD0(this)`, `0x800F6CB0(this)`,
     `0x801680E0(this + 0x770, 1.0)`; `+0x7B0 = 10`, `+0x7AC = 14`.
   * `14`: if `+0x7B0 > 0` decrement and stop; else
     `0x801006E0(scene, "W101", &pos)`; `offset = -(0x80377DD4..DC)`;
     `0x800F69F0(this, &pos, 13.0, 3.5, 0.35, 0, 20, &offset)` (the floats are
     `1.0*3.5` and `0.1*3.5`);
     `0x80105240([0x8042A27C], 104, &this->+0xAC, 1)`;
     if `0x80792EE0(this) != 0` then
     `0x8076D1F0(0x80792EE0(this), 0x8081B124, 0x807CFF20)`;
     `+0x7AC = 15`.
   * `15`: if `0x800F6AB0() != 0`: `this->+0x124 = 0`; `0x80162510(this)`;
     `return 1`.
3. Return `r29` - `1` only when state 15 ran to completion, `0` otherwise.

**Signature.**

```rust
/// World-map node start-up: places the node from the `W103` map object, spawns
/// its effect, spins it through 90 degrees in two staged ramps, then plays the
/// `W101` arrival effect and retires the actor.
/// # Original
/// `FUN_80792a20`, `0x80792A20`, 1,212 bytes,
/// `work/ghidra/rel-d_basesNP/export/80792a20_FUN_80792a20.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - map node actor; `+0x7AC` state, `+0x7B0` timer, `+0x7B4` step.
/// # Returns
/// `1` once the actor has been retired, else `0`.
/// # Confidence
/// medium - every state, id and float is literal; the `0x80377DD4` vector is
/// runtime BSS.
pub unsafe fn fn_80792a20(this: usize) -> u32
```

**Constants.** state table `0x8081B12C`; names `W101` (`0x8081B124`),
`W103` (`0x8081B11C`), `F7C0` (`0x8081B078`), `0x8081B110`;
`0x807CFE98 = 9.0`; `0x807CFEAC = 3.5`; `0x807CFF10 = 13.0`;
`0x807CFF18 = 0.2`; `0x807CFF1C = 1.0`; `0x807CFF24 = 13.0`;
`0x807CFF2C = 1.0`; `0x807CFF30 = 0.1`; `0x807CFF58 = 1.0`;
`0x807CFF5C = 0.0`; `0x807CFF6C = 100.0`; sounds `101`, `102`, `104`;
angle constant `16383`; spawn ids `12` and `2`; globals `0x8042A18C`
(scene), `0x8042A27C` (sound), `0x8042A26C` (effect).

**Verdict: ready (medium).** All sixteen table entries, all eight live states
and every id are decoded. `0x80792EE0` is called twice on purpose (Ghidra does
not fold it; the first call is the guard, the second is the argument), and the
`0x80377DD4` vector is BSS, so it is documented as "read, negated, not
reproducible statically" rather than guessed.

---

## 6. `0x80605BA0` - contents-screen pane rebuild

**Evidence.** `work/ghidra/rel-d_basesNP/export/80605ba0_FUN_80605ba0.c`,
1,164 bytes, 291 words, synthetic `0x80605BA0`.

**What it is.** The shared row builder of the "contents" screen
(`src/game/bases.rs`, `fn_80606030` calls it as
`0x80605BA0(screen, live_value, row)` with row `0` or `1`). It clears the
visibility bit of a block of panes, rebuilds one of three text rows, and then
re-shows two panes. The three row writers are the already-translated
`fn_80605a20` (row `+4`), `fn_80605aa0` (row `+2`) and `fn_80605b20`
(row `+6`).

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x254 + 8*k` | pointer | pane record `k`; `pane->+0xBB` bit 0 = visible |
| `+0x274 + 4*k` | pointer/`u32` | row source slot `k` (see verdict) |
| `+0x2AC` | object | text object used by the row writers |

### Exact behaviour

1. Save `r30`/`r31`; `screen = r3`, `kind = r4`, `slot = r5`, `i = r5`.
2. **Clear phase A** (`slot <= 7`): for `block in 0..((7-slot+2)/2)` and the
   eight pointers `[screen + 4*slot + 0x254 + 8*j]`, `j = 0..7`, clear bit 0 of
   `pane->+0xBB`; advance the base by `64`.
   The loop is peeled: it runs only when `((7-slot+2)/2) > 8`, otherwise the
   same eight-pointer block is skipped and the tail loop covers it.
3. **Tail phase A**: while `i <= 7`, clear bit 0 of
   `[screen + 4*i + 0x254]->+0xBB`, `i += 2`.
4. **Clear phase B** (`slot < 4`): the same two loops with `(4-slot+1)/2` and
   base `+0x254 + 4*slot`, covering `i <= 4`.
5. **Dispatch on `kind`**:
   * `2` -> `fn_80605a20(screen, slot)`
   * `0` -> `fn_80605aa0(screen, slot)`
   * `1` -> `fn_80605b20(screen, slot)`
   * `5` -> set bit 0 of `[screen + 4*slot + 0x254]->+0xBB`, then
     `fn_80605a20(screen, slot)`
   * `3` -> set bit 0 of the same pane, then `fn_80605aa0(screen, slot)`
   * `4` -> set bit 0 of the same pane, then `fn_80605b20(screen, slot)`
   * anything else -> nothing
6. Unconditionally set bit 0 of `[screen + 4*(slot+2) + 0x274]->+0xBB` and of
   `[screen + 4*slot + 0x274]->+0xBB`.

**Signature.**

```rust
/// Rebuilds one contents-screen row: hides the panes in the slot range, calls
/// the row writer selected by `kind`, then re-shows the two panes that follow
/// the slot.
/// # Original
/// `FUN_80605ba0`, `0x80605BA0`, 1,164 bytes,
/// `work/ghidra/rel-d_basesNP/export/80605ba0_FUN_80605ba0.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `screen` - contents screen; panes at `+0x254`, values at `+0x274`.
/// * `kind` - row kind `0..5` (see the dispatch table).
/// * `slot` - first pane index of the range.
/// # Confidence
/// medium - every branch, index and mask is literal; the pane stride is the
/// open question recorded in the verdict.
pub unsafe fn fn_80605ba0(screen: usize, kind: i32, slot: i32)
```

**Constants.** pane array `+0x254`; value array `+0x274`; visibility bit
`pane->+0xBB` bit 0; row writers `0x80605A20`, `0x80605AA0`, `0x80605B20`;
row text ids `0x807D1EA0`/`0x807D1EA4`/`0x807D1EA8`; index bases `+4`/`+2`/`+6`.

**Verdict: conditional (medium).** The control flow is complete, but the pane
addressing does not agree with the already-translated row writers: the clear
loops here walk `+0x254` with a **stride of 8 bytes** (`0x80605C98..0x80605D20`:
`+596, +604, +612, +620, +628, +636, +644, +652`, then `+64`), while
`fn_80605a20`/`aa0`/`b20` read panes at `+0x254 + 4*(index + k)` and values at
`+0x274 + 4*index`. One of the two readings is wrong, and the fix must come
from `0x800B3B60`'s argument contract (text object vs. pane pointer), not from
this export. Writing the loops from this document alone would embed the
stride-8 assumption.

---

## 7. `0x8067E640` - seven-bit layer gate that retires the actor

**Evidence.** `work/ghidra/rel-d_basesNP/export/8067e640_FUN_8067e640.c`,
1,160 bytes, 290 words, synthetic `0x8067E640`. The export is
`void FUN_8067e640(void)` with the owner recovered as `func_0x802dcd6c()`; the
body uses `r25..r31`, so `this` is the incoming `r3`.

**What it is.** A per-tick gate that builds a presence mask over four player
slots and the actor's own layer/id fields, changes one animation/link state
through `0x800E41C0`, and retires the actor through `0x80162510`. It writes
**nothing** to `this`, so the whole effect is the two calls.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x3E4` | `u32` | packed flags: bit 0 = "active", bits 4..7 = a 4-bit layer id |
| `+0x3EC` | `u8` | id passed to `0x800E41C0` |
| `+0x3ED` | `i8` | "unused" marker: `-1` sets bit 0 and skips everything |
| `+0x3EE` | `u8` | `0xFF` sets bit 1 |
| `+0x3EF` | `u8` | `0` sets bit 2; otherwise the 3-way compare below |
| `+0x3F0` | `u8` | `0` sets bit 3 |
| `+0x3F1` | `u8` | height/anim byte used in bit 2 |

### Exact behaviour

1. `mask = 0`.
2. If `int8(this->+0x3ED) == -1` then `mask = 1` and jump to step 8.
3. `entry = [0x80429E98] + (u8)[0x8042A1C8 + 0x120E] * 0x3B0`;
   if `entry->+0x2C == 0` use `0` instead; `0x8008E890(entry,
   (i8)this->+0x3ED, &out)` produces the four heights.
4. For `slot in 0..4`: `p = 0x8005F900(slot)`; if `p != 0` and
   `bit = (*[p+0x60])[0x6C](p)->+0x0` and `([0x80429CA8] & (1 << bit)) != 0`
   and `0x8067DCF0(this, p, &out) != 0` then `mask |= result` and
   `rider_bits |= 1 << slot`.
5. If `int8(this->+0x3EE) == 0xFF` then `mask |= 2`; else if
   `0x8067E500(this) != 0` then `mask |= 2`.
6. If `this->+0x3EF == 0` then `mask |= 4`; else if
   `this->+0x3F1 != 0`:
   * `n = this->+0x3F1 - 1`;
   * if `[0x80429CA0] >= 2`: for `slot in 0..4`: `p = 0x8005F900(slot)`; if
     `(*[p+0x60])[0x6C](p)->+0x0 == n` and
     `((u16*)[0x807CA6A0])[this->+0x3EF - 1] == *(u32*)(p + 0x1090)` then
     `mask |= 4`;
   * else `p = 0x8067DC50(this)`; if `this->+0x3EF >= 7` then use
     `(*[p+0x60])[0x3D4](p)`; else `((u16*)[0x807CA6A0])[this->+0x3EF-1]` is
     compared with `*(u32*)(p + 0x1090)`; equal -> `mask |= 4`.
7. If `this->+0x3F0 != 0 && (mask & 1) != 0`: for `slot in 0..4`: if bit
   `slot` of `rider_bits` is set and `0x8005F900(slot) != 0` then
   `mask |= 0x8067DD60(this, p, this->+0x3F0)`.
8. `mask &= 0xFF`. If `mask == 0x0F` take **block A**, otherwise **block B**.
   In both, `layer = (this->+0x3E4 >> 4) & 0xF`, and
   `a = this->+0x3E0 & [0x8042A078]->0x0`,
   `b = this->+0x3E4 & [0x8042A078]->0x4`.
   * **Block A** (`mask == 0x0F`): if `(this->+0x3E4 & 1) != 0` then, when
     `(a|b) != 0`, call `0x800E41C0([0x8042A078], this->+0x3EC, 0, 0, 0, 0, 0)`
     and set `done = 1`; if `(this->+0x3E4 & 1) == 0` then, when `(a|b) == 0`,
     call the same function with `r6 = 1` and set `done = 1`. Finally, if
     `layer == 0 && done != 0` -> `0x80162510(this)`.
   * **Block B** (`mask != 0x0F`): if `layer != 2` -> return without doing
     anything. If `(this->+0x3E4 & 1) != 0` then, when `(a|b) == 0`, call with
     `r6 = 1`; if `(this->+0x3E4 & 1) == 0` then, when `(a|b) != 0`, call with
     `r6 = 0`. No retire in this block.
9. Nothing else is written.
10. Epilogue (`_restgpr`).

**Signature.**

```rust
/// Builds the seven-bit presence/layer mask, refreshes the `0x800E41C0`
/// animation slot when the layer word allows it, and retires the actor once the
/// "active" bit and the layer branch agree.
/// # Original
/// `FUN_8067e640`, `0x8067E640`, 1,160 bytes,
/// `work/ghidra/rel-d_basesNP/export/8067e640_FUN_8067e640.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - actor whose `+0x3E4` flags, `+0x3EC` id and `+0x3ED..+0x3F1`
///   bytes drive the mask.
/// # Confidence
/// medium - every mask bit, comparison and call argument is literal; the
/// meaning of the seven bits is inferred from their branches.
pub unsafe fn fn_8067e640(this: usize)
```

**Constants.** `0x80429E98` (entry table), `0x8042A1C8 + 0x120E`
(course/area index), stride `0x3B0`; `0x80429CA8` (slot bit table);
`0x80429CA0` (player count); `0x807CA6A0` (height table); `0x8042A078`
(two-word layer mask); `0x80162510` (retire); `0x800E41C0` (animation slot);
`0x8008E890` (height probe).

**Verdict: ready (medium).** Every constant and branch is decoded; the mask
never escapes the function, so the seven bits only need to reproduce the two
`0x800E41C0` argument patterns and the retire condition. `0x80162510` and
`0x800E41C0` are called by address exactly as in `fn_80792a20`.

---

## 8. `0x8062B5C0` - layout bind and first-show for the 20-pane screen

**Evidence.** `work/ghidra/rel-d_basesNP/export/8062b5c0_FUN_8062b5c0.c`,
1,136 bytes, 284 words, synthetic `0x8062B5C0`.

**What it is.** The "bind once, then show" body of a 20-pane layout screen. It
validates twenty pane pointers, resolves the pane named **`_ef_attack`**
(`0x807EBD98`), publishes `screen + 0x208` as the parent of all twenty panes,
then applies one of two visibility patterns selected by bit 5 of
`[0x80429F80]`, copies four positions and calls `0x8062C4B0(screen)`.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x208` | object | layout/pane parent object |
| `+0x2DC + 4*k` | pointer | twenty panes, `k = 0..19` |
| `+0x4E0 + 4*k` | `f32` | four published heights (`+0x4E0..+0x4EC`) |
| `+0x50C` | `u8` | "shown" latch; returns `1` immediately when set |
| `+0x50D` | `u8` | cleared |
| `+0x510` | `u8` | cleared |
| `+0x512` | `u8` | "bound" latch |
| `+0x7C` | `u8` | set to `11` |

### Exact behaviour

1. Validate: if any of the twenty pointers `+0x2DC + 4*k`, `k = 0..19`, is
   `0` return `0` immediately (peeled as two 10-pointer blocks with
   `addi r4,r4,40`).
2. If `this->+0x512 == 0`:
   `0x80006BE0(this + 0x208, "_ef_attack")`; if it returns `0` return `0`;
   else for all twenty panes `pane->+0xF4 = this + 0x208`;
   `this->+0x512 = 1` (`r0 = 1`).
3. If `this->+0x50C != 0` return `1`.
4. If `0x8062BA50(this) == 0` return `0`.
5. Visibility patterns, selected by `([0x80429F80] >> 5) & 1` (the `r0 = 1`/`0`
   that step 6 reuses is `0` in the first branch and `1` in the second):
   * bit set: `+0xBB |= 1` on `+0x3A8` (936), `+0x378` (888), `+0x42C` (1068),
     `+0x434` (1076); `+0xBB &= ~1` on `+0x3AC` (940), `+0x37C` (892),
     `+0x430` (1072), `+0x438` (1080).
   * bit clear: the exact inverse of the eight stores above.
6. `+0xBB |= r0` on `+0x380` (896), `+0x384` (900), `+0x388` (904),
   `+0x38C` (908) and `+0xBB |= 1` on `+0x368` (872), where `r0` is `0` in the
   first branch and `1` in the second.
7. `this->+0x7C = 11`; `+0x50C = 1`; `+0x50D = 0`; `+0x510 = 0`.
8. `+0x4E0 = ((f32*)pane0)[0x2C/4]` i.e. copy `pane->+0x2C` from each of the
   four panes `+0x380/+0x384/+0x388/+0x38C` into `+0x4E0/+0x4E4/+0x4E8/+0x4EC`
   (only the first float of each `+0x2C` vector is stored).
9. `0x8062C4B0(this)`; return `1`.

**Signature.**

```rust
/// Binds the twenty-pane layout once (`_ef_attack` must resolve), applies the
/// `0x80429F80` bit-5 visibility pattern, publishes the four pane heights and
/// reports whether the screen is ready.
/// # Original
/// `FUN_8062b5c0`, `0x8062B5C0`, 1,136 bytes,
/// `work/ghidra/rel-d_basesNP/export/8062b5c0_FUN_8062b5c0.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `screen` - layout screen; panes `+0x2DC`, parent `+0x208`, latches
///   `+0x50C`/`+0x512`.
/// # Returns
/// `1` when bound and shown, `0` while a pane or the `_ef_attack` name is
/// still missing.
/// # Confidence
/// medium - the twenty pointer slots, both visibility patterns, the four
/// copies and all four latches are literal.
pub unsafe fn fn_8062b5c0(screen: usize) -> u32
```

**Constants.** pane array `+0x2DC`; parent `+0x208`; pane name `_ef_attack`
(`0x807EBD98`); `+0xBB` bit 0 = visible; `+0xF4` = parent link; visibility
selector `0x80429F80` bit 5; heights `+0x4E0..+0x4EC`; tail `0x8062C4B0`.

**Verdict: ready (medium).** All twenty slots, both patterns and the four
copies are literal; the only unclaimed callee is `0x8062BA50` (a "children
ready" gate) and it is used only as a `0`/non-`0` test.

---

## Deliberate skips

Read and rejected, or deferred with the reason:

* **`0x80772370` (1,108)**, **`0x806639A0` (1,096)**, **`0x807BEF60` (1,096)**
  and the rest of the eight-row handover list above were not reached; they are
  the next wave's population and their sizes are unchanged.
* **`0x8077B450` (4,076) and `0x8065CB20` (3,788)** are the two remaining
  "ready" rows of `docs/SPEC_WAVE140_BASES.md`; **`0x8079C330` (4,232),
  `0x80661070` (3,432), `0x8079D3C0` (3,176), `0x8078DC70` (2,576),
  `0x8076EF80` (2,544) and `0x80773210` (2,096)** are that wave's six
  "conditional" rows. All eight remain open.
* **`0x8067E160` (412)** still reads its only non-zero argument from the
  incoming `r7` while the export prints a `void` signature, so it stays
  unclaimed.
* **`0x80605A20`/`0x80605AA0`/`0x80605B20`** were *not* re-claimed: they are
  already translated and are cited here only as the row writers that make
  `0x80605BA0` conditional.

## Verification

* Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (8 rows, 10,400 bytes),
  re-read from the TSV for this pass rather than from prose.
* Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
* Every one of the eight addresses is absent from `ledger/functions.csv` and
  from every `#{1,6} ` heading in `docs/*.md` as the tree stood before this
  file existed (a re-run of the selector must exclude this file's own
  headings).
* The decoder was validated against the `psq_l`/`psq_st` and GQR facts in
  `docs/REL_ELF_MAPPING.md`, and the two constant pools it reads
  (`0x807CB3D0` for `0x806D09B0`, `0x807CAB48` for `0x806B8380`) were decoded
  word by word rather than taken from an export comment.
* The TSV sizes were checked to be instruction spans: `blr` at
  `address + size - 4` in all eight.
