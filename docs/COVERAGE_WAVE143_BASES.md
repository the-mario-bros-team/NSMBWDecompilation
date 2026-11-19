# Coverage - Wave 143 (`game::bases`)

## How to read this file

This page documents the Wave 143 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`**; the Wii REL loader chooses
the real load address, so these numbers identify positions inside the REL image,
not runtime addresses. Every `# Original` block in the source carries the
`Synthetic base: 0x80600000; this is not a runtime address.` line.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee contract, a field role or a virtual dispatch
had to be named rather than resolved. Comments in the bodies keep
`// observed:` / `// inferred:` / `// unknown:` markers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 10 | 9,680 | 0 | 10 |

All 10 rows are `translated`; none is `partial`. The module header now reports
3613 bodies (3603 + 10).

## What this pass covers

- **Six "ready" bodies of `docs/SPEC_WAVE142_BASES.md`** (7,808 bytes), each
  re-decoded from `work/rels/elf/d_basesNP.elf` at
  `(address - 0x80600000) + 0x1000` and cross-checked against the export:

  | cluster | body |
  |---|---|
  | `remo_lift_up` execute | `0x806D09B0`: rider mask, `+0x430` slot, lean `+0xE8`, sounds `0x2B0`/`0x2B1` and the four-state settle machine |
  | `wendy` ring trail | `0x806B8380`: surface probe, two smoothing passes, 120-entry history, five cursors/nodes, `Wm_we_ringhit` bounce/wall and `Wm_we_ringkira` virtual |
  | thirteen-state transition | `0x80664610`: link `+0x424`/`+0x420`, camera zoom, emotes `165..168`, squared-distance gate, `0x80102260` fade |
  | world-map node start-up | `0x80792A20`: `W103` anchor, two staged `16383`-unit ramps, `W101` arrival effect, retire |
  | seven-bit layer gate | `0x8067E640`: mask bits from `+0x3ED..+0x3F1`, `0x800E41C0` refresh and `0x80162510` retire |
  | twenty-pane bind | `0x8062B5C0`: `Layout_MultiCourseSelectContents`, twenty `+0xF4` parents, bit-5 pattern, four heights |

- **Four `0x807B` planner-bucket bodies** (1,872 bytes), taking that bucket from
  81 to 77 untranslated addresses:

  | cluster | bodies |
  |---|---|
  | pane rect initialiser | `0x807B08B0` (`+0xD4`/`+0x1BC` rects, `6000.0`/`1.0` block, record `0x80164B20(1)`) |
  | row/audio selector | `0x807B23B0` (manager `+0x22`/`+0x23`, four `0x3B0` entries, `0x18`-byte records) |
  | player-slot mode table | `0x807B27A0` (`0x80354E50..0x80354E8C`, `+0x54` manager bytes) |
  | child-record spawner | `0x807B7080` (`0x2C0..0x2E8` record ids, handles `+0x11E8..+0x1204`) |

## Where the binary differed from the wave-142 prose

1. **`0x8067E640` uses different offsets.** The layer nibble and the "active"
   bit come from `+0x3E0`, not `+0x3E4`, and the two mask words are `+0x3D0` and
   `+0x3D4`, not `+0x3E0`/`+0x3E4`. The height table is `0x807C960E/0x807C9610`,
   not `0x807CA6A0`. `0x8042A078` is a **pointer** to a two-word mask table.
2. **`0x80664610` has no `+0x3D0` gate in state 7.** The raw words run the emote
   loop and the `dx*dx + dy*dy + dz*dz < 500.0` test on every tick; the only
   timer tests are in states 2, 5, 6, 8 and 11.
3. **`0x80792A20` uses `0x80162590`, not `0x80792EE0`,** and the state-6
   sub-object argument is `this + 0x704` (the export's `&this->+0x770` is the
   animation object read by `0x801680E0`, not the spawn argument).
4. **`0x8062B5C0` resolves `Layout_MultiCourseSelectContents`** (`0x807DBDD8`)
   with the length `18` left in `r5` by the peeled twenty-pointer validation
   loop; it is not the `_ef_attack` pane of the wave-142 prose. `0x8062C4B0`
   takes a second argument `1`.
5. **`0x806B8380` stores `+0x100C`, not `+0x1008`,** in the scalar history, and
   both bounce and wall tests reuse `Wm_we_ringhit`; `Wm_we_ringkira` is only the
   final virtual call. The scale sign test is `0.0 <= negated`, not an equality.
6. **`0x806D09B0`'s settle state 2 moves `+0xB0`,** not `+0xAC`; the ride path
   moves `+0xAC`. The state-0 `0x806CFFD0` call site does not load the float
   argument, which the body documents rather than invents.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8062B5C0` | `fn_8062b5c0` | 1,136 | medium |
| `0x80664610` | `fn_80664610` | 1,292 | medium |
| `0x8067E640` | `fn_8067e640` | 1,160 | medium |
| `0x806B8380` | `fn_806b8380` | 1,500 | medium |
| `0x806D09B0` | `fn_806d09b0` | 1,508 | medium |
| `0x80792A20` | `fn_80792a20` | 1,212 | medium |
| `0x807B08B0` | `fn_807b08b0` | 460 | medium |
| `0x807B23B0` | `fn_807b23b0` | 372 | medium |
| `0x807B27A0` | `fn_807b27a0` | 456 | medium |
| `0x807B7080` | `fn_807b7080` | 584 | medium |

**Totals:** 10 bodies, 9,680 bytes. The address window runs from `0x8062B5C0` to
`0x807B7080`.

## Notes on inference

- The ten `medium` rows each call at least one unclaimed sibling or main.dol
  address (`0x800441E0`, `0x806CFB30`, `0x806CFFD0`, `0x806D0410`, `0x806D0780`,
  `0x80075270`, `0x8006CD40`, `0x8006CED0`, `0x800E78E0`, `0x807211D0`,
  `0x80721290`, `0x80721270`, `0x80662E10`, `0x80663000`, `0x80663050`,
  `0x80663180`, `0x80662F40`, `0x80162590`, `0x8076D1F0`, `0x80006BE0`,
  `0x8062BA50`, `0x8062C4B0`, `0x8008E890`, `0x8067DCF0`, `0x8067DC50`,
  `0x800E41C0`, `0x800B54B0`, `0x8008E3D0`, `0x8008E5C0`, `0x8008E5F0`,
  `0x807B1EE0`, `0x807B87F0`, `0x80164B20`, `0x802BF420`, `0x802BFFF0`). Their
  argument shapes are fixed by the instruction stream; their bodies are not
  claimed.
- The `0x80377DD4` negation vector in `0x80792A20` and the `0x80354E50` table in
  `0x807B27A0` live in `main.dol` BSS; the bodies read/write them by absolute
  address and say so.
- `0x80792A20`, `0x807B08B0`, `0x807B23B0` and `0x807B7080` call their virtual
  slots through `ptr_at`, exactly as the raw `lwz r12,slot; mtspr CTR; bctr`
  sequences require.

## Deliberate skips

Read and rejected, or deferred with the reason:

- **`0x807962B0` (1,428)** and **`0x80605BA0` (1,164)** are the two "conditional"
  rows of `docs/SPEC_WAVE142_BASES.md`. `0x807962B0` still needs the `r3` left by
  the `void` `0x801002C0`/`0x800F53A0` tail call, and `0x80605BA0`'s pane stride
  disagrees between the clear loops and `fn_80605a20`/`aa0`/`b20`. Neither value
  can be invented, so both remain open.
- The remaining `0x807B` bucket rows above 400 bytes (`0x807BFFA0`,
  `0x807B77E0`, `0x807BEC10`, `0x807B84C0`, `0x807BC310`, `0x807B87F0`,
  `0x807B2050`, `0x807BAC40`, `0x807BBE20`, `0x807B2550`, `0x807BE750`,
  `0x807B7B40`, `0x807B8CB0`, `0x807B7540`, `0x807BF930`, `0x807BC7E0`,
  `0x807BBA40`, `0x807B7D60`, `0x807B90F0`, `0x807BE4C0`, `0x807BE230`,
  and the 1,096-byte `0x807BEF60`) were not started; several are `FUN_807B87F0`
  neighbours in the same call graph and are the natural next wave.
- `0x807BEF60` was read in full and deferred: its `0x802D1210` calls need the
  `0x803504E0`/`0x803504EC` fixed-point screen words decoded as a pair before the
  rect arguments can be written honestly.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (10 rows, 9,680 bytes),
  re-read from the TSV for this pass rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 10 were checked against
  the ledger's address column before being written, and none is duplicated.
- Each body's last word is `0x4E800020` (`blr`) at `entry + size - 4`; no
  `functions.tsv` row starts inside any body.
- `node tools/check-lane-csv.mjs wave143` reports `OK`: 10 rows, 10 unique
  addresses, all `translated`.
- `cargo check --message-format=short` reports no error or warning naming
  `src/game/bases.rs` (the two remaining errors are in the parallel
  `src/game/enemies.rs` lane).
