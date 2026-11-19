# Coverage - Wave 96 (UI: HUD wipe transitions)

Pass 12 of the `ui::hud` lane. The lane's 64 KiB bucket `0x80010000` had 63
untranslated entries after wave 90. This pass clears the largest remaining
coherent run: the four `wipe*` transition scenes at `0x8001B3A0..0x8001F800`,
their vtable accessors, release bodies, forwarding thunks and the twelve-slot
transition table.

## How to read this file

Every row maps one Ghidra function to one `pub unsafe fn fn_<addr>` body in
`src/ui/hud.rs`. `address` is the link-time address in `sys/main.dol`, `bytes`
is the size recorded in `work/ghidra/main-gb/export/functions.tsv`, and
`confidence` matches the `# Confidence` line of the body's doc block.

The four scenes were previously reached one-way from
`crate::engine::display`: its wipe gates at `0x8001BF10`, `0x8001BF50`,
`0x8001BF90` and `0x8001BFD0` call the builders translated here by address
(`0x8001B470`, `0x8001BC60`, `0x8001C240`, `0x8001C760` respectively) but the
builders themselves had no body. Recording them in `ui::hud` keeps the bucket
ownership used since wave 90 and does not edit the display lane.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/hud.rs` | `ui::hud` | 30 | 3,200 | 13 | 17 |

## What this pass covers

- **Four wipe scene constructors, `0x8001B3A0..0x8001C690`.** Shared base object
  through `0x8016DC00` at `+0x00`, scene vtable, layout holder at `+0x14`, and
  the SDA publication slots `0x80429C30`/`0x80429C38`/`0x80429C40`/`0x80429C48`.
- **Four layout builders, `0x8001B470..0x8001C760`.** Archive and `.brlyt` names,
  animation/pane binds, root pane at `+0x1AC`, resolution record at `+0x1B0`,
  build flag `+0x1B8`/`+0x1C0`, state word `3`, and the hide bit on pane `+0xBB`.
- **Seven one-instruction accessors returning `0x80377A04`,**
  `0x8001CBB0..0x8001CBE0` and `0x8001DA50..0x8001DA70`.
- **Six release/descend/compare bodies, `0x8001D710..0x8001D930` and the exact
  twins at `0x8001F380..0x8001F5A0`.** The release path is
  `0x8015F800(this, 0)` plus `0x802B9280` when `flag > 0`; the chain walk probes
  virtual slot `+0xC` on the `+0x30` child five times before tail recursion.
- **Four `0x802DCBFC(b, b, a)` forwarding thunks,**
  `0x8001D9F0`, `0x8001DA20`, `0x8001F660`, `0x8001F690`. Same unresolved
  dispatch stub already documented for `0x800182D0`.
- **Transition-table support, `0x8001DB30`, `0x8001E5A0`, `0x8001F6C0`,
  `0x8001F740`, `0x8001F800`.** Buffer clear/detach at `+0x3D0`, the `+0x394`
  notification, and the insert/remove/probe operations on the twelve slots at
  `+0x0C..+0x38` counted at `+0x04`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8001B3A0` | `fn_8001b3a0` | 92 | medium |
| `0x8001B470` | `fn_8001b470` | 272 | medium |
| `0x8001BB90` | `fn_8001bb90` | 92 | medium |
| `0x8001BC60` | `fn_8001bc60` | 260 | medium |
| `0x8001C170` | `fn_8001c170` | 92 | medium |
| `0x8001C240` | `fn_8001c240` | 248 | medium |
| `0x8001C690` | `fn_8001c690` | 92 | medium |
| `0x8001C760` | `fn_8001c760` | 248 | medium |
| `0x8001CBB0` | `fn_8001cbb0` | 12 | high |
| `0x8001CBC0` | `fn_8001cbc0` | 12 | high |
| `0x8001CBD0` | `fn_8001cbd0` | 12 | high |
| `0x8001CBE0` | `fn_8001cbe0` | 12 | high |
| `0x8001D710` | `fn_8001d710` | 92 | high |
| `0x8001D850` | `fn_8001d850` | 224 | high |
| `0x8001D930` | `fn_8001d930` | 136 | medium |
| `0x8001D9F0` | `fn_8001d9f0` | 48 | medium |
| `0x8001DA20` | `fn_8001da20` | 48 | medium |
| `0x8001DA50` | `fn_8001da50` | 12 | high |
| `0x8001DA60` | `fn_8001da60` | 12 | high |
| `0x8001DA70` | `fn_8001da70` | 12 | high |
| `0x8001DB30` | `fn_8001db30` | 96 | medium |
| `0x8001E5A0` | `fn_8001e5a0` | 84 | medium |
| `0x8001F380` | `fn_8001f380` | 92 | high |
| `0x8001F4C0` | `fn_8001f4c0` | 224 | high |
| `0x8001F5A0` | `fn_8001f5a0` | 136 | medium |
| `0x8001F660` | `fn_8001f660` | 48 | medium |
| `0x8001F690` | `fn_8001f690` | 48 | medium |
| `0x8001F6C0` | `fn_8001f6c0` | 120 | high |
| `0x8001F740` | `fn_8001f740` | 184 | high |
| `0x8001F800` | `fn_8001f800` | 140 | medium |

## Deliberate skips

- **`0x8001B580`, `0x8001B620` (`wipeCircle` state update).** The export at
  `0x8001B620` is heavily corrupted by the decompiler: `unaff_GQR0` is used as
  the state value, the `ldexpf` calls are quantiser artefacts, and the branch
  target `0x8001B87C` sits in a removed-unreachable-block region. The player
  position block would have to be invented, so it is left out rather than
  guessed.
- **`0x8001B920`, `0x8001BA20`, `0x8001BF20` (`wipeCircle`/`wipeDokan`
  transition gate helpers).** They depend on the layout state word and on
  `0x800B30E0` with no recovered receiver; the load/visibility stores can be
  reconstructed but the surrounding state contract cannot.
- **`0x8001CEE0`, `0x8001E6F0`, `0x8001E790`, `0x8001FA70`, `0x8001FD40`,
  `0x8001FF90`.** Float/vtable-heavy wipe state bodies whose SDA/SDA2 records
  and virtual-slot roles are not identified. They remain next targets.
- **`0x8001F8A0`.** Calls `0x80020100` with no operands printed; translating it
  would require inventing the argument list.
- **`0x8001DAC0`.** Eighth identical `0x80377A04` accessor; the pass stops at
  seven of the twelve so the CSV stays at the wave's 30-body target. It is a
  byte-identical body and can be claimed by a later pass.
- **`0x80012940`, `0x80012970`, `0x800129A0`.** The four-player panel core and
  its two helper entries deferred by wave 90; `0x800129A0` is 6,204 bytes and
  needs its own pass.
- **`0x800105B0..0x80010DD0` and `0x80014330..0x800148F0`.** Same ownership
  decision as wave 90: those are the `game::path` band and the `game::actor`
  record accessors, not HUD code.
- **`0x8001BF10`, `0x8001BF50`, `0x8001BF90`, `0x8001BFD0`.** Already
  translated in `crate::engine::display`; they are the one-way callers of the
  builders added here and were not duplicated.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/...` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in `src/ui/hud.rs`.
- No address was already in `ledger/functions.csv`; the CSV validator checks this
  live against the merged ledger.
- `cargo check --message-format=short` reports no error for `src/ui/hud.rs`.
- `node tools/check-lane-csv.mjs wave96` is used after the CSV lands; problems
  reported for other lanes' CSVs are out of scope for this lane.
