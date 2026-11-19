# Audit - Wave 257 (verification of the 0x800967A0 fix, batch 28)

## 1. The `0x800967A0` fix holds under independent re-decode

Three independent facts confirm the correction:

1. `800967A8: mr r3,r4` overwrites the incoming `r3`;
2. the vtable load (`lwz r12,0x60(r3)`) and the float load
   (`lfs f2,0x10C(r4)`) both occur *after* that move, so they address the same
   object - the old model read them from two different objects, which was the
   defect;
3. nothing writes `f1`, so the caller's float is forwarded unchanged.

Caller census: a whole-image `b`/`bl` scan finds **exactly the five tail branches**
in the doc. Each sits in a slot body that only produces `f1` and leaves `r3`/`r4`
alone; all four `+0x24C` dispatchers set `r3 = this`, `r4 = other` before
`bcctr`. On the Rust side the address has one caller
(`enemies::c243_shared_tail`, `(other, value)`).

## 2. Semantic-classification samples

Three classifications were sampled and all hold:

- `0x8016D180` (`canonical` -> `lyt::layout_register`): the 256-byte body matches
  instruction for instruction, including the `addi r3,r3,4` + `bl 0x802E1AB4`
  compare with `r4`/`r5` preserved and the `0x80377B1C`/`0x80377C30` list split.
- `0x801F73B0` (`canonical` -> `input::fn_801f73b0`): `(buf-8)&1` guard and the
  table fold resolve to `0x803CDCB0` with a `pool*16` index.
- `0x801AD4E0` (`adapter` x3): one variadic diagnostic routine reached by three
  arity forwarders; no competing implementation.

Bonus: `0x802E9300`/`0x802E9398` differ only in the two documented places
(`xori ... 0x8000` clearing `MSR.EE`, and `li r3,2` vs `or r3,r5,r5`).

## 3. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| B | die-state 193-203 | 10 | 1,640 |
| C | `dEn_c` 292-301 | 10 | ~250 |
| D | `dEnBoss_c` 179-197 | 10 | ~1,100 |
| E | archive band 0x803508E8+ | 10 | ~780 |

Coverage: die-state 207/235, `dEnBoss_c` 47 uncovered, archive band 56/79.

## 4. Findings

- The archive band's allocator is now fully understood: the new `0x802D98B0` is
  the **free half** of `runtime::cpp::fn_802d9380`, and the allocator's split
  path proves `+0x08`/`+0x0C` are **neighbour links**, correcting the earlier
  "child/owner, sibling" guess.
- `dEnBoss_c` bodies 187/189/190/195 are one damage-spin family differing only
  in entry guard, notice/stat calls and final mode.
- Lane F audited 40 wave-256 bodies: **40/40** word, slot, extent, ownership and
  reachability claims hold; 4 doc corrections (a false "56 zero bytes precede
  it", a `lis`/`addi` split, a `bcctr` address, a forwarder count).

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations.
- `check-dupe-bodies` exit 0 (59 allowed names, 19,415 claimed addresses).
- `scripts/check.ps1` exit 0.
