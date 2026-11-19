# Coverage - wave 23 NW4R layout text-window helpers (`0x801886A0..0x80193EF0`)

## How to read this file

- **Scope** - the twenty-third `lyt` translation pass. It adds 25 bodies to the
  semantic module `src/ui/lyt.rs` and creates no staging module. The pass
  continues the layout band after wave 20 (`0x801819C0..0x801909F0`) with the
  small helpers that surround the per-message text-window records, plus the
  table `atan2` used by the pane matrix.
- **Addresses** are virtual `sys/main.dol` addresses (image base
  `0x80004000`), written as plain hex (`80190fb0`) in the CSV and with the
  `0x` prefix in this document and in the Rust doc comments.
- **Evidence** is the Ghidra C export under
  `work/ghidra/main-gb/export/`; every row names its file.
- **Status** follows `docs/RUST_STYLE.md`: `translated` = complete control
  flow and every call accounted for; `partial` = flow and major effects
  present, one operand not literal; `stub` = address/role only.
- **Confidence** measures the evidence, not code quality. `medium` marks a
  body whose control flow, offsets and stores are exact but whose callee
  roles, object fields or paired-single quantisation were recovered from call
  shapes.
- Start with the **summary**, then the **function table**. The **cluster
  notes** explain how the records connect to the still-untranslated builders
  `0x80191260` and `0x80191D20` and to the shared id table in
  `src/runtime/ids.rs`.

## Summary

| metric | value |
|---|---:|
| functions added to `src/ui/lyt.rs` | 25 |
| bytes translated | 3,564 |
| `translated` / `partial` / `stub` | 25 / 0 / 0 |
| high confidence | 19 |
| medium confidence | 6 |
| new staging modules | 0 |
| files written | `src/ui/lyt.rs`, this document, `ledger/agents/wave23-lyt.csv` |

Address span: `0x801886A0..0x80193EF0`. The selection was made after checking
both the merged ledger and the destination file: every address below was absent
from `ledger/functions.csv` and from the `fn_` definitions in
`src/ui/lyt.rs`, and every body exists in the Ghidra export. No ledger row was
created from a guess.

Before this pass the layout band `0x80180000..0x80192000` had four untranslated
bodies; three remain deliberately deferred and the fourth (`0x801886A0`) is
selected here (see **Selection method**). This pass therefore moved into the
adjacent text-window block above `0x80190F00`; the records it manipulates are
2D layout cells and use the same 1/256-turn angle unit as the wave-20
sine/cosine lookups.

## Function table

| address | Rust function | role | status | conf. | bytes | evidence |
|---|---|---|---|---:|---:|---|
| `0x801886A0` | `fn_801886a0` | table `atan2` in 1/256-turn units, zero on `+Y` | translated | medium | 448 | `801886a0_FUN_801886a0.c` |
| `0x80190FB0` | `fn_80190fb0` | select the two colour/offset tables of a window record | translated | high | 160 | `80190fb0_FUN_80190fb0.c` |
| `0x80191BB0` | `fn_80191bb0` | clear a window record's live-object pointer | translated | high | 44 | `80191bb0_FUN_80191bb0.c` |
| `0x80191BE0` | `fn_80191be0` | store record `+0x08`, set dirty flag `0x10000000` | translated | high | 56 | `80191be0_FUN_80191be0.c` |
| `0x80191C20` | `fn_80191c20` | store record `+0x0C`, set dirty flag `0x40000000` | translated | high | 56 | `80191c20_FUN_80191c20.c` |
| `0x80191C60` | `fn_80191c60` | clamp and store record `+0x10`, rebuild tables | translated | high | 128 | `80191c60_FUN_80191c60.c` |
| `0x80191CE0` | `fn_80191ce0` | store record `+0x14`, set dirty flag `0x40000000` | translated | high | 56 | `80191ce0_FUN_80191ce0.c` |
| `0x80192900` | `fn_80192900` | remove a node from the global message list | translated | high | 140 | `80192900_FUN_80192900.c` |
| `0x80192990` | `fn_80192990` | reset the sixteen message records and guard | translated | high | 216 | `80192990_FUN_80192990.c` |
| `0x80192A70` | `fn_80192a70` | retire sixteen records and drain the list | translated | high | 108 | `80192a70_FUN_80192a70.c` |
| `0x80192AE0` | `fn_80192ae0` | build, zero and link a message object | translated | medium | 392 | `80192ae0_FUN_80192ae0.c` |
| `0x80192C70` | `fn_80192c70` | unlink one message from records and list | translated | medium | 168 | `80192c70_FUN_80192c70.c` |
| `0x80192D80` | `fn_80192d80` | seed sixteen position/colour column slots | translated | medium | 220 | `80192d80_FUN_80192d80.c` |
| `0x80193090` | `fn_80193090` | execute one three-byte window command | translated | high | 284 | `80193090_FUN_80193090.c` |
| `0x801931B0` | `fn_801931b0` | drain the three-byte command buffer | translated | high | 104 | `801931b0_FUN_801931b0.c` |
| `0x801934F0` | `fn_801934f0` | advance one column's intro/outro state | translated | high | 268 | `801934f0_FUN_801934f0.c` |
| `0x80193600` | `fn_80193600` | compute one column's text position | translated | high | 40 | `80193600_FUN_80193600.c` |
| `0x80193630` | `fn_80193630` | copy one column's byte to `object + 0x0F` | translated | high | 24 | `80193630_FUN_80193630.c` |
| `0x80193650` | `fn_80193650` | fixed-point position sum, shifted by 16 | translated | high | 20 | `80193650_FUN_80193650.c` |
| `0x80193670` | `fn_80193670` | fixed-point column position, shifted by 16 | translated | high | 36 | `80193670_FUN_80193670.c` |
| `0x801936A0` | `fn_801936a0` | publish position/texture/depth/scale to a record | translated | high | 160 | `801936a0_FUN_801936a0.c` |
| `0x80193740` | `fn_80193740` | frame count through percent/minute/second tables | translated | medium | 272 | `80193740_FUN_80193740.c` |
| `0x80193850` | `fn_80193850` | compute spacing and glyph count for a text block | translated | medium | 104 | `80193850_FUN_80193850.c` |
| `0x80193E00` | `fn_80193e00` | dispatch a record on its mode halfword | translated | high | 48 | `80193e00_FUN_80193e00.c` |
| `0x80193EF0` | `fn_80193ef0` | mark a record as state `3` | translated | high | 12 | `80193ef0_FUN_80193ef0.c` |

All evidence paths are relative to `work/ghidra/main-gb/export/`.

## Cluster notes

### A. Window-record access (`0x80190FB0..0x80191CE0`)

All five accessors resolve a record through the same expression:

```text
record = *(u32 *)0x803780EC + (signed char)*(u8 *)(0x80378088 + message[+0x18]) * 100
```

`0x80378088` is the inverse id -> slot table that `src/runtime/ids.rs` already
documents; this pass only reads it. The 100-byte stride is observed here and in
the still-untranslated builders `0x80191260` and `0x80191D20`, which write the
per-character halfwords at `+0x30..+0x62`. `0x80190FB0` selects one of two
colour/offset table pairs from the record index at `+0x10`, stores the entry,
its mirrored entry (`0x7F - index`), the first table halfword at `+0x20`, a
sign-extended bias at `+0x24` and two halfwords at `+0x28`/`+0x2C`. Mode `2`
uses the signed halfword tables `0x8032D2D0`/`0x8032D3D0`; the default uses the
word table `0x8032D0D0` and zero halfwords. The four setters store one field
each and mark the record dirty with the observed flag bits.

### B. Message-list lifecycle (`0x80192900..0x80192C70`)

The global list head is `0x80378738`; each node's first word is the next link.
`0x80192990` resets the static 16 x 0x4C message table at `0x80378740`,
`0x80192AE0` builds a new object and pushes it, and `0x80192C70` unlinks one
object from every window record and from the list. Both list operations run
between `0x801B1140` and `0x801B1180`, which the original treats as a lock pair.
`0x80192900` is the pointer-splicing removal; `0x80192A70` retires all sixteen
records and then drains whatever remains in the list.

### C. Column initialisation and commands (`0x80192D80..0x801931B0`)

`0x80192D80` seeds the sixteen column slots in two passes of eight. Every word
slot receives the same base value in the original, so the Rust body keeps the
two-pass shape and the observed `+0x20` word stride / `+8` byte stride.
`0x80193090` is the command interpreter: the first byte packs a 4-bit opcode
and a 4-bit column, the second is a row/value, the third a payload. Opcodes
`8`, `9`, `0xB` and `0xC` are the only ones present; `0xB` selects one of three
column fields by row (`7`, `10`, `0x5B`). `0x801931B0` walks the 3-byte commands
at `object + 0xFC`.

### D. Column layout and timing (`0x801934F0..0x80193850`)

`0x801934F0` is the four-state intro/outro machine. `0x80193600`,
`0x80193630`, `0x80193650`, `0x80193670` and `0x801936A0` read the fixed-point
column fields from the text block at `object + 0x08` and publish them to the
window record through cluster A. `0x80193740` converts the 16.16 frame count at
`+0x28` through three single-precision tables (percent, minute, second) and
returns their product. `0x80193850` computes the spacing from the column byte
and the block's `+0x02` halfword, and stores a two-step `+0x28` result exactly
as the original does.

### E. Angle table (`0x801886A0`)

This is the layout `atan2` used by the pane matrix. Zero is the `+Y` axis:
`(0, 1)` returns `0`, `(1, 0)` returns `64` and `(-1, 0)` returns `-64`, the
same 1/256-turn unit as `fn_801885b0` and `fn_80188630`. The table at
`0x8032C5D8` holds `{angle, slope}` pairs; entry 1 is `1.272825`, which is
`atan(1/32)` expressed in 1/256 turns. The quadrant logic and the interpolated
table lookup are the same shape as the wave-20 sine/cosine bodies.

## Selection method (why these 25)

Before this pass the layout band `0x80180000..0x80192000` was exhausted
except for four bodies. Three were deliberately left:

| address | size | why |
|---|---:|---|
| `0x80180780` | 4,484 | the `BRLAN`/animation-transform constructor, a 900-line state machine; too large for a faithful pass of this size |
| `0x80185870` | 1,092 | five floats arrive in `f27..f31`; Ghidra's signature loses them |
| `0x80185F10` | 360 | wraps `0x80185870` with the same lost float arguments |

The fourth, `0x801886A0`, is the only one with a complete scalar signature, so
it is included. The remaining 24 rows continue the adjacent UI text-window
block, which is still `lyt`-adjacent layout code: it positions character cells,
uses the pane/block pointer at `+0x10` and the shared id table, and is called by
the layout builders `0x80191260`/`0x80191D20`. Every selected body has one
Ghidra file and no unresolved `r13`/`r2` table other than the explicit data
addresses named in the Rust.

## Unknowns and limits

1. **Role of the window records.** The 100-byte stride, the 16-slot table and
   the per-character builders are observed, but no header in this range names
   the object. "Window record" and "column" are `// inferred:` from the fields
   and call shapes.
2. **Hidden `this` operands.** `0x80192C70` and `0x80192D80` are printed as
   `void` by Ghidra but receive the object in `r3`; the register-save helpers
   `0x802DCD6C`/`0x802DCDB8` preserve that register. The Rust signatures make
   the operand explicit. The same reconstruction is used for the three mode
   updaters called from `0x80193E00`.
3. **Call argument loss at `0x80193EF0`.** The three call sites print
   `FUN_80193ef0()` without arguments; the only useful value in `r3` at each
   site is the non-null record pointer that was just tested, so the Rust body
   passes that pointer.
4. **Paired-single quantisation.** `0x801886A0` builds its fraction through an
   8-bit quantised paired-single store. The translation keeps the exact
   truncated index (`scaled & 0xFF`) and the single-precision fraction, as the
   wave-20 trig bodies do; a bit-exact quantised result is not claimed.
5. **Unrolled stores.** `0x80192990` and `0x80192D80` are fully unrolled in the
   Ghidra output. The Rust bodies use counted loops; the addresses and values
   are the same, and normal (non-MMIO) memory makes store order unobservable.
6. **Table meanings.** `0x8032D0D0`, `0x8032D2D0`, `0x8032D3D0`, `0x8032D6D0`,
   `0x8032D860`, `0x8032D8D0`, `0x8032DA60`, `0x8032DBF0`, `0x8032DC20` and
   `0x8032DC50` are named by their use only (colour, position, percent, minute,
   second). The names are not recovered symbols.
7. **Untranslated callees.** `0x80192E60`, `0x801939C0`, `0x80193BE0`,
   `0x80193CF0` and `0x80193F00` are still untranslated and are called through
   `crate::platform` by address, as are the runtime id helper `0x801928D0` and
   the message release `0x801A0300`.
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
`ledger/agents/wave23-lyt.csv`. `src/lib.rs`, every `mod.rs`, `Cargo.toml`,
`src/platform.rs`, the generated ledger index and all other lanes' files were
left untouched. No staging module was created.
