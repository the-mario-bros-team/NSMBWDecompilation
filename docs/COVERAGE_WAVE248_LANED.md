# Coverage - Wave 248 lane D (machine-state / `va_list` residue)

## 1. Sampled half of the 52 partials; 6 converted

23 of the 52 `partial` rows were re-read from the shipped image. Five sampled
rows were genuinely complete and are now `translated` / `high`; their twin
`0x801EEA50` was converted on the same evidence.

| address | module | change |
|---|---|---|
| `0x801AF5D4` | `audio::ax` | literal deadline and `0x801ABEE0` operands; body fixed to store `*param_1`/`*param_2` and pass `[0x80000038]` on the non-zero arm |
| `0x80082240` | `game::stage` | paired-single ops are dead `f29..f31` spills |
| `0x8028E310` | `render::gx_band` | `psq_l ...,0,0` is GQR0 float; `r4` is only the `y` mask |
| `0x801EE9A0` / `0x801EEA50` | `runtime::input` | 16-argument `va_list` frame rebuilt explicitly and fed to `fn_802e1560` |
| `0x80015160` | `ui::hud` | `0x802EDDA0 + state*12` descriptor via `interface_call_desc` |

Ledger: **20,202 translated / 46 partial / 2 stub**.

## 2. `interface_call_desc` argument order was reversed

The helper called `fn_802dcbfc(descriptor, object)` although the function reads
the descriptor from its second parameter. This also made the 145 wave-246/247
forwarders dispatch with the receiver and descriptor swapped. It now delegates
as `fn_802dcbfc(object, descriptor)`.

## 3. 216 raw dispatcher calls remain in `translated` rows

`work/wave248/d_scan_calls.mjs` finds `platform::call3(FN_802DCBFC, b, b, a)` in
216 `translated` bodies (`engine::display` 17, `engine::dsys` 2,
`game::enemies` 153, `ui::hud` 44). They match the wave-247 forwarder shape but
pass the descriptor in `r5`, never `r12`. Six were disassembled and all are
`K = 0x18`. Follow-up: convert to `interface_call(a, b, K)`.

## 4. Residue of the 46 partials

Machine state 7, `va_list` overflow/descriptor 5, `bres` caller-owned
descriptors 5, `runtime::cpp` catalogue placeholders 14, removed blocks 4,
paired-single/missing mode 3, jump tables 2, and one each of caller stack slot,
multi-helper, literal descriptor, `f1`+out-pointer adapter, extra dispatcher
arguments and an undocumented `ui::hud` row. Details:
`work/wave248/laneD_report.md`.

## Verification

`cargo check` 0; `doc-lint` 0 over 68 files; `scripts/check.ps1` `OK`
(`verify-ledger --enforce-bodies` clean).
