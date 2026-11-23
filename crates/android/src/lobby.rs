//! LAN play for the Java side (`LanPlay.java`): finding rooms on the
//! network and over Tailscale (`netplay::lan`) while the start menu's LAN
//! Play is open, the devices asked for rooms, and the session the game's
//! process starts with (`netplay::net`, set before `Native.start`).

use std::sync::Mutex;

use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jint, jobjectArray, jstring};
use jni::JNIEnv;
use netplay::{lan, net};

/// The session the game starts with, and this device's name for a room it
/// hosts.
static NET: Mutex<Option<(net::Mode, String)>> = Mutex::new(None);

/// Whether the game's session is a net one (`Native.setNet`).
static ONLINE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// The rooms finder, while LAN Play is open.
static FINDER: Mutex<Option<lan::Finder>> = Mutex::new(None);

/// The session set by `Native.setNet` (none: a game of one's own), taken
/// once; `name` is the room's.
pub fn take_net() -> (net::Mode, String) {
    NET.lock().unwrap().take().unwrap_or_default()
}

/// Whether this game is a net one: it keeps running while the app is away,
/// since the other players' games wait for this one.
pub fn online() -> bool {
    ONLINE.load(std::sync::atomic::Ordering::Relaxed)
}

fn strings(env: &mut JNIEnv, items: &[String]) -> jobjectArray {
    let Ok(array) = env.new_object_array(items.len() as i32, "java/lang/String", JObject::null()) else { return std::ptr::null_mut() };
    for (i, s) in items.iter().enumerate() {
        if let Ok(js) = env.new_string(s) {
            let _ = env.set_object_array_element(&array, i as i32, &js);
            let _ = env.delete_local_ref(js);
        }
    }
    array.into_raw()
}

fn string(env: &mut JNIEnv, s: &JString) -> String {
    env.get_string(s).map(Into::into).unwrap_or_default()
}

/// `Native.setNet(mode, addr, name)`: before `start`, in the game's
/// process: 0 alone, 1 host a room (named `name`) on the LAN play port, 2
/// join the room at `addr` (`ip`, `ip:port` or a Tailscale name).
#[no_mangle]
pub extern "system" fn Java_com_wither_nsmbwport_Native_setNet<'l>(mut env: JNIEnv<'l>, _: JClass<'l>, mode: jint, addr: JString<'l>, name: JString<'l>) {
    let (addr, name) = (string(&mut env, &addr), string(&mut env, &name));
    let mode = match mode {
        1 => net::Mode::Host { port: lan::GAME_PORT, wait_join: None },
        2 => net::Mode::Join { addr: lan::with_port(&addr) },
        _ => net::Mode::Off,
    };
    ONLINE.store(!matches!(mode, net::Mode::Off), std::sync::atomic::Ordering::Relaxed);
    *NET.lock().unwrap() = Some((mode, name));
}

/// `Native.lanOpen(filesDir)`: starts looking for rooms (the known devices
/// are kept under `filesDir`). Returns null, or why it cannot.
#[no_mangle]
pub extern "system" fn Java_com_wither_nsmbwport_Native_lanOpen<'l>(mut env: JNIEnv<'l>, _: JClass<'l>, files: JString<'l>) -> jstring {
    let files = string(&mut env, &files);
    let result = std::env::set_current_dir(&files).map_err(|e| format!("{files}: {e}")).and_then(|()| lan::Finder::listening().map_err(|e| e.to_string()));
    match result {
        Ok(f) => {
            *FINDER.lock().unwrap() = Some(f);
            std::ptr::null_mut()
        }
        Err(e) => env.new_string(e).map(JString::into_raw).unwrap_or(std::ptr::null_mut()),
    }
}

/// `Native.lanClose()`: stops looking.
#[no_mangle]
pub extern "system" fn Java_com_wither_nsmbwport_Native_lanClose<'l>(_: JNIEnv<'l>, _: JClass<'l>) {
    *FINDER.lock().unwrap() = None;
}

/// `Native.lanRooms()`: the rooms heard from lately, three strings each:
/// name, address (`ip:port`) and "1" when found over Tailscale.
#[no_mangle]
pub extern "system" fn Java_com_wither_nsmbwport_Native_lanRooms<'l>(mut env: JNIEnv<'l>, _: JClass<'l>) -> jobjectArray {
    let rooms = FINDER.lock().unwrap().as_mut().map(lan::Finder::poll).unwrap_or_default();
    let flat: Vec<String> = rooms.into_iter().flat_map(|r| [r.name.clone(), r.addr.to_string(), if r.tailscale() { "1".into() } else { "0".into() }]).collect();
    strings(&mut env, &flat)
}

/// `Native.lanAddresses()`: this device's addresses, Tailscale first: what
/// another player types in to find it.
#[no_mangle]
pub extern "system" fn Java_com_wither_nsmbwport_Native_lanAddresses<'l>(mut env: JNIEnv<'l>, _: JClass<'l>) -> jobjectArray {
    let mut list: Vec<String> = lan::tailscale_address().map(|ip| ip.to_string()).into_iter().collect();
    list.extend(lan::local_address().map(|ip| ip.to_string()).filter(|ip| !list.contains(ip)));
    strings(&mut env, &list)
}

/// `Native.lanDevices()`: the devices asked for rooms directly.
#[no_mangle]
pub extern "system" fn Java_com_wither_nsmbwport_Native_lanDevices<'l>(mut env: JNIEnv<'l>, _: JClass<'l>) -> jobjectArray {
    strings(&mut env, &lan::known_devices())
}

/// `Native.lanAddDevice(nameOrAddress)`: false when it is known already or
/// is not a name or address.
#[no_mangle]
pub extern "system" fn Java_com_wither_nsmbwport_Native_lanAddDevice<'l>(mut env: JNIEnv<'l>, _: JClass<'l>, text: JString<'l>) -> jboolean {
    let text = string(&mut env, &text);
    lan::add_device(&text).into()
}

/// `Native.lanRemoveDevice(nameOrAddress)`.
#[no_mangle]
pub extern "system" fn Java_com_wither_nsmbwport_Native_lanRemoveDevice<'l>(mut env: JNIEnv<'l>, _: JClass<'l>, text: JString<'l>) {
    let text = string(&mut env, &text);
    lan::remove_device(&text);
}
