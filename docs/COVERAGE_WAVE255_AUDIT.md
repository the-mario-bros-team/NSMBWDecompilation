# Audit - Wave 255 (duplicate gate armed, boss vtable opened)

## 1. Duplicate bodies are now gate-enforced

`tools/check-dupe-bodies.mjs` groups every `fn_<6..8 hex>` definition in `src/`
and fails a group unless all but one occurrence is a one-line `canonical:`
delegation, or the occurrences are a differing `cfg(target_arch)` pair. Brace
matching runs on a comment/string-masked copy, so braces inside prose cannot
skew spans.

Wave 256 added a second pass over semantic names: every definition is grouped
by the primary address its `# Original` block claims (`Ghidra: FUN_x at 0xADDR`,
an untabled `Ghidra: none ... 0xADDR`, or an explicit `- Address:`), and a group
with more than one body fails unless it is a delegation, a documented
`// adapter:` forwarder, or an arch split. On the wave-256 tree the pass found
the real `0x800967A0` duplicate and nine further address-level groups
(over-broad host-side claims, wrappers without the delegation marker, and one
orphaned doc fragment), all recorded in `work/wave256/laneA_report.md`.

It found exactly **one** real duplicate, which I fixed before arming:

- `fn_80237ca0` existed as a host stand-in in `render/geometry.rs` (21 uses,
  `transmute` to the guest address) and as the canonical GQR translation in
  `resource/bres.rs` (the ledger row). The host helper is now `host_sincos`.

State: **20,331 definitions, 59 duplicated names, 59 allowed, 0 failing**. The
check is wired into `scripts/check.ps1` after `verify-ledger`, and the full gate
is green.

## 2. `dEnBoss_c` vtable opened

`0x80312750..0x80312B74` is **265 slots**, all `main.dol` text pointers, with no
nulls and no duplicate targets, identified by the adjacent
`dEnBoss_c::StateID_DieFire/DieSlide/DieShell/DieStar` strings. 77 uncovered at
the start, **67** after the first 10 bodies. Slot offsets at 89-91 are
deliberately non-monotonic (`0x178`/`0x17C`/`0x180` in table order).

## 3. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| B | die-state vtable 171-181 | 10 | 2,596 |
| C | `dEn_c` 271-281 | 10 | 2,460 |
| D | next archive band 0x80350854+ | 9 | 1,060 |
| E | `dEnBoss_c` first 10 | 10 | 492 |

Coverage: die-state 187/235, archive band 248/249 (old) and 36/79 (new),
`dEnBoss_c` 198/265.

Lane D closed the old band's last word (`0x80350848`) by pointing at
`enemies::fn_802d83d0` rather than duplicating it, and decoded the next band's
two vtables (`0x80350858` 9 slots, `0x80350898` 24 slots) plus the `"dummy_tex"`
name record.

## 4. Findings

- Lane C corrected another sign-extension: the descriptor base is
  **`0x80358138`**, not `0x80368138`.
- Lane B's slot 171 reduces a register-level sign-mask sequence at
  `0x800A5EB4..0x800A5F20` to `+0x102 != -1`; documented as such.
- Lane E: five of its six `0x80329958` targets are untabled and `0x8016D180` is
  the tabled ledger row; `0x802D83D0` has seven table-word references, not one.
- Evidence audit (lane F): **36/36** wave-254 bodies verified; 7 doc-only
  defects fixed (three padding claims, two count claims, one stale coverage
  figure, one byte figure).

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-dupe-bodies` 59 allowed, 0 failing.
- Ledger 20,250 rows: 20,226 translated / 22 partial / 2 stub.
- `scripts/check.ps1` exit 0.
