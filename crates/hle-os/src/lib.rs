//! OS-level HLE (docs/PORT_PLAN.md, section 2.3): direct boot of `main.dol`
//! the way the apploader leaves it, the run loop that delivers external and
//! decrementer interrupts between instruction slices, and the `OSLink` hook
//! that records where each REL module was placed.

pub mod camera;
pub mod join;
pub mod light_texture;
pub mod reverb;

use std::cell::RefCell;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use guest_cpu::blocks::Blocks;
use guest_cpu::{Cpu, Stop, MSR_EE, SPR_HID0, SPR_HID4};
use guest_mem::Mem;
use hle_ios::Ios;
use hw::{Hw, HwPort};

/// `OSLink(module, bss)` and `OSLinkFixed(module, bss)` in the USA rev 1 DOL.
pub const OS_LINK: u32 = 0x801B_1F20;
pub const OS_LINK_FIXED: u32 = 0x801B_1F30;
/// Title ID of the USA release (`SMNE`).
pub const TITLE_ID: u64 = 0x0001_0000_534D_4E45;
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
const IDLE_LOOP: u32 = 0x801B_4FF0;
const RUN_QUEUE_BITS: u32 = 0x8042_A708;
/// Functions answered on the host with an immediate return:
/// - the data-cache range loops (`DCInvalidateRange`, `DCFlushRange`,
///   `DCStoreRange`): cache operations do nothing in this emulator, and the
///   loops cost a few percent of guest time;
/// - the AX aux-bus effect callbacks (`AXFXReverbHi`, `ReverbStd`, `Delay`
///   and `Chorus`, stereo and Dolby Pro Logic II): their output only reaches
///   the speakers, which P5 adds (they will run natively then). In menus they
///   were over a third of guest time, and the delay on the world map 2-3%.
/// The AXFX callbacks in [`RETURN_AT_ONCE`] lie in this range.
const AXFX: std::ops::Range<u32> = 0x801A_3450..0x801A_9400;
const RETURN_AT_ONCE: [u32; 12] = [
    0x801A_C440, 0x801A_C470, 0x801A_C4A0, // DC range loops
    0x801A_3450, 0x801A_3880, 0x801A_48F0, // AXFXReverbHi
    0x801A_5C10, 0x801A_6580, // AXFXDelay
    0x801A_6D30, 0x801A_7A80, // AXFXReverbStd
    0x801A_88B0, 0x801A_93F0, // AXFXChorus
];
/// fdlibm's bit-by-bit `sqrt` (`__ieee754_sqrt`), computed natively: it is
/// correctly rounded, so `f64::sqrt` gives the same bits.
const SQRT: u32 = 0x802E_8620;
/// MSL's `strcmp`, computed natively (the first differing bytes' difference,
/// as unsigned chars). Scene loads compare every effect name with it.
const STRCMP: u32 = 0x802E_1AB4;
/// Its `errno` (r13 - 0x4698) and the NaN it returns for negative inputs.
const ERRNO_OFFSET: u32 = 0x4698;
const EDOM: u32 = 0x21;

/// Every address [`System::before`] acts on (`OSLink` returns aside).
/// Recompiled code never runs through one of these (`ppc-recomp` bakes this
/// list in, and [`guest_cpu::recomp::Rt::new`] checks it).
pub const SPECIAL: [u32; 21] = {
    let mut s = [0u32; 21];
    let mut i = 0;
    while i < RETURN_AT_ONCE.len() {
        s[i] = RETURN_AT_ONCE[i];
        i += 1;
    }
    let rest = [SQRT, STRCMP, light_texture::DRAW_AND_CAPTURE, reverb::UPDATE_BUFFER, IDLE_LOOP, OS_LINK, OS_LINK_FIXED, camera::BG_EXECUTE, camera::BG_EXECUTE_END];
    let mut j = 0;
    while j < rest.len() {
        s[i + j] = rest[j];
        j += 1;
    }
    s
};

/// Bit `(pc >> 2) & 255` set for each [`SPECIAL`] address.
const SPECIAL_FILTER: [u64; 4] = {
    let mut f = [0u64; 4];
    let mut i = 0;
    while i < SPECIAL.len() {
        let b = (SPECIAL[i] >> 2) & 255;
        f[(b >> 6) as usize] |= 1 << (b & 63);
        i += 1;
    }
    f
};

#[inline(always)]
fn may_be_special(pc: u32) -> bool {
    let b = (pc >> 2) & 255;
    SPECIAL_FILTER[(b >> 6) as usize] & (1 << (b & 63)) != 0
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
    /// Instructions run by recompiled code (the rest were interpreted).
    pub recomp_steps: u64,
    /// The shared camera for online play, and this PC's view.
    pub camera: camera::SharedCamera,
    /// Players joining a course in progress.
    pub joins: join::Joins,
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
}

impl Slices {
    /// Ends the slice: the hardware runs up to the CPU clock, and a pending
    /// external or decrementer interrupt is taken. Returns false if code
    /// running natively must return to the dispatcher (an interrupt was
    /// taken, or the run is over).
    pub fn end_slice(&mut self, cpu: &mut Cpu, mem: &mut Mem) -> bool {
        let mut hw = self.hw.borrow_mut();
        hw.advance(cpu.cycles, mem);
        let mut taken = false;
        if cpu.msr & MSR_EE != 0 {
            if hw.interrupt_pending() {
                self.interrupts += 1;
                cpu.exception(0x500, cpu.pc);
                taken = true;
            } else if cpu.take_decrementer() {
                self.decrementers += 1;
                taken = true;
            }
        }
        if hw.retraces > self.until_retraces || cpu.cycles >= self.limit {
            self.stop = true;
        }
        drop(hw);
        if let Some(s) = &mut self.samples {
            *s.entry(cpu.pc).or_default() += 1;
        }
        self.end = cpu.cycles + SLICE;
        !taken && !self.stop
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
        let read = |p: &str| fs::read(game.join(p)).map_err(|e| format!("{p}: {e}"));
        let (dol, fst, boot_bin, tmd) = (read("sys/main.dol")?, read("sys/fst.bin")?, read("sys/boot.bin")?, read("tmd.bin")?);
        let ios_version = tmd.get(0x18B).copied().ok_or("tmd.bin too short")? as u32;

        let hw = Rc::new(RefCell::new(Hw::new(Ios::new(game, nand, TITLE_ID)?)));
        let mut mem = Mem::new(Box::new(HwPort(hw.clone())));
        let entry = load_dol(&mut mem, &dol)?;
        setup_low_memory(&mut mem, &boot_bin, &fst, ios_version)?;

        let mut cpu = Cpu::new();
        cpu.pc = entry;
        cpu.msr = 0x2030; // FP available, instruction and data translation on
        cpu.spr[SPR_HID0] = 0x0011_C464;
        cpu.spr[SPR_HID4] = 0x8200_0000;
        let slices = Rc::new(RefCell::new(Slices { hw: hw.clone(), end: 0, until_retraces: 0, limit: 0, stop: false, interrupts: 0, decrementers: 0, samples: None }));
        Ok(Self { cpu, mem, hw, ios_version, linked: Vec::new(), link_calls: Vec::new(), steps: 0, slices, idle_cycles: 0, blocks: Blocks::new(&SPECIAL), light_textures: Default::default(), aux_effects: true, recomp: None, recomp_steps: 0, camera: Default::default(), joins: Default::default() })
    }

    /// Bookkeeping before the instruction at `pc`: `OSLink` calls and
    /// returns. Returns true when the scheduler is idle with interrupts on;
    /// the clock then jumps to the next video line and the slice should end.
    fn before(&mut self, pc: u32) -> bool {
        if RETURN_AT_ONCE.contains(&pc) && (!self.aux_effects || !AXFX.contains(&pc)) {
            self.cpu.pc = self.cpu.lr;
            return false;
        }
        if pc == reverb::UPDATE_BUFFER && self.aux_effects && reverb::update_buffer(&mut self.mem, self.cpu.gpr[3], self.cpu.gpr[5], self.cpu.gpr[8]) {
            self.cpu.pc = self.cpu.lr;
            return false;
        }
        if pc == STRCMP {
            if let Some(r) = self.strcmp(self.cpu.gpr[3], self.cpu.gpr[4]) {
                self.cpu.gpr[3] = r as u32;
                self.cpu.pc = self.cpu.lr;
            }
            return false;
        }
        // The run loops execute the instruction at `pc` right after this
        // (no interrupt in between), so a call the hook lets run is drawn
        // before the hook can see it again.
        if pc == light_texture::DRAW_AND_CAPTURE && self.light_textures.skip(&self.cpu, &mut self.mem) {
            self.cpu.pc = self.cpu.lr;
            return false;
        }
        // With the FPU off, f1 may still be another thread's: let the real
        // code run and take the FP-unavailable exception first.
        if pc == SQRT && self.cpu.msr & guest_cpu::MSR_FP != 0 {
            self.cpu.fpr[1][0] = self.sqrt(self.cpu.fpr[1][0]);
            self.cpu.pc = self.cpu.lr;
            return false;
        }
        if pc == IDLE_LOOP && self.cpu.msr & MSR_EE != 0 && self.mem.read_u32(RUN_QUEUE_BITS) == Ok(0) {
            let skip = hw::CYCLES_PER_LINE - self.cpu.cycles % hw::CYCLES_PER_LINE;
            self.cpu.cycles += skip;
            self.idle_cycles += skip;
            return true;
        }
        if pc == camera::BG_EXECUTE {
            self.camera.restore(&mut self.mem);
            return false;
        }
        if pc == camera::BG_EXECUTE_END {
            if self.joins.returned(&mut self.cpu, &mut self.mem) {
                return false;
            }
            self.camera.apply(&mut self.mem);
            // The game's draws for this frame come after: nothing of them
            // is in the FIFO yet.
            if self.camera.view_player.is_some() {
                self.hw.borrow_mut().gpu.set_view(self.camera.view);
            }
            let near = camera::followed_players(&self.mem).first().copied();
            self.joins.start(&mut self.cpu, &mut self.mem, near);
            return false;
        }
        if pc == OS_LINK || pc == OS_LINK_FIXED {
            self.link_calls.push((self.cpu.gpr[3], self.cpu.lr));
        } else if self.link_calls.last().is_some_and(|c| c.1 == pc) {
            let (header, _) = self.link_calls.pop().unwrap();
            self.record_link(header);
        }
        false
    }

    /// [`STRCMP`]'s result, or `None` (run the guest's own) if a string
    /// leaves RAM.
    fn strcmp(&self, a: u32, b: u32) -> Option<i32> {
        for i in 0.. {
            let x = self.mem.ram(a.wrapping_add(i), 1)?[0];
            let y = self.mem.ram(b.wrapping_add(i), 1)?[0];
            if x != y {
                return Some(i32::from(x) - i32::from(y));
            }
            if x == 0 {
                return Some(0);
            }
        }
        unreachable!()
    }

    /// [`SQRT`]'s result, with its special cases and `errno`.
    fn sqrt(&mut self, x: f64) -> f64 {
        let errno = self.cpu.gpr[13].wrapping_sub(ERRNO_OFFSET);
        if !x.is_finite() {
            // NaN stays NaN, +inf stays +inf, -inf becomes NaN.
            let _ = self.mem.write_u32(errno, EDOM);
            return x + x * x;
        }
        if x == 0.0 {
            return x;
        }
        if x < 0.0 {
            let _ = self.mem.write_u32(errno, EDOM);
            return f64::from_bits(0x7FFF_FFFF_0000_0000);
        }
        x.sqrt()
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
    /// [`Slices::end_slice`].
    pub fn set_recomp(&mut self, mut rt: guest_cpu::recomp::Rt) {
        let slices = self.slices.clone();
        rt.set_slice_end(Box::new(move |c, m| {
            let mut s = slices.borrow_mut();
            s.end_slice(c, m).then_some(s.end)
        }));
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
            if (!self.link_calls.is_empty() || may_be_special(pc)) && self.before(pc) {
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

    fn record_link(&mut self, header: u32) {
        let m = &mut self.mem;
        let id = m.read_u32(header).unwrap_or(0);
        let count = m.read_u32(header + 0xC).unwrap_or(0).min(64);
        let table = m.read_u32(header + 0x10).unwrap_or(0);
        let sections = (0..count)
            .filter_map(|i| {
                let off = m.read_u32(table + 8 * i).ok()?;
                let size = m.read_u32(table + 8 * i + 4).ok()?;
                (size != 0).then_some((off & !1, size, off & 1 != 0))
            })
            .collect();
        let ok = self.cpu.gpr[3] != 0;
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
const STATE_MAGIC: &[u8; 8] = b"NSMBWst5";

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
        let mut s = Snap::loader(bytes);
        self.snap_all(&mut s);
        s.finish()
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
