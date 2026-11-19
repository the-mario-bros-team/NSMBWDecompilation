# Coverage - Wave 210, lane D (`game::player`, the `0x80061950..0x80062260` cluster)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 10 | 1072 | 8 | 2 | 0 |

Status split: 10 `translated`, 0 `partial`, 0 deferred. Address range covered:
`0x80061950..0x80062260`.

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
  `r2 = 0x80433080`; `r13` (the `0x8042F680` small-data area) is not touched by
  this lane.

## What this pass covers

- **The anchor lookup `0x80061950` (100 B).** Reads the `u8` kind at `+0x3E4`
  and the `u8` id at `+0x3E3`, calls the `(kind, id)` actor table search
  `0x80061540` (kind `0..3` selects tables `0x1E9`/`0x1C0`/`0x1A3`/`0x1A4`),
  and on a hit hands the actor to `0x80061590` (latch `+0x3D0` plus the kind-`0`
  position offset `+0x3D4..+0x3DC`) and `0x800616A0` (apply the record).
- **The anchor notifier `0x80061A80` (80 B).** Re-runs the lookup and, when it
  succeeds, calls the `+0x18` slot of the interface object at `+0x394` with the
  receiver `player + 0x394` and the address of the zero-initialised BSS global
  `0x80355D0C`.
- **The release twins `0x80061D50` (88 B) and `0x80061DB0` (92 B).** Both test
  the handle and then call `0x8015F800(handle, 0)` and, when `count > 0`,
  `0x802B9280(handle)`; the second body carries the extra `handle != 0` test
  the original compiler emitted for the nested source guard. Both return the
  handle unchanged.
- **The chain walker `0x80061EF0` (224 B).** Five unrolled stages test
  `(*(u32 *)(node->+0x30))[0xC](node->+0x30)`; the first non-zero result
  returns that stage's parent, and after five zero results the walker recurses
  (`bl`, not a loop) on the sixth link.
- **The name comparator `0x80061FD0` (136 B).** Compares the object's
  `vtable[8]` name and the caller's string after their last `':'` using
  `strrchr` (`0x802E1C40`) and `strcmp` (`0x802E1AB4`); no colon guard is added
  because the image has none.
- **The member-pointer invokers `0x80062090` (48 B) and `0x800620C0` (48 B).**
  Each sets `r12` to a 12-byte pointer-to-member record (`owner + 0x18` and
  `owner + 0x24`, i.e. consecutive PMF slots), `r3 = r4 = object`, `r5 = owner`
  and tail-calls the thunk `0x802DCBFC`; the thunk is reimplemented as
  `invoke_pmf_d`.
- **The diagonal-extent setter `0x800620F0` (192 B).** Nudges `+0xAC`/`+0xB0`
  by `0.5a`/`-0.5b`, writes `1.5 * max(a, b)` to `+0x318`/`+0x31C`, clears
  `+0x320`/`+0x324`, writes the four half-extents to `+0x4D0..+0x4DC`, then
  publishes the three pointers through `0x80062600`.
- **The one-shot release `0x80062260` (64 B).** `handle != 0 && count > 0`
  directly calls `0x802B9280(handle)` and returns the handle.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80061950` | `fn_80061950` | 100 | high |
| `0x80061A80` | `fn_80061a80` | 80 | high |
| `0x80061D50` | `fn_80061d50` | 88 | high |
| `0x80061DB0` | `fn_80061db0` | 92 | high |
| `0x80061EF0` | `fn_80061ef0` | 224 | high |
| `0x80061FD0` | `fn_80061fd0` | 136 | high |
| `0x80062090` | `fn_80062090` | 48 | medium |
| `0x800620C0` | `fn_800620c0` | 48 | medium |
| `0x800620F0` | `fn_800620f0` | 192 | high |
| `0x80062260` | `fn_80062260` | 64 | high |

Byte total: 1072. Address range covered: `0x80061950..0x80062260`.

## Deliberate skips and why

**None.** Three details are knowingly left out of the bodies and are stated in
the matching doc blocks:

* the outlined 16-byte register-save frame and the trailing 4-byte alignment
  pad of every body (`blr` positions are recorded per function);
* the four dead `stfs` spills to `r1+8..r1+0x14` in `0x800620F0` (the 16-byte
  callee `0x80062600` reads only `r3..r6`);
* the `r12` member-pointer convention itself: `0x802DCBFC` cannot be called
  through the Rust ABI with `r12` preset, so `invoke_pmf_d` reproduces its
  three-instruction body (`add this`, optional vtable double-indirection, tail
  call).

## Verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to a scratch directory, this
lane's block was appended to the copy of `src/game/player.rs`, and
`cargo check --lib` there exits 0 with the same 23 pre-existing warnings as the
unmodified tree.
