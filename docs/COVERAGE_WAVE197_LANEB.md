# Wave 197 lane B coverage (`game::bases`)

Ten bodies, all translated, no deferrals:

| Address | Size | Body |
| --- | --- | --- |
| `0x80762EA0` | 312 | `fn_80762ea0` - World-6 vine interface value: class-name table `0x80815470`, kind byte `0x807CEE40`, two `0x800FADxx` query pairs, writes `+0x1EC`. |
| `0x80763A30` | 308 | `fn_80763a30` - `cobGhost` break model: BRRES load, six `0x38`-stride panels, flag bytes `+0x1E8`. |
| `0x8079B300` | 240 | `fn_8079b300` - offsets the marker by the `0x8031D39C` anchor row selected by `0x800FC040`. |
| `0x8079BC10` | 196 | `fn_8079bc10` - re-poses the ghost-house root (`0x1B`/`0x1C`) and every live child. |
| `0x8079C150` | 224 | `fn_8079c150` - initialises a ghost child from a peer and selects state `0x12`. |
| `0x8079F500` | 232 | `fn_8079f500` - resolves the ghost record for the world byte and publishes `0x80C7115x`. |
| `0x8079FA10` | 212 | `fn_8079fa10` - picks a record id out of `{0x20,0x21,0x22}` for one board slot. |
| `0x807A4450` | 216 | `fn_807a4450` - egg sine break phase (variant `+0x5F4`, facing `+0x348`). |
| `0x807AB590` | 252 | `fn_807ab590` - per-player ghost release pass with two vtable slots and the `0x80354E80` bit-0 clear. |
| `0x807AF520` | 204 | `fn_807af520` - shared screen display-list rebuild; byte-identical to `fn_807aa310` minus the trailing font reset. |

## Notes carried into the bodies

- **`0x80762EA0` is a twin of `fn_807946d0`.** Same `+0x04` low-byte variant,
  same `0x800F9530` resolver, same `1`/`0` kind split, same
  `0x800FAD60`/`0x800FAE80` and `0x800FADB0`/`0x800FAF70` pairs. Only the class
  table (`0x80815470` instead of `0x8081B4D4`), the kind byte table
  (`0x807CEE40`) and the single value written to `+0x1EC` differ. The
  `0x80815470` entries are `W601`/`W603`/`W603`/`W605`/`W605`, i.e. World 6.
- **`0x80763A30` local aliasing.** The stack block `r1+0x14` holds the BRRES
  holder and is reused as the first argument of `0x80239E30` and
  `0x8023A0B0`; the model result lives at `r1+8` and `r1+0x10` is a copy of it
  passed to `0x801650D0`. The Rust uses one-element arrays for both so the
  address-taking stays explicit, as in `fn_8075a9a0`. The six names at
  `0x80815738` are `cobGhostOpen`, `cobGhostClose`, `cobGhostOut`,
  `cobGhostCloseAll`, `cobGhostCloseWindow`, `cobGhostCloseDoor`; the
  `0x807CEE88` words are all `1`.
- **`0x8079B300` table bases.** `addi r31,r31,-11364` resolves to `0x8031D39C`;
  the three offsets are `0x220`/`0x224`/`0x228` scaled by the `0xC` record
  stride, and the height row is `0x238 + index*4`. The three gate calls are the
  already-translated `fn_8079b1d0`/`fn_8079b700`/`fn_8079b240`, all argument-free
  in Rust, so the raw `mr r3,r29` before each is dead.
- **`0x8079BC10` undefined fallback.** For `kind` other than `0`/`1` the raw
  words branch straight to the common tail without writing `r30`. Its only
  caller `fn_8078e9a0` passes `+0x1CC`, which that state machine only ever sets
  to `0` or `1`; the body keeps `0x1B` as the documented fallback. This is the
  reason the existing caller still uses `abi_call1!(0x8079_BC10, selector)`.
- **`0x8079F500` / `0x8079FA10` share the board slot.** `fn_8079fa10` indexes
  `board + (index & 0xFF)*4 + 0x70`, not `this`; the raw
  `lwz r0,-0x6084(r30)` + `add r3,r0,r29` proves the board base, and
  `fn_8079f500` writes the `board+0x58` sentinel the retry loop compares
  against. `stb r0,0x14(r1)` clears the record's most significant byte, so the
  four-word record is masked with `0x00FF_FFFF`.
- **`0x807A4450` is single precision.** The raw words are `lfs`/`fadds`, so the
  body uses `call_f32_f32` for `0x800B2F10` rather than the `f64` spelling that
  its near-twin `fn_807a4550` uses. Constants: `0x807D0490 = 1.8`,
  `0x807D0494 = -1.8`, `0x807D0498 = 0.3`, `0x807D049C = 0.7`,
  `0x807D04A0 = 2.5`, `0x807D04A4 = 2.1`, `0x807D04A8 = 3.4`,
  `0x807D0418 = 0.0`.
- **`0x807AB590` link hop.** Each of the four `+0xC0` entries leads through
  `+0x128` then `+4` to the object carrying the `0x17C` flag word; the object is
  re-read between the two vtable dispatches (`+0x44`, `+0x4C`) exactly as the
  raw words do.
- **`0x807AF520` call order.** Identical to `fn_807aa310` up to and including
  the two `0x80093F10` font loops; it omits that function's trailing
  `0x80093F10(0, 2)` before the epilogue.

## Deferred

None.

## Verification

- The block was appended to a scratch copy of the crate (`cargo check
  --offline`) and compiled with no errors and no warnings in the appended range.
- `node tools/doc-lint.mjs <scratch>/src` reports `ok: true`, 0 violations.
- `node tools/check-lane-csv.mjs wave197` reports 10 unique lane-B rows; its six
  "no body" notes are expected until the coordinator appends the block (the
  other four addresses already appear in `src/game/bases.rs` as callee
  literals).

## Files

- `work/wave197/laneB_block.rs`
- `ledger/agents/wave197-laneb.csv`
- `docs/COVERAGE_WAVE197_LANEB.md`
