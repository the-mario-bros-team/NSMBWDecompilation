# Coverage - Wave 208, lane C (`ui::hud`, `0x8003CCC0..0x8003DDF4`)

## How to read this file

This page records the Wave 208 lane C pass over `src/ui/hud.rs`: ten bodies of
`main.dol` in the run `0x8003CCC0..0x8003DDF4` (1,596 bytes by the verified
extents; 1,440 by the `functions.tsv` rows). The block is staged in
`work/wave208/laneC_block.rs` for the merge pass; it is appended verbatim at the
end of `src/ui/hud.rs`. One row per body is mirrored in
`ledger/agents/wave208-laneC.csv`.

Every address is a **runtime address** of `sys/main.dol`, the same address the
Ghidra exports in `work/ghidra/main-gb/export/` use, and each function doc
carries `# Original` (Ghidra name, address, byte count, evidence path) and
`# Confidence` plus an argument/return/side-effect/safety block.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/hud.rs` | `ui::hud` | 10 | 1,596 | 8 | 2 |

All ten rows are `translated`; none is `partial` or `stub`. Covered range:
`0x8003CCC0..0x8003DDF4`.

Raw words were decoded with `node $env:TEMP\opencode\ppc159.js work/dol/main.elf
<addr>`, and each body was followed from its entry to its terminal `bclr`.
Verified extents, as `entry .. blr + 4`:

| body | `functions.tsv` | verified extent |
|---|---:|---:|
| `0x8003CCC0` | 120 | `0x78` = 120 |
| `0x8003CE20` | 236 | `0xEC` = 236 |
| `0x8003CF40` | 16 | `0xAC` = 172 (prologue artefact, see below) |
| `0x8003D890` | 92 | `0x5C` = 92 |
| `0x8003D9D0` | 224 | `0xE0` = 224 |
| `0x8003DAB0` | 136 | `0x88` = 136 |
| `0x8003DB70` | 48 | `0x30` = 48 |
| `0x8003DBA0` | 48 | `0x30` = 48 |
| `0x8003DBD0` | 228 | `0xE4` = 228 |
| `0x8003DCD0` | 292 | `0x124` = 292 |

Nine rows are exact with no alignment padding. The one exception is
`0x8003CF40`, whose `functions.tsv` row in the `main` analysis is `16` bytes -
the prologue-only artefact the shared instructions warn about. The body starts
at `0x8003CF40`, its terminal `bclr` is at `0x8003CFE8` and the next body starts
at `0x8003CFEC`, so the real extent is `0xAC` (172 bytes), which is exactly what
`work/ghidra/main-gb/export/functions.tsv` records for the same address. The
`# Original` line of the doc block keeps the checker's value (`16`) and the
prose records the real extent.

## What this pass covers

The run is the closing slice of the `togezo` (Spiny) scene plus the helpers
immediately before it. Wave 207 lane D already translated the release band, the
base destructor and the model builder `0x80037D20..0x800381B0`, and documented
that the only in-image caller of [`fn_800381b0`] is `0x8003DBD0` - the build
body in this lane.

- **`0x8003CCC0`, `0x8003CE20`, `0x8003CF40` - the per-frame step (120/236/172
  bytes).** `0x8003CCC0` stores the result of owner slot `+0x2E0` at `+0x694`,
  runs the `+0x394` member's virtual `+0x10` (receiver `this + 0x394`, whose vptr
  is at `+0x394`), calls the two bodies below, then dispatches through owner
  slot `+0x2CC` and returns `1`.
  `0x8003CE20` asks slots `+0x2C0`/`+0x2D0` for the current position, copies the
  `u16` angle triple `+0x100..+0x104`, normalises it in place with
  `0x80064AF0(position, angles, 1)`, emits the position through `0x801C0BD0`,
  writes the three angles back in reverse order (`+0x104`, `+0x102`, `+0x100`)
  and rebinds `+0x544` to `+0x7C` and `+0xDC`. `0x8003CF40` integrates the `i16`
  angle at `+0x102` by `rate * (i16)count`, clamped at `0xA00` (2560) with no
  negative clamp; the addition is a 16-bit `sth`, so the field wraps.

- **`0x8003D890` - release wrapper (92 bytes).** `0x8015F800(this, 0)` and,
  when `flag > 0`, `0x802B9280(this)`; returns `this` in every path. It is
  instruction-identical to [`fn_80037d20`].

- **`0x8003D9D0` - bounded child walk (224 bytes).** Five unrolled levels of
  `slot +0x0C` on `*(node + 0x30)`; the first non-zero answer returns the
  **parent** node, and an all-zero run recurses with the next child. The
  function at `0x8003D8F0` (outside this lane) contains the same five levels
  inline and ends in a `bl 0x8003D9D0`.

- **`0x8003DAB0` - name-tail compare (136 bytes).** Byte-identical to
  [`fn_8003c8d0`]: search both names for `':'` with `0x802E1C40`, advance past
  the separator (the object side advances even on a null search), and compare
  with `0x802E1AB4`. The object's name comes from its slot `+0x20`.

- **`0x8003DB70`, `0x8003DBA0` - `0x802DCBFC` adjustor thunks (48 bytes each).**
  `r5 = a`, `r3 = r4 = b`, `r12 = a + 0x18` (resp. `+0x24`), then the dispatch.
  They are the `+0x18`/`+0x24` instances of the shape already translated as
  [`fn_80037e40`]/[`fn_80037e70`].

- **`0x8003DBD0` - scene build (228 bytes).** `0x8003DCD0(this)`, then
  `0x800381B0` with the literals `"togezo"` (`r13 - 0x7DB0`),
  `"g3d/togezo.brres"` (`0x80306388`), `"togezo_shell"` (`0x8030639C`), `0` and
  the `0.0f` at `r2 - 0x79C0`; then eighteen field stores (the `f32` block
  `+0x318..+0x324` and `+0xD0..+0xFC`, `+0x102` from the halfword table at
  `r2 - 0x79C4` indexed by the kind byte `+0x348`, `+0x518 = 8`, `+0x894 = 2`)
  and the `+0x300` virtual; returns `1`.

- **`0x8003DCD0` - archive/model setup (292 bytes).** Clears the `0x20`-byte
  name buffer at `+0x7DC`, loads `g3d/togezo.brres` into `+0x7F8`, resolves
  `"walk"`, binds `+0x7FC`, resolves and binds the `"paipo"` clip into `+0x83C`,
  loads `g3d/paipo.brres` into `+0x874`, resolves `"paipo"`, binds `+0x878`
  through `0x8016A3A0` and detaches the buffer with `0x800690E0`.

## Notes and limitations

- **New constants are `_C`-suffixed and few.** `FN_8016A3A0_C`, the strings
  `S_TOGEZO_ARCHIVE_C` (`0x804278D0`), `S_TOGEZO_MODEL_WALK_C` (`0x804278D8`),
  `S_TOGEZO_ARCHIVE_PAIPO_C` (`0x804278E0`), `S_G3D_TOGEZO_BRRES_C`,
  `S_TOGEZO_SHELL_C`, `S_G3D_PAIPO_BRRES_C`, the six `F32_SDA2_79xx_C` anchors
  and `TABLE_SDA2_79C4_C`. Everything else already existed in the module and is
  reused unchanged: `FN_8015F800`, `FN_802B9280`, `FN_802E1C40`,
  `FN_802E1AB4`, `FN_802DCBFC`, `FN_800690C0`, `FN_800DF180`, `FN_80239E30`,
  `FN_80169D90`, `FN_80064BD0`, `FN_8023A0B0`, `FN_801650D0`, `FN_800690E0`,
  `FN_80064AF0`, `FN_801C0BD0`, `FN_8016EEF0`, `FN_8016EDD0`, `FN_8016ECB0`,
  `FN_8016A170`, `FN_8016A150`, `FN_8016A1A0`, `DAT_80377C48`, the helper
  `call_3f_1u`, plus the module bodies `fn_800381b0` (called directly) and
  `fn_8003dcd0` (called directly by `fn_8003dbd0`).

- **No new host-side helper.** The one float-returning virtual (`+0x2C8`) is
  modelled with an inline `extern "C" fn(u32) -> f32` transmute, exactly as
  `fn_8001e6f0`'s neighbours already do for vtable slot `+0x29C`.

- **`functions.tsv` prologue artefact.** `0x8003CF40` is the only row whose
  `main` size (16) is not the real extent (172); see the table above. The
  `# Original` line keeps the checker's value and the prose states both.

- **`fctiwz` versus Rust's `as`.** `0x8003CF40` truncates the `f32` product with
  `fctiwz`, then sign-extends to `i16` with `extsh` and clamps at `0xA00`. The
  reconstruction uses `as i32 as i16` (Rust saturates instead of producing
  `0x80000000`), which can differ only for products outside the `i32` range -
  impossible for a rate times a 16-bit count at the observed call sites.

- **`lhax` read as `u16`.** `0x8003DBD0` loads `+0x102`'s new value with `lhax`
  (sign-extending) but stores only the low half with `sth`, so the `u16` read is
  bit-identical to the original.

- **`r12` descriptor of `0x802DCBFC`.** `0x8003DB70`/`0x8003DBA0` install
  `a + 0x18`/`a + 0x24` in `r12`, and the dispatcher really reads its slot table
  from `r12` (`FUN_802dcbfc` exports `in_r12`), so that descriptor is live. The
  host can only pass the three register operands, so the body documents the
  descriptor instead of expressing it; both rows are therefore `medium`.

- **Unknowns.** The role of the `+0x0C` virtual predicate in the child walk is
  not proven and stays unnamed; the `+0x348` kind byte only selects one of four
  halfword entries (`{12000, -12000, 0, 0}` observed at `0x8042B6BC`). The
  semantic names of the `+0x318..+0x324` / `+0xD0..+0xFC` floats are not
  established by these bodies - only their values are (`0.0f`, `8.0f`, `16.0f`,
  `6.0f`, `-4.0f`, `1.0f`).

## Verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to a scratch directory and the
block appended to the copy's `src/ui/hud.rs`; `cargo check --lib` exits `0` with
no warning or error that mentions the appended region of `src/ui/hud.rs` (the 23
pre-existing warnings all come from other modules).
