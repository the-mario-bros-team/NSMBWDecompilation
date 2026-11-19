# Wave 246, lane A - hidden-`r12` forwarders

## Scope

The `partial` ledger class targeted by this lane is the cluster of 149 rows filed
under `game::bosses`, `game::enemies`, `game::player` and `game::profile` whose
bodies are 48-byte `0x802DCBFC` thunks. All four modules share the same adapter
shape; none of them is a `game::bases` row, which is why the shared helper had
not been reused across module boundaries.

## How `r12` is built

Every genuine forwarder in the class has the same four-instruction setup:

```
mr   r5,r3          ; r5 = interface (the C `param_1`)
mr   r3,r4          ; r3 = receiver (the C `param_2`)
addi r12,r5,K       ; descriptor = interface + K
bl   0x802DCBFC     ; main.dol interface dispatcher
```

The `r12` operand is therefore always a **literal `addi r12,r5,K`**. No row in
this class uses a `lis`/`addi` literal descriptor pair or a table-derived
address. The offsets have only three values:

| `K` | rows |
|---|---:|
| `0x18` | 71 |
| `0x24` | 71 |
| `0x0C` | 3 |

The `0x802DCBFC` dispatcher reads `{self_adjust, index, table}` at `r12`, adds
`self_adjust` to `r3`, and tail-calls the resolved slot. That is exactly the
model already implemented by `game::bases::interface_call(interface, object,
offset)` / `interface_call_desc(descriptor, object)`, which delegates to the
canonical `runtime::cpp::fn_802dcbfc`. Both helpers are now `pub(crate)` so
sibling modules can reuse them instead of the lossy three-argument
`platform::call3` transcription.

## Converted batch (25)

These are the first 25 of the 44 `game::bosses` adapters, in address order.
Each body now calls
`interface_call(param_1 as usize, param_2, K)`, and its doc comment records the
raw instruction sequence, the descriptor offset, and the descriptor/vtable
preconditions.

| address | `K` | address | `K` |
|---|---|---|---|
| `0x80C04AD0` | `0x18` | `0x80C15BC0` | `0x18` |
| `0x80C04B00` | `0x24` | `0x80C15BF0` | `0x24` |
| `0x80C06B20` | `0x18` | `0x80C18530` | `0x18` |
| `0x80C06B50` | `0x24` | `0x80C18560` | `0x24` |
| `0x80C0A070` | `0x18` | `0x80C1C630` | `0x18` |
| `0x80C0A0A0` | `0x24` | `0x80C1C660` | `0x24` |
| `0x80C0E2F0` | `0x18` | `0x80C21560` | `0x18` |
| `0x80C0E320` | `0x24` | `0x80C21590` | `0x24` |
| `0x80C11C10` | `0x18` | `0x80C29560` | `0x18` |
| `0x80C11C40` | `0x24` | `0x80C29590` | `0x24` |
| `0x80C2A330` | `0x18` | `0x80C2A360` | `0x24` |
| `0x80C2C2F0` | `0x18` | `0x80C2C320` | `0x24` |
| `0x80C2E430` | `0x18` | | |

All 25 were re-adjudicated to `translated` / `high` in
`ledger/agents/wave246-bosses.csv` (merged into `ledger/functions.csv`).

## Residue

After the batch, 124 of the original 149 rows remain `partial`:

| module | remaining | genuine forwarders | other |
|---|---:|---:|---:|
| `game::bosses` | 19 | 19 | 0 |
| `game::enemies` | 92 | 90 | 2 |
| `game::player` | 11 | 9 | 2 |
| `game::profile` | 2 | 2 | 0 |
| **total** | **124** | **120** | **4** |

### 120 forwarders

These are mechanically identical to the converted batch: the same four
instructions with `K` in `{0x18, 0x24, 0x0C}`. They remain `partial` only
because they were outside this batch. Converting them requires no new analysis
or helper: replace the `platform::call3(0x802D_CBFC, param_2, param_2,
param_1)` line with `interface_call(param_1 as usize, param_2, K)` and update
the doc block. The remaining `game::bosses` rows are the 19 siblings directly
after the converted run; the `game::enemies` rows are scattered adapter pairs
across that module; `game::profile` holds `0x80501920`/`0x80501950`.

### 4 non-forwarders

These rows are in the same module/class bucket but are **not** `0x802DCBFC`
adapters, so the `interface_call` model does not apply:

* `game::enemies` `0x80A0DB40` - height compare plus two `+0xE8` stores; the
  jump table at `0x80A0DB80` is not recovered.
* `game::enemies` `0x80AEEA30` - packed-nibble to `0.5 * (1.0 + n)` float
  conversion; one store reads an uninitialised caller stack slot.
* `game::player` `0x80049940` - ordinary body that walks animation/state
  fields and calls several helpers; not a thunk.
* `game::player` `0x8004DB00` - calls `0x802DCBFC` with a *literal* descriptor
  (`0x80308300`) and a scaled `r5` argument, rather than the
  `addi r12,r5,K` interface convention. It needs its own argument/descriptor
  analysis, not the shared `interface_call` shortcut.

## Verification

* `cargo check` - clean.
* `doc-lint` (`node tools/doc-lint.mjs src`) - 0 violations.
* `verify-ledger --enforce-bodies` - 0 gaps after the wave-246 ledger merge.
