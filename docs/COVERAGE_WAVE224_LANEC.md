# Wave 224, lane C - `src/game/stage.rs`, addresses `0x800A7540..0x800AA4F0`

Ten entries of `work/ghidra/main-gb/export/functions.tsv` in the `0x800A0000`
band of the `game::stage` ledger module. All ten are translated; none deferred.
Bytes translated: **664**.

| address | tsv size | real extent | body |
|---|---|---|---|
| `0x800A7540` | 12 | 12 (`b 0x800A7560` at `0x800A7548`) | queue thunk: narrow `r7`, `r8 = 0`, tail-branch |
| `0x800A7560` | 220 | 220 (`blr` at `0x800A7638`) | build the queue record and submit it twice |
| `0x800A8710` | 12 | 12 (`blr` at `0x800A8718`) | return the shared type-info record |
| `0x800A8760` | 12 | 12 (`blr` at `0x800A8768`) | same getter, second alias |
| `0x800A87D0` | 12 | 12 (`blr` at `0x800A87D8`) | same getter, third alias |
| `0x800A8820` | 12 | 12 (`blr` at `0x800A8828`) | same getter, fourth alias |
| `0x800A8890` | 12 | 12 (`blr` at `0x800A8898`) | same getter, fifth alias |
| `0x800A9FC0` | 188 | 188 (`blr` at `0x800AA078`) | ground-gap ratio with `1.0f`/`-1.0f` sentinels |
| `0x800AA220` | 120 | 120 (`blr` at `0x800AA294`) | actor accepts the record's source |
| `0x800AA4F0` | 64 | 64 (`blr` at `0x800AA52C`) | release through `+0x4D8`, clear `+0x100` |

No `functions.tsv` size in this lane is a prologue artefact: each was checked
against the prologue and the closing `blr` (or, for `0x800A7540`, the
unconditional tail branch) in `work/dol/main.elf`, and in every case the body
ends exactly at `address + tsv_size`. What follows is four-byte `0x00000000`
padding, except after `0x800AA220` (eight bytes, before `0x800AA2A0`) and after
`0x800AA4F0`, where the next word `0x4E800020` is a separate untracked
four-byte stub at `0x800AA530`.

## What the run is

Six of the ten are byte-level decodes of naked instruction streams:

* `0x800A7540` is `extsb r7,r7` / `li r8,0` / `b 0x800A7560`
  (`0x7CE70774` / `0x39000000` / `0x48000018`) - a one-call tail wrapper that
  narrows the caller's route byte and supplies the sixth argument `0`.
* The five getters at `0x800A8710`, `0x800A8760`, `0x800A87D0`, `0x800A8820` and
  `0x800A8890` are all `lis r3,0x8037` / `addi r3,r3,0x7A04` / `bclr 20,0`
  (`0x3C608037` / `0x38637A04` / `0x4E800020`), i.e. the same
  `return &DAT_80377a04;` stub the file already carries fifteen times. They
  reuse [`W84_TYPE_INFO`].

`0x800A7560` is the worker behind the thunk and the reason the pair exists: the
one existing caller (`0x80150540`) passes `(owner, vector, player, mirror,
route)` in `r3..r7`, and the thunk only fills in `r8`. The body's real content is
that **two different vectors are published**: the mapped pair at `r1+8`
(`0x8006A4B0` wraps x through the BG scene and rescales) goes to the effect
manager as id `0x25D`, while the raw triple at `r1+0x10` goes to `0x80064610`
as id `0x14A` together with the packed word
`variant << 16 | category << 20 | 0x802F0888[player] | mirror << 4 | 0x1000`.
The `-1` category byte means `15`.

`0x800A9FC0` is the only body in the lane with floating-point arithmetic, and it
is single precision throughout: `fadds`/`fsubs`/`fabs`/`frsp`/`fdivs` plus
`fmr f1,f30` in the tail. Ghidra prints it as `double` only because of the four
`psq` spills, all of which are `qr0` `f30`/`f31` save/restore pairs
(`psq_st f31,0x28(r1),0,0` at `0x800A9FD0`, etc.) - plain float movement, no GQR
quantisation. That is the same conclusion `docs/ANALYSIS_WAVE158_BASES.md` item 3
records for the rest of the image.

`0x800AA220` and `0x800AA4F0` are the `+0x60`-vtable pair: the first forwards
`(actor, this + 4, 0)` into slot `+0x3F4` once the `+0x38C` byte is `1` and slot
`+0x3F0` answers `0`; the second writes `0` to the 16-bit word at the two
hundred-and-fifty-sixth byte.

## New constants (lane C)

| name | value | role |
|---|---|---|
| `FN_8006A4B0_C` | `0x8006A4B0` | scene wrap/scale of the two-float pair |
| `FN_80198C30_C` | `0x80198C30` | positional effect post (`0x25D`) |
| `FN_80064610_C` | `0x80064610` | stage-effect submit (`0x14A`) |
| `FN_800B30C0_C` | `0x800B30C0` | scene clock probe of `0x800A9FC0` |
| `FN_800AA090_C` | `0x800AA090` | the per-player height test |
| `W224_F_MIRROR_OFFSET_C` | `r2-0x6CB0` = `0x8042C3D0` | `4.0f`/`0.0f` x offset pair |
| `W224_PLAYER_SOUND_TABLE_C` | `0x802F0888` | per-player word OR-ed into the parameter word |
| `W224_F_PROBE_BIAS_C` | `r2-0x6C60` = `0x8042C420` | `4.0f`, added to the slot `+0x4E0` answer |
| `W224_F_RESULT_DEFAULT_C` | `r2-0x6C70` = `0x8042C410` | `1.0f`, the default result |
| `W224_F_RESULT_LIMIT_C` | `r2-0x6C58` = `0x8042C428` | `16.0f`, the divide threshold |
| `W224_F_RESULT_FALLBACK_C` | `r2-0x6C5C` = `0x8042C424` | `-1.0f`, the "no gap" result |

Existing helpers reused instead of redefined: `W84_TYPE_INFO` (`0x80377A04`),
`W23_G_EFFECT_MANAGER` (`r13-0x5920`), `platform::call1`/`call2`/`call3`, the
module's own `call4`/`call5`, `call1_f32`, `call0_f32`, `read_f32`, `read_u8`,
`read_u32`, `write_u16`.

One helper had to be added: `w224c_call_f32_u32(addr, r3, f1)`, because
`0x800AA090` takes its float argument in `f1` and returns in `r3`, a shape the
file did not have yet (`call1_f32` is the mirror case, integer in and float out).

## Evidence notes

* Every byte count was confirmed against `work/dol/main.elf`, not the export:
  `0x800A7540`'s tail branch and the two 12-byte getter bodies are invisible in
  their exports (the thunk prints as a bare `FUN_800a7560();`).
* The `r2` constants were read from the image: `0x8042C3D0 = 4.0f`,
  `0x8042C3D4 = 0.0f`, `0x8042C410 = 1.0f`, `0x8042C420 = 4.0f`,
  `0x8042C424 = -1.0f`, `0x8042C428 = 16.0f`. `0x802F0888[0..3]` is
  `0, 3, 1, 2`.
* `0x800A7560`'s export prints the source vector as `extraout_r4`; the raw
  listing shows it is the incoming `r4` (`lfs r3,0x0(r4)` / `lfs r1,0x8(r4)` /
  `lfs r2,0x4(r4)` at `0x800A759C..0x800A75AC`). The same export prints the
  fifth argument as `char`, which matches the `extsb` pair in the thunk and the
  worker.
* `0x800A9FC0`'s compare is `fcmpu cr0,f1,f0` followed by `bc 4,0` - "branch if
  not less than" - so the division runs when the gap is `>= 16.0f`. The body
  keeps that sense as `if !(gap < limit)` rather than `>=` so a NaN keeps the
  original path.
* The `0x800AA090` call in `0x800A9FC0` is issued with `r3 = this` and `f1 =
  4.0f + slot(+0x4E0)`; its result is tested with the unsigned `cmpi cr0,3`
  against `0`, so only exactly `0` skips the gap computation.
* `0x800AA4F0`'s clear is `sth`, not `stb`: the field at `+0x100` is a 16-bit
  word (`b01f0100` at `0x800AA518`).
