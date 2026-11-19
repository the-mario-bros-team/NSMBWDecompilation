# Coverage - wave 203, lane F (`ui::hud`, `0x80023B40..0x80023CB0` + `0x80023F80`)

Ten functions of `src/ui/hud.rs` translated from `sys/main.dol` in wave 203,
lane F. All ten now have a faithful Rust body appended to `src/ui/hud.rs`;
nothing in this lane is deferred.

## Scope

| address | size | name | confidence | role |
|---|---|---|---|---|
| `0x80023B40` | 136 | `fn_80023b40` | high | animation-name compare after `':'` |
| `0x80023C00` | 48 | `fn_80023c00` | high | forward `(b, b, a)` to `0x802DCBFC`, interface at `a + 0x18` |
| `0x80023C30` | 48 | `fn_80023c30` | high | forward `(b, b, a)` to `0x802DCBFC`, interface at `a + 0x24` |
| `0x80023C60` | 12 | `fn_80023c60` | high | returns `0x80377A04` |
| `0x80023C70` | 12 | `fn_80023c70` | high | returns `0x80377A04` |
| `0x80023C80` | 12 | `fn_80023c80` | high | returns `0x80377A04` |
| `0x80023C90` | 12 | `fn_80023c90` | high | returns `0x80377A04` |
| `0x80023CA0` | 12 | `fn_80023ca0` | high | returns `0x80377A04` |
| `0x80023CB0` | 12 | `fn_80023cb0` | high | returns `0x80377A04` |
| `0x80023F80` | 296 | `fn_80023f80` | medium | builds the `bros` model and its two animation records |

Total: 640 bytes, 10/10 translated.

## Raw words

The six 12-byte bodies were checked against the shipped words before being
written; each is exactly `3C608037 38637A04 4E800020` (`lis r3,0x8037` /
`addi r3,r3,0x7A04` / `blr`), which is a constant-address accessor, not a
`blr` stub. The four instructions after the terminator (`00000000`) belong to
the padding between functions.

`0x80023C00` and `0x80023C30` are byte-identical to `fn_80016a20` /
`fn_80016a50` except for the `addi r12,r5,0x18` / `+0x24` interface selector,
so they reuse the existing `FN_802DCBFC` stub constant and `call3` shape.

## `0x80023F80` in detail

`FUN_80023f80` is the `bros` twin of `fn_8001fa70` (`daikonbou`), with three
differences:

- the archive is `"g3d/bros.brres"` at `0x803023C0`, not a `.sdata` pointer;
- the model name is the inline `.sdata` string `"bros"` at `SDA_BASE - 0x7E68`
  and the first animation name is the inline string `"walk"` at
  `SDA_BASE - 0x7E60` (both read as `addi r4,r13,-32360` / `-32352`);
- it uses `0x80064bd0` + `0x801650d0` for the first pair and skips the scale
  application (`0x80169f80`) that the `daikonbou` body performs.

The buffer detach call is `bl 0x800690e0`, which Ghidra resolves as
`thunk_FUN_8016a8e0`; the transcription calls the thunk address itself
(`FN_800690E0`) so the instruction stream matches.

## Evidence

- Disassembly: `work/dol/main.elf` via `ppc159.js`, `LEN=80`/`296`, `STOP=0`.
- Decompilation: `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` (lowercase)
  for all ten addresses; `work/ghidra/main/export/` was not needed as a
  fallback because the `main-gb` export was complete for this range.
- Strings: `0x803023C0` = `"g3d/bros.brres"`; `0x80427818` = `"bros"`;
  `0x80427820` = `"walk"`.

## Verification

`work/wave203/laneF_block.rs` was appended to a temporary copy of the crate's
`src/ui/hud.rs` and `cargo check --lib` exited 0 (see the lane report).
