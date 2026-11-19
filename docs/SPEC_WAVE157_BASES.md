# Wave 157 - `game::bases` - the next eight (560-544 byte band)

Read-only specification lane. No Rust file, no `ledger/functions.csv` row and no
existing document was modified by this lane. The only file written is this one.
No merge or check tool was run. The throwaway tools live under
`%TEMP%\opencode\w157` (`pick.js`, `mdump.js`, plus the wave-156 decoders
`d.js`/`md.js` under `%TEMP%\opencode\w156`); none of them is in the repository.
`d.js` decodes `work/rels/elf/d_basesNP.elf` through its own program headers,
`md.js` is the same decoder retargeted at `work/dol/main.elf` so every
`main.dol` callee below was disassembled at its real address instead of guessed
from the export.

Waves 128-156 walked this bucket from the top in size order; wave 156 ended at
560 bytes and named the next rows explicitly
(`docs/SPEC_WAVE156_BASES.md:216-222`): "the 560-byte group is
`{0x80628F70, 0x80763C20, 0x807A0090}` - so **the ninth is `0x80763C20` (560)**
and the tenth is `0x807A0090` (560)", "the 556-byte row is `0x8077C440` (556),
then `0x80741A70` (552), then the 548-byte group
`{0x807390D0, 0x80776440, 0x807B2550}`". This wave claims exactly those seven
plus the 544-byte row that follows them:

```
0x80763C20 560 | 0x807A0090 560 | 0x8077C440 556 | 0x80741A70 552
0x807390D0 548 | 0x80776440 548 | 0x807B2550 548 | 0x806D8B90 544
```

## The eight

| # | address | TSV size | words | export file | verdict |
|--:|---:|---:|---:|---|:--:|
| 1 | `0x80763C20` | 560 | 140 | `80763c20_FUN_80763c20.c` | ready (high) |
| 2 | `0x807A0090` | 560 | 140 | `807a0090_FUN_807a0090.c` | ready (medium) |
| 3 | `0x8077C440` | 556 | 139 | `8077c440_FUN_8077c440.c` | ready (medium) |
| 4 | `0x80741A70` | 552 | 138 | `80741a70_FUN_80741a70.c` | ready (high) |
| 5 | `0x807390D0` | 548 | 137 | `807390d0_FUN_807390d0.c` | ready (medium) |
| 6 | `0x80776440` | 548 | 137 | `80776440_FUN_80776440.c` | ready (medium) |
| 7 | `0x807B2550` | 548 | 137 | `807b2550_FUN_807b2550.c` | ready (medium) |
| 8 | `0x806D8B90` | 544 | 136 | `806d8b90_FUN_806d8b90.c` | ready (high) |

**Ready: 8. Conditional: 0.** 4,416 bytes / 1,104 words in total.

Every `size` above is the `size` column of
`work/ghidra/rel-d_basesNP/export/functions.tsv` verbatim, `decompiled` is
`true` for all eight and all eight export files exist on disk. All eight
extents end exactly on their own `blr` (`address + size - 4`), so no row needs
the `docs/REL_ELF_MAPPING.md:37-42` extent correction:

| # | `blr` | `address + size - 4` | span is exact | next row | gap |
|--:|---|---|:--:|---:|---:|
| 1 | `0x80763E4C` | `0x80763E4C` | yes | `0x80763E50` | 0 |
| 2 | `0x807A02BC` | `0x807A02BC` | yes | `0x807A02C0` | 0 |
| 3 | `0x8077C668` | `0x8077C668` | yes | `0x8077C670` | 4 |
| 4 | `0x80741C94` | `0x80741C94` | yes | `0x80741CA0` | 8 |
| 5 | `0x807392F0` | `0x807392F0` | yes | `0x80739300` | 12 |
| 6 | `0x80776660` | `0x80776660` | yes | `0x80776750` | 236 |
| 7 | `0x807B2770` | `0x807B2770` | yes | `0x807B27A0` | 44 |
| 8 | `0x806D8DAC` | `0x806D8DAC` | yes | `0x806D8DB0` | 0 |

Gap contents, read word by word:

* row 3: one zero word;
* row 4: two zero words, then `0x80741CA0` (4 bytes) is `b 0x80741A70` - the
  four-byte thunk wave 117 and wave 150 both recorded
  (`docs/COVERAGE_WAVE117_BASES.md:121`, `docs/COVERAGE_WAVE150_BASES.md:130`);
  it is a *tail branch into this row*, not a separate body;
* row 5: three zero words;
* row 6: three zero words, then `0x80776670` (128 bytes: the class registrar -
  it copies the float triple at `0x807CF494..0x807CF49C` into
  `0x80817398..0x808173A0`, stores `*(u32*)0x8042CF6C` at `0x8081738C`, and
  registers `0x80600160(0, 0x80776700, 0x80832910)`), then three zero words,
  `0x80776700` (36-byte class descriptor:
  `0x802DCA98(0x80817380, 0x806189A0, 36, 1)`), three zero words, and
  `0x80776720` (the factory: `0x801628C0(0x25C)` then `0x80776750`). The
  `0x25C` fixes the allocation size of row 6's object at **604 bytes** and
  `0x80776750` (the next TSV row, 116 bytes) is its constructor:
  `0x800F5FD0(this)`, vtable `+0x60 = 0x808174B8`, `0x80069020(this+0x184)`,
  `+0x1A0 = 0`, `0x80169CD0(this+0x1A4)`, and two 56-byte sub-objects at
  `+0x1E4` through `0x802DC9A0`. None of the gap bodies is part of row 6;
* row 7: three zero words, a one-word `blr` stub at `0x807B2780`, three zero
  words, a second `blr` stub at `0x807B2790`, three zero words, then the next
  row `0x807B27A0` (456 bytes, the mirror body wave 76 rejected);
* rows 1, 2 and 8: no gap at all; the next TSV row starts on the very next
  word.

The decoder reports **no unknown word** in the 1,104 words: zero unknown
opcodes, zero unhandled X-form, zero unhandled paired-single form, zero branch
leaving its own body, and every `bl` lands on a named callee outside the body.
`psq` appears in **one row only**:

| row | `psq` words |
|--:|---|
| 4 | `psq_st f31,88` / `f30,72` / `f29,56` and `psq_l f31,88` / `f30,72` / `f29,56`, all `qr0`, all `W = 0` |
| 1, 2, 3, 5, 6, 7, 8 | none |

Row 4's six `psq` words are the standard MWCC float save bracket
(`stfd`/`psq_st` pairs in the prologue, `psq_l`/`lfd` pairs in the epilogue):
the low half of each `f64` slot is saved with `stfd` and the second `f32` half
with `psq_st ... W = 0`. There is no data-side paired-single traffic anywhere
in the eight bodies, so the wave-138 `GQR3 = 0x00050005` pin is not needed by
anything in this set, and every `qr0` operand is type-0 `f32`.

## Method

**Address mapping.** Re-derived from the program headers of
`work/rels/elf/d_basesNP.elf` (`d.js ph`), not copied:

```
PT_LOAD  p_offset  p_vaddr     filesz    memsz
  0      0x0010F0  0x806000F0  0x1C6004  0x1C6004
  1      0x001C70F4 0x807C60F4 0x0004FC  0x0004FC
  2      0x1C75F0  0x807C65F0  0x00000C  0x00000C
  3      0x1C7600  0x807C6600  0x00A5FC  0x00A5FC
  4      0x1D1C00  0x807D0C00  0x051CB8  0x051CB8
  5      0x000000  0x80822900  0x000000  0x012484 (zero fill)
```

so `file_offset = 0x10F0 + (address - 0x806000F0) = (address - 0x80600000) +
0x1000` for every code and data address below `0x807C60F4`, and segments 1-4
cover the literal pools in `0x807Cxxxx`/`0x807Dxxxx`.
`0x80822900..0x80834D84` is this REL's BSS and reads as zero at file level;
`0x80830F50`, `0x80830FE8`, `0x80831068`, `0x808310A8`, `0x8082D928`,
`0x80833DE8` and `0x80815778` are all in it and are built by the class
registrars at run time.

`main.dol`-resident data and code are read from `work/dol/main.elf` through
**its own** program headers (`md.js ph`):

```
p_offset  p_vaddr     filesz    memsz
0x000200  0x80004000  0x0026C0  0x0026C0
0x002900  0x800066C0  0x000060  0x000060
0x002A00  0x80006720  0x000060  0x000060
0x002B00  0x80006780  0x2E7260  0x2E7260
0x2E9E00  0x802ED9E0  0x0002E0  0x0002E0
0x2EA100  0x802EDCC0  0x000020  0x000020
0x2EA200  0x802EDCE0  0x0106C0  0x0106C0
0x2FA900  0x802FE3A0  0x0532A0  0x0532A0
0x000000  0x80351680  0x000000  0x0D6000  (BSS)
0x34DC00  0x80427680  0x002540  0x002540
0x000000  0x80429BC0  0x000000  0x0014C0  (BSS)
0x350200  0x8042B080  0x004B60  0x004B60
0x000000  0x8042FBE0  0x000000  0x00005C  (BSS)
```

so `0x8031A834` (row 7's registry), `0x8031BEFC`/`0x8031BF20` (row 2's rail
name arrays), `0x80428AB8..0x80428B78` (the `"Fa00".."Fc06"` tags they point at)
and `0x80427F80..0x80428000` (row 7's `"kuribo"`/`"met"` name lists) are
`main.dol` **file** data, while `0x80354E10` (row 5's slot table), `0x80373D88`/
`0x80373DC8` (row 7's two name arrays), `0x80429E98` (row 7's group table),
`0x8042A038`, `0x8042A078`, `0x8042A18C`, `0x8042A1A0`, `0x8042A1AC`,
`0x8042A20C`, `0x8042A210`, `0x8042A24D`, `0x8042A284` and `0x8042CF6C` are
`main.dol` **BSS** and read as zero at file level. Row 6's constructor and
factory are in the gap after row 6, so its object size and vtable are proven,
not inferred.

**Candidate selection (recomputed).** `%TEMP%\opencode\w157\pick.js` re-reads
four sources from scratch:

* `work/ghidra/rel-d_basesNP/export/functions.tsv` - 5,615 data rows, all
  `decompiled = true`, and all 5,615 export files named in the `file` column
  exist on disk;
* `ledger/functions.csv` - 17,219 distinct `address` fields (case-insensitive);
* `src/game/bases.rs` - every `fn fn_<addr>` definition token (3,799);
* every `#{2,4} ` heading of **all** `docs/SPEC_WAVE*_BASES.md` files that
  existed before this one (302 headings, 293 distinct `0x` addresses). The task
  names `SPEC_WAVE12*_BASES.md`, of which waves 128 and 129 are the only
  matches; the superset is used so that no address already claimed by a
  wave-130..156 heading can be picked twice.

That leaves **1,786** rows with an export. The eight largest are the table
above. Ties are broken by **ascending address**; this is the only place the
ordering is not forced by size, and it is stated so wave 158 can reproduce it:

* the 560-byte group is `{0x80763C20, 0x807A0090}` - exactly the two rows
  wave 156 named as the ninth and tenth, so both are claimed here;
* the 556-byte row is `0x8077C440` alone;
* the 552-byte row is `0x80741A70` alone;
* the 548-byte group is `{0x807390D0, 0x80776440, 0x807B2550}` - exactly three
  rows;
* the 544-byte row is `0x806D8B90` alone, so **the ninth is `0x80731D10`
  (540)**, then `0x806D6950` (536), then `0x806E0110` (532), then
  `0x806D62C0` (528), then `0x807B7B40` (524), then the 520-byte group
  `{0x806F2100, 0x806F2710}` and `0x8061F2F0`/`0x806D5FB0`/`0x806D6680`/
  `0x80702FD0`/`0x807826B0`/`0x807B8CB0` (516).

None of the eight is referenced by a `fn fn_<addr>` body or by any spec
heading (`pick.js` re-counts all four sources independently at each run). The
only prose appearances are the previous lanes' skip lists and neighbour notes:

* `0x80763C20` - `docs/SPEC_WAVE156_BASES.md:216-222` (ninth/tenth note) only;
* `0x807A0090` - `docs/COVERAGE_WAVE73_BASES.md:135`,
  `docs/COVERAGE_WAVE99_BASES.md:130`, `docs/COVERAGE_WAVE127_BASES.md:100-104`
  (deferred for the stacked-float-locals reading, now resolved) and
  `docs/SPEC_WAVE156_BASES.md:216-222`;
* `0x8077C440` - `docs/COVERAGE_WAVE129_BASES.md:141`,
  `docs/SPEC_WAVE140_BASES.md:67,306,395` (phase-A routes `0x0A` and `0x30` of
  [`fn_8077b450`] call it) and `docs/SPEC_WAVE156_BASES.md:219`;
* `0x80741A70` - `docs/COVERAGE_WAVE64_BASES.md:106-113` (rejected for the
  paired-single bracket, now disproved), `docs/COVERAGE_WAVE117_BASES.md:121`
  (its thunk), `docs/COVERAGE_WAVE150_BASES.md:130` (same thunk) and
  `docs/SPEC_WAVE156_BASES.md:219`;
* `0x807390D0` - `docs/COVERAGE_WAVE101_BASES.md:126-131` (rejected for the
  `0x802DCD74` "frame pointer" reading and `countLeadingZeros`, now disproved)
  and `docs/SPEC_WAVE156_BASES.md:220`;
* `0x80776440` - `docs/SPEC_WAVE156_BASES.md:220` only;
* `0x807B2550` - `docs/COVERAGE_WAVE76_BASES.md:145-148` (rejected because "the
  table identity behind the `Pa3_rail` string compares cannot be verified",
  now resolved) and `docs/SPEC_WAVE156_BASES.md:220`;
* `0x806D8B90` - `docs/SPEC_WAVE156_BASES.md:1663` (the size table) only.

### Why all eight are ready

The verdict rule is wave 144's, unchanged:

* **ready** - every reachable instruction is decoded, every literal is read out
  of the image or out of `main.dol`, every callee is named by address (or by its
  ledger name), and the body can be written as code with no assumed calling
  convention beyond the standard PowerPC EABI. Data in `main.dol`
  (`0x800xxxxx`, `0x802xxxxx`, `0x803xxxxx`, `0x804xxxxx`), in this REL's data
  segments, or in this REL's BSS is an *input*, named by address.
* **conditional** - exactly one named input would still be missing.

Four rows deserve a paragraph each, because a reader may expect a condition:

* **Row 4** was rejected by wave 64 for the "outlined paired-single
  `GQR`/`ldexpf` conversion block (`unaff_GQR0`, `in_f29`/`in_f30`/`in_f31`)".
  The raw words show six `psq` instructions in the whole body, all `qr0`,
  all `W = 0`, all in the `stfd`/`psq_st` prologue and `psq_l`/`lfd` epilogue;
  the export's `unaff_GQR0`/`ldexpf`/`CONCAT22` block is the decompiler's model
  of that save bracket, not of the algorithm. What wave 64 called "a 16-step
  angle interpolation" is the 16-slot `s16` tape at `+0x7E0` driven by
  `0x8015F4F0`, and it is now decoded instruction by instruction.
* **Row 5** was rejected by wave 101 because "`0x807390D0` reads the caller's
  frame pointer through the `0x802DCD74`/`0x802DCDC0` save/restore pair and
  then treats the high half of the returned 64-bit value as the actor", and
  because it "additionally needs `countLeadingZeros` and the dead `unreachable`
  block at `0x80739384`". All three are artefacts: `0x807390E4` is
  `mr r28,r3` (the receiver is entry `r3`), `0x802DCD74` is the CodeWarrior
  `_savegpr_27` stub which leaves `r3` untouched, the `cntlzw` chain
  (`fcmpo` / `cror 2,1,2` / `mfcr` / `rlwinm ..,3,31,31` / `cntlzw` /
  `rlwinm ..,27,24,31`) is the standard boolean materialisation of
  `a >= b`, and the reachable extent is 137 contiguous words ending on its own
  `blr` at `0x807392F0` - there is no unreachable block inside it.
* **Row 2** was deferred by wave 127 because of "stacked `float` locals whose
  addresses are reused across three sub-blocks". The three sub-blocks use
  distinct, non-overlapping slots (`r1+80` and `r1+68` for the first, `r1+56`
  and `r1+44` for the second, `r1+8` for the third), and every one of those
  slots is an *out* parameter of a callee whose body was read
  (`0x800F8130`, `0x800F8050`, `vcall +0xAC`), so the live value at each call
  is pinned by the call, not by the C declaration order.
* **Row 7** was deferred by wave 76 because "the table identity behind the
  `Pa3_rail` string compares cannot be verified". It is verified now:
  `-0x7FCE57CC` is `0x8031A834`, a `main.dol` array of pointers to
  `NULL`-terminated `const char*` arrays (`entry 20 -> {"kuribo"}`, `entry 21 ->
  {"kuribo","wing"}`, `entry 24 -> {"met"}`; entries `0..19` are zero), and the
  four compared names are this REL's `"Pa3_rail"` (0x8081EE5C),
  `"Pa3_rail_white"` (0x8081EE68), `"Pa3_daishizen"` (0x8081EE78) and
  `"Pa3_MG_house_ami_rail"` (0x8081EE88), with `"rail"` at `0x8081EEA0`.

Rows 1, 3, 6 and 8 carried no conditional reading at all.

### Shared callee notes used by several rows

Everything below was read out of `work/dol/main.elf` with `md.js`, not inferred
from the Ghidra export.

* `0x800FD8A0(this)` = `0x800FAE40(r13[-21555], (this+4) & 0xFF)` - a per-player
  state query keyed by the actor's own low flag byte. `0x800FD820(this)` =
  `0x800FA9C0(same, (this+4) & 0xFF, ((this+4) >> 8) & 0xFF)` - the two-byte variant
  query (modes `0..8`). `0x800FD880(this)` = `0x800FAE10(...)`,
  `0x800FD960(this)` = `0x800FB1F0(...)`, `0x800FD980(this)` = `0x800FB240(...)`.
  All five are tail calls; the `r13` byte is the SDA cell at `0x8042A24D`
  (`0x8042F680 - 0x5433`).
* `0x800F2A00(id, parent, variant, pos, flag)` is a tail call to `0x8006CBA0`
  (the generic actor factory). It returns the new actor.
* `0x801016B0(base, id, arg3, arg4, arg5)` walks the four 16-byte slots at
  `base + 0x168 + i*0x10`; the first with a negative first word is handed to
  `0x80101690(slot, id, arg3, arg4)` and the call returns 1. **The fifth
  argument is not forwarded** - the body only reads `r4..r6`, so the `200` at
  every call site below is dead.
* `0x80101630(ctx)` = `0x801015A0(ctx)`'s record `+4`, or `-1`.
* `0x800F6410(actor, vec3)` is `fn_800f6410` (the world-map facing/placement
  writer); `0x800F69F0(x, y, z, owner, target, arg6, angle, vec)` is
  `fn_800f69f0`.
* `0x80100200(out, scene, node_id)` = `0x800F32F0(scene + 0x338C*0x3F08 +
  0x6C7F4, node_id)`, then copies the node's `+4` transform into `out`
  (three `f32`); `0x800F8130(out, actor)` is the same lookup for the actor's
  `+0x688` sub-object plus the `vcall(+0x60, +0xAC)` offset, and
  `0x800F8050(out, actor)` is the same lookup for the actor's `+0x184`
  sub-object. Both add the node transform and the offset with `ps_add` on
  `qr0` type-0 `f32` pairs.
* `0x800F71A0(sub, index)` stores `sub + 4 + index*0x14` at `sub+0x504`
  (selects a 20-byte element); `0x800F7270(sub, flag)` returns that element's
  `+0xC`/`+0x10` word and latches `sub+0x508`.
* `0x8006CD40(this)` = `0x8006CD00(this, this+0xE8)` = `pos += *(this+0xE8)`
  (three `f32` adds, `0x8006CD00..0x8006CD30`).
* `0x800DAED0(iface)` is the intrusive-list unlink paired with `0x800BFFC0`
  (`docs/COVERAGE_WAVE69_BASES.md:47`).
* `0x80101C60(str)` / `0x80101D40(str)` allocate `strlen(str)+1` bytes, copy the
  string, append the pointer to the array `0x80373D88` / `0x80373DC8` and bump
  the count at the SDA cell `0x8042A20C` / `0x8042A210` (`r13-0x5474` /
  `r13-0x5470`). They are name-array collectors, not printers.
* `0x800DF7D0(mgr, name, array, count, flag)` = `dres_mng_set_res_array`
  (`docs/COVERAGE_WAVE4_REGISTRY.md:89`): it calls
  `0x800DF080(mgr+4, array[i], name, 0, flag)` for `i < count`.
* `0x802E19F8` is `strncpy` with `0x20` as the bound (and zero fill);
  `0x802E1AB4` is `strcmp`.
* `0x80056CF0(actor, bit)` is the event-flag bit test (`fn_80056cf0`,
  `docs/COVERAGE_WAVE4_PLAYER.md:55`).
* `0x80160E80(from_vec3, to_vec3)` returns the `s16` yaw from `from` to `to`
  (`0x801611C0(dx, dz)`); `0x80160AE0(a, b, step, max, min)` moves the vector
  `a` toward `b` by a step bounded by the last two floats, snaps `a` to `b`
  when the remaining distance is already below `min`, and returns the
  **remaining distance** `sqrt(|a - b|^2)` through `0x802BE470` - not an angle.
* `0x802BE470(x)` is `sqrtf` (a `frsp` wrapper over the `f64` routine at
  `0x802E8620`).

---

## 1. `0x80763C20` - `cobGhost` model-load state/cue body

**Evidence.** `work/ghidra/rel-d_basesNP/export/80763c20_FUN_80763c20.c`,
560 bytes, 140 words, synthetic `0x80763C20`. Body `0x80763C20..0x80763E4C`
(one `blr`, last word). Frame: `stwu r1,-32(r1)`, `__savegprlr_27`
(`0x802DCD74`) / `__restgprlr_27` (`0x802DCDC0`). No `psq`, no jump table, no
indirect call except the animator slot.

**What it is.** The state-entry and cue body of the `cobGhost` course object,
run once from the model-load callback. The class is proven by its own
constructor and model loader in the same window: the constructor `0x80763740`
calls `0x800F5FD0` (the world-map/course base constructor), installs vtable
`+0x60 = 0x80815778`, stores `-1` at `+0x184`, builds `+0x18C` with
`0x80069020`, builds the animator `+0x1A8` with `0x8016A2F0` and **six**
56-byte clip elements at `+0x1B4` with `0x802DC9A0(..., 56, 6)`; the loader
`0x80763A30` opens `g3d/model.brres` (`0x80815750`) and binds the pane
`cobGhost` (`0x80815760`). This body therefore drives clip element 0 (`+0x1B4`)
and clip element 2 (`+0x1B4 + 2*0x38 = +0x224`), keeps the state word `+0x3D4`,
the "latched" byte `+0x3D0` and the height `+0x1DC`, and ends by spawning
effect `0x2B0` and scheduling a cue.

Its action dispatcher is the next-row `0x80763E50` (action ids `0x15..0x19`,
`0x5F`, plus a "finished" byte at `+0x139`). Its only caller is the 14-word
model-load callback at `0x807638F0` (in the gap, not a TSV row), which runs
`0x80763A30` (bind model) -> this body -> `0x80763B70`; the class destructor is
the gap body at `0x80763800`.

**Exact behaviour.**

1. Read `ctx = *(u32*)0x8042A1AC` and `player = *(u32*)0x8042A1A0`, then set
   `state = -1`, `flag = 0`, `mode = 0`.
2. `if 0x800FD8A0(this) == 0` goto step 6; otherwise `mode = 1` and
   `m = 0x800FD820(this)`:
   * `m` in `{2,3,4,5}` (the raw test is the unsigned `m - 2 <= 3`) or
     `m == 7`: start clip element 0 -
     `vcall(this+0x1A8, +0x18)(this+0x1A8, this+0x1B4)`, then
     `0x80168FE0(0.0f, this+0x1B4)`, then `state = 13`;
   * `m == 6`: `state = (0x800FD880(this) != 0) ? 14 : 15`, then
     `vcall(this+0x1A8, +0x18)(this+0x1A8, this+0x224)`,
     `0x801680E0(0.0f, this+0x224)`, `0x80168FE0(0.0f, this+0x224)`
     (note the `0x801680E0`-before-`0x80168FE0` order, the reverse of the
     other arms);
   * `(m == 0 || m == 8) && 0x800FD880(this) == 0`: set `flag = 1`, then
     `vcall(this+0x1A8, +0x18)(this+0x1A8, this+0x1B4)`, then
     `0x80168FE0(this+0x1DC - 1.0f, this+0x1B4)`;
   * anything else: nothing.
3. Common tail: if `0x800FD960(this) == 0 && 0x800FD980(this) == 0` then
   `mode = 0`.
4. `0x800F2A00(0x2B0, this, mode, this + 0xAC, 0)`.
5. If `state >= 0`: `0x801016B0(ctx, state, this, player, 200)`.
6. Return.

The `0x800FD8A0 == 0` arm (step 2's `goto 6`) is:

* if `0x800FD820(this) == 1`: `state = 14`, `0x80168520(this+0x1A8, 0)`,
  `vcall(this+0x1A8, +0x18)(this+0x1A8, this+0x224)`,
  `0x80168FE0(0.0f, this+0x224)`, `0x801680E0(0.0f, this+0x224)`, `mode = 2`;
* otherwise nothing: the body then jumps straight to step 4 with `mode = 0`
  and skips the `0x800FD960`/`0x800FD980` test.

**Signature.**

```rust
/// State-entry and cue body of the `cobGhost` course object, run from the
/// model-load callback.
///
/// # Original
/// - Ghidra: `FUN_80763c20` at `0x80763C20` (560 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/80763c20_FUN_80763c20.c`
///
/// # Arguments
/// * `this` - live `cobGhost` object; `+0x04` flag word (low byte = player
///   state key), `+0xAC..+0xB4` position, animator `+0x1A8`, clip elements
///   `+0x1B4` (0) and `+0x224` (2) of the six built by `0x80763740`,
///   height `+0x1DC`, state `+0x3D4`, latch `+0x3D0`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x3D4`, `+0x3D0`, starts one of the two clips and zeroes its
/// weight, spawns effect `0x2B0` with `mode` as the variant, and queues the
/// cue `state` for `200` frames (the fifth argument is not forwarded by
/// `0x801016B0`).
///
/// # Safety
/// `this` must be live with its animator and both clip elements initialised;
/// `0x8042A1AC`/`0x8042A1A0` must hold the current context and player.
///
/// # Confidence
/// high - every branch, state value and constant is literal; the class name
/// and the clip stride come from the constructor `0x80763740` and the loader
/// `0x80763A30` in the same window.
pub unsafe fn fn_80763c20(this: usize)
```

**Constants.** `0.0f` = `0x807CEEA0`, `1.0f` = `0x807CEEA4`; spawn id `0x2B0`;
cue delay `200`; states `-1`, `13`, `14`, `15`; modes `0`, `1`, `2`;
clip-element stride `0x38` (elements `+0x1B4 + i*0x38`, six of them); globals
`0x8042A1AC` (context) and `0x8042A1A0` (player); vtable `+0x60 = 0x80815778`,
animator `+0x1A8`.

**Verdict: ready (high).**

---

## 2. `0x807A0090` - world-map rail-vehicle position/aim update

**Evidence.** `work/ghidra/rel-d_basesNP/export/807a0090_FUN_807a0090.c`,
560 bytes, 140 words, synthetic `0x807A0090`. Body `0x807A0090..0x807A02BC`
(one `blr`, last word). Frame: `stwu r1,-112(r1)` plus one saved GPR (`r31`);
no `psq`, no jump table. Ghidra's `param_1` is the receiver (entry `r3`, kept
in `r31`); the export drops the second argument (`0x8081BE40`) of the
`+0x60`/`+0xC0` slot call, which the raw words carry in `r4`.

**What it is.** The position and aim-point update of the unnamed
world-map rail vehicle. Its sibling `0x807A02C0` (428 bytes, the next TSV row,
the "spawn probe" of wave 127) uses the same `+0x184` sub-object through
`0x800F8050` and the same `0x8081BE40` constant block. The only caller of row 2
is the gap body `0x8079FCE0..0x8079FE20` (not a TSV row), at `0x8079FD2C`.

The first step selects a rail-name array by `(this+4) & 0xF` and hands it to
`0x800F7560`: variant `0` uses `0x8031BEFC`, whose nine entries point at the
`main.dol` four-byte tags `"Fa00".."Fa08"` (`0x80428AB8..0x80428AF8`), and
variant `1` uses `0x8031BF20`, which starts at `"Fb00"` (`0x80428B00`). So the
object is rail-bound and the low nibble of `+0x04` picks the rail family.

**Exact behaviour.**

1. `rail = (*(u32*)(this+4)) & 0xF`. If `rail == 0`,
   `0x800F7560(this, 0x8031BEFC, -1, 0)`; else if `rail == 1`,
   `0x800F7560(this, 0x8031BF20, -1, 0)`; otherwise nothing.
2. `if 0x800FC140(0x8042A24D, 0) != 0` (the world-mode predicate):
   a. `0x800F8130(&v, this)` - the scene-node world position of `this` plus
      the `+0xAC` offset of the `+0x688` sub-object;
   b. `this+0xAC`, `+0xB0`, `+0xB4` = `v.x`, `v.y`, `v.z`;
   c. `vcall(this+0x60, +0xAC)(&d, this, *(u32*)(*(u32*)(this+0x688)+4))`;
   d. `this+0xAC -= d.x`, `+0xB0 -= d.y`, `+0xB4 -= d.z`;
   e. `this+0xAC += 0.0f`, `+0xB0 += 0.0f`, `+0xB4 += 0.0f`
      (`0x8081BEA4/A8/AC`, all zero).
3. `0x800F71A0(this+0x184, 0)`; `0x800F8050(&t, this)` - the scene-node world
   position of the `+0x184` sub-object plus its selected element offset;
   `e = 0x800F7270(this+0x184, 0)`;
   `vcall(this+0x60, +0xAC)(&d2, this, *(u32*)(e+4))`.
4. `a = t - d2` (component-wise); then
   `this+0x784 = a.x + 0.0f`, `this+0x788 = a.y + 0.0f`,
   `this+0x78C = a.z + 0.0f` (`0x8081BEF0/F4/F8`, all zero); the same triple is
   also written to the dead stack slot `r1+20..+28` (an inlined temporary; it
   is not an argument).
5. `r = vcall(this+0x60, +0xC0)(this, 0x8081BE40)`; `0x800F71A0(this+0x184, r)`.
6. `delta = (this+0x784..+0x78C) - (this+0xAC..+0xB4)`;
   `0x800F6410(this, &delta)`; `0x800F78E0(this)` (`*(u32*)(this+0x6C4) = 0`).

**Signature.**

```rust
/// Recomputes the rail vehicle's position and the aim point it steers at.
///
/// # Original
/// - Ghidra: `FUN_807a0090` at `0x807A0090` (560 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/807a0090_FUN_807a0090.c`
///
/// # Arguments
/// * `this` - the rail vehicle; `+0x04` flag word (low nibble = rail variant),
///   vtable `+0x60`, position `+0xAC..+0xB4`, motion sub-object `+0x184`
///   (element table `+0x504`, latch `+0x508`), scene binding `+0x688`,
///   aim point `+0x784..+0x78C`, scratch `+0x6C4`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Rebinds the rail name array, rewrites the position and the aim point, and
/// hands the new aim vector to `0x800F6410`.
///
/// # Safety
/// `this` must be live with both sub-objects initialised, and the world-mode
/// byte `0x8042A24D` must be initialised.
///
/// # Confidence
/// medium - every store, both rail arrays, the two `+0x60` slots and all six
/// zero constants are literal; the semantic role of the `0x8081BE40` block
/// passed to slot `+0xC0` is `// inferred:` (the callee is unclaimed).
pub unsafe fn fn_807a0090(this: usize)
```

**Constants.** `0x8031BEFC` = nine pointers to `"Fa00".."Fa08"`;
`0x8031BF20` = pointers starting at `"Fb00"`; zero triplets
`0x8081BEA4/A8/AC` and `0x8081BEF0/F4/F8`; block `0x8081BE40` (its `+0xB0..`
words are all zero, its earlier words are `5.0/1.5/1.1/0.5/180/0/1/0.1/150/...`);
world-mode byte `0x8042A24D`; vtable slots `+0xAC` and `+0xC0` on `+0x60`.

**Verdict: ready (medium).**

---

## 3. `0x8077C440` - world-map route actor: pick target and heading

**Evidence.** `work/ghidra/rel-d_basesNP/export/8077c440_FUN_8077c440.c`,
556 bytes, 139 words, synthetic `0x8077C440`. Body `0x8077C440..0x8077C668`
(one `blr`, last word), then one zero word of padding. Frame:
`stwu r1,-80(r1)` plus `__savegprlr_27` (`0x802DCD74`). No `psq`, no indirect
call except the two class slots. Ghidra loses the receiver to
`func_0x802dcd74()`; the raw words show `mr r31,r3` at `0x8077C468`.

**What it is.** The "arrive at a route target" body of the world-map route
actor whose per-frame dispatcher is `0x8077B450`
(`docs/SPEC_WAVE140_BASES.md:306,308`): phase A calls it from the `0x0A` block
(`0x8077B538`) and from the `0x30` block (`0x8077B7E8`). It writes the
player-count-derived index `+0x210`, snapshots the position, rebuilds the
target point `+0x1D8..+0x1E0` from the player, the scene node `+0x1F4` or the
player's carried node, sets the heading and finally chooses one of the four
direction states `2..5` (and `3` for the -z case) by comparing the target
against the position.

**Exact behaviour.**

1. `n1 = 0x80780A40(this)`; `n2 = 0x80780A40(this)` (the body really calls it
   twice; `0x80780A40` counts the live players, so a stateful callee is
   reproduced faithfully). `this+0x210 = 15*n2 + 4*(n1 - 1)`.
2. `0x8077E570(this)` (effect release, `0x8077E410(0.0f, this, 0, -1)` plus the
   `+0x1CC` sub-object's `+0x50 - 1.0f` and its `+0x84` slot).
3. `this+0x278 = this+0xAC`, `this+0x27C = this+0xB0`, `this+0x280 = this+0xB4`.
4. `this+0x10C = *(f32*)(*(u32*)(this+0x294) + 8)`;
   `this+0x114 = 0.0f`; `this+0x118 = 0.0f`.
5. `sel = 0x80101630(*(u32*)0x8042A1AC)`:
   * `sel == 0`: `this+0x1D8/1DC/1E0 = *(u32*)(player+0x264/0x268/0x26C)` and
     `this+0x274 = 0`;
   * `sel == 1 || sel == 3`: `0x80100200(&p, *(u32*)0x8042A18C, this+0x1F4)`,
     `0x80780BD0(&out, this, &p, 0)`,
     `this+0x1D8/1DC/1E0 = out`, `this+0x274 = 4`, and when `sel == 3` also
     `vcall(this+0x60, +0x68)(this)`;
   * `sel == 2`: `0x80100200(&p, world, *(u32*)(player+0x22C))`,
     `this+0x1D8/1DC/1E0 = p`, `this+0x274 = 6`;
   * `sel == 5`: `this+0x274 = 10`;
   * otherwise: nothing (the target keeps its previous value).
6. `h = 0x80160E80(this+0xAC, this+0x1D8)`; `this+0x108 = h` and
   `this+0x102 = h` (two halfwords).
7. `this+0x200 = 2`, then the direction tree:
   * `target.x > pos.x` -> `5`;
   * `target.x < pos.x` -> `4`;
   * equal x: `target.z > pos.z` -> `2` (also the default), `target.z < pos.z`
     -> `3`.
8. `this+0x18C = 1`.

**Signature.**

```rust
/// Chooses the route actor's next target and heading and sets its direction
/// state.
///
/// # Original
/// - Ghidra: `FUN_8077c440` at `0x8077C440` (556 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/8077c440_FUN_8077c440.c`
///
/// # Arguments
/// * `this` - world-map route actor; vtable `+0x60`, position `+0xAC..+0xB4`,
///   route model `+0x294`, player holder `+0x1CC`, scene node `+0x1F4`,
///   orientation bytes `+0x102`/`+0x108`, target `+0x1D8..+0x1E0`,
///   target kind `+0x274`, direction state `+0x200`, snapshots
///   `+0x278..+0x280`, flag `+0x18C`, table index `+0x210`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x210`, the three position snapshots, `+0x10C`, `+0x114`,
/// `+0x118`, the target triple, `+0x274`, the two halfwords at `+0x102`/
/// `+0x108`, `+0x200` and `+0x18C`, and may release the `+0x1CC` effect and
/// run the `+0x60` slot `+0x68`.
///
/// # Safety
/// `this` must be live; `0x8042A18C` (scene), `0x8042A1A0` (player) and
/// `0x8042A1AC` (context) must be initialised.
///
/// # Confidence
/// medium - all offsets, states and constants are literal and both `0x60`
/// slots are visible; `0x8077E570`, `0x80780BD0` and `0x80780A40` are named by
/// address only.
pub unsafe fn fn_8077c440(this: usize)
```

**Constants.** `0.0f` = `0x807CF698`; state values `0`, `2`, `3`, `4`, `5`,
`6`, `10`; `+0x210 = 15*n2 + 4*(n1-1)`; globals `0x8042A18C`, `0x8042A1A0`,
`0x8042A1AC`; vtable slots `+0x68` and `+0xAC` on `+0x60`.

**Verdict: ready (medium).**

---

## 4. `0x80741A70` - marker actor: 16-slot angle tape and history ring

**Evidence.** `work/ghidra/rel-d_basesNP/export/80741a70_FUN_80741a70.c`,
552 bytes, 138 words, synthetic `0x80741A70`. Body `0x80741A70..0x80741C94`
(one `blr`, last word), then two zero words and the `b 0x80741A70` thunk at
`0x80741CA0`. Frame: `stwu r1,-96(r1)` with `stfd f31/f30/f29` +
`psq_st` (`qr0`, `W = 0`) and `__savegprlr_26` (`0x802DCD70`). The only `psq`
words in the body are those three prologue saves and their three epilogue
restores; the export's `unaff_GQR0`/`ldexpf`/`CONCAT22` block at lines 39-94 is
the model of that bracket. Ghidra also reports three "removing unreachable
block" warnings at `0x80741C6C..0x80741C7C`: those are the epilogue `psq_l`
words, reached only through the `blr`-less `bdnz` chain, and the raw stream
reaches all 138 words.

**What it is.** The driver of the wave-64 marker actor's 16-slot
angle tape (`docs/COVERAGE_WAVE64_BASES.md:98-113`). The object is the one
whose layout is recorded at `src/game/bases.rs:906-910`: sixteen-word pointer
arrays at `+0x680` and `+0x714` with counts at `+0x7ED` and `+0x881`, plus the
sibling driver [`fn_80741510`]. This body walks the `+0x714` pointer array
(count at `+0x881`); each element is a pointer whose `+2` is a target `s16` and
whose `+0x14` is a due time. It steps the `s16` slots at `+0x7E0` toward those
targets with `0x8015F4F0(ptr, target, 0x200)`, appends each result to the
64-entry ring at `+0x800`, and finally replays older ring entries into the
remaining slots of the 16-slot window.

**Exact behaviour.**

1. The local `cur` is pre-loaded from `*(s16*)(this+0x4D2)`; it is what step 4
   pushes into the ring when no slot was emitted.
2. `acc = 0.0f` (`0x807CE08C`), `step = 8.0f` (`0x807CE0DC`), `count = 0`,
   `dst = this` (i.e. `this+0x7E0` is addressed as `dst` with `dst += 2` per
   emitted slot).
3. `for each rec in this+0x714 .. this+0x714 + 4*(*(u8*)(this+0x881))`:
   `target = *(s16*)(rec+2)`, `due = *(f32*)(rec+0x14)`;
   `while (due >= acc)`:
   * if `count == 16` break out of the record loop;
   * `cur = *(s16*)(this + 0x7E0 + 2*count)`;
   * `0x8015F4F0(&cur, target, 0x200)`;
   * `acc += step` (`fadds`);
   * `*(s16*)(this + 0x7E0 + 2*count) = cur`;
   * `count += 1`.
   Note that `acc` is **not** reset per record: it is a running tape clock, and
   the 16 limit is global to the call.
4. `ring = *(u8*)(this+0x880)`; `*(u8*)(this+0x880) = ring + 1`;
   `ring &= 0x3F`; `*(s16*)(this + 0x800 + 2*ring) = cur` (the last value).
5. If `count < 16`, replay `16 - count` halfwords from the ring into
   `this + 0x7E0 + 2*count`, starting at `idx = (ring - 3) & 0x3F` and
   subtracting 2 from `idx` per halfword (`(idx - 2) & 0x3F` before each
   store). The body is the compiler's 8-way unrolled copy
   (`(16-count) >> 3` blocks of eight plus `(16-count) & 7` singles).
   **As shipped, the source stride is 2 halfwords while the destination stride
   is 1**; the reconstruction must keep that, and the comment should say so.
6. Restore the three floats and return.

**Signature.**

```rust
/// Advances the marker actor's 16-slot angle tape and appends to its 64-entry
/// history ring.
///
/// # Original
/// - Ghidra: `FUN_80741a70` at `0x80741A70` (552 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/80741a70_FUN_80741a70.c`
///
/// # Arguments
/// * `this` - marker actor; `+0x4D2` current `s16`, record pointers
///   `+0x714` (count `+0x881`), 16-slot tape `+0x7E0`, ring index `+0x880`,
///   64-halfword ring `+0x800`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Steps up to sixteen tape slots toward their records' targets (0x200 per
/// step), advances the ring index and copies the remaining slots from the
/// ring.
///
/// # Safety
/// `this` must be live with the record array and the tape initialised; every
/// record pointer must be readable at `+2` and `+0x14`.
///
/// # Confidence
/// high - the loop bounds, the two floats, the ring mask and both strides are
/// literal, and the only `psq` words are the `f29`/`f30`/`f31` save bracket.
pub unsafe fn fn_80741a70(this: usize)
```

**Constants.** `0.0f` = `0x807CE08C`; `8.0f` = `0x807CE0DC`; step `0x200`;
slot count `16`; ring mask `0x3F`; tape base `+0x7E0`; ring base `+0x800`;
record array `+0x714`; record target `+2`; record due time `+0x14`.

**Verdict: ready (high).**

---

## 5. `0x807390D0` - course actor: character heading and mount transitions

**Evidence.** `work/ghidra/rel-d_basesNP/export/807390d0_FUN_807390d0.c`,
548 bytes, 137 words, synthetic `0x807390D0`. Body `0x807390D0..0x807392F0`
(one `blr`, last word), then three zero words. Frame: `stwu r1,-64(r1)` plus
`__savegprlr_27` (`0x802DCD74`). No `psq`. Ghidra loses the receiver to
`func_0x802dcd74()`; `0x807390E4` is `mr r28,r3`.

**What it is.** A state/update body of the unnamed `0x8073` course actor
(identity and layout in `docs/COVERAGE_WAVE101_BASES.md:38-52`: class vtable
`+0x60`, NW4R animator `+0x394`, motion object `+0x1EC`, state byte `+0x38C`,
character byte `+0x38D`, direction byte `+0x348`, ground word `+0xEC`, and the
child list at `+0x4B8`). It resolves the character object through `0x8005F900`,
composes the actor's yaw from a mirror table (`+/-1 * 0x3000`) and the
character's own angle, and then runs one of two mount/dismount transitions
depending on the low two bits of the mask at `+0x12C`.

`0x8005F900(i8)` = `0x80162D00(*(u32*)(0x80354E10 + i*4))`, which returns the
character record's `+8` field (or `0`); `0x80129910(character)` returns
`(i16)*(character+0x2534)` when `0x801298B0(character) != 0`, else `0`.

**Exact behaviour.**

1. `tgt = 0x8005F900((i8)*(u8*)(this+0x38D))`.
2. `tgt_dir = *(u8*)(tgt+0x348)`; `tbl = 0x807CDC00` (two `i32`: `-1`, `+1`);
   `a = (i16)0x80129910(tgt)`;
   `this+0x102 = (u16)(tbl[tgt_dir] * 0x3000 + a + (i16)*(s16*)(tgt+0x102))`.
3. `flags = *(u32*)(this+0x12C)`. If `flags & 1 != 0`:
   * `this+0x348 = *(u8*)(this+0x130)`;
   * `this+0x114 = -0.1875f` (`0x807CDBF8`);
   * if `flags & 2 != 0`: `vcall(this+0x394, +0x18)(this+0x394, 0x80830FE8)`;
   * else: `this+0x348 = *(u8*)(tgt+0x348) ^ 1` and
     `vcall(this+0x394, +0x18)(this+0x394, 0x808310A8)`;
   * return.
4. `v = { *(f32*)(tgt+0xC4), *(f32*)(tgt+0xC8), *(f32*)(tgt+0xCC) }`;
   `below_x = (*(f32*)(tgt+0xC4) < 0.0f)` (the `0x807CDBE4` constant is `0.0f`).
5. If `*(f32*)(tgt+0xC8) > 0.0f` (raw test: `fcmpo` then `ble`):
   * `this+0xEC = 3.0f` (`0x807CDC08`);
   * `r = 0x8006EF40(this+0x1EC, 0)`;
   * `this+0xEC = 0.0f` (the `0x807CDBE4` constant again);
   * if `r != 0`: `0x80738780(this)` and return (start clip `0x80831028`);
   * else `q = 0`; `r = 0x80738D40(this, &q)`; if `r != 0 && q != 0`:
     `0x80738780(this)` and return.
6. If `0x80056CF0(tgt, 0x46) != 0` return.
7. `this+0x175 = 9`.
8. `r = 0x8006FA40(this+0x1EC, &v)` (point-in-motion test at the character's
   position):
   * `r != 0`: `this+0x348 = below_x ^ 1` and
     `vcall(this+0x394, +0x18)(this+0x394, 0x80831068)`; return;
   * `r == 0`: `q = 0`; `r = 0x80738DB0(this, below_x as u32, &q)`; if
     `r != 0 && q != 0`: `this+0x348 = below_x ^ 1` and
     `vcall(this+0x394, +0x18)(this+0x394, 0x80831068)`.

**Signature.**

```rust
/// Drives the course actor's character heading and its mount/dismount
/// transitions.
///
/// # Original
/// - Ghidra: `FUN_807390d0` at `0x807390D0` (548 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/807390d0_FUN_807390d0.c`
///
/// # Arguments
/// * `this` - live actor; yaw `+0x102`, mask `+0x12C`, direction byte
///   `+0x348`, copy source `+0x130`, float `+0x114`, motion `+0x1EC`,
///   state `+0x175`, animator `+0x394`, character byte `+0x38D`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the yaw, the direction byte, `+0x114`, `+0xEC`, `+0x175` and may
/// start the clip `0x80831028` or dispatch one of the three state descriptors
/// `0x80830FE8` / `0x808310A8` / `0x80831068`.
///
/// # Safety
/// `this` must be live, `0x8005F900(this+0x38D)` must return a character
/// record, and the `0x80830F50` descriptor block must be built.
///
/// # Confidence
/// medium - every offset, both comparisons, the `+/-1` table, the three
/// constants and the four call sites are literal; `0x80738780`,
/// `0x80738D40` and `0x80738DB0` are named by address, and the descriptor
/// objects at `0x80830FE8`/`0x80831068`/`0x808310A8` are REL BSS.
pub unsafe fn fn_807390d0(this: usize)
```

**Constants.** `0x807CDC00` = `{-1, 1}` (`i32`); `0x3000` per table unit;
`0.0f` = `0x807CDBE4`; `-0.1875f` = `0x807CDBF8`; `3.0f` = `0x807CDC08`;
state byte `9`; event id `0x46`; descriptor base `0x80830F50` with
`+0x98 = 0x80830FE8`, `+0x118 = 0x80831068`, `+0x158 = 0x808310A8`
(the three `addi r4,r30,152/280/344` forms); clip name `0x80831028`;
slot table `0x80354E10`.

**Verdict: ready (medium).**

---

## 6. `0x80776440` - world-map actor: 60-frame target/idle handler

**Evidence.** `work/ghidra/rel-d_basesNP/export/80776440_FUN_80776440.c`,
548 bytes, 137 words, synthetic `0x80776440`. Body `0x80776440..0x80776660`
(one `blr`, last word). Frame: `stwu r1,-48(r1)` with four saved GPRs; no
`psq`, no indirect call except the `+0x60` slot `+0x68`.

**What it is.** The action handler of the 604-byte world-map actor built by the
factory `0x80776720` and the constructor `0x80776750` (both in the gap after
this row: vtable `+0x60 = 0x808174B8`, resource holder `+0x184`, sub-object
`+0x1A4`, two 56-byte sub-objects at `+0x1E4`). It is the
`(this, action, flag)` shape of the same family as `cobGhost`'s dispatcher:
`action == -1` is the "no transition" early-out, `31` and `32` are the two real
actions, anything else just sets the "finished" byte `+0x139`.

Action `31` starts a 60-frame approach: it copies the player's position
(`+0xAC..+0xB4` of `0x8042A1A0`) with `+50.0f` on y, sets the heading to
`0xC000` (`-90.0` degrees), asks `0x80772BD0(world, 1, -1)` for a node id and
stores the node's position at `+0x1B8..+0x1C0` with `+10.0f` on y.
Action `32` publishes `*this` into the shared world record
`*(u32*)0x8042A284` and then runs a two-phase timer: phase `0` steers the
position toward the node with `0x80160AE0` (which returns the remaining
distance) and arms the 60-frame countdown as soon as that distance exceeds
`1.1920929e-7f`; phase `1` counts the 60 frames down.

**Exact behaviour.**

1. `if action == -1 return`.
2. If `flag != 0`:
   * `action == 31`:
     `this+0x1AC = 1`;
     `this+0xAC = player+0xAC`, `this+0xB0 = player+0xB0 + 50.0f`,
     `this+0xB4 = player+0xB4`;
     `*(u16*)(this+0x102) = 0xC000`;
     `node = 0x80772BD0(world, 1, -1)`; `this+0x1B4 = node`;
     `0x80100200(&p, world, node)`;
     `this+0x1B8 = p.x`, `this+0x1BC = p.y + 10.0f`, `this+0x1C0 = p.z`;
     `this+0x1C4 = 60`;
   * `action == 32`:
     `*(u32*)(*(u32*)0x8042A284 + 0x604) = 1`,
     `+0x5F4 = 0`, `+0x5F0 = *(u32*)this`, `+0x624 = 0`, `+0x608 = 0`;
     `this+0x1B0 = 0`.
3. `action == 31`: if `this+0x1C4 > 0` decrement it; if it is now `0`,
   `this+0x139 = 1`.
4. `action == 32`:
   * `this+0x1B0 == 0`:
     `d = 0x80160AE0(this+0xAC, &{this+0x1B8, this+0x1BC, this+0x1C0},
     0.008f, 10.0f, 2.0f)` - it steers the position toward the node and
     returns the remaining distance; if `|d| > 1.1920929e-7f` then
     `this+0x1C4 = 60` and `this+0x1B0 = 1` (the `fcmpo`/`cror 2,0,2`/`beq`
     sequence returns *early* when the distance is at or below the epsilon);
   * `this+0x1B0 == 1`: if `this+0x1C4 > 0` decrement it; if it is now `0`,
     `this+0x139 = 1`;
   * otherwise: nothing.
5. Any other action: `this+0x139 = 1`.

**Signature.**

```rust
/// Runs the world-map actor's `31`/`32` action: a 60-frame approach to a scene
/// node followed by an idle countdown.
///
/// # Original
/// - Ghidra: `FUN_80776440` at `0x80776440` (548 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/80776440_FUN_80776440.c`
///
/// # Arguments
/// * `this` - the 604-byte actor; `+0x102` heading, `+0x139` finished byte,
///   `+0x1AC` active byte, `+0x1B0` phase, `+0x1B4` node id, `+0x1B8..+0x1C0`
///   node position, `+0x1C4` timer, `+0xAC..+0xB4` position.
/// * `action` - `31`, `32`, any other value, or `-1` for "no transition".
/// * `flag` - non-zero runs the action's entry block.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the position, heading, node id/position and timer, publishes
/// `*this` into `*(u32*)0x8042A284 + 0x5F0`, and may set `+0x139`.
///
/// # Safety
/// `this` must be live, and `0x8042A18C`/`0x8042A1A0`/`0x8042A284` must be
/// initialised.
///
/// # Confidence
/// medium - all offsets, constants and the four call sites are literal;
/// `0x80772BD0` (the node lookup) and `0x80160AE0` (steer-toward, returns the
/// remaining distance) are named by address, and the action-id meanings are
/// `// inferred:` from their blocks.
pub unsafe fn fn_80776440(this: usize, action: i32, flag: i32)
```

**Constants.** `50.0f` = `0x807CF480`; `10.0f` = `0x807CF484`; `0.008f` =
`0x807CF488`; `2.0f` = `0x807CF48C`; `1.1920929e-7f` = `0x807CF490`;
`0.0f` = `0x807CF698`; heading `0xC000`; timer `60`; actions `31`, `32`, `-1`;
globals `0x8042A18C`, `0x8042A1A0`, `0x8042A284` (+`0x5F0`, `+0x5F4`,
`+0x604`, `+0x608`, `+0x624`).

**Verdict: ready (medium).**

---

## 7. `0x807B2550` - archive-name collector and two-group request

**Evidence.** `work/ghidra/rel-d_basesNP/export/807b2550_FUN_807b2550.c`,
548 bytes, 137 words, synthetic `0x807B2550`. Body `0x807B2550..0x807B2770`
(one `blr`, last word), then eleven words and the next row. Frame:
`stwu r1,-96(r1)` with 88 bytes of locals and `__savegprlr_22`
(`0x802DCD5C`). No `psq`.

**What it is.** The resource-name collector that runs before the world-map
archive groups are opened. It walks the four 0x3B0-byte group records at
`0x80429E98 + i*0x3B0`, and for each live group (`+0x2C != 0`):

* appends the group's four 32-byte names at `*(entry+8) + 0x00/0x20/0x40/0x60`
  to the `Stage/Texture` array (`0x80373D88`, count `0x8042A20C`) through
  `0x80101C60`;
* if the fourth name (`+0x60`) equals `"Pa3_rail"`, `"Pa3_rail_white"`,
  `"Pa3_daishizen"` or `"Pa3_MG_house_ami_rail"`, appends `"rail"` to the
  `Object` array (`0x80373DC8`, count `0x8042A210`) through `0x80101D40`;
* walks the `u16` list at `*(u32*)(entry+0x28)` (stride 4 bytes, `+0x98`
  entries) and, for each id, appends every string of the registry list
  `0x8031A834[id]` (a `NULL`-terminated `const char*` array such as
  `{"kuribo","wing"}`, `{"met"}`) to the `Object` array.

It then asks `dres_mng` for the two groups and dispatches the name object
`0x80833DE8` on the `+0x74` interface.

**Exact behaviour.**

1. `mgr = *(u32*)0x8042A038`.
2. `for i in 0..4`: `entry = *(u32*)0x80429E98 + i*0x3B0` (the base word is
   re-read on every iteration);
   if `*(u32*)(entry+0x2C) == 0` continue;
   * `names = *(u32*)(entry+8)`;
   * for `off in [0x00, 0x20, 0x40, 0x60]`: if `*(u8*)(names+off) != 0`
     then `0x80101C60(strncpy(&buf, names+off, 0x20))`;
   * `s = names + 0x60`; if `strcmp(s, 0x8081EE5C) == 0 ||
     strcmp(s, 0x8081EE68) == 0 || strcmp(s, 0x8081EE78) == 0 ||
     strcmp(s, 0x8081EE88) == 0` then `0x80101D40(0x8081EEA0)`;
   * `n = *(u32*)(entry+0x98)`; `list = *(u32*)(entry+0x28)`;
     for `k in 0..n`: `p = *(u32*)(0x8031A834 + (u16)*(u16*)(list + k*4)*4)`;
     if `p != 0` then for each `q` in the `NULL`-terminated `u32` array at `p`:
     `0x80101D40(*q)`.
3. `0x800DF7D0(mgr, 0x8081EEA8 /* "Stage/Texture" */, 0x80373D88,
   *(u32*)0x8042A20C, 0)`.
4. `0x800DF7D0(mgr, 0x8081EE38 /* "Object" */, 0x80373DC8,
   *(u32*)0x8042A210, 0)`.
5. `vcall(this+0x74, +0x18)(this+0x74, 0x80833DE8)`.

**Signature.**

```rust
/// Collects the archive names of the four live resource groups and requests
/// the `Stage/Texture` and `Object` groups.
///
/// # Original
/// - Ghidra: `FUN_807b2550` at `0x807B2550` (548 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/807b2550_FUN_807b2550.c`
///
/// # Arguments
/// * `this` - object whose `+0x74` interface takes the final name object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Appends up to `4 * (4 + 1 + list)` strings to the two name arrays, then
/// issues the two `0x800DF7D0` requests and one `+0x74` slot call.
///
/// # Safety
/// The group table `0x80429E98`, the name arrays `0x80373D88`/`0x80373DC8`,
/// their count cells and the registry `0x8031A834` must be initialised; every
/// live group record must have a readable name block and `u16` list.
///
/// # Confidence
/// medium - all four strings, the four `Pa3_` compares, the two requests, the
/// registry walk and every constant are literal; the *contents* of the
/// `0x8031A834` registry and of the group table are runtime data and stay
/// address-named.
pub unsafe fn fn_807b2550(this: usize)
```

**Constants.** `0x80429E98` = four group records, stride `0x3B0`, live flag
`+0x2C`, name pointer `+8`, `u16` list `+0x28`, count `+0x98`; name arrays
`0x80373D88` (`Stage/Texture`, count `0x8042A20C`) and `0x80373DC8`
(`Object`, count `0x8042A210`); registry `0x8031A834`; strings `"Stage/Texture"`
`0x8081EEA8`, `"Object"` `0x8081EE38`, `"Pa3_rail"` `0x8081EE5C`,
`"Pa3_rail_white"` `0x8081EE68`, `"Pa3_daishizen"` `0x8081EE78`,
`"Pa3_MG_house_ami_rail"` `0x8081EE88`, `"rail"` `0x8081EEA0`; name object
`0x80833DE8`; `0x20`-byte name slots; four groups.

**Verdict: ready (medium).**

---

## 8. `0x806D8B90` - rider-carrying course actor: clamp and release

**Evidence.** `work/ghidra/rel-d_basesNP/export/806d8b90_FUN_806d8b90.c`,
544 bytes, 136 words, synthetic `0x806D8B90`. Body `0x806D8B90..0x806D8DAC`
(one `blr`, last word), and the next TSV row starts on the very next word. Frame:
`stwu r1,-16(r1)` with one saved GPR; no `psq`.

**What it is.** The clamp body of the `0x806D8xxx` rider-carrying
course actor. Its twin `0x806D8A40` (196 bytes, immediately before it) has the
same `+0x588` release block and the same `+0x394` slot `+0x18` with
`0x8082D928`, but calls `0x806D86E0` (the rider scan) instead of this clamp;
`0x806D86E0` reads the rider list at `+0x518`, so the two together are "clamp
the platform, then report the riders".

The clamp keeps the actor inside a box of half-width `+0x584` around the centre
`(+0x574, +0x578)` and latches `+0x58C` whenever a clamp fires; the mode field
`(flags >> 16) & 7` (clamped to `4`) selects the edge, and the outer gate is
`(flags >> 20) & 0xF != 0`.

**Exact behaviour.**

1. `0x8006CD40(this)` (`pos += *(this+0xE8)`).
2. `flags = *(u32*)(this+4)`. If `((flags >> 20) & 0xF) != 0`:
   `mode = (flags >> 16) & 7`; if `mode > 4` then `mode = 4`; then
   * `0`: `v = this+0x578 + this+0x584`; if `pos.y >= v` then
     `pos.y = v`, `this+0x58C = 1`;
   * `1`: `v = this+0x578 - this+0x584`; if `pos.y <= v` then
     `pos.y = v`, `this+0x58C = 1`;
   * `2`: `v = this+0x574 + this+0x584`; if `pos.x >= v` then
     `pos.x = v`, `this+0x58C = 1`;
   * `3`: `v = this+0x574 - this+0x584`; if `pos.x <= v` then
     `pos.x = v`, `this+0x58C = 1`;
   * `4`: `hits = 0`; `v = this+0x574 + this+0x584`;
     if `pos.x >= v` then `pos.x = v`, `hits += 1`;
     `v = this+0x578 - this+0x584`;
     if `pos.y <= v` then `pos.y = v`, `hits += 1`;
     if `hits == 2` then `this+0x58C = 1`.
   (`>=` is the `cror 2,1,2` form and `<=` the `cror 2,0,2` form; the `bne`
   after each sets the branch sense correctly.)
3. `0x806D8860(this)` - the rider pass (see below).
4. If `*(u32*)(this+0x588) != 0` and
   `(*(u32*)0x8042A078 & *(u32*)(this+0x358)) == 0` and
   `(*(u32*)(*(u32*)0x8042A078 + 4) & *(u32*)(this+0x35C)) == 0`:
   `0x800DAED0(this+0x508)` and `vcall(this+0x394, +0x18)(this+0x394,
   0x8082D928)`.
5. `vcall(this+0x508, +0xC)(this+0x508)`.
6. If `((*(u32*)(this+4) >> 12) & 0xF) == 2`:
   `vcall(this+0x60, +0xE0)(this+0xAC, this+0x580, this)` (two `f32` arguments
   in `f1`/`f2` and the receiver in `r3`).

`0x806D8860(this)` (152 bytes, read in full):

```text
result = 0
for node = *(u32*)(this+0x518); node != 0; node = *(u32*)(node+0xC):
    rider = *(u32*)(node+4)
    if rider == 0: continue
    state = *(u8*)(rider+0x38C)
    if state == 1 || state == 2:
        if *(u32*)(this+0x58C) == 0:
            r = 0x80056370(rider, this, 1); result = 1
        // else: nothing for this rider
    else:
        *(u8*)(rider+0x34A) |= 4
return result
```

**Signature.**

```rust
/// Clamps the rider-carrying actor to its box, reports the riders and releases
/// the `+0x508` interface when it is empty.
///
/// # Original
/// - Ghidra: `FUN_806d8b90` at `0x806D8B90` (544 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/806d8b90_FUN_806d8b90.c`
///
/// # Arguments
/// * `this` - live actor; `+0x04` flag word (mode `>>16 & 7`, gate `>>20 & 0xF`),
///   position `+0xAC`/`+0xB0`, centre `+0x574`/`+0x578`, half-width `+0x584`,
///   radius `+0x580`, occupancy `+0x588`, clamp latch `+0x58C`, masks
///   `+0x358`/`+0x35C`, rider list `+0x518`, animator `+0x394`, interface
///   `+0x508`, vtable `+0x60`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Moves the position, sets the latch, attaches marked riders through
/// `0x80056370`, sets bit `4` of a rider's `+0x34A`, may unlink and dispatch
/// `+0x508`, and calls vtable slot `+0xE0` in mode `2`.
///
/// # Safety
/// `this` must be live with both interfaces initialised; `0x8042A078` must
/// hold the two mask words.
///
/// # Confidence
/// high - every branch, bound, mask, latch and call is literal, including the
/// full `0x806D8860` rider pass.
pub unsafe fn fn_806d8b90(this: usize)
```

**Constants.** mode field `(flags >> 16) & 7`, gate `(flags >> 20) & 0xF`,
final mode test `(flags >> 12) & 0xF == 2`; latch `1`; rider state bytes `1`
and `2`; rider flag bit `4` (`0x34A`); mask cells `0x8042A078` (two words);
`+0x508` slots `+0x0C` (dispatch) and `0x800DAED0` (unlink); animator name
`0x8082D928`; vtable slot `+0xE0`.

**Verdict: ready (high).**

---

## Verification

A second, independent pass (`%TEMP%\opencode\w157\pick.js` plus a fresh `walk`
over each address, and `mdump.js` over the `main.dol` callees) re-checked, for
all eight:

* the address is absent from `ledger/functions.csv`, from every
  `fn fn_<addr>` in `src/game/bases.rs`, and from every `##`-level heading in
  `docs/SPEC_WAVE*_BASES.md`;
* the TSV row exists, `decompiled = true`, the export file exists, and
  `size / 4` equals the number of reachable words;
* the last reachable word is `blr` at `address + size - 4`;
* the walk reports zero unknown words and every `bl` target is a `main.dol`
  address or a `d_basesNP` address outside the body;
* the only `psq` words in the eight bodies are row 4's six `qr0`/`W = 0`
  prologue/epilogue saves.

Totals: **8 bodies, 4,416 bytes, 1,104 words, 8 ready / 0 conditional**
(3 high, 5 medium).
