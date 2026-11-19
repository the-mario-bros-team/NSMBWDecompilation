# Coverage - Wave 214, lane A (`game::player`, the `0x8006C8A0..0x8006CBA0` derived vtable tail)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 9 | 672 | 9 | 0 | 0 |

Status split: 9 `translated`, 0 `partial`, 0 deferred.

Body block: `work/wave214/laneA_block.rs`. Ledger rows:
`ledger/agents/wave214-laneA.csv`.

## How to read this file

* **Address** - virtual entry point in the DOL image, not a file offset.
* **Name** - the Rust name, `fn_<address>` lowercase; the Ghidra name is in
  each body's `# Original` block.
* **Size** - the `functions.tsv` value. Eight of the nine were re-derived from
  the prologue and the closing `blr` and match exactly. `0x8006CAE0` ends after
  84 real bytes; the tsv's 88 is the aligned slot up to the next 16-byte
  boundary at `0x8006CB40` (three zero words of padding). No size in this lane
  is a 16/20-byte prologue artefact and none of the bodies is a prologue stub.
* **Confidence** - `high` here means every address, field offset, immediate and
  branch is literal in the disassembly.
* `r13 = 0x8042F680`, so `r13 - 0x6F4E` = `0x80428732` (the mode halfword),
  `r13 - 0x58E8` = `0x80429D98` and `r13 - 0x58E4` = `0x80429D9C` (the two
  placement pointers this lane writes).

## Where the nine bodies sit

Five are slots of the derived vtable `0x8030F228` that `0x8006C6D0` installs
(wave 213 lane F translated the constructor and the neighbouring slots). The
table was re-dumped for this lane:

| slot | address | body | role |
|---|---|---|---|
| `+0x10` | `0x8006C8A0` | [`fn_8006c8a0`] | `b 0x8006C570` -> `b 0x80161DA0` |
| `+0x18` | `0x8006C8B0` | [`fn_8006c8b0`] | `fn_8006c580(this) != 0` |
| `+0x24` | `0x8006C8F0` | [`fn_8006c8f0`] | layout state is neither `1` nor `5` |
| `+0x28` | `0x8006C980` | [`fn_8006c980`] | stage-2 velocity latch |
| `+0x30` | `0x8006C9F0` | [`fn_8006c9f0`] | `+0x124` byte, gated on `0x801621E0` |

The other four are not in the vtable. `0x8006CA70` is the two-word SDA writer;
`0x8006CA80` / `0x8006CBA0` are the two bank constructors that call it
immediately before entering the factory `0x80162B00`; `0x8006CAE0` is the
type-`2` registry walk. The neighbouring entries `0x8006C8E0` (`b 0x8006C5B0`),
`0x8006CA40` (`b 0x8006C650`), `0x8006CA50` / `0x8006CA60` (bare `blr`) and
`0x8006CB40` remain untranslated and belong to other lanes.

## Notable findings

### The forwarder chain `0x8006C8A0`

The single word `0x4BFFFCD0` decodes as `b 0x8006C570` (`LI = -0x330`), and
`0x8006C570` is itself the four-byte `b 0x80161DA0` translated in wave 213. The
Rust body therefore calls [`fn_8006c570`] rather than skipping a hop, which
keeps the three-entry chain the wave 211 lane B note established visible.

### Two `void` helpers whose `r3` is consumed

`0x8006C5C0` and `0x8006C610` both end in a real `bl` to a predicate
(`0x801620E0` and `0x801621E0` respectively) followed by a normal epilogue, so
callers still see the predicate's result in `r3`. Two bodies in this lane do
exactly that:

* `0x8006C8F0` gates everything on `0x8006C5C0`'s trailing predicate
  (`cmpi cr0,3,r3,0` right after the `bl`), then reads `+0x0C` and `+0x08` of
  the layout object returned by `0x80107F10` and returns `1` only when neither
  is `1` and neither is `5` (`cntlzw` + `rlwinm r0,27,5,31`, i.e. `clz >> 5` of
  the survivor flag).
* `0x8006C9F0` gates on `0x8006C610`'s trailing predicate before returning the
  sign bit of `(-b | b)` for the `+0x124` byte, which is `1` exactly when that
  byte is non-zero.

Because the module's translations of both helpers are declared `void`, the two
bodies re-express the helper's instruction sequence inline (one vtable call plus
one predicate call for `0x8006C8F0`; one `0x800B2EC0()` read, one SDA store and
one predicate call for `0x8006C9F0`) instead of calling the helper and then
re-evaluating the predicate. Each machine-level effect still happens exactly
once and in the original order.

### `0x8006C980` is the derived replacement for an empty hook

Base vtable `+0x28` is `0x8006C600`, a four-byte branch to the empty leaf
`0x80162110`. The derived table replaces it with [`fn_8006c980`], which for
stage `2` turns `+0xAC..+0xB4` into the "current position" and `+0xC4..+0xCC`
into the difference against the previous position kept at `+0xB8..+0xC0` - and
then still tail-calls [`fn_8006c600`], so the base hook keeps running.

### The two bank constructors and their return value

`0x8006CA80` publishes `(param_3, param_4)` through [`fn_8006ca70`], looks the
owner up in the `0x80377A10` list with the `u16` at `0x80428732` as key
(`0x8006C500(key, 0)`), and enters `0x80162B00` as
`(param_1, owner, param_2, 2)`. `0x8006CBA0` is the same tail call with the
caller's own owner and with `(param_4, param_5)` as the placement pair.

Ghidra declares both `void`, but the factory's result stays in `r3` and the
known caller `0x8007EA80` does test it and then reads `*result`. Both bodies
therefore return `u32`; this is the only place in the lane where the port
intentionally differs from the decompiler's declared type, and the evidence for
it is `8007ea80_FUN_8007ea80.c` lines 32-40.

### `0x8006CAE0` cursor order

The loop stores `0x80162DB0(2, cursor)` into the cursor *before* the zero test,
so the body's `lwz r12,0x60(r31)` sees the freshly returned node and the next
iteration passes that node as the search start. Slot `+0x50` of `0x8030F228`
is a bare `blr` (`0x8006CA50`), so for instances of this family the walk is a
no-op tick; the type-`2` key selects the object class, not the callback.

## Verification

* Sizes re-derived from the prologue and the closing `blr` for all nine
  addresses; eight match `functions.tsv`, one (`0x8006CAE0`) is the aligned
  slot (84 real vs 88 tsv) and is documented as such in its `# Original` block.
* All bodies were read from the raw disassembly as well as from
  `work/ghidra/main-gb/export/*.c`; the decompiler's argument-less calls
  (`FUN_8006c5c0()`, `FUN_8006c8b0(void)`) were resolved by checking which
  registers were live at the `bl`.
* Scratch crate: `src/`, `Cargo.toml` and `Cargo.lock` copied to a temporary
  directory, this block appended to the copy's `src/game/player.rs`, and
  `cargo check --lib` exits 0 with no diagnostic mentioning any of the nine
  bodies.

## Deferrals

None. All nine bodies were translated from literal evidence; no body needed to
be guessed.
