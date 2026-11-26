//! OS-level HLE (docs/PORT_PLAN.md, section 2.3): direct boot of `main.dol`
//! the way the apploader leaves it, the run loop that delivers external and
//! decrementer interrupts between instruction slices, and the `OSLink` hook
//! that records where each REL module was placed.

pub mod camera;
mod memcpy_validation;
mod native_memcpy;
#[cfg(test)]
mod synthetic_actor_gate;
pub mod display;
pub mod join;
pub mod light_texture;
pub mod menu_layout;
pub mod menu_settings;
pub mod p1_character;
pub mod reverb;

use std::cell::RefCell;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use game_version::{Version, E1};
use guest_cpu::blocks::Blocks;
use guest_cpu::{Cpu, GuestRuntime, GuestWord, Stop, MSR_EE, SPR_HID0, SPR_HID4};
use guest_mem::Mem;
use hle_ios::Ios;
use hw::{Hw, HwPort};

// The game's addresses here and in the other modules are USA revision 1's
// ([`E1`]); [`Addrs`] and `E1::va` give the running version's.

/// `OSLink(module, bss)` and `OSLinkFixed(module, bss)`.
pub const OS_LINK: E1 = E1(0x801B_1F20);
pub const OS_LINK_FIXED: E1 = E1(0x801B_1F30);
/// Cycles between hardware updates and interrupt checks. A slice ends at
/// the first instruction after a branch once this many have run (or when
/// the scheduler goes idle): the interpreter's blocks end at branches, and
/// recompiled code checks after each branch, so both engines end slices at
/// the same instructions and recompiled code never has to stop mid-block.
pub const SLICE: u64 = 500;
/// With recompiled code: instructions stepped one at a time looking for a
/// block start before decoding interpreter blocks.
const PROBE: u32 = 16;
/// Head of the scheduler's idle loop (`lwz r0,-0x4F78(r13)`; `cmpwi`; `beq`),
/// spun with interrupts on while the run-queue bitmap is empty.
const IDLE_LOOP: E1 = E1(0x801B_4FF0);
const RUN_QUEUE_BITS: E1 = E1(0x8042_A708);
/// Functions answered on the host with an immediate return:
/// - the data-cache range loops (`DCInvalidateRange`, `DCFlushRange`,
///   `DCStoreRange`): cache operations do nothing in this emulator, and the
///   loops cost a few percent of guest time;
/// - the AX aux-bus effect callbacks (`AXFXReverbHi`, `ReverbStd`, `Delay`
///   and `Chorus`, stereo and Dolby Pro Logic II): their output only reaches
///   the speakers, which P5 adds (they will run natively then). In menus they
///   were over a third of guest time, and the delay on the world map 2-3%.
/// The first [`DC_LOOPS`] of [`RETURN_AT_ONCE`] are the cache loops, the
/// rest the AXFX callbacks.
const DC_LOOPS: usize = 3;
const RETURN_AT_ONCE: [u32; 12] = [
    0x801A_C440, 0x801A_C470, 0x801A_C4A0, // DC range loops
    0x801A_3450, 0x801A_3880, 0x801A_48F0, // AXFXReverbHi
    0x801A_5C10, 0x801A_6580, // AXFXDelay
    0x801A_6D30, 0x801A_7A80, // AXFXReverbStd
    0x801A_88B0, 0x801A_93F0, // AXFXChorus
];
/// fdlibm's bit-by-bit `sqrt` (`__ieee754_sqrt`), computed natively: it is
/// correctly rounded, so `f64::sqrt` gives the same bits.
const SQRT: E1 = E1(0x802E_8620);
/// MSL's `strcmp`, computed natively (the first differing bytes' difference,
/// as unsigned chars). Scene loads compare every effect name with it.
const STRCMP: E1 = E1(0x802E_1AB4);
/// The boot library's memcpy entry in USA revision 1.
const MEMCPY: E1 = E1(0x8000_4364);
/// The following routine's entry in USA revision 1.
const MEMCPY_END: u32 = 0x8000_4600;
/// Its `errno` (r13 - 0x4698) and the NaN it returns for negative inputs.
const ERRNO: E1 = E1(0x8042_AFE8);
const EDOM: u32 = 0x21;
/// [`System::call_guest`]'s data, in low memory the game leaves unused
/// (up to [`p1_character::STUBS`]), and its 64 KiB stack in MEM2 above the
/// game's (from 0x93400000: IOS's own memory on a real Wii, which nothing
/// uses here).
const CALL_SCRATCH: u32 = 0x8000_1800;
const CALL_SCRATCH_CAPACITY: usize = 0x400;
const CALL_STACK_ARG_CAPACITY: usize = 0x400;
const CALL_STACK_LINKAGE: usize = 8;
const CALL_STACK_TOP: u32 = 0x9341_0000 - 0x10;
/// The OS's current thread (`OSGetCurrentThread`), and `OSThread`'s size.
const CURRENT_THREAD: u32 = 0x8000_00E4;
const OS_THREAD_SIZE: usize = 0x318;
/// The scheduler's hold count (r13 - 0x4F80; `OSDisableScheduler` adds
/// one): `SelectThread` switches no thread while it is above 0.
const RESCHEDULE: E1 = E1(0x8042_A700);

/// The addresses the run loop checks at every block, in the version that
/// runs: taken from their USA ones once, at boot. One the version lacks is
/// 0, which no code runs at (its hook never acts).
#[derive(Clone)]
pub(crate) struct Addrs {
    return_at_once: [u32; 12],
    sqrt: u32,
    strcmp: u32,
    memcpy: u32,
    errno: u32,
    update_buffer: u32,
    idle_loop: u32,
    run_queue_bits: u32,
    os_link: u32,
    os_link_fixed: u32,
    draw_and_capture: u32,
    bg_execute: u32,
    bg_execute_end: u32,
    pub(crate) menu_hooks: [u32; 18],
    pub(crate) display_hooks: [u32; 4],
    /// Every address [`System::before`] acts on (`OSLink` returns aside),
    /// and a bit `(pc >> 2) & 255` set for each.
    special: Vec<u32>,
    filter: [u64; 4],
}

fn va_or_none(a: E1, missing: &mut Vec<u32>) -> u32 {
    a.try_va().unwrap_or_else(|| {
        missing.push(a.0);
        0
    })
}

impl Addrs {
    fn new() -> Self {
        Self::with_memcpy(std::env::var("NSMBW_RUST_MEMCPY").as_deref() == Ok("1"))
    }

    fn with_memcpy(rust_memcpy: bool) -> Self {
        let memcpy = if rust_memcpy && game_version::current() == Version::E1 {
            MEMCPY.0
        } else {
            0
        };
        let mut missing = Vec::new();
        let mut va = |a: u32| va_or_none(E1(a), &mut missing);
        let return_at_once = RETURN_AT_ONCE.map(&mut va);
        let menu_hooks = menu_settings::HOOKS.map(|h| va(h.0));
        let display_hooks = display::HOOKS.map(|h| va(h.0));
        let mut a = Self {
            return_at_once,
            sqrt: va(SQRT.0),
            strcmp: va(STRCMP.0),
            memcpy,
            errno: va(ERRNO.0),
            update_buffer: va(reverb::UPDATE_BUFFER.0),
            idle_loop: va(IDLE_LOOP.0),
            run_queue_bits: va(RUN_QUEUE_BITS.0),
            os_link: va(OS_LINK.0),
            os_link_fixed: va(OS_LINK_FIXED.0),
            draw_and_capture: va(light_texture::DRAW_AND_CAPTURE.0),
            bg_execute: va(camera::BG_EXECUTE.0),
            bg_execute_end: va(camera::BG_EXECUTE_END.0),
            menu_hooks,
            display_hooks,
            special: Vec::new(),
            filter: [0; 4],
        };
        if !missing.is_empty() {
            let list: Vec<String> = missing.iter().map(|m| format!("{m:#010x}")).collect();
            eprintln!("{}: no counterpart of these USA addresses, their hooks are off: {}", game_version::current().describe(), list.join(", "));
        }
        let mut special: Vec<u32> = a.return_at_once.to_vec();
        special.extend([a.sqrt, a.strcmp, a.memcpy, a.draw_and_capture, a.update_buffer, a.idle_loop, a.os_link, a.os_link_fixed, p1_character::PORTRAIT, a.bg_execute, a.bg_execute_end]);
        special.extend(a.menu_hooks);
        special.extend(a.display_hooks);
        special.retain(|&x| x != 0);
        special.sort_unstable();
        special.dedup();
        for &x in &special {
            let b = (x >> 2) & 255;
            a.filter[(b >> 6) as usize] |= 1 << (b & 63);
        }
        a.special = special;
        a
    }

    #[inline(always)]
    fn may_be_special(&self, pc: u32) -> bool {
        let b = (pc >> 2) & 255;
        self.filter[(b >> 6) as usize] & (1 << (b & 63)) != 0
    }
}

/// Every address [`System::before`] acts on in the version that runs
/// ([`game_version::set`] first). Recompiled code never runs through one of
/// these (`ppc-recomp` bakes this list in, and
/// [`guest_cpu::recomp::Rt::new`] checks it). The USA revision 1 memcpy
/// experiment (`NSMBW_RUST_MEMCPY=1`) uses the interpreter dispatcher.
pub fn special() -> Vec<u32> {
    Addrs::new().special
}

/// The hooks that replace a whole function: return at once (DC loops,
/// AXFX callbacks), the native reverb, [`STRCMP`], and [`SQRT`]. Runs
/// the one at `pc` if it applies, returning to `lr`; true if
/// it ran. One that
/// does not apply changes nothing, so recompiled code calls this at a call
/// to a hook (`Rt::set_hook`) instead of unwinding to the dispatcher, and
/// unwinds only when it declines ([`System::before`] then asks again).
fn replace(cpu: &mut Cpu, mem: &mut Mem, a: &Addrs, pc: u32, aux_effects: bool) -> bool {
    if pc == 0 {
        return false;
    }
    if let Some(i) = a.return_at_once.iter().position(|&r| r == pc) {
        if !aux_effects || i < DC_LOOPS {
            cpu.pc = cpu.lr;
            return true;
        }
    }
    if pc == a.update_buffer && aux_effects && reverb::update_buffer(mem, cpu.gpr[3], cpu.gpr[5], cpu.gpr[8]) {
        cpu.pc = cpu.lr;
        return true;
    }
    if pc == a.strcmp {
        if let Some(r) = strcmp(mem, cpu.gpr[3], cpu.gpr[4]) {
            cpu.gpr[3] = r as u32;
            cpu.pc = cpu.lr;
            return true;
        }
        return false;
    }
    // With the FPU off, f1 may still be another thread's: let the real
    // code run and take the FP-unavailable exception first.
    if pc == a.sqrt && cpu.msr & guest_cpu::MSR_FP != 0 {
        cpu.fpr[1][0] = sqrt(mem, a.errno, cpu.fpr[1][0]);
        cpu.pc = cpu.lr;
        return true;
    }
    false
}

/// Byte-copy candidate for the diagnostic validator.
fn replace_memcpy(cpu: &mut Cpu, mem: &mut Mem) -> bool {
    if !mem.copy_ram_nonoverlapping(cpu.gpr[3], cpu.gpr[4], cpu.gpr[5] as usize) {
        return false;
    }
    cpu.pc = cpu.lr;
    true
}

/// [`STRCMP`]'s result, or `None` (run the guest's own) if a string
/// leaves RAM. Each string's RAM is taken once, to the end of its bank
/// ([`ram_from`]); scene loads call this hundreds of thousands of times.
fn strcmp(mem: &Mem, a: u32, b: u32) -> Option<i32> {
    let (sa, sb) = (ram_from(mem, a), ram_from(mem, b));
    for i in 0.. {
        let x = *sa?.get(i)?;
        let y = *sb?.get(i)?;
        if x != y {
            return Some(i32::from(x) - i32::from(y));
        }
        if x == 0 {
            return Some(0);
        }
    }
    unreachable!()
}

/// The RAM from `addr` to the end of the bank it is in (MEM1, MEM2 or the
/// locked cache, at any of their addresses), or `None` if `addr` is not
/// RAM. The bytes after a bank's end are never RAM, so reading a string
/// from this slice stops where reading it a byte at a time would.
fn ram_from(mem: &Mem, addr: u32) -> Option<&[u8]> {
    let low = (addr & 0x0FFF_FFFF) as usize;
    let size = match addr >> 28 {
        0x0 | 0x8 | 0xC => guest_mem::MEM1_SIZE,
        0x1 | 0x9 | 0xD => guest_mem::MEM2_SIZE,
        0xE => guest_mem::LOCKED_CACHE_SIZE,
        _ => return None,
    };
    mem.ram(addr, size.checked_sub(low)?)
}

/// [`SQRT`]'s result, with its special cases and `errno`.
fn sqrt(mem: &mut Mem, errno: u32, x: f64) -> f64 {
    if !x.is_finite() {
        // NaN stays NaN, +inf stays +inf, -inf becomes NaN.
        let _ = mem.write_u32(errno, EDOM);
        return x + x * x;
    }
    if x == 0.0 {
        return x;
    }
    if x < 0.0 {
        let _ = mem.write_u32(errno, EDOM);
        return f64::from_bits(0x7FFF_FFFF_0000_0000);
    }
    x.sqrt()
}

fn be32(b: &[u8], at: usize) -> u32 {
    u32::from_be_bytes(b[at..at + 4].try_into().unwrap())
}

const MODULE_LIST_HEAD: u32 = 0x8000_30c8;

fn resolve_rel_address_in_memory(
    mem: &Mem,
    address: RelAddress,
    len: usize,
    executable: bool,
) -> Option<u32> {
    let span = len.max(if executable { 4 } else { 1 });
    if address.module == 0 || span > guest_mem::MEM2_SIZE {
        return None;
    }
    // OSLink and OSUnlink maintain this list and the relocated section
    // tables. Read them from RAM so loaded snapshots need no extra metadata.
    let mut current = be32(mem.ram(MODULE_LIST_HEAD, 4)?, 0);
    let mut previous = 0;
    while current != 0 {
        let header = mem.ram(current, 20)?;
        if be32(header, 8) != previous {
            return None;
        }
        if be32(header, 0) == address.module {
            if address.section >= be32(header, 12) {
                return None;
            }
            let table = be32(header, 16);
            if table == 0 {
                return None;
            }
            let entry = mem.ram(table.checked_add(address.section.checked_mul(8)?)?, 8)?;
            let base_and_flags = be32(entry, 0);
            let base = base_and_flags & !1;
            if base == 0 || address.offset.checked_add(span as u32)? > be32(entry, 4) {
                return None;
            }
            let resolved = base.checked_add(address.offset)?;
            if executable && (base_and_flags & 1 == 0 || resolved % 4 != 0) {
                return None;
            }
            return mem.ram(resolved, span).map(|_| resolved);
        }
        previous = current;
        current = be32(header, 4);
    }
    None
}

/// A REL module after `OSLink` returned.
#[derive(Debug, Clone)]
pub struct LinkedModule {
    pub id: u32,
    pub header: u32,
    pub ok: bool,
    /// (address, size, executable) of each non-empty section.
    pub sections: Vec<(u32, u32, bool)>,
    pub at_step: u64,
}

pub use guest_cpu::{GuestCallResult, RelAddress};

/// A Rust body that replaces the guest function at one address.
///
/// It receives the incoming GPRs and FPRs, returns the value for `r3`, and
/// must leave `cpu.pc = cpu.lr`; [`System::before`] does the return. Bodies
/// run under `platform::with_guest_runtime`, so their guest loads, stores and
/// calls use the same `System`.
pub type NativeBody = unsafe fn(&mut System, &[u32; 32], &[[f64; 2]; 32]) -> u32;

pub struct System {
    memcpy_validator: Option<memcpy_validation::Validator>,
    native_memcpy: Option<native_memcpy::NativeCopy>,
    /// Opt-in Rust bodies by guest address ([`System::register_native`]).
    native_bodies: Vec<(u32, NativeBody)>,
    pub cpu: Cpu,
    pub mem: Mem,
    pub hw: Rc<RefCell<Hw>>,
    pub ios_version: u32,
    pub linked: Vec<LinkedModule>,
    /// `OSLink` calls in flight: (module header, return address).
    link_calls: Vec<(u32, u32)>,
    pub steps: u64,
    /// Slice ends: the hardware clock, interrupt delivery, when a field's
    /// run stops. Shared with recompiled code, which ends slices itself.
    pub slices: Rc<RefCell<Slices>>,
    /// Cycles skipped while idle.
    pub idle_cycles: u64,
    /// Decoded blocks for [`System::run_field`].
    pub blocks: Blocks,
    /// Light-texture redraws skipped while nothing they read changed.
    pub light_textures: light_texture::LightTextures,
    /// Run the game's AX aux effect callbacks (reverb, delay, chorus); the
    /// DSP then mixes their output back. Off: they return at once and the
    /// aux returns are left out (`NSMBW_AUXFX=off`).
    pub aux_effects: bool,
    /// Recompiled code (P6), if registered.
    pub recomp: Option<guest_cpu::recomp::Rt>,
    /// The game menus' Settings buttons ([`menu_settings`]).
    pub menu: menu_settings::MenuSettings,
    /// Player 1's character pick ([`p1_character`]), if applied.
    pub p1: p1_character::P1Character,
    /// Depth Blur and the widened picture ([`display`]).
    pub display: display::Display,
    /// Instructions run by recompiled code (the rest were interpreted).
    pub recomp_steps: u64,
    /// Times the dispatcher entered recompiled code: after an interrupt, a
    /// thread switch, a hook or interpreted code (calls within recompiled
    /// code stay native).
    pub recomp_entries: u64,
    /// The shared camera for online play, and this PC's view.
    pub camera: camera::SharedCamera,
    /// Players joining a course in progress.
    pub joins: join::Joins,
    /// The version that runs, and its addresses for the run loop.
    pub version: Version,
    addrs: Addrs,
}

/// What happens at the end of each slice ([`SLICE`]).
pub struct Slices {
    hw: Rc<RefCell<Hw>>,
    /// The current slice ends after a branch once `cpu.cycles` reaches this.
    pub end: u64,
    /// [`System::run_field`] stops once VI retraces pass this count, or
    /// `cpu.cycles` reaches `limit`.
    until_retraces: u64,
    limit: u64,
    pub stop: bool,
    pub interrupts: u64,
    pub decrementers: u64,
    /// The PC after each slice end, counted (`--profile-guest`).
    pub samples: Option<std::collections::HashMap<u32, u64>>,
    /// The interrupt [`Slices::advance`] found to take, for [`Slices::take`].
    pending: Option<Pending>,
}

#[derive(Clone, Copy)]
enum Pending {
    External,
    Decrementer,
}

impl Slices {
    /// Ends the slice: the hardware runs up to the CPU clock, and a pending
    /// external or decrementer interrupt is taken. Returns false if code
    /// running natively must return to the dispatcher (an interrupt was
    /// taken, or the run is over).
    pub fn end_slice(&mut self, cpu: &mut Cpu, mem: &mut Mem) -> bool {
        let go = self.advance(cpu, mem);
        self.take(cpu);
        go
    }

    /// [`Slices::end_slice`] up to the CPU: the hardware runs up to the CPU
    /// clock, the stop and the next slice are decided, and an interrupt to
    /// take is noted for [`Slices::take`]. False as `end_slice` returns.
    pub fn advance(&mut self, cpu: &Cpu, mem: &mut Mem) -> bool {
        let mut hw = self.hw.borrow_mut();
        hw.advance(cpu.cycles, mem);
        self.pending = if cpu.msr & MSR_EE == 0 {
            None
        } else if hw.interrupt_pending() {
            Some(Pending::External)
        } else if cpu.decrementer_pending() {
            Some(Pending::Decrementer)
        } else {
            None
        };
        if hw.retraces > self.until_retraces || cpu.cycles >= self.limit {
            self.stop = true;
        }
        drop(hw);
        if self.pending.is_none() {
            self.sample(cpu.pc);
        }
        self.end = cpu.cycles + SLICE;
        self.pending.is_none() && !self.stop
    }

    /// Takes the interrupt [`Slices::advance`] noted, if any.
    pub fn take(&mut self, cpu: &mut Cpu) {
        match self.pending.take() {
            Some(Pending::External) => {
                self.interrupts += 1;
                cpu.exception(0x500, cpu.pc);
            }
            Some(Pending::Decrementer) => {
                self.decrementers += 1;
                cpu.take_decrementer();
            }
            None => return,
        }
        self.sample(cpu.pc);
    }

    fn sample(&mut self, pc: u32) {
        if let Some(s) = &mut self.samples {
            *s.entry(pc).or_default() += 1;
        }
    }
}

fn load_dol(mem: &mut Mem, dol: &[u8]) -> Result<u32, String> {
    // The DOL's BSS range spans .sdata/.sdata2 too (0x80351680..0x8042FC3C
    // covers .sdata at 0x80427680), so clear it first and load sections over it.
    let (bss, bss_size) = (be32(dol, 0xD8), be32(dol, 0xDC) as usize);
    if let Some(b) = mem.ram_mut(bss, bss_size) {
        b.fill(0);
    }
    for (count, off_base, addr_base, size_base) in [(7, 0x00, 0x48, 0x90), (11, 0x1C, 0x64, 0xAC)] {
        for i in 0..count {
            let (off, addr, size) = (be32(dol, off_base + 4 * i) as usize, be32(dol, addr_base + 4 * i), be32(dol, size_base + 4 * i) as usize);
            if size == 0 {
                continue;
            }
            let dst = mem.ram_mut(addr, size).ok_or(format!("DOL section at {addr:#x} is outside RAM"))?;
            dst.copy_from_slice(dol.get(off..off + size).ok_or("DOL section past end of file")?);
        }
    }
    Ok(be32(dol, 0xE0))
}

/// Low-memory globals the apploader and IOS leave for the game.
// Values follow Dolphin's direct-boot setup for Wii titles; the boot report
// lists which of them OSInit reads.
fn setup_low_memory(mem: &mut Mem, boot_bin: &[u8], fst: &[u8], ios: u32) -> Result<(), String> {
    let mut w = |addr: u32, v: u32| mem.write_u32(addr, v).map_err(|_| format!("low memory {addr:#x}"));
    for i in 0..8 {
        w(0x8000_0000 + 4 * i, be32(boot_bin, 4 * i as usize))?; // disc header: id, maker, version, magic
    }
    w(0x8000_0020, 0x0D15_EA5E)?; // normal boot
    w(0x8000_0024, 1)?;
    w(0x8000_0028, 0x0180_0000)?; // MEM1 size
    w(0x8000_002C, 0x23)?; // retail console
    w(0x8000_00CC, 0)?; // NTSC video
    w(0x8000_00F0, 0x0180_0000)?; // simulated MEM1 size
    w(0x8000_00F8, 243_000_000)?; // bus clock
    w(0x8000_00FC, 729_000_000)?; // CPU clock
    w(0x8000_3100, 0x0180_0000)?;
    w(0x8000_3104, 0x0180_0000)?;
    w(0x8000_3118, 0x0400_0000)?; // MEM2 size
    w(0x8000_311C, 0x0400_0000)?;
    w(0x8000_3120, 0x9340_0000)?; // end of MEM2 for the game
    w(0x8000_3124, 0x9000_0800)?; // usable MEM2 start
    w(0x8000_3128, 0x933E_0000)?; // usable MEM2 end
    w(0x8000_3130, 0x933E_0000)?; // IOS IPC buffer
    w(0x8000_3134, 0x9340_0000)?;
    w(0x8000_3138, 0x11)?; // Hollywood revision
    w(0x8000_3140, ios << 16)?; // IOS version (revision unknown)
    w(0x8000_3158, 0x0000_FF16)?; // GDDR vendor
    w(0x8000_3180, be32(boot_bin, 0))?; // game id
    w(0x8000_3188, ios << 16)?;

    // FST at the top of MEM1, 32-byte aligned.
    let fst_addr = (0x8180_0000 - fst.len() as u32) & !0x1F;
    mem.ram_mut(fst_addr, fst.len()).ok_or("FST does not fit")?.copy_from_slice(fst);
    mem.write_u32(0x8000_0038, fst_addr).unwrap();
    mem.write_u32(0x8000_003C, fst.len() as u32).unwrap();
    Ok(())
}

impl System {
    /// Loads the game from the extracted disc folder `game`; NAND files live
    /// under `nand`.
    pub fn boot(game: &Path, nand: PathBuf) -> Result<Self, String> {
        Self::boot_with(game, nand, None, Vec::new())
    }

    /// [`System::boot`], with the menus' additions ([`menu_layout`]) if
    /// `labels` is `Some`: the pause window, the world map's menu and the
    /// save-select screen get their Settings button reading its first
    /// word, and the save files' cards their hidden text reading its second
    /// ("Cheated"). [`System::menu`] says which got theirs. `mods`: files
    /// served in place of the disc's (disc path, bytes), before the menus'
    /// additions are made to them.
    pub fn boot_with(game: &Path, nand: PathBuf, labels: Option<(&str, &str)>, mods: Vec<(String, Vec<u8>)>) -> Result<Self, String> {
        let read = |p: &str| fs::read(game.join(p)).map_err(|e| format!("{p}: {e}"));
        let (dol, boot_bin, tmd) = (read("sys/main.dol")?, read("sys/boot.bin")?, read("tmd.bin")?);
        let ios_version = tmd.get(0x18B).copied().ok_or("tmd.bin too short")? as u32;
        // A disc of no known version (a test's, a mod's own ID) runs as the
        // USA one.
        let version = Version::from_boot_bin(&boot_bin).unwrap_or(Version::E1);
        game_version::set(version);
        let addrs = Addrs::new();

        let mut ios = Ios::new(game, nand, version.title_id())?;
        let mut menu = menu_settings::MenuSettings::default();
        for (path, data) in mods {
            if !ios.disc.replace(&path, data) {
                eprintln!("mods: {path} is not on the disc (only files the disc has can be replaced)");
            }
        }
        if let Some((settings, cheated)) = labels {
            menu.buttons = menu_layout::add_buttons(&mut ios.disc, settings);
            menu.cheated_label = menu_layout::add_cheated_label(&mut ios.disc, cheated);
        }
        let fst = ios.disc.fst().to_vec();
        let hw = Rc::new(RefCell::new(Hw::new(ios)));
        let mut mem = Mem::new(Box::new(HwPort(hw.clone())));
        let entry = load_dol(&mut mem, &dol)?;
        setup_low_memory(&mut mem, &boot_bin, &fst, ios_version)?;

        let mut cpu = Cpu::new();
        cpu.pc = entry;
        cpu.msr = 0x2030; // FP available, instruction and data translation on
        cpu.spr[SPR_HID0] = 0x0011_C464;
        cpu.spr[SPR_HID4] = 0x8200_0000;
        let slices = Rc::new(RefCell::new(Slices { hw: hw.clone(), end: 0, until_retraces: 0, limit: 0, stop: false, interrupts: 0, decrementers: 0, samples: None, pending: None }));
        let memcpy_validator = (version == Version::E1
            && std::env::var("NSMBW_RUST_MEMCPY").as_deref() == Ok("validate"))
            .then(memcpy_validation::Validator::new);
        if memcpy_validator.is_some() {
            eprintln!("memcpy validation enabled for eight calls using the interpreter");
        }
        let native_memcpy = (addrs.memcpy != 0)
            .then(|| native_memcpy::NativeCopy::new(addrs.memcpy..MEMCPY_END));
        Ok(Self { memcpy_validator, native_memcpy, native_bodies: Vec::new(), cpu, mem, hw, ios_version, linked: Vec::new(), link_calls: Vec::new(), steps: 0, slices, idle_cycles: 0, blocks: Blocks::new(&addrs.special), light_textures: Default::default(), aux_effects: true, recomp: None, recomp_steps: 0, recomp_entries: 0, menu, p1: Default::default(), display: Default::default(), camera: Default::default(), joins: Default::default(), version, addrs })
    }

    /// Registers a Rust body that replaces the guest function at `addr`.
    ///
    /// Only interpreted code dispatches these; recompiled code uses the hooks
    /// baked into `game-recomp`. The switch is opt-in: callers decide when to
    /// register.
    pub fn register_native(&mut self, addr: u32, body: NativeBody) {
        self.native_bodies.retain(|(a, _)| *a != addr);
        self.native_bodies.push((addr, body));
        self.native_bodies.sort_unstable_by_key(|(a, _)| *a);
    }

    /// How many Rust bodies are registered.
    pub fn native_count(&self) -> usize {
        self.native_bodies.len()
    }

    #[inline]
    fn native_body(&self, pc: u32) -> Option<NativeBody> {
        self.native_bodies.iter().find(|(a, _)| *a == pc).map(|(_, b)| *b)
    }

    /// Runs the Rust body at `pc`, if one is registered: sets `r3` to its
    /// result and returns to `lr`. Returns whether a body ran.
    fn run_native(&mut self, pc: u32) -> bool {
        let Some(body) = self.native_body(pc) else { return false };
        let gprs = self.cpu.gpr;
        let fprs = self.cpu.fpr;
        let ret = unsafe { body(self, &gprs, &fprs) };
        self.cpu.gpr[3] = ret;
        self.cpu.pc = self.cpu.lr;
        true
    }

    /// Bookkeeping before the instruction at `pc`: `OSLink` calls and
    /// returns. Returns true when the scheduler is idle with interrupts on;
    /// the clock then jumps to the next video line and the slice should end.
    fn before(&mut self, pc: u32) -> bool {
        if self.run_native(pc) {
            return false;
        }
        let a = &self.addrs;
        if replace(&mut self.cpu, &mut self.mem, a, pc, self.aux_effects) || pc == a.strcmp {
            return false;
        }
        if self.display.watching() {
            self.display_return(pc);
        }
        if self.addrs.menu_hooks.contains(&pc) && self.menu_hook(pc) {
            return false;
        }
        // A card picks its picture; the displaced instruction runs after.
        if pc == p1_character::PORTRAIT && self.p1.on {
            self.card_portrait();
        }
        if self.addrs.display_hooks.contains(&pc) && self.display_hook(pc) {
            return false;
        }
        // The run loops execute the instruction at `pc` right after this
        // (no interrupt in between), so a call the hook lets run is drawn
        // before the hook can see it again.
        if pc == self.addrs.draw_and_capture && self.light_textures.skip(&self.cpu, &mut self.mem) {
            self.cpu.pc = self.cpu.lr;
            return false;
        }
        if pc == self.addrs.idle_loop && self.cpu.msr & MSR_EE != 0 && self.mem.read_u32(self.addrs.run_queue_bits) == Ok(0) {
            let skip = hw::CYCLES_PER_LINE - self.cpu.cycles % hw::CYCLES_PER_LINE;
            self.cpu.cycles += skip;
            self.idle_cycles += skip;
            return true;
        }
        if pc == self.addrs.bg_execute {
            self.camera.restore(&mut self.mem);
            return false;
        }
        if pc == self.addrs.bg_execute_end {
            if self.joins.returned(&mut self.cpu, &mut self.mem) {
                return false;
            }
            self.camera.apply(&mut self.mem);
            // The game's draws for this frame come after: nothing of them
            // is in the FIFO yet.
            {
                let mut hw = self.hw.borrow_mut();
                hw.gpu.set_world(Some(self.camera.drawn));
                hw.gpu.set_views(self.camera.views);
            }
            let near = camera::followed_players(&self.mem).first().copied();
            if let Some(slot) = self.joins.start(&mut self.cpu, &mut self.mem, near) {
                // The game turns page scan off in a course.
                self.hw.borrow_mut().ios.bt.force_connect(slot);
            }
            return false;
        }
        if pc == self.addrs.os_link || pc == self.addrs.os_link_fixed {
            self.link_calls.push((self.cpu.gpr[3], self.cpu.lr));
        } else if self.link_calls.last().is_some_and(|c| c.1 == pc) {
            let (header, _) = self.link_calls.pop().unwrap();
            self.record_link(header);
        }
        false
    }


    /// Advances the hardware to the CPU clock and delivers a pending
    /// external or decrementer interrupt.
    fn end_slice(&mut self) {
        self.slices.borrow_mut().end_slice(&mut self.cpu, &mut self.mem);
    }

    pub fn interrupts(&self) -> u64 {
        self.slices.borrow().interrupts
    }

    pub fn decrementers(&self) -> u64 {
        self.slices.borrow().decrementers
    }

    /// Registers recompiled code; it ends slices itself through
    /// [`Slices::end_slice`], and calls the function-replacing hooks itself
    /// ([`replace`]; set `aux_effects` first).
    pub fn set_recomp(&mut self, mut rt: guest_cpu::recomp::Rt) {
        if self.memcpy_uses_interpreter() {
            return;
        }
        let (slices, leave) = (self.slices.clone(), self.slices.clone());
        rt.set_slice_end(
            Box::new(move |c, m| {
                let mut s = slices.borrow_mut();
                s.advance(c, m).then_some(s.end)
            }),
            Box::new(move |c| leave.borrow_mut().take(c)),
        );
        let (aux_effects, addrs) = (self.aux_effects, self.addrs.clone());
        rt.set_hook(Box::new(move |c, m, pc| replace(c, m, &addrs, pc, aux_effects)));
        self.recomp = Some(rt);
    }

    /// Runs up to `n` instructions, calling `after(cpu, mem, pc)` after each
    /// one (`pc` is the instruction just executed); it returns false to stop
    /// early. Then advances the hardware and delivers a pending interrupt.
    pub fn run_slice(&mut self, n: u64, mut after: impl FnMut(&mut Cpu, &mut Mem, u32) -> bool) -> Result<bool, Stop> {
        let mut keep_going = true;
        for _ in 0..n {
            let pc = self.cpu.pc;
            if self.before(pc) {
                break;
            }
            self.cpu.step(&mut self.mem)?;
            self.steps += 1;
            if !after(&mut self.cpu, &mut self.mem, pc) {
                keep_going = false;
                break;
            }
        }
        self.end_slice();
        Ok(keep_going)
    }

    /// Runs the guest until a VI retrace (the end of a field) or until
    /// `cpu.cycles` reaches `limit`, in slices ([`SLICE`]), through the
    /// decoded-block cache and recompiled code if registered.
    pub fn run_field(&mut self, limit: u64) -> Result<(), Stop> {
        if self.memcpy_validator.is_some() {
            self.run_field_inner::<true, false>(limit)
        } else if self.native_memcpy.is_some() {
            self.run_field_inner::<false, true>(limit)
        } else {
            self.run_field_inner::<false, false>(limit)
        }
    }

    /// Whether the configured memcpy experiment uses the interpreter dispatcher.
    pub fn memcpy_uses_interpreter(&self) -> bool {
        self.memcpy_validator.is_some() || self.native_memcpy.is_some()
    }

    /// Counts only calls and bytes written by the native hook to live RAM.
    pub fn native_memcpy_commits(&self) -> Option<(u64, u64)> {
        self.native_memcpy.as_ref().map(native_memcpy::NativeCopy::commits)
    }

    fn run_field_inner<const VALIDATE_MEMCPY: bool, const NATIVE_MEMCPY: bool>(
        &mut self,
        limit: u64,
    ) -> Result<(), Stop> {
        {
            let mut s = self.slices.borrow_mut();
            s.until_retraces = self.hw.borrow().retraces;
            s.limit = limit;
            s.stop = false;
            s.end = self.cpu.cycles + SLICE;
        }
        // Whether the last instruction run was a branch: slices end only
        // there. Recompiled code checks after its own branches.
        let mut after_branch = false;
        // With recompiled code, the few instructions between an interrupt's
        // return and the next block start are stepped: decoding interpreter
        // blocks at such odd addresses costs more than running them.
        let mut probe = PROBE;
        // `Slices::end` and `stop` change only when a slice ends (here or in
        // recompiled code), so they are read back only then.
        let mut end = self.cpu.cycles + SLICE;
        loop {
            if VALIDATE_MEMCPY {
                if let Some(result) = self.memcpy_validator.as_mut().unwrap().finish(&self.cpu, &self.mem) {
                    result.report();
                }
            }
            if after_branch && self.cpu.cycles >= end {
                after_branch = false;
                if self.finish_slice(&mut end) {
                    return Ok(());
                }
                continue;
            }
            let pc = self.cpu.pc;
            if NATIVE_MEMCPY
                && pc == self.addrs.memcpy
                && self.link_calls.is_empty()
                && !self.display.watching()
            {
                let native = self.native_memcpy.as_mut().unwrap();
                if let Some(retired) = native.try_copy(&mut self.cpu, &mut self.mem, end, &self.addrs.special) {
                    self.steps += retired;
                    after_branch = true;
                    continue;
                }
            }
            if VALIDATE_MEMCPY && pc == MEMCPY.0 {
                self.memcpy_validator.as_mut().unwrap().begin(&self.cpu, &self.mem);
            }
            if (!self.link_calls.is_empty() || self.display.watching() || self.addrs.may_be_special(pc) || self.native_body(pc).is_some()) && self.before(pc) {
                // Idle: the clock jumped; end the slice here.
                if self.finish_slice(&mut end) {
                    return Ok(());
                }
                continue;
            }
            // Recompiled code from here if there is any (not while `OSLink`
            // runs: its return is watched at every block). It returns with
            // nothing run if its code changed or the FPU is off.
            if let Some(rt) = self.recomp.as_mut().filter(|_| self.link_calls.is_empty()) {
                if let Some(f) = rt.lookup(pc) {
                    self.recomp_entries += 1;
                    let before = self.cpu.cycles;
                    rt.end = end;
                    rt.depth = 0;
                    if let Err(e) = f(&mut self.cpu, &mut self.mem, rt) {
                        // At the failing instruction, as the interpreter leaves it.
                        self.cpu.pc = e.pc();
                        return Err(e);
                    }
                    let ran = self.cpu.cycles - before;
                    if ran != 0 {
                        self.steps += ran;
                        self.recomp_steps += ran;
                        after_branch = false;
                        probe = PROBE;
                        // It may have ended slices itself.
                        let s = self.slices.borrow();
                        if s.stop {
                            return Ok(());
                        }
                        end = s.end;
                        continue;
                    }
                    after_branch = self.step_one()?;
                    continue;
                }
                if probe > 0 {
                    probe -= 1;
                    after_branch = self.step_one()?;
                    continue;
                }
            }
            let (n, branch) = self.cpu.run_block(&mut self.mem, &mut self.blocks)?;
            self.steps += u64::from(n);
            after_branch = branch;
        }
    }

    /// Ends the slice; `end` becomes the next one's. True if the run is over.
    fn finish_slice(&mut self, end: &mut u64) -> bool {
        if let Some(validator) = &mut self.memcpy_validator {
            validator.cancel_at_slice_boundary();
        }
        let mut s = self.slices.borrow_mut();
        s.end_slice(&mut self.cpu, &mut self.mem);
        *end = s.end;
        s.stop
    }

    /// One interpreted instruction; true if it was a branch.
    fn step_one(&mut self) -> Result<bool, Stop> {
        let branch = self.mem.read_u32(self.cpu.pc).is_ok_and(guest_cpu::is_branch);
        self.cpu.step(&mut self.mem)?;
        self.steps += 1;
        Ok(branch)
    }

    /// Calls guest function `addr` with `args` in r3.. from the host, between
    /// fields, and returns its r3; the CPU is then put back as it was (the
    /// clock keeps the time the call took). For tools (`--course`) and the
    /// Settings window's Power-Up: the call must not wait or switch threads.
    ///
    /// It runs with interrupts off and the FPU on (so no exception can reach
    /// the guest OS), on a stack in MEM2 above the game's, which nothing
    /// else uses. `data` is copied to 0x80001800 first, for pointer
    /// arguments: low memory the game leaves unused (debuggers and cheat
    /// handlers live there on a real Wii).
    pub fn call_guest(&mut self, addr: u32, args: &[u32], data: &[u8]) -> Result<u32, String> {
        assert!(args.len() <= 8 && data.len() <= CALL_SCRATCH_CAPACITY);
        self.call_guest_with_registers(addr, args, &[], data)
            .map(|result| result.r3)
    }

    /// [`System::call_guest`] for a function that touches the OS's threads
    /// (locks and unlocks a mutex, signals a condition) and needs a big
    /// stack, such as the game's `SaveGame`, whose NAND copy takes 16 KiB.
    /// It runs as a thread of its own that is never switched out: the OS's
    /// current thread is a blank one at 0x80001800 and the scheduler is held
    /// as `OSDisableScheduler` holds it (a thread it wakes runs at the next
    /// reschedule), on [`System::call_guest`]'s 64 KiB stack.
    /// `None`, and nothing run, where the guest stopped with interrupts off:
    /// in a critical section, whose OS state may be half updated.
    pub fn call_guest_as_thread(&mut self, addr: u32, args: &[u32]) -> Option<Result<u32, String>> {
        // Interrupts on, or one just taken (only ever with them on) and its
        // handler not started.
        let just_taken = matches!(self.cpu.pc, 0x500 | 0x900) && self.cpu.spr[guest_cpu::SPR_SRR1] & MSR_EE != 0;
        if self.cpu.msr & MSR_EE == 0 && !just_taken {
            return None;
        }
        let reschedule = RESCHEDULE.va();
        let (thread, held) = (self.mem.read_u32(CURRENT_THREAD).ok()?, self.mem.read_u32(reschedule).ok()?);
        let _ = self.mem.write_u32(CURRENT_THREAD, CALL_SCRATCH);
        let _ = self.mem.write_u32(reschedule, held.wrapping_add(1));
        let r = self.call_guest(addr, args, &[0; OS_THREAD_SIZE]);
        let _ = self.mem.write_u32(CURRENT_THREAD, thread);
        let _ = self.mem.write_u32(reschedule, held);
        Some(r)
    }

    /// Resolve a range in a live REL section. Code addresses require an
    /// executable section and room for a full aligned PowerPC instruction.
    pub fn resolve_rel_address(
        &self,
        address: RelAddress,
        len: usize,
        executable: bool,
    ) -> Option<u32> {
        resolve_rel_address_in_memory(&self.mem, address, len, executable)
    }

    /// Runs the same synchronous guest call as [`Self::call_guest`] with
    /// register arguments for a fixed-prototype call. Integer words go to
    /// r3–r10 and scalar floating-point values go to f1–f8. The caller lays
    /// out register pairs. Stack arguments are unsupported.
    pub fn call_guest_with_registers(
        &mut self,
        addr: u32,
        gprs: &[u32],
        fprs: &[f64],
        data: &[u8],
    ) -> Result<GuestCallResult, String> {
        self.invoke_guest_call(addr, gprs, fprs, data, CALL_STACK_TOP)
    }

    fn invoke_guest_call(
        &mut self,
        addr: u32,
        gprs: &[u32],
        fprs: &[f64],
        data: &[u8],
        stack_top: u32,
    ) -> Result<GuestCallResult, String> {
        const RETURN: u32 = CALL_SCRATCH; // never executed: the call stops there
        if gprs.len() > 8 || fprs.len() > 8 || data.len() > CALL_SCRATCH_CAPACITY {
            return Err("guest call exceeds the argument registers or scratch area".into());
        }
        let saved = self.cpu.clone();
        self.mem.ram_mut(CALL_SCRATCH, data.len()).ok_or("scratch memory")?.copy_from_slice(data);
        let _ = self.mem.write_u32(stack_top, 0); // end of the back chain
        self.cpu.gpr[1] = stack_top;
        self.cpu.gpr[3..3 + gprs.len()].copy_from_slice(gprs);
        for (register, &value) in self.cpu.fpr[1..].iter_mut().zip(fprs) {
            register[0] = value;
        }
        self.cpu.lr = RETURN;
        self.cpu.pc = addr;
        self.cpu.msr = (self.cpu.msr & !MSR_EE) | guest_cpu::MSR_FP;
        let mut result = Err(format!("call to {addr:#010x} did not return"));
        for _ in 0..1000 {
            match self.run_slice(10_000, |cpu, _, _| cpu.pc != RETURN) {
                Ok(_) if self.cpu.pc == RETURN => {
                    result = Ok(GuestCallResult {
                        r3: self.cpu.gpr[3],
                        r4: self.cpu.gpr[4],
                        f1: self.cpu.fpr[1][0],
                    });
                    break;
                }
                Ok(_) => {}
                Err(e) => {
                    result = Err(format!("call to {addr:#010x} stopped: {e:?}"));
                    break;
                }
            }
        }
        let cycles = self.cpu.cycles;
        self.cpu = saved;
        self.cpu.cycles = cycles;
        result
    }

    fn record_link(&mut self, header: u32) {
        let m = &mut self.mem;
        let id = m.read_u32(header).unwrap_or(0);
        let count = m.read_u32(header + 0xC).unwrap_or(0).min(64);
        let table = m.read_u32(header + 0x10).unwrap_or(0);
        let sections: Vec<(u32, u32, bool)> = (0..count)
            .filter_map(|i| {
                let off = m.read_u32(table + 8 * i).ok()?;
                let size = m.read_u32(table + 8 * i + 4).ok()?;
                (size != 0).then_some((off & !1, size, off & 1 != 0))
            })
            .collect();
        let ok = self.cpu.gpr[3] != 0;
        if ok {
            self.p1_linked(&sections);
        }
        self.linked.push(LinkedModule { id, header, ok, sections, at_step: self.steps });
    }
}

impl GuestRuntime for System {
    fn memory(&mut self) -> &mut Mem {
        &mut self.mem
    }

    fn resolve_rel_address(&self, address: RelAddress, len: usize, executable: bool) -> Option<u32> {
        System::resolve_rel_address(self, address, len, executable)
    }

    fn call_registers(
        &mut self,
        addr: u32,
        gprs: &[u32],
        fprs: &[f64],
    ) -> Result<GuestCallResult, String> {
        self.call_guest_with_registers(addr, gprs, fprs, &[])
    }

    fn call_frame_with_scratch(
        &mut self,
        addr: u32,
        gprs: &[GuestWord],
        fprs: &[f64],
        stack: &[u8],
        scratch: &mut [u8],
    ) -> Result<GuestCallResult, String> {
        let len = scratch.len();
        if gprs.len() > 8 || fprs.len() > 8 || len > CALL_SCRATCH_CAPACITY {
            return Err("guest call exceeds the argument registers or scratch area".into());
        }
        if stack.len() > CALL_STACK_ARG_CAPACITY {
            return Err("guest call exceeds the stack argument area".into());
        }
        let mut words = [0; 8];
        for (word, arg) in words.iter_mut().zip(gprs) {
            *word = match *arg {
                GuestWord::Immediate(value) => value,
                GuestWord::ScratchOffset(offset) if offset < len => CALL_SCRATCH + offset as u32,
                GuestWord::ScratchOffset(_) => {
                    return Err("guest scratch offset is outside the buffer".into());
                }
            };
        }

        let mut saved = [0; CALL_SCRATCH_CAPACITY];
        let saved = &mut saved[..len];
        saved.copy_from_slice(self.mem.ram(CALL_SCRATCH, len).ok_or("scratch memory")?);
        let stack_top = CALL_STACK_TOP - stack.len().next_multiple_of(16) as u32;
        let mut stack_backup = [0; CALL_STACK_ARG_CAPACITY + CALL_STACK_LINKAGE];
        let frame_len = if stack.is_empty() {
            0
        } else {
            CALL_STACK_LINKAGE + stack.len()
        };
        let stack_backup = &mut stack_backup[..frame_len];
        if frame_len != 0 {
            let frame = self
                .mem
                .ram_mut(stack_top, frame_len)
                .ok_or("guest call stack")?;
            stack_backup.copy_from_slice(frame);
            frame[CALL_STACK_LINKAGE..].copy_from_slice(stack);
        }
        let result = self.invoke_guest_call(addr, &words[..gprs.len()], fprs, scratch, stack_top);
        if frame_len != 0 {
            self.mem
                .ram_mut(stack_top, frame_len)
                .ok_or("guest call stack")?
                .copy_from_slice(stack_backup);
        }
        let ram = self.mem.ram_mut(CALL_SCRATCH, len).ok_or("scratch memory")?;
        if result.is_ok() {
            scratch.copy_from_slice(ram);
        }
        ram.copy_from_slice(saved);
        result
    }
}

use guest_mem::snap::{Snap, Snapshot};

impl Snapshot for LinkedModule {
    fn snap(&mut self, s: &mut Snap) {
        guest_mem::snap!(s; self.id, self.header, self.ok, self.sections, self.at_step);
    }
}

impl Default for LinkedModule {
    fn default() -> Self {
        Self { id: 0, header: 0, ok: false, sections: Vec::new(), at_step: 0 }
    }
}

/// First bytes of a save state; the number changes with the layout.
const STATE_MAGIC: &[u8; 8] = b"NSMBWst7";

impl System {
    /// The whole machine as bytes (docs/PORT_PLAN.md, section 12): CPU, RAM,
    /// hardware, IOS, GX registers and the HLE state. Decoded blocks are
    /// left out (rebuilt), and NAND files are on disk, not in the state.
    ///
    /// Light-texture keys are forgotten first, here and in the state, so the
    /// next redraw runs on the saving machine and on a loader alike, and a
    /// loader gets its textures (which live on the host GPU) back.
    pub fn save_state(&mut self) -> Vec<u8> {
        self.light_textures.forget();
        let mut s = Snap::saver();
        self.snap_all(&mut s);
        s.into_bytes()
    }

    /// Loads a state from [`System::save_state`] into a booted system.
    pub fn load_state(&mut self, bytes: Vec<u8>) -> Result<(), String> {
        let disc = self.mem.ram(0x8000_0000, 8).map(<[u8]>::to_vec);
        let mut s = Snap::loader(bytes);
        self.snap_all(&mut s);
        self.display.forget();
        s.finish()?;
        // The disc header the state's game booted from: another version's
        // state cannot run on this one's code.
        let state = self.mem.ram(0x8000_0000, 8).map(<[u8]>::to_vec);
        if let (Some(d), Some(st)) = (&disc, &state) {
            if d[..6] != st[..6] || d[7] != st[7] {
                return Err(format!("this state is from {} revision {}, not this disc ({} revision {})", String::from_utf8_lossy(&st[..6]), st[7], String::from_utf8_lossy(&d[..6]), d[7]));
            }
        }
        Ok(())
    }

    fn snap_all(&mut self, s: &mut Snap) {
        s.tag(&STATE_MAGIC[..4].try_into().unwrap());
        s.tag(&STATE_MAGIC[4..].try_into().unwrap());
        guest_mem::snap!(s; self.cpu, self.mem);
        self.hw.borrow_mut().snap(s);
        {
            let mut sl = self.slices.borrow_mut();
            guest_mem::snap!(s; self.linked, self.link_calls, self.steps, sl.interrupts, sl.decrementers, self.idle_cycles, self.light_textures, self.camera, self.joins);
        }
        s.tag(b"END ");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_memcpy_dispatches_for_disjoint_ram_and_declines_aliases() {
        let addrs = Addrs::with_memcpy(true);
        let entry = 0x8000_4364;
        assert!(addrs.special.contains(&entry));
        assert!(addrs.may_be_special(entry));
        assert!(!Addrs::with_memcpy(false).special.contains(&entry));

        let mut cpu = Cpu::new();
        let mut mem = Mem::new(Box::new(guest_mem::NullMmio));
        let mut rt =
            guest_cpu::recomp::Rt::new(&[], &mut mem, &addrs.special, &addrs.special).unwrap();
        let special = addrs.special.clone();
        rt.set_hook(Box::new(move |c, m, pc| replace(c, m, &addrs, pc, true)));

        let src = 0x8000_2000;
        let dst = 0x8000_2010;
        mem.write_u32(src, 0x1122_3344).unwrap();
        cpu.pc = entry;
        cpu.lr = 0x8000_1000;
        cpu.gpr[3] = dst;
        cpu.gpr[4] = src;
        cpu.gpr[5] = 4;

        assert!(!rt.hook(&mut cpu, &mut mem, entry));
        assert_eq!(mem.read_u32(dst), Ok(0));
        assert_eq!(cpu.pc, entry);
        // lwz r6,0(r4); stw r6,0(r3); blr.
        mem.write_u32(entry, (32 << 26) | (6 << 21) | (4 << 16)).unwrap();
        mem.write_u32(entry + 4, (36 << 26) | (6 << 21) | (3 << 16)).unwrap();
        mem.write_u32(entry + 8, 0x4e80_0020).unwrap();
        let mut native = native_memcpy::NativeCopy::new(entry..entry + 12);
        assert_eq!(native.try_copy(&mut cpu, &mut mem, 3, &special), Some(3));
        assert_eq!(mem.read_u32(dst), Ok(0x1122_3344));
        assert_eq!(cpu.gpr[3], dst);
        assert_eq!(cpu.pc, cpu.lr);

        cpu.pc = entry;
        cpu.gpr[3] = src + 1;
        cpu.gpr[4] = 0xc000_2000;
        assert_eq!(native.try_copy(&mut cpu, &mut mem, 20, &special), None);
        assert_eq!(cpu.pc, entry);
        assert_eq!(cpu.gpr[3], src + 1);
        assert_eq!(mem.read_u32(src), Ok(0x1122_3344));
    }

    #[test]
    fn rel_address_uses_original_section_indices_and_live_module_membership() {
        let mut mem = Mem::new(Box::new(guest_mem::NullMmio));
        let first = 0x8000_9000;
        let module = 0x8000_9100;
        let table = 0x8000_a000;
        let code = 0x8000_b000;
        let data = 0x9000_c000;

        mem.write_u32(0x8000_30c8, first).unwrap();
        mem.write_u32(first, 3).unwrap();
        mem.write_u32(first + 4, module).unwrap();
        mem.write_u32(module, 7).unwrap();
        mem.write_u32(module + 8, first).unwrap();
        mem.write_u32(module + 12, 4).unwrap();
        mem.write_u32(module + 16, table).unwrap();
        // Section 0 and section 2 are empty. Indices 1 and 3 are stable.
        mem.write_u32(table + 8, code | 1).unwrap();
        mem.write_u32(table + 12, 16).unwrap();
        mem.write_u32(table + 24, data).unwrap();
        mem.write_u32(table + 28, 32).unwrap();

        let addr = RelAddress {
            module: 7,
            section: 1,
            offset: 12,
        };
        assert_eq!(
            resolve_rel_address_in_memory(&mem, addr, 4, true),
            Some(code + 12)
        );
        assert_eq!(resolve_rel_address_in_memory(&mem, addr, 5, true), None);
        assert_eq!(
            resolve_rel_address_in_memory(&mem, RelAddress { offset: 1, ..addr }, 4, true),
            None
        );
        assert_eq!(
            resolve_rel_address_in_memory(&mem, RelAddress { section: 2, ..addr }, 1, false),
            None
        );
        let data_addr = RelAddress {
            section: 3,
            offset: 31,
            ..addr
        };
        assert_eq!(
            resolve_rel_address_in_memory(&mem, data_addr, 0, false),
            Some(data + 31)
        );
        assert_eq!(
            resolve_rel_address_in_memory(&mem, data_addr, 2, false),
            None
        );
        assert_eq!(
            resolve_rel_address_in_memory(
                &mem,
                RelAddress {
                    offset: 0,
                    ..data_addr
                },
                4,
                true
            ),
            None
        );
        assert_eq!(
            resolve_rel_address_in_memory(
                &mem,
                RelAddress {
                    section: u32::MAX,
                    ..addr
                },
                1,
                false
            ),
            None
        );

        // Detaching the module leaves its bytes in RAM but removes its address.
        mem.write_u32(first + 4, 0).unwrap();
        assert_eq!(resolve_rel_address_in_memory(&mem, addr, 4, true), None);
    }

    #[test]
    fn rel_address_declines_invalid_list_and_section_pointers() {
        let mut mem = Mem::new(Box::new(guest_mem::NullMmio));
        let module = 0x8000_9100;
        let address = RelAddress {
            module: 7,
            section: 0,
            offset: 0,
        };
        mem.write_u32(0x8000_30c8, module).unwrap();
        mem.write_u32(module, 7).unwrap();
        mem.write_u32(module + 12, 1).unwrap();
        mem.write_u32(module + 16, 0xcc00_0000).unwrap();
        assert_eq!(resolve_rel_address_in_memory(&mem, address, 4, false), None);

        mem.write_u32(module, 6).unwrap();
        mem.write_u32(module + 4, module).unwrap();
        assert_eq!(resolve_rel_address_in_memory(&mem, address, 4, false), None);
    }
}
