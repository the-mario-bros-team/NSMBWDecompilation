//! Boot scene and save banner/icon sequence (`dScBoot_c`).
//!
//! # What lives here
//!
//! This module is the semantic home for the boot scene that runs before the
//! title/menu: the save banner/icon loading helpers and the first
//! `dScBoot_c` state handlers. Twenty-nine functions are translated here: the
//! twelve described in the table below and the seventeen untabled boot-task
//! bodies added in wave 228 (see the slot map at the bottom of this file).
//!
//! | address | Rust function | role |
//! |---|---|---|
//! | `0x8015BB60` | [`fn_8015bb60`] | boolean controller-state query |
//! | `0x8015BC40` | [`fn_8015bc40`] | create resource `0x26C` into `+0xF0` |
//! | `0x8015BDD0` | [`fn_8015bdd0`] | poll controller readiness and latch `+0x102` |
//! | `0x8015BE60` | [`load_save_banner`] | mount the US save-banner resource |
//! | `0x8015C010` | [`fn_8015c010`] | create resources `0x2C0`/`0x2CF` |
//! | `0x8015C130` | [`dsc_boot_construct`] | construct the boot scene and its two state managers |
//! | `0x8015C5A0` | [`dsc_boot_update`] | run the task list and tick both scene interfaces |
//! | `0x8015C630` | [`fn_8015c630`] | rebind the save/controller scale |
//! | `0x8015C710` | [`dsc_boot_begin_reset`] | enter the reset/wait phase |
//! | `0x8015C770` | [`dsc_boot_update_reset`] | wait for input/controller state, then fade or finish |
//! | `0x8015C8D0` | [`dsc_boot_update_controller_info`] | route controller-information states |
//! | `0x8015D480` | [`load_save_icon`] | load `save_icon.bti` or enter the error fade |
//!
//! # Original address range
//!
//! Coverage band H, `0x8015B840..0x8015DDD0` (59 entries, 10,956 bytes). The
//! band also contains the 4,716-byte `dScBoot_c` state-descriptor initialiser at
//! `0x8015DDD0`, which registers the 24 `StateID_*` names; the handlers below
//! are the observable bodies between that initialiser and the save banner.
//! The range belongs to the address lane `0x80140000..0x80180000`, documented
//! in `docs/COVERAGE_r80140_80180.md`; this module's coverage is tracked in
//! `docs/COVERAGE_DISPLAY.md`.
//!
//! # Why it exists
//!
//! The boot scene is a scene/state machine, not a UI layout. It reuses the same
//! descriptor pattern as [`crate::engine::display`] and
//! [`crate::game::yoshi`], so keeping it in its own file makes the boot/save
//! path discoverable without reading the rest of the 0x8015 address lane.
//!
//! # Boot-scene state descriptors (observed in `0x8015DDD0`)
//!
//! | descriptor | `StateID_*` | used by |
//! |---|---|---|
//! | `0x80377408` | `ResetWait` | [`dsc_boot_construct`], [`dsc_boot_update_reset`] |
//! | `0x80377448` | `ResetFadeOut` | not translated yet |
//! | `0x80377488` | `ResetFadeIn` | [`dsc_boot_update_reset`] |
//! | `0x803774C8` | `FadeOutWait` | [`dsc_boot_construct`] |
//! | `0x80377548` | `WiiStrapDispEnd` | [`dsc_boot_update_controller_info`] |
//! | `0x80377608` | `ControllerInfo...` | [`dsc_boot_update_controller_info`] |
//! | `0x80377688` | `ControllerInfo...` | [`dsc_boot_update_controller_info`] |
//! | `0x803776C8` | `NandCommandEnd` | [`dsc_boot_update_controller_info`] |
//! | `0x80377788` | `CreateFile` | [`load_save_icon`] |
//! | `0x80377808` | `GoToErrorFade...` | [`load_save_icon`] |
//! | `0x80377948` | `ProcEnd` | [`dsc_boot_update_reset`] |
//!
//! The `...` names are truncated in the export's string symbols; they are not
//! invented here.
//!
//! # Object layout used here (observed)
//!
//! Offsets are relative to the `dScBoot_c` object.
//!
//! | offset | width | meaning | confidence |
//! |---|---|---|---|
//! | `+0x060` | word | scene vtable pointer | observed |
//! | `+0x070` | word | global scene-state pointer stored at construction | inferred |
//! | `+0x074` | word | scene-state interface; slot `+0x18` changes state | observed |
//! | `+0x078/+0x07C` | word | interfaces passed to the first state manager | observed |
//! | `+0x08C` | object | state manager initialised with `FadeOutWait` | observed |
//! | `+0x0B0` | word | second state interface | observed |
//! | `+0x0B4/+0x0B8` | word | interfaces passed to the second state manager | observed |
//! | `+0x0C8` | object | state manager initialised with `ResetWait` | observed |
//! | `+0x0F0/+0x0F4` | word | resource handles created by the descriptor initialiser | inferred |
//! | `+0x0FC` | word | save/controller object used by the reset path | inferred |
//! | `+0x101` | byte | "controller ready" latch | observed |
//! | `+0x103` | byte | "reset started" flag | observed |
//! | `+0x104` | byte | controller/timer value returned by `0x800B0A90` | observed |
//! | `+0x105` | byte | "finish instead of fade" flag | observed |
//! | `+0x108/+0x10C` | word | fade counters (`0x4B0`, `0x3C` in other states) | inferred |
//!
//! # Dependencies
//!
//! [`crate::platform`] for the volatile loads/stores and indirect calls. The
//! scene/resource helpers (`0x800DF840`, `0x800DF180`, `0x800E0450`,
//! `0x800E19B0`) and the layout/scene queries are called by address because
//! their reconstructed homes are other lanes. `0x8015FC90` is the state-manager
//! constructor documented in [`crate::engine::display`].
//!
//! # Start here
//!
//! [`dsc_boot_construct`] shows the object layout and the two state managers.
//! Then read [`dsc_boot_update`] for the per-frame task list and
//! [`dsc_boot_update_reset`] for the transition into the save/error path.
//! [`load_save_banner`] and [`load_save_icon`] are the visible save-file half.
//!
//! # Glossary
//!
//! * **boot scene** - the first scene after boot; it shows the save banner and
//!   hands control to the title/menu.
//! * **save banner** - the banner/icon block read from the save file and drawn
//!   during the boot scene.
//! * **state descriptor** - a 0x40-byte record naming one scene state and its
//!   callbacks; `dScBoot_c` registers 24 of them.
//! * **state manager** - the object built by `0x8015FC90`; it owns the current
//!   descriptor and dispatches the state callbacks.
//! * **NAND** - the Wii's internal flash filesystem; the save/error states
//!   check/create the save file there.
//! * **SDA** - the PowerPC small-data area based at `r13 = 0x8042F680`;
//!   Ghidra prints it as `unaff_r13`.
//!
//! # Confidence summary
//!
//! Six bodies are `high`. [`dsc_boot_update_controller_info`] is `medium`
//! because its three event queries pass through two virtual interfaces whose
//! return values are only tested for zero; the event addresses themselves are
//! exact. [`load_save_icon`] is `medium` because one archive path is passed
//! through `0x800DF180` whose argument roles are inferred from neighbouring
//! call sites.

use crate::platform;

// ---------------------------------------------------------------------------
// Small-data anchors
// ---------------------------------------------------------------------------

/// PowerPC `r13` small-data base, set by `__init_registers` at `0x80004244`.
const SDA_BASE: usize = 0x8042_F680;

/// PowerPC `r2` (small-data-2) base. The boot helpers below read one float
/// through it; every access is expressed as `SDA2_BASE - offset`.
const SDA2_BASE: usize = 0x8043_3080;

/// `r13 - 0x5648` -> `0x8042A038`: pointer to the archive/resource manager.
///
/// observed: the same global is used by the stage loader (`src/game/stage.rs`)
/// and the dokan actor loader; its `+4` field is the archive handle.
const G_ARCHIVE_MANAGER: usize = SDA_BASE - 0x5648;

/// `r13 - 0x56C8` -> `0x80429FB8`: save manager; `+0x78` is the "save file
/// exists" slot tested by [`load_save_icon`].
///
/// `// inferred:` role from the `+0x78` test and the adjacent save icon load.
const G_SAVE_MANAGER: usize = SDA_BASE - 0x56C8;

/// `r13 - 0x5640` -> `0x8042A040`: object polled by `0x800E0450` before the
/// save icon load.
///
/// `// inferred:` a load/save worker manager; the poll returns non-zero while
/// the previous operation is still busy.
const G_LOAD_MANAGER: usize = SDA_BASE - 0x5640;

/// `r13 - 0x53A8` -> `0x8042A2D8`: object whose `+0xB00` counter is cleared by
/// [`dsc_boot_update_reset`].
///
/// `// unknown:` the object's owner is not recovered; it is only tested for a
/// positive counter and passed to `0x8010E490`.
const G_SAVE_PROGRESS: usize = SDA_BASE - 0x53A8;

/// `r13 - 0x53B8` -> `0x8042A2C8`: handle passed to `0x8010C780` when the boot
/// scene finishes.
///
/// `// unknown:` a BSS handle; the value is written at runtime.
const G_SAVE_BANNER_HANDLE: usize = SDA_BASE - 0x53B8;

/// `r13 - 0x5328` -> `0x8042A358`: task-list header passed to `0x8015F620`.
///
/// observed: `0x8015F600` initialises this header and `0x8015F620` runs the
/// table until a non-`1` result.
const G_TASK_LIST: usize = SDA_BASE - 0x5328;

/// `r13 - 0x6F44` -> `0x8042873C`: byte cleared when the reset phase starts.
///
/// `// unknown:` owner not named in the export.
const G_RESET_FLAG: usize = SDA_BASE - 0x6F44;

/// `r13 - 0x5340` -> `0x8042A340`: global pointer to the live `dScBoot_c`.
const G_BOOT_OBJECT: usize = SDA_BASE - 0x5340;

/// `r13 - 0x5338` -> `0x8042A348`: scene-state pointer stored at `+0x70`.
const G_SCENE_STATE: usize = SDA_BASE - 0x5338;

/// `r13 - 0x5240` -> `0x8042A440`: player/controller manager pointer.
///
/// observed: the same global is queried by the 2-D effects in
/// [`crate::engine::display`]; vtable slot `+0x10` returns the player count.
const G_PLAYER_MANAGER: usize = SDA_BASE - 0x5240;

// ---------------------------------------------------------------------------
// Boot state descriptors (observed in `0x8015DDD0`)
// ---------------------------------------------------------------------------

/// `0x80377408`: `dScBoot_c::StateID_ResetWait`.
const BOOT_STATE_RESET_WAIT: u32 = 0x8037_7408;

/// `0x80377488`: `dScBoot_c::StateID_ResetFadeIn`.
const BOOT_STATE_RESET_FADE_IN: u32 = 0x8037_7488;

/// `0x803774C8`: `dScBoot_c::StateID_FadeOutWait`.
const BOOT_STATE_FADE_OUT_WAIT: u32 = 0x8037_74C8;

/// `0x80377548`: `dScBoot_c::StateID_WiiStrapDispEnd`.
const BOOT_STATE_WII_STRAP_DISP_END: u32 = 0x8037_7548;

/// `0x80377608`: one of the four `dScBoot_c::StateID_ControllerInfo...`
/// descriptors (the second in the initialiser order).
const BOOT_STATE_CONTROLLER_INFO_2: u32 = 0x8037_7608;

/// `0x80377688`: the fourth `dScBoot_c::StateID_ControllerInfo...`
/// descriptor.
const BOOT_STATE_CONTROLLER_INFO_4: u32 = 0x8037_7688;

/// `0x803776C8`: `dScBoot_c::StateID_NandCommandEnd`.
const BOOT_STATE_NAND_COMMAND_END: u32 = 0x8037_76C8;

/// `0x80377788`: `dScBoot_c::StateID_CreateFile`.
const BOOT_STATE_CREATE_FILE: u32 = 0x8037_7788;

/// `0x80377808`: first `dScBoot_c::StateID_GoToErrorFade...` descriptor.
const BOOT_STATE_GOTO_ERROR_FADE: u32 = 0x8037_7808;

/// `0x80377948`: `dScBoot_c::StateID_ProcEnd`.
const BOOT_STATE_PROC_END: u32 = 0x8037_7948;

// ---------------------------------------------------------------------------
// Vtable/interface words observed in the constructor `0x8015C130`
// ---------------------------------------------------------------------------

/// `0x80328180`: `dScBoot_c` base vtable written at `+0x60`.
const VTABLE_BOOT_BASE: u32 = 0x8032_8180;

/// `0x803281D0`: state-interface vtable written at `+0x74` and `+0xB0` after
/// the state managers are built.
const VTABLE_BOOT_IFACE: u32 = 0x8032_81D0;

/// `0x80328200`: first interface table stored at `+0x74`/`+0xB0` during
/// construction.
const VTABLE_BOOT_IFACE_200: u32 = 0x8032_8200;

/// `0x80328230`: interface table stored at `+0x7C`/`+0xB8`.
const VTABLE_BOOT_IFACE_230: u32 = 0x8032_8230;

/// `0x80328248`: interface table stored at `+0x80`/`+0xBC`.
const VTABLE_BOOT_IFACE_248: u32 = 0x8032_8248;

/// `0x802FEAF8`: shared interface table stored at `+0x78`/`+0xB4`.
const VTABLE_SHARED_IFACE: u32 = 0x802F_EAF8;

// ---------------------------------------------------------------------------
// Named callees (called by address; definitions live outside this module)
// ---------------------------------------------------------------------------

/// `0x800DF840`: mounts an archive resource `(manager, path, resource, 0)`.
const FN_800DF840: usize = 0x800D_F840;

/// `0x800DF180`: loads a file from the archive manager
/// `(handle, resource, file)`; non-zero means success.
const FN_800DF180: usize = 0x800D_F180;

/// `0x800E0450`: returns non-zero while a save/load operation is busy.
const FN_800E0450: usize = 0x800E_0450;

/// `0x800E19B0`: base scene constructor called before the boot vtables are
/// written.
const FN_800E19B0: usize = 0x800E_19B0;

/// `0x8015FC90`: state-manager constructor with four arguments.
const FN_8015FC90: usize = 0x8015_FC90;

/// `0x8015F620`: runs the global task list until a task returns non-`1`.
const FN_8015F620: usize = 0x8015_F620;

/// `0x8015CB20`: boot-state helper called from the reset path.
///
/// `// unknown:` its body is not translated in this pass; the call is kept by
/// address so no behaviour is invented.
const FN_8015CB20: usize = 0x8015_CB20;

/// `0x80107F10`: returns the current system/controller object.
const FN_80107F10: usize = 0x8010_7F10;

/// `0x80108230`: advances the system object into its next controller phase.
const FN_80108230: usize = 0x8010_8230;

/// `0x80108330`: returns non-zero when the system object is ready.
const FN_80108330: usize = 0x8010_8330;

/// `0x800B0DB0`: polls the save/controller state; `0` requests a fresh poll.
const FN_800B0DB0: usize = 0x800B_0DB0;

/// `0x800B0A90`: returns the controller/timer value for a `0x1E` delay.
const FN_800B0A90: usize = 0x800B_0A90;

/// `0x8010E490`: resets the save-progress object `(object, 0)`.
const FN_8010E490: usize = 0x8010_E490;

/// `0x8010C780`: releases a save-banner/player slot `(handle, 0)`.
const FN_8010C780: usize = 0x8010_C780;

/// `0x803280A0`: `"US/save_banner"` resource path (Ghidra symbol
/// `s__US_save_banner_803280a0`).
const S_US_SAVE_BANNER: usize = 0x8032_80A0;

/// `0x802F5A30`: `"save_banner"` resource name (Ghidra symbol
/// `s_save_banner_802f5a30`).
const S_SAVE_BANNER_802F5A30: usize = 0x802F_5A30;

/// `0x80328170`: `"save_banner"` archive name (Ghidra symbol
/// `s_save_banner_80328170`).
const S_SAVE_BANNER_80328170: usize = 0x8032_8170;

/// `0x80328160`: `"save_icon.bti"` file name.
const S_SAVE_ICON_BTI: usize = 0x8032_8160;

/// `0x8008F620`: controller/input state query. Ghidra prints the callee without
/// parameters; each call site still supplies an id in `r3`.
const FN_8008F620: usize = 0x8008_F620;

/// `0x80162B00`: creates a registered resource from an id and an owner:
/// `(id, owner, 0, 0)`. The last two operands are dead in the callee but are
/// kept because the source passes them.
const FN_80162B00: usize = 0x8016_2B00;

/// `0x8006A760`: polls the controller/save device and returns non-zero when it
/// is ready.
const FN_8006A760: usize = 0x8006_A760;

/// `0x801637D0`: writes one float into two adjacent object fields:
/// `(f64 value, object)`.
const FN_801637D0: usize = 0x8016_37D0;

/// `0x801644D0`: commits the object written by [`FN_801637D0`].
const FN_801644D0: usize = 0x8016_44D0;

/// `r2 - 0x5490` -> `0x8042DBF0`: the one `f32` scale used by
/// [`fn_8015c630`]. Reading the absolute address preserves the operand instead
/// of inventing a float literal.
const F32_SDA2_5490: usize = SDA2_BASE - 0x5490;

// ---------------------------------------------------------------------------
// Shared memory and call helpers
// ---------------------------------------------------------------------------

/// Reads the big-endian word at `base + offset`.
#[inline]
unsafe fn read_word(base: usize, offset: usize) -> u32 {
    platform::read_u32(base + offset)
}

/// Writes the big-endian word `value` at `base + offset`.
#[inline]
unsafe fn write_word(base: usize, offset: usize, value: u32) {
    platform::write_u32(base + offset, value);
}

/// Reads the big-endian signed word at `base + offset`.
#[inline]
unsafe fn read_i32(base: usize, offset: usize) -> i32 {
    platform::read_i32(base + offset)
}

/// Reads the byte at `base + offset`.
#[inline]
unsafe fn read_byte(base: usize, offset: usize) -> u8 {
    platform::read_u8(base + offset)
}

/// Writes the byte `value` at `base + offset`.
#[inline]
unsafe fn write_byte(base: usize, offset: usize, value: u8) {
    platform::write_u8(base + offset, value);
}

/// Reads the big-endian word at an absolute global address.
#[inline]
unsafe fn read_global_word(addr: usize) -> u32 {
    platform::read_u32(addr)
}

/// Writes the big-endian word `value` at an absolute global address.
#[inline]
unsafe fn write_global_word(addr: usize, value: u32) {
    platform::write_u32(addr, value);
}

/// Writes the byte `value` at an absolute global address.
#[inline]
unsafe fn write_global_byte(addr: usize, value: u8) {
    platform::write_u8(addr, value);
}

/// Reads a big-endian `f32` at an absolute address.
#[inline]
unsafe fn read_f32_abs(addr: usize) -> f32 {
    f32::from_bits(platform::read_u32(addr))
}

/// Calls a function with the EABI prototype `(f64, u32)`.
///
/// # Safety
/// `addr` must point at a function with this prototype.
#[inline]
unsafe fn call_f64_u32(addr: usize, a: f64, b: u32) {
    let f: extern "C" fn(f64, u32) = core::mem::transmute(addr);
    f(a, b)
}

/// Calls a no-argument function that returns `u32`.
#[inline]
unsafe fn call0_r(addr: usize) -> u32 {
    let f: extern "C" fn() -> u32 = core::mem::transmute(addr);
    f()
}

/// Calls the virtual method at `this_arg`'s vtable slot `slot`.
#[inline]
unsafe fn vcall0(this_arg: usize, slot: usize) -> u32 {
    let vtable = read_word(this_arg, 0) as usize;
    let entry = read_word(vtable, slot) as usize;
    platform::call1(entry, this_arg as u32)
}

/// [`vcall0`] with one integer argument (`r4`).
#[inline]
unsafe fn vcall1(this_arg: usize, slot: usize, arg: u32) -> u32 {
    let vtable = read_word(this_arg, 0) as usize;
    let entry = read_word(vtable, slot) as usize;
    platform::call2(entry, this_arg as u32, arg)
}

/// `true` when `value > 0` for the Ghidra idiom
/// `countLeadingZeros(value - 1) >> 5 != 0`.
#[inline]
fn positive_test(value: u32) -> bool {
    value.wrapping_sub(1).leading_zeros() >> 5 != 0
}

/// Changes the state of the interface at `iface` through vtable slot `+0x18`.
#[inline]
unsafe fn change_state(iface: usize, descriptor: u32) {
    let _ = vcall1(iface, 0x18, descriptor);
}

// ===========================================================================
// Save banner / icon
// ===========================================================================

/// Mounts the US save-banner resource through the archive manager.
///
/// # Original
/// - Ghidra: `FUN_8015be60` at `0x8015BE60` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015be60_FUN_8015be60.c`
///
/// # Returns
/// Always `1`, matching the original `return 1`.
///
/// # Side effects
/// Calls `0x800DF840(*0x8042A038, "US/save_banner", "save_banner", 0)`.
///
/// # Safety
/// The archive-manager global `0x8042A038` must point to a live manager.
///
/// # Confidence
/// high - one call with four observed operands and a constant return.
pub unsafe fn load_save_banner() -> u32 {
    let _ = platform::call4(
        FN_800DF840,
        read_global_word(G_ARCHIVE_MANAGER),
        S_US_SAVE_BANNER as u32,
        S_SAVE_BANNER_802F5A30 as u32,
        0,
    );
    1
}

/// Loads `save_icon.bti`, or routes the boot scene to the error fade.
///
/// # Original
/// - Ghidra: `FUN_8015d480` at `0x8015D480` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015d480_FUN_8015d480.c`
///
/// # Arguments
/// * `boot` - the live `dScBoot_c` object.
///
/// # Side effects
/// Returns immediately while `0x800E0450` reports a busy save manager. When the
/// save manager's `+0x78` slot is zero, tries
/// `0x800DF180(archive+4, "save_banner", "save_icon.bti")`; success requests
/// `CreateFile`, failure is ignored. When `+0x78` is non-zero, requests the
/// `GoToErrorFade` descriptor.
///
/// # Safety
/// `boot + 0x74` must be a live state interface; the archive and save-manager
/// globals must point to live objects.
///
/// # Confidence
/// medium - the flow and strings are exact; `0x800DF180`'s first argument is
/// `archive_manager + 4`, inferred from the stage loader's use of the same
/// global.
pub unsafe fn load_save_icon(boot: usize) {
    let busy = platform::call1(FN_800E0450, read_global_word(G_LOAD_MANAGER));
    if busy != 0 {
        return;
    }

    let save_manager = read_global_word(G_SAVE_MANAGER) as usize;
    if read_word(save_manager, 0x78) == 0 {
        let loaded = platform::call3(
            FN_800DF180,
            read_global_word(G_ARCHIVE_MANAGER).wrapping_add(4),
            S_SAVE_BANNER_80328170 as u32,
            S_SAVE_ICON_BTI as u32,
        );
        if loaded != 0 {
            change_state(boot + 0x74, BOOT_STATE_CREATE_FILE);
        }
    } else {
        change_state(boot + 0x74, BOOT_STATE_GOTO_ERROR_FADE);
    }
}

// ===========================================================================
// dScBoot_c lifecycle
// ===========================================================================

/// Constructs the boot scene and its two state managers.
///
/// # Original
/// - Ghidra: `FUN_8015c130` at `0x8015C130` (228 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015c130_FUN_8015c130.c`
/// - observed: the entry calls `FUN_802dcd6c` (`__savegprlr_25`), so the
///   decompiler prints `void`; `r3` is the first argument and survives the
///   helper. The Rust signature makes it explicit.
///
/// # Arguments
/// * `boot` - storage for the `dScBoot_c` object. The caller owns the memory.
///
/// # Side effects
/// Calls the base scene constructor `0x800E19B0`, writes the two interface
/// groups, builds state managers at `+0x8C` (`FadeOutWait`) and `+0xC8`
/// (`ResetWait`), stores the scene-state global at `+0x70`, and publishes the
/// object at `0x8042A340`.
///
/// # Safety
/// `boot` must point to at least `0x110` writable bytes; the constructors must
/// be callable on the target.
///
/// # Confidence
/// high - every store and both state-manager argument lists are visible.
pub unsafe fn dsc_boot_construct(boot: usize) {
    // observed: iVar1 = FUN_802dcd6c(); FUN_800e19b0(); -- r3 is the object.
    let _ = platform::call1(FN_800E19B0, boot as u32);

    write_word(boot, 0x60, VTABLE_BOOT_BASE);

    // First interface group at +0x74 and its state manager at +0x8C.
    write_word(boot, 0x74, VTABLE_BOOT_IFACE_200);
    write_word(boot, 0x78, VTABLE_SHARED_IFACE);
    write_word(boot, 0x7C, VTABLE_BOOT_IFACE_230);
    write_word(boot, 0x80, VTABLE_BOOT_IFACE_248);
    write_word(boot, 0x84, boot as u32);
    write_word(boot, 0x88, 0);
    let _ = platform::call4(
        FN_8015FC90,
        (boot + 0x8C) as u32,
        (boot + 0x78) as u32,
        (boot + 0x7C) as u32,
        BOOT_STATE_FADE_OUT_WAIT,
    );

    // Second interface group at +0xB0 and its state manager at +0xC8.
    write_word(boot, 0x74, VTABLE_BOOT_IFACE);
    write_word(boot, 0xB0, VTABLE_BOOT_IFACE_200);
    write_word(boot, 0xB4, VTABLE_SHARED_IFACE);
    write_word(boot, 0xB8, VTABLE_BOOT_IFACE_230);
    write_word(boot, 0xBC, VTABLE_BOOT_IFACE_248);
    write_word(boot, 0xC0, boot as u32);
    write_word(boot, 0xC4, 0);
    let _ = platform::call4(
        FN_8015FC90,
        (boot + 0xC8) as u32,
        (boot + 0xB4) as u32,
        (boot + 0xB8) as u32,
        BOOT_STATE_RESET_WAIT,
    );

    write_word(boot, 0xB0, VTABLE_BOOT_IFACE);
    write_word(boot, 0x70, G_SCENE_STATE as u32);
    write_global_word(G_BOOT_OBJECT, boot as u32);
}

/// Per-frame boot-scene update: run the task list and tick both interfaces.
///
/// # Original
/// - Ghidra: `FUN_8015c5a0` at `0x8015C5A0` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015c5a0_FUN_8015c5a0.c`
///
/// # Arguments
/// * `boot` - the live `dScBoot_c` object.
///
/// # Returns
/// Always `1`, matching the original.
///
/// # Side effects
/// Runs the global task list, ticks the interface at `+0xB0`, and ticks the
/// interface at `+0x74` while byte `+0x103` is clear.
///
/// # Safety
/// `boot + 0x74` and `boot + 0xB0` must be live state interfaces and the task
/// list at `0x8042A358` must be initialised.
///
/// # Confidence
/// high - the task-list call and both virtual slots are explicit.
pub unsafe fn dsc_boot_update(boot: usize) -> u32 {
    // observed: FUN_8015f620(unaff_r13 - 0x5328, param_1);
    let _ = platform::call2(FN_8015F620, G_TASK_LIST as u32, boot as u32);
    let _ = vcall0(boot + 0xB0, 0x10);
    if read_byte(boot, 0x103) == 0 {
        let _ = vcall0(boot + 0x74, 0x10);
    }
    1
}

/// Starts the reset phase: sets the flag, clears the global, and polls.
///
/// # Original
/// - Ghidra: `FUN_8015c710` at `0x8015C710` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015c710_FUN_8015c710.c`
///
/// # Arguments
/// * `boot` - the live `dScBoot_c` object.
///
/// # Side effects
/// Sets byte `+0x103`, clears the byte at `0x8042873C`, calls
/// `0x800B0DB0(0)`, stores the `0x800B0A90(0x1E)` result at `+0x104`, and
/// clears `+0x101`.
///
/// # Safety
/// `boot` must be live; the save/controller globals used by `0x800B0DB0` and
/// `0x800B0A90` must be initialised.
///
/// # Confidence
/// high - four stores and two calls, all observed.
pub unsafe fn dsc_boot_begin_reset(boot: usize) {
    // +0x103: reset started; +0x101: controller-ready latch.
    write_byte(boot, 0x103, 1);
    write_global_byte(G_RESET_FLAG, 0);
    let _ = platform::call1(FN_800B0DB0, 0);
    // 0x1E is the 30-frame poll delay passed to the controller helper.
    let polled = platform::call1(FN_800B0A90, 0x1E);
    write_byte(boot, 0x104, polled as u8);
    write_byte(boot, 0x101, 0);
}

/// Reset-phase update: waits for the controller, then fades or finishes.
///
/// # Original
/// - Ghidra: `FUN_8015c770` at `0x8015C770` (344 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015c770_FUN_8015c770.c`
///
/// # Arguments
/// * `boot` - the live `dScBoot_c` object.
///
/// # Side effects
/// * While `+0x104` is zero, re-polls `0x800B0A90(0x1E)`.
/// * Once polled, waits for `0x80108330` to report the controller ready and
///   sets `+0x101`.
/// * When the player manager has a player, `+0xFC` is live and its `+0x208`
///   byte is set: optionally resets the save-progress object, then either
///   requests `ResetFadeIn` (`+0x105 == 0`) or runs `0x8015CB20`, requests
///   `ProcEnd`, clears the save handle and returns to `ResetWait`.
///
/// # Safety
/// `boot + 0xFC` must be a live save/controller object; the player-manager and
/// save-progress globals must be live when the branch is taken.
///
/// # Confidence
/// high - the control flow is complete. The call to `0x8015CB20` is kept by
/// address because its body is not translated in this pass.
pub unsafe fn dsc_boot_update_reset(boot: usize) {
    if read_byte(boot, 0x104) == 0 {
        // 0x1E is the 30-frame poll delay; +0x104 holds the polled value.
        let polled = platform::call1(FN_800B0A90, 0x1E);
        write_byte(boot, 0x104, polled as u8);
        return;
    }

    if read_byte(boot, 0x101) == 0 {
        let system = call0_r(FN_80107F10);
        if platform::call1(FN_80108330, system) != 0 {
            write_byte(boot, 0x101, 1);
        }
    }

    if read_byte(boot, 0x101) == 0 {
        return;
    }

    let manager = read_global_word(G_PLAYER_MANAGER) as usize;
    let players = vcall0(manager, 0x10);
    let save_object = read_word(boot, 0xFC) as usize;

    if positive_test(players)
        && save_object != 0
        && read_byte(save_object, 0x208) != 0
    {
        let progress = read_global_word(G_SAVE_PROGRESS) as usize;
        // +0xB00: pending save-progress counter.
        if read_i32(progress, 0xB00) > 0 {
            let _ = platform::call2(FN_8010E490, progress as u32, 0);
        }

        if read_byte(boot, 0x105) == 0 {
            change_state(boot + 0xB0, BOOT_STATE_RESET_FADE_IN);
        } else {
            // unknown: 0x8015CB20 body is not translated here.
            let _ = platform::call1(FN_8015CB20, boot as u32);
            change_state(boot + 0x74, BOOT_STATE_PROC_END);

            let window = read_word(boot, 0xF4) as usize;
            let _ = platform::call2(
                FN_8010C780,
                read_global_word(G_SAVE_BANNER_HANDLE),
                0,
            );
            // +0x295: window-open flag; +0x209: save/controller busy flag.
            write_byte(window, 0x295, 0);
            write_byte(save_object, 0x209, 0);
            change_state(boot + 0xB0, BOOT_STATE_RESET_WAIT);
        }
    }
}

/// Routes the controller-information states from the two event queries.
///
/// # Original
/// - Ghidra: `FUN_8015c8d0` at `0x8015C8D0` (364 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015c8d0_FUN_8015c8d0.c`
///
/// # Arguments
/// * `boot` - the live `dScBoot_c` object.
///
/// # Side effects
/// Calls `0x80107F10` / `0x80108230`, then asks the interface at `+0x74` for
/// its state object and tests two event ids (`0x80377548`, `0x80377688`). If
/// either is active, clears `+0x101`, updates the save/controller flags and
/// requests `ControllerInfo` or `NandCommandEnd`. Always clears `+0x104` at the
/// end; the early "both events zero" path only clears `+0x104`.
///
/// # Safety
/// `boot + 0x74` must be a live state interface and its returned state object
/// must be live. The save-progress global must be live when the first event is
/// active.
///
/// # Confidence
/// medium - the event addresses and branches are exact, but the two virtual
/// queries return an opaque word that is only tested for zero.
pub unsafe fn dsc_boot_update_controller_info(boot: usize) {
    let system = call0_r(FN_80107F10);
    let _ = platform::call1(FN_80108230, system);

    // observed: first query on the interface at +0x74, vtable slot +0x28,
    // then event slot +0x14 on the returned object.
    let mut iface = vcall0(boot + 0x74, 0x28) as usize;
    let mut event = vcall1(iface, 0x14, BOOT_STATE_WII_STRAP_DISP_END);

    if event == 0 {
        iface = vcall0(boot + 0x74, 0x28) as usize;
        event = vcall1(iface, 0x14, BOOT_STATE_CONTROLLER_INFO_4);
        if event == 0 {
            write_byte(boot, 0x104, 0);
            return;
        }
    }

    write_byte(boot, 0x101, 0);

    if event != 0 {
        let progress = read_global_word(G_SAVE_PROGRESS) as usize;
        let _ = platform::call2(FN_8010E490, progress as u32, 0);
        write_byte(read_word(boot, 0xFC) as usize, 0x209, 1);
        write_byte(read_word(boot, 0xF0) as usize, 0x209, 0);
        change_state(boot + 0x74, BOOT_STATE_CONTROLLER_INFO_2);
    } else {
        write_byte(read_word(boot, 0xFC) as usize, 0x209, 0);
        change_state(boot + 0x74, BOOT_STATE_NAND_COMMAND_END);
    }

    write_word(boot, 0x108, 0);
    write_word(boot, 0x10C, 0);
    write_byte(boot, 0x104, 0);
}

// ===========================================================================
// Save/controller-state helpers
//
// The five bodies below sit between the save-banner loader and the boot-state
// handlers. They are the small queries and resource registrations that the
// state descriptors use; none of them has a jump table or an unresolved
// register.
// ===========================================================================

/// Tests one controller/input state id and returns the boolean as `0`/`1`.
///
/// The original computes `countLeadingZeros(value - 1) >> 5`, which is the
/// compiler's branchless "value > 0" test. The shared [`positive_test`] helper
/// expresses exactly that.
///
/// # Original
/// - Ghidra: `FUN_8015BB60` at `0x8015BB60` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015bb60_FUN_8015bb60.c`
///
/// # Returns
/// `1` when the state query is positive, `0` otherwise.
///
/// # Side effects
/// Calls `0x8008F620(3)`.
///
/// # Safety
/// `0x8008F620` must be readable/callable; it reads an SDA global.
///
/// # Confidence
/// high - the boolean idiom is identical to the module's existing helper. The
/// id `3` is passed by the source even though the callee's export omits the
/// parameter.
pub unsafe fn fn_8015bb60() -> u32 {
    let value = platform::call1(FN_8008F620, 3);
    positive_test(value) as u32
}

/// Creates a resource with id `0x26C` and stores the handle at `+0xF0`.
///
/// # Original
/// - Ghidra: `FUN_8015BC40` at `0x8015BC40` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015bc40_FUN_8015bc40.c`
///
/// # Arguments
/// * `boot` - the `dScBoot_c` object that receives the handle.
///
/// # Returns
/// Always `1`.
///
/// # Side effects
/// Calls `0x80162B00(0x26C, boot, 0, 0)` and writes `+0xF0`.
///
/// # Safety
/// `boot` must be a live object accepted by the resource creator.
///
/// # Confidence
/// high - the id, owner and store are explicit.
pub unsafe fn fn_8015bc40(boot: usize) -> u32 {
    let handle = platform::call4(FN_80162B00, 0x26C, boot as u32, 0, 0);
    write_word(boot, 0xF0, handle);
    1
}

/// Polls the controller/save device and latches success at `+0x102`.
///
/// # Original
/// - Ghidra: `FUN_8015BDD0` at `0x8015BDD0` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015bdd0_FUN_8015bdd0.c`
///
/// # Arguments
/// * `boot` - the `dScBoot_c` object.
///
/// # Returns
/// `1` when the poll succeeds, `0` otherwise.
///
/// # Side effects
/// When the poll succeeds, writes `1` to the byte at `+0x102`.
///
/// # Safety
/// `boot + 0x102` must be writable.
///
/// # Confidence
/// high - one call, one conditional byte store.
pub unsafe fn fn_8015bdd0(boot: usize) -> u32 {
    let ready = call0_r(FN_8006A760);
    if ready != 0 {
        write_byte(boot, 0x102, 1);
    }
    if ready != 0 { 1 } else { 0 }
}

/// Creates the two resources `0x2C0` and `0x2CF` and stores their handles at
/// `+0xF4` and `+0xF8`.
///
/// # Original
/// - Ghidra: `FUN_8015C010` at `0x8015C010` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015c010_FUN_8015c010.c`
///
/// # Arguments
/// * `boot` - the `dScBoot_c` object that receives both handles.
///
/// # Returns
/// Always `1`.
///
/// # Side effects
/// Two `0x80162B00` calls and two word stores.
///
/// # Safety
/// `boot` must be a live object accepted by the resource creator.
///
/// # Confidence
/// high - both ids and both destination fields are explicit.
pub unsafe fn fn_8015c010(boot: usize) -> u32 {
    let first = platform::call4(FN_80162B00, 0x2C0, boot as u32, 0, 0);
    write_word(boot, 0xF4, first);
    let second = platform::call4(FN_80162B00, 0x2CF, boot as u32, 0, 0);
    write_word(boot, 0xF8, second);
    1
}

/// Clears the reset latch and rebinds scale `f32` on the save/controller
/// object reached through `+0xF0`.
///
/// The object pointer is the word at `(*(boot + 0xF0)) + 0x1F4`; the original
/// passes its first word to `0x801637D0` together with the r2 float at
/// `-0x5490`, then commits it with `0x801644D0`.
///
/// # Original
/// - Ghidra: `FUN_8015C630` at `0x8015C630` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015c630_FUN_8015c630.c`
///
/// # Arguments
/// * `boot` - the `dScBoot_c` object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Clears `+0x103`; calls `0x801637D0(scale, *object)` and `0x801644D0(object)`.
///
/// # Safety
/// `boot+0xF0` must point at a live resource object whose `+0x1F4` word points
/// at a live save/controller object.
///
/// # Confidence
/// high for the flow and addresses; medium for the object role
/// (`+0x1F4` is only described by this call site).
pub unsafe fn fn_8015c630(boot: usize) {
    let scale = read_f32_abs(F32_SDA2_5490) as f64;

    write_byte(boot, 0x103, 0);

    let resource = read_word(boot, 0xF0) as usize;
    // observed: 500 decimal = 0x1F4.
    let object = read_word(resource, 0x1F4) as usize;
    call_f64_u32(FN_801637D0, scale, read_word(object, 0));
    let _ = platform::call1(FN_801644D0, object as u32);
}

// ===========================================================================
// Wave 228, lane C - the untabled entries of the boot task list at `0x803280E0`
// ===========================================================================
//
// `0x803280E0` is a 32-word function table in `.data`
// (`0x803280E0..0x80328160`, 128 bytes of pointer data). It has no
// `functions.tsv` row and no Ghidra export of its own; every body below was
// read from `work/dol/main.elf`.
//
// ## How the table is reached
//
// 1. `engine::display::fn_8015ddd0` (`0x8015DDD0`, the `dScBoot_c` descriptor
//    registration) initialises two SDA task-list headers. The second one is
//    this table:
//    `0x8015F600(r13 - 0x5328, 0x803280E0, 0x20)`; the first is the 3-slot
//    table at `0x80328090` copied to `r13 - 0x5338`.
// 2. `0x8015F600` (`0x8015F600..0x8015F610`) writes the table word at `+0x00`,
//    the entry count (`0x20`) as a halfword at `+0x04` and clears the cursor
//    halfword at `+0x06`.
// 3. `dsc_boot_update` (`0x8015C5A0`, this module) calls
//    `0x8015F620(r13 - 0x5328, boot)` once per frame, where
//    `r13 - 0x5328` = `0x8042A358` = [`G_TASK_LIST`].
// 4. `0x8015F620` (`0x8015F620..0x8015F6AC`) reads the cursor
//    (`lhz r5,6(r3)` / `lhz r0,4(r3)` / `cmpl`), and while
//    `cursor < count` performs `table[cursor](boot)` through
//    `lwzx`/`mtspr ctr`/`bcctr`. A task returning `1` advances the cursor; the
//    first task returning anything else aborts the frame and that value is
//    returned; `2` is returned once the cursor reaches the count.
//
// Every slot therefore has the signature `u32 task(usize boot)` and answers
// "has this step finished?" with `1` (continue) or `0` (wait).
//
// ## Slot map (32 slots, 12 already translated, 20 untabled in 19 bodies)
//
// | slot | table word | target | owner |
// |---:|---|---|---|
// | 0 | `0x803280E0` | `0x8015BBF0` | `engine::display` |
// | 1 | `0x803280E4` | `0x800E50D0` | **lane C** -> `engine::dsys` |
// | 2 | `0x803280E8` | `0x800E5110` | `engine::dsys` |
// | 3 | `0x803280EC` | `0x8015C010` | [`fn_8015c010`] |
// | 4 | `0x803280F0` | `0x8015C100` | **lane C**, below |
// | 5 | `0x803280F4` | `0x8015BCB0` | **lane C**, below |
// | 6 | `0x803280F8` | `0x8015C070` | **lane C**, below |
// | 7 | `0x803280FC` | `0x8015BE20` | **lane C**, below |
// | 8 | `0x80328100` | `0x8015C090` | **lane C**, below |
// | 9 | `0x80328104` | `0x8015C0E0` | **lane C**, below |
// | 10 | `0x80328108` | `0x8015BD40` | **lane C**, below |
// | 11 | `0x8032810C` | `0x8015BD80` | **lane C**, below |
// | 12 | `0x80328110` | `0x8015BDD0` | [`fn_8015bdd0`] |
// | 13 | `0x80328114` | `0x8015BE60` | `game::boot_scene` (save banner) |
// | 14 | `0x80328118` | `0x8015BEE0` | **lane C**, below |
// | 15 | `0x8032811C` | `0x8015B970` | **lane C**, below |
// | 16 | `0x80328120` | `0x8015B9C0` | **lane C**, below |
// | 17 | `0x80328124` | `0x8015BA10` | **lane C**, below |
// | 18 | `0x80328128` | `0x8015BA60` | **lane C**, below |
// | 19 | `0x8032812C` | `0x8015BAF0` | **lane C**, below |
// | 20 | `0x80328130` | `0x8015BB20` | **lane C**, below |
// | 21 | `0x80328134` | `0x8015BB30` | **lane C**, below |
// | 22 | `0x80328138` | `0x8015BB60` | [`fn_8015bb60`] |
// | 23 | `0x8032813C` | `0x8015BB90` | `engine::display` |
// | 24 | `0x80328140` | `0x8015BBC0` | `engine::display` |
// | 25 | `0x80328144` | `0x8015BEA0` | **lane C**, below |
// | 26 | `0x80328148` | `0x8015BEE0` | same body as slot 14 |
// | 27 | `0x8032814C` | `0x8015BF10` | `engine::display` |
// | 28 | `0x80328150` | `0x8015BF50` | `engine::display` |
// | 29 | `0x80328154` | `0x8015BF90` | `engine::display` |
// | 30 | `0x80328158` | `0x8015BFD0` | `engine::display` |
// | 31 | `0x8032815C` | `0x800E4FB0` | **lane C** -> `engine::dsys` |
//
// ## Measured extents (all read from `work/dol/main.elf`)
//
// The fifteen prologue-verified bodies sum to the 1,112 bytes the wave
// assignment quotes. Four further words point at *prologue-less leaves* that
// the wave-227 prologue filter did not count (slots 4, 6, 9 and 20); they add
// 100 bytes and are translated here as well, so the run is complete.
//
// | slot | address | code extent | bytes | following bytes |
// |---:|---|---|---:|---|
// | 4 | `0x8015C100` | `0x8015C100..0x8015C130` | 48 | next entry `0x8015C130` |
// | 5 | `0x8015BCB0` | `0x8015BCB0..0x8015BCD8` | 40 | 8 zero bytes |
// | 6 | `0x8015C070` | `0x8015C070..0x8015C084` | 20 | 12 zero bytes |
// | 7 | `0x8015BE20` | `0x8015BE20..0x8015BE60` | 64 | next entry `0x8015BE60` |
// | 8 | `0x8015C090` | `0x8015C090..0x8015C0D4` | 68 | 12 zero bytes |
// | 9 | `0x8015C0E0` | `0x8015C0E0..0x8015C0F8` | 24 | 8 zero bytes |
// | 10 | `0x8015BD40` | `0x8015BD40..0x8015BD74` | 52 | 12 zero bytes |
// | 11 | `0x8015BD80` | `0x8015BD80..0x8015BDD0` | 80 | next entry `0x8015BDD0` |
// | 14 | `0x8015BEE0` | `0x8015BEE0..0x8015BF10` | 48 | next entry `0x8015BF10` |
// | 15 | `0x8015B970` | `0x8015B970..0x8015B9BC` | 76 | 4 zero bytes |
// | 16 | `0x8015B9C0` | `0x8015B9C0..0x8015BA0C` | 76 | 4 zero bytes |
// | 17 | `0x8015BA10` | `0x8015BA10..0x8015BA5C` | 76 | 4 zero bytes |
// | 18 | `0x8015BA60` | `0x8015BA60..0x8015BAE8` | 136 | 8 zero bytes |
// | 19 | `0x8015BAF0` | `0x8015BAF0..0x8015BB20` | 48 | next entry `0x8015BB20` |
// | 20 | `0x8015BB20` | `0x8015BB20..0x8015BB28` | 8 | 8 zero bytes |
// | 21 | `0x8015BB30` | `0x8015BB30..0x8015BB60` | 48 | next entry `0x8015BB60` |
// | 25 | `0x8015BEA0` | `0x8015BEA0..0x8015BEDC` | 60 | 4 zero bytes |
// | (1) | `0x800E50D0` | `0x800E50D0..0x800E5110` | 64 | next entry `0x800E5110` |
// | (31) | `0x800E4FB0` | `0x800E4FB0..0x800E5060` | 176 | next entry `0x800E5060` |
//
// Slots 1 and 31 are `engine::dsys` bodies and live in `src/engine/dsys.rs`;
// the remaining seventeen are in this module. See `work/wave228/laneC_tables.md`
// for the per-body accounting and for why lane C did not use `src/ui/menu.rs`:
// the decoded table is the `dScBoot_c` task list, not a `Pausewindow_c` state
// table (the `Pausewindow_c::StateID_*` strings are simply adjacent in `.data`).

/// `0x8006A6A0(id)`: publishes the current sound id (see
/// [`crate::game::player::fn_8006a6a0`]).
const W228_FN_8006A6A0: usize = 0x8006_A6A0;
/// `0x8006A6F0()`: opens the sound descriptor and keeps its handle (see
/// [`crate::game::player::fn_8006a6f0`]). Takes no argument.
const W228_FN_8006A6F0: usize = 0x8006_A6F0;
/// `0x8016B950(descriptor, arg)`: opens one engine descriptor and returns its
/// handle (`b 0x8016B8D0`).
const W228_FN_8016B950: usize = 0x8016_B950;
/// `0x8016B7E0(handle)`: releases one opened descriptor.
const W228_FN_8016B7E0: usize = 0x8016_B7E0;
/// `0x8015BD10`: the 36-byte descriptor opened by slot 10; already translated
/// as [`crate::engine::display::fn_8015bd10`] (it calls `0x8006A280` and
/// returns `1`).
const W228_DESC_8015BD10: usize = 0x8015_BD10;
/// `r13 - 0x5330` -> `0x8042A350`: handle of the descriptor opened by slot 10.
const W228_G_DESCRIPTOR_5330: usize = SDA_BASE - 0x5330;
/// `0x800DF4E0(dres)` -> `1` while a DVD mount was advanced, `0` when every
/// row of the resource table is synced
/// (see [`crate::resource::registry::dres_sync_all_res`]).
const W228_FN_800DF4E0: usize = 0x800D_F4E0;
/// `0x800DF7D0(manager, names, table, count, 0)`: registers an array of
/// resource names with the resource manager
/// (see [`crate::resource::registry::dres_mng_set_res_array`]).
const W228_FN_800DF7D0: usize = 0x800D_F7D0;
/// `0x803280D4`: the three-name pointer array
/// (`0x803280B0` `"Env_world"`, `0x803280BC` `"Env_course"`,
/// `0x803280C8` `"Env_movie"`). It sits 12 bytes before the task table.
const W228_TABLE_803280D4: usize = 0x8032_80D4;
/// `r13 - 0x6288` -> `0x804293F8`: the name array the three `Env_*` archives
/// are published to.
const W228_G_ENV_NAMES_6288: usize = SDA_BASE - 0x6288;
/// `0x8010D190()`: requests the `0x2D0` helper record
/// (see [`crate::game::worldmap::fn_8010d190`]).
const W228_FN_8010D190: usize = 0x8010_D190;
/// `0x8015BCE0()`: increments the scene refcount at `0x8042A2E0`
/// (see [`crate::engine::display::fn_8015bce0`]).
const W228_FN_8015BCE0: usize = 0x8015_BCE0;
/// `r13 - 0x53A0` -> `0x8042A2E0`: the scene refcount `0x8015BCE0` bumps.
const W228_G_SCENE_REFCOUNT_53A0: usize = SDA_BASE - 0x53A0;
/// `r13 - 0x53A3` -> `0x8042A2DD`: the progress byte, i.e.
/// [`G_SAVE_PROGRESS`]` + 5`.
const W228_G_PROGRESS_BYTE_53A3: usize = SDA_BASE - 0x53A3;
/// `0x80109340()`: the live display manager.
const W228_FN_80109340: usize = 0x8010_9340;
/// `0x80109480(object)`: loads six resources and returns `1` when all are
/// present (see [`crate::game::worldmap::fn_80109480`]).
const W228_FN_80109480: usize = 0x8010_9480;
/// `0x80160020(handle)`: probes a handle - `1` while its first halfword is
/// set, otherwise the `+0x20` virtual result
/// (see [`crate::resource::registry::fn_80160020`]).
const W228_FN_80160020: usize = 0x8016_0020;
/// `0x8015FF40(state)`: advances one state object and returns `1` while it
/// stays live (see [`crate::engine::display::fn_8015ff40`]).
const W228_FN_8015FF40: usize = 0x8015_FF40;
/// `0x80377300`, `0x80377358`, `0x803773B0`: the three actor-list objects that
/// `fn_8015ddd0` registers (`0x8008F870` / `0x802DC780`) with the names at
/// `r13 - 0x6280`, `"d_enemies"` and `"d_en_boss"`.
const W228_LIST_ENV: usize = 0x8037_7300;
const W228_LIST_ENEMY: usize = 0x8037_7358;
const W228_LIST_ENEMY_BOSS: usize = 0x8037_73B0;

/// Waits for the pause/strap window and the save object to report ready.
///
/// In plain English: both `boot + 0xF4` (the window opened from resource
/// `0x2C0`) and `boot + 0xF8` (the save/controller object from `0x2CF`) must
/// have raised their ready byte; otherwise the task list stops for this frame.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8015C100`. The body was
///   read from `work/dol/main.elf`.
/// - Table: `0x803280E0` slot 4 (`0x803280F0`), the boot task list copied to
///   `r13 - 0x5328` and run by `0x8015F620`.
/// - Extent: `0x8015C100..0x8015C130` (48 bytes); the early `bclr 20,0` is at
///   `0x8015C124` and the success arm is the tail block at
///   `0x8015C128` (`li r3,1`) / `0x8015C12C` (`bclr 20,0`). No prologue, so the
///   wave-227 prologue filter did not list it as an entry.
/// - Evidence: `work/wave228/laneC_disasm.txt`
///
/// # Arguments
/// * `boot` - the live `dScBoot_c` object; `+0xF4` and `+0xF8` are the two
///   resource handles created by [`fn_8015c010`].
///
/// # Returns
/// `1` when both flags are set, `0` otherwise.
///
/// # Side effects
/// None; three loads.
///
/// # Safety
/// `boot + 0xF4` and `boot + 0xF8` must point at live objects.
///
/// # Confidence
/// high - the two offsets, the flag bytes and the branch polarity are literal.
pub unsafe fn fn_8015c100(boot: usize) -> u32 {
    // observed: `lwz r4,0xf4(r3)` / `lwz r3,0xf8(r3)`, then the two byte tests.
    let window = read_word(boot, 0xF4) as usize;
    let save_object = read_word(boot, 0xF8) as usize;

    // +0x294: window-open flag (the same byte block the pause window exposes at
    // +0x294/+0x295); +0xD38: save-object ready flag.
    if read_byte(window, 0x294) == 0 {
        return 0;
    }
    (read_byte(save_object, 0xD38) != 0) as u32
}

/// Requests the `0x2D0` helper record and bumps the scene refcount.
///
/// In plain English: ask the world-map helper for its `0x2D0` record and then
/// increment the scene refcount at `0x8042A2E0`; the step is always complete.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8015BCB0`.
/// - Table: `0x803280E0` slot 5 (`0x803280F4`), the boot task list.
/// - Extent: `0x8015BCB0..0x8015BCD8` (40 bytes, closing `bclr 20,0` at
///   `0x8015BCD4`), followed by 8 zero bytes up to `0x8015BCE0`.
/// - Evidence: `work/wave228/laneC_disasm.txt`
///
/// # Arguments
/// None. The two callees ignore `r3`.
///
/// # Returns
/// Always `1`.
///
/// # Side effects
/// `0x8010D190()` requests resource `0x2D0`; `0x8015BCE0()` adds one to the
/// word at `0x8042A2E0`.
///
/// # Safety
/// Both callees must be callable in the current state; the refcount word must
/// be writable.
///
/// # Confidence
/// high - both calls and the constant `1` are literal.
pub unsafe fn fn_8015bcb0() -> u32 {
    platform::call0(W228_FN_8010D190);
    platform::call0(W228_FN_8015BCE0);
    1
}

/// Reports the progress byte at `0x8042A2DD` (`G_SAVE_PROGRESS + 5`).
///
/// In plain English: the step is complete as soon as that byte is non-zero.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8015C070`.
/// - Table: `0x803280E0` slot 6 (`0x803280F8`), the boot task list.
/// - Extent: `0x8015C070..0x8015C084` (20 bytes, `bclr 20,0` at `0x8015C080`),
///   followed by 12 zero bytes. Leaf with no prologue.
/// - Evidence: `work/wave228/laneC_disasm.txt`
///
/// # Arguments
/// None.
///
/// # Returns
/// `1` when the byte is non-zero, `0` otherwise.
///
/// # Side effects
/// None. The boolean fold is `neg`/`or`/`rlwinm` on the loaded byte, i.e.
/// `value != 0` (not the `value == 1` fold used by [`positive_test`]).
///
/// # Safety
/// The SDA byte must be readable.
///
/// # Confidence
/// high - one `lbz` and the exact boolean fold are literal.
pub unsafe fn fn_8015c070() -> u32 {
    // observed: `lbz r3,0xffffac5d(r13)`; `neg`/`or`/`rlwinm r3,r0,1,31,31`.
    (platform::read_u8(W228_G_PROGRESS_BYTE_53A3) != 0) as u32
}

/// Waits for the display manager's six-resource load to finish.
///
/// In plain English: take the live display manager and ask
/// `0x80109480(manager)`; the step completes when that call reports `1`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8015BE20`.
/// - Table: `0x803280E0` slot 7 (`0x803280FC`), the boot task list.
/// - Extent: `0x8015BE20..0x8015BE60` (64 bytes), immediately followed by the
///   next entry `0x8015BE60`.
/// - Evidence: `work/wave228/laneC_disasm.txt`
///
/// # Arguments
/// None.
///
/// # Returns
/// `1` when `0x80109480` returns exactly `1` (the `cntlzw(value - 1) >> 5`
/// fold, i.e. [`positive_test`]), `0` otherwise.
///
/// # Side effects
/// Whatever [`crate::game::worldmap::fn_80109480`] does: up to six resource
/// loads and one animation-bank start.
///
/// # Safety
/// The display manager must be live and its name pointers readable.
///
/// # Confidence
/// high - both calls and the fold are literal.
pub unsafe fn fn_8015be20() -> u32 {
    // observed: `bl 0x80109340` then `bl 0x80109480` with that result in `r3`.
    let manager = call0_r(W228_FN_80109340);
    positive_test(platform::call1(W228_FN_80109480, manager)) as u32
}

/// Creates resource `0x2E5` for the boot object and stores it at `+0xFC`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8015C090`.
/// - Table: `0x803280E0` slot 8 (`0x80328100`), the boot task list.
/// - Extent: `0x8015C090..0x8015C0D4` (68 bytes, `bclr 20,0` at `0x8015C0D0`),
///   followed by 12 zero bytes.
/// - Evidence: `work/wave228/laneC_disasm.txt`
///
/// # Arguments
/// * `boot` - the live `dScBoot_c` object; it is also the resource owner.
///
/// # Returns
/// Always `1`.
///
/// # Side effects
/// `0x80162B00(0x2E5, boot, 0, 0)` and one word store.
///
/// # Safety
/// `boot` must be a live object accepted by the resource creator.
///
/// # Confidence
/// high - the id `741` (`li r3,0x2E5`), the owner register and the store are
/// explicit.
pub unsafe fn fn_8015c090(boot: usize) -> u32 {
    // observed: `li r3,741` / `mr r4,r31` / `li r5,0` / `li r6,0` /
    // `bl 0x80162b00` / `stw r3,0xfc(r31)`.
    let handle = platform::call4(FN_80162B00, 0x2E5, boot as u32, 0, 0);
    write_word(boot, 0xFC, handle);
    1
}

/// Reports whether the object at `boot + 0xFC` has raised its `+0x208` flag.
///
/// In plain English: this is the readiness test of the save/controller object
/// created by [`fn_8015c090`]; the step completes once its `+0x208` byte is set.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8015C0E0`.
/// - Table: `0x803280E0` slot 9 (`0x80328104`), the boot task list.
/// - Extent: `0x8015C0E0..0x8015C0F8` (24 bytes, `bclr 20,0` at `0x8015C0F4`),
///   followed by 8 zero bytes. Leaf with no prologue.
/// - Evidence: `work/wave228/laneC_disasm.txt`
///
/// # Arguments
/// * `boot` - the live `dScBoot_c` object.
///
/// # Returns
/// `1` when the flag byte is non-zero, `0` otherwise.
///
/// # Side effects
/// None; two loads.
///
/// # Safety
/// `boot + 0xFC` must point at a live object.
///
/// # Confidence
/// high - the field chain and the boolean fold are literal.
pub unsafe fn fn_8015c0e0(boot: usize) -> u32 {
    // observed: `lwz r3,0xfc(r3)`; `lbz r3,0x208(r3)`; `neg`/`or`/`rlwinm`.
    let save_object = read_word(boot, 0xFC) as usize;
    (read_byte(save_object, 0x208) != 0) as u32
}

/// Opens the `0x8015BD10` descriptor and publishes its handle.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8015BD40`.
/// - Table: `0x803280E0` slot 10 (`0x80328108`), the boot task list.
/// - Extent: `0x8015BD40..0x8015BD74` (52 bytes, `bclr 20,0` at
///   `0x8015BD70`), followed by 12 zero bytes.
/// - Evidence: `work/wave228/laneC_disasm.txt`
///
/// # Arguments
/// None.
///
/// # Returns
/// Always `1`.
///
/// # Side effects
/// `0x8016B950(0x8015BD10, 0)` and one store to `0x8042A350`, which slot 11
/// releases again.
///
/// # Safety
/// The descriptor at `0x8015BD10` must be a valid engine descriptor.
///
/// # Confidence
/// high - `lis 0x8016`/`addi -17136` resolves to the literal `0x8015BD10` and
/// the destination word is `r13 - 0x5330`.
pub unsafe fn fn_8015bd40() -> u32 {
    let handle = platform::call2(W228_FN_8016B950, W228_DESC_8015BD10 as u32, 0);
    write_global_word(W228_G_DESCRIPTOR_5330, handle);
    1
}

/// Waits for the `0x8015BD10` descriptor, then releases it and starts the sound.
///
/// In plain English: while the descriptor opened by [`fn_8015bd40`] still
/// reports `+0x08 == 0` the step stops; once it is set, the descriptor is
/// released, the global handle is cleared, sound id `1` is published and its
/// descriptor opened.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8015BD80`.
/// - Table: `0x803280E0` slot 11 (`0x8032810C`), the boot task list.
/// - Extent: `0x8015BD80..0x8015BDD0` (80 bytes), immediately followed by the
///   next entry `0x8015BDD0`.
/// - Evidence: `work/wave228/laneC_disasm.txt`
///
/// # Arguments
/// None.
///
/// # Returns
/// `0` while the descriptor is not ready, then `1`.
///
/// # Side effects
/// `0x8016B7E0(handle)`, one global clear, `0x8006A6A0(1)` and `0x8006A6F0()`.
///
/// # Safety
/// The global must hold a live descriptor handle when the body runs.
///
/// # Confidence
/// high - the guard, both calls and the store are literal; the sound id `1` is
/// the only argument the source passes to `0x8006A6A0`.
pub unsafe fn fn_8015bd80() -> u32 {
    let handle = read_global_word(W228_G_DESCRIPTOR_5330) as usize;
    // observed: `lbz r0,8(r3)`; `cmpi cr0,0`; `bc 4,2` keeps going, `bc 12,2`
    // jumps to the `li r3,0` early return.
    if read_byte(handle, 8) == 0 {
        return 0;
    }
    let _ = platform::call1(W228_FN_8016B7E0, handle as u32);
    write_global_word(W228_G_DESCRIPTOR_5330, 0);
    let _ = platform::call1(W228_FN_8006A6A0, 1);
    let _ = call0_r(W228_FN_8006A6F0);
    1
}

/// Waits until every DVD mount of the resource manager is synced.
///
/// In plain English: `0x800DF4E0(manager + 4)` returns `0` once no holder is
/// still mounting, so the step completes exactly then.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8015BEE0`.
/// - Table: `0x803280E0` slots 14 (`0x80328118`) and 26 (`0x80328148`); both
///   words hold the same address, so this is one body with two slots.
/// - Extent: `0x8015BEE0..0x8015BF10` (48 bytes), immediately followed by the
///   next entry `0x8015BF10`.
/// - Evidence: `work/wave228/laneC_disasm.txt`
///
/// # Arguments
/// None.
///
/// # Returns
/// `1` when the mount scan returns `0`, `0` otherwise. The fold is
/// `cntlzw(value) >> 5`, i.e. `value == 0` (not [`positive_test`]).
///
/// # Side effects
/// Whatever the scan does for a pending holder: it may finish a mount and run
/// its loader callback.
///
/// # Safety
/// [`G_ARCHIVE_MANAGER`] must hold a live `dRes_c` at `+4`.
///
/// # Confidence
/// high - the pointer chain, the fold and the branch polarity are literal.
pub unsafe fn fn_8015bee0() -> u32 {
    // observed: `lwz r3,0xffffa9b8(r13)` = 0x8042A038; `addi r3,r3,4`;
    // `bl 0x800df4e0`; `cntlzw r0,r3`; `rlwinm r3,r0,27,5,31`.
    let manager = read_global_word(G_ARCHIVE_MANAGER) as usize;
    (platform::call1(W228_FN_800DF4E0, (manager + 4) as u32) == 0) as u32
}

/// Advances the `Env_*` actor list while its handle reports unfinished work.
///
/// In plain English: probe the list object at `0x80377300`; when the probe says
/// the list is still running, run one `0x8015FF40` state advance and report the
/// step complete.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8015B970`.
/// - Table: `0x803280E0` slot 15 (`0x8032811C`), the boot task list.
/// - Extent: `0x8015B970..0x8015B9BC` (76 bytes, `bclr 20,0` at
///   `0x8015B9B8`), followed by 4 zero bytes.
/// - Evidence: `work/wave228/laneC_disasm.txt`
///
/// # Arguments
/// None.
///
/// # Returns
/// `0` when the probe returns `0`, otherwise `1`.
///
/// # Side effects
/// One `0x80160020` probe and, when it is non-zero, one `0x8015FF40` advance.
///
/// # Safety
/// `0x80377300` must be the live list object that `fn_8015ddd0` registered.
///
/// # Confidence
/// high - `lis 0x8037`/`addi 29440` resolves to the literal `0x80377300`, and
/// both calls plus the branch are explicit.
pub unsafe fn fn_8015b970() -> u32 {
    // observed: `addi r3,r31,29440` with `lis r31,0x8037`; `bl 0x80160020`;
    // `cmpi cr0,0`; beq -> `li r3,0`.
    if platform::call1(W228_FN_80160020, W228_LIST_ENV as u32) == 0 {
        return 0;
    }
    let _ = platform::call1(W228_FN_8015FF40, W228_LIST_ENV as u32);
    1
}

/// Advances the `d_enemies` actor list (see [`fn_8015b970`] for the shape).
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8015B9C0`.
/// - Table: `0x803280E0` slot 16 (`0x80328120`), the boot task list.
/// - Extent: `0x8015B9C0..0x8015BA0C` (76 bytes, `bclr 20,0` at
///   `0x8015BA08`), followed by 4 zero bytes.
/// - Evidence: `work/wave228/laneC_disasm.txt`
///
/// # Arguments
/// None.
///
/// # Returns
/// `0` when the probe returns `0`, otherwise `1`.
///
/// # Side effects
/// One `0x80160020` probe on `0x80377358` and one `0x8015FF40` advance.
///
/// # Safety
/// `0x80377358` must be the live `d_enemies` list from `fn_8015ddd0`.
///
/// # Confidence
/// high - `addi r3,r31,29528` resolves to `0x80377358`; the rest is identical
/// to slot 15.
pub unsafe fn fn_8015b9c0() -> u32 {
    if platform::call1(W228_FN_80160020, W228_LIST_ENEMY as u32) == 0 {
        return 0;
    }
    let _ = platform::call1(W228_FN_8015FF40, W228_LIST_ENEMY as u32);
    1
}

/// Advances the `d_en_boss` actor list (see [`fn_8015b970`] for the shape).
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8015BA10`.
/// - Table: `0x803280E0` slot 17 (`0x80328124`), the boot task list.
/// - Extent: `0x8015BA10..0x8015BA5C` (76 bytes, `bclr 20,0` at
///   `0x8015BA58`), followed by 4 zero bytes.
/// - Evidence: `work/wave228/laneC_disasm.txt`
///
/// # Arguments
/// None.
///
/// # Returns
/// `0` when the probe returns `0`, otherwise `1`.
///
/// # Side effects
/// One `0x80160020` probe on `0x803773B0` and one `0x8015FF40` advance.
///
/// # Safety
/// `0x803773B0` must be the live `d_en_boss` list from `fn_8015ddd0`.
///
/// # Confidence
/// high - `addi r3,r31,29616` resolves to `0x803773B0`; the rest is identical
/// to slot 15.
pub unsafe fn fn_8015ba10() -> u32 {
    if platform::call1(W228_FN_80160020, W228_LIST_ENEMY_BOSS as u32) == 0 {
        return 0;
    }
    let _ = platform::call1(W228_FN_8015FF40, W228_LIST_ENEMY_BOSS as u32);
    1
}

/// Waits for six controller/input readiness ids at once.
///
/// In plain English: the step completes when `0x8008F620` reports exactly `1`
/// for every id in `7, 5, 6, 9, 0x2C7, 8`; the first mismatch returns `0`
/// immediately, in that order.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8015BA60`.
/// - Table: `0x803280E0` slot 18 (`0x80328128`), the boot task list.
/// - Extent: `0x8015BA60..0x8015BAE8` (136 bytes, `bclr 20,0` at
///   `0x8015BAE4`), followed by 8 zero bytes.
/// - Evidence: `work/wave228/laneC_disasm.txt`
///
/// # Arguments
/// None.
///
/// # Returns
/// `1` when all six queries return `1`, `0` otherwise.
///
/// # Side effects
/// Up to six `0x8008F620( id )` queries.
///
/// # Safety
/// `0x8008F620` must be callable.
///
/// # Confidence
/// high - the six ids are literal (`li r3,7`; `li r3,5`; `li r3,6`;
/// `li r3,9`; `li r3,711`; `li r3,8`) and every arm compares against `1`.
pub unsafe fn fn_8015ba60() -> u32 {
    for id in [7u32, 5, 6, 9, 0x2C7, 8] {
        if platform::call1(FN_8008F620, id) != 1 {
            return 0;
        }
    }
    1
}

/// Waits for readiness id `0`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8015BAF0`.
/// - Table: `0x803280E0` slot 19 (`0x8032812C`), the boot task list.
/// - Extent: `0x8015BAF0..0x8015BB20` (48 bytes), immediately followed by the
///   next entry `0x8015BB20`.
/// - Evidence: `work/wave228/laneC_disasm.txt`
///
/// # Arguments
/// None.
///
/// # Returns
/// `1` when `0x8008F620(0)` returns exactly `1` (the
/// `cntlzw(value - 1) >> 5` fold, i.e. [`positive_test`]), `0` otherwise.
///
/// # Side effects
/// One `0x8008F620(0)` query.
///
/// # Safety
/// `0x8008F620` must be callable.
///
/// # Confidence
/// high - `li r3,0`, the call and the fold are literal.
pub unsafe fn fn_8015baf0() -> u32 {
    // observed: `li r3,0`; `bl 0x8008f620`; `addi r0,r3,-1`; `cntlzw`;
    // `rlwinm r3,r0,27,5,31`.
    positive_test(platform::call1(FN_8008F620, 0)) as u32
}

/// A task-list step that is always complete.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8015BB20`.
/// - Table: `0x803280E0` slot 20 (`0x80328130`), the boot task list.
/// - Extent: `0x8015BB20..0x8015BB28` (8 bytes: `li r3,1`; `bclr 20,0`),
///   followed by 8 zero bytes. Leaf with no prologue.
/// - Evidence: `work/wave228/laneC_disasm.txt`
///
/// # Arguments
/// None.
///
/// # Returns
/// `1`.
///
/// # Side effects
/// None. `// inferred:` a deliberately empty slot, used to keep a state index
/// stable in the middle of the list.
///
/// # Safety
/// Nothing to require; the body touches no memory.
///
/// # Confidence
/// high - the whole body is two instructions.
pub unsafe fn fn_8015bb20() -> u32 {
    1
}

/// Waits for readiness id `4`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8015BB30`.
/// - Table: `0x803280E0` slot 21 (`0x80328134`), the boot task list.
/// - Extent: `0x8015BB30..0x8015BB60` (48 bytes), immediately followed by the
///   next entry `0x8015BB60` ([`fn_8015bb60`], which tests id `3`).
/// - Evidence: `work/wave228/laneC_disasm.txt`
///
/// # Arguments
/// None.
///
/// # Returns
/// `1` when `0x8008F620(4)` returns exactly `1`, `0` otherwise.
///
/// # Side effects
/// One `0x8008F620(4)` query.
///
/// # Safety
/// `0x8008F620` must be callable.
///
/// # Confidence
/// high - `li r3,4` and the same fold as slot 19 are literal.
pub unsafe fn fn_8015bb30() -> u32 {
    positive_test(platform::call1(FN_8008F620, 4)) as u32
}

/// Registers the three `Env_world` / `Env_course` / `Env_movie` archives.
///
/// In plain English: hand the resource manager the three-entry name array at
/// `0x803280D4`, publishing them to the name block at `0x804293F8`, then report
/// the step complete.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8015BEA0`.
/// - Table: `0x803280E0` slot 25 (`0x80328144`), the boot task list.
/// - Extent: `0x8015BEA0..0x8015BEDC` (60 bytes, `bclr 20,0` at
///   `0x8015BED8`), followed by 4 zero bytes.
/// - Evidence: `work/wave228/laneC_disasm.txt`
///
/// # Arguments
/// None.
///
/// # Returns
/// Always `1`.
///
/// # Side effects
/// `0x800DF7D0(manager, 0x804293F8, 0x803280D4, 3, 0)`, which walks the three
/// names and publishes each one.
///
/// # Safety
/// The archive manager, the three name strings and the destination block must
/// be live.
///
/// # Confidence
/// high - the manager global, the destination (`r13 - 0x6288`), the array
/// (`lis 0x8033`/`addi -32556` = `0x803280D4`) and the count `3` are literal;
/// the three array words were read back from the image and are exactly the
/// `Env_world` / `Env_course` / `Env_movie` pointers.
pub unsafe fn fn_8015bea0() -> u32 {
    // observed: `lis r5,0x8033`; `addi r4,r13,-25224`; `addi r5,r5,-32556`;
    // `li r6,3`; `li r7,0`; `lwz r3,0xffffa9b8(r13)`; `bl 0x800df7d0`.
    let _ = platform::call5(
        W228_FN_800DF7D0,
        read_global_word(G_ARCHIVE_MANAGER),
        W228_G_ENV_NAMES_6288 as u32,
        W228_TABLE_803280D4 as u32,
        3,
        0,
    );
    1
}

// ---------------------------------------------------------------------------
// Wave 275 lane E - untabled gap factory for the boot scene (`_E275`)
// ---------------------------------------------------------------------------

/// Allocates and constructs the boot scene through [`dsc_boot_construct`].
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x8015B930`.
/// - Extent: `0x8015B930..0x8015B960` (48 bytes, 12 instructions), ending at
///   the `blr` at `0x8015B95C`; two zero words precede it at
///   `0x8015B928..0x8015B930` and the following word at `0x8015B960` is a
///   `b`, not another prologue.
/// - Reach: no `b` or `bl` in the executable segments targets it.
/// - Table: the only data word naming it is `0x804293F0`; it is not inside a
///   pointer run.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `li r3,0x110`;
///   `stw r0,0x14(r1)`; `bl 0x801628C0`; `cmpi cr0,0,r3,0`;
///   `bc 12,2` (`beq`) to the epilogue; `bl 0x8015C130`; epilogue; `blr`.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing; the constructed scene is not published by this wrapper.
///
/// # Side effects
/// Allocates 0x110 bytes and runs the `dScBoot_c` constructor, which installs
/// the `0x80327D30`-class scene vtable chain.
///
/// # Safety
/// The size-class allocator at `0x801628C0` must be initialised.
///
/// # Confidence
/// high - the allocation size, the null guard and the constructor call are
/// literal.
pub unsafe fn fn_8015b930() {
    let object = platform::call1(0x8016_28C0, 0x110) as usize;
    if object != 0 {
        dsc_boot_construct(object);
    }
}

// ---------------------------------------------------------------------------
// Wave 281 lane B - five open `dScBoot_c` primary-vtable slots (`_B281`)
//
// The table itself is `0x80328180` (`VTABLE_BOOT_BASE`), installed at `+0x60`
// by [`dsc_boot_construct`]. It is the `dScBoot_c` scene vtable, not a
// `Pausewindow_c` table: the `Pausewindow_c::StateID_*` strings at
// `0x80327F68..0x8032802C` merely precede it in `.data`, the same false lead
// wave 228 corrected for `0x803280E0`. Relative to the base scene table
// `0x8031AFC0`, slots 0, 3, 6, 7, 9, 12 and 16 are overridden; the five bodies
// below are the override words that were still open.
// ---------------------------------------------------------------------------

/// `r13 - 0x5728` -> `0x80429F58`: object last registered by `0x800B6450`.
///
/// [`fn_800b6450`](crate::game::stage::fn_800b6450) writes the object pointer
/// here after clearing its first word, so slot 12's store below is the mirror
/// operation for the boot scene.
const W281_G_REGISTERED_OBJECT: usize = SDA_BASE - 0x5728;

/// `0x800E4850(value_ptr)`: copies four bytes to the `dSys` child `+0x14`.
const W281_FN_800E4850: usize = 0x800E_4850;
/// `0x800E4830(byte)`: writes one byte to the `dSys` child `+0x08`.
const W281_FN_800E4830: usize = 0x800E_4830;
/// `0x800E1F50(id)`: writes scene auxiliary id A (`0x80428738`).
const W281_FN_800E1F50: usize = 0x800E_1F50;
/// `0x800E1F60(id)`: writes scene auxiliary id B (`0x8042873A`).
const W281_FN_800E1F60: usize = 0x800E_1F60;
/// `0x800B2E70()`: refreshes the stage record and forwards it to `0x80161380`.
const W281_FN_800B2E70: usize = 0x800B_2E70;
/// `0x800B2FB0()`: releases the two stage managers and clears their flags.
const W281_FN_800B2FB0: usize = 0x800B_2FB0;
/// `0x802B9210(size)`: the fixed-block allocator (`operator new`).
const W281_FN_802B9210: usize = 0x802B_9210;
/// `0x800DF680(block)`: constructs the resource manager in `block`.
const W281_FN_800DF680: usize = 0x800D_F680;
/// `0x800BB090(block)`: constructs the course manager in `block`.
const W281_FN_800BB090: usize = 0x800B_B090;
/// `0x801025C0()`: initialises the world-map effect bit table.
const W281_FN_801025C0: usize = 0x8010_25C0;
/// `0x801BE2F0(value)`: conditional 1-byte SDA replace at `0x8042A860`.
const W281_FN_801BE2F0: usize = 0x801B_E2F0;
/// `0x80108170(fade, value)`: stores a byte at `fade + 0x2C`.
const W281_FN_80108170: usize = 0x8010_8170;
/// `0x801082D0(fade)`: releases the fade controller phase and clears `+0x2D`.
const W281_FN_801082D0: usize = 0x8010_82D0;
/// `0x80069820()`: builds the scene's three effect objects.
const W281_FN_80069820: usize = 0x8006_9820;
/// `0x8015F9F0(object, free)`: state-object release (`free > 0` frees).
const W281_FN_8015F9F0: usize = 0x8015_F9F0;
/// `0x80162920(object)`: frees one block on the game heap.
const W281_FN_80162920: usize = 0x8016_2920;

/// Boot-scene virtual slot 0 - creates the scene's managers and resets its flags.
///
/// In plain English: publish `0xFF` and `1` to the `dSys` display child, set
/// both scene auxiliary ids to `0x1E`, refresh the stage record, allocate and
/// construct the resource manager (`0x14` bytes) and the course manager
/// (`0xB5C` bytes), initialise the world-map effect bit table, release the two
/// stage managers, clear the object's two controller bytes and commit mode `1`.
/// Every step tolerates a failed allocation.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row and no export file for
///   `0x8015C410`; the body was read from `work/dol/main.elf`.
/// - Extent: `0x8015C410..0x8015C4AC` (156 bytes, 39 instructions), ending at
///   the `blr` at `0x8015C4A8`; `0x8015C4AC` is zero and the next function
///   starts at `0x8015C4B0`.
/// - Table: `0x80328180` (`VTABLE_BOOT_BASE`) slot 0 (`0x80328188`); the base
///   table `0x8031AFC0` slot 0 (`0x8031AFC8`) is the `li r3,1; blr` stub.
/// - Observed: `li r0,0xFF`; `stw r0,8(r1)`; `addi r3,r1,8`;
///   `bl 0x800E4850`; `li r3,1`; `bl 0x800E4830`; `li r3,0x1E`;
///   `bl 0x800E1F50`; `li r3,0x1E`; `bl 0x800E1F60`; `bl 0x800B2E70`;
///   `li r3,0x14`; `bl 0x802B9210`; `beq`; `bl 0x800DF680`; `li r3,0xB5C`;
///   `bl 0x802B9210`; `beq`; `bl 0x800BB090`; `bl 0x801025C0`;
///   `bl 0x800B2FB0`; `stb r0,0x104(r31)`; `li r3,1`; `stb r0,0x105(r31)`;
///   `bl 0x801BE2F0`; `li r3,1`.
///
/// # Arguments
/// * `boot` - the `dScBoot_c` object (`r3`), kept in `r31`.
///
/// # Returns
/// Always `1`.
///
/// # Side effects
/// Two `dSys` writes, two auxiliary-id writes, one stage refresh, at most two
/// manager constructions, one effect-table initialisation, one stage-manager
/// release, two byte clears at `+0x104`/`+0x105` and one SDA replacement.
///
/// # Safety
/// `boot` must provide the 0x110-byte `dScBoot_c` layout and the manager,
/// task and display subsystems must be initialised.
///
/// # Confidence
/// high - every call target, allocator size, store offset and the return value
/// are literal. `// inferred:` the mode commit is `0x801BE2F0(1)` because the
/// callee replaces the SDA byte at `0x8042A860` with `1`.
pub unsafe fn fn_8015c410(boot: usize) -> u32 {
    // 0x8015C41C..0x8015C430: the callee reads one word and copies its four
    // bytes in big-endian order, so the local word is 0x000000FF.
    let value: u32 = 0xFF;
    let _ = platform::call1(
        W281_FN_800E4850,
        core::ptr::addr_of!(value) as *const u32 as u32,
    );
    let _ = platform::call1(W281_FN_800E4830, 1);
    let _ = platform::call1(W281_FN_800E1F50, 0x1E);
    let _ = platform::call1(W281_FN_800E1F60, 0x1E);
    platform::call0(W281_FN_800B2E70);

    let dres = platform::call1(W281_FN_802B9210, 0x14);
    if dres != 0 {
        let _ = platform::call1(W281_FN_800DF680, dres);
    }
    let course = platform::call1(W281_FN_802B9210, 0xB5C);
    if course != 0 {
        let _ = platform::call1(W281_FN_800BB090, course);
    }

    platform::call0(W281_FN_801025C0);
    platform::call0(W281_FN_800B2FB0);
    write_byte(boot, 0x104, 0);
    write_byte(boot, 0x105, 0);
    let _ = platform::call1(W281_FN_801BE2F0, 1);
    1
}

/// Boot-scene virtual slot 7 - runs the base update, then requests the fade.
///
/// In plain English: run [`crate::game::scene::scene_base_update`]; if it
/// returns `0` the frame is done. Otherwise store `1` at `fade + 0x2C` on the
/// fade context and return `1`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x8015C560`.
/// - Extent: `0x8015C560..0x8015C5A0` (64 bytes, 16 instructions), ending at
///   the `blr` at `0x8015C59C`; the next function `dsc_boot_update` starts
///   exactly at `0x8015C5A0`.
/// - Table: `0x80328180` (`VTABLE_BOOT_BASE`) slot 7 (`0x803281A4`); the base
///   slot 7 (`0x8031AFE4`) is `0x800E1BE0`, the plain scene update.
/// - Observed: `bl 0x800E1BE0`; `cmpi cr0,0,r3,0`; `bc 4,2` (`bne`) around
///   `li r3,0`; `b` to the epilogue; on the other arm `bl 0x80107F10`;
///   `li r4,1`; `bl 0x80108170`; `li r3,1`.
///
/// # Arguments
/// * `boot` - the `dScBoot_c` object.
///
/// # Returns
/// `0` when the base update reports no progress, otherwise `1`.
///
/// # Side effects
/// Whatever the base update does, plus one fade-context read and one byte store
/// at `fade + 0x2C`.
///
/// # Safety
/// `boot` must be a live scene object and the fade manager initialised.
///
/// # Confidence
/// high - the branch, the fade accessor and the byte store are literal.
pub unsafe fn fn_8015c560(boot: usize) -> u32 {
    if crate::game::scene::scene_base_update(boot) == 0 {
        return 0;
    }
    // 0x8015C580: r3 = fade context from 0x80107F10, r4 = 1.
    let fade = call0_r(FN_80107F10);
    let _ = platform::call2(W281_FN_80108170, fade, 1);
    1
}

/// Boot-scene virtual slot 9 - the constant `1` predicate.
///
/// In plain English: return `1` without reading any register or memory. The
/// base table's slot 9 (`0x801621D0`) is the same two-instruction stub; this
/// override simply provides a distinct word in the derived table.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x8015C620`.
/// - Extent: `0x8015C620..0x8015C628` (8 bytes, 2 instructions: `li r3,1`;
///   `blr`).
/// - Table: `0x80328180` (`VTABLE_BOOT_BASE`) slot 9 (`0x803281AC`).
/// - Observed: `38600001 li r3,1`; `4E800020 blr`.
///
/// # Arguments
/// None.
///
/// # Returns
/// `1`.
///
/// # Side effects
/// None.
///
/// # Safety
/// None.
///
/// # Confidence
/// high - both instructions are literal and identical to the base stub.
pub unsafe fn fn_8015c620() -> u32 {
    1
}

/// Boot-scene virtual slot 12 - clears the registered object's first word.
///
/// In plain English: load the object last published at `0x80429F58` and store
/// `0` in its first word. There is no null check in the original.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x8015C4B0`.
/// - Extent: `0x8015C4B0..0x8015C4C0` (16 bytes, 4 instructions), ending at
///   the `blr` at `0x8015C4BC`; the next function `0x8015C4C0` starts exactly
///   at `0x8015C4C0`.
/// - Table: `0x80328180` (`VTABLE_BOOT_BASE`) slot 12 (`0x803281B8`); the base
///   slot 12 (`0x8031AFF8`) is a bare `blr`.
/// - Observed: `806DA8D8 lwz r3,-0x5728(r13)`; `li r0,0`; `stw r0,0(r3)`;
///   `blr`.
///
/// # Arguments
/// None.
///
/// # Returns
/// The object pointer that was loaded, in `r3`.
///
/// # Side effects
/// Clears the first word of `*(0x80429F58)`.
///
/// # Safety
/// The slot must already hold a live object; the original does not test it.
///
/// # Confidence
/// high - the load, the zero and the store are literal.
pub unsafe fn fn_8015c4b0() -> u32 {
    let object = read_global_word(W281_G_REGISTERED_OBJECT) as usize;
    write_word(object, 0, 0);
    object as u32
}

/// Boot-scene virtual slot 16 - the deleting destructor.
///
/// In plain English: clear the global boot object, release the two embedded
/// state objects at `+0xC8` (group at `+0xB0`) and `+0x8C` (group at `+0x74`)
/// with a zero free flag, run the base-scene destructor, and free the block
/// only when `free > 0`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x8015C370`.
/// - Extent: `0x8015C370..0x8015C40C` (156 bytes, 39 instructions), ending at
///   the `blr` at `0x8015C408`; the next function `0x8015C410` starts exactly
///   at `0x8015C410`.
/// - Table: `0x80328180` (`VTABLE_BOOT_BASE`) slot 16 (`0x803281C8`); the base
///   slot 16 (`0x8031B008`) is `0x800E1A20`, the plain base destructor.
/// - Observed: `cmpi cr0,0,r3,0`; `bc 12,2` (`beq`) to the epilogue;
///   `addic. r3,r3,0xB0`; `li r0,0`; `stw r0,-0x5340(r13)`;
///   `beq`/`beq`/`addic. r3,r3,0x18`/`beq`; `li r4,0`;
///   `bl 0x8015F9F0`; `addic. r0,r30,0x74`; the same test chain for
///   `r0 + 0x18` at `+0x8C`; `bl 0x800E1A20` with `r4=0`;
///   `cmpi cr0,0,r31,0`; `bc 4,1` (`ble`) around `bl 0x80162920`.
///
/// # Arguments
/// * `boot` - the object to destroy (`r3`, kept in `r30`).
/// * `free` - `>0` frees the block through `0x80162920`.
///
/// # Returns
/// `boot`, so the caller can chain the release.
///
/// # Side effects
/// Clears `0x8042A340`, releases two state objects, runs the base destructor
/// and optionally frees the allocation.
///
/// # Safety
/// `boot` must be null or a live `dScBoot_c` allocation whose two embedded
/// state groups are initialised or zeroed.
///
/// # Confidence
/// high - the field offsets, the zero free flags, the base-destructor call and
/// the `free > 0` guard are literal. The duplicated `addic.` tests are the
/// compiler's inlined null checks on the member addresses; they cannot fail for
/// a non-null `boot`, which is why the release calls are unconditional here,
/// matching the existing `0x8015C2A0`/`0x8015C300` translations.
pub unsafe fn fn_8015c370(boot: usize, free: i32) -> usize {
    if boot != 0 {
        write_global_word(G_BOOT_OBJECT, 0);
        let _ = platform::call2(W281_FN_8015F9F0, (boot + 0xC8) as u32, 0);
        let _ = platform::call2(W281_FN_8015F9F0, (boot + 0x8C) as u32, 0);
        let _ = crate::game::scene::scene_base_destroy(boot, 0);
        if free > 0 {
            let _ = platform::call1(W281_FN_80162920, boot as u32);
        }
    }
    boot
}
