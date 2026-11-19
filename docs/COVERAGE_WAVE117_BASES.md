# Coverage - Wave 117 (`game::bases`)

## How to read this file

This page documents the Wave 117 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`** produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
cited file is a real Ghidra C export under
`work/ghidra/rel-d_basesNP/export/`, and every byte count was re-read from
`work/ghidra/rel-d_basesNP/export/functions.tsv`.

"Confidence" is `high` when the guards, stores, constants and call arguments are
literal, and `medium` when a callee identity, a lost argument or a lost receiver
had to be inferred. No body in this pass is `low`. Comments in the bodies keep
`// observed:`, `// inferred:` and `// unknown:` markers.

The `0x8074` bucket holds 233 functions. Before this pass 124 were already
translated, leaving 109. This pass takes 27 of them from one contiguous sweep
between `0x807476B0` and `0x80749F60`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 27 | 3,876 | 18 | 9 |

## What this pass covers

- **Rectangle-gated spawner (`0x807476B0..0x80747E20`).** Nine bodies of the
  `0x3F0`-byte object: the destructor twin [`fn_807476b0`] / [`fn_80747e20`],
  the constructor [`fn_80747830`], the `0x8008E890` rectangle refresh
  [`fn_80747940`], the four-slot "all masked players inside" scan
  [`fn_807479e0`], the closed-rectangle point test [`fn_80747a80`], the two
  timer reloads [`fn_80747ae0`] / [`fn_80747af0`] and the per-frame gate
  [`fn_80747c30`]. Shared fields: player mask `+0x3D4`, wait timer `+0x3D0`,
  rectangle `+0x3D8..+0x3E4`, vtable `+0x60`.
- **Character-matched child spawner (`0x80747FA0..0x80748D50`).** Eleven
  bodies of the `0x3A0`-byte object: the constructor [`fn_80747fa0`], the
  update wrapper [`fn_807482c0`], the character lookup [`fn_807486e0`], the
  release scan [`fn_807489c0`], the routing [`fn_807484c0`] and the
  inside/outside transition [`fn_80748510`], the `0xE0` spawn
  [`fn_80748750`], the destructor [`fn_80748840`], the rectangle test
  [`fn_80748a50`] and the proximity cue [`fn_80748d50`] that queues sound
  `0x1BD`. Shared fields: child handle `+0x398`, countdown `+0x39C`/`+0x39E`,
  character byte `+0x349`, rectangle `+0x3D4..+0x3E4`, counter `+0x3E8`.
- **`0x420`-byte owner (`0x80749000..0x80749250`).** The destructor
  [`fn_80749000`], the constructor [`fn_80749180`] (vtable `0x80811358`) and
  the sub-object virtual [`fn_80749250`].
- **Eight-way player-selection switch (`0x807499D0..0x80749F60`).** The
  position switch [`fn_807499d0`], the three axis probes [`fn_80749d00`] /
  [`fn_80749de0`] / [`fn_80749e90`] and the per-frame driver
  [`fn_80749f60`] that sorts the list, picks a mode, builds the position and
  spawns type `0x118`.

Start with [`fn_80747c30`] for the rectangle gate, [`fn_80748510`] for the
child transitions and [`fn_807499d0`] for the selection switch.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x807476B0` | `fn_807476b0` | 88 | high |
| `0x80747830` | `fn_80747830` | 76 | high |
| `0x80747940` | `fn_80747940` | 152 | high |
| `0x807479E0` | `fn_807479e0` | 160 | high |
| `0x80747A80` | `fn_80747a80` | 96 | high |
| `0x80747AE0` | `fn_80747ae0` | 12 | high |
| `0x80747AF0` | `fn_80747af0` | 56 | high |
| `0x80747C30` | `fn_80747c30` | 116 | medium |
| `0x80747E20` | `fn_80747e20` | 88 | high |
| `0x80747FA0` | `fn_80747fa0` | 76 | high |
| `0x807482C0` | `fn_807482c0` | 40 | medium |
| `0x807484C0` | `fn_807484c0` | 80 | medium |
| `0x80748510` | `fn_80748510` | 188 | medium |
| `0x807486E0` | `fn_807486e0` | 104 | high |
| `0x80748750` | `fn_80748750` | 240 | medium |
| `0x80748840` | `fn_80748840` | 88 | high |
| `0x807489C0` | `fn_807489c0` | 140 | high |
| `0x80748A50` | `fn_80748a50` | 196 | high |
| `0x80748D50` | `fn_80748d50` | 316 | medium |
| `0x80749000` | `fn_80749000` | 88 | high |
| `0x80749180` | `fn_80749180` | 76 | high |
| `0x80749250` | `fn_80749250` | 48 | medium |
| `0x807499D0` | `fn_807499d0` | 560 | high |
| `0x80749D00` | `fn_80749d00` | 224 | medium |
| `0x80749DE0` | `fn_80749de0` | 176 | high |
| `0x80749E90` | `fn_80749e90` | 176 | high |
| `0x80749F60` | `fn_80749f60` | 216 | medium |

**Totals:** 27 translated, 0 partial, 0 stub (18 high, 9 medium, 3,876 bytes).

## Deliberate skips

Read and rejected, with the reason:

- **Mangled paired-single bodies.** `0x80740370`, `0x80747B30`, `0x807485D0`,
  `0x80748B20` and `0x80749750` all carry Ghidra's GQR reconstruction
  (`unaff_GQR0` plus `ldexpf`, and in two of them `in_f31`/`in_ps31_1` payload
  registers that never reach a named parameter). The spawn constants and call
  sites are visible, but the position arithmetic cannot be rebuilt without
  guessing which paired slot holds which float, so the bodies that call them do
  so by address and say so. [`fn_80747c30`] consumes only the `r3` result of
  `0x80747B30`, which the export itself tests; [`fn_80748d50`] calls
  `0x80748B20` with no observable result; [`fn_80749f60`] stores the `r3`
  result of `0x80749750` at `+0x3D4` exactly as the C does.
- **Receiver-less prologue fragments.** `0x807481B0` recovers its object through
  `0x802DCD74`, a five-register save stub that stores `r27..r31`; the export
  prints its return as an `undefined8` whose halves are the actor and the loop
  count, so the count is unrecoverable. `0x807492B0` builds its three-float
  argument through `0x80749310`, which is itself a call to the `0x802DCC24`
  save stub with no visible body. `0x807481B0` is called by [`fn_807482c0`] and
  `0x807492B0` by [`fn_80749f60`], both by address.
- **`0x80749C50` blend.** The first of its two `0x80749C00` calls loses both the
  owner and the player index, so only one of the two blended positions has an
  evidenced source. Mode `7` of [`fn_807499d0`] calls it by address instead.
- **`0x802DCBFC` thunks.** `0x807477D0`, `0x80747800`, `0x80747F40`,
  `0x80747F70`, `0x80749120` and `0x80749150` only forward two unnamed
  arguments to the unresolved indirect-jump stub `0x802DCBFC`; `0x80741CA0` is
  the four-byte inventory thunk `thunk_FUN_80741a70`. The module header already
  records that stub as unresolved, so a body here would be a signature guess.
- **The rest of the bucket.** The 82 other untranslated addresses in `0x8074`
  were not translated in this pass; they remain for a later sweep.

## Notes on inference

- No class name survives in these exports. The two spawner objects are
  identified only by allocation size and vtable (`0x3F0`/`0x80810DC8` and
  `0x3A0`/`0x80810F2C`); the prose never presents either as a named class.
- `0x8005F900` is the same player-index lookup already modelled by
  [`fn_807495b0`] and [`fn_80749c00`]; `0x80429CA8` is the same active-player
  mask used there. Both keep their raw addresses.
- [`fn_80748510`] calls `0x807485D0` without arguments in the export. The two
  arguments are restored as `(actor, player)` because that skipped body reads
  its first argument's `+0x394` rectangle and its second argument's
  `+0xAC`/`+0xB0` position; the doc block marks this `// inferred:`.
- [`fn_807484c0`] calls `0x807486E0` without an argument. It is restored as
  `actor` because that callee compares `+0x349` of its argument with the same
  field, which only `actor` supplies.
- [`fn_807499d0`] and [`fn_80749f60`] show two callees declared `void` whose
  `r3` is nonetheless consumed (`0x807498C0` as the selection mode and
  `0x807492B0` as the spawned object). The Rust keeps the register-shaped call
  through the address and records the contradiction in the doc block rather
  than inventing a prototype.
- `0x80748750` uses the `0x43300000`/`^ 0x80000000` construction on the sign
  table `0x8042C1A0`; it is written as the equivalent `f64::from_bits` pattern
  already used by [`fn_80651830`], not as a new symbol.
- [`fn_80748d50`] and [`fn_80749d00`] both use doubles in the C only because of
  the `ABS`/probe prototypes; the comparisons are kept at the precision the C
  shows (`f64` around the `ABS` results, `f32` for the coordinates).

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (27 rows, 3,876 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 124
  translated addresses were re-read first and none is duplicated here. The file
  now holds 2,938 bodies.
- `node tools/check-lane-csv.mjs wave117` reports `OK` and names no problem in
  `wave117-bases.csv`.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/game/bases.rs`; diagnostics raised by other lanes' files are ignored.
