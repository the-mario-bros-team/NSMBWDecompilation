# Coverage - Wave 173 (`game::bases` 352-364 byte sub-band)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 19 | 6,808 | 14 | 5 |

Second five-lane wave on the paid model; all 19 bodies are in the 352-364 byte
sub-band.

## Lane A (4 bodies, 1,408 bytes)

- `0x80638960` - four-panel score-screen tick, cue `154`, `+0x7E8` clamp; the
  all-zero tail fires `0x80826D60` on `+0x6A8`. **medium**.
- `0x80753B80` - `daWaterMove_c` destructor; returns `this`.
- `0x8076EAA0` - ship-actor update on `*(this+4) & 0xF`. **medium**, because
  `bc 4,1` at `0x8076EAD4` means "not greater", so kinds `3`/`4` take the
  moving arm and kinds `>4` take no arm.
- `0x8079AE00` - course-route weight scan; writes `1.0` or the main.dol
  `0x8031D5B8` float. **medium**.

## Lane B (4 bodies, 1,424 bytes)

- `0x806F0DB0` - `manjirou` lift: two four-player countdown arrays at
  `+0x95C`/`+0x960` and the `0x5C`-byte wheel lists; re-arms `8`.
- `0x8071F120` - three `0x60`-byte collision records at `+0x434`/`+0x494`/
  `+0x4F4`, byte `10` at `+0x47`, ids `0x20D80`/`0x20900`/`0x20480`.
- `0x80774A60` - name-matched fallback spawn over the eight `0x807CF418`
  entries; sets bit `0x10000` for `cobRotary`.
- `0x807BAF50` - world-map zone table build (nine worlds, `0x16518` objects).

## Lane C (4 bodies, 1,440 bytes)

- `0x8064F510` / `0x8064F820` - the `+0x5BC`/`+0x584` camera hand-off twins
  (`0x80828338` / `0x808282F8`), completing the `fn_8064f390`/`fn_8064f6a0`
  family. Both pass the live `f1` to `0x801680E0`.
- `0x80693BE0` - `+0x4 & 0xF` death handler (explosion / bombheibreak, sounds
  `0x1CC` / `0x26C`).
- `0x806C97D0` - recursive subtree id collector; `X31_954` at `0x806C989C` is
  an `extsb` before the unsigned `< 4` test.

## Lane D (4 bodies, 1,444 bytes)

- `0x80699CC0` - swept-box hit test with the `20.0` scale and the
  `36.0`/`24.0 * 0.5` window.
- `0x8074C3D0` - player-pane portrait placement, sprite `159`.
- `0x8074DBA0` - horizontal-sweep sprite `388`; **medium** because the depth
  arm depends on the `0x80429D40 +0x44` ceiling and a `128.0` X window.
- `0x8079F790` - world-map pane republish over the four `+0x384` state words.

## Lane E (3 bodies, 1,092 bytes)

- `0x806D2390` - `remo_lift` see-saw rider sync with the `0x807CB540` slot
  mask.
- `0x80742470` - four-slot marker spawner delegating to `fn_807422d0` /
  `fn_807422f0` / `fn_80742340` / `fn_807423e0`.
- `0x8076FF80` - `cobKoopaShip` child transform with suffix strip, `0x802E2108`
  parse and the `+0x48C` double store. **medium**.

## Corrections and cross-checks

- Ghidra's `_savegpr` model swaps `r3`/`r4` in `0x80742470` and `0x8076FF80`;
  the in-REL callers `0x80742240` and `0x8076EE24` confirm the real argument
  counts.
- Ghidra's `_savegpr` also hides the second argument of `0x806F0DB0`
  (`_savegpr_24` preserves `r3` **and** `r4`), so it is `(this, out)`.
- `0x8071F120`'s `qr3` block made Ghidra invent a `cos()` argument; the raw
  words compute and discard the cosine, and the third record float is the
  incoming `f1`, with `-192.0`/`8.0` coming from `fadds` doublings of
  `-96.0`/`4.0`.
- `0x8076FF80`'s name argument is a load of `0x80816BA0` (→ `0x80816B90`,
  `"cobKoopaShip"`), not the address of the pointer variable.
- `0x80770C50` takes its owner in `r3` of its own `_savegpr_27` prologue, so
  `0x8076EAA0` calls it as `abi_call1!(0x8077_0C50, this)`.
- `0x8031D5B8` lives in `main.dol`, so `0x8079AE00` reads it at runtime.

## Verification

- `cargo check` succeeds with 4,189 public bodies and no duplicate names.
- `node tools/check-lane-csv.mjs wave173`: 19 rows, 19 unique addresses,
  14 high + 5 medium.
- All 19 evidence exports exist and every size matches `functions.tsv`
  (6,808 bytes total).
