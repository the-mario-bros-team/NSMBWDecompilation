# Coverage - Wave 210 lane F (`game::player`, main.dol)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 10 | 1496 | 6 | 4 |

Addresses `0x80062600`-`0x800630B0`: the dokan-style model actor that owns the
`+0x394` model context and the `obj_dokan` model set. The record shape matches
the one already documented in this module for the neighbouring dokan code:
class vtable at `+0x60`, position at `+0xAC..+0xB4`, heading at `+0x104`,
owning-player byte at `+0x38D`, model context at `+0x394`, BRRES at `+0x3B0`,
main model at `+0x3B4`, 32 `B` segments at `+0x3C0` (stride `0xC`), cap model at
`+0x540`, angle copies at `+0x54C..+0x554`.

## What landed

- **Setup-block setter.** `0x80062600` (16 B) is three `stw` at
  `+0x3DC/+0x3E0/+0x3E4` followed by `blr`. Two REL call sites in
  `crate::game::bases` pass `(this, start, end, constructor)` and call the
  triple a range/constructor binder; that role is recorded as inferred.
- **Variant bind.** `0x80062610` (140 B) gates on the class vtable slot `+0xE0`,
  clears the model handle `+0x3D8`, copies `+0x04 & 0xFF` to the variant byte
  `+0x3E8`, calls `0x80062520(this, params & 0xFF)`, and on failure hands the
  empty descriptor `0x80355D4C` to the `+0x394` context slot `+0x18`.
- **Release pair.** `0x80062B60` (88 B) and `0x80062BC0` (92 B) are the
  `(ptr, count)` release: `0x8015F800(this, 0)` for every non-null object, then
  `0x802B9280(this)` when `count > 0`, returning `this` unchanged. The second
  copy carries one dead `beq` (`0x80062BE0`) that re-uses the `cmpi` flags of
  `0x80062BC8` and can never be taken; that extra pair of instructions explains
  the four-byte size difference and is not reproduced.
- **Chain probe.** `0x80062D00` (224 B) walks up to five `+0x30` children,
  stopping at the first whose virtual `+0x0C` reports success, and recurses when
  all five fail. It is a byte-identical twin of `fn_80044ff0`.
- **Name predicate.** `0x80062DE0` (136 B) is a byte-identical twin of
  `fn_800450d0`: the virtual `+0x20` name and the `path` argument are both cut
  at the last `:` with `0x802E1C40` and compared with `0x802E1AB4`.
- **Member forwarders.** `0x80062EA0` (48 B) and `0x80062ED0` (48 B) are the
  standard `0x802DCBFC` pointer-to-member thunks: `mr r5,r3; mr r3,r4; addi
  r12,r5,0x18|0x24; bl 0x802DCBFC`, i.e. `0x802DCBFC(second, second, first)`
  with the member descriptor at `first + 0x18` / `first + 0x24`.
- **Spawn/init.** `0x80062F00` (368 B) decodes `+0x640 = (params >> 4) & 3` and
  `+0x63C = (params >> 8) & 3`, builds the models through `0x800630B0`, writes
  the scale triple `1.0f` and `+0x644 = 16.0f * ((params >> 16) & 0xFF)`, stores
  `params & 3` in the owning-player byte, runs the five `+0x60` vtable slots
  `+0xE4/+0xDC/+0xE0/+0xE8/+0xD8`, resets `+0x558`, writes the heading pair
  (mirrored by `+0x8000` in mode `2`), projects `+0xAC` and starts sound `0x70E`
  through `0x80429D60`, writes `0.0f` to the second scale triple and copies the
  position to `+0x54C..+0x554`.
- **Model build.** `0x800630B0` (336 B) prepares the `+0x394` context, opens
  `g3d/obj_dokan.brres` through `0x8042A038`, creates the `0x80309C54[mode]`
  main model, 32 `obj_dokan_B` segments and the `0x80309C6C[mode]` cap model
  (all at flag `0x20`, each finalised by `0x80064BF0`), then detaches the
  context through the `0x800690E0` thunk to `0x8016A8E0`.

## Corrections and recoveries

- **No prologue artefacts in this lane.** All ten `functions.tsv` sizes are the
  exact extent: the `blr`s are at `0x8006260C`, `0x80062698`, `0x80062BB4`,
  `0x80062C18`, `0x80062DDC`, `0x80062E64`, `0x80062ECC`, `0x80062EFC`,
  `0x8006306C` and `0x800631FC`, i.e. 16/140/88/92/224/136/48/48/368/336.
- **The Ghidra export of `0x80062EA0`/`0x80062ED0` loses `r12`.** It prints
  `FUN_802dcbfc(param_2,param_2,param_1)` with a third argument that is not an
  EABI register. Disassembly shows `addi r12,r5,0x18` (resp. `0x24`), so the
  hidden member descriptor is known exactly; only the register itself cannot be
  set through `platform::call3`, which is the same `// partial:` limitation
  already accepted for `fn_80059260`.
- **`0x80062F00`'s `r2`/`r13` references were resolved from the ELF.** The
  export leaves `unaff_r2`/`unaff_r13`; with `r2 = 0x80433080` and
  `r13 = 0x8042F680` the constants are `0x8042BAC4 = 1.0f`,
  `0x8042BAC8 = 16.0f`, `0x8042BACC = 0.0f`, `0x8042BAD0 = 2^52`, `r13 - 0x5920 =
  0x80429D60` and `r13 - 0x5648 = 0x8042A038`. The `0x43300000` pair is the
  unsigned-int-to-double bias, written as `kind as f32`.
- **`+0x38D` is `params & 3` here, not `(params >> 12) & 3`.** The disassembly
  is `rlwinm r0,r5,0,30,31` then `stb r0,0x38d(r30)`; the neighbouring dokan
  spawn (`fn_80041c00`) uses the shifted field instead. Recorded in the body.
- **`0x800630B0`'s final call is `0x800690E0`, not `0x8016A8E0` directly.** The
  export prints `thunk_FUN_8016a8e0`; the site is `bl 0x800690E0`, a six-byte
  `b 0x8016A8E0`. The translation uses the already-named
  `W92_MODEL_FLUSH_8016A8E0`.
- **The two cap-model name tables are not the ones in `actors::dokan`.** This
  address range uses `0x80309C54` = `{obj_dokan_A, obj_dokan_CU, obj_dokan_A}`
  and `0x80309C6C` = `{obj_dokan_B, obj_dokan_B, obj_dokan_A}`; the
  `0x80306CFC`/`0x80306D18` pair used by `fn_80041d40` names `CD` instead. Mode
  is still `(params >> 4) & 3`, so mode `3` reads one entry past both tables
  exactly as the original does.

## Deferrals

None. All ten bodies were reconstructible from the shipped code.

## Verification

- Temp-copy (`%TEMP%\opencode\w210f`) `cargo check --lib` exits 0 with the 23
  pre-existing warnings and none in `src/game/player.rs`.
- The lane CSV has the canonical seven-column header, ten rows, plain lowercase
  hex addresses and `# Original` sizes equal to `functions.tsv`
  (16/140/88/92/224/136/48/48/368/336).
- No files outside `work/wave210/laneF_block.rs`,
  `ledger/agents/wave210-laneF.csv` and this document were written;
  `src/game/player.rs` is untouched.
