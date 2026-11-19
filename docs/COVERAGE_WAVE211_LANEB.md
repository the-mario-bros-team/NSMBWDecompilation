# Coverage - Wave 211, lane B (`game::player`, the `0x80063F20..0x80064540` cluster)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 10 | 1536 | 9 | 1 | 0 |

Status split: 10 `translated`, 0 `partial`, 0 deferred. Address range covered:
`0x80063F20..0x80064540`.

Every `functions.tsv` size here except two is 4 bytes larger than the real
body, because the aligned entry after each `blr` inherits one pad word; the
real extents are noted per function below. The `# Original` lines and the
`Bytes` column keep the `functions.tsv` values.

## How to read this file

* **Address** - virtual entry point in the DOL image, not a file offset.
* **Name** - the Ghidra name (`FUN_<address>`); the Rust body keeps the
  `fn_<address>` name because no symbol table survives for these functions.
* **Role** - one plain sentence per function. A role is marked "inferred" when
  it comes from shared field offsets and call shapes.
* **Confidence** - strength of the evidence, not code quality: `high` = every
  address, offset, constant and branch is literal; `medium` = a calling
  convention or callee meaning had to be reconstructed.
* SDA constants were re-read from `work/dol/main.elf` with the module's fixed
  `r2 = 0x80433080` and `r13 = 0x8042F680`; every resolved float is quoted at
  its definition.

## What this pass covers

- **The two member-pointer forwarders `0x80063F20` (48 B) and `0x80063F50`
  (48 B).** Both are byte-identical to the wave-210 twins `0x80062090` /
  `0x800620C0` except for the `addi r12` displacement: `r5 = owner`,
  `r3 = r4 = object`, `r12 = owner + 0x18` / `owner + 0x24`, then the
  `0x802DCBFC` thunk (reused here as [`invoke_pmf_d`]).
- **The key initialiser `0x80063F80` (176 B, real 172).** Unpacks four
  sub-fields of the word at `+0x04` with `rlwinm`, widens each with the
  `lis 0x4330` + `lfd` integer-to-single idiom, and stores the step
  (`0.5 * n + 0.5`, scale `0.5f` at `0x8042BAF8`) at `+0x5D4`, the high
  endpoint (`16 * n + 16`, scale `16.0f` at `0x8042BAFC`) at `+0x5D0`, the low
  endpoint at `+0x5CC` and `+0x5D8`, and forwards `(high, bits >> 16 & 3, this,
  index, 1, 0, 0)` to [`fn_80045a10`].
- **The ping-pong driver `0x80064030` (212 B, real 208).** Probes
  `0x80064F50(this, 0)`; on a clear result picks `target - step` or
  `target + step` for `+0x5C8`, flips the `+0x5DC` endpoint index and reloads
  `+0x5D8` from the `+0x5CC`/`+0x5D0` pair, then calls `0x80045D00(value, this)`
  and tail-calls `0x80045B10(this)`.
- **The constructor `0x80064110` (412 B, real 408).** Base-class hook, vtable
  word `0x80309F6C` at `+0x60`, four more scalar stores, the `+0x148` effect
  record, the `+0x1EC` animation object, the `+0x2DC` display context, eight
  `0.0f` size floats at `+0x318..+0x334`, seven scalar clears
  (`+0x34C`, `+0x350`, `+0x368`, `+0x36D`, `+0x380`, `+0x38D`, `+0x390`),
  `0x800646E0(this, 0)`, the `80/256/128/128` and `256/256/80/80` size block
  at `+0x328..+0x344`, the
  `0x3B0`-stride record lookup (`r13 - 0x54B8` header, `r13 - 0x57E8` array,
  `+0x2C == 0` coerced to a null model), the `0x8008E770` attach byte stored at
  `+0x349`, both sub-object back-pointers, the `r13 - 0x5970` byte copied into
  `+0x38F`/`+0x1E8`/`+0x2D8`/`+0x30E`, and two final clears.
- **The destructor `0x800642B0` (148 B, real 144).** Null-gated; re-installs the
  vtable word, `0x8008C380(+0x148)`, `0x800D9880(+0x2DC, -1)`,
  `0x8006CFA0(+0x1EC, -1)`, `0x8008C260(+0x148, -1)`, `0x8006C7F0(this, 0)`,
  and `0x80162920(this)` only when `count > 0`. Returns `this`.
- **The branch thunk `0x80064380` (4 B).** One word, `b 0x8006C8A0`, which is
  in turn `b 0x8006C570`, which is `b 0x80161DA0`; the body forwards
  `(param_1, param_2)` to `0x80161DA0` and discards its `r3`.
- **The one-shot teardown gate `0x800643F0` (160 B, real 156).** Three gates
  (`0x8006C8F0`, `0x800B3B50(-1)`, the `r13 - 0x5986` mask ANDed with `+0x38E`)
  then `0x800D9AC0(+0x2DC)`; when `+0x366 == 1` it clears the byte and
  dispatches `[[this+0x60]+0x64](this)`.
- **The mode-2 hook `0x80064490` (168 B, real 164).** Rounds `+0xAC` through
  `0x80101AA0` unless `+0x120 & 0x400`, resets `+0x148`, ticks `+0x1EC` and
  `+0x2DC` while `+0x0B == 0`, posts `0x80069240(manager, *this)` when
  `+0x120 & 8`, clears `+0x310`/`+0x314`, then always calls
  `0x8006C980(this, mode)`.
- **The step predicate `0x80064540` (160 B, real 156).** Rejects on
  `0x8006C9F0 == 0`, on `((1 << +0x38C) & 0xFF & 0x80429CFB) != 0`, on
  `+0x36C == 2`, and accepts outright when `+0x120 & 2 == 0`; otherwise it
  returns "allowed" only for a **zero** result from `[[this+0x60]+0x60]`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80063F20` | `fn_80063f20` | 48 | high |
| `0x80063F50` | `fn_80063f50` | 48 | high |
| `0x80063F80` | `fn_80063f80` | 176 | high |
| `0x80064030` | `fn_80064030` | 212 | high |
| `0x80064110` | `fn_80064110` | 412 | high |
| `0x800642B0` | `fn_800642b0` | 148 | high |
| `0x80064380` | `fn_80064380` | 4 | medium |
| `0x800643F0` | `fn_800643f0` | 160 | high |
| `0x80064490` | `fn_80064490` | 168 | high |
| `0x80064540` | `fn_80064540` | 160 | high |

Byte total (`functions.tsv`): 1536. Real extents: 1508 (`48, 48, 172, 208,
408, 144, 4, 156, 164, 156`). Address range covered:
`0x80063F20..0x80064540`.

## Deliberate skips and why

**No body is deferred.** Three details are knowingly left out and are stated in
the matching doc blocks:

* the outlined register-save frames and the trailing 4-byte alignment pad of
  every body except the two 48-byte forwarders and the 4-byte thunk
  (`blr` positions are recorded per function);
* the four dead `stfs` spills to `r1+8..r1+0x14` in `0x80064110` (two
  `256.0f`, two `80.0f`; nothing reloads them);
* the `r12` member-pointer convention: `0x802DCBFC` cannot be called through
  the Rust ABI with `r12` preset, so the existing `invoke_pmf_d` reproduces its
  body.

## Verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to a scratch directory
(`%TEMP%\opencode\w211laneB`), this lane's block was appended to the copy of
`src/game/player.rs`, and `cargo check --lib` there exits 0 with the same 23
warnings as the unmodified tree (checked side by side in
`%TEMP%\opencode\w211laneB_base`).

Each `functions.tsv` size was re-derived from the prologue and the `blr` in
`work/dol/main.elf`; the discrepancies are the pad word before the next
aligned entry, and the `# Original` lines keep the tabulated value.
