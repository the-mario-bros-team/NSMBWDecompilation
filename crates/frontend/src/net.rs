//! Online multiplayer (docs/PORT_PLAN.md, section 12): every machine runs
//! the whole game, in lockstep on the players' Wii Remote inputs.
//!
//! - The host plays as remote 1 and listens. A joiner connects at any time;
//!   at the start of the host's next field the host saves its machine and
//!   sends it, with its NAND files, and the joiner carries on from there as
//!   the next free remote. Nobody restarts.
//! - Each field, a machine sends its local input for field `n + delay` and
//!   runs field `n` once every active remote's input for it is in. The host
//!   passes each joiner's input on to the others as it arrives (on the
//!   connection's reader thread, not at the host's next field).
//! - The delay follows the round trips: the host pings every joiner and
//!   picks the fewest fields an input needs to reach every machine
//!   ([`pick_delay`]), from 1 on a LAN; each machine changes its own at
//!   once (a longer delay sends the same input for the fields in between).
//! - Each machine measures how long before it was needed the last input of
//!   each field came in (its slack) and they compare it every
//!   [`SYNC_EVERY`] fields: one with far less than the others moves its
//!   fields a little later ([`Session::take_shift`]), so a late packet
//!   rarely makes anyone wait.
//! - Joiners already in a session are told of a new one ([`JOINED`]).
//! - Every [`HASH_EVERY`] fields each machine sends a hash of guest RAM; a
//!   mismatch stops the session.
//! - The game's own rules apply, but for Exit Course in every course
//!   ([`Session::exit_course`]): on while every player has it on. A joiner
//!   without it turns it off for everyone from the field it joins at.
//!
//! Messages are framed as a kind byte, a little-endian u32 length and the
//! body, over TCP with Nagle off.

use std::collections::{BTreeMap, VecDeque};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use guest_mem::snap::{Snap, Snapshot};
use hle_ios::bt::REMOTES;
use hle_ios::wiimote::Input;
use hle_os::System;

/// Bumped when messages or the save-state layout change.
const PROTOCOL: u32 = 3;
/// Fields between sampling a local input and the field it is used in, once
/// another machine is in the session, until a round trip has been measured
/// (33 ms; a solo host has none).
const START_DELAY: u64 = 2;
/// The longest input delay (200 ms).
const MAX_DELAY: u64 = 12;
/// One field, in ms.
const FIELD_MS: f64 = 1000.0 * 1001.0 / 60_000.0;
/// Added to an input's one-way time when picking the delay: pacing jitter
/// and the time a machine takes to send.
const DELAY_MARGIN_MS: f32 = 6.0;
/// Fields a lower delay must have been enough for before the delay drops
/// to it (2 s).
const LOWER_AFTER: u64 = 120;
/// Fields between the host's pings.
const PING_EVERY: u64 = 30;
/// Fields between slack reports ([`Session::sync`]).
const SYNC_EVERY: u64 = 60;
/// The least difference in slack worth moving a machine's fields for, and
/// the most one move takes.
const SYNC_MIN: Duration = Duration::from_millis(2);
const SYNC_MAX_STEP: Duration = Duration::from_millis(8);
/// Added to every message this machine sends (ms), to stand in for a long
/// connection when testing (`--net-latency`).
pub static LATENCY_MS: AtomicU64 = AtomicU64::new(0);
/// Fields between RAM hash checks.
pub const HASH_EVERY: u64 = 120;
/// A peer silent this long is taken as gone.
const TIMEOUT: Duration = Duration::from_secs(20);

const HELLO: u8 = 1;
const WELCOME: u8 = 2;
const INPUT: u8 = 3;
const HASH: u8 = 4;
const LEAVE: u8 = 5;
/// Host to the joiners already in: (remote, the field at whose start the
/// game is saved for it, the field it plays from, the session's Exit Course
/// from the first).
const JOINED: u8 = 6;
/// Host to joiners: the input delay.
const SET_DELAY: u8 = 7;
/// Host to a joiner and back, answered on the joiner's reader thread:
/// microseconds since the host's epoch.
const PING: u8 = 8;
const PONG: u8 = 9;
/// A machine's least slack over the last [`SYNC_EVERY`] fields (us).
const SYNC: u8 = 10;

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

/// A message as sent: kind, length and body in one buffer (one write, so
/// one packet for a small message).
fn frame(kind: u8, body: &[u8]) -> Vec<u8> {
    let mut m = Vec::with_capacity(5 + body.len());
    m.push(kind);
    m.extend_from_slice(&(body.len() as u32).to_le_bytes());
    m.extend_from_slice(body);
    m
}

fn send(stream: &mut TcpStream, kind: u8, body: &[u8]) -> std::io::Result<()> {
    stream.write_all(&frame(kind, body))
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
    /// The input delay.
    delay: u64,
    /// Joins on the way: the fields the game is saved at for them, and the
    /// session's Exit Course from each.
    saves: Vec<(u64, bool)>,
}

impl Snapshot for Welcome {
    fn snap(&mut self, s: &mut Snap) {
        guest_mem::snap!(s; self.slot, self.field, self.active_from, self.active_until, self.inputs, self.nand, self.state, self.exit_course, self.delay, self.saves);
    }
}

/// What a peer's writer thread sends.
enum Out {
    /// A whole message (kind, length and body).
    Bytes(Arc<Vec<u8>>),
    /// The welcome, encoded but not yet deflated: deflated on the writer
    /// thread, so the host's game does not wait for it.
    Welcome(Vec<u8>),
}

/// A connection to another machine: the remote it plays (on a joiner, the
/// host's is 0) and its writer thread.
struct Link {
    slot: usize,
    tx: Sender<(Instant, Out)>,
}

impl Link {
    fn send(&self, bytes: &Arc<Vec<u8>>) {
        let _ = self.tx.send((Instant::now(), Out::Bytes(bytes.clone())));
    }
}

/// Every connection. The host's reader threads pass each joiner's input
/// straight on to the others under this lock, the moment it arrives.
type Links = Arc<Mutex<Vec<Link>>>;

/// A message from a peer's reader thread: its remote, when it came in, and
/// the message (an error once the connection ends).
struct Msg {
    slot: usize,
    at: Instant,
    m: std::io::Result<(u8, Vec<u8>)>,
}

/// Writes queued messages to `stream` in order (after [`LATENCY_MS`] when
/// set, to stand in for a long connection).
fn spawn_writer(mut stream: TcpStream, slot: usize) -> std::io::Result<Sender<(Instant, Out)>> {
    let (tx, rx) = mpsc::channel::<(Instant, Out)>();
    std::thread::Builder::new().name("net-send".into()).spawn(move || {
        for (queued, out) in rx {
            let latency = Duration::from_millis(LATENCY_MS.load(Ordering::Relaxed));
            let due = queued + latency;
            let now = Instant::now();
            if due > now {
                std::thread::sleep(due - now);
            }
            let bytes = match out {
                Out::Bytes(b) => b,
                Out::Welcome(raw) => {
                    let started = Instant::now();
                    let body = miniz_oxide::deflate::compress_to_vec(&raw, 1);
                    println!("net: sending player {} the game ({} KB, deflated in {:.1} s)", slot + 1, body.len() >> 10, started.elapsed().as_secs_f32());
                    Arc::new(frame(WELCOME, &body))
                }
            };
            if stream.write_all(&bytes).is_err() {
                return;
            }
        }
    })?;
    Ok(tx)
}

/// Reads `stream`'s messages into `inbox`. On the host (`relay`), a
/// joiner's input goes on to every other joiner at once, and pings are
/// answered here on a joiner, so neither waits for the game thread.
fn spawn_reader(mut stream: TcpStream, slot: usize, links: Links, relay: bool, inbox: Sender<Msg>) -> std::io::Result<()> {
    std::thread::Builder::new().name("net-peer".into()).spawn(move || loop {
        let m = recv(&mut stream);
        let at = Instant::now();
        match &m {
            Ok((INPUT, body)) if relay => {
                let all = links.lock().unwrap();
                let bytes = Arc::new(frame(INPUT, body));
                for l in all.iter().filter(|l| l.slot != slot) {
                    l.send(&bytes);
                }
                // Posted under the lock: a joiner let in meanwhile gets this
                // input either relayed or in its welcome.
                let gone = inbox.send(Msg { slot, at, m }).is_err();
                drop(all);
                if gone {
                    return;
                }
                continue;
            }
            Ok((PING, body)) => {
                let bytes = Arc::new(frame(PONG, body));
                if let Some(l) = links.lock().unwrap().iter().find(|l| l.slot == slot) {
                    l.send(&bytes);
                }
                continue;
            }
            _ => {}
        }
        let end = m.is_err();
        if inbox.send(Msg { slot, at, m }).is_err() || end {
            return;
        }
    })?;
    Ok(())
}

/// The input delay (fields) for these round trips (ms) to the joiners: long
/// enough for an input to reach every machine, through the host between
/// two joiners, before it is needed.
fn pick_delay(round_trips: &[f32]) -> u64 {
    let mut r = round_trips.to_vec();
    r.sort_by(|a, b| b.total_cmp(a));
    let one_way = match r.as_slice() {
        [] => return START_DELAY,
        [a] => a / 2.0,
        [a, b, ..] => (a + b) / 2.0,
    };
    let field_ms = FIELD_MS as f32;
    (((one_way + DELAY_MARGIN_MS) / field_ms).ceil() as u64).clamp(1, MAX_DELAY)
}

/// Host: a joiner let in, whose game is saved at `save_at` ([`Session::admit`]).
struct PendingJoin {
    stream: TcpStream,
    slot: usize,
    save_at: u64,
    joins_at: u64,
    /// The session's Exit Course from `save_at` on.
    exit_course: bool,
}

pub struct Session {
    /// The remote this machine plays.
    pub slot: usize,
    host: bool,
    links: Links,
    inbox: Receiver<Msg>,
    inbox_tx: Sender<Msg>,
    /// Joiners that said hello, waiting for the next field boundary (host),
    /// and whether each has Exit Course on.
    arrivals: Option<Receiver<(TcpStream, bool)>>,
    /// Host: joiners let in whose game is not saved yet.
    pending: Vec<PendingJoin>,
    /// Joiner: fields at whose start the host saves the game for a new
    /// joiner, and the session's Exit Course from there.
    saves: Vec<(u64, bool)>,
    /// Exit Course in every course: on while every player has it on.
    pub exit_course: bool,
    wait_join: Option<u64>,
    /// Inputs by field and remote.
    inputs: BTreeMap<u64, [Option<Input>; REMOTES]>,
    /// When the last of each field's inputs from the network came in.
    arrived: BTreeMap<u64, Instant>,
    /// The field from which each remote's input comes from the network,
    /// and the field from which it no longer does (it left).
    active_from: [Option<u64>; REMOTES],
    active_until: [Option<u64>; REMOTES],
    /// The last local input, repeated over the delay when the first peer joins.
    last_local: Input,
    /// This machine's own input is recorded (and sent) up to this field.
    sent_until: Option<u64>,
    /// Fields between sampling the local input and using it, once anyone
    /// has joined: the host picks it from the round trips ([`pick_delay`]).
    delay: u64,
    /// Host: what times pings (microseconds since then).
    epoch: Instant,
    /// Host: recent round trips to each joiner (ms).
    round_trips: [VecDeque<f32>; REMOTES],
    /// Host: since when a lower delay would have done.
    lower_since: Option<u64>,
    /// This window's least slack (ms): how long before it was needed the
    /// last input of a field came in (negative: this machine waited).
    slack: Option<f32>,
    /// The other machines' slack (ms) and the field it came by: on the host
    /// each joiner's, on a joiner the least of everyone else's (slot 0).
    their_slack: [Option<(f32, u64)>; REMOTES],
    /// Time to add before the next field ([`Session::take_shift`]).
    shift: Duration,
    /// Fields played with someone else in the session, and those that waited
    /// for input (and how long in all).
    fields: u64,
    waits: (u64, Duration),
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
    fn new(slot: usize, host: bool, exit_course: bool, nand: PathBuf) -> Self {
        let (inbox_tx, inbox) = mpsc::channel();
        Self {
            slot,
            host,
            links: Arc::default(),
            inbox,
            inbox_tx,
            arrivals: None,
            pending: Vec::new(),
            saves: Vec::new(),
            exit_course,
            wait_join: None,
            inputs: BTreeMap::new(),
            arrived: BTreeMap::new(),
            active_from: [None; REMOTES],
            active_until: [None; REMOTES],
            last_local: Input::default(),
            sent_until: None,
            delay: START_DELAY,
            epoch: Instant::now(),
            round_trips: Default::default(),
            lower_since: None,
            slack: None,
            their_slack: [None; REMOTES],
            shift: Duration::ZERO,
            fields: 0,
            waits: (0, Duration::ZERO),
            ours: BTreeMap::new(),
            theirs: BTreeMap::new(),
            nand,
        }
    }

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
        let mut s = Self::new(0, true, exit_course, nand);
        s.active_from[0] = Some(0);
        s.arrivals = Some(rx);
        s.wait_join = wait_join;
        Ok(s)
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
        if exit_course && !w.exit_course {
            println!("net: Exit Course in every course is off in this game: not every player has it on");
        }
        let mut s = Self::new(w.slot, false, w.exit_course, nand);
        for (f, slot, i) in &w.inputs {
            s.inputs.entry(*f).or_default()[*slot] = Some(*i);
        }
        (s.active_from, s.active_until, s.delay, s.saves) = (w.active_from, w.active_until, w.delay, std::mem::take(&mut w.saves));
        // Our own input is needed from the field we join at.
        s.sent_until = w.active_from[w.slot].and_then(|f| f.checked_sub(1));
        let writer = spawn_writer(stream.try_clone().map_err(|e| e.to_string())?, 0).map_err(|e| e.to_string())?;
        s.links.lock().unwrap().push(Link { slot: 0, tx: writer });
        spawn_reader(stream, 0, s.links.clone(), false, s.inbox_tx.clone()).map_err(|e| e.to_string())?;
        Ok((s, w.field, std::mem::take(&mut w.state)))
    }

    fn linked(&self) -> bool {
        !self.links.lock().unwrap().is_empty()
    }

    fn delay(&self) -> u64 {
        if self.linked() {
            self.delay
        } else {
            0
        }
    }

    /// Sends a message to every machine linked to this one.
    fn broadcast(&self, kind: u8, body: &[u8]) {
        let bytes = Arc::new(frame(kind, body));
        for l in self.links.lock().unwrap().iter() {
            l.send(&bytes);
        }
    }

    /// Records this machine's input `input` for every field after the last
    /// one it has to `upto` (a delay that grew leaves no field without
    /// one), sending each to `links`.
    fn record_own(&mut self, upto: u64, input: Input, links: &[Link]) {
        let from = self.sent_until.map_or(upto, |s| s + 1);
        for f in from..=upto {
            self.inputs.entry(f).or_default()[self.slot] = Some(input);
            if !links.is_empty() {
                let (mut f, mut slot, mut i) = (f, self.slot, input);
                let bytes = Arc::new(frame(INPUT, &encode(|s| guest_mem::snap!(s; f, slot, i))));
                for l in links {
                    l.send(&bytes);
                }
            }
        }
        if from <= upto {
            self.sent_until = Some(upto);
        }
    }

    /// Host: lets in anyone waiting, at the start of field `index`. With
    /// `wait_join`, joiners wait for that field, which waits for one.
    ///
    /// The game is saved for a joiner at the first field no other machine
    /// can have played yet (this machine's input for it is not sent): saving
    /// forgets the light-texture keys, so the joiners already in forget them
    /// at the start of that field too, and take the session's Exit Course
    /// from there ([`JOINED`]). Alone, that is this field.
    fn admit(&mut self, sys: &mut System, index: u64, stop: &dyn Fn() -> bool) -> Result<(), String> {
        let Some(arrivals) = &self.arrivals else { return Ok(()) };
        if !self.wait_join.is_some_and(|w| index < w) {
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
                let links = self.links.clone();
                let all = links.lock().unwrap();
                let save_at = if all.is_empty() { index } else { self.sent_until.map_or(index, |s| s + 1).max(index) };
                let joins_at = save_at + self.delay;
                // The session's Exit Course after the joins already on the way.
                let before = self.pending.last().map_or(self.exit_course, |p| p.exit_course);
                let ec = before && exit_course;
                if before && !exit_course {
                    println!("net: player {} does not have Exit Course in every course on: it is off in this game from field {save_at}", slot + 1);
                }
                self.active_from[slot] = Some(joins_at);
                self.active_until[slot] = None;
                let (mut s, mut sa, mut j, mut e) = (slot, save_at, joins_at, ec);
                let bytes = Arc::new(frame(JOINED, &encode(|d| guest_mem::snap!(d; s, sa, j, e))));
                for l in all.iter() {
                    l.send(&bytes);
                }
                drop(all);
                println!("net: player {} joins: the game is saved for them at field {save_at}, and they play from field {joins_at}", slot + 1);
                self.pending.push(PendingJoin { stream, slot, save_at, joins_at, exit_course: ec });
            }
        }
        while self.pending.first().is_some_and(|p| p.save_at <= index) {
            let p = self.pending.remove(0);
            self.welcome(sys, index, p)?;
        }
        Ok(())
    }

    /// Host: saves the game at the start of field `index` and sends it to
    /// the joiner `p` (deflated on its writer thread).
    fn welcome(&mut self, sys: &mut System, index: u64, p: PendingJoin) -> Result<(), String> {
        self.exit_course = p.exit_course;
        sys.menu.exit_course = p.exit_course;
        let links = self.links.clone();
        let mut all = links.lock().unwrap();
        // Inputs the readers passed on before this lock are waiting here:
        // taken in, so the welcome has them.
        let gone = self.take_inbox(None)?;
        // Our own input up to the field before the joiner plays (over the
        // delay, if we were alone until now).
        self.record_own(p.joins_at - 1, self.last_local, &all);
        let started = Instant::now();
        let state = sys.save_state();
        let inputs: Vec<(u64, usize, Input)> = self.inputs.range(index..).flat_map(|(&f, a)| a.iter().enumerate().filter_map(move |(s, i)| i.map(|i| (f, s, i)))).collect();
        // Joins still on the way: the new joiner forgets the keys at theirs too.
        let saves = self.pending.iter().map(|q| (q.save_at, q.exit_course)).collect();
        let mut w = Welcome { slot: p.slot, field: index, active_from: self.active_from, active_until: self.active_until, inputs, nand: read_tree(&self.nand), state, exit_course: self.exit_course, delay: self.delay, saves };
        let raw = encode(|s| w.snap(s));
        let writer = p.stream.try_clone().and_then(|s| spawn_writer(s, p.slot)).map_err(|e| e.to_string())?;
        let _ = writer.send((Instant::now(), Out::Welcome(raw)));
        all.push(Link { slot: p.slot, tx: writer });
        drop(all);
        spawn_reader(p.stream, p.slot, self.links.clone(), true, self.inbox_tx.clone()).map_err(|e| e.to_string())?;
        println!("net: saved the game for player {} at field {index} ({:.1} s)", p.slot + 1, started.elapsed().as_secs_f32());
        self.handle_gone(gone, index)
    }

    /// Host: the machine playing `slot` disconnected. Its remote switches
    /// off after the last field it sent input for (never before `index`),
    /// on every machine.
    fn drop_peer(&mut self, slot: usize, index: u64) {
        self.links.lock().unwrap().retain(|l| l.slot != slot);
        let last = self.inputs.iter().rev().find(|(_, a)| a[slot].is_some()).map(|(&f, _)| f);
        let mut at = last.map_or(index, |f| f + 1).max(index);
        self.active_until[slot] = Some(at);
        self.round_trips[slot].clear();
        self.their_slack[slot] = None;
        println!("net: player {} left; their remote switches off at field {at}", slot + 1);
        let mut gone = slot;
        self.broadcast(LEAVE, &encode(|s| guest_mem::snap!(s; gone, at)));
    }

    /// Whether remote `s` plays field `n` with input from the network.
    fn active(&self, s: usize, n: u64) -> bool {
        self.active_from[s].is_some_and(|a| a <= n) && self.active_until[s].is_none_or(|u| n < u)
    }

    /// Handles one message from the machine playing `slot`, which came in
    /// at `at`.
    fn handle(&mut self, slot: usize, at: Instant, kind: u8, body: Vec<u8>) -> Result<(), String> {
        match kind {
            INPUT => {
                let (mut f, mut s, mut i) = (0u64, 0usize, Input::default());
                decode(body, |d| guest_mem::snap!(d; f, s, i))?;
                if s >= REMOTES {
                    return Err("net: input for a remote that does not exist".into());
                }
                self.inputs.entry(f).or_default()[s] = Some(i);
                let last = self.arrived.entry(f).or_insert(at);
                *last = (*last).max(at);
            }
            JOINED if !self.host => {
                let (mut s, mut save_at, mut from, mut ec) = (0usize, 0u64, 0u64, false);
                decode(body, |d| guest_mem::snap!(d; s, save_at, from, ec))?;
                if s < REMOTES {
                    self.active_from[s] = Some(from);
                    self.active_until[s] = None;
                    self.saves.push((save_at, ec));
                    println!("net: player {} joins, playing from field {from}", s + 1);
                }
            }
            SET_DELAY if !self.host => {
                let mut d = 0u64;
                decode(body, |e| guest_mem::snap!(e; d))?;
                self.delay = d.clamp(1, MAX_DELAY);
            }
            PONG if self.host => {
                let mut sent = 0u64;
                decode(body, |e| guest_mem::snap!(e; sent))?;
                let ms = at.saturating_duration_since(self.epoch + Duration::from_micros(sent)).as_secs_f32() * 1000.0;
                let r = &mut self.round_trips[slot];
                r.push_back(ms);
                if r.len() > 8 {
                    r.pop_front();
                }
            }
            SYNC => {
                let mut us = 0i64;
                decode(body, |e| guest_mem::snap!(e; us))?;
                self.their_slack[slot] = Some((us as f32 / 1000.0, self.fields));
            }
            LEAVE if !self.host => {
                let (mut s, mut at) = (0usize, 0u64);
                decode(body, |d| guest_mem::snap!(d; s, at))?;
                if s < REMOTES {
                    self.active_until[s] = Some(at);
                    println!("net: player {} left; their remote switches off at field {at}", s + 1);
                }
            }
            HASH => {
                let (mut f, mut h) = (0u64, 0u64);
                decode(body, |d| guest_mem::snap!(d; f, h))?;
                self.theirs.entry(f).or_default().push(h);
                self.check_hashes()?;
            }
            _ => return Err(format!("net: unexpected message {kind} from player {}", slot + 1)),
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

    /// Handles what has come in, waiting up to `wait` for the first message
    /// (`None`: not at all). Returns the remotes whose connection ended.
    fn take_inbox(&mut self, wait: Option<Duration>) -> Result<Vec<usize>, String> {
        let first = match wait {
            Some(w) => self.inbox.recv_timeout(w).ok(),
            None => self.inbox.try_recv().ok(),
        };
        let mut msgs: Vec<Msg> = first.into_iter().collect();
        msgs.extend(self.inbox.try_iter());
        let mut gone = Vec::new();
        for Msg { slot, at, m } in msgs {
            match m {
                Ok((kind, body)) => self.handle(slot, at, kind, body)?,
                Err(_) if !gone.contains(&slot) => gone.push(slot),
                Err(_) => {}
            }
        }
        Ok(gone)
    }

    /// Host: drops machines that left. A joiner cannot go on without the host.
    fn handle_gone(&mut self, gone: Vec<usize>, index: u64) -> Result<(), String> {
        if gone.is_empty() {
            return Ok(());
        }
        if !self.host {
            return Err("net: the host left".into());
        }
        for slot in gone {
            self.drop_peer(slot, index);
        }
        Ok(())
    }

    /// Host: the delay from the round trips measured, raised at once and
    /// lowered once a lower one has done for [`LOWER_AFTER`] fields; every
    /// joiner is told.
    fn update_delay(&mut self, index: u64) {
        let trips: Vec<f32> = self.round_trips.iter().filter_map(|r| r.iter().copied().reduce(f32::max)).collect();
        if trips.is_empty() {
            return;
        }
        let want = pick_delay(&trips);
        let new = if want > self.delay {
            self.lower_since = None;
            want
        } else if want < self.delay {
            let since = *self.lower_since.get_or_insert(index);
            if index >= since + LOWER_AFTER {
                self.lower_since = None;
                want
            } else {
                self.delay
            }
        } else {
            self.lower_since = None;
            self.delay
        };
        if new != self.delay {
            self.delay = new;
            let longest = trips.iter().copied().fold(0.0, f32::max);
            println!("net: input delay {new} field{} (longest round trip {longest:.0} ms)", if new == 1 { "" } else { "s" });
            let mut d = new;
            self.broadcast(SET_DELAY, &encode(|e| guest_mem::snap!(e; d)));
        }
    }

    /// Every [`SYNC_EVERY`] fields: tells the others this machine's least
    /// slack, and moves this machine's fields later when it has much less
    /// than they do, which evens it out (a machine with none waits at every
    /// late packet; the one with all the slack loses nothing by giving some).
    fn sync(&mut self, index: u64) {
        if index % SYNC_EVERY != 0 || !self.linked() {
            return;
        }
        let mine = self.slack.take();
        let fresh = |v: Option<(f32, u64)>, now: u64| v.filter(|(_, at)| now <= at + 2 * SYNC_EVERY).map(|(s, _)| s);
        let others = if self.host {
            let links = self.links.lock().unwrap();
            for l in links.iter() {
                // The least of the host's and the other joiners'.
                let rest = (1..REMOTES).filter(|&s| s != l.slot).filter_map(|s| fresh(self.their_slack[s], self.fields));
                if let Some(least) = mine.into_iter().chain(rest).reduce(f32::min) {
                    let mut us = (least * 1000.0) as i64;
                    l.send(&Arc::new(frame(SYNC, &encode(|e| guest_mem::snap!(e; us)))));
                }
            }
            (1..REMOTES).filter_map(|s| fresh(self.their_slack[s], self.fields)).reduce(f32::min)
        } else {
            if let Some(m) = mine {
                let mut us = (m * 1000.0) as i64;
                self.broadcast(SYNC, &encode(|e| guest_mem::snap!(e; us)));
            }
            fresh(self.their_slack[0], self.fields)
        };
        if let (Some(mine), Some(others)) = (mine, others) {
            let gap = Duration::from_secs_f32(((others - mine) / 2.0 / 1000.0).max(0.0));
            if gap >= SYNC_MIN {
                self.shift += gap.min(SYNC_MAX_STEP);
            }
        }
    }

    /// Time to wait before the next field ([`Session::sync`]), taken.
    pub fn take_shift(&mut self) -> Duration {
        std::mem::take(&mut self.shift)
    }

    /// The remotes' inputs for field `index`: admits joiners (host), records
    /// and sends `local` for its field, and waits for everyone's input for
    /// this one.
    pub fn inputs(&mut self, sys: &mut System, index: u64, local: Input, stop: &dyn Fn() -> bool) -> Result<[Input; REMOTES], String> {
        if self.host {
            self.admit(sys, index, stop)?;
            self.update_delay(index);
        }
        let mut local = local;
        local.connected = true;
        self.last_local = local;
        let target = index + self.delay();
        {
            let links = self.links.clone();
            let all = links.lock().unwrap();
            self.record_own(target, local, &all);
        }
        let linked = self.linked();
        if self.host && linked && index % PING_EVERY == 0 {
            // Only joiners already playing: one still loading the game
            // answers late, and a round trip that long would raise the
            // delay for nothing.
            let mut t = self.epoch.elapsed().as_micros() as u64;
            let bytes = Arc::new(frame(PING, &encode(|e| guest_mem::snap!(e; t))));
            for l in self.links.lock().unwrap().iter().filter(|l| self.active(l.slot, index)) {
                l.send(&bytes);
            }
        }
        let started = Instant::now();
        let mut waited = false;
        loop {
            let gone = self.take_inbox(None)?;
            self.handle_gone(gone, index)?;
            let row = self.inputs.get(&index).copied().unwrap_or_default();
            let Some(s) = (0..REMOTES).find(|&s| self.active(s, index) && row[s].is_none()) else { break };
            if stop() {
                return Err("stopped".into());
            }
            if started.elapsed() > TIMEOUT {
                return Err(format!("net: no input from player {} for field {index} in {} s", s + 1, TIMEOUT.as_secs()));
            }
            waited = true;
            let gone = self.take_inbox(Some(Duration::from_millis(5)))?;
            self.handle_gone(gone, index)?;
        }
        // The host saves the game for a joiner at the start of this field:
        // as saving does there, the light-texture keys are forgotten, and
        // the session's Exit Course holds from here.
        while let Some(i) = self.saves.iter().position(|&(f, _)| f <= index) {
            let (_, ec) = self.saves.remove(i);
            sys.light_textures.forget();
            self.exit_course = ec;
            sys.menu.exit_course = ec;
        }
        if linked {
            self.fields += 1;
            let now = Instant::now();
            if waited {
                self.waits.0 += 1;
                self.waits.1 += now - started;
            }
            if let Some(&at) = self.arrived.get(&index) {
                let ms = if at > started { -((at - started).as_secs_f32()) } else { (started - at).as_secs_f32() } * 1000.0;
                self.slack = Some(self.slack.map_or(ms, |m| m.min(ms)));
            }
            self.sync(index);
        }
        let row = self.inputs.remove(&index).unwrap_or_default();
        while self.inputs.first_key_value().is_some_and(|(&f, _)| f < index) {
            self.inputs.pop_first();
        }
        while self.arrived.first_key_value().is_some_and(|(&f, _)| f <= index) {
            self.arrived.pop_first();
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
        if index % HASH_EVERY != 0 || !self.linked() {
            return Ok(());
        }
        let mut h = sys.mem.hash();
        self.ours.insert(index, h);
        let mut f = index;
        self.broadcast(HASH, &encode(|s| guest_mem::snap!(s; f, h)));
        self.check_hashes()
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        if self.fields > 0 {
            println!(
                "net: {} of {} fields waited for input ({:.0} ms in all); input delay {} field{}",
                self.waits.0,
                self.fields,
                self.waits.1.as_secs_f32() * 1000.0,
                self.delay,
                if self.delay == 1 { "" } else { "s" }
            );
        }
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
        let mut w = Welcome { slot: 1, field: 77, active_from: [Some(0), Some(79), None, None], active_until: [None; REMOTES], inputs: vec![(78, 0, Input { connected: true, buttons: 3, shake: true, tilt: -0.5 })], nand: vec![("a/b".into(), vec![1, 2])], state: vec![9; 10], exit_course: true, delay: 3, saves: vec![(90, false)] };
        let bytes = encode(|s| w.snap(s));
        let mut v = Welcome::default();
        decode(bytes, |s| v.snap(s)).unwrap();
        assert_eq!((v.slot, v.field, v.active_from, v.inputs.len(), v.nand.len(), v.state.len(), v.exit_course, v.delay), (1, 77, w.active_from, 1, 1, 10, true, 3));
        assert_eq!(v.inputs[0].2, w.inputs[0].2);
    }

    #[test]
    fn delay_follows_the_round_trips() {
        assert_eq!(pick_delay(&[]), START_DELAY);
        // A LAN: one field.
        assert_eq!(pick_delay(&[0.4]), 1);
        // 60 ms there and back: 30 + 6 ms is three fields.
        assert_eq!(pick_delay(&[60.0]), 3);
        // Two joiners: one's input reaches the other through the host.
        assert_eq!(pick_delay(&[60.0, 40.0, 10.0]), 4);
        assert_eq!(pick_delay(&[5000.0]), MAX_DELAY);
    }

    #[test]
    fn nand_paths_cannot_escape() {
        let dir = std::env::temp_dir().join("nsmbw-net-test");
        assert!(write_tree(&dir, &[("../x".into(), vec![])]).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
