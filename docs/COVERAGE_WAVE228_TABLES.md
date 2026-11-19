# Audit - Wave 228 (pointer-table batch 1)

First translation work on code outside `functions.tsv`. Six tables/groups were
decoded from `work/dol/main.elf` and their bodies translated into the modules
that own the referenced data. These rows stay **out** of
`ledger/functions.csv` by design, so the canonical 20,252 inventory is
unchanged; the untabled backlog shrank instead.

## Translated

| Lane | Table | Bodies | Bytes | Destination |
|---|---|---:|---:|---|
| A | `0x802EF328` (6-word find/compare run) | 4 + 2 bonus | 400 + 204 + 60 | `src/game/player.rs`, `src/runtime/input.rs` |
| B | `0x802F8510` (8-word curve dispatch) | 8 | 1,004 | `src/runtime/os.rs` |
| C | `0x803280E0` (`dScBoot_c` task table) | 19 | 1,212 | `src/game/boot_scene.rs`, `src/engine/dsys.rs` |
| D | `0x80310DB0` (collision-pair table) | 3 | 1,744 | `src/game/stage.rs` |
| E | `0x80343E90` (WPAD/KPAD callbacks) | 12 | 5,196 | `src/runtime/input.rs` |
| F | `0x802F8EF0` (gather-format kernels) | 38 | 7,920 | `src/render/geometry.rs` |

Totals: **84 bodies, ~17,700 bytes** of previously unrepresented code.

## Corrections found while decoding the tables

- **`0x802EF328` is one 6-word run, not two 3-entry tables**: slots 0-2 are the
  "find" policies (`0x800647A0`, `0x80064880`, `0x80064880`) and 3-5 the
  "compare" ones (`0x80064A40`, `0x80064A50`, `0x80064A50`); installed by
  `fn_80064720`/`fn_80064a10` into `0x80429CFC`/`0x80429D00` and reached only
  through the tail jumps `0x80064790`/`0x80064A30`. Two further frame-less
  leaves were found and translated.
- **`0x802F8510` is a jump table, not a vtable**: the eight words are the cold
  arms of a computed dispatch in the already-translated `fn_80241690`
  (`rlwinm r0,r8,12,27,29` -> `lwzx`/`bcctr`). All eight arms were translated
  (1,004 B, one more than the prologue sweep found) and the dispatcher was
  rewired.
- **`0x803280E0` is not `Pausewindow_c`**: it is the 32-word `dScBoot_c`
  per-frame task table (`u32 task(usize)`, walked by `0x8015F620`), installed
  by `0x8015DDD0`. The `Pausewindow_c::StateID_*` strings are merely adjacent
  in `.data`. Four additional frame-less leaves bring it to 19 bodies / 1,212 B.
- **`0x802F8EF0` is not `ScnObj`/`ScnLeaf` vtables**: it is six 7-entry
  gather-format kernel tables (dispatchers `fn_802556b0`, `fn_802562b0`,
  `fn_80256f90`; selector `(format >> 1) & 7`, 7 = "no kernel"). All 42 slots
  are unique; four were already translated in `runtime::os`, and the remaining
  **38 bodies (7,920 B)** were translated - the wave-227 inventory had listed
  only 20 bodies / 5,272 B and silently dropped 18 complete bodies.
- **`0x80343E90`** is a 32-word table indexed by `fn_801E6BE0` as
  `*(0x80343E90 + (id - 0x20) * 4)`, called with `(channel, report, out)`; the
  five `lis 0x8034` references nearby are string anchors for KPAD
  diagnostics, not table loads.
- **`0x80310DB0`** is a 16-word collision-pair table indexed by
  `a.+0x9E * 4 + b.+0x9E` and called as `handler(b, a)` from
  `collision_area_overlap`; slots 2/3/6/7 were already translated and 8-15 are
  constant stubs, so the group is now fully accounted for.

## Real defects fixed in existing code

- `w104_eval_curve5` had dropped the `f1` curve parameter and invented a
  six-argument variant that shifted every register (`r3 = 0`); the asm at
  `0x80241620` is five words plus `f1`.
- Two associativity/precedence bugs in lane F's own emitted bodies were caught
  before landing by a differential interpreter that runs the raw PowerPC body
  and the emitted Rust on identical random inputs: 38 functions x 40 trials,
  **0 mismatches** after the fixes.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `scripts/check.ps1` exit 0.
- Ledger unchanged at 20,250 rows (99.99%), 0 duplicates - untabled bodies are
  tracked in `work/wave228/lane*_tables.md` and the coverage docs.
