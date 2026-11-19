# Coverage - Wave 182, lane A (`game::bases`, sub-300-byte sweep)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 10 | 1,084 | 10 | 0 |

All ten lane-A addresses are 100-124 bytes in
`work/ghidra/rel-d_basesNP/export/functions.tsv`. Lane A added no helpers and
did not touch `src/game/bases.rs`; the block is in
`work/wave182/laneA_block.rs`.

## Translated

- `0x8062_49B0` (112 bytes) - `dModelPlayArrow_c` animation gate: when the
  `0x800C9610(+0xAC, 4)` and `(+0xAC, 5)` probes are both idle, dispatch
  descriptor `0x8082_5748` (`StateID_ActionWait`) through the `+0x18` virtual
  of `+0x70`.
- `0x8062_4B60` (112 bytes) - same gate with ids `6`/`7` and descriptor
  `0x8082_5788` (`StateID_OnStageAnimeEndWait`).
- `0x8062_4C80` (112 bytes) - same gate with ids `6`/`7` and descriptor
  `0x8082_5708` (`StateID_StartWait`).
- `0x8062_69A0` (100 bytes) - fourth `+0x18`-teardown deleting destructor of
  the `0x80626` screen cluster: `0x8015F9F0(this+0x18, 0)`, then
  `0x802B9280(this)` when `free > 0`, returning `this`.
- `0x8062_7E10` (100 bytes) - byte-identical destructor twin next to
  [`fn_80627db0`].
- `0x8062_B4C0` (100 bytes) - byte-identical destructor twin next to
  [`fn_8062b460`].
- `0x8063_7310` (100 bytes) - four full sweeps of [`fn_80637120`] over record
  indices `0..3`; the results-screen reset `0x80637D20` calls it right after
  `0x80637050`.
- `0x8063_7CB0` (112 bytes) - hides the four results panes: for each of the
  four handles at `+0x76C`, `0x800C93D0(+0x70, index, 0)` then
  `pane->+0xBB &= 0xFE`.
- `0x8063_8720` (124 bytes) - starts animation `panel_index + 8` for every
  `0x3C`-strided record whose `+0x7E4` and `+0x7E5` bytes are both set, via
  `0x800C92F0(+0x70, id, 0)`.
- `0x8063_87A0` (112 bytes) - probes animation ids `8..11`; when all are idle
  it dispatches descriptor `0x8082_6D20` (`StateID_RankChange`) through slot
  `+0x18` of the `+0x6A8` state interface.

## Deferred

None.

## Notes and limitations

- The three `0x8062_49B0`/`4B60`/`4C80` gates are one template: the `bl`
  words differ only in the `0x800C9610` target (identical) and the `addi`
  descriptor constants `22344`/`22408`/`22280`.
- Descriptor names come from the existing registration tables:
  [`fn_80624ee0`] for `0x8082_5708`/`5748`/`5788`
  (`dModelPlayArrow_c`: StartWait, ActionWait, OnStageAnimeEndWait) and
  [`fn_806393f0`] for `0x8082_6D20` (`dPointResultDateFile_c::StateID_RankChange`).
  All three descriptors live past the file-backed end of the ELF
  (`0x8082_28B8`), so they are passed through as address literals and never
  read.
- `0x8063_7CB0` reuses the `+0xBB` bit-0 meaning already established by
  `0x80602B70` (`read_u8(+0xBB) & 0xFE | 1` raises the cursor/panel
  visibility). The `rlwinm r0,r0,0,24,30` word at `0x80637CF8` clears that bit.
- `0x8063_8720`'s record layout (`+0x7E0` index, `+0x7E4`/`+0x7E5` bytes,
  `0x3C` stride) matches the already-translated four-panel cluster
  [`fn_80637380`]/[`fn_80638960`]/`0x80639000`.
- `0x8063_87A0` dispatches through `this + 0x6A8`; `vmethod1` loads the vtable
  from that address and calls slot `+0x18` with `this + 0x6A8` as the receiver,
  which is exactly the raw `lwz r12,0x6A8/0x18` + `bcctr` sequence.
- The four destructors are the same template as [`fn_806241d0`]; the 100-byte
  copies keep the duplicated `cmpi cr0,3,r3,0` before the `addic.` sub-object
  guard. `0x8015F9F0` stays `// unknown:`; its class attribution for the three
  anonymous twins is `// unknown:`.
