# Wave 219 lane D coverage (`resource::archive`)

Slice: the ten `resource::archive` bodies at `0x802C0E30`, `0x802C1280`,
`0x802C1340`, `0x802C1370`, `0x802C17C0`, `0x802C20F0`, `0x802C22C0`,
`0x802C2700`, `0x802C3050` and `0x802C3210` (2480 bytes of `functions.tsv`
extent; 3952 bytes of real code because `0x802C2700` really runs to 2048
bytes). All ten are translated; nothing was deferred.

Bodies live in `work/wave219/laneD_block.rs` and are appended to
`src/resource/archive.rs` by the merge step; ledger rows are in
`ledger/agents/wave219-laneD.csv` (`module = resource::archive`).

This is the software-texture / hand-built-GX-stream corner of the
`0x802C0000` band. It sits directly under the texture-object helpers the
earlier archive waves translated: `0x802C0E30` turns one decoded source block
into a tiled texture through `0x802C1280`, `0x802C1340`/`0x802C1370` push raw
index bytes into the FIFO, `0x802C20F0`/`0x802C3050`/`0x802C3210` are GX stage
and render-state presets, and `0x802C2700` is the 18-way "draw then capture"
loop that fills the `0x804245B8` table.

## Translated

| Address | Bytes | Rust body | Summary |
| --- | --- | --- | --- |
| `0x802C0E30` | 936 | `fn_802c0e30` | Picks the `+4`/`+6` dimension order from the format (`0x73`/`0x53` use `+4` first), calls `0x802C05E0(src, work, first, row_begin, row_end, r8, r9)`, optionally exchanges the two halves of every row (`work[i < first/2 ? first/2 + i : i - first/2]`) and copies the staging array back, then scatters each row word through `fn_802c1280` over `second` columns in the `whole != 0 ? [0, first) : [row_begin, row_end)` window, and finishes with `0x802C0C20(tex)`. |
| `0x802C1280` | 184 | `fn_802c1280` | Leaf: format `6`/`0x16` writes four bytes at `data + ((x>>2) + (y>>2 & 0x3FFF) * (width>>2)) * 0x40 + (x&3)*2 + (y&3)*8` in the order `texel[3], texel[0], texel[1], texel[2]` (`+0`, `+1`, `+0x20`, `+0x21`); format `1`/`0x11` writes `texel[0]` at `data + ((x>>3) + (y>>2 & 0x3FFF) * (width>>3)) * 0x20 + (x&7) + (y&3)*8`; anything else returns. |
| `0x802C1340` | 40 | `fn_802c1340` | Leaf: eight `stb` to `0xCC008000` in the order `a, high, b, high, c, high, d, high`. Ghidra shows no parameter list because `r7` is never written; the call sites in `fn_802c2700` prove all five registers are live, and `high << 8 | value` is the halfword form. |
| `0x802C1370` | 124 | `fn_802c1370` | `0x801C5570(0xB0, 0, 5)` (`GX_LINESTRIP`) then ten `stb`: `a, high, b, high, c, high, d, high, a, high`. The repeated first index closes the strip. |
| `0x802C17C0` | 8 | `fn_802c17c0` | Leaf: `lwz r3,0xFFFFB834(r13)` - the shared word at `0x8042AEB4` that `fn_802c20f0` also forwards to `0x801C8790`. |
| `0x802C20F0` | 352 | `fn_802c20f0` | Two complete presets. `mode == 0`: `0x801C8130()`, `0x801C8790(tex,0xFF,0xFF,4)`, `0x801C8600(tex,0,0)`, `0x801C82F0(tex,0xF,0xF,0xF,10)`, `0x801C8370(tex,0,0,0,1,0)`, `0x801C8330(tex,7,7,7,5)`, `0x801C83D0(tex,0,0,0,1,0)`. `mode == 1` repeats the shape with `0x801C8790(tex, 0, *(0x8042AEB4), 4)`, `0x801C82F0(tex,0xF,8,10,0xF)` and `0x801C8330(tex,7,4,5,7)`. Any other mode is a no-op. |
| `0x802C22C0` | 64 | `fn_802c22c0` | `0x802C7B70(&texobj, descriptor)` fills a 36-byte stack texture object, then `0x801C74C0(&texobj, map)` binds it. |
| `0x802C2700` | 576 (real 2048) | `fn_802c2700` | Eighteen iterations: `DCInvalidateRange(buf,0x4000)`, `0x801C9470(buf,0x4000)`, one arm of the 18-entry jump table at `0x80350198`, then `0x801C9530()` (byte count) -> slot size at `0x804245BC + i*8`, `0x802B9270(size, heap, 0x20)` -> slot pointer at `0x804245B8 + i*8`, buffer copy and `DCStoreRange`. Sets bit 0 of `0x8042AEB0` afterwards. |
| `0x802C3050` | 108 | `fn_802c3050` | `0x802C3000(2)`; `0x801C5860(flag == 0 ? 2 : 1)`; `0x801C8DC0(*(0x8042AF4F) == 0 ? 1 : 3, 1, 1, 5)`. |
| `0x802C3210` | 88 | `fn_802c3210` | `0x802D1D70(this)` (writes `1` at `+0` and `&DAT_80350518` at `+4`), then `+8 = 0`, `+0xC (u16) = 0`, `+4 = 0x803501E0`, `+0x10 = 0x803501EC`, `+0x14 = 0x803501FC`; returns `this`. |

## Sizes and extents

Every `# Original` block keeps the canonical `main-gb` `functions.tsv` value.
Nine were re-checked against the prologue head and the closing `blr` in
`work/dol/main.elf` and are the real extent:

| address | tsv | first insn | closing `blr` | extent |
| --- | --- | --- | --- | --- |
| `0x802C0E30` | 936 | `stwu r1,-0x1440` | `0x802C11D4` | 936 |
| `0x802C1280` | 184 | `lbz r0,9(r3)` (leaf) | `0x802C1334` | 184 |
| `0x802C1340` | 40 | `lis r8,0xCC01` (leaf) | `0x802C1364` | 40 |
| `0x802C1370` | 124 | `stwu r1,-0x20` | `0x802C13E8` | 124 |
| `0x802C17C0` | 8 | `lwz r3,-0x47CC(r13)` (leaf) | `0x802C17C4` | 8 |
| `0x802C20F0` | 352 | `stwu r1,-0x10` | `0x802C224C` | 352 |
| `0x802C22C0` | 64 | `stwu r1,-0x30` | `0x802C22FC` | 64 |
| `0x802C3050` | 108 | `stwu r1,-0x10` | `0x802C30B8` | 108 |
| `0x802C3210` | 88 | `stwu r1,-0x10` | `0x802C3264` | 88 |

`0x802C2700` is the one outlier, and in the opposite direction from the usual
prologue artefact: the tsv says 576 (`0x240`), but the body jumps into an
18-case jump table at `0x80350198` whose arms run from `0x802C280C` to
`0x802C2DC8` and then fall into the shared capture tail at `0x802C2DCC`. The
closing `blr` is at `0x802C2EFC`, so the real extent is 2048 bytes
(`0x802C2700 .. 0x802C2F00`) and the next tsv entry is `0x802C2F00`. The tsv
value stops at the jump table because Ghidra treated the indirect jump as a
call and split the function there. The `# Original` line keeps 576 for the lane
checker; the prose records the 2048-byte extent.

## Notes on fidelity

* `fn_802c0e30`'s `r8`/`r9` (the sixth and seventh parameters) are never read
  in that body, but the compiler left them in place across the `0x802C05E0`
  call, and `0x802C05E0` *does* read them as its sixth and seventh arguments.
  They are therefore modelled as forwarded parameters rather than as dead
  arguments. The caller `0x802CB930` passes `r2 - 0x3E40` and `r2 - 0x3E3C`.
* The 0x1440-byte frame of `fn_802c0e30` is modelled as one `[u32; 0x510]`
  scratch array with the 0x400-byte staging array at `+0x10` and the 0x1030-byte
  working array at `+0x410`, so the write order and the unaligned offsets stay
  exactly as compiled instead of being replaced by slice bounds checks.
* `fn_802c2700`'s four `mtspr 914..917` writes program GQR2..GQR5
  (`0x00040004`, `0x00050005`, `0x00060006`, `0x00070007`). No `psq_l`/`psq_st`
  in this body or in its callees uses GQR2..GQR5, so they have no visible
  reader; they are reproduced verbatim. Ghidra's `unaff_GQR0`/`ldexpf`/
  `CONCAT13` prologue block is its misreading of the `psq_st` register saves and
  of the `0x43300000` integer-to-double slot, and the real body takes no float
  arguments at all.
* The integer-to-double slot at `0x8042F270` (`0x43300000_00000000`) is
  reproduced as a plain `as f32` conversion, which is what the magic-addend
  sequence computes.
* The four shape cases of `fn_802c2700` all read the same two bytes
  (`0x8042F308` = `0x10`, `0x8042F309` = `0x20`, i.e. counts 16 and 32). The
  compiler folded each `i - 5 / i - 7 / i - 9 / i - 11` index into a constant
  displacement off one base, so the pair arms always land on the same record.
* `fn_802c2700`'s jump-table guard (`i > 17` skips the switch) is unreachable
  because the loop's own bound is `i < 18`; the `_ => {}` arm records it.
