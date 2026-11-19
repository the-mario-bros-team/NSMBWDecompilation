# Coverage — wave 25 geometry band (`0x80005000..0x8000C000`)

## How to read this file

This is the completion record for wave 25 of the `main.dol` geometry band in
[`src/render/geometry.rs`](../src/render/geometry.rs). It answers five
questions for a reader with no context:

1. **What was added?** Eight function bodies that finish the band's
   **Ghidra-indexed** inventory: one 196-byte object-attachment function and
   seven sub-16-byte leaves/thunks that earlier waves skipped.
2. **Where is it?** Exclusively in
   [`src/render/geometry.rs`](../src/render/geometry.rs). No other Rust file,
   `lib.rs`, `mod.rs`, `Cargo.toml` or `platform.rs` was edited.
3. **How sure is it?** `confidence` measures the evidence. `high` means every
   branch, load, store and call operand was read from the Ghidra C **and** the
   raw instruction words. All eight rows are `high`.
4. **What is still missing?** The 1,632-byte GQR effect function
   `0x80007DE0` remains deliberately deferred. Separately, the raw image
   contains **27 function entries that Ghidra never indexed** (about 15.7 KB
   of code); they have no C export, so they are listed in
   [Appendix A](#appendix-a--unindexed-code-in-the-band) rather than
   translated under this lane's evidence rule.
5. **What is the entry point?** Read `fn_80007230` first (the only new
   multi-branch body), then `fn_80007220` and `fn_80007510`.

Addresses are plain lowercase hex **virtual** addresses (image base
`0x80004000`), not file offsets. Evidence paths are relative to the repository
root and live under `work/ghidra/main-gb/export/`. Raw instruction checks used
`work/dol/main.elf`.

Terms: **DOL** is the Wii executable container; **GX** is the Wii GPU;
**GQR** is a paired-single quantisation register; **r13** is the small-data
base (`0x8042F680`); **r2** is the second small-data base (`0x80433080`);
**SDA** is the small-data area. See `docs/GLOSSARY.md`.

Status follows `docs/RUST_STYLE.md`: `translated` means the recorded control
flow and every call are present; `partial` would mean one operand or callee is
unresolved (none here); `stub` is not used.

## Summary

| Metric | Count |
|---|---:|
| Ghidra entries in `[0x80005000, 0x8000C000)` | 37 |
| entries already translated before wave 25 | 28 |
| — in `src/render/geometry.rs` | 26 |
| — in `src/game/actor.rs` (out of lane) | 2 |
| entries untranslated at lane start | 9 |
| **new bodies in this batch** | **8** |
| bytes of new function size | 240 |
| `translated` | 8 |
| `partial` | 0 |
| `stub` | 0 |
| confidence `high` | 8 |
| remaining Ghidra-indexed entries | 1 (`0x80007DE0`) |
| **unindexed raw entries found in the gaps** | **27** (15,744 B, not translated) |

The arithmetic is exact: 28 already translated + 8 new + 1 deferred = 37.

## Selection method

1. `work/ghidra/main-gb/export/functions.tsv` was filtered to
   `0x80005000 <= entry < 0x8000C000`, giving 37 entries.
2. Every entry was crossed against `ledger/functions.csv` and against the
   `# Original` blocks in `src/render/geometry.rs`. Twenty-eight were already
   translated; the remaining nine are the subject of this lane.
3. The nine were read in Ghidra C form and then checked word by word in
   `work/dol/main.elf`. Three turned out to be 4-byte tail branches
   (`b target`); Ghidra's export for those addresses contains the **target**
   body because it followed the branch.
4. Eight of the nine are complete and are translated here. `0x80007DE0` is
   the exception (see "Deliberately deferred").
5. While validating the gaps between indexed entries, 27 additional entry
   points were found in the raw image. They are real code but have no Ghidra
   export, so they are recorded in Appendix A for a raw-instruction lane.

## Functions translated

| # | Address | Ghidra | Size | Rust | Role | Status | Conf. | Evidence |
|---:|---|---|---:|---|---|---|---|---|
| 1 | `80006790` | `FUN_80006790` | 4 | `fn_80006790` | Empty leaf: the whole body is `blr`. | translated | high | `80006790_FUN_80006790.c` |
| 2 | `800067a0` | `thunk_FUN_80163280` | 4 | `fn_800067a0` | Tail branch that drains the frame-callback list at `0x80377AB0`. | translated | high | `800067a0_thunk_FUN_80163280.c` |
| 3 | `80006a40` | `FUN_80006A40` | 12 | `fn_80006a40` | Stores `this + 0x0C` into `this + 0x04` (inline-member pointer). | translated | high | `80006a40_FUN_80006a40.c` |
| 4 | `80006be0` | `FUN_80006BE0` | 8 | `fn_80006be0` | Forwards to `fn_80006b60` with the third argument forced to `0`. | translated | high | `80006be0_FUN_80006be0.c` |
| 5 | `80006ea0` | `thunk_FUN_80163850` | 4 | `fn_80006ea0` | Tail branch that inserts a node into the sorted list at `0x80377AB0`. | translated | high | `80006ea0_thunk_FUN_80163850.c` |
| 6 | `80007220` | `FUN_80007220` | 8 | `fn_80007220` | Returns the child pointer at `this + 0x20`. | translated | high | `80007220_FUN_80007220.c` |
| 7 | `80007230` | `FUN_80007230` | 196 | `fn_80007230` | Attaches a child candidate and notifies the `+0x10` and `+8` members. | translated | high | `80007230_FUN_80007230.c` |
| 8 | `80007510` | `thunk_FUN_80164fd0` | 4 | `fn_80007510` | Tail branch to the actor/scene base initialiser `0x80164FD0`. | translated | high | `80007510_thunk_FUN_80164fd0.c` |

All eight also have a row in
[`ledger/agents/wave25-geometry.csv`](../ledger/agents/wave25-geometry.csv).

## Cluster A — Object attachment (`80007230`)

`0x80007230` is the only new body with a real branch tree. The raw words
`0x80007230..0x800072F0` decode to:

```
if (this->+0x20 != 0) return 1;
candidate = param_3;
if (candidate == 0) candidate = this->+0x84;
if (candidate == 0) return 0;
result = fn_80006a20(candidate, 0, param_2);
if (result == 0) return 0;
result = vcall_0C(this + 0x10, result, candidate + 0x0C);
if (result != 0) vcall_10(this + 8, this);
return result;
```

`fn_80006a20` is the already-translated tail dispatcher at `0x80006A20`; it
invokes virtual slot `+0x0C` of the candidate's member at `candidate + 0x0C`.
The two remaining calls are virtual slots of the members at `this + 0x10`
(slot `+0x0C`) and `this + 8` (slot `+0x10`).

The Ghidra C agrees line for line apart from its usual `(code **)` spelling
and its duplicated `if (iVar1 == 0) iVar1 = 0;` assignment, which the Rust
keeps as an early `return 0`.

**Class identity is unknown.** The object has no RTTI name in the image, so
the function keeps its address name. `param_2` is forwarded unchanged and its
meaning is `// unknown:`.

## Cluster B — Band-completion leaves (`80006790..80007510`)

The seven small entries are the ones wave 11 skipped under its 16-byte
minimum. They are not padding: each is a separately indexed Ghidra entry with
its own C file.

| address | raw body | translation shape |
|---|---|---|
| `80006790` | `blr` | empty function |
| `800067a0` | `b 80163280` | `platform::call0(FN_80163280)` |
| `80006a40` | `addi r0,r3,0xC; stw r0,4(r3); blr` | one store |
| `80006be0` | `li r5,0; b 80006b60` | call `fn_80006b60(this, p2, 0)` |
| `80006ea0` | `b 80163850` | `platform::call1(FN_80163850, node)` |
| `80007220` | `lwz r3,0x20(r3); blr` | one load |
| `80007510` | `b 80164fd0` | `platform::call3(FN_80164FD0, a, b, c)` |

The three tail branches deserve a note. `b` does **not** set LR, so the target
returns straight to the thunk's caller. Calling the target normally (as the
Rust does) produces the same observable result; the extra link-word write is
not observable outside the callee. Ghidra's export for `800067a0`,
`80006ea0` and `80007510` contains the full target body, not a 4-byte thunk;
the Rust therefore calls the target by absolute address instead of copying its
body a second time.

`fn_80006be0` is the one non-tail wrapper: it sets `r5 = 0` and branches to
`0x80006B60`, which is defined in this same file. Its Rust return type is
`u32` because the target returns `r3`; Ghidra prints `void` for the wrapper.

## Deliberately deferred — `80007DE0`

`0x80007DE0` (1,632 bytes) is **not** translated. It is the band's only
remaining Ghidra-indexed entry and the reason is unchanged from wave 11:

* every float crosses `psq_l`/`psq_st` and a GQR quantisation register;
* Ghidra's own export begins with 21 `Removing unreachable block` warnings and
  prints `unaff_GQR0`/`ldexpf` expansions instead of the real paired-single
  arithmetic;
* the body is a full GX effect setup (dozens of `0x801Cxxxx` state calls plus
  a 190-iteration loop), not a geometry primitive that this module owns.

Translating it faithfully needs a dedicated paired-single pass. It is left
listed here so the band's remaining work is visible rather than silently
declared complete.

## Appendix A — unindexed code in the band

The Ghidra index is **not** a complete inventory of the band. Sweeping the
gaps between indexed entries in `work/dol/main.elf` found 27 more entry points
with standard prologues or leaf bodies. They have no `FUN_*` export and no
`functions.tsv` row, so they could not be translated under this lane's
"Ghidra C is the source of truth" rule. They are recorded here so the next
lane can pick them up with a raw-instruction workflow.

The sizes are start-to-last-instruction; the body of `8000BA70` crosses the
band end and stops at `0x8000C010`.

| address | size | inferred role (from the raw calls) |
|---|---:|---|
| `800067b0` | 8 | `li r3,0x81; b 0x80163300` forwarder |
| `800067c0` | 8 | `li r3,0x80; b 0x80163390` forwarder |
| `800067d0` | 4 | `b 0x80163420` forwarder |
| `800067e0` | 108 | deleting destructor; member at `+0x24` |
| `80006850` | 24 | message handler: copies a byte to `+0xB8` and `+0xB9` |
| `80006930` | 112 | deleting destructor variant with a redundant null test |
| `80006a50` | 24 | magic-word dispatcher to `0x800B12D0` / `0x802B6720` |
| `80006a70` | 44 | byte normaliser: `0x800B1410` → mask `0xFF` → `0x800B1310` |
| `80006d90` | 88 | deleting destructor; releases the member at `+0` |
| `80006eb0` | 192 | refreshes a GX matrix block into `+0x34..+0x60`, then `fn_80006fa0` |
| `80007010` | 528 | GX setup plus camera/matrix arithmetic (8 paired-single ops) |
| `800073c0` | 8 | returns the SDA address `0x8042ADC0` |
| `800077a0` | 188 | scene attachment; stores a pointer at `+8`, state `6`/`7` |
| `80007860` | 860 | GX render setup plus a draw loop (8 paired-single ops) |
| `80007bc0` | 544 | scene attachment plus a 200-entry vertex-grid clear |
| `800086c0` | 936 | GX setup plus a 150-quad emission loop (14 paired-single ops) |
| `80008a70` | 476 | scene attachment plus a 500-entry vertex-grid clear |
| `80008c50` | 1,480 | GX render loop (26 paired-single ops) |
| `80009220` | 476 | 75-entry vertex-grid clear variant of `80008A70` |
| `80009400` | 1,488 | GX render loop (26 paired-single ops) |
| `80009bd0` | 1,252 | GX setup plus draw loop (28 paired-single ops) |
| `8000a0c0` | 780 | GX setup plus draw loop |
| `8000a3d0` | 1,692 | GX setup plus draw loop (30 paired-single ops) |
| `8000ab50` | 760 | GX setup plus four-quad emission |
| `8000afa0` | 884 | GX setup plus a 20-iteration quad loop (8 paired-single ops) |
| `8000b530` | 1,340 | GX setup plus effect loop |
| `8000ba70` | 1,440 | GX effect loop; body ends at `0x8000C010` |

The twelve scalar entries (`800067E0`, `80006930`, `80006A70`, `80006D90`,
`80006EB0`, `800077A0`, `80007BC0`, `80008A70`, `80009220`, `8000A0C0`,
`8000AB50`, `8000B530`) have **no** paired-single instructions and are the
best first targets. The remaining entries need the same GQR treatment as
`80007DE0`.

The strongest layout evidence is in `80007BC0` and `80008A70`: their clear
loops tile the object with zero gaps.

* `80007BC0` clears 200 entries at `+0x78` (1 byte), `+0x140` (1 float),
  `+0x460` (1 float) and `+0x780` (3 floats), then writes `1.0f` to
  `+0x68`/`+0x6C` and zero to `+0x1270`.
* `80008A70` clears 500 entries at `+0x6C` (2 floats), `+0x100C` (1 word),
  `+0x17DC` (4 bytes), `+0x1FAC` (4 bytes), `+0x277C` (1 word),
  `+0x2F4C` (1 float), `+0x371C` (1 float), `+0x3EEC` (3 floats) and
  `+0x565C` (3 floats), plus two `u16` arrays at `+0x6DCC` and `+0x71B4`,
  then stores the count `500` at `+8`.

## Cross-lane callees used by the new bodies

| callee | used by | observed interface |
|---|---|---|
| `0x80163280` | `fn_800067a0` | `()`: runs frame callbacks, drains `0x80377AB0` |
| `0x80163850` | `fn_80006ea0` | `(node)`: sorted insert by the byte at `node + 0x0C` |
| `0x80164FD0` | `fn_80007510` | `(owner, key, 0) -> 0/1` validity check |
| `fn_80006a20` | `fn_80007230` | `(member, a, b) -> u32` virtual slot `+0x0C` |
| `fn_80006b60` | `fn_80006be0` | `(this, arg, 0) -> u32` resource-create path |
| `fn_80006f70` / `fn_80006fa0` | Appendix A only | named here so the future lane sees the local helpers |

## Verification

* `cargo check` (with the workspace toolchain at
  `C:\Users\WitherULTRA7\.cargo\bin\cargo.exe`) reports **no errors and no
  warnings in `src/render/geometry.rs`**; the 11 warnings are pre-existing in
  other lanes' files.
* `node tools/doc-lint.mjs src` reports
  `{"ok": true, "files": 68, "violations": 0}`.
* Every new body's raw words were decoded and compared with its Ghidra C:
  * `80006790` = `4E800020`;
  * `800067A0` = `4815CAE0` → `b 0x80163280`;
  * `80006A40` = `3803000C 90030004 4E800020`;
  * `80006BE0` = `38A00000 4BFFFF7C` → `li r5,0; b 0x80006B60`;
  * `80006EA0` = `4815C9B0` → `b 0x80163850`;
  * `80007220` = `80630020 4E800020`;
  * `80007230` = the 49-word listing at `0x80007230..0x800072F0`;
  * `80007510` = `4815DAC0` → `b 0x80164FD0`.
* No ledger row exists without a body: all eight rows in
  `ledger/agents/wave25-geometry.csv` correspond to a `pub unsafe fn` in
  `src/render/geometry.rs`.

## Uncertainties and limitations

1. **Class identities.** The objects touched by `80007230` have no RTTI names
   in the image; the Rust keeps `fn_<address>` names and documents the field
   offsets instead of inventing classes.
2. **`80007230` arguments.** `param_2` is forwarded unchanged to
   `fn_80006a20`; its role is `// unknown:`. The same applies to `param_3`,
   which may be a pointer or a handle, although `candidate + 0x0C` is used as
   an address in the second virtual call.
3. **Tail-branch wrappers.** The three 4-byte thunks do not carry their own
   control flow. Their Rust forwarders call the target by absolute address;
   the ledger marks them `translated` because the entry's observable behavior
   is exactly one call with the incoming registers forwarded.
4. **`80006790`.** A bare `blr` has no recoverable purpose. It may be an
   unused virtual slot, a deleted constructor or alignment padding that
   Ghidra promoted to a function. The Rust reproduces the only behavior it
   has: return immediately.
5. **`80006BE0` return type.** Ghidra prints `void`; the target's own export
   returns `u32` in `r3`, and the tail branch forwards it, so the Rust keeps
   the `u32`.
6. **Unindexed code is not translated.** Appendix A is a map, not a claim of
   coverage. Its sizes and roles were read from raw instructions only. The
   scalar twelve are the recommended next target; the paired-single entries
   need the same dedicated pass as `80007DE0`.
7. **No runtime validation.** `cargo check` proves the module type-checks; it
   does not execute the PowerPC code.
