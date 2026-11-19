# Coverage - Wave 211 lane E (`game::player`, carried-slot and cue helpers)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 10 | 1960 | 9 | 1 | 0 |

Status split: 10 `translated`, 0 `partial`, 0 deferred. Bytes are the
`functions.tsv` sizes from `work/ghidra/main-gb/export/functions.tsv`: 132 + 176
+ 700 + 152 + 64 + 128 + 116 + 292 + 12 + 188.

## How to read this file

* **Address** - virtual entry point in the DOL image, not a file offset.
* **Name** - the Ghidra name (`FUN_<address>`). No symbol table survives for
  these functions, so the Rust body keeps the `fn_<address>` name.
* **Role** - one plain sentence per function in "What this pass covers". A role
  is marked "inferred" when it comes from shared field offsets and call shapes
  rather than from a recovered symbol.
* **Confidence** - strength of the evidence, not code quality: `high` = every
  address, offset and branch is literal; `medium` = a field role or a callee's
  meaning had to be inferred.

## Size verification

Every `functions.tsv` size was re-checked against the prologue and the closing
`blr` in `work/dol/main.elf`:

| address | tsv size | prologue | terminal `blr` | real extent |
|---|---:|---|---:|---:|
| `0x80065080` | 132 | `stwu r1,-0x30` | `0x80065100` | 132 |
| `0x80065110` | 176 | `stwu r1,-0x10` | `0x800651BC` | 176 |
| `0x800651C0` | 700 | `stwu r1,-0xD0` | `0x80065478` | 700 |
| `0x80065480` | 152 | `stwu r1,-0x20` | `0x80065514` | 152 |
| `0x80065A00` | 64 | `stwu r1,-0x10` (frame unused, no LR save) | `0x80065A3C` | 64 |
| `0x80065A40` | 128 | `stwu r1,-0x10` | `0x80065ABC` | 128 |
| `0x80065AC0` | 116 | `stwu r1,-0x60` | `0x80065B30` | 116 |
| `0x80065B80` | 292 | `stwu r1,-0x30` | `0x80065CA0` | 292 |
| `0x80065CB0` | 12 | none (leaf) | `0x80065CB8` | 12 |
| `0x80065FC0` | 188 | `stwu r1,-0x20` | `0x80066078` | 188 |

**One brief note.** The lane target list quoted **16 bytes** for `0x800651C0`.
That is the prologue artefact of `work/ghidra/main/export/functions.tsv`
(which also says 16 for this address); it stops at the opening `stwu`. The
`main-gb` export - the evidence directory this lane uses - lists 700 bytes, and
the body agrees: `stwu r1,-0xD0` at `0x800651C0` plus the epilogue
`addi r1,r1,0xD0; blr` at `0x80065478` give `0x800651C0..0x8006547C` = 700
bytes, ending exactly at the next function (`0x80065480`). The `# Original`
line of [`fn_800651c0`] keeps 700 and states the artefact.

The two other "short" frames in this lane are real, not artefacts: `0x80065110`
saves `r31` and the LR in its 16-byte frame, and `0x80065A00` allocates 16 bytes
it never uses (it is a leaf and saves nothing).

## What this pass covers

The ten bodies are the `0x80065xxx` gap between the already-translated
`0x80060xxx` player management group and `0x80065520`: the carried/held-slot
bookkeeping, the three-rail cue ramp, the model-effect vector refresh, the
nine-id cue table and two slot resolvers.

| address | function | role | confidence |
|---|---|---|---|
| `0x80065080` | [`fn_80065080`] | rotate `+0xAC..+0xB4` with a null angle triple (`0x80064AF0(v,0,1)`), build the sprite rect `(+0x320, +0x324, 0.5*+0x318, 16 + 0.5*+0x31C)` and return the `0x800B55F0` off-screen flag | high |
| `0x80065110` | [`fn_80065110`] | four-way `+0x34A` predicate: bit `1` vs state `0x15`, bit `2` vs `0x2A`, bit `8` via the `+0x1EC` probe `0x80070CB0`, bit `4` vs state `0x3C000000` | high |
| `0x800651C0` | [`fn_800651c0`] | keep `+0x128` (the carried-slot id) alive: invalidate it from the `0x80429F7C` record byte, then box-overlap the player against the tracked actor (`0x8005F900`) or a candidate (`0x80065520`) through `0x800B3100` | high |
| `0x80065480` | [`fn_80065480`] | walk four slots, promote `chosen` (`0x800B3720` + `0x800B3780`) and clear the rest (`0x800B3750`), then `+0x128 = -1` | high |
| `0x80065A00` | [`fn_80065a00`] | map the `+0x1530` counter to `1.0`, or `counter / 1.25` while it is below `2` | medium |
| `0x80065A40` | [`fn_80065a40`] | copy `+0x370/+0x374/+0x378` to `+0xDC/+0xE0/+0xE4` and scale the copy by class slot `+0xA8` when it is below `1.0` | high |
| `0x80065AC0` | [`fn_80065ac0`] | build a stack `Mtx` from `0x80152480` and store its x into `+0xAC` and y-minus-`+0xD4` into `+0xB0` | high |
| `0x80065B80` | [`fn_80065b80`] | seed and play one of the nine character cue ids `0x196..0x19C` through `0x80197F00`, with the `4`/`7` notification threshold and the `0x80069530` mask | high |
| `0x80065CB0` | [`fn_80065cb0`] | clear the `+0x134` word | high |
| `0x80065FC0` | [`fn_80065fc0`] | scan the four slots for `0x80162D00([slot+0x2A78]) == id` and report its `+0x6C` id byte (or `-1`) through `*out` | high |

## Object map used by the lane

| offset | type | role |
|---|---:|---|
| `+0x60` | `u32` | class vtable (`(*(this+0x60))[slot](this)`); slots `+0x6C` and `+0xA8` are used here |
| `+0xAC..+0xB4` | 3 `f32` | scene position (x, y, z) |
| `+0xD4` | `f32` | y offset removed by [`fn_80065ac0`] |
| `+0xDC..+0xE4` | 3 `f32` | effect vector refreshed by [`fn_80065a40`] |
| `+0x128` | `u32` | carried-slot id; `-1` is the empty sentinel |
| `+0x134` | `u32` | timer cleared by [`fn_80065cb0`] |
| `+0x1EC` | object | sub-object probed by `0x80070CB0` |
| `+0x274` | `u32` | state word masked by [`fn_80065110`] |
| `+0x2A78` | `u32` | slot id read by [`fn_80065fc0`] before `0x80162D00` |
| `+0x318` / `+0x31C` | `f32` | sprite size whose halves feed the `0x800B55F0` rect |
| `+0x320` / `+0x324` | `f32` | sprite rect origin |
| `+0x34A` | `u8` | attachment flag byte of [`fn_80065110`] |
| `+0x370..+0x378` | 3 `f32` | source of the `+0xDC..+0xE4` copy |
| `+0x38D` | `u8` (signed) | character index; `< 4` gates [`fn_80065b80`] |
| `+0x1530` | `u32` | ramp counter of [`fn_80065a00`] |

## Constants resolved for this lane

Ghidra printed `unaff_r2` / `unaff_r13` for every small-data reference. With
the bases this module already fixes (`r2 = 0x80433080`, `r13 = 0x8042F680`) the
offsets resolve to:

| offset | address | value | use |
|---|---|---|---|
| `r2 - 0x7544` | `0x8042BB3C` | `0x3F000000` = `0.5f` | half the sprite size in [`fn_80065080`] |
| `r2 - 0x7530` | `0x8042BB50` | `0x41800000` = `16.0f` | sprite height bias in [`fn_80065080`] |
| `r2 - 0x7554` | `0x8042BB2C` | `0x00000000` = `0.0f` | the unused `f1` of the `0x800B3100` calls and of the four `0x800B37xx` calls |
| `r2 - 0x7540` | `0x8042BB40` | `0x40000000` = `2.0f` | x margin of the tracked-actor box |
| `r2 - 0x7558` | `0x8042BB28` | `0x3F800000` = `1.0f` | "keep the counter" value and the `+0xA8` threshold |
| `r2 - 0x7508` | `0x8042BB78` | `0x43300000_80000000` = `2^52 + 2^31` | unsigned-int to double conversion bias |
| `r2 - 0x750C` | `0x8042BB74` | `0x3FA00000` = `1.25f` | ramp divisor |
| `r13 - 0x5704` | `0x80429F7C` | BSS pointer | record table with the `+0xAFE` slot byte |
| `r13 - 0x5360` | `0x8042A320` | BSS pointer | record table with stride `0x23C`, byte `+0x381` |
| `r13 - 0x5924` | `0x80429D5C` | BSS pointer | effect/sound handle for `0x80197F00` |
| `r13 - 0x5957` | `0x80429D29` | BSS byte | "cue table seeded" flag |
| `r13 - 0x56D0` | `0x80429FB0` | BSS pointer | manager passed to `0x800CE8C0` |

`0x80355E40` holds the nine cue ids `0x196, 0x197, 0x198, 0x199, 0x19A, 0x19B,
0x19C, 0x19C, 0x19C`.

## Reuse and new symbols

No existing constant or helper was redefined. The lane reuses
`W98_NOTICE_80429F7C`, `W18_SOUND_80429FB0`, `W210_SLOT_MASK_80069530_A`,
`read_f32` / `write_f32` / `read_i8`, `call0_u32`, `call_actor_slot1` and
`call_actor_slot_f32_ret`. New lane symbols are suffixed `_E`:
`TABLE_8042A320_E`, `SOUND_HANDLE_80429D5C_E`, `CUE_TABLE_80355E40_E`,
`CUE_READY_80429D29_E`, `f32_of_f32_E` and `rect_overlap_E`. (`p32`,
`vtable_method` and `call0_r` are not defined in `src/game/player.rs`, so the
lane uses `platform::read_*` and the local helpers instead of assuming them.)

`fn_80065080` is declared `-> u32` although the export types `FUN_80065080` as
`void`: the `0x800B55F0` result stays in `r3` through the epilogue
(`lwz`/`mtspr`/`addi r1` do not touch it), so the value is visible to callers.

## Deferrals

None.

## Verification

A scratch copy of `src/`, `Cargo.toml` and `Cargo.lock` had
`work/wave211/laneE_block.rs` appended to `src/game/player.rs`;
`cargo check --lib` in that copy finished with **exit 0** and no errors and no
diagnostics in `src/game/player.rs`.
