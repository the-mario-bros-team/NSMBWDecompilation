//! Finding LAN rooms, after WitherBlitz's NSMB-Wii-Recompilation's LAN
//! play: a game hosting a room (`--host`) answers queries on UDP port
//! [`DISCOVERY_PORT`] with its name and TCP port, and the Settings window's
//! LAN tab sends a query every second (broadcast, and to this PC) and lists
//! the answers. Joining, hosting and leaving start the game again with
//! `--join`, `--host` or neither (`play.rs`): every player of a room runs
//! the game from boot, and anyone joins at any time (`net.rs`).
//!
//! Packets: "NWPL", a version byte, then `?` (a query) or `!`, the room's
//! TCP port (u16, little-endian) and its name (UTF-8).

use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::time::{Duration, Instant};

/// Where hosts listen for queries.
pub const DISCOVERY_PORT: u16 = 52130;
/// The TCP port a room hosted from the LAN tab listens on.
pub const GAME_PORT: u16 = 52131;
const MAGIC: &[u8; 4] = b"NWPL";
const VERSION: u8 = 1;
/// A room not heard from for this long is dropped from the list.
const FORGET: Duration = Duration::from_secs(4);

#[derive(Clone, Debug, PartialEq)]
pub struct Room {
    pub name: String,
    /// Where to join it (`--join`).
    pub addr: SocketAddr,
}

/// This PC's name, as rooms are called.
pub fn computer_name() -> String {
    std::env::var("COMPUTERNAME").ok().filter(|n| !n.is_empty()).unwrap_or_else(|| "NSMBW".into())
}

/// This PC's address on its network (the one its default route uses; no
/// packet is sent to find it).
pub fn local_address() -> Option<IpAddr> {
    let s = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).ok()?;
    s.connect((Ipv4Addr::new(192, 0, 2, 1), 9)).ok()?;
    s.local_addr().ok().map(|a| a.ip()).filter(|ip| !ip.is_unspecified())
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

/// Answers room queries for as long as the game runs: this PC hosts a
/// room on TCP `port`. Gives up quietly when another game on this PC
/// already answers.
pub fn announce(port: u16, name: String) {
    let Ok(socket) = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, DISCOVERY_PORT)) else {
        crate::say::say_err!("LAN: another game on this PC already answers room queries");
        return;
    };
    let _ = std::thread::Builder::new().name("lan-announce".into()).spawn(move || {
        let answer = reply(port, &name);
        let mut buf = [0u8; 64];
        while let Ok((n, from)) = socket.recv_from(&mut buf) {
            if buf[..n] == query()[..] {
                let _ = socket.send_to(&answer, from);
            }
        }
    });
}

/// Looks for rooms while the LAN tab is shown.
pub struct Finder {
    /// This PC's address on its network, when it has one.
    pub here: Option<IpAddr>,
    socket: UdpSocket,
    rooms: Vec<(Room, Instant)>,
    asked: Option<Instant>,
}

impl Finder {
    pub fn new() -> std::io::Result<Self> {
        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))?;
        socket.set_broadcast(true)?;
        socket.set_nonblocking(true)?;
        Ok(Self { here: local_address(), socket, rooms: Vec::new(), asked: None })
    }

    /// The rooms heard from lately (asking again every second).
    pub fn poll(&mut self) -> Vec<Room> {
        let now = Instant::now();
        if self.asked.is_none_or(|t| now - t >= Duration::from_secs(1)) {
            self.asked = Some(now);
            for to in [Ipv4Addr::BROADCAST, Ipv4Addr::LOCALHOST] {
                let _ = self.socket.send_to(&query(), (to, DISCOVERY_PORT));
            }
        }
        let mut buf = [0u8; 128];
        while let Ok((n, from)) = self.socket.recv_from(&mut buf) {
            if let Some(room) = parse_reply(&buf[..n], from) {
                // This PC answers on both its own address and 127.0.0.1.
                let same = |r: &Room| r.addr == room.addr || (r.name == room.name && r.addr.port() == room.addr.port() && (r.addr.ip().is_loopback() || room.addr.ip().is_loopback()));
                match self.rooms.iter_mut().find(|(r, _)| same(r)) {
                    Some((r, seen)) => {
                        if r.addr.ip().is_loopback() && !room.addr.ip().is_loopback() {
                            *r = room;
                        }
                        *seen = now;
                    }
                    None => self.rooms.push((room, now)),
                }
            }
        }
        self.rooms.retain(|(_, seen)| now - *seen < FORGET);
        self.rooms.iter().map(|(r, _)| r.clone()).collect()
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
    }

    /// A host on this PC answers a finder's query.
    #[test]
    fn a_finder_hears_a_host_on_this_pc() {
        // Not on the real port, which a running game may hold.
        let host = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        let host_addr = host.local_addr().unwrap();
        let socket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        socket.set_nonblocking(true).unwrap();
        let mut finder = Finder { here: None, socket, rooms: Vec::new(), asked: Some(Instant::now()) };
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
}
