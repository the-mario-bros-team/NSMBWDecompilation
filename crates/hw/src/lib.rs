//! Hollywood/Flipper hardware registers for the P2 boot (docs/PORT_PLAN.md):
//! the processor interface (interrupt cause/mask, CPU FIFO), video timing
//! interrupts, the IPC registers that carry IOS requests, DSP/AI, EXI/SI,
//! and the CP/PE registers in front of the `gx` command decoder.
//!
//! Register writes that need memory (DMA, IOS requests, the gather pipe) are
//! queued and carried out by [`Hw::advance`], which the run loop calls every
//! few hundred instructions with the current cycle count.

pub mod ax;
mod dsp;
mod exi;

use std::collections::{HashMap, VecDeque};

use guest_mem::Mem;
use hle_ios::Ios;

pub use dsp::Dsp;

// Processor-interface interrupt causes.
pub const INT_SI: u32 = 0x8;
pub const INT_EXI: u32 = 0x10;
pub const INT_AI: u32 = 0x20;
pub const INT_DSP: u32 = 0x40;
pub const INT_VI: u32 = 0x100;
pub const INT_PE_TOKEN: u32 = 0x200;
pub const INT_PE_FINISH: u32 = 0x400;
pub const INT_IPC: u32 = 0x4000;
/// Reset button state (1 = released); a level, not an interrupt.
const INT_RESET_BUTTON: u32 = 0x10000;
/// Hollywood IRQ flag for IPC from IOS to Broadway.
const HW_IRQ_IPC: u32 = 0x4000_0000;

/// Bus cycles (the CPU counter) per second.
const CPU_HZ: u64 = 729_000_000;
/// NTSC: 525 lines per frame at 29.97 frames/s (two fields, 59.94 Hz).
const LINES_PER_FRAME: u64 = 525;
pub const CYCLES_PER_LINE: u64 = CPU_HZ * 1001 / (30_000 * LINES_PER_FRAME);

#[derive(Default)]
struct Ipc {
    ppcmsg: u32,
    armmsg: u32,
    x1: bool,
    x2: bool,
    y1: bool,
    y2: bool,
    iy1: bool,
    iy2: bool,
    requests: VecDeque<u32>,
    replies: VecDeque<u32>,
}

impl Ipc {
    fn ctrl(&self) -> u32 {
        (self.iy2 as u32) << 5 | (self.iy1 as u32) << 4 | (self.x2 as u32) << 3 | (self.y1 as u32) << 2 | (self.y2 as u32) << 1 | self.x1 as u32
    }
}

/// An access to a register this model does not know, kept for the boot report.
#[derive(Clone, Copy)]
pub struct Unknown {
    pub first: u64,
    pub count: u64,
    pub last: u32,
}

pub struct Hw {
    now: u64,
    pub ios: Ios,
    // Processor interface.
    pi_mask: u32,
    pi_regs: [u32; 0x40],
    /// Gather pipe: bytes until a 32-byte burst is complete.
    gather: Vec<u8>,
    /// Bytes written to the gather pipe, and those in bursts already given
    /// to the GP: stream positions for `drops`.
    gather_in: u64,
    gather_out: u64,
    /// Switches of the GX decoder's draw drop ([`Hw::drop_draws`]), at the
    /// stream positions they were made.
    drops: std::collections::VecDeque<(u64, bool)>,
    /// Gather-pipe bursts and EFB pokes, in program order.
    bursts: Vec<Burst>,
    /// GX command decoder fed from the CPU FIFO.
    /// GX decoding, inline or on its own thread (`gx::thread`).
    pub gpu: gx::thread::GpuPort,
    // Hollywood registers (IPC and IRQ at 0x0D000000).
    ipc: Ipc,
    hw_irq_flag: u32,
    hw_irq_mask: u32,
    hollywood: HashMap<u32, u32>,
    // Video interface.
    vi: [u16; 0x80],
    vi_line: u64,
    pub retraces: u64,
    // CP/PE.
    cp: [u16; 0x40],
    pe: [u16; 0x80],
    pe_token: u16,
    pub pe_finishes: u64,
    // MI, DI, and anything else stored as written.
    other: HashMap<u32, u32>,
    pub dsp: Dsp,
    exi: exi::Exi,
    si: exi::Si,
    /// (address, size, write) -> statistics, in first-seen order.
    pub unknown: Vec<((u32, u32, bool), Unknown)>,
    /// The EFB as the renderer last read it back (640x528 RGBA8, 24-bit
    /// depth), answering CPU peeks.
    pub efb: Option<(Vec<u8>, Vec<u32>)>,
    /// CPU reads of the EFB so far; the frontend keeps `efb` fresh while
    /// this grows.
    pub efb_peeks: u64,
}

/// Work for the GP, in the order the CPU issued it.
enum Burst {
    /// 32 bytes written to the CPU FIFO at a RAM address; `linked`: the GP
    /// reads them too.
    Fifo { addr: u32, data: [u8; 32], linked: bool },
    /// A CPU write to one EFB pixel (colour, or depth when `z`).
    Poke { x: u32, y: u32, value: u32, z: bool },
}

/// Physical address with the 0x0D8 mirror and legacy 0x0C006 window folded in.
fn normalize(addr: u32) -> u32 {
    let a = addr & 0x0FFF_FFFF;
    match a >> 20 {
        0x0D8 => a - 0x0080_0000,
        0x0C0 if a >> 12 == 0x0C006 => a + 0x0100_0000,
        _ => a,
    }
}

impl Hw {
    pub fn new(ios: Ios) -> Self {
        Self {
            now: 0,
            ios,
            pi_mask: 0,
            pi_regs: [0; 0x40],
            gather: Vec::new(),
            gather_in: 0,
            gather_out: 0,
            drops: Default::default(),
            bursts: Vec::new(),
            gpu: gx::thread::GpuPort::default(),
            ipc: Ipc::default(),
            hw_irq_flag: 0,
            hw_irq_mask: 0,
            hollywood: HashMap::new(),
            vi: [0; 0x80],
            vi_line: 0,
            retraces: 0,
            cp: [0; 0x40],
            pe: [0; 0x80],
            pe_token: 0,
            pe_finishes: 0,
            other: HashMap::new(),
            dsp: Dsp::default(),
            exi: exi::Exi::default(),
            si: exi::Si::default(),
            unknown: Vec::new(),
            efb: None,
            efb_peeks: 0,
        }
    }

    fn note_unknown(&mut self, addr: u32, size: u32, write: bool, value: u32) {
        let key = (addr, size, write);
        match self.unknown.iter_mut().find(|e| e.0 == key) {
            Some(e) => {
                e.1.count += 1;
                e.1.last = value;
            }
            None => self.unknown.push((key, Unknown { first: self.now, count: 1, last: value })),
        }
    }

    /// Interrupt causes currently asserted (before the mask).
    pub fn cause(&self) -> u32 {
        let mut c = 0;
        if self.si.interrupt() {
            c |= INT_SI;
        }
        if self.exi.interrupt() {
            c |= INT_EXI;
        }
        if self.dsp.interrupt() {
            c |= INT_DSP;
        }
        if self.vi_interrupt() {
            c |= INT_VI;
        }
        let pe = self.pe[0x0A / 2];
        if pe & 0x5 == 0x5 {
            c |= INT_PE_TOKEN;
        }
        if pe & 0xA == 0xA {
            c |= INT_PE_FINISH;
        }
        if self.hw_irq_flag & self.hw_irq_mask != 0 {
            c |= INT_IPC;
        }
        c
    }

    /// An external interrupt is waiting for MSR[EE].
    pub fn interrupt_pending(&self) -> bool {
        self.cause() & self.pi_mask != 0
    }

    fn vi_interrupt(&self) -> bool {
        (0..4).any(|i| {
            let hi = self.vi[0x30 / 2 + 2 * i];
            hi & 0x8000 != 0 && hi & 0x1000 != 0
        })
    }

    fn update_ipc_irq(&mut self) {
        let i = &self.ipc;
        if (i.y1 && i.iy1) || (i.y2 && i.iy2) {
            self.hw_irq_flag |= HW_IRQ_IPC;
        }
    }

    // ---- 16-bit register files: CP, PE, VI, MI, DSP ----

    fn read16(&mut self, a: u32) -> Option<u16> {
        let off = a & 0xFFF;
        Some(match a >> 12 {
            0x0C000 => match off {
                // Status: GP idle for reads and commands, FIFO below the low watermark.
                0x00 => 0x0E,
                // Read/write distance 0; read pointer = write pointer = PI's.
                0x30 | 0x32 => 0,
                0x34 | 0x38 => (self.pi_regs[0x14 / 4] & 0xFFFF) as u16,
                0x36 | 0x3A => (self.pi_regs[0x14 / 4] >> 16) as u16,
                _ => self.cp[(off / 2) as usize & 0x3F],
            },
            0x0C001 => match off {
                0x0E => self.pe_token,
                _ => self.pe[(off / 2) as usize & 0x7F],
            },
            0x0C002 => match off {
                0x2C => (self.vi_line % LINES_PER_FRAME) as u16 + 1,
                0x2E => ((self.now % CYCLES_PER_LINE) * 858 / CYCLES_PER_LINE) as u16 + 1,
                _ => self.vi[(off / 2) as usize & 0x7F],
            },
            0x0C004 => *self.other.get(&a).unwrap_or(&0) as u16,
            0x0C005 => self.dsp.read16(off & 0x7F),
            _ => return None,
        })
    }

    fn write16(&mut self, a: u32, v: u16) -> bool {
        let off = a & 0xFFF;
        match a >> 12 {
            0x0C000 => self.cp[(off / 2) as usize & 0x3F] = v,
            0x0C001 => {
                if off == 0x0A {
                    // Status bits 2 (token) and 3 (finish) clear on 1.
                    let st = self.pe[5] & 0xC & !(v & 0xC);
                    self.pe[5] = (v & 0x3) | st;
                } else {
                    self.pe[(off / 2) as usize & 0x7F] = v;
                }
            }
            0x0C002 => self.vi[(off / 2) as usize & 0x7F] = v,
            0x0C004 => {
                self.other.insert(a, u32::from(v));
            }
            0x0C005 => self.dsp.write16(off & 0x7F, v),
            _ => return false,
        }
        true
    }

    // ---- 32-bit register files: PI, Hollywood, DI, SI, EXI, AI ----

    fn read32(&mut self, a: u32) -> Option<u32> {
        let off = a & 0xFFF;
        Some(match a >> 12 {
            0x0C003 => match off {
                0x00 => self.cause() | INT_RESET_BUTTON,
                0x04 => self.pi_mask,
                0x2C => 0x2465_00B1, // Flipper revision
                _ => self.pi_regs[(off / 4) as usize & 0x3F],
            },
            0x0D000 => match off {
                0x00 => self.ipc.ppcmsg,
                0x04 => self.ipc.ctrl(),
                0x08 => self.ipc.armmsg,
                0x30 => self.hw_irq_flag,
                0x34 => self.hw_irq_mask,
                _ => {
                    if !self.hollywood.contains_key(&a) {
                        self.note_unknown(a, 4, false, 0);
                    }
                    *self.hollywood.get(&a).unwrap_or(&0)
                }
            },
            0x0D006 => match off {
                0x400..=0x7FF => self.si.read(off - 0x400),
                0x800..=0xBFF => self.exi.read(off - 0x800),
                0xC00..=0xFFF => self.dsp.ai_read(off - 0xC00, self.now),
                _ => *self.other.get(&a).unwrap_or(&0),
            },
            _ => return None,
        })
    }

    fn write32(&mut self, a: u32, v: u32) -> bool {
        let off = a & 0xFFF;
        match a >> 12 {
            0x0C003 => match off {
                // Causes are levels from the devices; nothing to acknowledge here.
                0x00 => {}
                0x04 => self.pi_mask = v,
                _ => self.pi_regs[(off / 4) as usize & 0x3F] = v,
            },
            0x0D000 => match off {
                0x00 => self.ipc.ppcmsg = v,
                0x04 => {
                    let i = &mut self.ipc;
                    if v & 1 != 0 {
                        i.x1 = true;
                        i.requests.push_back(i.ppcmsg);
                    }
                    if v & 2 != 0 {
                        i.y2 = false;
                    }
                    if v & 4 != 0 {
                        i.y1 = false;
                    }
                    i.x2 = v & 8 != 0;
                    i.iy1 = v & 0x10 != 0;
                    i.iy2 = v & 0x20 != 0;
                    self.update_ipc_irq();
                }
                0x30 => self.hw_irq_flag &= !v,
                0x34 => self.hw_irq_mask = v,
                _ => {
                    self.note_unknown(a, 4, true, v);
                    self.hollywood.insert(a, v);
                }
            },
            0x0D006 => match off {
                0x400..=0x7FF => self.si.write(off - 0x400, v),
                0x800..=0xBFF => self.exi.write(off - 0x800, v),
                0xC00..=0xFFF => self.dsp.ai_write(off - 0xC00, v, self.now),
                _ => {
                    self.other.insert(a, v);
                }
            },
            _ => return false,
        }
        true
    }

    /// Runs queued work up to cycle `now`: gather-pipe data, DMA, IOS
    /// requests and replies, video lines, audio DMA timing.
    pub fn advance(&mut self, now: u64, mem: &mut Mem) {
        self.now = now;
        self.flush_gather(mem);
        self.dsp.advance(now, mem);
        self.exi.now = now;
        self.exi.advance(mem);
        self.run_ipc(mem);
        let line = now / CYCLES_PER_LINE;
        while self.vi_line < line {
            self.vi_line += 1;
            let vct = (self.vi_line % LINES_PER_FRAME) as u16 + 1;
            for i in 0..4 {
                let idx = 0x30 / 2 + 2 * i;
                let hi = self.vi[idx];
                if hi & 0x1000 != 0 && hi & 0x7FF == vct {
                    self.vi[idx] = hi | 0x8000;
                    if i == 0 || i == 1 {
                        self.retraces += 1;
                    }
                }
            }
        }
    }

    fn run_ipc(&mut self, mem: &mut Mem) {
        let done = self.ios.poll(mem, self.now);
        self.ipc.replies.extend(done);
        if self.ipc.y1 || self.ipc.y2 {
            return;
        }
        if let Some(req) = self.ipc.requests.pop_front() {
            // Acknowledge, then run it; the reply follows once the ack is cleared.
            self.ipc.x1 = false;
            self.ipc.armmsg = req;
            self.ipc.y2 = true;
            if self.ios.execute(mem, req) {
                self.ipc.replies.push_back(req);
            }
        } else if let Some(rep) = self.ipc.replies.pop_front() {
            self.ipc.armmsg = rep;
            self.ipc.y1 = true;
        }
        self.update_ipc_irq();
    }

    /// The CPU FIFO in RAM (PI's base and end), as cached virtual addresses.
    /// Gather-pipe bursts reach it only when `advance` runs, so a check made
    /// right after a call must not compare it.
    pub fn cpu_fifo(&self) -> (u32, u32) {
        let (base, end) = (self.pi_regs[0x0C / 4] & 0x1FFF_FFE0, self.pi_regs[0x10 / 4] & 0x1FFF_FFE0);
        (base | 0x8000_0000, end.max(base) | 0x8000_0000)
    }

    /// A gather-pipe write: every 32 bytes form a burst to the CPU FIFO in
    /// RAM at PI's write pointer, which advances (and wraps) right away so the
    /// guest sees consistent pointers; the copy happens in `advance`.
    fn gather_write(&mut self, bytes: &[u8]) {
        self.gather_in += bytes.len() as u64;
        self.gather.extend_from_slice(bytes);
        while self.gather.len() >= 32 {
            let burst: [u8; 32] = self.gather[..32].try_into().unwrap();
            self.gather.drain(..32);
            let (base, end) = (self.pi_regs[0x0C / 4] & 0x1FFF_FFE0, self.pi_regs[0x10 / 4] & 0x1FFF_FFE0);
            let wp = self.pi_regs[0x14 / 4] & 0x1FFF_FFE0;
            // CP control bit 4: the GP reads what the CPU writes (not when
            // the CPU is recording a display list).
            let linked = self.cp[1] & 0x10 != 0;
            self.bursts.push(Burst::Fifo { addr: wp, data: burst, linked });
            let next = wp + 32;
            self.pi_regs[0x14 / 4] = if next >= end && base < end { base } else { next };
        }
    }

    fn flush_gather(&mut self, mem: &mut Mem) {
        if self.bursts.is_empty() {
            return;
        }
        // Taken out while the GPU reads them, then put back emptied: a new
        // vector every slice cost an allocation and its regrowth.
        let mut bursts = std::mem::take(&mut self.bursts);
        let mut events = Vec::new();
        for b in &bursts {
            match *b {
                Burst::Fifo { addr, ref data, linked } => {
                    if let Some(d) = mem.ram_mut(addr, 32) {
                        d.copy_from_slice(data);
                    }
                    let start = self.gather_out;
                    self.gather_out += 32;
                    // Fed in pieces at the draw-drop switches inside it.
                    let mut from = 0;
                    while let Some(&(at, on)) = self.drops.front().filter(|d| d.0 < start + 32) {
                        let cut = at.saturating_sub(start) as usize;
                        if linked && cut > from {
                            self.gpu.feed(&data[from..cut], mem, &mut events);
                        }
                        from = from.max(cut);
                        self.gpu.set_drop(on);
                        self.drops.pop_front();
                    }
                    if linked && from < 32 {
                        self.gpu.feed(&data[from..], mem, &mut events);
                    }
                }
                Burst::Poke { x, y, value, z } => self.gpu.poke(x, y, value, z),
            }
        }
        bursts.clear();
        self.bursts = bursts;
        for e in events {
            match e {
                gx::Event::Finish => {
                    self.pe[5] |= 0x8;
                    self.pe_finishes += 1;
                }
                gx::Event::TokenInt(t) => {
                    self.pe_token = t;
                    self.pe[5] |= 0x4;
                }
                gx::Event::Token(t) => self.pe_token = t,
            }
        }
    }

    /// From the bytes written to the gather pipe so far on, the GX decoder
    /// drops draws (`on`) or draws them again: a switch in the command
    /// stream, at the point the CPU has reached (the GX thread decodes
    /// later). Guest memory never sees it.
    pub fn drop_draws(&mut self, on: bool) {
        self.drops.push_back((self.gather_in, on));
    }

    /// A CPU read of the EFB window (physical 0x08000000): x in bits 2..11,
    /// y in bits 12..21, depth when bit 22 is set. Colour reads as
    /// 0xAARRGGBB. Answered from the last snapshot.
    fn efb_peek(&mut self, a: u32) -> u32 {
        self.efb_peeks += 1;
        let (x, y) = (((a >> 2) & 0x3FF) as usize, ((a >> 12) & 0x3FF) as usize);
        let Some((color, depth)) = &self.efb else { return 0 };
        let i = y.min(527) * 640 + x.min(639);
        if a & (1 << 22) != 0 {
            depth[i]
        } else {
            let p = &color[4 * i..4 * i + 4];
            u32::from_be_bytes([p[3], p[0], p[1], p[2]])
        }
    }

    /// Physical address of the XFB the video interface scans out (top
    /// field, VI register 0x1C).
    /// The video interface shows a picture: VISetBlack sets the active lines
    /// (VTR's ACV) to zero while the screen is black, as before the game
    /// first sets up the video.
    pub fn display_on(&self) -> bool {
        (self.vi[0] >> 4) & 0x3FF != 0
    }

    pub fn xfb_addr(&self) -> u32 {
        let v = u32::from(self.vi[0x1C / 2]) << 16 | u32::from(self.vi[0x1E / 2]);
        if v & (1 << 28) != 0 {
            (v & 0xFF_FFFF) << 5
        } else {
            v & 0xFF_FFFF
        }
    }
}

/// `Mmio` for [`Mem`]: shares the [`Hw`] with the run loop.
pub struct HwPort(pub std::rc::Rc<std::cell::RefCell<Hw>>);

impl guest_mem::Mmio for HwPort {
    fn read(&mut self, addr: u32, size: u32) -> u32 {
        let mut hw = self.0.borrow_mut();
        let a = normalize(addr);
        if a >> 24 == 0x08 {
            return hw.efb_peek(a);
        }
        let is16 = matches!(a >> 12, 0x0C000 | 0x0C001 | 0x0C002 | 0x0C004 | 0x0C005);
        let v = if is16 {
            match size {
                4 => hw.read16(a).map(|h| (u32::from(h) << 16) | u32::from(hw.read16(a + 2).unwrap_or(0))),
                2 => hw.read16(a).map(u32::from),
                _ => hw.read16(a & !1).map(|h| u32::from(if a & 1 == 0 { h >> 8 } else { h & 0xFF })),
            }
        } else {
            hw.read32(a & !3).map(|w| match size {
                4 => w,
                2 => (w >> (16 - 8 * (a & 2))) & 0xFFFF,
                _ => (w >> (24 - 8 * (a & 3))) & 0xFF,
            })
        };
        match v {
            Some(v) => v,
            None => {
                hw.note_unknown(a, size, false, 0);
                0
            }
        }
    }

    fn write(&mut self, addr: u32, size: u32, value: u32) {
        let mut hw = self.0.borrow_mut();
        let a = normalize(addr);
        if a >> 12 == 0x0C008 {
            // Gather pipe: bytes in program order.
            hw.gather_write(&value.to_be_bytes()[4 - size as usize..]);
            return;
        }
        if a >> 24 == 0x08 {
            let (x, y) = ((a >> 2) & 0x3FF, (a >> 12) & 0x3FF);
            hw.bursts.push(Burst::Poke { x, y, value, z: a & (1 << 22) != 0 });
            return;
        }
        let is16 = matches!(a >> 12, 0x0C000 | 0x0C001 | 0x0C002 | 0x0C004 | 0x0C005);
        let ok = if is16 {
            match size {
                4 => hw.write16(a, (value >> 16) as u16) && hw.write16(a + 2, value as u16),
                2 => hw.write16(a, value as u16),
                _ => {
                    let old = hw.read16(a & !1).unwrap_or(0);
                    let v = if a & 1 == 0 { (old & 0xFF) | (value as u16) << 8 } else { (old & 0xFF00) | (value as u16 & 0xFF) };
                    hw.write16(a & !1, v)
                }
            }
        } else if size == 4 {
            hw.write32(a, value)
        } else {
            let old = hw.read32(a & !3).unwrap_or(0);
            let shift = if size == 2 { 16 - 8 * (a & 2) } else { 24 - 8 * (a & 3) };
            let mask = if size == 2 { 0xFFFF } else { 0xFF } << shift;
            hw.write32(a & !3, (old & !mask) | ((value << shift) & mask))
        };
        if !ok {
            hw.note_unknown(a, size, true, value);
        }
    }
}

use guest_mem::snap::{Snap, Snapshot};

impl Snapshot for Ipc {
    fn snap(&mut self, s: &mut Snap) {
        guest_mem::snap!(s; self.ppcmsg, self.armmsg, self.x1, self.x2, self.y1, self.y2, self.iy1, self.iy2, self.requests, self.replies);
    }
}

impl Default for Burst {
    fn default() -> Self {
        Burst::Poke { x: 0, y: 0, value: 0, z: false }
    }
}

impl Snapshot for Burst {
    fn snap(&mut self, s: &mut Snap) {
        let mut fifo = matches!(self, Burst::Fifo { .. });
        fifo.snap(s);
        if s.loading() {
            *self = if fifo { Burst::Fifo { addr: 0, data: [0; 32], linked: false } } else { Burst::default() };
        }
        match self {
            Burst::Fifo { addr, data, linked } => guest_mem::snap!(s; *addr, *data, *linked),
            Burst::Poke { x, y, value, z } => guest_mem::snap!(s; *x, *y, *value, *z),
        }
    }
}

/// Everything but host-side state: the register-access statistics
/// (`unknown`) and the renderer's EFB for CPU peeks (`efb`, `efb_peeks`).
/// GX is saved by its port: the scanner, and the decoder's registers.
impl Snapshot for Hw {
    fn snap(&mut self, s: &mut Snap) {
        s.tag(b"HW  ");
        guest_mem::snap!(s; self.now, self.ios, self.pi_mask, self.pi_regs, self.gather, self.bursts, self.gpu, self.ipc, self.hw_irq_flag, self.hw_irq_mask, self.hollywood, self.vi, self.vi_line, self.retraces, self.cp, self.pe, self.pe_token, self.pe_finishes, self.other, self.dsp, self.exi, self.si);
        if s.loading() {
            // Stream positions start again from the loaded bursts, with
            // nothing dropped.
            let queued = self.bursts.iter().filter(|b| matches!(b, Burst::Fifo { .. })).count() as u64;
            (self.gather_out, self.gather_in) = (0, 32 * queued + self.gather.len() as u64);
            self.drops.clear();
            self.gpu.set_drop(false);
        }
    }
}
