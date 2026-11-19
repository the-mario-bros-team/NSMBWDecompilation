# Coverage - Wave 213, lane C (`game::player`, the `0x8006A200..0x8006A4A4` music-state cluster)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 10 | 596 | 10 | 0 | 0 |

Status split: 10 `translated`, 0 `partial`, 0 deferred.

Body block: `work/wave213/laneC_block.rs`. Ledger rows:
`ledger/agents/wave213-laneC.csv`.

## How to read this file

* **Address** - virtual entry point in the DOL image, not a file offset.
* **Name** - the Ghidra name (`FUN_<address>`). No symbol table survives, so the
  Rust body keeps the `fn_<address>` name; every new constant in the block is
  prefixed `W213C_` (this lane's letter is `C`).
* **Size** - the `functions.tsv` value, which is also the real extent. All ten
  were re-derived from the prologue and the closing `blr`; none is a 16/20-byte
  prologue artefact (six of them have no prologue at all - they are one
  instruction plus `blr`, except `0x8006A200`, which is five plus `blr`).
* **Confidence** - strength of the evidence, not code quality. All ten are
  `high`: every address, offset, mask and branch is literal. Where a *role* is
  guessed rather than demonstrated, the doc block marks it `// inferred:` (the
  two opaque accessor pairs `0x8006A240/0x8006A250` and
  `0x8006A260/0x8006A270`, and the `+0x0C` UI vtable slot of `0x8006A320`).
* All ten are main.dol bodies in `r13`/`r2` small-data code: the decompiler never
  saw the SDA base loads, so each export prints `*(... *)(unaff_r13 + -0xNNNN)`
  instead of an address. The bases this module already fixes are
  `r13 = 0x8042F680` and `r2 = 0x80433080`.

## SDA map used by the lane

| global | bias | role |
|---|---|---|
| `0x8042792E` | `r13 - 0x7D52` | `u16` music/sequence id (reset to `120`) |
| `0x80427930` | `r13 - 0x7D50` | `u32` companion word, only this accessor pair |
| `0x80427934` | `r13 - 0x7D4C` | `u16` last committed request (`0x8006A620`) |
| `0x8042793C` | `r13 - 0x7D44` | `u16` latched request (`0x8006A2B0`) |
| `0x80429D6A` | `r13 - 0x5916` | `u8` flag byte, bitmask-tested |
| `0x80429D6C` | `r13 - 0x5914` | `u32` third state word |
| `0x80429D70` | `r13 - 0x5910` | `u8` "sound driver started" latch |
| `0x80429D7C` | `r13 - 0x5904` | `u8` "controller enabled" latch |
| `0x80429E00` | `r13 - 0x5880` | scene rectangle object (`W22_SCENE_80429E00`) |
| `0x8042A298` | `r13 - 0x53E8` | object probed by `0x80107850` |
| `0x8042A488` | `r13 - 0x51F8` | UI/event table (`W18_UI_8042A488`) |
| `0x8042A4A8` | `r13 - 0x51D8` | game-sound object (`W74_SOUND_8042A4A8`) |

Image constants: `0.5f` at `0x8042BBFC`, `2.0f` at `0x8042BC00` (both read from
`work/dol/main.elf`), and the stage-mode word `0x80315B98` (already
`W98_BONUS_MODE_80315B98`).

## What this pass covers

### The four accessor pairs and the mask test

- **`0x8006A200` (24 B, high).** Six instructions: `lbz r0,0xffffa6ea(r13)`
  (the `0x80429D6A` flag byte), `and r3,r0,r3` (the incoming mask),
  `neg`/`or`/`rlwinm r3,r0,1,31,31` - the compiler's canonical `int != 0`
  expansion, so the body is `(flag_byte & mask) != 0` and returns `0`/`1` in
  `r3`. Ghidra prints the same thing as `-(flag & param) >> 0x1f`.
- **`0x8006A230` / `0x8006A240` / `0x8006A250` / `0x8006A260` / `0x8006A270`
  (8 B each, high).** One `lhz`/`stw`/`lwz` plus `blr`. The `u16` at
  `0x8042792E` is the music id: `0x8006A110` stores `120`, `0x8006A130` stores
  `0x80196690(sound)` and falls back to `120` when it returns `0`. The two
  `u32` pairs (`0x80427930`, `0x80429D6C`) have no other exported reader or
  writer, so their roles stay `// inferred:`.

### The enable path

- **`0x8006A280` (44 B, high).** `0x8006A6A0(0)` - which runs
  `0x8019B660(sound, 0)` and stores `0` at `0x80427938` - then `li r0,1` +
  `stb r0,0xffffa6fc(r13)` latches `0x80429D7C`. `0x800E4820` is a bare thunk to
  `0x8006A320`, so `0x80429D7C` is effectively the per-frame on/off switch.

### The request filter

- **`0x8006A2B0` (112 B, high).** Always calls `0x8006A110` first (`id = 120`,
  three flag bytes `0`), then applies three **unsigned** gates
  (`cmpli` + `bc 4,1`): accept when `request - 8 <= 3` (i.e. `8..=11`) or
  `request - 3 <= 1` (i.e. `3..=4`); otherwise `request == 5` is accepted only
  when `0x80315B98 != 3` (`lwz r0,8(0x80315B90)` + `cmpi 3` + `beq` to the
  epilogue), and `request == 0x2C7` is accepted unconditionally. Accepted
  requests are stored with `sth r31,0xffff82bc(r13)` (low halfword, so the
  parameter is modelled `u32` and truncated).

### The per-frame update

- **`0x8006A320` (196 B, high).** Returns while `0x80429D7C == 0`. When the
  latched request at `0x8042793C` is not the idle value `0x2EE`, it tears the
  sound object down with `0x8019B560(sound)`, and if the committed request
  (`0x80427934`) is `5` and the third state word (`0x80429D6C`, read through
  `0x8006A270`) is `0`, it calls `0x8019BC50(sound, 1)` and `0x80060700()`;
  the latched request then becomes `0x2EE`. It then probes
  `0x80107850(*(u32 *)0x8042A298)` (a zero-extended byte at `+4`) and drives
  the `0x80429D70` latch on each edge through `0x80069590` (stop,
  `0x8019CEF0`) or `0x80069580` (start, `0x8019CE70`). Finally it dispatches
  `(*(*(u32 *)0x8042A488)[3])(table)` - `lwz r3` / `lwz r12,0(r3)` /
  `lwz r12,0xC(r12)` / `mtspr CTR` / `bcctr` with `r3` untouched, so the table
  pointer is the single argument.

### The scene projection

- **`0x8006A3F0` (180 B, high).** With `scene = *(u32 *)0x80429E00`,
  `edge = 0x80082240(x, scene)` (left edge including the callee's wrap
  adjustment), `cx = edge + 0.5f * scene[0x40]`, `cy = 0x800B30E0()` =
  `scene[0x3C] - 0.5f * scene[0x44]`:

  ```text
  out[0] = 2.0f * (x - cx) / scene[0x40]
  out[1] = 2.0f * (y - cy) / scene[0x44]
  ```

  so `out` spans `-1..=1` across the rectangle. The export's `ldexpf`,
  `unaff_GQR0` and `double` clutter is the decompiler failing on the
  paired-single spill `psq_st f31,0x38(r1),0,0`; the disassembly shows
  `lfs`/`fmuls`/`fadds`/`fsubs`/`fdivs`/`stfs` throughout, so the lane models
  `0x80082240` as `(f32, u32) -> f32` and `0x800B30E0` as `() -> f32` with two
  local `extern "C"` thunks instead of reusing the wave-98 `f64` helper, which
  was written for the same address but mis-modelled its precision.

## Verification

Scratch crate: `src/`, `Cargo.toml` and `Cargo.lock` copied to
`C:\Users\WitherULTRA7\AppData\Local\Temp\opencode\w213-laneC`, this lane's
block appended to the copy's `src/game/player.rs`, `cargo check --lib` exits
`0` (`Finished dev profile`) with the same 23 pre-existing warnings as the base
tree and no diagnostic pointing at the appended region.

Every size in this doc was re-derived from the raw instruction stream with
`node $env:TEMP\opencode\ppc159.js work/dol/main.elf <address>`:

| address | tsv | prologue | `blr` at | real extent |
|---|---|---|---|---|
| `0x8006A200` | 24 | none | `0x8006A214` | `0x18` = 24 |
| `0x8006A230` | 8 | none | `0x8006A234` | `0x08` = 8 |
| `0x8006A240` | 8 | none | `0x8006A244` | `0x08` = 8 |
| `0x8006A250` | 8 | none | `0x8006A254` | `0x08` = 8 |
| `0x8006A260` | 8 | none | `0x8006A264` | `0x08` = 8 |
| `0x8006A270` | 8 | none | `0x8006A274` | `0x08` = 8 |
| `0x8006A280` | 44 | 16 | `0x8006A2A8` | `0x2C` = 44 |
| `0x8006A2B0` | 112 | 20 | `0x8006A31C` | `0x70` = 112 |
| `0x8006A320` | 196 | 16 | `0x8006A3E0` | `0xC4` = 196 |
| `0x8006A3F0` | 180 | 64 | `0x8006A4A0` | `0xB4` = 180 |
