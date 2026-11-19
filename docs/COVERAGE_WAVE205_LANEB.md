# Coverage - Wave 205 lane B (`ui::hud`, main.dol)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/hud.rs` | `ui::hud` | 10 | 868 | 10 | 0 |

Addresses `0x8002B690`-`0x8002C860`, the tail of the `0x8002B6xx`-`0x8002C8xx`
run that the wave-204 door/leaf lanes left unclaimed.

## What landed

- **State ticks.** `0x8002B690` (144 B) resets the `+0x544` record through
  virtual `+0x1C`, updates the `+0x60` owner through `+0x2AC`, and - when
  `0x80169020(this + 0x584)` reports the animation finished - calls `+0x2A8`
  and then `+0xD4` with the BSS record `0x80353090`. `0x8002B780` (136 B) is the
  same shape with slot `+0x298` and a state-pair notify gate.
- **Deleting-destructor wrappers.** `0x8002BCC0` and `0x8002C7F0` are the
  `0x8015F800` / `0x802B9280` pair already translated as `fn_8002a740` and
  `fn_8002a7a0`.
- **Interface forwards.** `0x8002BDE0` and `0x8002BE10` are the `0x802DCBFC`
  thunks with the descriptor at `a + 0x18` and `a + 0x24`, identical to
  `fn_80026b50` / `fn_80026b80`.
- **Animation binder.** `0x8002BE60` binds `"dokan_attack"` (the second copy of
  that name, at `0x80303A80`) into the `+0x544` model, applies the caller's
  scale, then sets the `+0x584` record scale to `1.0` and its alpha to `0.0`.
- **Actor destructor.** `0x8002C370` (148 B) is byte-equivalent to
  `fn_800282e0`: five member destructors in reverse construction order, then
  `0x80162920` for a positive delete flag.
- **Record accessors.** `0x8002C850` and `0x8002C860` are the 12-byte
  `lis`/`addi`/`blr` bodies returning `0x8035330C` and `0x803532CC`; the
  wave-204 lane F accessors `0x8002A820` / `0x8002A830` are byte-identical.

## Corrections and recoveries

- **`0x8002BE60`'s size looks like a prologue artefact but is not.** The
  wave-205 size list carries 24 bytes for this address; `functions.tsv` and the
  prologue/`blr` extent agree on 140 bytes (`0x8002BE60..0x8002BEEC`). The lane
  doc uses the `functions.tsv` value and records the list discrepancy here.
- **`0x8002BE60`'s Ghidra signature and float noise are wrong.** The export
  prints `undefined8 param_1, int param_2` plus `unaff_r2`/GQR `ldexpf`
  pseudo-code; the disassembly is `mr r31,r3` / `fmr f31,f1` with two plain
  `lfs` loads from `r2 - 0x7C98` (`1.0`) and `r2 - 0x7C94` (`0.0`). The
  reconstruction takes `(this, scale)` and feeds the saved registers through
  `call_f32_2u` / `call_f32_1u`.
- **The `0xD4` operands are BSS, not strings.** `0x80353090` and `0x80353010`
  sit in the `0x80351680..0x80427680` zero-filled segment
  (`p_filesz = 0`, `p_memsz = 0xD6000`), so both are state records passed by
  address. Ghidra's `DAT_` names are kept, suffixed `_B` to avoid cross-lane
  collisions.
- **`0x8002B780`'s second gate is unsigned.** `addi r0,r3,-1` / `cmpli 0,1`
  is `*(this + 0x5CC) - 1 < 2` on unsigned words; `wrapping_sub(1) < 2` keeps
  `0` from wrapping into the accepted range.
- **Both destructors carry a dead duplicate null test.** `0x8002C370`
  branches twice on the same `cr0` bits (`41820058`, then `41820044`), and
  `0x8002C7F0` repeats the test before the free ladder. Neither duplicate is
  reproduced; the observable behaviour is unchanged.
- **`0x8002BE60` belongs to a second name table.** The name at `0x80303A80`
  is `"dokan_attack"` but sits next to `"dead_leaf"`, not next to the
  `dokan_fire_*` run at `0x80303090`; the wave-204 constant
  `W204_ANIM_ATTACK` (`0x803030C4`) is a different copy, so this lane adds its
  own `S_DOKAN_ATTACK_B`.

## Deferrals

None. All ten bodies are fully reconstructible from shipped code.

## Verification

- Temp-copy `cargo check --lib` exit 0 (23 pre-existing warnings elsewhere; none
  from `src/ui/hud.rs`).
- `node tools/check-lane-csv.mjs wave205` reviewed locally against this lane's
  own CSV: 10 rows, 10 unique addresses, plain lowercase hex, evidence paths
  present.
- No files outside `work/wave205/laneB_block.rs`,
  `ledger/agents/wave205-laneB.csv` and this document were written;
  `src/ui/hud.rs` is untouched.
