# Coverage - Wave 210 lane A (`game::player`, roster slot bookkeeping)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 10 | 1504 | 7 | 3 | 0 |

Status split: 10 `translated`, 0 `partial`, 0 deferred. Bytes are the
`functions.tsv` sizes: 92 + 100 + 80 + 520 + 296 + 112 + 144 + 104 + 24 + 32.

## How to read this file

* **Address** - virtual entry point in the DOL image, not a file offset.
* **Name** - the Ghidra name (`FUN_<address>`). No symbol table survives for
  these functions, so the Rust body keeps the `fn_<address>` name.
* **Role** - one plain sentence per function, in "What this pass covers". A
  role is marked "inferred" when it comes from shared field offsets and call
  shapes rather than from a recovered symbol.
* **Confidence** - strength of the evidence, not code quality: `high` = every
  address, offset and branch is literal; `medium` = a field role or a callee's
  meaning had to be inferred.
* All ten bodies use the SDA bases this module already fixes,
  `r2 = 0x80433080` and `r13 = 0x8042F680`. Ghidra never recovered either base
  for this run, so every export prints `*(...)(unaff_r13 + -0xNNNN)` /
  `(unaff_r2 + -0xNNNN)`. The four offsets that were still unnamed are resolved
  here from `work/dol/main.elf`:

  | offset | address | value | meaning |
  |---|---|---|---|
  | `r13 - 0x7D7C` | `0x80427904` | `99` | ceiling for the roster tally |
  | `r13 - 0x7D80` | `0x80427900` | `99` | ceiling for a per-character counter |
  | `r13 - 0x7D78` | `0x80427908` | `999999999` | ceiling for the shared total |
  | `r2 - 0x75E8` | `0x8042BA98` | `0.5f` | half-extent scale |

  Every other global is an existing constant: `W88_SLOT_HANDLES_80354E50`,
  `W88_SLOT_CHAR_ID_80354E60`, `W88_CHAR_COUNTER_80354E90`,
  `W88_CHAR_AUX_80354EA0`, `W88_WORD_80429CC0`, `W88_WORD_80429CE8`,
  `W22_SCENE_80429E00`, `W18_SOUND_80429FB0`, `W18_UI_8042A488`,
  `W18_ACTOR_MANAGER_8042A050`, `W74_SOUND_8042A4A8`,
  `W97_EFFECT_OWNER_80429D60`. No constant was redefined.

## What this pass covers

The ten bodies are one closed cluster: the four-entry roster tables of wave 88
(`0x80354E50`..`0x80354EA0`) and the sparse one-word "slot handle" the rest of
the game passes around. They are the only users of the handle in this file.

- **`0x80060110` (92 B, high).** Character id -> slot index. A four-way compare
  against `0x80354E60..0x80354E6C`, returning `0`/`1`/`2`/`3` or `-1`. Both
  translated callers (`0x80010690`, `0x80010800`) drop the result, so the
  `-1` sentinel is only documented, never taken.
- **`0x80060170` (100 B, high).** Handle validation. `0x8005F900(handle)` then
  the actor vtable entry `+0xE0` (`call_actor_slot1(actor, 0xE0)`); on success
  the handle is zeroed through the pointer and `1` is returned. Every other
  body in the lane runs it first, which is why they all keep the handle in a
  stack word (`local_38`/`local_8`/`local_18` in the exports).
- **`0x80060200` (80 B, high).** `AUX[CHAR[0]] + AUX[CHAR[1]] + AUX[CHAR[2]] +
  AUX[CHAR[3]]`. The export's `(&DAT_80354ea0)[DAT_80354e60]` notation is the
  nested index: the four words at `0x80354E60` are the character ids that index
  `0x80354EA0`.
- **`0x80060250` (520 B, medium).** The per-frame driver, and the only body in
  the lane whose argument Ghidra drops: it is declared `void FUN_80060250(void)`
  because the lone argument (`r3`) is consumed after the `bl 0x802DCD74`
  register-save stub. `0x8006D170` calls it with the character index. Flow:
  validate the handle, bump the sound manager's per-slot count, compare the
  tally with `99`. Under the ceiling it adds one to `AUX[CHAR[slot]]`; at or
  over it posts effects `0xB3`/`0xB4` (only when more than one slot is
  occupied), then runs the four-slot spend loop and clears all four `AUX`
  entries. Two compiler idioms were read bit by bit and are reproduced
  literally rather than simplified: the occupancy test
  `bit30((count ^ 1) >> 1 - ((count ^ 1) & count))` and the
  `neg; or; rlwinm 1,31,31` form of `(uint)-flag >> 31`. This is the medium
  rating.
- **`0x80060460` (296 B, high).** `COUNTER[CHAR[slot]] = min(amount + value,
  99)`, optionally preceded by the `0xB1`/`0xB2` effect pair (third argument
  exactly `1`). Each descriptor gets its own `0x80069530(slot)` mask, unlike the
  shared mask in `0x80060250`.
- **`0x80060590` (112 B, high).** Four-iteration loop: `0x80060460(slot, 1,
  flag)` for every occupied slot. `0x800E1F80` and `0x800E2340` are the callers.
- **`0x80060600` (144 B, high).** Announces sound `0xB5`, then decrements
  `COUNTER[CHAR[slot]]`; returns `0` when the counter was already non-positive
  (the store is skipped entirely in that case) and clears `0x80429CE8` when the
  affected character is character `0`.
- **`0x80060690` (104 B, high).** `0x80429CC0 = min(amount + value, 999999999)`
  and, for `slot < 4`, `0x800CE950(0x80429FB0, amount, slot)` - the *second*
  parameter is the slot even though `0x80087F40`/`0x80088030` print one
  argument.
- **`0x80060700` / `0x80060720` (24 B / 32 B, medium).** The pause/resume pair
  around bit `0` of `0x80429CE4`. `0x80060720` sets the bit (`ori 1`) and calls
  `0x8019BC50(sound_object, 4)`; `0x80060700` tests the bit and calls the same
  helper when it is set. Both are `b`-tail calls in the original, with a dead
  `blr` after the branch that the `functions.tsv` sizes exclude. The bit
  polarity and the single caller of `0x80060720` (`0x80145BD0`) are what make
  the pause/resume reading an inference, hence medium.

## Notes and limits

* Extents were checked against each `blr`. All ten match `functions.tsv`
  exactly; the only padding in the run is `0x8006016C`, `0x800601D4`,
  `0x80060458`, `0x800606F8`, `0x8006071C` and `0x80060744`.
* The two stack buffers handed to `0x8006A3F0` are the `8`-byte pairs at
  `r1 + 0x18` / `r1 + 0x10` (in `0x80060250`) and `r1 + 0x18` / `r1 + 0x10` (in
  `0x80060460`); they are modelled as `[f32; 2]`.
* Nothing was deferred. No body was guessed: the three `medium` ratings mark
  inferred *roles* (effect ids `0xB1`..`0xB5`, the pause latch), not uncertain
  behaviour.
