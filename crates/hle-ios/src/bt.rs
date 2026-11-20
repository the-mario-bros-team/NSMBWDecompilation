//! `/dev/usb/oh1/57e/305`: the Wii's Bluetooth controller as seen over USB,
//! with up to four emulated Wii Remotes paired to it (plan section 2.4).
//!
//! HCI commands arrive as control messages; HCI events go back on the
//! interrupt endpoint (0x81), and ACL data travels on the bulk endpoints
//! (0x02 out, 0x82 in), each answering a request the game left pending.
//!
//! Connection flow, as on hardware and in Dolphin's emulated remotes:
//! 1. The remote pages the Wii once it enables page scan (Connection
//!    Request); the Wii accepts, and the link comes up.
//! 2. The remote opens the HID control (PSM 0x11) and interrupt (PSM 0x13)
//!    L2CAP channels and both sides configure them.
//! 3. HID reports flow on the interrupt channel ([`crate::wiimote`]).

use std::collections::VecDeque;

use crate::wiimote::{Input, Wiimote, REPORTS_PER_SECOND};

/// A pending request: (request address, buffer address, buffer length).
pub type Waiting = (u32, u32, u32);

pub const REMOTES: usize = 4;
const CPU_HZ: u64 = 729_000_000;
/// The remote opens its HID channels this long after the link is up.
const CHANNEL_DELAY: u64 = CPU_HZ / 100;
/// Data reports wait while this many ACL packets are already queued for
/// the Wii, so a slow reader does not build up a backlog.
const MAX_QUEUED_ACL: usize = 4;

const PSM_HID_CONTROL: u16 = 0x11;
const PSM_HID_INTERRUPT: u16 = 0x13;
const CID_SIGNALING: u16 = 0x0001;
/// The remote's own channel ids (each link has its own id space).
const CID_CONTROL: u16 = 0x0040;
const CID_INTERRUPT: u16 = 0x0041;
const L2CAP_MTU: u16 = 185;

const EV_INQUIRY_COMPLETE: u8 = 0x01;
const EV_CONNECTION_COMPLETE: u8 = 0x03;
const EV_CONNECTION_REQUEST: u8 = 0x04;
const EV_DISCONNECTION_COMPLETE: u8 = 0x05;
const EV_AUTHENTICATION_COMPLETE: u8 = 0x06;
const EV_REMOTE_NAME: u8 = 0x07;
const EV_ENCRYPTION_CHANGE: u8 = 0x08;
const EV_REMOTE_FEATURES: u8 = 0x0B;
const EV_REMOTE_VERSION: u8 = 0x0C;
const EV_QOS_SETUP_COMPLETE: u8 = 0x0D;
const EV_COMMAND_COMPLETE: u8 = 0x0E;
const EV_COMMAND_STATUS: u8 = 0x0F;
const EV_ROLE_CHANGE: u8 = 0x12;
const EV_COMPLETED_PACKETS: u8 = 0x13;
const EV_MODE_CHANGE: u8 = 0x14;
const EV_RETURN_LINK_KEYS: u8 = 0x15;
const EV_CLOCK_OFFSET: u8 = 0x1C;
const EV_PACKET_TYPE_CHANGED: u8 = 0x1D;

const STATUS_PAGE_TIMEOUT: u8 = 0x04;
const STATUS_UNKNOWN_CONNECTION: u8 = 0x02;

/// HCI-order (least significant byte first) address of remote `i`, as
/// Dolphin assigns them.
pub fn remote_address(i: usize) -> [u8; 6] {
    [0x11, 0x02, 0x19, 0x79, 0x00, i as u8]
}

fn link_key(i: usize) -> [u8; 16] {
    let mut k = [0u8; 16];
    k[0] = 0xA0 + i as u8;
    k
}

#[derive(Clone, Copy, Default)]
struct Channel {
    /// The Wii's channel id.
    remote: u16,
    /// A connection request was sent or accepted.
    requested: bool,
    open: bool,
    /// The Wii accepted our configuration, and we accepted the Wii's.
    ours_done: bool,
    theirs_done: bool,
}

impl Channel {
    fn ready(&self) -> bool {
        self.open && self.ours_done && self.theirs_done
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Link {
    Off,
    /// Connection Request sent; waiting for the Wii to accept.
    Paging,
    Up {
        since: u64,
        /// The Wii created this link, so it opens the channels.
        wii_opens: bool,
    },
}

struct Remote {
    hid: Wiimote,
    link: Link,
    /// The Wii dropped the link: stay off until a button is pressed.
    asleep: bool,
    /// Page the Wii even with page scan off (a player joining online in a
    /// course, where the game turns page scan off).
    force_page: bool,
    control: Channel,
    interrupt: Channel,
    next_report: u64,
    /// ACL fragments being reassembled into an L2CAP frame.
    rx: Vec<u8>,
}

impl Remote {
    fn new() -> Self {
        Self { hid: Wiimote::new(), link: Link::Off, asleep: false, force_page: false, control: Channel::default(), interrupt: Channel::default(), next_report: 0, rx: Vec::new() }
    }

    fn channel(&mut self, local: u16) -> Option<&mut Channel> {
        match local {
            CID_CONTROL => Some(&mut self.control),
            CID_INTERRUPT => Some(&mut self.interrupt),
            _ => None,
        }
    }

    fn drop_link(&mut self) {
        let input = self.hid.input;
        *self = Self::new();
        self.hid.input = input;
    }
}

pub struct Bt {
    events: VecDeque<Vec<u8>>,
    /// ACL packets for the Wii.
    acl: VecDeque<Vec<u8>>,
    pub intr_waiting: VecDeque<Waiting>,
    pub bulk_waiting: VecDeque<Waiting>,
    pub commands: Vec<u16>,
    scan_enable: u8,
    now: u64,
    ident: u8,
    remotes: Vec<Remote>,
    /// Print the HCI and L2CAP conversation.
    pub trace: bool,
}

impl Default for Bt {
    fn default() -> Self {
        Self {
            events: VecDeque::new(),
            acl: VecDeque::new(),
            intr_waiting: VecDeque::new(),
            bulk_waiting: VecDeque::new(),
            commands: Vec::new(),
            scan_enable: 0,
            now: 0,
            ident: 0,
            remotes: (0..REMOTES).map(|_| Remote::new()).collect(),
            trace: false,
        }
    }
}

fn le16(b: &[u8], at: usize) -> u16 {
    b.get(at..at + 2).map_or(0, |s| u16::from_le_bytes([s[0], s[1]]))
}

fn handle_of(i: usize) -> u16 {
    0x100 + i as u16
}

/// One remote's state as the frontend shows it.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RemoteStatus {
    /// Both HID channels are configured.
    pub connected: bool,
    pub leds: u8,
    pub rumble: bool,
}

impl Bt {
    fn event(&mut self, code: u8, params: &[u8]) {
        let mut e = vec![code, params.len() as u8];
        e.extend_from_slice(params);
        self.events.push_back(e);
    }

    fn complete(&mut self, opcode: u16, ret: &[u8]) {
        let mut p = vec![1];
        p.extend_from_slice(&opcode.to_le_bytes());
        p.push(0); // status: success
        p.extend_from_slice(ret);
        self.event(EV_COMMAND_COMPLETE, &p);
    }

    fn status(&mut self, opcode: u16, status: u8) {
        let [lo, hi] = opcode.to_le_bytes();
        self.event(EV_COMMAND_STATUS, &[status, 1, lo, hi]);
    }

    fn by_address(&self, a: &[u8]) -> Option<usize> {
        (0..REMOTES).find(|&i| a.get(..6) == Some(&remote_address(i)[..]))
    }

    fn by_handle(&self, h: u16) -> Option<usize> {
        (0..REMOTES).find(|&i| handle_of(i) == h & 0x0FFF && matches!(self.remotes[i].link, Link::Up { .. }))
    }

    /// The event every command on an open link answers with: status, the
    /// connection handle, then `extra`.
    fn link_event(&mut self, opcode: u16, p: &[u8], code: u8, extra: &[u8]) {
        let h = le16(p, 0);
        if self.by_handle(h).is_none() {
            self.status(opcode, STATUS_UNKNOWN_CONNECTION);
            return;
        }
        self.status(opcode, 0);
        let mut e = vec![0];
        e.extend((h & 0x0FFF).to_le_bytes());
        e.extend_from_slice(extra);
        self.event(code, &e);
    }

    fn connection_complete(&mut self, i: usize, wii_opens: bool) {
        let mut e = vec![0];
        e.extend(handle_of(i).to_le_bytes());
        e.extend(remote_address(i));
        e.extend([1, 0]); // ACL link, no encryption
        self.event(EV_CONNECTION_COMPLETE, &e);
        self.remotes[i].link = Link::Up { since: self.now, wii_opens };
        if self.trace {
            println!("  [bt] remote {} linked (handle {:#x})", i + 1, handle_of(i));
        }
    }

    /// Handles one HCI command packet (opcode LE, length, parameters).
    pub fn command(&mut self, packet: &[u8]) {
        if packet.len() < 3 {
            return;
        }
        let opcode = u16::from_le_bytes([packet[0], packet[1]]);
        let p = &packet[3..];
        self.commands.push(opcode);
        if self.trace {
            println!("  [bt] command {opcode:#06x} {p:02x?}");
        }
        match opcode {
            // Inquiry: the remotes are already paired, so nobody answers.
            0x0401 => {
                self.status(opcode, 0);
                self.event(EV_INQUIRY_COMPLETE, &[0]);
            }
            // Create Connection: the Wii pages a paired remote.
            0x0405 => {
                self.status(opcode, 0);
                match self.by_address(p) {
                    Some(i) if self.remotes[i].hid.input.connected && self.remotes[i].link == Link::Off => self.connection_complete(i, true),
                    _ => {
                        let mut e = vec![STATUS_PAGE_TIMEOUT, 0, 0];
                        e.extend(p.get(..6).unwrap_or(&[0; 6]));
                        e.extend([1, 0]);
                        self.event(EV_CONNECTION_COMPLETE, &e);
                    }
                }
            }
            // Disconnect
            0x0406 => {
                let h = le16(p, 0);
                match self.by_handle(h) {
                    Some(i) => {
                        self.status(opcode, 0);
                        let mut e = vec![0];
                        e.extend((h & 0x0FFF).to_le_bytes());
                        e.push(0x16); // terminated by the local host
                        self.event(EV_DISCONNECTION_COMPLETE, &e);
                        self.remotes[i].drop_link();
                        self.remotes[i].asleep = true;
                        if self.trace {
                            println!("  [bt] remote {} disconnected by the Wii", i + 1);
                        }
                    }
                    None => self.status(opcode, STATUS_UNKNOWN_CONNECTION),
                }
            }
            // Accept Connection Request (address, role).
            0x0409 => {
                self.status(opcode, 0);
                if let Some(i) = self.by_address(p) {
                    if p.get(6) == Some(&0) {
                        // The Wii becomes master: role switch.
                        let mut e = vec![0];
                        e.extend(remote_address(i));
                        e.push(0);
                        self.event(EV_ROLE_CHANGE, &e);
                    }
                    self.connection_complete(i, false);
                }
            }
            // Reject Connection Request
            0x040A => {
                self.status(opcode, 0);
                if let Some(i) = self.by_address(p) {
                    let mut e = vec![p.get(6).copied().unwrap_or(0x0F), 0, 0];
                    e.extend(remote_address(i));
                    e.extend([1, 0]);
                    self.event(EV_CONNECTION_COMPLETE, &e);
                    self.remotes[i].link = Link::Off;
                    self.remotes[i].asleep = true;
                }
            }
            // Link key and PIN replies (positive and negative): the address.
            0x040B..=0x040E => self.complete(opcode, p.get(..6).unwrap_or(&[0; 6])),
            // Change Connection Packet Type
            0x040F => {
                let ty = [p.get(2).copied().unwrap_or(0), p.get(3).copied().unwrap_or(0)];
                self.link_event(opcode, p, EV_PACKET_TYPE_CHANGED, &ty);
            }
            0x0411 => self.link_event(opcode, p, EV_AUTHENTICATION_COMPLETE, &[]),
            0x0413 => {
                let on = p.get(2).copied().unwrap_or(0);
                self.link_event(opcode, p, EV_ENCRYPTION_CHANGE, &[on]);
            }
            // Remote Name Request
            0x0419 => {
                self.status(opcode, 0);
                let mut e = vec![0];
                e.extend(p.get(..6).unwrap_or(&[0; 6]));
                let mut name = [0u8; 248];
                name[..19].copy_from_slice(b"Nintendo RVL-CNT-01");
                e.extend(name);
                self.event(EV_REMOTE_NAME, &e);
            }
            0x041B => self.link_event(opcode, p, EV_REMOTE_FEATURES, &[0xBC, 0x02, 0x04, 0x38, 0x08, 0x00, 0x00, 0x00]),
            0x041D => self.link_event(opcode, p, EV_REMOTE_VERSION, &[0x02, 0x0F, 0x00, 0x29, 0x02]),
            0x041F => self.link_event(opcode, p, EV_CLOCK_OFFSET, &[0x18, 0x38]),
            op if op >> 10 == 0x01 => self.status(opcode, 0),
            // Sniff Mode (handle, max, min, attempt, timeout) / Exit Sniff Mode.
            0x0803 => {
                let interval = [p.get(2).copied().unwrap_or(0), p.get(3).copied().unwrap_or(0)];
                self.link_event(opcode, p, EV_MODE_CHANGE, &[2, interval[0], interval[1]]);
            }
            0x0804 => self.link_event(opcode, p, EV_MODE_CHANGE, &[0, 0, 0]),
            // QoS Setup: granted as asked.
            0x0807 => {
                let rest = p.get(2..).unwrap_or(&[]).to_vec();
                self.link_event(opcode, p, EV_QOS_SETUP_COMPLETE, &rest);
            }
            // Read Link Policy Settings
            0x080C => self.complete(opcode, &[p.first().copied().unwrap_or(0), p.get(1).copied().unwrap_or(0), 0x0F, 0]),
            // Write Link Policy Settings, Write Link Supervision Timeout,
            // Flush: the handle.
            0x080D | 0x0C37 | 0x0C08 => self.complete(opcode, &[p.first().copied().unwrap_or(0), p.get(1).copied().unwrap_or(0)]),
            // Read Local Version Information: HCI 3, Broadcom.
            0x1001 => self.complete(opcode, &[3, 0xA7, 0x40, 3, 0x0F, 0x00, 0x0E, 0x43]),
            // Read Local Supported Features.
            0x1003 => self.complete(opcode, &[0xFF, 0xFF, 0x8D, 0xFE, 0x9B, 0xF9, 0x00, 0x80]),
            // Read Buffer Size: ACL 339 bytes x 10, SCO 64 x 0.
            0x1005 => self.complete(opcode, &[0x53, 0x01, 0x40, 0x0A, 0x00, 0x00, 0x00]),
            // Read BD_ADDR.
            0x1009 => self.complete(opcode, &[0x79, 0x19, 0x02, 0x11, 0x00, 0x00]),
            // Read Stored Link Key: one key per remote, as Dolphin does.
            0x0C0D => {
                let mut e = vec![REMOTES as u8];
                for i in 0..REMOTES {
                    e.extend(remote_address(i));
                    e.extend(link_key(i));
                }
                self.event(EV_RETURN_LINK_KEYS, &e);
                self.complete(opcode, &[0xFF, 0x00, REMOTES as u8, 0x00]);
            }
            // Write Stored Link Key: number written.
            0x0C11 => self.complete(opcode, &[p.first().copied().unwrap_or(0)]),
            // Delete Stored Link Key: none deleted.
            0x0C12 => self.complete(opcode, &[0, 0]),
            // Read Local Name.
            0x0C14 => self.complete(opcode, &[0; 248]),
            // Read Page Timeout, Read Voice Setting.
            0x0C17 | 0x0C25 => self.complete(opcode, &[0, 0]),
            // Write Scan Enable: the remotes page the Wii once page scan is on.
            0x0C1A => {
                self.scan_enable = p.first().copied().unwrap_or(0);
                self.complete(opcode, &[]);
            }
            // Read Scan Enable, Read Authentication Enable.
            0x0C19 => self.complete(opcode, &[self.scan_enable]),
            0x0C1F => self.complete(opcode, &[0]),
            // Read Class of Device.
            0x0C23 => self.complete(opcode, &[0, 0, 0]),
            // Host Number Of Completed Packets: no reply.
            0x0C35 => {}
            // Everything else (resets, writes, vendor commands): status only.
            _ => self.complete(opcode, &[]),
        }
    }

    /// Sends an L2CAP frame on remote `i`'s link.
    fn send(&mut self, i: usize, cid: u16, payload: &[u8]) {
        let l2 = payload.len() + 4;
        let mut p = Vec::with_capacity(l2 + 4);
        p.extend((0x2000 | handle_of(i)).to_le_bytes()); // first, auto-flushable
        p.extend((l2 as u16).to_le_bytes());
        p.extend((payload.len() as u16).to_le_bytes());
        p.extend(cid.to_le_bytes());
        p.extend_from_slice(payload);
        self.acl.push_back(p);
    }

    fn signal(&mut self, i: usize, code: u8, ident: u8, data: &[u8]) {
        let mut s = vec![code, ident];
        s.extend((data.len() as u16).to_le_bytes());
        s.extend_from_slice(data);
        self.send(i, CID_SIGNALING, &s);
    }

    fn next_ident(&mut self) -> u8 {
        self.ident = self.ident.wrapping_add(1).max(1);
        self.ident
    }

    fn connect_channel(&mut self, i: usize, psm: u16, local: u16) {
        if self.trace {
            println!("  [bt] remote {} opens PSM {psm:#x}", i + 1);
        }
        let id = self.next_ident();
        let mut d = psm.to_le_bytes().to_vec();
        d.extend(local.to_le_bytes());
        self.signal(i, 0x02, id, &d);
    }

    fn configure(&mut self, i: usize, remote_cid: u16) {
        let id = self.next_ident();
        let mut d = remote_cid.to_le_bytes().to_vec();
        d.extend([0, 0]); // flags
        d.extend([0x01, 0x02]); // MTU option
        d.extend(L2CAP_MTU.to_le_bytes());
        self.signal(i, 0x04, id, &d);
    }

    /// An ACL packet from the Wii.
    pub fn acl(&mut self, packet: &[u8]) {
        let (h, len) = (le16(packet, 0), le16(packet, 2) as usize);
        let Some(data) = packet.get(4..4 + len) else { return };
        let mut ev = vec![1];
        ev.extend((h & 0x0FFF).to_le_bytes());
        ev.extend(1u16.to_le_bytes());
        self.event(EV_COMPLETED_PACKETS, &ev);
        let Some(i) = self.by_handle(h) else { return };
        let r = &mut self.remotes[i];
        if (h >> 12) & 3 == 1 {
            r.rx.extend_from_slice(data);
        } else {
            r.rx = data.to_vec();
        }
        let need = le16(&r.rx, 0) as usize + 4;
        if r.rx.len() < need {
            return;
        }
        let frame = std::mem::take(&mut r.rx);
        let cid = le16(&frame, 2);
        let payload = &frame[4..need];
        match cid {
            CID_SIGNALING => {
                let mut at = 0;
                while at + 4 <= payload.len() {
                    let n = le16(payload, at + 2) as usize;
                    let end = (at + 4 + n).min(payload.len());
                    self.signaling(i, payload[at], payload[at + 1], &payload[at + 4..end]);
                    at += 4 + n;
                }
            }
            CID_CONTROL | CID_INTERRUPT => self.hid(i, cid, payload),
            _ => {}
        }
    }

    fn signaling(&mut self, i: usize, code: u8, ident: u8, d: &[u8]) {
        if self.trace {
            println!("  [bt] remote {} L2CAP signal {code:#04x} {d:02x?}", i + 1);
        }
        match code {
            // Connection Request from the Wii.
            0x02 => {
                let (psm, scid) = (le16(d, 0), le16(d, 2));
                let local = match psm {
                    PSM_HID_CONTROL => CID_CONTROL,
                    PSM_HID_INTERRUPT => CID_INTERRUPT,
                    _ => {
                        let mut r = vec![0, 0];
                        r.extend(scid.to_le_bytes());
                        r.extend([0x02, 0x00, 0, 0]); // PSM not supported
                        self.signal(i, 0x03, ident, &r);
                        return;
                    }
                };
                let ch = self.remotes[i].channel(local).unwrap();
                *ch = Channel { remote: scid, requested: true, open: true, ..Channel::default() };
                let mut r = local.to_le_bytes().to_vec();
                r.extend(scid.to_le_bytes());
                r.extend([0, 0, 0, 0]);
                self.signal(i, 0x03, ident, &r);
                self.configure(i, scid);
            }
            // Connection Response to ours.
            0x03 => {
                let (dcid, scid, result) = (le16(d, 0), le16(d, 2), le16(d, 4));
                let Some(ch) = self.remotes[i].channel(scid) else { return };
                match result {
                    0 => {
                        ch.remote = dcid;
                        ch.open = true;
                        self.configure(i, dcid);
                    }
                    1 => {} // pending
                    // Refused (for example a security block): the remote gives
                    // up; the Wii will drop the link.
                    _ => {
                        if self.trace {
                            println!("  [bt] remote {} channel {scid:#x} refused: result {result}", i + 1);
                        }
                    }
                }
            }
            // Configuration Request: accept as given.
            0x04 => {
                let Some(ch) = self.remotes[i].channel(le16(d, 0)) else { return };
                ch.theirs_done = true;
                let mut r = ch.remote.to_le_bytes().to_vec();
                r.extend([0, 0, 0, 0]);
                self.signal(i, 0x05, ident, &r);
            }
            // Configuration Response to ours.
            0x05 => {
                if let Some(ch) = self.remotes[i].channel(le16(d, 0)) {
                    ch.ours_done = le16(d, 4) == 0;
                }
            }
            // Disconnection Request.
            0x06 => {
                let (dcid, scid) = (le16(d, 0), le16(d, 2));
                if let Some(ch) = self.remotes[i].channel(dcid) {
                    *ch = Channel { requested: true, ..Channel::default() };
                }
                let mut r = dcid.to_le_bytes().to_vec();
                r.extend(scid.to_le_bytes());
                self.signal(i, 0x07, ident, &r);
            }
            0x07 => {}
            // Information Request: not supported.
            0x0A => {
                let mut r = d.get(..2).unwrap_or(&[0, 0]).to_vec();
                r.extend([1, 0]);
                self.signal(i, 0x0B, ident, &r);
            }
            _ => self.signal(i, 0x01, ident, &[0, 0]),
        }
    }

    /// A HID transaction: DATA (0xA2) on either channel, or SET_REPORT
    /// (0x5x) on the control channel, which gets a handshake.
    fn hid(&mut self, i: usize, cid: u16, d: &[u8]) {
        let Some((&head, report)) = d.split_first() else { return };
        if cid == CID_CONTROL && head >> 4 == 0x5 {
            let remote = self.remotes[i].control.remote;
            self.send(i, remote, &[0x00]);
        } else if head != 0xA2 {
            return;
        }
        if self.trace {
            println!("  [bt] remote {} output report {report:02x?}", i + 1);
        }
        let secs = self.now as f64 / CPU_HZ as f64;
        let mut out = Vec::new();
        self.remotes[i].hid.output(report, secs, &mut out);
        let to = self.remotes[i].interrupt.remote;
        for r in out {
            let mut p = vec![0xA1];
            p.extend(r);
            self.send(i, to, &p);
        }
    }

    /// Advances the remotes to guest cycle `now`: connection requests,
    /// channel setup, and data reports.
    pub fn update(&mut self, now: u64) {
        self.now = now;
        for i in 0..REMOTES {
            let r = &mut self.remotes[i];
            if r.asleep && r.hid.input.buttons != 0 {
                r.asleep = false;
            }
            match r.link {
                Link::Off if r.hid.input.connected && !r.asleep && (self.scan_enable & 2 != 0 || r.force_page) => {
                    r.link = Link::Paging;
                    r.force_page = false;
                    let mut e = remote_address(i).to_vec();
                    e.extend([0x04, 0x25, 0x00, 1]); // class: Wii Remote; ACL
                    self.event(EV_CONNECTION_REQUEST, &e);
                    if self.trace {
                        println!("  [bt] remote {} pages the Wii", i + 1);
                    }
                }
                Link::Up { since, wii_opens } => {
                    if !wii_opens && now >= since + CHANNEL_DELAY {
                        if !r.control.requested {
                            r.control.requested = true;
                            self.connect_channel(i, PSM_HID_CONTROL, CID_CONTROL);
                        } else if r.control.ready() && !r.interrupt.requested {
                            self.remotes[i].interrupt.requested = true;
                            self.connect_channel(i, PSM_HID_INTERRUPT, CID_INTERRUPT);
                        }
                    }
                    let r = &mut self.remotes[i];
                    if r.interrupt.ready() && now >= r.next_report && self.acl.len() < MAX_QUEUED_ACL {
                        let period = CPU_HZ / REPORTS_PER_SECOND;
                        r.next_report = if now - r.next_report > 4 * period { now + period } else { r.next_report + period };
                        if let Some(rep) = r.hid.poll(now as f64 / CPU_HZ as f64) {
                            let to = r.interrupt.remote;
                            let mut p = vec![0xA1];
                            p.extend(rep);
                            self.send(i, to, &p);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// Remote `i` connects at its next chance even if the Wii is not
    /// scanning for pages (see `Remote::force_page`).
    pub fn force_connect(&mut self, i: usize) {
        if let Some(r) = self.remotes.get_mut(i) {
            r.force_page = true;
            r.asleep = false;
        }
    }

    pub fn set_input(&mut self, i: usize, input: Input) {
        if let Some(r) = self.remotes.get_mut(i) {
            r.hid.input = input;
        }
    }

    pub fn remote_status(&self, i: usize) -> RemoteStatus {
        self.remotes.get(i).map_or_else(RemoteStatus::default, |r| RemoteStatus {
            connected: r.control.ready() && r.interrupt.ready(),
            leds: r.hid.leds,
            rumble: r.hid.rumble,
        })
    }

    pub fn next_event(&mut self) -> Option<Vec<u8>> {
        self.events.pop_front()
    }

    pub fn has_event(&self) -> bool {
        !self.events.is_empty()
    }

    pub fn next_acl(&mut self) -> Option<Vec<u8>> {
        self.acl.pop_front()
    }

    pub fn has_acl(&self) -> bool {
        !self.acl.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wiimote::button;

    fn cmd(bt: &mut Bt, op: u16, p: &[u8]) {
        let mut c = op.to_le_bytes().to_vec();
        c.push(p.len() as u8);
        c.extend_from_slice(p);
        bt.command(&c);
    }

    fn events(bt: &mut Bt) -> Vec<Vec<u8>> {
        std::iter::from_fn(|| bt.next_event()).collect()
    }

    fn acls(bt: &mut Bt) -> Vec<Vec<u8>> {
        std::iter::from_fn(|| bt.next_acl()).collect()
    }

    /// An ACL packet from the Wii carrying one L2CAP frame.
    fn wii_acl(bt: &mut Bt, cid: u16, payload: &[u8]) {
        let mut p = 0x2100u16.to_le_bytes().to_vec();
        p.extend(((payload.len() + 4) as u16).to_le_bytes());
        p.extend((payload.len() as u16).to_le_bytes());
        p.extend(cid.to_le_bytes());
        p.extend_from_slice(payload);
        bt.acl(&p);
    }

    fn wii_signal(bt: &mut Bt, code: u8, ident: u8, d: &[u8]) {
        let mut s = vec![code, ident];
        s.extend((d.len() as u16).to_le_bytes());
        s.extend_from_slice(d);
        wii_acl(bt, CID_SIGNALING, &s);
    }

    /// The signaling command inside an ACL packet from the remote.
    fn signal_of(p: &[u8]) -> (u8, u8, Vec<u8>) {
        assert_eq!(le16(p, 6), CID_SIGNALING);
        (p[8], p[9], p[12..].to_vec())
    }

    #[test]
    fn remote_connects_and_reports() {
        let mut bt = Bt::default();
        bt.set_input(0, Input { connected: true, ..Input::default() });
        bt.update(0);
        assert!(events(&mut bt).is_empty(), "no paging before page scan is on");
        cmd(&mut bt, 0x0C1A, &[3]);
        bt.update(1);
        let ev = events(&mut bt);
        assert_eq!(ev.last().unwrap()[0], EV_CONNECTION_REQUEST);
        let mut accept = remote_address(0).to_vec();
        accept.push(0);
        cmd(&mut bt, 0x0409, &accept);
        let ev = events(&mut bt);
        assert_eq!(ev.iter().map(|e| e[0]).collect::<Vec<_>>(), [EV_COMMAND_STATUS, EV_ROLE_CHANGE, EV_CONNECTION_COMPLETE]);
        assert_eq!(le16(&ev[2], 3), 0x100);

        // The remote opens the control channel; the Wii accepts and both configure.
        bt.update(2 + CHANNEL_DELAY);
        let a = acls(&mut bt);
        let (code, id, d) = signal_of(&a[0]);
        assert_eq!((code, le16(&d, 0), le16(&d, 2)), (0x02, PSM_HID_CONTROL, CID_CONTROL));
        wii_signal(&mut bt, 0x03, id, &[0x50, 0, 0x40, 0, 0, 0, 0, 0]);
        let (code, cid, _) = signal_of(&acls(&mut bt)[0]);
        assert_eq!((code, cid), (0x04, 0x02));
        wii_signal(&mut bt, 0x05, cid, &[0x40, 0, 0, 0, 0, 0]);
        wii_signal(&mut bt, 0x04, 9, &[0x40, 0, 0, 0, 0x01, 0x02, 0xB9, 0x00]);
        let (code, _, d) = signal_of(&acls(&mut bt)[0]);
        assert_eq!((code, le16(&d, 0), le16(&d, 4)), (0x05, 0x50, 0));
        // Then the interrupt channel.
        bt.update(3 + CHANNEL_DELAY);
        let (code, id, d) = signal_of(&acls(&mut bt)[0]);
        assert_eq!((code, le16(&d, 0)), (0x02, PSM_HID_INTERRUPT));
        wii_signal(&mut bt, 0x03, id, &[0x51, 0, 0x41, 0, 0, 0, 0, 0]);
        let _ = acls(&mut bt);
        wii_signal(&mut bt, 0x05, 0, &[0x41, 0, 0, 0, 0, 0]);
        wii_signal(&mut bt, 0x04, 10, &[0x41, 0, 0, 0]);
        let _ = acls(&mut bt);
        assert!(bt.remote_status(0).connected);

        // The Wii asks for buttons + acceleration; the remote reports at once
        // and then every 5 ms.
        bt.set_input(0, Input { connected: true, buttons: button::TWO, ..Input::default() });
        wii_acl(&mut bt, CID_INTERRUPT, &[0xA2, 0x12, 0x04, 0x31]);
        let a = acls(&mut bt);
        assert_eq!(le16(&a[0], 6), 0x51);
        assert_eq!(&a[0][8..11], &[0xA1, 0x31, 0x00]);
        assert_eq!(a[0][11] & 0x1F, 0x01);
        let t = 4 + CHANNEL_DELAY;
        bt.update(t);
        bt.update(t + CPU_HZ / REPORTS_PER_SECOND);
        assert_eq!(acls(&mut bt).len(), 2);
        // Every ACL packet from the Wii is counted back.
        assert!(events(&mut bt).iter().all(|e| e[0] == EV_COMPLETED_PACKETS));

        // The Wii disconnects: the remote sleeps until a button is pressed.
        cmd(&mut bt, 0x0406, &[0x00, 0x01, 0x13]);
        assert_eq!(events(&mut bt)[1][0], EV_DISCONNECTION_COMPLETE);
        bt.set_input(0, Input { connected: true, ..Input::default() });
        bt.update(t + 10);
        assert!(events(&mut bt).is_empty());
        bt.set_input(0, Input { connected: true, buttons: button::A, ..Input::default() });
        bt.update(t + 11);
        assert_eq!(events(&mut bt)[0][0], EV_CONNECTION_REQUEST);
    }
}

use guest_mem::snap::{Snap, Snapshot};

impl Snapshot for Channel {
    fn snap(&mut self, s: &mut Snap) {
        guest_mem::snap!(s; self.remote, self.requested, self.open, self.ours_done, self.theirs_done);
    }
}

impl Snapshot for Link {
    fn snap(&mut self, s: &mut Snap) {
        let (mut tag, mut since, mut wii_opens) = match *self {
            Link::Off => (0u8, 0u64, false),
            Link::Paging => (1, 0, false),
            Link::Up { since, wii_opens } => (2, since, wii_opens),
        };
        guest_mem::snap!(s; tag, since, wii_opens);
        *self = match tag {
            1 => Link::Paging,
            2 => Link::Up { since, wii_opens },
            _ => Link::Off,
        };
    }
}

impl Default for Remote {
    fn default() -> Self {
        Self::new()
    }
}

impl Snapshot for Remote {
    fn snap(&mut self, s: &mut Snap) {
        guest_mem::snap!(s; self.hid, self.link, self.asleep, self.force_page, self.control, self.interrupt, self.next_report, self.rx);
    }
}

/// The HCI command log (`commands`) is left out.
impl Snapshot for Bt {
    fn snap(&mut self, s: &mut Snap) {
        s.tag(b"BT  ");
        guest_mem::snap!(s; self.events, self.acl, self.intr_waiting, self.bulk_waiting, self.scan_enable, self.now, self.ident, self.remotes);
    }
}
