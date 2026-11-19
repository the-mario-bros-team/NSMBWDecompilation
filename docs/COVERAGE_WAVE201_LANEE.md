# Wave 201 lane E - `game::bases` dispatcher callers

Two previously `partial` bodies that call the shipped main.dol interface
dispatcher `0x802DCBFC` directly. Both are now `high` confidence: the `r12`
descriptor setup, the receiver and the surrounding calls are all literal.

| Address | Name | Bytes | Confidence |
|---|---|---:|---|
| `0x806F9890` | `fn_806f9890` | 48 | high |
| `0x80762890` | `fn_80762890` | 156 | high |

## Dispatcher recap (re-read from `work/dol/main.elf`)

```
802dcbfc  lwz   r0,0x0(r12)   ; self_adjust
802dcc00  lwz   r11,0x4(r12)  ; index (signed)
802dcc04  lwz   r12,0x8(r12)  ; table offset, or fn address when index < 0
802dcc08  add   r3,r3,r0
802dcc0c  cmpwi cr0,r11,0
802dcc10  bt    0x802dcc1c    ; BI=0 -> branch if index < 0
802dcc14  lwzx  r12,r3,r12
802dcc18  lwzx  r12,r12,r11
802dcc1c  mtspr 288,r12
802dcc20  bcctr
```

**Correction:** `0x4180000c` is `bt 0` (`BO=12`, `BI=0` = `cr0.LT`), so the
guard is a *signed* `index < 0` test, not `index != 0`. A negative entry uses
`[r12+8]` as the function address itself. Lane A's `0x802DCBFC` recap (and the
`index == 0` shortcut in the private helper `interface_call_desc`) reflect the
older `index != 0` reading; the shipped branch is `index < 0`. Descriptors with
`index > 0` are unaffected by the difference, which is why lanes A-D still hold.

## Bodies

### `fn_806f9890` - `0x806F9890` (48 bytes)

```
806f9898  mr   r5,r3          ; r5 = interface (param_1)
806f989c  mr   r3,r4          ; r3 = receiver (param_2)
806f98a4  addi r12,r5,0xc     ; descriptor at param_1 + 0xc
806f98a8  bl   0x802dcbfc
```

- `r12` setup: `addi r12,r5,0xC` with `r5 = r3`.
- object: `r4` (moved into `r3`), not `param_1`.
- Final form: `interface_call(interface, object, 0xC)`.
- The old doc claimed the body was byte-for-byte identical to `fn_806f9860`
  (offset `0x18`). It is not: `fn_806f9860` uses `addi r12,r5,0x18`, this body
  uses `addi r12,r5,0xC`. The shared function is the shape, not the offset.
- Return: Ghidra's `void` is kept. There is no `li r3,1` override; the
  dispatcher's `r3` is not propagated.

### `fn_80762890` - `0x80762890` (156 bytes)

```
807628d4  lwz   r0,0x254(r30) ; state word from the actor
807628d8  lis   r4,0x807d
807628dc  addi  r4,r4,-4532   ; 0x807CEE4C
807628e0  mr    r3,r30        ; receiver = actor
807628e4  mulli r0,r0,12      ; 12-byte descriptor stride
807628e8  add   r12,r4,r0     ; r12 = 0x807CEE4C + 12 * state
807628ec  bl    0x802dcbfc
```

- `r12` setup: **not** a plain literal and **not** `r4`. `r4` is loaded with the
  literal base `0x807CEE4C` and then indexed by `12 * [actor+0x254]`, and the
  result is moved into `r12`. So the `0x807C_EE4C` in the old
  `abi_call2!(0x802D_CBFC, actor, 0x807C_EE4C)` was a table base, never an `r4`
  payload for the dispatcher (`r4` is dead at the `bl`).
- object: `r3 = r30 = actor`.
- Final form: descriptor computed from the table base, then the dispatcher
  modelled inline with the signed `index < 0` guard (the existing
  `interface_call_desc` shortcut would take the wrong path here).
- Table contents: the only populated entry is `state == 0`,
  `{self_adjust = 0, index = -1, table = 0x80762C50}` (`0x807CEE4C = 0`,
  `0x807CEE50 = 0xFFFFFFFF`, `0x807CEE54 = 0x80762C50`), a direct call to the
  `bcr` stub at `0x80762C50`. That entry is exactly what the signed guard
  predicts, and it cross-checks the guard reading. Words after `0x807CEE58`
  belong to the shared constant pool (`0x807CEE58` is read as a byte by
  `fn_80762970`, `0x807CEE5C`/`0x807CEE60`/`0x807CEE64` as floats by
  `fn_80762ce0`, `0x807CEE68` is the `0x43300000` int-to-double magic), so the
  array has a single entry in this image.
- Neighbour `0x80762C40` is the 12-byte setter that stores `0` into
  `+0x254`, consistent with the state word being zero in this class.

## Verification

Crate copied to `%TEMP%\opencode\laneE_check`, both bodies replaced, and
`cargo check --lib` exited 0 (23 pre-existing unrelated warnings, none from
these two functions).
