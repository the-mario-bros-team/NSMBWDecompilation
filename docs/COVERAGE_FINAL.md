# Final coverage accounting (wave 250)

Authoritative, frozen summary of translation coverage for the SMNE01 build
`r37675`. The row counts were frozen at wave 239; the ledger status split was
re-frozen at wave 250 after wave 249 lane D finished the machine-state revisit.
Regenerate the underlying counts with the commands at the end of this file; do
not hand-edit the figures in other docs without changing this one first.

## Headline

| Quantity | Count |
|---|---:|
| Ghidra `functions.tsv` inventory | **20,252** |
| Rows with a Rust translation (the ledger) | **20,250** |
| ... status `translated` | **20,225** |
| ... status `partial` | **23** |
| ... status `stub` | **2** |
| Documented deferrals | **2** |
| Remaining tabled functions | **0** |
| Coverage | **99.99%** |

The two outstanding rows are `game::bases` deferrals, not missing work:

| Address | Export | Bytes | Why deferred |
|---|---|---:|---|
| `0x806E2BF0` | `d_basesNP` | 96 | reads state no shipped code writes |
| `0x80697090` | `d_basesNP` | 248 | reads state no shipped code writes |

Together they account for 344 bytes. `20,250 + 2 = 20,252`; every other tabled
function has a translated body. The `functions.tsv` inventory is therefore
complete and no lane should expect another tabled row.

## Residual partial and stub rows (23 partial + 2 stub)

The wave-249 status pass closed every partial that had a Rust expressible form.
What remains is the **defensible residue**: each row's remaining gap is a
property of the target machine or of the caller's ABI, not an unread body. The
control flow and the observable data effects are translated; the listed piece
cannot be written in safe, portable Rust without changing the calling contract.

| Category | Rows | Addresses | Why it remains partial |
|---|---:|---|---|
| Machine state with no Rust form | 8 | `0x801ACE30`, `0x800041C0`, `0x802E9300`, `0x802E9398`, `0x802EA6BC`, `0x802EC9CC`, `0x802ECCB4`, `0x801A9D40` | `rfi`, `mtmsr`, LR/MSR composition, and GPR/SPR/FPR save-restore bodies. The bodies record the register image; executing it from Rust would clobber ABI-reserved registers. |
| Caller-owned descriptors | 5 | `0x80234A80`, `0x80236230`, `0x802365E0`, `0x80236FC0`, `0x80237370` | The `resource::bres` heap/format/sink entries keep the caller's FPR/GPR descriptor as an explicit `args` argument. The frame is built by many call sites; no site in the tree passes an explicit frame yet. |
| `va_list` overflow area | 2 | `0x80107050`, `0x80107170` | Bounded variadic formatter and text measure. The named arguments are modelled; the overflow argument area belongs to the caller's frame. |
| Unrecovered jump tables | 2 | `0x80A0DB40`, `0x80133E10` | The switch targets behind `0x80A0DB80` and `0x80133E58` are not recoverable from the shipped words; the visible arms are literal and the table dispatch is documented. |
| Paired-single / missing mode | 3 | `0x80292C90`, `0x80299840`, `0x802A71D0` | GQR/`psq` ladders (case-4/case-7 direction builders, the grid mode of the sphere emitter, and the `ps5[0]`/`ps6[0]` edge offsets). Every operand is GQR0 scalar `f32`, but the remaining construction is not literal. |
| One-off limits | 3 | `0x80AEEA30`, `0x80086B20`, `0x80176410` | Respectively: the `+0xE4` store reads the caller's uninitialised `0x10(r1)` slot; `fn_802DCBFC` forwards only `r3` while the call passes `r4`/`r5`; the `0x80176410` gauge body is a GQR0/`psq_l` image whose paired-single decode is not literal. |

The two `stub` rows are entry points with intentionally reduced bodies, both
runtime services that are dispatched by address: `sdk_panic` (`0x801AD4E0`,
300-byte variadic panic) and `set_exception_handler` (`0x801AD610`, the
handler-table install). Neither has a translated caller path that needs the
body's internals, so they stay marked `stub` rather than `partial`.

This is the final residue. All 20,225 other rows are `translated`; closing any
of these 25 requires an ABI/`asm` facility or caller-frame work, not another
translation pass over unread code.

## Per-source-module coverage

These are the five Ghidra exports, the unit the ledger is verified against.

| Export | Translated | Total | Percent | Remaining |
|---|---:|---:|---:|---:|
| `main.dol` | 10,276 | 10,276 | 100% | 0 |
| `d_profileNP` | 36 | 36 | 100% | 0 |
| `d_basesNP` | 5,613 | 5,615 | 99.96% | 2 |
| `d_enemiesNP` | 3,555 | 3,555 | 100% | 0 |
| `d_en_bossNP` | 770 | 770 | 100% | 0 |
| **total** | **20,250** | **20,252** | **99.99%** | **2** |

A translated row means a body is present in `src/`. It does not mean every
field name or branch polarity is proven; the ledger `status` and `confidence`
columns carry that second axis.

## Ledger status and confidence

| Status | Rows | Meaning |
|---|---:|---|
| translated | 20,225 | control flow represented, evidence recorded |
| partial | 23 | body present; the named machine/ABI sub-path is documented and cannot be expressed without a contract change |
| stub | 2 | entry point present; body intentionally reduced |
| **total** | **20,250** | |

The 25 partial/stub rows are already counted in the 20,250 coverage figure.
Coverage is "a body exists"; status is "how complete that body is". The full
residue inventory is in "Residual partial and stub rows" above.

| Confidence | Rows |
|---|---:|
| high | 12,954 |
| medium | 7,061 |
| low | 235 |
| **total** | **20,250** |

## Per-semantic-module row counts

Rows are grouped by `rust_file` (the semantic module a reader opens), which is
why this table differs from the export table above: six `d_basesNP` addresses
are player functions and live in `src/game/player.rs`, so `game::bases` holds
5,607 rows while the `d_basesNP` export supplies 5,613 translated rows.

| Module | Rows | Translated | Partial | Stub |
|---|---:|---:|---:|---:|
| `audio::ax` | 399 | 398 | 1 | 0 |
| `audio::seq` | 263 | 263 | 0 | 0 |
| `boot::cache` | 1 | 1 | 0 | 0 |
| `boot::entry` | 6 | 5 | 1 | 0 |
| `boot::init_tables` | 1 | 1 | 0 | 0 |
| `boot::mem` | 3 | 3 | 0 | 0 |
| `engine::display` | 402 | 402 | 0 | 0 |
| `engine::dsys` | 397 | 397 | 0 | 0 |
| `engine::frame` | 15 | 15 | 0 | 0 |
| `engine::globals` | 47 | 47 | 0 | 0 |
| `engine::main_loop` | 203 | 198 | 5 | 0 |
| `engine::startup` | 4 | 4 | 0 | 0 |
| `game::actor` | 57 | 57 | 0 | 0 |
| `game::actors::dokan` | 5 | 5 | 0 | 0 |
| `game::bases` | 5,607 | 5,607 | 0 | 0 |
| `game::boot_scene` | 12 | 12 | 0 | 0 |
| `game::bosses` | 770 | 770 | 0 | 0 |
| `game::enemies` | 3,555 | 3,553 | 2 | 0 |
| `game::interaction` | 17 | 17 | 0 | 0 |
| `game::path` | 37 | 37 | 0 | 0 |
| `game::player` | 830 | 830 | 0 | 0 |
| `game::profile` | 36 | 36 | 0 | 0 |
| `game::scene` | 31 | 31 | 0 | 0 |
| `game::stage` | 1,226 | 1,225 | 1 | 0 |
| `game::task` | 16 | 16 | 0 | 0 |
| `game::worldmap` | 1,001 | 998 | 3 | 0 |
| `game::yoshi` | 7 | 7 | 0 | 0 |
| `render::geometry` | 50 | 50 | 0 | 0 |
| `render::gx` | 3 | 3 | 0 | 0 |
| `render::gx_band` | 345 | 342 | 3 | 0 |
| `resource::archive` | 747 | 747 | 0 | 0 |
| `resource::bres` | 261 | 256 | 5 | 0 |
| `resource::course` | 19 | 19 | 0 | 0 |
| `resource::registry` | 423 | 423 | 0 | 0 |
| `runtime::bt` | 695 | 695 | 0 | 0 |
| `runtime::cpp` | 361 | 361 | 0 | 0 |
| `runtime::dvd` | 536 | 536 | 0 | 0 |
| `runtime::ids` | 4 | 4 | 0 | 0 |
| `runtime::init` | 7 | 5 | 1 | 1 |
| `runtime::input` | 388 | 388 | 0 | 0 |
| `runtime::link` | 12 | 12 | 0 | 0 |
| `runtime::os` | 537 | 536 | 0 | 1 |
| `ui::hud` | 542 | 541 | 1 | 0 |
| `ui::lyt` | 221 | 221 | 0 | 0 |
| `ui::menu` | 151 | 151 | 0 | 0 |

Semantic total: **20,250 rows** (20,225 translated, 23 partial, 2 stub).
Staging rows: **0**.

## Untabled corpus (outside `functions.tsv`)

Wave 226 found 2,457 prologue-verified function starts in `main.dol` with no
tsv row. They are not part of the 20,252 and never enter
`ledger/functions.csv`; they carry `# Original (untabled)` and live in the
modules that own the referencing pointer table.

| Quantity | Count |
|---|---:|
| Corpus entries (frozen at wave 226) | **2,457** |
| With a translated body in `src` | **259** |
| ... exact `fn_<address>` name | 251 |
| ... suffixed or semantic name | 8 |
| Still without a body | **2,198** |
| Entries that are aliases of other slots | **371** |
| ... aliases spanning two or more runs | 367 |
| ... aliases among the 2,198 bodyless entries | 311 |
| Named by any `src` literal (upper bound) | 628 |
| Present in `functions.tsv` / the ledger | 0 / 0 |

The corpus closes: `259 + 2,198 = 2,457`. Aliases are slot reuse, not a third
bucket: one translated body serves every slot word that names it, and they are
already counted once among the 259. The 628 literal count is an upper bound
because a literal can be a prose mention.

Separately, `src` holds **583 untabled body blocks over 548 distinct
addresses**:

| Kind | Addresses |
|---|---:|
| Corpus entries | 259 |
| Other `main.dol` starts without a standard prologue | 288 |
| One REL address | 1 |

The biggest remaining runs are `0x80325428` (WorldMap vtable, 58 corpus entries
left), `0x80314068` (`dEn_c`, 52), `0x8032679C` (`WM_PLAYER_VTABLE`, 48),
`0x80313768` (33), `0x80303104` (31) and `0x80312750` (30). The complete map
and per-run tables are in `docs/UNTABLED_POINTER_TABLES.md`.

## What "complete" means here

- The Ghidra `functions.tsv` inventory is fully accounted for: 20,250 bodies
  plus two documented deferrals.
- Body completeness is 20,225 `translated`; the remaining 23 `partial` and 2
  `stub` rows are the documented residue above. They are not unstarted work.
- The untabled corpus is **not** complete: 2,198 of 2,457 entries still lack a
  body. That is the remaining translation mass.
- Resource formats and the Windows port are independent tracks; see
  `docs/PORT_WINDOWS.md`.

## Reproduce these numbers

```powershell
node tools/merge-ledger.mjs
node tools/verify-ledger.mjs
node tools/progress.mjs
node work/wave239/laneB_account.mjs
node work/wave238/laneF_accounting.mjs
```

`laneB_account.mjs` prints the tsv/ledger reconciliation, per-module counts,
status/confidence split and the two deferrals. `laneF_accounting.mjs` prints
the untabled 2,457-entry accounting. Both were run at the wave-239 freeze and
agree with this document. The current status residue is also tabulated in
`work/wave249/laneD_report.md` ("Final residue").
