# Coverage — wave 4 NW4R `lyt` third pass (`0x8016CF50..0x8018AA80`)

## How to read this file

- **Scope** — this document covers the third `lyt` translation pass. It adds
  25 functions to the semantic module `src/ui/lyt.rs`; no new staging module
  was created and no other lane's files were touched.
- **Addresses** are virtual `sys/main.dol` addresses (image base
  `0x80004000`), not file offsets.
- **Evidence** is the Ghidra C export under
  `work/ghidra/main-gb/export/`; each row names its file.
- **Status** follows `docs/READING_GUIDE.md`:
  `translated` = full control flow, every call accounted for;
  `partial` = flow and major effects present, one operand not literal;
  `stub` = address/role only.
- **Confidence** measures the evidence, not the code style. `medium` marks a
  function whose bytes and control flow are exact but whose callers' object
  roles are inferred.
- Start with the **summary**, then the **function table**. The
  **cluster notes** explain how the new bodies connect to the earlier waves.

## Summary

| metric | value |
|---|---:|
| functions added to `src/ui/lyt.rs` | 25 |
| bytes translated | 2,740 |
| `translated` / `partial` / `stub` | 25 / 0 / 0 |
| high confidence | 17 |
| medium confidence | 8 |
| new staging modules | 0 |
| files written | `src/ui/lyt.rs`, this document, `ledger/agents/wave4-lyt.csv` |

Address span: `0x8016CF50..0x8018AA80`. All 25 functions are at least 16
bytes, outside the runtime-lane exclusions, and self-contained enough to read
end-to-end from one Ghidra file.

## Function table

| address | Rust function | role | status | conf. | evidence |
|---|---|---|---|---|---|
| `0x8016CF50` | `fn_8016cf50` | attach a prepared child after the `+0x74` handle check | translated | high | `8016cf50_FUN_8016cf50.c` |
| `0x8016D090` | `fn_8016d090` | walk the pane tree, call visitor `+0x0C` per node | translated | high | `8016d090_FUN_8016d090.c` |
| `0x8016D120` | `fn_8016d120` | register `0x8016D090` as the `+0x0C` visitor | translated | medium | `8016d120_FUN_8016d120.c` |
| `0x8016D330` | `fn_8016d330` | attach a prepared child after virtual `+0x9C` | translated | high | `8016d330_FUN_8016d330.c` |
| `0x8016D3C0` | `fn_8016d3c0` | walk the pane tree, call visitor `+0x120` per node | translated | high | `8016d3c0_FUN_8016d3c0.c` |
| `0x8016D440` | `fn_8016d440` | register `0x8016D3C0`, then run `0x802D85C0` | translated | medium | `8016d440_FUN_8016d440.c` |
| `0x8016D8E0` | `fn_8016d8e0` | build a transform, run a scoped helper, attach a child | translated | medium | `8016d8e0_FUN_8016d8e0.c` |
| `0x8016DC00` | `fn_8016dc00` | construct the animation-state object | translated | medium | `8016dc00_FUN_8016dc00.c` |
| `0x8016DCF0` | `fn_8016dcf0` | copy three colour bytes into the state | translated | high | `8016dcf0_FUN_8016dcf0.c` |
| `0x8016DD20` | `fn_8016dd20` | `0 -> 2` timer transition (zero test) | translated | high | `8016dd20_FUN_8016dd20.c` |
| `0x8016DD50` | `fn_8016dd50` | `1 -> 3` timer transition (one test) | translated | high | `8016dd50_FUN_8016dd50.c` |
| `0x8016DD80` | `fn_8016dd80` | advance the timer and report the end flag | translated | high | `8016dd80_FUN_8016dd80.c` |
| `0x8016E4B0` | `fn_8016e4b0` | pack input bits 0..2 into one byte | translated | high | `8016e4b0_FUN_8016e4b0.c` |
| `0x8016E610` | `fn_8016e610` | align upward with the `0x88`-byte header formula | translated | high | `8016e610_FUN_8016e610.c` |
| `0x8016E760` | `fn_8016e760` | remaining animation length after the `0x80` lead-in | translated | medium | `8016e760_FUN_8016e760.c` |
| `0x8016E7D0` | `fn_8016e7d0` | align upward with the `0x80` header formula | translated | high | `8016e7d0_FUN_8016e7d0.c` |
| `0x8016E8F0` | `fn_8016e8f0` | allocate/construct an animation resource | translated | medium | `8016e8f0_FUN_8016e8f0.c` |
| `0x801822F0` | `fn_801822f0` | save, apply and restore one `0x20`-byte `RLAN` record | translated | high | `801822f0_FUN_801822f0.c` |
| `0x80182380` | `fn_80182380` | copy eight words into a record, then refresh it | translated | high | `80182380_FUN_80182380.c` |
| `0x801823E0` | `fn_801823e0` | `RLMC` material-colour byte writer | translated | medium | `801823e0_FUN_801823e0.c` |
| `0x80183910` | `fn_80183910` | insert a descriptor into an embedded sentinel list | translated | high | `80183910_FUN_80183910.c` |
| `0x80183F40` | `fn_80183f40` | notify a pane and its subtree with one argument | translated | medium | `80183f40_FUN_80183f40.c` |
| `0x8018A4B0` | `fn_8018a4b0` | unlink every node, then optionally free the header | translated | high | `8018a4b0_FUN_8018a4b0.c` |
| `0x8018A540` | `fn_8018a540` | detach every node before a circular sentinel | translated | high | `8018a540_FUN_8018a540.c` |
| `0x8018AA80` | `fn_8018aa80` | conditional release wrapper | translated | high | `8018aa80_FUN_8018aa80.c` |

All evidence paths are relative to `work/ghidra/main-gb/export/`.

## Cluster notes

### Pane construction and pane-tree walks (`0x8016CF50..0x8016D8E0`)

The two attach wrappers are siblings:

```text
fn_8016cf50:  handle(+0x74) -> vcall +0x8C(data) -> vcall +0x94
fn_8016d330:  vcall +0x9C  -> vcall +0x8C(data) -> vcall +0x94
```

`fn_8016d8e0` is the same attach pair with `0x8016C9E0` filling a 48-byte
scratch transform first, and with the 48-byte scoped helper (`0x802DB6C0`,
`0x802DB750`, `0x802DB8C0`, `0x802DB970`, `0x802DB710`) wrapped around the
attach.

The two tree walks share one shape:

```text
node = 0x80228FF0(tree + 0x38, 0)
while node != 0:
    callback(node, context)
    node = 0x80228FF0(tree + 0x38, node)
```

`fn_8016d090` reads the callback pair at `+0x0C`/`+0x10`; `fn_8016d3c0` reads
it at `+0x120`/`+0x124`. The next-node query is deliberately placed after the
callback because that is where the original `for`-loop update expression sits.
`fn_8016d120` and `fn_8016d440` register the two walks with `0x802862C0`.

### Animation state (`0x8016DC00..0x8016DD80`)

| offset | type | meaning |
|---|---|---|
| `+0x00` | pointer | vtable `0x80329AD8` |
| `+0x04` | `u32` | mode: initial `-1`, then `0`/`1`, transitions to `2`/`3` |
| `+0x08` | `u8` | flags; bit 0 and bit 1 are returned by the timer |
| `+0x0A` | `u16` | counter limit (`0x14` at construction) |
| `+0x0C` | `u16` | counter |
| `+0x10..+0x13` | 4 bytes | three colour bytes plus the untouched `0xFF` |

`fn_8016dd20`/`fn_8016dd50` use `countLeadingZeros` as equality tests
(`word == 0`, `word == 1`). `fn_8016dd80` preserves the off-by-one difference
between the two transitions: the `2 -> 1` branch compares the limit with the
pre-increment counter, the `3 -> 0` branch compares `limit + 1` with the
post-increment value.

### Animation resources (`0x8016E610..0x8016E8F0`)

`fn_8016e8f0` is the allocator/constructor; `fn_8016e760` reports the usable
length after a fixed `0x80` lead-in; the two alignment helpers are the
`0x87`/`0x7F` variants of the same `RoundUp(size + header, align)` expression.

### `RLAN` / `RLMC` (`0x801822F0..0x801823E0`)

These complete the animation-transform cluster started in wave 2 (see
`docs/COVERAGE_r80180_801c0.md`). `fn_801822f0` applies one record and restores
the two words that `0x801C7100` owns; `fn_80182380` copies an eight-word
descriptor and refreshes it; `fn_801823e0` writes one colour byte for the
`RLMC` path (`0x524C4D43` at caller `0x8017DCC0`).

The `fn_801823e0` selector map is:

| selector | destination | width |
|---|---|---|
| `0..3` | packed indirect-material base, byte `selector` | `u8` |
| `4..15` | `+0x24 + ((s - 4) >> 2) * 8 + ((s - 4) & 3) * 2` | `u16` |
| `16..31` | `+0x3C + ((s - 16) >> 2) * 4 + ((s - 16) & 3)` | `u8` |
| `>= 32` | no write | — |

### Lists (`0x80183910..0x8018AA80`)

`fn_80183910` uses the wave-2 sentinel insert `0x8018A590` to add a descriptor
to the list at `owner + 0x10`/`owner + 0x14`. `fn_80183f40` walks a different
sentinel list whose link starts at `+0x14`; each node's vtable is the word four
bytes before its link, so the node base is `link - 4`.

`fn_8018a4b0` clears an embedded sentinel list (header `{count, sentinel}` at
`+0x00`/`+0x04`) and optionally frees the header. `fn_8018a540` clears a
circular list up to its self-linked sentinel. `fn_8018aa80` is the same
conditional-release wrapper as `0x8016C6F0` and `0x8016DCA0`.

## Unknowns and limits

1. **Virtual argument lists.** Ghidra prints several virtual calls with no
   arguments (`+0x8C`, `+0x94`, `+0x9C`, `+0x18`, `+0x2C`, `+0x14`). The Rust
   code passes `this` because it is still in `r3` at the `bl`; these sites are
   marked `// inferred:`. If a later caller proves a different signature, the
   slot number remains the evidence and the note should be updated, not the
   control flow.
2. **Dropped call arguments.** `0x8016DCF0` is printed as `FUN_8016dcf0()` in
   `0x8016DC00`, and `0x802B9280`/`0x802D85C0` are printed without arguments.
   The pointer is restored from `r3` (and the colour pointer from `r4`), the
   same convention the sibling modules already use.
3. **Scoped helper (`0x8016D8E0`).** The 48-byte stack object built by
   `0x802DB6C0` and released by `0x802DB710(helper, -1)` is not identified. Its
   calls are reproduced by address; the class, fields and purpose are
   `// unknown:`.
4. **Material-colour selector.** The branch map is exact, but the mapping from
   selector to material/texture index comes from the `RLMC` dispatch at
   `0x8017DCC0`, not from a recovered header. The colour clamp is reproduced
   instruction-for-instruction, including the signed-short corner.
5. **`fn_8016e760` lead-in.** `0x8016E7D0(0, 4)` evaluates to `0x80`; whether
   that is a file header, a frame lead-in or a block size is not proven.
6. **List header ownership.** `fn_8018a4b0` frees the header itself when
   `delete_flag > 0`; the allocator that created it is outside this lane.
   `fn_8018a540` dereferences `*head_word` without a null check, exactly like
   the original.

## Verification

```text
node tools/doc-lint.mjs src
# {"ok": true, "files": 68, "violations": 0, "byRule": {}}
```

`cargo check` in the shared tree is currently blocked by two diagnostics in
other lanes (`src/render/gx_band.rs:1813` missing `call4`, and
`src/audio/ax.rs:2195` missing `FN_801AC440`). Neither diagnostic is in this
lane. To verify this file independently, the crate was copied to a scratch
directory outside the repository, those two unrelated lines were neutralised,
and `cargo check` finished with 0 errors and no warnings from
`src/ui/lyt.rs`.

Only `src/ui/lyt.rs`, this document and `ledger/agents/wave4-lyt.csv` were
written. `src/lib.rs`, every `mod.rs`, `Cargo.toml`, `src/platform.rs` and all
other lanes' files were left untouched.
