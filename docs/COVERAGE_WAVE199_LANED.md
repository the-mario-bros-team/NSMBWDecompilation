# Wave 199 lane D coverage (`game::bases`)

Slice: the `0xA08` actor's two-arm scale stepper (`0x8073_1D10`) and the
`0x760`-byte class's 2576-byte action-command state machine (`0x8078_DC70`).
Two functions, two translated, none deferred.

| address | size | body |
| --- | --- | --- |
| `0x8073_1D10` | 540 | `fn_80731d10` - phase `+0x9E4` machine: two `fn_80731cb0` arm steppers (`1.5`/`0.08`, `0.75`/`0.04`), `+0x9E8` level counter with the `1.15`/`1.075` seed, `+0x9F8` direction, four-entry ring at `+0x9B8` with `+0xA07` index, flag-driven `+0x958`/`+0x968`/`+0x970` output pairs |
| `0x8078_DC70` | 2576 | `fn_8078dc70` - `(this, id, gate)` command dispatcher: 30-arm jump table at `0x8081_A580` for the gated first pass, then the `0x8078_E20C` compare tree; touches scene requests, HUD publish, player/course records and the `+0x60` virtual `+0x64`/`+0x68` |

## Signature recoveries

* `0x8073_1D10` is a plain one-argument body; `r30 = 0x807CD6C8` is the constant
  table base and `r3` is the only live input.
* `0x8078_DC70` is declared `void FUN_8078dc70(undefined4, undefined4, int)`.
  The prologue is `mr r30,r3`, `mr r31,r4`, then `cmpi r5,0`; the body reads
  `r30` as the object (`+0x60` vtable, `+0x1A4`/`+0x1B8` timers), `r31` as the
  command word and `r5` as the enable gate. Recovers as
  `fn_8078dc70(this: usize, id: i32, gate: u32)`. Ghidra binds `r3`/`r4` to the
  return pair of the `_savegpr_26` thunk at `0x802D_CD70`, which is why its
  parameter list looks permuted.

## Dispatch proof for `0x8078_DC70`

* The jump table is `lis r5,0x8082` + `addi r5,r5,-23168` = `0x8081_A580`
  (not `0x8082_A580`: `-23168 = -0x5A80`). All 169 entries were dumped; only
  `0x00`, `0x08`, `0x09`, `0x4F`, `0x50`, `0x52`, `0x53`, `0x54`, `0x6D`,
  `0x6F`, `0x74`, `0x75`, `0x79`, `0x7C`, `0x80`, `0x81`, `0x84`, `0x85`,
  `0x86`, `0x8C`, `0x8D`, `0x96`, `0x99`, `0x9C`, `0x9D`, `0xA1`, `0xA5`,
  `0xA6`, `0xA7`, `0xA8` leave `0x8078_E20C`; every other id is a no-op in the
  first pass. The pass is skipped entirely when `gate == 0` or
  `id > 0xA8` (unsigned `cmpli`), and `id == -1` returns before both passes.
* The second pass is the literal `cmpi` tree at `0x8078_E20C..0x8078_E300`
  (`0x79`, `0x52`, `0x00`, `0x99`, `0xA6` -> `0x8078_E304`; `0x08`/`0x74` ->
  `0x8078_E328`; `0x09` -> `0x8078_E364`; `0x50` -> `0x8078_E39C`;
  `0x4F`/`0x53` -> `0x8078_E3C4`; `0x6F`, `0x5C`, `0x7C`, `0x84`, `0x85`,
  `0x96`, `0x9C`, `0x9D`, `0xA5`, `0xA7`, `0xA8`; default `0x8078_E660`).
  It is written as the equivalent `match`, since the tree reaches exactly those
  ids and nothing else.
* `cntlzw(x)>>5` survives as `rlwinm r0,r0,27,5,31` and is written
  `leading_zeros() >> 5`; the `0x08`/`0x74` arm tests `value - 1`, the `0x09`
  arm tests `value`.

## Constants and globals

* `0x807C_D780 = 1.5`, `0x807C_D784 = 0.08`, `0x807C_D788 = 0.75`,
  `0x807C_D78C = 0.04`, `0x807C_D790 = 1.15`, `0x807C_D794 = 1.075`
  (`r30+0xB8..0xCC`), `0x807C_D710 = 1.0` (`r30+0x48`), `0x807C_FD78 = 0.0`.
* Absolute globals recomputed from the `lis` + negative displacements:
  `0x8042_873C` (`-0x78C4`), `0x8042_9C30` (`-0x63D0`), `0x8042_9F7C`
  (`-0x6084`), `0x8042_A040` (`-0x5FC0`), `0x8042_A1A0` (`-0x5E60`),
  `0x8042_A1AC` (`-0x5E54`), `0x8042_A24D` (`-0x5DB3`), `0x8042_A250`
  (`-0x5DB0`), `0x8042_A27C` (`-0x5D84`), `0x8083_4470` (`+0x4470`),
  `0x8081_A554` (`-0x5AAC` via `0x80820000`).
* `0x8042_A19E` is written as a **halfword** (`sth` at `0x8078_E05C` and
  `0x8078_E4CC`); Ghidra only shows the overlapping `_DAT_8042a19e` byte.

## Ghidra gaps

Ghidra drops the argument to `0x800B0DB0` in the `0x08`/`0x09`/`0x74`/`0x75`
arms. The raw words keep `r3` live from the preceding call
(`0x80101630` for `0x08`/`0x74`, and the `fn_8078e8d0` result for `0x09`/`0x75`),
so the value is forwarded. It also prints `0xA1`'s stored constant as `-1`
correctly only after folding `addi r5,r0,-1`, which is `li r5,-1` (`rA == 0` is
the literal zero), so the stored word is `0xFFFF_FFFF`.

## Deliberate call-by-address sites

`0x8010_1630` (scheduler query), `0x800E_0380`/`0x800C_E360`/`0x800C_E190`/
`0x800F_C390` (slot/record queries), `0x8010_0D90` (HUD float publish),
`0x8010_5240` (record bind), `0x800B_0DB0`/`0x800B_0A20`/`0x800B_0A90`
(scene requests) and `0x800F_C810`/`0x800F_C830`/`0x800F_C970`/`0x800F_CB50`/
`0x8010_0900`/`0x800B_52F0` have no Rust bodies and stay `abi_callN!` /
`call0_ret`. `0x8078_E900`/`0x8078_E910` ignore the `r3`/`r4` pair the caller
computes, so the argument call `0x80101630` is kept for its own side effect and
the callee is invoked with its existing zero-argument Rust body. The `+0x60`
vtable slots `+0x64`/`+0x68` go through `vmethod0(this + 0x60, ..)` and the
`0x8042_A440` object's `+0x10` through `vmethod0(ptr, 0x10)`, matching the
surrounding `fn_8078e6xx` family style.
