# Wave 173, lane A (`game::bases`, REL `d_basesNP`)

Four bodies, all re-derived from the raw ELF words plus the Ghidra exports.
Evidence root: `work/ghidra/rel-d_basesNP/export/`.

| Address | Size | Rust body | Description |
| --- | --- | --- | --- |
| `0x80638960` | 352 | `fn_80638960` | Four-panel score-screen tick: posts cue `154`, then for each of the four `0x3C`-strided records (index `+0x7E0`, bound `+0x7E8`, offset `+0x7EC`) resolves the panel through the pointer array at `this + index*4 + 0x6EC`, adds the offset to the panel's `+0x30` and pins the result to the bound while zeroing the offset when the sum crosses it (positive arm `> bound`, negative arm `< bound`); `+0x2C`/`+0x34` are rewritten unchanged. When all four offsets truncate to `0`, dispatches `+0x6A8` virtual slot `+0x18` with `0x80826D60`. |
| `0x80753B80` | 352 | `fn_80753b80` | `daWaterMove_c` destructor: destroys the arrays at `+0xD730`/`+0xD6E0` (`4` x `0x14`) and `+0xD578` (`0xC` x `0x14`), the sub-objects at `+0xD3F0` and `+0x554`, the effect holders `+0x530`/`+0x514`, the motion holders `+0x49C`/`+0x470`, the animation holders `+0x430`/`+0x3F0` and the accessor `+0x3D0`, runs `0x800671B0(this, 0)`, frees through `0x80162920` when `free > 0`, and returns `this`. |
| `0x8076EAA0` | 352 | `fn_8076eaa0` | Ship-actor frame update dispatched on `*(this+4) & 0xF`: kinds `0`/`2` run class vtable `+0x60` slot `+0x60` with `(this, 0x801015E0([0x8042A1AC]), manager[0x164])` then `0x807702A0`; kind `1` runs `0x8076F970`, bumps `+0x39C` when `+0x3A0` is set and calls `0x8076FB00(this, -1)`; kinds `3`/`4` run `0x8076F970` and add `+0x4BC` to `+0xB0`; kinds above `4` do nothing. Then `0x80770C50` when `+0x124` is set, virtual slots `+0x14` of `+0x410`/`+0x3D8`, slot `+0x1C` of `+0x1A0`, `0x8076EE60`, and returns `1`. |
| `0x8079AE00` | 352 | `fn_8079ae00` | Course-route weight scan: probes the node at `+0x22C` with `0x800F57A0(route, node, 0x100)`, walks the sibling chain with `0x80799A50` while `0x800F9970` reports type `1`, stops at the alternate id from `0x80799B00` or a negative successor, re-probes a node whose type is not `1` when that node ends the walk, and writes `+0x298` = the `main.dol` float at `0x8031D5B8` on success, `1.0` otherwise. |

## Deferred

None. All four bodies were fully recovered; the two readings that are not
provable from this REL alone are recorded as `observed:`/`inferred:` notes below
instead of blocking the bodies.

## Verification notes

- **`0x80638960` clamp constant.** `lis r3,0x807c` / `lfs f1,0x7eb8(r3)` reads
  `0x807C7EB8` = `0x00000000`, i.e. `0.0`. The two clamp arms use opposite
  senses: at `0x806389D8` `bc 4,0` is *branch if not less* (offset `>= 0.0`, the
  fall-through is the negative arm) and the inner `bc 4,0` at `0x806389E0`
  skips when `sum < bound`; the negative arm's `bc 4,1` at `0x806389F8` is
  *branch if not greater*, i.e. it skips when `sum > bound`. Both arms store
  `0.0` to `+0x7EC` and the bound to the panel's `+0x30`.
- **`0x80638960` settle test.** The four checks at `0x80638A2C..0x80638A8C` are
  `lfs` + `fctiwz` + `stfd` + `lwz` + `cmpi 0`, i.e. the float truncated to a
  signed integer, not a bit compare, so any offset with `|off| < 1` counts as
  settled. The body reproduces this with `f32 as i32`.
- **`0x80638960` panel array.** `lwz r0,0x7e0(r6)` / `rlwinm r0,r0,2,0,29` /
  `add r5,r31,r0` / `lwz r4,0x6ec(r5)` resolves the panel as
  `*(this + index*4 + 0x6EC)`, the same owner-relative pointer idiom as
  `0x80637050` (`screen + index*4` then a `+0x70C`-relative word). The fourth
  check reads `0x828(r3)` with `r3 = r31 + 0x78`, i.e. `this + 0x8A0`.
- **`0x80638960` descriptor.** `lis r4,0x8082` / `addi r4,r4,0x6D60` = the
  `0x80826D60` descriptor that [`fn_806393f0`] registers as
  `dPointResultDateFile_c::StateID_BigSmallAnimeEndWait`; the dispatch is the
  same `vmethod1(this + 0x6A8, 0x18, desc)` shape as [`fn_80638120`]. The
  class attribution is `// inferred:` from that descriptor, since the constructor
  [`fn_8063a120`] sits at a higher address.
- **`0x80753B80` arguments.** The `0x802DCD74` `_savegpr_27` prologue is a
  compiler helper; `mr r27,r3` and `mr r28,r4` follow it, so the body takes
  `(this, free)`. Ghidra's 64-bit return puts `this` in the high half and the
  `free` test on the low half, but the raw `cmpi ... r28` at `0x80753CB4` uses
  `r4`, and `bc 4,1` (branch if not greater) skips the free for `free <= 0`.
- **`0x80753B80` guards.** The `addic.`/`bc 12,2` pairs at `0x80753BA4`,
  `0x80753BB0`, `0x80753BFC`, `0x80753C34` and `0x80753C64` test
  `this + K != 0` for `K` in `0x514`/`0xD3F0`/`0x554`/`0x3D0`, which can only
  fail for a negative `this`; they are emitted as unconditional, matching the
  precedent of [`fn_806b3570`] and [`fn_806b35f0`].
- **`0x80753B80` array descriptors.** `0x802DCA98` takes
  `(array, element destructor, element size, count)` and is called with
  `(this+0xD730, 0x80646C90, 4, 0x14)`, `(this+0xD6E0, 0x80646C90, 4, 0x14)` and
  `(this+0xD578, 0x80601CD0, 0xC, 0x14)`; the constructor [`fn_807531d0`] builds
  the same three arrays at `+0xD730`, `+0xD6E0` and `+0xD578` with `0x802DC9A0`
  and identical sizes/counts, which fixes the argument order. `+0x59C` is the
  `0x1130`-element array, freed through `0x8016A030(this+0x554, 0)`.
- **`0x8076EAA0` dispatch.** `cmpi r0,3` / `bc 12,0` (blt) sends kinds `0..2` to
  the `0`/`1`/`2` dispatch at `0x8076EAD8`; `cmpi r0,4` / `bc 4,1` (ble) sends
  kinds `3` and `4` to the moving arm at `0x8076EB64`; kinds above `4` fall into
  the dispatch and match neither `0`/`2` nor `1`, so `b 0x8076EB8C` skips every
  arm. This is Ghidra's `if (kind < 3 || 4 < kind) { ... } else { ... }` and the
  `bc 4,1` must not be read as `bgt`.
- **`0x8076EAA0` virtual dispatch.** `lwz r12,0x60(r30)` / `lwz r12,0x60(r12)` /
  `bcctr` with `r3 = this`, `r4 = 0x801015E0` result and `r5 = manager[0x164]`;
  identical to [`fn_8075ea40`], which is why it is emitted inline instead of
  through `vmethod*` (there is no two-argument helper).
- **`0x8076EAA0` owner hand-off.** `0x80770C50` recomputes its receiver from the
  `r3` its own `_savegpr_27` prologue saves ([`fn_80770c50`] shows the
  `call0_ret(0x802DCD74)` trick), so the call is emitted as
  `abi_call1!(0x8077_0C50, this)` to keep `this` in `r3`; calling the
  no-argument Rust body directly would leave `r3` indeterminate.
- **`0x8079AE00` weight constants.** `lis r3,0x8032` / `addi r3,r3,-0x2C64` is
  `0x8031D39C` and the load is `lfs r0,0x21C(r3)`, i.e. `0x8031D5B8`, which lies
  in `main.dol` (not in this REL's segments), so the body reads it at run time
  rather than folding a value. The failure branch is `0x807D00C4` = `0x3F800000`
  = `1.0`.
- **`0x8079AE00` loop rotation.** `r24 = r26` before the recursive
  `0x80799A50(this, previous, next, key)` and `r25 = r24` after it, so the
  "dropped" argument of [`fn_80799a50`] is the previous node, seeded with
  `+0x22C`. `r27` holds the success flag, seeded `0` and set to
  `0x800F57A0(route, next, 0x100) != 0` only on the type-`!=1` exit.
- **`0x8079AE00` route base.** The raw words reload `[scene+0x338C]` and
  recompute `scene + index*0x16518 + 0x3394` before each of the three uses; the
  scene and its index are stable in this body, so one binding is equivalent.
