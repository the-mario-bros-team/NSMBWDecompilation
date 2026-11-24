//! A GPU driver the player imported (the app's GPU Driver setting, such as
//! Turnip), used in place of the phone's own: libadrenotools (BSD-2-Clause,
//! `android/third_party/libadrenotools`) opens the system's `libvulkan.so`
//! with its driver loading redirected to the imported one. The app unpacks
//! libadrenotools and its hook libraries into its files only once a driver
//! is picked (`GpuDrivers.java`), so a phone on its own driver never loads
//! them.

use std::ffi::{c_char, c_int, c_void, CStr, CString};

/// `ADRENOTOOLS_DRIVER_CUSTOM` (`adrenotools/priv.h`).
const DRIVER_CUSTOM: c_int = 1;

type OpenLibvulkan = unsafe extern "C" fn(c_int, c_int, *const c_char, *const c_char, *const c_char, *const c_char, *const c_char, *mut *mut c_void) -> *mut c_void;

fn dl_error() -> String {
    // SAFETY: dlerror returns null or a NUL-terminated string.
    let e = unsafe { libc::dlerror() };
    if e.is_null() {
        "unknown error".into()
    } else {
        unsafe { CStr::from_ptr(e) }.to_string_lossy().into_owned()
    }
}

/// libadrenotools joins a folder and a file name as they are: folders end
/// in a slash.
fn folder(path: &str) -> Result<CString, String> {
    let path = if path.ends_with('/') { path.to_string() } else { format!("{path}/") };
    CString::new(path).map_err(|e| e.to_string())
}

/// Opens Vulkan with the driver `library` in `driver_dir`, through the
/// libadrenotools and hooks in `hook_dir` (`tmp_dir`: for patched
/// libraries on Android 9, which has no memfd), and has the renderer use it.
/// Once per process, before the renderer is made.
pub fn open(hook_dir: &str, driver_dir: &str, library: &str, tmp_dir: &str) -> Result<(), String> {
    let (hooks, drivers, tmp) = (folder(hook_dir)?, folder(driver_dir)?, folder(tmp_dir)?);
    let name = CString::new(library).map_err(|e| e.to_string())?;
    let tools = CString::new(format!("{}libadrenotools.so", hooks.to_string_lossy())).map_err(|e| e.to_string())?;
    // SAFETY: plain dlopen/dlsym of a library the app unpacked; the symbol
    // has the signature `adrenotools/driver.h` declares.
    unsafe {
        let lib = libc::dlopen(tools.as_ptr(), libc::RTLD_NOW);
        if lib.is_null() {
            return Err(format!("libadrenotools: {}", dl_error()));
        }
        let open = libc::dlsym(lib, c"adrenotools_open_libvulkan".as_ptr());
        if open.is_null() {
            return Err(format!("libadrenotools: {}", dl_error()));
        }
        let open: OpenLibvulkan = std::mem::transmute(open);
        let vulkan = open(libc::RTLD_NOW, DRIVER_CUSTOM, tmp.as_ptr(), hooks.as_ptr(), drivers.as_ptr(), name.as_ptr(), std::ptr::null(), std::ptr::null_mut());
        if vulkan.is_null() {
            return Err(format!("libadrenotools could not open {library}"));
        }
        let get_instance_proc_addr = libc::dlsym(vulkan, c"vkGetInstanceProcAddr".as_ptr());
        if get_instance_proc_addr.is_null() {
            return Err(format!("{library}: no vkGetInstanceProcAddr"));
        }
        // The library stays open for the process's life.
        vk_render::set_vulkan_loader(std::mem::transmute::<*mut c_void, _>(get_instance_proc_addr));
    }
    Ok(())
}
