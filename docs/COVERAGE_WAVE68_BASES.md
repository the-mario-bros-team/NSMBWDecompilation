# Coverage - Wave 68 (`game::bases`, `0x8065` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** from the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader picks the real load address. Byte counts come from
`work/ghidra/rel-d_basesNP/export/functions.tsv`; the cited evidence files are
the Ghidra C exports under `work/ghidra/rel-d_basesNP/export/`. Confidence is
`high` when the guards, stores, constants and call arguments are literal, and
`medium` when a field role, a callee identity or an argument dropped by Ghidra
had to be inferred (the body then carries `// observed:`, `// inferred:` or
`// unknown:` notes).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 27 | 2636 | 14 | 13 |

## What this pass covers

- **Destructor instantiations, `0x80650020..0x80653A20`** — nine bodies that
  share the two registered-destructor shapes already recorded for the
  `daBigShell` family: interface teardown `0x8015F800` plus free `0x802B9280`
  ([`fn_80650020`], [`fn_80651f60`], [`fn_806525e0`], [`fn_80653490`]), the
  helper teardown `0x800DAE00` variant [`fn_80650b00`], the base destructor
  `0x800642B0` plus free `0x80162920` pair [`fn_806529c0`] / [`fn_80653a20`],
  and the two free-only copies [`fn_80650350`] / [`fn_80650390`] whose
  `0x802B9280` argument Ghidra drops.

- **Small constructors, `0x806501A0..0x806537E0`** — the `0x400`-byte
  allocator wrapper [`fn_806501a0`] around the already-translated
  [`fn_806501d0`], the `0x430`-byte class object [`fn_806520e0`] with its
  adapter tick [`fn_806521f0`], and the two plain `0x398`-byte actors
  [`fn_80652760`] / [`fn_806537e0`] that differ only in the `+0x60` vtable.

- **Transform and utility bodies, `0x80651990..0x80653C50`** — the 40-record
  global spawn ring reset [`fn_80651990`], the position/angle transform writer
  [`fn_80652230`], the `g3d/Copyrights.brres` loader [`fn_80652340`], the
  player-speed height step [`fn_80652410`], the constant getter
  [`fn_806534f0`], the `+0x30` child-chain walk [`fn_806535e0`], the
  `:`-suffix name compare [`fn_806536c0`], and the two orientation-block
  writers [`fn_80653c20`] / [`fn_80653c50`].

- **Event and query bodies, `0x80654240..0x80654700`** — the kind-`0x1A2`
  event handler [`fn_80654240`], the ground-surface gate [`fn_80654580`], the
  rider-chain scan [`fn_80654670`], and the `+0x5A0` timer reset
  [`fn_80654700`].

- **Recurring layout.** The constructors repeat the two base-constructor
  chains already recorded for this bucket: `0x80064110` with the vtable at
  `+0x60` (size `0x398`), and `0x80066FC0` with the interface vtable at `+0x394`
  (sizes `0x3E0`..`0x430`). The actor vtable-at-`+0x60` dispatch pattern uses
  the existing `actor_method0` helper; the embedded-object dispatch uses
  `vmethod0`/`vmethod1`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80650020` | `fn_80650020` | 88 | high |
| `0x806501A0` | `fn_806501a0` | 48 | medium |
| `0x80650350` | `fn_80650350` | 64 | medium |
| `0x80650390` | `fn_80650390` | 64 | medium |
| `0x80650B00` | `fn_80650b00` | 88 | high |
| `0x80651990` | `fn_80651990` | 184 | medium |
| `0x80651F60` | `fn_80651f60` | 88 | high |
| `0x806520E0` | `fn_806520e0` | 100 | medium |
| `0x806521F0` | `fn_806521f0` | 48 | high |
| `0x80652230` | `fn_80652230` | 220 | medium |
| `0x80652340` | `fn_80652340` | 176 | high |
| `0x80652410` | `fn_80652410` | 60 | medium |
| `0x806525E0` | `fn_806525e0` | 88 | high |
| `0x80652760` | `fn_80652760` | 76 | high |
| `0x806529C0` | `fn_806529c0` | 88 | high |
| `0x80653490` | `fn_80653490` | 92 | high |
| `0x806534F0` | `fn_806534f0` | 12 | medium |
| `0x806535E0` | `fn_806535e0` | 224 | medium |
| `0x806536C0` | `fn_806536c0` | 136 | medium |
| `0x806537E0` | `fn_806537e0` | 76 | high |
| `0x80653A20` | `fn_80653a20` | 88 | high |
| `0x80653C20` | `fn_80653c20` | 40 | high |
| `0x80653C50` | `fn_80653c50` | 24 | high |
| `0x80654240` | `fn_80654240` | 180 | medium |
| `0x80654580` | `fn_80654580` | 92 | medium |
| `0x80654670` | `fn_80654670` | 136 | medium |
| `0x80654700` | `fn_80654700` | 56 | high |

## Address range

`0x80650020..0x80654700`, in short runs:

- `0x80650020..0x80650B00` — five bodies, interrupted by the jump stubs
  `0x80650140`, `0x80650170`, `0x80650810`, `0x80650840` and by the rejected
  `0x806505B0`;
- `0x80651990..0x806529C0` — nine bodies, interrupted by the already-translated
  `0x806501D0..0x80651910` wave-3/4 rows, the rejected `0x80651590`,
  `0x80651640`, `0x80651A60` and `0x80652310`;
- `0x80653490..0x80653C50` — nine bodies, interrupted by the jump stubs
  `0x80653780`, `0x806537B0` and the rejected `0x80653CE0`;
- `0x80654240..0x80654700` — four bodies, interrupted by `0x80654600`.

At lane start the `0x8065` bucket held 185 functions, 24 of them already
translated (`0x806501D0..0x80651910`, `0x80652C70`, `0x80654300`, `0x806574D0`,
`0x8065B660`). This pass adds 27, leaving 134 for later waves.

## Deliberate skips

- `0x80650140` (48), `0x80650170` (48), `0x80650810` (48), `0x80650840` (48),
  `0x80652080` (48), `0x806520B0` (48), `0x80652700` (48), `0x80652730` (48),
  `0x80653780` (48), `0x806537B0` (48) — the external jump stub `0x802DCBFC`
  (`func_0x802dcbfc(param_2, param_2, param_1)`) with a hidden `r12`
  this-adjust that `platform::call3` cannot model. Same rejection recorded for
  `0x80704E00`, `0x80704E30` and `0x80705510` in wave 67; the two wave-3 rows
  `0x806512F0`/`0x80651320` stay the only `partial` entries for this stub.
- `0x806505B0` (80) — calls `0x8006A230` twice with every argument dropped and
  converts `uVar1 & 0xFFFF` through the `0x43300000` double bias; the two
  call results cannot be told apart, so the value being scaled is invented.
- `0x80651590` (24) — truncated by bad instruction data (`halt_baddata`); only
  the `+0x3D8` guard survives.
- `0x80651640` (384) — the outlined paired-single block
  (`unaff_GQR0`, `ldexpf`, the `bVar == 4 || bVar == 6` lane split, incoming
  `in_f27..in_f31`). Same decompiler failure class already rejected for
  `0x806914D0`, `0x806F0F20` and `0x80740370`.
- `0x80651A60` (452) — shifts the six-record global ring at `0x80828430`
  through an inline copy loop whose record stride is only recoverable from raw
  bytes; the two `(&DAT_80828438)[index * 3]` reads in the C are not a faithful
  picture of the real `0x60`-byte move. Deferred rather than guessed.
- `0x80652310` (44) — truncated by bad instruction data.
- `0x80653B60` (88), `0x80653BC0` (88) — the `0x8006CED0` -> `0x8006A4B0` ->
  `0x80198C30` sound-cue pair with ids `0x15F`/`0x160`; the two stack buffers
  are declared `auStack_20[28]` and `auStack_28[8]` by frame layout only, so
  their real sizes are not in the evidence.
- `0x80653CE0` (124) — indexes the float table at `0x807C8848` with
  `countLeadingZeros` of a comparison result; the table stride and the
  `+0x70`/`+0x74` role split would be guesses.
- `0x80654600` (108) — writes `DAT_807C8860` into the stack slot next to the
  `0x8006CED0` output after the call, but the printed six-argument call does
  not consume it; the unmodelled seventh argument makes an honest body
  impossible.
- The large bodies later in the bucket (`0x80654B90` 672 B, `0x8065CB20`
  3788 B, `0x8065DD50`/`0x8065F090` 1772 B, `0x8065F910` 2204 B, and the
  `0x8065B`/`0x8065C` families) are untried, not rejected; they belong to
  later waves.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (re-read, not copied from
  prose).
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000` line.
- No address was already in `ledger/functions.csv` (checked across the whole
  `0x8065` bucket before writing).
- `node tools/check-lane-csv.mjs wave68` reports `OK` with
  `wave68-bases.csv: 27 rows`.
- `cargo check --message-format=short` produces no error for
  `src/game/bases.rs`.
