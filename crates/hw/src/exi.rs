//! External interface (`0xCD006800`): three channels. Only channel 0 device 1
//! (the "IPL" chip: RTC and SRAM) is present; memory card slots and the rest
//! report no device. Also the serial interface (`0xCD006400`), where every
//! GameCube controller port answers "no response".

use std::time::{SystemTime, UNIX_EPOCH};

use guest_mem::Mem;

const CSR_EXIINT: u32 = 1 << 1;
const CSR_TCINT: u32 = 1 << 3;
const CSR_EXTINT: u32 = 1 << 11;
const CSR_INTS: u32 = CSR_EXIINT | CSR_TCINT | CSR_EXTINT;

#[derive(Default, Clone, Copy)]
struct Channel {
    csr: u32,
    mar: u32,
    len: u32,
    cr: u32,
    data: u32,
}

/// Which region of the IPL chip the current command addresses.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Region {
    None,
    Rtc,
    Sram,
    Other,
}

pub struct Exi {
    ch: [Channel; 3],
    /// A transfer was started and runs at the next `advance`.
    pending: [bool; 3],
    sram: [u8; 64],
    /// The RTC at power-on (host clock, seconds since 2000); it then runs on
    /// emulated time, so it is the same on every machine in a net session.
    rtc_at_boot: i64,
    rtc_bias: i64,
    /// Emulated time, in CPU cycles (set by `Hw::advance`).
    pub now: u64,
    // IPL command state for channel 0 device 1.
    selected: bool,
    region: Region,
    cursor: usize,
    write: bool,
    pub transfers: u64,
}

const CPU_HZ: u64 = 729_000_000;
/// Seconds from 1970 to 2000-01-01, the RTC epoch.
const RTC_EPOCH: u64 = 946_684_800;

fn default_sram() -> [u8; 64] {
    let mut s = [0u8; 64];
    s[0x12] = 0; // language: English
    s[0x13] = 0x2C; // flags (as Dolphin's default)
    let (mut sum, mut inv) = (0u16, 0u16);
    for i in 0..4 {
        let w = u16::from_be_bytes([s[0x0C + 2 * i], s[0x0D + 2 * i]]);
        sum = sum.wrapping_add(w);
        inv = inv.wrapping_add(!w);
    }
    s[0..2].copy_from_slice(&sum.to_be_bytes());
    s[2..4].copy_from_slice(&inv.to_be_bytes());
    s
}

impl Default for Exi {
    fn default() -> Self {
        Self {
            ch: [Channel::default(); 3],
            pending: [false; 3],
            sram: default_sram(),
            rtc_at_boot: SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs()).saturating_sub(RTC_EPOCH) as i64,
            rtc_bias: 0,
            now: 0,
            selected: false,
            region: Region::None,
            cursor: 0,
            write: false,
            transfers: 0,
        }
    }
}

impl Exi {
    pub fn read(&mut self, off: u32) -> u32 {
        let (n, r) = ((off / 0x14) as usize, off % 0x14);
        let Some(c) = self.ch.get(n) else { return 0 };
        match r {
            0x00 => c.csr,
            0x04 => c.mar,
            0x08 => c.len,
            0x0C => c.cr,
            _ => c.data,
        }
    }

    pub fn write(&mut self, off: u32, v: u32) {
        let (n, r) = ((off / 0x14) as usize, off % 0x14);
        if n >= 3 {
            return;
        }
        match r {
            0x00 => {
                let ints = self.ch[n].csr & CSR_INTS & !(v & CSR_INTS);
                // EXT (bit 12, device present) is read-only and always 0 here.
                self.ch[n].csr = (v & !(CSR_INTS | 0x1000)) | ints;
                if n == 0 {
                    let sel = (v >> 7) & 7 == 2; // chip select: device 1
                    if sel && !self.selected {
                        self.region = Region::None;
                    }
                    self.selected = sel;
                }
            }
            0x04 => self.ch[n].mar = v & 0x13FF_FFE0,
            0x08 => self.ch[n].len = v & 0x03FF_FFE0,
            0x0C => {
                self.ch[n].cr = v;
                if v & 1 != 0 {
                    self.pending[n] = true;
                }
            }
            _ => self.ch[n].data = v,
        }
    }

    pub fn interrupt(&self) -> bool {
        self.ch.iter().any(|c| (c.csr & CSR_EXIINT != 0 && c.csr & 1 != 0) || (c.csr & CSR_TCINT != 0 && c.csr & 4 != 0) || (c.csr & CSR_EXTINT != 0 && c.csr & 0x400 != 0))
    }

    /// Seconds since 2000 by the clock before the game's bias.
    fn clock(&self) -> i64 {
        self.rtc_at_boot + (self.now / CPU_HZ) as i64
    }

    fn rtc(&self) -> [u8; 4] {
        ((self.clock() + self.rtc_bias) as u32).to_be_bytes()
    }

    /// Bytes read from the IPL chip at the current cursor.
    fn ipl_read(&mut self, out: &mut [u8]) {
        for b in out.iter_mut() {
            *b = match self.region {
                Region::Rtc => self.rtc()[self.cursor & 3],
                Region::Sram => self.sram[self.cursor & 63],
                _ => 0,
            };
            self.cursor += 1;
        }
    }

    fn ipl_write(&mut self, data: &[u8]) {
        for &b in data {
            match self.region {
                Region::Sram => self.sram[self.cursor & 63] = b,
                Region::Rtc => {
                    let mut r = self.rtc();
                    r[self.cursor & 3] = b;
                    self.rtc_bias = i64::from(u32::from_be_bytes(r)) - self.clock();
                }
                _ => {}
            }
            self.cursor += 1;
        }
    }

    pub fn advance(&mut self, mem: &mut Mem) {
        for n in 0..3 {
            if !std::mem::take(&mut self.pending[n]) {
                continue;
            }
            self.transfers += 1;
            let c = self.ch[n];
            let ipl = n == 0 && self.selected;
            let rw = (c.cr >> 2) & 3;
            let dma = c.cr & 2 != 0;
            if !dma {
                let len = ((c.cr >> 4) & 3) as usize + 1;
                if ipl && rw == 1 && self.region == Region::None {
                    // First immediate write after select: the command word.
                    let cmd = c.data;
                    self.write = cmd & 0x8000_0000 != 0;
                    let addr = cmd & 0x7FFF_FFFF;
                    // Byte offsets are encoded << 6: RTC at 0x20000000, SRAM's
                    // 64 bytes from 0x20000100.
                    self.region = match addr {
                        0x2000_0000..=0x2000_00FF => Region::Rtc,
                        0x2000_0100..=0x2000_10FF => Region::Sram,
                        _ => Region::Other,
                    };
                    self.cursor = match self.region {
                        Region::Sram => ((addr - 0x2000_0100) >> 6) as usize,
                        _ => 0,
                    };
                } else if ipl && rw == 1 {
                    let bytes = c.data.to_be_bytes();
                    self.ipl_write(&bytes[..len]);
                } else if ipl {
                    let mut b = [0u8; 4];
                    self.ipl_read(&mut b[..len]);
                    self.ch[n].data = u32::from_be_bytes(b);
                } else if rw != 1 {
                    // No device: the bus floats high.
                    self.ch[n].data = 0xFFFF_FFFF;
                }
            } else if ipl {
                let len = c.len as usize;
                if rw == 0 {
                    let mut buf = vec![0u8; len];
                    self.ipl_read(&mut buf);
                    if let Some(d) = mem.ram_mut(c.mar, len) {
                        d.copy_from_slice(&buf);
                    }
                } else if let Some(src) = mem.ram(c.mar, len).map(<[u8]>::to_vec) {
                    self.ipl_write(&src);
                }
            }
            self.ch[n].cr &= !1;
            self.ch[n].csr |= CSR_TCINT;
        }
    }
}

/// Serial interface: no controllers on the GameCube ports.
pub struct Si {
    regs: [u32; 0x40],
    buf: [u8; 128],
    comcsr: u32,
    sr: u32,
}

impl Default for Si {
    fn default() -> Self {
        Self { regs: [0; 0x40], buf: [0; 128], comcsr: 0, sr: 0 }
    }
}

impl Si {
    pub fn read(&mut self, off: u32) -> u32 {
        match off {
            0x34 => self.comcsr,
            0x38 => self.sr,
            0x80..=0xFF => {
                let i = (off - 0x80) as usize & !3;
                u32::from_be_bytes(self.buf[i..i + 4].try_into().unwrap())
            }
            _ => self.regs[(off / 4) as usize & 0x3F],
        }
    }

    pub fn write(&mut self, off: u32, v: u32) {
        match off {
            0x34 => {
                // TCINT (31) and RDSTINT (28) clear on 1; TSTART runs the transfer.
                let ints = self.comcsr & 0x9000_0000 & !(v & 0x9000_0000);
                self.comcsr = (v & !0x9000_0001) | ints;
                if v & 1 != 0 {
                    let chan = (v >> 1) & 3;
                    self.comcsr |= 0x8000_0000 | 0x2000_0000; // TCINT, COMERR
                    self.sr |= 1 << (27 - 8 * chan); // NOREP for that channel
                }
            }
            // SISR: error bits clear on 1.
            0x38 => self.sr &= !(v & 0x0F0F_0F0F),
            0x80..=0xFF => {
                let i = (off - 0x80) as usize & !3;
                self.buf[i..i + 4].copy_from_slice(&v.to_be_bytes());
            }
            _ => self.regs[(off / 4) as usize & 0x3F] = v,
        }
    }

    pub fn interrupt(&self) -> bool {
        let c = self.comcsr;
        (c & 0x8000_0000 != 0 && c & 0x4000_0000 != 0) || (c & 0x1000_0000 != 0 && c & 0x0800_0000 != 0)
    }
}

use guest_mem::snap::{Snap, Snapshot};

impl Snapshot for Channel {
    fn snap(&mut self, s: &mut Snap) {
        guest_mem::snap!(s; self.csr, self.mar, self.len, self.cr, self.data);
    }
}

impl Snapshot for Region {
    fn snap(&mut self, s: &mut Snap) {
        let mut v = *self as u8;
        v.snap(s);
        *self = match v {
            1 => Region::Rtc,
            2 => Region::Sram,
            3 => Region::Other,
            _ => Region::None,
        };
    }
}

/// `transfers` (a statistic) is left out.
impl Snapshot for Exi {
    fn snap(&mut self, s: &mut Snap) {
        s.tag(b"EXI ");
        guest_mem::snap!(s; self.ch, self.pending, self.sram, self.rtc_at_boot, self.rtc_bias, self.now, self.selected, self.region, self.cursor, self.write);
    }
}

impl Snapshot for Si {
    fn snap(&mut self, s: &mut Snap) {
        s.tag(b"SI  ");
        guest_mem::snap!(s; self.regs, self.buf, self.comcsr, self.sr);
    }
}
