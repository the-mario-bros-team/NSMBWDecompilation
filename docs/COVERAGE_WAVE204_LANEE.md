# Coverage - wave 204, lane E (`ui::hud`, `0x80028B60..0x8002A830`)

Ten functions of `src/ui/hud.rs` translated from `sys/main.dol` in wave 204,
lane E. All ten have a faithful Rust body in `work/wave204/laneE_block.rs`
(appended to `src/ui/hud.rs` on merge); nothing in this lane is deferred.

## Scope

| address | size | name | confidence | role |
|---|---|---|---|---|
| `0x80028B60` | 560 | `fn_80028b60` | medium | build the fire-door pair (`dokan_fire_neck` + `dokan_fire_attack`) |
| `0x80028D90` | 556 | `fn_80028d90` | medium | build the plain-door pair (both records `dokan_attack`) |
| `0x800295A0` | 96 | `fn_800295a0` | high | step the `+0x74C` fade toward `0x802EE4F0[index]` by `3.0` |
| `0x80029600` | 100 | `fn_80029600` | high | spawn effect actor `0x6A` at `+0x5DC` with the packed mask |
| `0x8002A740` | 88 | `fn_8002a740` | high | deleting-destructor wrapper (destroy, free when `> 0`) |
| `0x8002A7A0` | 92 | `fn_8002a7a0` | high | same wrapper with one dead duplicate null test |
| `0x8002A800` | 12 | `fn_8002a800` | high | accessor of the static record `0x8035334C` |
| `0x8002A810` | 12 | `fn_8002a810` | high | accessor of the static record `0x8035338C` |
| `0x8002A820` | 12 | `fn_8002a820` | high | accessor of the static record `0x8035330C` |
| `0x8002A830` | 12 | `fn_8002a830` | high | accessor of the static record `0x803532CC` |

Total: 1,540 bytes, 10/10 translated, 0 deferred.

## What the cluster is

`0x80028B60..` is the animation half of the door (`dokan`) transition actor,
the same object family as the `wipeDokan` builders of wave 96
(`fn_8001bb90`/`fn_8001bc60` in this file). Every body walks the same fields:

| offset | type | role |
|---|---|---|
| `+0x348` | `u8` | owner/player byte, shifted into the effect mask |
| `+0x540` | `u32` | layout/archive handle |
| `+0x544` | record | primary model record |
| `+0x584` | record | secondary model record |
| `+0x588` | `u32` | polymorphic member of the secondary record |
| `+0x5DC..+0x5E4` | `f32 x3` | effect spawn position |
| `+0x6A8` | `u32` | effect mask low word |
| `+0x6E0` | record | primary animation group |
| `+0x6E4` | `u32` | polymorphic member of the primary group |
| `+0x718` | record | two-slot transform (slots `0`, `1`) |
| `+0x748` | `u32` | effect variant, shifted left by four |
| `+0x74C` | `f32` | fade/alpha value stepped by `0x800295A0` |

The two builders repeat the `fn_8001bc60` idiom: `0x80239E30(+0x540, name)`
resolves the model, `0x8023A0B0(+0x540, animation)` resolves an animation,
`0x801651F0(group, +0x544, &animation, 1)` binds it, virtual `+0x34` resets the
group, virtual `+0x50` programmes `(slot, flag)` pairs, `0x80168FE0` writes the
group alpha at `group + 0x30`, `0x801656C0` attaches both groups to the two
transform slots and `0x80169F80` applies the transform to the model.

## Corrections and recoveries

1. **Manifest sizes.** The wave manifest lists `20` bytes for `0x80028B60` and
   `0x80028D90`. The real extents are **560** and **556** bytes
   (prologue/`blr` at `0x80028D8C` and `0x80028FB8`), matching `functions.tsv`.
   Both bodies are full builders, not the 20-byte stubs the manifest implies.
2. **The 12-byte accessors are not `0x80377A04`.** The lane brief asked for
   verification of that claim; verification **fails**. Every one of the four is
   `lis r3,0x8035; addi r3,r3,<off>; blr` and returns a *distinct* record:
   `0x8035334C`, `0x8035338C`, `0x8035330C`, `0x803532CC`. They belong to the
   six-getter run `0x8002A800..0x8002A85C`, which returns the six 0x40-byte BSS
   records `0x8035338C`, `0x8035334C`, `0x8035330C`, `0x803532CC`,
   `0x8035328C`, `0x8035324C` in descending value / ascending function order.
   `0x8002D0C0` passes `0x8035334C` to virtual `+0x14` of the manager behind
   `scene + 0x394`, and `0x8002C850`/`0x8002C860` are byte-identical copies of
   `0x8002A820`/`0x8002A830`. The `0x80377A04` accessors are the separate
   `0x8001CBB0..0x8001DA70` family. Each new body carries its own
   `W204_RECORD_*` constant; no existing constant was redefined.
3. **Argument registers.** Ghidra models the two builders as
   `(undefined8, undefined8, int)` / `(undefined8, int)` because of its float
   promotion. The disassembly shows `r3 = this`, `f1`/`f2` in `f1`/`f2`
   (`fmr f31,f2` / `fmr f30,f1` at `0x80028B78`/`0x80028B84`), so the Rust
   signatures are `fn_80028b60(this, alpha: f32, scale: f32)` and
   `fn_80028d90(this, scale: f32)`. In `0x800295A0` the incoming `f1` is
   overwritten by the table load, so its signature is `(this, index)`.
4. **`0x8015F470` is a thunk.** The call at `0x800295D0` targets
   `0x8015F470`, whose only instruction is `b 0x8015F480`; Ghidra names the
   target `thunk_FUN_8015f480`. The body is called at the real entry
   `0x8015F480` (`W204_FN_8015F480`) and the thunk address is recorded in that
   constant's doc instead of being defined as an unused constant.
5. **`0x8002A7A0` dead branch.** After the first `cmpi cr0,3,r3,0` /
   `bc 12,2` at `0x8002A7A8`/`0x8002A7C0`, the body repeats the same `cr0` test
   at `0x8002A7C4`; nothing writes `cr0` in between, so the second branch is
   dead and both wrappers behave identically.
6. **SDA name is inline data.** `addi r4,r13,-0x7E28` (`0x80427858`) is not a
   pointer slot: the bytes at that address are the C string `"pakkun"`
   (neighbours in the same SDA block are `"jump_ed.dead"`, `"float"`,
   `"attack"`, `"damage"`, `"freeze"`). The constants doc records this.
7. **`0x802EE4F0` is an `f32` table.** Read directly from the image the first
   four words are `0.0`, `90.0`, `10.0`, `80.0`. The `lfsx` at `0x800295C0` is
   therefore a float load (the words after those four are unrelated bit
   patterns, so the table is consumed only for its first four entries here).

## Resolved constants

| expression | address | value / meaning |
|---|---|---|
| `SDA_BASE - 0x7E28` | `0x80427858` | `"pakkun"` (inline SDA string) |
| `0x80303090` | `0x80303090` | `"dokan_fire_neck"` |
| `0x803030B0` | `0x803030B0` | `"dokan_fire_attack"` |
| `0x803030C4` | `0x803030C4` | `"dokan_attack"` |
| `SDA2_BASE - 0x7CDC` | `0x8042B3A4` | `1.0` |
| `SDA2_BASE - 0x7CD0` | `0x8042B3B0` | `0.0` |
| `SDA2_BASE - 0x7CC8` | `0x8042B3B8` | `3.0` |
| `0x802EE4F0` | `0x802EE4F0` | `f32` fade table (`0.0, 90.0, 10.0, 80.0`, ...) |

Callees reached by address, all reused from existing `hud.rs` constants:
`FN_80239E30` (model lookup), `FN_8023A0B0` (animation lookup),
`FN_80064610` (effect factory), `FN_8015F800` (record destructor) and
`FN_802B9280` (record deallocator). Newly declared: `W204_FN_801651F0`,
`W204_FN_80168FE0`, `W204_FN_801656C0`, `W204_FN_8015F480`.

Three ABI helpers had no existing form and are declared with the `w204_`
prefix to avoid clashing with the other wave-204 lanes: `w204_call_f32_1u`
(`(f32, u32)`), `w204_call_f32_3u` (`(f32, u32, u32, u32)`) and
`w204_call_2f_1u` (`(f32, f32, u32)`). The existing `call_f32_2u` is reused for
`0x80169F80`.

## Notes on the bodies

1. **Slot programming loops.** The four `+0x50` calls of the primary group use
   `(slot, flag)` = `(0,0)`, `(3,0)`, `(4,0)`, `(5,0)`; the three calls of the
   secondary group use `(6,1)`, `(1,1)`, `(2,1)`. The value stored into each
   stack local is always the model word from the first `0x80239E30`, so the
   Rust loops write the same local per iteration. The vtable slot pointer is
   taken once per group, exactly as the original caches it in `r12`.
2. **Alpha ordering differs between the twins.** `0x80028B60` writes the group
   alpha after the four primary binds; `0x80028D90` writes it before the group
   is even read, and repeats the write for the secondary record. Both orders
   are preserved.
3. **Two `lfs` of `1.0`.** `0x801656C0` is entered twice with a separate
   `lfs f1,-0x7CDC(r2)` each time; the Rust re-reads `W204_F32_ONE` per call.
4. **`0x80029600` mask.** `rlwinm r4,r4,8,0,23` is an unmasked left shift by
   eight of the owner byte and `rlwinm r0,r0,4,0,27` is `(variant << 4) &
   0xFFFFFFF0`; both are plain `<<` on a `u32` here, which has the same low-32
   result.
5. **`0x800295A0` index masking.** The original computes the byte offset with
   `rlwinm r0,r4,2,0,29`, so the index is taken modulo the low 30 bits of
   `index * 4`. The Rust keeps the `i32` parameter and casts through `u32`
   before the multiply.
6. **Four distinct records, four distinct constants.** Unlike the wave-203
   `0x80377A04` family, these accessors cannot share one constant; each returns
   a different 0x40-byte record and each gets its own `W204_RECORD_*`.

## Evidence

- Disassembly: `work/dol/main.elf` via `$TEMP/opencode/ppc159.js`, `STOP=0`,
  `LEN=80` for the eight short bodies. The script's 80-instruction window
  (`i < 320`) truncates the two long builders, so they were dumped with a local
  copy that widens the window (`ppc159b.js`, `i < 2000`, 0x1400-byte read):
  `0x80028B60` (140 instructions) and `0x80028D90` (139 instructions), each
  ending in the `blr` at `0x80028D8C`/`0x80028FB8`.
- Decompilation: `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` (lowercase)
  for all ten addresses.
- Callee bodies consulted: `0x801651F0`, `0x80168FE0`, `0x801656C0`,
  `0x80169F80`, `0x8023A0B0`, `0x8015F480` and the thunk `0x8015F470` (all
  `main-gb` exports); `0x8002D0C0` for the record-usage evidence.
- Data: the four `f32` SDA2 words, the SDA name block at `0x80427840`, the
  three animation strings at `0x80303090` and the table at `0x802EE4F0` were
  read directly from the ELF image.

## Verification

`work/wave204/laneE_block.rs` was appended to a temporary copy of `src/`,
`Cargo.toml` and `Cargo.lock` (repo copy with `target/` and `.git/` excluded),
and `cargo check --lib` exited `0` with 0 errors. The copy reports the same 23
pre-existing warnings as the untouched tree and **none** of them point at
`src/ui/hud.rs` (the filter `hud.rs` returns no warning lines at all).

`node tools/check-lane-csv.mjs wave204` accepts the ledger file as `10 rows`
next to the lane C and lane F files that were present when it ran (30 rows,
30 unique addresses, `translated/high=27 translated/medium=3`). It still
reports every lane,
including this one, as "has no body and no `# Original` reference in
`src/ui/hud.rs`" because the block is not merged yet; that is the expected
pre-merge state and resolves once the coordinator appends the block.
