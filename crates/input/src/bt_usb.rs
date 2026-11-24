//! Bluetooth passthrough's USB side (`hle_ios::bt_passthrough`): finds a
//! USB Bluetooth adapter that has the WinUSB driver (given with Zadig),
//! claims its HCI interface and moves its traffic on threads of its own.
//! After WitherBlitz's NSMB-Wii-Recompilation (`bt_usb_libusb.cpp`), through
//! nusb instead of libusb.
//!
//! An adapter on any other driver, such as the PC's own Bluetooth on
//! Windows' driver, is never opened: it is only named in the reason none
//! could be used.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

use hle_ios::bt_passthrough::Adapter;
use nusb::descriptors::TransferType;
use nusb::transfer::{Buffer, Bulk, ControlOut, ControlType, In, Interrupt, Out, Recipient};
use nusb::{DeviceInfo, Interface, MaybeFuture};

const NINTENDO: u16 = 0x057E;
const WII_MODULE: u16 = 0x0305;
/// Wireless controller, RF, Bluetooth.
const BLUETOOTH: (u8, u8, u8) = (0xE0, 0x01, 0x01);
const IN_TRANSFERS: usize = 2;

/// A claimed adapter.
pub struct UsbAdapter {
    out: Option<Sender<Outgoing>>,
    events: Receiver<Vec<u8>>,
    acl: Receiver<Vec<u8>>,
    failed: Arc<Mutex<Option<String>>>,
    stop: Arc<AtomicBool>,
    threads: Vec<JoinHandle<()>>,
}

enum Outgoing {
    Command(Vec<u8>),
    Acl(Vec<u8>),
}

/// What [`open`] found.
pub struct Opened {
    pub adapter: UsbAdapter,
    /// Its name and USB ID.
    pub name: String,
    /// A Wii's own Bluetooth module (057e:0305).
    pub wii_module: bool,
}

fn id(d: &DeviceInfo) -> String {
    format!("{:04x}:{:04x}", d.vendor_id(), d.product_id())
}

fn name(d: &DeviceInfo) -> String {
    match d.product_string() {
        Some(p) => format!("{} ({})", p.trim(), id(d)),
        None => id(d),
    }
}

fn is_bluetooth(d: &DeviceInfo) -> bool {
    (d.vendor_id(), d.product_id()) == (NINTENDO, WII_MODULE)
        || (d.class(), d.subclass(), d.protocol()) == BLUETOOTH
        || d.interfaces().any(|i| i.interface_number() == 0 && (i.class(), i.subclass(), i.protocol()) == BLUETOOTH)
}

/// Finds and claims the first usable adapter (a Wii's own module first);
/// otherwise why none could be used.
pub fn open() -> Result<Opened, String> {
    let devices: Vec<DeviceInfo> = nusb::list_devices().wait().map_err(|e| format!("listing USB devices: {e}"))?.filter(is_bluetooth).collect();
    let mut sorted: Vec<&DeviceInfo> = devices.iter().collect();
    sorted.sort_by_key(|d| (d.vendor_id(), d.product_id()) != (NINTENDO, WII_MODULE));
    let mut problems = Vec::new();
    for d in sorted {
        // Windows' own Bluetooth driver (or any but WinUSB, or the composite
        // driver that may hand interface 0 to WinUSB) is left alone.
        #[cfg(windows)]
        if let Some(driver) = d.driver().filter(|v| !v.eq_ignore_ascii_case("winusb") && !v.eq_ignore_ascii_case("usbccgp")) {
            problems.push(format!("{}: on the {driver} driver, not WinUSB (Zadig can give it WinUSB)", name(d)));
            continue;
        }
        match claim(d) {
            Ok(adapter) => return Ok(Opened { adapter, name: name(d), wii_module: (d.vendor_id(), d.product_id()) == (NINTENDO, WII_MODULE) }),
            Err(e) => problems.push(format!("{}: {e}", name(d))),
        }
    }
    Err(if problems.is_empty() { "no USB Bluetooth adapter found".into() } else { problems.join("; ") })
}

/// Interface 0's endpoints: interrupt IN (events), bulk IN and OUT (ACL).
fn endpoints(i: &Interface) -> Option<(u8, u8, u8)> {
    let desc = i.descriptor()?;
    let find = |ty, inward: bool| desc.endpoints().find(|e| e.transfer_type() == ty && (e.address() & 0x80 != 0) == inward).map(|e| e.address());
    Some((find(TransferType::Interrupt, true)?, find(TransferType::Bulk, true)?, find(TransferType::Bulk, false)?))
}

fn claim(d: &DeviceInfo) -> Result<UsbAdapter, String> {
    let device = d.open().wait().map_err(|e| format!("couldn't open it ({e})"))?;
    let interface = device.claim_interface(0).wait().map_err(|e| format!("couldn't claim it ({e})"))?;
    let (ev, acl_in, acl_out) = endpoints(&interface).ok_or("its interface 0 has no HCI endpoints")?;
    let mut events = interface.endpoint::<Interrupt, In>(ev).map_err(|e| e.to_string())?;
    let mut acl = interface.endpoint::<Bulk, In>(acl_in).map_err(|e| e.to_string())?;
    let mut acl_out = interface.endpoint::<Bulk, Out>(acl_out).map_err(|e| e.to_string())?;
    let failed = Arc::new(Mutex::new(None));
    let stop = Arc::new(AtomicBool::new(false));
    let (out_tx, out_rx) = channel::<Outgoing>();
    let (ev_tx, ev_rx) = channel();
    let (acl_tx, acl_rx) = channel();
    let mut threads = Vec::new();
    // The readers: each keeps two transfers waiting.
    macro_rules! reader {
        ($ep:ident, $tx:ident, $len:expr) => {{
            let (failed, stop) = (failed.clone(), stop.clone());
            threads.push(std::thread::spawn(move || {
                let mps = $ep.max_packet_size().max(1);
                let len = ($len / mps).max(1) * mps;
                for _ in 0..IN_TRANSFERS {
                    $ep.submit(Buffer::new(len));
                }
                while !stop.load(Ordering::Relaxed) {
                    let Some(done) = $ep.wait_next_complete(Duration::from_millis(100)) else { continue };
                    if let Err(e) = done.status {
                        failed.lock().unwrap().get_or_insert(format!("reading endpoint {:#04x}: {e}", $ep.endpoint_address()));
                        break;
                    }
                    if done.actual_len > 0 && $tx.send(done.buffer[..done.actual_len].to_vec()).is_err() {
                        break;
                    }
                    let mut buf = done.buffer;
                    buf.clear();
                    buf.set_requested_len(len);
                    $ep.submit(buf);
                }
                $ep.cancel_all();
                while $ep.pending() > 0 && $ep.wait_next_complete(Duration::from_secs(1)).is_some() {}
            }));
        }};
    }
    reader!(events, ev_tx, 1024);
    reader!(acl, acl_tx, 4096);
    // The writer: HCI commands as class requests to the device, ACL on
    // the bulk OUT endpoint, in the order the game sent them.
    {
        let failed = failed.clone();
        threads.push(std::thread::spawn(move || {
            for packet in out_rx {
                let result = match packet {
                    Outgoing::Command(c) => interface
                        .control_out(ControlOut { control_type: ControlType::Class, recipient: Recipient::Device, request: 0, value: 0, index: 0, data: &c }, Duration::from_secs(1))
                        .wait()
                        .map_err(|e| format!("sending a command: {e}")),
                    Outgoing::Acl(a) => acl_out.transfer_blocking(Buffer::from(a), Duration::from_secs(1)).status.map_err(|e| format!("sending ACL data: {e}")),
                };
                if let Err(e) = result {
                    failed.lock().unwrap().get_or_insert(e);
                }
            }
        }));
    }
    Ok(UsbAdapter { out: Some(out_tx), events: ev_rx, acl: acl_rx, failed, stop, threads })
}

impl Adapter for UsbAdapter {
    fn send_command(&mut self, packet: &[u8]) {
        if let Some(out) = &self.out {
            let _ = out.send(Outgoing::Command(packet.to_vec()));
        }
    }

    fn send_acl(&mut self, packet: &[u8]) {
        if let Some(out) = &self.out {
            let _ = out.send(Outgoing::Acl(packet.to_vec()));
        }
    }

    fn recv_events(&mut self) -> Option<Vec<u8>> {
        self.events.try_recv().ok()
    }

    fn recv_acl(&mut self) -> Option<Vec<u8>> {
        self.acl.try_recv().ok()
    }

    fn failed(&self) -> Option<String> {
        self.failed.lock().unwrap().clone()
    }
}

impl Drop for UsbAdapter {
    /// What was sent goes out (the shutdown's Reset among it), then the
    /// adapter is let go.
    fn drop(&mut self) {
        self.out = None;
        self.stop.store(true, Ordering::Relaxed);
        for t in self.threads.drain(..) {
            let _ = t.join();
        }
    }
}
