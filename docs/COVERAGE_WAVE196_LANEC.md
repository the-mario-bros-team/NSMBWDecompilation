# Wave 196 lane C coverage (`game::bases`)

Slice: the final `0x807BDF20..0x807C3664` block of `d_basesNP` (REL module 2).
All nine assigned bodies are translated; nothing was deferred.

## Translated

| Address | Bytes | Rust body | Summary |
| --- | --- | --- | --- |
| `0x807BDF20` | 260 | `fn_807bdf20` | Mounts `worldSelectMapGuide.arc`, loads `worldSelectMapGuide_01.brlyt` through layout slot `+0x14`, binds the `inWindow`/`A00_guide`/`T_worldNum_00`/`P_pochi_00`/`N_wCCGuide_00`/`T_titleWorld_00` sets; returns `true` when the accessor was created. |
| `0x807BEAF0` | 280 | `fn_807beaf0` | Ten-way state dispatcher on `+0x604`, then `fn_807c1220`, then the `+0x6C0` latch and the `fn_807c1380` choice of the `1.0`/`0.3` trail weights. |
| `0x807BF5A0` | 260 | `fn_807bf5a0` | Rebuilds both position triples from `+0x6FC + 2400.0` and `0.7111111 * 28.0`; sine feeds the y pair, cosine the z pair. |
| `0x807BFD00` | 276 | `fn_807bfd00` | Promotes state `0` to `1`, measures the anchor against `+0x4BC` through `0x80160AE0`, and releases the ride when `|distance| <= FLT_EPSILON`. |
| `0x807C07A0` | 260 | `fn_807c07a0` | Decays the aim offset and `+0x6F8 → +0x6FC` by `0.8`; when the scaled length is `< 2.0` and the frame gap `<= +0x73C`, resets the offset from `0x80377DC8` and returns `1`. |
| `0x807C0C10` | 276 | `fn_807c0c10` | Solves the crossing of the travel and aim lines (`m1`, `m2`, `c1`), stores `+0x710`/`+0x718`/`+0x714`, and returns `z - +0x4B8`. |
| `0x807C0EB0` | 288 | `fn_807c0eb0` | Steps `+0x6F8` with `0x8015F140(…, 0.4, 100.0, 0.5, &+0x6FC)`; on convergence either completes the ride (`+0x604 = 9`, display byte `0x0F`, return `1`) or copies `+0x4B0 → +0x710` and adds the `0x802BE470` length. |
| `0x807C1220` | 260 | `fn_807c1220` | Aim-speed ramp on `+0x724` (mode `0`: `+1.0` per 10 frames, cap `8.0`; mode `1`: `-+0x728`, floor `0.0`), then three `+0x724 * rand()` offsets. |
| `0x807C3540` | 292 | `fn_807c3540` | Pad-edge poll on the `0x80429F50`/`0x8042A464` record (`8`/`4` bits), `0x900`-byte allocation through `0x8042A468` slot `0x20`, `0x800B53A0(0)` fallback, and the `+0x70` pane slot `+0x18` event `0x80834B00`. |

## Notes on the raw words

- `0x807BEAF0`'s jump table at `0x8082187C` was read directly: case `3` sends to
  `0x807BFB20` and case `4` to `0x807BF930` (the source order in the
  disassembly is not the case order). The table, not Ghidra's listing, is the
  authority; both agree.
- `0x807BFD00`/`0x807C07A0`/`0x807C0EB0`/`0x807C1220` branch folds were decoded
  from `bc` BO/BI and `cror` fields: `bc 4,x` is *branch if bit clear*, and
  `cror cr0[EQ], cr0[LT], cr0[EQ]` folds `<=`. This makes the `FLT_EPSILON`,
  `2.0`, `+0x73C`, `> 0` and `> 8.0` tests match Ghidra's `<`/`<=` reading.
- `0x807BF5A0`/`0x807C0C10` show `ldexpf`/`GQR` lines in the export; they are
  the `psq_st`/`psq_l` scalar (`GQR0`) float saves, not arithmetic.
- `0x807C0C10` and `0x807C0EB0` are typed `double`/`void` by Ghidra. The
  existing callers settle the ABI: `fn_807bffa0` reads `0x807C0C10` with
  `call_f32_1`, and `fn_807c0d30` documents the same. `0x807C0EB0` returns its
  `1`/`0` in `r3` with no float result.

## Reused neighbours

- Direct calls: `fn_807bf6b0`, `fn_807bf930`, `fn_807bfb20`, `fn_807bfbe0`,
  `fn_807bfe20`, `fn_807bec10` (weight pointer), `fn_807c1190`,
  `fn_807c1380`, `fn_807c3b10`.
- Address-only callees (no Rust body): `0x800C8CB0`, `0x800C8FB0`,
  `0x800C90F0`, `0x800C8EB0`, `0x800C8E40`, `0x800C8D60`, `0x800C8F20`,
  `0x80007220`, `0x80160AE0`, `0x801C1650`, `0x8015F140`, `0x801C14C0`,
  `0x802BE470`, `0x800B2F00`, `0x800B53A0`, `0x807BF760`, `0x807BF860`,
  `0x807BFC20`.

## Deferred

None.

## Verification

The block was appended to a temporary copy of `src/game/bases.rs` and
`cargo check --lib` passed with no new errors or warnings (the only diagnostics
are the pre-existing ones in the untouched file).
