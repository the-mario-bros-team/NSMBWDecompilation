# Coverage - Wave 204 lane F (`ui::hud`, main.dol)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/hud.rs` | `ui::hud` | 10 | 800 | 10 | 0 |

Addresses `0x8002A840`-`0x8002AC40`, in the `0x8002A7xx`-`0x8002ADxx` run of
`dBase`-shaped helpers that the earlier lanes left unclaimed.

## What landed

- **Record accessors.** `0x8002A840` and `0x8002A850` are the two 12-byte
  `lis`/`addi`/`blr` bodies of this lane, returning `0x8035328C` and
  `0x8035324C`.
- **Child-chain search.** `0x8002A940` (224 B) walks the `+0x30` link and
  probes the child's virtual `+0x0C` slot.
- **Name compare.** `0x8002AA20` (136 B) splits both names at `':'` and
  compares the tails through `0x802E1AB4`.
- **Interface forwards.** `0x8002AAE0` and `0x8002AB10` are the `(b, b, a)`
  `0x802DCBFC` thunks with the embedded interface at `a + 0x18` and `a + 0x24`.
- **Actor lookups.** `0x8002AB40` and `0x8002ABA0` scan the profile lists
  `0x1A3` and `0x1A4` for a byte match at `+0x39C` / `+0x3E2`;
  `0x8002AC00` is their fallback wrapper.
- **Position snapshot.** `0x8002AC40` (80 B) copies the reference record word
  and stores `this.pos - source.pos`.

## Corrections and recoveries

- **The 12-byte accessors do not return `0x80377A04`.** Both bodies are
  `lis r3,0x8035` plus `addi r3,r3,0x328C` / `0x324C`, i.e. two *distinct*
  addresses. Checked against the disassembly and against the Ghidra export;
  both words sit in BSS (`0x80351680..0x8042FC3C`) and are zero at boot, unlike
  the shared `0x80377A04` word the existing `fn_8001cbb0` family returns.
- **The accessors are one table, not six copies.** `0x8002A800`..`0x8002A850`
  return `0x8035334C`, `0x8035338C`, `0x8035330C`, `0x803532CC`, `0x8035328C`
  and `0x8035324C`; sorted, these are exactly `0x8035324C + 0x40 * n` for
  `n = 0..5`. Documented as a six-entry `0x40`-byte BSS table.
- **Byte-identical clones exist.** `0x8002C870` and `0x8002C880` return
  `0x8035328C` and `0x8035324C` with the same twelve bytes; left for whichever
  lane claims the `0x8002C8xx` run.
- **`0x8002A940` returns the parent.** The virtual `+0x0C` probe runs on the
  *child* (`lwz r3,0x30(r31)` then `lwz r12,0(r3)`), but the non-zero arm is
  `mr r3,r31`, i.e. the node whose child answered, and the first-link failure
  arm jumps to `0x8002AA08` with `r31 == node`, returning the argument itself.
- **`0x8002A940` is unbounded recursion, modelled as a loop.** The shipped body
  unrolls five probes and then does `bl 0x8002a940` on the next link (a tail
  call the compiler did not turn into `b`). The reconstruction keeps the walk
  as a `loop` so the host does not grow the stack per link.
- **`0x8002AA20` advances its two tails asymmetrically.** The argument is
  advanced only when `0x802E1C40` found a `':'` (`beq` around
  `addi r31,r3,1`), while the current name's result is advanced
  unconditionally (`addi r3,r3,1` at `0x8002AA74`). Preserved as-is; the body
  is otherwise word-for-word `fn_80023b40`.
- **`0x8002AC40` is float code, not integer code.** Ghidra prints
  `(float)param_2[0x2d]` and integer casts; the disassembly is three
  `lfs` / `fsubs` / `stfs` triples. The reconstruction uses `read_f32` /
  `write_f32` throughout. The `+0x5BC` store really is a word copy
  (`lwz r0,0x0(r4)` / `stw r0,0x5bc(r3)`).
- **`0x8002AC00`'s Ghidra signature is wrong.** It prints `FUN_8002ab40()` with
  no argument; `r3` still holds the incoming parameter at `0x8002AC14`, so the
  key is passed through to both lookups.
- **`0x80162D50`'s cursor `0` is a sentinel.** The iterator shifts a non-null
  previous link by `+0x10` before walking `0x80163090(&DAT_80377A10, ...)`, so
  a starting value of `0` means "restart at the head", not "record zero".

## Deferrals

None. All ten bodies are fully reconstructible from shipped code; no constant
pinning was required. The `r12` interface selectors of `0x8002AAE0` and
`0x8002AB10` cannot be expressed by `platform::call3` and are carried as the
third operand, matching every other `0x802DCBFC` call site in `ui::hud`.

## Verification

- Temp-copy `cargo check --lib` exit 0 (23 pre-existing warnings, none from
  `src/ui/hud.rs`).
- `doc-lint` on the temp tree: 0 violations.
- `check-lane-csv.mjs wave204`: 10 rows, 10 unique addresses, 10 high, `OK`
  including the `# Original` byte-size cross-check against `functions.tsv`.
- No files outside `work/wave204/laneF_block.rs`,
  `ledger/agents/wave204-lanef.csv` and this document were written;
  `src/ui/hud.rs` is untouched.
