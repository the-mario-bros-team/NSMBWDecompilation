//! OS-level HLE (docs/PORT_PLAN.md, section 2.3): direct boot of `main.dol`
//! the way the apploader leaves it, the run loop that delivers external and
//! decrementer interrupts between instruction slices, and the `OSLink` hook
//! that records where each REL module was placed.

pub mod camera;
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
use guest_cpu::{Cpu, Stop, MSR_EE, SPR_HID0, SPR_HID4};
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
/// Its `errno` (r13 - 0x4698) and the NaN it returns for negative inputs.
const ERRNO: E1 = E1(0x8042_AFE8);
const EDOM: u32 = 0x21;
/// [`System::call_guest`]'s data, in low memory the game leaves unused
/// (up to [`p1_character::STUBS`]), and its 64 KiB stack in MEM2 above the
/// game's (from 0x93400000: IOS's own memory on a real Wii, which nothing
/// uses here).
const CALL_SCRATCH: u32 = 0x8000_1800;
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
        let mut missing = Vec::new();
        let mut va = |a: u32| va_or_none(E1(a), &mut missing);
        let return_at_once = RETURN_AT_ONCE.map(&mut va);
        let menu_hooks = menu_settings::HOOKS.map(|h| va(h.0));
        let display_hooks = display::HOOKS.map(|h| va(h.0));
        let mut a = Self {
            return_at_once,
            sqrt: va(SQRT.0),
            strcmp: va(STRCMP.0),
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
        special.extend([a.sqrt, a.strcmp, a.draw_and_capture, a.update_buffer, a.idle_loop, a.os_link, a.os_link_fixed, p1_character::PORTRAIT, a.bg_execute, a.bg_execute_end]);
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
/// [`guest_cpu::recomp::Rt::new`] checks it).
pub fn special() -> Vec<u32> {
    Addrs::new().special
}

/// The hooks that replace a whole function: return at once (DC loops,
/// AXFX callbacks), the native reverb, [`STRCMP`] and [`SQRT`]. Runs the
/// one at `pc` if it applies, returning to `lr`; true if it ran. One that
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

pub struct System {
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
        Ok(Self { cpu, mem, hw, ios_version, linked: Vec::new(), link_calls: Vec::new(), steps: 0, slices, idle_cycles: 0, blocks: Blocks::new(&addrs.special), light_textures: Default::default(), aux_effects: true, recomp: None, recomp_steps: 0, recomp_entries: 0, menu, p1: Default::default(), display: Default::default(), camera: Default::default(), joins: Default::default(), version, addrs })
    }

    /// Bookkeeping before the instruction at `pc`: `OSLink` calls and
    /// returns. Returns true when the scheduler is idle with interrupts on;
    /// the clock then jumps to the next video line and the slice should end.
    fn before(&mut self, pc: u32) -> bool {
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
            if after_branch && self.cpu.cycles >= end {
                after_branch = false;
                if self.finish_slice(&mut end) {
                    return Ok(());
                }
                continue;
            }
            let pc = self.cpu.pc;
            if (!self.link_calls.is_empty() || self.display.watching() || self.addrs.may_be_special(pc)) && self.before(pc) {
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
        self.call_guest_on(addr, args, data)
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
        let r = self.call_guest_on(addr, args, &[0; OS_THREAD_SIZE]);
        let _ = self.mem.write_u32(CURRENT_THREAD, thread);
        let _ = self.mem.write_u32(reschedule, held);
        Some(r)
    }

    fn call_guest_on(&mut self, addr: u32, args: &[u32], data: &[u8]) -> Result<u32, String> {
        const RETURN: u32 = CALL_SCRATCH; // never executed: the call stops there
        assert!(args.len() <= 8 && data.len() <= 0x400);
        let saved = self.cpu.clone();
        self.mem.ram_mut(CALL_SCRATCH, data.len()).ok_or("scratch memory")?.copy_from_slice(data);
        let _ = self.mem.write_u32(CALL_STACK_TOP, 0); // end of the back chain
        self.cpu.gpr[1] = CALL_STACK_TOP;
        for (i, &a) in args.iter().enumerate() {
            self.cpu.gpr[3 + i] = a;
        }
        self.cpu.lr = RETURN;
        self.cpu.pc = addr;
        self.cpu.msr = (self.cpu.msr & !MSR_EE) | guest_cpu::MSR_FP;
        let mut result = Err(format!("call to {addr:#010x} did not return"));
        for _ in 0..1000 {
            match self.run_slice(10_000, |cpu, _, _| cpu.pc != RETURN) {
                Ok(_) if self.cpu.pc == RETURN => {
                    result = Ok(self.cpu.gpr[3]);
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
