# Wave 173, lane E (`game::bases`, REL `d_basesNP`)

Three bodies, all re-derived from the raw ELF words plus the Ghidra exports.
Evidence root: `work/ghidra/rel-d_basesNP/export/`.

| Address | Size | Rust body | Description |
| --- | --- | --- | --- |
| `0x806D2390` | 364 | `fn_806d2390` | `remo_lift` see-saw rider sync: runs `0x800441E0(this, &out)` for the four-slot presence mask, releases the seat (`+0x430 = 0xFF`) when the mask is empty or the rider's slot bit from `0x807CB540` is clear, re-seats the driver's cached byte when `+0x430 < 0` (clearing `+0x556`, re-centring `+0x54C` through `0x806D1ED0`, raising `+0x53C`, posting cue `0x2A6` and spawning the seat effect `0x806D21A0`), sets `+0x52C` when the cached byte matches, then runs `0x806D22B0` and, on a clear `*(0x80429D48)+0xBCB`, `0x806D1EF0(this, mask)`. |
| `0x80742470` | 364 | `fn_80742470` | Four-slot marker-actor spawner: row `0x807422D0(this)` and mask `0x80742340(this)` once, then per slot `0x807422F0(this)` for the table word, a `mode`-selected pair of float offsets from `0x807CE100 + 16*row + {0x10,0x50} / {0x90,0xD0} / {0x110,0x150}`, and `0x807423E0(this, &pos, flags, variant, slot)` with `flags = ((+0x348 & 1) << 31) | 0x40000000 | (slot << 4) | ((word & 0xFFF) << 12)` and `variant = mask & (1 << slot)`. |
| `0x8076FF80` | 364 | `fn_8076ff80` | `cobKoopaShip` child transform: scene lookup `0x80100530(*(0x8042A18C), "cobKoopaShip", 0)`; on no record the `+0x480..+0x48C` template comes from `0x80377DC8..+8` and `0x807CF2A4`; otherwise the record name at `record + *(record+8)` loses its prefix, the suffix is copied with `0x802B9250`/`0x802E19F8`, parsed with `0x802E2108`, converted unsigned-int-to-double and stored at `+0x48C`, `0x801006E0` writes `+0x480`, the `0x80816C8C/90/94` nudges are added, `+0x490` is raised and the buffer freed with `0x802B9290`. |

## Deferred

None. All three bodies were fully recovered; the two unresolved readings are
recorded as `observed:`/`inferred:` notes below rather than blocking the bodies.

## Verification notes

- **`0x806D2390` table.** `lis r3,0x807D` / `addi r3,r3,-0x4AC0` = `0x807CB540`,
  and the access is `lbzx r0,r3,r4` with `r4` = `extsb` of `+0x430`. The bytes
  are `01 02 04 08 10 20 40 80`, i.e. the slot-bit table, so the body loads the
  byte rather than hard-coding `1 << rider`.
- **`0x806D2390` second `0x806D21D0` call.** Ghidra prints one argument because
  it does not track `r4`, but the raw words show no write to `r4` between the
  `extsb` at `0x806D23F4` and the `bl` at `0x806D2410`, so the sign-extended
  `+0x430` value is passed. Both calls therefore use `fn_806d21d0(this, rider)`.
- **`0x806D2390` branch senses.** `bc 4,2` (bne) at `0x806D23C0` skips the
  `mask == 0` block; `bc 4,0` (bge) at `0x806D2428` skips the re-seat block, so
  the test is the signed `+0x430 < 0`. The re-seat block re-loads `+0x430` after
  the possible `0xFF` store, and the trailing comparison is `cmp` (signed).
- **`0x806D2390` stack buffer.** `addi r4,r1,8` + `lbz 0x8(r1)` is the `out`
  byte of `0x800441E0` (which stores exactly one byte there); modelled as
  `[u32; 2]` so the driver's dword stores stay in bounds.
- **`0x80742470` arguments.** The body opens with `0x802DCD58` (a `_savegpr`
  stub that only stores registers), and both `mr r21,r3` / `mr r22,r4` follow
  it, so `r3`/`r4` are the entry arguments. Ghidra models the stub as a 64-bit
  return: its `iVar1` is `r3` (the object) and `(int)uVar12` is `r4` (the mode),
  which is the reverse of how the C reads. The caller `0x80742200` decodes
  `*(marker+4)` nibbles (`bits 28..31` -> `+0x3E0`, `24..27` -> `+0x3E4`,
  `20..23` -> `+0x3E8`, `16..19` -> `+0x3F0`, bit 30 -> `+0x348`) and calls this
  body at `0x80742240` with `r4` = that low nibble, still holding the mode.
- **`0x80742470` tables.** `lis r20,0x807D` / `addi r20,r20,-0x7F00` =
  `0x807CE100`; the six pointers are `+0x10/+0x50/+0x90/+0xD0/+0x110/+0x150`
  with a `16 * row` stride (`rlwinm r23,r3,2` then `rlwinm r9,r23,2`) and a
  four-byte per-slot step. Re-read data confirms row `0` slot `0` gives
  `16/-8/8/-8/8/-8`, so `mode 0` = `+0x10`/`+0x50`, `mode 1` =
  `+0x90`/`+0xD0`, else `+0x110`/`+0x150`. Each block is `0x40` bytes, i.e.
  four rows, matching the `0..3` range of `+0x3E8` and of `0x800B2EE0(4)`.
- **`0x80742470` flags.** `rlwinm r0,r0,31,0,0` operates on the GPR loaded by
  `lbz r0,0x348(r21)` (not on `f0`) and keeps `bit 30` of the nibble, giving
  `(+0x348 & 1) << 31`; `ori`/`oris` supplies `0x4000_0000`, and `rlwinm
  r3,r3,12,0,19` gives `(word & 0xFFF) << 12`. The `and` + `rlwinm ...,0,24,31`
  pair yields `(mask & (1 << slot)) & 0xFF`.
- **`0x8076FF80` argument.** `0x802DCD70` is the save stub and `mr r31,r3`
  follows it; the only in-REL caller (`0x8076EE24`) passes `r3` only (`mr r3,r24`)
  and leaves `r4` holding an unrelated pointer, so the body takes `this` alone.
- **`0x8076FF80` name pointer.** `lwz r4,0x6ba0(r4)` loads the *word* at
  `0x80816BA0` (which holds `0x80816B90`, the `"cobKoopaShip"` string); Ghidra's
  `PTR_s_cobKoopaShip_80816ba0` spelling denotes the pointer variable, but the
  instruction is a load, so the body re-reads `platform::read_u32(0x8081_6BA0)`
  before the lookup and again before the two length scans.
- **`0x8076FF80` conversion.** `xoris r3,r3,0x8000` + `lis r0,0x4330` +
  `lfd 0xfffff318(r4)` = the magic `0x43300000` double at `0x807CF318`; the body
  reproduces `f64::from_bits(0x4330_0000_0000_0000 | (value ^ 0x8000_0000))`
  minus `read_f64(0x807C_F318)`, then narrows to `f32`. The three `fadds` at
  `0x8077007C/88/94` read `0x80816BE8 + 0xA4/0xA8/0xAC` (= `0x80816C8C/90/94`,
  all `0.0` in this build) and the `stb r0,0x490` precedes the `stfs` to
  `+0x488`, which the body preserves.
- **`0x8076FF80` record text.** `lwz r0,0x8(r3)` then `add r28,r3,r0` (or
  `r28 = 0` when the word is zero) is a self-relative name offset;
  `subf r29,r3,r29` makes the copied span `strlen(text) - strlen(prefix)` and
  the source `text + strlen(prefix)`. `// inferred:` the parsed number is the
  ship variant id, which the body stores at `+0x48C` before `0x801006E0`
  rewrites the position.
