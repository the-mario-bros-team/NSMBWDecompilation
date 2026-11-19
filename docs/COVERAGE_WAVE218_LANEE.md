# Wave 218 lane E - `src/engine/dsys.rs`, actor cluster `0x800FE040..0x800FF680`

Lane E translated 12 of the 12 bodies it was given. Nothing was deferred.

## Bodies

| address | size (tsv) | real extent | name | confidence | what it does |
|---|---|---|---|---|---|
| `0x800FE040` | 8 | `0x800FE040..0x800FE048` (8) | `fn_800fe040` | high | tail call `0x8016E230(param + 4)` |
| `0x800FE050` | 8 | `0x800FE050..0x800FE058` (8) | `fn_800fe050` | high | returns `*(u32*)(r13 - 0x6A60)` |
| `0x800FE1D0` | 196 | `0x800FE1D0..0x800FE294` (196) | `fn_800fe1d0` | high | construct the 0x404-byte actor |
| `0x800FE2E0` | 120 | `0x800FE2E0..0x800FE358` (120) | `fn_800fe2e0` | high | destroy, optional free |
| `0x800FE400` | 344 | `0x800FE400..0x800FE558` (344) | `fn_800fe400` | medium | four `g3d/model.brres` loads + `YB_model` record walk |
| `0x800FE560` | 512 | `0x800FE560..0x800FE760` (512) | `fn_800fe560` | medium | bind the model and the four animation tracks |
| `0x800FE760` | 200 | `0x800FE760..0x800FE828` (200) | `fn_800fe760` | medium | build the two matrices, start the rebuild passes |
| `0x800FEC10` | 440 | `0x800FEC10..0x800FEDC8` (440) | `fn_800fec10` | high | `YB_Rwait` variant select + three transform applies |
| `0x800FF000` | 84 | `0x800FF000..0x800FF054` (84) | `fn_800ff000` | high | base step and release of the bound object |
| `0x800FF180` | 144 | `0x800FF180..0x800FF210` (144) | `fn_800ff180` | high | look a record up and apply it to `+0x218` |
| `0x800FF660` | 20 | `0x800FF660..0x800FF674` (20) | `fn_800ff660` | high | tail call with kind `0x11` |
| `0x800FF680` | 32 | `0x800FF680..0x800FF6A0` (32) | `fn_800ff680` | high | virtual tail call, slot `+0x50`, kind `0x17` |

Total: 2108 bytes, 12 bodies, 0 deferrals.

## Extent checks

Two bodies are 8 bytes. Neither has a conventional prologue:

* `0x800FE040` is `addi r3,r3,4` + `b 0x8016E230` - a two-instruction tail call,
  no frame, no `blr`. 8 bytes matches `functions.tsv`.
* `0x800FE050` is `lwz r3,0xffff95a0(r13)` + `blr` - a leaf. 8 bytes matches
  `functions.tsv`.

`0x800FF660` (20 bytes) is also a pure tail call (`mr` x3, `li`, `b`) and
`0x800FF680` (32 bytes) ends in `bcctr`, so all four small bodies were verified
from the instruction stream rather than from a prologue. Every larger body was
checked the same way: the `stwu` frame and the `blr` (or restored `r1` + `blr`)
of each one land exactly on the `functions.tsv` end address, as listed in the
table above.

## Evidence and reconstruction notes

* `0x800FE1D0` prints its float seed as `undefined4`, but `0x800FE24C` is an
  `lfs` and the four stores at `0x800FE260..0x800FE270` are `stfs`, so the slot
  is read and written with `read_f32`/`write_f32`.
* `0x800FE400` and `0x800FE560` address the SDA resource strings directly. The
  image bytes at `0x80428BD0`, `0x80428BD8`, `0x80428BE0`, `0x80428BE8`,
  `0x80428BF0` are `"Yoshi"`, `"P_rcha"`, `"Y_rcha"`, `"wait"`, `"YB_wait"`,
  and `0x8031C258`/`0x8031C268` are `"g3d/model.brres"`/`"YB_model"`.
* `0x800FEC10`: the export reorders the `0x800D65E0` call as
  `(param_1, param_3, param_2, param_4[9], iVar3)`. `0x800FED88..0x800FED98`
  shows `f1 = f29`, `f2 = f31`, `f3 = f30`, with `r3 = *(param_4 + 0x24)` and
  `r4 = iVar3` already loaded, so the reconstruction uses a
  `(f64, f64, f64, u32, u32)` call shape and passes `(param_1, param_3,
  param_2)` in that order.
* `0x800FE760`: `0x801C0540` is called with `r3 = r5 = r1 + 0x38`, so the
  matrix multiply is in place; the export prints `auStack_38` twice.
* `0x800FF680`: Ghidra's "could not recover jumptable" warning is spurious. The
  six instructions are a plain virtual tail call and the next function begins at
  `0x800FF6A0`.
* All new constants and helpers in the lane block carry the `_W218E` suffix;
  existing unsuffixed constants of `src/engine/dsys.rs` were reused where they
  name the same address (`FN_800DF180`, `FN_80239E30`, `FN_8023B8C0`,
  `FN_80169D90`, `FN_8023A0B0`, `FN_80165520`, `FN_801656C0`, `FN_8016F130`,
  `FN_801C0BD0`, `FN_801C0540`, `FN_8016A170`, `FN_8016A1A0`, `FN_801680E0`,
  `FN_80168FE0`, `FN_80169F80`, `FN_80168080`, `FN_802B9280`,
  `FN_800B2F10`, `G_RESOURCE_ROOT`).

## Verification

A scratch copy (`src/`, `Cargo.toml`, `Cargo.lock`) with this lane block appended
to `src/engine/dsys.rs` builds cleanly:

```
cargo check --lib
Finished `dev` profile [unoptimized + debuginfo] target(s) in 18.06s
Exit code: 0
```

The three warnings attributed to the appended lines are
"value assigned to `stack` is never read" on the stack buffers whose address is
handed to opaque callees - the same shape the rest of the file already uses.

## Deferrals

None.
