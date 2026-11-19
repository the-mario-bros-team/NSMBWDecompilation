# Wave 155 - the final eight `d_enemiesNP` addresses

## What this page is

`docs/ANALYSIS_WAVE127_ENEMIES.md` reclassified 384 formerly-skipped
`d_enemiesNP` addresses; waves 128-154 worked through that block and every
later tranche. This page is the **last** specification pass for the module: it
takes the addresses that were still absent from all four tracking sets and
gives each one an implementation-ready specification.

Snapshot: **2026-11-13 09:15** for the selection scan below. At that moment
`src/game/enemies.rs` was 7,302,223 bytes (modified 09:10:49),
`ledger/functions.csv` 3,288,873 bytes (08:56:56) and
`work/ghidra/rel-d_enemiesNP/export/functions.tsv` 3,555 rows, all
`decompiled = true`. The Rust file kept growing during the pass (7,311,789
bytes at 09:25) as the sibling translation lane described below appended
bodies.

A live scan of

* the `address` column of `ledger/functions.csv` - **17,169** distinct
  addresses,
* every `0x80xxxxxx` address in a `^\s*#` heading of any `docs/*.md` -
  **1,618** addresses (416 of them in the `0x80A`/`0x80B` range, including the
  eight headings this page adds; the opening scan had 1,610 / 408), across the
  28 `docs/SPEC_WAVE*_ENEMIES.md` pages and the rest of the documentation,
* every `fn fn_<addr>` **body** defined under `src/` - **16,005** definitions
  (16,004 distinct) across 68 `.rs` files,

leaves exactly **eight** addresses that appeared in none of them at the opening
snapshot (09:10:30, minutes before the append described next). They are all
that remains: the module has no smaller unspecced candidate, so this page lists
**8, not 12**.

### The tranche changed while this page was being prepared

At 09:10:49 - during this pass - a parallel wave-155 **translation** lane
appended bodies for exactly these eight addresses (plus nineteen others) to
`src/game/enemies.rs`. Consequences, stated plainly:

* the four-set gap is now **empty**: every one of the 3,555 `functions.tsv`
  rows is now in the ledger, in a `^#` heading, or has a body;
* the ledger has not been merged yet, so all **25** addresses that were absent
  from `ledger/functions.csv` at this snapshot are still absent from it (8 from
  this page plus 17 that earlier spec pages already cover - see the module list
  at the end);
* this page keeps the eight sections because they were the module's only
  unspecced addresses, because they are what those fresh bodies must be checked
  against, and because five of them are `partial` in substance no matter who
  wrote them.

This page is **read-only**: it writes no Rust, no ledger row and no other doc.

## Selection

The ranking metric is the `functions.tsv` extent. For each candidate the
reachable span was walked from the entry through every conditional branch, every
internal `b` and every linked `bctrl` until all paths ended in `bclr`, `blr`, a
tail `b` or a tail `bcctr`; conditional `bclr` forms were followed on both
edges. In all eight the last instruction is at `entry + size - 4`, so
`reachable == extent`: no truncated arm, no out-of-range branch, no `bctr` jump
table.

| # | address | TSV size | reachable insns | `qr3` sites | role | verdict |
|---:|---|---:|---:|---:|---|---|
| 1 | `0x80A25F00` | 48 | 12 | 0 | `daEnBlueCoin_c` object this-adjust destructor, descriptor `+0x24` | **conditional** |
| 2 | `0x80A2F9D0` | 48 | 12 | 0 | `daEnBubble_c` destructor adjustor, descriptor `+0x18` | **conditional** |
| 3 | `0x80A2FA00` | 48 | 12 | 0 | `daEnBubble_c` destructor adjustor, descriptor `+0x24` | **conditional** |
| 4 | `0x80A43EE0` | 48 | 12 | 0 | `Wm_ob_coin` destructor adjustor, descriptor `+0x18` | **conditional** |
| 5 | `0x80A43F10` | 48 | 12 | 0 | `Wm_ob_coin` destructor adjustor, descriptor `+0x24` | **conditional** |
| 6 | `0x80A07C80` | 16 | 4 | 0 | read the `+0x5B4` holder's matrix into `+0x710` | **ready** |
| 7 | `0x80AF3840` | 16 | 4 | 0 | guide-angle step, `+0x7A4`, step `400` | **ready** |
| 8 | `0x80A337B0` | 4 | 1 | 0 | 4-byte alias of the translated `0x80A337C0` | **ready** |

**Split: 3 ready, 5 conditional.** Total 276 bytes of TSV extent and 69
reachable instructions.

The five conditional rows are not blocked on unknown bytes: every operand is
decoded. They share one bounded gap - the **hidden `r12` this-adjust
descriptor**, which `platform::call3` cannot model. The repo already ships 188
bodies of exactly this shape (`call3(FN_802DCBFC, second, second, first)` in
`src/game/enemies.rs`, among them `fn_80a6c810`, `fn_80a6d520`, `fn_80a25ea0`
and the `+0x18` sibling `fn_80a25ed0`), so these rows inherit a convention
rather than inventing one. Section 4 gives a byte-exact expansion for a harness
that can model the descriptor directly.

## Reading conventions

* *Synthetic base* is `0x80A00000`. The REL has five file-backed `PT_LOAD`
  segments (`0x80A000F0`, `0x80B2A518`, `0x80B2A7A4`, `0x80B2A7B0`,
  `0x80B32DC0`); `file_offset = p_offset + (address - p_vaddr)`, which for the
  first segment is `address - 0x80A00000 + 0x1000`. All eight addresses below
  live in the first segment.
* `this` is the incoming `r3` unless stated. `+0x7A4` is the world-map guide
  angle (`s16`), `+0x5B4` an effect holder, `+0x710` a 12-word (3x4) matrix
  buffer, `+0x6CC`/`+0x6D0` an effect x/y pair.
* `addi rX,r0,imm` is `li rX,imm` and `addis rX,r0,imm` is `lis rX,imm`
  (RA = 0 means the literal zero, never `r0`). Negative `addi` offsets are
  written resolved: `addis r4,0,0x80B4` + `addi r4,r4,-0x5400` =
  `0x80B3AC00`.
* A `functions.tsv` size is a bucket extent; each section's "reachable" line is
  the count from the control-flow walk, which agrees with the extent here.
* `nop` is `0x60000000` (`ori r0,r0,0`).
* Some `functions.tsv` neighbours are not functions at all: `0x80A07930` (the
  only caller of row 6) has **no** TSV row because nothing branches to it - it
  is installed from data. It is quoted as a body region, not as a TSV row.

## The class-record framing used in sections 1-5

The five conditional rows sit in three class blocks - one data shape this module
repeats for every class whose destructor chain it emits. The `daEnBubble_c`
block emits two adjacent method runs; the other two emit one. Verified against
all three below:

```
+0x00  u32 0                     header word 0
+0x04  u32 0                     header word 1
+0x08  N x u32                   method words; word 0 is the deleting destructor
+...   u32                       factory / in-charge constructor
+...   u16 kind, u16 kind_alt    actor kind pair
+...   u32                       profile word
+...   char[]                    class/profile name, or 0
```

`N` is 11 in the `daEnBubble_c` and `Wm_ob_coin` records (factory at `+0x34`,
kind pair `+0x38`, profile word `+0x3C`, name `+0x40`) and 12 in the
`daEnBlueCoin_c` record, whose twelfth method word is a zero adjustor
(factory at `+0x38`, kind pair `+0x3C`, profile word `+0x40`, name `+0x44`).
The object's vtable pointer is the **header** address (`0x80B40708` is stored at
`+0x60` by `fn_80a43f40` at `0x80A44020`/`0x80A4402C`; `0x80B3AC00` likewise by
`fn_80a25f30`), which is why the installed table appears to start with two zero
words. Method indices below are counted from the **first method word**
(`header + 8`), not from the header, and every row also quotes the referencing
**word address**, so nothing depends on the framing.

In all three families the last method words are the destructor adjustors for
the embedded sub-objects, so a row's *byte* offset from the first method word is
`(referencing_word - first_method_word)`.

---

## 1. `0x80A25F00` - `daEnBlueCoin_c` object this-adjust destructor, `+0x24`

| field | value |
|---|---|
| TSV size | 48 bytes |
| reachable | 12 instructions, ends at `0x80A25F2C` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80a25f00_FUN_80a25f00.c` |
| signature | `pub unsafe fn fn_80a25f00(object: u32, receiver: u32)` |
| verdict | **conditional** |

### What the stream does

```c
void fn_80a25f00(u32 object, u32 receiver)      // r3 = object, r4 = receiver
{
    // hidden r12 = object + 0x24, the 12-byte pointer-to-member inside the object
    r3  = receiver;
    r4  = receiver;            // unchanged; 0x802DCBFC reads only r12 and r3
    r5  = object;
    r12 = object + 0x24;
    0x802DCBFC();              // descriptor dispatch, see section 4
}
```

Raw words (`0x80A25F00..0x80A25F2C`):

```
stwu  r1,-16(r1)        mfspr r0,LR
or    r5,r3,r3          or    r3,r4,r4
stw   r0,20(r1)         addi  r12,r5,0x24
bl    0x802DCBFC        nop
lwz   r0,20(r1)         mtspr LR,r0
addi  r1,r1,16          blr
```

The body returns whatever the dispatched destructor leaves in `r3`; C++
destructors conventionally leave `this`, so `()` is the correct Rust type and
the 188 existing copies use it.

### Record evidence

| word | holds | note |
|---|---|---|
| `0x80B3ABB8` / `0x80B3ABBC` | `0`, `0` | record header |
| `0x80B3ABC0` | `0x80A25DB0` | method 0: translated deleting destructor `fn_80a25db0` |
| `0x80B3ABE0` | `0x80A25EA0` | method 8: adjustor `+0x0C` |
| `0x80B3ABE4` | `0x80A25ED0` | method 9: adjustor `+0x18`, translated as `fn_80a25ed0` |
| `0x80B3ABE8` | `0x80A25F00` | method 10: **this row**, `+0x24` |
| `0x80B3ABEC` | `0` | method 11: zero adjustor (this record's twelfth word) |
| `0x80B3ABF0` | `0x80A25F30` | factory: `0x540`-byte allocator, installs `0x80B3AC00` at `+0x60` |
| `0x80B3ABF4` | `0x025B026B` | kind pair `0x025B` / `0x026B` |
| `0x80B3ABF8` | `0` | profile word |
| `0x80B3ABFC` | `0` | no name word for this record |

Class identity: the state-name pool immediately before the record reads
`daEnBlueCoin_c::StateID_Wait` at `0x80B3AB78` and
`daEnBlueCoin_c::StateID_DemoUp` at `0x80B3AB98`, and `0x80A25F30` allocates the
same `0x540`-byte actor that `COVERAGE_WAVE105_ENEMIES.md` calls "block D's actor".
The row is therefore the third of a three-adjustor run (`+0x0C`, `+0x18`,
`+0x24`); the `+0x18` sibling is already a shipped `partial`, which fixes the
convention for this one.

### Verdict: conditional

Twelve literal instructions; only the hidden `r12` operand is unmodelled.
Keep the shipped convention
`call3(FN_802DCBFC, receiver, receiver, object)` with a `partial` comment and
record `+0x24`; section 4's expansion makes it byte-exact.

---

## 2-3. `0x80A2F9D0` and `0x80A2FA00` - `daEnBubble_c` destructor adjustors

| field | `0x80A2F9D0` | `0x80A2FA00` |
|---|---|---|
| TSV size | 48 bytes | 48 bytes |
| reachable | 12, ends `0x80A2F9FC` (`blr`) | 12, ends `0x80A2FA2C` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80a2f9d0_FUN_80a2f9d0.c` | `work/ghidra/rel-d_enemiesNP/export/80a2fa00_FUN_80a2fa00.c` |
| descriptor | `object + 0x18` | `object + 0x24` |
| signature | `pub unsafe fn fn_80a2f9d0(object: u32, receiver: u32)` | `pub unsafe fn fn_80a2fa00(object: u32, receiver: u32)` |
| verdict | **conditional** | **conditional** |

Both are byte-identical to section 1 except for the `addi r12,r5,{0x18|0x24}`
word at `0x80A2F9E4` / `0x80A2FA14`. Same prologue (`stwu r1,-16`), same
`bl 0x802DCBFC` (`0x80A2F9E8` = `0x4B8AD215`, `0x80A2FA18` = `0x4B8AD1E5`),
same `nop` + epilogue.

### Record evidence

Both thunks are referenced twice, by two consecutive 11-word method runs:

| word | holds | note |
|---|---|---|
| `0x80B3C74C` | `0x80A2F9D0` | method 9 of the earlier run (starts `0x80B3C728`) |
| `0x80B3C750` | `0x80A2FA00` | method 10 of the earlier run |
| `0x80B3C754` / `0x80B3C758` | `0`, `0` | separator / header words |
| `0x80B3C75C` | `0x80A2F670` | method 0 of the described run: teardown `fn_80a2f670` |
| `0x80B3C780` | `0x80A2F9D0` | method 9: **row 2**, `+0x18` |
| `0x80B3C784` | `0x80A2FA00` | method 10: **row 3**, `+0x24` |
| `0x80B3C788` | `0x80A2FA30` | factory: `0x6950`-byte constructor `fn_80a2fa30` |
| `0x80B3C78C` | `0x016D017F` | kind pair `0x016D` / `0x017F` |
| `0x80B3C790` | `0` | profile word |
| `0x80B3C794` | `"g3d/env_underwater.brres"` | model path |
| `0x80B3C7B0` / `0x80B3C7C0` | `env_underwater`, `wmy_ef_bubble` | `daEnBubble_c` name pool |

**Correction to `docs/SPEC_WAVE154_ENEMIES.md`:** that page reads
`0x80B3C74C` as "slot `+0x1C` of the table at `0x80B3C730`" and `0x80B3C780`
as "slot 0 of the second table", implying two unrelated tables. What the image
shows is two *adjacent* 11-word runs that both end with the same two adjustor
words; `0x80B3C730` is not a table start (it is an interior word of the earlier
run), and the descriptor - factory `0x80A2FA30`, kind pair, model path - belongs
to the run that begins with the teardown at `0x80B3C75C`. Either way the thunk's
own bytes and `+0x18`/`+0x24` adjust are unaffected.

### Verdict: conditional

Same single bound as section 1: report `+0x18` / `+0x24` and use the shipped
`call3(FN_802DCBFC, receiver, receiver, object)` convention.

---

## 4-5. `0x80A43EE0` and `0x80A43F10` - `Wm_ob_coin` destructor adjustors

| field | `0x80A43EE0` | `0x80A43F10` |
|---|---|---|
| TSV size | 48 bytes | 48 bytes |
| reachable | 12, ends `0x80A43F0C` (`blr`) | 12, ends `0x80A43F3C` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80a43ee0_FUN_80a43ee0.c` | `work/ghidra/rel-d_enemiesNP/export/80a43f10_FUN_80a43f10.c` |
| descriptor | `object + 0x18` | `object + 0x24` |
| signature | `pub unsafe fn fn_80a43ee0(object: u32, receiver: u32)` | `pub unsafe fn fn_80a43f10(object: u32, receiver: u32)` |
| verdict | **conditional** | **conditional** |

Same twelve-word stream as sections 1-3; the adjust words are at `0x80A43EF4`
(`addi r12,r5,0x18`) and `0x80A43F24` (`addi r12,r5,0x24`), and the dispatcher
calls are `0x4B898D05` / `0x4B898CD5` to `0x802DCBFC`.

### Record evidence

| word | holds | note |
|---|---|---|
| `0x80B406BC` / `0x80B406C0` | `0`, `0` | record header |
| `0x80B406C4` | `0x80A43DC0` | method 0: translated deleting destructor `fn_80a43dc0` |
| `0x80B406E8` | `0x80A43EE0` | method 9: **row 4**, `+0x18` |
| `0x80B406EC` | `0x80A43F10` | method 10: **row 5**, `+0x24` |
| `0x80B406F0` | `0x80A43F40` | factory: `0xA00`-byte constructor `fn_80a43f40` |
| `0x80B406F4` | `0x0192019C` | kind pair `0x0192` / `0x019C` |
| `0x80B406F8` | `6` | profile word |
| `0x80B406FC` | `"Wm_ob_coin"` | class name |
| `0x80B40708` | `0x00000000` | the class's installed vtable header (`+0x60`), stored by `fn_80a43f40` at `0x80A44020`/`0x80A4402C` |

`fn_80a43f40` walks a constructor chain at `+0x60` - `0x80301EA8`
(`0x80A43F6C`), `0x80B3A828` (`0x80A43F88`), `0x80302D78` (`0x80A43F9C`) - and
finishes with `0x80B40708` (`0x80A44020`), which confirms the ledger row's
"class vtable `0x80B40708`" claim and the record framing above.

### Verdict: conditional

Identical bound and identical convention. Note for the ledger reviewer: the
five rows in sections 1-5 should carry `status = partial` (like
`fn_80a25ed0`/`fn_80a6c810`), not `translated`, until an `r12`-capable adapter
exists.

### The descriptor dispatch, exactly (`0x802DCBFC`, `main.dol`)

```
0x802DCBFC  lwz   r0,0(r12)      ; delta  (s32)
0x802DCC00  lwz   r11,4(r12)     ; index  (s32, used as a BYTE offset)
0x802DCC04  lwz   r12,8(r12)     ; target
0x802DCC08  add   r3,r3,r0       ; r3 = receiver + delta
0x802DCC0C  cmpwi r11,0
0x802DCC10  blt   0x802DCC1C
0x802DCC14  lwzx  r12,r3,r12     ; r12 = *(u32*)(receiver+delta+target)
0x802DCC18  lwzx  r12,r12,r11    ; r12 = *(u32*)(r12+index)
0x802DCC1C  mtspr CTR,r12
0x802DCC20  bctr                 ; tail call, r3 = receiver+delta
```

A byte-exact `r12`-free expansion of any row in sections 1-5 (all five differ
only in `OFF`):

```rust
pub unsafe fn fn_80a25f00(object: u32, receiver: u32) {
    const OFF: u32 = 0x24;                       // +0x18 in rows 2, 4
    let d = (object + OFF) as usize;
    let delta = platform::read_u32(d) as i32;
    let index = platform::read_u32(d + 4) as i32;
    let mut target = platform::read_u32(d + 8) as usize;
    let adjusted = (receiver as i32).wrapping_add(delta) as u32;
    if index >= 0 {
        target = platform::read_u32((adjusted.wrapping_add(target as u32)) as usize) as usize;
        target = platform::read_u32((target as u32).wrapping_add(index as u32)) as usize;
    }
    platform::call1(target, adjusted);
}
```

This reproduces the dispatcher instruction for instruction and removes the
`partial` marker; it is listed as an option, not a requirement, because the repo
has already chosen the `call3` convention for 188 siblings.

---

## 6. `0x80A07C80` - read the `+0x5B4` holder's matrix into `+0x710`

| field | value |
|---|---|
| TSV size | 16 bytes |
| reachable | 4 instructions, ends at `0x80A07C8C` (`b 0x80168180`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80a07c80_FUN_80a07c80.c` |
| signature | `pub unsafe fn fn_80a07c80(actor: usize) -> u32` |
| verdict | **ready** |

### What the stream does

```c
u32 fn_80a07c80(usize actor)
{
    return 0x80168180(actor + 0x5B4, 0, actor + 0x710);   // tail b
}
```

```
0x80A07C80  addi  r5,r3,0x710      ; out  = actor + 0x710 (3x4 matrix buffer)
0x80A07C84  li    r4,0             ; arg  = 0
0x80A07C88  addi  r3,r3,0x5B4      ; this = actor + 0x5B4 (the effect holder)
0x80A07C8C  b     0x80168180       ; 0x4B7604F4
```

### The callee contract exists - twice

`0x80168180` is **not** contractless; earlier waves and the body that landed at
09:10 were wrong about that:

* export: `work/ghidra/main-gb/export/80168180_FUN_80168180.c` - `piVar3 =
  *(int**)(param_1 + 4); ... local_18[0] = &DAT_802f8fd8;
  iVar2 = (**(code**)(*piVar3 + 8))(piVar3, local_18); ...
  FUN_8025b420(piVar3, param_3, 1, param_2);`
* translated body: `src/resource/registry.rs` `fn_80168180(this: usize,
  arg: u32, out: u32) -> u32` ("Casts the object at `+0x04` to the
  `0x802F8FD8` type and installs it"), ledger row
  `resource::registry,80168180,...`
* the module's own constant comment at `src/game/enemies.rs:13656`:
  "Reads one of the holder's matrices into a caller buffer (`main.dol`).
  The 12-word output is a 3x4 matrix; words 3, 7 and 11 are the translation."

Argument order is `(r3 = wrapper, r4 = arg, r5 = out)`, matching the staged
registers above exactly (`arg = 0`, `out = +0x710`).

### Why the row exists

`0x80A07C80` has one caller, `0x80A07954`, inside the **unexported** shared
state handler that starts at `0x80A07930` (104 bytes, `0x80A07930..0x80A07994`,
no `functions.tsv` row; installed from data at `0x80B33FB8`, `0x80B55B0C` and
`0x80B6790C`). That handler is the per-frame step of the fire-cannon family:

| site | call |
|---|---|
| `0x80A07944` | `fn_80a07ab0(actor)` - place the body holder at the actor position |
| `0x80A0794C` | `fn_80a07ba0(actor)` - place the effect holder, commit to `+0x5B4` |
| `0x80A07954` | **this row** - refresh `+0x710` from `+0x5B4` |
| `0x80A07958`-`0x80A07968` | virtual `+0x2A0` of the class table at `+0x60` |
| `0x80A07978` | if non-zero: `fn_80a079a0` - copies 28 bytes of `+0x710`, emits `Wm_en_fireburner3ind` |
| `0x80A07980` | else: `fn_80a07a30` - the second emitter, same `+0x710` matrix |

So this row is the only write of `+0x710` in the frame: without it both
translated emitters would read a stale matrix.

### Constants

| operand | value | use |
|---|---|---|
| `r3` | `actor + 0x5B4` | holder whose `+0x04` object is cast to `0x802F8FD8` |
| `r4` | `0` | instance flag of the install call |
| `r5` | `actor + 0x710` | 3x4 matrix output buffer |
| `0x80168180` | translated | matrix read/install (`resource::registry`) |

### Verdict: ready

Four literal words and a fully documented, already-translated callee. The body
that landed at 09:10 already passes the right three arguments in the right
order; only its "no export row exists / role unknown" comment and its `low`
confidence need replacing with the contract above.

---

## 7. `0x80AF3840` - guide-angle step, `+0x7A4`, step `400`

| field | value |
|---|---|
| TSV size | 16 bytes |
| reachable | 4 instructions, ends at `0x80AF384C` (`b 0x8015F4F0`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80af3840_FUN_80af3840.c` |
| signature | `pub unsafe fn fn_80af3840(this: usize, target: i16) -> u32` |
| verdict | **ready** |

### What the stream does

```c
u32 fn_80af3840(usize this, i16 target)
{
    r4 = (i16)target;                    // extsh r4,r4
    r5 = 400;                            // li r5,0x190
    r3 = this + 0x7A4;                   // addi r3,r3,0x7A4
    return 0x8015F4F0(r3, r4, r5);       // tail b
}
```

```
0x80AF3840  extsh r4,r4            ; target = (s16)r4
0x80AF3844  li    r5,0x190         ; step = 400
0x80AF3848  addi  r3,r3,0x7A4      ; &this->guide_angle
0x80AF384C  b     0x8015F4F0       ; 0x4B66BCA4
```

`0x8015F4F0` is the ledgered `game::task::step_short_clamped_wrap(value: *mut
i16, target: i32, step: i32) -> u32`: it returns `1` when the value reaches the
target on this call and `0` otherwise, and it snaps to the target on arrival. The
`extsh` truncates the incoming register to `s16` before the helper sees it, so an
`i16` parameter is exact.

### Callers (both already translated)

| site | caller | argument |
|---|---|---|
| `0x80AF36B0` | `fn_80af3690` (`Wm_en_burst_m` steer step) | `target` = the probe angle, pre-`extsh`ed at `0x80AF3698` |
| `0x80AF42F0` | `fn_80af42c0` (cannon sweep tail) | `li r4,0` - sweeps the angle back to `0` |

Both existing bodies call the address directly -
`call2(BURST_FN_80AF3840, this as u32, target as i32 as u32)` at
`src/game/enemies.rs:59915` and `call2(W154_FN_80AF3840, this as u32, 0)` at
`:198863` - so the call sites need no edit; only the body is new.

### Constants

| operand | value | use |
|---|---|---|
| `+0x7A4` | `i16` | current guide angle (world-map steering) |
| `0x190` | `400` | per-call step bound |
| `0x8015F4F0` | translated | `step_short_clamped_wrap` |

### Verdict: ready

Four literal instructions, one translated callee, two translated callers whose
argument construction already exists. Write it as
`call3(FN_8015F4F0, (this + 0x7A4) as u32, target as i32 as u32, 0x400)` (the
file's convention) or call `step_short_clamped_wrap` directly; both are
byte-exact.

---

## 8. `0x80A337B0` - 4-byte alias of the translated `0x80A337C0`

| field | value |
|---|---|
| TSV size | 4 bytes |
| reachable | 1 instruction, ends at `0x80A337B0` (`b 0x80A337C0`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80a337b0_thunk_FUN_80a337c0.c` |
| signature | `pub unsafe fn fn_80a337b0(this: usize)` |
| verdict | **ready** |

### What the stream does

```
0x80A337B0  48000010  b 0x80A337C0        ; +0x10
0x80A337B4  00000000                     ; padding, not part of the row
```

The target is already translated:

```rust
// src/game/enemies.rs:176975, ledger row game::enemies,80a337c0
pub unsafe fn fn_80a337c0(this: usize) {
    let x = read_f32(this + 0x6CC);
    let y = read_f32(this + 0x6D0);
    let manager = platform::read_u32(W139_PTR_80429DD0);
    let _ = w139_call5_f32x2(W139_FN_80078470, x, y, manager, 2, 4);
}
```

so the alias is exactly:

```rust
pub unsafe fn fn_80a337b0(this: usize) {
    fn_80a337c0(this);
}
```

### Caller

`0x80A3351C`, inside `fn_80a334d0` (the `ankou` per-frame step, 352 bytes,
translated). The gate is `*(u8*)(this + 0x390) == 0 && 0x8006A200(1) != 0`; the
translated caller at `:192881` already calls
`call1(0x80A3_37B0, this as u32)` with a comment naming the chain. Switching
that line to `fn_80a337b0(this)` is optional; the address call is equivalent
because the target is a `b`, not a `bl`.

### Verdict: ready

One word. The only reason this row has no body is a deliberate earlier decision
(`src/game/enemies.rs:111719` "deliberately left untranslated ... 4- and 32-byte
stubs", and `:176974` "needs no body of its own"), which
`COVERAGE_WAVE144_ENEMIES.md` repeated. That decision is defensible for the
*call graph* but leaves a `functions.tsv` row with no ledger entry; the one-line
forwarder closes the account at zero semantic cost. Note that
`docs/SPEC_WAVE154_ENEMIES.md`'s "the target is unclaimed, so there is no body to
type" is stale - `0x80A337C0` has been in the ledger since wave 139.

---

## Module completion list

At this snapshot **25** `d_enemiesNP` addresses are still absent from
`ledger/functions.csv`. All 25 now have a specification; this page adds the last
eight. Nothing in `functions.tsv` remains unspecced.

| address | TSV size | spec page | body at 09:25 |
|---|---:|---|---|
| `0x80A3DEA0` | 2940 | `SPEC_WAVE144_ENEMIES.md` | no |
| `0x80A3A4A0` | 2620 | `SPEC_WAVE140_ENEMIES.md` | no |
| `0x80B04E90` | 1592 | `SPEC_WAVE144_ENEMIES.md` | no |
| `0x80A7BEC0` | 1464 | `SPEC_WAVE144_ENEMIES.md` | no |
| `0x80A9B3C0` | 1196 | `SPEC_WAVE144_ENEMIES.md` | no |
| `0x80A92F40` | 1172 | `SPEC_WAVE144_ENEMIES.md` | no |
| `0x80A10880` | 940 | `SPEC_WAVE151_ENEMIES.md` | no |
| `0x80A22820` | 700 | `SPEC_WAVE145_ENEMIES.md` | no |
| `0x80A0F610` | 660 | `SPEC_WAVE146_ENEMIES.md` | yes |
| `0x80A85260` | 528 | `SPEC_WAVE146_ENEMIES.md` | yes |
| `0x80ACDB00` | 148 | `SPEC_WAVE154_ENEMIES.md` | yes |
| `0x80A03C30` | 92 | `SPEC_WAVE154_ENEMIES.md` | yes |
| `0x80AF3850` | 88 | `SPEC_WAVE154_ENEMIES.md` | yes |
| `0x80A25F00` | 48 | **this page** | yes |
| `0x80A2F9D0` | 48 | **this page** | yes |
| `0x80A2FA00` | 48 | **this page** | yes |
| `0x80A43EE0` | 48 | **this page** | yes |
| `0x80A43F10` | 48 | **this page** | yes |
| `0x80A48940` | 48 | `SPEC_WAVE154_ENEMIES.md` | yes |
| `0x80AF0370` | 44 | `SPEC_WAVE154_ENEMIES.md` | yes |
| `0x80A933E0` | 36 | `SPEC_WAVE154_ENEMIES.md` | yes |
| `0x80B14D80` | 28 | `SPEC_WAVE154_ENEMIES.md` | yes |
| `0x80A07C80` | 16 | **this page** | yes |
| `0x80AF3840` | 16 | **this page** | yes |
| `0x80A337B0` | 4 | **this page** | yes |

Read this table as the module's remaining *merge* backlog, not a spec gap. At
09:25 the eight largest rows are the only ones still without a body; the other
seventeen - including every row from this page - have bodies but no merged
ledger row. Because the intersection of "not in the ledger", "not in a heading"
and "no body" is empty, **this exhausts the module**: there is no thirteenth
address to carry into a wave 156 enemies spec lane.

## Corrections this page makes to earlier docs

1. `COVERAGE_WAVE144_ENEMIES.md` (and `SPEC_WAVE154_ENEMIES.md` after it) skip
   `0x80A07C80` because "the three staged arguments cannot be cross-checked
   against a callee contract". The callee is `0x80168180`, which has an export
   *and* a translated body (`src/resource/registry.rs`, ledger row
   `resource::registry,80168180`), and the module's own constant comment at
   `enemies.rs:13656` states the contract. The row is `ready`.
2. `SPEC_WAVE154_ENEMIES.md` rows 5 and 6 name `0x80B3C74C` "slot `+0x1C` of
   `0x80B3C730`" and `0x80B3C780` "slot 0 of the second table". The image shows
   two adjacent 11-word method runs (`0x80B3C728`, `0x80B3C75C`); the thunks sit
   at methods 9 and 10 of *both*, and the factory/kind/name descriptor belongs
   to the second. `0x80B3C730` is an interior word of the earlier run, not a
   table start.
3. `SPEC_WAVE154_ENEMIES.md` row 8 says `0x80A337B0`'s "target is unclaimed, so
   there is no body to type". `0x80A337C0` has been translated since wave 139
   (`ledger/functions.csv`), and `enemies.rs:176975` is the body.
4. `SPEC_WAVE154_ENEMIES.md` row 1 (`0x80A25F00`) identifies the class only as
   "slot `+8` of the class table `0x80B3ABE0`". The record starts at
   `0x80B3ABB8`; the word is method 10, the third of the `+0x0C`/`+0x18`/`+0x24`
   adjustor run, and the block's name pool is `daEnBlueCoin_c`.
5. `COVERAGE_WAVE136_ENEMIES.md` says `0x80AF3840` is "pure `halt_baddata`;
   nothing can be reconstructed without decoding the raw stream against an
   unrecoverable entry". The four words decode unambiguously and the callee is
   the translated `step_short_clamped_wrap`.

## Verification

* 8 addresses, all absent from `ledger/functions.csv`, from every `^#` heading
  in `docs/*.md` and from every `fn fn_<addr>` body at the opening snapshot;
  each has a `decompiled = true` TSV row and an existing export file at the
  cited path. All eight gained `fn fn_<addr>` bodies at 09:10:49 during this
  pass, which is reported above rather than hidden.
* TSV sizes sum to 276 bytes; reachable counts sum to 69 instructions.
* For every row the last instruction of the walk is exactly `entry + size - 4`
  (`0x80A25F2C`, `0x80A2F9FC`, `0x80A2FA2C`, `0x80A43F0C`, `0x80A43F3C`,
  `0x80A07C8C`, `0x80AF384C`, `0x80A337B0`); no path leaves the extent and no
  `bctr` appears in the set.
* No row contains a `psq_l`/`psq_st` word and none reads a quantised pair:
  every `GQR3 = 0x00050005` site in the module stays outside this page, so the
  GQR3 constant is not exercised here.
* The three class records were read word by word from the REL image; the
  constructor chain of `fn_80a43f40` was decoded at `0x80A43F6C`, `0x80A43F88`,
  `0x80A43F9C` and `0x80A44020` to confirm that `0x80B40708` is the installed
  table, and `fn_80a25f30` at `0x80A25F58`/`0x80A25F64` for `0x80B3AC00`.
* `0x802DCBFC`'s ten words were read from `work/dol/main.elf` at
  `0x802DCBFC..0x802DCC20`, not from the Ghidra export.
* No Rust file, ledger CSV or existing document was modified by this pass.
