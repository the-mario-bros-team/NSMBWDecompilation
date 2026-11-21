//! Real Wii Remotes paired with Windows (Settings > Bluetooth), read and
//! written as HID devices through hidapi, each standing in for one player's
//! emulated remote ([`hle_ios::bt::HidRemote`]). After WitherBlitz's
//! NSMB-Wii-Recompilation (`wii_remote_input.cpp`), which goes through
//! SDL's HIDAPI Wii driver; here the remote's own reports pass straight to
//! the game, which drives the remote itself (reporting mode, LEDs, rumble,
//! calibration reads, extensions). The Bluetooth adapter keeps Windows'
//! driver: nothing is switched to WinUSB, and there is no passthrough.
//!
//! Not tried on hardware: no Wii Remote was at hand when this was written.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::sync::Arc;
use std::time::Duration;

use hle_ios::bt::{HidRemote, REMOTES};

const NINTENDO: u16 = 0x057E;
/// RVL-CNT-01 and RVL-CNT-01-TR (the MotionPlus Inside remote).
const WII_REMOTES: [u16; 2] = [0x0306, 0x0330];
/// Paired remotes are looked for this often.
const SCAN_EVERY: Duration = Duration::from_secs(2);

/// One open remote: reports to and from its own thread.
pub struct Remote {
    to: Sender<Vec<u8>>,
    from: Receiver<Vec<u8>>,
    alive: Arc<AtomicBool>,
}

impl HidRemote for Remote {
    fn send(&mut self, report: &[u8]) {
        let _ = self.to.send(report.to_vec());
    }

    fn recv(&mut self) -> Option<Vec<u8>> {
        self.from.try_recv().ok()
    }

    fn alive(&self) -> bool {
        self.alive.load(Ordering::Relaxed)
    }
}

/// The remote's reports both ways until it goes away (or is dropped).
fn serve(device: hidapi::HidDevice, to: Receiver<Vec<u8>>, from: Sender<Vec<u8>>, alive: Arc<AtomicBool>) {
    let mut buf = [0u8; 64];
    'run: loop {
        match device.read_timeout(&mut buf, 2) {
            Ok(0) => {}
            Ok(n) => {
                if from.send(buf[..n].to_vec()).is_err() {
                    break;
                }
            }
            Err(_) => break,
        }
        loop {
            match to.try_recv() {
                Ok(report) => {
                    if device.write(&report).is_err() {
                        break 'run;
                    }
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break 'run,
            }
        }
    }
    alive.store(false, Ordering::Relaxed);
}

/// The players' real remotes: which slots are taken (by which device), so a
/// remote found again is not opened twice and one gone frees its player.
#[derive(Default)]
struct Slots {
    taken: [Option<(Vec<u8>, Arc<AtomicBool>)>; REMOTES],
}

impl Slots {
    /// The player for a remote at `path`: none if it is open already or all
    /// four players have one.
    fn place(&mut self, path: &[u8]) -> Option<usize> {
        for slot in &mut self.taken {
            if slot.as_ref().is_some_and(|(_, alive)| !alive.load(Ordering::Relaxed)) {
                *slot = None;
            }
        }
        if self.taken.iter().flatten().any(|(p, _)| p == path) {
            return None;
        }
        self.taken.iter().position(Option::is_none)
    }
}

/// Looks for paired remotes on a thread of its own; each one found is
/// opened and handed over with the player (0-3) it plays.
pub struct Scanner {
    found: Receiver<(usize, Remote, String)>,
    stop: Arc<AtomicBool>,
}

impl Scanner {
    pub fn start() -> Self {
        let (tx, found) = mpsc::channel();
        let stop = Arc::new(AtomicBool::new(false));
        let quit = stop.clone();
        let _ = std::thread::Builder::new().name("wii remotes".into()).spawn(move || {
            let mut api = match hidapi::HidApi::new() {
                Ok(a) => a,
                Err(e) => {
                    eprintln!("Wii Remotes: HID unavailable: {e}");
                    return;
                }
            };
            let mut slots = Slots::default();
            while !quit.load(Ordering::Relaxed) {
                let _ = api.refresh_devices();
                let remotes: Vec<(std::ffi::CString, String)> = api
                    .device_list()
                    .filter(|d| d.vendor_id() == NINTENDO && WII_REMOTES.contains(&d.product_id()))
                    .map(|d| (d.path().to_owned(), d.product_string().unwrap_or("Wii Remote").to_string()))
                    .collect();
                for (path, name) in remotes {
                    let Some(player) = slots.place(path.as_bytes()) else { continue };
                    let device = match api.open_path(&path) {
                        Ok(d) => d,
                        // Paired but switched off: Windows still lists it.
                        Err(_) => continue,
                    };
                    let alive = Arc::new(AtomicBool::new(true));
                    let (to_tx, to_rx) = mpsc::channel();
                    let (from_tx, from_rx) = mpsc::channel();
                    let a = alive.clone();
                    if std::thread::Builder::new().name("wii remote".into()).spawn(move || serve(device, to_rx, from_tx, a)).is_err() {
                        continue;
                    }
                    slots.taken[player] = Some((path.as_bytes().to_vec(), alive.clone()));
                    if tx.send((player, Remote { to: to_tx, from: from_rx, alive }, name)).is_err() {
                        return;
                    }
                }
                std::thread::sleep(SCAN_EVERY);
            }
        });
        Self { found, stop }
    }

    /// A remote found since the last look: its player, the remote, its name.
    pub fn take(&self) -> Option<(usize, Remote, String)> {
        self.found.try_recv().ok()
    }
}

impl Drop for Scanner {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remotes_take_the_first_free_player() {
        let mut s = Slots::default();
        assert_eq!(s.place(b"a"), Some(0));
        let a = Arc::new(AtomicBool::new(true));
        s.taken[0] = Some((b"a".to_vec(), a.clone()));
        assert_eq!(s.place(b"a"), None, "open already");
        assert_eq!(s.place(b"b"), Some(1));
        // Gone: its player is free again, and it can come back.
        a.store(false, Ordering::Relaxed);
        assert_eq!(s.place(b"a"), Some(0));
    }
}
