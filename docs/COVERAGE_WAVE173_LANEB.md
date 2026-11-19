# Coverage - Wave 173 lane B (`game::bases`, four 356-byte bodies)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 4 | 1,424 | 4 | 0 |

Lane B of the 356-byte sub-band. All four bodies are translated; nothing is
deferred.

## What this pass covers

- **Lift contact timers (1 body).** `0x806F0DB0` decrements the two four-player
  countdown arrays at `+0x95C`/`+0x960` of the `manjirou` four-wheel lift,
  walks the two `0x5C`-byte wheel-record lists at `+0x610`/`+0x66C`, and for each
  player actor with `+0x38C == 1` reads its index through the `+0x6C` virtual of
  the `+0x60` vtable, re-arms `+0x95C + index` with `8`, and — when
  `0x80056CF0(player, 0x1C)` succeeds and `+0x960 + index` was clear — writes
  `1` to the caller's out-word and `4` to the second timer.
- **Three-record collision builder (1 body).** `0x8071F120` is the initialiser
  of the actor whose four `0x60`-byte records start at `+0x434` (the walkers
  `fn_8071f290`/`fn_8071f2e0`/`fn_8071f340` cover the same array). It builds
  records `+0x434`, `+0x494` and `+0x4F4` with `0x800DB180` from five floats and
  five words each, then stores byte `10` at each record's `+0x47` and ids
  `0x20D80`/`0x20900`/`0x20480` at `+0x3C`.
- **Fallback spawn by name (1 body).** `0x80774A60` matches the record's
  REL-offset string against the eight `{name, id}` entries at `0x807CF418`
  (`cobKiller`, `cobRotary`, `enemy`, `cobBubble`, `hashira`, `sandpillar`,
  `DokanRoute`, `Pakkun`), seeds a position through `0x801006C0`, computes the
  name-suffix length with `0x802E2108`, sets bit `0x10000` for `cobRotary`, spawns
  the variant through `0x800F2A00(id, owner, flags, position, 0)` and clears bit
  `0x100` in the record's `+0x14`. It is the fallback branch of `0x807740D0`.
- **World-map zone table build (1 body).** `0x807BAF50`, guarded by the
  `0x8042A24F` built byte, allocates `0x16518`-byte scene objects through
  `0x802B9210` for every `0x803748A8[world]` slot (nine worlds, three slots
  each), installs vtable `0x8031BE20` with zeroed `+0xC`/`+0x10`/`+0x14`, stores
  each handle at `manager + 0x80 + (slot + 3 * world) * 4`, calls
  `0x800F35A0(object, world, slot)` and then publishes the `0x800F9530` value of
  every kind-`3` element through `0x801026D0` with mask `0x10`/`0x20` when the
  matching `0x800F5770` count is positive.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806F0DB0` | `fn_806f0db0` | 356 | high |
| `0x8071F120` | `fn_8071f120` | 356 | high |
| `0x80774A60` | `fn_80774a60` | 356 | high |
| `0x807BAF50` | `fn_807baf50` | 356 | high |

## Corrections and notes

- `0x806F0DB0` takes `this` **and** an out-word: the export prints
  `void f(void)` because `bl 0x802DCD5C` is `_savegpr_24`, which preserves
  `r3`/`r4`. The `*(u32 *)...= 1` in the C is Ghidra losing `r4`; the raw words
  keep `r30 = r4` (`0x806F0DC8`) and the store is `stw r27,0(r30)`
  (`0x806F0EAC`). The trailing `+0x95C..+0x960` scan only feeds `_restgpr_24`,
  so the body returns nothing.
- `0x806F0DB0`: the index is read through the **actor** vtable at `actor+0x60`
  slot `0x6C` (`lwz r12,0x60(r26)` / `lwz r12,0x6C(r12)`), so the body uses
  `actor_method0`, matching the existing `actor_method0(rider, 0x6C)` uses in
  this module. The `7FFF0774` word is a byte mask (`clrlwi`-family), i.e. the
  index is an unsigned byte.
- `0x8071F120`: the `psq_l f1,0(r1+8),1,3` at `0x8071F16C` reads the `0x2000`
  stack halfword as an **unsigned 16-bit** GQR3 sample, so `f1 = 8192.0`, and
  `0x8071F174` multiplies it by `0x807CD088 = 0.00390625` (32.0) before the
  `0x80237C40` cosine. **That cosine result is discarded**: `f1` is reloaded with
  `0x807CD050 = 0.0` at `0x8071F198` before the first `bl`, and the first
  record's third float is the copied incoming `f1` (`fmr f29,f1` at
  `0x8071F164` → `fmr f3,f29` at `0x8071F188`). Ghidra's `f3 = cos` and its
  `4.0 + 16.0/cos` third argument are artefacts of the `qr3` block it dropped;
  the true `fdivs`/`fadds` operands are `16.0/4.0` and the `-96.0`/`4.0`
  doublings (`-192.0`/`8.0`). The `f1` argument is therefore the third float of
  the `+0x434` record; the two later records use `0.0` there.
- `0x80774A60` matches the **full** table name length against the record string
  with `0x802E1BD0(text, name, strlen(name))` and breaks on `0`, then measures
  `strlen(record + offset + strlen(name))` (through `0x802E2108`, low 16 bits) to
  build the flag word. The `+8` offset read has **no null guard** on the record
  pointer in the shipped words (only the `+0xC`/`+0x14` reads are guarded), so
  the body keeps that.
- `0x807BAF50`: `r3`/`r4` are never read, so the body takes no arguments. The
  slot stride is `+3` per world (27 handles), the manager word is `0x8042A234`
  (published by `fn_807b9e40`) and the counts are the same `0x803748A8` array
  that `fn_807bac40` bumps while loading `pointW<n>.csv`. `0x800F35A0` is
  untranslated and called through `abi_call3!`.
- Follow-up for the coordinator (this lane did not edit anything outside its
  three files): `fn_8071f120` has no caller in this REL (it is a vtable slot, so
  its `f1` argument is only observable through the vtable), and
  `0x800F35A0`/`0x800F9530`/`0x801026D0`/`0x802B9210` remain address-only.

## Deferred

None.

## Verification

- `cargo check` succeeds with the block appended to a scratch copy of the tree
  under `%TEMP%\opencode\w173b` (the real `src/game/bases.rs` was not modified).
- `node tools/doc-lint.mjs <scratch>/src` reports `ok: true`, 0 violations.
- All four evidence exports exist under `work/ghidra/rel-d_basesNP/export/`.
- Constants read out of `work/rels/elf/d_basesNP.elf`:
  `0x807CD050 = 0.0`, `0x807CD084 = 16.0`, `0x807CD088 = 0.00390625`,
  `0x807CD08C = 4.0`, `0x807CD090 = -96.0`; table `0x807CF418` =
  `cobKiller 0x275`, `cobRotary 0x275`, `enemy 0x2A6`, `cobBubble 0x2A6`,
  `hashira 0x2A9`, `sandpillar 0x2A8`, `DokanRoute 0x2AC`, `Pakkun 0x2AB`.
