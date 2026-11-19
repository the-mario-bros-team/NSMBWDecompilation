# Audit - Wave 256 (semantic duplicate pass, batch 27)

## 1. Semantic-name duplicates: one real body bug found

`check-dupe-bodies` now has a second pass that groups definitions by the primary
address their `# Original` claims (`Ghidra: FUN_x at 0xADDR`, untabled
`Ghidra: none ... 0xADDR`, `- Address:`), ignoring `Evidence:`/`inlined`/host
prose. Groups fail unless the delegation/arch-split rules hold, or all
non-identical occurrences are <=2-line `// adapter:` forwarders.

State: **20,371 fn definitions; 59 duplicated names (0 allowed to fail); 19,395
claimed addresses, 6 claimed twice (all allowed)**.

Of the five groups wave 254 flagged:

| Address | Verdict | Action |
|---|---|---|
| `0x800967A0` | **duplicate body, and the stage copy was wrong** | `stage::fn_800967a0` corrected to the raw register image (`mr r3,r4`, so the receiver is the *second* argument and incoming `f1` is forwarded); `enemies::c243_shared_tail` now delegates |
| `0x801AB150` | adapter (host stub for inlined `mtspr 912+i`) | `// adapter:` marker |
| `0x801AD4E0` | three arity adapters | `// adapter:` markers |
| `0x80296750` | doc error (thunk cited the callee as its own body) | cites its own slot |
| `0x802D0CC0` | doc error (same) | cites its own slot |

The pass also caught and fixed **9 further groups**: missing `canonical:` markers
(`fn_8016d180`, `gki_buf_size`, `fn_802e9300/9398`), over-broad host-helper
claims (`0x80244B80`, `0x802DCBFC`, `0x80018330`, `0x80019560`, `0x80174560`),
and an orphaned doc fragment merged into `fn_8076da30` (`0x8062B460`).

## 2. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| B | die-state vtable 182-192 | 10 | 644 |
| C | `dEn_c` 282-291 | 10 | ~968 |
| D | `dEnBoss_c` next 10 | 10 | ~836 |
| E | archive band 0x803508B8+ | 10 | 424 |

Coverage: die-state 197/235, `dEnBoss_c` 198->208/265, archive band 46/79.

## 3. Findings

- `0x80350898` repeats one five-method cluster **once per owner pointer**
  (`+0x08`/`+0x10`/`+0x18`/`+0x20`): detach, move, sum, `u16[+0x18]`,
  `u16[+0x0C]`. The band's remaining content is largely four copies.
- `0x800B2EE0` is a thin wrapper over the shared LCG `0x80161460`
  (`+0x848 = high32(state*3)+1`).
- `dEnBoss_c` slots 89-91 are deliberately non-monotonic.
- Lane F audited 39 bodies with **10 doc corrections**, including C slot 278's
  extent **16 B -> 468 B** (stale `main` tsv row) and E's four extents each 4 B
  short (final `bclr` omitted).

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-dupe-bodies` exit 0 (both passes).
- Ledger 20,250 rows: 20,226 translated / 22 partial / 2 stub.
- `scripts/check.ps1` exit 0.
