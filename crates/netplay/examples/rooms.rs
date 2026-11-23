//! Lists the rooms found on the network and over Tailscale for a few
//! seconds, as the LAN tab would: `cargo run -p netplay --example rooms
//! [seconds] [announce]` (from the game's folder, which holds
//! saves/lan-devices.txt). With `announce`, this also answers and announces
//! a room called after this PC, as a hosting game does (no game runs).

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let secs: u64 = args.first().and_then(|s| s.parse().ok()).unwrap_or(5);
    if args.iter().any(|a| a == "announce") {
        netplay::lan::announce(netplay::lan::GAME_PORT, netplay::lan::computer_name()).expect("answer on the LAN play port");
    }
    let mut finder = netplay::lan::Finder::new().expect("open a UDP socket");
    println!("this device: {:?}, Tailscale {:?}", finder.here, finder.tailscale);
    let until = std::time::Instant::now() + std::time::Duration::from_secs(secs);
    let mut last = Vec::new();
    while std::time::Instant::now() < until {
        let rooms = finder.poll();
        if rooms != last {
            for r in &rooms {
                println!("room {:?} at {}{}", r.name, r.addr, if r.tailscale() { " (Tailscale)" } else { "" });
            }
            last = rooms;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    println!("{} room(s)", last.len());
}
