# Coverage - REL module `d_en_bossNP` (`game::bosses`)

## How to read this file

- **Scope** - this document covers only the boss-actor REL module
  `d_en_bossNP` (REL module 4), whose translation lives in
  `src/game/bosses.rs` under `game::bosses`.
- **Addresses are synthetic.** The REL-to-ELF adapter places this module at
  `0x80C00000` with entry `0x80C000F0`. The retail loader chooses a different
  base, so these are analysis addresses, not runtime addresses. Calls into
  `main.dol` (for example `0x8015F7C0`) are absolute DOL addresses.
- **Evidence** is the Ghidra pseudocode under
  `work/ghidra/rel-d_en_bossNP/export/`; the exact file is named in every row.
  `functions.tsv` is the function index.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and all observable memory writes are represented; `partial`
  means entry/exit and major effects only; `stub` means address/role only.
- **Confidence** is the strength of the translation, not whether the guessed
  role is correct. `high` means the bytes and stores are exact; `medium` means
  the bytes are exact but an object role or helper name is inferred.
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table, so these are
  reconstruction names. Every function doc carries an `# Original` block with
  the Ghidra name and evidence path.
- Start with the **summary**, then the **state-registration cluster**, then the
  **function table**. The **deferred candidates** section lists the highest-value
  neighbours that were deliberately left out of this pass.

## Summary

| metric | value |
|---|---:|
| functions in `functions.tsv` | 770 |
| text bytes in module | 356,396 (`0x5702C`) |
| functions translated in this pass | 23 |
| bytes translated | 27,768 |
| translated share of functions | 2.99% |
| translated share of text bytes | 7.79% |
| `StateID`-bearing files covered | 8 of 8 |
| state descriptors registered by those files | 113 |

The 23 functions are not a random address sample. They are the eight
`StateID` registrars, the state-list core they depend on, the Iggy actor
factory/destructor/name helper, and the model/animation loaders or constructors
that are tied to `g3d/...brres` strings.

## Synthetic-base warning in one place

The adapter image uses:

| item | value |
|---|---|
| canonical base | `0x80C00000` |
| entry | `0x80C000F0` |
| BSS start | `0x80C6C900` |
| BSS size | `0x487C` |

Every `0x80C...` address in this document and in `src/game/bosses.rs` is an
analysis address. A live dump will not match it. The BSS state list head
`0x80C6C900` is therefore "synthetic BSS + 0", not a hardware address.

## Cluster 1 - module core and the state list

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C000F0` | 44 | `fn_80c000f0` | REL entry; call the DOL constructor-table walker and the empty module hook | translated | high |
| `0x80C00160` | 28 | `fn_80c00160` | link one `{next, runner, state}` node at the list head | translated | high |
| `0x80C00180` | 84 | `fn_80c00180` | walk the list, advance the head, call each runner with `-1` | translated | high |
| `0x80C001E0` | 4 | `fn_80c001e0` | empty hook called by the entry point | translated | high |

The state list is the shared dependency of all eight registrars. The node
layout is exact: `+0x00` next, `+0x04` runner, `+0x08` state. `fn_80c00180`
advances the head **before** calling the runner, so the runner observes the
already-advanced list.

## Cluster 2 - the eight `StateID` registrars

Each registrar is a flat sequence of the same block: construct the state name,
copy nine descriptor words, install a vtable, optionally call a zero-argument
state-id factory and store its result at `+0x30`, then link a three-word node.
The copy rotation and the optional field are documented once in
`copy_state_descriptor` and `register_state` in the Rust module.

| address | size | boss class | states | vtables | runner(s) | evidence |
|---|---:|---|---:|---|---|---|
| `0x80C03880` | 3,660 | `daEnBossCastleIggy_c` | 18 | `0x80C5A200`, `0x80C5A1CC` | `0x80C046D0`, `0x80C04730` | `80c03880_FUN_80c03880.c` |
| `0x80C10BD0` | 3,160 | `daEnBossCastleMorton_c` | 15 | `0x80C5D208`, `0x80C5D1D4` | `0x80C11830`, `0x80C11890` | `80c10bd0_FUN_80c10bd0.c` |
| `0x80C146B0` | 4,368 | `daEnBossCastleRoy_c` | 20 | `0x80C5E19C` | `0x80C15820` | `80c146b0_FUN_80c146b0.c` |
| `0x80C29CC0` | 812 | `daEnBossKoopaDemoCage_c` | 3 | `0x80C62CA4` | `0x80C2A050` | `80c29cc0_FUN_80c29cc0.c` |
| `0x80C37030` | 2,216 | `daEnBossLarry_c` | 9 | `0x80C65EB4` | `0x80C37940` | `80c37030_FUN_80c37030.c` |
| `0x80C45810` | 3,132 | `daEnBossMorton_c` | 14 | `0x80C694A0` | `0x80C464B0` | `80c45810_FUN_80c45810.c` |
| `0x80C49540` | 3,852 | `daEnBossRoy_c` | 17 | `0x80C6A684` | `0x80C4A4B0` | `80c49540_FUN_80c49540.c` |
| `0x80C4E010` | 3,880 | `daEnBossWendy_c` | 17 | `0x80C6B8AC` | `0x80C4EFA0` | `80c4e010_FUN_80c4e010.c` |

What is proven:

- The number of `func_0x8015f7c0` calls, `FUN_80c00160` calls and descriptor
  copies is identical in every file (18/15/20/3/9/14/17/17).
- Every one of the 113 descriptor sources is a contiguous 9-word table and uses
  the same rotation.
- Every vtable, runner, node and name address is taken directly from the C
  export; no state name was invented.
- The registrars also write static tables and one-time guarded tables. Those
  writes are preserved byte-for-byte and in source order; their gameplay
  meaning is not claimed.

## Cluster 3 - Iggy actor methods

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C046D0` | 88 | `fn_80c046d0` | shared state-object destructor: clear base, optional release | translated | high |
| `0x80C04A10` | 136 | `fn_80c04a10` | compare the object's `:`-suffix name with a candidate | translated | medium |
| `0x80C04B30` | 156 | `fn_80c04b30` | Iggy actor factory: allocate `0x1108`, vtable, two controllers | translated | high |
| `0x80C04C10` | 88 | `fn_80c04c10` | Iggy actor destructor: clear base, optional release | translated | high |

The factory and destructor are tied together by the Iggy vtable `0x80C5A280`
and the `0x80C65B68` field written at `+0xE74`. The name helper is included
because the `:` handling and vtable slot `+0x20` are exact, even though the
`strchr`/`strcmp` callee names are inferred.

## Cluster 4 - model and animation loaders

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C297A0` | 228 | `fn_80c297a0` | cage model/animation loader (`g3d/cage_boss_koopa.brres`) | translated | medium |
| `0x80C2A600` | 556 | `fn_80c2a600` | Kameck princess/Kameck/broom loader | translated | medium |
| `0x80C2E4D0` | 128 | `fn_80c2e4d0` | W4 flying-boss constructor and model names | translated | high |
| `0x80C30150` | 128 | `fn_80c30150` | W6 flying-boss constructor and model names | translated | high |
| `0x80C32380` | 92 | `fn_80c32380` | W8 flying-boss constructor and model names | translated | high |
| `0x80C34F10` | 464 | `fn_80c34f10` | Larry body/damage/shell loader (`g3d/larry.brres`) | translated | medium |
| `0x80C4AAD0` | 464 | `fn_80c4aad0` | Wendy body/damage/shell loader (`g3d/wendy.brres`) | translated | medium |

The strings are the evidence: `g3d/cage_boss_koopa.brres`, `g3d/kameck.brres`,
`g3d/kameck_princess.brres`, `g3d/W4_Fly_Boss.brres`, `g3d/W6_Fly_Boss.brres`,
`g3d/W8_Fly_Boss.brres`, `g3d/larry.brres` and `g3d/wendy.brres`. The field
offsets and call arguments are exact; the model-animation object roles are
inferred from the repeating `0x80239E30 -> 0x801650D0` pattern.

## Function table

| address | Ghidra name | Rust function | role | status | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80C000F0` | `entry` | `fn_80c000f0` | REL entry | translated | high | `80c000f0_entry.c` |
| `0x80C00160` | `FUN_80c00160` | `fn_80c00160` | list insert | translated | high | `80c00160_FUN_80c00160.c` |
| `0x80C00180` | `FUN_80c00180` | `fn_80c00180` | list teardown | translated | high | `80c00180_FUN_80c00180.c` |
| `0x80C001E0` | `FUN_80c001e0` | `fn_80c001e0` | empty hook | translated | high | `80c001e0_FUN_80c001e0.c` |
| `0x80C03880` | `FUN_80c03880` | `fn_80c03880` | Iggy state registrar | translated | high | `80c03880_FUN_80c03880.c` |
| `0x80C046D0` | `FUN_80c046d0` | `fn_80c046d0` | state destructor | translated | high | `80c046d0_FUN_80c046d0.c` |
| `0x80C04A10` | `FUN_80c04a10` | `fn_80c04a10` | name-suffix comparison | translated | medium | `80c04a10_FUN_80c04a10.c` |
| `0x80C04B30` | `FUN_80c04b30` | `fn_80c04b30` | Iggy factory | translated | high | `80c04b30_FUN_80c04b30.c` |
| `0x80C04C10` | `FUN_80c04c10` | `fn_80c04c10` | Iggy destructor | translated | high | `80c04c10_FUN_80c04c10.c` |
| `0x80C10BD0` | `FUN_80c10bd0` | `fn_80c10bd0` | Castle Morton registrar | translated | high | `80c10bd0_FUN_80c10bd0.c` |
| `0x80C146B0` | `FUN_80c146b0` | `fn_80c146b0` | Castle Roy registrar | translated | high | `80c146b0_FUN_80c146b0.c` |
| `0x80C297A0` | `FUN_80c297a0` | `fn_80c297a0` | cage model loader | translated | medium | `80c297a0_FUN_80c297a0.c` |
| `0x80C29CC0` | `FUN_80c29cc0` | `fn_80c29cc0` | Koopa demo cage registrar | translated | high | `80c29cc0_FUN_80c29cc0.c` |
| `0x80C2A600` | `FUN_80c2a600` | `fn_80c2a600` | Kameck loader | translated | medium | `80c2a600_FUN_80c2a600.c` |
| `0x80C2E4D0` | `FUN_80c2e4d0` | `fn_80c2e4d0` | W4 constructor | translated | high | `80c2e4d0_FUN_80c2e4d0.c` |
| `0x80C30150` | `FUN_80c30150` | `fn_80c30150` | W6 constructor | translated | high | `80c30150_FUN_80c30150.c` |
| `0x80C32380` | `FUN_80c32380` | `fn_80c32380` | W8 constructor | translated | high | `80c32380_FUN_80c32380.c` |
| `0x80C34F10` | `FUN_80c34f10` | `fn_80c34f10` | Larry model loader | translated | medium | `80c34f10_FUN_80c34f10.c` |
| `0x80C37030` | `FUN_80c37030` | `fn_80c37030` | Larry registrar | translated | high | `80c37030_FUN_80c37030.c` |
| `0x80C45810` | `FUN_80c45810` | `fn_80c45810` | Morton registrar | translated | high | `80c45810_FUN_80c45810.c` |
| `0x80C49540` | `FUN_80c49540` | `fn_80c49540` | Roy registrar | translated | high | `80c49540_FUN_80c49540.c` |
| `0x80C4AAD0` | `FUN_80c4aad0` | `fn_80c4aad0` | Wendy model loader | translated | medium | `80c4aad0_FUN_80c4aad0.c` |
| `0x80C4E010` | `FUN_80c4e010` | `fn_80c4e010` | Wendy registrar | translated | high | `80c4e010_FUN_80c4e010.c` |

## Deferred candidates

These are deliberately outside this pass. They are listed so the next lane can
continue without re-inventorying the module.

| address | size | why deferred |
|---|---:|---|
| `0x80C00200` | 64 | allocator/release wrapper; role overlaps the shared destructor family |
| `0x80C003C0` | 88 | another destructor variant; needs its vtable owner identified |
| `0x80C04730` | 92 | second Iggy runner used by 12 action states |
| `0x80C04930` | 224 | recursive chain resolver; argument register for the virtual call is unclear |
| `0x80C04C80` | 60 | Iggy controller callback passed to `0x802DC9A0` |
| `0x80C04CC0` | 92 | Iggy controller callback passed to `0x802DC9A0` |
| `0x80C11830` | 88 | Castle Morton runner (5 states) |
| `0x80C11890` | 92 | Castle Morton action runner (10 states) |
| `0x80C15820` | 92 | Castle Roy runner (20 states) |
| `0x80C2A050` | 92 | Koopa demo cage runner (3 states) |
| `0x80C37940` | 92 | Larry runner (9 states) |
| `0x80C464B0` | 92 | Morton runner (14 states) |
| `0x80C4A4B0` | 92 | Roy runner (17 states) |
| `0x80C4EFA0` | 92 | Wendy runner (17 states) |
| `0x80C22CA0` | 2,520 | large boss AI function; not yet classified |
| `0x80C2F0C0` | 1,352 | large boss AI function; not yet classified |
| `0x80C30D10` | 1,404 | large boss AI function; not yet classified |
| `0x80C32800` | 2,492 | large boss AI function; not yet classified |
| `0x80C331C0` | 1,324 | large boss AI function; not yet classified |
| `0x80C4FF10` | 1,284 | large post-Wendy gameplay function; not yet classified |

The 12-byte state-id factories (`0x80C048xx`, `0x80C159xx`, `0x80C42Cxx`,
`0x80C465xx`, `0x80C468xx`, `0x80C4A5xx`, `0x80C4F0xx`) are **not** translated
as standalone functions. Their return values are preserved by calling them at
the same point as the original and storing the result in the state field.

## Unknowns

- The imported DOL lock/unlock variants (`0x802DCD50/64/68/6C` and
  `0x802DCD9C/B0/B4/B8`) are called with no arguments and bracket static
  initialization. Their exact names are unknown; the role is inferred from the
  guard flags and the table writes around them.
- The state-id factory return values (`0x8035...` and `0x8037...`) are opaque
  IDs. They are stored, never called.
- Most object fields in the model loaders (`+0x524`, `+0x544`, `+0x548`,
  `+0x600`, `+0x604`, `+0x67C`, `+0x6A8`, `+0x6AC`, `+0x6D8`, `+0x718`) are
  concrete offsets with inferred meanings: model setup context, BRRES handle,
  model, animation, damage animation, shell model and shell animation.
- No runtime addresses are claimed. Recovering the real REL base is the job of
  the `OSLink` loader lane, not this module.

## Verification

- `cargo check` passes.
- `node tools/doc-lint.mjs src` reports 0 violations.
- The 113 `register_state` calls were mechanically compared with the eight C
  files: destination, descriptor source, vtable, factory, runner, node and name
  all match.
- All non-state table writes and guard flags were compared with the C export in
  source order.
- No files outside this lane were edited.
