# Wave 219, lane A - `0x802C0450..0x802C46F8` (`resource::archive`)

Ten bodies from the `0x802C0000` band of `src/resource/archive.rs`: the byte
colour blender, the three "wrap a stack material block and bind it" adapters
(plus the four-byte thunk beside one of them), the GX matrix/texobj applier, the
child-state walker of a panel list, the two deleting destructors of that panel
class, and the 2,424-byte two-mode panel builder that drives all of them.

All ten are `translated`; nothing is deferred.

Block file: `work/wave219/laneA_block.rs`
Ledger: `ledger/agents/wave219-laneA.csv`

## Bodies

| address | size (tsv) | real extent | name | confidence | role |
|---|---|---|---|---|---|
| `0x802C0450` | 392 | `0x802C0450..0x802C05D8` (392) | [`fn_802c0450`] | high | blend a four-byte colour by one factor |
| `0x802C0B60` | 60 | `0x802C0B60..0x802C0B9C` (60) + 4 pad | [`fn_802c0b60`] | high | build and bind the material block of a pane |
| `0x802C11E0` | 132 | `0x802C11E0..0x802C1264` (132) + 12 pad | [`fn_802c11e0`] | high | allocate the `0x20`-byte GIF buffer and bind it |
| `0x802C1270` | 4 | `0x802C1270..0x802C1274` (4) | [`fn_802c1270`] | high | tail-branch alias of `0x802C11E0` |
| `0x802C2300` | 88 | `0x802C2300..0x802C2358` (88) + 8 pad | [`fn_802c2300`] | high | build a window block from one colour word |
| `0x802C2F70` | 136 | `0x802C2F70..0x802C2FF8` (136) + 8 pad | [`fn_802c2f70`] | high | apply one matrix + one texobj pair |
| `0x802C32B0` | 432 | `0x802C32B0..0x802C3460` (432) | [`fn_802c32b0`] | high | walk and reset every child state of a panel list |
| `0x802C34B0` | 92 | `0x802C34B0..0x802C350C` (92) | [`fn_802c34b0`] | high | deleting destructor with the `+0x08` release |
| `0x802C35F0` | 64 | `0x802C35F0..0x802C3630` (64) | [`fn_802c35f0`] | high | deleting destructor without it |
| `0x802C3D80` | 2424 | `0x802C3D80..0x802C46F8` (2424) | [`fn_802c3d80`] | medium | build (mode `0`) / tear down (mode `2`) the panel set |

Every `functions.tsv` size was re-checked in `work/dol/main.elf` from the
prologue down to the closing `bclr`. In this lane **all ten tsv values are
exact**: `0x802C0450`'s `blr` sits at `start + 392 - 4`, and so on to
`0x802C3D80`'s at `0x802C46F4`. Where the next entry does not abut, the bytes
behind the `blr` are alignment zero words the tsv does not count (`+4` after
`0x802C0B60`, `+12` after `0x802C11E0` because `0x802C1270` is the four-byte
thunk, `+8` after `0x802C2300` and after `0x802C2F70`).

Two gaps belong to bodies that are **not** in this lane's list and not in
`functions.tsv` at all:

* `0x802C3460..0x802C3484` (36 bytes) - a byte-flag getter that dispatches
  vtable slot `+0x40`; and `0x802C3490..0x802C34B0` (32 bytes) - the same shape
  with slot `+0x44`. That is why `0x802C32B0` ends at `0x802C3460` and not at
  the next listed entry.
* `0x802C3510`/`0x802C3520` (eight-byte `addi r3,r3,-8` / `-0x14` thunks onto
  `0x802C34B0`) and `0x802C3530` (another destructor). `0x802C34B0` therefore
  ends exactly on its `blr` at `0x802C3508`.

## Byte weight

392 + 60 + 132 + 4 + 88 + 136 + 432 + 92 + 64 + 2424 = **3,824 bytes**.

## Evidence notes

### The `0x802DCD50..0x802DCD88` "returns"

`0x802C3D80` opens with `addi r11,r1,0x130` / `bl 0x802DCD54` and closes with
`bl 0x802DCDA0`. `0x802DCD54` is not a function: it is the middle of the
Metrowerks `_savegpr_19` block, whose body is `stw r19,-0x34(r11)` ...
`stw r31,-4(r11)` / `blr`, i.e. it spills `r19..r31` into the caller frame and
leaves `r3`/`r4` untouched. Ghidra renders the call as a 64-bit return, so the
export declares `void FUN_802c3d80(void)`; the disassembly shows `mr r21,r3` /
`mr r19,r4` and `cmpi cr0,r19,0` / `cmpi cr0,r19,2`, so the real signature is
one pointer plus one mode word. The matching `0x802DCDA0` tail is
`_restgpr_19`, so no call is modelled for either stub. The same reading is what
settles the argument roles of `0x802B0680` in wave 105, and it is applied here
to `0x802C3D80` only - the other nine bodies have ordinary prologues.

### The `0x43300000` pairs

`0x802C0450` and `0x802C3D80` both pack `0x43300000` into stack words, store an
integer in the following word (with `xoris ...,0x8000` for the signed forms) and
subtract `2^52` / `2^52 + 2^31` from the resulting double. That is the
compiler's `i32 -> f64` conversion, not real double arithmetic: `0x8042F218` is
`0x4330000000000000`, `0x8042F220` and `0x8042F380` are
`0x4330000080000000`, and `0x802C0450`'s `lfs f0`/`fcmpo` guards plus its
`fmuls` loads prove the working values are single precision. Both bodies are
written with plain `as f32` casts, as in the wave-105 `0x802B0680` note.
`0x802C0450`'s `fctiwz` + `stb` pair truncates toward zero and keeps the low
byte; the Rust `as i32 as u8` is that pair for every value the two guards let
through.

### `0x802C0450` - the two bounds are `0.0` and `1.0`, tested with `>`/`<`

The export prints `if (0.0 < param_1) { if (param_1 < 1.0) { blend } else
copy end } else copy start`, which is exactly what the disassembly does:
`fcmpo cr0,f1,f0` + `cror 2,0,2` (i.e. "not greater") + `bc 4,2` branches over
the copy when the factor is *above* the bound, once for `0xC190(r2)` (`0.0`) and
once for `0xC194(r2)` (`1.0`). A factor of exactly `0.0` therefore copies the
source colour and exactly `1.0` copies the target colour, byte for byte - the
blend loop is only reached inside the open interval.

### `0x802C11E0` / `0x802C1270` - a zero selection means "the SDA default"

`0x802C11E0` tests `r4` with `cmpi cr0,r4,0` and loads `-0x4870(r13)`
(`0x8042AE10`) on the zero path before creating the texture. The four-byte
`0x802C1270` is one `b 0x802C11E0` instruction (image word `4BFFFF70`, `AA = LK
= 0`), so it forwards both arguments unchanged and is modelled as a plain call;
Ghidra's export for it is byte-for-byte the `0x802C11E0` body because it
follows the branch.

### `0x802C2F70` - the handle table is eight bytes per entry

`lis r3,0x8042; addi r3,r3,0x45B8; rlwinm r0,r29,3,0,28; lwzx r3,r3,r0;
lwz r4,4(r3)` reads the pair at `0x804245B8 + index*8`, i.e. the two words that
`0x801C95E0` consumes. The matrix block is reset with `0` in both `0x801C9940`
and `0x801C99C0`, and the colour is handed to `0x801C69E0` as a four-byte
payload, which is why the third argument is a pointer to a single word.

### `0x802C32B0` - the count is re-read, the child is re-read per state

The count comes from vtable slot `+0x3C` of the object at `+0x04`, and the body
calls it **once per loop iteration** (`0x802C33BC` is the loop head, reached
from `0x802C32F8` before the first body pass). The child pointer is likewise
re-read from `[+0x08] + index*12 + 4` before each of the five `+0x20` calls,
because `0x802C7EF0` has just rebuilt the slots. The five states are pushed
enabled (`8`, `2`, `3`, `4`, `5`); the tail then clears state `5` of the first
child and programs `4`, `1`, `2`, `4` through `0x80257720`/`0x802576A0`/
`0x802576E0`. The `+0xD4` store writes `panel + 0x14`, or `0` for a null panel.

### `0x802C3D80` - what the mode ladder actually builds

Mode `0` pushes the three GX state words to `1`
(`0x802D2F20`/`2F80`/`2FE0`), then:

* the divisor is `2`, or `4` when bit `0x04` of `[owner+0x1C]` is set, and both
  the HUD record (`+0x08`/`+0x0C`) and the geometry record (`+0x08`/`+0x0C`)
  extents are halved with it - the HUD pair by integer `divw`, the geometry pair
  by `fdivs` and `fctiwz`;
* the panel kind at `[owner+0x20]` selects the layout: kind `0` stores `0` at
  `[[owner+0x84]+0x60]` and creates one pane of kind `3` at the *undivided*
  geometry extent with texture id `0x13`; kind `1` stores `2` there and creates
  one pane of kind `6` with texture id `0x16`, binds its material, then runs the
  whole `0x801C0C60` projection / `0x801C8920` ortho / fixed GX state ladder up
  to `0x802C2360(0x0B)`;
* both arms fall into the shared tail at `0x802C41FC`, which flushes through
  `0x801C76C0`, publishes the sprite at `[[owner+0x84]+4]` with `+8 = 1`, and
  then lays out the rest of the set: after destroying the published sprite it
  creates the kind-1 `0x28` pane, draws the animation strip through the first
  `0x802D0830` window block (slot `0` = the animation handle, colour
  `0xFFFFFFFF`), creates the kind-6 overlay pane stored at `[[owner+0x80]+4]`
  and draws it with the halved HUD extents, draws the same block again with slot
  `2`, the `0x28` pane as child and the `0xFFFFFF00`/`[owner+0x1F]` tint, and
  finishes with `0x802C4FB0` plus the two remaining `0x802D3370` destroys.

The two ratio terms used for the pane UV scale come from
`0x803504E0 + lane*12` (halfwords `[0]` and `[1]`, divided by the halfwords at
`r13 - 0x4748` / `r13 - 0x4746`). The lane index is chosen from the `+0x20` and
`+0x08` bits of `[hud+0x34]`, falling back to `*(r13 - 0x4778)`. The compiler
emits that three-instruction ladder twice (`0x802C424C` and `0x802C42B4`); the
Rust computes it once because nothing between the two sites touches
`hud+0x34`.

The three window draws save and restore the GX state triple around themselves
(`0x802C43B0`/`0x802C44E4`), and each draw selects a vtable slot: the pane
objects use `[[obj+0x28]+0x0C]` with `(this, f1, f2)`, which is why the block
returned by `0x802D0830` has a vtable pointer installed at `+0x28`.

The four `SDA` colour/size constants the extra panes store
(`0x8042F200` = `0x15000016`, `0x8042F204` = `0x0015`,
`0x8042F206` = `0x00`) and the two mask words (`0x8042F244` =
`0xFFFFFFFF`, `0x8042F370` = `0xFFFFFF00`) are read at run time exactly as the
original does, so the bodies do not embed them.

The body is marked `medium` only because of the naming: the offsets, mode
ladder, constants and callees are literal, while "panel owner", "pane" and
"cell" are `// inferred:` from the callee set (`0x802D3700` factory,
`0x802D3370` destructor, `0x802C02C0` texture stamp).

### New helpers added by this lane

`fn_802c0450`, `fn_802c3d80` and `fn_802c32b0` need call shapes the file did not
have yet, so the block adds six `_a`-suffixed helpers next to the existing
`call_double0`/`call_f64_3` family: `call_u32_0_a` (`() -> u32`),
`call_f32x3_ptr_a` (`0x801C0C60`), `call_f32x4_u32_ptr_a` (`0x801C8920`),
`vcall_f32_pair_a` (`+0x0C` of the `+0x28` sub-vtable), `vcall_count_a` (`+0x3C`
of the `+0x04` vtable) and `vcall_state_a` (`+0x20` of a child vtable). Every
new constant is `_A`-suffixed and the sixteen `0x801Cxxxx`/`0x802Cxxxx`
constants that earlier waves already define (`FN_801C8130`, `FN_801C86C0`,
`FN_801C88F0`, `FN_801C8DC0`, `FN_801C8E70`, `FN_801C8EB0`, `FN_801C69E0`,
`FN_801C82F0_105`, `FN_801C8330_105`, `FN_801C8370_105`, `FN_801C83D0_105`,
`FN_801C8700_105`, `FN_801C8790_105`, `FN_801C9940_105`, `FN_801C74C0_105`,
`HEAP_DELETE`) are reused rather than redefined, as are `SDA_BASE` and the
file-local `SDA2_BASE`.

## Verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to `%TEMP%\opencode\w219a`,
the block was appended to the copy's `src/resource/archive.rs` and
`cargo check --lib` finished with **exit code 0**. The warning count is
unchanged from the unmodified tree (26, all from `ui::menu` / `runtime::bt`
casts and parentheses); no warning comes from the appended block.

## Deferrals

None. All 606 instructions of `0x802C3D80` were decoded from
`work/dol/main.elf` with a local PowerPC pass (`b`/`bl` split on the link bit,
`li`/`addi` split on `rA == 0`, `rlwinm.` record form and `cror` fields checked
by hand), because the Ghidra export drops three unreachable blocks and mangles
the four `ldexpf`/GQR prologue fragments.
