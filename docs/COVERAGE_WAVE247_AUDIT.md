# Audit - Wave 247 (forwarders finished, two vtables complete)

## 1. The `partial` column collapsed: 172 -> 52

All **120 remaining forwarders** were converted to
`interface_call(interface, object, K)`: bosses 19, enemies 90, player 9,
profile 2. Every body was re-read from its shipped image and verified to have
the uniform `mr r5,r3` / `mr r3,r4` / `addi r12,r5,K` / `bl 0x802DCBFC` shape,
with the `bl` resolving to `0x802DCBFC`. `K` census over all 145 forwarders:
**`0x18` x71, `0x24` x71, `0x0C` x3**. Ten were spot-checked by disassembly and
a full source re-scan confirms every offset.

Ledger: **20,196 translated / 52 partial / 2 stub**.

The 52 remaining partials are:

- the 4 non-forwarders (`0x80A0DB40` jump table, `0x80AEEA30` uninitialised
  stack slot, `0x80049940` multi-helper, `0x8004DB00` literal-descriptor call);
- 2 stubs (`sdk_panic`, `set_exception_handler`) whose 300-byte bodies are
  forwarded, not re-created;
- ~46 documented machine-state / `va_list` / `rfi` / `stmw` limitations where
  the missing information is not expressible in Rust.

## 2. Two vtables completed

| Table | Slots | Result |
|---|---:|---|
| player-class `0x803084C8` | 275 | **275/275**, `0x8030891C` terminator |
| WorldMap actor `0x80325420` | 278 | **0 uncovered**; run ends at `0x80325878` |

WorldMap's last nine words were 269-277, not 269-278: 278+ are the null head of
the *next* vtable (`0x80325878`, the map object's `+0x14FC` secondary), which
must be treated as its own table next time.

## 3. Bodies translated (41)

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| B | `WM_PLAYER_VTABLE` 265-283 | 12 | 820 |
| C | `dEn_c` 183-194 | 10 | 760 |
| D | WorldMap vtable 269-277 | 4 + 3 reused | 976 |
| E | player-class vtable, last 8 | 5 + 3 reused | 200 |
| F | `0x803502F8` band 0x80350584+ | 10 | 612 |

## 4. Findings

- Lane B documented an unreachable residual: slot 274's `cmpli r0,3` +
  `bc 4,1` sends `(unsigned)(b-3) >= 3` to the `+0x3F4` arm, so the residual
  `cmpi b,17` in the `3..=5` arm can never run - the `+0x108` arm is documented
  but not modelled.
- `0x801462A0` (WorldMap `+0x450`) is a message dispatcher with a 16-entry jump
  table at `0x803253C0`; messages 8/14/15 alias the default arm.
- `0x80376B48` confirmed as `daYoshi_c::StateID_DamageRun` from
  `0x80153400`'s registration.
- Lane B again avoided a duplicate: slot 273 was already published by lane E, so
  it dropped its copy and took slot 283 instead.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `scripts/check.ps1` exit 0, body enforcement armed.
