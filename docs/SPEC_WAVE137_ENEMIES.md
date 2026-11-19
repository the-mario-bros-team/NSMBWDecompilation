# Wave 137 - implementation specification for 12 class-B `d_enemiesNP` bodies

## What this page is

Wave 127 (`docs/ANALYSIS_WAVE127_ENEMIES.md`) reclassified 384 formerly-skipped
`d_enemiesNP` addresses into 173 class **B** (reconstructible now), 160 class C
and 51 class D. Waves 128 - 136 (`docs/SPEC_WAVE128_ENEMIES.md` ..
`docs/SPEC_WAVE136_ENEMIES.md`) specified **108** of those B rows, twelve per
wave.

Wave 136's window opened at 28 reachable instructions and closed at 22, ending
at `0x80A2F670`. Its "next band" note listed ten 22-instruction/88-byte rows:
three were already ledger rows, one was taken, and seven were left for this
lane. Two of those seven - `0x80AFEC20` and `0x80AFF4F0` - have since been
claimed by implementation lanes and now carry `ledger/functions.csv` rows, so
five of the seven are still open.

This lane is the continuation of the same ranking: it starts at the second
22-instruction row (ascending address after wave 136's last pick) and runs down
to 13 reachable instructions. It specifies the twelve highest-ranked class-B
rows that are still absent from `ledger/functions.csv`, from every heading of
every `docs/*.md`, and from the Rust translation (no `fn_80...` body for them
exists in `src/game/enemies.rs`).

It is a **read-only** pass: it writes no Rust, no ledger row and no other doc.

Each body was decoded from `work/rels/elf/d_enemiesNP.elf` with the `+0x1000`
segment shift of `docs/REL_ELF_MAPPING.md` (first `PT_LOAD` is
`p_offset = 0x10F0`, `p_vaddr = 0x80A000F0`), cross-checked against the Ghidra
export at the cited path and against the `size` column of
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`. Every constant that lives
inside the REL image was read out of the ELF itself, so the float and double
values below are the shipped values. Addresses outside the file-backed image
are marked **(ext)**.

## Selection

Ordering is the wave-127 "reachable instruction count" - the only size metric
that is not inflated by the TSV extent quirk - with ties broken by TSV size and
then by ascending address.

The metric was re-validated before this page was written: walking the twelve
wave-136 addresses reproduces `28, 27, 26, 25, 25, 25, 25, 25, 24, 23, 23, 22`
exactly - the same calibration wave 136 used to re-walk its 96-address chain
(287 down to 28) before publishing its own twelve rows (28 down to 22). This
page picks up at the second 22-instruction row and runs down to 13.

| # | address | TSV size | reachable insns | owner / table row | verdict |
|---:|---|---:|---:|---|---|
| 1 | `0x80A43DC0` | 88 | 22 | deleting dtor, CoinFloor vtable `0x80B406C4` | **ready** |
| 2 | `0x80A446B0` | 88 | 22 | deleting dtor, CoinJugem vtable `0x80B40D90` | **ready** |
| 3 | `0x80AECF00` | 88 | 22 | deleting dtor, RailPolyChild vtable `0x80B61720` | **ready** |
| 4 | `0x80AEE6D0` | 88 | 22 | deleting dtor, RailPolyParent vtable `0x80B61DC4` | **ready** |
| 5 | `0x80AEF910` | 88 | 22 | deleting dtor, RailPoly vtable `0x80B62314` | **ready** |
| 6 | `0x80A7ECA0` | 84 | 21 | Icicle state row `0x80B4D588`; `+0x61C` guard, `Vibration` dispatch | **ready** |
| 7 | `0x80B0D150` | 84 | 21 | Teresa `+0x640`/`+0x678` registry predicate | **ready** |
| 8 | `0x80A2C2A0` | 76 | 19 | bounce-ball pair-contact raw-yaw helper | **ready** |
| 9 | `0x80B0AF60` | 76 | 19 | SyncroBarnar row `0x80B67B70`, kind-`0x99` lookup | **ready** |
| 10 | `0x80B0DAF0` | 76 | 19 | Teresa row `0x80B68318`, `+0x9E7` latch reset | **ready** |
| 11 | `0x80A78700` | 64 | 16 | Hnswich row `0x80B4C4D8`, release helper | **ready** |
| 12 | `0x80AA0300` | 52 | 13 | `0x80064680(0xA5, ...)` spawn thunk | **ready** |

**Split: 12 ready, 0 conditional.** Total 952 bytes, 238 reachable
instructions.

No row in this tranche needs a datum established before a byte-exact port, and
each of the four risk classes that made waves 110-126 skip a body is absent or
bounded:

* **no paired-single traffic.** Not one `psq_l`/`psq_st`, no `psq_*`, no
  `bctr` jump table, no `ldexpf` pseudo-register in any of the twelve bodies.
  Rank 8's export *looks* GQR-damaged (wave 92 and wave 123 skipped it for
  that reason) but the stream is plain `lfs`/`fmuls`/`fctiwz`; there is no
  quantiser anywhere in the body.
* **no incoming float argument.** No body reads `f1`/`f2`/`f3`/`f4` before
  writing it. Rank 8 reads an `f32` from the actor, never from the ABI.
* **no unresolved callee landmine.** Every in-REL `bl` target is a
  `functions.tsv` entry (`0x80B0AC50`, `0x80B0ABF0`, `0x80A7E630`); the two
  non-ledger targets (`0x80B0CD80`, `0x80A7E630`) have their call-site
  signatures fixed by this page, and the one `main.dol` target in rank 12
  (`0x80064680`) has a documented six-argument shape (see that section).
* **no `bctrl` whose receiver is unknown.** Rank 6 has exactly one `bctrl`;
  the receiver chain is `*(this + 0x60) + 0xD4`, which the module already
  models with the `w134_class_slot2` helper used by `fn_80a3f8f0` and
  `fn_80a7ed20`.

The next band - 12 instructions, 48 bytes - is `0x80A2B490`, `0x80A6F8C0`
and `0x80AA39D0`; `0x80A22B20` (44 bytes, 10 instructions) follows. Those are
wave 138's candidates. (Wave 136's own rank 12, `0x80A2F670`, is still neither
ledgered nor defined in Rust at this pass; it was specified there and does not
re-enter the pool.)

## Reading conventions

* Synthetic base is `0x80A00000`. ELF file offset is
  `(address - 0x80A00000) + 0x1000` for the first `PT_LOAD`
  (`p_offset = 0x10F0`, `p_vaddr = 0x80A000F0`). The file-backed image ends at
  `0x80B6E438`; the REL's zero-fill segment starts at `0x80B6E500` and is
  `0xB4E0` bytes long (`0x80B6E500..0x80B799E0`).
* `this` is the incoming `r3` unless stated otherwise. All `+0xNNN` are byte
  offsets from `this`.
* Branch mnemonics are the C form after the exact `bc` decode:
  `bc 12,2` = `beq`, `bc 4,2` = `bne`, `bc 12,1` = `bgt`, `bc 4,1` = `ble`,
  `bc 12,0` = `blt`, `bc 4,0` = `bge`, `bc 20,x` = unconditional `b`.
  `bclr 20,0` / `bclrl 20,0` are printed `blr` / `bclrl`, and a `bcctr 20,0`
  is printed `bctrl`.
* `addi rX,r0,imm` is `li rX,imm` (`r0` is not read when `RA == 0`).
  `xoris rD,rS,UI` XORs the **upper half** of `rS`, i.e. with `UI << 16`.
* **Rank 8's export is a decompiler artefact, not a plan.** The `2^52`
  integer-to-double idiom
  (`lis r0,0x4330` + `xoris` + `stw` pair + `lfd` + `fsub`) is an exact
  conversion. Ghidra folds the stack double to `2^52` and drops the embedded
  integer, producing
  `4.50360177485824e+15 - DAT_80b2bbb8`; the wrongness is documented in that
  section. Wave 92 and wave 123 copied Ghidra's reading; wave 127 corrected
  it. This page follows the bytes.
* **`_savegpr_27` is not argument-destroying.** `0x802DCD70`/`0x802DCD74` and
  `0x802DCDBC` (called by the `0x80B0CD80` callee and by `0x80064680`) are
  CodeWarrior save/restore helpers. The incoming `r3..r10` survive; a 64-bit
  "return pair" from them is the saved register pair, not a value.
* All twelve bodies are exactly `TSV size / 4` instructions with no trailing
  unreachable words, and all twelve end in a `blr` except rank 12, which ends
  in a tail `b` (`0x80AA0330`).

## Symbolic helpers used below

Roles are the ledger's (`ledger/functions.csv` evidence column) or the helper
tables of waves 128-136.

| address | role |
|---|---|
| `0x8015F800` | `state_descriptor_release(this, 0)` - returns `this`, frees only when `free > 0` (`game::task`, ledgered) |
| `0x802B9280` | `arc_heap_free_object(this)` - tail call `0x802B8F70(block, 0)` (ledgered) |
| `0x8008C380` | `collision_area_unregister(area)` - unlink from the stage collision lists (`game::stage`, ledgered) |
| `0x8007F950` | `fn_8007f950(link)` - two-sided splice on the `r13-0x58A0` list (`game::stage`, ledgered) |
| `0x80169020` | `fn_80169020(element)` - resource-registry query; returns 0 for an unbound element (`resource::registry`, ledgered) |
| `0x80B0AC50` | `fn_80b0ac50(this) -> *match` - recursive class-`0x99` search, matches `+0x998` against `+0x398` (ledgered, Rust `fn_80b0ac50(this: usize) -> usize`) |
| `0x80B0ABF0` | `fn_80b0abf0(this, source)` - `0x4330` word-to-double conversion of `+0x39C`, seeds the `+0x99C`/`+0x9A0`/`+0x9A4` timers (ledgered) |
| `0x80B0CD80` | `fn_80b0cd80(this, mode)` - **not ledgered, not in Rust**; 292-byte `_savegpr`-wrapped reset body (`work/ghidra/rel-d_enemiesNP/export/80b0cd80_FUN_80b0cd80.c`). Its export prints no arguments because of the save helper; this page fixes the two-argument call site. Call it by address (`call2(0x80B0_CD80, this, 0)`). |
| `0x80A7E630` | 8-byte thunk: `addi r3,r3,0x5FC; b 0x800944C0` (unledgered; already declared in Rust as `W135_FN_80A7E630` and called by address) |
| `0x800944C0` | **(ext)** final target of the `0x80A7E630` thunk; takes `r3 = this + 0x5FC` |
| `0x80064680` | **(ext)** six-argument mutex-wrapped spawn/registration: `_savegpr_27`, `0x80064600(r8)`, `0x8006CBA0(r3,r4,r5,r6,r7)` (`work/ghidra/main/export/80064680_FUN_80064680.c`). `r3..r8` are the live arguments; `r9`/`r10` are not read. |
| `0x80B73338` | state descriptor in this REL's zero-fill segment, identified as `Vibration` by `docs/COVERAGE_WAVE19_ENEMIES.md`; passed as `r4` of vtable slot `+0xD4` |
| `w134_class_slot2(this, slot, arg)` | module helper: `call2(*(u32*)(*(u32*)(this+0x60) + slot), this, arg)` |

---

## 1. `0x80A43DC0` - deleting destructor of the `Wm_ob_coin` CoinFloor vtable

* **Address:** `0x80A43DC0`
* **TSV size:** 88 bytes
* **Reachable body:** 22 instructions; 2 `bc`; 2 `bl`; ends at `0x80A43E14 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a43dc0_FUN_80a43dc0.c`
* **Proposed signature:** `pub unsafe fn fn_80a43dc0(this: usize, free_count: i32) -> usize`
* **Callers in this REL:** none by direct `bl`/`b`; the only reference is the
  32-bit word at `0x80B406C4`, which is slot 0 of the class vtable that follows
  the `daEnCoinFloor_c::StateID_AngleMove` name block (`0x80B40698`). The row
  runs `0x80A43DC0, 0x8015F840, 0x8015F850, 0x8015F8C0, 0x8015F8D0,
  0x80A43E20, ...`, then the id pair `0x0192_019C` / `6` and the model name
  `"Wm_ob_coin"` at `0x80B406FC`.
* **Constants:** none (no literal, no in-image word).
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-0x10(r1)`; LR at `+0x14`; `r31` at `+0x0C` = `free_count`; `r30` at
`+0x08` = `this`. No float temporaries; the frame exists only for the two saved
words and the LR.

### Instruction map

Every one of the five deleting destructors in this tranche (ranks 1-5) is the
same 22-word body; only the relative displacements differ. The offsets below
are from the function base.

| off | instruction |
|---:|---|
| `+0x00` | `stwu r1,-0x10(r1)` |
| `+0x04` | `mflr r0` |
| `+0x08` | `cmpwi cr0,r3,0` |
| `+0x0C` | `stw r0,0x14(r1)` |
| `+0x10` | `stw r31,0x0C(r1)` |
| `+0x14` | `mr r31,r4` |
| `+0x18` | `stw r30,0x08(r1)` |
| `+0x1C` | `mr r30,r3` |
| `+0x20` | `beq +0x3C` (`0x80A43DFC`) |
| `+0x24` | `li r4,0` |
| `+0x28` | `bl 0x8015F800` |
| `+0x2C` | `cmpwi cr0,r31,0` |
| `+0x30` | `bgt +0x3C` (`0x80A43DFC`) |
| `+0x34` | `mr r3,r30` |
| `+0x38` | `bl 0x802B9280` |
| `+0x3C` | `mr r3,r30` |
| `+0x40` | `lwz r31,0x0C(r1)` |
| `+0x44` | `lwz r30,0x08(r1)` |
| `+0x48` | `lwz r0,0x14(r1)` |
| `+0x4C` | `mtlr r0` |
| `+0x50` | `addi r1,r1,0x10` |
| `+0x54` | `blr` |

### Behaviour

```c
usize fn_80a43dc0(usize this, i32 free_count) {
    if (this != 0) {                       // 0x80A43DC8 cmpwi r3,0; 0x80A43DE0 beq -> return
        fn_8015f800(this, 0);              // 0x80A43DE4 li r4,0; 0x80A43DE8 bl
        if (free_count > 0)                // 0x80A43DEC cmpwi r31,0; 0x80A43DF0 bgt -> return
            fn_802b9280(this);             // 0x80A43DF4 mr r3,r30; 0x80A43DF8 bl
    }
    return this;                           // 0x80A43DFC mr r3,r30, epilogue
}
```

The return value is the incoming `this`, not the callee's result: `r30` is
restored into `r3` on both paths. The guard uses the *incoming* `r3`, before
`r30` is written, so the compiler kept the original argument register.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A43DE8` | `0x8015F800` | `r3 = this`, `r4 = 0` |
| `0x80A43DF8` | `0x802B9280` | `r3 = this` |

### Ambiguity

None. Verified mechanically: the 22 words are word-for-word identical to the
ledgered `fn_80a1f390` / `fn_80a1ffb0` bodies and to wave 136's rank 12
(`0x80A2F670`) once `bl`/`bc` displacements are normalised. Wave 127 already
recorded it as "byte-identical to `0x80A446B0`".

---

## 2. `0x80A446B0` - deleting destructor of the CoinJugem vtable

* **Address:** `0x80A446B0`
* **TSV size:** 88 bytes
* **Reachable body:** 22 instructions; 2 `bc`; 2 `bl`; ends at `0x80A44704 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a446b0_FUN_80a446b0.c`
* **Proposed signature:** `pub unsafe fn fn_80a446b0(this: usize, free_count: i32) -> usize`
* **Callers in this REL:** none by direct `bl`/`b`; referenced at `0x80B40D90`,
  slot 0 of the vtable that follows the
  `daEnCoinJugem_c::StateID_Jugemthrow` name block (`0x80B40D64`). The next
  words are `0x8015F840, 0x8015F850, 0x8015F8C0, 0x8015F8D0, 0x80A44710, ...`;
  the id pair at `0x80B40DC4` is `0x0193_019D` / `0x16`.
* **Constants:** none.
* **Verdict:** **ready** (confidence high)

### Frame

Identical to rank 1: `stwu r1,-0x10(r1)`; LR `+0x14`; `r31@+0x0C` =
`free_count`; `r30@+0x08` = `this`.

### Behaviour

The body is rank 1's instruction map at base `0x80A446B0`: the two branch
targets are `0x80A446EC` (`beq` at `0x80A446D0`, `bgt` at `0x80A446E0`) and the
two calls are `0x80A446D8 -> 0x8015F800` and `0x80A446E8 -> 0x802B9280`.

```c
usize fn_80a446b0(usize this, i32 free_count) {
    if (this != 0) {
        fn_8015f800(this, 0);
        if (free_count > 0) fn_802b9280(this);
    }
    return this;
}
```

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A446D8` | `0x8015F800` | `r3 = this`, `r4 = 0` |
| `0x80A446E8` | `0x802B9280` | `r3 = this` |

### Ambiguity

None. Byte-identical to rank 1 and to `fn_80a1f390`; only the encoded
displacements differ. A port may implement it as a call to `fn_80a1f390`, but
the original has an independent entry and the repo's convention
(`COVERAGE_WAVE19_ENEMIES.md`, duplicated callbacks) is to keep one Rust body
per entry.

---

## 3. `0x80AECF00` - deleting destructor of the RailPolyChild `ice` vtable

* **Address:** `0x80AECF00`
* **TSV size:** 88 bytes
* **Reachable body:** 22 instructions; 2 `bc`; 2 `bl`; ends at `0x80AECF54 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aecf00_FUN_80aecf00.c`
* **Proposed signature:** `pub unsafe fn fn_80aecf00(this: usize, free_count: i32) -> usize`
* **Callers in this REL:** none by direct `bl`/`b`; referenced at `0x80B61720`,
  slot 0 of the vtable that follows the
  `daEnRailPolyChild_c::StateID_Move` name block (`0x80B616F4`). Row:
  `0x80AECF00, 0x8015F840, 0x8015F850, 0x8015F8C0, 0x8015F8D0, 0x80AECF60, ...`;
  id pair `0x01E8_01EE` / `2`; model `g3d/ice.brres` at `0x80B6175C`.
* **Constants:** none.
* **Verdict:** **ready** (confidence high)

### Frame

Identical to rank 1: `stwu r1,-0x10(r1)`; LR `+0x14`; `r31@+0x0C` =
`free_count`; `r30@+0x08` = `this`.

### Behaviour

Rank 1's instruction map at base `0x80AECF00`: both branches land on
`0x80AECF3C` (`beq` at `0x80AECF20`, `bgt` at `0x80AECF30`), and the calls are
`0x80AECF28 -> 0x8015F800` and `0x80AECF38 -> 0x802B9280`.

```c
usize fn_80aecf00(usize this, i32 free_count) {
    if (this != 0) {
        fn_8015f800(this, 0);
        if (free_count > 0) fn_802b9280(this);
    }
    return this;
}
```

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80AECF28` | `0x8015F800` | `r3 = this`, `r4 = 0` |
| `0x80AECF38` | `0x802B9280` | `r3 = this` |

### Ambiguity

None. Byte-identical to ranks 1/2. `COVERAGE_WAVE122_ENEMIES.md` skipped the
whole `0x80AECF00..0x80AEF910` teardown group; there is nothing in the stream
to block it.

---

## 4. `0x80AEE6D0` - deleting destructor of the RailPolyParent `test_lift` vtable

* **Address:** `0x80AEE6D0`
* **TSV size:** 88 bytes
* **Reachable body:** 22 instructions; 2 `bc`; 2 `bl`; ends at `0x80AEE724 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aee6d0_FUN_80aee6d0.c`
* **Proposed signature:** `pub unsafe fn fn_80aee6d0(this: usize, free_count: i32) -> usize`
* **Callers in this REL:** none by direct `bl`/`b`; referenced at `0x80B61DC4`,
  slot 0 of the vtable after the
  `daEnRailPolyParent_c::StateID_Drop` name block (`0x80B61D98`). Row:
  `0x80AEE6D0, 0x8015F840, 0x8015F850, 0x8015F8C0, 0x8015F8D0, 0x80AEE730, ...`;
  id pair `0x01E7_01ED` / `2`; model `g3d/test_lift.brres` at `0x80B61DFC`.
* **Constants:** none.
* **Verdict:** **ready** (confidence high)

### Frame

Identical to rank 1: `stwu r1,-0x10(r1)`; LR `+0x14`; `r31@+0x0C` =
`free_count`; `r30@+0x08` = `this`.

### Behaviour

Rank 1's instruction map at base `0x80AEE6D0`: both branches land on
`0x80AEE70C` (`beq` at `0x80AEE6F0`, `bgt` at `0x80AEE700`), and the calls are
`0x80AEE6F8 -> 0x8015F800` and `0x80AEE708 -> 0x802B9280`.

```c
usize fn_80aee6d0(usize this, i32 free_count) {
    if (this != 0) {
        fn_8015f800(this, 0);
        if (free_count > 0) fn_802b9280(this);
    }
    return this;
}
```

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80AEE6F8` | `0x8015F800` | `r3 = this`, `r4 = 0` |
| `0x80AEE708` | `0x802B9280` | `r3 = this` |

### Ambiguity

None. Byte-identical to ranks 1/2/3.

---

## 5. `0x80AEF910` - deleting destructor of the RailPoly `red_ring` vtable

* **Address:** `0x80AEF910`
* **TSV size:** 88 bytes
* **Reachable body:** 22 instructions; 2 `bc`; 2 `bl`; ends at `0x80AEF964 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aef910_FUN_80aef910.c`
* **Proposed signature:** `pub unsafe fn fn_80aef910(this: usize, free_count: i32) -> usize`
* **Callers in this REL:** none by direct `bl`/`b`; referenced at `0x80B62314`,
  slot 0 of the vtable after the `daEnRailPoly_c::StateID_Drop` name block
  (`0x80B622EC`). Row: `0x80AEF910, 0x8015F840, 0x8015F850, 0x8015F8C0,
  0x8015F8D0, 0x80AEF970, ...`; id pair `0x01EF_01F5` / `2`; model
  `g3d/red_ring.brres` at `0x80B6234C`.
* **Constants:** none.
* **Verdict:** **ready** (confidence high)

### Frame

Identical to rank 1: `stwu r1,-0x10(r1)`; LR `+0x14`; `r31@+0x0C` =
`free_count`; `r30@+0x08` = `this`.

### Behaviour

Rank 1's instruction map at base `0x80AEF910`: both branches land on
`0x80AEF94C` (`beq` at `0x80AEF930`, `bgt` at `0x80AEF940`), and the calls are
`0x80AEF938 -> 0x8015F800` and `0x80AEF948 -> 0x802B9280`.

```c
usize fn_80aef910(usize this, i32 free_count) {
    if (this != 0) {
        fn_8015f800(this, 0);
        if (free_count > 0) fn_802b9280(this);
    }
    return this;
}
```

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80AEF938` | `0x8015F800` | `r3 = this`, `r4 = 0` |
| `0x80AEF948` | `0x802B9280` | `r3 = this` |

### Ambiguity

None. Byte-identical to ranks 1-4.

---

## 6. `0x80A7ECA0` - Icicle state row: `+0x61C` guard then the `Vibration` dispatch

* **Address:** `0x80A7ECA0`
* **TSV size:** 84 bytes
* **Reachable body:** 21 instructions; 1 `bc`; 1 `bctrl`; 1 `bl`; ends at
  `0x80A7ECF0 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a7eca0_FUN_80a7eca0.c`
* **Proposed signature:** `pub unsafe fn fn_80a7eca0(this: usize)`
* **Callers in this REL:** none by direct `bl`/`b`; referenced as the word at
  `0x80B4D588` in a 12-byte descriptor table (`{handler, 0, 0xFFFFFFFF}`) whose
  rows are the `daEnIcicle_c` state callbacks. Neighbours:
  `... 0x80A7EC90, 0, -1, 0x80A7ECA0, 0, -1, 0x80A7EC80, 0, -1 ...`.
* **Constants:** none that is read as data; the only literal is the token
  address below.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-0x10(r1)`; LR `+0x14`; `r31` at `+0x0C` = `this`.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B73338` | `u32` (address constant) | state descriptor in the REL's zero-fill segment (`0x80B6E500..0x80B799E0`); `docs/COVERAGE_WAVE19_ENEMIES.md` identifies the eight descriptors at `0x80B731B8..0x80B73378` as `ParentWait`, `ParentChild`, `ParentCrea`, `ParentDel`, `AnmWait`, `DownWait`, `Vibration`, `DownMove`, so this one is `Vibration`. Passed as `r4` of vtable slot `+0xD4`. |

### Behaviour

```c
void fn_80a7eca0(usize this) {
    if (*(u32 *)(this + 0x61C) != 0) {            // 0x80A7ECB4 lwz r0,0x61C(r3); 0x80A7ECB8 cmpwi; 0x80A7ECBC beq skip
        w134_class_slot2(this, 0xD4, 0x80B7_3338); // 0x80A7ECC0 lwz r12,0x60(r3); 0x80A7ECC4/C8 r4=&desc; 0x80A7ECCC lwz r12,0xD4(r12); 0x80A7ECD0 mtctr; 0x80A7ECD4 bctrl
    }
    call1(0x80A7_E630, this as u32);               // 0x80A7ECD8 mr r3,r31; 0x80A7ECDC bl
}
```

* The dispatch target is `*(u32 *)(*(u32 *)(this + 0x60) + 0xD4)`, i.e. the
  class vtable slot `+0xD4`, called with `r3 = this` and `r4 = 0x80B73338`.
  The helper `w134_class_slot2(this, 0xD4, token)` already implements exactly
  that shape in `src/game/enemies.rs` and is used by `fn_80a3f8f0` and
  `fn_80a7ed20`.
* The second call is to the 8-byte thunk `0x80A7E630`
  (`addi r3,r3,0x5FC; b 0x800944C0`). Keep the call at the thunk address, as
  wave 135 did: the repo already declares `W135_FN_80A7E630` and calls it with
  `call1`.
* `r3` at the thunk call is `r31` (the incoming `this`), not the result of the
  virtual call.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A7ECD4` | `*(u32*)(this+0x60) + 0xD4` (`bctrl`) | `r3 = this`, `r4 = 0x80B73338` |
| `0x80A7ECDC` | `0x80A7E630` | `r3 = this` |

### Ambiguity

None for byte-exactness. `COVERAGE_WAVE19_ENEMIES.md` recorded this row as
skipped because "`0x80A7E630`'s export is `halt_baddata()`"; the thunk is two
literal words and wave 135 already documented it. The 84 bytes are 21
instructions, not an extent artefact.

---

## 7. `0x80B0D150` - Teresa `+0x640`/`+0x678` registry predicate

* **Address:** `0x80B0D150`
* **TSV size:** 84 bytes
* **Reachable body:** 21 instructions; 2 `bc`; 2 `bl`; 1 intra-body `b`; ends
  at `0x80B0D1A0 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b0d150_FUN_80b0d150.c`
* **Proposed signature:** `pub unsafe fn fn_80b0d150(this: usize) -> u32`
* **Callers in this REL:** three direct calls, all in the `daEnTeresa_c`
  state bodies: `0x80B0DCC4` and `0x80B0DD18` (inside `0x80B0DC40`, 320 bytes)
  and `0x80B0DE70` (inside `0x80B0DE10`, 180 bytes). No table word references
  it, so `0x80B0D150` is a plain helper, not a virtual method.
* **Constants:** none. The two offsets are `+0x640` and `+0x678`; the results
  are the two registry handles the Teresa state machine checks before a state
  change.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-0x10(r1)`; LR `+0x14`; `r31@+0x0C` = `this`.

### Behaviour

```c
u32 fn_80b0d150(usize this) {
    if (call1(0x8016_9020, this + 0x640) == 0)   // 0x80B0D164 addi; 0x80B0D168 bl; 0x80B0D16C cmpwi; 0x80B0D170 beq .zero
        return 0;
    if (call1(0x8016_9020, this + 0x678) == 0)   // 0x80B0D174 addi r3,r31,0x678; 0x80B0D178 bl; 0x80B0D17C cmpwi; 0x80B0D180 beq .zero
        return 0;
    return 1;                                    // 0x80B0D184 li r3,1; 0x80B0D188 b .epilogue
  .zero:
    return 0;                                    // 0x80B0D18C li r3,0
}
```

Exact listing:

```
0x80B0D150  stwu r1,-0x10(r1)
0x80B0D154  mflr r0
0x80B0D158  stw r0,0x14(r1)
0x80B0D15C  stw r31,0x0C(r1)
0x80B0D160  mr r31,r3
0x80B0D164  addi r3,r3,0x640
0x80B0D168  bl 0x80169020
0x80B0D16C  cmpwi cr0,r3,0
0x80B0D170  beq 0x80B0D18C
0x80B0D174  addi r3,r31,0x678
0x80B0D178  bl 0x80169020
0x80B0D17C  cmpwi cr0,r3,0
0x80B0D180  beq 0x80B0D18C
0x80B0D184  li r3,1
0x80B0D188  b 0x80B0D190
0x80B0D18C  li r3,0
0x80B0D190  lwz r0,0x14(r1)
0x80B0D194  lwz r31,0x0C(r1)
0x80B0D198  mtlr r0
0x80B0D19C  addi r1,r1,0x10
0x80B0D1A0  blr
```

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80B0D168` | `0x80169020` | `r3 = this + 0x640` |
| `0x80B0D178` | `0x80169020` | `r3 = this + 0x678` |

### Ambiguity

None. `0x80169020` is ledgered (`resource::registry`), takes the element
address in `r3` and returns the bound descriptor word (`0` when unbound). The
predicate is exactly "both handles are bound".

---

## 8. `0x80A2C2A0` - bounce-ball pair-contact raw-yaw helper

* **Address:** `0x80A2C2A0`
* **TSV size:** 76 bytes
* **Reachable body:** 19 instructions; 0 `bc`; 0 `bl`; ends at `0x80A2C2E8 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a2c2a0_FUN_80a2c2a0.c`
* **Proposed signature:** `pub unsafe fn fn_80a2c2a0(this: usize) -> i32`
* **Callers in this REL:** none by direct `bl`/`b`; referenced as the 32-bit
  word at `0x80B3BC14` in the flat callback list that precedes the
  `daEnBounceBall_c::StateID_Move` name block (`0x80B3BCC0`). Wave 105 groups
  it with the `0x80A2B490..0x80A2C2A0` pair-contact/impact cluster.
* **Constants:** see the table below.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-0x20(r1)`. No LR save and no GPR save: the body is a leaf. One
16-byte double scratch pair at `+0x08`/`+0x0C` and a 8-byte `stfd` slot at
`+0x10`.

### Constants

| address | type | value |
|---|---|---|
| `0x80B2BBB8` | `f64` / `u64` | `0x4330000080000000` = `2^52 + 2^31` (`4503601774854144.0`) |
| `0x4330` / `0x8000` | literals | `lis r0,0x4330` and `xoris r4,r4,0x8000` build the `2^52` bias |
| `4096` | literal | `li r4,4096` - the integer the bias converts (dead result) |

`0x80B2BBB8` is in the third `PT_LOAD` (`p_vaddr = 0x80B2A7B0`,
`p_offset = 0x12B7B0`), file offset `0x12CBB8`.

### Behaviour

```
0x80A2C2A0  stwu r1,-0x20(r1)
0x80A2C2A4  li r4,4096
0x80A2C2A8  lis r0,0x4330
0x80A2C2AC  xoris r4,r4,0x8000
0x80A2C2B0  stw r4,0x0C(r1)
0x80A2C2B4  lis r4,0x80B3
0x80A2C2B8  lfs f0,0xE8(r3)
0x80A2C2BC  stw r0,0x08(r1)
0x80A2C2C0  lfd f2,-0x4448(r4)      ; f2 = *(f64*)0x80B2BBB8
0x80A2C2C4  lfd f1,0x08(r1)        ; f1 = 2^52 + 0x80001000
0x80A2C2C8  fsubs f1,f1,f2         ; f1 = 4096.0f  (DEAD)
0x80A2C2CC  fmuls f0,f0,f0         ; f0 = (f32)(x*x), x = *(f32*)(this+0xE8)
0x80A2C2D0  fctiwz f0,f0           ; f0 = (double)(i32)trunc(f0)
0x80A2C2D4  stfd f0,0x10(r1)
0x80A2C2D8  lwz r0,0x14(r1)        ; r0 = low word = the i32
0x80A2C2DC  neg r0,r0
0x80A2C2E0  extsh r3,r0            ; r3 = (i32)(i16)(-n)
0x80A2C2E4  addi r1,r1,0x20
0x80A2C2E8  blr
```

```c
i32 fn_80a2c2a0(usize this) {
    f32 x = read_f32(this + 0xE8);
    // 0x80A2C2A4..0x80A2C2C8 computes 4096.0f into f1 and never reads it.
    let n = fctiwz(x * x);          // f32 product, truncate toward zero
    (-n as i16) as i32              // neg r0,r0 ; extsh r3,r0
}
```

* `fsubs` is single-precision, so the exact result is `4096.0f`, not
  `4096.0` in double. Nothing consumes it; a port may delete the whole
  `0x80A2C2A4..0x80A2C2C8` block without changing the result.
* `fctiwz` truncates toward zero. On Gekko an out-of-range product yields
  `0x80000000`; `neg` of that is `0x80000000` and `extsh` then returns `0`, so
  the boundary behaviour is fixed by the ISA and needs no extra modelling.
* This is the row waves 92/123 skipped as "the yaw scale cannot be recovered".
  The 2^52 value is *not* the yaw scale - it is an unused conversion of the
  literal 4096. Ghidra folded the stack double to `2^52` and lost the
  `0x80001000` low word, which is why its export multiplies by
  `(4.50360177485824e+15 - DAT_80b2bbb8)`.

### Call sites

None. The body is a leaf.

### Ambiguity

None. No GQR, no `psq`, no incoming FPR. The only "constant" that reaches the
result is the actor's `+0xE8` word; the in-image double is dead.

---

## 9. `0x80B0AF60` - SyncroBarnar kind-`0x99` lookup and timer seed

* **Address:** `0x80B0AF60`
* **TSV size:** 76 bytes
* **Reachable body:** 19 instructions; 1 `bc`; 2 `bl`; ends at `0x80B0AFA8 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b0af60_FUN_80b0af60.c`
* **Proposed signature:** `pub unsafe fn fn_80b0af60(this: usize)`
* **Callers in this REL:** none by direct `bl`/`b`; referenced as the word at
  `0x80B67B70` in a 12-byte descriptor table whose neighbours are
  `0x80B0AF50` and `0x80B0AF30`. The table's state-name block starts at
  `0x80B67BC8` (`daEnSyncroBarnar_c::StateID_Syncro` / `Wait` / `Fire`), so
  this is the SyncroBarnar state callback row.
* **Constants:** none (offsets `+0x9B4` and the two callee addresses only).
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-0x10(r1)`; LR `+0x14`; `r31@+0x0C` = `this`.

### Behaviour

```
0x80B0AF60  stwu r1,-0x10(r1)
0x80B0AF64  mflr r0
0x80B0AF68  li r4,0
0x80B0AF6C  stw r0,0x14(r1)
0x80B0AF70  stw r31,0x0C(r1)
0x80B0AF74  mr r31,r3
0x80B0AF78  bl 0x80B0AC50
0x80B0AF7C  cmpwi cr0,r3,0
0x80B0AF80  mr r4,r3
0x80B0AF84  beq 0x80B0AF98
0x80B0AF88  lwz r0,0(r3)
0x80B0AF8C  mr r3,r31
0x80B0AF90  stw r0,0x9B4(r31)
0x80B0AF94  bl 0x80B0ABF0
0x80B0AF98  lwz r0,0x14(r1)
0x80B0AF9C  lwz r31,0x0C(r1)
0x80B0AFA0  mtlr r0
0x80B0AFA4  addi r1,r1,0x10
0x80B0AFA8  blr
```

```c
void fn_80b0af60(usize this) {
    let p = fn_80b0ac50(this) as usize;   // 0x80B0AF68 li r4,0 ; 0x80B0AF78 bl
    if (p != 0) {                          // 0x80B0AF7C cmpwi ; 0x80B0AF84 beq
        write_u32(this + 0x9B4, read_u32(p));   // 0x80B0AF88 lwz r0,0(r3) ; 0x80B0AF90 stw
        fn_80b0abf0(this, p);                   // 0x80B0AF8C mr r3,r31 ; 0x80B0AF94 bl
    }
}
```

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80B0AF78` | `0x80B0AC50` | `r3 = this`, `r4 = 0` (callee reads only `r3`; see below) |
| `0x80B0AF94` | `0x80B0ABF0` | `r3 = this`, `r4 = p` |

### Ambiguity

The only earlier objection (`COVERAGE_WAVE115_ENEMIES.md`) was that
`0x80B0AC50` "does not declare" its second argument. The callee's body is a
recursive one-argument search (`fn_80b0ac50(this) -> usize` in Rust, ledger
row `game::enemies,80b0ac50`), so the `li r4,0` is a leftover register fill and
the port passes one argument. The `mr r4,r3` at `0x80B0AF80` happens *before*
the `beq`, so `r4 = p` on the non-null path is not conditional. `p` is the
matched class-`0x99` record; `*(u32*)p` is its id word.

---

## 10. `0x80B0DAF0` - Teresa `+0x9E7` latch reset

* **Address:** `0x80B0DAF0`
* **TSV size:** 76 bytes
* **Reachable body:** 19 instructions; 1 `bc`; 1 `bl`; ends at `0x80B0DB38 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b0daf0_FUN_80b0daf0.c`
* **Proposed signature:** `pub unsafe fn fn_80b0daf0(this: usize)`
* **Callers in this REL:** none by direct `bl`/`b`; referenced as the word at
  `0x80B68318` in the Teresa descriptor table (neighbours `0x80B0DB40`,
  `0x80B0DB50`, `0x80B0DC30`, `0x80B0DC40`; state-name block
  `daEnTeresa_c::StateID_Move`/`Stop`/`Recover` at `0x80B68364`).
* **Constants:** none.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-0x10(r1)`; LR `+0x14`; `r31@+0x0C` = `this`.

### Behaviour

```
0x80B0DAF0  stwu r1,-0x10(r1)
0x80B0DAF4  mflr r0
0x80B0DAF8  li r4,0
0x80B0DAFC  stw r0,0x14(r1)
0x80B0DB00  stw r31,0x0C(r1)
0x80B0DB04  mr r31,r3
0x80B0DB08  lbz r0,0x9E7(r3)
0x80B0DB0C  sth r4,0x9E0(r3)
0x80B0DB10  cmpwi cr0,r0,0
0x80B0DB14  bne 0x80B0DB20
0x80B0DB18  li r4,0
0x80B0DB1C  bl 0x80B0CD80
0x80B0DB20  li r0,0
0x80B0DB24  stb r0,0x9E7(r31)
0x80B0DB28  lwz r31,0x0C(r1)
0x80B0DB2C  lwz r0,0x14(r1)
0x80B0DB30  mtlr r0
0x80B0DB34  addi r1,r1,0x10
0x80B0DB38  blr
```

```c
void fn_80b0daf0(usize this) {
    write_u16(this + 0x9E0, 0);          // 0x80B0DB0C sth r4 (r4 == 0 from 0x80B0DAF8)
    if (read_u8(this + 0x9E7) == 0)      // 0x80B0DB08 lbz ; 0x80B0DB10 cmpwi ; 0x80B0DB14 bne skip
        fn_80b0cd80(this, 0);            // 0x80B0DB18 li r4,0 ; 0x80B0DB1C bl
    write_u8(this + 0x9E7, 0);           // 0x80B0DB20 li r0,0 ; 0x80B0DB24 stb
}
```

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80B0DB1C` | `0x80B0CD80` | `r3 = this`, `r4 = 0` |

### Ambiguity

**The earlier prose has the polarity backwards.**
`COVERAGE_WAVE115_ENEMIES.md` and the wave-127 table both say "if `+0x9E7` is
set, `0x80B0CD80(0)`; always clears `+0x9E0`". The word at `0x80B0DB14` is
`bc 4,2` (`bne`) to the instruction after the call, so the call happens when
`+0x9E7 == 0`; and the byte cleared after the call is `+0x9E7`, while `+0x9E0`
is a `u16` cleared *before* the test. The Ghidra export agrees with the bytes
(`if (*(char *)(param_1 + 0x9e7) == '\0')`). A port must follow the bytes.

`0x80B0CD80` is 292 bytes and **not** ledgered, so corrupting its signature is
the only real risk: its export prints `FUN_80b0cd80(void)` because its
`_savegpr` helper hides the incoming `r3`. The call site fixes the signature at
`(this, mode)`; declare it by address (`call2(0x80B0_CD80, this, 0)`) until a
later lane translates it.

---

## 11. `0x80A78700` - Hnswich release helper

* **Address:** `0x80A78700`
* **TSV size:** 64 bytes
* **Reachable body:** 16 instructions; 0 `bc`; 2 `bl`; ends at `0x80A7873C blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a78700_FUN_80a78700.c`
* **Proposed signature:** `pub unsafe fn fn_80a78700(this: usize)`
* **Callers in this REL:** none by direct `bl`/`b`; referenced as the word at
  `0x80B4C4D8` in the Hnswich descriptor table (neighbours `0x80A78750`,
  `0x80A786E0`, `0x80A786F0`; state-name block `daEnHnswich_c::StateID_Wait`
  at `0x80B4C500`).
* **Constants:** none.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-0x10(r1)`; LR `+0x14`; `r31@+0x0C` = `this`.

### Behaviour

```
0x80A78700  stwu r1,-0x10(r1)
0x80A78704  mflr r0
0x80A78708  stw r0,0x14(r1)
0x80A7870C  li r0,0
0x80A78710  stw r31,0x0C(r1)
0x80A78714  mr r31,r3
0x80A78718  stb r0,0x124(r3)
0x80A7871C  addi r3,r3,0x148
0x80A78720  bl 0x8008C380
0x80A78724  addi r3,r31,0x754
0x80A78728  bl 0x8007F950
0x80A7872C  lwz r0,0x14(r1)
0x80A78730  lwz r31,0x0C(r1)
0x80A78734  mtlr r0
0x80A78738  addi r1,r1,0x10
0x80A7873C  blr
```

```c
void fn_80a78700(usize this) {
    write_u8(this + 0x124, 0);          // 0x80A7870C li r0,0 ; 0x80A78718 stb
    collision_area_unregister(this + 0x148);   // 0x80A7871C addi ; 0x80A78720 bl 0x8008C380
    fn_8007f950(this + 0x754);                 // 0x80A78724 addi ; 0x80A78728 bl 0x8007F950
}
```

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A78720` | `0x8008C380` (`collision_area_unregister`) | `r3 = this + 0x148` |
| `0x80A78728` | `0x8007F950` | `r3 = this + 0x754` |

### Ambiguity

None. `COVERAGE_WAVE16/17_ENEMIES.md` call this "duplicate of `0x80A786A0`;
one body is enough". Verified: the 16 words are identical to `fn_80a786a0`
(already translated and ledgered) except for the two `bl` displacements. The
original still has a separate entry, so the port should either forward to
`fn_80a786a0` or carry a copy; the module doc convention
(`COVERAGE_WAVE19_ENEMIES.md`) is to keep one Rust body per entry.

---

## 12. `0x80AA0300` - `0x80064680(0xA5, actor, flags, point, 0, 0)` spawn thunk

* **Address:** `0x80AA0300`
* **TSV size:** 52 bytes
* **Reachable body:** 13 instructions; 0 `bc`; 0 `bl`; 1 tail `b`; ends at
  `0x80AA0330 b 0x80064680`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aa0300_FUN_80aa0300.c`
  - **truncated** (`halt_baddata`), so the export is not usable as a plan; the
  decoded body below is authoritative.
* **Proposed signature:** `pub unsafe fn fn_80aa0300(actor: usize)`
* **Callers in this REL:** one direct call, `0x80AA09F8` (`mr r3,r30; bl
  0x80AA0300`) inside the per-frame body whose actor is `r30`. No table word
  references it.
* **Constants:** `165` = `0xA5` (spawn kind) and offsets `+0x348`, `+0x75C`,
  `+0x872`, `+0x876`. No in-image data word.
* **Verdict:** **ready** (confidence high)

### Frame

None. The body is a pure argument thunk: no stack frame, no LR save, no
callee-saved register.

### Behaviour

```
0x80AA0300  mr r4,r3              ; r4 = actor (incoming r3)
0x80AA0304  li r3,165             ; arg1 = 0xA5
0x80AA0308  lbz r0,0x348(r4)      ; u8  @ actor+0x348
0x80AA030C  addi r6,r4,0x75C      ; arg4 = actor+0x75C
0x80AA0310  lbz r7,0x876(r4)      ; u8  @ actor+0x876
0x80AA0314  li r8,0               ; arg6 = 0
0x80AA0318  lhz r5,0x872(r4)      ; u16 @ actor+0x872
0x80AA031C  rlwinm r0,r0,4,0,27   ; r0 = (u8 << 4) & 0xFFFFFFF0
0x80AA0320  rlwinm r9,r7,8,0,23   ; r9 = (u8 << 8) & 0xFFFFFF00
0x80AA0324  li r7,0               ; arg5 = 0
0x80AA0328  or r0,r5,r0           ; r0 = u16 | shifted byte
0x80AA032C  or r5,r9,r0           ; arg3 = high byte | r0
0x80AA0330  b 0x80064680          ; tail call, 6 live arguments
```

```c
void fn_80aa0300(usize actor) {
    let b348 = read_u8(actor + 0x348) as u32;
    let b876 = read_u8(actor + 0x876) as u32;
    let h872 = read_u16(actor + 0x872) as u32;

    let flags = (h872 | ((b348 << 4) & 0xFFFF_FFF0) | ((b876 << 8) & 0xFFFF_FF00)) as u32;

    // 0x80064680: r3..r8 are the live arguments, r9/r10 are not read.
    call6(
        0x8006_4680,
        0xA5,
        actor as u32,
        flags,
        (actor + 0x75C) as u32,
        0,
        0,
    );
}
```

* `r9` is also left holding `(b876 << 8) & 0xFFFFFF00`, but `0x80064680`'s
  prologue only consumes `r3..r8` (it saves `r3..r7` into `r27..r31`, forwards
  them to `0x8006CBA0`, and passes `r8` to `0x80064600`), so `r9` is not a
  seventh argument. If a port models the full register picture anyway, the
  value is exactly `(b876 << 8) & 0xFFFFFF00`.
* `flags` keeps the low nibble of `h872` because the `u16` is OR-ed in
  unshifted; bits 4-11 come from `b348` and bits 8-15 from `b876`. Do not
  "clean" the overlap.
* The `0x80AA09F8` call site ignores the return value; the tail call's `r3`
  comes from `0x80064680`, which returns `void`, so the thunk is `-> ()`.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80AA0330` | `0x80064680` (tail `b`) | `r3 = 0xA5`, `r4 = actor`, `r5 = flags`, `r6 = actor+0x75C`, `r7 = 0`, `r8 = 0` |

### Ambiguity

The Ghidra export is `halt_baddata` because the tail branch leaves the bucket;
the thirteen words at `0x80AA0300..0x80AA0330` above are the complete body (52
bytes = `TSV size / 4`). `0x80064680` has no
`ledger/functions.csv` row, but its shape is documented in
`COVERAGE_REL_d_basesNP.md` ("actor resource registration") and
`COVERAGE_WAVE122_ENEMIES.md` uses the same six-argument call
(`0x80064680(0x9F, actor, flags, point, 0, actor[+0x38F])`). Nothing in the
thunk depends on an unresolved value, which is why the row is ready rather
than conditional.

---

## Cross-references and collisions

All twelve addresses were checked against the `address` column of
`ledger/functions.csv` (absent), against the function-definition scan of
`src/game/enemies.rs` (no `fn_80...` body for any of them), and against every
`0x80Axxxxx` / `0x80Bxxxxx` token in every `#`-heading of every `docs/*.md`
(absent). They are *named* in prose in earlier waves, always as a neighbour,
skip-list member or callee, never as a specified body:

| address | where it is mentioned | in what role |
|---|---|---|
| `0x80A43DC0` | `ANALYSIS_WAVE127_ENEMIES.md` (B table), `COVERAGE_WAVE126_ENEMIES.md:133` (skip list), `SPEC_WAVE136_ENEMIES.md:62` (next-band list) | "byte-identical to `0x80A446B0`"; never a heading |
| `0x80A446B0` | `ANALYSIS_WAVE127_ENEMIES.md`, `COVERAGE_WAVE126_ENEMIES.md:132`, `COVERAGE_WAVE93_ENEMIES.md:160`, `SPEC_WAVE136_ENEMIES.md:62` | skip-list member |
| `0x80AECF00` | `COVERAGE_WAVE122_ENEMIES.md:121/146`, `SPEC_WAVE136_ENEMIES.md:62` | teardown group member |
| `0x80AEE6D0` | `COVERAGE_WAVE122_ENEMIES.md:124`, `SPEC_WAVE136_ENEMIES.md:62` | teardown group member |
| `0x80AEF910` | `COVERAGE_WAVE122_ENEMIES.md:126`, `SPEC_WAVE136_ENEMIES.md:62` | teardown group member |
| `0x80A7ECA0` | `COVERAGE_WAVE121_ENEMIES.md:137`, `COVERAGE_WAVE17_ENEMIES.md:106`, `COVERAGE_WAVE19_ENEMIES.md:182` | "calls `0x80A7E630`, whose export is `halt_baddata()`" |
| `0x80B0D150` | `COVERAGE_WAVE115_ENEMIES.md:129`, `SPEC_WAVE131_ENEMIES.md:847/851/860` | "readable, but the lane budget was spent"; later a call-site row (`r3 = this`) |
| `0x80A2C2A0` | `COVERAGE_WAVE105_ENEMIES.md:85`, `COVERAGE_WAVE123_ENEMIES.md:117/124`, `COVERAGE_WAVE92_ENEMIES.md:148` | pair-contact cluster; "yaw scale cannot be recovered" |
| `0x80B0AF60` | `COVERAGE_WAVE115_ENEMIES.md:123`, `COVERAGE_WAVE97_ENEMIES.md:119` | "second argument the callee does not declare" |
| `0x80B0DAF0` | `COVERAGE_WAVE115_ENEMIES.md:115` | consumer of the blocked GQR group |
| `0x80A78700` | `COVERAGE_WAVE121_ENEMIES.md:131`, `COVERAGE_WAVE16_ENEMIES.md:187`, `COVERAGE_WAVE17_ENEMIES.md:172` | "duplicate of `0x80A786A0`" |
| `0x80AA0300` | `COVERAGE_WAVE125_ENEMIES.md:129`, `COVERAGE_WAVE75_ENEMIES.md:150` | truncated-export skip list |

Three readings need reconciling, none of them blocking:

1. **`0x80B0DAF0` polarity.** The wave-115 and wave-127 prose says the call is
   made when `+0x9E7` is set. The bytes (`bc 4,2` = `bne` past the call) and
   the Ghidra export say it is made when `+0x9E7 == 0`. Rank 10 follows the
   bytes.
2. **`0x80A2C2A0`'s "yaw scale".** Waves 92/123 concluded the scale cannot be
   recovered because Ghidra folds the `2^52` stack double. Wave 127
   reclassified it B on the grounds that the converted value is never
   consumed; this page confirms that reading instruction by instruction
   (`0x80A2C2C8` writes `f1`, nothing reads it). The real result is
   `-(i16)(i32)(x*x)` with `x = *(f32*)(this+0xE8)`.
3. **`0x80A78700`'s duplication.** Waves 16/17 said "one body is enough"; the
   original still has two entries, and `fn_80a786a0` is already translated.
   The repo keeps one Rust body per entry (`COVERAGE_WAVE19_ENEMIES.md`), so
   rank 11 should be a forwarder or a copy, not a deletion.

One correction to wave 136's prose: its next-band note listed
`0x80AFEC20` and `0x80AFF4F0` as unclaimed 22-instruction rows. Both now have
`ledger/functions.csv` rows (`0x80AFF4F0`/`0x80AFEC20`, same teardown family),
and `0x80A1F390`/`0x80A1FFB0`/`0x80ADD0E0` were already ledgered, so this
lane's 22-instruction band has exactly five open members - ranks 1-5.

New helper claims: none. `0x80B0CD80`, `0x80A7E630` and `0x80064680` are
declared by address (`call2` / `call1` / `call6`) as the module already does
for untranslated callees; `0x80B73338` is a BSS descriptor constant.

## Verification

* **Selection.** The twelve addresses are the first twelve non-ledger,
  non-specified, non-Rust class-B rows below wave 136's last pick: reachable
  counts `22, 22, 22, 22, 22, 21, 21, 19, 19, 19, 16, 13`, TSV sizes
  `88, 88, 88, 88, 88, 84, 84, 76, 76, 76, 64, 52`. The 22-instruction band
  is entered at its lowest open address and exhausted (five rows); the
  21-instruction band contributes two, the 19-instruction band three, then one
  each from 16 and 13. Ties are broken by ascending address; no ledger row or
  specified body sits *between* the twelve chosen ones.
* **Sizes.** Every TSV size above matches the `size` column of
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`, and in all twelve the
  reachable span is exactly `TSV size / 4` instructions with no trailing
  unreachable words.
* **Control flow.** 15 conditional branches, all with in-body targets; 18
  `bl`; one `bctrl` (rank 6, vtable slot `+0xD4`); one intra-body `b` (rank 7,
  skips the `li r3,0`); one tail `b` (rank 12, to `0x80064680`); 11
  terminating `blr`s. No body reads `f1`/`f2`/`f3`/`f4` before writing it, and
  there is no `psq` op anywhere in the tranche.
* **Constants.** `0x80B2BBB8 = 0x4330000080000000` (double `2^52 + 2^31`) was
  read from `work/rels/elf/d_enemiesNP.elf` at file offset `0x12CBB8`
  (third `PT_LOAD`, `p_vaddr = 0x80B2A7B0`, `p_offset = 0x12B7B0`).
  `0x80B73338` lies in the zero-fill segment `0x80B6E500..0x80B799E0` and has
  no file bytes; its role is the wave-19 `Vibration` descriptor naming. No
  other in-REL constant is consumed by any of the twelve bodies.
* **Entry references.** Direct-branch and data-word scans over the whole REL
  agree with the "callers" lines: `0x80B0D150` <- `0x80B0DCC4`, `0x80B0DD18`,
  `0x80B0DE70`; `0x80AA0300` <- `0x80AA09F8`; the other ten appear only as
  32-bit words in the vtable/descriptor blocks quoted per section.
* **Byte identity.** Ranks 1-5 are word-for-word identical to each other and
  to the ledgered `fn_80a1f390` / `fn_80a1ffb0` bodies after normalising
  `bl`/`bc` displacements (checked mechanically on all 22 words). Rank 11 is
  word-identical to `fn_80a786a0` except the two `bl` displacements.
* **Decode cross-check.** The five deleting destructors and ranks 6/7/9/10/11
  agree with the Ghidra C exports line for line. The two exports that do not
  are documented above: rank 8 (Ghidra folds the `2^52` stack double) and
  rank 12 (`halt_baddata` truncation).
* **Ledger/doc state.** No ledger row, Rust file, CSV or other doc was written
  by this pass. `docs/SPEC_WAVE137_ENEMIES.md` is the only artefact. A
  concurrent implementation lane is editing `src/game/enemies.rs` while this
  page is written; at the time of writing none of the twelve has a `fn_80...`
  definition there, but five are already *named* as neighbours or call targets
  (`0x80AECF00` in a doc comment, `0x80B0D150` as the `W132_FN_80B0D150` call
  constant, `0x80B0DAF0` and `0x80A78700` in comments, `0x80AA0300` in a
  comment) - all as callers/callees, never as a body. The binding selection
  criterion is the ledger plus the spec headings, exactly as in waves 128-136.
