//! Finding rooms, after WitherBlitz's NSMB-Wii-Recompilation's LAN play: a
//! game hosting a room (`--host`) answers queries on UDP port
//! [`DISCOVERY_PORT`] with its name and TCP port, and a finder (the PC
//! Settings window's LAN tab, the Android app's LAN Play) sends a query
//! every second (broadcast, and to this device) and lists the answers.
//! Joining, hosting and leaving start the game again with `--join`,
//! `--host` or neither: every player of a room runs the game from boot, and
//! anyone joins at any time (`net.rs`).
//!
//! Tailscale carries no broadcasts, so over it rooms are found the way the
//! recompilation finds them:
//! - a PC asks every device `tailscale status` lists, and a host announces
//!   its room (an answer nobody asked for) every second to those devices
//!   and to the known ones;
//! - the known devices ([`DEVICES`]) are the Tailscale devices this one met
//!   running the game (a query or a room from a Tailscale address, or a
//!   room joined), the ones other devices passed on, and the ones the
//!   player added;
//! - finders and hosts pass on their own Tailscale address and the known
//!   devices, so every device on a tailnet that runs the game learns the
//!   others through any one it meets;
//! - a phone's finder listens on [`DISCOVERY_PORT`] to hear those
//!   announcements, since it cannot list the tailnet's devices.
//!
//! Packets: "NWPL", a version byte, then
//! - `?`: a query;
//! - `!`, the room's TCP port (u16, little-endian) and its name (UTF-8): a
//!   room;
//! - `D`, a count and that many IPv4 addresses: Tailscale devices that run
//!   the game, the sender's own first. Older games ignore it.

use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs, UdpSocket};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Where hosts listen for queries.
pub const DISCOVERY_PORT: u16 = 52130;
/// The TCP port a room hosted from the LAN tab listens on.
pub const GAME_PORT: u16 = 52131;
const MAGIC: &[u8; 4] = b"NWPL";
const VERSION: u8 = 1;
/// A room not heard from for this long is dropped from the list.
const FORGET: Duration = Duration::from_secs(4);
/// The known devices, one Tailscale name or address per line (relative to
/// the working folder: the PC's game folder, the Android app's files).
pub const DEVICES: &str = "saves/lan-devices.txt";
const MAX_DEVICES: usize = 64;
/// Devices passed on in one packet.
const MAX_SHARED: usize = 16;
/// How often the devices to ask are found again (the known ones, and the
/// Tailscale CLI's peers).
const REFRESH: Duration = Duration::from_secs(10);

#[derive(Clone, Debug, PartialEq)]
pub struct Room {
    pub name: String,
    /// Where to join it (`--join`).
    pub addr: SocketAddr,
}

impl Room {
    /// Whether it was found over Tailscale.
    pub fn tailscale(&self) -> bool {
        is_tailscale(self.addr.ip())
    }
}

/// Whether `ip` is a Tailscale address (100.64.0.0/10).
pub fn is_tailscale(ip: IpAddr) -> bool {
    matches!(ip, IpAddr::V4(v4) if v4.octets()[0] == 100 && v4.octets()[1] & 0xC0 == 64)
}

/// This PC's name, as rooms are called.
pub fn computer_name() -> String {
    std::env::var("COMPUTERNAME").ok().filter(|n| !n.is_empty()).unwrap_or_else(|| "NSMBW".into())
}

/// The address a packet to `to` leaves this device from (no packet is sent).
fn route_from(to: Ipv4Addr) -> Option<IpAddr> {
    let s = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).ok()?;
    s.connect((to, 9)).ok()?;
    s.local_addr().ok().map(|a| a.ip()).filter(|ip| !ip.is_unspecified())
}

/// This device's address on its network (the one its default route uses).
pub fn local_address() -> Option<IpAddr> {
    route_from(Ipv4Addr::new(192, 0, 2, 1))
}

/// This device's Tailscale address, while Tailscale is on (the one a packet
/// to Tailscale's own 100.100.100.100 leaves from).
pub fn tailscale_address() -> Option<Ipv4Addr> {
    match route_from(Ipv4Addr::new(100, 100, 100, 100)) {
        Some(IpAddr::V4(ip)) if is_tailscale(ip.into()) => Some(ip),
        _ => None,
    }
}

fn reply(port: u16, name: &str) -> Vec<u8> {
    let mut p = MAGIC.to_vec();
    p.push(VERSION);
    p.push(b'!');
    p.extend_from_slice(&port.to_le_bytes());
    p.extend_from_slice(name.as_bytes());
    p
}

fn query() -> Vec<u8> {
    let mut p = MAGIC.to_vec();
    p.extend_from_slice(&[VERSION, b'?']);
    p
}

/// A reply's room, coming from `from`.
fn parse_reply(p: &[u8], from: SocketAddr) -> Option<Room> {
    if p.len() < 8 || &p[..4] != MAGIC || p[4] != VERSION || p[5] != b'!' {
        return None;
    }
    let port = u16::from_le_bytes([p[6], p[7]]);
    let name = String::from_utf8_lossy(&p[8..]).chars().filter(|c| !c.is_control()).take(24).collect();
    Some(Room { name, addr: SocketAddr::new(from.ip(), port) })
}

/// The devices to pass on: `own` (this device's Tailscale address) first,
/// then the known devices' Tailscale addresses.
fn devices_packet(own: Option<Ipv4Addr>) -> Vec<u8> {
    let mut ips: Vec<Ipv4Addr> = own.into_iter().collect();
    for d in known_devices() {
        if let Ok(ip) = d.parse::<Ipv4Addr>() {
            if is_tailscale(ip.into()) && !ips.contains(&ip) && ips.len() < MAX_SHARED {
                ips.push(ip);
            }
        }
    }
    let mut p = MAGIC.to_vec();
    p.extend_from_slice(&[VERSION, b'D', ips.len() as u8]);
    for ip in ips {
        p.extend_from_slice(&ip.octets());
    }
    p
}

fn parse_devices(p: &[u8]) -> Option<Vec<Ipv4Addr>> {
    if p.len() < 7 || &p[..4] != MAGIC || p[4] != VERSION || p[5] != b'D' {
        return None;
    }
    Some(p[7..].chunks_exact(4).take((p[6] as usize).min(MAX_SHARED)).map(|b| Ipv4Addr::new(b[0], b[1], b[2], b[3])).collect())
}

/// Guards [`DEVICES`] between threads.
static DEVICES_LOCK: Mutex<()> = Mutex::new(());

fn read_devices() -> Vec<String> {
    let text = std::fs::read_to_string(DEVICES).unwrap_or_default();
    text.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')).map(String::from).collect()
}

fn write_devices(list: &[String]) {
    let mut text = String::from("# Devices asked for LAN play rooms directly (Tailscale carries no broadcasts).\n# One Tailscale name or address per line.\n");
    for d in list {
        text.push_str(d);
        text.push('\n');
    }
    if let Some(dir) = std::path::Path::new(DEVICES).parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let _ = std::fs::write(DEVICES, text);
}

/// The devices asked for rooms directly ([`DEVICES`]).
pub fn known_devices() -> Vec<String> {
    let _guard = DEVICES_LOCK.lock().unwrap();
    read_devices()
}

/// Asks `text` (a Tailscale name or an address) for rooms from now on.
/// False when it is known already or is not a name or address.
pub fn add_device(text: &str) -> bool {
    let text = text.trim();
    let valid = !text.is_empty() && text.len() <= 253 && text.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_' | ':'));
    if !valid {
        return false;
    }
    let _guard = DEVICES_LOCK.lock().unwrap();
    let mut list = read_devices();
    if list.len() >= MAX_DEVICES || list.iter().any(|d| d.eq_ignore_ascii_case(text)) {
        return false;
    }
    list.push(text.to_string());
    write_devices(&list);
    true
}

/// Stops asking `text` for rooms.
pub fn remove_device(text: &str) {
    let _guard = DEVICES_LOCK.lock().unwrap();
    let mut list = read_devices();
    list.retain(|d| d != text);
    write_devices(&list);
}

/// After joining the room at `addr` (`ip:port`): a Tailscale host is asked
/// for rooms from now on.
pub fn joined(addr: &str) {
    if let Ok(a) = addr.trim().parse::<SocketAddr>() {
        if is_tailscale(a.ip()) {
            add_device(&a.ip().to_string());
        }
    }
}

/// The Tailscale CLI's `status --json` (none on Android, or without
/// Tailscale).
#[cfg(not(target_os = "android"))]
fn tailscale_status() -> Option<String> {
    let exes: &[&str] = if cfg!(windows) { &[r"C:\Program Files\Tailscale\tailscale.exe", "tailscale"] } else if cfg!(target_os = "macos") { &["tailscale", "/Applications/Tailscale.app/Contents/MacOS/Tailscale"] } else { &["tailscale"] };
    for exe in exes {
        let mut cmd = std::process::Command::new(exe);
        cmd.args(["status", "--json"]).stdin(std::process::Stdio::null()).stderr(std::process::Stdio::null());
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }
        if let Ok(out) = cmd.output() {
            if out.status.success() {
                return Some(String::from_utf8_lossy(&out.stdout).into_owned());
            }
        }
    }
    None
}

#[cfg(target_os = "android")]
fn tailscale_status() -> Option<String> {
    None
}

/// The peers' Tailscale IPv4 addresses in `tailscale status --json`: those
/// under "Peer" (this device's own come before it, at the top and in
/// "Self").
fn parse_peers(json: &str) -> Vec<Ipv4Addr> {
    let mut out = Vec::new();
    let Some(start) = json.find("\"Peer\"") else { return out };
    let mut rest = &json[start..];
    while let Some(i) = rest.find("\"TailscaleIPs\"") {
        rest = &rest[i + "\"TailscaleIPs\"".len()..];
        let (Some(open), Some(close)) = (rest.find('['), rest.find(']')) else { break };
        if open > close {
            break;
        }
        for part in rest[open + 1..close].split(',') {
            if let Ok(ip) = part.trim().trim_matches('"').parse::<Ipv4Addr>() {
                if is_tailscale(ip.into()) && !out.contains(&ip) {
                    out.push(ip);
                }
            }
        }
        rest = &rest[close..];
    }
    out
}

/// The devices asked for rooms (and told of this device's) one by one: the
/// known devices and, on a PC, the Tailscale CLI's peers. Found again every
/// [`REFRESH`] on a thread of its own while this lives: resolving names
/// and running the CLI can take a moment.
#[derive(Clone)]
struct Targets {
    list: Arc<Mutex<Vec<Ipv4Addr>>>,
    /// This device's Tailscale address, never asked.
    own: Option<Ipv4Addr>,
}

impl Targets {
    fn spawn() -> Self {
        let own = tailscale_address();
        let list = Arc::new(Mutex::new(Vec::new()));
        let weak = Arc::downgrade(&list);
        let _ = std::thread::Builder::new().name("lan-devices".into()).spawn(move || loop {
            let mut found: Vec<Ipv4Addr> = Vec::new();
            let mut names = known_devices();
            names.extend(tailscale_status().map(|j| parse_peers(&j)).unwrap_or_default().iter().map(Ipv4Addr::to_string));
            for name in names {
                let addr = match name.parse::<Ipv4Addr>() {
                    Ok(ip) => Some(ip),
                    Err(_) => (name.as_str(), DISCOVERY_PORT).to_socket_addrs().ok().and_then(|mut a| a.find_map(|a| match a.ip() {
                        IpAddr::V4(ip) => Some(ip),
                        IpAddr::V6(_) => None,
                    })),
                };
                if let Some(ip) = addr.filter(|ip| Some(*ip) != own && !ip.is_loopback() && !found.contains(ip)) {
                    found.push(ip);
                }
            }
            let Some(list) = weak.upgrade() else { return };
            {
                let mut list = list.lock().unwrap();
                // The ones learned since stay (a name may not resolve).
                for ip in list.iter() {
                    if !found.contains(ip) {
                        found.push(*ip);
                    }
                }
                if found.len() != list.len() {
                    println!("LAN: asking {} known or Tailscale device(s) directly", found.len());
                }
                *list = found;
            }
            drop(list);
            std::thread::sleep(REFRESH);
        });
        Self { list, own }
    }

    fn list(&self) -> Vec<Ipv4Addr> {
        self.list.lock().unwrap().clone()
    }

    /// A Tailscale device that runs the game: asked from now on, this time
    /// and the next.
    fn learn(&self, ip: Ipv4Addr) {
        if !is_tailscale(ip.into()) || Some(ip) == self.own {
            return;
        }
        let mut list = self.list.lock().unwrap();
        if list.contains(&ip) {
            return;
        }
        list.push(ip);
        drop(list);
        if add_device(&ip.to_string()) {
            println!("LAN: will ask {ip} for rooms from now on");
        }
    }

    /// Sends `packets` to every device asked.
    fn send(&self, socket: &UdpSocket, packets: &[&[u8]]) {
        for ip in self.list() {
            for p in packets {
                let _ = socket.send_to(p, (ip, DISCOVERY_PORT));
            }
        }
    }
}

/// Whether a socket error only means a packet sent earlier was refused
/// (Windows reports that on the next receive).
fn refused(e: &std::io::Error) -> bool {
    e.kind() == std::io::ErrorKind::ConnectionReset
}

/// Answers room queries for as long as the game runs, and announces the
/// room to the known and Tailscale devices every second: this device hosts
/// a room on TCP `port`. Fails when another game on this device already
/// answers.
pub fn announce(port: u16, name: String) -> std::io::Result<()> {
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, DISCOVERY_PORT))?;
    socket.set_read_timeout(Some(Duration::from_millis(250)))?;
    std::thread::Builder::new().name("lan-announce".into()).spawn(move || {
        let targets = Targets::spawn();
        let answer = reply(port, &name);
        let mut told: Option<Instant> = None;
        let mut buf = [0u8; 128];
        loop {
            if told.is_none_or(|t| t.elapsed() >= Duration::from_secs(1)) {
                told = Some(Instant::now());
                targets.send(&socket, &[&answer, &devices_packet(targets.own)]);
            }
            match socket.recv_from(&mut buf) {
                Ok((n, from)) => {
                    let p = &buf[..n];
                    if p == query() {
                        let _ = socket.send_to(&answer, from);
                        let _ = socket.send_to(&devices_packet(targets.own), from);
                        // Whoever looks for rooms over Tailscale runs the game too.
                        if let IpAddr::V4(ip) = from.ip() {
                            targets.learn(ip);
                        }
                    } else if let Some(ips) = parse_devices(p) {
                        for ip in ips {
                            targets.learn(ip);
                        }
                    }
                }
                Err(e) if refused(&e) || matches!(e.kind(), std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut) => {}
                Err(_) => return,
            }
        }
    })?;
    Ok(())
}

/// Looks for rooms while the LAN tab (or the app's LAN Play) is shown.
pub struct Finder {
    /// This device's address on its network, when it has one.
    pub here: Option<IpAddr>,
    /// This device's Tailscale address, while Tailscale is on.
    pub tailscale: Option<Ipv4Addr>,
    socket: UdpSocket,
    rooms: Vec<(Room, Instant)>,
    asked: Option<Instant>,
    targets: Option<Targets>,
}

/// How much a room's address is worth keeping over another for the same
/// room: the LAN's over Tailscale's over this device's 127.0.0.1.
fn rank(ip: IpAddr) -> u8 {
    if ip.is_loopback() {
        0
    } else if is_tailscale(ip) {
        1
    } else {
        2
    }
}

impl Finder {
    /// On any port: a PC asks every device on its tailnet, so it needs no
    /// announcements (and a fixed port makes Windows' firewall ask).
    pub fn new() -> std::io::Result<Self> {
        Self::open(false)
    }

    /// On [`DISCOVERY_PORT`] when it is free, so hosts' announcements
    /// arrive: a phone cannot list its tailnet's devices.
    pub fn listening() -> std::io::Result<Self> {
        Self::open(true)
    }

    fn open(listen: bool) -> std::io::Result<Self> {
        let socket = match listen {
            true => UdpSocket::bind((Ipv4Addr::UNSPECIFIED, DISCOVERY_PORT)).or_else(|_| UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)))?,
            false => UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))?,
        };
        socket.set_broadcast(true)?;
        socket.set_nonblocking(true)?;
        let targets = Targets::spawn();
        Ok(Self { here: local_address(), tailscale: targets.own, socket, rooms: Vec::new(), asked: None, targets: Some(targets) })
    }

    /// The rooms heard from lately (asking again every second).
    pub fn poll(&mut self) -> Vec<Room> {
        let now = Instant::now();
        if self.asked.is_none_or(|t| now - t >= Duration::from_secs(1)) {
            self.asked = Some(now);
            let devices = devices_packet(self.tailscale);
            for to in [Ipv4Addr::BROADCAST, Ipv4Addr::LOCALHOST] {
                let _ = self.socket.send_to(&query(), (to, DISCOVERY_PORT));
            }
            let _ = self.socket.send_to(&devices, (Ipv4Addr::BROADCAST, DISCOVERY_PORT));
            if let Some(t) = &self.targets {
                t.send(&self.socket, &[&query(), &devices]);
            }
        }
        let mut buf = [0u8; 128];
        loop {
            let (n, from) = match self.socket.recv_from(&mut buf) {
                Ok(r) => r,
                Err(e) if refused(&e) => continue,
                Err(_) => break,
            };
            if let Some(ips) = parse_devices(&buf[..n]) {
                if let Some(t) = &self.targets {
                    ips.into_iter().for_each(|ip| t.learn(ip));
                }
                continue;
            }
            let Some(room) = parse_reply(&buf[..n], from) else { continue };
            if let (Some(t), IpAddr::V4(ip)) = (&self.targets, from.ip()) {
                t.learn(ip);
            }
            self.heard(room, now);
        }
        self.rooms.retain(|(_, seen)| now - *seen < FORGET);
        self.rooms.iter().map(|(r, _)| r.clone()).collect()
    }

    /// Lists `room`, once: this device answers on both its own address and
    /// 127.0.0.1, and a host on the same LAN and tailnet on both its
    /// addresses.
    fn heard(&mut self, room: Room, now: Instant) {
        let same = |r: &Room| r.addr == room.addr || (r.name == room.name && r.addr.port() == room.addr.port() && rank(r.addr.ip()).min(rank(room.addr.ip())) < 2);
        match self.rooms.iter_mut().find(|(r, _)| same(r)) {
            Some((r, seen)) => {
                if rank(room.addr.ip()) > rank(r.addr.ip()) {
                    *r = room;
                }
                *seen = now;
            }
            None => self.rooms.push((room, now)),
        }
    }
}

/// `addr` as typed into the LAN tab, with the room port if it has none.
pub fn with_port(addr: &str) -> String {
    let addr = addr.trim();
    if addr.contains(':') {
        addr.to_string()
    } else {
        format!("{addr}:{GAME_PORT}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_reply_names_its_room() {
        let from: SocketAddr = "192.168.1.20:52130".parse().unwrap();
        let room = parse_reply(&reply(52131, "LIVING-ROOM"), from).unwrap();
        assert_eq!(room, Room { name: "LIVING-ROOM".into(), addr: "192.168.1.20:52131".parse().unwrap() });
        assert!(parse_reply(&query(), from).is_none());
        assert!(parse_reply(b"NWPL", from).is_none());
        assert_eq!(with_port("10.0.0.5"), "10.0.0.5:52131");
        assert_eq!(with_port(" 10.0.0.5:6000 "), "10.0.0.5:6000");
        assert_eq!(with_port("s26-ultra"), "s26-ultra:52131");
    }

    #[test]
    fn tailscale_addresses() {
        assert!(is_tailscale("100.64.0.1".parse().unwrap()));
        assert!(is_tailscale("100.127.163.24".parse().unwrap()));
        assert!(!is_tailscale("100.128.0.1".parse().unwrap()));
        assert!(!is_tailscale("192.168.50.10".parse().unwrap()));
        assert!(Room { name: "X".into(), addr: "100.118.175.48:52131".parse().unwrap() }.tailscale());
    }

    #[test]
    fn devices_pass_on() {
        let p = devices_packet(Some(Ipv4Addr::new(100, 64, 1, 2)));
        // The sender's own first; an old game's query check ignores it.
        assert_eq!(parse_devices(&p).unwrap()[0], Ipv4Addr::new(100, 64, 1, 2));
        assert_ne!(p, query());
        assert!(parse_reply(&p, "100.64.1.2:52130".parse().unwrap()).is_none());
        assert!(parse_devices(&query()).is_none());
        // A count longer than the packet reads what is there.
        let mut short = MAGIC.to_vec();
        short.extend_from_slice(&[VERSION, b'D', 9, 100, 64, 0, 1, 100]);
        assert_eq!(parse_devices(&short).unwrap(), vec![Ipv4Addr::new(100, 64, 0, 1)]);
    }

    #[test]
    fn peers_from_the_tailscale_cli() {
        let json = r#"{
  "TailscaleIPs": ["100.118.175.48", "fd7a:115c:a1e0::237:af31"],
  "Self": { "HostName": "TRI-BEAST", "TailscaleIPs": ["100.118.175.48", "fd7a:115c:a1e0::237:af31"] },
  "Peer": {
    "nodekey:a": { "HostName": "s26-ultra", "TailscaleIPs": ["100.126.15.31", "fd7a:115c:a1e0::1"], "Online": false },
    "nodekey:b": { "HostName": "serverpc", "TailscaleIPs": [ "100.108.118.77" ], "Online": true }
  },
  "User": { "1": { "LoginName": "x" } }
}"#;
        assert_eq!(parse_peers(json), vec![Ipv4Addr::new(100, 126, 15, 31), Ipv4Addr::new(100, 108, 118, 77)]);
        assert!(parse_peers("not json").is_empty());
    }

    /// A host on this PC answers a finder's query.
    #[test]
    fn a_finder_hears_a_host_on_this_pc() {
        // Not on the real port, which a running game may hold.
        let host = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        let host_addr = host.local_addr().unwrap();
        let socket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        socket.set_nonblocking(true).unwrap();
        let mut finder = Finder { here: None, tailscale: None, socket, rooms: Vec::new(), asked: Some(Instant::now()), targets: None };
        finder.socket.send_to(&query(), host_addr).unwrap();
        let mut buf = [0u8; 64];
        let (n, from) = host.recv_from(&mut buf).unwrap();
        assert_eq!(&buf[..n], &query()[..]);
        host.send_to(&reply(52131, "TEST"), from).unwrap();
        let mut rooms = Vec::new();
        for _ in 0..100 {
            rooms = finder.poll();
            if !rooms.is_empty() {
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        assert_eq!(rooms.len(), 1);
        assert_eq!(rooms[0].name, "TEST");
        assert_eq!(rooms[0].addr.port(), 52131);
    }

    /// The same room heard on 127.0.0.1, over Tailscale and on the LAN is
    /// listed once, at its LAN address.
    #[test]
    fn one_entry_per_room() {
        let socket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        let mut finder = Finder { here: None, tailscale: None, socket, rooms: Vec::new(), asked: Some(Instant::now()), targets: None };
        for ip in ["127.0.0.1", "100.118.175.48", "192.168.50.118", "100.118.175.48"] {
            finder.heard(Room { name: "PC".into(), addr: SocketAddr::new(ip.parse().unwrap(), 52131) }, Instant::now());
        }
        // Another PC's room of the same name on the LAN is another room.
        finder.heard(Room { name: "PC".into(), addr: "192.168.50.7:52131".parse().unwrap() }, Instant::now());
        let addrs: Vec<String> = finder.rooms.iter().map(|(r, _)| r.addr.ip().to_string()).collect();
        assert_eq!(addrs, ["192.168.50.118", "192.168.50.7"]);
    }
}
