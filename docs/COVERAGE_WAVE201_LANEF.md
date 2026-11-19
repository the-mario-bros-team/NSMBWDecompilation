# Wave 201 lane F - `game::bases` dispatcher callers

Two previously `partial` bodies that call the shipped main.dol interface
dispatcher `0x802DCBFC` directly. Both are now `high` confidence: the `r12`
descriptor setup, the receiver and the surrounding calls are all literal.

| Address | Name | Bytes | Confidence |
|---|---|---:|---|
| `0x8075C550` | `fn_8075c550` | 340 | high |
| `0x8064B660` | `fn_8064b660` | 84 | high |

## Dispatcher recap (re-read from `work/dol/main.elf`)

```
802dcbfc  lwz   r0,0x0(r12)   ; self_adjust
802dcc00  lwz   r11,0x4(r12)  ; index (signed)
802dcc04  lwz   r12,0x8(r12)  ; table offset, or fn address when index < 0
802dcc08  add   r3,r3,r0
802dcc0c  cmpwi cr0,r11,0
802dcc10  bt    0x802dcc1c    ; 0x4180000c: BO=12, BI=0 -> cr0.LT
802dcc14  lwzx  r12,r3,r12
802dcc18  lwzx  r12,r12,r11
802dcc1c  mtspr 288,r12
802dcc20  bcctr
```

`0x4180000c` is a signed `index < 0` guard, so a negative entry uses
`[r12+8]` verbatim as the function address. The only copy of this stub is
`0x802DCC24` (the `r4` variant), which carries the identical `blt`.

Both descriptors reached by this lane use the **literal** form
(`index = -1`, function address in `[r12+8]`), so the private helper
`interface_call_desc` (whose shortcut tests `index == 0`) would take the
double-indirection path and read `*(object + fn_address)`. The two bodies
therefore reproduce the dispatcher inline with the signed guard, exactly as
lane E did for `fn_80762890`.

## Bodies

### `fn_8075c550` - `0x8075C550` (340 bytes)

```
8075c5ec  lwz   r0,0x1e8(r30) ; mode word from the ship actor
8075c5f0  lis   r4,0x807d
8075c5f4  addi  r4,r4,-5088   ; r4 = 0x807CEC20 (table base)
8075c5f8  mr    r3,r30        ; receiver = actor
8075c5fc  mulli r0,r0,12      ; 12-byte descriptor stride
8075c600  add   r12,r4,r0     ; r12 = 0x807CEC20 + 12 * mode
8075c604  bl    0x802dcbfc
```

- `r12` setup: `0x807C_EC20 + *(this + 0x1E8) * 12`. It is **not** `r4`: `r4`
  only receives the table base `0x807C_EC20` (the old
  `abi_call2!(0x802D_CBFC, this, 0x807C_EC20)` passed a dead register).
- object: `r3 = r30 = this`.
- Final form: descriptor address computed from the table base, then the
  dispatcher modelled inline (negative index = literal function address).
- Record 0 is `{self_adjust = 0, index = -1, table = 0x8075CA50}` and
  `0x8075CA50` is `blr`, i.e. mode 0 is a no-op dispatch. The neighbouring
  words (`0x807CEC2C` onwards) are shared float/int constants, so only the
  populated entries of this table are dispatcher records.

### `fn_8064b660` - `0x8064B660` (84 bytes)

```
8064b674  lwz   r12,0x3b4(r3) ; +0x3B4 model object vtable
8064b678  lwz   r12,0x1c(r12) ; slot +0x1C
8064b680  addi  r3,r3,0x3b4   ; sub-object receiver
8064b684  bcctr
8064b688  mr    r3,r31        ; receiver = this
8064b68c  addi  r12,r31,1068  ; r12 = this + 0x42C  (1068 = 0x42C)
8064b690  bl    0x802dcbfc
8064b698  mr    r3,r31
8064b69c  bl    0x8006cd40
```

- `r12` setup: `addi r12,r31,1068` = **`this + 0x42C`**, not `+0x438`.
  `+0x438` is a separate word (`0x3C`) written by `fn_8064b1c0` and is not read
  by the dispatcher.
- object: `r3 = r31 = this`.
- Descriptor provenance: `fn_8064b1c0` copies `0x802F_D328..0x802F_D330`
  (zero in `work/dol/main.elf`) into `+0x42C..+0x434` and stores `0x3C` at
  `+0x438`; the class initialiser at `0x8064B2E0` then overwrites the triple
  with `0x807C_85A0 + ((*(this + 4) & 3) * 12)` (`rlwinm r4,r4,0,30,31` keeps
  the low two bits, `mulli r4,r4,12`).
- Table `0x807C_85A0` records (all literal form):

  | record | value |
  |---|---|
  | 0 | `{0, -1, 0x8064_B6C0}` |
  | 1 | `{0, -1, 0x8064_B6F0}` |
  | 2 | `{0, -1, 0x8064_B720}` |
  | 3 | `{0, -1, 0x8064_B750}` |

  Those four targets are the `+0xE8`/`+0xEC` step helpers right after this
  function, indexed by `*(this + 4) & 3`.
- Final form: descriptor at `this + 0x42C`, dispatcher modelled inline.

## Verification

Crate copied to `%TEMP%\opencode\laneF_check`, both bodies replaced, and
`cargo check --lib` exited 0 (23 pre-existing unrelated warnings, none from
these two functions).
