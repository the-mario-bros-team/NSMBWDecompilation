# Audit - Wave 291 (helper class closed, base vtable added)

## 1. `_savegpr`/`_restgpr` defect class fully closed

| Wave | Live refs | Bodies |
|---|---:|---:|
| 276 (start) | 616 | 324 |
| 290 (after input.rs) | 310 | 167 |
| **291** | **0** | **0** |

Swept this wave: `os.rs` 203/89, `stage.rs` 58/44, `bt.rs` 41/28, plus
`registry.rs` 6/5 and `bres.rs` 2/1 already done. **Stub calls removed: 352**
(203 + 99 + 6 + 2), constants deleted 74, 1,000+ fake returns/arguments rebound.

The band is one Metrowerks table, not code: `0x802DCD40..84` is the
`stw r14..r31` save chain, `0x802DCD8C..D0` the `lwz` restore chain. So bodies
labelled "publish heap", "global lock/unlock" or "GX state save/restore" were
just prologue/epilogue pairs - and their fake return values were the incoming
`r3`/`r4`.

Mis-modelled aliases found (still helpers, still removed): `fn_801bd950`
(`_restgpr_26`, not 27), `fn_801f3310` (`_restgpr_25`), `fn_801f0b10`
(`_savegpr_25`).

## 2. Measurement bug that hid the work

Wave 289's "0 remaining, 220/89 residue" was wrong twice:

- the token regex was **case-sensitive**, and `os.rs` writes `0x802d_cd70`;
- attribution used `indexOf(lineText)`, collapsing every identical helper line
  into one body (167 bodies read as 89).

Fixed by `tools/check-helper-calls.mjs`: value-based resolution (case-insensitive,
underscores optional), comment/string masking, **brace-span** attribution,
per-file counts, separate dead-declaration list, fatal on live refs. Wired into
`scripts/check.ps1`; the tree is at 0 refs, so the gate is green with it armed.

## 3. Table closed

`0x80317920` **22/22**: base vtable of a `dBase_c`-derived non-actor, installed
only by `fn_800cd710` at `+0x60`; caller `fn_806f2e50` then replaces it with the
`d_basesNP` derived table `0x80801210` (0xBEC0 object, `DEMO_field_op/end`,
`g3d/bros_mega.brres`, 15 inherited / 7 overridden). Slot 17 returns the
`0x8030F1B8` "non-actor derived from dBase_c" descriptor.

**Ninety-two vtables complete.**

## 4. Two more body defects fixed

- `fn_802b9500` / `fn_802b9c60` (EGG heap destroy): both stopped at the
  destructor `bctrl` and omitted the guarded `previous->+0x18(previous, this)`
  notification. Extents were 80 B; true extent **136 B**. Both fixed.
- `res_palette_table_copy` (wave 291 C): the Ghidra `u64` is `r3` **high** /
  `r4` low; the body now takes `context` and `dst` explicitly.

## 5. Hygiene debt

Warnings went 31 -> 63. Categories: ~30 `value assigned ... never read` in
`os.rs`/`stage.rs`/`bt.rs` left by removing consuming helper calls, and 13
`unnecessary parentheses` in `bt.rs`. No errors, no `dead_code`. Queued as a
dedicated hygiene lane rather than accepted silently.

## 6. Survey

111 fully covered runs (unchanged this wave), 508 distinct prologue targets,
max 4. Remaining top targets: `0x8034fb40` (layout header), `0x80306f80` and
`0x80306d68` (actor pair), `0x8030efc8` (state record), `0x803226a4`
(`W110_GOAL_STATE_VTABLE`).

## 7. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | `os.rs` sweep | 89 fixed | - |
| B | `stage.rs` + `bt.rs` sweep | 72 fixed | - |
| C | `registry.rs` + `bres.rs` sweep | 6 fixed | - |
| D | `0x80317920` close | 9 | ~430 |
| E | gap batch 21 | 12 | ~1,390 |
| F | audit + helper tool + 11 dead consts | 0 | - |

## Verification

- `cargo check` exit 0 (63 warnings); `doc-lint` 0 violations.
- `check-dupe-bodies` exit 0; `check-page-offsets` 63 pairs, 0 mismatches;
  `check-helper-calls` 0 refs; `check-source-integrity` 0 findings.
- `scripts/check.ps1` exit 0.
