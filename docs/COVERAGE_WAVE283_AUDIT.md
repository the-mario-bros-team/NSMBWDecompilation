# Audit - Wave 283 (four tables closed, ABI defect found)

## 1. Tables closed

| Table | Slots | Identity |
|---|---:|---|
| `0x80301EB0` | **185/185** | EN59 sub-object (sibling of `0x80302D80`) |
| `0x80302D80` | **185/185** | EN59 sub-object |
| `0x8034EA70` | **23/23** | derived window-stream vtable over a DVD descriptor |
| `0x8034CCA8` | **23/23** | derived streamed-texture-palette range object |
| `0x8034CDAC` | **21/21** | palette/font device record |

**Sixty-two vtables complete.**

## 2. Table relationships

- The EN59 pair are **181/185 identical** (differing in slots 16, 133, 141,
  142); a 19-slot shared vertical-rate accessor run (slots 166-184) closed both.
- `0x8034EA70` is a **derived** table: 18 of 23 words are byte-identical to
  `0x8034CCB0`, with slots 1, 3, 14, 15, 20 the window-aware overrides. Its
  class is a 120-byte synchronised window-stream over a `DVDFileInfo`
  (`0x801CA990`/`0x801CAB20`/`0x801CAA00`), sharing an `OSMutex` at
  `0x803DA0C0` and an `OSThreadQueue` at `0x8042AC30`. No RTTI words exist, so
  no C++ name was invented.
- `0x8034CCA8` is a transient derived table: the `0x8034EA68` class overwrites
  it at `+0` immediately after construction.

## 3. ABI defect in an existing body

`fn_80096240` read its point from **`r3`**, but the image reads it from **`r4`**
(`lfs r1,0x4(r4)` / `lfs r0,0x0(r4)`) while `r3` is overwritten with the probe
output buffer. All three Rust callers passed the point in `r3`, so the probe
would have read the wrong address. Fixed to `(this, point)` with all three
callers updated.

A second instance of the cue-gate class was also fixed:
`wave104_spawn_player_se` had **swapped parameters** (`r3` is the table -
`lwz r0,0x54(r25)` - and `r4` the cue id - `mr r4,r26` into `0x80275760`) and
passed `0` to the `0x8019DC90` gate. Corrected to `(table, id)` with the gate
taking `id`, and both callers swapped.

## 4. `_savegpr`/`_restgpr` sweep, part 8

| Metric | Before | After |
|---|---:|---:|
| Modelled helper calls | 329 | **286** |
| Bodies containing them | 174 | **149** |
| Bodies changed | - | 25 |
| Calls removed | - | **47** |
| Constants deleted | - | 11 |

All 25 sites verified at prologue/epilogue positions in both `main.elf` and
`d_basesNP.elf`. The lane re-checked **every** live register at each rebound
site (the requested `wave99_start_stream` class check): no wrong incoming value
survived, and three pre-existing `r3:r4` pair splits were confirmed correct
(`CONCAT44` places `r3` in the high half).

## 5. Reported for follow-up

Three argument defects found by the audit in older `seq.rs` bodies:

1. `fn_8027b360` passes `first` where the image passes `second` to
   `0x80264C10`, `0x80282FF0`, `0x80267D30`; `second` is computed and unused.
2. `fn_8027ddd0` - `0x802830A0` should get `[base+0xC]` not `[base+4]`, the
   callback object needs `+0xB58`, and `0x802810A0` should get the slot word, not
   the element address.
3. `fn_80274ce0` - `0x80274320`'s 4th argument is `r3 + 4`, not `r3`.

Queued as their own lane; each needs the full register read, not a one-line edit.

## 6. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | EN59 sibling close | 20 | 1,232 |
| B | `0x8034ea70` close | 5 (+18 shared) | 280 |
| C | savegpr sweep part 8 | 25 fixed | - |
| D | two range-object tables close | 41 | ~2,000 |
| E | gap batch 13 | 12 | ~1,080 |
| F | audit wave 282 | 0 (doc fixes) | - |

## Verification

- `cargo check` exit 0 (30 warnings); `doc-lint` 0 violations.
- `check-dupe-bodies` exit 0; `check-page-offsets` 41 pairs / 13,755 tokens,
  0 mismatches.
- `scripts/check.ps1` exit 0.
