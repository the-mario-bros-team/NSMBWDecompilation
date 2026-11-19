# Coverage - Wave 215, lane C (`engine::dsys`, the `0x800F10B0..0x800F18AC` binding cluster)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/engine/dsys.rs` | `engine::dsys` | 10 | 1600 | 10 | 0 | 0 |

Status split: 10 `translated`, 0 `partial`, 0 deferred.

Body block: `work/wave215/laneC_block.rs`. Ledger rows:
`ledger/agents/wave215-laneC.csv`.

Scratch verification: the block was appended to a copy of
`src/engine/dsys.rs` in a scratch crate (with `Cargo.toml`/`Cargo.lock`) at
`%TEMP%\opencode\w215c`, and `cargo check --lib` exits `0` (`Checking nsmbw
v0.1.0` -> `Finished dev profile` in 5.06 s). The block adds no warnings: the
crate's 23 pre-existing warnings are all in `src/ui/menu.rs`, and no line of
`src/engine/dsys.rs` is named in the log.

## How to read this file

* **Address** - virtual entry point in the `DOL` image, not a file offset.
* **Name** - the Ghidra name; the Rust body keeps it as `fn_<address>`. New
  constants in the block are suffixed `_C`.
* **Size** - the `functions.tsv` value, which the lane checker compares
  against. Every one was re-derived from the prologue and the closing `blr`
  (or from the tail branch of the four leaf entries) and all ten match the tsv
  exactly; the real extents are given in the per-body doc blocks. Four entries
  (`0x800F1150`, `0x800F11C0`, `0x800F11D0`, `0x800F11E0`) are 8 bytes of code
  with 8 bytes of zero padding to the next 0x10 boundary - genuine leaves, not
  16/20-byte prologue artefacts.
* **Confidence** - `high` everywhere: every address, field offset, stride,
  immediate and branch in these ten bodies is literal in the disassembly. The
  only inferred material is the *purpose* of the two float arguments and of the
  `+0x100`-bit flag, which is marked `// inferred:` in the block.

`r13 = 0x8042F680` and `r2 = 0x80433080` throughout, so:

| expression | address | value / use |
|---|---|---|
| `r13 - 0x6DD0` | `0x804288B0` | static key passed **by address** (`addi`) |
| `r13 - 0x6DCC` | `0x804288B4` | key word **loaded** (`lwz`) |
| `r13 - 0x6EE0` | `0x804287A0` | key word **loaded** (`lwz`) |
| `r2 - 0x61AC` | `0x8042CED4` | `0.0f`, the "negative" test |
| `r2 - 0x61B0` | `0x8042CED0` | `1.0f`, the `-1.0f` adjustment |
| `lis 0x8032` / `addi -17756` | `0x8031BAA4` | key table of `0x800F10B0` |

The existing `F_CONST_61AC` / `F_CONST_61B0` constants were reused rather than
redeclared, as were `FN_800F11F0`, `FN_800F1390`, `FN_800F1550`,
`FN_800F1850`, `FN_8023A0B0/190/270`, `FN_8023B2A0`, `FN_8023B760`,
`FN_802DC69C`, `FN_80168400`, `FN_80167B20/40/80`, `FN_801662F0/6310/6360`,
`FN_80165FD0`, `FN_80167420`, `FN_80167800` and the helpers `call_f64_ptr`
(`(f64, u32)`), `call_f64_u32x2` (`(f64, u32, u32)`) and
`call_u32x2_f32_ret` (`(u32, u32) -> f32`).

## What this pass covers

These ten bodies are the implementation half of the effect/resource binding
whose call half is already in this file: `fn_800ee610` (0x800EE610, translated
earlier) calls `0x800F1390`, `0x800F11F0` and `0x800F1550` twice each with
`f1 = 1.0f`, `f2 = 0.0f`, the object, a context word, a 16-byte name buffer and
zero, and calls `0x800F1850` last. Until now those addresses existed only as
`const FN_…` targets. The cluster has two halves:

1. **The small name-table accessors** (`0x800F1150`..`0x800F11E0` plus
   `0x800F10B0`). They all funnel into `0x800F0F70`, which is
   `0x80164F00(0x80168400(object+8), key)` turned into a boolean. The four
   `+0x7DC` bodies describe a second, fixed-size string table:

   | offset | meaning (`// inferred:`) |
   |---|---|
   | `+0x7DC` | table header; the word at `+0x7E0` is the live count |
   | `+0x7E0` | `u32` count (read by `0x800F11D0`) |
   | `+0x7E4` | first entry: one 0xE-byte NUL-terminated key per slot |
   | entry | compared with `0x802E1BD0(entry, key, 0xE)`; `0x800FD720` returns `table + index*0xE + 8`, which is the entry itself |

   `0x800FB4B0` (already in this module) uses them exactly that way: probe with
   `0x800F11C0`, then walk `0..0x800F11D0(...)` resolving each name with
   `0x800F11E0`.

2. **The three binding bodies.** They share one shape, with different record
   families:

   | body | sub-object | count | records | probe / duplicate test | register | float writers |
   |---|---|---|---|---|---|---|
   | `0x800F11F0` | (none) | `+0x7CC` | `0xD8`, stride `0x48`, names at `0xC8` | `0x8023A0B0` / `0x800F0CC0` | `0x801650D0` | `0x801680E0` / `0x80168FE0` |
   | `0x800F1390` | `+0x9C` | `+0x7D0` | `0x688`, stride `0x14` | `0x8023A270` / `0x800F0B60` | `0x80167420` + vtable `+0x18` | `0x80167B40` / `0x80167B20` |
   | `0x800F1550` | `+0x70` | `+0x7C8` | `0x548`, stride `0x14` | `0x8023A190` / `0x800F0C10` | `0x80165C30` + vtable `+0x18` | `0x80166310` / `0x801662F0` |

   All three: return the stored index (the old count) or `-1`; copy the name
   with `0x802E19F8(dst, key, 0x10)`; force the byte at `min(strlen(key),15)`
   to `0`; and increment the count as their last store. The `0x14`-stride
   families keep the name *inside* the record (`record[0..16]`), the `0x48`
   family keeps a parallel name array 0x10 bytes before each record.

   The two `0x14` bodies also carry a one-shot initialisation that only runs
   while the count is still `0`: `0x80167420` / `0x80165C30` with six
   arguments, followed by the virtual slot `+0x18` of the `+0x8` handle with
   `(handle, sub_object)`.

   `// inferred:` the two floats are a start offset and a per-record extent;
   the `second < 0` branch replaces it with `record[+0x28] - 1.0f` (0x48
   family) or `0x80167B80(...) - 1.0f` / `0x80166360(...) - 1.0f` (the
   `0x14` families), computed in **single** precision exactly as the
   `fcmpo`/`fsubs` pair shows. All three call sites in `fn_800ee610` pass
   `f1 = 1.0f`, `f2 = 0.0f`, so the adjustment does not fire there.

3. **The two flag helpers.**
   `0x800F17B0` probes the loaded key at `0x804287A0`, resolves the record
   with `0x800F0F30` + `0x8023B2A0` and then either `ori`s `0x100` into, or
   masks `0x100` out of, the record's `+0x14` word (`rlwinm r0,r0,0,24,22` -
   a masked clear, not a byte store). `0x800F1850` resolves the loaded key at
   `0x804288B4` with `0x8023B760`, guards on `record + 0x14` and then stamps
   `3` through `0x8023D0A0(&record_plus_0x14, 3)`, which writes to that
   record's `+4`.

## Notes on the four 8-byte entries

They were decoded word for word from `work/dol/main.elf` rather than trusted to
the export, because Ghidra prints a one-argument `void` signature for each:

| address | raw words | decode |
|---|---|---|
| `0x800F1150` | `388D9230`, `4BFFFE1C` | `addi r4,r13,-28112`; `b 0x800F0F70` |
| `0x800F11C0` | `386307DC`, `4800C58C` | `addi r3,r3,2012`; `b 0x800FD750` |
| `0x800F11D0` | `806307E0`, `4E800020` | `lwz r3,0x7E0(r3)`; `blr` |
| `0x800F11E0` | `386307DC`, `4800C53C` | `addi r3,r3,2012`; `b 0x800FD720` |

Two consequences are recorded in the bodies:

* the three tail branches return the callee's `r3`, and their callers
  (`0x800F1160`, `0x800FB4B0`, `0x800FB6C0`) all test or store it, so the
  bodies return `u32` although the exports say `void`;
* the `b` leaves `r4` (and the rest of the incoming argument registers)
  untouched, so `0x800F11C0` and `0x800F11E0` are modelled with the second
  argument that their callees `0x800FD750` / `0x800FD720` actually read
  (`key` / `index`), and `0x800F1850`/`0x800F17B0` with the word their caller
  `0x800EE610` passes.

## Remaining gaps

None for this lane. Every one of the ten addresses is `translated` with no
deferral. What stays opaque is the interior of the callees (`0x800F0F70`,
`0x800F0CC0`, `0x8023A0B0`/`0x8023A270`/`0x8023A190`, `0x801650D0`,
`0x80167420`, `0x80165C30`, `0x802E19F8`, `0x800FD750`, `0x800FD720`) and the
upper word of both root-heap-style handles at `+4`/`+8`; their behaviour is
called, not reconstructed, which is why the bodies are `translated` rather
than `partial`.
