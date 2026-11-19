# Coverage - Wave 181 lane C (`game::bases`)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane C block) | `game::bases` | 8 | 724 | 5 | 3 |

Full lane from the sub-300-byte sweep of the band. Source of the bodies:
`work/wave181/laneC_block.rs`. Every address is the REL `d_basesNP`; all
addresses in the docs are `0x8060_0000`-based analysis addresses, not runtime
addresses.

## What this pass covers

- **`dCourseSelectMenu_c` state tails (3 bodies).** `0x8060DB40` sets `+0x272`
  and dispatches `0x8082_3810` (invalid choice) when the `-1` layout query
  reports `0`; `0x8060DC10` clears `+0x271`, clears bit 0 of the window pane's
  `+0xBB` byte and dispatches `0x8082_36D0` (`StateID_InitWait`); `0x8060DBB0`
  posts sound cue `0x7A`, stops the layout with `0x800C_9500` and starts
  animation `0x12`. All three sit in the block that already holds
  [`fn_8060d620`], [`fn_8060d740`], [`fn_8060d810`], [`fn_8060d890`],
  [`fn_8060d910`] and [`fn_8060d990`], and reuse their exact idiom for the
  `+0x208` interface slot `+0x18`.
- **Deleting destructor (1 body).** `0x8060F670` is instruction-for-instruction
  identical to the already-translated [`fn_806357e0`] except for the two `bl`
  displacements: `if this { if this+0x18 != 0 { 0x8015_F9F0(this+0x18, 0) }
  if free > 0 { 0x802B_9280(this) } } return this`. The REL keeps both entries
  because each class registers its own destructor.
- **`dEasyPairing_c` state tails (4 bodies).** `0x80613180` is the layout-done
  probe that clears the system "screen open" bit `0x40` of `+0x1D8` and returns
  a real bool; `0x806135F0` arms `+0x27A`/`+0x27B` and dispatches
  `0x8082_3DF0` (`StateID_keyWait`); `0x806138F0` dispatches `0x8082_3E70`
  (`StateID_StopEndWait`); `0x80613960` clears `+0x27A` and dispatches
  `0x8082_3EB0` (`StateID_ExitAnimeEndWait`) at state `3`. The three
  descriptors are the ones [`fn_80613ca0`] registers, and the fields match
  [`fn_80613380`] / [`fn_806131d0`].

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8060DB40` | `fn_8060db40` | 96 | high |
| `0x8060DBB0` | `fn_8060dbb0` | 84 | high |
| `0x8060DC10` | `fn_8060dc10` | 112 | high |
| `0x8060F670` | `fn_8060f670` | 96 | high |
| `0x80613180` | `fn_80613180` | 72 | high |
| `0x806135F0` | `fn_806135f0` | 96 | medium |
| `0x806138F0` | `fn_806138f0` | 80 | medium |
| `0x80613960` | `fn_80613960` | 88 | medium |

## Deferred

None. All eight bodies decode cleanly from the raw words.

## Corrections and notes

- **Branch polarity was re-derived from the encodings, not copied from Ghidra.**
  The three-way pattern here is `cmpi` + `bc 4,2` (`bne`) skipping the body, so
  `0x8060DB40` / `0x8060DC10` run on `== 0` and `0x80613180` runs on `!= 0`.
  `0x80613180`'s `4082000c` jumps *into* the body when the query is non-zero,
  which is why the two `li r3` returns are `0` (fall-through) and `1` (body).
  `0x8060F670`'s `bc 4,1` is `ble`, i.e. `if free > 0 { release }`; the raw
  words therefore agree with the Ghidra text and with [`fn_806357e0`].
- **`0x80613180` is not a byte-twin of [`fn_80613a30`].** Both clear bit `0x40`
  of the system record, but `0x80613180` queries `0x800C_9930` (layout-done)
  instead of `0x800C_9610` (animation id), stores nothing else and returns the
  query result. The clear mask is `rlwinm r0,r0,0,26,24`, i.e. `& 0xFFFF_FFBF`.
- **`0x801D_FAC0` / `0x801D_FAD0` / `0x801D_FB30` stay address-named.**
  `src/runtime/dvd.rs` reconstructs these four-byte main.dol thunks as WUD/disc
  queries (`0x801F3840` is `void`, `0x801F38F0` returns a completion flag,
  `0x801F3530` returns the byte at `0x803A3828`). That reading does not fit a
  pairing screen, and `0x801F3840` returning `void` makes the `!= 0` test at
  `0x806135F0` a raw post-call `r3` read. The Rust reproduces exactly the
  instruction stream (`call0_ret`), asserts nothing about the callee, and those
  three bodies are marked `medium` for that reason only.
- **Descriptor identity.** `0x8082_36D0` = `StateID_InitWait` and
  `0x8082_3810` = the invalid-choice/close descriptor (both from
  [`fn_8060de60`] and [`fn_8060d990`]); `0x8082_3DF0` = `StateID_keyWait`,
  `0x8082_3E70` = `StateID_StopEndWait`, `0x8082_3EB0` =
  `StateID_ExitAnimeEndWait` (all three from [`fn_80613ca0`]). The immediates
  are `addi r4,r4,14032` / `14352` / `15856` / `15984` / `16048` on
  `lis r4,0x8082`.
- **`0x8060DBB0` sound manager.** `lis r4,0x8043` + `lwz r3,-0x5B78(r4)` resolves
  to `0x8042_A488` (not `0x8043A488`), the same word every other sound post in
  this module reads.
- **`0x8060DC10` store order** is `+0x271`, then the pane bit, then the slot
  call, matching the raw `stb` / `rlwinm`+`stb` / `lwz r12,0x208` sequence.

## Verification

- Every body was checked against the raw words printed by
  `work/.../ppc159.js` (`$env:LEN='24'`-`'34'`), not only against the Ghidra
  export; all eight exports agree with the listings apart from the usual
  dropped `r3` arguments on the `0x801D_FACx` thunks and `0x802B_9280`.
- `0x8060F670` and `0x806357E0` were diffed word-by-word over their 96 bytes:
  only the `bl` displacement fields differ.
- The block was type-checked standalone against stub signatures for
  `platform::read_u8/write_u8/read_u32/write_u32/read_i32/write_i32`,
  `call0_ret`, `vmethod1` and `abi_call1..3` (`work/wave181/_syntax_check.rs`,
  `rustc --emit=metadata`, exit 0); the scratch files were removed afterwards.
- `src/game/bases.rs` was not modified; the coordinator's `merge.cjs` appends
  the block.
