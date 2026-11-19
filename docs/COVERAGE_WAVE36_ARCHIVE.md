# Coverage - Wave 36 (archive)

## How to read this file

Addresses are **virtual addresses of the unrelocated `main.dol`** (image base
`0x80004000`), not file offsets. Evidence paths are relative to
`work/ghidra/main-gb/export/`. Status and confidence follow the lane brief:
`translated` means the complete recorded control flow is present; `high` means
the bytes and call order are explicit, while `medium` means one hidden
argument or callee contract is inferred. Function names in this pass are
`fn_<addr>` per the wave-36 brief.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/archive.rs` | `resource::archive` | 30 | 2316 | 15 | 15 |

## What this pass covers

- the **heap-object factory cluster** at `0x802B8B40..0x802BA000` — the
  concrete heap-object types that wave 30's allocator
  (`0x802B8CC0`/`0x802B8F70`) drives but did not build. It is 30 functions
  sharing one object: the base heap object registered on the global heap list
  (`0x80423880` / mutex `0x80423890`), four vtables
  (`0x8034FD60`, `0x8034FD90`, `0x8034FDC0`, `0x8034FDF0`), the
  allocate/free trampolines forwarding to `arc_heap_allocate` /
  `arc_heap_free`, and the four placement factories that align a byte range,
  init the sub-heap through the `0x801Dxxxx` helpers and publish the object
  through the `r13 - 0x484C` hook.
- one coherent object family, so `+0x10` (owner), `+0x14` (base),
  `+0x18` (previous heap), `+0x28` (client list) and `+0x34` (name) are
  justified once and reused.

## What the `0x802B0000` band actually contains

`0x802B0000` has 306 functions in `functions.tsv`, 51 already translated
before this pass (`resource::archive` 28, `resource::bres` 12, others).
Sampling 8-10 exports shows the band is **not** uniformly archive/path code:

- `0x802B0680` (1844 B) is float-heavy geometry (matrices, `+0xF8` clamps,
  `FUN_802DCD64` pair) — NW4R math, not archive. Skipped.
- `0x802B1C90`, `0x802B2720..0x802B2B50` (dozens of 8-36 B copies) are
  small struct-copy helpers (`copy 1-4 words/shorts/bytes`) for math types —
  geometry, not archive. Skipped.
- `0x802B3300..0x802B4EE0` and `0x802B4940` are typed resource accessors:
  `+0x40 + (>>0x1C)*0x1C` / `+ (>>0x18 & 0xF)*0x14` seeks and a vtable
  dispatcher `(*this+0x14)(this,0)`. That is the NW4R `ResManager` /
  BRRES lookup family — the `resource::bres` lane's ground, left untouched.
- `0x802B5AF0..0x802B71E0` is a memset/zero-fill and larger state code
  (`0x802B5AF0` zeroes 16 bytes; `0x802B5E50`, `0x802B60B0`, `0x802B6C80`
  are 500-1400 B state machines) — utility/sys, not the heap object.
- `0x802B7380..0x802BA800` (51 already translated) is the EGG/U8 archive
  core: U8 walker (`0x802B7380..0x802B7A40`, `bres`), EGG object
  (`0x802B7AC0..0x802B8150`, `archive`), Yaz0 (`0x802B85D0..0x802B89A0`,
  `archive`), heap bridge (`0x802B8A70..0x802B9290`, `archive`) and
  mount-stream base (`0x802BA2B0..0x802BA800`, `registry`).
- `0x802B8420..0x802B8570` (5 functions) is a small `+0x0C` object with
  `countLeadingZeros(x) >> 5` wrappers over `0x801D6xxx` — a bit/flag
  object sharing no heap-list field. Skipped as a different object.
- `0x802B8B40..0x802BA000` (this pass) is the heap-object factory family
  described above.
- `0x802BA870..0x802BFFFF` is the EGG root-heap manager
  (`0x802BA870` calls `0x802B8B40`/`0x802B9360` three times and names
  `EGGRootMEM1/MEM2/Debug/System`), stream objects (`0x802BB580`
  `0x8034FF40` + mutex/cond), and later `EGG::CoreControllerMgr`
  (`0x802BD4A0`) and float/`ldexpf`/`GQR0` code (`0x802BF540`). The
  latter is explicitly skipped by the lane brief.

Identification: the band is NW4R archive/file-system **only** in the
`0x802B7380..0x802BFFFF` high half. The low half is math/ResManager
geometry. The archive-heap family is the only 25-30-function cluster in the
band that shares one registry object and has no float/paired-single code.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802B8B40` | `fn_802b8b40` | 64 | high |
| `0x802B8B80` | `fn_802b8b80` | 148 | medium |
| `0x802B8C20` | `fn_802b8c20` | 148 | high |
| `0x802B9070` | `fn_802b9070` | 4 | high |
| `0x802B9080` | `fn_802b9080` | 228 | medium |
| `0x802B9210` | `fn_802b9210` | 12 | high |
| `0x802B9220` | `fn_802b9220` | 8 | high |
| `0x802B9230` | `fn_802b9230` | 8 | high |
| `0x802B9240` | `fn_802b9240` | 16 | high |
| `0x802B9250` | `fn_802b9250` | 12 | high |
| `0x802B9260` | `fn_802b9260` | 8 | high |
| `0x802B9270` | `fn_802b9270` | 16 | high |
| `0x802B9290` | `fn_802b9290` | 8 | high |
| `0x802B92A0` | `fn_802b92a0` | 60 | medium |
| `0x802B9360` | `fn_802b9360` | 196 | medium |
| `0x802B9430` | `fn_802b9430` | 204 | medium |
| `0x802B9650` | `fn_802b9650` | 8 | high |
| `0x802B9660` | `fn_802b9660` | 8 | high |
| `0x802B9670` | `fn_802b9670` | 8 | medium |
| `0x802B9730` | `fn_802b9730` | 64 | medium |
| `0x802B9780` | `fn_802b9780` | 60 | medium |
| `0x802B9840` | `fn_802b9840` | 196 | medium |
| `0x802B99C0` | `fn_802b99c0` | 8 | high |
| `0x802B9AB0` | `fn_802b9ab0` | 76 | medium |
| `0x802B9B80` | `fn_802b9b80` | 212 | medium |
| `0x802B9DE0` | `fn_802b9de0` | 8 | high |
| `0x802B9DF0` | `fn_802b9df0` | 36 | medium |
| `0x802B9E70` | `fn_802b9e70` | 60 | medium |
| `0x802B9F30` | `fn_802b9f30` | 200 | medium |
| `0x802BA000` | `fn_802ba000` | 232 | medium |

## Cluster notes

### Base object and list

`0x802B8B40` inits the global heap list (`0x80423880`, capacity `0x20`)
and its mutex (`0x80423890`), then sets `r13 - 0x486C`. `0x802B8B80`
is the base constructor: it runs the mount-stream base (`0x802BA2D0`),
installs vtable `0x8034FD10`, stores the owner at `+0x10`, zeroes
`+0x14`/`+0x18`/`+0x1C`, stores the SDA word `r13 - 0x5B60` at `+0x34`,
inits the client list at `+0x28` (capacity 8) and appends itself.
`0x802B8C20` is the deleting destructor: null-tolerant, restores the
base vtable, removes itself under the lock, runs the base destructor
(`0x802BA330`) and frees when `deleting > 0`.

### Reporting and trampolines

`0x802B9080` holds the heap-list lock and calls each node's `+0x28`
slot with `(node, 4)`; the `0x90000000` test has identical arms in the
export, so the test is kept and the call duplicated. The inner search
for `node[6]` (`+0x18`, the previous-heap word) is preserved literally.

`0x802B9210..0x802B9290` are pure forwards to `arc_heap_allocate` /
`arc_heap_free`. `0x802B9210`/`0x802B9250` are `(size, 4, 0)`;
`0x802B9230`/`0x802B9260` are `(size, align, 0)`; `0x802B9240`/`0x802B9270`
are `(size, align, heap)` (the C swaps the last two operands of
`0x802B8CC0`); `0x802B9290` is `arc_heap_free(block, 0)`;
`0x802B9220` returns its second word (the factories' placement holder,
so `FUN_802b9220(0x38, aligned)` is `aligned`).

### Factories and allocate-and-construct wrappers

The four factories share one shape:

```text
aligned = (base + 3) & ~3
end     = (base + size) & ~3
if end < aligned || end - aligned < 0x3C (0x40 for type C) -> 0
sub     = 0x801Dxxxx(aligned + 0x38/0x3C, span - 0x38/0x3C)
holder  = 0x802B9220(0x38/0x3C, aligned)   (types B/C/D; type A uses aligned)
heap    = concrete_ctor(holder, sub)       (vtable A/B/C/D)
heap[+0x14] = base; heap[+0x18] = arc_heap_current()   (type C skips +0x14)
hook(r13 - 0x484C)(heap) if set
```

- type A (`0x802B92A0`/`0x802B9360`, `0x801D4380`, vtable `0x8034FD60`)
- type B (`0x802B9780`/`0x802B9840`, `0x801D4B80`, vtable `0x8034FD90`)
- type C (`0x802B9AB0`/`0x802B9B80`, `0x801D5180`, `0x3C`, vtable
  `0x8034FDC0`, extra at `+0x38`)
- type D (`0x802B9E70`/`0x802B9F30`, `0x801D4B80` with trailing `0`,
  vtable `0x8034FDF0`)

`0x802B9430`/`0x802BA000` are the allocate-and-construct wrappers: resolve
the heap (or current), expand `-1` through the `+0x28` slot (and `0`
through `0x802BA2B0`'s `0x80` for type D), allocate through `+0x14`,
construct through the factory, and free through `+0x18` on factory
failure only.

`0x802B9650`/`0x802B9660`/`0x802B99C0`/`0x802B9DE0` and
`0x802B9670` forward `*(this + 0x10)` (the owner word) to the
`0x801D4940`/`0x801D49A0`/`0x801D4D60`/`0x801D5420`/`0x801D49F0`
helpers. `0x802B9730` drains clients (`0x802B9010`) then forwards
three registers (including the `0x802B9720` label) to `0x802B9670`.
`0x802B9DF0` returns `0x801D54A0() + 0x3C` (the type-C header size).

## Unknowns

1. **`0x801Dxxxx` heap helpers are not exported as functions.** Their
   bodies are not in this lane, so they are called by address and marked
   `// unknown:` in the bodies. The factories' use (align, bound, init)
   is inferred from the argument shapes; the call order and stores are
   literal.
2. **`0x802DCD74`/`0x802DCDC0` pairs hide the factories' (`base`, `size`)
   arguments.** The pair is the CodeWarrior `__savegprlr` prologue; high
   is the base and low is the size (or, for the wrappers, high is the
   size and low is the heap). The third register is ignored by the
   bodies and is carried as `extra` / `unused` for call-shape fidelity.
3. **`0x802B8B80` and the concrete constructors drop `user`.** The
   factories call them with two words (`holder, subheap`), so the
   second word is forwarded as `user`. This matches `+0x10` being the
   sub-heap handle in the `0x802B9650`-family forwards.
4. **`0x802B9670` forwards three registers.** Only the first load is
   in the export; the extra two are the `0x802B9730` caller's
   `callback`/`arg`. Forwarding all three is harmless when the callee
   takes fewer.
5. **The `0x802B9080` inner search has no observed effect.** The
   `node[6]` comparison target walks from the head each iteration but is
   never used afterwards in the export. The translation preserves the
   walk literally.
6. **No Nintendo class symbols are claimed.** Heap vtables are given as
   `0x8034FDxx` addresses; the mount-stream base is referenced as
   `0x802BA2D0`/`0x802BA330` (translated in `registry`) rather than
   invented as `EGG::...` symbols.

## Deliberate skips

- `0x802B9700` (`thunk_FUN_801d4a90`, 4 B) — a 4-byte thunk
  (`return *(param1 - 0xC)`), not a body. Skipped per the lane brief's
  paired-single/thunk caution; called out instead of translated.
- `0x802B8420..0x802B8570` (5 functions) — small `+0x0C` flag object
  with `CLZ >> 5` wrappers over `0x801D6xxx`. Shares no heap-list field,
  so it is not the same cluster. Left for the owner of `0x801D6xxx`.
- `0x802B0680..0x802B71E0`, `0x802B2720..0x802B2B50`,
  `0x802B3300..0x802B3940`, `0x802B4940..0x802B4EE0` — math/struct-copy
  and ResManager/BRRES accessors. The latter are the `resource::bres`
  lane's ground; the former are geometry. Left untouched so the archive
  lane never claims a BRRES loader/decoder.
- `0x802BA870..0x802BFFFF` (EGG root manager, stream, core-controller and
  float code) — the heap manager builds on the factories this pass
  supplies but is a separate 30+ function pass; `0x802BF540` (`ldexpf` /
  `GQR0` paired-single) is an explicit skip per the brief.
- `0x802B8420`'s call to `0x801D6880` and the `0x802B8450`-family
  `0x801D6xxx` calls — not archive heap callees; not claimed.

## Verification

- Every address and byte size matches `functions.tsv` (read from the TSV,
  not from prose).
- Every evidence file exists at the cited `work/ghidra/main-gb/export/`
  path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/archive.rs`.
- No address was already in `ledger/functions.csv` (the 30 addresses
  were all in the `0x802B8B40..0x802BA000` miss list; the other 28
  `wave36-main-loop` rows are a concurrent lane and are ignored).
- `cargo check` is free of errors for `src/resource/archive.rs` (the
  6 errors reported are in `src/runtime/dvd.rs`, another lane's file).
- `node tools/check-lane-csv.mjs wave36` reports `wave36-archive.csv: 30 rows`
  with no problem naming `wave36-archive.csv`.
