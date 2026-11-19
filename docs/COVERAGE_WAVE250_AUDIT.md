# Audit - Wave 250 (dispatcher family, residue documented)

## 1. Dispatcher helpers generalised

`game::bases` now has a full family, all sharing one descriptor resolver
(`runtime::cpp::fn_802dcbfc_target`):

| helper | register window |
|---|---|
| `interface_call`, `interface_call2..5` | `r3`; `r3+r4`; `r3..r5`; `r3..r6`; `r3..r7` |
| `interface_call_descN` | same, descriptor address direct |
| `interface_call_f32_1` | `f1 + r3 + r4` |

Return type is generic, so `f1`-returning sites flow through the same path.

**88 call sites converted**: 44 `interface_call3` (stage 19 - including the 11
raw `0x802D_CBFC` sites and 6 constant-form thunks; player 7; worldmap 18),
21 `interface_call2`, 16 `interface_call_desc` for caller-built cursor records
(bres 7, bt 5, gx_band 2, registry 2), plus `interface_call_desc3`/`2`/`5` and
`interface_call_f32_1` cases.

This removed 8 `// partial:`/medium caveats and several stale constants whose
docs had claimed `0x802DCBFC` was an allocator, a character decoder or a
"jump-table trampoline".

## 2. Residue that is genuinely not expressible (9 sites)

1. `src/ui/lyt.rs` - 8 character-reader calls in `fn_8018d540`, `fn_8018ff00`,
   `fn_8018da40`, `fn_80190420`: the original passes
   `r3 = &{cursor, font[+4], font[+8], font[+0xC]}`, `r12 = r3 + 4`, and the
   callee advances `record[0]` **in place**. The Rust tracks `cursor` as a local
   and advances it arithmetically, so a temporary record would drop the in-place
   advance; expressing it needs the four text loops restructured.
2. `src/engine/display.rs` `fn_800d16b0`:
   `r12 = 0x802F11F0 + 12 * *(r4 + 0x20)` with the caller's `r3`/`r4` untouched;
   the declaration has no parameters, so neither base can be named.

## 3. Bodies and tables

| Lane | Work | Result |
|---|---|---|
| B | `WM_PLAYER_VTABLE` secondary tables | **already complete**: 27/27 function words have one owner (18 distinct targets). The span is four tables, not one: `0x80326BE8` (live, `+0x14E0`), `0x80326C18`, `0x80326C48` (`+0x14E8`), `0x80326C60` (`+0x14EC`) |
| C | `dEn_c` 216-225 | 10 bodies, 436 B |
| D | `0x803502F8` band 0x80350650+ | 10 bodies, 232 B |
| E | `daYoshi_c` state templates | 3 bodies; **states 4-14 have no handler word** - their 33 records are `{0, key, 0x60}` with class-independent ids whose target is unresolved |

## 4. Ledger invariant check

Lane A's fragment accidentally added `0x8004AAC0` (an untabled body) to the
ledger, taking it to 20,251 rows. The address is in no `functions.tsv`; the row
was removed from the agent CSV and re-merged.

Ledger is back to **20,250 rows: 20,226 translated / 22 partial / 2 stub**.

The 22 remaining partials are catalogued by category in
`docs/COVERAGE_FINAL.md` (8 machine-state, 5 caller-owned descriptors, 2
`va_list`, 2 jump tables, 3 paired-single, 2 one-off), plus the 2 forwarded
stubs. They are documented as the defensible residue, not unstarted work.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `scripts/check.ps1` exit 0, body enforcement armed.
