# Wave 181 lane E coverage (`game::bases`, REL `d_basesNP`)

Scope: eight bodies under 300 bytes, all `0x8061`/`0x8062`.

| Address | Size | Function | Result |
| --- | --- | --- | --- |
| `0x8061B820` | 104 | `fn_8061b820` | translated |
| `0x8061D870` | 100 | `fn_8061d870` | translated |
| `0x8061E750` | 96 | `fn_8061e750` | translated |
| `0x8061E7B0` | 100 | `fn_8061e7b0` | translated |
| `0x80620570` | 100 | `fn_80620570` | translated |
| `0x80624230` | 100 | `fn_80624230` | translated |
| `0x80624790` | 128 | `fn_80624790` | translated |
| `0x80624930` | 120 | `fn_80624930` | translated |

## Notes

- `0x8061B820` is the layout-finished half of the `+0x330` wrapper pair:
  `0x800C9610(this+0x70, -1) == 0` dispatches block `0x8082_4B80` through vtable
  slot `+0x18`, then sets `+0x375 = 1`, `+0x376 = 0`. Its sibling
  `0x8061B630` (already in `src/game/bases.rs` as `fn_8061b630`) uses the same
  wrapper and block under a different gate, so the shape was confirmed against
  existing code.
- `0x8061D870`, `0x8061E750`, `0x8061E7B0`, `0x80620570`, `0x80624230` are five
  registrations of one deleting destructor: guard `this != 0`, tear down the
  embedded `+0x18` member via `0x8015F9F0(..., 0)`, then `0x802B9280(this)` only
  for `free > 0`, returning `this`. Identical to the already-translated
  `fn_80632180` / `fn_806321e0` pair; the `0x8061E750` copy is 96 bytes only
  because the compiler dropped a redundant second null test.
- `0x80624790` latches two shadow/pending byte pairs (`+0x257`/`+0x258`,
  `+0x259`/`+0x25A`) into layout animations `0` and `1` on the `+0xAC` object,
  then raises the dirty byte `+0x256`.
- `0x80624930` picks layout flag `2` + animation `4` when the mode word at
  `+0x25C` equals `2`, otherwise flag `3` + animation `5`, and raises `+0x256`.
- Every callee (`0x800C9610`, `0x800C92F0`, `0x800C9490`, `0x8015F9F0`,
  `0x802B9280`) already has a Rust body or an established `abi_callN!` usage in
  `src/game/bases.rs`; no new helper was introduced.

## Deferred

None.
