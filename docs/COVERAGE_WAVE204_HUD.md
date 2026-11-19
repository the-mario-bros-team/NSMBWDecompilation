# Coverage - Wave 204 (`ui::hud`, main.dol part 2)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/hud.rs` | `ui::hud` | 60 | 11,720 | 47 | 13 |

Second 60 main.dol hud bodies, `0x80024150`-`0x8002AC40`.

## Corrections and recoveries

- **Three distinct 12-byte accessor families, not one.**
  `0x80026BB0` returns `0x80377A04`; `0x8002A800..0x8002A850` is a descending
  `0x40`-stride BSS run (`0x8035334C`, `0x8035338C`, `0x8035330C`,
  `0x803532CC`, `0x8035328C`, `0x8035324C`); `0x800268B0`/`0x800268C0` return
  `0x80357BF4`/`0x80357BB4`.
- **Manifest sizes corrected from `blr`+prologue extents:** `0x80025800` = 508
  (not 16), `0x80024E80` = 132 (not 16), `0x80028780` = 424 (not 20),
  `0x80028930` = 560 (not 20). The 16/20 values are `psq_st f31` prologue
  artefacts in `functions.tsv`.
- `0x8002AC40` is `f32` code; Ghidra's integer casts are a mis-read.
- `0x8002AC00`'s Ghidra signature drops a real pass-through argument.
- `0x80026A90`'s asymmetry is literal: the needle advances only on a found
  `':'`, the candidate advances unconditionally.
- `0x8015F470` is a `b 0x8015F480` thunk (call the real entry).
- `0x802EE4F0` is an `f32` table `0.0, 90.0, 10.0, 80.0`.
- `r13-0x7E28` (`0x80427858`) is the inline SDA string `"pakkun"`.
- `0x800270F0` is byte-identical to `0x8001D850`; the existing Rust loop for
  that body recurses one link too deep and probes with `call0_r` instead of the
  child in `r3` - flagged for a later fix round.
- Lane B added `call_f32_arg_u32`/`call_u32_f32`; lane D added
  `call_2f_1u`/`call_f32_1u`/`call_f32_3u` because `hud.rs` lacked them.
- Duplicate `FN_80168D80`/`FN_80072BC0` definitions across lanes were removed
  at merge; lane A suffixed its own duplicates instead.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations; `scripts/check.ps1` exit 0.
- `check-lane-csv.mjs wave204`: 60 rows, 60 unique, 47 high + 13 medium.
- Ledger: 19,149 rows (94.55%); `ui::hud` 312 translated.
