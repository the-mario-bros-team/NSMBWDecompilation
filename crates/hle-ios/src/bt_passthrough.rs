//! Bluetooth passthrough (experimental): the game's own Bluetooth stack
//! drives a real USB Bluetooth adapter, so real Wii Remotes pair and
//! connect to it as on a Wii. After WitherBlitz's NSMB-Wii-Recompilation
//! (`runtime/src/bt`) and Dolphin's real-adapter mode.
//!
//! The game's HCI commands go to the adapter and its events and ACL data
//! come back, with these changes, since the game's stack was made for the
//! Wii's own Broadcom module:
//! - The module's firmware patch (vendor commands 0xFC4C and 0xFC4F)
//!   "succeeds" without reaching the adapter.
//! - Other adapters keep no pairings, so the link keys are kept here
//!   ([`Passthrough::link_keys`]): Read, Write and Delete Stored Link Key
//!   are answered from them, and each new pairing (Link Key Notification)
//!   is added.
//! - Read Buffer Size says the Wii module's 339-byte ACL buffers, at most
//!   ten of them, whatever the adapter's are.
//! - Once a remote connects, a QoS Setup asks for guaranteed service (some
//!   adapters deliver its reports well under 200 a second otherwise); its
//!   reply is not passed on.
//! - The console's sync button is a vendor event (0xFF): pressed, it looks
//!   for remotes to pair; held, it forgets them all.
//! - Commands wait for the adapter's command credits (with a 1 s timeout
//!   for an adapter that never answers).

use std::collections::{BTreeMap, HashMap, VecDeque};
use std::time::{Duration, Instant};

/// A USB Bluetooth adapter's HCI endpoints.
pub trait Adapter: Send {
    /// An HCI command (opcode, length, parameters).
    fn send_command(&mut self, packet: &[u8]);
    /// An ACL data packet (handle and flags, length, data).
    fn send_acl(&mut self, packet: &[u8]);
    /// The next transfer from the event endpoint (one or more events, or
    /// part of one).
    fn recv_events(&mut self) -> Option<Vec<u8>>;
    /// The next transfer from the ACL endpoint.
    fn recv_acl(&mut self) -> Option<Vec<u8>>;
    /// Why the adapter stopped working, once it has.
    fn failed(&self) -> Option<String>;
}

pub type Address = [u8; 6];
pub type LinkKey = [u8; 16];

const OP_RESET: u16 = 0x0C03;
const OP_READ_STORED_LINK_KEY: u16 = 0x0C0D;
const OP_WRITE_STORED_LINK_KEY: u16 = 0x0C11;
const OP_DELETE_STORED_LINK_KEY: u16 = 0x0C12;
const OP_READ_BUFFER_SIZE: u16 = 0x1005;
const OP_QOS_SETUP: u16 = 0x0807;
const OP_BROADCOM_WRITE_RAM: u16 = 0xFC4C;
const OP_BROADCOM_LAUNCH_RAM: u16 = 0xFC4F;
const EV_CONNECTION_COMPLETE: u8 = 0x03;
const EV_DISCONNECTION_COMPLETE: u8 = 0x05;
const EV_COMMAND_COMPLETE: u8 = 0x0E;
const EV_COMMAND_STATUS: u8 = 0x0F;
const EV_RETURN_LINK_KEYS: u8 = 0x15;
const EV_LINK_KEY_NOTIFICATION: u8 = 0x18;
const EV_VENDOR: u8 = 0xFF;
const SYNC_PRESSED: u8 = 0x08;
const SYNC_HELD: u8 = 0x09;
const WII_ACL_SIZE: u16 = 339;
const WII_ACL_COUNT: u16 = 10;
const WII_SCO_SIZE: u8 = 64;
const LINK_KEY_ROOM: u16 = 255;
/// Per queue: a few seconds of four remotes' reports. A game that stops
/// reading for longer loses the oldest.
const MAX_QUEUED: usize = 8192;
const COMMAND_TIMEOUT: Duration = Duration::from_secs(1);
/// Lines said per kind of command or event (`log`).
const LOGGED_PER_KIND: u32 = 4;

/// HCI Reset, sent when the game is done with the adapter
/// ([`Passthrough::shutdown`]).
pub const RESET: [u8; 3] = [OP_RESET as u8, (OP_RESET >> 8) as u8, 0];

fn le16(b: &[u8], at: usize) -> u16 {
    b.get(at..at + 2).map_or(0, |s| u16::from_le_bytes([s[0], s[1]]))
}

/// Splits a byte stream into packets: events (2-byte header, length at 1)
/// or ACL data (4-byte header, length at 2).
#[derive(Default)]
struct Assembler {
    buf: Vec<u8>,
    acl: bool,
}

impl Assembler {
    /// Adds `data`; returns the whole packets, and how many bytes were
    /// dropped as no longer on a packet boundary.
    fn feed(&mut self, data: &[u8]) -> (Vec<Vec<u8>>, usize) {
        self.buf.extend_from_slice(data);
        let (header, limit) = if self.acl { (4, 4 + 4096) } else { (2, 2 + 255) };
        let mut out = Vec::new();
        let mut at = 0;
        let mut dropped = 0;
        while self.buf.len() - at >= header {
            let total = header + if self.acl { usize::from(le16(&self.buf, at + 2)) } else { usize::from(self.buf[at + 1]) };
            if total > limit {
                dropped = self.buf.len() - at;
                at = self.buf.len();
                break;
            }
            if self.buf.len() - at < total {
                break;
            }
            out.push(self.buf[at..at + total].to_vec());
            at += total;
        }
        self.buf.drain(..at);
        (out, dropped)
    }
}

/// Counts for the log at the end.
#[derive(Clone, Copy, Debug, Default)]
pub struct Stats {
    pub commands_from_game: u64,
    pub commands_to_adapter: u64,
    pub events_from_adapter: u64,
    pub acl_out: u64,
    pub acl_in: u64,
    pub dropped: u64,
    pub command_timeouts: u64,
}

impl Stats {
    /// The line for the console at the end.
    pub fn summary(&self) -> String {
        format!(
            "{} commands from the game ({} sent), {} events, ACL {} out / {} in, {} dropped, {} command timeouts",
            self.commands_from_game, self.commands_to_adapter, self.events_from_adapter, self.acl_out, self.acl_in, self.dropped, self.command_timeouts
        )
    }
}

pub struct Passthrough {
    adapter: Box<dyn Adapter>,
    /// The adapter is a Wii's own module (057e:0305), which keeps its own
    /// pairings and buffers.
    wii_module: bool,
    pub link_keys: BTreeMap<Address, LinkKey>,
    keys_changed: bool,
    /// Commands waiting for a credit: the packet, and whether it is ours
    /// (its reply is not passed on).
    commands: VecDeque<(Vec<u8>, bool)>,
    credits: u8,
    last_sent: Instant,
    /// Our commands' replies still to come, by opcode.
    ours: HashMap<u16, u32>,
    events: VecDeque<Vec<u8>>,
    acl_in: VecDeque<Vec<u8>>,
    event_bytes: Assembler,
    acl_bytes: Assembler,
    logged: HashMap<u32, u32>,
    /// Commands from the game, by opcode.
    pub command_counts: BTreeMap<u16, u64>,
    /// What happened, for the console (`take_log`).
    log: Vec<String>,
    /// The adapter's failure, said once.
    failed: bool,
    pub stats: Stats,
}

impl Passthrough {
    pub fn new(adapter: Box<dyn Adapter>, wii_module: bool, link_keys: BTreeMap<Address, LinkKey>) -> Self {
        Self {
            adapter,
            wii_module,
            link_keys,
            keys_changed: false,
            commands: VecDeque::new(),
            credits: 1,
            last_sent: Instant::now(),
            ours: HashMap::new(),
            events: VecDeque::new(),
            acl_in: VecDeque::new(),
            event_bytes: Assembler::default(),
            acl_bytes: Assembler { acl: true, ..Assembler::default() },
            logged: HashMap::new(),
            command_counts: BTreeMap::new(),
            log: Vec::new(),
            failed: false,
            stats: Stats::default(),
        }
    }

    fn should_log(&mut self, key: u32) -> bool {
        let n = self.logged.entry(key).or_insert(0);
        *n += 1;
        *n <= LOGGED_PER_KIND
    }

    fn push_event(&mut self, e: Vec<u8>, front: bool) {
        if self.events.len() >= MAX_QUEUED {
            self.events.pop_front();
            self.stats.dropped += 1;
        }
        if front {
            self.events.push_front(e);
        } else {
            self.events.push_back(e);
        }
    }

    fn complete(&mut self, opcode: u16, ret: &[u8]) {
        let mut e = vec![EV_COMMAND_COMPLETE, 3 + ret.len() as u8, 1];
        e.extend(opcode.to_le_bytes());
        e.extend_from_slice(ret);
        self.push_event(e, false);
    }

    /// An HCI command from the game.
    pub fn command(&mut self, packet: &[u8]) {
        self.stats.commands_from_game += 1;
        if packet.len() < 3 {
            return;
        }
        let opcode = le16(packet, 0);
        *self.command_counts.entry(opcode).or_insert(0) += 1;
        let p = &packet[3..3 + usize::from(packet[2]).min(packet.len() - 3)];
        let answered = !self.wii_module && self.answer(opcode, p);
        if self.should_log(u32::from(opcode)) {
            self.log.push(format!("game: HCI command {opcode:#06x} ({} bytes){}", p.len(), if answered { ", answered here" } else { "" }));
        }
        if !answered {
            self.commands.push_back((packet.to_vec(), false));
        }
    }

    /// Answers the commands a non-Wii adapter can't; false for the rest.
    fn answer(&mut self, opcode: u16, p: &[u8]) -> bool {
        match opcode {
            OP_BROADCOM_WRITE_RAM | OP_BROADCOM_LAUNCH_RAM => self.complete(opcode, &[0]),
            OP_READ_STORED_LINK_KEY => {
                let all = p.get(6).is_some_and(|&b| b != 0);
                let keys: Vec<(Address, LinkKey)> = self.link_keys.iter().filter(|(a, _)| all || p.get(..6) == Some(&a[..])).map(|(a, k)| (*a, *k)).collect();
                for chunk in keys.chunks(11) {
                    let mut e = vec![EV_RETURN_LINK_KEYS, 1 + 22 * chunk.len() as u8, chunk.len() as u8];
                    for (a, k) in chunk {
                        e.extend_from_slice(a);
                        e.extend_from_slice(k);
                    }
                    self.push_event(e, false);
                }
                let mut ret = vec![0];
                ret.extend(LINK_KEY_ROOM.to_le_bytes());
                ret.extend((keys.len() as u16).to_le_bytes());
                self.complete(opcode, &ret);
            }
            OP_WRITE_STORED_LINK_KEY => {
                let n = p.first().map_or(0, |&n| usize::from(n).min((p.len() - 1) / 22));
                for entry in p.get(1..).unwrap_or(&[]).chunks_exact(22).take(n) {
                    self.link_keys.insert(entry[..6].try_into().unwrap(), entry[6..].try_into().unwrap());
                }
                self.keys_changed |= n != 0;
                self.complete(opcode, &[0, n as u8]);
            }
            OP_DELETE_STORED_LINK_KEY => {
                let n = if p.get(6).is_some_and(|&b| b != 0) {
                    std::mem::take(&mut self.link_keys).len()
                } else {
                    p.get(..6).map_or(0, |a| usize::from(self.link_keys.remove(a).is_some()))
                };
                self.keys_changed |= n != 0;
                let mut ret = vec![0];
                ret.extend((n as u16).to_le_bytes());
                self.complete(opcode, &ret);
            }
            _ => return false,
        }
        true
    }

    /// ACL data from the game: sent on, paced by the adapter's own flow
    /// control (Number Of Completed Packets, which goes to the game).
    pub fn acl(&mut self, packet: &[u8]) {
        self.stats.acl_out += 1;
        self.adapter.send_acl(packet);
    }

    /// The console's sync button: pressed (pair the remotes that press
    /// theirs), or held for 10 s (forget every pairing). Goes before
    /// anything else waiting for the game.
    pub fn sync_button(&mut self, held: bool) {
        self.push_event(vec![EV_VENDOR, 1, if held { SYNC_HELD } else { SYNC_PRESSED }], true);
    }

    /// Moves what the adapter sent to the game's queues, and waiting
    /// commands to the adapter as it has room.
    pub fn pump(&mut self) {
        self.pump_at(Instant::now());
    }

    fn pump_at(&mut self, now: Instant) {
        while let Some(bytes) = self.adapter.recv_events() {
            let (events, dropped) = self.event_bytes.feed(&bytes);
            self.stats.dropped += dropped as u64;
            for e in events {
                self.adapter_event(e);
            }
        }
        while let Some(bytes) = self.adapter.recv_acl() {
            let (packets, dropped) = self.acl_bytes.feed(&bytes);
            self.stats.dropped += dropped as u64;
            for p in packets {
                self.stats.acl_in += 1;
                if self.acl_in.len() >= MAX_QUEUED {
                    self.acl_in.pop_front();
                    self.stats.dropped += 1;
                }
                self.acl_in.push_back(p);
            }
        }
        while !self.commands.is_empty() {
            if self.credits == 0 {
                if now.duration_since(self.last_sent) < COMMAND_TIMEOUT {
                    break;
                }
                // The adapter never answered the last one.
                self.credits = 1;
                self.stats.command_timeouts += 1;
            }
            let (packet, ours) = self.commands.pop_front().unwrap();
            if ours {
                *self.ours.entry(le16(&packet, 0)).or_insert(0) += 1;
            }
            self.credits -= 1;
            self.last_sent = now;
            self.stats.commands_to_adapter += 1;
            self.adapter.send_command(&packet);
        }
        if !self.failed {
            if let Some(why) = self.adapter.failed() {
                self.failed = true;
                self.log.push(format!("the adapter stopped: {why}"));
            }
        }
    }

    /// Our own command's reply: not for the game.
    fn ours(&mut self, opcode: u16) -> bool {
        match self.ours.get_mut(&opcode) {
            Some(n) if *n > 0 => {
                *n -= 1;
                true
            }
            _ => false,
        }
    }

    fn adapter_event(&mut self, mut e: Vec<u8>) {
        self.stats.events_from_adapter += 1;
        let code = e[0];
        let opcode = match code {
            EV_COMMAND_COMPLETE if e.len() >= 6 => le16(&e, 3),
            EV_COMMAND_STATUS if e.len() >= 6 => le16(&e, 4),
            _ => 0,
        };
        // Not Number Of Completed Packets (one per ACL packet).
        let mut log = code != 0x13 && self.should_log(0x10000 | u32::from(code) << 16 | u32::from(opcode));
        let mut note = String::new();
        match code {
            EV_COMMAND_COMPLETE | EV_COMMAND_STATUS if e.len() >= 6 => {
                self.credits = if code == EV_COMMAND_COMPLETE { e[2] } else { e[3] };
                let status = if code == EV_COMMAND_COMPLETE { e[5] } else { e[2] };
                note = format!(" for {opcode:#06x}, status {status}");
                if self.ours(opcode) {
                    if log {
                        self.log.push(format!("adapter: HCI event {code:#04x}{note} (ours, not passed on)"));
                    }
                    return;
                }
                if code == EV_COMMAND_COMPLETE && opcode == OP_READ_BUFFER_SIZE && !self.wii_module && e.len() >= 13 {
                    // The game's stack is built around the Wii module's buffers.
                    let (size, packets) = (le16(&e, 6), le16(&e, 9));
                    let used = if e[5] != 0 || packets == 0 { WII_ACL_COUNT } else { packets.min(WII_ACL_COUNT) };
                    e[5] = 0;
                    e[6..8].copy_from_slice(&WII_ACL_SIZE.to_le_bytes());
                    e[8] = WII_SCO_SIZE;
                    e[9..11].copy_from_slice(&used.to_le_bytes());
                    e[11..13].copy_from_slice(&[0, 0]);
                    self.log.push(format!("adapter: {size}-byte ACL buffers x{packets}, shown to the game as the Wii module's {WII_ACL_SIZE} x{used}"));
                }
            }
            EV_CONNECTION_COMPLETE if e.len() >= 13 => {
                log = true;
                let handle = le16(&e, 3) & 0x0FFF;
                note = format!(": status {}, {}, handle {handle}", e[2], format_address(e[5..11].try_into().unwrap()));
                if e[2] == 0 && e[11] == 1 && !self.wii_module {
                    // A remote connected: guaranteed service for its reports.
                    let mut qos = OP_QOS_SETUP.to_le_bytes().to_vec();
                    qos.push(20);
                    qos.extend(handle.to_le_bytes());
                    qos.extend([0, 2]); // flags; service type: guaranteed
                    qos.extend(u32::MAX.to_le_bytes()); // token rate
                    qos.extend(u32::MAX.to_le_bytes()); // peak bandwidth
                    qos.extend(10_000u32.to_le_bytes()); // latency, us
                    qos.extend(u32::MAX.to_le_bytes()); // delay variation
                    self.commands.push_back((qos, true));
                }
            }
            EV_DISCONNECTION_COMPLETE if e.len() >= 6 => {
                log = true;
                note = format!(": handle {}, reason {}", le16(&e, 3) & 0x0FFF, e[5]);
            }
            EV_LINK_KEY_NOTIFICATION if e.len() >= 25 => {
                log = true;
                let address: Address = e[2..8].try_into().unwrap();
                note = format!(": {} paired", format_address(address));
                if !self.wii_module {
                    self.link_keys.insert(address, e[8..24].try_into().unwrap());
                    self.keys_changed = true;
                }
            }
            _ => {}
        }
        if log {
            self.log.push(format!("adapter: HCI event {code:#04x}{note}"));
        }
        self.push_event(e, false);
    }

    pub fn next_event(&mut self) -> Option<Vec<u8>> {
        self.events.pop_front()
    }

    pub fn has_event(&self) -> bool {
        !self.events.is_empty()
    }

    pub fn next_acl(&mut self) -> Option<Vec<u8>> {
        self.acl_in.pop_front()
    }

    pub fn has_acl(&self) -> bool {
        !self.acl_in.is_empty()
    }

    /// The game paired or forgot a remote since the last call.
    pub fn take_keys_changed(&mut self) -> bool {
        std::mem::take(&mut self.keys_changed)
    }

    /// Lines for the console since the last call.
    pub fn take_log(&mut self) -> Vec<String> {
        std::mem::take(&mut self.log)
    }

    /// The game is done with the adapter: an HCI Reset drops the remotes'
    /// connections, so none is left hanging on to it.
    pub fn shutdown(&mut self) {
        self.adapter.send_command(&RESET);
    }
}

/// `11:22:33:44:55:66` for an HCI-order (least significant first) address.
pub fn format_address(a: Address) -> String {
    format!("{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}", a[5], a[4], a[3], a[2], a[1], a[0])
}

/// The pairings file: a comment, then one line a remote: its address and
/// the link key in HCI byte order.
pub fn format_link_keys(keys: &BTreeMap<Address, LinkKey>) -> String {
    let mut text = String::from(
        "# Wii Remote pairings for Bluetooth passthrough: address, then the link key in HCI byte order.\n\
         # Delete this file (or hold the pairing key for 10 seconds in the game) to forget them all.\n",
    );
    for (a, k) in keys {
        text.push_str(&format_address(*a));
        text.push(' ');
        for b in k {
            text.push_str(&format!("{b:02x}"));
        }
        text.push('\n');
    }
    text
}

pub fn parse_link_keys(text: &str) -> BTreeMap<Address, LinkKey> {
    let hex = |s: &str| (0..s.len() / 2).map(|i| u8::from_str_radix(s.get(2 * i..2 * i + 2)?, 16).ok()).collect::<Option<Vec<u8>>>();
    let mut keys = BTreeMap::new();
    for line in text.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let mut fields = line.split_whitespace();
        let (Some(address), Some(key)) = (fields.next(), fields.next()) else { continue };
        let address: Option<Vec<u8>> = address.split(':').map(|b| if b.len() == 2 { u8::from_str_radix(b, 16).ok() } else { None }).collect();
        match (address, hex(key)) {
            (Some(mut a), Some(k)) if a.len() == 6 && k.len() == 16 && key.len() == 32 => {
                a.reverse();
                keys.insert(a.try_into().unwrap(), k.try_into().unwrap());
            }
            _ => {}
        }
    }
    keys
}

/// An adapter that answers every command itself (`NSMBW_BT_FAKE_ADAPTER=1`
/// and the tests): Command Complete with success, and nothing ever
/// connects. It runs the game's Bluetooth stack through the passthrough
/// without hardware.
#[derive(Default)]
pub struct FakeAdapter {
    events: VecDeque<Vec<u8>>,
    pub commands: Vec<u16>,
    pub acl: usize,
}

impl Adapter for FakeAdapter {
    fn send_command(&mut self, packet: &[u8]) {
        let opcode = le16(packet, 0);
        let p = packet.get(3..).unwrap_or(&[]);
        self.commands.push(opcode);
        // Inquiry and Create Connection: a status, then nobody is found.
        if opcode == 0x0401 || opcode == 0x0405 {
            let [lo, hi] = opcode.to_le_bytes();
            self.events.push_back(vec![EV_COMMAND_STATUS, 4, 0, 1, lo, hi]);
            let done = if opcode == 0x0401 {
                vec![0x01, 1, 0]
            } else {
                let mut e = vec![EV_CONNECTION_COMPLETE, 11, 0x04, 0, 0];
                e.extend(p.get(..6).unwrap_or(&[0; 6]));
                e.extend([1, 0]);
                e
            };
            self.events.push_back(done);
            return;
        }
        let two = [p.first().copied().unwrap_or(0), p.get(1).copied().unwrap_or(0)];
        let mut ret = vec![0];
        match opcode {
            // Version (Broadcom), features, address, buffers (1021 x 8).
            0x1001 => ret.extend([3, 0xA7, 0x40, 3, 0x0F, 0x00, 0x0E, 0x43]),
            0x1003 => ret.extend([0xFF, 0xFF, 0x8D, 0xFE, 0x9B, 0xF9, 0x00, 0x80]),
            0x1009 => ret.extend([0x66, 0x55, 0x44, 0x33, 0x22, 0x11]),
            OP_READ_BUFFER_SIZE => ret.extend([0xFD, 0x03, 0x40, 8, 0, 0, 0]),
            0x0C14 => ret.extend([0; 248]),
            0x0C17 | 0x0C25 => ret.extend([0, 0]),
            0x0C19 | 0x0C1F => ret.push(0),
            0x0C23 => ret.extend([0, 0, 0]),
            0x080C => ret.extend([two[0], two[1], 0x0F, 0]),
            0x080D | 0x0C37 | 0x0C08 => ret.extend(two),
            _ => {}
        }
        let mut e = vec![EV_COMMAND_COMPLETE, 3 + ret.len() as u8, 1];
        e.extend(opcode.to_le_bytes());
        e.extend(ret);
        // Two events in one transfer at times, as adapters do.
        match self.events.back_mut() {
            Some(last) if self.commands.len() % 3 == 0 => last.extend(e),
            _ => self.events.push_back(e),
        }
    }

    fn send_acl(&mut self, _: &[u8]) {
        self.acl += 1;
    }

    fn recv_events(&mut self) -> Option<Vec<u8>> {
        self.events.pop_front()
    }

    fn recv_acl(&mut self) -> Option<Vec<u8>> {
        None
    }

    fn failed(&self) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    /// A stand-in adapter the test feeds and reads.
    #[derive(Clone, Default)]
    struct Wire(Arc<Mutex<(Vec<Vec<u8>>, VecDeque<Vec<u8>>, VecDeque<Vec<u8>>)>>);

    impl Adapter for Wire {
        fn send_command(&mut self, p: &[u8]) {
            self.0.lock().unwrap().0.push(p.to_vec());
        }
        fn send_acl(&mut self, _: &[u8]) {}
        fn recv_events(&mut self) -> Option<Vec<u8>> {
            self.0.lock().unwrap().1.pop_front()
        }
        fn recv_acl(&mut self) -> Option<Vec<u8>> {
            self.0.lock().unwrap().2.pop_front()
        }
        fn failed(&self) -> Option<String> {
            None
        }
    }

    fn cmd(op: u16, p: &[u8]) -> Vec<u8> {
        let mut c = op.to_le_bytes().to_vec();
        c.push(p.len() as u8);
        c.extend_from_slice(p);
        c
    }

    fn complete(op: u16, credits: u8, ret: &[u8]) -> Vec<u8> {
        let mut e = vec![EV_COMMAND_COMPLETE, 3 + ret.len() as u8, credits];
        e.extend(op.to_le_bytes());
        e.extend_from_slice(ret);
        e
    }

    #[test]
    fn commands_wait_for_credits() {
        let wire = Wire::default();
        let mut p = Passthrough::new(Box::new(wire.clone()), false, BTreeMap::new());
        let t = Instant::now();
        p.command(&cmd(OP_RESET, &[]));
        p.command(&cmd(0x1001, &[]));
        p.pump_at(t);
        assert_eq!(wire.0.lock().unwrap().0.len(), 1, "one credit to start");
        // Its reply, split across two transfers, gives a credit back.
        let reply = complete(OP_RESET, 1, &[0]);
        wire.0.lock().unwrap().1.extend([reply[..2].to_vec(), reply[2..].to_vec()]);
        p.pump_at(t);
        assert_eq!(wire.0.lock().unwrap().0.len(), 2);
        assert_eq!(p.next_event(), Some(reply));
        // No reply: the next goes after a second.
        p.command(&cmd(0x1009, &[]));
        p.pump_at(t + Duration::from_millis(500));
        assert_eq!(wire.0.lock().unwrap().0.len(), 2);
        p.pump_at(t + Duration::from_millis(1100));
        assert_eq!(wire.0.lock().unwrap().0.len(), 3);
        assert_eq!(p.stats.command_timeouts, 1);
    }

    #[test]
    fn pairings_are_kept_here() {
        let wire = Wire::default();
        let mut p = Passthrough::new(Box::new(wire.clone()), false, BTreeMap::new());
        let remote: Address = [6, 5, 4, 3, 2, 1];
        // A new pairing from the adapter.
        let mut n = vec![EV_LINK_KEY_NOTIFICATION, 23];
        n.extend(remote);
        n.extend([0xAB; 16]);
        n.push(0);
        wire.0.lock().unwrap().1.push_back(n.clone());
        p.pump();
        assert_eq!(p.next_event(), Some(n));
        assert!(p.take_keys_changed());
        assert_eq!(p.link_keys.get(&remote), Some(&[0xAB; 16]));
        // The game reads them all back, answered here.
        p.command(&cmd(OP_READ_STORED_LINK_KEY, &[0, 0, 0, 0, 0, 0, 1]));
        let keys = p.next_event().unwrap();
        assert_eq!(&keys[..3], &[EV_RETURN_LINK_KEYS, 23, 1]);
        assert_eq!(&keys[3..9], &remote);
        assert_eq!(p.next_event(), Some(complete(OP_READ_STORED_LINK_KEY, 1, &[0, 255, 0, 1, 0])));
        // The firmware patch never reaches the adapter.
        p.command(&cmd(OP_BROADCOM_WRITE_RAM, &[1, 2, 3]));
        assert_eq!(p.next_event(), Some(complete(OP_BROADCOM_WRITE_RAM, 1, &[0])));
        // Delete all.
        p.command(&cmd(OP_DELETE_STORED_LINK_KEY, &[0, 0, 0, 0, 0, 0, 1]));
        assert_eq!(p.next_event(), Some(complete(OP_DELETE_STORED_LINK_KEY, 1, &[0, 1, 0])));
        assert!(p.link_keys.is_empty());
        p.pump();
        assert!(wire.0.lock().unwrap().0.is_empty(), "all answered here");
        // A Wii's own module keeps its own.
        let mut wii = Passthrough::new(Box::new(Wire::default()), true, BTreeMap::new());
        wii.command(&cmd(OP_READ_STORED_LINK_KEY, &[0, 0, 0, 0, 0, 0, 1]));
        assert!(!wii.has_event());
    }

    #[test]
    fn buffers_look_like_the_wii_modules_and_remotes_get_qos() {
        let wire = Wire::default();
        let mut p = Passthrough::new(Box::new(wire.clone()), false, BTreeMap::new());
        p.command(&cmd(OP_READ_BUFFER_SIZE, &[]));
        p.pump();
        wire.0.lock().unwrap().1.push_back(complete(OP_READ_BUFFER_SIZE, 1, &[0, 0xFD, 0x03, 0x40, 8, 0, 0, 0]));
        p.pump();
        assert_eq!(p.next_event(), Some(complete(OP_READ_BUFFER_SIZE, 1, &[0, 0x53, 0x01, 64, 8, 0, 0, 0])));
        // A remote connects: a QoS Setup follows, and its reply stays here.
        let mut c = vec![EV_CONNECTION_COMPLETE, 11, 0, 0x40, 0];
        c.extend([6, 5, 4, 3, 2, 1, 1, 0]);
        wire.0.lock().unwrap().1.push_back(c.clone());
        p.pump();
        assert_eq!(p.next_event(), Some(c));
        let sent = wire.0.lock().unwrap().0.last().unwrap().clone();
        assert_eq!((le16(&sent, 0), sent[2], le16(&sent, 3), sent[6]), (OP_QOS_SETUP, 20, 0x40, 2));
        wire.0.lock().unwrap().1.push_back(vec![EV_COMMAND_STATUS, 4, 0, 1, 0x07, 0x08]);
        p.pump();
        assert!(!p.has_event(), "our QoS Setup's status is not passed on");
    }

    #[test]
    fn sync_button_goes_first() {
        let mut p = Passthrough::new(Box::<FakeAdapter>::default(), false, BTreeMap::new());
        p.command(&cmd(OP_BROADCOM_LAUNCH_RAM, &[]));
        p.sync_button(false);
        p.sync_button(true);
        assert_eq!(p.next_event(), Some(vec![EV_VENDOR, 1, SYNC_HELD]));
        assert_eq!(p.next_event(), Some(vec![EV_VENDOR, 1, SYNC_PRESSED]));
        assert_eq!(p.next_event(), Some(complete(OP_BROADCOM_LAUNCH_RAM, 1, &[0])));
    }

    #[test]
    fn acl_is_split_into_packets() {
        let mut a = Assembler { acl: true, ..Assembler::default() };
        let two = [0x40, 0x20, 2, 0, 0xA1, 0x30, 0x40, 0x20, 1, 0, 0xA1];
        let (packets, dropped) = a.feed(&two[..7]);
        assert_eq!((packets, dropped), (vec![two[..6].to_vec()], 0));
        assert_eq!(a.feed(&two[7..]).0, vec![two[6..].to_vec()]);
        // Too long to be a packet: dropped.
        assert_eq!(a.feed(&[0x40, 0x20, 0xFF, 0xFF, 1]), (vec![], 5));
    }

    #[test]
    fn link_key_file_round_trips() {
        let mut keys = BTreeMap::new();
        keys.insert([0x66, 0x55, 0x44, 0x33, 0x22, 0x11], [0x0F; 16]);
        keys.insert([1, 2, 3, 4, 5, 6], std::array::from_fn(|i| i as u8));
        let text = format_link_keys(&keys);
        assert!(text.contains("11:22:33:44:55:66 0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f\n"), "{text}");
        assert_eq!(parse_link_keys(&text), keys);
        assert!(parse_link_keys("zz:22:33:44:55:66 00\nnot a line\n").is_empty());
    }
}
