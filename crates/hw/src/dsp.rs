//! DSP interface (`0xCC005000`, 16-bit registers) and the audio interface
//! (`0xCD006C00`). Mailboxes answer the boot handshakes, ARAM DMA copies (on
//! the Wii, "ARAM" addresses land in MEM2, as in Dolphin), command lists run
//! on the host AX mixer (`ax`), and the AI DMA plays the frames it writes:
//! each finished block's samples go to [`Dsp::audio`] for the host, at the
//! real cadence, so the game's audio thread runs as on hardware.

use std::collections::VecDeque;

use guest_mem::Mem;

use crate::ax::Ax;

const CSR_RES: u16 = 0x0001;
const CSR_HALT: u16 = 0x0004;
const CSR_AIDINT: u16 = 0x0008;
const CSR_AIDINTMSK: u16 = 0x0010;
const CSR_ARINT: u16 = 0x0020;
const CSR_ARINTMSK: u16 = 0x0040;
const CSR_DSPINT: u16 = 0x0080;
const CSR_DSPINTMSK: u16 = 0x0100;
/// Interrupt status bits, cleared by writing 1.
const CSR_INTS: u16 = CSR_AIDINT | CSR_ARINT | CSR_DSPINT;
/// Mail the DSP boot ROM sends after it is released from halt.
pub const MAIL_ROM_READY: u32 = 0x8071_FEED;
/// Task boot: the CPU sends five (command, value) pairs, then the task's
/// microcode sends this once running.
pub const MAIL_TASK_INIT: u32 = 0xDCD1_0000;
const MAIL_RESUMED: u32 = 0xDCD1_0001;
/// The AX microcode yields after each command list.
const MAIL_YIELD: u32 = 0xDCD1_0002;
// CPU -> DSP requests understood by the task framework.
const REQ_RESUME: u32 = 0xCDD1_0000;
const REQ_NEW_UCODE: u32 = 0xCDD1_0001;
/// `0xBABE<size>` then the command list address.
const REQ_CMDLIST: u32 = 0xBABE_0000;
/// Cycles between a command list and the yield mail (Dolphin: games need >= 814).
const AX_LATENCY: u64 = 2000;

pub struct Dsp {
    regs: [u16; 0x40],
    csr: u16,
    /// (mail, raises DSPINT, visible from cycle).
    to_cpu: VecDeque<(u32, bool, u64)>,
    now: u64,
    /// Mail the CPU sent, most recent last (for diagnostics and the boot state).
    pub from_cpu: Vec<u32>,
    /// Mails left in a microcode upload (five command/value pairs).
    upload_left: usize,
    cmdlist_next: bool,
    pub command_lists: u64,
    /// A command list to run at the next `advance`.
    frame_pending: Option<u32>,
    /// The AX microcode, on the host.
    pub ax: Ax,
    dma_pending: bool,
    /// AI DMA: next time a block "finishes", in cycles; 0 when stopped.
    ai_next: u64,
    /// The block playing: its address and length in bytes (latched when it
    /// starts, as the hardware does).
    ai_block: (u32, u32),
    pub ai_blocks: u64,
    /// Samples the AI DMA played, interleaved left and right, at
    /// [`Dsp::audio_rate`]; the frontend drains it. Bounded to one second.
    pub audio: Vec<i16>,
    pub audio_rate: u32,
    // Audio interface (0xCD006C00).
    aicr: u32,
    ai_regs: [u32; 4],
    scnt_base: u32,
    scnt_since: u64,
}

impl Default for Dsp {
    fn default() -> Self {
        Self {
            regs: [0; 0x40],
            csr: CSR_HALT,
            to_cpu: VecDeque::new(),
            now: 0,
            from_cpu: Vec::new(),
            upload_left: 0,
            cmdlist_next: false,
            command_lists: 0,
            frame_pending: None,
            ax: Ax::default(),
            dma_pending: false,
            ai_next: 0,
            ai_block: (0, 0),
            ai_blocks: 0,
            audio: Vec::new(),
            audio_rate: 32_000,
            aicr: 0,
            ai_regs: [0; 4],
            scnt_base: 0,
            scnt_since: 0,
        }
    }
}

/// Bus cycles (the CPU counter) per second.
const CPU_HZ: u64 = 729_000_000;

impl Dsp {
    pub fn read16(&mut self, off: u32) -> u16 {
        match off {
            0x00 => self.regs[0] & 0x7FFF,
            0x04 => self.visible_mail().map_or(0, |m| 0x8000 | (m >> 16) as u16),
            0x06 => match self.visible_mail() {
                Some(m) => {
                    self.to_cpu.pop_front();
                    m as u16
                }
                None => 0,
            },
            0x0A => self.csr,
            // AI DMA blocks left.
            0x3A => 0,
            _ => self.regs[(off / 2) as usize & 0x3F],
        }
    }

    pub fn write16(&mut self, off: u32, v: u16) {
        self.regs[(off / 2) as usize & 0x3F] = v;
        match off {
            0x02 => {
                let mail = (u32::from(self.regs[0]) << 16) | u32::from(v);
                self.mail_from_cpu(mail);
            }
            0x0A => {
                let was_halted = self.csr & CSR_HALT != 0;
                let ints = (self.csr & CSR_INTS) & !(v & CSR_INTS);
                self.csr = (v & !(CSR_INTS | CSR_RES | 0x0600)) | ints;
                if v & CSR_RES != 0 {
                    self.to_cpu.clear();
                    self.upload_left = 0;
                    self.cmdlist_next = false;
                }
                if was_halted && self.csr & CSR_HALT == 0 {
                    self.send(MAIL_ROM_READY, false, 0);
                }
            }
            // ARAM DMA count, low half: start the transfer.
            0x2A => {
                self.dma_pending = true;
                self.csr |= 0x0200;
            }
            // AI DMA control: bit 15 starts it.
            0x36 => {
                if v & 0x8000 == 0 {
                    self.ai_next = 0;
                }
            }
            _ => {}
        }
    }

    fn mail_from_cpu(&mut self, mail: u32) {
        self.from_cpu.push(mail);
        if self.from_cpu.len() > 256 {
            self.from_cpu.drain(..128);
        }
        if self.cmdlist_next {
            // The command list address: run it, and answer as AX does when done.
            self.cmdlist_next = false;
            self.command_lists += 1;
            self.frame_pending = Some(mail);
            self.send(MAIL_YIELD, true, AX_LATENCY);
        } else if self.upload_left > 0 || mail >> 16 == 0x80F3 {
            // Microcode boot: 0x80F3xxxx command words, each followed by a value.
            if self.upload_left == 0 {
                self.upload_left = 10;
            }
            self.upload_left -= 1;
            if self.upload_left == 0 {
                self.send(MAIL_TASK_INIT, true, 0);
            }
        } else if mail == REQ_RESUME {
            self.send(MAIL_RESUMED, true, 0);
        } else if mail == REQ_NEW_UCODE {
            self.upload_left = 10;
        } else if mail & 0xFFFF_0000 == REQ_CMDLIST {
            self.cmdlist_next = true;
        }
    }

    fn send(&mut self, mail: u32, int: bool, delay: u64) {
        self.to_cpu.push_back((mail, int, self.now + delay));
    }

    fn visible_mail(&self) -> Option<u32> {
        self.to_cpu.front().filter(|m| m.2 <= self.now).map(|m| m.0)
    }

    pub fn interrupt(&self) -> bool {
        let c = self.csr;
        (c & CSR_AIDINT != 0 && c & CSR_AIDINTMSK != 0)
            || (c & CSR_ARINT != 0 && c & CSR_ARINTMSK != 0)
            || (c & CSR_DSPINT != 0 && c & CSR_DSPINTMSK != 0)
    }

    fn reg32(&self, off: u32) -> u32 {
        (u32::from(self.regs[(off / 2) as usize]) << 16) | u32::from(self.regs[(off / 2 + 1) as usize])
    }

    pub fn advance(&mut self, now: u64, mem: &mut Mem) {
        if self.dma_pending {
            self.dma_pending = false;
            let mram = self.reg32(0x20) & 0x1FFF_FFE0;
            let aram = (self.reg32(0x24) & 0x03FF_FFE0) | 0x1000_0000;
            let cnt = self.reg32(0x28);
            let len = (cnt & 0x03FF_FFE0) as usize;
            let (src, dst) = if cnt & 0x8000_0000 != 0 { (aram, mram) } else { (mram, aram) };
            if let Some(data) = mem.ram(src, len).map(<[u8]>::to_vec) {
                if let Some(d) = mem.ram_mut(dst, len) {
                    d.copy_from_slice(&data);
                }
            }
            self.csr = (self.csr & !0x0200) | CSR_ARINT;
        }
        self.now = now;
        if let Some(list) = self.frame_pending.take() {
            self.ax.frame(list, mem);
        }
        // A pending mail with an interrupt raises DSPINT when it reaches the front.
        if let Some(m) = self.to_cpu.front_mut().filter(|m| m.2 <= now) {
            if m.1 {
                m.1 = false;
                self.csr |= CSR_DSPINT;
            }
        }
        // AI DMA: one interrupt per block of (length * 32) bytes of 16-bit
        // stereo. A block plays from the address latched when it started;
        // the interrupt then lets the game point the next one elsewhere.
        let ctrl = self.regs[0x36 / 2];
        if ctrl & 0x8000 != 0 {
            let bytes = u32::from(ctrl & 0x7FFF) * 32;
            let rate = if self.aicr & 0x40 != 0 { 32_000 } else { 48_000 };
            let period = u64::from(bytes / 4).max(1) * CPU_HZ / u64::from(rate);
            if self.ai_next == 0 || now >= self.ai_next {
                if self.ai_next != 0 {
                    self.play_block(mem, rate);
                    self.ai_next += period;
                } else {
                    self.ai_next = now + period;
                }
                self.ai_block = ((self.reg32(0x30) & 0x1FFF_FFE0) | 0x8000_0000, bytes);
                self.csr |= CSR_AIDINT;
                self.ai_blocks += 1;
            }
        }
    }

    /// The finished block's samples (right then left, big-endian) to
    /// [`Dsp::audio`] as left, right.
    fn play_block(&mut self, mem: &Mem, rate: u32) {
        let (addr, bytes) = self.ai_block;
        let Some(b) = mem.ram(addr, bytes as usize) else { return };
        if self.audio_rate != rate {
            self.audio.clear();
            self.audio_rate = rate;
        }
        if self.audio.len() > 2 * rate as usize {
            self.audio.drain(..bytes as usize / 2);
        }
        for f in b.chunks_exact(4) {
            self.audio.push(i16::from_be_bytes([f[2], f[3]]));
            self.audio.push(i16::from_be_bytes([f[0], f[1]]));
        }
    }

    // Audio interface registers (offsets from 0xCD006C00).

    pub fn ai_read(&mut self, off: u32, now: u64) -> u32 {
        match off {
            0x00 => self.aicr,
            0x08 => self.sample_count(now),
            _ => self.ai_regs[(off / 4) as usize & 3],
        }
    }

    pub fn ai_write(&mut self, off: u32, v: u32, now: u64) {
        match off {
            0x00 => {
                let playing = self.aicr & 1 != 0;
                if playing && v & 1 == 0 {
                    self.scnt_base = self.sample_count(now);
                }
                if v & 0x20 != 0 {
                    self.scnt_base = 0;
                }
                if (!playing && v & 1 != 0) || v & 0x20 != 0 {
                    self.scnt_since = now;
                }
                // AIINT (bit 3) is cleared by writing 1; SCRESET (bit 5) self-clears.
                let int = self.aicr & 0x8 & !(v & 0x8);
                self.aicr = (v & !0x28) | int;
            }
            _ => self.ai_regs[(off / 4) as usize & 3] = v,
        }
    }

    /// Streaming sample counter: advances at 48 kHz while playing.
    fn sample_count(&self, now: u64) -> u32 {
        if self.aicr & 1 == 0 {
            return self.scnt_base;
        }
        self.scnt_base.wrapping_add((now.saturating_sub(self.scnt_since) * 48_000 / CPU_HZ) as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mail(d: &mut Dsp) -> Option<u32> {
        let hi = d.read16(0x04);
        (hi & 0x8000 != 0).then(|| (u32::from(hi & 0x7FFF) << 16 | u32::from(d.read16(0x06))) | 0x8000_0000)
    }

    fn send(d: &mut Dsp, m: u32) {
        d.write16(0x00, (m >> 16) as u16);
        d.write16(0x02, m as u16);
    }

    #[test]
    fn rom_handshake_task_boot_and_command_lists() {
        let mut d = Dsp::default();
        assert_eq!(mail(&mut d), None);
        // Release from halt: the ROM says hello.
        d.write16(0x0A, 0);
        assert_eq!(mail(&mut d), Some(MAIL_ROM_READY));
        // Five command/value pairs boot a task, which then reports in with DSPINT.
        for (c, v) in [(0x80F3_A001, 0x10), (0x80F3_C002, 0), (0x80F3_A002, 0x2000), (0x80F3_B002, 0), (0x80F3_D001, 0x10)] {
            send(&mut d, c);
            send(&mut d, v);
        }
        d.csr |= CSR_DSPINTMSK;
        let mut mem = Mem::new(Box::new(guest_mem::NullMmio));
        d.advance(0, &mut mem);
        assert!(d.interrupt());
        assert_eq!(mail(&mut d), Some(MAIL_TASK_INIT));
        // A command list is answered with a yield, but not before the latency.
        send(&mut d, REQ_CMDLIST | 0x80);
        send(&mut d, 0x8037_BEA0);
        assert_eq!(mail(&mut d), None);
        d.advance(AX_LATENCY, &mut mem);
        assert_eq!(mail(&mut d), Some(MAIL_YIELD));
        assert_eq!(d.command_lists, 1);
    }

    #[test]
    fn aram_dma_copies_and_raises_arint() {
        let mut d = Dsp::default();
        let mut mem = Mem::new(Box::new(guest_mem::NullMmio));
        mem.write_u32(0x8100_0000, 0xDEAD_BEEF).unwrap();
        // MRAM 0x01000000 -> ARAM 0x40 (MEM2 on the Wii), 32 bytes.
        for (off, v) in [(0x20, 0x0100), (0x22, 0), (0x24, 0), (0x26, 0x40), (0x28, 0), (0x2A, 0x20)] {
            d.write16(off, v);
        }
        assert_eq!(d.read16(0x0A) & CSR_ARINT, 0);
        d.advance(1, &mut mem);
        assert_eq!(mem.read_u32(0x9000_0040), Ok(0xDEAD_BEEF));
        assert_ne!(d.read16(0x0A) & CSR_ARINT, 0);
        // Writing 1 clears it.
        d.write16(0x0A, CSR_ARINT);
        assert_eq!(d.read16(0x0A) & CSR_ARINT, 0);
    }
}

/// The statistics (`from_cpu`, `command_lists`, `ai_blocks`, the mixer's
/// counters) and [`Dsp::audio`] (output already produced) are left out.
impl guest_mem::snap::Snapshot for Dsp {
    fn snap(&mut self, s: &mut guest_mem::snap::Snap) {
        s.tag(b"DSP ");
        guest_mem::snap!(s; self.regs, self.csr, self.to_cpu, self.now, self.upload_left, self.cmdlist_next, self.frame_pending, self.dma_pending, self.ai_next, self.ai_block, self.ax.last_main_volume, self.ax.last_aux_volume, self.aicr, self.ai_regs, self.scnt_base, self.scnt_since);
    }
}
