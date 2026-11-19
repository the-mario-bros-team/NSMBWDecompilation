# Coverage - Wave 214, lane C (`game::player`, the `0x8006CED0..0x8006D434` actor sub-record)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 9 | 1296 | 8 | 1 | 0 |

Status split: 9 `translated`, 0 `partial`, 0 deferred.

Body block: `work/wave214/laneC_block.rs`. Ledger rows:
`ledger/agents/wave214-laneC.csv`.

Scratch verification: the block was appended to a copy of `src/game/player.rs`
in a scratch crate (with `Cargo.toml`/`Cargo.lock`) and
`cargo check --lib` exits `0`; the block adds no warnings.

## How to read this file

* **Address** - virtual entry point in the DOL image, not a file offset.
* **Name** - the Ghidra name of the body, which the Rust function keeps as
  `fn_<address>`; every new constant in the block is prefixed `W214C_`.
* **Size** - the `functions.tsv` value. Seven of the nine were re-derived from
  the prologue and the closing `blr` and match exactly. `0x8006CFA0` (60 B
  real vs 64) and `0x8006D090` (140 B real vs 144) end 4 bytes before the next
  aligned slot; both are ordinary frame functions with a real epilogue, not
  16/20-byte prologue artefacts. `0x8006CFE0` is the one case where the tsv's
  172 is genuine but covers **two** prologued halves (see below).
* **Confidence** - strength of the evidence: `high` means every address, field
  offset, immediate and branch is literal in the disassembly. `0x8006D170` is
  `medium` only for its two coordinate-conversion idioms.
* `r13 = 0x8042F680` and `r2 = 0x80433080` throughout, so the small-data
  accesses read as `r13 - 0x58B0` = `0x80429DD0` (tile/scene object),
  `r13 - 0x5920` = `0x80429D60` (sound object), `r2 - 0x7440` = `0x8042BC40`
  (`0.0f`), `r2 - 0x743C` = `0x8042BC44` (`8.0f`) and `r2 - 0x7438` =
  `0x8042BC48` (`0x43300000_80000000` = `2^52 + 2^31`, the signed
  int-to-double bias).

## What this pass covers

The run continues wave 213 lane F's `Base_c` family but belongs to a different
class: one **embedded actor sub-record**. The actor constructor at
`0x80064110` builds it in place at `actor + 0x1EC` (right after the
`0x8008C200` block at `+0x148`), and the next sub-object `0x800D9860` starts
at `+0x2DC`, so the record is `0xF0` bytes:

| offset | meaning (`// inferred:`) |
|---|---|
| `+0x00` | vtable `0x8030F3D0`; slot `+0x08` is `0x8006CFA0` (delete) |
| `+0x04` | owner actor pointer (set by `0x8006D090`) |
| `+0x08..+0x10` | three words copied from the caller's table |
| `+0x14` / `+0x18` / `+0x1C` | owner `+0xAC` / `+0xB8` / `+0xE8` (position, angle, sub-object) |
| `+0x20..+0x28` | three `f32` cleared by the initialiser |
| `+0x50` | word cleared by the constructor |
| `+0x88` | word cleared by the initialiser |
| `+0x98` / `+0x99` | two layer indices, both `0xFF` until a bind |
| `+0xA4`, `+0xAC`, `+0xB8`, `+0xB9` | flag bytes cleared by the initialiser |
| `+0xBD`/`+0xBE`, `+0xC0`/`+0xC2` | two `{10, angle}` pairs (`0x09FB`, `0xF605`) |
| `+0xC8` | `u16` zero |
| `+0xE0` / `+0xE1` | `0` / `0xFF` (via `0x8006D440`) |
| `+0xE2` | `2` |
| `+0xE5` | `1` |
| `+0xE8` | data pointer: own `+0xEC` until bound, then `owner + 0x38F` |
| `+0xEC` | game-layer / tile value (read by the tile writers) |

The bind callers are the three `0x8002747x` sites, which pass members of the
table at `0x802EE450` as the three words, and the geometry users are the four
`0x8006D170` call sites that build a point record (`0x8006E098`,
`0x8006F4A0`, `0x800701F8`, `0x80075030`).

### The initialiser `0x8006CFE0` (172 B, high)

The tsv's 172 spans `0x8006CFE0..0x8006D088`, but the body is two prologued
halves. The first (`0x8006CFE0..0x8006CFFC`, seven stores, no frame) ends in
`b 0x8006D000`; `0x8006D000` opens a fresh 16-byte frame and returns straight
to the original caller. A full scan of every `b`/`bl` word in `.text` finds
exactly one reference to `0x8006D000` - that `b` - so the two halves are one
entry point (and one tsv row), not an accidental merge of two functions. The
Rust body keeps the original store order and marks the seam with a comment.

The two angle halfwords are read with `lha` from `0x802EF55C`/`0x802EF584`
and written with `sth`, so the sign extension is truncated again; the Rust
code copies the raw halfwords (`0x09FB`, `0xF605`) and says so.

### The delete slot `0x8006CFA0` (60 B real, high)

`(this, mode)`: for `this != 0 && 0 < mode` one `0x802B9280(this)` archive
heap free, then `this` is returned (dangling after a free, as in the image).
The slot is confirmed by dumping `0x8030F3D0`: `+0x08` holds this address,
which is the position both wave-213 vtables give their destructor.

### The bind `0x8006D090` (140 B real, high)

Stores `owner`, the three words, `this + 0xEC` at `+0xE8`; then, when the
owner is non-zero, the three owner-relative pointers, one indirect call
through `owner->vtable[0x6C]` whose result byte is copied to `+0x98`, and
`+0xE8 = owner + 0x38F`.

### The step helper `0x8006D120` (56 B, high)

`(limit: f64, value: *mut f32)`: returns `1` when `*value >= limit`
(`fcmpo` + `cror 2,1,2` + `bne`, the compiler's ordered `>=` fold), otherwise
adds `8.0f` (`0x8042BC44`), clamps to `limit`, and returns `0`.

### The bit leaf `0x8006D160` (8 B, high)

Raw word `0x5463FFFE` = `rlwinm r3,r3,31,31,31` = `(value >> 1) & 1`.

### The dispatcher `0x8006D170` (576 B, medium)

Guards: `0x80070530(point[1], this)` must not return `-1` (it reads
`this+0x98`/`+0x99`, the same two bytes this record carries), and bit 1 of
`param_2` must be set (`0x8006D160`).

* flag `0x40000`: `kind == 2` returns `1` untouched; otherwise the point is
  copied to a 12-byte local, `0x80060250(slot)` and `0x80060690(100, slot)`
  run, and the sparkle at the copy is removed (`0x80094390` for kind `4`,
  `0x80094370` otherwise). Then `{x + 8, y - 8, 0}` is built from the
  cell-snapped coordinates, projected by `0x8006A4B0`, and sound `0x237` is
  queued through `0x80198C30` with `0x80069530(slot)`. With `param_6 != 0`
  the cell is also published through `0x80077860` with `this[0xEC]` as the
  kind.
* flag `0x20000`: `kind == 2` returns `1`; otherwise the 10-byte record
  `{x & 0xFFF0, (-y) & 0xFFF0, kind, slot}` is handed to `0x80088B30`
  (already translated as `crate::game::stage::stage_actor_mirror_effect`,
  which reads exactly those four fields).
* neither bit: `0`.

`medium` is only about the two coordinate idioms. `fctiwz` + `extsh` +
`xoris 0x8000` + the `0x43300000_80000000` double is the compiler's **signed**
int-to-float conversion, so the sparkle path computes
`((int)point & 0xFFF0) as i16 as f32`; the Rust `as i32` saturates where
`fctiwz` would return `0x80000000`, which only differs for out-of-range
inputs. Every flag test, mask, store and callee register is literal.

### The tile writer `0x8006D3B0` (136 B, high)

Guards: `param_2 == 0` (the first cell probe `0x80070BA0` found no
tile), `(u8)(kind + 0xD8) <= 2` (the second probe `0x80070BF0` is
`0x28..=0x2A` modulo 256, via the `addi`/`rlwinm` wrap) and bit `0x80000` of
`param_4`. Then one `0x80078250(tile_object, kind, x & 0xFFF0,
(-y) & 0xFFF0, this[0xEC])` and `1`; otherwise `0`.

## Deferrals

None. All nine bodies were translated; no body needed to be guessed. The only
documented uncertainty is the meaning (not the arithmetic) of a few field
roles, marked `// inferred:` in the doc comments and argued from the call
sites in the prose above.
