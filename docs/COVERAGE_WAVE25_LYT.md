# Coverage - wave 25 NW4R layout window records (`0x80191050..0x801949B0`)

## How to read this file

- **Scope** - the twenty-fifth `lyt` translation pass. It adds 24 bodies to the
  semantic module `src/ui/lyt.rs` and creates no staging module. The pass
  finishes the text-window record band that wave 23 started: the table
  initialiser, the three mode updates, the command stream, the create/retire
  paths and the window-manager singleton.
- **Addresses** are virtual `sys/main.dol` addresses (image base
  `0x80004000`), written as plain hex (`80191050`) in the CSV and with the
  `0x` prefix in this document and in the Rust doc comments.
- **Evidence** is the Ghidra C export under
  `work/ghidra/main-gb/export/`; every row names its file.
- **Status** follows `docs/RUST_STYLE.md`: `translated` = complete control
  flow and every call accounted for; `partial` = flow and major effects
  present, one operand not literal; `stub` = address/role only.
- **Confidence** measures the evidence, not code quality. `medium` marks a
  body whose control flow, offsets and stores are exact but whose hidden
  operands, callee roles or object fields were recovered from call shapes and
  the register-save helpers.
- Start with the **summary**, then the **function table**. The **cluster
  notes** explain how the records connect to the message objects around
  `0x80378C08` and to the still-untranslated builders `0x80191260` and
  `0x80193220`.

## Summary

| metric | value |
|---|---:|
| functions added to `src/ui/lyt.rs` | 24 |
| bytes translated | 5,236 |
| `translated` / `partial` / `stub` | 24 / 0 / 0 |
| high confidence | 15 |
| medium confidence | 9 |
| new staging modules | 0 |
| files written | `src/ui/lyt.rs`, this document, `ledger/agents/wave25-lyt.csv` |

Address span: `0x80191050..0x801949B0`. The selection was made after checking
both the merged ledger and the destination file: every address below was absent
from `ledger/functions.csv` and from the `fn_` definitions in
`src/ui/lyt.rs`, and every body exists in the Ghidra export. No ledger row was
created from a guess.

This pass completes the record band below `0x80194A00`. The three larger
bodies left in the same band (`0x80191260`, `0x80193220`, `0x80191D20`) were
deliberately not selected: they are 2,376, 716 and 2,808 bytes of float/glyph
builder code and would dominate a pass whose value is the complete record
lifecycle around them. They are called by address, not invented.

## Function table

| address | Rust function | role | status | conf. | bytes | evidence |
|---|---|---|---|---|---:|---|
| `0x80191050` | `fn_80191050` | initialise the sixteen window records and handle tables | translated | high | 512 | `80191050_FUN_80191050.c` |
| `0x80192D20` | `fn_80192d20` | append one three-byte command to a message buffer | translated | high | 80 | `80192d20_FUN_80192d20.c` |
| `0x80192D70` | `fn_80192d70` | store a 16.16 value at `+0x68` | translated | high | 12 | `80192d70_FUN_80192d70.c` |
| `0x80192E60` | `fn_80192e60` | create or clear one `[column][row]` window cell | translated | medium | 556 | `80192e60_FUN_80192e60.c` |
| `0x801938C0` | `fn_801938c0` | publish frame count to window `+0xCE..+0xDA` | translated | medium | 156 | `801938c0_FUN_801938c0.c` |
| `0x80193960` | `fn_80193960` | store the frame-count word at window `+0xCE` | translated | medium | 92 | `80193960_FUN_80193960.c` |
| `0x801939C0` | `fn_801939c0` | layout a multi-glyph cell with colour/UV copy | translated | high | 532 | `801939c0_FUN_801939c0.c` |
| `0x80193BE0` | `fn_80193be0` | mode-1 single-glyph cell layout | translated | high | 272 | `80193be0_FUN_80193be0.c` |
| `0x80193CF0` | `fn_80193cf0` | mode-2 single-glyph cell layout | translated | high | 272 | `80193cf0_FUN_80193cf0.c` |
| `0x80193E40` | `fn_80193e40` | release one text object and its message record | translated | high | 164 | `80193e40_FUN_80193e40.c` |
| `0x80193F00` | `fn_80193f00` | advance one message record and retire it | translated | high | 240 | `80193f00_FUN_80193f00.c` |
| `0x80193FF0` | `fn_80193ff0` | decode one byte stream into three-byte commands | translated | medium | 704 | `80193ff0_FUN_80193ff0.c` |
| `0x801943E0` | `fn_801943e0` | parse a track descriptor into the window object | translated | medium | 136 | `801943e0_FUN_801943e0.c` |
| `0x80194470` | `fn_80194470` | raise the window-manager list guard | translated | high | 40 | `80194470_FUN_80194470.c` |
| `0x801944A0` | `fn_801944a0` | advance every live window's command stream | translated | medium | 380 | `801944a0_FUN_801944a0.c` |
| `0x80194630` | `fn_80194630` | build and link one window object | translated | high | 144 | `80194630_FUN_80194630.c` |
| `0x801946C0` | `fn_801946c0` | unlink a window and destroy its message | translated | medium | 152 | `801946c0_FUN_801946c0.c` |
| `0x80194760` | `fn_80194760` | reset or re-seed a window's command records | translated | medium | 316 | `80194760_FUN_80194760.c` |
| `0x801948B0` | `fn_801948b0` | read the window-manager word at `+0x04` | translated | high | 8 | `801948b0_FUN_801948b0.c` |
| `0x801948C0` | `fn_801948c0` | forward a fixed-point value to `+0x14` | translated | high | 8 | `801948c0_FUN_801948c0.c` |
| `0x801948D0` | `fn_801948d0` | destroy the child object at manager `+0x8DC` | translated | high | 120 | `801948d0_FUN_801948d0.c` |
| `0x80194950` | `fn_80194950` | allocate/return the window-manager singleton | translated | high | 76 | `80194950_FUN_80194950.c` |
| `0x801949A0` | `fn_801949a0` | clear both singleton slots | translated | high | 16 | `801949a0_FUN_801949a0.c` |
| `0x801949B0` | `fn_801949b0` | construct the 0x970-byte window manager | translated | medium | 248 | `801949b0_FUN_801949b0.c` |

All evidence paths are relative to `work/ghidra/main-gb/export/`.

## Cluster notes

### A. Record table initialisation (`0x80191050`)

`0x80191050` is the missing initialiser for the 16 x 100-byte records that
wave 23 addressed through `0x803780EC`. It runs only once (`0x80378730` is the
guard) and only after the message singleton `0x801A00B0` exists. It fills the
16 forward slots at `0x80378078` and the first `0x60` bytes at `0x80378088`
with `0xFF`; the original only clears 96 of the 256 inverse entries. Every
record receives `+0x04 = 0x50000000`, `+0x0C = 0xFFFFFC40`, `+0x10 = 0x40`,
zero at `+0x00`/`+0x08`/`+0x14` and thirteen zero halfwords at
`+0x30..+0x60`, then calls `0x80190FB0` to fill `+0x18..+0x2C`. Mode becomes
`1`.

### B. Per-record layout (`0x801938C0..0x80193CF0`)

`0x801939C0`, `0x80193BE0` and `0x80193CF0` are the three bodies dispatched by
`0x80193E00` on the halfword at `*record[+0x18]`. They share the same skeleton:

- `record + 0x04` is the layout window object that receives `+0x96..+0xE0`;
- `record + 0x08` is the message object (base at `+0x24` for mode 0, `+0x1C`
  for mode 1 and `+0x20` for mode 2);
- `record + 0x10` is the glyph block (`+0x08` start, `+0x0C` count);
- `record + 0x18` is a block with `+0x04`/`+0x08` layout offsets;
- `record + 0x1C` is a 0x2E-byte colour/UV block copied to `+0xA6..+0xE0`.

The multi-glyph body maps glyph indices onto a 14-row, 16-pixel grid. The
`0x124924925` multiply is the compiler's divide-by-14: after the high-word
shift and `& 0xFFFFFFF0` it produces `(index / 14) * 16`. The three bodies end
by rewriting the window flag bits `0x7800` with state `0x8400` (plus
`0x48000` for the multi-glyph case). `0x801938C0` and `0x80193960` convert the
record's fixed-point frame count through `0x80193740` and the float at
`0x802F5F70`.

### C. Command stream (`0x80192D20`, `0x80193FF0`, `0x801944A0`, `0x80194760`)

The embedded message object at window `+0x14` owns a command buffer at
`+0xFC`, a cursor at `+0x3FC`, a count at `+0x400` and sixteen command records
at `+0x241C` (0x28-byte stride). `0x80192D20` appends one three-byte command.
`0x80193FF0` interprets a byte stream: bytes above `0x7F` become the pending
command at record `+0x10`; `0xF0`/`0xF7` skip a varint-length blob; `0xFF`
handles the extended `/` (end of stream) and `Q` (24-bit duration plus derived
frame value) tags; every other command reads 0, 1 or 2 payload bytes from the
table at `0x8032DDD0` and appends the command. `0x801944A0` advances all
windows by consuming their `+0x20` durations, and `0x80194760` either restarts
the records from their `+0x04` streams (modes 1/2) or appends sixteen
`0xB0..0xBF 0x7B 0x00` commands (modes 0/3).

### D. Create and retire (`0x80192E60`, `0x80193E40`, `0x80193F00`, `0x801943E0`)

`0x80192E60` is the create/clear pair used by command opcode `9`. It allocates
a text object through `0x801A0380`, reserves an id through `0x80192820`, and
fills one 0x4C-byte message record. A glyph index of `0xFFFF` rolls the record
back. `0x80193E40` is the allocator's release callback: it frees the id, clears
the `[column][row]` cell and decrements the message live count at `+0x404`.
`0x80193F00` retires a record by index after advancing its state machine
`0x801934F0`. `0x801943E0` parses a track descriptor whose data starts at
`descriptor + 8 + descriptor[+4]` and feeds the `MTrk`-tag parser `0x801942C0`.

### E. Window manager (`0x80194470..0x801949B0`)

`0x80194470` raises the one-time guard at `0x80378C0C`. `0x80194630` links a
window object (message at `+0x14`) into the list head at `0x80378C08` under the
`0x801B1140`/`0x801B1180` interrupt pair. `0x801946C0` rebuilds that list
without the object and destroys its embedded message through `0x80192C70`.
`0x801948B0`/`0x801948C0` are the two tiny accessors of the singleton class.
`0x80194950` lazily allocates the 0x970-byte manager and its child at `+0x8DC`;
`0x801949A0` clears both singleton slots; `0x801949B0` is the constructor,
installing the four vtables (`0x8032DEF0`, `0x8032DF00`, `0x8032DF14`,
`0x8032DFC4`) and clearing the manager counters and flags.

## Selection method (why these 24)

The `lyt` band below `0x80194A00` had 149 untranslated bodies in
`0x80180000..0x801A0000`, but most of that tail is audio/input code above
`0x8019B000`. Inside the layout range the remaining bodies fall into four
groups:

| group | addresses | why selected / deferred |
|---|---|---|
| record lifecycle and mode updates | `0x80191050..0x80193F00` | selected: complete scalar control flow and direct evidence |
| command stream | `0x80192D20`, `0x80193FF0`, `0x801944A0`, `0x80194760` | selected: the largest group with exact byte-level flow |
| window manager | `0x801943E0..0x801949B0` | selected: the singleton/list code that ties the records together |
| glyph/float builders | `0x80191260`, `0x80193220`, `0x80191D20` | deferred: 2,376 / 716 / 2,808 bytes of builder code; called by address |

Every selected body has one Ghidra file, no unresolved `r13`/`r2` table other
than the explicit SDA bases in `src/ui/lyt.rs`, and no GQR/paired-single code.
The two 8-byte accessors `0x801948B0`/`0x801948C0` are included because their
bodies are complete: they read the manager's `+0x04` field and write the
`+0x14` sub-object.

## Unknowns and limits

1. **Hidden operands.** `0x80192E60` is printed with four arguments but its
   object and column arrive through the `0x802DCD64` register-save helper;
   `0x801946C0` and `0x80194760` are printed as `void` and hide `(this)` and
   `(this, mode)` behind `0x802DCD70`. The Rust signatures restore those
   operands and the doc comments say so.
2. **`0x802DCC4C`.** The MWCC double-to-int helper is modelled with Rust's
   saturating `as i32`. The two frame-count bodies only call it with small
   non-negative values, where the two agree; a bit-exact result for values
   above `2^31` is not claimed.
3. **`0x801943E0` count register.** Ghidra prints the second `0x801942C0`
   call with two arguments. The Rust body passes the halfword that was just
   stored at object `+0x08`, which is also the explicit third argument of the
   first call.
4. **Glyph record fields.** The `+0x10`/`+0x14` words of the 0x18-byte glyph
   record are used as 0x50- and 0x10-stride indices; the names "glyph record",
   "colour/UV block" and "cell" are inferred from the offsets and the copy
   shapes, not from a recovered header.
5. **Table meanings.** `0x8032DDD0` is named by its use (payload-size table);
   the float addresses `0x802F5F70..0x802F5F84` are named by their roles in
   the frame-count and `Q` bodies. None is a recovered symbol.
6. **Untranslated callees.** `0x80191260`, `0x80193220`, `0x801942C0`,
   `0x801A0380`, `0x802D46E0`, `0x802BA2D0`, `0x802BA330`, `0x802B9210` and
   `0x802B9280` are still untranslated and are called through `crate::platform`
   by address.
7. **Window-manager field names.** `0x801949B0` writes about forty fields; the
   offsets are exact but only the counters/flags read by `0x801944A0` and
   `0x80194760` are named.
8. **Host safety.** All bodies touch target memory through `crate::platform`;
   running them on a host still requires the platform helpers to be backed by
   an emulator or a mock.

## Verification

```text
cargo check
# Finished `dev` profile [unoptimized + debuginfo] target(s)
# no diagnostic named src/ui/lyt.rs

node tools/doc-lint.mjs src
# {"ok": true, "files": 68, "violations": 0, "byRule": {}}
```

This pass touched only `src/ui/lyt.rs`, this document and
`ledger/agents/wave25-lyt.csv`. `src/lib.rs`, every `mod.rs`, `Cargo.toml`,
`src/platform.rs`, the generated ledger index and all other lanes' files were
left untouched. No staging module was created.
