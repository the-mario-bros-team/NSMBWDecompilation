# Coverage - Wave 208 lane A (`ui::hud`, main.dol)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/hud.rs` | `ui::hud` | 10 | 2832 | 8 | 2 |

Addresses `0x8003B020`-`0x8003C690`, the `daEnShell_c` shell-state block that
follows `0x8003ACD0` (`fn_8003acd0`, the last body of wave 207 lane F). The
family record shape is unchanged: class vtable at `+0x60`, position at
`+0xAC..+0xB4`, player/direction byte at `+0x348`, animation block
`+0xE8..+0xFC`, collision extents `+0x164..+0x170`/`+0x7A0..+0x7AC`, flag word
`+0x178`, controller at `+0x1EC` and the `+0x614`/`+0x620`/`+0x630`/`+0x640`
controller flag words.

## What landed

- **Run-state entry.** `0x8003B020` (272 B) adopts the digit word `+0x7C8` into
  `+0x38D`, resolves the player record through `0x8005F900`, picks the contact
  layer from `0x8042BB18` (byte `1` or `2`), caches the extents at
  `+0x7A0..+0x7AC` and - when the player's `+0x1090` mode is neither `0` nor `3`
  - installs `0.0 / 5.0 / 7.0 / 9.0` as the live extents.
- **Run-state exit.** `0x8003B6F0` (304 B) releases the `+0x51C` holder (with
  its virtual `+0x08(holder, 1)`) after allocating a fresh four-byte holder
  stamped `0x80305C50`, or runs `0x80065CB0(this)` when `+0x7CC` is already
  zero. It then restores the `-4.0f` floor, clears `+0x7CC`, masks the four
  flag words and re-binds the controller through `0x8006D090`.
- **Shared ticks.** `0x8003BBB0` (116 B) and `0x8003BC80` (136 B) are the same
  `0x8006CDE0` / `0x800955F0` / `0x80096E60` sequence with the bit-`1`/bit-`4`
  `+0xEC`/`+0xE8` resets; the first switches to the registered `WakeupTurn`
  state (`0x80353B24`), the second runs the virtual `+0x2E4` query and, when it
  answers, the `+0x2E0` release.
- **Probe and pre-release.** `0x8003BC30` (48 B) refreshes `+0x348` from
  `0x80096240(this, &(+0xAC))`. `0x8003BE00` (84 B) asks virtual `+0x2F0`,
  clears `+0x77C` when it answers, clears `+0x45C` and runs `0x800A16A0`.
- **Walk step.** `0x8003BE80` (244 B) adds `3072` to `+0x100` and the
  `{-1024, +1024}` table entry (sign-selected by `+0x348 == +0x459`) to
  `+0x102`, doubles both by `0.5` when `+0x45D` is set, and then runs the
  class and position ticks.
- **Release pair.** `0x8003C010` (60 B) is the `(ptr, size)` free wrapper
  (`0x802B9280(ptr)` when `ptr != 0 && size > 0`, returning `ptr`) and is
  byte-identical to `0x8015F800`, the helper already called from this module.
  `0x8003C690` (88 B) chains `0x8015F800(ptr, 0)` and the same free; it is the
  destructor registered for all seven shell states.
- **State registration.** `0x8003C050` (1496 B) is the static initialiser that
  builds the seven 0x40-byte records at `0x803539D8`: read the nine source
  words, call the per-state accessor (`0x80037EA0` family, `0x8003C700`,
  `0x8003C6F0`), stamp the descriptor with `0x8015F7C0`, store the words in
  triple order `3, 2, 1`, write the `0x80305E58` vtable and the accessor, and
  link the record with `0x802DC780(descriptor, 0x8003C690, record)`.

## Corrections and recoveries

- **No prologue artefacts in this lane beyond the usual four bytes.** All ten
  `functions.tsv` sizes are the real extent plus four alignment bytes. The
  `blr`/epilogue sites are `0x8003B130`, `0x8003B820`, `0x8003BC24`,
  `0x8003BC60`, `0x8003BD08`, `0x8003BE54`, `0x8003BF74`, `0x8003C04C`,
  `0x8003C628` and `0x8003C6E8`, i.e. 272/304/116/48/136/84/244/60/1496/88.
- **`0x8003B6F0`'s flag-mask block is inconsistent in the shipped image.** The
  `lis`/`addi` pairs build `0xFB64FBFF` (`0x8003B780`/`0x8003B798`) and
  `0xFF64FBFF` (`0x8003B788`/`0x8003B7B8`), but the `stw` at `0x8003B7EC`
  stores `r10`, a register this function never loads, and the related
  `and r6,r10,r5` result is dead. The Ghidra export instead reports the
  coherent read-modify-write pair `& 0xFB63FBFF` / `& 0xFF63FBFF`. The body
  follows the export's read-modify-write form (the only behavioural reading
  that can be stated at all) and uses the masks the instruction stream builds;
  the discrepancy is recorded as an observed note on the body.
- **The two `0x80037EA0`-style accessors return `0x80377A04`, not a state
  word.** `STATE_GETTERS_A` therefore lists raw addresses rather than assuming
  a common payload: five return `0x80377A04`, `0x8003C700` returns
  `0x80352DB4` and `0x8003C6F0` returns `0x80357BF4`.
- **`0x8003C050`'s nine-word copy is a permutation, not a straight copy.**
  `descriptor + 0x0C/0x10/0x14` receive source words 6/7/8, `+0x18/0x1C/0x20`
  receive 3/4/5 and `+0x24/0x28/0x2C` receive 0/1/2. The source blocks hold
  three `{0, size, 0x60}` triples (sizes `0x294, 0x290, 0x28C, ...`); the
  destination order is triple 3, 2, 1. Reproduced exactly.
- **`0x8003C050`'s bracket calls use `r11`.** `0x802DCD6C` stores `r25..r31`
  at `r11 - 0x1C` and `0x802DCDB8` restores them; `r11` is not an EABI argument
  register, so both are called with no operands (as the export prints them) and
  the Rust prologue covers the callee-saved set.
- **`0x8003C690` carries a dead duplicate null test** at `0x8003C6B4`
  (`4182000C`), which re-uses the `cr0` of the `cmpi` at `0x8003C698` and can
  never be taken. Not reproduced; behaviour is unchanged. The same pattern was
  already recorded for `fn_800365d0`.
- **The `0x8003BE80` s16 scalings are the `2^52` conversion idiom.** The export
  prints `CONCAT44(0x43300000, ...) - *(double *)(r2 - 0x7AA8)`; it is a
  signed-integer-to-double conversion followed by `fctiwz`, so the body writes
  it as `(value as f32) * 0.5` truncated to `i32` and then `i16`. The
  `lha`/`add`/`sth` accumulation wraps at 16 bits and is written with
  `wrapping_add`.

## Deferrals

None. All ten bodies were reconstructible from the shipped code, including the
1500-byte registration body, which needed no partial deferral.

## Verification

- Temp-copy `cargo check --lib` exit 0; the appended block re-checked clean
  (the 23 warnings are the pre-existing ones in other modules, none in
  `src/ui/hud.rs`).
- The lane CSV was validated by hand against `tools/check-lane-csv.mjs`'s rules:
  canonical 7-column header, 10 rows, 7 fields each, unique plain lowercase hex
  addresses, evidence files present, and `# Original` sizes equal to
  `functions.tsv` (276/308/120/52/140/88/248/64/1500/92).
- No files outside `work/wave208/laneA_block.rs`,
  `ledger/agents/wave208-laneA.csv` and this document were written;
  `src/ui/hud.rs` is untouched.
