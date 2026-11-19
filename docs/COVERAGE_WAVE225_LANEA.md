# Wave 225, lane A - `0x800AA850`, `0x800AA920` (`game::stage`)

The last two `game::stage` bodies: two byte-identical effect submits from the
actor's `+0xAF0` effect-handle set. Both are `translated`, both `high`
confidence, nothing deferred.

Block file: `work/wave225/laneA_block.rs`
Ledger: `ledger/agents/wave225-laneA.csv`

**Module completion.** These two entries are the final `game::stage` functions
of this wave; the module reaches completion when the remaining wave-225 lanes
merge the other ten `main.dol` bodies. No `game::stage` entry stays open after
that merge.

## Bodies

| address | size (tsv) | real extent | name | confidence | role |
|---|---|---|---|---|---|
| `0x800AA850` | 92 | `0x800AA850..0x800AA8AC` (92) | [`fn_800aa850`] | high | emit handle `*(*(this+0xAF0)+4)` at `{+0xAC; +0xB0; 5500.0f}` |
| `0x800AA920` | 92 | `0x800AA920..0x800AA97C` (92) | [`fn_800aa920`] | high | byte-identical twin, two vtable slots later |

Neither size is a prologue artefact:

- `0x800AA850`: `stwu r1,-0x20` at the entry, `bclr 20,0` at `0x800AA8A8`
  (entry + `0x58`), extent 92 exactly. The four zero bytes at `0x800AA8AC` are
  alignment padding before the next body at `0x800AA8B0`.
- `0x800AA920`: same frame, `bclr 20,0` at `0x800AA978` (entry + `0x58`),
  extent 92 exactly. The four zero bytes at `0x800AA97C` are padding before
  `0x800AA980`.

## Byte weight

92 + 92 = **184 bytes**.

## Evidence notes

- Both bodies were disassembled from `work/dol/main.elf`, not just read from
  the export, because Ghidra prints only `unaff_r2 + -0x6c44` for the third
  position word and drops the array layout. The raw sequence is identical in
  both: `lwz r8,0xaf0(r3)`; `lwz r0,0x4(r8)`; `cmpi`/`beq` guard; three
  `stfs` to `0x8`/`0xC`/`0x10(r1)`; `li r4,0`/`li r6,0`/`li r7,0`;
  `bl 0x8016C890`.
- `r2 - 0x6C44` -> `0x8042C43C`; the image word is `0x45ABE000` = `5500.0f`
  (verified by mapping the ELF segment `0x8042B080`/file `0x350200`). The
  existing `W224B_F_5500` constant already names that slot, so no new constant
  was added. `0x8016C890` is the same effect submitter the module already
  calls as a literal `0x8016_C890` ([`fn_800954d0`] and others), so it stays a
  literal too.
- The `+0xAF0` pointer is a set of resolved effect handles, observed from the
  cluster: this lane reads slot `+0x04`, `0x800AA8B0` (vtable slot 22, between
  the two bodies) reads `+0x1C`, and `0x800AADE0` reads `+0x20`/`+0x24`. Both
  functions live in the `0x80314534` vtable at slots 21 and 23.
- The third position word is *not* zero: `W224_F_ZERO_6C44_A224`
  (`SDA2_BASE - 0x6C44`) carries the same address but its doc block claims
  `0.0f`, while the slot holds `5500.0f`. The body of [`fn_800aa7b0`] is
  unaffected (same address), but that doc block is worth correcting in a later
  docs pass. This lane deliberately uses `W224B_F_5500` to avoid the stale
  name.

## Scratch verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to a temp directory, the
block was appended to the copy's `src/game/stage.rs`, and `cargo check --lib`
there finished with exit code 0 (26 pre-existing warnings elsewhere in the
crate, none from `src/game/stage.rs`).
