//! Online multiplayer (docs/PORT_PLAN.md, section 12): every machine runs
//! the whole game, in lockstep on the players' Wii Remote inputs.
//!
//! - The host plays as remote 1 and listens. A joiner connects at any time;
//!   at the start of the host's next field the host saves its machine and
//!   sends it, with its NAND files, and the joiner carries on from there as
//!   the next free remote. Nobody restarts.
//! - Each field, a machine sends its local input for field `n + DELAY` and
//!   runs field `n` once every active remote's input for it is in. The host
//!   relays joiners' inputs to each other.
//! - Every [`HASH_EVERY`] fields each machine sends a hash of guest RAM; a
//!   mismatch stops the session.
//! - The game's own rules apply, but for Exit Course in every course
//!   ([`Session::exit_course`]): on while every player has it on. A joiner
//!   without it turns it off for everyone from the field it joins at.
//!
//! Messages are framed as a kind byte, a little-endian u32 length and the
//! body, over TCP with Nagle off.

use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::{Duration, Instant};

use guest_mem::snap::{Snap, Snapshot};
use hle_ios::bt::REMOTES;
use hle_ios::wiimote::Input;
use hle_os::System;

/// Bumped when messages or the save-state layout change.
const PROTOCOL: u32 = 2;
/// Fields between sampling a local input and the field it is used in, once
/// another machine is in the session (33 ms; a solo host has none).
pub const DELAY: u64 = 2;
/// Fields between RAM hash checks.
pub const HASH_EVERY: u64 = 120;
/// A peer silent this long is taken as gone.
const TIMEOUT: Duration = Duration::from_secs(20);

const HELLO: u8 = 1;
const WELCOME: u8 = 2;
const INPUT: u8 = 3;
const HASH: u8 = 4;
const LEAVE: u8 = 5;

/// What this machine is in a session.
#[derive(Clone, Debug, Default)]
pub enum Mode {
    #[default]
    Off,
    /// Listen on `port`. With `wait_join`, block at that field until a
    /// joiner connects (for scripted tests).
    Host { port: u16, wait_join: Option<u64> },
    Join { addr: String },
}

fn send(stream: &mut TcpStream, kind: u8, body: &[u8]) -> std::io::Result<()> {
    let mut head = [0u8; 5];
    head[0] = kind;
    head[1..].copy_from_slice(&(body.len() as u32).to_le_bytes());
    stream.write_all(&head)?;
    stream.write_all(body)
}

fn recv(stream: &mut TcpStream) -> std::io::Result<(u8, Vec<u8>)> {
    let mut head = [0u8; 5];
    stream.read_exact(&mut head)?;
    let len = u32::from_le_bytes(head[1..].try_into().unwrap()) as usize;
    if len > 1 << 30 {
        return Err(std::io::Error::other("message too long"));
    }
    let mut body = vec![0; len];
    stream.read_exact(&mut body)?;
    Ok((head[0], body))
}

/// Encodes with the save-state walk.
fn encode(f: impl FnOnce(&mut Snap)) -> Vec<u8> {
    let mut s = Snap::saver();
    f(&mut s);
    s.into_bytes()
}

fn decode(bytes: Vec<u8>, f: impl FnOnce(&mut Snap)) -> Result<(), String> {
    let mut s = Snap::loader(bytes);
    f(&mut s);
    s.finish()
}

/// The game's identity and the settings both machines must share. A
/// joiner's hello is this, then a byte: 1 if it has Exit Course on.
fn hello_body(game: &Path, light_textures: bool) -> Vec<u8> {
    let mut id = std::fs::read(game.join("sys/boot.bin")).map(|b| b[..8.min(b.len())].to_vec()).unwrap_or_default();
    let (mut proto, mut lt) = (PROTOCOL, light_textures);
    encode(|s| guest_mem::snap!(s; proto, id, lt))
}

/// Every file under `root`, with paths relative to it ('/'-separated).
fn read_tree(root: &Path) -> Vec<(String, Vec<u8>)> {
    fn walk(root: &Path, dir: &Path, out: &mut Vec<(String, Vec<u8>)>) {
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        let mut entries: Vec<_> = entries.flatten().collect();
        entries.sort_by_key(|e| e.path());
        for e in entries {
            let p = e.path();
            if p.is_dir() {
                walk(root, &p, out);
            } else if let (Ok(data), Ok(rel)) = (std::fs::read(&p), p.strip_prefix(root)) {
                out.push((rel.to_string_lossy().replace('\\', "/"), data));
            }
        }
    }
    let mut out = Vec::new();
    walk(root, root, &mut out);
    out
}

/// Replaces `root` with `files`.
fn write_tree(root: &Path, files: &[(String, Vec<u8>)]) -> Result<(), String> {
    let _ = std::fs::remove_dir_all(root);
    for (rel, data) in files {
        if rel.split('/').any(|c| c == ".." || c.is_empty()) {
            return Err(format!("bad NAND path from host: {rel}"));
        }
        let p = root.join(rel);
        if let Some(d) = p.parent() {
            std::fs::create_dir_all(d).map_err(|e| format!("{}: {e}", d.display()))?;
        }
        std::fs::write(&p, data).map_err(|e| format!("{}: {e}", p.display()))?;
    }
    std::fs::create_dir_all(root).map_err(|e| format!("{}: {e}", root.display()))
}

/// What a joiner receives.
#[derive(Default)]
struct Welcome {
    slot: usize,
    field: u64,
    active_from: [Option<u64>; REMOTES],
    active_until: [Option<u64>; REMOTES],
    /// (field, slot, input) known to the host for fields from `field` on.
    inputs: Vec<(u64, usize, Input)>,
    nand: Vec<(String, Vec<u8>)>,
    state: Vec<u8>,
    /// The session's Exit Course from `field` on.
    exit_course: bool,
}

impl Snapshot for Welcome {
    fn snap(&mut self, s: &mut Snap) {
        guest_mem::snap!(s; self.slot, self.field, self.active_from, self.active_until, self.inputs, self.nand, self.state, self.exit_course);
    }
}

/// A peer's messages, read on their own thread.
struct Peer {
    stream: TcpStream,
    rx: Receiver<std::io::Result<(u8, Vec<u8>)>>,
    /// The remote this peer plays (on a joiner, the host's peer is slot 0).
    slot: usize,
}

impl Peer {
    fn new(stream: TcpStream, slot: usize) -> std::io::Result<Self> {
        stream.set_nodelay(true)?;
        let mut reader = stream.try_clone()?;
        let (tx, rx) = mpsc::channel();
        std::thread::Builder::new()
            .name("net-peer".into())
            .spawn(move || loop {
                let m = recv(&mut reader);
                let end = m.is_err();
                if tx.send(m).is_err() || end {
                    return;
                }
            })
            .map_err(std::io::Error::other)?;
        Ok(Self { stream, rx, slot })
    }
}

pub struct Session {
    /// The remote this machine plays.
    pub slot: usize,
    host: bool,
    peers: Vec<Peer>,
    /// Joiners that said hello, waiting for the next field boundary (host),
    /// and whether each has Exit Course on.
    arrivals: Option<Receiver<(TcpStream, bool)>>,
    /// Exit Course in every course: on while every player has it on.
    pub exit_course: bool,
    wait_join: Option<u64>,
    /// Inputs by field and remote.
    inputs: BTreeMap<u64, [Option<Input>; REMOTES]>,
    /// The field from which each remote's input comes from the network,
    /// and the field from which it no longer does (it left).
    active_from: [Option<u64>; REMOTES],
    active_until: [Option<u64>; REMOTES],
    /// The last local input, repeated over the delay when the first peer joins.
    last_local: Input,
    /// Hashes by field: ours and the peers'.
    ours: BTreeMap<u64, u64>,
    theirs: BTreeMap<u64, Vec<u64>>,
    /// The NAND this machine's game uses.
    nand: PathBuf,
}

/// Where a joiner keeps the host's NAND files.
pub fn joiner_nand() -> PathBuf {
    PathBuf::from("saves/net-join/nand")
}

impl Session {
    /// Starts listening; the game boots normally. `exit_course`: the host's
    /// own setting.
    pub fn host(port: u16, wait_join: Option<u64>, game: &Path, light_textures: bool, nand: PathBuf, exit_course: bool) -> Result<Self, String> {
        let listener = TcpListener::bind(("0.0.0.0", port)).map_err(|e| format!("listen on port {port}: {e}"))?;
        println!("net: hosting on port {port}");
        let (tx, rx) = mpsc::channel();
        let ours = hello_body(game, light_textures);
        std::thread::Builder::new()
            .name("net-listen".into())
            .spawn(move || accept_loop(listener, ours, tx))
            .map_err(|e| e.to_string())?;
        let mut active_from = [None; REMOTES];
        active_from[0] = Some(0);
        Ok(Self { slot: 0, host: true, peers: Vec::new(), arrivals: Some(rx), exit_course, wait_join, inputs: BTreeMap::new(), active_from, active_until: [None; REMOTES], last_local: Input::default(), ours: BTreeMap::new(), theirs: BTreeMap::new(), nand })
    }

    /// Connects to a host and receives its machine. Returns the session,
    /// the field to carry on from and the state to load; the host's NAND
    /// files are written to [`joiner_nand`] first. `exit_course`: this
    /// player's own setting (the session's comes back from the host).
    pub fn join(addr: &str, game: &Path, light_textures: bool, exit_course: bool) -> Result<(Self, u64, Vec<u8>), String> {
        let mut stream = TcpStream::connect(addr).map_err(|e| format!("connect to {addr}: {e}"))?;
        stream.set_nodelay(true).map_err(|e| e.to_string())?;
        let mut hello = hello_body(game, light_textures);
        hello.push(u8::from(exit_course));
        send(&mut stream, HELLO, &hello).map_err(|e| e.to_string())?;
        println!("net: connected to {addr}, waiting for the host's game");
        let started = Instant::now();
        let (kind, body) = recv(&mut stream).map_err(|e| format!("{addr}: {e}"))?;
        if kind != WELCOME {
            return Err(format!("{addr}: refused: {}", String::from_utf8_lossy(&body)));
        }
        let raw = miniz_oxide::inflate::decompress_to_vec(&body).map_err(|e| format!("{addr}: bad welcome ({e:?})"))?;
        let mut w = Welcome::default();
        decode(raw, |s| w.snap(s)).map_err(|e| format!("{addr}: {e}"))?;
        let nand = joiner_nand();
        write_tree(&nand, &w.nand)?;
        println!("net: joined as player {} at field {} ({} KB in {:.1} s)", w.slot + 1, w.field, body.len() >> 10, started.elapsed().as_secs_f32());
        let mut inputs: BTreeMap<u64, [Option<Input>; REMOTES]> = BTreeMap::new();
        for (f, slot, i) in &w.inputs {
            inputs.entry(*f).or_default()[*slot] = Some(*i);
        }
        let peer = Peer::new(stream, 0).map_err(|e| e.to_string())?;
        if exit_course && !w.exit_course {
            println!("net: Exit Course in every course is off in this game: not every player has it on");
        }
        let s = Self { slot: w.slot, host: false, peers: vec![peer], arrivals: None, exit_course: w.exit_course, wait_join: None, inputs, active_from: w.active_from, active_until: w.active_until, last_local: Input::default(), ours: BTreeMap::new(), theirs: BTreeMap::new(), nand };
        Ok((s, w.field, std::mem::take(&mut w.state)))
    }

    fn delay(&self) -> u64 {
        if self.peers.is_empty() {
            0
        } else {
            DELAY
        }
    }

    /// Host: lets in anyone waiting, at the start of field `index`. With
    /// `wait_join`, joiners wait for that field, which waits for one.
    fn admit(&mut self, sys: &mut System, index: u64, stop: &dyn Fn() -> bool) -> Result<(), String> {
        let Some(arrivals) = &self.arrivals else { return Ok(()) };
        if self.wait_join.is_some_and(|w| index < w) {
            return Ok(());
        }
        let mut waiting: Vec<(TcpStream, bool)> = arrivals.try_iter().collect();
        if waiting.is_empty() && self.wait_join == Some(index) {
            println!("net: field {index}: waiting for a player to join");
            while waiting.is_empty() && !stop() {
                waiting.extend(arrivals.recv_timeout(Duration::from_millis(50)).ok());
            }
        }
        for (mut stream, exit_course) in waiting {
            let Some(slot) = (1..REMOTES).find(|&s| self.active_from[s].is_none()) else {
                let _ = send(&mut stream, 0, b"the game is full");
                continue;
            };
            // From this field on, on every machine: the state below does
            // not hold it, so the welcome does.
            if self.exit_course && !exit_course {
                println!("net: player {} does not have Exit Course in every course on: it is off in this game from field {index}", slot + 1);
            }
            self.exit_course &= exit_course;
            sys.menu.exit_course = self.exit_course;
            // Our own input over the delay, if we were alone until now.
            if self.peers.is_empty() {
                for f in index..index + DELAY {
                    self.inputs.entry(f).or_default()[self.slot].get_or_insert(self.last_local);
                }
            }
            let joins_at = index + DELAY;
            self.active_from[slot] = Some(joins_at);
            self.active_until[slot] = None;
            let started = Instant::now();
            let state = sys.save_state();
            let inputs: Vec<(u64, usize, Input)> = self.inputs.range(index..).flat_map(|(&f, a)| a.iter().enumerate().filter_map(move |(s, i)| i.map(|i| (f, s, i)))).collect();
            let mut w = Welcome { slot, field: index, active_from: self.active_from, active_until: self.active_until, inputs, nand: read_tree(&self.nand), state, exit_course: self.exit_course };
            let body = miniz_oxide::deflate::compress_to_vec(&encode(|s| w.snap(s)), 1);
            send(&mut stream, WELCOME, &body).map_err(|e| format!("net: sending the game: {e}"))?;
            println!("net: player {} joined at field {index}, playing from field {joins_at} ({} KB, {:.1} s)", slot + 1, body.len() >> 10, started.elapsed().as_secs_f32());
            self.peers.push(Peer::new(stream, slot).map_err(|e| e.to_string())?);
        }
        Ok(())
    }

    /// Host: peer `p` disconnected. Its remote switches off after the last
    /// field it sent input for (never before `index`), on every machine.
    fn drop_peer(&mut self, p: usize, index: u64) {
        let slot = self.peers.remove(p).slot;
        let last = self.inputs.iter().rev().find(|(_, a)| a[slot].is_some()).map(|(&f, _)| f);
        let mut at = last.map_or(index, |f| f + 1).max(index);
        self.active_until[slot] = Some(at);
        println!("net: player {} left; their remote switches off at field {at}", slot + 1);
        let mut gone = slot;
        let body = encode(|s| guest_mem::snap!(s; gone, at));
        for peer in &mut self.peers {
            let _ = send(&mut peer.stream, LEAVE, &body);
        }
    }

    /// Whether remote `s` plays field `n` with input from the network.
    fn active(&self, s: usize, n: u64) -> bool {
        self.active_from[s].is_some_and(|a| a <= n) && self.active_until[s].is_none_or(|u| n < u)
    }

    /// Handles one message from peer `p`.
    fn handle(&mut self, p: usize, kind: u8, body: Vec<u8>) -> Result<(), String> {
        match kind {
            INPUT => {
                let (mut f, mut slot, mut i) = (0u64, 0usize, Input::default());
                let raw = body.clone();
                decode(body, |s| guest_mem::snap!(s; f, slot, i))?;
                if slot >= REMOTES {
                    return Err("net: input for a remote that does not exist".into());
                }
                self.inputs.entry(f).or_default()[slot] = Some(i);
                if self.host {
                    for (q, peer) in self.peers.iter_mut().enumerate() {
                        if q != p {
                            let _ = send(&mut peer.stream, INPUT, &raw);
                        }
                    }
                }
            }
            LEAVE if !self.host => {
                let (mut slot, mut at) = (0usize, 0u64);
                decode(body, |s| guest_mem::snap!(s; slot, at))?;
                if slot < REMOTES {
                    self.active_until[slot] = Some(at);
                    println!("net: player {} left; their remote switches off at field {at}", slot + 1);
                }
            }
            HASH => {
                let (mut f, mut h) = (0u64, 0u64);
                decode(body, |s| guest_mem::snap!(s; f, h))?;
                self.theirs.entry(f).or_default().push(h);
                self.check_hashes()?;
            }
            _ => return Err(format!("net: unexpected message {kind} from player {}", self.peers[p].slot + 1)),
        }
        Ok(())
    }

    fn check_hashes(&mut self) -> Result<(), String> {
        for (f, hs) in &self.theirs {
            if let Some(ours) = self.ours.get(f) {
                if hs.iter().any(|h| h != ours) {
                    return Err(format!("net: DESYNC at field {f}: our RAM hash {ours:016x}, a peer's {:016x?}", hs));
                }
            }
        }
        // Keep the last few, compared or not.
        while self.ours.len() > 8 {
            self.ours.pop_first();
        }
        while self.theirs.len() > 8 {
            self.theirs.pop_first();
        }
        Ok(())
    }

    /// Receives whatever has arrived, waiting up to `wait` for something.
    /// Returns the peers whose connection ended.
    fn pump(&mut self, wait: Duration) -> Result<Vec<usize>, String> {
        let deadline = Instant::now() + wait;
        let mut gone = Vec::new();
        loop {
            let mut got = false;
            for p in 0..self.peers.len() {
                if gone.contains(&p) {
                    continue;
                }
                loop {
                    match self.peers[p].rx.try_recv() {
                        Ok(Ok((kind, body))) => {
                            self.handle(p, kind, body)?;
                            got = true;
                        }
                        Ok(Err(_)) | Err(mpsc::TryRecvError::Disconnected) => {
                            gone.push(p);
                            break;
                        }
                        Err(mpsc::TryRecvError::Empty) => break,
                    }
                }
            }
            if got || !gone.is_empty() || Instant::now() >= deadline || self.peers.is_empty() {
                return Ok(gone);
            }
            std::thread::sleep(Duration::from_micros(500));
        }
    }

    /// Host: drops peers that left. A joiner cannot go on without the host.
    fn handle_gone(&mut self, mut gone: Vec<usize>, index: u64) -> Result<(), String> {
        if gone.is_empty() {
            return Ok(());
        }
        if !self.host {
            return Err("net: the host left".into());
        }
        gone.sort_unstable();
        for p in gone.into_iter().rev() {
            self.drop_peer(p, index);
        }
        Ok(())
    }

    /// The remotes' inputs for field `index`: admits joiners (host), sends
    /// `local` for its field, and waits for everyone's input for this one.
    pub fn inputs(&mut self, sys: &mut System, index: u64, local: Input, stop: &dyn Fn() -> bool) -> Result<[Input; REMOTES], String> {
        if self.host {
            self.admit(sys, index, stop)?;
        }
        let mut local = local;
        local.connected = true;
        let target = index + self.delay();
        let mine = &mut self.inputs.entry(target).or_default()[self.slot];
        if mine.is_none() {
            *mine = Some(local);
            self.last_local = local;
            let (mut f, mut slot, mut i) = (target, self.slot, local);
            let body = encode(|s| guest_mem::snap!(s; f, slot, i));
            for peer in &mut self.peers {
                let _ = send(&mut peer.stream, INPUT, &body);
            }
        }
        let started = Instant::now();
        loop {
            let gone = self.pump(Duration::ZERO)?;
            self.handle_gone(gone, index)?;
            let row = self.inputs.get(&index).copied().unwrap_or_default();
            let Some(s) = (0..REMOTES).find(|&s| self.active(s, index) && row[s].is_none()) else { break };
            if stop() {
                return Err("stopped".into());
            }
            if started.elapsed() > TIMEOUT {
                return Err(format!("net: no input from player {} for field {index} in {} s", s + 1, TIMEOUT.as_secs()));
            }
            let gone = self.pump(Duration::from_millis(5))?;
            self.handle_gone(gone, index)?;
        }
        let row = self.inputs.remove(&index).unwrap_or_default();
        while self.inputs.first_key_value().is_some_and(|(&f, _)| f < index) {
            self.inputs.pop_first();
        }
        let out = std::array::from_fn(|s| match row[s] {
            Some(i) if self.active(s, index) => i,
            _ => Input::default(),
        });
        // A remote that left frees its slot once its last field has run.
        for s in 0..REMOTES {
            if self.active_until[s].is_some_and(|u| index + 1 >= u) {
                (self.active_from[s], self.active_until[s]) = (None, None);
            }
        }
        Ok(out)
    }

    /// Remotes that start playing at field `index` after the session began:
    /// their players are brought into the course (`System::joins`).
    pub fn joining_at(&self, index: u64) -> Vec<usize> {
        (0..REMOTES).filter(|&s| self.active_from[s] == Some(index) && index > 0).collect()
    }

    /// After field `index` ran: every [`HASH_EVERY`] fields, sends and checks
    /// the RAM hash.
    pub fn after_field(&mut self, sys: &System, index: u64) -> Result<(), String> {
        if index % HASH_EVERY != 0 || self.peers.is_empty() {
            return Ok(());
        }
        let mut h = sys.mem.hash();
        self.ours.insert(index, h);
        let mut f = index;
        let body = encode(|s| guest_mem::snap!(s; f, h));
        for peer in &mut self.peers {
            let _ = send(&mut peer.stream, HASH, &body);
        }
        self.check_hashes()
    }
}

/// Host: accepts connections and checks each joiner's hello.
fn accept_loop(listener: TcpListener, ours: Vec<u8>, tx: Sender<(TcpStream, bool)>) {
    for stream in listener.incoming() {
        let Ok(mut stream) = stream else { continue };
        let _ = stream.set_nodelay(true);
        match recv(&mut stream) {
            Ok((HELLO, body)) if body.split_last().is_some_and(|(_, shared)| *shared == ours[..]) => {
                if tx.send((stream, body.last() == Some(&1))).is_err() {
                    return;
                }
            }
            Ok(_) => {
                println!("net: refused a joiner (different game, version or light-texture setting)");
                let _ = send(&mut stream, 0, b"different game, port version or NSMBW_LIGHTTEX setting");
            }
            Err(_) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn welcome_round_trips() {
        let mut w = Welcome { slot: 1, field: 77, active_from: [Some(0), Some(79), None, None], active_until: [None; REMOTES], inputs: vec![(78, 0, Input { connected: true, buttons: 3, shake: true, tilt: -0.5 })], nand: vec![("a/b".into(), vec![1, 2])], state: vec![9; 10], exit_course: true };
        let bytes = encode(|s| w.snap(s));
        let mut v = Welcome::default();
        decode(bytes, |s| v.snap(s)).unwrap();
        assert_eq!((v.slot, v.field, v.active_from, v.inputs.len(), v.nand.len(), v.state.len(), v.exit_course), (1, 77, w.active_from, 1, 1, 10, true));
        assert_eq!(v.inputs[0].2, w.inputs[0].2);
    }

    #[test]
    fn nand_paths_cannot_escape() {
        let dir = std::env::temp_dir().join("nsmbw-net-test");
        assert!(write_tree(&dir, &[("../x".into(), vec![])]).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
