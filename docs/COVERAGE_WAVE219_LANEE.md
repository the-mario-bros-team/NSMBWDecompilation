# Wave 219, lane E - `src/resource/archive.rs`, addresses `0x802C0A80..0x802C378C`

Ten entries of `work/ghidra/main-gb/export/functions.tsv` (rows 9532-9577) in
the `0x802C0000` band of the `resource::archive` ledger module. All ten are
translated; nothing was deferred.

Block file: `work/wave219/laneE_block.rs`
Ledger: `ledger/agents/wave219-laneE.csv`

| address | tsv size | real extent | name | confidence | role |
|---|---|---|---|---|---|
| `0x802C0A80` | 64 | `0x802C0A80..0x802C0AC0` (64) | [`fn_802c0a80`] | high | construct the 0x18-byte display object |
| `0x802C0AC0` | 112 | `0x802C0AC0..0x802C0B30` (112) | [`fn_802c0ac0`] | high | deleting destructor, conditional block free |
| `0x802C0B30` | 12 | `0x802C0B30..0x802C0B3C` (12) + 4 pad | [`fn_802c0b30`] | high | mark the object active (`+0x08 = 1`) |
| `0x802C0C20` | 76 | `0x802C0C20..0x802C0C6C` (76) + 4 pad | [`fn_802c0c20`] | high | allocate a texture from the object's fields and flush it |
| `0x802C1D10` | 612 | `0x802C1D10..0x802C1F74` (612) + 12 pad | [`fn_802c1d10`] | medium | one-shot render setup, two mode pushes, layer draw |
| `0x802C1F80` | 216 | `0x802C1F80..0x802C2058` (216) + 8 pad | [`fn_802c1f80`] | high | configure the two direct GX channels |
| `0x802C2060` | 124 | `0x802C2060..0x802C20DC` (124) + 4 pad | [`fn_802c2060`] | high | select one of two window/layer rectangles |
| `0x802C2360` | 920 | `0x802C2360..0x802C26F8` (920) + 8 pad | [`fn_802c2360`] | high | fourteen-arm GX channel configuration |
| `0x802C3020` | 36 | `0x802C3020..0x802C3044` (36) + 8 pad | [`fn_802c3020`] | high | tail forward of one 16-byte table record |
| `0x802C3630` | 348 | `0x802C3630..0x802C378C` (348) + 4 pad | [`fn_802c3630`] | medium | construct the 0x160-byte display object |

Total: 2520 bytes, 10 bodies, 0 deferrals.

## Extent checks

Every `functions.tsv` size in this lane is exact; **none** is a prologue
artefact. Each was re-checked in `work/dol/main.elf` from the prologue down to
the closing `bclr`:

* `0x802C0A80`, `0x802C0AC0`, `0x802C0B30`, `0x802C0C20`, `0x802C1D10`,
  `0x802C1F80`, `0x802C2060`, `0x802C2360` and `0x802C3630` end exactly on
  their `bclr` (`start + tsv - 4`); the words between that `bclr` and the next
  entry are alignment zero fill the tsv does not count (4 bytes for
  `0x802C0B30`/`0x802C0C20`/`0x802C2060`/`0x802C3630`, 8 for
  `0x802C1F80`/`0x802C2360`, 12 for `0x802C1D10`).
* `0x802C3020` has no `bclr` at all: its last instruction is
  `4bf05d80 b 0x801C8DC0`, a pure tail branch at `0x802C3040`, and the tsv's
  36 bytes cover exactly that branch plus the two alignment words.

## What the run is

`// inferred:` this is the display-object side of the `0x802C0000` band, not
archive reading. The callees are the GX render-state library
(`0x801C37C0`/`0x801C40B0`/`0x801C4780`/`0x801C82F0..0x801C8DC0`), and the two
constructors install the pointer sets at `0x80350110` and
`0x80350230`/`0x8035023C`/`0x8035024C`/`0x80350280`. The band belongs to this
file because its objects are built and released on the same resource/disc path
as the archive heap bodies above, and no other module owns the addresses.

Two small accessors and their constructor:

* `0x802C0A80` stores the vtable, the two halfword dimensions, the format byte,
  two `1` flags and two zero words; `0x802C0AC0` is its deleting destructor
  (null guard, vtable restore, `+0x08 & 4` test freeing `+0x14` through
  `0x802B9290`, optional object free through `0x802B9280`); `0x802C0B30` sets
  the active byte.

`0x802C0C20` allocates a GX texture with
`0x801C6BA0(*u16(+4), *u16(+6), *u8(+9), 0, 1)` and flushes it to the slot at
`+0x10` with `0x801AC470`.

The render setup at `0x802C1D10` is the one body whose Ghidra shape needed
repair:

* the `FUN_802dcd70`/`FUN_802dcdbc` pair are the compiler's save/restore
  register thunks (`bl 0x802dcd70` is followed by `mr r26,r3` from the
  *incoming* `r3`), so no call is emitted for them; `FUN_802dcd70`'s fake
  64-bit return in the export is the `(r3, r4)` pair, and the value the body
  later passes to `0x801C9940` is `param_1`;
* the Ghidra `local_28[10]` block is a ten-word stack slot of which only
  `local_28[0] = *param_5` is initialised; the other nine are frame garbage and
  are modelled as zeros;
* the two `0x802D2F20`/`0x802D2F80` pushes are restored at the end in reverse
  order with both results masked to a byte (`rlwinm ...,0,24,31`), exactly as
  the disassembly shows.

`0x802C1F80` and `0x802C2060` are the two-state helpers it calls: the first
writes the seven-argument channel descriptors through `0x801C6AF0` (the `r7`
argument of the `mode == 0` arm is `0x802C17D0()`), the second reuses one
six-argument rectangle for both layer selectors.

`0x802C2360` is a `bctr` dispatch through the fourteen-word table at
`0x80350160`, dumped from `work/dol/main.elf` as
`0x802C23A8`, `0x802C24D0`, `0x802C2414` x4, `0x802C2460`, `0x802C24F8` x2,
`0x802C25B0`, `0x802C261C` x4. Ids above `0xD` (`cmpli ...,0xd`) fall straight
to the epilogue. The descriptor blocks it copies through `0x801C4780` are the
`0x802FCFA0`-relative records at `0x802FCFC0`/`0x802FD000`/`0x802FD040`/
`0x802FD060` and the `sdata2` records at `r2 - 0x3E00`, `-0x3DE0`, `-0x3DC0`,
`-0x3DA0` and `-0x3D80`; all are passed as addresses (`addi r4,r2,-0x3E00`,
no floating load), with lengths `2`, `4` or `6` words.

`0x802C3020` is a 36-byte tail forwarder into `0x801C8DC0` of the four words at
`0x802FD0A8 + (index << 4)`.

`0x802C3630` constructs the 0x160-byte object: base constructor `0x802C3210`,
the four pointer slots, the 0x1C/0x20 flags, then three heap blocks from
`0x802B9210` (100, 0x78, 0x1C0 bytes) each handed to a member constructor
(`0x802CFBA0`, `0x802CEF70`, `0x802CF6F0`).

## Evidence repairs

Two Ghidra exports in this lane are type-wrong rather than truncated; both were
resolved against the instruction stream, and the `# Original` blocks say so:

1. **`0x802C3630` floats.** The export reads ten `sdata2` words as integers
   (`*(undefined4 *)(unaff_r2 + -0x3d60)` etc.). The disassembly is `lfs`
   (`c102c2a0 lfs f8,-0x3D60(r2)`, ...), and `work/dol/main.elf` at
   `0x8042F31C..0x8042F344` holds `30.0`, `20.0`, `1.6`, `0.0`, `0.4`, `0.2`,
   `-0.0075`, `10.0`, `15.0`, `1.0`. The body stores those floats and documents
   the mapping.
2. **`0x802C1D10` thunks and stack block.** See above.

Constructor-argument notes: the three member constructors of `0x802C3630` take the
allocated block in `r3` plus the register-live context word in `r4`
(`0x802CFBA0(block, 0)`, `0x802CEF70(block, block)`,
`0x802CF6F0(block, obj_b)`); the export leaves those arguments blank. This is
the one inferred call shape in the lane and is the reason `0x802C3630` is
`medium`.

## Not part of this lane

`0x802C20E0`, `0x802C3000`, `0x802C17D0`, `0x802C3210`, `0x802CFBA0`,
`0x802CEF70` and `0x802CF6F0` are called by absolute address and are **not**
translated here; each has a `_E` constant documenting the observed contract.
The `0x802C0000` bodies below `0x802C0A80` and above `0x802C378C` are owned by
other lanes.

## Verification

The block was appended to a scratch copy of `src/`, `Cargo.toml` and
`Cargo.lock` (no source file of the repository was modified) and
`cargo check --lib` there exits 0 with no warnings in
`src/resource/archive.rs`.
