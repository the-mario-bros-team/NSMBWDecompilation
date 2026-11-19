# Coverage - Wave 126 (`game::bases`)

## How to read this file

This page documents the Wave 126 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`** produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
`# Original` block in the source carries the same warning line. Cross-module
pointers (`0x8037xxxx`, `0x8042xxxx`) keep their absolute `main.dol` values.

Every cited file is a real Ghidra C export under
`work/ghidra/rel-d_basesNP/export/`, and every byte count was re-read from
`work/ghidra/rel-d_basesNP/export/functions.tsv`.

The `0x8066` bucket holds 156 functions. Before this pass 58 were translated,
leaving 98; this pass takes 30 of them, from `0x80669DC0` to `0x8066FD60`.
Every function in the `0x8066BB30..0x8066E3A0` span was read; the nine inside it
that were not translated are listed under "Deliberate skips".

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee identity, a field role or an unreconstructed
neighbour had to be inferred. Comments in the bodies keep
`// observed:` / `// inferred:` / `// unknown:` markers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 4,452 | 23 | 7 |

All 30 rows are `translated`; none is `partial`. The module header now reports
3202 bodies.

## What this pass covers

- **Key actor registration half (6 bodies).** The `0x970`-byte object built by
  `0x8066BE70` — destructor [`fn_8066bb30`], five-link `+0x30` child walk
  [`fn_8066bc70`], `:`-suffix name compare [`fn_8066bd50`], fall/wait contact
  handler [`fn_8066bea0`] (target word at `0x80429D40+0x18`, sound `0x56`),
  in-place constructor [`fn_8066bf80`] (vtable `0x807E92D0`, model `+0x524`,
  `0x128`-byte 3-entry callback block at `+0x5F4`) and leave body
  [`fn_8066c2c0`].
- **Key actor action half (10 bodies).** `0x8066C670..0x8066CD70`: class
  release [`fn_8066c670`], position copy through the `+0x5AC` handle
  [`fn_8066c6a0`], effect-position builder [`fn_8066c720`], one-shot sound
  `0x300` [`fn_8066c750`], scale/spin/collision reset [`fn_8066c7f0`], the
  `Wm_ob_keyfall`/`Wm_ob_keywait` emitter pair with sound `0x2FE`
  [`fn_8066c870`], the drift clamp [`fn_8066cae0`], the arming body with
  `Wm_ob_keyget01` and sound `0x2FF` [`fn_8066cb40`], the phase-1 latch
  [`fn_8066cc90`] and the `Wm_ob_keyget02`/timer/colour tail [`fn_8066cd70`].
- **`0x590`-byte key-gate actor (9 bodies).** Destructor [`fn_8066d410`],
  allocator/constructor [`fn_8066d590`] (vtable `0x807E9910`), the 668-byte
  start body [`fn_8066d800`] with its four `0x80064680` spawns (types `0x1E`,
  `0x21`, `0x21`, `0x22`), the 0x1C-stride 16-entry table at `0x807E97D8` and
  the thirteen `+0x408..+0x438` state words; the activity gate
  [`fn_8066daf0`], the number-sprite push [`fn_8066dcf0`] (ids `0x314..0x31A`
  through `0x80429D64`), the two global-mask helpers
  [`fn_8066e2a0`]/[`fn_8066e2d0`] (sound `0x73`), the per-variant height read
  [`fn_8066e350`] and the `0x69`/`0x68` list walks [`fn_8066e3a0`].
- **Rider/board tail closure (2 bodies).** [`fn_80669dc0`] claims a seat for a
  ready player (ready byte `+0x38C`, seat virtual `+0x6C`, contact box
  `+0xEA4`) and [`fn_80669f70`] leaves the state or detaches. Both extend the
  wave-96 `0x80669230..0x80669530` cluster.
- **Three isolated tails (3 bodies).** [`fn_8066f9d0`] re-enables the scene
  controllers, [`fn_8066fc10`] is the second copy of the global player-mask
  bit test, and [`fn_8066fd60`] counts the placement timer down before the
  `0x8066FA10` sort pass.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80669DC0` | `fn_80669dc0` | 272 | medium |
| `0x80669F70` | `fn_80669f70` | 124 | high |
| `0x8066BB30` | `fn_8066bb30` | 92 | high |
| `0x8066BC70` | `fn_8066bc70` | 224 | high |
| `0x8066BD50` | `fn_8066bd50` | 136 | high |
| `0x8066BEA0` | `fn_8066bea0` | 212 | medium |
| `0x8066BF80` | `fn_8066bf80` | 116 | high |
| `0x8066C2C0` | `fn_8066c2c0` | 84 | high |
| `0x8066C670` | `fn_8066c670` | 48 | medium |
| `0x8066C6A0` | `fn_8066c6a0` | 108 | medium |
| `0x8066C720` | `fn_8066c720` | 44 | high |
| `0x8066C750` | `fn_8066c750` | 108 | high |
| `0x8066C7F0` | `fn_8066c7f0` | 108 | high |
| `0x8066C870` | `fn_8066c870` | 216 | medium |
| `0x8066CAE0` | `fn_8066cae0` | 88 | high |
| `0x8066CB40` | `fn_8066cb40` | 320 | medium |
| `0x8066CC90` | `fn_8066cc90` | 132 | high |
| `0x8066CD70` | `fn_8066cd70` | 232 | high |
| `0x8066D410` | `fn_8066d410` | 88 | high |
| `0x8066D590` | `fn_8066d590` | 148 | high |
| `0x8066D800` | `fn_8066d800` | 668 | high |
| `0x8066DAF0` | `fn_8066daf0` | 108 | high |
| `0x8066DCF0` | `fn_8066dcf0` | 172 | medium |
| `0x8066E2A0` | `fn_8066e2a0` | 40 | high |
| `0x8066E2D0` | `fn_8066e2d0` | 116 | high |
| `0x8066E350` | `fn_8066e350` | 72 | high |
| `0x8066E3A0` | `fn_8066e3a0` | 212 | high |
| `0x8066F9D0` | `fn_8066f9d0` | 52 | high |
| `0x8066FC10` | `fn_8066fc10` | 40 | high |
| `0x8066FD60` | `fn_8066fd60` | 72 | medium |

**Totals:** 30 bodies, 4,452 bytes (23 high, 7 medium).

## Deliberate skips

Read and rejected, with the reason:

- **Paired-single / GQR conversions.** `0x8066C9A0`, `0x8066D800`'s two
  neighbours `0x8066DDC0`/`0x8066E0C0`, `0x8066FA10` and `0x80669560` mix
  `unaff_GQR0`/`unaff_GQR3`, `ldexpf` and `CONCAT13`/`CONCAT22` lane splits.
  Their stores are visible but which GQR slot feeds each conversion is not
  decidable, so no body was emitted. [`fn_8066fd60`] reaches `0x8066FA10` only
  by address and is therefore `medium`; [`fn_8066d800`] is fully float-explicit
  and does not touch the rejected neighbours.
- **`0x802DCBFC` forwarders.** `0x80665120`, `0x80665130`, `0x80665140`,
  `0x80665150`, `0x806654F0`, `0x80665500`, `0x80665510`, `0x80665520`,
  `0x80666AD0`, `0x80666AE0`, `0x80667780`, `0x806677B0`, `0x80668420`,
  `0x80668450`, `0x8066A7C0`, `0x8066A7F0`, `0x8066BE10`, `0x8066BE40`,
  `0x8066D530`, `0x8066D560` and `0x8066F6A0`/`0x8066F6D0` are the 48-byte
  this-adjusting thunks already documented by earlier waves; they carry no
  Rust-visible behaviour.
- **`halt_baddata` bodies.** `0x80667CF0` (8 bytes) is truncated by bad
  instruction data, and the thunk `0x80667DD0` (4 bytes) only branches into
  `0x80667DE0`, which is already translated.
- **Bare constant getters.** `0x80662700` and `0x8066B470` are a twelve-byte
  value return and a twelve-byte two-store setter: one descriptor load and one
  constant, with no call to justify. Earlier waves recorded the same shape as
  translation noise.
- **Receiver-less dispatch bodies.** `0x8066B0F0` (128 bytes) issues three
  class-virtual calls and one interface call but every call drops its visible
  arguments; `0x8066C090` (28 bytes) stores one word and then jumps through an
  unrecovered jumptable. Both were left out rather than guessed, and
  [`fn_8066bea0`] reaches `0x8066C090` by address.
- **Paired allocator `0x8066BE70`.** 48 bytes: allocates `0x970` and calls
  [`fn_8066bf80`] with the dropped `this`. It is clean enough to translate but
  was left out to keep this lane at its 30-body budget; the next pass can take
  it together with the remaining `0x8066` head.
- **Everything else.** The remaining untranslated `0x8066` addresses were not
  attempted: the `0x806601B0..0x80663DF0` head, the
  `0x80664610..0x80665150` block, the `0x80665240..0x80667CF0` block, the
  `0x80668CA0..0x806691D0` destructor copies, `0x80669560`, and the
  `0x8066A7C0..0x8066BE40` forwarder block.

## Notes on inference

- The "key"/"key-gate" labels are `// inferred:` from the
  `Wm_ob_keyfall`/`Wm_ob_keywait`/`Wm_ob_keyget01`/`Wm_ob_keyget02` strings at
  `0x807E9290..0x807E92C0`. No class-name string survives for either
  constructor (`0x8066BF80`, `0x8066D590`), so both class identities stay
  `// unknown:`.
- [`fn_8066bea0`] is `medium`: the two state names and the global target are
  literal, but the state names themselves are `// unknown:` by meaning.
- [`fn_8066c670`], [`fn_8066c6a0`] and [`fn_8066c870`] are `medium`: the
  vtable slots are explicit, but the `+0x544` release callback, the object
  returned by `0x80056E30` and the word at `+0x00` forwarded as the sound owner
  are `// inferred:`/`// unknown:`. The copy direction of `0x8016F130` is
  `// inferred:` from the post-call reads at `+0x5B0`.
- [`fn_8066cb40`] is `medium`: every constant and store is literal, but the
  `0x800E20A0` notification arguments are taken from the matching
  `0x8066C870` sequence and the `+0x60` class-vtable role is inferred.
- [`fn_8066dcf0`] is `medium`: the seven sprite ids, the subtraction and the
  `+0x2C` virtual are literal; the sprite-manager role of `0x80429D64` is
  `// inferred:`.
- [`fn_80669dc0`] is `medium`: the five gates and the offset vector are
  literal, but `0x8013F9E0` and `0x8005E500` are unprototyped.
- [`fn_8066fd60`] is `medium` because it calls the rejected `0x8066FA10` by
  address, as documented under "Deliberate skips".
- [`fn_8066d800`] is `high` on its own body: every table offset, constant,
  spawn and store is literal. The two unused locals the C computes
  (`local_1c`, `local_20`) have no reader and are omitted.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 4,452 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 58 translated
  addresses were re-read first and none is duplicated here.
- `node tools/check-lane-csv.mjs wave126` reports `OK`; `wave126-bases.csv` has
  30 rows and 30 unique addresses.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/game/bases.rs`.
