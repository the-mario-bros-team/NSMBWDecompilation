# Coverage - Wave 146 (`game::bases`)

## How to read this file

This page documents the Wave 146 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`**; the Wii REL loader chooses
the real load address, so these numbers identify positions inside the REL image,
not runtime addresses. Every `# Original` block in the source carries the
`Synthetic base: 0x80600000; this is not a runtime address.` line.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee contract, a field role or a virtual dispatch
had to be named rather than resolved. Comments in the bodies keep
`// observed:` / `// inferred:` / `// unknown:` markers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 13 | 7,964 | 4 | 9 |

All 13 rows are `translated`; none is `partial`. The module header now reports
3657 bodies (3644 + 13), and the `0x8064` planner bucket drops from 77 to 72
untranslated addresses.

## What this pass covers

- **The eight "ready" rows of `docs/SPEC_WAVE145_BASES.md`** (7,448 bytes), each
  re-decoded from `work/rels/elf/d_basesNP.elf` at
  `(address - 0x80600000) + 0x1000` and cross-checked against the export:

  | cluster | body |
  |---|---|
  | multi-course-select texture screen | `0x80630820`: index-change gate, two hide arms, `"MultiCorseSelectTexture/"` path build, archive load, thirteen BRK panes, mode bits `0x40`/`0x20` of `0x80429F80` |
  | course-tree node actor | `0x80797D30`: 40-byte name record, unlocked-area effects `0x36`/`0x37`, seven-arm state machine, `0x35` kind re-read |
  | course-select scene controller | `0x8077DC80`: eight-entry dispatch table `0x807CF6C8`, interface id at vtable `+0x60`, `"Fs00"`/`"Fs01"` probe, sound submit, model matrix and 3D entry |
  | ride actor child gate | `0x8077A4B0`: `"s0"`/`"s1"` key, four create arms (`0x27B`/`0x27C`/`0x27D` + `0x27E`), three clear arms, the asymmetric `+0x124` writes |
  | cursor transition machine | `0x8077C670`: twelve-state jump table `0x8081845C`, four documented fall-throughs, two `200 + n` distance ladders, vtable `+0x64` tail |
  | shadow-quad builder | `0x80669560`: two mirrored arms, `qr3` `(f32)(u16)` angle conversion, four `0x800DB210` records, three culling extents |
  | `Wm_ob_witchcraft` sweep | `0x8070FAB0`: `+0x9FC` slot ring, six kill tests (one through the player `+0x3C8`/`0x3CC` slots), two emitted vectors, `1` return |
  | world-map bounds rebuild | `0x807BFFA0`: `±99999` seed, area sweep, two probe paths, display vtable `+0x28` with `150`/`300`, ten clamps, audio bank 6 |

- **Five `0x8064` planner-bucket bodies** (516 bytes), taking that bucket from
  77 to 72 untranslated addresses:

  | cluster | bodies |
  |---|---|
  | release wrappers | `0x80645E10`, `0x80645080` (`this != 0 && count > 0` → `0x802B9280`, returns `this`) |
  | angle refresh | `0x80649010` (flag nibbles 16..19 → `+0x1A4EE`/`+0x1A4F2`, 20..23 → `+0x1A4F0`, each `* 0x3C`; `0x80648550` state at `+0x1A4E0`) |
  | motion start | `0x8064AC90` (`+0x764` gate, `0x80166EF0(+0x704, 0.0, 0)`, vtable `+0xD4` with the BSS name `0x80827F90`) |
  | constructor | `0x8064D5E0` (`0x418` bytes, vtable `0x807E46D0`, `+0x3D0`/`+0x3F0` sub-objects) |

## Where the binary differed from the wave-145 prose

1. **`0x80797D30`'s failed-`0x400`-probe arm is not the clear arm.** The raw
   words at `0x80797EF0..0x80797F28` re-issue `0x807980F0`, call
   `0x8079F900(this, 0)` and close with **`0x807984F0`**; there is no store of
   the BSS word `0x80832EB0` into `+0x22C` and no `0x80798310` call on that path.
   The BSS store and `0x80798310` belong to the separate
   `fn_8079A4B0(this, +0x22C) != 0` arm at `0x80798044..0x80798078`.
2. **`0x8077DC80`'s `0x80100CA0` second argument is a float.** The raw sequence
   `bl 0x80168080; lwz r4,460(r31); fmr f2,f1; lwz r3,728(r31);
   lwz r5,4(r4); lwz r4,496(r31); lfs f0,88(r5); fmr f1,f0` at
   `0x8077DE50..0x8077DE70` shows `r4` being overwritten, so the callee's result
   cannot be a 64-bit integer argument. The call is
   `0x80100CA0(f1 = owner+0x58, f2 = result, r3 = manager, r4 = id, r5 = owner)`.
3. **`0x80649010`'s nibble shifts.** Ghidra prints `>> 0x14` / `>> 0x18`, but
   the raw `rlwinm 16,28,31` and `rlwinm 12,28,31` at
   `0x8064902c..0x80649030` select bits 16..19 and 20..23 of the word at `+4`.
   The first pair lands at `+0x1A4EE`/`+0x1A4F2`, the second at `+0x1A4F0`.

Two further points that are confirmations rather than corrections:

- `0x80630820`'s `0x80630800` call really takes three arguments: the raw
  `lwz r4,684(r3)` / `lwz r5,688(r3)` at `0x8063087C`/`0x80630888` survive the
  `cmpi` chain into the `bl` at `0x80630898`.
- `0x8077A4B0`'s create arm clears `+0x124` on **both** children
  (`0x8077A5E4..0x8077A5F4`) while the clear arm clears only `this`
  (`stb r0,292(r31)` at `0x8077A788`).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80630820` | `fn_80630820` | 972 | medium |
| `0x80645080` | `fn_80645080` | 100 | high |
| `0x80645E10` | `fn_80645e10` | 100 | high |
| `0x80649010` | `fn_80649010` | 104 | medium |
| `0x8064AC90` | `fn_8064ac90` | 100 | medium |
| `0x8064D5E0` | `fn_8064d5e0` | 112 | high |
| `0x80669560` | `fn_80669560` | 916 | high |
| `0x8070FAB0` | `fn_8070fab0` | 912 | medium |
| `0x8077A4B0` | `fn_8077a4b0` | 924 | medium |
| `0x8077C670` | `fn_8077c670` | 924 | medium |
| `0x8077DC80` | `fn_8077dc80` | 936 | medium |
| `0x80797D30` | `fn_80797d30` | 960 | medium |
| `0x807BFFA0` | `fn_807bffa0` | 904 | medium |

**Totals:** 13 bodies, 7,964 bytes. The address window runs from `0x80630820` to
`0x807BFFA0`.

## Notes on inference

- The nine `medium` rows each call at least one unclaimed helper or `main.dol`
  address (`0x80630800`, `0x800B4F40`, `0x80630BF0`, `0x8079BB60`, `0x8079F5F0`,
  `0x8079B660`, `0x8079F900`, `0x8079B720`, `0x807980F0`, `0x80796A10`,
  `0x80799200`, `0x80797810`, `0x80796CF0`, `0x80797190`, `0x80799340`,
  `0x80774FA0`, `0x8075C010`, `0x800F57A0`, `0x800FC9C0`, `0x800F9080`,
  `0x80C561E0`, `0x8077E410`, `0x8077E570`, `0x8006CC00`, `0x8006CD40`,
  `0x800FDC60`, `0x800F9CD0`, `0x800F6410`, `0x800FDC80`, `0x800F2A00`,
  `0x800FB330`, `0x800FB3C0`, `0x800FB410`, `0x800FB460`, `0x800FB350`,
  `0x800FD920`, `0x800F1710`, `0x8077A850`, `0x8007FDA0`, `0x80162D00`,
  `0x80070CB0`, `0x8005F900`, `0x8006CED0`, `0x8070FE40`, `0x807C0B20`,
  `0x807C0330`, `0x807C13A0`, `0x807C1410`, `0x807C0C10`, `0x8019CA00`,
  `0x80166EF0`, `0x80648550`, `0x807E46D0`). Their argument shapes are fixed by
  the instruction stream; their bodies are not claimed.
- `0x8077DC80` reads the dispatch triplets at `0x807CF6C8` directly. Every entry
  is `{ 0, -1, handler }` (verified in the image), so the CodeWarrior thunk
  `0x802DCBFC` reduces to `handler(this, state * 12)`; the comment in the body
  records that reduction instead of claiming the thunk's body.
- `0x80669560`'s four `psq_l ...,qr3,W=1` on a stack halfword are `(f32)(u16)`
  conversions under `GQR3 = 0x00050005`; the `unaff_GQR0` blocks in the export
  are the five `f27..f31` `stfd` + `psq_st` save/restore pairs and contain no
  arithmetic.
- The `0x807D0160`, `0x807C8E1C..0x807C8E30`, `0x807CF698`, `0x807CF6B4`,
  `0x807CF764`, `0x807CF768`, `0x807D0A6C`, `0x807D0B30`, `0x807D0B34`,
  `0x807CCA78`, `0x807CCA7C`, `0x807CCA88`, `0x8082158C`, `0x80821590`,
  `0x808215DC`, `0x808215E0`, `0x80805EA0`, `0x80806618`, `0x80806624`,
  `0x807C8588` pool words and the `0x807DD5D8`, `0x807DD600`, `0x807DD620`,
  `0x808184A0`, `0x80818090`, `0x80818094`, `0x80805DF4`, `0x807CF6C8` tables
  were read from the REL image by address; the `0x8035xxxx`/`0x8037xxxx`/
  `0x8042xxxx` words are `main.dol` BSS and the `0x8082xxxx`/`0x8083xxxx` words
  are this REL's BSS, all read as zeros there and named by address.

## Deliberate skips

- **Eighteen 48-byte `0x802DCBFC` forwarders of the `0x8064` bucket**
  (`0x80641570`, `0x806415A0`, `0x80644180`, `0x806441B0`, `0x806457C0`,
  `0x806457F0`, `0x80646590`, `0x806465C0`, `0x8064A0F0`, `0x8064A120`,
  `0x8064B160`, `0x8064B190`, `0x8064C340`, `0x8064C370`, `0x8064D580`,
  `0x8064D5B0`, `0x8064E590`, `0x8064E5C0`) are three-instruction
  `func_0x802dcbfc` wrappers whose behaviour depends on the unresolved
  indirect-jump stub. Writing them would require inventing the jump table, so
  they stay open, exactly as the 20 `0x8061` wrappers in
  `docs/COVERAGE_WAVE145_BASES.md`. `0x8064B660` (84 bytes) is the same family
  with a vtable `+0x1C` call in front and `0x8006CD40` behind, and is skipped for
  the same reason.
- **`0x80640AC0` (116 bytes), `0x80642CE0` (64 bytes) and `0x80643260`
  (148 bytes)** truncate on bad instruction data (`halt_baddata`) in their
  exports; no honest body exists.
- The remaining 51 `0x8064` rows were not started; they are the natural next
  wave for this bucket.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (13 rows, 7,964 bytes),
  re-read from the TSV for this pass rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 13 were checked against
  the ledger's address column before being written.
- `node tools/check-lane-csv.mjs wave146` reports `OK` for
  `wave146-bases.csv`: 13 rows, 13 unique addresses, no duplicates.
- `cargo check --message-format=short` reports no error naming
  `src/game/bases.rs` (the two remaining errors name `src/game/enemies.rs`, the
  parallel lane).
