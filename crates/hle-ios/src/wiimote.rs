//! One emulated Wii Remote's HID report protocol (WiiBrew "Wiimote"):
//! output reports from the Wii (LEDs, reporting mode, status, memory and
//! register reads and writes, IR and speaker switches) and the input reports
//! it answers with. No extension is attached; the IR camera reports no
//! points; the speaker accepts data and plays nothing.
//!
//! Reports here exclude the HID transaction byte (0xA2 out, 0xA1 in).

/// Core buttons as one word: the report's first byte in the high half.
pub mod button {
    pub const LEFT: u16 = 0x0100;
    pub const RIGHT: u16 = 0x0200;
    pub const DOWN: u16 = 0x0400;
    pub const UP: u16 = 0x0800;
    pub const PLUS: u16 = 0x1000;
    pub const TWO: u16 = 0x0001;
    pub const ONE: u16 = 0x0002;
    pub const B: u16 = 0x0004;
    pub const A: u16 = 0x0008;
    pub const MINUS: u16 = 0x0010;
    pub const HOME: u16 = 0x0080;
    pub const ALL: u16 = 0x1F9F;
}

/// What the player is doing with one remote, set by the frontend.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Input {
    /// The remote is switched on (it connects when the Wii scans for it).
    pub connected: bool,
    pub buttons: u16,
    /// Shaking: the accelerometer swings while this is held.
    pub shake: bool,
    /// Sideways tilt, -1 (left) ..= 1 (right), like turning a steering wheel.
    pub tilt: f32,
}

/// Accelerometer calibration (8-bit): zero and one g, as on Dolphin's
/// emulated remote.
const ACCEL_ZERO: u8 = 0x80;
const ACCEL_ONE_G: u8 = 0x9A;
/// The remote reports this often in continuous mode (as Dolphin's does).
pub const REPORTS_PER_SECOND: u64 = 200;
const SHAKE_HZ: f64 = 6.0;
const SHAKE_G: f64 = 2.5;
/// Tilt at full deflection.
const TILT_DEGREES: f64 = 60.0;

const EEPROM_SIZE: usize = 0x1700;

// Error codes in read replies and acknowledgements.
const OK: u8 = 0;
const ERR_UNAVAILABLE: u8 = 7;
const ERR_BAD_ADDRESS: u8 = 8;

pub struct Wiimote {
    eeprom: Vec<u8>,
    /// Register spaces 0xA2 (speaker), 0xA4 (extension), 0xB0 (IR camera).
    speaker: Vec<u8>,
    ir_regs: Vec<u8>,
    pub leds: u8,
    pub rumble: bool,
    ir: bool,
    speaker_on: bool,
    /// Data reporting mode (0x30..=0x3F) and whether to report continuously.
    pub mode: u8,
    continuous: bool,
    pub input: Input,
    last_sent: Option<Vec<u8>>,
}

impl Default for Wiimote {
    fn default() -> Self {
        Self::new()
    }
}

/// The 10-byte accelerometer calibration block (zero, one g, volume, checksum).
fn accel_calibration() -> [u8; 10] {
    let mut c = [ACCEL_ZERO, ACCEL_ZERO, ACCEL_ZERO, 0, ACCEL_ONE_G, ACCEL_ONE_G, ACCEL_ONE_G, 0, 0, 0];
    c[9] = c[..9].iter().fold(0x55u8, |s, &b| s.wrapping_add(b));
    c
}

impl Wiimote {
    pub fn new() -> Self {
        let mut eeprom = vec![0u8; EEPROM_SIZE];
        // IR camera calibration (twice) as Dolphin ships it, then the
        // accelerometer calibration (twice).
        let ir = [0xA1, 0xAA, 0x8B, 0x99, 0xAE, 0x9E, 0x78, 0x30, 0xA7, 0x00, 0x00];
        eeprom[0x00..0x0B].copy_from_slice(&ir);
        eeprom[0x0B..0x16].copy_from_slice(&ir);
        let acc = accel_calibration();
        eeprom[0x16..0x20].copy_from_slice(&acc);
        eeprom[0x20..0x2A].copy_from_slice(&acc);
        Self {
            eeprom,
            speaker: vec![0; 0x100],
            ir_regs: vec![0; 0x100],
            leds: 0,
            rumble: false,
            ir: false,
            speaker_on: false,
            mode: 0x30,
            continuous: false,
            input: Input::default(),
            last_sent: None,
        }
    }

    fn buttons(&self) -> [u8; 2] {
        (self.input.buttons & button::ALL).to_be_bytes()
    }

    /// Acceleration in g at guest time `secs`, for the remote held sideways
    /// and flat (face up): gravity on +Z, tilt rolls it towards Y, and
    /// shaking swings all three axes.
    pub fn accel(&self, secs: f64) -> [f64; 3] {
        let a = f64::from(self.input.tilt.clamp(-1.0, 1.0)) * TILT_DEGREES.to_radians();
        let mut v = [0.0, a.sin(), a.cos()];
        if self.input.shake {
            let s = SHAKE_G * (secs * SHAKE_HZ * std::f64::consts::TAU).sin();
            for c in &mut v {
                *c += s;
            }
        }
        v
    }

    /// Buttons with the accelerometer's low bits folded in, and the three
    /// high bytes.
    fn core_accel(&self, secs: f64) -> ([u8; 2], [u8; 3]) {
        let span = f64::from(ACCEL_ONE_G - ACCEL_ZERO) * 4.0;
        let raw = self.accel(secs).map(|g| (f64::from(ACCEL_ZERO) * 4.0 + g * span).round().clamp(0.0, 1023.0) as u16);
        let mut b = self.buttons();
        b[0] |= ((raw[0] & 3) << 5) as u8;
        b[1] |= (((raw[1] >> 1) & 1) << 5) as u8 | (((raw[2] >> 1) & 1) << 6) as u8;
        (b, raw.map(|r| (r >> 2) as u8))
    }

    /// The current data report in the selected mode.
    pub fn data_report(&self, secs: f64) -> Vec<u8> {
        let (b, acc) = self.core_accel(secs);
        let mut r = vec![self.mode];
        let (with_acc, ir, ext) = match self.mode {
            0x30 => (false, 0, 0),
            0x31 => (true, 0, 0),
            0x32 => (false, 0, 8),
            0x33 => (true, 12, 0),
            0x34 => (false, 0, 19),
            0x35 => (true, 0, 16),
            0x36 => (false, 10, 9),
            0x37 => (true, 10, 6),
            0x3D => {
                r.extend([0u8; 21]);
                return r;
            }
            // 0x3E/0x3F interleave halves of a full IR report: send the
            // buttons and acceleration with an empty IR half.
            _ => (true, 18, 0),
        };
        r.extend(if with_acc { b } else { self.buttons() });
        if with_acc {
            r.extend(acc);
        }
        r.extend(std::iter::repeat_n(0xFF, ir));
        r.extend(std::iter::repeat_n(0, ext));
        r
    }

    /// The report to send now, if any: every period when continuous,
    /// otherwise only when it changed.
    pub fn poll(&mut self, secs: f64) -> Option<Vec<u8>> {
        let r = self.data_report(secs);
        if !self.continuous && self.last_sent.as_ref() == Some(&r) {
            return None;
        }
        self.last_sent = Some(r.clone());
        Some(r)
    }

    pub fn status(&self) -> Vec<u8> {
        let b = self.buttons();
        let flags = (self.speaker_on as u8) << 2 | (self.ir as u8) << 3 | (self.leds << 4);
        vec![0x20, b[0], b[1], flags, 0, 0, 0xC0]
    }

    fn ack(&self, report: u8, error: u8) -> Vec<u8> {
        let b = self.buttons();
        vec![0x22, b[0], b[1], report, error]
    }

    /// Handles one output report; pushes the input reports it causes.
    pub fn output(&mut self, report: &[u8], secs: f64, out: &mut Vec<Vec<u8>>) {
        let Some((&id, data)) = report.split_first() else { return };
        let first = data.first().copied().unwrap_or(0);
        self.rumble = first & 1 != 0;
        let wants_ack = first & 2 != 0;
        let acked = |s: &Self, out: &mut Vec<Vec<u8>>| {
            if wants_ack {
                out.push(s.ack(id, OK));
            }
        };
        match id {
            0x10 => {}
            0x11 => {
                self.leds = first >> 4;
                acked(self, out);
            }
            0x12 => {
                self.continuous = first & 4 != 0;
                self.mode = data.get(1).copied().unwrap_or(0x30);
                self.last_sent = None;
                acked(self, out);
                if let Some(r) = self.poll(secs) {
                    out.push(r);
                }
            }
            0x13 | 0x1A => {
                self.ir = first & 4 != 0;
                acked(self, out);
            }
            0x14 => {
                self.speaker_on = first & 4 != 0;
                acked(self, out);
            }
            0x15 => out.push(self.status()),
            0x16 => {
                let e = self.write(data);
                out.push(self.ack(id, e));
            }
            0x17 => self.read(data, out),
            0x18 | 0x19 => acked(self, out),
            _ => {}
        }
    }

    /// Space byte, 24-bit address, then the size.
    fn address(data: &[u8]) -> Option<(bool, u32)> {
        let d = data.get(..4)?;
        Some((d[0] & 0x0C != 0, u32::from_be_bytes([0, d[1], d[2], d[3]])))
    }

    fn register(&mut self, addr: u32) -> Option<&mut Vec<u8>> {
        match addr >> 16 {
            0xA2 => Some(&mut self.speaker),
            0xB0 => Some(&mut self.ir_regs),
            // 0xA4 (extension) and 0xA6 (MotionPlus): nothing attached.
            _ => None,
        }
    }

    fn write(&mut self, data: &[u8]) -> u8 {
        let Some((regs, addr)) = Self::address(data) else { return ERR_BAD_ADDRESS };
        let size = (data.get(4).copied().unwrap_or(0) as usize).min(16);
        let bytes = data.get(5..5 + size).unwrap_or(&[]);
        let (buf, at) = if regs {
            match self.register(addr) {
                Some(r) => (r, (addr & 0xFF) as usize),
                None => return ERR_UNAVAILABLE,
            }
        } else {
            (&mut self.eeprom, addr as usize)
        };
        for (i, &b) in bytes.iter().enumerate() {
            match buf.get_mut(at + i) {
                Some(d) => *d = b,
                None => return ERR_BAD_ADDRESS,
            }
        }
        OK
    }

    /// A read answers in 16-byte replies (0x21).
    fn read(&mut self, data: &[u8], out: &mut Vec<Vec<u8>>) {
        let Some((regs, addr)) = Self::address(data) else { return };
        let size = data.get(4..6).map_or(0, |s| u16::from_be_bytes([s[0], s[1]]) as usize);
        let b = self.buttons();
        let reply = |error: u8, at: u32, chunk: &[u8]| {
            let mut r = vec![0x21, b[0], b[1], ((chunk.len().max(1) as u8 - 1) << 4) | error];
            r.extend((at as u16).to_be_bytes());
            let mut d = [0u8; 16];
            d[..chunk.len()].copy_from_slice(chunk);
            r.extend(d);
            r
        };
        let (buf, base): (&[u8], usize) = if regs {
            match self.register(addr) {
                Some(r) => (r, (addr & 0xFF) as usize),
                None => {
                    out.push(reply(ERR_UNAVAILABLE, addr, &[]));
                    return;
                }
            }
        } else {
            (&self.eeprom, addr as usize)
        };
        if base + size > buf.len() {
            out.push(reply(ERR_BAD_ADDRESS, addr, &[]));
            return;
        }
        for off in (0..size).step_by(16) {
            let n = (size - off).min(16);
            out.push(reply(OK, addr + off as u32, &buf[base + off..base + off + n]));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_calibration_and_reports_buttons() {
        let mut w = Wiimote::new();
        let mut out = Vec::new();
        // Read 10 bytes of EEPROM at 0x16.
        w.output(&[0x17, 0x00, 0x00, 0x00, 0x16, 0x00, 0x0A], 0.0, &mut out);
        assert_eq!(out.len(), 1);
        let r = &out[0];
        assert_eq!((r[0], r[3], &r[4..6]), (0x21, 0x90, &[0x00, 0x16][..]));
        assert_eq!(&r[6..16], &accel_calibration());
        assert_eq!(accel_calibration()[9], 0xA3);
        // No extension: reading its ID fails with "unavailable".
        out.clear();
        w.output(&[0x17, 0x04, 0xA4, 0x00, 0xFA, 0x00, 0x06], 0.0, &mut out);
        assert_eq!(out[0][3] & 0x0F, ERR_UNAVAILABLE);
        // Buttons and acceleration, continuous.
        out.clear();
        w.input.buttons = button::TWO | button::RIGHT;
        w.output(&[0x12, 0x04, 0x31], 0.0, &mut out);
        let r = out.last().unwrap();
        assert_eq!(r[0], 0x31);
        assert_eq!((r[1] & 0x1F, r[2] & 0x9F), (0x02, 0x01));
        // At rest: X and Y at zero g, Z at one g.
        assert_eq!(&r[3..6], &[ACCEL_ZERO, ACCEL_ZERO, ACCEL_ONE_G]);
        assert!(w.poll(0.005).is_some(), "continuous mode reports every period");
    }

    #[test]
    fn writes_registers_and_acks() {
        let mut w = Wiimote::new();
        let mut out = Vec::new();
        let mut rep = vec![0x16, 0x04, 0xB0, 0x00, 0x30, 0x01, 0x08];
        rep.resize(22, 0);
        w.output(&rep, 0.0, &mut out);
        assert_eq!(out, vec![vec![0x22, 0, 0, 0x16, OK]]);
        assert_eq!(w.ir_regs[0x30], 0x08);
        out.clear();
        w.output(&[0x11, 0x12], 0.0, &mut out);
        assert_eq!((w.leds, out.len()), (1, 1));
        out.clear();
        w.output(&[0x15, 0x00], 0.0, &mut out);
        assert_eq!(out[0][..4], [0x20, 0, 0, 0x10]);
    }
}

use guest_mem::snap::{Snap, Snapshot};

impl Snapshot for Input {
    fn snap(&mut self, s: &mut Snap) {
        guest_mem::snap!(s; self.connected, self.buttons, self.shake, self.tilt);
    }
}

impl Snapshot for Wiimote {
    fn snap(&mut self, s: &mut Snap) {
        guest_mem::snap!(s; self.eeprom, self.speaker, self.ir_regs, self.leds, self.rumble, self.ir, self.speaker_on, self.mode, self.continuous, self.input, self.last_sent);
    }
}
