//! What the port sees of controllers and Bluetooth adapters, without the
//! game: `cargo run -p input --example controllers [seconds]`.
//! Lists the controllers SDL finds, then twice a second each one's
//! accelerometer and the tilt and shake it makes (`input::motion`); on
//! Windows it also says which USB Bluetooth adapter passthrough would use.

use std::time::{Duration, Instant};

use input::motion::PadMotion;
use input::pads::Pads;
use input::PLAYERS;

fn main() {
    let seconds = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(10u64);
    #[cfg(windows)]
    match input::bt_usb::open() {
        Ok(o) => println!("Bluetooth passthrough would use: {}{}", o.name, if o.wii_module { " (a Wii's own module)" } else { "" }),
        Err(e) => println!("Bluetooth passthrough: no adapter ({e})"),
    }
    let Some(mut pads) = Pads::new() else { return };
    let mut motion: [PadMotion; PLAYERS] = Default::default();
    let start = Instant::now();
    let mut next_print = Duration::ZERO;
    while start.elapsed() < Duration::from_secs(seconds) {
        for (slot, name) in pads.update() {
            println!("controller {}: {name}", slot + 1);
        }
        let now = start.elapsed();
        for slot in 0..PLAYERS {
            let Some(a) = pads.accel(slot) else { continue };
            let m = motion[slot].update(a, now);
            if now >= next_print {
                println!("controller {}: accel {:6.2} {:6.2} {:6.2} m/s2 -> tilt {:5.2}{}", slot + 1, a[0], a[1], a[2], m.tilt, if m.shake { ", shake" } else { "" });
            }
        }
        if now >= next_print {
            next_print = now + Duration::from_millis(500);
        }
        std::thread::sleep(Duration::from_millis(16));
    }
    if !pads.any_connected() {
        println!("no controllers");
    }
}
