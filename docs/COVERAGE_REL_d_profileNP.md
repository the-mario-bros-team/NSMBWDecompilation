# Coverage - REL module `d_profileNP` (`game::profile`)

## How to read this file

This document covers **REL module 1**, `d_profileNP`, and the Rust module
[`src/game/profile.rs`](../src/game/profile.rs). The module is the profile
front end's yes/no confirmation dialog: it registers eight
`dYesNoWindow_c` state descriptors, constructs the window object, and runs the
open/select/hit/close state machine. It also contains a
`ControllerInformation` overlay.

Read it in this order:

1. **Scope and addresses** - what was exported and where it lives.
2. **Status counts** - how much is translated and what confidence means.
3. **Cluster summary** - the six roles the 36 functions fall into.
4. **Inventory** - one row per exported function.
5. **External calls and referenced-but-unexported addresses** - what is still
   unresolved.
6. **Evidence notes, uncertainties and next targets**.

All addresses are **synthetic analysis addresses**, not runtime addresses. The
REL was relocated to the canonical base **`0x80500000`** by `tools/rel2elf.mjs`
(see `docs/REL.md` and `docs/REL2ELF.md`); the entry point is
**`0x805000E0`**. Addresses outside `0x80500000..0x80503B08` are imports:
mostly `main.dol`, plus cross-REL pointers in the descriptor block at
`0x80502288`. They remain absolute analysis addresses, not runtime addresses.

Status and confidence follow `docs/RUST_STYLE.md`:

| status | meaning |
|---|---|
| `translated` | full control flow; every call is represented or marked |
| `partial` | flow is present, but one operand is not recoverable |
| `stub` | address and role only |

Evidence paths are relative to the repository root. The canonical evidence is
`work/ghidra/rel-d_profileNP/export/` (36 C files and `functions.tsv`); the
relocated image used to read data tables is
`work/rels/elf/d_profileNP.elf`.

**Start here:** [`entry`](../src/game/profile.rs) (`0x805000E0`) ->
`register_yes_no_states` (`0x80501980`) -> `state_enter_open_wait`
(`0x80501050`) -> `state_poll_close` (`0x80501700`).

## Scope and addresses

| item | value |
|---|---|
| REL module ID | 1 |
| canonical base | `0x80500000` |
| entry | `0x805000E0` |
| text range | `0x805000E0..0x80502098` (8,120 bytes) |
| data range | `0x80502098..0x80503884` |
| BSS range | `0x80503900..0x80503B08` (520 bytes) |
| `functions.tsv` entries | 36 |
| total exported bytes | 5,596 |
| entries below 16 bytes | 1 (`0x805001F0`, 4-byte branch thunk; included as a wrapper) |

The text contains all 36 exported functions. The data holds the state-seed
table, the descriptor block at `0x80502288`, the state vtable at `0x80503850`,
strings and the 29-entry profile lookup tables. The BSS holds the list head and
the eight `0x40`-byte state records.

## Status counts

| status | functions | bytes | notes |
|---|---:|---:|---|
| `translated` | 34 | 5,500 | includes the 4-byte thunk |
| `partial` | 2 | 96 | `0x80501920` / `0x80501950`: hidden `r12` operand |
| `stub` | 0 | 0 | none |
| **total** | **36** | **5,596** | |

| confidence | functions |
|---|---:|
| high | 23 |
| medium | 13 |
| low | 0 |

No function was skipped. The 4-byte thunk at `0x805001F0` is implemented as
[`thunk_install_profile_descriptor`](../src/game/profile.rs) so that the entry
point can call the same symbol the original calls.

## Cluster summary

| cluster | functions | bytes | status |
|---|---:|---:|---|
| Boot and module install | 4 | 84 | translated |
| State / profile table registration | 4 | 1,776 | translated |
| Yes/no window lifecycle | 10 | 820 | translated (medium for stubs) |
| Controller-information overlay | 4 | 1,188 | translated (one medium) |
| Descriptor lookup / state callbacks | 12 | 1,632 | translated |
| Virtual forwarders | 2 | 96 | partial |

### Boot and module install

`entry` calls a `main.dol` initializer runner with the word at `0x80502098`
(which holds `0x80501980`), then calls the `install_profile_descriptor` thunk.
`install_profile_descriptor` stores the descriptor block at `0x80502288` into
the `main.dol` global `0x8042A3B8`; `clear_profile_descriptor` clears it.

### State / profile table registration

`register_yes_no_states` builds the eight records and links them through
`register_state_descriptor`. Each record is:

```text
+0x00 next (list)       +0x04 cleanup (0x80501FB0)
+0x08 descriptor ptr    +0x0C padding
+0x10 descriptor: vtable 0x80503850, then nine words from 0x805035D0
```

`unregister_all_states` drains the list and calls the cleanup callback with
delete flag `-1`, so `state_cleanup` runs the base destructor but does not free.

### Yes/no window lifecycle

`yes_no_window_ctor` creates the embedded `lyt` layout, four interface
sub-objects and the state manager, and installs the final state-manager vtable
`0x805034C0`. The seven identical stubs at `0x805006A0..0x80500820` are
deleting destructors whose owning classes Ghidra could not associate. The two
stubs at `0x80500860` / `0x805008C0` additionally destroy an embedded object at
`+0x18`.

### Controller-information overlay

`create_controller_information` allocates `0x210` bytes and runs
`controller_information_ctor`. `controller_information_load_layout` loads
`ControllerInformation/ControllerInformation.arc`, builds
`ControllerInformation_07.brlyt`, and binds three `brlan` groups.
`refresh_controller_hint` uses the 29-entry tables to set pane visibility and
animation frame ids for the selected profile index.

### Descriptor lookup / state callbacks

Twelve exported callbacks cover six state pairs. The exported slots are the
`+0x0C` descriptor triple (enter/setup) and the `+0x18` triple (poll/wait).
Each poll calls the state manager's vtable slot `+0x18` with a fixed descriptor
address:

| state | enter | poll | next descriptor |
|---|---|---|---|
| `OpenAnimeEndWait` | `0x80501050` | `0x80501160` | `0x80503998` |
| `ButtonOnStageAnimeEndWait` | `0x805011D0` | `0x80501230` | `0x805039D8` |
| `ButtonChangeAnimeEndWait` | `0x805012B0` | `0x80501320` | `0x80503A18` |
| `SelectWait` | `0x80501390` | `0x80501400` | hit, close or button-change |
| `HitAnimeEndWait` | `0x80501530` | `0x805015C0` | `0x80503A98` or `0x80503AD8` |
| `ClouseAnimeEndWait` | `0x80501660` | `0x80501700` | `0x80503918` (`InitWait`) |

The `InitWait` state (`0x80503918`) has all three callbacks outside the export,
so its enter/poll pair is not in this file; only the descriptor is created.

## Inventory

Every row below is one `functions.tsv` entry. `synthetic base: 0x80500000`
applies to every address.

| address | name here | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x805000E0` | `entry` | queue state-table initializer; install descriptor | translated | high | `805000e0_entry.c` |
| `0x80500150` | `register_state_descriptor` | push record onto state list | translated | high | `80500150_FUN_80500150.c` |
| `0x80500170` | `unregister_all_states` | drain list and call cleanup `(-1)` | translated | high | `80500170_FUN_80500170.c` |
| `0x805001D0` | `install_profile_descriptor` | store `0x80502288` into `0x8042A3B8` | translated | high | `805001d0_FUN_805001d0.c` |
| `0x805001F0` | `thunk_install_profile_descriptor` | 4-byte branch thunk | translated | high | `805001f0_thunk_FUN_805001d0.c` |
| `0x80500200` | `clear_profile_descriptor` | clear the same global | translated | high | `80500200_FUN_80500200.c` |
| `0x80500210` | `create_controller_information` | allocate `0x210` and construct | translated | medium | `80500210_FUN_80500210.c` |
| `0x80500240` | `controller_information_ctor` | base ctor, vtable, layout, clear `+0x208` | translated | high | `80500240_FUN_80500240.c` |
| `0x805003A0` | `controller_information_load_layout` | archive, `.brlyt`, three `.brlan` groups | translated | high | `805003a0_FUN_805003a0.c` |
| `0x805005F0` | `yes_no_window_ctor` | vtables, interfaces, state manager | translated | high | `805005f0_FUN_805005f0.c` |
| `0x805006A0` | `fn_805006a0` | deleting-destructor stub | translated | medium | `805006a0_FUN_805006a0.c` |
| `0x805006E0` | `fn_805006e0` | deleting-destructor stub | translated | medium | `805006e0_FUN_805006e0.c` |
| `0x80500720` | `fn_80500720` | deleting-destructor stub | translated | medium | `80500720_FUN_80500720.c` |
| `0x80500760` | `fn_80500760` | deleting-destructor stub | translated | medium | `80500760_FUN_80500760.c` |
| `0x805007A0` | `fn_805007a0` | deleting-destructor stub | translated | medium | `805007a0_FUN_805007a0.c` |
| `0x805007E0` | `fn_805007e0` | deleting-destructor stub | translated | medium | `805007e0_FUN_805007e0.c` |
| `0x80500820` | `fn_80500820` | deleting-destructor stub | translated | medium | `80500820_FUN_80500820.c` |
| `0x80500860` | `fn_80500860` | destructor with `+0x18` sub-object | translated | medium | `80500860_FUN_80500860.c` |
| `0x805008C0` | `fn_805008c0` | destructor with `+0x18` sub-object | translated | medium | `805008c0_FUN_805008c0.c` |
| `0x80500C70` | `refresh_controller_hint` | profile tables, pane flags, animations | translated | medium | `80500c70_FUN_80500c70.c` |
| `0x80501050` | `state_enter_open_wait` | enter `OpenAnimeEndWait` | translated | high | `80501050_FUN_80501050.c` |
| `0x80501160` | `state_poll_open_wait` | wait, then button-on-stage | translated | high | `80501160_FUN_80501160.c` |
| `0x805011D0` | `state_enter_button_on` | play animations 4/5/6 | translated | high | `805011d0_FUN_805011d0.c` |
| `0x80501230` | `state_poll_button_on` | wait, then button-change | translated | high | `80501230_FUN_80501230.c` |
| `0x805012B0` | `state_enter_button_change` | previous/current button anims | translated | high | `805012b0_FUN_805012b0.c` |
| `0x80501320` | `state_poll_button_change` | wait, then select-wait | translated | high | `80501320_FUN_80501320.c` |
| `0x80501390` | `state_enter_select` | idle animation, hide focused pane | translated | high | `80501390_FUN_80501390.c` |
| `0x80501400` | `state_poll_select` | confirm/cancel routing | translated | high | `80501400_FUN_80501400.c` |
| `0x80501530` | `state_enter_hit` | hit animation and event `0x79` | translated | high | `80501530_FUN_80501530.c` |
| `0x805015C0` | `state_poll_hit` | wait, then close or hit-after | translated | high | `805015c0_FUN_805015c0.c` |
| `0x80501660` | `state_enter_close` | close animation and event `0x7A` | translated | high | `80501660_FUN_80501660.c` |
| `0x80501700` | `state_poll_close` | wait, then `InitWait` | translated | high | `80501700_FUN_80501700.c` |
| `0x80501920` | `fn_80501920` | vtable forwarder, hidden `r12 = +0x18` | partial | medium | `80501920_FUN_80501920.c` |
| `0x80501950` | `fn_80501950` | vtable forwarder, hidden `r12 = +0x0C` | partial | medium | `80501950_FUN_80501950.c` |
| `0x80501980` | `register_yes_no_states` | build and link eight descriptors | translated | high | `80501980_FUN_80501980.c` |
| `0x80501FB0` | `state_cleanup` | base destructor, optional free | translated | high | `80501fb0_FUN_80501fb0.c` |

## External calls

All 26 distinct non-prologue `main.dol` call targets are marked by address in
the Rust source. Only `0x8015F7C0` has a documented name (`dStateBase_ctor` in
`docs/SYMBOLS.md`); the others are described by the call shape the REL exposes,
not by an invented Nintendo name.

| address | observed role | status |
|---|---|---|
| `0x8006A990` | no-argument helper on the confirm path | unknown |
| `0x8006C420` | base constructor; writes vtable at `+0x60` | observed |
| `0x800B3B60` | animation binding for profile `6` | unknown |
| `0x800B44D0` | pane show/hide helper (`pane, 0, 0`) | inferred |
| `0x800C8950` | embedded `lyt` layout constructor | observed |
| `0x800C8CB0` | archive/binding probe | unknown |
| `0x800C8F20` | binds two named animations | inferred |
| `0x800C8FB0` | binds three named animations | inferred |
| `0x800C90F0` | binds three pane names plus table | inferred |
| `0x800C92F0` | starts one layout animation by id | inferred |
| `0x800C9380` | clears one layout animation flag | inferred |
| `0x800C93D0` | layout animation helper (id, `0`) | unknown |
| `0x800C9500` | stops/resets all layout animations | inferred |
| `0x800C9610` | reads layout animation flag at index `-1` | inferred |
| `0x800C9640` | non-zero while a layout animation runs | inferred |
| `0x800C9A60` | starts an animation at a frame id | unknown |
| `0x800CDC60` | returns global word `0x80359CB4` (clock?) | unknown |
| `0x8015F7C0` | `dStateBase_ctor` (documented) | named |
| `0x8015F800` | base-destructor wrapper (`ptr, 0`) | observed |
| `0x8015F9F0` | destructor wrapper for nested object | observed |
| `0x8015FC90` | state-manager constructor; writes vtable | inferred |
| `0x80160A20` | initializer-table runner | unknown |
| `0x801628C0` | allocator (size, zero-filled) | observed |
| `0x80195380` | event/sound helper `(manager, id, flag)` | inferred |
| `0x802B9280` | deallocation wrapper (`FUN_802b8f70(ptr, 0)`) | observed |
| `0x802DCBFC` | unresolved indirect-jump stub (hidden `r12`) | unknown |

`0x802DCD74` (`__savegprlr_27`) and `0x802DCDC0` (`__restgprlr_27`) bracket
`register_yes_no_states`; they only save/restore registers and are documented
in the Rust function instead of called.

## Referenced text addresses outside the export

### State seed callbacks (12)

These addresses are linked from the seed table at `0x805035D0` but are not in
`functions.tsv`. They are the missing halves of the state machine.

| state | `+0x0C` slot | `+0x18` slot | `+0x24` slot |
|---|---|---|---|
| `InitWait` (`0x80503918`) | `0x80500FE0` | `0x80501000` | `0x80501040` |
| `OpenAnimeEndWait` (`0x80503958`) | `0x80501050` | `0x80501160` | `0x805011C0` |
| `ButtonOnStageAnimeEndWait` (`0x80503998`) | `0x805011D0` | `0x80501230` | `0x80501290` |
| `ButtonChangeAnimeEndWait` (`0x805039D8`) | `0x805012B0` | `0x80501320` | `0x80501380` |
| `SelectWait` (`0x80503A18`) | `0x80501390` | `0x80501400` | `0x80501520` |
| `HitAnimeEndWait` (`0x80503A58`) | `0x80501530` | `0x805015C0` | `0x80501650` |
| `ClouseAnimeEndWait` (`0x80503A98`) | `0x80501660` | `0x80501700` | `0x80501760` |
| `HitAnimeAfterWait` (`0x80503AD8`) | `0x80501770` | `0x80501780` | `0x80501790` |

The rows whose `+0x0C` and `+0x18` slots match the Rust functions above are
translated; the other 12 callbacks are known only by address.

### Vtable-only text addresses (29)

The relocated data references these text addresses, but they have no
`functions.tsv` row. They are probably small constructors, adjustors or
unrecognised functions inside gaps. The list has 29 distinct addresses; one of
them, `0x80501040`, is also a seed callback listed above.

| group | addresses |
|---|---|
| controller-information vtable `0x80502F60` | `0x80500290`, `0x80500300`, `0x80500460`, `0x80500570`, `0x805005B0`, `0x805005C0` |
| yes/no window vtable `0x80503470` | `0x80500930`, `0x805009C0`, `0x80500B50`, `0x80500C00`, `0x80500C60`, `0x80501880` |
| interface vtables `0x805034C0` / `0x805034F0` | `0x80500BF0`, `0x80501030`, `0x80501890`, `0x805018A0`, `0x805018B0`, `0x805018C0`, `0x805018D0`, `0x805018E0` |
| interface vtables `0x80503520` / `0x80503538` / `0x80503580` | `0x80501040`, `0x805017A0`, `0x805017B0`, `0x80501810`, `0x80501820`, `0x80501840`, `0x80501860` |
| state vtable `0x80503850` | `0x805018F0`, `0x80502010` |

## Data tables

| address | contents | use |
|---|---|---|
| `0x805020B0` | 472-byte word table | passed to `0x800C90F0` |
| `0x805020C0` | 24-byte word table | passed to `0x800C8F20` |
| `0x80502138` | 29 x `u32` animation/frame ids | `refresh_controller_hint` |
| `0x805021AC` | 29 x 3 `u8` pane-visibility flags | three button panes |
| `0x80502204` | 29 x `u32` default button (0..2) | focused-button word |
| `0x80502288` | large relocated pointer block | installed at `0x8042A3B8` |
| `0x80503384` | binding table, first word `2` | profile index `6` |
| `0x805035D0` | 8 x 9-word state seeds | `register_yes_no_states` |
| `0x80503850` | state vtable | every descriptor |

The raw REL stores zero placeholders at pointer sites; the relocated ELF has
the applied values. Any future extraction of these tables must use the ELF (or
apply relocations), not the raw `.rel` bytes.

## Evidence notes

* The Ghidra export is the primary evidence. `functions.tsv` has 36 rows and
  every row is accounted for above.
* The strings file names exactly one class, `dYesNoWindow_c`, and eight
  `StateID_*` labels. There are no actor/profile descriptor strings in this
  REL, so the module header and this coverage doc describe the recovered UI
  role rather than the lane label "actor descriptors".
* Main-constructor arguments dropped by Ghidra were recovered from the
  `main.dol` C exports where available (`0x8006C420`, `0x801628C0`,
  `0x8015F800`, `0x8015F9F0`, `0x8015FC90`, `0x802B9280`, `0x802DCBFC`).
  Where no signature exists, the Rust doc says `unknown:` instead of guessing.
* The hidden `r12` operand in `0x80501920` / `0x80501950` is visible in the
  disassembly (`addi r12,r5,0x18` and `addi r12,r5,0x0c`) but cannot be passed
  through `platform::call3`. Those two rows are therefore `partial`.

## Uncertainties

1. **No runtime base.** Every REL-internal address depends on the synthetic
   `0x80500000`. Do not compare them with addresses from a running game.
2. **Twelve state callbacks are outside the export.** The descriptors are
   complete, but the `InitWait` enter/poll logic and all `+0x24` (third-slot)
   callbacks remain disassembly-only.
3. **Nine-argument seed semantics.** The seed words `{0, -1, callback}` are
   copied faithfully, but the meaning of the leading `0` and `-1` is not
   proven. They may be timers, flags or state ids.
4. **Pane identity.** `+0x244..+0x280` are pane pointers, but the `.brlyt`
   name-to-offset mapping is not reconstructed. The docs call them by offset.
5. **Profile index semantics.** The 29-entry tables strongly suggest a
   per-profile question/button configuration, but no caller sets `+0x28C` in
   this REL; the setter is outside the export.
6. **Owner classes for the seven destructor stubs.** The bodies are identical;
   which vtable slot owns each one is not recovered.
7. **Hidden forwarder target.** `0x802DCBFC` is an unresolved indirect jump;
   `fn_80501920` and `fn_80501950` cannot be completed until it is resolved.

## Next targets

Prioritised:

1. **Recover the 12 seed callbacks.** Start with the `InitWait` triple
   (`0x80500FE0`, `0x80501000`, `0x80501040`) and the `HitAnimeAfterWait` triple
   (`0x80501770`, `0x80501780`, `0x80501790`), then the third-slot callbacks
   `0x805011C0`, `0x80501290`, `0x80501380`, `0x80501520`, `0x80501650`,
   `0x80501760`. These complete the state machine.
2. **Resolve the 29 vtable-only addresses.** Most are likely adjustor thunks or
   small constructors; add them to the export before translating.
3. **Name the external calls from `main.dol`.** The existing
   `work/ghidra/main-gb/export` already contains `0x800C*`, `0x8015F*` and
   `0x8019*` bodies; no new Ghidra import is needed. Prioritise
   `0x800C92F0`, `0x800C9500`, `0x800C9610`, `0x800C9640` and `0x80195380`,
   which determine the state timing and event semantics.
4. **Reconstruct the lookup tables as Rust data.** The 29-entry animation,
   pane-flag and default-button tables can become typed arrays once the
   `+0x28C` setter is known. Until then, the Rust module reads them by address.
5. **Map panes from the `.brlyt`.** The names are already in
   `work/rels/d_profileNP.strings.txt`; pair them with the pointer fields
   `+0x244..+0x280` to improve the field table.
6. **Resolve `0x802DCBFC`.** This is the only blocker for the two `partial`
   rows; it is an indirect jump whose table is not recovered.
7. **Confirm the real REL base via the OSLink lane.** The loader analysis
   (`runtime::link`) is the only way to replace `0x80500000` with a runtime
   address.
