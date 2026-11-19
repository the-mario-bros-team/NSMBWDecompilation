# Coverage - wave 203, lane D (`ui::hud`, `0x800217F0..0x80022284`)

Ten functions of `src/ui/hud.rs` translated from `sys/main.dol` in wave 203,
lane D. All ten now have a faithful Rust body in
`work/wave203/laneD_block.rs` (ready to append to `src/ui/hud.rs`); nothing in
this lane is deferred.

## Scope

| address | size | name | confidence | role |
|---|---|---|---|---|
| `0x800217F0` | 276 | `fn_800217f0` | medium | small map-enemy burst (`Wm_en_burst_s`) at a slot-dependent offset |
| `0x80021A20` | 140 | `fn_80021a20` | high | effect `0x237` with a byte-packed `(first, second)` argument |
| `0x80021AB0` | 120 | `fn_80021ab0` | high | is slot `n` still owned by a live player |
| `0x80021B30` | 152 | `fn_80021b30` | high | round-robin four-slot allocator |
| `0x80021BD0` | 172 | `fn_80021bd0` | high | effect `0x14A` for one allocated slot |
| `0x80021C80` | 544 | `fn_80021c80` | medium | effect `0x14A` for `players - live` slots, then for the live slots with `0x2000` |
| `0x80021EA0` | 176 | `fn_80021ea0` | high | effect `0x14A` for one allocated slot, with `0x2000` |
| `0x80021F50` | 292 | `fn_80021f50` | medium | effect `0x14A` with `0x2000` for every player index |
| `0x80022180` | 96 | `fn_80022180` | high | is player `n` in state `1..2` or `4..6` |
| `0x800221E0` | 392 | `fn_800221e0` | medium | fill the per-player mode array at `+0x650` |

Total: 2,360 bytes, 10/10 translated. Addresses come from
`ledger/agents/wave203-laned.csv` (`module = ui::hud`).

The range is the map/world-map half of the module: the four `0x14A`/`0x237`
emitters fan one "enemy burst" out over the four player slots, and
`0x800221E0` maintains the four-word mode array that those emitters read. The
next body after this lane is `0x80022390` (`max(1, active players) - 1`), which
all four multi-slot emitters already call.

## Two size traps

The lane manifest (`%TEMP%/opencode/wave203.json`, generated from
`main-missing.json`) lists `0x80021C80` and `0x80021F50` as **20 bytes** each.
That is wrong: both are long multi-emission bodies.

- `0x80021C80` ends with `blr` at `0x80021E9C`; the next function starts at
  `0x80021EA0`, so the extent is exactly `0x220` = **544** bytes - which is what
  `work/ghidra/main-gb/export/functions.tsv` says.
- `0x80021F50` ends with `blr` at `0x80022070`; `0x80022074..0x8002207C` is
  zero padding and `0x80022080` is the next `stwu` prologue, so the extent is
  `0x124` = **292** bytes - again matching `functions.tsv`.

The `# Original` headers use the `functions.tsv` sizes (544 and 292) because
`tools/check-lane-csv.mjs` cross-checks them; the manifest's `20` is a
listing artefact, not a code fact.

## The `GQR0` / `ldexpf` prologue is a register save

Ghidra prints a bogus block at the top of `0x80021C80` and `0x80021F50`:

```c
bVar1 = (byte)unaff_GQR0 & 7;
if ((unaff_GQR0 & 0x3f00) == 0) dVar12 = 1.0; else dVar12 = ldexpf(...);
...
```

The shipped words are, in order:

```
9421ff90  stwu r1,-0x70(r1)
7c0802a6  mfspr r0,256
90010074  stw r0,0x74(r1)
39610060  addi r11,r1,0x60
dbe10060  stfd f31,0x60(r1)
f3e10068  psq_st f31,0x68(r1),0,0
482bb0c5  bl 0x802dcd5c        ; __savegprlr_21
```

`0x802DCD5C`, `0x802DCD60` and `0x802DCD74` are the compiler's
`__savegprlr_*` helpers and `0x802DCDA8`/`0x802DCDAC`/`0x802DCDC0` are the
matching `__restgprlr_*` helpers (their exports are pure register spills with
`in_r11` frame pointers). The `GQR0`/`ldexpf` text is Ghidra mis-modelling
`psq_st f31,0x68(r1),0,0` (a paired-single store of a saved `f31`, quantised by
GQR0). None of it is arithmetic, so the reconstruction omits it - the same
conclusion as `docs/SPEC_WAVE158_BASES.md:179`.

This also explains why Ghidra types `FUN_80021C80` and `FUN_80021F50` as
`void f(void)`: the save helper swallows `r3`/`r4` into an `undefined8` return,
so the `this` and `mode` operands disappear. The disassembly shows
`mr r23,r3` / `mr r24,r4` (respectively `r29`/`r30`) immediately after the
helper returns, so both bodies are `(this, mode)` methods; both are called as
`FUN_80021c80(uVar4, 2)` resp. `FUN_80021f50(uVar4, 2)` from `0x800223C0`.
`0x800221E0` has the same shape (`r3` = scene, `r4` = mode).

## Shipped data this lane reads

`node %TEMP%/opencode/rd.js work/dol/main.elf <vaddr>` (program-header
reader) gave the following, all of which are now named constants in the block:

| address | content |
|---|---|
| `0x80301E98` | `"Wm_en_burst_s"` |
| `0x802EE2C0` | `0, 2, 3, 1` - slot id per slot number |
| `0x802EE2D0` | `0, 1, 0, 1` - stored into bit 31 of the packed argument |
| `0x802EE2E0` | `0, 0, 1, 1` - stored into bits 28..30 |
| `0x8042B2B0` (`SDA2 - 0x7DD0`) | `24.0, -8.0, 16.0, -16.0` - spawn lift |
| `0x8042B2B8` (`SDA2 - 0x7DC8`) | `1500.0, -2500.0, 5500.0, 0.0` - record lift |
| `0x8042B2C0` (`SDA2 - 0x7DC0`) | `1500.0` - default third component |
| `0x8042B2C4` (`SDA2 - 0x7DBC`) | `-2500.0` - alternate third component |
| `0x8042B2C8` (`SDA2 - 0x7DB8`) | `5500.0` - spawn record third component |
| `0x8042B2E4` (`SDA2 - 0x7D9C`) | `16.0` - mode-`2` lift, used by all six emitters |
| `0x8042B2E8` (`SDA2 - 0x7D98`) | `32.0` - subtracted from `+0xB4` |
| `0x8042B2EC` (`SDA2 - 0x7D94`) | `-32.0` - replacement third component |
| `0x80429D48` (`SDA - 0x5938`) | BSS: slot-owner pointer, cursor at `+0xBC8` |

Two subtleties of `0x800217F0` that the Ghidra listing hides:

- the two lift tables are **both** added to `+0xB0`, and the second one
  (`1500/-2500/5500`) is added to the *unmodified* `+0xB0`, so the record vector
  does not accumulate the spawn lift;
- `0x80064610` receives `&local_14` (the record vector), not `&local_20` (the
  spawn vector), even though the spawn vector is the one given to `0x8016C890`.

The index into both tables is `b = (u8[+0x68D] - 2) as i8`, clamped to `1` when
negative; note that `+0x68D == 2` still yields `b == 0`, so there is no lower
clamp at zero, only at negative.

## Reused module helpers

`FN_8006A4B0`, `FN_80198C30` and `FN_8005F900` already exist in `src/ui/hud.rs`
(lines 9782, 9784 and 8589), so the block reuses them instead of redefining
them. Everything else in the block is new and namespaced for this lane
(`S_WM_EN_BURST_S`, `TABLE_SLOT_IDS`, `TABLE_BURST_BIT_31`,
`TABLE_BURST_BIT_28`, `TABLE_BURST_SPAWN_LIFT`, `TABLE_BURST_RECORD_LIFT`,
`F32_SDA2_BURST_Z`, `F32_SDA2_BURST_Z_ALT`, `F32_SDA2_BURST_SPAWN_Z`,
`F32_SDA2_MODE_LIFT`, `F32_SDA2_TRIGGER_DELTA`, `F32_SDA2_TRIGGER_Z`,
`G_SDA_SLOT_OWNER`, `FN_8005FB70`, `FN_8005FB00`, `FN_8005FEF0`, `FN_80056E30`,
`FN_80022390`, `FN_80064610`, `FN_8016C890`). A `grep` for all of them and for
`fn_80021*`/`fn_80022*` in `src/ui/hud.rs` before writing returned zero hits.

## Untranslated-looking operands kept deliberate

- `fn_80021ab0` and `fn_80022180` ignore their `r3` operand. The original really
  does (no read of `r3` before the first `bl`), so the Rust bodies keep the
  parameter for ABI parity and drop it with `let _ = this;`.
- `fn_80021b30` writes the cursor twice on the success path (slot, then
  `(slot + 1) & 3`). The block reproduces both stores; the returned value is
  the pre-increment slot still held in `r31`.
- `fn_80021c80` calls `0x8005FB00` three times; the block mirrors all three
  calls rather than caching the count.
- `fn_80021c80`'s second loop indexes `TABLE_BURST_BIT_31/BIT_28` from
  `last + 1`, i.e. past the fourth entry when the first loop consumed all four.
  The block keeps that read; it is what the original does, and the tables are
  followed by other `.data` words.

## Verification

`work/wave203/laneD_block.rs` was appended to a temporary copy of the crate
(`%TEMP%/opencode/laneD_verify`, `Cargo.toml` + `src/`) and
`cargo check --lib` exited **0** (17.4 s, 23 pre-existing warnings, none in the
appended range; a `grep` of the warning output for `fn_8002*`, `TABLE_BURST*`,
`F32_SDA2_*`, `G_SDA_SLOT*`, `S_WM*` and `FN_8005/8006/8016*` matched nothing).

Ledger: `ledger/agents/wave203-laned.csv`, header + 10 rows, addresses as plain
lowercase 8-digit hex so that `node tools/check-lane-csv.mjs wave203` accepts
them. After this lane was written the only remaining problems the checker
reports for `wave203-laned.csv` are the ten "has no body and no `# Original`
reference in `src/ui/hud.rs`" lines, which disappear once the coordinator
appends the block.

**Coordinator note:** `wave203-lanea.csv` and `wave203-lanef.csv` use `0x` +
uppercase addresses, so `tools/check-lane-csv.mjs wave203` rejects all twenty of
their rows (`address "0x..." is not plain 8-digit lowercase hex`) and the run
reports `50 rows / 30 unique addresses`. Those two files need the same
lowercase-plain rewrite before the wave gate can go green.

## Evidence

- Disassembly: `work/dol/main.elf` via `ppc159.js`; `LEN` 80..140 at
  `0x800217F0`, `0x80021A20`, `0x80021AB0`, `0x80021BD0`, `0x80021D0C`,
  `0x80021EA0`, `0x80021FDC`, `0x80022180`, `0x800222BC`, `0x80021E84`;
  `STOP=0` throughout.
- Decompilation: `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` (lowercase)
  for all ten addresses; `work/ghidra/main/export/` was not needed.
- Callee bodies consulted: `8006a4b0` (3D -> 2D projection), `80198c30`,
  `80064610`, `8005f900`, `8005fb00`, `8005fb70`, `80056e30`, `8005fef0`,
  `802dcd5c/60/74/a8/ac/c0`.
- Shipped data: `%TEMP%/opencode/rd.js` over the ELF program headers (table
  above).

## Files

- Generated block: `work/wave203/laneD_block.rs`
- Ledger: `ledger/agents/wave203-laned.csv`
- This document: `docs/COVERAGE_WAVE203_LANED.md`
- `src/ui/hud.rs` was **not** modified (coordinator merges).
